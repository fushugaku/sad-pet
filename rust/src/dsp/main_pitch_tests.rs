use super::*;
use crate::{
    sequence::{Edit, Stage, Target, Track},
    timbres::Kind,
};
use metropolix_core::sequencer::field;

const RATE: u32 = 8_000;
const SYNTH_A: u32 = 1 << 8;
const SYNTH_B: u32 = 1 << 9;
const WAVETABLE_A: u32 = 1 << 12;
const WAVETABLE_B: u32 = 1 << 13;
const RESYNTH_A: u32 = 1 << 16;
const TB303_VCO: u32 = 1 << 24;

fn track(id: u32, pitch: u8, oscillator_mask: u32, granular: bool) -> Track {
    let mut t = Track {
        id,
        target: Target::MainPitch,
        oscillator_mask,
        granular,
        env_gate: true,
        ..Track::default()
    };
    t.steps.fill(Stage {
        pitch,
        ..Stage::default()
    });
    t.config[field::SCALE] = metropolix_core::scale::FACTORY_SCALES
        .iter()
        .position(|s| s.name == "Chromatic")
        .unwrap() as i32;
    t.config[field::GATE] = 100;
    t
}

fn sample() -> Arc<SampleBuffer> {
    Arc::new(
        SampleBuffer::new(
            "Main pitch test".into(),
            RATE,
            (0..RATE as usize * 2)
                .map(|n| {
                    [((n as f32) * std::f32::consts::TAU * 220.0 / RATE as f32).sin() * 0.3; 2]
                })
                .collect(),
        )
        .unwrap(),
    )
}

fn engine(mut preset: Preset, tracks: Vec<Track>) -> Engine {
    preset.sequencer.enabled = true;
    preset.sequencer.tracks = tracks;
    preset.color_chain.clear();
    preset.pre_color_chain.clear();
    preset.params.sample_root_note = 60.0;
    preset.params.master_volume = 0.5;
    preset.envs[0].attack = 0.0;
    preset.envs[0].decay = 0.0;
    preset.envs[0].sustain = 1.0;
    preset.envs[0].release = 0.8;
    let mut engine = Engine::new(RATE, sample(), preset);
    engine.set_midi_gate(true);
    engine.set_playing(true);
    engine
}

fn advance(engine: &mut Engine, frames: usize) {
    for _ in 0..frames {
        assert!(engine.process_frame().into_iter().all(f32::is_finite));
    }
}

fn held(engine: &Engine, owner: u32) -> Option<usize> {
    engine
        .voices
        .iter()
        .position(|v| v.main_pitch && v.owner == owner && v.held)
}

fn capture(mut engine: Engine, frames: usize) -> Vec<f32> {
    (0..frames)
        .map(|_| {
            let frame = engine.process_frame();
            assert!(frame.into_iter().all(f32::is_finite));
            frame[0]
        })
        .collect()
}

fn energy(frames: &[f32]) -> f32 {
    frames.iter().map(|v| v.abs()).sum()
}

fn difference(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum()
}

#[test]
fn separate_parties_keep_fractional_pitch_and_independent_glide() {
    let mut sliding = track(1, 60, SYNTH_A, false);
    sliding.config[field::LENGTH] = 2;
    sliding.config[field::SLIDE_AMOUNT] = 100;
    sliding.config[field::SLIDE_TYPE] = 1;
    sliding.steps[0].gate = 3;
    sliding.steps[1] = Stage {
        pitch: 72,
        gate: 3,
        slide: true,
        ..Stage::default()
    };
    let mut fixed = track(2, 73, SYNTH_B, false);
    fixed.amount = 0.5;
    let mut e = engine(Kind::Synth.preset().unwrap(), vec![sliding, fixed]);
    advance(&mut e, 32);
    let (a, b) = (held(&e, 1).unwrap(), held(&e, 2).unwrap());
    assert_ne!(a, b);
    assert!((e.voices[a].pitch - 60.0).abs() < 0.01);
    assert!((e.voices[b].pitch - 66.5).abs() < 0.01);
    let mut fractional_slide = false;
    for _ in 0..2_600 {
        e.process_frame();
        if let Some(index) = held(&e, 1) {
            let pitch = e.voices[index].pitch;
            fractional_slide |=
                pitch > 60.01 && pitch < 71.99 && (pitch - pitch.round()).abs() > 0.01;
        }
        if let Some(index) = held(&e, 2) {
            assert!((e.voices[index].pitch - 66.5).abs() < 0.01);
        }
    }
    assert!(
        fractional_slide,
        "the first party never produced an independent fractional slide"
    );
}

#[test]
fn released_note_keeps_its_pitch_and_routing_when_the_next_step_changes() {
    let mut t = track(7, 60, SYNTH_A, false);
    t.config[field::LENGTH] = 2;
    t.config[field::GATE] = 50;
    t.steps[1].pitch = 72;
    let mut e = engine(Kind::Synth.preset().unwrap(), vec![t]);
    advance(&mut e, 100);
    let first = held(&e, 7).unwrap();
    let (pitch, mask) = (e.voices[first].pitch, e.voices[first].oscillator_mask);
    let mut next = None;
    for _ in 0..2_000 {
        e.process_frame();
        if held(&e, 7).is_some_and(|i| i != first) {
            next = held(&e, 7);
            break;
        }
    }
    let next = next.expect("the next note never started");
    assert!(
        e.voices[first].active(),
        "release was cut off by the next note"
    );
    assert!(!e.voices[first].held);
    assert!((e.voices[first].pitch - pitch).abs() < 0.001);
    assert_eq!(e.voices[first].oscillator_mask, mask);
    assert!((e.voices[next].pitch - 72.0).abs() < 0.01);
    let mut preset = e.preset.clone();
    preset.sequencer.tracks[0].oscillator_mask = SYNTH_B;
    e.set_preset(preset);
    advance(&mut e, 64);
    assert_eq!(e.voices[first].oscillator_mask, SYNTH_A);
    assert!((e.voices[first].pitch - pitch).abs() < 0.001);
    assert_eq!(e.voices[next].oscillator_mask, SYNTH_B);
}

#[test]
fn duplicate_notes_mute_remove_and_stop_without_releasing_other_parties() {
    let mut e = engine(
        Kind::Synth.preset().unwrap(),
        vec![
            track(1, 60, SYNTH_A, false),
            track(2, 60, SYNTH_A, false),
            track(3, 60, SYNTH_A, false),
        ],
    );
    advance(&mut e, 100);
    let (a, b, c) = (
        held(&e, 1).unwrap(),
        held(&e, 2).unwrap(),
        held(&e, 3).unwrap(),
    );
    assert!(a != b && b != c && a != c);
    e.apply_sequence_edits(&[Edit {
        track: 1,
        stage: None,
        field: field::MUTE as u8,
        value: 1.0,
    }]);
    assert!(!e.voices[a].held);
    assert!(e.voices[b].held && e.voices[c].held);
    let mut preset = e.preset.clone();
    preset.sequencer.tracks.retain(|t| t.id != 2);
    e.set_preset(preset);
    assert!(!e.voices[b].held);
    assert!(e.voices[c].held);
    e.set_sequence_transport(false);
    assert!(!e.voices[c].held);
}

fn instrument(kind: Kind) -> Preset {
    let mut preset = kind.preset().unwrap();
    match kind {
        Kind::Synth => {
            let synth = preset.synth.as_mut().unwrap();
            synth.count = 2;
            synth.oscillators[1].transpose = 12.0;
        }
        Kind::Wavetable => {
            let table = preset.wavetable.as_mut().unwrap();
            table.count = 2;
            table.oscillators[1].controls.enabled = true;
            table.oscillators[1].controls.transpose = 12.0;
        }
        Kind::Oscillators => {
            let osc = preset.oscillators.as_mut().unwrap();
            osc.add(crate::oscillators::Kind::Radias).unwrap();
            osc.slots[1].controls.transpose = 12.0;
        }
        Kind::Tb303 => {
            preset.tb303.as_mut().unwrap().pattern = false;
        }
        _ => unreachable!(),
    }
    preset
}

#[test]
fn oscillator_masks_reach_synth_wavetable_and_mixed_audio() {
    for (kind, a, b) in [
        (Kind::Synth, SYNTH_A, SYNTH_B),
        (Kind::Wavetable, WAVETABLE_A, WAVETABLE_B),
        (Kind::Oscillators, 1, 2),
    ] {
        let preset = instrument(kind);
        let first = capture(engine(preset.clone(), vec![track(1, 60, a, false)]), 450);
        let second = capture(engine(preset.clone(), vec![track(1, 60, b, false)]), 450);
        let none = capture(engine(preset, vec![track(1, 60, 0, false)]), 450);
        assert!(
            energy(&first) > 0.01,
            "{kind:?}: first oscillator is silent"
        );
        assert!(
            energy(&second) > 0.01,
            "{kind:?}: second oscillator is silent"
        );
        assert!(
            difference(&first, &second) > 0.01,
            "{kind:?}: oscillator selection did not change audio"
        );
        assert!(
            energy(&none) < 1e-5,
            "{kind:?}: deselected oscillators still sound"
        );
    }
}

#[test]
fn granular_participation_can_be_switched_independently() {
    let mut granular = Preset::default();
    granular.params.mix = 1.0;
    granular.params.grains = 0.8;
    granular.params.cloud = 0.0;
    granular.params.detune = 0.5;
    let on = capture(engine(granular.clone(), vec![track(1, 60, 0, true)]), 450);
    let off = capture(engine(granular, vec![track(1, 60, 0, false)]), 450);
    assert!(energy(&on) > 0.01, "selected grains did not sound");
    assert!(energy(&off) < 1e-5, "deselected grains still sound");
    let osc = capture(
        engine(instrument(Kind::Synth), vec![track(1, 60, SYNTH_A, false)]),
        450,
    );
    assert!(energy(&osc) > 0.01, "oscillator-only party did not sound");
}

#[test]
fn separate_303_and_resynth_parties_change_the_actual_output() {
    let bass = instrument(Kind::Tb303);
    let first = track(1, 48, TB303_VCO, false);
    let second = track(2, 60, TB303_VCO, false);
    let one = capture(engine(bass.clone(), vec![first.clone()]), 400);
    let other = capture(engine(bass.clone(), vec![second.clone()]), 400);
    let two = capture(engine(bass, vec![first, second]), 400);
    assert!(energy(&one) > 0.001, "selected 303 VCO did not sound");
    assert!(
        difference(&one, &two) > 0.001,
        "second 303 party did not affect output"
    );
    assert!(
        difference(&other, &two) > 0.001,
        "second 303 party replaced the first"
    );

    let mut resynth = Preset::default();
    resynth.resynth.enabled = true;
    resynth.resynth.mix = 1.0;
    resynth.resynth.attack = 0.0;
    resynth.resynth.threshold = 0.0;
    let first = track(1, 60, RESYNTH_A, false);
    let second = track(2, 72, RESYNTH_A, false);
    let one = capture(engine(resynth.clone(), vec![first.clone()]), 400);
    let other = capture(engine(resynth.clone(), vec![second.clone()]), 400);
    let two = capture(engine(resynth, vec![first, second]), 400);
    assert!(
        energy(&one) > 0.001,
        "selected Resynth carrier did not sound"
    );
    assert!(
        difference(&one, &two) > 0.001,
        "second Resynth party did not affect output"
    );
    assert!(
        difference(&other, &two) > 0.001,
        "second Resynth party replaced the first"
    );
}

fn spectral_energy(samples: &[f32], hz: f32) -> f32 {
    let (mut sin, mut cos) = (0.0, 0.0);
    for (i, sample) in samples.iter().enumerate() {
        let phase = std::f32::consts::TAU * hz * i as f32 / RATE as f32;
        sin += sample * phase.sin();
        cos += sample * phase.cos();
    }
    (sin * sin + cos * cos).sqrt()
}

#[test]
fn generated_history_keeps_each_partys_pitch_without_transposing_it_twice() {
    let mut preset = Kind::Synth.preset().unwrap();
    let synth = preset.synth.as_mut().unwrap();
    synth.granular = true;
    synth.buffer_seconds = 0.25;
    synth.oscillators[0].waveform = crate::resynth::Waveform::Sine;
    preset.params.mix = 1.0;
    preset.params.position = 0.0;
    preset.params.grain_size = 0.5;
    preset.params.grains = 0.9;
    preset.params.cloud = 0.0;
    preset.params.detune = 0.5;
    let mut first = track(1, 60, SYNTH_A, true);
    first.env_gate = false;
    let mut second = track(2, 72, SYNTH_A, true);
    second.env_gate = false;
    let mut e = engine(preset.clone(), vec![first, second.clone()]);
    advance(&mut e, 4_000);
    let bank = e.preset_runtime.sequence_histories.as_ref().unwrap();
    let live = e.live_input.as_ref().unwrap();
    let range = live.range();
    let epoch = live.range_epoch();
    let render_history = |owner| {
        let slot = bank.slot(owner).unwrap();
        (0..1024)
            .map(|i| {
                bank.read(
                    slot,
                    (range.end - 1024 + i) as f64,
                    &range,
                    epoch,
                    1.0,
                    false,
                )[0]
            })
            .collect::<Vec<_>>()
    };
    let low = render_history(1);
    let high = render_history(2);
    assert!(spectral_energy(&low, 261.63) > spectral_energy(&low, 523.25) * 2.0);
    assert!(spectral_energy(&high, 523.25) > spectral_energy(&high, 261.63) * 2.0);
    let mut found = false;
    for _ in 0..2_000 {
        e.process_frame();
        for grain in e.live_grain_list() {
            if e.voices[grain.voice].owner == 2 {
                assert!(grain.history.is_some());
                assert!(
                    (grain.rate.abs() - 1.0).abs() < 1e-4,
                    "already-pitched history was transposed again"
                );
                found = true;
            }
        }
        if found {
            break;
        }
    }
    assert!(found, "the second party never played a history grain");

    let output = capture(engine(preset, vec![second]), 12_000);
    let output = &output[4_000..];
    let fundamental = spectral_energy(output, 523.25);
    let doubled = spectral_energy(output, 1046.5);
    assert!(fundamental > 0.01, "generated history produced no note");
    assert!(
        fundamental > doubled * 1.5,
        "generated history sounded an octave too high: {fundamental} / {doubled}"
    );
}

#[test]
fn unpitched_rack_history_is_transposed_for_each_party() {
    use crate::rack::{Module, Slot};
    let mut preset = Kind::Drums.rack_instrument().unwrap();
    let rack = preset.rack.as_mut().unwrap();
    rack.modules.insert(
        1,
        Slot {
            module: Module::Grains,
            enabled: true,
            id: 0,
        },
    );
    rack.buffer_seconds = 0.25;
    preset.params.grain_size = 0.5;
    preset.params.grains = 0.95;
    preset.params.detune = 0.5;
    let mut low = track(1, 60, 0, true);
    let mut high = track(2, 72, 0, true);
    low.env_gate = false;
    high.env_gate = false;
    let mut e = engine(preset, vec![low, high]);
    let mut found = [false; 2];
    for _ in 0..4_000 {
        e.process_frame();
        for grain in e.live_grain_list() {
            match e.voices[grain.voice].owner {
                1 => {
                    assert!((grain.rate.abs() - 1.0).abs() < 1e-4);
                    found[0] = true;
                }
                2 => {
                    assert!((grain.rate.abs() - 2.0).abs() < 1e-4);
                    found[1] = true;
                }
                _ => {}
            }
        }
        if found.into_iter().all(|v| v) {
            break;
        }
    }
    assert!(!e.individual_history());
    assert!(
        found.into_iter().all(|v| v),
        "both parties should generate grains"
    );
}

#[test]
fn sequenced_303_and_keyboard_coexist_and_the_release_survives_stop() {
    let preset = instrument(Kind::Tb303);
    let mut t = track(1, 48, TB303_VCO, false);
    t.env_gate = false;
    let mut solo = engine(preset.clone(), vec![t.clone()]);
    let mut together = engine(preset, vec![t]);
    advance(&mut solo, 200);
    advance(&mut together, 200);
    together.midi(Message::NoteOn {
        channel: 0,
        note: 72,
        velocity: 100,
    });
    let mut difference = 0.0_f32;
    for _ in 0..400 {
        difference += (solo.process_frame()[0] - together.process_frame()[0]).abs();
    }
    assert!(
        difference > 0.01,
        "the keyboard 303 was dropped while a party played"
    );
    solo.set_sequence_transport(false);
    let tail: Vec<_> = (0..200).map(|_| solo.process_frame()[0]).collect();
    assert!(energy(&tail) > 0.001, "stopping cut off the 303 release");
}

#[test]
fn envelope_off_sustains_through_zero_sustain_and_releases_on_stop() {
    for (kind, mask) in [
        (Kind::Synth, SYNTH_A),
        (Kind::Wavetable, WAVETABLE_A),
        (Kind::Oscillators, 1),
        (Kind::Tb303, TB303_VCO),
    ] {
        let mut lane = track(1, 60, mask, false);
        lane.env_gate = false;
        let mut e = engine(instrument(kind), vec![lane]);
        let mut preset = e.preset.clone();
        preset.envs[0].decay = 0.0;
        preset.envs[0].sustain = 0.0;
        e.set_preset(preset);
        advance(&mut e, 4_000);
        let index = held(&e, 1).expect("drone stopped being held");
        assert!(
            e.voices[index].value > 0.5,
            "{kind:?}: zero ENV sustain silenced the drone"
        );
        let output: Vec<_> = (0..400).map(|_| e.process_frame()[0]).collect();
        assert!(energy(&output) > 0.001, "{kind:?}: held drone is inaudible");
        e.set_sequence_transport(false);
        let output: Vec<_> = (0..400).map(|_| e.process_frame()[0]).collect();
        assert!(energy(&output) > 0.001, "{kind:?}: release tail was cut");
    }
}

#[test]
fn sequenced_303_drives_its_own_external_filter_envelope_and_meter() {
    use crate::preset::EnvSource;
    use crate::rack::{Module, Slot};
    let mut preset = instrument(Kind::Tb303);
    preset.rack.as_mut().unwrap().modules.push(Slot {
        module: Module::Filters,
        enabled: true,
        id: 0,
    });
    preset.params.filter = 0.1;
    preset.params.filter_env = EnvSource::Tb303;
    preset.params.filter_env_depth = 0.7;
    preset.params.filter_keytrack = 0.0;
    let mut e = engine(preset, vec![track(1, 72, TB303_VCO, false)]);
    advance(&mut e, 300);
    let index = held(&e, 1).unwrap();
    let env = e.voice_tb303_envelope(index);
    assert!(env > 0.01, "303 voice envelope did not attack");
    assert!(
        (e.tb303_view().note - 72.0).abs() < 0.1,
        "303 telemetry is not following the party"
    );
    let cutoff = e.voice_filter_cutoffs(72).unwrap()[0];
    let closed = filter::voice_cutoffs(
        &e.modulated.params,
        72.0,
        filter::envelopes(e.voices[index].env_values, 0.0),
    )[0];
    assert!(
        cutoff > closed,
        "the party's external filter ignored its 303 envelope"
    );
}

#[test]
fn resynth_keeps_main_pitch_and_keyboard_notes_independent() {
    let mut preset = Preset::default();
    preset.resynth.enabled = true;
    preset.resynth.mix = 1.0;
    preset.resynth.attack = 0.0;
    preset.resynth.threshold = 0.0;
    let lane = track(1, 60, RESYNTH_A, false);
    let mut solo = engine(preset.clone(), vec![lane.clone()]);
    let mut together = engine(preset, vec![lane]);
    advance(&mut solo, 300);
    advance(&mut together, 300);
    together.midi(Message::NoteOn {
        channel: 0,
        note: 72,
        velocity: 100,
    });
    let mut delta = 0.0;
    for _ in 0..500 {
        delta += (solo.process_frame()[0] - together.process_frame()[0]).abs();
    }
    assert!(delta > 0.001, "the keyboard Resynth voice was dropped");
    assert!(
        together
            .resynth_meter()
            .bands
            .iter()
            .any(|level| *level > 0.0)
    );
}

#[test]
fn generated_grains_are_not_transposed_a_second_time_by_resynth() {
    let mut preset = Kind::Synth.preset().unwrap();
    let synth = preset.synth.as_mut().unwrap();
    synth.granular = true;
    synth.buffer_seconds = 0.25;
    synth.oscillators[0].waveform = crate::resynth::Waveform::Sine;
    preset.params.mix = 1.0;
    preset.params.position = 0.0;
    preset.params.grain_size = 0.5;
    preset.params.grains = 0.9;
    preset.params.cloud = 0.0;
    preset.params.detune = 0.5;
    preset.resynth.enabled = true;
    preset.resynth.mix = 1.0;
    preset.resynth.input = crate::resynth::Input::Grains;
    preset.resynth.track = 1.0;
    preset.resynth.tracking = crate::resynth::Tracking::Follow;
    preset.resynth.attack = 0.0;
    preset.resynth.threshold = 0.0;
    let mut lane = track(1, 72, SYNTH_A | RESYNTH_A, true);
    lane.env_gate = false;
    let output = capture(engine(preset, vec![lane]), 16_000);
    let output = &output[5_000..];
    let fundamental = spectral_energy(output, 523.25);
    let doubled = spectral_energy(output, 1046.5);
    assert!(
        fundamental > 0.001,
        "Resynth's pitched input produced no fundamental"
    );
    assert!(
        fundamental > doubled * 1.5,
        "Resynth sounded an octave too high: {fundamental} / {doubled}"
    );
}

#[test]
fn rack_generated_histories_keep_parties_separate() {
    use crate::rack::{Module, Slot};
    for (kind, mask) in [
        (Kind::Wavetable, WAVETABLE_A),
        (Kind::Oscillators, 1),
        (Kind::Tb303, TB303_VCO),
    ] {
        let mut preset = instrument(kind);
        let rack = preset.rack.as_mut().unwrap();
        rack.modules.insert(
            1,
            Slot {
                module: Module::Grains,
                enabled: true,
                id: 0,
            },
        );
        rack.buffer_seconds = 0.25;
        preset.params.mix = 1.0;
        preset.params.position = 0.0;
        preset.params.grain_size = 0.5;
        preset.params.grains = 0.9;
        preset.params.cloud = 0.0;
        preset.params.detune = 0.5;
        let mut low = track(1, 60, mask, true);
        let mut high = track(2, 72, mask, true);
        low.env_gate = false;
        high.env_gate = false;
        let mut both = engine(preset.clone(), vec![low.clone(), high]);
        let mut solo = engine(preset, vec![low]);
        advance(&mut both, 3_500);
        advance(&mut solo, 3_500);
        assert!(
            both.individual_history(),
            "{kind:?}: note history was marked as shared"
        );
        let history = |e: &Engine, owner| {
            let bank = e.preset_runtime.sequence_histories.as_ref().unwrap();
            let live = e.live_input.as_ref().unwrap();
            let range = live.range();
            let slot = bank.slot(owner).unwrap();
            (0..512)
                .map(|i| {
                    bank.read(
                        slot,
                        (range.end - 512 + i) as f64,
                        &range,
                        live.range_epoch(),
                        1.0,
                        false,
                    )[0]
                })
                .collect::<Vec<_>>()
        };
        let first = history(&both, 1);
        let second = history(&both, 2);
        let isolated = history(&solo, 1);
        assert!(energy(&first) > 0.01, "{kind:?}: party history is silent");
        assert!(
            energy(&second) > 0.01,
            "{kind:?}: second party history is silent"
        );
        assert!(
            difference(&first, &second) > 0.01,
            "{kind:?}: both parties read the same history"
        );
        assert!(
            difference(&first, &isolated) < energy(&isolated) * 0.02 + 1e-4,
            "{kind:?}: other party leaked into the first history"
        );
    }
}

#[test]
fn rack_grain_bypass_keeps_each_selected_oscillator_audible() {
    use crate::rack::{Module, Slot};
    for (kind, mask) in [
        (Kind::Wavetable, WAVETABLE_A),
        (Kind::Oscillators, 1),
        (Kind::Tb303, TB303_VCO),
    ] {
        let mut preset = instrument(kind);
        preset.rack.as_mut().unwrap().modules.insert(
            1,
            Slot {
                module: Module::Grains,
                enabled: true,
                id: 0,
            },
        );
        preset.params.mix = 1.0;
        preset.params.grains = 0.8;
        let mut lane = track(1, 60, mask, false);
        lane.env_gate = false;
        let output = capture(engine(preset, vec![lane]), 1_000);
        assert!(
            energy(&output[400..]) > 0.01,
            "{kind:?}: disabling its grains cut the oscillator"
        );
    }
}
