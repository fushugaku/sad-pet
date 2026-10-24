//! Shared pieces of the pitch group: the Main services the editors call (as
//! plain f32 math), the window shapes of the recovered shifter callbacks, and
//! a monophonic pitch tracker that stands in for Main's detector (the period,
//! note and level the HPS/MonoPitch/Ba Pitch `pitchUpdata` editors read through
//! `context[29]`, `context[33]` and `context[28]`).
use crate::dsp::{Biquad, Coefficients, flush};
use crate::zoom::{self, ORIGINAL_RATE};

/// `GetString_PitSft` (PTCHSHFT, MNPIT, B_PITCH): Shift 0..25.
pub const SHIFT_LABELS: &[&str] = &[
    "-12", "-11", "-10", "-9", "-8", "-7", "-6", "-5", "-4", "-3", "-2", "-1", "0", "1", "2", "3",
    "4", "5", "6", "7", "8", "9", "10", "11", "12", "24",
];

/// Semitones of a Shift step: 0..24 are -12..+12, 25 is two octaves up.
pub fn shift_semitones(step: usize) -> f32 {
    if step >= 25 { 24.0 } else { step as f32 - 12.0 }
}

/// The pitch ratio of a Shift step plus a Fine step (0..50 = -25..+25 cents),
/// as the PTCHSHFT/MNPIT/B_PITCH increment tables encode it.
pub fn shift_ratio(shift: usize, fine: usize) -> f32 {
    let semitones = shift_semitones(shift) + (fine as f32 - 25.0) * 0.01;
    (semitones / 12.0).exp2()
}

/// A stepped parameter in pedal units.
pub fn step(value: f32, max: usize) -> usize {
    (value.round().max(0.0) as usize).min(max)
}

/// Main's two-slope curve over 0..100 (`context[21]`, maximum 1, shape 2), the
/// Bal/Mix law of most editors here.
pub fn two_slope(value: f32) -> f32 {
    zoom::curve(1.0, 0.0, 101, value.clamp(0.0, 100.0), 2)
}

/// A linear 0..100 knob as 0..1 (`context[21]`, maximum 1, shape 0).
pub fn percent(value: f32) -> f32 {
    zoom::curve(1.0, 0.0, 101, value.clamp(0.0, 100.0), 0)
}

/// Every `outLv_edit`: `context[21]` over 0..150 with maximum 1.5.
pub fn out_level(value: f32) -> f32 {
    zoom::curve(1.5, 0.0, 151, value.clamp(0.0, 150.0), 0)
}

/// Samples of the 44.1 kHz original at the instance rate.
pub fn samples(at_44k: f32, sample_rate: f32) -> f32 {
    crate::dsp::scale_samples(at_44k, sample_rate)
}

/// A per-sample decay factor of the original at the instance rate.
pub fn decay(factor: f32, sample_rate: f32) -> f32 {
    (factor as f64).powf(ORIGINAL_RATE as f64 / sample_rate as f64) as f32
}

/// The gain `a` of a ROM one-pole `y += a (x - y)` at the instance rate.
pub fn one_pole(gain: f32, sample_rate: f32) -> f32 {
    1.0 - decay(1.0 - gain, sample_rate)
}

/// A ROM biquad in the callbacks' feedback form
/// `y = b0 x + b1 x1 + b2 x2 + f1 y1 + f2 y2`, re-derived for `rate` from the
/// rate it was designed for.
pub fn rom_biquad(words: &[u32], designed_at: f32, rate: f32) -> Coefficients {
    let w = |i: usize| f32::from_bits(words[i]);
    Coefficients {
        b0: w(0),
        b1: w(1),
        b2: w(2),
        a1: -w(3),
        a2: -w(4),
    }
    .resample(designed_at, rate)
}

/// A ROM first-order section `y = b0 x + b1 x1 + f1 y1` at `rate`.
pub fn rom_first_order(words: &[u32], designed_at: f32, rate: f32) -> Coefficients {
    let w = |i: usize| f32::from_bits(words[i]);
    Coefficients {
        b0: w(0),
        b1: w(1),
        b2: 0.0,
        a1: -w(2),
        a2: 0.0,
    }
    .resample(designed_at, rate)
}

/// Main's filter designer (`context[25]`, c00bcb20 → c00bc2e0) type 9, the
/// first-order low shelf: `K = tan(π f / fs)`, a cut moves the corner up by
/// the cut's ratio. Same formulas as the dynamics_eq group's port.
pub fn low_shelf(sample_rate: f32, hz: f32, gain_db: f32) -> Coefficients {
    let fs = f64::from(sample_rate);
    let gain = 10f64.powf(f64::from(gain_db) / 20.0);
    let hz = f64::from(hz) / if gain_db >= 0.0 { 1.0 } else { gain };
    let k = (std::f64::consts::PI * hz.clamp(1.0, fs * 0.49) / fs).tan();
    let n = 1.0 / (1.0 + k);
    Coefficients {
        b0: ((1.0 + k * gain) * n) as f32,
        b1: ((k * gain - 1.0) * n) as f32,
        b2: 0.0,
        a1: (-(1.0 - k) * n) as f32,
        a2: 0.0,
    }
}

/// Main's designer type 10, the Zölzer peak (boost branch; cuts mirror it).
pub fn peak(sample_rate: f32, hz: f32, gain_db: f32, q: f32) -> Coefficients {
    let fs = f64::from(sample_rate);
    let k = (std::f64::consts::PI * f64::from(hz).clamp(1.0, fs * 0.49) / fs).tan();
    let (k2, q) = (k * k, f64::from(q).max(1e-3));
    let v = 10f64.powf(f64::from(gain_db).abs() / 20.0);
    let (zero, pole) = if gain_db >= 0.0 { (v, 1.0) } else { (1.0, v) };
    let n = 1.0 / (1.0 + pole * k / q + k2);
    Coefficients {
        b0: ((1.0 + zero * k / q + k2) * n) as f32,
        b1: (2.0 * (k2 - 1.0) * n) as f32,
        b2: ((1.0 - zero * k / q + k2) * n) as f32,
        a1: (2.0 * (k2 - 1.0) * n) as f32,
        a2: ((1.0 - pole * k / q + k2) * n) as f32,
    }
}

/// Main `context[7]` (c00cc94c): a coefficient moves linearly to its target in
/// a fixed number of samples (the editors pass 705.6, 16 ms at 44.1 kHz).
#[derive(Debug, Clone, Copy)]
pub struct TimedRamp {
    pub current: f32,
    target: f32,
    step: f32,
    length: f32,
}

/// The 16 ms of every `context[7]` call in this group.
pub const FADE_SECONDS: f32 = 705.6 / ORIGINAL_RATE;

impl TimedRamp {
    pub fn new(value: f32, seconds: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            length: (seconds * sample_rate).max(1.0),
        }
    }
    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step = (target - self.current).abs() / self.length;
        }
    }
    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }
    pub fn target(&self) -> f32 {
        self.target
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.current != self.target {
            let difference = self.target - self.current;
            if difference.abs() <= self.step {
                self.current = self.target;
            } else {
                self.current += self.step.copysign(difference);
            }
        }
        self.current
    }
}

/// Trapezoid window threshold c6 (0x08f5c200) and slope c7 (0x4e739c00) of the
/// PTCHSHFT/DETUNE/B_DETUNE/Z_ORGAN shifters.
const TRAPEZOID_THRESHOLD: f32 = 150_323_712.0 / 2_147_483_648.0;
const TRAPEZOID_SLOPE: f32 = 1.0 + 1_316_199_424.0 / 2_147_483_648.0;

/// Magnitude of the trapezoid window of a Q32 tap phase: saturated
/// `(|phase| + c6 - 1) (1 + c7)`. Flat 1 around the sweep centre, zero near
/// the wrap where the tap jumps.
#[inline]
pub fn trapezoid(phase: u32) -> f32 {
    let x = (phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0);
    ((1.0 - (x + TRAPEZOID_THRESHOLD).min(1.0)) * TRAPEZOID_SLOPE).min(1.0)
}

/// The tap sweep `(phase >> 1) + 0x40000000` as 0..1.
#[inline]
pub fn sweep(phase: u32) -> f32 {
    (phase as i32) as f32 * (1.0 / 4_294_967_296.0) + 0.5
}

/// Triangle window `|phase|` of the HPS/MNPIT/B_PITCH shifters, 0..1.
#[inline]
pub fn triangle(phase: u32) -> f32 {
    (phase as i32).unsigned_abs() as f32 * (1.0 / 2_147_483_648.0)
}

/// What the stand-in for Main's pitch detector reports after each analysis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Detection {
    /// Period in samples of the original 44.1 kHz rate, 0 when unvoiced.
    pub period: f32,
    /// Pitch class with C = 0, `None` when unvoiced (Main answers 12).
    pub note: Option<u8>,
    /// Peak input level of the analysed hop.
    pub level: f32,
}

/// Main's level gate in every `pitchUpdata`: 0x3b4f3e39, -50 dB.
pub const DETECTOR_GATE: f32 = 0.003_162_277_6;

/// Monophonic pitch tracker (YIN on a decimated copy of the input). Main's own
/// detector is not recovered; this one reports the same three quantities on a
/// fixed hop of about 11.6 ms.
pub struct PitchTracker {
    decimation: usize,
    phase: usize,
    lowpass: [Biquad; 2],
    ring: Vec<f32>,
    write: usize,
    since: usize,
    scratch: Vec<f32>,
    difference: Vec<f32>,
    peak: f32,
    rate: f32,
    to_original: f32,
    tau_min: usize,
    tau_max: usize,
    pub detection: Detection,
}

const RING: usize = 1024;
const WINDOW: usize = 256;
const HOP: usize = 128;

impl PitchTracker {
    pub fn new(sample_rate: f32) -> Self {
        let decimation = ((sample_rate / 11_025.0).round() as usize).max(1);
        let rate = sample_rate / decimation as f32;
        let tau_min = ((rate / 1_500.0).floor() as usize).max(2);
        let tau_max = ((rate / 40.0).ceil() as usize).min(RING - WINDOW - 2);
        let corner = (rate * 0.2).min(2_000.0);
        let lowpass = Coefficients::lowpass(sample_rate, corner, std::f32::consts::FRAC_1_SQRT_2);
        Self {
            decimation,
            phase: 0,
            lowpass: [Biquad::new(lowpass); 2],
            ring: vec![0.0; RING],
            write: 0,
            since: 0,
            scratch: vec![0.0; WINDOW + tau_max + 2],
            difference: vec![0.0; tau_max + 2],
            peak: 0.0,
            rate,
            to_original: decimation as f32 * ORIGINAL_RATE / sample_rate,
            tau_min,
            tau_max,
            detection: Detection {
                period: 0.0,
                note: None,
                level: 0.0,
            },
        }
    }

    pub fn clear(&mut self) {
        self.lowpass.iter_mut().for_each(Biquad::clear);
        self.ring.fill(0.0);
        self.phase = 0;
        self.since = 0;
        self.peak = 0.0;
        self.detection = Detection {
            period: 0.0,
            note: None,
            level: 0.0,
        };
    }

    /// Feed one mono sample; true when a new [`Detection`] is ready.
    #[inline]
    pub fn push(&mut self, x: f32) -> bool {
        self.peak = self.peak.max(x.abs());
        let first = self.lowpass[0].process(x);
        let filtered = self.lowpass[1].process(first);
        self.phase += 1;
        if self.phase < self.decimation {
            return false;
        }
        self.phase = 0;
        self.write = (self.write + 1) & (RING - 1);
        self.ring[self.write] = filtered;
        self.since += 1;
        if self.since < HOP {
            return false;
        }
        self.since = 0;
        self.analyse();
        true
    }

    fn analyse(&mut self) {
        let level = self.peak;
        self.peak = 0.0;
        let length = self.scratch.len();
        for (i, value) in self.scratch.iter_mut().enumerate() {
            *value = self.ring[(self.write + RING + 1 - length + i) & (RING - 1)];
        }
        let reference = &self.scratch[..WINDOW];
        let mut running = 0.0f32;
        self.difference[0] = 1.0;
        for tau in 1..=self.tau_max {
            let d = squared_difference(reference, &self.scratch[tau..tau + WINDOW]);
            running += d;
            self.difference[tau] = if running > 1e-12 {
                d * tau as f32 / running
            } else {
                1.0
            };
        }
        let d = &self.difference;
        let mut best = None;
        let mut tau = self.tau_min;
        while tau <= self.tau_max {
            if d[tau] < 0.15 {
                while tau < self.tau_max && d[tau + 1] < d[tau] {
                    tau += 1;
                }
                best = Some(tau);
                break;
            }
            tau += 1;
        }
        if best.is_none() {
            let (at, value) = (self.tau_min..=self.tau_max)
                .map(|t| (t, d[t]))
                .fold((0, f32::MAX), |a, b| if b.1 < a.1 { b } else { a });
            if value < 0.35 {
                best = Some(at);
            }
        }
        self.detection = match best {
            Some(tau) if level > 1e-5 => {
                let (a, b, c) = (d[tau - 1], d[tau], d[(tau + 1).min(self.tau_max)]);
                let denominator = a - 2.0 * b + c;
                let offset = if denominator.abs() > 1e-9 {
                    (0.5 * (a - c) / denominator).clamp(-0.5, 0.5)
                } else {
                    0.0
                };
                let period = tau as f32 + offset;
                let hz = self.rate / period;
                let midi = 69.0 + 12.0 * (hz / 440.0).log2();
                Detection {
                    period: period * self.to_original,
                    note: Some((midi.round() as i32).rem_euclid(12) as u8),
                    level,
                }
            }
            _ => Detection {
                period: 0.0,
                note: None,
                level,
            },
        };
    }
}

/// `sum((a - b)^2)` over equal slices, in eight lanes so it vectorizes.
fn squared_difference(a: &[f32], b: &[f32]) -> f32 {
    let mut lanes = [0.0f32; 8];
    for (x, y) in a.chunks_exact(8).zip(b.chunks_exact(8)) {
        for lane in 0..8 {
            let d = x[lane] - y[lane];
            lanes[lane] += d * d;
        }
    }
    flush(lanes.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_ratios_follow_the_increment_tables() {
        assert!((shift_ratio(0, 25) - 0.5).abs() < 1e-6);
        assert!((shift_ratio(24, 25) - 2.0).abs() < 1e-5);
        assert!((shift_ratio(25, 25) - 4.0).abs() < 1e-5);
        assert!((shift_ratio(12, 50) - 2f32.powf(0.25 / 12.0)).abs() < 1e-6);
    }

    #[test]
    fn trapezoid_is_flat_in_the_middle_and_closed_at_the_wrap() {
        assert_eq!(trapezoid(0), 1.0);
        assert_eq!(trapezoid(0x8000_0000), 0.0);
        let half = trapezoid(0x4000_0000);
        assert!((half - (0.93 - 0.5) * 1.6129).abs() < 1e-3, "{half}");
        assert!((sweep(0x8000_0000) - 0.0).abs() < 1e-6);
        assert!((sweep(0) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn timed_ramp_takes_its_duration() {
        let mut ramp = TimedRamp::new(0.0, FADE_SECONDS, 44_100.0);
        ramp.set(1.0);
        let mut n = 0;
        while ramp.tick() < 1.0 {
            n += 1;
        }
        assert!((704..=706).contains(&n), "{n}");
    }

    #[test]
    fn tracker_finds_the_period_and_note() {
        for sample_rate in [44_100.0f32, 48_000.0, 96_000.0] {
            for hz in [82.41f32, 110.0, 196.0, 440.0, 659.3] {
                let mut tracker = PitchTracker::new(sample_rate);
                let mut last = None;
                for i in 0..(sample_rate as usize / 2) {
                    let t = i as f32 / sample_rate;
                    let phase = t * hz * std::f32::consts::TAU;
                    let x =
                        0.4 * phase.sin() + 0.2 * (2.0 * phase).sin() + 0.1 * (3.0 * phase).sin();
                    if tracker.push(x) {
                        last = Some(tracker.detection);
                    }
                }
                let detection = last.unwrap();
                let expected = ORIGINAL_RATE / hz;
                assert!(
                    (detection.period - expected).abs() < expected * 0.01,
                    "{sample_rate} {hz}: {detection:?} vs {expected}"
                );
                let midi = (69.0 + 12.0 * (hz / 440.0).log2()).round() as i32;
                assert_eq!(detection.note, Some(midi.rem_euclid(12) as u8), "{hz}");
            }
        }
    }

    #[test]
    fn tracker_reports_silence_as_unvoiced() {
        let mut tracker = PitchTracker::new(48_000.0);
        let mut seen = false;
        for _ in 0..48_000 {
            seen |= tracker.push(0.0);
        }
        assert!(seen);
        assert_eq!(tracker.detection.note, None);
        assert_eq!(tracker.detection.period, 0.0);
    }
}
