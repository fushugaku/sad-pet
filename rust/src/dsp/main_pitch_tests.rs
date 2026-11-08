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
fn main_pitch_grains_route_selects_control_without_muting_the_file_cloud() {
    let mut granular = Preset::default();
    granular.params.mix = 1.0;
    granular.params.grains = 0.8;
    granular.params.cloud = 0.0;
    granular.params.detune = 0.5;
    let on = capture(engine(granular.clone(), vec![track(1, 60, 0, true)]), 450);
    let off = capture(engine(granular, vec![track(1, 60, 0, false)]), 450);
    assert!(energy(&on) > 0.01, "selected grains did not sound");
    assert!(
        energy(&off) > 0.01,
        "removing pitch control muted the physical Grain engine"
    );
    let osc = capture(
        engine(instrument(Kind::Synth), vec![track(1, 60, SYNTH_A, false)]),
        450,
    );
    assert!(energy(&osc) > 0.01, "oscillator-only party did not sound");
}

#[test]
fn file_grain_parties_start_each_note_while_previous_notes_release() {
    for rack in [false, true] {
        for midi_gate in [false, true] {
            let mut preset = Preset::default();
            preset.params.mix = 1.0;
            preset.params.grains = 0.48;
            preset.params.grain_size = 0.69;
            preset.params.grain_attack = 1.0;
            preset.params.cloud = 0.0;
            preset.params.detune = 0.5;
            if rack {
                preset = preset.as_rack().unwrap();
            }
            let mut slow = track(1, 60, 0, true);
            let mut fast = track(2, 72, 0, true);
            slow.config[field::DIVISION] = 4;
            fast.config[field::DIVISION] = 1;
            slow.config[field::GATE] = 55;
            fast.config[field::GATE] = 55;
            slow.steps.fill(Stage {
                pitch: 60,
                gate: 2,
                ..Stage::default()
            });
            fast.steps.fill(Stage {
                pitch: 72,
                gate: 2,
                ..Stage::default()
            });
            // engine() sets a long ENV 1 release, so several old notes still
            // overlap when the next pair of gates opens.
            let mut e = engine(preset, vec![slow, fast]);
            e.set_midi_gate(midi_gate);
            let mut current = [None; 2];
            let mut received = [false; 2];
            let mut finished = [0; 2];
            let mut missed = [0; 2];
            for _ in 0..RATE as usize * 4 {
                e.process_frame();
                for party in 0..2 {
                    let now = held(&e, (party + 1) as u32);
                    if now != current[party] {
                        if current[party].is_some() {
                            finished[party] += 1;
                            missed[party] += usize::from(!received[party]);
                        }
                        current[party] = now;
                        received[party] = false;
                    }
                    if let Some(voice) = now {
                        received[party] |= e
                            .live_grain_list()
                            .any(|grain| !grain.shared && grain.voice == voice);
                    }
                }
            }
            assert!(finished[0] >= 4 && finished[1] >= 16, "{finished:?}");
            assert_eq!(
                missed,
                [0, 0],
                "notes ended without their own grains: rack={rack}, midi_gate={midi_gate}, finished={finished:?}"
            );
        }
    }
}

#[test]
fn independent_grain_groups_start_both_parties_and_respect_all_groups_off() {
    use crate::interaction::Scheduler;
    for active in [false, true] {
        for attack in [0.0, 1.0] {
            let mut preset = Preset::default();
            preset.params.mix = 1.0;
            preset.params.grains = 0.2;
            preset.grain_groups.advanced = true;
            preset.grain_scheduler = Scheduler::Independent;
            for (i, group) in preset.grain_groups.groups.iter_mut().enumerate() {
                group.share = f32::from(active && i == 0);
                group.attack = attack;
            }
            let mut e = engine(preset, vec![track(1, 60, 0, true), track(2, 72, 0, true)]);
            let mut owners = [false; 2];
            for _ in 0..RATE / 2 {
                e.process_frame();
                for grain in e.live_grain_list() {
                    match e.voices[grain.voice].owner {
                        1 => owners[0] = true,
                        2 => owners[1] = true,
                        _ => {}
                    }
                }
                if !active {
                    assert_eq!(e.active_grains(), 0, "attack={attack}");
                }
            }
            if active {
                assert_eq!(owners, [true, true], "attack={attack}");
            }
        }
    }
}

#[test]
fn file_grain_parties_share_the_full_grain_pool() {
    let mut preset = Preset::default();
    preset.params.mix = 1.0;
    preset.params.grains = 1.0;
    preset.params.grain_size = 1.0;
    preset.params.grain_attack = 1.0;
    preset.params.cloud = 0.0;
    preset.params.detune = 0.5;
    let mut first = track(1, 60, 0, true);
    let mut second = track(2, 72, 0, true);
    for lane in [&mut first, &mut second] {
        lane.config[field::DIVISION] = 32;
        for step in &mut lane.steps {
            step.gate = 3;
        }
    }
    let mut e = engine(preset, vec![first, second]);
    advance(&mut e, RATE as usize * 2);
    assert_eq!(e.active_grains(), MAX_GRAINS);
    let mut owners = [0; 2];
    for grain in e.live_grain_list() {
        match e.voices[grain.voice].owner {
            1 => owners[0] += 1,
            2 => owners[1] += 1,
            _ => {}
        }
    }
    assert!(
        owners[0] >= 32 && owners[1] >= 32,
        "one of the parties was starved in a full grain pool: {owners:?}"
    );
}

#[test]
fn file_grain_new_party_can_start_when_the_pool_is_full() {
    let mut preset = Preset::default();
    preset.params.mix = 1.0;
    preset.params.grains = 1.0;
    preset.params.grain_size = 1.0;
    preset.params.grain_attack = 1.0;
    preset.params.cloud = 0.0;
    preset.params.detune = 0.5;
    let mut first = track(1, 60, 0, true);
    first.config[field::DIVISION] = 32;
    first.steps.fill(Stage {
        pitch: 60,
        gate: 3,
        ..Stage::default()
    });
    let mut second = track(2, 72, 0, true);
    second.config[field::DIVISION] = 1;
    second.config[field::GATE] = 55;
    second.config[field::MUTE] = 1;
    second.steps.fill(Stage {
        pitch: 72,
        gate: 2,
        ..Stage::default()
    });
    let mut e = engine(preset, vec![first, second]);
    advance(&mut e, RATE as usize * 3 / 2);
    assert_eq!(e.active_grains(), MAX_GRAINS);
    e.apply_sequence_edits(&[Edit {
        track: 2,
        stage: None,
        field: field::MUTE as u8,
        value: 0.0,
    }]);
    let mut opened = false;
    let mut heard = false;
    for _ in 0..RATE / 2 {
        e.process_frame();
        if let Some(voice) = held(&e, 2) {
            opened = true;
            heard |= e
                .live_grain_list()
                .any(|grain| !grain.shared && grain.voice == voice);
        } else if opened {
            break;
        }
    }
    assert!(opened, "the second gate never opened");
    assert!(
        heard,
        "the second gate ended while all grain slots were occupied"
    );
}

#[test]
fn file_grain_control_refresh_does_not_advance_clouds_without_audio() {
    for rack in [false, true] {
        let mut preset = Preset::default();
        preset.params.mix = 1.0;
        preset.params.grains = 0.8;
        preset.params.grain_size = 0.8;
        preset.params.grain_attack = 1.0;
        preset.params.cloud = 0.0;
        preset.params.detune = 0.5;
        if rack {
            preset = preset.as_rack().unwrap();
        }
        let tracks = vec![track(1, 60, 0, true), track(2, 72, 0, true)];
        let mut refreshed = engine(preset.clone(), tracks.clone());
        let mut reference = engine(preset, tracks);
        for _ in 0..1234 {
            assert_eq!(
                refreshed.process_frame().map(f32::to_bits),
                reference.process_frame().map(f32::to_bits),
            );
        }
        assert!(!refreshed.frame.is_multiple_of(CONTROL_INTERVAL));
        assert!(refreshed.active_grains() > 0);
        refreshed.refresh_parameters();
        for frame in 0..1024 {
            assert_eq!(
                refreshed.process_frame().map(f32::to_bits),
                reference.process_frame().map(f32::to_bits),
                "unchanged control refresh changed audio: rack={rack}, frame={frame}"
            );
        }
    }
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
fn file_grain_parties_sound_together_and_release_independently() {
    for rack in [false, true] {
        for midi_gate in [false, true] {
            let mut preset = Preset::default();
            preset.params.mix = 1.0;
            preset.params.grains = 0.9;
            preset.params.grain_size = 0.8;
            preset.params.cloud = 0.0;
            preset.params.detune = 0.5;
            if rack {
                preset = preset.as_rack().unwrap();
            }
            let mut low = track(1, 60, 0, true);
            let mut high = track(2, 72, 0, true);
            for lane in [&mut low, &mut high] {
                lane.config[field::DIVISION] = 32;
                for step in &mut lane.steps {
                    step.gate = 3;
                }
            }
            let mut e = engine(preset, vec![low, high]);
            let mut adjusted = e.preset.clone();
            adjusted.envs[0].release = 0.0;
            e.set_preset(adjusted);
            e.set_midi_gate(midi_gate);
            advance(&mut e, RATE as usize);
            let (a, b) = (held(&e, 1).unwrap(), held(&e, 2).unwrap());
            assert_ne!(a, b);
            assert!(e.voices[a].value > 0.1 && e.voices[b].value > 0.1);
            let before: Vec<_> = (0..RATE / 2).map(|_| e.process_frame()[0]).collect();
            let before_low = spectral_energy(&before, 220.0);
            let before_high = spectral_energy(&before, 440.0);
            assert!(
                before_low > 0.1 && before_high > 0.1,
                "both pitches must sound: rack={rack}, midi_gate={midi_gate}, {before_low}/{before_high}"
            );
            e.apply_sequence_edits(&[Edit {
                track: 1,
                stage: None,
                field: field::MUTE as u8,
                value: 1.0,
            }]);
            advance(&mut e, RATE as usize / 2);
            assert!(held(&e, 1).is_none());
            assert!(held(&e, 2).is_some());
            assert!(e.voices[b].value > 0.1);
            let after: Vec<_> = (0..RATE / 2).map(|_| e.process_frame()[0]).collect();
            let after_low = spectral_energy(&after, 220.0);
            let after_high = spectral_energy(&after, 440.0);
            assert!(
                after_low < before_low * 0.2 + 0.05 && after_high > before_high * 0.2,
                "muting one party changed the other: rack={rack}, midi_gate={midi_gate}, before={before_low}/{before_high}, after={after_low}/{after_high}"
            );
        }
    }
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
    let output = capture(engine(preset.clone(), vec![lane.clone()]), 16_000);
    let output = &output[5_000..];
    let fundamental = spectral_energy(output, 523.25);
    assert!(
        fundamental > 0.001,
        "Resynth's pitched input produced no fundamental"
    );

    // Give Resynth the exact same already-pitched input for voices an octave
    // apart. The note must not transpose grains that already carry pitch.
    let render_input = |input, pitch| {
        let mut preset = preset.clone();
        preset.resynth.input = input;
        let mut engine = engine(preset, vec![lane.clone()]);
        advance(&mut engine, 32);
        let index = held(&engine, 1).unwrap();
        engine.voices[index].pitch = pitch;
        engine.voices[index].generation = engine.voices[index].generation.wrapping_add(1);
        (0..8_000)
            .map(|i| {
                let sample = (std::f32::consts::TAU * 523.25 * i as f32 / RATE as f32).sin() * 0.3;
                engine.render_main_resynth(index, [0.0; 2], [sample; 2], [sample; 2], false)[0]
            })
            .collect::<Vec<_>>()
    };
    let grains_low = render_input(crate::resynth::Input::Grains, 60.0);
    let grains_high = render_input(crate::resynth::Input::Grains, 72.0);
    let source_low = render_input(crate::resynth::Input::Source, 60.0);
    let source_high = render_input(crate::resynth::Input::Source, 72.0);
    let grains_energy = energy(&grains_low);
    let grains_delta = difference(&grains_low, &grains_high);
    assert!(
        grains_energy > 0.001 && grains_delta < grains_energy * 1e-5,
        "Resynth transposed the already-pitched grains: {grains_delta} / {grains_energy}"
    );
    let source_energy = energy(&source_low);
    let source_delta = difference(&source_low, &source_high);
    assert!(
        source_energy > 0.001 && source_delta > source_energy * 0.01,
        "the octave control did not change the unpitched source: {source_delta} / {source_energy}"
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
fn rack_unbound_grains_still_capture_each_selected_oscillator() {
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
        preset.rack.as_mut().unwrap().buffer_seconds = 0.25;
        preset.params.mix = 1.0;
        preset.params.grains = 0.8;
        preset.params.position = 0.0;
        preset.params.cloud = 0.0;
        preset.params.detune = 0.5;
        let mut lane = track(1, 60, mask, false);
        lane.env_gate = false;
        let mut e = engine(preset, vec![lane]);
        advance(&mut e, 4_000);
        assert!(
            energy(&history_tail(&e, None, 512)) > 0.01,
            "{kind:?}: the shared capture was empty"
        );
        assert!(
            e.live_grain_list().any(|grain| grain.shared),
            "{kind:?}: the independent cloud never spawned"
        );
        let output = capture(e, 512);
        assert!(
            energy(&output) > 0.01,
            "{kind:?}: removing note control silenced the captured grains"
        );
    }
}

fn oscillator_grain_rack(grains_enabled: bool, filters_enabled: bool) -> Preset {
    use crate::rack::{Module, Slot};
    let mut preset = instrument(Kind::Oscillators);
    let rack = preset.rack.as_mut().unwrap();
    rack.modules = [
        (Module::Oscillators, true),
        (Module::Grains, grains_enabled),
        (Module::Filters, filters_enabled),
        (Module::Mixer, true),
        (Module::PostFx, true),
    ]
    .into_iter()
    .map(|(module, enabled)| Slot {
        module,
        enabled,
        id: 0,
    })
    .collect();
    rack.buffer_seconds = 0.25;
    preset.params.mix = 1.0;
    preset.params.position = 0.0;
    preset.params.grain_size = 0.5;
    preset.params.grains = 0.9;
    preset.params.cloud = 0.0;
    preset.params.detune = 0.5;
    preset.params.filter_keytrack = 0.0;
    preset.params.filter2_keytrack = 0.0;
    preset
}

fn history_tail(engine: &Engine, owner: Option<u32>, count: usize) -> Vec<f32> {
    let live = engine
        .live_input
        .as_ref()
        .expect("prepared capture history");
    let range = live.range();
    assert!(range.len() >= count);
    if let Some(owner) = owner {
        let bank = engine.preset_runtime.sequence_histories.as_ref().unwrap();
        let slot = bank.slot(owner).expect("party history slot");
        (range.end - count..range.end)
            .map(|position| {
                bank.read(
                    slot,
                    position as f64,
                    &range,
                    live.range_epoch(),
                    1.0,
                    false,
                )[0]
            })
            .collect()
    } else {
        live.sample.frames()[range.end - count..range.end]
            .iter()
            .map(|frame| frame[0])
            .collect()
    }
}

#[test]
fn owned_file_voice_zero_and_independent_grains_keep_separate_onsets_and_returns() {
    let mut preset = Preset::default();
    preset.params.grain_size = 0.8;
    preset.params.grain_attack = 0.8;
    preset.params.cloud = 0.0;
    preset.params.detune = 0.5;
    preset.color_chain.clear();
    let mut e = Engine::new(RATE, sample(), preset);
    e.set_midi_gate(false);
    e.set_playing(true);
    assert!(e.voices[0].active());
    assert_eq!(
        e.grain_voice(false),
        Some((0, false)),
        "ordinary free-file voice 0 lost note ownership"
    );

    // Deliberately overlap the old note-owned cloud and the new independent
    // cloud in logical slot 0. Their DSP returns must remain distinct.
    e.live_grains = 0;
    e.spawn_grain(0, false, 0, false);
    e.spawn_grain(0, false, 0, true);
    assert_eq!(e.live_grain_list().filter(|grain| !grain.shared).count(), 1);
    assert_eq!(e.live_grain_list().filter(|grain| grain.shared).count(), 1);
    e.onsets |= 1;
    e.voices[0].onset[crate::grain_groups::GROUPS] = 0.0;
    e.voices[0].onset_step[crate::grain_groups::GROUPS] = 0.0;
    let mut shared_energy = 0.0;
    for _ in 0..64 {
        let (owned, _, owners, shared) = e.render_grains(false);
        assert_eq!(owned[0], [0.0; 2], "ordinary file ATTACK was bypassed");
        assert_ne!(owners & 1, 0);
        shared_energy += shared[0].abs() + shared[1].abs();
    }
    assert!(
        shared_energy > 1e-4,
        "shared grains inherited voice 0's onset gate"
    );
    e.kill_voice_grains(0);
    assert!(
        e.live_grain_list().any(|grain| grain.shared),
        "reusing voice 0 killed an independent tail"
    );
    assert!(e.live_grain_list().all(|grain| grain.shared));
}

#[test]
fn mixed_private_and_independent_histories_survive_release_and_freeze() {
    let preset = oscillator_grain_rack(true, false);
    let mut private = track(1, 60, 1, true);
    let mut independent = track(2, 72, 1, false);
    private.env_gate = false;
    independent.env_gate = false;
    let mut both = engine(preset.clone(), vec![private.clone(), independent]);
    let mut isolated = engine(preset, vec![private]);
    advance(&mut both, 4_000);
    advance(&mut isolated, 4_000);
    let own = history_tail(&both, Some(1), 512);
    let reference = history_tail(&isolated, Some(1), 512);
    let shared = history_tail(&both, None, 512);
    assert!(energy(&own) > 0.01 && energy(&shared) > 0.01);
    assert!(
        difference(&own, &reference) < energy(&reference) * 0.02 + 1e-4,
        "the independent source leaked into the private party's history"
    );
    assert!(
        energy(&history_tail(&both, Some(2), 512)) < 1e-7,
        "the independent party also recorded into a private tape"
    );
    // At grain_size=0.5 a grain lasts only four frames: a single snapshot can
    // legitimately land between births in either half of the round robin.
    let mut saw_shared = false;
    let mut saw_private = false;
    for _ in 0..512 {
        both.process_frame();
        saw_shared |= both.live_grain_list().any(|grain| grain.shared);
        saw_private |= both
            .live_grain_list()
            .any(|grain| !grain.shared && grain.history.is_some());
    }
    assert!(
        saw_shared && saw_private,
        "both grain domains must receive births"
    );

    both.set_frozen(true);
    let frozen_shared = history_tail(&both, None, 512);
    let frozen_private = history_tail(&both, Some(1), 512);
    let mut short_release = both.preset.clone();
    short_release.envs[0].release = 0.0;
    both.set_preset(short_release);
    both.set_sequence_transport(false);
    let tail: Vec<_> = (0..512).map(|_| both.process_frame()[0]).collect();
    assert!(
        energy(&tail) > 0.01,
        "note release cut the frozen independent cloud"
    );
    assert_eq!(history_tail(&both, None, 512), frozen_shared);
    assert_eq!(history_tail(&both, Some(1), 512), frozen_private);
}

#[test]
fn unbound_oscillator_capture_obeys_chain_and_selected_oscillator_modes() {
    use crate::interaction::CaptureMode;
    let mut captured = Vec::new();
    for (mode, slots) in [
        (CaptureMode::Chain, 3),
        (CaptureMode::Osc, 1),
        (CaptureMode::Osc, 2),
    ] {
        let mut preset = oscillator_grain_rack(true, false);
        preset.grain_capture.mode = mode;
        preset.grain_capture.slots = slots;
        let mut lane = track(1, 60, 3, false);
        lane.env_gate = false;
        let mut e = engine(preset, vec![lane]);
        advance(&mut e, 4_000);
        let tape = history_tail(&e, None, 512);
        assert!(energy(&tape) > 0.01, "capture was silent for slots={slots}");
        assert!(e.live_grain_list().any(|grain| grain.shared));
        assert_eq!(
            e.sounding & 1,
            0,
            "the grain fallback started a ghost oscillator voice"
        );
        captured.push(tape);
    }
    assert!(
        difference(&captured[1], &captured[2]) > 0.1,
        "the selected oscillator did not affect captured PCM"
    );
    let sum: Vec<_> = captured[1]
        .iter()
        .zip(&captured[2])
        .map(|(a, b)| a + b)
        .collect();
    assert!(
        difference(&captured[0], &sum) < energy(&sum) * 0.01 + 1e-4,
        "Chain capture did not contain the selected oscillator streams once each"
    );
}

fn render_filter_envelope(
    granular: bool,
    grains_enabled: bool,
    sustain: f32,
    ws: bool,
) -> Vec<f32> {
    let mut preset = oscillator_grain_rack(grains_enabled, true);
    preset.params.filter = if ws { 1.0 } else { 0.35 };
    preset.params.filter_env = crate::preset::EnvSource::Env2;
    preset.params.filter_env_depth = if ws { 0.0 } else { 0.8 };
    preset.envs[1].attack = 0.0;
    preset.envs[1].decay = 0.0;
    preset.envs[1].sustain = sustain;
    if ws {
        preset.params.filter_shapers[0].before = crate::waveshaper::Stage {
            enabled: true,
            mode: crate::color::WaveshaperMode::Decimator,
            depth: 0.0,
            options: crate::waveshaper::Options {
                envelope: 1.0,
                envelope_source: Some(crate::preset::EnvSource::Env2),
                ..Default::default()
            },
            ..Default::default()
        };
    }
    let mut lane = track(1, 60, 1, granular);
    lane.env_gate = false;
    let mut e = engine(preset, vec![lane]);
    advance(&mut e, 4_000);
    capture(e, 512)
}

#[test]
fn filters_after_history_grains_follow_envelopes_even_when_grains_are_bypassed() {
    for granular in [false, true] {
        for grains_enabled in [false, true] {
            let closed = render_filter_envelope(granular, grains_enabled, 0.0, false);
            let open = render_filter_envelope(granular, grains_enabled, 1.0, false);
            assert!(energy(&open) > 0.01);
            assert!(
                difference(&closed, &open) > energy(&open) * 0.05,
                "filter ENV did not reach its active bank: private={granular}, grains_on={grains_enabled}"
            );
        }
    }
}

#[test]
fn filter_waveshaper_after_grains_receives_private_and_shared_envelopes() {
    for granular in [false, true] {
        let low = render_filter_envelope(granular, true, 0.0, true);
        let high = render_filter_envelope(granular, true, 1.0, true);
        assert!(energy(&high) > 0.01);
        assert!(
            difference(&low, &high) > energy(&high) * 0.05,
            "filter WS ENV did not reach its active bank: private={granular}"
        );
    }
}

#[test]
fn independent_file_cloud_has_dry_and_wet_without_note_transpose_or_ghost_playback() {
    for rack in [false, true] {
        for mix in [0.0, 1.0] {
            let mut preset = if rack {
                let mut p = oscillator_grain_rack(true, false);
                p.oscillators = None;
                p.rack.as_mut().unwrap().modules[0].module = crate::rack::Module::Sample;
                p
            } else {
                Preset::default()
            };
            preset.params.mix = mix;
            preset.params.position = 0.0;
            preset.params.grains = 0.9;
            preset.params.cloud = 0.0;
            preset.params.detune = 0.5;
            preset.params.filter_keytrack = 0.0;
            let low = capture(engine(preset.clone(), vec![track(1, 60, 0, false)]), 2_048);
            let high = capture(engine(preset.clone(), vec![track(1, 84, 0, false)]), 2_048);
            assert!(
                energy(&low) > 0.01,
                "independent file mix={mix}, rack={rack} was silent"
            );
            assert!(
                difference(&low, &high) < energy(&low) * 1e-5 + 1e-5,
                "MainPitch transposed the unbound file path: mix={mix}, rack={rack}"
            );

            let mut e = engine(preset, vec![track(1, 60, 0, false)]);
            advance(&mut e, 256);
            assert_eq!(e.sounding & 1, 0);
            let mut release = e.preset.clone();
            release.envs[0].release = 0.0;
            e.set_preset(release);
            e.set_sequence_transport(false);
            advance(&mut e, 256);
            assert!(
                !e.file_grain_fallback(),
                "stopped note lanes started a default file player"
            );
            let remaining = e
                .live_grain_list()
                .map(|grain| grain.duration.saturating_sub(grain.age))
                .max()
                .unwrap_or(0);
            advance(&mut e, remaining as usize + 256);
            let silent = capture(e, 256);
            assert!(
                energy(&silent) < 1e-5,
                "stopping the sequencer left ghost file playback: mix={mix}, rack={rack}"
            );
        }
    }
}

#[test]
fn classic_independent_file_return_obeys_pre_fx_and_does_not_start_an_idle_sub() {
    for mix in [0.0, 1.0] {
        let mut preset = Preset::default();
        preset.params.mix = mix;
        preset.params.grains = 0.9;
        preset.params.cloud = 0.0;
        preset.params.detune = 0.5;
        let mut plain = engine(preset.clone(), vec![track(1, 60, 0, false)]);
        advance(&mut plain, 2_048);
        assert!(energy(&capture(plain, 512)) > 0.01);

        // A zero transfer in physical PreFX must silence both the independent
        // file dry and its grains. Clean2x also tests alignment of that return.
        let mut fx = crate::color::ColorSlot {
            id: 81,
            kind: crate::preset::SaturatorType::Waveshaper,
            enabled: true,
            mix: 1.0,
            ..Default::default()
        };
        fx.settings.waveshaper_type = crate::color::WaveshaperMode::Decimator;
        fx.settings.waveshaper_depth = 0.0;
        fx.settings.waveshaper_options = crate::waveshaper::Options {
            quality: crate::waveshaper::Quality::Clean2x,
            curve: crate::waveshaper::Curve::Custom,
            points: [0.0; 17],
            ..Default::default()
        };
        let mut processed = engine(preset, vec![track(1, 60, 0, false)]);
        let mut routed = processed.preset.clone();
        routed.pre_color_chain = vec![fx];
        processed.set_preset(routed);
        advance(&mut processed, 2_048);
        assert!(
            energy(&capture(processed, 512)) < 1e-5,
            "independent file mix={mix} bypassed its physical PreFX"
        );
    }

    let mut silent = Preset::default();
    silent.sequencer.enabled = false;
    silent.color_chain.clear();
    silent.pre_color_chain.clear();
    silent.params.filter_shapers[0].before = crate::waveshaper::Stage {
        enabled: true,
        mode: crate::color::WaveshaperMode::SubSine,
        depth: 1.0,
        ..Default::default()
    };
    let mut e = Engine::new(RATE, sample(), silent);
    e.set_midi_gate(true);
    e.set_playing(true);
    assert!(
        energy(&capture(e, 1_024)) < 1e-5,
        "an unused shared filter started a Sub oscillator without a cloud or a note"
    );
}
