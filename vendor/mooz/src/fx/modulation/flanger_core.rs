//! The audio kernel FLANGER.ZDL and B_FLNGR.ZDL share (`Fx_MOD_Flanger`,
//! 7800..7d40) with the editors of both modules.
//!
//! Callback, per sample: the stereo mean goes through a 300 Hz first-order
//! high-pass (c14..16), adds `c13 × last tap` (Reso) and passes the feedback
//! filter c17..19 (identity in Flanger, the LoCut high-pass in BaFlanger) into
//! the delay line. The LFO is the callbacks' parabolic sine of a triangle
//! phase (c21 increment, c22 = 0, c23 = 1), mapped to 0..1 and smoothed by a
//! one-pole of 2^-14 per sample (c25/c26), which is why fast rates sweep less.
//! The delay is `5 + PreD + Depth × lfo` samples (c8, c30 smoothed by 1/512
//! per sample through c27/c28, c29) read with linear interpolation; the tap is
//! scaled by `1 − 0.15 × lfo` (c31/c32 → c33), multiplied by 3 (c34) and by
//! the signed Mix (c6) and added to each dry channel; Level (c2) scales both.
//!
//! Editors (decoded from the disassembly):
//! - Depth: `context[22]` max 0x27b59a8a, square law → c29 = 635.36 samples ×
//!   (d/100)², integer ramp of 0x3deb7 per sample.
//! - Rate: 0..40 → increment 4870..63110 linear, 41..50 → 69079..1460873
//!   (0.71..15 Hz), above 50 the tempo notes (`context[23]`); stored in c21.
//! - Reso: `Fx_MOD_Flanger_reso_tbl` → c13 (−0.92..+0.92), ramp 0x3af7c5ec.
//! - PreD: `context[22]` 1..653 → `c30 = n << 20` (n samples, 0.02..14.8 ms).
//! - Mix: 0..1 → c6, negated while Reso < 10, ramp 0x3a77c5f5.
//! - Level: 0..1.5 → c2, ramp 0x3a77c5f5.
//! - LoCut (BaFlanger): `ModChoLoCutTbl` 20 (Off), 60..800 Hz bilinear HPF.
use super::common::{
    FirstOrder, Lfo, Ramp, centered_triangle, guard, parabola, rate_hz, stepped, table,
};
use crate::dsp::{Coefficients, DelayLine, flush, scale_samples};
use crate::zoom::{DEFAULT_STEP, ORIGINAL_RATE};
use crate::{Effect, Values};

/// `Fx_MOD_Flanger_reso_tbl`, second word of each entry (c13).
pub const RESO: [f32; 21] = [
    -0.92,
    -0.801_111_1,
    -0.732_222_2,
    -0.663_333_4,
    -0.594_444_45,
    -0.525_555_6,
    -0.456_666_7,
    -0.387_777_8,
    -0.318_888_9,
    -0.25,
    0.0,
    0.25,
    0.318_888_9,
    0.387_777_8,
    0.456_666_7,
    0.525_555_6,
    0.594_444_45,
    0.663_333_4,
    0.732_222_2,
    0.801_111_1,
    0.92,
];

/// BaFlanger LoCut corners (`ModChoLoCutTbl`, bilinear first-order high-passes).
pub const LOCUT_HZ: [f32; 11] = [
    20.0, 60.0, 120.0, 180.0, 200.0, 280.0, 340.0, 400.0, 500.0, 630.0, 800.0,
];

/// Delay sweep at Depth 100 (0x27b59a8a >> 20) and the ramp of c29.
const DEPTH_SAMPLES: f32 = 635.361_3;
const DEPTH_STEP: f32 = 253_623.0 / 1_048_576.0;
/// c8: fixed part of the read distance.
const BASE_SAMPLES: f32 = 5.0;
/// `Fx_MOD_Flanger_reso_edit` step 0x3af7c5ec.
const RESO_STEP: f32 = 1.888_39e-3;

/// Rate curve of the Flanger family as an original phase increment.
pub fn rate_increment(value: u32, fast_maximum: f32) -> f32 {
    if value <= 40 {
        super::common::curve(63_110.0, 4_870.0, 41, value, 0)
    } else {
        super::common::curve(fast_maximum, 69_079.0, 10, value - 41, 0)
    }
}

pub struct FlangerCore {
    sample_rate: f32,
    ratio: f32,
    line: DelayLine,
    input: FirstOrder,
    locut: Option<FirstOrder>,
    lfo: Lfo,
    sweep: f32,
    sweep_coefficient: f32,
    predelay: f32,
    predelay_target: f32,
    predelay_coefficient: f32,
    depth: Ramp,
    feedback: Ramp,
    mix: Ramp,
    level: Ramp,
    last_tap: f32,
    first: bool,
    cache: [f32; 7],
    bpm: f32,
}

impl FlangerCore {
    pub fn new(sample_rate: f32, locut: bool) -> Self {
        let ratio = sample_rate / ORIGINAL_RATE;
        let longest = BASE_SAMPLES + 653.0 + DEPTH_SAMPLES + 4.0;
        Self {
            sample_rate,
            ratio,
            line: DelayLine::new(scale_samples(longest, sample_rate).ceil() as usize + 4),
            input: FirstOrder::new(Coefficients::highpass1(sample_rate, 300.0)),
            locut: locut.then(|| FirstOrder::new(Coefficients::highpass1(sample_rate, 60.0))),
            lfo: Lfo::default(),
            sweep: 0.0,
            sweep_coefficient: super::common::one_pole_at(1.0 / 16_384.0, sample_rate),
            predelay: 0.0,
            predelay_target: 0.0,
            predelay_coefficient: super::common::one_pole_at(1.0 / 512.0, sample_rate),
            depth: Ramp::with_step(0.0, DEPTH_STEP * ratio, sample_rate),
            feedback: Ramp::with_step(0.0, RESO_STEP, sample_rate),
            mix: Ramp::with_step(0.0, DEFAULT_STEP, sample_rate),
            level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
            last_tap: 0.0,
            first: true,
            cache: [f32::NAN; 7],
            bpm: 0.0,
        }
    }

    /// `values`: Depth, Rate, Reso, PreD, Mix, Level[, LoCut].
    pub fn set(&mut self, values: &Values, bpm: f32, fast_maximum: f32) {
        let [depth, rate, reso, predelay, mix, level, locut] =
            [0, 1, 2, 3, 4, 5, 6].map(|i| values[i]);
        if rate != self.cache[1] || bpm != self.bpm {
            let hz = rate_hz(rate, 50, bpm, |v| {
                super::common::increment_hz(stepped(v, |n| rate_increment(n, fast_maximum)))
            });
            self.lfo.set_hz(hz, self.sample_rate);
            self.bpm = bpm;
        }
        let d = depth.clamp(0.0, 100.0) * 0.01;
        self.depth.set(DEPTH_SAMPLES * d * d * self.ratio);
        self.feedback.set(table(&RESO, reso));
        let negative = reso.round() < 10.0;
        let wet = mix.clamp(0.0, 100.0) * 0.01;
        self.mix.set(if negative { -wet } else { wet });
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        // c30: integer curve 1..653 over 51 steps, then whole samples.
        let samples = stepped(predelay.clamp(0.0, 50.0), |n| {
            super::common::curve(653.0, 1.0, 51, n, 0).floor()
        });
        self.predelay_target = samples * self.ratio;
        if let Some(filter) = self.locut.as_mut()
            && locut != self.cache[6]
        {
            let hz = table(&LOCUT_HZ, locut.round());
            filter.set(Coefficients::highpass1(self.sample_rate, hz));
        }
        if self.first {
            self.first = false;
            self.depth.jump(self.depth.target);
            self.feedback.jump(self.feedback.target);
            self.mix.jump(self.mix.target);
            self.level.jump(self.level.target);
            self.predelay = self.predelay_target;
        }
        self.cache = [depth, rate, reso, predelay, mix, level, locut];
    }

    #[inline]
    pub fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mono = 0.5 * (left + right);
        let input = self.input.process(mono);
        let feedback = self.feedback.tick();
        let mut written = input + feedback * self.last_tap;
        if let Some(filter) = self.locut.as_mut() {
            written = filter.process(written);
        }
        self.line.write(flush(written));
        let wave = parabola(centered_triangle(self.lfo.tick()));
        self.sweep += self.sweep_coefficient * (0.5 + 0.5 * wave - self.sweep);
        self.predelay += self.predelay_coefficient * (self.predelay_target - self.predelay);
        let delay = BASE_SAMPLES * self.ratio + self.predelay + self.depth.tick() * self.sweep;
        let tap = self.line.read_linear(delay) * (1.0 - 0.15 * self.sweep);
        self.last_tap = tap;
        let wet = 3.0 * tap * self.mix.tick();
        let level = self.level.tick();
        [guard(level * (left + wet)), guard(level * (right + wet))]
    }

    pub fn clear(&mut self) {
        self.line.clear();
        self.input.clear();
        if let Some(filter) = self.locut.as_mut() {
            filter.clear();
        }
        self.last_tap = 0.0;
        self.sweep = 0.0;
        self.lfo.phase = 0;
        self.predelay = self.predelay_target;
    }
}

/// An `Effect` wrapper around the shared kernel.
pub struct Flanger {
    pub core: FlangerCore,
    pub fast_maximum: f32,
}

impl Effect for Flanger {
    fn set(&mut self, values: &Values, bpm: f32) {
        self.core.set(values, bpm, self.fast_maximum);
    }
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }
    fn clear(&mut self) {
        self.core.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::modulation::common::increment_hz;

    #[test]
    fn rate_curve_matches_the_editor() {
        assert!((increment_hz(rate_increment(0, 1_460_873.0)) - 0.0501).abs() < 1e-3);
        assert!((increment_hz(rate_increment(40, 1_460_873.0)) - 0.648).abs() < 1e-3);
        assert!((increment_hz(rate_increment(50, 1_460_873.0)) - 15.0).abs() < 0.01);
        assert!((increment_hz(rate_increment(50, 973_915.0)) - 10.0).abs() < 0.01);
    }

    #[test]
    fn delay_spans_the_original_range() {
        let mut core = FlangerCore::new(44_100.0, false);
        let mut values = [0.0; crate::MAX_PARAMS];
        values[2] = 10.0;
        values[3] = 50.0;
        values[4] = 100.0;
        values[5] = 100.0;
        core.set(&values, 120.0, 1_460_873.0);
        assert_eq!(core.predelay_target, 653.0);
        // An impulse comes back after 5 + 653 samples (Depth 0), times 3.
        let mut out = Vec::new();
        out.push(core.process([1.0, 1.0])[0]);
        for _ in 0..700 {
            out.push(core.process([0.0, 0.0])[0]);
        }
        let peak = out[1..]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .unwrap();
        assert_eq!(peak.0 + 1, 658);
        // 300 Hz HPF of a unit impulse: first sample b0 = 0.979.
        assert!((peak.1 - 3.0 * 0.979).abs() < 0.01, "{}", peak.1);
    }
}
