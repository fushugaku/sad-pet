// Actual shipping WASM, rendered into discarded buffers without Web Audio.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Core } from "../dist/web/core.mjs";
const module = await WebAssembly.compile(
  await readFile(new URL("../dist/web/sad_pet.wasm", import.meta.url)),
);
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const prepared = utility.call("sad_pack");
const preset = utility.request("/api/preset");
preset.color_chain = [];
preset.pre_color_chain = [];
preset.params.filter = 1;
preset.params.filter2 = 1;
preset.resynth.enabled = true;
preset.resynth.mix = 1;
const energy = (data) => data.reduce((sum, x) => sum + x * x, 0);
function render(p, rate, block, frames = 12000) {
  const core = new Core(module, rate);
  core.json("sad_load", prepared);
  core.request("/api/preset", p);
  core.request("/api/transport", { playing: true, midi_gate: false });
  const result = new Float32Array(frames * 2);
  for (let offset = 0; offset < frames; ) {
    const n = Math.min(block, frames - offset);
    result.set(core.process(n), offset * 2);
    offset += n;
  }
  assert.ok(result.every(Number.isFinite));
  return { core, result };
}
for (const wave of [
  "saw",
  "pulse",
  "triangle",
  "sine",
  "formant",
  "noise",
  "audio_in",
  "pcm",
]) {
  for (const mode of ["wave", "cross", "unison", "vpm"]) {
    if (
      ["formant", "noise", "audio_in", "pcm"].includes(wave) &&
      mode !== "wave"
    )
      continue;
    Object.assign(preset.resynth.oscillators[0], {
      waveform: wave,
      mode,
      control1: 0.4,
      control2: 0.3,
    });
    const { result } = render(preset, 48000, 128);
    assert.ok(energy(result) > 1e-8, `Silent ${wave}/${mode}`);
  }
}
preset.resynth.count = 4;
for (const fold_shape of [
  "triangle",
  "sine",
  "soft",
  "asymmetric",
  "rectify",
]) {
  preset.resynth.oscillators.forEach((o, i) =>
    Object.assign(o, {
      model: "wavefolder",
      waveform: ["saw", "pulse", "triangle", "sine"][i],
      fold_shape,
      control1: 0.4,
      control2: 0.5,
    }),
  );
  assert.ok(
    energy(render(preset, 48000, 128).result) > 1e-8,
    `Silent OSC4 ${fold_shape}`,
  );
}
preset.resynth.oscillators.forEach((o, i) =>
  Object.assign(o, {
    model: i === 0 ? "radias" : "wavefolder",
    waveform: ["saw", "pulse", "triangle", "sine"][i],
    fold_shape: ["triangle", "sine", "soft", "rectify"][i],
    interaction: ["off", "ring", "sync", "ring_sync"][i],
    interaction_source: [0, 2, 3, 1][i],
    mode: i === 0 ? "cross" : "wave",
    control1: 0.4,
    control2: 0.3,
    cross_source: 4,
    transpose: i * 7 - 12,
    pan: i / 3,
  }),
);
for (const rate of [44100, 48000, 96000]) {
  const { result, core } = render(preset, rate, 128);
  assert.deepEqual(
    result,
    render(preset, rate, 511).result,
    `Quantum-dependent Resynth at ${rate}`,
  );
  const memory = core.wasm.memory.buffer;
  const times = [];
  for (let i = 0; i < 256; i++) {
    const start = performance.now();
    core.process(128);
    times.push((performance.now() - start) * 1000);
  }
  assert.equal(core.wasm.memory.buffer, memory, "Audio grew WASM memory");
  core.request("/api/parameters", {
    resynth_osc4_transpose: 12,
    resynth_osc2_control1: 0.8,
    resynth_attack: 0.3,
    resynth_osc4_fold_shape: 3,
    resynth_osc2_interaction: 3,
    resynth_osc2_interaction_source: 1,
    resynth_osc1_cross_input: 2,
    resynth_track: 0.6,
    resynth_tracking: 1,
  });
  const state = core.request("/api/state");
  assert.equal(state.preset.resynth.oscillators[3].transpose, 12);
  assert.equal(state.preset.resynth.oscillators[3].fold_shape, "asymmetric");
  assert.equal(state.preset.resynth.oscillators[1].interaction, "ring_sync");
  assert.equal(state.preset.resynth.oscillators[1].interaction_source, 1);
  assert.equal(state.preset.resynth.oscillators[0].cross_input, "grains");
  assert.ok(Math.abs(state.preset.resynth.track - 0.6) < 1e-6);
  assert.equal(state.preset.resynth.tracking, "follow");
  assert.equal(state.resynth.bands.length, 16);
  assert.ok(state.resynth.bands.some((x) => x > 0));
  times.sort((a, b) => a - b);
  console.log(
    `Resynth WASM ${rate}: 4 layers, median ${times[128].toFixed(1)} us (${((times[128] / ((128 / rate) * 1e6)) * 100).toFixed(1)}% block budget), p99 ${times[253].toFixed(1)} us`,
  );
}
console.log(
  "Resynth checks passed: all 8 types, all analog modes, all 5 OSC4 Wavefolders in every slot, Ring/Sync, separate Source/Grains/oscillator taps, 16 envelopes, tracking, host parameters and deterministic quanta.",
);
