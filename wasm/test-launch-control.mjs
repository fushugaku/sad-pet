import assert from "node:assert/strict";
import { LaunchControl, launchControlModel } from "./web/launch-control.mjs";
import { MidiInputs } from "./web/midi-inputs.mjs";

const part = (id, kind = "rack") => ({ id, kind, name: `Timbre ${id}`, gain: 1,
  muted: false, solo: false, sends: Array(8).fill(0) });
const context = { active: 0, timbres: Array.from({ length: 10 }, (_, i) => part(i)) };
context.timbres.push(...Array.from({ length: 5 }, (_, i) => ({ ...part(20 + i, "bus"), bus: i + 1 })),
  part(30, "master"), part(31, "control"), part(32, "empty"), part(33, "pad_fx"));
const changes = [], errors = [], leds = [];
const ui = {
  context: () => context,
  async change(path, data, scalar) {
    changes.push({ path, data: structuredClone(data), scalar });
    if (path.endsWith("select")) context.active = data.id;
    else Object.assign(context.timbres.find(p => p.id === data.id), data);
  },
  notify: message => errors.push(message),
};
const surface = new LaunchControl({ ...ui, error: ui.notify });
surface.configure({ output: { send: bytes => leds.push(bytes) } });
const cc = (n, v, channel = 8) => surface.input([0xb0 | channel, n, v]);
const press = n => { surface.input([0x98, n, 127]); surface.input([0x88, n, 0]); };
const zero = context.timbres[0];

// Absolute controls use pickup, with channel isolation and coalesced bursts.
cc(77, 0);
await surface.flush();
assert.equal(zero.gain, 1);
assert.match(surface.status().pickup, /Fader 1: move up to 50%/);
cc(77, 64, 0);
assert.equal(zero.gain, 1);
cc(77, 64); cc(77, 100);
await surface.flush();
assert.equal(zero.gain, 200 / 127);
assert.equal(changes.length, 1, "hardware bursts were not coalesced");
assert.equal(changes[0].data.id, 0);
assert.equal(surface.status().pickup, "");

// Mouse edits and scene recalls re-arm pickup; crossing need not be exact.
zero.gain = 0.2;
cc(77, 110);
assert.equal(zero.gain, 0.2);
cc(77, 0);
await surface.flush();
assert.equal(zero.gain, 0);

// Three rows address existing buses and preserve the other rows' sends.
cc(13, 0); cc(13, 40);
cc(29, 0); cc(29, 60);
cc(49, 0); cc(49, 80);
await surface.flush();
assert.deepEqual(zero.sends.slice(0, 3), [80 / 127, 120 / 127, 160 / 127]);
assert.equal(surface.status().sends.length, 3);
cc(105, 127); cc(105, 0);
assert.equal(surface.sendBank, 1);
cc(13, 0); cc(13, 50);
await surface.flush();
assert.equal(zero.sends[3], 100 / 127);
cc(49, 127);
assert.equal(zero.sends[4], 0, "a missing third bus changed a different send");

// Focus, mute/solo, note-off and repeated button messages.
press(42);
await surface.selections;
assert.equal(context.active, 1);
surface.input([0x98, 74, 127]);
surface.input([0x98, 74, 127]);
surface.input([0x98, 74, 0]);
await surface.flush();
assert.equal(context.timbres[1].muted, true);
press(107); press(74);
await surface.flush();
assert.equal(context.timbres[1].solo, true);
assert.equal(context.timbres[1].muted, true);

// Bank by position, address by stable ID; buses and Control take no columns.
cc(107, 127); cc(107, 0);
assert.deepEqual(surface.status().tracks.map(p => p.id), [8, 9]);
cc(77, 0);
assert.equal(context.timbres[8].gain, 1, "a bank change kept the old pickup");
cc(77, 64); cc(77, 127);
await surface.flush();
assert.equal(context.timbres[8].gain, 2);
press(41);
await surface.selections;
assert.equal(context.active, 8);
surface.sync();
const count = leds.length;
surface.status(); surface.sync();
assert.equal(leds.length, count, "unchanged polls flooded the MIDI output");
assert.ok(leds.some(b => b[0] === 0x98 && b[1] === 41 && b[2] === 60));

// Removed targets never redirect pending edits to another track.
cc(78, 64);
context.timbres = context.timbres.filter(p => p.id !== 9);
const beforeRemove = changes.length;
await surface.flush();
assert.equal(changes.length, beforeRemove);
surface.configure({ mode: "learn" });
cc(77, 0);
assert.equal(context.timbres[8].gain, 2);

// XL3's immutable Custom Mode 16 uses different faders and button CCs.
surface.bank = 0;
surface.configure({ model: "xl3", mode: "mixer", output: null });
surface.input([0xbf, 5, 0]); surface.input([0xbf, 5, 90]);
await surface.flush();
assert.equal(zero.gain, 180 / 127);
surface.input([0xbf, 38, 127]); surface.input([0xbf, 38, 0]);
await surface.selections;
assert.equal(context.active, 1);
surface.dispose();

// Audio replies replace the context while the next two knob edits are still
// in flight. All three send rows must survive those older replies.
let delayedContext = { active: 0, timbres: [part(0),
  ...Array.from({ length: 3 }, (_, i) => ({ ...part(10 + i, "bus"), bus: i + 1 }))] };
const replies = [], delayedWrites = [];
const delayed = new LaunchControl({ context: () => delayedContext, error: ui.notify,
  async change(path, data) {
    delayedWrites.push(structuredClone(data));
    await new Promise(resolve => replies.push(resolve));
    delayedContext = { ...delayedContext,
      timbres: delayedContext.timbres.map(p => p.id === data.id ? { ...p, ...data } : p) };
  },
});
const turn = (n, v) => delayed.input([0xb8, n, v]);
turn(13, 0); turn(13, 40);
const writing = delayed.flush();
turn(29, 0); turn(29, 50);
replies[0]();
await new Promise(resolve => setTimeout(resolve, 0));
turn(49, 0); turn(49, 60);
replies[1]();
await new Promise(resolve => setTimeout(resolve, 0));
replies[2]();
await writing;
assert.deepEqual(delayedContext.timbres[0].sends.slice(0, 3), [80 / 127, 100 / 127, 120 / 127]);
assert.equal(delayedWrites.length, 3);
assert.equal(delayed.optimistic.size, 0);
delayed.dispose();
for (const name of ["Launch Control XL", "LaunchControl XL 2"])
  assert.equal(launchControlModel(name), "xl");
for (const name of ["Launch Control XL 3 MIDI", "LCXL3 MIDI"])
  assert.equal(launchControlModel(name), "xl3");
for (const name of ["LaunchControl XL HUI", "Launch Control XL 3 DAW", "Some keyboard"])
  assert.equal(launchControlModel(name), null);

// Shared Web MIDI lifecycle with two surfaces, HUI and a separate keyboard.
const port = (id, name) => ({ id, name, manufacturer: "Novation", state: "connected",
  async close() {}, send: bytes => leds.push(bytes) });
const keyboard = port("keys", "Keyboard"), controller = port("xl", "Launch Control XL");
const other = port("other", "Launch Control XL 2"), hui = port("hui", "LaunchControl XL HUI");
const output = port("out", controller.name), calls = [], learned = [], stored = new Map();
const access = { inputs: new Map([keyboard, controller, other, hui].map(p => [p.id, p])),
  outputs: new Map([[output.id, output]]) };
let requests = 0, consume = false, changed = 0, mappingPage = 0;
const nav = {
  async requestMIDIAccess(options) { requests++; assert.deepEqual(options, { sysex: false }); return access; },
  permissions: { async query() { return { state: "prompt" }; } },
};
const backend = { state: { timbres: context }, audio: { async call(...args) { calls.push(args); } },
  midiClock: { input: e => e.data[0] === 0xf8 }, async resume() {} };
const dependencies = { navigator: nav, ui: () => ui,
  learn: () => ({ bytes: data => { learned.push([...data]); return consume; },
    page: () => mappingPage, selectPage: p => { mappingPage = p; },
    pages: () => Array.from({ length: 16 }, (_, id) => ({ id, count: 0 })),
  }),
  storage: { getItem: k => stored.get(k), setItem: (k, v) => stored.set(k, v) },
  changed: () => changed++,
};
const midi = new MidiInputs(backend, dependencies);
const url = new URL("http://localhost/api/controller"), ask = new URL(`${url}?request=1`);
await midi.restore(); await midi.request(url);
assert.equal(requests, 0, "page load requested MIDI permission");
await Promise.all([midi.access(), midi.access()]);
assert.equal(requests, 1, "simultaneous requests opened two permission prompts");
assert.equal(midi.controller, controller);
assert.equal(midi.surface.output, output);
assert.equal(midi.status().ports.length, 2, "HUI was treated as a control surface");
await midi.keyboard(url, { port: 0 });
assert.equal(backend.selectedMidi, keyboard);
assert.ok(!((await midi.keyboard(url)).ports.some(p => p.name === controller.name)));
const engineCount = calls.filter(c => c[3] === "/api/midi-bytes").length;
controller.onmidimessage({ data: [0x98, 41, 127] });
controller.onmidimessage({ data: [0x88, 41, 0] });
await midi.surface.selections;
assert.equal(calls.filter(c => c[3] === "/api/midi-bytes").length, engineCount,
  "a control surface button played a synth note");
consume = true;
const oldGain = zero.gain;
controller.onmidimessage({ data: [0xb8, 77, 0] });
assert.equal(zero.gain, oldGain, "the mixer also acted on a learned CC");
consume = false;
keyboard.onmidimessage({ data: [0x90, 60, 100] });
await new Promise(resolve => setTimeout(resolve, 0));
assert.deepEqual(calls.filter(c => c[3] === "/api/midi-bytes").at(-1)[1], [0x90, 60, 100]);
const clocks = calls.length;
keyboard.onmidimessage({ data: [0xf8] });
assert.equal(calls.length, clocks, "clock leaked into note handling");

await midi.request(url, { device: controller.id, mode: "learn" });
const learnedBeforePages = learned.length;
for (const [page, number] of [41, 42, 43, 44, 57, 58, 59, 60, 73, 74, 75, 76, 89, 90, 91, 92].entries()) {
  controller.onmidimessage({ data: [0x98, number, 127] });
  controller.onmidimessage({ data: [0x88, number, 0] });
  assert.equal(mappingPage, page, `legacy button ${number} chose the wrong page`);
}
controller.onmidimessage({ data: [0x98, 92, 127] });
controller.onmidimessage({ data: [0x88, 92, 0] });
assert.equal(midi.status().page, 15, "the eighth lower button did not select Page 16");
controller.onmidimessage({ data: [0xb8, 107, 127] });
controller.onmidimessage({ data: [0xb8, 107, 0] });
assert.equal(mappingPage, 15, "page navigation wrapped past Page 16");
controller.onmidimessage({ data: [0xb8, 106, 127] });
controller.onmidimessage({ data: [0xb8, 106, 127] });
controller.onmidimessage({ data: [0xb8, 106, 0] });
assert.equal(mappingPage, 14, "a repeated press switched more than one page");
assert.equal(learned.length, learnedBeforePages, "page navigation was learned as a parameter");
assert.ok(leds.some(bytes => bytes[0] === 0x98 && bytes[1] === 92 && bytes[2] === 60));
assert.equal(midi.status().pages.length, 16);
await midi.request(url, { page: 3 });
assert.equal(mappingPage, 3);
await assert.rejects(midi.request(url, { page: 16 }), /page from 1 to 16/);
assert.equal(mappingPage, 3);
// XL3 selects the same sixteen pages through CC buttons on channel 16.
midi.surface.configure({ model: "xl3", mode: "learn", page: 3, output: null });
for (let page = 0; page < 16; page++) {
  controller.onmidimessage({ data: [0xbf, 37 + page, 127] });
  controller.onmidimessage({ data: [0xbf, 37 + page, 0] });
  assert.equal(mappingPage, page, `XL3 button ${37 + page} chose the wrong page`);
}
controller.onmidimessage({ data: [0xbf, 52, 127] });
controller.onmidimessage({ data: [0xbf, 52, 0] });
assert.equal(mappingPage, 15);
controller.onmidimessage({ data: [0xbf, 37, 127] });
controller.onmidimessage({ data: [0xbf, 37, 0] });
assert.equal(mappingPage, 0);
assert.equal(learned.length, learnedBeforePages);
midi.refresh();
controller.state = "disconnected";
access.onstatechange();
assert.equal(midi.controller, null);
assert.equal(controller.onmidimessage, null);
assert.equal(backend.selectedMidi, keyboard, "controller unplug stopped the keyboard");
const replacement = port(controller.id, controller.name);
access.inputs.set(controller.id, replacement);
access.onstatechange();
assert.equal(midi.controller, replacement);
assert.equal(typeof replacement.onmidimessage, "function");
assert.equal(midi.status().mode, "learn");
await midi.request(url, { device: null });
assert.equal(replacement.onmidimessage, null);
await assert.rejects(midi.keyboard(url, { port: 1 }), /keyboard input/);
access.onstatechange();
assert.equal(midi.controller, null, "Off auto-connected again");
const remembered = new MidiInputs(backend, dependencies);
assert.equal(remembered.settings.enabled, false);
assert.equal(remembered.settings.mode, "learn");
await assert.rejects(midi.request(url, { device: "missing" }), /no longer connected/);
assert.equal(midi.settings.enabled, false);
assert.ok(changed > 0);
midi.dispose(); remembered.dispose();
assert.equal(keyboard.onmidimessage, null);
assert.equal(access.onstatechange, null);
await midi.access();
assert.equal(typeof keyboard.onmidimessage, "function", "returning from pagehide did not restore input");
assert.equal(typeof access.onstatechange, "function");
midi.dispose();

const denied = new MidiInputs({ ...backend, midiAccess: null }, { ...dependencies,
  navigator: { async requestMIDIAccess() { throw Object.assign(new Error(), { name: "NotAllowedError" }); } },
});
await assert.rejects(denied.request(ask), /MIDI access was denied/);
assert.match(denied.status().error, /MIDI access was denied/);
assert.equal(denied.accessPending, null, "denial blocked the retry");
denied.dispose();

// Prepared XL3 modes route by channel. Both halves of a 14-bit fader must
// arrive before any value is applied; buttons remain available for Learn.
const nativeValues = [], nativePages = [], nativePort = port("native", "LCXL3 1 MIDI Out");
let nativePage = 0, migration = 0, armed = "pitch";
const nativeLearner = { page: () => nativePage,
  selectPage: (page, options) => { nativePage = page; nativePages.push([page, options]); if (!options?.preserveLearning) armed = null; },
  input: (...args) => { nativeValues.push(args); armed = null; },
  configureXL3: () => migration++,
};
const native = new MidiInputs(backend, { ...dependencies, learn: () => nativeLearner });
await native.request(url, { mode: "native" });
native.controller = nativePort;
native.surface.configure({ model: "xl3", mode: "native" });
const sendNative = (channel, number, value) => native.message(nativePort, { data: [0xb0 | channel, number, value] });
sendNative(2, 13, 65);
assert.equal(nativePage, 2);
assert.deepEqual(nativePages.at(-1), [2, { preserveLearning: true }]);
assert.deepEqual(nativeValues.at(-1), [2, 13, 65, "relative"]);
assert.equal(migration, 1);
assert.equal(armed, null);
const beforePair = nativeValues.length;
sendNative(2, 5, 63);
assert.equal(nativeValues.length, beforePair, "a fader emitted its coarse MSB before its LSB");
sendNative(2, 37, 127);
assert.deepEqual(nativeValues.at(-1), [2, 5, 8191, "14bit"]);
sendNative(2, 5, 64);
assert.equal(nativeValues.length, beforePair + 1, "an MSB crossing emitted an intermediate jump");
sendNative(2, 37, 0);
assert.deepEqual(nativeValues.at(-1), [2, 5, 8192, "14bit"]);
sendNative(2, 37, 1);
await new Promise(resolve => setTimeout(resolve, 8));
assert.deepEqual(nativeValues.at(-1), [2, 5, 8193, "14bit"], "LSB-only motion was lost");
sendNative(2, 38, 127); sendNative(2, 6, 127);
assert.deepEqual(nativeValues.at(-1), [2, 6, 16383, "14bit"], "reversed byte order was not supported");
sendNative(2, 64, 127);
assert.deepEqual(nativeValues.at(-1), [2, 64, 127]);
sendNative(15, 13, 64);
assert.equal(nativePage, 15);
assert.deepEqual(nativeValues.at(-1), [15, 13, 64], "immutable Mode 16 was interpreted as relative");
sendNative(0, 106, 127);
assert.equal(nativePage, 15, "a global side button selected an unrelated native mode");
sendNative(2, 5, 65); sendNative(2, 37, 0);
sendNative(2, 37, 1);
native.dispose();
const disposedCount = nativeValues.length;
await new Promise(resolve => setTimeout(resolve, 8));
assert.equal(nativeValues.length, disposedCount, "a disposed fader retained its pending write");
const nativeRestored = new MidiInputs(backend, dependencies);
assert.equal(nativeRestored.settings.mode, "native");
nativeRestored.dispose();
assert.deepEqual(errors, []);
console.log("Launch Control passed: Mk1/Mk2 + XL3, pickup, mixer, banks, coalescing, LEDs, Learn priority, independent keyboard, USB reconnect, permission and saved settings.");
