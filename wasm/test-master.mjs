// The shipping mixer: hot floating-point buses, the final fader and limiter.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));

for (const rate of [8000, 44100, 48000]) {
  const core = new Core(module, rate);
  const drySlot = {
    ...core.request("/api/color-defaults").slot,
    id: 1, kind: "compressor", enabled: true, mix: 0,
  };
  for (let i = 0; i < 8; i++) {
    if (i) core.request("/api/timbres/add", { kind: "granular", classic: true });
    const p = core.request("/api/preset");
    p.sequencer.enabled = false;
    p.color_chain = []; p.pre_color_chain = []; p.source_color_chain = [];
    p.resynth.enabled = false;
    Object.assign(p.params, { master_volume: 1, in_gain: 0, out_gain: 0 });
    core.request("/api/preset", p);
    core.request("/api/input", { active: true, mode: "effects" });
    core.request("/api/transport", { bypass: true });
  }
  core.request("/api/transport", { playing: true, midi_gate: false });
  const context = core.request("/api/timbres");
  const master = context.timbres.find(t => t.kind === "master");
  core.request("/api/timbres/select", { id: master.id });
  const preset = core.request("/api/preset");
  assert.equal(preset.bus.limiter, true);
  preset.source_color_chain = [drySlot];
  core.request("/api/preset", preset);
  core.request("/api/timbres/update", { id: master.id, gain: 0.125 });
  const input = [new Float32Array(128), new Float32Array(128)];
  let sample = 0;
  const render = (scale) => {
    let peak = 0, error = 0;
    for (let offset = 0; offset < rate * 2; offset += 128) {
      for (let i = 0; i < 128; i++, sample++) {
        input[0][i] = 0.6 * Math.sin(2 * Math.PI * 125 * sample / rate);
        input[1][i] = -0.5 * input[0][i];
      }
      const out = core.process(128, input);
      assert.ok(out.every(Number.isFinite));
      if (preset.bus.limiter)
        assert.ok(out.every(x => Math.abs(x) <= 0.891251), "Master overshot its ceiling");
      if (offset < rate) continue; // Settle the transport and mixer ramps.
      for (let i = 0; i < 128; i++) {
        peak = Math.max(peak, Math.abs(out[i * 2]));
        if (scale !== undefined)
          error = Math.max(error, Math.abs(out[i * 2] - input[0][i] * scale));
        assert.ok(Math.abs(out[i * 2 + 1] + out[i * 2] * 0.5) < 1e-6, "Stereo balance changed");
      }
    }
    if (scale !== undefined)
      assert.ok(error / scale < 1e-4, `At ${rate}, mix waveform error ${error}`);
    return peak;
  };
  assert.ok(render(1) > 0.59, "Master FX clipped before the fader");
  assert.ok(core.request("/api/state").peak[0] < 0.61, "Meter reads before the final fader");
  core.request("/api/timbres/update", { id: master.id, gain: 2 });
  assert.ok(render() > 0.88);
  const meter = core.request("/api/state").master_limiter;
  assert.ok(meter.input_peak > 9.5 && meter.reduction_db > 20, JSON.stringify(meter));
  preset.bus.limiter = false;
  core.request("/api/preset", preset);
  assert.ok(render(16) > 9.5, "Limiter bypass did not preserve float output");

  // A processed intermediate bus must retain the same headroom.
  const added = core.request("/api/timbres/add", { kind: "bus" });
  const bus = added.timbres.find(t => t.kind === "bus");
  const busPreset = core.request("/api/preset");
  busPreset.source_color_chain = [drySlot];
  core.request("/api/preset", busPreset);
  for (const t of context.timbres.filter(t => t.kind === "granular")) {
    const sends = Array(8).fill(0); sends[bus.bus - 1] = 1;
    core.request("/api/timbres/update", { id: t.id, gain: 0, sends });
  }
  core.request("/api/timbres/select", { id: master.id });
  core.request("/api/timbres/update", { id: master.id, gain: 0.125 });
  assert.ok(render(1) > 0.59, "Bus FX clipped before reaching Master");

  const scene = core.call("sad_scene_pack");
  core.call("sad_scene_load", scene);
  assert.equal(core.request("/api/preset").bus.limiter, false, "Lost explicit bypass");
  const size = new DataView(scene).getUint32(8, true);
  const header = JSON.parse(new TextDecoder().decode(new Uint8Array(scene, 12, size)));
  delete header.parts.find(p => p.meta.kind === "master").preset.bus.limiter;
  const bytes = new TextEncoder().encode(JSON.stringify(header));
  const legacy = new Uint8Array(scene.byteLength - size + bytes.length);
  legacy.set(new Uint8Array(scene, 0, 8));
  new DataView(legacy.buffer).setUint32(8, bytes.length, true);
  legacy.set(bytes, 12);
  legacy.set(new Uint8Array(scene, 12 + size), 12 + bytes.length);
  core.call("sad_scene_load", legacy);
  assert.equal(core.request("/api/preset").bus.limiter, true, "Legacy scene lacks protection");
  console.log(`Master ${rate}: eight tracks, Bus/FX headroom, final fader/meter, stereo ceiling, bypass and scene recall passed.`);
}
