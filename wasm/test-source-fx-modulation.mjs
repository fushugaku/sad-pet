// Exercise the actual Rack insert, saved routes and AudioWorklet snapshots.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));
const utility = new Core(module, 8000);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const prepared = utility.call("sad_pack");
const catalog = utility.request("/api/sequencer/catalog");
const defaults = utility.request("/api/color-defaults");
for (const id of ["source_fx1_waveshaper_depth", "source_fx1_mix"])
  assert.ok(catalog.parameters.some(p => p.id === id), `${id} missing from the sequencer`);

const close = (actual, expected) => assert.ok(Math.abs(actual - expected) < 1e-5, `${actual} != ${expected}`);
const route = parameter => ({ color: { slot: 90, parameter } });
const key = parameter => `color:90:${parameter}`;
const render = (core, blocks = 16) => {
  const audio = [];
  for (let i = 0; i < blocks; i++) audio.push(...core.process(128));
  assert.ok(audio.every(Number.isFinite));
  return audio;
};
const difference = (a, b) => a.reduce((sum, v, i) => sum + (v - b[i]) ** 2, 0) / a.length;
function rack() {
  const core = new Core(module, 8000);
  core.call("sad_load", prepared);
  core.request("/api/timbres/update", { id: 0, muted: true });
  core.request("/api/timbres/add", { kind: "rack" });
  const p = core.request("/api/preset");
  p.color_chain = [];
  p.pre_color_chain = [];
  p.source_color_chain = [{ ...structuredClone(defaults.slot), id: 90, kind: "waveshaper", mix: 0.2 }];
  Object.assign(p.source_color_chain[0].settings, { waveshaper_type: "hard_clip", waveshaper_depth: 0.1 });
  p.rack.modules = ["sample", "source_fx", "mixer"].map(module => ({ module, enabled: true }));
  p.lfos.forEach(l => Object.assign(l, { destination: "none", destination2: "none", destination3: "none", depth: 0, depth2: 0, depth3: 0 }));
  p.envs.forEach(e => Object.assign(e, { destination: "none", extra_destination: "none", destination3: "none", depth: 0, extra_depth: 0, depth3: 0, attack: 0, decay: 0, sustain: 1, release: 0 }));
  p.matrix = [];
  p.macros = [];
  p.sequencer.enabled = false;
  return { core, p };
}
function start(core, p) {
  core.request("/api/preset", p);
  core.request("/api/transport", { playing: true, midi_gate: false });
  core.request("/api/note", { kind: "note_on", channel: 0, note: 60, velocity: 127 });
}
function snapshot(core) {
  const packet = new Uint8Array(core.wasm.sad_snapshot_capacity());
  const size = core.snapshot(packet);
  assert.ok(size > 0, "source modulation snapshot overflowed");
  const state = utility.json("sad_snapshot_import", packet.slice(0, size)).state;
  core.wasm.sad_snapshot_ack(core.wasm.sad_snapshot_sequence(), 1);
  return state;
}

// Both routes survive sanitize, a JSON round trip and worker adoption.
const { core, p } = rack();
Object.assign(p.envs[1], { destination: route("waveshaper_depth"), depth: 0.6 });
Object.assign(p.lfos[0], { destination: route("mix"), depth: 0.3, rate_hz: 3 });
core.request("/api/preset", JSON.parse(JSON.stringify(p)));
let recalled = core.request("/api/preset");
assert.deepEqual(recalled.envs[1].destination, route("waveshaper_depth"));
assert.deepEqual(recalled.lfos[0].destination, route("mix"));
start(core, recalled);
let previous;
for (let i = 0; i < 4; i++) {
  render(core, 3);
  const direct = core.request("/api/state"), published = snapshot(core);
  for (const parameter of ["waveshaper_depth", "mix"]) {
    const a = direct.knobs[key(parameter)], b = published.knobs[key(parameter)];
    assert.ok(a.modulated && b.modulated);
    for (const field of ["base", "current", "min", "max"]) close(a[field], b[field]);
  }
  close(published.knobs[key("waveshaper_depth")].current, 0.7);
  close(published.knobs[key("mix")].min, 0.05);
  close(published.knobs[key("mix")].max, 0.35);
  assert.notEqual(published.knobs[key("mix")].current, previous);
  previous = published.knobs[key("mix")].current;
  close(published.preset.source_color_chain[0].settings.waveshaper_depth, 0.1);
  close(published.preset.source_color_chain[0].mix, 0.2);
}

// A changed marker must correspond to changed PCM, not just UI feedback.
const base = rack(), mod = rack();
Object.assign(mod.p.envs[1], { destination: route("waveshaper_depth"), depth: 0.7 });
base.p.source_color_chain[0].mix = mod.p.source_color_chain[0].mix = 1;
start(base.core, base.p);
start(mod.core, mod.p);
const clean = render(base.core), wet = render(mod.core);
assert.ok(clean.some(v => Math.abs(v) > 0.001), "Rack's file did not play");
assert.ok(difference(clean, wet) > 1e-6, "ENV changed only the marker, not the Insert FX audio");
// Release the route without replacing the preset: the DSP must restore base.
for (const c of [base.core, mod.core]) c.request("/api/note", { kind: "note_off", channel: 0, note: 60, velocity: 0 });
render(base.core, 10); render(mod.core, 10);
assert.ok(difference(render(base.core), render(mod.core)) < 1e-12, "the processor retained the previous modulation");
close(mod.core.request("/api/state").knobs[key("waveshaper_depth")].current, 0.1);

// A CV lane and an additional destination reach the same realtime insert.
const seq = rack(), lane = structuredClone(catalog.track);
lane.target = { kind: "parameter", parameter: "source_fx1_waveshaper_depth" };
lane.destinations = ["source_fx1_mix", null, null];
lane.steps.forEach(s => Object.assign(s, { cv: 1000, destination1: 0.3 }));
seq.p.sequencer.enabled = true;
seq.p.sequencer.tracks = [lane];
start(seq.core, seq.p);
render(seq.core, 4);
let state = seq.core.request("/api/state");
close(state.knobs[key("waveshaper_depth")].current, 0.3);
close(state.knobs[key("mix")].current, 0.5);
close(state.preset.source_color_chain[0].settings.waveshaper_depth, 0.1);
close(state.preset.source_color_chain[0].mix, 0.2);
state = snapshot(seq.core);
close(state.knobs[key("waveshaper_depth")].current, 0.3);
close(state.knobs[key("mix")].current, 0.5);

// A stale stable ID is still rejected; enabling inserts must not weaken this.
recalled = core.request("/api/preset");
recalled.envs[1].destination = { color: { slot: 999999, parameter: "mix" } };
core.request("/api/preset", recalled);
assert.equal(core.request("/api/preset").envs[1].destination, "none");

// The WS source selector must drive the realtime insert itself, even when
// neither envelope has an ordinary modulation destination.
function envelopeInsert(source, unusedLevel = 1) {
  const fixture = rack();
  fixture.p.envs[1].sustain = 0;
  fixture.p.envs[2].sustain = unusedLevel;
  const slot = fixture.p.source_color_chain[0];
  slot.mix = 1;
  Object.assign(slot.settings, { waveshaper_type: "decimator", waveshaper_depth: 0,
    waveshaper_options: { quality: "clean2x", curve: "soft_clip", envelope: 1,
      envelope_source: source } });
  start(fixture.core, fixture.p);
  return fixture;
}
const externalA = envelopeInsert("env2"), externalB = envelopeInsert("env3"),
  unused = envelopeInsert("env2", 0);
const a = render(externalA.core), b = render(externalB.core), notSelected = render(unused.core);
assert.ok(a.some(v => Math.abs(v) > 0.001), "The external-ENV comparison must play real audio");
assert.ok(difference(a, b) > 1e-6, "The Source FX selector changed metadata without changing PCM");
assert.deepEqual(a, notSelected, "An unselected envelope changed the realtime insert");
const changed = externalA.core.request("/api/preset");
changed.source_color_chain[0].settings.waveshaper_options.envelope_source = "env3";
externalA.core.request("/api/preset", changed);
assert.ok(difference(render(externalA.core), render(unused.core)) > 1e-6,
  "Changing the selected envelope during playback must refresh the realtime insert");
console.log("Source FX modulation passed: saved ENV/LFO routes, PCM, release, CV lanes and worker feedback.");
