use sad_pet::{
    dsp::{Engine, SampleBuffer},
    midi::Message,
    parameters,
    preset::Preset,
    resynth::{Interaction, Mode, Model, Waveform},
    timbres::Kind,
};
use std::sync::Arc;

fn engine(rate: u32, configure: impl FnOnce(&mut Preset)) -> Engine {
    let mut preset = Kind::Synth.preset().unwrap();
    configure(&mut preset);
    let mut engine = Engine::new(rate, Arc::new(SampleBuffer::demo(rate)), preset);
    engine.set_midi_gate(true);
    engine
}
fn note(engine: &mut Engine, note: u8, on: bool) {
    engine.midi(if on {
        Message::NoteOn {
            channel: 0,
            note,
            velocity: 110,
        }
    } else {
        Message::NoteOff {
            channel: 0,
            note,
            velocity: 0,
        }
    });
}
fn render(engine: &mut Engine, frames: usize) -> Vec<[f32; 2]> {
    (0..frames).map(|_| engine.process_frame()).collect()
}
fn peak(frames: &[[f32; 2]]) -> f32 {
    frames
        .iter()
        .flatten()
        .fold(0.0_f32, |peak, x| peak.max(x.abs()))
}
/// Zero crossings per second of the left channel, an estimate of the pitch.
fn frequency(frames: &[[f32; 2]], rate: u32) -> f32 {
    let crossings = frames
        .windows(2)
        .filter(|w| w[0][0] <= 0.0 && w[1][0] > 0.0)
        .count();
    crossings as f32 * rate as f32 / frames.len() as f32
}

#[test]
fn notes_play_oscillators_at_their_pitch_and_release_to_silence() {
    for rate in [44_100, 48_000, 96_000] {
        let mut engine = engine(rate, |p| {
            let o = &mut p.synth.as_mut().unwrap().oscillators[0];
            o.waveform = Waveform::Sine;
            p.params.filter = 1.0;
        });
        assert!(
            peak(&render(&mut engine, 2000)) == 0.0,
            "silent before notes"
        );
        note(&mut engine, 69, true);
        let held = render(&mut engine, rate as usize / 2);
        let sustain = &held[held.len() / 2..];
        assert!(peak(sustain) > 0.05, "{rate}: a note sounds");
        assert!(
            held.iter()
                .flatten()
                .all(|x| x.is_finite() && x.abs() < 1.0)
        );
        let hz = frequency(sustain, rate);
        assert!(
            (hz - 440.0).abs() < 8.0,
            "{rate}: A4 plays near 440 Hz, got {hz}"
        );
        note(&mut engine, 69, false);
        let released = render(&mut engine, rate as usize * 4);
        assert!(
            peak(&released[released.len() - 1000..]) < 1e-4,
            "{rate}: release ends"
        );
        note(&mut engine, 81, true);
        let octave = render(&mut engine, rate as usize / 2);
        let hz = frequency(&octave[octave.len() / 2..], rate);
        assert!(
            (hz - 880.0).abs() < 12.0,
            "{rate}: A5 plays an octave up, got {hz}"
        );
    }
}

#[test]
fn every_oscillator_type_and_link_renders_bounded_chords() {
    let waves = [
        Waveform::Saw,
        Waveform::Pulse,
        Waveform::Triangle,
        Waveform::Sine,
        Waveform::Formant,
        Waveform::Noise,
        Waveform::Pcm,
    ];
    for (case, wave) in waves.into_iter().enumerate() {
        for mode in Mode::ALL {
            let mut engine = engine(48_000, |p| {
                let synth = p.synth.as_mut().unwrap();
                synth.count = 4;
                for (i, o) in synth.oscillators.iter_mut().enumerate() {
                    o.waveform = waves[(case + i) % waves.len()];
                    o.mode = mode;
                    o.model = if i == 3 {
                        Model::Wavefolder
                    } else {
                        Model::Radias
                    };
                    o.control1 = 0.3 + i as f32 * 0.1;
                    o.control2 = 0.5;
                    o.transpose = [0.0, 7.0, -12.0, 12.0][i];
                    o.interaction = Interaction::ALL[i];
                    o.interaction_source = 2 + ((i + 1) % 4) as u8;
                    o.cross_source = ((i + 2) % 5) as u8;
                    o.level = 1.0;
                }
                synth.oscillators[0].waveform = wave;
            });
            for n in [36, 48, 55, 60, 67, 72, 79, 91, 103] {
                note(&mut engine, n, true);
            }
            let frames = render(&mut engine, 24_000);
            assert!(
                frames
                    .iter()
                    .flatten()
                    .all(|x| x.is_finite() && x.abs() <= 1.0),
                "{wave:?} {mode:?} stays bounded"
            );
            assert!(peak(&frames) > 0.01, "{wave:?} {mode:?} sounds");
        }
    }
}

#[test]
fn the_granular_stage_follows_the_vca_and_keeps_tails() {
    let mut engine = engine(48_000, |p| {
        let synth = p.synth.as_mut().unwrap();
        synth.granular = true;
        synth.buffer_seconds = 1.0;
        p.params.grains = 0.8;
        p.params.mix = 1.0;
        p.params.position = 0.0;
        p.envs[0].release = 0.0;
        p.color_chain.clear();
    });
    note(&mut engine, 60, true);
    let held = render(&mut engine, 24_000);
    assert!(
        peak(&held[12_000..]) > 0.01,
        "grains replay the played history"
    );
    assert!(engine.active_grains() > 0);
    note(&mut engine, 60, false);
    render(&mut engine, 2_400);
    // The voice has released, the grains still read the history behind it.
    let tail = render(&mut engine, 4_800);
    assert!(
        peak(&tail) > 0.005,
        "grains keep the tail after the VCA closed"
    );
    // Without grains the same release is silent.
    let mut dry = self::engine(48_000, |p| {
        p.envs[0].release = 0.0;
        p.color_chain.clear();
    });
    note(&mut dry, 60, true);
    render(&mut dry, 24_000);
    note(&mut dry, 60, false);
    render(&mut dry, 2_400);
    assert!(peak(&render(&mut dry, 4_800)) < 1e-4);
}

#[test]
fn sample_tune_stays_on_the_voices_whatever_the_grain_engine_does() {
    let play = |granular: bool, chain: f32| {
        let mut engine = engine(48_000, |p| {
            let synth = p.synth.as_mut().unwrap();
            synth.granular = granular;
            synth.oscillators[0].waveform = Waveform::Sine;
            p.params.filter = 1.0;
            p.params.mix = 1.0;
            p.params.detune = 0.5;
            p.params.cloud = 0.0;
            p.params.sample_tune_semis = 12.0;
            p.taps.grains.chain = chain;
            p.color_chain.clear();
        });
        note(&mut engine, 69, true);
        render(&mut engine, 24_000)
    };
    let direct = play(false, 1.0);
    let hz = frequency(&direct[12_000..], 48_000);
    assert!((hz - 880.0).abs() < 12.0, "A4 tuned an octave up, got {hz}");
    // Out of the chain, the grain engine leaves the tuned voices as they are.
    assert!(
        play(true, 0.0) == direct,
        "the grain engine moved the tuning"
    );
    // The grains read the tuned history without tuning it again.
    let grains = &play(true, 1.0)[12_000..];
    assert!(peak(grains) > 0.01);
    let [below, tuned, twice] = [440.0, 880.0, 1760.0].map(|hz| energy(grains, hz, 48_000));
    assert!(
        tuned > 10.0 * below.max(twice),
        "grains replay the tuned voice: {below} at 440 Hz, {tuned} at 880, {twice} at 1760"
    );
}
/// The left channel's energy at one frequency (Goertzel).
fn energy(frames: &[[f32; 2]], hz: f32, rate: u32) -> f32 {
    let k = 2.0 * (std::f32::consts::TAU * hz / rate as f32).cos();
    let (mut a, mut b) = (0.0_f32, 0.0_f32);
    for frame in frames {
        (a, b) = (frame[0] + k * a - b, a);
    }
    a * a + b * b - k * a * b
}

#[test]
fn parameters_edit_the_synth_and_presets_round_trip() {
    let mut engine = engine(48_000, |_| {});
    let changes = parameters::parse(&serde_json::json!({
        "synth_count": 3,
        "synth_granular": 1,
        "synth_buffer_seconds": 2.5,
        "synth_osc2_waveform": 7,
        "synth_osc3_mode": 2,
        "synth_osc3_transpose": -12,
        "synth_osc3_interaction_source": 3
    }))
    .unwrap();
    engine.apply_parameters(&changes);
    let synth = *engine.preset.synth.as_deref().unwrap();
    assert_eq!(synth.count, 3);
    assert!(synth.granular);
    assert_eq!(synth.buffer_seconds, 2.5);
    assert_eq!(synth.oscillators[1].waveform, Waveform::Pcm);
    assert_eq!(synth.oscillators[2].mode, Mode::Unison);
    assert_eq!(synth.oscillators[2].transpose, -12.0);
    note(&mut engine, 64, true);
    assert!(peak(&render(&mut engine, 48_000)) > 0.01);
    let json = serde_json::to_value(&engine.preset).unwrap();
    let mut restored: Preset = serde_json::from_value(json).unwrap();
    restored.sanitize();
    assert_eq!(restored.synth.as_deref(), Some(&synth));
    assert_eq!(Kind::of(&restored), Kind::Synth);
    // Granular and drum presets keep their exact serialized form.
    for preset in [Preset::default(), Kind::Drums.preset().unwrap()] {
        let value = serde_json::to_value(&preset).unwrap();
        assert!(value.get("synth").is_none());
        assert_ne!(Kind::of(&preset), Kind::Synth);
    }
}

#[test]
fn synth_controls_append_after_every_other_host_parameter() {
    let specs = parameters::specs();
    let start = specs.iter().position(|s| s.id == "synth_count").unwrap();
    assert_eq!(specs[start - 1].id, "env8_curve");
    assert_eq!(specs[start + 1].id, "synth_granular");
    assert_eq!(specs[start + 2].id, "synth_buffer_seconds");
    assert_eq!(specs[start + 3].id, "synth_osc1_enabled");
    let end = start + 3 + 4 * 14;
    assert_eq!(specs[end - 1].id, "synth_osc4_interaction_source");
    // Then the mooz controls of every FX/Pre/Source slot, then each drum
    // track's pattern note shift.
    let mooz = end + (2 * 12 + 1) * 9;
    assert_eq!(specs[end].id, "fx1_mooz_p1");
    assert_eq!(specs[mooz - 1].id, "source_fx1_mooz_p9");
    let later: Vec<_> = specs[mooz..].iter().map(|s| s.id.as_str()).collect();
    let shifts: Vec<_> = (1..=7).map(|t| format!("drum{t}_note_shift")).collect();
    assert_eq!(later[..7], shifts);
    // Then each section's tap: Resynth's send and bus, the six Chain
    // shares, then the send and bus of the other five sections.
    assert_eq!(later[7..9], ["resynth_send", "resynth_bus"]);
    assert_eq!(later[9], "tap_source_chain");
    // Then routes 2 and 3 of each LFO with route 3 of its ENV, then the macros.
    let taps = 9 + 6 + 5 * 2;
    assert_eq!(
        later[taps..taps + 3],
        ["lfo1_depth2", "lfo1_depth3", "env1_depth3"]
    );
    assert_eq!(later[taps + 8 * 3], "macro1");
    // Then advanced grain mode and its four groups' controls.
    let groups = taps + 8 * 3 + 8;
    assert_eq!(later[groups], "grain_advanced");
    // Then each group's output: chain, send and bus.
    assert_eq!(later[groups + 1 + 4 * 16], "grain_group1_chain");
    assert_eq!(later.len(), groups + 1 + 4 * 16 + 4 * 3);
}

#[test]
fn switching_engines_keeps_the_input_each_one_replaced() {
    let mut engine = Engine::new(
        48_000,
        Arc::new(SampleBuffer::demo(48_000)),
        Preset::default(),
    );
    assert!(engine.live_input.is_none());
    for kind in [Kind::Synth, Kind::Drums, Kind::Synth] {
        engine.set_preset(kind.preset().unwrap());
        assert!(engine.live_input.is_some(), "{kind:?} owns a history");
        render(&mut engine, 64);
    }
    engine.set_preset(Preset::default());
    assert!(engine.live_input.is_none(), "the file source returns");
}
