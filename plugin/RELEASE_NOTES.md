SAD PET instrument and SAD PET FX, with native Rust DSP and an embedded editor.

**SuperEcho and AlphaVerb in all three FX stages.** SuperEcho ports KekLocator's
native Rust Tape/BBD delay with orbit, filters, pitch/frequency shimmer, dub,
compression, freeze and tempo sync. AlphaVerb ports Dayverb's native Rust reverb,
including damping, EQ, early/late balance, spin, stereo, shimmer, compressor,
gate and freeze. Their controls use the SAD PET editor and save in presets and
DAW projects. Pre/post-filter controls support host automation and Neuropolis.

The same original DSP cores run natively in AU/VST3 and in the web version.
They retain their 48 kHz processing, with bandlimited conversion at other host
rates. Instances are prepared outside the native audio callback; knob updates
preserve their running tails without allocating audio storage. Existing host
parameter indices remain stable.

**Delay Smooth** is now beside the clock selector in L/C/R Delay and Mod Delay.
It allows intermediate note divisions and smoothly glides the delay time under
manual, LFO and ENV control. The display shows the division and milliseconds.

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
Добавлены **SuperEcho** (KekLocator) и **AlphaVerb** (Dayverb) во все три стойки.
Исходный Rust DSP работает нативно в AU/VST3 и тем же движком в web. Доступны
оригинальные звуковые параметры, сохранение пресетов и автоматизация. Поворот
ручек сохраняет хвосты и не выделяет память в аудиопотоке.
**Smooth** появился рядом с Tempo sync в L/C/R Delay и Mod Delay.
macOS: AU и VST3 для Apple silicon и Intel. Windows: VST3 x64.
Закройте DAW перед установкой, затем обновите список плагинов.
Пользовательские пресеты сохраняются при обновлении и удалении.
