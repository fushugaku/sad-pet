import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

// Just enough DOM for createKnob, the knob menu (SadLazySelect) and its badge.
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
  // Clicks bubble: the menu handles its rows' clicks on its layer.
  click() {
    for (let node = this; node; node = node.parentNode) node.onclick?.({ target: this, detail: 1 });
  }
  focus() { document.activeElement = this; }
  setPointerCapture() {}
  getBoundingClientRect() { return { left: 10, top: 10, right: 87, bottom: 87, width: 120, height: 60 }; }
}
const listeners = new Set();
const document = {
  activeElement: null,
  body: new Element("body"),
  documentElement: { clientWidth: 1200, clientHeight: 800 },
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
const sequence = await readFile(new URL("../rust/web/sequence.js", import.meta.url), "utf8");
const menus = sequence.slice(0, sequence.indexOf("/* Neuropolis"));
const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const knobSource = app.slice(app.indexOf("function createKnob("), app.indexOf("function syncControl("));
function editor() {
  const context = vm.createContext({
    document, localStorage, innerWidth: 1200, innerHeight: 800,
    addEventListener() {}, removeEventListener() {}, setTimeout, clearTimeout,
    preset: {}, state: { knobs: {} }, editing: 0,
    controls: new Map(), modulationViews: new Map(), knobUnits: new Map(),
  });
  context.window = context;
  vm.runInContext(menus, context);
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
assert.equal(learn.bytes(new Uint8Array([0xb0, 74, 64])), true, "a learned CC was not consumed");
assert.equal(values.cutoff, 64 / 127);
assert.equal(learn.bytes(new Uint8Array([0x90, 74, 100])), false);
assert.equal(learn.bytes(new Uint8Array([0xb0, 75, 100])), false, "an unmapped CC was consumed");
assert.equal(values.cutoff, 64 / 127, "a note turned the knob");
disabled = true;
learn.input(0, 74, 127);
assert.equal(values.cutoff, 64 / 127, "a disabled knob moved");
disabled = false;

// Stepped knobs land on their steps; learning a controller again moves it.
dial(pitch).dispatch("contextmenu", {});
assert.deepEqual(items(), ["MIDI learn", "Forget all mappings on Page 1 (1)"]);
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
assert.deepEqual(items(), ["Cancel MIDI learn", "Forget all mappings on Page 1 (1)"]);
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
stored.delete("sad-pet-midi-pages");
assert.deepEqual(plain(editor().SadMidiLearn.mappings()), { a: [0, 7] }, "invalid saved mappings loaded");
dial(rebuilt).dispatch("contextmenu", {});
assert.deepEqual(items(), ["MIDI learn", "Forget CC 74 · ch 1"]);
choose("Forget CC 74 · ch 1");
assert.deepEqual(plain(learn.mappings()), {});
assert.equal(dial(rebuilt).querySelector(".knob-midi"), null);
learn.input(0, 74, 127);
assert.equal(values.pitch, 1, "a forgotten controller still turned the knob");

// Existing mappings migrate into Page 1, while another page reuses the same
// physical CC for a different knob. Switching removes old badges and Learn.
const paging = context.SadMidiLearn;
paging.setup({ notify: text => notices.push(text) });
assert.equal(paging.page(), 0);
assert.equal(paging.pages().length, 16);
assert.deepEqual(plain(paging.mappings()), { drum1_9: [0, 74] });
paging.selectPage(1);
assert.deepEqual(plain(paging.mappings()), {});
const pageValues = Array(32).fill(0);
const pageKnobs = pageValues.map((_, i) => knob(`page_control_${i}`, `Page control ${i + 1}`, {
  get: () => pageValues[i], set: v => { pageValues[i] = v; }, reset: () => 0,
}));
paging.learn("page_control_0");
paging.input(0, 74, 64);
assert.equal(pageValues[0], 64 / 127);
assert.equal(dial(pageKnobs[0]).dataset.midi, "CC 74 · ch 1");
paging.learn("page_control_1");
paging.selectPage(0);
assert.equal(paging.learning(), null, "changing pages retained an armed Learn");
assert.equal(notices.at(-1), "MIDI learn cancelled", "the previous page's Learn prompt stayed active");
assert.equal(dial(pageKnobs[0]).querySelector(".knob-midi"), null);
assert.deepEqual(plain(paging.mappings()), { drum1_9: [0, 74] });
assert.equal(paging.input(0, 74, 0), true);
assert.equal(pageValues[0], 64 / 127, "the inactive page responded to its CC");
paging.forget("drum1_9");
paging.selectPage(1);
paging.forget("page_control_0");

// All 32 physical knobs/faders can have a different destination on every
// one of the 16 pages: 512 saved assignments, with no cross-page collisions.
const hardware = [13, 29, 49, 77].flatMap(start => Array.from({ length: 8 }, (_, i) => start + i));
const expectedPages = [];
for (let p = 0; p < 16; p++) {
  paging.selectPage(p);
  for (let i = 0; i < hardware.length; i++) {
    paging.learn(`page_control_${(i + p) % 32}`);
    paging.input(8, hardware[i], 64);
  }
  assert.equal(paging.pages()[p].count, 32);
  expectedPages.push(plain(paging.mappings()));
}
paging.selectPage(0);
paging.input(8, 13, 127);
assert.equal(pageValues[0], 1);
paging.selectPage(1);
paging.input(8, 13, 0);
assert.equal(pageValues[1], 0);
assert.equal(pageValues[0], 1);
assert.equal(dial(pageKnobs[0]).querySelector(".knob-midi").textContent, "84");
const restoredPages = editor().SadMidiLearn;
assert.equal(restoredPages.page(), 1, "the active page was not restored");
for (let p = 0; p < 16; p++) {
  restoredPages.selectPage(p);
  assert.deepEqual(plain(restoredPages.mappings()), expectedPages[p]);
}
assert.deepEqual(JSON.parse(stored.get("sad-pet-midi-map")), expectedPages[0], "legacy storage stopped representing Page 1");
paging.selectPage(15);
dial(pageKnobs[0]).dispatch("contextmenu", {});
choose("Forget all mappings on Page 16 (32)");
assert.deepEqual(plain(paging.mappings()), {});
assert.equal(dial(pageKnobs[0]).querySelector(".knob-midi"), null);
paging.selectPage(0);
assert.deepEqual(plain(paging.mappings()), expectedPages[0], "clearing one page erased another");
for (const invalid of [-1, 16, .5, "1", null]) assert.throws(() => paging.selectPage(invalid), /page from 1 to 16/);
assert.equal(paging.page(), 0);
const goodPages = stored.get("sad-pet-midi-pages"), goodLegacy = stored.get("sad-pet-midi-map");
stored.set("sad-pet-midi-pages", "broken JSON");
stored.set("sad-pet-midi-map", JSON.stringify({ migrated: [0, 7] }));
assert.deepEqual(plain(editor().SadMidiLearn.mappings()), { migrated: [0, 7] }, "corrupt pages discarded the legacy map");
stored.set("sad-pet-midi-pages", goodPages);
stored.set("sad-pet-midi-map", goodLegacy);
console.log("MIDI learn UI tests passed");
console.log("MIDI pages: 16 × 32 assignments, independent routing, migration, persistence, badges, cancellation and page-local clearing passed.");

// Real grain Pitch: 0.1 st per relative tick, regardless of range, even when
// starting from an old 7-bit value; high-resolution faders land on that grid.
const fine = editor(), fineBank = new Element("div");
document.body.append(fineBank);
fine.preset = { params: { pitch: 0.5, pitch_range: 48 } };
fine.presets = [{ params: { pitch: 0.5 } }];
fine.setParam = (key, value) => { fine.preset.params[key] = value; };
fine.createKnob(["pitch", "Pitch", v => `${((v * 2 - 1) * fine.preset.params.pitch_range).toFixed(1)} st`], fineBank);
const fineLearn = fine.SadMidiLearn;
fineLearn.learn("pitch");
fineLearn.input(0, 13, 65, "relative");
const semitones = () => (fine.preset.params.pitch * 2 - 1) * fine.preset.params.pitch_range;
assert.ok(Math.abs(semitones() - 0.1) < 1e-10);
for (let i = 0; i < 100; i++) fineLearn.input(0, 13, 65, "relative");
assert.ok(Math.abs(semitones() - 10.1) < 1e-10, "relative Pitch accumulated rounding drift");
fineLearn.input(0, 13, 63, "relative");
assert.ok(Math.abs(semitones() - 10) < 1e-10);
fineLearn.input(0, 13, 68, "relative");
assert.ok(Math.abs(semitones() - 10.4) < 1e-10, "relative acceleration lost its tick count");
fine.preset.params.pitch_range = 12;
fine.preset.params.pitch = 0.5;
fineLearn.input(0, 13, 65, "relative");
assert.ok(Math.abs(semitones() - 0.1) < 1e-10, "Pitch precision depended on its range");
fineLearn.learn("pitch");
fineLearn.input(0, 5, 8260, "14bit");
assert.ok(Math.abs(semitones() - 0.1) < 1e-10);
assert.deepEqual(plain(fineLearn.mappings()).pitch, [0, 5, "14bit"]);
assert.deepEqual(plain(editor().SadMidiLearn.mappings()).pitch, [0, 5, "14bit"], "14-bit format was lost on reload");
fineLearn.learn("pitch");
fineLearn.selectPage(3, { preserveLearning: true });
assert.equal(fineLearn.learning(), "pitch", "the first native mode message cancelled Learn");
fineLearn.input(3, 13, 65, "relative");
assert.deepEqual(plain(fineLearn.mappings()).pitch, [3, 13, "relative"]);
fine.preset.params.pitch = 1;
fineLearn.input(3, 13, 65, "relative");
assert.equal(fine.preset.params.pitch, 1, "relative input exceeded the parameter range");
const beforeInvalid = fine.preset.params.pitch;
for (const invalid of [-1, 16384, 1.5]) fineLearn.input(3, 5, invalid, "14bit");
assert.equal(fine.preset.params.pitch, beforeInvalid);

// Switching to the supplied hardware modes preserves existing assignments,
// changes only the factory XL3 controls and keeps a recoverable local copy.
fineLearn.selectPage(0);
fineLearn.learn("old_pitch"); fineLearn.input(15, 14, 64);
fineLearn.learn("old_button"); fineLearn.input(15, 37, 127);
fineLearn.learn("keyboard"); fineLearn.input(2, 74, 127);
fineLearn.configureXL3();
assert.deepEqual(plain(fineLearn.mappings()).old_pitch, [0, 14, "relative"]);
assert.deepEqual(plain(fineLearn.mappings()).old_button, [0, 80]);
assert.deepEqual(plain(fineLearn.mappings()).keyboard, [2, 74]);
assert.ok(stored.get("sad-pet-midi-pages-before-xl3-native"));
const migrated = plain(fineLearn.mappings());
fineLearn.configureXL3();
assert.deepEqual(plain(fineLearn.mappings()), migrated, "native migration was not idempotent");
console.log("XL3 precision: relative Pitch in 0.1 st, range changes, 14-bit faders, native Learn, bounds, persistence and existing-map migration passed.");

// A Control surface may show hundreds of linked knobs. Unchanged telemetry
// must not rewrite text/styles on each poll; real base/feedback edits still paint.
const painted = new Element("div");
document.body.append(painted);
let paintValue = .5, feedback = { base: .5, current: .6, min: .4, max: .7, modulated: true };
const paint = context.createKnob(["control:0:filter", "Cutoff", String], painted, {
  register: false, get: () => paintValue, set: v => { paintValue = v; }, reset: () => .5,
  feedback: () => feedback,
});
let writes = 0;
for (const node of painted.all()) {
  let text = node.textContent;
  Object.defineProperty(node, "textContent", { get: () => text, set: value => { writes++; text = value; } });
  node.style = new Proxy(node.style, { set(target, key, value) { writes++; target[key] = value; return true; } });
}
paint.paint();
writes = 0;
for (let i = 0; i < 1000; i++) paint.paint();
assert.equal(writes, 0, "Unchanged Control knobs repainted at every poll");
paintValue = .8;
paint.paint();
assert.equal(dial(painted.firstElementChild).getAttribute("aria-valuenow"), "80");
feedback = { base: .8, current: .9, min: .7, max: 1, modulated: true };
paint.paint();
assert.equal(dial(painted.firstElementChild).dataset.modulationCurrent, "0.9");
console.log("Control knob feedback: 1000 unchanged polls make no text/style writes; edits still repaint.");
const sharedDial = dial(painted.firstElementChild);
context.SadMidiLearn.learn("control:0:filter");
context.SadMidiLearn.input(0, 11, 127);
assert.equal(paintValue, 1);
context.SadMidiLearn.detach(sharedDial);
context.SadMidiLearn.input(0, 11, 0);
assert.equal(paintValue, 1, "A disposed Control view retained its source callback");
assert.deepEqual(plain(context.SadMidiLearn.mappings())["control:0:filter"], [0, 11],
  "Disposing a view must preserve its saved MIDI mapping");

// Browser mappings pin a parameter to its source, rather than whichever
// instrument currently owns the reused knob DOM. Both pitches can coexist.
stored.clear();
const scoped = editor(), scopedBank = new Element("div"), routed = [];
document.body.append(scopedBank);
scoped.state.timbres = { active: 0, timbres: [{ id: 0, kind: "rack" }, { id: 1, kind: "rack" }] };
scoped.preset = { params: { pitch: .5, pitch_range: 36 } };
scoped.sequencerParameter = key => key;
scoped.SadMidiTargets = { enabled: true, describe: d => ({ ...d, id: `control:${d.timbre}:${d.parameter}` }),
  command: (target, format, value) => routed.push({ target: plain(target), format, value }),
  value: () => ({ unit: .5, text: "0.0 st" }), refresh() {},
};
scoped.createKnob(["pitch", "Pitch", String], scopedBank);
const scopedLearn = scoped.SadMidiLearn, scopedDial = dial(scopedBank.firstElementChild);
scopedLearn.learn("pitch"); scopedLearn.input(0, 13, 65, "relative");
assert.deepEqual(plain(scopedLearn.mappings()), { "control:0:pitch": [0, 13, "relative"] });
scoped.state.timbres.active = 1;
scopedLearn.refresh();
assert.equal(scopedBank.firstElementChild.classList.contains("midi-mapped"), false, "another instrument inherited the source's badge");
scopedLearn.input(0, 13, 66, "relative");
assert.equal(routed.at(-1).target.timbre, 0, "switching instruments redirected the mapping");
scopedLearn.learn("pitch"); scopedLearn.input(0, 14, 65, "relative");
assert.equal(routed.at(-1).target.timbre, 1);
assert.equal(Object.keys(scopedLearn.mappings()).length, 2, "two instruments' Pitch mappings collided");
scopedBank.remove();
// Old effect controls close over modules absent in the new editor. Painting
// current badges must not evaluate those detached bindings.
const retired = new Element("div");
document.body.append(retired);
let disposed = false;
scopedLearn.attach(retired, "retired-effect", "Retired", () => {}, null,
  { describe() { if (disposed) throw new Error("Disposed editor binding was evaluated"); return null; } });
disposed = true;
scopedLearn.refresh();
retired.remove();
scopedLearn.refresh();
scopedLearn.input(0, 13, 63, "relative");
assert.equal(routed.at(-1).target.timbre, 0, "a removed editor disabled its MIDI target");
const scopedRestored = editor();
scopedRestored.SadMidiTargets = scoped.SadMidiTargets;
scopedRestored.SadMidiLearn.input(0, 14, 65, "relative");
assert.equal(routed.at(-1).target.timbre, 1, "source identity was lost on reload");
assert.equal(JSON.parse(stored.get("sad-pet-midi-pages")).version, 2);
assert.equal(Object.keys(scopedRestored.SadMidiLearn.targets()).length, 2);
assert.equal(scopedLearn.feedback()[0].label, "Pitch");

// Existing assignments migrate once, retain their CC/channel, and keep a copy.
stored.clear(); stored.set("sad-pet-midi-map", JSON.stringify({ pitch: [0, 16] }));
const legacyScoped = editor(), legacyBank = new Element("div");
document.body.append(legacyBank);
legacyScoped.SadMidiTargets = scoped.SadMidiTargets;
legacyScoped.sequencerParameter = key => key;
legacyScoped.state.timbres = { active: 0, timbres: [{ id: 0, kind: "rack" }] };
legacyScoped.preset = { params: { pitch: .5, pitch_range: 36 } };
legacyScoped.createKnob(["pitch", "Pitch", String], legacyBank);
legacyScoped.SadMidiLearn.setup({});
assert.deepEqual(plain(legacyScoped.SadMidiLearn.mappings()), { "control:0:pitch": [0, 16] });
assert.ok(stored.get("sad-pet-midi-pages-before-targets") || stored.get("sad-pet-midi-map"));
legacyScoped.SadMidiLearn.refresh();
assert.equal(Object.keys(legacyScoped.SadMidiLearn.mappings()).length, 1);
console.log("Pinned MIDI UI passed: independent instrument targets, offscreen control, badges, reload and legacy migration.");
