use crate::musical::gate::Gate;
use crate::musical::note::Note;
use micromath::F32Ext;

const N: usize = 8;

#[derive(Debug, Clone)]
pub struct Sequencer {
    config: Config,
    position: Position,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Self { position: Position { stage: 0, pulse: 0, direction: Direction::Forward }, config: Config::new() }
    }

    pub fn config(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn state(&self) -> State {
        let current_stage = self.stage(self.position).expect("stage should exist");
        //TODO: We need to implement a notion of time to trigger gates
        State { gate: Gate::Open, note: current_stage.note }
    }

    pub fn step(&mut self) {
        if !self.config.has_pulses() { return; }

        let current_stage = self.stage(self.position).expect("stage should exist");
        if self.position.pulse < current_stage.pulse_count - 1 && !current_stage.skipped {
            self.position = Position { stage: self.position.stage, pulse: self.position.pulse + 1, direction: self.position.direction }
        } else {
            self.position = self.next_stage_position(self.position)
        }
    }

    fn next_stage_position(&self, position: Position) -> Position {
        self.config.stage_mode.next_stage(self.config.has_pulses_mask(), position)
     }

    fn stage(&self, pos: Position) -> Option<&Stage> {
        self.config.stages.get(pos.stage as usize)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StageMode {
    Forward = 0,
    Reverse = 1,
    PingPong = 2,
    Brownian = 3,
    Random = 4,
}

impl StageMode {
    pub fn next_stage(self, stage_mask: MaskU8, position: Position) -> Position {
        match self {
            StageMode::Forward => StageMode::forward(stage_mask, position),
            StageMode::Reverse => StageMode::reverse(stage_mask, position),
            StageMode::PingPong => StageMode::ping_pong(stage_mask, position),
            StageMode::Brownian => StageMode::brownian(stage_mask, position),
            StageMode::Random => StageMode::random(stage_mask, position),
        }
    }

    fn random(stage_mask: MaskU8, position: Position) -> Position {
        if stage_mask.count() == 0 {
            Position{ stage: position.stage, pulse: 0, direction: position.direction }
        } else {
            let rnd = stage_mask.next_lower(F32Ext::round(Self::rnd() * 7 as f32) as u8).expect("should exist");
            Position{ stage: rnd, pulse: 0, direction: position.direction }
        }
    }

    fn brownian(stage_mask: MaskU8, position: Position) -> Position {
        match (Self::rnd(), Self::rnd()) {
            (a, _) if a > 0.5 => Self::forward(stage_mask, position),
            (_, b) if b > 0.5 => Position{ stage: position.stage, pulse: 0, direction: position.direction },
            _ => Self::reverse(stage_mask, position),
        }
    }

    fn ping_pong(stage_mask: MaskU8, position: Position) -> Position {
        let lower = stage_mask.next_lower(position.stage);
        let higher = stage_mask.next_higher(position.stage);
        let dir = position.direction;
        match (dir, lower, higher) {
            (Direction::Forward,Some(p),None) =>
                Position{ stage: p, pulse: 0, direction: Direction::Reverse },
            (Direction::Reverse,None,Some(p)) =>
                Position{ stage: p, pulse: 0, direction: Direction::Forward },
            (Direction::Forward, _, Some(p)) =>
                Position{ stage: p, pulse: 0, direction: Direction::Forward },
            (Direction::Reverse, Some(p), _) =>
                Position{ stage: p, pulse: 0, direction: Direction::Reverse },
            _ =>
                Position{ stage: position.stage, pulse: 0, direction: position.direction }
        }
    }

    fn reverse(stage_mask: MaskU8, position: Position) -> Position {
        let lower = stage_mask.next_lower(position.stage);
        let highest = stage_mask.highest().expect("should exist");
        match lower {
            Some(p) => Position{ stage: p, pulse: 0, direction: Direction::Reverse },
            None => Position{ stage: highest, pulse: 0, direction: Direction::Reverse },
        }
    }

    fn forward(stage_mask: MaskU8, position: Position) -> Position {
        let higher = stage_mask.next_higher(position.stage);
        let lowest = stage_mask.lowest().expect("should exist");
        match higher {
            Some(p) => Position{ stage: p, pulse: 0, direction: Direction::Forward },
            None => Position{ stage: lowest, pulse: 0, direction: Direction::Forward },
        }
    }

    fn rnd() -> f32 {
        1.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    stage: u8,
    pulse: u8,
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Forward,
    Reverse,
}

#[derive(Debug, Clone)]
pub struct Config {
    stages: [Stage; N],
    stage_mode: StageMode,
}

impl Config
{
    pub fn new() -> Self {
        Self { stages: [Stage::default(); N], stage_mode: StageMode::Forward }
    }

    pub fn stage(&mut self, index: usize) -> Option<&mut Stage> {
        self.stages.get_mut(index)
    }

    pub fn stages(&self) -> &[Stage] { &self.stages }

    pub fn has_pulses(&self) -> bool { self.stages.iter().any(|s| s.has_pulses()) }

    pub fn has_pulses_mask(&self) -> MaskU8 {
        let mut mask = 0_u8;
        for i in 0..N {
            if self.stages[i].has_pulses() {
                mask |= 1 << i
            }
        };
        MaskU8(mask)
    }

}

#[derive(Debug, Clone, Copy)]
pub struct MaskU8(u8);

impl MaskU8 {
    pub fn new() -> MaskU8 { Self(0) }

    pub fn next_higher(&self, pos: u8) -> Option<u8> {
        for i in pos+1..8 {
            if self.0 & (1 << i) > 0 {
                return Some(i)
            }
        };
        None
    }

    fn next_lower(&self, pos: u8) -> Option<u8> {
        if pos == 0 { return None }
        for i in (0..pos).rev() {
            if self.0 & (1 << i) > 0 {
                return Some(i)
            }
        };
        None
    }

    fn highest(&self) -> Option<u8> {
        self.next_lower(7)
    }

    fn lowest(&self) -> Option<u8> {
        self.next_higher(0)
    }

    fn is_set(&self, pos: u8) -> bool {
        self.0 & (1 << pos) > 0
    }

    fn count(&self) -> u8 {
        let mut count = 0;
        let mut bits = self.0;
        while bits > 0 {
            bits = bits & (bits -1);
            count = count + 1;
        }
        count
    }
}

#[derive(Copy, Clone)]
pub struct Usize {
    usize: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    note: Note,
    gate: Gate,
}

#[derive(Debug, Clone, Copy)]
pub enum GateMode {
    Repeat,
    Sustain,
    Single,
    Silent,
}

#[derive(Debug, Clone, Copy)]
pub struct Stage {
    pub note: Note,
    pub pulse_count: u8,
    pub gate_mode: GateMode,
    pub skipped: bool,
}

impl Stage {
    pub fn default() -> Stage {
        Stage { note: Note::C, pulse_count: 1, gate_mode: GateMode::Repeat, skipped: false }
    }

    pub fn has_pulses(&self) -> bool {
        self.pulse_count > 0 && !self.skipped
    }
}
