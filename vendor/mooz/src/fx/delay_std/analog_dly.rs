//! AnalogDly (ANLGDLY.ZDL). The shared DELAY/ANLGDLY callback (see
//! `feedback.rs`) with ANLGDLY's ROM: the delay input is darkened by a
//! one-pole (~2 kHz) and a first-order low-pass (~760 Hz shelf), every pass
//! goes through a second-order low-pass, and HiDMP selects a second-order
//! low-pass in the loop (~300 Hz at 0 up to ~5 kHz at 10). F.B tops out at
//! 0.65. Editors decoded completely; see `feedback.rs`.
use super::feedback::{AnalogRom, FeedbackDelay, TIME_THRESHOLD, Voicing};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_analog_dly",
    name: "AnalogDly",
    category: Category::Delay,
    zdl: "ANLGDLY.ZDL",
    description: "Analog-style delay up to 4 s, darker repeats",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 4022,
            default: 359,
            display: Display::DelayTime {
                threshold: TIME_THRESHOLD,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 28,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 40,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "HiDMP",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "P-P",
            max: 1,
            default: 0,
            display: Display::OFF_ON,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// ROM 0x80000430: eleven second-order low-passes.
const HIDMP: [[u32; 5]; 11] = [
    [0x39a2903f, 0x3a229060, 0x39a2903f, 0x3ffa20b4, 0xbf7492b0],
    [0x3a9ef810, 0x3b1ef808, 0x3a9ef810, 0x3ff43551, 0xbf69a893],
    [0x3b2ef0d8, 0x3baef0d4, 0x3b2ef0d8, 0x3fee404b, 0xbf5f3c5a],
    [0x3b98294c, 0x3c18294a, 0x3b98294c, 0x3fe843ce, 0xbf5548e6],
    [0x3c241012, 0x3ca41013, 0x3c241012, 0x3fdc3be5, 0xbf42b8cc],
    [0x3c8bf37f, 0x3d0bf37f, 0x3c8bf37f, 0x3fd02a61, 0xbf31d332],
    [0x3cd2207e, 0x3d52207e, 0x3cd2207e, 0x3fc4190e, 0xbf22762c],
    [0x3d119135, 0x3d919136, 0x3d119135, 0x3fb80f49, 0xbf1482df],
    [0x3d3ee1fa, 0x3dbee1fa, 0x3d3ee1fa, 0x3fac1273, 0xbf07dd64],
    [0x3d707fe0, 0x3df07fe0, 0x3d707fe0, 0x3fa02652, 0xbef8d936],
    [0x3daf7edc, 0x3e2f7edc, 0x3daf7edc, 0x3f888900, 0xbed1a2dc],
];

const VOICING: Voicing = Voicing {
    // ROM 0x8000066c, coefficients 12..23.
    analog: Some(AnalogRom {
        pole: 0x3f412b48,
        input: [0x3de38230, 0xbc0d8b86, 0x00000000, 0x3f65c5e8, 0x00000000],
        feedback: [0x3d707fe0, 0x3df07fe0, 0x3d707fe0, 0x3fa02652, 0xbef8d936],
    }),
    hidmp: &HIDMP,
    // fb_edit: curve maximum 0x3f266666.
    feedback_max: 0.65,
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(FeedbackDelay::new(sample_rate, &VOICING))
}

#[cfg(test)]
mod tests {
    use super::super::common::{self, testing::*};
    use super::*;
    use crate::Model;

    fn defaults() -> crate::Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn rom_filters_are_unity_at_dc_and_dark_above() {
        for words in HIDMP
            .iter()
            .chain([&VOICING.analog.as_ref().unwrap().feedback])
        {
            let c = common::rom_biquad(*words, 48_000.0);
            assert!((c.magnitude(48_000.0, 5.0) - 1.0).abs() < 0.01);
            // No resonance above unity: the loop stays stable at F.B 100.
            for hz in [50.0, 200.0, 500.0, 1000.0, 3000.0] {
                assert!(c.magnitude(48_000.0, hz) < 1.02, "{hz}");
            }
        }
        let input = common::rom_biquad(VOICING.analog.as_ref().unwrap().input, 44_100.0);
        assert!(input.magnitude(44_100.0, 5000.0) < 0.3);
    }

    #[test]
    fn echo_is_low_passed_and_on_time() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        effect.set(&with(defaults(), &[(0, 49.0), (1, 0.0), (2, 80.0)]), 120.0);
        let out = render(effect.as_mut(), 9600, sine(200.0, 0.5, sample_rate));
        let low = rms(&out, 0, 4800);
        let mut effect = create(sample_rate);
        effect.set(&with(defaults(), &[(0, 49.0), (1, 0.0), (2, 80.0)]), 120.0);
        let wet_high = render(effect.as_mut(), 9600, sine(6000.0, 0.5, sample_rate));
        // Dry is unity, so a dark echo shows as little extra level at 6 kHz.
        assert!(low > rms(&wet_high, 0, 4800));
    }

    #[test]
    fn maximum_feedback_decays() {
        let mut effect = create(44_100.0);
        effect.set(&with(defaults(), &[(0, 9.0), (1, 100.0), (3, 10.0)]), 120.0);
        let out = render(effect.as_mut(), 220_500, |i| {
            if i < 4410 { [0.8, 0.8] } else { [0.0, 0.0] }
        });
        assert!(rms(&out, 0, 200_000) < 1e-4);
    }
}
