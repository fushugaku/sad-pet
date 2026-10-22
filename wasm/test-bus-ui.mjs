// Grain Engine controls follow each timbre's own history: a Bus or Master with
// Granular on must stay editable even though its input arrives "live".
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const slice = (start, end) => app.slice(app.indexOf(start), app.indexOf(end));
const source =
  slice("const historySettings", "function grainSeconds(") +
  slice("function syncInput(", "async function poll(");

class Element {
  constructor() {
    this.style = {};
    this.dataset = {};
    this.classes = new Set();
    this.classList = {
      toggle: (name, on) => (on ? this.classes.add(name) : this.classes.delete(name)),
    };
    this.parentElement = this;
  }
  setAttribute() {}
  getAttribute() {}
  append() {}
  replaceChildren() {}
}
const elements = new Map();
const context = vm.createContext({
  preset: null,
  inputDevicesKey: "",
  window: {},
  percent: (v) => `${v}`,
  option: () => new Element(),
  document: {
    activeElement: null,
    getElementById(id) {
      if (!elements.has(id)) elements.set(id, new Element());
      return elements.get(id);
    },
    querySelector: () => new Element(),
  },
});
vm.runInContext(`const $ = (id) => document.getElementById(id);\n${source}`, context);

const grains = elements.get("grain-controls") ?? context.document.getElementById("grain-controls");
const check = (preset, input, editable, label) => {
  context.preset = preset;
  vm.runInContext(`syncInput(${JSON.stringify(input)})`, context);
  assert.equal(grains.inert, !editable, `${label}: grain controls ${editable ? "locked" : "editable"}`);
  assert.equal(grains.classes.has("inactive"), !editable, `${label}: dimming`);
};
// A Granular Bus or Master reports its history as an active live input.
const live = { active: true, mode: "effects", peak: [0, 0] };
check({ bus: { master: true, granular: true } }, live, true, "Granular Master");
check({ bus: { granular: true } }, live, true, "Granular Bus");
check({ bus: { master: true, granular: false } }, { active: false, peak: [0, 0] }, false, "Master without Granular");
check({ drums: { granular: true } }, live, true, "Granular drums");
check({ synth: { granular: false } }, live, false, "Synth without Granular");
check({}, live, false, "Microphone in effects mode");
check({}, { ...live, mode: "grains" }, true, "Microphone in grains mode");
check({}, { active: false, peak: [0, 0] }, true, "File source");
console.log("Grain controls: Bus, Master, drums, Synth and microphone modes lock exactly when grains are off.");
