//! The shared PTCHSHFT/DETUNE audio callback (recovered `pitch_shift.rs`,
//! original 7800..7ba0), also the core of B_DETUNE (`bass_detune.rs`, same
//! graph plus a LoCut stage the model adds).
//!
//! Mono (L+R)/2 → one-pole Tone (c19/c20) → ring. A Q32 phase advances by c5
//! per sample; two taps half a period apart read the ring at
//! `c9 + c8 * sweep(phase)` with linear interpolation and are weighted by the
//! trapezoid window (c6/c7). The tap sum is multiplied by the mute word c3
//! twice (c3²) and crossfaded by the clamped ramp c13/c14 against the filtered
//! input. The model mixes the result with its dry gains.
use super::common::{FADE_SECONDS, TimedRamp, samples, sweep, trapezoid};
use crate::dsp::{DelayLine, flush};

/// What the editors set: the phase increment (c5), the base distance (c9)
/// and sweep depth (c8) in samples at the instance rate, and whether the
/// shifted taps are heard at all (the ramp, c25 = 1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geometry {
    pub increment: u32,
    pub base: f32,
    pub depth: f32,
    pub shifted: bool,
}

impl Geometry {
    /// Taps for pitch `ratio` sweeping `depth_44k` samples (44.1 kHz units)
    /// above `base_44k`: the delay changes by `1 - ratio` samples per sample,
    /// as the ROM increment tables do (`ratio = 1 - depth * c5 / 2^32`).
    pub fn new(ratio: f32, base_44k: f32, depth_44k: f32, sample_rate: f32, shifted: bool) -> Self {
        let depth = samples(depth_44k, sample_rate);
        let turns = (1.0 - ratio as f64) / depth as f64;
        Self {
            increment: (turns * 4_294_967_296.0).round() as i64 as i32 as u32,
            base: samples(base_44k, sample_rate).max(1.0),
            depth,
            shifted,
        }
    }
}

pub struct TwoTapShifter {
    line: DelayLine,
    tone: f32,
    filtered: f32,
    phase: u32,
    active: Geometry,
    pending: Option<Geometry>,
    mute: TimedRamp,
    blend: TimedRamp,
    started: bool,
}

impl TwoTapShifter {
    /// `longest_44k` is the largest `base + depth + 1` any setting reaches.
    pub fn new(sample_rate: f32, longest_44k: f32) -> Self {
        let geometry = Geometry {
            increment: 0,
            base: 2.0,
            depth: 0.0,
            shifted: true,
        };
        Self {
            line: DelayLine::new(samples(longest_44k, sample_rate).ceil() as usize + 4),
            tone: 1.0,
            filtered: 0.0,
            phase: 0,
            active: geometry,
            pending: None,
            mute: TimedRamp::new(1.0, FADE_SECONDS, sample_rate),
            blend: TimedRamp::new(1.0, FADE_SECONDS, sample_rate),
            started: false,
        }
    }

    pub fn set_tone(&mut self, gain: f32) {
        self.tone = gain;
    }

    /// Apply new taps. `fade` reproduces the editors that mute c3 over 16 ms,
    /// change the coefficients and fade c3 back in; otherwise (Detune's Cent)
    /// the increment changes in place. The first call after creation applies
    /// immediately, like the init.
    pub fn request(&mut self, geometry: Geometry, fade: bool) {
        if !self.started {
            self.started = true;
            self.apply(geometry);
            self.blend.jump(if geometry.shifted { 1.0 } else { 0.0 });
            return;
        }
        if self.pending.is_none() && geometry == self.active {
            return;
        }
        if fade {
            self.pending = Some(geometry);
            self.mute.set(0.0);
        } else if self.pending.is_some() {
            self.pending = Some(geometry);
        } else {
            self.apply(geometry);
        }
    }

    fn apply(&mut self, geometry: Geometry) {
        self.active = geometry;
        self.blend.set(if geometry.shifted { 1.0 } else { 0.0 });
    }

    /// One mono sample in; the ramped wet signal out (before the wet gain).
    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        self.filtered = flush(self.filtered + self.tone * (input - self.filtered));
        self.line.write(self.filtered);
        if let Some(next) = self.pending
            && self.mute.current <= 0.0
        {
            self.pending = None;
            self.apply(next);
            self.mute.set(1.0);
        }
        let g = self.active;
        self.phase = self.phase.wrapping_add(g.increment);
        let other = self.phase.wrapping_add(0x8000_0000);
        let a = self.line.read_linear(g.base + g.depth * sweep(self.phase));
        let b = self.line.read_linear(g.base + g.depth * sweep(other));
        let taps = trapezoid(self.phase) * a + trapezoid(other) * b;
        let mute = self.mute.tick();
        let blend = self.blend.tick();
        self.filtered * (1.0 - blend) + taps * (mute * mute * blend)
    }

    pub fn clear(&mut self) {
        self.line.clear();
        self.filtered = 0.0;
        self.phase = 0;
        if let Some(next) = self.pending.take() {
            self.apply(next);
        }
        self.mute.jump(1.0);
        self.blend.jump(self.blend.target());
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Power-weighted mean frequency of `signal` between `low` and `high`
    /// (1 Hz DFT scan). Splice sidebands of a two-tap shifter sit around the
    /// shifted line, so the centroid of a band around it lands on the line.
    pub(crate) fn centroid(signal: &[f32], sample_rate: f32, low: f32, high: f32) -> f32 {
        let (mut weighted, mut total) = (0.0f64, 0.0f64);
        let mut hz = low;
        while hz <= high {
            let w = std::f64::consts::TAU * hz as f64 / sample_rate as f64;
            let (mut re, mut im) = (0.0f64, 0.0f64);
            for (n, &x) in signal.iter().enumerate() {
                let p = w * n as f64;
                re += x as f64 * p.cos();
                im += x as f64 * p.sin();
            }
            let power = re * re + im * im;
            weighted += power * hz as f64;
            total += power;
            hz += 0.25;
        }
        (weighted / total.max(1e-30)) as f32
    }

    #[test]
    fn shifts_a_sine_by_the_ratio() {
        for sample_rate in [44_100.0f32, 96_000.0] {
            let mut shifter = TwoTapShifter::new(sample_rate, 6_000.0);
            shifter.request(Geometry::new(1.5, 2.0, 3_612.7, sample_rate, true), false);
            let n = sample_rate as usize;
            let mut out = Vec::with_capacity(n);
            for i in 0..n {
                let x = (i as f32 * 220.0 / sample_rate * std::f32::consts::TAU).sin();
                out.push(shifter.process(x));
            }
            let found = centroid(&out[n / 2..], sample_rate, 290.0, 370.0);
            assert!((found - 330.0).abs() < 2.0, "{sample_rate}: {found}");
        }
    }

    #[test]
    fn fades_around_a_geometry_change() {
        let mut shifter = TwoTapShifter::new(44_100.0, 6_000.0);
        shifter.request(Geometry::new(0.5, 2.0, 2_890.0, 44_100.0, true), true);
        for _ in 0..100 {
            shifter.process(0.5);
        }
        shifter.request(Geometry::new(2.0, 2.0, 4_335.0, 44_100.0, true), true);
        let mut lowest = f32::MAX;
        for _ in 0..2_000 {
            shifter.process(0.5);
            lowest = lowest.min(shifter.mute.current);
        }
        assert_eq!(lowest, 0.0);
        assert_eq!(shifter.mute.current, 1.0);
        assert!(shifter.pending.is_none());
    }
}
