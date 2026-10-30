import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));

function scene(rate, bus = false) {
  const core = new Core(module, rate);
  const added = core.request("/api/timbres/add", { kind: "tb303" });
  const bass = added.timbres.find(t => t.id === added.active);
  assert.equal(bass.kind, "tb303");
  assert.equal(bass.name, "303 Bass 1");
  core.request("/api/timbres/update", { id: 0, muted: true });
  if (bus) {
    const next = core.request("/api/timbres/add", { kind: "bus" });
    const target = next.timbres.find(t => t.id === next.active);
    const sends = Array(8).fill(0); sends[target.bus - 1] = 1;
    core.request("/api/timbres/update", { id: bass.id, gain: 0, sends });
    core.request("/api/timbres/select", { id: bass.id });
  }
  return { core, bass };
}
function render(core, frames, block = 128) {
  const out = new Float32Array(frames * 2);
  for (let n = 0; n < frames; n += block) out.set(core.process(Math.min(block, frames - n)), n * 2);
  assert.ok(out.every(x => Number.isFinite(x) && Math.abs(x) <= 0.891251));
  return out;
}
const energy = a => a.reduce((sum, x) => sum + x * x, 0) / a.length;
for (const rate of [8000, 44100, 48000]) {
  const { core, bass } = scene(rate);
  assert.equal(energy(render(core, 512)), 0, "303 played with no gate or transport");
  core.request("/api/transport", { playing: true });
  const visited = new Set();
  let power = 0;
  for (let n = 0; n < rate * 2.1; n += 128) {
    power += energy(render(core, 128));
    visited.add(core.request("/api/state").tb303.step);
  }
  assert.ok(power > 0.01, "303 pattern is silent");
  assert.equal(visited.size, 16, "Pattern did not play every step");
  core.request("/api/transport", { playing: false });
  render(core, Math.round(rate * .3));
  assert.ok(energy(render(core, 512)) < 1e-10, "Stop left a 303 gate open");
  core.request("/api/parameters", { tb303_pattern: 0, tb303_step3_note: 55, tb303_step3_accent: 1, tb303_step3_slide: 1 });
  core.request("/api/note", { kind: "note_on", channel: 0, note: 36, velocity: 80 });
  assert.ok(energy(render(core, Math.round(rate * .1))) > 1e-6, "303 MIDI is silent while transport is stopped");
  const before = core.request("/api/state").tb303.envelope;
  core.request("/api/note", { kind: "note_on", channel: 0, note: 48, velocity: 80 });
  render(core, Math.round(rate * .022));
  const sliding = core.request("/api/state").tb303;
  assert.ok(sliding.slide && Math.abs(sliding.note - (48 - 12 / Math.E)) < .03);
  assert.ok(sliding.envelope < before, "Legato restarted the 303 envelope");
  for (const note of [36, 48]) core.request("/api/note", { kind: "note_off", channel: 0, note, velocity: 0 });
  render(core, Math.round(rate * .3));
  assert.ok(energy(render(core, 512)) < 1e-10);
  const preset = core.request("/api/preset");
  const packed = core.call("sad_scene_pack");
  const restored = new Core(module, rate);
  restored.call("sad_scene_load", packed);
  assert.deepEqual(restored.request("/api/preset").tb303, preset.tb303);
  assert.equal(restored.request("/api/timbres").timbres.find(t => t.id === bass.id).kind, "tb303");
  // A 303 preset cannot turn the scene's protected Master into an instrument.
  const master = restored.request("/api/timbres").timbres.find(t => t.kind === "master");
  restored.request("/api/timbres/select", { id: master.id });
  restored.request("/api/preset", preset);
  assert.equal(restored.request("/api/preset").bus.master, true);
  assert.equal(restored.request("/api/preset").tb303, undefined);
  console.log(`303 ${rate}: pattern, all sixteen steps, MIDI, RC slide, legato, stop, scene recall and Master role passed.`);
}
const rate = 48000;
const direct = scene(rate), routed = scene(rate, true), varied = scene(rate);
for (const { core } of [direct, routed, varied]) core.request("/api/transport", { playing: true });
const a = render(direct.core, rate), b = render(varied.core, rate, 511), c = render(routed.core, rate);
assert.deepEqual(a, b, "303 timing depends on audio block size");
let error = 0;
for (let i = rate / 2; i < a.length; i++) error = Math.max(error, Math.abs(a[i] - c[i]));
assert.ok(error < 1e-4, `303 bus route changed audio: ${error}`);
const worker = new Core(module, rate);
worker.call("sad_scene_adopt", direct.core.call("sad_scene_pack"));
const document = worker.json("sad_scene_document");
const wav = new DataView(worker.call("sad_scene_render_document", { seconds: 1, scene: document }));
assert.equal(String.fromCharCode(...new Uint8Array(wav.buffer, 0, 4)), "RIFF");
assert.ok(wav.byteLength > rate * 4, "303 WAV export has no audio");
console.log("303: sample-exact block sizes, Bus routing and scene WAV export passed.");

// Extra RADIAS layers share the 303 ladder/VCA, and can sound on their own.
{
  const { core } = scene(8000);
  core.request("/api/parameters", { tb303_pattern: 0, tb303_level: 0, tb303_count: 4 });
  core.request("/api/note", { kind: "note_on", channel: 0, note: 36, velocity: 80 });
  assert.ok(energy(render(core, 800)) > 1e-5, "RADIAS layers do not reach the 303 filter");
  for (let wave = 0; wave < 6; wave++) for (let mode = 0; mode < 4; mode++) {
    core.request("/api/parameters", Object.fromEntries(Array.from({ length: 4 }, (_, i) => [
      [`tb303_osc${i + 1}_waveform`, wave], [`tb303_osc${i + 1}_mode`, mode],
      [`tb303_osc${i + 1}_control1`, .8], [`tb303_osc${i + 1}_control2`, .6],
    ]).flat()));
    render(core, 256);
  }
  core.request("/api/parameters", { tb303_count: 0 });
  render(core, 2400);
  assert.ok(energy(render(core, 512)) < 1e-10, "Removing all RADIAS layers leaves sound behind");
}
// Muting the audible 303 VCO must not remove it as a sync/ring/cross source.
{
  const outputs = [];
  for (const interaction of [0, 1, 2, 3, 4]) {
    const { core } = scene(8000);
    core.request("/api/parameters", {
      tb303_pattern: 0, tb303_level: 0, tb303_count: 1,
      tb303_osc1_transpose: 7, tb303_osc1_interaction_source: 0,
      tb303_osc1_interaction: interaction < 4 ? interaction : 0,
      tb303_osc1_mode: interaction === 4 ? 1 : 0,
      tb303_osc1_cross_input: 1, tb303_osc1_control1: .8,
    });
    core.request("/api/note", { kind: "note_on", channel: 0, note: 36, velocity: 80 });
    render(core, 800);
    const out = render(core, 1024);
    assert.ok(energy(out) > 1e-8, "Muted 303 VCO stopped driving its RADIAS modulation link");
    outputs.push(out);
    const restored = new Core(module, 8000);
    restored.call("sad_scene_load", core.call("sad_scene_pack"));
    assert.equal(restored.request("/api/preset").tb303.oscillators[0].interaction_source, 0);
    assert.equal(restored.request("/api/preset").tb303.oscillators[0].cross_input, "source");
  }
  for (let a = 0; a < outputs.length; a++) for (let b = a + 1; b < outputs.length; b++)
    assert.ok(outputs[a].some((v, i) => Math.abs(v - outputs[b][i]) > 1e-4), "303 Ring/Sync/Cross links did not change RADIAS audio");
}
function lane(core, id, target, pitch) {
  const t = structuredClone(core.request("/api/sequencer/catalog").track);
  t.id = id; t.target = target; t.root_note = 60;
  t.steps.forEach(s => { s.pitch = pitch; s.velocity = 80; s.gate = 2; });
  return t;
}
{
  const { core } = scene(8000);
  const p = core.request("/api/preset");
  p.tb303.count = 1;
  p.sequencer.enabled = true;
  p.sequencer.tracks = [lane(core, 1, { kind: "main_pitch" }, 72),
    lane(core, 2, { kind: "tb303_oscillator", index: 0 }, 67)];
  p.sequencer.tracks[0].destinations = ["tb303_cutoff", "tb303_osc1_control1", null];
  p.sequencer.tracks[0].steps.forEach(s => { s.destination1 = .1; s.destination2 = .2; });
  core.request("/api/preset", p);
  core.request("/api/transport", { playing: true });
  render(core, 256);
  const state = core.request("/api/state");
  assert.ok(Math.abs(state.tb303.note - 48) < .01, "Main pitch does not transpose the 303 voice");
  assert.ok(Math.abs(state.knobs.tb303_osc1_transpose.current - 7) < .01);
  assert.ok(Math.abs(state.knobs.tb303_cutoff.current - .55) < .01);
  assert.ok(Math.abs(state.knobs.tb303_osc1_control1.current - .2) < .01);
  const restored = new Core(module, 8000);
  restored.call("sad_scene_load", core.call("sad_scene_pack"));
  assert.deepEqual(restored.request("/api/preset").tb303, p.tb303);
  assert.equal(restored.request("/api/preset").sequencer.tracks[1].target.kind, "tb303_oscillator");
}
{
  const { core } = scene(8000);
  const p = core.request("/api/preset");
  p.tb303.count = 2;
  p.sequencer.enabled = true;
  const notes = lane(core, 1, { kind: "notes" }, 48);
  notes.config[1] = 4;
  notes.steps.forEach(s => { s.gate = 0; });
  notes.steps[0].gate = 2;
  p.sequencer.tracks = [notes];
  core.request("/api/preset", p);
  core.request("/api/transport", { playing: true });
  assert.ok(energy(render(core, 400)) > 1e-6, "Neuropolis Notes do not play 303/RADIAS");
  const state = core.request("/api/state");
  assert.equal(state.tb303.step, null, "The internal pattern competes with Neuropolis Notes");
  assert.ok(Math.abs(state.tb303.note - 48) < .01);
  render(core, 3000);
  assert.ok(energy(render(core, 256)) < 1e-9, "303 pattern filled a Neuropolis rest");
}
console.log("303 + RADIAS: six waves, four modes, layer removal, Main pitch, individual offsets, parameter lanes, Notes rests and recall passed.");
