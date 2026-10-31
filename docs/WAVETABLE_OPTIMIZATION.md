# Wavetable: FM, sync и произвольные связи

31 октября 2026. Изменения внутри существующего Rack, engine revision 3. Каждая из четырёх волн имеет собственные FM source и Sync source A–D; FM/PM доступны также B и D. Поддерживаются self-FM, встречные связи и кольца. Актуальная WASM U4 проверка сравнивает два shipping snapshots engine revision 3 на одинаковом патче. Замеры предыдущего этапа и миграция старых пар приведены ниже.

## Подготовка web alpha к деплою

Компилятор закреплён на Rust 1.97.0 в `rust-toolchain.toml` и workflows: это версия, которой собраны измеренные snapshots. Переход на новую Rust/V8 версию требует отдельного сравнения CPU и PCM.

Релиз собран отдельно от незавершённых изменений Control/MIDI в рабочей копии. Исходники WT DSP совпадают с измеренным профилем ниже; состав полного WASM-модуля отличается. [Проверка эквивалентности](../reports/wavetable/web-alpha-release/audio-equivalence.json): все 21 динамическое сравнение релизного SIMD с измеренным оптимизированным snapshot совпали побитово, включая первые отсчёты и live transitions. Это проверка переноса в релиз; CPU-цифры ниже относятся к исходному измеренному snapshot. [Релизная верификация](../reports/wavetable/web-alpha-release/verification.json) содержит hashes сборки и результаты проверок. Web alpha не закрывает оставшиеся stable 1.0 device/DAW gates. Перебор sample-библиотеки занимает 134.25 MiB WASM high-water при 42.56 MiB после старта; прежний абсолютный mobile-бюджет 128 MiB не пройден. Регрессия теперь отдельно ограничивает startup (<48 MiB) и рост export scratch (<96 MiB), сохраняя этот результат и незакрытый mobile gate в отчёте.

## Актуальный WASM-профиль: 4 ноты × 4 волны × U4

31.10.2026, Apple M5 Max / arm64, 48 kHz / 128 frames. Четыре разные комплексные таблицы, detune 7.3 cents, Voice shift 0.06, случайные начальные фазы и четыре активных unison lane у каждой волны. В основном сценарии все четыре linear FM и все четыре sync включены одновременно; источники образуют два независимых кольца. Это **64 осцилляторных голоса** внутри существующего Rack. [Полный патч](../reports/wavetable/wasm-unison4/target-preset.json).

### CPU до/после

[AB/BA и все повторы](../reports/wavetable/wasm-unison4/cpu/comparison.json). По два независимых прогона на версию и сценарий, 1 s audio warmup + 2 s измерения. Указана медиана percentile отдельных прогонов, по 1500 блоков на версию/сценарий. Node 24.19 / V8 13.6, shipping SIMD WASM без специальных V8 flags; CPU потока user+system через [threadCpuUsage](https://nodejs.org/api/process.html#processthreadcpuusagepreviousvalue), wall отдельно. Подготовка, компиляция, control requests и проверка PCM исключены из измеряемого интервала. Свои сборки/тесты не выполнялись параллельно замеру; сторонняя работа пользователя продолжала работать.

| Сценарий, всегда 4 ноты / 4 волны / U4 | Thread CPU p50 до → после, ms | Снижение | Node wall p99 после, ms |
|---|---:|---:|---:|
| FM + sync, кольца | 4.010 → 2.401 | 40.1% | 3.030 |
| Прежние A←B / C←D + self-sync | 3.471 → 2.592 | 25.3% | 3.273 |
| FM + sync, кольца; четыре Position движутся | 4.146 → 2.593 | 37.5% | 3.392 |

Бюджет 128 frames / 48 kHz — 2.667 ms. **Node p99 пока выше этого бюджета**; превышения и максимумы сохранены в JSON. Это замер обычного вызывающего потока Node, не доказательство отсутствия пропусков на аудиоустройстве. Отдельная проверка ниже использует production AudioWorklet и browser playout; их результаты не подменяют Node p99.

### Production AudioWorklet: реальный browser playout

Chrome 156 (in-app browser), Apple M5 Max, default audio destination, 48 kHz / 128-frame quantum, latencyHint `interactive`; base latency 5.333 ms, output latency 16.000 ms. Используется production `worklet.mjs` со staging prepared banks; DSP не подменён тестовой реализацией. Для каждой версии отдельный AudioContext и тот же target preset. После 5 s прогрева фиксируется delta `AudioContext.playbackStats`; raw start/end также сохранены. Собственные builds/tests/CPU benchmarks во время playout не выполнялись.

| Версия | Wall time, s | Audio time, s | Underrun events за интервал | Processor errors |
|---|---:|---:|---:|---:|
| [До](../reports/wavetable/wasm-unison4/realtime-before.json) | 30.07 | 20.50 | 5637 | 0 |
| [После](../reports/wavetable/wasm-unison4/realtime-after.json) | 120.28 | 120.28 | **0** | **0** |
| [После, прежние пары](../reports/wavetable/wasm-unison4/realtime-paired-after.json) | 30.07 | 30.07 | **0** | **0** |

У финальной сборки и в первых 5 s значение `underrunEvents=0`. Hash загруженного WASM совпадает с after в CPU/null-test отчётах: `732c35f4a39afe10159f63b3ab4e1d34cdf63eddb6d4da477ca1afff6aa0e0fd`. Целевой профиль прошёл этот длительный browser run. [Снимок](../reports/wavetable/wasm-unison4/realtime.png), [итоговая проверка и hashes](../reports/wavetable/wasm-unison4/verification.json). Это измеренный reference desktop, а не закрытие всей mobile/Windows/DAW performance matrix. В Node 24 другой V8 и вызывающий поток; его p99 остаётся выше quantum budget и явно приведён выше.

### Изменения DSP

- SIMD128 считает четыре unison lane вместе: f64 фазы и master clocks, полиномиальные чтения, восемь полюсов ограничения FM и стерео FIR. Для старых пар есть свой SIMD-путь с прежним tap до source sync.
- Коэффициенты в текущей ячейке хранятся между выходными отсчётами. Ключ включает полный hash банка, адреса frame views, Position/mip blend и сетку. Кэш хранит только численные данные/ключи; чтения всё ещё ограничены живым immutable borrow банка. Изменение таблицы или позиции инвалидирует нужные lanes.
- Cell/fraction общих сеток вычисляются один раз. Слышимые соседние mips одинаковой длины можно объединить в один полином через f64 coefficient mixing. FM taps сохраняют исходный порядок mip arithmetic. Это даёт небольшую разницу округления в выходном PCM.
- Расчёт четырёх узлов развёрнут, полный FM/sync-профиль специализируется до внутреннего цикла, mip selection и деление FM frequency на internal sample rate кешируются/выносятся из него. Циклы feedback по-прежнему читают снимок предыдущего внутреннего отсчёта.
- Сохранены 8× FM/PM, 32× sync, sinc16/degree-7 kernel, 63/641-tap decimators, fractional BLEP/BLAMP и DC blocker. Смена U, неготовые/отсутствующие views, crossfade банков и большие sinc mips используют общий путь. Новых quality modes нет.

Engine/schema и формат prepared banks остаются 3/2 и compiler/layout/kernel 2/2/1. Дополнительные cache state принадлежат голосу Rack. SIMD не требует relaxed-SIMD или новых браузерных возможностей сверх прежнего SIMD128; scalar fallback остаётся доступен.

### Звук и регрессии

[21 полное динамическое сравнение](../reports/wavetable/wasm-unison4/after/audio.json): 44.1/48/96 kHz, FM/sync rings, PM, FM-only, прежние пары, self-FM/self-sync, sync без FM и краткий переход ring → fan-out → ring. Включены первые отсчёты нот, note-off/retrigger, moving Position/Voice shift/index/sync ratio, U3/U8/U4, замена таблицы и возврат во время звучания. Прогрев и переходы не вырезаются; время и уровень не подгоняются.

Худшая ошибка: **-124.34 dBr**, максимум **9.78e-09** при порогах −100 dBr / 1×10⁻⁶. Результат не является битово идентичным из-за округления coefficient mixing и f64 step arithmetic.

[A/B WAV](../reports/wavetable/wasm-unison4/audio/listening.json): [до](../reports/wavetable/wasm-unison4/audio/before.wav) / [после](../reports/wavetable/wasm-unison4/audio/after.wav), 3.2 s с движением всех четырёх Position и release. Для прослушивания оба файла умножены на один фиксированный gain 16; clipping отсутствует. Сравнение исходного PCM: −129.11 dBr, max 3.73×10⁻⁹.

Проверены 176 core unit + 19 WT integration tests; shipping SIMD staging/recall/Maker/capture/no-memory-grow suite; [3 scalar/SIMD/host-block сравнения](../reports/wavetable/wasm-unison4/parity.json); WASM SIMD clippy `-D warnings`. Независимый static alias oracle предыдущего этапа остаётся отдельной проверкой; текущий null test не выдаётся за новый динамический alias oracle.

Воспроизведение на этом workspace:

```sh
python3 wasm/build.py --simd --output /tmp/sadpet-wt-web
node wasm/bench-wavetable-ab.mjs /tmp/sadpet-wasm-u4-before/sad_pet.simd.wasm \
  /tmp/sadpet-wt-web reports/wavetable/wasm-unison4/cpu
node wasm/test-wavetable-unison4.mjs /tmp/sadpet-wasm-u4-before/sad_pet.simd.wasm \
  /tmp/sadpet-wt-web/sad_pet.simd.wasm /tmp/sadpet-wt-web reports/wavetable/wasm-unison4/after
```

Исходная shipping сборка сохранена перед изменениями в `/tmp/sadpet-wasm-u4-before`; её hash закреплён в [manifest](../reports/wavetable/wasm-unison4/before-manifest.json). Это сравнение snapshots рабочей копии. Для browser acceptance положить `wasm/verify-wavetable-realtime.html`, target preset и `baseline/{core.mjs,worklet.mjs,sad_pet.simd.wasm}` рядом с production bundle и открыть localhost. Счётчики [AudioPlaybackStats](https://webaudio.github.io/web-audio-api/#AudioPlaybackStats) относятся к настоящему audio destination; Node-код их не эмулирует.

## Как пользоваться

В секции волны **FM / Sync** выбрать Modulation: Linear FM или Phase modulation, затем FM source и Index. Hard sync включается отдельно и использует свой Sync source. Оба источника могут различаться. Отсутствующие волны недоступны в меню; добавление выполняется обычной кнопкой + Wave.

FM source — нормализованная сумма unison источника до Level/pan/XY. Поэтому Level=0 позволяет использовать волну как неслышимый оператор; выключение Power убирает её вклад. Voice shift применяется к каждому операторному чтению. Linear FM допускает отрицательный шаг фазы; индекс задаёт отклонение `index × source Hz × source signal`. Индекс PM задаётся в радианах.

В новой сети берётся сигнал после FM/sync источника. Все узлы читают снимок предыдущего внутреннего отсчёта: перестановка A/B/C/D не меняет установившийся результат сети. Задержка связи на 48 kHz — 2.60 µs при 8× и 0.65 µs при 32×; это не полная latency инструмента. Sync берёт естественное пересечение границы цикла источника. Входящий reset не становится новым тактовым импульсом: кольцо не должно бесконечно распространять собственный reset. Unison-полосы clock сопоставляются по `voice index % source voice count`.

При Sync source, равном самой волне, остаётся собственный базовый clock, а Sync ratio меняет скорость slave. Это прежний self-sync, тогда как self-FM является настоящей обратной связью. Источник FM с собственной FM/sync получает ограничение полосы перед feedback tap; глубина и верхние ноты могут уменьшать яркость.

Патчи с прежними A←B / C←D, FM выключенным у B/D и собственными sync clocks сохраняют парный renderer, включая прежний tap до sync. Переход на новую сеть и обратно занимает 20 ms. Возврат к парной топологии завершает работу сети и у уже звучащих нот. Legacy и новая сеть могут отличаться в этой переходной области; установившаяся сеть проверяется отдельно от неё. Index=0 и недоступный источник оставляют обычное воспроизведение с продолжающейся фазой.

## Что ускорено

- Подготовка frame/mip views и постоянных коэффициентов вынесена из внутреннего цикла oversampling. Убраны повторные деления на sample rate, вычисления gain и ненужные FM tap reads в режиме sync-only.
- Чтение сухого пути выполняется только во время перехода. Неактивные source-unison lanes продвигают clock один раз на выходной отсчёт.
- В новой сети слышимый сигнал и FM tap делят один расчёт фазы, clock и sync-событий. Редкие дополнительные чтения для BLEP/BLAMP вынесены из обычного цикла; сами corrections сохранены.
- Для небольших mips подготовлены полиномы с восемью коэффициентами на ячейку. Они аппроксимируют **тот же sinc16 kernel** по восьми Chebyshev-Lobatto узлам; не заменяют его обычным cubic lookup. Коэффициенты смешиваются по Position и кешируются на текущей ячейке. Большие mips используют полный sinc16 с SIMD reduction.
- В native ARM64 стерео FIR вычисляется NEON с теми же четырьмя цепочками суммирования. Сохранены 8× FM/PM, 32× sync, дробное время reset, BLEP/BLAMP, 63/641 taps и DC blocker. Новая сеть складывает четыре узла перед общим decimator одной ноты.

Clean `Bank::read`, raw assets, factory IDs и content hashes сохранены. Подготовка коэффициентов выполняется на control/worker thread; в render нет FFT, компиляции банков, allocation/deallocation или memory.grow.

## Предыдущий этап: CPU до и после

Финальная сборка с проверкой отсутствующих/выключенных источников измерена повторно: [native](../reports/wavetable/optimization/cpu-before-after.json), [WASM U2](../reports/wavetable/optimization/wasm-cpu-current.json), [полная сеть WASM U2](../reports/wavetable/optimization/wasm-routed-current.json).

| Native: режим | Нот | Unison | CPU p50 до → после, ms | Снижение | Wall p99 после, ms |
|---|---:|---:|---:|---:|---:|
| fm | 4 | 4 | 3.462 → 1.426 | 58.8% | 2.060 |
| sync | 4 | 4 | 11.492 → 3.121 | 72.8% | 3.873 |
| mixed | 4 | 2 | 7.047 → 2.427 | 65.6% | 3.386 |

Новая полная сеть, native after only:

- 4 notes × 4 waves × U2: CPU p50 2.074 ms; wall p99 2.502 ms.

| WASM SIMD в Node/V8 | Нот | Unison | Wall p50 до → после, ms | Wall p99 после, ms |
|---|---:|---:|---:|---:|
| fm | 4 | 2 | 2.764 → 2.213 | 3.321 |
| mixed | 3 | 2 | 7.296 → 3.083 | 3.870 |

Новая полная сеть WASM (after only), 2 notes / 4 waves / U2: wall p50 2.397 ms, p99 3.162 ms.

Бюджет 128/48 kHz — 2.667 ms. Финальная native проверка: два последовательных AB/BA прогона по 1 s; WASM — по 0.75 s. Это повторная проверка представительных профилей после правки выбора источника; расширенные stress-результаты ниже относятся к предыдущей сборке того же DSP kernel. На том этапе WASM performance gate оставался открытым; актуальная проверка U4 приведена в начале документа.

### Расширенная матрица до последней проверки источников

Арифметика FM/sync и фильтры в этих более ранних замерах те же; последняя правка касается выбора пути при отсутствующем или выключенном источнике. Хеши измеренных binaries сохранены в JSON.

Источник расширенной матрицы — [cpu-kernel-before-after.json](../reports/wavetable/optimization/cpu-kernel-before-after.json). Shared production Engine, четыре разных комплексных таблицы, detune и Voice shift, 48 kHz, блок 128. Deadline блока — 2.667 ms. Подготовка assets исключена. Прогоны идут последовательно, порядок AB/BA чередуется; таблица содержит медиану p50/p99 отдельных прогонов.

| Режим | Нот | Unison | CPU p50 до, ms | CPU p50 после, ms | Снижение CPU | Wall p99 после, ms | < 2.667 ms |
|---|---:|---:|---:|---:|---:|---:|---|
| off | 8 | 4 | 0.295 | 0.313 | -5.9% | 0.531 | да |
| fm | 4 | 4 | 1.958 | 0.935 | 52.2% | 1.283 | да |
| fm | 8 | 4 | 4.117 | 2.083 | 49.4% | 2.617 | да |
| fm | 16 | 2 | 4.947 | 3.414 | 31.0% | 4.190 | нет |
| fm | 8 | 8 | 7.633 | 3.165 | 58.5% | 4.046 | нет |
| pm | 8 | 4 | 4.008 | 2.163 | 46.0% | 2.691 | нет |
| sync | 4 | 4 | 6.877 | 2.042 | 70.3% | 2.533 | да |
| sync | 8 | 4 | 13.703 | 4.043 | 70.5% | 5.200 | нет |
| mixed | 4 | 4 | 7.140 | 2.324 | 67.5% | 3.011 | нет |
| mixed | 8 | 4 | 13.940 | 4.860 | 65.1% | 5.975 | нет |

Для обычного lookup (off) добавилось около 0.018 ms на блок в этом профиле; ускорение относится к interaction paths. В таблице это отражено отрицательным снижением.

Дополнение [FM+sync с U2](../reports/wavetable/optimization/native-unison2-before-after.json): четыре ноты / четыре волны, CPU p50 3.910 → 1.787 ms; wall p99 2.509 ms. Это два последовательных AB/BA прогона по 1 s. Новая четырёхоператорная FM+sync сеть с U2 на четырёх нотах: CPU p50 1.453 ms, wall p99 2.038 ms. У новой сети нет эквивалентного патча в исходном парном движке.

Полная новая сеть, after only:

| Режим | Нот | Unison | Блок | CPU p50, ms | Wall p99, ms | Бюджет блока, ms |
|---|---:|---:|---:|---:|---:|---:|
| routed-fm | 4 | 4 | 128 | 0.996 | 1.312 | 2.667 |
| routed-fm | 8 | 4 | 128 | 1.846 | 2.475 | 2.667 |
| routed-pm | 4 | 4 | 128 | 1.118 | 1.515 | 2.667 |
| routed-pm | 8 | 4 | 128 | 2.381 | 2.907 | 2.667 |
| routed-sync | 4 | 4 | 128 | 2.028 | 2.542 | 2.667 |
| routed-sync | 8 | 4 | 128 | 4.447 | 5.064 | 2.667 |
| routed-mixed | 4 | 4 | 128 | 2.817 | 3.506 | 2.667 |
| routed-mixed | 8 | 4 | 128 | 5.625 | 6.864 | 2.667 |
| routed-sync | 4 | 4 | 256 | 4.461 | 5.260 | 5.333 |
| routed-mixed | 4 | 4 | 256 | 5.529 | 6.919 | 5.333 |

Shipping SIMD WASM измерен отдельно в Node/V8 через `Core.process(128)` с JS boundary/output view. Это **не browser AudioWorklet/device test**. [U4](../reports/wavetable/optimization/wasm-cpu-before-after.json), [U2](../reports/wavetable/optimization/wasm-unison2-before-after.json): по два последовательных AB/BA прогона, 0.5 s audio warmup и 0.75 s измерения; подготовка и валидация сигнала исключены. Приведена медиана percentile каждого прогона.

| WASM режим | Нот | Unison | Wall p50 до, ms | Wall p50 после, ms | Снижение | Wall p99 после, ms | < 2.667 ms |
|---|---:|---:|---:|---:|---:|---:|---|
| off | 8 | 4 | 0.490 | 0.505 | -3.1% | 0.817 | да |
| fm | 4 | 4 | 2.933 | 1.620 | 44.8% | 3.067 | нет |
| fm | 8 | 4 | 5.772 | 3.112 | 46.1% | 4.115 | нет |
| sync | 4 | 4 | 9.620 | 3.104 | 67.7% | 3.938 | нет |
| mixed | 4 | 4 | 9.798 | 3.573 | 63.5% | 4.588 | нет |
| fm | 4 | 2 | 1.600 | 1.130 | 29.4% | 1.746 | да |
| sync | 4 | 2 | 5.354 | 2.047 | 61.8% | 2.778 | нет |
| mixed | 4 | 2 | 5.464 | 2.391 | 56.2% | 3.096 | нет |
| mixed | 3 | 2 | 4.064 | 1.753 | 56.9% | 2.413 | да |

[Новая четырёхоператорная сеть в WASM, after only](../reports/wavetable/optimization/wasm-routed-unison2.json): все четыре FM и sync одновременно, четыре комплексные таблицы, U2.

| Нот | Wall p50, ms | Wall p99, ms | < 2.667 ms |
|---:|---:|---:|---|
| 2 | 1.334 | 1.926 | да |
| 3 | 2.124 | 2.770 | нет |
| 4 | 2.748 | 3.702 | нет |

В этом замере native укладывается по p99 для FM 8 notes/U4, sync 4 notes/U4 и совместного FM+sync 4 notes/U2. WASM укладывается для обычного FM 4 notes/U2, прежнего смешанного пути 3 notes/U2 и новой полной сети 2 notes/U2. Более тяжёлые профили превышают бюджет: общая задача высокой полифонии для browser/device beta остаётся открытой.

Thread CPU исключает время вытеснения потока. Wall включает конкуренцию с другой локальной работой; частота CPU и тепловой режим не изолированы. В отчёте сохранены оба вида времени, все повторы, стресс U8/16 notes и новые четырёхоператорные маршруты. Это границы измеренного профиля на одном компьютере, без гарантии отсутствия underruns в DAW или на мобильном устройстве. Высокая полифония с одновременными FM и sync остаётся тяжёлым случаем.

Воспроизведение сравнения:

```sh
cargo build -p sad-pet --release --example wavetable_opt
python3 rust/scripts/bench_wavetable_interactions.py BEFORE_BINARY \
  target/release/examples/wavetable_opt reports/wavetable/optimization/cpu-before-after.json
```

BEFORE_BINARY — сохранённый перед изменениями `wavetable_opt`, собранный с исходным DSP. Он имеет тот же сценарий и интерфейс CLI. Хеши обоих executable указаны в отчёте; это сравнение рабочей копии, а не двух git commits. Патчи сохранены в [benchmark-presets.json](../reports/wavetable/optimization/benchmark-presets.json).

## Проверка звука

[30 динамических native сравнений](../reports/wavetable/optimization/audio-difference.json): 44.1/48/96 kHz, 1/8 нот, четыре комплексных таблицы, U4 с переходами к U2/U8. Двигаются Position, Voice shift, индекс и sync ratio; есть note-off/retrigger и FM→PM→sync/off. Сигналы сравниваются отсчёт в отсчёт без подгонки времени/уровня. Максимальная разница 1.02×10⁻⁷, худшая ошибка RMS −111.10 dBr при gates 10⁻⁵ / −95 dBr. Это численная регрессия, а не заявление о битовой идентичности или проведённом listening test.

[Статический старый путь](../reports/wavetable/optimization/after-interactions.json) и [новый D→B / внешний clock](../reports/wavetable/optimization/routed-interactions.json): по 162 случая FM/PM/sync, sine/saw/complex, 44.1/48/96 kHz и MIDI 45…129. Энергия вне ожидаемой гармонической сетки должна быть ниже −60 dBr. Для внешнего sync, чей основной тон выше Nyquist, нет ожидаемых слышимых гармоник: проверяется абсолютный stopband RMS ≤−90 dBFS. Обратная связь и движущаяся модуляция не входят в этот независимый FFT oracle.

Отдельные [WASM block/scalar/SIMD проверки](../reports/wavetable/optimization/wasm-parity.json) включают произвольные источники и feedback. Сравнение shipping WASM до/после находится в [wasm-audio-difference.json](../reports/wavetable/optimization/wasm-audio-difference.json). Native и WASM здесь имеют отдельные before/after references: общий межплатформенный phase/voice-generation oracle остаётся отдельной задачей.

Ещё [48 сравнений установившейся новой сети](../reports/wavetable/optimization/routing-audio-difference.json) охватывают четырёхоператорную обратную связь, внешний clock ring, U4→U2/U8 и живые изменения на 44.1/48/96 kHz. Оба движка сначала запускают 2048 clean-отсчётов, затем включают маршруты и прогревают их и последующие фильтры 0.5 s. Здесь ноты удерживаются: первые 20 ms новой ноты намеренно изменены, чтобы не подставлять чужой оператор. [Диагностика этого изменения](../reports/wavetable/optimization/routing-note-entry-diagnostic.json) сохранена отдельно и не является критерием совместимости с ошибочным прототипом. Note lifecycle покрывают исходный native/WASM корпус и integration tests. Установившиеся scalar/SIMD результаты проверяются отсчёт в отсчёт без подгонки времени или gain.

## Совместимость, память и проверки

4578 host parameters; первые 4566 закреплены fixture без перестановки, переименования и изменения диапазонов. Восемь source-параметров и четыре FM mode/index B/D добавлены в конец. Schema=2; engine revisions 1/2 мигрируют в 3, неизвестная будущая ревизия отвергается. Старые missing-source поля получают прежнюю пару FM и собственный sync clock. Удаление волны перенумеровывает ссылки; маршруты в удалённый источник выключаются.

Compiler/layout revision банка повышена до 2. Дополнительные коэффициенты ограничены **4 MiB на банк**; исходные sinc samples остаются на месте. Полиномы получают меньшие mips, остальным остаётся sinc fallback. Память учитывается в bank payload и проверяется существующим 64 MiB staging/arena budget. Chunk upload остаётся ≤32 KiB. Четыре разных больших банка, previous banks и preview требуют отдельного общего peak preflight; лимит 128 MiB всей системы этим изменением не доказан.

Core/integration проверяют все 16 пар отдельно для FM и sync, ring/route automation, перезапуск нот, отсутствие audio heap work, migration/recall, влияние выбора источника на звук и перестановку узлов. Native Rust и C++ instrument/FX host tests проверяют automation и сохранение новых источников. Shipping WASM проверяет memory identity при edits, все источники, feedback и portable scene recall. Итоговые результаты и хеши сборок — [verification.json](../reports/wavetable/optimization/verification.json).

Локальная alpha включает новые маршруты. Для beta остаются device/DAW CPU matrix, длительные host runs, общий peak memory preflight, полноценный динамический alias oracle и межплатформенный phase corpus. Параметры factory :v1, качество oversampling и поддержка старых патчей не должны изменяться ради прохождения CPU gate.
