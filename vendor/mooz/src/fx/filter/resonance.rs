//! Resonance (RESONS.ZDL): envelope-swept resonant low-pass.
//!
//! Callback `Fx_FLT_Resonance` is the AutoWah graph (`wah_core`) with its own
//! ROM block (0x800005f8: the first detector section is a plain wire, c7 = 1)
//! and tables. Editors decoded from the disassembly:
//! - Sense (`sense_edit`): group = Sense / 10 picks a direction set at
//!   0x80000680 (c19, c20, c14, c15, c17, c31: the low-pass output, 3.5 down
//!   or 3.0 up); c24 = 0x800006b0 table (1.0..3.98, 0..+12 dB) indexed
//!   `9 - Sense % 10` for the down half and `Sense % 10` for the up half,
//!   with the 0x3af7c5ec ramp. A direction change fades c3 out and in.
//! - Reso (`reso_edit`): c18/c28 from 0x800004a8, row `group * 11 + Reso`.
//! - Level (`Lvl_edit`): c2 = Level / 100.
use super::common::{SENSE_LABELS, clamp_value, step, w};
use super::wah_core::{Direction, Rom, Settings, WahCore};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_resonance",
    name: "Resonance",
    category: Category::Filter,
    zdl: "RESONS.ZDL",
    description: "Envelope-swept resonant low-pass filter",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Sense",
            max: 19,
            default: 14,
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
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

const ROM: Rom = Rom {
    first: [1.0, 0.0, 0.0],
    second: [w(0x3ffd_9a99), w(0xbffa_01ed), w(0x3f78_cea7)],
    c13: w(0x3f7f_bac7),
    c16: 1.0,
};

/// 0x80000680: c19, c20, c14, c15, c17, c31 per direction.
const DIRECTIONS: [Direction; 2] = [
    Direction {
        c19: w(0xbe69_6eb5),
        c20: w(0x3e83_4e46),
        c14: w(0x3aad_f899),
        c15: w(0x3f7f_a904),
        c17: 0.0,
        c30: 0.0,
        c31: 3.5,
    },
    Direction {
        c19: w(0x3e8f_b4f4),
        c20: w(0x3b8c_0f39),
        c14: w(0x3aad_f899),
        c15: w(0x3f7f_a904),
        c17: 0.0,
        c30: 0.0,
        c31: 3.0,
    },
];

/// 0x800006b0: c24 (0..+12 dB in ten steps).
const SENSITIVITY: [u32; 10] = [
    0x3f800000, 0x3f950215, 0x3fad76d9, 0x3fc9ef2b, 0x3feb13b0, 0x4008d45f, 0x401f4970, 0x40396e14,
    0x4057dd27, 0x407ec9e1,
];

/// 0x800004a8: (c18, c28) per `group * 11 + Reso`.
const RESONANCE: [u32; 44] = [
    0x3eb5aa1a, 0x3f800000, 0x3eb5aa1a, 0x3f5e9bd3, 0x3ea1e89b, 0x3f200000, 0x3e809bcc, 0x3ed9df52,
    0x3e4c509b, 0x3e969697, 0x3e361887, 0x3e579436, 0x3e224b06, 0x3e200000, 0x3e224b06, 0x3df544fb,
    0x3e224b06, 0x3dc13522, 0x3e10a4d3, 0x3d9bbdcf, 0x3e10a4d3, 0x3d800000, 0x3f004dce, 0x3f800000,
    0x3efdaba6, 0x3f5e9bd3, 0x3ef50f01, 0x3f200000, 0x3ee759a9, 0x3ed9df52, 0x3ecbd4b4, 0x3e969697,
    0x3ea1e89b, 0x3e579436, 0x3e904d1c, 0x3e200000, 0x3e904d1c, 0x3df544fb, 0x3e653ebb, 0x3dc13522,
    0x3e361887, 0x3d9bbdcf, 0x3e361887, 0x3d800000,
];

struct Resonance {
    core: WahCore,
    last: Option<(usize, usize, f32)>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = Resonance {
        core: WahCore::new(sample_rate, &ROM, &DIRECTIONS),
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
    let index = if group == 0 {
        9 - sense % 10
    } else {
        sense % 10
    };
    let at = (group * 11 + reso) * 2;
    Settings {
        group,
        c18: f32::from_bits(RESONANCE[at]),
        c28: f32::from_bits(RESONANCE[at + 1]),
        c24: f32::from_bits(SENSITIVITY[index]),
    }
}

impl Effect for Resonance {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let sense = step(values[0], 19);
        let reso = step(values[1], 10);
        let level = clamp_value(values[2], 150.0) / 100.0;
        if self.last == Some((sense, reso, level)) {
            return;
        }
        self.last = Some((sense, reso, level));
        self.core.configure(settings(sense, reso));
        self.core.set_level(level);
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

    fn gain_at(effect: &mut dyn Effect, hz: f32, amplitude: f32, sample_rate: f32) -> f32 {
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
    fn closed_up_filter_is_a_low_pass() {
        // Sense +5 with a tiny input: the filter sits near its bottom (~30 Hz),
        // so 100 Hz passes far better than 3 kHz.
        let mut low = create(48_000.0);
        let mut high = create(48_000.0);
        let a = gain_at(low.as_mut(), 100.0, 1e-4, 48_000.0);
        let b = gain_at(high.as_mut(), 3000.0, 1e-4, 48_000.0);
        assert!(a > 20.0 * b, "{a} {b}");
    }

    #[test]
    fn sensitivity_index_mirrors_the_down_half() {
        assert_eq!(settings(0, 0).c24, settings(19, 0).c24);
        assert_eq!(settings(9, 0).c24, 1.0);
        assert_eq!(settings(10, 0).c24, 1.0);
        assert_eq!(settings(3, 10).group, 0);
        assert_eq!(settings(13, 10).c28, 0.0625);
    }
}
