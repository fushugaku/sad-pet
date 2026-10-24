//! Exciter (EXCITER.ZDL), after the recovered `exciter.rs` callback.
//!
//! Per channel, a state-variable filter of clipped integrators:
//! `hp = clip(c5·x − c7·bp − lp)`, `bp = clip(bp + c6·hp)`,
//! `lp = clip(lp + 1.6·c6·bp)` (c5 0.72, c6 ≈ 2 sin(π·350/44.1k), damping
//! c7 = 6, i.e. corners near 100 Hz and 2 kHz), mixed as
//! `clip(c8·hp + c9·bp + c10·lp)` with c9 = 8, then ×0.9 (c16) and Level (c2).
//! The callback's following biquad (c11..15) is identity in ROM and never
//! edited, so it is omitted. The clipped integrators add the harmonics.
//!
//! Editors decoded: Trebl (`process_edit`) sets the high-pass weight
//! `c8 = curve(6, 1.8875, 101, v, square)`, Bass (`loContour_edit`) the low-pass
//! weight `c10 = curve(10, 3.6448, 101, v, square)`, both ramped by 0.00189 per
//! 44.1 kHz sample; Level is `v/100`. The integrator gain c6 is recomputed for
//! the host rate.
use super::common::{clip1, level};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_exciter",
    name: "Exciter",
    category: Category::Eq,
    zdl: "EXCITER.ZDL",
    description: "Saturating three-band contour with bass and treble",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Bass",
            max: 100,
            default: 0,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Trebl",
            max: 100,
            default: 0,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// ROM 0x80000414: c5, c6, c7, c9, c16.
const INPUT: f32 = 0.72;
const INTEGRATOR: f32 = 0.049_861_4;
const DAMPING: f32 = 6.0;
const BAND_WEIGHT: f32 = 8.0;
const OUTPUT: f32 = 0.9;
/// The editors' ramp step 0x3af7c5ec.
const STEP: f32 = 1.890_359e-3;

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Exciter::new(sample_rate))
}

pub struct Exciter {
    integrator: f32,
    high: Ramp,
    low: Ramp,
    level: Ramp,
    state: [[f32; 2]; 2],
}

impl Exciter {
    fn new(sample_rate: f32) -> Self {
        // c6 as a Chamberlin frequency 2 sin(π f / fs), f ≈ 350 Hz.
        let hz = (INTEGRATOR * 0.5).asin() * ORIGINAL_RATE / std::f32::consts::PI;
        let mut effect = Self {
            integrator: 2.0 * (std::f32::consts::PI * hz / sample_rate).sin(),
            high: Ramp::new(0.0, STEP, sample_rate),
            low: Ramp::new(0.0, STEP, sample_rate),
            level: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            state: [[0.0; 2]; 2],
        };
        effect.set(&crate::Model::Exciter.defaults(), 120.0);
        for ramp in [&mut effect.high, &mut effect.low, &mut effect.level] {
            ramp.jump(ramp.target);
        }
        effect
    }

    #[inline]
    fn channel(&mut self, channel: usize, x: f32, high: f32, low: f32) -> f32 {
        let [bp, lp] = self.state[channel];
        let hp = clip1(INPUT * x - DAMPING * bp - lp);
        let bp = clip1(bp + self.integrator * hp);
        let lp = clip1(lp + 1.6 * self.integrator * bp);
        self.state[channel] = [bp, lp];
        clip1(low * lp + BAND_WEIGHT * bp + high * hp)
    }
}

impl Effect for Exciter {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.low
            .set(curve(10.0, 3.644_8, 101, values[0].clamp(0.0, 100.0), 1));
        self.high
            .set(curve(6.0, 1.887_5, 101, values[1].clamp(0.0, 100.0), 1));
        self.level.set(level(values[2]));
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let high = self.high.tick();
        let low = self.low.tick();
        let gain = OUTPUT * self.level.tick();
        [
            self.channel(0, left, high, low) * gain,
            self.channel(1, right, high, low) * gain,
        ]
    }

    fn clear(&mut self) {
        self.state = [[0.0; 2]; 2];
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::test_util::{sine_gain, values};
    use super::*;

    #[test]
    fn integrator_keeps_its_frequency() {
        let at44 = Exciter::new(44_100.0);
        assert!((at44.integrator - INTEGRATOR).abs() < 1e-6);
        let at96 = Exciter::new(96_000.0);
        assert!((at96.integrator - INTEGRATOR * 44_100.0 / 96_000.0).abs() < 1e-5);
    }

    #[test]
    fn bass_and_treble_shape_the_ends() {
        let fs = 48_000.0;
        let measure = |bass: f32, treble: f32, hz: f32| {
            let mut effect = Exciter::new(fs);
            effect.set(&values(&[bass, treble, 100.0]), 120.0);
            effect.low.jump(effect.low.target);
            effect.high.jump(effect.high.target);
            sine_gain(&mut effect, fs, hz, 0.05)
        };
        // DC-ish: lp settles at c5·x, weighted by c10.
        let low = measure(0.0, 0.0, 30.0);
        assert!((low - 0.9 * 0.72 * 3.6448).abs() < 0.4, "{low}");
        assert!(measure(100.0, 0.0, 30.0) > 1.8 * low);
        let high = measure(0.0, 0.0, 15_000.0);
        assert!(measure(0.0, 100.0, 15_000.0) > 2.0 * high);
    }

    #[test]
    fn stays_bounded_when_driven() {
        let mut effect = Exciter::new(48_000.0);
        effect.set(&values(&[100.0, 100.0, 150.0]), 120.0);
        for i in 0..48_000 {
            let x = 4.0 * (i as f32 * 0.05).sin();
            let [l, r] = effect.process([x, -x]);
            assert!(l.abs() <= 1.35 + 1e-6 && r.abs() <= 1.35 + 1e-6);
        }
    }
}
