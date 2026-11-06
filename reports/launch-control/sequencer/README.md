# XL3 Sequencer verification

Implemented on 2026-11-03. `XL3 · Sequencer` uses the DAW Mixer hardware
surface; DAW Control and Custom Mode gestures return to the existing MIDI
mapping profile. The browser remains the source of the instrument, selected
Neuropolis track, step block and persistent step selection.

Passed `test-xl3-sequencer`, `test-xl3-feedback`, `test-launch-control`,
`test-sequencer-ui`, `test-midi-targets` and `test-midi-learn-ui` with Node.
Built the static WASM site successfully with `wasm/build.py`.

Browser QA used the real application and a simulated two-port XL3 MIDI
connection. Holding step 1 and selecting step 2 in the browser changed both
pitches from C4/G6 to D4/A6 and both Gate lengths from 95% to 96%. Release
preserved both enabled states. Shift + 2 selected steps 17–32, Track navigation
changed the browser's track, and hardware mode reports switched profiles.
There were no browser console errors.

At a 390px mobile viewport, the controller panel had no horizontal overflow;
the two selects and Connect/Refresh button all measured 44px high.
`mobile.jpg` and `desktop.jpg` record the browser checks. These checks simulate
MIDI events; they are not a physical-device test.
