//! Reusable motion lives in Neuropolis. Saved programs contain declarative
//! tracks only, travel with instruments/scenes, and allocate no idle runtimes.
use super::{STAGES, Target, Track, field};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "tracks")]
    pub tracks: Vec<Track>,
    #[serde(default)]
    pub xy: bool,
}
fn tracks<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<Track>, D::Error> {
    let tracks = Vec::<Track>::deserialize(d)?;
    if tracks.is_empty() || tracks.len() > 16 {
        return Err(serde::de::Error::custom(
            "A saved motion contains 1–16 tracks",
        ));
    }
    Ok(tracks)
}
pub(super) fn patterns<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<Pattern>, D::Error> {
    let patterns = Vec::<Pattern>::deserialize(d)?;
    if patterns.len() > 16 {
        return Err(serde::de::Error::custom(
            "Keep at most 16 saved motions per instrument",
        ));
    }
    Ok(patterns)
}
fn track(
    id: u32,
    name: &str,
    target: &str,
    division: i32,
    smooth: bool,
    values: impl Fn(usize) -> f32,
) -> Track {
    let mut t = Track {
        id,
        name: name.into(),
        target: Target::Parameter {
            parameter: target.into(),
        },
        absolute: true,
        ..Track::default()
    };
    t.config[field::DIVISION] = division;
    t.config[field::LENGTH] = STAGES as i32;
    for (n, s) in t.steps.iter_mut().take(STAGES).enumerate() {
        s.cv = (values(n).clamp(-1.0, 1.0) * 5000.0).round() as i32;
        s.cv_ramp = u8::from(smooth);
    }
    t
}
pub fn factory() -> Vec<Pattern> {
    let phase = |n: usize| n as f32 / STAGES as f32 * std::f32::consts::TAU;
    let sine = |n: usize| phase(n).sin();
    let make = |key: &str, name: &str, tracks: Vec<Track>, xy| Pattern {
        id: format!("factory:motion:{key}:v1"),
        name: name.into(),
        tracks,
        xy,
    };
    vec![
        make(
            "surf",
            "Slow Surf",
            vec![track(1, "Surf", "wt_surf", 8, true, |n| sine(n) * 0.65)],
            false,
        ),
        make(
            "pendulum",
            "Pendulum scan",
            vec![track(1, "Scan", "wt_surf", 4, true, |n| {
                0.6 * (1.0 - 4.0 * (n as f32 / 16.0 - 0.5).abs())
            })],
            false,
        ),
        make(
            "xy",
            "XY orbit",
            vec![
                track(1, "X", "wt_x", 4, true, |n| sine(n) * 0.7),
                track(2, "Y", "wt_y", 4, true, |n| phase(n).cos() * 0.7),
            ],
            true,
        ),
        make(
            "breathe",
            "Breathing filter",
            vec![track(1, "Cutoff", "filter", 8, true, |n| {
                sine(n) * 0.4 + 0.1
            })],
            false,
        ),
        make(
            "resonance",
            "Resonant steps",
            vec![
                track(1, "Cutoff", "filter", 2, false, |n| {
                    [-0.3, 0.3, -0.1, 0.55][n % 4]
                }),
                track(2, "Resonance", "filter_resonance", 2, true, |n| {
                    [-0.7, -0.5, -0.2, -0.6][n % 4]
                }),
            ],
            false,
        ),
        make(
            "opposed",
            "Opposed waves",
            vec![
                track(1, "A", "wt_a_position", 4, true, |n| sine(n) * 0.8),
                track(2, "B", "wt_b_position", 4, true, |n| -sine(n) * 0.8),
            ],
            false,
        ),
        make(
            "drift",
            "Four-wave drift",
            (0..4)
                .map(|i| {
                    track(
                        i + 1,
                        ["A", "B", "C", "D"][i as usize],
                        &format!("wt_{}_position", ["a", "b", "c", "d"][i as usize]),
                        8,
                        true,
                        |n| (phase(n) + i as f32 * std::f32::consts::FRAC_PI_2).sin() * 0.65,
                    )
                })
                .collect(),
            false,
        ),
        make(
            "stairs",
            "Staircase Surf",
            vec![track(1, "Surf", "wt_surf", 1, false, |n| {
                [-0.75, -0.25, 0.25, 0.75][n % 4]
            })],
            false,
        ),
    ]
}
