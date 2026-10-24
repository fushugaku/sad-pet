//! Chamber (CHAMBER.ZDL): a small, bright room reverb.
//!
//! Runs the CHAMBER/CAVE kernel of [`super::chamber_cave`] with the Chamber
//! ROM: a 60 Hz input high-pass, a 14 kHz damping low-pass, the low band
//! decaying 1.06 × longer than the highs, a high-shelf boost / treble cut pair
//! at the output and tight early reflections.
use super::chamber_cave::{ChamberCave, Config};
use super::common::FirstOrder;
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_chamber",
    name: "Chamber",
    category: Category::Reverb,
    zdl: "CHAMBER.ZDL",
    description: "Bright chamber with tight early reflections",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 100,
            default: 73,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 48,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "PreD",
            max: 200,
            default: 24,
            display: Display::number(0.0, 1.0, 0, " ms"),
        },
    ],
};

static CONFIG: Config = Config {
    output_taps: [
        14086, 13221, 22951, 21814, 30679, 29947, 38411, 37864, 45457, 45210,
    ],
    input_hp: FirstOrder::new(0.995_744, -0.995_744, 0.991_488),
    tap_gains: [0.639_99, 0.637_383],
    output_filters: [
        [1.406_717, -1.386_816, 0.0, 0.977_67, 0.0],
        [0.422_681, -0.341_025, 0.0, 0.918_343, 0.0],
    ],
    damping: FirstOrder::new(0.620_007, 0.620_007, -0.240_014),
    tone: None,
    early_taps: [542, 847, 1345, 1746, 997, 772, 1138, 1319],
    early_left: [
        -0.943_712, 0.857_921, -0.779_949, 0.709_036, -0.434_108, 0.394_643, -0.358_777, 0.326_157,
    ],
    early_right: [
        -0.434_108, 0.394_643, -0.358_777, 0.326_157, -0.943_712, 0.857_921, -0.779_949, 0.709_036,
    ],
    apf_tbl: [
        0.599_994, 0.599_994, 0.599_994, 0.599_994, 0.599_994, 0.599_994, 0.599_994, 0.675_008,
        0.675_008, 0.750_023, 0.750_023,
    ],
    decay_ms_tbl: [
        500.0, 500.6, 509.6, 548.6, 653.6, 875.0, 1277.6, 1940.6, 2957.6, 4436.6, 6500.0,
    ],
    wet_tbl: [
        0.0, 0.0626, 0.1543, 0.2613, 0.3798, 0.5077, 0.6434, 0.7862, 0.9352, 1.09, 1.25,
    ],
    low_factor: 1.06,
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = ChamberCave::new(&CONFIG, sample_rate);
    effect.set(&crate::Model::Chamber.defaults(), 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decay_gains_match_the_firmware_sweep() {
        // Decay 50: comb 1 low/high c[46], c[47]; comb 5 c[54], c[55].
        let (low, high) = ChamberCave::comb_gains(&CONFIG, 50.0);
        assert!((low[0] - 0.633_528_7).abs() < 2e-4, "{}", low[0]);
        assert!((high[0] - 0.616_413_7).abs() < 2e-4, "{}", high[0]);
        assert!((low[4] - 0.759_179_9).abs() < 2e-4 && (high[4] - 0.746_733).abs() < 2e-4);
        assert_eq!(ChamberCave::tone_hz(73.0), 4312.0);
        // Mix 48 -> wet c[69] 0.48212.
        assert!(
            (super::super::common::tbl(&CONFIG.wet_tbl, 101, 48.0, 0.0) - 0.482_12).abs() < 1e-4
        );
    }

    #[test]
    fn decays_and_stays_bounded() {
        let mut effect = create(96_000.0);
        let mut v = crate::Model::Chamber.defaults();
        v[0] = 100.0;
        effect.set(&v, 120.0);
        let mut energy = [0.0f32; 2];
        for n in 0..96_000 * 4 {
            let x = if n < 96 { 0.5 } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            assert!(l.abs() < 4.0 && r.abs() < 4.0);
            energy[usize::from(n >= 96_000 * 3)] += l * l + r * r;
        }
        assert!(energy[1] < energy[0] * 0.1, "{energy:?}");
    }
}
