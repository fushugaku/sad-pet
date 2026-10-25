//! TapeEcho3 (TAPEECH3.ZDL), from the recovered `Fx_DLY_TapeEcho3`
//! (`tape_echo_three.rs`: `SUB_TapeEcho3` then `SUB_KawaOD_EP_Booster`), its ROM
//! image (`tape_echo_three_parameters.rs`) and the decoded editors.
//!
//! Delay stage: the half-sum times RecLv [90] plus the previous tap times
//! F.B [97] is clipped to ±[126], shaped by a first-order low shelf [98..102]
//! and a first-order low-pass [103..107] and written to the ring. The read
//! position (Q13) follows the Time target [96] through a one-pole (~120 ms,
//! so time changes glide) plus a 35 Hz flutter: a triangle smoothed by a
//! one-pole (0.000427) and scaled to at most 25 samples. The tap feeds back
//! and, through a fixed head-bump biquad [108..112], forms the mono output
//! `dry·[92] + echo·[91]`.
//!
//! Preamp stage (the EP-3 "P-Amp"): the mono signal runs through six input
//! biquads, a 4× polyphase interpolator (16 ROM taps), an asymmetric
//! saturator (positive half saturates softly at 0.925, negative half at
//! −0.308), the original's 21-tap decimator, six output biquads and a gain of
//! 1.897·0.462. Both outputs carry this mono result.
//!
//! Editors: TIME is `v + 10` ms when SYNC is Off, otherwise SYNC selects a note
//! (limited to 3900 ms); live edits move only the target. F.B sets [97] =
//! `0.007 + 1.893·(v/100)²`; above 0.9 the excess `x` (≤ 1) moves the shelf
//! corner `75 + 525x` Hz (−6 dB, then +2 dB overall), the low-pass `1770 − 546x`
//! Hz and the clip `1 − 0.3x` (Main's filter designer, types 9 and 0, designed
//! here at the host rate). RecLv sets [90] = `0.5 + (v/100)²` (the original's
//! on/off handler also resets it to 1 until RecLv or F.B is touched; that
//! load-time quirk is not reproduced). MIX: echo `(v/99)^1.41`, dry 1 up to 70
//! then `1 − ((v−71)/29)^1.41`, both with 16 ms ramps. P-Amp only decides
//! whether the preamp stays on while the effect is bypassed; a mooz effect is
//! never bypassed, so the preamp always runs and P-Amp has no audible effect.
use super::common::{self, Biquad, DelayLine, Fade, FirstOrder, Ramp, f, flush, q31};
use crate::{Effect, Values, catalog::*, dsp::Coefficients, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_tape_echo3",
    name: "TapeEcho3",
    category: Category::Delay,
    zdl: "TAPEECH3.ZDL",
    description: "EP-3 style tape echo with its saturating preamp",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 20,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "TIME",
            max: 990,
            default: 350,
            display: Display::number(10.0, 1.0, 0, " ms"),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "RecLv",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "SYNC",
            max: 15,
            default: 0,
            display: Display::labels(&common::SYNC_LABELS),
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "P-Amp",
            max: 1,
            default: 0,
            display: Display::OFF_ON,
        },
    ],
};

const MAX_MS: f32 = 3900.0;

/// ROM 0x80000698 (128 coefficient words).
const ROM: [u32; 128] = [
    0x00000000, 0x00000000, 0x3f47ae14, 0x3f800000, 0x00000000, 0x3f800000, 0x00000000, 0x3eecbd35,
    0x00000000, 0x3f800000, 0x00000000, 0x3f800000, 0x3f800000, 0x00000000, 0x3f7e799a, 0xbffe7992,
    0x3f7e799a, 0x3ffe7903, 0xbf7cf451, 0x3f730ced, 0xbfafabd2, 0x3f0f0670, 0x3fafabd2, 0xbf02135e,
    0x3f7f8a09, 0xbf7ca490, 0x00000000, 0x3f7c2e99, 0x00000000, 0x3f4f99d4, 0xbec1198b, 0x00000000,
    0x3f10f2f1, 0x00000000, 0x3e126245, 0x3e926224, 0x3e126245, 0x3f2d9ce0, 0xbe7ffc54, 0x3ffc8f75,
    0xbff6dde8, 0x00000000, 0x3f749cf5, 0x00000000, 0x3f7f7436, 0xbf7f7436, 0x00000000, 0x3f7ee87d,
    0x00000000, 0x3f81592e, 0xbf761c47, 0x00000000, 0x3f78cea3, 0x00000000, 0x3f806905, 0xbf7eff4c,
    0x00000000, 0x3f7fd157, 0x00000000, 0x3fe6f556, 0xbfdb268d, 0x00000000, 0x3f7a3ad2, 0x00000000,
    0x3f83caf3, 0xbff224c4, 0x3f5e79ff, 0x3ff224c4, 0xbf660fe4, 0x3f327f91, 0xbf05dc1e, 0x00000000,
    0x3f535c8e, 0x00000000, 0x3ff2dcec, 0x00000000, 0x3ee938de, 0x3fac4218, 0x00000000, 0x3f6ccccd,
    0x3fba39a4, 0x00000000, 0xbe9dddd3, 0xc08bab39, 0x00000000, 0x3f800000, 0x3f800000, 0x00000000,
    0x3f800000, 0x00000000, 0x3f800000, 0x3f3e76c9, 0x3f800000, 0x0000000d, 0x00000013, 0x00000000,
    0x005feae1, 0x00000000, 0x3f9acfd9, 0xbf8e1733, 0x00000000, 0x3f57ad19, 0x00000000, 0x3e67ff15,
    0x3e67ff15, 0x00000000, 0x3f0c0075, 0x00000000, 0x3f8e76b6, 0xbf73fed8, 0x3eada98b, 0x3f73fed8,
    0xbee78463, 0x000dfd4d, 0x7ff202b3, 0x000dfd4d, 0x7ff202b3, 0x40000000, 0x40000000, 0x00061546,
    0x7ff9eab8, 0x0034b23c, 0x0034b23c, 0x00032462, 0x00000000, 0x3f800000, 0x3f800000, 0x00000000,
];

/// ROM 0x80000dc0: interpolator, `UP[4k + phase]` weights `x[n-k]`.
const UP: [u32; 16] = [
    0xbb4a0800, 0xbc1e8000, 0xbcb2ba40, 0xbd229ac0, 0xbd7ce1a0, 0xbdad0730, 0xbdd3bc30, 0xbde9c2c0,
    0xbde9c2c0, 0xbdd3bc30, 0xbdad0730, 0xbd7ce1a0, 0xbd229ac0, 0xbcb2ba40, 0xbc1e8000, 0xbb4a0800,
];
/// ROM 0x80000d68: decimator.
const DOWN: [u32; 21] = [
    0xba56c7ff, 0x3bcd8800, 0x3c265100, 0x3ad73800, 0xba7057ff, 0xba723800, 0x3b7ff600, 0x3c30f980,
    0x3b7ff600, 0xba723800, 0xba7057ff, 0x3ad73800, 0x3c265100, 0x3bcd8800, 0xba56c7ff, 0xba105800,
    0x39318001, 0x3c0d2d00, 0x3c0d2d00, 0x39318001, 0xba105800,
];

fn rom_coefficients(at: usize, sample_rate: f32) -> Coefficients {
    let words = [ROM[at], ROM[at + 1], ROM[at + 2], ROM[at + 3], ROM[at + 4]];
    common::rom_biquad(words, sample_rate)
}

fn rom_section(at: usize, sample_rate: f32) -> Biquad {
    Biquad::new(rom_coefficients(at, sample_rate))
}

fn rom_first(at: usize, sample_rate: f32) -> FirstOrder {
    FirstOrder::new(rom_coefficients(at, sample_rate))
}

/// Main's filter designer (`context[25]`, c00bc2e0) for the two kinds the
/// RecLv/F.B editor uses, with feedback signs negated for [`Biquad`].
fn first_order(sample_rate: f32, hz: f32, low_shelf_db: Option<f32>) -> Coefficients {
    let fs = f64::from(sample_rate);
    let tan = |hz: f64| (std::f64::consts::PI * hz.clamp(1.0, fs * 0.49) / fs).tan();
    let (b0, b1, a1) = match low_shelf_db {
        // Type 9: first-order low shelf; a cut moves the corner up by 1/gain.
        Some(db) => {
            let gain = 10f64.powf(f64::from(db) / 20.0);
            let k = if db >= 0.0 {
                tan(f64::from(hz))
            } else {
                tan(f64::from(hz) / gain)
            };
            let n = 1.0 / (1.0 + k);
            ((1.0 + k * gain) * n, (k * gain - 1.0) * n, (1.0 - k) * n)
        }
        // Type 0: first-order low-pass.
        None => {
            let k = tan(f64::from(hz));
            let n = 1.0 / (1.0 + k);
            (k * n, k * n, (1.0 - k) * n)
        }
    };
    Coefficients {
        b0: b0 as f32,
        b1: b1 as f32,
        b2: 0.0,
        a1: -a1 as f32,
        a2: 0.0,
    }
}

/// The six input and six output ROM biquads; the ROM leaves eight of them
/// first order (b2 = a2 = 0), which run as [`FirstOrder`] (bit-identical).
struct Preamp {
    in0: Biquad,
    in1: Biquad,
    in2: FirstOrder,
    in3: FirstOrder,
    in4: Biquad,
    in5: FirstOrder,
    out0: FirstOrder,
    out1: FirstOrder,
    out2: FirstOrder,
    out3: FirstOrder,
    out4: Biquad,
    out5: FirstOrder,
    up: [f32; 16],
    down: [f32; 21],
    history: [f32; 3],
    lines: [[f32; 5]; 4],
    drive: f32,
    gain: f32,
}

impl Preamp {
    fn new(sample_rate: f32) -> Self {
        Self {
            in0: rom_section(14, sample_rate),
            in1: rom_section(19, sample_rate),
            in2: rom_first(24, sample_rate),
            in3: rom_first(29, sample_rate),
            in4: rom_section(34, sample_rate),
            in5: rom_first(39, sample_rate),
            out0: rom_first(44, sample_rate),
            out1: rom_first(49, sample_rate),
            out2: rom_first(54, sample_rate),
            out3: rom_first(59, sample_rate),
            out4: rom_section(64, sample_rate),
            out5: rom_first(69, sample_rate),
            up: UP.map(f),
            down: DOWN.map(f),
            history: [0.0; 3],
            lines: [[0.0; 5]; 4],
            // [76] drive times the interpolator's ×4; [74]·[7] output gain.
            drive: f(ROM[76]) * 4.0,
            gain: f(ROM[74]) * f(ROM[7]),
        }
    }

    /// The asymmetric transfer of `SUB_KawaOD_EP_Booster` ([77..83]).
    #[inline]
    fn shape(driven: f32) -> f32 {
        let clip = |x: f32| x.clamp(-1.0, 1.0);
        let negative = driven < 0.0;
        let t = clip(driven * if negative { f(ROM[83]) } else { f(ROM[80]) });
        let p = clip(f(ROM[77]) * driven);
        let g = if negative { f(ROM[82]) } else { f(ROM[79]) };
        p * (t - 1.0).abs() + t * g
    }

    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let value = self.in0.process(x);
        let value = self.in1.process(value);
        let value = self.in2.process(value);
        let value = self.in3.process(value);
        let value = self.in4.process(value);
        let value = self.in5.process(value);
        let [h0, h1, h2] = self.history;
        let mut shaped = [0.0; 4];
        for (phase, out) in shaped.iter_mut().enumerate() {
            let up = self.up[phase] * value
                + self.up[4 + phase] * h0
                + self.up[8 + phase] * h1
                + self.up[12 + phase] * h2;
            *out = Self::shape(self.drive * up);
        }
        self.history = [value, h0, h1];
        // The decimator line exactly as the callback lays it out: each phase's
        // newest sample replaces the oldest slot of the previous phase's history.
        let l = &self.lines;
        let line = [
            shaped[0], l[0][0], l[0][1], l[0][2], l[0][3], shaped[1], l[1][0], l[1][1], l[1][2],
            l[1][3], shaped[2], l[2][0], l[2][1], l[2][2], l[2][3], shaped[3], l[3][0], l[3][1],
            l[3][2], l[3][3], l[3][4],
        ];
        let mut sum = 0.0;
        for (tap, x) in self.down.iter().zip(line) {
            sum += tap * x;
        }
        for (history, new) in self.lines.iter_mut().zip(shaped) {
            history.copy_within(..4, 1);
            history[0] = flush(new);
        }
        let out = self.out0.process(4.0 * sum);
        let out = self.out1.process(out);
        let out = self.out2.process(out);
        let out = self.out3.process(out);
        let out = self.out4.process(out);
        let out = self.out5.process(out);
        out * self.gain
    }

    fn clear(&mut self) {
        for section in [&mut self.in0, &mut self.in1, &mut self.in4, &mut self.out4] {
            section.clear();
        }
        for section in [
            &mut self.in2,
            &mut self.in3,
            &mut self.in5,
            &mut self.out0,
            &mut self.out1,
            &mut self.out2,
            &mut self.out3,
            &mut self.out5,
        ] {
            section.clear();
        }
        self.history = [0.0; 3];
        self.lines = [[0.0; 5]; 4];
    }
}

pub struct TapeEcho3 {
    sample_rate: f32,
    line: DelayLine,
    shelf: FirstOrder,
    low_pass: FirstOrder,
    head: Biquad,
    preamp: Preamp,
    target: f32,
    position: f32,
    glide: f32,
    phase: u32,
    increment: u32,
    flutter: f32,
    flutter_smoothing: f32,
    depth: f32,
    tap: f32,
    feedback: Ramp,
    record: Ramp,
    limit: Ramp,
    echo: Fade,
    dry: Fade,
    ramp: f32,
    feedback_key: u32,
    started: bool,
}

impl TapeEcho3 {
    pub fn new(sample_rate: f32) -> Self {
        let ratio = common::rate_ratio(sample_rate);
        // Q31 [123] times the Q31 LFO sum, in Q13 samples.
        let depth = q31(ROM[123]) * 2_147_483_648.0 / 8192.0 * ratio;
        let mut effect = Self {
            sample_rate,
            line: common::line_for_ms(MAX_MS, depth.ceil() as usize + 2, sample_rate),
            shelf: FirstOrder::default(),
            low_pass: FirstOrder::default(),
            head: rom_section(108, sample_rate),
            preamp: Preamp::new(sample_rate),
            target: 1.0,
            position: 1.0,
            glide: common::smoothing_at(q31(ROM[119]), sample_rate),
            phase: 0,
            increment: (ROM[121] as f32 / ratio).round() as u32,
            flutter: 0.0,
            flutter_smoothing: common::smoothing_at(q31(ROM[113]), sample_rate),
            depth,
            tap: 0.0,
            feedback: common::setter(0.0, sample_rate),
            record: common::setter(1.0, sample_rate),
            limit: common::setter(1.0, sample_rate),
            echo: Fade::new(0.0),
            dry: Fade::new(1.0),
            ramp: common::TAP_RAMP_44K * ratio,
            feedback_key: u32::MAX,
            started: false,
        };
        effect.design(0.0);
        effect
    }

    pub fn feedback_gain(value: f32) -> f32 {
        zoom::curve(
            f(0x3ff33333),
            f(0x3be56042),
            101,
            common::param(value, 100.0),
            1,
        )
    }

    /// RecLvl_edit's filter and clip updates for a feedback gain.
    fn design(&mut self, feedback: f32) -> f32 {
        let excess = (feedback - 0.9).clamp(0.0, 1.0);
        let mut shelf = first_order(self.sample_rate, 75.0 + 525.0 * excess, Some(-6.0));
        // The editor scales the numerator by 0x3fa12478 (+2 dB).
        let boost = f(0x3fa12478);
        shelf.b0 *= boost;
        shelf.b1 *= boost;
        self.shelf.set(shelf);
        self.low_pass
            .set(first_order(self.sample_rate, 1770.0 - 546.0 * excess, None));
        1.0 - 0.3 * excess
    }

    pub fn mix_gains(mix: f32) -> (f32, f32) {
        let mix = common::param(mix, 100.0);
        let echo = zoom::curve(1.0, 0.0, 100, mix, 5);
        let dry = if mix < 71.0 {
            1.0
        } else {
            zoom::curve(0.0, 1.0, 30, mix - 71.0, 5)
        };
        (echo, dry)
    }
}

impl Effect for TapeEcho3 {
    fn set(&mut self, values: &Values, bpm: f32) {
        let feedback = Self::feedback_gain(values[0]);
        let (echo, dry) = Self::mix_gains(values[1]);
        let sync = common::index(values[4], 15);
        let ms = if sync == 0 {
            common::param(values[2], 990.0) + 10.0
        } else {
            zoom::delay_note_ms(sync - 1, bpm, MAX_MS)
        };
        let record = zoom::curve(1.5, 0.5, 101, common::param(values[3], 100.0), 1);
        self.target = common::ms_to_samples(ms, self.sample_rate);
        if feedback.to_bits() != self.feedback_key {
            self.feedback_key = feedback.to_bits();
            let limit = self.design(feedback);
            self.limit.set(limit);
        }
        self.feedback.set(feedback);
        self.record.set(record);
        if self.started {
            if echo != self.echo.target {
                self.echo.start(echo, self.ramp);
            }
            if dry != self.dry.target {
                self.dry.start(dry, self.ramp);
            }
        } else {
            self.started = true;
            self.position = self.target;
            self.feedback.jump(feedback);
            self.record.jump(record);
            self.limit.jump(self.limit.target);
            self.echo.jump(echo);
            self.dry.jump(dry);
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let feedback = self.feedback.tick();
        let record = self.record.tick();
        let limit = self.limit.tick();
        let echo = self.echo.tick();
        let dry = self.dry.tick();
        let mono = 0.5 * (left + right);
        let driven = (mono * record + self.tap * feedback).clamp(-limit, limit);
        let written = self.low_pass.process(self.shelf.process(driven));
        self.line.write(written);
        self.phase = self.phase.wrapping_add(self.increment);
        let triangle = (self.phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0);
        self.flutter += (triangle - 0.5 - self.flutter) * self.flutter_smoothing;
        self.position += (self.target - self.position) * self.glide;
        let read = self.position + (self.flutter + 0.5) * self.depth;
        self.tap = flush(self.line.read_linear(read));
        let delayed = dry * mono + echo * self.head.process(self.tap);
        let out = self.preamp.process(delayed);
        [out, out]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.shelf.clear();
        self.low_pass.clear();
        self.head.clear();
        self.preamp.clear();
        self.tap = 0.0;
        self.flutter = 0.0;
        self.position = self.target;
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(TapeEcho3::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::super::common::testing::*;
    use super::*;
    use crate::{Model, dsp::gain_to_db};

    fn defaults() -> Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn saturator_is_continuous_and_asymmetric() {
        assert!(Preamp::shape(0.0).abs() < 1e-7);
        assert!((Preamp::shape(1.0) - 0.925).abs() < 1e-3);
        assert!((Preamp::shape(-1.0) + 0.3083).abs() < 1e-3);
        let slope = Preamp::shape(1e-3) / 1e-3;
        assert!((slope - 2.69).abs() < 0.02, "{slope}");
        let mut previous = Preamp::shape(-2.0);
        for i in -199..200 {
            let y = Preamp::shape(i as f32 * 0.01);
            assert!(y >= previous - 1e-6, "monotonic at {i}");
            previous = y;
        }
    }

    #[test]
    fn designer_kinds_match_the_editor() {
        let shelf = first_order(44_100.0, 600.0, Some(-6.0));
        assert!((gain_to_db(shelf.magnitude(44_100.0, 5.0)) + 6.0).abs() < 0.1);
        assert!(gain_to_db(shelf.magnitude(44_100.0, 15_000.0)).abs() < 0.2);
        // At x = 1 the editor reproduces the ROM's initial [98..102] (before ×1.2589).
        assert!(
            (shelf.b0 * f(0x3fa12478) - f(ROM[98])).abs() < 2e-3,
            "{}",
            shelf.b0
        );
        assert!((-shelf.a1 - f(ROM[101])).abs() < 2e-3);
        let low = first_order(44_100.0, 1770.0, None);
        assert!((gain_to_db(low.magnitude(44_100.0, 1770.0)) + 3.01).abs() < 0.05);
    }

    #[test]
    fn echo_time_follows_time_plus_ten() {
        let sample_rate = 48_000.0;
        let mut effect = TapeEcho3::new(sample_rate);
        // TIME 190 -> 200 ms, MIX 100: echo only.
        effect.set(
            &with(defaults(), &[(0, 0.0), (1, 100.0), (2, 190.0)]),
            120.0,
        );
        assert!((effect.target - 9600.0).abs() < 1e-3);
        let out = render(&mut effect, 14_400, impulse);
        // The flutter adds about 12.5 samples at 44.1 kHz.
        let at = peak_index(&out, 0, 4000);
        assert!(at > 9600 && at < 9650, "{at}");
    }

    #[test]
    fn preamp_gain_is_moderate_at_defaults() {
        let sample_rate = 48_000.0;
        let mut effect = TapeEcho3::new(sample_rate);
        effect.set(&with(defaults(), &[(1, 0.0)]), 120.0);
        let out = render(&mut effect, 48_000, sine(1000.0, 0.1, sample_rate));
        let db = gain_to_db(rms(&out, 0, 24_000) / (0.1 / 2f32.sqrt()));
        assert!(db.abs() < 12.0, "{db}");
    }

    #[test]
    fn maximum_feedback_stays_bounded() {
        let mut effect = TapeEcho3::new(44_100.0);
        effect.set(
            &with(defaults(), &[(0, 100.0), (1, 100.0), (2, 0.0), (3, 100.0)]),
            120.0,
        );
        assert!((effect.limit.target - 0.7).abs() < 1e-6);
        let out = render(&mut effect, 441_000, |i| {
            if i < 4410 { [0.9, 0.9] } else { [0.0; 2] }
        });
        assert!(out.iter().all(|f| f[0].is_finite() && f[0].abs() < 16.0));
    }

    #[test]
    fn mix_curves() {
        assert_eq!(TapeEcho3::mix_gains(0.0), (0.0, 1.0));
        let (echo, dry) = TapeEcho3::mix_gains(100.0);
        assert!((echo - (100.0f32 / 99.0).powf(1.41)).abs() < 1e-5);
        assert!(dry.abs() < 1e-6);
        assert_eq!(TapeEcho3::mix_gains(70.0).1, 1.0);
    }
}
