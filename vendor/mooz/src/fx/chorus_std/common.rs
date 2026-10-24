//! Shared pieces of the chorus_std group: the ROM filter tables the editors
//! copy, first-order sections re-derived for the host rate, per-sample poles
//! rescaled from 44.1 kHz, Main's two coefficient setters and a click-free
//! delay-time follower.
use crate::{
    MAX_PARAMS, Values,
    catalog::ModelInfo,
    dsp::{Coefficients, flush},
    zoom::ORIGINAL_RATE,
};

/// One Q31 unit.
pub const Q31: f32 = 1.0 / 2_147_483_648.0;

/// Main's timed setter (`context[7]`, c00cc94c) divides the distance by
/// 705.6 samples: every move takes 16 ms.
pub const TIMED_SAMPLES: f32 = 705.6;

/// Main's integer setter step (`context[10]`, 0x0003deb7 per sample) used for
/// the Q31 depth words, as a fraction of full scale.
pub const DEPTH_STEP: f32 = 253_623.0 * Q31;

/// A raw ROM word as f32.
pub const fn word(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// Default values of a catalog entry.
pub fn defaults(info: &ModelInfo) -> Values {
    let mut values = [0.0; MAX_PARAMS];
    for (value, param) in values.iter_mut().zip(info.params) {
        *value = f32::from(param.default);
    }
    values
}

/// Values limited to each parameter's `0..=max` (non-finite values become 0).
pub fn sanitize(values: &Values, info: &ModelInfo) -> Values {
    let mut out = [0.0; MAX_PARAMS];
    for ((out, &value), param) in out.iter_mut().zip(values).zip(info.params) {
        *out = if value.is_finite() {
            value.clamp(0.0, f32::from(param.max))
        } else {
            0.0
        };
    }
    out
}

/// `NaN` marks "never applied" so the first `set` computes everything.
pub const UNSET: Values = [f32::NAN; MAX_PARAMS];

/// Index into a stepped table from a possibly fractional pedal value.
pub fn step(value: f32, max: usize) -> usize {
    if value.is_finite() {
        (value.round().max(0.0) as usize).min(max)
    } else {
        0
    }
}

/// Tone table copied by the Tone editors of Chorus (800003e8), StereoCho,
/// Vibrato, Ensemble and Ba Ensmbl (all identical): per row a first-order
/// high-pass `(b0, b1, fb)` then a first-order low-pass `(b0, b1, fb)`.
/// Rows 0..=7 are HP 15 Hz with LP 250 (+3 dB), 500 (+2 dB), 1k (+1 dB), 2k,
/// 3.5k, 5k, 10k and 22 kHz; rows 8..=10 are LP 22 kHz with HP 250 Hz,
/// 800 Hz (+3 dB) and 2 kHz (+6 dB).
pub const TONE: [[u32; 6]; 11] = [
    [
        0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417, 0x3cca7f62, 0x3cca7f62, 0x3f770a48,
    ],
    [
        0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417, 0x3d316d33, 0x3d316d33, 0x3f6e6215,
    ],
    [
        0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417, 0x3d990d92, 0x3d990d92, 0x3f5de5d9,
    ],
    [
        0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417, 0x3e00767d, 0x3e00767d, 0x3f3fc4c2,
    ],
    [
        0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417, 0x3e4fd2ef, 0x3e4fd2ef, 0x3f181689,
    ],
    [
        0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417, 0x3e8ad692, 0x3e8ad692, 0x3eea52dc,
    ],
    [
        0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417, 0x3eed461d, 0x3eed461d, 0x3d95cf1b,
    ],
    [
        0x3f7fba0c, 0xbf7fba0c, 0x3f7f7417, 0x3f7f1765, 0x3f7f1765, 0xbf7e2eca,
    ],
    [
        0x3f7b8524, 0xbf7b8524, 0x3f770a48, 0x3f7f1765, 0x3f7f1765, 0xbf7e2eca,
    ],
    [
        0x3fab0bd7, 0xbfab0bd7, 0x3f645dab, 0x3f7f1765, 0x3f7f1765, 0xbf7e2eca,
    ],
    [
        0x3fdf5a9c, 0xbfdf5a9c, 0x3f3fc4c2, 0x3f7f1765, 0x3f7f1765, 0xbf7e2eca,
    ],
];

/// The high-pass half of a [`TONE`] row.
pub fn tone_hp(row: &[u32; 6]) -> [u32; 3] {
    [row[0], row[1], row[2]]
}

/// The low-pass half of a [`TONE`] row.
pub fn tone_lp(row: &[u32; 6]) -> [u32; 3] {
    [row[3], row[4], row[5]]
}

/// First-order section in the callbacks' form `y = b0 x + b1 x1 + fb y1`.
#[derive(Debug, Clone, Copy, Default)]
pub struct FirstOrder {
    pub b0: f32,
    pub b1: f32,
    pub fb: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrder {
    /// ROM words `(b0, b1, fb)` designed at 44.1 kHz, re-derived for
    /// `sample_rate` (see [`warp`]).
    pub fn rom(words: [u32; 3], sample_rate: f32) -> Self {
        let mut section = Self::default();
        section.set_rom(words, sample_rate);
        section
    }

    pub fn set_rom(&mut self, words: [u32; 3], sample_rate: f32) {
        let b0 = f64::from(word(words[0]));
        let b1 = f64::from(word(words[1]));
        let a1 = -f64::from(word(words[2]));
        // In u = s / (2 fs): N(u) = n0 + n1 u, D(u) = d0 + d1 u.
        let (n0, n1) = (b0 + b1, b0 - b1);
        let (d0, d1) = (1.0 + a1, 1.0 - a1);
        let c = warp(d0 / d1.abs().max(1e-12), sample_rate);
        let (n1, d1) = (n1 * c, d1 * c);
        let norm = d0 + d1;
        self.b0 = ((n0 + n1) / norm) as f32;
        self.b1 = ((n0 - n1) / norm) as f32;
        self.fb = (-(d0 - d1) / norm) as f32;
    }

    /// Main's biquad designer (context[52], c00bc2e0) type 0: bilinear
    /// first-order low-pass, designed directly at the host rate.
    pub fn set_lowpass(&mut self, hz: f32, sample_rate: f32) {
        let hz = f64::from(hz.clamp(1.0, sample_rate * 0.49));
        let k = (std::f64::consts::PI * hz / f64::from(sample_rate)).tan();
        self.b0 = (k / (1.0 + k)) as f32;
        self.b1 = self.b0;
        self.fb = ((1.0 - k) / (1.0 + k)) as f32;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = flush(self.b0 * x + self.b1 * self.x1 + self.fb * self.y1);
        self.x1 = x;
        self.y1 = y;
        y
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

    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

/// Ratio `c` of the bilinear variable `u = s / (2 fs)` between 44.1 kHz and
/// `sample_rate` (`u_44k = c u_host`) that keeps a ROM filter's response at
/// its natural frequency, given as `u0` at 44.1 kHz. With the anchor at DC it
/// would be `fs / 44.1k` ([`Coefficients::resample`], which keeps the analog
/// prototype and so moves digital corners at 96 kHz); anchoring at the corner
/// keeps it in place. Anchors are limited to 30 % of the lower rate so
/// near-Nyquist "open" sections stay open.
pub fn warp(u0: f64, sample_rate: f32) -> f64 {
    let from = f64::from(ORIGINAL_RATE);
    let to = f64::from(sample_rate);
    if (from - to).abs() < 0.5 {
        return 1.0;
    }
    let pi = std::f64::consts::PI;
    let anchor = (from / pi * u0.atan()).clamp(1.0, 0.3 * from.min(to));
    (pi * anchor / from).tan() / (pi * anchor / to).tan()
}

/// A ROM biquad in the callbacks' form `(b0, b1, b2, fb1, fb2)` with
/// `y = b0 x + b1 x1 + b2 x2 + fb1 y1 + fb2 y2` designed at 44.1 kHz,
/// re-derived for the host rate (see [`warp`]).
pub fn rom_biquad(words: [u32; 5], sample_rate: f32) -> Coefficients {
    let b = [
        f64::from(word(words[0])),
        f64::from(word(words[1])),
        f64::from(word(words[2])),
    ];
    let a = [1.0, -f64::from(word(words[3])), -f64::from(word(words[4]))];
    // z^-1 = (1 - u) / (1 + u): polynomials in u.
    let to_u = |p: [f64; 3]| [p[0] + p[1] + p[2], 2.0 * (p[0] - p[2]), p[0] - p[1] + p[2]];
    let (bu, au) = (to_u(b), to_u(a));
    let c = warp((au[0] / au[2].abs().max(1e-18)).abs().sqrt(), sample_rate);
    let scale = |p: [f64; 3]| [p[0], p[1] * c, p[2] * c * c];
    let (bu, au) = (scale(bu), scale(au));
    // u = (1 - z^-1) / (1 + z^-1): back to z^-1.
    let to_z = |p: [f64; 3]| [p[0] + p[1] + p[2], 2.0 * (p[0] - p[2]), p[0] - p[1] + p[2]];
    let (bz, az) = (to_z(bu), to_z(au));
    Coefficients {
        b0: (bz[0] / az[0]) as f32,
        b1: (bz[1] / az[0]) as f32,
        b2: (bz[2] / az[0]) as f32,
        a1: (az[1] / az[0]) as f32,
        a2: (az[2] / az[0]) as f32,
    }
}

/// A per-sample one-pole `y = input x + pole y` given at 44.1 kHz, rescaled
/// to `sample_rate` with the same time constant and DC gain.
pub fn one_pole(input: f32, pole: f32, sample_rate: f32) -> (f32, f32) {
    let ratio = f64::from(ORIGINAL_RATE) / f64::from(sample_rate);
    let host_pole = f64::from(pole).powf(ratio);
    let gain = f64::from(input) / (1.0 - f64::from(pole));
    ((gain * (1.0 - host_pole)) as f32, host_pole as f32)
}

/// A per-sample decay multiplier given at 44.1 kHz, at `sample_rate`.
pub fn decay(multiplier: f32, sample_rate: f32) -> f32 {
    f64::from(multiplier).powf(f64::from(ORIGINAL_RATE) / f64::from(sample_rate)) as f32
}

/// A u32 phase increment per 44.1 kHz sample, at `sample_rate`.
pub fn increment(per_44k: f32, sample_rate: f32) -> u32 {
    (f64::from(per_44k) * f64::from(ORIGINAL_RATE) / f64::from(sample_rate))
        .round()
        .clamp(0.0, f64::from(u32::MAX)) as u32
}

/// The u32 phase increment of an LFO at `hz`.
pub fn hz_increment(hz: f32, sample_rate: f32) -> u32 {
    (f64::from(hz) / f64::from(sample_rate) * 4_294_967_296.0)
        .round()
        .clamp(0.0, f64::from(u32::MAX)) as u32
}

/// LFO frequency of a u32 increment (tests).
#[cfg(test)]
pub fn increment_hz(increment: u32, sample_rate: f32) -> f32 {
    (f64::from(increment) * f64::from(sample_rate) / 4_294_967_296.0) as f32
}

/// The callbacks' `ABS(phase)` of a wrapping Q31 phase: a 0..1 triangle.
#[inline]
pub fn triangle(phase: u32) -> f32 {
    (phase as i32).unsigned_abs() as f32 * Q31
}

/// Main's timed float setter (context[7]): a linear move that reaches its
/// target in 16 ms whatever the distance.
#[derive(Debug, Clone, Copy)]
pub struct TimedRamp {
    pub current: f32,
    target: f32,
    step: f32,
    samples: f32,
    minimum: f32,
}

impl TimedRamp {
    pub fn new(value: f32, sample_rate: f32) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        Self {
            current: value,
            target: value,
            step: 0.0,
            samples: TIMED_SAMPLES * scale,
            // The relative minimum step (|desired| * 0x3eb0c6f7a0b5ed8d).
            minimum: 1.0e-6 / scale,
        }
    }

    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step =
                ((target - self.current).abs() / self.samples).max(target.abs() * self.minimum);
        }
    }

    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }

    /// Finish the move at once.
    pub fn settle(&mut self) {
        self.current = self.target;
    }

    #[inline]
    pub fn tick(&mut self) -> f32 {
        let difference = self.target - self.current;
        if difference.abs() <= self.step {
            self.current = self.target;
        } else {
            self.current += self.step.copysign(difference);
        }
        self.current
    }
}

/// A linear slew with a fixed step per host sample.
#[derive(Debug, Clone, Copy)]
pub struct Slew {
    pub current: f32,
    target: f32,
    step: f32,
}

impl Slew {
    pub fn new(value: f32, step: f32) -> Self {
        Self {
            current: value,
            target: value,
            step,
        }
    }

    pub fn set(&mut self, target: f32) {
        self.target = target;
    }

    /// Finish the move at once.
    pub fn settle(&mut self) {
        self.current = self.target;
    }

    #[inline]
    pub fn tick(&mut self) -> f32 {
        let difference = self.target - self.current;
        if difference.abs() <= self.step {
            self.current = self.target;
        } else {
            self.current += self.step.copysign(difference);
        }
        self.current
    }
}

/// Follows a delay-time target without clicks. The originals mute the wet tap
/// for 16 ms around every delay change (`onf_aft` reopens it); that would
/// silence the effect whenever a modulator moves the knob, so this port
/// glides small moves at a bounded rate (at most `glide` samples per sample,
/// a short pitch bend) and crossfades jumps larger than `jump` from the old
/// tap to the new one over the same 16 ms.
#[derive(Debug, Clone, Copy)]
pub struct TapTime {
    pub current: f32,
    pub previous: f32,
    target: f32,
    glide: f32,
    jump: f32,
    fade: f32,
    fade_step: f32,
}

impl TapTime {
    /// `value` and `jump` in host samples.
    pub fn new(value: f32, glide: f32, jump: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            previous: value,
            target: value,
            glide,
            jump,
            fade: 0.0,
            fade_step: 1.0 / (TIMED_SAMPLES * sample_rate / ORIGINAL_RATE),
        }
    }

    pub fn set(&mut self, target: f32) {
        self.target = target;
    }

    pub fn jump_to(&mut self, value: f32) {
        self.current = value;
        self.previous = value;
        self.target = value;
        self.fade = 0.0;
    }

    /// Finish any glide or crossfade at once.
    pub fn settle(&mut self) {
        self.jump_to(self.target);
    }

    /// Advance one sample; returns the weight of the `previous` tap (0 when no
    /// crossfade runs).
    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.fade > 0.0 {
            self.fade = (self.fade - self.fade_step).max(0.0);
            return self.fade;
        }
        let difference = self.target - self.current;
        if difference.abs() > self.jump {
            self.previous = self.current;
            self.current = self.target;
            self.fade = 1.0 - self.fade_step;
            return self.fade;
        }
        if difference.abs() <= self.glide {
            self.current = self.target;
        } else {
            self.current += self.glide.copysign(difference);
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn tone_rows_keep_their_corners_at_other_rates() {
        for rate in [44_100.0, 48_000.0, 96_000.0] {
            // Row 5: HP 15 Hz, LP 5 kHz.
            let hp = FirstOrder::rom(tone_hp(&TONE[5]), rate);
            let lp = FirstOrder::rom(tone_lp(&TONE[5]), rate);
            assert!((gain_to_db(lp.magnitude(rate, 5000.0)) + 3.01).abs() < 0.1);
            assert!(gain_to_db(lp.magnitude(rate, 100.0)).abs() < 0.05);
            assert!((gain_to_db(hp.magnitude(rate, 15.0)) + 3.01).abs() < 0.1);
            // Row 10: HP 2 kHz with +6 dB passband.
            let hp = FirstOrder::rom(tone_hp(&TONE[10]), rate);
            assert!((gain_to_db(hp.magnitude(rate, 15_000.0)) - 5.9).abs() < 0.3);
        }
    }

    #[test]
    fn rom_biquads_keep_their_corners() {
        // A 2 kHz RBJ low-pass designed at 44.1 kHz, in the callbacks' form.
        let c = Coefficients::lowpass(44_100.0, 2000.0, 0.8);
        let words = [c.b0, c.b1, c.b2, -c.a1, -c.a2].map(f32::to_bits);
        for rate in [44_100.0, 48_000.0, 96_000.0] {
            let at = rom_biquad(words, rate);
            let direct = Coefficients::lowpass(rate, 2000.0, 0.8);
            for hz in [100.0, 1000.0, 2000.0, 4000.0] {
                let error =
                    gain_to_db(at.magnitude(rate, hz)) - gain_to_db(direct.magnitude(rate, hz));
                assert!(error.abs() < 0.05, "{rate} {hz}: {error}");
            }
        }
        // An open 22 kHz first-order low-pass stays open.
        let open = FirstOrder::rom(tone_lp(&TONE[7]), 96_000.0);
        assert!(gain_to_db(open.magnitude(96_000.0, 15_000.0)).abs() < 0.3);
    }

    #[test]
    fn one_pole_keeps_time_constant_and_gain() {
        let (input, pole) = one_pole(0.07, 0.93, 88_200.0);
        assert!((pole - 0.93f32.sqrt()).abs() < 1e-6);
        assert!((input / (1.0 - pole) - 1.0).abs() < 1e-4);
    }

    #[test]
    fn increments_follow_the_rate() {
        let at44 = increment(9739.0, 44_100.0);
        assert_eq!(at44, 9739);
        assert!((increment_hz(increment(9739.0, 96_000.0), 96_000.0) - 0.1).abs() < 1e-3);
        assert!((triangle(0x8000_0000) - 1.0).abs() < 1e-6);
        assert!(triangle(0).abs() < 1e-9);
    }

    #[test]
    fn timed_ramp_takes_16_ms() {
        let mut ramp = TimedRamp::new(0.0, 48_000.0);
        ramp.set(1.0);
        let mut n = 0;
        while ramp.tick() < 1.0 {
            n += 1;
        }
        assert!((n as f32 - 768.0).abs() < 3.0, "{n}");
    }

    #[test]
    fn tap_time_glides_small_moves_and_crossfades_jumps() {
        let mut time = TapTime::new(100.0, 0.06, 50.0, 48_000.0);
        time.set(110.0);
        assert_eq!(time.tick(), 0.0);
        assert!((time.current - 100.06).abs() < 1e-4);
        time.set(1000.0);
        let weight = time.tick();
        assert!(weight > 0.99 && time.current == 1000.0);
        let mut n = 1;
        while time.tick() > 0.0 {
            n += 1;
        }
        assert!((n as f32 - 768.0).abs() < 3.0, "{n}");
    }
}
