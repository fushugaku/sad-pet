//! Wavetable integration baselines and independent Fourier/CPU measurements.
use rustfft::{FftPlanner, num_complex::Complex64};
use sad_pet::{
    dsp::wavetable::Bank,
    wavetable::{
        FRAME_SIZE,
        assets::{Asset, Pool},
    },
};
use serde_json::json;
use std::{hint::black_box, time::Instant};

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("registry") => {
            serde_json::to_writer_pretty(std::io::stdout(), sad_pet::parameters::specs()).unwrap();
        }
        Some("spectral") => spectral(),
        Some("lookup") => lookup(),
        Some("rack") => rack(),
        Some("factory") => factory(),
        Some("content") => content(),
        Some("interactions") => interactions(),
        _ => eprintln!(
            "Usage: wavetable_bench registry|spectral|lookup|rack [seconds]|factory [wav]"
        ),
    }
}

fn content() {
    use sad_pet::{
        dsp::{Engine, SampleBuffer},
        midi::Message,
        timbres::Kind,
        wavetable::factory::FACTORY,
    };
    use std::sync::Arc;
    let sample = Arc::new(SampleBuffer::new("Unused".into(), 48000, vec![[0.0; 2]; 2]).unwrap());
    let mut output = Vec::new();
    let mut cues = Vec::new();
    for table in FACTORY.iter().skip(32).step_by(8) {
        let mut p = Kind::Wavetable.preset().unwrap();
        p.wavetable.as_mut().unwrap().oscillators[0].asset_id = table.id.into();
        p.wavetable.as_mut().unwrap().oscillators[0].controls.level = 0.8;
        p.envs[0].attack = 0.0;
        p.envs[0].decay = 0.0;
        p.envs[0].sustain = 1.0;
        p.params.adsr_attack = 0.0;
        p.params.adsr_decay = 0.0;
        p.params.adsr_sustain = 1.0;
        let mut e = Engine::new(48000, sample.clone(), p);
        e.set_midi_gate(true);
        e.set_playing(true);
        e.midi(Message::NoteOn {
            channel: 0,
            note: 60,
            velocity: 100,
        });
        let start = output.len();
        let mut peak = 0.0_f32;
        for i in 0..96_000 {
            if i % 32 == 0 {
                let changes =
                    sad_pet::parameters::parse(&json!({"wt_a_position":i as f32/95999.0})).unwrap();
                e.apply_parameters(&changes);
            }
            let fade = (i as f32 / 480.0).min(1.0) * ((96000 - i) as f32 / 960.0).min(1.0);
            let frame = e.process_frame().map(|x| x * fade);
            assert!(frame.iter().all(|x| x.is_finite()));
            peak = peak.max(frame[0].abs()).max(frame[1].abs());
            output.push(frame);
        }
        cues.push(json!({"id":table.id,"name":table.name,"pack":table.pack_name,"start_seconds":start as f64/48000.0,"end_seconds":output.len() as f64/48000.0,"peak":peak}));
        output.extend([[0.0; 2]; 4800]);
    }
    if let Some(path) = std::env::args().nth(2) {
        std::fs::write(path, sad_pet::wav::bytes(48000, &output).unwrap()).unwrap();
    }
    println!("{}",serde_json::to_string_pretty(&json!({"scope":"Eight original complex packs; one dry C4 voice, U=1, Position 0..1, shared production Engine, no effects", "sample_rate":48000,"cues":cues})).unwrap());
}

fn factory() {
    use sad_pet::{
        dsp::{Engine, SampleBuffer},
        midi::Message,
        wavetable::presets::INSTRUMENTS,
    };
    use std::{collections::BTreeSet, sync::Arc};
    let sample = Arc::new(SampleBuffer::new("Unused".into(), 48000, vec![[0.0; 2]; 2]).unwrap());
    let mut rows = Vec::new();
    let mut montage = Vec::new();
    let mut cues = Vec::new();
    let mut heard = BTreeSet::new();
    for instrument in INSTRUMENTS {
        for scenario in ["note", "chord", "macros_full_chord"] {
            let mut preset = instrument.preset();
            if scenario == "macros_full_chord" {
                for m in &mut preset.macros {
                    m.value = 1.0;
                }
            }
            let mut engine = Engine::new(48000, sample.clone(), preset);
            engine.set_midi_gate(true);
            engine.set_playing(true);
            let notes: &[u8] = if scenario == "note" {
                &[60]
            } else {
                &[48, 55, 60, 64]
            };
            for &note in notes {
                engine.midi(Message::NoteOn {
                    channel: 0,
                    note,
                    velocity: 100,
                });
            }
            let (mut peak, mut energy, mut finite) = (0.0_f32, 0.0_f64, true);
            let include = scenario == "note" && heard.insert(instrument.category);
            let start = montage.len();
            for i in 0..144_000 {
                let frame = engine.process_frame();
                for x in frame {
                    finite &= x.is_finite();
                    peak = peak.max(x.abs());
                    energy += x as f64 * x as f64;
                }
                if include && i < 72_000 {
                    let fade =
                        (i as f32 / 768.0).min(1.0) * ((72_000 - i) as f32 / 3072.0).min(1.0);
                    montage.push(frame.map(|x| x * fade));
                }
            }
            if include {
                cues.push(json!({"name":instrument.name,"category":instrument.category,"start_seconds":start as f64/48000.0,"end_seconds":montage.len() as f64/48000.0}));
            }
            rows.push(json!({"id":instrument.id(),"name":instrument.name,"category":instrument.category,"scenario":scenario,
                "finite":finite,"peak":peak,"peak_dbfs":db(peak as f64),"rms_dbfs":db((energy/(144000.0*2.0)).sqrt()),
                "passed":finite&&peak>0.001&&peak<0.98}));
        }
    }
    if let Some(path) = std::env::args().nth(2) {
        std::fs::write(path, sad_pet::wav::bytes(48000, &montage).unwrap()).unwrap();
    }
    println!("{}",serde_json::to_string_pretty(&json!({"scope":"Original factory instruments, shared Engine / dry Rack, 3 s held notes at velocity 100; four macros at 1 in third scenario",
        "sample_rate":48000,"instruments":INSTRUMENTS.len(),"cases":rows.len(),"passed":rows.iter().all(|r|r["passed"]==true),"cues":cues,"results":rows})).unwrap());
}

fn rack() {
    use sad_pet::{
        dsp::{Engine, SampleBuffer},
        midi::Message,
        rack::Module,
        timbres::Kind,
    };
    use std::sync::Arc;
    let seconds: f64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(60.0);
    let blocks = ((seconds * 48000.0 / 128.0) as usize).max(100);
    let sample = Arc::new(SampleBuffer::new("Unused".into(), 48000, vec![[0.0; 2]; 2]).unwrap());
    let mut profiles = Vec::new();
    let quantiles = |mut times: Vec<f64>| {
        times.sort_by(f64::total_cmp);
        json!({"p50_ms":times[times.len()/2]*1000.0,"p99_ms":times[times.len()*99/100]*1000.0,
            "max_ms":times[times.len()-1]*1000.0,"p99_deadline_fraction":times[times.len()*99/100]/(128.0/48000.0)})
    };
    let mode = std::env::args().nth(3).unwrap_or_default();
    let profiles_to_run = if mode == "fm" || mode == "sync" {
        vec![(1, 4, 1), (4, 4, 1), (8, 4, 1), (8, 4, 2)]
    } else {
        vec![
            (32, 2, 1),
            (16, 2, 4),
            (8, 2, 2),
            (32, 2, 8),
            (32, 4, 1),
            (16, 4, 4),
            (8, 4, 2),
            (32, 4, 8),
        ]
    };
    for (notes, oscillators, unison) in profiles_to_run {
        let mut preset = Kind::Wavetable.preset().unwrap();
        preset.wavetable.as_mut().unwrap().count = oscillators;
        for oscillator in &mut preset.wavetable.as_mut().unwrap().oscillators {
            oscillator.controls.enabled = true;
            oscillator.controls.unison = unison;
            oscillator.controls.position = 0.357;
            oscillator.controls.phase_random = 1.0;
        }
        if mode == "fm" || mode == "sync" {
            let wt = preset.wavetable.as_mut().unwrap();
            for i in [0, 2] {
                wt.oscillators[i].controls.fm_mode = if mode == "fm" { 1 } else { 0 };
                wt.oscillators[i].controls.fm_index = 2.0;
                wt.oscillators[i].controls.sync = mode == "sync";
                wt.oscillators[i].controls.sync_ratio = 3.25;
            }
        }
        let mut baseline = preset.clone();
        baseline
            .rack
            .as_mut()
            .unwrap()
            .modules
            .retain(|s| s.module != Module::Wavetable);
        baseline.sanitize();
        let mut engine = Engine::new(48000, sample.clone(), preset);
        let mut neutral = Engine::new(48000, sample.clone(), baseline);
        for e in [&mut engine, &mut neutral] {
            e.set_midi_gate(true);
            e.set_playing(true);
            for i in 0..notes {
                e.midi(Message::NoteOn {
                    channel: 0,
                    note: 24 + i * 2,
                    velocity: 100,
                });
            }
        }
        let (mut full, mut base, mut delta) = (
            Vec::with_capacity(blocks),
            Vec::with_capacity(blocks),
            Vec::with_capacity(blocks),
        );
        let mut peak = 0.0_f32;
        for block in 0..blocks + 1875 {
            let start = Instant::now();
            for _ in 0..128 {
                for x in black_box(neutral.process_frame()) {
                    black_box(x);
                }
            }
            let baseline = start.elapsed().as_secs_f64();
            let start = Instant::now();
            for _ in 0..128 {
                for x in black_box(engine.process_frame()) {
                    assert!(x.is_finite());
                    peak = peak.max(x.abs());
                }
            }
            let elapsed = start.elapsed().as_secs_f64();
            if block >= 1875 {
                base.push(baseline);
                full.push(elapsed);
                delta.push((elapsed - baseline).max(0.0));
            }
        }
        profiles.push(json!({"notes":notes,"unison":unison,"oscillators":oscillators,"audio_seconds":blocks as f64*128.0/48000.0,
            "warmup_audio_seconds":5,"full_engine_rack":quantiles(full),"matched_neutral_rack":quantiles(base),"paired_added_wt_path":quantiles(delta),"peak":peak}));
    }
    let command = |exe: &str, args: &[&str]| {
        std::process::Command::new(exe)
            .args(args)
            .output()
            .ok()
            .map(|r| String::from_utf8_lossy(&r.stdout).trim().to_string())
    };
    println!("{}",serde_json::to_string_pretty(&json!({"scope":"Full shared Engine / neutral Rack comparison, in-process native CPU measurement; excludes DAW scheduling and device underruns",
        "cpu":command("sysctl",&["-n","machdep.cpu.brand_string"]),"rust":command("rustc",&["--version"]),"os":std::env::consts::OS,
        "sample_rate":48000,"block_frames":128,"mode":mode,"profiles":profiles})).unwrap());
}

fn db(value: f64) -> f64 {
    20.0 * value.max(1e-15).log10()
}

fn spectral() {
    const SAMPLES: usize = 131_072;
    let mut rows = Vec::new();
    let mut planner = FftPlanner::<f64>::new();
    let inverse_cycle = planner.plan_fft_inverse(FRAME_SIZE);
    let cycle_fft = planner.plan_fft_forward(FRAME_SIZE);
    let inverse_reference = planner.plan_fft_inverse(SAMPLES);
    let fft = planner.plan_fft_forward(SAMPLES);
    for sample_rate in [44100.0, 48000.0, 88200.0, 96000.0, 176400.0, 192000.0] {
        for requested in [16.35_f64, 27.5, 55.0, 110.0, 440.0, 1760.0, 7040.0] {
            let mut bin = (requested * SAMPLES as f64 / sample_rate).round() as usize;
            if bin.is_multiple_of(2) {
                bin += 1;
            }
            let hz = bin as f64 * sample_rate / SAMPLES as f64;
            // Independent Fourier signals share only the specified safe mip weights.
            let temporary =
                Bank::resolve(sad_pet::wavetable::DEFAULT_ASSET, &Pool::default()).unwrap();
            let weights = temporary.harmonic_weights(temporary.selection(hz, sample_rate));
            for kind in [
                "saw",
                "square",
                "triangle",
                "random_phase",
                "highest_partial",
            ] {
                let mut cycle = vec![Complex64::default(); FRAME_SIZE];
                for h in 1..FRAME_SIZE / 2 {
                    if kind == "highest_partial" && h != weights[0].0 {
                        continue;
                    }
                    if ["square", "triangle"].contains(&kind) && h.is_multiple_of(2) {
                        continue;
                    }
                    let value = match kind {
                        "highest_partial" => Complex64::new(0.0, -0.4),
                        "triangle" => Complex64::new(
                            0.0,
                            -0.5 * if h % 4 == 1 { 1.0 } else { -1.0 } / (h * h) as f64,
                        ),
                        "random_phase" => Complex64::from_polar(
                            0.4 / (h as f64).sqrt(),
                            (h as f64 * 2.399963229728653).rem_euclid(std::f64::consts::TAU),
                        ),
                        _ => Complex64::new(0.0, -0.5 / h as f64),
                    };
                    cycle[h] = value;
                    cycle[FRAME_SIZE - h] = value.conj();
                }
                inverse_cycle.process(&mut cycle);
                let samples: Vec<_> = cycle.iter().map(|x| x.re as f32).collect();
                let asset = Asset::from_cycles(kind.into(), FRAME_SIZE, &samples).unwrap();
                let bank = Bank::compile(&asset).unwrap();
                let selection = bank.selection(hz, sample_rate);
                let mut canonical: Vec<_> = asset
                    .samples
                    .iter()
                    .map(|&x| Complex64::new(x as f64, 0.0))
                    .collect();
                cycle_fft.process(&mut canonical);
                for phase in [0.0, 0.123456789, 0.49991] {
                    let mut reference = vec![Complex64::default(); SAMPLES];
                    let mut model = vec![false; SAMPLES];
                    for (h, coefficient) in
                        canonical.iter().enumerate().take(FRAME_SIZE / 2).skip(1)
                    {
                        let gain = weights
                            .iter()
                            .filter(|(limit, _)| h <= *limit)
                            .map(|(_, gain)| gain)
                            .sum::<f64>();
                        if gain == 0.0 {
                            continue;
                        }
                        let target = h * bin;
                        assert!(target < SAMPLES / 2);
                        let value = coefficient / FRAME_SIZE as f64
                            * gain
                            * Complex64::from_polar(1.0, std::f64::consts::TAU * phase * h as f64);
                        reference[target] = value;
                        reference[SAMPLES - target] = value.conj();
                        model[target] = true;
                        model[SAMPLES - target] = true;
                    }
                    inverse_reference.process(&mut reference);
                    let actual: Vec<_> = (0..SAMPLES)
                        .map(|i| {
                            bank.read(
                                (i as f64 * bin as f64 / SAMPLES as f64 + phase).fract(),
                                0.0,
                                selection,
                            ) as f64
                        })
                        .collect();
                    let rms = (reference.iter().map(|x| x.re * x.re).sum::<f64>() / SAMPLES as f64)
                        .sqrt()
                        .max(1e-6);
                    let error = (actual
                        .iter()
                        .zip(&reference)
                        .map(|(a, b)| (a - b.re).powi(2))
                        .sum::<f64>()
                        / SAMPLES as f64)
                        .sqrt();
                    let mut frequency: Vec<_> =
                        actual.iter().map(|&x| Complex64::new(x, 0.0)).collect();
                    fft.process(&mut frequency);
                    let spurious = frequency
                        .iter()
                        .zip(&model)
                        .filter(|(_, expected)| !**expected)
                        .map(|(v, _)| v.norm_sqr())
                        .sum::<f64>()
                        .sqrt()
                        / SAMPLES as f64;
                    rows.push(json!({"sample_rate":sample_rate,"kind":kind,"requested_hz":requested,"actual_hz":hz,"phase":phase,"spurious_dbr":db(spurious/rms),"total_error_dbr":db(error/rms),"harmonic_weights":weights,"bank_bytes":bank.bytes()}));
                }
            }
        }
    }
    let worst_spurious = rows
        .iter()
        .map(|r| r["spurious_dbr"].as_f64().unwrap())
        .fold(f64::NEG_INFINITY, f64::max);
    let worst_error = rows
        .iter()
        .map(|r| r["total_error_dbr"].as_f64().unwrap())
        .fold(f64::NEG_INFINITY, f64::max);
    let passed = worst_spurious <= -70.0 && worst_error <= -60.0;
    println!("{}", serde_json::to_string_pretty(&json!({"scope":"Production f32 bank and polyphase sinc16; independent f64 Fourier oracle; static signal only","sample_rates":[44100,48000,88200,96000,176400,192000],"samples":SAMPLES,"cases":rows.len(),"worst_spurious_dbr":worst_spurious,"worst_error_dbr":worst_error,"passed":passed,"rows":rows})).unwrap());
    if !passed {
        std::process::exit(1);
    }
}

fn lookup() {
    let pool = Pool::default();
    let banks = [
        Bank::resolve(sad_pet::wavetable::DEFAULT_ASSET, &pool).unwrap(),
        Bank::resolve("factory:wt-vowel-sweep:v1", &pool).unwrap(),
    ];
    let seconds: f64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2.0);
    let blocks = ((seconds * 48_000.0 / 128.0) as usize).max(100);
    let mut profiles = Vec::new();
    for (notes, unison) in [(32, 1), (16, 4), (8, 2), (32, 8)] {
        let count = notes * unison * 2;
        let hz: Vec<_> = (0..count)
            .map(|i| {
                55.0 * 2.0_f64.powf(
                    ((i / (2 * unison)) % 24) as f64 / 12.0 + (i % unison) as f64 * 0.07 / 12.0,
                )
            })
            .collect();
        let selections: Vec<_> = hz
            .iter()
            .enumerate()
            .map(|(i, f)| banks[i % 2].selection(*f, 48_000.0))
            .collect();
        let mut phases = vec![0.1234_f64; count];
        let mut times = Vec::with_capacity(blocks);
        let mut output = 0.0;
        for block in 0..blocks + 375 {
            let start = Instant::now();
            for _ in 0..128 {
                for i in 0..count {
                    output += banks[i % 2].read(phases[i], 0.357, selections[i]);
                    phases[i] += hz[i] / 48_000.0;
                    if phases[i] >= 1.0 {
                        phases[i] -= 1.0;
                    }
                }
            }
            black_box(output);
            if block >= 375 {
                times.push(start.elapsed().as_secs_f64());
            }
        }
        times.sort_by(f64::total_cmp);
        profiles.push(json!({"notes":notes,"oscillators":2,"unison":unison,"seconds":blocks as f64*128.0/48000.0,"p50_ms":times[times.len()/2]*1000.0,"p99_ms":times[times.len()*99/100]*1000.0,"max_ms":times[times.len()-1]*1000.0,"p99_deadline_fraction":times[times.len()*99/100]/(128.0/48000.0)}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"scope":"Isolated production lookup, not full Rack/runtime or browser deadline","sample_rate":48000,"block_frames":128,"profiles":profiles})).unwrap());
}

fn interactions() {
    use sad_pet::{dsp::wavetable::Runtime, wavetable::Settings};
    let routed = std::env::args().nth(2).as_deref() == Some("routed");
    let mut pool = Pool::default();
    let mut ids = Vec::new();
    for kind in ["sine", "saw", "complex"] {
        let samples = (0..FRAME_SIZE)
            .map(|i| {
                let p = i as f64 / FRAME_SIZE as f64 * std::f64::consts::TAU;
                (if kind == "sine" {
                    p.sin()
                } else if kind == "saw" {
                    (1..=127)
                        .map(|h| (p * h as f64).sin() / h as f64)
                        .sum::<f64>()
                        * 0.4
                } else {
                    (1..=63)
                        .map(|h| {
                            ((p * h as f64) + (h * h) as f64 * 0.123).sin() / (h as f64).powf(0.7)
                        })
                        .sum::<f64>()
                        * 0.18
                }) as f32
            })
            .collect::<Vec<_>>();
        let asset = Asset::from_cycles(kind.into(), FRAME_SIZE, &samples).unwrap();
        ids.push(pool.insert(asset).unwrap().id.clone());
    }
    let mut planner = FftPlanner::<f64>::new();
    let mut rows = Vec::new();
    for rate in [44100_u32, 48000, 96000] {
        let fft = planner.plan_fft_forward(rate as usize);
        for note in [45.0_f32, 69.0, 93.0, 105.0, 117.0, 129.0] {
            let root = (440.0 * 2.0_f64.powf((note as f64 - 69.0) / 12.0)).round() as usize;
            for (mode, index, ratio, kind, source) in [
                (1, 0.5, 1.0, 0, 0),
                (1, 2.0, 1.0, 0, 0),
                (1, 4.0, 1.0, 1, 2),
                (2, 0.5, 1.0, 0, 0),
                (2, 2.0, 1.0, 0, 0),
                (2, 4.0, 1.0, 2, 1),
                (0, 0.0, 2.75, 0, 0),
                (0, 0.0, 7.5, 1, 0),
                (0, 0.0, 3.25, 2, 0),
            ] {
                let mut s = Settings::default();
                let (carrier_id, source_id) = if routed { (1, 3) } else { (0, 1) };
                s.count = if routed { 4 } else { 2 };
                for o in &mut s.oscillators {
                    o.controls.enabled = false;
                }
                s.oscillators[carrier_id].asset_id = ids[kind].clone();
                s.oscillators[source_id].asset_id = ids[source].clone();
                let carrier = &mut s.oscillators[carrier_id].controls;
                carrier.enabled = true;
                carrier.fm_mode = mode;
                carrier.fm_index = index;
                carrier.fm_source = source_id as u8;
                carrier.sync = ratio > 1.0;
                carrier.sync_ratio = ratio;
                carrier.sync_source = if routed { source_id } else { carrier_id } as u8;
                s.oscillators[source_id].controls.enabled = true;
                s.oscillators[source_id].controls.level = 0.0;
                s.oscillators[source_id].controls.transpose = 12.0;
                let mut runtime = Runtime::new(rate, &s, &pool).unwrap();
                runtime.configure(s.controls(), 0.0);
                let mut samples = Vec::with_capacity(rate as usize);
                for i in 0..rate as usize * 3 / 2 {
                    runtime.begin_frame(true);
                    let frame = runtime.render(1, 1, note, &[0.0; 8]);
                    assert!(frame.iter().all(|x| x.is_finite()));
                    if i >= rate as usize / 2 {
                        samples.push(Complex64::new(frame[0] as f64, 0.0));
                    }
                }
                let rms = (samples.iter().map(|x| x.norm_sqr()).sum::<f64>()
                    / samples.len() as f64)
                    .sqrt();
                fft.process(&mut samples);
                let grid = if routed && ratio > 1.0 {
                    root * 2
                } else {
                    root
                };
                let mut allowed = vec![false; samples.len()];
                allowed[0] = true;
                for h in 1..=rate as usize / 2 / grid {
                    let bin = h * grid;
                    for delta in -1_i64..=1 {
                        let b = (bin as i64 + delta) as usize;
                        allowed[b] = true;
                        allowed[samples.len() - b] = true;
                    }
                }
                let spurious = (samples
                    .iter()
                    .zip(&allowed)
                    .filter(|(_, a)| !**a)
                    .map(|(x, _)| x.norm_sqr())
                    .sum::<f64>())
                .sqrt()
                    / samples.len() as f64;
                // An external clock above Nyquist has no expected in-band
                // partial. Its residual must meet an absolute stopband gate;
                // dividing that tiny residual by itself is not an alias metric.
                let inband = grid < rate as usize / 2;
                let passed = if inband {
                    db(spurious / rms.max(1e-10)) <= -60.0
                } else {
                    db(rms) <= -90.0
                };
                rows.push(json!({"sample_rate":rate,"note":note,"root_hz":root,"expected_grid_hz":grid,"inband_fundamental":inband,"mode":mode,"index":index,"sync_ratio":ratio,"carrier":(["sine","saw","complex"][kind]),"source":(["sine","saw","complex"][source]),"rms":rms,"rms_dbfs":db(rms),"spurious_dbfs":db(spurious),"spurious_dbr":db(spurious/rms.max(1e-10)),"passed":passed}));
            }
        }
    }
    let worst = rows
        .iter()
        .filter(|r| r["inband_fundamental"] == true)
        .map(|r| r["spurious_dbr"].as_f64().unwrap())
        .fold(f64::NEG_INFINITY, f64::max);
    let passed = rows.iter().all(|r| r["passed"] == true);
    println!("{}",serde_json::to_string_pretty(&json!({"scope":"Production Runtime, FM/PM/sync static integer-ratio operator notes; nonharmonic FFT energy against the expected harmonic grid. External sync above Nyquist uses absolute RMS stopband rejection. Not a dynamic modulation oracle; feedback is excluded.","routing":if routed {"D→B, external D clock"} else {"legacy B→A, self clock"},"fm_oversample":8,"sync_oversample":32,"fir_taps":[63,641],"cases":rows.len(),"criterion_spurious_dbr":-60,"criterion_stopband_rms_dbfs":-90,"worst_inband_spurious_dbr":worst,"passed":passed,"rows":rows})).unwrap());
    if !passed {
        std::process::exit(1);
    }
}
