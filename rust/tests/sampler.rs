use sad_pet::{
    Stereo,
    dsp::{Engine, SampleBuffer},
    midi::Message,
    sampler::{Trigger, Zone},
    sequence::{Target, Track},
    timbres::Kind,
};
use std::sync::Arc;

fn tone(hz: f32, seconds: f32) -> Arc<SampleBuffer> {
    let frames = (0..(48_000.0 * seconds) as usize)
        .map(|i| [(i as f32 * hz * std::f32::consts::TAU / 48_000.0).sin() * 0.5; 2])
        .collect();
    Arc::new(SampleBuffer::new("tone".into(), 48_000, frames).unwrap())
}
/// A kit with a tone on pads 1 and 2, as a Sampler timbre starts.
fn kit(edit: impl FnOnce(&mut sad_pet::sampler::Settings)) -> Engine {
    let mut preset = Kind::Sampler.preset().unwrap();
    let settings = preset.sampler.as_deref_mut().unwrap();
    settings.pads[0].zones.push(Zone::new("a"));
    settings.pads[1].zones.push(Zone::new("b"));
    edit(settings);
    let mut engine = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), preset);
    let pool = engine.sampler_pool_mut().unwrap();
    pool.insert("a".into(), tone(440.0, 0.5));
    pool.insert("b".into(), tone(220.0, 0.5));
    engine.refresh_sampler();
    engine
}
fn energy(engine: &mut Engine, frames: usize) -> f32 {
    (0..frames)
        .map(|_| engine.process_input_frame([0.0; 2]))
        .map(|f: Stereo| f[0] * f[0] + f[1] * f[1])
        .sum()
}

#[test]
fn notes_play_the_pads_on_their_keys_through_a_passing_chain() {
    let mut engine = kit(|_| {});
    assert_eq!(energy(&mut engine, 2_000), 0.0);
    engine.midi(Message::NoteOn {
        channel: 0,
        note: 36,
        velocity: 127,
    });
    let first: Vec<Stereo> = (0..100)
        .map(|_| engine.process_input_frame([0.0; 2]))
        .collect();
    // The chain passes the kit unchanged: pad 1's first frames are the tone's.
    let sample = tone(440.0, 0.5);
    for (out, frame) in first.iter().zip(sample.frames()) {
        assert!((out[0] - frame[0]).abs() < 1e-5, "{out:?} {frame:?}");
    }
    // One-shot pads ring past the note-off.
    engine.midi(Message::NoteOff {
        channel: 0,
        note: 36,
        velocity: 0,
    });
    assert!(energy(&mut engine, 4_000) > 1.0);
    // Note 40 has an empty pad.
    let mut engine = kit(|_| {});
    engine.midi(Message::NoteOn {
        channel: 0,
        note: 40,
        velocity: 127,
    });
    assert_eq!(energy(&mut engine, 2_000), 0.0);
}

#[test]
fn note_on_pads_release_with_the_note_and_the_pedal() {
    let mut engine = kit(|s| {
        s.pads[0].trigger = Trigger::NoteOn;
        s.pads[0].envelope.release = 5.0;
    });
    engine.midi(Message::Control {
        channel: 0,
        controller: 64,
        value: 127,
    });
    engine.midi(Message::NoteOn {
        channel: 0,
        note: 36,
        velocity: 100,
    });
    energy(&mut engine, 500);
    engine.midi(Message::NoteOff {
        channel: 0,
        note: 36,
        velocity: 0,
    });
    assert!(energy(&mut engine, 1_000) > 1.0, "the pedal holds it");
    engine.midi(Message::Control {
        channel: 0,
        controller: 64,
        value: 0,
    });
    energy(&mut engine, 1_000);
    assert!(engine.sampler_idle());
}

#[test]
fn a_pad_track_plays_its_pad_whatever_the_step_pitch() {
    let mut engine = kit(|_| {});
    let mut track = Track {
        id: 7,
        target: Target::Pad { pad: 2 },
        ..Track::default()
    };
    // Every step at C5: no pad has that key.
    for step in &mut track.steps {
        step.pitch = 72;
    }
    let mut preset = engine.preset.clone();
    preset.sequencer.enabled = true;
    preset.sequencer.tracks = vec![track];
    engine.set_preset(preset);
    engine.set_playing(true);
    let hits = |e: &Engine| {
        e.sampler_view()
            .hits
            .iter()
            .find(|h| h.0 == 2)
            .map_or(0, |h| h.1)
    };
    energy(&mut engine, 48_000);
    assert!(hits(&engine) >= 2, "{:?}", engine.sampler_view().hits);
    assert_eq!(engine.sampler_view().hits[0].1, 0);
}

#[test]
fn routed_pads_leave_the_mix_for_their_send() {
    let mut engine = kit(|_| {});
    engine.set_sampler_routes(&[1]);
    engine.trigger_pad(1, None, 127);
    let mut sent = 0.0;
    for _ in 0..1_000 {
        let out = engine.process_input_frame([0.0; 2]);
        assert_eq!(out, [0.0; 2]);
        sent += engine.sampler_send(1)[0].abs();
    }
    assert!(sent > 1.0);
    // The input still passes: a Pad FX timbre's return.
    assert_eq!(engine.process_input_frame([0.25, -0.5]), [0.25, -0.5]);
}

#[test]
fn kits_survive_a_save_and_load() {
    let engine = kit(|s| s.pads[3].name = "Snare".into());
    let json = serde_json::to_value(&engine.preset).unwrap();
    let preset: sad_pet::preset::Preset = serde_json::from_value(json).unwrap();
    assert_eq!(Kind::of(&preset), Kind::Sampler);
    assert_eq!(preset.sampler.unwrap().pads[3].name, "Snare");
    // Presets of other timbres serialize without a kit.
    let plain = serde_json::to_value(sad_pet::preset::Preset::default()).unwrap();
    assert!(plain.get("sampler").is_none());
}
