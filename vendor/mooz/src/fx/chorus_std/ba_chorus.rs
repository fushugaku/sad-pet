//! Ba Chorus (B_CHORUS.ZDL): the Chorus callback and editors (see
//! [`super::chorus`], [`super::modulated`]) for bass: the first section is a
//! LoCut high-pass from its own table, the second the fixed ROM 5 kHz
//! low-pass, the LFO smoothing pole is c20/c21 = 1134460/2146349186.
//!
//! Editors decoded from the disassembly:
//! - Depth: as Chorus, plus PreD: c22 += 441 samples (10 ms) when on.
//! - LoCut: memcpy of a 3-word row into c7..c9: Off (20 Hz), 60..800 Hz.
//! - Rate, Mix (timed), Level: as Chorus.
use super::{
    chorus::{depth_words, rate_word},
    common::{TONE, UNSET, defaults, sanitize, step, tone_lp},
    modulated::{Core, Layout},
};
use crate::{Effect, Values, catalog::*, zoom::curve};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_chorus",
    name: "Ba Chorus",
    category: Category::Modulation,
    zdl: "B_CHORUS.ZDL",
    description: "Bass chorus with low cut and optional 10 ms pre-delay",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 42,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 22,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 57,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "LoCut",
            max: 10,
            default: 2,
            display: Display::labels(&[
                "Off", "60Hz", "120Hz", "180Hz", "200Hz", "280Hz", "340Hz", "400Hz", "500Hz",
                "630Hz", "800Hz",
            ]),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "PreD",
            max: 1,
            default: 0,
            display: Display::OFF_ON,
        },
    ],
};

/// LoCut table (B_CHORUS 800004e8): first-order high-pass `(b0, b1, fb)` at
/// 20, 60, 120, 180, 200, 280, 340, 400, 500, 630 and 800 Hz.
const LOCUT: [[u32; 3]; 11] = [
    [0x3f7fa2c3, 0xbf7fa2c3, 0x3f7f4585],
    [0x3f7ee912, 0xbf7ee912, 0x3f7dd225],
    [0x3f7dd47f, 0xbf7dd47f, 0x3f7ba8ff],
    [0x3f7cc23d, 0xbf7cc23d, 0x3f79847a],
    [0x3f7c6753, 0xbf7c6753, 0x3f78cea7],
    [0x3f7afe2d, 0xbf7afe2d, 0x3f75fc5a],
    [0x3f79f1e7, 0xbf79f1e7, 0x3f73e3cd],
    [0x3f78e7cd, 0xbf78e7cd, 0x3f71cf9a],
    [0x3f77310b, 0xbf77310b, 0x3f6e6215],
    [0x3f74ff46, 0xbf74ff46, 0x3f69fe8c],
    [0x3f722ed6, 0xbf722ed6, 0x3f645dab],
];

const LAYOUT: Layout = Layout {
    max_base: 423.0 + 441.0,
    span: 512.0,
    smoothing: (1_134_460, 2_146_349_186),
    offset: 0,
    // Depth moves glide (at most 159 samples); PreD switches crossfade.
    jump: 250.0,
    timed_wet: true,
};

struct BaChorus {
    core: Core,
    last: Values,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = BaChorus {
        core: Core::new(LAYOUT, sample_rate),
        last: UNSET,
    };
    effect.set(&defaults(&INFO), 120.0);
    effect.core.settle();
    Box::new(effect)
}

impl Effect for BaChorus {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let v = sanitize(values, &INFO);
        let last = self.last;
        if v[0] != last[0] || step(v[5], 1) != step(last[5], 1) || last[5].is_nan() {
            let (base, depth) = depth_words(v[0]);
            self.core.set_base(base + 441.0 * step(v[5], 1) as f32);
            self.core.set_depth(depth);
        }
        if v[1] != last[1] {
            self.core.set_increment_44k(rate_word(v[1]));
        }
        if v[2] != last[2] {
            self.core.set_wet(curve(1.0, 0.0, 101, v[2], 0));
        }
        if step(v[3], 10) != step(last[3], 10) || last[3].is_nan() {
            self.core
                .set_filters(LOCUT[step(v[3], 10)], tone_lp(&TONE[5]));
        }
        if v[4] != last[4] {
            self.core.set_level(curve(1.5, 0.0, 151, v[4], 0));
        }
        self.last = v;
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
    use crate::{dsp::gain_to_db, fx::chorus_std::common::FirstOrder};

    #[test]
    fn locut_rows_keep_their_corners() {
        for (row, hz) in [(1, 60.0), (10, 800.0)] {
            for rate in [44_100.0, 96_000.0] {
                let hp = FirstOrder::rom(LOCUT[row], rate);
                assert!((gain_to_db(hp.magnitude(rate, hz)) + 3.01).abs() < 0.1);
            }
        }
    }

    #[test]
    fn pre_delay_adds_ten_ms() {
        let mut effect = BaChorus {
            core: Core::new(LAYOUT, 48_000.0),
            last: UNSET,
        };
        let mut values = defaults(&INFO);
        effect.set(&values, 120.0);
        effect.core.settle();
        let (off, _) = effect.core.delays();
        values[5] = 1.0;
        effect.set(&values, 120.0);
        effect.core.settle();
        let (on, _) = effect.core.delays();
        assert!(((on - off) / 48.0 - 10.0).abs() < 0.01);
    }
}
