mod hz;
mod limit;
mod note;

pub use hz::Hz;
pub use limit::LimitTuning;
pub use note::NoteMap;

/// Denotes an octave.
pub type Octave = i32;
/// Denotes a note within the octave.
pub type Tone = u8;
/// Denotes a raw note value that takes octave into account.
pub type Note = i64;

pub trait Frequency {
    fn frequency(&self, note: Note) -> Hz;
    //fn note(&self, hz: Hz) -> Option<Note>;
}
