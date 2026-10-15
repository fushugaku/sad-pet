use sad_pet::{
    dsp::{Engine, Region, SampleBuffer},
    preset::Preset,
    wav,
};
use std::{io::Cursor, sync::Arc};

#[test]
fn trim_keeps_exact_stereo_frames_and_the_original() {
    let frames: Vec<_> = (0..8_000)
        .map(|i| [i as f32 / 8_000.0, -(i as f32) / 16_000.0])
        .collect();
    let sample = SampleBuffer::new("Take".into(), 8_000, frames.clone()).unwrap();
    let bytes = sample
        .trim_wav(Region {
            start: 0.250125,
            end: 0.750375,
        })
        .unwrap();
    let trimmed = wav::decode(Cursor::new(bytes), "Trim".into()).unwrap();
    assert_eq!(trimmed.sample_rate, 8_000);
    assert_eq!(trimmed.frames(), &frames[2001..6003]);
    assert_eq!(sample.frames(), frames);
    assert_eq!(sample.duration(), 1.0);
    let one = sample
        .trim_wav(Region {
            start: 0.5,
            end: 0.500125,
        })
        .unwrap();
    assert_eq!(
        wav::decode(Cursor::new(one), "One".into()).unwrap().len(),
        1
    );
    for region in [
        Region {
            start: -0.1,
            end: 0.5,
        },
        Region {
            start: 0.0,
            end: 1.1,
        },
        Region {
            start: 0.5,
            end: 0.5,
        },
        Region {
            start: 0.6,
            end: 0.4,
        },
        Region {
            start: f64::NAN,
            end: 1.0,
        },
        Region {
            start: 0.5,
            end: 0.500001,
        },
    ] {
        assert!(sample.trim_wav(region).is_err());
    }
}

#[test]
fn interpolation_and_mips_never_include_excluded_audio() {
    let mut frames = vec![[0.9, -0.7]; 8_001];
    frames[2001..6000].fill([0.0; 2]);
    let sample = SampleBuffer::new("Outside is loud".into(), 8_000, frames).unwrap();
    for rate in [0.1, 1.0, 2.1, 4.0, 16.0, 64.0, -0.7, -32.0] {
        for i in -2000..20000 {
            assert_eq!(
                sample.read_region(i as f64 * 0.75, rate, true, &(2001..6000)),
                [0.0; 2]
            );
        }
        for position in [-0.75, 0.0, 2001.3, 5999.9, 16000.2] {
            assert_eq!(
                sample.read(position, rate, true),
                sample.read_region(position, rate, true, &(0..sample.len()))
            );
        }
    }
}

#[test]
fn moving_a_loop_limits_dry_bypass_grains_cloud_and_frozen_position() {
    let mut frames = vec![[0.4; 2]; 8_000];
    frames[2000..4000].fill([0.0; 2]);
    let sample = Arc::new(SampleBuffer::new("Selected silence".into(), 8_000, frames).unwrap());
    for bypass in [false, true] {
        for mix in [0.0, 1.0] {
            for grain_size in [0.2, 0.8] {
                let mut preset = Preset::default();
                preset.params.mix = mix;
                preset.params.cloud = 1.0;
                preset.params.grain_size = grain_size;
                preset.params.pitch = 1.0;
                preset.params.position = 0.8;
                let mut engine = Engine::new(8_000, sample.clone(), preset);
                engine.bypass = bypass;
                engine.set_playing(true);
                for _ in 0..1_000 {
                    engine.process_frame();
                }
                engine.set_frozen(true);
                engine
                    .set_region(Region {
                        start: 0.25,
                        end: 0.5,
                    })
                    .unwrap();
                for i in 0..16_000 {
                    if i == 4_000 {
                        engine.set_frozen(false);
                    }
                    assert_eq!(
                        engine.process_frame(),
                        [0.0; 2],
                        "excluded sound leaked with mix {mix} bypass {bypass} grain {grain_size}"
                    );
                    assert!((2000.0..4000.0).contains(&engine.center()));
                    assert!(
                        engine
                            .grain_views()
                            .iter()
                            .all(|g| (0.25..0.5).contains(&g.position))
                    );
                }
                engine.set_live_input(true, true);
                assert!(engine.set_region(Region::default()).is_err());
                engine.set_live_input(false, false);
                assert!((2000.0..4000.0).contains(&engine.center()));
                engine.set_sample(sample.clone());
                assert_eq!(engine.region, Region::default());
            }
        }
    }
}
