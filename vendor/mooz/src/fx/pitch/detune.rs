//! Detune (DETUNE.ZDL): one slightly detuned voice mixed with the dry signal.
//!
//! Callback: the recovered DETUNE/PTCHSHFT graph ([`super::two_tap`]) with the
//! DETUNE ROM (c8 = 0x5a5119cc, c11 = 21: a 722.5-sample sweep, c25 = 1 so
//! the taps are always heard, c16 = 1 dry). Editors decoded:
//! - `cent_edit`: c5 from the 51-word table at 0x800004d0, i.e. -25..+25 cents
//!   (reproduced by formula); the increment changes in place.
//! - `predly_edit`: c9 = 441·PreD/10 + 2 samples (0..50 ms), wrapped in the
//!   16 ms c3 mute/fade-in.
//! - `mix_edit`: c15 = two-slope(Mix) through the 16 ms `context[7]` ramp.
//! - `tone_edit`: c19/c20 input one-pole; `outLv_edit`: c2 = Level / 100.
use super::{
    common::{FADE_SECONDS, TimedRamp, one_pole, out_level, step, two_slope},
    rom::{TWO_TAP_TONE, f},
    two_tap::{Geometry, TwoTapShifter},
};
use crate::{
    Effect, Values,
    catalog::*,
    zoom::{DEFAULT_STEP, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_detune",
    name: "Detune",
    category: Category::Pitch,
    zdl: "DETUNE.ZDL",
    description: "Detuned voice (±25 cents) with pre-delay",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Cent",
            max: 50,
            default: 35,
            display: Display::offset(-25.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PreD",
            max: 50,
            default: 0,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 52,
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

/// DETUNE/B_DETUNE sweep: `0x5a5119cc >> 21` samples at 44.1 kHz.
pub(super) const DETUNE_DEPTH: f32 = 1_515_264_460.0 / 2_097_152.0;

/// The detune taps for `cents` and a PreD step (`predly_edit`).
pub(super) fn detune_geometry(cents: f32, pre_delay: usize, sample_rate: f32) -> Geometry {
    let base = (441 * pre_delay / 10 + 2) as f32;
    Geometry::new(
        (cents / 1200.0).exp2(),
        base,
        DETUNE_DEPTH,
        sample_rate,
        true,
    )
}

/// Longest distance: 2 + 2205 + the sweep.
pub(super) const DETUNE_LONGEST: f32 = 2_207.0 + DETUNE_DEPTH + 2.0;

struct Detune {
    sample_rate: f32,
    shifter: TwoTapShifter,
    pre_delay: Option<usize>,
    wet: TimedRamp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Detune {
        sample_rate,
        shifter: TwoTapShifter::new(sample_rate, DETUNE_LONGEST),
        pre_delay: None,
        wet: TimedRamp::new(two_slope(52.0), FADE_SECONDS, sample_rate),
        level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
    })
}

impl Effect for Detune {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let pre_delay = step(values[1], 50);
        let cents = step(values[0], 50) as f32 - 25.0;
        let geometry = detune_geometry(cents, pre_delay, self.sample_rate);
        let fade = self.pre_delay.is_some_and(|old| old != pre_delay);
        self.shifter.request(geometry, fade);
        self.pre_delay = Some(pre_delay);
        let tone = f(TWO_TAP_TONE[step(values[3], 10)]);
        self.shifter.set_tone(one_pole(tone, self.sample_rate));
        self.wet.set(two_slope(values[2]));
        self.level.set(out_level(values[4]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let wet = self.shifter.process(0.5 * (left + right)) * self.wet.tick();
        let level = self.level.tick();
        [level * (left + wet), level * (right + wet)]
    }

    fn clear(&mut self) {
        self.shifter.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::pitch::two_tap::tests::centroid;

    #[test]
    fn detunes_by_the_cent_value() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        let mut values = crate::Model::Detune.defaults();
        values[0] = 50.0; // +25 cents
        values[3] = 10.0;
        effect.set(&values, 120.0);
        let n = 4 * sample_rate as usize;
        let out: Vec<f32> = (0..n)
            .map(|i| {
                let x = 0.5 * (i as f32 * 1_000.0 / sample_rate * std::f32::consts::TAU).sin();
                // Subtract the dry path to look at the voice alone.
                effect.process([x, x])[0] - x
            })
            .collect();
        let found = centroid(&out[n / 4..], sample_rate, 1_004.5, 1_024.5);
        let expected = 1_000.0 * (25.0f32 / 1200.0).exp2();
        assert!((found - expected).abs() < 1.0, "{found} vs {expected}");
    }

    #[test]
    fn pre_delay_sets_the_base_distance() {
        let geometry = detune_geometry(0.0, 50, 44_100.0);
        assert_eq!(geometry.base, 2_207.0);
        assert_eq!(geometry.increment, 0);
        let geometry = detune_geometry(0.0, 10, 96_000.0);
        assert!((geometry.base - 443.0 * 96_000.0 / 44_100.0).abs() < 1e-2);
    }
}
