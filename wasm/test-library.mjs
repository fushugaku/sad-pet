// Upgrade a remembered factory source without replacing or deleting user files.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { runInNewContext } from "node:vm";

const window = {};
runInNewContext(
  await readFile(new URL("../rust/web/library.js", import.meta.url), "utf8"),
  { window },
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
