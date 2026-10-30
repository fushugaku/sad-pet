// Section outputs: CHAIN and SEND knobs, the bus menu and heading chips.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const panels = await readFile(new URL("../rust/web/panels.js", import.meta.url), "utf8");
const source = app.slice(app.indexOf("const TAP_STAGES = ["), app.indexOf("function bindGranularToggle("));

class Element {
  constructor(tag = "div") {
    this.tagName = tag.toUpperCase();
    this.children = [];
    this.nodes = new Map();
    this.attributes = new Map();
    this.dataset = {};
    this.style = { setProperty: (key, value) => (this.style[key] = value) };
    this.classes = new Set();
    this.classList = {
      toggle: (name, on) => (on ? this.classes.add(name) : this.classes.delete(name)),
    };
    this.textContent = "";
  }
  set innerHTML(html) {
    for (const [, name] of html.matchAll(/class="([\w-]+)"/g))
      this.nodes.set(`.${name}`, new Element());
    this.nodes.set("select", new Element("select"));
    this.nodes.set(".tap-name span", new Element("span"));
  }
  append(...nodes) {
    this.children.push(...nodes);
  }
  replaceChildren(...nodes) {
    this.children = nodes;
  }
  querySelector(selector) {
    if (!this.nodes.has(selector)) this.nodes.set(selector, new Element());
    return this.nodes.get(selector);
  }
  querySelectorAll(selector) {
    if (selector === "input") {
      const all = node => [node, ...node.children.flatMap(all)];
      return all(this).filter(node => node.tagName === "INPUT");
    }
    return this.children.map((knob) => knob.dial);
  }
  closest() {
    return this.knob;
  }
  setAttribute(key, value) {
    this.attributes.set(key, String(value));
  }
  getAttribute(key) {
    return this.attributes.get(key);
  }
}
const sections = new Map();
const calls = [];
const context = vm.createContext({
  window: {},
  $: () => new Element(),
  preset: { params: {} },
  state: null,
  timbreEditor: null,
  syncFilterOutputs: () => {},
  document: {
    activeElement: null,
    createElement: (tag) => new Element(tag),
    querySelector: (selector) => {
      if (!sections.has(selector)) sections.set(selector, new Element("section"));
      return sections.get(selector);
    },
  },
  percent: (v) => `${Math.round(v * 100)}%`,
  option: (value, label) => Object.assign(new Element("option"), { value: String(value), textContent: label }),
  setText: (element, text) => (element.textContent = String(text)),
  setAttr: (element, key, value) => element.setAttribute(key, value),
  createKnob: ([key, label], bank) => {
    const knob = new Element();
    knob.dial = Object.assign(new Element(), { knob, key, label });
    bank.append(knob);
  },
  scheduleParameter: (id, value) => calls.push(["parameter", id, value]),
  changeTimbre: (path, data) => calls.push(["timbre", path, data]),
});
vm.runInContext(panels, context);
vm.runInContext(`${source}\nbuildTaps();`, context);
const view = (stage) => vm.runInContext(`tapViews.get(${JSON.stringify(stage)})`, context);
const sync = () => vm.runInContext("syncTaps()", context);
const run = (code) => vm.runInContext(code, context);
// Objects made inside the context have its prototypes.
const plain = (value) => JSON.parse(JSON.stringify(value));

context.state = {
  timbres: {
    active: 0,
    timbres: [
      { id: 0, kind: "granular", name: "Granular 1" },
      { id: 4, kind: "bus", bus: 2, name: "Verb" },
      { id: 3, kind: "bus", bus: 1, name: "Bus 1" },
    ],
  },
};
sync();
assert.equal(context.preset.taps, undefined, "Reading the taps changed the preset");
const grains = view("grains");
assert.deepEqual(
  grains.select.children.map((o) => [o.value, o.textContent]),
  [["0", "No bus"], ["1", "Bus 1"], ["2", "Verb"], ["new", "+ New bus"]],
);
assert.equal(grains.send.getAttribute("aria-disabled"), "true", "SEND waits for a bus");
assert.equal(grains.row.classes.has("no-sends"), false);

run('editTap("grains", "bus", 2); editTap("grains", "send", 0.5); editTap("grains", "chain", 0)');
assert.deepEqual(plain(calls.splice(0)), [
  ["parameter", "tap_grains_bus", 2],
  ["parameter", "tap_grains_send", 0.5],
  ["parameter", "tap_grains_chain", 0],
]);
assert.deepEqual(plain(context.preset.taps.grains), { chain: 0, send: 0.5, bus: 2 });
assert.equal(grains.send.getAttribute("aria-disabled"), "false");
assert.ok(grains.row.classes.has("sending") && grains.row.classes.has("skipped"));
assert.ok(grains.section.classes.has("tap-skipped"), "A skipped section dims");
assert.deepEqual(JSON.parse(grains.chips.dataset.labels), [["skip", "Skipped"], ["send", "→ Verb 50%"]]);
assert.equal(grains.row.querySelector(".tap-status").textContent, "Verb only: the chain skips this section");

// Resynth keeps the IDs its send was released with.
run('editTap("resynth", "send", 0.25); editTap("resynth", "chain", 0.5)');
assert.deepEqual(plain(calls.splice(0)), [
  ["parameter", "resynth_send", 0.25],
  ["parameter", "tap_resynth_chain", 0.5],
]);
assert.deepEqual(JSON.parse(view("resynth").chips.dataset.labels), [["chain", "Chain 50%"]]);

// A removed bus stays chosen and is named as gone.
context.state.timbres.timbres.splice(1, 1);
sync();
assert.deepEqual(
  grains.select.children.map((o) => o.textContent),
  ["No bus", "Bus 1", "Bus 2 (removed)", "+ New bus"],
);
assert.equal(grains.row.querySelector(".tap-status").textContent, "Bus 2 is gone: nothing is sent");

// + New bus routes to the next free number and comes back to the timbre.
run('addTapBus("filters")');
assert.deepEqual(plain(calls.splice(0)), [
  ["parameter", "tap_filters_bus", 2],
  ["timbre", "/api/timbres/add", { kind: "bus" }],
  ["timbre", "/api/timbres/select", { id: 0 }],
]);

// A Bus sends to the other buses, but not to one that feeds it, directly
// (Echo, by its sections) or through another bus (Room, by its send to Echo).
const sends = (to) => Array.from({ length: 8 }, (_, i) => Number(i === to - 1));
context.state.timbres.timbres = [
  { id: 0, kind: "granular", name: "Granular 1" },
  { id: 3, kind: "bus", bus: 1, name: "Bus 1", sends: sends(0) },
  { id: 5, kind: "bus", bus: 2, name: "Echo", feeds: 0b1 },
  { id: 6, kind: "bus", bus: 3, name: "Room", sends: sends(2) },
  { id: 7, kind: "bus", bus: 4, name: "Free" },
  { id: 9, kind: "master", name: "Master" },
];
context.state.timbres.active = 3;
context.preset.taps.grains = { chain: 0, send: 0.5, bus: 0 };
sync();
assert.ok(!grains.row.classes.has("no-sends"), "a bus sends");
assert.deepEqual(
  grains.select.children.map((o) => [o.textContent, !!o.disabled]),
  [["No bus", false], ["Echo (feeds this bus)", true], ["Room (feeds this bus)", true], ["Free", false], ["+ New bus", false]],
);
run('editTap("grains", "bus", 4)');
assert.deepEqual(plain(calls.splice(0)), [["parameter", "tap_grains_bus", 4]]);
assert.ok(grains.row.classes.has("sending"));
// A loop made anyway stays chosen and is named, and sends nothing.
context.preset.taps.grains.bus = 2;
sync();
assert.ok(!grains.row.classes.has("sending"));
assert.equal(grains.select.children[1].disabled, false, "the chosen bus stays selectable");
assert.equal(grains.row.querySelector(".tap-status").textContent, "Echo feeds this bus: nothing is sent");
context.preset.taps.grains.bus = 1;
sync();
assert.equal(grains.select.children[1].textContent, "Bus 1 (this bus)");
assert.equal(grains.row.querySelector(".tap-status").textContent, "A bus cannot send to itself");
// Another bus rebuilds the menu from its own place in the graph.
context.state.timbres.active = 5;
sync();
assert.deepEqual(
  grains.select.children.map((o) => [o.textContent, !!o.disabled]),
  [["No bus", false], ["Bus 1", false], ["Room (feeds this bus)", true], ["Free", false], ["+ New bus", false]],
);
// Master sends nowhere: only CHAIN shows there.
context.state.timbres.active = 9;
sync();
assert.ok(grains.row.classes.has("no-sends"));
assert.equal(grains.row.querySelector(".tap-status").textContent, "Skipped: its input goes on");
// A Synth's grains follow its filters.
context.preset.synth = {};
sync();
assert.equal(grains.row.querySelector(".tap-name span").textContent, "Grain engine, after Filters");
console.log("Section outputs: CHAIN/SEND IDs, bus menu, removed buses, + New bus and chips behave.");

// The timbre header keeps independent send controls, including zero sends,
// without rebuilding a slider while it is being dragged.
const header = Object.create(context.window.SadTimbres.prototype);
const route = { key: "group:0", label: "Grain Group A", bus: 1, value: .4, set: value => { route.value = value; } };
const extra = { key: "filter:17", label: "Filter 3", bus: 1, value: 0, set: value => { extra.value = value; } };
header.context = { active: 0, timbres: [
  { id: 0, kind: "rack", sends: Array(8).fill(0) },
  { id: 3, kind: "bus", bus: 1, name: "Bus 1" },
] };
header.hooks = { routes: () => [route, extra] };
header.sends = new Element();
header.syncSends(header.active(), false);
assert.deepEqual(header.sends.children.map(n => n.textContent), ["→ Bus 1", "Grain Group A → Bus 1", "Filter 3 → Bus 1"]);
const slider = header.sends.querySelectorAll("input")[1];
assert.equal(Number(slider.value), .4);
slider.value = ".75"; slider.oninput();
assert.equal(route.value, .75);
header.syncSends(header.active(), false);
assert.equal(header.sends.querySelectorAll("input")[1], slider);
header.context.timbres[1].name = "Delay";
header.syncSends(header.active(), false);
assert.equal(header.sends.children[1].textContent, "Grain Group A → Delay");
header.context.timbres.pop();
header.syncSends(header.active(), false);
assert.equal(header.sends.children[0].textContent, "Grain Group A → Bus 1 (removed)");
assert.equal(header.sends.querySelectorAll("input")[0].disabled, true);
console.log("Timbre send overview: named sources, zero levels, slider edits, stable drag and renamed/removed buses passed.");
