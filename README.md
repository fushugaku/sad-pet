# SAD PET

A granular instrument with a shared Rust audio engine for desktop and WebAssembly.

**[Play SAD PET in your browser](https://fushugaku.github.io/sad-pet/)**

The web version runs entirely in the browser: Rust DSP in an AudioWorklet, with
sample preparation, WAV rendering and MP3 encoding off the audio thread. Dedicated
workers handle binary UI telemetry and recording ingestion. Audio views and
recording buffers are reused. The UI receives preset changes and meter deltas,
and only visible, changed graphs are drawn. Large FX histories are prepared for
the selected effects; Source FX upload buffers are reserved on demand. Adding a
web timbre spreads preparation across audio blocks while existing parts continue
playing; drum history initialization is limited to 256 KiB per step. Voices
share immutable filter tables at the same sample rate, and
unchanged gain controls reuse their exact conversion. A lightweight telemetry
decoder keeps only preset/view metadata, without creating DSP engines for its
timbres. Sample rates, antialiasing, interpolation and effect algorithms are
unchanged. No backend
or installation is required. Click Play or a keyboard key to enable sound.
The standalone **SAD PET app** for macOS and Windows runs the native Rust engine
on the computer's audio and MIDI devices in its own window; its editor never
handles sample data. See [plugin/APP.md](plugin/APP.md).
The desktop version uses the system audio output and a local browser panel.
Its DSP publishes into three reusable snapshot buffers. A separate thread builds
UI state, retires replaced presets and acknowledges edits in order; a busy panel
does not block rendering or discard unpublished peaks. MIDI notes keep their
direct route to the DSP worker.

New browser sessions and DAW instances start with **glasses5**: its Dusty keys
source, selection/fades, four-oscillator Resynth, sequencer and both FX racks.
It is also the first Factory choice; Init remains available. Saved DAW projects
and remembered browser User presets take priority over this initial sound.

New browser sessions and DAW instances start with **glasses5**: its Dusty keys
source, selection/fades, four-oscillator Resynth, sequencer and both FX racks.
It is also the first Factory choice; Init remains available. Saved DAW projects
and remembered browser User presets take priority over this initial sound.

Each effect has its own named preset selector and Save button. Effect presets
are shared across the three FX racks and all timbres of the same effect type.
Portable `.sadfx` import/export moves settings between the web and native
plug-ins, including modes, tempo sync, Smooth and Mix.

- Thirty built-in audio sources, WAV import and an IndexedDB file library, including
  Matrix POV video audio and a 42.67-second Harvest Dawn harp/chamber-string cover
  loop: 16 bars at 90 BPM, with the recognizable eight-bar introduction played
  in two variations. Note pitches/rhythm are checked against published transcriptions.
  Note releases and the room decay continue across the loop boundary.
  Composition credit remains Jeremy Soule (Oblivion, 2006).
  It is a new recording of a copyrighted composition, not royalty-free music.
- Twenty additional original synthesized sounds and loops: lo-fi, breakbeat,
  house drums, electro percussion, dub chords, acid bass, marimba, kalimba,
  nylon harmonics, music box, tape piano, strings, shimmer pad, drone, singing
  bowl, bells, glitches, radio transmissions, ocean surf and vinyl texture.
  Rhythmic additions are four-bar loops with BPM; tails and echoes wrap at the seam.
- Browser microphone input with device selection, gain and level metering:
  direct filters/effects or granular processing of an eight-second live buffer.
- Polyphonic keyboard, note/chord hold and arpeggiator.
- **Neuropolis sequencer**, using its actual Rust core: add any number of independently
  clocked 16-stage tracks. Route a track to main sample pitch, OSC 1–4 pitch offsets, notes
  and envelopes, or a realtime parameter (including oscillator controls and FX).
  Each track has 21 play orders, length, clock multiplier/divider, swing, quantizer,
  pulses, Rest/Single/Multi/Hold gates, ratchets, probability, Analog/Tempo/Acid
  slide, CV ramps and pitch accumulation. Tracks and assignments save with presets
  and DAW projects; old chord patterns migrate into note tracks. Track count has
  no fixed cap; the instrument retains its eight-note audio polyphony.
  Pitch, a per-step Gate length knob, Pulses, Ratchet, Slide, Probability and
  Velocity are available together. Main pitch can also target any combination
  of OSC 1–4; their own tracks add −24…+24 semitones to that pitch, at independent
  clocks. With Gate ENV, Main pitch steps also restart and release the envelopes
  like notes. Double-click a step's Gate knob to inherit the track's Gate setting.
  Select steps on the bar above each stage (drag for a run, Shift to extend,
  ⌘/Ctrl to add or remove, Esc to clear), then **Copy** and **Paste** (⌘/Ctrl+C,
  ⌘/Ctrl+V) within a track or into another one. A single selected step is where
  the copy starts; a longer selection is filled, repeating the copy, and every
  selected region gets it. Pasting between note and OSC lanes keeps the intervals.
- In the browser and DAW plug-ins, add independent **Granular**, **Drum machine**
  and **Synth engine** timbres as tabs across the top. They run together, with per-timbre level, mute,
  solo and MIDI channel. Each owns its source, effects, Resynth, envelopes, LFOs
  and Neuropolis program. Save/recall includes the entire scene and all sources.
- Up to **8 LFOs** with tempo sync up to 8 bars and **8 ADSR envelopes** per timbre,
  plus eight modulation-matrix routes. OSC 1–4 shape, mode, tuning, level, pan,
  Ring/Sync and other realtime numeric controls are available as destinations.
  Existing three-modulator patches retain their sound and routing.
  Each envelope has a **Type**: SAD PET (the original ADSR, default) or a Rustias
  EG type — RADIAS (ADSR with a selectable decay/release curve), Q ADSR, Q ADS1DS2R
  (Attack level, Decay 2, Sustain 2), Q One Shot, Q Loop S1S2 and Q Loop All.
  Older presets keep the original envelope.
  **− LFO** / **− ENV** removes the last added modulator while keeping the first
  three and the remaining assignments intact.
- **DXM-088 / LXR-02 drum timbres**, using the original Rust synthesis engine:
  three drum voices, snare, clap/cymbal and shared closed/open hi-hat. All 56 voice
  controls are available, including FM, transients, envelopes and both filters.
  Each drum track has 1–128 steps, its own clock, swing/groove, MIDI note, level,
  mute and solo. Steps expose velocity, chance, pitch, ratchet, microtiming,
  conditions and two parameter locks. The drum page has its own Neuropolis mod
  sequencer. **Granular** routes the drum mix through the shared Grain Engine;
  turn it off for direct processing through the remaining filter/FX chain.
  Every engine keeps the Granular page order: its own section on top, then the
  waveform, the transport row with the **Granular** switch beside Dry/Wet, Source
  FX and Grain Engine with a live grain count. **Buffer** selects 125 ms–32 s of live drum history (8 s by default).
  The waveform, Position and Cloud use that duration; Size scales its full range
  to shorter buffers. Changing duration retains captured audio, including Freeze,
  and uses reserved memory without restarting the pattern or rebuilding DSP.
  The duration saves separately per timbre in presets and DAW projects, in both
  the browser and native plug-ins.
  Selecting a sequencer track or step also selects that drum's sound;
  voice knobs repaint immediately while dragging.
  The firmware clock, original stereo DSP and host-rate resampler are preserved.
- **Synth engine timbres** run the granular engine in reverse order: up to four
  Rustias OSC1 / Wavefolder oscillators per note (Saw, Pulse, Triangle, Sine in
  Wave/Cross/Unison/VPM, Formant, Noise and PCM of the loaded file) play the eight
  note voices first, with Ring/Sync and Cross links between oscillators. Each note
  then passes its own filters, ENV/VCA, LFOs, matrix and Pre-filter FX, from MIDI,
  arpeggiator or Neuropolis tracks. **Granular** sends the mixed voices after the
  VCA into a 125 ms–32 s live history read by the Grain Engine, so grains keep the
  tails of released notes; Pitch then transposes the grains instead of the
  oscillators. Oscillator controls are host parameters and modulation
  destinations (`synth_osc1_waveform`…, `synth_granular`, `synth_buffer_seconds`).
  The desktop app, which has one engine, switches it with **Engine: Granular /
  Synth engine**. Presets of the other timbres serialize unchanged.
- **Bus and Master** (browser and DAW plug-ins). Every scene has one **Master**
  tab that sums all timbres and buses into the output; add up to eight **Bus**
  timbres with **+ Timbre**. Each timbre's mixer row sets its level into Master
  and a send level to every bus. A bus or Master runs the granular engine on its
  input, with switchable **Granular** (125 ms–8 s history) and **Filters**, plus
  Resynth, all effect racks, added filters, LFOs and the matrix. Everything starts
  switched off, and then the input passes through bit for bit at almost no CPU:
  no grains, no history writes and no filters run. Solo selects timbres while
  buses and Master keep processing them; MIDI never reaches a bus. Scenes saved
  before buses existed open with a default Master and render unchanged.
- **Vocal timbres** sing a score, in the manner of VOCALOID5 (browser, DAW
  plug-ins and app). Draw notes in the piano roll, double-click one to type its
  lyric (Tab moves on) or use **Lyrics…** for a whole line, in **Japanese**
  (kana or romaji), **English** (dictionary plus letter-to-sound rules) or
  **Russian** (palatalization, voicing and stress marks such as `при+вет`).
  `-` holds the previous vowel, `+` or a trailing hyphen (`hap-`, `py`) spreads
  one word over several notes, and each note can take explicit phonemes. The
  twelve VOCALOID5 lanes — Dynamics, Breathiness, Brightness, Clearness,
  Character, Exciter, Growl, Air, Mouth, Portamento timing, Pitch bend and its
  sensitivity — draw with the mouse; notes have VEL, vibrato and Attack/Release
  effects (scoop, bend, soft, accent, breathy, turn; fall, rise, soft, breathy,
  swell), and **Styles** (Natural, Pop, Ballad, Rock, Whisper, Opera, Robot) set
  how the pitch moves and the voice color. Three voices come with SAD PET —
  Kasane Teto (Japanese and English) and Kumi (Russian) — and a new Vocal timbre
  sings with the one for its language; **Steps** (or the marker on the ruler)
  makes the score start over after that many grid steps; the loop's start
  marker leaves an intro before it that is sung once. Under each note the
  editor shows the voicebank units that sing it, and the note inspector's
  **Unit** picks another one; **Edit ▾** copies, cuts, pastes and duplicates
  notes. The synthetic formant
  voice sings without recordings; **Voices ▾** imports a UTAU voicebank (a folder,
  a `.zip` or a `.sadvoice` file): its `oto.ini` units are analyzed into
  spectral frames, then selected and joined in the spectral domain on the sung
  pitch. **Import score…** reads Standard MIDI files with lyric events, UTAU
  `.ust`, MusicXML/`.mxl` and VOCALOID5 `.vpr`. Phrases render off the audio
  thread and only when they change; the timbre then runs like a Bus input, so
  Granular, Filters, Resynth and every effect apply to the voice. Plug-ins play
  the score at the DAW position and tempo; the browser and app play it with
  **▶ Play** or the main transport. See [the voice engine](voice/README.md).
- **Sampler timbres** (browser) play kits of samples in the manner of an MPC: eight
  banks of sixteen pads, each with **zones** for velocity layers (by velocity, cycle
  or random) and key groups. Every zone has Start/End, a loop (forward, back and
  forth, or while held) with **crossfade**, Reverse, **fade in / fade out** with
  curves, root note and tuning; every pad adds One shot / Note on, Mono, **mute
  groups**, keytracking, an AHDSR amp envelope, a filter (low/high/band pass, notch)
  with its own envelope, a pitch envelope and velocity to level, start and attack.
  The **Sample editor** shows the waveform with draggable markers and zero-crossing
  snap: **Trim**, **Loop**, **Chop** (transients, equal regions or beats, onto pads,
  onto keys of one pad, or into new samples) and **Process**: crop, delete, silence,
  insert silence, normalize, gain, reverse, fades, baked loop crossfade, DC removal,
  phase invert, mono, swap, pitch shift (varispeed or keeping the length), time
  stretch, resample, bit crush, trim silence, copy/paste/mix paste and undo. Each
  edit makes a new sample; the original stays. **Detect root** finds a sample's pitch.
  **Record** takes a sample from any input or resamples SAD PET's own output, starting
  at a level if you like. Every pad can get its own **Pad FX** timbre — Granular,
  Filters, Resynth and both effect racks, edited like any timbre — and **Bounce**
  renders each note of a range through it into plain samples (key groups) that the pad
  then plays, freeing the chain's CPU; **Unbounce** goes back. **Sequence this pad**
  adds a Neuropolis track that plays one pad whatever its keys; note tracks play pads
  by key. Samples, takes, slices, edits and bounces live in the browser's IndexedDB;
  **packs** keep a kit with its samples and move between browsers as `.zip` files
  (`pack.json` and WAVs). Imports take WAV and whatever the browser decodes (MP3,
  AAC, OGG, FLAC), files dropped on pads, and `.zip` packs or plain zips of samples.
  Scenes and presets carry the kit's samples. The DAW plug-ins and app play Sampler
  timbres of loaded scenes silently for now.
- Two filters with single, serial and parallel routing, envelope depth and key tracking.
- All **18 Rustias filter models** in either slot: RADIAS, Ladder LP/HP/BP24,
  MS-20 LP/HP12, Pro LP24, SEM, Multi Filter, M/Poly LP24 and Linear LP/HP/BP/Notch
  at 12/24 dB. Original RADIAS Filter 2 LP/BP/HP and Comb are also available.
  Drive and model Morph support LFO/ENV/matrix, sequencer and DAW automation.
  RADIAS Morph sweeps LP24 → LP12 → BP → HP → Thru; SEM and Multi have their
  own response morphs. Existing SAD PET filters preserve old preset sounds.
- A **Source FX** slot before the Grain Engine, with all existing effect types.
  Live audio is processed before grain capture; file selections are prepared in
  the background with the original FIR antialiasing and smooth source changes.
- **Resynth**: 16 analysis bands drive up to four independent Rustias oscillator
  layers, with Saw, Pulse, Triangle, Sine, Formant, Noise, Audio In and sample-based
  PCM. Each slot can also select Rustias OSC 4's Wavefolder, including Triangle,
  Sine, Soft, Asymmetric and Rectify shapers with depth/bias controls. Analog
  RADIAS waves support Wave, Cross, Unison and VPM. Ring, Sync and Ring+Sync
  independently select Source, Grains or another oscillator, before level/pan.
  Cross also has separate Source/Grains/oscillator routing. Each layer has its own
  controls, tuning, level and pan; the shared detector has sensitivity, attack,
  release and threshold. Choose Source or the Grain Engine output as excitation.
- Two independent reorderable chains of up to 12 effects each, before and after
  the filters: distortion, rate reducer, bit crusher,
  flanger, chorus, binaural processing, reverb, L/C/R delay, modulated delay and
  a stereo-linked compressor (threshold, ratio, attack, release, knee, makeup and mix).
  Delay **Smooth** allows intermediate note divisions and glides the echo time
  under manual, LFO and envelope control; the readout shows both divisions and milliseconds.
  All compressor controls accept LFO, envelope and matrix modulation.
  Each compressor has a live gain reduction meter with a dB readout.
- RADIAS Waveshaper in either chain: Drive, Hard Clip, Decimator, OctSaw,
  MultiTri, MultiSin, SubSaw, SubSquare, SubTriangle, SubSine, Pickup and LvlBoost.
  Original fixed-point kernels and Depth transfer, plus per-slot Mix and bypass.
  Depth accepts LFO, envelope, matrix and DAW automation; SubOSC/Pickup follow MIDI pitch.
- Live modulation feedback on controls, MIDI input and stereo WAV/MP3 recording.
- Dry/Wet blends the original source with grains before pre-filter FX, note
  filters and post-filter FX, so even fully dry audio receives the full chain.
- Waveform loop selection, adjustable fade in/out and sample-exact trimming to new library files.
- Optional **Loop 1 bar**: each MIDI note plays the selection in one bar while
  pitch, tuning and pitch bend remain independent of its duration. **Root** sets
  the original sample note. Plug-ins follow the DAW tempo and time signature.
- Factory/User preset browser with named saving, overwrite, Save as new and
  portable `.sadpet` import/export, including the source sample and region/fades.
  Plug-ins share an on-disk User bank; the browser keeps its bank in IndexedDB.

## Run

The packaged macOS application is `SAD PET.app`. Opening it starts the audio engine
and opens [the local panel](http://127.0.0.1:3214/). Quit in the panel stops the engine.

To build and run from source on macOS:

```sh
cargo run --release
```

Use a stable Rust toolchain and Xcode Command Line Tools. The bundled LAME encoder
is compiled from source; its build also needs Autoconf, Automake and Libtool.
The packaged application does not need Rust or a separate MP3 encoder installed.

## Develop

```sh
./dev.sh
```

The frontend runs on port 3214 and proxies the Rust API/audio engine on port 3215.
Frontend files load directly from `rust/web`; reload after editing them. Restart
after Rust changes. Ctrl+C stops both services and their child processes.
Close a packaged instance with Quit before starting the development stack.

```sh
./dev.sh --no-open
./dev.sh --frontend-port 3314 --backend-port 3315
```

## Build the web version

```sh
rustup target add wasm32-unknown-unknown
python3 wasm/build.py
node wasm/test.mjs
node wasm/test-realtime.mjs
node wasm/test-telemetry.mjs
node wasm/test-loader.mjs
node wasm/test-resynth.mjs
node wasm/test-neuropolis.mjs
node wasm/test-sequencer-ui.mjs
node wasm/test-microphone.mjs
node wasm/test-library.mjs
node wasm/test-presets.mjs
node wasm/test-parameters.mjs
python3 -m http.server 8080 --directory dist/web --bind 127.0.0.1
```

Open `http://127.0.0.1:8080/`. Deployment needs only the files in `dist/web` on an
HTTPS static host. Relative asset paths also work under a project subdirectory.
Release builds omit WASM debug names; pass `--keep-debug-names` to `wasm/build.py`
when profiling. Only metadata is removed.
The browser compiles WASM while downloading it when the host serves
`application/wasm`. Generic MIME types and older webviews use buffered loading.
The GitHub Actions workflow builds, checks and publishes the site on every push to
`main`; repository Settings → Pages uses **GitHub Actions** as its source.

`wasm/build.py` compiles the engine with the `wasm` Cargo profile (fat LTO,
abort on panic). Three optional engines are off by default. `--tail-call` also
builds `sad_pet.tc.wasm` with WebAssembly tail calls, which the loader prefers
where `WebAssembly.validate` accepts a `return_call` (Chrome 112, Firefox 121,
Safari 18.2): the translated firmware continues from block to block in tail
position instead of returning to its dispatch loop, identical samples, measured
about 2.5% faster on SuperEcho scenes once the hot flash pages were fused, not
enough for a second 20 MB download by default (`node wasm/test-tailcall.mjs`
checks the equivalence). The other two measured slower on the translated
SuperEcho firmware in V8: `--simd` also builds `sad_pet.simd.wasm` (+simd128;
`node wasm/test-simd.mjs` checks that it renders identical samples, about 5%
slower), and `--wasm-opt [LEVEL]` runs Binaryen's `wasm-opt` (float semantics
stay exact without `--fast-math`; `-O3` was 25-47% slower and takes about ten
minutes per binary).
The translated SuperEcho and AlphaVerb firmware is dispatched through jump
tables generated by `vendor/superecho/build.rs` and `vendor/alphaverb/build.rs`.
Their ITCM code and hottest flash routines are fused: a block whose only
predecessor branches forward to it is pasted into that predecessor, every other
forward branch is a direct call, blocks return the next program counter as a
plain integer and park faults in a cell instead of unwinding (a `Result` would
cross each indirect call through linear memory). Inside each fused function the
emulated registers are local variables, written back where the function exits:
LLVM cannot prove that firmware stores leave the register bytes alone, so with
the registers in memory it reloaded them after every store. The emulated memory
regions are fixed-size arrays, checked heap and DTCM first with one comparison
each. VFMA runs the hardware fused multiply-add on aarch64 (and x86 built with
`+fma`); wasm32 and baseline x86 use exact emulations instead of the software
`fmaf`/`fma` routines (float32 through float64 with rounding to odd, float64 by
Boldo and Melquiond's method). `cargo test -p sad-pet-superecho --release --lib`
and `cargo test -p sad-pet-alphaverb --release --lib` check the fused dispatch
against the original block dispatchers byte for byte and the emulated FMA against
the hardware instruction bit for bit.

Use a browser with WebAssembly and AudioWorklet support. Web MIDI is optional and
requires a supported browser and permission through the MIDI refresh button.

Audio files, presets, rendered WAVs and WAV/MP3 recordings stay in IndexedDB and
can be downloaded from Library. Storage belongs to each browser profile and site
origin; the hosted site and the local desktop panel have separate libraries.

For DSP changes, capture a reference before editing and compare with the new
build on the same platform/compiler. These offline benchmarks require exact
PCM equality, including FX histories, Resynth and drum scenes:

```sh
node wasm/benchmark.mjs capture target/perf/reference
node wasm/benchmark-scenes.mjs capture target/perf/scenes-reference
node wasm/benchmark-controls.mjs capture target/perf/controls-reference
node wasm/benchmark-alphaverb.mjs capture target/perf/alphaverb-reference
node wasm/benchmark-superecho.mjs capture target/perf/superecho-reference
# Rebuild after the change, then:
node wasm/benchmark.mjs compare target/perf/reference
node wasm/benchmark-scenes.mjs compare target/perf/scenes-reference
node wasm/benchmark-controls.mjs compare target/perf/controls-reference
node wasm/benchmark-alphaverb.mjs compare target/perf/alphaverb-reference
node wasm/benchmark-superecho.mjs compare target/perf/superecho-reference
```

The scenes, controls, AlphaVerb and SuperEcho compares accept a binary path as
their third argument, so an experimental engine (for example `--simd`) can be
checked against references captured from the default build.

The AlphaVerb benchmark covers one/two instances, parameter changes and tails at
44.1/48/96 kHz, checks exact PCM and reports audio-block render times. Its native
firmware-state comparison runs with:

```sh
cargo test -p sad-pet-alphaverb --release --lib --locked
```

The SuperEcho benchmark also covers Tape/BBD, two simultaneous instances and a
SuperEcho + AlphaVerb chain, including shimmer, dynamics, freeze and tempo sync.
Both imported effects retain the original firmware arithmetic and audio rate.

Use **Load WAV** or drop a mono/stereo WAV onto the waveform to import your own
audio. Drag across the waveform to choose a loop, adjust its **IN / OUT** handles,
or enter exact boundaries in seconds. The selection immediately limits playback,
including reverse grains and cloud spread. **Full** restores the whole file.
**Fade In / Fade Out** set smooth fades in milliseconds at the selection boundaries.
They apply during playback and on each loop, including grain reads. Their settings
are stored per source file in IndexedDB and restored when that file is loaded.
**Save edit** saves and loads a new WAV with the selection and fades baked in,
without changing the original;
download it from the saved result or Library. After recording, **Trim** opens the
saved take in this editor. Selecting a different source resets the loop.

Select **Microphone** to allow a live input, then choose **Filters + FX**
for immediate processing or **Grains + Filters + FX** for granular processing.
**Listen** enables microphone monitoring; it starts off, so connecting a microphone
does not immediately feed it to the speakers. **Record mic** starts a timed take;
choose **Microphone only** for input before filters/effects, or **With processing**
for the processed output. **Stop & save** stores WAV and MP3 in Library and shows
the saved take with **Use as sample** and download actions. Recording works with
Listen off. **Capture last 8 s** saves a short buffer snapshot instead of starting
a take. **Freeze** retains the granular input buffer. **File**, **Stop audio**,
loading a WAV or closing the page releases
the microphone. Capture permission is requested only when selecting Microphone;
audio and saved files stay in the browser. The packaged desktop edition continues
to use file input; live microphone capture is available in the hosted web edition.

## Check and package

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
python3 rust/scripts/test_dev.py
bash rust/scripts/package_macos.sh
```

The packaging script creates `dist/SAD PET.app` and `dist/SAD-PET-macOS.zip`.
Original firmware images and generated analysis are not required to build or run.
One ignored reconstruction test uses seven optional local `.fw` fixtures; with those
fixtures in the workspace root, run it with `cargo test -- --ignored`.

## Files and compatibility

New presets download as `.sad.json`. Existing `.evil.json` presets, imported audio,
recordings and browser library data remain compatible. Keep using the same browser
profile at `http://127.0.0.1:3214/` to access the existing IndexedDB library.

See [the user guide](rust/README.txt) for controls, recording and CLI rendering.

## DAW plug-ins

**[Download macOS and Windows installers](https://github.com/fushugaku/sad-pet/releases/latest)**

The native plug-in build reuses this engine and embeds the existing editor.
**SAD PET** is a MIDI instrument; **SAD PET FX** processes stereo host audio and
also accepts MIDI. Both provide VST3 on macOS/Windows and Audio Units on macOS.
Projects save the full preset and sample audio. See the [plug-in guide](plugin/README.md)
for installation, Logic's MIDI-controlled effect routing, building and validation.

```sh
python3 plugin/scripts/build.py
```

## Background

SAD PET began as a desktop reconstruction informed by analysis of EVIL PET 1.63.
It adds its own performance modes, modulation, filters, effects and file workflows.
Resynth uses the shared Rustias fixed-point OSC1 kernels and control compilers
at [91cae3f](https://github.com/fushugaku/Rustias/tree/91cae3f57a868a80d32a2e6ebb113938efd65025), with analytic firmware-free tables. Its host-rate
16-band analyzer is a SAD PET addition. The independent OSC 4 Wavefolder uses
Rustias's f64 four-step folding, smoothing and DC blocking from
`circuit/expanded.rs`; its five models are available in every Resynth slot.
PCM reads the selected SAD PET sample;
Audio In uses the analyzed input bands. Hardware PCM ROM and firmware are not
included. The same Rust implementation compiles natively and to WebAssembly.
Both filter slots also use Rustias's original fixed-point resonant/Comb kernels
and its `circuit/filters.rs` expanded models at the same pinned revision.
The f64 nonlinear kernels retain two oversampling passes and their transfer
functions; the adapter extends coefficient preparation to host sample rates.
These are the Rustias software models, not measured hardware replicas.
It does not claim bit-identical audio or full hardware firmware compatibility.
[Reconstruction notes](rust/PORT_STATUS.txt) and the [table manifest](rust/assets/manifest.json)
describe which formulas and lookup tables came from the supplied image.
Original firmware images and decompiler output are excluded from this repository.

The recorded sample sources and credits are listed in
[rust/assets/samples/SOURCES.txt](rust/assets/samples/SOURCES.txt).

Desktop MP3 support uses LAME and `mp3lame-encoder`; their source references and
license texts are preserved in [rust/licenses](rust/licenses/README.txt) and in the
application bundle. Browser MP3 support uses the unmodified, locally bundled
[`@breezystack/lamejs`](wasm/web/vendor/README.txt) module.

SuperEcho and AlphaVerb are available in Source FX, Pre FX and FX. SuperEcho uses
KekLocator's original Rust Tape/BBD delay, orbit, filters, pitch/frequency shimmer,
dub, compressor, freeze and 17 tempo divisions. AlphaVerb uses Dayverb's original
Rust reverb, early/late balance, damping, EQ, spin, stereo, shimmer, compressor,
gate and freeze. SuperEcho includes all 51 original KekLocator factory presets;
AlphaVerb includes all 45 Dayverb presets, including the 15 gated reverbs. They
appear under Factory in each effect's preset selector in every rack and can be
saved as User copies. All controls save in presets; Pre FX/FX controls support DAW
automation, LFO/ENV, matrix and Neuropolis. Source FX is prepared before grain reads.
The same Rust cores run natively and in WASM. Both process at their original
48 kHz with 96-tap bandlimited conversion at other host rates. The wet paths have
23 frames of streaming latency at 48 kHz (plus conversion at other rates);
SAD PET's dry path remains immediate. Original DSP instances are allocated only
for selected slots, prepared on the control thread in native builds. Numeric
updates retain tails and do not allocate; removed instances retire off the audio
callback. See `vendor/superecho/NOTICE.md` and `vendor/alphaverb/NOTICE.md` for source
revisions, firmware ownership and provenance.
