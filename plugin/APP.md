# SAD PET app

The standalone SAD PET for macOS and Windows. The Rust engine is compiled
natively (no WebAssembly) and runs on the computer's audio and MIDI devices
through JUCE: CoreAudio on macOS, WASAPI or DirectSound on Windows. The window
shows the same editor as the plug-ins.

## Audio never passes through the editor

The editor is a page in the system WebView (WKWebView / WebView2). It sends
commands — a knob value, “load this library file”, “render 16 seconds” — and
shows the engine's state, meters and waveform outlines. Sample data stays in
native code:

- Playback, the microphone, rendering and recording run in Rust and C++ on the
  audio device's thread.
- **Load WAV**, **+ Import files**, preset and effect-preset import open native
  file dialogs; Rust reads the chosen file. On macOS, files dropped on the
  window go straight to the native library.
- **Save edit**, **Render WAV** and **Record** write WAV files into the library
  in native code. **↓** / **WAV ↓** and preset export save through a native
  Save dialog.
- The app refuses any editor request that would send or receive sample data,
  so a regression shows up as an error rather than a silent JavaScript path.

## Install

**macOS 11 or later** (Apple silicon and Intel): unzip
`SAD-PET-App-macOS-universal.zip` and move **SAD PET** to Applications. The app
is ad-hoc signed, so on first launch Control-click it and choose **Open**.

**Windows 10/11 x64**: unzip `SAD-PET-App-Windows-x64.zip` and run
`SAD PET.exe`; keep the DLLs beside it. The editor needs the Microsoft Edge
WebView2 Runtime, which Windows 11 and updated Windows 10 already include.

## Audio and MIDI

**Audio/MIDI…** in the footer (or in the SAD PET menu on macOS) chooses the
output device, sample rate, buffer size and MIDI inputs. All MIDI inputs are
enabled automatically, including devices connected later. The computer
keyboard plays notes as in the plug-ins.

The app keeps its own transport: **Play** starts the sequencer, arpeggiator and
drum patterns at the editor's BPM.

## Microphone

The audio input opens only while **Microphone** is the source; macOS asks for
permission the first time. Choose the input device in **Audio/MIDI…**.
Switching back to **File** closes the input again. **Record** can capture the
microphone alone or with processing.

## Library and session

Audio files, edits, renders and recordings are stored in

- macOS: `~/Library/Application Support/SAD PET/Library`
- Windows: `%APPDATA%\SAD PET\Library`

User presets and effect presets share the plug-ins' folders, so a preset saved
in the app opens in the DAW and the other way round. Quitting the app saves the
whole scene, including its samples, and the next launch restores it.

## Build from source

Requires CMake 3.22+, Rust (rustup) and Xcode command-line tools or Visual
Studio 2022:

```
python3 plugin/scripts/build_app.py --smoke              # this Mac's architecture
python3 plugin/scripts/build_app.py --universal --smoke  # arm64 + x86_64
python plugin/scripts/build_app.py --smoke               # Windows x64
```

The archive is written to `dist/app/`. `--smoke` launches the built app once
and checks that its editor connects to the engine without errors.

For editor QA without the native build, `python3 plugin/scripts/preview.py`
serves the app editor at `/?app` against the real engine library; native file
dialogs are simulated with `--open` files and an `Exports` folder.
