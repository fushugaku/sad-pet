use sad_pet::{
    Stereo,
    recording::{Recorder, mp3},
    wav,
};
use std::{io::Cursor, sync::Arc};

#[test]
fn recording_captures_exact_blocks_and_stopping_is_retryable() {
    let mut recorder = Recorder::new(48_000);
    let first = [[0.1, -0.2], [0.25, -0.5]];
    let second = [[0.0, 0.0], [-0.75, 0.75], [1.0, -1.0]];
    recorder.capture(&first);
    assert!(recorder.stop().is_err());
    recorder.start().unwrap();
    assert!(recorder.start().is_err());
    recorder.capture(&first);
    recorder.capture(&second);
    let active = recorder.view();
    assert!(active.active);
    assert!((active.seconds - 5.0 / 48_000.0).abs() < 1e-12);
    let take = recorder.stop().unwrap();
    assert_eq!(
        take.frames,
        first.into_iter().chain(second).collect::<Vec<Stereo>>()
    );
    assert!(Arc::ptr_eq(&take, &recorder.stop().unwrap()));
    recorder.capture(&[[0.5, 0.5]]);
    assert_eq!(take.frames.len(), 5);
    let view = recorder.view();
    assert!(!view.active && view.available);
    assert_eq!(view.id.as_deref(), Some(take.id.as_str()));
    let bytes = wav::bytes(take.sample_rate, &take.frames).unwrap();
    let decoded = wav::decode(Cursor::new(bytes), "recording".into()).unwrap();
    assert_eq!(decoded.frames(), take.frames);
    recorder.start().unwrap();
    recorder.capture(&[[0.4, -0.3]]);
    let next = recorder.stop().unwrap();
    assert_ne!(next.id, take.id);
    assert_eq!(next.frames, vec![[0.4, -0.3]]);
    assert_eq!(take.frames.len(), 5);
}

#[test]
fn mp3_contains_stereo_mpeg_frames_at_the_requested_bitrate() {
    let frames = (0..48_000)
        .map(|i| {
            let phase = i as f32 / 48_000.0 * std::f32::consts::TAU;
            [(phase * 220.0).sin() * 0.3, (phase * 330.0).sin() * 0.2]
        })
        .collect::<Vec<_>>();
    let bytes = mp3(48_000, &frames).unwrap();
    assert!((24_000..28_000).contains(&bytes.len()));
    let header = bytes
        .windows(4)
        .find(|h| h[0] == 0xff && h[1] == 0xfb)
        .unwrap();
    // MPEG-1 Layer III, 192 kbps, 48 kHz, two channels (stereo/joint stereo).
    assert_eq!(header[2] >> 4, 11);
    assert_eq!((header[2] >> 2) & 3, 1);
    assert_ne!(header[3] >> 6, 3);
}
