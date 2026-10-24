//! Hall (HALL.ZDL): the shared Hall/Room/Spring/TiledRoom/Arena kernel
//! (`hall_reverb.rs`) with the HALL.ZDL tap, filter, decay and tone tables.
//! Signal flow and editors are described in [`super::hall_family`]; all five
//! editors were decoded (Decay, Tone, Mix, PreD, Level).
use super::hall_family::{HallModel, hall_family_params};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_hall",
    name: "Hall",
    category: Category::Reverb,
    zdl: "HALL.ZDL",
    description: "Hall reverb with pre-delay and tone control",
    params: hall_family_params!(9, 5, 46, 48),
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(HallModel::new(&super::rom::HALL, sample_rate))
}
