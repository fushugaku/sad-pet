//! BaAutoWah (B_ATWAH.ZDL): bass envelope wah with a dry blend.
//!
//! Callback `Fx_FLT_BaAutoWah` (`sweep_core`, linear envelope, peak fed back
//! into the second detector section as the firmware does). ROM block at
//! 0x80000870. Editors decoded from the disassembly:
//! - Sense (`sense_edit`): group = Sense / 10; c22 = 0x800008d8[Sense]
//!   (0x3af7c5ec ramp); on a group change the band output c8 is ramped to 0,
//!   0x80000928 + 28 group is copied (c23, c24/c7, c18, c19, c20, c21), Reso
//!   is re-run and c8 ramps to the table's seventh word (0.70 down, 0.53 up).
//! - Reso (`reso_edit`): c5 (input gain) and c6 (minus damping / 4) from
//!   0x80000720 + 88 group + 8 Reso, while c2 is dipped and restored.
//! - Dry (`drymix_edit`): c10 = 0.25 Dry / 100. Level: c2 = Level / 100.
use super::common::{SENSE_LABELS, STEP_FAST, clamp_value, step, w};
use super::sweep_core::{Direction, Rom, Settings, SweepCore};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_auto_wah",
    name: "BaAutoWah",
    category: Category::Filter,
    zdl: "B_ATWAH.ZDL",
    description: "Bass auto-wah with dry blend",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Sense",
            max: 19,
            default: 11,
            display: Display::labels(&SENSE_LABELS),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Dry",
            max: 100,
            default: 0,
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

const ROM: Rom = Rom {
    first: [w(0x4020_4487), w(0xc01f_cfc8), w(0x3f7d_b41a)],
    second: [w(0x3f82_ac16), w(0xbf6f_ea77), w(0x3f75_42a3)],
    c17: w(0x3f7f_ba57),
    c25: 4.0,
    squared: false,
    peak_feedback: true,
    sense_step: STEP_FAST,
};

/// 0x80000928: c23, c24, c18, c19, c20, c21, c8 per direction.
const DIRECTIONS: [Direction; 2] = [
    Direction {
        c23: w(0xbcaf_1307),
        c24: w(0x3ce9_6eb5),
        c18: w(0x398b_3333),
        c19: w(0x3f7f_ea40),
        c20: 1.0,
        c21: 0.0,
        c8: w(0x3f33_3200),
        c9: 0.0,
    },
    Direction {
        c23: w(0x3d8b_547a),
        c24: w(0x3b52_16d3),
        c18: w(0x3a0b_671d),
        c19: w(0x3f7f_d8cb),
        c20: w(0x3f8d_2348),
        c21: w(0x3e33_b040),
        c8: w(0x3f08_8800),
        c9: 0.0,
    },
];

/// 0x800008d8: c22 per Sense value.
const SENSITIVITY: [u32; 20] = [
    0x3efe2c00, 0x3ed75800, 0x3eb8fc00, 0x3e9ee800, 0x3e888000, 0x3e6a8000, 0x3e497000, 0x3e2d0800,
    0x3e14a000, 0x3dff6000, 0x3db4c000, 0x3dd52000, 0x3dfb4000, 0x3e142000, 0x3e2ea000, 0x3e4de800,
    0x3e72c000, 0x3e8f1800, 0x3ea8b400, 0x3ec9e800,
];

/// 0x80000720: (c5, c6) per `group * 11 + Reso`.
const RESONANCE: [u32; 44] = [
    0x3de4b3b6, 0xbe9b2800, 0x3de4b3b6, 0xbe91ff75, 0x3dcbd4b4, 0xbe78115d, 0x3db5aa1a, 0xbe46567b,
    0x3d904d1c, 0xbe1adf36, 0x3d809bcc, 0xbdf1a771, 0x3d809bcc, 0xbdbe7a32, 0x3d653ebb, 0xbd9858f8,
    0x3d653ebb, 0xbd7785dd, 0x3d4c509b, 0xbd4c1dcd, 0x3d361887, 0xbd2aaaab, 0x3e21866c, 0xbe800000,
    0x3e187d50, 0xbe71826a, 0x3e0ff59a, 0xbe4e739d, 0x3e07e80b, 0xbe263bd8, 0x3e004dce, 0xbe029cbc,
    0x3df240e2, 0xbdcccccd, 0x3de4b3b6, 0xbda2067b, 0x3dd7e89b, 0xbd81f301, 0x3dcbd4b4, 0xbd5391fc,
    0x3dc06dc3, 0xbd2ebe78, 0x3db5aa1a, 0xbd124925,
];

struct BaAutoWah {
    core: SweepCore,
    last: Option<(usize, usize, f32, f32)>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = BaAutoWah {
        core: SweepCore::new(sample_rate, &ROM, &DIRECTIONS),
        last: None,
    };
    let mut values = [0.0; crate::MAX_PARAMS];
    for (value, param) in values.iter_mut().zip(INFO.params) {
        *value = f32::from(param.default);
    }
    effect.set(&values, 120.0);
    Box::new(effect)
}

fn settings(sense: usize, reso: usize) -> Settings {
    let group = sense / 10;
    let at = (group * 11 + reso) * 2;
    Settings {
        group,
        c5: f32::from_bits(RESONANCE[at]),
        c6: f32::from_bits(RESONANCE[at + 1]),
        c22: f32::from_bits(SENSITIVITY[sense]),
    }
}

impl Effect for BaAutoWah {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let sense = step(values[0], 19);
        let reso = step(values[1], 10);
        let dry = clamp_value(values[2], 100.0) / 100.0;
        let level = clamp_value(values[3], 150.0) / 100.0;
        if self.last == Some((sense, reso, dry, level)) {
            return;
        }
        self.last = Some((sense, reso, dry, level));
        self.core.configure(settings(sense, reso));
        self.core.set_mix(dry, level);
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }

    fn clear(&mut self) {
        self.core.clear();
        self.last = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn gain(effect: &mut dyn Effect, hz: f32, amplitude: f32, sample_rate: f32) -> f32 {
        let frames = sample_rate as usize;
        let mut peak = 0.0f32;
        for i in 0..frames {
            let x = amplitude * (std::f32::consts::TAU * hz * i as f32 / sample_rate).sin();
            let y = effect.process([x, x])[0];
            if i > frames / 2 {
                peak = peak.max(y.abs());
            }
        }
        peak / amplitude
    }

    #[test]
    fn dry_knob_passes_the_input() {
        let mut values = Model::BaAutoWah.defaults();
        values[1] = 0.0;
        values[2] = 100.0;
        let mut with_dry = create(48_000.0);
        with_dry.set(&values, 120.0);
        values[2] = 0.0;
        let mut without = create(48_000.0);
        without.set(&values, 120.0);
        // Far above the band the filter is quiet and the dry path dominates.
        let a = gain(with_dry.as_mut(), 12_000.0, 1e-3, 48_000.0);
        let b = gain(without.as_mut(), 12_000.0, 1e-3, 48_000.0);
        assert!((a - b - 1.0).abs() < 0.05 && b < 0.2, "{a} {b}");
    }

    #[test]
    fn quiet_input_sits_at_the_bottom_of_the_up_sweep() {
        // Sense +2 with a tiny signal: f = 4 c24 = 0.0128, about 90 Hz.
        let mut low = create(44_100.0);
        let mut high = create(44_100.0);
        let a = gain(low.as_mut(), 90.0, 1e-5, 44_100.0);
        let b = gain(high.as_mut(), 1000.0, 1e-5, 44_100.0);
        assert!(a > 10.0 * b, "{a} {b}");
    }

    #[test]
    fn tables_follow_the_editor_indexing() {
        assert_eq!(settings(11, 8).c22, w(0x3dd5_2000));
        assert_eq!(settings(3, 10).c6, w(0xbd2a_aaab));
        assert_eq!(settings(19, 0).c5, w(0x3e21_866c));
    }
}
