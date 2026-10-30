//! Stereo-linked protection of the final mix with a smooth lookahead attack.
use crate::Stereo;

pub const CEILING: f32 = 0.891_250_9; // -1 dBFS

/// A fixed two-millisecond delay, also while bypassed, so toggling protection
/// does not move the audio relative to the host or other instruments.
pub fn latency_frames(rate: u32) -> u32 {
    (rate / 500).max(1)
}

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
    delay: Box<[Stereo]>,
    gains: Box<[f64]>,
    gain_sum: f64,
    position: usize,
    held: [f32; 2],
    held_frames: usize,
    enabled: bool,
}

impl Limiter {
    pub fn new(rate: u32) -> Self {
        let length = latency_frames(rate) as usize;
        Self {
            gain: 1.0,
            hold: 0,
            // Hold across a bass cycle instead of recovering between its
            // peaks and repeatedly flattening the waveform.
            hold_frames: (rate / 20).max(1),
            release: 1.0 - (-1.0 / (rate as f64 * 0.15)).exp(),
            input_peak: 0.0,
            delay: vec![[0.0; 2]; length].into_boxed_slice(),
            gains: vec![1.0; length].into_boxed_slice(),
            gain_sum: length as f64,
            position: 0,
            held: [0.0; 2],
            held_frames: 0,
            enabled: true,
        }
    }

    pub fn process(&mut self, input: Stereo, enabled: bool) -> Stereo {
        self.enabled = enabled;
        let input = input.map(|x| if x.is_finite() { x } else { 0.0 });
        let peak = input[0].abs().max(input[1].abs());
        self.input_peak = self.input_peak.max(peak);
        // Every peak remains in the detector for at least the entire delay.
        // Thus every gain in the moving average is safe for the delayed
        // sample, including when an earlier, larger peak is releasing.
        self.held[0] = self.held[0].max(peak);
        let held_peak = self.held[0].max(self.held[1]);
        self.held_frames += 1;
        if self.held_frames > self.delay.len() {
            self.held_frames = 0;
            self.held = [0.0, self.held[0]];
        }
        let target = if held_peak > CEILING {
            CEILING as f64 / held_peak as f64
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
        // An immediate detector drop becomes a two-millisecond ramp before
        // its transient sounds. It cannot step the other channel or the
        // sustained tracks underneath a drum hit.
        let at = self.position;
        self.gain_sum += self.gain - self.gains[at];
        self.gains[at] = self.gain;
        let gain = (self.gain_sum / self.gains.len() as f64).clamp(0.0, 1.0);
        let delayed = std::mem::replace(&mut self.delay[at], input);
        self.position = (at + 1) % self.delay.len();
        if !enabled {
            return delayed;
        }
        // The clamp catches only multiplication roundoff at the ceiling.
        delayed.map(|x| ((x as f64 * gain) as f32).clamp(-CEILING, CEILING))
    }

    pub fn meter(&self) -> Meter {
        Meter {
            input_peak: self.input_peak,
            reduction_db: if self.enabled {
                -super::fast::db((self.gain_sum / self.gains.len() as f64).clamp(1e-20, 1.0) as f32)
            } else {
                0.0
            },
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
            for _ in 0..latency_frames(rate) {
                assert_eq!(limiter.process([0.2, -0.4], true), [0.0; 2]);
            }
            assert_eq!(limiter.process([0.2, -0.4], true), [0.2, -0.4]);
            for i in 0..rate {
                let x = (i as f32 * 0.317).sin() * (1.0 + (i % 17) as f32);
                let out = limiter.process([x * -0.5, x], true);
                assert!(out.iter().all(|x| x.is_finite() && x.abs() <= CEILING));
                assert!((out[0] + out[1] * 0.5).abs() < 1e-7);
            }
            assert!(limiter.meter().reduction_db > 20.0);
            for _ in 0..latency_frames(rate) {
                limiter.process([4.0, -2.0], false);
            }
            assert_eq!(limiter.process([4.0, -2.0], false), [4.0, -2.0]);
            assert_eq!(limiter.meter().reduction_db, 0.0);
            for _ in 0..latency_frames(rate) {
                limiter.process([f32::NAN, f32::INFINITY], true);
            }
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
                    let delayed = (std::f64::consts::TAU * 40.0 * (n - latency_frames(rate)) as f64
                        / rate as f64)
                        .sin() as f32;
                    error = error.max((out[0] - delayed * CEILING).abs());
                }
            }
            assert!(error < 0.0001, "{rate}: bass waveform error {error}");
            for _ in 0..rate * 2 {
                limiter.process([0.0; 2], true);
            }
            for _ in 0..latency_frames(rate) {
                limiter.process([0.25; 2], true);
            }
            assert!(limiter.process([0.25; 2], true)[0] > 0.2499);
        }
    }

    #[test]
    fn transients_do_not_click_the_other_channel_or_escape_the_ceiling() {
        for rate in [8000, 44100, 48000, 96000, 384000] {
            let mut limiter = Limiter::new(rate);
            let mut previous = 0.25;
            let mut largest_step = 0.0_f32;
            for n in 0..rate {
                // A quiet sustained left channel and isolated, increasingly
                // loud right-channel drum-like transients, including release.
                let pulse = if n > rate / 4 && n % (rate / 8) == 0 {
                    2.0 + 2.0 * n as f32 / rate as f32
                } else {
                    0.0
                };
                let out = limiter.process([0.25, pulse], true);
                assert!(out.iter().all(|x| x.abs() <= CEILING));
                if n > rate / 4 {
                    largest_step = largest_step.max((out[0] - previous).abs());
                }
                previous = out[0];
            }
            assert!(
                largest_step * (rate as f32) < 125.0,
                "{rate}: transient stepped the sustained channel by {largest_step}"
            );
            assert!(limiter.meter().reduction_db > 1.0);
        }
    }

    #[test]
    fn quiet_audio_and_bypass_keep_the_same_exact_delay() {
        for rate in [8000, 44100, 48000, 96000] {
            let mut limiter = Limiter::new(rate);
            let signal = |n: u32| [n as f32 % 17.0 * 0.01, n as f32 % 23.0 * -0.01];
            let latency = latency_frames(rate);
            for n in 0..rate {
                let out = limiter.process(signal(n), n % 127 < 64);
                let expected = if n < latency {
                    [0.0; 2]
                } else {
                    signal(n - latency)
                };
                assert_eq!(out, expected);
            }
        }
    }
}
