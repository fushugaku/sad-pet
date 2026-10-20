# SAD PET

A granular instrument with a shared Rust audio engine for desktop and WebAssembly.

**[Play SAD PET in your browser](https://fushugaku.github.io/sad-pet/)**

The web version runs entirely in the browser: Rust DSP in an AudioWorklet, with
sample preparation, WAV rendering and MP3 encoding off the audio thread. Dedicated
workers handle binary UI telemetry and recording ingestion. Audio views and
recording buffers are reused. No backend
or installation is required. Click Play or a keyboard key to enable sound.
The desktop version uses the system audio output and a local browser panel.

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
- Polyphonic keyboard, note/chord hold, arpeggiator and a 16-step chord sequencer.
- Three LFOs with tempo sync up to 8 bars, three ADSR envelopes and eight modulation routes.
- Two filters with single, serial and parallel routing, envelope depth and key tracking.
- A **Source FX** slot before the Grain Engine, with all existing effect types.
  Live audio is processed before grain capture; file selections are prepared in
  the background with the original FIR antialiasing and smooth source changes.
- **Resynth**: 16 analysis bands drive up to four independent Rustias oscillator
  layers, with Saw, Pulse, Triangle, Sine, Formant, Noise, Audio In and sample-based
  PCM. Analog waves support Wave, Cross, Unison and VPM. Each layer has its own
  controls, tuning, level and pan; the shared detector has sensitivity, attack,
  release and threshold. Choose Source or the Grain Engine output as excitation.
- Two independent reorderable chains of up to 12 effects each, before and after
  the filters: distortion, rate reducer, bit crusher,
  flanger, chorus, binaural processing, reverb, L/C/R delay, modulated delay and
  a stereo-linked compressor (threshold, ratio, attack, release, knee, makeup and mix).
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
node wasm/test-resynth.mjs
node wasm/test-microphone.mjs
node wasm/test-library.mjs
node wasm/test-presets.mjs
node wasm/test-parameters.mjs
python3 -m http.server 8080 --directory dist/web --bind 127.0.0.1
```

Open `http://127.0.0.1:8080/`. Deployment needs only the files in `dist/web` on an
HTTPS static host. Relative asset paths also work under a project subdirectory.
The GitHub Actions workflow builds, checks and publishes the site on every push to
`main`; repository Settings → Pages uses **GitHub Actions** as its source.

Use a browser with WebAssembly and AudioWorklet support. Web MIDI is optional and
requires a supported browser and permission through the MIDI refresh button.
Audio files, presets, rendered WAVs and WAV/MP3 recordings stay in IndexedDB and
can be downloaded from Library. Storage belongs to each browser profile and site
origin; the hosted site and the local desktop panel have separate libraries.

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
16-band analyzer is a SAD PET addition. PCM reads the selected SAD PET sample;
Audio In uses the analyzed input bands. Hardware PCM ROM and firmware are not
included. The same Rust implementation compiles natively and to WebAssembly.
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
