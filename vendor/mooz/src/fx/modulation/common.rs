//! Building blocks shared by the modulation group: the Q31 LFO shapes of the
//! callbacks, the Main rate curves the editors call, ramps that mirror Main's
//! float/integer setters, first-order filters and a first-order all-pass whose
//! 44.1 kHz coefficient is re-warped to the host rate.
use crate::dsp::{Coefficients, flush};
use crate::zoom::{self, ORIGINAL_RATE};

/// One LFO cycle of the original u32 phase accumulators.
const TURN: f64 = 4_294_967_296.0;

/// An original phase increment (u32 turns per 44.1 kHz sample) in Hz.
pub fn increment_hz(increment: f32) -> f32 {
    increment * ORIGINAL_RATE / TURN as f32
}

/// u32 phase increment per host sample for a frequency in Hz.
pub fn hz_increment(hz: f32, sample_rate: f32) -> u32 {
    let turns = (f64::from(hz.max(0.0)) / f64::from(sample_rate)).fract();
    (turns * TURN) as u32
}

/// Wrapping u32 phase, read as the signed Q31 word of the callbacks.
#[derive(Debug, Clone, Copy, Default)]
pub struct Lfo {
    pub phase: u32,
    pub increment: u32,
}

impl Lfo {
    pub fn set_hz(&mut self, hz: f32, sample_rate: f32) {
        self.increment = hz_increment(hz, sample_rate);
    }
    /// Advance and return the new phase as a signed fraction in [-1, 1).
    #[inline]
    pub fn tick(&mut self) -> f32 {
        self.phase = self.phase.wrapping_add(self.increment);
        signed(self.phase)
    }
}

/// A u32 phase as a signed Q31 fraction in [-1, 1).
#[inline]
pub fn signed(phase: u32) -> f32 {
    (phase as i32) as f32 * (1.0 / 2_147_483_648.0)
}

/// `|phase| + 0xc0000000`: a triangle in [-0.5, 0.5], +0.5 at phase 0.
#[inline]
pub fn centered_triangle(s: f32) -> f32 {
    s.abs() - 0.5
}

/// `4 t (1 - |t|)` for t in [-0.5, 0.5]: the callbacks' parabolic sine,
/// [-1, 1] with zero slope at the peaks.
#[inline]
pub fn parabola(t: f32) -> f32 {
    4.0 * t * (1.0 - t.abs())
}

/// Main integer/float curve (`context[21]`/`[22]`) at a possibly fractional
/// position, linearly interpolated between the stepped original values.
pub fn stepped(value: f32, f: impl Fn(u32) -> f32) -> f32 {
    let value = value.max(0.0);
    let low = value.floor();
    let fraction = value - low;
    let a = f(low as u32);
    if fraction <= 0.0 {
        a
    } else {
        a + (f(low as u32 + 1) - a) * fraction
    }
}

/// `zoom::curve` taking an integer position.
pub fn curve(maximum: f32, minimum: f32, count: u32, selected: u32, shape: u8) -> f32 {
    zoom::curve(maximum, minimum, count, selected as f32, shape)
}

/// Linear interpolation into a per-value table (fractional positions).
pub fn table(values: &[f32], position: f32) -> f32 {
    let last = values.len() - 1;
    let position = position.clamp(0.0, last as f32);
    let index = (position as usize).min(last);
    let fraction = position - index as f32;
    if index == last {
        values[last]
    } else {
        values[index] + (values[index + 1] - values[index]) * fraction
    }
}

/// A 44.1 kHz per-sample one-pole coefficient (`y += a (x - y)`) at `rate`.
pub fn one_pole_at(coefficient_44k: f32, sample_rate: f32) -> f32 {
    let keep = 1.0 - f64::from(coefficient_44k.clamp(0.0, 1.0));
    (1.0 - keep.powf(f64::from(ORIGINAL_RATE) / f64::from(sample_rate))) as f32
}

/// A 44.1 kHz feedback pole moved to `rate` with the same time constant.
pub fn pole_at(pole_44k: f32, sample_rate: f32) -> f32 {
    (f64::from(pole_44k.clamp(0.0, 1.0)).powf(f64::from(ORIGINAL_RATE) / f64::from(sample_rate)))
        as f32
}

/// Main's float setter (`context[8]`, step per 44.1 kHz sample) or its
/// duration variant (`context[7]`, `A6` samples to reach the target).
#[derive(Debug, Clone, Copy)]
pub struct Ramp {
    pub current: f32,
    pub target: f32,
    step: f32,
    duration: f32,
}

impl Ramp {
    /// Fixed step per 44.1 kHz sample, like `context[8]`.
    pub fn with_step(value: f32, step_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: step_at_44k * ORIGINAL_RATE / sample_rate,
            duration: 0.0,
        }
    }
    /// Reaches each new target after `samples_at_44k`, like `context[7]`/`[9]`.
    pub fn with_duration(value: f32, samples_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            duration: (samples_at_44k * sample_rate / ORIGINAL_RATE).max(1.0),
        }
    }
    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            if self.duration > 0.0 {
                self.step = (target - self.current).abs() / self.duration;
            }
        }
    }
    pub fn jump(&mut self, value: f32) {
        self.current = value;
        self.target = value;
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.current != self.target {
            let difference = self.target - self.current;
            if difference.abs() <= self.step {
                self.current = self.target;
            } else if difference > 0.0 {
                self.current += self.step;
            } else {
                self.current -= self.step;
            }
        }
        self.current
    }
    #[cfg(test)]
    pub fn settled(&self) -> bool {
        self.current == self.target
    }
}

/// `y = b0 x + b1 x1 - a1 y1`, the first-order sections of the callbacks.
#[derive(Debug, Clone, Copy, Default)]
pub struct FirstOrder {
    pub b0: f32,
    pub b1: f32,
    pub a1: f32,
    x1: f32,
    y1: f32,
}

impl FirstOrder {
    pub fn new(c: Coefficients) -> Self {
        Self {
            b0: c.b0,
            b1: c.b1,
            a1: c.a1,
            x1: 0.0,
            y1: 0.0,
        }
    }
    pub fn set(&mut self, c: Coefficients) {
        self.b0 = c.b0;
        self.b1 = c.b1;
        self.a1 = c.a1;
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = flush(self.b0 * x + self.b1 * self.x1 - self.a1 * self.y1);
        self.x1 = x;
        self.y1 = y;
        y
    }
    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

/// A ROM biquad in the callbacks' form `y = c0 x + c1 x1 + c2 x2 + c3 y1 + c4 y2`
/// (44.1 kHz), re-derived for the host rate with the same analog prototype.
pub fn rom_biquad(c: [f32; 5], sample_rate: f32) -> Coefficients {
    Coefficients {
        b0: c[0],
        b1: c[1],
        b2: c[2],
        a1: -c[3],
        a2: -c[4],
    }
    .resample(ORIGINAL_RATE, sample_rate)
}

/// A ROM first-order section `y = c0 x + c1 x1 + c2 y1` (44.1 kHz) moved to the
/// host rate through its bilinear prototype, kept first-order.
pub fn rom_first_order(c: [f32; 3], sample_rate: f32) -> Coefficients {
    let (b0, b1, a1) = (f64::from(c[0]), f64::from(c[1]), -f64::from(c[2]));
    let r = f64::from(sample_rate) / f64::from(ORIGINAL_RATE);
    // H(z) -> polynomials in u = s / (2 fs): (n0 + n1 u) / (d0 + d1 u).
    let (n0, n1) = (b0 + b1, b0 - b1);
    let (d0, d1) = (1.0 + a1, 1.0 - a1);
    // u at 44.1 kHz = r u at the host rate; back with u = (1 - z^-1)/(1 + z^-1).
    let (nb0, nb1) = (n0 + r * n1, n0 - r * n1);
    let (da0, da1) = (d0 + r * d1, d0 - r * d1);
    Coefficients {
        b0: (nb0 / da0) as f32,
        b1: (nb1 / da0) as f32,
        b2: 0.0,
        a1: (da1 / da0) as f32,
        a2: 0.0,
    }
}

/// Maps an original first-order all-pass coefficient `w` (the callbacks'
/// `z = x + w h; y = h - w z`, designed at 44.1 kHz) to the coefficient with the
/// same 90° frequency at the host rate. Identity at 44.1 kHz.
#[derive(Debug, Clone)]
pub struct AllpassWarp {
    table: Option<Box<[f32; WARP_POINTS + 1]>>,
}

const WARP_POINTS: usize = 1024;

impl AllpassWarp {
    pub fn new(sample_rate: f32) -> Self {
        if (sample_rate - ORIGINAL_RATE).abs() < 0.5 {
            return Self { table: None };
        }
        let mut table = Box::new([0.0; WARP_POINTS + 1]);
        let ratio = f64::from(ORIGINAL_RATE) / f64::from(sample_rate);
        for (i, slot) in table.iter_mut().enumerate() {
            let w = (i as f64 / WARP_POINTS as f64) * 2.0 - 1.0;
            let w = w.clamp(-0.999_999, 0.999_999);
            let t = (1.0 - w) / (1.0 + w);
            let warped = (t.atan() * ratio)
                .min(std::f64::consts::FRAC_PI_2 * 0.999_999)
                .tan();
            *slot = ((1.0 - warped) / (1.0 + warped)) as f32;
        }
        Self { table: Some(table) }
    }
    #[inline]
    pub fn map(&self, w: f32) -> f32 {
        match &self.table {
            None => w,
            Some(table) => {
                let position = ((w.clamp(-1.0, 1.0) + 1.0) * 0.5) * WARP_POINTS as f32;
                let index = (position as usize).min(WARP_POINTS - 1);
                let fraction = position - index as f32;
                table[index] + (table[index + 1] - table[index]) * fraction
            }
        }
    }
}

/// Flushes denormal all-pass memories every 32 samples (the stages
/// themselves skip the per-sample flush to keep the serial chain fast).
#[derive(Debug, Clone, Copy, Default)]
pub struct Denormals(u32);

impl Denormals {
    #[inline]
    pub fn sweep(&mut self, memories: &mut [f32]) {
        self.0 = self.0.wrapping_add(1);
        if self.0 & 31 == 0 {
            for memory in memories {
                *memory = flush(*memory);
            }
        }
    }
}

/// One first-order all-pass stage in the callbacks' form (memories are
/// flushed by `Denormals::sweep`).
#[inline]
pub fn allpass(memory: &mut f32, input: f32, w: f32) -> f32 {
    let z = input + w * *memory;
    let y = *memory - w * z;
    *memory = z;
    y
}

/// Output guard: transparent below ±8, saturating smoothly towards ±16. The
/// original's float path never clips, Main's fixed-point output does at ±1;
/// this only keeps extreme resonance settings inside the host's bounds.
#[inline]
pub fn guard(x: f32) -> f32 {
    let magnitude = x.abs();
    if magnitude <= 8.0 {
        x
    } else {
        let limited = 8.0 + 7.9 * crate::dsp::tanh((magnitude - 8.0) * 0.125);
        if x.is_finite() {
            limited.copysign(x)
        } else {
            0.0
        }
    }
}

/// Synced or free LFO frequency of a `Rate` knob with a note tail.
pub fn rate_hz(value: f32, threshold: u16, bpm: f32, free: impl Fn(f32) -> f32) -> f32 {
    if zoom::is_synced(value, threshold) {
        zoom::rate_note_hz(zoom::sync_index(value, threshold), bpm)
    } else {
        free(value.min(f32::from(threshold)))
    }
}

/// 101 labels "L50".."L1", "0", "R1".."R50" for AutoPan's Width.
pub const PAN_LABELS: [&str; 101] = [
    "L50", "L49", "L48", "L47", "L46", "L45", "L44", "L43", "L42", "L41", "L40", "L39", "L38",
    "L37", "L36", "L35", "L34", "L33", "L32", "L31", "L30", "L29", "L28", "L27", "L26", "L25",
    "L24", "L23", "L22", "L21", "L20", "L19", "L18", "L17", "L16", "L15", "L14", "L13", "L12",
    "L11", "L10", "L9", "L8", "L7", "L6", "L5", "L4", "L3", "L2", "L1", "0", "R1", "R2", "R3",
    "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15", "R16", "R17",
    "R18", "R19", "R20", "R21", "R22", "R23", "R24", "R25", "R26", "R27", "R28", "R29", "R30",
    "R31", "R32", "R33", "R34", "R35", "R36", "R37", "R38", "R39", "R40", "R41", "R42", "R43",
    "R44", "R45", "R46", "R47", "R48", "R49", "R50",
];

/// The tremolo wave names of `GetString_TrmWave`.
pub const TREMOLO_WAVES: [&str; 30] = [
    "UP 0", "UP 1", "UP 2", "UP 3", "UP 4", "UP 5", "UP 6", "UP 7", "UP 8", "UP 9", "DWN 0",
    "DWN 1", "DWN 2", "DWN 3", "DWN 4", "DWN 5", "DWN 6", "DWN 7", "DWN 8", "DWN 9", "TRI 0",
    "TRI 1", "TRI 2", "TRI 3", "TRI 4", "TRI 5", "TRI 6", "TRI 7", "TRI 8", "TRI 9",
];

/// Tremolo clip table (`Fx_MOD_Tremolo_clip_tbl`): Q31 words; the callback
/// multiplies the wave by `16 * entry` and saturates.
pub const TREMOLO_CLIP: [f32; 10] = [
    0.0625,
    0.069_306_25,
    0.089_725,
    0.123_756_25,
    0.1714,
    0.232_656_25,
    0.307_525,
    0.396_006_25,
    0.4981,
    0.625,
];

#[cfg(test)]
pub(crate) mod testing {
    /// RMS of a slice.
    pub fn rms(x: &[f32]) -> f32 {
        (x.iter().map(|v| v * v).sum::<f32>() / x.len().max(1) as f32).sqrt()
    }

    /// Count upward zero crossings of a signal's mean-removed copy.
    pub fn crossings(x: &[f32]) -> usize {
        let mean = x.iter().sum::<f32>() / x.len() as f32;
        x.windows(2)
            .filter(|w| w[0] - mean < 0.0 && w[1] - mean >= 0.0)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increments_round_trip() {
        // The Flanger editor's slowest free rate.
        assert!((increment_hz(4870.0) - 0.05).abs() < 1e-3);
        let inc = hz_increment(2.0, 48_000.0);
        assert!((f64::from(inc) * 48_000.0 / TURN - 2.0).abs() < 1e-4);
    }

    #[test]
    fn parabola_matches_the_callback_wave() {
        assert!((parabola(0.5) - 1.0).abs() < 1e-6);
        assert!((parabola(-0.5) + 1.0).abs() < 1e-6);
        assert_eq!(parabola(0.0), 0.0);
        assert!((parabola(0.25) - 0.75).abs() < 1e-6);
    }

    #[test]
    fn allpass_warp_keeps_the_90_degree_point() {
        let warp = AllpassWarp::new(96_000.0);
        for w in [0.2f32, 0.5, 0.9] {
            let t44 = (1.0 - w) / (1.0 + w);
            let hz = t44.atan() * 44_100.0 / std::f32::consts::PI;
            let w96 = warp.map(w);
            let t96 = (1.0 - w96) / (1.0 + w96);
            let hz96 = t96.atan() * 96_000.0 / std::f32::consts::PI;
            assert!((hz - hz96).abs() / hz < 0.01, "{w}: {hz} {hz96}");
        }
        assert_eq!(AllpassWarp::new(44_100.0).map(0.3), 0.3);
    }

    #[test]
    fn ramps_follow_main_timing() {
        let mut ramp = Ramp::with_duration(0.0, 705.6, 44_100.0);
        ramp.set(1.0);
        let mut n = 0;
        while !ramp.settled() {
            ramp.tick();
            n += 1;
        }
        assert!((705..=707).contains(&n), "{n}");
        let mut ramp = Ramp::with_step(0.0, zoom::DEFAULT_STEP, 88_200.0);
        ramp.set(1.0);
        let mut n = 0;
        while !ramp.settled() {
            ramp.tick();
            n += 1;
        }
        assert!((2110..=2120).contains(&n), "{n}");
    }

    #[test]
    fn rom_sections_keep_their_response() {
        // The Flanger's 300 Hz input high-pass from its ROM words.
        let rom = rom_first_order([0.979_072_7, -0.979_072_7, 0.958_145_4], 96_000.0);
        let direct = Coefficients::highpass1(96_000.0, 300.0);
        for hz in [50.0, 300.0, 3_000.0] {
            let a = rom.magnitude(96_000.0, hz);
            let b = direct.magnitude(96_000.0, hz);
            assert!((a - b).abs() < 0.01, "{hz}: {a} {b}");
        }
        let same = rom_first_order([0.5, 0.25, 0.1], 44_100.0);
        assert!((same.b0 - 0.5).abs() < 1e-6 && (same.a1 + 0.1).abs() < 1e-6);
    }

    #[test]
    fn guard_is_transparent_and_bounded() {
        assert_eq!(guard(3.0), 3.0);
        assert!(guard(1e9) < 16.0 && guard(-1e9) > -16.0);
        assert_eq!(guard(f32::NAN), 0.0);
    }
}
