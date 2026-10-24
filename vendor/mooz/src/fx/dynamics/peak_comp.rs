//! The peak compressor kernel shared by M_COMP.ZDL and RACKCOMP.ZDL (their
//! 960-byte callbacks are identical) and, with the per-frame ordering of
//! LIMITER.ZDL, by the Limiter.
//!
//! Per sample: `mono = (L + R) / 2`; detector section `f = c14 x + c15 x[-1] +
//! c16 f[-1]`; envelope `e += c6 (|f| - e)` while rising, `e = |f|` when
//! falling; `peak = e` unless `e < peak`, then `peak *= c7`; static gain
//! `(c8 + c9 (peak - c8)) / peak` above the threshold c8 (Main's divide),
//! limited to 1; smoothing `g = c17 gain + c18 g`. The ring buffer delays the
//! mono signal by c19 = 1 sample (a one-sample lookahead), and the wet output
//! `delayed g c11 c2 c12 c3 + c13 mono` replaces both channels (c0 = 1).
//! c3 is the onf "tap mute", 1 once the pedal is on; c12 = 1 and c13 = 0 in
//! every ROM image. Coefficients the editors write through `context[7]` glide
//! over 16 ms like the original; time constants are converted to the host rate.
use super::common::{self, FirstOrder, Glide, decay, follow, one_pole};
use crate::dsp::flush;
use crate::zoom::Ramp;

pub(crate) struct PeakComp {
    sample_rate: f32,
    detector: FirstOrder,
    attack: Glide,
    release: Glide,
    threshold: Glide,
    ratio: Glide,
    makeup: Glide,
    smoothing: f32,
    keep: f32,
    level: Ramp,
    primed: bool,
    envelope: f32,
    peak: f32,
    gain: f32,
    delayed: f32,
}

/// ROM constants of one module (44.1 kHz words).
pub(crate) struct Rom {
    pub detector: [u32; 3],
    pub smoothing: [u32; 2],
}

impl PeakComp {
    pub fn new(rom: &Rom, sample_rate: f32) -> Self {
        let (smoothing, keep) = one_pole(
            f32::from_bits(rom.smoothing[0]),
            f32::from_bits(rom.smoothing[1]),
            sample_rate,
        );
        Self {
            sample_rate,
            detector: FirstOrder::from_words(rom.detector).resample(sample_rate),
            attack: Glide::new(1.0, sample_rate),
            release: Glide::new(0.0, sample_rate),
            threshold: Glide::new(1.0, sample_rate),
            ratio: Glide::new(1.0, sample_rate),
            makeup: Glide::new(1.0, sample_rate),
            smoothing,
            keep,
            level: common::level_ramp(100.0, sample_rate),
            primed: false,
            envelope: 0.0,
            peak: 0.0,
            gain: 1.0,
            delayed: 0.0,
        }
    }

    /// c6, a 44.1 kHz follower coefficient.
    pub fn set_attack(&mut self, coefficient: f32) {
        self.attack.set(follow(coefficient, self.sample_rate));
    }
    /// c7, a 44.1 kHz per-sample decay.
    pub fn set_release(&mut self, factor: f32) {
        self.release.set(decay(factor, self.sample_rate));
    }
    /// c8, linear.
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold.set(threshold);
    }
    /// c9 = 1 / ratio.
    pub fn set_ratio(&mut self, inverse: f32) {
        self.ratio.set(inverse);
    }
    /// The product of the wet-path gains (c11 c12, and c21 for the Limiter).
    pub fn set_makeup(&mut self, gain: f32) {
        self.makeup.set(gain);
    }
    /// Level parameter (`v / 100`, float setter ramp).
    pub fn set_level(&mut self, value: f32) {
        self.level.set(common::level(value));
    }

    /// Call at the end of every `set`: the first one jumps to the values.
    pub fn prime(&mut self) {
        if !self.primed {
            self.primed = true;
            self.settle();
        }
    }

    fn settle(&mut self) {
        for glide in [
            &mut self.attack,
            &mut self.release,
            &mut self.threshold,
            &mut self.ratio,
            &mut self.makeup,
        ] {
            glide.snap();
        }
        common::settle(&mut self.level);
    }

    /// The static gain for a detector peak.
    #[inline]
    pub fn static_gain(peak: f32, threshold: f32, inverse_ratio: f32) -> f32 {
        if threshold < peak {
            ((threshold + inverse_ratio * (peak - threshold)) / peak).min(1.0)
        } else {
            1.0
        }
    }

    #[inline]
    pub fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mono = 0.5 * (left + right);
        let magnitude = self.detector.process(mono).abs();
        let attack = self.attack.tick();
        let release = self.release.tick();
        let difference = magnitude - self.envelope;
        self.envelope = if 0.0 < difference {
            self.envelope + attack * difference
        } else {
            magnitude
        };
        self.peak = if self.envelope < self.peak {
            flush(self.peak * release)
        } else {
            self.envelope
        };
        let gain = Self::static_gain(self.peak, self.threshold.tick(), self.ratio.tick());
        self.gain = flush(gain * self.smoothing + self.gain * self.keep);
        let wet = self.delayed * self.gain * self.makeup.tick() * self.level.tick();
        self.delayed = mono;
        [wet, wet]
    }

    pub fn clear(&mut self) {
        self.detector.clear();
        self.envelope = 0.0;
        self.peak = 0.0;
        self.gain = 1.0;
        self.delayed = 0.0;
        self.settle();
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::Effect;

    /// Output peak for a steady stereo sine of `amplitude` after settling.
    pub fn steady_peak(effect: &mut dyn Effect, amplitude: f32, sample_rate: f32) -> f32 {
        let frames = sample_rate as usize;
        let mut peak = 0.0f32;
        for i in 0..frames {
            let x = amplitude * (i as f32 * std::f32::consts::TAU * 220.0 / sample_rate).sin();
            let [l, r] = effect.process([x, x]);
            assert_eq!(l, r);
            if i > frames / 2 {
                peak = peak.max(l.abs());
            }
        }
        peak
    }

    #[test]
    fn static_gain_follows_threshold_and_ratio() {
        use super::PeakComp;
        assert_eq!(PeakComp::static_gain(0.01, 0.1, 0.25), 1.0);
        let gain = PeakComp::static_gain(0.4, 0.1, 0.25);
        assert!((gain * 0.4 - (0.1 + 0.3 * 0.25)).abs() < 1e-6);
    }
}
