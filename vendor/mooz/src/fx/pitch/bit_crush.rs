//! BitCrush (BITCRUSH.ZDL): bit depth and sample rate reduction.
//!
//! Callback: recovered `bit_crusher.rs`. Mono (L+R)/2 × 1.3 (c5) is held for
//! SMPL+1 samples (c9; at other host rates the hold keeps its 44.1 kHz
//! duration through a fractional counter), converted to saturated Q31, its
//! magnitude masked to Bit+4 bits (c8, sign kept: truncation toward zero),
//! then two cascaded Tone one-poles (c10/c11). `out = c2 (c7 in + c6 wet)`.
//!
//! Editors decoded: Bit = `BIT_CRUSH_MASK`, SMPL = c9 = SMPL + 1, Tone =
//! `BIT_CRUSH_TONE`, Bal = two-slope (c7 dry from 100 - Bal, c6 wet) with
//! `context[8]` ramps, Out_Lv c2.
use super::{
    common::{one_pole, out_level, step, two_slope},
    rom::{BIT_CRUSH_MASK, BIT_CRUSH_TONE, f},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::flush,
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_bit_crush",
    name: "BitCrush",
    category: Category::Drive,
    zdl: "BITCRUSH.ZDL",
    description: "Lo-fi bit depth and sample rate reduction",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Bit",
            max: 12,
            default: 5,
            display: Display::offset(4.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "SMPL",
            max: 50,
            default: 2,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 90,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 8,
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

/// Input gain c5 (0x3fa66666).
const DRIVE: f32 = 1.3;

/// Sign-magnitude Q31 truncation of `x` to the bits of `mask`.
#[inline]
fn crush(x: f32, mask: u32) -> f32 {
    let magnitude = (x.abs().min(1.0) * 2_147_483_648.0) as u32;
    let kept = magnitude.min(0x7fff_ffff) & mask;
    let value = kept as f32 * (1.0 / 2_147_483_648.0);
    if x < 0.0 { -value } else { value }
}

struct BitCrush {
    sample_rate: f32,
    mask: u32,
    hold: f32,
    counter: f32,
    advance: f32,
    held: f32,
    tone: f32,
    poles: [f32; 2],
    dry: Ramp,
    wet: Ramp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(BitCrush::new(sample_rate))
}

impl BitCrush {
    fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            mask: BIT_CRUSH_MASK[5],
            hold: 3.0,
            counter: 0.0,
            advance: ORIGINAL_RATE / sample_rate,
            held: 0.0,
            tone: 1.0,
            poles: [0.0; 2],
            dry: Ramp::new(two_slope(10.0), DEFAULT_STEP, sample_rate),
            wet: Ramp::new(two_slope(90.0), DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.0, DEFAULT_STEP, sample_rate),
        }
    }
}

impl Effect for BitCrush {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.mask = BIT_CRUSH_MASK[step(values[0], 12)];
        self.hold = (step(values[1], 50) + 1) as f32;
        self.dry.set(two_slope(100.0 - values[2]));
        self.wet.set(two_slope(values[2]));
        self.tone = one_pole(f(BIT_CRUSH_TONE[step(values[3], 10)]), self.sample_rate);
        self.level.set(out_level(values[4]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.counter += self.advance;
        if self.counter >= self.hold {
            self.counter -= self.hold;
            self.counter = self.counter.min(self.hold);
            self.held = 0.5 * (left + right) * DRIVE;
        }
        let crushed = crush(self.held, self.mask);
        self.poles[0] = flush(self.poles[0] + self.tone * (crushed - self.poles[0]));
        self.poles[1] = flush(self.poles[1] + self.tone * (self.poles[0] - self.poles[1]));
        let wet = self.poles[1] * self.wet.tick();
        let (dry, level) = (self.dry.tick(), self.level.tick());
        [level * (dry * left + wet), level * (dry * right + wet)]
    }

    fn clear(&mut self) {
        self.counter = 0.0;
        self.held = 0.0;
        self.poles = [0.0; 2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_keep_bit_plus_four_bits() {
        // 4 bits: sign + 3 magnitude bits, steps of 1/8.
        assert_eq!(crush(0.3, BIT_CRUSH_MASK[0]), 0.25);
        assert_eq!(crush(-0.3, BIT_CRUSH_MASK[0]), -0.25);
        assert_eq!(crush(2.0, BIT_CRUSH_MASK[0]), 0.875);
        let fine = crush(0.123_456, BIT_CRUSH_MASK[12]);
        assert!((fine - 0.123_456).abs() < 1.0 / 32_768.0);
    }

    #[test]
    fn holds_for_smpl_plus_one_samples() {
        for sample_rate in [44_100.0f32, 88_200.0] {
            let mut effect = BitCrush::new(sample_rate);
            let mut values = crate::Model::BitCrush.defaults();
            values[1] = 4.0;
            values[0] = 12.0;
            effect.set(&values, 120.0);
            let mut changes = 0i32;
            let mut last = f32::NAN;
            let n = sample_rate as usize / 10;
            for i in 0..n {
                effect.process([i as f32 / n as f32 * 0.5; 2]);
                if effect.held != last {
                    changes += 1;
                }
                last = effect.held;
            }
            let expected = 4_410 / 5;
            assert!((changes - expected).abs() <= 2, "{sample_rate}: {changes}");
        }
    }
}
