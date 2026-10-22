# SAD PET voice engine

`sad-pet-voice` turns a score of notes and lyrics into singing. It is plain Rust
with no threads or platform code, so the plug-ins, the app and the browser
(WebAssembly in a Worker) render with the same code.

## How it sings

1. **Lyrics → phonemes** (`g2p`). Japanese kana or romaji map to morae;
   English words come from a built-in dictionary of common lyric words, regular
   suffixes and the NRL letter-to-sound rules (a CMUdict file can be loaded);
   Russian applies palatalization, iotated vowels, voicing assimilation, final
   devoicing and the usual silent clusters, and reduces unstressed о only where
   the stress is marked. All languages share one phoneme inventory, so a voice
   of one language substitutes its nearest sounds for another's.
2. **Timing** (`timing`). Every syllable's vowel starts on its note; onset
   consonants are sung ahead of the beat at the voicebank's own lengths (VEL
   scales them), codas close the note, and consonants shrink before vowels do.
3. **Pitch** (`pitch`). Note-to-note transitions with overshoot and
   preparation, scoops into and falls out of phrases, delayed vibrato with a
   little rate and depth wander, slow drift, Attack/Release effects and the
   PIT/PBS curve.
4. **Units** (`concat`). A dynamic program picks voicebank units (CV, VCV, VC,
   diphones, sustained vowels) that cover the phonemes with the cheapest joins:
   inside vowels and pauses rather than bursts, between spectrally similar
   frames, from units recorded near the sung pitch. Units overlap on a shared
   phoneme and cross-fade there in the spectral domain; vowels stretch over
   their steady region, looping it for long notes. Units are brought to a
   common singing level, and the middle of each vowel moves most of the way
   toward the voice's typical shape of that vowel near the sung pitch, so
   takes recorded on different days still match. The built-in voice
   (`formant`) instead glides between formant and noise targets per phoneme.
5. **Voice color** (`params`). Dynamics, Breathiness, Brightness, Clearness,
   Character (formant scale), Exciter, Growl, Air and Mouth act on the frames.
6. **Vocoder** (`vocoder`). A WORLD-style source-filter synthesis: per glottal
   pulse a minimum-phase response of the envelope, split into periodic and
   noise parts by the aperiodicity.

Frames are 5 ms apart: F0, the power spectral envelope on a 96-point mel grid
(dB per Hz, independent of the sample rate) and aperiodicity in eight bands.

## Voicebanks

`utau` imports UTAU voicebanks: every `oto.ini` (Shift-JIS or UTF-8, in
subfolders for pitch layers), `character.txt` and `prefix.map`, from a folder
or a `.zip`. Aliases are spelled into phonemes (kana, romaji, Cyrillic, the
Latin notation of Russian UTAU banks, ARPAbet or X-SAMPA); the preutterance
marks where the final vowel (or a VC alias's consonant) begins, the overlap ends
a VCV leading vowel and the consonant area stays unstretched. `analysis`
measures F0 (YIN with a Viterbi path), the envelope (CheapTrick) and
aperiodicity (period-to-period coherence per band). `bank` stores the result as
a `.sadvoice` file: a JSON header and quantized frames, a few MB per bank.

## Command line

```sh
cargo run -p sad-pet-voice --features cli --release -- import <utau folder or .zip> -o voice.sadvoice
cargo run -p sad-pet-voice --features cli --release -- demo ru -o demo.wav
cargo run -p sad-pet-voice --features cli --release -- render score.json --voice voice.sadvoice -o out.wav
cargo run -p sad-pet-voice --features cli --release -- analyze recording.wav
cargo run -p sad-pet-voice --features cli --release -- resynth recording.wav
cargo run -p sad-pet-voice --features cli --release -- reconstruct <utau folder> <recording.wav> --language ru --bank cache.sadvoice
```

`resynth` measures the vocoder alone: it analyzes a recording, synthesizes it
back and prints the spectral distance. `reconstruct` measures the whole
engine: it sings one recording of a bank from the rest of the bank (its own
units removed) on the recording's syllables and pitches and compares the
spectral envelopes. With the free Kasane Teto (Japanese, English) and Kumi
(Russian) banks, 20 recordings average 4.3 dB (Japanese 3.5, Russian 4.6,
English 5.1); singing them from their own units gives 2.6 dB.

`render` reads a score as JSON (the format Vocal timbres save); `import` in the
editor also reads MIDI, `.ust`, MusicXML and `.vpr`.
