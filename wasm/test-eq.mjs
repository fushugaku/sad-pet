// A Rack's EQ modules in the shipping WASM, in discarded buffers without an
// audio device; then the page's curve, which must be the engine's.
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
core.request("/api/timbres/add", { kind: "rack" });
let state = core.request("/api/state");

const peak = (blocks) => {
  let p = 0;
  for (let i = 0; i < blocks; i++)
    for (const x of core.process(128)) {
      assert.ok(Number.isFinite(x));
      p = Math.max(p, Math.abs(x));
    }
  return p;
};
// A Synth into two EQs, with every band off: the EQs pass it as it is.
const flat = (id) => ({
  id,
  bands: Array.from({ length: 8 }, () => ({ enabled: false })),
  output: 0,
});
core.request("/api/preset", {
  ...state.preset,
  color_chain: [],
  rack: {
    ...state.preset.rack,
    modules: [
      { module: "synth", enabled: true },
      { module: "eq", enabled: true, id: 2 },
      { module: "eq", enabled: true },
    ],
    equalizers: [flat(2)],
  },
});
let preset = core.request("/api/preset");
assert.deepEqual(preset.rack.modules.map((m) => [m.module, m.id ?? 0]), [["synth", 0], ["eq", 2], ["eq", 1]],
  "an EQ without a number takes the first free one");
assert.deepEqual(preset.rack.equalizers.map((eq) => eq.id), [1, 2]);
assert.equal(preset.rack.equalizers[0].bands[1].kind, "low_shelf", "a new EQ starts from the defaults");
core.request("/api/transport", { playing: true });
core.request("/api/note", { kind: "note_on", channel: 0, note: 57, velocity: 110 });
peak(40);
const open = peak(40);
assert.ok(open > 0.01, "the synth plays through both EQs");
// The output gain is a host parameter: −12 dB on each EQ is −24 dB in all.
core.request("/api/parameters", { rack_eq1_output: -12, rack_eq2_output: -12 });
peak(20);
const down = peak(40);
assert.ok(Math.abs(20 * Math.log10(down / open) + 24) < 0.5, `${down} / ${open}`);
assert.equal(core.request("/api/preset").rack.equalizers[1].output, -12);

// An LFO on a band's gain: the knob reports where modulation has it.
const catalog = core.request("/api/sequencer/catalog");
const spec = catalog.modulation.find((s) => s.id === "rack_eq1_band3_gain");
assert.ok(spec, "EQ bands can be modulated");
assert.ok(catalog.parameters.some((p) => p.id === "rack_eq1_band3_frequency"), "and sequenced");
preset = core.request("/api/preset");
preset.lfos[0] = { ...preset.lfos[0], destination: { control: { index: spec.index } }, depth: 0.5, rate_hz: 3 };
core.request("/api/preset", preset);
peak(30);
state = core.request("/api/state");
const knob = state.knobs.rack_eq1_band3_gain;
assert.ok(knob?.modulated, JSON.stringify(knob));
assert.ok(knob.min < -5 && knob.max > 5, "the knob shows the LFO's reach");
assert.ok(!("rack_eq3_output" in state.knobs), "only the rack's EQs report");
// The page's telemetry worker reads the EQs from the audio thread's
// snapshot: a set band reports where it is, not where a new EQ starts.
{
  const packet = new Uint8Array(core.wasm.sad_snapshot_capacity());
  const size = core.snapshot(packet);
  assert.ok(size > 0);
  const { knobs } = utility.json("sad_snapshot_import", packet.slice(0, size)).state;
  assert.deepEqual([knobs.rack_eq2_output.base, knobs.rack_eq2_output.current], [-12, -12]);
  assert.equal(knobs.rack_eq2_output.modulated, false);
  assert.ok(knobs.rack_eq1_band3_gain.modulated, "and the LFO moves the band");
}
core.request("/api/note", { kind: "note_off", channel: 0, note: 57, velocity: 0 });

// The page's curve is the engine's response (dsp::eq::band_response).
const context = vm.createContext({ window: {}, document: {}, Node: {} });
vm.runInContext(await readFile(new URL("../rust/web/eq.js", import.meta.url), "utf8"), context);
vm.runInContext(await readFile(new URL("../rust/web/rack.js", import.meta.url), "utf8"), context);
const { responses, defaults, unitFrequency, frequencyOf } = context.window.SadEq;
const probes = [30, 100, 440, 1000, 3000, 9000, 16000, 21000];
for (const [band, rate, expected] of [
  [{ kind: "bell", frequency: 1000, gain: 6, q: 2 }, 48000,
    [0.001464, 0.016517, 0.442657, 6.0, 0.216678, 0.018788, 0.005168, 0.003102]],
  [{ kind: "bell", frequency: 15000, gain: 12, q: 0.7 }, 44100,
    [0.00021, 0.002332, 0.044971, 0.228269, 1.773126, 8.58557, 11.638782, 8.992288]],
  [{ kind: "low_shelf", frequency: 120, gain: -9, q: 0.71 }, 48000,
    [-8.963174, -5.968403, -0.053109, -0.001118, 0.000094, 0.000012, 0.000003, 0.000002]],
  [{ kind: "high_shelf", frequency: 9000, gain: 7.5, q: 1.4 }, 48000,
    [-0.000051, -0.000567, -0.010973, -0.056628, -0.496536, 3.75, 8.761259, 8.375829]],
  [{ kind: "low_cut", frequency: 80, q: 0.71, slope: 18 }, 48000,
    [-25.564499, -0.981584, 0.001081, 0.000257, 0.000056, 0.000034, 0.000032, 0.000032]],
  [{ kind: "high_cut", frequency: 6000, q: 3, slope: 48 }, 96000,
    [0.000016, 0.000173, 0.00339, 0.018292, 0.267868, -27.371147, -68.00515, -86.867844]],
  [{ kind: "notch", frequency: 50, q: 6 }, 48000,
    [-0.104755, -0.053288, -0.001599, -0.000303, -0.000034, -0.000004, -0.000001, -0.000001]],
  [{ kind: "band_pass", frequency: 2500, q: 0.5 }, 48000,
    [-32.366653, -21.921761, -9.304687, -3.201859, -0.15687, -6.094279, -11.201283, -13.325713]],
]) {
  const full = { enabled: true, gain: 0, slope: 24, placement: "stereo", ...band };
  const { each, total } = responses([full], rate, probes);
  probes.forEach((f, i) => {
    assert.ok(Math.abs(each[0][i] - expected[i]) < 2e-3, `${band.kind} at ${f} Hz: ${each[0][i]} vs ${expected[i]}`);
    assert.equal(total[i], each[0][i]);
  });
}
// A fresh EQ is flat; bands off add nothing.
const fresh = responses(defaults(1).bands, 48000, probes);
assert.ok([...fresh.total].every((db) => Math.abs(db) < 1e-6), String([...fresh.total]));
assert.ok(Math.abs(frequencyOf(unitFrequency(632.46)) - 632.46) < 1e-9);
assert.ok(Math.abs(frequencyOf(0.5) - Math.sqrt(20 * 20000)) < 1e-9);

// The rack's plan keys each EQ by its number, in any number.
const plan = (modules) => JSON.parse(JSON.stringify(context.window.SadRack.plan(modules)));
const steps = plan([
  { module: "drums", enabled: true },
  { module: "eq", enabled: true, id: 1 },
  { module: "synth", enabled: true },
  { module: "eq", enabled: true, id: 3 },
  { module: "mixer", enabled: true },
  { module: "eq", enabled: false, id: 2 },
]);
assert.deepEqual(steps.map((s) => s.key), ["drums", "eq1", "synth", "eq3", "mixer", "eq2"]);
assert.deepEqual(steps.map((s) => s.chain), ["drums", "drums", "synth", "synth", "mix", "mix"]);
assert.deepEqual(steps.map((s) => s.perNote), [false, false, true, false, false, false]);
console.log("ok");
