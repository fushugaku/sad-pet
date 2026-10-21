use sad_pet::{
    dsp::{Engine, Fades, Region, SampleBuffer},
    preset::{Env, Preset},
    wav,
};
use std::{io::Cursor, sync::Arc};

#[test]
fn fades_have_silent_ends_smooth_monotonic_ramps_and_follow_the_selected_region() {
    let fades = Fades {
        fade_in: 0.02,
        fade_out: 0.03,
    };
    let range = 800..1600;
    assert_eq!(fades.gain(800.0, 8000, &range), 0.0);
    assert_eq!(fades.gain(1599.0, 8000, &range), 0.0);
    assert_eq!(fades.gain(1100.0, 8000, &range), 1.0);
    for i in 800..960 {
        assert!(fades.gain(i as f64, 8000, &range) <= fades.gain(i as f64 + 1.0, 8000, &range));
    }
    for i in 1360..1599 {
        assert!(fades.gain(i as f64, 8000, &range) >= fades.gain(i as f64 + 1.0, 8000, &range));
    }
    assert_eq!(fades.gain(1600.0, 8000, &range), 0.0);
    let long = Fades {
        fade_in: 10.0,
        fade_out: 10.0,
    };
    for i in 0..=80 {
        assert!((0.0..=1.0).contains(&long.gain(i as f64 * 0.5, 8000, &(0..40))));
    }
    assert!(
        Fades {
            fade_in: -0.1,
            fade_out: 0.0
        }
        .validate()
        .is_err()
    );
    assert!(
        Fades {
            fade_in: 0.0,
            fade_out: f64::NAN
        }
        .validate()
        .is_err()
    );
}

#[test]
fn fade_export_preserves_duration_original_frames_and_stereo_balance() {
    let sample = SampleBuffer::new("Take".into(), 8000, vec![[0.4, -0.2]; 8000]).unwrap();
    let region = Region {
        start: 0.25,
        end: 0.75,
    };
    let fades = Fades {
        fade_in: 0.1,
        fade_out: 0.1,
    };
    let original = sample.trim_wav(region).unwrap();
    assert_eq!(
        original,
        sample
            .trim_wav_with_fades(region, Fades::default())
            .unwrap()
    );
    let faded = wav::decode(
        Cursor::new(sample.trim_wav_with_fades(region, fades).unwrap()),
        "Fade".into(),
    )
    .unwrap();
    assert_eq!(faded.len(), 4000);
    assert_eq!(faded.frames()[0], [0.0; 2]);
    assert_eq!(faded.frames()[3999], [0.0; 2]);
    assert_eq!(faded.frames()[2000], [0.4, -0.2]);
    assert!(
        faded
            .frames()
            .iter()
            .all(|[l, r]| (*l + 2.0 * r).abs() < 1e-7)
    );
    assert!(sample.frames().iter().all(|f| *f == [0.4, -0.2]));
}

#[test]
fn realtime_dry_and_bypass_loop_through_fades_and_new_files_reset_them() {
    let sample = Arc::new(
        SampleBuffer::new(
            "Ramp".into(),
            8000,
            (0..8000).map(|i| [i as f32 / 8000.0 - 0.5; 2]).collect(),
        )
        .unwrap(),
    );
    for bypass in [false, true] {
        let mut p = Preset::default();
        p.params.mix = 0.0;
        p.params.grains = 0.0;
        p.params.min_grains = 0.0;
        p.params.master_volume = 0.5;
        p.color_chain.clear();
        p.envs.fill(Env {
            attack: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
            ..Env::default()
        });
        let mut e = Engine::new(8000, sample.clone(), p);
        e.set_region(Region {
            start: 0.25,
            end: 0.75,
        })
        .unwrap();
        e.set_fades(Fades {
            fade_in: 0.05,
            fade_out: 0.05,
        })
        .unwrap();
        e.bypass = bypass;
        e.set_playing(true);
        let mut output = vec![[0.0; 2]; 12000];
        e.process(&mut output);
        for boundary in [4000, 8000] {
            assert_eq!(output[boundary - 1], [0.0; 2]);
            assert_eq!(output[boundary], [0.0; 2]);
            assert!((output[boundary + 1][0] - output[boundary][0]).abs() < 0.0001);
        }
        assert!(output[2000][0].abs() < 0.0001);
        assert!(output.iter().flatten().any(|v| v.abs() > 0.03));
        e.set_live_input(true, true);
        assert!(e.set_fades(Fades::default()).is_err());
        e.set_sample(sample.clone());
        assert_eq!(e.fades, Fades::default());
    }
}

#[test]
fn forward_and_reverse_grains_read_the_faded_source() {
    let sample =
        Arc::new(SampleBuffer::new("Constant".into(), 8000, vec![[0.2; 2]; 8000]).unwrap());
    for grain_size in [0.2, 0.8] {
        let run = |fades| {
            let mut p = Preset::default();
            p.params.mix = 1.0;
            p.params.cloud = 1.0;
            p.params.grains = 0.7;
            p.params.grain_size = grain_size;
            p.color_chain.clear();
            let mut e = Engine::new(8000, sample.clone(), p);
            e.set_region(Region {
                start: 0.4,
                end: 0.5,
            })
            .unwrap();
            e.set_fades(fades).unwrap();
            e.set_playing(true);
            (0..8000).map(|_| e.process_frame()[0].powi(2)).sum::<f32>()
        };
        let baseline = run(Fades::default());
        assert!(baseline > 0.1);
        // Overlapping full-length ramps peak at 1/4 amplitude (1/16 power).
        assert!(
            run(Fades {
                fade_in: 1.0,
                fade_out: 1.0
            }) < baseline * 0.065
        );
    }
}
