//! MojoRolle (MJROLLE.ZDL, `Fx_MOD_MojoRoller`): vibrato or pitch roll that
//! rises in after each picked note.
//!
//! Callback topology, followed as recovered: the dry input feeds a doubled
//! 50 Hz high-pass, a decaying peak and fast/slow followers; a rising attack
//! drops the gate target (`c14`), whose smoothed value (`s14`) falling below
//! 0.02 restarts the rise envelope and the pitch window. The envelope climbs
//! by the Rise increment; its cube times the depth word, smoothed (`c22`/
//! `c23`), multiplies the LFO wave (smoothed shape of a 2^32 phase LFO). In
//! Up-Dn mode that product is a delay of up to 946 samples read with linear
//! interpolation from the mono ring (vibrato). In Up/Down modes it advances a
//! two-tap sawtooth delay pitch shifter (946-sample window, 2-sample offset,
//! trapezoid crossfade windows) whose output is ducked by the gate. The wet
//! signal then gets a 12 kHz high-pass blended back in (`c42`, less with more
//! Depth) and Level, on both channels.
//!
//! Editors decoded from the disassembly: Depth (Up-Dn: square curve up to
//! 992301852 per 2^31; Up/Down: cubic curves of `Depth - 1` to -1520218 /
//! 2270067, i.e. about +5 / -12 semitones at the wave's peak; `c42 = 0.8 -
//! 0.8 (Depth/100)^2`), Speed (square integer curve 99854..1414903 per 2^32,
//! 1.03..14.5 Hz; synced values use the Main LFO divisions; in Up/Down modes
//! the speed also sets the wave's dead zone `c25` and slope `c26`), Rise
//! (sqrt integer curve 131072..16384 per 2^31 per sample, 0.4..3 s; 0 jumps
//! to full depth), Mode (LFO shape mix, smoothing poles and gate smoothing per
//! mode, applied between a 16 ms fade-out of `c37` and its fade-in) and Level
//! (`value / 100`). Deviation: the crossfade windows use the current window
//! phase instead of the callback's values from two samples earlier.
use super::common::{self, Changes, Q31, SwitchGate};
use crate::{
    Effect, Values,
    catalog::*,
    dsp::{Biquad, DelayLine, flush},
    zoom::{self, Ramp},
};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_mojo_rolle",
    name: "MojoRolle",
    category: Category::Modulation,
    zdl: "MJROLLE.ZDL",
    description: "Vibrato or pitch roll rising after each note",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 37,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Speed",
            max: 128,
            default: 57,
            display: Display::sync_rate(100, 0.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Rise",
            max: 100,
            default: 75,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Mode",
            max: 2,
            default: 0,
            display: Display::labels(&["Up-Dn", "Up", "Down"]),
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

/// A Q31 word shifted right by 20: 2048 samples at full scale.
const DELAY_SAMPLES: f32 = 2048.0;
/// `c49`: the pitch window span (Q31); `c51`: its 2-sample offset.
const WINDOW: f32 = 0.462_077_57;
const WINDOW_OFFSET: f32 = 2.0;
/// `c45`, `1 + c46`: trapezoid window shaping.
const WINDOW_LEAD: f32 = 0.07;
const WINDOW_SLOPE: f32 = 1.612_903_1;
/// `c13`: the detector's absolute threshold.
const TRIGGER: f32 = 1e-4;

/// Per-mode coefficients set by the Mode editor (44.1 kHz values).
#[derive(Clone, Copy)]
struct ModeSet {
    /// `c15`, `c16`: gate smoothing.
    gate: (f32, f32),
    /// `c28`, `c29`, `c30`: triangle, dead-zone ramp and sine weights.
    shape: (f32, f32, f32),
    /// `c31`, `c32`: wave smoothing.
    wave: (f32, f32),
}

const MODES: [ModeSet; 2] = [
    ModeSet {
        gate: (1.0, 0.0),
        shape: (0.25, 0.0, 0.75),
        wave: (1.0 / 2048.0, 1.0 - 1.0 / 2048.0),
    },
    ModeSet {
        gate: (0.05, 0.95),
        shape: (0.0, 1.0, 0.0),
        wave: (1.0 / 256.0, 1.0 - 1.0 / 256.0),
    },
];

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(MojoRolle::new(sample_rate))
}

/// Speed 0..100 → LFO Hz.
fn normal_speed_hz(value: f32) -> f32 {
    common::q32_hz(common::int_curve(1_414_903.0, 99_854.0, 101, value, 1))
}

/// Depth → the depth word (`c20` for Up-Dn, `c21` for Up/Down) as Q31.
fn depth_word(value: f32, mode: usize) -> f32 {
    let word = match mode {
        0 => common::int_curve(992_301_852.0, 0.0, 101, value, 1),
        _ if value < 0.5 => 0.0,
        1 => common::int_curve(-1_520_218.0, -26_301.0, 100, value - 1.0, 6),
        _ => common::int_curve(2_270_067.0, 26_149.0, 100, value - 1.0, 6),
    };
    (word / Q31) as f32
}

/// Rise → envelope increment per 44.1 kHz sample (full scale 1).
fn rise_increment(value: f32) -> f32 {
    if value < 0.5 {
        1.0
    } else {
        (common::int_curve(16_384.0, 131_072.0, 100, value, 3) / Q31) as f32
    }
}

/// The trapezoid crossfade window of the pitch shifter, `magnitude = |phase|`.
#[inline]
fn window(magnitude: f32) -> f32 {
    ((1.0 - (magnitude + WINDOW_LEAD).min(1.0)) * WINDOW_SLOPE).min(1.0)
}

struct MojoRolle {
    sample_rate: f32,
    scale: f32,
    changes: Changes,
    // Detector and rise envelope.
    detector: Biquad,
    peak: f32,
    peak_decay: f32,
    fast: f32,
    fast_gain: f32,
    fast_pole: f32,
    slow: f32,
    slow_gain: f32,
    slow_pole: f32,
    latch: bool,
    gate_target: f32,
    gate: f32,
    gate_gain: f32,
    gate_pole: f32,
    envelope: f32,
    rise: f32,
    // Modulation.
    depth: f32,
    depth_value: f32,
    amplitude: f32,
    amplitude_gain: f32,
    amplitude_pole: f32,
    lfo_phase: u32,
    lfo_increment: u32,
    speed_hz: f32,
    dead_zone: f32,
    slope: f32,
    shape: (f32, f32, f32),
    wave: f32,
    wave_gain: f32,
    wave_pole: f32,
    // Audio.
    line: DelayLine,
    window_phase: u32,
    tone: Biquad,
    boost: Ramp,
    mode: usize,
    pending: usize,
    switch: SwitchGate,
    level: Ramp,
}

impl MojoRolle {
    fn new(sample_rate: f32) -> Self {
        let bits = f32::from_bits;
        let scale = 1.0 / common::ratio(sample_rate);
        let (fast_gain, fast_pole) =
            common::one_pole(bits(0x3d8f_5c29), bits(0x3f6e_147b), sample_rate);
        let (slow_gain, slow_pole) =
            common::one_pole(bits(0x3c23_d70a), bits(0x3f7e_b852), sample_rate);
        let (amplitude_gain, amplitude_pole) =
            common::one_pole(1.0 / 2048.0, 1.0 - 1.0 / 2048.0, sample_rate);
        let mut effect = Self {
            sample_rate,
            scale,
            changes: Changes::new(),
            detector: common::first_order(
                bits(0x3ffe_8f7f),
                bits(0xbffe_8f7f),
                bits(0x3f7e_2eca),
                sample_rate,
            ),
            peak: 0.0,
            peak_decay: common::pole(bits(0x3f7f_be77), sample_rate),
            fast: 0.0,
            fast_gain,
            fast_pole,
            slow: 0.0,
            slow_gain,
            slow_pole,
            latch: false,
            gate_target: 0.0,
            gate: 0.0,
            gate_gain: 1.0,
            gate_pole: 0.0,
            envelope: 1.0,
            rise: 0.0,
            depth: 0.0,
            depth_value: 0.0,
            amplitude: 0.0,
            amplitude_gain,
            amplitude_pole,
            lfo_phase: 0,
            lfo_increment: 0,
            speed_hz: 1.0,
            dead_zone: 0.5,
            slope: 1.0,
            shape: MODES[0].shape,
            wave: 0.0,
            wave_gain: 0.0,
            wave_pole: 0.0,
            line: DelayLine::new((960.0 * scale).ceil() as usize + 4),
            window_phase: 0x8000_0000,
            tone: common::first_order(
                bits(0x3f09_5cf2),
                bits(0xbf09_5cf2),
                bits(0x3d95_cf1b),
                sample_rate,
            ),
            boost: Ramp::new(0.0, common::FAST_STEP, sample_rate),
            mode: 0,
            pending: 0,
            switch: SwitchGate::new(sample_rate),
            level: Ramp::new(1.0, zoom::DEFAULT_STEP, sample_rate),
        };
        effect.apply_mode();
        effect
    }

    /// The Mode editor's coefficient set; it also sets the rise envelope and
    /// the smoothed amplitude straight to full depth.
    fn apply_mode(&mut self) {
        self.mode = self.pending;
        let set = MODES[self.mode.min(1)];
        (self.gate_gain, self.gate_pole) =
            common::one_pole(set.gate.0, set.gate.1, self.sample_rate);
        (self.wave_gain, self.wave_pole) =
            common::one_pole(set.wave.0, set.wave.1, self.sample_rate);
        self.shape = set.shape;
        self.depth = depth_word(self.depth_value, self.mode);
        self.amplitude = self.depth;
        self.envelope = 1.0;
        self.update_wave_shape();
    }

    /// Up/Down: the rate editor's dead zone `c25` and slope `c26` from Speed.
    fn update_wave_shape(&mut self) {
        let t = ((self.speed_hz - 1.025_28) / 13.502_7).clamp(0.0, 1.0);
        let steps = (10_000.0 * t).trunc() / 9999.0;
        self.dead_zone = 0.1 + 0.4 * steps.min(1.0);
        self.slope = 0.555_555_6 + 0.444_444_4 * steps.min(1.0);
    }
}

impl Effect for MojoRolle {
    fn set(&mut self, values: &Values, bpm: f32) {
        let (changed, tempo, first) = self.changes.update(values, bpm);
        if changed & 1 != 0 {
            self.depth_value = values[0].clamp(0.0, 100.0);
            self.depth = depth_word(self.depth_value, self.mode);
            let ratio = self.depth_value / 100.0;
            self.boost.set(0.8 - 0.8 * ratio * ratio);
            if first {
                self.boost.jump(self.boost.target);
            }
        }
        if changed & 2 != 0 || (tempo && zoom::is_synced(values[1], 100)) {
            self.speed_hz = common::sync_hz(values[1], 100, bpm, normal_speed_hz);
            self.lfo_increment = common::increment(self.speed_hz, self.sample_rate);
            self.update_wave_shape();
        }
        if changed & 4 != 0 {
            self.rise = (rise_increment(values[2].clamp(0.0, 100.0)) / self.scale).min(1.0);
        }
        if changed & 8 != 0 {
            let mode = common::index(values[3], 2);
            if first {
                self.pending = mode;
                self.apply_mode();
            } else if mode != self.pending {
                self.pending = mode;
                self.switch.request();
            }
        }
        if changed & 16 != 0 {
            self.level.set(values[4].max(0.0) / 100.0);
            if first {
                self.level.jump(self.level.target);
            }
        }
    }

    #[inline]
    fn process(&mut self, [left, right]: [f32; 2]) -> [f32; 2] {
        let mean = 0.5 * (left + right);
        if self.switch.tick() {
            self.apply_mode();
        }
        // Detector and rise envelope.
        let high = self.detector.process(mean).abs();
        let decayed = self.peak * self.peak_decay;
        self.peak = flush(if decayed < high { high } else { decayed });
        self.fast = flush(self.fast * self.fast_pole + self.fast_gain * self.peak);
        self.slow = flush(self.slow * self.slow_pole + self.slow_gain * self.peak);
        let trigger = self.fast >= TRIGGER && self.slow < self.fast;
        if trigger && !self.latch {
            self.gate_target = 0.0;
        }
        self.latch = trigger;
        self.gate = flush(self.gate * self.gate_pole + self.gate_gain * self.gate_target);
        if self.gate > 0.02 {
            self.envelope = (self.envelope + self.rise).min(1.0);
        } else {
            self.envelope = 0.0;
            self.gate_target = 1.0;
            self.window_phase = 0x8000_0000;
        }
        let cube = self.envelope * self.envelope * self.envelope;
        // Modulation.
        self.lfo_phase = self.lfo_phase.wrapping_add(self.lfo_increment);
        let magnitude = (self.lfo_phase as i32 as f32).abs() * (1.0 / 2_147_483_648.0);
        let (triangle, ramp, sine) = self.shape;
        let dead_zone = ((magnitude - self.dead_zone).max(0.0) * self.slope * 2.0).min(1.0);
        let folded = magnitude - 0.5;
        let parabola = (2.0 * folded * (1.0 - folded.abs()) + 0.5).clamp(0.0, 1.0);
        let shaped = (triangle * magnitude + ramp * dead_zone + sine * parabola).clamp(-1.0, 1.0);
        self.amplitude =
            flush(self.amplitude * self.amplitude_pole + self.amplitude_gain * self.depth * cube);
        self.wave = flush(self.wave * self.wave_pole + self.wave_gain * shaped);
        let product = self.wave * self.amplitude;
        // Audio.
        self.line.write(mean);
        let wet = if self.mode == 0 {
            self.line.read_linear(product * DELAY_SAMPLES * self.scale)
        } else {
            let advance = (product * Q31 as f32 / self.scale) as i32;
            self.window_phase = self.window_phase.wrapping_add(advance as u32);
            let first = self.window_phase as i32 as f32 * (1.0 / 2_147_483_648.0);
            let second =
                self.window_phase.wrapping_sub(0x8000_0000) as i32 as f32 * (1.0 / 2_147_483_648.0);
            let span = WINDOW * DELAY_SAMPLES * self.scale;
            let offset = WINDOW_OFFSET * self.scale;
            let tap = |phase: f32| offset + span * (0.5 + 0.5 * phase);
            let a = self.line.read_linear(tap(first)) * window(first.abs());
            let b = self.line.read_linear(tap(second)) * window(second.abs());
            self.gate * (a + b)
        };
        let input = wet * self.switch.gain;
        let colored = input + self.boost.tick() * self.tone.process(input);
        let out = colored * self.level.tick();
        [out, out]
    }

    fn clear(&mut self) {
        self.detector.clear();
        self.peak = 0.0;
        self.fast = 0.0;
        self.slow = 0.0;
        self.latch = false;
        self.gate_target = 0.0;
        self.gate = 0.0;
        self.envelope = 0.0;
        self.amplitude = 0.0;
        self.lfo_phase = 0;
        self.wave = 0.0;
        self.line.clear();
        self.window_phase = 0x8000_0000;
        self.tone.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Model;

    #[test]
    fn speed_and_depth_follow_the_editors() {
        assert!((normal_speed_hz(0.0) - 1.0253).abs() < 1e-3);
        assert!((normal_speed_hz(100.0) - 14.528).abs() < 1e-2);
        assert!((common::sync_hz(108.0, 100, 120.0, normal_speed_hz) - 2.0).abs() < 1e-4);
        // Up-Dn at Depth 100: a 946-sample delay swing.
        assert!((depth_word(100.0, 0) * DELAY_SAMPLES - 946.3).abs() < 0.2);
        // Down at Depth 100 halves the pitch at the wave's peak.
        let slope = depth_word(100.0, 2) * WINDOW * DELAY_SAMPLES * 0.5;
        assert!((slope - 0.5).abs() < 0.01, "{slope}");
        assert!(depth_word(100.0, 1) < 0.0 && depth_word(0.0, 1) == 0.0);
    }

    #[test]
    fn rise_follows_the_editor_curve() {
        let seconds = |rise: f32| 1.0 / rise_increment(rise) / 44_100.0;
        assert!((seconds(100.0) - 2.97).abs() < 0.01);
        assert!((seconds(1.0) - 0.407).abs() < 0.01);
        assert_eq!(rise_increment(0.0), 1.0);
    }

    fn zero_crossing_hz(mode: f32, rate: f32) -> f64 {
        let mut effect = MojoRolle::new(rate);
        let mut values = Model::from_key("mooz_mojo_rolle").unwrap().defaults();
        values[0] = 100.0;
        values[1] = 0.0; // slowest LFO
        values[2] = 0.0; // full depth at once
        values[3] = mode;
        effect.set(&values, 120.0);
        let tone = 220.0;
        let mut crossings = 0;
        let mut last = 0.0f32;
        let total = rate as usize;
        let mut counted = 0;
        for i in 0..total {
            let x = 0.3 * (std::f32::consts::TAU * tone * i as f32 / rate).sin();
            let [out, _] = effect.process([x, x]);
            // The slowest LFO's wave sits near its peak around 0.4..0.6 s.
            let t = i as f32 / rate;
            if t > 0.42 && t < 0.58 {
                if last <= 0.0 && out > 0.0 {
                    crossings += 1;
                }
                counted += 1;
            }
            last = out;
        }
        crossings as f64 / (counted as f64 / rate as f64)
    }

    #[test]
    fn down_mode_shifts_pitch_down() {
        let hz = zero_crossing_hz(2.0, 48_000.0);
        assert!(hz < 200.0 && hz > 90.0, "{hz}");
        let up = zero_crossing_hz(1.0, 48_000.0);
        assert!(up > 240.0 && up < 330.0, "{up}");
    }

    #[test]
    fn default_vibrato_is_bounded() {
        let mut effect = MojoRolle::new(44_100.0);
        effect.set(
            &Model::from_key("mooz_mojo_rolle").unwrap().defaults(),
            120.0,
        );
        let mut peak = 0.0f32;
        for i in 0..88_200 {
            let x = 0.4 * (i as f32 * 0.03).sin();
            peak = peak.max(effect.process([x, x])[0].abs());
        }
        assert!(peak > 0.3 && peak < 1.0, "{peak}");
    }
}
