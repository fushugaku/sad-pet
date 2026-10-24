//! The HD Hall network shared by HDHALL.ZDL (`Fx_REV_HD_Hall`) and the hall
//! stage of SHIMMER.ZDL (`Fx_SUB_HDHALL`), following the recovered callback
//! (zoom-ms70-cdr `hd_hall/hall.rs`, `shimmer/hall.rs`):
//!
//! mono input → DC high-pass → 2 kHz low-pass → predelay tap → four series
//! Schroeder all-passes (gain from the Decay ROM table) → six parallel combs
//! (190.99 … 104.73 ms) whose taps are split by a ~2 kHz one-pole into a low
//! and a high band with separate feedback gains, then two first-order filters
//! (HiDMP / LoDMP) before re-entering the line. Six output taps per side feed
//! two all-passes per channel, a low shelf and a 5 kHz low-pass ("main").
//! Early reflections: a four-tap FIR line and an eight-tap line split into
//! left (even taps) and right (odd taps), with their own input and output
//! filters. Ring offsets are the ROM words of the coefficient block.
use super::common::{BLOCK, Design, FirstOrder, FirstOrderState, Ramps, Ring, design1, scale};
use crate::zoom::ORIGINAL_RATE;

/// ROM read offsets c[46..71] (Shimmer numbering; HD Hall c[7..32]) after the
/// predelay tap: all-pass reads, comb reads, output taps, stereo all-passes.
const READS: [u32; 25] = [
    14231, 15681, 17149, 18634, // all-pass 1..4
    27422, 35283, 42370, 48672, 54178, 58877, // combs 1..6
    19404, 19414, 28173, 28394, 36067, 35787, 43084, 43245, 49223, 49163,
    54484, // output taps
    59881, 60863, 61997, 63024, // stereo all-passes
];
/// ROM write offsets c[114..129]: input, four all-passes, six combs, four
/// stereo all-passes.
const WRITES: [u32; 15] = [
    0, 13230, 14553, 15876, 17199, 18963, 27783, 35721, 42777, 48951, 54243, 59094, 59976, 61299,
    62181,
];
/// Early reflections: the region starts after the network (the original
/// places it at +88200; only differences inside it matter).
const EARLY_BASE: u32 = 64000;
const EARLY_SECOND: u32 = 19000;
const EARLY_FIR: [u32; 4] = [2645, 2760, 2925, 3162];
const EARLY_TAPS: [u32; 8] = [3, 16, 623, 653, 1116, 1189, 1508, 1681];
const FIR_GAIN: f32 = 0.749_99;
const EARLY_LEFT_GAIN: f32 = 0.509_994_8;
const EARLY_RIGHT_GAIN: f32 = 0.749_99;
/// Output tap gains c[99], c[100] and the stereo all-pass output gain c[101].
const TAP_GAIN: f32 = 0.6;
const STEREO_GAIN: f32 = 0.61;
/// Comb split one-pole c[86].
const SPLIT: f32 = 0.247_016;
/// Step of the decay and mix setters (0x3af7c5ec per 44.1 kHz sample).
const STEP: f32 = 0.001_889;

/// HD_Hall_CombFB_Delay_tbl: the comb lengths the decay editor assumes (ms).
pub const COMB_MS: [f32; 6] = [
    190.9942, 169.36615, 150.18724, 133.18013, 118.0989, 104.72546,
];

/// HD_Hall_APF_FB_tbl (101 entries, constant 0.8 from 48 on).
const APF_FB: [f32; 49] = [
    0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.53, 0.53, 0.53, 0.53, 0.56, 0.56, 0.56,
    0.6, 0.6, 0.6, 0.61, 0.61, 0.61, 0.61, 0.62, 0.63, 0.633, 0.64, 0.645, 0.65, 0.66, 0.67, 0.675,
    0.685, 0.695, 0.7, 0.7, 0.7, 0.7, 0.71, 0.73, 0.74, 0.7525, 0.76, 0.77, 0.7775, 0.7875, 0.7975,
    0.8,
];

/// The all-pass gain of the Decay knob (ROM table lookup).
pub fn apf_feedback(decay: f32) -> f32 {
    APF_FB[(decay.round().max(0.0) as usize).min(APF_FB.len() - 1)]
}

// Fixed ROM sections (44.1 kHz), first-order in the `[b0, b1, f1]` layout.
const INPUT_HP: FirstOrder = FirstOrder::new(0.990_124_8, -0.990_124_8, 0.980_249_7);
const INPUT_LP: FirstOrder = FirstOrder::new(0.123_781_1, 0.123_781_1, 0.752_437_8);
/// The second comb section as initialised from ROM (HD Hall never edits it).
pub const LANE_LOW_SHELF: FirstOrder = FirstOrder::new(0.998_321_4, -0.971_378_3, 0.969_699_7);
const WET_SHELF: FirstOrder = FirstOrder::new(0.978_284_6, -0.934_647, 0.912_931_6);
const EARLY_IN_HP: FirstOrder = FirstOrder::new(0.491_315_8, -0.491_315_8, 0.960_607_9);
const EARLY_IN_SHELF: FirstOrder = FirstOrder::new(0.595_685, -0.443_312_1, 0.847_627_1);
const EARLY_OUT: FirstOrder = FirstOrder::new(1.114_882, -0.333_775_7, 0.218_893_6);

/// What feeds the early-reflection filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarlyInput {
    /// HDHALL: predelayed input plus half the (wet-scaled) main output.
    HdHall,
    /// SHIMMER: diffused input plus 0.9 × predelayed input. The callback
    /// also adds the constant c[36] = 0.51, which the following high-pass
    /// removes within milliseconds; it is left out here.
    Shimmer,
}

// Ramp slots.
const R_APF: usize = 0;
const R_LOW: usize = 1;
const R_HIGH: usize = 7;
const R_WET: usize = 13;
const R_EARLY: usize = 14;
const R_DRY: usize = 15;
const R_DAMP: usize = 16; // 3 words of the first comb section
const R_LOWCUT: usize = 19; // 3 words of the second comb section
const RAMPS: usize = 22;

/// Coefficient targets the model editors compute.
#[derive(Debug, Clone, Copy)]
pub struct HallSettings {
    /// Predelay in 44.1 kHz samples.
    pub predelay: f32,
    pub allpass: f32,
    pub low: [f32; 6],
    pub high: [f32; 6],
    pub wet: f32,
    pub early: f32,
    pub dry: f32,
    /// First comb section (HiDMP), designed at the instance rate.
    pub damp: FirstOrder,
    /// Second comb section (LoDMP), designed at the instance rate.
    pub lowcut: FirstOrder,
}

pub struct Hall {
    ring: Ring,
    reads: [usize; 25],
    writes: [usize; 15],
    early_a: usize,
    early_b: usize,
    early_fir: [usize; 4],
    early_taps: [usize; 8],
    early_input: EarlyInput,
    rate_ratio: f32,
    predelay: f32,
    predelay_target: f32,
    predelay_max: f32,
    predelay_coefficient: f32,
    split: f32,
    input_hp: FirstOrder,
    input_lp: FirstOrder,
    input_state: [FirstOrderState; 2],
    low: [f32; 6],
    damp_state: [FirstOrderState; 6],
    lowcut_state: [FirstOrderState; 6],
    wet_shelf: FirstOrder,
    wet_lp: FirstOrder,
    wet_state: [[FirstOrderState; 2]; 2],
    early_in: [FirstOrder; 2],
    early_in_state: [FirstOrderState; 2],
    early_out: FirstOrder,
    early_out_state: [FirstOrderState; 2],
    ramps: Ramps<RAMPS>,
    moving: bool,
    counter: usize,
    primed: bool,
}

impl Hall {
    pub fn new(sample_rate: f32, early_input: EarlyInput) -> Self {
        let early_a = scale(EARLY_BASE, sample_rate);
        let early_b = scale(EARLY_BASE + EARLY_SECOND, sample_rate);
        let span = scale(EARLY_BASE + EARLY_SECOND + EARLY_TAPS[7], sample_rate) + 2;
        let rate_ratio = sample_rate / ORIGINAL_RATE;
        Self {
            ring: Ring::new(span),
            reads: READS.map(|o| scale(o, sample_rate)),
            writes: WRITES.map(|o| scale(o, sample_rate)),
            early_a,
            early_b,
            early_fir: EARLY_FIR.map(|o| early_a + scale(o, sample_rate)),
            early_taps: EARLY_TAPS.map(|o| early_b + scale(o, sample_rate)),
            early_input,
            rate_ratio,
            predelay: 0.0,
            predelay_target: 0.0,
            // The predelay tap must stay inside the input line (< 13230).
            predelay_max: (WRITES[1] as f32 - 2.0) * rate_ratio,
            predelay_coefficient: crate::dsp::one_pole_coefficient(0.03, sample_rate),
            split: 1.0 - (1.0 - SPLIT).powf(1.0 / rate_ratio),
            input_hp: INPUT_HP.resample(sample_rate),
            input_lp: INPUT_LP.resample(sample_rate),
            input_state: [FirstOrderState::default(); 2],
            low: [0.0; 6],
            damp_state: [FirstOrderState::default(); 6],
            lowcut_state: [FirstOrderState::default(); 6],
            wet_shelf: WET_SHELF.resample(sample_rate),
            // Decay_PostEQ2_set: Main design 0 at a constant 5000 Hz.
            wet_lp: design1(Design::LowPass1, sample_rate, 5000.0, 0.0),
            wet_state: [[FirstOrderState::default(); 2]; 2],
            early_in: [
                EARLY_IN_HP.resample(sample_rate),
                EARLY_IN_SHELF.resample(sample_rate),
            ],
            early_in_state: [FirstOrderState::default(); 2],
            early_out: EARLY_OUT.resample(sample_rate),
            early_out_state: [FirstOrderState::default(); 2],
            ramps: Ramps::new(STEP, BLOCK, sample_rate),
            moving: false,
            counter: 0,
            primed: false,
        }
    }

    /// Apply new editor results; the first call jumps, later ones ramp.
    pub fn apply(&mut self, s: &HallSettings) {
        self.predelay_target = (s.predelay * self.rate_ratio).clamp(1.0, self.predelay_max);
        let t = &mut self.ramps.target;
        t[R_APF] = s.allpass;
        t[R_LOW..R_LOW + 6].copy_from_slice(&s.low);
        t[R_HIGH..R_HIGH + 6].copy_from_slice(&s.high);
        t[R_WET] = s.wet;
        t[R_EARLY] = s.early;
        t[R_DRY] = s.dry;
        t[R_DAMP..R_DAMP + 3].copy_from_slice(&[s.damp.b0, s.damp.b1, s.damp.f1]);
        t[R_LOWCUT..R_LOWCUT + 3].copy_from_slice(&[s.lowcut.b0, s.lowcut.b1, s.lowcut.f1]);
        if self.primed {
            self.moving = true;
        } else {
            self.ramps.jump();
            self.predelay = self.predelay_target;
            self.primed = true;
        }
    }

    pub fn clear(&mut self) {
        self.ring.clear();
        self.input_state = Default::default();
        self.low = [0.0; 6];
        self.damp_state = Default::default();
        self.lowcut_state = Default::default();
        self.wet_state = Default::default();
        self.early_in_state = Default::default();
        self.early_out_state = Default::default();
    }

    /// One frame. `left`/`right` feed the network (it sums them to mono);
    /// `dry` is what the internal dry gain mixes in. Returns the HD Hall
    /// output `dry * c[113] + early + main`.
    #[inline]
    pub fn process(&mut self, left: f32, right: f32, dry: [f32; 2]) -> [f32; 2] {
        if self.moving {
            self.counter += 1;
            if self.counter >= BLOCK {
                self.counter = 0;
                self.moving = self.ramps.tick();
            }
        }
        let g = &self.ramps.current;
        let allpass = g[R_APF];
        let damp = FirstOrder::new(g[R_DAMP], g[R_DAMP + 1], g[R_DAMP + 2]);
        let lowcut = FirstOrder::new(g[R_LOWCUT], g[R_LOWCUT + 1], g[R_LOWCUT + 2]);
        self.predelay += (self.predelay_target - self.predelay) * self.predelay_coefficient;

        self.ring.advance();
        let r = &self.reads;
        let ring = &self.ring;
        let predelayed = ring.read_frac(self.predelay);
        let tap = |i: usize| ring.read(r[i]);

        // Input filters into the predelay line.
        let mono = 0.5 * (left + right);
        let first = self.input_state[0].process(&self.input_hp, mono);
        let input = self.input_state[1].process(&self.input_lp, first);

        // Four series all-passes: v = x + g d, y = d - g v.
        let mut node = predelayed;
        let mut diffuser_writes = [0.0; 4];
        for (section, write) in diffuser_writes.iter_mut().enumerate() {
            let delayed = tap(section);
            let v = node + allpass * delayed;
            *write = v;
            node = delayed - allpass * v;
        }
        let diffused = node;

        // Six combs with a two-band damping split and two sections.
        let complement = 1.0 - self.split;
        let mut comb_writes = [0.0; 6];
        for lane in 0..6 {
            let x = tap(4 + lane);
            let previous = self.low[lane];
            let low = self.split * x + complement * previous;
            let high = complement * (x - previous);
            self.low[lane] = low;
            let incoming = diffused + g[R_HIGH + lane] * high + g[R_LOW + lane] * low;
            let filtered = self.damp_state[lane].process(&damp, incoming);
            comb_writes[lane] = self.lowcut_state[lane].process(&lowcut, filtered);
        }

        // Output taps and two all-passes per side.
        let left_sum = TAP_GAIN * (tap(10) + tap(12) + tap(14) + tap(16) + tap(18) + tap(20));
        let right_sum = TAP_GAIN * (tap(11) + tap(13) + tap(15) + tap(17) + tap(19) + tap(20));
        let (d22, d23, d24, d25) = (tap(21), tap(22), tap(23), tap(24));
        let s74 = left_sum + allpass * d22;
        let y74 = d22 - allpass * s74;
        let s75 = y74 + allpass * d23;
        let wet_left = STEREO_GAIN * (d23 - allpass * s75);
        let s76 = right_sum + allpass * d24;
        let y76 = d24 - allpass * s76;
        let s77 = y76 + allpass * d25;
        let wet_right = STEREO_GAIN * (d25 - allpass * s77);

        // Early reflection lines (read before this frame's writes).
        let mut convolved = 0.0;
        for &offset in &self.early_fir {
            convolved += ring.read(offset);
        }
        convolved *= FIR_GAIN;
        let t = &self.early_taps;
        let early_left = EARLY_LEFT_GAIN
            * (ring.read(t[0]) + ring.read(t[2]) + ring.read(t[4]) + ring.read(t[6]));
        let early_right = EARLY_RIGHT_GAIN
            * (ring.read(t[1]) + ring.read(t[3]) + ring.read(t[5]) + ring.read(t[7]));

        // Network writes.
        let w = self.writes;
        let ring = &mut self.ring;
        ring.write(w[0], input);
        for (i, &value) in diffuser_writes.iter().enumerate() {
            ring.write(w[1 + i], value);
        }
        for (i, &value) in comb_writes.iter().enumerate() {
            ring.write(w[5 + i], value);
        }
        ring.write(w[11], s74);
        ring.write(w[12], s75);
        ring.write(w[13], s76);
        ring.write(w[14], s77);

        // Main output filters, scaled by the wet gain.
        let wet = g[R_WET];
        let [ls, rs] = &mut self.wet_state;
        let shelved = ls[0].process(&self.wet_shelf, wet_left);
        let main_left = wet * ls[1].process(&self.wet_lp, shelved);
        let shelved = rs[0].process(&self.wet_shelf, wet_right);
        let main_right = wet * rs[1].process(&self.wet_lp, shelved);

        let incoming = match self.early_input {
            EarlyInput::HdHall => predelayed + 0.5 * (main_left + main_right),
            EarlyInput::Shimmer => diffused + 0.9 * predelayed,
        };
        let first = self.early_in_state[0].process(&self.early_in[0], incoming);
        let second = self.early_in_state[1].process(&self.early_in[1], first);
        ring.write(self.early_a, second);
        ring.write(self.early_b, convolved);
        let early = g[R_EARLY];
        let early_left = early * self.early_out_state[0].process(&self.early_out, early_left);
        let early_right = early * self.early_out_state[1].process(&self.early_out, early_right);

        let dry_gain = g[R_DRY];
        [
            dry_gain * dry[0] + early_left + main_left,
            dry_gain * dry[1] + early_right + main_right,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comb_lengths_match_the_rom_offsets() {
        // Comb n is written at WRITES[5 + n] and read at READS[4 + n].
        let lengths: Vec<u32> = (0..6).map(|n| READS[4 + n] - WRITES[5 + n]).collect();
        assert_eq!(lengths, [8459, 7500, 6649, 5895, 5227, 4634]);
        // All-pass reads sit inside the line written just before them.
        for n in 0..4 {
            assert!(READS[n] > WRITES[1 + n] && READS[n] < WRITES[2 + n]);
        }
    }
}
