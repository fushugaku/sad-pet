// The MIDI clock row: hidden without /api/clock, this computer's choices
// restored, and the tempo and Play handed to the master while it is followed.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

class Element {
  constructor(tag, id) {
    this.tagName = tag.toUpperCase();
    this.id = id;
    this.children = [];
    this.hidden = false;
    this.disabled = false;
    this.checked = false;
    this.value = "";
    this.title = "";
    this.textContent = "";
    this.names = new Set();
    this.classList = {
      toggle: (name, force) => (force ? this.names.add(name) : this.names.delete(name)),
      contains: (name) => this.names.has(name),
    };
    this.parentElement = { hidden: false };
  }
  replaceChildren(...children) {
    this.children = children;
  }
  get selectedOptions() {
    return this.children.filter((o) => o.value === this.value);
  }
}
function page() {
  const ids = ["clock-controls", "clock-led", "clock-follow", "clock-lead", "clock-out", "clock-status", "bpm", "play"];
  const elements = Object.fromEntries(ids.map((id) => [id, new Element("div", id)]));
  elements["clock-controls"].hidden = true;
  elements.bpm.value = "120";
  const document = {
    activeElement: null,
    getElementById: (id) => elements[id],
    createElement: (tag) => new Element(tag),
  };
  const stored = new Map();
  const localStorage = {
    getItem: (key) => stored.get(key) ?? null,
    setItem: (key, value) => stored.set(key, String(value)),
  };
  return { elements, document, localStorage, stored };
}
const source = await readFile(new URL("../rust/web/clock.js", import.meta.url), "utf8");
function load(dom) {
  const context = vm.createContext({ document: dom.document, localStorage: dom.localStorage });
  context.window = context;
  vm.runInContext(source, context);
  return context.SadMidiClock;
}
const summary = (patch = {}) => ({
  supported: { follow: true, send: true },
  follow: false,
  lead_ms: 0,
  out: null,
  outputs: [{ id: "o1", name: "IAC Bus 1" }, { id: "o2", name: "Network Session 1" }],
  ...patch,
});

// A backend without MIDI clock (a DAW, the desktop server) keeps it hidden.
{
  const dom = page();
  const clock = load(dom);
  await clock.setup({
    request: async () => {
      throw new Error("Unsupported plugin request: /api/clock");
    },
    notify: () => assert.fail("no notice"),
    hasInput: () => true,
    released: () => {},
  });
  assert.equal(dom.elements["clock-controls"].hidden, true);
  clock.sync({ clock: { follow: true, present: true, tempo: 99 } });
  assert.equal(dom.elements.bpm.disabled, false);
}

// The browser: Follow, the offset and the output chosen last time return.
{
  const dom = page();
  dom.stored.set("sad-pet-clock", JSON.stringify({ follow: true, lead_ms: 12, out: "Network Session 1" }));
  const calls = [];
  let current = summary();
  const request = async (path, data) => {
    assert.equal(path, "/api/clock");
    calls.push(data ?? null);
    if (data) {
      current = {
        ...current,
        ...("follow" in data ? { follow: data.follow } : {}),
        ...("lead_ms" in data ? { lead_ms: data.lead_ms } : {}),
        ...("out" in data ? { out: data.out } : {}),
      };
    }
    return current;
  };
  let released = 0;
  const clock = load(dom);
  await clock.setup({ request, notify: (text) => assert.fail(text), hasInput: () => true, released: () => released++ });
  assert.deepEqual(JSON.parse(JSON.stringify(calls)), [null, { follow: true, lead_ms: 12 }, { out: "o2" }]);
  const { elements } = dom;
  assert.equal(elements["clock-controls"].hidden, false);
  assert.equal(elements["clock-follow"].checked, true);
  assert.equal(elements["clock-lead"].value, 12);
  assert.deepEqual(elements["clock-out"].children.map((o) => o.textContent), ["No clock out", "IAC Bus 1", "Network Session 1"]);
  assert.equal(elements["clock-out"].value, "o2");
  // Choosing another output remembers it by name.
  elements["clock-out"].value = "o1";
  elements["clock-out"].onchange({ target: elements["clock-out"] });
  await new Promise((r) => setImmediate(r));
  assert.equal(JSON.parse(dom.stored.get("sad-pet-clock")).out, "IAC Bus 1");
  assert.deepEqual(JSON.parse(JSON.stringify(calls.at(-1))), { out: "o1" });
  // Following: the master's tempo shows and owns the tempo field and Play.
  clock.sync({ clock: { follow: true, present: true, running: true, tempo: 127.96, sending: "IAC Bus 1", error: null } });
  assert.equal(elements["clock-status"].textContent, "Playing · 128.0 BPM · out to IAC Bus 1");
  assert.equal(elements.bpm.value, "128.0");
  assert.equal(elements.bpm.disabled, true);
  assert.equal(elements.play.disabled, true);
  assert.equal(elements["clock-led"].classList.contains("off"), false);
  // The master goes away: both are the editor's again.
  clock.sync({ clock: { follow: true, present: false, running: false, tempo: null, sending: null, error: null } });
  assert.equal(elements["clock-status"].textContent, "Waiting for clock");
  assert.equal(elements.bpm.disabled, false);
  assert.equal(elements.play.disabled, false);
  assert.equal(released, 1);
  assert.equal(elements["clock-led"].classList.contains("off"), true);
}

// Follow without a MIDI input says what is missing.
{
  const dom = page();
  const clock = load(dom);
  await clock.setup({ request: async () => summary({ follow: true }), notify: () => {}, hasInput: () => false, released: () => {} });
  clock.sync({ clock: { follow: true, present: false } });
  assert.equal(dom.elements["clock-status"].textContent, "Choose a MIDI input to follow");
}
console.log("MIDI clock row: hidden, restored and following as expected.");
