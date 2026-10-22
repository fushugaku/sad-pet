//! Imports UTAU voicebanks: every `oto.ini` (in any subfolder, for pitch
//! layers), `character.txt` and `prefix.map`. Each oto line becomes a unit:
//! its alias is spelled into phonemes, the preutterance marks where the
//! final vowel (or, for a VC alias, the consonant) begins, the overlap ends
//! a leading vowel, and the consonant area is kept unstretched.
use crate::analysis::{self, Options};
use crate::bank::{Info, Unit, Voicebank};
use crate::g2p;
use crate::phoneme::{Class, Language, Phoneme};
use crate::spectrum::{FLOOR_DB, FRAME_SECONDS};
use std::collections::BTreeMap;

/// Where the files of a voicebank come from: a folder, or the contents of a
/// zip or of a folder the browser read.
pub trait Files {
    /// Paths relative to the voicebank root, with `/` separators.
    fn paths(&self) -> Vec<String>;
    fn read(&self, path: &str) -> Option<Vec<u8>>;
}

#[derive(Default)]
pub struct MemoryFiles(pub BTreeMap<String, Vec<u8>>);
impl Files for MemoryFiles {
    fn paths(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        self.0.get(path).cloned().or_else(|| {
            // oto.ini names files case-insensitively on Windows banks.
            let lower = path.to_lowercase();
            self.0
                .iter()
                .find(|(k, _)| k.to_lowercase() == lower)
                .map(|(_, v)| v.clone())
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Directory(pub std::path::PathBuf);
#[cfg(not(target_arch = "wasm32"))]
impl Files for Directory {
    fn paths(&self) -> Vec<String> {
        fn walk(root: &std::path::Path, dir: &std::path::Path, out: &mut Vec<String>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(root, &path, out);
                } else if let Ok(relative) = path.strip_prefix(root) {
                    out.push(relative.to_string_lossy().replace('\\', "/"));
                }
            }
        }
        let mut out = Vec::new();
        walk(&self.0, &self.0, &mut out);
        out.sort();
        out
    }
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        let full = self.0.join(path);
        std::fs::read(&full).ok().or_else(|| {
            // Case-insensitive fallback within the folder.
            let parent = full.parent()?;
            let name = full.file_name()?.to_string_lossy().to_lowercase();
            std::fs::read_dir(parent)
                .ok()?
                .flatten()
                .find(|e| e.file_name().to_string_lossy().to_lowercase() == name)
                .and_then(|e| std::fs::read(e.path()).ok())
        })
    }
}

/// UTAU files are Shift-JIS unless they are valid UTF-8 (with or without BOM).
pub fn decode_text(bytes: &[u8]) -> String {
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => encoding_rs::SHIFT_JIS.decode(bytes).0.into_owned(),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OtoEntry {
    /// WAV path relative to the voicebank root.
    pub file: String,
    pub alias: String,
    pub offset: f32,
    pub consonant: f32,
    pub cutoff: f32,
    pub preutter: f32,
    pub overlap: f32,
}

pub fn parse_oto(text: &str, dir: &str) -> Vec<OtoEntry> {
    let prefix = if dir.is_empty() {
        String::new()
    } else {
        format!("{dir}/")
    };
    text.lines()
        .filter_map(|line| {
            let (file, rest) = line.trim().split_once('=')?;
            let mut fields = rest.split(',');
            let alias = fields.next()?.trim().to_string();
            let mut number = || {
                fields
                    .next()
                    .and_then(|v| v.trim().parse::<f32>().ok())
                    .unwrap_or(0.0)
            };
            let (offset, consonant, cutoff, preutter, overlap) =
                (number(), number(), number(), number(), number());
            let file = file.trim();
            let alias = if alias.is_empty() {
                file.rsplit_once('.')
                    .map_or(file, |(stem, _)| stem)
                    .to_string()
            } else {
                alias
            };
            Some(OtoEntry {
                file: format!("{prefix}{file}"),
                alias,
                offset,
                consonant,
                cutoff,
                preutter,
                overlap,
            })
        })
        .collect()
}

#[derive(Clone, Debug, Default)]
pub struct ImportOptions {
    pub language: Option<Language>,
    pub name: Option<String>,
}

fn guess_language(aliases: &[&str]) -> Language {
    let (mut kana, mut cyrillic, mut arpabet) = (0, 0, 0);
    for alias in aliases {
        for c in alias.chars() {
            match c {
                '\u{3040}'..='\u{30FF}' => kana += 1,
                '\u{0400}'..='\u{04FF}' => cyrillic += 1,
                _ => {}
            }
        }
        if alias.split_whitespace().any(|t| {
            matches!(
                t,
                "aa" | "ae" | "ah" | "ao" | "eh" | "ih" | "iy" | "uw" | "hh" | "dh" | "ng"
            )
        }) {
            arpabet += 1;
        }
    }
    if cyrillic > kana && cyrillic > 0 {
        Language::Ru
    } else if kana > 0 {
        Language::Ja
    } else if arpabet > aliases.len() / 10 {
        Language::En
    } else {
        Language::Ja
    }
}

/// Alias spellings to try: the alias itself, then without pitch-layer
/// suffixes or prefixes, take numbers and decorations.
fn alias_candidates(alias: &str, affixes: &[String]) -> Vec<String> {
    let mut out = vec![alias.to_string()];
    for affix in affixes {
        if let Some(s) = alias.strip_suffix(affix.as_str()) {
            out.push(s.to_string());
        }
        if let Some(s) = alias.strip_prefix(affix.as_str()) {
            out.push(s.to_string());
        }
    }
    let trimmed = alias.trim_end_matches(|c: char| c.is_ascii_digit() || "↑↓_+*#!~".contains(c));
    out.push(trimmed.to_string());
    if let Some((head, _)) = alias.rsplit_once('_') {
        out.push(head.to_string());
    }
    out.retain(|s| !s.trim().is_empty());
    out
}

fn phonemes_of(language: Language, alias: &str, affixes: &[String]) -> Option<Vec<Phoneme>> {
    alias_candidates(alias, affixes).iter().find_map(|a| {
        g2p::alias(language, a).filter(|p| !p.is_empty() && p.iter().any(|p| !p.is_pause()))
    })
}

/// Typical lengths in seconds, used to divide a consonant area between
/// several consonants.
pub fn default_duration(p: Phoneme) -> f32 {
    match p.class() {
        Class::Stop => 0.07,
        Class::Affricate => 0.1,
        Class::Fricative => 0.1,
        Class::Nasal => 0.06,
        Class::Liquid => 0.05,
        Class::Glide => 0.05,
        Class::Closure => 0.08,
        Class::Breath => 0.3,
        Class::Silence => 0.1,
        Class::Vowel => 0.15,
    }
}

fn split(region: (f32, f32), phonemes: &[Phoneme]) -> Vec<f32> {
    // Boundaries (excluding the region start) for phonemes sharing a region.
    let weights: Vec<f32> = phonemes.iter().map(|&p| default_duration(p)).collect();
    let total: f32 = weights.iter().sum::<f32>().max(1e-6);
    let mut at = region.0;
    weights
        .iter()
        .map(|w| {
            at += (region.1 - region.0) * w / total;
            at
        })
        .collect()
}

/// Phoneme boundaries in ms for one oto entry, given the file length and a
/// function telling whether a time is audible.
fn boundaries(
    entry: &OtoEntry,
    phonemes: &[Phoneme],
    file_ms: f32,
    audible: &dyn Fn(f32) -> bool,
) -> Option<Vec<f32>> {
    let start = entry.offset.max(0.0);
    let end = if entry.cutoff < 0.0 {
        start - entry.cutoff
    } else {
        file_ms - entry.cutoff
    }
    .min(file_ms);
    let pre = (start + entry.preutter).clamp(start, end);
    let overlap = start + entry.overlap;
    if end - start < 2.0 * FRAME_SECONDS * 1000.0 {
        return None;
    }
    let n = phonemes.len();
    let anchor = if phonemes[n - 1].is_nucleus() {
        n - 1
    } else {
        // A VC alias: the consonant after the leading vowel starts at
        // preutterance; a lone consonant (`-b`) starts there itself.
        phonemes.iter().position(|p| p.is_vowel()).map_or_else(
            || phonemes.iter().position(|p| !p.is_pause()).unwrap_or(0),
            |v| (v + 1).min(n - 1),
        )
    };
    let mut bounds = vec![0.0f32; n + 1];
    bounds[n] = end;
    bounds[anchor] = pre;
    if anchor == 0 {
        bounds[0] = if phonemes[0].is_vowel() { pre } else { start };
    } else {
        let head = &phonemes[..anchor];
        if head[0].is_vowel() && head.len() == 1 {
            // VC: the vowel lasts until the consonant at preutterance.
            bounds[0] = start;
        } else if head[0].is_vowel() {
            let vowel_end = if overlap > start + 5.0 && overlap < pre - 5.0 {
                overlap
            } else {
                start + (pre - start) * 0.5
            };
            bounds[0] = start;
            let inner = split((vowel_end, pre), &head[1..]);
            bounds[1] = vowel_end;
            for (k, b) in inner.iter().enumerate().take(head.len().saturating_sub(2)) {
                bounds[2 + k] = *b;
            }
        } else if head[0].is_pause() {
            // Silence lasts until the sound begins.
            let mut onset = start;
            let mut t = pre - 5.0;
            while t > start {
                if !audible(t) {
                    onset = t;
                    break;
                }
                t -= 5.0;
            }
            // A stop's closure is silent too: leave its natural length.
            let consonants = &head[1..];
            let wanted: f32 = consonants.iter().map(|&p| default_duration(p)).sum::<f32>() * 1000.0;
            let onset = onset.min(pre - wanted.min(pre - start) * 0.5).max(start);
            bounds[0] = start;
            bounds[1] = onset;
            let inner = split((onset, pre), consonants);
            for (k, b) in inner
                .iter()
                .enumerate()
                .take(consonants.len().saturating_sub(1))
            {
                bounds[2 + k] = *b;
            }
        } else {
            bounds[0] = start;
            let inner = split((start, pre), head);
            for (k, b) in inner.iter().enumerate().take(head.len() - 1) {
                bounds[1 + k] = *b;
            }
        }
    }
    if anchor + 1 < n {
        let tail = &phonemes[anchor + 1..];
        let first = pre + (end - pre) * 0.6;
        bounds[anchor + 1] = first;
        let inner = split((first, end), &tail[..tail.len().saturating_sub(1)]);
        for (k, b) in inner.iter().enumerate() {
            bounds[anchor + 2 + k] = *b;
        }
    }
    for i in 1..=n {
        bounds[i] = bounds[i].max(bounds[i - 1]);
    }
    Some(bounds)
}

/// What an import used and left out.
#[derive(Clone, Debug, Default)]
pub struct Report {
    pub entries: usize,
    pub units: usize,
    /// Aliases that were not read as phonemes (some of them).
    pub unread: Vec<String>,
    pub unread_count: usize,
    pub missing_files: Vec<String>,
}

/// Imports a UTAU voicebank. `progress(done, total)` reports analyzed files.
pub fn import(
    files: &dyn Files,
    options: &ImportOptions,
    progress: &mut dyn FnMut(usize, usize),
) -> Result<Voicebank, String> {
    import_with_report(files, options, progress).map(|(bank, _)| bank)
}

pub fn import_with_report(
    files: &dyn Files,
    options: &ImportOptions,
    progress: &mut dyn FnMut(usize, usize),
) -> Result<(Voicebank, Report), String> {
    let mut importer = Importer::new(Box::new(files), options)?;
    while importer.step() {
        progress(importer.done(), importer.total());
    }
    importer.finish()
}

impl<T: Files + ?Sized> Files for &T {
    fn paths(&self) -> Vec<String> {
        (**self).paths()
    }
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        (**self).read(path)
    }
}

/// One recording's oto entries with the phonemes each alias spells.
type Entries = Vec<(OtoEntry, Vec<Phoneme>)>;

/// An import that analyzes one recording per `step`, so a caller can show
/// progress or stay responsive in between.
pub struct Importer<'a> {
    files: Box<dyn Files + 'a>,
    /// Recordings still to analyze, last first.
    queue: Vec<(String, Entries)>,
    total: usize,
    language: Language,
    bank: Voicebank,
    report: Report,
}

impl<'a> Importer<'a> {
    /// Reads every oto.ini, character.txt and prefix.map and spells the aliases.
    pub fn new(files: Box<dyn Files + 'a>, options: &ImportOptions) -> Result<Self, String> {
        let mut report = Report::default();
        let paths = files.paths();
        let mut entries = Vec::new();
        for path in &paths {
            let name = path.rsplit('/').next().unwrap_or(path);
            if !name.eq_ignore_ascii_case("oto.ini") {
                continue;
            }
            let dir = path.rsplit_once('/').map_or("", |(d, _)| d);
            let text = decode_text(&files.read(path).ok_or("oto.ini is unreadable")?);
            entries.extend(parse_oto(&text, dir));
        }
        if entries.is_empty() {
            return Err("No oto.ini with entries was found".into());
        }
        // The shallowest character.txt and prefix.map belong to the bank.
        let root_file = |name: &str| {
            paths
                .iter()
                .filter(|p| {
                    p.rsplit('/')
                        .next()
                        .is_some_and(|n| n.eq_ignore_ascii_case(name))
                })
                .min_by_key(|p| p.matches('/').count())
                .and_then(|p| files.read(p))
                .map(|b| decode_text(&b))
        };
        let mut name = options.name.clone().unwrap_or_default();
        let mut author = String::new();
        if let Some(character) = root_file("character.txt") {
            for line in character.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    match key.trim().to_lowercase().as_str() {
                        "name" if name.is_empty() => name = value.trim().to_string(),
                        "author" => author = value.trim().to_string(),
                        _ => {}
                    }
                }
            }
        }
        let mut affixes: Vec<String> = Vec::new();
        if let Some(map) = root_file("prefix.map") {
            for line in map.lines() {
                for field in line.split('\t').skip(1) {
                    let field = field.trim();
                    if !field.is_empty() && !affixes.iter().any(|a| a == field) {
                        affixes.push(field.to_string());
                    }
                }
            }
        }
        // Longer affixes first, so "_C#4" is removed before "4".
        affixes.sort_by_key(|a| std::cmp::Reverse(a.len()));
        let language = options.language.unwrap_or_else(|| {
            guess_language(&entries.iter().map(|e| e.alias.as_str()).collect::<Vec<_>>())
        });
        if name.is_empty() {
            name = "Imported voice".into();
        }
        let mut by_file: BTreeMap<String, Entries> = BTreeMap::new();
        report.entries = entries.len();
        for entry in entries {
            if let Some(phonemes) = phonemes_of(language, &entry.alias, &affixes) {
                by_file
                    .entry(entry.file.clone())
                    .or_default()
                    .push((entry, phonemes));
            } else {
                report.unread_count += 1;
                if report.unread.len() < 40 {
                    report.unread.push(entry.alias);
                }
            }
        }
        if by_file.is_empty() {
            return Err(format!(
                "No alias could be read as {} phonemes",
                language.code()
            ));
        }
        let bank = Voicebank::new(Info {
            name,
            author,
            language,
            rate: 0,
            description: "Imported from UTAU".into(),
        });
        let mut queue: Vec<_> = by_file.into_iter().collect();
        queue.reverse();
        Ok(Self {
            files,
            total: queue.len(),
            queue,
            language,
            bank,
            report,
        })
    }
    pub fn total(&self) -> usize {
        self.total
    }
    pub fn done(&self) -> usize {
        self.total - self.queue.len()
    }
    /// Analyzes the next recording; false once all are done.
    pub fn step(&mut self) -> bool {
        let Some((file, list)) = self.queue.pop() else {
            return false;
        };
        self.analyze(file, list);
        true
    }
    pub fn finish(mut self) -> Result<(Voicebank, Report), String> {
        while self.step() {}
        if self.bank.units.is_empty() {
            return Err("No usable oto entries".into());
        }
        self.bank.finish();
        self.report.units = self.bank.units.len();
        Ok((self.bank, self.report))
    }

    fn analyze(&mut self, file: String, list: Entries) {
        let options = Options::default();
        let Some(audio) = self
            .files
            .read(&file)
            .and_then(|b| crate::wav::read(&b).ok())
        else {
            self.report.missing_files.push(file);
            return;
        };
        if self.bank.info.rate == 0 {
            self.bank.info.rate = audio.rate;
        }
        let file_ms = audio.samples.len() as f32 * 1000.0 / audio.rate as f32;
        // Analyze only what the entries use, with a margin.
        let mut lo = f32::MAX;
        let mut hi = 0.0f32;
        for (e, _) in &list {
            lo = lo.min(e.offset.max(0.0));
            let end = if e.cutoff < 0.0 {
                e.offset - e.cutoff
            } else {
                file_ms - e.cutoff
            };
            hi = hi.max(end.min(file_ms));
        }
        let lo = (lo - 60.0).max(0.0);
        let hi = (hi + 60.0).min(file_ms);
        if hi <= lo {
            return;
        }
        let a = (lo / 1000.0 * audio.rate as f32) as usize;
        let b = ((hi / 1000.0 * audio.rate as f32) as usize).min(audio.samples.len());
        let frames = analysis::analyze(&audio.samples[a..b], audio.rate, &options);
        let first = self.bank.push_frames(&frames);
        let frame_ms = FRAME_SECONDS * 1000.0;
        let to_frame = |ms: f32| -> u32 {
            let local = ((ms - lo) / frame_ms)
                .round()
                .clamp(0.0, frames.len() as f32 - 1.0);
            first + local as u32
        };
        let loudest = frames.iter().map(|f| f.level()).fold(-300.0f32, f32::max);
        let audible = |ms: f32| {
            let i = ((ms - lo) / frame_ms)
                .round()
                .clamp(0.0, frames.len() as f32 - 1.0) as usize;
            frames[i].level() > loudest - 35.0
        };
        for (entry, phonemes) in list {
            let Some(ms) = boundaries(&entry, &phonemes, file_ms, &audible) else {
                continue;
            };
            let bounds: Vec<u32> = ms.iter().map(|&t| to_frame(t)).collect();
            if bounds.last() <= bounds.first() {
                continue;
            }
            let start = bounds[0];
            let end = *bounds.last().unwrap();
            let last = *phonemes.last().unwrap();
            let last_start = bounds[phonemes.len() - 1];
            let fixed = to_frame(entry.offset + entry.consonant).clamp(start, end);
            let margin = (0.04 / FRAME_SECONDS) as u32;
            let stable = if last.class().sustainable() && !last.is_pause() {
                let a = fixed.max(last_start + margin);
                let b = end.saturating_sub(margin);
                if b > a + 2 { [a, b] } else { [0, 0] }
            } else {
                [0, 0]
            };
            // Pitch and level come from the unit's vowel: a VC unit's final
            // consonant is quieter and says nothing about the voice's level.
            let (pitch, level) = match phonemes.iter().rposition(|p| p.is_vowel()) {
                Some(v) => self.bank.measure(bounds[v], bounds[v + 1]),
                None => (self.bank.measure(last_start, end).0, FLOOR_DB),
            };
            // Japanese banks record alternative takes under katakana aliases.
            let variant = self.language == Language::Ja
                && entry
                    .alias
                    .chars()
                    .any(|c| ('\u{30A1}'..='\u{30F6}').contains(&c));
            self.bank.add_unit(Unit {
                source: entry.file.clone(),
                alias: entry.alias,
                phonemes,
                bounds,
                fixed,
                stable,
                pitch,
                level,
                variant,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ja(list: &str) -> Vec<Phoneme> {
        Phoneme::parse_list(Language::Ja, list).unwrap()
    }

    #[test]
    fn oto_lines() {
        let text =
            "_ああいあうえあ.wav=- あ,250,100,-500,60,0\nka.wav=,10,80,-300,50,20\nbad line\n";
        let e = parse_oto(text, "A4");
        assert_eq!(e.len(), 2);
        assert_eq!(e[0].file, "A4/_ああいあうえあ.wav");
        assert_eq!(e[0].alias, "- あ");
        assert_eq!(e[1].alias, "ka");
        assert_eq!(e[1].cutoff, -300.0);
    }

    #[test]
    fn shift_jis_text() {
        let (bytes, _, _) = encoding_rs::SHIFT_JIS.encode("あ.wav=- あ,1,2,3,4,5");
        assert_eq!(decode_text(&bytes), "あ.wav=- あ,1,2,3,4,5");
    }

    #[test]
    fn suffixes_and_takes_are_removed() {
        let affixes = vec!["↑".to_string(), "_C4".to_string()];
        assert_eq!(
            phonemes_of(Language::Ja, "a か↑", &affixes),
            Some(ja("a k a"))
        );
        assert_eq!(
            phonemes_of(Language::Ja, "ka_C4", &affixes),
            Some(ja("k a"))
        );
        assert_eq!(
            phonemes_of(Language::Ja, "a ka2", &affixes),
            Some(ja("a k a"))
        );
    }

    #[test]
    fn boundaries_follow_the_oto_markers() {
        let entry = |alias: &str| OtoEntry {
            file: String::new(),
            alias: alias.into(),
            offset: 100.0,
            consonant: 120.0,
            cutoff: -400.0,
            preutter: 80.0,
            overlap: 30.0,
        };
        let loud = |_: f32| true;
        // VCV: the leading vowel ends at the overlap, the vowel starts at preutterance.
        let b = boundaries(&entry("a ka"), &ja("a k a"), 1000.0, &loud).unwrap();
        assert_eq!(b, vec![100.0, 130.0, 180.0, 500.0]);
        // CV: the consonant fills offset → preutterance.
        let b = boundaries(&entry("ka"), &ja("k a"), 1000.0, &loud).unwrap();
        assert_eq!(b, vec![100.0, 180.0, 500.0]);
        // VC: the consonant starts at preutterance.
        let b = boundaries(&entry("a k"), &ja("a k"), 1000.0, &loud).unwrap();
        assert_eq!(b, vec![100.0, 180.0, 500.0]);
        // Initial CV: silence until the sound begins.
        let onset = |t: f32| t >= 140.0;
        let b = boundaries(&entry("- ka"), &ja("sil k a"), 1000.0, &onset).unwrap();
        assert_eq!(b[0], 100.0);
        assert!((b[1] - 135.0).abs() <= 5.0, "{b:?}");
        assert_eq!(b[2], 180.0);
    }
}
