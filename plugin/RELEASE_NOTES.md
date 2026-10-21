SAD PET instrument and SAD PET FX, with native Rust DSP and an embedded editor.

**Selectable drum granular buffer.** The drum heading's **Buffer** selector sets
125 ms–32 s of captured history per timbre, with 8 s as the compatible default.
The waveform, Position and Cloud use that duration; Size scales its full range
to short buffers. Changing duration retains audio and Freeze, uses preallocated
memory and leaves the drum clock running. Presets and DAW projects recall it.

**Live drum waveform.** Drum timbres now show the same waveform and grain display
as sample timbres, directly above Grain Engine. It reads the selected live
drum input history in both direct and granular modes. Native editors publish
the drum buffer instead of the unrelated file waveform. File selection controls
and fades return when switching back to a sample timbre.

**Responsive drum controls and visible granular processing.** Voice knobs repaint
during each drag instead of waiting for engine telemetry. Selecting a sequencer
track or step also selects its sound editor. With Granular enabled, the drum page
shows Grain Engine controls directly below the switch and a live grain count.

**Continuous LFO Time Ratio and removable extra modulators.** LFO/ENV/matrix routes
change L/C/R Delay's physical ratio continuously, without the knob gesture's
0.25% quantization or the original table's flat regions. Audio gliding and saved
ratio anchors are preserved. **− LFO** / **− ENV** removes the last added slot and
its outgoing routes; the first three and remaining slot addresses stay intact.

**glasses5 on first open.** New instances and fresh browser sessions load the
supplied complete sound: Dusty keys, its selection/fades, four-oscillator Resynth,
Neuropolis tracks and both FX racks. It appears first in Factory presets, with
Init still available. Existing project state and remembered browser User presets
take priority. Host parameter initialization preserves the startup sound.

**Three extra destinations per sequencer track.** Assign Destination 1–3 to
realtime controls; every step has independent −100…+100% amount knobs. They
follow the track's own clock and order, save with presets and DAW projects, and
use sparse realtime edits. Modulated FX knobs now move their cyan pointer to
the current value while marking the saved base setting separately. LFO Depth
shows a signed numeric percentage. L/C/R Delay Time Ratio changes continuously
in 0.25 percentage point gesture steps and glides even with Smooth disabled.

**Presets for every effect.** Each Source FX, pre-filter and post-filter slot has
its own named preset selector and Save button. Presets are shared across slots
and timbres of the same effect type, including all modes, tempo sync, Smooth and
Mix. Save, overwrite or save as new; import/export portable `.sadfx` files.
Deleting a preset supports Undo. Plug-ins share a disk library across instances;
the web version stores its effect presets in the browser.

**Multiple timbres, DXM-088 drums and eight modulators per bank.** Add Granular
or Drum machine timbres as top tabs; they mix simultaneously and retain their own
source, Resynth, filter/FX chains, LFO/ENV banks and Neuropolis program. Each has
level, mute, solo and MIDI-channel controls. Named presets and DAW projects save
the whole scene and all source files; old single-timbre presets remain compatible.

The drum timbre uses the native Rust DXM-088 / LXR-02 DSP: seven tracks, all 56
voice controls, two filters, independent clocks and up to 128 steps per track.
Velocity, chance, pitch, ratchet, microtiming, conditions and parameter locks are
editable. **Granular** optionally routes its live mix through Grain Engine; its
own Neuropolis mod sequencer can address drum and processing parameters.

Add up to **8 LFOs and 8 ENV** per timbre. Destinations include OSC 1–4 controls,
tuning, level/pan, Ring/Sync and the drum voices. Numeric edits, drum steps and
routing toggles use sparse updates without rebuilding running modulation or DSP.
The drum adapter is checked against the original engine's stereo samples.

**Main pitch → OSC 1–4** checkboxes are directly below the track destination.
Oscillator lanes add −24…+24 semitone offsets to main pitch. Every step keeps
Gate, Pulses, Ratchet, Slide, Chance and Velocity visible together. Pulses and
Ratchet use integer 1–8 knobs; drag, focus and scroll, or use arrow keys.
Double-click resets them to 1; Gate resets to the track's inherited length.

All sound processing is native Rust in the plug-ins. The web version compiles
the same engine to WASM. SuperEcho, AlphaVerb, Delay Smooth, Rustias filters and
four-oscillator Resynth remain included.

| System | Installer | Plug-in formats |
| --- | --- | --- |
| macOS 11+ · Apple silicon and Intel | [Download .pkg](https://github.com/fushugaku/sad-pet/releases/download/{tag}/SAD-PET-Plugins-macOS-universal.pkg) | AU + VST3 |
| Windows 10/11 · x64 | [Download Setup.exe](https://github.com/fushugaku/sad-pet/releases/download/{tag}/SAD-PET-Plugins-Windows-x64-Setup.exe) | VST3 |

Close your DAW, run the installer, then reopen the DAW and rescan plug-ins.
Works in Ableton Live and FL Studio; Logic Pro uses the macOS Audio Units.
In Logic, SAD PET FX is under **AU MIDI-controlled Effects** and takes audio from
the plug-in's **Side Chain** selector.

This release includes named User presets with portable `.sadpet` import/export,
independent pre-filter and post-filter FX racks, all 12 RADIAS Waveshaper modes,
and sparse knob updates that preserve held notes and existing modulation routes.

The Windows installer installs both VST3 bundles in the system VST3 folder and
provides an uninstaller. It includes the Microsoft Visual C++ Runtime installer
and installs WebView2 from Microsoft if missing; that WebView2 step needs internet.
Installing, upgrading and uninstalling preserve User presets.

Installers are unsigned; macOS plug-in bundles are ad-hoc signed and are not
Apple-notarized. ZIPs are available for manual installation. SHA-256 checksums
are supplied individually and in `SHA256SUMS.txt`.

[Installation and routing guide](https://github.com/fushugaku/sad-pet/blob/{tag}/plugin/README.md)
· [Corresponding source](https://github.com/fushugaku/sad-pet/tree/{tag})
· [Third-party notices](https://github.com/fushugaku/sad-pet/blob/{tag}/plugin/THIRD_PARTY.md)

---

В обоих установщиках есть инструмент **SAD PET** и эффект **SAD PET FX**.
**Buffer** на странице драм-машины выбирает от 125 мс до 32 с истории звука.
Волна, Position и Cloud масштабируются под буфер, а Size использует весь ход
ручки и на коротких буферах. При смене длительности захваченный звук и Freeze
сохраняются, паттерн продолжает играть, память в аудиопотоке не выделяется.
Длительность сохраняется отдельно для каждого тембра в пресетах и проектах DAW.
На странице драм-машины есть такая же волна, как у обычного тембра: реальная
история барабанного сигнала выбранной длины с позициями гранул. Она доступна
с Granular и без него, в web и нативных плагинах.
Ручки барабанов обновляются при движении, выбор дорожки или шага выбирает тот же
звук. При включении Granular его ручки и счётчик гранул находятся сразу под
переключателем. LFO/ENV меняют Time Ratio плавно, без ступеней таблицы и без
шага ручки 0,25%. Кнопки **− LFO** / **− ENV** удаляют последний добавленный слот;
первые три остаются на месте.
При первом открытии загружается **glasses5** со своим сэмплом, фейдами, Resynth,
секвенсором и эффектами. Он стоит первым в Factory; Init остаётся доступен.
Сохранённые проекты DAW и последний выбранный User-пресет в web сохраняют приоритет.
У каждой дорожки секвенсора есть три дополнительных назначения и три ручки
силы воздействия на каждом шаге. Ручки эффектов показывают модулированное
значение движущимся указателем, Depth у LFO — числом. Time Ratio у L/C/R Delay
меняется с шагом 0,25% и сглаживается независимо от переключателя Smooth.
Добавлены вкладки независимых тембров, драм-машина DXM-088 с дорожками до 128
шагов, отдельным mod sequencer и переключателем Granular. В каждом тембре можно
добавить до 8 LFO и 8 ENV; доступны параметры осцилляторов и барабанных голосов.
Пресеты и проекты DAW сохраняют всю сцену. Галочки OSC 1–4 находятся сразу под
выбором Main Pitch; дорожки осцилляторов задают относительные −24…+24 полутона.
Pulses и Ratchet управляются ручками от 1 до 8. Параметры передаются отдельно,
без пересборки работающего DSP. macOS: AU и VST3 для Apple silicon и Intel.
Windows: VST3 x64. Закройте DAW перед установкой и затем обновите список плагинов.
Пользовательские пресеты сохраняются при обновлении и удалении.
