//! New instruments are Racks: each kind's Rack plays as a timbre of that
//! kind did, with the modules its engine runs.
use sad_pet::{
    dsp::{Engine, SampleBuffer},
    midi::Message,
    preset::Preset,
    rack::Module,
    timbres::Kind,
};
use std::sync::Arc;

const RATE: u32 = 48_000;
type Stereo = [f32; 2];

fn engine(preset: Preset, kind: Kind) -> Engine {
    let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset);
    engine.set_midi_gate(kind.waits_for_notes());
    engine.set_playing(true);
    engine
}
fn note(engine: &mut Engine, note: u8, on: bool) {
    engine.midi(if on {
        Message::NoteOn {
            channel: 0,
            note,
            velocity: 100,
        }
    } else {
        Message::NoteOff {
            channel: 0,
            note,
            velocity: 0,
        }
    });
}
/// A chord, a release and a new note; drums play their pattern meanwhile.
fn play(engine: &mut Engine) -> Vec<Stereo> {
    let mut out: Vec<Stereo> = (0..24_000).map(|_| engine.process_frame()).collect();
    note(engine, 60, true);
    note(engine, 64, true);
    out.extend((0..9_000).map(|_| engine.process_frame()));
    note(engine, 60, false);
    out.extend((0..4_000).map(|_| engine.process_frame()));
    note(engine, 67, true);
    out.extend((0..6_000).map(|_| engine.process_frame()));
    note(engine, 64, false);
    note(engine, 67, false);
    out.extend((0..8_000).map(|_| engine.process_frame()));
    out
}
fn peak(frames: &[Stereo]) -> f32 {
    frames
        .iter()
        .flatten()
        .fold(0.0_f32, |peak, x| peak.max(x.abs()))
}
/// The largest difference between `a` and `b` shifted by `lag` frames,
/// relative to `b`'s peak.
fn difference(a: &[Stereo], b: &[Stereo], lag: usize) -> f32 {
    let worst = a
        .iter()
        .zip(&b[lag..])
        .flat_map(|(a, b)| [(a[0] - b[0]).abs(), (a[1] - b[1]).abs()])
        .fold(0.0_f32, f32::max);
    worst / peak(b)
}

#[test]
fn each_instrument_kind_becomes_a_rack_of_its_engines_modules() {
    use Module::*;
    for (kind, modules) in [
        (Kind::Granular, vec![Sample, Grains, Filters, PostFx]),
        (Kind::Drums, vec![Drums, Filters, PostFx]),
        (Kind::Synth, vec![Synth, Filters, PostFx]),
        (Kind::Sampler, vec![Sampler]),
        (Kind::Vocal, vec![Vocal]),
        (Kind::Tb303, vec![Tb303]),
    ] {
        let preset = kind.rack_instrument().unwrap();
        assert_eq!(Kind::of(&preset), Kind::Rack, "{kind:?}");
        let rack = preset.rack.as_ref().unwrap();
        let listed: Vec<_> = rack.modules.iter().map(|s| s.module).collect();
        assert_eq!(listed, modules, "{kind:?}");
        // The source's own settings come with it.
        assert_eq!(preset.drums.is_some(), kind == Kind::Drums);
        assert_eq!(preset.synth.is_some(), kind == Kind::Synth);
        assert_eq!(preset.sampler.is_some(), kind == Kind::Sampler);
        assert_eq!(preset.vocal.is_some(), kind == Kind::Vocal);
        assert_eq!(preset.tb303.is_some(), kind == Kind::Tb303);
    }
    for kind in [
        Kind::Rack,
        Kind::Bus,
        Kind::Master,
        Kind::PadFx,
        Kind::Empty,
    ] {
        assert!(kind.rack_instrument().is_none(), "{kind:?}");
    }
}

#[test]
fn granular_synth_and_drum_racks_sound_as_their_timbres() {
    // A Granular timbre starts from the host's preset, which is the default.
    let granular = (Preset::default(), Kind::Granular.rack_instrument().unwrap());
    let synth = (
        Kind::Synth.preset().unwrap(),
        Kind::Synth.rack_instrument().unwrap(),
    );
    let mut drums = (
        Kind::Drums.preset().unwrap(),
        Kind::Drums.rack_instrument().unwrap(),
    );
    for preset in [&mut drums.0, &mut drums.1] {
        let kit = preset.drums.as_mut().unwrap();
        for step in [0, 4, 8, 12] {
            kit.tracks[0].steps[step].velocity = 1.0;
            kit.tracks[2].steps[step + 2].velocity = 0.8;
        }
    }
    for (kind, (timbre, rack), lag) in [
        (Kind::Granular, granular, 0),
        // A Synth timbre hears its voices through its history, a frame later
        // than the Rack does.
        (Kind::Synth, synth, 1),
    ] {
        let expected = play(&mut engine(timbre, kind));
        let rendered = play(&mut engine(rack, kind));
        assert!(peak(&expected) > 0.01, "{kind:?} plays");
        let off = difference(&rendered[..rendered.len() - lag], &expected, lag);
        assert!(off < 1e-4, "{kind:?}: {off}");
    }
    // A Drum timbre plays its pattern through voice 0's envelope, held at
    // sustain, which its Rack's level stands in for (notes leave the Rack's
    // drums alone).
    let pattern = |preset| {
        let mut engine = engine(preset, Kind::Drums);
        (0..RATE as usize * 2)
            .map(|_| engine.process_frame())
            .collect::<Vec<_>>()
    };
    let expected = pattern(drums.0);
    let rendered = pattern(drums.1);
    assert!(peak(&expected) > 0.01, "the drums play");
    // The envelope's attack and decay shape only the first beat there.
    let settled = RATE as usize / 2;
    let off = difference(&rendered[settled..], &expected[settled..], 0);
    assert!(off < 0.015, "Drums: {off}");
}

#[test]
fn factory_presets_are_racks_that_play_as_their_granular_engines() {
    use Module::*;
    for index in 0..=sad_pet::startup::INDEX {
        let classic = Preset::builtin(index);
        let rack = Preset::factory(index);
        assert_eq!(Kind::of(&rack), Kind::Rack, "{}", classic.name);
        assert_eq!(rack.name, classic.name);
        let modules: Vec<_> = rack
            .rack
            .as_ref()
            .unwrap()
            .modules
            .iter()
            .map(|s| s.module)
            .collect();
        assert_eq!(modules[..2], [Sample, Grains], "{}", classic.name);
        assert_eq!(
            modules[modules.len() - 2..],
            [Filters, PostFx],
            "{}",
            classic.name
        );
        assert_eq!(modules.contains(&Resynth), classic.resynth.enabled);
        assert_eq!(
            modules.contains(&PreFx),
            !classic.pre_color_chain.is_empty()
        );
        // With notes, and with voice 0 playing the file without them.
        for gate in [true, false] {
            let run = |preset: Preset| {
                let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset);
                engine.set_midi_gate(gate);
                engine.set_playing(true);
                play(&mut engine)
            };
            let expected = run(classic.clone());
            let rendered = run(rack.clone());
            assert!(peak(&expected) > 0.01, "{} plays", classic.name);
            let off = difference(&rendered, &expected, 0);
            assert!(off < 1e-5, "{} (gate {gate}): {off}", classic.name);
        }
    }
    // Startup's own preset converts too, and Source FX keep a preset classic.
    assert_eq!(
        Kind::of(&sad_pet::startup::preset().as_rack().unwrap()),
        Kind::Rack
    );
    let mut fx = Preset::builtin(1);
    fx.source_color_chain = vec![sad_pet::color::ColorSlot::default()];
    assert!(fx.as_rack().is_none());
    assert!(Kind::Synth.preset().unwrap().as_rack().is_none());
}

#[test]
fn a_live_input_plays_in_a_racks_sample_as_in_a_granular_timbre() {
    use Module::*;
    // What a microphone hands the engine: a chirp that runs on across renders.
    let input = |i: usize| {
        let t = i as f32 / RATE as f32;
        [(std::f32::consts::TAU * (220.0 + 180.0 * t) * t).sin() * 0.3; 2]
    };
    for (index, effects_only) in [(1, false), (sad_pet::startup::INDEX, false), (3, true)] {
        let run = |preset: Preset| {
            let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset);
            engine.set_midi_gate(true);
            engine.set_playing(true);
            engine.set_live_input(true, effects_only);
            assert!(engine.live_input.is_some());
            let mut frame = 0;
            let mut render = |engine: &mut Engine, n: usize| {
                (0..n)
                    .map(|_| {
                        frame += 1;
                        engine.process_input_frame(input(frame))
                    })
                    .collect::<Vec<_>>()
            };
            // A second of input fills the history before a note plays it.
            let mut out = render(&mut engine, RATE as usize);
            note(&mut engine, 57, true);
            out.extend(render(&mut engine, RATE as usize * 3 / 2));
            note(&mut engine, 57, false);
            out.extend(render(&mut engine, RATE as usize / 2));
            out
        };
        let classic = Preset::builtin(index);
        let expected = run(classic.clone());
        let rendered = run(Preset::factory(index));
        assert!(peak(&expected) > 0.01, "{} plays the input", classic.name);
        let off = difference(&rendered, &expected, 0);
        assert!(
            off < 1e-5,
            "{} (effects only {effects_only}): {off}",
            classic.name
        );
    }
    // The input stays through edits of a Rack that can play it, and from a
    // Granular timbre into one and back.
    let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), Preset::builtin(1));
    engine.set_live_input(true, false);
    let mut rack = Preset::factory(1);
    engine.set_preset(rack.clone());
    assert!(
        engine.live_input.is_some(),
        "a Granular timbre's input plays in its Rack"
    );
    rack.rack.as_mut().unwrap().modules[3].enabled = false;
    engine.set_preset(rack.clone());
    assert!(engine.live_input.is_some(), "a Rack's edit keeps the input");
    engine.set_preset(Preset::builtin(2));
    assert!(
        engine.live_input.is_some(),
        "and so does a Granular timbre after it"
    );
    engine.set_preset(rack.clone());
    // A Grain engine reading its chain's history takes the input's place.
    let modules = &mut rack.rack.as_mut().unwrap().modules;
    modules.swap(1, 2);
    assert_eq!(modules[2].module, Grains);
    engine.set_preset(rack.clone());
    assert!(!engine.takes_input());
    engine.set_live_input(true, false);
    assert_eq!(engine.live_input.as_ref().unwrap().sample.name, "Rack");
    // A Synth Rack has no Sample for it.
    engine.set_preset(Kind::Synth.rack_instrument().unwrap());
    assert!(!engine.takes_input());
}
