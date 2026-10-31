# Численный опыт чтения wavetable

Этот каталог содержит исследовательские и production-измерения для [исследования](../../docs/WAVETABLE_RESEARCH.md) и [PRD](../../docs/WAVETABLE_PRD.md). Масштаб каждого опыта указан отдельно.

## Production Rust/WASM

- `rust-lookup-spectral.json`: 630 статических случаев, production f32 bank/sinc16 против независимого f64 Fourier oracle; 5 форм, 7 высот, 3 фазы, 6 Fs. Worst spurious −77.06 dBr; total error −74.76 dBr. Динамические scan/mip transitions сюда не входят.
- `native-rack-four-profile.json`: общий Engine/Rack, 2 и 4 активных слоя, 48 kHz/128, по 60 секунд аудио после 5 секунд прогрева. M5 Max; сборки на других ядрах продолжались. Это CPU timing внутри процесса. Физические device underruns и сторонний DAW не измеряются. Максимальный stress 32×4×8 превысил deadline.
- `native-rack-profile.json`: исторический двухслойный runtime; его цифры нельзя переносить на четыре слоя.
- `wasm-four-parity.json`: 24 сравнения shipping scalar/SIMD и блоков 16…2048 на 44.1/48/96 kHz, четыре разных таблицы и sample-timed events; max abs difference = 0. Сравнение с native backend не входит.
- `factory-levels-original-48.json`: исходные 48 инструментов × три сценария, 144 случая; сохранены как baseline. `factory-levels.json` и `factory-audition.wav`: 80 инструментов / 240 случаев, все finite/nonzero/peak<0.98; новые пресеты максимум −3.73 dBFS, прежние 48 peak/RMS не изменились. WAV — короткий монтаж шести категорий.
- `content-quality.json`, `content_quality.py`: 96 авторских таблиц, отдельные критерии спектральной сложности и переходов для 64 новых; проверка неизменности 32 исходных IDs/hashes.
- `complex-packs-audition.wav` / `.json`: восемь новых семейств по одной таблице, сухая C4/U=1, sweep Position 0…1. Живой production Engine, без FX, 16.8 s, с cue points.
- `mobile-360-four.jpg`, `mobile-390-four.jpg`: реальные browser screenshots A–D; ширина iframe ровно 360/390 CSS px, без горизонтального скролла страницы.

Из корня репозитория:

```sh
cargo run -p sad-pet --release --no-default-features --example wavetable_bench -- spectral
cargo run -p sad-pet --release --no-default-features --example wavetable_bench -- rack 60
cargo run -p sad-pet --release --no-default-features --example wavetable_bench -- factory /tmp/factory-audition.wav
python3 wasm/build.py --simd --output /tmp/sadpet-wt-web
node wasm/test-wavetable.mjs /tmp/sadpet-wt-web
node wasm/test-wavetable-parity.mjs /tmp/sadpet-wt-web /tmp/wasm-four-parity.json
```

Текущие пробелы и release gates перечислены в [ledger реализации](../../docs/WAVETABLE_IMPLEMENTATION.md). Наличие файла с результатом не означает прохождение всей матрицы PRD.

## Исходный offline lookup study

Этот Python-опыт не подключён к production DSP и не измеряет CPU SAD PET.

Запуск из корня проекта с Python 3 и NumPy:

```sh
python3 reports/wavetable/lookup_study.py
```

Скрипт перезаписывает только `lookup_results.json` в этом каталоге. В JSON указаны версии Python/NumPy, частота дискретизации, длина записи, phase offset, точные частоты, dtype/арифметика, конфигурации и все 96 результатов. В terminal выводятся худшие значения каждой конфигурации.

Эталон строится непосредственно из Fourier coefficients, независимо от проверяемого lookup. Два сигнала — сумма гармоник saw с амплитудами 1/h и одна самая высокая гармоника текущего уровня. Во втором случае ошибка интерполяции не маскируется естественным спадом спектра saw.

`spurious_db_relative_to_reference_rms` измеряет энергию вне ожидаемых FFT bins. `total_error_db_relative_to_reference_rms` дополнительно учитывает изменение разрешённых гармоник. Это относительные dBr, не абсолютные dBFS. Для исходной одиночной гармоники наивного варианта выше Nyquist эталон — тишина, знаменатель — исходный RMS синусоиды.

Эксперимент использует f32-таблицы и округление выхода к f32, но f64-фазу и арифметику интерполяции. Он не проверяет production f32/SIMD, polyphase coefficient quantization, движение Position, переходы mip, импорт, real-time callbacks или потребление CPU. Его вывод — выбрать кандидатов для следующего Rust/WASM опыта, а не объявить движок готовым.
