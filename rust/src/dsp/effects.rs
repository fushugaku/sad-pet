//! Portable desktop effects. These replace, rather than translate, unresolved
//! NEON kernels; the original reverb impulse responses are not reproduced.
use crate::{
    Stereo,
    preset::{Params, ReverbType},
};

pub struct Delay {
    frames: Vec<Stereo>,
    cursor: usize,
}
impl Delay {
    pub fn new(frames: usize) -> Self {
        Self {
            frames: vec![[0.0; 2]; frames.max(4)],
            cursor: 0,
        }
    }
    pub fn push(&mut self, frame: Stereo) {
        self.frames[self.cursor] = frame;
        self.cursor = (self.cursor + 1) % self.frames.len();
    }
    pub fn read(&self, samples: f32) -> Stereo {
        let position = (self.cursor as f64
            - samples.clamp(1.0, (self.frames.len() - 2) as f32) as f64)
            .rem_euclid(self.frames.len() as f64);
        let i = position as usize;
        let f = (position - i as f64) as f32;
        let a = self.frames[i];
        let b = self.frames[(i + 1) % self.frames.len()];
        [a[0] + (b[0] - a[0]) * f, a[1] + (b[1] - a[1]) * f]
    }
    pub fn clear(&mut self) {
        self.frames.fill([0.0; 2]);
        self.cursor = 0;
    }
}

pub struct Effects {
    sample_rate: f32,
    predelay: Delay,
    tank: [Delay; 8],
    tank_damping: [f32; 8],
    feedback: [f32; 8],
    lengths: [f32; 8],
    diffuser: [Delay; 2],
    reverse: Delay,
    reverse_phase: usize,
    shifter: Delay,
    shifter_phase: f64,
    tone: f32,
    predelay_samples: f32,
}

impl Effects {
    pub fn new(sample_rate: u32) -> Self {
        let sr = sample_rate as f32;
        Self {
            sample_rate: sr,
            predelay: Delay::new((sr * 0.25) as usize),
            tank: std::array::from_fn(|_| Delay::new((sr * 0.18) as usize)),
            tank_damping: [0.0; 8],
            feedback: [0.8; 8],
            lengths: [1000.0; 8],
            diffuser: [
                Delay::new((sr * 0.013) as usize),
                Delay::new((sr * 0.019) as usize),
            ],
            reverse: Delay::new((sr * 0.6) as usize + 4),
            reverse_phase: 0,
            shifter: Delay::new((sr * 0.1) as usize + 8),
            shifter_phase: 0.0,
            tone: 0.5,
            predelay_samples: 1.0,
        }
    }

    pub fn configure(&mut self, p: &Params) {
        let scale = match p.reverb_type {
            ReverbType::PlateS => 0.65,
            ReverbType::PlateL => 1.45,
            ReverbType::Room => 0.4,
            ReverbType::Reverse => 1.0,
        };
        let ms = [29.7, 37.1, 41.1, 43.7, 53.3, 59.3, 61.7, 71.9];
        let decay = 0.25 + p.reverb_decay.powi(2) * 14.0;
        for (i, ms) in ms.iter().enumerate() {
            self.lengths[i] = self.sample_rate * ms * 0.001 * scale;
            self.feedback[i] = 10.0_f32.powf(-3.0 * self.lengths[i] / (self.sample_rate * decay));
        }
        self.tone = 0.03 + 0.9 * p.reverb_tone.powi(2);
        self.predelay_samples = (p.predelay.powi(2) * 0.2 * self.sample_rate).max(1.0);
    }

    pub fn reverb(&mut self, frame: Stereo, p: &Params, wet: f32) -> Stereo {
        self.predelay.push(frame);
        let mut input = self.predelay.read(self.predelay_samples);
        if p.reverb_type == ReverbType::Reverse {
            self.reverse.push(input);
            let window = (self.sample_rate * 0.22) as usize;
            let phase = self.reverse_phase as f32 / window as f32;
            let other = (phase + 0.5).fract();
            let a = self.reverse.read((self.reverse_phase * 2 + 1) as f32);
            let b = self
                .reverse
                .read((other * window as f32 * 2.0 + 1.0).min(self.sample_rate * 0.55));
            let wa = (std::f32::consts::PI * phase).sin().powi(2);
            input = std::array::from_fn(|i| a[i] * wa + b[i] * (1.0 - wa));
            self.reverse_phase = (self.reverse_phase + 1) % window;
        }
        for i in 0..2 {
            let old =
                self.diffuser[i].read(self.sample_rate * (if i == 0 { 0.0097 } else { 0.0151 }));
            let gain = p.diffuse * 0.7;
            let next = std::array::from_fn(|c| input[c] + old[c] * gain);
            self.diffuser[i].push(next);
            input = std::array::from_fn(|c| old[c] - next[c] * gain);
        }
        self.shifter.push(input);
        let shift_window = self.sample_rate * 0.06;
        let phase = self.shifter_phase as f32;
        let other = (phase + 0.5).fract();
        let a = self.shifter.read(1.0 + (1.0 - phase) * shift_window);
        let b = self.shifter.read(1.0 + (1.0 - other) * shift_window);
        let wa = (std::f32::consts::PI * phase).sin().powi(2);
        self.shifter_phase = (self.shifter_phase + 1.0 / shift_window as f64).rem_euclid(1.0);
        input = std::array::from_fn(|c| {
            input[c] * (1.0 - p.shimmer * 0.5) + (a[c] * wa + b[c] * (1.0 - wa)) * p.shimmer * 0.5
        });
        let mut taps = [0.0_f32; 8];
        for (i, tap) in taps.iter_mut().enumerate() {
            let x = self.tank[i].read(self.lengths[i])[0];
            self.tank_damping[i] += self.tone * (x - self.tank_damping[i]);
            *tap = self.tank_damping[i];
        }
        let reflection = taps.iter().sum::<f32>() * 0.25;
        for (i, &tap) in taps.iter().enumerate() {
            let value = input[i % 2] * 0.22 + (tap - reflection) * self.feedback[i];
            let clean = if value.abs() < 1e-20 {
                0.0
            } else {
                value.clamp(-8.0, 8.0)
            };
            self.tank[i].push([clean, clean]);
        }
        let tail = [
            (taps[0] + taps[2] - taps[4] + taps[6]) * 0.5,
            (taps[1] + taps[3] - taps[5] + taps[7]) * 0.5,
        ];
        std::array::from_fn(|i| frame[i] * (1.0 - wet * 0.4) + tail[i] * wet * 2.0)
    }

    pub fn clear(&mut self) {
        self.tank_damping = [0.0; 8];
        self.reverse_phase = 0;
        self.shifter_phase = 0.0;
        self.predelay.clear();
        self.reverse.clear();
        self.shifter.clear();
        self.tank.iter_mut().for_each(Delay::clear);
        self.diffuser.iter_mut().for_each(Delay::clear);
    }
}
