SAD PET instrument and SAD PET FX, with native Rust DSP and an embedded editor.

**Neuropolis replaces the old sequencer editor.** Add any number of independently
clocked tracks and route each to the main sample pitch, any of the four Resynth
oscillators, notes/envelopes or a realtime parameter. Each track offers all 21
Neuropolis orders, 16 stages, clock multipliers/dividers, length, swing, quantizer,
pulses, Rest/Single/Multi/Hold gates, ratchets, probability, slide, CV ramps and
accumulation. Numeric edits preserve playheads and send only changed values;
adding/removing tracks preserves the other lanes by ID. DAW transport stops only
the sequencer's own notes, keeping live input and keyboard audition available.

The actual pinned Neuropolis Rust core runs natively in AU/VST3 and as the same
Rust DSP in the browser. Tracks, assignments and settings save in presets and
projects. Old chord patterns migrate into note lanes. The track list pages its
readouts without limiting processing; note tracks share eight audio voices.

**OSC 4 in every Resynth slot:** select **Generator → Wavefolder · OSC 4**, then
Triangle, Sine, Soft, Asymmetric or Rectify. Both CTRL 1 (Fold Depth) and CTRL 2
(Bias) are visible, alongside tuning, level and pan. The f64 four-step fold,
smoothing and DC blocker use the Rustias OSC 4 model.

Every layer now has independent **Ring / Sync** (Off, Ring, Sync, Ring + Sync)
and a selectable **Source**, **Grains** or **OSC 1–4** reference. Oscillator taps
precede level/pan, so an unmixed oscillator can drive another one. Cross also
selects Source and Grains independently of the envelope detector. Grains uses
the actual grain output before Dry/Wet. Source/grain sync uses band zero
crossings; oscillator sync uses phase wraps. Existing automation indices and
ranges are retained, and the new controls are appended and saved in projects.

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
Старый редактор секвенсора заменён на **Neuropolis**: любое число дорожек,
назначение на основной питч, OSC 1–4, ноты/ENV или параметр. У каждой дорожки
независимые часы, длина, порядок, свинг, пульсы, Gate, ратчеты, вероятность,
Slide и накопление. Всё сохраняется в пресетах и проектах. Числовые правки
не пересобирают секвенсор; позиции остальных дорожек сохраняются при изменении
их списка. Используется настоящий Rust-core Neuropolis в native и web.
В каждом слоте Resynth доступен **Generator → Wavefolder · OSC 4**: Triangle,
Sine, Soft, Asymmetric и Rectify, с CTRL 1 (Fold Depth) и CTRL 2 (Bias).
Добавлены Ring, Sync и Ring + Sync с независимым источником Source, Grains
или OSC 1–4. Осциллятор с нулевым Level продолжает работать как источник связи.
Cross отдельно выбирает Source/Grains/осциллятор; Grains берётся до Dry/Wet.
Новые настройки сохраняются; индексы и диапазоны старой автоматизации сохранены.
**Resynth**: 16 полос входа управляют огибающими до четырёх слоёв Rustias.
Saw, Pulse, Triangle, Sine, Formant, Noise, Audio In и PCM; аналоговые волны
поддерживают Wave, Cross, Unison и VPM. Отдельные настройки и строй каждого
слоя, общие Sensitivity, Attack/Release, Threshold и Mix. PCM использует
выбранный сэмпл SAD PET. Все настройки сохраняются в пресетах и проекте.
Нативный Rust в плагинах и тот же движок в web; старые пресеты звучат без изменений.
macOS: AU и VST3 для Apple silicon и Intel. Windows: VST3 x64.
Закройте DAW перед установкой, затем обновите список плагинов.
Пользовательские пресеты сохраняются при обновлении и удалении.
