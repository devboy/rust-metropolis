use crate::musical::note::Note;

impl Note {
    pub fn voltage(self) -> f32 {
        (self as i8 + 1) as f32 / Note::COUNT as f32
    }
}
