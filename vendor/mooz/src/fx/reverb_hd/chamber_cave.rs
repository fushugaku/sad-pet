//! The kernel shared by CHAMBER.ZDL and CAVE.ZDL (`Fx_REV_Chamber`,
//! `Fx_REV_Cave`), following the recovered callback (zoom-ms70-cdr
//! `chamber_cave/reverb.rs`):
//!
//! mono input → DC high-pass → Tone low-pass (Main design 4, Q 0.9) → predelay
//! tap → four series diffusers (`v = 0.75 x + g d`, `y = 0.625 x - 0.5 d`,
//! the written value low-passed) → five parallel combs whose taps are split by
//! a ~200 Hz one-pole into low/high bands with their own feedback gains, then
//! the diffusers' low-pass and a DC blocker → five output taps per side, two
//! more diffusers per side, a post tone section → the early reflections
//! (four-tap FIR line, eight taps per side) are added at 0.21 → two output
//! biquads → `wet · out + dry · input`.
//!
//! Editors (disassembly + firmware sweep `chamber-cave-parameters-v1`):
//! - `decay_Calc_set`: `T = TBL_TO_VAL(101, Decay, *_Decay_ms_tbl)`; comb `n`
//!   gains `base_n^(D_n/T)` (high band) and `base_n^(D_n/(k·T))` (low band,
//!   k = 1.06 Chamber, 0.72 Cave) from *_CombFB_Delay_tbl; the diffuser gain
//!   from *_Decay_APF_FB_tbl.
//! - `tone_edit`: low-pass at `round(1000 + 4000·(Tone/100)^0.6)` Hz.
//! - `mix_edit`: wet and dry from *_WET_MIX_tbl / *_DRY_MIX_tbl.
//! - `predly_edit`: `floor(PreD · 44.1) + 1` samples.
use super::common::{BLOCK, Design, FirstOrder, FirstOrderState, Ramps, Ring, design, scale, tbl};
use crate::dsp::{Biquad, Coefficients};
use crate::zoom::ORIGINAL_RATE;
use crate::{Effect, Values};

/// Per-model ROM constants.
pub struct Config {
    pub output_taps: [u32; 10],
    pub input_hp: FirstOrder,
    pub tap_gains: [f32; 2],
    pub output_filters: [[f32; 5]; 2],
    pub damping: FirstOrder,
    pub tone: Option<FirstOrder>,
    pub early_taps: [u32; 8],
    pub early_left: [f32; 8],
    pub early_right: [f32; 8],
    pub apf_tbl: [f32; 11],
    pub decay_ms_tbl: [f32; 11],
    pub wet_tbl: [f32; 11],
    pub low_factor: f32,
}

const DRY_TBL: [f32; 11] = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0];
const COMB_MS: [f32; 5] = [96.4, 86.56, 82.64, 69.52, 61.04];
const COMB_BASE: [f32; 5] = [0.01238, 0.013032, 0.013718, 0.01444, 0.0152];

/// c[7..16]: diffuser and comb reads; c[26..30]: output diffuser reads.
const READS: [u32; 9] = [
    10145, 11174, 11973, 12594, 16846, 25074, 33129, 40606, 47709,
];
const OUT_READS: [u32; 4] = [59361, 60233, 60880, 61664];
/// c[71..85]: input, four diffusers, five combs, four output diffusers.
const WRITES: [u32; 14] = [
    0, 8821, 10146, 11175, 11974, 12595, 21257, 29485, 37540, 45017, 58627, 59362, 60234, 60881,
];
const EARLY_BASE: u32 = 62000;
const EARLY_SECOND: u32 = 19000;
const EARLY_FIR: [u32; 4] = [88, 900, 2230, 3299];
const EARLY_FIR_GAINS: [f32; 4] = [-0.915_039, -0.332_739, -0.120_996, -0.044];
const EARLY_MIX: f32 = 0.21;
// Diffuser shape c[40], c[42], c[43] and the comb split c[45].
const DIFF_IN: f32 = 0.75;
const DIFF_DIRECT: f32 = 0.625;
const DIFF_DELAYED: f32 = -0.5;
const SPLIT: f32 = 0.028_157;
const DC_BLOCK: FirstOrder = FirstOrder::new(0.998_577, -0.998_577, 0.997_155);
const STEP: f32 = 0.001_889;

const R_APF: usize = 0;
const R_LOW: usize = 1;
const R_HIGH: usize = 6;
const R_WET: usize = 11;
const R_DRY: usize = 12;
const R_TONE: usize = 13; // 5 words
const RAMPS: usize = 18;

pub struct ChamberCave {
    config: &'static Config,
    sample_rate: f32,
    ring: Ring,
    reads: [usize; 9],
    out_reads: [usize; 4],
    output_taps: [usize; 10],
    writes: [usize; 14],
    early_a: usize,
    early_b: usize,
    early_fir: [usize; 4],
    early_taps: [usize; 8],
    predelay: f32,
    predelay_target: f32,
    predelay_max: f32,
    predelay_coefficient: f32,
    split: f32,
    input_hp: FirstOrder,
    input_hp_state: FirstOrderState,
    tone: Biquad,
    damping: FirstOrder,
    dc_block: FirstOrder,
    diffuser_state: [FirstOrderState; 4],
    low: [f32; 5],
    lane_state: [[FirstOrderState; 2]; 5],
    post_tone: Option<FirstOrder>,
    post_tone_state: [FirstOrderState; 2],
    output: [[Biquad; 2]; 2],
    ramps: Ramps<RAMPS>,
    moving: bool,
    counter: usize,
    values: Option<Values>,
}

impl ChamberCave {
    pub fn new(config: &'static Config, sample_rate: f32) -> Self {
        let early_a = scale(EARLY_BASE, sample_rate);
        let early_b = scale(EARLY_BASE + EARLY_SECOND, sample_rate);
        let longest = config.early_taps.iter().copied().max().unwrap_or(0);
        let span = scale(EARLY_BASE + EARLY_SECOND + longest, sample_rate) + 2;
        let output_filters = config
            .output_filters
            .map(|row| super::common::rom_biquad(row).resample(ORIGINAL_RATE, sample_rate));
        Self {
            config,
            sample_rate,
            ring: Ring::new(span),
            reads: READS.map(|o| scale(o, sample_rate)),
            out_reads: OUT_READS.map(|o| scale(o, sample_rate)),
            output_taps: config.output_taps.map(|o| scale(o, sample_rate)),
            writes: WRITES.map(|o| scale(o, sample_rate)),
            early_a,
            early_b,
            early_fir: EARLY_FIR.map(|o| early_a + scale(o, sample_rate)),
            early_taps: config.early_taps.map(|o| early_b + scale(o, sample_rate)),
            predelay: 1.0,
            predelay_target: 1.0,
            predelay_max: scale(WRITES[1], sample_rate) as f32 - 1.0,
            predelay_coefficient: crate::dsp::one_pole_coefficient(0.03, sample_rate),
            split: 1.0 - (1.0 - SPLIT).powf(ORIGINAL_RATE / sample_rate),
            input_hp: config.input_hp.resample(sample_rate),
            input_hp_state: FirstOrderState::default(),
            tone: Biquad::new(Coefficients::IDENTITY),
            damping: config.damping.resample(sample_rate),
            dc_block: DC_BLOCK.resample(sample_rate),
            diffuser_state: [FirstOrderState::default(); 4],
            low: [0.0; 5],
            lane_state: [[FirstOrderState::default(); 2]; 5],
            post_tone: config.tone.map(|t| t.resample(sample_rate)),
            post_tone_state: [FirstOrderState::default(); 2],
            output: [
                output_filters.map(Biquad::new),
                output_filters.map(Biquad::new),
            ],
            ramps: Ramps::new(STEP, BLOCK, sample_rate),
            moving: false,
            counter: 0,
            values: None,
        }
    }

    /// Tone low-pass frequency of `tone_edit`.
    pub fn tone_hz(tone: f32) -> f32 {
        (1000.0 + 4000.0 * (tone.clamp(0.0, 100.0) / 100.0).powf(0.6)).round()
    }

    /// Comb gains (low, high) for a Decay value.
    pub fn comb_gains(config: &Config, decay: f32) -> ([f32; 5], [f32; 5]) {
        let time = tbl(&config.decay_ms_tbl, 101, decay, 0.0);
        let gain = |n: usize, t: f32| super::common::comb_gain(COMB_BASE[n], COMB_MS[n], t);
        (
            std::array::from_fn(|n| gain(n, time * config.low_factor)),
            std::array::from_fn(|n| gain(n, time)),
        )
    }
}

impl Effect for ChamberCave {
    /// `values`: Decay, Tone, Mix, PreD.
    fn set(&mut self, values: &Values, _bpm: f32) {
        if self.values.as_ref() == Some(values) {
            return;
        }
        let first = self.values.is_none();
        self.values = Some(*values);
        let [decay, tone, mix, predelay, ..] = *values;
        let c = self.config;
        let (low, high) = Self::comb_gains(c, decay);
        let t = &mut self.ramps.target;
        t[R_APF] = tbl(&c.apf_tbl, 101, decay, 0.0);
        t[R_LOW..R_LOW + 5].copy_from_slice(&low);
        t[R_HIGH..R_HIGH + 5].copy_from_slice(&high);
        t[R_WET] = tbl(&c.wet_tbl, 101, mix, 0.0);
        t[R_DRY] = tbl(&DRY_TBL, 101, mix, 0.0);
        let lp = design(
            Design::LowPass2,
            self.sample_rate,
            Self::tone_hz(tone),
            0.9,
            0.0,
        );
        t[R_TONE..R_TONE + 5].copy_from_slice(&[lp.b0, lp.b1, lp.b2, lp.a1, lp.a2]);
        let samples = (predelay.max(0.0) * 44.1).floor() + 1.0;
        self.predelay_target =
            (samples * self.sample_rate / ORIGINAL_RATE).clamp(1.0, self.predelay_max);
        if first {
            self.ramps.jump();
            self.predelay = self.predelay_target;
            self.tone.coefficients = lp;
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
                let t = &self.ramps.current[R_TONE..R_TONE + 5];
                self.tone.coefficients = Coefficients {
                    b0: t[0],
                    b1: t[1],
                    b2: t[2],
                    a1: t[3],
                    a2: t[4],
                };
            }
        }
        let g = &self.ramps.current;
        let allpass = g[R_APF];
        self.predelay += (self.predelay_target - self.predelay) * self.predelay_coefficient;
        self.ring.advance();
        let ring = &self.ring;
        let r = &self.reads;
        let predelayed = ring.read_frac(self.predelay);

        let mono = 0.5 * (frame[0] + frame[1]);
        let input = self
            .tone
            .process(self.input_hp_state.process(&self.input_hp, mono));

        let mut node = predelayed;
        let mut diffuser_writes = [0.0; 4];
        for (section, write) in diffuser_writes.iter_mut().enumerate() {
            let delayed = ring.read(r[section]);
            let incoming = DIFF_IN * node + allpass * delayed;
            *write = self.diffuser_state[section].process(&self.damping, incoming);
            node = DIFF_DELAYED * delayed + DIFF_DIRECT * node;
        }

        let complement = 1.0 - self.split;
        let mut comb_writes = [0.0; 5];
        for lane in 0..5 {
            let x = ring.read(r[4 + lane]);
            let previous = self.low[lane];
            let low = self.split * x + complement * previous;
            let high = complement * (x - previous);
            self.low[lane] = low;
            let incoming = node + g[R_HIGH + lane] * high + g[R_LOW + lane] * low;
            let [first, second] = &mut self.lane_state[lane];
            let damped = first.process(&self.damping, incoming);
            comb_writes[lane] = second.process(&self.dc_block, damped);
        }

        let o = &self.output_taps;
        let [lg, rg] = self.config.tap_gains;
        let left_sum = lg
            * (ring.read(o[0])
                + ring.read(o[2])
                + ring.read(o[4])
                + ring.read(o[6])
                + ring.read(o[8]));
        let right_sum = rg
            * (ring.read(o[1])
                + ring.read(o[3])
                + ring.read(o[5])
                + ring.read(o[7])
                + ring.read(o[9]));
        let d = self.out_reads.map(|offset| ring.read(offset));
        let w35 = DIFF_IN * left_sum + allpass * d[0];
        let ln = DIFF_DIRECT * left_sum + DIFF_DELAYED * d[0];
        let w36 = allpass * d[1] + DIFF_IN * ln;
        let wet_left = DIFF_DIRECT * ln + DIFF_DELAYED * d[1];
        let w37 = allpass * d[2] + DIFF_IN * right_sum;
        let rn = DIFF_DELAYED * d[2] + DIFF_DIRECT * right_sum;
        let w38 = DIFF_IN * rn + allpass * d[3];
        let wet_right = DIFF_DELAYED * d[3] + DIFF_DIRECT * rn;

        let mut convolved = predelayed;
        for (&offset, &gain) in self.early_fir.iter().zip(&EARLY_FIR_GAINS) {
            convolved += gain * ring.read(offset);
        }
        let (mut early_left, mut early_right) = (0.0, 0.0);
        for (i, &offset) in self.early_taps.iter().enumerate() {
            let sample = ring.read(offset);
            early_left += self.config.early_left[i] * sample;
            early_right += self.config.early_right[i] * sample;
        }

        let w = self.writes;
        let ring = &mut self.ring;
        ring.write(w[0], input);
        for (i, &value) in diffuser_writes.iter().enumerate() {
            ring.write(w[1 + i], value);
        }
        for (i, &value) in comb_writes.iter().enumerate() {
            ring.write(w[5 + i], value);
        }
        ring.write(w[10], w35);
        ring.write(w[11], w36);
        ring.write(w[12], w37);
        ring.write(w[13], w38);
        ring.write(self.early_a, predelayed);
        ring.write(self.early_b, convolved);

        let (tone_left, tone_right) = match &self.post_tone {
            Some(t) => (
                self.post_tone_state[0].process(t, wet_left),
                self.post_tone_state[1].process(t, wet_right),
            ),
            None => (wet_left, wet_right),
        };
        let mixed = [
            EARLY_MIX * early_left + tone_left,
            EARLY_MIX * early_right + tone_right,
        ];
        let (wet, dry) = (g[R_WET], g[R_DRY]);
        let mut out = [0.0; 2];
        for ch in 0..2 {
            let [a, b] = &mut self.output[ch];
            out[ch] = wet * b.process(a.process(mixed[ch])) + dry * frame[ch];
        }
        out
    }

    fn clear(&mut self) {
        self.ring.clear();
        self.input_hp_state.clear();
        self.tone.clear();
        self.diffuser_state = Default::default();
        self.low = [0.0; 5];
        self.lane_state = Default::default();
        self.post_tone_state = Default::default();
        self.output.iter_mut().flatten().for_each(Biquad::clear);
    }
}
