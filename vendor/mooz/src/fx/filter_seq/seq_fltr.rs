//! SeqFLTR (SEQFLTR.ZDL, `Fx_FLT_SeqFLTR`): a step sequencer driving a
//! resonant filter.
//!
//! Callback topology, followed as recovered: a 2^32 phase counter advances
//! the step on its signed rollover and wraps after `Step` steps; the step's
//! frequency and resonance words (`Fc_tbl`/`Q_tbl`, 7 lengths × 8 patterns ×
//! 8 steps) glide through the Shape one-pole (`c25`/`c26`) into a two-stage
//! clipped Chamberlin filter (input ×0.5, damping `c31 + q * c32`, output
//! `0.8 low + 1.884 band + 0.15 high`, every node clipped to ±1). A tenth of
//! the dry mean is added before the ROM DC-blocking biquad (`c36..c40`,
//! re-derived for the host rate); the mono result times Level feeds both
//! channels.
//!
//! Editors decoded from the disassembly: Step/PTTRN (whole table copy), Speed
//! (square integer curve 162319..1947831 per 2^32, 1.67..20 steps/s; synced
//! values use the Main LFO divisions), Shape (`c26` = power-1.41 curve
//! 0.999..0.9, `c25 = 1 - c26`), Reso (`c31`/`c32` pairs from ROM
//! `0x80001480`, 16 ms timed ramp) and Level (`value / 100`). The table
//! frequency words are converted to the same cutoffs at the host rate.
use super::common::{self, Changes, ClippedSvf, TimedRamp, clip};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, Coefficients, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_seq_fltr",
    name: "SeqFLTR",
    category: Category::Filter,
    zdl: "SEQFLTR.ZDL",
    description: "Step-sequenced resonant filter pattern",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Step",
            max: 6,
            default: 6,
            display: Display::offset(2.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "PTTRN",
            max: 7,
            default: 6,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Speed",
            max: 77,
            default: 25,
            display: Display::sync_rate(49, 1.0),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Shape",
            max: 10,
            default: 10,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Reso",
            max: 10,
            default: 10,
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

/// ROM `0x80001480`: Reso → (`c31` damping base, `c32` resonance-word scale).
const RESO: [(u32, u32); 11] = [
    (0x3fa00000, 0xbf155555),
    (0x3f93205e, 0xbf0b19f3),
    (0x3f882b93, 0xbf021e43),
    (0x3f7d7721, 0xbef45dd9),
    (0x3f6d097b, 0xbee643ba),
    (0x3f5e9bd3, 0xbed9a96e),
    (0x3f51d608, 0xbece56bb),
    (0x3f46731a, 0xbec41d6b),
    (0x3f3c3c3c, 0xbebad749),
    (0x3f33055f, 0xbeb26491),
    (0x3f2aaaab, 0xbeaaaaab),
];
/// `c28`: filter input gain; `c33`, `c34`, `c35`: high, band and low weights.
const INPUT: f32 = 0.5;
const HIGH: f32 = f32::from_bits(0x3e19_999a);
const BAND: f32 = f32::from_bits(0x3ff1_241f);
const LOW: f32 = f32::from_bits(0x3f4c_cccd);
/// `c6`: dry mean added before the output biquad.
const DRY: f32 = f32::from_bits(0x3dcc_cccd);

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(SeqFltr::new(sample_rate))
}

/// Speed 0..49 → steps per second.
fn normal_speed_hz(value: f32) -> f32 {
    common::q32_hz(common::int_curve(1_947_831.0, 162_319.0, 50, value, 1))
}

struct SeqFltr {
    sample_rate: f32,
    changes: Changes,
    phase: u32,
    increment: u32,
    step: usize,
    steps: usize,
    frequency: [f32; 8],
    resonance: [f32; 8],
    glide: f32,
    f: f32,
    q: f32,
    base: TimedRamp,
    scale: TimedRamp,
    filters: [ClippedSvf; 2],
    output: Biquad,
    level: Ramp,
}

impl SeqFltr {
    fn new(sample_rate: f32) -> Self {
        let output = Biquad::new(
            Coefficients {
                b0: f32::from_bits(0x3f7d_ab99),
                b1: f32::from_bits(0xbffd_ab99),
                b2: f32::from_bits(0x3f7d_ab99),
                a1: -f32::from_bits(0x3ffd_a84d),
                a2: -f32::from_bits(0xbf7b_5dc9),
            }
            .resample(zoom::ORIGINAL_RATE, sample_rate),
        );
        Self {
            sample_rate,
            changes: Changes::new(),
            phase: 0,
            increment: 0,
            step: 0,
            steps: 8,
            frequency: [0.0; 8],
            resonance: [0.0; 8],
            glide: 1.0,
            f: 0.0,
            q: 0.0,
            base: TimedRamp::new(f32::from_bits(RESO[10].0), sample_rate),
            scale: TimedRamp::new(f32::from_bits(RESO[10].1), sample_rate),
            filters: [ClippedSvf::default(); 2],
            output,
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
        }
    }

    fn load_pattern(&mut self, length: usize, pattern: usize) {
        let start = length * 64 + pattern * 8;
        for i in 0..8 {
            self.frequency[i] =
                common::svf_f(f32::from_bits(FREQUENCY[start + i]), self.sample_rate);
            self.resonance[i] = f32::from_bits(RESONANCE[start + i]);
        }
        self.steps = length + 2;
    }
}

impl Effect for SeqFltr {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, tempo, first) = self.changes.update(values, bpm);
        if changed & 3 != 0 {
            self.load_pattern(common::index(values[0], 6), common::index(values[1], 7));
            if first {
                self.f = self.frequency[0];
                self.q = self.resonance[0];
            }
        }
        if changed & 4 != 0 || (tempo && zoom::is_synced(values[2], 49)) {
            let hz = common::sync_hz(values[2], 49, bpm, normal_speed_hz);
            self.increment = common::increment(hz, self.sample_rate);
        }
        if changed & 8 != 0 {
            let pole = zoom::curve(
                0.9,
                f32::from_bits(0x3f7f_be84),
                11,
                values[3].clamp(0.0, 10.0),
                5,
            );
            self.glide = 1.0 - common::pole(pole, self.sample_rate);
        }
        if changed & 16 != 0 {
            let reso = values[4].clamp(0.0, 10.0);
            let i = (reso as usize).min(9);
            let fraction = reso - i as f32;
            let lerp = |a: u32, b: u32| {
                let (a, b) = (f32::from_bits(a), f32::from_bits(b));
                a + (b - a) * fraction
            };
            let base = lerp(RESO[i].0, RESO[i + 1].0);
            let scale = lerp(RESO[i].1, RESO[i + 1].1);
            self.base.set(base);
            self.scale.set(scale);
            if first {
                self.base.jump(base);
                self.scale.jump(scale);
            }
        }
        if changed & 32 != 0 {
            self.level.set(values[5].max(0.0) / 100.0);
            if first {
                self.level.jump(values[5].max(0.0) / 100.0);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        let old = self.phase;
        self.phase = old.wrapping_add(self.increment);
        if (old as i32) > (self.phase as i32) {
            self.step += 1;
            if self.step >= self.steps {
                self.step = 0;
            }
        }
        let at = self.step.min(7);
        self.f = flush(self.f + (self.frequency[at] - self.f) * self.glide);
        self.q = flush(self.q + (self.resonance[at] - self.q) * self.glide);
        let damping = self.base.tick() + self.q * self.scale.tick();
        let (high, band, low) = self.filters[0].process(INPUT * mean, self.f, damping);
        let first = clip(low * LOW + band * BAND + high * HIGH);
        let (high, band, low) = self.filters[1].process(INPUT * first, self.f, damping);
        let second = clip(low * LOW + band * BAND + high * HIGH);
        let out = self.output.process(second + DRY * mean) * self.level.tick();
        [out, out]
    }

    fn clear(&mut self) {
        self.phase = 0;
        self.step = 0;
        self.f = self.frequency[0];
        self.q = self.resonance[0];
        for filter in &mut self.filters {
            filter.clear();
        }
        self.output.clear();
    }
}
/// ROM `Fc_tbl`: step length × pattern × step → SVF frequency word at 44.1 kHz.
const FREQUENCY: [u32; 448] = [
    0x3e9166f5, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf,
    0x3e87ddb1, 0x3cec25dd, 0x3cec25dd, 0x3cec25dd, 0x3cec25dd, 0x3cec25dd, 0x3cec25dd, 0x3cec25dd,
    0x3e7d8073, 0x3cf8d480, 0x3cf8d480, 0x3cf8d480, 0x3cf8d480, 0x3cf8d480, 0x3cf8d480, 0x3cf8d480,
    0x3e6c1ace, 0x3d09f037, 0x3d09f037, 0x3d09f037, 0x3d09f037, 0x3d09f037, 0x3d09f037, 0x3d09f037,
    0x3e5b87ef, 0x3d2048be, 0x3d2048be, 0x3d2048be, 0x3d2048be, 0x3d2048be, 0x3d2048be, 0x3d2048be,
    0x3e4bc521, 0x3d40d389, 0x3d40d389, 0x3d40d389, 0x3d40d389, 0x3d40d389, 0x3d40d389, 0x3d40d389,
    0x3e3ccf7f, 0x3d6cc540, 0x3d6cc540, 0x3d6cc540, 0x3d6cc540, 0x3d6cc540, 0x3d6cc540, 0x3d6cc540,
    0x3e2ea3f9, 0x3d9299dd, 0x3d9299dd, 0x3d9299dd, 0x3d9299dd, 0x3d9299dd, 0x3d9299dd, 0x3d9299dd,
    0x3e9166f5, 0x3e08bd06, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf,
    0x3e667d3a, 0x3ded21e2, 0x3d078041, 0x3d078041, 0x3d078041, 0x3d078041, 0x3d078041, 0x3d078041,
    0x3e3346e3, 0x3dccbbd5, 0x3d287c72, 0x3d287c72, 0x3d287c72, 0x3d287c72, 0x3d287c72, 0x3d287c72,
    0x3e08bd06, 0x3db021df, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81,
    0x3d5efc81, 0x3db021df, 0x3e08bd06, 0x3e08bd06, 0x3e08bd06, 0x3e08bd06, 0x3e08bd06, 0x3e08bd06,
    0x3d287c72, 0x3dccbbd5, 0x3e3346e3, 0x3e3346e3, 0x3e3346e3, 0x3e3346e3, 0x3e3346e3, 0x3e3346e3,
    0x3d078041, 0x3ded21e2, 0x3e667d3a, 0x3e667d3a, 0x3e667d3a, 0x3e667d3a, 0x3e667d3a, 0x3e667d3a,
    0x3ce96caf, 0x3e08bd06, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5,
    0x3e9166f5, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf,
    0x3e9166f5, 0x3e3346e3, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf,
    0x3ce96caf, 0x3d1588cb, 0x3d972bc4, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5,
    0x3ce96caf, 0x3d972bc4, 0x3e3346e3, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5,
    0x3e9166f5, 0x3e4bc521, 0x3e1cf411, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf,
    0x3e667d3a, 0x3d972bc4, 0x3d972bc4, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf,
    0x3e3346e3, 0x3dccbbd5, 0x3e1cf411, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81,
    0x3e9166f5, 0x3d972bc4, 0x3e1cf411, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81, 0x3d5efc81,
    0x3e9166f5, 0x3d40d389, 0x3e3346e3, 0x3ce96caf, 0x3d972bc4, 0x3d972bc4, 0x3d972bc4, 0x3d972bc4,
    0x3e9166f5, 0x3e667d3a, 0x3e08bd06, 0x3d972bc4, 0x3d078041, 0x3d078041, 0x3d078041, 0x3d078041,
    0x3ce96caf, 0x3dccbbd5, 0x3d972bc4, 0x3e3346e3, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5,
    0x3ce96caf, 0x3d972bc4, 0x3e08bd06, 0x3e667d3a, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5,
    0x3e9166f5, 0x3cf01145, 0x3d972bc4, 0x3e08bd06, 0x3d287c72, 0x3d287c72, 0x3d287c72, 0x3d287c72,
    0x3ce96caf, 0x3d5efc81, 0x3e9166f5, 0x3e08bd06, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5,
    0x3e9166f5, 0x3e08bd06, 0x3ce96caf, 0x3d5efc81, 0x3e667d3a, 0x3e667d3a, 0x3e667d3a, 0x3e667d3a,
    0x3e9166f5, 0x3d078041, 0x3e9166f5, 0x3e3346e3, 0x3d287c72, 0x3d287c72, 0x3d287c72, 0x3d287c72,
    0x3e9166f5, 0x3e667d3a, 0x3e08bd06, 0x3d972bc4, 0x3d5efc81, 0x3d078041, 0x3d078041, 0x3d078041,
    0x3e9166f5, 0x3e08bd06, 0x3d972bc4, 0x3d287c72, 0x3d287c72, 0x3d972bc4, 0x3d972bc4, 0x3d972bc4,
    0x3d078041, 0x3d5efc81, 0x3d972bc4, 0x3e08bd06, 0x3e667d3a, 0x3e9166f5, 0x3e9166f5, 0x3e9166f5,
    0x3e9166f5, 0x3d5efc81, 0x3e08bd06, 0x3e3346e3, 0x3dccbbd5, 0x3d078041, 0x3d078041, 0x3d078041,
    0x3e9166f5, 0x3d5efc81, 0x3e4bc521, 0x3d972bc4, 0x3e08bd06, 0x3ce96caf, 0x3ce96caf, 0x3ce96caf,
    0x3e9166f5, 0x3cf01145, 0x3ce96caf, 0x3e3346e3, 0x3dccbbd5, 0x3d078041, 0x3d078041, 0x3d078041,
    0x3e9166f5, 0x3ce96caf, 0x3dccbbd5, 0x3d40d389, 0x3e4bc521, 0x3cf01145, 0x3cf01145, 0x3cf01145,
    0x3e9166f5, 0x3ce96caf, 0x3ce96caf, 0x3e3346e3, 0x3ce96caf, 0x3d287c72, 0x3d287c72, 0x3d287c72,
    0x3e9166f5, 0x3e4bc521, 0x3e08bd06, 0x3dccbbd5, 0x3d9be13f, 0x3d287c72, 0x3ce96caf, 0x3ce96caf,
    0x3ce96caf, 0x3d287c72, 0x3d9be13f, 0x3dccbbd5, 0x3e08bd06, 0x3e4bc521, 0x3e9166f5, 0x3e9166f5,
    0x3e9166f5, 0x3ce96caf, 0x3e9166f5, 0x3e4bc521, 0x3d9be13f, 0x3d287c72, 0x3ce96caf, 0x3ce96caf,
    0x3e9166f5, 0x3e4bc521, 0x3ce96caf, 0x3e9166f5, 0x3e4bc521, 0x3d9be13f, 0x3ce96caf, 0x3ce96caf,
    0x3ce96caf, 0x3d287c72, 0x3d9be13f, 0x3e4bc521, 0x3e9166f5, 0x3cfbbaec, 0x3e9166f5, 0x3e9166f5,
    0x3ce96caf, 0x3d5efc81, 0x3e4bc521, 0x3e9166f5, 0x3d078041, 0x3dccbbd5, 0x3e667d3a, 0x3e667d3a,
    0x3e9166f5, 0x3cf01145, 0x3e08bd06, 0x3d972bc4, 0x3dccbbd5, 0x3d078041, 0x3e667d3a, 0x3e667d3a,
    0x3e9166f5, 0x3cf01145, 0x3e08bd06, 0x3e3346e3, 0x3dccbbd5, 0x3d078041, 0x3e667d3a, 0x3e667d3a,
    0x3e81be23, 0x3d972bc4, 0x3e667d3a, 0x3d287c72, 0x3e08bd06, 0x3d078041, 0x3dccbbd5, 0x3ce96caf,
    0x3d287c72, 0x3dccbbd5, 0x3e9166f5, 0x3d5efc81, 0x3e3346e3, 0x3d287c72, 0x3e08bd06, 0x3e4bc521,
    0x3e9166f5, 0x3cf01145, 0x3e08bd06, 0x3e3346e3, 0x3dccbbd5, 0x3d078041, 0x3e667d3a, 0x3d972bc4,
    0x3e9166f5, 0x3e3346e3, 0x3d078041, 0x3e667d3a, 0x3e3346e3, 0x3d078041, 0x3e667d3a, 0x3d972bc4,
    0x3e9166f5, 0x3e08bd06, 0x3dccbbd5, 0x3d287c72, 0x3cfbbaec, 0x3d9be13f, 0x3ce96caf, 0x3e81be23,
    0x3e9166f5, 0x3e08bd06, 0x3d287c72, 0x3cfbbaec, 0x3e667d3a, 0x3db021df, 0x3ce96caf, 0x3d366a4e,
    0x3dccbbd5, 0x3ce96caf, 0x3d5efc81, 0x3e81be23, 0x3db021df, 0x3d5efc81, 0x3e1cf411, 0x3d287c72,
    0x3ce96caf, 0x3d972bc4, 0x3d287c72, 0x3e667d3a, 0x3dccbbd5, 0x3e3346e3, 0x3d81aeee, 0x3e08bd06,
];
/// ROM `Q_tbl`: the matching resonance words (scaled by `c32`).
const RESONANCE: [u32; 448] = [
    0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x3f6d3ae7, 0x3aaa64c3, 0x3aaa64c3, 0x3aaa64c3, 0x3aaa64c3, 0x3aaa64c3, 0x3aaa64c3, 0x3aaa64c3,
    0x3f5b4f61, 0x3bf0307f, 0x3bf0307f, 0x3bf0307f, 0x3bf0307f, 0x3bf0307f, 0x3bf0307f, 0x3bf0307f,
    0x3f4a3ad2, 0x3ca58f71, 0x3ca58f71, 0x3ca58f71, 0x3ca58f71, 0x3ca58f71, 0x3ca58f71, 0x3ca58f71,
    0x3f39f8a1, 0x3d29f16b, 0x3d29f16b, 0x3d29f16b, 0x3d29f16b, 0x3d29f16b, 0x3d29f16b, 0x3d29f16b,
    0x3f2a862f, 0x3d946b27, 0x3d946b27, 0x3d946b27, 0x3d946b27, 0x3d946b27, 0x3d946b27, 0x3d946b27,
    0x3f1bdf8f, 0x3dea209b, 0x3dea209b, 0x3dea209b, 0x3dea209b, 0x3dea209b, 0x3dea209b, 0x3dea209b,
    0x3f0e0179, 0x3e2c1a8b, 0x3e2c1a8b, 0x3e2c1a8b, 0x3e2c1a8b, 0x3e2c1a8b, 0x3e2c1a8b, 0x3e2c1a8b,
    0x3f800000, 0x3ed1e647, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x3f44b7d4, 0x3eae670e, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d,
    0x3f128ac6, 0x3e8ec56d, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc,
    0x3ed1e647, 0x3e65b963, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4,
    0x3dcf3cb4, 0x3e65b963, 0x3ed1e647, 0x3ed1e647, 0x3ed1e647, 0x3ed1e647, 0x3ed1e647, 0x3ed1e647,
    0x3d49eecc, 0x3e8ec56d, 0x3f128ac6, 0x3f128ac6, 0x3f128ac6, 0x3f128ac6, 0x3f128ac6, 0x3f128ac6,
    0x3c928e0d, 0x3eae670e, 0x3f44b7d4, 0x3f44b7d4, 0x3f44b7d4, 0x3f44b7d4, 0x3f44b7d4, 0x3f44b7d4,
    0x00000000, 0x3ed1e647, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000,
    0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x3f800000, 0x3f128ac6, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x3d000000, 0x3e3505d1, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000,
    0x00000000, 0x3e3505d1, 0x3f128ac6, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000,
    0x3f800000, 0x3f2a862f, 0x3ef96a6a, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x3f44b7d4, 0x3e3505d1, 0x3e3505d1, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x3f128ac6, 0x3e8ec56d, 0x3ef96a6a, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4,
    0x3f800000, 0x3e3505d1, 0x3ef96a6a, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4, 0x3dcf3cb4,
    0x3f800000, 0x3d946b27, 0x3f128ac6, 0x00000000, 0x3e3505d1, 0x3e3505d1, 0x3e3505d1, 0x3e3505d1,
    0x3f800000, 0x3f44b7d4, 0x3ed1e647, 0x3e3505d1, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d,
    0x00000000, 0x3e8ec56d, 0x3e3505d1, 0x3f128ac6, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000,
    0x00000000, 0x3e3505d1, 0x3ed1e647, 0x3f44b7d4, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000,
    0x3f800000, 0x3b4f1801, 0x3e3505d1, 0x3ed1e647, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc,
    0x00000000, 0x3dcf3cb4, 0x3f800000, 0x3ed1e647, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000,
    0x3f800000, 0x3ed1e647, 0x00000000, 0x3dcf3cb4, 0x3f44b7d4, 0x3f44b7d4, 0x3f44b7d4, 0x3f44b7d4,
    0x3f800000, 0x3c928e0d, 0x3f800000, 0x3f128ac6, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc,
    0x3f800000, 0x3f44b7d4, 0x3ed1e647, 0x3e3505d1, 0x3dcf3cb4, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d,
    0x3f800000, 0x3ed1e647, 0x3e3505d1, 0x3d49eecc, 0x3d49eecc, 0x3e3505d1, 0x3e3505d1, 0x3e3505d1,
    0x3c928e0d, 0x3dcf3cb4, 0x3e3505d1, 0x3ed1e647, 0x3f44b7d4, 0x3f800000, 0x3f800000, 0x3f800000,
    0x3f800000, 0x3dcf3cb4, 0x3ed1e647, 0x3f128ac6, 0x3e8ec56d, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d,
    0x3f800000, 0x3dcf3cb4, 0x3f2a862f, 0x3e3505d1, 0x3ed1e647, 0x00000000, 0x00000000, 0x00000000,
    0x3f800000, 0x3b4f1801, 0x00000000, 0x3f128ac6, 0x3e8ec56d, 0x3c928e0d, 0x3c928e0d, 0x3c928e0d,
    0x3f800000, 0x00000000, 0x3e8ec56d, 0x3d946b27, 0x3f2a862f, 0x3b4f1801, 0x3b4f1801, 0x3b4f1801,
    0x3f800000, 0x00000000, 0x00000000, 0x3f128ac6, 0x00000000, 0x3d49eecc, 0x3d49eecc, 0x3d49eecc,
    0x3f800000, 0x3f2a862f, 0x3ed1e647, 0x3e8ec56d, 0x3e3e353f, 0x3d49eecc, 0x00000000, 0x00000000,
    0x00000000, 0x3d49eecc, 0x3e3e353f, 0x3e8ec56d, 0x3ed1e647, 0x3f2a862f, 0x3f800000, 0x3f800000,
    0x3f800000, 0x00000000, 0x3f800000, 0x3f2a862f, 0x3e3e353f, 0x3d49eecc, 0x00000000, 0x00000000,
    0x3f800000, 0x3f2a862f, 0x00000000, 0x3f800000, 0x3f2a862f, 0x3e3e353f, 0x00000000, 0x00000000,
    0x00000000, 0x3d49eecc, 0x3e3e353f, 0x3f2a862f, 0x3f800000, 0x3c0eb463, 0x3f800000, 0x3f800000,
    0x00000000, 0x3dcf3cb4, 0x3f2a862f, 0x3f800000, 0x3c928e0d, 0x3e8ec56d, 0x3f44b7d4, 0x3f44b7d4,
    0x3f800000, 0x3b4f1801, 0x3ed1e647, 0x3e3505d1, 0x3e8ec56d, 0x3c928e0d, 0x3f44b7d4, 0x3f44b7d4,
    0x3f800000, 0x3b4f1801, 0x3ed1e647, 0x3f128ac6, 0x3e8ec56d, 0x3c928e0d, 0x3f44b7d4, 0x3f44b7d4,
    0x3f6130be, 0x3e3505d1, 0x3f44b7d4, 0x3d49eecc, 0x3ed1e647, 0x3c928e0d, 0x3e8ec56d, 0x00000000,
    0x3d49eecc, 0x3e8ec56d, 0x3f800000, 0x3dcf3cb4, 0x3f128ac6, 0x3d49eecc, 0x3ed1e647, 0x3f2a862f,
    0x3f800000, 0x3b4f1801, 0x3ed1e647, 0x3f128ac6, 0x3e8ec56d, 0x3c928e0d, 0x3f44b7d4, 0x3e3505d1,
    0x3f800000, 0x3f128ac6, 0x3c928e0d, 0x3f44b7d4, 0x3f128ac6, 0x3c928e0d, 0x3f44b7d4, 0x3e3505d1,
    0x3f800000, 0x3ed1e647, 0x3e8ec56d, 0x3d49eecc, 0x3c0eb463, 0x3e3e353f, 0x00000000, 0x3f6130be,
    0x3f800000, 0x3ed1e647, 0x3d49eecc, 0x3c0eb463, 0x3f44b7d4, 0x3e65b963, 0x00000000, 0x3d801f75,
    0x3e8ec56d, 0x00000000, 0x3dcf3cb4, 0x3f6130be, 0x3e65b963, 0x3dcf3cb4, 0x3ef96a6a, 0x3d49eecc,
    0x00000000, 0x3e3505d1, 0x3d49eecc, 0x3f44b7d4, 0x3e8ec56d, 0x3f128ac6, 0x3e0b19a4, 0x3ed1e647,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn speed_follows_the_editor_curve() {
        assert!((normal_speed_hz(0.0) - 1.6667).abs() < 1e-3);
        assert!((normal_speed_hz(49.0) - 20.0).abs() < 1e-2);
        assert!((common::sync_hz(57.0, 49, 120.0, normal_speed_hz) - 2.0).abs() < 1e-4);
    }

    #[test]
    fn steps_advance_at_the_speed_and_wrap_at_the_length() {
        let mut effect = SeqFltr::new(48_000.0);
        let mut values = Model::from_key("mooz_seq_fltr").unwrap().defaults();
        values[0] = 2.0; // four steps
        values[2] = 57.0; // quarter notes
        effect.set(&values, 120.0);
        assert_eq!(effect.steps, 4);
        let mut changes = 0;
        let mut last = effect.step;
        let mut seen = [false; 8];
        for _ in 0..48_000 * 2 {
            effect.process([0.1, 0.1]);
            if effect.step != last {
                changes += 1;
                last = effect.step;
            }
            seen[effect.step] = true;
        }
        assert_eq!(changes, 4, "two steps per second for two seconds");
        assert!(seen[..4].iter().all(|&s| s) && !seen[4]);
    }

    #[test]
    fn frequency_words_keep_their_cutoff() {
        let mut effect = SeqFltr::new(96_000.0);
        effect.set(&Model::from_key("mooz_seq_fltr").unwrap().defaults(), 120.0);
        let hz = |f: f32, rate: f32| (f as f64 * 0.5).asin() * rate as f64 / std::f64::consts::PI;
        let start = 6 * 64 + 6 * 8;
        for i in 0..8 {
            let original = hz(f32::from_bits(FREQUENCY[start + i]), 44_100.0);
            assert!((hz(effect.frequency[i], 96_000.0) - original).abs() < 0.05);
        }
    }

    #[test]
    fn default_output_is_bounded_and_audible() {
        let mut effect = SeqFltr::new(44_100.0);
        effect.set(&Model::from_key("mooz_seq_fltr").unwrap().defaults(), 120.0);
        let mut peak = 0.0f32;
        for i in 0..44_100 {
            let x = 0.5 * ((i as f32) * 0.07).sin() + 0.3 * ((i as f32) * 0.011).sin();
            peak = peak.max(effect.process([x, x])[0].abs());
        }
        assert!(peak > 0.1 && peak < 2.0, "{peak}");
    }
}
