//! Bouncing a pad: each note renders on fresh engines — the pad alone in a
//! kit whose own chain is off, and a new copy of its Pad FX chain — so no
//! tail, LFO phase or grain of one note reaches the next. The result is the
//! pad's playback with everything after it baked in.
use super::{BounceOptions, Settings, Zone};
use crate::{Stereo, dsp::Engine, dsp::SampleBuffer, dsp::sampler::Pool, preset::Preset};
use std::sync::Arc;

/// After the release, rendering stops once this many seconds stay below
/// −96 dBFS with every voice ended: long enough for the gaps of echoes.
const QUIET_SECONDS: f64 = 1.5;
const SILENT: f32 = 1.6e-5;
/// No note renders longer than this, held part included.
pub const MAX_SECONDS: f64 = 60.0;

/// Renders `note` of pad `pad`, through `chain` when the pad has Pad FX.
pub fn note(
    rate: u32,
    settings: &Settings,
    pool: &Pool,
    pad: u32,
    chain: Option<&Preset>,
    options: &BounceOptions,
    note: u8,
) -> Result<Vec<Stereo>, String> {
    let source = settings.pad(pad).ok_or("Unknown pad")?;
    let mut alone = source.clone();
    alone.bounce = None;
    alone.muted = false;
    alone.solo = false;
    alone.choke = 0;
    if !alone.zones.iter().any(|z| pool.contains_key(&z.sample)) {
        return Err("The pad has no sample to bounce".into());
    }
    let mut kit = crate::timbres::Kind::Sampler
        .preset()
        .ok_or("Sampler preset")?;
    kit.sampler = Some(Box::new(Settings {
        pads: vec![alone],
        polyphony: settings.polyphony,
        selected: pad,
    }));
    let placeholder = Arc::new(SampleBuffer::new("Bounce".into(), rate, vec![[0.0; 2]; 2])?);
    let mut engine = Engine::new(rate, placeholder.clone(), kit);
    if let Some(target) = engine.sampler_pool_mut() {
        for zone in &source.zones {
            if let Some(sample) = pool.get(&zone.sample) {
                target.insert(zone.sample.clone(), sample.clone());
            }
        }
    }
    engine.refresh_sampler();
    let mut fx = chain.map(|preset| {
        let mut fx = Engine::new(rate, placeholder.clone(), preset.clone());
        fx.set_playing(true);
        fx
    });
    if fx.is_some() {
        engine.set_sampler_routes(&[pad]);
    }
    engine.trigger_pad(pad, Some(note), options.velocity);
    let hold = (options.hold as f64 * rate as f64) as usize;
    let tail = (options.tail as f64 * rate as f64) as usize;
    let limit = (hold + tail)
        .min((MAX_SECONDS * rate as f64) as usize)
        .max(1);
    let quiet_limit = (QUIET_SECONDS * rate as f64) as usize;
    let mut frames = Vec::with_capacity(limit.min(rate as usize * 8));
    let mut quiet = 0;
    let mut last_loud = 0;
    for t in 0..limit {
        if t == hold {
            engine.release_pad(pad);
        }
        let returned = match &mut fx {
            // The pad's voices of the previous frame, as in the live mix.
            Some(fx) => fx.process_input_frame(engine.sampler_send(pad)),
            None => [0.0; 2],
        };
        let frame = engine.process_input_frame(returned);
        frames.push(frame);
        if frame[0].abs().max(frame[1].abs()) > SILENT {
            quiet = 0;
            last_loud = t;
        } else {
            quiet += 1;
        }
        if t >= hold && quiet >= quiet_limit && engine.sampler_idle() {
            break;
        }
    }
    // Keep what sounds, then a 5 ms ramp to silence.
    frames.truncate(last_loud + 1);
    let ramp = (rate as usize / 200).min(frames.len());
    let at = frames.len() - ramp;
    for (i, frame) in frames[at..].iter_mut().enumerate() {
        let gain = 1.0 - (i + 1) as f32 / ramp.max(1) as f32;
        frame.iter_mut().for_each(|v| *v *= gain);
    }
    if frames.iter().flatten().all(|v| *v == 0.0) {
        return Err(format!("Note {note} rendered silence"));
    }
    Ok(frames)
}

/// The zones a bounce's rendered notes become: each plays its own note at
/// its root and the keys up to the next note.
pub fn zones(options: &BounceOptions, rendered: &[(u8, String)]) -> Vec<Zone> {
    rendered
        .iter()
        .enumerate()
        .map(|(i, (note, sample))| {
            let low = if i == 0 { 0 } else { *note };
            let high = rendered
                .get(i + 1)
                .map_or(127, |(next, _)| next.saturating_sub(1));
            Zone {
                sample: sample.clone(),
                root: *note,
                keys: [low, high],
                velocities: [1, 127],
                ..Zone::new(sample)
            }
        })
        .filter(|_| !options.notes().is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sampler::{Pad, Zone};

    fn kit() -> (Settings, Pool) {
        let mut settings = Settings::new();
        settings.pads[0] = Pad {
            keytrack: true,
            keys: [0, 127],
            zones: vec![Zone::new("tone")],
            ..Pad::new(1, 36)
        };
        settings.sanitize();
        let tone: Vec<Stereo> = (0..24_000)
            .map(|i| [(i as f32 * 0.05).sin() * 0.5; 2])
            .collect();
        let mut pool = Pool::new();
        pool.insert(
            "tone".into(),
            Arc::new(SampleBuffer::new("tone".into(), 48_000, tone).unwrap()),
        );
        (settings, pool)
    }

    #[test]
    fn a_bounce_without_fx_is_the_pad_itself() {
        let (settings, pool) = kit();
        let options = BounceOptions::default();
        let audio = note(48_000, &settings, &pool, 1, None, &options, 60).unwrap();
        // The one-shot plays its 24 000 frames once.
        assert!((audio.len() as i64 - 24_000).abs() < 10, "{}", audio.len());
        let up = note(48_000, &settings, &pool, 1, None, &options, 72).unwrap();
        assert!((up.len() as i64 - 12_000).abs() < 10, "{}", up.len());
    }

    #[test]
    fn a_bounce_runs_through_the_pad_fx_chain() {
        let (settings, pool) = kit();
        let mut chain = crate::timbres::Kind::PadFx.preset().unwrap();
        chain.params.master_volume = 0.5;
        chain.sanitize();
        let options = BounceOptions::default();
        let dry = note(48_000, &settings, &pool, 1, None, &options, 60).unwrap();
        let wet = note(48_000, &settings, &pool, 1, Some(&chain), &options, 60).unwrap();
        let peak = |a: &[Stereo]| a.iter().flatten().fold(0.0_f32, |m, v| m.max(v.abs()));
        assert!(
            (peak(&wet) / peak(&dry) - 0.5).abs() < 0.02,
            "{} {}",
            peak(&wet),
            peak(&dry)
        );
    }

    #[test]
    fn bounced_zones_cover_the_keyboard() {
        let options = BounceOptions {
            low: 36,
            high: 48,
            step: 6,
            ..Default::default()
        };
        let rendered: Vec<(u8, String)> = options
            .notes()
            .into_iter()
            .map(|n| (n, format!("b{n}")))
            .collect();
        let zones = zones(&options, &rendered);
        let ranges: Vec<[u8; 2]> = zones.iter().map(|z| z.keys).collect();
        assert_eq!(ranges, vec![[0, 41], [42, 47], [48, 127]]);
        assert_eq!(zones[1].root, 42);
    }
}
