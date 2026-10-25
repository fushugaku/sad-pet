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
  controls: new Map(), modulationViews: new Map(), knobUnits: new Map(),
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

// Filter graphs follow each type: lows pass low-pass types, highs pass
// high-pass types, notches dip at the cutoff and resonance lifts it.
const gain = (kind, hz, resonance = 0, morph = 0) =>
  20 * Math.log10(panels.window.SadDrums.filterGain(kind, hz, 1000, resonance, morph));
for (const kind of [1, 7, 8, 9, 11, 12, 13, 16, 17, 18])
  assert.ok(gain(kind, 50) > -3 && gain(kind, 15000) < -30, `Filter type ${kind} must be a low-pass`);
for (const kind of [2, 10, 14, 19, 20])
  assert.ok(gain(kind, 15000) > -3 && gain(kind, 50) < -30, `Filter type ${kind} must be a high-pass`);
for (const kind of [5, 23, 24])
  assert.ok(gain(kind, 1000) < -30 && gain(kind, 50) > -3, `Filter type ${kind} must be a notch`);
assert.ok(gain(12, 1000, 0, 0.5) < -30, "SEM halfway must be a notch");
assert.ok(gain(1, 1000, 0.9) > 10, "Resonance must lift the cutoff");
assert.equal(gain(0, 1000), 0);

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
// A native host reports knob feedback 30 times a second. Right after an edit
// its "current" still shows the old value: that is stale, not modulation.
let mix = 0.2;
context.state.knobs.resynth_mix = { base: 0.2, current: 0.2, min: 0.2, max: 0.6, modulated: true };
const lagBank = new Element("div");
const lagUpdate = context.createKnob(["resynth_mix", "MIX", (v) => `${Math.round(v * 100)}%`], lagBank, {
  register: false, get: () => mix, set: (v) => { mix = v; }, reset: () => 0.2,
});
lagUpdate();
const lagBox = lagBank.children[0];
assert.equal(lagBox.querySelector("output").textContent, "20%");
document.activeElement = null;
lagBox.querySelector(".dial").events.keydown({ key: "End", preventDefault() {} });
assert.equal(mix, 1);
assert.equal(lagBox.querySelector("output").textContent, "100%", "Stale feedback held the knob back");
assert.equal(lagBox.querySelector(".knob-mod-range").textContent, "");

// Drum destination menus group each voice's controls like its modules, every
// control once; the Envelopes page holds the amp, pitch, filter and assign rows.
for (const [voice, count] of [[0, 55], [5, 56]]) {
  const sections = panels.window.SadDrums.controlSections(voice);
  const ids = sections.flatMap(([, controls]) => controls.map(([id]) => id));
  assert.equal(new Set(ids).size, count, `Voice ${voice} menu must list each control once`);
  const envelopes = new Map(sections).get("Envelopes").map(([, name]) => name);
  for (const name of ["Amp attack", "Pitch sweep", "Filter attack", "Assign decay"])
    assert.ok(envelopes.includes(name), `${name} missing from Envelopes`);
}
assert.equal(new Map(panels.window.SadDrums.controlSections(0)).get("Filter 1")
  .find(([id]) => id === 12)[1], "Filter 1 cutoff");

// Modulation depths read in the target's units: semitones for pitch, percent
// for a 0…1 control and the reached value on a curved display.
const unitsSource = app.slice(app.indexOf("const signedPercent ="), app.indexOf("function buildEnvs("));
const units = vm.createContext({
  window: {}, preset: { params: { pitch: 0.5, filter: 0.5 } },
  sequenceCatalog: {
    modulation: [{ index: 7, id: "resynth_osc1_transpose" }],
    parameters: [{ id: "resynth_osc1_transpose", min: -48, max: 48 }],
  },
});
vm.runInContext(`${unitsSource};this.depthText=depthText;this.knobUnits=knobUnits;this.unitDepth=unitDepth;this.parameterUnit=parameterUnit;`, units);
units.knobUnits.set("pitch", { min: 0, max: 1, get: () => 0.5, format: (v) => `${((v * 2 - 1) * 24).toFixed(1)} st` });
units.knobUnits.set("grain_shape", { min: 0, max: 1, get: () => 0.5, format: (v) => `${Math.round(v * 100)}%` });
units.knobUnits.set("filter", { min: 0, max: 1, get: () => 0.5, format: (v) => `${Math.round(20 * 1000 ** v).toLocaleString("en")} Hz` });
units.knobUnits.set("resynth_osc1_transpose", { min: -48, max: 48, get: () => 0, format: (v) => `${v > 0 ? "+" : ""}${Math.round(v)} st` });
assert.equal(units.depthText("pitch", 0.25), "+12.0 st");
assert.equal(units.depthText("pitch", -0.5), "−24.0 st");
assert.equal(units.depthText("grain_shape", 0.3), "+30%");
assert.equal(units.depthText("filter", 0.5), "→ 20,000 Hz");
assert.equal(units.depthText({ control: { index: 7 } }, 0.125), "+12 st");
assert.equal(units.depthText("cloud", 0.4), "+40%", "A target without a knob keeps percent");
assert.equal(units.unitDepth(units.parameterUnit("resynth_osc1_transpose"), -0.25), "−24 st");
// A pattern note shift paints blue where each note sounds now; the green
// steps stay where they were set, and a step added later shows in both.
{
  const tracks = Array.from({ length: 7 }, () => ({
    length: 16, steps: Array.from({ length: 128 }, () => ({ velocity: 0 })),
  }));
  tracks[3].steps[0].velocity = 1;
  tracks[3].steps[4].velocity = 0.5;
  tracks[3].steps[20].velocity = 1;
  const hits = Array.from({ length: 7 * 32 }, (_, i) => {
    const classes = new Set(), style = new Map();
    return {
      dataset: { track: String(Math.floor(i / 32)), step: String(i % 32) }, classes, style,
      classList: { toggle: (name, on) => (on ? classes.add(name) : classes.delete(name)) },
    };
  });
  for (const hit of hits) hit.style.setProperty = (key, value) => hit.style.set(key, value);
  const editor = Object.assign(Object.create(panels.window.SadDrums.prototype), {
    hits, settings: () => ({ tracks }),
  });
  const view = (shift) => ({ playing: false, positions: Array(7).fill(-1), shifts: [0, 0, 0, shift, 0, 0, 0] });
  const moved = () => hits.filter((h) => h.classes.has("moved"))
    .map((h) => [Number(h.dataset.step), h.style.get("--moved-velocity")]);
  editor.update(view(-1));
  assert.deepEqual(moved(), [[3, 0.5], [15, 1]], "A left shift wraps inside the track length");
  tracks[3].steps[8].velocity = 0.7;
  editor.paintShift();
  assert.deepEqual(moved(), [[3, 0.5], [7, 0.7], [15, 1]], "A new step must also play shifted");
  assert.equal(tracks[3].steps[0].velocity, 1, "The shift moved the step itself");
  editor.update(view(16));
  assert.deepEqual(moved(), [], "A whole-length shift plays every note in place");
  editor.update(view(0));
  assert.deepEqual(moved(), []);
}

// Every drum track, Open HH included, routes to its Pattern note shift from
// the modulation and sequencer destination menus.
{
  const menus = vm.createContext({ window: {}, document });
  vm.runInContext(await readFile(new URL("../rust/web/sequence.js", import.meta.url), "utf8"), menus);
  vm.runInContext(await readFile(new URL("../rust/web/panels.js", import.meta.url), "utf8"), menus);
  const tracks = [...menus.window.SadDrums.trackNames];
  const shifts = tracks.map((_, t) => `drum${t + 1}_note_shift`);
  // Plain copies: the menus are built in another realm.
  const pattern = (groups) => JSON.parse(JSON.stringify(groups.map(({ label, choices }) =>
    [label, choices.find((c) => c.label === "Pattern")?.choices])));
  const lanes = menus.window.SadNeuropolis.prototype.drumGroups(
    new Map([["drum1_12", "Drum 1 · Filter 1 cutoff"], ...shifts.map((id) => [id, ""])]), "param:");
  assert.deepEqual(pattern(lanes), tracks.map((track, t) =>
    [track, [[`param:${shifts[t]}`, "Note shift", `${track} · Pattern note shift`]]]));
  assert.equal(lanes[6].choices.length, 1, "Open HH has no voice controls of its own");
  vm.runInContext(app.slice(app.indexOf("function drumDestinations("), app.indexOf("function refreshColorDestinations(")), menus);
  menus.destinationValue = (d) => `control:${d.control.index}`;
  menus.sequenceCatalog = { modulation: shifts.map((id, index) => ({ id, index: 900 + index })) };
  assert.deepEqual(pattern(menus.drumDestinations([["Drum 1", []]])), tracks.map((track, t) =>
    [track, [[`control:${900 + t}`, "Note shift", `${track} · Pattern note shift`]]]));
}
console.log("Controls: immediate drum-knob paint, linked track/voice selection, drum note shift and extra LFO/ENV removal passed.");
