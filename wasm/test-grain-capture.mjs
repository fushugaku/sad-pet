// Exercise the shipping audio engine: Main pitch routes control the cloud,
// while the physical Rack still captures and processes its selected input.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const folder = resolve(process.argv[2] || "dist/web");
const variant = process.argv[3] || "sad_pet.wasm";
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, variant)));
const rate = 8000;
const telemetry = new Core(module, rate);

function setup({ granular = false, mode = "chain", slots = 1, filterBefore = false, env = 1, grainsEnabled = true } = {}) {
  const core = new Core(module, rate);
  core.request("/api/timbres/add", { kind: "oscillators" });
  core.request("/api/timbres/update", { id: 0, muted: true });
  const preset = core.request("/api/preset");
  preset.rack.buffer_seconds = .25;
  preset.rack.modules = [
    { module: "oscillators", enabled: true },
    ...(filterBefore ? [{ module: "filters", enabled: true }] : []),
    { module: "grains", enabled: grainsEnabled },
    ...(!filterBefore ? [{ module: "filters", enabled: true }] : []),
    { module: "mixer", enabled: true },
    { module: "post_fx", enabled: true },
  ];
  for (let i = 0; i < preset.oscillators.slots.length; i++)
    preset.oscillators.slots[i].enabled = i === 0;
  preset.grain_capture = { mode, slots };
  preset.color_chain = [];
  preset.pre_color_chain = [];
  preset.source_color_chain = [];
  Object.assign(preset.params, {
    mix: 1, grains: .9, grain_size: .7, position: 0, cloud: 0,
    detune: .5, pitch: .5, stereo_spread: 0, grain_feedback: 0,
    filter: .35, filter_env_depth: .8, filter_env: "env2",
  });
  Object.assign(preset.envs[0], { attack: 0, decay: 0, sustain: 1, release: .1 });
  Object.assign(preset.envs[1], { attack: 0, decay: 0, sustain: env, release: .1 });
  preset.sequencer.enabled = true;
  const track = preset.sequencer.tracks[0];
  track.granular = granular;
  track.oscillator_mask = 1;
  track.config[1] = 1;
  track.config[7] = 100;
  for (const step of track.steps) Object.assign(step, { pitch: 60, gate: 2, velocity: 100 });
  core.request("/api/preset", preset);
  core.request("/api/transport", { playing: true, midi_gate: false });
  return core;
}

function render(core, frames = rate) {
  const audio = [];
  for (let at = 0; at < frames; at += 128) {
    const block = core.process(Math.min(128, frames - at));
    assert.ok(block.every(Number.isFinite), "capture produced nonfinite audio");
    audio.push(...block);
  }
  return audio;
}
const energy = (audio) => audio.reduce((sum, value) => sum + value * value, 0);
const wavePeak = (state) => Math.max(0, ...state.waveform.flat().map(Math.abs));
const difference = (a, b) => a.reduce((sum, value, i) => sum + Math.abs(value - b[i]), 0);

for (const granular of [false, true]) {
  for (const mode of ["chain", "osc"]) {
    const core = setup({ granular, mode });
    const audio = render(core);
    const state = core.request("/api/state");
    assert.ok(state.active_grains > 0, `${mode}, route=${granular}: cloud stopped`);
    assert.ok(wavePeak(state) > .01, `${mode}, route=${granular}: capture waveform is empty`);
    assert.ok(energy(audio) > .001, `${mode}, route=${granular}: wet grains are silent`);
    const packet = new Uint8Array(core.wasm.sad_snapshot_capacity());
    const size = core.snapshot(packet);
    assert.ok(size > 0, "capture snapshot is missing");
    const decoded = telemetry.json("sad_snapshot_import", packet.slice(0, size));
    assert.deepEqual(decoded.state.waveform, state.waveform,
      "the browser telemetry lost the capture waveform");
    core.request("/api/transport", { frozen: true });
    core.request("/api/parameters", { osc1_level: 0 });
    assert.equal(core.request("/api/preset").oscillators.slots[0].level, 0);
    const frozen = render(core);
    const wetEnergy = energy(frozen.slice(rate));
    assert.ok(wetEnergy > .001,
      `${mode}, route=${granular}: captured audio vanished when its OSC went silent`);
    assert.deepEqual(core.request("/api/state").waveform, state.waveform,
      "freeze changed the capture waveform");
    core.request("/api/parameters", { mix: 0 });
    const dry = render(core);
    assert.ok(energy(dry.slice(rate)) < wetEnergy * .001,
      `${mode}, route=${granular}: the frozen playback is coming from the direct OSC`);
  }
}

const excluded = setup({ granular: true, mode: "osc", slots: 2 });
render(excluded);
assert.equal(wavePeak(excluded.request("/api/state")), 0, "unselected OSC entered the capture");

// Both shared and private clouds must reach configured filters on either side.
for (const granular of [false, true]) {
  for (const filterBefore of [false, true]) {
    const closed = render(setup({ granular, filterBefore, env: 0 }));
    const open = render(setup({ granular, filterBefore, env: 1 }));
    assert.ok(difference(closed, open) > 1,
      `filter ENV is bypassed ${filterBefore ? "before" : "after"} Grains, route=${granular}`);
  }
}

assert.ok(difference(
  render(setup({ env: 0, grainsEnabled: false })),
  render(setup({ env: 1, grainsEnabled: false })),
) > 1, "bypassing Grains disabled the downstream filter ENV");

// The shaper on the main filter must also reach the cloud's actual bank.
for (const granular of [false, true]) {
  const reference = energy(render(setup({ granular })));
  const muted = setup({ granular });
  const preset = muted.request("/api/preset");
  const shaper = preset.params.filter_shapers[0].before;
  Object.assign(shaper, { enabled: true, mix: 1, depth: 1 });
  Object.assign(shaper.options, { curve: "custom", points: Array(17).fill(0), quality: "clean2x" });
  muted.request("/api/preset", preset);
  assert.ok(energy(render(muted)) < reference * .001,
    `the filter waveshaper does not process the cloud, route=${granular}`);
}

// Inactive reserved branches must not add another generated signal. Both
// a Sub oscillator and a custom curve with nonzero f(0) expose this mistake.
for (const granular of [false, true]) {
  for (const kind of ["sub", "custom"]) {
    function generatedEnergy(withGrains) {
      const core = setup({ granular });
      const preset = core.request("/api/preset");
      if (!withGrains) preset.rack.modules = preset.rack.modules.filter(m => m.module !== "grains");
      preset.params.grains = 0;
      preset.params.filter = 1;
      preset.params.filter_env_depth = 0;
      preset.oscillators.slots[0].level = 0;
      const track = preset.sequencer.tracks[0];
      track.env_gate = false;
      for (const step of track.steps) Object.assign(step, { gate: 3, velocity: 127 });
      const shaper = preset.params.filter_shapers[0].before;
      Object.assign(shaper, { enabled: true, depth: 1, mix: 1, mode: kind === "sub" ? "sub_sine" : "drive" });
      if (kind === "custom") Object.assign(shaper.options, {
        curve: "custom", points: Array(17).fill(.2), quality: "clean2x",
      });
      core.request("/api/preset", preset);
      render(core);
      return energy(render(core));
    }
    const reference = generatedEnergy(false), shaped = generatedEnergy(true);
    assert.ok(reference > .001);
    assert.ok(shaped > reference * .95 && shaped < reference * 1.05,
      `the cloud duplicated or lost the filter ${kind} signal, route=${granular}`);
  }
}

console.log(`${variant}: Rack capture, independent/private clouds, selected OSC, frozen playback, filter ENV and waveshaper passed`);
