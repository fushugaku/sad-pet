// Vocal timbres in the shipping WASM: the worklet registry queues render
// jobs, the vocal worker ABI renders phrases, the clips play at the song clock
// through the timbre's chain, and scenes and presets keep the score.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core, VocalCore } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const module = await WebAssembly.compile(
  await readFile(resolve(folder, "sad_pet.wasm")),
);
const rate = 16000;
const utility = new Core(module, rate),
  core = new Core(module, rate),
  vocal = new VocalCore(module, rate);
utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
core.call("sad_load", utility.call("sad_pack"));

const jobs = () => core.json("sad_vocal_jobs");
// What the worklet does with the worker's updates (worklet.mjs vocalUpdate).
const deliver = () => {
  const updates = vocal.updates();
  const wasm = core.wasm;
  for (const update of updates) {
    const count = update.plan.length;
    const plan = new Float64Array(wasm.memory.buffer, wasm.sad_vocal_plan(count), count * 3);
    update.plan.forEach((entry, i) => plan.set(entry, i * 3));
    wasm.sad_vocal_plan_commit(update.id, count);
    for (const key of update.fresh) {
      const samples = new Float32Array(vocal.clip(key));
      const start = update.plan.find((p) => p[0] === key)[1];
      const pointer = wasm.sad_vocal_clip_begin(update.id, key, start, samples.length);
      new Float32Array(wasm.memory.buffer, pointer, samples.length).set(samples);
      wasm.sad_vocal_clip_commit();
    }
  }
  return updates;
};
const renderAll = () => {
  let steps = 0;
  while (vocal.step(0)) steps++;
  return steps;
};
const levels = (seconds) => {
  const blocks = Math.round((seconds * rate) / 128);
  const out = [];
  for (let i = 0; i < blocks; i++) {
    let peak = 0;
    for (const x of core.process(128)) {
      assert.ok(Number.isFinite(x));
      peak = Math.max(peak, Math.abs(x));
    }
    out.push(peak);
  }
  return out;
};
const max = (list) => list.reduce((m, v) => Math.max(m, v), 0);

// Silence the first timbre so only the voice is heard.
core.request("/api/timbres/update", { id: 0, gain: 0 });
let context = core.request("/api/timbres/add", { kind: "vocal", classic: true });
const part = context.timbres.find((t) => t.kind === "vocal");
assert.ok(part, "A Vocal timbre was added");
assert.equal(part.name, "Vocal 1");
const added = jobs();
assert.equal(added.length, 1, "Adding the timbre asks the worker to plan it");
assert.equal(added[0].id, part.id);
assert.equal(added[0].fresh, true);

core.request("/api/timbres/select", { id: part.id });
const preset = core.request("/api/preset");
assert.ok(preset.vocal && preset.bus && !preset.bus.master, "A Vocal preset is a voice on a bus");

// Two notes at 120 BPM: beats 1 and 2 (0.5 s and 1.0 s), then one at 2.5 s.
const score = {
  language: "ja",
  // No auto breath, so the time before the phrases is silent.
  style: { auto_breath: false },
  notes: [
    { id: 1, start: 480, length: 480, key: 64, lyric: "さ" },
    { id: 2, start: 960, length: 480, key: 67, lyric: "く" },
    { id: 3, start: 2400, length: 480, key: 69, lyric: "ら" },
  ],
};
const stored = core.request("/api/vocal/score", { score });
assert.equal(stored.score.notes.length, 3);
const edited = jobs();
assert.equal(edited.length, 1);
for (const job of edited) vocal.command(job);
assert.equal(jobs().length, 0, "An unchanged score is not sent again");
assert.equal(renderAll(), 2, "Two phrases rendered");
const updates = deliver();
assert.equal(updates[0].plan.length, 2);
assert.equal(core.request("/api/vocal/state").clips, 2);

// Stopped: silence. Playing from 0 s: the voice where the notes are.
assert.ok(max(levels(0.5)) < 1e-6, "A stopped song is silent");
core.request("/api/vocal/transport", { start: 0, playing: true });
const sung = levels(3.2);
const at = (s) => max(sung.slice(Math.floor((s * rate) / 128), Math.ceil(((s + 0.2) * rate) / 128)));
assert.ok(at(0.0) < 1e-4, `Silent before the first phrase: ${at(0.0)}`);
assert.ok(at(0.6) > 0.01, `The first note sings: ${at(0.6)}`);
assert.ok(at(1.1) > 0.01, `The second note sings: ${at(1.1)}`);
assert.ok(at(2.0) < 1e-3, `The rest is quiet: ${at(2.0)}`);
assert.ok(at(2.6) > 0.01, `The last phrase sings: ${at(2.6)}`);
const state = core.request("/api/vocal/state");
// Playback from 0 began 0.1 s early, for the first note's lead-in.
assert.ok(state.playing && Math.abs(state.position - 3.1) < 0.02, JSON.stringify(state));
core.request("/api/transport", { playing: false });
assert.equal(core.request("/api/vocal/state").playing, false, "The global Stop stops the song");

// An edit re-renders only its phrase; the worklet keeps the other clip.
score.notes[2].key = 72;
core.request("/api/vocal/score", { score });
for (const job of jobs()) vocal.command(job);
assert.equal(renderAll(), 1, "Only the edited phrase renders again");
const update = deliver()[0];
assert.equal(update.fresh.length, 1);
assert.equal(core.request("/api/vocal/state").clips, 2);

// A loop of 1440 ticks (1.5 s): the third note lies past it, and the song
// starts over, the position wrapping with it.
score.loop_ticks = 1440;
core.request("/api/vocal/score", { score });
for (const job of jobs()) vocal.command(job);
renderAll();
assert.equal(deliver()[0].plan.length, 1, "Only the phrase inside the loop is planned");
core.request("/api/vocal/transport", { start: 0, playing: true });
const looped = levels(3.2);
const loopAt = (s) => max(looped.slice(Math.floor((s * rate) / 128), Math.ceil(((s + 0.2) * rate) / 128)));
assert.ok(loopAt(2.1) > 0.01, `The first note sings again: ${loopAt(2.1)}`);
assert.ok(loopAt(2.6) > 0.01, `The second note sings again: ${loopAt(2.6)}`);
// The first note (0.5 s) leads in 0.6 s before it, so playback from 0
// starts 0.1 s early for the first pass to have its consonant too.
const wrapped = core.request("/api/vocal/state").position;
assert.ok(Math.abs(wrapped - (3.2 - 0.1 - 1.5 * 2)) < 0.02, `The position wraps: ${wrapped}`);
core.request("/api/transport", { playing: false });
delete score.loop_ticks;
core.request("/api/vocal/score", { score });
for (const job of jobs()) vocal.command(job);
renderAll();
deliver();

// The voices that come with SAD PET: listed for the browser, and they load.
{
  const packs = JSON.parse(await readFile(resolve(folder, "voices/packs.json"), "utf8"));
  assert.deepEqual(packs.map((p) => [p.id, p.language]), [["kasane-teto", "ja"], ["kasane-teto-en", "en"], ["kumi-ru", "ru"]]);
  const bytes = await readFile(resolve(folder, "voices", packs[0].file));
  assert.equal(bytes.length, packs[0].size);
  vocal.addVoice("kasane-teto", bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.length));
  const listed = vocal.command({ type: "voices" });
  assert.ok(listed.some((v) => v.id === "kasane-teto" && v.units > 1000), JSON.stringify(listed));

  // The editor learns which units sing each note, and a note can ask for one.
  core.request("/api/vocal/score", { score: { ...score, voice: "kasane-teto" } });
  for (const job of jobs()) vocal.command(job);
  let units = vocal.command({ type: "units", id: part.id });
  assert.ok(units["1"]?.used.length && units["1"].options.length > 1, JSON.stringify(units));
  const other = units["1"].options.find((o) => !units["1"].used.includes(o));
  const asked = structuredClone(score);
  asked.voice = "kasane-teto";
  asked.notes[0].unit = other;
  core.request("/api/vocal/score", { score: asked });
  for (const job of jobs()) vocal.command(job);
  units = vocal.command({ type: "units", id: part.id });
  assert.ok(units["1"].used.includes(other), `${other} sings the note: ${JSON.stringify(units["1"])}`);
  core.request("/api/vocal/score", { score });
  for (const job of jobs()) vocal.command(job);
  renderAll();
  deliver();
}

// A loaded preset keeps the role and the score; Granular and Filters work.
core.request("/api/preset", utility.request("/api/preset"));
const kept = core.request("/api/preset");
assert.equal(kept.vocal.score.notes.length, 3, "The score survives a preset load");
assert.equal(core.request("/api/timbres").timbres.find((t) => t.id === part.id).kind, "vocal");
kept.bus.granular = true;
kept.bus.filters = true;
core.request("/api/preset", kept);
assert.equal(jobs().length, 0, "Effects do not re-render the voice");
core.request("/api/vocal/transport", { start: 0.5, playing: true });
assert.ok(max(levels(0.5)) > 0.001, "The voice passes a Granular, filtered chain");
core.request("/api/vocal/transport", { playing: false });

// Scenes keep the score; the recalled part asks for its clips again.
const packed = core.call("sad_scene_pack");
const recalled = new Core(module, rate);
recalled.call("sad_load", utility.call("sad_pack"));
recalled.call("sad_scene_load", packed);
const recalledJobs = recalled.json("sad_vocal_jobs");
assert.equal(recalledJobs.length, 1);
assert.equal(recalledJobs[0].fresh, true);
assert.equal(recalledJobs[0].settings.score.notes[2].key, 72);

// Render WAV: the utility worker renders the scene with the vocal worker's clips.
{
  const packed = core.call("sad_scene_pack");
  const worker = new Core(module, rate);
  worker.call("sad_scene_adopt", packed);
  const scene = worker.json("sad_scene_document");
  const songs = vocal.json("sad_vocal_songs");
  assert.equal(songs.length, 1);
  for (const song of songs)
    for (const [key, start] of song.clips) {
      const samples = new Float32Array(vocal.clip(key));
      const pointer = worker.wasm.sad_render_clip(song.id, start, samples.length);
      new Float32Array(worker.wasm.memory.buffer, pointer, samples.length).set(samples);
    }
  const wav = new DataView(worker.call("sad_scene_render_document", { seconds: 3, scene }));
  let at = 12;
  while (at + 8 <= wav.byteLength && String.fromCharCode(...new Uint8Array(wav.buffer, at, 4)) !== "data")
    at += 8 + wav.getUint32(at + 4, true);
  const end = at + 8 + wav.getUint32(at + 4, true);
  let peak = 0;
  for (let i = at + 8; i + 4 <= end; i += 4) peak = Math.max(peak, Math.abs(wav.getFloat32(i, true)));
  assert.ok(peak < 1.5, `A float WAV: ${peak}`);
  assert.ok(peak > 0.01, `The rendered WAV contains the voice: ${peak}`);
}

// Removing the timbre tells the worker to forget it.
core.request("/api/timbres/remove", { id: part.id });
assert.deepEqual(jobs(), [{ type: "forget", id: part.id }]);

// The worker reads lyrics into phonemes for the editor.
assert.deepEqual(
  vocal.command({ type: "phonemes", language: "ru", lyrics: ["при-", "вет", "-"] }),
  ["p r' i", "v' e t", "-"],
);
console.log("vocal timbres ok");
