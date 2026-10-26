// Bus and Master routing in the shipping WASM, without an audio device.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(
  await readFile(resolve(folder, "sad_pet.wasm")),
);
const utility = new Core(module, 8000),
  core = new Core(module, 8000);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
core.call("sad_load", utility.call("sad_pack"));
const render = (blocks) => {
  let peak = 0;
  for (let i = 0; i < blocks; i++)
    for (const x of core.process(128)) {
      assert.ok(Number.isFinite(x));
      peak = Math.max(peak, Math.abs(x));
    }
  return peak;
};

let context = core.request("/api/timbres");
const master = context.timbres.find((t) => t.kind === "master");
assert.ok(master, "Every scene has a Master");
assert.throws(() => core.request("/api/timbres/remove", { id: master.id }), /Master/);
assert.throws(() => core.request("/api/timbres/add", { kind: "master" }), /Master/);
const masterPreset = (() => {
  core.request("/api/timbres/select", { id: master.id });
  const preset = core.request("/api/preset");
  core.request("/api/timbres/select", { id: 0 });
  return preset;
})();
assert.deepEqual(
  [masterPreset.bus.master, masterPreset.bus.granular, masterPreset.bus.filters],
  [true, false, false],
  "Master starts with everything off",
);

core.request("/api/transport", { playing: true, midi_gate: false });
assert.ok(render(80) > 0.001, "The first timbre reaches Master directly");

context = core.request("/api/timbres/add", { kind: "bus" });
const bus = context.timbres.find((t) => t.kind === "bus");
assert.equal(bus.bus, 1);
assert.equal(bus.name, "Bus 1");
const sends = Array(8).fill(0);
sends[0] = 1;
core.request("/api/timbres/update", { id: 0, gain: 0, sends });
render(20);
assert.ok(render(60) > 0.001, "The send reaches Master through Bus 1");
core.request("/api/timbres/update", { id: 0, sends: Array(8).fill(0) });
render(40);
assert.ok(render(20) < 1e-4, "Without a level or a send the timbre is silent");

// A bus keeps its role when another preset loads into it, and Granular
// prepares its input history with the preset.
core.request("/api/timbres/update", { id: 0, sends });
core.request("/api/timbres/select", { id: bus.id });
core.request("/api/preset", utility.request("/api/preset"));
let preset = core.request("/api/preset");
assert.equal(preset.bus.master, false, "A loaded preset turned the bus into something else");
assert.equal(core.request("/api/timbres").timbres.find((t) => t.id === bus.id).kind, "bus");
preset.bus.granular = true;
preset.bus.filters = true;
core.request("/api/preset", preset);
render(40);
const state = core.request("/api/state");
assert.equal(state.sample_name, "Bus input");
assert.ok(render(40) > 0.001, "A Granular bus with filters still passes sound");

// Scenes keep Master, the bus number and every send.
const packed = core.call("sad_scene_pack");
const recalled = new Core(module, 8000);
recalled.call("sad_load", utility.call("sad_pack"));
recalled.call("sad_scene_load", packed);
context = recalled.request("/api/timbres");
assert.equal(context.timbres.filter((t) => t.kind === "master").length, 1);
const restored = context.timbres.find((t) => t.kind === "bus");
assert.equal(restored.bus, 1);
assert.equal(context.timbres.find((t) => t.id === 0).sends[0], 1);

// Removing the bus clears the sends to its number.
context = core.request("/api/timbres/remove", { id: bus.id });
assert.equal(context.timbres.find((t) => t.id === 0).sends[0], 0);
// Each section sends its output to a bus; its Chain share decides what goes
// on down the timbre's chain.
const split = new Core(module, 8000);
split.call("sad_load", utility.call("sad_pack"));
const resynthBus = split
  .request("/api/timbres/add", { kind: "bus" })
  .timbres.find((t) => t.kind === "bus");
split.request("/api/timbres/select", { id: 0 });
// Fully wet, one voice, no effect tails and the filters out of the chain:
// the timbre plays what Resynth hands on.
const wet = split.request("/api/preset");
wet.sequencer.enabled = false;
wet.color_chain = [];
wet.pre_color_chain = [];
Object.assign(wet.resynth, { enabled: true, mix: 1 });
wet.taps = { resynth: { chain: 1, send: 0, bus: 1 }, filters: { chain: 0 } };
split.request("/api/preset", wet);
assert.deepEqual(split.request("/api/preset").taps.resynth, { chain: 1, send: 0, bus: 1 });
split.request("/api/transport", { playing: true, midi_gate: false });
const splitLevel = (changes) => {
  split.request("/api/parameters", changes);
  let peak = 0;
  for (let i = 0; i < 60; i++)
    for (const x of split.process(128)) {
      assert.ok(Number.isFinite(x));
      if (i >= 40) peak = Math.max(peak, Math.abs(x));
    }
  return peak;
};
const near = (a, b) => a > b * 0.7 && a < b * 1.4;
const kept = splitLevel({ resynth_send: 0 });
assert.ok(kept > 0.01, `Resynth plays in the timbre (${kept})`);
const dry = splitLevel({ resynth_enabled: 0 });
split.request("/api/timbres/update", { id: 0, gain: 0 });
const throughBus = splitLevel({ resynth_enabled: 1, resynth_send: 1 });
assert.ok(near(throughBus, kept), `The bus passes the Resynth output to Master (${throughBus} vs ${kept})`);
split.request("/api/timbres/update", { id: 0, muted: true });
const muted = splitLevel({});
assert.ok(muted < kept * 0.001, `Mute silences the sends (${muted})`);
split.request("/api/timbres/update", { id: 0, gain: 1, muted: false });
split.request("/api/timbres/update", { id: resynthBus.id, muted: true });
const left = splitLevel({ tap_resynth_chain: 0 });
assert.ok(near(left, dry), `Out of the chain, the timbre plays without Resynth (${left} vs ${dry})`);
split.request("/api/timbres/remove", { id: resynthBus.id });
const missing = splitLevel({ tap_resynth_chain: 1 });
assert.ok(near(missing, kept), `A missing bus leaves the timbre as it was (${missing} vs ${kept})`);
// An advanced mode grain group sends its grains to a bus of its own. Each
// case is a fresh scene rendered from the start, so all hear the same grains.
const layerScene = (changes, edit = () => {}) => {
  const scene = new Core(module, 8000);
  scene.call("sad_load", utility.call("sad_pack"));
  const bus = scene.request("/api/timbres/add", { kind: "bus" }).timbres.find((t) => t.kind === "bus");
  scene.request("/api/timbres/select", { id: 0 });
  const grainy = scene.request("/api/preset");
  grainy.sequencer.enabled = false;
  grainy.color_chain = [];
  grainy.pre_color_chain = [];
  grainy.params.mix = 1;
  // Only group B plays.
  grainy.grain_groups = { advanced: true, groups: [0, 1, 0, 0].map((share) => ({ share })) };
  scene.request("/api/preset", grainy);
  scene.request("/api/parameters", changes);
  edit(scene, bus);
  scene.request("/api/transport", { playing: true, midi_gate: false });
  let peak = 0;
  for (let i = 0; i < 60; i++)
    for (const x of scene.process(128)) {
      assert.ok(Number.isFinite(x));
      if (i >= 40) peak = Math.max(peak, Math.abs(x));
    }
  return { scene, peak };
};
const onBus = { grain_group2_bus: 1, grain_group2_send: 1, grain_group2_chain: 0 };
const layerKept = layerScene({}).peak;
assert.ok(layerKept > 0.01, `Group B plays in the timbre (${layerKept})`);
const routed = layerScene(onBus);
assert.deepEqual(routed.scene.request("/api/preset").grain_groups.groups[1].output, { chain: 0, send: 1, bus: 1 });
assert.ok(near(routed.peak, layerKept), `Bus 1 passes group B to Master (${routed.peak} vs ${layerKept})`);
const layerOff = layerScene(onBus, (scene, bus) =>
  scene.request("/api/timbres/update", { id: bus.id, muted: true })).peak;
assert.ok(layerOff < layerKept * 0.001, `Out of the chain, group B plays only through its bus (${layerOff})`);
const layerClosed = layerScene({ ...onBus, grain_advanced: 0 }).peak;
assert.ok(near(layerClosed, layerKept), `Outside advanced mode the grains stay in the timbre (${layerClosed})`);
// The mixer chooses where a timbre enters Master: at Post-filter FX it skips
// Master's filters, at the input it runs them. Each case starts a fresh scene.
const entering = (entries, filters) => {
  const scene = new Core(module, 8000);
  scene.call("sad_load", utility.call("sad_pack"));
  const id = scene.request("/api/timbres").timbres.find((t) => t.kind === "master").id;
  scene.request("/api/timbres/select", { id });
  const preset = scene.request("/api/preset");
  preset.bus.filters = filters;
  preset.params.filter = 0.05;
  scene.request("/api/preset", preset);
  scene.request("/api/timbres/select", { id: 0 });
  if (entries) scene.request("/api/timbres/update", { id: 0, entries });
  scene.request("/api/transport", { playing: true, midi_gate: false });
  const out = [];
  for (let i = 0; i < 64; i++) out.push(...scene.process(128));
  return { out: out.slice(out.length / 2), scene };
};
const atPost = Array.from({ length: 9 }, () => [1, 0, 0, 0]);
atPost[0] = [0, 0, 0, 1];
const open = entering(null, false).out;
const closedMaster = entering(null, true).out;
const { out: skipped, scene: mixed } = entering(atPost, true);
const most = (a, b) => a.reduce((m, x, i) => Math.max(m, Math.abs(x - (b ? b[i] : 0))), 0);
const loud = most(open);
assert.ok(loud > 0.01, `The timbre plays (${loud})`);
assert.ok(most(closedMaster, open) > loud * 0.1, "Master's filters shape what enters at its input");
assert.ok(most(skipped, open) < loud * 1e-3, `Post-filter FX entry skips Master's filters (${most(skipped, open)})`);
assert.deepEqual(mixed.request("/api/timbres").timbres[0].entries[0], [0, 0, 0, 1]);
const reloaded = new Core(module, 8000);
reloaded.call("sad_load", utility.call("sad_pack"));
reloaded.call("sad_scene_load", mixed.call("sad_scene_pack"));
assert.deepEqual(reloaded.request("/api/timbres").timbres[0].entries[0], [0, 0, 0, 1], "Scenes keep the entries");
console.log(
  "Buses: Master always present, sends reach Master through Bus 1, roles survive preset loads, scenes keep routing, sections send to a bus, timbres enter Master after its filters.",
);
