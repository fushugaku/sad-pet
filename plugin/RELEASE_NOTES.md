SAD PET instrument and SAD PET FX, with native Rust DSP and an embedded editor.

**All Rustias filters in both slots.** Choose RADIAS, Ladder LP/HP/BP24,
MS-20 LP/HP12, Pro LP24, SEM, Multi Filter, M/Poly LP24 or Linear LP/HP/BP/Notch
at 12/24 dB. RADIAS Filter 2 LP/BP/HP and Comb are also selectable in either slot.

Drive and Morph are visible with the other filter controls. RADIAS Morph sweeps
LP24 → LP12 → BP → HP → Thru; SEM morphs LP → Notch → HP, and Multi Filter
morphs LP → BP → HP → Notch. Both slots retain their envelope, key tracking and
Single/Serial/Parallel routing. Drive/Morph accept LFO, ENV, matrix, Neuropolis
tracks and DAW automation. Numeric edits preserve filter history without audio
allocations; new automation controls append after the existing indices.

Original fixed-point and f64 Rustias kernels run in the shared Rust engine,
natively in AU/VST3 and as WASM on the web, with host-rate coefficient preparation.
These are the Rustias software models, not measured hardware replicas. Old
presets keep their original SAD PET filter sound. All settings save in presets
and DAW projects. Resynth, Neuropolis, the three FX stages and Loop 1 bar remain
available.

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
Все 18 моделей фильтров Rustias доступны в каждом из двух слотов, включая
M/Poly и Linear LP/HP/BP/Notch 12/24; также доступны RADIAS Filter 2 и Comb.
Drive и Morph принимают модуляцию, Neuropolis и автоматизацию DAW.
Исходные Rust-алгоритмы работают нативно в AU/VST3 и тем же движком в web.
Старые пресеты сохраняют звучание; новые параметры сохраняются в проектах.
macOS: AU и VST3 для Apple silicon и Intel. Windows: VST3 x64.
Закройте DAW перед установкой, затем обновите список плагинов.
Пользовательские пресеты сохраняются при обновлении и удалении.
