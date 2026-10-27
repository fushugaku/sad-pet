//! The supplied glasses5 scene uses the existing Dusty keys factory recording.
//! Its float32 PCM was checked byte-for-byte against the exported .sadpet.
use crate::{
    dsp::{Fades, Region, SampleBuffer},
    preset::Preset,
};
use std::sync::Arc;

pub const INDEX: usize = 5;
pub const SAMPLE_ID: &str = "dusty-keys";
pub const REGION: Region = Region {
    start: 0.0,
    end: 1.0,
};
pub const FADES: Fades = Fades {
    fade_in: 1.0,
    fade_out: 1.0,
};

pub fn preset() -> Preset {
    serde_json::from_str(include_str!("../presets/glasses5.json"))
        .expect("validated glasses5 factory preset")
}

pub fn sample() -> Arc<SampleBuffer> {
    crate::samples::get(SAMPLE_ID).expect("bundled Dusty keys")
}

pub fn configuration() -> serde_json::Value {
    let sample_index = crate::samples::TRACKS
        .iter()
        .position(|s| s.id == SAMPLE_ID)
        .unwrap();
    // The scene opens on its preset as a Rack, which plays it the same.
    serde_json::json!({
        "index": INDEX, "preset": Preset::factory(INDEX), "sample_index": sample_index,
        "sample_id": format!("factory:{SAMPLE_ID}:v1"),
        "sample_name": crate::samples::TRACKS[sample_index].name,
        "region": REGION, "fades": FADES,
    })
}
