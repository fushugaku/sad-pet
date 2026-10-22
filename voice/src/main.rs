//! `sad-voice`: voicebank import and offline rendering from the command line.
//!
//!     sad-voice import <utau folder> [--language ja|en|ru] [--name NAME] -o voice.sadvoice
//!     sad-voice render <score.json> [--voice voice.sadvoice] [--rate 48000] -o out.wav
//!     sad-voice demo <ja|en|ru> [--voice voice.sadvoice] -o out.wav
//!     sad-voice analyze <file.wav>
//!     sad-voice info <voice.sadvoice>
use sad_pet_voice::bank::Voicebank;
use sad_pet_voice::phoneme::Language;
use sad_pet_voice::render::{self, Voice};
use sad_pet_voice::score::{Note, Score, TICKS_PER_BEAT};
use sad_pet_voice::{analysis, utau, wav};
use std::path::PathBuf;
use std::sync::Arc;

struct Args {
    positional: Vec<String>,
    options: std::collections::HashMap<String, String>,
}

fn args() -> Args {
    let mut positional = Vec::new();
    let mut options = std::collections::HashMap::new();
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        if let Some(key) = a.strip_prefix("--").or_else(|| a.strip_prefix('-')) {
            options.insert(key.to_string(), it.next().unwrap_or_default());
        } else {
            positional.push(a);
        }
    }
    Args {
        positional,
        options,
    }
}

fn fail(message: impl std::fmt::Display) -> ! {
    eprintln!("sad-voice: {message}");
    std::process::exit(1)
}

fn voice(args: &Args) -> (Voice, String) {
    match args.options.get("voice") {
        Some(path) => {
            let bytes = std::fs::read(path).unwrap_or_else(|e| fail(format!("{path}: {e}")));
            let bank = Voicebank::from_bytes(&bytes).unwrap_or_else(|e| fail(e));
            (Voice::Bank(Arc::new(bank)), path.clone())
        }
        None => (Voice::Formant, String::new()),
    }
}

fn output(args: &Args) -> PathBuf {
    PathBuf::from(
        args.options
            .get("o")
            .or(args.options.get("output"))
            .cloned()
            .unwrap_or_else(|| fail("missing -o")),
    )
}

fn demo(language: Language) -> Score {
    // A short tune in each language; one syllable per note.
    let (lyrics, keys): (&[&str], &[u8]) = match language {
        Language::Ja => (
            &[
                "さ", "く", "ら", "さ", "く", "ら", "や", "よ", "い", "の", "そ", "ら", "は",
            ],
            &[69, 69, 71, 69, 69, 71, 69, 71, 72, 71, 69, 71, 69],
        ),
        Language::En => (
            &[
                "twin-", "kle", "twin-", "kle", "lit-", "tle", "star", "how", "I", "won-", "der",
                "what", "you", "are",
            ],
            &[60, 60, 67, 67, 69, 69, 67, 65, 65, 64, 64, 62, 62, 60],
        ),
        Language::Ru => (
            &[
                "в ле-", "су", "ро-", "ди-", "лась", "ё-", "лоч-", "ка", "в ле-", "су", "о-", "на",
                "рос-", "ла",
            ],
            &[60, 69, 69, 67, 69, 65, 60, 60, 60, 69, 69, 70, 67, 72],
        ),
    };
    let mut score = Score {
        language,
        ..Default::default()
    };
    let mut tick = TICKS_PER_BEAT * 2;
    for (i, (lyric, key)) in lyrics.iter().zip(keys).enumerate() {
        let length = if i + 1 == lyrics.len() || i % 7 == 6 {
            TICKS_PER_BEAT * 2
        } else {
            TICKS_PER_BEAT
        };
        score.notes.push(Note {
            id: i as u32 + 1,
            start: tick,
            length,
            key: *key,
            lyric: lyric.to_string(),
            ..Default::default()
        });
        tick += length + if i % 7 == 6 { TICKS_PER_BEAT } else { 0 };
    }
    score.sanitize();
    score
}

fn write(path: &PathBuf, rate: u32, samples: &[f32]) {
    std::fs::write(path, wav::write_float(rate, samples))
        .unwrap_or_else(|e| fail(format!("{}: {e}", path.display())));
    let peak = samples.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    eprintln!(
        "wrote {} ({:.2} s, peak {:.1} dBFS)",
        path.display(),
        samples.len() as f32 / rate as f32,
        20.0 * peak.max(1e-9).log10()
    );
}

fn main() {
    let args = args();
    let command = args.positional.first().map(String::as_str).unwrap_or("");
    let rate: u32 = args
        .options
        .get("rate")
        .and_then(|r| r.parse().ok())
        .unwrap_or(48_000);
    match command {
        "import" => {
            let dir = args
                .positional
                .get(1)
                .unwrap_or_else(|| fail("missing voicebank folder"));
            let options = utau::ImportOptions {
                language: args
                    .options
                    .get("language")
                    .and_then(|l| Language::parse(l)),
                name: args.options.get("name").cloned(),
            };
            let started = std::time::Instant::now();
            let bank = utau::import(
                &utau::Directory(PathBuf::from(dir)),
                &options,
                &mut |done, total| {
                    eprint!("\ranalyzing {done}/{total}");
                },
            )
            .unwrap_or_else(|e| fail(e));
            eprintln!();
            let path = output(&args);
            let bytes = bank.to_bytes();
            std::fs::write(&path, &bytes).unwrap_or_else(|e| fail(e));
            eprintln!(
                "{}: {} units, {} phonemes, {} frames, {:.1} MB, {:.1} s",
                bank.info.name,
                bank.units.len(),
                bank.inventory().len(),
                bank.frame_count(),
                bytes.len() as f64 / 1e6,
                started.elapsed().as_secs_f64()
            );
        }
        "info" => {
            let path = args
                .positional
                .get(1)
                .unwrap_or_else(|| fail("missing voicebank"));
            let bank = Voicebank::from_bytes(&std::fs::read(path).unwrap_or_else(|e| fail(e)))
                .unwrap_or_else(|e| fail(e));
            println!(
                "{} ({}) by {}",
                bank.info.name,
                bank.info.language.code(),
                bank.info.author
            );
            println!("{} units, {} frames", bank.units.len(), bank.frame_count());
            let symbols: Vec<_> = bank.inventory().iter().map(|p| p.symbol()).collect();
            println!("phonemes: {}", symbols.join(" "));
            for unit in bank.units.iter().take(
                args.options
                    .get("units")
                    .and_then(|n| n.parse().ok())
                    .unwrap_or(0),
            ) {
                let ph: Vec<_> = unit.phonemes.iter().map(|p| p.symbol()).collect();
                println!(
                    "  {:>12}  [{}]  pitch {:.1}  level {:.1}  bounds {:?}",
                    unit.alias,
                    ph.join(" "),
                    unit.pitch,
                    unit.level,
                    unit.bounds
                );
            }
        }
        "render" | "demo" => {
            let (voice, id) = voice(&args);
            let score = if command == "demo" {
                let language = args
                    .positional
                    .get(1)
                    .and_then(|l| Language::parse(l))
                    .unwrap_or(Language::Ja);
                demo(language)
            } else {
                let path = args
                    .positional
                    .get(1)
                    .unwrap_or_else(|| fail("missing score"));
                let text = std::fs::read_to_string(path).unwrap_or_else(|e| fail(e));
                let mut score: Score = serde_json::from_str(&text).unwrap_or_else(|e| fail(e));
                score.sanitize();
                score
            };
            let mut score = score;
            if let Some(style) = args.options.get("style") {
                score.style = sad_pet_voice::score::Style::presets()
                    .into_iter()
                    .find(|s| s.name.eq_ignore_ascii_case(style))
                    .unwrap_or_else(|| fail(format!("unknown style {style}")));
            }
            let started = std::time::Instant::now();
            let samples = render::render_song(&score, &voice, &id, rate, None);
            let elapsed = started.elapsed().as_secs_f64();
            eprintln!(
                "rendered in {:.2} s ({:.1}x real time)",
                elapsed,
                samples.len() as f64 / rate as f64 / elapsed
            );
            write(&output(&args), rate, &samples);
        }
        "analyze" => {
            let path = args
                .positional
                .get(1)
                .unwrap_or_else(|| fail("missing WAV"));
            let audio = wav::read(&std::fs::read(path).unwrap_or_else(|e| fail(e)))
                .unwrap_or_else(|e| fail(e));
            let frames =
                analysis::analyze(&audio.samples, audio.rate, &analysis::Options::default());
            let step = args
                .options
                .get("step")
                .and_then(|s| s.parse().ok())
                .unwrap_or(10usize);
            for (i, f) in frames.iter().enumerate().step_by(step) {
                println!(
                    "{:7.3}s f0 {:7.2} level {:6.1} ap {:.2} {:.2} {:.2} {:.2}",
                    i as f32 * 0.005,
                    f.f0,
                    f.level(),
                    f.ap[0],
                    f.ap[2],
                    f.ap[4],
                    f.ap[6]
                );
            }
        }
        _ => {
            eprintln!("usage: sad-voice import|info|render|demo|analyze ...");
            std::process::exit(2);
        }
    }
}
