//! Reproducible FM/sync CPU and differential-audio harness. The same binary
//! interface is used before/after; no asset compilation is timed.
use sad_pet::{
    dsp::{Engine, SampleBuffer},
    midi::Message,
    preset::Preset,
    timbres::Kind,
    wavetable::factory::FACTORY,
};
use serde_json::json;
use std::{hint::black_box, sync::Arc, time::Instant};

fn cpu_time() -> f64 {
    #[cfg(unix)]
    {
        let mut t = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        assert_eq!(
            unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut t) },
            0
        );
        t.tv_sec as f64 + t.tv_nsec as f64 * 1e-9
    }
    #[cfg(not(unix))]
    {
        0.0
    }
}
fn preset(mode: &str, unison: u8) -> Preset {
    let routed = mode.starts_with("routed-");
    let mode = mode.strip_prefix("routed-").unwrap_or(mode);
    let mut p = Kind::Wavetable.preset().unwrap();
    let s = p.wavetable.as_mut().unwrap();
    s.count = 4;
    for (i, o) in s.oscillators.iter_mut().enumerate() {
        o.asset_id = FACTORY[[32, 41, 52, 65][i]].id.into();
        let c = &mut o.controls;
        c.enabled = true;
        c.level = 0.07;
        c.position = 0.27 + i as f32 * 0.13;
        c.unison = unison;
        c.detune = 7.3;
        c.spread = 0.75;
        c.phase_random = 0.7;
        c.voice_shift = 0.06;
        c.transpose = [0.0, 12.0, 7.0, 19.0][i];
        c.sync = mode == "sync" || mode == "mixed";
        c.sync_ratio = 2.75 + i as f32 * 0.5;
        if i % 2 == 0 || routed {
            c.fm_mode = match mode {
                "fm" | "mixed" => 1,
                "pm" => 2,
                _ => 0,
            };
            c.fm_index = 2.0;
        }
        if routed {
            c.fm_source = ((i + 3) % 4) as u8;
            c.sync_source = ((i + 1) % 4) as u8;
        }
    }
    p.envs[0].attack = 0.0;
    p.envs[0].decay = 0.0;
    p.envs[0].sustain = 1.0;
    p.params.adsr_attack = 0.0;
    p.params.adsr_decay = 0.0;
    p.params.adsr_sustain = 1.0;
    p
}
fn engine(mode: &str, unison: u8, rate: u32) -> Engine {
    let sample = Arc::new(SampleBuffer::new("Unused".into(), rate, vec![[0.0; 2]; 2]).unwrap());
    let mut e = Engine::new(rate, sample, preset(mode, unison));
    e.set_midi_gate(true);
    e.set_playing(true);
    e
}
fn note(e: &mut Engine, i: u8, on: bool) {
    let pitch = 48 + (i / 4) * 12 + [0, 3, 7, 10][i as usize % 4];
    e.midi(if on {
        Message::NoteOn {
            channel: 0,
            note: pitch,
            velocity: 100,
        }
    } else {
        Message::NoteOff {
            channel: 0,
            note: pitch,
            velocity: 0,
        }
    });
}
fn quantiles(mut data: Vec<f64>, deadline: f64) -> serde_json::Value {
    data.sort_by(f64::total_cmp);
    json!({"p50_ms":data[data.len()/2]*1000.0,"p95_ms":data[data.len()*95/100]*1000.0,
        "p99_ms":data[data.len()*99/100]*1000.0,"max_ms":data[data.len()-1]*1000.0,
        "p99_deadline_fraction":data[data.len()*99/100]/deadline})
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.get(1).map(String::as_str) == Some("presets") {
        let patches: serde_json::Map<String, serde_json::Value> =
            ["off", "fm", "pm", "sync", "mixed"]
                .into_iter()
                .map(|mode| (mode.into(), serde_json::to_value(preset(mode, 4)).unwrap()))
                .collect();
        println!("{}", serde_json::Value::Object(patches));
    } else if args.get(1).map(String::as_str) == Some("cpu") {
        let mode = args.get(2).map(String::as_str).unwrap_or("fm");
        let notes: u8 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(8);
        let unison: u8 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(4);
        let seconds: f64 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(3.0);
        let block: usize = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(128);
        let rate: u32 = args.get(7).and_then(|s| s.parse().ok()).unwrap_or(48000);
        let mut e = engine(mode, unison, rate);
        for i in 0..notes {
            note(&mut e, i, true);
        }
        let warm = rate as usize / 4 / block;
        let blocks = (seconds * rate as f64 / block as f64).ceil() as usize;
        let (mut wall, mut cpu) = (Vec::with_capacity(blocks), Vec::with_capacity(blocks));
        let mut peak = 0.0_f32;
        for b in 0..blocks + warm {
            let ct = cpu_time();
            let at = Instant::now();
            for _ in 0..block {
                for v in black_box(e.process_frame()) {
                    assert!(v.is_finite());
                    peak = peak.max(v.abs());
                    black_box(v);
                }
            }
            let dt = at.elapsed().as_secs_f64();
            let dc = cpu_time() - ct;
            if b >= warm {
                wall.push(dt);
                cpu.push(dc);
            }
        }
        println!(
            "{}",
            json!({"mode":mode,"notes":notes,"waves":4,"unison":unison,"sample_rate":rate,"block":block,
            "audio_seconds":seconds,"warmup_audio_seconds":warm as f64*block as f64/rate as f64,"peak":peak,
            "wall":quantiles(wall,block as f64/rate as f64),"thread_cpu":quantiles(cpu,block as f64/rate as f64),
            "scope":"Shared production Engine, four different complex tables, detune and Voice shift, both FM pairs or all four synced. Compilation excluded; thread CPU excludes descheduling, wall time includes contention."})
        );
    } else if args.get(1).map(String::as_str) == Some("capture") {
        let folder = std::path::Path::new(args.get(2).expect("capture directory"));
        std::fs::create_dir_all(folder).unwrap();
        let mut rows = Vec::new();
        for rate in [44100, 48000, 96000] {
            for mode in ["off", "fm", "pm", "sync", "mixed"] {
                for notes in [1, 8] {
                    let mut e = engine(mode, 4, rate);
                    for i in 0..notes {
                        note(&mut e, i, true);
                    }
                    let mut samples = Vec::with_capacity(12288 * 8);
                    for at in 0..12288 {
                        if at % 128 == 0 {
                            let t = at as f32 / 12288.0;
                            let changes=sad_pet::parameters::parse(&json!({"wt_a_position":0.1+t*0.8,"wt_d_position":0.9-t*0.7,
                        "wt_a_fm_index":0.2+t*3.7,"wt_c_fm_index":3.9-t*3.7,"wt_a_sync_ratio":1.1+t*6.5,"wt_d_voice_shift":0.15-t*0.3})).unwrap();
                            e.apply_parameters(&changes);
                        }
                        if at == 1301 {
                            note(&mut e, 0, false);
                        }
                        if at == 2583 {
                            note(&mut e, 0, true);
                        }
                        if at == 4097 {
                            let changes = sad_pet::parameters::parse(
                                &json!({"wt_b_unison":2,"wt_c_unison":8}),
                            )
                            .unwrap();
                            e.apply_parameters(&changes);
                        }
                        if at == 7001 {
                            let changes = sad_pet::parameters::parse(
                                &json!({"wt_a_fm_mode":2,"wt_b_sync":1}),
                            )
                            .unwrap();
                            e.apply_parameters(&changes);
                        }
                        if at == 10007 {
                            let changes = sad_pet::parameters::parse(
                                &json!({"wt_a_fm_mode":0,"wt_c_fm_mode":0,"wt_b_sync":0}),
                            )
                            .unwrap();
                            e.apply_parameters(&changes);
                        }
                        for v in e.process_frame() {
                            assert!(v.is_finite());
                            samples.extend(v.to_le_bytes());
                        }
                    }
                    let name = format!("{rate}-{mode}-{notes}.f32");
                    std::fs::write(folder.join(&name), samples).unwrap();
                    rows.push(json!({"file":name,"rate":rate,"mode":mode,"notes":notes,"waves":4,"initial_unison":4,"frames":12288}));
                }
            }
        }
        std::fs::write(
            folder.join("manifest.json"),
            serde_json::to_vec_pretty(&rows).unwrap(),
        )
        .unwrap();
        println!("Captured {} dynamic references", rows.len());
    } else {
        panic!("wavetable_opt cpu MODE NOTES UNISON [SECONDS BLOCK RATE] | capture DIRECTORY");
    }
}
