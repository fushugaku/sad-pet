// Exercise the actual shared WASM engine, including scalar control messages.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Core } from "../dist/web/core.mjs";
const module = await WebAssembly.compile(
  await readFile(new URL("../dist/web/sad_pet.wasm", import.meta.url)),
);
const utility = new Core(module);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
const prepared = utility.call("sad_pack");
for (const rate of [44100, 48000, 96000]) {
  const core = new Core(module, rate);
  core.json("sad_load", prepared);
  const p = core.request("/api/preset");
  const defaults = core.request("/api/color-defaults");
  p.color_chain = [];
  p.pre_color_chain = [];
  p.source_color_chain = [];
  for (const [kind, rack, id] of [
    ["super_echo", "color_chain", 1],
    ["alpha_verb", "pre_color_chain", 2],
    ["alpha_verb", "color_chain", 3],
    ["super_echo", "pre_color_chain", 4],
  ]) {
    const slot = structuredClone(defaults.slot);
    slot.id = id;
    slot.kind = kind;
    slot.mix = 0.7;
    if (kind === "super_echo")
      Object.assign(slot.settings, {
        super_echo_time: 0.3,
        super_echo_feedback: 0.4,
        super_echo_sync: 1,
        super_echo_note: 0,
        super_echo_dirty: id === 4 ? 1 : 0,
      });
    p[rack].push(slot);
  }
  core.request("/api/preset", p);
  core.request("/api/transport", { playing: true, midi_gate: false });
  const memory = core.wasm.memory.buffer;
  let energy = 0;
  for (let block = 0; block < 64; block++) {
    const samples = core.process(128);
    assert.ok(samples.every(Number.isFinite));
    for (const v of samples) energy += v * v;
  }
  assert.ok(energy > 1e-6, `Silent imported effects at ${rate}`);
  assert.equal(core.wasm.memory.buffer, memory, "audio grew WASM memory");
  core.request("/api/parameters", {
    fx1_super_echo_feedback: 0.63,
    fx1_super_echo_dirty: 1,
    fx1_super_echo_note: 0.375,
    pre_fx1_alpha_verb_size: 0.72,
    pre_fx1_alpha_verb_dynamics: 1,
    fx2_alpha_verb_size: 0.34,
    pre_fx2_super_echo_feedback: 0.27,
  });
  for (let i = 0; i < 8; i++)
    assert.ok(core.process(128).every(Number.isFinite));
  assert.equal(core.wasm.memory.buffer, memory, "knob edit grew WASM memory");
  const restored = core.request("/api/preset");
  assert.equal(restored.color_chain[0].kind, "super_echo");
  assert.equal(restored.pre_color_chain[0].kind, "alpha_verb");
  assert.equal(restored.color_chain[1].kind, "alpha_verb");
  assert.equal(restored.pre_color_chain[1].kind, "super_echo");
  assert.ok(
    Math.abs(restored.color_chain[0].settings.super_echo_feedback - 0.63) <
      1e-6,
  );
  assert.ok(
    Math.abs(restored.pre_color_chain[0].settings.alpha_verb_size - 0.72) <
      1e-6,
  );
  assert.ok(
    Math.abs(restored.color_chain[1].settings.alpha_verb_size - 0.34) < 1e-6,
  );
  assert.ok(
    Math.abs(restored.pre_color_chain[1].settings.super_echo_feedback - 0.27) < 1e-6,
  );
  const catalog = JSON.stringify(core.request("/api/sequencer/catalog"));
  assert.ok(
    catalog.includes("fx1_super_echo_feedback") &&
      catalog.includes("pre_fx1_alpha_verb_size"),
  );
}
console.log(
  "Two SuperEcho + two AlphaVerb instances: 3 sample rates, simultaneous pre/post FX, independent controls, preset recall, Neuropolis and no audio memory growth passed.",
);
