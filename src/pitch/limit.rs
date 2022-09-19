use crate::pitch::Hz;
use crate::maths::PRIMES;
use std::fmt::Display;

// ? Look into "Regular Numbers"

#[macro_export]
macro_rules! limitTuning {
    ( $( $x:expr ),* ) => {
        {
            let mut lts = LimitTuning::new();
            $(
                lts.add_power($x);
            )*
            lts
        }
    };
}

/// "x Limit Tuning" system, which is quite nifty. Typically this consists of the first three primes, but we can go as deep as we like!
/// each prime to a power results in an interval. e.g. 2^x, where x is the number of octaves.
pub struct LimitTuning {
    powers: Vec<i32>,
    ratio: f32,
}

impl LimitTuning {
    pub fn new() -> Self {
        Self {
            powers: Vec::new(),
            ratio: 1.,
        }
    }

    pub fn major_third() -> Self {
        limitTuning![1, 0, -2]
    }

    pub fn add_power(&mut self, value: i32) {
        if self.powers.len() < 32 {
            self.powers.push(value);
            self.calc_ratio();
        } else {
            panic!("Too many primes- calm down and be sensible!");
        }
    }

    pub fn limit(&self) -> usize {
        self.powers.len()
    }

    fn calc_ratio(&mut self) {
        let depth = self.powers.len();
        if depth == 0 {
            self.ratio = 1.;
            return;
        }

        let mut ratio = 1.;

        for (i, v) in self.powers.iter().enumerate() {
            let n = (PRIMES[depth - i - 1] as f32).powi(*v);
            ratio *= n;
        }
        self.ratio = ratio;
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn apply(&self, hz: Hz) -> Hz {
        hz * self.ratio
    }
}

impl Display for LimitTuning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.powers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pitch::Hz;

    #[test]
    fn major_third_test() {
        let major_third_up = limitTuning![1, 0, -2];
        let major_third_up2 = limitTuning![0, 4, -6];
        let c = Hz::new(256.);
        let e = major_third_up.apply(c);
        let e2 = major_third_up2.apply(c);
        assert_eq!(e, Hz::new(320.));
        assert_eq!(e2, Hz::new(324.));
    }

    // #[test]
    // fn seven_limit_tuning() {
    //     let interval = limitTuning![1, 0, 1, 0, -2];
    //     println!("interval = {}, ratio = {}", interval, interval.ratio());
    //     let c = Hz::new(256.);
    //     let e = interval.apply(c);
    //     println!("c = {}, interval = {}", c, e);
    // }
}
