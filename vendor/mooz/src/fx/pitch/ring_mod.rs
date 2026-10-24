//! RingMod (RINGMOD.ZDL): ring modulator with a soft square carrier.
//!
//! Callback: recovered `ring_modulator.rs`. The carrier phase rate glides
//! toward ⅔·c5 (c6/c7 one-pole, ~50 ms); the triangle `2|phase| - 1` is
//! scaled by 16·c9 and saturated (a trapezoid, nearly square), then smoothed
//! by the c14/c15 one-pole (~2.3 kHz). Mono (L+R)/2 → 60 Hz first-order
//! high-pass (c16..c18) → × carrier → Tone biquad (c19..c23) → c25; `out =
//! c2 (c24 in + wet)`.
//!
//! Editors decoded: `freq_edit`: Q31 `context[22]` square curve 62..3050,
//! c5 = value << 16, i.e. about 27.8 Hz .. 1.37 kHz (Freq shown 1..50);
//! `tone_edit` copies `RING_MOD_TONE` (step 10 is flat); `mix_edit`: c24 =
//! two-slope(100 - Bal), c25 = two-slope(Bal), `context[8]` step 0x3af7c5ec;
//! `outlv_edit` c2.
use super::{
    common::{decay, one_pole, out_level, rom_biquad, rom_first_order, step, two_slope},
    rom::{RING_MOD_ROM, RING_MOD_TONE, q31},
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, flush},
    zoom::{DEFAULT_STEP, ORIGINAL_RATE, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ring_mod",
    name: "RingMod",
    category: Category::Modulation,
    zdl: "RINGMOD.ZDL",
    description: "Ring modulator, 28 Hz to 1.4 kHz carrier",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Freq",
            max: 49,
            default: 27,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 10,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Level",
            max: 150,
            default: 120,
            display: Display::INT,
        },
    ],
};

/// The carrier frequency of a Freq step in Hz.
pub fn carrier_hz(freq: usize) -> f32 {
    let t = freq.min(49) as f32 / 49.0;
    let units = 62 + (2_988.0 * t * t) as u32;
    // Steady rate ⅔·c5 per 44.1 kHz sample, c5 = units << 16.
    units as f32 * 65_536.0 * (2.0 / 3.0) / 4_294_967_296.0 * ORIGINAL_RATE
}

struct RingMod {
    sample_rate: f32,
    glide: f32,
    rate: f32,
    target: f32,
    started: bool,
    phase: u32,
    slope: f32,
    smoothing: f32,
    carrier: f32,
    input: Biquad,
    tone: Biquad,
    tone_step: Option<usize>,
    dry: Ramp,
    wet: Ramp,
    level: Ramp,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let mix_step = f32::from_bits(0x3af7_c5ec);
    Box::new(RingMod {
        sample_rate,
        glide: 1.0 - decay(q31(RING_MOD_ROM[7]), sample_rate),
        rate: 0.0,
        target: 0.0,
        started: false,
        phase: 0,
        slope: 16.0 * q31(RING_MOD_ROM[9]),
        smoothing: one_pole(q31(RING_MOD_ROM[14]), sample_rate),
        carrier: 0.0,
        input: Biquad::new(rom_first_order(
            &RING_MOD_ROM[16..19],
            ORIGINAL_RATE,
            sample_rate,
        )),
        tone: Biquad::default(),
        tone_step: None,
        dry: Ramp::new(two_slope(50.0), mix_step, sample_rate),
        wet: Ramp::new(two_slope(50.0), mix_step, sample_rate),
        level: Ramp::new(1.2, DEFAULT_STEP, sample_rate),
    })
}

impl Effect for RingMod {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.target = carrier_hz(step(values[0], 49)) / self.sample_rate * 4_294_967_296.0;
        if !self.started {
            self.started = true;
            self.rate = self.target;
        }
        let tone = step(values[1], 10);
        if self.tone_step != Some(tone) {
            let words = &RING_MOD_TONE[5 * tone..5 * tone + 5];
            self.tone.coefficients = rom_biquad(words, ORIGINAL_RATE, self.sample_rate);
            self.tone_step = Some(tone);
        }
        self.dry.set(two_slope(100.0 - values[2]));
        self.wet.set(two_slope(values[2]));
        self.level.set(out_level(values[3]));
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.rate += self.glide * (self.target - self.rate);
        self.phase = self.phase.wrapping_add(self.rate as u32);
        let p = (self.phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0);
        let square = ((2.0 * p - 1.0) * self.slope).clamp(-1.0, 1.0);
        self.carrier = flush(self.carrier + self.smoothing * (square - self.carrier));
        let filtered = self.input.process(0.5 * (left + right));
        let wet = self.tone.process(filtered * self.carrier) * self.wet.tick();
        let (dry, level) = (self.dry.tick(), self.level.tick());
        [level * (dry * left + wet), level * (dry * right + wet)]
    }

    fn clear(&mut self) {
        self.input.clear();
        self.tone.clear();
        self.carrier = 0.0;
        self.phase = 0;
        self.rate = self.target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fx::pitch::two_tap::tests::centroid;

    #[test]
    fn carrier_range_follows_the_freq_curve() {
        assert!((carrier_hz(0) - 27.81).abs() < 0.05);
        assert!((carrier_hz(49) - 1_368.3).abs() < 0.5);
        assert!((carrier_hz(27) - 434.7).abs() < 0.5);
    }

    #[test]
    fn makes_sum_and_difference_tones() {
        let sample_rate = 48_000.0;
        let mut effect = create(sample_rate);
        let mut values = crate::Model::RingMod.defaults();
        values[2] = 100.0;
        effect.set(&values, 120.0);
        let carrier = carrier_hz(27);
        let n = sample_rate as usize;
        let out: Vec<f32> = (0..n)
            .map(|i| {
                let x = 0.5 * (i as f32 * 1_000.0 / sample_rate * std::f32::consts::TAU).sin();
                effect.process([x, x])[0]
            })
            .collect();
        let lower = centroid(
            &out[n / 2..],
            sample_rate,
            1_000.0 - carrier - 20.0,
            1_000.0 - carrier + 20.0,
        );
        assert!((lower - (1_000.0 - carrier)).abs() < 2.0, "{lower}");
        let upper = centroid(
            &out[n / 2..],
            sample_rate,
            1_000.0 + carrier - 20.0,
            1_000.0 + carrier + 20.0,
        );
        assert!((upper - (1_000.0 + carrier)).abs() < 2.0, "{upper}");
    }
}
