Vendored DSP subset of https://github.com/fushugaku/KekLocator
commit d7901c9dc5b9720819ccd8a2fa14c67a73864586.

SAD PET names this effect SuperEcho. The static Rust DSP and parameter mappings
are retained; reset reuses its buffers. No upstream UI, sequencer or plug-in glue
is included. The pinned ROM is included for reproducible offline builds.
SHA-256: 9cea9f9a998ac7bf05297821290d36ef2843ad72b10f0da583765cc914c58211.

# Source attribution

KekLocator is an unofficial software port of the digital audio processing in
**Erica Synths Echolocator firmware 1.05**. Erica Synths and Echolocator are the
names of the original manufacturer and product. This project is not affiliated
with or endorsed by Erica Synths.

The firmware-derived static Rust translation is in `crates/engine/src/generated`.
The original firmware and factory bank remain manufacturer material; renaming
this software does not change their ownership or grant a license to that material.
The upstream KekLocator repository obtains its pinned source data with
`tools/prepare-firmware.py`; its provenance is recorded in `PORTING.md` and
`crates/engine/firmware/source.json`. This DSP subset includes only the extracted
ROM named above, not the UF2 or factory preset bank.

This repository does not assert a blanket open-source license over the
firmware-derived code or factory data.

