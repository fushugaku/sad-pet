//! Stereo-linked protection of the final mix, with no added latency.
use crate::Stereo;

pub const CEILING: f32 = 0.891_250_9; // -1 dBFS

#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Meter {
    pub input_peak: f32,
    pub reduction_db: f32,
}

pub(super) struct Limiter {
    gain: f64,
    hold: u32,
    hold_frames: u32,
    release: f64,
    input_peak: f32,
}

impl Limiter {
    pub fn new(rate: u32) -> Self {
        Self {
            gain: 1.0,
            hold: 0,
            // Hold across a bass cycle instead of recovering between its
            // peaks and repeatedly flattening the waveform.
            hold_frames: (rate / 20).max(1),
            release: 1.0 - (-1.0 / (rate as f64 * 0.15)).exp(),
            input_peak: 0.0,
        }
    }

    pub fn process(&mut self, input: Stereo, enabled: bool) -> Stereo {
        let input = input.map(|x| if x.is_finite() { x } else { 0.0 });
        let peak = input[0].abs().max(input[1].abs());
        self.input_peak = self.input_peak.max(peak);
        if !enabled {
            self.gain = 1.0;
            self.hold = 0;
            return input;
        }
        let target = if peak > CEILING {
            CEILING as f64 / peak as f64
        } else {
            1.0
        };
        if target <= self.gain {
            self.gain = target;
            self.hold = self.hold_frames;
        } else if self.hold > 0 {
            self.hold -= 1;
        } else {
            self.gain = target.min(self.gain + (1.0 - self.gain) * self.release);
        }
        // The clamp catches only multiplication roundoff at the ceiling.
        input.map(|x| ((x as f64 * self.gain) as f32).clamp(-CEILING, CEILING))
    }

    pub fn meter(&self) -> Meter {
        Meter {
            input_peak: self.input_peak,
            reduction_db: -super::fast::db(self.gain.max(1e-20) as f32),
        }
    }

    pub fn clear_peak(&mut self) {
        self.input_peak = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protects_both_channels_preserves_balance_and_bypasses_exactly() {
        for rate in [8000, 44100, 48000, 96000] {
            let mut limiter = Limiter::new(rate);
            assert_eq!(limiter.process([0.2, -0.4], true), [0.2, -0.4]);
            for i in 0..rate {
                let x = (i as f32 * 0.317).sin() * (1.0 + (i % 17) as f32);
                let out = limiter.process([x * -0.5, x], true);
                assert!(out.iter().all(|x| x.is_finite() && x.abs() <= CEILING));
                assert!((out[0] + out[1] * 0.5).abs() < 1e-7);
            }
            assert!(limiter.meter().reduction_db > 20.0);
            assert_eq!(limiter.process([4.0, -2.0], false), [4.0, -2.0]);
            assert_eq!(limiter.process([f32::NAN, f32::INFINITY], true), [0.0; 2]);
        }
    }

    #[test]
    fn sustained_bass_is_scaled_instead_of_clipped_and_release_recovers() {
        for rate in [8000, 44100, 48000, 96000] {
            let mut limiter = Limiter::new(rate);
            let mut error = 0.0_f32;
            for n in 0..rate {
                let sine = (std::f64::consts::TAU * 40.0 * n as f64 / rate as f64).sin() as f32;
                let out = limiter.process([sine * 8.0; 2], true);
                if n > rate / 2 {
                    error = error.max((out[0] - sine * CEILING).abs());
                }
            }
            assert!(error < 0.0001, "{rate}: bass waveform error {error}");
            for _ in 0..rate * 2 {
                limiter.process([0.0; 2], true);
            }
            assert!(limiter.process([0.25; 2], true)[0] > 0.2499);
        }
    }
}
