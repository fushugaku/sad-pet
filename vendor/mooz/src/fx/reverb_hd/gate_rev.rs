//! GateRev (GATE_REV.ZDL): a diffused line read by 20 equal taps of
//! alternating sign, so the reverb is dense and then stops dead; see
//! [`super::fir_reverb`].
use super::fir_reverb::{Config, FirReverb};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_gate_rev",
    name: "GateRev",
    category: Category::Reverb,
    zdl: "GATE_REV.ZDL",
    description: "Gated reverb with an abrupt cutoff",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Level",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 55,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 6
        ParamInfo {
            name: "Dry",
            max: 1,
            default: 1,
            display: Display::OFF_ON,
        },
    ],
};

static CONFIG: Config = Config {
    fir_gains: [
        0.19, -0.19, 0.19, -0.19, 0.19, -0.19, 0.19, -0.19, 0.19, -0.19, 0.19, -0.19, 0.19, -0.19,
        0.19, -0.19, 0.19, -0.19, 0.19, -0.19,
    ],
    decay_taps: [
        [
            8542, 8542, 8542, 8586, 8586, 8586, 8675, 8763, 9027, 9204, 9204,
        ],
        [
            8675, 8675, 8675, 8763, 8763, 8939, 9204, 9468, 10262, 10350, 10527,
        ],
        [
            8939, 8939, 8939, 9027, 9204, 9292, 9733, 10350, 11409, 11673, 12114,
        ],
        [
            9027, 9204, 9204, 9292, 9468, 9821, 10350, 11409, 12908, 13173, 14055,
        ],
        [
            9292, 9468, 9468, 9733, 10086, 10350, 11144, 12379, 14231, 15113, 15819,
        ],
        [
            9733, 9733, 9821, 10086, 10350, 11056, 11673, 13173, 15819, 16877, 17230,
        ],
        [
            9821, 10086, 10262, 10350, 10791, 11585, 12732, 14496, 17142, 18553, 19523,
        ],
        [
            10262, 10350, 10527, 10791, 11409, 12114, 13173, 15642, 18994, 20317, 21375,
        ],
        [
            10527, 10791, 11056, 11144, 11673, 12908, 14231, 16877, 20405, 22169, 23845,
        ],
        [
            11056, 11144, 11409, 11585, 12379, 13261, 15113, 18288, 22257, 24286, 25962,
        ],
        [
            11144, 11409, 11585, 11938, 12908, 14055, 15819, 18994, 24021, 26138, 27814,
        ],
        [
            11585, 11673, 11938, 12379, 13173, 14584, 16877, 20317, 25962, 27990, 29931,
        ],
        [
            11938, 12114, 12379, 12908, 14055, 15378, 17230, 21375, 27549, 29931, 32577,
        ],
        [
            12114, 12732, 12908, 13173, 14496, 16083, 18553, 23051, 29049, 32312, 34870,
        ],
        [
            12732, 12996, 13173, 13437, 15113, 16877, 19523, 24286, 30901, 34605, 36810,
        ],
        [
            12996, 13173, 13437, 14231, 15642, 17230, 20317, 25344, 33547, 36281, 38574,
        ],
        [
            13173, 13437, 14231, 14584, 16083, 18465, 20934, 26932, 34958, 38133, 41220,
        ],
        [
            13437, 14231, 14584, 15113, 16877, 18994, 22169, 27990, 36810, 40515, 44131,
        ],
        [
            14231, 14584, 15113, 15642, 17142, 19788, 23316, 29049, 38310, 42367, 46248,
        ],
    ],
    level_tbl: [
        0.0, 0.015_945, 0.095_668, 0.111_613, 0.159_447, 0.207_281, 0.318_894, 0.433_697, 0.51,
        0.688_812, 0.668_344,
    ],
    wet_gain_tbl: None,
    hpf_tbl: [
        140.0, 140.0, 140.0, 140.0, 160.0, 180.0, 150.0, 110.0, 80.0, 80.0, 80.0,
    ],
    hpf_at_input: true,
    direct: -1.0,
    feedback: [0.28, 0.7],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = FirReverb::new(&CONFIG, sample_rate);
    effect.set(&crate::Model::GateRev.defaults(), 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taps_follow_the_decay_table() {
        // Decay 30: tap 2 = 8585, tap 20 = 15641 (c[14], c[32]).
        assert!((FirReverb::tap(&CONFIG, 0, 30.0) - 8585.0).abs() < 1.5);
        assert!((FirReverb::tap(&CONFIG, 18, 30.0) - 15641.0).abs() < 1.5);
    }

    #[test]
    fn stops_after_the_gate() {
        let fs = 48_000.0;
        let mut effect = create(fs);
        let mut v = crate::Model::GateRev.defaults();
        v[3] = 0.0;
        effect.set(&v, 120.0);
        let mut body = 0.0f32;
        let mut after = 0.0f32;
        for n in 0..(fs as usize) {
            let x = if n < 48 { 0.5 } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            let e = l * l + r * r;
            // Decay 55: last tap ~0.48 s.
            if n < 20_000 {
                body += e;
            } else if n > 30_000 {
                after += e;
            }
        }
        assert!(after < body * 0.05, "after {after} body {body}");
    }
}
