//! LOFI Rev (LOFI_REV.ZDL): the LOFI/CHURCH kernel of [`super::lofi_church`]
//! with the LOFI ROM: stereo-offset predelays wobbled at 1.07 Hz, combs
//! drifting with a 0.2 Hz LFO, a 410 ms cross feedback loop, a 10 kHz input
//! low-pass and a +3.5 dB 8 kHz output shelf.
use super::common::tbl;
use super::lofi_church::{Config, LofiChurch, Settings};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_lofi_rev",
    name: "LOFI Rev",
    category: Category::Reverb,
    zdl: "LOFI_REV.ZDL",
    description: "Wobbly lo-fi reverb with a cross feedback loop",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "DECAY",
            max: 100,
            default: 52,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "TONE",
            max: 100,
            default: 95,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "FxLVL",
            max: 100,
            default: 44,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "PreD",
            max: 1,
            default: 0,
            display: Display::OFF_ON,
        },
        // values[4] is firmware parameter word 7
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
        0, 9426, 18375, 29401, 40427, 41881, 43075, 44043, 44813, 45421, 45889, 47343,
    ],
    comb_writes: [51351, 56707, 62063, 67355, 72647, 78003, 83359, 88651],
    loop_reads: [25431, 40426],
    diffuser_reads: [41880, 43074, 44042, 44812, 45420, 45888, 47342, 48536],
    comb_reads: [53391, 59343, 65129, 70999, 74687, 80639, 86425, 92295],
    output_taps: [52021, 57616, 63442, 69057, 73317, 78912, 84738, 90353],
    cross_mix: 0.67,
    centre_mix: 2.42,
    diffuser_mix: 0.3,
    tap_mix: 0.33,
    comb_input: 1.0,
    split: 0.180_099,
    input_sections: [
        [0.995_744, -0.995_744, 0.0, 0.991_488, 0.0],
        [0.463_426, 0.463_426, 0.0, 0.073_149, 0.0],
    ],
    comb_shelf: super::common::FirstOrder::new(0.992_131, -0.908_645, 0.900_776),
    output_section: [1.002_765, -1.990_417, 0.987_782, 1.990_417, -0.990_547],
    shelf_hz: 8000.0,
    lfo: [(23.04, 104_209), (55.04, 18_991)],
    apf_tbl: [0.0, 0.2, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
    apf_wet_tbl: [
        0.42, 0.672, 0.84, 0.84, 0.84, 0.84, 0.84, 0.84, 0.84, 0.84, 0.84,
    ],
    high_ratio_tbl: [
        0.07, 0.1, 0.348_351, 0.472_22, 0.582_092, 0.640_886, 0.711_161, 0.843_36, 0.893_545,
        0.935_36, 1.0,
    ],
    mix_tbl: [
        0.000_398, 0.002_239, 0.008_913, 0.025_119, 0.056_234, 0.089_125, 0.105_925, 0.141_254,
        0.177_828, 0.167_88, 0.199_526,
    ],
    loop_ms: [47.52, 61.04, 69.52, 82.64],
    loop_base: [0.001_238_049_5, 0.001_303_21, 0.001_371_8, 0.001_444],
    cross_loop: Some((410.0, 0.0003)),
};

/// Editor results for LOFI Rev knob values.
pub fn settings(values: &Values) -> Settings {
    let [decay, tone, level, predelay, dry, ..] = *values;
    let t = (decay / 100.0).clamp(0.0, 1.0);
    let base = 12_000.0 * t * t;
    let (low, high, loop_gains) = CONFIG.gains(
        base * (0.8 - 0.3 * t),
        base * tbl(&CONFIG.high_ratio_tbl, 101, tone, 0.0),
    );
    Settings {
        apf: tbl(&CONFIG.apf_tbl, 101, decay, 0.0),
        apf_wet: tbl(&CONFIG.apf_wet_tbl, 101, decay, 0.0),
        low,
        high,
        loop_gains,
        wet: tbl(&CONFIG.mix_tbl, 101, level, 0.0),
        dry: if dry >= 0.5 { 1.0 } else { 0.0 },
        damp_hz: 15_000.0,
        shelf_db: 3.5,
        predelay: if predelay >= 0.5 { 829.0 } else { 521.0 },
    }
}

struct LofiRev {
    kernel: LofiChurch,
    values: Option<Values>,
}

impl Effect for LofiRev {
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
    let mut effect = LofiRev {
        kernel: LofiChurch::new(&CONFIG, sample_rate),
        values: None,
    };
    effect.set(&crate::Model::LofiRev.defaults(), 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn editors_match_the_firmware_sweep() {
        let s = settings(&Model::LofiRev.defaults());
        // c[97] (comb 1 low), c[93] (high), c[84]/c[85] (cross loop), c[57].
        assert!((s.low[0] - 0.858_79).abs() < 2e-4, "{}", s.low[0]);
        assert!((s.high[0] - 0.903_651_9).abs() < 2e-4, "{}", s.high[0]);
        assert!((s.loop_gains[0] - 0.203_607_2).abs() < 2e-4);
        assert!((s.loop_gains[1] - 0.346_733_4).abs() < 2e-4);
        assert!((s.wet - 0.069_390_4).abs() < 1e-5);
        let mut v = Model::LofiRev.defaults();
        v[0] = 0.0;
        assert_eq!(settings(&v).high, [0.0; 4]);
    }

    #[test]
    fn stays_bounded_at_maximum_decay() {
        let mut effect = create(48_000.0);
        let mut v = Model::LofiRev.defaults();
        v[0] = 100.0;
        v[1] = 100.0;
        v[2] = 100.0;
        effect.set(&v, 120.0);
        let mut peak: f32 = 0.0;
        for n in 0..48_000 * 6 {
            let x = if n < 48_000 {
                0.5 * ((n as f32) * 0.03).sin()
            } else {
                0.0
            };
            let [l, r] = effect.process([x, x]);
            peak = peak.max(l.abs()).max(r.abs());
        }
        assert!(peak.is_finite() && peak < 4.0, "{peak}");
    }
}
