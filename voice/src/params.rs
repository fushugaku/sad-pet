//! VOCALOID5's voice parameters as changes to the spectral frames: each
//! acts on the envelope, the aperiodicity or the pulses, so any voice —
//! recorded or built in — responds the same way.
use crate::score::{Attack, Param, Release};
use crate::spectrum::{
    AP_BANDS, AP_EDGES, ENV_POINTS, Envelope, FLOOR_DB, Frame, grid_hz, grid_position, sample_grid,
};
use crate::vocoder::Pulse;
use std::sync::LazyLock;

/// Parameter values for one frame, in VOCALOID5 units.
#[derive(Clone, Copy, Debug)]
pub struct Values {
    pub dynamics: f32,
    pub breathiness: f32,
    pub brightness: f32,
    pub clearness: f32,
    pub character: f32,
    pub exciter: f32,
    pub growl: f32,
    pub air: f32,
    pub mouth: f32,
}
impl Default for Values {
    fn default() -> Self {
        let d = |p: Param| p.range().2;
        Self {
            dynamics: d(Param::Dynamics),
            breathiness: d(Param::Breathiness),
            brightness: d(Param::Brightness),
            clearness: d(Param::Clearness),
            character: d(Param::Character),
            exciter: d(Param::Exciter),
            growl: d(Param::Growl),
            air: d(Param::Air),
            mouth: d(Param::Mouth),
        }
    }
}
impl Values {
    pub fn get(score: &crate::score::Score, tick: f32) -> Self {
        Self {
            dynamics: score.param(Param::Dynamics, tick),
            breathiness: score.param(Param::Breathiness, tick),
            brightness: score.param(Param::Brightness, tick),
            clearness: score.param(Param::Clearness, tick),
            character: score.param(Param::Character, tick),
            exciter: score.param(Param::Exciter, tick),
            growl: score.param(Param::Growl, tick),
            air: score.param(Param::Air, tick),
            mouth: score.param(Param::Mouth, tick),
        }
    }
}

static HZ: LazyLock<[f32; ENV_POINTS]> = LazyLock::new(|| std::array::from_fn(grid_hz));
static OCTAVES_ABOVE_1K: LazyLock<[f32; ENV_POINTS]> =
    LazyLock::new(|| std::array::from_fn(|g| (grid_hz(g).max(1000.0) / 1000.0).log2()));

fn warp(env: &Envelope, factor: f32, below: f32) -> Envelope {
    // Reads the envelope at f / factor: factor > 1 moves formants up.
    let mut out = *env;
    for g in 0..ENV_POINTS {
        let hz = HZ[g];
        let f = if below > 0.0 && hz > below {
            // Only the band below `below` is warped; above it, blend back.
            let blend = ((hz - below) / below).clamp(0.0, 1.0);
            factor + (1.0 - factor) * blend
        } else {
            factor
        };
        out[g] = sample_grid(env, grid_position(hz / f));
    }
    out
}

/// Applies the parameters to one frame and returns its pulse controls.
pub fn apply(frame: &mut Frame, v: &Values, robot: bool) -> Pulse {
    let silent = frame.env.iter().all(|&e| e <= FLOOR_DB + 1.0);
    let pulse = Pulse {
        growl: (v.growl / 127.0).clamp(0.0, 1.0),
        jitter: if robot { 0.0 } else { 0.15 },
    };
    if silent {
        return pulse;
    }
    let dynamics = (v.dynamics - 64.0) / 64.0; // -1..1
    let bright = (v.brightness - 64.0) / 64.0;
    let breath = (v.breathiness / 127.0).clamp(0.0, 1.0);
    let air = (v.air / 127.0).clamp(0.0, 1.0);
    let clear = (v.clearness / 127.0).clamp(0.0, 1.0);
    let exciter = (v.exciter / 64.0).clamp(-1.0, 1.0);
    let mouth = (-v.mouth / 127.0).clamp(0.0, 1.0);
    let character = (v.character / 64.0).clamp(-1.0, 1.0);

    // Character: formant scale ±20 %; Mouth closes the low formants.
    if character.abs() > 1e-3 {
        frame.env = warp(&frame.env, 2f32.powf(character * 0.26), 0.0);
    }
    if mouth > 1e-3 {
        frame.env = warp(&frame.env, 1.0 - 0.22 * mouth, 1800.0);
    }
    // Clearness: deeper valleys between formants.
    if clear > 1e-3 {
        let original = frame.env;
        for g in 0..ENV_POINTS {
            let lo = g.saturating_sub(5);
            let hi = (g + 5).min(ENV_POINTS - 1);
            let mean = original[lo..=hi].iter().sum::<f32>() / (hi - lo + 1) as f32;
            frame.env[g] = original[g] + 0.6 * clear * (original[g] - mean);
        }
    }
    let louder = 18.0 * dynamics.min(0.0) * 1.6 + 10.0 * dynamics.max(0.0);
    for g in 0..ENV_POINTS {
        let octaves = OCTAVES_ABOVE_1K[g];
        let hz = HZ[g];
        // Loud singing is also brighter; Brightness tilts above 1 kHz.
        let tilt = (2.5 * dynamics + 4.0 * bright) * octaves;
        // Exciter: shelf above 2.5 kHz (bright) or a muffling roll-off.
        let shelf = if hz > 2500.0 {
            let x = ((hz - 2500.0) / 2500.0).min(1.0);
            if exciter >= 0.0 {
                8.0 * exciter * x
            } else {
                16.0 * exciter * x * (hz / 2500.0).log2().max(1.0)
            }
        } else {
            0.0
        };
        let closed = -9.0 * mouth * ((hz - 1500.0) / 1500.0).clamp(0.0, 1.0);
        frame.env[g] =
            (frame.env[g] + louder + tilt.clamp(-18.0, 18.0) + shelf + closed).max(FLOOR_DB);
    }
    // Breathiness and Air raise the noise share, more in the high bands.
    for b in 0..AP_BANDS {
        let hz = (AP_EDGES[b] + AP_EDGES[b + 1]) * 0.5;
        let high = ((hz - 800.0) / 4000.0).clamp(0.0, 1.0);
        let mut a = frame.ap[b];
        a *= 1.0 - 0.85 * clear;
        if exciter > 0.0 && hz > 2500.0 {
            a *= 1.0 - 0.5 * exciter;
        }
        a += (1.0 - a) * breath * (0.25 + 0.7 * high);
        a += (1.0 - a) * air * high * 0.8;
        a += (1.0 - a) * pulse.growl * 0.25 * (1.0 - high);
        frame.ap[b] = a.clamp(0.0, 1.0);
    }
    if air > 1e-3 {
        // Air adds breath noise on top rather than replacing the voice.
        for g in 0..ENV_POINTS {
            let high = ((HZ[g] - 1500.0) / 3000.0).clamp(0.0, 1.0);
            frame.env[g] += 5.0 * air * high;
        }
    }
    if breath > 1e-3 {
        // A breathy voice loses some of its harmonic strength up high.
        for g in 0..ENV_POINTS {
            frame.env[g] -= 3.0 * breath * OCTAVES_ABOVE_1K[g].min(2.0);
        }
    }
    pulse
}

/// Level (dB) and breath added by a note's attack and release effects at a
/// time `since` its start and `left` before its end.
pub fn effects(
    attack: (Attack, f32),
    release: (Release, f32),
    since: f64,
    left: f64,
) -> (f32, f32, f32) {
    let smooth = |x: f64| {
        let x = x.clamp(0.0, 1.0);
        (x * x * (3.0 - 2.0 * x)) as f32
    };
    let (mut db, mut breath, mut bright) = (0.0f32, 0.0f32, 0.0f32);
    let (kind, a) = attack;
    match kind {
        Attack::Soft => db -= 24.0 * a * (1.0 - smooth(since / 0.25)),
        Attack::Accent => {
            let x = 1.0 - smooth(since / 0.18);
            db += 5.0 * a * x;
            bright += 25.0 * a * x;
        }
        Attack::Breathy => breath += 90.0 * a * (1.0 - smooth(since / 0.3)),
        _ => {}
    }
    let (kind, r) = release;
    let tail = 1.0 - smooth(left / 0.3);
    match kind {
        Release::Soft => db -= 24.0 * r * tail,
        Release::Breathy => breath += 100.0 * r * tail,
        _ => {}
    }
    (db, breath, bright)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn vowel() -> Frame {
        let mut f = Frame::SILENT;
        f.f0 = 220.0;
        for g in 0..ENV_POINTS {
            let hz = grid_hz(g);
            f.env[g] = -80.0 - 10.0 * ((hz - 700.0) / 300.0).powi(2).min(6.0) - 0.002 * hz;
        }
        f.ap = [0.01, 0.01, 0.02, 0.05, 0.1, 0.2, 0.3, 0.4];
        f
    }
    #[test]
    fn defaults_change_nothing_but_jitter() {
        let mut f = vowel();
        let original = f;
        let pulse = apply(&mut f, &Values::default(), false);
        assert_eq!(pulse.growl, 0.0);
        for (a, b) in f.env.iter().zip(&original.env) {
            assert!((a - b).abs() < 1e-4);
        }
        assert_eq!(f.ap, original.ap);
    }
    #[test]
    fn each_parameter_moves_its_dimension() {
        let base = vowel();
        let with = |v: Values| {
            let mut f = base;
            apply(&mut f, &v, false);
            f
        };
        let d = Values::default();
        let loud = with(Values {
            dynamics: 127.0,
            ..d
        });
        assert!(loud.level() > base.level() + 6.0);
        let breathy = with(Values {
            breathiness: 127.0,
            ..d
        });
        assert!(breathy.ap[6] > 0.7 && breathy.ap[0] > base.ap[0]);
        let bright = with(Values {
            brightness: 127.0,
            ..d
        });
        assert!(bright.env_at(6000.0) > base.env_at(6000.0) + 6.0);
        let up = with(Values {
            character: 63.0,
            ..d
        });
        // The formant at 700 Hz moves up.
        let peak = |f: &Frame| {
            (300..1500)
                .step_by(10)
                .max_by(|a, b| f.env_at(*a as f32).total_cmp(&f.env_at(*b as f32)))
                .unwrap()
        };
        assert!(peak(&up) > peak(&base) + 80);
        let clear = with(Values {
            clearness: 127.0,
            ..d
        });
        assert!(clear.ap[6] < base.ap[6]);
        let muffled = with(Values {
            exciter: -64.0,
            ..d
        });
        assert!(muffled.env_at(8000.0) < base.env_at(8000.0) - 10.0);
    }
}
