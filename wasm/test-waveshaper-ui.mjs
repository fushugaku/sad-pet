// Run the shipping shared waveshaper editor with a small DOM and canvas.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const source = app.slice(app.indexOf("const WAVESHAPER_OPTION_DEFAULTS ="), app.indexOf("function scheduleDrum("));
assert.ok(source.includes("function buildWaveshaperOptions"));

const bindings = new Map();
let paints = 0;
const context2d = new Proxy({}, {
  get: () => () => { paints++; },
  set: () => true,
});
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
  document, structuredClone,
  percent: (value) => `${Math.round(value * 100)}%`,
  signedPercent: (value) => `${Math.round(value * 100)}%`,
  WAVESHAPER_TYPES: [["drive", "Drive"]],
  option: (value, label) => ({ value, label }),
  requestAnimationFrame: (fn) => { fn(); return 1; },
  createKnob: (schema, bank, binding) => {
    bindings.set(schema[0], binding);
    bank.append(new Element());
    return () => {};
  },
});
vm.runInContext(source, context);
assert.equal(vm.runInContext("WAVESHAPER_OPTION_DEFAULTS.curve_b", context), "soft_clip");
const format = (field, value) => vm.runInContext(`WAVESHAPER_OPTION_KNOBS.find(([name]) => name === '${field}')[2](${value})`, context);
assert.equal(format("bass_frequency", 0), "40 Hz");
assert.equal(format("bass_frequency", 1), "640 Hz");
assert.equal(format("attack", 1), "200.0 ms");
assert.equal(format("release", 1), "1000 ms");

let saved = { future: { retained: true } };
const writes = [];
let editor;
editor = context.buildWaveshaperOptions({
  label: "FX 1 waveshaper", key: "color:9", register: true,
  get: () => saved,
  set: (patch) => { saved = { ...saved, ...patch }; writes.push(["preset", patch]); editor.sync(); },
  setNumeric: (field, value) => { saved = { ...saved, [field]: value }; writes.push(["host", field, value]); editor.sync(); },
});
const details = editor.element;
const part = (name) => details.querySelector(name);
const canvas = part(".waveshaper-curve");
editor.sync();
assert.equal(canvas.hidden, true, "RADIAS has no misleading static transfer preview");
assert.equal(bindings.size, 12);
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
part(".ws-curve").value = "radias"; part(".ws-curve").onchange();
part(".ws-curve-b").value = "custom"; part(".ws-curve-b").onchange();
assert.equal(canvas.hidden, false);
assert.equal(canvas.getAttribute("role"), "slider");
canvas.onpointerdown({ button: 0, clientX: 160, clientY: 32.5, pointerId: 1, preventDefault() {} });
canvas.onpointerup();
assert.equal(saved.points.length, 17);
assert.equal(saved.points[8], .5);
canvas.onkeydown({ key: "ArrowRight", preventDefault() {} });
canvas.onkeydown({ key: "ArrowUp", preventDefault() {} });
assert.ok(Math.abs(saved.points[9] - .145) < 1e-9);
assert.match(canvas.getAttribute("aria-valuetext"), /Point 10 of 17/);
canvas.onkeydown({ key: "End", preventDefault() {} });
assert.equal(saved.points[9], 1);
part(".ws-linear").onclick();
assert.deepEqual(Array.from(saved.points), Array.from({ length: 17 }, (_, i) => i / 8 - 1));
assert.equal(saved.future.retained, true, "Partial edits keep unknown settings");

const curve = context.waveshaperCurve;
const opts = { knee: .5, points: Array.from({ length: 17 }, (_, i) => i / 8 - 1) };
assert.equal(curve("custom", .0625, opts), .0625);
assert.equal(curve("rectify", -.5, opts), .5);
assert.equal(curve("soft_clip", .5, opts), .5);
const gain = .15 + 5.85 * .5;
assert.ok(Math.abs(curve("tanh", .5, opts) - Math.tanh(gain * .5) / Math.tanh(gain)) < 1e-12);
assert.ok(Math.abs(curve("diode", -.5, opts) - Math.tanh(gain * .55 * -.5) / Math.tanh(gain * .55)) < 1e-12);
console.log("Waveshaper editor: options, numeric writes, curve preview, custom pointer and keyboard editing passed.");
