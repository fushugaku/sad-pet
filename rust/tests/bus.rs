use sad_pet::{
    bus,
    color::ColorSlot,
    dsp::{Engine, SampleBuffer},
    preset::{FilterType, Preset, SaturatorType},
    timbres::Kind,
};
use std::sync::Arc;

fn engine(kind: Kind, edit: impl FnOnce(&mut Preset)) -> Engine {
    let mut preset = kind.preset().unwrap();
    edit(&mut preset);
    Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), preset)
}
fn input(n: usize) -> [f32; 2] {
    let t = n as f32 / 48_000.0;
    [
        (t * 440.0 * std::f32::consts::TAU).sin() * 1.3,
        (t * 6_100.0 * std::f32::consts::TAU).sin() * 0.4,
    ]
}
fn settings(p: &mut Preset) -> &mut bus::Settings {
    p.bus.as_deref_mut().unwrap()
}

#[test]
fn default_master_and_bus_pass_their_input_bit_for_bit() {
    for kind in [Kind::Master, Kind::Bus] {
        let mut e = engine(kind, |_| {});
        for playing in [false, true] {
            e.set_playing(playing);
            for n in 0..4_000 {
                let x = input(n);
                // Loud input too: the passthrough neither clips nor fades in.
                assert_eq!(
                    e.process_input_frame(x).map(f32::to_bits),
                    x.map(f32::to_bits)
                );
            }
        }
    }
}

#[test]
fn switched_on_processing_passes_at_unity_without_notes_or_transport() {
    // An empty-but-present FX slot at zero mix keeps the engine path active.
    let mut e = engine(Kind::Bus, |p| {
        p.color_chain = vec![ColorSlot {
            id: 1,
            kind: SaturatorType::Compressor,
            mix: 0.0,
            ..Default::default()
        }];
    });
    e.set_playing(false);
    let mut error = 0.0_f32;
    for n in 0..12_000 {
        let x = input(n).map(|v| v * 0.5);
        let y = e.process_input_frame(x);
        if n > 6_000 {
            error = error.max((y[0] - x[0]).abs()).max((y[1] - x[1]).abs());
        }
    }
    assert!(error < 1e-4, "unity path changed the input by {error}");
}

#[test]
fn filters_switch_on_and_shape_the_bus() {
    let energy = |filters: bool| {
        let mut e = engine(Kind::Bus, |p| {
            settings(p).filters = filters;
            p.params.vcf_type = FilterType::LowPass;
            p.params.filter = 0.15;
        });
        (0..24_000)
            .map(|n| {
                let y = e.process_input_frame([0.0, input(n)[1]]);
                y[1] * y[1]
            })
            .skip(4_000)
            .sum::<f32>()
    };
    let (open, filtered) = (energy(false), energy(true));
    assert!(
        open > 100.0,
        "bypassed filters must pass the 6.1 kHz tone ({open})"
    );
    assert!(
        filtered < open * 0.05,
        "the low-pass did not apply: {filtered} vs {open}"
    );
}

#[test]
fn granular_grows_a_history_and_turning_it_off_retires_it_off_thread() {
    let mut e = engine(Kind::Bus, |p| {
        settings(p).granular = true;
        p.params.mix = 1.0;
        p.params.grains = 0.8;
    });
    assert!(
        e.live_input.is_some(),
        "Granular prepares the input history"
    );
    e.set_playing(true);
    let mut energy = 0.0;
    for n in 0..48_000 {
        let y = e.process_input_frame(input(n).map(|v| v * 0.5));
        energy += y[0] * y[0];
    }
    assert!(energy > 1.0, "grains read the bus input ({energy})");
    assert!(e.active_grains() > 0);
    // A prepared update without Granular hands the history back to the caller.
    let mut off = e.preset.clone();
    settings(&mut off).granular = false;
    off.sanitize();
    let mut prepared = off.prepared_clone_for(48_000);
    assert!(prepared.bus_runtime.0.is_none());
    e.exchange_preset(&mut prepared);
    assert!(e.live_input.is_none());
    assert!(
        prepared.bus_runtime.0.is_some(),
        "the history retires with the old preset"
    );
    // Back on with a prepared buffer: the engine takes it.
    let mut on = e.preset.clone();
    settings(&mut on).granular = true;
    let mut prepared = on.prepared_clone_for(48_000);
    e.exchange_preset(&mut prepared);
    assert!(e.live_input.is_some());
    assert!(prepared.bus_runtime.0.is_none());
}

#[test]
fn bus_presets_keep_their_kind_and_round_trip() {
    for kind in [Kind::Bus, Kind::Master] {
        let preset = kind.preset().unwrap();
        let text = serde_json::to_string(&preset).unwrap();
        let back: Preset = serde_json::from_str(&text).unwrap();
        assert_eq!(Kind::of(&back), kind);
        assert_eq!(back.bus, preset.bus);
    }
    // Other presets serialize without a bus field at all.
    let granular = serde_json::to_value(Preset::default()).unwrap();
    assert!(granular.get("bus").is_none());
}
