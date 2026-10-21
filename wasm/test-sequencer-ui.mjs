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
  }
  append(...nodes) {
    this.children.push(...nodes);
  }
  replaceChildren(...nodes) {
    this.children = nodes;
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
const context = vm.createContext({
  window: {},
  document: { createElement: (tag) => new Element(tag) },
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

const lazy = control("Track destination");
const selectedValue = lazy.value;
assert.equal(lazy.all().filter(n => n.tagName === "OPTION").length, 1);
lazy.onfocus();
assert.ok(lazy.all().filter(n => n.tagName === "OPTION").length > 1);
assert.equal(lazy.value, selectedValue);
lazy.onblur();
assert.equal(lazy.all().filter(n => n.tagName === "OPTION").length, 1);
assert.equal(lazy.value, selectedValue);
