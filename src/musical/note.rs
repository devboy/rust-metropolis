#[derive(Debug, Clone, Copy)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Note {
    pub const COUNT: u8 = 12;

    pub fn index(&self) -> u8 {
        match self {
            Note::C => 1,
            Note::CSharp => 2,
            Note::D => 3,
            Note::DSharp => 4,
            Note::E => 5,
            Note::F => 6,
            Note::FSharp => 7,
            Note::G => 8,
            Note::GSharp => 9,
            Note::A => 10,
            Note::ASharp => 11,
            Note::B => 12
        }
    }

    pub fn distance(&self, b: Note) -> u8 {
        (self.index() as i8 - b.index() as i8).abs() as u8
    }
}
