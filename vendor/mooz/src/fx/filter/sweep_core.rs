//! The 26-word envelope-filter callback shared by B_ATWAH.ZDL
//! (`Fx_FLT_BaAutoWah`, 0420..0700), A_FILTER.ZDL (`Fx_FLT_A_Filter`,
//! 0420..0760) and Z_TRON.ZDL (`Fx_FLT_Z_Tron`, 0000..0640). Per sample:
//!
//! - mono input `m = (L + R) / 2`;
//! - detector: section c11/c12/c13, then section c14/c15/c16. In BaAutoWah the
//!   c16 feedback multiplies the previous peak (state word 8 receives the
//!   peak), in A-Filter and Z Tron the section's own output;
//! - peak = max(|d|, c17 peak); `e0 = c19 e0 + c18 peak`,
//!   `e1 = c21 e1 + c20 e0`;
//! - sweep `min(16 c22 e1, 1)` (BaAutoWah) or `min(16 c22 e1^2, 1)`;
//!   `g = c24 + sweep c23` (written to c7);
//! - SVF in quarter scale: `high = c5 m + 4 c6 band - low`, `band += 4g high`,
//!   `low += 4g band` (c6 holds minus a quarter of the damping);
//! - `wet = 4 (c10 m + 4 c25 c3 (c8 band + c9 low)) c2`; both channels get
//!   the mono wet (c0 = 1).
use super::common::{Duck, FirstOrder, STEP_DEFAULT, SvfFrequency, decay, one_pole};
use crate::dsp::flush;
use crate::zoom::Ramp;

/// ROM words (44.1 kHz) and the model's variant switches.
pub struct Rom {
    /// c11, c12, c13.
    pub first: [f32; 3],
    /// c14, c15, c16.
    pub second: [f32; 3],
    /// Peak decay.
    pub c17: f32,
    /// Output scale.
    pub c25: f32,
    /// A-Filter / Z Tron square the envelope before the sweep.
    pub squared: bool,
    /// BaAutoWah feeds the peak back into the second detector section.
    pub peak_feedback: bool,
    /// Ramp step of the Sense editor's c22 writes.
    pub sense_step: f32,
}

/// One Sense direction (the words the Sense editor copies on a group change).
#[derive(Debug, Clone, Copy)]
pub struct Direction {
    pub c18: f32,
    pub c19: f32,
    pub c20: f32,
    pub c21: f32,
    pub c23: f32,
    pub c24: f32,
    pub c8: f32,
    pub c9: f32,
}

/// What the Sense and Reso/Peak editors set for one value combination.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Settings {
    pub group: usize,
    /// SVF input gain.
    pub c5: f32,
    /// Minus a quarter of the SVF damping.
    pub c6: f32,
    /// Sensitivity.
    pub c22: f32,
}

pub struct SweepCore {
    first: FirstOrder,
    second: FirstOrder,
    c17: f32,
    c25x16: f32,
    squared: bool,
    peak_feedback: bool,
    groups: [Direction; 2],
    group: Direction,
    active: Option<usize>,
    pending: Option<Settings>,
    duck: Duck,
    c5: Ramp,
    c6: Ramp,
    c22: Ramp,
    dry: Ramp,
    level: Ramp,
    frequency: SvfFrequency,
    input: f32,
    first_out: f32,
    second_out: f32,
    peak: f32,
    e0: f32,
    e1: f32,
    band: f32,
    low: f32,
}

impl SweepCore {
    pub fn new(sample_rate: f32, rom: &Rom, directions: &[Direction; 2]) -> Self {
        let convert = |d: &Direction| {
            let (c18, c19) = one_pole(d.c18, d.c19, sample_rate);
            let (c20, c21) = one_pole(d.c20, d.c21, sample_rate);
            Direction {
                c18,
                c19,
                c20,
                c21,
                ..*d
            }
        };
        let groups = [convert(&directions[0]), convert(&directions[1])];
        let [b0, b1, a] = rom.first;
        let first = FirstOrder::rom(b0, b1, a, sample_rate);
        let [b0, b1, a] = rom.second;
        let second = FirstOrder::rom(b0, b1, a, sample_rate);
        Self {
            first,
            second,
            c17: decay(rom.c17, sample_rate),
            c25x16: 16.0 * rom.c25,
            squared: rom.squared,
            peak_feedback: rom.peak_feedback,
            groups,
            group: groups[0],
            active: None,
            pending: None,
            duck: Duck::new(sample_rate),
            c5: Ramp::new(0.0, STEP_DEFAULT, sample_rate),
            c6: Ramp::new(0.0, STEP_DEFAULT, sample_rate),
            c22: Ramp::new(0.0, rom.sense_step, sample_rate),
            dry: Ramp::new(0.0, STEP_DEFAULT, sample_rate),
            level: Ramp::new(1.0, STEP_DEFAULT, sample_rate),
            frequency: SvfFrequency::new(sample_rate),
            input: 0.0,
            first_out: 0.0,
            second_out: 0.0,
            peak: 0.0,
            e0: 0.0,
            e1: 0.0,
            band: 0.0,
            low: 0.0,
        }
    }

    fn jump(&mut self, settings: &Settings) {
        self.group = self.groups[settings.group];
        self.active = Some(settings.group);
        self.c5.jump(settings.c5);
        self.c6.jump(settings.c6);
        self.c22.jump(settings.c22);
    }

    /// Applies the Sense/Reso coefficients; a direction change ducks the
    /// filter output (the editor ramps the output word to 0 and back).
    pub fn configure(&mut self, settings: Settings) {
        match self.active {
            None => self.jump(&settings),
            Some(group) if group == settings.group && !self.duck.is_falling() => {
                self.c5.set(settings.c5);
                self.c6.set(settings.c6);
                self.c22.set(settings.c22);
            }
            Some(_) => {
                self.pending = Some(settings);
                self.duck.start();
            }
        }
    }

    /// Dry editor: c10 = Dry / 100 * 0.25 (the wet path multiplies by 4), and
    /// Level editor: c2 = Level / 100.
    pub fn set_mix(&mut self, dry: f32, level: f32) {
        if self.active.is_none() {
            self.dry.jump(dry);
            self.level.jump(level);
        } else {
            self.dry.set(dry);
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
        let c5 = self.c5.tick();
        let c6 = self.c6.tick();
        let c22 = self.c22.tick();
        let dry = self.dry.tick();
        let level = self.level.tick();
        let x = 0.5 * (frame[0] + frame[1]);
        let first = self.first.b0 * x + self.first.b1 * self.input + self.first.a * self.first_out;
        let feedback = if self.peak_feedback {
            self.peak
        } else {
            self.second_out
        };
        let second =
            self.second.b0 * first + self.second.b1 * self.first_out + self.second.a * feedback;
        self.input = x;
        self.first_out = flush(first);
        self.second_out = flush(second);
        let magnitude = second.abs();
        let held = self.c17 * self.peak;
        self.peak = flush(if magnitude < held { held } else { magnitude });
        self.e0 = flush(g.c19 * self.e0 + g.c18 * self.peak);
        self.e1 = flush(g.c21 * self.e1 + g.c20 * self.e0);
        let shaped = if self.squared {
            self.e1 * self.e1
        } else {
            self.e1
        };
        let sweep = (16.0 * c22 * shaped).min(1.0);
        let f = self.frequency.map(4.0 * (g.c24 + sweep * g.c23));
        let high = c5 * x + 4.0 * c6 * self.band - self.low;
        self.band = flush(self.band + f * high);
        self.low = flush(self.low + f * self.band);
        let filtered = g.c8 * self.band + g.c9 * self.low;
        let out = (dry * x + self.c25x16 * self.duck.gain * filtered) * level;
        [out, out]
    }

    pub fn clear(&mut self) {
        self.input = 0.0;
        self.first_out = 0.0;
        self.second_out = 0.0;
        self.peak = 0.0;
        self.e0 = 0.0;
        self.e1 = 0.0;
        self.band = 0.0;
        self.low = 0.0;
        if let Some(settings) = self.pending.take() {
            self.jump(&settings);
        }
        self.duck.reset();
        self.active = None;
    }
}
