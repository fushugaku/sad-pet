//! NoiseGate (NOISEGTE.ZDL): the shared NoiseGate/DirtyGate kernel in [`super::gate`]
//! with the module's complete recovered editors (threshold table, EnvIn = stereo
//! detector, Level = `v / 100` through the float setter ramp).
//! Separate open/close thresholds and an 11 ms peak hold keep it from chattering.
use super::gate::{Gate, GateKind};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_noise_gate",
    name: "NoiseGate",
    category: Category::Dynamics,
    zdl: "NOISEGTE.ZDL",
    description: "Gate with hysteresis and a short peak hold",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "THRSH",
            max: 24,
            default: 9,
            display: Display::offset(1.0),
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

struct NoiseGate(Gate);

impl Effect for NoiseGate {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.0.set(values[0], values[1]);
    }
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.0.process(frame)
    }
    fn clear(&mut self) {
        self.0.clear();
    }
}

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(NoiseGate(Gate::new(
        GateKind::Noise,
        sample_rate,
        9.0,
        100.0,
    )))
}
