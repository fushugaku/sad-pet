//! ANA234Cho (ANA234CH.ZDL), an MXR M234 style analog chorus. Callback
//! `Fx_MOD_ANA234Cho` is the SuperCho graph ([`bbd`](super::bbd)) with a
//! per-channel input filter (c52..c56, ROM: first-order high shelf, -3 dB
//! above ~3 kHz) in front of the mono sum, the LFO polynomial
//! `1 - c50 (0.75 |p| + 0.25 p^2)` and three tone slots c25 / c30 / c35.
//!
//! Decoded editors (`TBL_TO_VAL` = 5-point linear tables over 0..100):
//! - `rate_edit`: increment `TBL_TO_VAL_int(25322, 64278, 133426, 229844,
//!   608697)` (0.26..6.25 Hz), wave depth `c50 = 1 - 0.03 (RATE/100)^2.2`.
//! - `depth_edit`: amplitude `TBL(0.24, 1.0, 2.6, 3.0, 3.6)` ms, center
//!   `TBL(4.92, 4.44, 3.76, 3.4, 3.28)` ms.
//! - `eq_low_edit`: c35 = high-pass (type 3) at `TBL(1200, 740, 350, 80, 20)` Hz,
//!   Q `TBL(.75, .75, .7, .7, .7)`; c25 = peaking (type 10) at 140 Hz, Q 1,
//!   gain `TBL(0, 0, 0, 0, 1.25)` dB.
//! - `eq_high_edit`: c30 = high shelf (type 8) at `TBL(750, 1000, ...)` Hz,
//!   gain `TBL(-11, -9, -6, -3, 0)` dB.
//! - `mix_edit`: wet `c6 = curve(1.25, 0.175)`; its second word c7
//!   (`curve(1, 0.65)`) is not read by the callback.
//! - `mode_edit`: c51 mono / stereo routing (`GetString_MonoStereo`).
use super::bbd::Bbd;
use super::common::{Kind, design, rom, table};
use super::super_cho::MONO_STEREO;
use crate::{Effect, Values, catalog::*, zoom::curve};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ana234_cho",
    name: "ANA234Cho",
    category: Category::Modulation,
    zdl: "ANA234CH.ZDL",
    description: "Analog M234 style chorus with low and high EQ",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "LEVEL",
            max: 100,
            default: 70,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "RATE",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "DEPTH",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "LOW",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "HIGH",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Mode",
            max: 1,
            default: 0,
            display: MONO_STEREO,
        },
    ],
};

/// ANA234CH ROM tables (0x80000560.. 0x80000608), five points each.
const AMPLITUDE_MS: [f32; 5] = [0.24, 1.0, 2.6, 3.0, 3.6];
const CENTER_MS: [f32; 5] = [4.92, 4.44, 3.76, 3.4, 3.28];
const HIGH_DB: [f32; 5] = [-11.0, -9.0, -6.0, -3.0, 0.0];
const HIGH_HZ: [f32; 5] = [750.0, 1000.0, 1000.0, 1000.0, 1000.0];
const LOW_DB: [f32; 5] = [0.0, 0.0, 0.0, 0.0, 1.25];
const LOW_Q: [f32; 5] = [0.75, 0.75, 0.7, 0.7, 0.7];
const LOW_HZ: [f32; 5] = [1200.0, 740.0, 350.0, 80.0, 20.0];
const RATE_INCREMENT: [f32; 5] = [25_322.0, 64_278.0, 133_426.0, 229_844.0, 608_697.0];

struct Ana234Cho {
    sample_rate: f32,
    core: Bbd,
    last: Values,
    fresh: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let input = rom(
        0.746_582_5,
        -0.481_996_3,
        0.0,
        0.735_413_8,
        0.0,
        sample_rate,
    );
    let mut effect = Ana234Cho {
        sample_rate,
        core: Bbd::new(sample_rate, 12.0, 0.75, 0.25, Some(input)),
        last: [f32::NAN; crate::MAX_PARAMS],
        fresh: true,
    };
    effect.set(&crate::Model::Ana234Cho.defaults(), 120.0);
    Box::new(effect)
}

impl Effect for Ana234Cho {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.fresh;
        self.fresh = false;
        let changed = |i: usize, last: &Values| values[i] != last[i];
        let sr = self.sample_rate;
        if changed(0, &self.last) {
            self.core
                .set_wet(curve(1.25, 0.175, 101, values[0], 0), jump);
        }
        if changed(1, &self.last) {
            let rate = values[1].clamp(0.0, 100.0);
            self.core
                .lfo
                .set_increment(table(&RATE_INCREMENT, rate, 100.0), sr);
            self.core.lfo.scale = 1.0 - 0.03 * (rate / 100.0).powf(2.2);
        }
        if changed(2, &self.last) {
            let center = table(&CENTER_MS, values[2], 100.0);
            let amplitude = table(&AMPLITUDE_MS, values[2], 100.0);
            self.core.lfo.set_targets(center, amplitude);
        }
        if changed(3, &self.last) {
            let low = values[3];
            let peak = design(Kind::Peaking, sr, 140.0, table(&LOW_DB, low, 100.0), 1.0);
            self.core.tone[0].set(peak);
            let hz = table(&LOW_HZ, low, 100.0);
            let q = table(&LOW_Q, low, 100.0);
            self.core.tone[2].set(design(Kind::HighPass2, sr, hz, 0.0, q));
        }
        if changed(4, &self.last) {
            let high = values[4];
            let hz = table(&HIGH_HZ, high, 100.0);
            let gain = table(&HIGH_DB, high, 100.0);
            self.core.tone[1].set(design(Kind::HighShelf1, sr, hz, gain, 1.0));
        }
        if changed(5, &self.last) {
            self.core.set_stereo(values[5].round() >= 1.0, jump);
        }
        self.last = *values;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }

    fn clear(&mut self) {
        self.core.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn dry_path_is_the_shelved_mono_sum() {
        // LEVEL 0 still leaves 0.175 of wet; check the dry shelf with the
        // wet silenced by a DC input (the wet high-pass removes DC).
        let mut effect = create(48_000.0);
        let mut out = [0.0; 2];
        for _ in 0..48_000 {
            out = effect.process([0.3, 0.1]);
        }
        assert!(
            (out[0] - 0.2).abs() < 2e-3 && (out[1] - 0.2).abs() < 2e-3,
            "{out:?}"
        );
    }

    #[test]
    fn input_shelf_is_three_db_down_at_the_top() {
        let shelf = rom(0.746_582_5, -0.481_996_3, 0.0, 0.735_413_8, 0.0, 96_000.0);
        assert!(gain_to_db(shelf.magnitude(96_000.0, 50.0)).abs() < 0.05);
        assert!((gain_to_db(shelf.magnitude(96_000.0, 20_000.0)) + 2.9).abs() < 0.2);
    }

    #[test]
    fn rate_table_endpoints() {
        let hz = |v: f32| table(&RATE_INCREMENT, v, 100.0) * 44_100.0 / 4_294_967_296.0;
        assert!((hz(0.0) - 0.26).abs() < 0.01);
        assert!((hz(100.0) - 6.25).abs() < 0.01);
    }
}
