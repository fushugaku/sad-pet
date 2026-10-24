//! The audio callback shared by SuperCho (SUPERCHO.ZDL 0..0x720), CE-Cho5
//! (CE_CHO5.ZDL, the same equations) and ANA234Cho (ANA234CH.ZDL
//! 0xb00..0x12c0, the same graph plus a stereo input filter):
//!
//! ```text
//! L,R -> [input filters c52..c56, ANA234 only] -> mono = (L+R)/2
//! mono -> [c15 biquad] -> + clip(tap * c12) -> [c20 biquad] -> ring
//! ring -> linear tap at the LFO delay -> tone A -> tone B -> tone C -> wet * c6
//! Mode 0: L = R = mono + wet          Mode 1 (stereo): L = wet, R = mono
//! ```
//!
//! The input, write and feedback biquads are identities in every ROM and no
//! editor touches them, and the feedback gain c12 is 0, so the port leaves
//! them out. The dry path is the mono sum, as in the original (c0 = 1 when the
//! effect is on, so the `1 - c0` stereo dry term is silent).
use super::common::{BbdLfo, Filter};
use crate::dsp::{Coefficients, DelayLine, Smoother};
use crate::zoom::{DEFAULT_STEP, Ramp};

#[derive(Debug, Clone)]
pub struct Bbd {
    line: DelayLine,
    pub lfo: BbdLfo,
    input: Option<[Filter; 2]>,
    pub tone: [Filter; 3],
    wet: Ramp,
    route: Smoother,
}

impl Bbd {
    /// `linear`/`square`: the LFO polynomial words. `input`: the ANA234
    /// per-channel input filter.
    pub fn new(
        sample_rate: f32,
        max_ms: f32,
        linear: f32,
        square: f32,
        input: Option<Coefficients>,
    ) -> Self {
        let input = input.map(|c| {
            let mut f = Filter::default();
            f.set(c);
            [f, f]
        });
        Self {
            line: DelayLine::with_seconds(max_ms * 0.001, sample_rate),
            lfo: BbdLfo::new(sample_rate, linear, square),
            input,
            tone: [Filter::default(); 3],
            wet: Ramp::new(0.0, DEFAULT_STEP, sample_rate),
            // Mode changes are muted for 705.6 samples by the editors.
            route: Smoother::new(0.0, 0.004, sample_rate),
        }
    }

    pub fn set_wet(&mut self, gain: f32, jump: bool) {
        if jump {
            self.wet.jump(gain);
        } else {
            self.wet.set(gain);
        }
    }

    pub fn set_stereo(&mut self, stereo: bool, jump: bool) {
        let target = if stereo { 1.0 } else { 0.0 };
        if jump {
            self.route.jump(target);
        } else {
            self.route.set(target);
        }
    }

    #[inline]
    pub fn process(&mut self, [l, r]: [f32; 2]) -> [f32; 2] {
        let (l, r) = match &mut self.input {
            Some([fl, fr]) => (fl.process(l), fr.process(r)),
            None => (l, r),
        };
        let mono = (l + r) * 0.5;
        self.line.write(mono);
        let delay = self.lfo.tick();
        let tap = self.line.read_linear(delay);
        let [a, b, c] = &mut self.tone;
        let wet = c.process(b.process(a.process(tap))) * self.wet.tick();
        let stereo = self.route.tick();
        let mixed = mono + wet;
        [mixed - mono * stereo, mixed - wet * stereo]
    }

    pub fn clear(&mut self) {
        self.line.clear();
        self.lfo.clear();
        if let Some(filters) = &mut self.input {
            filters.iter_mut().for_each(Filter::clear);
        }
        self.tone.iter_mut().for_each(Filter::clear);
    }
}
