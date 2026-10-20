//! Silent 128-frame budget check for the maximum 64-carrier configuration.
use sad_pet::{
    dsp::{Engine, SampleBuffer},
    preset::Preset,
    resynth::{Mode, Waveform},
};
use std::{hint::black_box, sync::Arc, time::Instant};
fn main() {
    for rate in [44100, 48000, 96000] {
        for (wave, mode) in [
            (Waveform::Sine, Mode::Wave),
            (Waveform::Saw, Mode::Unison),
            (Waveform::Sine, Mode::Vpm),
            (Waveform::Formant, Mode::Wave),
            (Waveform::Noise, Mode::Wave),
            (Waveform::Pcm, Mode::Wave),
        ] {
            let mut p = Preset::default();
            p.resynth.enabled = true;
            p.resynth.count = 4;
            p.resynth.mix = 1.0;
            for (i, o) in p.resynth.oscillators.iter_mut().enumerate() {
                o.waveform = wave;
                o.mode = mode;
                o.control1 = 0.5;
                o.control2 = 0.3;
                o.transpose = i as f32 * 0.13;
            }
            let mut e = Engine::new(rate, Arc::new(SampleBuffer::demo(rate)), p);
            e.set_playing(true);
            let mut times = Vec::new();
            let mut peak = 0.0_f32;
            for block in 0..(rate / 128) {
                let start = Instant::now();
                for _ in 0..128 {
                    let f = black_box(e.process_frame());
                    peak = peak.max(f[0].abs()).max(f[1].abs());
                }
                if block > 20 {
                    times.push(start.elapsed().as_secs_f64() * 1e6);
                }
            }
            times.sort_by(f64::total_cmp);
            let budget = 128.0 / rate as f64 * 1e6;
            println!(
                "{rate} {wave:?}/{mode:?}: median={:.1}us ({:.1}%) p99={:.1}us peak={peak:.3}",
                times[times.len() / 2],
                times[times.len() / 2] / budget * 100.0,
                times[times.len() * 99 / 100]
            );
        }
    }
}
