# SAD PET plug-ins

The existing Rust granular engine runs inside the DAW. The editor is embedded in
the plug-in window; it does not launch a separate browser, audio device or server.

| Plug-in | Use | macOS | Windows x64 |
| --- | --- | --- | --- |
| SAD PET | MIDI instrument with imported or built-in samples | VST3, AU instrument | VST3 |
| SAD PET FX | Stereo input through filters/effects or an eight-second granular buffer; also receives MIDI | VST3, AU MIDI-controlled effect | VST3 |

## Install

Close your DAW before installing or replacing plug-ins.

On macOS, open `SAD-PET-Plugins-macOS-universal.pkg` and follow Installer. It installs
both plug-ins as AU and VST3 for the current user, without administrator access, in
`~/Library/Audio/Plug-Ins/Components` and `~/Library/Audio/Plug-Ins/VST3`. Reinstalling
replaces the bundles at those paths; user presets stay in Application Support.
The universal package supports both Apple silicon and Intel. The ZIP with
`Install.command` remains available for manual installation. These development
builds have an ad-hoc signature; the installer is unsigned. Developer ID signing
and Apple notarization are separate release steps.

On Windows, copy the two `.vst3` directories from the `VST3` folder into
`C:\Program Files\Common Files\VST3`, or run `Install.ps1` from an administrator
PowerShell window. The editor requires the Microsoft Edge WebView2 Runtime,
normally already installed on Windows 10/11. If it is missing, install it from
[Microsoft](https://developer.microsoft.com/microsoft-edge/webview2/).

Restart your DAW and rescan plug-ins:

- **Ableton Live:** enable VST3 system folders in Settings → Plug-ins. Place SAD
  PET on a MIDI track or SAD PET FX on an audio track. To send MIDI to the effect,
  route a MIDI track's output to the track and the SAD PET FX device.
- **FL Studio:** use Options → Manage plugins → Find installed plugins. Load SAD
  PET as a channel instrument or SAD PET FX in a mixer slot. To control the
  effect with MIDI, assign its wrapper a MIDI input port and send a MIDI Out
  channel to that port.
- **Logic Pro:** select SAD PET under AU Instruments → SAD PET. SAD PET FX is an
  **AU MIDI-controlled effect**: load it in a software instrument channel from
  AU MIDI-controlled Effects → SAD PET, select the source audio track in the
  plug-in's Side Chain menu, and play MIDI on the instrument channel. This is
  Logic's routing for effects that accept MIDI.
  When processing another software instrument, keep that instrument on its
  source track and put SAD PET FX in the Instrument slot of a second track.
  Copy the MIDI region to the SAD PET FX track and select the source instrument
  in Side Chain. Inserting SAD PET FX in Audio FX after the source instrument
  sends it audio alone; that track's MIDI region does not reach its envelopes.
  Bypass that earlier insert when using the second track. To hear only the
  processed signal, route the source output to an unused bus, set the bus's aux
  output to No Output, and choose that bus as the SAD PET FX Side Chain source.

## Controls and project recall

The preset selector opens **Factory** and **User** banks. **Save** opens a name
field and saves directly into User. Saving an existing user preset replaces it;
**Save as new** creates a separate patch. The dot beside the name marks changes.
The same User bank is available to every SAD PET / SAD PET FX instance.

User presets live in `~/Library/Application Support/SAD PET/Presets` on macOS
and `%APPDATA%\SAD PET\Presets` on Windows. A `.sadpet` file contains compressed
settings, both FX racks, modulation, tuning, sequence data and embedded sample
audio with its region/fades. It can be transferred between computers without the
original WAV. Import and Export are in the preset browser; older `.sad.json`
presets can still be imported. Previously saved browser presets migrate into
the native User bank once. Delete moves user files to the preset folder's Trash.

Preset loading retains this instance's transport and bypass. An input FX patch
also opens in the instrument using its stored file source. DAW projects retain
their active preset name/selection and modified status as well as the sound.

SAD PET starts silently and responds to MIDI notes. SAD PET FX starts processing
its stereo host input immediately with **Grains + Filters + FX** selected.
Choose **Filters + FX** to bypass the grain engine. **File** switches to the
sample instrument path. Loading a WAV also selects File.

MIDI includes note velocity, sustain, pitch bend, modulation wheel, channel
pressure and polyphonic pressure. Notes engage the engine's MIDI gate, including
in the effect, so they can trigger envelopes and gate the processed input.
Use Play to return to continuous playback when the keys are released.

**DAW · … BPM** follows the host tempo and starts/stops the sequencer/arpeggiator
with the host transport. Moving or looping the playhead repositions those modes.
**Free tempo** enables the instrument's own tempo control. The MIDI keyboard
continues to work while the host is stopped. Audio processing adds zero latency;
grain position/history and individual effects can introduce audible delays.

The signal path is grains/source blend → Pre-filter effects → note filters →
Post-filter effects. Both racks have the same catalog and up to 12 independent
slots. Old presets load with an empty pre-filter rack and retain their sound.

**Waveshaper** ports Drive and all eleven RADIAS WS types from radias-emulator:
Hard Clip, Decimator, OctSaw, MultiTri, MultiSin, SubSaw, SubSquare, SubTriangle,
SubSine, Pickup and LvlBoost. Type and Depth retain their original transfers;
Mix and bypass belong to the FX slot. Select a rack for its placement. SubOSC
and Pickup use the latest MIDI pitch (C4 before the first note); SubOSC in the
post-filter rack follows the note envelope and stays silent without a voice.

Host automation exposes grain, filter, gain, tuning, LFO, envelope, performance
and every numeric FX setting. FX automation addresses rack positions (FX 1–12
and Pre FX 1–12), including Waveshaper Depth. Existing parameter IDs and order
are preserved.
Editor knobs send coalesced numeric changes through `/api/parameters`, including
both FX racks, LFOs, envelopes, performance controls and matrix depths. They do
not replace the preset, rebuild performance state or restart held notes. The
audio callback refreshes changed values once per block, after host tempo sync;
unchanged FX/filter/envelope coefficients are reused. Loading a preset and
editing routing or rack structure still use the complete preset.
Discrete routing, waveform/type selections and modulation routes are stored in
the full preset. Automation follows the host's parameter-block delivery; MIDI
events are rendered at their exact sample offsets within each block.

DAW project state contains the full preset, imported sample audio, selected
region, fades, input mode and sync settings. It does not depend on an IndexedDB
library or an external sample path. Live input history, held keys and effect
tails are transient. Live input Freeze is cleared on recall. The embedded
browser library remains useful for browsing/importing files, but cannot replace
the source or fades restored by the DAW.

Use the DAW to record, bounce and export. The standalone application's recording,
microphone selection, MIDI-device selection and Quit controls are hidden here.

## Build

Install stable Rust, Python 3, CMake 3.22+ and the native C++ tools: Xcode Command
Line Tools on macOS or Visual Studio's Desktop development with C++ on Windows.
The build uses JUCE **8.0.16-security** at a pinned commit. It downloads JUCE on
the first build; `--juce /path/to/JUCE` uses an existing checkout. Windows also
downloads a pinned WebView2 SDK package from NuGet.

```sh
python3 plugin/scripts/build.py
# Apple silicon + Intel in the same macOS package:
python3 plugin/scripts/build.py --universal --build-dir target/plugins-universal
```

Builds and native host integration tests run before packaging. ZIPs, a macOS
`.pkg` installer and SHA-256 checksums are written to `dist/plugins/`.
`--package-only` packages an existing build without recompiling it; `--skip-tests`
is available for local iteration. macOS packaging uses the system `pkgbuild` and
`productbuild` tools. The package contains no sample/preset library writes or
network downloads; documentation and notices go in
`~/Library/Application Support/SAD PET/Documentation`.

The GitHub Actions **Build DAW plug-ins** workflow creates Windows x64 and macOS
universal packages, runs native integration tests on both systems, and validates
both Audio Units on macOS. Workflow artifacts contain the ZIPs, macOS installer
and checksums.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
ctest --test-dir target/plugins -C Release --output-on-failure
auval -v aumu SaPi SdPt
auval -v aumf SaPf SdPt
```

For browser editor QA without playing sound:

```sh
cargo rustc -p sad-pet-plugin --lib --release --crate-type cdylib --locked
python3 plugin/scripts/preview.py
# http://127.0.0.1:8766/ and http://127.0.0.1:8766/?fx
```

`plugin/src` owns the host-driven Rust adapter, its stable parameter registry and
the C ABI. `plugin/native` owns the JUCE format wrappers and editor bridge.
Commands and fixed-size telemetry cross bounded SPSC queues. Presets, sample
mipmaps, live buffers and restored project data are prepared off the audio
callback; retired buffers are also destroyed off it. Tests track heap allocation
and freeing during rendering, MIDI capture, preset/source changes and transport.

See [third-party notices](THIRD_PARTY.md) before distributing binaries.
