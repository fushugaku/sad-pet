// The Bus and Master mixer: a strip per timbre that reaches the chain, its
// level there and the four places it enters the chain.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const panels = await readFile(new URL("../rust/web/panels.js", import.meta.url), "utf8");

class Element {
  constructor(tag = "div") {
    this.tagName = tag.toUpperCase();
    this.children = [];
    this.attributes = new Map();
    this.dataset = {};
    this.style = {};
    this.classes = new Set();
    this.classList = {
      add: (name) => this.classes.add(name),
      contains: (name) => this.classes.has(name),
      toggle: (name, on) => (on ? this.classes.add(name) : this.classes.delete(name)),
    };
    this.textContent = "";
    this.hidden = false;
  }
  set className(names) {
    this.classes = new Set(names.split(" ").filter(Boolean));
  }
  get className() {
    return [...this.classes].join(" ");
  }
  set innerHTML(html) {
    this.children = [...html.matchAll(/class="([\w-]+)"/g)].map(([, name]) => {
      const node = new Element();
      node.className = name;
      return node;
    });
  }
  append(...nodes) {
    this.children.push(...nodes);
  }
  replaceChildren(...nodes) {
    this.children = nodes;
  }
  *walk() {
    for (const child of this.children) {
      yield child;
      yield* child.walk();
    }
  }
  querySelectorAll(selector) {
    return [...this.walk()].filter((node) => node.classes.has(selector.slice(1)));
  }
  querySelector(selector) {
    return this.querySelectorAll(selector)[0] ?? null;
  }
  setAttribute(key, value) {
    this.attributes.set(key, String(value));
  }
  getAttribute(key) {
    return this.attributes.get(key);
  }
}
const window = {};
const context = vm.createContext({
  window,
  document: { activeElement: null, createElement: (tag) => new Element(tag) },
});
vm.runInContext(panels, context);

const changes = [];
// Plain objects of this realm, for strict comparisons with the vm's arrays.
const json = (value) => JSON.parse(JSON.stringify(value));
const knobs = new Map();
let repaints = 0;
const root = new Element();
const mixer = new window.SadMixer(root, {
  change: (path, data, scalar) => changes.push(json({ path, data, scalar })),
  // createKnob: a box with a dial, whose update repaints from the binding.
  knob: ([key, label, format, min, max, aria], container, binding) => {
    const box = new Element();
    const dial = new Element();
    dial.className = "dial";
    box.append(dial);
    container.append(box);
    const knob = { label, aria, binding, dial, shown: null };
    knobs.set(key, knob);
    return () => {
      repaints++;
      knob.shown = format(binding.get());
    };
  },
});
const plain = () => Array.from({ length: 9 }, () => [1, 0, 0, 0]);
const timbre = (id, kind, extra = {}) => ({
  id,
  kind,
  name: `${kind} ${id}`,
  bus: 0,
  gain: 1,
  muted: false,
  solo: false,
  sends: Array(8).fill(0),
  ...extra,
});
const scene = (active) => ({
  active,
  timbres: [
    timbre(0, "granular", { sends: [0, 0.5, 0, 0, 0, 0, 0, 0] }),
    timbre(1, "synth", { gain: 0.5 }),
    timbre(2, "bus", { bus: 2, name: "Reverb" }),
    timbre(3, "pad_fx", { owner: 4 }),
    timbre(4, "sampler"),
    timbre(9, "master"),
  ],
});
const strips = () => root.querySelector(".mixer-strips").children;
const names = () => strips().map((s) => s.children[0].textContent);

// Master: every timbre and every bus, with Mute and Solo on the timbres.
let context0 = scene(9);
mixer.sync(context0);
assert.equal(root.hidden, false);
assert.deepEqual(names(), ["granular 0", "synth 1", "Reverb", "sampler 4"], "Master shows timbres and buses, not Pad FX");
const level = (strip) => strip.querySelector(".mixer-level").textContent;
assert.equal(level(strips()[1]), "-6.0 dB", "Master strips show the level into Master");
assert.equal(strips()[2].querySelector(".mixer-buttons").children.length, 1, "a bus strip has Mute only");
assert.equal(knobs.get("mixer_0_1_0").shown, "100%", "timbres enter at the input by default");
assert.equal(knobs.get("mixer_0_1_3").shown, "0%");

// A knob sends the whole entry table, changed only at this destination,
// and shows its value before the host answers.
knobs.get("mixer_0_1_3").binding.set(0.75);
let last = changes.at(-1);
assert.equal(last.path, "/api/timbres/update");
assert.equal(last.scalar, true);
const expected = plain();
expected[0][3] = 0.75;
assert.deepEqual(last.data, { id: 1, entries: expected });
assert.equal(knobs.get("mixer_0_1_3").binding.get(), 0.75);
assert.equal(knobs.get("mixer_0_1_3").binding.reset(), 0);
assert.equal(knobs.get("mixer_0_1_0").binding.reset(), 1);
mixer.sync(context0);
assert.ok(strips()[1].classes.has("entered"), "a strip that enters later is marked");

// The fader is the level into Master; M and S act on the timbre.
const fader = (strip) => strip.querySelector(".mixer-fader").children[0];
fader(strips()[0]).value = "0.25";
fader(strips()[0]).oninput();
assert.deepEqual(changes.at(-1).data, { id: 0, gain: 0.25 });
strips()[0].querySelector(".mixer-buttons").children[0].onclick();
assert.deepEqual(changes.at(-1).data, { id: 0, muted: true });

// Polls repeat the same values without repainting the knobs.
repaints = 0;
mixer.sync(context0);
assert.equal(repaints, 4, "the edited strip repaints its four knobs once");
mixer.sync(context0);
assert.equal(repaints, 4, "an unchanged poll repaints nothing");

// Bus 2: timbres only; the fader is the send and a silent send is dimmed.
const context2 = scene(2);
mixer.sync(context2);
assert.deepEqual(names(), ["granular 0", "synth 1", "sampler 4"], "a bus shows the timbres, not buses");
assert.equal(level(strips()[0]), "-6.0 dB", "the send to Bus 2");
assert.equal(level(strips()[1]), "−∞ dB");
assert.ok(strips()[1].classes.has("idle"), "a timbre that does not send is dimmed");
assert.ok(!strips()[0].classes.has("idle"));
assert.equal(strips()[0].querySelector(".mixer-buttons"), null, "a bus mixer leaves Mute and Solo to Master");
fader(strips()[1]).value = "1";
fader(strips()[1]).oninput();
assert.deepEqual(changes.at(-1).data, { id: 1, sends: [0, 1, 0, 0, 0, 0, 0, 0] });
knobs.get("mixer_2_0_2").binding.set(1);
const entries = changes.at(-1).data.entries;
assert.deepEqual(entries[2], [1, 0, 1, 0], "Bus 2 entries change at index 2");
assert.deepEqual(entries[0], [1, 0, 0, 0]);

// Other timbres show no mixer; the name opens the timbre.
strips()[0].children[0].onclick();
assert.deepEqual(changes.at(-1), { path: "/api/timbres/select", data: { id: 0 } });
mixer.sync(scene(0));
assert.equal(root.hidden, true, "a timbre's page has no mixer");
console.log("Mixer: Master and Bus strips, levels, entries, Mute/Solo and repaint-free polls.");
