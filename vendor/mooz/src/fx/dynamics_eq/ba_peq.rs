//! Ba PEQ (B_PEQ.ZDL), after the recovered `bass_parametric_eq.rs` callback: two
//! Zölzer peaks (Main designer type 10) on the mono sum `0.5 (L + R)`, Level
//! (c2) and the ±1 clamp, on both outputs.
//!
//! Editors decoded: `Fx_PEQ_edit_1/2` read the frequency from the ROM table
//! below, Q from {0.5, 1, 2, 4, 8, 16} and pass `Gain − 20` dB to the designer;
//! Level is `v/100`. Frequency and Q are stepped (rounded), gain is continuous.
use super::eq::{Parametric, Q_LABELS};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_peq",
    name: "Ba PEQ",
    category: Category::Eq,
    zdl: "B_PEQ.ZDL",
    description: "Two-band parametric EQ for bass, ±20 dB",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Freq1",
            max: 37,
            default: 8,
            display: FREQ_LABELS,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Q1",
            max: 5,
            default: 1,
            display: Q_LABELS,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Gain1",
            max: 40,
            default: 20,
            display: Display::number(-20.0, 1.0, 0, " dB"),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Freq2",
            max: 37,
            default: 15,
            display: FREQ_LABELS,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Q2",
            max: 5,
            default: 1,
            display: Q_LABELS,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Gain2",
            max: 40,
            default: 20,
            display: Display::number(-20.0, 1.0, 0, " dB"),
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

const FREQ_LABELS: Display = Display::labels(&[
    "20Hz", "25Hz", "32Hz", "40Hz", "50Hz", "63Hz", "70Hz", "80Hz", "100Hz", "120Hz", "140Hz",
    "150Hz", "160Hz", "180Hz", "200Hz", "250Hz", "300Hz", "350Hz", "400Hz", "450Hz", "500Hz",
    "630Hz", "800Hz", "1.0kHz", "1.2kHz", "1.6kHz", "2.0kHz", "2.5kHz", "3.0kHz", "3.6kHz",
    "4.0kHz", "4.5kHz", "6.3kHz", "8.0kHz", "10kHz", "12kHz", "16kHz", "20kHz",
]);

/// B_PEQ.ZDL 0x80000418: frequency table (every other word; the odd words are 1.0).
pub const FREQUENCIES: [f32; 38] = [
    20.0, 25.0, 32.0, 40.0, 50.0, 63.0, 70.0, 80.0, 100.0, 120.0, 140.0, 150.0, 160.0, 180.0,
    200.0, 250.0, 300.0, 350.0, 400.0, 450.0, 500.0, 630.0, 800.0, 1000.0, 1200.0, 1600.0, 2000.0,
    2500.0, 3000.0, 3600.0, 4000.0, 4500.0, 6300.0, 8000.0, 10000.0, 12000.0, 16000.0, 20000.0,
];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(Parametric::new(
        &FREQUENCIES,
        20.0,
        crate::Model::BaPeq.defaults(),
        sample_rate,
    ))
}

#[cfg(test)]
mod tests {
    use super::super::common::test_util::{sine_gain, values};
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn bands_follow_the_tables() {
        let fs = 48_000.0;
        // Band 1 at table entry 17, Q 2, full boost; band 2 flat.
        let settings = values(&[17.0, 2.0, 40.0, 15.0, 1.0, 20.0, 100.0]);
        let effect = Parametric::new(&FREQUENCIES, 20.0, settings, fs);
        let hz = FREQUENCIES[17];
        let peak = effect.band(0).magnitude(fs, hz);
        assert!((gain_to_db(peak) - 20.0).abs() < 0.05, "{peak}");
        assert!(gain_to_db(effect.band(1).magnitude(fs, 1000.0)).abs() < 1e-3);
    }

    #[test]
    fn defaults_are_flat() {
        let fs = 44_100.0;
        let mut effect = create(fs);
        let gain = sine_gain(effect.as_mut(), fs, 1000.0, 0.3);
        assert!((gain - 1.0).abs() < 1e-3, "{gain}");
    }
}
