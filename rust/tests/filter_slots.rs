//! Filters 3 and up: positions, bypass, presets, host IDs, modulation and the
//! allocation-free audio path.
use sad_pet::{
    dsp::Engine,
    midi::Message,
    parameters,
    preset::{
        Destination, EnvSource, FilterModel, FilterPosition, FilterSlot, FilterSlotParameter,
        FilterType, MAX_FILTER_SLOTS, Preset,
    },
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
    let mut next = next.prepared_clone_for(RATE);
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
