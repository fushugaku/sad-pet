DSP subset of https://github.com/fushugaku/dayverb
commit c8b27d3cf2c981b8efeadbee4d4cb21d5af1820d.

SAD PET names this effect AlphaVerb. The original static Rust translation,
parameter callbacks, memory adapter and pinned source images are retained.
The C ABI, standalone UI, sequencer and plug-in wrappers are excluded.
`presets.json` is copied unchanged from `web/presets.json` at the revision above:
30 original factory presets and 15 gated reverb presets from firmware 1.02.
SHA-256: 61a77ea316a1d54637e84d642877b9d2be0a2fa63f893fac7c41a57490252d0b.
SAD PET maps all 20 saved controls to AlphaVerb, with freeze off and the original
default 50% mix.

SAD PET's build joins the original blocks of the two hot delay-line routines
with direct calls. Their arithmetic, state writes and error checks are retained;
the original block dispatcher remains the differential-test reference.

Dayverb is an unofficial software port of the digital processing in Erica Synths /
112dB Nightverb firmware 1.02. This software is not affiliated with or endorsed by
Erica Synths or 112dB. The firmware-derived code and data remain the property of
their original rights holders; renaming does not grant additional rights.
No blanket open-source license over the firmware-derived material is asserted.

Firmware source: https://www.ericasynths.lv/service/file/download/product_id/1149/file_id/626/
Extracted flash SHA-256: 67229c75ff8bc92d90fb4c625fdcce50463252ac939b77db6d754584ede518a8.
