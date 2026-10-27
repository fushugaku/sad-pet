use sad_pet::{
    dsp::{Engine, SampleBuffer},
    grain_groups::FilterType,
    midi::Message,
    parameters,
    preset::Preset,
};
use std::sync::Arc;

const RATE: u32 = 48_000;

/// Steady noise, so the grains' level shows only their own build-up.
fn noise() -> Arc<SampleBuffer> {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    let frames = (0..RATE as usize * 4)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let v = ((state >> 40) as f32 / (1 << 24) as f32 * 2.0 - 1.0) * 0.25;
            [v, v]
        })
        .collect();
    Arc::new(SampleBuffer::new("noise".into(), RATE, frames).unwrap())
}
fn engine(attack: f32, edit: impl FnOnce(&mut Preset)) -> Engine {
    let mut preset = Preset::default();
    preset.params.reverb = 0.0;
    preset.color_chain.clear();
    // 455 ms grains: from nothing, the cloud takes over 100 ms to sound.
    preset.params.grain_size = 0.8;
    preset.params.cloud = 0.0;
    preset.params.grain_attack = attack;
    edit(&mut preset);
    let mut e = Engine::new(RATE, noise(), preset);
    e.set_midi_gate(true);
    e.set_playing(true);
    // The output gain settles from silence first.
    for _ in 0..RATE / 4 {
        e.process_frame();
    }
    e
}
fn note_on(e: &mut Engine, note: u8) {
    e.midi(Message::NoteOn {
        channel: 0,
        note,
        velocity: 127,
    });
}
/// 10 ms RMS of a note for its first second, relative to its level from
/// 600 ms on.
fn onset(e: &mut Engine) -> Vec<f32> {
    note_on(e, 60);
    let frames: Vec<_> = (0..RATE).map(|_| e.process_frame()).collect();
    let rms: Vec<f32> = frames
        .chunks(480)
        .map(|c| (c.iter().map(|f| f[0] * f[0] + f[1] * f[1]).sum::<f32>() / c.len() as f32).sqrt())
        .collect();
    let held = rms[60..].iter().sum::<f32>() / (rms.len() - 60) as f32;
    rms.iter().map(|x| x / held).collect()
}
/// Milliseconds until a note reaches 90% of its level.
fn rise(levels: &[f32]) -> usize {
    levels.iter().position(|&x| x >= 0.9).unwrap() * 10
}

#[test]
fn presets_from_before_grow_their_grains_as_they_did() {
    assert_eq!(Preset::default().params.grain_attack, 1.0);
    let mut json = serde_json::to_value(Preset::default()).unwrap();
    json["params"]
        .as_object_mut()
        .unwrap()
        .remove("grain_attack");
    let preset: Preset = serde_json::from_value(json).unwrap();
    assert_eq!(preset.params.grain_attack, 1.0);
    let changes = parameters::parse(&serde_json::json!({ "grain_attack": 0.25 })).unwrap();
    let mut e = engine(1.0, |_| {});
    e.apply_parameters(&changes);
    assert_eq!(e.preset.params.grain_attack, 0.25);
}

#[test]
fn no_attack_sounds_the_whole_cloud_at_once() {
    let grown = onset(&mut engine(1.0, |_| {}));
    let played_in = onset(&mut engine(0.0, |_| {}));
    assert!(grown[0] < 0.1 && rise(&grown) >= 100, "{grown:?}");
    assert!(
        played_in[0] > 0.8 && rise(&played_in) <= 10,
        "{played_in:?}"
    );
    // A played-in cloud holds its level: no bump while its grains are young.
    assert!(played_in[..40].iter().all(|&x| x < 1.15), "{played_in:?}");
}

#[test]
fn attack_sets_how_long_the_cloud_fades_in() {
    let rises: Vec<usize> = [0.0, 0.25, 0.5, 0.75, 0.99]
        .into_iter()
        .map(|attack| {
            let levels = onset(&mut engine(attack, |_| {}));
            // Smooth: it never climbs past its level on the way.
            assert!(
                levels[..40].iter().all(|&x| x < 1.15),
                "{attack}: {levels:?}"
            );
            rise(&levels)
        })
        .collect();
    assert!(rises.windows(2).all(|r| r[0] < r[1]), "{rises:?}");
    // Near 100% it takes about as long as a cloud growing from none.
    let grown = rise(&onset(&mut engine(1.0, |_| {})));
    assert!(rises[4].abs_diff(grown) <= 50, "{rises:?} vs {grown}");
}

#[test]
fn a_sounding_note_keeps_its_grains_when_played_again() {
    let mut e = engine(0.0, |_| {});
    note_on(&mut e, 60);
    for _ in 0..RATE / 2 {
        e.process_frame();
    }
    let before = e.active_grains();
    note_on(&mut e, 60);
    assert!(e.active_grains() <= before + 1);
}

#[test]
fn grain_groups_play_in_with_their_filters_tuned() {
    let levels = onset(&mut engine(1.0, |p| {
        p.grain_groups.advanced = true;
        for group in &mut p.grain_groups.groups {
            group.filter_type = FilterType::LowPass;
            group.cutoff = 0.6;
            group.attack = 0.0;
        }
    }));
    assert!(levels.iter().all(|x| x.is_finite()));
    // Filtered noise wanders more: on average it starts at its level.
    let start = levels[..20].iter().sum::<f32>() / 20.0;
    assert!(
        levels[0] > 0.7 && (0.85..1.15).contains(&start),
        "{levels:?}"
    );
}

/// Advanced mode with groups A and B dealt half the grains each.
fn two_groups(a: f32, b: f32) -> impl FnOnce(&mut Preset) {
    move |p| {
        p.grain_groups.advanced = true;
        for (i, group) in p.grain_groups.groups.iter_mut().enumerate() {
            group.share = if i < 2 { 1.0 } else { 0.0 };
        }
        p.grain_groups.groups[0].attack = a;
        p.grain_groups.groups[1].attack = b;
    }
}

#[test]
fn each_group_plays_in_at_its_own_attack() {
    // The engine's ATTACK gives way to the groups' in advanced mode.
    let mut e = engine(0.0, two_groups(1.0, 1.0));
    note_on(&mut e, 60);
    assert_eq!(e.active_grains(), 0, "no group plays grains in");
    let mut e = engine(1.0, two_groups(0.0, 1.0));
    note_on(&mut e, 60);
    let views = e.grain_views();
    assert!(views.len() > 2, "group A plays in its share");
    assert!(
        views.iter().all(|g| g.group == 1),
        "group B grows from none"
    );
    // A group at 0% sounds at once, one at 50% fades in.
    let both = onset(&mut engine(1.0, two_groups(0.0, 0.0)));
    let slower = onset(&mut engine(1.0, two_groups(0.0, 0.5)));
    assert!(both[0] > 0.8, "{both:?}");
    assert!(
        slower[0] < both[0] * 0.85 && rise(&slower) > rise(&both),
        "{slower:?}"
    );
    let changes = parameters::parse(&serde_json::json!({ "grain_group2_attack": 0.3 })).unwrap();
    e.apply_parameters(&changes);
    assert_eq!(e.preset.grain_groups.groups[1].attack, 0.3);
}
