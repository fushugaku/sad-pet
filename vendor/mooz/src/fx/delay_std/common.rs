//! Pieces shared by the delay_std ports: ROM words, Zoom-convention biquads
//! re-derived for the host rate, Main's two float setters and the tap
//! mute/reopen sequence most delay editors run when Time changes.
use crate::dsp::{Coefficients, DelayLine};
use crate::zoom::{self, ORIGINAL_RATE};

/// `context[7]` (Main c00cc94c) ramps every tap open/mute over 705.6 samples
/// (16 ms at 44.1 kHz): the editors pass `0x44306666` as the divisor.
pub const TAP_RAMP_44K: f32 = 705.6;

/// A ROM word as an f32.
#[inline]
pub const fn f(word: u32) -> f32 {
    f32::from_bits(word)
}

/// A Q31 ROM word as a fraction.
#[inline]
pub const fn q31(word: u32) -> f32 {
    word as i32 as f32 * (1.0 / 2_147_483_648.0)
}

/// Host samples per original 44.1 kHz sample.
#[inline]
pub fn rate_ratio(sample_rate: f32) -> f32 {
    sample_rate / ORIGINAL_RATE
}

/// Milliseconds to host samples.
#[inline]
pub fn ms_to_samples(ms: f32, sample_rate: f32) -> f32 {
    ms * sample_rate * 0.001
}

/// A firmware biquad (`b0 b1 b2 a1 a2` with `y = b·x + a1 y1 + a2 y2`, i.e.
/// feedback signs stored positive) designed at 44.1 kHz, re-derived for the
/// host rate through its analog prototype.
pub fn rom_biquad(words: [u32; 5], sample_rate: f32) -> Coefficients {
    zoom_biquad(words.map(f), sample_rate)
}

/// Like [`rom_biquad`] for coefficients already in f32.
pub fn zoom_biquad(c: [f32; 5], sample_rate: f32) -> Coefficients {
    Coefficients {
        b0: c[0],
        b1: c[1],
        b2: c[2],
        a1: -c[3],
        a2: -c[4],
    }
    .resample(ORIGINAL_RATE, sample_rate)
}

/// A one-pole pole (`y = (1 - p) x + p y`) re-derived for the host rate.
#[inline]
pub fn pole_at(pole_44k: f32, sample_rate: f32) -> f32 {
    pole_44k.max(0.0).powf(ORIGINAL_RATE / sample_rate)
}

/// The smoothing coefficient `a` of `y += a (x - y)` re-derived for the host rate.
#[inline]
pub fn smoothing_at(coefficient_44k: f32, sample_rate: f32) -> f32 {
    1.0 - pole_at(1.0 - coefficient_44k, sample_rate)
}

/// Main c00cc94c (`context[7]`): a coefficient moves linearly to its target
/// in a fixed number of samples (the step is `|target - current| / samples`).
#[derive(Debug, Clone, Copy)]
pub struct Fade {
    pub current: f32,
    pub target: f32,
    step: f32,
}

impl Fade {
    pub const fn new(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
        }
    }

    /// Reach `target` in `samples` host samples.
    pub fn start(&mut self, target: f32, samples: f32) {
        self.target = target;
        self.step = ((target - self.current).abs() / samples.max(1.0)).max(1e-7);
    }

    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }

    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.current != self.target {
            let difference = self.target - self.current;
            if difference.abs() <= self.step * 1.001 {
                self.current = self.target;
            } else {
                self.current += self.step.copysign(difference);
            }
        }
        self.current
    }

    #[inline]
    pub fn idle(&self) -> bool {
        self.current == self.target
    }
}

/// One delay tap's gain as the editors drive it: `mute` ramps it to zero
/// in 16 ms, `open_after` schedules the 16 ms reopening ramp (the original
/// `context[16]` timer callbacks `TapOpen`/`tapmuteOpen`/`onf_aft`).
#[derive(Debug, Clone, Copy)]
pub struct Gate {
    pub gain: Fade,
    countdown: u32,
    ramp: f32,
}

impl Gate {
    pub fn new(sample_rate: f32, open: bool) -> Self {
        Self {
            gain: Fade::new(if open { 1.0 } else { 0.0 }),
            countdown: 0,
            ramp: TAP_RAMP_44K * rate_ratio(sample_rate),
        }
    }

    pub fn mute(&mut self) {
        self.countdown = 0;
        self.gain.start(0.0, self.ramp);
    }

    /// Reopen once `samples` have passed (0 opens now).
    pub fn open_after(&mut self, samples: u32) {
        if samples == 0 {
            self.gain.start(1.0, self.ramp);
            self.countdown = 0;
        } else {
            self.countdown = samples;
        }
    }

    pub fn set_open(&mut self, open: bool) {
        self.countdown = 0;
        self.gain.jump(if open { 1.0 } else { 0.0 });
    }

    /// True once a mute has fully closed the tap.
    #[inline]
    pub fn closed(&self) -> bool {
        self.gain.current == 0.0 && self.gain.target == 0.0
    }

    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.countdown > 0 {
            self.countdown -= 1;
            if self.countdown == 0 {
                self.gain.start(1.0, self.ramp);
            }
        }
        self.gain.tick()
    }

    pub fn clear(&mut self) {
        self.countdown = 0;
        self.gain.jump(1.0);
    }
}

/// A step setter (`context[8]`, Main c00cc8c8) with the common editor step.
pub fn setter(value: f32, sample_rate: f32) -> zoom::Ramp {
    zoom::Ramp::new(value, zoom::DEFAULT_STEP, sample_rate)
}

/// A delay line holding at least `ms` milliseconds plus `extra` samples.
pub fn line_for_ms(ms: f32, extra: usize, sample_rate: f32) -> DelayLine {
    DelayLine::new(ms_to_samples(ms, sample_rate).ceil() as usize + extra + 4)
}

/// Integer delay in samples for a time in milliseconds, kept inside `line`.
#[inline]
pub fn tap_samples(ms: f32, sample_rate: f32, line: &DelayLine) -> usize {
    (ms_to_samples(ms, sample_rate) as usize).min(line.capacity())
}

/// Clamp a parameter value to `0..=max`, mapping non-finite values to 0.
#[inline]
pub fn param(value: f32, max: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, max)
    } else {
        0.0
    }
}

/// A stepped parameter as an index.
#[inline]
pub fn index(value: f32, max: usize) -> usize {
    (param(value, max as f32).round() as usize).min(max)
}

/// The note durations a synced value selects, used by every Sync display.
pub const SYNC_LABELS: [&str; 16] = [
    "Off", "1/16", "1/8T", "1/16.", "1/8", "1/4T", "1/8.", "1/4", "1/4.", "1/4×2", "1/4×3",
    "1/4×4", "1/4×5", "1/4×6", "1/4×7", "1/4×8",
];

#[cfg(test)]
pub mod testing {
    use crate::{Effect, Values};

    /// Run `frames` of silence (or `input`) and collect the left/right outputs.
    pub fn render(
        effect: &mut dyn Effect,
        frames: usize,
        mut input: impl FnMut(usize) -> [f32; 2],
    ) -> Vec<[f32; 2]> {
        (0..frames).map(|i| effect.process(input(i))).collect()
    }

    pub fn impulse(i: usize) -> [f32; 2] {
        if i == 0 { [1.0, 1.0] } else { [0.0, 0.0] }
    }

    /// Index of the largest absolute sample of a channel at or after `from`.
    pub fn peak_index(output: &[[f32; 2]], channel: usize, from: usize) -> usize {
        let mut best = from;
        for (i, frame) in output.iter().enumerate().skip(from) {
            if frame[channel].abs() > output[best][channel].abs() {
                best = i;
            }
        }
        best
    }

    pub fn rms(output: &[[f32; 2]], channel: usize, from: usize) -> f32 {
        let slice = &output[from..];
        (slice.iter().map(|f| f[channel] * f[channel]).sum::<f32>() / slice.len() as f32).sqrt()
    }

    pub fn sine(hz: f32, amplitude: f32, sample_rate: f32) -> impl FnMut(usize) -> [f32; 2] {
        move |i| {
            let x = amplitude * (std::f32::consts::TAU * hz * i as f32 / sample_rate).sin();
            [x, x]
        }
    }

    pub fn with(defaults: Values, changes: &[(usize, f32)]) -> Values {
        let mut values = defaults;
        for &(i, v) in changes {
            values[i] = v;
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_reaches_target_in_the_requested_time() {
        let mut fade = Fade::new(1.0);
        fade.start(0.0, 100.0);
        for _ in 0..99 {
            fade.tick();
        }
        assert!(fade.current > 0.0 && fade.current < 0.02);
        fade.tick();
        assert_eq!(fade.current, 0.0);
    }

    #[test]
    fn gate_mutes_then_reopens_after_its_timer() {
        let mut gate = Gate::new(44_100.0, true);
        gate.mute();
        let mut n = 0;
        while !gate.closed() {
            gate.tick();
            n += 1;
        }
        assert!((704..=707).contains(&n), "{n}");
        gate.open_after(1000);
        for _ in 0..999 {
            assert_eq!(gate.tick(), 0.0);
        }
        for _ in 0..800 {
            gate.tick();
        }
        assert_eq!(gate.gain.current, 1.0);
    }

    #[test]
    fn rom_biquads_keep_their_response_at_other_rates() {
        // DELAY HiDMP 0: a one-pole low-pass near 250 Hz.
        let words = [0x3c8f5b79, 0x3c8f5b79, 0, 0x3f770a48, 0];
        let at44 = rom_biquad(words, 44_100.0);
        let at96 = rom_biquad(words, 96_000.0);
        for hz in [100.0, 250.0, 1000.0, 4000.0] {
            let a = at44.magnitude(44_100.0, hz);
            let b = at96.magnitude(96_000.0, hz);
            assert!((a - b).abs() < 0.05 * a.max(0.05), "{hz}: {a} {b}");
        }
        assert!((at44.magnitude(44_100.0, 10.0) - 1.0).abs() < 1e-3);
    }
}
