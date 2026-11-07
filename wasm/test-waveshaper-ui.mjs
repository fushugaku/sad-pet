// Run the shipping shared waveshaper editor with a small DOM and canvas.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const source = app.slice(app.indexOf("const WAVESHAPER_OPTION_DEFAULTS ="), app.indexOf("function scheduleDrum("));
assert.ok(source.includes("function buildWaveshaperOptions"));

const bindings = new Map();
const knobNodes = new Map(), paths = [];
let paints = 0, path = [];
const context2d = {
  setTransform() {}, clearRect() {},
  beginPath() { path = []; },
  moveTo(x, y) { path.push([x, y]); },
  lineTo(x, y) { path.push([x, y]); },
  arc() {}, fill() {},
  stroke() { paints++; paths.push({ color: this.strokeStyle, points: [...path] }); },
};
class Element {
  constructor(tag = "div") {
    this.tagName = tag.toUpperCase();
    this.children = [];
    this.parts = new Map();
    this.attributes = new Map();
    this.dataset = {};
    this.style = { setProperty() {} };
  }
  set innerHTML(value) { this.html = value; }
  querySelector(selector) {
    if (!this.parts.has(selector)) this.parts.set(selector, new Element(selector === ".waveshaper-curve" ? "canvas" : "div"));
    return this.parts.get(selector);
  }
  replaceChildren(...nodes) { this.children = nodes; }
  append(...nodes) { this.children.push(...nodes); }
  setAttribute(name, value) { this.attributes.set(name, String(value)); }
  removeAttribute(name) { this.attributes.delete(name); }
  getAttribute(name) { return this.attributes.get(name); }
  getBoundingClientRect() { return { left: 0, top: 0, width: 320, height: 120 }; }
  getContext() { return context2d; }
  focus() { document.activeElement = this; }
  setPointerCapture() {}
}
const document = { activeElement: null, createElement: (tag) => new Element(tag) };
const context = vm.createContext({
  document, structuredClone, preset: { env_count: 3, tb303: null },
  percent: (value) => `${Math.round(value * 100)}%`,
  signedPercent: (value) => `${Math.round(value * 100)}%`,
  WAVESHAPER_TYPES: [["drive", "Drive"]],
  option: (value, label) => ({ value, label }),
  requestAnimationFrame: (fn) => { fn(); return 1; },
  createKnob: (schema, bank, binding) => {
    bindings.set(schema[0], binding);
    const node = new Element();
    knobNodes.set(schema[0], node);
    bank.append(node);
    return () => {};
  },
});
const envelopeHelper = app.slice(app.indexOf("function syncFilterEnvelope("), app.indexOf("function syncFilterEnvelopes("));
vm.runInContext(`${envelopeHelper}\n${source}`, context);
assert.equal(vm.runInContext("WAVESHAPER_OPTION_DEFAULTS.curve_b", context), "soft_clip");
const format = (field, value) => vm.runInContext(`WAVESHAPER_OPTION_KNOBS.find(([name]) => name === '${field}')[2](${value})`, context);
assert.equal(format("bass_frequency", 0), "40 Hz");
assert.equal(format("bass_frequency", 1), "640 Hz");
assert.equal(format("attack", 1), "200.0 ms");
assert.equal(format("release", 1), "1000 ms");

let saved = { future: { retained: true } };
const writes = [];
let editor, pointsFinished = 0, liveSource = true;
editor = context.buildWaveshaperOptions({
  label: "FX 1 waveshaper", key: "color:9", register: true,
  get: () => saved,
  set: (patch, streaming = false) => { saved = { ...saved, ...patch }; writes.push(["preset", patch, streaming]); editor.sync(); },
  setNumeric: (field, value) => { saved = { ...saved, [field]: value }; writes.push(["host", field, value]); editor.sync(); },
  finishPoints: () => { pointsFinished++; },
  canUseEnvelope: () => liveSource,
});
const details = editor.element;
const part = (name) => details.querySelector(name);
const canvas = part(".waveshaper-curve");
editor.sync();
assert.equal(canvas.hidden, true, "RADIAS has no misleading static transfer preview");
assert.equal(bindings.size, 12);
assert.ok(part(".ws-morph-knob").children.length === 1, "Morph belongs next to the curve");
bindings.get("color:9:waveshaper_input").set(.75);
assert.deepEqual(writes.at(-1), ["host", "input", .75]);
assert.equal(saved.future.retained, true);
part(".ws-quality").value = "clean4x"; part(".ws-quality").onchange();
assert.equal(saved.quality, "clean4x");
part(".ws-dc").checked = true; part(".ws-dc").onchange();
part(".ws-gain").checked = true; part(".ws-gain").onchange();
assert.equal(saved.dc_block, true);
assert.equal(saved.auto_gain, true);
part(".ws-curve").value = "soft_clip"; part(".ws-curve").onchange();
details.open = true; details.ontoggle();
assert.equal(canvas.hidden, false);
assert.ok(paints > 0, "Static curves are drawn");

// B can be edited even while A is the stateful RADIAS model.
const greenBeforeRadias = paths.filter(p => p.color === "#c4f486").length;
part(".ws-curve").value = "radias"; part(".ws-curve").onchange();
part(".ws-curve-b").value = "custom"; part(".ws-curve-b").onchange();
assert.equal(canvas.hidden, false);
assert.equal(canvas.getAttribute("role"), "slider");
assert.equal(paths.filter(p => p.color === "#c4f486").length, greenBeforeRadias,
  "A stateful RADIAS mix must not be drawn as a static curve");
canvas.onpointerdown({ button: 0, clientX: 160, clientY: 32.5, pointerId: 1, preventDefault() {} });
assert.equal(writes.at(-1)[2], true, "Pointer edits must stream before release");
assert.equal(pointsFinished, 0);
canvas.onpointermove({ clientX: 160, clientY: 32.5 });
assert.equal(writes.at(-1)[2], true);
canvas.onpointerup();
assert.equal(pointsFinished, 1, "Releasing a drag flushes the last point");
assert.equal(saved.points.length, 17);
assert.equal(saved.points[8], .5);
assert.equal(saved.morph ?? 0, 0, "Editing a custom point must not change Morph");
canvas.onkeydown({ key: "ArrowRight", preventDefault() {} });
canvas.onkeydown({ key: "ArrowUp", preventDefault() {} });
assert.ok(Math.abs(saved.points[9] - .145) < 1e-9);
assert.match(canvas.getAttribute("aria-valuetext"), /Point 10 of 17/);
canvas.onkeydown({ key: "End", preventDefault() {} });
assert.equal(saved.points[9], 1);
part(".ws-linear").onclick();
assert.deepEqual(Array.from(saved.points), Array.from({ length: 17 }, (_, i) => i / 8 - 1));
assert.equal(saved.future.retained, true, "Partial edits keep unknown settings");

// The effective mix and the editable custom curve are separate paths. At
// Morph 0 green is A, while the white line passes through B's edited point.
part(".ws-curve").value = "soft_clip"; part(".ws-curve").onchange();
canvas.onpointerdown({ button: 0, clientX: 160, clientY: 32.5, pointerId: 2, preventDefault() {} });
canvas.onpointerup();
let green = paths.filter(p => p.color === "#c4f486").at(-1),
  white = paths.filter(p => p.color === "#e6ebe5").at(-1);
assert.equal(green.points.length, 129);
assert.equal(white.points.length, 17);
assert.ok(Math.abs(green.points[64][1] - 60) < 1e-6);
assert.ok(Math.abs(white.points[8][1] - 32.5) < 1e-6);
assert.equal(part(".waveshaper-curve-key").hidden, false);
part(".ws-morph-b").onclick();
assert.equal(saved.morph, 1);
assert.deepEqual(writes.at(-1), ["host", "morph", 1]);
green = paths.filter(p => p.color === "#c4f486").at(-1);
assert.ok(Math.abs(green.points[64][1] - 32.5) < 1e-6, "Morph 1 must show B");
bindings.get("color:9:waveshaper_morph").set(.5);
green = paths.filter(p => p.color === "#c4f486").at(-1);
assert.ok(Math.abs(green.points[64][1] - 46.25) < 1e-6, "Morph 0.5 must show the A/B mix");
assert.deepEqual(writes.at(-1), ["host", "morph", .5]);
part(".ws-morph-a").onclick();
assert.equal(saved.morph, 0);
assert.deepEqual(writes.at(-1), ["host", "morph", 0]);

// External envelopes follow the same availability rules as filters. A
// missing saved source stays visible; offline Source FX cannot select one.
const env = part(".ws-env-source");
assert.deepEqual(env.children.map(item => item.value), ["", "env1", "env2", "env3"]);
env.value = "env2"; env.onchange();
assert.equal(saved.envelope_source, "env2");
assert.equal(knobNodes.get("color:9:waveshaper_attack").hidden, true);
assert.equal(knobNodes.get("color:9:waveshaper_release").hidden, true);
saved.envelope_source = "env8"; editor.sync();
assert.equal(env.value, "env8");
assert.equal(env.children.find(item => item.value === "env8").disabled, true);
context.preset.env_count = 8; editor.sync();
assert.equal(env.children.find(item => item.value === "env8").disabled, false);
context.preset.tb303 = {}; editor.sync();
assert.equal(env.children.find(item => item.value === "tb303").disabled, false);
env.value = "tb303"; env.onchange();
assert.equal(saved.envelope_source, "tb303");
delete context.preset.tb303; editor.sync();
assert.equal(env.children.find(item => item.value === "tb303").disabled, true);
liveSource = false; editor.sync();
assert.equal(env.disabled, false);
assert.equal(env.value, "tb303", "Offline Source FX must retain the saved source");
assert.equal(env.children.find(item => item.value === "").disabled, false);
assert.ok(env.children.filter(item => item.value).every(item => item.disabled));
env.value = ""; env.onchange();
assert.equal(saved.envelope_source, null);
assert.equal(knobNodes.get("color:9:waveshaper_attack").hidden, false);
assert.equal(knobNodes.get("color:9:waveshaper_release").hidden, false);
liveSource = true; editor.sync();

// Changes to the available envelopes refresh all three owners, including
// already-open editors rather than only newly constructed controls.
const synced = { filter: 0, grain: 0, color: 0 };
context.filterEnvelopeSync = () => { synced.filter++; };
context.grainWaveshaperViews = [{ sync: () => { synced.grain++; } }];
context.colorWaveshaperViews = new Map([[1, { sync: () => { synced.color++; } }]]);
vm.runInContext('filterShaperViews.set("test", { sync: filterEnvelopeSync })', context);
vm.runInContext(app.slice(app.indexOf("function syncWaveshaperEnvelopeSources()"), app.indexOf("function syncEnvs()")), context);
context.syncWaveshaperEnvelopeSources();
assert.deepEqual(synced, { filter: 1, grain: 1, color: 1 });

const curve = context.waveshaperCurve;
const opts = { knee: .5, points: Array.from({ length: 17 }, (_, i) => i / 8 - 1) };
assert.equal(curve("custom", .0625, opts), .0625);
assert.equal(curve("rectify", -.5, opts), .5);
assert.equal(curve("soft_clip", .5, opts), .5);
const gain = .15 + 5.85 * .5;
assert.ok(Math.abs(curve("tanh", .5, opts) - Math.tanh(gain * .5) / Math.tanh(gain)) < 1e-12);
assert.ok(Math.abs(curve("diode", -.5, opts) - Math.tanh(gain * .55 * -.5) / Math.tanh(gain * .55)) < 1e-12);
console.log("Waveshaper editor: options, numeric writes, curve preview, custom pointer and keyboard editing passed.");
