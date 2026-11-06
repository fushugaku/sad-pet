# Model and DSP boundaries

SAD PET shares one model and audio engine between the desktop server, the native
app and plug-ins, and WebAssembly. Hosts own storage, external assets, publication
and retirement. They use the shared application policy before preparing audio.

## Persisted model

`rust/src/preset.rs` defines the saved instrument document. Its fields describe
the sound: settings, modulation, sequencing, routing and effect slots. Cloning or
deserializing a `Preset` does not construct voices, resolve assets or allocate
audio histories. The format identifier remains `evil-pet-rust-v1`.

The root instrument modules define their settings and normalization rules.
Their runtime and preparation owners live under `dsp`:

| Persisted settings | Audio owner |
| --- | --- |
| `bus::Settings` | `dsp::bus::Prepared` |
| `rack::Settings` | `dsp::rack::{Prepared, Runtime}` |
| `drums::Settings` | `dsp::drums::{Prepared, Runtime}` |
| `tb303::Settings` | `dsp::tb303::{Prepared, Runtime}` |

`sequence::Program` likewise contains the saved program; `dsp::sequence::Runtime`
owns its clocks, gates and playback state.

`color::WaveshaperMode` defines the persisted waveshaper choices and their host
order. DSP converts that selection into its own coefficients; the model does
not depend on those processing types. Grain group settings use the same enum.

Keep validation that describes a valid saved sound with its model. Put rules for
loading a sound into a particular destination in the application layer.

## Application policy

`application::presets::load_into_timbre` resolves a decoded preset against the
current timbre's role and tempo policy. It handles inherited Vocal and Sampler
content and normalizes the resulting model. It borrows the current preset and
returns a candidate, so failed preparation cannot publish half an edit.

Asset resolution belongs to the host. A preset contains asset identities; its
wavetable pool and sampler kit are separate owners. Browser preparation uses the
staged assets supplied by its workers. It does not read the native asset library.

## DSP ownership and updates

`dsp::PresetPreparation` constructs a candidate's resources. Its incremental
interface lets the browser spread work across audio blocks. `finish` produces a
`dsp::PreparedPreset` only when preparation is complete. The prepared envelope
has no `Clone` or serialization implementation.

Keep new audio buffers, vendor engines and prepared banks on this side of the
boundary. Bus and Rack histories are constructed through the shared incremental
preparation stages.

The update lifecycle is:

1. The host decodes the model and resolves its external assets.
2. Shared application policy produces the destination's candidate settings.
3. DSP preparation validates and constructs the required resources.
4. The host publishes the prepared envelope to the audio owner.
5. `Engine::exchange_preset` installs it by swapping ownership. The envelope
   receives the previous settings and resources.
6. The host retires that envelope outside rendering and publishes the accepted
   model to its editor or telemetry owner.

Exchange rejects an incompatible sample rate or a previously used envelope
before changing the engine. An unsuccessful exchange must leave the host's saved
and displayed model unchanged. It does not allocate or destroy audio buffers.

Queued updates use independent preparation. `Engine::prepare_preset_update` may
reuse an engine's active banks, so its result must be exchanged with that same
engine before rendering resumes. The synchronous `Engine::set_preset` helper is
for control and offline work.

## Host adapters

`plugin/src` adapts native storage, DAW state and audio callbacks. `rust/src/audio`
and `server.rs` adapt desktop audio, state publication and HTTP commands.
`wasm/src` adapts browser workers, asset staging, incremental instrument creation
and telemetry. Each adapter keeps its asset owners separate from `Preset` and
publishes only successfully prepared, accepted changes.

## Extending the code

- Add persistent fields, defaults, migration and normalization to the model.
- Add preparation, playback and buffer retirement to the matching DSP module.
- Share destination and loading rules through `application`; hosts adapt I/O and
  scheduling to those rules.
- Preserve released parameter IDs, their order and serialized preset fields.
  Use the parameter contract and preset round-trip tests when changing them.
- Verify structural updates with the existing audio allocation and rendering
  regressions, including live tails and scene save/recall.

The module tree is being separated incrementally. Some root modules, including
`control` and `imported_fx`, still mix model and runtime responsibilities. A
future extraction should move the actual owner and its callers together and
keep the model independent of DSP.
