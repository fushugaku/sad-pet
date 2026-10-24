//! CombFLTR (COMBFLTR.ZDL): feedback comb filter added to the dry signal.
//!
//! Callback `Fx_FLT_CombFLTR` (04a0..0800). Its Q31 prepass is the flanger
//! LFO of the shared code with zero rate and depth (c18 = c23 = 0), so the
//! tap is just `c8 + offset` with `offset = c21 c24 + c22 offset` (Q31
//! one-pole, ~2.7 ms) in 2^-22 sample units: c8 = 5, shift c10 = 22, fraction
//! shift c11 = 10 (set by `init`). Per sample:
//! - `m = (L + R) / 2`; `in = c9 (c13 s[n-1] + m)`; first-order low-pass
//!   `fb = c14 in + c15 in[n-1] + c16 fb[n-1]` (HiDMP); write `fb c17`;
//! - linear-interpolated tap `s` at the distance above (0 = the sample just
//!   written), `s *= c3`; `out = c2 (c7 x + c6 c9 s)` per channel (c0 = 1).
//!
//! Editors decoded from the disassembly:
//! - Freq (`freq_edit`): c24 = Q31 `0x17c48353 (50 - Freq) / 50` (0..95.07
//!   samples), plus 11 samples when Reso is below 0 on the display, through
//!   Main's Q31 ramp (`context[10]`, 0x000f7adf per sample = 0.242
//!   samples/sample).
//! - Reso (`reso_edit`): c13 = 0x80000478 table (-0.95..+0.95, 0 at the
//!   centre) with the 0x3af7c5ec ramp; re-runs Freq and Mix.
//! - Mix (`mix_edit`): c7 = 1 + 0.1332 Mix / 100, c6 = 1.3999 Mix / 100,
//!   negated for negative Reso (default ramp).
//! - HiDMP (`hidamp_edit`): (c14, c15, c16) from 0x80000520, a bilinear
//!   first-order low-pass from ~250 Hz (0) to ~20 kHz (10).
//! - Level: c2 = Level / 100.
//!
//! At other rates the HiDMP sections are re-derived through their analog
//! prototypes and the tap keeps the loop period in time:
//! `(d44 + 1) r - 1`, `r = fs / 44100`. The editors' c17 fade around a HiDMP
//! change is not reproduced (the low-pass coefficients switch directly).
use super::common::{FirstOrder, STEP_DEFAULT, STEP_FAST, clamp_value, step, w};
use crate::dsp::{DelayLine, flush};
use crate::zoom::{ORIGINAL_RATE, Ramp, curve};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_comb_fltr",
    name: "CombFLTR",
    category: Category::Filter,
    zdl: "COMBFLTR.ZDL",
    description: "Comb filter with feedback blended into the dry signal",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Freq",
            max: 49,
            default: 24,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Reso",
            max: 20,
            default: 15,
            display: Display::offset(-10.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 30,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "HiDMP",
            max: 10,
            default: 6,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// 0x80000478 (second word of each pair; the first, c5, is unused): c13.
const FEEDBACK: [u32; 21] = [
    0xbf7331e6, 0xbf5f49bf, 0xbf4b5f97, 0xbf37776f, 0xbf238d47, 0xbf0fa31f, 0xbef775ef, 0xbecfa19f,
    0xbea7d150, 0xbe7ffa00, 0x00000000, 0x3e7ffa00, 0x3ea7d150, 0x3ecfa19f, 0x3ef775ef, 0x3f0fa31f,
    0x3f238d47, 0x3f37776f, 0x3f4b5f97, 0x3f5f49bf, 0x3f7331e6,
];

/// 0x80000520: c14, c15, c16 per HiDMP.
const HIDAMP: [[u32; 3]; 11] = [
    [0x3c8f5b79, 0x3c8f5b79, 0x3f770a48],
    [0x3d0cef57, 0x3d0cef57, 0x3f6e6215],
    [0x3d88689d, 0x3d88689d, 0x3f5de5d9],
    [0x3e00767d, 0x3e00767d, 0x3f3fc4c2],
    [0x3e67ff0a, 0x3e67ff0a, 0x3f0c007b],
    [0x3e8ad692, 0x3e8ad692, 0x3eea52dc],
    [0x3ea03bca, 0x3ea03bca, 0x3ebf886c],
    [0x3eb48431, 0x3eb48431, 0x3e96f79f],
    [0x3ec7f697, 0x3ec7f697, 0x3e6025a5],
    [0x3eed461d, 0x3eed461d, 0x3d95cf1b],
    [0x3f5f2ca7, 0x3f5f2ca7, 0xbf3e594e],
];

/// c8: fixed part of the tap, in samples.
const BASE: f32 = 5.0;
/// Longest tap: 5 + 95.07 + 11 samples.
const MAX_TAP: f32 = 112.0;
/// Q31 words of the offset smoother and the c24 ramp, as plain numbers.
const SMOOTH_IN: f32 = 0x0117_0000 as f32 / 2_147_483_648.0;
const SMOOTH_POLE: f32 = 0x7ee8_0000 as f32 / 2_147_483_648.0;
const SWEEP: f32 = 0x17c4_8353 as f32 / 4_194_304.0;
const SLEW: f32 = 0x000f_7adf as f32 / 4_194_304.0;

struct CombFltr {
    r: f32,
    hidamp: [FirstOrder; 11],
    damp: FirstOrder,
    smooth_in: f32,
    smooth_pole: f32,
    slew: f32,
    target: f32,
    c24: f32,
    offset: f32,
    c13: Ramp,
    c6: Ramp,
    c7: Ramp,
    level: Ramp,
    line: DelayLine,
    previous_in: f32,
    previous_fb: f32,
    tap: f32,
    fresh: bool,
    last: Option<Values>,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let r = sample_rate / ORIGINAL_RATE;
    let hidamp = HIDAMP.map(|[b0, b1, a]| FirstOrder::rom(w(b0), w(b1), w(a), sample_rate));
    let smooth_pole = super::common::decay(SMOOTH_POLE, sample_rate);
    let mut effect = CombFltr {
        r,
        hidamp,
        damp: hidamp[6],
        smooth_in: SMOOTH_IN * (1.0 - smooth_pole) / (1.0 - SMOOTH_POLE),
        smooth_pole,
        slew: SLEW / r,
        target: 0.0,
        c24: 0.0,
        offset: 0.0,
        c13: Ramp::new(0.0, STEP_FAST, sample_rate),
        c6: Ramp::new(0.0, STEP_DEFAULT, sample_rate),
        c7: Ramp::new(1.0, STEP_DEFAULT, sample_rate),
        level: Ramp::new(1.0, STEP_DEFAULT, sample_rate),
        line: DelayLine::new(((MAX_TAP + 1.0) * r) as usize + 4),
        previous_in: 0.0,
        previous_fb: 0.0,
        tap: 0.0,
        fresh: true,
        last: None,
    };
    let mut values = [0.0; crate::MAX_PARAMS];
    for (value, param) in values.iter_mut().zip(INFO.params) {
        *value = f32::from(param.default);
    }
    effect.set(&values, 120.0);
    Box::new(effect)
}

impl Effect for CombFltr {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.last.as_ref() == Some(values) {
            return;
        }
        self.last = Some(*values);
        let freq = clamp_value(values[0], 49.0);
        let reso = step(values[1], 20);
        let mix = clamp_value(values[2], 100.0);
        let negative = reso < 10;
        let target = SWEEP * (50.0 - freq) / 50.0 + if negative { 11.0 } else { 0.0 };
        let c13 = f32::from_bits(FEEDBACK[reso]);
        let c7 = curve(w(0x3f91_0c00), 1.0, 101, mix, 0);
        let c6 = curve(w(0x3fb3_3000), 0.0, 101, mix, 0) * if negative { -1.0 } else { 1.0 };
        let level = clamp_value(values[4], 150.0) / 100.0;
        self.damp = self.hidamp[step(values[3], 10)];
        self.target = target;
        if self.fresh {
            self.fresh = false;
            self.c24 = target;
            self.offset = target * self.smooth_in / (1.0 - self.smooth_pole);
            self.c13.jump(c13);
            self.c6.jump(c6);
            self.c7.jump(c7);
            self.level.jump(level);
        } else {
            self.c13.set(c13);
            self.c6.set(c6);
            self.c7.set(c7);
            self.level.set(level);
        }
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        let difference = self.target - self.c24;
        self.c24 += difference.clamp(-self.slew, self.slew);
        self.offset = self.smooth_in * self.c24 + self.smooth_pole * self.offset;
        let c13 = self.c13.tick();
        let c6 = self.c6.tick();
        let c7 = self.c7.tick();
        let level = self.level.tick();
        let x = 0.5 * (frame[0] + frame[1]);
        let input = c13 * self.tap + x;
        let d = self.damp;
        let fb = flush(d.b0 * input + d.b1 * self.previous_in + d.a * self.previous_fb);
        self.previous_in = input;
        self.previous_fb = fb;
        self.line.write(fb);
        let distance = (BASE + self.offset + 1.0) * self.r - 1.0;
        self.tap = flush(self.line.read_linear(distance));
        let wet = c6 * self.tap;
        [(c7 * frame[0] + wet) * level, (c7 * frame[1] + wet) * level]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.previous_in = 0.0;
        self.previous_fb = 0.0;
        self.tap = 0.0;
        self.fresh = true;
        self.last = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    fn impulse_echoes(values: &Values, sample_rate: f32) -> Vec<f32> {
        let mut effect = create(sample_rate);
        effect.set(values, 120.0);
        effect.clear();
        effect.set(values, 120.0);
        (0..4000)
            .map(|i| {
                let x = if i == 0 { 1.0 } else { 0.0 };
                effect.process([x, x])[0]
            })
            .collect()
    }

    fn strongest_after(signal: &[f32], start: usize) -> usize {
        (start..start + 400)
            .max_by(|&a, &b| signal[a].abs().total_cmp(&signal[b].abs()))
            .unwrap()
    }

    #[test]
    fn freq_sets_the_comb_period() {
        // Freq 1 (value 0), Reso +10, HiDMP 10: loop period 5 + 95.07 * 0.9964
        // + 1 = 100.8 samples at 44.1 kHz, scaled with the rate.
        let mut values = Model::CombFltr.defaults();
        values[0] = 0.0;
        values[1] = 20.0;
        values[2] = 100.0;
        values[3] = 10.0;
        for sample_rate in [44_100.0f32, 96_000.0] {
            let out = impulse_echoes(&values, sample_rate);
            let first = strongest_after(&out, 3);
            let second = strongest_after(&out, first + 10);
            let period = (second - first) as f32 * 44_100.0 / sample_rate;
            assert!((period - 100.8).abs() < 1.5, "{sample_rate}: {period}");
        }
    }

    #[test]
    fn negative_reso_inverts_the_comb() {
        let mut values = Model::CombFltr.defaults();
        values[1] = 0.0;
        values[3] = 10.0;
        let out = impulse_echoes(&values, 44_100.0);
        let first = strongest_after(&out, 3);
        assert!(out[first] < 0.0, "{}", out[first]);
        values[1] = 20.0;
        let out = impulse_echoes(&values, 44_100.0);
        let first = strongest_after(&out, 3);
        assert!(out[first] > 0.0, "{}", out[first]);
    }

    #[test]
    fn mix_zero_is_dry() {
        let mut values = Model::CombFltr.defaults();
        values[2] = 0.0;
        let out = impulse_echoes(&values, 48_000.0);
        assert!((out[0] - 1.0).abs() < 1e-6);
        assert!(out[1..].iter().all(|x| x.abs() < 1e-6));
    }

    #[test]
    fn hidamp_is_a_low_pass_with_unity_dc() {
        for set in HIDAMP {
            let [b0, b1, a] = set.map(w);
            let at96 = FirstOrder::rom(b0, b1, a, 96_000.0);
            assert!(((at96.b0 + at96.b1) / (1.0 - at96.a) - 1.0).abs() < 1e-3);
        }
    }
}
