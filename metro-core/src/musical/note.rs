#[derive(Debug, Clone, Copy)]
pub enum Note {
    C = 0,
    CSharp = 1,
    D = 2,
    DSharp = 3,
    E = 4,
    F = 5,
    FSharp = 6,
    G = 7,
    GSharp = 8,
    A = 9,
    ASharp = 10,
    B = 11,
}

impl Note {
    pub const COUNT: u8 = 12;

    pub fn distance(self, b: Note) -> u8 {
        (self as i8 - b as i8).abs() as u8
    }
}
