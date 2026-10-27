//! A Rack's Sampler and Vocal modules: a kit of pads and a sung voice as
//! sources of their own chains, beside the Synth and Drums in its Mixer.
use sad_pet::{
    Stereo,
    dsp::{Engine, SampleBuffer},
    midi::Message,
    preset::Preset,
    rack::{Module, STRIP_SOURCES, Slot},
    sampler::Zone,
    sequence::{Target, Track},
    timbres::Kind,
};
use std::sync::Arc;

const RATE: u32 = 48_000;
const SAMPLER: usize = 3;
const VOCAL: usize = 4;

fn tone(hz: f32, seconds: f32) -> Arc<SampleBuffer> {
    let frames = (0..(RATE as f32 * seconds) as usize)
        .map(|i| [(i as f32 * hz * std::f32::consts::TAU / RATE as f32).sin() * 0.5; 2])
        .collect();
    Arc::new(SampleBuffer::new("tone".into(), RATE, frames).unwrap())
}
fn slots(modules: &[Module]) -> Vec<Slot> {
    modules
        .iter()
        .map(|&module| Slot {
            module,
            enabled: true,
        })
        .collect()
}
/// Pads 1 and 2 hold a tone, as in the Sampler timbre's tests.
fn with_kit(preset: &mut Preset) {
    if let Some(kit) = preset.sampler.as_deref_mut() {
        kit.pads[0].zones.push(Zone::new("a"));
        kit.pads[1].zones.push(Zone::new("b"));
    }
}
fn engine(mut preset: Preset) -> Engine {
    with_kit(&mut preset);
    let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset);
    if let Some(pool) = engine.sampler_pool_mut() {
        pool.insert("a".into(), tone(440.0, 0.5));
        pool.insert("b".into(), tone(220.0, 0.5));
    }
    engine.refresh_sampler();
    engine.set_midi_gate(true);
    engine.set_playing(true);
    engine
}
fn rack_preset(modules: &[Module], edit: impl FnOnce(&mut Preset)) -> Preset {
    let mut preset = Kind::Rack.preset().unwrap();
    preset.rack.as_mut().unwrap().modules = slots(modules);
    preset.color_chain.clear();
    preset.params.master_volume = 1.0;
    preset.params.out_gain = 0.0;
    preset.sanitize();
    edit(&mut preset);
    preset.sanitize();
    preset
}
fn rack(modules: &[Module], edit: impl FnOnce(&mut Preset)) -> Engine {
    engine(rack_preset(modules, edit))
}
fn note(engine: &mut Engine, note: u8, on: bool) {
    engine.midi(if on {
        Message::NoteOn {
            channel: 0,
            note,
            velocity: 127,
        }
    } else {
        Message::NoteOff {
            channel: 0,
            note,
            velocity: 0,
        }
    });
}
fn render_with(engine: &mut Engine, frames: usize, input: impl Fn(usize) -> Stereo) -> Vec<Stereo> {
    (0..frames)
        .map(|i| engine.process_input_frame(input(i)))
        .collect()
}
fn render(engine: &mut Engine, frames: usize) -> Vec<Stereo> {
    render_with(engine, frames, |_| [0.0; 2])
}
fn peak(frames: &[Stereo]) -> f32 {
    frames
        .iter()
        .flatten()
        .fold(0.0_f32, |peak, x| peak.max(x.abs()))
}
fn bits(frames: &[Stereo]) -> Vec<[u32; 2]> {
    frames.iter().map(|f| f.map(f32::to_bits)).collect()
}
/// What a Vocal timbre's host hands its engine: the voice, the same on both sides.
fn voice(i: usize) -> Stereo {
    [(i as f32 * 0.031).sin() * 0.4 * (i as f32 * 0.0007).cos(); 2]
}
/// The same sound up to the output level: a Rack's settles 1.5e-5 short of
/// unity, where its one-pole step falls under an ulp; a bus starts there.
fn close(a: &[Stereo], b: &[Stereo]) {
    assert_eq!(a.len(), b.len());
    for (i, (a, b)) in a.iter().zip(b).enumerate() {
        for c in 0..2 {
            assert!(
                (a[c] - b[c]).abs() <= 2e-5 * b[c].abs() + 1e-7,
                "frame {i}: {a:?} {b:?}"
            );
        }
    }
}

#[test]
fn the_strips_and_modules_of_kits_and_voices_follow_the_drums() {
    assert_eq!(STRIP_SOURCES[SAMPLER], Module::Sampler);
    assert_eq!(STRIP_SOURCES[VOCAL], Module::Vocal);
    assert!(Module::Sampler.source() && Module::Vocal.source());
    let preset = rack_preset(&[Module::Sampler, Module::Vocal, Module::Mixer], |_| {});
    let steps: Vec<_> = preset.rack.as_ref().unwrap().plan().iter().collect();
    // Their pads and phrases have envelopes of their own: one stream each.
    assert!(steps[0].mixed && !steps[0].per_note);
    assert!(steps[1].mixed && !steps[1].per_note);
}

#[test]
fn a_sampler_module_plays_its_kit_as_a_sampler_timbre_does() {
    let play = |engine: &mut Engine| {
        // The rack's output level settles first; a Sampler's starts there.
        render(engine, 24_000);
        note(engine, 36, true);
        let mut out = render(engine, 6_000);
        note(engine, 36, false);
        note(engine, 37, true);
        out.extend(render(engine, 6_000));
        out
    };
    let expected = play(&mut engine(Kind::Sampler.preset().unwrap()));
    let rendered = play(&mut rack(&[Module::Sampler], |_| {}));
    assert!(peak(&expected) > 0.1);
    close(&rendered, &expected);
}

#[test]
fn a_vocal_module_sings_the_voice_its_host_hands_in() {
    let play = |engine: &mut Engine| {
        render(engine, 24_000);
        render_with(engine, 8_000, voice)
    };
    let mut vocal = Kind::Vocal.preset().unwrap();
    vocal.sanitize();
    let expected = play(&mut engine(vocal));
    let rendered = play(&mut rack(&[Module::Vocal], |_| {}));
    assert!(peak(&expected) > 0.1);
    close(&rendered, &expected);
    // Without a Vocal module a Rack takes no input at all.
    let drums = |input: fn(usize) -> Stereo| {
        let mut engine = rack(&[Module::Drums, Module::Synth, Module::Mixer], |p| {
            let drums = p.drums.as_mut().unwrap();
            drums.tracks[0].steps[0].velocity = 1.0;
        });
        note(&mut engine, 60, true);
        render_with(&mut engine, 12_000, input)
    };
    let silent = drums(|_| [0.0; 2]);
    assert!(peak(&silent) > 0.01);
    assert_eq!(bits(&drums(voice)), bits(&silent));
}

#[test]
fn notes_play_the_kit_and_the_synth_into_the_mixer() {
    let mut engine = rack(&[Module::Sampler, Module::Synth, Module::Mixer], |_| {});
    assert_eq!(peak(&render(&mut engine, 4_000)), 0.0);
    note(&mut engine, 36, true);
    assert!(peak(&render(&mut engine, 6_000)) > 0.05);
    let peaks = engine.rack_peaks().unwrap();
    engine.clear_rack_peaks();
    assert!(peaks[SAMPLER] > 0.05 && peaks[1] > 0.01, "{peaks:?}");
    note(&mut engine, 36, false);
    // The one-shot pad rings past the note; the synth's voice releases.
    render(&mut engine, 24_000);
    engine.clear_rack_peaks();
    note(&mut engine, 37, true);
    render(&mut engine, 2_000);
    let peaks = engine.rack_peaks().unwrap();
    assert!(peaks[SAMPLER] > 0.05 && peaks[1] > 0.01, "{peaks:?}");

    // Muted, the Sampler strip keeps the kit out; the synth plays on.
    let mut muted = rack(&[Module::Sampler, Module::Synth, Module::Mixer], |p| {
        p.rack.as_mut().unwrap().strips.sampler.muted = true;
    });
    note(&mut muted, 36, true);
    render(&mut muted, 6_000);
    let peaks = muted.rack_peaks().unwrap();
    assert!(peaks[SAMPLER] == 0.0 && peaks[1] > 0.01, "{peaks:?}");
}

#[test]
fn the_pedal_holds_the_synth_and_the_kit_alike() {
    let mut engine = rack(&[Module::Sampler, Module::Synth, Module::Mixer], |p| {
        let kit = p.sampler.as_deref_mut().unwrap();
        kit.pads[0].trigger = sad_pet::sampler::Trigger::NoteOn;
        kit.pads[0].envelope.release = 5.0;
    });
    engine.midi(Message::Control {
        channel: 0,
        controller: 64,
        value: 127,
    });
    note(&mut engine, 36, true);
    render(&mut engine, 4_000);
    note(&mut engine, 36, false);
    render(&mut engine, 12_000);
    engine.clear_rack_peaks();
    render(&mut engine, 2_000);
    let held = engine.rack_peaks().unwrap();
    assert!(held[SAMPLER] > 0.05 && held[1] > 0.01, "{held:?}");
    engine.midi(Message::Control {
        channel: 0,
        controller: 64,
        value: 0,
    });
    render(&mut engine, RATE as usize);
    engine.clear_rack_peaks();
    render(&mut engine, 2_000);
    let released = engine.rack_peaks().unwrap();
    assert!(
        released[SAMPLER] == 0.0 && released[1] < 1e-4,
        "{released:?}"
    );
}

#[test]
fn a_pad_track_plays_only_its_pad() {
    let mut engine = rack(&[Module::Sampler, Module::Synth, Module::Mixer], |_| {});
    let mut track = Track {
        id: 7,
        target: Target::Pad { pad: 2 },
        ..Track::default()
    };
    // Every step at C5: the Synth would play it, and no pad has that key.
    for step in &mut track.steps {
        step.pitch = 72;
    }
    let mut preset = engine.preset.clone();
    preset.sequencer.enabled = true;
    preset.sequencer.tracks = vec![track];
    engine.set_preset(preset);
    engine.set_playing(true);
    render(&mut engine, RATE as usize);
    let hits = engine.sampler_view().hits;
    assert!(hits.iter().any(|h| h.0 == 2 && h.1 >= 2), "{hits:?}");
    assert_eq!(hits[0].1, 0);
    let peaks = engine.rack_peaks().unwrap();
    assert!(peaks[SAMPLER] > 0.05 && peaks[1] == 0.0, "{peaks:?}");
}

#[test]
fn pad_chains_return_into_the_sampler_chain() {
    let mut engine = rack(&[Module::Sampler, Module::Mixer], |_| {});
    render(&mut engine, 24_000);
    engine.set_sampler_routes(&[1]);
    engine.trigger_pad(1, None, 127);
    let mut sent = 0.0_f32;
    let mut heard = 0.0_f32;
    for _ in 0..1_000 {
        heard = heard.max(peak(&render(&mut engine, 1)));
        sent = sent.max(engine.sampler_send(1)[0].abs());
    }
    // The routed pad leaves the mix for its Pad FX timbre...
    assert!(sent > 0.1 && heard == 0.0, "{sent} {heard}");
    // ...whose return joins the kit's chain in the next frame only.
    engine.return_pads([0.25; 2]);
    let returned = render(&mut engine, 2);
    assert!(
        returned[0][0] > 0.2 && returned[1][0] == 0.0,
        "{returned:?}"
    );
    engine.apply_parameters(
        &sad_pet::parameters::parse(&serde_json::json!({"rack_sampler_mute": 1})).unwrap(),
    );
    render(&mut engine, RATE as usize / 4);
    engine.return_pads([0.25; 2]);
    assert_eq!(peak(&render(&mut engine, 1)), 0.0);
}

#[test]
fn kits_drums_synth_and_voice_meet_on_their_strips() {
    let mut engine = rack(
        &[
            Module::Sampler,
            Module::Drums,
            Module::Synth,
            Module::Vocal,
            Module::Mixer,
        ],
        |p| {
            let rack = p.rack.as_mut().unwrap();
            rack.strips.drums.muted = true;
            rack.strips.synth.muted = true;
            rack.strips.vocal.pan = 0.0;
        },
    );
    note(&mut engine, 36, true);
    let frames = render_with(&mut engine, 6_000, voice);
    let peaks = engine.rack_peaks().unwrap();
    assert!(peaks[SAMPLER] > 0.05 && peaks[VOCAL] > 0.05, "{peaks:?}");
    assert!(peaks[1] == 0.0 && peaks[2] == 0.0, "{peaks:?}");
    assert!(peak(&frames) > 0.1);
    // Solo the voice, panned hard left: the right side is silent.
    engine.apply_parameters(
        &sad_pet::parameters::parse(&serde_json::json!({"rack_vocal_solo": 1})).unwrap(),
    );
    assert!(engine.preset.rack.as_ref().unwrap().strips.vocal.solo);
    render_with(&mut engine, RATE as usize / 2, voice);
    let soloed = render_with(&mut engine, 4_000, voice);
    assert!(soloed.iter().all(|f| f[1] == 0.0) && peak(&soloed) > 0.05);
    // The strip levels are host parameters too.
    engine.apply_parameters(
        &sad_pet::parameters::parse(&serde_json::json!({"rack_sampler_level": 0.5})).unwrap(),
    );
    assert_eq!(
        engine.preset.rack.as_ref().unwrap().strips.sampler.level,
        0.5
    );
}

#[test]
fn a_rack_keeps_the_kit_and_score_of_its_modules_only() {
    let preset = rack_preset(&[Module::Sampler, Module::Vocal, Module::Mixer], |p| {
        p.sampler.as_deref_mut().unwrap().polyphony = 9;
        p.vocal.as_deref_mut().unwrap().score.transpose = 5;
    });
    assert_eq!(Kind::of(&preset), Kind::Rack);
    assert!(preset.bus.is_none(), "a Rack's chains are its modules");
    let json = serde_json::to_string(&preset).unwrap();
    let back: Preset = serde_json::from_str(&json).unwrap();
    assert_eq!(Kind::of(&back), Kind::Rack);
    assert_eq!(back.rack, preset.rack);
    assert_eq!(back.sampler.as_ref().unwrap().polyphony, 9);
    assert_eq!(back.vocal.as_ref().unwrap().score.transpose, 5);
    // A new module starts from the settings a new timbre of its kind has.
    let fresh = rack_preset(&[Module::Sampler, Module::Vocal], |_| {});
    assert_eq!(
        fresh.sampler.as_ref().unwrap().pads.len(),
        Kind::Sampler.preset().unwrap().sampler.unwrap().pads.len()
    );
    assert!(!fresh.vocal.as_ref().unwrap().score.voice.is_empty());
    // A removed module leaves nothing behind.
    let mut removed = back;
    removed.rack.as_mut().unwrap().modules = slots(&[Module::Mixer]);
    removed.sanitize();
    assert!(removed.sampler.is_none() && removed.vocal.is_none());
    assert_eq!(Kind::of(&removed), Kind::Rack);
}

#[test]
fn a_rack_edit_keeps_the_kit_pool_and_dropping_the_module_frees_it() {
    let mut engine = rack(&[Module::Sampler, Module::Mixer], |_| {});
    let mut edited = engine.preset.clone();
    edited.rack.as_mut().unwrap().modules =
        slots(&[Module::Sampler, Module::Filters, Module::Mixer]);
    engine.set_preset(edited);
    assert_eq!(engine.sampler_pool().unwrap().len(), 2);
    note(&mut engine, 36, true);
    assert!(peak(&render(&mut engine, 4_000)) > 0.05);
    let mut dropped = engine.preset.clone();
    dropped.rack.as_mut().unwrap().modules = slots(&[Module::Synth, Module::Mixer]);
    engine.set_preset(dropped);
    assert!(engine.sampler_pool().is_none() && engine.preset.sampler.is_none());
    render(&mut engine, 1_000);
}
