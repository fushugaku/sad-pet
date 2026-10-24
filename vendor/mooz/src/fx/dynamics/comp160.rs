//! 160 Comp (160_COMP.ZDL, callback 0..0x720) with its complete recovered
//! parameter lifecycle (`comp160_parameters.rs`) and ROM image (0x800003c8).
//!
//! A dbx-160-style feed-forward compressor on `mono = (L + R) / 2`. The
//! detector runs on the previous sample's peak `|x| c27`: an instant-attack
//! hold `s1` decaying by c5 (23 ms), then the envelope `s2` that releases by
//! c7 (230 ms) while `s1 >= c6` (-46 dBFS) and by c8 (45 ms) below it.
//! `db = 20 log10(s2)`; above the threshold c9 the gain is
//! `(c10 - 1)(db - c9)` dB (c10 = 1 / ratio); with Knee on, inside
//! `c13 < db < c14` it is `((db - c13) / c15)² c16`, a 60 dB wide quadratic
//! knee that meets the straight line at `c9 + 30`. `10^(dB / 20)` is smoothed
//! (`g = c28 gain + c29 g`, 2.3 ms) and the wet `c30 x g c2` replaces both
//! channels. The two biquads c17..c26 are identities in the ROM and no editor
//! writes them, so they are omitted.
//!
//! Editors: THRSH → c9 = `v - 60` dB, Ratio → c10 = `10 / (10 + v)`
//! (1.0:1 to 10.0:1), Gain → c30 = `10^(v / 20)` (all three 16 ms glides);
//! Knee → c13..c16 recomputed from the threshold and ratio (immediately for a
//! Knee change; after a THRSH/Ratio change the original editor resumes and
//! recomputes the knee only once the 16 ms glide has finished, and so does this
//! port); Level → c2 = `v / 100`. log10 and pow use fast approximations
//! (<0.001 dB).
use super::common::{self, Glide, decay, fast_exp2, fast_log2, one_pole};
use crate::dsp::flush;
use crate::zoom::Ramp;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_160_comp",
    name: "160 Comp",
    category: Category::Dynamics,
    zdl: "160_COMP.ZDL",
    description: "dbx 160-style compressor with optional soft knee",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "THRSH",
            max: 60,
            default: 38,
            display: Display::number(-60.0, 1.0, 0, " dB"),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Ratio",
            max: 90,
            default: 30,
            display: Display::number(1.0, 0.1, 1, ""),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Gain",
            max: 20,
            default: 6,
            display: Display::number(0.0, 1.0, 0, " dB"),
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Knee",
            max: 1,
            default: 0,
            display: Display::labels(&["Hard", "Soft"]),
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// ROM c5 (hold decay), c6 (release switch level), c7/c8 (releases),
/// c27 (detector scale), c28/c29 (gain smoother).
const HOLD: u32 = 0x3f7fbfc6;
const RELEASE_SWITCH: u32 = 0x3ba3d70a;
const RELEASE_SLOW: u32 = 0x3f7ff972;
const RELEASE_FAST: u32 = 0x3f7fdf3b;
const DETECTOR_SCALE: f32 = 1.0;
const SMOOTHING: [u32; 2] = [0x3c23d70a, 0x3f7d70a4];
/// Knee editor constants: half width 30 dB, width 60 dB.
const KNEE_HALF: f32 = 30.0;
const KNEE_WIDTH: f32 = 60.0;
/// 20 log10(x) = DB_PER_OCTAVE log2(x); 10^(dB / 20) = 2^(dB OCTAVES_PER_DB).
const DB_PER_OCTAVE: f32 = 6.020_6;
const OCTAVES_PER_DB: f32 = 0.166_096_4;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Knee {
    low: f32,
    high: f32,
    width: f32,
    curve: f32,
}

impl Knee {
    /// `knee_edit` from the threshold and inverse ratio it reads (c9, c10).
    fn new(enabled: bool, threshold: f32, inverse_ratio: f32) -> Self {
        if !enabled {
            return Self {
                low: 0.0,
                high: 0.0,
                width: 0.0,
                curve: 0.0,
            };
        }
        let high = KNEE_HALF + threshold;
        let delta = high - threshold;
        Self {
            low: threshold - KNEE_HALF,
            high,
            width: KNEE_WIDTH,
            curve: delta * inverse_ratio - delta,
        }
    }
}

/// The original's gain computer, in dB.
#[inline]
fn gain_db(db: f32, threshold: f32, inverse_ratio: f32, knee: &Knee) -> f32 {
    let difference = db - threshold;
    let mut gain = if threshold < db {
        inverse_ratio * difference - difference
    } else {
        0.0
    };
    if knee.width != 0.0 && knee.low < db && db < knee.high {
        let q = (db - knee.low) / knee.width;
        gain = q * q * knee.curve;
    }
    gain
}

struct Comp160 {
    hold: f32,
    switch: f32,
    slow: f32,
    fast: f32,
    smoothing: f32,
    keep: f32,
    threshold: Glide,
    ratio: Glide,
    makeup: Glide,
    level: Ramp,
    knee: Knee,
    /// Knee switch waiting for the THRSH/Ratio glides to finish.
    pending_knee: Option<bool>,
    cache: [usize; 4],
    primed: bool,
    peak: f32,
    attack: f32,
    envelope: f32,
    gain: f32,
}

impl Comp160 {
    fn threshold_db(index: usize) -> f32 {
        index as f32 - 60.0
    }

    fn inverse_ratio(index: usize) -> f32 {
        1.0 / ((10.0 + index as f32) / 10.0)
    }
}

impl Effect for Comp160 {
    fn set(&mut self, values: &Values, _bpm: f32) {
        let threshold = common::index(values[0], 60);
        let ratio = common::index(values[1], 90);
        let gain = common::index(values[2], 20);
        let knee = common::index(values[3], 1) == 1;
        let gliding = threshold != self.cache[0] || ratio != self.cache[1];
        if gliding {
            self.threshold.set(Self::threshold_db(threshold));
            self.ratio.set(Self::inverse_ratio(ratio));
        }
        if gain != self.cache[2] {
            self.makeup.set(10f32.powf(gain as f32 / 20.0));
        }
        if gliding && self.primed || self.pending_knee.is_some() {
            self.pending_knee = Some(knee);
        } else if usize::from(knee) != self.cache[3] {
            self.knee = Knee::new(knee, self.threshold.target, self.ratio.target);
        }
        self.level.set(common::level(values[4]));
        self.cache = [threshold, ratio, gain, usize::from(knee)];
        if !self.primed {
            self.primed = true;
            self.threshold.snap();
            self.ratio.snap();
            self.makeup.snap();
            common::settle(&mut self.level);
            self.knee = Knee::new(knee, self.threshold.target, self.ratio.target);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        self.attack = if self.attack < self.peak {
            self.peak
        } else {
            flush(self.attack * self.hold)
        };
        let release = if self.attack < self.switch {
            self.fast
        } else {
            self.slow
        };
        self.envelope = if self.envelope < self.attack {
            self.attack
        } else {
            flush(release * self.envelope)
        };
        let db = DB_PER_OCTAVE * fast_log2(self.envelope);
        let threshold = self.threshold.tick();
        let ratio = self.ratio.tick();
        if let Some(on) = self.pending_knee
            && threshold == self.threshold.target
            && ratio == self.ratio.target
        {
            self.knee = Knee::new(on, threshold, ratio);
            self.pending_knee = None;
        }
        let reduction = gain_db(db, threshold, ratio, &self.knee);
        let target = fast_exp2(reduction * OCTAVES_PER_DB);
        let mono = 0.5 * (left + right);
        self.peak = mono.abs() * DETECTOR_SCALE;
        self.gain = flush(self.smoothing * target + self.gain * self.keep);
        let wet = self.makeup.tick() * mono * self.gain * self.level.tick();
        [wet, wet]
    }

    fn clear(&mut self) {
        self.peak = 0.0;
        self.attack = 0.0;
        self.envelope = 0.0;
        self.gain = 0.0;
        self.threshold.snap();
        self.ratio.snap();
        self.makeup.snap();
        common::settle(&mut self.level);
        if let Some(on) = self.pending_knee.take() {
            self.knee = Knee::new(on, self.threshold.target, self.ratio.target);
        }
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(build(sample_rate))
}

fn build(sample_rate: f32) -> Comp160 {
    let (smoothing, keep) = one_pole(
        f32::from_bits(SMOOTHING[0]),
        f32::from_bits(SMOOTHING[1]),
        sample_rate,
    );
    Comp160 {
        hold: decay(f32::from_bits(HOLD), sample_rate),
        switch: f32::from_bits(RELEASE_SWITCH),
        slow: decay(f32::from_bits(RELEASE_SLOW), sample_rate),
        fast: decay(f32::from_bits(RELEASE_FAST), sample_rate),
        smoothing,
        keep,
        threshold: Glide::new(Comp160::threshold_db(38), sample_rate),
        ratio: Glide::new(Comp160::inverse_ratio(30), sample_rate),
        makeup: Glide::new(1.0, sample_rate),
        level: common::level_ramp(100.0, sample_rate),
        knee: Knee::new(false, -22.0, 0.25),
        pending_knee: None,
        cache: [usize::MAX; 4],
        primed: false,
        peak: 0.0,
        attack: 0.0,
        envelope: 0.0,
        gain: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db(x: f32) -> f32 {
        20.0 * x.log10()
    }

    fn steady(comp: &mut Comp160, amplitude: f32) -> f32 {
        let mut peak = 0.0f32;
        for i in 0..96_000 {
            let x = amplitude * (i as f32 * 0.03).sin();
            let [l, r] = comp.process([x, x]);
            assert_eq!(l, r);
            if i > 72_000 {
                peak = peak.max(l.abs());
            }
        }
        peak
    }

    fn values(threshold: f32, ratio: f32, gain: f32, knee: f32) -> Values {
        [threshold, ratio, gain, knee, 100.0, 0.0, 0.0, 0.0, 0.0]
    }

    #[test]
    fn editors_follow_the_recovered_lifecycle() {
        assert_eq!(Comp160::threshold_db(0), -60.0);
        assert_eq!(Comp160::threshold_db(60), 0.0);
        assert!((Comp160::inverse_ratio(90) - 0.1).abs() < 1e-7);
        assert_eq!(Comp160::inverse_ratio(0), 1.0);
        let knee = Knee::new(true, -40.0, 0.1);
        assert_eq!(knee.low, -70.0);
        assert_eq!(knee.high, -10.0);
        assert!((knee.curve - (3.0 - 30.0)).abs() < 1e-5);
        // The knee meets the hard curve at its upper edge.
        let edge = gain_db(-10.0 - 1e-3, -40.0, 0.1, &knee);
        assert!((edge - gain_db(-10.0, -40.0, 0.1, &Knee::new(false, 0.0, 0.0))).abs() < 0.01);
    }

    #[test]
    fn static_curve_has_the_selected_ratio_and_makeup() {
        let mut comp = build(48_000.0);
        comp.set(&values(30.0, 30.0, 6.0, 0.0), 120.0); // -30 dB, 4:1, +6 dB
        let quiet = steady(&mut comp, 0.01);
        assert!((db(quiet / 0.01) - 6.0).abs() < 0.1, "{quiet}");
        let a = steady(&mut comp, 0.1);
        let b = steady(&mut comp, 0.8);
        let slope = db(b / a) / db(8.0);
        assert!((slope - 0.25).abs() < 0.03, "{slope}");
    }

    #[test]
    fn knee_follows_the_threshold_after_its_glide() {
        let mut comp = build(48_000.0);
        comp.set(&values(30.0, 30.0, 0.0, 1.0), 120.0);
        assert_eq!(comp.knee.low, -60.0);
        comp.set(&values(40.0, 30.0, 0.0, 1.0), 120.0);
        assert_eq!(comp.knee.low, -60.0);
        for _ in 0..800 {
            comp.process([0.0; 2]);
        }
        assert_eq!(comp.pending_knee, None);
        assert_eq!(comp.knee.low, -50.0);
    }

    #[test]
    fn soft_knee_starts_compressing_below_the_threshold() {
        let mut hard = build(44_100.0);
        hard.set(&values(30.0, 90.0, 0.0, 0.0), 120.0);
        let mut soft = build(44_100.0);
        soft.set(&values(30.0, 90.0, 0.0, 1.0), 120.0);
        // -40 dBFS: below the -30 dB threshold, inside the soft knee.
        assert!((steady(&mut hard, 0.01) - 0.01).abs() < 1e-4);
        assert!(steady(&mut soft, 0.01) < 0.009);
    }
}
