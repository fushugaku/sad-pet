// Advanced grain mode: the switch, group tabs and each group's controls.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const source = app.slice(app.indexOf("const GRAIN_GROUP_NAMES = ["), app.indexOf("function createKnob("));

class Element {
  constructor(tag = "div") {
    this.tagName = tag.toUpperCase();
    this.children = [];
    this.attributes = new Map();
    this.dataset = {};
    this.style = { setProperty: (key, value) => (this.style[key] = value) };
    this.classes = new Set();
    this.classList = {
      toggle: (name, on) => (on ? this.classes.add(name) : this.classes.delete(name)),
      contains: (name) => this.classes.has(name),
    };
    this.textContent = "";
    this.hidden = false;
    this.value = "";
  }
  set innerHTML(html) {
    // Card shells: a title row, then the knob grid; Output rows: knobs, bus
    // menu and status.
    this.children = [];
    this.parts = new Map();
    if (html.includes("grain-group-card-title"))
      this.children.push(new Element(), new Element());
    else if (html.includes("<small>")) this.children.push(new Element("i"), new Element("small"));
    else if (html.includes("tap-knobs"))
      for (const [selector, tag] of [[".tap-knobs"], ["select", "select"], [".tap-status", "output"]]) {
        const part = new Element(tag);
        this.parts.set(selector, part);
        this.children.push(part);
      }
  }
  get firstElementChild() {
    return this.children[0];
  }
  get lastElementChild() {
    return this.children.at(-1);
  }
  append(...nodes) {
    for (const node of nodes) {
      node.parent?.children.splice(node.parent.children.indexOf(node), 1);
      node.parent = this;
    }
    this.children.push(...nodes);
  }
  prepend(node) {
    node.parent = this;
    this.children.unshift(node);
  }
  get parentElement() {
    return this.parent;
  }
  querySelector(selector) {
    const all = (node) => [node, ...node.children.flatMap(all)];
    const card = /^\[data-card="(.+)"\]$/.exec(selector)?.[1];
    if (card) return all(this).find((n) => n.dataset.card === card);
    if (selector === ".grain-group-chain")
      return all(this).find((n) => n.className?.includes?.("grain-group-chain"));
    return this.parts?.get(selector);
  }
  querySelectorAll(selector) {
    const all = (node) => [node, ...node.children.flatMap(all)];
    if (selector === ".grain-group-menu")
      return all(this).filter((n) => n.className === "grain-group-menu");
    return selector === ".dial" ? this.children.map((knob) => knob.dial) : [];
  }
  closest(selector) {
    return selector === ".knob" ? this.knob : null;
  }
  setAttribute(key, value) {
    this.attributes.set(key, String(value));
  }
  get className() {
    return this._className ?? "";
  }
  set className(name) {
    this._className = name;
    this.classes = new Set(name.split(" ").filter(Boolean));
  }
  getAttribute(key) {
    return this.attributes.get(key);
  }
  focus() {}
}
const elements = new Map();
const $ = (id) => {
  if (!elements.has(id)) elements.set(id, new Element(id === "grain-group-curve" ? "canvas" : "div"));
  return elements.get(id);
};
const shapeKnob = new Element();
const shapeDial = Object.assign(new Element(), { knob: shapeKnob });
const knobs = new Map();
const controls = new Map();
const calls = [];
const context = vm.createContext({
  $,
  controls,
  preset: { params: { grain_shape: 0.35, grain_attack: 0.2, grain_size: 0.75 } },
  document: {
    createElement: (tag) => new Element(tag),
    querySelector: (selector) =>
      selector.includes('data-control-key="grain_shape"') ? shapeDial : null,
  },
  percent: (v) => `${Math.round(v * 100)}%`,
  signedPercent: (v) => `${v > 0 ? "+" : ""}${Math.round(v * 100)}%`,
  grainSeconds: (v) => 0.01 + 2 * Math.abs(v * 2 - 1) ** 3,
  option: (value, label) => Object.assign(new Element("option"), { value, textContent: label }),
  setText: (element, text) => (element.textContent = String(text)),
  setAttr: (element, key, value) => element.setAttribute(key, value),
  resizeCanvas: () => [
    new Proxy({}, { get: () => () => {}, set: () => true }),
    200,
    46,
  ],
  createKnob: ([key, label, format, min, max, aria], bank, binding) => {
    knobs.set(key, { label, format, min, max, aria, binding });
    controls.set(key, () => {});
    const knob = new Element();
    knob.dial = Object.assign(new Element(), { knob });
    bank.append(knob);
  },
  sendTargets: () => ({ sends: true, buses: [{ bus: 1, name: "Bus 1" }, { bus: 2, name: "Verb" }] }),
  fillBusMenu: (select, chosen, buses) => {
    select.choices = [0, ...buses.map((b) => b.bus), "new"];
    select.value = String(chosen);
  },
  addTapBus: (stage, choose) => {
    calls.push(["new bus", stage]);
    choose(3);
  },
  scheduleParameter: (id, value) => calls.push(["parameter", id, value]),
  schedulePreset: () => calls.push(["preset"]),
  structuredClone,
});
const run = (code) => vm.runInContext(code, context);
run(`${source}\nbuildGrainGroups(); syncGrainGroups();`);

// Four groups of twenty-eight knobs each, and CHAIN and SEND, under the host IDs.
assert.equal(knobs.size, 4 * 30);
assert.equal(knobs.get("grain_group2_attack").aria, "Group B attack");
assert.equal(knobs.get("grain_group2_attack").binding.get(), 1);
assert.equal(knobs.get("grain_group2_filter2_cutoff").aria, "Group B filter 2 cutoff");
assert.equal(knobs.get("grain_group3_random_pitch").aria, "Group C random pitch");
assert.equal(knobs.get("grain_group1_size").format(-2), "×0.25");
assert.equal(knobs.get("grain_group1_size").format(1), "×2");
assert.equal(knobs.get("grain_group1_random_cutoff").format(0.3), "±30%");
assert.equal(knobs.get("grain_group2_send").aria, "Group B bus send");
assert.ok(knobs.has("grain_group1_share") && knobs.has("grain_group4_filter_release"));
assert.equal(knobs.get("grain_group2_pitch").aria, "Group B pitch");
assert.equal(knobs.get("grain_group3_amp_attack").aria, "Group C amp envelope attack");
assert.equal(knobs.get("grain_group1_pitch").min, -24);
assert.equal(knobs.get("grain_group3_drive").aria, "Group C drive");
assert.equal(knobs.get("grain_group3_drive").binding.get(), 0);
assert.equal(knobs.get("grain_group4_waveshaper_depth").aria, "Group D waveshaper depth");
assert.equal(knobs.get("grain_group4_waveshaper_depth").format(64 / 127), "64");

// Off: no groups panel, the main SHAPE plays.
assert.equal($("grain-groups").hidden, true);
assert.equal($("grain-advanced").getAttribute("aria-pressed"), "false");
assert.equal(shapeDial.getAttribute("aria-disabled"), "false");

// The first switch-on starts every group from the engine's SHAPE and ATTACK
// in one preset write.
$("grain-advanced").onclick();
assert.deepEqual(calls.splice(0), [["preset"]]);
const groups = () => JSON.parse(run("JSON.stringify(preset.grain_groups)"));
assert.equal(groups().advanced, true);
assert.deepEqual(groups().groups.map((g) => g.shape), [0.35, 0.35, 0.35, 0.35]);
assert.deepEqual(groups().groups.map((g) => g.attack), [0.2, 0.2, 0.2, 0.2]);
assert.equal($("grain-groups").hidden, false);
assert.equal(shapeDial.getAttribute("aria-disabled"), "true");
assert.ok(shapeKnob.classes.has("is-replaced"));

// Knobs write their group, nested envelopes included, as numeric parameters.
knobs.get("grain_group2_amp_attack").binding.set(0.25);
knobs.get("grain_group4_cutoff").binding.set(0.4);
assert.deepEqual(calls.splice(0), [
  ["parameter", "grain_group2_amp_attack", 0.25],
  ["parameter", "grain_group4_cutoff", 0.4],
]);
assert.equal(groups().groups[1].amp.attack, 0.25);
assert.equal(groups().groups[1].amp.sustain, 1);
assert.equal(knobs.get("grain_group2_amp_attack").binding.get(), 0.25);
assert.equal(knobs.get("grain_group1_amp_attack").binding.get(), 0.5);
// Pitch moves in whole semitones; envelope times read in grain milliseconds.
assert.equal(knobs.get("grain_group1_pitch").binding.quantize(6.6), 7);
assert.equal(knobs.get("grain_group1_pitch").format(7), "+7 st");
assert.equal(knobs.get("grain_group1_amp_decay").format(0.5), "50% 130ms");
run("preset.params.grain_size = 1");
assert.equal(knobs.get("grain_group1_amp_decay").format(0.6), "60% 1.2s");

// The filter type goes by its host index.
const pages = $("grain-group-pages").children;
const menu = (page, field) =>
  pages[page].querySelectorAll(".grain-group-menu").find((s) => s.dataset.field === field);
const select = menu(0, "filter_type");
select.value = "high_pass";
select.onchange();
assert.deepEqual(calls.splice(0), [["parameter", "grain_group1_filter_type", 2]]);
assert.equal(groups().groups[0].filter_type, "high_pass");
// The scale goes by its host index too, and placement knobs write the group.
const scale = menu(0, "scale");
assert.equal(scale.value, "off");
scale.value = "chord";
scale.onchange();
knobs.get("grain_group1_size").binding.set(-1);
knobs.get("grain_group1_spread").binding.set(0.5);
assert.deepEqual(calls.splice(0), [
  ["parameter", "grain_group1_scale", 12],
  ["parameter", "grain_group1_size", -1],
  ["parameter", "grain_group1_spread", 0.5],
]);
assert.deepEqual(
  [groups().groups[0].scale, groups().groups[0].size, groups().groups[0].spread],
  ["chord", -1, 0.5],
);
// Filter 2 is routed off until its menu says Serial or Parallel.
const routing = menu(0, "filter_routing");
assert.equal(routing.value, "single");
assert.equal(routing.getAttribute("aria-label"), "Group A filter 2 routing");
routing.value = "parallel";
routing.onchange();
const type2 = menu(0, "filter2_type");
type2.value = "band_pass";
type2.onchange();
assert.deepEqual(calls.splice(0), [
  ["parameter", "grain_group1_filter_routing", 2],
  ["parameter", "grain_group1_filter2_type", 1],
]);
assert.deepEqual(
  [groups().groups[0].filter_routing, groups().groups[0].filter2_type],
  ["parallel", "band_pass"],
);

// The waveshaper goes by its host index, Off first; DEPTH dims while it is
// off and DRIVE writes the group.
const shaper = menu(0, "waveshaper");
assert.equal(shaper.value, "off");
assert.equal(shaper.getAttribute("aria-label"), "Group A waveshaper");
const depthKnob = pages[0].querySelector('[data-card="Drive · Waveshaper"]').lastElementChild.children[1];
assert.ok(depthKnob.classes.has("is-idle"));
shaper.value = "hard_clip";
shaper.onchange();
knobs.get("grain_group1_drive").binding.set(0.6);
assert.deepEqual(calls.splice(0), [
  ["parameter", "grain_group1_waveshaper", 2],
  ["parameter", "grain_group1_drive", 0.6],
]);
assert.deepEqual([groups().groups[0].waveshaper, groups().groups[0].drive], ["hard_clip", 0.6]);
assert.ok(!depthKnob.classes.has("is-idle"));
assert.equal(knobs.get("grain_group1_waveshaper_depth").binding.get(), 64 / 127);

// Each page has three rows: Grain, Pitch and Playback; the stages in the
// group's order; the envelopes; then its Output.
const rowCards = (page) =>
  pages[page].children.slice(0, 3).map((row) => row.children.map((card) => card.dataset.card));
assert.deepEqual(rowCards(1), [
  ["Grain", "Pitch", "Playback"],
  ["Drive · Waveshaper", "Filter 1", "Filter 2"],
  ["Amp envelope", "Filter envelope"],
]);
const chainOf = (page) => pages[page].querySelector(".grain-group-chain");
const stagesOf = (page) => chainOf(page).children.map((card) => card.dataset.stage);
const joinsOf = (page) => chainOf(page).children.map((card) => card.dataset.join);
// Group A's filters run in parallel: the later filter is mixed in.
assert.deepEqual(joinsOf(0), ["", "›", "+"]);
assert.deepEqual(joinsOf(1), ["", "›", "›"]);
// The arrow keys move a stage by its grip and write the order by host index.
const grip = (page, stage) =>
  chainOf(page).children.find((card) => card.dataset.stage === stage).firstElementChild.children[0];
assert.equal(grip(0, "drive").getAttribute("aria-label"), "Move Group A Drive · Waveshaper");
const key = (target, name) => target.onkeydown({ key: name, preventDefault() {} });
key(grip(0, "drive"), "ArrowRight");
assert.deepEqual(calls.splice(0), [["parameter", "grain_group1_order", 2]]);
assert.deepEqual(groups().groups[0].order, ["filter1", "drive", "filter2"]);
assert.deepEqual(stagesOf(0), ["filter1", "drive", "filter2"]);
// Drive between parallel filters rides the earlier one's branch.
assert.deepEqual(joinsOf(0), ["", "›", "+"]);
key(grip(0, "drive"), "ArrowRight");
key(grip(0, "filter1"), "ArrowLeft");
assert.deepEqual(calls.splice(0), [["parameter", "grain_group1_order", 3]]);
key(grip(0, "filter2"), "ArrowLeft");
assert.deepEqual(calls.splice(0), [["parameter", "grain_group1_order", 5]]);
assert.deepEqual(stagesOf(0), ["filter2", "filter1", "drive"]);
assert.deepEqual(joinsOf(0), ["", "+", "›"]);
// An order from the host rearranges the cards; one that is not every stage
// once shows the usual order, as the engine plays it.
run(`preset.grain_groups.groups[1].order = ["filter2", "drive", "filter1"]; syncGrainGroups();`);
assert.deepEqual(stagesOf(1), ["filter2", "drive", "filter1"]);
run(`preset.grain_groups.groups[1].order = ["drive", "drive"]; syncGrainGroups();`);
assert.deepEqual(stagesOf(1), ["drive", "filter1", "filter2"]);

// Tabs show each group's share, Off at zero, and pick the page.
knobs.get("grain_group3_share").binding.set(0);
calls.length = 0;
const tabs = $("grain-group-tabs").children;
assert.equal(tabs[2].lastElementChild.textContent, "Off");
assert.ok(tabs[2].classes.has("is-off"));
assert.equal(tabs[0].lastElementChild.textContent, "100%");
tabs[1].onclick();
assert.deepEqual([...pages].map((p) => p.hidden), [true, false, true, true]);
assert.deepEqual(
  [...tabs].map((t) => t.getAttribute("aria-selected")),
  ["false", "true", "false", "false"],
);

// Each group's Output row routes its grains to a bus of its own.
const output = pages[1].lastElementChild;
const busMenu = output.querySelector("select"),
  sendDial = knobs.get("grain_group2_send");
assert.equal(output.className, "tap grain-group-output");
busMenu.value = "2";
busMenu.onchange();
assert.deepEqual(calls.splice(0), [["parameter", "grain_group2_bus", 2]]);
assert.deepEqual(groups().groups[1].output, { chain: 1, send: 0, bus: 2 });
assert.equal(output.querySelector(".tap-status").textContent, "Turn SEND up to feed Verb");
sendDial.binding.set(1);
knobs.get("grain_group2_chain").binding.set(0);
assert.deepEqual(calls.splice(0), [
  ["parameter", "grain_group2_send", 1],
  ["parameter", "grain_group2_chain", 0],
]);
assert.equal(output.querySelector(".tap-status").textContent, "Verb only: the timbre leaves this group out");
assert.ok(output.classes.has("sending") && output.classes.has("skipped"));
assert.equal(tabs[1].lastElementChild.textContent, "100% → Verb");
// + New bus adds the next bus and routes the group to it.
busMenu.value = "new";
busMenu.onchange();
assert.deepEqual(calls.splice(0), [["new bus", null], ["parameter", "grain_group2_bus", 3]]);
assert.equal(output.querySelector(".tap-status").textContent, "Bus 3 is gone: nothing is sent");

// Switching off again is one numeric change and keeps the groups.
$("grain-advanced").onclick();
assert.deepEqual(calls.splice(0), [["parameter", "grain_advanced", 0]]);
assert.equal(groups().advanced, false);
assert.equal(groups().groups[0].filter_type, "high_pass");
assert.equal($("grain-groups").hidden, true);
assert.equal(shapeDial.getAttribute("aria-disabled"), "false");

console.log("grain groups UI ok");
