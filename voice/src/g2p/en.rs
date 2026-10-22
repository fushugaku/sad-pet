//! English lyrics: a built-in dictionary of common words, a loadable
//! CMUdict-format dictionary, simple suffix handling, and the Naval Research
//! Laboratory letter-to-sound rules (Elovitz et al., 1976) for everything else.
use crate::phoneme::{Language, Phoneme, Syllable, syllabify};
use std::collections::HashMap;
use std::sync::LazyLock;

fn ph(symbol: &str) -> Phoneme {
    Phoneme::parse(Language::En, symbol).unwrap_or_else(|| panic!("en phoneme {symbol}"))
}

/// Words to ARPAbet. `parse` accepts both this crate's files and CMUdict
/// (`WORD  W ER1 D`, `WORD(2) ...` alternates and `;;;` comments).
#[derive(Clone, Debug, Default)]
pub struct Dictionary {
    words: HashMap<String, Vec<Phoneme>>,
}
impl Dictionary {
    pub fn parse(text: &str) -> Self {
        let mut words = HashMap::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(";;;") {
                continue;
            }
            let mut parts = line.split_whitespace();
            let Some(word) = parts.next() else { continue };
            if word.ends_with(')') {
                continue; // alternate pronunciation; keep the first
            }
            let phonemes: Option<Vec<Phoneme>> =
                parts.map(|p| Phoneme::parse(Language::En, p)).collect();
            if let Some(p) = phonemes.filter(|p| !p.is_empty()) {
                words.entry(normalize(word)).or_insert(p);
            }
        }
        Self { words }
    }
    pub fn get(&self, word: &str) -> Option<&[Phoneme]> {
        self.words.get(&normalize(word)).map(Vec::as_slice)
    }
    pub fn len(&self) -> usize {
        self.words.len()
    }
    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }
    pub fn extend(&mut self, other: Dictionary) {
        self.words.extend(other.words);
    }
}

static BUILT_IN: LazyLock<Dictionary> =
    LazyLock::new(|| Dictionary::parse(include_str!("en_dict.txt")));

fn normalize(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn lookup(word: &str, extra: Option<&Dictionary>) -> Option<Vec<Phoneme>> {
    extra
        .and_then(|d| d.get(word))
        .or_else(|| BUILT_IN.get(word))
        .map(<[Phoneme]>::to_vec)
}

fn voiceless(p: Phoneme) -> bool {
    matches!(p.symbol(), "p" | "t" | "k" | "f" | "th")
}
fn sibilant(p: Phoneme) -> bool {
    matches!(p.symbol(), "s" | "z" | "sh" | "zh" | "ch" | "jh")
}

/// A known stem plus a regular suffix, e.g. "dreaming", "walked", "hearts".
fn suffixed(word: &str, extra: Option<&Dictionary>) -> Option<Vec<Phoneme>> {
    let w = normalize(word);
    let stem = |s: &str| lookup(s, extra).filter(|p| !p.is_empty());
    let with = |mut base: Vec<Phoneme>, tail: &[&str]| {
        base.extend(tail.iter().map(|s| ph(s)));
        base
    };
    if let Some(s) = w.strip_suffix("in").filter(|_| word.ends_with('\'')) {
        let full = format!("{s}ing");
        return lookup(&full, extra).or_else(|| suffixed(&full, extra));
    }
    if let Some(s) = w.strip_suffix("ing") {
        if let Some(b) = stem(s).or_else(|| stem(&format!("{s}e"))) {
            return Some(with(b, &["ih", "ng"]));
        }
        if s.len() > 2
            && s.as_bytes()[s.len() - 1] == s.as_bytes()[s.len() - 2]
            && let Some(b) = stem(&s[..s.len() - 1])
        {
            return Some(with(b, &["ih", "ng"]));
        }
    }
    if let Some(s) = w.strip_suffix("ed") {
        let base = stem(s).or_else(|| stem(&format!("{s}e"))).or_else(|| {
            (s.len() > 2 && s.as_bytes()[s.len() - 1] == s.as_bytes()[s.len() - 2])
                .then(|| stem(&s[..s.len() - 1]))
                .flatten()
        });
        if let Some(b) = base {
            let last = *b.last()?;
            let tail: &[&str] = if matches!(last.symbol(), "t" | "d") {
                &["ih", "d"]
            } else if voiceless(last) || matches!(last.symbol(), "s" | "sh" | "ch") {
                &["t"]
            } else {
                &["d"]
            };
            return Some(with(b, tail));
        }
    }
    if let Some(s) = w.strip_suffix("ly")
        && let Some(b) = stem(s)
    {
        return Some(with(b, &["l", "iy"]));
    }
    if let Some(s) = w.strip_suffix("er")
        && let Some(b) = stem(s).or_else(|| stem(&format!("{s}e")))
    {
        return Some(with(b, &["er"]));
    }
    if let Some(s) = w.strip_suffix("es")
        && let Some(b) = stem(s).filter(|b| b.last().is_some_and(|p| sibilant(*p)))
    {
        return Some(with(b, &["ih", "z"]));
    }
    if let Some(s) = w.strip_suffix('s')
        && let Some(b) = stem(s)
    {
        let last = *b.last()?;
        let tail: &[&str] = if sibilant(last) {
            &["ih", "z"]
        } else if voiceless(last) {
            &["s"]
        } else {
            &["z"]
        };
        return Some(with(b, tail));
    }
    None
}

// --- NRL letter-to-sound rules ------------------------------------------

/// `(left context, text, right context, phonemes)`. Context symbols:
/// `#` one or more vowels, `:` zero or more consonants, `^` one consonant,
/// `.` a voiced consonant, `+` a front vowel (E, I, Y), `%` a suffix
/// (-ER, -E, -ES, -ED, -ING, -ELY), `&` a sibilant, `@` T S R D L Z N J TH
/// CH SH, and space for the word edge.
type Rule = (&'static str, &'static str, &'static str, &'static str);

const RULES: &[Rule] = &[
    // A
    ("", "A", " ", "ah"),
    (" ", "ARE", " ", "aa r"),
    ("", "AR", "O", "er"),
    ("", "AR", "#", "eh r"),
    (" ^", "AS", "#", "ey s"),
    ("", "A", "WA", "ah"),
    ("", "AW", "", "ao"),
    (" :", "ANY", "", "eh n iy"),
    ("", "A", "^+#", "ey"),
    ("#:", "ALLY", "", "ah l iy"),
    (" ", "AL", "#", "ah l"),
    ("", "AGAIN", "", "ah g eh n"),
    ("#:", "AG", "E", "ih jh"),
    ("", "A", "^+:#", "ae"),
    (" :", "A", "^+ ", "ey"),
    ("", "A", "^%", "ey"),
    (" ", "ARR", "", "ah r"),
    ("", "ARR", "", "ae r"),
    (" :", "AR", " ", "aa r"),
    ("", "AR", " ", "er"),
    ("", "AR", "", "aa r"),
    ("", "AIR", "", "eh r"),
    ("", "AI", "", "ey"),
    ("", "AY", "", "ey"),
    ("", "AU", "", "ao"),
    ("#:", "AL", " ", "ah l"),
    ("#:", "ALS", " ", "ah l z"),
    ("", "ALK", "", "ao k"),
    ("", "AL", "^", "ao l"),
    (" :", "ABLE", "", "ey b ah l"),
    ("", "ABLE", "", "ah b ah l"),
    ("", "ANG", "+", "ey n jh"),
    ("", "A", "", "ae"),
    // B
    (" ", "BE", "^#", "b ih"),
    ("", "BEING", "", "b iy ih ng"),
    (" ", "BOTH", " ", "b ow th"),
    (" ", "BUS", "#", "b ih z"),
    ("", "BUIL", "", "b ih l"),
    ("", "B", "", "b"),
    // C
    (" ", "CH", "^", "k"),
    ("^E", "CH", "", "k"),
    ("", "CH", "", "ch"),
    (" S", "CI", "#", "s ay"),
    ("", "CI", "A", "sh"),
    ("", "CI", "O", "sh"),
    ("", "CI", "EN", "sh"),
    ("", "C", "+", "s"),
    ("", "CK", "", "k"),
    ("", "COM", "%", "k ah m"),
    ("", "C", "", "k"),
    // D
    ("#:", "DED", " ", "d ih d"),
    (".E", "D", " ", "d"),
    ("#^:E", "D", " ", "t"),
    (" ", "DE", "^#", "d ih"),
    (" ", "DO", " ", "d uw"),
    (" ", "DOES", "", "d ah z"),
    (" ", "DOING", "", "d uw ih ng"),
    (" ", "DOW", "", "d aw"),
    ("", "DU", "A", "jh uw"),
    ("", "D", "", "d"),
    // E
    ("#:", "E", " ", ""),
    ("'^:", "E", " ", ""),
    (" :", "E", " ", "iy"),
    ("#", "ED", " ", "d"),
    ("#:", "E", "D ", ""),
    ("", "EV", "ER", "eh v"),
    ("", "E", "^%", "iy"),
    ("", "ERI", "#", "iy r iy"),
    ("", "ERI", "", "eh r ih"),
    ("#:", "ER", "#", "er"),
    ("", "ER", "#", "eh r"),
    ("", "ER", "", "er"),
    (" ", "EVEN", "", "iy v eh n"),
    ("#:", "E", "W", ""),
    ("@", "EW", "", "uw"),
    ("", "EW", "", "y uw"),
    ("", "E", "O", "iy"),
    ("#:&", "ES", " ", "ih z"),
    ("#:", "E", "S ", ""),
    ("#:", "ELY", " ", "l iy"),
    ("#:", "EMENT", "", "m eh n t"),
    ("", "EFUL", "", "f uh l"),
    ("", "EE", "", "iy"),
    ("", "EARN", "", "er n"),
    (" ", "EAR", "^", "er"),
    ("", "EAD", "", "eh d"),
    ("#:", "EA", " ", "iy ah"),
    ("", "EA", "SU", "eh"),
    ("", "EA", "", "iy"),
    ("", "EIGH", "", "ey"),
    ("", "EI", "", "iy"),
    (" ", "EYE", "", "ay"),
    ("", "EY", "", "iy"),
    ("", "EU", "", "y uw"),
    ("", "E", "", "eh"),
    // F
    ("", "FUL", "", "f uh l"),
    ("", "F", "", "f"),
    // G
    ("", "GIV", "", "g ih v"),
    (" ", "G", "I^", "g"),
    ("", "GE", "T", "g eh"),
    ("SU", "GGES", "", "g jh eh s"),
    ("", "GG", "", "g"),
    (" B#", "G", "", "g"),
    ("", "G", "+", "jh"),
    ("", "GREAT", "", "g r ey t"),
    ("#", "GH", "", ""),
    ("", "G", "", "g"),
    // H
    (" ", "HAV", "", "hh ae v"),
    (" ", "HERE", "", "hh iy r"),
    (" ", "HOUR", "", "aw er"),
    ("", "HOW", "", "hh aw"),
    ("", "H", "#", "hh"),
    ("", "H", "", ""),
    // I
    (" ", "IN", "", "ih n"),
    (" ", "I", " ", "ay"),
    ("", "IN", "D", "ay n"),
    ("", "IER", "", "iy er"),
    ("#:R", "IED", " ", "iy d"),
    ("", "IED", " ", "ay d"),
    ("", "IEN", "", "iy eh n"),
    ("", "IE", "T", "ay eh"),
    (" :", "I", "%", "ay"),
    ("", "I", "%", "iy"),
    ("", "IE", "", "iy"),
    ("", "I", "^+:#", "ih"),
    ("", "IR", "#", "ay r"),
    ("", "IZ", "%", "ay z"),
    ("", "IS", "%", "ay z"),
    ("", "I", "D%", "ay"),
    ("+^", "I", "^+", "ih"),
    ("", "I", "T%", "ay"),
    ("#^:", "I", "^+", "ih"),
    ("", "I", "^+", "ay"),
    ("", "IR", "", "er"),
    ("", "IGH", "", "ay"),
    ("", "ILD", "", "ay l d"),
    ("", "IGN", " ", "ay n"),
    ("", "IGN", "^", "ay n"),
    ("", "IGN", "%", "ay n"),
    ("", "IQUE", "", "iy k"),
    ("", "I", "", "ih"),
    // J
    ("", "J", "", "jh"),
    // K
    (" ", "K", "N", ""),
    ("", "K", "", "k"),
    // L
    ("", "LO", "C#", "l ow"),
    ("L", "L", "", ""),
    ("#^:", "L", "%", "ah l"),
    ("", "LEAD", "", "l iy d"),
    ("", "L", "", "l"),
    // M
    ("", "MOV", "", "m uw v"),
    ("", "M", "", "m"),
    // N
    ("E", "NG", "+", "n jh"),
    ("", "NG", "R", "ng g"),
    ("", "NG", "#", "ng g"),
    ("", "NGL", "%", "ng g ah l"),
    ("", "NG", "", "ng"),
    ("", "NK", "", "ng k"),
    (" ", "NOW", " ", "n aw"),
    ("", "N", "", "n"),
    // O
    ("", "OF", " ", "ah v"),
    ("", "OROUGH", "", "er ow"),
    ("#:", "OR", " ", "er"),
    ("#:", "ORS", " ", "er z"),
    ("", "OR", "", "ao r"),
    (" ", "ONE", "", "w ah n"),
    ("", "OW", "", "ow"),
    (" ", "OVER", "", "ow v er"),
    ("", "OV", "", "ah v"),
    ("", "O", "^%", "ow"),
    ("", "O", "^EN", "ow"),
    ("", "O", "^I#", "ow"),
    ("", "OL", "D", "ow l"),
    ("", "OUGHT", "", "ao t"),
    ("", "OUGH", "", "ah f"),
    (" ", "OU", "", "aw"),
    ("H", "OU", "S#", "aw"),
    ("", "OUS", "", "ah s"),
    ("", "OUR", "", "ao r"),
    ("", "OULD", "", "uh d"),
    ("^", "OU", "^L", "ah"),
    ("", "OUP", "", "uw p"),
    ("", "OU", "", "aw"),
    ("", "OY", "", "oy"),
    ("", "OING", "", "ow ih ng"),
    ("", "OI", "", "oy"),
    ("", "OOR", "", "ao r"),
    ("", "OOK", "", "uh k"),
    ("", "OOD", "", "uh d"),
    ("", "OO", "", "uw"),
    ("", "O", "E", "ow"),
    ("", "O", " ", "ow"),
    ("", "OA", "", "ow"),
    (" ", "ONLY", "", "ow n l iy"),
    (" ", "ONCE", "", "w ah n s"),
    ("C", "O", "N", "aa"),
    ("", "O", "NG", "ao"),
    (" :^", "O", "N", "ah"),
    ("I", "ON", "", "ah n"),
    ("#:", "ON", " ", "ah n"),
    ("#^", "ON", "", "ah n"),
    ("", "O", "ST ", "ow"),
    ("", "OF", "^", "ao f"),
    ("", "OTHER", "", "ah dh er"),
    ("", "OSS", " ", "ao s"),
    ("#^:", "OM", "", "ah m"),
    ("", "O", "", "aa"),
    // P
    ("", "PH", "", "f"),
    ("", "PEOP", "", "p iy p"),
    ("", "POW", "", "p aw"),
    ("", "PUT", " ", "p uh t"),
    ("", "P", "", "p"),
    // Q
    ("", "QUAR", "", "k w ao r"),
    ("", "QU", "", "k w"),
    ("", "Q", "", "k"),
    // R
    (" ", "RE", "^#", "r iy"),
    ("", "R", "", "r"),
    // S
    ("", "SH", "", "sh"),
    ("#", "SION", "", "zh ah n"),
    ("", "SOME", "", "s ah m"),
    ("#", "SUR", "#", "zh er"),
    ("", "SUR", "#", "sh er"),
    ("#", "SU", "#", "zh uw"),
    ("#", "SSU", "#", "sh uw"),
    ("#", "SED", " ", "z d"),
    ("#", "S", "#", "z"),
    ("", "SAID", "", "s eh d"),
    ("^", "SION", "", "sh ah n"),
    ("", "S", "S", ""),
    (".", "S", " ", "z"),
    ("#:.E", "S", " ", "z"),
    ("#^:##", "S", " ", "z"),
    ("#^:#", "S", " ", "s"),
    ("U", "S", " ", "s"),
    (" :#", "S", " ", "z"),
    (" ", "SCH", "", "s k"),
    ("", "S", "C+", ""),
    ("#", "SM", "", "z m"),
    ("", "S", "", "s"),
    // T
    (" ", "THE", " ", "dh ah"),
    ("", "TO", " ", "t uw"),
    ("", "THAT", " ", "dh ae t"),
    (" ", "THIS", " ", "dh ih s"),
    (" ", "THEY", "", "dh ey"),
    (" ", "THERE", "", "dh eh r"),
    ("", "THER", "", "dh er"),
    ("", "THEIR", "", "dh eh r"),
    (" ", "THAN", " ", "dh ae n"),
    (" ", "THEM", " ", "dh eh m"),
    ("", "THESE", " ", "dh iy z"),
    (" ", "THEN", "", "dh eh n"),
    ("", "THROUGH", "", "th r uw"),
    ("", "THOSE", "", "dh ow z"),
    ("", "THOUGH", " ", "dh ow"),
    (" ", "THUS", "", "dh ah s"),
    ("", "TH", "", "th"),
    ("#:", "TED", " ", "t ih d"),
    ("S", "TI", "#N", "ch"),
    ("", "TI", "O", "sh"),
    ("", "TI", "A", "sh"),
    ("", "TIEN", "", "sh ah n"),
    ("", "TUR", "#", "ch er"),
    ("", "TU", "A", "ch uw"),
    (" ", "TWO", "", "t uw"),
    ("", "T", "", "t"),
    // U
    (" ", "UN", "I", "y uw n"),
    (" ", "UN", "", "ah n"),
    (" ", "UPON", "", "ah p ao n"),
    ("@", "UR", "#", "uh r"),
    ("", "UR", "#", "y uh r"),
    ("", "UR", "", "er"),
    ("", "U", "^ ", "ah"),
    ("", "U", "^^", "ah"),
    ("", "UY", "", "ay"),
    (" G", "U", "#", ""),
    ("G", "U", "%", ""),
    ("G", "U", "#", "w"),
    ("#N", "U", "", "y uw"),
    ("@", "U", "", "uw"),
    ("", "U", "", "y uw"),
    // V
    ("", "VIEW", "", "v y uw"),
    ("", "V", "", "v"),
    // W
    (" ", "WERE", "", "w er"),
    ("", "WA", "S", "w aa"),
    ("", "WA", "T", "w aa"),
    ("", "WHERE", "", "w eh r"),
    ("", "WHAT", "", "w ah t"),
    ("", "WHOL", "", "hh ow l"),
    ("", "WHO", "", "hh uw"),
    ("", "WH", "", "w"),
    ("", "WAR", "", "w ao r"),
    ("", "WOR", "^", "w er"),
    ("", "WR", "", "r"),
    ("", "W", "", "w"),
    // X
    ("", "X", "", "k s"),
    // Y
    ("", "YOUNG", "", "y ah ng"),
    (" ", "YOU", "", "y uw"),
    (" ", "YES", "", "y eh s"),
    (" ", "Y", "", "y"),
    ("#^:", "Y", " ", "iy"),
    ("#^:", "Y", "I", "iy"),
    (" :", "Y", " ", "ay"),
    (" :", "Y", "#", "ay"),
    (" :", "Y", "^+:#", "ih"),
    (" :", "Y", "^#", "ay"),
    ("", "Y", "", "ih"),
    // Z
    ("", "Z", "", "z"),
];

fn is_vowel(c: u8) -> bool {
    b"AEIOUY".contains(&c)
}
fn is_consonant(c: u8) -> bool {
    c.is_ascii_uppercase() && !is_vowel(c)
}

/// Matches a right-context pattern starting at `at`; returns success.
fn match_right(word: &[u8], mut at: usize, pattern: &[u8]) -> bool {
    let mut i = 0;
    while i < pattern.len() {
        let p = pattern[i];
        let c = word.get(at).copied().unwrap_or(b' ');
        match p {
            b'#' => {
                if !is_vowel(c) {
                    return false;
                }
                while word.get(at).is_some_and(|&c| is_vowel(c)) {
                    at += 1;
                }
            }
            b':' => {
                while word.get(at).is_some_and(|&c| is_consonant(c)) {
                    at += 1;
                }
            }
            b'^' => {
                if !is_consonant(c) {
                    return false;
                }
                at += 1;
            }
            b'.' => {
                if !b"BDVGJLMNRWZ".contains(&c) {
                    return false;
                }
                at += 1;
            }
            b'+' => {
                if !b"EIY".contains(&c) {
                    return false;
                }
                at += 1;
            }
            b'%' => {
                let rest = &word[at.min(word.len())..];
                let end =
                    |s: &[u8]| rest.starts_with(s) && rest.get(s.len()).is_none_or(|&c| c == b' ');
                let found = [b"ING".as_slice(), b"ELY", b"ER", b"ES", b"ED", b"E"]
                    .into_iter()
                    .find(|s| end(s));
                match found {
                    Some(s) => at += s.len(),
                    None => return false,
                }
            }
            b' ' => {
                if c != b' ' {
                    return false;
                }
                at += 1;
            }
            _ => {
                if c != p {
                    return false;
                }
                at += 1;
            }
        }
        i += 1;
    }
    true
}

/// Matches a left-context pattern ending just before `end`, right to left.
fn match_left(word: &[u8], end: usize, pattern: &[u8]) -> bool {
    let mut at = end as isize - 1;
    let get = |i: isize| if i < 0 { b' ' } else { word[i as usize] };
    for &p in pattern.iter().rev() {
        let c = get(at);
        match p {
            b'#' => {
                if !is_vowel(c) {
                    return false;
                }
                while at >= 0 && is_vowel(get(at)) {
                    at -= 1;
                }
            }
            b':' => {
                while at >= 0 && is_consonant(get(at)) {
                    at -= 1;
                }
            }
            b'^' => {
                if !is_consonant(c) {
                    return false;
                }
                at -= 1;
            }
            b'.' => {
                if !b"BDVGJLMNRWZ".contains(&c) {
                    return false;
                }
                at -= 1;
            }
            b'+' => {
                if !b"EIY".contains(&c) {
                    return false;
                }
                at -= 1;
            }
            b'&' => {
                let two = (get(at - 1), c);
                if matches!(two, (b'C', b'H') | (b'S', b'H')) {
                    at -= 2;
                } else if b"SCGZXJ".contains(&c) {
                    at -= 1;
                } else {
                    return false;
                }
            }
            b'@' => {
                let two = (get(at - 1), c);
                if matches!(two, (b'T', b'H') | (b'C', b'H') | (b'S', b'H')) {
                    at -= 2;
                } else if b"TSRDLZNJ".contains(&c) {
                    at -= 1;
                } else {
                    return false;
                }
            }
            b' ' => {
                if c != b' ' {
                    return false;
                }
                at -= 1;
            }
            b'\'' => {
                if c != b'\'' {
                    return false;
                }
                at -= 1;
            }
            _ => {
                if c != p {
                    return false;
                }
                at -= 1;
            }
        }
    }
    true
}

/// Letter-to-sound conversion of one word.
pub fn rules(word: &str) -> Vec<Phoneme> {
    let upper: Vec<u8> = word
        .chars()
        .filter(|c| c.is_ascii_alphabetic() || *c == '\'')
        .map(|c| c.to_ascii_uppercase() as u8)
        .collect();
    let mut out = Vec::new();
    let mut at = 0;
    while at < upper.len() {
        let c = upper[at];
        if !c.is_ascii_uppercase() {
            at += 1;
            continue;
        }
        let rule = RULES.iter().find(|(left, text, right, _)| {
            let text = text.as_bytes();
            text[0] == c
                && upper[at..].starts_with(text)
                && match_left(&upper, at, left.as_bytes())
                && match_right(&upper, at + text.len(), right.as_bytes())
        });
        match rule {
            Some((_, text, _, phonemes)) => {
                out.extend(phonemes.split_whitespace().map(ph));
                at += text.len();
            }
            None => at += 1,
        }
    }
    out
}

/// Phonemes of one word: dictionary, then suffixes, then rules.
pub fn word(text: &str, extra: Option<&Dictionary>) -> Vec<Phoneme> {
    if normalize(text).is_empty() {
        return Vec::new();
    }
    lookup(text, extra)
        .or_else(|| suffixed(text, extra))
        .unwrap_or_else(|| rules(text))
}

pub fn syllables(text: &str, extra: Option<&Dictionary>) -> Vec<Syllable> {
    text.split_whitespace()
        .flat_map(|w| syllabify(&word(w, extra)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn show(p: Vec<Phoneme>) -> String {
        p.iter().map(|p| p.symbol()).collect::<Vec<_>>().join(" ")
    }
    #[test]
    fn dictionary_and_suffixes() {
        assert_eq!(show(word("Love", None)), "l ah v");
        assert_eq!(show(word("don't", None)), "d ow n t");
        assert_eq!(show(word("dreaming", None)), "d r iy m ih ng");
        assert_eq!(show(word("walked", None)), "w ao k t");
        assert_eq!(show(word("hearts", None)), "hh aa r t s");
        assert_eq!(show(word("nothin'", None)), "n ah th ih ng");
        assert_eq!(show(word("kisses", None)), "k ih s ih z");
    }
    #[test]
    fn letter_to_sound_rules() {
        assert_eq!(show(rules("cat")), "k ae t");
        assert_eq!(show(rules("ship")), "sh ih p");
        assert_eq!(show(rules("night")), "n ay t");
        assert_eq!(show(rules("phone")), "f ow n");
        assert_eq!(show(rules("make")), "m ey k");
        assert_eq!(show(rules("glow")), "g l ow");
    }
    #[test]
    fn cmudict_format() {
        let d =
            Dictionary::parse(";;; comment\nSKY  S K AY1\nSKY(2)  S K IY1\nNEON  N IY1 AA0 N\n");
        assert_eq!(d.len(), 2);
        assert_eq!(show(word("neon", Some(&d))), "n iy aa n");
        assert_eq!(show(word("sky", Some(&d))), "s k ay");
    }
}
