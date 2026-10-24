//! M Comp (M_COMP.ZDL): the [`super::peak_comp`] kernel with the module's
//! decoded editors and ROM image (0x800003c4).
//!
//! - THRSH → c8 = Main curve(0.00156, 0.0398, 51, v, two slopes), a 16 ms
//!   glide: THRSH 0 is -28 dBFS and 50 is -56 dBFS (higher = more compression).
//! - Ratio → c9 from the ROM table (2, 3, 4, 5, 6, 8, 10, 20, 40, 60 : 1).
//! - ATTCK → c6 = curve(0.00332, 0.00126, 10, v, two slopes): 18 ms (1) to
//!   7 ms (10) follower times.
//! - Level → c2 = `v / 100`.
//!
//! ROM: release c7 = 0.9998/sample (113 ms), makeup c11 = 2 (+6 dB), a
//! treble-weighted detector shelf (c14..c16: unity at DC, +12 dB at the top)
//! and a 0.7 ms gain smoother (c17/c18). Output is mono like the original.
use super::common;
use super::peak_comp::{PeakComp, Rom};
use crate::zoom;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_m_comp",
    name: "M Comp",
    category: Category::Dynamics,
    zdl: "M_COMP.ZDL",
    description: "Peak compressor with a treble-weighted detector",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "THRSH",
            max: 50,
            default: 40,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Ratio",
            max: 9,
            default: 3,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "ATTCK",
            max: 9,
            default: 0,
            display: Display::offset(1.0),
        },
    ],
};

/// Ratio → c9 (M_COMP 0x80000418).
const RATIO: [u32; 10] = [
    0x3efffc00, 0x3eaaa800, 0x3e7ff800, 0x3e4cc800, 0x3e2aa800, 0x3dfff000, 0x3dccc000, 0x3d4cc000,
    0x3cccc000, 0x3c888000,
];
/// `thr_edit` curve bounds (A4 maximum, B4 minimum).
const THRESHOLD_MAXIMUM: u32 = 0x3acc0000;
const THRESHOLD_MINIMUM: u32 = 0x3d230000;
/// `attack_edit` curve bounds.
const ATTACK_MAXIMUM: u32 = 0x3b59afe4;
const ATTACK_MINIMUM: u32 = 0x3aa57132;
/// ROM c7 (release) and c11 (makeup).
const RELEASE: u32 = 0x3f7ff2e6;
const MAKEUP: f32 = 2.0;
const ROM: Rom = Rom {
    detector: [0x407c5f7c, 0xc07ac09a, 0x3f79847a],
    smoothing: [0x3d09c63e, 0x3f77639c],
};

pub(crate) fn threshold(value: f32) -> f32 {
    zoom::curve(
        f32::from_bits(THRESHOLD_MAXIMUM),
        f32::from_bits(THRESHOLD_MINIMUM),
        51,
        value.clamp(0.0, 50.0),
        2,
    )
}

pub(crate) fn attack(value: f32) -> f32 {
    zoom::curve(
        f32::from_bits(ATTACK_MAXIMUM),
        f32::from_bits(ATTACK_MINIMUM),
        10,
        value.clamp(0.0, 9.0),
        2,
    )
}

struct MComp {
    core: PeakComp,
    cache: [f32; 4],
}

impl Effect for MComp {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let values = [values[0], values[1], values[2], values[3]]
            .map(|v| if v.is_finite() { v } else { 0.0 });
        if values[0] != self.cache[0] {
            self.core.set_threshold(threshold(values[0]));
        }
        if values[1] != self.cache[1] {
            self.core
                .set_ratio(f32::from_bits(RATIO[common::index(values[1], 9)]));
        }
        if values[3] != self.cache[3] {
            self.core.set_attack(attack(values[3]));
        }
        self.core.set_level(values[2]);
        self.cache = values;
        self.core.prime();
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }

    fn clear(&mut self) {
        self.core.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut core = PeakComp::new(&ROM, sample_rate);
    core.set_release(f32::from_bits(RELEASE));
    core.set_makeup(MAKEUP);
    Box::new(MComp {
        core,
        cache: [f32::NAN; 4],
    })
}

#[cfg(test)]
mod tests {
    use super::super::peak_comp::tests::steady_peak;
    use super::*;

    #[test]
    fn editor_curves_match_the_disassembly() {
        assert!((common_db(threshold(0.0)) + 28.0).abs() < 0.1);
        assert!((common_db(threshold(50.0)) + 56.16).abs() < 0.1);
        assert!((attack(0.0) - 0.001_262).abs() < 1e-6);
        assert!((attack(9.0) - 0.003_322).abs() < 1e-6);
    }

    fn common_db(x: f32) -> f32 {
        20.0 * x.log10()
    }

    #[test]
    fn quiet_input_gets_the_rom_makeup_and_loud_input_is_compressed() {
        let mut effect = create(48_000.0);
        let mut values = INFO_DEFAULTS;
        effect.set(&values, 120.0);
        // -60 dBFS is below every threshold: +6 dB of c11.
        let quiet = steady_peak(effect.as_mut(), 0.001, 48_000.0);
        assert!((quiet / 0.001 - 2.0).abs() < 0.05, "{quiet}");
        // THRSH 0 (-28 dBFS), 5:1, fastest attack: loud input is pulled down.
        // The envelope rises at c6 and drops with |x|, so a steady sine reads
        // well below its peak, the more so the slower the attack.
        values[0] = 0.0;
        values[3] = 9.0;
        effect.set(&values, 120.0);
        let a = steady_peak(effect.as_mut(), 0.05, 48_000.0) / 0.05;
        let b = steady_peak(effect.as_mut(), 0.8, 48_000.0) / 0.8;
        assert!(b < 0.5 * a, "{a} {b}");
        assert!(b > 0.2, "{b}");
    }

    const INFO_DEFAULTS: Values = [40.0, 3.0, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
}
