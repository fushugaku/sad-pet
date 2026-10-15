# SAD PET

A desktop granular instrument with a Rust audio engine and a local browser panel.
Audio plays through the system output; the browser controls the same running engine.

- Eight built-in audio tracks, WAV import and an IndexedDB file library.
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

MP3 support uses LAME and `mp3lame-encoder`; their source references and license texts
are preserved in [rust/licenses](rust/licenses/README.txt) and in the application bundle.
