//! Filters 3 and up: positions, bypass, presets, host IDs, modulation and the
//! allocation-free audio path.
use sad_pet::{
    color::{ColorSettings, ColorSlot},
    dsp::Engine,
    midi::Message,
    parameters,
    preset::{
        Destination, EnvSource, FilterFxChain, FilterModel, FilterPosition, FilterSlot,
        FilterSlotParameter, FilterType, MAX_FILTER_SLOTS, Preset, SaturatorType,
    },
    rack::{Module, Settings, Slot},
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
};

struct Allocator;
thread_local! {
    static TRACK: Cell<bool> = const { Cell::new(false) };
    static COUNT: Cell<usize> = const { Cell::new(0) };
}
fn count() {
    let _ = TRACK.try_with(|t| {
        if t.get() {
            let _ = COUNT.try_with(|c| c.set(c.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        count();
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        count();
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        count();
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
fn no_heap(f: impl FnOnce()) {
    COUNT.with(|c| c.set(0));
    TRACK.with(|c| c.set(true));
    f();
    TRACK.with(|c| c.set(false));
    assert_eq!(COUNT.with(Cell::get), 0, "audio allocated or freed storage");
}

const RATE: u32 = 48_000;

/// A dark low-pass that audibly changes whatever passes through it.
fn dark(id: u32, position: FilterPosition) -> FilterSlot {
    FilterSlot {
        id,
        position,
        cutoff: 0.05,
        resonance: 0.0,
        ..FilterSlot::default()
    }
}

/// The first-open scene: grains, four Resynth oscillators, Pre FX and FX.
fn scene(slots: &[FilterSlot]) -> Preset {
    let mut preset = sad_pet::startup::preset();
    preset.filter_slots = slots.to_vec();
    preset
}

fn play(preset: Preset, notes: [u8; 3]) -> Engine {
    let mut e = Engine::new(RATE, sad_pet::startup::sample(), preset);
    e.set_fades(sad_pet::startup::FADES).unwrap();
    e.set_midi_gate(true);
    e.set_playing(true);
    for note in notes {
        e.midi(Message::NoteOn {
            channel: 0,
            note,
            velocity: 100,
        });
    }
    e
}

fn engine(slots: &[FilterSlot]) -> Engine {
    play(scene(slots), [48, 60, 67])
}

const SHARED: [FilterPosition; 3] = [
    FilterPosition::AfterOsc1,
    FilterPosition::AfterResynth,
    FilterPosition::AfterFx,
];

#[test]
fn filter_slot_parameter_registry_keeps_control_ranges_and_sanitization() {
    let parameters = FilterSlotParameter::ALL;
    assert_eq!(
        parameters.map(FilterSlotParameter::key),
        [
            "cutoff",
            "resonance",
            "drive",
            "morph",
            "env_depth",
            "keytrack"
        ]
    );

    let mut slot = FilterSlot {
        cutoff: f32::NAN,
        resonance: f32::NEG_INFINITY,
        drive: 2.0,
        morph: -1.0,
        env_depth: 3.0,
        keytrack: -1.0,
        ..FilterSlot::default()
    };
    for (parameter, range) in parameters.into_iter().zip([
        (0.0, 1.0),
        (0.0, 1.0),
        (0.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (0.0, 1.0),
    ]) {
        assert_eq!(parameter.range(), range);
        let (_, min, max) = slot.parameter_mut(parameter);
        assert_eq!((min, max), range);
    }

    slot.sanitize();
    assert_eq!(slot.cutoff, FilterSlot::default().cutoff);
    assert_eq!(slot.resonance, FilterSlot::default().resonance);
    assert_eq!(slot.drive, 1.0);
    assert_eq!(slot.morph, 0.0);
    assert_eq!(slot.env_depth, 1.0);
    assert_eq!(slot.keytrack, 0.0);
}

fn render(e: &mut Engine, frames: usize) -> Vec<u32> {
    let mut out = vec![[0.0; 2]; frames];
    e.process(&mut out);
    out.into_iter().flatten().map(f32::to_bits).collect()
}

#[test]
fn every_position_changes_the_sound_and_bypass_is_transparent() {
    // Long enough for Resynth's bands to open behind the grains.
    let frames = 24_000;
    let reference = render(&mut engine(&[]), frames);
    for position in FilterPosition::ALL {
        let filtered = render(&mut engine(&[dark(1, position)]), frames);
        assert_ne!(filtered, reference, "{position:?} did not change the sound");
        let bypassed = render(
            &mut engine(&[FilterSlot {
                enabled: false,
                ..dark(1, position)
            }]),
            frames,
        );
        assert_eq!(
            bypassed, reference,
            "bypassed {position:?} changed the sound"
        );
    }
}

#[test]
fn presets_keep_added_filters_and_sanitize_them() {
    let mut preset = Preset {
        filter_slots: (0..MAX_FILTER_SLOTS + 3)
            .map(|i| FilterSlot {
                id: if i < 2 { 7 } else { 0 },
                position: FilterPosition::ALL[i % FilterPosition::ALL.len()],
                model: FilterModel::LadderLp24,
                kind: FilterType::BandPass,
                cutoff: f32::NAN,
                env_depth: -3.0,
                ..FilterSlot::default()
            })
            .collect(),
        ..Preset::default()
    };
    preset.lfos[0].destination = Destination::FilterSlot {
        slot: 7,
        parameter: FilterSlotParameter::Cutoff,
    };
    preset.lfos[1].destination = Destination::FilterSlot {
        slot: 999,
        parameter: FilterSlotParameter::Drive,
    };
    preset.sanitize();
    assert_eq!(preset.filter_slots.len(), MAX_FILTER_SLOTS);
    let mut ids: Vec<_> = preset.filter_slots.iter().map(|s| s.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), MAX_FILTER_SLOTS, "slot ids must be unique");
    assert!(ids.iter().all(|&id| id != 0));
    assert_eq!(preset.filter_slots[0].id, 7);
    assert!(
        preset
            .filter_slots
            .iter()
            .all(|s| s.cutoff == 0.7 && s.env_depth == -1.0)
    );
    assert!(matches!(
        preset.lfos[0].destination,
        Destination::FilterSlot { slot: 7, .. }
    ));
    assert_eq!(preset.lfos[1].destination, Destination::None);

    let json = serde_json::to_value(&preset).unwrap();
    assert_eq!(json["filter_slots"][1]["position"], "after_osc1");
    assert_eq!(json["filter_slots"][1]["model"], "ladder_lp24");
    let restored: Preset = serde_json::from_value(json).unwrap();
    assert_eq!(restored.filter_slots, preset.filter_slots);
    assert_eq!(restored.lfos[0].destination, preset.lfos[0].destination);
    // Presets from before added filters load with none.
    let old: Preset = serde_json::from_str(r#"{"format":"evil-pet-rust-v1"}"#).unwrap();
    assert!(old.filter_slots.is_empty());
}

#[test]
fn fx_filter_targets_recall_and_old_after_fx_means_b_last() {
    let old: FilterSlot = serde_json::from_str(r#"{"id":1,"position":"after_fx"}"#).unwrap();
    assert_eq!(old.fx_chain, FilterFxChain::B);
    assert_eq!(old.fx_slot, None);
    let mut preset = fx_scene(FilterFxChain::A, None);
    preset.filter_slots[0].fx_slot = Some(11);
    let restored: Preset = serde_json::from_value(serde_json::to_value(&preset).unwrap()).unwrap();
    assert_eq!(restored.filter_slots, preset.filter_slots);
    preset.pre_color_chain.reverse();
    preset.sanitize();
    assert_eq!(preset.filter_slots[0].fx_slot, Some(11));
    preset.filter_slots[0].fx_chain = FilterFxChain::B;
    preset.sanitize();
    assert_eq!(
        preset.filter_slots[0].fx_slot, None,
        "An effect in another rack is not a target"
    );
    preset.filter_slots[0].fx_slot = Some(1);
    preset.color_chain.push(ColorSlot {
        id: 0,
        ..ColorSlot::default()
    });
    preset.sanitize();
    assert_eq!(preset.filter_slots[0].fx_slot, None);
    assert!(
        preset.color_chain.iter().all(|fx| fx.id != 1),
        "A new FX must not inherit a stale target ID"
    );
}

fn fx_scene(chain: FilterFxChain, modules: Option<&[Module]>) -> Preset {
    let mut preset = scene(&[FilterSlot {
        fx_chain: chain,
        ..dark(1, FilterPosition::AfterFx)
    }]);
    preset.params.mix = 0.0;
    preset.resynth.enabled = false;
    preset.fx_chain_version = 1;
    preset.color_chain.clear();
    preset.pre_color_chain.clear();
    preset.source_color_chain.clear();
    let effects = vec![
        ColorSlot {
            id: 11,
            kind: SaturatorType::Distortion,
            mix: 1.0,
            settings: ColorSettings {
                distortion_drive: 0.9,
                ..Default::default()
            },
            ..ColorSlot::default()
        },
        ColorSlot {
            id: 12,
            kind: SaturatorType::Crusher,
            mix: 1.0,
            settings: ColorSettings {
                crusher_bits: 0.3,
                crusher_drive: 0.8,
                crusher_dither: 0.0,
                ..Default::default()
            },
            ..ColorSlot::default()
        },
    ];
    match chain {
        FilterFxChain::A => preset.pre_color_chain = effects,
        FilterFxChain::B => preset.color_chain = effects,
    }
    if let Some(modules) = modules {
        preset.rack = Some(Box::new(Settings {
            modules: modules
                .iter()
                .map(|&module| Slot {
                    module,
                    enabled: true,
                    id: 0,
                })
                .collect(),
            ..Default::default()
        }));
    }
    preset.sanitize();
    preset
}

fn fx_chain(preset: &mut Preset, chain: FilterFxChain) -> &mut Vec<ColorSlot> {
    match chain {
        FilterFxChain::A => &mut preset.pre_color_chain,
        FilterFxChain::B => &mut preset.color_chain,
    }
}

#[test]
fn fx_filters_run_between_effects_follow_ids_and_keep_bypass_and_last() {
    let sound = |preset| render(&mut play(preset, [48, 60, 67]), 4096);
    let per_note = [Module::Sample, Module::PreFx, Module::PostFx];
    let shared = [Module::Sample, Module::Mixer, Module::PreFx, Module::PostFx];
    for modules in [None, Some(per_note.as_slice()), Some(shared.as_slice())] {
        for chain in [FilterFxChain::A, FilterFxChain::B] {
            let mut preset = fx_scene(chain, modules);
            let last = sound(preset.clone());
            preset.filter_slots[0].fx_slot = Some(12);
            assert_eq!(
                sound(preset.clone()),
                last,
                "Last must match the final effect"
            );
            preset.filter_slots[0].fx_slot = Some(11);
            assert_ne!(
                sound(preset.clone()),
                last,
                "The first FX target ran at Last ({chain:?}, {modules:?})"
            );
            fx_chain(&mut preset, chain).reverse();
            let moved = sound(preset.clone());
            preset.filter_slots[0].fx_slot = None;
            assert_eq!(
                sound(preset.clone()),
                moved,
                "A reordered effect lost its filter"
            );
            fx_chain(&mut preset, chain).reverse();
            fx_chain(&mut preset, chain)[0].enabled = false;
            let bypassed_last = sound(preset.clone());
            preset.filter_slots[0].fx_slot = Some(11);
            assert_ne!(
                sound(preset.clone()),
                bypassed_last,
                "A bypassed FX lost its filter point"
            );
            fx_chain(&mut preset, chain).clear();
            preset.filter_slots[0].fx_slot = None;
            let empty_last = sound(preset.clone());
            preset.filter_slots[0].enabled = false;
            assert_ne!(sound(preset), empty_last, "Last skipped an empty rack");
        }
    }
}

#[test]
fn empty_fx_a_filters_survive_rack_conversion_and_run_without_heap() {
    let mut preset = fx_scene(FilterFxChain::A, None);
    preset.pre_color_chain.clear();
    let rack = preset.as_rack().unwrap();
    assert!(rack.rack.as_ref().unwrap().has(Module::PreFx));
    assert_eq!(
        render(&mut play(preset, [48, 60, 67]), 4096),
        render(&mut play(rack, [48, 60, 67]), 4096)
    );
    let mut preset = fx_scene(FilterFxChain::A, None);
    preset.filter_slots[0].fx_slot = Some(11);
    let mut engine = play(preset, [48, 60, 67]);
    let mut next = engine.preset.clone();
    next.pre_color_chain.reverse();
    let mut next = engine.prepare_preset_update(next).unwrap();
    let mut out = [[0.0; 2]; 2048];
    no_heap(|| {
        engine.process(&mut out);
        engine.exchange_preset(&mut next);
        engine.process(&mut out);
    });
    assert!(out.iter().flatten().all(|v| v.is_finite()));
}

#[test]
fn fx_last_filters_send_the_same_normalized_audio_as_their_rack() {
    let per_note = [Module::Sample, Module::PreFx, Module::PostFx];
    let shared = [Module::Sample, Module::Mixer, Module::PreFx, Module::PostFx];
    for modules in [None, Some(per_note.as_slice()), Some(shared.as_slice())] {
        for chain in [FilterFxChain::A, FilterFxChain::B] {
            let mut preset = fx_scene(chain, modules);
            preset.filter_slots[0].output = sad_pet::taps::Tap {
                bus: 1,
                send: 1.0,
                chain: 1.0,
            };
            let tap = match chain {
                FilterFxChain::A => &mut preset.taps.pre_fx,
                FilterFxChain::B => &mut preset.taps.post_fx,
            };
            tap.bus = 2;
            tap.send = 1.0;
            let mut engine = play(preset, [48, 60, 67]);
            engine.set_open_buses(1 << 1 | 1 << 2);
            let mut peak = 0.0_f32;
            for _ in 0..4096 {
                engine.process_frame();
                let sends = engine.bus_sends().unwrap();
                assert_eq!(
                    sends[0], sends[1],
                    "Wrong FX filter send gain ({chain:?}, {modules:?})"
                );
                peak = peak.max(sends[0][0].abs());
            }
            assert!(peak > 1e-6, "FX filter send is silent");
        }
    }
}

#[test]
fn fx_last_and_final_effect_share_filter_list_order() {
    for chain in [FilterFxChain::A, FilterFxChain::B] {
        let mut preset = fx_scene(chain, None);
        preset.filter_slots.push(FilterSlot {
            fx_chain: chain,
            model: FilterModel::LadderLp24,
            drive: 0.8,
            cutoff: 0.5,
            ..dark(2, FilterPosition::AfterFx)
        });
        let reference = render(&mut play(preset.clone(), [48, 60, 67]), 4096);
        preset.filter_slots[1].fx_slot = Some(12);
        assert_eq!(render(&mut play(preset, [48, 60, 67]), 4096), reference);
    }
}

#[test]
fn host_parameters_address_added_filters_by_position() {
    let specs = parameters::specs();
    let buffer = specs
        .iter()
        .position(|s| s.id == "drum_buffer_seconds")
        .unwrap();
    // Appended after every existing control, so host indices do not move.
    assert_eq!(specs[buffer + 1].id, "filter3_cutoff");
    assert_eq!(specs[buffer + 8 * MAX_FILTER_SLOTS].id, "filter18_response");
    // ENV types follow the added filters.
    assert_eq!(specs[buffer + 1 + 8 * MAX_FILTER_SLOTS].id, "env1_mode");

    let mut preset = Preset {
        filter_slots: vec![
            dark(4, FilterPosition::AfterFx),
            dark(9, FilterPosition::AfterGrains),
        ],
        ..Preset::default()
    };
    let changes = parameters::parse(&serde_json::json!({
        "filter4_cutoff": 0.25, "filter4_model": 2.0, "filter4_response": 2.0,
        "filter3_env_depth": -0.5,
    }))
    .unwrap();
    assert!(parameters::apply(&mut preset, &changes));
    let second = preset.filter_slots[1];
    assert_eq!(second.id, 9);
    assert_eq!(second.cutoff, 0.25);
    assert_eq!(second.model, FilterModel::LadderLp24);
    assert_eq!(second.kind, FilterType::HighPass);
    assert_eq!(preset.filter_slots[0].env_depth, -0.5);
    // A position without a filter is ignored rather than created.
    let missing = parameters::parse(&serde_json::json!({"filter12_cutoff": 0.5})).unwrap();
    parameters::apply(&mut preset, &missing);
    assert_eq!(preset.filter_slots.len(), 2);
}

#[test]
fn modulation_routes_reach_added_filters() {
    let frames = 6000;
    let slot = FilterSlot {
        resonance: 0.6,
        ..dark(3, FilterPosition::AfterFx)
    };
    let routed = |depth: f32| {
        let mut preset = sad_pet::startup::preset();
        preset.filter_slots = vec![slot];
        preset.lfos[0].destination = Destination::FilterSlot {
            slot: 3,
            parameter: FilterSlotParameter::Cutoff,
        };
        preset.lfos[0].depth = depth;
        preset.lfos[0].rate_hz = 3.0;
        let mut e = Engine::new(RATE, sad_pet::startup::sample(), preset);
        e.set_midi_gate(true);
        e.set_playing(true);
        e.midi(Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 100,
        });
        render(&mut e, frames)
    };
    assert_ne!(routed(0.8), routed(0.0));
}

#[test]
fn shared_filters_follow_the_loudest_envelope_like_an_env_route() {
    let frames = 24_000;
    for position in SHARED {
        let render_with = |env_depth: f32, routed: f32| {
            let mut preset = scene(&[FilterSlot {
                env: EnvSource::Env1,
                env_depth,
                ..dark(1, position)
            }]);
            preset.envs[0].extra_destination = Destination::FilterSlot {
                slot: 1,
                parameter: FilterSlotParameter::Cutoff,
            };
            preset.envs[0].extra_depth = routed;
            render(&mut play(preset, [48, 60, 67]), frames)
        };
        let opened = render_with(0.8, 0.0);
        assert_ne!(
            opened,
            render_with(0.0, 0.0),
            "{position:?} ignored ENV DEPTH"
        );
        assert_eq!(
            opened,
            render_with(0.0, 0.8),
            "{position:?} ENV DEPTH differs from routing the ENV to its cutoff"
        );
    }
}

#[test]
fn shared_filters_track_the_last_note() {
    let frames = 24_000;
    for position in SHARED {
        let render_with = |notes: [u8; 3], keytrack: f32| {
            let mut preset = scene(&[FilterSlot {
                keytrack,
                ..dark(1, position)
            }]);
            // Sequencer steps are notes too; keep the chord's last note last.
            preset.sequencer.enabled = false;
            render(&mut play(preset, notes), frames)
        };
        // Ending on C4, full tracking adds nothing; ending a fifth above, it
        // opens the filter although C4 is still held.
        assert_eq!(
            render_with([48, 67, 60], 1.0),
            render_with([48, 67, 60], 0.0),
            "{position:?} tracked a note other than the last"
        );
        assert_ne!(
            render_with([48, 60, 67], 1.0),
            render_with([48, 60, 67], 0.0),
            "{position:?} ignored KEYTRACK"
        );
    }
}

#[test]
fn added_filters_render_and_update_without_heap() {
    let slots = [
        dark(1, FilterPosition::AfterGrains),
        dark(2, FilterPosition::AfterOsc1),
        dark(3, FilterPosition::AfterResynth),
        dark(4, FilterPosition::BeforeFilters),
        dark(5, FilterPosition::AfterFilters),
        dark(6, FilterPosition::AfterFx),
    ];
    let mut e = engine(&slots);
    let mut out = vec![[0.0; 2]; 2048];
    // Knob and host edits, including model, response and the per-voice
    // controls, reuse every bank as prepared.
    let changes = parameters::parse(&serde_json::json!({
        "filter3_cutoff": 0.6, "filter4_model": 5.0, "filter5_response": 3.0,
        "filter6_keytrack": 1.0, "filter8_resonance": 0.9,
    }))
    .unwrap();
    // A structural update prepared off the audio thread: a slot moved to
    // another position, one removed and a new one added.
    let mut next = e.preset.clone();
    next.filter_slots[0].position = FilterPosition::AfterOsc3;
    next.filter_slots.remove(2);
    next.filter_slots
        .push(dark(11, FilterPosition::AfterGrains));
    let mut next = e.prepare_preset_update(next).unwrap();
    no_heap(|| {
        e.process(&mut out);
        e.apply_parameters(&changes);
        e.process(&mut out);
        e.exchange_preset(&mut next);
        e.process(&mut out);
    });
    assert!(out.iter().flatten().all(|v| v.is_finite()));
    assert_eq!(e.preset.filter_slots.len(), 6);
}
