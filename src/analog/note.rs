use crate::musical::note::{Note};

impl Note {
    pub fn voltage(&self) -> f32 {
        self.index() as f32 / Note::COUNT as f32
    }
}
