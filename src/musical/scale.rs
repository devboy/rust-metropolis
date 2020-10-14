use crate::musical::note::Note;
use crate::musical::note::Note::*;

// Scales in Manual:
// Chromatic CHrOM 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
// Major MAJOr 0, 2, 4, 5, 7, 9, 11
// Minor MInOr 0, 2, 3, 5, 7, 8, 10
// Dorian dOrIA 0, 2, 3, 5, 7, 9, 10
// Mixolydian MIXOL 0, 2, 4, 5, 7, 9, 10
// Lydia LYdIA 0, 2, 4, 6, 7, 9, 11
// Phrygian PHrYG 0, 1, 3, 5, 7, 8, 10
// Locrian LOCrI 0, 1, 3, 4, 7, 8, 10
// Diminished dIMIn 0, 1, 3, 4, 6, 7, 9, 10
// Whole-Half -HALF 0, 2, 3, 5, 6, 8, 9, 11
// Whole Tone -HOLE 0, 2, 4, 6, 8, 10
// Minor Blues bLUES 0, 3, 5, 6, 7, 10
// Minor Pentatonic PEnT- 0, 3, 5, 7, 10
// Major Pentatonic PEnTA 0, 2, 4, 7, 9
// Harmonic Minor HArMI 0, 2, 3, 5, 7, 8, 11
// Melodic Minor MELMI 0, 2, 3, 5, 7, 9, 11
// Super Locrian SULOC 0, 1, 3, 4, 6, 8, 10
// Arabic / Bhairav ArAbI 0, 1, 4, 5, 7, 8, 11
// Hungarian Minor HUnGA 0, 2, 3, 6, 7, 8, 11
// Minor Gypsy GYPSY 0, 1, 4, 5, 7, 8, 10
// Hirojoshi HIrOJ 0, 2, 3, 7, 8
// In-Sen InSEn 0, 1, 5, 7, 10
// Japanese / Iwato JAPAn 0, 1, 5, 6, 10
// Kumoi KUMOI 0, 2, 3, 7, 9
// Pelog PELOG 0, 1, 3, 4, 7, 8
// Spanish SPAIn 0, 1, 3, 4, 5, 6, 8, 10
// Tritone 3TOnE 0, 1, 4, 6, 7, 10
// Prometheus PrOME 0, 2, 4, 6, 9, 10
// Augmented AUGME 0, 3, 4, 7, 8, 11
// Enigmatic EnIGM 0, 1, 4, 6, 8, 10, 11
pub enum Scale {
    Chromatic,
    Bassline,
    Octave,
    PentatonicMajor,
    PentatonicMinor,
}

impl Scale {
    fn notes(&self) -> &'static [Note] {
        match self {
            Scale::Chromatic => &[C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B],
            Scale::Bassline => &[C, G, ASharp],
            Scale::Octave => &[C],
            Scale::PentatonicMajor => &[C, D, E, G, A],
            Scale::PentatonicMinor => &[C, DSharp, F, G, ASharp],
        }
    }

    pub fn quantize(&self, input: Note) -> Note {
        let notes: &'static [Note] = self.notes();
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
