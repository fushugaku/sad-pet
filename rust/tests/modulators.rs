use sad_pet::{
    dsp::{Engine, SampleBuffer, modulation::Oscillator, telemetry::RealtimeView},
    matrix::{Route, Source},
    parameters,
    preset::{Destination, Env, Lfo, Preset, Waveform},
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::Arc,
};
struct Allocator;
thread_local! {static WATCH:Cell<bool>=const{Cell::new(false)};static EVENTS:Cell<usize>=const{Cell::new(0)};}
fn event() {
    let _ = WATCH.try_with(|w| {
        if w.get() {
            let _ = EVENTS.try_with(|n| n.set(n.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        event();
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        event();
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        event();
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
fn target(id: &str) -> Destination {
    Destination::Control {
        index: parameters::specs().iter().position(|s| s.id == id).unwrap() as u16,
    }
}
fn render(e: &mut Engine, n: usize) {
    for _ in 0..n {
        assert!(e.process_frame().into_iter().all(f32::is_finite));
    }
}

#[test]
fn eight_generators_modulate_oscillators_and_numeric_controls_without_audio_allocations() {
    let mut p = Preset {
        lfo_count: 8,
        env_count: 8,
        ..Preset::default()
    };
    p.params.grains = 0.0;
    p.color_chain.clear();
    p.lfos[7] = Lfo {
        waveform: Waveform::Square,
        depth: 1.0,
        destination: target("resynth_osc4_control1"),
        ..Lfo::default()
    };
    p.lfos[6].waveform = Waveform::Square;
    p.envs[7] = Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        depth: 0.1,
        destination: target("resynth_osc3_fine"),
        ..Env::default()
    };
    p.matrix.push(Route {
        source: Source::Lfo7,
        destination: target("resynth_osc2_transpose"),
        depth: 0.125,
        enabled: true,
    });
    let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), p);
    e.set_playing(true);
    EVENTS.with(|n| n.set(0));
    WATCH.with(|w| w.set(true));
    render(&mut e, 2000);
    WATCH.with(|w| w.set(false));
    assert_eq!(EVENTS.with(Cell::get), 0);
    assert_eq!(e.env_values[7], 1.0);
    let knobs = e.knob_states();
    assert_eq!(knobs["resynth_osc4_control1"].current, 0.5);
    assert!((knobs["resynth_osc3_fine"].current - 20.0).abs() < 1e-5);
    assert_eq!(knobs["resynth_osc2_transpose"].current, 12.0);
    assert_eq!(e.preset.resynth.oscillators[3].control1, 0.0);
    let mut v = RealtimeView::default();
    e.realtime_view(&mut v);
    let host = e.knob_states_from_view(&v);
    assert_eq!(
        host["resynth_osc4_control1"].current,
        knobs["resynth_osc4_control1"].current
    );
    assert_eq!(
        host["resynth_osc3_fine"].current,
        knobs["resynth_osc3_fine"].current
    );
    assert!(knobs["resynth_osc4_control1"].modulated);
}

#[test]
fn old_three_generator_presets_pad_without_changing_their_routes_and_new_banks_roundtrip() {
    let mut value = serde_json::to_value(Preset::default()).unwrap();
    value.as_object_mut().unwrap().remove("lfo_count");
    value.as_object_mut().unwrap().remove("env_count");
    value["lfos"].as_array_mut().unwrap().truncate(3);
    value["envs"].as_array_mut().unwrap().truncate(3);
    value["lfos"][2]["destination"] = serde_json::json!("pitch");
    let mut p: Preset = serde_json::from_value(value).unwrap();
    p.sanitize();
    assert_eq!((p.lfo_count, p.env_count), (3, 3));
    assert_eq!(p.lfos.len(), 8);
    assert_eq!(p.envs.len(), 8);
    assert_eq!(p.lfos[2].destination, Destination::Pitch);
    p.lfo_count = 8;
    p.env_count = 8;
    p.lfos[7].destination = target("resynth_osc4_waveform");
    p.envs[7].destination = target("resynth_osc4_control2");
    let mut copy: Preset = serde_json::from_slice(&serde_json::to_vec(&p).unwrap()).unwrap();
    copy.sanitize();
    assert_eq!((copy.lfo_count, copy.env_count), (8, 8));
    assert_eq!(copy.lfos[7].destination, p.lfos[7].destination);
    assert_eq!(copy.envs[7].destination, p.envs[7].destination);
    assert_eq!(parameters::specs()[1611].id, "vcf2_type");
    assert!(
        parameters::specs()
            .iter()
            .any(|s| s.id == "env8_extra_depth")
    );
    assert!(parameters::specs().iter().any(|s| s.id == "lfo8_rate"));
}

#[test]
fn synced_lfos_stretch_to_thirty_two_bars() {
    let mut p = Preset::default();
    p.params.grains = 0.0;
    p.lfos[0].waveform = Waveform::Saw;
    p.lfos[0].sync = true;
    let spec = parameters::specs()
        .iter()
        .find(|s| s.id == "lfo1_division")
        .unwrap();
    spec.set(&mut p, 128.0);
    let mut copy: Preset = serde_json::from_slice(&serde_json::to_vec(&p).unwrap()).unwrap();
    copy.sanitize();
    assert_eq!(copy.lfos[0].division, 128.0);
    // 32 bars of 4/4 at 120 BPM take 64 seconds: a quarter of the cycle is 8 bars.
    let mut lfo = Oscillator::new(1);
    for _ in 0..16_000 {
        lfo.next_sample(&copy.lfos[0], 1000.0, 120.0, 0.0, None);
    }
    assert!((lfo.phase - 0.25).abs() < 1e-9, "{}", lfo.phase);
    // The transport places the cycle by the beat: beat 96 is three quarters in.
    let mut e = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), copy);
    e.set_playing(true);
    e.host_transport(120.0, Some(96.0), true, false);
    e.process_frame();
    assert!((e.lfo_values[0] - 0.5).abs() < 1e-3, "{}", e.lfo_values[0]);
}
