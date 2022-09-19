use crate::pitch::*;

mod equal;
mod pythag;
mod circle;

pub use circle::Circle;
pub use equal::EqualTemperament;
pub use pythag::*;

pub trait Temperament {
    /// The number of tones per octave.
    fn tone_count(&self) -> usize;
    /// Gets the note value, using octave and tone.
    fn note(&self, oct: Octave, tone: Tone) -> Note;
    /// Gets the octave number of the given note.
    fn octave(&self, note: Note) -> Octave;
    /// Gets the tone value for the given note.
    fn tone(&self, note: Note) -> Tone;
}
