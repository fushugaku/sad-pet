# SAD PET · Launch Control XL3

Exported from Novation Components. Send each `.syx` file to the matching slot,
then select **XL3 · 16 modes** in SAD PET.

| Controls | MIDI messages |
| --- | --- |
| 24 encoders | CC13–36, Relative (offset binary, centre 64) |
| 8 faders | CC5–12 MSB + CC37–44 LSB, 14-bit absolute |
| 16 buttons | CC64–79, momentary, on 127 / off 0 |

All 48 controls in mode N explicitly use MIDI channel N. Modes 1–15 are
editable. Factory mode 16 already uses channel 16 and cannot be replaced;
its encoders and faders keep Novation's 7-bit absolute format.

Pitch moves 0.1 st per relative encoder tick. The XL3's global Encoder Curve
setting controls acceleration: choose Slow on the hardware for fine turns.

Vendor references:
- https://support.novationmusic.com/hc/en-gb/articles/27203903097362-Launch-Control-XL-3-Components-guide
- https://support.novationmusic.com/hc/en-gb/articles/38440851693458-Using-Relative-Encoders
- https://userguides.novationmusic.com/hc/en-gb/articles/26260540230162-Launch-Control-XL-3-appendix

`manifest.json` records the exported files and their SHA-256 hashes.
