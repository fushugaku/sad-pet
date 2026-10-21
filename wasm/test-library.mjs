// Upgrade a remembered factory source without replacing or deleting user files.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { runInNewContext } from "node:vm";

const window = { fetch: async () => ({ ok: true, json: async () => [] }) };
runInNewContext(
  await readFile(new URL("../rust/web/library.js", import.meta.url), "utf8"),
  {
    window,
    document: { getElementById: () => ({}) },
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
  sceneLibrary.seed = async () => {};
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
}
