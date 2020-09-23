use crate::maths::Frequency;
use std::f32::consts::PI;
use std::i16;
use hound;
use prima::render::*;

const SAMPLE_RATE: u32 = 44100;
const IMAGE_WIDTH: u32 = 8192;
const IMAGE_HEIGHT: u32 = 258;

const COLOURS: [[u8; 3]; 7] = [[255,255,0], [255,0,255], [255,0,0], [0,255,0], [0,0,255], [0,255,255], [124,64,255]];

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
        let mut image = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
        let image_height_half = IMAGE_HEIGHT as f32 / 2.;
        let image_amplitude = image_height_half / self.channels.len() as f32;
        let image_width_amplitude = IMAGE_WIDTH as f32 / self.length as f32;


        // Center line
        for x in 0 .. IMAGE_WIDTH {
            paint_pixel(&mut image, x as u32, image_height_half as u32, RgbRaw([255,255,255]));
        }
        

        for t in (0 .. (self.sample_rate * self.length)).map(|x| x as f32 / self.sample_rate as f32) {
            let x = t * image_width_amplitude;
            let mut sample = 0.;

            for (i, wave) in self.channels.iter().enumerate() {
                let freq = sinwave(t + wave.faze, wave.root.0, wave.ratio);
                let mut y = image_height_half;
                if i % 2 == 0 {
                    sample += freq;
                    y += image_amplitude * freq;
                } else {
                    sample -= freq;
                    y -= image_amplitude * freq;
                }
                paint_pixel(&mut image, x as u32, y as u32, RgbRaw(COLOURS[i]));
            }

            let amplitude = i16::MAX as f32 / self.channels.len() as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();

            //render image
            let y = image_height_half + (sample * image_amplitude);
            paint_pixel(&mut image, x as u32, y as u32, RgbRaw([0, 255, 255]));
        }
        image.save("sine.png").unwrap();
    }
}

fn sinwave(time: f32, root:f32, ratio: f32) -> f32 {
    (time * root * 2. *  PI * ratio).sin()
}

#[test]
fn wave_test() {
    let mut sampler = Sampler::new(1);
    let note = Frequency::new(10.);
    sampler.add_wave(Wave::freq(note.haronic(1)));
    sampler.add_wave(Wave::freq(note.haronic(2)));
    sampler.add_wave(Wave::freq(note.haronic(4)));
    sampler.render();
}
