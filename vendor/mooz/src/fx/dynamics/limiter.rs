//! Limiter (LIMITER.ZDL, callback 0..0x340) with its decoded editors and ROM
//! image (0x800004e4). The callback is the [`super::peak_comp`] kernel run one
//! frame at a time: c6 = 1 makes the envelope follow |x| instantly, the
//! detector is the identity, the 1-sample ring tap is the lookahead, the wet
//! path carries c11 c21 = 2 (+6 dB) and the gain smoother (c17 = 0.116,
//! c18 = 0.884) settles in ~0.2 ms.
//!
//! - THRSH → c8 from `thr_tbl`: -6 dBFS (0) to -30 dBFS (50), 16 ms glide.
//! - Ratio → c9 from `ratio_tbl` (4, 6, 8, 10, 12, 14, 16, 24, 36, 60 : 1).
//! - REL → c7 from `release_tbl`: per-sample peak decay of 35 ms (1) to
//!   160 ms (9); REL 10 is exactly 1.0 in the ROM, so the peak (and the gain
//!   reduction it causes) is held until `clear`, as on the pedal.
//! - Level → c2 = `v / 100`.
//!
//! The original's NaN mask on the tap only matters for non-finite input.
//! Output is mono like the original.
use super::common;
use super::peak_comp::{PeakComp, Rom};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_limiter",
    name: "Limiter",
    category: Category::Dynamics,
    zdl: "LIMITER.ZDL",
    description: "Peak limiter with threshold, ratio and release",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "THRSH",
            max: 50,
            default: 20,
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
            default: 90,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "REL",
            max: 9,
            default: 2,
            display: Display::offset(1.0),
        },
    ],
};

/// THRSH → c8 (Fx_DYN_Limiter_thr_tbl) (LIMITER 0x80000418).
const THRESHOLD: [u32; 51] = [
    0x3f004c00, 0x3ef2cc00, 0x3ee5c000, 0x3ed96400, 0x3ecdb400, 0x3ec2a400, 0x3eb82c00, 0x3eae4800,
    0x3ea4e800, 0x3e9c0c00, 0x3e93a800, 0x3e8bb800, 0x3e843400, 0x3e7a3000, 0x3e6cb800, 0x3e600000,
    0x3e53f800, 0x3e489000, 0x3e3dc800, 0x3e339000, 0x3e29e800, 0x3e20c800, 0x3e182000, 0x3e0ff800,
    0x3e083800, 0x3e00e800, 0x3df3f000, 0x3de6d000, 0x3dda6000, 0x3dceb000, 0x3dc39000, 0x3db90000,
    0x3daf1000, 0x3da5b000, 0x3d9cc000, 0x3d945000, 0x3d8c6000, 0x3d84d000, 0x3d7b6000, 0x3d6dc000,
    0x3d610000, 0x3d54e000, 0x3d498000, 0x3d3ea000, 0x3d346000, 0x3d2aa000, 0x3d218000, 0x3d18e000,
    0x3d10a000, 0x3d08e000, 0x3d018000,
];
/// Ratio → c9 (Fx_DYN_Limiter_ratio_tbl) (LIMITER 0x80000540).
const RATIO: [u32; 10] = [
    0x3e800000, 0x3e2aaaab, 0x3e000000, 0x3dcccccd, 0x3daaaaab, 0x3d924925, 0x3d800000, 0x3d2aaaab,
    0x3ce38e39, 0x3c888889,
];
/// REL → c7 (Fx_DYN_Limiter_release_tbl) (LIMITER 0x80000568).
const RELEASE: [u32; 10] = [
    0x3f7fd4f6, 0x3f7fd8b4, 0x3f7fdce9, 0x3f7fe11f, 0x3f7fe5cc, 0x3f7fe98b, 0x3f7feeb0, 0x3f7ff2e6,
    0x3f7ff6a4, 0x3f800000,
];
/// ROM c11 * c21 (c12 = 1).
const MAKEUP: f32 = 2.0;
const ROM: Rom = Rom {
    detector: [0x3f800000, 0x00000000, 0x00000000],
    smoothing: [0x3ded8c43, 0x3f624e78],
};

struct Limiter {
    core: PeakComp,
    cache: [usize; 3],
}

impl Effect for Limiter {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let threshold = common::index(values[0], 50);
        let ratio = common::index(values[1], 9);
        let release = common::index(values[3], 9);
        if threshold != self.cache[0] {
            self.core
                .set_threshold(f32::from_bits(THRESHOLD[threshold]));
        }
        if ratio != self.cache[1] {
            self.core.set_ratio(f32::from_bits(RATIO[ratio]));
        }
        if release != self.cache[2] {
            self.core.set_release(f32::from_bits(RELEASE[release]));
        }
        self.core.set_level(values[2]);
        self.cache = [threshold, ratio, release];
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
    // ROM c6 = 1: the envelope jumps to every new peak.
    core.set_attack(1.0);
    core.set_makeup(MAKEUP);
    Box::new(Limiter {
        core,
        cache: [usize::MAX; 3],
    })
}

#[cfg(test)]
mod tests {
    use super::super::peak_comp::tests::steady_peak;
    use super::*;

    fn db(x: f32) -> f32 {
        20.0 * x.log10()
    }

    #[test]
    fn tables_match_the_rom() {
        assert!((db(f32::from_bits(THRESHOLD[0])) + 6.0).abs() < 0.01);
        assert!((db(f32::from_bits(THRESHOLD[50])) + 30.0).abs() < 0.01);
        assert_eq!(f32::from_bits(RELEASE[9]), 1.0);
        assert!((1.0 / f32::from_bits(RATIO[0]) - 4.0).abs() < 1e-6);
    }

    #[test]
    fn limits_above_threshold_and_doubles_below() {
        let mut effect = create(96_000.0);
        let mut values = [0.0; crate::MAX_PARAMS];
        values[0] = 50.0; // -30 dBFS
        values[1] = 9.0; // 60:1
        values[2] = 100.0;
        values[3] = 0.0;
        effect.set(&values, 120.0);
        let quiet = steady_peak(effect.as_mut(), 0.01, 96_000.0);
        assert!((quiet / 0.01 - 2.0).abs() < 0.02, "{quiet}");
        // c6 = 1: the detector reads the true peak, so the output follows the
        // linear static curve 2 (c8 + (p - c8) / 60).
        let threshold = f32::from_bits(THRESHOLD[50]);
        for amplitude in [0.1, 0.8] {
            let out = steady_peak(effect.as_mut(), amplitude, 96_000.0);
            let expected = 2.0 * (threshold + (amplitude - threshold) / 60.0);
            assert!(
                (db(out / expected)).abs() < 0.3,
                "{amplitude}: {out} {expected}"
            );
        }
    }
}
