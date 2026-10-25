import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

// A knob's menu routes LFOs, envelopes, sequencer tracks, macros and matrix
// sources to it. The editor's helpers are stubbed: a destination is its
// control key, as for the main controls.
const source = await readFile(new URL("../rust/web/modulation.js", import.meta.url), "utf8");
const context = vm.createContext({ document: { querySelectorAll: () => [], getElementById: () => null }, setTimeout, structuredClone });
context.window = context;
vm.runInContext(source, context);
const modulation = context.SadModulation;

const lfo = () => ({ destination: "none", depth: 0, destination2: "none", depth2: 0, destination3: "none", depth3: 0, rate_hz: 0.2 });
const env = () => ({ destination: "none", depth: 0, extra_destination: "none", extra_depth: 0, destination3: "none", depth3: 0 });
const fresh = () => ({
  lfos: Array.from({ length: 8 }, lfo),
  envs: Array.from({ length: 8 }, env),
  lfo_count: 3,
  env_count: 3,
  matrix: [],
  sequencer: { enabled: false, tracks: [{ id: 1, name: "Notes 1", target: { kind: "notes" }, destinations: [null, null, null] }] },
});
let preset = fresh();
const notices = [],
  added = [];
let changes = 0;
modulation.setup({
  read: () => preset,
  defaults: () => ({ lfos: Array.from({ length: 8 }, () => ({ ...lfo(), rate_hz: 1 })), envs: Array.from({ length: 8 }, env) }),
  sources: [["lfo1", "LFO 1"], ["env1", "ENV 1"], ["velocity", "Velocity"], ["mod_wheel", "Mod wheel"]],
  // "macro1" and "sampler_tune" turn nothing a route reaches.
  destination: (key) => (["macro1", "sampler_tune", "drum_step"].includes(key) ? null : key),
  sequencer: (key) => (["macro1", "sampler_tune", "pad_only"].includes(key) ? null : key),
  key: (d) => (typeof d === "string" ? d : JSON.stringify(d)),
  value: (d) => (typeof d === "string" ? d : JSON.stringify(d)),
  parse: (v) => v,
  notify: (text) => notices.push(text),
  changed: () => changes++,
  addTrack(name, target) {
    const track = { id: 9, name, target, destinations: [null, null, null] };
    preset.sequencer.tracks.push(track);
    added.push(track);
    return track;
  },
});
const plain = (value) => JSON.parse(JSON.stringify(value));
const menu = (key = "filter", label = "Filter 1 cutoff") => modulation.menu(key, label);
const section = (m, label) => m.groups.find((g) => g.label === label);
const rows = (m, label) => section(m, label)?.choices.map(([, text]) => text);
// A row by its name, with or without its note ("LFO 2 · via matrix").
const choose = (m, label, text) => {
  const [value] = section(m, label).choices.find(([, t]) => t === text || t.startsWith(`${text} · `));
  m.choose(value);
  return value;
};

// Controls no route reaches keep only MIDI learn.
assert.equal(modulation.menu("macro1", "Macro 1"), null);
assert.equal(modulation.menu("sampler_tune", "Tune"), null);
// A lane-only parameter offers the sequencer alone.
assert.deepEqual(plain(menu("drum_step").groups.map((g) => g.label)), ["Sequencer"]);

let m = menu();
assert.deepEqual(plain(m.groups.map((g) => g.label)), ["LFO", "ENV", "Sequencer", "Macro", "Mod matrix"]);
assert.deepEqual(plain(rows(m, "LFO")), ["LFO 1", "LFO 2", "LFO 3", "New LFO 4"]);
assert.deepEqual(plain(rows(m, "ENV")), ["ENV 1 · Amp", "ENV 2", "ENV 3", "New ENV 4"]);
assert.deepEqual(plain(rows(m, "Mod matrix")), ["Velocity", "Mod wheel"], "LFOs and ENVs are listed by their own menus");
assert.deepEqual(plain(rows(m, "Macro")), ["New macro"]);

// An LFO takes a free route of its own at half depth, once.
choose(m, "LFO", "LFO 2");
assert.deepEqual([preset.lfos[1].destination, preset.lfos[1].depth], ["filter", 0.5]);
assert.equal(notices.at(-1), "LFO 2 → Filter 1 cutoff");
assert.equal(changes, 1);
m = menu();
const lfo2 = choose(m, "LFO", "LFO 2");
assert.ok(m.checked(lfo2), "the routed LFO is not ticked");
assert.equal(notices.at(-1), "LFO 2 already modulates Filter 1 cutoff");
assert.equal(changes, 1);
// Routes 2 and 3 fill next; with all three taken the matrix carries it.
for (const key of ["pitch", "cloud", "grains"]) choose(menu(key, key), "LFO", "LFO 2");
assert.deepEqual([preset.lfos[1].destination2, preset.lfos[1].destination3], ["pitch", "cloud"]);
assert.deepEqual(plain(preset.matrix), [{ source: "lfo2", destination: "grains", depth: 0.25, enabled: true }]);
assert.match(notices.at(-1), /in the mod matrix/);
assert.equal(rows(menu("detune"), "LFO")[1], "LFO 2 · via matrix");
assert.ok(menu("grains").checked(section(menu("grains"), "LFO").choices[1][0]), "a matrix route from the LFO is not ticked");

// A new LFO or ENV starts fresh, counts up and appears in the menus.
preset.lfos[3] = { ...lfo(), destination: "pan", depth: 0.9, rate_hz: 30 };
choose(menu(), "LFO", "New LFO 4");
assert.equal(preset.lfo_count, 4);
assert.deepEqual([preset.lfos[3].destination, preset.lfos[3].destination2, preset.lfos[3].rate_hz], ["filter", "none", 1]);
choose(menu(), "ENV", "New ENV 4");
assert.equal(preset.env_count, 4);
assert.deepEqual([preset.envs[3].destination, preset.envs[3].depth], ["filter", 0.5]);
assert.deepEqual(plain(rows(menu(), "ENV")), ["ENV 1 · Amp", "ENV 2", "ENV 3", "ENV 4", "New ENV 5"]);
preset.lfo_count = 8;
assert.ok(!rows(menu(), "LFO").some((t) => t.startsWith("New")), "a ninth LFO was offered");
preset.lfo_count = 4;

// Sequencer tracks take the control in a free destination; New track plays it.
m = menu();
assert.deepEqual(plain(rows(m, "Sequencer")), ["Notes 1", "New track"]);
choose(m, "Sequencer", "Notes 1");
assert.deepEqual(plain(preset.sequencer.tracks[0].destinations), ["filter", null, null]);
choose(menu(), "Sequencer", "Notes 1");
assert.equal(notices.at(-1), "Notes 1 already plays Filter 1 cutoff");
preset.sequencer.tracks[0].destinations = ["a", "b", "c"];
choose(menu("pitch", "Pitch"), "Sequencer", "Notes 1");
assert.equal(notices.at(-1), "Notes 1 has no free destination");
choose(menu("pitch", "Pitch"), "Sequencer", "New track");
assert.deepEqual(plain(added[0].target), { kind: "parameter", parameter: "pitch" });
assert.equal(added[0].name, "Pitch");
assert.match(notices.at(-1), /^Track 9 → Pitch/);

// Macros: a new one turns the control; others add it as a target.
choose(menu(), "Macro", "New macro");
assert.deepEqual(plain(preset.macros), [{ name: "Macro 1", value: 0, targets: [{ destination: "filter", depth: 0.5 }] }]);
choose(menu("cloud", "Cloud"), "Macro", "Macro 1");
assert.equal(preset.macros[0].targets.length, 2);
choose(menu("cloud", "Cloud"), "Macro", "Macro 1");
assert.equal(notices.at(-1), "Macro 1 already turns Cloud");
preset.macros[0].name = "Bloom";
for (let i = 0; i < 6; i++) choose(menu(`p${i}`, `P${i}`), "Macro", "Bloom");
choose(menu("q", "Q"), "Macro", "Bloom");
assert.equal(notices.at(-1), "Bloom already turns 8 controls");
assert.equal(preset.macros[0].targets.length, 8);
for (let i = 1; i < 8; i++) preset.macros.push({ name: "", value: 0, targets: [] });
assert.deepEqual(plain(rows(menu(), "Macro")).slice(-2), ["Macro 7", "Macro 8"], "a full set offered a new macro");

// Other sources add matrix routes until the matrix is full.
choose(menu(), "Mod matrix", "Velocity");
assert.deepEqual(plain(preset.matrix.at(-1)), { source: "velocity", destination: "filter", depth: 0.25, enabled: true });
choose(menu(), "Mod matrix", "Velocity");
assert.equal(notices.at(-1), "Velocity already modulates Filter 1 cutoff");
while (preset.matrix.length < 8) preset.matrix.push({ source: "gate", destination: "none", depth: 0, enabled: true });
choose(menu(), "Mod matrix", "Mod wheel");
assert.equal(notices.at(-1), "The mod matrix is full");

// Remove lists every route to the control and takes each away.
preset.sequencer.tracks[0].destinations = ["filter", null, null];
preset.sequencer.tracks.push({ id: 4, name: "Cutoff lane", target: { kind: "parameter", parameter: "filter" }, destinations: [] });
m = menu();
assert.deepEqual(plain(rows(m, "Remove modulation")), [
  "LFO 2 · route 1", "LFO 4 · route 1", "ENV 4 · route 1", "Matrix 2 · Velocity",
  "Bloom", "Notes 1 · destination 1", "Delete track Cutoff lane",
]);
for (const text of ["LFO 2 · route 1", "ENV 4 · route 1", "Matrix 2 · Velocity", "Bloom", "Notes 1 · destination 1", "Delete track Cutoff lane"])
  choose(menu(), "Remove modulation", text);
assert.deepEqual([preset.lfos[1].destination, preset.lfos[1].depth, preset.lfos[1].destination2], ["none", 0, "pitch"]);
assert.equal(preset.envs[3].destination, "none");
assert.ok(!preset.matrix.some((r) => r.source === "velocity"));
assert.ok(!preset.macros[0].targets.some((t) => t.destination === "filter"));
assert.deepEqual(plain(preset.sequencer.tracks[0].destinations), [null, null, null]);
assert.ok(!preset.sequencer.tracks.some((t) => t.id === 4));
assert.equal(notices.at(-1), "Deleted track Cutoff lane");
assert.deepEqual(plain(rows(menu(), "Remove modulation")), ["LFO 4 · route 1"]);

// Hidden LFOs and ENVs neither route nor show.
preset.lfos[5].destination = "filter";
assert.deepEqual(plain(rows(menu(), "Remove modulation")), ["LFO 4 · route 1"]);

// A menu outlives no preset: a choice after the editor replaced it does nothing.
m = menu();
const before = changes;
preset = fresh();
choose(m, "LFO", "LFO 1");
assert.equal(preset.lfos[0].destination, "none");
assert.equal(changes, before);
assert.equal(notices.at(-1), "The preset changed. Open the menu again.");
console.log("Modulation menu: LFO/ENV routes, matrix fallback, new modulators, sequencer, macros, matrix sources and removal passed.");
