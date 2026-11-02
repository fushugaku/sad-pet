import assert from "node:assert/strict";
import { Xl3Feedback, feedbackRgb, parameterColor, matchingOutput, xl3DawPorts } from "./web/xl3-feedback.mjs";
import { MidiInputs } from "./web/midi-inputs.mjs";
const port = name => ({ name, manufacturer: "Novation", state: "connected" });
const input = port("LCXL3 1 MIDI Out"), output = port("LCXL3 1 MIDI In");
const dawIn = port("LCXL3 2 DAW Out"), dawOut = port("LCXL3 2 DAW In");
assert.equal(matchingOutput(input, [output, dawOut]), output);
assert.deepEqual(xl3DawPorts(input, [input, dawIn], [output, dawOut]), { input: dawIn, output: dawOut });
assert.equal(xl3DawPorts(input, [input, dawIn, { ...dawIn }], [dawOut]).input, null);
assert.deepEqual(parameterColor("pitch"), [127, 0, 0]);
assert.deepEqual(parameterColor("filter_resonance"), [0, 25, 127]);
assert.deepEqual(parameterColor("filter", "Filter 1 cutoff"), [0, 127, 12]);
assert.ok(feedbackRgb("pitch", "Pitch", 0)[0] < feedbackRgb("pitch", "Pitch", 1)[0]);
const messages = [], positions = [], errors = [], pages = [];
let bindings = [{ key: "control:0:pitch", parameter: "pitch", label: "Pitch", instrument: "Grains", controller: 13,
  channel: 0, available: true, unit: .5, text: "0.0 st", format: "relative" },
{ key: "control:0:filter", parameter: "filter", label: "Filter 1 cutoff", controller: 5,
  channel: 0, available: true, unit: 8193 / 16383, text: "50%", format: "14bit" }];
const feedback = new Xl3Feedback({ learn: () => ({ feedback: () => bindings, async refreshFeedback() {} }),
  page: p => pages.push(p), error: m => errors.push(m) });
feedback.configure({ enabled: false, output: { send: bytes => messages.push([...bytes]) }, page: 0 });
assert.equal(messages.length, 0, "feedback ran without permission");
const daw = { send: bytes => messages.push([...bytes]) }, midi = { send: bytes => positions.push([...bytes]) };
feedback.configure({ enabled: true, output: daw, midiOutput: midi, page: 0 });
await new Promise(resolve => setTimeout(resolve, 0));
assert.ok(messages.some(b => b.join() === "159,12,127"));
assert.ok(messages.some(b => b.join() === "182,30,6"));
const rgb = messages.find(b => b[6] === 1 && b[7] === 83 && b[8] === 13);
assert.deepEqual(rgb.slice(9, 12), feedbackRgb("pitch", "Pitch", .5));
assert.deepEqual(positions, [[0xb0, 5, 64], [0xb0, 37, 1]]);
assert.ok(!positions.some(b => b[1] === 13), "absolute feedback was sent into the relative MIDI input");
feedback.touch(13); await feedback.sync();
const displayText = messages.filter(b => b[6] === 6 && b[7] === 54 && b[8] === 1);
assert.equal(String.fromCharCode(...displayText.at(-1).slice(9, -1)), "Pitch");
const count = messages.length;
await feedback.sync();
assert.equal(messages.length, count, "unchanged values flooded MIDI");
// At a range edge the device may extinguish a relative LED on movement;
// re-assert its clamped colour even though the value remains unchanged.
feedback.touch(13); await feedback.sync();
assert.ok(messages.length > count);
bindings[0].unit = 1; bindings[0].text = "36.0 st";
await feedback.sync();
assert.deepEqual(messages.filter(b => b[6] === 1 && b[8] === 13).at(-1).slice(9, 12), [127, 0, 0]);
for (let page = 0; page < 16; page++) {
  feedback.input([0xb6, 30, page < 4 ? page + 6 : page + 14]);
  await new Promise(resolve => setTimeout(resolve, 0));
  assert.equal(pages.at(-1), page);
}
feedback.selectPage(2);
assert.ok(messages.some(b => b.join() === "182,30,8"));
feedback.dispose();
assert.ok(messages.some(b => b.join() === "159,12,0"), "disconnect left DAW mode enabled");
const stopped = messages.length;
await feedback.sync();
assert.equal(messages.length, stopped);
assert.deepEqual(errors, []);

// Upgrade an existing keyboard MIDI grant without losing its main input.
// The DAW interface owns feedback/mode reports and never plays synth notes.
const ports = [input, output, dawIn, dawOut].map((p, id) => ({ ...p, id: String(id),
  async close() {}, send: bytes => messages.push([...bytes]) }));
const [mainIn, mainOut, auxiliaryIn, auxiliaryOut] = ports;
const midiAccess = { sysexEnabled: false,
  inputs: new Map([mainIn, auxiliaryIn].map(p => [p.id, p])),
  outputs: new Map([mainOut, auxiliaryOut].map(p => [p.id, p])),
};
const requests = [], received = [], notes = [];
let activePage = 0, deny = false;
const inputs = new MidiInputs({ audio: { async call(...args) { notes.push(args); } },
  midiClock: { input: () => false }, async resume() {} }, {
  navigator: { async requestMIDIAccess(options) {
    requests.push(options.sysex);
    if (deny) throw Object.assign(new Error(), { name: "NotAllowedError" });
    if (options.sysex) return { ...midiAccess, sysexEnabled: true };
    return midiAccess;
  } },
  storage: { getItem() { return JSON.stringify({ mode: "native" }); }, setItem() {} },
  learn: () => ({ page: () => activePage, selectPage: p => { activePage = p; },
    input: (...args) => received.push(args), feedback: () => [] }),
  ui: () => ({ context: () => ({ active: 0, timbres: [] }), notify: m => errors.push(m) }), changed() {},
});
await inputs.access();
assert.equal(inputs.status().feedbackAvailable, true);
assert.equal(inputs.status().feedback, false);
assert.equal(inputs.status().feedbackPermission, false);
deny = true;
await assert.rejects(inputs.access({ sysex: true }), /MIDI access was denied/);
mainIn.onmidimessage({ data: [0xb0, 13, 65] });
assert.deepEqual(received.at(-1), [0, 13, 65, "relative"], "permission denial removed the MIDI input");
deny = false;
await inputs.access({ sysex: true });
assert.equal(inputs.status().feedback, true);
assert.deepEqual(requests, [false, true, true]);
auxiliaryIn.onmidimessage({ data: [0xb6, 30, 29] });
assert.equal(activePage, 15);
auxiliaryIn.onmidimessage({ data: [0x90, 60, 100] });
assert.equal(notes.filter(args => args[3] === "/api/midi-bytes").length, 0, "DAW messages reached the synth");
auxiliaryIn.state = "disconnected";
inputs.refresh();
assert.equal(inputs.status().feedback, false);
assert.equal(auxiliaryIn.onmidimessage, null);
assert.equal(typeof mainIn.onmidimessage, "function");
inputs.dispose();
assert.deepEqual(errors, []);
console.log("XL3 feedback passed: port pairing, RGB families/brightness, OLED names, 14-bit positions, endpoint refresh, deduplication, all mode reports and cleanup.");
