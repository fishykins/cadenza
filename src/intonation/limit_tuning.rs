use crate::maths::{Frequency, PRIMES};
use std::fmt::Display;

// ? Look into "Regular Numbers"

/// "x Limit Tuning" system, which is quite nifty. Typically this consists of the first three primes, but we can go as deep as we like!
/// each prime to a power results in an interval. e.g. 2^x, where x is the number of octaves
pub struct LimitTuning {
    primes: Vec<f32>,
    ratio: f32,
}

impl LimitTuning {
    pub fn new() -> Self {
        Self {
            primes: Vec::new(),
            ratio: 1.,
        }
    }

    pub fn add_prime(&mut self, value: f32) {
        if self.primes.len() < 32 {
            self.primes.push(value);
            self.calc_ratio();
        } else {
            panic!("Too many primes- calm down and be sensible!");
        }
    }

    pub fn limit(&self) -> usize {
        self.primes.len()
    }

    fn calc_ratio(&mut self) {
        let depth = self.primes.len();
        if depth == 0 {
            self.ratio = 1.;
            return;
        }

        let mut ratio = 1.;

        for (i, v) in self.primes.iter().enumerate() {
            let n = (PRIMES[depth - i - 1] as f32).powf(*v);
            ratio *= n;
        }
        self.ratio = ratio;
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn apply(&self, freq: f32) -> f32 {
        freq * self.ratio
    }
}

impl Display for LimitTuning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.primes)
    }
}


#[macro_export]
macro_rules! limitTuning {
    ( $( $x:expr ),* ) => {
        {
            let mut lts = LimitTuning::new();
            $(
                lts.add_prime($x);
            )*
            lts
        }
    };
}

#[test]
fn name() {
    let major_third_up = limitTuning![1.,0.,-2.];
    println!("3rd = {}, ratio = {}", major_third_up, major_third_up.ratio());
    let c = Frequency::new(256.);
    let e = c.apply_lts(major_third_up);
    println!("c = {}, e = {}", c, e);
    assert_eq!(e, Frequency::new(320.));

}