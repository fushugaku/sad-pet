//! Feed-forward peak compressor with a stereo-linked, peak-holding detector
//! and soft knee.
use super::fast;
use crate::{Stereo, color::ColorSettings};

/// The detector reads the largest peak of the last 2.5-5 ms.
const HOLD_SECONDS: f32 = 0.005;

pub struct Compressor {
    sample_rate: f32,
    threshold_db: f32,
    slope: f32,
    knee_db: f32,
    attack_alpha: f32,
    release_alpha: f32,
    makeup_gain: f32,
    reduction_db: f32,
    /// Peaks of the current and the previous half of the hold window.
    held: [f32; 2],
    hold_frames: u32,
    hold_count: u32,
}

impl Compressor {
    pub fn new(sample_rate: u32) -> Self {
        let mut compressor = Self {
            sample_rate: sample_rate as f32,
            threshold_db: 0.0,
            slope: 0.0,
            knee_db: 0.0,
            attack_alpha: 1.0,
            release_alpha: 1.0,
            makeup_gain: 1.0,
            reduction_db: 0.0,
            held: [0.0; 2],
            hold_frames: ((sample_rate as f32 * HOLD_SECONDS * 0.5) as u32).max(1),
            hold_count: 0,
        };
        compressor.configure(&ColorSettings::default());
        compressor
    }

    pub fn clear(&mut self) {
        self.reduction_db = 0.0;
        self.held = [0.0; 2];
        self.hold_count = 0;
    }

    /// Applied attenuation before makeup gain, in dB. Reading never changes DSP.
    pub fn gain_reduction_db(&self) -> f32 {
        self.reduction_db
    }

    pub fn configure(&mut self, settings: &ColorSettings) {
        self.threshold_db = -60.0 + 60.0 * settings.compressor_threshold;
        self.slope = 1.0 - 1.0 / 20.0_f32.powf(settings.compressor_ratio);
        self.knee_db = 24.0 * settings.compressor_knee;
        let attack = 0.0001 * 1000.0_f32.powf(settings.compressor_attack);
        let release = 0.01 * 200.0_f32.powf(settings.compressor_release);
        self.attack_alpha = 1.0 - (-1.0 / (attack * self.sample_rate)).exp();
        self.release_alpha = 1.0 - (-1.0 / (release * self.sample_rate)).exp();
        self.makeup_gain = 10.0_f32.powf(24.0 * settings.compressor_makeup / 20.0);
    }

    pub fn process(&mut self, frame: Stereo) -> Stereo {
        // Both channels receive the same gain, preserving their balance even
        // when a transient arrives on just one side. No lookahead or latency.
        // A rising peak takes effect at once; a held one keeps the level
        // between the half-waves. An instantaneous peak falls to zero at
        // every zero crossing, so fast release recovered between peaks and
        // fast attack grabbed the next: noise-like input, such as a reverb
        // tail, crackled with random gain dips.
        self.held[0] = self.held[0].max(frame[0].abs().max(frame[1].abs()));
        let peak = self.held[0].max(self.held[1]).max(1e-12);
        self.hold_count += 1;
        if self.hold_count == self.hold_frames {
            self.hold_count = 0;
            self.held = [0.0, self.held[0]];
        }
        let over = fast::db(peak) - self.threshold_db;
        let half_knee = self.knee_db * 0.5;
        let target = if over <= -half_knee {
            0.0
        } else if self.knee_db > 0.0 && over < half_knee {
            self.slope * (over + half_knee).powi(2) / (2.0 * self.knee_db)
        } else {
            self.slope * over
        };
        // Smooth gain reduction in dB. Settings can change each control block
        // without resetting the envelope, including through modulation routes.
        let alpha = if target > self.reduction_db {
            self.attack_alpha
        } else {
            self.release_alpha
        };
        self.reduction_db = fast::flush(self.reduction_db + alpha * (target - self.reduction_db));
        let gain = self.makeup_gain * fast::gain_db(-self.reduction_db);
        [frame[0] * gain, frame[1] * gain]
    }
}
