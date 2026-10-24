# Porting the MS-70CDR effects into `sad-pet-mooz`

`sad-pet-mooz` rebuilds all 137 effects of the Zoom MS-70CDR 2.10 firmware as
native f32 DSP for SAD PET. In the app every model appears as an FX type named
`mooz <original name>` (slot kind `mooz_<name>`). This file is the brief for
everyone porting a group of models.

## Fidelity standard

The reconstruction project `/Users/georgy/Projects/zoom-ms70-cdr` recovered the
**audio callbacks** of every ZDL bit-exactly (Rust, operating on raw 32-bit
coefficient words and emulated C674x float/fixed arithmetic), but only a few
**parameter editors** (knob → coefficient code): gates, ZNR, 160 Comp,
SlwAtkDly, parts of Plate/Tremolo/AutoWah/Step/SeqFLTR/RndmFLTR/ReverseDL and
the ROM tables in the `*_parameters.rs` files. The editors of every module are
available as disassembly with symbol names.

A port, in order of priority:

1. **Signal flow from the recovered callback.** Same structure: filters, delay
   taps, feedback paths, LFO shapes, detectors, mixing order, stereo routing,
   saturation/clipping points. Understand what every coefficient index does.
2. **Coefficients from the original editors.** Read the init and `*_edit`
   functions in the disassembly to learn how each knob value becomes
   coefficient values (ROM tables, Main curve calls, divisions, tempo math), and
   reproduce that math in f32. Prefer the original numbers to invented ones.
3. Where an editor is impractical to decode in reasonable time, choose a
   musically faithful mapping that matches the parameter's name, range and the
   coefficient's role in the callback, and say so in the module doc comment and
   in your report.

Not goals: bit exactness, emulated rounding, the C674x memory layout, block
size 8, Main's scheduling. Write ordinary, efficient f32 code.

## Sources (all under `/Users/georgy/Projects/zoom-ms70-cdr`)

- `src/domain/effects/<module>.rs` or `src/domain/effects/<module>/` — the
  recovered callbacks. `RecoveredEffect` in `recovered.rs` lists them. Each
  group's assignment below names the files. `fp::multiply/add/subtract` are
  plain float ops with a rounding mode; `fixed32` helpers are Q31 math.
- `src/domain/effects/*_parameters.rs`, `.../<module>/parameters.rs`,
  `src/application/*_parameters.rs` — recovered storage resets, ROM
  coefficient tables and (for a few models) complete editors.
- `docs/<name>-reconstruction.md` — notes per model (Russian), often naming
  what coefficient slots mean.
- `artifacts/analysis/2.10/disassembly/<ZDL>.dis` — official TI `dis6x`
  listing of the module: `Fx_<KIND>_<Name>` (the audio callback),
  `..._init`, `..._onf` and one `..._<param>_edit` per knob.
- `artifacts/analysis/2.10/elf/<ZDL>.elf` — the ELF image. ROM tables live in
  `.const` at `0x80000000`; `.fardata` follows. Dump them with
  `python3 vendor/mooz/tools/zdl_dump.py <ZDL> [address|.section] [words]`
  (prints hex, float, int and Q31 for each word).
- `tools/*.py` — reference/equation scripts some stages used; occasionally
  they spell out formulas.
- `tools/reference/ZoomMultistompZDL/docs/EDIT-HANDLER-ABI.md` — editor ABI.
- TI manuals: `tools/reference/ZoomMultistompZDL/docs/sprui04g.pdf`
  (C674x instruction set) if you need an instruction's exact meaning.

### Reading an editor

An editor gets `A4` = the 53-word Main context of its slot and calls services
through it with `CALLP __call_stub` after loading `B31 = context[i]`
(`LDW *+A4[i],B31`). Arguments in `A4, B4, A6, B6, A8, B8...`, result in
`A4`. `context[1]` is the coefficient base, `context[2]` the state base. The
services met most often:

| index | Main address | service |
|---|---|---|
| 31 | c00b820c | get parameter word: `A4`=slot (`context[0]`), `B4`=firmware parameter index (0 OnOff, 1 name record, 2.. user params) → `A4` |
| 21 | c00ca380 | parameter curve → float: `A4`=maximum, `B4`=minimum, `A6`=count, `A8`=selected, `B6`=shape (see `zoom::curve`) |
| 22 | c00ca160 | integer (Q31) variant of the curve, see `src/domain/machine/main_integer_parameter_curve.rs` |
| 8  | c00cc8c8 | float setter with ramp: `A4`=coefficient address, `B4`=value, `A6`=step per 44.1 kHz sample (`zoom::Ramp`) |
| 7  | c00cc94c | ramped store: `step = max(|current − target| / A6, 1e-6·|target|)`, then `context[8]`; editors pass `A6` = 705.6 → a 16 ms ramp |
| 25 | c00bcb20 → c00bc2e0 | filter designer (types: 0/1 first-order LP/HP, 2/4 and 3/5 second-order LP/HP, 6 BP, 7 notch, 8 first-order high shelf, 9 first-order low shelf, 10 Zölzer peak/dip); a host-rate port is `src/fx/dynamics_eq/common.rs` |
| 26 | c00d2f08 | dB → linear, `10^(x/20)` |
| 15, 34 | c00dd4b0, c00ddda0 | copy designed coefficients into place |
| 23 | c00d7c60 | tempo-synced LFO phase increment (`zoom::rate_note_hz`) |
| 24 | c00d7bc0 | tempo-synced delay duration in 1/100 ms (`zoom::delay_note_ms`) |
| 39 | c00df880 | float division |
| 43, 45 | c00d6540, c00d6220 | power / exp-log helpers (`main_power_workspace.rs`, `main_transcendental.rs`) |
| 50 | c00ddb20 | sqrt |
| 51 | c00e1640 | int → float |

Other indices: see `src/domain/effects/main_context.rs` for the table and
grep the Main address under `src/domain/machine` or `src/domain/audio`; the
Main code listings are `artifacts/analysis/2.10/main/*.dis`.

The ROM-copy in `init` (`memcpy` of a `.const` block into the coefficient
area) gives every coefficient's starting value; `*_parameters.rs` usually
already has it as a Rust array.

## The crate

```
vendor/mooz/src/lib.rs        Effect trait, Model helpers        (shared, do not edit)
vendor/mooz/src/catalog.rs    ModelInfo / ParamInfo / Display    (shared, do not edit)
vendor/mooz/src/dsp.rs        Biquad, DelayLine, Allpass, ...    (shared, do not edit)
vendor/mooz/src/zoom.rs       curve, tempo tables, Ramp          (shared, do not edit)
vendor/mooz/src/registry.rs   Model enum                         (shared, do not edit)
vendor/mooz/src/fx/<group>/   YOUR directory: mod.rs + one module per model
vendor/mooz/tests/realtime.rs shared contract test               (shared, do not edit)
```

Each model module must export, with these exact names:

```rust
pub const INFO: ModelInfo = ...;                       // keep key/name/zdl and param order/max/default
pub fn create(sample_rate: f32) -> Box<dyn Effect>;   // allocates everything
```

and implement `Effect`:

- `set(&mut self, values: &Values, bpm: f32)` — values in pedal units, in the
  order of `INFO.params` (the stub's comments give each one's firmware
  parameter word, i.e. the index the editors pass to `context[31]`). Values may
  be fractional while modulated: round where the original is stepped, or
  interpolate where a continuous value is musically better. Called often
  (every audio block while a modulator runs); cache derived coefficients and
  recompute only what changed. Recompute tempo-synced times when `bpm` changes.
- `process(&mut self, [f32; 2]) -> [f32; 2]` — one stereo frame, full scale
  ±1.0. Mono originals: follow what the callback does with L/R.
- `clear(&mut self)` — zero all histories.

Rules:

- **Host sample rate.** `create` gets 44.1, 48, 88.2 or 96 kHz (also other
  rates). Recompute every rate-dependent number: delay lengths
  (`dsp::scale_samples`), LFO increments, time constants, and filters. Re-derive
  ROM biquads by designing the equivalent filter for the new rate, or with
  `Coefficients::resample(44_100.0, sample_rate)` for ROM sets whose analog
  shape is unclear. Size buffers for the largest parameter value at the instance
  rate.
- **Realtime.** No allocation, locking, I/O or panics in `set`/`process`/
  `clear` (the shared test counts allocations). No `Vec::push`, `Box::new`,
  `format!`. Pre-allocate in `create`.
- **Efficiency.** Several instances can run in a browser (WASM) at 48 kHz.
  Avoid per-sample `powf/exp/sin` (use `dsp::sin_turns`, recurrences, tables,
  per-block updates every 16–32 samples where the original updates at block
  rate). Typical budgets at 48 kHz on this Mac, release build: simple effects
  < 40 ns/frame, delays/modulation < 80, reverbs/pitch < 250.
- **Smoothing.** Avoid zipper noise and clicks when values change: ramp gains
  and coefficients like the original float setter (`zoom::Ramp`) or with
  `dsp::Smoother`; change delay times smoothly (slew or crossfade) the way the
  original does.
- **Levels.** Keep the original gain staging: Level is usually linear
  `value / 100` (check the `outLv_edit`), defaults should sound roughly at
  unity loudness. Clamp or soft-limit only where the original saturates.
  Outputs must stay finite and below ±16 even with feedback at maximum.
- **Display.** Fill `Display` for every parameter so the UI shows what the
  pedal shows: `Display::offset(1.0)` for 0..29 shown as 1..30, `labels(&[..])`
  for modes/waves/patterns (one label per value), `DelayTime { threshold }` for
  ms + note times, `sync_rate(threshold, offset)` for rates with a note tail,
  `number(offset, scale, decimals, unit)` for scaled values (e.g. dB). Find the
  real ranges from the editors (e.g. a 0..24 EQ gain is −12..+12 dB:
  `number(-12.0, 1.0, 0, " dB")`). Write a short `description`
  (≤ 60 characters, English) and fix `category` if the stub guessed wrong.
- Unused trailing `values` entries are 0. The original `Tail` switch is not a
  parameter here.
- Shared code between models of your group goes in your group directory (e.g.
  `fx/<group>/common.rs`, declared in your `mod.rs`). Never edit the shared files;
  if one is wrong or a helper is missing, write your own in your directory and
  mention it in your report.
- Tests: add focused `#[cfg(test)]` unit tests in your modules (frequency
  response of a filter at a known setting, delay time in samples, LFO rate,
  tempo sync, gain at defaults, no runaway at max feedback, ...).
- `cargo fmt` your files; `cargo clippy --all-targets -- -D warnings` must be
  clean for your directory.

## Workflow (several porters run at the same time)

Work in a private copy so nobody's half-finished code breaks your build:

```sh
W=<your scratch dir>/mooz-<group>
rm -rf $W && mkdir -p $W && cp -R /Users/georgy/Projects/SadPetRust/vendor/mooz/. $W/
cd $W
export CARGO_TARGET_DIR=$W/target
MOOZ_ONLY=mooz_delay,mooz_analog_dly cargo test            # your models only
MOOZ_ONLY=... cargo test --release --test realtime -- --ignored --nocapture cost
cargo clippy --all-targets -- -D warnings
```

(`cargo` here is Homebrew's; it is fine for native builds.) The copy has a
`[workspace]`-free manifest; never build inside the SadPetRust tree while you
work. When everything passes, copy **only your group directory** back:

```sh
rm -rf /Users/georgy/Projects/SadPetRust/vendor/mooz/src/fx/<group>
cp -R $W/src/fx/<group> /Users/georgy/Projects/SadPetRust/vendor/mooz/src/fx/<group>
```

Do not touch anything else in `/Users/georgy/Projects/SadPetRust` (another
agent may be editing it), do not commit, do not run git commands that change
state.

## Report

Finish with a short report per model: what the port follows (callback
topology, which editors were decoded, which mappings are approximations), the
release cost in ns/frame from the `cost` test, and anything left undone.
