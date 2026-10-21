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
console.log("Controls: immediate drum-knob paint, linked track/voice selection and extra LFO/ENV removal passed.");
