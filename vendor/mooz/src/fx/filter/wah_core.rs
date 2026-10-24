//! The 33-word envelope-filter callback shared by AUTOWAH.ZDL
//! (`Fx_FLT_AutoWah`, 7800..7e00) and RESONS.ZDL (`Fx_FLT_Resonance`,
//! 0000..0620); the two callbacks are the same graph:
//!
//! - mono input `m = (L + R) / 2`;
//! - detector: first-order section c7/c8/c9, then a second section c10/c11
//!   whose feedback word c12 multiplies the previous *peak* (state words 15
//!   and 17 receive the peak, not the section output — kept as the firmware
//!   does it), magnitude `|x| * c23` clipped to 1, peak hold with decay c13;
//! - two smoothers: `e1 = c15 e1 + c14 c25 peak`, `e2 = c16 e1 + c17 e2`;
//! - sweep `min(c24 e2, 1)` (c21 = 0: linear curve, c22 = 1);
//! - Chamberlin SVF with frequency `c20 + sweep * c19` (written to c27 every
//!   sample), input gain c18, damping c28, output `c30 band + c31 low`;
//! - `wet = (m c6 + out c3 c5) c2`, both channels `c4 ((1 - c0) x + c0 wet)`
//!   (c0 = c4 = c5 = 1, c6 = 0 here: the output is the mono filter).
//!
//! Rate conversion: the detector sections are re-derived through their analog
//! prototypes, decays and smoothers keep their time constants and DC gains,
//! and the SVF coefficient keeps its cutoff (`SvfFrequency`).
use super::common::{Duck, FirstOrder, STEP_DEFAULT, STEP_FAST, SvfFrequency, decay, one_pole};
use crate::dsp::flush;
use crate::zoom::Ramp;

/// Detector words of the ROM block (44.1 kHz).
pub struct Rom {
    /// c7, c8, c9.
    pub first: [f32; 3],
    /// c10, c11, c12 (c12 multiplies the previous peak).
    pub second: [f32; 3],
    /// Peak decay.
    pub c13: f32,
    /// Second smoother input gain.
    pub c16: f32,
}

/// One Sense direction: the words the Sense editor copies when Sense crosses
/// between its negative (down) and positive (up) halves.
#[derive(Debug, Clone, Copy)]
pub struct Direction {
    pub c19: f32,
    pub c20: f32,
    pub c14: f32,
    pub c15: f32,
    pub c17: f32,
    pub c30: f32,
    pub c31: f32,
}

/// Everything the Sense/Reso editors set for one value pair.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub group: usize,
    pub c18: f32,
    pub c28: f32,
    pub c24: f32,
}

#[derive(Debug, Clone, Copy)]
struct Group {
    c14: f32,
    c15: f32,
    c16: f32,
    c17: f32,
    c19: f32,
    c20: f32,
    c30: f32,
    c31: f32,
}

pub struct WahCore {
    first: FirstOrder,
    second: FirstOrder,
    c13: f32,
    groups: [Group; 2],
    group: Group,
    active: Option<usize>,
    pending: Option<Settings>,
    duck: Duck,
    c18: Ramp,
    c28: Ramp,
    c24: Ramp,
    level: Ramp,
    frequency: SvfFrequency,
    input: f32,
    first_out: f32,
    peak: f32,
    e1: f32,
    e2: f32,
    band: f32,
    low: f32,
}

impl WahCore {
    pub fn new(sample_rate: f32, rom: &Rom, directions: &[Direction; 2]) -> Self {
        let group = |d: &Direction| {
            let (c14, c15) = one_pole(d.c14, d.c15, sample_rate);
            let (c16, c17) = one_pole(rom.c16, d.c17, sample_rate);
            Group {
                c14,
                c15,
                c16,
                c17,
                c19: d.c19,
                c20: d.c20,
                c30: d.c30,
                c31: d.c31,
            }
        };
        let groups = [group(&directions[0]), group(&directions[1])];
        let [b0, b1, a] = rom.first;
        let first = FirstOrder::rom(b0, b1, a, sample_rate);
        let [b0, b1, a] = rom.second;
        let second = FirstOrder::rom(b0, b1, a, sample_rate);
        Self {
            first,
            second,
            c13: decay(rom.c13, sample_rate),
            groups,
            group: groups[0],
            active: None,
            pending: None,
            duck: Duck::new(sample_rate),
            c18: Ramp::new(0.0, STEP_FAST, sample_rate),
            c28: Ramp::new(1.0, STEP_FAST, sample_rate),
            c24: Ramp::new(1.0, STEP_FAST, sample_rate),
            level: Ramp::new(1.0, STEP_DEFAULT, sample_rate),
            frequency: SvfFrequency::new(sample_rate),
            input: 0.0,
            first_out: 0.0,
            peak: 0.0,
            e1: 0.0,
            e2: 0.0,
            band: 0.0,
            low: 0.0,
        }
    }

    fn jump(&mut self, settings: &Settings) {
        self.group = self.groups[settings.group];
        self.active = Some(settings.group);
        self.c18.jump(settings.c18);
        self.c28.jump(settings.c28);
        self.c24.jump(settings.c24);
    }

    /// Applies the Sense/Reso coefficients. A direction change fades the
    /// filter out, swaps the set and fades back in, like the editor's c3 ramps.
    pub fn configure(&mut self, settings: Settings) {
        match self.active {
            None => self.jump(&settings),
            Some(group) if group == settings.group && !self.duck.is_falling() => {
                self.c18.set(settings.c18);
                self.c28.set(settings.c28);
                self.c24.set(settings.c24);
            }
            Some(_) => {
                self.pending = Some(settings);
                self.duck.start();
            }
        }
    }

    /// Level editor: c2 = value / 100 through the float setter.
    pub fn set_level(&mut self, level: f32) {
        if self.active.is_none() {
            self.level.jump(level);
        } else {
            self.level.set(level);
        }
    }

    #[inline]
    pub fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.duck.tick()
            && let Some(settings) = self.pending.take()
        {
            self.jump(&settings);
        }
        let g = self.group;
        let c18 = self.c18.tick();
        let c28 = self.c28.tick();
        let c24 = self.c24.tick();
        let level = self.level.tick();
        let x = 0.5 * (frame[0] + frame[1]);
        let first = self.first.b0 * x + self.first.b1 * self.input + self.first.a * self.first_out;
        let second =
            self.second.b0 * first + self.second.b1 * self.first_out + self.second.a * self.peak;
        self.input = x;
        self.first_out = flush(first);
        let magnitude = second.abs().min(1.0);
        self.peak = if self.peak < magnitude {
            magnitude
        } else {
            flush(self.c13 * self.peak)
        };
        self.e1 = flush(g.c15 * self.e1 + g.c14 * self.peak);
        self.e2 = flush(g.c16 * self.e1 + g.c17 * self.e2);
        let sweep = (c24 * self.e2).min(1.0);
        let f = self.frequency.map(g.c20 + sweep * g.c19);
        let high = c18 * x - c28 * self.band - self.low;
        self.band = flush(self.band + f * high);
        self.low = flush(self.low + f * self.band);
        let out = (g.c30 * self.band + g.c31 * self.low) * self.duck.gain * level;
        [out, out]
    }

    pub fn clear(&mut self) {
        self.input = 0.0;
        self.first_out = 0.0;
        self.peak = 0.0;
        self.e1 = 0.0;
        self.e2 = 0.0;
        self.band = 0.0;
        self.low = 0.0;
        if let Some(settings) = self.pending.take() {
            self.jump(&settings);
        }
        self.duck.reset();
        self.active = None;
    }
}
