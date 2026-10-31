# Third Wave: реализация в SAD PET

31 октября 2026. Объём: TW01–TW13, TW15 и TW16 из [исследования 3rd Wave](WAVETABLE_THIRD_WAVE.md), плюс запрошенный Unison Voice shift. **TW14 MPE исключён пользователем.** Код находится внутри существующего Rack. Это локальная alpha: наличие функций не означает завершённую stable/DAW/device-приёмку.

## Что доступно

| ID | Реализация | Основные файлы |
|---|---|---|
| TW01 | 96 авторских таблиц / 13 паков, 80 инструментов, включая 32 четырёхволновых | `wavetable/factory.rs`, `presets.rs`, `rust/assets/wavetables/` |
| TW02 | Общий 6-point Wave ENV: Time/Level/Curve, seconds/beats, held loop, release, Copy | `envelope.rs`, `dsp/modulation/wave_envelope.rs`, общий ENV UI |
| TW03 | Surf −1…1 и signed depth каждой A–D | `wavetable/motion.rs`, общий modulation frame |
| TW04 | Smooth/Step кадров, 2 ms переход каждого голоса | `dsp/wavetable.rs` |
| TW05 | Per-note Key → Waves относительно C4 | тот же Runtime, до clamp |
| TW06 | Retrigger/free phase, общий LFO Mono/Poly с per-note состояниями | `preset::Lfo`, `dsp/modulation.rs`, Rack voice routing |
| TW07 | Любая A–D → любая A–D: through-zero linear FM или PM, self/feedback | `dsp/wavetable/interaction.rs`, `interaction/routing.rs` |
| TW08 | Hard sync A–D, независимый источник A–D, slave ratio 1…8 | тот же interaction path |
| TW09 | Общий Capture input/output до 2 s, Maker Spectral/Pitch cycles | `capture.rs`, `wavetable/maker.rs`, WASM chunks/utility worker |
| TW10 | Порядок/повторы/trim/reverse/align, локальные фаза и H1–H16 | `wavetable/edit.rs`, `web/wavetable-tools.js` |
| TW11 | Offline bit depth, phase steps, harmonic limit/tilt | тот же editor; Clean не изменён |
| TW12 | 6 рецептов пары общих Filters | `filter_presets.rs`, Filters → Tone |
| TW13 | Key/velocity splits и layers для входящего MIDI | `timbres::Mix`, native/WASM routing, MIDI range |
| TW15 | 8 factory motions, до 16 пользовательских в инструменте, update/delete/Undo | `sequence/motion.rs`, общий Neuropolis Program/UI |
| TW16 | XY A/B/C/D с bilinear весами без auto-gain | `wavetable/motion.rs`, performance controls |
| Дополнение | Unison Voice shift −1…1: соседние голоса читают разные Position | Controls + Runtime, `wt_a_voice_shift` … `wt_d_voice_shift` |

Пути Rust указаны относительно `rust/src/`. Общие Filters, FX, MIDI parser и sequencer не дублируются внутри WT. Сторонние таблицы/записи и закрытые алгоритмы Groove Synthesis не копировались.

## Position, Unison и XY

Для каждой ноты складываются base Position, Surf × depth, Key to Waves, local Position LFO и ENV. Затем для голоса унисона `j` добавляется `(j − (N − 1) / 2) × voice_shift`, где `j = 0…N−1`. Clamp 0…1 выполняется после всех смещений, без wrap.

**Voice shift — шаг между соседними голосами**, доля всей таблицы. Position 50%, N=3 и Shift=20% дают 30%, 50%, 70%. Отрицательное значение меняет порядок; N=1 всегда даёт нулевой сдвиг. Shift сглаживается за 2 ms, расположение при смене числа голосов — за 5 ms. Default 0 сохраняет прежний звук.

Step округляет позицию каждого голоса до кадра и достигает её за 2 ms. Качественная интерполяция фазы и bandlimiting остаются включены. Telemetry показывает основную Position и диапазон реальных позиций нот/унисона.

Key to Waves=1 означает полный диапазон Position за пять октав от C4; отрицательное значение разворачивает движение. Free-running сохраняет фазовый ход через переиспользование и простой voice slots; retrigger остаётся default.

XY умножает Level на веса с суммой 1: A/B сверху, C/D снизу. У двух волн действует только X; у трёх C занимает нижнюю сторону; одна получает вес 1. Auto-gain нет. Surf, X/Y и Voice shift доступны host automation, macros/matrix, MIDI learn, Neuropolis и Control.

## Wave ENV и poly LFO

Wave — отдельный enable поверх прежнего ENV Mode: диапазон выпущенного Mode 0…6 не меняется. Шесть сегментов имеют Time 0…32, Level 0…1, Curve −1…1. В beats единица — четверть. Held loop start/end используют первые пять точек; шестая доступна для release. Note-off начинается с фактического уровня. Ненулевой последний уровень получает 5 ms хвост в ноль. Изменение времени/темпа сохраняет прогресс; редактирование целевого уровня использует slew.

Copy to selected A–D создаёт независимые ENV в свободных слотах до общего лимита 8, назначает Position ENV/depth и сохраняет AMP и существующие маршруты. При нехватке места показывает ошибку и ничего не перезаписывает.

Poly LFO хранит отдельные состояния звучащих нот. Local Position LFO получает per-note значения; общие Rack destinations, которым нужен один параметр, продолжают получать mono stream. Reset Free/Note/Tempo/Sequencer остаётся общей настройкой. Это ограниченный poly scope, не полифония любого эффекта.

## FM/PM и sync

У каждой A–D есть независимые FM source и Sync source A–D. FM и PM доступны всем четырём волнам, включая self-FM, встречные связи и кольца. Новая сеть читает фактический сигнал оператора после его FM/sync, до Level/pan/XY, с нормализацией unison. Power плавно включает/выключает оператор. Одно внутреннее запаздывание делает feedback причинным. Sync передаёт естественные пересечения границы цикла, без повторного распространения входящего reset. При выборе себя Sync использует собственный базовый clock. Подробнее — [маршрутизация и оптимизация](WAVETABLE_OPTIMIZATION.md).

Старые B→A и D→C с собственным clock сохраняют прежний путь и источник до sync. Индекс linear FM задаёт отклонение `index × source Hz × source signal`; частота может проходить через ноль. Индекс PM — радианы, диапазон обоих 0…4. Voice shift действует и на слышимый источник, и на операторные чтения.

FM/PM работают в 8× пути. Sync использует 32×, дробное время reset и value/slope BLEP/BLAMP corrections, затем 63-tap prefilter до 8× и 641-tap Kaiser lowpass до 1×, DC blocker 5 Hz. Спектральный бюджет зависит от индекса и скорости фазы: верхние ноты/большой индекс намеренно теряют часть яркости. Неограниченное сохранение sidebands не обещается.

В совместимом парном пути пара фильтруется вместе, включая слышимый source; новая сеть складывает четыре оператора перед общим decimator ноты. Внутренний групповой delay — около 40 отсчётов на 48 kHz, плюс prefilter для sync; это не host latency всего инструмента. Переключение пути имеет 20 ms crossfade. При выключенных взаимодействиях используется прежний lookup с продолжающейся фазой.

## Capture, Maker и редактор

Capture — один заранее подготовленный stereo buffer на общую сцену, 0.05…2 s. Native читает готовый take на control thread. AudioWorklet передаёт до 32 KiB PCM за сообщение; utility worker создаёт WAV/base64 и Asset. Аудиопоток не формирует большой JSON и не выделяет/освобождает память для capture.

UI Capture пишет выход всей сцены; Solo изолирует Rack. Core также поддерживает input при подключённом входе. Запись сама не меняет патч. Maker принимает WAV до 16 MiB и выбранный фрагмент до 10 s, 1…256 кадров, Left/Right/Mix:

- Spectral snapshots снимает Hann FFT magnitudes с согласованными фазами: периодическая текстура без исходного ритма/высоты.
- Pitch cycles ищет период и выравнивает восходящие нулевые переходы; root note помогает на сложном материале. Шум без надёжного периода получает ошибку с предложением Spectral.

Редактор хранит оригинал; preview — временный Asset. Apply проверяет generation/identity, фиксирует содержимое и выбирает его обычным Rack edit. Cancel и устаревшие async результаты не заменяют звук. Frame order/repeats/trim, phase/H1–H16, align/normalize и Character выполняются offline. Новый звук получает новый hash; factory original остаётся доступен. User assets сохраняются на диск/IndexedDB и включаются в scene/.sadinst.

## Общие сценические функции

Filters → Tone применяет шесть рецептов к существующей паре Filters. Это не новая модель SSI2140 и не эмуляция аналогового тракта 3rd Wave.

MIDI range задаёт channel/key/velocity для входящих NoteOn. NoteOff, velocity-zero и sustain не отсекаются диапазоном. Изменение диапазона освобождает held notes без остановки transport/Drums. Экранная и компьютерная клавиатура прослушивают активный timbre напрямую; split предназначен для входящего MIDI.

Motion patterns сохраняют declarative Neuropolis tracks с инструментом через существующий .sadinst. Восемь factory motions покрывают Surf, противоположные Position, XY и Filters. Применение включает Program, но не запускает общий transport. Update/Delete/Undo ограничены текущим инструментом. Для ID есть fallback без secure-context randomUUID, нужный native editor.

## Совместимость и realtime

Первые 4566 host specs закреплены `optimization_parameters_before.json.gz`; предыдущие prefix fixtures тоже проверяются. В конец добавлены восемь источников A–D и FM mode/index для B/D: всего 4578 параметров. IDs Voice shift и старый ENV Mode сохранены. Прежние 32 таблицы/48 инструментов, затем расширение до 96/80 сохраняют IDs/hashes.

Schema остаётся 2 для A–D; engine revisions 1/2 мигрируют в 3, будущая неизвестная ревизия отвергается. Отсутствующие источники восстанавливают пары A←B, B←A, C←D, D←C и собственный sync clock. Другие отсутствующие поля дают нейтральные defaults: Wave disabled, Surf/Key/LFO/Shift=0, Step/FM/sync/XY/poly=false, phase retrigger. Старый патч не включает новую сеть.

Хранилища ENV/LFO, unison, interaction filters и capture выделяются вне callback. No-heap проверки охватывают notes/render, scalar/unison/topology edits, capture writer, preset exchange/retirement. Незавершённый общий memory preflight этим не закрыт.

## Проверки

Результаты финальной сборки и отчёты подтверждают указанный масштаб:

**После оптимизации: 176 core lib + 19 WT integration + 60 native Rust + 2 C++ host tests проходят.** Текущие хеши, 30 native и 108 WASM сравнений звука, свободные источники и CPU до/после — в [отчёте оптимизации](WAVETABLE_OPTIMIZATION.md). Clippy WASM с `-D warnings`, JS syntax и `git diff --check` проходят. Первоначальная Third Wave приёмка (174/16 тестов, baseline parity 24) сохранена в [thirdwave-verification.json](../reports/wavetable/thirdwave-verification.json); её [advanced parity](../reports/wavetable/thirdwave-wasm-parity.json) — исторический результат предыдущей сборки.

- Core lib и WT integration: prefix/recall, per-note ENV/LFO, Surf/Step/Key/XY, VCA once, Rack/history, interactions, capture, splits, filter recipes/motions. Voice shift проверяется независимыми Fourier projections трёх различных кадров; N=1 bit-identical при любом Shift.
- Native plugin: полный Rust suite; capture → Maker → editor → выбор → save/restore → звук без audio heap work. Дисковые библиотеки этих тестов изолированы во временных папках.
- Реальный scalar/SIMD WASM: staging/cache-only worklet, catalog, четыре волны, no memory.grow на edits/capture, portable assets/новые настройки. Отдельный event/block parity профиль включает Wave ENV, Voice shift и FM→PM→sync.
- Native VST3/AU: локальная сборка и C++ instrument/FX host tests. Это не проверка сторонних DAW/Windows.
- Браузер: fresh startup без `.label` exception; Filters Tone/Silk; Capture 2 s → 16 frames → Apply; редактор duplicate/move/trim/phase/harmonics/Character, Apply/Cancel. Desktop/mobile screenshots — `reports/wavetable/`.
- [162 interaction cases](../reports/wavetable/thirdwave-interactions.json): 44.1/48/96 kHz, sine/saw/complex, шесть высот до MIDI 129, FM/PM/sync. Worst nonharmonic energy около −81.1 dBr при критерии −60. Это static integer-ratio grid, **не dynamic modulation oracle**.

CPU до оптимизации: [FM](../reports/wavetable/thirdwave-fm-cpu.json) и [sync](../reports/wavetable/thirdwave-sync-cpu.json). Актуальное сравнение на одинаковых патчах, U4/U8, thread CPU/wall и динамическая проверка звука находятся в [отчёте оптимизации](WAVETABLE_OPTIMIZATION.md). Interaction modes дороже обычного lookup; высокие полифония/unison могут превышать deadline. In-process p99 не доказывает отсутствие DAW/device underruns.

Измерения шли при другой локальной работе и открытых браузерных сессиях: contention/preemption не изолированы, в stress-профилях есть большие задержки. По этим данным нельзя назначить гарантированный лимит голосов. CPU gate остаётся открытым; для beta нужны оптимизация тяжёлых interaction paths и изолированные измерения на целевых устройствах.

## Выпуск

1. **Локальная alpha — текущий результат.** Все функции доступны для проверки; IDs/Clean defaults сохранены. В changelog разделить контент, общие ENV/LFO/Timbres и WT interactions/tools. Не объявлять stable performance по одному компьютеру.
2. **Beta.** Закрыть CPU/device matrix, зафиксировать поддерживаемые interaction profiles, замерить суммарный memory peak при preview/restore/старых банках. Выполнить dynamic spectral oracle, pitch/phase native↔WASM corpus, стресс переключений и 30 min host run. Проверить .sadinst/scene migration и downgrade policy. Провести музыкальную приёмку контента и motion defaults.
3. **RC/stable.** Закрыть [базовые PRD/release gates](WAVETABLE_IMPLEMENTATION.md), проверить физические mobile devices, Windows и выбранные DAW, packaging/signing/rollback. Последовательность: web alpha → desktop/plugin beta → общий RC. Хранить прошлые artifacts и не менять звук factory ID :v1.

Открыты: суммарный 128 MiB memory budget и peak preflight (64 MiB Pool/arena сами его не доказывают), стоимость всех 96 таблиц в каждом WASM module, dynamic oracle и device/DAW matrix. Модуль около 49 MB включает примерно 22.55 MB compressed factory data; это долг загрузки. MPE и новая аналоговая filter model в выпуск не входят.
