use crate::maths::Frequency;
use super::Temperament;
use super::CircleOfFifths;

pub struct PythagoreanTemperament {
    /// Tuning note
    pub refference_note: i16,
    /// Pitch of the tuning note
    pub refference_frequency: Frequency,
    /// Number of notes per octave
    pub tone_count: usize,
    /// The "root" or "tonic" that the scale is tuned to
    root: i16,

    cof: CircleOfFifths,
    root_freq_cashe: Frequency,
    
}

impl PythagoreanTemperament {
    pub fn new(tone_count: usize, refference_note: i16, refference_frequency: Frequency) -> Self {
        Self {
            refference_note,
            refference_frequency,
            tone_count,
            root: refference_note,
            cof: CircleOfFifths::new(tone_count, 0),
            root_freq_cashe: refference_frequency,
        }
    }

    pub fn set_root(&mut self, tone: i16) {
        self.root = tone;
        let ratio = self.ratio(self.refference_note);
        self.root_freq_cashe = self.refference_frequency / ratio;
    }

    pub fn root(&self) -> i16 {
        self.root
    }

    pub fn standard() -> Self {
        Self::new(12, 69, Frequency::tuning_standard())
    }

    /// Gets the absolute ratio to given tone, which will result in crazy big numbers. Good for analysis, bad for pitching
    pub fn absolute_ratio(&self, tone: i16) -> f32 {
        let interval = tone as f32 - self.root as f32;
        // apply mod to interval to get the actual scale degree (explicitly done to avoid issues with negatives)
        let scale_degree = (interval - (self.tone_count as f32) * (interval / self.tone_count as f32).floor()) as i16;

        let index = self.cof.index(scale_degree as usize); // this give us our power

        let r = if index > self.tone_count / 2 {
            // LHS
            ( 2_f32 / 3_f32).powf(-(self.tone_count as f32 - index as f32))
            
        } else {
            // RHS
            ( 3_f32 / 2_f32).powi(index as i32)
        };
        r
    }

    /// Returns the tonal space ratio of this scale degree, bound to the "primary" octave
    pub fn tonespace_ratio(&self, tone: i16) -> f32 {
        self.sanitise_ratio(self.absolute_ratio(tone))
    }

    /// Brings a ratio to within limits.
    // TODO: remove the while loops and do some real maths, you wimp
    pub fn sanitise_ratio(&self, ratio: f32) -> f32 {
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
}

impl Temperament for PythagoreanTemperament {
    fn ratio(&self, tone: i16) -> f32 {
        let interval = tone - self.root;
        let tsr = self.tonespace_ratio(tone);
        let octave_displacement = ((interval as f32 / self.tone_count as f32)).floor() as i32;
        tsr * 2_f32.powi(octave_displacement)
    }

    fn frequency(&self, tone: i16) -> Frequency {
        let ratio = self.ratio(tone); //ratio from tonic
        self.root_freq_cashe * ratio
    }
}

#[test]
fn pythag_temp_test() {
    let mut pythag = PythagoreanTemperament::standard();
    pythag.set_root(62); //Middle D

    let mut sampler = crate::output::Sampler::new(2);
    sampler.add_wave(crate::output::Wave::freq(pythag.frequency(62)));
    sampler.add_wave(crate::output::Wave::freq(pythag.frequency(66)));
    sampler.add_wave(crate::output::Wave::freq(pythag.frequency(69)));
    sampler.render();
}