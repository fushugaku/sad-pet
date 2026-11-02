//! Local render timing: cargo run --release -p sad-pet --example oscillator_bench.
//! Preparation and JSON are outside the measured callback-sized loops.
use sad_pet::{
    dsp::{Engine, SampleBuffer},
    interaction::{Capture, CaptureMode, Node, Route, Scope, SyncMode},
    midi::Message,
    oscillators::{Kind, Slot},
    rack::{Module, Slot as RackSlot},
};
use std::{sync::Arc, time::Instant};

fn main() {
    let sample = Arc::new(SampleBuffer::demo(48000));
    let mut results = Vec::new();
    for (name, count, voices, groups, births) in [
        ("1 RADIAS / 1 voice", 1, 1, false, false),
        ("4 mixed / 1 voice", 4, 1, false, false),
        ("8 mixed / 1 voice", 8, 1, false, false),
        ("8 mixed / 4 voices", 8, 4, false, false),
        (
            "8 mixed + group Playback/Ring / 4 voices",
            8,
            4,
            true,
            false,
        ),
        (
            "8 mixed + group cycle / 4 voices (audio-rate births)",
            8,
            4,
            true,
            true,
        ),
    ] {
        let mut p = sad_pet::timbres::Kind::Oscillators.preset().unwrap();
        let bank = p.oscillators.as_mut().unwrap();
        for i in 0..count {
            bank.slots[i] = Slot {
                id: i as u32 + 1,
                kind: [Kind::Radias, Kind::Wavefolder, Kind::Pcm, Kind::Wavetable][i % 4],
                ..Default::default()
            };
            bank.slots[i].controls.level = 0.1;
            bank.slots[i].controls.transpose = i as f32;
        }
        bank.next_id = count as u32 + 1;
        p.params.filter = 1.0;
        p.params.filter2 = 1.0;
        p.color_chain.clear();
        p.pre_color_chain.clear();
        if groups {
            p.rack.as_mut().unwrap().modules.insert(
                1,
                RackSlot {
                    module: Module::Grains,
                    ..Default::default()
                },
            );
            p.grain_capture = Capture {
                mode: CaptureMode::Osc,
                slots: 85,
            };
            p.grain_groups.advanced = true;
            p.interactions.routes = vec![
                Route {
                    source: Node::Osc { id: 4 },
                    target: Node::Osc { id: 1 },
                    ring: 0.5,
                    sync: SyncMode::Hard,
                    ..Default::default()
                },
                Route {
                    source: Node::Osc { id: 1 },
                    target: Node::Group { index: 0 },
                    sync: if births {
                        SyncMode::Hard
                    } else {
                        SyncMode::Soft
                    },
                    scope: if births { Scope::Both } else { Scope::Playback },
                    ..Default::default()
                },
                Route {
                    source: Node::Group { index: 0 },
                    target: Node::Osc { id: 8 },
                    ring: 0.5,
                    sync: SyncMode::Soft,
                    ..Default::default()
                },
                Route {
                    source: Node::Group { index: 1 },
                    target: Node::Group { index: 2 },
                    ring: 0.5,
                    sync: SyncMode::Hard,
                    scope: Scope::Clock,
                    ..Default::default()
                },
            ];
        }
        let mut engine = Engine::new(48000, sample.clone(), p);
        engine.set_midi_gate(true);
        engine.set_playing(true);
        for &note in [60, 64, 67, 72].iter().take(voices) {
            engine.midi(Message::NoteOn {
                channel: 0,
                note,
                velocity: 100,
            });
        }
        for _ in 0..256 * 128 {
            std::hint::black_box(engine.process_frame());
        }
        let mut times = Vec::with_capacity(512);
        let mut block = [[0.0; 2]; 128];
        let mut peak = 0.0_f32;
        for _ in 0..512 {
            let begin = Instant::now();
            for frame in &mut block {
                *frame = engine.process_frame();
            }
            times.push(begin.elapsed().as_secs_f64() * 1000.0);
            for &value in block.iter().flatten() {
                assert!(value.is_finite());
                peak = peak.max(value.abs());
            }
        }
        assert!(peak > 1e-5, "silent render: {name}");
        times.sort_by(f64::total_cmp);
        results.push(serde_json::json!({"case":name,"p50_ms":times[256],"p95_ms":times[486],"p99_ms":times[506],"worst_ms":times[511],"deadline_ms":128.0/48.0}));
    }
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({"variant":"native release","rate":48000,"frames":128,"results":results})).unwrap());
}
