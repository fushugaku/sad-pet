//! BaFlanger (B_FLNGR.ZDL): the same recovered `Fx_MOD_Flanger` kernel as
//! Flanger (see `flanger_core`) with identical Depth/Rate/Reso/PreD/Mix/Level
//! editors plus `Fx_MOD_Flanger_locut_edit`, which copies a first-order
//! high-pass from `ModChoLoCutTbl` into the feedback filter c17..19 (Off =
//! 20 Hz, then 60..800 Hz). All editors were decoded.
use super::flanger::FAST_MAXIMUM;
use super::flanger_core::{Flanger, FlangerCore};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_flanger",
    name: "BaFlanger",
    category: Category::Modulation,
    zdl: "B_FLNGR.ZDL",
    description: "Bass flanger with a low cut in the delay path",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 76,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 78,
            default: 40,
            display: Display::sync_rate(50, 0.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Reso",
            max: 20,
            default: 15,
            display: Display::offset(-10.0),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "PreD",
            max: 50,
            default: 2,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Level",
            max: 150,
            default: 95,
            display: Display::INT,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "LoCut",
            max: 10,
            default: 1,
            display: Display::labels(&[
                "Off", "60 Hz", "120 Hz", "180 Hz", "200 Hz", "280 Hz", "340 Hz", "400 Hz",
                "500 Hz", "630 Hz", "800 Hz",
            ]),
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Flanger {
        core: FlangerCore::new(sample_rate, true),
        fast_maximum: FAST_MAXIMUM,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locut_removes_lows_from_the_wet_path() {
        // Depth 0, Reso 0 (index 10), Mix 100: output = dry + 3 × delayed wet.
        let measure = |locut: f32| {
            let mut effect = create(44_100.0);
            let values = [0.0, 0.0, 10.0, 0.0, 100.0, 100.0, locut, 0.0, 0.0];
            effect.set(&values, 120.0);
            let mut energy = 0.0;
            for i in 0..44_100 {
                let x = (i as f32 * std::f32::consts::TAU * 400.0 / 44_100.0).sin() * 0.1;
                let y = effect.process([x, x])[0] - x;
                if i > 4_410 {
                    energy += y * y;
                }
            }
            energy
        };
        // At 400 Hz an 800 Hz first-order high-pass removes ~7 dB more than 20 Hz.
        let ratio = measure(10.0) / measure(0.0);
        assert!(ratio < 0.3 && ratio > 0.1, "{ratio}");
    }
}
