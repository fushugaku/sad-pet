SAD PET instrument and SAD PET FX, with native Rust DSP and an embedded editor.

**Resynth** adds a 16-band audio-excited oscillator bank with up to four
independent Rustias layers (64 carriers). It supports Saw, Pulse, Triangle, Sine,
Formant, Noise, Audio In and PCM, plus Wave, Cross, Unison and VPM modes for
analog waves. Each layer has its own controls, tuning, level and pan. Shared
sensitivity, attack, release, threshold and Mix shape the response; Source or
Grains selects the analysis input. PCM uses the selected SAD PET sample and trim.

Resynth runs in the shared Rust engine, natively in AU/VST3 and as WASM on the
web. All settings are stored in presets/projects; new automation IDs are appended
without moving existing controls. Knob updates retain phase, feedback and
envelope histories. Old presets keep Resynth disabled and retain their sound.

The existing Source FX slot, both 12-slot effects racks, RADIAS Waveshaper,
Loop 1 bar, portable presets and MIDI routing remain available.

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
**Resynth**: 16 полос входа управляют огибающими до четырёх слоёв Rustias.
Saw, Pulse, Triangle, Sine, Formant, Noise, Audio In и PCM; аналоговые волны
поддерживают Wave, Cross, Unison и VPM. Отдельные настройки и строй каждого
слоя, общие Sensitivity, Attack/Release, Threshold и Mix. PCM использует
выбранный сэмпл SAD PET. Все настройки сохраняются в пресетах и проекте.
Нативный Rust в плагинах и тот же движок в web; старые пресеты звучат без изменений.
macOS: AU и VST3 для Apple silicon и Intel. Windows: VST3 x64.
Закройте DAW перед установкой, затем обновите список плагинов.
Пользовательские пресеты сохраняются при обновлении и удалении.
