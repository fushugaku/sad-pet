use sad_pet::{
    drums::Settings,
    dsp::{Engine, SampleBuffer, sample::LiveInput},
    parameters,
    preset::Preset,
};
use std::sync::Arc;

#[test]
fn resizing_the_drum_window_retains_history_across_wraps_and_freeze() {
    let mut input = LiveInput::for_drums(8000, false, 8.);
    let capacity = input.sample.len();
    let pointer = input.sample.frames().as_ptr();
    let signal = |i: usize| [(i % 31) as f32 / 31. - 0.5; 2];
    let end = 34 * 8000 + 137;
    for i in 0..end {
        input.push(signal(i), false);
    }
    for seconds in [0.125, 1.375, 16., 0.5, 32., 8.] {
        assert!(input.set_window_seconds(seconds));
        let frames = (seconds * 8000.) as usize;
        assert_eq!(input.window_frames(), frames);
        assert_eq!(input.range().len(), frames);
        assert_eq!(input.seconds(), seconds as f64);
        assert_eq!(
            input.capture(),
            (end - frames..end).map(signal).collect::<Vec<_>>()
        );
        // Completing the visual refresh while frozen must leave the PCM intact.
        for _ in 0..frames.div_ceil(16) {
            input.push([0.; 2], true);
        }
        assert!(input.sample.waveform.iter().any(|bin| bin[1] > 0.4));
        assert_eq!(input.sample.len(), capacity);
        assert_eq!(input.sample.frames().as_ptr(), pointer);
        assert_eq!(input.capture().last().copied(), Some(signal(end - 1)));
    }
}

#[test]
fn granular_reads_wrap_inside_the_selected_history_at_the_physical_ring_seam() {
    let mut input = LiveInput::for_drums(8000, false, 0.125);
    for _ in 0..32 * 8000 - 500 {
        input.push([0.9; 2], false);
    }
    for _ in 0..1000 {
        input.push([0.25; 2], false);
    }
    let range = input.range();
    assert!(range.end > input.sample.len());
    for position in [
        range.start as f64 - 0.3,
        range.start as f64 + 0.2,
        input.sample.len() as f64 - 0.2,
        input.sample.len() as f64 + 0.3,
        range.end as f64 - 0.2,
        range.end as f64 + 0.7,
    ] {
        for rate in [1., -1., 4., -8.] {
            let frame = input.sample.read_region(position, rate, true, &range);
            assert!(
                frame.into_iter().all(|x| (x - 0.25).abs() < 0.00001),
                "Excluded history leaked at position {position}, rate {rate}: {frame:?}"
            );
        }
    }
}

#[test]
fn short_buffers_scale_the_whole_size_range_and_old_presets_keep_eight_seconds() {
    let old = serde_json::to_value(Settings::default()).unwrap();
    let mut old = old.as_object().unwrap().clone();
    old.remove("buffer_seconds");
    assert_eq!(
        serde_json::from_value::<Settings>(old.into())
            .unwrap()
            .buffer_seconds,
        8.
    );
    for seconds in [0.125, 0.5, 1., 2., 8., 32.] {
        let settings = Settings {
            buffer_seconds: seconds,
            ..Settings::default()
        };
        let max = seconds.min(2.01);
        assert!((settings.grain_seconds(0.) - max).abs() < 0.000001);
        assert!((settings.grain_seconds(1.) - max).abs() < 0.000001);
        assert!((settings.grain_seconds(0.5) - 0.01).abs() < 0.000001);
        for i in 50..100 {
            assert!(
                settings.grain_seconds(i as f32 / 100.)
                    < settings.grain_seconds((i + 1) as f32 / 100.)
            );
        }
    }
}

#[test]
fn resizing_a_frozen_playing_drum_timbre_preserves_capture_and_the_drum_clock() {
    let mut preset = Preset {
        drums: Some(Box::default()),
        color_chain: vec![],
        ..Preset::default()
    };
    preset.drums.as_mut().unwrap().granular = true;
    preset.drums.as_mut().unwrap().tracks[0].steps[0].velocity = 1.;
    preset.params.grains = 0.7;
    let mut engine = Engine::new(8000, Arc::new(SampleBuffer::demo(8000)), preset);
    engine.set_playing(true);
    for _ in 0..72000 {
        engine.process_frame();
    }
    engine.set_frozen(true);
    let capture = engine.live_input.as_ref().unwrap().capture();
    let frames = engine.live_input.as_ref().unwrap().sample.frames().as_ptr();
    let clock = engine.drum_view().positions;
    engine.apply_parameters(
        &parameters::parse(&serde_json::json!({"drum_buffer_seconds":0.125})).unwrap(),
    );
    assert_eq!(engine.drum_view().positions, clock);
    assert_eq!(engine.source_frames(), 1000);
    assert_eq!(engine.source_duration(), 0.125);
    for _ in 0..2048 {
        assert!(engine.process_frame().into_iter().all(f32::is_finite));
    }
    assert_eq!(
        engine.live_input.as_ref().unwrap().capture(),
        capture[capture.len() - 1000..]
    );
    engine.apply_parameters(
        &parameters::parse(&serde_json::json!({"drum_buffer_seconds":8.})).unwrap(),
    );
    for _ in 0..4096 {
        engine.process_frame();
    }
    assert_eq!(engine.live_input.as_ref().unwrap().capture(), capture);
    assert_eq!(
        engine.live_input.as_ref().unwrap().sample.frames().as_ptr(),
        frames
    );
    assert!((0.0..1.0).contains(&engine.source_position(engine.center())));
    assert!(
        engine
            .grain_views()
            .iter()
            .all(|g| (0.0..=1.0).contains(&g.position))
    );
}
