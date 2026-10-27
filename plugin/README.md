# SAD PET plug-ins

The existing Rust granular engine runs inside the DAW. The editor is embedded in
the plug-in window; it does not launch a separate browser, audio device or server.

| Plug-in | Use | macOS | Windows x64 |
| --- | --- | --- | --- |
| SAD PET | MIDI instrument with imported or built-in samples | VST3, AU instrument | VST3 |
| SAD PET FX | Stereo input through filters/effects or an eight-second granular buffer; also receives MIDI | VST3, AU MIDI-controlled effect | VST3 |

The same engine and editor also run as a standalone app for macOS and Windows,
on the computer's audio and MIDI devices: see [APP.md](APP.md).

New instances open **glasses5**, including the Dusty keys source, selection/fades,
Resynth, sequencer and effect chains. It appears first in Factory presets; Init
is still available. Restoring a saved DAW project replaces this initial scene.
SAD PET FX keeps its DAW input as the source.

## Effect presets

Every Source FX, pre-filter FX and post-filter FX slot has a preset selector and
**Save**. Presets are shared across all slots and timbres of the same effect type.
They store the effect's controls, modes, tempo sync, Smooth and Mix; applying one
keeps the slot identity, bypass state, modulation routes and other effects.

**AlphaVerb** includes all 45 original Dayverb factory presets (30 original and
15 gated reverb); **SuperEcho** includes all 51 KekLocator factory presets.
They appear under **Factory** in the effect's selector in every rack. Save makes
an editable User copy; factory entries stay available and can be exported.

Save updates the selected preset after edits; **Save as new** creates a separate
entry. The **⋯** menu imports/exports portable `.sadfx` files and deletes saved
entries with **Undo delete**. **Init** restores the algorithm's default settings.
Plug-in instances share the disk library in the **Effects** subfolder of the
User preset folder. The web version keeps its bank in IndexedDB; use `.sadfx`
to transfer an effect preset between browsers, macOS and Windows.

## Instrument presets

**Instruments** next to a timbre's name saves and loads one timbre's sound:
its engine (Granular, Drums, Synth, Vocal or Sampler), every setting, its
sample and source selection, and a Sampler's kit. **Save…** asks for a name and
a category of your own; a name already in that category is replaced. The
browser lists instruments by category with search. Clicking one replaces the
active timbre's sound in place, keeping its level, sends, MIDI channel and
position, while the other timbres play on; **+** loads it as a new timbre. The
**⋯** menu renames or moves an instrument, exports a portable `.sadinst` file or
deletes it, and **Rename category…** renames a whole category. Bus and Master
hold no instrument. Plug-in instances share the disk library in the
**Instruments** subfolder of the User preset folder (deleted files go to its
**Trash**); the web version keeps its library in IndexedDB.

## Timbres and drum machine

Use the top **Timbres** bank: choose Granular, Drum machine, Synth engine or Bus,
then **+ Timbre**. Every scene also has one **Master** tab: all timbres and buses
sum into it, and its engine renders the plug-in output. A timbre's mixer row sets
its level into Master and a send to each bus; a bus or Master can switch on
Granular, Filters, Resynth and effects, and passes its input untouched while
everything is off.
Tabs select the editor; every timbre continues running. Each has independent
level, mute, solo and MIDI-channel selection (All, or 1–16). Existing host parameter
IDs continue to automate the first timbre; every timbre supports its own numeric
editor updates, LFO/ENV/matrix routes and Neuropolis modulation tracks. DAW project
state and User presets save the complete scene, including all source files.

The drum timbre uses the native DXM-088 / LXR-02 Rust engine, with seven tracks
(three drums, snare, clap/cymbal, closed and open hats sharing one voice) and all
56 voice controls. Length is 1–128 steps per track; the eight pattern-page buttons
show 16 steps at a time. Tracks retain independent clocks, swing, groove and mix.
Click a step to toggle a hit and edit it below; Shift-click selects without
changing its velocity. Velocity, chance, pitch, ratchet and microtiming have
rotary controls. Conditions and two parameter locks remain visible alongside.
Audition plays a single drum even while DAW transport is stopped.

Enable **Granular** beside Dry/Wet to send the drum mix through Grain Engine.
Drum, Synth engine, Bus and Master pages keep the Granular page order: their own
section on top, then the waveform, transport, Source FX and Grain Engine. Selecting a
drum timbre also shows the shared waveform display and its grain positions, in
both direct and granular modes. **Buffer** selects 125 ms–32 s of history (8 s
by default). Position and Cloud follow that duration; Size uses the full knob
range up to the shorter buffer length, with a 10 ms minimum. Changing duration
retains captured sound and Freeze without allocating audio buffers or restarting
the pattern. Each timbre saves its own duration in presets and DAW projects. Selecting a
drum track or step also selects its sound editor, and voice knobs repaint during
the drag without waiting for the next engine-state poll.
Source FX processes the drum audio before capture; pre-filter FX, filters, Resynth
and post-filter FX remain available on the selected timbre. Main pitch transposes
only the grains: switching Granular off returns the drum voices to their own
tuning. Sample tune transposes the drum voices themselves, before the grains
hear them. The drum page's Neuropolis editor is independent of every other timbre's
program.

The Synth engine timbre reverses the granular order: up to four Rustias OSC1 /
Wavefolder oscillators (the Resynth card set without Audio In) play each of the
eight note voices from DAW MIDI, the arpeggiator or Neuropolis, then that note's
filters, ENV/VCA and Pre-filter FX. **Granular** beside Dry/Wet sends the mixed
voices after the VCA into a 125 ms–32 s **Buffer** read by Grain Engine; Source FX
processes that mix before capture, and Main pitch transposes the grains. A new
synth timbre waits for notes (MIDI gate on) instead of droning with the transport.

Use **+ LFO** / **+ ENV** to add up to eight of each per timbre, and **− LFO** /
**− ENV** to remove the last added modulator down to three. Destination menus
include every Resynth oscillator's realtime numeric controls and all drum voice
controls. Knob edits, drum steps and the Granular switch use sparse commands;
they preserve clocks, voice histories and other timbres. Three-modulator patches
migrate without changing the original routes or existing host parameter IDs.

On a **Main track pitch** lane, the **Main pitch → OSC 1–4** checkboxes sit directly
below its destination selector. An oscillator's own pitch lane uses signed
−24…+24 semitone offsets relative to the selected main pitch. Pulses and Ratchet
have integer 1–8 knobs: drag, focus and scroll, or use arrow keys; double-click
resets to 1. Gate remains a separate percent knob.

Each Neuropolis track also has **Destination 1–3**. Assign them to any available
realtime parameter: filters, FX, OSC controls, drum voices or modulators. Every
step has three corresponding bipolar amount knobs (−100…+100% of the target's
range). Values follow that track's own clock and play order, remain active for
the whole step and are added to the saved base value. Muting the track removes
all its contributions. Step edits use sparse commands and preserve playback.

To copy steps, select them on the bar above each stage: drag for a run, Shift
extends, ⌘/Ctrl adds or removes, Esc clears. **Copy** and **Paste** (or ⌘/Ctrl+C
and ⌘/Ctrl+V, when the host passes them to the editor) work within a track and
between tracks. A single selected step is where the copy starts; a longer
selection is filled, repeating the copy, and every selected region gets it.

Modulated knobs show the actual value with a moving cyan pointer and range;
the small pale tick and green arc retain the saved base setting. This includes
pre-filter and post-filter effects. LFO Depth displays its signed percentage.
L/C/R Delay **Time Ratio** always uses continuous values and a 0.25 percentage
point gesture step. Ratio changes glide in audio even with Smooth off; Smooth
still enables the intermediate tap times between musical divisions. Existing
RADIAS ratio positions in saved presets and host automation retain their mapping.
LFO/ENV/matrix modulation changes the physical Time Ratio continuously without
the gesture's 0.25% quantization or the original ratio table's flat regions.

### Vocal timbres

**+ Timbre → Vocal** adds a singing part with its own score editor (see the main
README). In the DAW the score plays at the host's song position and tempo while
**DAW tempo** is on: start the transport where the notes are. Phrases render on
a background thread as soon as the score changes and are ready long before
playback; a phrase that is still rendering stays silent for that pass, so let
the status line read *ready* before an offline bounce. The project saves the
score, not the rendered audio. The voicebanks that come with SAD PET are built
into the plug-in. Imported voicebanks are stored as `.sadvoice`
files in the **Voices** folder next to the User presets
(`~/Library/Application Support/SAD PET/Voices`, `%APPDATA%\SAD PET\Voices`),
so every instance and the app share them.

## Install

[Download the macOS and Windows installers](https://github.com/fushugaku/sad-pet/releases/latest).

Close your DAW before installing or replacing plug-ins.

On macOS, open `SAD-PET-Plugins-macOS-universal.pkg` and follow Installer. It installs
both plug-ins as AU and VST3 for the current user, without administrator access, in
`~/Library/Audio/Plug-Ins/Components` and `~/Library/Audio/Plug-Ins/VST3`. Reinstalling
replaces the bundles at those paths; user presets stay in Application Support.
The universal package supports both Apple silicon and Intel. The ZIP with
`Install.command` remains available for manual installation. These development
builds have an ad-hoc signature; the installer is unsigned. Developer ID signing
and Apple notarization are separate release steps.

On Windows 10/11 x64, run `SAD-PET-Plugins-Windows-x64-Setup.exe`. It installs both
VST3 bundles in `C:\Program Files\Common Files\VST3` and adds an uninstaller to
Windows Settings → Apps. Updates replace the plug-in bundles and preserve User
presets. The installer includes Microsoft's Visual C++ Runtime installer and
installs WebView2 from Microsoft if missing; only that WebView2 step needs internet.
It checks Microsoft signatures when packaging prerequisites. The SAD PET installer
itself is unsigned. The ZIP and administrator `Install.ps1` remain available for
manual installation.

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
settings, Resynth, Source FX and both FX racks, modulation, tuning, sequence data and embedded sample
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

**Resynth** uses 16 frequency-band envelopes to excite up to four Rustias
oscillator layers (64 carriers). Turn **On**, choose **Source** or **Grains** as
the detector input, and add layers with **+ Oscillator**. Each layer offers Saw,
Pulse, Triangle, Sine, Formant, Noise, Audio In or PCM. Saw/Pulse/Triangle/Sine
also have Wave, Cross, Unison and VPM modes. CTRL 1/2 retain the original Rustias
control transfers; Wave/Cross LFO depth uses LFO 1, Unison uses detune/phase,
and VPM uses depth/ratio. **Generator → Wavefolder · OSC 4** is available in
every slot, with Triangle, Sine, Soft, Asymmetric and Rectify shapers. Its CTRL 1
sets Fold Depth and CTRL 2 sets Bias, using Rustias OSC 4's f64 four-step folding,
smoothing and DC blocker.

Each **Filter 1 / Filter 2** model selector includes all 18 Rustias models:
RADIAS, Ladder LP/HP/BP24, MS-20 LP/HP12, Pro LP24, SEM, Multi Filter,
M/Poly LP24 and Linear LP/HP/BP/Notch at 12/24 dB. RADIAS Filter 2's LP/BP/HP
and Comb are available in either slot too. **Drive** shapes the input;
**Morph** selects RADIAS's LP24 → LP12 → BP → HP → Thru, SEM's LP → Notch → HP,
or Multi Filter's LP → BP → HP → Notch. Controls unused by the selected model
are dimmed. These are the actual Rustias software kernels, with host-rate
coefficient preparation. Existing SAD PET filter choices retain their sound.
Both slots retain ENV 1–3, signed depth, key tracking and Single/Serial/Parallel
routing. Drive and Morph accept modulation, Neuropolis tracks and DAW automation.
Numeric edits preserve filter history and existing host parameter indices.

Each slot has independent **Ring / Sync** (Off, Ring, Sync, Ring + Sync) and
**Ring / Sync source** (Source, Grains or another oscillator). Oscillator taps
precede their level/pan and envelope, so a source at zero Level still drives the
other layer. Audio In supports Ring; sampled PCM also supports Sync. Source and
Grains use their individual band-filtered zero crossings with hysteresis; an
oscillator reference uses its carrier phase wrap. Grains is tapped before the
Dry/Wet blend. Cross separately selects Analysis input, Source, Grains or another
oscillator. For oscillator connections, earlier slots use the current internal
sample and self/forward
connections use the previous internal sample. PCM reads and loops the selected
sample region at the layer's tuning, using **Root** as its original pitch. Start
and Loop Fade adjust that loop. Audio In passes the analyzed input through the
band envelopes. Sensitivity, attack, release, threshold and Mix are shared.
The signal then enters the existing per-note filters and envelopes.

Resynth settings and all four layers are saved in User presets and DAW projects;
its host controls are appended after existing parameter IDs. Old presets keep
Resynth off and preserve their original output. Numeric changes retain oscillator
phase/feedback and envelope histories. All processing is native Rust in the
plug-ins, with the same Rust implementation compiled to WASM for the web version.

MIDI includes note velocity, sustain, pitch bend, modulation wheel, channel
pressure and polyphonic pressure. Notes engage the engine's MIDI gate, including
in the effect, so they can trigger envelopes and gate the processed input.
Use Play to return to continuous playback when the keys are released.

**MIDI learn**: right-click any knob (or press the menu key on a focused knob),
choose **MIDI learn** and move a controller; from then on that CC turns the knob
across its range and its number shows on the knob. Esc cancels. One CC turns one
knob, so learning it again moves it; **Forget CC…** and **Forget all MIDI
mappings** in the same menu remove mappings. A mapping names the knob, not the
timbre: the controller turns that knob on whichever timbre the editor shows.
Mappings are kept by the editor for every project and preset of that host
(the browser, or the DAW's plug-in web view). The editor applies them, so a
learned controller turns knobs while the plug-in window is open; use the DAW's
own MIDI learn on host parameters to automate with the window closed.

The same menu routes **modulation** to the knob or slider: an existing or new
LFO, ENV, Neuropolis track or macro, or another mod matrix source (velocity, mod
wheel, pitch bend…), and **Remove modulation** lists every route to it. LFOs and
envelopes each have three routes of their own and fall back to the matrix when
those are taken; a new track plays the control and is selected for drawing.
**Macros** are knobs of your own that each turn up to eight destinations; their
values are host parameters `macro1`–`macro8`.

**DAW · … BPM** follows the host tempo and starts/stops the sequencer/arpeggiator
with the host transport. Moving or looping the playhead repositions those modes.
**Free tempo** enables the instrument's own tempo control. The MIDI keyboard
continues to work while the host is stopped. Audio processing adds zero latency;
grain position/history and individual effects can introduce audible delays.

**Loop 1 bar**, beside the sample selection, scans the entire selection once per
bar at any MIDI note or Pitch setting. Each note starts its own loop from the
selection's start; playing another note does not restart held notes. **Root** is
the original sample note (C4 by default). Pitch, semitone/cents tuning and pitch
bend change the sound without changing loop duration. The plug-in uses the DAW's
time signature, including 3/4 and 6/8, and follows tempo changes without restarting
the loop. Without a host, the bar is four beats at the Tempo setting. Freeze pauses
the scan; note releases still follow the envelope. Save a live recording as a file
to select and loop it. The checkbox and Root are stored in presets and DAW projects.

The signal path is source → **Source FX** → grains/source blend → **Resynth**
→ Pre-filter effects → note filters → Post-filter effects. Source FX is one independent slot
with the same effect catalog. Live input is processed before entering the grain
history. For files, only the selection is prepared in the background; changes
become audible when ready and crossfade without restarting notes or loops.
Tails outside that selection are excluded. Source FX numeric controls accept DAW
automation with asynchronous file updates; the slot is not an LFO/matrix destination. Both racks have the same catalog and up to 12 independent
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

## Neuropolis sequencer

Enable **Neuropolis**, then **+ Add track**. The destination selects **Main track
pitch**, **OSC 1–4 pitch offset**, **Notes → grains + ENV**, or a named realtime parameter.
Main pitch uses **Root note** as zero transposition. Its **Pitch → OSC 1–4**
checkboxes also transpose the selected oscillators. A separate oscillator track
adds its own **−24…+24 st** offset to that main pitch, with its own clock, length,
gate and slide. The oscillator's saved Transpose and Fine settings remain additive.
Unchecked oscillators keep their own tuning and oscillator-track offsets.
Parameter tracks edit signed values: Relative adds CV × Depth to the saved knob;
Absolute blends the full parameter range according to Depth. Source FX for files
is prepared ahead of playback and is not a realtime destination.

Each track has its own Clock, Length, Order, Swing and Gate. Clock starts at 1/16;
×2/×4 produce 1/32 and 1/64, and ÷2–÷64 slow that track independently. Each of the
16 columns shows its Pitch/value fader, **Gate length knob**, gate type, 1–8
pulses, Ratchet, Slide, Chance and Velocity together. Gate is 0–100% of the pulse;
double-click its knob to inherit the track's Gate %. Click the gate type to cycle
Rest/Single/Multi/Hold; click a step number to skip it. **Step overrides &
accumulation** contains pitch overrides, accumulation and CV ramps.
Scale, slide & accumulation holds the other
track settings and independent track/accumulator resets. Duplicate copies a
track; M mutes only that lane. Adding, removing or retargeting a lane preserves
the other playheads by stable track ID. An edit that moves a track in time (Clock,
Length, Order, direction, Swing, Offset, pulse settings, a step's pulses or skip)
puts it where its new settings would have it had it played since the transport
started, or since its own **Restart track**, so undoing an edit brings it back
in step with the other tracks. A track added while playing waits silently for
its place in the same way.

There is no fixed track-count limit. The list pages its readouts in groups of 64;
all tracks continue processing. Note tracks share SAD PET's existing eight-note
audio polyphony. DAW sync follows host tempo and starts each pattern at the first
96 PPQN grid tick after transport starts or seeks; preset changes while the host
plays keep the patterns running. Stopping the sequencer releases
its own notes while input processing and keyboard audition remain available.
Turn off DAW sync to use the editor's transport instead.

The same pinned Neuropolis Rust core drives native AU/VST3 and the web version.
Numeric edits update one field without rebuilding the sequencer or resetting its
clock. Structural changes are prepared by the native Controller and adopted by
Audio without allocation/freeing. Browser scalar telemetry carries changed values
and readouts, with buffers sized for the program. Presets and projects store all
lanes and assignments; legacy chord patterns migrate to separate note tracks.

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

`--pgo` builds the engine with profile-guided optimization: an instrumented build
runs `plugin/examples/pgo_training.rs` (the plug-in, effect and app scenes plus every
filter, effect, modulator and oscillator type), then the engine is rebuilt with that
profile. It renders the same samples, which the build checks against the training
run. `--pgo aarch64-apple-darwin` (comma-separated triples) limits it to those
targets. It needs `llvm-profdata` from rustc's LLVM version (`rustup component add
llvm-tools`, or `LLVM_PROFDATA`); without one, or for a target this machine cannot
run (x86_64 needs Rosetta 2 on Apple silicon), that target builds without PGO and
prints a warning. Profiles are cached in the build folder under a hash of the
toolchain and every engine source, so any change trains a new one. On an M5 Max
every case of the engine benchmark (see `rust/examples/audio_benchmark.rs`) runs
1.8-23% faster, 9.0% on average. Native x86_64 runs most cases faster as well (6.3%
on average on GitHub's AMD EPYC Windows runner, 9.3% on its Intel Mac), but on Windows
the scenes that play Resynth layers, including the first-open preset, run up to 21%
slower, so CI profile-guides only the Apple silicon slice. `python3 plugin/scripts/pgo_benchmark.py`
repeats the comparison on this machine, alternating normal and PGO builds and failing
if a sample changes; the **Benchmark profile-guided engine** workflow runs it on
Windows x64, an Intel Mac and Apple silicon.

Builds and native host integration tests run before packaging. ZIPs, a macOS
`.pkg` installer, a Windows Setup `.exe` and SHA-256 checksums are written to `dist/plugins/`.
`--package-only` packages an existing build without recompiling it; `--skip-tests`
is available for local iteration. macOS packaging uses the system `pkgbuild` and
`productbuild` tools. The package contains no sample/preset library writes or
network downloads; documentation and notices go in
`~/Library/Application Support/SAD PET/Documentation`.
Windows packaging needs [Inno Setup 6](https://jrsoftware.org/isinfo.php),
preinstalled on GitHub's Windows runner. Prerequisite installers are downloaded
from Microsoft and their Authenticode signatures are checked before inclusion.
Windows documentation/notices go in `C:\Program Files\SAD PET\Plugins\Documentation`.

The GitHub Actions **Build DAW plug-ins** workflow creates Windows x64 and macOS
universal packages, runs native integration tests on both systems, and validates
both Audio Units on macOS. Windows CI installs twice, verifies payload hashes and
stale-file removal, then uninstalls while checking preset/other-plug-in preservation.
Workflow artifacts contain both installers, ZIPs and checksums. Pushing
`plugins-v<version>` publishes a GitHub Release only after both systems pass;
the tagged commit must already have successful CI, and the version must match
`plugin/Cargo.toml`. The release reuses those exact tested packages without
rebuilding them. Packages are verified together and uploaded
to a draft before the complete release becomes public.
Installer-only CI changes may reuse previously validated native binaries after
comparing every Rust source/manifest, C++ wrapper, CMake setting and embedded editor
file against the reference commit. The installer is rebuilt and tested on both
systems; any native source change uses a full compilation instead.

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
