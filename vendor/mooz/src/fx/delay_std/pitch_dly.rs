//! PitchDly (PITCHDLY.ZDL), from the recovered `Fx_DLY_PitchDly`
//! (`pitch_delay.rs`) and the decoded editors.
//!
//! Callback: the half-sum plus the pitch-shifted repeat times F.B [6] passes a
//! second-order high-pass [9..13] and a second-order low-pass [14..18] into the
//! main ring. The tap at Time [19] (gated by [3]) is written into a second,
//! short ring that a two-grain shifter reads: a Q32 phase advances by the
//! Pitch increment [20]; each grain reads at `2 + (½ + phase)·3970` samples
//! (two grains half a cycle apart) and is weighted by
//! `max(-1, 1.613·(min(|phase| + 0.07, 1) - 1))` (bias [21], slope [22]): full
//! in the middle of its sweep,
//! fading to zero before the jump. The grain sum (negative, as in the
//! original) is both the repeat fed back and, through the Tone one-pole
//! [33]/[34], the wet signal: `Level·(0.7·dry + Mix·wet)` ([8] = 0.7 in ROM).
//! So the first echo is already shifted and every repeat shifts again.
//!
//! Editors: Time is `v + 1` ms (no sync); a live edit mutes [3] in 16 ms,
//! changes the delay and reopens it after `ms + 120` ms. Pitch selects one of
//! 31 ROM increments (−12..−1 semitones, −15/−10/−5 cents, 0, +5..+15 cents,
//! +1..+12 semitones) applied at once (Main's integer setter with a step
//! larger than any change). Mix `(v/100)^1.41`, F.B `0.7·v/100`, Tone one of
//! eleven one-pole coefficients (~300 Hz..~8.6 kHz), Level `v/100`. Grain
//! length and rate keep their time/Hz values at any host rate; the
//! two-frame lag of the original's window weights is not reproduced.
use super::common::{self, Biquad, DelayLine, Gate, Ramp, f, flush, q31};
use crate::{Effect, Values, catalog::*, zoom};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_pitch_dly",
    name: "PitchDly",
    category: Category::Delay,
    zdl: "PITCHDLY.ZDL",
    description: "Delay whose repeats are pitch shifted again each pass",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 1999,
            default: 89,
            display: Display::number(1.0, 1.0, 0, " ms"),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Pitch",
            max: 30,
            default: 21,
            display: Display::labels(&[
                "-12", "-11", "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "-0.15",
                "-0.10", "-0.05", "0", "0.05", "0.10", "0.15", "1", "2", "3", "4", "5", "6", "7",
                "8", "9", "10", "11", "12",
            ]),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 8,
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

const MAX_MS: f32 = 2000.0;

/// ROM 0x80000458: the filters before the main ring.
const HIGH_PASS: [u32; 5] = [0x3f7eb364, 0xbffeb364, 0x3f7eb364, 0x3ffeb290, 0xbf7d686f];
const LOW_PASS: [u32; 5] = [0x3da9c440, 0x3e29c440, 0x3da9c440, 0x3f8413fb, 0xbeba142e];
/// Window bias [21], slope [22], grain span [23] (Q31) and base offset [24].
const BIAS: u32 = 0x08f5c200;
const SLOPE: u32 = 0x4e739c00;
const SPAN: u32 = 0x7c0fc0fb;
const BASE: f32 = 2.0;
/// Fixed dry gain [8].
const DRY: u32 = 0x3f333333;

/// ROM 0x800006d0: Q32 phase increments per 44.1 kHz sample.
const STEPS: [u32; 31] = [
    0x00084188, 0x0007c3db, 0x00073eb4, 0x0006b1a3, 0x00061c2e, 0x00057dd6, 0x0004d614, 0x00042458,
    0x0003680b, 0x0002a08a, 0x0001cd2e, 0x0000ed3f, 0x00002477, 0x00001859, 0x00000c31, 0x00000000,
    0xfffff3c6, 0xffffe783, 0xffffdb37, 0xffff04a5, 0xfffdfa58, 0xfffce036, 0xfffbb54c, 0xfffa789c,
    0xfff92918, 0xfff7c5a0, 0xfff64d05, 0xfff4be05, 0xfff3174b, 0xfff1576e, 0xffef7cf0,
];

/// ROM 0x80000750: Tone one-pole coefficients [33].
const TONE: [u32; 11] = [
    0x3d2bc234, 0x3d3ce73f, 0x3d85de54, 0x3dd73339, 0x3e2ad5e2, 0x3e7e8852, 0x3eb08151, 0x3ee4ef96,
    0x3f0befe1, 0x3f22a11f, 0x3f35422b,
];

pub struct PitchDly {
    sample_rate: f32,
    main: DelayLine,
    shifter: DelayLine,
    high_pass: Biquad,
    low_pass: Biquad,
    gate: Gate,
    delay: usize,
    pending: usize,
    reopen: u32,
    muting: bool,
    phase: u32,
    increment: u32,
    increments: [u32; 31],
    base: f32,
    span: f32,
    bias: f32,
    slope: f32,
    tones: [f32; 11],
    tone: f32,
    smoothed: f32,
    tap: f32,
    feedback: Ramp,
    wet: Ramp,
    level: Ramp,
    time_ms: f32,
    started: bool,
}

impl PitchDly {
    pub fn new(sample_rate: f32) -> Self {
        let ratio = common::rate_ratio(sample_rate);
        // [23] in Q31 times 2^31, shifted down by [26] = 19 bits.
        let span = q31(SPAN) * 4096.0 * ratio;
        let base = BASE * ratio;
        let increments = STEPS.map(|step| (step as i32 as f32 / ratio).round() as i32 as u32);
        Self {
            sample_rate,
            main: common::line_for_ms(MAX_MS, 2, sample_rate),
            shifter: DelayLine::new((span + base).ceil() as usize + 4),
            high_pass: Biquad::new(common::rom_biquad(HIGH_PASS, sample_rate)),
            low_pass: Biquad::new(common::rom_biquad(LOW_PASS, sample_rate)),
            gate: Gate::new(sample_rate, true),
            delay: 1,
            pending: 1,
            reopen: 0,
            muting: false,
            phase: 0,
            increment: increments[21],
            increments,
            base,
            span,
            bias: q31(BIAS),
            slope: 1.0 + q31(SLOPE),
            tones: TONE.map(|w| common::smoothing_at(f(w), sample_rate)),
            tone: 1.0,
            smoothed: 0.0,
            tap: 0.0,
            feedback: common::setter(0.0, sample_rate),
            wet: common::setter(0.0, sample_rate),
            level: common::setter(1.0, sample_rate),
            time_ms: -1.0,
            started: false,
        }
    }

    /// One grain: its read offset and window weight for a Q32 phase.
    #[inline]
    fn grain(&self, phase: u32) -> (f32, f32) {
        let signed = phase as i32;
        let offset = self.base + (0.5 + signed as f32 * (1.0 / 4_294_967_296.0)) * self.span;
        let magnitude = signed.unsigned_abs() as f32 * (1.0 / 2_147_483_648.0);
        let weight = (self.slope * ((magnitude + self.bias).min(1.0) - 1.0)).max(-1.0);
        (offset, weight)
    }
}

impl Effect for PitchDly {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let ms = common::param(values[0], 1999.0).round() + 1.0;
        self.increment = self.increments[common::index(values[1], 30)];
        let wet = zoom::curve(1.0, 0.0, 101, common::param(values[2], 100.0), 5);
        let feedback = 0.7 * common::param(values[3], 100.0) / 100.0;
        self.tone = self.tones[common::index(values[4], 10)];
        let level = common::param(values[5], 150.0) / 100.0;
        self.wet.set(wet);
        self.feedback.set(feedback);
        self.level.set(level);
        if ms != self.time_ms {
            self.time_ms = ms;
            self.pending = common::tap_samples(ms, self.sample_rate, &self.main).max(1);
            self.reopen = common::ms_to_samples(ms + 120.0, self.sample_rate) as u32;
            if self.started {
                self.muting = true;
                self.gate.mute();
            } else {
                self.delay = self.pending;
            }
        }
        if !self.started {
            self.started = true;
            self.wet.jump(wet);
            self.feedback.jump(feedback);
            self.level.jump(level);
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let feedback = self.feedback.tick();
        let wet = self.wet.tick();
        let level = self.level.tick();
        if self.muting && self.gate.closed() {
            self.muting = false;
            self.delay = self.pending;
            self.gate.open_after(self.reopen);
        }
        let gain = self.gate.tick();
        self.shifter.write(self.tap);
        self.phase = self.phase.wrapping_add(self.increment);
        let (offset_a, weight_a) = self.grain(self.phase);
        let (offset_b, weight_b) = self.grain(self.phase ^ 0x8000_0000);
        let shifted = gain
            * (weight_a * self.shifter.read_linear(offset_a)
                + weight_b * self.shifter.read_linear(offset_b));
        let driven = feedback * shifted + 0.5 * (left + right);
        let recorded = self.low_pass.process(self.high_pass.process(driven));
        self.main.write(recorded);
        self.smoothed = flush(self.smoothed + (shifted - self.smoothed) * self.tone);
        let echo = self.smoothed * wet;
        self.tap = flush(gain * self.main.read(self.delay));
        let dry = f(DRY);
        [level * (left * dry + echo), level * (right * dry + echo)]
    }

    fn clear(&mut self) {
        self.main.clear();
        self.shifter.clear();
        self.high_pass.clear();
        self.low_pass.clear();
        self.smoothed = 0.0;
        self.tap = 0.0;
        self.phase = 0;
        if self.muting {
            self.muting = false;
            self.delay = self.pending;
        }
        self.gate.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(PitchDly::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::super::common::testing::*;
    use super::*;
    use crate::Model;

    fn defaults() -> Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    /// Pitch ratio implied by an increment: the grain delay changes by
    /// `span · increment / 2^32` samples per sample.
    fn ratio(effect: &PitchDly, index: usize) -> f32 {
        let increment = effect.increments[index] as i32 as f32;
        1.0 - effect.span * increment / 4_294_967_296.0
    }

    #[test]
    fn pitch_table_spans_an_octave_each_way() {
        for sample_rate in [44_100.0, 96_000.0] {
            let effect = PitchDly::new(sample_rate);
            let cents = |i| 1200.0 * ratio(&effect, i).log2();
            assert!((cents(0) + 1200.0).abs() < 3.0, "{}", cents(0));
            assert!((cents(30) - 1200.0).abs() < 3.0, "{}", cents(30));
            assert!((cents(11) + 100.0).abs() < 1.0);
            assert!((cents(16) - 5.0).abs() < 0.3);
            assert!(cents(15).abs() < 1e-6);
        }
    }

    #[test]
    fn windows_overlap_without_gaps() {
        let effect = PitchDly::new(48_000.0);
        for i in 0..64 {
            let phase = (i as u32) << 26;
            let (_, a) = effect.grain(phase);
            let (_, b) = effect.grain(phase ^ 0x8000_0000);
            let sum = -(a + b);
            assert!((0.99..1.4).contains(&sum), "{i}: {sum}");
        }
        // The jump of each grain (|phase| near 1) is silent.
        assert!(effect.grain(0x7fff_ffff).1.abs() < 1e-6);
    }

    #[test]
    fn octave_up_doubles_the_frequency() {
        let sample_rate = 48_000.0;
        let mut effect = PitchDly::new(sample_rate);
        // Pitch +12, no feedback, wet only through a bright tone.
        effect.set(
            &with(defaults(), &[(0, 99.0), (1, 30.0), (3, 0.0), (4, 10.0)]),
            120.0,
        );
        let out = render(&mut effect, 48_000, sine(300.0, 0.5, sample_rate));
        // Count zero crossings of the wet part (output minus 0.7 dry).
        let wet: Vec<f32> = out
            .iter()
            .enumerate()
            .skip(24_000)
            .map(|(i, f)| f[0] - 0.7 * sine(300.0, 0.5, sample_rate)(i)[0])
            .collect();
        let crossings = wet.windows(2).filter(|w| w[0] < 0.0 && w[1] >= 0.0).count();
        assert!((crossings as f32 - 300.0).abs() < 30.0, "{crossings}");
    }

    #[test]
    fn maximum_feedback_stays_bounded() {
        let mut effect = PitchDly::new(44_100.0);
        effect.set(&with(defaults(), &[(0, 0.0), (3, 100.0), (4, 10.0)]), 120.0);
        let out = render(&mut effect, 441_000, |i| {
            if i < 4410 { [0.9, 0.9] } else { [0.0; 2] }
        });
        assert!(out.iter().all(|f| f[0].abs() < 16.0 && f[0].is_finite()));
        assert!(rms(&out, 0, 400_000) < 0.05);
    }
}
