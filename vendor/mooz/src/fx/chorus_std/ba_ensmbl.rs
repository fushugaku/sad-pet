//! Ba Ensmbl (B_ENSMBL.ZDL): the Ensemble module with bass defaults. Its
//! callback, editors and tone table are byte-identical to ENSEMBLE.ZDL (only
//! the table addresses differ), so it runs [`super::ensemble`]'s core.
use super::ensemble::Ensemble;
use crate::{Effect, Values, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_ba_ensmbl",
    name: "Ba Ensmbl",
    category: Category::Modulation,
    zdl: "B_ENSMBL.ZDL",
    description: "Bass ensemble chorus, wet in opposite phase L/R",
    params: &[
        // values[0] is firmware parameter word 2
        ParamInfo {
            name: "Depth",
            max: 100,
            default: 48,
            display: Display::INT,
        },
        // values[1] is firmware parameter word 3
        ParamInfo {
            name: "Rate",
            max: 49,
            default: 22,
            display: Display::offset(1.0),
        },
        // values[2] is firmware parameter word 4
        ParamInfo {
            name: "Mix",
            max: 100,
            default: 80,
            display: Display::INT,
        },
        // values[3] is firmware parameter word 5
        ParamInfo {
            name: "Tone",
            max: 10,
            default: 5,
            display: Display::INT,
        },
        // values[4] is firmware parameter word 6
        ParamInfo {
            name: "Level",
            max: 150,
            default: 100,
            display: Display::INT,
        },
    ],
};

struct BaEnsmbl(Ensemble);

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(BaEnsmbl(Ensemble::new(&INFO, sample_rate)))
}

impl Effect for BaEnsmbl {
    fn set(&mut self, values: &Values, _bpm: f32) {
        self.0.apply(values, &INFO);
    }
    fn process(&mut self, frame: [f32; 2]) -> [f32; 2] {
        self.0.tick(frame)
    }
    fn clear(&mut self) {
        self.0.reset();
    }
}
