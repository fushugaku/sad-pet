//! L/C/R and mono-input Mod Delay, adapted from radias-emulator's native
//! effect_audio.rs. Control tables retain RADIAS ranges; buffers, interpolation
//! and arithmetic use this desktop engine, not original FXD03 instructions.
use super::{effects::Delay, fast};
use crate::{
    Stereo,
    color::{ColorSettings, ColorSlot},
    preset::SaturatorType,
    radias_delay, tables,
};

pub struct RadiasDelay {
    buffer: Delay,
    sample_rate: f32,
    kind: SaturatorType,
    settings: ColorSettings,
    times: [f64; 3],
    target_times: [f64; 3],
    time_alpha: f64,
    initialized: bool,
    smooth: bool,
    ratio_glide: bool,
    phase: f64,
    phase_step: f64,
    feedback: f32,
    high_alpha: f32,
    low_alpha: f32,
    high_state: Stereo,
    low_state: Stereo,
}
impl RadiasDelay {
    pub fn new(sample_rate: u32) -> Self {
        Self::create(sample_rate, true)
    }
    pub(crate) fn unallocated(sample_rate: u32) -> Self {
        Self::create(sample_rate, false)
    }
    fn create(sample_rate: u32, allocate: bool) -> Self {
        Self {
            buffer: if allocate {
                Delay::new((sample_rate as f32 * radias_delay::MAX_SECONDS).ceil() as usize + 4)
            } else {
                Delay::empty()
            },
            sample_rate: sample_rate as f32,
            kind: SaturatorType::LcrDelay,
            settings: ColorSettings::default(),
            times: [1.0; 3],
            target_times: [1.0; 3],
            time_alpha: 1.0 - (-1.0 / (sample_rate as f64 * 0.01)).exp(),
            initialized: false,
            smooth: false,
            ratio_glide: false,
            phase: 0.0,
            phase_step: 0.0,
            feedback: 0.0,
            high_alpha: 1.0,
            low_alpha: 1.0,
            high_state: [0.0; 2],
            low_state: [0.0; 2],
        }
    }
    pub(crate) fn has_storage(&self) -> bool {
        self.buffer.has_storage()
    }
    pub(crate) fn take_storage(&mut self) -> Vec<Stereo> {
        self.buffer.take_storage()
    }
    pub(crate) fn install_storage(&mut self, frames: Vec<Stereo>) {
        self.buffer.install_storage(frames);
    }
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.phase = 0.0;
        self.high_state = [0.0; 2];
        self.low_state = [0.0; 2];
        self.times = self.target_times;
        self.initialized = false;
        self.ratio_glide = false;
    }
    pub fn configure(&mut self, slot: ColorSlot, bpm: f32) {
        let lcr = slot.kind == SaturatorType::LcrDelay;
        if lcr && self.settings.lcr_ratio != slot.settings.lcr_ratio {
            self.ratio_glide = true;
        } else if !lcr {
            self.ratio_glide = false;
        }
        self.kind = slot.kind;
        self.settings = slot.settings;
        let s = slot.settings;
        self.smooth = if lcr {
            s.lcr_smooth
        } else {
            s.mod_delay_smooth
        };
        let (sync, ratio, free, notes) = if lcr {
            (
                s.lcr_sync,
                s.lcr_ratio,
                [s.lcr_left_delay, s.lcr_center_delay, s.lcr_right_delay],
                [s.lcr_left_note, s.lcr_center_note, s.lcr_right_note],
            )
        } else {
            (
                s.mod_delay_sync,
                s.mod_delay_ratio,
                [s.mod_delay_left_delay, s.mod_delay_right_delay, 0.0],
                [s.mod_delay_left_note, s.mod_delay_right_note, 0.0],
            )
        };
        let seconds = if self.smooth {
            radias_delay::smooth_seconds
        } else {
            radias_delay::seconds
        };
        self.target_times = std::array::from_fn(|i| {
            (seconds(sync, ratio, free[i], notes[i], bpm, !lcr) * self.sample_rate).max(1.0) as f64
        });
        if !self.initialized || (!self.smooth && !self.ratio_glide) {
            self.times = self.target_times;
        }
        self.initialized = true;
        self.feedback = if lcr {
            s.lcr_feedback
        } else {
            s.mod_delay_feedback
        }
        .min(0.995);
        self.phase_step = radias_delay::lfo_hz(s.mod_delay_rate) as f64 / self.sample_rate as f64;
        let high = if lcr { s.lcr_high_damp } else { 0.0 };
        let low = if lcr { s.lcr_low_damp } else { 0.0 };
        let alpha = |hz: f32| {
            1.0 - (-std::f32::consts::TAU * hz.min(self.sample_rate * 0.45) / self.sample_rate)
                .exp()
        };
        self.high_alpha = alpha(18_000.0 * 0.018_f32.powf(high));
        self.low_alpha = alpha(20.0 + 780.0 * low);
    }
    fn damp(&mut self, channel: usize, sample: f32, low_amount: f32) -> f32 {
        self.high_state[channel] = fast::flush(
            self.high_state[channel] + self.high_alpha * (sample - self.high_state[channel]),
        );
        let high = self.high_state[channel];
        self.low_state[channel] = fast::flush(
            self.low_state[channel] + self.low_alpha * (high - self.low_state[channel]),
        );
        high - self.low_state[channel] * low_amount
    }
    pub fn process(&mut self, input: Stereo) -> Stereo {
        if self.smooth || self.ratio_glide {
            // Audio-rate glide removes control-block steps without resetting the tail.
            for (time, target) in self.times.iter_mut().zip(self.target_times) {
                *time += self.time_alpha * (target - *time);
            }
            if self.ratio_glide
                && self
                    .times
                    .iter()
                    .zip(self.target_times)
                    .all(|(t, target)| (*t - target).abs() < 0.0001)
            {
                self.times = self.target_times;
                self.ratio_glide = false;
            }
        }
        let mono = (input[0] + input[1]) * 0.5;
        let s = self.settings;
        if self.kind == SaturatorType::LcrDelay {
            let taps: [f32; 3] = std::array::from_fn(|i| self.buffer.read(self.times[i] as f32)[0]);
            let tail = self.damp(0, taps[1], s.lcr_low_damp);
            let write = fast::flush((mono * s.lcr_trim + tail * self.feedback).clamp(-8.0, 8.0));
            self.buffer.push([write; 2]);
            let center = taps[1] * s.lcr_center_level * 0.707;
            let left = taps[0] * s.lcr_left_level + center;
            let right = taps[2] * s.lcr_right_level + center;
            let mid = (left + right) * 0.5;
            let side = (left - right) * 0.5 * s.lcr_spread;
            [mid + side, mid - side]
        } else {
            let depth = s.mod_delay_depth * 0.012 * self.sample_rate;
            let spread = s.mod_delay_spread as f64 - 0.5;
            let delayed: Stereo = std::array::from_fn(|c| {
                let modulation = tables::sine(self.phase + c as f64 * spread);
                self.buffer.read(self.times[c] as f32 + modulation * depth)[c]
            });
            let write = std::array::from_fn(|c| {
                fast::flush((mono + self.damp(c, delayed[c], 0.0) * self.feedback).clamp(-8.0, 8.0))
            });
            self.buffer.push(write);
            self.phase = super::sample::wrap_position(self.phase + self.phase_step, 1.0);
            delayed
        }
    }
}
