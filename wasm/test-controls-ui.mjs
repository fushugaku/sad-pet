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
  append(...nodes) { this.children.push(...nodes); }
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
Object.assign(drums, {
  root: { querySelector: () => grid }, view: () => view,
  sync() {}, hooks: { trigger() {} },
});
drums.buildPattern();
const control = label => grid.all().find(e => e.getAttribute("aria-label") === label);
control("Open HH track").onclick();
assert.equal(view.voice, 6);
assert.equal(view.track, 6);
control("Snare step 8").onclick({ shiftKey: true });
assert.equal(view.voice, 3);
assert.equal(view.track, 3);
assert.equal(view.step, 7);

const drumPreset = { drums: { buffer_seconds: 8 } };
const routingEdits = [];
const drumRoot = new Element("div");
const drumEditor = new panels.window.SadDrums(drumRoot, { controls: [] }, {
  read: () => drumPreset, parameter: (...edit) => routingEdits.push(edit), routing() {},
});
drumEditor.buffer.value = "0.125";
drumEditor.buffer.onchange();
assert.equal(drumPreset.drums.buffer_seconds, 0.125);
assert.deepEqual(routingEdits, [["drum_buffer_seconds", 0.125]], "Buffer choice must send a scalar edit");
const sizeContext = vm.createContext({ preset: drumPreset });
vm.runInContext(app.slice(app.indexOf("function grainSeconds("), app.indexOf("const grainSchema")), sizeContext);
assert.equal(sizeContext.grainSeconds(0.5), 0.01);
assert.equal(sizeContext.grainSeconds(1), 0.125);
drumPreset.drums.buffer_seconds = 32;
assert.equal(sizeContext.grainSeconds(1), 2.01);
sizeContext.preset = { params: {} };
assert.equal(sizeContext.grainSeconds(1), 2.01);

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
