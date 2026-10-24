//! ModDelay2 (MODDLY2.ZDL): ModDelay (see [`super::mod_delay`]) with a Depth
//! knob. Its editor (not run by init, the ROM word equals Depth 50) sets c23 =
//! curve(0x27d11f8c, 0, 51, linear) below 50 and curve(0x4b423000,
//! 0x27d11f8c, 51, Depth - 50, linear) above, through the integer ramp:
//! 0..159 then 159..301 samples of sweep.
use super::{
    common::{UNSET, defaults, sanitize},
    mod_delay::{DEPTH, apply, new_core},
    modulated::Core,
};
use crate::{Effect, Values, catalog::*, zoom::curve};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_mod_delay2",
    name: "ModDelay2",
    category: Category::Delay,
    zdl: "MODDLY2.ZDL",
    description: "Modulated delay up to 2 s with adjustable depth",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 2014,
            default: 529,
            display: Display::DelayTime {
                threshold: 1999,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 45,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 16,
            display: Display::offset(1.0),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 50,
            display: Display::INT,
        },
    ],
};

fn depth_word(depth: f32) -> f32 {
    if depth < 50.0 {
        curve(DEPTH, 0.0, 51, depth, 0)
    } else {
        curve(1_262_628_864.0, DEPTH, 51, depth - 50.0, 0)
    }
}

struct ModDelay2 {
    core: Core,
    last: Values,
    bpm: f32,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = ModDelay2 {
        core: new_core(sample_rate),
        last: UNSET,
        bpm: f32::NAN,
    };
    effect.set(&defaults(&INFO), 120.0);
    effect.core.settle();
    Box::new(effect)
}

impl Effect for ModDelay2 {
    fn set(&mut self, values: &Values, bpm: f32) {
        let v = sanitize(values, &INFO);
        apply(&mut self.core, &v, &self.last, bpm, self.bpm);
        if v[5] != self.last[5] {
            self.core.set_depth(depth_word(v[5]));
        }
        self.last = v;
        self.bpm = bpm;
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
    fn depth_is_continuous_and_matches_the_rom_at_50() {
        assert_eq!(depth_word(50.0), DEPTH);
        assert!((depth_word(49.999) - DEPTH).abs() / DEPTH < 1e-4);
        assert_eq!(depth_word(0.0), 0.0);
        assert!((depth_word(100.0) * Q31 * 512.0 - 301.0).abs() < 1.0);
    }
}
