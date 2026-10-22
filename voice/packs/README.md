# Voicebanks that come with SAD PET

The plug-in and the app build these in (`plugin/src/vocal.rs`); the web build
copies them to `voices/` with a `packs.json` list (`wasm/build.py`). Their ids
and languages are `sad_pet::vocal::PACKS`, and a new Vocal timbre sings with
the one for its language.

| File | Voice | Source |
| --- | --- | --- |
| `kasane-teto.sadvoice` | Kasane Teto, Japanese (重音テト音声ライブラリー, CV and VCV) | `TETO-tougou-110401.zip` from https://kasaneteto.jp/utau/ — © 小山乃舞世 (Oyamano Mayo) |
| `kasane-teto-en.sadvoice` | Kasane Teto, English | `TETO-English-150401.zip` from https://kasaneteto.jp/utau/ — © 小山乃舞世 |
| `kumi-ru.sadvoice` | 筆音クミ (Kumi), Russian CVC, four pitches | `Kumi - Russian.zip` by Cubialpha |

Rebuild one after the analysis or the file format changes:

```sh
cargo run -p sad-pet-voice --features cli --release -- import TETO-tougou-110401.zip \
  --language ja --name "Kasane Teto (JA)" --author "小山乃舞世" \
  --description "UTAU voicebank by 小山乃舞世 (Oyamano Mayo), kasaneteto.jp" -o voice/packs/kasane-teto.sadvoice
cargo run -p sad-pet-voice --features cli --release -- import TETO-English-150401.zip \
  --language en --name "Kasane Teto (EN)" --author "小山乃舞世" \
  --description "UTAU voicebank by 小山乃舞世 (Oyamano Mayo), kasaneteto.jp" -o voice/packs/kasane-teto-en.sadvoice
cargo run -p sad-pet-voice --features cli --release -- import "Kumi - Russian.zip" \
  --language ru --name "Kumi (RU)" --author "Cubialpha" \
  --description "UTAU voicebank 筆音クミ - Russian by Cubialpha" -o voice/packs/kumi-ru.sadvoice
```
