use crate::{Stereo, tables::Random};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct SampleBuffer {
    pub name: String,
    pub library_id: Option<String>,
    pub sample_rate: u32,
    levels: Vec<Vec<Stereo>>,
    pub waveform: Vec<[f32; 2]>,
}

impl SampleBuffer {
    pub fn new(name: String, sample_rate: u32, mut frames: Vec<Stereo>) -> Result<Self, String> {
        if !(8_000..=384_000).contains(&sample_rate) {
            return Err("Unsupported sample rate (8–384 kHz)".into());
        }
        if frames.is_empty() {
            return Err("The audio file is empty".into());
        }
        for frame in &mut frames {
            for value in frame {
                *value = if value.is_finite() {
                    value.clamp(-16.0, 16.0)
                } else {
                    0.0
                };
            }
        }
        let bins = frames.len().min(640);
        let waveform = (0..bins)
            .map(|i| {
                let start = i * frames.len() / bins;
                let end = ((i + 1) * frames.len() / bins).max(start + 1);
                frames[start..end].iter().fold([1.0_f32, -1.0_f32], |m, x| {
                    let value = (x[0] + x[1]) * 0.5;
                    [m[0].min(value), m[1].max(value)]
                })
            })
            .collect();
        let mut levels = vec![frames];
        // Windowed-sinc lowpass before each 2:1 decimation. Constructed on load,
        // never on the audio callback. This replaces the unresolved NEON FIR.
        let mut kernel = [0.0_f64; 33];
        for (i, k) in kernel.iter_mut().enumerate() {
            let x = i as f64 - 16.0;
            let sinc = if x == 0.0 {
                0.45
            } else {
                (PI * 0.45 * x).sin() / (PI * x)
            };
            let window = 0.42 - 0.5 * (2.0 * PI * i as f64 / 32.0).cos()
                + 0.08 * (4.0 * PI * i as f64 / 32.0).cos();
            *k = sinc * window;
        }
        let sum: f64 = kernel.iter().sum();
        kernel.iter_mut().for_each(|x| *x /= sum);
        while levels.len() < 7 && levels.last().unwrap().len() >= 64 {
            let source = levels.last().unwrap();
            let len = source.len() as i64;
            let mut reduced = Vec::with_capacity(source.len().div_ceil(2));
            for center in (0..source.len()).step_by(2) {
                let mut frame = [0.0_f64; 2];
                for (tap, &k) in kernel.iter().enumerate() {
                    let i = (center as i64 + tap as i64 - 16).rem_euclid(len) as usize;
                    for (c, value) in frame.iter_mut().enumerate() {
                        *value += source[i][c] as f64 * k;
                    }
                }
                reduced.push([frame[0] as f32, frame[1] as f32]);
            }
            levels.push(reduced);
        }
        Ok(Self {
            name,
            library_id: None,
            sample_rate,
            levels,
            waveform,
        })
    }
    pub fn len(&self) -> usize {
        self.levels[0].len()
    }
    pub fn is_empty(&self) -> bool {
        self.levels[0].is_empty()
    }
    pub fn duration(&self) -> f64 {
        self.len() as f64 / self.sample_rate as f64
    }

    pub fn frames(&self) -> &[Stereo] {
        &self.levels[0]
    }

    pub fn read(&self, position: f64, rate: f64, antialiasing: bool) -> Stereo {
        if !position.is_finite() {
            return [0.0; 2];
        }
        let speed = rate.abs();
        let level = if antialiasing && speed >= 2.0 {
            (speed.log2().floor() as usize).min(self.levels.len() - 1)
        } else {
            0
        };
        let data = &self.levels[level];
        // Keep indices in f64: f32 loses sub-sample precision on long recordings.
        let p = position.rem_euclid(self.len() as f64) / (1_usize << level) as f64;
        let base = p.floor() as i64;
        let fraction = (p - p.floor()) as f32;
        let get = |offset: i64| data[(base + offset).rem_euclid(data.len() as i64) as usize];
        let a = get(-1);
        let b = get(0);
        let c = get(1);
        let d = get(2);
        std::array::from_fn(|i| {
            let c1 = 0.5 * (c[i] - a[i]);
            let c2 = a[i] - 2.5 * b[i] + 2.0 * c[i] - 0.5 * d[i];
            let c3 = 0.5 * (d[i] - a[i]) + 1.5 * (b[i] - c[i]);
            ((c3 * fraction + c2) * fraction + c1) * fraction + b[i]
        })
    }

    pub fn demo(sample_rate: u32) -> Self {
        let mut random = Random::default();
        let notes = [
            130.8128_f64,
            195.9977,
            164.8138,
            246.9417,
            261.6256,
            164.8138,
            195.9977,
            329.6276,
        ];
        let frames = (0..sample_rate as usize * 8)
            .map(|i| {
                let t = i as f64 / sample_rate as f64;
                let step = (t * 2.0) as usize;
                let age = t - step as f64 * 0.5;
                let frequency = notes[step % notes.len()];
                let pluck = (1.0 - (-age * 500.0).exp()) * (-age * 5.0).exp();
                let phase = t * frequency * 2.0 * PI;
                let tone = (phase.sin() + 0.3 * (phase * 2.001).sin() + 0.12 * (phase * 3.0).sin())
                    * pluck
                    * 0.28;
                let pad =
                    (t * 65.4064 * 2.0 * PI).sin() * 0.06 + (t * 97.9989 * 2.0 * PI).sin() * 0.04;
                let tick = random.bipolar() as f64 * (-age * 100.0).exp() * 0.06;
                [
                    (tone + pad + tick) as f32,
                    (tone * (0.88 + 0.12 * (t * 0.8).sin()) + pad - tick) as f32,
                ]
            })
            .collect();
        Self::new("Glass sequence".into(), sample_rate, frames).expect("valid built-in sample")
    }
}
