use micromath::F32Ext;

use Direction::{Forward, Reverse};

use crate::sequencer::sequencer::{Direction, MaskU8, Position};

#[derive(Debug, Clone, Copy)]
pub enum StageMode {
    Forward = 0,
    Reverse = 1,
    PingPong = 2,
    Brownian = 3,
    Random = 4,
}

impl StageMode {
    pub fn next_stage(self, stage_mask: MaskU8, pos: Position) -> Position {
        match self {
            Self::Forward => Self::forward(stage_mask, pos),
            Self::Reverse => Self::reverse(stage_mask, pos),
            Self::PingPong => Self::ping_pong(stage_mask, pos),
            Self::Brownian => Self::brownian(stage_mask, pos),
            Self::Random => Self::random(stage_mask, pos),
        }
    }

    fn random(stage_mask: MaskU8, pos: Position) -> Position {
        let idx = F32Ext::round(Self::rnd() * 7 as f32) as u8;
        let lower = stage_mask.next_lower(idx);
        let higher = stage_mask.next_higher(idx);
        match (lower, higher) {
            (Some(i), None) | (None, Some(i)) =>
                Position { stage: i, pulse: 0, dir: pos.dir },
            (Some(l), Some(h)) => {
                if idx - l < h - idx {
                    Position { stage: l, pulse: 0, dir: pos.dir }
                } else {
                    Position { stage: h, pulse: 0, dir: pos.dir }
                }
            },
            (None, None) =>
                Position { stage: pos.stage, pulse: 0, dir: pos.dir },
        }
    }

    fn brownian(stage_mask: MaskU8, pos: Position) -> Position {
        match (Self::rnd(), Self::rnd()) {
            (a, _) if a > 0.5 => Self::forward(stage_mask, pos),
            (_, b) if b > 0.5 => Position { stage: pos.stage, pulse: 0, dir: pos.dir },
            _ => Self::reverse(stage_mask, pos),
        }
    }

    fn ping_pong(stage_mask: MaskU8, pos: Position) -> Position {
        let lower = stage_mask.next_lower(pos.stage);
        let higher = stage_mask.next_higher(pos.stage);
        let dir = pos.dir;
        match (dir, lower, higher) {
            (Forward, Some(p), None) =>
                Position { stage: p, pulse: 0, dir: Reverse },
            (Reverse, None, Some(p)) =>
                Position { stage: p, pulse: 0, dir: Forward },
            (Forward, _, Some(p)) =>
                Position { stage: p, pulse: 0, dir: Forward },
            (Reverse, Some(p), _) =>
                Position { stage: p, pulse: 0, dir: Reverse },
            _ =>
                Position { stage: pos.stage, pulse: 0, dir: pos.dir }
        }
    }

    fn reverse(stage_mask: MaskU8, pos: Position) -> Position {
        let lower = stage_mask.next_lower(pos.stage);
        let highest = stage_mask.highest().expect("should exist");
        match lower {
            Some(p) => Position { stage: p, pulse: 0, dir: Reverse },
            None => Position { stage: highest, pulse: 0, dir: Reverse },
        }
    }

    fn forward(stage_mask: MaskU8, pos: Position) -> Position {
        let higher = stage_mask.next_higher(pos.stage);
        let lowest = stage_mask.lowest().expect("should exist");
        match higher {
            Some(p) => Position { stage: p, pulse: 0, dir: Forward },
            None => Position { stage: lowest, pulse: 0, dir: Forward },
        }
    }

    // TODO: Inject a RNG
    fn rnd() -> f32 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use crate::sequencer::sequencer::{Direction, MaskU8, Position};
    use crate::sequencer::stage_mode::StageMode::{Forward, PingPong, Reverse};

    fn pos(stage: u8, dir: Direction) -> Position {
        Position { stage, pulse: 0, dir }
    }

    fn pos_fwd(stage: u8) -> Position {
        pos(stage, Direction::Forward)
    }

    fn pos_rev(stage: u8) -> Position {
        pos(stage, Direction::Reverse)
    }

    #[test]
    fn test_next_stage() {
        // Forward
        assert_eq!(0, Forward.next_stage(MaskU8(0b_0000_0001), pos_fwd(0)).stage);
        assert_eq!(1, Forward.next_stage(MaskU8(0b_0000_0011), pos_fwd(0)).stage);
        assert_eq!(1, Forward.next_stage(MaskU8(0b_0000_0010), pos_fwd(0)).stage);
        assert_eq!(0, Forward.next_stage(MaskU8(0b_0000_0001), pos_fwd(7)).stage);
        assert_eq!(1, Forward.next_stage(MaskU8(0b_0000_0010), pos_fwd(7)).stage);

        // Reverse
        assert_eq!(7, Reverse.next_stage(MaskU8(0b_1000_0000), pos_rev(7)).stage);
        assert_eq!(6, Reverse.next_stage(MaskU8(0b_1100_0000), pos_rev(7)).stage);
        assert_eq!(6, Reverse.next_stage(MaskU8(0b_0100_0000), pos_rev(7)).stage);
        assert_eq!(7, Reverse.next_stage(MaskU8(0b_1000_0000), pos_rev(0)).stage);
        assert_eq!(6, Reverse.next_stage(MaskU8(0b_0100_0000), pos_rev(0)).stage);

        // PingPong Forward
        assert_eq!(0, PingPong.next_stage(MaskU8(0b_0000_0001), pos_fwd(0)).stage);
        assert_eq!(1, PingPong.next_stage(MaskU8(0b_0000_0011), pos_fwd(0)).stage);
        assert_eq!(0, PingPong.next_stage(MaskU8(0b_0100_0001), pos_fwd(6)).stage);
        assert_eq!(5, PingPong.next_stage(MaskU8(0b_0110_0001), pos_fwd(6)).stage);
        assert_eq!(Direction::Reverse, PingPong.next_stage(MaskU8(0b_0110_0001), pos_fwd(6)).dir);

        // PingPong Reverse
        assert_eq!(7, PingPong.next_stage(MaskU8(0b_1000_0000), pos_rev(7)).stage);
        assert_eq!(6, PingPong.next_stage(MaskU8(0b_1100_0000), pos_rev(7)).stage);
        assert_eq!(7, PingPong.next_stage(MaskU8(0b_1000_0010), pos_rev(1)).stage);
        assert_eq!(2, PingPong.next_stage(MaskU8(0b_1000_0110), pos_rev(1)).stage);
        assert_eq!(Direction::Forward, PingPong.next_stage(MaskU8(0b_1000_0110), pos_rev(1)).dir);
    }
}
