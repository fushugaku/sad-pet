//! Control/worker-only polynomial cells fitted to the existing 16-tap sinc
//! kernel. Eight Chebyshev-Lobatto nodes retain endpoints and avoid changing
//! the antialias filter, harmonic support or phase convention.
use super::{Kernel, Level, TAPS};
use crate::wavetable::assets::Asset;
use std::sync::OnceLock;

fn weights() -> &'static [[f64; TAPS]; 8] {
    static WEIGHTS: OnceLock<[[f64; TAPS]; 8]> = OnceLock::new();
    WEIGHTS.get_or_init(|| {
        let kernel = Kernel::shared();
        let mut matrix = [[0.0_f64; 8 + TAPS]; 8];
        for (i, row) in matrix.iter_mut().enumerate() {
            let t = (1.0 - (std::f64::consts::PI * i as f64 / 7.0).cos()) * 0.5;
            let mut power = 1.0;
            for x in &mut row[..8] {
                *x = power;
                power *= t;
            }
            let (k, f) = kernel.position(t);
            for j in 0..TAPS {
                row[8 + j] = (kernel.phases[k][j]
                    + (kernel.phases[k + 1][j] - kernel.phases[k][j]) * f)
                    as f64;
            }
        }
        for col in 0..8 {
            let pivot = (col..8)
                .max_by(|&a, &b| matrix[a][col].abs().total_cmp(&matrix[b][col].abs()))
                .unwrap();
            matrix.swap(col, pivot);
            let scale = matrix[col][col];
            for x in &mut matrix[col][col..] {
                *x /= scale;
            }
            let pivot_row = matrix[col];
            for (row, values) in matrix.iter_mut().enumerate() {
                if row != col {
                    let factor = values[col];
                    for (value, pivot) in values[col..].iter_mut().zip(&pivot_row[col..]) {
                        *value -= factor * pivot;
                    }
                }
            }
        }
        std::array::from_fn(|i| std::array::from_fn(|j| matrix[i][8 + j]))
    })
}
pub(super) fn prepare(data: &mut [f32], levels: &[Level], asset: &Asset) {
    let weights = weights();
    for l in levels {
        if l.polynomial == usize::MAX {
            continue;
        }
        for frame in 0..asset.frame_count {
            let clean = l.offset + frame * l.stride;
            let target = l.polynomial + frame * l.length * 8;
            for i in 0..l.length {
                for (power, weights) in weights.iter().enumerate() {
                    let mut value = 0.0_f64;
                    for j in 0..TAPS {
                        value += data[clean + i + 1 + j] as f64 * weights[j];
                    }
                    data[target + i * 8 + power] = value as f32;
                }
            }
        }
    }
}
