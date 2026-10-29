use sad_pet::{
    dsp::{Engine, SampleBuffer},
    preset::Preset,
    rack::{Module, Settings, Slot},
};
use std::sync::Arc;

#[test]
fn changing_grain_count_does_not_step_the_level_of_a_constant_source() {
    for rate in [8000, 44100, 48000, 96000] {
        let sample = Arc::new(
            SampleBuffer::new("Constant".into(), rate, vec![[0.05; 2]; rate as usize / 2]).unwrap(),
        );
        // Legacy grains, Rack file grains, and Rack grains of a live chain
        // all share the same normalization, including advanced group sends.
        for (name, modules) in [
            ("classic", vec![]),
            ("rack file", vec![Module::Sample, Module::Grains]),
            (
                "rack history",
                vec![Module::Sample, Module::PostFx, Module::Grains],
            ),
        ] {
            let mut p = Preset::default();
            p.color_chain.clear();
            p.pre_color_chain.clear();
            p.source_color_chain.clear();
            p.sequencer.enabled = false;
            p.resynth.enabled = false;
            p.params.filter = 1.0;
            p.params.filter_env_depth = 0.0;
            p.params.filter_resonance = 0.0;
            p.params.grain_size = 0.65;
            p.params.grains = 0.8;
            p.params.min_grains = 0.0;
            p.params.grain_shape = 0.5;
            p.params.stereo_spread = 0.0;
            p.params.cloud = 0.0;
            p.params.detune = 0.5;
            p.params.mix = 1.0;
            p.params.master_volume = 0.5;
            p.params.adsr_sustain = 1.0;
            if !modules.is_empty() {
                p.rack = Some(Box::new(Settings {
                    modules: modules
                        .into_iter()
                        .map(|module| Slot {
                            module,
                            enabled: true,
                            id: 0,
                        })
                        .collect(),
                    ..Default::default()
                }));
            }
            let mut e = Engine::new(rate, sample.clone(), p);
            e.set_playing(true);
            let (mut previous, mut max_delta, mut peak) = (0.0_f32, 0.0_f32, 0.0_f32);
            for n in 0..rate * 2 {
                let frame = e.process_frame();
                assert!(frame.iter().all(|x| x.is_finite()));
                if n > rate {
                    max_delta = max_delta.max((frame[0] - previous).abs());
                    peak = peak.max(frame[0].abs());
                }
                previous = frame[0];
            }
            assert!(peak > 0.01 && peak < 0.2, "{name} at {rate}: peak {peak}");
            assert!(
                max_delta * (rate as f32) < 8.0,
                "{name} at {rate}: grain count made a one-sample jump of {max_delta}"
            );
        }
    }
}
