use super::*;
use crate::grain_groups::Scale;

/// A file timbre in advanced mode where only group A plays.
fn engine(edit: impl FnOnce(&mut Preset)) -> Engine {
    let mut preset = Preset::default();
    preset.params.grains = 0.8;
    preset.params.cloud = 0.0;
    preset.params.detune = 0.5;
    preset.color_chain.clear();
    preset.grain_groups.advanced = true;
    for (i, g) in preset.grain_groups.groups.iter_mut().enumerate() {
        g.share = if i == 0 { 1.0 } else { 0.0 };
    }
    edit(&mut preset);
    let mut e = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), preset);
    e.set_playing(true);
    e
}
fn group_a(preset: &mut Preset) -> &mut crate::grain_groups::Group {
    &mut preset.grain_groups.groups[0]
}
/// Every live grain, as (rate, duration, voice), over a stretch of frames.
fn grains(e: &mut Engine, frames: usize) -> Vec<(f64, u32, usize)> {
    let mut seen = Vec::new();
    for _ in 0..frames {
        e.process_frame();
        for slot in e.live_grain_slots() {
            let g = &e.grains[slot];
            if g.age == 1 {
                seen.push((g.rate, g.duration, g.voice));
            }
        }
    }
    seen
}
/// A grain's pitch in semitones from its rate on the demo source.
fn semitones(e: &Engine, rate: f64) -> f64 {
    12.0 * (rate.abs() * e.sample_rate as f64 / e.sample.sample_rate as f64).log2()
}

#[test]
fn all_groups_off_stop_legacy_independent_and_forced_clock_births() {
    use crate::interaction::{Node, Route, Scheduler, Scope, SyncMode};
    for (scheduler, forced) in [
        (Scheduler::Legacy, false),
        (Scheduler::Independent, false),
        (Scheduler::Independent, true),
    ] {
        let mut e = engine(|p| {
            p.params.mix = 1.0;
            for group in &mut p.grain_groups.groups {
                group.share = 0.0;
            }
            p.grain_scheduler = scheduler;
            if forced {
                p.synth = Some(Box::default());
                p.interactions.routes.push(Route {
                    source: Node::Synth { index: 0 },
                    target: Node::Group { index: 0 },
                    sync: SyncMode::Hard,
                    scope: Scope::Both,
                    ..Route::default()
                });
            }
        });
        // A note's prefill and a forced direct birth must obey Off too.
        e.midi(Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 100,
        });
        e.spawn_grain_group((0, false), false, 0, Some(0), true, 0.0);
        assert_eq!(e.live_grains, 0);
        for _ in 0..24_000 {
            e.process_frame();
            assert_eq!(e.live_grains, 0, "{scheduler:?}, forced={forced}");
        }
    }
}

#[test]
fn switching_advanced_on_with_all_groups_off_only_finishes_existing_grains() {
    let mut e = engine(|p| {
        p.grain_groups.advanced = false;
        p.params.grain_size = 0.6;
        p.params.mix = 1.0;
        for group in &mut p.grain_groups.groups {
            group.share = 0.0;
        }
    });
    grains(&mut e, 8_000);
    let before = e.live_grains;
    assert_ne!(before, 0);
    let longest_tail = e
        .live_grain_slots()
        .map(|i| e.grains[i].duration - e.grains[i].age)
        .max()
        .unwrap();
    e.apply_parameters(
        &crate::parameters::parse(&serde_json::json!({"grain_advanced":1})).unwrap(),
    );
    for _ in 0..longest_tail + 128 {
        e.process_frame();
        assert_eq!(e.live_grains & !before, 0, "Off creates no replacements");
    }
    assert_eq!(e.live_grains, 0);
}

#[test]
fn a_groups_size_and_reverse_shape_its_grains() {
    let mut plain = engine(|_| {});
    let base = grains(&mut plain, 24_000);
    let mut short = engine(|p| group_a(p).size = -2.0);
    let quarter = grains(&mut short, 24_000);
    assert!(!base.is_empty() && !quarter.is_empty());
    let expected = (base[0].1 as f32 / 4.0).round() as u32;
    assert!(
        quarter.iter().all(|g| g.1.abs_diff(expected) <= 1),
        "{expected}"
    );
    // Longer grains stop at the engine's longest.
    let mut long = engine(|p| group_a(p).size = 2.0);
    let longest = (plain.grain_seconds(1.0) * 48_000.0).round() as u32;
    assert!(grains(&mut long, 24_000).iter().all(|g| g.1 <= longest));

    assert!(base.iter().all(|g| g.0 > 0.0), "the engine plays forwards");
    let mut back = engine(|p| group_a(p).reverse = 1.0);
    assert!(grains(&mut back, 24_000).iter().all(|g| g.0 < 0.0));
    let mut some = engine(|p| group_a(p).reverse = 0.5);
    let mixed = grains(&mut some, 48_000);
    let reversed = mixed.iter().filter(|g| g.0 < 0.0).count();
    assert!(reversed > mixed.len() / 4 && reversed < mixed.len() * 3 / 4);
}

#[test]
fn a_groups_position_moves_its_grains_from_the_engines() {
    let mut e = engine(|p| group_a(p).position = 0.25);
    let length = e.source_range().len() as f64;
    let mut checked = 0;
    for _ in 0..24_000 {
        e.process_frame();
        for slot in e.live_grain_slots() {
            let g = &e.grains[slot];
            if g.age == 1 {
                let start = g.position - g.rate;
                let expected = e.layer_center(g.voice, 0.25);
                let distance = (start - expected).rem_euclid(length);
                assert!(distance.min(length - distance) < 4.0, "{start} {expected}");
                let engine = e.voice_center(g.voice);
                let offset = (start - engine).rem_euclid(length) / length;
                assert!((offset - 0.25).abs() < 1e-3, "{offset}");
                checked += 1;
            }
        }
    }
    assert!(checked > 4);
}

#[test]
fn random_pitch_snaps_to_the_groups_scale() {
    let mut e = engine(|p| {
        let g = group_a(p);
        g.random_pitch = 12.0;
        g.scale = Scale::PentatonicMinor;
    });
    let pitches: Vec<f64> = grains(&mut e, 96_000)
        .iter()
        .map(|g| semitones(&e, g.0))
        .collect();
    let minor = Scale::PentatonicMinor.intervals().unwrap();
    let mut degrees = 0u16;
    for &pitch in &pitches {
        let whole = pitch.round();
        assert!((pitch - whole).abs() < 0.01, "{pitch} is a whole semitone");
        let step = (whole as i32).rem_euclid(12);
        assert!(minor & 1 << step != 0, "{pitch} is in the scale");
        degrees |= 1 << step;
    }
    assert!(
        degrees.count_ones() >= 4,
        "random pitch reaches many degrees"
    );
    // Without a scale the same random pitch slides between semitones.
    let mut free = engine(|p| group_a(p).random_pitch = 12.0);
    let pitches: Vec<f64> = grains(&mut free, 48_000)
        .iter()
        .map(|g| semitones(&free, g.0))
        .collect();
    assert!(pitches.iter().any(|p| (p - p.round()).abs() > 0.1));
}

#[test]
fn chord_scale_snaps_each_note_to_the_notes_held() {
    let mut e = engine(|p| {
        p.params.sample_root_note = 60.0;
        let g = group_a(p);
        g.random_pitch = 12.0;
        g.scale = Scale::Chord;
    });
    e.set_midi_gate(true);
    for note in [60, 64, 67] {
        e.midi(Message::NoteOn {
            channel: 0,
            note,
            velocity: 100,
        });
    }
    let mut classes = 0u16;
    for (rate, _, voice) in grains(&mut e, 96_000) {
        if voice == 0 {
            continue;
        }
        let pitch = 60.0 + semitones(&e, rate);
        let class = (pitch.round() as i32).rem_euclid(12);
        assert!([0, 4, 7].contains(&class), "{pitch} is a chord tone");
        classes |= 1 << class;
    }
    assert_eq!(classes, 1 | 1 << 4 | 1 << 7);
}
