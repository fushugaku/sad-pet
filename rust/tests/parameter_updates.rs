use sad_pet::{
    color::ColorSlot,
    dsp::{
        color::ColorRack,
        radias_waveshaper::{RadiasWaveshaper, WaveshaperMode},
    },
    preset::SaturatorType,
};

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
