use crate::intonation::LimitTuning;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Frequency(pub f32);

impl Frequency {
    pub fn new(f: f32) -> Self {
        Self(f)
    }

    pub fn apply_lts(&self, lts: LimitTuning) -> Frequency {
        Frequency::new(lts.apply(self.0))
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Hz", self.0)
    }
}