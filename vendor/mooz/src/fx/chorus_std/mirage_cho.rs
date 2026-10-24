//! MirageCho (MIRAGECH.ZDL): `mirage_chorus.rs` (original 0a80..11e0).
//!
//! Callback: the mono mean `0.5 (L + R)` goes into the ring (its input, delay
//! and feedback biquads are identity in ROM and the feedback c12 is never
//! set). Two Q31 LFOs: the first gives `(1 - |p0|²) * c55`, the second an
//! envelope `1 - 2 * 0.667 * ||p1| - 0.25|` that sways the first's depth.
//! `delay = shape * depth + center` where depth and center follow their
//! targets through one-poles of ≈122 ms (c45..c48), then an output one-pole
//! (c43/c44) that smooths fast rates. The tap passes a ROM high-pass
//! (≈50 Hz), a ROM low-pass biquad (≈11.5 kHz) and the Tone low-pass;
//! `out = c2 (mean + c6 tone)` on both channels: the output is mono.
//!
//! Editors decoded from the disassembly:
//! - Depth (and PreD, whose editor jumps to Depth): depth = round(44.1 *
//!   curve(9.3, 2, 101, linear)) samples, center = round(44.1 * (PreD + 1))
//!   samples, both gliding.
//! - Rate: t = (Rate/100)^1.62; c54 = 16430 + 266836 t (0.17..2.9 Hz),
//!   c49 = 5649 + 91743 t (0.06..1 Hz); above 35 the output smoothing comes
//!   from a 14-row table. The editor also stores float 1.0 into the Q31 word
//!   c55 (`SPINT` then `INTSP` of `1 - 0.4 t`): the first LFO runs at 0.496 of
//!   full scale, kept here.
//! - Mix: c6 = curve(2, 0, 101, linear). Level: c2 = Level/100.
//! - Tone: Main's filter designer, first-order low-pass at the frequency
//!   interpolated in a 400..16000 Hz table (Tone/10 → row, Tone%10 → step).
use super::common::{
    FirstOrder, Q31, UNSET, defaults, increment, one_pole, rom_biquad, sanitize, step, triangle,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, DelayLine},
    zoom::{self, ORIGINAL_RATE, Ramp, curve},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_mirage_cho",
    name: "MirageCho",
    category: Category::Modulation,
    zdl: "MIRAGECH.ZDL",
    description: "Wavering mono chorus: two LFOs, pre-delay and tone",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 60,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 100,
            default: 38,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 55,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "PreD",
            max: 19,
            default: 10,
            display: Display::offset(1.0),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Tone",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[5] is firmware parameter word 7
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// `Fx_MOD_MirageCho_eq_Freq_tbl` (80000678).
const TONE_HZ: [f32; 11] = [
    400.0, 800.0, 1000.0, 2000.0, 3000.0, 4800.0, 6000.0, 8000.0, 10_000.0, 12_000.0, 16_000.0,
];

/// `rate_iLFO_SMS_table` (80000608): output smoothing c43/c44 above Rate 35.
const SMOOTHING: [[u32; 2]; 14] = [
    [0x08d15cc3, 0x772ea33d],
    [0x07f6f75f, 0x780908a1],
    [0x071af4da, 0x78e50b26],
    [0x063d5380, 0x79c2ac80],
    [0x055e11a4, 0x7aa1ee5c],
    [0x047d2d9b, 0x7b82d265],
    [0x039aa5c2, 0x7c655a3e],
    [0x02b67877, 0x7d498789],
    [0x01d0a420, 0x7e2f5be0],
    [0x00e92729, 0x7f16d8d7],
    [0x00baa7e0, 0x7f455820],
    [0x008c1787, 0x7f73e879],
    [0x005d761b, 0x7fa289e5],
    [0x005d761b, 0x7fa289e5],
];

/// ROM c28, c29, c31: first-order high-pass ≈50 Hz after the tap.
const HIGH_PASS: [u32; 3] = [0x3f7f12c2, 0xbf7f12c2, 0x3f7e2585];
/// ROM c33..c37: low-pass biquad ≈11.5 kHz.
const LOW_PASS: [u32; 5] = [0x3e85db07, 0x3f05db07, 0x3e85db07, 0x3e365dc4, 0xbe6535fd];
/// Depth/center glide c45/c46 (= c47/c48).
const GLIDE: (u32, u32) = (0x0006_1546, 0x7ff9_eab8);
/// c55 as left by the Rate editor: the bits of float 1.0 read as Q31.
const FIRST_AMPLITUDE: f32 = 0x3f80_0000 as f32 * Q31;
/// c52: 0x55555550 Q31.
const ENVELOPE_SCALE: f32 = 0x5555_5550 as f32 * Q31;

struct MirageCho {
    sample_rate: f32,
    scale: f32,
    line: DelayLine,
    phases: [u32; 2],
    increments: [u32; 2],
    glide: (f32, f32),
    depth: f32,
    center: f32,
    depth_target: f32,
    center_target: f32,
    smoothing: (f32, f32),
    delay: f32,
    high_pass: FirstOrder,
    low_pass: Biquad,
    tone: FirstOrder,
    wet: Ramp,
    level: Ramp,
    last: Values,
}

impl MirageCho {
    fn new(sample_rate: f32) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        let mut effect = Self {
            sample_rate,
            scale,
            line: DelayLine::new(((882.0 + 9.3 * 44.1) * scale).ceil() as usize + 8),
            phases: [0; 2],
            increments: [0; 2],
            glide: one_pole(GLIDE.0 as f32 * Q31, GLIDE.1 as f32 * Q31, sample_rate),
            depth: 0.0,
            center: 0.0,
            depth_target: 0.0,
            center_target: 0.0,
            smoothing: (1.0, 0.0),
            delay: 0.0,
            high_pass: FirstOrder::rom(HIGH_PASS, sample_rate),
            low_pass: Biquad::new(rom_biquad(LOW_PASS, sample_rate)),
            tone: FirstOrder::default(),
            wet: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            last: UNSET,
        };
        effect.set(&defaults(&INFO), 120.0);
        effect.depth = effect.depth_target;
        effect.center = effect.center_target;
        effect.delay = effect.center;
        effect.wet.jump(effect.wet.target);
        effect.level.jump(effect.level.target);
        effect
    }

    fn tone_hz(tone: f32) -> f32 {
        let row = ((tone / 10.0).floor() as usize).min(9);
        let fraction = tone / 10.0 - row as f32;
        TONE_HZ[row] + (TONE_HZ[row + 1] - TONE_HZ[row]) * fraction
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(MirageCho::new(sample_rate))
}

impl Effect for MirageCho {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let v = sanitize(values, &INFO);
        let last = self.last;
        if v[0] != last[0] || v[3] != last[3] {
            self.depth_target = (44.1 * curve(9.3, 2.0, 101, v[0], 0)).round() * self.scale;
            self.center_target = (44.1 * curve(20.0, 1.0, 20, v[3], 0)).round() * self.scale;
        }
        if v[1] != last[1] {
            let t = (v[1] / 100.0).powf(1.62);
            self.increments = [
                increment((16_430.19 + 266_835.66 * t).round(), self.sample_rate),
                increment((5649.0 + 91_743.0 * t).round(), self.sample_rate),
            ];
            let rate = step(v[1], 100);
            self.smoothing = if rate > 35 {
                let [input, pole] = SMOOTHING[((rate - 35) / 5).min(13)];
                one_pole(input as f32 * Q31, pole as f32 * Q31, self.sample_rate)
            } else {
                (1.0, 0.0)
            };
        }
        if v[2] != last[2] {
            self.wet.set(curve(2.0, 0.0, 101, v[2], 0));
        }
        if v[4] != last[4] {
            self.tone.set_lowpass(Self::tone_hz(v[4]), self.sample_rate);
        }
        if v[5] != last[5] {
            self.level.set(curve(1.5, 0.0, 151, v[5], 0));
        }
        self.last = v;
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        self.line.write(mean);

        let [p0, p1] = &mut self.phases;
        *p0 = p0.wrapping_add(self.increments[0]);
        *p1 = p1.wrapping_add(self.increments[1]);
        let first = triangle(*p0);
        let curve = (1.0 - first * first) * FIRST_AMPLITUDE;
        let envelope = 1.0 - 2.0 * ((triangle(*p1) - 0.25).abs() * ENVELOPE_SCALE);
        let (glide_in, glide_pole) = self.glide;
        self.depth = self.depth * glide_pole + self.depth_target * glide_in;
        self.center = self.center * glide_pole + self.center_target * glide_in;
        let driven = curve * envelope * self.depth + self.center;
        let (smooth_in, smooth_pole) = self.smoothing;
        self.delay = driven * smooth_in + self.delay * smooth_pole;

        let tap = self.line.read_linear(self.delay);
        let toned = self
            .tone
            .process(self.low_pass.process(self.high_pass.process(tap)));
        let out = self.level.tick() * (mean + self.wet.tick() * toned);
        [out, out]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.high_pass.clear();
        self.low_pass.clear();
        self.tone.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dsp::gain_to_db, fx::chorus_std::common::increment_hz};

    #[test]
    fn tone_interpolates_the_frequency_table() {
        assert_eq!(MirageCho::tone_hz(0.0), 400.0);
        assert_eq!(MirageCho::tone_hz(55.0), 5400.0);
        assert_eq!(MirageCho::tone_hz(100.0), 16_000.0);
        let mut effect = MirageCho::new(48_000.0);
        let mut values = defaults(&INFO);
        values[4] = 30.0;
        effect.set(&values, 120.0);
        assert!((gain_to_db(effect.tone.magnitude(48_000.0, 2000.0)) + 3.01).abs() < 0.05);
    }

    #[test]
    fn rates_follow_the_editor() {
        let mut effect = MirageCho::new(48_000.0);
        let mut values = defaults(&INFO);
        values[1] = 100.0;
        effect.set(&values, 120.0);
        assert!((increment_hz(effect.increments[0], 48_000.0) - 2.908).abs() < 0.01);
        assert!((increment_hz(effect.increments[1], 48_000.0) - 1.0).abs() < 0.01);
        assert!(effect.smoothing.1 > 0.99);
    }

    #[test]
    fn output_is_mono_and_delay_stays_in_range() {
        let mut effect = MirageCho::new(44_100.0);
        let mut range = (f32::MAX, 0.0f32);
        for i in 0..441_000 {
            let x = (i as f32 * 0.03).sin() * 0.3;
            let [l, r] = effect.process([x, -0.5 * x]);
            assert_eq!(l, r);
            range = (range.0.min(effect.delay), range.1.max(effect.delay));
        }
        // PreD 10 → 11 ms center; Depth 60 → 6.4 ms at most half used.
        assert!(
            range.0 >= 485.0 - 1.0 && range.1 <= 485.0 + 0.5 * 282.0 + 1.0,
            "{range:?}"
        );
        assert!(range.1 - range.0 > 50.0, "{range:?}");
    }
}
