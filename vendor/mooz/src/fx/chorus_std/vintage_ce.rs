//! VintageCE (VTGCE.ZDL): `vintage_ce.rs` (original 0000..0500), a BBD-style
//! chorus with emphasis and a wet ducker.
//!
//! Callback: mono `0.5 (L + R)` through two ROM high-passes (≈20 and 15 Hz,
//! the "first" output is also the dry signal and the detector input), a ROM
//! low-pass (≈13 kHz) and a ROM pre-emphasis biquad (+8.5 dB near 5 kHz) into
//! the ring. A Q31 triangle `|phase| * c6` (0.487) reads one tap at
//! `87 + lfo * 256` samples (2.0..4.8 ms). The tap passes the de-emphasis
//! (first-order ≈1.5 kHz plus a biquad). A ROM high-pass biquad on "first"
//! feeds a ducker: `4 |detector| * c12`, limited to 1, sets the target of a
//! ≈10 ms one-pole gain `1 - level` that scales the wet. `out = c2 (first +
//! c41 * wet * gain)` on both channels: the output is mono. (The held peak
//! the callback also tracks is never read.)
//!
//! Editors decoded from the disassembly:
//! - Comp: c12 = 0 at 0, else 2^Comp (doubling loop): more Comp ducks the
//!   chorus on louder playing.
//! - Rate: c5 = curve(267826, 38956, 50, square): 0.4..2.75 Hz.
//! - Mix: c41 = curve(2, 0, 101, linear), timed setter. Level: c2 = Level/100.
use super::common::{
    FirstOrder, Q31, TimedRamp, UNSET, defaults, increment, one_pole, rom_biquad, sanitize, step,
    triangle, word,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, DelayLine, flush},
    zoom::{self, ORIGINAL_RATE, Ramp, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_vintage_ce",
    name: "VintageCE",
    category: Category::Modulation,
    zdl: "VTGCE.ZDL",
    description: "BBD-style mono chorus with emphasis and wet ducking",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Comp",
            max: 9,
            default: 2,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 24,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

const INPUT_1: [u32; 3] = [0x3f7f5d01, 0xbf7f5d01, 0x3f7eba02];
const INPUT_2: [u32; 3] = [0x3f7f743d, 0xbf7f743d, 0x3f7ee87a];
const INPUT_3: [u32; 3] = [0x3f1220f4, 0x3f1220f4, 0xbe1107a2];
const EMPHASIS: [u32; 5] = [0x3fc71f12, 0xbf80ebf0, 0xbe662dd0, 0x3f80ebf0, 0xbea96562];
const DEEMPHASIS_1: [u32; 3] = [0x3dc665fd, 0x3dc665fd, 0x3f4e6681];
const DEEMPHASIS_2: [u32; 5] = [0x3f5efa45, 0x3f1ca77b, 0x3e89fa60, 0xbf1ca77b, 0xbe0fddd4];
const DETECTOR: [u32; 5] = [0x3f7edd4c, 0xbffedd4c, 0x3f7edd4c, 0x3ffedd17, 0xbf7dbb02];
/// c43/c44: ducker gain smoothing.
const DUCK: (u32, u32) = (0x3b10e963, 0x3f7f6f17);
/// c6 (Q31) and c7 (samples): sweep and base of the tap.
const SWEEP: f32 = 0x3e48_85f6 as f32 * Q31 * 256.0;
const BASE: f32 = 87.0;

struct VintageCe {
    sample_rate: f32,
    scale: f32,
    line: DelayLine,
    input: [FirstOrder; 3],
    emphasis: Biquad,
    deemphasis: (FirstOrder, Biquad),
    detector: Biquad,
    duck: (f32, f32),
    sensitivity: f32,
    gain: f32,
    phase: u32,
    increment: u32,
    wet: TimedRamp,
    level: Ramp,
    last: Values,
}

impl VintageCe {
    fn new(sample_rate: f32) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        let mut effect = Self {
            sample_rate,
            scale,
            line: DelayLine::new(((BASE + SWEEP) * scale).ceil() as usize + 8),
            input: [
                FirstOrder::rom(INPUT_1, sample_rate),
                FirstOrder::rom(INPUT_2, sample_rate),
                FirstOrder::rom(INPUT_3, sample_rate),
            ],
            emphasis: Biquad::new(rom_biquad(EMPHASIS, sample_rate)),
            deemphasis: (
                FirstOrder::rom(DEEMPHASIS_1, sample_rate),
                Biquad::new(rom_biquad(DEEMPHASIS_2, sample_rate)),
            ),
            detector: Biquad::new(rom_biquad(DETECTOR, sample_rate)),
            duck: one_pole(word(DUCK.0), word(DUCK.1), sample_rate),
            sensitivity: 0.0,
            gain: 1.0,
            phase: 0,
            increment: 0,
            wet: TimedRamp::new(0.0, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            last: UNSET,
        };
        effect.set(&defaults(&INFO), 120.0);
        effect.wet.settle();
        effect.level.jump(effect.level.target);
        effect
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(VintageCe::new(sample_rate))
}

impl Effect for VintageCe {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let v = sanitize(values, &INFO);
        let last = self.last;
        if v[0] != last[0] {
            let comp = step(v[0], 9);
            self.sensitivity = if comp == 0 {
                0.0
            } else {
                (1u32 << comp) as f32
            };
        }
        if v[1] != last[1] {
            self.increment = increment(curve(267_826.0, 38_956.0, 50, v[1], 1), self.sample_rate);
        }
        if v[2] != last[2] {
            self.wet.set(curve(2.0, 0.0, 101, v[2], 0));
        }
        if v[3] != last[3] {
            self.level.set(curve(1.5, 0.0, 151, v[3], 0));
        }
        self.last = v;
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let [first_section, second_section, third_section] = &mut self.input;
        let first = first_section.process(0.5 * (left + right));
        let third = third_section.process(second_section.process(first));
        self.line.write(self.emphasis.process(third));

        self.phase = self.phase.wrapping_add(self.increment);
        let delay = (BASE + triangle(self.phase) * SWEEP) * self.scale;
        let tap = self.line.read_linear(delay);
        let (section, biquad) = &mut self.deemphasis;
        let chorus = biquad.process(section.process(tap));

        let detector = self.detector.process(first);
        let candidate = (4.0 * detector.abs() * self.sensitivity).min(1.0);
        let (duck_in, duck_pole) = self.duck;
        self.gain = flush(self.gain * duck_pole + duck_in * (1.0 - candidate));

        let out = self.level.tick() * (first + self.wet.tick() * chorus * self.gain);
        [out, out]
    }

    fn clear(&mut self) {
        self.line.clear();
        for section in &mut self.input {
            section.clear();
        }
        self.emphasis.clear();
        self.deemphasis.0.clear();
        self.deemphasis.1.clear();
        self.detector.clear();
        self.gain = 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dsp::gain_to_db, fx::chorus_std::common::increment_hz};

    #[test]
    fn emphasis_and_deemphasis_roughly_cancel() {
        for rate in [44_100.0, 96_000.0] {
            let pre = rom_biquad(EMPHASIS, rate);
            let post = FirstOrder::rom(DEEMPHASIS_1, rate);
            let post2 = rom_biquad(DEEMPHASIS_2, rate);
            let at = |hz| {
                gain_to_db(
                    pre.magnitude(rate, hz) * post.magnitude(rate, hz) * post2.magnitude(rate, hz),
                )
            };
            assert!((gain_to_db(pre.magnitude(rate, 5000.0)) - 8.5).abs() < 0.3);
            assert!(at(200.0).abs() < 0.5);
        }
    }

    #[test]
    fn comp_doubles_and_ducks_loud_playing() {
        let mut effect = VintageCe::new(48_000.0);
        let mut values = defaults(&INFO);
        values[0] = 9.0;
        effect.set(&values, 120.0);
        assert_eq!(effect.sensitivity, 512.0);
        for i in 0..9600 {
            let x = (i as f32 * 0.05).sin() * 0.5;
            effect.process([x, x]);
        }
        assert!(effect.gain < 0.05, "{}", effect.gain);
        values[0] = 0.0;
        effect.set(&values, 120.0);
        for i in 0..9600 {
            let x = (i as f32 * 0.05).sin() * 0.5;
            effect.process([x, x]);
        }
        assert!(effect.gain > 0.99);
    }

    #[test]
    fn rate_spans_point_four_to_two_point_seven_hz() {
        let mut effect = VintageCe::new(48_000.0);
        let mut values = defaults(&INFO);
        values[1] = 0.0;
        effect.set(&values, 120.0);
        assert!((increment_hz(effect.increment, 48_000.0) - 0.4).abs() < 0.005);
        values[1] = 49.0;
        effect.set(&values, 120.0);
        assert!((increment_hz(effect.increment, 48_000.0) - 2.75).abs() < 0.01);
    }
}
