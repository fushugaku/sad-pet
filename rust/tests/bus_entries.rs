//! A timbre can enter a Bus or Master chain after its input: at Pre-filter
//! FX, Filters or Post-filter FX, skipping the sections before that entry.
use sad_pet::{
    color::ColorSlot,
    dsp::{Engine, SampleBuffer},
    preset::{Preset, SaturatorType},
    timbres::{DESTINATIONS, INPUT, Kind, Meta, Mix, SendGains},
};
use std::sync::Arc;

fn bus(edit: impl FnOnce(&mut Preset)) -> Engine {
    let mut preset = Kind::Bus.preset().unwrap();
    edit(&mut preset);
    let mut e = Engine::new(48_000, Arc::new(SampleBuffer::demo(48_000)), preset);
    e.set_playing(true);
    e
}
fn input(n: usize) -> [f32; 2] {
    let t = n as f32 / 48_000.0;
    [
        (t * 220.0 * std::f32::consts::TAU).sin() * 0.3,
        (t * 3_100.0 * std::f32::consts::TAU).sin() * 0.2,
    ]
}
fn distortion() -> Vec<ColorSlot> {
    vec![ColorSlot {
        id: 1,
        kind: SaturatorType::Distortion,
        mix: 1.0,
        ..Default::default()
    }]
}
/// The largest difference between two buses fed the same signal, one at its
/// input and one at entry `at` (0 Pre-filter FX, 1 Filters, 2 Post-filter FX).
fn difference(edit: impl Fn(&mut Preset), at: usize, reference: &mut Engine) -> f32 {
    let mut entering = bus(&edit);
    let mut error = 0.0_f32;
    for n in 0..8_000 {
        let x = input(n);
        let mut entered = [[0.0; 2]; 3];
        entered[at] = x;
        entering.enter(entered);
        let (y, r) = (
            entering.process_input_frame([0.0; 2]),
            reference.process_input_frame(x),
        );
        if n > 2_000 {
            for c in 0..2 {
                error = error.max((y[c] - r[c]).abs());
            }
        }
    }
    error
}

#[test]
fn a_passthrough_bus_adds_every_entry_to_its_input_bit_for_bit() {
    let mut plain = bus(|_| {});
    let mut entering = bus(|_| {});
    for n in 0..4_000 {
        let x = input(n);
        let parts = [
            x.map(|v| v * 0.5),
            x.map(|v| v * 0.25),
            x.map(|v| v * 0.125),
        ];
        entering.enter([parts[0], parts[1], parts[2]]);
        let sum = std::array::from_fn(|c| x[c] * 0.125 + parts[0][c] + parts[1][c] + parts[2][c]);
        assert_eq!(
            entering.process_input_frame(x.map(|v| v * 0.125)),
            plain.process_input_frame(sum),
            "frame {n}"
        );
    }
}

#[test]
fn a_later_entry_skips_pre_filter_fx() {
    let pre = |p: &mut Preset| p.pre_color_chain = distortion();
    // Entering at Pre-filter FX is the input of a bus without Granular or Resynth.
    let at_pre = difference(pre, 0, &mut bus(pre));
    assert!(
        at_pre < 1e-5,
        "Pre-filter FX entry runs the distortion ({at_pre})"
    );
    // At Filters or Post-filter FX the input passes as if the rack were empty.
    for at in [1, 2] {
        let skipped = difference(pre, at, &mut bus(|_| {}));
        assert!(skipped < 1e-5, "entry {at} skips Pre-filter FX ({skipped})");
    }
    let distorted = difference(|_| {}, 0, &mut bus(pre));
    assert!(
        distorted > 0.01,
        "the distortion changes the input ({distorted})"
    );
}

#[test]
fn post_filter_fx_entry_skips_the_filters_and_still_runs_its_rack() {
    let filters = |p: &mut Preset| {
        p.bus.as_mut().unwrap().filters = true;
        p.params.filter = 0.05;
    };
    let dark = difference(filters, 1, &mut bus(|_| {}));
    assert!(dark > 0.01, "Filters entry is filtered ({dark})");
    let skipped = difference(filters, 2, &mut bus(|_| {}));
    assert!(
        skipped < 1e-5,
        "Post-filter FX entry skips the filters ({skipped})"
    );
    let post = |p: &mut Preset| p.color_chain = distortion();
    let racked = difference(post, 2, &mut bus(post));
    assert!(
        racked < 1e-5,
        "Post-filter FX entry runs the rack ({racked})"
    );
}

#[test]
fn entries_ramp_from_the_input_and_settle_back_to_the_plain_sum() {
    let mut gains = SendGains::default();
    let mut levels = [INPUT; DESTINATIONS];
    assert_eq!(gains.plan(&levels), 0, "nothing splits at the defaults");
    levels[2] = [0.0, 0.0, 0.0, 1.0];
    assert_eq!(gains.plan(&levels), 1 << 2);
    let (mut input, mut entered) = ([0.0; 2], [[0.0; 2]; 3]);
    gains.enter(2, &levels[2], [1.0; 2], &mut input, &mut entered);
    assert!(
        input[0] > 0.99 && entered[2][0] > 0.0 && entered[2][0] < 0.01,
        "ramps from the input"
    );
    for _ in 0..4_000 {
        (input, entered) = ([0.0; 2], [[0.0; 2]; 3]);
        gains.enter(2, &levels[2], [1.0; 2], &mut input, &mut entered);
    }
    assert_eq!(
        (input, entered[2]),
        ([0.0; 2], [1.0; 2]),
        "settles at the level"
    );
    levels[2] = INPUT;
    for _ in 0..4_000 {
        gains.enter(2, &levels[2], [1.0; 2], &mut input, &mut entered);
    }
    assert_eq!(
        gains.plan(&levels),
        0,
        "back at the input, the plain sum returns"
    );
}

#[test]
fn entries_serialize_only_when_set_and_a_bus_never_enters_itself() {
    let mut mix = Mix::default();
    let plain = serde_json::to_value(mix).unwrap();
    assert!(
        plain.get("entries").is_none(),
        "default entries stay out of scenes"
    );
    mix.entries[3] = [0.5, 0.0, 2.0, f32::NAN];
    mix.sanitize();
    assert_eq!(mix.entries[3], [0.5, 0.0, 1.0, 0.0]);
    let recalled: Mix = serde_json::from_value(serde_json::to_value(mix).unwrap()).unwrap();
    assert_eq!(recalled.entries, mix.entries);
    let mut meta = Meta {
        kind: Kind::Bus,
        bus: 1,
        ..Default::default()
    };
    meta.mix.entries[0] = [0.0, 0.0, 0.0, 1.0];
    meta.mix.entries[1] = [0.0, 1.0, 0.0, 0.0];
    meta.mix.entries[3] = [0.0, 1.0, 0.0, 0.0];
    meta.sanitize();
    assert_eq!(
        meta.mix.entries[0],
        [0.0, 0.0, 0.0, 1.0],
        "a Bus enters Master"
    );
    assert_eq!(
        meta.mix.entries[3],
        [0.0, 1.0, 0.0, 0.0],
        "a Bus enters the other buses"
    );
    assert_eq!(
        meta.mix.entries[1], INPUT,
        "a Bus never enters its own chain"
    );
    meta.kind = Kind::Master;
    meta.sanitize();
    assert_eq!(
        meta.mix.entries, [INPUT; DESTINATIONS],
        "Master enters nothing"
    );
    assert!(mix.forget_bus(3));
    assert_eq!(mix.entries[3], INPUT);
    assert!(!mix.forget_bus(3));
}
