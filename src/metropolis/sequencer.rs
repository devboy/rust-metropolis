use crate::musical::gate::Gate;
use crate::musical::note::Note;

pub struct Sequencer {
    pulse_position: u8,
    config: Config,
}

impl Sequencer {
    pub fn new(cfg: Config) -> Sequencer {
        Self { pulse_position: 0, config: cfg }
    }

    pub fn update(&mut self, cfg: Config) -> State {
        self.config = cfg;
        let stage = self.current_stage();
        State { note: stage.note, gate: Gate::Open }
    }

    pub fn step(&mut self) {
        self.next()
    }

    fn current_stage(&self) -> &Stage {
        let mut passed_stages = 0_u8;
        for i in 0..=self.pulse_position {
            let stage = self.config.stages.get(passed_stages as usize).expect("not enough stages for position");
            if i < stage.pulse_count + passed_stages {
                return stage;
            } else {
                passed_stages += 1;
            };
        };
        unreachable!()
    }

    fn next(&mut self) {
        self.pulse_position = match self.pulse_position + 1 {
            x if x < self.config.pulse_count() => x,
            _ => 0,
        }
    }
}

pub struct Config {
    stages: [Stage; 64],
}

impl Config {
    pub fn new() -> Self {
        Self { stages: [Stage::default(); 64] }
    }

    pub fn pulse_count(&self) -> u8 {
        let mut counts = 0_u8;
        for stage in self.stages.iter() {
            counts += stage.pulse_count
        }
        return counts;
    }
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
        Stage { note: Note::C, pulse_count: 1, gate_mode: GateMode::Repeat, skipped: true }
    }
}
