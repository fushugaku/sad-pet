// MIDI clock with the shipping WASM: one scene plays and sends its clock as
// the browser does, a second follows it; plus clock out's own rules.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const folder = resolve(process.argv[2] || "dist/web");
const { Core } = await import(pathToFileURL(resolve(folder, "core.mjs")));
const { ClockSender, MidiClock, LOOKAHEAD } = await import(
  pathToFileURL(resolve(folder, "midi-clock.mjs"))
);
const module = await WebAssembly.compile(
  await readFile(resolve(folder, "sad_pet.wasm")),
);
const RATE = 8000,
  BLOCK = 128;
const TICK = 0xf8,
  START = 0xfa,
  CONTINUE = 0xfb,
  STOP = 0xfc,
  POSITION = 0xf2;

// Clock out's rules, on its own.
{
  const sender = new ClockSender();
  const transport = (time, beat, playing, epoch, bpm = 120) => ({
    time,
    beat,
    bpm,
    playing,
    epoch,
  });
  // Stopped: ticks a 48th of a second apart from the audio rendered, never
  // beyond it.
  let out = sender.update(transport(0.05, 0, false, 0), 0);
  assert.deepEqual(out, [{ time: 0.05, bytes: [TICK] }]);
  out = sender.update(transport(0.1, 0, false, 0), 0.05);
  assert.deepEqual(out.map((m) => m.bytes[0]), [TICK, TICK]);
  assert.ok(Math.abs(out[0].time - (0.05 + 1 / 48)) < 1e-9);
  assert.ok(Math.abs(out[1].time - out[0].time - 1 / 48) < 1e-9);
  out = sender.update(transport(0.15, 0, false, 0), 0.1);
  const stoppedUntil = out.at(-1).time;
  assert.ok(stoppedUntil <= 0.15);
  // Start sounds with the scene's first beat, a tick with it, none before
  // it that a slave could count.
  out = sender.update(transport(0.2, 0, true, 1), 0.1);
  assert.deepEqual(out[0], { time: 0.2, bytes: [START] });
  assert.deepEqual(out[1], { time: 0.2, bytes: [TICK] });
  out = sender.update(transport(0.25, 0.1, true, 1), 0.15);
  assert.ok(out.length >= 2 && out.every((m) => m.bytes[0] === TICK));
  assert.ok(out.every((m) => m.time <= 0.15 + LOOKAHEAD + 1e-9));
  // Ticks sit on the beat grid of the scene's tempo.
  assert.ok(Math.abs(out[0].time - (0.2 + 1 / 48)) < 1e-9, `${out[0].time}`);
  const scheduled = out.at(-1).time;
  // Stop goes after every tick already scheduled.
  out = sender.update(transport(0.3, 0.2, false, 2), 0.2);
  assert.equal(out[0].bytes[0], STOP);
  assert.ok(out[0].time >= scheduled);
  // Moving while playing: Stop, then the song position of the next
  // sixteenth after everything scheduled, and Continue there.
  sender.update(transport(0.4, 0, true, 3), 0.35);
  sender.update(transport(0.45, 0.1, true, 3), 0.4);
  out = sender.update(transport(0.5, 4.1, true, 4), 0.45);
  const bytes = out.map((m) => m.bytes);
  assert.deepEqual(bytes[0], [STOP]);
  const position = bytes.findIndex((b) => b[0] === POSITION);
  const sixteenths = bytes[position][1] | (bytes[position][2] << 7);
  assert.ok(sixteenths >= 17, `${sixteenths}`);
  assert.deepEqual(bytes[position + 1], [CONTINUE]);
  assert.equal(bytes[position + 2]?.[0] ?? TICK, TICK);
  const resumed = out[position].time;
  assert.ok(Math.abs(resumed - (0.5 + (sixteenths / 4 - 4.1) * 0.5)) < 1e-9);
  assert.ok(out.every((m, i) => i === 0 || m.time >= out[i - 1].time));
  // A stalled page does not pour out more than two beats of late ticks.
  out = sender.update(transport(10.5, 24, true, 4), 10.45);
  assert.ok(out.length <= 48 + 8, `${out.length}`);
}

// Two scenes: A plays and sends, B follows.
function scene() {
  const utility = new Core(module, RATE),
    core = new Core(module, RATE);
  utility.call("sad_prepare", utility.call("sad_factory", undefined, 0));
  core.call("sad_load", utility.call("sad_pack"));
  return core;
}
const view = (core) => {
  const values = new Float64Array(
    core.wasm.memory.buffer,
    core.wasm.sad_clock_view(),
    10,
  );
  const [frame, beat, bpm, playing, epoch, follow, present, running, tempo] =
    values;
  return { frame, beat, bpm, playing: !!playing, epoch, follow: !!follow, present: !!present, running: !!running, tempo };
};
const a = scene(),
  b = scene();
a.request("/api/parameters", { bpm: 100 });
assert.equal(b.request("/api/preset").params.bpm, 120);
let summary = b.request("/api/clock", { follow: true, lead_ms: 0 });
assert.equal(summary.follow, true);
assert.equal(summary.present, false);
const sender = new ClockSender();
// The link: messages reach B at the time they are stamped with, and B hears
// of each only once its render passes that time.
const link = [];
// A's output sounds 20 ms after it renders.
const LATENCY = 0.02;
function run(seconds) {
  const blocks = Math.round((seconds * RATE) / BLOCK);
  for (let i = 0; i < blocks; i++) {
    a.process(BLOCK);
    const sent = view(a);
    const time = sent.frame / RATE;
    link.push(...sender.update({ ...sent, time }, time - LATENCY));
    const frame = view(b).frame;
    while (link.length && link[0].time * RATE <= frame) {
      const { time, bytes } = link.shift();
      b.wasm.sad_clock_event(time * RATE, bytes[0], bytes[1] ?? 0, bytes[2] ?? 0);
    }
    b.process(BLOCK);
  }
}
run(2);
let followed = view(b);
assert.ok(followed.present && !followed.running && !followed.playing);
assert.ok(Math.abs(followed.tempo - 100) < 0.05, `${followed.tempo}`);
assert.ok(Math.abs(followed.bpm - 100) < 0.5, `${followed.bpm}`);
a.request("/api/transport", { playing: true });
run(4);
let leader = view(a);
followed = view(b);
assert.ok(leader.playing && followed.playing && followed.running);
assert.equal(leader.frame, followed.frame);
// Both scenes sit on the same beat, to a few milliseconds.
const offBy = Math.abs(leader.beat - followed.beat) * (60 / 100);
assert.ok(offBy < 0.003, `${leader.beat} vs ${followed.beat}`);
assert.ok(leader.beat > 6, `${leader.beat}`);
let state = b.request("/api/state");
assert.equal(state.playing, true);
assert.ok(Math.abs(state.preset.params.bpm - 100) < 0.5, `${state.preset.params.bpm}`);
// Stop on A stops B.
a.request("/api/transport", { playing: false });
run(0.5);
followed = view(b);
assert.ok(!followed.playing && !followed.running && followed.present);
assert.equal(b.request("/api/state").playing, false);
// Playing again starts both at the top.
a.request("/api/transport", { playing: true });
run(1);
leader = view(a);
followed = view(b);
assert.ok(followed.playing);
assert.ok(leader.beat < 2 && Math.abs(leader.beat - followed.beat) * 0.6 < 0.003,
  `${leader.beat} vs ${followed.beat}`);
// A lead plays B ahead of A.
b.request("/api/clock", { lead_ms: 30 });
run(2);
leader = view(a);
followed = view(b);
const lead = (followed.beat - leader.beat) * 0.6;
assert.ok(Math.abs(lead - 0.03) < 0.003, `lead ${lead}`);
// Without Follow, B lets go: its timbres keep the tempo, to a tenth.
b.request("/api/clock", { follow: false });
run(0.1);
followed = view(b);
assert.ok(!followed.follow && !followed.present);
assert.equal(b.request("/api/preset").params.bpm, 100);

// Without Follow, Start and Stop still play and stop the scene.
{
  const c = scene();
  c.process(BLOCK);
  c.wasm.sad_clock_event(BLOCK, START, 0, 0);
  c.process(BLOCK);
  assert.equal(c.request("/api/state").playing, true);
  c.wasm.sad_clock_event(2 * BLOCK, TICK, 0, 0);
  c.wasm.sad_clock_event(2 * BLOCK, STOP, 0, 0);
  c.process(BLOCK);
  assert.equal(c.request("/api/state").playing, false);
}

// The browser's side: Web MIDI clock in, stamped on the audio clock, and
// clock out to a chosen port with the performance time each beat sounds.
{
  const posted = [];
  const sent = [];
  const output = {
    id: "out-1",
    name: "Network Session 1",
    state: "connected",
    send: (bytes, at) => sent.push({ bytes, at }),
  };
  const context = {
    currentTime: 1,
    getOutputTimestamp: () => ({ contextTime: 0.95, performanceTime: 5000 }),
  };
  const backend = {
    context,
    midiAccess: { outputs: new Map([["out-1", output]]) },
    audio: {
      port: { postMessage: (message) => posted.push(message) },
      call: async (type, data, transfer, path) => {
        assert.equal(path, "/api/clock");
        return { follow: !!data.follow, lead_ms: data.lead_ms ?? 0 };
      },
    },
  };
  const clock = new MidiClock(backend);
  summary = await clock.request(null);
  assert.deepEqual(summary.outputs, [{ id: "out-1", name: "Network Session 1" }]);
  assert.equal(clock.input({ data: [0x90, 60, 100], timeStamp: 4990 }), false);
  assert.equal(clock.input({ data: [TICK], timeStamp: 4990 }), true);
  // 10 ms before the output timestamp's performance time.
  assert.deepEqual(posted.at(-1), { type: "clock-event", data: [0.94, TICK, 0, 0] });
  summary = await clock.request({ follow: true, out: "out-1" });
  assert.equal(summary.follow, true);
  assert.equal(summary.out, "out-1");
  assert.deepEqual(posted.at(-1), { type: "clock-active", data: true });
  clock.report({ view: [0, 0, 120, 1, 1, 1, 1, 1, 120, 0], time: 1 });
  assert.deepEqual(sent[0], { bytes: [START], at: 5050 });
  assert.deepEqual(sent[1], { bytes: [TICK], at: 5050 });
  assert.deepEqual(clock.status(), {
    follow: true,
    present: true,
    running: true,
    tempo: 120,
    sending: "Network Session 1",
    error: null,
  });
  await assert.rejects(clock.request({ out: "gone" }), /no longer connected/);
}
console.log("MIDI clock: follow, clock out and the browser's ports work.");
