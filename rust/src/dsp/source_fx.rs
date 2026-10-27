//! Source effects precede grain reads. Files are prepared off-thread; live audio
//! is processed once before entering the granular history buffer.
use super::{
    SampleBuffer,
    sample::{Mip, Region},
};
use crate::{Stereo, color::ColorSlot};
use std::sync::Arc;
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    pub region: Region,
    pub slot: Option<ColorSlot>,
    pub bpm: f32,
    pub root: f32,
}
impl Recipe {
    pub fn new(region: Region, slot: Option<ColorSlot>, bpm: f32, root: f32) -> Self {
        let slot = slot.filter(|s| s.enabled && s.mix > 0.0);
        let tempo = slot.is_some_and(|s| match s.kind {
            crate::preset::SaturatorType::LcrDelay => s.settings.lcr_sync,
            crate::preset::SaturatorType::ModDelay => s.settings.mod_delay_sync,
            crate::preset::SaturatorType::SuperEcho => s.settings.super_echo_sync >= 0.5,
            // Any mooz model with tempo-synced times or rates renders per tempo.
            crate::preset::SaturatorType::Mooz(model) => model.info().params.iter().any(|p| {
                matches!(
                    p.display,
                    sad_pet_mooz::Display::DelayTime { .. }
                        | sad_pet_mooz::Display::SyncRate { .. }
                ) || p.name.eq_ignore_ascii_case("sync")
            }),
            _ => false,
        });
        let pitched = slot.is_some_and(|s| {
            use crate::dsp::radias_waveshaper::WaveshaperMode::*;
            s.kind == crate::preset::SaturatorType::Waveshaper
                && matches!(
                    s.settings.waveshaper_type,
                    Pickup | SubSaw | SubSquare | SubTriangle | SubSine
                )
        });
        Self {
            region,
            slot,
            bpm: if tempo { bpm } else { 120.0 },
            root: if pitched { root } else { 60.0 },
        }
    }
    pub fn from_engine(e: &super::Engine) -> Self {
        Self::new(
            e.region,
            e.preset.source_color_chain.first().copied(),
            e.preset.params.bpm,
            e.preset.params.sample_root_note,
        )
    }
}

pub struct ProcessedSource {
    pub original: Arc<SampleBuffer>,
    pub sample: SampleBuffer,
    pub range: std::ops::Range<usize>,
    pub slot: ColorSlot,
    pub bpm: f32,
    pub root: f32,
}

pub fn prepare(
    original: Arc<SampleBuffer>,
    region: Region,
    slot: Option<ColorSlot>,
    bpm: f32,
    root: f32,
    cancelled: impl Fn() -> bool,
) -> Result<Option<Arc<ProcessedSource>>, String> {
    let recipe = Recipe::new(region, slot, bpm, root);
    let bpm = recipe.bpm;
    let root = recipe.root;
    let Some(slot) = slot.filter(|s| s.enabled && s.mix > 0.0) else {
        return Ok(None);
    };
    let range = region.bounds(original.len())?;
    let mut processor = super::color::ColorProcessor::new(original.sample_rate);
    processor.prepare_imported(slot);
    processor.set_pitch(root);
    processor.configure_tempo(slot, bpm);
    let mut frames = Vec::with_capacity(range.len());
    for (i, frame) in original.frames()[range.clone()].iter().enumerate() {
        if i % 4096 == 0 && cancelled() {
            return Err("Source FX preparation superseded".into());
        }
        frames.push(processor.process(*frame));
    }
    if cancelled() {
        return Err("Source FX preparation superseded".into());
    }
    let sample = SampleBuffer::new_cancellable(
        original.name.clone(),
        original.sample_rate,
        frames,
        &cancelled,
    )?;
    Ok(Some(Arc::new(ProcessedSource {
        original,
        sample,
        range,
        slot,
        bpm,
        root,
    })))
}

#[inline]
pub(super) fn read(
    raw: &Arc<SampleBuffer>,
    current: &Option<Arc<ProcessedSource>>,
    previous: &Option<Arc<ProcessedSource>>,
    remaining: u32,
    position: f64,
    mip: Mip,
    range: &std::ops::Range<usize>,
) -> Stereo {
    if current.is_none() && remaining == 0 {
        return raw.read_region_mip(position, mip, range);
    }
    let sample = |cache: &Option<Arc<ProcessedSource>>| {
        if let Some(cache) = cache
            .as_ref()
            .filter(|c| Arc::ptr_eq(&c.original, raw) && c.range == *range)
        {
            cache.sample.read_region_mip(
                position - range.start as f64,
                mip,
                &(0..cache.sample.len()),
            )
        } else {
            raw.read_region_mip(position, mip, range)
        }
    };
    let frame = sample(current);
    if remaining == 0 {
        return frame;
    }
    let old = sample(previous);
    let mix = 1.0 - remaining as f32 / 128.0;
    std::array::from_fn(|c| old[c] * (1.0 - mix) + frame[c] * mix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dsp::{Engine, color::ColorProcessor},
        midi::Message,
        preset::{Preset, SaturatorType},
    };
    fn sample() -> Arc<SampleBuffer> {
        Arc::new(
            SampleBuffer::new(
                "Source".into(),
                8000,
                (0..8000)
                    .map(|i| [(i as f32 * 0.17).sin() * 0.3; 2])
                    .collect(),
            )
            .unwrap(),
        )
    }
    #[test]
    fn source_recipe_tracks_pitch_for_pickup_and_every_sub_oscillator() {
        use crate::dsp::radias_waveshaper::WaveshaperMode;
        let raw = sample();
        for mode in WaveshaperMode::ALL {
            let mut slot = ColorSlot::legacy(SaturatorType::Waveshaper, 1.0);
            slot.settings.waveshaper_type = mode;
            let pitched = matches!(
                mode,
                WaveshaperMode::Pickup
                    | WaveshaperMode::SubSaw
                    | WaveshaperMode::SubSquare
                    | WaveshaperMode::SubTriangle
                    | WaveshaperMode::SubSine
            );
            let a = Recipe::new(Region::default(), Some(slot), 120.0, 48.0);
            let b = Recipe::new(Region::default(), Some(slot), 120.0, 60.0);
            assert_eq!(a != b, pitched, "{mode:?}");
            if pitched {
                let prepare_at = |root| {
                    prepare(
                        raw.clone(),
                        Region::default(),
                        Some(slot),
                        120.0,
                        root,
                        || false,
                    )
                    .unwrap()
                    .unwrap()
                };
                let low = prepare_at(48.0);
                let high = prepare_at(60.0);
                assert_ne!(low.sample.frames(), high.sample.frames(), "{mode:?}");
            }
        }
    }
    #[test]
    fn file_fx_process_selected_input_before_any_grain_or_pitch() {
        let raw = sample();
        let region = Region {
            start: 0.25,
            end: 0.75,
        };
        for kind in [
            SaturatorType::Distortion,
            SaturatorType::Reducer,
            SaturatorType::Crusher,
            SaturatorType::Flanger,
            SaturatorType::Chorus,
            SaturatorType::Binaural,
            SaturatorType::Reverb,
            SaturatorType::LcrDelay,
            SaturatorType::ModDelay,
            SaturatorType::Compressor,
            SaturatorType::Waveshaper,
        ] {
            let slot = ColorSlot::legacy(kind, 0.7);
            let cache = prepare(raw.clone(), region, Some(slot), 120.0, 60.0, || false)
                .unwrap()
                .unwrap();
            let mut reference = ColorProcessor::new(8000);
            reference.set_pitch(60.0);
            reference.configure_tempo(slot, 120.0);
            let expected: Vec<_> = raw.frames()[2000..6000]
                .iter()
                .map(|f| {
                    reference.process(*f).map(|v| {
                        if v.is_finite() {
                            v.clamp(-16.0, 16.0)
                        } else {
                            0.0
                        }
                    })
                })
                .collect();
            assert_eq!(cache.sample.frames(), &expected, "{kind:?}");
            assert_eq!(cache.range, 2000..6000);
        }
    }
    #[test]
    fn excluded_audio_never_enters_a_source_delay() {
        let raw = Arc::new(
            SampleBuffer::new(
                "Cut".into(),
                8000,
                (0..8000)
                    .map(|i| if i < 2000 { [0.8; 2] } else { [0.0; 2] })
                    .collect(),
            )
            .unwrap(),
        );
        let cache = prepare(
            raw,
            Region {
                start: 0.25,
                end: 0.75,
            },
            Some(ColorSlot::legacy(SaturatorType::LcrDelay, 1.0)),
            120.0,
            60.0,
            || false,
        )
        .unwrap()
        .unwrap();
        assert!(cache.sample.frames().iter().flatten().all(|v| *v == 0.0));
    }
    #[test]
    fn live_source_is_processed_once_and_raw_meter_and_freeze_are_preserved() {
        let mut p = Preset {
            source_color_chain: vec![ColorSlot::legacy(SaturatorType::Distortion, 1.0)],
            ..Default::default()
        };
        p.source_color_chain[0].settings.distortion_drive = 0.8;
        let mut reference = ColorProcessor::new(8000);
        reference.set_pitch(60.0);
        reference.configure_tempo(p.source_color_chain[0], 120.0);
        let mut e = Engine::new(8000, sample(), p);
        e.set_live_input(true, false);
        e.set_playing(true);
        let expected: Vec<_> = (0..256)
            .map(|i| {
                let frame = [(i as f32 * 0.1).sin() * 0.25; 2];
                e.process_input_frame(frame);
                reference.process(frame)
            })
            .collect();
        assert_eq!(
            &e.live_input.as_ref().unwrap().sample.frames()[..256],
            &expected
        );
        let before = e.live_input.as_ref().unwrap().capture();
        e.set_frozen(true);
        for _ in 0..128 {
            e.process_input_frame([0.8; 2]);
        }
        assert_eq!(e.live_input.as_ref().unwrap().capture(), before);
        assert_eq!(e.live_input.as_ref().unwrap().peak, [0.8; 2]);
    }
    #[test]
    fn cache_swaps_keep_held_notes_and_loop_phase_and_do_not_affect_raw_bypass() {
        let mut p = Preset::default();
        p.params.loop_one_bar = true;
        p.source_color_chain = vec![ColorSlot::legacy(SaturatorType::Distortion, 1.0)];
        let mut e = Engine::new(8000, sample(), p);
        e.midi(Message::NoteOn {
            channel: 0,
            note: 72,
            velocity: 100,
        });
        for _ in 0..4000 {
            e.process_frame();
        }
        let phase = e.voices[1].loop_phase;
        let mut cache = prepare(
            e.sample.clone(),
            e.region,
            e.preset.source_color_chain.first().copied(),
            120.0,
            60.0,
            || false,
        )
        .unwrap();
        e.exchange_processed_source(&mut cache);
        assert_eq!(e.voices[1].loop_phase, phase);
        assert_eq!(e.held_notes(), vec![72]);
        assert!(!e.source_effects_pending());
        let raw = e.sample.clone();
        let mut plain = Engine::new(8000, raw, e.preset.clone());
        plain.preset.source_color_chain.clear();
        plain.set_playing(true);
        e.set_playing(true);
        e.cursor = 0.0;
        plain.cursor = 0.0;
        e.bypass = true;
        plain.bypass = true;
        e.smooth_gain = 0.0;
        e.smooth_bypass_gate = 0.0;
        for _ in 0..512 {
            assert_eq!(e.process_frame(), plain.process_frame());
        }
    }
}
