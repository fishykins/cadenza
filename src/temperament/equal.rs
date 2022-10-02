use super::*;

pub struct EqualTemperament {
    /// The number of tones per octave.
    tone_count: Tone,
    /// The reference note.
    reference: NoteMap,
    /// Reference note for octave 0, note 0.
    octave_note_zero: Note,
}

impl EqualTemperament {
    pub fn new(tone_count: Tone, reference_note: NoteMap) -> Self {
        let octave_note_zero = reference_note.note
            - reference_note.tone as Note
            - (reference_note.octave as Note * tone_count as Note);

        Self {
            reference: reference_note,
            tone_count,
            octave_note_zero,
        }
    }

    pub fn tet() -> Self {
        Self::new(12, NoteMap::concert_a())
    }

    pub fn reference_note(&self) -> Note {
        self.reference.note
    }

    fn ratio(&self, tone: Note) -> f32 {
        (tone as f32 / self.tone_count as f32).exp2()
    }
}

impl Frequency for EqualTemperament {
    fn frequency(&self, note: Note) -> Hz {
        let offset = note - self.reference.note;
        self.reference.frequency * self.ratio(offset)
    }
}

impl Temperament for EqualTemperament {
    fn tone_count(&self) -> usize {
        self.tone_count as usize
    }
    fn note(&self, oct: Octave, tone: Tone) -> Note {
        let oct_mul = oct as Note * self.tone_count as Note;
        oct_mul + tone as Note + self.octave_note_zero
    }

    fn octave(&self, note: Note) -> Octave {
        let remainder = note % self.tone_count as Note;
        let my_root = note - remainder - self.octave_note_zero;
        return (my_root / self.tone_count as Note) as Octave
    }

    fn tone(&self, note: Note) -> Tone {
        (note % self.tone_count as Note) as Tone
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tet() {
        let tet = EqualTemperament::tet();
        assert_eq!(tet.reference_note(), 69);
        assert_eq!(tet.frequency(75), Hz::new(622.25397));

        assert_eq!(tet.note(2, 5), 41);
        assert_eq!(tet.note(-1, 11), 11);
        assert_eq!(tet.tone(69), 9);
        assert_eq!(tet.octave(69), 4);
    }
}
