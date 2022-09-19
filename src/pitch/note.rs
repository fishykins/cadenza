use super::*;

/// A data map that helps convert between a raw note index, octave/tone index and frequency.
pub struct NoteMap {
    /// The frequency of this note.
    pub frequency: Hz,
    pub note: Note,
    pub tone: Tone,
    pub octave: Octave,
}

impl NoteMap {
    pub fn new(frequency: Hz, note: Note, tone: Tone, octave: Octave) -> Self {
        Self {
            frequency,
            note,
            tone,
            octave,
        }
    }

    pub fn concert_a() -> Self {
        Self::new(Hz::new(440.0), 69, 9, 4)
    }
}