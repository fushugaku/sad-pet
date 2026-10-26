//! A Bus sends to other buses by its mixer sends and its sections, never in
//! a loop: buses render after every bus that feeds them.
use sad_pet::{
    preset::Preset,
    timbres::{BusOrder, Feeds, Kind, Meta, bus_feeds, closes_loop, refuse_loops, tapped},
};

fn bus(number: u8, sends: &[(u8, f32)]) -> Meta {
    let mut meta = Meta {
        id: number as u32 + 10,
        kind: Kind::Bus,
        bus: number,
        ..Default::default()
    };
    for &(to, level) in sends {
        meta.mix.sends[to as usize - 1] = level;
    }
    meta
}
fn feeds(buses: &[(Meta, Preset)]) -> Feeds {
    bus_feeds(buses.iter().map(|(m, p)| (m, p)))
}
fn waves(order: &BusOrder) -> Vec<Vec<u8>> {
    order.waves().map(<[u8]>::to_vec).collect()
}

#[test]
fn a_bus_feeds_the_buses_it_sends_to_or_its_sections_are_routed_to() {
    let mut tapping = Preset::default();
    tapping.taps.filters.bus = 3;
    // A chosen bus counts at any send level: modulation can open it.
    tapping.taps.filters.send = 0.0;
    assert_eq!(tapped(&tapping), 0b100);
    // Grain groups route only in advanced mode.
    tapping.grain_groups.groups[1].output.bus = 4;
    assert_eq!(tapped(&tapping), 0b100);
    tapping.grain_groups.advanced = true;
    assert_eq!(tapped(&tapping), 0b1100);
    let scene = [
        (bus(1, &[(2, 0.5), (3, 0.0)]), Preset::default()),
        (bus(2, &[]), tapping),
        (bus(3, &[]), Preset::default()),
        (bus(4, &[]), Preset::default()),
        // Other timbres send to buses but are no part of the bus graph.
        (
            Meta {
                kind: Kind::Synth,
                mix: bus(9, &[(1, 1.0)]).mix,
                ..Default::default()
            },
            Preset::default(),
        ),
    ];
    assert_eq!(feeds(&scene)[..4], [0b10, 0b1100, 0, 0]);
    // A send to a bus that does not exist feeds nothing.
    let lonely = [(bus(1, &[(5, 1.0)]), Preset::default())];
    assert_eq!(feeds(&lonely)[0], 0);
}

#[test]
fn a_route_that_would_close_a_loop_is_refused() {
    // Bus 3 → Bus 1 → Bus 2.
    let scene = [
        (bus(1, &[(2, 1.0)]), Preset::default()),
        (bus(2, &[]), Preset::default()),
        (bus(3, &[(1, 1.0)]), Preset::default()),
    ];
    let feeds = feeds(&scene);
    assert!(closes_loop(&feeds, 2, 1), "Bus 1 already feeds Bus 2");
    assert!(closes_loop(&feeds, 2, 3), "Bus 3 feeds Bus 2 through Bus 1");
    assert!(closes_loop(&feeds, 2, 2), "a bus never feeds itself");
    assert!(closes_loop(&feeds, 1, 3), "Bus 3 already feeds Bus 1");
    assert!(
        !closes_loop(&feeds, 3, 2),
        "Bus 3 may feed Bus 2 directly too"
    );
    assert!(
        !closes_loop(&feeds, 2, 4),
        "a bus outside the chain is free"
    );
    let mut mix = bus(2, &[(1, 0.5), (3, 1.0), (4, 0.25)]).mix;
    assert!(refuse_loops(&mut mix, &feeds, 2));
    assert_eq!(mix.sends[..4], [0.0, 0.0, 0.0, 0.25]);
    assert!(!refuse_loops(&mut mix, &feeds, 2));
    // Its own routes stay: Bus 1 keeps sending to Bus 2.
    let mut mix = scene[0].0.mix;
    assert!(!refuse_loops(&mut mix, &feeds, 1));
    assert_eq!(mix.sends[1], 1.0);
}

#[test]
fn a_loop_made_anyway_loses_the_later_buss_route() {
    // Bus 2's section sends back to Bus 1, as automation or a loaded preset
    // could make it.
    let mut back = Preset::default();
    back.taps.post_fx.bus = 1;
    let scene = [
        (bus(1, &[(2, 1.0)]), Preset::default()),
        (bus(2, &[]), back),
    ];
    let feeds = feeds(&scene);
    assert_eq!(feeds[..2], [0b10, 0]);
    let order = BusOrder::new([1, 2], &feeds);
    assert_eq!(order.buses(), [1, 2]);
}

#[test]
fn buses_render_after_every_bus_that_feeds_them() {
    let none = Feeds::default();
    let order = BusOrder::new([3, 1, 2], &none);
    assert_eq!(
        waves(&order),
        [vec![3, 1, 2]],
        "no routes: one wave in list order"
    );
    assert_eq!(BusOrder::new([], &none).buses(), [] as [u8; 0]);
    // Bus 2 → Bus 1 → Bus 4, Bus 3 alone, listed 1, 2, 3, 4.
    let mut feeds = Feeds::default();
    feeds[1] = 0b1;
    feeds[0] = 0b1000;
    let order = BusOrder::new([1, 2, 3, 4], &feeds);
    assert_eq!(waves(&order), [vec![2, 3], vec![1], vec![4]]);
    assert_eq!(order.buses(), [2, 3, 1, 4]);
    // Two buses feeding one: it waits for both.
    let mut feeds = Feeds::default();
    feeds[0] = 0b100;
    feeds[1] = 0b100;
    let order = BusOrder::new([3, 2, 1], &feeds);
    assert_eq!(waves(&order), [vec![2, 1], vec![3]]);
    // A bus listed twice or out of range is skipped; a route to a bus that
    // is not listed does not hold anything back.
    let mut feeds = Feeds::default();
    feeds[6] = 0b1;
    let order = BusOrder::new([1, 1, 0, 9], &feeds);
    assert_eq!(order.buses(), [1]);
    // A loop, which bus_feeds never leaves, still renders every bus once.
    let mut feeds = Feeds::default();
    feeds[0] = 0b10;
    feeds[1] = 0b1;
    let order = BusOrder::new([2, 1], &feeds);
    assert_eq!(order.buses(), [2, 1]);
}

#[test]
fn a_bus_keeps_its_sends_to_other_buses() {
    let mut meta = bus(2, &[(1, 0.5), (2, 1.0), (3, 2.5)]);
    meta.feeds = 0b101;
    meta.sanitize();
    assert_eq!(meta.mix.sends[..3], [0.5, 0.0, 2.0], "never to itself");
    assert_eq!(meta.feeds, 0b101, "the host's note stays on a bus");
    meta.kind = Kind::Master;
    meta.sanitize();
    assert_eq!(meta.mix.sends, [0.0; 8], "Master sends nowhere");
    assert_eq!(meta.feeds, 0);
    // Old scenes without the field read, and a bus feeding nothing writes none.
    let json = serde_json::to_value(bus(1, &[])).unwrap();
    assert!(json.get("feeds").is_none(), "{json}");
    let read: Meta = serde_json::from_value(json).unwrap();
    assert_eq!(read.feeds, 0);
}
