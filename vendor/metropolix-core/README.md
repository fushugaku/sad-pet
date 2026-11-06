# metropolix-core

Vendored from https://github.com/fushugaku/neuropolis at revision
`872a28c70de0cc6264dfab84aad8b78a2f224395` (crate version 0.2.2).
The upstream AGPL-3.0-only license is included in LICENSE. Runtime source and
standalone sequencer regression tests are retained; firmware fixtures are omitted.

SAD PET adds `Step::ratchet_span`: zero preserves the upstream scheduler exactly.
Values 1–8 emit one ratchet burst per stage over that many clock pulses, capped
by the stage length. This override applies only to ratcheted, non-held gates;
pulse/stage clocks remain unchanged. Duration and count are captured at stage
entry, so live span edits take effect on the next stage visit. The 96 PPQN clock
limits distinct ratchet onsets to one per tick.
