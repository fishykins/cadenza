use crate::pitch::*;

use super::Temperament;

/// A [`Circle`] is used to iterate over a tone system using a base interval.
/// This is usually done using the "Circle of Fifths" over a twelve-tone system, but there is no reason it
/// can't be used for any other sized tonal system, or by any other interval other than 7 semi-tones.
#[derive(Debug, Clone)]
pub struct Circle {
    interval: usize,
    tone_count: usize,
    i: usize,
}

impl Circle {
    /// Builds a new [`Circle`] that will step by the given interval. To avoid potentially creating imperfect circles, use [`new`] instead.
    /// The iterator will only iterate n times, where n is equal to the tone count. Imperfect circles will therefor potentially fail to return all possible tones.
    pub fn new_raw(interval: Tone, tone_count: usize) -> Self {
        Self {
            interval: interval as usize,
            tone_count,
            i: 0,
        }
    }

    /// Calculates a [`Circle`] that will iterate over the given number of tones exactly once.
    pub fn new(tone_count: usize) -> Self {
        Self::new_raw(circle_step(tone_count) as Tone, tone_count)
    }

    /// The classic circle of fifths.
    pub fn circle_of_fifths() -> Self {
        Self::new_raw(7, 12)
    }

    /// Calculates a circle that will iterate over each tone of the given [`Temperament`] exactly once.
    pub fn from_temperament(temperament: &dyn Temperament) -> Self {
        Self::new(temperament.tone_count())
    }
}

impl Iterator for Circle {
    type Item = Tone;

    fn next(&mut self) -> Option<Self::Item> {
        let t = (self.i * self.interval) % self.tone_count;
        self.i += 1;
        if self.i < self.tone_count {
            return Some(t as Tone);
        }
        self.i = 0;
        None
    }   
}

/// Calculates the step size of a given octave of size i. For example, in conventional harmony, the circle of fifths has a step size of 7 semitones.
/// This step size will result in every octave index being visited exactly once, no matter how many notes you have.
/// WARNING: This function will panic if the value of i is less than 2.
pub fn circle_step(i: usize) -> usize {
    (i / 2) + (4 - ((i - 2) % 4)) / 4 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_step_test() {
        assert_eq!(7, circle_step(12));
    }

    #[test]
    #[should_panic]
    fn circle_step_overflow_test() {
        circle_step(1);
    }

    #[test]
    fn circle_test() {
        let mut results = Vec::new();
        for t in Circle::circle_of_fifths() {
            results.push(t);
        }
        assert_eq!(vec![0,7,2,9,4,11,6,1,8,3,10], results);
    }
}
