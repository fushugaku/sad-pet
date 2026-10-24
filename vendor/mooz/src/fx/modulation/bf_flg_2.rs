//! BF FLG 2 (BF_FLG_2.ZDL), a Brain Freeze style flanger, from the recovered
//! `Fx_MOD_BF_FLG_2` callback (`bf_flanger_two/`) and its decoded editors.
//!
//! Callback, per sample: the stereo mean passes a 400 Hz first-order
//! high-pass and a 2.4 kHz low-pass biquad with ×2.24 gain (c14..23), adds
//! `Reso × last tap` (c13) and goes through the `BF_2_CLIPPER_KAWAOD` shaper
//! (c24..31: unity for small signals, saturating smoothly), then a 10 kHz
//! low-pass (c32..36) and a 53 Hz high-pass (c37..41) into the delay line.
//! The LFO is `0.2 m + 0.8 m⁴` of the triangle `m = |phase|` (c43..45),
//! smoothed by a rate-dependent one-pole (c48/c49); the read point is
//! `5 + pre-delay + sweep × lfo` samples (c8, c53 smoothed by 1/512 via
//! c50/c51, c52), linearly interpolated. The tap passes a 5 kHz low-pass at
//! ×0.5 and a 200 Hz high-pass (c57..66), adds `1.07 × mean` (c7) and three
//! more biquads (c67..81) shape the mono output on both channels.
//!
//! Editors: RATE interpolates `BF2_Sawf_tbl` per decade (0.07..10.2 Hz) and
//! `BF2_LPF_S_a0_tbl` for the LFO smoother; RES interpolates `BF2_Reso_tbl`
//! (0..0.99). MNL and DEPTH (`manual_depth_set`): pre-delay ms
//! `a + PreDelay_Courve(DEPTH) (b − a)` and maximum ms
//! `a' + MaxDelay_Courve(DEPTH) (b' − a')` with (a, b) interpolated from
//! `BF_2_PreDelay_ms_tbl` / `BF_2_MaxDelay_ms_tbl` by MNL; c53 = pre-delay
//! samples, c52 = (maximum − pre-delay) / 46.4172 ms (ramped). Level and Mix
//! are fixed at 1. All editors decoded; ROM filters are re-derived for the
//! host rate.
use super::common::{
    FirstOrder, Lfo, Ramp, guard, increment_hz, one_pole_at, rom_biquad, rom_first_order,
};
use crate::dsp::{Biquad, DelayLine, flush, scale_samples};
use crate::zoom::ORIGINAL_RATE;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_bf_flg_2",
    name: "BF FLG 2",
    category: Category::Modulation,
    zdl: "BF_FLG_2.ZDL",
    description: "Brain Freeze style flanger with manual sweep and drive",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "MNL",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "DEPTH",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "RATE",
            max: 100,
            default: 30,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "RES",
            max: 100,
            default: 50,
            display: Display::INT,
        },
    ],
};

/// Per-decade tables of the editors (index = value / 10, interpolated).
const SAWF: [f32; 12] = [
    6_762.9, 6_810.6, 8_619.2, 18_036.9, 28_311.7, 36_073.8, 48_208.8, 89_347.0, 159_624.7,
    559_709.2, 993_393.8, 993_393.8,
];
const LPF_A0: [f32; 12] = [
    4_073_665.0,
    4_072_473.0,
    4_030_026.0,
    3_859_367.0,
    3_719_580.0,
    3_630_374.0,
    3_508_378.0,
    3_181_703.0,
    2_767_357.0,
    1_331_216.0,
    306_670.0,
    306_670.0,
];
const RESO: [f32; 12] = [
    0.0, 0.156_985, 0.289_04, 0.406_367, 0.512_287, 0.608_649, 0.696_666, 0.777_21, 0.850_946,
    0.968_399, 0.99, 0.99,
];
const MAX_CURVE: [f32; 12] = [
    0.0, 0.0, 0.083, 0.167, 0.25, 0.333, 0.417, 0.5, 0.75, 1.0, 1.0, 1.0,
];
const PRE_CURVE: [f32; 12] = [
    0.0, 0.088, 0.432, 0.616, 0.736, 0.84, 0.888, 0.936, 0.968, 1.0, 1.0, 1.0,
];
/// `BF_2_MaxDelay_ms_tbl` / `BF_2_PreDelay_ms_tbl`: (at DEPTH curve 0, at 1).
const MAX_MS: [[f32; 2]; 12] = [
    [14.0, 12.6],
    [11.5, 12.6],
    [8.0, 12.6],
    [5.6, 12.6],
    [4.2, 12.6],
    [3.1, 12.6],
    [2.44, 12.6],
    [1.92, 12.6],
    [1.52, 12.6],
    [1.1, 12.6],
    [1.0, 12.6],
    [0.9, 12.6],
];
const PRE_MS: [[f32; 2]; 12] = [
    [13.5, 0.9],
    [11.5, 0.9],
    [7.76, 0.9],
    [5.28, 0.9],
    [4.08, 0.9],
    [3.08, 0.9],
    [2.36, 0.9],
    [1.88, 0.9],
    [1.48, 0.9],
    [1.1, 0.9],
    [1.0, 0.9],
    [0.9, 0.9],
];

/// ROM filters (callback form `y = c0 x + c1 x1 + c2 x2 + c3 y1 + c4 y2`).
const FRONT_HIGH_PASS: [f32; 3] = [0.972_287, -0.972_287, 0.944_574];
const FRONT_LOW_PASS: [f32; 5] = [0.122_039, 0.244_079, 0.122_039, 1.161_615, -0.379_667];
const LOOP_LOW_PASS: [f32; 3] = [0.463_426, 0.463_426, 0.073_149];
const LOOP_HIGH_PASS: [f32; 5] = [0.994_681, -1.989_362, 0.994_681, 1.989_353, -0.989_371];
const TAP_LOW_PASS: [f32; 3] = [0.135_906, 0.135_906, 0.457_663];
const TAP_HIGH_PASS: [f32; 3] = [0.985_952, -0.985_952, 0.971_903];
const OUTPUT_EQ: [[f32; 5]; 3] = [
    [1.096_88, -1.842_568, 0.902_703, 1.642_19, -0.782_13],
    [0.969_489, -1.822_742, 0.865_158, 1.822_742, -0.834_647],
    [1.001_238, -1.989_673, 0.988_637, 1.989_673, -0.989_875],
];

/// 0x4239ab3f: 2048 samples at 44.1 kHz in ms (the Q31 sweep unit).
#[cfg(test)]
const SWEEP_UNIT_MS: f32 = 46.417_2;
const BASE_SAMPLES: f32 = 5.0;

/// Interpolate a per-decade table (`tbl[v/10] + (tbl[v/10+1] − tbl[v/10]) × (v%10)/10`).
fn decade(table: &[f32; 12], value: f32) -> f32 {
    let value = value.clamp(0.0, 100.0);
    let index = ((value / 10.0) as usize).min(10);
    let fraction = (value - 10.0 * index as f32) / 10.0;
    table[index] + (table[index + 1] - table[index]) * fraction
}

fn decade_pair(table: &[[f32; 2]; 12], value: f32) -> [f32; 2] {
    let value = value.clamp(0.0, 100.0);
    let index = ((value / 10.0) as usize).min(10);
    let fraction = (value - 10.0 * index as f32) / 10.0;
    [0, 1].map(|i| table[index][i] + (table[index + 1][i] - table[index][i]) * fraction)
}

/// Pre-delay and sweep in milliseconds for MNL and DEPTH.
pub fn delays_ms(manual: f32, depth: f32) -> (f32, f32) {
    let [pre_a, pre_b] = decade_pair(&PRE_MS, manual);
    let [max_a, max_b] = decade_pair(&MAX_MS, manual);
    let pre = pre_a + decade(&PRE_CURVE, depth) * (pre_b - pre_a);
    let maximum = max_a + decade(&MAX_CURVE, depth) * (max_b - max_a);
    (pre, (maximum - pre).max(0.0))
}

/// `BF_2_CLIPPER_KAWAOD` as the callback evaluates it.
#[inline]
fn shaper(value: f32) -> f32 {
    let (slope, offset, gain) = if value < 0.0 {
        (0.09, -0.51, -0.892_157)
    } else {
        (0.09, 0.51, 0.892_157)
    };
    let first = (value * gain).clamp(-1.0, 1.0);
    let second = (0.545 * value).clamp(-1.0, 1.0);
    let third = (offset + slope * value).clamp(-1.0, 1.0);
    (first - 1.0).abs() * second + first * third
}

struct BfFlanger {
    sample_rate: f32,
    ratio: f32,
    line: DelayLine,
    front_high_pass: FirstOrder,
    front_low_pass: Biquad,
    loop_low_pass: FirstOrder,
    loop_high_pass: Biquad,
    tap_low_pass: FirstOrder,
    tap_high_pass: FirstOrder,
    output_eq: [Biquad; 3],
    lfo: Lfo,
    slew: f32,
    slew_coefficient: f32,
    predelay: f32,
    predelay_target: f32,
    predelay_coefficient: f32,
    sweep: Ramp,
    reso: Ramp,
    last_tap: f32,
    first: bool,
    rate: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let ratio = sample_rate / ORIGINAL_RATE;
    let longest_ms = 14.5;
    let longest = BASE_SAMPLES + longest_ms * 44.1 + 4.0;
    Box::new(BfFlanger {
        sample_rate,
        ratio,
        line: DelayLine::new(scale_samples(longest, sample_rate).ceil() as usize + 4),
        front_high_pass: FirstOrder::new(rom_first_order(FRONT_HIGH_PASS, sample_rate)),
        front_low_pass: Biquad::new(rom_biquad(FRONT_LOW_PASS, sample_rate)),
        loop_low_pass: FirstOrder::new(rom_first_order(LOOP_LOW_PASS, sample_rate)),
        loop_high_pass: Biquad::new(rom_biquad(LOOP_HIGH_PASS, sample_rate)),
        tap_low_pass: FirstOrder::new(rom_first_order(TAP_LOW_PASS, sample_rate)),
        tap_high_pass: FirstOrder::new(rom_first_order(TAP_HIGH_PASS, sample_rate)),
        output_eq: OUTPUT_EQ.map(|c| Biquad::new(rom_biquad(c, sample_rate))),
        lfo: Lfo::default(),
        slew: 0.0,
        slew_coefficient: 0.0,
        predelay: 0.0,
        predelay_target: 0.0,
        predelay_coefficient: one_pole_at(1.0 / 512.0, sample_rate),
        // c52: integer ramp 0x3deb7 per sample in units of 2048 samples.
        sweep: Ramp::with_step(0.0, 253_623.0 / 1_048_576.0 * ratio, sample_rate),
        reso: Ramp::with_step(0.0, 1.888_39e-3, sample_rate),
        last_tap: 0.0,
        first: true,
        rate: f32::NAN,
    })
}

impl Effect for BfFlanger {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let [manual, depth, rate, reso] = [values[0], values[1], values[2], values[3]];
        if rate != self.rate {
            self.rate = rate;
            let hz = increment_hz(decade(&SAWF, rate));
            self.lfo.set_hz(hz, self.sample_rate);
            self.slew_coefficient =
                one_pole_at(decade(&LPF_A0, rate) / 2_147_483_648.0, self.sample_rate);
        }
        let (pre_ms, sweep_ms) = delays_ms(manual, depth);
        // c53 holds whole samples at 44.1 kHz (SPINT).
        self.predelay_target = (pre_ms * 44.1).round() * self.ratio;
        self.sweep.set(sweep_ms * 44.1 * self.ratio);
        self.reso.set(decade(&RESO, reso));
        if self.first {
            self.first = false;
            self.predelay = self.predelay_target;
            self.sweep.jump(self.sweep.target);
            self.reso.jump(self.reso.target);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mono = 0.5 * (left + right);
        let front = self
            .front_low_pass
            .process(self.front_high_pass.process(mono));
        let compressed = shaper(self.reso.tick() * self.last_tap + front);
        let written = self
            .loop_high_pass
            .process(self.loop_low_pass.process(compressed));
        self.line.write(flush(written));
        let m = self.lfo.tick().abs();
        let m2 = m * m;
        let wave = 0.2 * m + 0.8 * m2 * m2;
        self.slew += self.slew_coefficient * (wave - self.slew);
        self.predelay += self.predelay_coefficient * (self.predelay_target - self.predelay);
        let delay = BASE_SAMPLES * self.ratio + self.predelay + self.sweep.tick() * self.slew;
        let tap = self.line.read_linear(delay);
        self.last_tap = tap;
        let wet = self.tap_high_pass.process(self.tap_low_pass.process(tap));
        let mut out = 1.07 * mono + wet;
        for biquad in &mut self.output_eq {
            out = biquad.process(out);
        }
        let out = guard(out);
        [out, out]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.front_high_pass.clear();
        self.front_low_pass.clear();
        self.loop_low_pass.clear();
        self.loop_high_pass.clear();
        self.tap_low_pass.clear();
        self.tap_high_pass.clear();
        for biquad in &mut self.output_eq {
            biquad.clear();
        }
        self.last_tap = 0.0;
        self.slew = 0.0;
        self.lfo.phase = 0;
        self.predelay = self.predelay_target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::modulation::common::testing::rms;

    #[test]
    fn editor_delays_match_the_original_tables() {
        // MNL 0, DEPTH 80: the original stores c53 = 57 samples and
        // c52 = 0.250915 (× 46.4172 ms).
        let (pre, sweep) = delays_ms(0.0, 80.0);
        assert_eq!((pre * 44.1).round(), 57.0);
        assert!((sweep / SWEEP_UNIT_MS - 0.250_915).abs() < 1e-4);
        // MNL 100, DEPTH 100: c53 = 40, c52 = 0.252062.
        let (pre, sweep) = delays_ms(100.0, 100.0);
        assert_eq!((pre * 44.1).round(), 40.0);
        assert!((sweep / SWEEP_UNIT_MS - 0.252_062).abs() < 1e-4);
        assert_eq!(delays_ms(100.0, 0.0).1, 0.0);
    }

    #[test]
    fn shaper_is_unity_for_small_signals_and_bounded() {
        for x in [0.001f32, -0.001, 0.01] {
            assert!((shaper(x) / x - 1.0).abs() < 0.02, "{x}");
        }
        for x in [-100.0f32, -2.0, 2.0, 100.0] {
            assert!(shaper(x).abs() <= 1.0);
        }
    }

    #[test]
    fn defaults_are_audible_and_finite() {
        for rate in [44_100.0, 96_000.0] {
            let mut effect = create(rate);
            let mut v = [0.0; crate::MAX_PARAMS];
            v[..4].copy_from_slice(&[100.0, 80.0, 30.0, 50.0]);
            effect.set(&v, 120.0);
            let n = rate as usize;
            let out: Vec<f32> = (0..n)
                .map(|i| {
                    let x = 0.3 * (i as f32 * std::f32::consts::TAU * 220.0 / rate).sin();
                    effect.process([x, x])[0]
                })
                .collect();
            let level = rms(&out[n / 2..]);
            assert!(out.iter().all(|x| x.is_finite()));
            assert!(level > 0.1 && level < 1.5, "{rate}: {level}");
        }
    }
}
