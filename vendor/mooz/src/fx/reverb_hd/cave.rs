//! Cave (CAVE.ZDL): a dark cave with late, distinct echoes.
//!
//! Runs the CHAMBER/CAVE kernel of [`super::chamber_cave`] with the Cave
//! ROM: a 25 Hz input high-pass, a 7.5 kHz damping low-pass, the low band
//! decaying in 0.72 × the high-band time, a treble-cut post section, two
//! peaking EQs at the output and early taps ~175 ms late.
use super::chamber_cave::{ChamberCave, Config};
use super::common::FirstOrder;
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_cave",
    name: "Cave",
    category: Category::Reverb,
    zdl: "CAVE.ZDL",
    description: "Dark cave reverb with late echoes",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 52,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 100,
            default: 54,
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
            name: "PreD",
            max: 200,
            default: 62,
            display: Display::number(0.0, 1.0, 0, " ms"),
        },
    ],
};

static CONFIG: Config = Config {
    output_taps: [
        14879, 14015, 23745, 22607, 31473, 30741, 39204, 38657, 46251, 46004,
    ],
    input_hp: FirstOrder::new(0.998_222, -0.998_222, 0.996_444),
    tap_gains: [0.879_987, 0.876_401],
    output_filters: [
        [1.006_97, -1.971_108, 0.964_939, 1.971_108, -0.971_909],
        [1.020_043, -1.864_375, 0.859_764, 1.864_375, -0.879_808],
    ],
    damping: FirstOrder::new(0.373_581, 0.373_581, 0.252_839),
    tone: Some(FirstOrder::new(0.623_236, -0.344_961, 0.721_725)),
    early_taps: [542, 847, 1345, 1746, 7612, 7387, 7753, 7934],
    early_left: [
        -0.849_085, 0.771_896, -0.701_743, 0.637_941, -0.178_308, 0.162_098, -0.147_366, 0.133_968,
    ],
    early_right: [
        -0.849_085, 0.771_896, -0.701_743, 0.637_941, -0.178_308, 0.162_098, -0.147_366, 0.133_968,
    ],
    apf_tbl: [
        0.599_994, 0.599_994, 0.599_994, 0.599_994, 0.675_008, 0.675_008, 0.675_008, 0.675_008,
        0.675_008, 0.750_023, 0.750_023,
    ],
    decay_ms_tbl: [
        1200.0, 1200.5, 1208.5, 1242.9, 1335.7, 1531.3, 1886.9, 2472.5, 3370.9, 4677.3, 6500.0,
    ],
    wet_tbl: [
        0.0, 0.0815, 0.1939, 0.3219, 0.4613, 0.6096, 0.7657, 0.9284, 1.0971, 1.2711, 1.45,
    ],
    low_factor: 0.72,
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = ChamberCave::new(&CONFIG, sample_rate);
    effect.set(&crate::Model::Cave.defaults(), 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decay_gains_match_the_firmware_sweep() {
        // Decay 50: comb 1 low/high c[46], c[47]; comb 5 c[54], c[55].
        let (low, high) = ChamberCave::comb_gains(&CONFIG, 50.0);
        assert!((low[0] - 0.681_141_6).abs() < 2e-4, "{}", low[0]);
        assert!((high[0] - 0.758_456_9).abs() < 2e-4, "{}", high[0]);
        assert!((low[4] - 0.793_123_6).abs() < 2e-4 && (high[4] - 0.846_302_1).abs() < 2e-4);
    }

    #[test]
    fn echoes_arrive_late() {
        // The Cave's last early taps sit ~180 ms behind the predelay.
        let mut effect = create(44_100.0);
        let mut v = crate::Model::Cave.defaults();
        v[3] = 0.0;
        v[2] = 100.0;
        effect.set(&v, 120.0);
        let mut energy = [0.0f32; 2];
        for n in 0..44_100 / 2 {
            let x = if n == 0 { 1.0 } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            energy[usize::from(n > 44_100 / 10)] += l * l + r * r;
        }
        assert!(energy[1] > 0.0 && energy[0] > 0.0, "{energy:?}");
    }
}
