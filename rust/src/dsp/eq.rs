//! How a Rack's EQ modules sound (see `crate::eq`).
//!
//! A band is up to four second-order sections and one first-order section.
//! Each section takes its poles from the analog prototype by impulse
//! invariance, and its zeros are fitted so that its magnitude matches the
//! analog one at 0 Hz, at the band's frequency and at Nyquist (Vicanek,
//! "Matched Second Order Digital Filters", 2016). A bell or shelf near the
//! top therefore keeps its analog shape, where the bilinear transform would
//! squeeze it towards Nyquist.
//!
//! Every second-order section runs as a trapezoidal state-variable filter
//! (Simper), and its three outputs are mixed to give that response. The
//! filter stays stable however fast modulation moves it. Its coefficients
//! glide across each control block toward settings that are themselves
//! smoothed over a few milliseconds, so a turned knob never clicks.
use super::{CONTROL_INTERVAL, Stereo, bits};
use crate::eq::{BANDS, Band, Equalizer, Kind, MAX_EQS, Placement};
use std::f64::consts::{FRAC_1_SQRT_2, PI};

/// Second-order sections of a band: four make a 48 dB/oct cut.
const SECTIONS: usize = 4;
const RAMP: u32 = CONTROL_INTERVAL as u32;
/// How long the settings take to settle, in seconds (time constant).
const SMOOTHING: f64 = 0.005;
/// Below these differences (log2 Hz, dB, log2 Q), a setting has arrived.
const SETTLED: [f32; 3] = [1e-4, 1e-3, 1e-4];
/// Poles stay under Nyquist, where impulse invariance would fold them back.
const HIGHEST_POLE: f64 = 0.98 * PI;
/// A band's frequency stays under Nyquist too.
const HIGHEST_BAND: f64 = 0.95 * PI;

/// A second-order section's transfer function, normalized so a0 = 1.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Biquad {
    b: [f64; 3],
    a: [f64; 2],
}

/// sin²(w/2) and the terms the magnitude of a biquad is built from:
/// |H(w)|² = (B0 φ0 + B1 φ1 + B2 φ2) / (A0 φ0 + A1 φ1 + A2 φ2).
fn phis(w: f64) -> [f64; 3] {
    let s = (w * 0.5).sin();
    let phi1 = s * s;
    let phi0 = 1.0 - phi1;
    [phi0, phi1, 4.0 * phi0 * phi1]
}

/// The A terms of a denominator 1 + a1 z⁻¹ + a2 z⁻².
fn squares([a1, a2]: [f64; 2]) -> [f64; 3] {
    [(1.0 + a1 + a2).powi(2), (1.0 - a1 + a2).powi(2), -4.0 * a2]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// The impulse-invariant poles of s² + (w/q) s + w², w in radians per sample.
fn poles(w: f64, q: f64) -> [f64; 2] {
    let w = w.min(HIGHEST_POLE);
    let zeta = 0.5 / q;
    let r = (-zeta * w).exp();
    let a1 = if zeta <= 1.0 {
        -2.0 * r * ((1.0 - zeta * zeta).sqrt() * w).cos()
    } else {
        -2.0 * r * ((zeta * zeta - 1.0).sqrt() * w).cosh()
    };
    [a1, r * r]
}

/// |H(jx)|² of the analog section (n0 + n1 s + n2 s²) / (d0 + d1 s + d2 s²).
fn analog(n: [f64; 3], d: [f64; 3], x: f64) -> f64 {
    let x2 = x * x;
    ((n[0] - n[2] * x2).powi(2) + (n[1] * x).powi(2))
        / ((d[0] - d[2] * x2).powi(2) + (d[1] * x).powi(2))
}

/// Zeros for the poles `a` whose magnitude squared is `h0` at 0, `hw` at `w`
/// and `hpi` at Nyquist.
fn matched(a: [f64; 2], w: f64, [h0, hw, hpi]: [f64; 3]) -> Biquad {
    let squares = squares(a);
    let phi = phis(w);
    let b0 = squares[0] * h0;
    let b1 = squares[1] * hpi;
    let b2 = (dot(squares, phi) * hw - b0 * phi[0] - b1 * phi[1]) / phi[2];
    let (s0, s1) = (b0.sqrt(), b1.sqrt());
    let half = 0.5 * (s0 + s1);
    let first = 0.5 * (half + (half * half + b2).max(0.0).sqrt());
    Biquad {
        b: [first, 0.5 * (s0 - s1), half - first],
        a,
    }
}

/// An analog section matched at 0, `w` and Nyquist, its poles at `wp` with `qp`.
fn fit(n: [f64; 3], d: [f64; 3], w: f64, [wp, qp]: [f64; 2]) -> Biquad {
    matched(
        poles(wp, qp),
        w,
        [analog(n, d, 0.0), analog(n, d, 1.0), analog(n, d, PI / w)],
    )
}

/// A bell of `gain` dB, its bandwidth set by `q`; cuts mirror boosts.
fn bell(w: f64, gain: f64, q: f64) -> Biquad {
    let a = 10f64.powf(gain / 40.0);
    fit([1.0, a / q, 1.0], [1.0, 1.0 / (a * q), 1.0], w, [w, a * q])
}

/// Shelves of `gain` dB, at half that gain at `w`.
fn low_shelf(w: f64, gain: f64, q: f64) -> Biquad {
    let a = 10f64.powf(gain / 40.0);
    let root = a.sqrt();
    fit(
        [a * a, a * root / q, a],
        [1.0, root / q, a],
        w,
        [w / root, q],
    )
}
fn high_shelf(w: f64, gain: f64, q: f64) -> Biquad {
    let a = 10f64.powf(gain / 40.0);
    let root = a.sqrt();
    fit(
        [a, a * root / q, a * a],
        [a, root / q, 1.0],
        w,
        [w * root, q],
    )
}

/// Unity gain at `w`, nothing at 0 Hz.
fn band_pass(w: f64, q: f64) -> Biquad {
    fit([0.0, 1.0 / q, 0.0], [1.0, 1.0 / q, 1.0], w, [w, q])
}

/// Zeros on the unit circle at `w`: nothing passes there at all.
fn notch(w: f64, q: f64) -> Biquad {
    let a = poles(w, q);
    let c = (1.0 + a[0] + a[1]) / (2.0 - 2.0 * w.cos());
    Biquad {
        b: [c, -2.0 * c * w.cos(), c],
        a,
    }
}

/// Vicanek's matched low pass: unity at 0 Hz and `q` at `w`.
fn low_pass(w: f64, q: f64) -> Biquad {
    let a = poles(w, q);
    let squares = squares(a);
    let phi = phis(w);
    let b0 = squares[0];
    let b1 = ((dot(squares, phi) * q * q - b0 * phi[0]) / phi[1]).max(0.0);
    let first = 0.5 * (b0.sqrt() + b1.sqrt());
    Biquad {
        b: [first, b0.sqrt() - first, 0.0],
        a,
    }
}

/// Vicanek's matched high pass: two zeros at 0 Hz and `q` at `w`.
fn high_pass(w: f64, q: f64) -> Biquad {
    let a = poles(w, q);
    let phi = phis(w);
    let b = dot(squares(a), phi).sqrt() * q / (4.0 * phi[1]);
    Biquad {
        b: [b, -2.0 * b, b],
        a,
    }
}

/// A first-order section, b0 + b1 z⁻¹ over 1 − p z⁻¹, matched at 0 Hz and
/// at Nyquist to the analog one-pole cornered at `w`.
fn one_pole(w: f64, high: bool) -> [f64; 3] {
    let w = w.min(HIGHEST_POLE);
    let p = (-w).exp();
    let ratio = PI / w;
    if high {
        let c = 0.5 * (1.0 + p) * ratio / (1.0 + ratio * ratio).sqrt();
        [c, -c, p]
    } else {
        let h = 1.0 / (1.0 + ratio * ratio).sqrt();
        [
            0.5 * ((1.0 - p) + (1.0 + p) * h),
            0.5 * ((1.0 - p) - (1.0 + p) * h),
            p,
        ]
    }
}

/// The state-variable filter whose mixed outputs m0 v0 + m1 v1 + m2 v2 have
/// the section's response: [g, k, m0, m1, m2].
fn state_variable(section: Biquad) -> [f32; 5] {
    let ([b0, b1, b2], [a1, a2]) = (section.b, section.a);
    let g = ((1.0 + a1 + a2) / (1.0 - a1 + a2)).sqrt();
    let k = 2.0 * (1.0 - a2) / (g * (1.0 - a1 + a2));
    let d = 1.0 + g * (g + k);
    let m0 = (b0 - b1 + b2) / (1.0 - a1 + a2);
    let m2 = d * (b1 - m0 * a1) / (2.0 * g * g);
    let m1 = d * (b0 - m0) / g - m2 * g;
    [g, k, m0, m1, m2].map(|x| x as f32)
}

/// What a band's sections become: None passes the signal unchanged.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Design {
    sections: [Option<Biquad>; SECTIONS],
    pole: Option<[f64; 3]>,
}

/// A band's sections at `frequency` Hz, `gain` dB and `q`, at `rate` Hz.
fn design(kind: Kind, slope: u8, frequency: f64, gain: f64, q: f64, rate: f64) -> Design {
    let w = (2.0 * PI * frequency / rate).min(HIGHEST_BAND);
    let mut design = Design::default();
    let one = |section| [Some(section), None, None, None];
    match kind {
        Kind::Bell => design.sections = one(bell(w, gain, q)),
        Kind::LowShelf => design.sections = one(low_shelf(w, gain, q)),
        Kind::HighShelf => design.sections = one(high_shelf(w, gain, q)),
        Kind::Notch => design.sections = one(notch(w, q)),
        Kind::BandPass => design.sections = one(band_pass(w, q)),
        Kind::LowCut | Kind::HighCut => {
            // A Butterworth cascade; Q raises the sharpest section's peak.
            let high = kind == Kind::LowCut;
            let order = usize::from(slope / 6).clamp(1, 2 * SECTIONS);
            let pairs = order / 2;
            for k in 0..pairs {
                let angle = if order % 2 == 0 {
                    (2 * k + 1) as f64 * PI / (2 * order) as f64
                } else {
                    (k + 1) as f64 * PI / order as f64
                };
                let mut section_q = 0.5 / angle.cos();
                if k + 1 == pairs {
                    section_q *= q / FRAC_1_SQRT_2;
                }
                design.sections[k] = Some(if high {
                    high_pass(w, section_q)
                } else {
                    low_pass(w, section_q)
                });
            }
            if order % 2 == 1 {
                design.pole = Some(one_pole(w, high));
            }
        }
    }
    design
}

/// Coefficients now, where they go, and their step per frame on the way.
#[derive(Clone, Copy, Debug)]
struct Glide<const N: usize> {
    now: [f32; N],
    target: [f32; N],
    step: [f32; N],
}
impl<const N: usize> Default for Glide<N> {
    fn default() -> Self {
        Self {
            now: [0.0; N],
            target: [0.0; N],
            step: [0.0; N],
        }
    }
}
impl<const N: usize> Glide<N> {
    fn aim(&mut self, target: [f32; N], snap: bool) {
        self.target = target;
        if snap {
            self.land();
        } else {
            self.step = std::array::from_fn(|i| (target[i] - self.now[i]) / RAMP as f32);
        }
    }
    #[inline(always)]
    fn advance(&mut self) {
        for i in 0..N {
            self.now[i] += self.step[i];
        }
    }
    fn land(&mut self) {
        self.now = self.target;
        self.step = [0.0; N];
    }
}

const IDENTITY_POLE: [f32; 3] = [1.0, 0.0, 0.0];

#[derive(Clone, Copy, Debug, Default)]
struct Section {
    /// g, k, m0, m1, m2.
    glide: Glide<5>,
    /// Simper's a1, a2, a3 for the current g and k.
    a: [f32; 3],
    /// Each channel's two integrator states.
    state: [[f32; 2]; 2],
    /// It filters: off, the signal passes it untouched.
    active: bool,
    /// Gliding to a straight pass, after which it rests.
    retiring: bool,
}
impl Section {
    fn refresh(&mut self) {
        let [g, k, ..] = self.glide.now;
        let a1 = 1.0 / (1.0 + g * (g + k));
        self.a = [a1, g * a1, g * g * a1];
    }
    fn aim(&mut self, target: Option<[f32; 5]>, snap: bool) {
        match target {
            Some(target) => {
                if !self.active {
                    // It starts as a straight pass at its new g and k, so
                    // its empty state takes nothing from the signal.
                    self.state = [[0.0; 2]; 2];
                    self.glide.now = [target[0], target[1], 1.0, 0.0, 0.0];
                    self.active = true;
                }
                self.retiring = false;
                self.glide.aim(target, snap);
            }
            None if self.active => {
                let [g, k, ..] = self.glide.target;
                self.retiring = !snap;
                self.active = !snap;
                self.glide.aim([g, k, 1.0, 0.0, 0.0], snap);
            }
            None => {}
        }
        self.refresh();
    }
    fn land(&mut self) {
        self.glide.land();
        self.refresh();
        if self.retiring {
            self.retiring = false;
            self.active = false;
        }
    }
    #[inline(always)]
    fn tick(&mut self, channel: usize, x: f32) -> f32 {
        let [a1, a2, a3] = self.a;
        let [_, _, m0, m1, m2] = self.glide.now;
        let state = &mut self.state[channel];
        let v3 = x - state[1];
        let v1 = a1 * state[0] + a2 * v3;
        let v2 = state[1] + a2 * state[0] + a3 * v3;
        state[0] = 2.0 * v1 - state[0];
        state[1] = 2.0 * v2 - state[1];
        m0 * x + m1 * v1 + m2 * v2
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Pole {
    /// b0, b1, p.
    glide: Glide<3>,
    state: [f32; 2],
    active: bool,
    retiring: bool,
}
impl Pole {
    fn aim(&mut self, target: Option<[f32; 3]>, snap: bool) {
        match target {
            Some(target) => {
                if !self.active {
                    self.state = [0.0; 2];
                    self.glide.now = IDENTITY_POLE;
                    self.active = true;
                }
                self.retiring = false;
                self.glide.aim(target, snap);
            }
            None if self.active => {
                self.retiring = !snap;
                self.active = !snap;
                self.glide.aim(IDENTITY_POLE, snap);
            }
            None => {}
        }
    }
    fn land(&mut self) {
        self.glide.land();
        if self.retiring {
            self.retiring = false;
            self.active = false;
        }
    }
    #[inline(always)]
    fn tick(&mut self, channel: usize, x: f32) -> f32 {
        let [b0, b1, p] = self.glide.now;
        let y = b0 * x + self.state[channel];
        self.state[channel] = b1 * x + p * y;
        y
    }
}

/// The filter channels a placement runs: mid and side take the first.
fn channels(placement: Placement) -> [bool; 2] {
    match placement {
        Placement::Stereo => [true, true],
        Placement::Right => [false, true],
        _ => [true, false],
    }
}

/// The settings a band's coefficients were designed for.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Key {
    enabled: bool,
    kind: Kind,
    slope: u8,
    smooth: [f32; 3],
}

#[derive(Clone, Copy, Debug, Default)]
struct BandState {
    sections: [Section; SECTIONS],
    pole: Pole,
    placement: Placement,
    /// log2 Hz, dB and log2 Q, on their way to the band's own.
    smooth: [f32; 3],
    key: Option<Key>,
}
impl BandState {
    /// Follows the band's settings for another control block; true when the
    /// coefficients start to glide.
    fn retarget(&mut self, band: &Band, rate: f64, blend: f32, snap: bool) -> bool {
        let target = [band.frequency.log2(), band.gain, band.q.log2()];
        for i in 0..3 {
            let distance = target[i] - self.smooth[i];
            self.smooth[i] = if snap || distance.abs() < SETTLED[i] {
                target[i]
            } else {
                self.smooth[i] + distance * blend
            };
        }
        if band.placement != self.placement {
            // A channel the band takes up continues from the one it used.
            let (before, after) = (channels(self.placement), channels(band.placement));
            for c in 0..2 {
                if after[c] && !before[c] {
                    for section in &mut self.sections {
                        section.state[c] = section.state[1 - c];
                    }
                    self.pole.state[c] = self.pole.state[1 - c];
                }
            }
            self.placement = band.placement;
        }
        let key = Key {
            enabled: band.enabled,
            kind: band.kind,
            slope: band.slope,
            smooth: self.smooth,
        };
        if !snap && self.key == Some(key) {
            return false;
        }
        self.key = Some(key);
        let design = if band.enabled {
            design(
                band.kind,
                band.slope,
                f64::from(self.smooth[0].exp2()),
                f64::from(self.smooth[1]),
                f64::from(self.smooth[2].exp2()),
                rate,
            )
        } else {
            Design::default()
        };
        for (section, target) in self.sections.iter_mut().zip(design.sections) {
            section.aim(target.map(state_variable), snap);
        }
        self.pole
            .aim(design.pole.map(|p| p.map(|x| x as f32)), snap);
        true
    }
    fn land(&mut self) {
        for section in &mut self.sections {
            section.land();
        }
        self.pole.land();
    }
    fn live(&self) -> bool {
        self.pole.active || self.sections.iter().any(|s| s.active)
    }
    /// The coefficients take another step of their glide.
    #[inline]
    fn advance(&mut self) {
        for section in self.sections.iter_mut().filter(|s| s.active) {
            section.glide.advance();
            section.refresh();
        }
        if self.pole.active {
            self.pole.glide.advance();
        }
    }
    #[inline]
    fn run(&mut self, channel: usize, mut x: f32) -> f32 {
        for section in self.sections.iter_mut().filter(|s| s.active) {
            x = section.tick(channel, x);
        }
        if self.pole.active {
            x = self.pole.tick(channel, x);
        }
        x
    }
    #[inline]
    fn process(&mut self, [left, right]: Stereo) -> Stereo {
        match self.placement {
            Placement::Stereo => [self.run(0, left), self.run(1, right)],
            Placement::Left => [self.run(0, left), right],
            Placement::Right => [left, self.run(1, right)],
            Placement::Mid | Placement::Side => {
                let (mut mid, mut side) = (0.5 * (left + right), 0.5 * (left - right));
                if self.placement == Placement::Mid {
                    mid = self.run(0, mid);
                } else {
                    side = self.run(0, side);
                }
                [mid + side, mid - side]
            }
        }
    }
    fn flush(&mut self) {
        for section in &mut self.sections {
            for state in section.state.as_flattened_mut() {
                *state = super::fast::flush(*state);
            }
        }
        for state in &mut self.pole.state {
            *state = super::fast::flush(*state);
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Processor {
    bands: [BandState; BANDS],
    output: Glide<1>,
    /// Frames left in the coefficients' glide.
    ramp: u32,
    /// Just switched on: its first settings apply at once.
    fresh: bool,
}

/// Every EQ module a rack can hold, by its number less one.
pub(super) struct Bank {
    eqs: [Processor; MAX_EQS],
    /// The EQs in the rack and switched on, bit n for EQ n + 1.
    active: u16,
    rate: f64,
    /// How far smoothed settings move toward their targets per control block.
    blend: f32,
}
impl Bank {
    pub(super) fn new(rate: u32) -> Self {
        let rate = f64::from(rate);
        Self {
            eqs: [Processor::default(); MAX_EQS],
            active: 0,
            rate,
            blend: (1.0 - (-(RAMP as f64) / (SMOOTHING * rate)).exp()) as f32,
        }
    }
    /// The EQs that play from now on. One switched on starts afresh: its
    /// filters empty, its settings applied at the next control block.
    pub(super) fn activate(&mut self, active: u16) {
        for index in bits(u64::from(active & !self.active)) {
            self.eqs[index] = Processor {
                fresh: true,
                ..Processor::default()
            };
        }
        self.active = active;
    }
    /// Each active EQ follows its settings, as modulated, for another
    /// control block.
    pub(super) fn retarget(&mut self, settings: &[Equalizer; MAX_EQS]) {
        for index in bits(u64::from(self.active)) {
            let eq = &mut self.eqs[index];
            let settings = &settings[index];
            let snap = std::mem::take(&mut eq.fresh);
            // A glide its chain did not play out ends where it was going.
            if eq.ramp > 0 {
                eq.ramp = 0;
                eq.output.land();
                for band in &mut eq.bands {
                    band.land();
                }
            }
            let mut moved = false;
            for (band, settings) in eq.bands.iter_mut().zip(&settings.bands) {
                moved |= band.retarget(settings, self.rate, self.blend, snap);
                band.flush();
            }
            let gain = 10f32.powf(settings.output / 20.0);
            if snap || gain != eq.output.target[0] {
                eq.output.aim([gain], snap);
                moved = true;
            }
            if moved && !snap {
                eq.ramp = RAMP;
            }
        }
    }
    /// EQ `index` (its number less one) filters a frame.
    #[inline]
    pub(super) fn process(&mut self, index: usize, frame: Stereo) -> Stereo {
        let eq = &mut self.eqs[index];
        let gliding = eq.ramp > 0;
        let mut x = frame;
        if gliding {
            eq.ramp -= 1;
            let landing = eq.ramp == 0;
            for band in &mut eq.bands {
                if landing {
                    band.land();
                } else {
                    band.advance();
                }
            }
            if landing {
                eq.output.land();
            } else {
                eq.output.advance();
            }
        }
        for band in &mut eq.bands {
            if band.live() {
                x = band.process(x);
            }
        }
        let gain = eq.output.now[0];
        [x[0] * gain, x[1] * gain]
    }
}

/// The magnitude in dB at `frequency` Hz of what a band does at `rate` Hz,
/// as its coefficients run.
pub fn band_response(band: &Band, rate: f32, frequency: f32) -> f32 {
    if !band.enabled {
        return 0.0;
    }
    let design = design(
        band.kind,
        band.slope,
        f64::from(band.frequency),
        f64::from(band.gain),
        f64::from(band.q),
        f64::from(rate),
    );
    let w = 2.0 * PI * f64::from(frequency) / f64::from(rate);
    let phi = phis(w);
    let mut power = 1.0;
    for section in design.sections.into_iter().flatten() {
        let b = section.b;
        let numerator = [
            (b[0] + b[1] + b[2]).powi(2),
            (b[0] - b[1] + b[2]).powi(2),
            -4.0 * b[0] * b[2],
        ];
        power *= dot(numerator, phi) / dot(squares(section.a), phi);
    }
    if let Some([b0, b1, p]) = design.pole {
        let (c, s) = (w.cos(), w.sin());
        power *=
            ((b0 + b1 * c).powi(2) + (b1 * s).powi(2)) / ((1.0 - p * c).powi(2) + (p * s).powi(2));
    }
    (10.0 * power.max(1e-30).log10()) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eq::Equalizer;

    const RATE: f32 = 48_000.0;

    fn band(kind: Kind, frequency: f32, gain: f32, q: f32) -> Band {
        Band {
            kind,
            frequency,
            gain,
            q,
            ..Band::default()
        }
    }

    /// The analog prototype's magnitude in dB.
    fn analog_db(n: [f64; 3], d: [f64; 3], x: f64) -> f64 {
        10.0 * analog(n, d, x).log10()
    }

    #[test]
    fn bells_keep_their_analog_shape_up_to_nyquist() {
        for (frequency, gain, q) in [
            (100.0, 12.0, 1.0),
            (3000.0, -9.0, 4.0),
            (15_000.0, 12.0, 0.7),
        ] {
            let b = band(Kind::Bell, frequency, gain, q);
            let a = 10f64.powf(f64::from(gain) / 40.0);
            let q = f64::from(q);
            let n = [1.0, a / q, 1.0];
            let d = [1.0, 1.0 / (a * q), 1.0];
            assert!(
                (band_response(&b, RATE, frequency) - gain).abs() < 0.01,
                "{frequency}"
            );
            for probe in [20.0, 200.0, 2000.0, 12_000.0, 18_000.0, 22_000.0] {
                let expected = analog_db(n, d, f64::from(probe / frequency));
                let got = f64::from(band_response(&b, RATE, probe));
                // Right by Nyquist the fit is looser, but nowhere near the
                // bilinear transform's dive to 0 dB.
                let tolerance = if probe > 18_000.0 { 1.0 } else { 0.6 };
                assert!(
                    (got - expected).abs() < tolerance,
                    "{frequency} Hz bell at {probe}: {got} vs {expected}"
                );
            }
        }
        // The bilinear transform would bring a 15 kHz bell back to 0 dB at
        // Nyquist; the analog one is still well up there.
        let high = band_response(&band(Kind::Bell, 15_000.0, 12.0, 0.7), RATE, 23_900.0);
        assert!(high > 5.0, "{high}");
    }

    #[test]
    fn shelves_cuts_and_others_land_where_they_should() {
        let low = band(Kind::LowShelf, 200.0, 9.0, 0.707);
        assert!((band_response(&low, RATE, 20.0) - 9.0).abs() < 0.2);
        assert!((band_response(&low, RATE, 200.0) - 4.5).abs() < 0.05);
        assert!(band_response(&low, RATE, 5000.0).abs() < 0.05);
        let high = band(Kind::HighShelf, 8000.0, -6.0, 0.707);
        assert!((band_response(&high, RATE, 8000.0) + 3.0).abs() < 0.05);
        assert!((band_response(&high, RATE, 23_000.0) + 6.0).abs() < 0.5);
        assert!(band_response(&high, RATE, 100.0).abs() < 0.05);
        for slope in crate::eq::SLOPES {
            let cut = Band {
                slope,
                ..band(Kind::LowCut, 1000.0, 0.0, 0.707)
            };
            // Butterworth: −3 dB at the corner, then `slope` dB per octave.
            assert!(
                (band_response(&cut, RATE, 1000.0) + 3.01).abs() < 0.05,
                "{slope}"
            );
            let octave = band_response(&cut, RATE, 125.0) - band_response(&cut, RATE, 250.0);
            assert!((octave + f32::from(slope)).abs() < 0.3, "{slope}: {octave}");
            assert!(band_response(&cut, RATE, 16_000.0).abs() < 0.02, "{slope}");
            let cut = Band {
                kind: Kind::HighCut,
                ..cut
            };
            assert!(
                (band_response(&cut, RATE, 1000.0) + 3.01).abs() < 0.05,
                "{slope}"
            );
            assert!(band_response(&cut, RATE, 30.0).abs() < 0.01, "{slope}");
        }
        let resonant = band(Kind::HighCut, 2000.0, 0.0, 4.0);
        assert!(band_response(&resonant, RATE, 2000.0) > 10.0);
        let notch = band(Kind::Notch, 1000.0, 0.0, 2.0);
        assert!(band_response(&notch, RATE, 1000.0) < -60.0);
        assert!(band_response(&notch, RATE, 100.0).abs() < 0.1);
        let pass = band(Kind::BandPass, 1000.0, 0.0, 2.0);
        assert!(band_response(&pass, RATE, 1000.0).abs() < 0.01);
        assert!(band_response(&pass, RATE, 100.0) < -20.0);
    }

    /// The impulse response of a section run as a mixed state-variable filter.
    fn impulse(coefficients: [f32; 5], frames: usize) -> Vec<f32> {
        let mut section = Section::default();
        section.aim(Some(coefficients), true);
        (0..frames)
            .map(|i| section.tick(0, if i == 0 { 1.0 } else { 0.0 }))
            .collect()
    }
    /// The same section in direct form.
    fn direct(section: Biquad, frames: usize) -> Vec<f64> {
        let (mut x1, mut x2, mut y1, mut y2) = (0.0, 0.0, 0.0, 0.0);
        (0..frames)
            .map(|i| {
                let x = if i == 0 { 1.0 } else { 0.0 };
                let y = section.b[0] * x + section.b[1] * x1 + section.b[2] * x2
                    - section.a[0] * y1
                    - section.a[1] * y2;
                (x2, x1, y2, y1) = (x1, x, y1, y);
                y
            })
            .collect()
    }

    #[test]
    fn the_state_variable_filter_plays_the_matched_section() {
        let w = |hz: f64| 2.0 * PI * hz / f64::from(RATE);
        for section in [
            bell(w(1000.0), 9.0, 2.0),
            bell(w(40.0), -12.0, 0.5),
            low_shelf(w(120.0), 6.0, 0.7),
            high_shelf(w(12_000.0), -8.0, 1.2),
            high_pass(w(80.0), 0.54),
            low_pass(w(9000.0), 1.3),
            band_pass(w(500.0), 3.0),
            notch(w(60.0), 8.0),
        ] {
            let svf = impulse(state_variable(section), 256);
            let reference = direct(section, 256);
            for (a, b) in svf.iter().zip(&reference) {
                assert!((f64::from(*a) - b).abs() < 2e-4, "{section:?}: {a} vs {b}");
            }
        }
    }

    fn sine(frequency: f32, frames: usize) -> impl Iterator<Item = f32> {
        (0..frames).map(move |i| (2.0 * std::f32::consts::PI * frequency * i as f32 / RATE).sin())
    }

    #[test]
    fn a_bank_filters_each_placement_and_glides_without_jumps() {
        let mut bank = Bank::new(RATE as u32);
        let mut eq = Equalizer::default();
        eq.bands[2] = band(Kind::Bell, 1000.0, 12.0, 1.0);
        let mut settings = [Equalizer::default(); MAX_EQS];
        settings[2] = eq;
        bank.activate(1 << 2);
        let mut loudest = 0f32;
        for (i, x) in sine(1000.0, 48_000).enumerate() {
            if i % RAMP as usize == 0 {
                bank.retarget(&settings);
            }
            let y = bank.process(2, [x, x]);
            if i > 24_000 {
                loudest = loudest.max(y[0].abs());
            }
        }
        assert!((20.0 * loudest.log10() - 12.0).abs() < 0.1, "{loudest}");
        // Mid only: a mono signal is all mid; a side one passes untouched.
        settings[2].bands[2].placement = Placement::Side;
        let mut loudest = [0f32; 2];
        let mut previous = [0f32; 2];
        let mut largest_step = 0f32;
        for (i, x) in sine(1000.0, 24_000).enumerate() {
            if i % RAMP as usize == 0 {
                // The gain swings ±12 dB at 5 Hz the while.
                settings[2].bands[2].gain =
                    12.0 * (i as f32 * 5.0 / RATE * std::f32::consts::TAU).cos();
                bank.retarget(&settings);
            }
            let y = bank.process(2, [x, -x]);
            for c in 0..2 {
                largest_step = largest_step.max((y[c] - previous[c]).abs());
                previous[c] = y[c];
                loudest[c] = loudest[c].max(y[c].abs());
            }
        }
        // A 1 kHz sine moves at most 0.13 per frame at unity, 0.53 at +12 dB.
        assert!(largest_step < 0.6, "{largest_step}");
        assert!(loudest[0] > 3.5 && loudest[0] < 4.1, "{loudest:?}");
        // Off, every section glides to a straight pass and rests.
        for band in &mut settings[2].bands {
            band.enabled = false;
        }
        settings[2].output = -6.0;
        for i in 0..4 * RAMP as usize {
            if i % RAMP as usize == 0 {
                bank.retarget(&settings);
            }
            bank.process(2, [0.25, 0.5]);
        }
        let y = bank.process(2, [0.25, 0.5]);
        assert!(
            (y[0] - 0.25 * 0.5012).abs() < 1e-4 && (y[1] - 0.5 * 0.5012).abs() < 1e-4,
            "{y:?}"
        );
        assert!(bank.eqs[2].bands.iter().all(|b| !b.live()));
    }
}
