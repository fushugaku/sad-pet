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
let feedbackTrack = 0;
const feedback = new Xl3Feedback({ learn: () => ({ feedback: () => bindings, async refreshFeedback() {},
  track: () => feedbackTrack, tracks: () => Array.from({ length: 6 }, (_, id) => ({ id })) }),
  page: p => pages.push(p), error: m => errors.push(m) });
feedback.configure({ enabled: false, output: { send: bytes => messages.push([...bytes]) }, page: 0 });
assert.equal(messages.length, 0, "feedback ran without permission");
const deviceRows = new Map([[69, 0], [72, 0], [73, 0]]);
const screenFields = new Map();
let screenTime = 0, temporaryScreen = null, stationaryScreen = false;
const visibleScreen = () => {
  const target = temporaryScreen?.until > screenTime ? temporaryScreen.target : stationaryScreen ? 53 : null;
  return target === null ? [] : [0, 1, 2].map(field => screenFields.get(target)?.get(field) || "");
};
const daw = { send: bytes => {
  messages.push([...bytes]);
  if (bytes[0] === 0xf0 && bytes[6] === 6) {
    const fields = screenFields.get(bytes[7]) || new Map();
    fields.set(bytes[8], String.fromCharCode(...bytes.slice(9, -1)));
    screenFields.set(bytes[7], fields);
  }
  if (bytes[0] === 0xf0 && bytes[6] === 4) {
    if (bytes[8] === 127) {
      if (bytes[7] === 53) stationaryScreen = true;
      else temporaryScreen = { target: bytes[7], until: screenTime + 100 };
    } else if (bytes[8] === 0) {
      if (bytes[7] === 53) stationaryScreen = false;
      if (temporaryScreen?.target === bytes[7]) temporaryScreen = null;
    }
  }
  // Observed on the real XL3: mode 2 resets row 1, position feedback does not.
  if (bytes.join() === "182,30,2") deviceRows.set(69, 0);
  if (bytes[0] === 182 && deviceRows.has(bytes[1])) deviceRows.set(bytes[1], bytes[2]);
} }, midi = { send: bytes => positions.push([...bytes]) };
feedback.configure({ enabled: true, output: daw, midiOutput: midi, page: 0 });
await new Promise(resolve => setTimeout(resolve, 0));
assert.ok(messages.some(b => b.join() === "159,12,127"));
assert.ok(messages.some(b => b.join() === "182,30,2"), "live feedback did not select DAW Control");
for (const row of [69, 72, 73]) assert.ok(messages.some(b => b.join() === `182,${row},127`));
assert.deepEqual([...deviceRows.values()], [127, 127, 127], "mode selection reset a configured encoder row");
const rgb = messages.find(b => b[6] === 1 && b[7] === 83 && b[8] === 13);
assert.deepEqual(rgb.slice(9, 12), feedbackRgb("pitch", "Pitch", .5));
assert.deepEqual(positions, [], "DAW feedback wrote into the Custom Mode's main MIDI interface");
assert.ok(messages.some(b => b.join() === "191,5,64"));
assert.ok(!positions.some(b => b[1] === 13), "absolute feedback was sent into the relative MIDI input");
feedback.touch(13); await feedback.sync();
const displayText = messages.filter(b => b[6] === 6 && b[7] === 53 && b[8] === 1);
assert.equal(String.fromCharCode(...displayText.at(-1).slice(9, -1)), "Pitch");
screenTime += 5000;
assert.deepEqual(visibleScreen(), ["Grains", "Pitch", "0.0 st"], "the last parameter expired with the temporary-display timeout");
assert.ok(!messages.some(b => b[6] === 6 && b[7] === 54), "parameters still used the expiring display");
assert.ok(!messages.some(b => b[6] === 4 && b[7] >= 5 && b[7] <= 36 && (b[8] & 0x60)),
  "native touch/change overlays could cover the stationary parameter");
feedback.touch(5); await feedback.sync();
assert.equal(visibleScreen()[1], "Filter 1 cutoff", "the next control did not replace the previous parameter");
screenTime += 5000;
assert.equal(visibleScreen()[1], "Filter 1 cutoff");
feedback.touch(13); await feedback.sync();
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
const originalBindings = structuredClone(bindings);
// Delayed feedback must not re-arm pickup during a fader gesture.
for (const value of [70, 75, 80]) {
  const before = messages.length;
  feedback.faderInput(5, value);
  await feedback.sync();
  assert.ok(!messages.slice(before).some(b => b[0] === 0xbf && b[1] === 5),
    "feedback re-armed pickup during an absolute fader gesture");
}
bindings[1].unit = 80 / 127;
feedback.faders.get(5).time = 0;
const settled = messages.length;
await feedback.sync();
assert.ok(!messages.slice(settled).some(b => b[0] === 0xbf && b[1] === 5), "settled fader echoed itself");
bindings[1].unit = .9;
await feedback.sync();
assert.equal(messages.filter(b => b[0] === 0xbf && b[1] === 5).at(-1)[2], 114,
  "an external edit no longer updated hardware pickup");
bindings = structuredClone(originalBindings);
feedbackTrack = 1;
bindings[0] = { ...bindings[0], parameter: "filter", label: "Cutoff", unit: .25, text: "25%" };
bindings[1] = { ...bindings[1], unit: .25 };
await feedback.sync();
assert.deepEqual(messages.filter(b => b[6] === 1 && b[8] === 13).at(-1).slice(9, 12), feedbackRgb("filter", "Cutoff", .25),
  "switching Track kept the previous parameter's colour");
assert.ok(messages.some(b => b.join() === "191,5,32"), "switching Track did not update fader pickup");
feedback.touch(13); await feedback.sync();
assert.equal(String.fromCharCode(...messages.filter(b => b[6] === 6 && b[7] === 53 && b[8] === 1).at(-1).slice(9, -1)), "Cutoff");
feedbackTrack = 0; bindings = originalBindings;
await feedback.sync();
for (let page = 0; page < 16; page++) {
  feedback.input([0xb6, 30, page < 4 ? page + 6 : page + 14]);
  await new Promise(resolve => setTimeout(resolve, 0));
  assert.equal(pages.at(-1), page);
  assert.equal(deviceRows.get(69), 127, "hardware page change left row 1 absolute");
}
deviceRows.set(69, 0);
feedback.input([0xb6, 30, 2]);
assert.equal(deviceRows.get(69), 127, "a device mode report did not restore the relative format");
feedback.selectPage(2);
assert.equal(feedback.hardwareMode, 2, "page selection left the RGB-capable DAW surface");
assert.ok(messages.some(b => b[6] === 6 && b[7] === 53 && String.fromCharCode(...b.slice(9, -1)) === "Page 3 / Track 1"));
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
const requests = [], received = [], notes = [], actions = [];
const transportState = { playing: false, recording: false };
let activePage = 0, deny = false, learningTarget = null, learnedBindings = [];
const activeTracks = Array(16).fill(0);
const inputs = new MidiInputs({ audio: { async call(...args) { notes.push(args); } },
  midiClock: { input: () => false }, async resume() {} }, {
  navigator: { async requestMIDIAccess(options) {
    requests.push(options.sysex);
    if (deny) throw Object.assign(new Error(), { name: "NotAllowedError" });
    if (options.sysex) return { ...midiAccess, sysexEnabled: true };
    return midiAccess;
  } },
  storage: { getItem() { return JSON.stringify({ mode: "daw" }); }, setItem() {} },
  learn: () => ({ page: () => activePage, selectPage: p => { activePage = p; },
    track: () => activeTracks[activePage], tracks: () => Array.from({ length: 6 }, (_, id) => ({ id, count: 0 })),
    selectTrack: t => { activeTracks[activePage] = t; },
    learning: () => learningTarget,
    input: (...args) => {
      received.push(args);
      if (learningTarget) {
        const pitch = learningTarget.endsWith(":pitch");
        learnedBindings = [{ key: learningTarget, channel: args[0], controller: args[1],
          parameter: pitch ? "pitch" : "filter", label: pitch ? "Pitch" : "Cutoff",
          available: true, unit: .5, text: "0.5" }];
        learningTarget = null;
      }
    }, feedback: () => learnedBindings }),
  ui: () => ({ context: () => ({ active: 0, timbres: [] }), notify: m => errors.push(m),
    transportState: () => transportState,
    async transport(action) {
      actions.push(action);
      await new Promise(resolve => setTimeout(resolve, 0));
      transportState[action === "play" ? "playing" : "recording"] = !transportState[action === "play" ? "playing" : "recording"];
    },
  }), changed() {},
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
const beforeDawInput = received.length;
mainIn.onmidimessage({ data: [0xb0, 13, 65] });
assert.equal(received.length, beforeDawInput, "a transient Custom Mode duplicated DAW input");
for (let page = 0; page < 16; page++) {
  auxiliaryIn.onmidimessage({ data: [0xb6, 30, page < 4 ? page + 6 : page + 14] });
  assert.equal(activePage, page);
  auxiliaryIn.onmidimessage({ data: [0xbf, 77, 65] });
  assert.deepEqual(received.at(-1), [page, 13, 65, "relative", 1], "DAW encoder lost its logical page/channel");
  auxiliaryIn.onmidimessage({ data: [0xbf, 5, 127] });
  assert.deepEqual(received.at(-1), [page, 5, 127]);
  auxiliaryIn.onmidimessage({ data: [0xb0, 37, 127] });
  assert.deepEqual(received.at(-1), [page, page === 15 ? 37 : 80, 127]);
  assert.equal(inputs.feedback.hardwareMode, 2);
}
auxiliaryIn.onmidimessage({ data: [0xb6, 30, 29] });
assert.equal(activePage, 15);
auxiliaryIn.onmidimessage({ data: [0xb0, 106, 127] });
auxiliaryIn.onmidimessage({ data: [0xb0, 106, 127] });
assert.equal(activePage, 14, "held page navigation repeated");
auxiliaryIn.onmidimessage({ data: [0xb0, 106, 0] });
auxiliaryIn.onmidimessage({ data: [0xb0, 106, 127] });
assert.equal(activePage, 13);
for (let track = 1; track < 6; track++) {
  auxiliaryIn.onmidimessage({ data: [0xb0, 102, 127] });
  auxiliaryIn.onmidimessage({ data: [0xb0, 102, 127] });
  assert.equal(activeTracks[13], track, "held Track repeated or skipped a bank");
  auxiliaryIn.onmidimessage({ data: [0xb0, 102, 0] });
}
auxiliaryIn.onmidimessage({ data: [0xb0, 102, 127] });
auxiliaryIn.onmidimessage({ data: [0xb0, 102, 0] });
assert.equal(activeTracks[13], 5);
assert.equal(activePage, 13, "Track navigation changed Mode");
auxiliaryIn.onmidimessage({ data: [0xb0, 103, 127] });
auxiliaryIn.onmidimessage({ data: [0xb0, 103, 0] });
assert.equal(activeTracks[13], 4);
await inputs.request(new URL("http://localhost/api/controller"), { mappingTrack: 2 });
await inputs.feedback.sync();
assert.equal(inputs.status().mappingTrack, 2);
assert.equal(inputs.status().mappingTracks.length, 6);
assert.ok(messages.some(b => b[6] === 6 && b[7] === 53 && String.fromCharCode(...b.slice(9, -1)) === "Page 14 / Track 3"));
auxiliaryIn.onmidimessage({ data: [0xb6, 30, 7] });
assert.equal(activeTracks[activePage], 0);
auxiliaryIn.onmidimessage({ data: [0xb6, 30, 27] });
assert.equal(activePage, 13);
assert.equal(activeTracks[13], 2, "Mode did not restore its selected Track");
const beforeTouch = received.length;
auxiliaryIn.onmidimessage({ data: [0xbe, 13, 127] });
auxiliaryIn.onmidimessage({ data: [0xbf, 13, 100] });
assert.equal(received.length, beforeTouch, "touch or absolute position feedback changed a relative parameter");
// Firmware Shift normally suppresses control changes. Release its preview,
// ignore that feature acknowledgement, and retain the physical modifier.
auxiliaryIn.onmidimessage({ data: [0xb6, 63, 127] });
assert.ok(messages.some(b => b.join() === "182,63,0"), "Shift did not release the firmware's preview-only mode");
auxiliaryIn.onmidimessage({ data: [0xb6, 63, 0] }); // Feature-control acknowledgement.
auxiliaryIn.onmidimessage({ data: [0xbf, 77, 72] });
assert.deepEqual(received.at(-1), [activePage, 13, 72, "relative", 10], "Shift lost accelerated or multi-tick input");
auxiliaryIn.onmidimessage({ data: [0xb6, 63, 0] }); // Physical release.
auxiliaryIn.onmidimessage({ data: [0xbf, 77, 65] });
assert.deepEqual(received.at(-1), [activePage, 13, 65, "relative", 1]);

// Transport bypasses Learn, fires once per press, and serializes quick taps.
const beforeTransportInput = received.length;
learningTarget = "control:0:pitch";
auxiliaryIn.onmidimessage({ data: [0xb0, 116, 127] });
auxiliaryIn.onmidimessage({ data: [0xb0, 116, 127] });
auxiliaryIn.onmidimessage({ data: [0xb0, 116, 0] });
await inputs.transportQueue;
assert.deepEqual(actions, ["play"]);
assert.equal(transportState.playing, true);
await inputs.feedback.sync();
assert.deepEqual(messages.filter(b => b[6] === 1 && b[8] === 116).at(-1).slice(9, 12), [0, 127, 0]);
for (const cc of [116, 116, 118]) {
  auxiliaryIn.onmidimessage({ data: [0xb0, cc, 127] });
  auxiliaryIn.onmidimessage({ data: [0xb0, cc, 0] });
}
await inputs.transportQueue;
assert.deepEqual(actions, ["play", "play", "play", "record"]);
assert.equal(transportState.playing, true, "rapid taps read stale playback state");
assert.equal(transportState.recording, true);
await inputs.feedback.sync();
assert.deepEqual(messages.filter(b => b[6] === 1 && b[8] === 118).at(-1).slice(9, 12), [127, 0, 0]);
assert.equal(received.length, beforeTransportInput, "transport was assigned to a parameter");
assert.equal(learningTarget, "control:0:pitch", "transport consumed the armed MIDI Learn");
learningTarget = null;
auxiliaryIn.onmidimessage({ data: [0xb6, 63, 127] });
auxiliaryIn.onmidimessage({ data: [0x90, 60, 100] });
assert.equal(notes.filter(args => args[3] === "/api/midi-bytes").length, 0, "DAW messages reached the synth");
auxiliaryIn.state = "disconnected";
inputs.refresh();
assert.equal(inputs.shiftHeld, false, "disconnect left the coarse modifier held");
assert.equal(inputs.status().feedback, false);
assert.equal(auxiliaryIn.onmidimessage, null);
assert.equal(typeof mainIn.onmidimessage, "function");
await inputs.request(new URL("http://localhost/api/controller"), { mode: "native" });
assert.equal(inputs.status().feedbackAvailable, false, "Custom Modes advertised unsupported live LEDs");

// Exactly the public Learn flow: no manual controller/profile selection.
// It also recovers if an external MIDI client disabled the device while the
// app's cached mode still says DAW Control.
auxiliaryIn.state = "connected";
await inputs.request(new URL("http://localhost/api/controller"), { device: null });
const learnRequest = new URL("http://localhost/api/controller?learn=1");
learningTarget = "control:0:filter";
await inputs.request(learnRequest);
assert.equal(inputs.status().mode, "daw");
assert.equal(inputs.status().connected, mainIn.name);
assert.equal(inputs.status().feedback, true);
assert.equal(activePage, 13, "automatic Learn changed the selected mapping page");
assert.equal(learningTarget, "control:0:filter", "preparing feedback cancelled Learn");
auxiliaryIn.onmidimessage({ data: [0xbf, 77, 65] });
await inputs.feedback.sync();
assert.equal(learningTarget, null);
assert.deepEqual(messages.filter(b => b[6] === 1 && b[8] === 13).at(-1).slice(9, 12), [0, 69, 6]);
const enables = () => messages.filter(b => b.join() === "159,12,127").length;
const beforeRelearn = enables();
const beforeRelearnMessages = messages.length;
learningTarget = "control:0:pitch";
await inputs.request(learnRequest);
assert.equal(enables(), beforeRelearn + 1, "Learn did not reassert the real hardware configuration");
assert.ok(!messages.slice(beforeRelearnMessages).some(b => b.join() === "159,12,0"),
  "Learn temporarily returned to a stale Custom Mode instead of preserving its page");
auxiliaryIn.onmidimessage({ data: [0xbf, 77, 65] });
await inputs.feedback.sync();
assert.deepEqual(messages.filter(b => b[6] === 1 && b[8] === 13).at(-1).slice(9, 12), [69, 0, 0]);
assert.deepEqual(requests, [false, true, true], "Learn repeatedly requested already-granted MIDI permission");
inputs.dispose();
assert.deepEqual(errors, []);
console.log("XL3 feedback passed: actual DAW Control surface, 16 logical pages, relative rows, pickup, RGB/OLED, mode gestures, independent ports, endpoint refresh and cleanup.");
