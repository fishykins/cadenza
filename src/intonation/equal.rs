use crate::maths::Frequency;
use super::Temperament;

pub struct EqualTemperament {
    refference_note: i16,
    refference_frequency: Frequency,
    tone_count: usize,
}

impl EqualTemperament {
    pub fn new(tone_count: usize, refference_note: i16, refference_frequency: Frequency) -> Self {
        Self {
            refference_note,
            refference_frequency,
            tone_count,
        }
    }

    pub fn standard() -> Self {
        Self::new(12, 69, Frequency::tuning_standard())
    }
}

impl Temperament for EqualTemperament {
    fn ratio(&self, tone: i16) -> f32 {
        2_f32.powf(tone as f32 / self.tone_count as f32)
    }

    fn frequency(&self, tone: i16) -> Frequency {
        let offset = tone - self.refference_note;
        self.refference_frequency * self.ratio(offset)
    }
}

#[test]
fn equaltemp_test() {
    let eq = EqualTemperament::standard();
    let c5 = eq.frequency(72);
    println!("C5 = {}", c5);
}