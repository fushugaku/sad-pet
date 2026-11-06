use sad_pet::{
    color::{ColorParameter, ColorSlot, WaveshaperMode},
    dsp::filter::FilterBank,
    parameters,
    preset::{FilterRouting, FilterSlot, FilterType, Params, Preset},
    waveshaper::{FilterShapers, Options, Quality, Stage},
};

fn clipped() -> Stage {
    Stage {
        enabled: true,
        mode: WaveshaperMode::HardClip,
        depth: 0.8,
        ..Stage::default()
    }
}

fn render(params: Params) -> Vec<[f32; 2]> {
    let mut filters = FilterBank::new(48_000);
    filters.configure(&params, 60.0, [0.0; 8]);
    (0..4096)
        .map(|i| {
            let x = (std::f32::consts::TAU * i as f32 * 1733.0 / 48_000.0).sin() * 0.8;
            filters.process([x, x * 0.7])
        })
        .collect()
}

#[test]
fn older_presets_load_with_unmodified_filter_order_and_disabled_shapers() {
    let mut document = serde_json::to_value(Preset::default()).unwrap();
    document["params"]
        .as_object_mut()
        .unwrap()
        .remove("filter_order");
    document["params"]
        .as_object_mut()
        .unwrap()
        .remove("filter_shapers");
    document["grain_groups"] =
        serde_json::to_value(sad_pet::grain_groups::Settings::default()).unwrap();
    document["grain_groups"]["groups"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .for_each(|group| {
            group.as_object_mut().unwrap().remove("waveshaper_options");
        });
    let preset: Preset = serde_json::from_value(document).unwrap();
    assert_eq!(preset.params.filter_order, [0, 1]);
    assert_eq!(preset.params.filter_shapers, [FilterShapers::default(); 2]);
    assert_eq!(
        preset.grain_groups.groups[0].waveshaper_options,
        Options::default()
    );
    let slot: FilterSlot = serde_json::from_value(serde_json::json!({ "id": 4 })).unwrap();
    assert_eq!(slot.shapers, FilterShapers::default());
}

#[test]
fn shaper_before_and_after_filter_are_different_audio_routes() {
    let mut before = Params {
        filter: 0.25,
        filter_env_depth: 0.0,
        ..Params::default()
    };
    before.filter_shapers[0].before = clipped();
    let mut after = before;
    after.filter_shapers[0] = FilterShapers {
        after: clipped(),
        ..FilterShapers::default()
    };
    let a = render(before);
    let b = render(after);
    let error: f32 = a[1024..]
        .iter()
        .zip(&b[1024..])
        .map(|(a, b)| (a[0] - b[0]).powi(2))
        .sum();
    assert!(
        error > 0.01,
        "WS placement must affect the actual audio, error={error}"
    );
}

#[test]
fn single_filter_skips_second_filter_and_both_its_shapers() {
    let params = Params {
        filter: 0.45,
        filter_env_depth: 0.0,
        ..Params::default()
    };
    let mut second = params;
    second.filter_shapers[1] = FilterShapers {
        before: clipped(),
        after: clipped(),
    };
    second.filter_order = [1, 0];
    assert_eq!(render(params), render(second));
}

#[test]
fn serial_reordering_moves_filter_identity_with_its_shapers() {
    let mut params = Params {
        filter: 0.4,
        filter_env_depth: 0.0,
        filter2: 0.3,
        vcf2_type: FilterType::HighPass,
        filter2_env_depth: 0.0,
        filter_routing: FilterRouting::Serial,
        ..Params::default()
    };
    params.filter_shapers[0].after = clipped();
    let original = render(params);
    params.filter_order = [1, 0];
    let swapped = render(params);
    assert!(
        original[1024..]
            .iter()
            .zip(&swapped[1024..])
            .any(|(a, b)| (a[0] - b[0]).abs() > 0.001)
    );
    assert_eq!(params.filter_shapers[0].after, clipped());
    assert!(!params.filter_shapers[1].after.enabled);
}

#[test]
fn extended_numeric_controls_update_filter_grain_and_fx_without_replacing_settings() {
    let mut preset = Preset::default();
    preset.filter_slots.push(FilterSlot {
        id: 77,
        ..FilterSlot::default()
    });
    preset.color_chain = vec![ColorSlot {
        id: 11,
        ..ColorSlot::default()
    }];
    preset.color_chain[0].settings.waveshaper_options.quality = Quality::Clean4x;
    let changes = parameters::parse(&serde_json::json!({
        "filter1_ws_before_depth": 0.7,
        "filter3_ws_after_input": 0.8,
        "grain_group1_waveshaper_morph": 0.4,
        "fx1_waveshaper_output": 0.6,
    }))
    .unwrap();
    assert!(parameters::apply(&mut preset, &changes));
    assert_eq!(preset.params.filter_shapers[0].before.depth, 0.7);
    assert_eq!(preset.filter_slots[0].shapers.after.options.input, 0.8);
    assert_eq!(preset.filter_slots[0].id, 77);
    assert_eq!(preset.grain_groups.groups[0].waveshaper_options.morph, 0.4);
    assert_eq!(
        preset.color_chain[0].settings.waveshaper_options.output,
        0.6
    );
    assert_eq!(
        preset.color_chain[0].settings.waveshaper_options.quality,
        Quality::Clean4x
    );
    let keys: std::collections::HashSet<_> = ColorParameter::ALL.iter().map(|p| p.key()).collect();
    assert_eq!(keys.len(), ColorParameter::ALL.len());
}

#[test]
fn normalization_repairs_invalid_order_and_custom_curve_values() {
    let mut params = Params {
        filter_order: [9, 9],
        ..Params::default()
    };
    params.filter_shapers[0].before.options.points[5] = f32::NAN;
    params.filter_shapers[0].after.depth = f32::INFINITY;
    params.sanitize();
    assert_eq!(params.filter_order, [0, 1]);
    assert!(
        params.filter_shapers[0]
            .before
            .options
            .points
            .iter()
            .all(|x| x.is_finite() && x.abs() <= 1.0)
    );
    assert!(params.filter_shapers[0].after.depth.is_finite());
}

#[test]
fn parallel_branches_align_different_clean_shaper_latencies() {
    let mut params = Params {
        filter_routing: FilterRouting::Parallel,
        filter_env_depth: 0.0,
        filter2_env_depth: 0.0,
        ..Params::default()
    };
    let mut stage = Stage {
        enabled: true,
        mix: 0.0,
        ..Stage::default()
    };
    stage.options.quality = Quality::Clean2x;
    params.filter_shapers[0] = FilterShapers {
        before: stage,
        after: stage,
    };
    stage.options.quality = Quality::Clean4x;
    params.filter_shapers[1].after = stage;
    let mut bank = FilterBank::new(48_000);
    bank.configure(&params, 60.0, [0.0; 8]);
    let mut output = Vec::new();
    for i in 0..160 {
        output.push(bank.process(if i == 0 { [0.8, -0.5] } else { [0.0; 2] }));
    }
    for (i, frame) in output.into_iter().enumerate() {
        assert_eq!(
            frame,
            if i == 64 { [0.8, -0.5] } else { [0.0; 2] },
            "parallel impulse at {i}"
        );
    }
}
