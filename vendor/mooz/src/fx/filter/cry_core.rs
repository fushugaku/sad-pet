//! The talk-box style comb shared by CRY.ZDL (`Fx_FLT_Cry`, 7800..7b20) and
//! B_CRY.ZDL (`Fx_FLT_Ba_Cry`, 0520..0920), and their editors (identical
//! code and tables in both modules, only the parameter defaults differ).
//!
//! Callback, per sample:
//! - `m = (L + R) / 2`; detector: sections c12/c13/c14 and c15/c16/c17
//!   (a high shelf and a ~80 Hz high-pass), magnitude `|d c6|` clipped to 1,
//!   peak hold `max(mag, c7 peak)`, envelope `e = c21 e + c20 peak`;
//! - ring write `w = post c9 + m c5` (c9 < 0: negative feedback);
//! - tap at `c22 + e c8 2^15` samples (the Q16 phase split into an integer
//!   shift and a linear-interpolation fraction), `s = tap c3`;
//! - `post = c19 post + c18 s` (one-pole low-pass in the loop);
//! - `tone = -0.80994 c10 s`; Cry: `out = c2 (c11 x + tone)` per channel,
//!   Ba Cry: `out = c2 (c11 m + tone)` on both channels.
//!
//! Editors (decoded from the disassembly):
//! - Sense (`sens_edit`): c6 = -(0.625 + 2.875 t / 9), t = 9 - Sense for
//!   -10..-1 and Sense - 10 for 1..10 (0x3af7c5ec ramp); group = Sense / 10
//!   copies (c20, c21) from 0x800005d8 and the base delay c22 (35 or 88
//!   samples) from 0x800005e8, then re-runs Range; a group change fades c3.
//! - Range (`range_edit`, via Main's int-to-float and division): c8 =
//!   0.000513 + 0.001538 Range / 10 (down group: the tap moves out to
//!   ~97 samples) or -0.00194 + 0.000485 Range / 10 (up group: the tap moves
//!   in from 88 towards ~24 samples), through Main's 16 ms timed setter.
//! - Reso (`reso_edit`): c9 = -(0.25 + 0.6858 Reso / 10).
//! - Bal (`balance_edit`): c10 = curve(Bal, shape 2), c11 = curve(100 - Bal,
//!   shape 2) over 0..1. Level (`outLv_edit`): c2 = Level / 100.
//!
//! At other rates the tap distance keeps the loop period in time:
//! `(d44 + 1) r - 1` with `r = fs / 44100`.
use super::common::{
    Duck, FirstOrder, GLIDE_16MS, Glide, STEP_DEFAULT, STEP_FAST, clamp_value, decay, one_pole,
    step, w,
};
use crate::dsp::{DelayLine, flush};
use crate::zoom::{Ramp, curve};

pub const RANGE_DISPLAY: crate::Display = crate::Display::offset(1.0);

const DETECTOR_FIRST: [f32; 3] = [w(0x407d_cfc6), w(0xc07c_84ec), w(0x3f7c_bcfc)];
const DETECTOR_SECOND: [f32; 3] = [w(0x3f7e_8c9f), w(0xbf7e_8c9f), w(0x3f7d_193e)];
const PEAK_DECAY: u32 = 0x3f7f_f973;
const POST: (u32, u32) = (0x3ea9_e4c2, 0x3f2b_0d9f);
/// 0x800005d8: (c20, c21) per group.
const ENVELOPE: [(u32, u32); 2] = [(0x3a40_ba8f, 0x3f7f_cfd1), (0x39d1_9c5b, 0x3f7f_e5cc)];
/// 0x800005e8: base tap c22 per group, in 44.1 kHz samples.
const BASE: [f32; 2] = [35.0, 88.0];
/// Range editor: (offset, slope) of c8 per group.
const RANGE: [(u32, u32); 2] = [(0x3a06_6aa9, 0x3ac9_a002), (0xbafe_0408, 0x39fe_0408)];
/// The fixed `tone` gain 0xbf4f5800.
const TONE: f32 = w(0xbf4f_5800);
/// Longest tap at 44.1 kHz: 35 + 0.0020510 * 32768.
const MAX_TAP: f32 = 103.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Settings {
    group: usize,
    c8: f32,
}

pub struct CryCore {
    mono: bool,
    r: f32,
    first: FirstOrder,
    second: FirstOrder,
    c7: f32,
    envelope: [(f32, f32); 2],
    post_b: f32,
    post_a: f32,
    group: usize,
    active: Option<usize>,
    pending: Option<Settings>,
    duck: Duck,
    c6: Ramp,
    c8: Glide,
    c9: Ramp,
    c10: Ramp,
    c11: Ramp,
    level: Ramp,
    line: DelayLine,
    input: f32,
    first_out: f32,
    second_out: f32,
    peak: f32,
    env: f32,
    post: f32,
}

impl CryCore {
    /// `mono`: Ba Cry mixes the mono input into the wet path.
    pub fn new(sample_rate: f32, mono: bool) -> Self {
        let r = sample_rate / crate::zoom::ORIGINAL_RATE;
        let [b0, b1, a] = DETECTOR_FIRST;
        let first = FirstOrder::rom(b0, b1, a, sample_rate);
        let [b0, b1, a] = DETECTOR_SECOND;
        let second = FirstOrder::rom(b0, b1, a, sample_rate);
        let envelope = ENVELOPE.map(|(b, a)| one_pole(w(b), w(a), sample_rate));
        let (post_b, post_a) = one_pole(w(POST.0), w(POST.1), sample_rate);
        Self {
            mono,
            r,
            first,
            second,
            c7: decay(w(PEAK_DECAY), sample_rate),
            envelope,
            post_b,
            post_a,
            group: 0,
            active: None,
            pending: None,
            duck: Duck::new(sample_rate),
            c6: Ramp::new(1.0, STEP_FAST, sample_rate),
            c8: Glide::new(0.0, GLIDE_16MS, sample_rate),
            c9: Ramp::new(0.0, STEP_FAST, sample_rate),
            c10: Ramp::new(1.0, STEP_DEFAULT, sample_rate),
            c11: Ramp::new(0.0, STEP_DEFAULT, sample_rate),
            level: Ramp::new(1.0, STEP_DEFAULT, sample_rate),
            line: DelayLine::new(((MAX_TAP + 1.0) * r) as usize + 4),
            input: 0.0,
            first_out: 0.0,
            second_out: 0.0,
            peak: 0.0,
            env: 0.0,
            post: 0.0,
        }
    }

    fn jump(&mut self, settings: &Settings) {
        self.group = settings.group;
        self.active = Some(settings.group);
        self.c8.jump(settings.c8);
    }

    /// Values in catalog order: Range, Reso, Sense, Bal, Level.
    pub fn set(&mut self, values: &crate::Values) {
        let range = step(values[0], 9);
        let reso = step(values[1], 10);
        let sense = step(values[2], 19);
        let balance = clamp_value(values[3], 100.0);
        let level = clamp_value(values[4], 150.0) / 100.0;
        let group = sense / 10;
        let t = if group == 0 { 9 - sense } else { sense - 10 };
        let c6 = curve(3.5, 0.625, 10, t as f32, 0);
        let (offset, slope) = RANGE[group];
        let settings = Settings {
            group,
            c8: w(offset) + w(slope) * (range as f32 / 10.0),
        };
        let c9 = -curve(w(0x3f6f_8fdf), w(0x3e80_0100), 11, reso as f32, 0);
        let c10 = curve(1.0, 0.0, 101, balance, 2);
        let c11 = curve(1.0, 0.0, 101, 100.0 - balance, 2);
        match self.active {
            None => {
                self.jump(&settings);
                self.c6.jump(c6);
                self.c9.jump(c9);
                self.c10.jump(c10);
                self.c11.jump(c11);
                self.level.jump(level);
                return;
            }
            Some(active) if active == group && !self.duck.is_falling() => {
                self.c8.set(settings.c8);
            }
            Some(_) => {
                self.pending = Some(settings);
                self.duck.start();
            }
        }
        self.c6.set(c6);
        self.c9.set(c9);
        self.c10.set(c10);
        self.c11.set(c11);
        self.level.set(level);
    }

    #[inline]
    pub fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        if self.duck.tick()
            && let Some(settings) = self.pending.take()
        {
            self.jump(&settings);
        }
        let c6 = self.c6.tick();
        let c8 = self.c8.tick();
        let c9 = self.c9.tick();
        let c10 = self.c10.tick();
        let c11 = self.c11.tick();
        let level = self.level.tick();
        let (c20, c21) = self.envelope[self.group];
        let x = 0.5 * (frame[0] + frame[1]);
        let first = self.first.b0 * x + self.first.b1 * self.input + self.first.a * self.first_out;
        let second = self.second.b0 * first
            + self.second.b1 * self.first_out
            + self.second.a * self.second_out;
        self.input = x;
        self.first_out = flush(first);
        self.second_out = flush(second);
        let magnitude = (second * c6).abs().min(1.0);
        let held = self.c7 * self.peak;
        self.peak = flush(if held < magnitude { magnitude } else { held });
        self.env = flush(c21 * self.env + c20 * self.peak);
        self.line.write(flush(self.post * c9 + x));
        let tap = BASE[self.group] + self.env * c8 * 32_768.0;
        let s = self.line.read_linear((tap + 1.0) * self.r - 1.0) * self.duck.gain;
        self.post = flush(self.post_a * self.post + self.post_b * s);
        let tone = TONE * c10 * s;
        if self.mono {
            let out = (c11 * x + tone) * level;
            [out, out]
        } else {
            [
                (c11 * frame[0] + tone) * level,
                (c11 * frame[1] + tone) * level,
            ]
        }
    }

    pub fn clear(&mut self) {
        self.line.clear();
        self.input = 0.0;
        self.first_out = 0.0;
        self.second_out = 0.0;
        self.peak = 0.0;
        self.env = 0.0;
        self.post = 0.0;
        if let Some(settings) = self.pending.take() {
            self.jump(&settings);
        }
        self.duck.reset();
        self.active = None;
    }

    /// Current tap in 44.1 kHz samples (tests).
    #[cfg(test)]
    pub fn tap(&self) -> f32 {
        BASE[self.group] + self.env * self.c8.current * 32_768.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(core: &mut CryCore, amplitude: f32, frames: usize) {
        for i in 0..frames {
            let x = amplitude * (i as f32 * 0.07).sin();
            core.process([x, x]);
        }
    }

    #[test]
    fn envelope_moves_the_tap_out_or_in() {
        // Range 10 (value 9), Sense -10: down group, the tap moves from 35 out.
        let mut values = [9.0, 8.0, 0.0, 100.0, 100.0, 0.0, 0.0, 0.0, 0.0];
        let mut down = CryCore::new(48_000.0, false);
        down.set(&values);
        assert_eq!(down.tap(), 35.0);
        run(&mut down, 0.8, 48_000);
        assert!(down.tap() > 80.0, "{}", down.tap());
        // Sense +10: up group, the tap starts at 88 and moves in.
        values[2] = 19.0;
        let mut up = CryCore::new(48_000.0, false);
        up.set(&values);
        assert_eq!(up.tap(), 88.0);
        run(&mut up, 0.8, 48_000);
        assert!(up.tap() < 50.0, "{}", up.tap());
    }

    #[test]
    fn comb_resonates_at_the_loop_period() {
        // Silence keeps the tap at 35 samples: an impulse comes back every
        // 36 samples at 44.1 kHz and every 36 * 96 / 44.1 samples at 96 kHz.
        for sample_rate in [44_100.0f32, 96_000.0] {
            let values = [0.0, 10.0, 0.0, 100.0, 100.0, 0.0, 0.0, 0.0, 0.0];
            let mut core = CryCore::new(sample_rate, true);
            core.set(&values);
            let mut out = Vec::new();
            for i in 0..2000 {
                let x = if i == 0 { 1e-6 } else { 0.0 };
                out.push(core.process([x, x])[0].abs());
            }
            let period = 36.0 * sample_rate / 44_100.0;
            let first = (period - 1.0).round() as usize;
            let second = (2.0 * period - 1.0).round() as usize;
            let at = |n: usize| out[n - 2..=n + 2].iter().cloned().fold(0.0f32, f32::max);
            let between = out[first + 4..second - 4]
                .iter()
                .cloned()
                .fold(0.0f32, f32::max);
            assert!(at(first) > 1e-7, "{sample_rate}");
            assert!(at(second) > 0.1 * at(first), "{sample_rate}");
            assert!(between < 0.5 * at(second), "{sample_rate}");
        }
    }

    #[test]
    fn balance_curve_has_two_slopes() {
        assert_eq!(curve(1.0, 0.0, 101, 100.0, 2), 1.0);
        assert!((curve(1.0, 0.0, 101, 50.0, 2) - 0.65).abs() < 1e-6);
    }
}
