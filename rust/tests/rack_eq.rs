//! A Rack's EQ modules: as many as eight, anywhere in the rack, each with
//! bands that are host parameters, so automation and modulation move them.
use sad_pet::{
    dsp::{Engine, SampleBuffer},
    eq::{Band, Equalizer, Kind as Shape, MAX_EQS},
    preset::{Destination, Preset},
    rack::{Module, Slot},
    timbres::Kind,
};
use std::sync::Arc;

const RATE: u32 = 48_000;
type Stereo = [f32; 2];

fn rack_preset(modules: &[Module], edit: impl FnOnce(&mut Preset)) -> Preset {
    let mut preset = Kind::Rack.preset().unwrap();
    preset.rack.as_mut().unwrap().modules = modules
        .iter()
        .map(|&module| Slot {
            module,
            enabled: true,
            id: 0,
        })
        .collect();
    preset.color_chain.clear();
    preset.params.master_volume = 1.0;
    preset.sanitize();
    edit(&mut preset);
    preset.sanitize();
    preset
}
fn engine(preset: Preset) -> Engine {
    let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset);
    engine.set_playing(true);
    engine
}
/// A Vocal module hands the rack whatever its host plays: here a sine that
/// runs on from one render to the next.
fn sine(frequency: f32) -> impl FnMut() -> Stereo {
    let mut frame = 0;
    move || {
        frame += 1;
        [(std::f32::consts::TAU * frequency * frame as f32 / RATE as f32).sin() * 0.1; 2]
    }
}
fn render(engine: &mut Engine, frames: usize, input: &mut impl FnMut() -> Stereo) -> Vec<Stereo> {
    (0..frames)
        .map(|_| engine.process_input_frame(input()))
        .collect()
}
fn peak(frames: &[Stereo]) -> f32 {
    frames
        .iter()
        .flatten()
        .fold(0.0_f32, |peak, x| peak.max(x.abs()))
}
/// The level in dB of a settled sine through the rack.
fn level(preset: &Preset, frequency: f32) -> f32 {
    let mut engine = engine(preset.clone());
    let mut input = sine(frequency);
    render(&mut engine, RATE as usize / 2, &mut input);
    20.0 * peak(&render(&mut engine, RATE as usize / 4, &mut input)).log10()
}
/// An EQ with one bell and every other band off.
fn bell(id: u8, frequency: f32, gain: f32) -> Equalizer {
    let mut eq = Equalizer::numbered(id);
    for band in &mut eq.bands {
        band.enabled = false;
    }
    eq.bands[2] = Band {
        kind: Shape::Bell,
        frequency,
        gain,
        q: 2.0,
        ..Band::default()
    };
    eq
}
fn index(id: &str) -> usize {
    sad_pet::parameters::specs()
        .iter()
        .position(|s| s.id == id)
        .unwrap()
}

#[test]
fn an_eq_shapes_its_chain_and_two_in_a_row_add_up() {
    let dry = rack_preset(&[Module::Vocal], |_| {});
    let boosted = rack_preset(&[Module::Vocal, Module::Eq], |p| {
        p.rack.as_mut().unwrap().equalizers = vec![bell(1, 1000.0, 12.0)];
    });
    let lift = level(&boosted, 1000.0) - level(&dry, 1000.0);
    assert!((lift - 12.0).abs() < 0.1, "{lift}");
    let below = level(&boosted, 100.0) - level(&dry, 100.0);
    assert!(below.abs() < 0.2, "{below}");
    // A second EQ cutting what the first boosts brings the sine back.
    let both = rack_preset(&[Module::Vocal, Module::Eq, Module::Eq], |p| {
        p.rack.as_mut().unwrap().equalizers = vec![bell(1, 1000.0, 12.0), bell(2, 1000.0, -12.0)];
    });
    let net = level(&both, 1000.0) - level(&dry, 1000.0);
    assert!(net.abs() < 0.1, "{net}");
    // Off, an EQ passes the chain untouched.
    let mut off = both.clone();
    for slot in &mut off.rack.as_mut().unwrap().modules {
        slot.enabled = slot.module != Module::Eq;
    }
    let a = render(&mut engine(off), 4000, &mut sine(1000.0));
    let b = render(&mut engine(dry), 4000, &mut sine(1000.0));
    assert_eq!(a, b);
}

#[test]
fn eq_bands_are_host_parameters_that_automation_and_lfos_move() {
    let preset = rack_preset(&[Module::Vocal, Module::Eq], |p| {
        p.rack.as_mut().unwrap().equalizers = vec![bell(1, 1000.0, 0.0)];
    });
    let mut input = sine(1000.0);
    let mut engine = engine(preset.clone());
    render(&mut engine, RATE as usize / 4, &mut input);
    let flat = peak(&render(&mut engine, RATE as usize / 4, &mut input));
    engine.apply_parameters(
        &sad_pet::parameters::parse(&serde_json::json!({"rack_eq1_band3_gain": -12.0})).unwrap(),
    );
    let eq = engine.preset.rack.as_ref().unwrap().equalizers[0];
    assert_eq!(eq.bands[2].gain, -12.0);
    // The gain glides down: no frame jumps.
    let glide = render(&mut engine, 2000, &mut input);
    for pair in glide.windows(2) {
        assert!((pair[1][0] - pair[0][0]).abs() < 0.02);
    }
    let cut = peak(&render(&mut engine, RATE as usize / 4, &mut input));
    let change = 20.0 * (cut / flat).log10();
    assert!((change + 12.0).abs() < 0.2, "{change}");
    // Frequency is a host parameter along its log scale: 0.5 is 632 Hz.
    let spec = &sad_pet::parameters::specs()[index("rack_eq1_band3_frequency")];
    assert_eq!((spec.min, spec.max), (0.0, 1.0));
    engine.apply_parameters(
        &sad_pet::parameters::parse(&serde_json::json!({"rack_eq1_band3_frequency": 0.5})).unwrap(),
    );
    let moved = engine.preset.rack.as_ref().unwrap().equalizers[0].bands[2].frequency;
    assert!((moved - 632.5).abs() < 1.0, "{moved}");
    // An LFO on the bell's frequency sweeps it past the sine and back.
    let mut swept = rack_preset(&[Module::Vocal, Module::Eq], |p| {
        p.rack.as_mut().unwrap().equalizers = vec![bell(1, 1000.0, 12.0)];
        p.lfos[0].rate_hz = 1.0;
        p.lfos[0].destination = Destination::Control {
            index: index("rack_eq1_band3_frequency") as u16,
        };
        p.lfos[0].depth = 0.8;
    });
    swept.sanitize();
    let mut engine = self::engine(swept);
    let levels: Vec<f32> = (0..8)
        .map(|_| peak(&render(&mut engine, RATE as usize / 8, &mut input)))
        .collect();
    let (low, high) = levels
        .iter()
        .fold((f32::MAX, 0f32), |(l, h), &x| (l.min(x), h.max(x)));
    assert!(high > low * 2.0, "{levels:?}");
    assert!(
        levels.iter().all(|x| x.is_finite() && *x < 1.0),
        "{levels:?}"
    );
}

#[test]
fn eqs_save_with_their_numbers_and_settings() {
    let preset = rack_preset(
        &[Module::Synth, Module::Eq, Module::Mixer, Module::Eq],
        |p| {
            let rack = p.rack.as_mut().unwrap();
            rack.equalizers[1].bands[4].gain = 4.5;
            rack.equalizers[1].output = -3.0;
        },
    );
    let rack = preset.rack.as_ref().unwrap();
    let ids: Vec<_> = rack.modules.iter().map(|s| s.id).collect();
    assert_eq!(ids, [0, 1, 0, 2]);
    let json = serde_json::to_value(&preset).unwrap();
    // Other modules save as they always have.
    assert_eq!(
        json["rack"]["modules"][0],
        serde_json::json!({"module": "synth", "enabled": true})
    );
    assert_eq!(json["rack"]["modules"][3]["id"], 2);
    assert_eq!(json["rack"]["equalizers"][1]["bands"][4]["gain"], 4.5);
    let back: Preset = serde_json::from_value(json).unwrap();
    assert_eq!(back.rack, preset.rack);
    // A rack without EQs saves no equalizers.
    let plain = rack_preset(&[Module::Synth], |_| {});
    assert!(
        !serde_json::to_string(&plain)
            .unwrap()
            .contains("equalizers")
    );
    // Every EQ a rack can hold plays.
    let modules: Vec<_> = std::iter::once(Module::Vocal)
        .chain([Module::Eq; MAX_EQS])
        .collect();
    let full = rack_preset(&modules, |_| {});
    assert_eq!(full.rack.as_ref().unwrap().equalizers.len(), MAX_EQS);
    let out = render(&mut engine(full), 4000, &mut sine(440.0));
    assert!(peak(&out) > 0.05 && out.iter().flatten().all(|x| x.is_finite()));
}
