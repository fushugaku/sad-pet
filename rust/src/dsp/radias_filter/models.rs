//! Rustias nonlinear and linear filter kernels, circuit/filters.rs at 91cae3f.
//! f64, two oversampling passes, transfer functions and operation order are
//! preserved. Host-rate prewarping and smoothing extend the original 48 kHz
//! model; parameter caches contain no signal history. Firmware-free.
#[derive(Default)]
pub struct Model {
    poles: [f64; 4],
    svf: [f64; 4],
    previous: f64,
    cutoff: f64,
    resonance: f64,
    drive: f64,
    morph: f64,
    key: Option<(u8, u64, u64, u64, u64)>,
    gain: f64,
    output: f64,
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
        input: f64,
        mode: u8,
        cutoff: f64,
        resonance: f64,
        drive: f64,
        morph: f64,
        gain: f64,
    ) -> f64 {
        self.sample_with_gain(input, mode, cutoff, resonance, drive, morph, Some(gain))
    }

    #[allow(clippy::too_many_arguments)]
    fn sample_with_gain(
        &mut self,
        input: f64,
        mode: u8,
        cutoff: f64,
        resonance: f64,
        drive: f64,
        morph: f64,
        gain: Option<f64>,
    ) -> f64 {
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
        self.output += (result.clamp(-4.0, 4.0) - self.output) * self.smoothing;
        self.output
    }

    fn render<const MODE: u8>(&mut self, input: f64) -> f64 {
        let mut result = 0.0;
        for x in [(self.previous + input) * 0.5, input] {
            let x = if MODE >= 10 && self.drive <= 1.0 {
                x
            } else {
                soft(x * self.drive)
            };
            let y = if matches!(MODE, 1 | 4 | 7 | 8 | 9) {
                let g = self.ladder_gain;
                let feedback = self.feedback;
                let mut y = soft(x - self.poles[3] * feedback);
                let mut taps = [0.0; 4];
                for (i, z) in self.poles.iter_mut().enumerate() {
                    let v = (y - *z) * g;
                    let p = v + *z;
                    *z = (p + v).clamp(-4.0, 4.0);
                    y = if MODE == 4 {
                        soft(p * 1.15) / 1.15
                    } else if MODE == 9 {
                        soft(p * self.stage_drive) / self.stage_drive
                    } else {
                        p
                    };
                    taps[i] = y;
                }
                match MODE {
                    7 => x - 4.0 * taps[0] + 6.0 * taps[1] - 4.0 * taps[2] + taps[3],
                    8 => 4.0 * (taps[1] - 2.0 * taps[2] + taps[3]),
                    _ => y * self.output_gain,
                }
            } else {
                let k = self.damping;
                let a1 = self.a1;
                if MODE >= 10 {
                    let mut output = x;
                    for stage in 0..if MODE % 2 == 1 { 2 } else { 1 } {
                        let i = stage * 2;
                        let v3 = output - self.svf[i + 1];
                        let band = a1 * self.svf[i] + self.a2 * v3;
                        let low = self.svf[i + 1] + self.a2 * self.svf[i] + self.a3 * v3;
                        self.svf[i] = 2.0 * band - self.svf[i];
                        self.svf[i + 1] = 2.0 * low - self.svf[i + 1];
                        let high = output - k * band - low;
                        output = match (MODE - 10) / 2 {
                            0 => low,
                            1 => high,
                            2 => band * k,
                            _ => low + high,
                        };
                    }
                    result += output * 0.5;
                    continue;
                }
                let v3 = x - self.svf[1];
                let band = a1 * self.svf[0] + self.a2 * v3;
                let low = self.svf[1] + self.a2 * self.svf[0] + self.a3 * v3;
                self.svf[0] = soft(2.0 * band - self.svf[0]);
                self.svf[1] = soft(2.0 * low - self.svf[1]);
                let high = x - k * band - low;
                match MODE {
                    2 => soft((low + band * self.resonance * 0.35) * 1.3) / 1.3,
                    3 => soft(high * 1.3) / 1.3,
                    5 => {
                        if self.morph < 0.5 {
                            low + high * self.morph * 2.0
                        } else {
                            low * (2.0 - self.morph * 2.0) + high
                        }
                    }
                    6 => {
                        let taps = [low, band, high, low + high];
                        let i = self.morph_index;
                        taps[i] + (taps[i + 1] - taps[i]) * self.morph_fraction
                    }
                    _ => low,
                }
            };
            result += y * 0.5;
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
        for mode in 0..18 {
            let mut m = Model::new(48_000.0);
            let mut seed = 0x51a3beef_u32;
            for i in 0..4096 {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                let input =
                    seed as i32 as f64 / 2147483648.0 * if i % 37 == 0 { 3.0 } else { 0.35 };
                let step = (i / 256) % 4;
                let cutoff = [120.0, 1800.0, 9900.0, 18000.0][step];
                let gain = (std::f64::consts::PI * cutoff / 96_000.0).tan();
                let output = m.sample(
                    input,
                    mode,
                    cutoff,
                    [0.0, 0.25, 0.99, 1.0][step],
                    [0.0, 0.23, 1.0, 0.7][step],
                    [0.0, 0.33, 0.67, 1.0][step],
                    gain,
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
