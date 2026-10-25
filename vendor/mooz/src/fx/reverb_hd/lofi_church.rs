//! The kernel shared by LOFI_REV.ZDL and CHURCH.ZDL (`Fx_REV_tc_LOFI`,
//! `Fx_REV_tc_CHURCH`), following the recovered callback (zoom-ms70-cdr
//! `lofi_church/filters.rs`, `reverb.rs`):
//!
//! stereo input → two fixed EQ biquads (this filtered signal is also the dry
//! path) → two more input sections → cross-mixed "drives" into L/R predelay
//! lines read through a parabolic LFO → plus a damped cross feedback loop
//! (L line → R and back, 410 ms round trip) → four series diffusers per side
//! (`w = x + g d`, `y = c56 d - 0.84 x`) → eight self-fed combs (four per side,
//! two of them modulated by a slow LFO in LOFI), each split into low/high bands
//! by a one-pole, then a low-pass (Tone_Hid_F) and a shelf → output taps,
//! diffuser output and predelayed input summed → two output sections →
//! `FxLVL/MIX · wet + Dry · filtered input`.
//!
//! Editors (disassembly + firmware sweep `lofi-church-parameters-v1`):
//! - `decay_set`: low-band time `T` (LOFI `12000·t²·(0.8 - 0.3t)`, Church
//!   `200 + 19800·t^1.9`, `t = DECAY/100`), high band `T · HighTime_Ratio_by_Tone`;
//!   comb gains `base^(D/T)` from *_DelayLoopTime_mS_tbl (LOFI also drives
//!   the 410 ms cross loop); diffuser gains from *_APF_FB_tbl / *_APF_WET_tbl.
//! - `tone_edit`: the high-band ratio above, and the output shelf (Main design
//!   8: LOFI fixed +3.5 dB at 8 kHz, Church *_Tone_PostEQ_1_G_tbl at 4 kHz).
//! - Church `HiDamp_edit`: comb low-pass (Main design 0) at *_Tone_Hid_F_tbl
//!   (LOFI: a fixed 15 kHz).
//! - `mix_edit`: wet gain from *_Mix_tbl; `Dry`/`KillDry_edit`: dry on/off.
//! - `predelay_edit`: Church `round(PreD · 44.1)` samples (min 2); LOFI's
//!   switch picks 521 or 829 samples. The right line is written 477 samples
//!   later in LOFI's ROM than the editor assumes, so its right predelay is
//!   477 samples shorter — kept as in the original.
use super::common::{
    BLOCK, Biquad, Design, FirstOrder, FirstOrderState, Ramps, Ring, comb_gain, design, design1,
    rom_biquad, scale,
};

use crate::zoom::ORIGINAL_RATE;

/// Per-model ROM constants (offsets in 44.1 kHz samples).
pub struct Config {
    /// c[59..71]: input L/R, feedback L/R, diffusers L 1..4, R 1..4.
    pub writes: [u32; 12],
    /// c[75..83]: combs 1..8.
    pub comb_writes: [u32; 8],
    /// c[9], c[10]: the cross-loop reads (fed to the right / left path).
    pub loop_reads: [u32; 2],
    /// c[11..19]: diffuser reads L 1..4, R 1..4.
    pub diffuser_reads: [u32; 8],
    /// Comb reads: the modulated bases c[142..146] for combs 1, 2, 5, 6 and
    /// the fixed c[27], c[28], c[33], c[34] for combs 3, 4, 7, 8.
    pub comb_reads: [u32; 8],
    /// c[35..43]: output taps L 1..4, R 1..4.
    pub output_taps: [u32; 8],
    /// c[112]: the opposite channel's share of each drive.
    pub cross_mix: f32,
    /// c[109], c[110], c[111]: output mix of the predelayed centre, the
    /// diffuser outputs and the output taps.
    pub centre_mix: f32,
    pub diffuser_mix: f32,
    pub tap_mix: f32,
    /// c[92]: diffuser output into each comb.
    pub comb_input: f32,
    pub split: f32,
    pub input_sections: [[f32; 5]; 2],
    pub comb_shelf: FirstOrder,
    pub output_section: [f32; 5],
    pub shelf_hz: f32,
    /// LFO1 (predelay) and LFO2 (combs): depth in samples, Q32 increment.
    pub lfo: [(f32, u32); 2],
    pub apf_tbl: [f32; 11],
    pub apf_wet_tbl: [f32; 11],
    pub high_ratio_tbl: [f32; 11],
    pub mix_tbl: [f32; 11],
    pub loop_ms: [f32; 4],
    pub loop_base: [f32; 4],
    /// The 410 ms cross loop (LOFI only).
    pub cross_loop: Option<(f32, f32)>,
}

const FRONT: [f32; 5] = [0.988_557, -1.465_969, 0.602_334, 1.465_969, -0.590_891];
const INPUT: [f32; 5] = [0.977_861, -0.175_855, 0.230_608, 0.175_855, -0.208_469];
const DIFFUSER_DIRECT: f32 = -0.84;
const STEP: f32 = 0.001_889;

const R_APF: usize = 0;
const R_APF_WET: usize = 1;
const R_LOW: usize = 2;
const R_HIGH: usize = 6;
const R_LOOP: usize = 10; // low, high
const R_WET: usize = 12;
const R_DRY: usize = 13;
const R_DAMP: usize = 14; // 3 words
const R_SHELF: usize = 17; // 3 words
const RAMPS: usize = 20;

/// Editor results in pedal-independent form.
#[derive(Debug, Clone, Copy, Default)]
pub struct Settings {
    pub apf: f32,
    pub apf_wet: f32,
    pub low: [f32; 4],
    pub high: [f32; 4],
    pub loop_gains: [f32; 2],
    pub wet: f32,
    pub dry: f32,
    pub damp_hz: f32,
    pub shelf_db: f32,
    /// Left predelay base c[152] (44.1 kHz samples).
    pub predelay: f32,
}

impl Config {
    /// Decay gains for low/high band times in ms.
    pub fn gains(&self, low_ms: f32, high_ms: f32) -> ([f32; 4], [f32; 4], [f32; 2]) {
        let low = std::array::from_fn(|n| comb_gain(self.loop_base[n], self.loop_ms[n], low_ms));
        let high = std::array::from_fn(|n| comb_gain(self.loop_base[n], self.loop_ms[n], high_ms));
        let cross = self.cross_loop.map_or([0.0; 2], |(ms, base)| {
            [comb_gain(base, ms, low_ms), comb_gain(base, ms, high_ms)]
        });
        (low, high, cross)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Lfo {
    phase: u32,
    increment: u32,
    depth: f32,
}

impl Lfo {
    /// The parabolic 0..1 wave of `modulation()`, times the depth.
    #[inline]
    fn tick(&mut self) -> f32 {
        self.phase = self.phase.wrapping_add(self.increment);
        let a = (self.phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0);
        let f = a - 0.5;
        (2.0 * f * (1.0 - f.abs()) + 0.5) * self.depth
    }
}

pub struct LofiChurch {
    config: &'static Config,
    sample_rate: f32,
    ring: Ring,
    writes: [usize; 12],
    comb_writes: [usize; 8],
    loop_reads: [usize; 2],
    diffuser_reads: [usize; 8],
    comb_reads: [f32; 8],
    /// `comb_reads` as whole offsets (all of them are integral).
    comb_whole: [usize; 8],
    /// False when LFO2 has no depth (Church): its offset is then always 0
    /// and the modulated reads land exactly on `comb_whole`.
    comb_modulated: bool,
    output_taps: [usize; 8],
    right_offset: f32,
    right_shift: f32,
    lfo: [Lfo; 2],
    predelay: f32,
    predelay_target: f32,
    predelay_coefficient: f32,
    predelay_limits: (f32, f32),
    split: f32,
    front: [[Biquad; 2]; 2],
    input_sections: [[Biquad; 2]; 2],
    comb_shelf: FirstOrder,
    loop_low: [f32; 2],
    loop_state: [[FirstOrderState; 2]; 2],
    comb_low: [f32; 8],
    comb_state: [[FirstOrderState; 2]; 8],
    output_section: [Biquad; 2],
    shelf_state: [FirstOrderState; 2],
    ramps: Ramps<RAMPS>,
    moving: bool,
    counter: usize,
    primed: bool,
}

impl LofiChurch {
    pub fn new(config: &'static Config, sample_rate: f32) -> Self {
        let ratio = sample_rate / ORIGINAL_RATE;
        let longest = config
            .output_taps
            .iter()
            .chain(&config.comb_reads)
            .copied()
            .max()
            .unwrap_or(0);
        let span = scale(longest, sample_rate) + (config.lfo[1].0 * ratio) as usize + 4;
        let biquad =
            |row: [f32; 5]| Biquad::new(rom_biquad(row).resample(ORIGINAL_RATE, sample_rate));
        let front = [biquad(FRONT), biquad(INPUT)];
        let inputs = config.input_sections.map(biquad);
        // Predelay reads must stay inside their lines.
        let low_limit = 1.0;
        let high_limit = (config.writes[1] as f32 - 2.0 - config.lfo[0].0) * ratio;
        Self {
            config,
            sample_rate,
            ring: Ring::new(span),
            writes: config.writes.map(|o| scale(o, sample_rate)),
            comb_writes: config.comb_writes.map(|o| scale(o, sample_rate)),
            loop_reads: config.loop_reads.map(|o| scale(o, sample_rate)),
            diffuser_reads: config.diffuser_reads.map(|o| scale(o, sample_rate)),
            // The modulated reads use the word before `base + coarse`.
            comb_reads: std::array::from_fn(|n| {
                let modulated = matches!(n, 0 | 1 | 4 | 5);
                scale(config.comb_reads[n], sample_rate) as f32 - if modulated { 1.0 } else { 0.0 }
            }),
            comb_whole: std::array::from_fn(|n| {
                let modulated = matches!(n, 0 | 1 | 4 | 5);
                scale(config.comb_reads[n], sample_rate) - usize::from(modulated)
            }),
            comb_modulated: config.lfo[1].0 != 0.0,
            output_taps: config.output_taps.map(|o| scale(o, sample_rate)),
            right_offset: scale(config.writes[1], sample_rate) as f32,
            right_shift: (8949.0 - config.writes[1] as f32) * ratio,
            lfo: config.lfo.map(|(depth, increment)| Lfo {
                phase: 0,
                increment: (increment as f32 / ratio).round() as u32,
                depth: depth * ratio,
            }),
            predelay: 0.0,
            predelay_target: 0.0,
            predelay_coefficient: crate::dsp::one_pole_coefficient(0.03, sample_rate),
            predelay_limits: (low_limit, high_limit),
            split: 1.0 - (1.0 - config.split).powf(1.0 / ratio),
            front: [front, front],
            input_sections: [inputs, inputs],
            comb_shelf: config.comb_shelf.resample(sample_rate),
            loop_low: [0.0; 2],
            loop_state: Default::default(),
            comb_low: [0.0; 8],
            comb_state: Default::default(),
            output_section: [biquad(config.output_section), biquad(config.output_section)],
            shelf_state: Default::default(),
            ramps: Ramps::new(STEP, BLOCK, sample_rate),
            moving: false,
            counter: 0,
            primed: false,
        }
    }

    pub fn apply(&mut self, s: &Settings) {
        let damp = design1(Design::LowPass1, self.sample_rate, s.damp_hz, 0.0);
        let shelf = design(
            Design::HighShelf1,
            self.sample_rate,
            self.config.shelf_hz,
            0.707,
            s.shelf_db,
        );
        let t = &mut self.ramps.target;
        t[R_APF] = s.apf;
        t[R_APF_WET] = s.apf_wet;
        t[R_LOW..R_LOW + 4].copy_from_slice(&s.low);
        t[R_HIGH..R_HIGH + 4].copy_from_slice(&s.high);
        t[R_LOOP..R_LOOP + 2].copy_from_slice(&s.loop_gains);
        t[R_WET] = s.wet;
        t[R_DRY] = s.dry;
        t[R_DAMP..R_DAMP + 3].copy_from_slice(&[damp.b0, damp.b1, damp.f1]);
        t[R_SHELF..R_SHELF + 3].copy_from_slice(&[shelf.b0, shelf.b1, -shelf.a1]);
        let ratio = self.sample_rate / ORIGINAL_RATE;
        // The read sits one word before `c[152] + coarse`.
        self.predelay_target =
            ((s.predelay - 1.0) * ratio).clamp(self.predelay_limits.0, self.predelay_limits.1);
        if self.primed {
            self.moving = true;
        } else {
            self.primed = true;
            self.ramps.jump();
            self.predelay = self.predelay_target;
        }
    }

    pub fn clear(&mut self) {
        self.ring.clear();
        self.front.iter_mut().flatten().for_each(Biquad::clear);
        self.input_sections
            .iter_mut()
            .flatten()
            .for_each(Biquad::clear);
        self.loop_low = [0.0; 2];
        self.loop_state = Default::default();
        self.comb_low = [0.0; 8];
        self.comb_state = Default::default();
        self.output_section.iter_mut().for_each(Biquad::clear);
        self.shelf_state = Default::default();
    }

    #[inline]
    pub fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.moving {
            self.counter += 1;
            if self.counter >= BLOCK {
                self.counter = 0;
                self.moving = self.ramps.tick();
            }
        }
        let g = &self.ramps.current;
        let c = self.config;
        let damp = FirstOrder::new(g[R_DAMP], g[R_DAMP + 1], g[R_DAMP + 2]);
        let shelf = FirstOrder::new(g[R_SHELF], g[R_SHELF + 1], g[R_SHELF + 2]);
        self.predelay += (self.predelay_target - self.predelay) * self.predelay_coefficient;
        let predelay_mod = self.lfo[0].tick();
        let comb_mod = self.lfo[1].tick();

        self.ring.advance();
        let ring = &self.ring;
        let left_delay = self.predelay + predelay_mod;
        // The right line is read at c[152] + 8949 but written at c[60].
        let right_delay = (left_delay + self.right_shift).max(1.0);
        let predelayed = [
            ring.read_frac(left_delay),
            ring.read_frac(self.right_offset + right_delay),
        ];

        // Input filters (the dry path) and the cross-mixed drives.
        let mut filtered = [0.0; 2];
        let mut toned = [0.0; 2];
        for ch in 0..2 {
            let [front, input] = &mut self.front[ch];
            filtered[ch] = input.process(front.process(frame[ch]));
            let [a, b] = &mut self.input_sections[ch];
            toned[ch] = b.process(a.process(filtered[ch]));
        }
        let drive = [
            toned[1] + c.cross_mix * toned[0],
            toned[0] + c.cross_mix * toned[1],
        ];

        // Cross feedback loop: the left path hears the right line and back.
        let complement = 1.0 - self.split;
        // Without the cross loop (Church) its gains stay 0, so both sections
        // only ever output +0 and `loop_in` is the predelayed input.
        let mut loop_in = predelayed;
        if c.cross_loop.is_some() {
            for (path, read) in [(0usize, 1usize), (1, 0)] {
                let x = ring.read(self.loop_reads[read]);
                let previous = self.loop_low[path];
                let low = self.split * x + complement * previous;
                let high = complement * (x - previous);
                self.loop_low[path] = low;
                let [a, b] = &mut self.loop_state[path];
                let damped = a.process(&damp, g[R_LOOP] * low + g[R_LOOP + 1] * high);
                loop_in[path] = b.process(&self.comb_shelf, damped) + predelayed[path];
            }
        }

        // Four diffusers per side.
        let (apf, apf_wet) = (g[R_APF], g[R_APF_WET]);
        let mut diffuser_writes = [0.0; 8];
        let mut diffused = [0.0; 2];
        for side in 0..2 {
            let mut x = loop_in[side];
            for k in 0..4 {
                let d = ring.read(self.diffuser_reads[side * 4 + k]);
                diffuser_writes[side * 4 + k] = x + apf * d;
                x = apf_wet * d + DIFFUSER_DIRECT * x;
            }
            diffused[side] = x;
        }

        // Eight combs.
        let mut comb_writes = [0.0; 8];
        for n in 0..8 {
            let x = if self.comb_modulated && matches!(n, 0 | 1 | 4 | 5) {
                ring.read_frac(self.comb_reads[n] + comb_mod)
            } else {
                ring.read(self.comb_whole[n])
            };
            let previous = self.comb_low[n];
            let low = self.split * x + complement * previous;
            let high = complement * (x - previous);
            self.comb_low[n] = low;
            let pair = n % 4;
            let incoming =
                c.comb_input * diffused[n / 4] + g[R_LOW + pair] * low + g[R_HIGH + pair] * high;
            let [a, b] = &mut self.comb_state[n];
            comb_writes[n] = b.process(&self.comb_shelf, a.process(&damp, incoming));
        }

        // Output.
        let o = &self.output_taps;
        let taps = [
            ring.read(o[0]) + ring.read(o[1]) + ring.read(o[2]) + ring.read(o[3]),
            ring.read(o[4]) + ring.read(o[5]) + ring.read(o[6]) + ring.read(o[7]),
        ];
        let centre = c.centre_mix * 0.5 * (predelayed[0] + predelayed[1]);
        let (wet, dry) = (g[R_WET], g[R_DRY]);
        let mut out = [0.0; 2];
        for ch in 0..2 {
            let sum = centre + c.tap_mix * taps[ch] + c.diffuser_mix * diffused[ch];
            let shaped = self.shelf_state[ch].process(&shelf, self.output_section[ch].process(sum));
            out[ch] = wet * shaped + dry * filtered[ch];
        }

        let ring = &mut self.ring;
        let w = self.writes;
        ring.write(w[0], drive[0]);
        ring.write(w[1], drive[1]);
        ring.write(w[2], loop_in[0]);
        ring.write(w[3], loop_in[1]);
        for (k, &value) in diffuser_writes.iter().enumerate() {
            ring.write(w[4 + k], value);
        }
        for (n, &value) in comb_writes.iter().enumerate() {
            ring.write(self.comb_writes[n], value);
        }
        out
    }
}
