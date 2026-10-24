//! Flanger (FLANGER.ZDL): the recovered `Fx_MOD_Flanger` kernel shared with
//! BaFlanger (see `flanger_core`), with the Flanger editors: Depth, Rate (free
//! 0.05..15 Hz or tempo notes above 50), Reso −10..+10, PreD, Mix, Level. All
//! editors were decoded; no mapping here is an approximation.
use super::flanger_core::{Flanger, FlangerCore};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_flanger",
    name: "Flanger",
    category: Category::Modulation,
    zdl: "FLANGER.ZDL",
    description: "Jet flanger with resonance, pre-delay and wet mix",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 47,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 78,
            default: 7,
            display: Display::sync_rate(50, 0.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Reso",
            max: 20,
            default: 18,
            display: Display::offset(-10.0),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "PreD",
            max: 50,
            default: 4,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 65,
            display: Display::INT,
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

/// Flanger's fastest free rate (increment 0x164a89, 15 Hz).
pub const FAST_MAXIMUM: f32 = 1_460_873.0;

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Flanger {
        core: FlangerCore::new(sample_rate, false),
        fast_maximum: FAST_MAXIMUM,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::modulation::common::testing::rms;

    fn run(effect: &mut dyn Effect, seconds: f32, rate: f32) -> Vec<f32> {
        let n = (seconds * rate) as usize;
        (0..n)
            .map(|i| {
                let x = 0.25 * (i as f32 * 0.0731).sin() + 0.1 * (i as f32 * 0.31).sin();
                effect.process([x, x])[0]
            })
            .collect()
    }

    #[test]
    fn defaults_are_finite_and_moderately_loud() {
        let mut effect = create(48_000.0);
        effect.set(&crate::Model::Flanger.defaults(), 120.0);
        let out = run(effect.as_mut(), 2.0, 48_000.0);
        assert!(out.iter().all(|x| x.is_finite()));
        let level = rms(&out[48_000..]);
        // Dry RMS is ~0.19; the original adds 3 × Mix of a resonant wet path.
        assert!(level > 0.15 && level < 1.2, "{level}");
    }

    #[test]
    fn maximum_resonance_stays_bounded() {
        let mut effect = create(96_000.0);
        let values = [100.0, 50.0, 20.0, 50.0, 100.0, 150.0, 0.0, 0.0, 0.0];
        effect.set(&values, 120.0);
        let out = run(effect.as_mut(), 1.0, 96_000.0);
        assert!(out.iter().all(|x| x.is_finite() && x.abs() < 16.0));
    }
}
