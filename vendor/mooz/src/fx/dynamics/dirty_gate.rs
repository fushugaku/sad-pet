//! DirtyGate (DIRTYGTE.ZDL): the shared NoiseGate/DirtyGate kernel in [`super::gate`]
//! with the module's complete recovered editors (threshold table, EnvIn = stereo
//! detector, Level = `v / 100` through the float setter ramp).
//! One threshold and no peak hold: near the threshold the gain follows the waveform, giving the rough, sputtering cut-off.
use super::gate::{Gate, GateKind};
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_dirty_gate",
    name: "DirtyGate",
    category: Category::Dynamics,
    zdl: "DIRTYGTE.ZDL",
    description: "Gate that chatters with the waveform near threshold",
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

struct DirtyGate(Gate);

impl Effect for DirtyGate {
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
    Box::new(DirtyGate(Gate::new(
        GateKind::Dirty,
        sample_rate,
        9.0,
        100.0,
    )))
}
