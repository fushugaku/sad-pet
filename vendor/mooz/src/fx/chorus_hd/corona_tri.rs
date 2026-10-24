//! CoronaTri (CRN_TRI.ZDL), the TC Corona tri-chorus: three modulated taps
//! per channel.
//!
//! Callback `Fx_MOD_CoronaTri`, per channel: ROM front biquads (also the dry,
//! see [`corona`](super::corona)) -> two ROM pre-write peaking filters
//! (+1 dB at 549 Hz, -0.5 dB at 201 Hz) -> ring. Three lanes read it at
//! `base[lane] + depth[tap] * wave[tap]` (bases 352/467/661 samples at
//! 44.1 kHz, depth a Q31 fraction of 2048 samples), each with its own phase
//! offset (c29..c34: L 0 / ¾ / 0 turn, R ½ / ¾ / ½ turn) and gain (c23..c28:
//! L -½, ½, ½; R ½, ½, -½). The waves are pure parabolic sines
//! `0.5 + 2 f (1 - |f|)` of the folded phase. The lane sum goes through the
//! tone shelf and `FxLVL * 1.15`.
//!
//! A new DEPTH only takes effect for a tap when its wave passes its minimum
//! (the editor writes pending words s34..s39 that the callback latches when
//! the tap's phase wraps), exactly as the original.
//!
//! Editors: `Speed_edit` increment table (0.06..10 Hz), `depth_edit` three
//! 11-point Q31 tables (lane 3 reaches 23 ms of sweep), `mix_edit`
//! `TBL * 1.15`, `tone_edit` shelf table, `OUTMODE_edit` fixes c46 = 1,
//! `outLv_edit` constant 1.0, DRY via `onf`/`KillDry_edit`.
use super::common::{Filter, fold, rescale_increment, rom, table};
use super::corona::{Front, Output, front_filters};
use crate::dsp::{Coefficients, DelayLine};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_corona_tri",
    name: "CoronaTri",
    category: Category::Modulation,
    zdl: "CRN_TRI.ZDL",
    description: "TC Corona style tri-chorus, three voices per side",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "SPEED",
            max: 100,
            default: 25,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "DEPTH",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "FxLVL",
            max: 100,
            default: 100,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "TONE",
            max: 100,
            default: 100,
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

/// CRN_TRI 0x80000928: Q32 phase increments at 44.1 kHz.
const SPEED_INCREMENT: [f32; 11] = [
    6130.0, 6688.0, 10584.0, 16168.0, 22294.0, 31204.0, 81940.0, 205_353.0, 453_597.0, 852_683.0,
    972_592.0,
];
/// CRN_TRI 0x80000898 / 0x800008c8 / 0x800008f8: Q31 depth per lane.
const DEPTH: [[f32; 11]; 3] = [
    [
        6_658_877.0,
        9_710_862.0,
        25_433_210.0,
        43_005_247.0,
        61_039_706.0,
        85_085_650.0,
        99_883_155.0,
        116_530_348.0,
        388_434_493.0,
        670_511_923.0,
        739_875_225.0,
    ],
    [
        5_549_064.0,
        6_843_845.0,
        13_502_722.0,
        21_271_412.0,
        27_190_414.0,
        35_144_073.0,
        59_190_018.0,
        84_160_806.0,
        295_950_090.0,
        527_161_098.0,
        573_403_299.0,
    ],
    [
        9_248_440.0,
        13_502_722.0,
        37_456_183.0,
        64_739_082.0,
        94_334_091.0,
        129_478_164.0,
        148_899_889.0,
        166_471_925.0,
        554_906_419.0,
        961_837_793.0,
        1_072_819_077.0,
    ],
];
/// CRN_TRI 0x80000868: FxLVL gains (scaled by 1.15 in `mix_edit`).
const FX_LEVEL: [f32; 11] = [
    0.0, 0.034_674, 0.058_884, 0.162_181, 0.281_838, 0.421_697, 0.512_861, 0.683_912, 0.933_254,
    0.988_553, 1.0,
];
/// CRN_TRI 0x80000958: TONE shelf gains in dB.
const TONE_DB: [f32; 11] = [
    -60.0, -55.0, -45.0, -30.0, -25.0, -22.0, -18.0, -13.0, -10.0, -5.0, 0.0,
];
/// c35..c37: lane base delays in 44.1 kHz samples.
const BASE: [f32; 3] = [352.0, 467.0, 661.0];
/// c23..c28 and c29..c34, indexed `lane * 2 + channel`.
const GAIN: [f32; 6] = [-0.5, 0.5, 0.5, 0.5, 0.5, -0.5];
const OFFSET: [u32; 6] = [0, 0x8000_0000, 0xc000_0000, 0xc000_0000, 0, 0x8000_0000];
/// The latch words of phase 0, so a fresh instance does not latch at once.
const PREVIOUS: [i32; 6] = {
    let mut words = [0; 6];
    let mut tap = 0;
    while tap < 6 {
        words[tap] = OFFSET[tap].wrapping_add(0x8000_0000) as i32;
        tap += 1;
    }
    words
};

/// CRN_TRI c6..c10 and c11..c15.
fn prewrite_filters(sample_rate: f32) -> [Coefficients; 2] {
    [
        rom(
            1.006_461_4,
            -1.888_278_7,
            0.887_629_75,
            1.888_278_7,
            -0.894_091_1,
            sample_rate,
        ),
        rom(
            0.999_168_46,
            -1.969_469_3,
            0.971_100_7,
            1.969_469_3,
            -0.970_269_1,
            sample_rate,
        ),
    ]
}

struct CoronaTri {
    sample_rate: f32,
    front: [Front; 2],
    prewrite: [[Filter; 2]; 2],
    lines: [DelayLine; 2],
    output: Output,
    phase: u32,
    increment: u32,
    base: [f32; 3],
    /// Active and pending sweep per tap (instance samples).
    depth: [f32; 6],
    pending: [f32; 6],
    has_pending: [bool; 6],
    previous: [i32; 6],
    last: Values,
    fresh: bool,
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let front = Front::new(front_filters(sample_rate));
    let [a, b] = prewrite_filters(sample_rate);
    let mut prewrite = [Filter::default(); 2];
    prewrite[0].set(a);
    prewrite[1].set(b);
    let scale = sample_rate / crate::zoom::ORIGINAL_RATE;
    let mut effect = CoronaTri {
        sample_rate,
        front: [front; 2],
        prewrite: [prewrite; 2],
        lines: [
            DelayLine::with_seconds(0.04, sample_rate),
            DelayLine::with_seconds(0.04, sample_rate),
        ],
        output: Output::new(sample_rate),
        phase: 0,
        increment: 0,
        base: BASE.map(|b| b * scale),
        depth: [0.0; 6],
        pending: [0.0; 6],
        has_pending: [false; 6],
        previous: PREVIOUS,
        last: [f32::NAN; crate::MAX_PARAMS],
        fresh: true,
    };
    effect.set(&crate::Model::CoronaTri.defaults(), 120.0);
    Box::new(effect)
}

/// The parabolic-sine wave of a phase word, 0 at the wrap, 1 half a turn later.
#[inline]
fn wave(phase: u32) -> f32 {
    let f = fold(phase) - 0.5;
    0.5 + 2.0 * f * (1.0 - f.abs())
}

impl Effect for CoronaTri {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let jump = self.fresh;
        self.fresh = false;
        if values[0] != self.last[0] {
            let increment = table(&SPEED_INCREMENT, values[0], 100.0);
            self.increment = rescale_increment(increment, self.sample_rate);
        }
        if values[1] != self.last[1] {
            let scale = 2048.0 / 2_147_483_648.0 * self.sample_rate / crate::zoom::ORIGINAL_RATE;
            for tap in 0..6 {
                let depth = table(&DEPTH[tap / 2], values[1], 100.0) * scale;
                if jump {
                    self.depth[tap] = depth;
                } else {
                    self.pending[tap] = depth;
                    self.has_pending[tap] = true;
                }
            }
        }
        if values[3] != self.last[3] {
            self.output.set_tone(&TONE_DB, values[3]);
        }
        if values[2] != self.last[2] || values[4] != self.last[4] {
            let fx = table(&FX_LEVEL, values[2], 100.0) * 1.15;
            self.output.set_levels(fx, values[4].round() >= 1.0, jump);
        }
        self.last = *values;
    }

    fn process(&mut self, [l, r]: [f32; 2]) -> [f32; 2] {
        let front = [self.front[0].process(l), self.front[1].process(r)];
        for (channel, &x) in front.iter().enumerate() {
            let [a, b] = &mut self.prewrite[channel];
            self.lines[channel].write(b.process(a.process(x)));
        }
        self.phase = self.phase.wrapping_add(self.increment);
        let mut wet = [0.0; 2];
        for tap in 0..6 {
            let phase = self.phase.wrapping_add(OFFSET[tap]);
            // Latch a pending depth where this tap's wave bottoms out.
            let raw = phase.wrapping_add(0x8000_0000) as i32;
            if self.has_pending[tap] && raw < self.previous[tap] {
                self.depth[tap] = self.pending[tap];
                self.has_pending[tap] = false;
            }
            self.previous[tap] = raw;
            let delay = self.base[tap / 2] + self.depth[tap] * wave(phase);
            wet[tap & 1] += GAIN[tap] * self.lines[tap & 1].read_linear(delay);
        }
        self.output.mix(front, wet)
    }

    fn clear(&mut self) {
        self.front.iter_mut().for_each(Front::clear);
        self.prewrite.iter_mut().flatten().for_each(Filter::clear);
        self.lines.iter_mut().for_each(DelayLine::clear);
        self.output.clear();
        self.phase = 0;
        self.previous = PREVIOUS;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;
    use crate::dsp::gain_to_db;

    #[test]
    fn prewrite_filters_are_the_decoded_peaks() {
        let [a, b] = prewrite_filters(44_100.0);
        assert!((gain_to_db(a.magnitude(44_100.0, 549.0)) - 1.0).abs() < 0.05);
        assert!((gain_to_db(b.magnitude(44_100.0, 201.0)) + 0.5).abs() < 0.05);
        assert!(gain_to_db(a.magnitude(44_100.0, 20.0)).abs() < 0.02);
    }

    #[test]
    fn wave_bottoms_out_at_the_wrap() {
        assert!(wave(0).abs() < 1e-6);
        assert!((wave(0x8000_0000) - 1.0).abs() < 1e-6);
        assert!((wave(0x4000_0000) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn depth_changes_wait_for_the_wave_minimum() {
        let mut effect = *create_raw(48_000.0);
        let mut values = Model::CoronaTri.defaults();
        values[0] = 100.0; // ~10 Hz: wraps every ~4800 samples
        effect.set(&values, 120.0);
        let before = effect.depth;
        values[1] = 0.0;
        effect.set(&values, 120.0);
        effect.process([0.0; 2]);
        assert_eq!(effect.depth, before);
        for _ in 0..5000 {
            effect.process([0.0; 2]);
        }
        assert!(
            effect.depth.iter().all(|d| *d < before[0] * 0.05),
            "{:?}",
            effect.depth
        );
    }

    fn create_raw(sample_rate: f32) -> Box<CoronaTri> {
        let front = Front::new(front_filters(sample_rate));
        let mut effect = CoronaTri {
            sample_rate,
            front: [front; 2],
            prewrite: [[Filter::default(); 2]; 2],
            lines: [DelayLine::new(4096), DelayLine::new(4096)],
            output: Output::new(sample_rate),
            phase: 0,
            increment: 0,
            base: BASE,
            depth: [0.0; 6],
            pending: [0.0; 6],
            has_pending: [false; 6],
            previous: PREVIOUS,
            last: [f32::NAN; crate::MAX_PARAMS],
            fresh: true,
        };
        effect.set(&Model::CoronaTri.defaults(), 120.0);
        Box::new(effect)
    }
}
