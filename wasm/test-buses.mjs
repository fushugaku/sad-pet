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
console.log(
  "Buses: Master always present, sends reach Master through Bus 1, roles survive preset loads, scenes keep routing.",
);
