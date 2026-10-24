//! 128 original, deterministic sources. Each family has sixteen authored
//! arrangements; pitches, rhythms, articulation and synthesis vary together.
//! No recordings, soundfonts, external downloads or runtime audio dependencies.
use super::{Bank, Tone, Track, hz, sine};
use crate::dsp::SampleBuffer;
use std::f64::consts::TAU;

macro_rules! catalog {
    ($($category:literal => [$(($id:literal, $name:literal, $bpm:literal, $character:literal)),+ $(,)?]),+ $(,)?) => {
        pub(in crate::samples) const TRACKS: [Track; 128] = [
            $($(super::super::loop_track($id, $name, $character, $bpm, $category)),+),+
        ];
    };
}

catalog! {
    "Drums" => [
        ("warehouse-stomp", "Warehouse stomp", 130, "Heavy four-to-the-floor, claps & open hats"),
        ("broken-concrete", "Broken concrete", 138, "Broken kicks, sharp snares & syncopated hats"),
        ("velvet-boom-bap", "Velvet boom bap", 88, "Swung kicks, backbeat & ghost notes"),
        ("tape-disco", "Tape disco", 116, "Disco kick, claps & sixteenth-note hats"),
        ("half-time-shadow", "Half-time shadow", 144, "Sparse half-time drums & low tom answers"),
        ("garage-shuffle", "Garage shuffle", 132, "Shuffled two-step drums & rim accents"),
        ("jungle-switch", "Jungle switch", 168, "Fast broken drums & rolling snare fills"),
        ("dawn-downtempo", "Dawn downtempo", 76, "Slow deep kick, soft snare & brushed noise"),
        ("motorik-room", "Motorik room", 122, "Steady kick train & driving closed hats"),
        ("electro-skips", "Electro skips", 126, "Syncopated machine beat & pitched toms"),
        ("trap-lattice", "Trap lattice", 150, "Half-time backbeat, hat rolls & sub kicks"),
        ("footwork-grid", "Footwork grid", 160, "Displaced kicks, claps & triplet fills"),
        ("dub-stepper", "Dub stepper", 72, "Spacious kick, snare & rim echoes"),
        ("minimal-pulse", "Minimal pulse", 125, "Dry kick, sparse rim & tick pattern"),
        ("industrial-march", "Industrial march", 108, "Stomping drums & metallic percussion"),
        ("sunset-latin", "Sunset Latin", 104, "Offbeat kick, hand-drum accents & shaker"),
    ],
    "Percussion" => [
        ("conga-courtyard", "Conga courtyard", 108, "Alternating tuned congas & muted strokes"),
        ("bongo-conversation", "Bongo conversation", 118, "High and low bongos in call and response"),
        ("woodblock-weave", "Woodblock weave", 96, "Interlocking wooden clicks"),
        ("rimshot-clock", "Rimshot clock", 120, "Dry rimshots with displaced accents"),
        ("shaker-sand", "Shaker sand", 102, "Soft stereo shaker, uneven accents"),
        ("tambourine-trail", "Tambourine trail", 112, "Jingling metal hits & rolling doubles"),
        ("cowbell-crossing", "Cowbell crossing", 116, "Two-tone cowbells & clave rhythm"),
        ("clave-spiral", "Clave spiral", 98, "3–2 wooden clave with answering taps"),
        ("frame-drum-circle", "Frame drum circle", 84, "Low resonant skins & fingertip taps"),
        ("tabla-orbit", "Tabla orbit", 92, "Synthesized bending drum skins"),
        ("steel-pan-drops", "Steel pan drops", 100, "Tuned metallic droplets & rolling answers"),
        ("ceramic-steps", "Ceramic steps", 86, "Hollow ceramic taps in alternating registers"),
        ("brushwork", "Brushwork", 78, "Filtered brush swishes & short taps"),
        ("found-metal", "Found metal", 110, "Clanging bars, bolts & damped resonances"),
        ("water-drum", "Water drum", 90, "Round drops with rising and falling pitch"),
        ("handclap-ensemble", "Handclap ensemble", 106, "Layered handclaps, flams & soft responses"),
    ],
    "Vocals" => [
        ("humming-lantern", "Humming lantern", 72, "Synthetic closed-mouth hum, gentle descending phrase"),
        ("alto-ah", "Alto ah", 84, "Warm formant alto, open ah melody"),
        ("tenor-oo", "Tenor oo", 76, "Low rounded oo with soft pitch scoops"),
        ("soprano-air", "Soprano air", 90, "High breathy ee–ah phrase"),
        ("choir-tide", "Choir tide", 64, "Three-part synthetic vowel swells"),
        ("doo-wop-moon", "Doo-wop moon", 96, "Short doo–ah answers with swung timing"),
        ("wordless-waltz", "Wordless turns", 108, "Lilting wordless melody with three-note groupings"),
        ("breathy-lullaby", "Breathy lullaby", 68, "Airy, unhurried humming melody"),
        ("nasal-chant", "Nasal chant", 88, "Narrow nasal formants & repeated chant notes"),
        ("vowel-carousel", "Vowel carousel", 112, "Ah–eh–ee–oh–oo changing syllables"),
        ("robot-scat", "Robot scat", 120, "Dry staccato formant syllables"),
        ("low-monks", "Low monks", 60, "Low synthetic chant with an octave response"),
        ("bright-la", "Bright la", 104, "Bright syllable onsets & climbing melody"),
        ("whisper-song", "Whisper song", 80, "Breath-heavy vowels & soft melodic fragments"),
        ("vocal-echoes", "Vocal echoes", 92, "Wordless call and delayed stereo answers"),
        ("small-choir", "Small choir", 74, "Layered alto vowels & upper harmony"),
    ],
    "Bass" => [
        ("subway-sub", "Subway sub", 128, "Rounded sub bass with syncopated rests"),
        ("rubber-funk", "Rubber funk", 106, "Springy octave bass & short slides"),
        ("acid-zigzag", "Acid zigzag", 134, "Resonant saw sequence with alternating accents"),
        ("reese-current", "Reese current", 170, "Detuned bass intervals & sustained growl"),
        ("dub-pressure", "Dub pressure", 74, "Deep, spacious sine bass with long releases"),
        ("fm-knuckle", "FM knuckle", 118, "Metallic FM bass & percussive attacks"),
        ("picked-copper", "Picked copper", 102, "Plucked harmonic bass & string-like decay"),
        ("square-cellar", "Square cellar", 124, "Hollow square bass with clipped articulation"),
        ("octave-runner", "Octave runner", 126, "Disco octaves & syncopated passing notes"),
        ("bowed-floor", "Bowed floor", 70, "Slow bowed low harmonics"),
        ("wobble-engine", "Wobble engine", 140, "Rhythmic harmonic bass swells"),
        ("warm-mono", "Warm mono", 94, "Soft mono bass, long roots & short answers"),
        ("pulse-crawler", "Pulse crawler", 114, "Narrow pulse bass with descending steps"),
        ("resonant-rubber", "Resonant rubber", 122, "Sliding resonant bass & accented offbeats"),
        ("broken-sub", "Broken sub", 136, "Broken low sine rhythm with octave pickups"),
        ("bass-harmonics", "Bass harmonics", 82, "Chiming upper bass partials & low anchors"),
    ],
    "Keys" => [
        ("felt-window", "Felt window", 76, "Soft felt-piano broken chords"),
        ("rhodes-at-dusk", "Rhodes at dusk", 88, "Electric tine keys & minor sevenths"),
        ("wurlitzer-walk", "Wurlitzer walk", 104, "Reedy electric keys with syncopated stabs"),
        ("celesta-snow", "Celesta snow", 72, "Bright, short celestial key strikes"),
        ("toy-piano-attic", "Toy piano attic", 96, "Tiny detuned piano tines"),
        ("organ-lantern", "Organ lantern", 92, "Drawbar-like sustained organ chords"),
        ("clavinet-corners", "Clavinet corners", 112, "Dry, hollow clavinet-style offbeats"),
        ("fm-diary", "FM diary", 84, "Bell-edged FM keyboard arpeggio"),
        ("tape-ballad", "Tape ballad", 68, "Slow fluttering piano & held chord tones"),
        ("detuned-parlor", "Detuned parlor", 100, "Uneven honky-tonk unisons"),
        ("glass-electric", "Glass electric", 110, "Glassy electric keys with high answers"),
        ("soft-chord-stabs", "Soft chord stabs", 120, "Short minor keyboard chords & dotted echoes"),
        ("midnight-voicings", "Midnight voicings", 78, "Spacious piano sevenths and ninths"),
        ("bright-organ", "Bright organ", 118, "Percussive organ with upper drawbars"),
        ("rainy-arpeggio", "Rainy arpeggio", 90, "Falling felt-key pattern in stereo"),
        ("broken-music-hall", "Broken music hall", 108, "Playful piano syncopation & octave turns"),
    ],
    "Plucks & mallets" => [
        ("marimba-path", "Marimba path", 104, "Warm wooden mallet melody"),
        ("kalimba-mosaic", "Kalimba mosaic", 96, "Interlocking thumb-piano patterns"),
        ("nylon-courtyard", "Nylon courtyard", 86, "Soft nylon arpeggio & plucked bass notes"),
        ("harp-fireflies", "Harp fireflies", 78, "Rising harp-like string harmonics"),
        ("koto-bridge", "Koto bridge", 92, "Bright, bent string plucks"),
        ("dulcimer-dance", "Dulcimer dance", 116, "Doubled hammered strings & quick answers"),
        ("vibraphone-night", "Vibraphone night", 80, "Soft metal bars with tremolo"),
        ("xylophone-puzzle", "Xylophone puzzle", 122, "Short high wooden mallets"),
        ("glockenspiel-stars", "Glockenspiel stars", 98, "High bell tines & descending answers"),
        ("muted-guitar-grid", "Muted guitar grid", 114, "Damped string plucks in a syncopated grid"),
        ("sitar-sparks", "Sitar sparks", 106, "Buzzing harmonic plucks & resonant tails"),
        ("mbira-sun", "Mbira sun", 108, "Low thumb-piano tines & upper interlocks"),
        ("bell-tree", "Bell tree", 74, "Cascading inharmonic bell notes"),
        ("pizzicato-garden", "Pizzicato garden", 100, "Rounded pizzicato-like string pattern"),
        ("plucked-circuit", "Plucked circuit", 128, "Bright digital FM plucks"),
        ("bowl-constellation", "Bowl constellation", 66, "Long bronze tones in a sparse melody"),
    ],
    "Pads & strings" => [
        ("amber-horizon", "Amber horizon", 60, "Warm detuned chord swells"),
        ("frozen-strings", "Frozen strings", 68, "Thin high bowed harmonics"),
        ("velvet-cinema", "Velvet cinema", 72, "Low string chords & slow upper voices"),
        ("organ-cloud", "Organ cloud", 64, "Floating, hollow organ pad"),
        ("fifth-drifter", "Fifth drifter", 76, "Open fifths and moving spectral color"),
        ("choral-glass", "Choral glass", 80, "Synthetic vowel pad with glassy overtones"),
        ("analog-sunset", "Analog sunset", 70, "Saw-like ensemble with slow detuning"),
        ("moonlit-cellos", "Moonlit cellos", 62, "Low bowed unisons & a rising inner voice"),
        ("hollow-sky", "Hollow sky", 84, "Odd-harmonic chords in a wide stereo field"),
        ("shimmer-meadow", "Shimmer meadow", 66, "Soft chords with a bright octave layer"),
        ("slow-brass", "Slow brass", 78, "Rounded brass-like swells"),
        ("tremolo-canopy", "Tremolo canopy", 96, "Bowed chords with measured tremolo"),
        ("minor-nebula", "Minor nebula", 58, "Dark minor ninths with a long decay"),
        ("suspended-orbit", "Suspended orbit", 74, "Suspended chords & bell-like upper partials"),
        ("rising-ensemble", "Rising ensemble", 88, "Rising chord inversions & string harmonics"),
        ("distant-reeds", "Distant reeds", 82, "Soft reed-like chords with a slow attack"),
    ],
    "Textures & FX" => [
        ("rain-on-copper", "Rain on copper", 80, "Synthesized rain bed & tiny metal droplets"),
        ("wind-in-wires", "Wind in wires", 64, "Filtered wind & whistling harmonics"),
        ("underwater-pings", "Underwater pings", 72, "Low water noise & sonar-like echoes"),
        ("tape-splice", "Tape splice", 98, "Hiss, wow, clicks & chopped tone fragments"),
        ("crystal-cave", "Crystal cave", 60, "Sparse crystalline strikes in stereo echoes"),
        ("satellite-birds", "Satellite birds", 100, "Synthetic chirps & answering radio bleeps"),
        ("electric-insects", "Electric insects", 112, "Buzzing pulses & high rhythmic trills"),
        ("vinyl-storm", "Vinyl storm", 86, "Crackle clusters, low rumble & surface hiss"),
        ("granite-scrape", "Granite scrape", 90, "Rough filtered-noise scrapes & impacts"),
        ("liquid-machine", "Liquid machine", 118, "Pitch-bending droplets & bubbling tones"),
        ("digital-confetti", "Digital confetti", 132, "Short FM fragments, ticks & stutters"),
        ("distant-machinery", "Distant machinery", 70, "Low mechanical harmonics & metallic knocks"),
        ("steam-valves", "Steam valves", 106, "Hissing pressure bursts & muted pipe hits"),
        ("reverse-lanterns", "Reverse lanterns", 92, "Reversed resonant plucks with wrapped tails"),
        ("magnetic-dust", "Magnetic dust", 78, "Soft magnetic hum, granular crackle & airy swells"),
        ("space-telegraph", "Space telegraph", 120, "Alternating radio tones, noise & dotted echoes"),
    ],
}

// Semitone phrases include rests (-1); shared musical vocabulary is voiced
// differently by each family, with separate rhythm, register and articulation.
const PHRASES: [[i8; 16]; 16] = [
    [0, -1, 7, 12, 10, -1, 7, 3, 0, 7, -1, 10, 12, 7, 3, -1],
    [0, 3, 7, -1, 10, 7, 12, -1, 5, 8, 12, 15, 12, -1, 8, 7],
    [0, 0, -1, 7, 3, -1, 10, 7, 0, 12, 10, -1, 7, 3, 2, -1],
    [12, 7, 3, 0, -1, 3, 7, 10, 14, 10, 7, 3, -1, 2, 5, 7],
    [0, -1, -1, 7, 12, -1, 10, -1, 8, -1, 5, -1, 3, 7, 2, -1],
    [0, 7, 0, 3, -1, 10, 7, -1, 5, 12, 5, 8, -1, 15, 12, 7],
    [0, 2, 3, 7, 5, 3, 2, -1, 7, 10, 12, 14, 12, 10, 7, -1],
    [12, -1, 10, 7, -1, 5, 3, -1, 0, -1, 3, 5, 7, -1, 2, -1],
    [0, 12, 0, -1, 7, 19, 7, -1, 3, 15, 3, 10, 7, 12, 10, -1],
    [0, -1, 3, -1, 7, -1, 10, 12, 8, -1, 5, -1, 3, -1, 2, 7],
    [0, 7, 10, 12, -1, 7, 3, 2, 0, -1, 12, 10, 7, -1, 5, 3],
    [0, -1, 0, 7, -1, 3, 0, -1, 5, -1, 5, 12, 8, 7, -1, 3],
    [0, 3, 5, 7, -1, 10, 12, 15, 14, 12, -1, 10, 7, 5, 3, 2],
    [7, -1, 3, 0, -1, 10, 7, -1, 12, -1, 8, 5, -1, 7, 2, 0],
    [0, 7, -1, 3, 10, -1, 12, 7, 5, 12, -1, 8, 15, 12, 7, -1],
    [12, -1, 7, -1, 3, 0, -1, 7, 10, -1, 5, -1, 2, 7, 0, -1],
];
const ROOTS: [u8; 16] = [
    48, 50, 45, 55, 48, 53, 50, 43, 48, 46, 51, 45, 50, 53, 48, 43,
];
const CHORDS: [[u8; 4]; 8] = [
    [0, 3, 7, 10],
    [0, 4, 7, 11],
    [0, 5, 7, 12],
    [0, 3, 7, 14],
    [0, 7, 12, 19],
    [0, 2, 7, 10],
    [0, 4, 9, 14],
    [0, 3, 8, 12],
];

const DRUMS: [(u16, u16, u16, f64); 16] = [
    (0x1111, 0x1010, 0x4444, 0.0),
    (0x0941, 0x1010, 0xdddd, 0.02),
    (0x0409, 0x1010, 0x5555, 0.17),
    (0x1111, 0x1010, 0xffff, 0.04),
    (0x0041, 0x0100, 0x5155, 0.0),
    (0x0841, 0x1010, 0xeeee, 0.16),
    (0x0921, 0x1290, 0xffff, 0.025),
    (0x0401, 0x1010, 0x4545, 0.12),
    (0x1111, 0x1010, 0x5555, 0.0),
    (0x0849, 0x1010, 0x5d55, 0.04),
    (0x0421, 0x0100, 0xf5d5, 0.0),
    (0x2941, 0x1040, 0x5555, 0.02),
    (0x0101, 0x1010, 0x4444, 0.08),
    (0x1111, 0x1000, 0x4404, 0.0),
    (0x0505, 0x1010, 0xaaaa, 0.02),
    (0x0449, 0x1010, 0xdddd, 0.1),
];

fn drums(bank: &mut Bank, v: usize) {
    let (kick, snare, hats, swing) = DRUMS[v];
    for bar in 0..4 {
        for step in 0..16 {
            let beat =
                bar as f64 * 4.0 + step as f64 * 0.25 + if step % 2 == 1 { swing } else { 0.0 };
            if kick & (1 << step) != 0 {
                bank.kick(beat, if step == 0 { 0.85 } else { 0.65 }, v % 3 != 1);
            }
            if snare & (1 << step) != 0 {
                if v == 13 {
                    percussion(bank, beat, 3, 440.0, 0.4, 0.5);
                } else {
                    bank.snare(beat, 0.5, matches!(v, 0 | 3 | 5 | 11));
                }
            }
            if hats & (1 << step) != 0 {
                bank.hat(
                    beat,
                    if step % 4 == 2 { 0.14 } else { 0.085 },
                    step % 8 == 6 && !matches!(v, 2 | 7 | 13),
                    0.25 + (step % 3) as f64 * 0.2,
                );
            }
            if (bar + step + v) % 13 == 9 {
                percussion(
                    bank,
                    beat + 0.125,
                    v % 16,
                    140.0 + v as f64 * 23.0,
                    0.12,
                    0.7,
                );
            }
        }
        if bar % 2 == 1 {
            for fill in 0..=v % 4 {
                let beat = bar as f64 * 4.0 + 3.5 + fill as f64 / 8.0;
                bank.snare(beat, 0.12 + fill as f64 * 0.04, false);
            }
        }
        if matches!(v, 6 | 10 | 11) {
            for roll in 0..6 {
                bank.hat(
                    bar as f64 * 4.0 + 2.5 + roll as f64 / 12.0,
                    0.06 + roll as f64 * 0.009,
                    false,
                    0.65,
                );
            }
        }
    }
    if v == 12 {
        bank.echo(bank.beat * 0.75, 0.18, 3);
    }
}

fn percussion(bank: &mut Bank, beat: f64, kind: usize, frequency: f64, level: f64, pan: f64) {
    if kind == 15 {
        bank.snare(beat, level, true);
        return;
    }
    let decay = [
        0.16, 0.10, 0.035, 0.025, 0.055, 0.18, 0.11, 0.028, 0.32, 0.19, 0.4, 0.12, 0.22, 0.36, 0.14,
    ][kind];
    let nyquist = bank.rate as f64 * 0.42;
    let rate = bank.rate as f64;
    let mut low = 0.0;
    let filter = 1.0 - (-TAU * if kind == 12 { 1800.0 } else { 4300.0 } / rate).exp();
    bank.event(beat * bank.beat, decay * 7.0, pan, |t, random| {
        let noise = random.bipolar() as f64;
        low += (noise - low) * filter;
        let env = (1.0 - (-t * 2400.0).exp()) * (-t / decay).exp();
        let bend = match kind {
            0 | 1 | 8 | 9 => 2.8,
            14 => -3.5,
            _ => 0.0,
        };
        let phase = t * frequency + bend * (1.0 - (-t * 25.0).exp());
        let sound = match kind {
            4 | 12 => (noise - low * 0.8) * (0.7 + 0.3 * sine(t * 37.0)),
            5 => {
                let metal: f64 = [1.0, 1.42, 1.87, 2.53]
                    .iter()
                    .filter(|r| frequency * **r < nyquist)
                    .map(|r| sine(phase * r))
                    .sum();
                metal * 0.22 + (noise - low) * 0.4
            }
            6 | 10 | 13 => {
                let ratios = if kind == 6 {
                    [1.0, 1.47, 2.09]
                } else {
                    [1.0, 2.32, 3.81]
                };
                ratios
                    .iter()
                    .enumerate()
                    .filter(|(_, r)| frequency * **r < nyquist)
                    .map(|(i, r)| sine(phase * r) * (-t * i as f64 * 8.0).exp() / (i + 1) as f64)
                    .sum()
            }
            _ => sine(phase) + 0.35 * sine(phase * 1.59) * (-t * 30.0).exp() + low * 0.12,
        };
        sound * env * level
    });
}

fn percussion_loop(bank: &mut Bank, v: usize) {
    let patterns = [
        0x4929_u16, 0xa549, 0x2493, 0x1014, 0xffff, 0x555d, 0x4925, 0x0929, 0x1149, 0x4b25, 0x5295,
        0x2529, 0x5555, 0x8911, 0x2a49, 0x1451,
    ];
    let frequency = [
        180., 310., 840., 1150., 2200., 1700., 570., 1350., 95., 230., 390., 680., 900., 270.,
        480., 300.,
    ][v];
    for bar in 0..4 {
        for step in 0..16 {
            if patterns[v].rotate_left((bar % 2 * (v % 3)) as u32) & (1 << step) == 0 {
                continue;
            }
            let beat = bar as f64 * 4.0
                + step as f64 * 0.25
                + if step % 2 == 1 {
                    (v % 4) as f64 * 0.025
                } else {
                    0.0
                };
            let accent = if step % 4 == 0 { 0.65 } else { 0.36 };
            percussion(
                bank,
                beat,
                v,
                frequency * if step % 3 == 0 { 1.0 } else { 1.32 },
                accent,
                if step % 2 == 0 { 0.3 } else { 0.7 },
            );
            if (step + bar) % 7 == 6 {
                percussion(bank, beat + 0.125, v, frequency * 0.75, 0.18, 0.6);
            }
        }
    }
    if matches!(v, 10 | 11 | 14) {
        bank.echo(bank.beat * 0.75, 0.24, 3);
    }
}

// A bandlimited glottal spectrum shaped by three vowel formants. Wavetables
// keep phrase preparation inexpensive, including on phones, while allowing
// continuous vowel morphs, scoop, vibrato, breath and consonant-like onsets.
fn vowel_table(note: u8, vowel: usize, nasal: bool, rate: u32) -> Vec<f64> {
    let formants = [
        [800., 1150., 2900.],
        [500., 1750., 2450.],
        [300., 2200., 3000.],
        [500., 800., 2600.],
        [350., 650., 2400.],
    ];
    let frequency = hz(note);
    let partials: Vec<_> = (1..=48)
        .filter_map(|h| {
            let f = frequency * h as f64;
            if f >= rate as f64 * 0.42 {
                return None;
            }
            let mut gain = 0.04;
            for (i, center) in formants[vowel].iter().enumerate() {
                gain += (-0.5 * ((f - center) / [100., 150., 220.][i]).powi(2)).exp();
            }
            if nasal {
                gain *= if f < 700. { 1.6 } else { 0.23 };
            }
            Some((h as f64, gain / (h as f64).powf(1.15)))
        })
        .collect();
    let mut table: Vec<f64> = (0..1024)
        .map(|i| {
            partials
                .iter()
                .map(|(h, gain)| sine(i as f64 / 1024.0 * h) * gain)
                .sum()
        })
        .collect();
    let peak = table.iter().fold(0.001_f64, |a, b| a.max(b.abs()));
    for value in &mut table {
        *value /= peak;
    }
    table
}

fn vocal(bank: &mut Bank, beat: f64, note: u8, v: usize, syllable: usize, duration: f64, pan: f64) {
    let vowel = match v {
        0 | 2 | 7 | 11 => 4,
        1 | 4 | 12 | 15 => 0,
        3 => 2,
        _ => syllable % 5,
    };
    let a = vowel_table(note, vowel, matches!(v, 0 | 7 | 8), bank.rate);
    let b = vowel_table(
        note,
        if v == 9 { (vowel + 1) % 5 } else { vowel },
        matches!(v, 0 | 7 | 8),
        bank.rate,
    );
    let frequency = hz(note);
    let rate = bank.rate as f64;
    let mut phase = 0.0_f64;
    let mut breath = 0.0;
    let read = |table: &[f64], phase: f64| {
        let at = phase * 1024.0;
        let i = at as usize & 1023;
        table[i] + (table[(i + 1) & 1023] - table[i]) * at.fract()
    };
    bank.event(beat * bank.beat, duration, pan, |t, random| {
        let vibrato = if v == 10 {
            0.0
        } else {
            sine(t * (4.8 + v as f64 * 0.035)) * 0.005 * (t * 4.0).min(1.0)
        };
        phase = (phase + frequency / rate * (1.0 + vibrato - 0.035 * (-t * 30.0).exp())).fract();
        let morph = (t / (duration * 0.7)).clamp(0.0, 1.0);
        let voice = read(&a, phase) * (1.0 - morph) + read(&b, phase) * morph;
        breath += (random.bipolar() as f64 - breath) * 0.32;
        let airy = if matches!(v, 3 | 7 | 13) { 0.23 } else { 0.035 };
        let onset = if matches!(v, 5 | 10 | 12) {
            0.012
        } else {
            0.065
        };
        let env = (1.0 - (-t / onset).exp()) * ((duration - t) / 0.12).clamp(0.0, 1.0);
        (voice + breath * airy + breath * (-t * 60.0).exp() * 0.35) * env * 0.35
    });
}

fn vocals(bank: &mut Bank, v: usize) {
    let root = [
        55, 60, 48, 69, 55, 57, 62, 55, 50, 60, 55, 38, 64, 57, 60, 55,
    ][v];
    for (i, &interval) in PHRASES[v].iter().enumerate() {
        if interval < 0 {
            continue;
        }
        let beat = i as f64 + if i % 2 == 1 && v == 5 { 0.16 } else { 0.0 };
        let duration = bank.beat * if matches!(v, 5 | 10 | 12) { 0.48 } else { 1.35 };
        vocal(bank, beat, root + interval as u8, v, i, duration, 0.45);
        if matches!(v, 4 | 15) {
            vocal(
                bank,
                beat + 0.04,
                root + interval as u8 + 7,
                v,
                i + 1,
                duration * 1.3,
                0.8,
            );
            vocal(
                bank,
                beat + 0.02,
                root + interval as u8 - 12,
                v,
                i + 2,
                duration * 1.1,
                0.2,
            );
        }
    }
    bank.echo(
        bank.beat * if v == 14 { 1.5 } else { 0.75 },
        if v == 14 { 0.4 } else { 0.16 },
        3,
    );
}

#[derive(Clone, Copy)]
enum Color {
    Sub,
    Reed,
    Saw,
    Fm,
    String,
    Square,
    Organ,
    Glass,
}

#[allow(clippy::too_many_arguments)]
fn pitched(
    bank: &mut Bank,
    beat: f64,
    note: u8,
    color: Color,
    duration: f64,
    slow: bool,
    movement: f64,
    pan: f64,
) {
    let frequency = hz(note);
    let rate = bank.rate as f64;
    let harmonics = (rate * 0.42 / frequency).floor().min(12.0) as usize;
    let partials: Vec<_> = (1..=harmonics)
        .map(|h| {
            let n = h as f64;
            let gain = match color {
                Color::Sub => {
                    if h == 1 {
                        1.0
                    } else if h == 2 {
                        0.12
                    } else {
                        0.0
                    }
                }
                Color::Reed | Color::Square => {
                    if h % 2 == 1 {
                        1.0 / n
                    } else {
                        0.0
                    }
                }
                Color::Organ => {
                    if matches!(h, 1 | 2 | 3 | 4 | 6 | 8) {
                        1.0 / n.sqrt()
                    } else {
                        0.0
                    }
                }
                Color::Glass => {
                    if h == 1 {
                        1.0
                    } else {
                        0.5 / n
                    }
                }
                _ => 1.0 / n.powf(1.25),
            };
            (n, gain)
        })
        .collect();
    bank.event(beat * bank.beat, duration, pan, |t, _| {
        let phase = t * frequency + sine(t * 0.63) * movement * 0.025;
        let env = if slow {
            (t / (duration * 0.22).min(1.4)).min(1.0)
                * ((duration - t) / (duration * 0.3)).clamp(0.0, 1.0)
        } else {
            (1.0 - (-t * 900.0).exp()) * (-t / (duration * 0.3)).exp()
        };
        let signal = match color {
            Color::Fm => sine(phase + sine(phase * 2.0) * (2.5 * (-t * 6.0).exp() + 0.1)),
            _ => partials
                .iter()
                .map(|(h, gain)| {
                    let ratio = if matches!(color, Color::Glass) {
                        h + (h - 1.0) * 0.013
                    } else {
                        *h
                    };
                    let damping = if slow {
                        0.7 + 0.3 * sine(t * 0.19 + h * 0.3)
                    } else {
                        (-t * (h - 1.0) * 3.0).exp()
                    };
                    sine(phase * ratio) * gain * damping
                })
                .sum::<f64>(),
        };
        let tremolo = 1.0 - movement.abs().min(0.8) * (0.5 + 0.5 * sine(t * (3.5 + movement)));
        signal * env * tremolo * 0.23
    });
}

fn basses(bank: &mut Bank, v: usize) {
    let color = [
        Color::Sub,
        Color::Reed,
        Color::Saw,
        Color::Saw,
        Color::Sub,
        Color::Fm,
        Color::String,
        Color::Square,
        Color::Reed,
        Color::String,
        Color::Saw,
        Color::Sub,
        Color::Square,
        Color::Saw,
        Color::Sub,
        Color::Glass,
    ][v];
    let root = ROOTS[v] - 12;
    let mut previous = root;
    for step in 0..32 {
        let interval = PHRASES[v][step % 16];
        if interval < 0 {
            continue;
        }
        let note = root + interval as u8;
        let beat = step as f64 * 0.5 + if step % 2 == 1 && v == 1 { 0.12 } else { 0.0 };
        let duration = bank.beat * if matches!(v, 3 | 4 | 9) { 1.6 } else { 0.65 };
        if matches!(v, 2 | 13) {
            bank.bass(beat, note, duration, step % 3 == 0, previous);
        } else {
            pitched(
                bank,
                beat,
                note,
                color,
                duration,
                v == 9,
                if v == 10 { 0.75 } else { 0.08 },
                0.5,
            );
            if v == 3 {
                pitched(
                    bank,
                    beat + 0.012,
                    note,
                    Color::Saw,
                    duration,
                    false,
                    -0.65,
                    0.7,
                );
            }
        }
        previous = note;
    }
}

fn keys(bank: &mut Bank, v: usize) {
    for (step, &interval) in PHRASES[v].iter().enumerate() {
        if interval < 0 {
            continue;
        }
        let beat = step as f64
            + if step % 2 == 1 {
                (v % 3) as f64 * 0.055
            } else {
                0.0
            };
        let note = ROOTS[v] + 12 + interval as u8;
        let pan = 0.25 + (step % 4) as f64 * 0.16;
        match v {
            0 | 8 | 12 | 14 | 15 => bank.note(
                beat,
                note,
                Tone::Piano,
                if v == 8 { 1.1 } else { 0.55 },
                pan,
                0.3,
            ),
            1 => bank.note(beat, note, Tone::Keys, 0.85, pan, 0.35),
            3 | 4 | 10 => bank.note(
                beat,
                note + if v == 3 { 12 } else { 0 },
                Tone::Box,
                0.3,
                pan,
                0.28,
            ),
            5 | 13 => pitched(
                bank,
                beat,
                note,
                Color::Organ,
                bank.beat * 1.3,
                v == 5,
                0.12,
                pan,
            ),
            2 | 6 => pitched(
                bank,
                beat,
                note,
                Color::Reed,
                bank.beat * 0.6,
                false,
                0.1,
                pan,
            ),
            7 => pitched(
                bank,
                beat,
                note,
                Color::Fm,
                bank.beat * 1.5,
                false,
                0.0,
                pan,
            ),
            9 => {
                bank.note(beat, note, Tone::Piano, 0.4, 0.3, 0.25);
                pitched(
                    bank,
                    beat + 0.008,
                    note,
                    Color::Glass,
                    bank.beat,
                    false,
                    0.45,
                    0.7,
                );
            }
            _ => bank.note(beat, note, Tone::Keys, 0.22, pan, 0.32),
        }
        if step % if matches!(v, 11 | 12) { 2 } else { 4 } == 0 {
            let chord = CHORDS[v % CHORDS.len()];
            for (j, interval) in chord.iter().enumerate() {
                bank.note(
                    beat + j as f64 * 0.025,
                    ROOTS[v] + interval,
                    Tone::Keys,
                    if v == 11 { 0.18 } else { 0.8 },
                    0.2 + j as f64 * 0.2,
                    0.12,
                );
            }
        }
    }
    bank.echo(bank.beat * 0.75, if v == 11 { 0.32 } else { 0.18 }, 3);
}

fn plucks(bank: &mut Bank, v: usize) {
    let tone = [
        Tone::Marimba,
        Tone::Kalimba,
        Tone::Nylon,
        Tone::Nylon,
        Tone::Nylon,
        Tone::Piano,
        Tone::Bell,
        Tone::Marimba,
        Tone::Box,
        Tone::Nylon,
        Tone::Nylon,
        Tone::Kalimba,
        Tone::Bell,
        Tone::Nylon,
        Tone::Marimba,
        Tone::Bowl,
    ][v];
    let decay = [
        0.35, 0.7, 0.55, 1.4, 0.5, 0.4, 1.2, 0.16, 0.7, 0.09, 0.8, 0.5, 1.8, 0.18, 0.23, 2.4,
    ][v];
    for step in 0..32 {
        let interval = PHRASES[v][step % 16];
        if interval < 0 {
            continue;
        }
        let note = ROOTS[v] + if v == 15 { 0 } else { 12 } + interval as u8;
        let beat = step as f64 * 0.5
            + if step % 2 == 1 {
                (v % 3) as f64 * 0.04
            } else {
                0.0
            };
        let pan = 0.2 + (step % 5) as f64 * 0.15;
        bank.note(beat, note, tone, decay, pan, 0.28);
        if matches!(v, 4 | 6 | 10 | 14) {
            pitched(
                bank,
                beat + 0.01,
                note,
                if v == 14 { Color::Fm } else { Color::String },
                decay * 3.0,
                false,
                if v == 6 { 0.65 } else { 0.3 },
                1.0 - pan,
            );
        }
        if v == 5 {
            bank.note(beat + 0.08, note, tone, decay, 1.0 - pan, 0.12);
        }
    }
    bank.echo(
        bank.beat * if v.is_multiple_of(2) { 0.75 } else { 1.5 },
        0.23,
        3,
    );
}

fn pads(bank: &mut Bank, v: usize) {
    let color = [
        Color::Saw,
        Color::String,
        Color::String,
        Color::Organ,
        Color::Glass,
        Color::Reed,
        Color::Saw,
        Color::String,
        Color::Square,
        Color::Glass,
        Color::Reed,
        Color::String,
        Color::Saw,
        Color::Glass,
        Color::String,
        Color::Reed,
    ][v];
    let root = ROOTS[v] - if matches!(v, 2 | 7 | 12) { 12 } else { 0 };
    for bar in 0..4 {
        let shift = [0, 5, 3, 7][(bar + v / 4) % 4];
        for (voice, &interval) in CHORDS[v % 8].iter().enumerate() {
            let note = root + shift + interval + if v == 1 { 12 } else { 0 };
            let beat = bar as f64 * 4.0 + voice as f64 * 0.025;
            pitched(
                bank,
                beat,
                note,
                color,
                bank.beat * 6.0,
                true,
                if v == 11 { 0.65 } else { 0.12 },
                0.15 + voice as f64 * 0.22,
            );
            if v == 5 {
                vocal(bank, beat, note + 12, 4, bar, bank.beat * 5.0, 0.7);
            } else if matches!(v, 6 | 9 | 14) {
                pitched(
                    bank,
                    beat + 0.06,
                    note + 12,
                    Color::Glass,
                    bank.beat * 5.5,
                    true,
                    0.22,
                    0.8 - voice as f64 * 0.2,
                );
            }
        }
    }
    bank.echo(bank.beat * 1.5, 0.22, 3);
}

fn textures(bank: &mut Bank, v: usize) {
    let seconds = bank.frames.len() as f64 / bank.rate as f64;
    // A cyclic noise bed is filtered through two periods: the first primes the
    // filter so the second starts with the previous period's state.
    let noise: Vec<f64> = (0..bank.frames.len())
        .map(|_| bank.random.bipolar() as f64)
        .collect();
    let cutoff = [
        3200., 620., 300., 2100., 450., 1800., 2700., 700., 1200., 380., 3800., 180., 2400., 600.,
        900., 1700.,
    ][v];
    let coefficient = 1.0 - (-TAU * cutoff / bank.rate as f64).exp();
    let mut low = 0.0;
    for &value in &noise {
        low += (value - low) * coefficient;
    }
    for (i, &value) in noise.iter().enumerate() {
        low += (value - low) * coefficient;
        let t = i as f64 / bank.rate as f64;
        let swell = 0.3 + 0.7 * (0.5 + 0.5 * sine(t / seconds * (1 + v % 4) as f64)).powi(2);
        let hum = sine(t / seconds * (seconds * (38.0 + v as f64 * 3.0)).round()) * 0.025;
        let bed = low
            * swell
            * if matches!(v, 1 | 7 | 8 | 11 | 12 | 14) {
                0.16
            } else {
                0.035
            }
            + hum;
        bank.frames[i][0] += bed as f32;
        bank.frames[(i + noise.len() / 7) % noise.len()][1] += bed as f32;
    }
    for step in 0..32 {
        let interval = PHRASES[v][step % 16];
        if interval < 0 {
            continue;
        }
        let beat = step as f64 * 0.5 + (step % 3) as f64 * 0.025;
        let pan = 0.15 + (step % 6) as f64 * 0.14;
        let frequency = hz(48 + interval as u8 + if matches!(v, 5 | 6 | 10) { 24 } else { 0 });
        match v {
            0 | 7 | 8 | 11 | 12 | 14 => percussion(
                bank,
                beat,
                [13, 4, 3, 6, 12, 2][[0, 7, 8, 11, 12, 14].iter().position(|x| *x == v).unwrap()],
                frequency * 2.0,
                0.12 + (step % 4) as f64 * 0.04,
                pan,
            ),
            4 | 13 => bank.note(beat, 60 + interval as u8, Tone::Bell, 0.8, pan, 0.15),
            2 | 9 => percussion(bank, beat, 14, frequency, 0.3, pan),
            _ => {
                let duration = bank.beat * if v == 1 { 2.5 } else { 0.3 };
                let bend = if v == 5 { 22.0 } else { -8.0 };
                bank.event(beat * bank.beat, duration, pan, |t, _| {
                    let env = (1.0 - (-t * 500.0).exp()) * ((duration - t) / duration).powi(2);
                    sine(frequency * t + bend * t * t) * env * 0.18
                });
            }
        }
    }
    bank.echo(bank.beat * if v == 15 { 0.75 } else { 1.5 }, 0.32, 4);
    if v == 13 {
        bank.frames.reverse();
    }
}

pub(super) fn generate(track: &Track, rate: u32, index: usize) -> SampleBuffer {
    let mut bank = Bank::new(track, rate);
    let variant = index % 16;
    match index / 16 {
        0 => drums(&mut bank, variant),
        1 => percussion_loop(&mut bank, variant),
        2 => vocals(&mut bank, variant),
        3 => basses(&mut bank, variant),
        4 => keys(&mut bank, variant),
        5 => plucks(&mut bank, variant),
        6 => pads(&mut bank, variant),
        7 => textures(&mut bank, variant),
        _ => unreachable!("expansion recipe index"),
    }
    bank.finish(track)
}
