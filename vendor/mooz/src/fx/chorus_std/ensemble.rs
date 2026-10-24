//! Ensemble (ENSEMBLE.ZDL), with Ba Ensmbl sharing this core and editors:
//! `ensemble.rs` (original 0000..0460, byte-identical in both modules).
//!
//! Callback: mono input `0.5 (L + R)` through the two Tone sections (c7..c12)
//! into the ring (the c6 feedback is 0 in ROM and never set). The LFO is a
//! Q31 triangle `w = |phase| - 0.5` bent into a sine-like curve
//! `w (1 - |w|)`, mapped to `0..1` and scaled by c26; one tap at
//! `c25 + lfo * 512` samples (the second tap is identical since c19 = 0 and
//! the c23/c24 recursions are 1/0). Left gets `+wet`, right `-wet`:
//! `L = c2 (L + c32 tap)`, `R = c2 (R - c32 tap)`.
//!
//! Editors decoded from the disassembly:
//! - Rate: c18 = curve(894784, 8947, 50, cube): 0.09..9.2 Hz; it reruns Depth.
//! - Depth: c26 = (curve(0x56c8eecb, 0, 101, atan) / 10000) * curve(916,
//!   10000, 50, Rate, atan), integer setter over 706 samples (16 ms): the
//!   sweep (up to 347 samples) shrinks with rate.
//! - Offset (run by Depth): c25 = round(0.0441 * (curve(3800, 0, 50, Rate,
//!   atan) + curve(3840, 2640, 101, Depth, atan))) samples (2.6..7.6 ms; µs
//!   to samples). The original mutes the tap around it; here it glides.
//! - Mix: c32 = Mix/100, timed setter. Tone: the shared tone table.
//!   Level: c2 = Level/100.
use super::common::{
    FirstOrder, Q31, TONE, TapTime, TimedRamp, UNSET, defaults, increment, sanitize, step, tone_hp,
    tone_lp, triangle,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::DelayLine,
    zoom::{self, ORIGINAL_RATE, Ramp, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ensemble",
    name: "Ensemble",
    category: Category::Modulation,
    zdl: "ENSEMBLE.ZDL",
    description: "Ensemble chorus, wet in opposite phase left and right",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 29,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 8,
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

/// c25 (samples at 44.1 kHz) and c26 (Q31 integer) from Depth and Rate.
fn depth_words(depth: f32, rate: f32) -> (f32, f32) {
    let microseconds =
        curve(3800.0, 0.0, 50, rate, 4).round() + curve(3840.0, 2640.0, 101, depth, 4).round();
    let base = (0.0441 * microseconds).round();
    let factor = curve(916.0, 10_000.0, 50, rate, 4).round();
    let sweep = (curve(1_456_008_907.0, 0.0, 101, depth, 4) / 10_000.0).trunc() * factor;
    (base, sweep)
}

const MAX_BASE: f32 = 0.0441 * (3800.0 + 3840.0);

pub(super) struct Ensemble {
    sample_rate: f32,
    scale: f32,
    line: DelayLine,
    hp: FirstOrder,
    lp: FirstOrder,
    phase: u32,
    increment: u32,
    base: TapTime,
    sweep: TimedRamp,
    wet: TimedRamp,
    level: Ramp,
    last: Values,
}

impl Ensemble {
    pub(super) fn new(info: &ModelInfo, sample_rate: f32) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        let mut effect = Self {
            sample_rate,
            scale,
            line: DelayLine::new(((MAX_BASE + 512.0) * scale).ceil() as usize + 8),
            hp: FirstOrder::default(),
            lp: FirstOrder::default(),
            phase: 0,
            increment: 0,
            base: TapTime::new(0.0, 0.06, 400.0 * scale, sample_rate),
            sweep: TimedRamp::new(0.0, sample_rate),
            wet: TimedRamp::new(0.0, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            last: UNSET,
        };
        effect.apply(&defaults(info), info);
        effect.base.settle();
        effect.sweep.settle();
        effect.wet.settle();
        effect.level.jump(effect.level.target);
        effect
    }

    pub(super) fn apply(&mut self, values: &Values, info: &ModelInfo) {
        let v = sanitize(values, info);
        let last = self.last;
        if v[0] != last[0] || v[1] != last[1] {
            let (base, sweep) = depth_words(v[0], v[1]);
            self.base.set(base * self.scale);
            self.sweep.set(sweep * Q31 * 512.0 * self.scale);
        }
        if v[1] != last[1] {
            self.increment = increment(curve(894_784.0, 8947.0, 50, v[1], 6), self.sample_rate);
        }
        if v[2] != last[2] {
            self.wet.set(curve(1.0, 0.0, 101, v[2], 0));
        }
        if step(v[3], 10) != step(last[3], 10) || last[3].is_nan() {
            let row = &TONE[step(v[3], 10)];
            self.hp.set_rom(tone_hp(row), self.sample_rate);
            self.lp.set_rom(tone_lp(row), self.sample_rate);
        }
        if v[4] != last[4] {
            self.level.set(curve(1.5, 0.0, 151, v[4], 0));
        }
        self.last = v;
    }

    #[inline]
    pub(super) fn tick(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let average = 0.5 * (left + right);
        self.line.write(self.lp.process(self.hp.process(average)));
        self.phase = self.phase.wrapping_add(self.increment);
        let w = triangle(self.phase) - 0.5;
        let lfo = 2.0 * w * (1.0 - w.abs()) + 0.5;
        let offset = self.sweep.tick() * lfo;
        let fade = self.base.tick();
        let mut tap = self.line.read_linear(self.base.current + offset);
        if fade > 0.0 {
            tap += (self.line.read_linear(self.base.previous + offset) - tap) * fade;
        }
        let wet = self.wet.tick() * tap;
        let level = self.level.tick();
        [level * (left + wet), level * (right - wet)]
    }

    pub(super) fn reset(&mut self) {
        self.line.clear();
        self.hp.clear();
        self.lp.clear();
    }
}

struct Model(Ensemble);

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Model(Ensemble::new(&INFO, sample_rate)))
}

impl Effect for Model {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.0.apply(values, &INFO);
    }
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.0.tick(frame)
    }
    fn clear(&mut self) {
        self.0.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::chorus_std::common::increment_hz;

    #[test]
    fn editors_follow_the_original_curves() {
        let (base, sweep) = depth_words(40.0, 29.0);
        assert_eq!(base, 256.0);
        assert!(
            (sweep * Q31 * 512.0 - 64.2).abs() < 0.5,
            "{}",
            sweep * Q31 * 512.0
        );
        let (base, sweep) = depth_words(100.0, 0.0);
        assert_eq!(base, (0.0441f32 * 3840.0).round());
        assert!((sweep * Q31 * 512.0 - 347.1).abs() < 0.5);
        let effect = Ensemble::new(&INFO, 48_000.0);
        assert!((increment_hz(effect.increment, 48_000.0) - 1.98).abs() < 0.02);
    }

    #[test]
    fn wet_is_in_opposite_phase() {
        let mut effect = Ensemble::new(&INFO, 48_000.0);
        let mut values = defaults(&INFO);
        values[2] = 100.0;
        effect.apply(&values, &INFO);
        let mut wet = 0.0f32;
        for i in 0..24_000 {
            let x = (i as f32 * 220.0 / 48_000.0 * std::f32::consts::TAU).sin() * 0.5;
            let [l, r] = effect.tick([x, x]);
            // The wet parts cancel in the sum and add in the difference.
            assert!((l + r - 2.0 * x).abs() < 1e-5);
            wet = wet.max((l - r).abs());
        }
        assert!(wet > 0.3, "{wet}");
    }
}
