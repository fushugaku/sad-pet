//! Church (CHURCH.ZDL): the LOFI/CHURCH kernel of [`super::lofi_church`]
//! with the CHURCH ROM: long combs (52.8 … 91.7 ms) without comb modulation,
//! predelay wobbling at 0.35 Hz, no cross loop, a 5 kHz input low-pass, the
//! HiDMP comb low-pass and a Tone shelf at 4 kHz.
use super::common::tbl;
use super::lofi_church::{Config, LofiChurch, Settings};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_church",
    name: "Church",
    category: Category::Reverb,
    zdl: "CHURCH.ZDL",
    description: "Long church reverb with tone and high damping",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DECAY",
            max: 100,
            default: 49,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PreD",
            max: 200,
            default: 96,
            display: Display::number(0.0, 1.0, 0, " ms"),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "MIX",
            max: 100,
            default: 46,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "TONE",
            max: 100,
            default: 61,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "HiDMP",
            max: 100,
            default: 83,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 8
        ParamInfo {
            name: "Dry",
            max: 1,
            default: 1,
            display: Display::OFF_ON,
        },
    ],
};

static CONFIG: Config = Config {
    writes: [
        0, 8949, 17898, 28924, 39950, 41578, 42906, 43976, 44840, 45518, 46042, 47670,
    ],
    comb_writes: [52134, 57711, 63288, 68801, 74314, 79891, 85468, 90981],
    loop_reads: [22308, 35936],
    diffuser_reads: [41577, 42905, 43975, 44839, 45517, 46041, 47669, 48997],
    comb_reads: [54459, 61755, 66691, 71789, 76639, 83935, 88871, 93969],
    output_taps: [53245, 59854, 64093, 70151, 75425, 82034, 86273, 92331],
    cross_mix: 0.5,
    centre_mix: 0.0,
    diffuser_mix: 1.0,
    tap_mix: 1.0,
    comb_input: 0.45,
    split: 0.247_016,
    input_sections: [
        [0.997_536, -1.995_073, 0.997_536, 1.995_066, -0.995_08],
        [0.271_168, 0.271_168, 0.0, 0.457_663, 0.0],
    ],
    comb_shelf: super::common::FirstOrder::new(0.999_918, -0.985_772, 0.985_69),
    output_section: [0.996_239, -0.996_239, 0.0, 0.992_477, 0.0],
    shelf_hz: 4000.0,
    lfo: [(43.52, 33_795), (0.0, 18_991)],
    apf_tbl: [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.6],
    apf_wet_tbl: [
        0.672, 0.672, 0.84, 0.84, 0.84, 0.84, 0.84, 0.84, 0.84, 0.84, 0.84,
    ],
    high_ratio_tbl: [
        0.1, 0.19, 0.28, 0.37, 0.46, 0.55, 0.64, 0.73, 0.82, 0.91, 1.0,
    ],
    mix_tbl: [
        0.000_345, 0.001_939, 0.007_718, 0.021_752, 0.048_697, 0.077_179, 0.091_728, 0.122_321,
        0.153_993, 0.145_378, 0.172_783,
    ],
    loop_ms: [52.75, 91.73, 77.17, 67.75],
    loop_base: [0.001_238_049_5, 0.001_303_21, 0.001_371_8, 0.001_444],
    cross_loop: None,
};

const HID_F: [f32; 11] = [
    3000.0, 4200.0, 5400.0, 6600.0, 7800.0, 9000.0, 10200.0, 11400.0, 12600.0, 15000.0, 22000.0,
];
const POST_EQ_DB: [f32; 11] = [
    -11.0, -6.6, -4.7, -3.3, -2.1, -1.1, -0.2, 0.7, 1.5, 2.3, 3.0,
];

/// Editor results for Church knob values.
pub fn settings(values: &Values) -> Settings {
    let [decay, predelay, mix, tone, hidmp, dry, ..] = *values;
    let t = (decay / 100.0).clamp(0.0, 1.0);
    let time = 200.0 + 19_800.0 * t.powf(1.9);
    let (low, high, loop_gains) =
        CONFIG.gains(time, time * tbl(&CONFIG.high_ratio_tbl, 101, tone, 0.0));
    Settings {
        apf: tbl(&CONFIG.apf_tbl, 101, decay, 0.0),
        apf_wet: tbl(&CONFIG.apf_wet_tbl, 101, decay, 0.0),
        low,
        high,
        loop_gains,
        wet: tbl(&CONFIG.mix_tbl, 101, mix, 0.0),
        dry: if dry >= 0.5 { 1.0 } else { 0.0 },
        damp_hz: tbl(&HID_F, 101, hidmp, 0.0),
        shelf_db: tbl(&POST_EQ_DB, 101, tone, 0.0),
        predelay: (predelay.max(0.0) * 44.1).round().max(2.0),
    }
}

struct Church {
    kernel: LofiChurch,
    values: Option<Values>,
}

impl Effect for Church {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.values.as_ref() != Some(values) {
            self.values = Some(*values);
            self.kernel.apply(&settings(values));
        }
    }
    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.kernel.process(frame)
    }
    fn clear(&mut self) {
        self.kernel.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = Church {
        kernel: LofiChurch::new(&CONFIG, sample_rate),
        values: None,
    };
    effect.set(&crate::Model::Church.defaults(), 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn editors_match_the_firmware_sweep() {
        let s = settings(&Model::Church.defaults());
        // c[97] (comb 1 low), c[93] (high), c[57], c[152].
        assert!((s.low[0] - 0.935_609_1).abs() < 2e-4, "{}", s.low[0]);
        assert!((s.high[0] - 0.902_529_5).abs() < 2e-4, "{}", s.high[0]);
        assert!((s.wet - 0.065_786_2).abs() < 1e-5, "{}", s.wet);
        assert_eq!(s.predelay, 4234.0);
        assert_eq!(s.loop_gains, [0.0; 2]);
        // HiDMP 50 -> 9 kHz low-pass, c[86] 0.427 / c[88] 0.145.
        let mut v = Model::Church.defaults();
        v[4] = 50.0;
        let d = super::super::common::design1(
            super::super::common::Design::LowPass1,
            44_100.0,
            settings(&v).damp_hz,
            0.0,
        );
        assert!(
            (d.b0 - 0.427_358).abs() < 1e-3 && (d.f1 - 0.145_283).abs() < 1e-3,
            "{d:?}"
        );
    }

    #[test]
    fn long_tail_at_maximum_decay() {
        let mut effect = create(44_100.0);
        let mut v = Model::Church.defaults();
        v[0] = 100.0;
        v[2] = 100.0;
        v[5] = 0.0;
        effect.set(&v, 120.0);
        let mut late = 0.0f32;
        for n in 0..44_100 * 4 {
            let x = if n < 441 { 0.5 } else { 0.0 };
            let [l, r] = effect.process([x, x]);
            assert!(l.abs() < 4.0 && r.abs() < 4.0);
            if n > 44_100 * 3 {
                late += l * l + r * r;
            }
        }
        assert!(late > 1e-7, "{late}");
    }
}
