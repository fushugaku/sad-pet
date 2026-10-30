// Scene tempo reaches every engine, survives recall and drives the actual drum clock.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(await readFile(resolve(folder, "sad_pet.wasm")));
const core = new Core(module, 8000), utility = new Core(module, 8000);
const select = (id) => core.request("/api/timbres/select", { id });
const tempo = (bpm) => {
  const context = core.request("/api/timbres");
  for (const part of context.timbres) {
    if (part.kind === "empty") continue;
    select(part.id);
    assert.equal(core.request("/api/preset").params.bpm, bpm, `${part.name} has a different tempo`);
  }
};
core.request("/api/parameters", { bpm: 90 });
const drums = core.request("/api/timbres/add", { kind: "drums", classic: true }).active;
tempo(90);
select(drums);
const rootMix = (() => { select(0); const mix = core.request("/api/preset").params.mix; select(drums); return mix; })();
core.request("/api/parameters", { bpm: 150, mix: 0.25 });
tempo(150);
select(0);
assert.equal(core.request("/api/preset").params.mix, rootMix, "A local mix edit leaked to the scene");
const bus = core.request("/api/timbres/add", { kind: "bus" }).active;
tempo(150);
select(bus);
core.request("/api/parameters", { bpm: 75 });
tempo(75);
select(drums);
const patch = core.request("/api/preset");
patch.params.bpm = 200;
core.request("/api/preset", patch);
tempo(75);
assert.throws(() => core.request("/api/parameters", { bpm: 100, unknown_parameter: 0 }), /Unknown parameter/);
tempo(75);

// A loaded instrument's saved BPM must not replace the scene's tempo, even
// when it replaces the first timbre that stores that tempo.
select(0);
const instrument = core.json("sad_part_document");
instrument.preset.params.bpm = 200;
utility.call("sad_scene_load", core.call("sad_scene_pack"));
const prepared = utility.call("sad_instrument_prepare", { id: 0, name: "Imported", state: instrument });
core.call("sad_instrument_load", prepared);
tempo(75);
core.request("/api/timbres/remove", { id: 0 });
core.request("/api/timbres/add", { kind: "synth" });
tempo(75);

// Older scenes can have different per-timbre BPMs; their first part supplies
// the shared tempo on recall. The normalized value survives another round trip.
utility.call("sad_scene_load", core.call("sad_scene_pack"));
const scene = utility.json("sad_scene_document");
scene.preset.params.bpm = 105;
for (const part of scene.timbres) part.preset.params.bpm = 200;
core.call("sad_scene_load", utility.call("sad_scene_prepare", scene));
tempo(105);
core.call("sad_scene_load", core.call("sad_scene_pack"));
tempo(105);

// Measure real engine playback rather than only saved parameter values.
for (const [bpm, expected] of [[60, 7], [120, 15]]) {
  const audio = new Core(module, 8000), ids = [];
  for (let i = 0; i < 2; i++) {
    ids.push(audio.request("/api/timbres/add", { kind: "drums", classic: true }).active);
    audio.request("/api/drums/edit", [
      { track: 0, step: null, field: 0, value: 128 },
      { track: 0, step: null, field: 1, value: 6 },
    ]);
  }
  audio.request("/api/parameters", { bpm });
  audio.request("/api/transport", { playing: true, midi_gate: false });
  audio.process(3900);
  for (const id of ids) {
    audio.request("/api/timbres/select", { id });
    assert.equal(audio.request("/api/state").drums.positions[0], expected,
      `${bpm} BPM did not drive every drum clock`);
  }
}
console.log("Global tempo: all timbres, mixed edits, instruments, recall and real drum playback passed.");
