//! Shared preset policies, independent of JSON routing and audio preparation.
use sad_pet::{
    application::presets::{LoadError, Target, TempoPolicy, load_into_timbre},
    preset::Preset,
    rack::Module,
    timbres::Kind,
};
use serde_json::json;

fn target(kind: Kind) -> Target {
    Target {
        kind,
        tempo: TempoPolicy::FromPreset,
    }
}

fn rack() -> Preset {
    let mut preset: Preset = serde_json::from_value(json!({
        "rack":{"modules":[{"module":"vocal"},{"module":"sampler"},{"module":"mixer"}]},
        "vocal":{"score":{"notes":[{"id":7,"start":0,"length":480,"key":64,"lyric":"ra"}]}},
        "sampler":{"pads":[{"id":3,"name":"My pad"}]}
    }))
    .unwrap();
    preset.sanitize();
    preset
}

#[test]
fn module_edits_inherit_content_but_explicit_content_replaces_it() {
    let current = rack();
    let mut incoming = current.clone();
    incoming.vocal = None;
    incoming.sampler = None;
    incoming.params.master_volume = 0.42;
    let loaded = load_into_timbre(&current, incoming, target(Kind::Rack)).unwrap();
    assert_eq!(loaded.vocal, current.vocal);
    assert_eq!(loaded.sampler, current.sampler);
    assert_eq!(loaded.params.master_volume, 0.42);
    assert_eq!(Kind::of(&loaded), Kind::Rack);

    let mut replacement = current.clone();
    replacement.vocal.as_mut().unwrap().score.notes[0].lyric = "la".into();
    replacement.sampler.as_mut().unwrap().pads[0].name = "New pad".into();
    let loaded = load_into_timbre(&current, replacement, target(Kind::Rack)).unwrap();
    assert_eq!(loaded.vocal.unwrap().score.notes[0].lyric, "la");
    assert_eq!(loaded.sampler.unwrap().pads[0].name, "New pad");
    assert_eq!(current.vocal.unwrap().score.notes[0].lyric, "ra");
    assert_eq!(current.sampler.unwrap().pads[0].name, "My pad");
}

#[test]
fn removing_modules_removes_their_content_even_when_it_was_inherited() {
    let current = rack();
    let mut incoming = current.clone();
    incoming
        .rack
        .as_mut()
        .unwrap()
        .modules
        .retain(|s| !matches!(s.module, Module::Vocal | Module::Sampler));
    incoming.vocal = None;
    incoming.sampler = None;
    let loaded = load_into_timbre(&current, incoming, target(Kind::Rack)).unwrap();
    assert!(loaded.vocal.is_none());
    assert!(loaded.sampler.is_none());
    assert!(loaded.rack.is_some());
}

#[test]
fn dedicated_voice_and_kit_keep_their_content_through_a_sound_load() {
    for kind in [Kind::Vocal, Kind::Sampler] {
        let mut current = kind.preset().unwrap();
        if kind == Kind::Vocal {
            current.vocal = rack().vocal;
        } else {
            current.sampler = rack().sampler;
        }
        let loaded = load_into_timbre(&current, Preset::default(), target(kind)).unwrap();
        assert_eq!(Kind::of(&loaded), kind);
        assert_eq!(loaded.vocal, current.vocal);
        assert_eq!(loaded.sampler, current.sampler);
    }
}

#[test]
fn routing_roles_survive_loading_every_instrument_template() {
    for role in [Kind::Bus, Kind::Master, Kind::PadFx] {
        let current = role.preset().unwrap();
        for source in [Kind::Rack, Kind::Oscillators, Kind::Wavetable, Kind::Tb303] {
            let incoming = source.preset().unwrap();
            let loaded = load_into_timbre(&current, incoming, target(role)).unwrap();
            assert_eq!(loaded.bus.as_ref().unwrap().master, role == Kind::Master);
            assert!(loaded.rack.is_none(), "{role:?} became a {source:?} rack");
            assert!(loaded.control.is_none());
            assert!(loaded.vocal.is_none());
            assert!(loaded.sampler.is_none());
            assert!(loaded.oscillators.is_none());
            assert!(loaded.wavetable.is_none());
        }
    }
}

#[test]
fn ordinary_instruments_do_not_acquire_routing_or_voice_roles() {
    let current = rack();
    for role in [Kind::Master, Kind::Bus, Kind::Vocal, Kind::Sampler] {
        let incoming = role.preset().unwrap();
        let loaded = load_into_timbre(&current, incoming, target(Kind::Rack)).unwrap();
        assert!(loaded.bus.is_none());
        assert!(loaded.vocal.is_none());
        assert!(loaded.sampler.is_none());
        assert_eq!(Kind::of(&loaded), Kind::Granular);
    }
}

#[test]
fn keeping_a_routing_role_retires_references_to_discarded_oscillators() {
    use sad_pet::interaction::{Node, Route};
    use sad_pet::{color::ColorSlot, preset::SaturatorType};
    let current = Kind::Master.preset().unwrap();
    let mut incoming = Kind::Oscillators.preset().unwrap();
    let id = incoming.oscillators.as_ref().unwrap().slots[0].id;
    incoming.interactions.routes.push(Route {
        source: Node::Osc { id },
        target: Node::Group { index: 0 },
        ring: 0.5,
        ..Default::default()
    });
    let mut effect = ColorSlot::legacy(SaturatorType::Distortion, 0.37);
    effect.settings.distortion_drive = 0.61;
    incoming.color_chain.push(effect);
    let mut loaded = load_into_timbre(&current, incoming, target(Kind::Master)).unwrap();
    assert!(!loaded.interactions.routes[0].enabled);
    assert_eq!(loaded.color_chain[0].mix, 0.37);
    assert_eq!(loaded.color_chain[0].settings.distortion_drive, 0.61);
    let saved = serde_json::to_value(&loaded).unwrap();
    loaded.sanitize();
    assert_eq!(serde_json::to_value(&loaded).unwrap(), saved);
    let restored: Preset = serde_json::from_value(saved).unwrap();
    assert_eq!(Kind::of(&restored), Kind::Master);
}

#[test]
fn a_session_tempo_overrides_the_loaded_sound_and_must_be_valid() {
    let current = Preset::default();
    let mut incoming = Preset::default();
    incoming.params.bpm = 93.0;
    let loaded = load_into_timbre(&current, incoming.clone(), target(Kind::Granular)).unwrap();
    assert_eq!(loaded.params.bpm, 93.0);
    let loaded = load_into_timbre(
        &current,
        incoming.clone(),
        Target {
            kind: Kind::Granular,
            tempo: TempoPolicy::Session(137.0),
        },
    )
    .unwrap();
    assert_eq!(loaded.params.bpm, 137.0);

    for tempo in [f32::NAN, f32::INFINITY, 19.0, 301.0] {
        let error = load_into_timbre(
            &current,
            incoming.clone(),
            Target {
                kind: Kind::Granular,
                tempo: TempoPolicy::Session(tempo),
            },
        )
        .unwrap_err();
        assert_eq!(error, LoadError::InvalidSessionTempo);
    }
    assert_eq!(current.params.bpm, 120.0);
}

#[test]
fn loading_a_definition_does_not_require_control_side_asset_originals() {
    let current = rack();
    let before = serde_json::to_value(&current).unwrap();
    let mut incoming = Kind::Wavetable.preset().unwrap();
    let id = format!("user:{}", "0".repeat(64));
    incoming.wavetable.as_mut().unwrap().oscillators[0].asset_id = id.clone();
    // Browser audio owns staged banks rather than the utility worker's originals.
    // Availability is validated by each host's preparation, after this policy.
    let loaded = load_into_timbre(&current, incoming, target(Kind::Rack)).unwrap();
    assert_eq!(
        loaded.wavetable.as_ref().unwrap().oscillators[0].asset_id,
        id
    );
    assert_eq!(serde_json::to_value(&current).unwrap(), before);
}
