//! External-envelope routing for the waveshaper scalar path.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use sad_pet::{
    Stereo,
    color::{ColorSlot, WaveshaperMode},
    dsp::{
        Engine, SampleBuffer,
        waveshaper::{Processor, StageProcessor},
    },
    midi::Message,
    preset::{EnvSource, Preset, SaturatorType},
    rack::{Module, Settings as RackSettings, Slot},
    waveshaper::{Options, Stage},
};

struct CountingAllocator;

static TRACK_ALLOCATIONS: AtomicBool = AtomicBool::new(false);
static ALLOCATION_OPERATIONS: AtomicUsize = AtomicUsize::new(0);

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if TRACK_ALLOCATIONS.load(Ordering::Relaxed) {
            ALLOCATION_OPERATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if TRACK_ALLOCATIONS.load(Ordering::Relaxed) {
            ALLOCATION_OPERATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if TRACK_ALLOCATIONS.load(Ordering::Relaxed) {
            ALLOCATION_OPERATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.realloc(ptr, layout, size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if TRACK_ALLOCATIONS.load(Ordering::Relaxed) {
            ALLOCATION_OPERATIONS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn options(source: EnvSource, amount: f32) -> Options {
    Options {
        envelope: amount,
        envelope_source: Some(source),
        ..Options::default()
    }
}

fn render(processor: &mut Processor, source: &[f32; 9]) -> Vec<Stereo> {
    let mut output = Vec::with_capacity(512);
    processor.set_envelopes(source);
    for sample in 0..512 {
        let x = 0.18 * (std::f32::consts::TAU * sample as f32 / 48.0).sin();
        output.push(processor.process([x, -x]));
    }
    output
}

fn stale_file_source_cache_is_ignored_after_switching_to_rack() {
    let sample = std::sync::Arc::new(SampleBuffer::demo(8_000));
    let source_fx = ColorSlot::legacy(SaturatorType::Distortion, 1.0);
    let classic = Preset {
        source_color_chain: vec![source_fx],
        ..Preset::default()
    };
    let rack_preset = Preset {
        rack: Some(Box::new(RackSettings {
            modules: [
                Module::Sample,
                Module::Grains,
                Module::SourceFx,
                Module::Mixer,
            ]
            .map(|module| Slot {
                module,
                ..Slot::default()
            })
            .to_vec(),
            ..RackSettings::default()
        })),
        ..classic.clone()
    };

    let mut stale = Engine::new(8_000, sample.clone(), classic);
    stale.prepare_source_effects().unwrap();
    assert!(stale.processed_source.is_some());
    stale.set_preset(rack_preset.clone());
    assert!(!stale.reads_source_cache());
    assert!(
        stale.processed_source.is_some(),
        "the scenario needs the old FileSource cache to remain present"
    );

    let mut fresh = Engine::new(8_000, sample, rack_preset);
    for engine in [&mut stale, &mut fresh] {
        engine.midi(Message::NoteOn {
            channel: 0,
            note: 72,
            velocity: 100,
        });
    }
    let mut stale_pcm = vec![[0.0; 2]; 2_048];
    let mut fresh_pcm = vec![[0.0; 2]; 2_048];
    stale.process(&mut stale_pcm);
    fresh.process(&mut fresh_pcm);
    assert!(
        stale_pcm
            .iter()
            .any(|frame| frame[0] != 0.0 || frame[1] != 0.0)
    );
    assert_eq!(
        stale_pcm, fresh_pcm,
        "a stale cache must not double-run live FX"
    );
}

#[test]
fn external_sources_are_independent_neutral_at_half_and_realtime_allocation_free() {
    let mut owner_envs_a = [0.0; 9];
    let mut owner_envs_b = [0.0; 9];
    owner_envs_a[0] = 0.0;
    owner_envs_a[1] = 1.0;
    owner_envs_b[0] = 1.0;
    owner_envs_b[1] = 0.0;

    let mut env1 = Processor::new(48_000);
    let mut env2 = Processor::new(48_000);
    env1.reset_for_note(
        WaveshaperMode::Drive,
        0.65,
        options(EnvSource::Env1, 1.0),
        60.0,
    );
    env2.reset_for_note(
        WaveshaperMode::Drive,
        0.65,
        options(EnvSource::Env2, 1.0),
        60.0,
    );
    let output1 = render(&mut env1, &owner_envs_a);
    let output2 = render(&mut env2, &owner_envs_a);
    let routed_difference = output1
        .iter()
        .zip(&output2)
        .map(|(a, b)| (a[0] - b[0]).abs())
        .sum::<f32>();
    assert!(
        routed_difference > 0.1,
        "Env1 and Env2 were not distinguished"
    );

    let mut silent = Processor::new(48_000);
    let mut active = Processor::new(48_000);
    let neutral = options(EnvSource::Env1, 0.5);
    silent.reset_for_note(WaveshaperMode::Drive, 0.65, neutral, 60.0);
    active.reset_for_note(WaveshaperMode::Drive, 0.65, neutral, 60.0);
    let neutral_a = render(&mut silent, &owner_envs_a);
    let neutral_b = render(&mut active, &owner_envs_b);
    assert_eq!(neutral_a, neutral_b, "amount 0.5 must stay neutral");

    let mut realtime = Processor::new(48_000);
    let mut active = options(EnvSource::Env3, 0.9);
    active.auto_gain = true;
    realtime.reset_for_note(WaveshaperMode::Drive, 0.65, active, 60.0);
    realtime.set_envelopes(&owner_envs_a); // Warm TLS level matching before the callback check.
    let keyed_stage = Stage {
        enabled: true,
        mode: WaveshaperMode::HardClip,
        keytrack: 1.0,
        ..Stage::default()
    };
    let mut filter_stage = StageProcessor::new(48_000);
    filter_stage.configure(keyed_stage, 60.0);
    ALLOCATION_OPERATIONS.store(0, Ordering::Relaxed);
    TRACK_ALLOCATIONS.store(true, Ordering::SeqCst);
    let mut finite = true;
    let changed_controls = Options {
        input: 0.61,
        ..active
    };
    for sample in 0..512 {
        realtime.configure(WaveshaperMode::Drive, 0.65, changed_controls);
        realtime.set_envelopes(&owner_envs_a);
        filter_stage.configure(keyed_stage, if sample < 256 { 60.0 } else { 72.0 });
        let x = 0.12 * (std::f32::consts::TAU * sample as f32 / 64.0).sin();
        finite &= realtime
            .process([x, -x])
            .iter()
            .all(|value| value.is_finite());
        finite &= filter_stage
            .process([x, -x])
            .iter()
            .all(|value| value.is_finite());
    }
    TRACK_ALLOCATIONS.store(false, Ordering::SeqCst);
    let allocations = ALLOCATION_OPERATIONS.load(Ordering::Relaxed);
    assert!(
        finite,
        "external-envelope processing produced non-finite audio"
    );
    assert_eq!(
        allocations, 0,
        "configure/set_envelopes/process allocated or freed"
    );

    // Keep this in the same test as the process-global allocation tracker:
    // multiple tests in parallel would otherwise contaminate its count.
    stale_file_source_cache_is_ignored_after_switching_to_rack();
}
