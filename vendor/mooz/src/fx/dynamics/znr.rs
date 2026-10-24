//! ZNR (ZNR.ZDL, callback 0x7800..0x7c60) with its complete recovered editors
//! (`znr_parameters.rs`).
//!
//! Per channel, a detector high-pass (c11..c13, ~360 Hz) feeds a peak meter.
//! Every four 8-sample blocks (32 samples at 44.1 kHz, kept in time here) the
//! held peak is updated (`held = peak` when larger, else `held c14`) and turned
//! into a downward-expander gain `min(held c5, 1) * min(held c6, 1)^c7`; the
//! peak restarts at 0. The gain is smoothed per sample (`s = s c10 + g c9`,
//! 2.3 ms) and multiplies the input, then Level c2 (`v / 100`, float setter
//! ramp). c5/c6/c7 come from the ROM threshold table and its power table; the
//! editor's `c7 == 5` cut-off belongs to an index beyond THRSH 24 and never runs.
//!
//! DETCT selects the detector source (c15): EfxIn (1) uses each channel's own
//! input; GtrIn (0) reads the pedal's mono input buffer, approximated here by
//! the slot input's mid signal `(L + R) / 2` driving both channels.
use super::common::{self, FirstOrder, one_pole};
use crate::dsp::flush;
use crate::zoom::{ORIGINAL_RATE, Ramp};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_znr",
    name: "ZNR",
    category: Category::Dynamics,
    zdl: "ZNR.ZDL",
    description: "Zoom noise reduction: smooth downward expander",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "THRSH",
            max: 24,
            default: 9,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "DETCT",
            max: 1,
            default: 1,
            display: Display::labels(&["GtrIn", "EfxIn"]),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

/// THRSH → c5, c6 (ZNR_Thr_Tbl, ZNR 0x80000438).
const THRESHOLD: [[u32; 2]; 25] = [
    [0x4545a47b, 0x4644b500],
    [0x4543ad48, 0x4642c048],
    [0x453de5c3, 0x463cffae],
    [0x4534a429, 0x4633c966],
    [0x45286fae, 0x4627a3a4],
    [0x4519f28f, 0x4619381f],
    [0x4509ebae, 0x460944ae],
    [0x44f23c29, 0x45f116a4],
    [0x44d08333, 0x45cf86a4],
    [0x44afeeb8, 0x45af19c3],
    [0x4491819a, 0x4590d15c],
    [0x446beb85, 0x456acdec],
    [0x443b78f6, 0x453a9614],
    [0x4412070a, 0x45115614],
    [0x43defc29, 0x44ddee14],
    [0x43a6e148, 0x44a6175c],
    [0x4374d70a, 0x4473af5c],
    [0x43300f5c, 0x442f3a3d],
    [0x42f82e14, 0x43f703d7],
    [0x42ab7ae1, 0x43aaa8f6],
    [0x42683d71, 0x43672666],
    [0x421a28f6, 0x431970a4],
    [0x41c8a3d7, 0x42c7ae14],
    [0x41800000, 0x427eb852],
    [0x41200000, 0x421f3d71],
];

/// THRSH → c7 (ZNR_GtNumTbl, ZNR 0x80000500).
const POWER: [u8; 25] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 4,
];

/// ROM c9/c10 (gain smoother), c11..c13 (detector high-pass), c14 (hold decay).
const C9: u32 = 0x3c23d70a;
const C10: u32 = 0x3f7d70a4;
const DETECTOR: [u32; 3] = [0x3f7998f6, 0xbf7998f6, 0x3f7331ec];
const C14: u32 = 0x3f7ab577;
/// Four blocks of eight samples between gain updates.
const PERIOD_AT_44K: f32 = 32.0;

struct Znr {
    upper: f32,
    lower: f32,
    power: u8,
    hold: f32,
    smoothing: f32,
    keep: f32,
    effect_detector: bool,
    level: Ramp,
    period: u32,
    counter: u32,
    threshold_index: usize,
    primed: bool,
    detector: [FirstOrder; 2],
    peak: [f32; 2],
    held: [f32; 2],
    gain: [f32; 2],
    smoothed: [f32; 2],
}

impl Znr {
    fn set_threshold(&mut self, index: usize) {
        if index != self.threshold_index {
            self.threshold_index = index;
            self.upper = f32::from_bits(THRESHOLD[index][0]);
            self.lower = f32::from_bits(THRESHOLD[index][1]);
            self.power = POWER[index];
        }
    }

    #[inline]
    fn expander_gain(&self, held: f32) -> f32 {
        let mut gain = (held * self.upper).min(1.0);
        let factor = (self.lower * held).min(1.0);
        for _ in 0..self.power {
            gain *= factor;
        }
        gain
    }
}

impl Effect for Znr {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.set_threshold(common::index(values[0], 24));
        self.effect_detector = common::index(values[1], 1) == 1;
        self.level.set(common::level(values[2]));
        if !self.primed {
            self.primed = true;
            common::settle(&mut self.level);
        }
    }

    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        if self.counter == 0 {
            for channel in 0..2 {
                let held = if self.held[channel] < self.peak[channel] {
                    self.peak[channel]
                } else {
                    flush(self.held[channel] * self.hold)
                };
                self.held[channel] = held;
                self.gain[channel] = self.expander_gain(held);
                self.peak[channel] = 0.0;
            }
        }
        self.counter += 1;
        if self.counter == self.period {
            self.counter = 0;
        }
        let selected = if self.effect_detector {
            [left, right]
        } else {
            let mid = 0.5 * (left + right);
            [mid, mid]
        };
        let input = [left, right];
        let level = self.level.tick();
        let mut output = [0.0; 2];
        for channel in 0..2 {
            let filtered = self.detector[channel].process(selected[channel]).abs();
            if self.peak[channel] < filtered {
                self.peak[channel] = filtered;
            }
            self.smoothed[channel] =
                flush(self.smoothed[channel] * self.keep + self.gain[channel] * self.smoothing);
            output[channel] = input[channel] * self.smoothed[channel] * level;
        }
        output
    }

    fn clear(&mut self) {
        for detector in &mut self.detector {
            detector.clear();
        }
        self.peak = [0.0; 2];
        self.held = [0.0; 2];
        self.gain = [0.0; 2];
        self.smoothed = [0.0; 2];
        self.counter = 0;
        common::settle(&mut self.level);
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(build(sample_rate))
}

fn build(sample_rate: f32) -> Znr {
    let (smoothing, keep) = one_pole(f32::from_bits(C9), f32::from_bits(C10), sample_rate);
    let detector = FirstOrder::from_words(DETECTOR).resample(sample_rate);
    let mut znr = Znr {
        upper: 0.0,
        lower: 0.0,
        power: 1,
        // c14 decays once per update period, which is kept in time.
        hold: f32::from_bits(C14),
        smoothing,
        keep,
        effect_detector: true,
        level: common::level_ramp(100.0, sample_rate),
        period: (PERIOD_AT_44K * sample_rate / ORIGINAL_RATE)
            .round()
            .max(1.0) as u32,
        counter: 0,
        threshold_index: usize::MAX,
        primed: false,
        detector: [detector; 2],
        peak: [0.0; 2],
        held: [0.0; 2],
        gain: [0.0; 2],
        smoothed: [0.0; 2],
    };
    znr.set_threshold(9);
    znr
}

#[cfg(test)]
mod tests {
    use super::*;

    fn steady(znr: &mut Znr, amplitude: f32, frames: usize) -> f32 {
        let mut peak = 0.0f32;
        for i in 0..frames {
            let x = amplitude * (i as f32 * 0.1).sin();
            let [l, _] = znr.process([x, x]);
            if i > frames / 2 {
                peak = peak.max(l.abs());
            }
        }
        peak
    }

    #[test]
    fn passes_loud_signals_and_expands_quiet_ones() {
        let mut znr = build(48_000.0);
        let mut values = [0.0; crate::MAX_PARAMS];
        values[0] = 9.0;
        values[1] = 1.0;
        values[2] = 100.0;
        znr.set(&values, 120.0);
        let loud = steady(&mut znr, 0.2, 48_000);
        assert!((loud - 0.2).abs() < 0.005, "{loud}");
        // 1/c5 at THRSH 10 (index 9) is about -63 dB; 20 dB lower is expanded hard.
        let quiet_in = 0.07 / znr.upper;
        let quiet = steady(&mut znr, quiet_in, 96_000);
        assert!(quiet < quiet_in * 0.05, "{quiet} vs {quiet_in}");
    }

    #[test]
    fn update_period_is_kept_in_time() {
        assert_eq!(build(44_100.0).period, 32);
        assert_eq!(build(96_000.0).period, 70);
        let at44 = build(44_100.0).detector[0];
        let at96 = build(96_000.0).detector[0];
        assert!((at44.magnitude(44_100.0, 360.0) - at96.magnitude(96_000.0, 360.0)).abs() < 0.01);
    }

    #[test]
    fn expander_slope_follows_the_power_table() {
        let mut znr = build(44_100.0);
        znr.set_threshold(23);
        assert_eq!(znr.power, 3);
        let held = 0.5 / znr.lower;
        let gain = znr.expander_gain(held);
        let expected = (held * znr.upper).min(1.0) * 0.125;
        assert!((gain - expected).abs() < 1e-6);
    }
}
