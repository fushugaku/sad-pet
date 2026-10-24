// Upgrade a remembered factory source without replacing or deleting user files.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { runInNewContext } from "node:vm";

const nodes = new Map();
const byId = id => {
  if (!nodes.has(id)) nodes.set(id, {});
  return nodes.get(id);
};
const window = {
  fetch: async () => ({ ok: true, json: async () => [] }),
  SadLazySelect: {
    configure: (anchor, value, groups) => Object.assign(anchor, { value, groups }),
    setValue: (anchor, value) => { anchor.value = value; },
  },
};
runInNewContext(
  await readFile(new URL("../rust/web/library.js", import.meta.url), "utf8"),
  {
    window,
    document: { getElementById: byId, querySelectorAll: () => [] },
  },
);
const library = Object.create(window.EvilFileLibrary.prototype);
const cover = { id: "harvest-dawn-loop", name: "Harvest Dawn · harp loop" };
const shortCover = { id: "factory:harvest-dawn-cover:v1", blob: "saved eight-second cover" };
const original = { id: "factory:harvest-dawn-excerpt:v1", blob: "saved original" };
const imported = { id: "audio:user-recording", blob: "user audio" };
const files = new Map([original, shortCover, imported].map((file) => [file.id, file]));
library.catalog = [cover];
library.store = {
  read: async (store, id) => {
    assert.equal(store, "files");
    return files.get(id);
  },
};
library.ensureFactory = async (track) => {
  assert.equal(track, cover);
  return { id: "factory:harvest-dawn-loop:v1", blob: "new loop" };
};
const restored = await library.previousAudio({ id: original.id });
assert.equal(restored.id, "factory:harvest-dawn-loop:v1");
assert.equal((await library.previousAudio({ id: shortCover.id })).id, "factory:harvest-dawn-loop:v1");
assert.equal(files.get(shortCover.id), shortCover, "keep earlier renders without reusing their audio cache");
assert.equal(files.get(original.id), original, "keep the user's saved soundtrack file");
assert.equal(await library.previousAudio({ id: imported.id }), imported);
assert.equal(await library.previousAudio(null), undefined);
// The same frontend can be used with an older native bank that has no cover.
library.catalog = [];
assert.equal(await library.previousAudio({ id: original.id }), original);
console.log("Library upgrade passed: soundtrack and short-cover selections move to the new loop; saved audio is preserved.");

// The scene's fades belong to its sound, rather than the sample library's last edit.
{
  const sceneLibrary = Object.create(window.EvilFileLibrary.prototype);
  let restoredFades = 0;
  sceneLibrary.actions = { presets: true, loadFades: async () => restoredFades++ };
  sceneLibrary.store = { open: async () => {}, setting: async () => null };
  sceneLibrary.refresh = async () => {};
  sceneLibrary.rememberNative = async () => {};
  sceneLibrary.render = () => {};
  const native = {
    instance_id: "fresh-browser",
    fades: { fade_in: 1, fade_out: 1 },
    timbres: { timbres: [{ id: 0 }] },
  };
  await sceneLibrary.initialize(native, []);
  assert.equal(restoredFades, 0, "Library initialization replaced the scene fades");
  await sceneLibrary.initialize({ ...native, timbres: undefined }, []);
  assert.equal(restoredFades, 1, "Legacy desktop sample fades no longer restore");

  const last = { id: "audio:previous", instance: "previous-session" };
  const audio = { id: last.id, kind: "audio" };
  sceneLibrary.store.setting = async () => last;
  sceneLibrary.previousAudio = async () => audio;
  let activated;
  sceneLibrary.activate = async (record) => { activated = record; };
  await sceneLibrary.initialize({
    ...native,
    sample_id: "factory:dusty-keys:v1",
    startup_preset_index: 5,
    preset: { name: "glasses5" },
  }, Array.from({ length: 6 }, (_, i) => ({ name: i === 5 ? "glasses5" : "Init" })));
  assert.equal(activated, audio, "The new initial scene prevented the remembered audio from restoring");

  const drumState = {
    ...native, plugin: { effect: false }, preset: { drums: {} },
    input: { active: false }, sample_name: "Drum machine", sample_id: null,
    file_source: { sample_name: "Dusty keys", sample_id: "factory:dusty-keys:v1", sample_seconds: 12 },
  };
  let remembered;
  sceneLibrary.rememberNative = async source => { remembered = source; };
  await sceneLibrary.initialize(drumState, []);
  assert.equal(remembered.sample_name, "Dusty keys",
    "Opening a drum project treated the live waveform as a stored source");
  let synchronized = 0;
  sceneLibrary.synchronizeNative = async () => synchronized++;
  sceneLibrary.observe(drumState);
  assert.equal(synchronized, 0, "Drum telemetry replaced the remembered file selection");
}

// The complete catalog appears before audio is generated. Cached originals,
// user imports and the current trashed source keep their identities.
{
  const bank = Object.create(window.EvilFileLibrary.prototype);
  const catalog = [
    { id: "beat", name: "Beat", category: "Drums", bpm: 120, seconds: 8 },
    { id: "hum", name: "Hum", category: "Vocals", bpm: 80, seconds: 12 },
  ];
  const cached = { id: "factory:beat:v1", kind: "audio", factory: true, displayName: "Beat", blob: "cached WAV", size: 100 };
  const user = { id: "audio:user", kind: "audio", name: "My loop.wav", addedAt: 2, size: 30 };
  const trash = { id: "audio:trash", kind: "audio", name: "Old take.wav", addedAt: 1, trashedAt: 5, size: 20 };
  const saved = [cached, user, trash];
  const generated = [], downloads = [];
  Object.assign(bank, {
    catalog, currentId: cached.id, actions: { download: (...args) => downloads.push(args) },
    store: { read: async (_, id) => id ? saved.find(f => f.id === id) : saved },
    render() {},
    ensureFactory: async track => {
      generated.push(track.id);
      return { blob: `${track.id} WAV`, name: `${track.id}.wav` };
    },
  });
  await bank.refresh();
  assert.equal(bank.files.length, 4, "Uncached sources must appear alongside saved files");
  assert.equal(bank.files[0].blob, "cached WAV", "Do not regenerate existing audio");
  assert.equal(generated.length, 0, "Opening the catalog must not render 158 files");
  const choices = bank.sourceGroups().filter(g => g.choices.length);
  assert.deepEqual(Array.from(choices, g => g.label), ["Drums", "Vocals", "Your audio"]);
  assert.deepEqual(Array.from(choices[1].choices[0]), ["factory:hum:v1", "Hum · 80 BPM", "Hum"]);
  assert.equal(choices[2].choices.length, 1, "Trash stays out of the source picker");
  bank.currentId = trash.id;
  assert.equal(bank.sourceGroups().at(-1).choices.length, 2, "A playing trashed source remains selectable");
  await bank.download("factory:hum:v1");
  assert.deepEqual(generated, ["hum"]);
  assert.deepEqual(downloads, [["hum WAV", "hum.wav"]]);

  bank.currentId = cached.id;
  bank.ready = true;
  bank.activateId = async () => { throw new Error("Preparation failed"); };
  let error;
  bank.error = e => { error = e.message; };
  bank.bind();
  const select = byId("sample-select");
  select.value = "factory:hum:v1";
  await select.onchange({ target: select });
  assert.equal(error, "Preparation failed");
  assert.equal(select.value, cached.id, "Failed preparation must restore the current source");
  assert.equal(select.disabled, false);

  runInNewContext(await readFile(new URL("../plugin/web/native-library.js", import.meta.url), "utf8"), {
    window, document: { getElementById: byId },
  });
  // The native subclass replaces the shared constructor at the end of its script.
  const native = Object.create(window.EvilFileLibrary.prototype);
  Object.assign(native, { catalog, files: bank.files, currentId: "preset:embedded", currentName: "Embedded phrase" });
  native.renderSources();
  const current = select.groups.find(g => g.label === "Current");
  assert.deepEqual(Array.from(current.choices[0]), ["preset:embedded", "Embedded phrase"]);
  assert.equal(select.value, "preset:embedded");
}
console.log("Source library: categorized choices, lazy audio, downloads, failed-load recovery and native preset audio passed.");
