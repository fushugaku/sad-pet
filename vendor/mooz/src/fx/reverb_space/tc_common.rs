//! Pieces shared by Ambience (AMBIENCE.ZDL) and DynaRev (DYNAREV.ZDL): both
//! callbacks are the same 0x16a0-byte "tc" network (one ring with a write
//! cursor, input filters, a pre-delay, lossy all-pass diffusers, two-band damped
//! combs with output taps and an output filter chain), and their editors call
//! the same Main services: the filter designer (`context[25]`, Main c00bc2e0),
//! `TBL_TO_VAL`/`TBL_TO_VAL_int` and the RT60 rule `g^(delay_ms / T60_ms)`.
use crate::dsp::{Biquad, Coefficients, DelayLine, flush};
use crate::zoom::ORIGINAL_RATE;
use std::f64::consts::PI;

/// `TBL_TO_VAL(count, value, table, len)`: linear interpolation of a float
/// table at `value * (len - 1) / (count - 1)`, exact at both ends. Fractional
/// values interpolate continuously.
pub fn table(values: &[f32], count: u32, value: f32) -> f32 {
    let last = values.len() - 1;
    let position = (value.max(0.0) * last as f32 / (count - 1) as f32).min(last as f32);
    let index = (position as usize).min(last.saturating_sub(1));
    let fraction = position - index as f32;
    values[index] + (values[index + 1] - values[index]) * fraction
}

/// `TBL_TO_VAL_int`: the same on an integer table, the interpolated step
/// truncated toward zero like the original `SPTRUNC`.
pub fn table_int(values: &[i32], count: u32, value: f32) -> f32 {
    let last = values.len() - 1;
    let position = (value.max(0.0) * last as f32 / (count - 1) as f32).min(last as f32);
    let index = (position as usize).min(last.saturating_sub(1));
    let fraction = position - index as f32;
    let step = ((values[index + 1] - values[index]) as f32 * fraction).trunc();
    values[index] as f32 + step
}

/// The decay rule of both `decay_Calc_set` editors: a loop of `delay_ms`
/// decays by `g` over `t60_ms` (`g` is about 0.001, -60 dB), so its gain is
/// `g^(delay_ms / t60_ms)`; a zero time silences the loop.
pub fn decay_gain(g: f32, delay_ms: f32, t60_ms: f32) -> f32 {
    if t60_ms <= 0.0 {
        0.0
    } else {
        g.powf(delay_ms / t60_ms)
    }
}

/// A first-order section in the firmware's storage order:
/// `y = b0 x + b1 x[-1] + fb y[-1]` (`fb` is minus the usual a1).
#[derive(Debug, Clone, Copy)]
pub struct FirstOrder {
    pub b0: f32,
    pub b1: f32,
    pub fb: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrder {
    pub const IDENTITY: Self = Self::new(1.0, 0.0, 0.0);

    pub const fn new(b0: f32, b1: f32, fb: f32) -> Self {
        Self {
            b0,
            b1,
            fb,
            x1: 0.0,
            y1: 0.0,
        }
    }

    /// A 44.1 kHz ROM section re-derived for `sample_rate` through its
    /// bilinear analog prototype (same corner, same DC and HF gains).
    pub fn rom(b0: f32, b1: f32, fb: f32, sample_rate: f32) -> Self {
        if (sample_rate - ORIGINAL_RATE).abs() < 0.5 {
            return Self::new(b0, b1, fb);
        }
        let (b0, b1, a1) = (f64::from(b0), f64::from(b1), -f64::from(fb));
        // z^-1 = (1 - u) / (1 + u), u = s / (2 fs): multiply through by (1 + u).
        let (n0, n1) = (b0 + b1, b0 - b1);
        let (d0, d1) = (1.0 + a1, 1.0 - a1);
        let r = f64::from(sample_rate) / f64::from(ORIGINAL_RATE);
        let (n1, d1) = (n1 * r, d1 * r);
        // Back with u = (1 - z^-1) / (1 + z^-1).
        let (nb0, nb1) = (n0 + n1, n0 - n1);
        let (da0, da1) = (d0 + d1, d0 - d1);
        Self::new((nb0 / da0) as f32, (nb1 / da0) as f32, (-da1 / da0) as f32)
    }

    pub fn set(&mut self, coefficients: (f32, f32, f32)) {
        (self.b0, self.b1, self.fb) = coefficients;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = flush(self.b0 * x + self.b1 * self.x1 + self.fb * self.y1);
        self.x1 = x;
        self.y1 = y;
        y
    }

    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }

    /// Magnitude at `hz` (tests).
    #[cfg(test)]
    pub fn magnitude(&self, sample_rate: f32, hz: f32) -> f32 {
        Coefficients {
            b0: self.b0,
            b1: self.b1,
            b2: 0.0,
            a1: -self.fb,
            a2: 0.0,
        }
        .magnitude(sample_rate, hz)
    }
}

/// First-order designs of the Main filter service (c00bc2e0) with
/// `K = tan(pi f / fs)`: type 0 low-pass, type 1 high-pass and type 8 high
/// shelf (a cut moves the corner down by the cut factor, Zoelzer style).
/// Returns `(b0, b1, fb)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Design {
    LowPass,
    HighPass,
    HighShelf,
}

pub fn design(kind: Design, hz: f32, gain_db: f32, sample_rate: f32) -> (f32, f32, f32) {
    let fs = f64::from(sample_rate);
    let hz = f64::from(hz).clamp(1.0, fs * 0.45);
    let tan = |f: f64| (PI * f / fs).tan();
    match kind {
        Design::LowPass => {
            let k = tan(hz);
            let n = 1.0 / (1.0 + k);
            ((k * n) as f32, (k * n) as f32, ((1.0 - k) * n) as f32)
        }
        Design::HighPass => {
            let k = tan(hz);
            let n = 1.0 / (1.0 + k);
            (n as f32, -n as f32, ((1.0 - k) * n) as f32)
        }
        Design::HighShelf => {
            let v = 10f64.powf(f64::from(gain_db).abs() / 20.0);
            if gain_db >= 0.0 {
                let k = tan(hz);
                let n = 1.0 / (1.0 + k);
                (
                    ((v + k) * n) as f32,
                    ((k - v) * n) as f32,
                    ((1.0 - k) * n) as f32,
                )
            } else {
                let k = tan(hz / v);
                let n = 1.0 / (1.0 + k);
                (
                    ((1.0 / v + k) * n) as f32,
                    ((k - 1.0 / v) * n) as f32,
                    ((1.0 - k) * n) as f32,
                )
            }
        }
    }
}

/// A 44.1 kHz ROM biquad stored as `[b0, b1, b2, -a1, -a2]`, re-derived for
/// the host rate.
pub fn rom_biquad(words: [f32; 5], sample_rate: f32) -> Biquad {
    Biquad::new(
        Coefficients {
            b0: words[0],
            b1: words[1],
            b2: words[2],
            a1: -words[3],
            a2: -words[4],
        }
        .resample(ORIGINAL_RATE, sample_rate),
    )
}

/// The one-pole low-pass of the damping crossover (`lp += c (x - lp)`), a ROM
/// coefficient at 44.1 kHz moved to the host rate with the same corner.
pub fn one_pole_rom(coefficient: f32, sample_rate: f32) -> f32 {
    let per_sample = -(1.0 - f64::from(coefficient)).ln();
    (1.0 - (-per_sample * f64::from(ORIGINAL_RATE) / f64::from(sample_rate)).exp()) as f32
}

/// A delay in 44.1 kHz samples scaled to the host rate (at least one sample).
pub fn samples(at_44k: f32, sample_rate: f32) -> usize {
    ((at_44k * sample_rate / ORIGINAL_RATE).round() as usize).max(1)
}

/// One fixed region of the original ring: written once per frame after it has
/// been read, read `delay` frames later.
#[derive(Debug, Clone)]
pub struct Line {
    line: DelayLine,
    pub delay: usize,
}

impl Line {
    pub fn new(delay: usize) -> Self {
        Self {
            line: DelayLine::new(delay + 2),
            delay: delay.max(1),
        }
    }
    pub fn with_capacity(delay: usize, capacity: usize) -> Self {
        Self {
            line: DelayLine::new(capacity.max(delay) + 2),
            delay: delay.max(1),
        }
    }
    /// The sample written `delay` frames ago (read before this frame's write).
    #[inline]
    pub fn out(&self) -> f32 {
        self.line.read(self.delay - 1)
    }
    /// An extra tap `delay` frames back.
    #[inline]
    pub fn tap(&self, delay: usize) -> f32 {
        self.line.read(delay - 1)
    }
    /// A fractional tap, linearly interpolated like the modulated taps of the
    /// original (`delay >= 1`).
    #[inline]
    pub fn tap_linear(&self, delay: f32) -> f32 {
        self.line.read_linear(delay - 1.0)
    }
    #[inline]
    pub fn write(&mut self, x: f32) {
        self.line.write(flush(x));
    }
    pub fn clear(&mut self) {
        self.line.clear();
    }
}

/// Main `context[7]` (c00cc94c): a coefficient moves linearly to a new value
/// in a fixed time (705.6 samples at 44.1 kHz, 16 ms, in these editors).
#[derive(Debug, Clone, Copy)]
pub struct Glide {
    pub current: f32,
    target: f32,
    step: f32,
    samples: f32,
}

impl Glide {
    pub fn new(value: f32, seconds: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            samples: (seconds * sample_rate).max(1.0),
        }
    }
    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step = (target - self.current).abs() / self.samples;
        }
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.current != self.target {
            let difference = self.target - self.current;
            if difference.abs() <= self.step {
                self.current = self.target;
            } else {
                self.current += self.step.copysign(difference);
            }
        }
        self.current
    }
}

/// A pre-delay read position that changes by a 16 ms crossfade between the
/// old and the new tap. The originals mute the pre-delayed input (16 ms ramp),
/// jump the read pointer and reopen it; the crossfade avoids the gap and the
/// clicks while a modulator moves the value.
#[derive(Debug, Clone, Copy)]
pub struct TapFade {
    pub current: f32,
    previous: f32,
    target: f32,
    fade: f32,
    step: f32,
}

impl TapFade {
    pub fn new(delay: f32, sample_rate: f32) -> Self {
        Self {
            current: delay,
            previous: delay,
            target: delay,
            fade: 1.0,
            step: 1.0 / (0.016 * sample_rate),
        }
    }
    pub fn set(&mut self, delay: f32) {
        self.target = delay;
    }
    pub fn jump(&mut self) {
        self.current = self.target;
        self.previous = self.target;
        self.fade = 1.0;
    }
    #[inline]
    pub fn tick(&mut self) {
        if self.fade >= 1.0 {
            if self.target != self.current {
                self.previous = self.current;
                self.current = self.target;
                self.fade = 0.0;
            }
        } else {
            self.fade = (self.fade + self.step).min(1.0);
        }
    }
    /// The tap `offset` samples beyond the pre-delay, crossfaded while moving.
    #[inline]
    pub fn read(&self, line: &Line, offset: f32) -> f32 {
        let new = line.tap_linear(self.current + offset);
        if self.fade >= 1.0 {
            new
        } else {
            let old = line.tap_linear(self.previous + offset);
            old + (new - old) * self.fade
        }
    }
}

/// The lossy all-pass of both diffusers: `w = x + g d` is written and
/// `y = c x + h d` leaves (C47..50 in Ambience, C62..65 in DynaRev; with
/// `c = -g` and `h = 1 - g^2` it would be a true all-pass).
#[derive(Debug, Clone)]
pub struct Diffuser {
    pub line: Line,
}

impl Diffuser {
    pub fn new(delay: usize) -> Self {
        Self {
            line: Line::new(delay),
        }
    }
    #[inline]
    pub fn process(&mut self, x: f32, g: f32, c: f32, h: f32) -> f32 {
        let d = self.line.out();
        self.line.write(x + g * d);
        c * x + h * d
    }
}

/// A two-band damped comb: the delayed signal is split by a one-pole
/// crossover, the low band scaled by `low`, the high band by `high`, the input
/// added, then two first-order sections (tone / fixed shelf) before the write.
#[derive(Debug, Clone)]
pub struct Comb {
    pub line: Line,
    lp: f32,
    pub tone: FirstOrder,
    pub shelf: FirstOrder,
}

impl Comb {
    pub fn new(delay: usize, tone: FirstOrder, shelf: FirstOrder) -> Self {
        Self {
            line: Line::new(delay),
            lp: 0.0,
            tone,
            shelf,
        }
    }
    /// `delayed` is this frame's read of the comb line (possibly a modulated
    /// tap); returns the value written.
    #[inline]
    pub fn process(
        &mut self,
        delayed: f32,
        input: f32,
        crossover: f32,
        low: f32,
        high: f32,
    ) -> f32 {
        self.lp = flush(self.lp + crossover * (delayed - self.lp));
        let v = low * self.lp + high * (delayed - self.lp) + input;
        let v = self.shelf.process(self.tone.process(v));
        self.line.write(v);
        v
    }
    pub fn clear(&mut self) {
        self.line.clear();
        self.lp = 0.0;
        self.tone.clear();
        self.shelf.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_interpolates_like_tbl_to_val() {
        let t = [0.0, 1.0, 4.0];
        assert_eq!(table(&t, 101, 0.0), 0.0);
        assert_eq!(table(&t, 101, 100.0), 4.0);
        assert!((table(&t, 101, 25.0) - 0.5).abs() < 1e-6);
        assert!((table(&t, 101, 75.0) - 2.5).abs() < 1e-6);
        let ti = [8, 9, 11, 14, 16, 27, 57, 82, 115, 145, 149];
        assert_eq!(table_int(&ti, 101, 0.0), 8.0);
        assert_eq!(table_int(&ti, 101, 100.0), 149.0);
        assert_eq!(table_int(&ti, 101, 55.0), 42.0); // 27 + trunc(0.5 * 30)
    }

    #[test]
    fn designs_match_the_rom_sets() {
        // Ambience C107..111: the type-1 high-pass the decay editor writes.
        let (b0, b1, fb) = design(Design::HighPass, 204.0, 0.0, 44_100.0);
        assert!((b0 - 0.985_651).abs() < 2e-4 && (b1 + 0.985_651).abs() < 2e-4);
        assert!((fb - 0.971_303).abs() < 4e-4);
        // DynaRev C57..61: type-0 low-pass at 5 kHz.
        let (b0, b1, fb) = design(Design::LowPass, 5000.0, 0.0, 44_100.0);
        assert!((b0 - 0.271_168).abs() < 1e-3 && (b1 - b0).abs() < 1e-7);
        assert!((fb - 0.457_663).abs() < 2e-3);
        // A high shelf keeps unity at DC and reaches its gain at Nyquist.
        for gain in [-12.0f32, 6.0] {
            let s = design(Design::HighShelf, 3000.0, gain, 44_100.0);
            let f = FirstOrder::new(s.0, s.1, s.2);
            assert!((f.magnitude(44_100.0, 1.0) - 1.0).abs() < 1e-3);
            let nyquist = crate::dsp::gain_to_db(f.magnitude(44_100.0, 22_049.0));
            assert!((nyquist - gain).abs() < 0.2, "{gain}: {nyquist}");
        }
    }

    #[test]
    fn rom_sections_keep_their_shape_at_other_rates() {
        let at44 = FirstOrder::new(0.271_168, 0.271_168, 0.457_663);
        let at96 = FirstOrder::rom(0.271_168, 0.271_168, 0.457_663, 96_000.0);
        for hz in [100.0, 2000.0, 5000.0] {
            let a = crate::dsp::gain_to_db(at44.magnitude(44_100.0, hz));
            let b = crate::dsp::gain_to_db(at96.magnitude(96_000.0, hz));
            assert!((a - b).abs() < 0.3, "{hz}: {a} {b}");
        }
        let c = one_pole_rom(0.129_666, 44_100.0);
        assert!((c - 0.129_666).abs() < 1e-6);
        assert!(one_pole_rom(0.129_666, 96_000.0) < c);
    }

    #[test]
    fn decay_rule_reaches_minus_60_db_at_t60() {
        // 16 passes of a 62.5 ms loop make one second.
        let g = decay_gain(0.001, 62.5, 1000.0);
        assert!((g.powi(16) - 0.001).abs() < 1e-5);
        assert_eq!(decay_gain(0.001, 50.0, 0.0), 0.0);
    }
}
