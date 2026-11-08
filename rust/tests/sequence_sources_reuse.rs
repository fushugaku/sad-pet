use sad_pet::{
    dsp::{Engine, PreparedPreset, SampleBuffer},
    midi::Message,
    parameters,
    preset::Preset,
    sequence::Edit,
    timbres::Kind,
    waveshaper::Quality,
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::Arc,
};

struct Allocator;
thread_local! {
    static TRACK: Cell<bool> = const { Cell::new(false) };
    static COUNT: Cell<usize> = const { Cell::new(0) };
}
fn count() {
    let _ = TRACK.try_with(|track| {
        if track.get() {
            let _ = COUNT.try_with(|count| count.set(count.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count();
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        count();
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count();
        unsafe { System.realloc(ptr, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

fn no_heap(f: impl FnOnce()) {
    COUNT.with(|count| count.set(0));
    TRACK.with(|track| track.set(true));
    f();
    TRACK.with(|track| track.set(false));
    assert_eq!(COUNT.with(Cell::get), 0, "audio allocated or freed");
}

#[test]
fn reused_queued_and_stale_source_exchanges_do_not_touch_the_heap() {
    let preset = Preset::default();
    let sample = Arc::new(SampleBuffer::demo(8_000));
    let mut engine = Engine::new(8_000, sample, preset.clone());
    let cancelled = engine.prepare_preset_update(preset.clone()).unwrap();
    drop(cancelled);

    let mut reused = engine.prepare_preset_update(preset.clone()).unwrap();
    let mut stale = engine.prepare_preset_update(preset.clone()).unwrap();
    no_heap(|| assert!(engine.exchange_preset(&mut reused)));

    let mut no_sources = preset.clone();
    no_sources.sequencer.tracks.clear();
    let mut remove = engine.prepare_preset_update(no_sources).unwrap();
    no_heap(|| assert!(engine.exchange_preset(&mut remove)));
    no_heap(|| assert!(!engine.exchange_preset(&mut stale)));
    assert!(engine.preset.sequencer.tracks.is_empty());

    for _ in 0..2 {
        let mut queued =
            PreparedPreset::prepare(preset.clone(), &Default::default(), 8_000).unwrap();
        no_heap(|| assert!(engine.exchange_preset(&mut queued)));
    }

    let mut with_bass = preset.clone();
    with_bass.tb303 = Some(Box::default());
    let mut expanded = engine.prepare_preset_update(with_bass).unwrap();
    no_heap(|| assert!(engine.exchange_preset(&mut expanded)));
    let mut without_bass = engine.prepare_preset_update(preset).unwrap();
    no_heap(|| assert!(engine.exchange_preset(&mut without_bass)));
}

#[test]
fn private_grain_clocks_and_full_pool_note_onsets_do_not_touch_the_heap() {
    for rack in [false, true] {
        let mut preset = if rack {
            Kind::Granular.rack_instrument().unwrap()
        } else {
            Preset::default()
        };
        preset.color_chain.clear();
        preset.params.mix = 1.0;
        preset.params.grains = 1.0;
        preset.params.grain_size = 1.0;
        preset.params.grain_attack = 1.0;
        preset.sequencer.enabled = true;
        preset.sequencer.tracks = [60, 72]
            .into_iter()
            .enumerate()
            .map(|(i, pitch)| {
                let mut track = sad_pet::sequence::Track {
                    id: i as u32 + 1,
                    oscillator_mask: 0,
                    granular: true,
                    env_gate: true,
                    ..Default::default()
                };
                track.config[1] = 1;
                track.config[3] = 32;
                track.config[7] = 100;
                track.config[8] = i as i32;
                for step in &mut track.steps {
                    step.pitch = pitch;
                    step.gate = 3;
                }
                track
            })
            .collect();
        let mut engine = Engine::new(8_000, Arc::new(SampleBuffer::demo(8_000)), preset);
        engine.set_midi_gate(true);
        engine.set_playing(true);
        no_heap(|| {
            for frame in 0..16_000 {
                if frame == 10_000 {
                    engine.apply_sequence_edits(&[Edit {
                        track: 2,
                        stage: None,
                        field: 8,
                        value: 0.0,
                    }]);
                }
                assert!(engine.process_frame().into_iter().all(f32::is_finite));
            }
        });
    }
}

/// Compare a bus born with mono-only resources with a dense engine switched
/// to the same bus before rendering. Every frame must stay bit-identical.
#[test]
fn mono_bus_matches_dense_with_midi_automation_and_granular_history() {
    const RATE: u32 = 8_000;
    for granular in [false, true] {
        let sample = Arc::new(SampleBuffer::demo(RATE));
        let mut bus = Kind::Bus.preset().unwrap();
        bus.bus.as_mut().unwrap().granular = granular;
        bus.bus.as_mut().unwrap().filters = true;
        bus.params.filter = 0.52;
        bus.params.filter_resonance = 0.35;
        bus.params.filter_shapers[0].before.enabled = true;
        bus.params.filter_shapers[0].before.mix = 0.7;
        bus.params.filter_shapers[0].before.options.quality = Quality::Clean4x;
        bus.resynth.enabled = false;
        bus.resynth.mix = 0.6;
        bus.resynth.attack = 0.0;
        bus.resynth.threshold = 0.0;
        bus.sequencer.enabled = false;

        let mut dense = Engine::new(RATE, sample.clone(), Preset::default());
        dense.set_preset(bus.clone());
        let mut mono = Engine::new(RATE, sample, bus);
        for engine in [&mut dense, &mut mono] {
            engine.set_midi_gate(true);
            engine.set_playing(true);
            for note in [48, 55, 60, 67] {
                engine.midi(Message::NoteOn {
                    channel: 0,
                    note,
                    velocity: 105,
                });
            }
        }
        let enable = parameters::parse(&serde_json::json!({"resynth_enabled": 1.0})).unwrap();
        let edit = [Edit {
            track: 0,
            stage: None,
            field: 0,
            value: 1.0,
        }];
        let mut energy = 0.0;
        for i in 0..4_096 {
            if i == 256 {
                no_heap(|| {
                    for engine in [&mut dense, &mut mono] {
                        engine.apply_parameters(&enable);
                        engine.apply_sequence_edits(&edit);
                    }
                });
            }
            let x = (i as f32 * std::f32::consts::TAU * 311.0 / RATE as f32).sin() * 0.3;
            let input = [x, x * 0.7];
            let mut a = [0.0; 2];
            let mut b = [0.0; 2];
            no_heap(|| {
                a = dense.process_input_frame(input);
                b = mono.process_input_frame(input);
            });
            assert_eq!(a, b, "bus mismatch at frame {i}, granular={granular}");
            energy += a[0].abs() + a[1].abs();
        }
        assert!(energy > 0.01, "the comparison must contain audible audio");
        let a = dense.resynth_meter();
        let b = mono.resynth_meter();
        assert_eq!(a.bands, b.bands);
        assert_eq!(a.frequencies, b.frequencies);

        // Existing held and MainPitch voices become audible when this same
        // engine changes role. The cold, prepared banks must match a dense
        // engine that has retained its banks throughout the bus interval.
        let mut instrument = mono.preset.clone();
        instrument.bus = None;
        let mut dense_update = dense.prepare_preset_update(instrument.clone()).unwrap();
        let mut mono_update = mono.prepare_preset_update(instrument).unwrap();
        no_heap(|| {
            assert!(dense.exchange_preset(&mut dense_update));
            assert!(mono.exchange_preset(&mut mono_update));
        });
        let mut instrument_energy = 0.0;
        for i in 0..2_048 {
            let mut a = [0.0; 2];
            let mut b = [0.0; 2];
            no_heap(|| {
                a = dense.process_frame();
                b = mono.process_frame();
            });
            assert_eq!(
                a, b,
                "instrument mismatch at frame {i}, granular={granular}"
            );
            instrument_energy += a[0].abs() + a[1].abs();
        }
        assert!(instrument_energy > 0.01, "the new instrument must sound");
    }
}

#[test]
fn bus_to_instrument_prepares_banks_and_exchanges_without_heap() {
    const RATE: u32 = 8_000;
    let sample = Arc::new(SampleBuffer::demo(RATE));
    let bus = Kind::Bus.preset().unwrap();
    let instrument = Preset::default();
    let mut engine = Engine::new(RATE, sample, bus.clone());

    let cancelled = engine.prepare_preset_update(instrument.clone()).unwrap();
    drop(cancelled);
    let mut prepared = engine.prepare_preset_update(instrument).unwrap();
    no_heap(|| assert!(engine.exchange_preset(&mut prepared)));
    no_heap(|| {
        engine.set_midi_gate(true);
        engine.set_playing(true);
        for note in [48, 55, 60, 67] {
            engine.midi(Message::NoteOn {
                channel: 0,
                note,
                velocity: 110,
            });
        }
        for _ in 0..512 {
            assert!(engine.process_frame().into_iter().all(f32::is_finite));
        }
    });
    let mut back = engine.prepare_preset_update(bus).unwrap();
    no_heap(|| assert!(engine.exchange_preset(&mut back)));
    no_heap(|| {
        for _ in 0..128 {
            assert!(
                engine
                    .process_input_frame([0.1, -0.1])
                    .into_iter()
                    .all(f32::is_finite)
            );
        }
    });
}
