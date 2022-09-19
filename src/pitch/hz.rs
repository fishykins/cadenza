use std::{ops::{Add, Sub, Mul, Div}, fmt::Display};
type Float = f32;

/// The number of hertz equals the number of cycles per second.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Hz(Float);

impl Hz {
    pub fn new(hz: Float) -> Hz {
        Hz(hz)
    }

    pub fn tuning_standard() -> Self {
        Self(440.)
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    pub fn harmonic(&self, h: usize) -> Self {
        Self::new(self.0 * (h + 1) as f32)
    }

    pub fn harmonics(&self, len: usize) -> Vec<Self> {
        let mut harmonics = Vec::new();
        for i in 0 .. len {
            harmonics.push(self.harmonic(i))
        }
        harmonics
    }
}

impl Display for Hz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}Hz", self.0)
    }
}

impl Into<Float> for Hz {
    fn into(self) -> Float {
        self.0
    }
}

impl From<Float> for Hz {
    fn from(hz: Float) -> Hz {
        Hz(hz)
    }
}

impl Add<Float> for Hz {
    type Output = Self;
    fn add(self, rhs: Float) -> Self::Output {
        Self(self.0 + rhs)
    }   
}

impl Add for Hz {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Float> for Hz {
    type Output = Self;
    fn sub(self, rhs: Float) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub for Hz {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Float> for Hz {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul for Hz {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div<f32> for Hz {
    type Output = Self;
    fn div(self, rhs: Float) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Div for Hz {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}