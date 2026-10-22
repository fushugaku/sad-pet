//! One phoneme inventory for every language. Japanese symbols follow the
//! romaji convention of UTAU and DiffSinger, English uses lower-case ARPAbet
//! and Russian marks palatalized consonants with an apostrophe. A sound that
//! two languages share still gets one entry per language, so each language
//! keeps its own realization; `distance` lets a voice of one language stand
//! in for a phoneme it lacks.
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Ja,
    En,
    Ru,
}
impl Language {
    pub const ALL: [Self; 3] = [Self::Ja, Self::En, Self::Ru];
    pub fn code(self) -> &'static str {
        match self {
            Self::Ja => "ja",
            Self::En => "en",
            Self::Ru => "ru",
        }
    }
    pub fn parse(code: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|l| l.code().eq_ignore_ascii_case(code.trim()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Class {
    /// A rest: the voice is silent.
    Silence,
    /// An audible breath before a phrase.
    Breath,
    /// A held stop before a geminate (Japanese っ).
    Closure,
    Vowel,
    Stop,
    Affricate,
    Fricative,
    Nasal,
    Liquid,
    Glide,
}
impl Class {
    /// Classes that can be held or stretched without sounding wrong.
    pub fn sustainable(self) -> bool {
        matches!(
            self,
            Self::Vowel
                | Self::Nasal
                | Self::Liquid
                | Self::Fricative
                | Self::Silence
                | Self::Breath
        )
    }
    /// A join inside this class hides well; a join inside a stop burst does not.
    pub fn join_cost(self) -> f32 {
        match self {
            Self::Silence | Self::Closure => 0.05,
            Self::Vowel => 0.1,
            Self::Nasal | Self::Fricative | Self::Breath => 0.3,
            Self::Liquid | Self::Glide => 0.6,
            Self::Affricate => 1.5,
            Self::Stop => 2.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Place {
    /// Vowels and the shared pauses.
    Open,
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Postalveolar,
    Palatal,
    Velar,
    Uvular,
    Glottal,
}

#[derive(Clone, Copy, Debug)]
pub struct Info {
    pub symbol: &'static str,
    /// `None` for the language-independent silence, breath and closure.
    pub language: Option<Language>,
    pub class: Class,
    pub voiced: bool,
    pub place: Place,
    /// Palatalized consonant (Russian soft, Japanese yōon).
    pub soft: bool,
    /// Vowel formants F1–F3 in Hz (start → end for diphthongs); 0 elsewhere.
    pub formants: [f32; 3],
    pub glide_to: [f32; 3],
}

const fn v(language: Language, symbol: &'static str, f: [f32; 3]) -> Info {
    Info {
        symbol,
        language: Some(language),
        class: Class::Vowel,
        voiced: true,
        place: Place::Open,
        soft: false,
        formants: f,
        glide_to: f,
    }
}
const fn d(language: Language, symbol: &'static str, f: [f32; 3], to: [f32; 3]) -> Info {
    Info {
        glide_to: to,
        ..v(language, symbol, f)
    }
}
const fn c(
    language: Language,
    symbol: &'static str,
    class: Class,
    voiced: bool,
    place: Place,
    soft: bool,
) -> Info {
    Info {
        symbol,
        language: Some(language),
        class,
        voiced,
        place,
        soft,
        formants: [0.0; 3],
        glide_to: [0.0; 3],
    }
}
const fn shared(symbol: &'static str, class: Class) -> Info {
    Info {
        symbol,
        language: None,
        class,
        voiced: false,
        place: Place::Open,
        soft: false,
        formants: [0.0; 3],
        glide_to: [0.0; 3],
    }
}

use Class::*;
use Language::*;
use Place::*;

static INVENTORY: &[Info] = &[
    shared("sil", Silence),
    shared("br", Breath),
    shared("cl", Closure),
    // Japanese
    v(Ja, "a", [800., 1250., 2600.]),
    v(Ja, "i", [300., 2300., 3000.]),
    v(Ja, "u", [350., 1400., 2400.]),
    v(Ja, "e", [480., 1900., 2600.]),
    v(Ja, "o", [500., 850., 2600.]),
    c(Ja, "N", Nasal, true, Uvular, false),
    c(Ja, "k", Stop, false, Velar, false),
    c(Ja, "g", Stop, true, Velar, false),
    c(Ja, "s", Fricative, false, Alveolar, false),
    c(Ja, "z", Affricate, true, Alveolar, false),
    c(Ja, "sh", Fricative, false, Postalveolar, true),
    c(Ja, "j", Affricate, true, Postalveolar, true),
    c(Ja, "t", Stop, false, Alveolar, false),
    c(Ja, "d", Stop, true, Alveolar, false),
    c(Ja, "ch", Affricate, false, Postalveolar, true),
    c(Ja, "ts", Affricate, false, Alveolar, false),
    c(Ja, "n", Nasal, true, Alveolar, false),
    c(Ja, "h", Fricative, false, Glottal, false),
    c(Ja, "f", Fricative, false, Bilabial, false),
    c(Ja, "b", Stop, true, Bilabial, false),
    c(Ja, "p", Stop, false, Bilabial, false),
    c(Ja, "m", Nasal, true, Bilabial, false),
    c(Ja, "y", Glide, true, Palatal, true),
    c(Ja, "r", Liquid, true, Alveolar, false),
    c(Ja, "w", Glide, true, Velar, false),
    c(Ja, "v", Fricative, true, Labiodental, false),
    c(Ja, "ky", Stop, false, Velar, true),
    c(Ja, "gy", Stop, true, Velar, true),
    c(Ja, "ny", Nasal, true, Palatal, true),
    c(Ja, "hy", Fricative, false, Palatal, true),
    c(Ja, "my", Nasal, true, Bilabial, true),
    c(Ja, "ry", Liquid, true, Alveolar, true),
    c(Ja, "by", Stop, true, Bilabial, true),
    c(Ja, "py", Stop, false, Bilabial, true),
    c(Ja, "ty", Stop, false, Alveolar, true),
    c(Ja, "dy", Stop, true, Alveolar, true),
    // English (ARPAbet)
    v(En, "aa", [730., 1090., 2440.]),
    v(En, "ae", [660., 1720., 2410.]),
    v(En, "ah", [640., 1190., 2390.]),
    v(En, "ao", [570., 840., 2410.]),
    v(En, "ax", [500., 1500., 2500.]),
    v(En, "eh", [530., 1840., 2480.]),
    v(En, "er", [490., 1350., 1690.]),
    v(En, "ih", [390., 1990., 2550.]),
    v(En, "iy", [270., 2290., 3010.]),
    v(En, "uh", [440., 1020., 2240.]),
    v(En, "uw", [300., 870., 2240.]),
    d(En, "aw", [730., 1090., 2440.], [440., 1020., 2240.]),
    d(En, "ay", [730., 1090., 2440.], [390., 1990., 2550.]),
    d(En, "ey", [530., 1840., 2480.], [390., 1990., 2550.]),
    d(En, "ow", [570., 840., 2410.], [440., 1020., 2240.]),
    d(En, "oy", [570., 840., 2410.], [390., 1990., 2550.]),
    c(En, "b", Stop, true, Bilabial, false),
    c(En, "ch", Affricate, false, Postalveolar, false),
    c(En, "d", Stop, true, Alveolar, false),
    c(En, "dh", Fricative, true, Dental, false),
    c(En, "f", Fricative, false, Labiodental, false),
    c(En, "g", Stop, true, Velar, false),
    c(En, "hh", Fricative, false, Glottal, false),
    c(En, "jh", Affricate, true, Postalveolar, false),
    c(En, "k", Stop, false, Velar, false),
    c(En, "l", Liquid, true, Alveolar, false),
    c(En, "m", Nasal, true, Bilabial, false),
    c(En, "n", Nasal, true, Alveolar, false),
    c(En, "ng", Nasal, true, Velar, false),
    c(En, "p", Stop, false, Bilabial, false),
    c(En, "r", Liquid, true, Postalveolar, false),
    c(En, "s", Fricative, false, Alveolar, false),
    c(En, "sh", Fricative, false, Postalveolar, false),
    c(En, "t", Stop, false, Alveolar, false),
    c(En, "th", Fricative, false, Dental, false),
    c(En, "v", Fricative, true, Labiodental, false),
    c(En, "w", Glide, true, Velar, false),
    c(En, "y", Glide, true, Palatal, true),
    c(En, "z", Fricative, true, Alveolar, false),
    c(En, "zh", Fricative, true, Postalveolar, false),
    // Russian
    v(Ru, "a", [750., 1300., 2500.]),
    v(Ru, "o", [520., 900., 2500.]),
    v(Ru, "u", [330., 800., 2300.]),
    v(Ru, "e", [480., 1850., 2550.]),
    v(Ru, "i", [290., 2300., 3000.]),
    v(Ru, "y", [330., 1600., 2400.]),
    c(Ru, "p", Stop, false, Bilabial, false),
    c(Ru, "p'", Stop, false, Bilabial, true),
    c(Ru, "b", Stop, true, Bilabial, false),
    c(Ru, "b'", Stop, true, Bilabial, true),
    c(Ru, "t", Stop, false, Dental, false),
    c(Ru, "t'", Stop, false, Dental, true),
    c(Ru, "d", Stop, true, Dental, false),
    c(Ru, "d'", Stop, true, Dental, true),
    c(Ru, "k", Stop, false, Velar, false),
    c(Ru, "k'", Stop, false, Velar, true),
    c(Ru, "g", Stop, true, Velar, false),
    c(Ru, "g'", Stop, true, Velar, true),
    c(Ru, "f", Fricative, false, Labiodental, false),
    c(Ru, "f'", Fricative, false, Labiodental, true),
    c(Ru, "v", Fricative, true, Labiodental, false),
    c(Ru, "v'", Fricative, true, Labiodental, true),
    c(Ru, "s", Fricative, false, Dental, false),
    c(Ru, "s'", Fricative, false, Dental, true),
    c(Ru, "z", Fricative, true, Dental, false),
    c(Ru, "z'", Fricative, true, Dental, true),
    c(Ru, "x", Fricative, false, Velar, false),
    c(Ru, "x'", Fricative, false, Velar, true),
    c(Ru, "m", Nasal, true, Bilabial, false),
    c(Ru, "m'", Nasal, true, Bilabial, true),
    c(Ru, "n", Nasal, true, Dental, false),
    c(Ru, "n'", Nasal, true, Dental, true),
    c(Ru, "l", Liquid, true, Alveolar, false),
    c(Ru, "l'", Liquid, true, Alveolar, true),
    c(Ru, "r", Liquid, true, Alveolar, false),
    c(Ru, "r'", Liquid, true, Alveolar, true),
    c(Ru, "sh", Fricative, false, Postalveolar, false),
    c(Ru, "zh", Fricative, true, Postalveolar, false),
    c(Ru, "sch", Fricative, false, Postalveolar, true),
    c(Ru, "ch", Affricate, false, Postalveolar, true),
    c(Ru, "ts", Affricate, false, Dental, false),
    c(Ru, "j", Glide, true, Palatal, true),
];

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Phoneme(u8);

impl Phoneme {
    pub const SIL: Self = Self(0);
    pub const BREATH: Self = Self(1);
    pub const CLOSURE: Self = Self(2);

    pub fn all() -> impl Iterator<Item = Self> {
        (0..INVENTORY.len()).map(|i| Self(i as u8))
    }
    pub fn of(language: Language) -> impl Iterator<Item = Self> {
        Self::all().filter(move |p| p.info().language.is_none_or(|l| l == language))
    }
    pub fn info(self) -> &'static Info {
        &INVENTORY[self.0 as usize]
    }
    pub fn index(self) -> usize {
        self.0 as usize
    }
    pub fn from_index(index: usize) -> Option<Self> {
        (index < INVENTORY.len()).then_some(Self(index as u8))
    }
    pub fn symbol(self) -> &'static str {
        self.info().symbol
    }
    pub fn class(self) -> Class {
        self.info().class
    }
    pub fn language(self) -> Option<Language> {
        self.info().language
    }
    pub fn is_vowel(self) -> bool {
        self.class() == Class::Vowel
    }
    /// A syllable nucleus: a vowel, or a Japanese moraic nasal sung on its own.
    pub fn is_nucleus(self) -> bool {
        self.is_vowel() || (self.class() == Class::Nasal && self.symbol() == "N")
    }
    pub fn is_pause(self) -> bool {
        matches!(self.class(), Class::Silence | Class::Breath)
    }
    pub fn voiced(self) -> bool {
        self.is_vowel() || self.info().voiced
    }
    /// Looks a symbol up in one language; the shared `sil`/`br`/`cl` and the
    /// UTAU aliases `-`, `R` and `pau` are accepted everywhere.
    pub fn parse(language: Language, symbol: &str) -> Option<Self> {
        let symbol = symbol.trim();
        match symbol {
            "-" | "R" | "pau" | "SP" | "sil" => return Some(Self::SIL),
            "br" | "AP" | "breath" | "息" => return Some(Self::BREATH),
            "cl" | "Q" => return Some(Self::CLOSURE),
            _ => {}
        }
        let exact = Self::of(language).find(|p| p.symbol() == symbol);
        // ARPAbet is often written in upper case, with stress digits.
        exact.or_else(|| {
            if language != Language::En {
                return None;
            }
            let lower = symbol
                .trim_end_matches(['0', '1', '2'])
                .to_ascii_lowercase();
            Self::of(language).find(|p| p.symbol() == lower)
        })
    }
    /// Parses a space-separated phoneme string; `None` names the first unknown symbol.
    pub fn parse_list(language: Language, text: &str) -> Result<Vec<Self>, String> {
        text.split_whitespace()
            .map(|s| Self::parse(language, s).ok_or_else(|| s.to_string()))
            .collect()
    }
    /// How badly `other` stands in for `self`: 0 for the same sound, about 1
    /// for a related sound and ≥ 4 for something unrelated.
    pub fn distance(self, other: Self) -> f32 {
        if self == other {
            return 0.0;
        }
        let (a, b) = (self.info(), other.info());
        let same_symbol = a.symbol == b.symbol && a.language != b.language;
        if a.class == Vowel && b.class == Vowel {
            let span = |f: [f32; 3], g: [f32; 3]| {
                ((f[0] / g[0]).ln().powi(2) * 4.0 + (f[1] / g[1]).ln().powi(2) * 2.0).sqrt()
            };
            let start = span(a.formants, b.formants);
            let end = span(a.glide_to, b.glide_to);
            return 0.15 + (start + end) * 1.5 + if same_symbol { -0.1 } else { 0.0 };
        }
        if a.class.sustainable() != b.class.sustainable() && (a.class == Vowel || b.class == Vowel)
        {
            return 8.0;
        }
        let mut cost = if same_symbol { 0.2 } else { 0.4 };
        if a.class != b.class {
            let close = matches!(
                (a.class, b.class),
                (Stop, Affricate)
                    | (Affricate, Stop)
                    | (Affricate, Fricative)
                    | (Fricative, Affricate)
                    | (Liquid, Glide)
                    | (Glide, Liquid)
            );
            cost += if close { 1.0 } else { 3.0 };
        }
        if a.place != b.place {
            cost += 0.8;
        }
        if a.voiced != b.voiced {
            cost += 0.9;
        }
        if a.soft != b.soft {
            cost += 0.5;
        }
        cost
    }
}
impl std::fmt::Debug for Phoneme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.language() {
            Some(l) => write!(f, "{}:{}", l.code(), self.symbol()),
            None => f.write_str(self.symbol()),
        }
    }
}
impl std::fmt::Display for Phoneme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.symbol())
    }
}

/// Serialized as `"ja:a"`, `"ru:t'"` or `"sil"`, so files stay readable.
impl Serialize for Phoneme {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(&format_args!("{self:?}"))
    }
}
impl<'de> Deserialize<'de> for Phoneme {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        let parsed = match text.split_once(':') {
            Some((code, symbol)) => Language::parse(code).and_then(|l| Self::parse(l, symbol)),
            None => Self::parse(Language::Ja, &text).filter(|p| p.language().is_none()),
        };
        parsed.ok_or_else(|| serde::de::Error::custom(format!("unknown phoneme {text}")))
    }
}

/// A sung syllable: its phonemes and which one carries the note.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Syllable {
    pub phonemes: Vec<Phoneme>,
    pub nucleus: usize,
}
impl Syllable {
    pub fn new(phonemes: Vec<Phoneme>) -> Self {
        let nucleus = phonemes
            .iter()
            .position(|p| p.is_vowel())
            .or_else(|| phonemes.iter().position(|p| p.is_nucleus()))
            .or_else(|| phonemes.iter().position(|p| p.voiced() && !p.is_pause()))
            .unwrap_or(0);
        Self { phonemes, nucleus }
    }
    pub fn onset(&self) -> &[Phoneme] {
        &self.phonemes[..self.nucleus]
    }
    pub fn coda(&self) -> &[Phoneme] {
        self.phonemes.get(self.nucleus + 1..).unwrap_or(&[])
    }
    pub fn vowel(&self) -> Option<Phoneme> {
        self.phonemes.get(self.nucleus).copied()
    }
}

/// Splits a word's phonemes into syllables: one per vowel; a lone consonant
/// between vowels opens the next syllable, and in a longer cluster a leading
/// sonorant closes the previous one.
pub fn syllabify(phonemes: &[Phoneme]) -> Vec<Syllable> {
    let nuclei: Vec<usize> = phonemes
        .iter()
        .enumerate()
        .filter(|(_, p)| p.is_vowel())
        .map(|(i, _)| i)
        .collect();
    if nuclei.is_empty() {
        return if phonemes.is_empty() {
            Vec::new()
        } else {
            vec![Syllable::new(phonemes.to_vec())]
        };
    }
    let mut cuts = vec![0];
    for pair in nuclei.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let cluster = &phonemes[a + 1..b];
        let to_coda = match cluster.len() {
            0 | 1 => 0,
            _ if matches!(cluster[0].class(), Nasal | Liquid | Glide) => 1,
            // English keeps a final consonant of a cluster like "st" together
            // only when the cluster is not a legal onset; one coda is a
            // reasonable sung compromise.
            n if n >= 3 => 1,
            _ => 0,
        };
        cuts.push(a + 1 + to_coda);
    }
    cuts.push(phonemes.len());
    cuts.windows(2)
        .map(|w| Syllable::new(phonemes[w[0]..w[1]].to_vec()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols_are_unique_per_language() {
        for language in Language::ALL {
            let mut seen = std::collections::HashSet::new();
            for p in Phoneme::of(language) {
                assert!(seen.insert(p.symbol()), "{language:?} {}", p.symbol());
                assert_eq!(Phoneme::parse(language, p.symbol()), Some(p));
            }
        }
        assert!(INVENTORY.len() < 256);
    }
    #[test]
    fn serde_round_trip() {
        let list = Phoneme::parse_list(Language::Ru, "sil p' r' i v' e t br").unwrap();
        let json = serde_json::to_string(&list).unwrap();
        assert_eq!(
            json,
            r#"["sil","ru:p'","ru:r'","ru:i","ru:v'","ru:e","ru:t","br"]"#
        );
        let back: Vec<Phoneme> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, list);
        assert_eq!(Phoneme::parse(Language::En, "AH0").unwrap().symbol(), "ah");
    }
    #[test]
    fn distance_prefers_related_sounds() {
        let ru = |s| Phoneme::parse(Language::Ru, s).unwrap();
        let ja = |s| Phoneme::parse(Language::Ja, s).unwrap();
        assert!(ru("a").distance(ja("a")) < ru("a").distance(ja("o")));
        assert!(ru("t'").distance(ru("t")) < ru("t'").distance(ru("k")));
        assert!(ru("a").distance(ru("t")) >= 4.0);
    }
    #[test]
    fn syllables_split_between_vowels() {
        let ph = Phoneme::parse_list(Language::En, "hh ax l ow").unwrap();
        let s = syllabify(&ph);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0].phonemes.len(), 2);
        assert_eq!(s[1].onset().len(), 1);
        let ph = Phoneme::parse_list(Language::Ru, "s' e r d t s e").unwrap();
        let s = syllabify(&ph);
        assert_eq!(s[0].coda(), &[Phoneme::parse(Language::Ru, "r").unwrap()]);
    }
}
