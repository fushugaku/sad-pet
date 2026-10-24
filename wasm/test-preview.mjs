import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { runInNewContext } from "node:vm";

const sources = [], errors = [], states = [], timers = new Map();
let timerId = 0, decode = async () => ({ duration: 20 });
class AudioContext {
  currentTime = 1;
  destination = {};
  async resume() {}
  decodeAudioData(data) { return decode(data); }
  createBufferSource() {
    const source = { connect() {}, disconnect() {}, start(...args) { this.started = args; }, stop() { this.stopped = true; } };
    sources.push(source);
    return source;
  }
  createGain() {
    return { connect() {}, disconnect() {}, gain: { setValueAtTime() {}, linearRampToValueAtTime() {}, cancelScheduledValues() {}, setTargetAtTime() {} } };
  }
}
const window = { AudioContext };
runInNewContext(await readFile(new URL("../rust/web/library.js", import.meta.url), "utf8"), {
  window, document: {},
  setTimeout: callback => { timers.set(++timerId, callback); return timerId; },
  clearTimeout: id => timers.delete(id),
});
const library = Object.create(window.EvilFileLibrary.prototype);
Object.assign(library, {
  currentId: "selected", catalog: [],
  store: { read: async (_, id) => ({ id, blob: { arrayBuffer: async () => id } }) },
  actions: { notify: error => errors.push(error), loadAudio: () => assert.fail("Preview selected a source") },
});
await library.preview("audio:first", state => states.push(state));
assert.deepEqual(states, ["loading", "playing"]);
assert.deepEqual(sources[0].started, [1, 0, 8], "Audition is bounded to eight seconds");
assert.equal(library.currentId, "selected");
await library.preview("audio:first", state => states.push(state));
assert.equal(sources[0].stopped, true);
assert.equal(library.previewId, null);
assert.equal(states.at(-1), "idle");

let finishDecode;
decode = () => new Promise(resolve => { finishDecode = resolve; });
const pending = library.preview("audio:slow", () => {});
while (!finishDecode) await Promise.resolve();
library.stopPreview();
finishDecode({ duration: 12 });
await pending;
assert.equal(sources.length, 1, "Closing the menu must cancel a pending decode");

decode = async () => ({ duration: 3 });
await library.preview("audio:short", () => {});
assert.deepEqual(sources[1].started, [1, 0, 3]);
sources[1].onended();
assert.equal(library.previewId, null);

decode = async () => { throw new Error("Invalid recording"); };
await library.preview("audio:broken", () => {});
assert.deepEqual(errors, ["Invalid recording"]);
assert.equal(library.previewId, null);
assert.equal(library.currentId, "selected");

window.sadPetPlugin = true;
library.catalog = [{ id: "native" }];
const requests = [];
window.sadPetFetch = async (_, options) => {
  requests.push(JSON.parse(options.body));
  return { ok: true, json: async () => ({ seconds: 8 }) };
};
await library.preview("factory:native:v1", () => {});
assert.equal(requests[0].id, "factory:native:v1");
library.stopPreview();
assert.equal(requests[1].id, undefined);
assert.ok(requests[1].generation > requests[0].generation);
assert.equal(timers.size, 0);
assert.equal(library.currentId, "selected");
console.log("Preview audio: isolated playback, toggle/auto-stop, cancellation, errors and native stop ordering passed.");
