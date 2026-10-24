//! LineSel (LINESEL.ZDL, `Fx_FLT_LineSel` 0x7800..0x7c80) with its decoded
//! editors.
//!
//! The callback splits every sample in two: the effect buffer that feeds the
//! following slots gets `x c5 c0 c4` and the output accumulator (which Main
//! adds after the last slot) gets `x c6 (1 - c0) c4`. `EfxLvl_edit` and
//! `OutLvl_edit` set c5 and c6 to curve(1.5, 0, 151, v) = `v / 100` through
//! `context[7]` (a 16 ms glide), `onf` glides c0 to 1 (on) or 0 (off), and c4
//! is Main's master fader (1 in normal use).
//!
//! A SAD PET slot has no "rest of the chain" to bypass, so its output is the
//! sum the pedal's mixer would hear right after LineSel with the switch ON:
//! `x (EFX_L c0 + OUT_L (1 - c0))` with c0 = 1, i.e. the send level EFX_L.
//! OUT_L is the level of the pedal's OFF path (signal sent straight to the
//! output while later effects are muted); it is kept in the formula but has
//! no audible effect while the slot is on.
use super::common::{self, Glide};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_line_sel",
    name: "LineSel",
    category: Category::Utility,
    zdl: "LINESEL.ZDL",
    description: "Send level stage; OUT_L is the pedal's switched-off level",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "EFX_L",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "OUT_L",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// c0 of the switched-on pedal.
const ON: f32 = 1.0;

struct LineSel {
    effect_level: Glide,
    output_level: Glide,
    primed: bool,
}

impl Effect for LineSel {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.effect_level.set(common::level(values[0]));
        self.output_level.set(common::level(values[1]));
        if !self.primed {
            self.primed = true;
            self.effect_level.snap();
            self.output_level.snap();
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let gain = self.effect_level.tick() * ON + self.output_level.tick() * (1.0 - ON);
        [left * gain, right * gain]
    }

    fn clear(&mut self) {
        self.effect_level.snap();
        self.output_level.snap();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(LineSel {
        effect_level: Glide::new(1.0, sample_rate),
        output_level: Glide::new(1.0, sample_rate),
        primed: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_level_is_percent_and_glides() {
        let mut effect = create(48_000.0);
        let mut values = [0.0; crate::MAX_PARAMS];
        values[0] = 100.0;
        values[1] = 100.0;
        effect.set(&values, 120.0);
        assert_eq!(effect.process([0.5, -0.5]), [0.5, -0.5]);
        values[0] = 50.0;
        values[1] = 0.0;
        effect.set(&values, 120.0);
        let first = effect.process([1.0, 1.0])[0];
        assert!(first > 0.99, "{first}");
        for _ in 0..800 {
            effect.process([1.0, 1.0]);
        }
        let settled = effect.process([1.0, 1.0])[0];
        assert!((settled - 0.5).abs() < 1e-6, "{settled}");
    }
}
