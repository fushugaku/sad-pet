//! The listening set: fixed phrases in each language (`voice/listening.json`)
//! sung by every voicebank in a folder and by the synthetic voice, measured,
//! and laid out on one page with players beside an earlier render of the same
//! set. Numbers help, but the rebuild benchmark has misled before: the page
//! is for listening.
use sad_pet_voice::analysis;
use sad_pet_voice::bank::Voicebank;
use sad_pet_voice::phoneme::Language;
use sad_pet_voice::render::{self, Voice};
use sad_pet_voice::score::{Note, Score, TICKS_PER_BEAT, Tempo};
use sad_pet_voice::spectrum::grid_position;
use sad_pet_voice::wav;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::path::Path;
use std::sync::Arc;

/// One phrase: notes as lyric, MIDI key and length in eighths.
#[derive(Deserialize)]
pub struct Phrase {
    pub name: String,
    pub language: Language,
    pub title: String,
    pub bpm: f32,
    pub notes: Vec<(String, u8, u32)>,
}

pub fn phrases() -> Vec<Phrase> {
    serde_json::from_str(include_str!("../listening.json")).expect("voice/listening.json")
}

impl Phrase {
    pub fn score(&self) -> Score {
        let mut score = Score {
            language: self.language,
            tempo: vec![Tempo {
                tick: 0,
                bpm: self.bpm,
            }],
            ..Default::default()
        };
        let eighth = TICKS_PER_BEAT / 2;
        let mut tick = TICKS_PER_BEAT * 2;
        for (lyric, key, eighths) in &self.notes {
            score.notes.push(Note {
                id: score.notes.len() as u32 + 1,
                start: tick,
                length: eighths * eighth,
                key: *key,
                lyric: lyric.clone(),
                ..Default::default()
            });
            tick += eighths * eighth;
        }
        score.sanitize();
        score
    }
}

/// What can be measured of a render without listening to it.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Metrics {
    /// Share of the sound that is unvoiced (hiss), and how loud its loudest
    /// tenth is next to the vowels' median (dB).
    pub hiss_share: f32,
    pub hiss_db: f32,
    /// Harmonics-to-noise ratio of voiced frames (dB), full band and above
    /// about 2 kHz.
    pub hnr: f32,
    pub hnr_high: f32,
    /// How far vowel envelopes swing around their tilt, 300 Hz – 4 kHz (dB):
    /// blurred formants swing less.
    pub contrast: f32,
    /// The 2–5 kHz band against 0.2–1 kHz (dB).
    pub brightness: f32,
}

pub fn measure(samples: &[f32], rate: u32) -> Metrics {
    let frames = analysis::analyze(samples, rate, &analysis::Options::default());
    let loudest = frames.iter().map(|f| f.level()).fold(f32::MIN, f32::max);
    let high: Vec<f32> = (0..samples.len())
        .map(|i| {
            let at = |k: usize| samples.get(i.wrapping_sub(k)).copied().unwrap_or(0.0);
            at(0) - 2.0 * at(1) + at(2)
        })
        .collect();
    let rate_f = rate as f32;
    // Normalized autocorrelation at the period, as a harmonics-to-noise ratio.
    let ratio = |s: &[f32], center: usize, period: f32| {
        let n = (0.04 * rate_f) as usize;
        let start = center.saturating_sub(n / 2);
        let mut best = 0.0f32;
        for lag in ((period * 0.9) as usize).max(1)..=(period * 1.1).ceil() as usize {
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
    let (mut full, mut top, mut count) = (0.0, 0.0, 0);
    for (i, f) in frames.iter().enumerate().step_by(4) {
        if f.level() < loudest - 30.0 || !f.voiced() {
            continue;
        }
        let center = (i as f32 * 0.005 * rate_f) as usize;
        let period = rate_f / f.f0;
        if let (Some(a), Some(b)) = (ratio(samples, center, period), ratio(&high, center, period)) {
            full += a;
            top += b;
            count += 1;
        }
    }
    let (lo, hi) = (
        grid_position(300.0) as usize,
        grid_position(4000.0) as usize,
    );
    let band = |env: &[f32], a: f32, b: f32| {
        let (a, b) = (grid_position(a) as usize, grid_position(b) as usize);
        env[a..=b].iter().sum::<f32>() / (b - a + 1) as f32
    };
    let (mut contrast, mut brightness, mut shaped) = (0.0f64, 0.0f64, 0usize);
    for f in frames
        .iter()
        .filter(|f| f.voiced() && f.level() > loudest - 20.0)
    {
        // Spread of the envelope around a straight line through it.
        let y = &f.env[lo..=hi];
        let m = y.len() as f64;
        let sx: f64 = (0..y.len()).map(|i| i as f64).sum();
        let sy: f64 = y.iter().map(|&v| v as f64).sum();
        let sxx: f64 = (0..y.len()).map(|i| (i * i) as f64).sum();
        let sxy: f64 = y
            .iter()
            .enumerate()
            .map(|(i, &v)| i as f64 * v as f64)
            .sum();
        let slope = (m * sxy - sx * sy) / (m * sxx - sx * sx);
        let offset = (sy - slope * sx) / m;
        let spread: f64 = y
            .iter()
            .enumerate()
            .map(|(i, &v)| (v as f64 - offset - slope * i as f64).powi(2))
            .sum::<f64>()
            / m;
        contrast += spread.sqrt();
        brightness += (band(&f.env, 2000.0, 5000.0) - band(&f.env, 200.0, 1000.0)) as f64;
        shaped += 1;
    }
    let active: Vec<_> = frames
        .iter()
        .filter(|f| f.level() > loudest - 40.0)
        .collect();
    let mut vowels: Vec<f32> = active
        .iter()
        .filter(|f| f.voiced())
        .map(|f| f.level())
        .collect();
    vowels.sort_by(f32::total_cmp);
    let mut hiss: Vec<f32> = active
        .iter()
        .filter(|f| !f.voiced())
        .map(|f| f.level())
        .collect();
    hiss.sort_by(f32::total_cmp);
    let median = vowels.get(vowels.len() / 2).copied().unwrap_or(f32::NAN);
    Metrics {
        hiss_share: hiss.len() as f32 / active.len().max(1) as f32,
        hiss_db: hiss.get(hiss.len() * 9 / 10).copied().unwrap_or(f32::NAN) - median,
        hnr: full / count.max(1) as f32,
        hnr_high: top / count.max(1) as f32,
        contrast: (contrast / shaped.max(1) as f64) as f32,
        brightness: (brightness / shaped.max(1) as f64) as f32,
    }
}

/// One render of the set: a phrase sung by a voice.
#[derive(Serialize, Deserialize)]
struct Take {
    phrase: String,
    voice: String,
    file: String,
    metrics: Metrics,
    /// Per note: its lyric and the units that sing it.
    units: Vec<(String, String)>,
}

/// Renders the set into `dir/after`, writes each phrase's score to
/// `dir/scores` (so another build of the engine can render the same set),
/// copies `before/after` into `dir/before` and writes `dir/index.html`.
pub fn run(dir: &Path, before: Option<&Path>, packs: &Path, rate: u32, label: &str) {
    fn fail(e: String) -> ! {
        eprintln!("sad-voice: {e}");
        std::process::exit(1)
    }
    for sub in ["after", "scores", "before"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap_or_else(|e| fail(e.to_string()));
    }
    // Every voicebank of the folder, by file name, with its language.
    let mut banks: Vec<(String, Arc<Voicebank>)> = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir(packs)
        .map(|d| d.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    files.sort();
    for path in files
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "sadvoice"))
    {
        let bytes = std::fs::read(path).unwrap_or_else(|e| fail(e.to_string()));
        let bank = Voicebank::from_bytes(&bytes).unwrap_or_else(|e| fail(e));
        let id = path.file_stem().unwrap().to_string_lossy().to_string();
        banks.push((id, Arc::new(bank)));
    }
    let mut takes = Vec::new();
    for phrase in phrases() {
        let score = phrase.score();
        std::fs::write(
            dir.join("scores").join(format!("{}.json", phrase.name)),
            serde_json::to_vec_pretty(&score).unwrap(),
        )
        .unwrap_or_else(|e| fail(e.to_string()));
        let mut voices: Vec<(String, Voice)> = banks
            .iter()
            .filter(|(_, b)| b.info.language == phrase.language)
            .map(|(id, b)| (id.clone(), Voice::Bank(b.clone())))
            .collect();
        voices.push(("synthetic".into(), Voice::Formant));
        for (id, voice) in voices {
            let samples = render::render_song(&score, &voice, &id, rate, None);
            let file = format!("{}-{id}.wav", phrase.name);
            std::fs::write(
                dir.join("after").join(&file),
                wav::write_pcm16(rate, &samples),
            )
            .unwrap_or_else(|e| fail(e.to_string()));
            let timeline = score.timeline(None);
            let units = render::plan(&score, &timeline, &id, rate, None)
                .iter()
                .flat_map(|p| render::units(&score, &timeline, p, &voice))
                .map(|(n, u)| (score.notes[n].lyric.clone(), u.used.join(" + ")))
                .collect();
            eprintln!("{file}");
            takes.push(Take {
                phrase: phrase.name.clone(),
                voice: id,
                file,
                metrics: measure(&samples, rate),
                units,
            });
        }
    }
    std::fs::write(
        dir.join("listening.json"),
        serde_json::to_vec_pretty(&takes).unwrap(),
    )
    .unwrap_or_else(|e| fail(e.to_string()));
    // The earlier render, measured the same way.
    let mut earlier = std::collections::HashMap::new();
    if let Some(before) = before {
        for take in &takes {
            let from = before.join("after").join(&take.file);
            let Ok(bytes) = std::fs::read(&from) else {
                continue;
            };
            let Ok(audio) = wav::read(&bytes) else {
                continue;
            };
            std::fs::write(dir.join("before").join(&take.file), &bytes)
                .unwrap_or_else(|e| fail(e.to_string()));
            earlier.insert(take.file.clone(), measure(&audio.samples, audio.rate));
        }
    }
    let html = page(&takes, &earlier, label, before.is_some());
    std::fs::write(dir.join("index.html"), html).unwrap_or_else(|e| fail(e.to_string()));
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn page(
    takes: &[Take],
    earlier: &std::collections::HashMap<String, Metrics>,
    label: &str,
    compared: bool,
) -> String {
    let mut body = String::new();
    let languages = [
        (Language::Ru, "Русский"),
        (Language::Ja, "日本語"),
        (Language::En, "English"),
    ];
    for (language, heading) in languages {
        let set: Vec<Phrase> = phrases()
            .into_iter()
            .filter(|p| p.language == language)
            .collect();
        if set.is_empty() {
            continue;
        }
        let _ = write!(body, "<section><h2>{heading}</h2>");
        for phrase in set {
            let lyrics: Vec<&str> = phrase.notes.iter().map(|n| n.0.as_str()).collect();
            let _ = write!(
                body,
                "<article><h3>{}</h3><p class=\"lyrics\">{}</p>",
                escape(&phrase.title),
                escape(&lyrics.join(" "))
            );
            for take in takes.iter().filter(|t| t.phrase == phrase.name) {
                let m = take.metrics;
                let before = earlier.get(&take.file);
                let cell = |name: &str, value: f32, old: Option<f32>, unit: &str| {
                    let delta = old
                        .filter(|o| o.is_finite() && value.is_finite())
                        .map(|o| format!("<span class=\"delta\">{:+.1}</span>", value - o))
                        .unwrap_or_default();
                    format!("<div><dt>{name}</dt><dd>{value:.1}{unit} {delta}</dd></div>")
                };
                let players = if compared {
                    let old = if before.is_some() {
                        format!(
                            "<audio controls preload=\"none\" src=\"before/{}\"></audio>",
                            escape(&take.file)
                        )
                    } else {
                        "<span class=\"missing\">no earlier render</span>".into()
                    };
                    format!(
                        "<div class=\"player\"><span class=\"tag\">before</span>{old}</div>\
                         <div class=\"player\"><span class=\"tag\">after</span>\
                         <audio controls preload=\"none\" src=\"after/{}\"></audio></div>",
                        escape(&take.file)
                    )
                } else {
                    format!(
                        "<div class=\"player\"><audio controls preload=\"none\" src=\"after/{}\"></audio></div>",
                        escape(&take.file)
                    )
                };
                let units: Vec<String> = take
                    .units
                    .iter()
                    .map(|(lyric, units)| {
                        format!("<span><b>{}</b> {}</span>", escape(lyric), escape(units))
                    })
                    .collect();
                let _ = write!(
                    body,
                    "<div class=\"take\"><div class=\"voice\">{}</div><div class=\"players\">{players}</div>\
                     <dl class=\"metrics\">{}{}{}{}{}</dl>{}</div>",
                    escape(&take.voice),
                    cell(
                        "hiss",
                        m.hiss_share * 100.0,
                        before.map(|b| b.hiss_share * 100.0),
                        " %"
                    ),
                    cell("loudest hiss", m.hiss_db, before.map(|b| b.hiss_db), " dB"),
                    cell(
                        "formant contrast",
                        m.contrast,
                        before.map(|b| b.contrast),
                        " dB"
                    ),
                    cell("2–5 kHz", m.brightness, before.map(|b| b.brightness), " dB"),
                    cell("HNR", m.hnr, before.map(|b| b.hnr), " dB"),
                    if units.is_empty() {
                        String::new()
                    } else {
                        format!("<p class=\"units\">{}</p>", units.join(""))
                    }
                );
            }
            body.push_str("</article>");
        }
        body.push_str("</section>");
    }
    format!(
        r#"<title>Vocal Listening Set</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+Condensed:wght@500;600&family=IBM+Plex+Sans:wght@400;500&family=IBM+Plex+Mono:wght@400&display=swap">
<style>
/* One column of phrases; each take a row: voice, players, measurements, units. */
:root {{
  --bg: #f3f5f0; --panel: #ffffff; --fg: #1c211b; --muted: #5d685b; --line: #d9ded4; --accent: #3f7a1f; --delta: #6b5a12;
  --display: "IBM Plex Sans Condensed", "Arial Narrow", sans-serif;
  --body: "IBM Plex Sans", system-ui, sans-serif;
  --data: "IBM Plex Mono", ui-monospace, monospace;
}}
@media (prefers-color-scheme: dark) {{ :root:not([data-theme="light"]) {{
  --bg: #121612; --panel: #191e18; --fg: #e3e9df; --muted: #8f9a8b; --line: #2b3329; --accent: #b6e07f; --delta: #e2c96a; color-scheme: dark; }} }}
:root[data-theme="dark"] {{ --bg: #121612; --panel: #191e18; --fg: #e3e9df; --muted: #8f9a8b; --line: #2b3329; --accent: #b6e07f; --delta: #e2c96a; color-scheme: dark; }}
body {{ background: var(--bg); color: var(--fg); font: 15px/1.5 var(--body); }}
main {{ max-width: 1080px; margin: 0 auto; padding-block: 28px 64px; padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); display: grid; gap: 32px; }}
header {{ display: grid; gap: 8px; }}
h1, h2, h3 {{ font-family: var(--display); text-wrap: balance; margin: 0; }}
h1 {{ font-size: 30px; font-weight: 600; }}
h2 {{ font-size: 13px; letter-spacing: 0.12em; text-transform: uppercase; color: var(--muted); }}
h3 {{ font-size: 19px; font-weight: 500; }}
header p, .lyrics {{ margin: 0; color: var(--muted); max-width: 65ch; }}
.legend {{ font-size: 13px; }}
section {{ display: grid; gap: 14px; }}
article {{ background: var(--panel); border: 1px solid var(--line); border-radius: 10px; padding: 16px; display: grid; gap: 10px; }}
.take {{ display: grid; grid-template-columns: 130px minmax(0, 1fr); gap: 6px 16px; padding-top: 10px; border-top: 1px solid var(--line); }}
.voice {{ font-family: var(--data); font-size: 13px; color: var(--accent); }}
.players {{ display: flex; flex-wrap: wrap; gap: 8px 16px; min-width: 0; }}
.player {{ display: flex; align-items: center; gap: 8px; min-width: 0; }}
.player audio {{ width: min(320px, 100%); height: 32px; }}
.tag {{ font-size: 11px; letter-spacing: 0.08em; text-transform: uppercase; color: var(--muted); width: 44px; }}
.missing {{ font-size: 13px; color: var(--muted); }}
.metrics {{ grid-column: 2; display: flex; flex-wrap: wrap; gap: 4px 18px; margin: 0; font-family: var(--data); font-size: 12px; font-variant-numeric: tabular-nums; }}
.metrics div {{ display: flex; gap: 6px; }}
.metrics dt {{ color: var(--muted); }}
.metrics dd {{ margin: 0; }}
.delta {{ color: var(--delta); }}
.units {{ grid-column: 2; margin: 0; display: flex; flex-wrap: wrap; gap: 2px 14px; font-family: var(--data); font-size: 12px; color: var(--muted); min-width: 0; }}
.units b {{ color: var(--fg); font-weight: 400; }}
@media (max-width: 640px) {{ .take {{ grid-template-columns: minmax(0, 1fr); }} .metrics, .units {{ grid-column: 1; }} }}
</style>
<main>
<header>
<h1>Vocal Listening Set</h1>
<p>{label}. Each phrase is sung by the bundled voicebank for its language and by the synthetic voice{compare}.</p>
<p class="legend">In real recordings of these voicebanks: hiss 8–18 % of the sound, its loudest tenth about −21 dB from the vowels, formant contrast about 8 dB, 2–5 kHz about −33 dB. Under each take, the units every syllable is joined from.</p>
</header>
{body}
</main>"#,
        label = escape(label),
        compare = if compared {
            ", beside the earlier render (before) with the change in each measurement"
        } else {
            ""
        },
        body = body
    )
}
