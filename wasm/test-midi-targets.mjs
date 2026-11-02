import assert from "node:assert/strict";
import vm from "node:vm";
import { readFile } from "node:fs/promises";

const source = await readFile(new URL("../rust/web/midi-targets.js", import.meta.url), "utf8");
let now = 1000;
const context = { active: 0, timbres: [
  { id: 0, kind: "rack", name: "Grains" }, { id: 1, kind: "rack", name: "Second" },
  { id: 2, kind: "sampler", name: "Sampler" },
] };
const params = new Map([[0, { pitch: .5, pitch_range: 36, filter: .2 }], [1, { pitch: .5, pitch_range: 12, filter: .7 }]]);
const sampler = { sampler: { pads: [{ id: "pad-a", tune: 0, gain: 0 }, { id: "pad-b", tune: 12, gain: -6 }] } };
const writes = [], reads = [], errors = [];
const sandbox = vm.createContext({ window: { sadPetAudio: {} }, setTimeout, clearTimeout,
  Date: { now: () => now }, console });
vm.runInContext(source, sandbox);
const midi = sandbox.window.SadMidiTargets;
midi.setup({ context: () => context, flush: async () => {}, dirty() {}, notify: m => errors.push(m),
  async request(_, data) {
    assert.equal(context.active, 1, "MIDI selected its source editor");
    if (data.path === "/api/preset") {
      if (data.data) { Object.assign(sampler, structuredClone(data.data)); writes.push(data); }
      return structuredClone(sampler);
    }
    if (data.path === "/api/parameters/values") {
      reads.push(data);
      return Object.fromEntries(data.data.map(id => [id, params.get(data.timbre)[id]]));
    }
    assert.equal(data.path, "/api/parameters");
    writes.push(structuredClone(data)); Object.assign(params.get(data.timbre), data.data);
    return { ok: true };
  },
});
const target = (timbre, parameter, extra = {}) => midi.describe({ timbre,
  kind: context.timbres.find(t => t.id === timbre).kind, key: parameter, parameter,
  label: parameter, min: 0, max: 1, ...extra });
const pitch = target(0, "pitch"), cutoff = target(0, "filter"), other = target(1, "pitch");
assert.equal(midi.active, false);
context.active = 1;
// A burst while another instrument is open changes only its addressed source.
await Promise.all(Array.from({ length: 100 }, () => midi.command(pitch, "relative", 65)));
await midi.flush();
assert.ok(Math.abs((params.get(0).pitch * 2 - 1) * 36 - 10) < 1e-8);
assert.equal(params.get(1).pitch, .5);
assert.equal(context.active, 1);
assert.ok(writes.length <= 2, "relative bursts were not coalesced");
assert.equal(midi.active, true, "linked views could not detect the MIDI gesture");
now += 500;
assert.equal(midi.active, false, "the editor stayed in rapid polling after MIDI stopped");
await midi.command(other, "relative", 65); await midi.flush();
assert.ok(Math.abs((params.get(1).pitch * 2 - 1) * 12 - .1) < 1e-8);
await midi.command(other, "relative", 65, 10); await midi.flush();
assert.ok(Math.abs((params.get(1).pitch * 2 - 1) * 12 - 1.1) < 1e-8, "Shift did not give a 1 st pitch step");
await midi.command(other, "relative", 63); await midi.flush();
assert.ok(Math.abs((params.get(1).pitch * 2 - 1) * 12 - 1) < 1e-8, "releasing Shift lost the fine step grid");
await midi.command(cutoff, "relative", 72, 10); await midi.flush();
assert.ok(Math.abs(params.get(0).filter - (.2 + 80 / 1024)) < 1e-8, "coarse continuous rotation clipped the MIDI delta");
await midi.command(cutoff, "14bit", 8193); await midi.flush();
assert.equal(params.get(0).filter, 8193 / 16383);
await midi.command(pitch, "absolute", 127); await midi.flush();
await midi.command(pitch, "relative", 65); await midi.flush();
assert.equal(params.get(0).pitch, 1, "a relative encoder exceeded the maximum");
await midi.command(pitch, "absolute", 0); await midi.flush();
await midi.command(pitch, "relative", 63); await midi.flush();
assert.equal(params.get(0).pitch, 0);

// Feedback reads one compact snapshot per instrument, follows external edits,
// and reports the physical range independently of the currently open tab.
now += 100;
params.get(0).pitch = .5;
reads.length = 0;
await midi.refresh([pitch, cutoff, other]);
assert.equal(reads.length, 2);
assert.equal(midi.value(pitch).unit, .5);
assert.equal(midi.value(pitch).text, "0.0 st");
assert.equal(midi.value(pitch).instrument, "Grains");

// Sampler controls retain the selected pad's identity after a pad or editor
// change. Whole-preset edits merge the latest settings, keeping other pads.
const pad = target(2, "preset:pad-a:tune", { min: -48, max: 48, step: 1,
  path: ["sampler", "pads", 0, "tune"], padId: "pad-a" });
await midi.command(pad, "relative", 65); await midi.flush();
assert.equal(sampler.sampler.pads[0].tune, 1);
await midi.command(pad, "relative", 65, 10); await midi.flush();
assert.equal(sampler.sampler.pads[0].tune, 11);
sampler.sampler.pads.reverse(); now += 100;
await midi.command(pad, "relative", 65); await midi.flush();
assert.equal(sampler.sampler.pads.find(p => p.id === "pad-a").tune, 12);
assert.equal(sampler.sampler.pads.find(p => p.id === "pad-b").tune, 12);
assert.equal(sampler.sampler.pads.find(p => p.id === "pad-b").gain, -6);

// Logarithmic controls keep the same travel and endpoint behaviour offscreen.
params.get(0).frequency = 20;
const frequency = target(0, "frequency", { min: 20, max: 20000, scale: { type: "log", min: 20, max: 20000 } });
await midi.command(frequency, "14bit", 8192); await midi.flush();
assert.ok(Math.abs(params.get(0).frequency - 20 * 1000 ** (8192 / 16383)) < 1e-6);
assert.ok(Math.abs(midi.value(frequency).unit - 8192 / 16383) < 1e-10);

// Deletion never redirects an old mapping to the next instrument or pad.
context.timbres = context.timbres.filter(t => t.id !== 0);
const count = writes.length;
await midi.command(pitch, "relative", 65); await midi.flush();
assert.equal(writes.length, count);
assert.equal(midi.value(pitch), null);
sampler.sampler.pads = sampler.sampler.pads.filter(p => p.id !== "pad-a"); now += 100;
await midi.refresh([pad]);
assert.equal(midi.value(pad), null);
assert.deepEqual(errors, []);
console.log("MIDI targets passed: offscreen instruments, burst precision, ranges, compact feedback, stable sampler pads, logarithmic scales and removed targets.");
