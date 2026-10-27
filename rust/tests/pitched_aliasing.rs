use sad_pet::{
    dsp::{Engine, SampleBuffer, resynth::Resynth},
    midi::Message,
    preset::Preset,
    resynth::{Mode, Waveform},
    sampler::Zone,
    timbres::Kind,
};
use std::{f64::consts::TAU, sync::Arc};

const RATE: u32 = 48_000;

/// Tones of `hz` at a quarter of full scale each, in a 48 kHz file.
fn tones(hz: &[f64], frames: usize) -> SampleBuffer {
    let frames = (0..frames)
        .map(|i| {
            let t = i as f64 / RATE as f64;
            let x = hz.iter().map(|f| (t * f * TAU).sin()).sum::<f64>() as f32 * 0.25;
            [x, x]
        })
        .collect();
    SampleBuffer::new("tones".into(), RATE, frames).unwrap()
}

fn rms(values: impl Iterator<Item = f32>) -> f64 {
    let (sum, n) = values.fold((0.0_f64, 0_usize), |(s, n), x| {
        (s + (x as f64).powi(2), n + 1)
    });
    (sum / n.max(1) as f64).sqrt()
}

/// What reads at `rate` keep of a tone of `hz`, in dB.
fn gain_db(hz: f64, rate: f64) -> f64 {
    let sample = tones(&[hz], 8192);
    let read = rms((0..1024).map(|i| sample.read(600.3 + i as f64 * rate, rate, true)[0]));
    20.0 * (read / (0.25 / 2_f64.sqrt())).log10()
}

/// Pitches up through the file and its first mip level.
const SEMITONES: [f64; 11] = [2.0, 3.5, 5.0, 7.0, 8.5, 10.0, 11.0, 11.9, 15.0, 19.0, 23.5];

#[test]
fn pitched_reads_reject_what_folds_back_below_18_khz() {
    // Reading a frame of a level per output frame only images; faster reads
    // fold everything above the new Nyquist back down. The flat kernel let
    // nearly all of it back: a fifth up, 21.4-23.5 kHz at 12.8-16 kHz.
    for semis in SEMITONES {
        let rate = 2_f64.powf(semis / 12.0);
        let level = rate.log2().floor() as i32;
        // The file up to 23.5 kHz, a mip up to 90% of its Nyquist.
        let held = if level == 0 {
            23_500.0
        } else {
            21_600.0 / 2_f64.powi(level)
        };
        let mut hz = 24_000.0 / rate * 1.01;
        while hz < held {
            let played = (hz * rate) % 48_000.0;
            let folded = if played > 24_000.0 {
                48_000.0 - played
            } else {
                played
            };
            let gain = gain_db(hz, rate);
            let limit = if folded < 16_000.0 { -50.0 } else { -35.0 };
            assert!(
                folded >= 18_000.0 || gain < limit,
                "{semis} st: {hz:.0} Hz folds to {folded:.0} Hz at {gain:.1} dB"
            );
            hz += 400.0;
        }
    }
}

#[test]
fn pitched_reads_keep_the_highs() {
    // The cutoff follows the rate, as far below the new Nyquist as the flat
    // kernel's is below the file's.
    for semis in SEMITONES {
        let rate = 2_f64.powf(semis / 12.0);
        for (out, least) in [
            (12_000.0, -0.1),
            (16_000.0, -0.25),
            (18_000.0, -0.5),
            (20_000.0, -1.5),
        ] {
            let gain = gain_db(out / rate, rate);
            assert!(gain > least, "{semis} st: {out} Hz at {gain:.2} dB");
        }
    }
}

#[test]
fn reads_up_to_an_eighth_of_an_octave_faster_keep_the_flat_kernel() {
    // Below 2^(1/8) frames a read, all that folds lands above 20 kHz.
    let sample = tones(&[3_000.0, 21_000.0], 4096);
    for position in [100.25, 1000.5, 2047.875, -3.4] {
        let flat = sample.read(position, 1.0, true);
        for rate in [-1.0, 1.0001, 1.05, 1.09, -1.09] {
            assert_eq!(sample.read(position, rate, true), flat, "rate {rate}");
        }
        // The same steps through the first mip.
        let mip = sample.read(position, 2.0, true);
        assert_eq!(sample.read(position, 2.18, true), mip);
        assert_ne!(sample.read(position, 1.1, true), flat);
        assert_ne!(sample.read(position, 2.2, true), mip);
    }
}

/// The grains' level at a fifth up, from 0.5 to 1 s of the default preset.
fn grains_at_a_fifth(hz: &[f64], antialiasing: bool) -> f64 {
    let mut preset = Preset::default();
    preset.params.reverb = 0.0;
    preset.color_chain.clear();
    preset.params.pitch = 0.5 + 7.0 / 72.0;
    preset.params.antialiasing = antialiasing;
    let sample = Arc::new(tones(hz, RATE as usize * 2));
    let mut engine = Engine::new(RATE, sample, preset);
    engine.set_playing(true);
    for _ in 0..RATE / 2 {
        engine.process_frame();
    }
    rms((0..RATE / 2).map(|_| {
        let frame = engine.process_frame();
        (frame[0] + frame[1]) * 0.5
    }))
}

#[test]
fn grains_a_fifth_up_fold_nothing_audible_back() {
    // 22-23.4 kHz plays at 33-35 kHz, which folds to 13-15 kHz.
    let folded = grains_at_a_fifth(&[22_000.0, 22_700.0, 23_400.0], true);
    let played = grains_at_a_fifth(&[5_000.0, 6_000.0, 7_000.0], true);
    let rejection = 20.0 * (folded / played).log10();
    assert!(rejection < -45.0, "folded back at {rejection:.1} dB");
    // Without antialiasing the grains read the file as they always did.
    let raw = grains_at_a_fifth(&[22_000.0, 22_700.0, 23_400.0], false);
    assert!(20.0 * (raw / played).log10() > -10.0);
}

#[test]
fn mono_files_read_as_their_stereo_twins() {
    // A mono file weighs one channel. Its twin differs only in one far
    // frame's right channel, so it weighs both: reads away from that frame
    // and its mips' footprint come out the same, at every kernel.
    let mono = tones(&[1_000.0, 9_000.0, 20_000.0], 32_768);
    let mut frames: Vec<_> = (0..32_768).map(|i| mono.frames()[i]).collect();
    frames[30_000][1] += 0.5;
    let twin = SampleBuffer::new("twin".into(), RATE, frames).unwrap();
    for semis in [-5.0, 0.0, 1.0, 3.0, 7.0, 11.9, 19.0, 30.0] {
        let rate = 2_f64.powf(semis / 12.0);
        for i in 0..2000 {
            let position = 200.37 + i as f64 * 7.3;
            assert_eq!(
                mono.read(position, rate, true),
                twin.read(position, rate, true),
                "{semis} st at {position}"
            );
            let range = 100..20_000;
            assert_eq!(
                mono.read_region(position, rate, true, &range),
                twin.read_region(position, rate, true, &range),
                "{semis} st at {position} in a region"
            );
        }
    }
}

#[test]
fn live_histories_keep_both_channels() {
    // A history starts silent, the same on both channels, and fills later.
    let mut input = sad_pet::dsp::sample::LiveInput::new(RATE, false);
    for _ in 0..RATE {
        input.push([0.5, -0.5], false);
    }
    let head = input.head() as f64;
    for rate in [1.0, 1.5, 3.0] {
        let frame = input.sample.read(head - 2000.3, rate, true);
        assert!(
            (frame[0] - 0.5).abs() < 1e-3 && (frame[1] + 0.5).abs() < 1e-3,
            "rate {rate}: {frame:?}"
        );
    }
}

/// Level from 0.25 to 0.75 s of `engine` after a note-on of `note`.
fn note_level(mut engine: Engine, note: u8) -> f64 {
    engine.set_playing(true);
    engine.midi(Message::NoteOn {
        channel: 0,
        note,
        velocity: 127,
    });
    for _ in 0..RATE / 4 {
        engine.process_frame();
    }
    rms((0..RATE / 2).map(|_| {
        let frame = engine.process_frame();
        (frame[0] + frame[1]) * 0.5
    }))
}

/// A Sampler pad playing `hz` a fifth up.
fn sampler_at_a_fifth(hz: &[f64]) -> f64 {
    let mut preset = Kind::Sampler.preset().unwrap();
    let settings = preset.sampler.as_deref_mut().unwrap();
    let mut zone = Zone::new("tones");
    zone.tune = 7.0;
    settings.pads[0].zones.push(zone);
    let key = settings.pads[0].keys[0];
    let mut engine = Engine::new(RATE, Arc::new(SampleBuffer::demo(RATE)), preset);
    let pool = engine.sampler_pool_mut().unwrap();
    pool.insert("tones".into(), Arc::new(tones(hz, RATE as usize * 2)));
    engine.refresh_sampler();
    note_level(engine, key)
}

#[test]
fn sampler_voices_a_fifth_up_fold_nothing_audible_back() {
    // 22-23.4 kHz plays at 33-35 kHz, which folds to 13-15 kHz.
    let folded = sampler_at_a_fifth(&[22_000.0, 22_700.0, 23_400.0]);
    let played = sampler_at_a_fifth(&[5_000.0, 6_000.0, 7_000.0]);
    let rejection = 20.0 * (folded / played).log10();
    assert!(rejection < -45.0, "folded back at {rejection:.1} dB");
}

/// A Synth engine PCM oscillator `semitones` above the sample's root.
fn synth_pcm(hz: &[f64], semitones: f32) -> f64 {
    let mut preset = Kind::Synth.preset().unwrap();
    let synth = preset.synth.as_mut().unwrap();
    synth.count = 1;
    let o = &mut synth.oscillators[0];
    o.waveform = Waveform::Pcm;
    o.transpose = semitones;
    o.level = 1.0;
    preset.params.filter = 1.0;
    preset.params.reverb = 0.0;
    preset.color_chain.clear();
    let root = preset.params.sample_root_note as u8;
    let mut engine = Engine::new(RATE, Arc::new(tones(hz, RATE as usize * 2)), preset);
    engine.set_midi_gate(true);
    note_level(engine, root)
}

#[test]
fn synth_pcm_carriers_pitched_up_fold_nothing_audible_back() {
    // Carriers run at 96 kHz: 23 semitones up, 22-23.4 kHz plays at
    // 83-88 kHz, which folds to 8-13 kHz.
    let folded = synth_pcm(&[22_000.0, 22_700.0, 23_400.0], 23.0);
    let played = synth_pcm(&[2_000.0, 3_000.0, 4_000.0], 23.0);
    let rejection = 20.0 * (folded / played).log10();
    assert!(rejection < -45.0, "folded back at {rejection:.1} dB");
}

/// A Resynth PCM layer reading high tones from a file whose root is the note
/// `root`, while its top band (12 kHz) hears a tone at its centre.
fn resynth_pcm(root: f32) -> f64 {
    let sample = tones(&[22_000.0, 22_700.0, 23_400.0], RATE as usize * 2);
    let mut settings = sad_pet::resynth::Settings {
        enabled: true,
        count: 1,
        mix: 1.0,
        ..Default::default()
    };
    let o = &mut settings.oscillators[0];
    o.waveform = Waveform::Pcm;
    o.mode = Mode::Wave;
    let mut resynth = Resynth::new(RATE);
    resynth.configure(settings, &sample, root);
    let range = 0..sample.len();
    let mut next = |i: usize| {
        let heard = (i as f64 * 12_000.0 * TAU / RATE as f64).sin() as f32 * 0.25;
        let frame = resynth.process([0.0; 2], [heard; 2], 0.0, 1.0, &sample, &range);
        (frame[0] + frame[1]) * 0.5
    };
    let half = RATE as usize / 2;
    for i in 0..half {
        next(i);
    }
    rms((half..2 * half).map(next))
}

#[test]
fn resynth_pcm_carriers_pitched_up_fold_nothing_audible_back() {
    // From a root of 12.5 kHz the top band's carrier plays the tones at
    // 21-22.4 kHz. From 3.16 kHz it plays them at 84-89 kHz, which the
    // carriers' 96 kHz folds to 7-12 kHz.
    let played = resynth_pcm(127.0);
    let folded = resynth_pcm(103.1);
    let rejection = 20.0 * (folded / played).log10();
    assert!(rejection < -45.0, "folded back at {rejection:.1} dB");
}
