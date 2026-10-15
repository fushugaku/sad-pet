use sad_pet::{
    dsp::{Engine, SampleBuffer},
    firmware::Image,
    preset::Preset,
    server, wav,
};
use std::{path::Path, sync::Arc};

fn value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|x| x == flag)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str).unwrap_or("serve") {
        "serve" => {
            let port = value(&args, "--port")
                .unwrap_or("3214")
                .parse::<u16>()
                .map_err(|_| "Invalid port")?;
            if port == 0 {
                return Err("Choose a port from 1 to 65535".into());
            }
            server::serve(
                port,
                !args.iter().any(|s| s == "--no-audio"),
                !args.iter().any(|s| s == "--no-open"),
            )
        }
        "render" | "demo" => {
            let demo = args[0] == "demo";
            let expected = if demo { 2 } else { 3 };
            if args.len() < expected {
                return Err("Usage: sad-pet render input.wav output.wav [--seconds 10] [--preset preset.sad.json]\n       sad-pet demo output.wav [--seconds 10] [--builtin 0..4]".into());
            }
            let sample = Arc::new(if demo {
                SampleBuffer::demo(48_000)
            } else {
                wav::read(Path::new(&args[1]))?
            });
            let output = &args[if demo { 1 } else { 2 }];
            let seconds = value(&args, "--seconds")
                .map(|s| s.parse::<f64>().map_err(|_| "Invalid duration".to_owned()))
                .transpose()?
                .unwrap_or(sample.duration());
            if !seconds.is_finite() || !(0.1..=3600.0).contains(&seconds) {
                return Err("Duration must be 0.1–3600 seconds".into());
            }
            let mut preset = if let Some(path) = value(&args, "--preset") {
                let value: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
                        .map_err(|e| e.to_string())?;
                if value.get("format").and_then(|v| v.as_str()) != Some("evil-pet-rust-v1") {
                    return Err("Use an SAD PET preset (.sad.json)".into());
                }
                serde_json::from_value::<Preset>(value).map_err(|e| e.to_string())?
            } else {
                Preset::builtin(
                    value(&args, "--builtin")
                        .unwrap_or("0")
                        .parse()
                        .map_err(|_| "Invalid builtin index")?,
                )
            };
            preset.sanitize();
            let started = std::time::Instant::now();
            let mut engine = Engine::new(48_000, sample, preset);
            engine.set_playing(true);
            let file = std::fs::File::create(output).map_err(|e| e.to_string())?;
            let spec = hound::WavSpec {
                channels: 2,
                sample_rate: 48_000,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            let mut writer = hound::WavWriter::new(file, spec).map_err(|e| e.to_string())?;
            let mut frames = [[0.0; 2]; 1024];
            let mut remaining = (seconds * 48_000.0).round() as usize;
            let mut peak = 0.0_f32;
            while remaining > 0 {
                let count = remaining.min(frames.len());
                engine.process(&mut frames[..count]);
                for frame in &frames[..count] {
                    for &value in frame {
                        peak = peak.max(value.abs());
                        writer.write_sample(value).map_err(|e| e.to_string())?;
                    }
                }
                remaining -= count;
            }
            writer.finalize().map_err(|e| e.to_string())?;
            println!(
                "Rendered {seconds:.2}s stereo, 48 kHz, float32 → {output}\nPeak {peak:.4}; processing {:.3}s",
                started.elapsed().as_secs_f64()
            );
            Ok(())
        }
        "inspect" => {
            let path = args.get(1).ok_or("Usage: sad-pet inspect firmware.fw")?;
            let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
            let image = Image::parse(&bytes).map_err(|e| e.to_string())?;
            println!(
                "STM32 v2 · {} payload bytes · entry 0x{:08x} · flags 0x{:08x} · checksum 0x{:08x} OK",
                image.payload.len(),
                image.entry,
                image.flags,
                image.checksum
            );
            Ok(())
        }
        "--help" | "help" | "-h" => {
            println!(
                "SAD PET\n  serve [--port 3214] [--no-open] [--no-audio]\n  render input.wav output.wav [--seconds 10] [--preset preset.sad.json]\n  demo output.wav [--seconds 10] [--builtin 0..4]\n  inspect firmware.fw"
            );
            Ok(())
        }
        other => Err(format!("Unknown command {other}; use --help")),
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
