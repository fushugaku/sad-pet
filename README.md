# SAD PET

A granular instrument with a shared Rust audio engine for desktop and WebAssembly.

**[Play SAD PET in your browser](https://fushugaku.github.io/sad-pet/)**

The web version runs entirely in the browser: Rust DSP in an AudioWorklet, with
sample preparation, WAV rendering and MP3 encoding in a separate worker. No backend
or installation is required. Click Play or a keyboard key to enable sound.
The desktop version uses the system audio output and a local browser panel.

- Ten built-in audio sources, WAV import and an IndexedDB file library, including
  Matrix POV video audio and a short Harvest Dawn excerpt from Oblivion.
- Browser microphone input with device selection, gain and level metering:
  direct filters/effects or granular processing of an eight-second live buffer.
- Polyphonic keyboard, note/chord hold, arpeggiator and a 16-step chord sequencer.
- Three LFOs with tempo sync up to 8 bars, three ADSR envelopes and eight modulation routes.
- Two filters with single, serial and parallel routing, envelope depth and key tracking.
- A reorderable chain of up to 12 effects: distortion, rate reducer, bit crusher,
  flanger, chorus, binaural processing, reverb, L/C/R delay and modulated delay.
- Live modulation feedback on controls, MIDI input and stereo WAV/MP3 recording.

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
node wasm/test-microphone.mjs
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
audio. Select **Microphone** to allow a live input, then choose **Filters + FX**
for immediate processing or **Grains + Filters + FX** for granular processing.
**Freeze** retains the granular input buffer; **Save input** saves up to the last
eight seconds as WAV in Library. **Record** captures the actual processed output
as WAV and MP3. **File**, **Stop audio**, loading a WAV or closing the page releases
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

## Background

SAD PET began as a desktop reconstruction informed by analysis of EVIL PET 1.63.
It adds its own performance modes, modulation, filters, effects and file workflows.
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
