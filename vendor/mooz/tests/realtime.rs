//! Contract every model must keep inside SAD PET's audio thread: no allocation
//! in `set`/`process`/`clear`, finite bounded output, sane defaults.
//! `MOOZ_ONLY=mooz_delay,mooz_hall cargo test` limits the run to some keys.
use sad_pet_mooz::{MAX_PARAMS, Model, Values, dsp::Noise};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    time::Instant,
};

struct Counting;
thread_local! {
    static GUARDED: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if GUARDED.with(Cell::get) {
            ALLOCATIONS.with(|count| count.set(count.get() + 1));
        }
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if GUARDED.with(Cell::get) {
            ALLOCATIONS.with(|count| count.set(count.get() + 1));
        }
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if GUARDED.with(Cell::get) {
            ALLOCATIONS.with(|count| count.set(count.get() + 1));
        }
        unsafe { System.realloc(ptr, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn guarded<T>(f: impl FnOnce() -> T) -> (T, usize) {
    ALLOCATIONS.with(|count| count.set(0));
    GUARDED.with(|g| g.set(true));
    let result = f();
    GUARDED.with(|g| g.set(false));
    (result, ALLOCATIONS.with(Cell::get))
}

fn selected() -> Vec<Model> {
    let only = std::env::var("MOOZ_ONLY").unwrap_or_default();
    let keys: Vec<&str> = only.split(',').filter(|k| !k.is_empty()).collect();
    Model::ALL
        .iter()
        .copied()
        .filter(|model| keys.is_empty() || keys.contains(&model.key()))
        .collect()
}

/// Guitar-like test signal: plucked decaying harmonics plus a little noise.
fn source(noise: &mut Noise, index: usize, sample_rate: f32) -> [f32; 2] {
    let t = index as f32 / sample_rate;
    let note = (t * 2.0).fract();
    let envelope = (-note * 6.0).exp();
    let phase = t * 196.0;
    let tone = (0.5 * (phase * std::f32::consts::TAU).sin()
        + 0.25 * (phase * 2.0 * std::f32::consts::TAU).sin()
        + 0.12 * (phase * 3.0 * std::f32::consts::TAU).sin())
        * envelope;
    let n = noise.bipolar() * 0.02;
    [0.5 * tone + n, 0.45 * tone - n]
}

fn random_values(model: Model, noise: &mut Noise, extreme: bool) -> Values {
    let mut values = [0.0; MAX_PARAMS];
    for (value, param) in values.iter_mut().zip(model.info().params) {
        let max = f32::from(param.max);
        *value = if extreme {
            if noise.unit() < 0.5 { 0.0 } else { max }
        } else {
            noise.unit() * max
        };
    }
    values
}

#[test]
fn catalog_is_consistent() {
    let mut keys = std::collections::BTreeSet::new();
    for &model in Model::ALL {
        let info = model.info();
        assert!(info.key.starts_with("mooz_"), "{}", info.key);
        assert!(keys.insert(info.key), "duplicate key {}", info.key);
        assert!(info.params.len() <= MAX_PARAMS, "{}", info.key);
        assert_eq!(Model::from_key(info.key), Some(model));
        for param in info.params {
            assert!(param.default <= param.max, "{} {}", info.key, param.name);
            if let sad_pet_mooz::Display::Labels { labels } = param.display {
                assert_eq!(
                    labels.len(),
                    usize::from(param.max) + 1,
                    "{} {}",
                    info.key,
                    param.name
                );
            }
        }
    }
    assert_eq!(Model::ALL.len(), 137);
}

#[test]
fn every_model_is_realtime_safe_and_bounded() {
    for model in selected() {
        for sample_rate in [44_100.0, 48_000.0, 96_000.0] {
            let mut effect = model.create(sample_rate);
            let mut noise = Noise::new(0x5eed);
            let frames = sample_rate as usize * 2;
            let ((), allocations) = guarded(|| {
                effect.set(&model.defaults(), 120.0);
                let mut peak = 0.0f32;
                for i in 0..frames {
                    let input = source(&mut noise, i, sample_rate);
                    let output = effect.process(input);
                    for x in output {
                        assert!(
                            x.is_finite(),
                            "{} at {sample_rate}: non-finite output",
                            model.key()
                        );
                        peak = peak.max(x.abs());
                    }
                }
                assert!(
                    peak < 16.0,
                    "{} at {sample_rate}: default peak {peak}",
                    model.key()
                );
                for round in 0..40 {
                    let values = random_values(model, &mut noise, round % 4 == 3);
                    effect.set(&values, 60.0 + round as f32 * 7.0);
                    for i in 0..(sample_rate as usize / 20) {
                        let output = effect.process(source(&mut noise, i, sample_rate));
                        for x in output {
                            assert!(
                                x.is_finite() && x.abs() < 64.0,
                                "{} at {sample_rate}: {x} with {values:?}",
                                model.key()
                            );
                        }
                    }
                }
                effect.clear();
                effect.set(&model.defaults(), 120.0);
                for _ in 0..1000 {
                    let output = effect.process([0.0; 2]);
                    assert!(output.iter().all(|x| x.is_finite()), "{}", model.key());
                }
            });
            assert_eq!(
                allocations,
                0,
                "{} at {sample_rate}: allocated on the audio thread",
                model.key()
            );
        }
    }
}

/// `cargo test --release --test realtime -- --ignored --nocapture cost`
#[test]
#[ignore]
fn cost() {
    for model in selected() {
        let sample_rate = 48_000.0;
        let mut effect = model.create(sample_rate);
        effect.set(&model.defaults(), 120.0);
        let mut noise = Noise::new(7);
        let frames = 48_000 * 4;
        let input: Vec<[f32; 2]> = (0..frames)
            .map(|i| source(&mut noise, i, sample_rate))
            .collect();
        let start = Instant::now();
        let mut sum = 0.0;
        for frame in &input {
            let output = effect.process(*frame);
            sum += output[0] + output[1];
        }
        let ns = start.elapsed().as_nanos() as f64 / frames as f64;
        println!("{:<22} {:>8.1} ns/frame ({sum:.1})", model.key(), ns);
    }
}
