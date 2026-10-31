//! Shared six-segment envelopes. Times are seconds, or quarter-note beats.
use serde::{Deserialize, Serialize};

pub const POINTS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Wave {
    pub enabled: bool,
    pub beats: bool,
    pub times: [f32; POINTS],
    pub levels: [f32; POINTS],
    /// -1 = concave, 0 = linear, +1 = convex; continuous and endpoint exact.
    pub curves: [f32; POINTS],
    pub loop_on: bool,
    /// Zero-based target indices. Release follows loop_end, even without looping.
    pub loop_start: u8,
    pub loop_end: u8,
}
impl Default for Wave {
    fn default() -> Self {
        Self {
            enabled: false,
            beats: false,
            times: [0.08, 0.3, 0.5, 0.5, 0.3, 0.3],
            levels: [1.0, 0.35, 0.8, 0.25, 0.65, 0.0],
            curves: [0.0; POINTS],
            loop_on: false,
            loop_start: 1,
            loop_end: 3,
        }
    }
}
impl Wave {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
    pub fn sanitize(&mut self) {
        for i in 0..POINTS {
            crate::preset::finite_clamp(&mut self.times[i], 0.0, 32.0, 0.3);
            crate::preset::finite_clamp(&mut self.levels[i], 0.0, 1.0, 0.0);
            crate::preset::finite_clamp(&mut self.curves[i], -1.0, 1.0, 0.0);
        }
        self.loop_end = self.loop_end.min(4);
        self.loop_start = self.loop_start.min(self.loop_end);
    }
    pub fn get(&self, field: usize) -> f32 {
        match field {
            0 => f32::from(self.enabled),
            1 => f32::from(self.beats),
            2 => f32::from(self.loop_on),
            3 => self.loop_start as f32,
            4 => self.loop_end as f32,
            _ => {
                let n = field - 5;
                match n % 3 {
                    0 => self.times[n / 3],
                    1 => self.levels[n / 3],
                    _ => self.curves[n / 3],
                }
            }
        }
    }
    pub fn set(&mut self, field: usize, value: f32) {
        match field {
            0 => self.enabled = value >= 0.5,
            1 => self.beats = value >= 0.5,
            2 => self.loop_on = value >= 0.5,
            3 => self.loop_start = value.round() as u8,
            4 => self.loop_end = value.round() as u8,
            _ => {
                let n = field - 5;
                match n % 3 {
                    0 => self.times[n / 3] = value,
                    1 => self.levels[n / 3] = value,
                    _ => self.curves[n / 3] = value,
                }
            }
        }
        self.sanitize();
    }
}

/// Does not overshoot and remains linear at zero. No pow/exp in the audio loop.
pub fn curve(x: f32, amount: f32) -> f32 {
    if amount >= 0.0 {
        x + (x * x - x) * amount
    } else {
        x + ((1.0 - (1.0 - x) * (1.0 - x)) - x) * (-amount)
    }
}
