use crate::musical::note::Note;
use crate::sequencer::sequencer::GateMode;

impl Note {
    pub fn voltage(self) -> f32 {
        (self as i8 + 1) as f32 / Note::COUNT as f32
    }
}

impl GateMode {
    pub fn from_float(f: f32) -> GateMode {
        if f < 0.25 {
            GateMode::Repeat
        } else if f < 0.5 {
            GateMode::Sustain
        } else if f < 0.75 {
            GateMode::Single
        } else {
            GateMode::Silent
        }
    }
}
