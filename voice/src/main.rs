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
use sad_pet_voice::score::{Note, Score};
use sad_pet_voice::{analysis, utau, wav};
use std::path::PathBuf;

// The listening set and its measures (command line only).
mod listen;
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
    // The first phrase of the listening set in that language.
    listen::phrases()
        .into_iter()
        .find(|p| p.language == language)
        .map(|p| p.score())
        .unwrap_or_default()
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

/// Leave-one-out test of concatenation: sings the syllables of one
/// recording with every other unit of the bank, at the recording's own
/// timing and pitch, and compares the spectral envelopes frame by frame.
///
///     sad-voice reconstruct <utau folder> <wav relative to it> [--language ja]
fn reconstruct(args: &Args) {
    use sad_pet_voice::spectrum::{FRAME_SECONDS, grid_position};
    let dir = PathBuf::from(
        args.positional
            .get(1)
            .unwrap_or_else(|| fail("missing voicebank folder")),
    );
    let wav_path = args
        .positional
        .get(2)
        .unwrap_or_else(|| fail("missing WAV"));
    let language = args
        .options
        .get("language")
        .and_then(|l| Language::parse(l))
        .unwrap_or(Language::Ja);
    let folder = wav_path.rsplit_once('/').map_or("", |(d, _)| d);
    let oto = std::fs::read(dir.join(folder).join("oto.ini")).unwrap_or_else(|e| fail(e));
    let name = wav_path.rsplit('/').next().unwrap_or(wav_path);
    let entries: Vec<_> = utau::parse_oto(&utau::decode_text(&oto), "")
        .into_iter()
        .filter(|e| e.file == name)
        .collect();
    // Syllable onsets: where each entry's final vowel starts.
    // Syllables as explicit phonemes (onset consonants and the final vowel
    // of each alias), so any alias notation reconstructs exactly.
    let affixes = ["A3", "D4", "A4", "D5", "↑", "↓"];
    let spell = |alias: &str| {
        let alias = affixes.iter().fold(alias.to_string(), |a, s| {
            a.strip_suffix(s).map_or(a.clone(), str::to_string)
        });
        sad_pet_voice::g2p::alias(language, &alias)
    };
    let symbols =
        |p: &[sad_pet_voice::Phoneme]| p.iter().map(|p| p.symbol()).collect::<Vec<_>>().join(" ");
    let mut onsets: Vec<(f64, String)> = Vec::new();
    for e in &entries {
        let Some(phonemes) = spell(&e.alias) else {
            continue;
        };
        if !phonemes.last().is_some_and(|p| p.is_nucleus()) {
            continue;
        }
        let vowel = phonemes.len() - 1;
        let first = (0..vowel)
            .rev()
            .find(|&i| phonemes[i].is_vowel() || phonemes[i].is_pause())
            .map_or(0, |i| i + 1);
        let at = (e.offset + e.preutter) as f64 / 1000.0;
        if !onsets.iter().any(|(t, _)| (t - at).abs() < 0.05) {
            onsets.push((at, symbols(&phonemes[first..])));
        }
    }
    onsets.sort_by(|a, b| a.0.total_cmp(&b.0));
    if onsets.is_empty() {
        fail("no syllables found for that recording");
    }
    // A VCV recording starts on a vowel that no alias ends with: sing it too.
    if let Some(first) = entries
        .iter()
        .filter(|e| e.alias.split_whitespace().count() == 2)
        .min_by(|a, b| a.offset.total_cmp(&b.offset))
    {
        let lead = first.alias.split_whitespace().next().unwrap_or("");
        let phonemes = spell(lead).unwrap_or_default();
        let at = first.offset as f64 / 1000.0;
        if phonemes.len() == 1 && phonemes[0].is_vowel() && onsets[0].0 - at > 0.1 {
            onsets.insert(0, (at, symbols(&phonemes)));
        }
    }
    // One phrase: banks with several pitches in one file repeat it later.
    if let Some(gap) = onsets.windows(2).position(|w| w[1].0 - w[0].0 > 0.9) {
        onsets.truncate(gap + 1);
    }
    let audio = wav::read(&std::fs::read(dir.join(wav_path)).unwrap_or_else(|e| fail(e)))
        .unwrap_or_else(|e| fail(e));
    let original = analysis::analyze(&audio.samples, audio.rate, &analysis::Options::default());
    let end_of = |i: usize| onsets.get(i + 1).map_or(onsets[i].0 + 0.45, |n| n.0);
    let mut score = Score {
        language,
        tempo: vec![sad_pet_voice::score::Tempo {
            tick: 0,
            bpm: 125.0,
        }],
        follow_host: false,
        ..Default::default()
    };
    // 125 BPM: one tick is exactly 1 ms.
    let style = sad_pet_voice::score::Style::presets()
        .into_iter()
        .find(|s| s.robot)
        .unwrap();
    score.style = sad_pet_voice::score::Style {
        auto_breath: false,
        ..style
    };
    for (i, (at, lyric)) in onsets.iter().enumerate() {
        let a = (at / FRAME_SECONDS as f64) as usize;
        let b = ((end_of(i) / FRAME_SECONDS as f64) as usize).min(original.len());
        let mut pitches: Vec<f32> = original[a..b.max(a + 1).min(original.len())]
            .iter()
            .filter(|f| f.voiced())
            .map(|f| 69.0 + 12.0 * (f.f0 / 440.0).log2())
            .collect();
        pitches.sort_by(f32::total_cmp);
        let key = pitches
            .get(pitches.len() / 2)
            .copied()
            .unwrap_or(60.0)
            .round() as u8;
        score.notes.push(Note {
            id: i as u32 + 1,
            start: (at * 1000.0) as u32,
            length: ((end_of(i) - at) * 1000.0) as u32,
            key,
            phonemes: Some(lyric.clone()),
            ..Default::default()
        });
    }
    score.sanitize();
    // The whole bank (imported once, then cached) without this recording's units.
    let mut bank = match args.options.get("bank") {
        Some(path) if std::path::Path::new(path).exists() => {
            Voicebank::from_bytes(&std::fs::read(path).unwrap_or_else(|e| fail(e)))
                .unwrap_or_else(|e| fail(e))
        }
        cache => {
            let options = utau::ImportOptions {
                language: Some(language),
                name: None,
            };
            let bank = utau::import(&utau::Directory(dir.clone()), &options, &mut |_, _| {})
                .unwrap_or_else(|e| fail(e));
            if let Some(path) = cache {
                std::fs::write(path, bank.to_bytes()).unwrap_or_else(|e| fail(e));
            }
            bank
        }
    };
    let before = bank.units.len();
    if !args.options.contains_key("keep") {
        bank.units.retain(|u| u.source != *wav_path);
    }
    if bank.units.len() == before && !args.options.contains_key("keep") {
        fail("no unit of the bank comes from that recording");
    }
    bank.finish();
    let voice = Voice::Bank(Arc::new(bank));
    let rendered = render::render_song(&score, &voice, "loo", audio.rate, None);
    if let Some(out) = args.options.get("o") {
        std::fs::write(out, wav::write_float(audio.rate, &rendered)).unwrap_or_else(|e| fail(e));
    }
    let synthesized = analysis::analyze(&rendered, audio.rate, &analysis::Options::default());
    let (lo, hi) = (
        grid_position(150.0) as usize,
        grid_position(6000.0) as usize,
    );
    let shape = |f: &sad_pet_voice::spectrum::Frame| {
        let mean = f.env[lo..=hi].iter().sum::<f32>() / (hi - lo + 1) as f32;
        f.env[lo..=hi]
            .iter()
            .map(|v| v - mean)
            .collect::<Vec<f32>>()
    };
    let (first, last) = (onsets[0].0, end_of(onsets.len() - 1));
    let (mut steady, mut moving) = ((0.0f64, 0usize), (0.0f64, 0usize));
    for i in 0..original.len().min(synthesized.len()) {
        let t = i as f64 * FRAME_SECONDS as f64;
        if t < first || t > last || !original[i].voiced() || !synthesized[i].voiced() {
            continue;
        }
        let (x, y) = (shape(&original[i]), shape(&synthesized[i]));
        let d = x.iter().zip(&y).map(|(a, b)| (a - b).abs()).sum::<f32>() as f64 / x.len() as f64;
        let near = onsets.iter().any(|(o, _)| (t - o).abs() < 0.06);
        let bucket = if near { &mut moving } else { &mut steady };
        bucket.0 += d;
        bucket.1 += 1;
    }
    let mean = |b: (f64, usize)| b.0 / b.1.max(1) as f64;
    println!(
        "{}: {} syllables, envelope distance {:.2} dB (steady {:.2} dB over {} frames, transitions {:.2} dB over {} frames)",
        wav_path,
        onsets.len(),
        (steady.0 + moving.0) / (steady.1 + moving.1).max(1) as f64,
        mean(steady),
        steady.1,
        mean(moving),
        moving.1
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
            let mut progress = |done, total| eprint!("\ranalyzing {done}/{total}");
            let (bank, report) = if dir.to_lowercase().ends_with(".zip") {
                let bytes = std::fs::read(dir).unwrap_or_else(|e| fail(e));
                let files = sad_pet_voice::zip::unzip(&bytes).unwrap_or_else(|e| fail(e));
                utau::import_with_report(&files, &options, &mut progress)
            } else {
                utau::import_with_report(
                    &utau::Directory(PathBuf::from(dir)),
                    &options,
                    &mut progress,
                )
            }
            .unwrap_or_else(|e| fail(e));
            eprintln!();
            eprintln!(
                "{} of {} oto entries became units; {} unread{}",
                report.units,
                report.entries,
                report.unread_count,
                if report.unread.is_empty() {
                    String::new()
                } else {
                    format!(": {}", report.unread.join(" | "))
                }
            );
            if !report.missing_files.is_empty() {
                eprintln!(
                    "missing or unreadable WAVs: {}",
                    report.missing_files.join(", ")
                );
            }
            let mut bank = bank;
            if let Some(author) = args.options.get("author") {
                bank.info.author = author.clone();
            }
            if let Some(description) = args.options.get("description") {
                bank.info.description = description.clone();
            }
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
            println!(
                "{} units, {} frames, median vowel level {:.1} dB",
                bank.units.len(),
                bank.frame_count(),
                bank.median_level()
            );
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
            if args.options.contains_key("units") {
                // Which voicebank units each phrase joins, with its syllables.
                let timeline = score.timeline(None);
                for phrase in render::plan(&score, &timeline, &id, rate, None) {
                    let units: Vec<String> = render::units(&score, &timeline, &phrase, &voice)
                        .into_iter()
                        .map(|(n, u)| {
                            let others: Vec<&str> =
                                u.options.iter().take(5).map(String::as_str).collect();
                            format!(
                                "{} = {} (or {})",
                                score.notes[n].lyric,
                                u.used.join(" + "),
                                others.join(", ")
                            )
                        })
                        .collect();
                    eprintln!("{}", units.join(" | "));
                }
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
        "reconstruct" => reconstruct(&args),
        "hnr" => {
            // Noise, harmonics and formant measures of renders or recordings.
            for path in &args.positional[1..] {
                let audio = wav::read(&std::fs::read(path).unwrap_or_else(|e| fail(e)))
                    .unwrap_or_else(|e| fail(e));
                let m = listen::measure(&audio.samples, audio.rate);
                println!(
                    "{path}: hiss {:.0}% (loudest {:+.1} dB from the vowels), HNR {:.1} dB \
                     (above 2 kHz {:.1}), formant contrast {:.2} dB, 2-5 kHz {:+.1} dB",
                    m.hiss_share * 100.0,
                    m.hiss_db,
                    m.hnr,
                    m.hnr_high,
                    m.contrast,
                    m.brightness
                );
            }
        }
        "listen" => {
            let dir = args
                .positional
                .get(1)
                .unwrap_or_else(|| fail("missing output folder"));
            let packs = args
                .options
                .get("packs")
                .map_or("voice/packs", String::as_str);
            let rate = args
                .options
                .get("rate")
                .and_then(|r| r.parse().ok())
                .unwrap_or(32_000);
            listen::run(
                std::path::Path::new(dir),
                args.options.get("before").map(std::path::Path::new),
                std::path::Path::new(packs),
                rate,
                args.options
                    .get("label")
                    .map_or("This build", String::as_str),
            );
        }
        "resynth" => {
            // Analysis and resynthesis of a recording at its own pitch, with
            // the log-spectral distance to the original as a fidelity measure.
            let path = args
                .positional
                .get(1)
                .unwrap_or_else(|| fail("missing WAV"));
            let audio = wav::read(&std::fs::read(path).unwrap_or_else(|e| fail(e)))
                .unwrap_or_else(|e| fail(e));
            let frames =
                analysis::analyze(&audio.samples, audio.rate, &analysis::Options::default());
            let mut vocoder = sad_pet_voice::vocoder::Vocoder::new(audio.rate);
            let jitter: f32 = args
                .options
                .get("jitter")
                .and_then(|j| j.parse().ok())
                .unwrap_or(0.0);
            let pulses = vec![sad_pet_voice::vocoder::Pulse { growl: 0.0, jitter }; frames.len()];
            let output = vocoder.render(&frames, &pulses, 1);
            let (lsd, voiced_lsd, level) =
                sad_pet_voice::analysis::spectral_distance(&audio.samples, &output, audio.rate);
            println!("{path}: LSD {lsd:.2} dB (voiced {voiced_lsd:.2} dB), level {level:+.2} dB");
            if let Some(out) = args.options.get("o") {
                std::fs::write(out, wav::write_float(audio.rate, &output))
                    .unwrap_or_else(|e| fail(e));
            }
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
            eprintln!(
                "usage: sad-voice import|info|render|demo|analyze|resynth|reconstruct|hnr|listen ..."
            );
            std::process::exit(2);
        }
    }
}
