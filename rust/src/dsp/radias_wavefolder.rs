//! Rustias OSC 4 Wavefolder, circuit/expanded.rs at 91cae3f.
//! Keep its f64, four internal steps, smoothing, five transfer functions and
//! DC blocker together. Every Resynth carrier owns independent history.
use crate::resynth::{FoldShape, Waveform};

#[derive(Default)]
pub(super) struct Wavefolder {
    pub phase: f64,
    output: f64,
    dc_input: f64,
    dc_output: f64,
    depth: Option<f64>,
    bias: Option<f64>,
}
impl Wavefolder {
    pub fn next(
        &mut self,
        increment: f64,
        wave: Waveform,
        shape: FoldShape,
        controls: [f64; 2],
        sync: bool,
    ) -> f32 {
        if sync {
            self.phase = 0.0;
        }
        let depth = self.depth.get_or_insert(controls[0]);
        *depth += (controls[0] - *depth) * 0.02;
        let depth = *depth;
        let bias = self.bias.get_or_insert(controls[1]);
        *bias += (controls[1] - *bias) * 0.02;
        let bias = *bias;
        let mut sum = 0.0;
        for _ in 0..4 {
            self.phase = (self.phase + increment * 0.25).fract();
            let x = match wave {
                Waveform::Saw => self.phase * 2.0 - 1.0,
                Waveform::Pulse => {
                    if self.phase < 0.5 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                Waveform::Triangle => 1.0 - 4.0 * (self.phase - 0.5).abs(),
                _ => (self.phase * std::f64::consts::TAU).sin(),
            };
            let driven = x * (1.0 + depth * 11.0) + bias * 2.0;
            let triangle = |v: f64| 1.0 - 4.0 * (((v + 1.0) * 0.25).rem_euclid(1.0) - 0.5).abs();
            let folded = match shape {
                FoldShape::Triangle => triangle(driven),
                FoldShape::Sine => (driven * std::f64::consts::FRAC_PI_2).sin(),
                FoldShape::Soft => soft(triangle(driven) * 1.5),
                FoldShape::Asymmetric => triangle(driven + depth),
                FoldShape::Rectify => driven.abs().rem_euclid(2.0) - 1.0,
            };
            self.output += (folded - self.output) * 0.6;
            sum += self.output * 0.25;
        }
        let dc = sum - self.dc_input + 0.9995 * self.dc_output;
        self.dc_input = sum;
        self.dc_output = dc;
        dc.clamp(-1.0, 1.0) as f32
    }
}
fn soft(x: f64) -> f64 {
    let x = x.clamp(-3.0, 3.0);
    x * (27.0 + x * x) / (27.0 + 9.0 * x * x)
}
