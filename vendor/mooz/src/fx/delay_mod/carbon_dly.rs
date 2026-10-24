//! CarbonDly (CRBNDLY.ZDL): `Fx_DLY_CarbonDly` with `SUB_KawaOD_DelayDist`.
//!
//! Signal flow of the recovered callback (`carbon_delay/processor.rs`,
//! `front.rs`, `distortion.rs`, `stereo.rs`):
//!
//! 1. Modulation: a Q31 phase (increment c47) folded into a triangle `f`,
//!    shaped as `0.75 · 4f(1-|f|) + 0.25 · 2f` (c50/c49), scaled by the
//!    depth c48 and added to the base delay, which glides to the time
//!    (c11) through the c45/c46 one-pole (τ ≈ 122 ms).
//! 2. Delay: the L/R mean plus the feedback `c12 · wet` (c12 is negative)
//!    through a ~20 Hz DC blocker (c23..27) is written to a 4 s line; the
//!    tap is read with linear interpolation and passes the Tone low-pass
//!    (c28..32) twice in series. That signal is both the feedback source and
//!    the wet input of the distortion. The input (c13..17), pre-ring
//!    (c18..22) and wet (c38..42) biquads are identity in every state the
//!    editors produce and are left out.
//! 3. Distortion (`SUB_KawaOD_DelayDist`): the wet signal times -0.219 is
//!    upsampled 4× with a 5th-order IIR (zero-stuffed, ×4), driven ×4 into a
//!    three-branch soft clipper (c55..62), decimated with the same IIR and
//!    scaled by `c52 · c64 · 0.4217 / c55` (≈ 0.263, c54 = 0: no clean wet).
//! 4. Output: `c6 · distorted + mono dry` on both channels (the callback's
//!    output is mono).
//!
//! Editors decoded from CRBNDLY.ZDL.dis; tables from `.const` through the
//! module's `TBL_TO_VAL` (11 points) and `TBL_TO_VAL_int` (3 points):
//! - DELAY (`Calc_DelayTime`): `value + 19` ms (GetString_offset_19, 19..581
//!   ms), `c11 = (441·ms/10) << 13`. Sync 1..15 replaces it with the Main
//!   tempo note `sync - 1` (context[24], limit 3950 ms).
//! - DELAY also sets the Tone low-pass (`FLTR_edit`, Main biquad designer type
//!   2 at `TBL(0x9f0)` = 1700..2050 Hz, Q `TBL(0xa50)` = 0.92..0.75), the wet
//!   gain `c6 = TBL(MIX, 0x9c0) · TBL(DELAY, 0xa20)` (`mix_edit`) and the
//!   modulation scale `TBL_int(DELAY, 0xaf0)` (`Width_edit`).
//! - REGEN (`fb_edit`): `c12 = -TBL(0xa80)` (0..0.94).
//! - WIDTH: `c48 = TBL_int(WIDTH, 0xb00) · TBL_int(DELAY, 0xaf0)` (Q31), i.e.
//!   up to ±38 samples at 44.1 kHz; SPEED: `c47 = TBL_int(SPEED, 0xae0)`,
//!   0.2..3.3 Hz. MOD Off sets c48 and c47 to 0 (`MOD_edit`).
//!
//! Approximations: a time change glides on the base-delay slew instead of
//! the tap mute (`tapmuteClose`/`tapmuteOpen`) that some original edits use.
//! The oversampling factor is chosen so the inner rate stays near 176.4 kHz
//! (4× at 44.1/48 kHz, 2× at 88.2/96 kHz) and the ROM IIR is kept as is. The
//! feedback write is clamped at ±4: at REGEN 100 the loop gain through the
//! resonant Tone filters exceeds 1 and the original would grow without bound.
//! Delay lengths, LFO rate and depth are rescaled to the host rate.
use super::common::{Biquad, DelayLine, Ramp};
use crate::{Effect, Values, catalog::*, dsp::Coefficients, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_carbon_dly",
    name: "CarbonDly",
    category: Category::Delay,
    zdl: "CRBNDLY.ZDL",
    description: "Dark analog-style delay with modulation and soft clipping",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DELAY",
            max: 562,
            default: 387,
            display: Display::number(19.0, 1.0, 0, " ms"),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "REGEN",
            max: 100,
            default: 47,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 69,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "MOD",
            max: 1,
            default: 1,
            display: Display::OFF_ON,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "WIDTH",
            max: 50,
            default: 31,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "SPEED",
            max: 50,
            default: 28,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 9
        ParamInfo {
            name: "Sync",
            max: 15,
            default: 0,
            display: Display::labels(&SYNC_LABELS),
        },
    ],
};

/// GetString_CarbonDelaySync: Off, then the Main delay notes 0..14.
const SYNC_LABELS: [&str; 16] = [
    "Off", "1/16", "1/8T", "1/16.", "1/8", "1/4T", "1/8.", "1/4", "1/4.", "1/4×2", "1/4×3",
    "1/4×4", "1/4×5", "1/4×6", "1/4×7", "1/4×8",
];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(CarbonDelay::new(sample_rate))
}

const fn w(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// `TBL_TO_VAL(101, MIX, 0x9c0, 11)`.
const WET_BY_MIX: [f32; 11] = [
    0.0,
    w(0x3d8c_d4aa),
    w(0x3dee_e40c),
    w(0x3e2e_147b),
    w(0x3e66_6666),
    w(0x3e99_999a),
    w(0x3ecc_cccd),
    0.5,
    w(0x3f26_6666),
    w(0x3f59_999a),
    w(0x3f8c_cccd),
];
/// `TBL_TO_VAL(563, DELAY, 0xa20, 11)`.
const WET_BY_DELAY: [f32; 11] = [
    w(0x3f7b_bb9f),
    w(0x3f84_50b6),
    w(0x3f86_de4b),
    w(0x3f83_580b),
    w(0x3f83_06d9),
    1.0,
    w(0x3f70_a3d7),
    w(0x3f61_47ae),
    w(0x3f51_eb85),
    w(0x3f42_8f5c),
    w(0x3f32_f1aa),
];
/// `TBL_TO_VAL(563, DELAY, 0x9f0, 11)`: Tone low-pass frequency.
const TONE_HZ: [f32; 11] = [
    1700.0, 1700.0, 1700.0, 1700.0, 1700.0, 1700.0, 1750.0, 1800.0, 1850.0, 1950.0, 2050.0,
];
/// `TBL_TO_VAL(563, DELAY, 0xa50, 11)`: Tone low-pass Q.
const TONE_Q: [f32; 11] = [
    w(0x3f6b_851f),
    w(0x3f6b_851f),
    w(0x3f6b_851f),
    w(0x3f6b_851f),
    w(0x3f6b_851f),
    w(0x3f6b_851f),
    0.875,
    w(0x3f59_999a),
    w(0x3f53_3333),
    w(0x3f4c_cccd),
    0.75,
];
/// `TBL_TO_VAL(101, REGEN, 0xa80, 11)`; c12 is its negation.
const REGEN: [f32; 11] = [
    0.0,
    w(0x3d99_999a),
    w(0x3e19_999a),
    w(0x3e38_51ec),
    w(0x3e61_47ae),
    w(0x3e8a_3d71),
    w(0x3ea8_f5c3),
    w(0x3ecc_cccd),
    w(0x3ef0_a3d7),
    w(0x3f19_999a),
    w(0x3f70_a3d7),
];
/// `TBL_TO_VAL_int(51, SPEED, 0xae0, 3)`: Q31 phase increment per 44.1 kHz sample.
const SPEED: [f32; 3] = [19_478.0, 107_130.0, 324_313.0];
/// `TBL_TO_VAL_int(51, WIDTH, 0xb00, 3)`: Q31 depth.
const WIDTH: [f32; 3] = [0.0, 425_487.0, 625_000.0];
/// `TBL_TO_VAL_int(563, DELAY, 0xaf0, 3)`: Q31 depth scale.
const WIDTH_BY_DELAY: [f32; 3] = [
    1_343_747_077.0 / 2_147_483_648.0,
    1.0,
    1_707_940_211.0 / 2_147_483_648.0,
];

/// ECHO-style `TBL_TO_VAL(count, value, table, 11)`.
fn table(points: &[f32; 11], value: f32, count: f32) -> f32 {
    let position = value.clamp(0.0, count - 1.0) * 10.0 / (count - 1.0);
    let index = (position as usize).min(9);
    points[index] + (points[index + 1] - points[index]) * (position - index as f32)
}

/// `TBL_TO_VAL_int(count, value, table, 3)` (continuous for fractional values).
fn table3(points: &[f32; 3], value: f32, count: f32) -> f32 {
    let position = value.clamp(0.0, count - 1.0) * 2.0 / (count - 1.0);
    let index = (position as usize).min(1);
    points[index] + (points[index + 1] - points[index]) * (position - index as f32)
}

/// c23..c27: DC blocker in the feedback path (callback sign convention).
const FEEDBACK_DC: Coefficients = Coefficients {
    b0: w(0x3f7f_a2c2),
    b1: w(0xbf7f_a2c2),
    b2: 0.0,
    a1: -w(0x3f7f_4585),
    a2: 0.0,
};
/// c45 = 1 - c46 (Q31): the base-delay slew per 44.1 kHz sample.
const SLEW_44K: f32 = 398_662.0 / 2_147_483_648.0;
/// Calc_DelayTime: `DELAY + 19` ms, synced notes limited to 3950 ms.
const DELAY_OFFSET_MS: f32 = 19.0;
const SYNC_LIMIT_MS: f32 = 3950.0;
/// The original 176400-word line.
const LINE_SECONDS: f32 = 4.0;
/// Clamp of the feedback write (not in the original, see the module doc).
const LOOP_LIMIT: f32 = 4.0;

// SUB_KawaOD_DelayDist.
const DRIVE_IN: f32 = w(0xbe60_4428); // -0.219
const OVERSAMPLE_B: [f32; 6] = [
    w(0x3a68_67aa),
    w(0x3b76_2dbb),
    w(0x3be3_88c6),
    w(0x3be3_88c6),
    w(0x3b76_2dbb),
    w(0x3a68_67aa),
];
/// Feedback taps of the 5th-order IIR (`y = Σb·u + Σa·y`).
const OVERSAMPLE_A: [f32; 5] = [
    w(0x4067_2e31),
    w(0xc0b7_48a3),
    w(0x409c_739a),
    w(0xc00e_da14),
    w(0x3ede_d247),
];
/// c55..c62 of the three-branch soft clipper.
const CLIP_SCALE: f32 = 3.0;
const CLIP_SECOND: f32 = w(0x3f3a_be11);
const CLIP_SLOPE: f32 = w(0x3e4c_cccd);
const CLIP_OFFSET: f32 = w(0x3ecc_cccd);
const CLIP_FIRST: f32 = w(0x3fa9_6d96);
/// `c52 · c53 · c64 · 0.4217 / c55`.
const DISTORTION_OUTPUT: f32 = w(0x400c_cccd) * w(0x3f59_999a) * w(0x3ed7_e910) / 3.0;
const INNER_RATE: f32 = 4.0 * zoom::ORIGINAL_RATE;

/// The original's soft clipper (`compress`): odd, ±1 at |v| ≥ 1.
///
/// Written on `|scaled|`: for a negative input `-CLIP_FIRST · scaled` is
/// `CLIP_FIRST · |scaled|` and the third branch is the exact negation of its
/// positive form (negation and clamping are symmetric), so only the sign of
/// `first · third` depends on the branch (`-0.0` takes the positive branch
/// with `first = -0.0`, also a negative zero product). Both products are
/// non-negative, so only the upper clamp can act.
#[inline]
fn compress(value: f32) -> f32 {
    let scaled = CLIP_SCALE * value;
    let second = (CLIP_SECOND * scaled).clamp(-1.0, 1.0);
    let magnitude = scaled.abs();
    let upper = |x: f32| if x > 1.0 { 1.0 } else { x };
    let first = upper(CLIP_FIRST * magnitude);
    let third = upper(CLIP_OFFSET + CLIP_SLOPE * magnitude);
    second * (first - 1.0).abs() + (first * third).copysign(value)
}

/// Transposed direct form II of the oversampling IIR.
#[derive(Clone, Copy, Default)]
struct Oversampler {
    state: [f32; 5],
}

impl Oversampler {
    #[inline]
    fn process(&mut self, u: f32) -> f32 {
        let (b, a, s) = (&OVERSAMPLE_B, &OVERSAMPLE_A, &mut self.state);
        let y = b[0] * u + s[0];
        s[0] = b[1] * u + a[0] * y + s[1];
        s[1] = b[2] * u + a[1] * y + s[2];
        s[2] = b[3] * u + a[2] * y + s[3];
        s[3] = b[4] * u + a[3] * y + s[4];
        s[4] = b[5] * u + a[4] * y;
        y
    }

    /// Zero input: the zero-stuffed upsampling phases.
    #[inline]
    fn ring(&mut self) -> f32 {
        let (a, s) = (&OVERSAMPLE_A, &mut self.state);
        let y = s[0];
        s[0] = a[0] * y + s[1];
        s[1] = a[1] * y + s[2];
        s[2] = a[2] * y + s[3];
        s[3] = a[3] * y + s[4];
        s[4] = a[4] * y;
        y
    }

    fn flush(&mut self) {
        self.state = self.state.map(super::common::flush);
    }
}

struct CarbonDelay {
    sample_rate: f32,
    fresh: bool,
    // Modulation.
    phase: u32,
    increment: u32,
    depth: Ramp,
    base: f32,
    target: f32,
    slew: f32,
    // Delay.
    line: DelayLine,
    feedback: Ramp,
    feedback_dc: Biquad,
    tone: [Biquad; 2],
    tone_value: f32,
    wet: f32,
    // Distortion.
    factor: usize,
    upsampler: Oversampler,
    downsampler: Oversampler,
    // Output.
    mix: Ramp,
}

impl CarbonDelay {
    fn new(sample_rate: f32) -> Self {
        let factor = (INNER_RATE / sample_rate).round().max(1.0) as usize;
        let ramp = |value| Ramp::new(value, zoom::DEFAULT_STEP, sample_rate);
        let mut delay = Self {
            sample_rate,
            fresh: true,
            phase: 0,
            increment: 0,
            // Depth in samples, set almost immediately like context[10].
            depth: Ramp::new(0.0, 0.5, sample_rate),
            base: 0.0,
            target: 0.0,
            slew: 1.0 - (1.0 - SLEW_44K).powf(zoom::ORIGINAL_RATE / sample_rate),
            line: DelayLine::with_seconds(LINE_SECONDS, sample_rate),
            feedback: ramp(0.0),
            feedback_dc: Biquad::new(FEEDBACK_DC.resample(zoom::ORIGINAL_RATE, sample_rate)),
            tone: [Biquad::default(); 2],
            tone_value: -1.0,
            wet: 0.0,
            factor,
            upsampler: Oversampler::default(),
            downsampler: Oversampler::default(),
            mix: ramp(0.0),
        };
        delay.design_tone(f32::from(INFO.params[0].default));
        delay
    }

    fn design_tone(&mut self, delay: f32) {
        self.tone_value = delay;
        let coefficients = Coefficients::lowpass(
            self.sample_rate,
            table(&TONE_HZ, delay, 563.0),
            table(&TONE_Q, delay, 563.0),
        );
        for biquad in &mut self.tone {
            biquad.coefficients = coefficients;
        }
    }

    /// `SUB_KawaOD_DelayDist` for one base-rate sample.
    #[inline]
    fn distort(&mut self, wet: f32) -> f32 {
        let factor = self.factor;
        // Zero-stuffing gain × the original ×4 drive.
        let drive = 4.0 * factor as f32;
        let x = DRIVE_IN * wet;
        let first = compress(drive * self.upsampler.process(x));
        let output = self.downsampler.process(first);
        for _ in 1..factor {
            let shaped = compress(drive * self.upsampler.ring());
            self.downsampler.process(shaped);
        }
        self.upsampler.flush();
        self.downsampler.flush();
        DISTORTION_OUTPUT * output
    }

    fn delay_ms(delay: f32, sync: f32, bpm: f32) -> f32 {
        if sync.round() >= 1.0 {
            let index = sync.round() as usize - 1;
            zoom::delay_note_ms(index, bpm, SYNC_LIMIT_MS)
        } else {
            delay.max(0.0) + DELAY_OFFSET_MS
        }
    }
}

impl Effect for CarbonDelay {
    fn set(&mut self, values: &Values, bpm: f32) {
        let delay = values[0].clamp(0.0, 562.0);
        let modulated = values[3].round() >= 1.0;
        self.target = Self::delay_ms(delay, values[6], bpm) * 0.001 * self.sample_rate;
        self.feedback.set(-table(&REGEN, values[1], 101.0));
        self.mix
            .set(table(&WET_BY_MIX, values[2], 101.0) * table(&WET_BY_DELAY, delay, 563.0));
        if delay != self.tone_value {
            self.design_tone(delay);
        }
        let rescale = zoom::ORIGINAL_RATE / self.sample_rate;
        if modulated {
            // c48 / 16384 samples at 44.1 kHz.
            let depth = table3(&WIDTH, values[4], 51.0) * table3(&WIDTH_BY_DELAY, delay, 563.0);
            self.depth.set(depth / 16_384.0 / rescale);
            self.increment = (table3(&SPEED, values[5], 51.0) * rescale).round() as u32;
        } else {
            self.depth.set(0.0);
            self.increment = 0;
        }
        if self.fresh {
            self.fresh = false;
            self.base = self.target;
            self.depth.jump(self.depth.target);
            self.feedback.jump(self.feedback.target);
            self.mix.jump(self.mix.target);
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mono = 0.5 * (left + right);

        // Folded Q31 phase: f in [-0.5, 0.5].
        self.phase = self.phase.wrapping_add(self.increment);
        let folded = (self.phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0) - 0.5;
        let curve = 4.0 * folded * (1.0 - folded.abs());
        let shape = 0.75 * curve + 0.5 * folded;
        self.base += (self.target - self.base) * self.slew;
        let distance = self.base + shape * self.depth.tick();

        let feedback = self.feedback_dc.process(self.feedback.tick() * self.wet);
        self.line
            .write((feedback + mono).clamp(-LOOP_LIMIT, LOOP_LIMIT));
        let tap = self.line.read_linear(distance);
        let toned = self.tone[0].process(tap);
        let wet = self.tone[1].process(toned);
        self.wet = wet;

        let out = self.mix.tick() * self.distort(wet) + mono;
        [out, out]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.base = self.target;
        self.wet = 0.0;
        self.feedback_dc.clear();
        self.tone.iter_mut().for_each(Biquad::clear);
        self.upsampler = Oversampler::default();
        self.downsampler = Oversampler::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(delay: f32, regen: f32, mix: f32, modulation: f32, sync: f32) -> Values {
        let mut v = [0.0; crate::MAX_PARAMS];
        v[..7].copy_from_slice(&[delay, regen, mix, modulation, 31.0, 28.0, sync]);
        v
    }

    #[test]
    fn tables_follow_the_editors() {
        // Defaults: MIX 69, DELAY 387.
        let wet = table(&WET_BY_MIX, 69.0, 101.0) * table(&WET_BY_DELAY, 387.0, 563.0);
        assert!((wet - 0.49 * 0.8868).abs() < 1e-3, "{wet}");
        assert!((table(&REGEN, 100.0, 101.0) - 0.94).abs() < 1e-6);
        assert_eq!(table(&TONE_HZ, 562.0, 563.0), 2050.0);
        // TBL_TO_VAL_int: WIDTH 31 → 425487 + 6 · (199513 / 25) (integer division).
        assert!((table3(&WIDTH, 31.0, 51.0) - 473_367.0).abs() < 5.0);
        assert!((table3(&WIDTH_BY_DELAY, 281.0, 563.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn delay_time_and_sync_follow_calc_delay_time() {
        assert_eq!(CarbonDelay::delay_ms(0.0, 0.0, 120.0), 19.0);
        assert_eq!(CarbonDelay::delay_ms(562.0, 0.0, 120.0), 581.0);
        assert!((CarbonDelay::delay_ms(0.0, 7.0, 120.0) - 500.0).abs() < 1e-3);
        // 1/4×8 at 120 BPM is 4000 ms: halved under the 3950 ms limit.
        assert!((CarbonDelay::delay_ms(0.0, 15.0, 120.0) - 2000.0).abs() < 1e-3);
        assert_eq!(SYNC_LABELS.len(), 16);
    }

    #[test]
    fn soft_clipper_is_odd_and_bounded() {
        for i in -400..=400 {
            let v = i as f32 / 100.0;
            let y = compress(v);
            assert!((y + compress(-v)).abs() < 1e-6, "{v}");
            assert!(y.abs() <= 1.0 + 1e-6, "{v}: {y}");
        }
        // Small-signal slope: 3 · (c56 + c58 · c59) ≈ 3.77.
        let slope = compress(1e-4) / 1e-4;
        assert!((slope - 3.0 * (CLIP_SECOND + CLIP_OFFSET * CLIP_FIRST)).abs() < 0.01);
        assert!((compress(1.0) - 1.0).abs() < 1e-6);
    }

    fn first_echo(sample_rate: f32, delay: f32) -> usize {
        let mut effect = CarbonDelay::new(sample_rate);
        effect.set(&values(delay, 0.0, 100.0, 0.0, 0.0), 120.0);
        let mut best = (0, 0.0f32);
        for i in 0..sample_rate as usize {
            let x = if i == 0 { 0.05 } else { 0.0 };
            let y = effect.process([x, x])[0].abs();
            if i > 10 && y > best.1 {
                best = (i, y);
            }
        }
        best.0
    }

    #[test]
    fn echo_arrives_after_delay_plus_19_ms() {
        for (rate, delay) in [(44_100.0, 81.0), (48_000.0, 381.0), (96_000.0, 0.0)] {
            let expected = (delay + 19.0) * 0.001 * rate;
            let at = first_echo(rate, delay) as f32;
            // Two Tone low-passes and the oversampling IIR add < 1 ms.
            assert!(
                at >= expected && at - expected < 0.001 * rate,
                "{rate}: {at} {expected}"
            );
        }
    }

    #[test]
    fn lfo_rate_and_depth_scale_with_the_host_rate() {
        for rate in [44_100.0, 96_000.0] {
            let mut effect = CarbonDelay::new(rate);
            let mut v = values(387.0, 47.0, 69.0, 1.0, 0.0);
            v[5] = 50.0;
            effect.set(&v, 120.0);
            let hz = effect.increment as f32 * rate / 4_294_967_296.0;
            assert!(
                (hz - 324_313.0 * 44_100.0 / 4_294_967_296.0).abs() < 0.01,
                "{hz}"
            );
            let ms = effect.depth.target / rate * 1000.0;
            let expected = 473_367.0 * table3(&WIDTH_BY_DELAY, 387.0, 563.0) / 16_384.0 / 44.1;
            assert!((ms - expected).abs() < 1e-3, "{ms} {expected}");
        }
    }

    #[test]
    fn mix_zero_passes_the_mono_dry_signal() {
        let mut effect = CarbonDelay::new(48_000.0);
        effect.set(&values(387.0, 47.0, 0.0, 1.0, 0.0), 120.0);
        for i in 0..4000 {
            let x = (i as f32 * 0.01).sin() * 0.4;
            let [l, r] = effect.process([x, 0.5 * x]);
            assert!((l - 0.75 * x).abs() < 1e-6 && l == r);
        }
    }

    #[test]
    fn maximum_regeneration_stays_bounded() {
        let mut effect = CarbonDelay::new(48_000.0);
        effect.set(&values(0.0, 100.0, 100.0, 1.0, 0.0), 120.0);
        let mut noise = crate::dsp::Noise::new(9);
        let mut peak = 0.0f32;
        for i in 0..48_000 * 10 {
            let x = if i < 4800 { noise.bipolar() } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            assert!(l.is_finite() && r.is_finite());
            peak = peak.max(l.abs());
        }
        // The distortion bounds the wet output to 0.263 · c6.
        assert!(peak < 1.5, "{peak}");
    }

    #[test]
    fn oversampler_is_a_unity_gain_lowpass() {
        let mut filter = Oversampler::default();
        let mut y = 0.0;
        for _ in 0..20_000 {
            y = filter.process(1.0);
        }
        assert!((y - 1.0).abs() < 0.01, "{y}");
    }
}
