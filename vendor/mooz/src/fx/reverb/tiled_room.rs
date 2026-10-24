//! TiledRoom (TLDROOM.ZDL): the shared Hall/Room/Spring/TiledRoom/Arena kernel
//! (`hall_reverb.rs`) with the TLDROOM.ZDL tap, filter, decay and tone tables.
//! Signal flow and editors are described in [`super::hall_family`]; all five
//! editors were decoded (Decay, Tone, Mix, PreD, Level).
use super::hall_family::{HallModel, hall_family_params};
use crate::{Effect, catalog::*};

pub const INFO: ModelInfo = ModelInfo {
    key: "mooz_tiled_room",
    name: "TiledRoom",
    category: Category::Reverb,
    zdl: "TLDROOM.ZDL",
    description: "Bright reverb of a tiled room",
    params: hall_family_params!(19, 4, 46, 9),
};

pub fn create(sample_rate: f32) -> Box<dyn Effect> {
    Box::new(HallModel::new(&super::rom::TILED_ROOM, sample_rate))
}
