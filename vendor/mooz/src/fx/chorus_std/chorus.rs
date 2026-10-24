//! Chorus (CHORUS.ZDL): the shared modulated-delay callback (see
//! [`super::modulated`]) as a mono chorus with the stereo dry kept.
//!
//! Editors decoded from the disassembly:
//! - Depth: c22 = curve(264, 423, 101, atan) samples and c23 =
//!   curve(0x4b423000, 0, 101, atan) Q31 (c24 = 22: 0..301 samples of sweep),
//!   so deeper settings shorten the base delay. The original mutes the wet
//!   tap around the change; here the delay glides (see `TapTime`).
//! - Rate: c15 = curve(545392, 9739, 50, square): 0.1..5.6 Hz triangle, then
//!   the ROM one-pole c20/c21 (≈4 Hz) rounds it.
//! - Mix: c29 = Mix/100 by the 16 ms timed setter (wet added twice, dry at 1).
//! - Tone: memcpy of the shared tone table row into c7..c12.
//! - Level: c2 = Level/100 by the stepped float setter.
use super::{
    common::{TONE, UNSET, defaults, sanitize, step, tone_hp, tone_lp},
    modulated::{Core, Layout},
};
use crate::{Effect, Values, catalog::*, zoom::curve};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_chorus",
    name: "Chorus",
    category: Category::Modulation,
    zdl: "CHORUS.ZDL",
    description: "Mono chorus over the stereo dry signal, with tone",
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
            name: "Tone",
            max: 10,
            default: 7,
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

/// Shared by Chorus and Ba Chorus.
pub(super) fn depth_words(depth: f32) -> (f32, f32) {
    (
        curve(264.0, 423.0, 101, depth, 4),
        curve(1_262_628_864.0, 0.0, 101, depth, 4),
    )
}

/// Rate editor of Chorus and Ba Chorus (c15).
pub(super) fn rate_word(rate: f32) -> f32 {
    curve(545_392.0, 9739.0, 50, rate, 1)
}

const LAYOUT: Layout = Layout {
    max_base: 423.0,
    span: 512.0,
    smoothing: (1_226_417, 2_146_257_231),
    offset: 0,
    jump: 400.0,
    timed_wet: true,
};

struct Chorus {
    core: Core,
    last: Values,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = Chorus {
        core: Core::new(LAYOUT, sample_rate),
        last: UNSET,
    };
    effect.set(&defaults(&INFO), 120.0);
    effect.core.settle();
    Box::new(effect)
}

impl Effect for Chorus {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let v = sanitize(values, &INFO);
        let last = self.last;
        if v[0] != last[0] {
            let (base, depth) = depth_words(v[0]);
            self.core.set_base(base);
            self.core.set_depth(depth);
        }
        if v[1] != last[1] {
            self.core.set_increment_44k(rate_word(v[1]));
        }
        if v[2] != last[2] {
            self.core.set_wet(curve(1.0, 0.0, 101, v[2], 0));
        }
        if step(v[3], 10) != step(last[3], 10) || last[3].is_nan() {
            let row = &TONE[step(v[3], 10)];
            self.core.set_filters(tone_hp(row), tone_lp(row));
        }
        if v[4] != last[4] {
            self.core.set_level(curve(1.5, 0.0, 151, v[4], 0));
        }
        self.last = v;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }

    fn clear(&mut self) {
        self.core.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::chorus_std::common::increment_hz;

    #[test]
    fn depth_moves_base_and_sweep_like_the_editor() {
        let (base, depth) = depth_words(0.0);
        assert_eq!((base, depth), (423.0, 0.0));
        let (base, depth) = depth_words(100.0);
        assert!((base - 264.0).abs() < 1e-3);
        assert!((depth * crate::fx::chorus_std::common::Q31 * 512.0 - 301.0).abs() < 1.0);
    }

    #[test]
    fn rate_spans_a_tenth_to_five_point_six_hz() {
        let mut effect = Chorus {
            core: Core::new(LAYOUT, 48_000.0),
            last: UNSET,
        };
        let mut values = defaults(&INFO);
        values[1] = 0.0;
        effect.set(&values, 120.0);
        assert!((increment_hz(effect.core.increment(), 48_000.0) - 0.1).abs() < 0.001);
        values[1] = 49.0;
        effect.set(&values, 120.0);
        assert!((increment_hz(effect.core.increment(), 48_000.0) - 5.6).abs() < 0.01);
    }

    #[test]
    fn defaults_are_near_unity_and_bounded() {
        let mut effect = create(48_000.0);
        let mut peak = 0.0f32;
        let mut energy_in = 0.0;
        let mut energy_out = 0.0;
        for i in 0..96_000 {
            let x = (i as f32 * 440.0 / 48_000.0 * std::f32::consts::TAU).sin() * 0.5;
            let [l, r] = effect.process([x, x]);
            peak = peak.max(l.abs()).max(r.abs());
            if i > 4800 {
                energy_in += x * x;
                energy_out += l * l;
            }
        }
        // Dry 1 + wet 2 * 0.5: between unity and +6 dB.
        let ratio = (energy_out / energy_in).sqrt();
        assert!(ratio > 0.9 && ratio < 2.1, "{ratio}");
        assert!(peak < 2.0);
    }
}
