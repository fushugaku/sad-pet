// The shipping DSP: Control routes add to the source's own sequencer and
// remain independent of the selected editor, scene recall and base edits.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { Core } from "../dist/web/core.mjs";
const module = await WebAssembly.compile(await readFile("dist/web/sad_pet.wasm"));
const core = new Core(module, 8000);
const catalog = core.request("/api/sequencer/catalog");
const target = (id, path, data) => core.request("/api/control/request", {
  timbre: id, path, ...(data === undefined ? {} : { data }),
});
const values = target(0, "/api/parameters/values", ["pitch", "pitch_range", "filter", "matrix1_depth", "macro1_depth1"]);
assert.ok(Number.isFinite(values.pitch));
assert.equal(values.pitch_range, 36);
assert.ok(Number.isFinite(values.filter));
assert.equal(values.matrix1_depth, 0);
assert.equal(values.macro1_depth1, 0);
assert.throws(() => target(0, "/api/parameters/values", ["unknown-midi-target"]), /Unknown parameter/);
const lane = (id, parameter, cv) => {
  const track = structuredClone(catalog.track);
  Object.assign(track, { id, target: { kind: "parameter", parameter }, absolute: false });
  track.config[1] = 1;
  track.steps.forEach(s => { s.cv = cv; });
  return track;
};
const root = target(0, "/api/preset");
root.params.filter = .3;
root.params.filter_env_depth = 0;
root.params.filter2_env_depth = 0;
root.matrix = []; root.macros = [];
for (const lfo of root.lfos) lfo.depth = lfo.depth2 = lfo.depth3 = 0;
for (const env of root.envs) env.extra_depth = env.depth3 = 0;
root.sequencer = { enabled: true, tracks: [lane(1, "filter", 500)] };
target(0, "/api/preset", root);
const control = core.request("/api/timbres/add", { kind: "control" }).active;
assert.equal(core.request("/api/timbres").timbres.find(t => t.id === control).kind, "control");
const preset = target(control, "/api/preset");
assert.deepEqual(preset.control.items, []);
preset.control.items.push({ id: "cutoff", timbre: 0, label: "Cutoff", selector: ".dial[data-control-key=filter]",
  parameter: "filter", module: false, view: {}, x: 180, y: 40, width: 128, height: 120 });
preset.sequencer = { enabled: true, tracks: [lane(1, "control:0:filter", 1250)] };
target(control, "/api/preset", preset);
core.request("/api/transport", { playing: true, midi_gate: false });
core.process(256);
let state = target(0, "/api/state");
const close = (value, expected, reason) => assert.ok(Math.abs(value - expected) < .003,
  `${reason}: ${value}, expected ${expected}`);
close(state.knobs.filter.current, .65, "Parent and Control must sum");
close(state.preset.params.filter, .3, "Modulation must preserve the base");
assert.equal(core.request("/api/timbres").active, control, "Addressed edits must not change the active tab");
target(0, "/api/parameters", { filter: .2 });
assert.ok(Math.abs(target(0, "/api/parameters/values", ["filter"]).filter - .2) < 1e-6);
assert.equal(core.request("/api/timbres").active, control, "Reading MIDI feedback selected its source editor");
core.process(128);
close(target(0, "/api/state").knobs.filter.current, .55, "A linked edit changes the source base");
const second = core.request("/api/timbres/add", { kind: "control" }).active;
const other = target(second, "/api/preset");
other.sequencer = { enabled: true, tracks: [lane(1, "control:0:filter", -500)] };
target(second, "/api/preset", other);
core.process(256);
close(target(0, "/api/state").knobs.filter.current, .45, "Multiple Controls sum independently");
target(control, "/api/sequencer/edit", [{ track: 1, stage: 0, field: 12, value: 500 }]);
core.process(256);
close(target(0, "/api/state").knobs.filter.current, .3, "Live step edits reach routed clocks");
core.request("/api/timbres/update", { id: second, muted: true });
core.process(128);
close(target(0, "/api/state").knobs.filter.current, .4, "Muting Control disables its routes");
// Opposite large offsets cancel before clamping, even at a range edge.
target(0, "/api/parameters", { filter: .7 });
target(control, "/api/sequencer/edit", [{ track: 1, stage: 0, field: 12, value: 4000 }]);
target(second, "/api/sequencer/edit", [{ track: 1, stage: 0, field: 12, value: -4000 }]);
core.request("/api/timbres/update", { id: second, muted: false });
core.process(256);
close(target(0, "/api/state").knobs.filter.current, .8, "Opposing Controls cancel before clamping");
target(0, "/api/parameters", { filter: .2 });
target(control, "/api/sequencer/edit", [{ track: 1, stage: 0, field: 12, value: 500 }]);
core.request("/api/timbres/update", { id: second, muted: true });
const packed = core.call("sad_scene_pack");
const restored = new Core(module, 8000);
restored.call("sad_scene_load", packed);
const restoredPreset = restored.request("/api/control/request", { timbre: control, path: "/api/preset" });
assert.deepEqual(restoredPreset.control.items, target(control, "/api/preset").control.items);
restored.request("/api/transport", { playing: true, midi_gate: false });
restored.process(256);
close(restored.request("/api/control/request", { timbre: 0, path: "/api/state" }).knobs.filter.current, .4,
  "Scene recall restores routing without opening the source");
core.request("/api/timbres/remove", { id: control });
core.process(128);
close(target(0, "/api/state").knobs.filter.current, .3, "Removing Control removes its modulation");
assert.throws(() => target(control, "/api/parameters", { filter: 1 }), /removed/);
// Transport commands and mute preserve agreement between the Control's
// visible playhead and the routed clock, including live restart.
const clock = new Core(module, 8000);
const cq = (id, path, data) => clock.request("/api/control/request", {
  timbre: id, path, ...(data === undefined ? {} : { data }),
});
const quiet = structuredClone(root);
quiet.params.filter = .2; quiet.sequencer.enabled = false;
cq(0, "/api/preset", quiet);
const cid = clock.request("/api/timbres/add", { kind: "control" }).active;
const cp = cq(cid, "/api/preset"), pattern = lane(1, "control:0:filter", 0);
pattern.config[1] = 4; pattern.config[3] = 1;
pattern.steps.slice(0, 4).forEach((s, i) => { s.cv = (i + 1) * 500; });
cp.sequencer = { enabled: true, tracks: [pattern] };
cq(cid, "/api/preset", cp);
clock.request("/api/transport", { playing: true, midi_gate: false });
for (let i = 0; i < 20; i++) clock.process(128);
assert.equal(cq(cid, "/api/state").sequencer.tracks[0].step, 2);
clock.request("/api/timbres/update", { id: cid, muted: true });
for (let i = 0; i < 8; i++) clock.process(128);
close(cq(0, "/api/state").knobs.filter.current, .2, "Mute silences the Control's contribution");
clock.request("/api/timbres/update", { id: cid, muted: false });
clock.process(64);
assert.equal(cq(cid, "/api/state").sequencer.tracks[0].step, 3);
close(cq(0, "/api/state").knobs.filter.current, .6, "Unmute keeps the visible playhead's phase");
cq(cid, "/api/sequencer/edit", [{ track: 0, stage: null, field: 1, value: 1 }]);
clock.process(64);
assert.equal(cq(cid, "/api/state").sequencer.tracks[0].step, 0);
close(cq(0, "/api/state").knobs.filter.current, .3, "Restart reaches the routed clock");
console.log("Control: linked edits, additive sequencers, mute, removal and scene recall pass.");
