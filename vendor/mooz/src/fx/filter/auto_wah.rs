//! AutoWah (AUTOWAH.ZDL): envelope-swept resonant band-pass.
//!
//! Callback `Fx_FLT_AutoWah` (see `wah_core`) with the ROM block at
//! 0x80000630. Editors decoded from the disassembly (and the recovered
//! `auto_wah_parameters.rs`):
//! - Sense (`sense_edit`): group = Sense / 10 selects a direction set
//!   (0x80000600: c19, c20, c14, c15, c17, c30; down for -10..-1, up for
//!   1..10) and c24 = `SENSITIVITY[Sense]` (0x800005b0: +12..-2 dB, then
//!   -12..+12 dB) through the 0x3af7c5ec ramp; a direction change fades c3
//!   out and back in (Main's 16 ms timed setter).
//! - Reso (`reso_edit`): c18 (input gain) and c28 (damping) from the
//!   0x80000460 table, row `group * 11 + Reso`.
//! - Level (`Level_edit`): c2 = Level / 100 with the default ramp.
use super::common::{SENSE_LABELS, step, w};
use super::wah_core::{Direction, Rom, Settings, WahCore};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_auto_wah",
    name: "AutoWah",
    category: Category::Filter,
    zdl: "AUTOWAH.ZDL",
    description: "Envelope-controlled wah that sweeps up or down",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Sense",
            max: 19,
            default: 17,
            display: Display::labels(&SENSE_LABELS),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 8,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

const ROM: Rom = Rom {
    first: [w(0x3f7c_2496), w(0xbf78_9347), w(0x3f74_b7dc)],
    second: [w(0x3ffd_9a99), w(0xbffa_01ed), w(0x3f78_cea7)],
    c13: w(0x3f7f_b7e9),
    c16: 1.0,
};

/// 0x80000600: c19, c20, c14, c15, c17, c30 per direction.
const DIRECTIONS: [Direction; 2] = [
    Direction {
        c19: w(0xbe69_6eb5),
        c20: w(0x3e91_e531),
        c14: w(0x3a60_881b),
        c15: w(0x3f7f_c7de),
        c17: 0.0,
        c30: 4.0,
        c31: 0.0,
    },
    Direction {
        c19: w(0x3e8f_b4f4),
        c20: w(0x3b8c_0f39),
        c14: w(0x3a11_f7b1),
        c15: w(0x3f7f_db82),
        c17: w(0x3e85_2400),
        c30: 3.5,
        c31: 0.0,
    },
];

/// c24 per Sense value.
const SENSITIVITY: [u32; 20] = [
    0x407ec9e1, 0x4051fb22, 0x402fdd7f, 0x40134ad3, 0x3ff6b923, 0x3fcea34d, 0x3fad10b7, 0x3f90f27f,
    0x3f72cb8d, 0x3f4b5918, 0x3e809bcc, 0x3eae49fb, 0x3eec31d1, 0x3f200b51, 0x3f58e3e5, 0x3f92f6b3,
    0x3fc729e2, 0x4006f3c3, 0x4036e2c3, 0x407ec9e1,
];

/// (c18, c28) per `group * 11 + Reso`.
const RESONANCE: [u32; 44] = [
    0x3ee4b3b6, 0x3f9b2800, 0x3ee4b3b6, 0x3f91ff75, 0x3ecbd4b4, 0x3f78115d, 0x3eb5aa1a, 0x3f46567b,
    0x3e904d1c, 0x3f1adf36, 0x3e809bcc, 0x3ef1a771, 0x3e809bcc, 0x3ebe7a32, 0x3e653ebb, 0x3e9858f8,
    0x3e653ebb, 0x3e7785dd, 0x3e4c509b, 0x3e4c1dcd, 0x3e361887, 0x3e2aaaab, 0x3f21866c, 0x3f800000,
    0x3f187d50, 0x3f71826a, 0x3f0ff59a, 0x3f4e739d, 0x3f07e80b, 0x3f263bd8, 0x3f004dce, 0x3f029cbc,
    0x3ef240e2, 0x3ecccccd, 0x3ee4b3b6, 0x3ea2067b, 0x3ed7e89b, 0x3e81f301, 0x3ecbd4b4, 0x3e5391fc,
    0x3ec06dc3, 0x3e2ebe79, 0x3eb5aa1a, 0x3e124925,
];

struct AutoWah {
    core: WahCore,
    last: Option<(usize, usize, f32)>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mut effect = AutoWah {
        core: WahCore::new(sample_rate, &ROM, &DIRECTIONS),
        last: None,
    };
    let mut values = [0.0; crate::MAX_PARAMS];
    for (value, param) in values.iter_mut().zip(INFO.params) {
        *value = f32::from(param.default);
    }
    effect.set(&values, 120.0);
    Box::new(effect)
}

fn settings(sense: usize, reso: usize) -> Settings {
    let group = sense / 10;
    let at = (group * 11 + reso) * 2;
    Settings {
        group,
        c18: f32::from_bits(RESONANCE[at]),
        c28: f32::from_bits(RESONANCE[at + 1]),
        c24: f32::from_bits(SENSITIVITY[sense]),
    }
}

impl Effect for AutoWah {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let sense = step(values[0], 19);
        let reso = step(values[1], 10);
        let level = super::common::clamp_value(values[2], 150.0) / 100.0;
        if self.last == Some((sense, reso, level)) {
            return;
        }
        self.last = Some((sense, reso, level));
        self.core.configure(settings(sense, reso));
        self.core.set_level(level);
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.core.process(frame)
    }

    fn clear(&mut self) {
        self.core.clear();
        self.last = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn tone_level(effect: &mut dyn Effect, hz: f32, sample_rate: f32, amplitude: f32) -> f32 {
        let mut peak = 0.0f32;
        let frames = sample_rate as usize / 2;
        for i in 0..frames {
            let x = amplitude * (std::f32::consts::TAU * hz * i as f32 / sample_rate).sin();
            let y = effect.process([x, x])[0];
            if i > frames / 2 {
                peak = peak.max(y.abs());
            }
        }
        peak
    }

    #[test]
    fn quiet_input_leaves_the_up_wah_closed_and_loud_input_opens_it() {
        // Sense +8 (up): a very quiet 1.5 kHz tone stays below the closed
        // band; a loud one opens the filter towards 2 kHz.
        let sample_rate = 48_000.0;
        let mut quiet = create(sample_rate);
        let mut loud = create(sample_rate);
        let soft = tone_level(quiet.as_mut(), 1500.0, sample_rate, 0.0005) / 0.0005;
        let hard = tone_level(loud.as_mut(), 1500.0, sample_rate, 0.5) / 0.5;
        assert!(hard > 4.0 * soft, "{soft} {hard}");
    }

    #[test]
    fn open_band_peaks_like_the_rom_predicts() {
        // Sense +8 opens fully: f = c20 + c19 = 0.285 (2.0 kHz). The band-pass
        // peak is c30 * c18 / c28 = 3.5 * 0.398 / 0.207 (Reso 8, up set).
        let expected = 3.5 * w(0x3ecb_d4b4) / w(0x3e53_91fc);
        for sample_rate in [44_100.0, 96_000.0] {
            let mut effect = create(sample_rate);
            let gain = tone_level(effect.as_mut(), 2000.0, sample_rate, 0.5) / 0.5;
            assert!(
                (gain / expected - 1.0).abs() < 0.25,
                "{sample_rate}: {gain} {expected}"
            );
            let mut effect = create(sample_rate);
            let off = tone_level(effect.as_mut(), 6000.0, sample_rate, 0.5) / 0.5;
            assert!(off < 0.4 * gain, "{off}");
        }
    }

    #[test]
    fn sense_direction_switch_is_click_free_and_takes_effect() {
        let mut effect = create(44_100.0);
        let mut values = Model::AutoWah.defaults();
        for i in 0..4410 {
            let x = 0.3 * (i as f32 * 0.05).sin();
            effect.process([x, x]);
        }
        values[0] = 3.0;
        effect.set(&values, 120.0);
        let mut previous = effect.process([0.0, 0.0])[0];
        for i in 0..8820 {
            let x = 0.3 * (i as f32 * 0.05).sin();
            let y = effect.process([x, x])[0];
            assert!((y - previous).abs() < 0.5, "jump at {i}");
            previous = y;
        }
    }

    #[test]
    fn level_is_linear() {
        let mut a = create(44_100.0);
        let mut b = create(44_100.0);
        let mut values = Model::AutoWah.defaults();
        values[2] = 50.0;
        b.set(&values, 120.0);
        let ga = tone_level(a.as_mut(), 800.0, 44_100.0, 0.2);
        let gb = tone_level(b.as_mut(), 800.0, 44_100.0, 0.2);
        assert!((gb / ga - 0.5).abs() < 0.02, "{ga} {gb}");
    }
}
