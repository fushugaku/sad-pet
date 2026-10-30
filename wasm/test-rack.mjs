// Rack timbres in the shipping WASM, in discarded buffers without an audio
// device: drums and a synth in one instrument, meeting in its Mixer; then a
// Sampler kit and a sung voice beside the synth.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import vm from "node:vm";

const folder = resolve(process.argv[2] || "dist/web");
const { Core, VocalCore } = await import(pathToFileURL(resolve(folder, "core.mjs")));
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
assert.equal(decoded.state.rack_peaks.length, 6);
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

steps = plan(["sampler", "filters", "vocal", "mixer"]);
assert.deepEqual(steps.map((s) => s.chain), ["sampler", "sampler", "vocal", "mix"]);
assert.deepEqual(steps.map((s) => s.perNote), [false, false, false, false]);
assert.deepEqual(steps.map((s) => s.mixed), [true, false, true, false]);

// Rearranging takes out only what moves: a module switched on or off stays
// in, so its button keeps focus and the page its scroll.
{
  const parent = {
    children: [],
    taken: [],
    get firstElementChild() {
      return this.children[0] ?? null;
    },
    insertBefore(node, before) {
      if (this.children.includes(node)) node.remove();
      const i = before ? this.children.indexOf(before) : this.children.length;
      this.children.splice(i, 0, node);
    },
  };
  const node = (name) => ({
    name,
    get nextElementSibling() {
      return parent.children[parent.children.indexOf(this) + 1] ?? null;
    },
    remove() {
      parent.taken.push(name);
      parent.children.splice(parent.children.indexOf(this), 1);
    },
  });
  const [a, b, c] = ["a", "b", "c"].map(node);
  const reorder = (nodes) => {
    parent.taken = [];
    context.window.SadRack.reorder(parent, nodes);
    assert.deepEqual(parent.children.map((n) => n.name), nodes.map((n) => n.name));
    return parent.taken;
  };
  reorder([a, node("link"), b, node("link"), c]);
  // New links between the same modules.
  assert.deepEqual(reorder([a, node("link"), b, node("link"), c]), ["link", "link"]);
  // Moving c up takes out c alone.
  assert.deepEqual(reorder([a, node("link"), c, node("link"), b]), ["link", "link", "c"]);
  assert.deepEqual(reorder([b]), ["a", "link", "c", "link"]);
}

// A Sampler and a Vocal module, at the rate the Vocal tests sing at: the kit
// takes samples into the Rack's pool, its pads and Pad FX play into the
// Sampler strip, and the voice the worker renders into the Vocal strip.
{
  const rate = 16000;
  const utility = new Core(module, rate),
    core = new Core(module, rate),
    vocal = new VocalCore(module, rate);
  utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
  core.call("sad_load", utility.call("sad_pack"));
  core.request("/api/timbres/update", { id: 0, muted: true });
  const peak = (seconds) => {
    let p = 0;
    for (let i = 0; i < Math.round((seconds * rate) / 128); i++)
      for (const x of core.process(128)) {
        assert.ok(Number.isFinite(x));
        p = Math.max(p, Math.abs(x));
      }
    return p;
  };
  /** A 32-bit float stereo WAV of a sine. */
  const sine = (hz, seconds) => {
    const frames = Math.round(rate * seconds);
    const buffer = new ArrayBuffer(44 + frames * 8),
      view = new DataView(buffer);
    const text = (o, v) => [...v].forEach((c, i) => view.setUint8(o + i, c.charCodeAt(0)));
    text(0, "RIFF");
    view.setUint32(4, buffer.byteLength - 8, true);
    text(8, "WAVE");
    text(12, "fmt ");
    view.setUint32(16, 16, true);
    view.setUint16(20, 3, true);
    view.setUint16(22, 2, true);
    view.setUint32(24, rate, true);
    view.setUint32(28, rate * 8, true);
    view.setUint16(32, 8, true);
    view.setUint16(34, 32, true);
    text(36, "data");
    view.setUint32(40, frames * 8, true);
    const data = new Float32Array(buffer, 44);
    for (let i = 0; i < frames; i++) data[i * 2] = data[i * 2 + 1] = Math.sin((2 * Math.PI * hz * i) / rate) * 0.5;
    return buffer;
  };
  const jobs = () => core.json("sad_vocal_jobs");
  const rack = core.request("/api/timbres/add", { kind: "rack" }).active;
  jobs();
  let preset = core.request("/api/preset");
  preset.rack.modules = ["sampler", "synth", "vocal", "mixer"].map((module) => ({ module, enabled: true }));
  core.request("/api/preset", preset);
  preset = core.request("/api/preset");
  assert.equal(core.request("/api/timbres").timbres.find((t) => t.id === rack).kind, "rack");
  assert.equal(preset.sampler.pads.length, 16, "a new Sampler module starts with bank A");
  assert.ok(preset.vocal.score.voice, "a new Vocal module sings with a bundled voice");
  assert.ok(!preset.bus, "a Rack's kit and voice need no bus");
  let queued = jobs();
  assert.ok(queued.some((j) => j.type === "job" && j.id === rack && j.fresh), "the worker plans the Rack's score");

  // The kit's pool, as the Sampler editor fills it from the library.
  const prepared = utility.call("sad_sampler_prepare", sine(440, 0.4));
  assert.equal(core.prefixedJson("sad_sampler_sample", "tone", prepared, rack).frames, Math.round(rate * 0.4));
  assert.deepEqual(core.request("/api/sampler/state").view.loaded, ["tone"]);
  preset.sampler.pads[0].zones = [{ sample: "tone" }];
  core.request("/api/sampler/settings", { settings: preset.sampler });
  // An edit of the modules that carries no kit keeps the timbre's own.
  preset = core.request("/api/preset");
  delete preset.sampler;
  preset.rack.strips = { ...preset.rack.strips, synth: { level: 1, pan: 0.5, muted: true, solo: false } };
  core.request("/api/preset", preset);
  assert.equal(core.request("/api/preset").sampler.pads[0].zones[0].sample, "tone");
  assert.deepEqual(core.request("/api/sampler/state").view.loaded, ["tone"], "the pool stays");

  core.request("/api/transport", { playing: true });
  // Long enough for the muted Synth strip to fade all the way.
  peak(0.6);
  core.request("/api/state");
  core.request("/api/note", { kind: "note_on", channel: 0, note: 36, velocity: 127 });
  assert.ok(peak(0.2) > 0.05, "a note plays the Rack's pad");
  let state = core.request("/api/state");
  assert.equal(state.rack_peaks.length, 6);
  assert.ok(state.rack_peaks[3] > 0.05 && state.rack_peaks[1] < 1e-6, JSON.stringify(state.rack_peaks));
  core.request("/api/note", { kind: "note_off", channel: 0, note: 36, velocity: 0 });
  peak(0.5);

  // A pad's FX chain belongs to the Rack and returns into its Sampler chain.
  const context = core.request("/api/sampler/chain", { pad: 1 });
  const chain = context.timbres.find((t) => t.kind === "pad_fx");
  assert.equal(chain?.owner, rack);
  core.request("/api/timbres/select", { id: rack });
  core.request("/api/state");
  core.request("/api/sampler/trigger", { pad: 1, velocity: 127 });
  assert.ok(peak(0.2) > 0.05, "the pad plays through its chain");
  state = core.request("/api/state");
  assert.ok(state.rack_peaks[3] > 0.05, JSON.stringify(state.rack_peaks));
  peak(0.5);

  core.request("/api/transport", { playing: false });

  // The voice: the worker renders the Rack's phrases, and they sing in the
  // Vocal strip with the song transport.
  const score = {
    language: "ja",
    style: { auto_breath: false },
    notes: [{ id: 1, start: 0, length: 960, key: 64, lyric: "さ" }],
  };
  core.request("/api/vocal/score", { score });
  queued = jobs();
  assert.equal(queued.length, 1);
  for (const job of queued) vocal.command(job);
  while (vocal.step(0));
  for (const update of vocal.updates()) {
    const wasm = core.wasm;
    const count = update.plan.length;
    const plan = new Float64Array(wasm.memory.buffer, wasm.sad_vocal_plan(count), count * 3);
    update.plan.forEach((entry, i) => plan.set(entry, i * 3));
    wasm.sad_vocal_plan_commit(update.id, count);
    for (const key of update.fresh) {
      const samples = new Float32Array(vocal.clip(key));
      const start = update.plan.find((p) => p[0] === key)[1];
      const pointer = wasm.sad_vocal_clip_begin(update.id, key, start, samples.length);
      new Float32Array(wasm.memory.buffer, pointer, samples.length).set(samples);
      wasm.sad_vocal_clip_commit();
    }
  }
  assert.equal(core.request("/api/vocal/state").clips, 1);
  core.request("/api/state");
  core.request("/api/vocal/transport", { start: 0, playing: true });
  assert.ok(peak(0.8) > 0.01, "the Rack sings");
  state = core.request("/api/state");
  assert.ok(state.rack_peaks[4] > 0.01 && state.rack_peaks[3] < 1e-6, JSON.stringify(state.rack_peaks));
  core.request("/api/transport", { playing: false });

  // Scenes keep the kit with its pool, the Pad FX chain and the score.
  const recalled = new Core(module, rate);
  recalled.call("sad_load", utility.call("sad_pack"));
  recalled.call("sad_scene_load", core.call("sad_scene_pack"));
  const timbres = recalled.request("/api/timbres").timbres;
  assert.equal(timbres.find((t) => t.id === rack).kind, "rack");
  assert.equal(timbres.find((t) => t.kind === "pad_fx")?.owner, rack);
  recalled.request("/api/timbres/select", { id: rack });
  const kept = recalled.request("/api/preset");
  assert.equal(kept.sampler.pads[0].zones[0].sample, "tone");
  assert.equal(kept.vocal.score.notes[0].lyric, "さ");
  assert.deepEqual(recalled.request("/api/sampler/state").view.loaded, ["tone"]);
  const asked = recalled.json("sad_vocal_jobs");
  assert.ok(asked.some((j) => j.id === rack && j.fresh), "the recalled Rack asks for its clips");
  // Saved scene documents keep the kit's audio too.
  utility.call("sad_scene_load", core.call("sad_scene_pack"));
  const document = utility.json("sad_scene_document");
  const saved = document.timbres.find((t) => t.meta.id === rack);
  assert.equal(saved.kit_samples[0].id, "tone");
  assert.deepEqual(saved.preset.rack.modules.map((m) => m.module), ["sampler", "synth", "vocal", "mixer"]);

  // Without the modules the kit, its chain and the score go.
  preset = core.request("/api/preset");
  preset.rack.modules = ["synth", "mixer"].map((module) => ({ module, enabled: true }));
  core.request("/api/preset", preset);
  preset = core.request("/api/preset");
  assert.ok(!preset.sampler && !preset.vocal);
  assert.ok(!core.request("/api/timbres").timbres.some((t) => t.kind === "pad_fx"), "the Pad FX chain goes with the kit");
  assert.ok(jobs().some((j) => j.type === "forget" && j.id === rack));
}

console.log("Rack timbres: drums and synth meet in the Mixer, grains read the chain, a kit and a voice join them");
