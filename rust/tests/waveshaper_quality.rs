//! Behavioral regressions for the extended waveshaper. Spectral residual below
//! includes aliasing and quantization; it is neither THD nor a listening score.

use rustfft::{FftPlanner, num_complex::Complex};
use sad_pet::{
    Stereo,
    color::WaveshaperMode as Mode,
    dsp::{radias_waveshaper::RadiasWaveshaper, waveshaper::Processor},
    waveshaper::{Curve, Options, Quality},
};

const SAMPLE_RATE: u32 = 48_000;
const MODES: [Mode; 12] = [
    Mode::Drive,
    Mode::HardClip,
    Mode::Decimator,
    Mode::OctSaw,
    Mode::MultiTriangle,
    Mode::MultiSine,
    Mode::SubSaw,
    Mode::SubSquare,
    Mode::SubTriangle,
    Mode::SubSine,
    Mode::Pickup,
    Mode::LevelBoost,
];

fn signal(sample: usize) -> Stereo {
    let time = sample as f64 / SAMPLE_RATE as f64;
    [
        (0.63 * (std::f64::consts::TAU * 997.0 * time).sin()) as f32,
        (0.41 * (std::f64::consts::TAU * 1733.0 * time + 0.37).sin()) as f32,
    ]
}

fn quality_options(quality: Quality) -> Options {
    Options {
        quality,
        ..Options::default()
    }
}

#[test]
fn clean_stateful_modes_remain_finite_and_sub_pitch_keeps_host_time() {
    for rate in [8_000, 44_100, 48_000, 96_000] {
        for quality in [Quality::Clean2x, Quality::Clean4x] {
            for mode in [
                Mode::Drive,
                Mode::Pickup,
                Mode::Decimator,
                Mode::SubSaw,
                Mode::SubSquare,
                Mode::SubTriangle,
                Mode::SubSine,
            ] {
                let mut processor = Processor::new(rate);
                processor.reset_for_note(mode, 1.0, quality_options(quality), 69.0);
                for sample in 0..2048 {
                    let x = (sample as f32 * 0.173).sin() * 0.9;
                    let output = processor.process([x, -x * 0.7]);
                    assert!(
                        output.iter().all(|x| x.is_finite() && x.abs() < 4.0),
                        "{mode:?} at {rate}, {quality:?}"
                    );
                }
            }
            let mut processor = Processor::new(rate);
            processor.reset_for_note(Mode::SubSine, 1.0, quality_options(quality), 69.0);
            for _ in 0..4096 {
                processor.process([0.0; 2]);
            }
            let mut previous = processor.process([0.0; 2])[0];
            let mut crossings = 0;
            for _ in 0..rate {
                let value = processor.process([0.0; 2])[0];
                if previous <= 0.0 && value > 0.0 {
                    crossings += 1;
                }
                previous = value;
            }
            assert!(
                (219..=221).contains(&crossings),
                "Sub should remain 220 Hz at {rate}, {quality:?}: {crossings}"
            );
        }
    }
}

#[test]
fn neutral_legacy_preserves_all_native_modes_from_first_sample_and_partial_mix() {
    for mode in MODES {
        for mix in [0.0_f32, 0.37, 1.0] {
            let mut original = RadiasWaveshaper::new(SAMPLE_RATE);
            let mut extended = Processor::new(SAMPLE_RATE);
            original.configure(mode, 0.73);
            extended.configure(mode, 0.73, Options::default());
            original.set_pitch(69.25);
            extended.set_pitch(69.25);
            for sample in 0..2048 {
                let input = if sample % 257 == 0 {
                    [16.0, -16.0]
                } else {
                    signal(sample)
                };
                let wet = original.process_with_voice_gain(input, 0.63);
                let actual = extended.process_mixed(input, mix, 0.63);
                for channel in 0..2 {
                    let expected = if mix == 0.0 {
                        input[channel]
                    } else if mix == 1.0 {
                        wet[channel]
                    } else {
                        input[channel] * (1.0 - mix) + mix * wet[channel]
                    };
                    assert_eq!(
                        actual[channel].to_bits(),
                        expected.to_bits(),
                        "{mode:?}, mix={mix}, sample={sample}, channel={channel}"
                    );
                }
            }
        }
    }
}

#[test]
fn repeated_target_configuration_finishes_curve_changes_in_legacy_quality() {
    let mut processor = Processor::new(SAMPLE_RATE);
    processor.configure(Mode::HardClip, 0.75, Options::default());
    for _ in 0..512 {
        assert!(processor.process([-0.2; 2])[0] < 0.0);
    }

    // Hosts may publish identical controls every sample. This must not restart
    // the curve fade, and Legacy quality must still apply the optional curve.
    for curve in [Curve::Rectify, Curve::Custom] {
        let options = Options {
            curve,
            points: [0.0; 17],
            ..Options::default()
        };
        let mut last = [0.0; 2];
        for _ in 0..1024 {
            processor.configure(Mode::HardClip, 0.75, options);
            last = processor.process([-0.2; 2]);
        }
        if curve == Curve::Rectify {
            assert!(
                last[0] > 0.01,
                "Rectify never replaced the negative native output: {last:?}"
            );
        } else {
            assert!(
                last.iter().all(|x| x.abs() < 1e-6),
                "Custom zero curve never settled: {last:?}"
            );
        }
    }
}

#[test]
fn clean_identity_impulse_and_dry_branch_agree_on_reported_latency() {
    for (quality, latency) in [(Quality::Clean2x, 32), (Quality::Clean4x, 48)] {
        let options = quality_options(quality);
        let mut wet = Processor::new(SAMPLE_RATE);
        let mut dry = Processor::new(SAMPLE_RATE);
        // Depth zero disables the creative sample-and-hold. The remaining
        // identity path still has interpolation/decimation filters.
        wet.reset_for_note(Mode::Decimator, 0.0, options, 60.0);
        dry.reset_for_note(Mode::Decimator, 0.0, options, 60.0);
        assert_eq!(wet.latency_samples(), latency);
        let mut response = Vec::new();
        for sample in 0..256 {
            let input = if sample == 0 {
                [0.125, -0.0625]
            } else {
                [0.0; 2]
            };
            let output = wet.process(input);
            response.push(output[0]);
            let dry_output = dry.process_mixed(input, 0.0, 1.0);
            let expected = if sample == latency {
                [0.125, -0.0625]
            } else {
                [0.0; 2]
            };
            assert_eq!(dry_output, expected, "{quality:?}, dry sample={sample}");
        }
        let peak = response
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.abs().total_cmp(&b.abs()))
            .unwrap()
            .0;
        assert_eq!(peak, latency, "{quality:?}: actual impulse peak");
        assert!(
            (response.iter().sum::<f32>() - 0.125).abs() < 2e-5,
            "{quality:?}: identity path changed DC gain"
        );
    }
}

#[test]
fn returning_to_clean_quality_does_not_replay_frozen_audio() {
    for quality in [Quality::Clean2x, Quality::Clean4x] {
        let mut processor = Processor::new(SAMPLE_RATE);
        processor.reset_for_note(Mode::Drive, 0.65, quality_options(quality), 60.0);
        for sample in 0..4096 {
            processor.process(signal(sample));
        }
        processor.configure(Mode::Drive, 0.65, Options::default());
        for _ in 0..4096 {
            processor.process([0.0; 2]);
        }
        processor.configure(Mode::Drive, 0.65, quality_options(quality));
        let mut peak = 0.0_f32;
        for _ in 0..512 {
            for sample in processor.process([0.0; 2]) {
                peak = peak.max(sample.abs());
            }
        }
        assert!(
            peak < 1e-6,
            "{quality:?}: old audio returned with peak {peak}"
        );
    }
}

#[test]
fn fresh_note_is_independent_of_previous_audio_and_configuration() {
    for quality in [Quality::Legacy, Quality::Clean2x, Quality::Clean4x] {
        for mode in [Mode::Drive, Mode::Pickup, Mode::SubSaw] {
            let options = Options {
                quality,
                dc_block: true,
                bass: 0.3,
                envelope: 0.6,
                pre_tone: 0.7,
                post_tone: 0.8,
                ..Options::default()
            };
            let mut reused = Processor::new(SAMPLE_RATE);
            reused.reset_for_note(Mode::Pickup, 0.9, quality_options(Quality::Clean4x), 84.0);
            for sample in 0..512 {
                reused.process_mixed(signal(sample), 0.23, 0.8);
            }
            reused.reset_for_note(mode, 0.35, options, 52.0);
            let mut fresh = Processor::new(SAMPLE_RATE);
            fresh.reset_for_note(mode, 0.35, options, 52.0);
            for sample in 0..512 {
                let input = signal(sample + 101);
                assert_eq!(
                    reused.process_mixed(input, 0.71, 0.4),
                    fresh.process_mixed(input, 0.71, 0.4),
                    "{quality:?}, {mode:?}, sample={sample}"
                );
            }
        }
    }
}

#[test]
fn invalid_or_extreme_input_cannot_poison_future_audio() {
    let options = Options {
        quality: Quality::Clean2x,
        envelope: 0.25,
        dc_block: true,
        bass: 0.5,
        pre_tone: 0.7,
        ..Options::default()
    };
    for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::MAX] {
        let mut processor = Processor::new(SAMPLE_RATE);
        processor.reset_for_note(Mode::HardClip, 0.75, options, 60.0);
        let spike = processor.process([invalid; 2]);
        assert!(
            spike.iter().all(|x| x.is_finite()),
            "invalid spike escaped: {spike:?}"
        );
        let mut recovered_energy = 0.0_f64;
        for sample in 0..8192 {
            let output = processor.process(signal(sample));
            assert!(
                output.iter().all(|x| x.is_finite()),
                "poisoned sample={sample}"
            );
            if sample >= 4096 {
                recovered_energy += (output[0] as f64).powi(2);
            }
        }
        assert!(
            recovered_energy > 0.01,
            "audio stayed silent after input {invalid}"
        );
    }
}

fn nonharmonic_residual_ratio(quality: Quality, fundamental_bin: usize) -> f64 {
    const LENGTH: usize = 16_384;
    let mut processor = Processor::new(SAMPLE_RATE);
    processor.reset_for_note(Mode::HardClip, 0.75, quality_options(quality), 60.0);
    let mut spectrum = Vec::with_capacity(LENGTH);
    for sample in 0..2 * LENGTH {
        let phase = std::f64::consts::TAU * fundamental_bin as f64 * (sample % LENGTH) as f64
            / LENGTH as f64;
        let input = (0.8 * phase.sin()) as f32;
        let output = processor.process([input; 2])[0];
        if sample >= LENGTH {
            spectrum.push(Complex::new(output as f64, 0.0));
        }
    }
    FftPlanner::<f64>::new()
        .plan_fft_forward(LENGTH)
        .process(&mut spectrum);
    let mut wanted = 0.0;
    let mut residual = 0.0;
    // Coherent record: all unaliased harmonic bins, including even harmonics,
    // belong to the desired distortion. Exclude DC; count folded/nonharmonic
    // bins as residual. Quantization also contributes, so this is not pure alias
    // power and does not establish perceptual quality.
    for (bin, value) in spectrum.iter().enumerate().take(LENGTH / 2 + 1).skip(1) {
        let energy = value.norm_sqr();
        if bin % fundamental_bin == 0 {
            wanted += energy;
        } else {
            residual += energy;
        }
    }
    assert!(
        wanted > 1.0,
        "{quality:?}: no useful output for FFT comparison"
    );
    residual / wanted
}

#[test]
fn clean_hard_clip_reduces_nonharmonic_residual_at_one_and_five_khz() {
    // 999.02 Hz and 5000.98 Hz, coherent with the 16384-sample FFT at 48 kHz.
    for fundamental_bin in [341, 1707] {
        let legacy = nonharmonic_residual_ratio(Quality::Legacy, fundamental_bin);
        let clean2 = nonharmonic_residual_ratio(Quality::Clean2x, fundamental_bin);
        let clean4 = nonharmonic_residual_ratio(Quality::Clean4x, fundamental_bin);
        eprintln!(
            "bin {fundamental_bin}: residual/harmonic dB Legacy={:.2}, 2x={:.2}, 4x={:.2}",
            10.0 * legacy.log10(),
            10.0 * clean2.log10(),
            10.0 * clean4.log10()
        );
        assert!(
            clean2 < legacy * 0.5,
            "2x failed to reduce residual by 3 dB at bin {fundamental_bin}"
        );
        assert!(
            clean4 < legacy * 0.25,
            "4x failed to reduce residual by 6 dB at bin {fundamental_bin}"
        );
    }
}

#[test]
fn gain_match_tracks_signal_level_and_remains_quiet_on_silence() {
    let mut processor = Processor::new(SAMPLE_RATE);
    let options = Options {
        input: 0.85,
        auto_gain: true,
        ..Options::default()
    };
    processor.reset_for_note(Mode::HardClip, 0.75, options, 60.0);
    for _ in 0..2048 {
        let output = processor.process([0.0; 2]);
        assert!(output.iter().all(|x| x.is_finite() && x.abs() < 1e-6));
    }
    let mut input_energy = 0.0_f64;
    let mut output_energy = 0.0_f64;
    for sample in 0..65_536 {
        let input = (0.2 * (std::f64::consts::TAU * sample as f64 / 48.0).sin()) as f32;
        let output = processor.process([input; 2])[0];
        assert!(output.is_finite());
        if sample >= 57_344 {
            input_energy += (input as f64).powi(2);
            output_energy += (output as f64).powi(2);
        }
    }
    let rms_ratio = (output_energy / input_energy).sqrt();
    assert!(
        (0.85..=1.15).contains(&rms_ratio),
        "gain match did not approach input RMS: output/input={rms_ratio}"
    );
}
