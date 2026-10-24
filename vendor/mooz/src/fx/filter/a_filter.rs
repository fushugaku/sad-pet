//! A-Filter (A_FILTER.ZDL): envelope-swept resonant low-pass, up or down.
//!
//! Callback `Fx_FLT_A_Filter` (`sweep_core`, squared envelope). Its ROM block
//! (0x80000510) makes both detector sections plain wires, so the envelope
//! follows `|m|` directly. Editors decoded from the disassembly:
//! - Sense/Mode (`sense_edit`, also the Mode editor): index = 10 + Sense for
//!   Up, 9 - Sense for Down; group = index / 10; c22 = 0x80000578[index]
//!   (default ramp). A group change ramps c9 to 0, copies 0x800005c8 +
//!   24 group (c18, c19, c21, c23, c24/c7), re-runs Peak and ramps c9 to the
//!   sixth word (0.75: the low-pass output).
//! - Peak (`reso_edit`): c5/c6 from 0x80000460 + 8 (Peak + 11 group).
//! - Dry: c10 = 0.25 Dry / 100. Level: c2 = Level / 100.
use super::common::{clamp_value, step, w};
use super::sweep_core::{Direction, Rom, Settings, SweepCore};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_a_filter",
    name: "A-Filter",
    category: Category::Filter,
    zdl: "A_FILTER.ZDL",
    description: "Resonant auto filter sweeping up or down",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Sense",
            max: 9,
            default: 7,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Peak",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: Display::labels(&["Up", "Down"]),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Dry",
            max: 100,
            default: 10,
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

const ROM: Rom = Rom {
    first: [1.0, 0.0, 0.0],
    second: [1.0, 0.0, 0.0],
    c17: w(0x3f7f_bfdf),
    c25: 2.0,
    squared: true,
    peak_feedback: false,
    sense_step: crate::zoom::DEFAULT_STEP,
};

/// 0x800005c8: c18, c19, c21, c23, c24, c9 per group (c20 = 1, c8 = 0).
const DIRECTIONS: [Direction; 2] = [
    Direction {
        c18: w(0x3a47_480d),
        c19: w(0x3f7f_ce2e),
        c20: 1.0,
        c21: w(0x3f71_6b13),
        c23: w(0xbd2f_1307),
        c24: w(0x3d5a_d7c9),
        c8: 0.0,
        c9: 0.75,
    },
    Direction {
        c18: w(0x3b04_dd10),
        c19: w(0x3f7f_7b23),
        c20: 1.0,
        c21: w(0x3f65_7a56),
        c23: w(0x3d8c_0f39),
        c24: w(0x3b3a_bef7),
        c8: 0.0,
        c9: 0.75,
    },
];

/// 0x80000578: c22 per index.
const SENSITIVITY: [u32; 20] = [
    0x3efe2c00, 0x3eaca400, 0x3e723000, 0x3e29e000, 0x3dee5000, 0x3da73000, 0x3d6a8000, 0x3d248000,
    0x3ce6c000, 0x3ca1c000, 0x3ca1c000, 0x3ce6c000, 0x3d248000, 0x3d6a8000, 0x3da73000, 0x3dee5000,
    0x3e29e000, 0x3e723000, 0x3eaca400, 0x3efe2c00,
];

/// 0x80000460: (c5, c6) per `group * 11 + Peak` (Z_TRON.ZDL has the same
/// table at 0x80000428).
pub(super) const RESONANCE: [u32; 44] = [
    0x3db5aa1a, 0xbe169697, 0x3db5aa1a, 0xbe0ae76a, 0x3da1e89b, 0xbde15a24, 0x3d809bcc, 0xbdab68d1,
    0x3d4c509b, 0xbd806299, 0x3d361887, 0xbd421f8b, 0x3d224b06, 0xbd158860, 0x3d224b06, 0xbceb368a,
    0x3d224b06, 0xbcbcb891, 0x3d10a4d3, 0xbc9a2e9a, 0x3d10a4d3, 0xbc800000, 0x3dcbd4b4, 0xbe169697,
    0x3dcbd4b4, 0xbe0ae76a, 0x3dcbd4b4, 0xbde15a24, 0x3dcbd4b4, 0xbdab68d1, 0x3dcbd4b4, 0xbd806299,
    0x3da1e89b, 0xbd421f8b, 0x3d904d1c, 0xbd158860, 0x3d904d1c, 0xbceb368a, 0x3d653ebb, 0xbcbcb891,
    0x3d4c509b, 0xbc9a2e9a, 0x3d361887, 0xbc800000,
];

struct AFilter {
    core: SweepCore,
    last: Option<(usize, usize, usize, f32, f32)>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = AFilter {
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

fn settings(sense: usize, peak: usize, down: bool) -> Settings {
    let index = if down { 9 - sense } else { 10 + sense };
    let group = index / 10;
    let at = (group * 11 + peak) * 2;
    Settings {
        group,
        c5: f32::from_bits(RESONANCE[at]),
        c6: f32::from_bits(RESONANCE[at + 1]),
        c22: f32::from_bits(SENSITIVITY[index]),
    }
}

impl Effect for AFilter {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let sense = step(values[0], 9);
        let peak = step(values[1], 10);
        let mode = step(values[2], 1);
        let dry = clamp_value(values[3], 100.0) / 100.0;
        let level = clamp_value(values[4], 150.0) / 100.0;
        if self.last == Some((sense, peak, mode, dry, level)) {
            return;
        }
        self.last = Some((sense, peak, mode, dry, level));
        self.core.configure(settings(sense, peak, mode == 1));
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
    fn modes_sweep_in_opposite_directions() {
        // A quiet tone leaves Up closed (cutoff ~80 Hz) and Down open (~1.5 kHz).
        let mut values = Model::AFilter.defaults();
        values[3] = 0.0;
        let mut up = create(48_000.0);
        up.set(&values, 120.0);
        values[2] = 1.0;
        let mut down = create(48_000.0);
        down.set(&values, 120.0);
        let a = gain(up.as_mut(), 800.0, 1e-4, 48_000.0);
        let b = gain(down.as_mut(), 800.0, 1e-4, 48_000.0);
        assert!(b > 10.0 * a, "{a} {b}");
    }

    #[test]
    fn closed_low_pass_dc_gain_matches_the_rom() {
        // Low frequencies pass with 24 c5 (16 c25 c9) plus the dry path.
        let mut values = Model::AFilter.defaults();
        values[3] = 0.0;
        let mut effect = create(44_100.0);
        effect.set(&values, 120.0);
        let expected = 24.0 * w(0x3d90_4d1c);
        let g = gain(effect.as_mut(), 20.0, 1e-4, 44_100.0);
        assert!((g / expected - 1.0).abs() < 0.1, "{g} {expected}");
    }

    #[test]
    fn sense_index_follows_the_mode() {
        assert_eq!(settings(0, 0, false).c22, settings(0, 0, true).c22);
        assert_eq!(settings(0, 0, false).group, 1);
        assert_eq!(settings(9, 0, true).group, 0);
        assert_eq!(settings(9, 0, false).c22, w(0x3efe_2c00));
    }
}
