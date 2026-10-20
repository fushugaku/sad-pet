//! Deterministic offline differential render and block timing. No audio device.
//! cargo run -p sad-pet --no-default-features --release --example audio_benchmark -- capture target/perf/before
//! Repeat with `compare` after a DSP change, using the same compiler/platform.
use sad_pet::{
    color::ColorSlot,
    dsp::{Engine, Region, SampleBuffer},
    midi::Message,
    preset::{Destination, FilterRouting, FilterType, Preset, SaturatorType},
};
use std::{hint::black_box, path::PathBuf, sync::Arc, time::Instant};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert!(args.len() == 3 && ["capture", "compare"].contains(&args[1].as_str()));
    let folder = PathBuf::from(&args[2]);
    std::fs::create_dir_all(&folder).unwrap();
    for rate in [44_100, 48_000, 96_000] {
        let sample = Arc::new(SampleBuffer::demo(rate));
        for case in 0..8 {
            let mut p = Preset::builtin(case % 5);
            if case >= 3 {
                p.params.grains = 1.0;
                p.params.grain_size = 0.9;
                p.params.filter_routing = FilterRouting::Serial;
                p.params.vcf_type = FilterType::BandPass;
                p.params.filter = 0.65;
                p.params.filter2 = 0.8;
                p.params.in_gain = 3.1;
                p.params.out_gain = -1.3;
                p.lfos[0].destination = Destination::Filter;
                p.lfos[0].depth = 0.3;
            }
            if case == 4 || case == 7 {
                let kinds = [
                    SaturatorType::Distortion,
                    SaturatorType::Crusher,
                    SaturatorType::Reducer,
                    SaturatorType::Chorus,
                    SaturatorType::Flanger,
                    SaturatorType::Binaural,
                    SaturatorType::Reverb,
                    SaturatorType::LcrDelay,
                    SaturatorType::ModDelay,
                    SaturatorType::Compressor,
                    SaturatorType::Waveshaper,
                ];
                p.color_chain = kinds
                    .into_iter()
                    .enumerate()
                    .map(|(i, kind)| {
                        let mut s = ColorSlot::legacy(kind, 0.2);
                        s.id = i as u32 + 1;
                        s.settings.distortion_bias = 0.62;
                        s.settings.crusher_dither = 0.15;
                        s
                    })
                    .collect();
                p.pre_color_chain = p
                    .color_chain
                    .iter()
                    .copied()
                    .map(|mut s| {
                        s.id += 100;
                        s
                    })
                    .collect();
            }
            p.params.loop_one_bar = case == 5;
            let mut e = Engine::new(rate, sample.clone(), p);
            if case == 2 || case == 5 {
                e.set_region(Region {
                    start: 0.251,
                    end: 0.613,
                })
                .unwrap();
            }
            if case == 6 {
                e.set_region(Region {
                    start: 0.5,
                    end: 0.5001,
                })
                .unwrap();
            }
            if case == 7 {
                e.set_live_input(true, false);
            }
            e.set_playing(true);
            if case >= 3 {
                e.set_midi_gate(true);
                for note in [36, 48, 55, 60, 67, 72, 79, 91] {
                    e.midi(Message::NoteOn {
                        channel: 0,
                        note,
                        velocity: 100,
                    });
                }
            }
            let mut output = Vec::with_capacity(rate as usize * 3 * 8);
            let mut times = Vec::new();
            let mut block = [[0.0; 2]; 128];
            for n in 0..(rate as usize * 3 / 128) {
                if n % 137 == 0 {
                    let changes = sad_pet::parameters::parse(&serde_json::json!({
                        "in_gain": (n % 9) as f32 - 4.0, "out_gain": (n % 7) as f32 - 3.0,
                        "pitch": 0.3 + (n % 5) as f32 * 0.15,
                    }))
                    .unwrap();
                    e.apply_parameters(&changes);
                    e.set_time_signature(if n % 274 == 0 { 7 } else { 4 }, 8);
                    e.midi(Message::PitchBend {
                        channel: 0,
                        value: (n * 13 % 16384) as u16,
                    });
                }
                if n == 280 {
                    e.set_frozen(true);
                }
                if n == 340 {
                    e.set_frozen(false);
                }
                if n == 420 {
                    e.bypass = true;
                }
                if n == 450 {
                    e.bypass = false;
                }
                let input = [(n as f32 * 0.01).sin() * 0.2; 2];
                let start = Instant::now();
                for frame in &mut block {
                    *frame = e.process_input_frame(black_box(input));
                }
                let elapsed = start.elapsed().as_secs_f64() * 1e6;
                if n > 100 {
                    times.push(elapsed);
                }
                for frame in &block {
                    for value in frame {
                        output.extend_from_slice(&value.to_bits().to_le_bytes());
                    }
                }
            }
            let file = folder.join(format!("{rate}-{case}.f32"));
            if args[1] == "capture" {
                std::fs::write(file, &output).unwrap();
            } else {
                let before = std::fs::read(&file).unwrap();
                assert_eq!(before.len(), output.len(), "{} length", file.display());
                let differences = before
                    .chunks_exact(4)
                    .zip(output.chunks_exact(4))
                    .filter(|(a, b)| a != b)
                    .count();
                assert_eq!(differences, 0, "{}: changed samples", file.display());
            }
            times.sort_by(f64::total_cmp);
            println!(
                "{rate}-{case}: samples={} median_us={:.2} p99_us={:.2} max_us={:.2} {}",
                output.len() / 4,
                times[times.len() / 2],
                times[times.len() * 99 / 100],
                times[times.len() - 1],
                args[1]
            );
        }
    }
}
