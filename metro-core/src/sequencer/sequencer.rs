use crate::musical::gate::Gate;
use crate::musical::note::Note;
use crate::sequencer::stage_mode::StageMode;

const N: usize = 8;

#[derive(Debug, Clone)]
pub struct Sequencer {
    config: Config,
    pos: Position,
}

impl Sequencer {
    pub fn new() -> Sequencer {
        Self { pos: Position { stage: 0, pulse: 0, dir: Direction::Forward }, config: Config::new() }
    }

    pub fn config(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn state(&self, last_beat_ms: u32) -> State {
        let current_stage = self.stage(self.pos).expect("stage should exist");
        let gate = current_stage.gate_mode.gate(self.config.gate_time_ms, last_beat_ms, self.pos.pulse == 0, self.pos.pulse > current_stage.pulse_count - 1);
        State { gate, note: current_stage.note, pos: self.pos }
    }

    pub fn step(&mut self) {
        if !self.config.has_pulses() { return; }

        let current_stage = self.stage(self.pos).expect("stage should exist");
        if self.pos.pulse < current_stage.pulse_count - 1 && !current_stage.skipped {
            self.pos = Position { stage: self.pos.stage, pulse: self.pos.pulse + 1, dir: self.pos.dir }
        } else {
            self.pos = self.next_stage_pos(self.pos)
        }
    }

    fn next_stage_pos(&self, pos: Position) -> Position {
        self.config.stage_mode.next_stage(self.config.has_pulses_mask(), pos)
    }

    fn stage(&self, pos: Position) -> Option<&Stage> {
        self.config.stages.get(pos.stage as usize)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub stage: u8,
    pub pulse: u8,
    pub dir: Direction,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Forward = 0,
    Reverse = 1,
}

impl Direction {
    pub fn invert(self) -> Self {
        match self {
            Self::Forward => Self::Reverse,
            Self::Reverse => Self::Forward,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    stages: [Stage; N],
    stage_mode: StageMode,
    gate_time_ms: u32,
}

impl Config
{
    pub fn new() -> Self {
        Self { stages: [Stage::default(); N], stage_mode: StageMode::Forward, gate_time_ms: 50 }
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

    pub fn set_gate_time_ms(&mut self, gate_time_ms: u32) {
        self.gate_time_ms = gate_time_ms
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaskU8(pub u8);

impl MaskU8 {
    pub fn new() -> MaskU8 { Self(0) }

    pub fn next_higher(&self, pos: u8) -> Option<u8> {
        for i in pos + 1..8 {
            if self.0 & (1 << i) > 0 {
                return Some(i);
            }
        };
        None
    }

    pub fn next_lower(&self, pos: u8) -> Option<u8> {
        if pos == 0 { return None; }
        for i in (0..pos).rev() {
            if self.0 & (1 << i) > 0 {
                return Some(i);
            }
        };
        None
    }

    pub fn highest(&self) -> Option<u8> {
        self.next_lower(8)
    }

    pub fn lowest(&self) -> Option<u8> {
        if self.0 & 1 > 0 {
            Some(0)
        } else {
            self.next_higher(0)
        }
    }

    pub fn is_set(&self, pos: u8) -> bool {
        self.0 & (1 << pos) > 0
    }

    pub fn count(&self) -> u8 {
        let mut count = 0;
        let mut bits = self.0;
        while bits > 0 {
            bits = bits & (bits - 1);
            count = count + 1;
        }
        count
    }
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub note: Note,
    pub gate: Gate,
    pub pos: Position,
}

#[derive(Debug, Clone, Copy)]
pub enum GateMode {
    Repeat,
    Sustain,
    Single,
    Silent,
}

impl GateMode {
    pub fn gate(self, gate_time_ms: u32, last_beat_ms: u32, first_pulse: bool, last_pulse: bool) -> Gate {
        match self {
            GateMode::Repeat if gate_time_ms >= last_beat_ms => Gate::Open,
            GateMode::Single if gate_time_ms >= last_beat_ms && first_pulse => Gate::Open,
            GateMode::Sustain if !last_pulse || gate_time_ms >= last_beat_ms => Gate::Open,
            _ => Gate::Closed,
        }
    }
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
