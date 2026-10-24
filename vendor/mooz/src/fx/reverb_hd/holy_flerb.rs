//! HolyFLERB (HolyFLRB.ZDL): a small mono reverb feeding a flanger.
//!
//! Follows the recovered callback (zoom-ms70-cdr `holy_flerb/`):
//! - front: per channel a gain/first-order shelf (`_REV_Filter_Coe`), summed
//!   to mono, a 120 Hz high-pass and a shelf into the reverb line;
//! - reverb (`reverb.rs`): 22 ms predelay, two diffusers
//!   (`w = 0.875 x - 0.708 d`, `y = d + 0.708 x`, the second scaled by 0.375),
//!   four negative-gain combs (123 … 80 ms) with a 6.5 kHz low-pass, eight
//!   output taps averaged; `0.2 · input + reverb` goes on;
//! - a 170-sample short delay (`short_delays`);
//! - flanger (`flanger.rs`): input shelf, 0.65 feedback, delay
//!   `180 + 141.7 · lfo` samples with a 0.70 Hz parabolic LFO, output gain
//!   `1 - 0.15 · lfo`; out `0.75 · dry + 1.2 · flanged`;
//! - post (`outer.rs`): eight first-order 8 kHz low-passes and two peaking
//!   sections (`_FLEAB_Filter_Coe`), then `1.9 · (wet · WET + dry · DRY)`.
//!
//! The only knob, RVRB, reads Fx_REV_HolyFLERB_WET_MIX_tbl and
//! Fx_REV_HolyFLERB_DRY_MIX_tbl (11 points); every other word is the
//! initialised coefficient block captured from the firmware (the 1.9 output
//! gain included).
use super::common::{
    BLOCK, Biquad, Design, FirstOrder, FirstOrderState, Ramps, Ring, design1, rom_biquad, scale,
    tbl,
};

use crate::zoom::ORIGINAL_RATE;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_holy_flerb",
    name: "HolyFLERB",
    category: Category::Reverb,
    zdl: "HolyFLRB.ZDL",
    description: "Reverb into a feedback flanger",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "RVRB",
            max: 100,
            default: 50,
            display: Display::INT,
        },
    ],
};

const WET_TBL: [f32; 11] = [
    0.0, 0.015_945, 0.095_668, 0.111_613, 0.159_447, 0.207_281, 0.318_894, 0.433_697, 0.51,
    0.688_812, 0.707_946,
];
const DRY_TBL: [f32; 11] = [
    0.562_341, 0.562_341, 0.562_341, 0.541_514, 0.510_273, 0.479_031, 0.364_48, 0.260_343,
    0.130_172, 0.042_696, 0.0,
];
const OUTPUT_GAIN: f32 = 1.9;

const FRONT: FirstOrder = FirstOrder::new(1.360_784, -1.510_313, 0.749_096);
const INPUT_HP: [f32; 5] = [0.976_499_8, -1.953, 0.976_499_8, 1.952_38, -0.953_619_2];
const INPUT_SHELF: [f32; 5] = [1.066_945, -1.389_825, 0.415_958_6, 1.389_825, -0.482_903_3];
const PEAK_1: [f32; 5] = [1.002_918, -1.976_949, 0.974_544_9, 1.976_949, -0.977_462_7];
const PEAK_2: [f32; 5] = [
    0.996_632_6,
    -1.934_923,
    0.941_436_8,
    1.934_923,
    -0.938_069_4,
];
const COMB_LP: FirstOrder = FirstOrder::new(0.249_262_1, 0.249_262_1, 0.501_475_7);
const FLANGER_IN: FirstOrder = FirstOrder::new(0.998_247_3, -0.989_749_9, 0.987_997_2);

// Reverb ring offsets (c[42..57] reads, c[73..79] writes).
const PREDELAY: u32 = 970;
const DIFFUSER_READS: [u32; 2] = [5836, 6943];
const COMB_READS: [u32; 4] = [12368, 17315, 21919, 25443];
const WRITES: [u32; 7] = [0, 4412, 5837, 6944, 12369, 17316, 21920];
const TAPS: [u32; 8] = [22098, 18081, 13606, 8234, 21965, 17803, 13385, 8224];
const COMB_GAINS: [f32; 4] = [-0.769_341_1, -0.787_347, -0.800_500_5, -0.843_470_6];
const TAP_GAIN: f32 = 0.250_007_6;
const DIFF_IN: f32 = 0.875_026_7;
const DIFF_G: f32 = 0.707_998_9;
const DIFF_OUT: f32 = 0.375_011_4;
const DIRECT: f32 = 0.2;

const SHORT_DELAY: u32 = 170;
const FLANGE_BASE: f32 = 180.0;
/// c[141] · 2^31 >> 23: the LFO's delay span in samples.
const FLANGE_SPAN: f32 = 0.553_411_8 * 256.0;
const FLANGE_INCREMENT: u32 = 68_174;
const FLANGE_FEEDBACK: f32 = 0.65;
const FLANGE_GAIN_DEPTH: f32 = -0.15;
const FLANGE_DRY: f32 = 0.75;
const FLANGE_WET: f32 = 1.2;

struct HolyFlerb {
    ring: Ring,
    predelay: usize,
    diffuser_reads: [usize; 2],
    comb_reads: [usize; 4],
    writes: [usize; 7],
    taps: [usize; 8],
    front: FirstOrder,
    front_state: [FirstOrderState; 2],
    input: [Biquad; 2],
    comb_lp: FirstOrder,
    comb_state: [FirstOrderState; 4],
    short: Ring,
    short_delay: usize,
    flange: Ring,
    flange_base: f32,
    flange_span: f32,
    phase: u32,
    increment: u32,
    flange_in: FirstOrder,
    flange_in_state: FirstOrderState,
    feedback: f32,
    lowpass: FirstOrder,
    lowpass_state: [FirstOrderState; 8],
    peaks: [Biquad; 2],
    ramps: Ramps<2>,
    moving: bool,
    counter: usize,
    values: Option<Values>,
}

impl Effect for HolyFlerb {
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.values.as_ref() == Some(values) {
            return;
        }
        let first = self.values.is_none();
        self.values = Some(*values);
        self.ramps.target = [
            OUTPUT_GAIN * tbl(&WET_TBL, 101, values[0], 0.0),
            OUTPUT_GAIN * tbl(&DRY_TBL, 101, values[0], 0.0),
        ];
        if first {
            self.ramps.jump();
        } else {
            self.moving = true;
        }
    }

    #[inline]
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.moving {
            self.counter += 1;
            if self.counter >= BLOCK {
                self.counter = 0;
                self.moving = self.ramps.tick();
            }
        }
        // Front filters and the reverb input.
        let left = self.front_state[0].process(&self.front, frame[0]);
        let right = self.front_state[1].process(&self.front, frame[1]);
        let mono = 0.5 * (left + right);
        let [hp, shelf] = &mut self.input;
        let input = shelf.process(hp.process(mono));

        // Reverb.
        self.ring.advance();
        let ring = &self.ring;
        let predelayed = ring.read(self.predelay);
        let d1 = ring.read(self.diffuser_reads[0]);
        let w1 = DIFF_IN * predelayed - DIFF_G * d1;
        let y1 = d1 + DIFF_G * predelayed;
        let d2 = ring.read(self.diffuser_reads[1]);
        let w2 = DIFF_IN * y1 - DIFF_G * d2;
        let y2 = DIFF_OUT * (DIFF_G * y1 + d2);
        let mut combs = [0.0; 4];
        for (n, comb) in combs.iter_mut().enumerate() {
            let x = COMB_GAINS[n] * ring.read(self.comb_reads[n]);
            *comb = y2 + self.comb_state[n].process(&self.comb_lp, x);
        }
        let t = &self.taps;
        let mut sum = 0.0;
        for &offset in t {
            sum += ring.read(offset);
        }
        let reverb = 0.5 * TAP_GAIN * sum;
        let ring = &mut self.ring;
        let w = self.writes;
        ring.write(w[0], input);
        ring.write(w[1], w1);
        ring.write(w[2], w2);
        for (n, &value) in combs.iter().enumerate() {
            ring.write(w[3 + n], value);
        }
        let stage = DIRECT * mono + reverb;

        // Short delay.
        self.short.advance();
        self.short.write(0, stage);
        let delayed = self.short.read(self.short_delay);

        // Flanger.
        self.phase = self.phase.wrapping_add(self.increment);
        let v = (self.phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0) - 0.5;
        let wave = (4.0 * v * (1.0 - v.abs())).clamp(-1.0, 1.0);
        let offset = 0.5 * wave + 0.5;
        let filtered = self.flange_in_state.process(&self.flange_in, delayed);
        self.flange.advance();
        self.flange
            .write(0, filtered + FLANGE_FEEDBACK * self.feedback);
        let tapped = self
            .flange
            .read_frac(self.flange_base + self.flange_span * offset);
        self.feedback = super::common::flush(tapped * (1.0 + FLANGE_GAIN_DEPTH * offset));
        let mut wet = FLANGE_DRY * delayed + FLANGE_WET * self.feedback;

        // Post filters.
        for state in &mut self.lowpass_state {
            wet = state.process(&self.lowpass, wet);
        }
        let [a, b] = &mut self.peaks;
        let wet = b.process(a.process(wet));
        let [wet_gain, dry_gain] = self.ramps.current;
        [
            wet_gain * wet + dry_gain * frame[0],
            wet_gain * wet + dry_gain * frame[1],
        ]
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.short.clear();
        self.flange.clear();
        self.front_state = Default::default();
        self.input.iter_mut().for_each(Biquad::clear);
        self.comb_state = Default::default();
        self.flange_in_state.clear();
        self.feedback = 0.0;
        self.lowpass_state = Default::default();
        self.peaks.iter_mut().for_each(Biquad::clear);
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    let ratio = sample_rate / ORIGINAL_RATE;
    let biquad = |row: [f32; 5]| Biquad::new(rom_biquad(row).resample(ORIGINAL_RATE, sample_rate));
    let flange_base = FLANGE_BASE * ratio;
    let flange_span = FLANGE_SPAN * ratio;
    let mut effect = HolyFlerb {
        ring: Ring::new(scale(COMB_READS[3], sample_rate) + 2),
        predelay: scale(PREDELAY, sample_rate),
        diffuser_reads: DIFFUSER_READS.map(|o| scale(o, sample_rate)),
        comb_reads: COMB_READS.map(|o| scale(o, sample_rate)),
        writes: WRITES.map(|o| scale(o, sample_rate)),
        taps: TAPS.map(|o| scale(o, sample_rate)),
        front: FRONT.resample(sample_rate),
        front_state: Default::default(),
        input: [biquad(INPUT_HP), biquad(INPUT_SHELF)],
        comb_lp: COMB_LP.resample(sample_rate),
        comb_state: Default::default(),
        short: Ring::new(scale(SHORT_DELAY, sample_rate) + 2),
        short_delay: scale(SHORT_DELAY, sample_rate),
        flange: Ring::new((flange_base + flange_span) as usize + 4),
        flange_base,
        flange_span,
        phase: 0,
        increment: (FLANGE_INCREMENT as f32 / ratio).round() as u32,
        flange_in: FLANGER_IN.resample(sample_rate),
        flange_in_state: FirstOrderState::default(),
        feedback: 0.0,
        lowpass: design1(Design::LowPass1, sample_rate, 8000.0, 0.0),
        lowpass_state: Default::default(),
        peaks: [biquad(PEAK_1), biquad(PEAK_2)],
        ramps: Ramps::new(0.001_889, BLOCK, sample_rate),
        moving: false,
        counter: 0,
        values: None,
    };
    effect.set(&crate::Model::HolyFlerb.defaults(), 120.0);
    Box::new(effect)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_tables_match_the_firmware_sweep() {
        // RVRB 50 -> c[5] 0.207281, c[6] 0.479031; RVRB 100 -> 0.707946 / 0.
        assert!((tbl(&WET_TBL, 101, 50.0, 0.0) - 0.207_281).abs() < 1e-6);
        assert!((tbl(&DRY_TBL, 101, 50.0, 0.0) - 0.479_031).abs() < 1e-6);
        assert_eq!(tbl(&DRY_TBL, 101, 100.0, 0.0), 0.0);
    }

    #[test]
    fn flanger_delay_spans_the_rom_range() {
        // 180 + (0.5534 · 2^31 >> 23) samples at the LFO peak.
        assert!((FLANGE_BASE + FLANGE_SPAN - 321.67).abs() < 0.1);
    }

    #[test]
    fn full_wet_is_bounded_and_decays() {
        let mut effect = create(48_000.0);
        let mut v = crate::Model::HolyFlerb.defaults();
        v[0] = 100.0;
        effect.set(&v, 120.0);
        let mut energy = [0.0f32; 2];
        for n in 0..48_000 * 3 {
            let x = if n < 4800 {
                0.5 * ((n as f32) * 0.07).sin()
            } else {
                0.0
            };
            let [l, r] = effect.process([x, x]);
            assert!(l.abs() < 4.0 && r.abs() < 4.0);
            energy[usize::from(n > 48_000 * 2)] += l * l;
        }
        assert!(
            energy[0] > 0.0 && energy[1] < energy[0] * 0.01,
            "{energy:?}"
        );
    }
}
