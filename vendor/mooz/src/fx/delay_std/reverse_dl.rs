//! ReverseDL (RVRSDLY.ZDL), from the recovered `Fx_DLY_ReverseDL`
//! (`reverse_delay.rs`), its recovered Time/HiDMP editors
//! (`reverse_delay_parameters.rs`) and the decoded Bal/F.B/Level editors.
//!
//! Callback: a window phase advances by 2 per sample up to the period
//! `2·N` (N = Time in samples) and restarts. One ring holds the half-sum
//! input plus feedback. A fixed tap at age N feeds back through a one-pole
//! low-pass (HiDMP [13], [14] = 1 - [13]) times F.B [9]; the reverse tap reads
//! at age `phase`, i.e. the last window backwards at normal speed. A
//! linear envelope [12] rises over the first 144 samples of each window and
//! falls over the last 144 (step [7] = 1/144) to hide the splice. Output:
//! `Level·(dry·Bal_dry + reverse·Bal_wet)` with the same mono echo on both
//! sides.
//!
//! Editors: Time is `v + 10` ms for 0..=1990 (sync notes above, limited to
//! 2000 ms), period `2·ms·44.1`; a live edit mutes the tap gain [3] in 16 ms
//! and reopens it `2·(ms + 20)` ms later. Bal uses Main's two-slope curve for
//! wet (`Bal/100`) and dry (`(100 - Bal)/100`), F.B `0.95·v/100`, Level
//! `v/100`, HiDMP a table of eleven one-pole coefficients (~250 Hz..~8.6 kHz)
//! re-derived for the host rate. Time shows `v + 10` ms, as the pedal does.
use super::common::{self, Gate};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{DelayLine, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_reverse_dl",
    name: "ReverseDL",
    category: Category::Delay,
    zdl: "RVRSDLY.ZDL",
    description: "Reverse delay: each window plays back reversed",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Time",
            max: 2005,
            default: 990,
            display: Display::DelayTime {
                threshold: TIME_THRESHOLD,
                offset: 10,
            },
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "F.B",
            max: 100,
            default: 20,
            display: Display::INT,
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Bal",
            max: 100,
            default: 50,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "HiDMP",
            max: 10,
            default: 8,
            display: Display::INT,
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

const TIME_THRESHOLD: u16 = 1990;
const MAX_MS: f32 = 2000.0;
/// Envelope edge [6] = 288 phase units = 144 samples at 44.1 kHz.
const EDGE_44K: f32 = 144.0;

/// `HIDAMP_COEFFICIENTS` of the recovered editor (ROM 0x800005b0).
const HIDMP: [u32; 11] = [
    0x3d0fa5b3, 0x3d8d15cc, 0x3e081095, 0x3e7cf1c2, 0x3e98647b, 0x3eb7120f, 0x3eda7922, 0x3efe0d2b,
    0x3f11faba, 0x3f250cc4, 0x3f35422b,
];

pub struct ReverseDl {
    sample_rate: f32,
    line: DelayLine,
    gate: Gate,
    /// Window length N in samples; the phase runs 0..=N (the original's 0..=2N in steps of 2).
    window: usize,
    pending: usize,
    muting: bool,
    reopen: u32,
    phase: usize,
    envelope: f32,
    edge: f32,
    edge_step: f32,
    damping: [f32; 11],
    damp: f32,
    feedback_state: f32,
    feedback: Ramp,
    wet: Ramp,
    dry: Ramp,
    level: Ramp,
    time_ms: f32,
    started: bool,
}

impl ReverseDl {
    pub fn new(sample_rate: f32) -> Self {
        let edge = EDGE_44K * common::rate_ratio(sample_rate);
        let damping = HIDMP.map(|w| common::smoothing_at(common::f(w), sample_rate));
        Self {
            sample_rate,
            // The reverse tap reads up to twice the window.
            line: common::line_for_ms(2.0 * MAX_MS, 2, sample_rate),
            gate: Gate::new(sample_rate, true),
            window: 1,
            pending: 1,
            muting: false,
            reopen: 0,
            phase: 0,
            envelope: 1.0,
            edge,
            edge_step: 1.0 / edge,
            damping,
            damp: damping[8],
            feedback_state: 0.0,
            feedback: common::setter(0.0, sample_rate),
            wet: common::setter(0.0, sample_rate),
            dry: common::setter(1.0, sample_rate),
            level: common::setter(1.0, sample_rate),
            time_ms: -1.0,
            started: false,
        }
    }

    /// Main curve 2 (two slopes) over 0..100.
    pub fn balance_curve(value: f32) -> f32 {
        zoom::curve(1.0, 0.0, 101, common::param(value, 100.0), 2)
    }

    pub fn time_ms(value: f32, bpm: f32) -> f32 {
        let value = common::param(value, 2005.0).round();
        if value <= f32::from(TIME_THRESHOLD) {
            value + 10.0
        } else {
            zoom::delay_note_ms(zoom::sync_index(value, TIME_THRESHOLD), bpm, MAX_MS)
        }
    }
}

impl Effect for ReverseDl {
    fn set(&mut self, values: &Values, bpm: f32) {
        let ms = Self::time_ms(values[0], bpm);
        let feedback = 0.95 * common::param(values[1], 100.0) / 100.0;
        let balance = common::param(values[2], 100.0);
        let wet = Self::balance_curve(balance);
        let dry = Self::balance_curve(100.0 - balance);
        let level = common::param(values[4], 150.0) / 100.0;
        self.damp = self.damping[common::index(values[3], 10)];
        self.feedback.set(feedback);
        self.wet.set(wet);
        self.dry.set(dry);
        self.level.set(level);
        if ms != self.time_ms {
            self.time_ms = ms;
            self.pending = (common::ms_to_samples(ms, self.sample_rate) as usize)
                .clamp(1, self.line.capacity() / 2);
            self.reopen = common::ms_to_samples(2.0 * (ms + 20.0), self.sample_rate) as u32;
            if self.started {
                self.muting = true;
                self.gate.mute();
            } else {
                self.window = self.pending;
            }
        }
        if !self.started {
            self.started = true;
            self.feedback.jump(feedback);
            self.wet.jump(wet);
            self.dry.jump(dry);
            self.level.jump(level);
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let feedback = self.feedback.tick();
        let wet = self.wet.tick();
        let dry = self.dry.tick();
        let level = self.level.tick();
        if self.muting && self.gate.closed() {
            self.muting = false;
            self.window = self.pending;
            self.gate.open_after(self.reopen);
        }
        let gain = self.gate.tick();
        self.phase += 1;
        if self.phase > self.window {
            self.phase = 0;
        }
        let phase = self.phase as f32;
        if phase < self.edge {
            self.envelope = (self.envelope + self.edge_step).min(1.0);
        }
        if phase > self.window as f32 - self.edge {
            self.envelope = (self.envelope - self.edge_step).max(0.0);
        }
        // Reads precede this frame's write: age a is `read(a - 1)`.
        let fixed = gain * self.line.read(self.window - 1);
        let reverse = gain * self.envelope * self.line.read((2 * self.phase).max(1) - 1);
        self.feedback_state =
            flush(self.feedback_state + (fixed - self.feedback_state) * self.damp);
        self.line
            .write(self.feedback_state * feedback + 0.5 * (left + right));
        let echo = reverse * wet;
        [level * (left * dry + echo), level * (right * dry + echo)]
    }

    fn clear(&mut self) {
        self.line.clear();
        self.feedback_state = 0.0;
        self.phase = 0;
        if self.muting {
            self.muting = false;
            self.window = self.pending;
        }
        self.gate.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(ReverseDl::new(sample_rate))
}

#[cfg(test)]
mod tests {
    use super::super::common::testing::*;
    use super::*;
    use crate::Model;

    fn defaults() -> Values {
        Model::from_key(INFO.key).unwrap().defaults()
    }

    #[test]
    fn a_ramp_comes_back_reversed() {
        let sample_rate = 44_100.0;
        let mut effect = ReverseDl::new(sample_rate);
        // Time 90 -> 100 ms windows; Bal 100: wet only.
        effect.set(&with(defaults(), &[(0, 90.0), (1, 0.0), (2, 100.0)]), 120.0);
        let n = 4410;
        let out = render(&mut effect, 3 * n, |i| {
            let x = (i % n) as f32 / n as f32;
            [x, x]
        });
        // Inside a later window, away from the fades, the output falls while the input rises.
        let a = out[2 * n + 1000][0];
        let b = out[2 * n + 3000][0];
        assert!(a > b + 0.2, "{a} {b}");
    }

    #[test]
    fn balance_uses_two_slopes() {
        assert!((ReverseDl::balance_curve(50.0) - 0.65).abs() < 1e-6);
        assert!((ReverseDl::balance_curve(100.0) - 1.0).abs() < 1e-6);
        assert!((ReverseDl::balance_curve(25.0) - 0.325).abs() < 1e-6);
    }

    #[test]
    fn window_follows_time_plus_ten() {
        let mut effect = ReverseDl::new(48_000.0);
        effect.set(&with(defaults(), &[(0, 0.0)]), 120.0);
        assert_eq!(effect.window, 480);
        assert_eq!(ReverseDl::time_ms(1990.0, 120.0), 2000.0);
        // Sync 1/4 at 120 BPM.
        assert_eq!(ReverseDl::time_ms(1997.0, 120.0), 500.0);
    }

    #[test]
    fn maximum_feedback_decays() {
        let mut effect = ReverseDl::new(44_100.0);
        effect.set(&with(defaults(), &[(0, 0.0), (1, 100.0), (3, 10.0)]), 120.0);
        let out = render(&mut effect, 441_000, |i| {
            if i < 4410 { [0.8, 0.8] } else { [0.0; 2] }
        });
        assert!(out.iter().all(|f| f[0].abs() < 16.0));
        assert!(rms(&out, 0, 400_000) < 1e-3);
    }
}
