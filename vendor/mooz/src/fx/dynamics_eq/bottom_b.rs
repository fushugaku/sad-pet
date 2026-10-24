//! Bottom B (BOTTOM_B.ZDL), after the recovered `mono_graphic_eq.rs`
//! (`BottomBoost`) callback: seven biquads on the mono sum `0.5 (L + R)`,
//! Level (c2) and the ±1 clamp, on both outputs.
//!
//! Editors decoded: Bass and Trebl interpolate ROM breakpoint tables at the
//! knob values 0, 2, 5, 8, 10 (segments `[0,2] [2,5] [5,8] [8,10]` with
//! `curve(next, this, count, v − start)`) for frequency, gain, Q and designer
//! type, then call the Main designer:
//! - Bass (0x80000498..): a type-9 low shelf at 16.5 Hz (−10 dB at 0, flat
//!   from 2), a type-10 peak at 47→33 Hz rising 0→22.5 dB (Q 0.6..0.7), a
//!   75 Hz peak (0→2 dB) and a 350→150 Hz dip (0→−2 dB);
//! - Trebl (0x800005d8..): a type-8 high shelf moving 200→850 Hz, 0→15 dB.
//!
//! Band 6 is a fixed ROM first-order treble lift (≈ +10 dB at the top) the
//! editors never touch; band 7 is identity. Level is `v/100`.
use super::common::{Equalizer, Kind, design, rom};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_bottom_b",
    name: "Bottom B",
    category: Category::Eq,
    zdl: "BOTTOM_B.ZDL",
    description: "Bass booster with treble shelf",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Bass",
            max: 10,
            default: 6,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Trebl",
            max: 10,
            default: 7,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 60,
            display: Display::INT,
        },
    ],
};

/// Knob values of the table breakpoints.
const BREAKPOINTS: [f32; 5] = [0.0, 2.0, 5.0, 8.0, 10.0];

struct Band {
    kind: Kind,
    hz: [f32; 5],
    db: [f32; 5],
    q: [f32; 5],
}

const BASS: [Band; 4] = [
    Band {
        kind: Kind::LowShelf1,
        hz: [16.5; 5],
        db: [-10.0, 0.0, 0.0, 0.0, 0.0],
        q: [0.0; 5],
    },
    Band {
        kind: Kind::Peak,
        hz: [47.0, 47.0, 35.0, 30.0, 33.0],
        db: [0.0, 5.0, 16.0, 19.0, 22.5],
        q: [0.6, 0.6, 0.7, 0.6, 0.7],
    },
    Band {
        kind: Kind::Peak,
        hz: [75.0; 5],
        db: [0.0, 0.0, 0.0, 2.0, 2.0],
        q: [0.3, 0.3, 0.3, 0.3, 0.4],
    },
    Band {
        kind: Kind::Peak,
        hz: [350.0, 350.0, 350.0, 350.0, 150.0],
        db: [0.0, 0.0, 0.0, -1.0, -2.0],
        q: [0.3, 0.3, 0.3, 0.3, 0.1],
    },
];

const TREBLE: Band = Band {
    kind: Kind::HighShelf1,
    hz: [200.0, 200.0, 300.0, 700.0, 850.0],
    db: [0.0, 1.7, 7.0, 13.0, 15.0],
    q: [0.0; 5],
};

/// ROM 0x800003f4 c30..34, a fixed treble lift.
const LIFT: [f32; 5] = [2.203_524, -1.448_714_3, 0.0, 0.181_841_2, 0.0];

fn at(table: &[f32; 5], value: f32) -> f32 {
    let value = value.clamp(0.0, 10.0);
    let segment = BREAKPOINTS[1..4].iter().filter(|&&x| value > x).count();
    let (start, end) = (BREAKPOINTS[segment], BREAKPOINTS[segment + 1]);
    table[segment] + (table[segment + 1] - table[segment]) * (value - start) / (end - start)
}

fn coefficients(band: &Band, value: f32, sample_rate: f32) -> crate::dsp::Coefficients {
    design(
        band.kind,
        sample_rate,
        at(&band.hz, value),
        at(&band.db, value),
        at(&band.q, value),
    )
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(BottomB::new(sample_rate))
}

pub struct BottomB {
    sample_rate: f32,
    eq: Equalizer<6>,
    bass: f32,
    treble: f32,
}

impl BottomB {
    fn new(sample_rate: f32) -> Self {
        let mut eq = Equalizer::new(false, sample_rate);
        eq.set_band(5, rom(LIFT, sample_rate));
        let defaults = crate::Model::BottomB.defaults();
        let mut effect = Self {
            sample_rate,
            eq,
            bass: f32::NAN,
            treble: f32::NAN,
        };
        effect.set(&defaults, 120.0);
        effect.eq.jump_level(defaults[2]);
        effect
    }
}

impl Effect for BottomB {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if values[0] != self.bass {
            self.bass = values[0];
            for (i, band) in BASS.iter().enumerate() {
                self.eq
                    .set_band(i, coefficients(band, values[0], self.sample_rate));
            }
        }
        if values[1] != self.treble {
            self.treble = values[1];
            self.eq
                .set_band(4, coefficients(&TREBLE, values[1], self.sample_rate));
        }
        self.eq.set_level(values[2]);
    }
    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.eq.process(frame)
    }
    fn clear(&mut self) {
        self.eq.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsp::gain_to_db;

    #[test]
    fn breakpoints_interpolate_like_the_editor() {
        // Bass 6: segment [5, 8], one third of the way.
        assert!((at(&BASS[1].hz, 6.0) - (35.0 - 5.0 / 3.0)).abs() < 1e-4);
        assert!((at(&BASS[1].db, 6.0) - 17.0).abs() < 1e-4);
        assert_eq!(at(&TREBLE.db, 2.0), 1.7);
        assert_eq!(at(&TREBLE.hz, 10.0), 850.0);
        assert!((at(&TREBLE.db, 3.0) - (1.7 + 5.3 / 3.0)).abs() < 1e-4);
    }

    #[test]
    fn bass_knob_boosts_the_low_end() {
        let fs = 48_000.0;
        let response = |bass: f32| {
            let mut effect = BottomB::new(fs);
            let mut values = crate::Model::BottomB.defaults();
            values[0] = bass;
            values[1] = 0.0;
            effect.set(&values, 120.0);
            gain_to_db(effect.eq.magnitude(fs, 40.0)) - gain_to_db(effect.eq.magnitude(fs, 1000.0))
        };
        assert!(response(10.0) > response(2.0) + 15.0);
        assert!(response(2.0) > 2.0);
    }

    #[test]
    fn rom_lift_is_a_treble_shelf() {
        let fs = 44_100.0;
        let lift = rom(LIFT, fs);
        assert!((gain_to_db(lift.magnitude(fs, 10_000.0)) - 7.9).abs() < 0.3);
        assert!(gain_to_db(lift.magnitude(fs, 100.0)).abs() < 1.0);
    }
}
