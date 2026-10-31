# Реализация Wavetable

Цель: весь обязательный объём 1.0 из [PRD](WAVETABLE_PRD.md), включая исходные 32 таблицы / 48 пресетов и расширение до 96 таблиц в 13 паках / 80 инструментов. Дополнения Third Wave и Unison Voice shift реализованы в локальной alpha; актуальные контракты, проверки и план выпуска — в [Third Wave ledger](WAVETABLE_THIRD_WAVE_IMPLEMENTATION.md). Таблицы ниже сохраняют базовые release gates: наличие новых функций не закрывает их автоматически.

## Текущее состояние

- Начальная проверка: текущие `rack` и `rack_sources` проходят, 20 тестов.
- Рабочие изменения Control/MIDI и остальные исходные изменения пользователя сохраняются.
- Production DSP 1–4 волн A–D, unison, per-note Position ENV и седьмой Mixer strip подключены к Rack.
- 96 оригинальных таблиц в 13 встроенных паках; 80 инструментов в 6 категориях, по 4 macros. Добавлены 64 сложные таблицы и 32 четырёхслойных патча. [3rd Wave study](WAVETABLE_THIRD_WAVE.md). Музыкальная приёмка продолжается.
- Native: 9 интеграционных WT-тестов и 20 прежних Rack-тестов проходят; сохранены префиксы 4274 исходных и 4304 A/B host-параметров; новые C/D дописаны в конец.
- WASM: utility compilation → 32 KiB staging → cache-only worklet → Rack sound → portable user scene проверены на настоящем модуле; копирование и scalar edits без memory.grow.
- Реальный браузер: добавление WT, мобильная библиотека паков, смена таблицы, Position с клавиатуры; на 360/390/1280 CSS px scrollWidth = clientWidth. Добавлены A–D, проверен предел 4, удаление, C/D controls; размеры рядов совпадают, основные цели 44 CSS px.
- Исторический двухосцилляторный прототип: native Engine/Rack, M5 Max, 48 kHz/128, 60 s audio/profile после 5 s прогрева: p99 WT addition P01 11.85%, P02 16.43%, P03-подобный профиль 5.01%, 32×2×8 50.60%.
- С текущим четырёхслойным runtime, M5 Max: 32×4×1 p99 full 0.671 ms / added 24.31% deadline; 16×4×4 0.936 ms / 34.45%; 8×4×2 0.330 ms / 11.78%; stress 32×4×8 3.725 ms / 138.81% (deadline превышен). [JSON](../reports/wavetable/native-rack-four-profile.json). Это in-process native, во время сборок на других ядрах; не M1/mobile/DAW underrun-приёмка.
- 630 статических spectral cases на 44.1/48/88.2/96/176.4/192 kHz: saw/square/triangle/random-phase/sparse partial × 7 высот × 3 фазы. Worst spurious −77.06 dBr, total error −74.76 dBr; [JSON](../reports/wavetable/rust-lookup-spectral.json).
- WASM scalar/SIMD, 4 активные волны, sample-timed notes/bend/edits: 24 сравнения (44.1/48/96 kHz × blocks 16…2048), max abs difference = 0; [JSON](../reports/wavetable/wasm-four-parity.json).
- Последняя проверка 31 октября: Native plugin Rust suite 60/60, core lib 174/174, WT integration 16/16. C++ host instrument + FX: 2/2, включая 4 волны, C/D automation и state recall. Native VST3/AU собраны локально; новые Wave ENV/motion/tools/interactions/Voice shift проверяются в Third Wave suite. Это не сторонняя DAW/device-приёмка.
- Factory headroom: 48 инструментов × note/chord/full-macros-chord, 144 случая finite/nonzero/peak<0.98; максимум −1.51 dBFS. Это численная проверка, не подтверждение слушательской приёмки. [JSON](../reports/wavetable/factory-levels.json).
- Web alpha выпускается отдельно через GitHub Pages; [релизная проверка](../reports/wavetable/web-alpha-release/verification.json). Полная браузерная/DAW-приёмка и стабильный выпуск 1.0 не завершены.

## Рабочие этапы

| Этап | Статус | Доказательство |
| --- | --- | --- |
| M0: kernel/layout, Rust/WASM benchmarks, staging | В работе | Rust spectral + native full Rack + real WASM transfer; ещё нет полной device/CPU/memory matrix |
| M1: assets/compiler/A–D/unison | В работе | 12 WT unit tests, 9 integration tests; расширяется quality matrix |
| M2: Rack/per-note/strip/runtime | В работе | Rack rendering, VCA once, local ENV, no-heap exchange, stable host prefix |
| M3: import/library/portable/native+WASM | В работе | WAV import/export tests, manifests, native recall, real WASM staging, disk/IndexedDB library |
| M4: host controls/modulation/Neuropolis/Control/UI | В работе | UI A–D + macros/catalog; проверяются pitch lanes, learn, Control и stale jobs |
| M5: 96 tables, 80 presets, quality/device/DAW matrix | В работе | Контент встроен; headroom/listening/device/DAW ещё проверяются |
| M6: release gates, docs, packaging, RC | Ожидает | — |

## Проверка требований

Отметка «готово» требует прямого подтверждения кодом и проверкой нужного масштаба. Пороговые значения PRD не являются уже полученными результатами.

| ID | Требование/сценарий | Статус | Доказательство |
| --- | --- | --- | --- |
| U01 | `+ Timbre → Wavetable` | Не проверено | — |
| U02 | `+ Module → Sources → Wavetable` в существующем Rack | Не проверено | — |
| U03 | Вращение Position при удержанной ноте | Не проверено | — |
| U04 | LFO 1 → A Position | Не проверено | — |
| U05 | Position ENV = ENV 2; две ноты с интервалом 200 ms | Не проверено | — |
| U06 | Включить B, расстроить, добавить unison | Не проверено | — |
| U07 | Перетащить подготовленный WAV на A | Не проверено | — |
| U08 | Импорт без достаточных метаданных | Не проверено | — |
| U09 | Поменять таблицу на длинном аккорде | Не проверено | — |
| U10 | Сохранить `.sadinst`, удалить исходный WAV, открыть в другом хосте | Не проверено | — |
| U11 | Переместить Wavetable ниже Mixer | Не проверено | — |
| U12 | Wavetable → Grains | Не проверено | — |
| U13 | A Position в Neuropolis и на Control canvas | Не проверено | — |
| U14 | Открыть старую сцену без Wavetable | Не проверено | — |
| R01 | `Module::Wavetable` является source, имеет label и стабильный snake_case ключ `wavetable` | Проверено в указанном масштабе | rack::Module + shared Rack tests |
| R02 | Wavetable стартует per-note flow как Synth | Не проверено | — |
| R03 | Runtime возвращает сигнал **до** Engine VCA | Проверено в указанном масштабе | the_engine_applies_the_note_vca_exactly_once |
| R04 | После collapse источник обрабатывается штатными общими модулями | Не проверено | — |
| R05 | В Mixer появляется седьмой source strip, после имеющихся шести | Не проверено | — |
| R06 | Ниже Mixer источник следует текущему правилу вывода рядом с суммой | Не проверено | — |
| R07 | Модуль поддерживает move, fold, power, remove и Undo | Не проверено | — |
| R08 | Удаление Wavetable не удаляет общие ENV/LFO/notes и другие source chains | Не проверено | — |
| R09 | Модуль безвреден для Rack без Wavetable | Не проверено | — |
| R10 | Расширение `rack_peaks` с 6 до 7 согласовано во всех binary telemetry/pack decoders | Не проверено | — |
| S01 | Пользовательский asset идентифицируется SHA-256 от versioned canonical header и f32 LE данных; название/путь не входят в идентичность звука | Проверено в указанном масштабе | assets::identity_preserves_frame_levels_and_is_portable |
| S02 | Factory ID включает неизменяемую ревизию, например `factory:wt-basic-morph:v1`; новый звук получает новый ID | Проверено в указанном масштабе | factory catalog IDs :v1 + content hash validation |
| S03 | Канонические данные однозначны: порядок кадров/отсчётов, f32 LE, отсутствие NaN/Inf, канонизация −0 перед хешированием | Проверено в указанном масштабе | assets::validate, from_cycles, samples_codec tests |
| S04 | Compile-cache key включает content hash, compiler revision, layout/kernel revision; file timestamp не является ключом | Проверено в указанном масштабе | compiler/layout/kernel revision cache key + BankHeader |
| S05 | `engine_revision` фиксирует звуковую семантику: gain law, interpolation и smoothing. Изменение звука старого патча требует явной новой ревизии/миграции | Не проверено | — |
| S06 | Derived mips можно пересоздать; одинаковый канонический asset не должен занимать отдельную копию на каждую ноту | Не проверено | — |
| S07 | Пользовательская таблица не меняется при замене/переименовании её исходного файла | Не проверено | — |
| S08 | Пропавший актив не заменяется произвольной похожей таблицей; при интерактивной смене сохраняется прежний звук, при restore выдаётся конкретная ошибка | Не проверено | — |
| I01 | RIFF/WAVE PCM16/24/32 integer и IEEE float32 | Проверено в указанном масштабе | import::formats_stereo_choice_and_portable_export |
| I02 | 1–256 кадров; входной период 128/256/512/1024/2048/4096 samples | Проверено в указанном масштабе | periodic_resampling + import bounds tests |
| I03 | Mono или выбор канала stereo | Не проверено | — |
| I04 | Поддерживаемая metadata `clm ` или явный размер периода | Не проверено | — |
| I05 | Неоднозначный файл | Не проверено | — |
| I06 | Подготовленный однозначный файл | Не проверено | — |
| I07 | Лимит входа 16 MiB, проверка decompressed размеров | Не проверено | — |
| I08 | Пустые/некорректные/NaN/Inf данные | Не проверено | — |
| I09 | Число frames не получается целым либо превышает 256 | Не проверено | — |
| I10 | Заголовочный sample rate | Не проверено | — |
| RT01 | Во время live render: 0 alloc/realloc/free, блокировок, file/network I/O, FFT и сериализации в аудиопути | Не проверено | — |
| RT02 | Это ограничение распространяется на note-on, numeric automation, asset commit, crossfade completion, remove/reorder и retirement | Не проверено | — |
| RT03 | На осциллятор одновременно не более active+transition bank; ожидающая смена coalesce до последнего запроса | Не проверено | — |
| RT04 | Каждый job имеет scene/timbre/oscillator generation; старый результат не применяется к новому тембру либо удалённому модулю | Не проверено | — |
| RT05 | Пока данные готовятся, старый bank звучит; для первого включения до готовности нового источника он молчит, остальные timbres продолжают играть | Не проверено | — |
| RT06 | Cancel, memory-limit и decode-error не изменяют прежнее состояние; промежуточная библиотечная запись не остаётся как готовый asset | Не проверено | — |
| RT07 | При заполнении очереди control owner объединяет новые команды или сообщает о невозможности операции; callback не ждёт и не освобождает накопленные банки | Не проверено | — |
| RT08 | Restore/scene swap применяются как цельное подготовленное состояние; при ошибке не остаётся половина нового Rack | Не проверено | — |
| RT09 | Asset selection и параметры принимаются через существующие хостовые границы; UI не держит частный источник истины о звучащем asset | Не проверено | — |
| UI01 | На ширине 360/390 px нет горизонтального скролла всей страницы; A–D редактируются по одному, без слишком мелких двух колонок | Не проверено | — |
| UI02 | Элементы одного ряда имеют одинаковую высоту; общий grid и существующие размеры проекта используются повторно | Не проверено | — |
| UI03 | Touch targets основных действий не меньше 44×44 CSS px; вертикальный скролл не блокируется декоративным canvas | Не проверено | — |
| UI04 | Position, tune и level можно менять без hover, right-click, wheel или drag-and-drop | Не проверено | — |
| UI05 | Есть клавиатурный фокус, стрелки/числовое редактирование, accessible names и значения в реальных единицах | Не проверено | — |
| UI06 | В библиотеке и импортном диалоге фокус сохраняется/возвращается; закрытие не теряет текущую таблицу | Не проверено | — |
| UI07 | Ошибка показывается рядом с действием и называет причину: format, frame size, length, missing asset, memory. Нет технических stack traces в основном потоке | Не проверено | — |
| UI08 | Fold/переключение A–D не прерывают звук и не сбрасывают viewport; снижается только работа отображения | Не проверено | — |
| UI09 | Loading относится к конкретному asset; после ошибки остаются прежние имя/волна/настройки, без ложного отображения незвучащей таблицы | Не проверено | — |
| P01 | Desktop: 32 ноты × A/B × U=1, 48 kHz/128 | Не проверено | — |
| P02 | Desktop: 16 нот × A/B × U=4, 48 kHz/128 | Не проверено | — |
| P03 | Mobile browser: 8 нот × A/B × U=2, 48 kHz/128 или фактический quantum | Не проверено | — |
| P04 | Asset staging во время P02/P03 | Не проверено | — |
| P05 | Работа без WT | Не проверено | — |
| P06 | Максимум 32×4×8 | Не проверено | — |
| P07 | Импорт 64×2048 mono на desktop | Не проверено | — |
| P08 | Максимальная таблица 256×2048 | Не проверено | — |
| P09 | Canonical + compiled bank одной максимальной таблицы | Не проверено | — |
| P10 | Bank sharing | Не проверено | — |
| P11 | Browser WT working set | Не проверено | — |
| P12 | Native WT working set | Не проверено | — |
| P13 | 1000 переключений таблиц/отмен jobs | Не проверено | — |
| Q01 | Строй U=1, MIDI 12…120, поддержанные Fs | Не проверено | — |
| Q02 | Статические saw/square/triangle, random-phase harmonic tables и sparse high partial | Не проверено | — |
| Q03 | Повторные ноты с phase random=0 | Не проверено | — |
| Q04 | Phase random=1 | Не проверено | — |
| Q05 | Position=0/1, F=1/2/256 и границы кадров | Не проверено | — |
| Q06 | Медленное и быстрое сканирование 0.1/1/20 Hz | Не проверено | — |
| Q07 | Bend/glide через каждую границу mip | Не проверено | — |
| Q08 | Смена одного и того же asset | Не проверено | — |
| Q09 | Смена разных assets, enabled/unison и edits | Не проверено | — |
| Q10 | DC после подготовки и стационарного чтения | Не проверено | — |
| Q11 | Extreme values, silence, short frames, transpose выше Nyquist | Не проверено | — |
| Q12 | Одинаковые inputs native/WASM scalar/SIMD | Не проверено | — |
| Q13 | Block-size independence | Не проверено | — |
| Q14 | Старые сцены без WT | Не проверено | — |

## Дополнение A–D

| ID | Требование | Статус | Доказательство |
| --- | --- | --- | --- |
| W01 | 1–4 добавляемые волны, максимум 4 | Проверено | Rust Settings; native no_heap; real WASM; browser 360/390/1280 |
| W02 | Независимые настройки каждой волны | Проверено в указанном масштабе | 52 host controls; C/D scalar edits; sample-timed WASM parity; four Neuropolis pitch targets |
| W03 | Удаление любой волны, минимум 1 | Проверено | Settings migration/edit test; held-note native/WASM; browser Remove D + re-add |
| W04 | Recall четырёх волн и старого A/B | Проверено в указанном масштабе | Schema 1→2 test; scene/native DAW count4+controls; portable manifests preserved |
| W05 | C/D host automation без сдвига A/B | Проверено в указанном масштабе | 4274+30 prefix snapshots; C++ instrument/FX host C/D automation+recall; numeric registry/Neuropolis; Control canvas UI ещё требует отдельной проверки |

Обязательная работа перед стабильным выпуском: общий peak-memory preflight P11/P12, полный dynamic oracle/phase/tuning/native-WASM corpus, 1000 switches и 30-minute host/device run, браузерные import/Control/MIDI end-to-end, физические reference mobile/Windows и сторонние DAW. Существующие 64 MiB arena и 64 MiB Pool являются локальными ограничениями; их нельзя выдавать за доказанный общий лимит WT-подсистемы.

## Комплексные паки и 3rd Wave

- 64 новые таблицы, по 8 в Complex FM, Formant Journeys, Resonant Objects, Phase Architectures, Spectral Journeys, Vector Worlds, Fractal Matter, Digital Evolution.
- [Content quality](../reports/wavetable/content-quality.json): все 96 hashes валидны; исходные 32 не изменены. У новых минимум 12 заметных гармоник в h=1..63; effective harmonic median 4.85…29.01; spectral distance ≥0.172; midpoint RMS ratio ≥0.980.
- Нелинейные offline recipes готовятся при 8× и Fourier-truncated до канонической полосы. Это контент для текущего lookup, не новая live FM/sync ветка.
- 32 новых four-wave пресета; 80 всего. Новые Scan/Width macros охватывают A–D. Проверка headroom: 240/240 finite/nonzero/peak<0.98; максимум среди новых пресетов −3.73 dBFS. Прежние 48 дали идентичные peak/RMS.
- Данные gzip: 22,554,271 bytes вместо 6,122,965. В текущей embedded WASM схеме это увеличивает каждый instance; вынесение browser packs в отдельные статические assets и общий memory preflight остаются важным release gate.
- Wave Envelope / Surf / Step / Key to Waves / XY / live FM / Capture реализованы в последующем [Third Wave этапе](WAVETABLE_THIRD_WAVE_IMPLEMENTATION.md). Произвольные источники и оптимизация interactions описаны [отдельно](WAVETABLE_OPTIMIZATION.md).

- Расширение проверено в compiled WASM (96/13, 80 пресетов, complex bank staging/scene recall), scalar/SIMD parity (24 сравнения), native WT tests и двух C++ host targets.

- Реальный browser: новый Fourfold Orchestra загружается четырьмя волнами; 13 паков и новые previews доступны. На 360 CSS px overflow отсутствует; menu rows и preview buttons измерены 44×44 px. Idle preview следует выбранному Position до старта аудио. [Снимок](../reports/wavetable/complex-mobile-360.jpg), [сухое демо](../reports/wavetable/complex-packs-audition.wav).
