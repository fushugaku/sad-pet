import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";
import { Xl3Sequencer } from "./web/xl3-sequencer.mjs";
import { MidiInputs } from "./web/midi-inputs.mjs";

// Use the shipping editor's addressing, scale quantisation and edit handler.
const scope = { window: {}, document: {} };
vm.runInNewContext(await readFile(new URL("../rust/web/sequence.js", import.meta.url), "utf8"), scope);
const step = { pitch: 60, gate: 2, gate_override: -1, velocity: 100, ratchet: 1,
  slide: false, pulses: 1, skip: false, destination1: 0, destination2: 0, destination3: 0 };
const lane = id => ({ id, name: `Track ${id}`, target: { kind: "notes" }, pages: 8,
  config: Array.from({ length: 43 }, (_, i) => ({ 1: 16, 7: 55, 41: 127 }[i] || 0)),
  destinations: ["filter", "pitch", "lfo1_reset"],
  steps: Array.from({ length: 128 }, () => ({ ...step })) });
let preset = { sequencer: { enabled: true, tracks: [lane(1), lane(2)] } }, instrument = 1;
const edits = [], editor = Object.create(scope.window.SadNeuropolis.prototype);
Object.assign(editor, { selected: 1, visiblePage: 0, selection: new Set(), catalog: {},
  hooks: { read: () => preset, edit: edit => edits.push(edit), view() {} },
  sync() {}, syncSelection() {}, depthText: (_, value) => `${value * 100}%`,
});
const source = () => ({ editor, context: instrument, instrument: `Instrument ${instrument}`,
  playing: true, view: { count: 2, tracks: [{ id: 1, step: 2 }, { id: 2, step: 0 }] } });
const sequencer = new Xl3Sequencer(source), track = () => editor.track();
sequencer.button(0, 127, false);
sequencer.button(0, 127, false);
assert.equal(track().steps[0].gate, 2, "holding immediately muted the note being edited");
assert.ok(editor.controllerHeld.has(0));
assert.equal(track().steps[0].skip, false);
sequencer.button(0, 0, false);
assert.equal(track().steps[0].gate, 0, "tap did not switch the step off");
sequencer.button(0, 0, false);
assert.equal(track().steps[0].gate, 0, "duplicate release switched it twice");

// Every sounding gate type switches to Rest; Rest always returns to Multi.
for (const gate of [1, 2, 3]) {
  track().steps[0].gate = gate;
  sequencer.button(0, 127, false); sequencer.button(0, 0, false);
  assert.equal(track().steps[0].gate, 0);
  sequencer.button(0, 127, false); sequencer.button(0, 0, false);
  assert.equal(track().steps[0].gate, 2);
  assert.equal(track().steps[0].skip, false, "gate toggle edited Skip");
}
track().steps[0].gate = 0;
editor.selection = new Set([0, 3]);
sequencer.button(0, 127, false); sequencer.button(1, 127, false);
sequencer.turn(29, 65);
assert.deepEqual([0, 1, 2, 3].map(n => track().steps[n].pitch), [61, 61, 60, 61],
  "held and browser-selected steps were not a deduplicated union");
sequencer.button(0, 0, false); sequencer.button(1, 0, false);
assert.equal(track().steps[0].gate, 0, "editing toggled a held step on release");
assert.equal(track().steps[1].gate, 2);
assert.deepEqual([...editor.selection], [0, 3], "hardware changed the browser's selection");
editor.selection = new Set([0]);
sequencer.turn(30, 65); assert.equal(track().steps[0].gate_override, 56);
sequencer.turn(31, 127); assert.equal(track().steps[0].velocity, 127);
sequencer.turn(32, 127); assert.equal(track().steps[0].ratchet, 8);
sequencer.turn(32, 1); assert.equal(track().steps[0].ratchet, 1);
sequencer.turn(33, 65); assert.equal(track().steps[0].slide, true);
sequencer.turn(33, 63); assert.equal(track().steps[0].slide, false);
track().steps[0].gate = 2;
sequencer.turn(34, 65); assert.equal(track().steps[0].gate, 3);
sequencer.turn(34, 65); assert.equal(track().steps[0].gate, 3);
sequencer.turn(34, 1); assert.equal(track().steps[0].gate, 0);
sequencer.turn(35, 127); assert.equal(track().steps[0].pulses, 8);
sequencer.turn(21, 65); sequencer.turn(21, 65);
assert.equal(track().steps[0].destination1, .02);
sequencer.turn(22, 1); assert.equal(track().steps[0].destination2, -.63);
sequencer.turn(23, 65); assert.equal(track().steps[0].destination3, 1);
sequencer.turn(23, 63); assert.equal(track().steps[0].destination3, 0);
const beforeUnused = edits.length;
sequencer.turn(13, 65); sequencer.turn(36, 65); sequencer.turn(29, 64);
assert.equal(edits.length, beforeUnused, "an unused encoder or neutral tick edited a step");

const previous = preset;
preset = structuredClone(preset);
sequencer.turn(29, 65);
assert.equal(track().steps[0].pitch, 62);
assert.equal(previous.sequencer.tracks[0].steps[0].pitch, 61, "controller mutated a stale snapshot");
track().target = { kind: "oscillator", index: 0 };
track().steps[0].pitch = 84;
sequencer.turn(29, 65); assert.equal(track().steps[0].pitch, 84, "OSC pitch exceeded +24 semitones");
track().steps[0].pitch = 36;
sequencer.turn(29, 63); assert.equal(track().steps[0].pitch, 36);
track().target = { kind: "notes" };
track().notes = (1 << 0) | (1 << 4) | (1 << 7);
track().config[40] = 60; track().config[41] = 72; track().steps[0].pitch = 60;
sequencer.turn(29, 65); assert.equal(track().steps[0].pitch, 64, "Pitch ignored the selected scale");
sequencer.turn(29, 127); assert.equal(track().steps[0].pitch, 72, "Pitch exceeded the track's octave range");
track().notes = 0; track().config[40] = 0; track().config[41] = 127; track().steps[0].pitch = 60;

for (let page = 0; page < 8; page++) {
  sequencer.button(page, 127, true); sequencer.button(page, 0, true);
  assert.equal(editor.offset(track()), page * 16);
  const before = track().steps[page * 16].pitch;
  sequencer.turn(29, 65);
  assert.equal(track().steps[page * 16].pitch, before + 1);
  sequencer.button(15, 127, false); sequencer.button(15, 0, false);
  assert.equal(track().steps[page * 16 + 15].gate, 0, "step buttons did not follow the chosen block");
}
const beforeInvalidPage = editor.visiblePage;
sequencer.button(15, 127, true); sequencer.button(15, 0, true);
assert.equal(editor.visiblePage, beforeInvalidPage, "an unavailable block extended the pattern");
editor.visiblePage = 0;
sequencer.button(1, 127, true);
const gateAfterPage = track().steps[17].gate;
sequencer.button(1, 127, false); // Repeated press after Shift was released first.
sequencer.button(1, 0, false);
assert.equal(track().steps[17].gate, gateAfterPage, "a repeating page button toggled a note after Shift release");
editor.visiblePage = 0; editor.selection = new Set();
sequencer.button(4, 127, false);
editor.selected = 2;
sequencer.button(4, 0, false);
assert.equal(track().steps[4].gate, 2, "a release from the previous track edited the new track");
sequencer.button(4, 127, false);
instrument = 2;
sequencer.button(4, 0, false);
assert.equal(track().steps[4].gate, 2, "same track IDs across instruments reused a held button");
sequencer.button(4, 127, false);
editor.showPage(1);
sequencer.button(4, 0, false);
assert.equal(track().steps[20].gate, 2, "browser page change redirected a held release");
sequencer.navigate(-1);
assert.equal(editor.selected, 1, "Track navigation did not change the browser selection");
sequencer.navigate(-1); assert.equal(editor.selected, 1);

editor.visiblePage = 0; editor.selection = new Set([5]);
let surface = sequencer.feedback();
assert.deepEqual(surface.colors.get(37 + 2), [127, 127, 127], "playhead did not follow the engine");
assert.deepEqual(surface.colors.get(37 + 5), [0, 100, 127], "browser selection did not light the pad");
assert.equal(surface.controls.get(29).label, "Pitch");
assert.equal(surface.controls.has(13), false, "top row was assigned instead of the row above the faders");
surface = sequencer.feedback(true);
assert.deepEqual(surface.colors.get(37), [100, 100, 127]);
assert.deepEqual(surface.colors.get(45), [0, 0, 0]);

// Exercise the real port owner, mode reports, Shift preview handshake and LEDs.
const messages = [], learned = [], errors = [];
const port = (id, name) => ({ id, name, manufacturer: "Novation", state: "connected",
  send: data => messages.push([...data]), async close() {} });
const main = port("main", "LCXL3 1 MIDI Out"), daw = port("daw", "LCXL3 2 DAW Out");
const out = port("out", "LCXL3 1 MIDI In"), dawOut = port("dawout", "LCXL3 2 DAW In");
const access = { sysexEnabled: true, inputs: new Map([[main.id, main], [daw.id, daw]]),
  outputs: new Map([[out.id, out], [dawOut.id, dawOut]]) };
const storage = new Map(); let page = 0;
const timbres = { active: 1, timbres: [{ id: 1, name: "First", kind: "granular" },
  { id: 2, name: "Second", kind: "granular" }] };
const midi = new MidiInputs({ audio: { async call() {} }, midiClock: { input: () => false } }, {
  navigator: { async requestMIDIAccess() { return access; } },
  storage: { getItem: key => storage.get(key), setItem: (key, value) => storage.set(key, value) }, changed() {},
  learn: () => ({ page: () => page, selectPage: p => { page = p; }, input: (...args) => learned.push(args) }),
  ui: () => ({ sequencer: source, context: () => timbres,
    async change(path, data) { assert.equal(path, "/api/timbres/select"); timbres.active = data.id; instrument = data.id; },
    notify: error => errors.push(error) }),
});
await midi.request(new URL("http://localhost/api/controller"), { mode: "sequencer" });
assert.equal(midi.feedback.hardwareMode, 1, "Sequencer was another MIDI mapping page");
assert.equal(midi.status().feedbackAvailable, true);
const send = (...data) => daw.onmidimessage({ data });
send(0xb6, 30, 1); // Acknowledge the initial programmatic mode selection.
editor.selection = new Set();
send(0xb0, 38, 127); send(0xbf, 93, 65); send(0xb0, 38, 0);
assert.equal(track().steps[1].pitch, 62);
assert.equal(track().steps[1].gate, 2);
send(0xb6, 63, 127); send(0xb6, 63, 0); // Feature acknowledgement, Shift stays held.
send(0xb0, 39, 127); send(0xb0, 39, 0);
assert.equal(editor.visiblePage, 2, "Shift + 3 did not select steps 33-48");
send(0xb6, 63, 0); // Physical release.
send(0xb0, 37, 127); send(0xb0, 37, 0);
assert.equal(track().steps[32].gate, 0);
assert.deepEqual(learned, [], "Sequencer controls leaked into MIDI Learn");
send(0xb0, 107, 127); send(0xb0, 107, 0);
await midi.surface.selections;
assert.equal(timbres.active, 2, "Page navigation did not select the instrument in the browser");
await midi.feedback.sync();
const beforeSteady = messages.length;
await midi.feedback.sync();
assert.equal(messages.length, beforeSteady, "steady Sequencer feedback flooded MIDI");
send(0xb6, 30, 2);
assert.equal(midi.status().mode, "daw");
send(0xbf, 93, 65); assert.equal(learned.length, 1);
// Simulate a reconnect/refresh that commands DAW Control, without delivering
// its acknowledgement until after the following physical Mode gesture.
midi.feedback.hardwareMode = null;
midi.feedback.selectPage(page);
send(0xb6, 30, 1); assert.equal(midi.status().mode, "sequencer");
// A refresh can command DAW Control before a physical Mode gesture. Its
// asynchronous acknowledgement must not undo the user's newer choice.
send(0xb6, 30, 2);
assert.equal(midi.status().mode, "sequencer", "late DAW Control acknowledgement exited Sequencer");
send(0xb6, 30, 1);
assert.equal(midi.status().mode, "sequencer");
send(0xb6, 30, 7); assert.equal(midi.status().mode, "daw"); assert.equal(page, 1);
await midi.request(new URL("http://localhost/api/controller"), { mode: "sequencer" });
assert.equal(page, 1, "entering Sequencer changed the saved MIDI mapping page");
send(0xb0, 37, 127);
daw.state = "disconnected"; midi.refresh();
assert.equal(editor.controllerHeld.size, 0, "USB disconnect left a step held");
assert.equal(midi.sequencer.held.size, 0);
midi.dispose();
assert.deepEqual(errors, []);
console.log("XL3 Sequencer passed: held + browser selection, all ten controls, eight step blocks, tap/hold gestures, snapshot and context safety, hardware mode separation, RGB/playhead and USB cleanup.");
