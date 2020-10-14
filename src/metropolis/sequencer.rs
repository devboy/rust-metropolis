use heapless::{ArrayLength, Vec};

use crate::musical::gate::Gate;
use crate::musical::note::Note;

#[derive(Debug, Clone)]
pub struct Sequencer<N> where N: ArrayLength<Stage> {
    config: Config<N>,
    pulse_position: u8,
}

impl<N> Sequencer<N> where N: ArrayLength<Stage> {
    pub fn new() -> Sequencer<N> {
        Self { pulse_position: 0, config: Config::new() }
    }

    pub fn update(&mut self, cfg: Config<N>) -> State {
        self.config = cfg;
        let stage = self.current_stage();
        State { note: stage.note, gate: Gate::Open }
    }

    pub fn cfg(&mut self) -> &mut Config<N> {
        &mut self.config
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

#[derive(Debug, Clone)]
pub struct Config<N> where N: ArrayLength<Stage> {
    stages: Vec<Stage, N>,
}

impl<N> Config<N> where N: ArrayLength<Stage>
{
    pub fn new() -> Self {
        Self { stages: Vec::<Stage, N>::new() }
    }

    pub fn stage(&mut self, index: usize) -> Option<&mut Stage> {
        self.stages.get_mut(index)
    }

    pub fn pulse_count(&self) -> u8 {
        let mut counts = 0_u8;
        for stage in self.stages.iter() {
            if !stage.skipped {
                counts += stage.pulse_count
            };
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
