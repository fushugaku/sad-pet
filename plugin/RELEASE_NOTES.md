SAD PET instrument and SAD PET FX, with native Rust DSP and an embedded editor.

**Neuropolis step controls and oscillator pitch routing.** Each step now shows
Pitch/value, a Gate length knob, gate type, Pulses, Ratchet, Slide, Chance and
Velocity together. Double-click Gate to inherit the track's Gate setting.
Advanced overrides and accumulation remain available in their own disclosure.

Pitch handlers resolve the current track by stable ID after telemetry updates.
Editing another step no longer restores an earlier slider's old value. Numeric
edits remain sparse and preserve running clocks, notes and audio histories.

**Main pitch → OSC 1–4** checkboxes select which Resynth layers follow the main
sample-pitch sequence. Each oscillator's own track adds a −24…+24 semitone offset
at an independent speed, length and play order. Contributions are summed before
clamping; tuning, routing, step gates and assignments save in presets and DAW
projects. All processing uses the same native Rust / WASM implementation.

SuperEcho, AlphaVerb and Delay Smooth from 0.9.0 remain included.

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
В секвенсоре Pitch, ручка Gate, Pulses, Ratchet, Slide, Chance и Velocity видны
одновременно. Исправлен откат ранее изменённых ползунков после обновления
состояния. У Main Pitch добавлен выбор OSC 1–4; отдельные дорожки осцилляторов
задают смещение −24…+24 полутона с независимой скоростью. Настройки сохраняются
в пресетах и проектах DAW. SuperEcho, AlphaVerb и Smooth также входят в выпуск.
macOS: AU и VST3 для Apple silicon и Intel. Windows: VST3 x64.
Закройте DAW перед установкой, затем обновите список плагинов.
Пользовательские пресеты сохраняются при обновлении и удалении.
