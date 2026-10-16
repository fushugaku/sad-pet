use super::compressor::Compressor;
use super::delay::RadiasDelay;
use super::effects::{Delay, Effects};
use crate::{
    Stereo,
    color::{ColorSlot, MAX_COLOR_SLOTS},
    preset::{Params, SaturatorType},
    tables::{self, Random},
};

pub struct ColorProcessor {
    id: u32,
    sample_rate: f32,
    slot: ColorSlot,
    delay: Delay,
    phase: f64,
    random: Random,
    held: Stereo,
    smoothed: Stereo,
    hold_clock: f64,
    tone_state: Stereo,
    dc_input: Stereo,
    dc_output: Stereo,
    drive: f32,
    tone_alpha: f32,
    dc_r: f32,
    period: f32,
    smooth_alpha: f32,
    levels: f32,
    phase_step: f64,
    base_delay: f32,
    excursion: f32,
    feedback: f32,
    stereo: f64,
    reverb: Effects,
    reverb_params: Params,
    radias_delay: RadiasDelay,
    compressor: Compressor,
}
impl ColorProcessor {
    pub fn new(sample_rate: u32) -> Self {
        let sr = sample_rate as f32;
        Self {
            id: 0,
            sample_rate: sr,
            slot: ColorSlot::default(),
            delay: Delay::new((sr * 0.08) as usize + 4),
            phase: 0.0,
            random: Random::default(),
            held: [0.0; 2],
            smoothed: [0.0; 2],
            hold_clock: 0.0,
            tone_state: [0.0; 2],
            dc_input: [0.0; 2],
            dc_output: [0.0; 2],
            drive: 1.0,
            tone_alpha: 1.0,
            dc_r: (-std::f32::consts::TAU * 5.0 / sr).exp(),
            period: 1.0,
            smooth_alpha: 1.0,
            levels: 65536.0,
            phase_step: 0.0,
            base_delay: 1.0,
            excursion: 0.0,
            feedback: 0.0,
            stereo: 0.0,
            reverb: Effects::new(sample_rate),
            reverb_params: Params::default(),
            radias_delay: RadiasDelay::new(sample_rate),
            compressor: Compressor::new(sample_rate),
        }
    }
    pub fn clear(&mut self) {
        if self.slot.kind == SaturatorType::Reverb {
            self.reverb.clear();
        }
        if matches!(
            self.slot.kind,
            SaturatorType::LcrDelay | SaturatorType::ModDelay
        ) {
            self.radias_delay.clear();
        }
        self.delay.clear();
        self.phase = 0.0;
        self.random = Random(0x853c49e6748fea9b_u64.wrapping_add(self.id as u64 * 0x12345679));
        self.held = [0.0; 2];
        self.smoothed = [0.0; 2];
        self.hold_clock = 0.0;
        self.tone_state = [0.0; 2];
        self.dc_input = [0.0; 2];
        self.dc_output = [0.0; 2];
        self.compressor.clear();
    }
    pub fn configure(&mut self, slot: ColorSlot) {
        self.configure_tempo(slot, 120.0);
    }
    pub fn configure_tempo(&mut self, slot: ColorSlot, bpm: f32) {
        let reset = self.id != slot.id
            || self.slot.kind != slot.kind
            || self.slot.enabled != slot.enabled
            || (slot.kind == SaturatorType::Reverb
                && self.slot.settings.reverb_type != slot.settings.reverb_type);
        self.id = slot.id;
        if reset {
            self.clear();
        }
        self.slot = slot;
        let s = slot.settings;
        match slot.kind {
            SaturatorType::Distortion => {
                self.drive = 1.0 + 18.0 * s.distortion_drive;
                let hz = 20.0 * 1000.0_f32.powf(s.distortion_tone);
                self.tone_alpha = 1.0 - (-std::f32::consts::TAU * hz / self.sample_rate).exp();
            }
            SaturatorType::Reducer => {
                self.period = 2.0_f32.powf(10.0 * (1.0 - s.reducer_rate));
                self.smooth_alpha = if s.reducer_smooth <= 0.0 {
                    1.0
                } else {
                    1.0 - (-1.0 / (self.period * s.reducer_smooth.powi(2) * 0.5)).exp()
                };
                // A rate increase takes effect immediately, even during a long hold.
                self.hold_clock = self.hold_clock.min(self.period as f64 * 1.95);
            }
            SaturatorType::Crusher => {
                self.levels = 2.0_f32.powf(1.0 + 15.0 * s.crusher_bits);
                self.drive = 1.0 + 18.0 * s.crusher_drive;
            }
            SaturatorType::Chorus | SaturatorType::Flanger => {
                let chorus = slot.kind == SaturatorType::Chorus;
                let (rate, depth, delay, feedback, stereo) = if chorus {
                    (
                        s.chorus_rate,
                        s.chorus_depth,
                        s.chorus_delay,
                        s.chorus_feedback,
                        s.chorus_stereo,
                    )
                } else {
                    (
                        s.flanger_rate,
                        s.flanger_depth,
                        s.flanger_delay,
                        s.flanger_feedback,
                        s.flanger_stereo,
                    )
                };
                self.phase_step = (0.01 * 2000.0_f32.powf(rate) / self.sample_rate) as f64;
                let seconds = if chorus {
                    0.005 + 0.035 * delay
                } else {
                    0.0001 + 0.0099 * delay
                };
                self.base_delay = seconds * self.sample_rate;
                self.excursion = self.base_delay * depth * if chorus { 0.75 } else { 0.99 };
                self.feedback = (feedback * 2.0 - 1.0) * 0.95;
                self.stereo = stereo as f64 * 0.5;
            }
            SaturatorType::Binaural => {
                self.base_delay = (s.binaural_delay * 0.0015 * self.sample_rate).max(1.0)
            }
            SaturatorType::Reverb => {
                self.reverb_params = slot.reverb_params();
                self.reverb.configure(&self.reverb_params);
            }
            SaturatorType::LcrDelay | SaturatorType::ModDelay => {
                self.radias_delay.configure(slot, bpm)
            }
            SaturatorType::Compressor => self.compressor.configure(&s),
        }
    }
    pub fn process(&mut self, frame: Stereo) -> Stereo {
        if !self.slot.enabled {
            return frame;
        }
        let s = self.slot.settings;
        let result = match self.slot.kind {
            SaturatorType::Distortion => {
                let bias = (s.distortion_bias * 2.0 - 1.0) * 0.5;
                let offset = bias.tanh();
                std::array::from_fn(|c| {
                    let shaped =
                        ((frame[c] * self.drive + bias).tanh() - offset) / self.drive.sqrt();
                    // Remove the DC produced by asymmetric clipping.
                    let highpass = shaped - self.dc_input[c] + self.dc_r * self.dc_output[c];
                    self.dc_input[c] = shaped;
                    self.dc_output[c] = highpass;
                    let input = if bias.abs() > 1e-6 { highpass } else { shaped };
                    if s.distortion_tone >= 0.999 {
                        self.tone_state[c] = input;
                    } else {
                        self.tone_state[c] += self.tone_alpha * (input - self.tone_state[c]);
                    }
                    self.tone_state[c]
                })
            }
            SaturatorType::Reducer => {
                if self.hold_clock <= 0.0 {
                    self.held = frame;
                    let jitter = if s.reducer_jitter > 0.0 {
                        self.random.bipolar() * s.reducer_jitter * 0.95
                    } else {
                        0.0
                    };
                    self.hold_clock += (self.period * (1.0 + jitter)).max(1.0) as f64;
                }
                self.hold_clock -= 1.0;
                std::array::from_fn(|c| {
                    self.smoothed[c] += self.smooth_alpha * (self.held[c] - self.smoothed[c]);
                    self.smoothed[c]
                })
            }
            SaturatorType::Crusher => std::array::from_fn(|c| {
                let dither = if s.crusher_dither > 0.0 {
                    (self.random.unit() - self.random.unit()) * s.crusher_dither
                } else {
                    0.0
                };
                let input = (frame[c] * self.drive).clamp(-1.0, 1.0);
                ((input * self.levels + dither).round() / self.levels) / self.drive.sqrt()
            }),
            SaturatorType::Chorus | SaturatorType::Flanger => {
                let l = self
                    .delay
                    .read(self.base_delay + self.excursion * tables::sine(self.phase))[0];
                let r = self.delay.read(
                    self.base_delay + self.excursion * tables::sine(self.phase + self.stereo),
                )[1];
                self.delay.push([
                    (frame[0] + l * self.feedback).clamp(-8.0, 8.0),
                    (frame[1] + r * self.feedback).clamp(-8.0, 8.0),
                ]);
                self.phase = (self.phase + self.phase_step).rem_euclid(1.0);
                [(frame[0] + l) * 0.7, (frame[1] + r) * 0.7]
            }
            SaturatorType::Binaural => {
                let delayed = self.delay.read(self.base_delay);
                self.delay.push(frame);
                let crossfeed = s.binaural_crossfeed * 0.75;
                let left = (frame[0] + delayed[1] * crossfeed).tanh();
                let right = (frame[1] + delayed[0] * crossfeed).tanh();
                let mid = (left + right) * 0.5;
                let side = (left - right) * s.binaural_width;
                [mid + side, mid - side]
            }
            // The outer Mix reproduces the previous reverb's dry/tail balance.
            SaturatorType::Reverb => self.reverb.reverb(frame, &self.reverb_params, 1.0),
            SaturatorType::LcrDelay | SaturatorType::ModDelay => self.radias_delay.process(frame),
            SaturatorType::Compressor => self.compressor.process(frame),
        };
        std::array::from_fn(|c| frame[c] * (1.0 - self.slot.mix) + self.slot.mix * result[c])
    }

    pub fn gain_reduction_db(&self) -> f32 {
        if self.slot.kind == SaturatorType::Compressor && self.slot.enabled && self.slot.mix > 0.0 {
            self.compressor.gain_reduction_db()
        } else {
            0.0
        }
    }
}

pub struct ColorRack {
    processors: [ColorProcessor; MAX_COLOR_SLOTS],
    len: usize,
}
impl ColorRack {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            processors: std::array::from_fn(|_| ColorProcessor::new(sample_rate)),
            len: 0,
        }
    }
    pub fn clear(&mut self) {
        self.processors.iter_mut().for_each(ColorProcessor::clear);
    }
    pub fn configure(&mut self, slots: &[ColorSlot]) {
        self.configure_tempo(slots, 120.0);
    }
    pub fn configure_tempo(&mut self, slots: &[ColorSlot], bpm: f32) {
        let slots = &slots[..slots.len().min(MAX_COLOR_SLOTS)];
        for (i, slot) in slots.iter().enumerate() {
            if self.processors[i].id != slot.id {
                let found = (i..MAX_COLOR_SLOTS)
                    .find(|&j| self.processors[j].id == slot.id)
                    .or_else(|| {
                        (i..MAX_COLOR_SLOTS)
                            .find(|&j| !slots.iter().any(|s| s.id == self.processors[j].id))
                    })
                    .unwrap_or(i);
                self.processors.swap(i, found);
            }
            self.processors[i].configure_tempo(*slot, bpm);
        }
        for processor in &mut self.processors[slots.len()..] {
            if processor.id != 0 {
                processor.clear();
                processor.id = 0;
            }
        }
        self.len = slots.len();
    }
    pub fn process(&mut self, mut frame: Stereo) -> Stereo {
        for processor in &mut self.processors[..self.len] {
            frame = processor.process(frame);
        }
        frame
    }

    pub fn gain_reduction(&self) -> std::collections::BTreeMap<u32, f32> {
        self.processors[..self.len]
            .iter()
            .filter(|p| p.slot.kind == SaturatorType::Compressor)
            .map(|p| (p.id, p.gain_reduction_db()))
            .collect()
    }
}
