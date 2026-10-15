import assert from "node:assert/strict";
import { Microphone } from "./web/microphone.mjs";
import { BrowserBackend } from "./web/browser-backend.mjs";

function stream(id) {
  const track = {
    label: id,
    stopped: false,
    onended: null,
    getSettings: () => ({ deviceId: id }),
    stop() {
      this.stopped = true;
    },
  };
  return { getTracks: () => [track], getAudioTracks: () => [track], track };
}
function host() {
  const calls = [],
    nodes = [];
  const node = () => {
    const n = {
      disconnected: false,
      gain: {
        value: 1,
        setTargetAtTime(value) {
          this.value = value;
        },
      },
      connect(target) {
        return target;
      },
      disconnect() {
        this.disconnected = true;
      },
    };
    nodes.push(n);
    return n;
  };
  return {
    calls,
    nodes,
    node: node(),
    context: {
      currentTime: 0,
      createMediaStreamSource: node,
      createGain: node,
    },
    audio: {
      async call(...args) {
        calls.push(args);
      },
    },
  };
}
const backend = host(),
  first = stream("first"),
  second = stream("second");
let requests = 0;
const microphone = new Microphone(backend, {
  async getUserMedia(options) {
    assert.equal(options.video, false);
    assert.equal(options.audio.echoCancellation, false);
    requests++;
    return requests === 1 ? first : second;
  },
  async enumerateDevices() {
    return [
      { kind: "audioinput", deviceId: "first", label: "First microphone" },
    ];
  },
});
assert.equal(requests, 0, "never ask for microphone access on page load");
await microphone.configure({ active: true });
assert.equal(microphone.snapshot().active, true);
assert.equal(
  microphone.snapshot().monitor,
  false,
  "connecting input must not enable speaker monitoring",
);
assert.equal(microphone.snapshot().device, "first");
await microphone.configure({ gain: 0.4, mode: "grains" });
assert.equal(microphone.gainNode.gain.value, 0.4);
assert.equal(requests, 1, "mode and gain changes must reuse capture");
const oldNodes = backend.nodes.slice(1);
await microphone.configure({ active: true, device: "second" });
assert.ok(first.track.stopped);
assert.ok(oldNodes.every((n) => n.disconnected));
assert.equal(microphone.name, "second");
second.track.onended();
assert.equal(
  microphone.active,
  false,
  "disconnect must immediately release capture",
);
assert.ok(second.track.stopped);
await microphone.stop();
assert.ok(
  backend.calls.some(
    ([, data, , path]) => path === "/api/input" && data.active === false,
  ),
);

let complete;
const lateStream = stream("late");
const cancelled = new Microphone(host(), {
  getUserMedia: () =>
    new Promise((resolve) => {
      complete = resolve;
    }),
});
const pending = cancelled.start();
assert.equal(cancelled.pending, true);
await cancelled.stop();
complete(lateStream);
await pending;
assert.equal(cancelled.active, false);
assert.ok(
  lateStream.track.stopped,
  "late permission must not reopen a cancelled input",
);

const denied = new Microphone(host(), {
  async getUserMedia() {
    throw new DOMException("Denied", "NotAllowedError");
  },
});
await assert.rejects(denied.start(), /access was denied/);
assert.equal(denied.pending, false);
assert.equal(denied.active, false);
const stopping = Object.create(BrowserBackend.prototype),
  stopOrder = [];
Object.assign(stopping, {
  recording: { active: true, microphone: true },
  async finishRecording() {
    stopOrder.push("save");
    this.recording.active = false;
  },
  microphone: {
    async stop() {
      stopOrder.push("disconnect");
      return { active: false };
    },
  },
  updateMonitor() {
    stopOrder.push("monitor");
  },
});
await stopping.stopInput();
assert.deepEqual(stopOrder, ["save", "disconnect", "monitor"]);
const restoring = Object.create(BrowserBackend.prototype);
Object.assign(restoring, {
  state: { sample_name: "Microphone", input: { active: true } },
  microphone: { active: false, snapshot: () => ({ active: false }) },
  context: { state: "running" },
  recording: {},
});
assert.equal(
  restoring.snapshot().input.active,
  true,
  "Library must see a live source until the worklet confirms restoration of the original file",
);
const exporting = Object.create(BrowserBackend.prototype);
globalThis.location = { href: "http://127.0.0.1/" };
const freshRegion = { start: 0.3, end: 0.6 };
Object.assign(exporting, {
  ready: Promise.resolve(),
  microphone: { active: false },
  state: { region: { start: 0, end: 1 } },
  audio: {
    async call(type, data, transfers, path) {
      assert.equal(path, "/api/state");
      return { region: freshRegion, preset: {}, bypass: false, frozen: false };
    },
  },
  worker: {
    async call(type, data) {
      assert.equal(type, "render");
      assert.deepEqual(
        data.region,
        freshRegion,
        "render must use the acknowledged region instead of a delayed UI snapshot",
      );
      return new ArrayBuffer(0);
    },
  },
});
const exportResult = await exporting.fetch("/api/render", {
  body: JSON.stringify({ seconds: 1 }),
});
assert.equal(exportResult.status, 200, await exportResult.text());
console.log(
  "Microphone lifecycle passed: explicit access, device switching, gain/mode reuse, disconnect, cancellation and denied permission.",
);
