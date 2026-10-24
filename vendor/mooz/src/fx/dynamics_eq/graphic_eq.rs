//! GraphicEQ (GEQ.ZDL), after the recovered `graphic_eq.rs`/`block_equalizer.rs` callback: 6
//! bands on the mono sum `0.5 (L + R)`, Level (c2) and the ±1 clamp.
//!
//! Editors decoded: `Fx_GEQ_edit_1..5` design Main type-10 peaks at 160/400/
//! 800/3.2k/6.4k Hz with Q √2 and `v − 12` dB; `edit_6` copies a first-order
//! shelf from the ROM table 0x80000440, which is exactly the designer's type 8
//! at 12 kHz with `v − 12` dB (checked in `common`), so it is designed here at
//! the host rate. Level is `v/100`.
use super::eq::{BAND_DB, GUITAR, Graphic};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_graphic_eq",
    name: "GraphicEQ",
    category: Category::Eq,
    zdl: "GEQ.ZDL",
    description: "Six-band graphic EQ for guitar",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "160",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "400",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "800",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "3.2k",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "6.4k",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "12k",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Graphic::new(
        &GUITAR,
        false,
        crate::Model::GraphicEq.defaults(),
        sample_rate,
    ))
}

#[cfg(test)]
mod tests {
    use super::super::common::test_util::{sine_gain, values};
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn defaults_are_flat() {
        let fs = 48_000.0;
        let mut effect = create(fs);
        for hz in [60.0, 700.0, 5000.0] {
            let gain = sine_gain(effect.as_mut(), fs, hz, 0.3);
            assert!(gain_to_db(gain).abs() < 0.05, "{hz}: {gain}");
        }
    }

    #[test]
    fn a_band_boosts_at_its_frequency() {
        let fs = 48_000.0;
        let mut settings = [12.0; 6];
        settings[2] = 24.0;
        let mut list = settings.to_vec();
        list.push(100.0);
        let effect = Graphic::new(&GUITAR, false, values(&list), fs);
        let boost = gain_to_db(effect.magnitude(GUITAR[2].hz));
        assert!((boost - GUITAR[2].boost).abs() < 0.6, "{boost}");
    }
}
