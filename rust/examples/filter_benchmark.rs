//! Per-voice filter bank cost for every model, with control-rate cutoff
//! sweeps as in the engine. No audio device. The checksum covers every output
//! bit, so two builds can also be compared for identical results.
//! cargo run -p sad-pet --no-default-features --release --example filter_benchmark
use sad_pet::{
    dsp::filter::FilterBank,
    preset::{FilterModel, FilterRouting, FilterType, Params},
};
use std::{hint::black_box, time::Instant};

fn main() {
    let rate = 48_000;
    let mut seed = 0x51a3_beef_u32;
    let input: Vec<[f32; 2]> = (0..rate as usize)
        .map(|_| {
            std::array::from_fn(|_| {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                seed as i32 as f32 / 2_147_483_648.0 * 0.6
            })
        })
        .collect();
    for model in FilterModel::ALL {
        for kind in [FilterType::LowPass, FilterType::BandPass] {
            let mut p = Params {
                filter_model: model,
                filter2_model: model,
                vcf_type: kind,
                vcf2_type: kind,
                filter_routing: FilterRouting::Serial,
                filter_resonance: 0.6,
                filter2_resonance: 0.3,
                filter_drive: 0.25,
                filter_morph: 0.4,
                ..Params::default()
            };
            let mut bank = FilterBank::new(rate);
            let mut checksum = 0_u32;
            let mut times = Vec::new();
            for pass in 0..6 {
                let start = Instant::now();
                for (i, frame) in input.iter().enumerate() {
                    if i % 32 == 0 {
                        p.filter = 0.3 + 0.4 * (i as f32 / rate as f32);
                        p.filter2 = 0.8 - 0.2 * (i as f32 / rate as f32);
                        bank.configure(&p, 60.0, [0.0_f32; 4]);
                    }
                    let out = bank.process(black_box(*frame));
                    checksum = checksum.rotate_left(5) ^ out[0].to_bits() ^ out[1].to_bits() << 1;
                }
                if pass > 0 {
                    times.push(start.elapsed().as_secs_f64() * 1e9 / input.len() as f64);
                }
            }
            times.sort_by(f64::total_cmp);
            println!(
                "{model:?}/{kind:?}: {:.2} ns/frame checksum={checksum:08x}",
                times[times.len() / 2]
            );
        }
    }
}
