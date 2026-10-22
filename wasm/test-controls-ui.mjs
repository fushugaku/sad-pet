import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const document = { activeElement: null };
class Element {
  constructor(tag) {
    this.tagName = tag.toUpperCase();
    this.children = [];
    this.nodes = new Map();
    this.attributes = new Map();
    this.dataset = {};
    this.events = {};
    this.style = {};
    this.classList = { add() {}, remove() {}, toggle() {} };
  }
  append(...nodes) {
    for (const node of nodes) node.parentNode = this;
    this.children.push(...nodes);
  }
  replaceChildren(...nodes) { this.children = nodes; }
  querySelector(selector) {
    if (!this.nodes.has(selector)) this.nodes.set(selector, new Element("div"));
    return this.nodes.get(selector);
  }
  setAttribute(key, value) { this.attributes.set(key, String(value)); }
  getAttribute(key) { return this.attributes.get(key); }
  removeAttribute(key) { this.attributes.delete(key); }
  addEventListener(name, callback) { this.events[name] = callback; }
  focus() { document.activeElement = this; }
  setPointerCapture() {}
  all() { return [this, ...this.children.flatMap(n => n.all())]; }
}
document.createElement = (tag) => new Element(tag);
const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const knobSource = app.slice(app.indexOf("function createKnob("), app.indexOf("function syncControl("));
const context = vm.createContext({
  document, preset: {}, state: { knobs: {} }, editing: 0,
  controls: new Map(), modulationViews: new Map(),
});
vm.runInContext(knobSource, context);
let value = 0.2, writes = 0;
const bank = new Element("div");
const update = context.createKnob(["drum1_12", "CUTOFF", v => `${Math.round(v * 100)}%`], bank, {
  register: false, get: () => value,
  set: v => { value = v; writes++; }, reset: () => 0.2,
});
update();
const box = bank.children[0], dial = box.querySelector(".dial");
dial.events.pointerdown({ pointerId: 1, clientX: 0, clientY: 100, preventDefault() {} });
dial.events.pointermove({ clientX: 0, clientY: 64 });
assert.equal(box.querySelector("output").textContent, "40%", "Drum knob did not repaint during the gesture");
assert.equal(Number(dial.getAttribute("aria-valuenow")), 40);
assert.equal(writes, 1);
dial.events.pointerup();
dial.events.keydown({ key: "ArrowUp", preventDefault() {} });
assert.equal(Number(dial.getAttribute("aria-valuenow")), value * 100);

const panels = vm.createContext({ window: {}, document });
vm.runInContext(await readFile(new URL("../rust/web/panels.js", import.meta.url), "utf8"), panels);
const drums = Object.create(panels.window.SadDrums.prototype);
const grid = new Element("div");
const view = { page: 0, track: 0, voice: 0, step: 0 };
const opened = [], stepEdits = [];
Object.assign(drums, {
  root: { querySelector: () => grid }, view: () => view,
  sync() {}, hooks: { trigger() {} }, openStep: (hit) => opened.push(hit),
  edit: (...edit) => stepEdits.push(edit),
});
drums.buildPattern();
const control = label => grid.all().find(e => e.getAttribute("aria-label") === label);
control("Open HH track").onclick();
assert.equal(view.voice, 6);
assert.equal(view.track, 6);
control("Snare step 8").onclick({ type: "click", shiftKey: true });
assert.equal(view.voice, 3);
assert.equal(view.track, 3);
assert.equal(view.step, 7);
// Step settings open from a right click (or Shift) and leave the step as it was.
let prevented = false;
control("Clap / Cym step 2").oncontextmenu({ type: "contextmenu", preventDefault: () => (prevented = true) });
assert.deepEqual([view.track, view.voice, view.step], [4, 4, 1]);
assert.ok(prevented, "The browser's own menu must stay closed");
assert.deepEqual(opened.map((hit) => hit.getAttribute("aria-label")), ["Snare step 8", "Clap / Cym step 2"]);
assert.equal(stepEdits.length, 0, "Opening step settings must not toggle the step");

// Every DXM-088 parameter appears once in the voice modules; only the hat
// voice shows Open decay.
const voicePreset = {
  drums: { params: Array.from({ length: 6 }, () => Array(56).fill(0)) },
};
for (const [voice, count] of [[0, 55], [5, 56]]) {
  const knobs = [];
  const voiceEditor = new panels.window.SadDrums(new Element("div"), { controls: [] }, {
    read: () => voicePreset, trigger() {}, parameter() {},
    knob: ([key]) => { knobs.push(key); return () => {}; },
  });
  Object.assign(voiceEditor, { view: () => ({ voice, track: voice }), knobs: [], fields: [] });
  voiceEditor.buildVoice();
  const menus = voiceEditor.voicePanel.querySelector(".drum-modules").all()
    .filter((e) => e.tagName === "SELECT").map((e) => e.getAttribute("aria-label"));
  assert.equal(new Set(knobs).size, knobs.length);
  assert.equal(new Set(menus).size, menus.length);
  assert.equal(knobs.length + menus.length, count, `Voice ${voice} must show each parameter once`);
  assert.equal(knobs.includes(`drum${voice + 1}_29`), voice === 5);
}

const drumPreset = { drums: { buffer_seconds: 8 } };
const routingEdits = [];
const bufferNodes = new Map();
const bufferNode = (id) => {
  if (!bufferNodes.has(id)) bufferNodes.set(id, new Element("select"));
  return bufferNodes.get(id);
};
const bufferContext = vm.createContext({
  document, preset: drumPreset, $: bufferNode,
  historySettings: () => bufferContext.preset.drums ?? bufferContext.preset.bus,
  option: (value, label) => Object.assign(new Element("option"), { value, textContent: label }),
  scheduleParameter: (...edit) => routingEdits.push(edit),
  schedulePreset: () => routingEdits.push(["preset"]),
  editSynth() {}, syncTimbreRouting() {},
});
vm.runInContext(app.slice(app.indexOf("const historyBuffers"), app.indexOf("function buildBus(")), bufferContext);
bufferContext.bindGrainBuffer();
bufferContext.syncGrainBuffer();
assert.equal(bufferNode("grain-buffer-field").hidden, false);
assert.equal(bufferNode("grain-buffer").children.length, 9);
assert.equal(bufferNode("grain-buffer").value, "8");
bufferNode("grain-buffer").value = "0.125";
bufferNode("grain-buffer").onchange();
assert.equal(drumPreset.drums.buffer_seconds, 0.125);
assert.deepEqual(routingEdits.splice(0), [["drum_buffer_seconds", 0.125]], "Buffer choice must send a scalar edit");
// A bus history stops at 8 s, and an unlisted length stays selectable.
bufferContext.preset = { bus: { granular: true, buffer_seconds: 3 } };
bufferContext.syncGrainBuffer();
assert.deepEqual(bufferNode("grain-buffer").children.map((o) => o.value), ["0.125", "0.25", "0.5", "1", "2", "4", "8", "3"]);
assert.equal(bufferNode("grain-buffer").value, "3");
bufferNode("grain-buffer").value = "2";
bufferNode("grain-buffer").onchange();
assert.equal(bufferContext.preset.bus.buffer_seconds, 2);
assert.deepEqual(routingEdits.splice(0), [["preset"]], "A bus buffer is a structural edit");
bufferContext.preset = { params: {} };
bufferContext.syncGrainBuffer();
assert.equal(bufferNode("grain-buffer-field").hidden, true);
const sizeContext = vm.createContext({ preset: drumPreset });
vm.runInContext(app.slice(app.indexOf("const historySettings"), app.indexOf("const grainSchema")), sizeContext);
assert.equal(sizeContext.grainSeconds(0.5), 0.01);
assert.equal(sizeContext.grainSeconds(1), 0.125);
drumPreset.drums.buffer_seconds = 32;
assert.equal(sizeContext.grainSeconds(1), 2.01);
sizeContext.preset = { params: {} };
assert.equal(sizeContext.grainSeconds(1), 2.01);
// A Granular bus scales grain size by its own history like the drums.
sizeContext.preset = { params: {}, bus: { granular: true, buffer_seconds: 0.125 } };
assert.equal(sizeContext.grainSeconds(1), 0.125);

const ids = new Map();
const state = {
  lfo_count: 4, env_count: 4,
  lfos: Array.from({ length: 8 }, () => ({ depth: 0.2 })),
  envs: Array.from({ length: 8 }, () => ({ depth: 0.3 })),
  matrix: [{ source: "lfo4" }, { source: "env4" }, { source: "lfo1" }],
};
let edits = 0;
const remove = vm.createContext({
  preset: state, presets: [{ lfos: state.lfos.map(() => ({ depth: 0 })), envs: state.envs.map(() => ({ depth: 0 })) }],
  $: id => { if (!ids.has(id)) ids.set(id, {}); return ids.get(id); },
  scopes: Array.from({ length: 8 }, () => ({ values: [1] })),
  structuredClone, syncLfos() {}, syncEnvs() {}, syncMatrix() {}, refreshColorDestinations() {},
  schedulePreset() { edits++; },
});
vm.runInContext(app.slice(app.indexOf("function removeModulator("), app.indexOf("function buildLfos(")), remove);
remove.removeModulator("lfo");
remove.removeModulator("env");
assert.equal(state.lfo_count, 3);
assert.equal(state.env_count, 3);
assert.equal(state.lfos[3].depth, 0);
assert.equal(state.envs[3].depth, 0);
assert.equal(state.matrix.length, 1);
assert.equal(state.matrix[0].source, "lfo1");
remove.removeModulator("lfo");
remove.removeModulator("env");
assert.equal(edits, 2, "The three original generators must not be removed");

const selectionNodes = new Map();
const selectionContext = vm.createContext({
  window: {}, performance: { now: () => 1000 },
  document: { activeElement: null, getElementById: id => {
    if (!selectionNodes.has(id)) selectionNodes.set(id, new Element("div"));
    return selectionNodes.get(id);
  } },
});
vm.runInContext(await readFile(new URL("../rust/web/selection.js", import.meta.url), "utf8"), selectionContext);
const selection = Object.create(selectionContext.window.SadSampleSelection.prototype);
selection.actions = { library: () => ({ ready: true }), params: () => ({}) };
const drumState = {
  preset: { drums: {} }, input: { active: false }, sample_id: null,
  sample_seconds: 8, sample_frames: 64000, sample_rate: 8000,
  region: { start: 0.25, end: 0.75 }, fades: { fade_in: 1, fade_out: 1 },
};
selection.sync(drumState);
assert.equal(selection.available(), false, "Native drums exposed file selection handles");
assert.equal(selectionNodes.get("sample-selection").hidden, true);
assert.equal(selectionNodes.get("region-start-handle").hidden, true);
assert.equal(selection.gainAt(0.25), 1, "The old sample's fade dimmed the drum waveform");
let selectionDraws = 0;
selection.draw({ fillRect() { selectionDraws++; } }, 400, 130);
assert.equal(selectionDraws, 0);
selection.sync({ ...drumState, preset: {} });
assert.equal(selection.available(), true);
assert.equal(selectionNodes.get("sample-selection").hidden, false);
assert.equal(selection.gainAt(0.25), 0, "Returning to the sample lost its fade display");
console.log("Controls: immediate drum-knob paint, linked track/voice selection and extra LFO/ENV removal passed.");
