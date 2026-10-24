//! Comp (COMP.ZDL, `Fx_DYN_Comp` 0x7800..0x7c80) with its decoded editors and
//! ROM image (0x80000464).
//!
//! A mono feedback compressor in the style of a dynamic "squeeze" pedal:
//! `mono = (L + R) / 2`; the detector high-pass (c10..c12, ~660 Hz) and a
//! +2 dB treble shelf (c13..c15) shape it (the callback runs two identical
//! branches and keeps the larger, i.e. one branch); `e = |c5 aa g|` uses the
//! gain g of the previous output (c6, rewritten by the callback), is limited
//! to 1 and smoothed twice (c16/c17, then c18/c19); `level = min(c20 e2, 1)`
//! and the new gain is `g = |c21 level - 1|`. The wet signal is
//! `c7 g² mono` through the Tone shelf (c22..c24), times Level c2 (c8 = 1,
//! c9 = 0), and replaces both channels.
//!
//! Editors: `sens_edit` glides c5 (detector gain) and c7 (auto level) to
//! `sens_tbl[ATTCK][Sense]` and `autolvl_tbl[ATTCK][Sense]` (16 ms);
//! `attack_edit` copies the six-word Slow/Fast set into c16..c21 and reruns
//! `sens_edit`; `tone_edit` copies three words of `tone_tbl` (a 1 kHz first-
//! order shelf, -6 dB at 0, flat at 5, +6 dB at 10); Level → c2 = `v / 100`.
//! The ROM filters are re-derived for the host rate through their bilinear
//! prototype. The original updates c6 once per 8-sample block for the
//! detector; this port feeds back the previous sample's gain.
use super::common::{self, FirstOrder, Glide, one_pole};
use crate::dsp::flush;
use crate::zoom::Ramp;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_comp",
    name: "Comp",
    category: Category::Dynamics,
    zdl: "COMP.ZDL",
    description: "Feedback compressor for a squeezed, sustaining sound",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Sense",
            max: 10,
            default: 6,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 6,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "ATTCK",
            max: 1,
            default: 0,
            display: Display::labels(&["Slow", "Fast"]),
        },
    ],
};

/// Sense → c5 per ATTCK (Fx_Cmp_cmp_sens_tbl) (COMP 0x80000520).
const SENSE: [[u32; 11]; 2] = [
    [
        0x3e4c0000, 0x3ea1c000, 0x3f004000, 0x3f4b4000, 0x3f800000, 0x3fa12000, 0x3fcad000,
        0x3fff6000, 0x404a6000, 0x40a06000, 0x40fe2c00,
    ],
    [
        0x3f218000, 0x3f4b4000, 0x3f800000, 0x3fa12000, 0x3fcad000, 0x3fe39000, 0x3fff6000,
        0x4020c000, 0x404a6000, 0x407ec800, 0x40a06000,
    ],
];
/// Sense → c7 per ATTCK (Fx_Cmp_cmp_autolvl_tbl) (COMP 0x800004c8).
const AUTO_LEVEL: [[u32; 11]; 2] = [
    [
        0x3fb4c000, 0x3fcad000, 0x3fe39000, 0x3fff6000, 0x400f4000, 0x4020c000, 0x404a6000,
        0x407ec800, 0x40b3f000, 0x40fe2c00, 0x41338400,
    ],
    [
        0x3f8f9000, 0x3f982000, 0x3fa12000, 0x3fb4c000, 0x3fcad000, 0x3ff11000, 0x400f4000,
        0x40346000, 0x40631000, 0x4086f000, 0x40a06000,
    ],
];
/// ATTCK → c16..c21 (Fx_Cmp_cmp_attack_tbl) (COMP 0x80000578).
const ATTACK: [[u32; 6]; 2] = [
    [
        0x3b0c3dfb, 0x3f7f73c2, 0x3d8ac4e9, 0x3f7ee7d1, 0x40074200, 0x3f700000,
    ],
    [
        0x3b8c1786, 0x3f7ee7d1, 0x3d0aeb00, 0x3f7f73c2, 0x3f700000, 0x3f780000,
    ],
];
/// Tone → c22..c24 (Fx_Cmp_cmp_tone_tbl) (COMP 0x800003e0).
const TONE: [[u32; 3]; 11] = [
    [0x3f04b531, 0xbee619d9, 0x3f6e57bc],
    [0x3f17984c, 0xbf036c0d, 0x3f6bd3c0],
    [0x3f2d0b79, 0xbf160387, 0x3f68f80e],
    [0x3f455e0f, 0xbf2b17ec, 0x3f65b9dd],
    [0x3f60e61b, 0xbf42f380, 0x3f620d65],
    [0x3f800000, 0xbf5de5d9, 0x3f5de5d9],
    [0x3f91b359, 0xbf80a645, 0x3f5de5d9],
    [0x3fa60607, 0xbf94f8f4, 0x3f5de5d9],
    [0x3fbd5b82, 0xbfac4e6e, 0x3f5de5d9],
    [0x3fd825fb, 0xbfc718e7, 0x3f5de5d9],
    [0x3ff6e88f, 0xbfe5db7b, 0x3f5de5d9],
];
/// ROM c10..c12 (detector high-pass) and c13..c15 (detector shelf).
const DETECTOR_HIGHPASS: [u32; 3] = [0x3f747896, 0xbf747896, 0x3f68f12c];
const DETECTOR_SHELF: [u32; 3] = [0x3fa0a13f, 0xbf9caba5, 0x3f7814cd];

/// c16..c21 of one ATTCK setting, at the host rate.
#[derive(Debug, Clone, Copy)]
struct Envelope {
    first: (f32, f32),
    second: (f32, f32),
    scale: f32,
    depth: f32,
}

impl Envelope {
    fn new(words: [u32; 6], sample_rate: f32) -> Self {
        let c = words.map(f32::from_bits);
        Self {
            first: one_pole(c[0], c[1], sample_rate),
            second: one_pole(c[2], c[3], sample_rate),
            scale: c[4],
            depth: c[5],
        }
    }
}

struct Comp {
    highpass: FirstOrder,
    shelf: FirstOrder,
    tone: FirstOrder,
    tones: [FirstOrder; 11],
    envelopes: [Envelope; 2],
    envelope: Envelope,
    sense: Glide,
    auto_level: Glide,
    level: Ramp,
    cache: [usize; 3],
    primed: bool,
    first: f32,
    second: f32,
    gain: f32,
}

impl Comp {
    #[inline]
    fn tick(&mut self, mono: f32) -> f32 {
        let detector = self.shelf.process(self.highpass.process(mono));
        let e = (self.sense.tick() * detector * self.gain).abs().min(1.0);
        let envelope = self.envelope;
        self.first = flush(e * envelope.first.0 + self.first * envelope.first.1);
        self.second = flush(self.second * envelope.second.1 + self.first * envelope.second.0);
        let level = (self.second * envelope.scale).min(1.0);
        self.gain = (level * envelope.depth - 1.0).abs();
        let wet = self.auto_level.tick() * self.gain * self.gain * mono;
        self.tone.process(wet) * self.level.tick()
    }
}

impl Effect for Comp {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let sense = common::index(values[0], 10);
        let tone = common::index(values[1], 10);
        let attack = common::index(values[3], 1);
        if attack != self.cache[2] {
            self.envelope = self.envelopes[attack];
        }
        if sense != self.cache[0] || attack != self.cache[2] {
            self.sense.set(f32::from_bits(SENSE[attack][sense]));
            self.auto_level
                .set(f32::from_bits(AUTO_LEVEL[attack][sense]));
        }
        if tone != self.cache[1] {
            self.tone.set_coefficients(self.tones[tone]);
        }
        self.level.set(common::level(values[2]));
        self.cache = [sense, tone, attack];
        if !self.primed {
            self.primed = true;
            self.sense.snap();
            self.auto_level.snap();
            common::settle(&mut self.level);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let wet = self.tick(0.5 * (left + right));
        [wet, wet]
    }

    fn clear(&mut self) {
        self.highpass.clear();
        self.shelf.clear();
        self.tone.clear();
        self.first = 0.0;
        self.second = 0.0;
        self.gain = 0.0;
        self.sense.snap();
        self.auto_level.snap();
        common::settle(&mut self.level);
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(build(sample_rate))
}

fn build(sample_rate: f32) -> Comp {
    let envelopes = ATTACK.map(|words| Envelope::new(words, sample_rate));
    Comp {
        highpass: FirstOrder::from_words(DETECTOR_HIGHPASS).resample(sample_rate),
        shelf: FirstOrder::from_words(DETECTOR_SHELF).resample(sample_rate),
        tone: FirstOrder::from_words(TONE[6]).resample(sample_rate),
        tones: TONE.map(|words| FirstOrder::from_words(words).resample(sample_rate)),
        envelopes,
        envelope: envelopes[0],
        sense: Glide::new(f32::from_bits(SENSE[0][6]), sample_rate),
        auto_level: Glide::new(f32::from_bits(AUTO_LEVEL[0][6]), sample_rate),
        level: common::level_ramp(100.0, sample_rate),
        cache: [6, 6, 0],
        primed: false,
        first: 0.0,
        second: 0.0,
        // The initializer clears the state; c6 starts at the ROM 0 too.
        gain: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db(x: f32) -> f32 {
        20.0 * x.log10()
    }

    fn steady(comp: &mut Comp, amplitude: f32, sample_rate: f32) -> f32 {
        let frames = sample_rate as usize * 2;
        let mut peak = 0.0f32;
        for i in 0..frames {
            let x = amplitude * (i as f32 * std::f32::consts::TAU * 1000.0 / sample_rate).sin();
            let [l, _] = comp.process([x, x]);
            if i > frames * 3 / 4 {
                peak = peak.max(l.abs());
            }
        }
        peak
    }

    #[test]
    fn tone_table_is_a_one_kilohertz_shelf() {
        let flat = FirstOrder::from_words(TONE[5]);
        let bright = FirstOrder::from_words(TONE[10]);
        let dark = FirstOrder::from_words(TONE[0]);
        for hz in [20.0, 1000.0, 15_000.0] {
            assert!((flat.magnitude(44_100.0, hz) - 1.0).abs() < 1e-4);
        }
        assert!((db(bright.magnitude(44_100.0, 22_000.0)) - 6.0).abs() < 0.1);
        assert!((db(dark.magnitude(44_100.0, 22_000.0)) + 6.0).abs() < 0.1);
        assert!((bright.magnitude(44_100.0, 1.0) - 1.0).abs() < 1e-3);
    }

    #[test]
    fn output_level_rises_much_slower_than_input() {
        let mut comp = build(48_000.0);
        let mut values = [0.0; crate::MAX_PARAMS];
        values[0] = 6.0;
        values[1] = 5.0;
        values[2] = 100.0;
        comp.set(&values, 120.0);
        let a = steady(&mut comp, 0.05, 48_000.0);
        let b = steady(&mut comp, 0.5, 48_000.0);
        // 20 dB more input gives a few dB more output (the gain is squared).
        let slope = db(b / a) / 20.0;
        assert!(slope < 0.3, "{a} {b} {slope}");
        assert!(a > 0.01 && b < 0.5, "{a} {b}");
    }

    #[test]
    fn attack_switch_selects_the_envelope_set() {
        let mut comp = build(44_100.0);
        let mut values = [6.0, 6.0, 100.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        comp.set(&values, 120.0);
        assert_eq!(comp.envelope.depth, 0.968_75);
        assert_eq!(comp.sense.target, f32::from_bits(SENSE[1][6]));
        values[3] = 0.0;
        comp.set(&values, 120.0);
        assert_eq!(comp.envelope.depth, 0.9375);
    }
}
