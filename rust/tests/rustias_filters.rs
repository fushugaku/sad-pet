use sad_pet::{
    dsp::{
        Engine, SampleBuffer,
        filter::{FilterBank, model_cutoff_hz, voice_cutoffs},
    },
    preset::{FilterModel as M, FilterRouting, FilterType, Params, Preset},
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::Arc,
};

struct Allocator;
thread_local! {static TRACK:Cell<bool>=const{Cell::new(false)};static COUNT:Cell<usize>=const{Cell::new(0)};}
fn count() {
    let _ = TRACK.try_with(|t| {
        if t.get() {
            let _ = COUNT.try_with(|n| n.set(n.get() + 1));
        }
    });
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        count();
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        count();
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        count();
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
fn no_heap(f: impl FnOnce()) {
    COUNT.with(|n| n.set(0));
    TRACK.with(|t| t.set(true));
    f();
    TRACK.with(|t| t.set(false));
    assert_eq!(COUNT.with(Cell::get), 0, "audio allocated or freed");
}
fn params(model: M) -> Params {
    Params {
        filter_model: model,
        filter2_model: model,
        filter: 0.55,
        filter2: 0.55,
        filter_resonance: 0.3,
        filter2_resonance: 0.3,
        filter_env_depth: 0.0,
        filter2_env_depth: 0.0,
        ..Params::default()
    }
}
fn rms(model: M, kind: FilterType, morph: f32, hz: f32) -> f32 {
    let mut bank = FilterBank::new(48000);
    let p = Params {
        vcf_type: kind,
        filter_morph: morph,
        filter_resonance: 0.0,
        ..params(model)
    };
    bank.configure(&p, 60.0, [0.0; 3]);
    let mut energy = 0.0;
    for i in 0..12000 {
        let input = (i as f32 * std::f32::consts::TAU * hz / 48000.0).sin() * 0.04;
        let output = bank.process([input, 0.0]);
        assert_eq!(output[1], 0.0, "stereo crosstalk: {model:?}");
        if i >= 6000 {
            energy += output[0] * output[0];
        }
    }
    (energy / 6000.0).sqrt()
}

#[test]
fn every_model_is_stereo_independent_and_has_low_high_band_or_notch_response() {
    for model in [
        M::Radias,
        M::LadderLp24,
        M::Ms20Lp12,
        M::ProLp24,
        M::MPolyLp24,
        M::LinearLp12,
        M::LinearLp24,
        M::Radias2,
    ] {
        assert!(
            rms(model, FilterType::LowPass, 0.0, 80.0)
                > rms(model, FilterType::LowPass, 0.0, 4000.0) * 8.0,
            "LP: {model:?}"
        );
    }
    for model in [M::Ms20Hp12, M::LadderHp24, M::LinearHp12, M::LinearHp24] {
        assert!(
            rms(model, FilterType::LowPass, 0.0, 4000.0)
                > rms(model, FilterType::LowPass, 0.0, 40.0) * 8.0,
            "HP: {model:?}"
        );
    }
    for model in [M::LadderBp24, M::LinearBp12, M::LinearBp24] {
        assert!(
            rms(model, FilterType::LowPass, 0.0, 300.0)
                > rms(model, FilterType::LowPass, 0.0, 30.0) * 2.0,
            "BP: {model:?}"
        );
    }
    for model in [M::LinearNotch12, M::LinearNotch24] {
        assert!(
            rms(model, FilterType::LowPass, 0.0, 30.0)
                > rms(model, FilterType::LowPass, 0.0, 310.0) * 2.0,
            "Notch: {model:?}"
        );
    }
    for model in [M::Radias, M::Sem, M::MultiFilter] {
        let hp = if model == M::Radias {
            96.0 / 127.0
        } else if model == M::Sem {
            1.0
        } else {
            2.0 / 3.0
        };
        // Original RADIAS LP24 and HP have different insertion gains; compare
        // their spectral slopes without assuming equal loudness.
        assert!(
            rms(model, FilterType::LowPass, hp, 4000.0)
                > rms(model, FilterType::LowPass, hp, 80.0) * 8.0,
            "Morph HP: {model:?}"
        );
        assert!(
            rms(model, FilterType::LowPass, 0.0, 80.0)
                > rms(model, FilterType::LowPass, 0.0, 4000.0) * 8.0,
            "Morph LP: {model:?}"
        );
    }
}

#[test]
fn all_models_rates_routings_and_live_extremes_are_finite_without_crosstalk() {
    for rate in [8000, 44100, 48000, 96000, 192000] {
        let mut bank = FilterBank::new(rate);
        for (i, model) in M::ALL.into_iter().enumerate() {
            let mut p = params(model);
            p.filter2_model = M::ALL[(i + 7) % M::ALL.len()];
            p.vcf_type = FilterType::Comb;
            p.vcf2_type = FilterType::Comb;
            p.filter_resonance = 1.0;
            p.filter2_resonance = 1.0;
            p.filter_drive = 1.0;
            p.filter2_drive = 1.0;
            for routing in [
                FilterRouting::Single,
                FilterRouting::Serial,
                FilterRouting::Parallel,
            ] {
                p.filter_routing = routing;
                for cutoff in [0.0, 0.5, 1.0] {
                    p.filter = cutoff;
                    p.filter2 = 1.0 - cutoff;
                    p.filter_morph = cutoff;
                    p.filter2_morph = 1.0 - cutoff;
                    bank.configure(&p, 60.0, [0.0; 3]);
                    for f in 0..1024 {
                        let x = if f % 37 == 0 { 3.0 } else { 0.0 };
                        let y = bank.process([x, 0.0]);
                        assert!(
                            y.into_iter().all(|v| v.is_finite() && v.abs() <= 4.0),
                            "{rate}/{model:?}/{routing:?}: {y:?}"
                        );
                        assert_eq!(y[1], 0.0);
                    }
                }
            }
        }
    }
}

#[test]
fn both_slots_track_notes_envelopes_and_pitch_bend_on_each_models_frequency_scale() {
    for model in M::ALL {
        let mut p = params(model);
        p.filter_keytrack = 1.0;
        p.filter2_keytrack = 0.5;
        let base = voice_cutoffs(&p, 60.0, [0.0; 3]);
        let octave = voice_cutoffs(&p, 72.0, [0.0; 3]);
        assert!(
            (model_cutoff_hz(model, p.vcf_type, octave[0])
                / model_cutoff_hz(model, p.vcf_type, base[0])
                - 2.0)
                .abs()
                < 1e-5
        );
        assert!(
            (model_cutoff_hz(model, p.vcf2_type, octave[1])
                / model_cutoff_hz(model, p.vcf2_type, base[1])
                - 2.0_f32.sqrt())
            .abs()
                < 1e-5
        );
        p.filter_env_depth = 0.3;
        p.filter2_env_depth = -0.3;
        let values = voice_cutoffs(&p, 60.0, [0.5; 3]);
        assert!((values[0] - base[0] - 0.15).abs() < 1e-6);
        assert!((values[1] - base[1] + 0.15).abs() < 1e-6);
    }
}

#[test]
fn live_drive_and_morph_edits_preserve_filter_history_and_repeated_configuration_is_exact() {
    for model in M::ALL.into_iter().skip(1) {
        let mut bank = FilterBank::new(48000);
        let mut repeated = FilterBank::new(48000);
        let mut fresh = FilterBank::new(48000);
        let mut p = params(model);
        p.vcf_type = FilterType::Comb;
        p.filter = 0.1;
        p.filter_resonance = 0.9;
        bank.configure(&p, 60.0, [0.0; 3]);
        repeated.configure(&p, 60.0, [0.0; 3]);
        for i in 0..100 {
            let x = if i == 0 { 0.7 } else { 0.0 };
            repeated.configure(&p, 60.0, [0.0; 3]);
            assert_eq!(bank.process([x; 2]), repeated.process([x; 2]));
        }
        p.filter_drive = 0.4;
        p.filter_morph = 0.3;
        bank.configure(&p, 60.0, [0.0; 3]);
        fresh.configure(&p, 60.0, [0.0; 3]);
        let mut difference = 0.0;
        for _ in 0..2048 {
            let a = bank.process([0.0; 2]);
            let b = fresh.process([0.0; 2]);
            difference += (a[0] - b[0]).abs();
        }
        assert!(difference > 1e-8, "lost history: {model:?}");
    }
}

#[test]
fn presets_and_host_ids_remain_compatible_and_new_controls_are_realtime() {
    let old: Preset =
        serde_json::from_value(serde_json::json!({"params":{"vcf_type":"high_pass","filter":0.4}}))
            .unwrap();
    assert_eq!(old.params.filter_model, M::Legacy);
    assert_eq!(old.params.filter2_model, M::Legacy);
    let specs = sad_pet::parameters::specs();
    let start = specs.iter().position(|s| s.id == "filter_drive").unwrap();
    assert_eq!(specs[start - 1].id, "resynth_osc4_cross_input");
    assert_eq!(specs[9].id, "filter");
    assert_eq!(specs[start + 7].id, "vcf2_type");
    assert_eq!(specs[start + 4].max, 19.0);
    let catalog = sad_pet::sequence::catalog();
    let catalog = serde_json::to_string(&catalog).unwrap();
    assert!(catalog.contains("filter2_morph"));
    let mut p = Preset::default();
    p.color_chain.clear();
    let changes: Vec<_> = (0..20)
        .map(|i| {
            sad_pet::parameters::parse(&serde_json::json!({
                "filter_model":i,"filter2_model":19-i,"vcf_type":3,"vcf2_type":3,
                "filter_drive":0.6,"filter2_drive":0.3,"filter_morph":0.4,"filter2_morph":0.8,
                "filter":0.5,"filter_resonance":0.8
            }))
            .unwrap()
        })
        .collect();
    let mut e = Engine::new(48000, Arc::new(SampleBuffer::demo(48000)), p);
    e.set_playing(true);
    for _ in 0..64 {
        e.process_frame();
    }
    no_heap(|| {
        for changes in &changes {
            e.apply_parameters(changes);
            for _ in 0..2048 {
                assert!(e.process_frame().into_iter().all(f32::is_finite));
            }
        }
    });
    let restored: Preset = serde_json::from_slice(&serde_json::to_vec(&e.preset).unwrap()).unwrap();
    assert_eq!(restored.params, e.preset.params);
    assert_eq!(restored.params.filter_model, M::Radias2);
}

#[test]
fn editor_responses_follow_each_models_shape() {
    use sad_pet::dsp::filter::{RESPONSE_POINTS, response, response_request};
    // dB at 50 Hz and 5 kHz, and the deepest point. Rustias models lose some
    // top end to their oversampling, so the 20 kHz end is not a pass band.
    let ends = |model, kind, cutoff, resonance, morph| {
        let db = response(model, kind, cutoff, resonance, morph);
        assert_eq!(db.len(), RESPONSE_POINTS);
        assert!(db.iter().all(|v| v.is_finite()), "{model:?} {kind:?}");
        let at =
            |hz: f64| db[((hz / 20.0).log(1000.0) * (RESPONSE_POINTS - 1) as f64).round() as usize];
        (
            at(50.0),
            at(5000.0),
            db.iter().copied().fold(f32::MAX, f32::min),
        )
    };
    for model in [
        M::LadderLp24,
        M::Ms20Lp12,
        M::ProLp24,
        M::MPolyLp24,
        M::LinearLp12,
        M::LinearLp24,
    ] {
        let (low, high, _) = ends(model, FilterType::LowPass, 0.4, 0.1, 0.0);
        assert!(
            low > -6.0 && high < low - 24.0,
            "{model:?} low-pass: {low} / {high} dB"
        );
    }
    for model in [M::Ms20Hp12, M::LadderHp24, M::LinearHp12, M::LinearHp24] {
        let (low, high, _) = ends(model, FilterType::LowPass, 0.6, 0.0, 0.0);
        assert!(
            high > -6.0 && low < high - 24.0,
            "{model:?} high-pass: {low} / {high} dB"
        );
    }
    let (low, high, deepest) = ends(M::LinearNotch24, FilterType::LowPass, 0.5, 0.0, 0.0);
    assert!(
        deepest < low.min(high) - 20.0,
        "Notch: {low} / {deepest} / {high} dB"
    );
    for kind in [FilterType::LowPass, FilterType::HighPass] {
        let (low, high, _) = ends(M::Legacy, kind, 0.5, 0.0, 0.0);
        let (pass, stop) = if kind == FilterType::LowPass {
            (low, high)
        } else {
            (high, low)
        };
        assert!(
            pass > -3.0 && stop < -24.0,
            "SAD PET {kind:?}: {low} / {high} dB"
        );
    }
    // Comb peaks repeat across the spectrum, so its curve swings widely.
    let comb = response(M::Legacy, FilterType::Comb, 0.3, 1.0, 0.0);
    let swing = comb.iter().copied().fold(f32::MIN, f32::max)
        - comb.iter().copied().fold(f32::MAX, f32::min);
    assert!(swing > 12.0, "Comb swing {swing} dB");
    // Resonance lifts a peak above the passband.
    let flat = response(M::Legacy, FilterType::LowPass, 0.5, 0.0, 0.0);
    let resonant = response(M::Legacy, FilterType::LowPass, 0.5, 0.9, 0.0);
    let peak = |db: &[f32]| db.iter().copied().fold(f32::MIN, f32::max);
    assert!(peak(&resonant) > peak(&flat) + 6.0);

    let reply = response_request(&serde_json::json!({
        "model": "radias2", "kind": "band_pass", "cutoffs": [0.3, 0.6], "resonance": 0.5, "morph": 0
    }))
    .unwrap();
    assert_eq!(reply["curves"].as_array().unwrap().len(), 2);
    for bad in [
        serde_json::json!({"model": "nope", "kind": "low_pass", "cutoffs": [0.5], "resonance": 0, "morph": 0}),
        serde_json::json!({"model": "sem", "kind": "low_pass", "cutoffs": [], "resonance": 0, "morph": 0}),
        serde_json::json!({"model": "sem", "kind": "low_pass", "cutoffs": [0.5], "resonance": "x", "morph": 0}),
    ] {
        assert!(response_request(&bad).is_err());
    }
}
