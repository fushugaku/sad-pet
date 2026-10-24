use sad_pet::{
    color::{ColorParameter, ColorSlot},
    dsp::{Engine, SampleBuffer, color::ColorProcessor},
    imported_fx::Core,
    parameters,
    preset::{Preset, SaturatorType as Kind},
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::Arc,
};
struct Allocator;
thread_local! { static TRACK: Cell<bool> = const { Cell::new(false) }; static COUNT: Cell<usize> = const { Cell::new(0) }; }
fn count() {
    let _ = TRACK.try_with(|t| {
        if t.get() {
            let _ = COUNT.try_with(|c| c.set(c.get() + 1));
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
    COUNT.with(|c| c.set(0));
    TRACK.with(|c| c.set(true));
    f();
    TRACK.with(|c| c.set(false));
    assert_eq!(
        COUNT.with(Cell::get),
        0,
        "audio allocated or destroyed storage"
    );
}
fn slot(kind: Kind, id: u32) -> ColorSlot {
    ColorSlot {
        kind,
        id,
        mix: 1.0,
        ..Default::default()
    }
}
fn processor(rate: u32, s: ColorSlot) -> ColorProcessor {
    let mut p = ColorProcessor::new(rate);
    p.prepare_imported(s);
    p.configure_tempo(s, 123.0);
    p
}
fn q31(data: &[u8]) -> Vec<[f32; 2]> {
    data.as_chunks::<8>()
        .0
        .iter()
        .map(|f| {
            std::array::from_fn(|c| {
                i32::from_le_bytes(f[c * 4..c * 4 + 4].try_into().unwrap()) as f32 / 2147483648.0
            })
        })
        .collect()
}
#[test]
fn super_echo_matches_the_unchanged_arm_firmware_in_tape_and_bbd_modes() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/erika-fx/manifest.json")).unwrap();
    let input = q31(include_bytes!("fixtures/erika-fx/super-echo-input.q31"));
    for (case, bytes) in manifest["super_echo"].as_array().unwrap().iter().zip([
        include_bytes!("fixtures/erika-fx/super-echo-tape.q31").as_slice(),
        include_bytes!("fixtures/erika-fx/super-echo-bbd.q31").as_slice(),
    ]) {
        let expected = q31(bytes);
        let mut s = slot(Kind::SuperEcho, 37);
        for (id, value) in case["values"].as_array().unwrap().iter().enumerate() {
            if id == 18 {
                continue;
            }
            let key = format!(
                "super_echo_{}",
                sad_pet_superecho::parameters::PARAMETERS[id].key
            );
            let parameter = *ColorParameter::ALL.iter().find(|p| p.key() == key).unwrap();
            *s.parameter_mut(parameter) = value.as_f64().unwrap() as f32;
        }
        let mut p = processor(48000, s);
        let mut max_error = 0.0_f32;
        let mut energy = 0.0;
        for i in 0..input.len() + 23 {
            let actual = p.process(input.get(i).copied().unwrap_or([0.0; 2]));
            if i >= 23 {
                for c in 0..2 {
                    max_error = max_error.max((actual[c] - expected[i - 23][c]).abs());
                    energy += actual[c] * actual[c];
                }
            }
        }
        assert!(energy > 0.001, "silent reference");
        assert!(
            max_error < 2e-6,
            "{} differs from original ARM by {max_error}",
            case["name"]
        );
    }
}

#[test]
fn super_echo_memory_adapter_keeps_checked_boundaries_and_fault_locations() {
    let mut machine = sad_pet_superecho::machine::Machine::new();
    machine.pc = 0x208;
    for (start, end) in [
        (0, 0x3500),
        (0x20000000, 0x20020000),
        (0x20200000, 0x20280000),
        (0x80000000, 0x82000000),
    ] {
        for size in [1, 2, 4, 8] {
            // Both unaligned accesses and the last valid span remain supported.
            for address in [start + 1, end - size as u32] {
                let value = 0x8877665544332211 & (u64::MAX >> (64 - size * 8));
                machine.store(address, size, value).unwrap();
                assert_eq!(machine.load(address, size).unwrap(), value);
            }
        }
        let read = machine.load(end - 1, 4).unwrap_err();
        let write = machine.store(end - 1, 4, 0).unwrap_err();
        assert_eq!(read.pc, 0x208);
        assert_eq!(write.pc, 0x208);
        assert_eq!(
            read.message,
            format!("read crosses region at 0x{:08x}", end - 1)
        );
        assert_eq!(
            write.message,
            format!("write crosses region at 0x{:08x}", end - 1)
        );
    }
    let flash = sad_pet_superecho::machine::FLASH_BASE;
    assert!(machine.load(flash, 4).is_ok());
    assert!(machine.store(flash, 4, 0).is_err());
    let fault = machine.load(0x12340000, 4).unwrap_err();
    assert_eq!(fault.pc, 0x208);
    assert_eq!(fault.message, "read unmapped address 0x12340000 (4 bytes)");
}
#[test]
fn alpha_verb_matches_the_upstream_original_firmware_reference() {
    let input = q31(include_bytes!("fixtures/erika-fx/alpha-verb-input.q31"));
    let expected = q31(include_bytes!("fixtures/erika-fx/alpha-verb-output.q31"));
    let mut p = processor(48000, slot(Kind::AlphaVerb, 2));
    for _ in 0..48000 {
        p.process([0.0; 2]);
    }
    let mut max_error = 0.0_f32;
    let mut energy = 0.0;
    for i in 0..input.len() + 23 {
        let actual = p.process(input.get(i).copied().unwrap_or([0.0; 2]));
        if i >= 23 {
            for c in 0..2 {
                max_error = max_error.max((actual[c] - expected[i - 23][c]).abs());
                energy += actual[c] * actual[c];
            }
        }
    }
    assert!(energy > 0.001);
    assert!(
        max_error < 5e-5,
        "AlphaVerb original reference delta {max_error}"
    );
}
#[test]
fn all_rates_extremes_controls_and_resets_are_finite_without_audio_allocation() {
    let _ = parameters::specs();
    for rate in [8000, 44100, 48000, 96000, 192000, 384000] {
        for kind in [Kind::SuperEcho, Kind::AlphaVerb] {
            let mut s = slot(kind, 1);
            let mut p = processor(rate, s);
            no_heap(|| {
                for value in [0.0, 0.37, 1.0] {
                    for &control in s.control_parameters() {
                        *s.parameter_mut(control) = value;
                    }
                    p.configure_tempo(s, 137.0);
                    for i in 0..384 {
                        assert!(
                            p.process([(i as f32 * 0.17).sin() * 0.1, 0.05])
                                .iter()
                                .all(|x| x.is_finite() && x.abs() < 2.0)
                        );
                    }
                }
                p.clear();
                p.configure_tempo(s, 95.0);
                for _ in 0..128 {
                    assert!(p.process([0.0; 2]).iter().all(|x| x.is_finite()));
                }
            });
        }
    }
}
fn engine(p: Preset) -> Engine {
    let sample = Arc::new(SampleBuffer::new("silence".into(), 48000, vec![[0.0; 2]; 128]).unwrap());
    let mut e = Engine::new(48000, sample, p);
    e.set_live_input(true, true);
    e.set_playing(true);
    e
}
#[test]
fn numeric_changes_and_prepared_preset_exchange_retain_running_tails_and_do_not_allocate() {
    for kind in [Kind::SuperEcho, Kind::AlphaVerb] {
        let s = slot(kind, 91);
        let preset = Preset {
            color_chain: vec![s],
            pre_color_chain: vec![],
            ..Default::default()
        };
        let mut a = engine(preset.clone());
        let mut b = engine(preset.clone());
        let mut next = preset.prepared_clone();
        next.name = "Renamed".into();
        let key = if kind == Kind::SuperEcho {
            "fx1_super_echo_feedback"
        } else {
            "fx1_alpha_verb_size"
        };
        let changes = parameters::parse(&serde_json::json!({key:0.73})).unwrap();
        for i in 0..2048 {
            let input = [(i as f32 * 0.07).sin() * 0.1, 0.05];
            assert_eq!(a.process_input_frame(input), b.process_input_frame(input));
        }
        no_heap(|| a.exchange_preset(&mut next));
        for i in 0..1024 {
            let input = [(i as f32 * 0.07).sin() * 0.1, 0.05];
            assert_eq!(
                a.process_input_frame(input),
                b.process_input_frame(input),
                "tail reset at {i}"
            );
        }
        no_heap(|| {
            a.apply_parameters(&changes);
            for _ in 0..128 {
                a.process_input_frame([0.1; 2]);
            }
        });
        // Deletion retires the large instance inside the returned preset.
        let mut removed = Preset {
            color_chain: vec![],
            ..Default::default()
        }
        .prepared_clone();
        no_heap(|| {
            a.exchange_preset(&mut removed);
            a.process_input_frame([0.1; 2]);
        });
    }
}
#[test]
fn all_three_racks_roundtrip_controls_and_released_parameter_positions_are_stable() {
    let mut p = Preset {
        color_chain: vec![slot(Kind::SuperEcho, 1)],
        pre_color_chain: vec![slot(Kind::AlphaVerb, 2)],
        source_color_chain: vec![slot(Kind::SuperEcho, 3)],
        ..Default::default()
    };
    let changes = parameters::parse(&serde_json::json!({"fx1_super_echo_feedback":0.63,"pre_fx1_alpha_verb_size":0.72,"source_fx1_super_echo_sync":1})).unwrap();
    parameters::apply(&mut p, &changes);
    p.sanitize();
    let data = serde_json::to_value(&p).unwrap();
    assert!(data.get("imported_effects").is_none());
    let restored: Preset = serde_json::from_value(data).unwrap();
    assert_eq!(p.color_chain, restored.color_chain);
    assert_eq!(p.pre_color_chain, restored.pre_color_chain);
    assert_eq!(p.source_color_chain, restored.source_color_chain);
    let specs = parameters::specs();
    assert_eq!(specs[1475].id, "sample_root_note");
    assert_eq!(specs[1536].id, "resynth_mix");
    let last = specs.iter().position(|s| s.id == "vcf2_type").unwrap();
    assert_eq!(last, 1611);
    assert_eq!(specs[last + 1].id, "fx1_super_echo_dirty");
    for rack in ["fx1_", "pre_fx1_", "source_fx1_"] {
        for c in &ColorParameter::ALL[ColorParameter::LEGACY_COUNT..] {
            assert!(specs.iter().any(|s| s.id == format!("{rack}{}", c.key())));
        }
    }
}
#[test]
fn only_selected_slots_allocate_original_engines() {
    assert!(Core::new(Kind::Distortion, 48_000).is_none());
    let slot = slot(Kind::SuperEcho, 1);
    let mut core = Core::new(Kind::SuperEcho, 48_000).unwrap();
    core.configure(slot, 120.0);
    // Mix belongs to SAD PET; the embedded digital mixer is forced fully wet.
    let mut dry = slot;
    dry.mix = 0.0;
    let mut p = processor(48000, dry);
    for frame in [[0.2, -0.3], [0.5, 0.4]] {
        assert_eq!(p.process(frame), frame);
    }
}
