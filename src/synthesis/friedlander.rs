/// The simplest form of a blast wave.
#[derive(Clone)]
pub struct FriedlanderWave {
    /// Initial delay of the blast.
    delay: f32,
    /// Positive phase duration.
    ppd: f32,
    /// Peak pressure.
    peak: f32,
    /// The agressiveness of the curve. Values between 1 and 2 seem to work best
    curve: f32,
    /// Multiplyer for time: high attack = quick wave
    frequency_mod: f32,
}

impl FriedlanderWave {
    /// Creates a new [`FriedlanderWave`] with the given properties.
    pub fn new(delay: f32, peak_pressure: f32, positive_phase_duration: f32, curve: f32) -> Self {
        Self {
            delay,
            peak: peak_pressure,
            ppd: positive_phase_duration,
            curve,
            frequency_mod: 8.0,
        }
    }

    /// Returns the pressure at given point in time.
    pub fn pressure(&self, time: f32) -> f32 {
        let t = time * self.frequency_mod;
        self.peak
            * (1.0 - (t - self.delay) / self.ppd)
            * self
                .curve
                .powf(-(t - self.delay) / self.ppd)
    }
}
