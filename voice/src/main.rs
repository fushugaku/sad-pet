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
    // A well-known tune in each language, one syllable per note: lyric, key
    // and length in eighths (0: an eighth's rest).
    let (bpm, tune): (f32, &[(&str, u8, u32)]) = match language {
        Language::Ja => (
            // さくら さくら
            100.0,
            &[
                ("さ", 69, 2),
                ("く", 69, 2),
                ("ら", 71, 4),
                ("さ", 69, 2),
                ("く", 69, 2),
                ("ら", 71, 4),
                ("や", 69, 2),
                ("よ", 71, 2),
                ("い", 72, 2),
                ("の", 71, 2),
                ("そ", 69, 2),
                ("ら", 71, 1),
                ("", 69, 1),
                ("は", 65, 4),
            ],
        ),
        Language::En => (
            // Twinkle, Twinkle, Little Star
            110.0,
            &[
                ("twin-", 60, 2),
                ("kle", 60, 2),
                ("twin-", 67, 2),
                ("kle", 67, 2),
                ("lit-", 69, 2),
                ("tle", 69, 2),
                ("star", 67, 4),
                ("how", 65, 2),
                ("I", 65, 2),
                ("won-", 64, 2),
                ("der", 64, 2),
                ("what", 62, 2),
                ("you", 62, 2),
                ("are", 60, 4),
            ],
        ),
        Language::Ru => (
            // В лесу родилась ёлочка
            100.0,
            &[
                ("в ле-", 60, 1),
                ("су", 69, 1),
                ("ро-", 69, 1),
                ("ди-", 67, 1),
                ("лась", 69, 1),
                ("ё-", 65, 1),
                ("лоч-", 60, 1),
                ("ка", 60, 1),
                ("в ле-", 60, 1),
                ("су", 69, 1),
                ("о-", 69, 1),
                ("на", 70, 1),
                ("рос-", 67, 1),
                ("ла", 72, 3),
            ],
        ),
    };
    let mut score = Score {
        language,
        tempo: vec![sad_pet_voice::score::Tempo { tick: 0, bpm }],
        ..Default::default()
    };
    let eighth = TICKS_PER_BEAT / 2;
    let mut tick = TICKS_PER_BEAT * 2;
    for &(lyric, key, eighths) in tune {
        if eighths == 0 {
            tick += eighth;
            continue;
        }
        score.notes.push(Note {
            id: score.notes.len() as u32 + 1,
            start: tick,
            length: eighths * eighth,
            key,
            // "" holds the vowel of the note before on a new pitch.
            lyric: if lyric.is_empty() { "-" } else { lyric }.to_string(),
            ..Default::default()
        });
        tick += eighths * eighth;
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
                    let mut units = Vec::new();
                    render::render_with_units(
                        &score,
                        &timeline,
                        &phrase,
                        &voice,
                        rate,
                        Some(&mut units),
                    );
                    let lyrics: Vec<&str> = phrase
                        .notes
                        .iter()
                        .map(|&n| score.notes[n].lyric.as_str())
                        .collect();
                    eprintln!("{}: {}", lyrics.join(" "), units.join(" | "));
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
            // Harmonics-to-noise ratio of the voiced frames, from the
            // normalized autocorrelation at the period (independent of the
            // analysis' aperiodicity), full band and above ~2 kHz.
            for path in &args.positional[1..] {
                let audio = wav::read(&std::fs::read(path).unwrap_or_else(|e| fail(e)))
                    .unwrap_or_else(|e| fail(e));
                let frames =
                    analysis::analyze(&audio.samples, audio.rate, &analysis::Options::default());
                let x = &audio.samples;
                let high: Vec<f32> = (0..x.len())
                    .map(|i| {
                        let at = |k: usize| x.get(i.wrapping_sub(k)).copied().unwrap_or(0.0);
                        at(0) - 2.0 * at(1) + at(2)
                    })
                    .collect();
                let rate = audio.rate as f32;
                let ratio = |s: &[f32], center: usize, period: f32| {
                    let n = (0.04 * rate) as usize;
                    let start = center.saturating_sub(n / 2);
                    let mut best = 0.0f32;
                    let lo = (period * 0.9) as usize;
                    let hi = (period * 1.1).ceil() as usize;
                    for lag in lo.max(1)..=hi {
                        if start + n + lag >= s.len() {
                            return None;
                        }
                        let (mut xy, mut xx, mut yy) = (0.0f64, 0.0f64, 0.0f64);
                        for i in start..start + n {
                            let (a, b) = (s[i] as f64, s[i + lag] as f64);
                            xy += a * b;
                            xx += a * a;
                            yy += b * b;
                        }
                        best = best.max((xy / (xx * yy).sqrt().max(1e-12)) as f32);
                    }
                    let r = best.clamp(1e-3, 0.9999);
                    Some(10.0 * (r / (1.0 - r)).log10())
                };
                let loudest = frames.iter().map(|f| f.level()).fold(f32::MIN, f32::max);
                let (mut full, mut top, mut count, mut voiced, mut loud) = (0.0, 0.0, 0, 0, 0);
                for (i, f) in frames.iter().enumerate().step_by(4) {
                    if f.level() < loudest - 30.0 {
                        continue;
                    }
                    loud += 1;
                    if !f.voiced() {
                        continue;
                    }
                    voiced += 1;
                    let center = (i as f32 * 0.005 * rate) as usize;
                    let period = rate / f.f0;
                    if let (Some(a), Some(b)) =
                        (ratio(x, center, period), ratio(&high, center, period))
                    {
                        full += a;
                        top += b;
                        count += 1;
                    }
                }
                let n = count.max(1) as f32;
                // Formant contrast: how far the envelope swings around its
                // tilt between 300 Hz and 4 kHz (blurred vowels swing less).
                let (lo, hi) = (
                    sad_pet_voice::spectrum::grid_position(300.0) as usize,
                    sad_pet_voice::spectrum::grid_position(4000.0) as usize,
                );
                let mut contrast = (0.0f64, 0usize);
                let mut brightness = (0.0f64, ());
                for f in frames
                    .iter()
                    .filter(|f| f.voiced() && f.level() > loudest - 20.0)
                {
                    let y = &f.env[lo..=hi];
                    let m = y.len() as f64;
                    let (sx, sy) = (
                        (0..y.len()).map(|i| i as f64).sum::<f64>(),
                        y.iter().map(|&v| v as f64).sum::<f64>(),
                    );
                    let sxx = (0..y.len()).map(|i| (i * i) as f64).sum::<f64>();
                    let sxy = y
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| i as f64 * v as f64)
                        .sum::<f64>();
                    let slope = (m * sxy - sx * sy) / (m * sxx - sx * sx);
                    let icpt = (sy - slope * sx) / m;
                    let var = y
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| (v as f64 - icpt - slope * i as f64).powi(2))
                        .sum::<f64>()
                        / m;
                    contrast.0 += var.sqrt();
                    contrast.1 += 1;
                    let band = |a: f32, b: f32| {
                        let (a, b) = (
                            sad_pet_voice::spectrum::grid_position(a) as usize,
                            sad_pet_voice::spectrum::grid_position(b) as usize,
                        );
                        f.env[a..=b].iter().sum::<f32>() / (b - a + 1) as f32
                    };
                    brightness.0 += (band(2000.0, 5000.0) - band(200.0, 1000.0)) as f64;
                }
                // Hiss: how much of the sound is unvoiced, and how loud its
                // loudest tenth is next to the vowels.
                let active: Vec<_> = frames
                    .iter()
                    .filter(|f| f.level() > loudest - 40.0)
                    .collect();
                let median = |mut v: Vec<f32>| {
                    v.sort_by(f32::total_cmp);
                    v.get(v.len() / 2).copied().unwrap_or(f32::NAN)
                };
                let vowels = median(
                    active
                        .iter()
                        .filter(|f| f.voiced())
                        .map(|f| f.level())
                        .collect(),
                );
                let mut hiss: Vec<f32> = active
                    .iter()
                    .filter(|f| !f.voiced())
                    .map(|f| f.level())
                    .collect();
                hiss.sort_by(f32::total_cmp);
                let loudest_hiss = hiss.get(hiss.len() * 9 / 10).copied().unwrap_or(f32::NAN);
                println!(
                    "{path}: hiss {:.0}% of the sound, its loudest tenth {:+.1} dB from the vowels",
                    100.0 * hiss.len() as f32 / active.len().max(1) as f32,
                    loudest_hiss - vowels
                );
                println!(
                    "{path}: HNR {:.1} dB, above 2 kHz {:.1} dB, voiced {:.0}% of loud frames, formant contrast {:.2} dB, 2-5 kHz {:+.1} dB from 0.2-1 kHz",
                    full / n,
                    top / n,
                    100.0 * voiced as f32 / loud.max(1) as f32,
                    contrast.0 / contrast.1.max(1) as f64,
                    brightness.0 / contrast.1.max(1) as f64
                );
            }
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
            eprintln!("usage: sad-voice import|info|render|demo|analyze|resynth|reconstruct ...");
            std::process::exit(2);
        }
    }
}
