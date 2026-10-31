use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Motion {
    pub surf: f32,
    pub xy_on: bool,
    pub x: f32,
    pub y: f32,
}
impl Default for Motion {
    fn default() -> Self {
        Self {
            surf: 0.0,
            xy_on: false,
            x: 0.5,
            y: 0.5,
        }
    }
}
pub const FIELDS: [(&str, f32, f32); 4] = [
    ("surf", -1.0, 1.0),
    ("xy_on", 0.0, 1.0),
    ("x", 0.0, 1.0),
    ("y", 0.0, 1.0),
];
impl Motion {
    pub fn get(self, n: usize) -> f32 {
        match n {
            0 => self.surf,
            1 => f32::from(self.xy_on),
            2 => self.x,
            _ => self.y,
        }
    }
    pub fn set(&mut self, n: usize, v: f32) {
        match n {
            0 => self.surf = v,
            1 => self.xy_on = v >= 0.5,
            2 => self.x = v,
            _ => self.y = v,
        }
        self.sanitize();
    }
    pub fn sanitize(&mut self) {
        crate::preset::finite_clamp(&mut self.surf, -1.0, 1.0, 0.0);
        crate::preset::finite_clamp(&mut self.x, 0.0, 1.0, 0.5);
        crate::preset::finite_clamp(&mut self.y, 0.0, 1.0, 0.5);
    }
}
/// A/B across the top, C/D below. With three waves, C owns the lower edge.
/// With two, Y is neutral. In every topology the weights sum to exactly one.
pub fn weights(count: usize, x: f32, y: f32) -> [f32; 4] {
    match count {
        1 => [1.0, 0.0, 0.0, 0.0],
        2 => [1.0 - x, x, 0.0, 0.0],
        3 => [(1.0 - x) * (1.0 - y), x * (1.0 - y), y, 0.0],
        _ => [(1.0 - x) * (1.0 - y), x * (1.0 - y), (1.0 - x) * y, x * y],
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn corners_and_reduced_topologies_conserve_gain() {
        for count in 1..=4 {
            for ix in 0..=10 {
                for iy in 0..=10 {
                    let w = weights(count, ix as f32 / 10.0, iy as f32 / 10.0);
                    assert!((w.iter().sum::<f32>() - 1.0).abs() < 1e-6);
                    assert!(w.iter().all(|v| *v >= 0.0));
                    assert!(w[count..].iter().all(|v| *v == 0.0));
                }
            }
        }
        assert_eq!(weights(4, 1.0, 1.0), [0.0, 0.0, 0.0, 1.0]);
    }
}
