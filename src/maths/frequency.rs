use crate::intonation::LimitTuning;
use std::fmt::Display;
use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Frequency(pub f32);

impl Frequency {
    pub fn new(f: f32) -> Self {
        Self(f)
    }

    pub fn tuning_standard() -> Self {
        Self(440.)
    }

    pub fn apply_lts(&self, lts: LimitTuning) -> Frequency {
        Frequency::new(lts.apply(self.0))
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn haronic(&self, h: usize) -> Self {
        Self::new(self.0 * (h + 1) as f32)
    }

    pub fn haronics(&self, len: usize) -> Vec<Self> {
        let mut harmonics = Vec::new();
        for i in 0 .. len {
            harmonics.push(self.haronic(i))
        }
        harmonics
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Hz", self.0)
    }
}

impl Div<f32> for Frequency {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Frequency::new(self.value() / rhs)
    }
}

impl Mul<f32> for Frequency {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Frequency::new(self.value() * rhs)
    }
}

impl Mul<usize> for Frequency {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Frequency::new(self.value() * rhs as f32)
    }
}

impl Div<usize> for Frequency {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        Frequency::new(self.value() / rhs as f32)
    }
}