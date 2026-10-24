//! The shared HPS/MNPIT audio callback (recovered `dual_window_pitch.rs`,
//! original 7800..7c80) and its Ba Pitch variant (`bass_pitch.rs`).
//!
//! A Q32 window phase advances by c8 per sample; two triangle windows
//! `|phase|` and `|phase + 0x80000001|` weight two taps. Each tap position
//! (Q17 in the original, samples here) moves by c9 = `1 - ratio` samples per
//! sample and jumps back to the reset position (state[4]) when its window
//! crosses zero, so a window period sweeps exactly the window length that the
//! `pitchUpdata` editors derive from the detected pitch period: the splices
//! fall a whole number of periods apart (pitch-synchronous shifting).
//!
//! Ba Pitch (`latch`) differs: c16 base 0, each tap picks up the pending
//! increment c10 only at its own reset, a zero increment resets every sample,
//! and editor updates go through the c12..c15 pending words that the callback
//! commits at the end of each 8-frame block.
use super::common::{Detection, samples, triangle};
use crate::dsp::DelayLine;

/// Reset positions (state[4] / c11): HPS/MNPIT 0x0a560000 (up) and
/// 0x01b90000 (down), Ba Pitch 0x181e0000 (up), all Q17.
pub const RESET_UP: f32 = 1_323.0;
pub const RESET_DOWN: f32 = 220.5;
pub const BASS_RESET_UP: f32 = 3_087.0;

/// The block length of the original; Ba Pitch commits pending words per block.
const BLOCK: u32 = 8;

/// Coefficients `pitchUpdata` writes: window increment c8, tap increment c9
/// (samples per sample) and the reset position, at the instance rate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Retune {
    pub window_increment: u32,
    pub tap_increment: f64,
    pub reset: f64,
}

impl Retune {
    /// `ratio` over a window of `window_44k` samples (44.1 kHz units):
    /// `c9 = (1 - ratio) 2^17`, `c8 = |(1 - ratio) 2^28 / window| << 4`.
    pub fn new(ratio: f32, window_44k: f32, reset_44k: f32, sample_rate: f32) -> Self {
        let slope = 1.0 - f64::from(ratio);
        let window = f64::from(samples(window_44k, sample_rate)).max(1.0);
        let turns = (slope.abs() / window).min(0.499);
        Self {
            window_increment: (turns * 4_294_967_296.0).round() as u32,
            tap_increment: slope,
            reset: f64::from(samples(reset_44k, sample_rate)),
        }
    }
}

/// The window length of `pitchUpdata`: eight periods halved until they fit
/// `cap`, doubled once more if that stays below `double_below`.
pub fn window_length(period: f32, cap: f32, double_below: f32) -> f32 {
    let mut length = period * 8.0;
    while length > cap {
        length *= 0.5;
    }
    if length * 2.0 < double_below {
        length *= 2.0;
    }
    length
}

/// The validity checks every `pitchUpdata` applies when its own parameters
/// did not change: level above -50 dB and a period of 16..2205 samples.
pub fn usable(detection: &Detection) -> bool {
    detection.level >= super::common::DETECTOR_GATE && (16.0..=2_205.0).contains(&detection.period)
}

pub struct DualWindow {
    line: DelayLine,
    base: f32,
    latch: bool,
    phase: u32,
    retune: Retune,
    pending: Option<Retune>,
    taps: [f64; 2],
    increments: [f64; 2],
    counter: u32,
}

impl DualWindow {
    /// `base_44k` is c10 (HPS/MNPIT 1000) or c16 (Ba Pitch 0); `longest_44k`
    /// bounds every tap distance.
    pub fn new(
        sample_rate: f32,
        base_44k: f32,
        longest_44k: f32,
        latch: bool,
        initial: Retune,
    ) -> Self {
        Self {
            line: DelayLine::new(samples(longest_44k, sample_rate).ceil() as usize + 4),
            base: samples(base_44k, sample_rate),
            latch,
            phase: 0,
            retune: initial,
            pending: None,
            taps: [initial.reset; 2],
            increments: [initial.tap_increment; 2],
            counter: 0,
        }
    }

    /// HPS/MNPIT start from the ROM state words (phase 0xffe96d00, taps at
    /// 0x0a560000 and 0x052b0000).
    pub fn set_phase(&mut self, phase: u32, taps: [f64; 2]) {
        self.phase = phase;
        self.taps = taps;
    }

    /// Write new coefficients: immediately (HPS/MNPIT), or as the pending words
    /// the Ba Pitch callback commits at its next block end.
    pub fn retune(&mut self, retune: Retune) {
        if self.latch {
            self.pending = Some(retune);
        } else {
            self.retune = retune;
        }
    }

    /// One ring sample in; the window-weighted tap sum out.
    #[inline]
    pub fn process(&mut self, ring: f32) -> f32 {
        self.line.write(ring);
        let increment = self.retune.window_increment;
        let old = self.phase;
        let new = old.wrapping_add(increment);
        let biased_old = old.wrapping_add(0x8000_0001);
        let biased = new.wrapping_add(0x8000_0001);
        self.phase = new;
        for (tap, (before, after)) in [(old, new), (biased_old, biased)].into_iter().enumerate() {
            let reset = if self.latch {
                ((before ^ after) & 0x8000_0000 != 0 && triangle(after) < 0.5) || increment == 0
            } else {
                (before as i32) < 0 && (after as i32) >= 0
            };
            if reset {
                self.taps[tap] = self.retune.reset;
                self.increments[tap] = self.retune.tap_increment;
            }
            if !self.latch {
                self.increments[tap] = self.retune.tap_increment;
            }
            self.taps[tap] += self.increments[tap];
        }
        let a = self.line.read_linear(self.base + self.taps[0] as f32);
        let b = self.line.read_linear(self.base + self.taps[1] as f32);
        let out = triangle(new) * a + triangle(biased) * b;
        if self.latch {
            self.counter += 1;
            if self.counter == BLOCK {
                self.counter = 0;
                if let Some(next) = self.pending.take() {
                    self.retune = next;
                }
            }
        }
        out
    }

    pub fn clear(&mut self) {
        self.line.clear();
        if let Some(next) = self.pending.take() {
            self.retune = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::pitch::two_tap::tests::centroid;

    #[test]
    fn window_lengths_follow_the_editor() {
        // 220 Hz guitar A: period 200.45 → 8 periods = 1603.6 → 801.8.
        assert!((window_length(200.45, 1_102.0, 1_110.0) - 801.8).abs() < 0.1);
        assert_eq!(window_length(1_102.0, 1_102.0, 1_110.0), 1_102.0);
        // Ba Pitch down: halved below 1102, then doubled under 2874.
        assert_eq!(window_length(2_866.0, 1_102.0, 2_874.0), 1_433.0);
        assert_eq!(window_length(2_866.0, 2_866.0, 2_874.0), 2_866.0);
    }

    #[test]
    fn retune_sweeps_one_window_per_period() {
        let retune = Retune::new(1.5, 800.0, RESET_UP, 44_100.0);
        let period = 4_294_967_296.0 / retune.window_increment as f64;
        assert!((period * retune.tap_increment.abs() - 800.0).abs() < 0.5);
    }

    #[test]
    fn shifts_a_sine_both_ways() {
        for latch in [false, true] {
            for (ratio, reset) in [(1.5f32, RESET_UP), (0.75, RESET_DOWN)] {
                let sample_rate = 48_000.0;
                let base = if latch { 0.0 } else { 1_000.0 };
                let retune = Retune::new(ratio, 1_000.0, reset, sample_rate);
                let mut shifter = DualWindow::new(sample_rate, base, 6_000.0, latch, retune);
                let n = sample_rate as usize;
                let out: Vec<f32> = (0..n)
                    .map(|i| {
                        shifter
                            .process((i as f32 * 264.6 / sample_rate * std::f32::consts::TAU).sin())
                    })
                    .collect();
                let found = centroid(&out[n / 2..], sample_rate, 225.0 * ratio, 304.0 * ratio);
                assert!(
                    (found - 264.6 * ratio).abs() < 2.0,
                    "{latch} {ratio}: {found}"
                );
            }
        }
    }
}
