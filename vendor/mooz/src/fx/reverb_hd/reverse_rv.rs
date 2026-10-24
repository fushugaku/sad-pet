//! ReverseRv (RVS_REV.ZDL): a diffused line read by 20 taps whose gains rise
//! towards the last one, so the reverb swells and cuts off; see
//! [`super::fir_reverb`].
use super::fir_reverb::{Config, FirReverb};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_reverse_rv",
    name: "ReverseRv",
    category: Category::Reverb,
    zdl: "RVS_REV.ZDL",
    description: "Reverse reverb that swells up and stops",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Level",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Decay",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 6
        ParamInfo {
            name: "Dry",
            max: 1,
            default: 0,
            display: Display::OFF_ON,
        },
    ],
};

static CONFIG: Config = Config {
    fir_gains: [
        0.1, 0.14, 0.18, 0.22, 0.25, 0.29, 0.33, 0.36, 0.4, 0.44, 0.47, 0.51, 0.55, 0.58, 0.62,
        0.66, 0.69, 0.78, 0.89, 1.0,
    ],
    decay_taps: [
        [
            8542, 8542, 8542, 8586, 8586, 8586, 8675, 8763, 9027, 9204, 9292,
        ],
        [
            8675, 8675, 8675, 8763, 8763, 8939, 9204, 9468, 10262, 10350, 10791,
        ],
        [
            8763, 8763, 8763, 9027, 9204, 9292, 9733, 10350, 11409, 11673, 12379,
        ],
        [
            9027, 9027, 9027, 9292, 9468, 9821, 10350, 11409, 12908, 13261, 14231,
        ],
        [
            9204, 9204, 9204, 9733, 10086, 10350, 11144, 12379, 14231, 15113, 16083,
        ],
        [
            9468, 9468, 9468, 10086, 10350, 11056, 11673, 13173, 15819, 16877, 18288,
        ],
        [
            9733, 9733, 9733, 10350, 10791, 11585, 12732, 14496, 17142, 18729, 20052,
        ],
        [
            10086, 10086, 10086, 10791, 11409, 12114, 13173, 15642, 18994, 20405, 22169,
        ],
        [
            10262, 10262, 10262, 11144, 11673, 12908, 14231, 16877, 20405, 22257, 24286,
        ],
        [
            10527, 10527, 10527, 11585, 12379, 13261, 15113, 18288, 22257, 24639, 26932,
        ],
        [
            10791, 10791, 10791, 11938, 12908, 14055, 15819, 18994, 24021, 26491, 28784,
        ],
        [
            11144, 11144, 11144, 12379, 13173, 14584, 16877, 20317, 25962, 28255, 30901,
        ],
        [
            11409, 11409, 11409, 12908, 14055, 15378, 17230, 21375, 27549, 30107, 33635,
        ],
        [
            11673, 11673, 11673, 13173, 14496, 16083, 18553, 23051, 29049, 33018, 35664,
        ],
        [
            11938, 11938, 11938, 13437, 15113, 16877, 19523, 24286, 30901, 34870, 37604,
        ],
        [
            12379, 12379, 12379, 14231, 15642, 17230, 20317, 25344, 33547, 36722, 40515,
        ],
        [
            12732, 12732, 12732, 14584, 16083, 18465, 20934, 26932, 34958, 38574, 42543,
        ],
        [
            12996, 12996, 12996, 15113, 16877, 18994, 22169, 27990, 36810, 41044, 45013,
        ],
        [
            13173, 13173, 13173, 15642, 17142, 19788, 23316, 29049, 38310, 43161, 47394,
        ],
    ],
    level_tbl: [
        0.0, 0.011_288, 0.067_728, 0.079_016, 0.112_88, 0.146_744, 0.225_76, 0.307_034, 0.361_052,
        0.487_642, 0.501_187,
    ],
    wet_gain_tbl: Some([
        1.303_166_7,
        1.258_925_4,
        1.230_268_8,
        1.202_264_4,
        1.174_897_6,
        1.148_153_5,
        1.122_018_5,
        1.059_253_7,
        1.0,
        1.0,
        1.0,
    ]),
    hpf_tbl: [
        300.0, 270.0, 240.0, 220.0, 200.0, 180.0, 160.0, 140.0, 120.0, 100.0, 100.0,
    ],
    hpf_at_input: false,
    direct: -0.3,
    feedback: [0.315, 1.035],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = FirReverb::new(&CONFIG, sample_rate);
    effect.set(&crate::Model::ReverseRv.defaults(), 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taps_and_levels_follow_the_decay_table() {
        // Decay 25: tap 2 = 8564, tap 20 = 14407 (c[14], c[32]).
        assert!((FirReverb::tap(&CONFIG, 0, 25.0) - 8564.0).abs() < 1.0);
        assert!((FirReverb::tap(&CONFIG, 18, 25.0) - 14407.0).abs() < 1.0);
        assert!((FirReverb::tap(&CONFIG, 18, 100.0) - 47394.0).abs() < 1e-3);
    }

    #[test]
    fn swells_towards_the_end() {
        // An impulse comes back loudest near the last tap (~0.88 s at Decay 100).
        let fs = 44_100.0;
        let mut effect = create(fs);
        let mut early = 0.0f32;
        let mut late = 0.0f32;
        for n in 0..(fs as usize) {
            let x = if n == 0 { 1.0 } else { 0.0 };
            let [l, _] = effect.process([x, x]);
            if (4_410..13_230).contains(&n) {
                early += l * l;
            } else if (30_000..39_000).contains(&n) {
                late += l * l;
            }
        }
        assert!(late > 2.0 * early, "late {late} early {early}");
    }
}
