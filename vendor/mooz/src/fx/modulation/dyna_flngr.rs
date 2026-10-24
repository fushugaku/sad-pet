//! DynaFLNGR (DYNFLNGR.ZDL), from the recovered `Fx_MOD_DynaFLNGR` callback
//! (`dynamic_flanger.rs`) and its decoded editors.
//!
//! Callback, per sample: an envelope of the stereo mean (80 Hz high-pass and
//! +6 dB high shelf c5..10, times the Sense gain c11, peak hold with a 0.32 s
//! release c12, a one-pole of DC gain 16 c13/c14, clamped to 1) becomes
//! `shaped = max(env² − 0.125, 0)` (c15..18). The delay line takes
//! `mean + feedback × filtered tap` (c40; the tap passes a 2.2 kHz one-pole
//! c38/c39); the read point is `24 + 256 × depth × (0.25 m + 0.75 m²)`
//! samples for the LFO triangle m (c26..34), linearly interpolated. The output
//! `(wet × tap + dry × mean) × Level` goes to both channels.
//!
//! Sense −10..−1 (mode 0) ducks the effect: feedback `r (1 − 0.66 shaped)`,
//! wet `1.125 − shaped`, dry 1.1. Sense 1..10 (mode 1) opens it: feedback
//! `r × shaped`, wet `1.1 × shaped`, dry `1.1 − 0.031 (1 − shaped)`
//! (`Fx_Mod_DuckFlg_mix_tbl`, wet negated while Reso < 10). Editors: Depth →
//! `context[22]` 0x029931ff..0x75999998 (ramp 706 samples); Rate 0 → 0x0f37,
//! 1..50 → 0x1305..0x164a89 square law over (n − 1)/49 (0.05..15 Hz), above 50
//! tempo notes; Sense → `Fx_Mod_DuckFlg_sens_tbl`; Reso → word 1 of
//! `Fx_Mod_DuckFlg_reso_tbl` (−0.97..+0.97); Level 0..1.5. All decoded; the
//! original only refreshes the wet sign when the Sense mode changes, here it
//! follows Reso directly (as after loading a patch).
use super::common::{
    FirstOrder, Lfo, Ramp, curve, guard, increment_hz, pole_at, rate_hz, rom_first_order, stepped,
    table,
};
use crate::dsp::{DelayLine, flush, scale_samples};
use crate::zoom::{DEFAULT_STEP, ORIGINAL_RATE};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_dyna_flngr",
    name: "DynaFLNGR",
    category: Category::Modulation,
    zdl: "DYNFLNGR.ZDL",
    description: "Flanger whose depth follows the picking dynamics",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 30,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 78,
            default: 38,
            display: Display::sync_rate(50, 0.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Sense",
            max: 19,
            default: 0,
            display: Display::labels(&[
                "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "1", "2", "3", "4",
                "5", "6", "7", "8", "9", "10",
            ]),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Reso",
            max: 20,
            default: 15,
            display: Display::offset(-10.0),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// `Fx_Mod_DuckFlg_reso_tbl`, word 1 of each entry.
const RESO: [f32; 21] = [
    -0.968_872_1,
    -0.872_009_3,
    -0.775_146_5,
    -0.678_253_2,
    -0.581_390_4,
    -0.484_497_07,
    -0.387_634_28,
    -0.290_771_48,
    -0.193_878_17,
    -0.097_015_38,
    0.0,
    0.096_862_79,
    0.193_725_59,
    0.290_618_9,
    0.387_481_7,
    0.484_344_48,
    0.581_237_8,
    0.678_100_6,
    0.774_993_9,
    0.871_856_7,
    0.968_719_5,
];

/// `Fx_Mod_DuckFlg_sens_tbl` (c11).
const SENS: [f32; 20] = [
    1.0,
    0.793_945_3,
    0.630_859_4,
    0.500_976_56,
    0.397_949_22,
    0.315_917_97,
    0.250_976_56,
    0.199_218_75,
    0.158_203_12,
    0.125_488_28,
    0.125_488_28,
    0.184_570_31,
    0.270_996_1,
    0.397_949_22,
    0.583_984_4,
    0.857_421_9,
    1.258_789_1,
    1.847_656_2,
    2.711_914,
    3.980_957,
];

/// `Fx_Mod_DuckFlg_mix_tbl`: c22 (wet slope), c23 (wet), c24 (dry slope), c25 (dry).
const MIX: [[f32; 4]; 2] = [
    [-1.000_122_1, 1.125_137_3, 0.0, 1.100_109_9],
    [1.100_109_9, 0.0, -0.031_253_815, 1.100_109_9],
];

/// Depth curve bounds (c31) as fractions of the 256-sample sweep.
const DEPTH_MIN: f32 = 0.020_299_9;
const DEPTH_MAX: f32 = 0.918_75;
const SWEEP_SAMPLES: f32 = 256.0;
const BASE_SAMPLES: f32 = 24.0;
/// c13 / (1 − c14): DC gain of the envelope smoother.
const ENVELOPE_GAIN: f32 = 16.043;
const RESO_STEP: f32 = 1.888_39e-3;

pub fn rate_increment(value: u32) -> f32 {
    if value == 0 {
        3_895.0
    } else {
        curve(1_460_873.0, 4_869.0, 50, value - 1, 1)
    }
}

struct DynaFlanger {
    sample_rate: f32,
    ratio: f32,
    line: DelayLine,
    lfo: Lfo,
    high_pass: FirstOrder,
    shelf: FirstOrder,
    release: f32,
    peak: f32,
    smooth: f32,
    smooth_pole: f32,
    smooth_gain: f32,
    feedback_pole: f32,
    feedback_gain: f32,
    feedback: f32,
    sens: Ramp,
    reso: Ramp,
    slope: f32,
    slope_target: f32,
    depth: Ramp,
    mix: [Ramp; 4],
    level: Ramp,
    first: bool,
    rate: f32,
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let ratio = sample_rate / ORIGINAL_RATE;
    let smooth_pole = pole_at(0.991_524_4, sample_rate);
    let feedback_pole = pole_at(0.731_425_76, sample_rate);
    let longest = BASE_SAMPLES + SWEEP_SAMPLES + 4.0;
    Box::new(DynaFlanger {
        sample_rate,
        ratio,
        line: DelayLine::new(scale_samples(longest, sample_rate).ceil() as usize + 4),
        lfo: Lfo::default(),
        high_pass: FirstOrder::new(rom_first_order(
            [0.994_333_2, -0.994_333_2, 0.988_666_4],
            sample_rate,
        )),
        shelf: FirstOrder::new(rom_first_order(
            [1.988_222_4, -1.974_075_3, 0.985_852_96],
            sample_rate,
        )),
        release: pole_at(0.999_928_6, sample_rate),
        peak: 0.0,
        smooth: 0.0,
        smooth_pole,
        smooth_gain: ENVELOPE_GAIN * (1.0 - smooth_pole),
        feedback_pole,
        feedback_gain: 1.0 - feedback_pole,
        feedback: 0.0,
        sens: Ramp::with_step(1.0, RESO_STEP, sample_rate),
        reso: Ramp::with_step(0.0, RESO_STEP, sample_rate),
        slope: 0.0,
        slope_target: 0.0,
        depth: Ramp::with_duration(0.0, 706.0, sample_rate),
        mix: [0.0; 4].map(|v| Ramp::with_step(v, RESO_STEP, sample_rate)),
        level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
        first: true,
        rate: f32::NAN,
        bpm: 0.0,
    })
}

impl Effect for DynaFlanger {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [depth, rate, sense, reso, level] = [0, 1, 2, 3, 4].map(|i| values[i]);
        if rate != self.rate || bpm != self.bpm {
            let hz = rate_hz(rate, 50, bpm, |v| increment_hz(stepped(v, rate_increment)));
            self.lfo.set_hz(hz, self.sample_rate);
            self.rate = rate;
            self.bpm = bpm;
        }
        let d = depth.clamp(0.0, 100.0) * 0.01;
        self.depth
            .set((DEPTH_MIN + (DEPTH_MAX - DEPTH_MIN) * d) * SWEEP_SAMPLES * self.ratio);
        self.sens.set(table(&SENS, sense));
        let mode = usize::from(sense.round() >= 10.0);
        let r = table(&RESO, reso);
        // c19/c20 packed as (slope, constant) of the feedback gain.
        let (slope, constant) = if mode == 0 { (-0.66 * r, r) } else { (r, 0.0) };
        self.reso.set(constant);
        let sign = if reso.round() < 10.0 { -1.0 } else { 1.0 };
        let row = MIX[mode];
        self.mix[0].set(sign * row[0]);
        self.mix[1].set(sign * row[1]);
        self.mix[2].set(row[2]);
        self.mix[3].set(row[3]);
        self.slope_target = slope;
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        if self.first {
            self.first = false;
            self.slope = slope;
            for ramp in [
                &mut self.sens,
                &mut self.reso,
                &mut self.depth,
                &mut self.level,
            ] {
                ramp.jump(ramp.target);
            }
            for ramp in &mut self.mix {
                ramp.jump(ramp.target);
            }
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        // Envelope.
        let filtered = self.shelf.process(self.high_pass.process(mean));
        let magnitude = (filtered * self.sens.tick()).abs().min(1.0);
        self.peak = (self.peak * self.release).max(magnitude);
        self.smooth = self.smooth_pole * self.smooth + self.smooth_gain * self.peak;
        let normalized = self.smooth.min(1.0);
        let shaped = (normalized * normalized - 0.125).max(0.0);
        // Feedback slope moves with the same step as the Reso setter.
        let step = RESO_STEP * ORIGINAL_RATE / self.sample_rate;
        self.slope += (self.slope_target - self.slope).clamp(-step, step);
        let feedback_gain = self.reso.tick() + self.slope * shaped;
        self.line.write(flush(feedback_gain * self.feedback + mean));
        let m = self.lfo.tick().abs();
        let shape = 0.25 * m + 0.75 * m * m;
        let delay = BASE_SAMPLES * self.ratio + self.depth.tick() * shape;
        let delayed = self.line.read_linear(delay);
        let wet = self.mix[1].tick() + shaped * self.mix[0].tick();
        let dry = (1.0 - shaped) * self.mix[2].tick() + self.mix[3].tick();
        let out = guard((wet * delayed + dry * mean) * self.level.tick());
        self.feedback = flush(self.feedback_pole * self.feedback + self.feedback_gain * delayed);
        [out, out]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.high_pass.clear();
        self.shelf.clear();
        self.peak = 0.0;
        self.smooth = 0.0;
        self.feedback = 0.0;
        self.lfo.phase = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::modulation::common::testing::rms;

    #[test]
    fn rate_curve_matches_the_editor() {
        assert!((increment_hz(rate_increment(0)) - 0.04).abs() < 2e-3);
        assert!((increment_hz(rate_increment(1)) - 0.05).abs() < 2e-3);
        assert!((increment_hz(rate_increment(50)) - 15.0).abs() < 0.01);
    }

    fn wet_energy(sense: f32, amplitude: f32) -> f32 {
        // Depth 100, slow LFO, Reso 0: compare the output with the dry part.
        let mut effect = create(44_100.0);
        effect.set(&[100.0, 1.0, sense, 10.0, 100.0, 0.0, 0.0, 0.0, 0.0], 120.0);
        let out: Vec<f32> = (0..44_100)
            .map(|i| {
                let x = amplitude * (i as f32 * 0.37).sin();
                effect.process([x, x])[0]
            })
            .collect();
        rms(&out[22_050..]) / amplitude
    }

    #[test]
    fn negative_sense_ducks_loud_signals() {
        let quiet = wet_energy(0.0, 0.001);
        let loud = wet_energy(0.0, 0.5);
        assert!(quiet > loud * 1.2, "{quiet} {loud}");
    }

    #[test]
    fn positive_sense_opens_on_loud_signals() {
        let quiet = wet_energy(19.0, 0.001);
        let loud = wet_energy(19.0, 0.5);
        assert!(loud > quiet * 1.2, "{quiet} {loud}");
    }
}
