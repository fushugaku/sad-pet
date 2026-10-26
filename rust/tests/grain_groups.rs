use sad_pet::{
    dsp::{Engine, SampleBuffer},
    grain_groups::{Envelope, FilterType, GROUPS, Settings},
    parameters,
    preset::Preset,
    timbres::Kind,
};
use std::sync::Arc;

fn engine(edit: impl FnOnce(&mut Preset)) -> Engine {
    let mut preset = Preset::default();
    preset.params.master_volume = 0.3;
    // Dense, short grains: every group plays many of them per second.
    preset.params.grains = 0.8;
    preset.color_chain.clear();
    edit(&mut preset);
    let mut e = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), preset);
    e.set_playing(true);
    e
}
fn render(e: &mut Engine, frames: usize) -> Vec<[f32; 2]> {
    (0..frames).map(|_| e.process_frame()).collect()
}
fn rms(frames: &[[f32; 2]]) -> f32 {
    (frames
        .iter()
        .map(|f| f[0] * f[0] + f[1] * f[1])
        .sum::<f32>()
        / frames.len() as f32)
        .sqrt()
}
fn advanced(p: &mut Preset) {
    p.grain_groups.advanced = true;
}
/// The largest sample difference between two renders.
fn error(a: &[[f32; 2]], b: &[[f32; 2]]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x[0] - y[0]).abs().max((x[1] - y[1]).abs()))
        .fold(0.0, f32::max)
}

#[test]
fn presets_without_advanced_mode_serialize_as_before() {
    let json = serde_json::to_value(Preset::default()).unwrap();
    assert!(json.get("grain_groups").is_none());
    let mut preset = Preset::default();
    preset.grain_groups.advanced = true;
    preset.grain_groups.groups[2].pitch = 7.0;
    preset.grain_groups.groups[1].filter_type = FilterType::HighPass;
    let json = serde_json::to_value(&preset).unwrap();
    assert_eq!(
        json["grain_groups"]["groups"][1]["filter_type"],
        "high_pass"
    );
    let back: Preset = serde_json::from_value(json).unwrap();
    assert_eq!(back.grain_groups, preset.grain_groups);
}

#[test]
fn advanced_mode_with_untouched_groups_sounds_like_the_engine() {
    // The default amp envelope draws the engine's window at the group's SHAPE.
    let mut off = engine(|p| p.params.grain_shape = Settings::default().groups[0].shape);
    let mut on = engine(advanced);
    let (a, b) = (render(&mut off, 24_000), render(&mut on, 24_000));
    let difference = error(&a, &b);
    assert!(difference < 1e-4 * rms(&a) / 0.05, "{difference}");
    assert!(rms(&a[12_000..]) > 0.01);
    // Groups only wait while advanced mode is off.
    let mut waiting = engine(|p| {
        p.grain_groups.groups[0].pitch = 12.0;
        p.grain_groups.groups[1].cutoff = 0.2;
    });
    assert_eq!(
        render(&mut engine(|_| {}), 24_000),
        render(&mut waiting, 24_000)
    );
}

#[test]
fn equal_shares_deal_grains_in_turn_and_zero_shares_sit_out() {
    let mut e = engine(advanced);
    render(&mut e, 24_000);
    let mut counts = [0; GROUPS + 1];
    for grain in e.grain_views() {
        counts[grain.group as usize] += 1;
    }
    assert_eq!(counts[0], 0, "every grain joined a group");
    let (low, high) = (counts[1..].iter().min(), counts[1..].iter().max());
    assert!(high.unwrap() - low.unwrap() <= 1, "{counts:?}");

    let mut only_b = engine(|p| {
        advanced(p);
        for (i, g) in p.grain_groups.groups.iter_mut().enumerate() {
            g.share = if i == 1 { 0.4 } else { 0.0 };
        }
    });
    render(&mut only_b, 24_000);
    assert!(only_b.active_grains() > 4);
    assert!(only_b.grain_views().iter().all(|g| g.group == 2));

    let mut none = engine(|p| {
        advanced(p);
        p.grain_groups.groups.iter_mut().for_each(|g| g.share = 0.0);
    });
    let out = render(&mut none, 24_000);
    assert_eq!(none.active_grains(), 0);
    assert!(rms(&out[12_000..]) < 1e-6);
}

#[test]
fn a_groups_pitch_and_shape_play_like_the_engines_own() {
    // +12 semitones on every group, against the engine's PITCH raised 12.
    let mut grouped = engine(|p| {
        advanced(p);
        for g in &mut p.grain_groups.groups {
            g.pitch = 12.0;
            g.shape = 0.2;
        }
    });
    let mut direct = engine(|p| {
        p.params.pitch = 0.5 + 12.0 / (2.0 * p.params.pitch_range);
        p.params.grain_shape = 0.2;
    });
    let (a, b) = (render(&mut grouped, 24_000), render(&mut direct, 24_000));
    let error = error(&a, &b);
    assert!(error < 1e-3, "{error}");
    assert!(rms(&a[12_000..]) > 0.01);
}

#[test]
fn each_group_filters_and_shapes_its_own_grains() {
    let open = rms(&render(&mut engine(advanced), 24_000)[12_000..]);
    let closed = rms(&render(
        &mut engine(|p| {
            advanced(p);
            p.grain_groups
                .groups
                .iter_mut()
                .for_each(|g| g.cutoff = 0.1);
        }),
        24_000,
    )[12_000..]);
    let high = rms(&render(
        &mut engine(|p| {
            advanced(p);
            for g in &mut p.grain_groups.groups {
                g.filter_type = FilterType::HighPass;
                g.cutoff = 0.95;
            }
        }),
        24_000,
    )[12_000..]);
    assert!(closed < open * 0.2, "low pass {closed} vs {open}");
    assert!(high < open * 0.2, "high pass {high} vs {open}");
    // A closed filter opened by its envelope at every grain's start lets
    // some of each grain through.
    let swept = rms(&render(
        &mut engine(|p| {
            advanced(p);
            for g in &mut p.grain_groups.groups {
                g.cutoff = 0.1;
                g.env_depth = 0.9;
            }
        }),
        24_000,
    )[12_000..]);
    assert!(
        swept > closed * 1.5 && swept < open,
        "{closed} < {swept} < {open}"
    );
    // The amp envelope replaces the window: held flat, the grains play
    // louder than through the window; a short pluck plays quieter.
    let with_amp = |amp: Envelope| {
        rms(&render(
            &mut engine(move |p| {
                advanced(p);
                p.grain_groups.groups.iter_mut().for_each(|g| g.amp = amp);
            }),
            24_000,
        )[12_000..])
    };
    let held = with_amp(Envelope {
        attack: 0.0,
        decay: 0.0,
        sustain: 1.0,
        release: 0.0,
    });
    let pluck = with_amp(Envelope {
        attack: 0.0,
        decay: 0.25,
        sustain: 0.0,
        release: 0.0,
    });
    assert!(held > open * 1.2, "{held} vs {open}");
    assert!(
        pluck < open * 0.8 && pluck > open * 0.1,
        "{pluck} vs {open}"
    );
    let half = rms(&render(
        &mut engine(|p| {
            advanced(p);
            p.grain_groups.groups.iter_mut().for_each(|g| g.level = 0.5);
        }),
        24_000,
    )[12_000..]);
    assert!((half / open - 0.5).abs() < 0.05, "{half} vs {open}");
}

#[test]
fn drum_and_synth_tracks_play_grain_groups_too() {
    for kind in [Kind::Drums, Kind::Synth] {
        let mut preset = kind.preset().unwrap();
        if let Some(d) = &mut preset.drums {
            d.granular = true;
        }
        if let Some(s) = &mut preset.synth {
            s.granular = true;
        }
        preset.params.grains = 0.8;
        preset.grain_groups.advanced = true;
        for (i, g) in preset.grain_groups.groups.iter_mut().enumerate() {
            g.share = if i == 3 { 1.0 } else { 0.0 };
        }
        let mut e = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), preset);
        e.set_playing(true);
        e.midi(sad_pet::midi::Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 100,
        });
        render(&mut e, 24_000);
        let views = e.grain_views();
        assert!(!views.is_empty(), "{kind:?} plays grains");
        assert!(views.iter().all(|g| g.group == 4), "{kind:?}");
    }
}

#[test]
fn grain_group_controls_append_after_the_macros() {
    let specs = parameters::specs();
    let macros = specs.iter().position(|s| s.id == "macro8").unwrap();
    assert_eq!(specs[macros + 1].id, "grain_advanced");
    assert_eq!(specs[macros + 2].id, "grain_group1_share");
    assert_eq!(specs[macros + 1 + 16].id, "grain_group1_filter_release");
    assert_eq!(specs[macros + 1 + 16 * 4].id, "grain_group4_filter_release");
    // Each group's output follows every group's other controls.
    let outputs = macros + 2 + 16 * 4;
    let ids: Vec<_> = specs[outputs..].iter().map(|s| s.id.as_str()).collect();
    assert_eq!(
        ids[..3],
        [
            "grain_group1_chain",
            "grain_group1_send",
            "grain_group1_bus"
        ]
    );
    assert_eq!(ids.len(), 4 * 3);
    let mut preset = Preset::default();
    let changes = parameters::parse(&serde_json::json!({
        "grain_advanced": 1.0,
        "grain_group2_cutoff": 0.3,
        "grain_group3_pitch": 99.0,
        "grain_group4_filter_type": 2.0,
    }))
    .unwrap();
    assert!(parameters::apply(&mut preset, &changes));
    let g = &preset.grain_groups;
    assert!(g.advanced);
    assert_eq!(g.groups[1].cutoff, 0.3);
    assert_eq!(g.groups[2].pitch, 24.0);
    assert_eq!(g.groups[3].filter_type, FilterType::HighPass);
    // LFOs and the matrix reach every group control.
    let catalog = sad_pet::sequence::catalog();
    let modulation = catalog["modulation"].as_array().unwrap();
    assert!(modulation.iter().any(|s| s["id"] == "grain_group1_cutoff"));
    assert!(
        modulation
            .iter()
            .any(|s| s["id"] == "grain_group4_amp_attack")
    );
    assert!(modulation.iter().any(|s| s["id"] == "grain_group2_send"));
    assert!(!modulation.iter().any(|s| s["id"] == "grain_group2_bus"));
}

/// Only group B plays, sending to Bus 2.
fn layer_to_bus(chain: f32, send: f32, advanced_mode: bool) -> Engine {
    let mut e = engine(|p| {
        p.grain_groups.advanced = advanced_mode;
        for (i, g) in p.grain_groups.groups.iter_mut().enumerate() {
            g.share = if i == 1 { 1.0 } else { 0.0 };
        }
        p.grain_groups.groups[1].output = sad_pet::taps::Tap {
            chain,
            send,
            bus: 2,
        };
    });
    e.set_open_buses(1 << 2 | 1 << 3);
    e
}
fn render_with_sends(e: &mut Engine, frames: usize) -> (Vec<[f32; 2]>, Vec<[[f32; 2]; 8]>) {
    (0..frames)
        .map(|_| {
            let out = e.process_frame();
            (out, e.bus_sends().copied().unwrap_or([[0.0; 2]; 8]))
        })
        .unzip()
}

#[test]
fn a_group_sends_its_grains_to_its_own_bus() {
    let (kept, _) = render_with_sends(&mut layer_to_bus(1.0, 0.0, true), 24_000);
    let mut sending = layer_to_bus(1.0, 1.0, true);
    assert_eq!(sending.tap_buses(), 1 << 1, "routed to Bus 2 only");
    let (out, sends) = render_with_sends(&mut sending, 24_000);
    let bus = |n: usize| -> Vec<[f32; 2]> { sends[12_000..].iter().map(|s| s[n - 1]).collect() };
    let heard = rms(&bus(2));
    assert!(heard > 0.01, "{heard}");
    assert_eq!(rms(&bus(3)), 0.0);
    // SEND leaves the chain as it was.
    assert_eq!(out, kept);
    // At CHAIN 0 the group leaves the timbre and only the bus hears it.
    let (only_bus, sends) = render_with_sends(&mut layer_to_bus(0.0, 1.0, true), 24_000);
    assert!(rms(&only_bus[12_000..]) < 1e-6);
    let sent: Vec<_> = sends[12_000..].iter().map(|s| s[1]).collect();
    assert!((rms(&sent) - heard).abs() < heard * 1e-3);
    // Outside advanced mode the groups send nothing and keep the chain.
    let mut off = layer_to_bus(0.0, 1.0, false);
    assert_eq!(off.tap_buses(), 0);
    let (out, sends) = render_with_sends(&mut off, 24_000);
    assert!(rms(&out[12_000..]) > 0.01);
    assert!(sends.iter().all(|s| *s == [[0.0; 2]; 8]));
}

#[test]
fn an_lfo_reaches_a_groups_filter() {
    let index = parameters::specs()
        .iter()
        .position(|s| s.id == "grain_group1_cutoff")
        .unwrap() as u16;
    let closed = |depth: f32| {
        engine(move |p| {
            advanced(p);
            p.grain_groups.groups.iter_mut().for_each(|g| {
                g.cutoff = 0.1;
                g.share = 0.0;
            });
            p.grain_groups.groups[0].share = 1.0;
            p.lfos[0].rate_hz = 3.0;
            p.lfos[0].destination = sad_pet::preset::Destination::Control { index };
            p.lfos[0].depth = depth;
        })
    };
    let still = rms(&render(&mut closed(0.0), 48_000)[12_000..]);
    let swept = rms(&render(&mut closed(0.8), 48_000)[12_000..]);
    assert!(swept > still * 2.0, "{still} → {swept}");
}
