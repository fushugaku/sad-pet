//! Step (STEP.ZDL, `Fx_FLT_Step`): a comb filter whose delay jumps in steps.
//!
//! Callback topology, followed as recovered: the stereo input is averaged;
//! the dry input's instantaneous level (×56.2, clipped to 1) is sampled and
//! held once per Rate period; that held value is smoothed by the Shape
//! one-pole (Q31 `c20`/`c21`) and scaled by Depth (`c23`) into a 0..256-sample
//! delay (`offset >> 23` plus a linear-interpolation fraction). The delay line
//! is fed by `mean * c5 + tap * c6` through a 3.5 kHz one-pole low-pass
//! (`c7`/`c9`); the output is `0.875 * (mean - tap)` times Level, the same on
//! both channels. The callback's second wave/tap pair is identical to the
//! first with the editors' coefficients (`c15..c19` stay at their ROM values),
//! so it is computed once.
//!
//! Editors decoded from the disassembly: Shape (ROM table `0x800005b8`),
//! Rate (square integer curve 48695..974939 per 2^31, i.e. 1..20 Hz, with the
//! original count of 49 that saturates above 48; synced values use the Main
//! LFO divisions), Reso (`c5 = 1 - 0.04 r`, `c6 = -0.4 - 0.05 r`, float setter
//! step 0x3af7c5ec), Depth (integer curve 374870113..986500299 per 2^31) and
//! Level (`value / 100`). Deviation: the original mutes the taps for ~16 ms
//! around a Depth change (`C[3]` ramp and `onfaft`); here the delay span
//! glides instead, so Depth can be modulated continuously.
use super::common::{self, Changes, Q31};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{DelayLine, Smoother, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_step",
    name: "Step",
    category: Category::Filter,
    zdl: "STEP.ZDL",
    description: "Comb filter stepped by the sampled input level",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 78,
            default: 25,
            display: Display::sync_rate(50, 0.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Shape",
            max: 10,
            default: 10,
            display: Display::INT,
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

/// ROM `0x800005b8`: Shape → Q31 smoothing coefficient `c20`.
const SHAPE_TABLE: [u32; 11] = [
    0x0020bdea, 0x00299f8f, 0x0037a445, 0x0048ec3a, 0x005c872e, 0x007362d1, 0x008ba03c, 0x00a4c76f,
    0x00bfc611, 0x00dd11f1, 0x00fa570d,
];
/// `c13`: detector gain (+35 dB).
const DETECTOR_GAIN: f32 = f32::from_bits(0x4260_efc1);
/// `c28`, `-c29`.
const MIX: f32 = f32::from_bits(0x3f60_0700);
/// `c7`, `c9`: the low-pass in front of the delay line.
const LOW_PASS_GAIN: f32 = f32::from_bits(0x3ec9_d9c4);
const LOW_PASS_POLE: f32 = f32::from_bits(0x3f1b_131e);
/// A Q31 offset shifted right by 23: 256 samples at full scale.
const OFFSET_SAMPLES: f32 = 256.0;

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Step::new(sample_rate))
}

impl Step {
    fn new(sample_rate: f32) -> Self {
        let scale = 1.0 / common::ratio(sample_rate);
        let (low_pass_gain, low_pass_pole) =
            common::one_pole(LOW_PASS_GAIN, LOW_PASS_POLE, sample_rate);
        Self {
            sample_rate,
            changes: Changes::new(),
            line: DelayLine::new((OFFSET_SAMPLES * scale).ceil() as usize + 4),
            span_scale: OFFSET_SAMPLES * scale,
            capture_phase: 0.0,
            capture_increment: 0.0,
            held: 0.0,
            primary: 0.0,
            shape: 0.0,
            span: Smoother::new(0.0, 0.02, sample_rate),
            low_pass_gain,
            low_pass_pole,
            low_pass: 0.0,
            feedback: 0.0,
            input_gain: Ramp::new(1.0, common::FAST_STEP, sample_rate),
            feedback_gain: Ramp::new(0.0, common::FAST_STEP, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
        }
    }
}

/// Rate 0..50 → Hz (period of the 2^31 capture counter).
fn normal_rate_hz(value: f32) -> f32 {
    let word = common::int_curve(974_939.0, 48_695.0, 49, value, 1);
    (word / Q31 * f64::from(zoom::ORIGINAL_RATE)) as f32
}

fn shape_coefficient(value: f32) -> f32 {
    let value = value.clamp(0.0, 10.0);
    let i = (value as usize).min(9);
    let fraction = value - i as f32;
    let a = SHAPE_TABLE[i] as f32;
    let b = SHAPE_TABLE[i + 1] as f32;
    ((a + (b - a) * fraction) as f64 / Q31) as f32
}

struct Step {
    sample_rate: f32,
    changes: Changes,
    line: DelayLine,
    span_scale: f32,
    capture_phase: f32,
    capture_increment: f32,
    held: f32,
    primary: f32,
    shape: f32,
    span: Smoother,
    low_pass_gain: f32,
    low_pass_pole: f32,
    low_pass: f32,
    feedback: f32,
    input_gain: Ramp,
    feedback_gain: Ramp,
    level: Ramp,
}

impl Effect for Step {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, tempo, first) = self.changes.update(values, bpm);
        if changed & 1 != 0 {
            let word =
                374_870_113.0 + (986_500_299.0 - 374_870_113.0) * f64::from(values[0]) / 100.0;
            let span = (word / Q31) as f32 * self.span_scale;
            self.span.set(span.max(0.0));
            if first {
                self.span.jump(span.max(0.0));
            }
        }
        if changed & 2 != 0 || (tempo && zoom::is_synced(values[1], 50)) {
            let hz = common::sync_hz(values[1], 50, bpm, normal_rate_hz);
            self.capture_increment = hz / self.sample_rate;
        }
        if changed & 4 != 0 {
            let reso = values[2].clamp(0.0, 10.0);
            self.input_gain.set(1.0 - 0.04 * reso);
            self.feedback_gain.set(-0.4 - 0.05 * reso);
            if first {
                self.input_gain.jump(1.0 - 0.04 * reso);
                self.feedback_gain.jump(-0.4 - 0.05 * reso);
            }
        }
        if changed & 8 != 0 {
            let coefficient = shape_coefficient(values[3]);
            self.shape = 1.0 - common::pole(1.0 - coefficient, self.sample_rate);
        }
        if changed & 16 != 0 {
            self.level.set(values[4].max(0.0) / 100.0);
            if first {
                self.level.jump(values[4].max(0.0) / 100.0);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        // Sample-and-hold of the clipped instantaneous dry level.
        self.capture_phase += self.capture_increment;
        if self.capture_phase >= 1.0 {
            self.capture_phase -= 1.0;
            self.held = (mean * DETECTOR_GAIN).abs().min(1.0);
        }
        self.primary = flush(self.primary + (self.held - self.primary) * self.shape);
        let delay = self.primary * self.span.tick();
        let drive = self.feedback * self.feedback_gain.tick() + mean * self.input_gain.tick();
        self.low_pass = flush(self.low_pass_gain * drive + self.low_pass_pole * self.low_pass);
        self.line.write(self.low_pass);
        let tap = self.line.read_linear(delay);
        self.feedback = tap;
        let out = MIX * (mean - tap) * self.level.tick();
        [out, out]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.capture_phase = 0.0;
        self.held = 0.0;
        self.primary = 0.0;
        self.low_pass = 0.0;
        self.feedback = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn rate_follows_the_editor_curve() {
        assert!((normal_rate_hz(0.0) - 1.0).abs() < 1e-3);
        assert!((normal_rate_hz(48.0) - 20.02).abs() < 0.01);
        assert!((normal_rate_hz(50.0) - 20.02).abs() < 0.01);
        assert!((common::sync_hz(58.0, 50, 120.0, normal_rate_hz) - 2.0).abs() < 1e-4);
    }

    #[test]
    fn shape_table_endpoints() {
        assert!((shape_coefficient(0.0) - 0.000_999_2).abs() < 1e-7);
        assert!((shape_coefficient(10.0) - 0.007_639_8).abs() < 1e-7);
    }

    #[test]
    fn depth_spans_the_original_delay() {
        // Depth 100 at full input: 256 * 0.459373 = 117.6 samples at 44.1 kHz.
        let mut step = Step::new(48_000.0);
        let mut values = Model::from_key("mooz_step").unwrap().defaults();
        values[0] = 100.0;
        step.set(&values, 120.0);
        let expected = 117.6 * 48_000.0 / 44_100.0;
        assert!(
            (step.span.target - expected).abs() < 0.2,
            "{}",
            step.span.target
        );
    }

    #[test]
    fn silence_stays_silent_and_steps_are_bounded() {
        let mut effect = create(44_100.0);
        let model = Model::from_key("mooz_step").unwrap();
        effect.set(&model.defaults(), 120.0);
        for _ in 0..10_000 {
            assert_eq!(effect.process([0.0; 2]), [0.0; 2]);
        }
        let mut peak = 0.0f32;
        for i in 0..44_100 {
            let x = (i as f32 * 0.05).sin() * 0.5;
            let [l, r] = effect.process([x, x]);
            assert_eq!(l, r);
            peak = peak.max(l.abs());
        }
        assert!(peak > 0.05 && peak < 2.0, "{peak}");
    }
}
