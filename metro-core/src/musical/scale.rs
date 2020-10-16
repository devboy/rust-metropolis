use crate::musical::note::Note;
use crate::musical::note::Note::*;
use micromath::F32Ext;

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    Chromatic = 0,
    Major = 1,
    Minor = 2,
    Dorian = 3,
    Mixolydian = 4,
    Lydia = 5,
    Phrygian = 6,
    Locrian = 7,
    Diminished = 8,
    WholeHalf = 9,
    WholeTone = 10,
    MinorBlues = 11,
    MinorPentatonic = 12,
    MajorPentatonic = 13,
    HarmonicMinor = 14,
    MelodicMinor = 15,
    SuperLocrian = 16,
    Arabic = 17,
    HungarianMinor = 18,
    MinorGypsy = 19,
    Hirojoshi = 20,
    InSen = 21,
    Japanese = 22,
    Kumoi = 23,
    Pelog = 24,
    Spanish = 25,
    Tritone = 26,
    Prometheus = 27,
    Augmented = 28,
    Enigmatic = 29,
}

static CHROMATIC_NOTES: [Note; 12] = [C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B];
static MAJOR: [Note; 7] = [C, D, E, F, G, A, B];
static MINOR: [Note; 7] = [C, D, DSharp, F, G, GSharp, ASharp];
static DORIAN: [Note; 7] = [C, D, DSharp, F, G, A, ASharp];
static MIXOLYDIAN: [Note; 7] = [C, D, E, F, G, A, ASharp];
static LYDIA: [Note; 7] = [C, D, E, GSharp, G, A, B];
static PHRYGIAN: [Note; 7] = [C, CSharp, DSharp, F, G, GSharp, ASharp];
static LOCRIAN: [Note; 7] = [C, CSharp, DSharp, E, G, GSharp, ASharp];
static DIMINISHED: [Note; 8] = [C, CSharp, DSharp, E, GSharp, G, A, ASharp];
static WHOLE_HALF: [Note; 8] = [C, D, DSharp, F, GSharp, GSharp, A, B];
static WHOLE_TONE: [Note; 6] = [C, D, E, GSharp, GSharp, ASharp];
static MINOR_BLUES: [Note; 6] = [C, DSharp, F, GSharp, G, ASharp];
static MINOR_PENTATONIC: [Note; 5] = [C, DSharp, F, G, ASharp];
static MAJOR_PENTATONIC: [Note; 5] = [C, D, E, G, A];
static HARMONIC_MINOR: [Note; 7] = [C, D, DSharp, F, G, GSharp, B];
static MELODIC_MINOR: [Note; 7] = [C, D, DSharp, F, G, A, B];
static SUPER_LOCRIAN: [Note; 7] = [C, CSharp, DSharp, E, GSharp, GSharp, ASharp];
static ARABIC: [Note; 7] = [C, CSharp, E, F, G, GSharp, B];
static HUNGARIAN_MINOR: [Note; 7] = [C, D, DSharp, GSharp, G, GSharp, B];
static MINOR_GYPSY: [Note; 7] = [C, CSharp, E, F, G, GSharp, ASharp];
static HIROJOSHI: [Note; 5] = [C, D, DSharp, G, GSharp];
static IN_SEN: [Note; 5] = [C, CSharp, F, G, ASharp];
static JAPANESE: [Note; 5] = [C, CSharp, F, GSharp, ASharp];
static KUMOI: [Note; 5] = [C, D, DSharp, G, A];
static PELOG: [Note; 6] = [C, CSharp, DSharp, E, G, GSharp];
static SPANISH: [Note; 8] = [C, CSharp, DSharp, E, F, GSharp, GSharp, ASharp];
static TRITONE: [Note; 6] = [C, CSharp, E, GSharp, G, ASharp];
static PROMETHEUS: [Note; 6] = [C, D, E, GSharp, A, ASharp];
static AUGMENTED: [Note; 6] = [C, DSharp, E, G, GSharp, B];
static ENIGMATIC: [Note; 7] = [C, CSharp, E, GSharp, GSharp, ASharp, B];

impl Scale {
    pub fn notes(self) -> &'static [Note] {
        match self {
            Scale::Chromatic => &CHROMATIC_NOTES,
            Scale::Major => &MAJOR,
            Scale::Minor => &MINOR,
            Scale::Dorian => &DORIAN,
            Scale::Mixolydian => &MIXOLYDIAN,
            Scale::Lydia => &LYDIA,
            Scale::Phrygian => &PHRYGIAN,
            Scale::Locrian => &LOCRIAN,
            Scale::Diminished => &DIMINISHED,
            Scale::WholeHalf => &WHOLE_HALF,
            Scale::WholeTone => &WHOLE_TONE,
            Scale::MinorBlues => &MINOR_BLUES,
            Scale::MinorPentatonic => &MINOR_PENTATONIC,
            Scale::MajorPentatonic => &MAJOR_PENTATONIC,
            Scale::HarmonicMinor => &HARMONIC_MINOR,
            Scale::MelodicMinor => &MELODIC_MINOR,
            Scale::SuperLocrian => &SUPER_LOCRIAN,
            Scale::Arabic => &ARABIC,
            Scale::HungarianMinor => &HUNGARIAN_MINOR,
            Scale::MinorGypsy => &MINOR_GYPSY,
            Scale::Hirojoshi => &HIROJOSHI,
            Scale::InSen => &IN_SEN,
            Scale::Japanese => &JAPANESE,
            Scale::Kumoi => &KUMOI,
            Scale::Pelog => &PELOG,
            Scale::Spanish => &SPANISH,
            Scale::Tritone => &TRITONE,
            Scale::Prometheus => &PROMETHEUS,
            Scale::Augmented => &AUGMENTED,
            Scale::Enigmatic => &ENIGMATIC,
        }
    }

    pub fn quantize_float(self, input: f32) -> Note {
        let notes = self.notes();
        let max_index = (notes.len() - 1).max(0);
        let index= F32Ext::round(clamp(input,0.0, 1.0) * max_index as f32) as usize;
        notes[index]
    }

    pub fn quantize(self, input: Note) -> Note {
        let notes = self.notes();
        let mut min_distance = u8::MAX;
        let mut output = input;
        for note in notes {
            let distance = note.distance(input);
            if distance == 0 {
                return input;
            }
            if distance < min_distance {
                min_distance = distance;
                output = note.clone();
            }
        };
        return output;
    }
}

fn clamp(num: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    let mut x = num;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}
