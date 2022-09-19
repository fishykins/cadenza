use super::*;

pub struct PythagoreanTemperament {
    /// Tuning note
    root: NoteMap,
    /// Number of notes per octave
    tone_count: Tone,
    /// Reference note for octave 0, note 0.
    octave_note_zero: Note,
}

impl PythagoreanTemperament {
    pub fn new(tone_count: Tone, reference_note: NoteMap) -> Self {
        let octave_note_zero = reference_note.note
            - reference_note.tone as Note
            - (reference_note.octave as Note * tone_count as Note);
        Self {
            tone_count,
            root: reference_note,
            octave_note_zero,
        }
    }

    pub fn root(&self) -> Tone {
        self.root.tone
    }

    pub fn standard() -> Self {
        Self::new(12, NoteMap::concert_a())
    }

    /// Translates the 'root' note for future calculations while maintaining its current frequency.
    pub fn move_root(&mut self, note: Note) {
        let freq = self.frequency(note);
        self.root = NoteMap::new(freq, note, self.tone(note), self.octave(note));
    }

    /// Gets the absolute ratio to given tone, which will result in crazy big numbers. Good for analysis, bad for pitching
    pub fn absolute_ratio(&self, tone: Tone) -> f32 {
        let interval = tone as f32 - self.root.tone as f32;
        // apply mod to interval to get the actual scale degree (explicitly done to avoid issues with negatives)
        let scale_degree = (interval
            - (self.tone_count as f32) * (interval / self.tone_count as f32).floor())
            as Note;

        let index = Circle::new(self.tone_count as usize)
            .index((scale_degree % self.tone_count as Note) as Tone);

        let r = if index > self.tone_count as usize / 2 {
            // LHS
            (2_f32 / 3_f32).powf(-(self.tone_count as f32 - index as f32))
        } else {
            // RHS
            (3_f32 / 2_f32).powi(index as i32)
        };
        r
    }

    /// Returns the tonal space ratio of this scale degree, bound to the "primary" octave
    pub fn tone_space_ratio(&self, tone: Tone) -> f32 {
        self.sanitize_ratio(self.absolute_ratio(tone))
    }

    /// Brings a ratio to within limits.
    // TODO: remove the while loops and do some real maths, you wimp
    pub fn sanitize_ratio(&self, ratio: f32) -> f32 {
        let mut r = ratio;
        if ratio < 1. {
            while r < 1. {
                r = r * 2.;
            }
            return r;
        } else if ratio > 2. {
            while r > 2. {
                r = r / 2.;
            }
            return r;
        }
        r
    }

    fn ratio(&self, note: Note) -> f32 {
        let interval = note - self.root.note;
        let tsr = self.tone_space_ratio(note as Tone);
        let octave_displacement = (interval as f32 / self.tone_count as f32).floor() as i32;
        tsr * 2_f32.powi(octave_displacement)
    }
}

impl Temperament for PythagoreanTemperament {
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
        return (my_root / self.tone_count as Note) as Octave;
    }

    fn tone(&self, note: Note) -> Tone {
        (note % self.tone_count as Note) as Tone
    }
}

impl Frequency for PythagoreanTemperament {
    fn frequency(&self, note: Note) -> Hz {
        self.root.frequency * self.ratio(note)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pythag_temp_test() {
        let py = PythagoreanTemperament::standard();
        let a = py.frequency(69);
        let e = py.frequency(69 + 7);
        assert_eq!(a.value(), 440.0);
        assert_eq!(e.value(), 440.0 * (3.0 / 2.0));
    }
}
