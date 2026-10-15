use sad_pet::{
    dsp::{Engine, SampleBuffer, modulation::Envelope},
    midi::Message,
    performance::Mode,
    preset::{Destination, Env, Preset},
};
use std::sync::Arc;

fn sample() -> Arc<SampleBuffer> {
    Arc::new(
        SampleBuffer::new(
            "tone".into(),
            8000,
            (0..8000)
                .map(|i| {
                    let value = (i as f32 / 8000.0 * 220.0 * std::f32::consts::TAU).sin() * 0.3;
                    [value, value]
                })
                .collect(),
        )
        .unwrap(),
    )
}
fn on(channel: u8, note: u8) -> Message {
    Message::NoteOn {
        channel,
        note,
        velocity: 127,
    }
}
fn off(channel: u8, note: u8) -> Message {
    Message::NoteOff {
        channel,
        note,
        velocity: 0,
    }
}
fn run(engine: &mut Engine, frames: usize) {
    for _ in 0..frames {
        engine.process_frame();
    }
}
fn fast_env() -> Env {
    Env {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        release: 0.0,
        ..Env::default()
    }
}
fn preset() -> Preset {
    let mut p = Preset::default();
    p.params.reverb = 0.0;
    p.color_chain[1].mix = 0.0;
    p.params.grains = 0.75;
    p.envs = [fast_env(); 3];
    p
}

#[test]
fn adsr_reaches_sustain_releases_and_retriggers_from_zero() {
    let config = Env {
        attack: 0.1,
        decay: 0.1,
        sustain: 0.4,
        release: 0.1,
        ..Env::default()
    };
    let mut env = Envelope::default();
    env.configure(1000.0, &config);
    assert!(!env.active());
    assert_eq!(env.next_sample(), 0.0);
    env.on();
    let attack: Vec<_> = (0..10).map(|_| env.next_sample()).collect();
    assert!(attack.windows(2).all(|w| w[0] <= w[1]));
    assert!((attack[9] - 1.0).abs() < 0.00001);
    for _ in 0..50 {
        env.next_sample();
    }
    assert!((env.next_sample() - 0.4).abs() < 0.00001);
    env.on();
    assert!((env.next_sample() - 0.1).abs() < 0.00001);
    env.off();
    for _ in 0..30 {
        env.next_sample();
    }
    assert!(!env.active());
    env.on();
    env.next_sample();
    env.kill();
    assert_eq!(env.next_sample(), 0.0);
}

#[test]
fn three_envelopes_have_independent_times_and_share_note_gates() {
    let mut p = preset();
    p.envs[1].attack = 0.5;
    p.envs[2].attack = 1.0;
    let mut engine = Engine::new(8000, sample(), p);
    run(&mut engine, 32);
    assert_eq!(engine.env_values, [0.0; 3]);
    engine.midi(on(0, 60));
    run(&mut engine, 800);
    assert!((engine.env_values[0] - 1.0).abs() < 0.00001);
    assert!(engine.env_values[1] > 0.09 && engine.env_values[1] < 0.11);
    assert!(engine.env_values[2] > 0.012 && engine.env_values[2] < 0.013);
    engine.midi(off(0, 60));
    run(&mut engine, 100);
    assert_eq!(engine.env_values, [0.0; 3]);
}

#[test]
fn amp_envelope_stays_polyphonic_and_sustain_defers_all_three_releases() {
    let mut engine = Engine::new(8000, sample(), preset());
    engine.midi(on(0, 60));
    engine.midi(on(1, 64));
    run(&mut engine, 300);
    engine.midi(off(0, 60));
    run(&mut engine, 100);
    assert_eq!(engine.env_values, [1.0; 3]);
    engine.midi(Message::Control {
        channel: 1,
        controller: 64,
        value: 127,
    });
    engine.midi(off(1, 64));
    run(&mut engine, 100);
    assert_eq!(engine.env_values, [1.0; 3]);
    engine.midi(Message::Control {
        channel: 1,
        controller: 64,
        value: 0,
    });
    run(&mut engine, 100);
    assert_eq!(engine.env_values, [0.0; 3]);
    engine.midi(on(0, 62));
    run(&mut engine, 100);
    engine.midi(Message::Control {
        channel: 0,
        controller: 120,
        value: 0,
    });
    run(&mut engine, 1);
    assert_eq!(engine.env_values, [0.0; 3]);
}

#[test]
fn each_env_can_drive_two_destinations_without_removing_amp_control() {
    for source in 0..3 {
        let mut p = preset();
        p.params.pitch = 0.3;
        p.params.cloud = 0.2;
        p.envs[source].destination = Destination::Pitch;
        p.envs[source].depth = 0.15;
        p.envs[source].extra_destination = Destination::Cloud;
        p.envs[source].extra_depth = -0.1;
        let mut engine = Engine::new(8000, sample(), p);
        engine.midi(on(0, 60));
        run(&mut engine, 320);
        let routed = engine.modulated_params();
        assert!((routed.pitch - 0.45).abs() < 0.00001);
        assert!((routed.cloud - 0.1).abs() < 0.00001);
        assert_eq!(engine.env_values[0], 1.0);
        engine.midi(off(0, 60));
        run(&mut engine, 100);
        assert_eq!(engine.env_values[0], 0.0);
        run(&mut engine, 32);
        assert_eq!(engine.modulated_params().pitch, 0.3);
    }
}

#[test]
fn default_second_env_drives_filter_and_new_routes_reach_effect_controls() {
    let mut p = preset();
    p.envs[1] = Preset::default().envs[1];
    p.params.filter = 0.3;
    p.envs[2].destination = Destination::Saturation;
    p.envs[2].depth = 0.4;
    p.envs[2].extra_destination = Destination::FilterResonance;
    p.envs[2].extra_depth = 0.2;
    let mut engine = Engine::new(8000, sample(), p);
    engine.midi(on(0, 60));
    run(&mut engine, 800);
    let routed = engine.modulated_params();
    let cutoff = engine.voice_filter_cutoffs(60).unwrap()[0];
    assert!(cutoff > 0.4 && cutoff < 0.65);
    assert!((routed.saturation - 0.4).abs() < 0.00001);
    assert!((routed.filter_resonance - 0.32).abs() < 0.00001);
    let mut p = engine.preset.clone();
    p.envs[2].extra_destination = Destination::Volume;
    p.envs[2].extra_depth = -1.0;
    engine.set_preset(p);
    run(&mut engine, 32);
    assert_eq!(engine.modulated_params().master_volume, 0.0);
}

#[test]
fn amp_controls_dry_audio_and_long_release_keeps_grains_alive() {
    let mut p = preset();
    p.params.mix = 0.0;
    p.envs[0].sustain = 0.0;
    let mut engine = Engine::new(8000, sample(), p);
    engine.set_playing(true);
    run(&mut engine, 8000);
    assert!((0..1024).all(|_| engine.process_frame().iter().all(|x| x.abs() < 0.00001)));
    let mut p = preset();
    p.envs[0].release = 0.6;
    p.params.grain_size = 0.5;
    let mut engine = Engine::new(8000, sample(), p);
    engine.midi(on(0, 60));
    run(&mut engine, 320);
    engine.midi(off(0, 60));
    run(&mut engine, 1600);
    assert!(engine.env_values[0] > 0.4);
    let mut generated = false;
    let energy: f32 = (0..800)
        .map(|_| {
            let frame = engine.process_frame();
            generated |= engine.active_grains() > 0;
            frame[0].abs()
        })
        .sum();
    assert!(generated);
    assert!(energy > 0.1);
    run(&mut engine, 24000);
    assert_eq!(engine.env_values[0], 0.0);
}

#[test]
fn envelope_routes_are_saved_legacy_adsr_migrates_and_bad_values_are_sanitized() {
    let mut p = preset();
    p.envs[0].attack = 0.7;
    p.envs[0].extra_destination = Destination::GrainShape;
    p.envs[0].extra_depth = -0.4;
    p.envs[2].destination = Destination::ReverbTone;
    let data = serde_json::to_string(&p).unwrap();
    let restored: Preset = serde_json::from_str(&data).unwrap();
    assert_eq!(p.envs, restored.envs);
    let mut legacy = serde_json::to_value(p).unwrap();
    legacy.as_object_mut().unwrap().remove("envs");
    legacy["params"]
        .as_object_mut()
        .unwrap()
        .remove("filter_routing");
    legacy["params"]
        .as_object_mut()
        .unwrap()
        .remove("filter_env_depth");
    legacy["params"]["adsr_attack"] = 0.81.into();
    legacy["params"]["adsr_sustain"] = 0.42.into();
    let restored: Preset = serde_json::from_value(legacy).unwrap();
    assert_eq!(restored.envs[0].attack, 0.81);
    assert_eq!(restored.envs[0].sustain, 0.42);
    assert_eq!(restored.envs[1].destination, Destination::None);
    assert_eq!(restored.envs[1].depth, 0.0);
    assert_eq!(restored.params.filter_env_depth, 0.0);
    let mut p = restored;
    p.envs[0].attack = f32::NAN;
    p.envs[1].depth = f32::INFINITY;
    p.envs[2].sustain = 2.0;
    p.envs[2].extra_depth = -5.0;
    p.sanitize();
    assert_eq!(p.envs[0].attack, Env::default().attack);
    assert_eq!(p.envs[1].depth, 0.0);
    assert_eq!(p.envs[2].sustain, 1.0);
    assert_eq!(p.envs[2].extra_depth, -1.0);
    assert_eq!(p.params.adsr_attack, p.envs[0].attack);
}

#[test]
fn envelopes_retrigger_in_hold_arp_and_sequence_and_render_is_block_independent() {
    for mode in [Mode::Hold, Mode::Arp, Mode::Sequencer, Mode::SequencerArp] {
        let mut p = preset();
        p.performance.mode = mode;
        p.performance.sequence_division = 0.25;
        p.envs[2].destination = Destination::Pitch;
        p.envs[2].depth = 0.1;
        p.envs[2].extra_destination = Destination::ReverbTone;
        p.envs[2].extra_depth = -0.2;
        let mut a = Engine::new(8000, sample(), p.clone());
        let mut b = Engine::new(8000, sample(), p);
        a.set_playing(true);
        b.set_playing(true);
        let mut expected = vec![[0.0; 2]; 8000];
        let mut actual = expected.clone();
        a.process(&mut expected);
        for block in actual.chunks_mut(71) {
            b.process(block);
        }
        assert_eq!(actual, expected, "{mode:?}");
        assert!(actual.iter().flatten().any(|x| x.abs() > 0.01));
        a.set_playing(false);
        run(&mut a, 100);
        assert_eq!(a.env_values, [0.0; 3]);
    }
}
