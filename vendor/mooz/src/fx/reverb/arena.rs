//! Arena (ARENA.ZDL): the shared Hall/Room/Spring/TiledRoom/Arena kernel
//! (`hall_reverb.rs`) with the ARENA.ZDL tap, filter, decay and tone tables.
//! Signal flow and editors are described in [`super::hall_family`]; all five
//! editors were decoded (Decay, Tone, Mix, PreD, Level).
use super::hall_family::{HallModel, hall_family_params};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_arena",
    name: "Arena",
    category: Category::Reverb,
    zdl: "ARENA.ZDL",
    description: "Reverb of a large arena with long pre-delay",
    params: hall_family_params!(14, 7, 56, 89),
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(HallModel::new(&super::rom::ARENA, sample_rate))
}
