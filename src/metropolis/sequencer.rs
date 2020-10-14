use heapless::{ArrayLength, Vec};

use crate::musical::gate::Gate;
use crate::musical::note::Note;

#[derive(Debug, Clone)]
pub struct Sequencer<N> where N: ArrayLength<Stage> {
    config: Config<N>,
    position: Position,
}

impl<N> Sequencer<N> where N: ArrayLength<Stage> {
    pub fn new() -> Sequencer<N> {
        Self { position: Position { stage: 0, pulse: 0 }, config: Config::new() }
    }

    pub fn config(&mut self) -> &mut Config<N> {
        &mut self.config
    }

    pub fn step(&mut self) {
        let current_stage = self.stage(self.position).expect("stage should exist");
        if self.position.pulse < current_stage.pulse_count - 1 {
            self.position = Position{stage: self.position.stage, pulse: self.position.pulse + 1 }
        } else {
            self.position = self.next_stage_position(self.position)
        }
    }

    fn next_stage_position(&self, position: Position) -> Position {
        //TODO: Implement other than forward
        //TODO: Take skipped stages into account
        let stage_count = self.config.stage_count();
        if position.stage < stage_count - 1 {
            Position{stage: position.stage + 1, pulse: 0 }
        } else {
            Position{stage: 0, pulse: 0 }
        }
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

#[derive(Debug, Clone, Copy)]
struct Position {
    stage: u8,
    pulse: u8,
}

#[derive(Debug, Clone)]
pub struct Config<N> where N: ArrayLength<Stage> {
    stages: Vec<Stage, N>,
    stage_mode: StageMode,
}

impl<N> Config<N> where N: ArrayLength<Stage>
{
    pub fn new() -> Self {
        Self { stages: Vec::<Stage, N>::new(), stage_mode: StageMode::Forward }
    }

    pub fn stage(&mut self, index: usize) -> Option<&mut Stage> {
        self.stages.get_mut(index)
    }

    pub fn stages(&self) -> &Vec<Stage, N> {
        &self.stages
    }

    pub fn stage_count(&self) -> u8 {
        self.stages.len() as u8
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
        Stage { note: Note::C, pulse_count: 1, gate_mode: GateMode::Repeat, skipped: false }
    }
}
