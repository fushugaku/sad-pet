use sad_pet::{
    dsp::{Engine, PreparedPreset, SampleBuffer},
    preset::Preset,
    wavetable::assets::Pool,
};
use std::sync::Arc;

fn engine(rate: u32) -> Engine {
    Engine::new(rate, Arc::new(SampleBuffer::demo(rate)), Preset::default())
}

#[test]
fn preset_model_keeps_its_golden_wire_shape_and_roundtrips_without_runtime_state() {
    let preset = Preset::default();
    let value = serde_json::to_value(&preset).unwrap();
    let mut keys: Vec<_> = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "color_chain",
            "drums",
            "env_count",
            "envs",
            "filter_slots",
            "format",
            "fx_chain_version",
            "lfo_count",
            "lfos",
            "matrix",
            "name",
            "params",
            "performance",
            "pre_color_chain",
            "resynth",
            "sequencer",
            "source_color_chain",
        ]
    );
    assert_eq!(value["format"], "evil-pet-rust-v1");
    assert!(value.get("wavetable_pool").is_none());
    assert!(value.get("prepared").is_none());
    assert!(value.get("runtime").is_none());

    let decoded: Preset = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(&decoded).unwrap(), value);
    assert_eq!(serde_json::to_value(preset.clone()).unwrap(), value);
}

#[test]
fn preparing_a_cloned_model_is_an_explicit_separate_step() {
    let model = Preset::default();
    let clone = model.clone();
    assert_eq!(
        serde_json::to_value(&clone).unwrap(),
        serde_json::to_value(&model).unwrap()
    );

    let prepared = PreparedPreset::prepare(clone, &Pool::default(), 48_000).unwrap();
    assert_eq!(prepared.sample_rate(), 48_000);
    assert_eq!(
        serde_json::to_value(prepared.preset()).unwrap(),
        serde_json::to_value(model).unwrap()
    );
}

#[test]
fn a_rate_mismatch_preserves_the_update_for_a_matching_engine() {
    let mut model = Preset::default();
    model.name = "Prepared for 48 kHz".into();
    let expected = serde_json::to_value(&model).unwrap();
    let mut prepared = PreparedPreset::prepare(model, &Pool::default(), 48_000).unwrap();

    let mut wrong_rate = engine(44_100);
    let before_wrong = serde_json::to_value(&wrong_rate.preset).unwrap();
    assert!(!wrong_rate.exchange_preset(&mut prepared));
    assert_eq!(
        serde_json::to_value(&wrong_rate.preset).unwrap(),
        before_wrong
    );
    assert_eq!(serde_json::to_value(prepared.preset()).unwrap(), expected);

    let mut matching_rate = engine(48_000);
    let retired = serde_json::to_value(&matching_rate.preset).unwrap();
    assert!(matching_rate.exchange_preset(&mut prepared));
    assert_eq!(matching_rate.preset.name, "Prepared for 48 kHz");
    assert_eq!(serde_json::to_value(prepared.preset()).unwrap(), retired);
}

#[test]
fn an_exchanged_retirement_envelope_cannot_be_applied_again() {
    let mut model = Preset::default();
    model.name = "Installed once".into();
    let mut prepared = PreparedPreset::prepare(model, &Pool::default(), 48_000).unwrap();
    let mut first_engine = engine(48_000);
    let original = serde_json::to_value(&first_engine.preset).unwrap();

    assert!(first_engine.exchange_preset(&mut prepared));
    let installed = serde_json::to_value(&first_engine.preset).unwrap();
    assert_eq!(first_engine.preset.name, "Installed once");
    assert_eq!(serde_json::to_value(prepared.preset()).unwrap(), original);

    assert!(!first_engine.exchange_preset(&mut prepared));
    assert_eq!(
        serde_json::to_value(&first_engine.preset).unwrap(),
        installed
    );

    let mut second_engine = engine(48_000);
    let second_before = serde_json::to_value(&second_engine.preset).unwrap();
    assert!(!second_engine.exchange_preset(&mut prepared));
    assert_eq!(
        serde_json::to_value(&second_engine.preset).unwrap(),
        second_before
    );
}
