//! Z Tron (Z_TRON.ZDL): envelope-swept resonant low-pass with dry blend.
//!
//! Callback `Fx_FLT_Z_Tron` (`sweep_core`, squared envelope; the original's
//! shared coefficient/state RAM aliasing only matters for overlapping slot
//! layouts and is not reproduced). ROM block 0x80000578: the first detector
//! section is a wire, the second a first-order high shelf c14/c15/c16.
//! Editors decoded from the disassembly:
//! - Sense (`sense_edit`): group = Sense / 10; c22 = 0x800005e0[Sense]
//!   (default ramp); a group change ramps c9 to 0, copies 0x80000630 +
//!   24 group (c18, c19, c21, c23, c24/c7), re-runs Reso and ramps c9 to the
//!   sixth word (0.75, low-pass output).
//! - Reso (`reso_edit`): c5/c6 from 0x80000428 + 8 (Reso + 11 group), the
//!   A-Filter Peak table.
//! - Dry: c10 = 0.25 Dry / 100. Level: c2 = Level / 100.
use super::a_filter::RESONANCE;
use super::common::{SENSE_LABELS, clamp_value, step, w};
use super::sweep_core::{Direction, Rom, Settings, SweepCore};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_z_tron",
    name: "Z Tron",
    category: Category::Filter,
    zdl: "Z_TRON.ZDL",
    description: "Envelope filter with a squelchy resonant sweep",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Sense",
            max: 19,
            default: 12,
            display: Display::labels(&SENSE_LABELS),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Dry",
            max: 100,
            default: 25,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Level",
            max: 150,
            default: 108,
            display: Display::INT,
        },
    ],
};

const ROM: Rom = Rom {
    first: [1.0, 0.0, 0.0],
    second: [w(0x3fc9_d088), w(0xbfc6_37db), w(0x3f78_cea7)],
    c17: w(0x3f7f_bfdf),
    c25: 2.0,
    squared: true,
    peak_feedback: false,
    sense_step: crate::zoom::DEFAULT_STEP,
};

/// 0x80000630: c18, c19, c21, c23, c24, c9 per group (c20 = 1, c8 = 0).
const DIRECTIONS: [Direction; 2] = [
    Direction {
        c18: w(0x3983_c3de),
        c19: w(0x3f7f_ef88),
        c20: 1.0,
        c21: w(0x3f65_3437),
        c23: w(0xbd86_f400),
        c24: w(0x3d91_e531),
        c8: 0.0,
        c9: 0.75,
    },
    Direction {
        c18: w(0x3969_e2e7),
        c19: w(0x3f7f_f162),
        c20: 1.0,
        c21: w(0x3f67_a392),
        c23: w(0x3da7_c792),
        c24: w(0x3be9_6eb4),
        c8: 0.0,
        c9: 0.75,
    },
];

/// 0x800005e0: c22 per Sense value.
const SENSITIVITY: [u32; 20] = [
    0x3e7ec800, 0x3e51f800, 0x3e2fd800, 0x3e134800, 0x3df6b000, 0x3dcea000, 0x3dad1000, 0x3d90f000,
    0x3d72c000, 0x3d4b4000, 0x3c808000, 0x3cae4000, 0x3cec0000, 0x3d200000, 0x3d58e000, 0x3d92f000,
    0x3dc72000, 0x3e06f000, 0x3e36e000, 0x3e7ec800,
];

struct ZTron {
    core: SweepCore,
    last: Option<(usize, usize, f32, f32)>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = ZTron {
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

impl Effect for ZTron {
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

    /// Steady-state gain of a tone burst followed through the sweep.
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
    fn loud_notes_open_the_up_sweep() {
        let mut values = Model::ZTron.defaults();
        values[2] = 0.0;
        let mut quiet = create(48_000.0);
        quiet.set(&values, 120.0);
        let mut loud = create(48_000.0);
        loud.set(&values, 120.0);
        let a = gain(quiet.as_mut(), 1200.0, 1e-4, 48_000.0);
        let b = gain(loud.as_mut(), 1200.0, 0.5, 48_000.0);
        assert!(b > 10.0 * a, "{a} {b}");
    }

    #[test]
    fn dry_and_level_scale_the_output() {
        let mut values = Model::ZTron.defaults();
        values[2] = 100.0;
        values[3] = 150.0;
        let mut effect = create(44_100.0);
        effect.set(&values, 120.0);
        // Far above the closed low-pass only the dry path remains: 1.5 x.
        let g = gain(effect.as_mut(), 15_000.0, 1e-4, 44_100.0);
        assert!((g - 1.5).abs() < 0.1, "{g}");
    }
}
