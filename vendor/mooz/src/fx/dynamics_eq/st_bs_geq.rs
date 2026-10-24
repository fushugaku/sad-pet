//! St Bs GEQ (ST_B_GEQ.ZDL), after the recovered `stereo_graphic_eq.rs` callback: 7
//! bands on each channel separately, Level (c2) and the ±1 clamp.
//!
//! Editors decoded: `Fx_ST_BS_GEQ_Set` reads per-band tables (ROM 0x80000610..):
//! type-10 peaks at 50/125/390/520/800/4.5k Hz (Q 2/2.5/5.5/5/5/5) and a
//! type-8 shelf at 10 kHz; each band spans its own cut..boost range
//! (−17..+17.5 dB at 50 Hz down to −11..+11 dB at 520 Hz) via
//! `curve(cut, 0, 13, v)` / `curve(boost, 0, 13, v − 12)`. Level is `v/100`.
use super::eq::{BAND_DB, BASS, Graphic};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_st_bs_geq",
    name: "St Bs GEQ",
    category: Category::Eq,
    zdl: "ST_B_GEQ.ZDL",
    description: "Stereo seven-band graphic EQ for bass",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "50",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "120",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "400",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "500",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "800",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "4.5k",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[6] is firmware parameter word 8
        ParamInfo {
            name: "10k",
            max: 24,
            default: 12,
            display: BAND_DB,
        },
        // values[7] is firmware parameter word 9
        ParamInfo {
            name: "Level ",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Graphic::new(
        &BASS,
        true,
        crate::Model::StBsGeq.defaults(),
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
        let mut settings = [12.0; 7];
        settings[2] = 24.0;
        let mut list = settings.to_vec();
        list.push(100.0);
        let effect = Graphic::new(&BASS, true, values(&list), fs);
        let boost = gain_to_db(effect.magnitude(BASS[2].hz));
        assert!((boost - BASS[2].boost).abs() < 0.6, "{boost}");
    }
}
