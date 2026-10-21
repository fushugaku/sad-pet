import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

// Run the actual editor transport, stopping before it constructs the DOM.
const source = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const boundary = source.indexOf("\nconst grainSchema = [");
assert.ok(boundary > 0);
const stepStart = source.indexOf("function queueStepEntry(");
const stepEnd = source.indexOf("\nfunction syncStepEntry(", stepStart);
assert.ok(stepStart > boundary && stepEnd > stepStart);

function editor() {
  const calls = [], timers = new Map();
  let timerId = 0, clones = 0, heldRequest = null;
  let stored = { format: "evil-pet-rust-v1", name: "Patch", params: {
    position: 0, mix: 1, master_volume: 0.65,
  }, color_chain: [], pre_color_chain: [] };
  const context = vm.createContext({
    window: {
      sadPetFetch: async (path, options = {}) => {
        if (options.body !== undefined) {
          const data = JSON.parse(options.body);
          calls.push({ path, data });
          const held = heldRequest;
          heldRequest = null;
          if (held) {
            held.enter();
            await held.wait;
            if (held.fail) return new Response('{"error":"offline"}', { status: 400 });
          }
          if (path === "/api/preset") stored = structuredClone(data);
          else if (path === "/api/parameters") Object.assign(stored.params, data);
          return new Response('{"ok":true}');
        }
        assert.equal(path, "/api/state");
        return new Response(JSON.stringify({ preset: stored }));
      },
    },
    document: { getElementById: () => ({ textContent: "", hidden: true }) },
    Response,
    structuredClone: (value) => { clones++; return structuredClone(value); },
    setTimeout: (fn, delay) => { const id = ++timerId; timers.set(id, { fn, delay }); return id; },
    clearTimeout: (id) => timers.delete(id),
  });
  vm.runInContext(source.slice(0, boundary) + `
    function syncControl() {}
    function syncAll() {}
    ${source.slice(stepStart, stepEnd)}
    preset = ${JSON.stringify(stored)};
  `, context);
  const editTimers = () => [...timers].filter(([, value]) => value.delay < 100);
  return {
    calls,
    evaluate: (code) => vm.runInContext(code, context),
    snapshot: () => JSON.parse(vm.runInContext(
      "JSON.stringify({pending,structuralPending,presetWriteCount,edits:parameterEdits.size,preset})", context)),
    clones: () => clones,
    timerCount: () => editTimers().length,
    next: () => {
      const item = editTimers()[0];
      assert.ok(item, "No editor write is scheduled");
      timers.delete(item[0]);
      return item[1].fn();
    },
    hold: (fail = false) => {
      let enter, release;
      const entered = new Promise((resolve) => { enter = resolve; });
      const wait = new Promise((resolve) => { release = resolve; });
      heldRequest = { enter, wait, fail };
      return { entered, release };
    },
  };
}

{
  const e = editor();
  e.evaluate("setParam('position',0.1);setParam('position',0.2);setParam('position',0.3)");
  assert.equal(e.timerCount(), 1);
  await e.next();
  assert.deepEqual(e.calls, [{ path: "/api/parameters", data: { position: 0.3 } }]);
  assert.equal(e.clones(), 0, "Moving a knob cloned the full preset");
  assert.equal(e.snapshot().pending, 0);
}
{
  const e = editor();
  e.evaluate("setParam('loop_one_bar',true);setParam('sample_root_note',48)");
  await e.next();
  assert.deepEqual(e.calls, [{path:"/api/parameters",data:{loop_one_bar:1,sample_root_note:48}}]);
  assert.equal(e.snapshot().preset.params.loop_one_bar, true);
  assert.equal(e.clones(), 0, "Loop checkbox rebuilt the preset");
}
{
  const e = editor(), gate = e.hold();
  e.evaluate("setParam('position',0.1)");
  const first = e.next();
  await gate.entered;
  e.evaluate("for(let i=0;i<1000;i++)setParam('position',i/1000)");
  assert.equal(e.calls.length, 1);
  assert.equal(e.timerCount(), 0, "Slow backends must not accumulate write jobs");
  gate.release();
  await first;
  await e.next();
  assert.deepEqual(e.calls.map((c) => c.data), [{ position: 0.1 }, { position: 0.999 }]);
  assert.equal(e.snapshot().pending, 0);
}
{
  const e = editor();
  e.evaluate("setParam('position',0.4);preset.color_chain.push({id:4,mix:1});schedulePreset();setParam('mix',0.2)");
  await e.next();
  assert.equal(e.calls.length, 1);
  assert.equal(e.calls[0].path, "/api/preset");
  assert.equal(e.calls[0].data.params.position, 0.4);
  assert.equal(e.calls[0].data.params.mix, 0.2);
  assert.equal(e.calls[0].data.color_chain[0].id, 4);
  assert.equal(e.snapshot().edits, 0);
}
{
  const e = editor(), gate = e.hold();
  e.evaluate("setParam('position',0.2)");
  const first = e.next();
  await gate.entered;
  e.evaluate("preset.color_chain.push({id:4,mix:1});schedulePreset()");
  const full = e.next();
  e.evaluate("setParam('position',0.85)");
  gate.release();
  await Promise.all([first, full]);
  await e.next();
  assert.deepEqual(e.calls.map((c) => c.path), ["/api/parameters", "/api/preset", "/api/parameters"]);
  assert.equal(e.calls[1].data.params.position, 0.2);
  assert.deepEqual(e.calls[2].data, { position: 0.85 });
}
{
  const e = editor();
  e.evaluate("setParam('position',0.7);setParam('master_volume',0.23)");
  await e.evaluate("flushPreset()");
  assert.equal(e.calls.length, 1);
  assert.equal(e.calls[0].path, "/api/parameters");
  assert.equal(e.snapshot().preset.params.position, 0.7);
  assert.equal(e.snapshot().preset.params.master_volume, 0.23);
  assert.equal(e.timerCount(), 0);
  assert.equal(e.snapshot().pending, 0);
}
{
  const e = editor(), gate = e.hold(true);
  e.evaluate("setParam('position',0.2)");
  const first = e.next();
  await gate.entered;
  const recall = e.evaluate("preparePresetRecall()");
  gate.release();
  await Promise.all([first, recall]);
  assert.equal(e.snapshot().edits, 0, "A failed old write leaked into a newly recalled preset");
  assert.equal(e.snapshot().structuralPending, false);
  assert.equal(e.snapshot().pending, 0);
  assert.equal(e.timerCount(), 0);
}
{
  const e = editor(), gate = e.hold(true);
  e.evaluate("setParam('position',0.2)");
  const first = e.next();
  await gate.entered;
  e.evaluate("setParam('position',0.9)");
  gate.release();
  await first;
  await e.next();
  assert.deepEqual(e.calls[1].data, { position: 0.9 }, "Retry overwrote the latest knob value");
  assert.equal(e.snapshot().pending, 0);
}
{
  const e = editor();
  e.evaluate("setParam('position',0.6)");
  await e.evaluate("queueStepEntry(2)");
  assert.deepEqual(e.calls.map((c) => c.path), ["/api/parameters", "/api/step-entry"]);
  assert.deepEqual(e.calls[0].data, { position: 0.6 });
  assert.equal(e.snapshot().pending, 0);
}
console.log("Parameter transport passed: sparse writes, coalesced dragging, structural ordering, save flush and recall/retry races.");

{
  const e=editor();
  e.evaluate('for(let i=0;i<1000;i++) scheduleSequence({track:9,stage:3,field:12,value:i})');
  await e.next();
  assert.deepEqual(e.calls,[{path:"/api/sequencer/edit",data:[{track:9,stage:3,field:12,value:999}]}]);
  assert.equal(e.clones(),0,"Dragging a stage rebuilt the full program");
}
{
  const e=editor();
  e.evaluate('scheduleSequence({track:0,stage:null,field:1,value:1});preset.sequencer={enabled:true,tracks:[]};schedulePreset()');
  await e.next();
  assert.deepEqual(e.calls.map(c=>c.path),["/api/preset","/api/sequencer/edit"]);
  assert.equal(e.calls[1].data[0].field,1,"A structural edit swallowed the restart command");
}
{
  const e=editor(),gate=e.hold(true);
  e.evaluate('scheduleSequence({track:4,stage:1,field:0,value:48})');
  const first=e.next();await gate.entered;
  e.evaluate('scheduleSequence({track:4,stage:1,field:0,value:72})');
  gate.release();await first;await e.next();
  assert.equal(e.calls[1].data[0].value,72,"Retry overwrote the newest stage value");
}
console.log("Sequencer transport passed: stage drags coalesce, structural edits preserve commands, retries keep the latest value.");

{
  const e=editor();
  e.evaluate('scheduleSequence({track:1,stage:null,field:49,value:1});preset.sequencer={enabled:true,tracks:[{id:1,pitch_oscillators:[true,false,false,false]}]};schedulePreset()');
  await e.next();
  assert.deepEqual(e.calls.map(c=>c.path),["/api/preset"],"A pitch routing checkbox was mistaken for a restart command");
  assert.equal(e.calls[0].data.sequencer.tracks[0].pitch_oscillators[0],true);
}
