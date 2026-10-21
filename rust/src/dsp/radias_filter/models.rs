//! Rustias nonlinear and linear filter kernels, circuit/filters.rs at 91cae3f.
//! f64, two oversampling passes, transfer functions and operation order are
//! preserved. Host-rate prewarping and smoothing extend the original 48 kHz
//! model; parameter caches contain no signal history. Firmware-free.
//! Both channels share one parameter state and run as two independent lanes,
//! each with the exact per-channel operations, so they can use one vector op.
use crate::dsp::fast::flush64;

pub type Lanes = [f64; 2];

#[inline(always)]
fn lanes(f: impl FnMut(usize) -> f64) -> Lanes {
    std::array::from_fn(f)
}

#[derive(Default)]
pub struct Model {
    poles: [Lanes; 4],
    svf: [Lanes; 4],
    previous: Lanes,
    cutoff: f64,
    resonance: f64,
    drive: f64,
    morph: f64,
    key: Option<(u8, u64, u64, u64, u64)>,
    gain: f64,
    output: Lanes,
    mode: u8,
    sample_rate: f64,
    smoothing: f64,
    ladder_gain: f64,
    feedback: f64,
    stage_drive: f64,
    output_gain: f64,
    damping: f64,
    a1: f64,
    a2: f64,
    a3: f64,
    morph_index: usize,
    morph_fraction: f64,
}
pub(super) fn soft(x: f64) -> f64 {
    // At either clamp endpoint the original rational expression is exactly
    // +/-1. Avoid its division for saturated inputs without approximating it.
    if x >= 3.0 {
        return 1.0;
    }
    if x <= -3.0 {
        return -1.0;
    }
    x * (27.0 + x * x) / (27.0 + 9.0 * x * x)
}
/// soft() on both lanes. Selects rather than branches, so the lanes can share
/// one division; the selected values are the ones soft() returns.
#[inline(always)]
fn soft_lanes(x: Lanes) -> Lanes {
    x.map(|x| {
        let y = x * (27.0 + x * x) / (27.0 + 9.0 * x * x);
        if x >= 3.0 {
            1.0
        } else if x <= -3.0 {
            -1.0
        } else {
            y
        }
    })
}
impl Model {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate,
            smoothing: if sample_rate == 48_000.0 {
                0.65
            } else {
                1.0 - 0.35_f64.powf(48_000.0 / sample_rate)
            },
            ..Self::default()
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn sample(
        &mut self,
        input: Lanes,
        mode: u8,
        cutoff: f64,
        resonance: f64,
        drive: f64,
        morph: f64,
        gain: f64,
    ) -> Lanes {
        self.sample_with_gain(input, mode, cutoff, resonance, drive, morph, Some(gain))
    }

    #[allow(clippy::too_many_arguments)]
    fn sample_with_gain(
        &mut self,
        input: Lanes,
        mode: u8,
        cutoff: f64,
        resonance: f64,
        drive: f64,
        morph: f64,
        gain: Option<f64>,
    ) -> Lanes {
        let key = (
            mode,
            cutoff.to_bits(),
            resonance.to_bits(),
            drive.to_bits(),
            morph.to_bits(),
        );
        if self.key != Some(key) {
            let cutoff = cutoff.clamp(12.0, 20_000.0_f64.min(self.sample_rate * 0.45));
            let resonance = resonance.clamp(0.0, 1.0);
            let cutoff_changed = self.key.is_none() || self.cutoff.to_bits() != cutoff.to_bits();
            let coefficients_changed = cutoff_changed
                || self.resonance.to_bits() != resonance.to_bits()
                || self.mode != mode;
            // Resonance, Drive and Morph modulation must not repeat tan() when
            // cutoff is unchanged. Keep the original f64 expression and rate.
            if cutoff_changed {
                self.gain = gain.unwrap_or_else(|| {
                    (std::f64::consts::PI * cutoff / (self.sample_rate * 2.0)).tan()
                });
            }
            self.key = Some(key);
            self.cutoff = cutoff;
            self.resonance = resonance;
            self.drive = 1.0 + drive.clamp(0.0, 1.0) * 11.0;
            self.morph = morph.clamp(0.0, 1.0);
            self.mode = mode;
            // Cache only parameter arithmetic, never signal history. Preserve
            // multiplication/division order so the rendered PCM remains exact.
            if coefficients_changed {
                if matches!(mode, 1 | 4 | 7 | 8 | 9) {
                    self.ladder_gain = self.gain / (1.0 + self.gain);
                    self.feedback = self.resonance * if mode == 4 { 4.1 } else { 3.95 };
                    self.stage_drive = 1.0 + self.resonance * 0.5;
                    self.output_gain = 1.0 + self.feedback * 0.2;
                } else {
                    self.damping = (2.0 - self.resonance * 1.98).max(0.02);
                    self.a1 = 1.0 / (1.0 + self.gain * (self.gain + self.damping));
                    self.a2 = self.gain * self.a1;
                    self.a3 = self.gain * self.gain * self.a1;
                }
            }
            if mode == 6 {
                let position = self.morph * 3.0;
                self.morph_index = (position as usize).min(2);
                self.morph_fraction = position - self.morph_index as f64;
            }
        }
        // Select the topology once per output sample. Const-specialized kernels
        // remove model tests from both oversampling passes and all four poles.
        let result = match self.mode {
            1 => self.render::<1>(input),
            2 => self.render::<2>(input),
            3 => self.render::<3>(input),
            4 => self.render::<4>(input),
            5 => self.render::<5>(input),
            6 => self.render::<6>(input),
            7 => self.render::<7>(input),
            8 => self.render::<8>(input),
            9 => self.render::<9>(input),
            10 => self.render::<10>(input),
            11 => self.render::<11>(input),
            12 => self.render::<12>(input),
            13 => self.render::<13>(input),
            14 => self.render::<14>(input),
            15 => self.render::<15>(input),
            mode if mode >= 16 && mode % 2 == 0 => self.render::<16>(input),
            mode if mode >= 16 => self.render::<17>(input),
            _ => self.render::<0>(input),
        };
        self.previous = input;
        // A short output smoother also de-clicks a live model change.
        for (output, result) in self.output.iter_mut().zip(result) {
            *output = flush64(*output + (result.clamp(-4.0, 4.0) - *output) * self.smoothing);
        }
        // Decaying integrators leave the subnormal range long before they matter.
        for lane in self.poles.iter_mut().chain(self.svf.iter_mut()) {
            *lane = lane.map(flush64);
        }
        self.output
    }

    fn render<const MODE: u8>(&mut self, input: Lanes) -> Lanes {
        let mut result = [0.0; 2];
        for x in [lanes(|c| (self.previous[c] + input[c]) * 0.5), input] {
            let x = if MODE >= 10 && self.drive <= 1.0 {
                x
            } else {
                soft_lanes(x.map(|x| x * self.drive))
            };
            let y = if matches!(MODE, 1 | 4 | 7 | 8 | 9) {
                let g = self.ladder_gain;
                let feedback = self.feedback;
                let mut y = soft_lanes(lanes(|c| x[c] - self.poles[3][c] * feedback));
                let mut taps = [[0.0; 2]; 4];
                for (i, z) in self.poles.iter_mut().enumerate() {
                    let v = lanes(|c| (y[c] - z[c]) * g);
                    let p = lanes(|c| v[c] + z[c]);
                    *z = lanes(|c| (p[c] + v[c]).clamp(-4.0, 4.0));
                    y = if MODE == 4 {
                        soft_lanes(p.map(|p| p * 1.15)).map(|y| y / 1.15)
                    } else if MODE == 9 {
                        soft_lanes(p.map(|p| p * self.stage_drive)).map(|y| y / self.stage_drive)
                    } else {
                        p
                    };
                    taps[i] = y;
                }
                match MODE {
                    7 => lanes(|c| {
                        x[c] - 4.0 * taps[0][c] + 6.0 * taps[1][c] - 4.0 * taps[2][c] + taps[3][c]
                    }),
                    8 => lanes(|c| 4.0 * (taps[1][c] - 2.0 * taps[2][c] + taps[3][c])),
                    _ => y.map(|y| y * self.output_gain),
                }
            } else {
                let k = self.damping;
                let a1 = self.a1;
                if MODE >= 10 {
                    let mut output = x;
                    for stage in 0..if MODE % 2 == 1 { 2 } else { 1 } {
                        let i = stage * 2;
                        let [s0, s1] = [self.svf[i], self.svf[i + 1]];
                        let v3 = lanes(|c| output[c] - s1[c]);
                        let band = lanes(|c| a1 * s0[c] + self.a2 * v3[c]);
                        let low = lanes(|c| s1[c] + self.a2 * s0[c] + self.a3 * v3[c]);
                        self.svf[i] = lanes(|c| 2.0 * band[c] - s0[c]);
                        self.svf[i + 1] = lanes(|c| 2.0 * low[c] - s1[c]);
                        let high = lanes(|c| output[c] - k * band[c] - low[c]);
                        output = match (MODE - 10) / 2 {
                            0 => low,
                            1 => high,
                            2 => band.map(|band| band * k),
                            _ => lanes(|c| low[c] + high[c]),
                        };
                    }
                    result = lanes(|c| result[c] + output[c] * 0.5);
                    continue;
                }
                let [s0, s1] = [self.svf[0], self.svf[1]];
                let v3 = lanes(|c| x[c] - s1[c]);
                let band = lanes(|c| a1 * s0[c] + self.a2 * v3[c]);
                let low = lanes(|c| s1[c] + self.a2 * s0[c] + self.a3 * v3[c]);
                self.svf[0] = soft_lanes(lanes(|c| 2.0 * band[c] - s0[c]));
                self.svf[1] = soft_lanes(lanes(|c| 2.0 * low[c] - s1[c]));
                let high = lanes(|c| x[c] - k * band[c] - low[c]);
                match MODE {
                    2 => soft_lanes(lanes(|c| (low[c] + band[c] * self.resonance * 0.35) * 1.3))
                        .map(|y| y / 1.3),
                    3 => soft_lanes(high.map(|high| high * 1.3)).map(|y| y / 1.3),
                    5 => {
                        if self.morph < 0.5 {
                            lanes(|c| low[c] + high[c] * self.morph * 2.0)
                        } else {
                            lanes(|c| low[c] * (2.0 - self.morph * 2.0) + high[c])
                        }
                    }
                    6 => {
                        let taps = [low, band, high, lanes(|c| low[c] + high[c])];
                        let i = self.morph_index;
                        lanes(|c| taps[i][c] + (taps[i + 1][c] - taps[i][c]) * self.morph_fraction)
                    }
                    _ => low,
                }
            };
            result = lanes(|c| result[c] + y[c] * 0.5);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Model;

    #[test]
    fn kernels_match_original_rustias_across_live_parameter_changes() {
        // Captured by running upstream circuit/filters.rs at 91cae3f directly
        // (not this adapter), with the deterministic input and settings below.
        let expected: Vec<Vec<f64>> = serde_json::from_str(include_str!(
            "../../../tests/fixtures/rustias-filters-91cae3f.json"
        ))
        .unwrap();
        // Lane 0 always carries the reference input. The second lane carries
        // the same input in one model and unrelated audio in the other, so it
        // must match bit for bit in the first and must not leak in the second.
        for mode in 0..18 {
            let mut same = Model::new(48_000.0);
            let mut other = Model::new(48_000.0);
            let mut seed = 0x51a3beef_u32;
            for i in 0..4096 {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                let input =
                    seed as i32 as f64 / 2147483648.0 * if i % 37 == 0 { 3.0 } else { 0.35 };
                let unrelated = (i as f64 * 0.37).sin() * if i % 53 == 0 { 4.0 } else { 0.6 };
                let step = (i / 256) % 4;
                let cutoff = [120.0, 1800.0, 9900.0, 18000.0][step];
                let gain = (std::f64::consts::PI * cutoff / 96_000.0).tan();
                let settings = (
                    cutoff,
                    [0.0, 0.25, 0.99, 1.0][step],
                    [0.0, 0.23, 1.0, 0.7][step],
                    [0.0, 0.33, 0.67, 1.0][step],
                );
                let (cutoff, resonance, drive, morph) = settings;
                let [output, copy] =
                    same.sample([input; 2], mode, cutoff, resonance, drive, morph, gain);
                let [isolated, _] = other.sample(
                    [input, unrelated],
                    mode,
                    cutoff,
                    resonance,
                    drive,
                    morph,
                    gain,
                );
                assert_eq!(output.to_bits(), copy.to_bits(), "model {mode}, frame {i}");
                assert_eq!(
                    output.to_bits(),
                    isolated.to_bits(),
                    "model {mode}, frame {i}"
                );
                if i % 128 == 127 {
                    let reference = expected[mode as usize][i / 128];
                    assert!(
                        (output - reference).abs() < 1e-11,
                        "model {mode}, frame {i}: {output} != {reference}"
                    );
                }
            }
        }
    }
}
