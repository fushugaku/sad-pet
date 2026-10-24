//! Shared pieces of the HD delay group: Main's biquad designer (`context[25]`,
//! c00bcb20 → c00bc2e0), ROM biquads at the host rate, the editors' helpers
//! (`TBL_TO_VAL`, the delay-time parameter, duration ramps), the Q31 delay-time
//! glide every callback runs, the two-tap crossfade of DualDigiD/FLTR PPD, the
//! mute → swap → unmute sequence the editors use for stepped filter tables and
//! a direct-form-1 biquad cascade with flat coefficient storage.
use crate::{
    dsp::Coefficients,
    zoom::{ORIGINAL_RATE, Ramp, delay_note_ms},
};

/// Q31 → float.
pub const Q31: f32 = 1.0 / 2_147_483_648.0;

/// The duration (44.1 kHz samples) editors pass to Main's `context[7]` ramp:
/// 705.6 samples = 16 ms.
pub const RAMP_SAMPLES: f32 = 705.6;

/// Filter types of the Main designer (`B8` of `context[25]`), from its jump
/// table at c00ef0f0.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Lowpass1 = 0,
    Highpass1 = 1,
    Lowpass2 = 2,
    /// First-order high shelf; a cut moves the corner down by the cut's ratio.
    HighShelf1 = 8,
    /// First-order low shelf; a cut moves the corner up by the cut's ratio.
    LowShelf1 = 9,
}

/// `dsp::DelayLine` with the same power-of-two layout and arithmetic, but
/// with the ring index masked into a buffer whose length is fixed at
/// construction, so the per-sample accesses need no bounds checks.
#[derive(Debug, Clone)]
pub struct DelayLine {
    buffer: Box<[f32]>,
    mask: usize,
    write: usize,
}

impl DelayLine {
    /// Holds at least `max_delay` samples of history plus interpolation room.
    pub fn new(max_delay: usize) -> Self {
        let length = (max_delay + 4).next_power_of_two();
        Self {
            buffer: vec![0.0; length].into_boxed_slice(),
            mask: length - 1,
            write: 0,
        }
    }
    /// For a maximum delay in seconds at `sample_rate`.
    pub fn with_seconds(seconds: f32, sample_rate: f32) -> Self {
        Self::new((seconds * sample_rate).ceil() as usize + 1)
    }
    #[inline]
    pub fn capacity(&self) -> usize {
        self.mask - 3
    }
    #[inline]
    fn at(&self, index: usize) -> f32 {
        debug_assert!(self.buffer.len() == self.mask + 1);
        // SAFETY: `index & mask < buffer.len()`, a power of two fixed in `new`.
        unsafe { *self.buffer.get_unchecked(index & self.mask) }
    }
    #[inline]
    pub fn write(&mut self, x: f32) {
        debug_assert!(self.buffer.len() == self.mask + 1);
        self.write = (self.write + 1) & self.mask;
        // SAFETY: `write <= mask < buffer.len()`.
        unsafe { *self.buffer.get_unchecked_mut(self.write) = x };
    }
    #[inline]
    pub fn read(&self, delay: usize) -> f32 {
        self.at(self.write.wrapping_sub(delay))
    }
    /// Linear interpolation, `delay` clamped to `[0, capacity]`.
    #[inline]
    pub fn read_linear(&self, delay: f32) -> f32 {
        let delay = delay.clamp(0.0, self.capacity() as f32);
        let whole = delay as usize;
        let fraction = delay - whole as f32;
        let a = self.read(whole);
        let b = self.read(whole + 1);
        a + (b - a) * fraction
    }
    /// `read_linear(delay)` split into its whole and fractional parts once,
    /// for a delay that does not change.
    pub fn fixed(&self, delay: f32) -> (usize, f32) {
        let delay = delay.clamp(0.0, self.capacity() as f32);
        let whole = delay as usize;
        (whole, delay - whole as f32)
    }
    /// `read_linear` at a delay prepared by [`DelayLine::fixed`].
    #[inline]
    pub fn read_fixed(&self, (whole, fraction): (usize, f32)) -> f32 {
        let a = self.read(whole);
        let b = self.read(whole + 1);
        a + (b - a) * fraction
    }
    /// `blend · read_linear(taps[0]) + (1 - blend) · read_linear(taps[1])`
    /// (the two-tap crossfade), skipping the silent tap once a fade is over:
    /// with a zero weight it adds only a signed zero, which leaves a nonzero
    /// sum unchanged, so the result is identical.
    #[inline]
    pub fn read_crossfade(&self, blend: f32, taps: [f32; 2]) -> f32 {
        if blend == 1.0 {
            let a = self.read_linear(taps[0]);
            if a != 0.0 {
                return a;
            }
        } else if blend == 0.0 {
            let b = self.read_linear(taps[1]);
            if b != 0.0 {
                return b;
            }
        }
        blend * self.read_linear(taps[0]) + (1.0 - blend) * self.read_linear(taps[1])
    }
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

/// Fixed delay lines that are all written exactly once per sample, side by
/// side in one buffer with one shared write position. Each lane is a
/// power-of-two ring, so a lane behaves exactly like its own `DelayLine`.
#[derive(Debug, Clone)]
pub struct Bank {
    buffer: Box<[f32]>,
    position: usize,
    wrap: usize,
}

/// One ring of a [`Bank`].
#[derive(Debug, Clone, Copy)]
pub struct Lane {
    offset: usize,
    mask: usize,
}

impl Bank {
    /// One lane per entry, each holding at least `max_delay` samples of
    /// history (sized like `DelayLine::new`).
    pub fn new<const N: usize>(max_delays: [usize; N]) -> (Self, [Lane; N]) {
        let mut offset = 0;
        let lanes = max_delays.map(|d| {
            let length = (d + 4).next_power_of_two();
            let lane = Lane {
                offset,
                mask: length - 1,
            };
            offset += length;
            lane
        });
        let wrap = lanes.iter().map(|l| l.mask).max().unwrap_or(0);
        let bank = Self {
            buffer: vec![0.0; offset].into_boxed_slice(),
            position: 0,
            wrap,
        };
        (bank, lanes)
    }
    /// Moves every lane to the next sample; call once before the sample's
    /// reads and writes.
    #[inline]
    pub fn advance(&mut self) {
        self.position = (self.position + 1) & self.wrap;
    }
    #[inline]
    fn index(&self, lane: Lane, position: usize) -> usize {
        let index = lane.offset + (position & lane.mask);
        debug_assert!(lane.mask <= self.wrap && index < self.buffer.len());
        index
    }
    /// Writes this sample's value of `lane`.
    #[inline]
    pub fn write(&mut self, lane: Lane, x: f32) {
        let index = self.index(lane, self.position);
        // SAFETY: lanes come from `new` and tile the buffer; the position is
        // masked into the lane.
        unsafe { *self.buffer.get_unchecked_mut(index) = x };
    }
    /// The value of `lane` written `ago` samples before this one (0 = this
    /// sample's write).
    #[inline]
    pub fn ago(&self, lane: Lane, ago: usize) -> f32 {
        let index = self.index(lane, self.position.wrapping_sub(ago));
        // SAFETY: as in `write`.
        unsafe { *self.buffer.get_unchecked(index) }
    }
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
    }
}

/// `Ramp::tick` with a fast path once the ramp has arrived: a settled
/// `tick` returns `target` and changes nothing else, so this is identical.
pub trait Settled {
    fn step(&mut self) -> f32;
}

impl Settled for Ramp {
    #[inline]
    fn step(&mut self) -> f32 {
        if self.current == self.target {
            self.target
        } else {
            self.tick()
        }
    }
}

/// `dsp::flush` (zero below 1e-20 in magnitude and for non-finite values)
/// as one range test on the magnitude bits: identical results.
#[inline]
pub fn flush(x: f32) -> f32 {
    const LOW: u32 = 1e-20f32.to_bits();
    const INFINITY: u32 = f32::INFINITY.to_bits();
    let magnitude = x.to_bits() & 0x7fff_ffff;
    if magnitude.wrapping_sub(LOW) < INFINITY - LOW {
        x
    } else {
        0.0
    }
}

/// Main c00bc2e0 (`A4`=out, `B4`=Hz, `A6`=gain dB, `A8`=Q, `B8`=type) at the
/// host rate: `K = tan(π f / fs)` and the original per-type formulas
/// (adapted from the dynamics/EQ group's port of the same service).
pub fn design(kind: Kind, sample_rate: f32, hz: f32, gain_db: f32, q: f32) -> Coefficients {
    let fs = f64::from(sample_rate);
    let tan = |hz: f64| (std::f64::consts::PI * hz.clamp(1e-3, fs * 0.49) / fs).tan();
    let hz = f64::from(hz);
    let g = f64::from(gain_db);
    let q = f64::from(q).max(1e-3);
    let k = tan(hz);
    // (b0, b1, b2, fb1, fb2)
    let (b0, b1, b2, fb1, fb2) = match kind {
        Kind::Lowpass1 => {
            let n = 1.0 / (1.0 + k);
            (k * n, k * n, 0.0, (1.0 - k) * n, 0.0)
        }
        Kind::Highpass1 => {
            let n = 1.0 / (1.0 + k);
            (n, -n, 0.0, (1.0 - k) * n, 0.0)
        }
        Kind::Lowpass2 => {
            let k2 = k * k;
            let n = 1.0 / (1.0 + k / q + k2);
            (
                k2 * n,
                2.0 * k2 * n,
                k2 * n,
                -2.0 * (k2 - 1.0) * n,
                -(1.0 - k / q + k2) * n,
            )
        }
        Kind::HighShelf1 => {
            let gain = 10f64.powf(g / 20.0);
            let k = if g >= 0.0 { k } else { tan(hz * gain) };
            let n = 1.0 / (1.0 + k);
            ((gain + k) * n, (k - gain) * n, 0.0, (1.0 - k) * n, 0.0)
        }
        Kind::LowShelf1 => {
            let gain = 10f64.powf(g / 20.0);
            let k = if g >= 0.0 { k } else { tan(hz / gain) };
            let n = 1.0 / (1.0 + k);
            (
                (1.0 + k * gain) * n,
                (k * gain - 1.0) * n,
                0.0,
                (1.0 - k) * n,
                0.0,
            )
        }
    };
    Coefficients {
        b0: b0 as f32,
        b1: b1 as f32,
        b2: b2 as f32,
        a1: -fb1 as f32,
        a2: -fb2 as f32,
    }
}

/// A ROM biquad in the original word order `b0, b1, b2, fb1, fb2`
/// (`y = b0 x + b1 x1 + b2 x2 + fb1 y1 + fb2 y2`), designed for `from_rate`
/// and re-derived for `sample_rate` through its analog prototype.
pub fn rom_at(words: [f32; 5], from_rate: f32, sample_rate: f32) -> Coefficients {
    Coefficients {
        b0: words[0],
        b1: words[1],
        b2: words[2],
        a1: -words[3],
        a2: -words[4],
    }
    .resample(from_rate, sample_rate)
}

/// A 44.1 kHz ROM biquad at the host rate.
pub fn rom(words: [f32; 5], sample_rate: f32) -> Coefficients {
    rom_at(words, ORIGINAL_RATE, sample_rate)
}

/// A per-sample multiplier recovered at `from_rate` at the host rate, keeping
/// its time constant.
pub fn pole_at(coefficient: f32, from_rate: f32, sample_rate: f32) -> f32 {
    f64::from(coefficient).powf(f64::from(from_rate) / f64::from(sample_rate)) as f32
}

/// A 44.1 kHz per-sample multiplier at the host rate.
pub fn pole(coefficient: f32, sample_rate: f32) -> f32 {
    pole_at(coefficient, ORIGINAL_RATE, sample_rate)
}

/// A one-pole smoother `y = gain·x + pole·y` recovered at `from_rate`, at the
/// host rate with the same time constant and DC gain: (gain, pole).
pub fn one_pole_at(gain: f32, pole: f32, from_rate: f32, sample_rate: f32) -> (f32, f32) {
    let new_pole = pole_at(pole, from_rate, sample_rate);
    (gain * (1.0 - new_pole) / (1.0 - pole), new_pole)
}

/// A 44.1 kHz sample count at the host rate.
#[inline]
pub fn samples(samples_at_44k: f32, sample_rate: f32) -> f32 {
    samples_at_44k * sample_rate / ORIGINAL_RATE
}

/// The modules' `TBL_TO_VAL(count, value, table, n)`: `value` over
/// `0..count-1` linearly interpolated across the `n` table points.
pub fn table(points: &[f32], value: f32, count: u32) -> f32 {
    let last = points.len() - 1;
    let position = (value.max(0.0) * last as f32 / (count - 1) as f32).min(last as f32);
    let index = (position as usize).min(last.saturating_sub(1));
    let fraction = position - index as f32;
    if last == 0 {
        return points[0];
    }
    points[index] + (points[index + 1] - points[index]) * fraction
}

/// Linear interpolation of a table with one point per `step` parameter units
/// (the DualDigiD mix tables).
pub fn table_step(points: &[f32], value: f32, step: f32) -> f32 {
    let last = points.len() - 1;
    let position = (value.max(0.0) / step).min(last as f32);
    let index = (position as usize).min(last.saturating_sub(1));
    points[index] + (points[index + 1] - points[index]) * (position - index as f32)
}

/// A delay-time parameter: `0..=threshold` is `value + offset` ms (each
/// module's `CalcDelayTime`), larger values are the tempo notes of Main
/// c00d7bc0 limited (halved) to `maximum_ms`.
pub fn time_ms(value: f32, threshold: u16, offset: f32, maximum_ms: f32, bpm: f32) -> f32 {
    let value = value.max(0.0);
    if value.round() <= f32::from(threshold) {
        (value + offset).min(maximum_ms)
    } else {
        let index = (value.round() as usize).saturating_sub(usize::from(threshold) + 1);
        delay_note_ms(index, bpm, maximum_ms)
    }
}

/// Main's `context[7]` setter: the coefficient moves linearly to each new
/// target in a fixed number of samples (705.6 at 44.1 kHz in every editor).
#[derive(Debug, Clone, Copy)]
pub struct DurationRamp {
    pub current: f32,
    pub target: f32,
    step: f32,
    duration: f32,
}

impl DurationRamp {
    pub fn new(value: f32, samples_at_44k: f32, sample_rate: f32) -> Self {
        Self {
            current: value,
            target: value,
            step: 0.0,
            duration: samples(samples_at_44k, sample_rate).max(1.0),
        }
    }
    pub fn set(&mut self, target: f32) {
        if target != self.target {
            self.target = target;
            self.step = ((target - self.current).abs() / self.duration).max(1e-6 * target.abs());
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
            } else {
                self.current += self.step.copysign(difference);
            }
        }
        self.current
    }
}

/// The Q31 one-pole every callback runs on its delay time
/// (`time = c·target + p·time`, 0x00061546 / 0x7ff9eab8 at 44.1 kHz, a 0.12 s
/// time constant): a knob or tempo change glides the read position.
#[derive(Debug, Clone, Copy)]
pub struct Glide {
    pub value: f32,
    pub target: f32,
    gain: f32,
    pole: f32,
}

impl Glide {
    pub fn new(value: f32, gain_q31: u32, pole_q31: u32, sample_rate: f32) -> Self {
        let (gain, pole) = one_pole_at(
            gain_q31 as f32 * Q31,
            pole_q31 as f32 * Q31,
            ORIGINAL_RATE,
            sample_rate,
        );
        Self {
            value,
            target: value,
            gain,
            pole,
        }
    }
    /// The common 0x00061546 / 0x7ff9eab8 pair.
    pub fn standard(value: f32, sample_rate: f32) -> Self {
        Self::new(value, 0x0006_1546, 0x7ff9_eab8, sample_rate)
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.value != self.target {
            self.value = self.gain * self.target + self.pole * self.value;
            if (self.value - self.target).abs() < 1e-4 {
                self.value = self.target;
            }
        }
        self.value
    }
}

/// The two-tap crossfade of DualDigiD and FLTR PPD: a new delay time goes to
/// the silent tap and the blend moves to it linearly in the XfdTime (200 ms).
/// The editors defer a change requested while a crossfade runs; it is applied
/// when the running one completes.
#[derive(Debug, Clone, Copy)]
pub struct CrossfadeTime {
    /// Delay of each tap in samples.
    pub taps: [f32; 2],
    /// Weight of `taps[0]`; `taps[1]` gets `1 - blend`.
    pub blend: f32,
    target: f32,
    step: f32,
    pending: Option<f32>,
}

impl CrossfadeTime {
    pub fn new(delay: f32, crossfade_ms: f32, sample_rate: f32) -> Self {
        Self {
            taps: [delay; 2],
            blend: 1.0,
            target: 1.0,
            step: 1.0 / (crossfade_ms * 0.001 * sample_rate).max(1.0),
            pending: None,
        }
    }
    pub fn request(&mut self, delay: f32) {
        if self.blend != self.target {
            self.pending = Some(delay);
            return;
        }
        self.pending = None;
        let active = if self.target > 0.5 { 0 } else { 1 };
        if (self.taps[active] - delay).abs() < 1e-3 {
            return;
        }
        self.taps[1 - active] = delay;
        self.target = 1.0 - self.target;
    }
    pub fn jump(&mut self, delay: f32) {
        self.taps = [delay; 2];
        self.blend = self.target;
        self.pending = None;
    }
    #[inline]
    pub fn tick(&mut self) -> f32 {
        if self.blend != self.target {
            if (self.target - self.blend).abs() <= self.step {
                self.blend = self.target;
                if let Some(delay) = self.pending.take() {
                    self.request(delay);
                }
            } else {
                self.blend += self.step.copysign(self.target - self.blend);
            }
        }
        self.blend
    }
}

/// The editors' mute → copy coefficients → unmute sequence for stepped filter
/// tables: a gain ramps to 0 in 16 ms, the new set is applied, and it ramps
/// back to 1.
#[derive(Debug, Clone, Copy)]
pub struct MuteSwap {
    pub gain: f32,
    step: f32,
    muting: bool,
}

impl MuteSwap {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            gain: 1.0,
            step: 1.0 / samples(RAMP_SAMPLES, sample_rate).max(1.0),
            muting: false,
        }
    }
    pub fn start(&mut self) {
        self.muting = true;
    }
    /// Returns the gain and whether the pending set must be applied now.
    #[inline]
    pub fn tick(&mut self) -> (f32, bool) {
        if self.muting {
            self.gain -= self.step;
            if self.gain <= 0.0 {
                self.gain = 0.0;
                self.muting = false;
                return (0.0, true);
            }
        } else if self.gain < 1.0 {
            self.gain = (self.gain + self.step).min(1.0);
        }
        (self.gain, false)
    }
}

/// A direct-form-1 biquad in the original word order with its own history,
/// used where the callbacks run long cascades.
#[derive(Debug, Clone, Copy, Default)]
pub struct Stage {
    pub c: [f32; 5],
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Stage {
    pub const IDENTITY: [f32; 5] = [1.0, 0.0, 0.0, 0.0, 0.0];
    pub fn new(c: [f32; 5]) -> Self {
        Self {
            c,
            ..Self::default()
        }
    }
    pub fn from(c: Coefficients) -> Self {
        Self::new(words(c))
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let c = &self.c;
        let y = c[0] * x + c[1] * self.x1 + c[2] * self.x2 + c[3] * self.y1 + c[4] * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = flush(y);
        self.y1
    }
    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

/// A first-order [`Stage`] (`b2 = fb2 = 0`, as every first-order design and
/// first-order ROM word set stays after resampling). The dropped terms only
/// add a signed zero, and a zero result is flushed to `+0.0` either way, so
/// the output is bit-identical to [`Stage::process`] for finite input.
#[derive(Debug, Clone, Copy, Default)]
pub struct Stage1 {
    b0: f32,
    b1: f32,
    fb1: f32,
    x1: f32,
    y1: f32,
}

impl Stage1 {
    /// From words in the original order; `c[2]` and `c[4]` must be zero.
    pub fn new(c: [f32; 5]) -> Self {
        let mut stage = Self::default();
        stage.load(c);
        stage
    }
    pub fn from(c: Coefficients) -> Self {
        Self::new(words(c))
    }
    /// Replaces the coefficients, keeping the history.
    pub fn load(&mut self, c: [f32; 5]) {
        assert!(c[2] == 0.0 && c[4] == 0.0, "not a first-order set: {c:?}");
        self.b0 = c[0];
        self.b1 = c[1];
        self.fb1 = c[3];
    }
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.fb1 * self.y1;
        self.x1 = x;
        self.y1 = flush(y);
        self.y1
    }
    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

/// A cascade of up to `N` biquads; identity stages of a ROM set are skipped.
#[derive(Debug, Clone, Copy)]
pub struct Cascade<const N: usize> {
    stages: [Stage; N],
    active: usize,
}

impl<const N: usize> Default for Cascade<N> {
    fn default() -> Self {
        Self {
            stages: [Stage::default(); N],
            active: 0,
        }
    }
}

impl<const N: usize> Cascade<N> {
    /// Loads a set (original word order, already at the host rate); histories
    /// of the loaded positions are kept.
    pub fn load(&mut self, set: &[[f32; 5]]) {
        let mut active = 0;
        for words in set.iter().filter(|w| **w != Stage::IDENTITY) {
            self.stages[active].c = *words;
            active += 1;
        }
        for stage in &mut self.stages[active..self.active.max(active)] {
            stage.clear();
        }
        self.active = active;
    }
    #[inline]
    pub fn process(&mut self, mut x: f32) -> f32 {
        for stage in &mut self.stages[..self.active] {
            x = stage.process(x);
        }
        x
    }
    pub fn clear(&mut self) {
        for stage in &mut self.stages {
            stage.clear();
        }
    }
}

/// Coefficients in the original word order.
pub fn words(c: Coefficients) -> [f32; 5] {
    [c.b0, c.b1, c.b2, -c.a1, -c.a2]
}

/// A set of ROM biquads at the host rate.
pub fn rom_set<const N: usize>(set: &[[f32; 5]; N], sample_rate: f32) -> [[f32; 5]; N] {
    let mut out = [[0.0; 5]; N];
    for (o, w) in out.iter_mut().zip(set) {
        *o = if *w == Stage::IDENTITY {
            Stage::IDENTITY
        } else {
            words(rom(*w, sample_rate))
        };
    }
    out
}

/// Safety limiter for feedback loops the original leaves unbounded at 100 %
/// feedback (no clip in the callback): transparent below ±2, saturating
/// smoothly toward ±4.
#[inline]
pub fn safety(x: f32) -> f32 {
    let a = x.abs();
    if a <= 2.0 {
        x
    } else {
        (2.0 + 2.0 * crate::dsp::tanh((a - 2.0) * 0.5)).copysign(x)
    }
}

/// The callbacks' clip to ±1 (feedback and SVF states).
#[inline]
pub fn clip1(x: f32) -> f32 {
    x.clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flush_matches_the_shared_one() {
        let probes = [
            0.0,
            -0.0,
            1e-20,
            -1e-20,
            f32::from_bits(1e-20f32.to_bits() - 1),
            f32::from_bits(1e-20f32.to_bits() + 1),
            1e-30,
            0.5,
            -3.0,
            f32::MAX,
            f32::MIN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
            f32::MIN_POSITIVE,
            f32::from_bits(1),
        ];
        for x in probes {
            assert_eq!(flush(x).to_bits(), crate::dsp::flush(x).to_bits(), "{x}");
        }
    }

    #[test]
    fn table_to_value_interpolates_like_the_modules() {
        let points = [0.0, 1.0, 3.0];
        assert_eq!(table(&points, 0.0, 101), 0.0);
        assert_eq!(table(&points, 100.0, 101), 3.0);
        assert!((table(&points, 25.0, 101) - 0.5).abs() < 1e-6);
        assert!((table(&points, 75.0, 101) - 2.0).abs() < 1e-6);
    }

    #[test]
    fn shelf_designs_have_their_gains() {
        let c = design(Kind::HighShelf1, 44_100.0, 10_992.0, -21.0, 0.0);
        assert!((c.magnitude(44_100.0, 20.0) - 1.0).abs() < 0.01);
        assert!(crate::dsp::gain_to_db(c.magnitude(44_100.0, 22_000.0)) < -20.0);
        let c = design(Kind::LowShelf1, 48_000.0, 480.0, -12.0, 0.8);
        assert!((crate::dsp::gain_to_db(c.magnitude(48_000.0, 5.0)) + 12.0).abs() < 0.2);
        assert!((c.magnitude(48_000.0, 20_000.0) - 1.0).abs() < 0.02);
    }

    #[test]
    fn crossfade_moves_to_the_new_tap_and_defers_changes() {
        let mut x = CrossfadeTime::new(100.0, 200.0, 1000.0);
        x.request(50.0);
        assert_eq!(x.taps, [100.0, 50.0]);
        x.request(70.0);
        for _ in 0..205 {
            x.tick();
        }
        // first fade done; the deferred one starts on the other tap
        assert_eq!(x.taps, [70.0, 50.0]);
        for _ in 0..205 {
            x.tick();
        }
        assert_eq!(x.blend, 1.0);
    }
}
