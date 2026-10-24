//! The audio kernel NOISEGTE.ZDL and DIRTYGTE.ZDL share byte for byte
//! (`Fx_DYN_NoiseGate`/`Fx_DYN_DirtyGate`, .text 0..0x200), with the two
//! modules' complete editors (recovered `gate_parameters.rs`).
//!
//! Per sample: the detector takes the larger of |L| and |R| (EnvIn sets c15 = 1
//! for both modules), runs the original recursion
//! `f = c5 x + c6 f[-1] + c7 f[-1]` (both feedback taps read the same history
//! word, so the net pole is c6 + c7 = -0.981), then `peak = max(|f c8|,
//! peak c9)`. The hysteresis opens when `peak > c10` and closes when
//! `peak < c11`, in that order; the gain follows the open flag through
//! `g = c12 open + c13 g` (1.3 ms). Output is `x g c2 c0 + x (1 - c0)` times
//! the master fader c4; this port is the switched-on pedal (c0 = c4 = 1).
//!
//! NoiseGate: c9 = 0x3f7f786e (an 11 ms peak hold), separate open/close
//! thresholds. DirtyGate: c9 = 0 (no hold, the gain chatters with the
//! waveform) and one threshold for both. Threshold tables are the ROM words.
use super::common::{self, decay, one_pole};
use crate::dsp::flush;
use crate::zoom::Ramp;

/// THRSH → open c10, close c11 (Fx_DYN_NoiseGate_thr_tbl, NOISEGTE 0x80000290).
const NOISE_THRESHOLDS: [[u32; 2]; 25] = [
    [0x3a03126f, 0x3851b717],
    [0x3a09874c, 0x385c0bb8],
    [0x3a27986e, 0x3886138e],
    [0x3a67b800, 0x38b96002],
    [0x3aa8d702, 0x39071268],
    [0x3af6001d, 0x3944cce1],
    [0x3b2f1aa0, 0x398c154d],
    [0x3b72069d, 0x39c19ee4],
    [0x3ba27a4d, 0x3a01fb71],
    [0x3bd47ffc, 0x3a29fffe],
    [0x3c07ccb5, 0x3a594788],
    [0x3c2a2232, 0x3a881b5b],
    [0x3c517c4a, 0x3aa796a1],
    [0x3c7e1425, 0x3acb4351],
    [0x3c98104a, 0x3af34d43],
    [0x3cb3eb2c, 0x3b0fef56],
    [0x3cd2b429, 0x3b289021],
    [0x3cf483dd, 0x3b439cb1],
    [0x3d0cb911, 0x3b61281b],
    [0x3d20cb10, 0x3b80a274],
    [0x3d36832f, 0x3b92028c],
    [0x3d4dec66, 0x3ba4bd1e],
    [0x3d671166, 0x3bb8dab8],
    [0x3d80fe52, 0x3bce63b6],
    [0x3d8f5c29, 0x3be56042],
];

/// THRSH → c10 = c11 (Fx_DYN_DirtyGate_thr_tbl, DIRTYGTE 0x800003d8).
const DIRTY_THRESHOLDS: [u32; 25] = [
    0x38d1b717, 0x38e4387e, 0x391d32f5, 0x39791806, 0x39c87910, 0x3a1b868e, 0x3a6631f9, 0x3aa30cdc,
    0x3ade7923, 0x3b131471, 0x3b3d6d61, 0x3b6ea18b, 0x3b938354, 0x3bb37753, 0x3bd75409, 0x3bff3f41,
    0x3c15aeb9, 0x3c2de8f0, 0x3c485f5e, 0x3c652299, 0x3c822163, 0x3c92e7cd, 0x3ca4ec37, 0x3cb8361c,
    0x3ccccccd,
];

/// ROM c5, c6, c7 (detector), c8 (detector gain), c12, c13 (gain follower).
const C5: u32 = 0x3ffd9a99;
const C6: u32 = 0xbffa01ed;
const C7: u32 = 0x3f78cea7;
const C12: u32 = 0x3c8b3162;
const C13: u32 = 0x3f7ba675;
/// NoiseGate's initializer stores c9 = 0x3f7f786e; DirtyGate keeps ROM 0.
const NOISE_HOLD: u32 = 0x3f7f786e;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GateKind {
    Noise,
    Dirty,
}

pub(crate) struct Gate {
    kind: GateKind,
    c5: f32,
    c6: f32,
    c7: f32,
    hold: f32,
    open_threshold: f32,
    close_threshold: f32,
    attack: f32,
    keep: f32,
    level: Ramp,
    threshold_index: usize,
    primed: bool,
    filtered: f32,
    peak: f32,
    open: f32,
    gain: f32,
}

impl Gate {
    pub fn new(kind: GateKind, sample_rate: f32, threshold: f32, level: f32) -> Self {
        let hold = match kind {
            GateKind::Noise => decay(f32::from_bits(NOISE_HOLD), sample_rate),
            GateKind::Dirty => 0.0,
        };
        let (attack, keep) = one_pole(f32::from_bits(C12), f32::from_bits(C13), sample_rate);
        let mut gate = Self {
            kind,
            // The detector recursion stays per sample: its net pole sits at
            // -0.981 (next to Nyquist) and its DC gain is 1 at any rate.
            c5: f32::from_bits(C5),
            c6: f32::from_bits(C6),
            c7: f32::from_bits(C7),
            hold,
            open_threshold: 0.0,
            close_threshold: 0.0,
            attack,
            keep,
            level: common::level_ramp(level, sample_rate),
            threshold_index: usize::MAX,
            primed: false,
            filtered: 0.0,
            peak: 0.0,
            open: 0.0,
            gain: 0.0,
        };
        gate.set_threshold(common::index(threshold, 24));
        gate
    }

    fn set_threshold(&mut self, index: usize) {
        if index == self.threshold_index {
            return;
        }
        self.threshold_index = index;
        let [open, close] = match self.kind {
            GateKind::Noise => NOISE_THRESHOLDS[index],
            GateKind::Dirty => [DIRTY_THRESHOLDS[index]; 2],
        };
        self.open_threshold = f32::from_bits(open);
        self.close_threshold = f32::from_bits(close);
    }

    /// THRSH (parameter word 2) and Level (word 3).
    pub fn set(&mut self, threshold: f32, level: f32) {
        self.set_threshold(common::index(threshold, 24));
        self.level.set(common::level(level));
        if !self.primed {
            self.primed = true;
            common::settle(&mut self.level);
        }
    }

    #[cfg(test)]
    pub fn thresholds(&self) -> (f32, f32) {
        (self.open_threshold, self.close_threshold)
    }

    #[inline]
    pub fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let input = left.abs().max(right.abs());
        let previous = self.filtered;
        self.filtered = flush(input * self.c5 + previous * self.c6 + previous * self.c7);
        let candidate = self.filtered.abs();
        let decayed = self.peak * self.hold;
        self.peak = flush(if decayed < candidate {
            candidate
        } else {
            decayed
        });
        if self.open_threshold < self.peak {
            self.open = 1.0;
        }
        if self.peak < self.close_threshold {
            self.open = 0.0;
        }
        self.gain = flush(self.attack * self.open + self.keep * self.gain);
        let gain = self.gain * self.level.tick();
        [left * gain, right * gain]
    }

    pub fn clear(&mut self) {
        self.filtered = 0.0;
        self.peak = 0.0;
        self.open = 0.0;
        self.gain = 0.0;
        common::settle(&mut self.level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(gate: &mut Gate, amplitude: f32, frames: usize) -> f32 {
        let mut peak = 0.0f32;
        for i in 0..frames {
            let x = amplitude * (i as f32 * 0.05).sin();
            let [l, _] = gate.process([x, x]);
            peak = peak.max(l.abs());
        }
        peak
    }

    #[test]
    fn noise_gate_opens_above_and_closes_below_threshold() {
        let mut gate = Gate::new(GateKind::Noise, 48_000.0, 9.0, 100.0);
        gate.set(9.0, 100.0);
        let (open, close) = gate.thresholds();
        assert!(close < open);
        // Well above the open threshold: passes at unity (after the 1.3 ms attack).
        run(&mut gate, 0.3, 4800);
        let loud = run(&mut gate, 0.3, 4800);
        assert!((loud - 0.3).abs() < 0.01, "{loud}");
        // Far below the close threshold: muted.
        run(&mut gate, close * 0.1, 48_000);
        let quiet = run(&mut gate, close * 0.1, 4800);
        assert!(quiet < close * 1e-3, "{quiet}");
    }

    #[test]
    fn dirty_gate_has_no_hysteresis_and_no_hold() {
        let gate = Gate::new(GateKind::Dirty, 44_100.0, 24.0, 100.0);
        let (open, close) = gate.thresholds();
        assert_eq!(open, close);
        assert!((open - 0.025).abs() < 1e-6);
        assert_eq!(gate.hold, 0.0);
    }

    #[test]
    fn level_is_linear_percent() {
        let mut gate = Gate::new(GateKind::Noise, 44_100.0, 0.0, 150.0);
        gate.set(0.0, 150.0);
        run(&mut gate, 0.2, 4410);
        let out = run(&mut gate, 0.2, 4410);
        assert!((out - 0.3).abs() < 0.01, "{out}");
    }
}
