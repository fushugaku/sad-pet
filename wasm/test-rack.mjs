// Rack timbres in the shipping WASM, in discarded buffers without an audio
// device: drums and a synth in one instrument, meeting in its Mixer.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import vm from "node:vm";

const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));
const utility = new Core(module, 8000),
  core = new Core(module, 8000);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
core.call("sad_load", utility.call("sad_pack"));
// Only the Rack sounds.
core.request("/api/timbres/update", { id: 0, muted: true });

const id = core.request("/api/timbres/add", { kind: "rack" }).active;
let state = core.request("/api/state");
assert.equal(state.timbres.timbres.find((t) => t.id === id).kind, "rack");
assert.deepEqual(
  state.preset.rack.modules.map((m) => m.module),
  ["drums", "synth", "filters", "mixer", "post_fx"],
);
assert.ok(state.preset.drums && state.preset.synth, "a new Rack holds its sources' settings");
assert.equal(state.midi_gate, true, "a Rack waits for notes, as a Synth does");
// Without FX B, no reverb tail outlasts a muted strip.
core.request("/api/preset", {
  ...state.preset,
  rack: { ...state.preset.rack, modules: state.preset.rack.modules.slice(0, 4) },
});

const peak = (blocks) => {
  let p = 0;
  for (let i = 0; i < blocks; i++)
    for (const x of core.process(128)) {
      assert.ok(Number.isFinite(x));
      p = Math.max(p, Math.abs(x));
    }
  return p;
};
// A kick on every beat; the drums play their pattern without notes.
core.request("/api/drums/edit", [0, 4, 8, 12].map((step) => ({ track: 0, step, field: 0, value: 1 })));
core.request("/api/transport", { playing: true });
assert.ok(peak(80) > 0.01, "the drum pattern plays in the Rack");
state = core.request("/api/state");
assert.ok(state.rack_peaks[2] > 0.01, "the Drums strip meters its chain");

// Muting the Drums strip silences the rack until a note plays the synth.
core.request("/api/parameters", { rack_drums_mute: 1 });
// The strip fades out rather than click, and the kick's tail with it.
peak(60);
assert.ok(peak(80) < 1e-4, "the muted strip still sounds");
core.request("/api/state"); // Reading the meters starts them over.
core.request("/api/note", { kind: "note_on", channel: 0, note: 60, velocity: 110 });
assert.ok(peak(40) > 0.01, "a note plays the Rack's synth");
state = core.request("/api/state");
assert.ok(state.rack_peaks[1] > 0.01 && state.rack_peaks[2] < 1e-6, JSON.stringify(state.rack_peaks));
core.request("/api/note", { kind: "note_off", channel: 0, note: 60, velocity: 0 });
assert.equal(core.request("/api/preset").rack.strips.drums.muted, true);

// A Grain engine under the Mixer keeps a live history of what it hears.
core.request("/api/parameters", { rack_drums_mute: 0 });
const preset = core.request("/api/preset");
preset.rack.modules = ["drums", "mixer", "grains", "post_fx"].map((module) => ({ module, enabled: true }));
core.request("/api/preset", preset);
state = core.request("/api/state");
assert.equal(state.sample_name, "Rack");
assert.equal(state.input.active, true);
assert.ok(!state.preset.synth, "a removed Synth module leaves no settings");
peak(120);
state = core.request("/api/state");
assert.ok(state.active_grains > 0, "the Grain engine granulates the mixed chain");
assert.ok(state.waveform.some((bin) => bin.some((v) => Math.abs(v) > 1e-4)), "the history shows");

// The telemetry worker decodes the Rack and its meters.
const capacity = core.wasm.sad_snapshot_capacity(),
  packet = new Uint8Array(capacity);
const size = core.snapshot(packet);
assert.ok(size > 0);
const decoded = utility.json("sad_snapshot_import", packet.slice(0, size));
assert.equal(decoded.state.preset.rack.modules.length, 4);
assert.equal(decoded.state.rack_peaks.length, 3);
assert.equal(decoded.state.sample_name, "Rack");

// Scenes keep the Rack.
utility.call("sad_scene_load", core.call("sad_scene_pack"));
const scene = utility.json("sad_scene_document");
assert.ok(scene.timbres.some((t) => t.meta.kind === "rack" && t.preset.rack.modules[2].module === "grains"));

// The page's plan of chains matches the engine's (rack::Plan tests).
const context = vm.createContext({ window: {}, document: {}, Node: {} });
vm.runInContext(await readFile(new URL("../rust/web/rack.js", import.meta.url), "utf8"), context);
const plan = (modules) =>
  JSON.parse(JSON.stringify(context.window.SadRack.plan(modules.map((m) =>
    typeof m === "string" ? { module: m, enabled: true } : m))));
let steps = plan(["post_fx", "drums", "filters", "synth", "filters", "mixer", "pre_fx", "sample"]);
assert.deepEqual(steps.map((s) => s.chain), [null, "drums", "drums", "synth", "synth", "mix", "mix", "sample"]);
assert.deepEqual(steps.map((s) => s.perNote), [false, false, false, true, true, false, false, true]);
assert.deepEqual(steps.map((s) => s.mixed), [false, true, false, true, false, false, false, false]);
steps = plan(["sample", "grains", "filters"]);
assert.ok(steps[1].readsFile && steps[1].perNote && steps[2].perNote);
steps = plan(["sample", "filters", "grains"]);
assert.ok(steps[1].perNote && !steps[2].readsFile && !steps[2].perNote);
steps = plan([{ module: "sample", enabled: false }, "grains"]);
assert.ok(!steps[1].readsFile);

console.log("Rack timbres: drums and synth meet in the Mixer, grains read the chain");
