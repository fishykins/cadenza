#[macro_use]
use crate::limitTuning;
use crate::intonation::LimitTuning;
use crate::maths::Frequency;
use std::f32::consts::PI;
use std::i16;
use hound;

const SAMPLE_RATE: u32 = 44100;

pub struct Wave {
    pub root: Frequency,
    pub ratio: f32,
    pub faze: f32,
}

pub struct Sampler {
    sample_rate: u32,
    channels: Vec<Wave>,
    length: u32,
}

impl Wave {
    pub fn new(root: Frequency, ratio: f32, faze: f32) -> Self {
        Self {
            root,
            ratio,
            faze,
        }
    }

    pub fn freq(root: Frequency) -> Self {
        Self {
            root,
            ratio: 1.,
            faze: 1.,
        }
    }
}

impl Sampler {
    pub fn new(length: u32) -> Self {
        Self {
            sample_rate: SAMPLE_RATE,
            channels: Vec::new(),
            length,
        }
    }

    pub fn add_wave(&mut self, wave: Wave) {
        self.channels.push(wave);
    }

    pub fn render(&self) {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

        for t in (0 .. (self.sample_rate * self.length)).map(|x| x as f32 / SAMPLE_RATE as f32) {

            let mut sample = 0.;
            for (i, wave) in self.channels.iter().enumerate() {
                let x = sinwave(t + wave.faze, wave.root.0, wave.ratio);
                if i % 2 == 0 {
                    sample += x;
                } else {
                    sample -= x;
                }
            }

            let amplitude = i16::MAX as f32 / self.channels.len() as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
    }
}

fn sinwave(time: f32, root:f32, ratio: f32) -> f32 {
    (time * root * 2. *  PI * ratio).sin()
}



#[test]
fn wave_test() {
    let mut sampler = Sampler::new(4);
    let note = Frequency::new(256.);

    sampler.add_wave(Wave::freq(note));
    sampler.add_wave(Wave::new(note, 5./4., 1.));
    sampler.add_wave(Wave::new(note, 3./2., 1.));
    sampler.add_wave(Wave::new(note, 9./5., 1.));

    sampler.render();
}
