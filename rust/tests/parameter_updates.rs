use sad_pet::{
    color::ColorSlot,
    dsp::{
        Engine, SampleBuffer,
        color::ColorRack,
        radias_waveshaper::{RadiasWaveshaper, WaveshaperMode},
    },
    performance::{Mode, Modulation, Performance, Settings},
    preset::{Preset, SaturatorType},
};
use std::sync::Arc;

#[test]
fn sparse_rate_edits_retime_the_next_note_like_a_full_preset_update() {
    for (mode, parameter) in [(Mode::Arp, "arp_rate"), (Mode::Sequencer, "sequence_rate")] {
        let mut preset = Preset::default();
        preset.performance.mode = mode;
        preset.performance.arp_division = 8.0;
        preset.performance.sequence_division = 8.0;
        let sample = Arc::new(
            SampleBuffer::new(
                "tone".into(),
                8000,
                (0..8000)
                    .map(|i| [(i as f32 * 0.17).sin() * 0.2; 2])
                    .collect(),
            )
            .unwrap(),
        );
        let mut sparse = Engine::new(8000, sample.clone(), preset.clone());
        let mut full = Engine::new(8000, sample, preset.clone());
        sparse.set_playing(true);
        full.set_playing(true);
        for _ in 0..128 {
            assert_eq!(sparse.process_frame(), full.process_frame());
        }
        assert_eq!(sparse.held_notes(), vec![60]);
        if mode == Mode::Arp {
            preset.performance.arp_division = 0.25;
        } else {
            preset.performance.sequence_division = 0.25;
        }
        let changes = sad_pet::parameters::parse(&serde_json::json!({parameter: 0.25})).unwrap();
        sparse.apply_parameters(&changes);
        full.set_preset(preset);
        // Editing a rate leaves the old note held until the next audio frame.
        assert_eq!(sparse.held_notes(), vec![60]);
        for frame in 0..256 {
            assert_eq!(
                sparse.process_frame(),
                full.process_frame(),
                "{mode:?}, frame {frame}"
            );
            assert_eq!(sparse.held_notes(), vec![64]);
        }
    }
}

#[test]
fn unrelated_numeric_refresh_does_not_retrigger_a_modulated_clock() {
    let settings = Settings {
        mode: Mode::Arp,
        arp_division: 8.0,
        ..Settings::default()
    };
    let mut performance = Performance::new(settings.clone());
    let mut modulation = Modulation::from_settings(&settings);
    modulation.arp_rate = 4.0;
    performance.modulate(modulation);
    performance.start();
    assert!(performance.tick(8000, 120.0).iter().any(Option::is_some));
    performance.refresh_timing(&settings);
    assert!(performance.tick(8000, 120.0).iter().all(Option::is_none));
}

#[test]
fn cached_waveshapers_match_direct_processing_across_pitch_changes_bypass_and_resets() {
    for mode in WaveshaperMode::ALL {
        let mut rack = ColorRack::new(8000);
        let mut reference = RadiasWaveshaper::new(8000);
        let mut previous_enabled = true;
        for frame in 0..1536 {
            if frame == 1024 {
                rack.clear();
                reference.clear();
            }
            let block = frame / 32;
            let enabled = !(20..24).contains(&block);
            if frame % 32 == 0 {
                if frame == 0 || enabled != previous_enabled {
                    reference.clear();
                }
                let depth = if block < 32 { 0.3 } else { 0.7 };
                let mut slot = ColorSlot {
                    id: 17,
                    kind: SaturatorType::Waveshaper,
                    enabled,
                    mix: 1.0,
                    ..Default::default()
                };
                slot.settings.waveshaper_type = mode;
                slot.settings.waveshaper_depth = depth;
                rack.configure_tempo(&[slot], 120.0);
                reference.configure(mode, depth);
                let note = 48.0 + (block % 8) as f32 * 3.1;
                rack.set_pitch(note);
                reference.set_pitch(note);
                previous_enabled = enabled;
            }
            let input = [
                (frame as f32 * 0.071).sin() * 0.25,
                (frame as f32 * 0.033).cos() * 0.2,
            ];
            let expected = if enabled {
                reference.process(input)
            } else {
                input
            };
            assert_eq!(rack.process(input), expected, "{mode:?}, frame {frame}");
        }
    }
}
