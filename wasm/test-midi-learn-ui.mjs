import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

// Just enough DOM for createKnob, the knob menu and its badge.
class ClassList {
  constructor() { this.names = new Set(); }
  add(...names) { names.forEach((n) => this.names.add(n)); }
  remove(...names) { names.forEach((n) => this.names.delete(n)); }
  toggle(name, force = !this.names.has(name)) {
    force ? this.names.add(name) : this.names.delete(name);
    return force;
  }
  contains(name) { return this.names.has(name); }
}
const KNOB_PARTS = [".dial", "output", ".pointer", ".base-pointer", ".arc", ".modulation-range", ".modulation-live", ".knob-mod-range", ".knob-label"];
class Element {
  constructor(tag) {
    this.tagName = tag.toUpperCase();
    this.children = [];
    this.parentNode = null;
    this.attributes = new Map();
    this.dataset = {};
    this.events = {};
    this.style = {};
    this.classList = new ClassList();
    this.textContent = "";
  }
  set className(value) {
    this.classList = new ClassList();
    this.classList.add(...value.split(/\s+/).filter(Boolean));
  }
  get className() { return [...this.classList.names].join(" "); }
  set innerHTML(_) {}
  append(...nodes) {
    for (const node of nodes) {
      node.remove();
      node.parentNode = this;
      this.children.push(node);
    }
  }
  remove() {
    if (!this.parentNode) return;
    this.parentNode.children = this.parentNode.children.filter((n) => n !== this);
    this.parentNode = null;
  }
  all() { return [this, ...this.children.flatMap((n) => n.all())]; }
  querySelector(selector) {
    const name = selector.replace(/^\./, "");
    const found = this.all().slice(1).find((n) => n.classList.contains(name) || n.tagName === name.toUpperCase());
    if (found || !KNOB_PARTS.includes(selector)) return found ?? null;
    const part = new Element(selector.startsWith(".") ? "div" : selector);
    part.classList.add(name);
    this.append(part);
    return part;
  }
  closest(selector) {
    for (let node = this; node; node = node.parentNode)
      if (node.classList.contains(selector.replace(/^\./, ""))) return node;
    return null;
  }
  contains(node) {
    for (; node; node = node.parentNode) if (node === this) return true;
    return false;
  }
  get isConnected() { return document.body.contains(this); }
  get firstElementChild() { return this.children[0] ?? null; }
  setAttribute(key, value) { this.attributes.set(key, String(value)); }
  getAttribute(key) { return this.attributes.get(key) ?? null; }
  removeAttribute(key) { this.attributes.delete(key); }
  addEventListener(name, callback) { (this.events[name] ||= []).push(callback); }
  dispatch(name, event = {}) {
    for (const callback of this.events[name] || []) callback({ preventDefault() {}, stopPropagation() {}, target: this, ...event });
  }
  click() { this.onclick?.(); }
  focus() { document.activeElement = this; }
  setPointerCapture() {}
  getBoundingClientRect() { return { left: 10, top: 10, right: 87, bottom: 87, width: 120, height: 60 }; }
}
const listeners = new Set();
const document = {
  activeElement: null,
  body: new Element("body"),
  createElement: (tag) => new Element(tag),
  addEventListener: (name, callback) => listeners.add(callback),
  removeEventListener: (name, callback) => listeners.delete(callback),
};
const stored = new Map();
const localStorage = {
  getItem: (key) => stored.get(key) ?? null,
  setItem: (key, value) => stored.set(key, String(value)),
};
const midi = await readFile(new URL("../rust/web/midi.js", import.meta.url), "utf8");
const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const knobSource = app.slice(app.indexOf("function createKnob("), app.indexOf("function syncControl("));
function editor() {
  const context = vm.createContext({
    document, localStorage, innerWidth: 1200, innerHeight: 800,
    addEventListener() {}, removeEventListener() {},
    preset: {}, state: { knobs: {} }, editing: 0,
    controls: new Map(), modulationViews: new Map(),
  });
  context.window = context;
  vm.runInContext(midi, context);
  vm.runInContext(knobSource, context);
  return context;
}
const notices = [];
let context = editor();
const learn = context.SadMidiLearn;
learn.setup({ notify: (text) => notices.push(text) });
const bank = new Element("div");
document.body.append(bank);
const values = { cutoff: 0.5, pitch: 60 };
let disabled = false;
const knob = (key, label, binding) => {
  context.createKnob([key, label, String, binding.min ?? 0, binding.max ?? 1, label], bank, { register: false, ...binding });
  return bank.children.at(-1);
};
const cutoff = knob("filter", "Filter 1 cutoff", {
  get: () => values.cutoff, set: (v) => { if (!disabled) values.cutoff = v; }, reset: () => 0.5,
});
const pitch = knob("drum1_9", "Kick pitch", {
  min: 0, max: 127, step: 1,
  get: () => values.pitch, set: (v) => (values.pitch = v), reset: () => 60,
});
// Objects from the editor's realm compare by value.
const plain = (value) => JSON.parse(JSON.stringify(value));
const dial = (box) => box.querySelector(".dial");
const menu = () => document.body.children.find((n) => n.classList.contains("sad-menu-layer"));
const items = () => menu()?.children[0].children.map((item) => item.children[0].textContent) ?? [];
const choose = (text) => menu().children[0].children.find((item) => item.children[0].textContent === text).click();

// The press that opens the menu does not turn the knob.
const press = (box, button) => {
  dial(box).dispatch("pointerdown", { button, pointerId: 1, clientX: 0, clientY: 100 });
  if (button === 0) dial(box).dispatch("contextmenu", {});
  dial(box).dispatch("pointermove", { clientX: 0, clientY: 0 });
  dial(box).dispatch("pointerup");
  menu()?.onkeydown({ key: "Escape", preventDefault() {}, stopPropagation() {} });
};
press(cutoff, 2);
press(cutoff, 0);
assert.equal(values.cutoff, 0.5, "opening the knob's menu dragged the knob");

// Right-click → MIDI learn → the next controller turns the knob.
let prevented = false;
dial(cutoff).dispatch("contextmenu", { clientX: 40, clientY: 50, preventDefault: () => (prevented = true) });
assert.ok(prevented, "the browser's own menu opened over the knob");
assert.deepEqual(items(), ["MIDI learn"]);
choose("MIDI learn");
assert.equal(menu(), undefined, "the menu stayed open");
assert.equal(learn.learning(), "filter");
assert.ok(cutoff.classList.contains("midi-learning"));
assert.match(notices.at(-1), /move a controller to turn Filter 1 cutoff/);
learn.input(0, 120, 0);
assert.equal(learn.learning(), "filter", "a channel mode message was learned");
learn.input(0, 74, 127);
assert.equal(learn.learning(), null);
assert.ok(!cutoff.classList.contains("midi-learning"));
assert.deepEqual(plain(learn.mappings()), { filter: [0, 74] });
assert.equal(values.cutoff, 1, "the learning controller did not move the knob");
assert.equal(dial(cutoff).querySelector(".knob-midi").textContent, "74");
assert.equal(dial(cutoff).dataset.midi, "CC 74 · ch 1");
assert.equal(notices.at(-1), "CC 74 · ch 1 → Filter 1 cutoff");
learn.input(0, 74, 0);
assert.equal(values.cutoff, 0);
learn.input(1, 74, 90);
learn.input(0, 75, 90);
assert.equal(values.cutoff, 0, "another channel or controller turned the knob");
learn.bytes(new Uint8Array([0xb0, 74, 64]));
assert.equal(values.cutoff, 64 / 127);
learn.bytes(new Uint8Array([0x90, 74, 100]));
assert.equal(values.cutoff, 64 / 127, "a note turned the knob");
disabled = true;
learn.input(0, 74, 127);
assert.equal(values.cutoff, 64 / 127, "a disabled knob moved");
disabled = false;

// Stepped knobs land on their steps; learning a controller again moves it.
dial(pitch).dispatch("contextmenu", {});
assert.deepEqual(items(), ["MIDI learn", "Forget all MIDI mappings (1)"]);
choose("MIDI learn");
learn.input(0, 74, 33);
assert.deepEqual(plain(learn.mappings()), { drum1_9: [0, 74] });
assert.equal(dial(cutoff).querySelector(".knob-midi"), null, "the old knob kept its badge");
assert.equal(values.pitch, 33);
learn.input(0, 74, 100);
assert.equal(values.pitch, 100);
assert.equal(values.cutoff, 64 / 127, "the knob the controller left still followed it");
assert.ok(Number.isInteger(values.pitch));

// Esc and the menu cancel learning.
learn.learn("filter");
for (const listener of listeners) listener({ key: "Escape", preventDefault() {}, stopPropagation() {} });
assert.equal(learn.learning(), null);
assert.equal(listeners.size, 0, "learning left its Esc listener behind");
dial(cutoff).dispatch("contextmenu", {});
choose("MIDI learn");
dial(cutoff).dispatch("contextmenu", {});
assert.deepEqual(items(), ["Cancel MIDI learn", "Forget all MIDI mappings (1)"]);
const layer = menu();
layer.onkeydown({ key: "ArrowDown", preventDefault() {}, stopPropagation() {} });
assert.ok(layer.children[0].children[1].classList.contains("is-active"));
layer.onkeydown({ key: "ArrowUp", preventDefault() {}, stopPropagation() {} });
layer.onkeydown({ key: "Enter", preventDefault() {}, stopPropagation() {} });
assert.equal(learn.learning(), null);
assert.equal(menu(), undefined);

// A rebuilt panel's knob shows and follows the mapping; the old one is gone.
pitch.remove();
const rebuilt = knob("drum1_9", "Kick pitch", {
  min: 0, max: 127, step: 1,
  get: () => values.pitch, set: (v) => (values.pitch = v), reset: () => 60,
});
assert.equal(dial(rebuilt).querySelector(".knob-midi").textContent, "74");
learn.input(0, 74, 1);
assert.equal(values.pitch, 1);

// Mappings survive a new editor, and the menu forgets them.
assert.deepEqual(JSON.parse(stored.get("sad-pet-midi-map")), { drum1_9: [0, 74] });
context = editor();
assert.deepEqual(plain(context.SadMidiLearn.mappings()), { drum1_9: [0, 74] });
stored.set("sad-pet-midi-map", JSON.stringify({ a: [0, 7], b: [16, 1], c: [0, 127], d: "x" }));
assert.deepEqual(plain(editor().SadMidiLearn.mappings()), { a: [0, 7] }, "invalid saved mappings loaded");
dial(rebuilt).dispatch("contextmenu", {});
assert.deepEqual(items(), ["MIDI learn", "Forget CC 74 · ch 1"]);
choose("Forget CC 74 · ch 1");
assert.deepEqual(plain(learn.mappings()), {});
assert.equal(dial(rebuilt).querySelector(".knob-midi"), null);
learn.input(0, 74, 127);
assert.equal(values.pitch, 1, "a forgotten controller still turned the knob");
console.log("MIDI learn UI tests passed");
