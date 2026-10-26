// Exercise the shipping editor's handlers across backend snapshot replacement.
// No audio device or browser globals are needed for the transport regression.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

class Element {
  constructor(tag) {
    this.tagName = tag.toUpperCase();
    this.children = [];
    this.dataset = {};
    this.attributes = new Map();
    this.value = "";
    this.style = {};
    this.classes = new Set();
    this.listeners = {};
  }
  get className() {
    return [...this.classes].join(" ");
  }
  set className(value) {
    this.classes = new Set(String(value).split(" ").filter(Boolean));
  }
  get classList() {
    const classes = this.classes;
    return {
      add: (name) => classes.add(name),
      remove: (name) => classes.delete(name),
      contains: (name) => classes.has(name),
      toggle: (name, on = !classes.has(name)) =>
        on ? classes.add(name) : classes.delete(name),
    };
  }
  append(...nodes) {
    for (const node of nodes) if (node instanceof Element) node.parentNode = this;
    this.children.push(...nodes);
  }
  replaceChildren(...nodes) {
    this.children = [];
    this.append(...nodes);
  }
  remove() {
    this.parentNode?.children.splice(this.parentNode.children.indexOf(this), 1);
    this.parentNode = null;
  }
  replaceWith(node) {
    const parent = this.parentNode;
    parent.children[parent.children.indexOf(this)] = node;
    node.parentNode = parent;
    this.parentNode = null;
  }
  contains(node) {
    for (let n = node; n; n = n.parentNode) if (n === this) return true;
    return false;
  }
  closest(selector) {
    for (let n = this; n; n = n.parentNode)
      if (n.classes.has(selector.slice(1))) return n;
    return null;
  }
  get isConnected() {
    let n = this;
    while (n.parentNode) n = n.parentNode;
    return !!n.connected;
  }
  focus() {
    document.activeElement = this;
  }
  getBoundingClientRect() {
    return { left: 40, top: 300, right: 240, bottom: 330, width: 200, height: 30 };
  }
  addEventListener(type, callback) {
    this.listeners[type] = callback;
  }
  dispatchEvent(event) {
    this[`on${event.type}`]?.(event);
    this.listeners[event.type]?.(event);
  }
  setAttribute(key, value) {
    this.attributes.set(key, String(value));
  }
  getAttribute(key) {
    return this.attributes.get(key);
  }
  all() {
    return [
      this,
      ...this.children.flatMap((n) => (n instanceof Element ? n.all() : [])),
    ];
  }
}
const source = await readFile(
  process.argv[2] ?? new URL("../rust/web/sequence.js", import.meta.url),
  "utf8",
);
const globalListeners = {},
  listen = (type, callback) => (globalListeners[type] = callback),
  forget = (type) => delete globalListeners[type],
  timers = [];
const document = {
  body: Object.assign(new Element("body"), { connected: true }),
  documentElement: { clientWidth: 1280, clientHeight: 800 },
  activeElement: null,
  createElement: (tag) => new Element(tag),
  addEventListener: listen,
  removeEventListener: forget,
};
const context = vm.createContext({
  window: { addEventListener: listen, removeEventListener: forget },
  document,
  setTimeout: (callback) => timers.push(callback),
  clearTimeout: () => {},
  Event: class {
    constructor(type) {
      this.type = type;
    }
  },
});
vm.runInContext(source, context);
const step = {
  pitch: 60,
  pulses: 1,
  gate: 2,
  ratchet: 1,
  probability: 100,
  slide: false,
  skip: false,
  accumulate: 0,
  pitch_override: -1,
  gate_override: -1,
  probability_target: 0,
  accumulate_trigger: 0,
  cv: 0,
  cv_ramp: 0,
  cv_gate: false,
  velocity: 100,
};
const track = {
  id: 1,
  name: "Main",
  target: { kind: "main_pitch" },
  amount: 1,
  absolute: false,
  gate_output: false,
  root_note: 60,
  pitch_oscillators: [false, false, false, false],
  config: Array(43).fill(0),
  steps: Array.from({ length: 16 }, () => ({ ...step })),
};
track.config[1] = 16;
track.config[3] = 1;
track.config[7] = 55;
let preset = {
  sequencer: { enabled: true, tracks: [structuredClone(track)] },
  color_chain: [],
  pre_color_chain: [],
  resynth: { count: 4 },
};
const editor = new Element("div"),
  edits = [],
  knobs = [];
let structures = 0,
  refreshes = 0;
const ui = Object.create(context.window.SadNeuropolis.prototype);
Object.assign(ui, {
  root: {
    querySelector: (selector) => {
      assert.equal(selector, ".neuro-editor");
      return editor;
    },
  },
  selected: 1,
  knobs: [],
  catalog: { parameters: [], orders: ["Linear"], scales: ["Chromatic"] },
  hooks: {
    read: () => preset,
    edit: (e) => edits.push(e),
    structure: () => structures++,
    knob: (schema, container, binding) => {
      knobs.push({ schema, binding });
      return () => {};
    },
  },
  sync: () => refreshes++,
});
const control = (label) => {
  const found = editor
    .all()
    .filter((e) => e.getAttribute("aria-label") === label);
  assert.equal(found.length, 1, label);
  return found[0];
};
ui.buildEditor();
assert.equal(
  knobs.length,
  96,
  "Every step needs Gate, Pulses, Ratchet and three destination knobs",
);
for (let n = 1; n <= 16; n++)
  for (const field of ["pitch", "slide", "probability", "velocity"])
    control(`Stage ${n} ${field}`);
assert.ok(
  !editor.all().some((e) => e.getAttribute("aria-label") === "Stage layer"),
);

const first = control("Stage 1 pitch"),
  second = control("Stage 2 pitch");
first.value = "72";
first.oninput();
assert.equal(preset.sequencer.tracks[0].steps[0].pitch, 72);
const old = preset.sequencer.tracks[0];
preset = structuredClone(preset); // Ordinary /api/state adoption; DOM stays intact.
second.value = "67";
second.oninput();
first.value = "69";
first.oninput();
assert.equal(preset.sequencer.tracks[0].steps[0].pitch, 69);
assert.equal(preset.sequencer.tracks[0].steps[1].pitch, 67);
assert.equal(
  old.steps[0].pitch,
  72,
  "A handler still mutated its captured snapshot",
);
assert.equal(old.steps[1].pitch, 60);
// Stable IDs also survive reorder; a handler must not start editing a new row.
preset.sequencer.tracks.unshift({ ...structuredClone(track), id: 2 });
second.value = "65";
second.oninput();
assert.equal(ui.track().steps[1].pitch, 65);
assert.equal(preset.sequencer.tracks[0].steps[1].pitch, 60);

const gate = knobs[0].binding;
assert.equal(gate.get(), 0.55);
gate.set(0.27);
assert.equal(ui.track().steps[0].gate_override, 27);
assert.equal(ui.track().steps[0].gate, 2, "Gate length changed the pulse mode");
preset = structuredClone(preset);
gate.set(gate.reset());
ui.track().config[7] = 63;
assert.equal(
  gate.get(),
  0.63,
  "Reset gate should follow the current track length",
);
for (const [key, index] of [
  ["pulses", 1],
  ["ratchet", 2],
]) {
  const knob = knobs[index];
  assert.equal(knob.schema[5], `Stage 1 ${key}`);
  assert.equal(knob.binding.step, 1);
  assert.equal(knob.binding.ariaScale, 1);
  knob.binding.set(4.2);
  assert.equal(ui.track().steps[0][key], 4);
  preset = structuredClone(preset);
  knob.binding.set(knob.binding.get() + 1);
  assert.equal(ui.track().steps[0][key], 5);
  knob.binding.set(knob.binding.reset());
  assert.equal(ui.track().steps[0][key], 1);
}
for (let i = 0; i < 3; i++) {
  const k = knobs[3 + i];
  assert.equal(k.schema[5], `Stage 1 destination ${i + 1} amount`);
  k.binding.set(0.2 * (i + 1));
  preset = structuredClone(preset);
  k.binding.set(-0.1 * (i + 1));
  assert.equal(ui.track().steps[0][`destination${i + 1}`], -0.1 * (i + 1));
  assert.equal(edits.at(-1).field, 16 + i);
  assert.equal(edits.at(-1).stage, 0);
}
assert.equal(structures, 0, "Destination amount rebuilt the track");
for (let i = 0; i < 3; i++) {
  const destination = control(`Track destination ${i + 1}`);
  destination.value = ["filter", "resynth_osc4_control1", "pre_fx1_lcr_ratio"][i];
  destination.onchange();
  preset = structuredClone(preset);
}
assert.deepEqual(ui.track().destinations, ["filter", "resynth_osc4_control1", "pre_fx1_lcr_ratio"]);
assert.equal(structures, 3);
structures = 0;

control("Stage 1 slide").onclick();
control("Stage 1 slide").onclick();
assert.equal(
  ui.track().steps[0].slide,
  false,
  "Slide button read an old snapshot",
);
for (let i = 0; i < 4; i++) {
  const input = control(`Main pitch affects OSC ${i + 1}`);
  input.checked = i % 2 === 0;
  input.onchange();
}
assert.deepEqual(ui.track().pitch_oscillators, [true, false, true, false]);
const envGate = control("Track gate env");
assert.equal(envGate.type, "checkbox");
envGate.value = "on";
envGate.checked = true;
envGate.oninput();
preset = structuredClone(preset);
assert.equal(ui.track().env_gate, true);
assert.equal(ui.configValue(ui.track(), 53), true);
assert.deepEqual({ ...edits.at(-1) }, { track: 1, stage: null, field: 53, value: 1 });
assert.equal(structures, 0, "A scalar gesture rebuilt the sequencer");
assert.ok(edits.some((e) => e.field === 49 && e.stage === null));
assert.ok(edits.some((e) => e.field === 9 && e.stage === 0 && e.value === 27));

const name = control("Track name");
name.value = "Current main";
name.onchange();
assert.equal(ui.track().name, "Current main");
const destination = control("Track destination");
destination.value = "osc:2";
destination.onchange();
ui.buildEditor();
assert.ok(
  !editor.all().some((e) => e.getAttribute("aria-label") === "Track gate env"),
  "Gate ENV is a Main pitch option",
);
const offset = control("Stage 1 pitch offset");
assert.equal(offset.min, -24);
assert.equal(offset.max, 24);
offset.value = "-24";
offset.oninput();
assert.equal(ui.track().steps[0].pitch, 36);
assert.equal(ui.pitchText(ui.track(), 36), "-24 st");
offset.value = "24";
offset.oninput();
assert.equal(ui.track().steps[0].pitch, 84);
assert.equal(ui.pitchText(ui.track(), 84), "+24 st");
const override = control("Stage 1 pitch override");
override.value = "-1";
override.oninput();
assert.equal(
  ui.track().steps[0].pitch_override,
  59,
  "-1 st is a valid relative override",
);
override.value = "";
override.oninput();
assert.equal(
  ui.track().steps[0].pitch_override,
  -1,
  "Empty override should inherit the step pitch",
);

const before = edits.length;
preset.sequencer.tracks = preset.sequencer.tracks.filter((t) => t.id !== 1);
ui.edit(old, 0, 0, 77);
assert.equal(edits.length, before, "A removed track received a stale edit");
assert.ok(refreshes > 0);
console.log(
  "Sequencer editor: snapshot/reorder-safe pitch, 96 step knobs, integer Pulses/Ratchet, scalar OSC routing and ±24 st offsets passed.",
);

// Destination pickers: a closed picker is one button. Its section → target
// menu exists only while open, and works by keyboard and pointer.
const catalog = ui.catalog;
editor.connected = true;
preset.sequencer.tracks = [{ ...structuredClone(track), target: { kind: "oscillator", index: 2 } }];
ui.catalog = {
  ...catalog,
  parameters: [
    ["mix", "Dry / wet"],
    ["position", "Position"],
    ["grain_feedback", "Grain feedback"],
    ["grain_group2_cutoff", "Grain group B cutoff"],
    ["filter", "Filter 1 cutoff"],
    ["filter_resonance", "Filter 1 resonance"],
    ["filter10_cutoff", "Filter 10 cutoff"],
    ["lfo1_rate", "LFO 1 rate"],
    ["resynth_mix", "Resynth mix"],
    ["resynth_osc2_level", "Resynth OSC 2 level"],
  ].map(([id, name]) => ({ id, name })),
};
ui.buildEditor();
const picker = control("Track destination"),
  menuLayer = () => document.body.children[0],
  menuRows = (depth) =>
    menuLayer().children[depth].children.map((row) => row.children[0].textContent),
  menuFocus = () => document.activeElement.children[0]?.textContent,
  menuKey = (key) =>
    menuLayer().onkeydown({
      key,
      target: document.activeElement,
      preventDefault() {},
      stopPropagation() {},
    });
assert.equal(picker.tagName, "BUTTON");
assert.equal(picker.getAttribute("aria-haspopup"), "menu");
assert.equal(picker.value, "osc:2");
assert.equal(picker.textContent, "OSC 3 pitch offset");
assert.equal(picker.children.length, 0, "A closed picker builds no menu");
picker.onclick();
assert.equal(picker.getAttribute("aria-expanded"), "true");
assert.deepEqual(menuRows(0), ["Pitch & notes", "Main", "Grains", "Filters", "LFO", "Resynth"]);
assert.equal(menuFocus(), "Pitch & notes", "The menu opens on the value's section");
menuKey("ArrowRight");
assert.equal(menuFocus(), "OSC 3 pitch offset");
assert.equal(document.activeElement.getAttribute("aria-checked"), "true");
menuKey("ArrowUp");
menuKey("Enter");
assert.equal(menuLayer(), undefined, "Picking closes the menu");
assert.equal(document.activeElement, picker);
assert.deepEqual({ ...ui.track().target }, { kind: "oscillator", index: 1 });
picker.onclick();
menuKey("f");
assert.equal(menuFocus(), "Filters", "Type-ahead");
menuKey("ArrowRight");
assert.deepEqual(menuRows(1), ["Filter 1", "Filter 10"]);
menuKey("ArrowDown");
menuKey("ArrowRight");
assert.equal(menuLayer().children[2].getAttribute("aria-label"), "Filter 10");
assert.deepEqual(menuRows(2), ["Cutoff"], "Filter 10 is not Filter 1");
menuKey("ArrowLeft");
assert.equal(menuLayer().children.length, 2);
assert.equal(menuFocus(), "Filter 10");
menuKey("ArrowLeft");
menuKey("r");
assert.equal(menuFocus(), "Resynth");
menuKey("ArrowRight");
assert.deepEqual(menuRows(1), ["Mix", "OSC 2"]);
menuKey("Escape");
assert.equal(menuLayer(), undefined);
assert.equal(picker.getAttribute("aria-expanded"), "false");

// A Synth engine timbre offers its own oscillators beside Resynth's.
preset.synth = { count: 1, oscillators: [] };
ui.buildEditor();
const synthPicker = control("Track destination");
assert.equal(synthPicker.textContent, "Resynth OSC 2 pitch offset");
synthPicker.value = "synth-osc:3";
synthPicker.onchange();
assert.deepEqual({ ...ui.track().target }, { kind: "synth_oscillator", index: 3 });
ui.buildEditor();
assert.equal(control("Stage 1 pitch offset").min, -24);
assert.equal(ui.targetName(ui.track()), "Synth OSC 4 offset");
assert.equal(ui.pitchText(ui.track(), 67), "+7 st");
ui.track().target = { kind: "oscillator", index: 1 };
delete preset.synth;
ui.buildEditor();
assert.equal(document.activeElement, picker);
assert.equal(picker.value, "osc:1", "Escape keeps the value");
picker.onclick();
for (const clientY of [5, 5, 6]) {
  menuLayer().onpointermove({ target: menuLayer().children[0].children[3].children[0], pointerType: "mouse", clientX: 5, clientY });
  assert.equal(menuFocus(), clientY < 6 ? "Pitch & notes" : "Filters", "A resting pointer does not steal the keyboard's row");
}
timers.splice(0).forEach((callback) => callback());
assert.deepEqual(menuRows(1), ["Filter 1", "Filter 10"], "Hovering a section opens it");
menuLayer().onclick({ target: menuLayer().children[1].children[0], detail: 1 });
assert.deepEqual(menuRows(2), ["Cutoff", "Resonance"]);
menuLayer().onclick({ target: menuLayer().children[2].children[1], detail: 1 });
assert.deepEqual({ ...ui.track().target }, { kind: "parameter", parameter: "filter_resonance" });
assert.equal(picker.textContent, "Filter 1 resonance");
document.documentElement.clientWidth = 375;
picker.onclick();
assert.equal(menuLayer().className, "sad-menu-layer is-drill");
assert.equal(menuFocus(), "Filters");
menuKey("ArrowUp");
menuKey("Enter");
assert.equal(menuLayer().children.length, 1, "Phones drill into a section in place");
// Grains holds the grain engine's controls, then each group's.
assert.deepEqual(menuRows(0), ["Grains", "Position", "Grain feedback", "Group B"]);
assert.equal(menuFocus(), "Position");
menuKey("ArrowLeft");
assert.deepEqual(menuRows(0).slice(0, 3), ["Pitch & notes", "Main", "Grains"]);
assert.equal(menuFocus(), "Grains");
globalListeners.pointerdown({ target: editor });
assert.equal(menuLayer(), undefined, "A click elsewhere closes the menu");
assert.equal(globalListeners.scroll, undefined);
document.documentElement.clientWidth = 1280;
ui.catalog = catalog;
document.activeElement = null;
console.log(
  "Destination menus: one closed button, section → target cascade, keyboard, type-ahead, hover, phone drill-in and outside click passed.",
);

// Audition is optional and never commits the source value. Space previews;
// Enter/name chooses. Closing/back/category changes stop even pending audio.
{
  const anchor = Object.assign(new Element("button"), { connected: true });
  const previews = [], updates = [];
  let stops = 0;
  context.window.SadLazySelect.configure(anchor, "old", [
    { label: "Vocals", choices: [["old", "Old voice"], ["new", "New voice"]] },
  ], { play: (id, notify) => { previews.push(id); updates.push(notify); }, stop: () => stops++ });
  anchor.onclick();
  menuKey("ArrowRight");
  menuKey("ArrowDown");
  menuKey(" ");
  assert.deepEqual(previews, ["new"]);
  assert.equal(anchor.value, "old");
  updates[0]("playing");
  const playButton = menuLayer().children[1].children[1].previewButton;
  assert.equal(playButton.getAttribute("aria-pressed"), "true");
  assert.equal(playButton.textContent, "■");
  menuLayer().onclick({ target: playButton, detail: 1 });
  assert.deepEqual(previews, ["new", "new"]);
  assert.equal(anchor.value, "old", "Preview button selected the sound");
  menuKey("Enter");
  assert.equal(anchor.value, "new");
  assert.equal(menuLayer(), undefined);
  assert.equal(stops, 1);
  updates[0]("playing"); // Late completions cannot recreate the dismissed menu.
  anchor.onclick();
  menuKey("ArrowRight");
  menuKey("ArrowLeft");
  assert.equal(stops, 2);
  menuKey("Escape");
  assert.equal(stops, 3);
}
console.log("Source preview menus: separate preview/selection, Space/Enter, stop on close and back, stale callbacks passed.");

// Step selection, copy and paste.
preset = {
  ...preset,
  sequencer: {
    enabled: true,
    tracks: [
      { ...structuredClone(track), id: 1, root_note: 48 },
      { ...structuredClone(track), id: 2, target: { kind: "oscillator", index: 0 } },
    ],
  },
};
const lane = (id) => preset.sequencer.tracks.find((t) => t.id === id);
lane(1).steps.forEach((s, i) => {
  s.pitch = 48 + i;
  s.velocity = 10 + i;
});
lane(1).steps[2].slide = true;
Object.assign(ui, {
  root: { ...ui.root, getClientRects: () => [{}], contains: () => true },
  selected: 1,
  selection: new Set(),
  anchor: null,
  clipboard: null,
  syncSelection: () => {},
});
const picked = () => [...ui.selection].sort((a, b) => a - b);
const press = (stage, keys = {}) =>
  ui.pickStart(stage, { button: 0, preventDefault() {}, currentTarget: {}, ...keys });
press(0);
ui.pickTo(3);
ui.pickEnd();
assert.deepEqual(picked(), [0, 1, 2, 3], "Dragging picks a run");
press(5, { metaKey: true });
ui.pickEnd();
press(1, { ctrlKey: true });
ui.pickEnd();
assert.deepEqual(picked(), [0, 2, 3, 5], "Cmd/Ctrl adds and removes steps");
press(7, { shiftKey: true });
ui.pickEnd();
assert.deepEqual(picked(), [1, 2, 3, 4, 5, 6, 7], "Shift extends from the last step picked");
press(4);
ui.pickEnd();
assert.deepEqual(picked(), [4]);
press(4);
ui.pickEnd();
assert.deepEqual(picked(), [], "Clicking the only selected step clears it");
ui.pickKey(9, {});
ui.pickKey(12, { shiftKey: true });
assert.deepEqual(picked(), [9, 10, 11, 12], "Enter / Space picks with the keyboard");

ui.select(new Set([0, 2]));
ui.copy();
assert.equal(ui.clipboard.steps.length, 3);
assert.equal(ui.clipboard.steps[1], null, "The gap between picked steps is kept");
lane(1).steps[0].pitch = 99;
assert.equal(ui.clipboard.steps[0].pitch, 48, "The copy is a snapshot");
edits.length = 0;
ui.select(new Set([8]));
ui.paste();
assert.deepEqual(lane(1).steps.slice(8, 11).map((s) => s.pitch), [48, 57, 50]);
assert.equal(lane(1).steps[10].slide, true);
assert.equal(lane(1).steps[10].velocity, 12);
assert.deepEqual(picked(), [8, 9, 10], "A lone step is where the whole copy starts");
assert.ok(edits.every((e) => e.track === 1 && e.stage !== 9), "The gap was pasted over");
assert.ok(!edits.some((e) => e.field === 1), "Unchanged fields were resent");

lane(1).steps[1].pitch_override = 50;
ui.select(new Set([0, 1]));
ui.copy();
ui.select(new Set([11, 12, 13, 14, 15]));
ui.paste();
assert.deepEqual(
  lane(1).steps.slice(11).map((s) => s.pitch),
  [99, 49, 99, 49, 99],
  "A longer selection repeats the copy",
);
assert.deepEqual(lane(1).steps.slice(11).map((s) => s.pitch_override), [-1, 50, -1, 50, -1]);
ui.select(new Set([15]));
ui.paste();
assert.deepEqual(picked(), [15], "A paste stops at step 16");
ui.select(new Set([3, 4, 9]));
ui.paste();
assert.deepEqual(
  [3, 4, 9, 10].map((i) => lane(1).steps[i].pitch),
  [99, 49, 99, 49],
  "Every selected region gets the copy",
);

// Main pitch (root C3) → oscillator offsets keep the intervals, within ±24 st.
lane(1).steps[0].pitch = 52;
lane(1).steps[1].pitch = 40;
lane(1).steps[1].pitch_override = 50;
lane(1).steps[2].pitch = 10;
ui.select(new Set([0, 1, 2]));
ui.copy();
ui.selected = 2;
ui.select(new Set([0]));
ui.paste();
assert.deepEqual(lane(2).steps.slice(0, 3).map((s) => s.pitch), [64, 52, 36]);
assert.equal(lane(2).steps[1].pitch_override, 62);
assert.equal(lane(1).steps[0].pitch, 52, "Pasting into one track changed another");
ui.select(new Set([0]));
ui.copy();
ui.selected = 1;
ui.select(new Set([5]));
ui.paste();
assert.equal(lane(1).steps[5].pitch, 52, "An oscillator offset pastes back onto the root");

// ⌘C / Ctrl+C / ⌘V, also on a Russian layout, but never inside a number field.
const keyEvent = (key, code, extra = {}) => {
  let prevented = false;
  ui.shortcut({
    key,
    code,
    target: { matches: () => false },
    preventDefault: () => (prevented = true),
    ...extra,
  });
  return prevented;
};
lane(1).steps[6].pitch = 70;
ui.select(new Set([6]));
assert.ok(keyEvent("с", "KeyC", { metaKey: true }));
assert.equal(ui.clipboard.steps[0].pitch, 70);
ui.select(new Set([7]));
assert.ok(keyEvent("v", "KeyV", { ctrlKey: true }));
assert.equal(lane(1).steps[7].pitch, 70);
assert.ok(!keyEvent("c", "KeyC", { metaKey: true, target: { matches: () => true } }));
assert.ok(!keyEvent("c", "KeyC"), "C without a modifier is a keyboard note");
keyEvent("Escape", "Escape");
assert.deepEqual(picked(), []);
assert.ok(!keyEvent("v", "KeyV", { metaKey: true }), "Paste with nothing selected");
console.log(
  "Sequencer steps: drag / Shift / Cmd selection, gap-keeping copy, start-point and repeating paste into every region, root-aware oscillator paste and layout-independent shortcuts passed.",
);

// Stages: pages of sixteen steps. New pages copy the last one, tabs show
// one page at a time, and step edits, picks and pastes land on that page.
{
  const fresh = { ...step, destination1: 0, destination2: 0, destination3: 0 };
  ui.catalog = { ...ui.catalog, track: { ...structuredClone(track), steps: Array.from({ length: 16 }, () => ({ ...fresh })) } };
  preset = {
    ...preset,
    sequencer: { enabled: true, tracks: [{ ...structuredClone(track), steps: lane(1).steps.map((s) => ({ ...fresh, ...s })) }] },
  };
  const paged = () => preset.sequencer.tracks[0];
  Object.assign(ui, { selected: 1, visiblePage: 0, selection: new Set(), clipboard: null });
  edits.length = 0;
  ui.buildEditor();
  assert.equal(control("Track stages").children.length, 8);
  control("Track stages").value = "3";
  control("Track stages").oninput();
  assert.deepEqual({ ...edits.at(-1) }, { track: 1, stage: null, field: 54, value: 3 });
  assert.equal(paged().pages, 3);
  assert.equal(paged().steps.length, 48);
  const json = (steps) => JSON.stringify(steps);
  assert.equal(json(paged().steps.slice(32, 48)), json(paged().steps.slice(0, 16)), "a new page copies the last");
  assert.notEqual(paged().steps[32], paged().steps[0], "pages share step objects");

  ui.buildEditor();
  const tabs = editor.all().filter((e) => e.classes.has("neuro-stage-page"));
  assert.deepEqual(tabs.map((t) => t.textContent), ["1–16", "17–32", "33–48"]);
  assert.equal(control("Stage 2, steps 17–32").getAttribute("aria-selected"), "false");
  control("Stage 2, steps 17–32").onclick();
  assert.equal(ui.visiblePage, 1);
  ui.buildEditor();
  assert.equal(control("Stage 2, steps 17–32").getAttribute("aria-selected"), "true");
  const first = paged().steps[0].pitch;
  control("Stage 17 pitch").value = "71";
  control("Stage 17 pitch").oninput();
  assert.deepEqual({ ...edits.at(-1) }, { track: 1, stage: 16, field: 0, value: 71 });
  assert.equal(paged().steps[16].pitch, 71);
  assert.equal(paged().steps[0].pitch, first, "the first page changed");
  assert.throws(() => control("Stage 1 pitch"), /Stage 1 pitch/);

  // Picks are columns of the page shown: a phrase copied on the second
  // page pastes into the same columns of the third.
  ui.syncSelection = () => {};
  ui.select(new Set([0, 1]));
  ui.copy();
  assert.deepEqual([...ui.clipboard.steps.map((s) => s.pitch)], [71, paged().steps[17].pitch]);
  ui.visiblePage = 2;
  ui.paste();
  assert.equal(paged().steps[32].pitch, 71);
  assert.ok(edits.some((e) => e.stage === 32 && e.field === 0 && e.value === 71));

  // Fewer pages show the last one left; the playing page is marked.
  ui.edit(1, null, 54, 2);
  assert.equal(ui.offset(paged()), 16);
  ui.buildEditor();
  const root = ui.root;
  ui.root = {
    querySelector: () => null,
    querySelectorAll: (selector) => editor.all().filter((e) => e.classes.has(selector.slice(1))),
  };
  ui.update({ count: 1, tracks: [{ id: 1, step: 20 }] }, true);
  const cards = editor.all().filter((e) => e.classes.has("neuro-stage"));
  assert.ok(cards[4].classes.has("is-current"), "step 21 is the fifth of the second page");
  assert.deepEqual(
    editor.all().filter((e) => e.classes.has("neuro-stage-page")).map((t) => t.classes.has("is-playing")),
    [false, true],
  );
  ui.update({ count: 1, tracks: [{ id: 1, step: 3 }] }, true);
  assert.ok(!cards.some((c) => c.classes.has("is-current")), "a step of another page lit up");
  ui.root = root;
}
console.log("Sequencer stages: page count with copied pages, tabs, edits, picks and pastes on the page shown, playing page passed.");
