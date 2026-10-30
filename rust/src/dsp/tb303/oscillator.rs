//! Band-limited VCO waves. Q8's periodic circuit solution is prepared before
//! audio runs, then read at the actual free-running phase. Band limiting must
//! follow the square converter: clipping an already limited saw aliases again.
use std::{f64::consts::TAU, sync::OnceLock};

const SIZE: usize = 2048;
const LEVELS: usize = 10;
const SHAPES: usize = 25; // MIDI 0..144 at six-semitone intervals.
const CIRCUIT_SAMPLES: usize = 8192;
type Wave = [f32; SIZE];
type Mips = [Wave; LEVELS];

pub(super) struct Tables {
    saw: Box<Mips>,
    square: Vec<Box<Mips>>,
}
#[derive(Clone, Copy)]
pub(super) struct Band {
    level: usize,
    blend: f64,
}

impl Tables {
    pub(super) fn shared() -> &'static Self {
        static TABLES: OnceLock<Tables> = OnceLock::new();
        TABLES.get_or_init(|| {
            let mut saw = Box::new([[0.0; SIZE]; LEVELS]);
            for (level, table) in saw.iter_mut().enumerate() {
                for (i, value) in table.iter_mut().enumerate() {
                    let phase = TAU * i as f64 / SIZE as f64;
                    *value = ((1..=512 >> level)
                        .map(|k| -(phase * k as f64).sin() / k as f64)
                        .sum::<f64>()
                        * 2.0
                        / std::f64::consts::PI) as f32;
                }
            }
            let mut square = Vec::with_capacity(SHAPES);
            let mut inverse = vec![(0.0, 0.0); SIZE];
            for shape in 0..SHAPES {
                let frequency = 440.0 * 2.0_f64.powf((shape as f64 * 6.0 - 69.0) / 12.0);
                let mut spectrum = square_cycle(frequency);
                fft(&mut spectrum, false);
                let mut mips = Box::new([[0.0; SIZE]; LEVELS]);
                for (level, table) in mips.iter_mut().enumerate() {
                    inverse.fill((0.0, 0.0));
                    // Each periodic wave is AC. The ladder's coupling network
                    // still models the real audio-rate high-pass response.
                    for k in 1..=512 >> level {
                        let (re, im) = spectrum[k];
                        let scale = SIZE as f64 / CIRCUIT_SAMPLES as f64;
                        inverse[k] = (re * scale, im * scale);
                        inverse[SIZE - k] = (re * scale, -im * scale);
                    }
                    fft(&mut inverse, true);
                    for (value, &(re, _)) in table.iter_mut().zip(&inverse) {
                        *value = re as f32;
                    }
                }
                square.push(mips);
            }
            Self { saw, square }
        })
    }
    pub(super) fn band(dt: f64) -> Band {
        let allowed = 0.45 / dt;
        let mut level = 0;
        while level + 1 < LEVELS && (512 >> level) as f64 > allowed {
            level += 1;
        }
        Band {
            level,
            blend: (allowed / (512 >> level) as f64 - 1.0).clamp(0.0, 1.0),
        }
    }
    fn read(table: &Wave, phase: f64) -> f64 {
        let position = phase * SIZE as f64;
        let i = position.floor() as usize;
        let t = position - position.floor();
        let [a, b, c, d] =
            [i.wrapping_sub(1), i, i + 1, i + 2].map(|j| table[j & (SIZE - 1)] as f64);
        b + 0.5 * t * (c - a + t * (2.0 * a - 5.0 * b + 4.0 * c - d + t * (3.0 * (b - c) + d - a)))
    }
    fn limited(mips: &Mips, phase: f64, band: Band) -> f64 {
        let high = Self::read(&mips[band.level], phase);
        if band.level + 1 == LEVELS {
            return high;
        }
        let low = Self::read(&mips[band.level + 1], phase);
        low + (high - low) * band.blend
    }
    pub(super) fn sample(&self, phase: f64, band: Band, pitch: f64, waveform: f64) -> f64 {
        let saw = if waveform < 1.0 {
            Self::limited(&self.saw, phase, band)
        } else {
            0.0
        };
        if waveform <= 0.0 {
            return saw;
        }
        let position = (pitch / 6.0).clamp(0.0, (SHAPES - 1) as f64);
        let i = position.floor() as usize;
        let next = (i + 1).min(SHAPES - 1);
        let a = Self::limited(&self.square[i], phase, band);
        let b = Self::limited(&self.square[next], phase, band);
        let square = a + (b - a) * (position - i as f64);
        saw + (square - saw) * waveform
    }
}

/// Small radix-2 transform used only while preparing the immutable VCO tables.
fn fft(data: &mut [(f64, f64)], inverse: bool) {
    let n = data.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            data.swap(i, j);
        }
    }
    let mut size = 2;
    while size <= n {
        let angle = if inverse { TAU } else { -TAU } / size as f64;
        let step = (angle.cos(), angle.sin());
        for start in (0..n).step_by(size) {
            let mut w = (1.0, 0.0);
            for k in 0..size / 2 {
                let a = data[start + k];
                let b = data[start + k + size / 2];
                let v = (b.0 * w.0 - b.1 * w.1, b.0 * w.1 + b.1 * w.0);
                data[start + k] = (a.0 + v.0, a.1 + v.1);
                data[start + k + size / 2] = (a.0 - v.0, a.1 - v.1);
                w = (w.0 * step.0 - w.1 * step.1, w.0 * step.1 + w.1 * step.0);
            }
        }
        size *= 2;
    }
    if inverse {
        for x in data {
            x.0 /= n as f64;
            x.1 /= n as f64;
        }
    }
}

fn period(mut shaper: SquareShaper) -> SquareShaper {
    for i in 0..CIRCUIT_SAMPLES {
        shaper.process(2.0 * i as f64 / CIRCUIT_SAMPLES as f64 - 1.0);
    }
    shaper
}
fn periodic_shaper(frequency: f64) -> SquareShaper {
    let mut shaper = SquareShaper::new(frequency * CIRCUIT_SAMPLES as f64);
    shaper.emitter = 9.0;
    for _ in 0..16 {
        let end = period(shaper);
        let r = [
            end.emitter - shaper.emitter,
            end.capacitor - shaper.capacitor,
        ];
        if r[0].abs().max(r[1].abs()) < 1e-10 {
            return shaper;
        }
        let eps = 1e-3;
        let mut e = shaper;
        e.emitter += eps;
        let e = period(e);
        let mut q = shaper;
        q.capacitor += eps;
        let q = period(q);
        let a = (e.emitter - end.emitter) / eps - 1.0;
        let b = (q.emitter - end.emitter) / eps;
        let c = (e.capacitor - end.capacitor) / eps;
        let d = (q.capacitor - end.capacitor) / eps - 1.0;
        let det = a * d - b * c;
        if det.abs() < 1e-12 {
            shaper = end;
            continue;
        }
        shaper.emitter = (shaper.emitter + (-r[0] * d + b * r[1]) / det).clamp(5.6, 12.0);
        shaper.capacitor += ((c * r[0] - a * r[1]) / det).clamp(-2.0, 2.0);
    }
    // Bounded fallback for a nominal transistor at a region boundary.
    for _ in 0..128 {
        shaper = period(shaper);
    }
    shaper
}
fn square_cycle(frequency: f64) -> Vec<(f64, f64)> {
    let mut shaper = periodic_shaper(frequency);
    (0..CIRCUIT_SAMPLES)
        .map(|i| {
            (
                shaper.process(2.0 * i as f64 / CIRCUIT_SAMPLES as f64 - 1.0),
                0.0,
            )
        })
        .collect()
}
/// Q8's PNP saw-to-pulse converter. C10/R34/R35 shape the base drive;
/// R45/C11 retain emitter charge. A constant-Vbe, finite-beta model solves
/// cutoff, forward-active and saturated regions implicitly at the VCO rate.
#[derive(Clone, Copy)]
struct SquareShaper {
    emitter: f64,
    capacitor: f64,
    dt: f64,
    a: f64,
    k: f64,
}
impl SquareShaper {
    fn new(rate: f64) -> Self {
        let k = 1.0 / (rate * 10_000.0 * 0.01e-6);
        Self {
            emitter: 12.0,
            capacitor: 0.0,
            dt: 1.0 / rate,
            a: 1.0 / (1.0 + k),
            k,
        }
    }
    fn process(&mut self, saw: f64) -> f64 {
        let saw = 8.75 - 3.25 * saw;
        let b = self.dt / 1e-6; // C11 = 1 uF.
        let off_e = (self.emitter + b * 12.0 / 22_000.0) / (1.0 + b / 22_000.0);
        let off_c = self.capacitor / (1.0 + self.dt / (110_000.0 * 0.01e-6));
        let base = saw - off_c * 100_000.0 / 110_000.0;
        let collector = if off_e - base <= 0.6 {
            self.emitter = off_e;
            self.capacitor = off_c;
            5.333
        } else {
            let a = 1.0 / 100_000.0 + self.a / 10_000.0;
            let offset = self.a * self.capacitor / 10_000.0;
            let mut e = (self.emitter + b * (12.0 / 22_000.0 + 201.0 * (a * (0.6 + saw) - offset)))
                / (1.0 + b * (1.0 / 22_000.0 + 201.0 * a));
            let mut ic = 200.0 * (a * (e - 0.6 - saw) + offset);
            if 5.333 + ic * 10_000.0 > e - 0.2 {
                e = (self.emitter
                    + b * (12.0 / 22_000.0 + 5.533 / 10_000.0 + a * (0.6 + saw) - offset))
                    / (1.0 + b * (1.0 / 22_000.0 + 1.0 / 10_000.0 + a));
                ic = ((e - 5.533) / 10_000.0).max(0.0);
            }
            self.capacitor = self.a * (self.capacitor - self.k * (e - 0.6 - saw));
            self.emitter = e;
            5.333 + ic * 10_000.0
        };
        (collector - 7.0) / 3.25
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_vco_waves_suppress_low_frequency_foldback() {
        let tables = Tables::shared();
        let n = 8192;
        let fundamental = 509;
        let band = Tables::band(fundamental as f64 / n as f64);
        for waveform in [0.0, 1.0] {
            for folded_bin in [48, 461, 557] {
                let mut re = 0.0;
                let mut im = 0.0;
                for i in 0..n {
                    let phase = (i * fundamental % n) as f64 / n as f64;
                    let x = tables.sample(phase, band, 110.0, waveform);
                    let angle = TAU * (i * folded_bin) as f64 / n as f64;
                    re += x * angle.cos();
                    im += x * angle.sin();
                }
                let amplitude = 2.0 * re.hypot(im) / n as f64;
                assert!(
                    amplitude < 1e-6,
                    "Wave {waveform}, folded bin {folded_bin}: {amplitude}"
                );
            }
        }
    }

    #[test]
    fn q8_periodic_solution_retains_the_frequency_dependent_pulse() {
        let mut widths = Vec::new();
        for shape in 0..SHAPES {
            let frequency = 440.0 * 2.0_f64.powf((shape as f64 * 6.0 - 69.0) / 12.0);
            let initial = periodic_shaper(frequency);
            let end = period(initial);
            assert!((end.emitter - initial.emitter).abs() < 1e-7);
            assert!((end.capacitor - initial.capacitor).abs() < 1e-7);
            let cycle = square_cycle(frequency);
            let min = cycle.iter().map(|x| x.0).fold(f64::INFINITY, f64::min);
            let max = cycle.iter().map(|x| x.0).fold(f64::NEG_INFINITY, f64::max);
            assert!(min.is_finite() && max.is_finite() && min >= -1.0 && max <= 1.5);
            widths.push(
                cycle.iter().filter(|x| x.0 > (min + max) * 0.5).count() as f64
                    / cycle.len() as f64,
            );
        }
        assert!(
            widths[6] > 0.4 && widths[14] < widths[6] - 0.1,
            "{widths:?}"
        );
    }

    #[test]
    fn spectral_bands_and_pitch_shapes_blend_continuously() {
        let tables = Tables::shared();
        for harmonics in [2.0, 4.0, 16.0, 128.0, 512.0] {
            for waveform in [0.0, 1.0] {
                let a = tables.sample(
                    0.731,
                    Tables::band(0.45 / harmonics - 1e-10),
                    83.999999,
                    waveform,
                );
                let b = tables.sample(
                    0.731,
                    Tables::band(0.45 / harmonics + 1e-10),
                    84.000001,
                    waveform,
                );
                assert!(
                    (a - b).abs() < 1e-5,
                    "Band/shape boundary jumped by {}",
                    a - b
                );
            }
        }
    }
}
