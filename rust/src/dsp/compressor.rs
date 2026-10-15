//! Feed-forward peak compressor with a stereo-linked detector and soft knee.
use crate::{Stereo, color::ColorSettings};

pub struct Compressor {
    sample_rate: f32,
    threshold_db: f32,
    slope: f32,
    knee_db: f32,
    attack_alpha: f32,
    release_alpha: f32,
    makeup_gain: f32,
    reduction_db: f32,
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
        };
        compressor.configure(&ColorSettings::default());
        compressor
    }

    pub fn clear(&mut self) {
        self.reduction_db = 0.0;
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
        let peak = frame[0].abs().max(frame[1].abs()).max(1e-12);
        let over = 20.0 * peak.log10() - self.threshold_db;
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
        self.reduction_db += alpha * (target - self.reduction_db);
        let gain = self.makeup_gain * 10.0_f32.powf(-self.reduction_db / 20.0);
        [frame[0] * gain, frame[1] * gain]
    }
}
