//! SmoothDly (SMS_DLY.ZDL): a mono delay whose repeats (and the dry input)
//! feed a fixed diffusion/reverb network, after the recovered
//! `Fx_DLY_SmoothDly` callback and the decoded editors.
//!
//! As in the callback: mono input → 8 kHz first-order low-pass → 3 s delay
//! with `F.B` feedback (`fb_tbl`: 0, .16, .5, .81, 1) of the raw tap; the tap
//! passes the fixed ROM EQ biquad and the TONE first-order high shelf
//! (10.99 kHz, `tone_tbl` −21…+17.5 dB, cut moves the corner) outside the
//! loop. Dry input + delayed signal → 50 Hz high-pass → network: four
//! low-passed allpass-like sections (1324/1028/798/620 samples), five damped
//! combs (96.4/86.6/82.6/69.5/61.0 ms, gains `0.01^(d/677 ms)` from the
//! constant `decay_Calc_set`, two-band split at 200 Hz, 12.5 kHz low-pass and
//! 63 Hz high-pass in each loop), ten comb multi-taps summed per side into two
//! output diffusers each, plus an early-reflection pair of multi-tap lines;
//! then two fixed output shelves. Output `E.LVL·0.1995·net + delay·v + dry`,
//! where the delay level is `(E/100)²·x` and x = 0.8, or for TONE < 51 the
//! square curve 0.708…0.8. All network delays are scaled to the host rate;
//! ROM filters are re-derived.
//!
//! Deviation: a TIME change glides through the callback's Q31 smoother (as for
//! tempo changes) instead of the knob path's mute → jump → reopen.
use super::common::{
    Glide, Kind, Stage, design, one_pole_at, rom, safety, samples, table, time_ms, words,
};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{DelayLine, flush},
    zoom::{DEFAULT_STEP, Ramp, curve, is_synced},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_smooth_dly",
    name: "SmoothDly",
    category: Category::Delay,
    zdl: "SMS_DLY.ZDL",
    description: "Delay blended into a smooth diffuse reverb tail",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "TIME",
            max: 3014,
            default: 322,
            display: Display::DelayTime {
                threshold: 2999,
                offset: 1,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 39,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "E.LVL",
            max: 100,
            default: 49,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "TONE",
            max: 100,
            default: 83,
            display: Display::INT,
        },
    ],
};

const MAX_MS: f32 = 3000.0;
const THRESHOLD: u16 = 2999;
const FB_TBL: [f32; 5] = [0.0, 0.16, 0.5, 0.81, 1.0];
const TONE_DB: [f32; 11] = [
    -21.0, -19.0, -16.0, -10.0, -6.0, 0.0, 3.0, 8.0, 12.0, 16.0, 17.5,
];
/// Network delays at 44.1 kHz.
const SECTIONS: [usize; 4] = [1324, 1028, 798, 620];
const COMBS: [usize; 5] = [4251, 3817, 3644, 3066, 2692];
const COMB_MS: [f32; 5] = [96.4, 86.56, 82.64, 69.52, 61.04];
const TAPS_LEFT: [usize; 5] = [1491, 1694, 1194, 871, 440];
const TAPS_RIGHT: [usize; 5] = [626, 557, 462, 324, 193];
const DIFFUSE_LEFT: [usize; 2] = [734, 871];
const DIFFUSE_RIGHT: [usize; 2] = [646, 783];
const EARLY_IN: [(usize, f32); 4] = [
    (88, -0.915_039),
    (900, -0.332_739),
    (2230, -0.120_996),
    (3299, -0.044),
];
const EARLY_GAINS: [f32; 4] = [-0.227_105, 0.206_459, -0.187_695, 0.170_63];
const EARLY_LEFT: [usize; 4] = [542, 847, 1345, 1746];
const EARLY_RIGHT: [usize; 4] = [997, 772, 1138, 1319];
/// Section coefficients C35..C38 (C36, C38 from `decay_Calc_set`).
const A_IN: f32 = 0.75;
const A_FB: f32 = 0.6;
const A_DIRECT: f32 = 0.625;
const A_DELAYED: f32 = -0.5;
/// Decay time of `decay_Calc_set`: curve(7743.75, 347.66, 91, 19, square) ms.
const DECAY_MS: f32 = 677.29;

struct Section {
    line: DelayLine,
    delay: usize,
    lowpass: Option<Stage>,
}

impl Section {
    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let d = self.line.read(self.delay - 1);
        let mut w = A_IN * x + A_FB * d;
        if let Some(lp) = &mut self.lowpass {
            w = lp.process(w);
        }
        self.line.write(flush(w));
        A_DELAYED * d + A_DIRECT * x
    }
}

struct Comb {
    line: DelayLine,
    delay: usize,
    gain: f32,
    split: f32,
    lowband: f32,
    lowpass: Stage,
    highpass: Stage,
}

struct SmoothDly {
    sample_rate: f32,
    input_lp: Stage,
    line: DelayLine,
    time: Glide,
    tap: f32,
    tap_eq: Stage,
    tone: Stage,
    net_hp: Stage,
    previous: f32,
    sections: [Section; 4],
    combs: [Comb; 5],
    taps_left: [usize; 5],
    taps_right: [usize; 5],
    diffuse: [Section; 4],
    early_in: DelayLine,
    early_in_taps: [usize; 4],
    early_out: DelayLine,
    early_left: [usize; 4],
    early_right: [usize; 4],
    out_shelf: [[Stage; 2]; 2],
    feedback: Ramp,
    delay_level: Ramp,
    net_level: Ramp,
    last: [f32; 4],
    bpm: f32,
}

impl SmoothDly {
    fn new(sample_rate: f32) -> Self {
        let sr = sample_rate;
        let n = |d: usize| (samples(d as f32, sr).round() as usize).max(1);
        let lowpass = words(rom([0.551_688, 0.551_688, 0.0, -0.103_376, 0.0], sr));
        let section = |d: usize, lp: bool| Section {
            line: DelayLine::new(n(d) + 2),
            delay: n(d),
            lowpass: lp.then(|| Stage::new(lowpass)),
        };
        // C40 one-pole (0.028 at 44.1 kHz, ~200 Hz) at the host rate.
        let split = 1.0 - one_pole_at(0.028_157, 1.0 - 0.028_157, 44_100.0, sr).1;
        let comb = |k: usize| Comb {
            line: DelayLine::new(n(COMBS[k]) + 2),
            delay: n(COMBS[k]),
            gain: 0.01f32.powf(COMB_MS[k] / DECAY_MS),
            split,
            lowband: 0.0,
            lowpass: Stage::new(lowpass),
            highpass: Stage::from(rom([0.995_532, -0.995_532, 0.0, 0.991_064, 0.0], sr)),
        };
        let shelves = || {
            [
                Stage::from(rom([0.937_38, -0.089_016, 0.0, 0.151_635, 0.0], sr)),
                Stage::from(rom([1.003_637, -0.968_265, 0.0, 0.971_903, 0.0], sr)),
            ]
        };
        let mut effect = Self {
            sample_rate,
            input_lp: Stage::from(rom([0.390_553, 0.390_553, 0.0, 0.218_893, 0.0], sr)),
            line: DelayLine::with_seconds(MAX_MS * 0.001 + 0.01, sr),
            time: Glide::new(0.0, 0x0005_0000, 0x7ffa_ffff, sr),
            tap: 0.0,
            tap_eq: Stage::from(rom(
                [1.005_432, -1.941_126, 0.936_925, 1.941_126, -0.942_358],
                sr,
            )),
            tone: Stage::new(Stage::IDENTITY),
            net_hp: Stage::from(rom([0.996_451, -0.996_451, 0.0, 0.992_901, 0.0], sr)),
            previous: 0.0,
            sections: [
                section(SECTIONS[0], true),
                section(SECTIONS[1], true),
                section(SECTIONS[2], true),
                section(SECTIONS[3], true),
            ],
            combs: [comb(0), comb(1), comb(2), comb(3), comb(4)],
            taps_left: TAPS_LEFT.map(n),
            taps_right: TAPS_RIGHT.map(n),
            diffuse: [
                section(DIFFUSE_LEFT[0], false),
                section(DIFFUSE_LEFT[1], false),
                section(DIFFUSE_RIGHT[0], false),
                section(DIFFUSE_RIGHT[1], false),
            ],
            early_in: DelayLine::new(n(3299) + 2),
            early_in_taps: EARLY_IN.map(|(d, _)| n(d)),
            early_out: DelayLine::new(n(1746) + 2),
            early_left: EARLY_LEFT.map(n),
            early_right: EARLY_RIGHT.map(n),
            out_shelf: [shelves(), shelves()],
            feedback: Ramp::new(0.0, DEFAULT_STEP, sr),
            delay_level: Ramp::new(0.0, DEFAULT_STEP, sr),
            net_level: Ramp::new(0.0, DEFAULT_STEP, sr),
            last: [f32::NAN; 4],
            bpm: 0.0,
        };
        effect.set(&crate::Model::SmoothDly.defaults(), 120.0);
        effect.time.value = effect.time.target;
        for ramp in [
            &mut effect.feedback,
            &mut effect.delay_level,
            &mut effect.net_level,
        ] {
            ramp.jump(ramp.target);
        }
        effect
    }

    /// E.LVL, TONE → (delay level, network level).
    pub fn levels(level: f32, tone: f32) -> (f32, f32) {
        let x = if tone < 51.0 {
            curve(0.8, 0.707_946, 51, tone, 1)
        } else {
            0.8
        };
        (
            curve(x, 0.0, 101, level, 1),
            curve(0.199_526_2, 0.0, 101, level, 0),
        )
    }
}

impl Effect for SmoothDly {
    fn set(&mut self, values: &Values, bpm: f32) {
        if values[..4] == self.last && bpm == self.bpm {
            return;
        }
        let sr = self.sample_rate;
        if values[0] != self.last[0] || (bpm != self.bpm && is_synced(values[0], THRESHOLD)) {
            let ms = time_ms(values[0], THRESHOLD, 1.0, MAX_MS, bpm);
            self.time.target = ms * 0.001 * sr;
        }
        self.feedback.set(table(&FB_TBL, values[1].round(), 101));
        let level = values[2].round().clamp(0.0, 100.0);
        let tone = values[3].round().clamp(0.0, 100.0);
        let (delay, net) = Self::levels(level, tone);
        self.delay_level.set(delay);
        self.net_level.set(net);
        if tone != self.last[3].round() {
            let db = table(&TONE_DB, tone, 101);
            self.tone.c = words(design(Kind::HighShelf1, sr, 10_992.0, db, 0.0));
        }
        self.last.copy_from_slice(&values[..4]);
        self.bpm = bpm;
    }

    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        let mono = 0.5 * (frame[0] + frame[1]);
        let feedback = self.feedback.tick();
        let filtered = self.input_lp.process(mono);
        self.line
            .write(safety(flush(filtered + feedback * self.tap)));
        self.tap = self.line.read_linear(self.time.tick());
        let delayed = self.tone.process(self.tap_eq.process(self.tap));

        // network input, one sample later (written at offset 0, read at 1)
        let x = self.previous;
        self.previous = self.net_hp.process(mono + delayed);

        // early reflections
        self.early_in.write(x);
        let mut early = x;
        for (&d, &(_, g)) in self.early_in_taps.iter().zip(&EARLY_IN) {
            early += g * self.early_in.read(d - 1);
        }
        self.early_out.write(flush(early));
        let mut early_left = early;
        let mut early_right = early;
        for ((g, &l), &r) in EARLY_GAINS
            .iter()
            .zip(&self.early_left)
            .zip(&self.early_right)
        {
            early_left += g * self.early_out.read(l - 1);
            early_right += g * self.early_out.read(r - 1);
        }

        // allpass-like chain into the combs
        let mut y = x;
        for section in &mut self.sections {
            y = section.process(y);
        }
        let mut sum_left = 0.0;
        let mut sum_right = 0.0;
        for (k, comb) in self.combs.iter_mut().enumerate() {
            let tap = comb.line.read(comb.delay - 1);
            let old = comb.lowband;
            comb.lowband = flush(comb.lowband + comb.split * (tap - comb.lowband));
            let w = y + comb.gain * (1.0 - comb.split) * (tap - old) + comb.gain * comb.lowband;
            let w = comb.highpass.process(comb.lowpass.process(w));
            sum_left += comb.line.read(self.taps_left[k] - 1);
            sum_right += comb.line.read(self.taps_right[k] - 1);
            comb.line.write(w);
        }
        let mut late_left = 1.196_782 * sum_left;
        let mut late_right = 1.191_906 * sum_right;
        let [d0, d1, d2, d3] = &mut self.diffuse;
        late_left = d1.process(d0.process(late_left));
        late_right = d3.process(d2.process(late_right));

        let net_level = self.net_level.tick();
        let delay_level = self.delay_level.tick();
        let net = [early_left + late_left, early_right + late_right];
        let mut out = [0.0; 2];
        for ch in 0..2 {
            let [a, b] = &mut self.out_shelf[ch];
            let reverb = b.process(a.process(net[ch]));
            out[ch] = net_level * reverb + delay_level * delayed + frame[ch];
        }
        out
    }

    fn clear(&mut self) {
        self.input_lp.clear();
        self.line.clear();
        self.tap = 0.0;
        self.tap_eq.clear();
        self.tone.clear();
        self.net_hp.clear();
        self.previous = 0.0;
        for s in self.sections.iter_mut().chain(self.diffuse.iter_mut()) {
            s.line.clear();
            if let Some(lp) = &mut s.lowpass {
                lp.clear();
            }
        }
        for c in &mut self.combs {
            c.line.clear();
            c.lowband = 0.0;
            c.lowpass.clear();
            c.highpass.clear();
        }
        self.early_in.clear();
        self.early_out.clear();
        for side in &mut self.out_shelf {
            for s in side {
                s.clear();
            }
        }
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(SmoothDly::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_follow_the_mix_editor() {
        let (d, n) = SmoothDly::levels(100.0, 83.0);
        assert!((d - 0.8).abs() < 1e-6 && (n - 0.199_526).abs() < 1e-5);
        let (d, _) = SmoothDly::levels(100.0, 0.0);
        assert!((d - 0.707_946).abs() < 1e-5);
        assert_eq!(SmoothDly::levels(0.0, 50.0), (0.0, 0.0));
    }

    #[test]
    fn echo_timing_and_tail_decay() {
        let sr = 48_000.0;
        let mut effect = SmoothDly::new(sr);
        let mut values = crate::Model::SmoothDly.defaults();
        values[0] = 199.0; // 200 ms
        values[1] = 0.0;
        values[2] = 100.0;
        effect.set(&values, 120.0);
        effect.time.value = effect.time.target;
        let mut out = Vec::new();
        for i in 0..(sr as usize * 4) {
            let x = if i == 0 { 1.0 } else { 0.0 };
            out.push(effect.process([x, x])[0]);
        }
        // the delayed impulse stands out around 200 ms
        let window = &out[9500..9700];
        assert!(window.iter().any(|x| x.abs() > 0.3));
        // the network tail decays (~0.68 s to −40 dB)
        let late = out[150_000..].iter().fold(0.0f32, |m, x| m.max(x.abs()));
        assert!(late < 1e-3, "{late}");
        let mid = out[12_000..30_000]
            .iter()
            .fold(0.0f32, |m, x| m.max(x.abs()));
        assert!(mid > 1e-3, "{mid}");
    }
}
