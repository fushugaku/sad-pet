//! StereoCho (STCHO.ZDL): the shared modulated-delay callback (see
//! [`super::modulated`]) with the second tap's LFO at 180° (c16 =
//! 0x80000000), so the two outputs sweep in antiphase. No LFO smoothing
//! (c20 = 1, c21 = 0), fixed base delay c22 = 798 samples (18.1 ms) and a
//! 256-sample span (c24 = 23).
//!
//! Editors decoded from the disassembly:
//! - Depth (also run by Rate): c23 = 100 * (curve(0x3cc09999, 0x1ae25b33, 101,
//!   linear) / curve(184, 100, 50, Rate, atan)), set by the integer ramp
//!   (0x3deb7 per sample): faster rates get less sweep.
//! - Rate: c15 = curve(0x4400b, 0x4848, 50, square): 0.19..2.9 Hz.
//! - Mix: c29 = Mix/100 by the stepped setter (wet added twice).
//! - Tone: the shared tone table; Level: c2 = Level/100.
use super::{
    common::{TONE, UNSET, defaults, sanitize, step, tone_hp, tone_lp},
    modulated::{Core, Layout},
};
use crate::{Effect, Values, catalog::*, zoom::curve};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_stereo_cho",
    name: "StereoCho",
    category: Category::Modulation,
    zdl: "STCHO.ZDL",
    description: "Stereo chorus: two taps swept in antiphase",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 80,
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

const LAYOUT: Layout = Layout {
    max_base: 798.0,
    span: 256.0,
    smoothing: (0x7fff_ffff, 0),
    offset: 0x8000_0000,
    jump: 400.0,
    timed_wet: false,
};

/// c23 from Depth and Rate (integer division as the editor does).
fn depth_word(depth: f32, rate: f32) -> f32 {
    let depth = curve(1_019_255_193.0, 451_042_099.0, 101, depth, 0).trunc();
    let divisor = curve(184.0, 100.0, 50, rate, 4).round().max(1.0);
    100.0 * (depth / divisor).trunc()
}

struct StereoCho {
    core: Core,
    last: Values,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = StereoCho {
        core: Core::new(LAYOUT, sample_rate),
        last: UNSET,
    };
    effect.core.set_base(798.0);
    effect.set(&defaults(&INFO), 120.0);
    effect.core.settle();
    Box::new(effect)
}

impl Effect for StereoCho {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let v = sanitize(values, &INFO);
        let last = self.last;
        if v[0] != last[0] || v[1] != last[1] {
            self.core.set_depth(depth_word(v[0], v[1]));
        }
        if v[1] != last[1] {
            self.core
                .set_increment_44k(curve(278_539.0, 18_504.0, 50, v[1], 1));
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
    use crate::fx::chorus_std::common::Q31;

    #[test]
    fn depth_shrinks_with_rate() {
        let slow = depth_word(100.0, 0.0) * Q31 * 256.0;
        let fast = depth_word(100.0, 49.0) * Q31 * 256.0;
        assert!((slow - 121.5).abs() < 0.5, "{slow}");
        assert!((fast / slow - 100.0 / 184.0).abs() < 0.01);
    }

    #[test]
    fn outputs_sweep_in_antiphase() {
        let mut effect = create(48_000.0);
        let mut difference = 0.0f32;
        for i in 0..48_000 {
            let x = (i as f32 * 330.0 / 48_000.0 * std::f32::consts::TAU).sin() * 0.5;
            let [l, r] = effect.process([x, x]);
            difference = difference.max((l - r).abs());
        }
        assert!(difference > 0.1, "{difference}");
    }
}
