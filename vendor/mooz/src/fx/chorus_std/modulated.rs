//! The audio core shared by Chorus, StereoCho, Vibrato, Ba Chorus, ModDelay and
//! ModDelay2: `modulated_delay.rs` (original 7800..7ca0, one callback for all
//! six). Per frame, with coefficient numbers of the original:
//!
//! - mono input `0.5 (L + R) * c5 + c6 * feedback_tap` through two first-order
//!   sections (c7..c9, c10..c12: the Tone/LoCut tables or fixed ROM) into the
//!   ring;
//! - a Q31 phase advanced by c15 gives `|phase|` (a 0..1 triangle, c17 = 1,
//!   c18 = 0 in every ROM), a second one is offset by c16 (180° for
//!   StereoCho); each is smoothed by the one-pole `c20 x + c21 y`;
//! - two linearly interpolated taps at `c22 + c23 * lfo * 2^(31-c24)` samples;
//!   the first one feeds back;
//! - `out = c2 * (in * c28 + 2 * c29 * tap)` per channel (c0, c3 and c4 are
//!   the OnOff crossfade, tap mute and Main's slot gain, all 1 while running;
//!   c26/c27 tap smoothing is 1/0 in every ROM).
use super::common::{FirstOrder, Q31, Slew, TapTime, TimedRamp, increment, triangle};
use crate::{
    dsp::{DelayLine, flush},
    zoom::{self, ORIGINAL_RATE, Ramp},
};

/// Model constants of the shared callback.
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    /// Largest base delay c22 in samples at 44.1 kHz.
    pub max_base: f32,
    /// Modulation span `2^(31 - c24)` in samples at 44.1 kHz.
    pub span: f32,
    /// LFO smoothing c20 / c21 as Q31 words.
    pub smoothing: (u32, u32),
    /// Second tap phase offset c16.
    pub offset: u32,
    /// Base-delay moves larger than this (44.1 kHz samples) crossfade.
    pub jump: f32,
    /// Mix uses the timed setter (context[7]) rather than the stepped one.
    pub timed_wet: bool,
}

#[derive(Debug, Clone, Copy)]
enum Setter {
    Timed(TimedRamp),
    Stepped(Ramp),
}

impl Setter {
    fn set(&mut self, target: f32) {
        match self {
            Self::Timed(ramp) => ramp.set(target),
            Self::Stepped(ramp) => ramp.set(target),
        }
    }
    fn settle(&mut self) {
        match self {
            Self::Timed(ramp) => ramp.settle(),
            Self::Stepped(ramp) => ramp.jump(ramp.target),
        }
    }
    #[inline]
    fn tick(&mut self) -> f32 {
        match self {
            Self::Timed(ramp) => ramp.tick(),
            Self::Stepped(ramp) => ramp.tick(),
        }
    }
}

pub struct Core {
    sample_rate: f32,
    scale: f32,
    span: f32,
    line: DelayLine,
    hp: FirstOrder,
    lp: FirstOrder,
    feedback: Ramp,
    level: Ramp,
    dry: Ramp,
    wet: Setter,
    phase: u32,
    increment: u32,
    offset: u32,
    smooth_in: f32,
    smooth_pole: f32,
    modulation: [f32; 2],
    base: TapTime,
    depth: Slew,
    feedback_tap: f32,
}

impl Core {
    pub fn new(layout: Layout, sample_rate: f32) -> Self {
        let scale = sample_rate / ORIGINAL_RATE;
        let max_delay = ((layout.max_base + layout.span) * scale).ceil() as usize + 8;
        let (smooth_in, smooth_pole) = super::common::one_pole(
            layout.smoothing.0 as f32 * Q31,
            layout.smoothing.1 as f32 * Q31,
            sample_rate,
        );
        let wet = if layout.timed_wet {
            Setter::Timed(TimedRamp::new(0.0, sample_rate))
        } else {
            Setter::Stepped(Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate))
        };
        Self {
            sample_rate,
            scale,
            span: layout.span * scale,
            line: DelayLine::new(max_delay),
            hp: FirstOrder::default(),
            lp: FirstOrder::default(),
            feedback: Ramp::new(0.0, zoom::DEFAULT_STEP, sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            dry: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
            wet,
            phase: 0,
            increment: 0,
            offset: layout.offset,
            smooth_in,
            smooth_pole,
            modulation: [0.0; 2],
            // The integer depth setter moves c23 by 0x3deb7 per sample: at
            // most 0.06 (c24 = 22) samples per sample, also used for c22.
            base: TapTime::new(0.0, 0.06, layout.jump * scale, sample_rate),
            depth: Slew::new(0.0, super::common::DEPTH_STEP * layout.span),
            feedback_tap: 0.0,
        }
    }

    /// c7..c12 from ROM words designed at 44.1 kHz.
    pub fn set_filters(&mut self, hp: [u32; 3], lp: [u32; 3]) {
        self.hp.set_rom(hp, self.sample_rate);
        self.lp.set_rom(lp, self.sample_rate);
    }

    /// c15 as a Q31 increment per 44.1 kHz sample.
    pub fn set_increment_44k(&mut self, increment_44k: f32) {
        self.increment = increment(increment_44k, self.sample_rate);
    }

    pub fn set_hz(&mut self, hz: f32) {
        self.increment = super::common::hz_increment(hz, self.sample_rate);
    }

    /// c22 in samples at 44.1 kHz.
    pub fn set_base(&mut self, samples_44k: f32) {
        self.base.set(samples_44k * self.scale);
    }

    /// c23 as a Q31 integer value.
    pub fn set_depth(&mut self, q31: f32) {
        self.depth.set(q31 * Q31 * self.span);
    }

    /// c2.
    pub fn set_level(&mut self, level: f32) {
        self.level.set(level);
    }

    /// c28.
    pub fn set_dry(&mut self, dry: f32) {
        self.dry.set(dry);
    }

    /// c29 (the wet tap is added twice).
    pub fn set_wet(&mut self, wet: f32) {
        self.wet.set(wet);
    }

    /// c6.
    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback.set(feedback);
    }

    /// Finish every ramp at once (used once after `create`).
    pub fn settle(&mut self) {
        self.feedback.jump(self.feedback.target);
        self.level.jump(self.level.target);
        self.dry.jump(self.dry.target);
        self.wet.settle();
        self.depth.settle();
        self.base.settle();
    }

    #[inline]
    fn read(&self, fade: f32, offset: f32) -> f32 {
        let now = self.line.read_linear(self.base.current + offset);
        if fade > 0.0 {
            now + (self.line.read_linear(self.base.previous + offset) - now) * fade
        } else {
            now
        }
    }

    #[inline]
    pub fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let average = 0.5 * (left + right);
        let input = average + self.feedback.tick() * self.feedback_tap;
        let filtered = self.lp.process(self.hp.process(input));
        self.line.write(filtered);

        self.phase = self.phase.wrapping_add(self.increment);
        let depth = self.depth.tick();
        let fade = self.base.tick();
        let a = triangle(self.phase) * self.smooth_in + self.modulation[0] * self.smooth_pole;
        self.modulation[0] = a;
        let primary = self.read(fade, depth * a);
        let secondary = if self.offset == 0 {
            primary
        } else {
            let b = triangle(self.phase.wrapping_add(self.offset)) * self.smooth_in
                + self.modulation[1] * self.smooth_pole;
            self.modulation[1] = b;
            self.read(fade, depth * b)
        };
        self.feedback_tap = flush(primary);

        let level = self.level.tick();
        let dry = self.dry.tick();
        let wet = 2.0 * self.wet.tick();
        [
            level * (left * dry + wet * primary),
            level * (right * dry + wet * secondary),
        ]
    }

    pub fn clear(&mut self) {
        self.line.clear();
        self.hp.clear();
        self.lp.clear();
        self.modulation = [0.0; 2];
        self.feedback_tap = 0.0;
    }
}

#[cfg(test)]
impl Core {
    /// Base delay and modulation depth targets reached so far, host samples.
    pub fn delays(&self) -> (f32, f32) {
        (self.base.current, self.depth.current)
    }
    pub fn increment(&self) -> u32 {
        self.increment
    }
    pub fn gains(&self) -> (f32, f32, f32, f32) {
        let wet = match self.wet {
            Setter::Timed(ramp) => ramp.current,
            Setter::Stepped(ramp) => ramp.current,
        };
        (
            self.level.current,
            self.dry.current,
            wet,
            self.feedback.current,
        )
    }
}
