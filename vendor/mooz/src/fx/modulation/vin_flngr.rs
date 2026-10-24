//! VinFLNGR (VINFLNGR.ZDL), from the recovered `Fx_MOD_VinFLNGR` callback
//! (`vintage_flanger.rs`) and its decoded editors.
//!
//! Callback, per sample: the stereo mean plus `c13 × last tap` (Reso) passes a
//! 10 kHz first-order low-pass (c14..16, bilinear) into the delay line — no
//! input high-pass, unlike Flanger. The LFO is a plain triangle `|phase|`
//! (c19/c20 add a 0.2 % square term) scaled by Depth (c23); the Manual/PreD
//! offset c24 is smoothed by a one-pole of 0.0085 per sample (c21/c22). The
//! delay is `5 + offset + depth × triangle` samples (c8, shift 21), read with
//! linear interpolation; the tap times the signed Mix (c6) is added to each dry
//! channel and Level (c2) scales both.
//!
//! Editors: Depth `context[22]` max 0x58494583 square law (706 samples at
//! 100), Rate as Flanger but 41..50 runs 0.71..10 Hz (0x000edc5b), Reso
//! `Fx_Mod_Flg_reso_tbl` −0.95..+0.95, PreD `c24 = (n + 5) << 22`
//! (10..110 samples), Mix 0..1 negated while Reso < 10, Level 0..1.5. All
//! editors were decoded; nothing here is approximated.
use super::common::{
    FirstOrder, Lfo, Ramp, guard, increment_hz, one_pole_at, rate_hz, stepped, table,
};
use super::flanger_core::rate_increment;
use crate::dsp::{Coefficients, DelayLine, flush, scale_samples};
use crate::zoom::{DEFAULT_STEP, ORIGINAL_RATE};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_vin_flngr",
    name: "VinFLNGR",
    category: Category::Modulation,
    zdl: "VINFLNGR.ZDL",
    description: "Vintage flanger with a dark triangle-swept delay",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 47,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 78,
            default: 7,
            display: Display::sync_rate(50, 0.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Reso",
            max: 20,
            default: 18,
            display: Display::offset(-10.0),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "PreD",
            max: 50,
            default: 4,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 65,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// `Fx_Mod_Flg_reso_tbl`, second word of each entry (c13).
const RESO: [f32; 21] = [
    -0.949_980_1,
    -0.872_219,
    -0.794_427_3,
    -0.716_666_2,
    -0.638_874_5,
    -0.561_082_8,
    -0.483_321_6,
    -0.405_53,
    -0.327_768_8,
    -0.249_977_1,
    0.0,
    0.249_977_1,
    0.327_768_8,
    0.405_53,
    0.483_321_6,
    0.561_082_8,
    0.638_874_5,
    0.716_666_2,
    0.794_427_3,
    0.872_219,
    0.949_980_1,
];

/// 0x58494583 >> 21 and the c23 ramp 0x3deb7 >> 21 per sample.
const DEPTH_SAMPLES: f32 = 706.295_9;
const DEPTH_STEP: f32 = 253_623.0 / 2_097_152.0;
/// c21/(1 − c22): DC gain of the Manual smoother.
const OFFSET_GAIN: f32 = 0.996_43;
const FAST_MAXIMUM: f32 = 973_915.0;
const RESO_STEP: f32 = 1.888_39e-3;

struct VinFlanger {
    sample_rate: f32,
    ratio: f32,
    line: DelayLine,
    feedback_filter: FirstOrder,
    lfo: Lfo,
    offset: f32,
    offset_target: f32,
    offset_coefficient: f32,
    depth: Ramp,
    feedback: Ramp,
    mix: Ramp,
    level: Ramp,
    last_tap: f32,
    first: bool,
    rate: f32,
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let ratio = sample_rate / ORIGINAL_RATE;
    let longest = 5.0 + 110.0 + DEPTH_SAMPLES + 4.0;
    Box::new(VinFlanger {
        sample_rate,
        ratio,
        line: DelayLine::new(scale_samples(longest, sample_rate).ceil() as usize + 4),
        feedback_filter: FirstOrder::new(Coefficients::lowpass1(sample_rate, 10_000.0)),
        lfo: Lfo::default(),
        offset: 0.0,
        offset_target: 0.0,
        offset_coefficient: one_pole_at(0.008_514_4, sample_rate),
        depth: Ramp::with_step(0.0, DEPTH_STEP * ratio, sample_rate),
        feedback: Ramp::with_step(0.0, RESO_STEP, sample_rate),
        mix: Ramp::with_step(0.0, DEFAULT_STEP, sample_rate),
        level: Ramp::with_step(1.0, DEFAULT_STEP, sample_rate),
        last_tap: 0.0,
        first: true,
        rate: f32::NAN,
        bpm: 0.0,
    })
}

impl Effect for VinFlanger {
    fn set(&mut self, values: &Values, bpm: f32) {
        let [depth, rate, reso, predelay, mix, level] = [0, 1, 2, 3, 4, 5].map(|i| values[i]);
        if rate != self.rate || bpm != self.bpm {
            let hz = rate_hz(rate, 50, bpm, |v| {
                increment_hz(stepped(v, |n| rate_increment(n, FAST_MAXIMUM)))
            });
            self.lfo.set_hz(hz, self.sample_rate);
            self.rate = rate;
            self.bpm = bpm;
        }
        let d = depth.clamp(0.0, 100.0) * 0.01;
        self.depth.set(DEPTH_SAMPLES * d * d * self.ratio);
        self.feedback.set(table(&RESO, reso));
        let wet = mix.clamp(0.0, 100.0) * 0.01;
        self.mix.set(if reso.round() < 10.0 { -wet } else { wet });
        self.level.set(level.clamp(0.0, 150.0) * 0.01);
        self.offset_target = (predelay.clamp(0.0, 50.0) + 5.0) * 2.0 * OFFSET_GAIN * self.ratio;
        if self.first {
            self.first = false;
            self.depth.jump(self.depth.target);
            self.feedback.jump(self.feedback.target);
            self.mix.jump(self.mix.target);
            self.level.jump(self.level.target);
            self.offset = self.offset_target;
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        let input = self.feedback.tick() * self.last_tap + mean;
        self.line.write(flush(self.feedback_filter.process(input)));
        let triangle = self.lfo.tick().abs();
        let shape = 0.997_917 * triangle + 0.002_083 * triangle * triangle;
        self.offset += self.offset_coefficient * (self.offset_target - self.offset);
        let delay = 5.0 * self.ratio + self.offset + self.depth.tick() * shape;
        let tap = self.line.read_linear(delay);
        self.last_tap = tap;
        let wet = tap * self.mix.tick();
        let level = self.level.tick();
        [guard(level * (left + wet)), guard(level * (right + wet))]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.feedback_filter.clear();
        self.last_tap = 0.0;
        self.lfo.phase = 0;
        self.offset = self.offset_target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_sets_the_resting_delay() {
        let mut effect = create(44_100.0);
        // Depth 0, Reso 0 (index 10), PreD 50, Mix 100.
        effect.set(&[0.0, 0.0, 10.0, 50.0, 100.0, 100.0, 0.0, 0.0, 0.0], 120.0);
        let mut out = vec![effect.process([1.0, 1.0])[0]];
        for _ in 0..200 {
            out.push(effect.process([0.0, 0.0])[0]);
        }
        let peak = (1..out.len())
            .max_by(|&a, &b| out[a].abs().total_cmp(&out[b].abs()))
            .unwrap();
        // 5 + 110 × 0.9964 = 114.6 samples, split over two taps.
        assert!(peak == 114 || peak == 115, "{peak}");
    }

    #[test]
    fn negative_reso_inverts_the_wet_path() {
        let mut effect = create(48_000.0);
        effect.set(&[0.0, 0.0, 0.0, 0.0, 100.0, 100.0, 0.0, 0.0, 0.0], 120.0);
        let mut out = vec![effect.process([1.0, 1.0])[0]];
        for _ in 0..40 {
            out.push(effect.process([0.0, 0.0])[0]);
        }
        let echo = out[1..]
            .iter()
            .copied()
            .fold(0.0f32, |a, b| if b.abs() > a.abs() { b } else { a });
        assert!(echo < -0.3, "{echo}");
    }
}
