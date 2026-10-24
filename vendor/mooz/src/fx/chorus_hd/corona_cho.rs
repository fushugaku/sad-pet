//! CoronaCho (CRN_CHO.ZDL), a TC Corona style stereo chorus.
//!
//! Callback `Fx_MOD_CoronaCho`, per channel: two ROM front biquads (also the
//! dry path, see [`corona`](super::corona)) -> pre-write filters (identities in
//! the ROM, not ported) -> ring -> one linear tap at
//! `base + wave * depth` -> tone shelf -> `FxLVL`. Both channels share one LFO
//! (the phase offset c20 is 0 and no editor sets it).
//!
//! LFO: a Q32 phase; with `t` the folded phase as a triangle in -1..1 the wave
//! is `0.5 + 0.5 (0.25 t + 0.75 t (2 - |t|))` (c21/c22: triangle blended with
//! its parabolic sine), smoothed by `c24/c25` (1/0 in the ROM). The delay is
//! a Q21 sample count: `base` (c30, smoothed per sample by c28/c29) plus the
//! wave times `depth` (c31, smoothed once per 8-sample block by c26/c27).
//!
//! Editors: `Speed_edit` sets the increment (0.05..10 Hz) and the base delay
//! (1.68..2.29 ms) from 11-point `TBL_TO_VAL_int` tables; `depth_edit`
//! interpolates a 26-point table (DEPTH/4, so the sweep grows from 0.1 ms to
//! 17.5 ms at the top); `mix_edit` FxLVL table; `tone_edit` shelf gain table;
//! `outLv_edit` is a constant 1.0; `KillDry_edit` the DRY switch.
use super::common::{fold, rescale_increment, rescale_pole, table};
use super::corona::{Front, Output, front_filters};
use crate::dsp::DelayLine;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_corona_cho",
    name: "CoronaCho",
    category: Category::Modulation,
    zdl: "CRN_CHO.ZDL",
    description: "TC Corona style chorus with tone and dry kill",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "SPEED",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "DEPTH",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "FxLVL",
            max: 100,
            default: 65,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "TONE",
            max: 100,
            default: 75,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "DRY",
            max: 1,
            default: 1,
            display: Display::OFF_ON,
        },
    ],
};

/// CRN_CHO 0x80000798: Q32 phase increments at 44.1 kHz.
const SPEED_INCREMENT: [f32; 11] = [
    5356.0, 9739.0, 24347.0, 54636.0, 116_869.0, 165_565.0, 194_783.0, 243_478.0, 292_174.0,
    681_740.0, 973_915.0,
];
/// CRN_CHO 0x80000768: base delay, Q21 samples at 44.1 kHz.
const SPEED_BASE: [f32; 11] = [
    155_340_528.0,
    155_340_528.0,
    155_340_528.0,
    157_469_748.0,
    157_469_748.0,
    159_506_394.0,
    161_635_615.0,
    163_764_835.0,
    180_520_876.0,
    203_664_578.0,
    211_996_310.0,
];
/// CRN_CHO 0x800006d0: Q31 depth, one entry per 4 DEPTH steps.
const DEPTH: [f32; 26] = [
    9_257_480.0,
    23_699_150.0,
    30_438_596.0,
    42_954_711.0,
    57_396_381.0,
    71_838_051.0,
    81_465_831.0,
    88_205_277.0,
    93_019_167.0,
    95_907_501.0,
    103_609_725.0,
    107_460_837.0,
    111_311_949.0,
    119_014_173.0,
    130_567_509.0,
    142_120_845.0,
    161_376_405.0,
    163_301_961.0,
    176_780_853.0,
    298_090_882.0,
    596_552_064.0,
    827_618_785.0,
    1_000_918_826.0,
    1_270_496_668.0,
    1_540_074_509.0,
    1_620_059_144.0,
];
/// CRN_CHO 0x80000738: FxLVL gains.
const FX_LEVEL: [f32; 11] = [
    0.0, 0.05, 0.1, 0.16, 0.28, 0.44, 0.544, 0.678, 0.753, 0.803, 1.05,
];
/// CRN_CHO 0x800007c8: TONE shelf gains in dB.
const TONE_DB: [f32; 11] = [
    -60.0, -50.0, -40.0, -30.0, -25.0, -25.0, -18.0, -13.0, -10.0, -5.0, 0.0,
];
const Q21: f32 = 1.0 / 2_097_152.0;
const Q31: f32 = 1.0 / 2_147_483_648.0;

struct CoronaCho {
    sample_rate: f32,
    front: [Front; 2],
    lines: [DelayLine; 2],
    output: Output,
    phase: u32,
    increment: u32,
    /// Sweep and base in instance samples, with their targets.
    depth: f32,
    depth_target: f32,
    depth_smoothing: f32,
    base: f32,
    base_target: f32,
    base_smoothing: f32,
    last: Values,
    fresh: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let front = Front::new(front_filters(sample_rate));
    // c26/c27 smooth once per 8 samples at 44.1 kHz.
    let per_block = 613_296.0 / 2_147_483_648.0_f64;
    let per_sample = 1.0 - (1.0 - per_block).powf(1.0 / 8.0);
    let mut effect = CoronaCho {
        sample_rate,
        front: [front; 2],
        lines: [
            DelayLine::with_seconds(0.024, sample_rate),
            DelayLine::with_seconds(0.024, sample_rate),
        ],
        output: Output::new(sample_rate),
        phase: 0,
        increment: 0,
        depth: 0.0,
        depth_target: 0.0,
        depth_smoothing: rescale_pole(per_sample as f32, sample_rate),
        base: 0.0,
        base_target: 0.0,
        base_smoothing: rescale_pole(398_662.0 / 2_147_483_648.0, sample_rate),
        last: [f32::NAN; crate::MAX_PARAMS],
        fresh: true,
    };
    effect.set(&crate::Model::CoronaCho.defaults(), 120.0);
    Box::new(effect)
}

/// The c21/c22 wave of a phase word, 0..1.
#[inline]
fn wave(phase: u32) -> f32 {
    let t = 2.0 * fold(phase) - 1.0;
    0.5 + 0.5 * (0.250_009_3 * t + 0.749_990_7 * t * (2.0 - t.abs()))
}

impl Effect for CoronaCho {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.fresh;
        self.fresh = false;
        let scale = self.sample_rate / crate::zoom::ORIGINAL_RATE;
        if values[0] != self.last[0] {
            let speed = values[0];
            self.increment =
                rescale_increment(table(&SPEED_INCREMENT, speed, 100.0), self.sample_rate);
            self.base_target = table(&SPEED_BASE, speed, 100.0) * Q21 * scale;
        }
        if values[1] != self.last[1] {
            // A Q31 fraction of the Q21 sample scale: 1024 samples at full depth.
            self.depth_target = table(&DEPTH, values[1], 100.0) * Q31 * 1024.0 * scale;
        }
        if values[3] != self.last[3] {
            self.output.set_tone(&TONE_DB, values[3]);
        }
        if values[2] != self.last[2] || values[4] != self.last[4] {
            let fx = table(&FX_LEVEL, values[2], 100.0);
            self.output.set_levels(fx, values[4].round() >= 1.0, jump);
        }
        if jump {
            self.depth = self.depth_target;
            self.base = self.base_target;
        }
        self.last = *values;
    }

    fn process(&mut self, [l, r]: [f32; 2]) -> [f32; 2] {
        let front = [self.front[0].process(l), self.front[1].process(r)];
        self.lines[0].write(front[0]);
        self.lines[1].write(front[1]);
        self.phase = self.phase.wrapping_add(self.increment);
        self.depth += (self.depth_target - self.depth) * self.depth_smoothing;
        self.base += (self.base_target - self.base) * self.base_smoothing;
        let delay = self.base + wave(self.phase) * self.depth;
        let wet = [
            self.lines[0].read_linear(delay),
            self.lines[1].read_linear(delay),
        ];
        self.output.mix(front, wet)
    }

    fn clear(&mut self) {
        self.front.iter_mut().for_each(Front::clear);
        self.lines.iter_mut().for_each(DelayLine::clear);
        self.output.clear();
        self.phase = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn wave_spans_zero_to_one() {
        assert!(wave(0).abs() < 1e-6);
        assert!((wave(0x8000_0000) - 1.0).abs() < 1e-6);
        assert!((wave(0x4000_0000) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn default_speed_and_delays() {
        let hz = table(&SPEED_INCREMENT, 50.0, 100.0) * 44_100.0 / 4_294_967_296.0;
        assert!((hz - 1.70).abs() < 0.01, "{hz}");
        let base_ms = table(&SPEED_BASE, 50.0, 100.0) * Q21 / 44.1;
        assert!((base_ms - 1.725).abs() < 0.01, "{base_ms}");
        let sweep_ms = table(&DEPTH, 100.0, 100.0) * Q31 * 1024.0 / 44.1;
        assert!((sweep_ms - 17.5).abs() < 0.1, "{sweep_ms}");
    }

    #[test]
    fn dry_switch_kills_the_front_signal() {
        let mut effect = create(48_000.0);
        let mut values = Model::CoronaCho.defaults();
        values[2] = 0.0;
        values[4] = 0.0;
        effect.set(&values, 120.0);
        let mut out = [1.0; 2];
        for _ in 0..4800 {
            out = effect.process([0.5, 0.5]);
        }
        assert!(out[0].abs() < 1e-4 && out[1].abs() < 1e-4, "{out:?}");
        values[4] = 1.0;
        effect.set(&values, 120.0);
        for _ in 0..4800 {
            out = effect.process([0.5, 0.5]);
        }
        assert!((out[0] - 0.5).abs() < 0.01, "{out:?}");
    }
}
