//! The built-in voice: a formant synthesizer that needs no recordings. Each
//! phoneme has acoustic targets (formants for vowels and sonorants, noise
//! spectra for fricatives and bursts); frames glide between targets, as the
//! vocal tract does. It sings every language of the inventory, intelligibly
//! if plainly, and keeps the instrument useful before a voicebank is loaded.
use crate::phoneme::{Class, Phoneme, Place};
use crate::spectrum::{AP_BANDS, AP_EDGES, ENV_POINTS, FLOOR_DB, FRAME_SECONDS, Frame, grid_hz};
use crate::timing::Segment;

/// Overall level of a vowel envelope peak, dB per Hz.
const LEVEL: f32 = -61.5;
/// Source slope above 250 Hz, dB per octave: with the formants' own falloff
/// it gives the 2–5 kHz band about the balance of the bundled voicebanks.
const TILT: f32 = -2.5;
const TRANSITION: f64 = 0.045;

#[derive(Clone, Copy, Debug)]
struct Target {
    formants: [f32; 5],
    bandwidths: [f32; 5],
    /// Gain of the voiced (formant) part relative to a vowel, dB.
    voice: f32,
    /// Noise part: center, width (Hz) and gain (dB) of its spectral hump.
    noise: Option<(f32, f32, f32)>,
    /// Silence (closure or pause).
    silent: bool,
}

fn vowel(f: [f32; 3]) -> Target {
    Target {
        formants: [f[0], f[1], f[2], 3500.0, 4500.0],
        // Wide enough not to ring: narrower ones buzzed by ear.
        bandwidths: [120.0, 150.0, 240.0, 370.0, 510.0],
        voice: 0.0,
        noise: None,
        silent: false,
    }
}

/// F2 locus of a consonant place: where formant transitions point.
fn locus(place: Place, soft: bool) -> [f32; 3] {
    let f2: f32 = match place {
        Place::Bilabial | Place::Labiodental => 900.0,
        Place::Dental | Place::Alveolar => 1700.0,
        Place::Postalveolar => 1900.0,
        Place::Palatal => 2200.0,
        Place::Velar => 1500.0,
        Place::Uvular => 1200.0,
        Place::Glottal | Place::Open => 1400.0,
    };
    let f2 = if soft { f2.max(2000.0) } else { f2 };
    [300.0, f2, 2600.0]
}

/// Noise band of a consonant: center, width (Hz) and gain (dB). The gains
/// put the loudest hiss about 20 dB under the vowels, as in recordings.
fn noise_of(place: Place, soft: bool) -> (f32, f32, f32) {
    match place {
        Place::Bilabial => (1200.0, 2500.0, -26.0),
        Place::Labiodental | Place::Dental => (6000.0, 6000.0, -28.0),
        Place::Alveolar => (6500.0, 3000.0, -13.0),
        Place::Postalveolar => (if soft { 3500.0 } else { 3000.0 }, 2000.0, -15.0),
        Place::Palatal => (3500.0, 2500.0, -20.0),
        Place::Velar => (if soft { 3000.0 } else { 1800.0 }, 1500.0, -20.0),
        Place::Uvular => (1300.0, 1200.0, -22.0),
        Place::Glottal | Place::Open => (1500.0, 3000.0, -28.0),
    }
}

fn target(p: Phoneme) -> Target {
    let info = p.info();
    let soft = info.soft;
    let near = locus(info.place, soft);
    let sonorant = |f: [f32; 3], gain: f32| Target {
        voice: gain,
        ..vowel(f)
    };
    match info.class {
        Class::Vowel => vowel(info.formants),
        Class::Silence | Class::Closure => Target {
            silent: true,
            ..vowel(near)
        },
        Class::Breath => Target {
            voice: -90.0,
            noise: Some((1800.0, 3500.0, -34.0)),
            ..vowel([600.0, 1500.0, 2500.0])
        },
        Class::Nasal => match info.place {
            Place::Bilabial => sonorant([280.0, 1100.0, 2300.0], -9.0),
            Place::Velar | Place::Uvular => sonorant([280.0, 1800.0, 2700.0], -10.0),
            _ => sonorant([280.0, if soft { 2100.0 } else { 1600.0 }, 2600.0], -9.0),
        },
        Class::Liquid => {
            if p.symbol() == "r" && p.language() == Some(crate::Language::En) {
                sonorant([350.0, 1100.0, 1600.0], -6.0)
            } else {
                sonorant([360.0, if soft { 1900.0 } else { 1250.0 }, 2700.0], -6.0)
            }
        }
        Class::Glide => {
            if matches!(info.place, Place::Palatal) {
                sonorant([280.0, 2250.0, 3000.0], -4.0)
            } else {
                sonorant([320.0, 750.0, 2300.0], -5.0)
            }
        }
        Class::Fricative => {
            let (c, w, g) = noise_of(info.place, soft);
            Target {
                voice: if info.voiced { -14.0 } else { -90.0 },
                noise: Some((c, w, g)),
                ..vowel(near)
            }
        }
        Class::Stop | Class::Affricate => {
            let (c, w, g) = noise_of(info.place, soft);
            Target {
                voice: if info.voiced { -24.0 } else { -90.0 },
                noise: Some((
                    c,
                    w,
                    g + if info.class == Class::Affricate {
                        0.0
                    } else {
                        -2.0
                    },
                )),
                ..vowel(near)
            }
        }
    }
}

/// The voice's color: a correction (dB) at each frequency, fitted so that
/// over the listening set (`voice/listening.json`) the synthetic voice's
/// average envelope matches what the bundled voicebanks sing there, within
/// 0.3 dB. Most of it lifts the highs, where the cascade falls far too
/// steeply past its fifth formant; it also takes the edge off 2.5 kHz.
const COLOR: [(f32, f32); 13] = [
    (250.0, 2.0),
    (500.0, -7.0),
    (750.0, 0.5),
    (1000.0, 1.8),
    (1500.0, 0.5),
    (2000.0, -1.9),
    (2500.0, -10.1),
    (3000.0, 0.7),
    (4000.0, 24.4),
    (5000.0, 47.0),
    (6500.0, 66.9),
    (8000.0, 90.6),
    (10000.0, 109.6),
];

fn color(hz: f32) -> f32 {
    let x = hz.clamp(COLOR[0].0, COLOR[12].0).ln();
    let i = COLOR
        .iter()
        .rposition(|c| c.0.ln() <= x)
        .unwrap_or(0)
        .min(11);
    let ((h0, c0), (h1, c1)) = (COLOR[i], COLOR[i + 1]);
    c0 + (c1 - c0) * (x - h0.ln()) / (h1.ln() - h0.ln())
}

/// The aperiodicity a voiced sound keeps in each band, from the same fit: a
/// recorded voice is a little breathy above 3 kHz, where a bare pulse train
/// buzzes.
const BREATH: [f32; AP_BANDS] = [0.006, 0.005, 0.017, 0.043, 0.053, 0.143, 0.204, 0.62];

/// How far the formants (share) and the loudness (dB) waver.
const WAVER_FORMANTS: f32 = 0.012;
const WAVER_DB: f32 = 0.8;

/// Two slow, irregular wavers in -1..1 at time `t`: sums of sines at rates
/// that never line up.
fn waver(t: f64) -> (f32, f32) {
    let wave = |rates: [(f64, f64, f64); 3]| {
        rates
            .iter()
            .map(|&(hz, phase, weight)| weight * (std::f64::consts::TAU * hz * t + phase).sin())
            .sum::<f64>() as f32
    };
    (
        wave([(3.1, 0.4, 0.5), (5.3, 2.1, 0.3), (1.7, 4.0, 0.2)]),
        wave([(4.3, 1.3, 0.5), (2.3, 5.2, 0.3), (6.7, 0.9, 0.2)]),
    )
}

/// dB of a cascade of resonators at `hz` relative to DC gain.
fn formant_db(t: &Target, hz: f32) -> f32 {
    let mut db = 0.0;
    for (&f, &b) in t.formants.iter().zip(&t.bandwidths) {
        let num = f * f + (b * 0.5).powi(2);
        let den = ((f * f - hz * hz + (b * 0.5).powi(2)).powi(2) + (hz * b).powi(2)).sqrt();
        // `num / den` is the resonator's amplitude gain.
        db += 20.0 * (num / den).log10();
    }
    db
}

fn mix_targets(a: &Target, b: &Target, t: f32) -> Target {
    let lerp = |x: f32, y: f32| x + (y - x) * t;
    let mut out = *a;
    for k in 0..5 {
        out.formants[k] = lerp(a.formants[k], b.formants[k]);
        out.bandwidths[k] = lerp(a.bandwidths[k], b.bandwidths[k]);
    }
    out.voice = lerp(a.voice, b.voice);
    out
}

/// Frames for laid-out phonemes; `pitch` is MIDI per frame.
pub fn frames(segments: &[Segment], pitch: &[f32]) -> Vec<Frame> {
    let count = pitch.len();
    let mut out = vec![Frame::SILENT; count];
    if segments.is_empty() {
        return out;
    }
    let targets: Vec<Target> = segments.iter().map(|s| target(s.phoneme)).collect();
    let mut seg = 0;
    for (i, frame) in out.iter_mut().enumerate() {
        let t = i as f64 * FRAME_SECONDS as f64;
        while seg + 1 < segments.len() && t >= segments[seg].end {
            seg += 1;
        }
        let s = &segments[seg];
        let here = targets[seg];
        // Glide from the previous target over the start of this phoneme and
        // towards the next over its end.
        let mut tract = here;
        if seg > 0 && !targets[seg - 1].silent {
            let into = ((t - s.start) / TRANSITION.min(s.length() * 0.5).max(1e-3)).clamp(0.0, 1.0);
            let w = 0.5 + 0.5 * into as f32;
            tract = mix_targets(&targets[seg - 1], &here, w);
        }
        if seg + 1 < segments.len() && !targets[seg + 1].silent {
            let out_of = ((s.end - t) / TRANSITION.min(s.length() * 0.5).max(1e-3)).clamp(0.0, 1.0);
            let w = 0.5 - 0.5 * out_of as f32;
            tract = mix_targets(&tract, &targets[seg + 1], w);
        }
        if here.silent {
            continue;
        }
        // A live voice never holds still: the formants and the loudness
        // waver a little, slowly and without a pattern.
        let (shift, shimmer) = waver(t);
        for f in tract.formants.iter_mut() {
            *f *= 1.0 + WAVER_FORMANTS * shift;
        }
        let progress = ((t - s.start) / s.length().max(1e-6)) as f32;
        // Stops: closure for the first part, then the burst.
        let (voice_gain, noise_gain) = match s.phoneme.class() {
            Class::Stop => {
                if progress < 0.6 {
                    (here.voice, -90.0)
                } else {
                    (here.voice, here.noise.map_or(-90.0, |n| n.2))
                }
            }
            Class::Affricate => {
                if progress < 0.35 {
                    (here.voice, -90.0)
                } else {
                    (here.voice, here.noise.map_or(-90.0, |n| n.2))
                }
            }
            _ => (tract.voice, here.noise.map_or(-90.0, |n| n.2)),
        };
        // Fade the voiced part in and out at pauses.
        let edge = |near: bool, d: f64| {
            if near {
                (d / 0.03).clamp(0.0, 1.0) as f32
            } else {
                1.0
            }
        };
        let fade = edge(seg == 0 || targets[seg - 1].silent, t - s.start)
            * edge(
                seg + 1 == segments.len() || targets[seg + 1].silent,
                s.end - t,
            );
        let fade_db = 20.0 * fade.max(1e-4).log10();
        let voiced = voice_gain > -60.0 && pitch[i].is_finite();
        frame.f0 = if voiced {
            440.0 * 2f32.powf((pitch[i] - 69.0) / 12.0)
        } else {
            0.0
        };
        for (g, value) in frame.env.iter_mut().enumerate() {
            let hz = grid_hz(g).max(1.0);
            let tilt = TILT * (hz / 250.0).max(1.0).log2();
            let periodic = LEVEL
                + voice_gain
                + tilt
                + formant_db(&tract, hz)
                + color(hz)
                + WAVER_DB * shimmer
                + fade_db;
            let noise = match here.noise {
                Some((center, width, _)) => {
                    let x = (hz - center) / width;
                    LEVEL + noise_gain - 12.0 * x * x + fade_db
                }
                None => FLOOR_DB,
            };
            let p =
                10f32.powf(periodic.max(FLOOR_DB) / 10.0) + 10f32.powf(noise.max(FLOOR_DB) / 10.0);
            *value = (10.0 * p.log10()).max(FLOOR_DB);
        }
        // Aperiodicity: the noise share in each band.
        for b in 0..AP_BANDS {
            let hz = (AP_EDGES[b] + AP_EDGES[b + 1]) * 0.5;
            let tilt = TILT * (hz / 250.0).max(1.0).log2();
            let periodic = LEVEL + voice_gain + tilt + formant_db(&tract, hz) + color(hz);
            let noise = here.noise.map_or(FLOOR_DB, |(center, width, _)| {
                let x = (hz - center) / width;
                LEVEL + noise_gain - 12.0 * x * x
            });
            let (pp, pn) = (10f32.powf(periodic / 10.0), 10f32.powf(noise / 10.0));
            // A little breath in the highest bands of every vowel.
            let floor = BREATH[b];
            frame.ap[b] = if voiced {
                (pn / (pp + pn)).max(floor)
            } else {
                1.0
            };
        }
    }
    let _ = ENV_POINTS;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phoneme::Language;
    use crate::timing::Role;

    #[test]
    fn vowels_have_their_formants_and_fricatives_are_noise() {
        let p = |s| Phoneme::parse(Language::Ru, s).unwrap();
        let seg = |ph, start, end| Segment {
            phoneme: ph,
            start,
            end,
            role: Role::Nucleus,
            note: Some(0),
        };
        let segments = [
            seg(Phoneme::SIL, 0.0, 0.1),
            seg(p("s"), 0.1, 0.25),
            seg(p("a"), 0.25, 0.6),
            seg(p("i"), 0.6, 0.9),
            seg(Phoneme::SIL, 0.9, 1.0),
        ];
        let pitch = vec![60.0; 200];
        let f = frames(&segments, &pitch);
        assert!(!f[10].voiced() && f[10].env[50] <= FLOOR_DB + 1.0);
        let s = &f[35];
        assert!(!s.voiced() && s.env_at(6500.0) > s.env_at(500.0));
        let a = &f[85];
        assert!(a.voiced() && (a.f0 - 261.6).abs() < 0.5);
        assert!(a.env_at(750.0) > a.env_at(2000.0));
        let i = &f[150];
        // /i/ has a low F1 and a high F2.
        assert!(i.env_at(2300.0) > a.env_at(2300.0));
        assert!(i.env_at(750.0) < a.env_at(750.0));
        // Like a recorded voice, a vowel keeps some air above 5 kHz and is a
        // little breathy there, where a bare pulse train would buzz.
        assert!(
            a.env_at(6500.0) > a.env_at(750.0) - 55.0,
            "{}",
            a.env_at(6500.0) - a.env_at(750.0)
        );
        assert!(a.ap[5] > 0.1 && a.ap[1] < 0.01);
    }
}
