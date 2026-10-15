use crate::{
    preset::{Env, Lfo, Waveform},
    tables::{self, Random},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Stage {
    #[default]
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Envelope {
    stage: Stage,
    value: f32,
    attack_step: f32,
    decay_coeff: f32,
    release_coeff: f32,
    sustain: f32,
}
impl Envelope {
    pub fn configure(&mut self, sample_rate: f32, p: &Env) {
        self.attack_step = 1.0 / (p.attack_seconds() * sample_rate);
        self.decay_coeff = (-6.0 / (p.decay_seconds() * sample_rate)).exp();
        self.release_coeff = (-9.0 / (p.release_seconds() * sample_rate)).exp();
        self.sustain = p.sustain;
    }
    pub fn on(&mut self) {
        self.value = 0.0;
        self.stage = Stage::Attack;
    }
    pub fn off(&mut self) {
        if self.stage != Stage::Idle {
            self.stage = Stage::Release;
        }
    }
    pub fn kill(&mut self) {
        self.stage = Stage::Idle;
        self.value = 0.0;
    }
    pub fn active(&self) -> bool {
        self.stage != Stage::Idle
    }
    pub fn next_sample(&mut self) -> f32 {
        match self.stage {
            Stage::Idle => self.value = 0.0,
            Stage::Attack => {
                self.value += self.attack_step;
                if self.value >= 1.0 {
                    self.value = 1.0;
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                self.value = self.sustain + (self.value - self.sustain) * self.decay_coeff;
                if (self.value - self.sustain).abs() < 0.001 {
                    self.stage = Stage::Sustain;
                }
            }
            Stage::Sustain => self.value = self.sustain,
            Stage::Release => {
                self.value *= self.release_coeff;
                if self.value < 0.0001 {
                    self.kill();
                }
            }
        }
        self.value
    }
}

#[derive(Clone, Debug)]
pub struct Oscillator {
    pub phase: f64,
    from: f32,
    to: f32,
    random: Random,
}
impl Oscillator {
    pub fn new(seed: u64) -> Self {
        let mut random = Random(seed);
        let from = random.bipolar();
        let to = random.bipolar();
        Self {
            phase: 0.0,
            from,
            to,
            random,
        }
    }
    pub fn next_sample(&mut self, lfo: &Lfo, sample_rate: f32, bpm: f32, envelope: f32) -> f32 {
        let p = self.phase as f32;
        let result = match lfo.waveform {
            Waveform::Sine => tables::sine(self.phase),
            Waveform::Triangle => 1.0 - 4.0 * (p - 0.5).abs(),
            Waveform::Saw => p * 2.0 - 1.0,
            Waveform::Square => {
                if p < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Random => self.to,
            Waveform::SmoothRandom => self.from + (self.to - self.from) * (p * p * (3.0 - 2.0 * p)),
            Waveform::Envelope => envelope * 2.0 - 1.0,
        };
        let hz = if lfo.sync {
            bpm / 60.0 / lfo.division
        } else {
            lfo.rate_hz
        };
        self.phase += hz as f64 / sample_rate as f64;
        if self.phase >= 1.0 {
            self.phase = self.phase.rem_euclid(1.0);
            self.from = self.to;
            self.to = self.random.bipolar();
        }
        result
    }
}
