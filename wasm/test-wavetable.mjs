// Real shipping Rust/WASM: utility compilation, bounded worklet staging,
// Rack voices, scalar edits, telemetry and portable user assets.
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {preparedWavetable} from "./web/wavetables.mjs";
const folder=resolve(process.argv[2]||"dist/web");
const {Core}=await import(pathToFileURL(resolve(folder,"core.mjs")));
const module=await WebAssembly.compile(await readFile(resolve(folder,"sad_pet.wasm")));
const utility=new Core(module,48000),audio=new Core(module,48000);
audio.wasm.sad_wavetable_realtime_init();
const stage=(id,target=audio,source=utility)=>{
  const prepared=source.call("sad_wavetable_prepare",{id}),bank=preparedWavetable(prepared);
  const state=target.json("sad_wavetable_stage_begin",bank.header);
  if(state.cached)return;
  const bytes=new Uint8Array(prepared,bank.offset),memory=target.wasm.memory.buffer;
  let done=0;
  for(let offset=0;offset<bytes.length;offset+=32768){
    const chunk=bytes.subarray(offset,offset+32768);
    new Uint8Array(target.wasm.memory.buffer,target.wasm.sad_wavetable_upload(),chunk.length).set(chunk);
    done=target.wasm.sad_wavetable_stage_chunk(chunk.length);
    assert.notEqual(done,2,"valid bank rejected");
    assert.equal(target.wasm.memory.buffer,memory,"bounded copying grew WASM memory");
    target.process(128);
  }
  assert.equal(done,1);target.json("sad_wavetable_stage_finish");
};
const catalog=utility.request("/api/wavetables");
assert.equal(catalog.tables.length,96);assert.equal(new Set(catalog.tables.map(t=>t.pack)).size,13);
const instruments=utility.request("/api/wavetables/presets");assert.equal(instruments.length,80);
const fourWave=utility.request("/api/wavetables/preset",{id:instruments.find(p=>p.name==="Fourfold Orchestra").id});
assert.equal(fourWave.preset.wavetable.count,4);assert.equal(fourWave.preset.params.out_gain,6);
utility.request("/api/preset",fourWave.preset);
assert.ok(Math.abs(utility.request("/api/state").wavetable_positions[0].current-fourWave.preset.wavetable.oscillators[0].position)<1e-6,
  "idle frame preview must follow the selected Position before audio starts");
assert.throws(()=>audio.request("/api/timbres/add",{kind:"wavetable"}),/loading|loading|finished/i,"worklet tried to compile an unstaged table");
stage("factory:wt-basic-morph:v1");
const id=audio.request("/api/timbres/add",{kind:"wavetable"}).active;
const preset=audio.request("/api/preset");
assert.deepEqual(preset.rack.modules.map(s=>s.module),["wavetable","filters","mixer","post_fx"]);
assert.equal(typeof preset.wavetable.seed,"string","64-bit seed must survive JS JSON round trips");
audio.request("/api/timbres/update",{id:0,muted:true});
audio.request("/api/transport",{playing:true,midi_gate:true});
audio.request("/api/note",{kind:"note_on",channel:0,note:60,velocity:100});
let peak=0;
for(let i=0;i<40;i++)for(const sample of audio.process(128)){assert.ok(Number.isFinite(sample));peak=Math.max(peak,Math.abs(sample));}
assert.ok(peak>0.01,`Wavetable is silent: ${peak}`);
assert.equal(audio.request("/api/state").rack_peaks.length,7);
for(let count=2;count<=4;count++) {
  const added=audio.request("/api/wavetables/layers",{action:"add",timbre:id});
  assert.equal(added.wavetable.count,count);
  assert.ok(audio.process(128).every(Number.isFinite));
}
assert.throws(()=>audio.request("/api/wavetables/layers",{action:"add",timbre:id}),/four/);
audio.request("/api/parameters",{wt_c_position:.25,wt_d_position:.75,wt_d_fine:-7});
assert.equal(audio.request("/api/preset").wavetable.oscillators[3].fine,-7);
const complex=catalog.tables.find(t=>t.pack==="fm").id;
stage(complex);audio.request("/api/wavetables/select",{id:complex,oscillator:2,timbre:id});
// A scalar Position edit never compiles/replaces its bank and does not grow memory.
const memory=audio.wasm.memory.buffer;
for(let i=0;i<100;i++){
  audio.request("/api/parameters",{wt_a_position:i/99,wt_b_level:.7});
  assert.ok(audio.process(128).every(Number.isFinite));
}
assert.equal(audio.wasm.memory.buffer,memory,"scalar controls grew WASM memory");
stage(catalog.tables.find(t=>t.pack==="formant").id);
audio.request("/api/wavetables/select",{id:catalog.tables.find(t=>t.pack==="formant").id,oscillator:0,timbre:id});
for(let i=0;i<12;i++)assert.ok(audio.process(128).every(Number.isFinite));
// A missing asset/version fails atomically, leaving the old sound/settings.
const before=audio.request("/api/preset");
assert.throws(()=>audio.request("/api/wavetables/select",{id:"user:"+"0".repeat(64),oscillator:0,timbre:id}));
assert.deepEqual(audio.request("/api/preset"),before);
const future=structuredClone(before);future.wavetable.engine_revision=999;
assert.throws(()=>audio.request("/api/preset",future),/version|update/);
assert.deepEqual(audio.request("/api/preset"),before);
// Canonical export/import is an original local file; stereo/period choices
// are enforced in Rust, and analyzing it must not add a library entry.
const exported=utility.request("/api/wavetables/export",{id:catalog.tables[0].id});
const info=utility.request("/api/wavetables/import",{wav:exported.wav,inspect:true});
assert.equal(info.frame_size,2048);assert.equal(info.channels,1);
utility.request("/api/wavetables/import",{wav:exported.wav,name:"Preview only",preview:true});
assert.equal(utility.request("/api/wavetables").tables.length,96);
const imported=utility.request("/api/wavetables/import",{wav:exported.wav,name:"My original"});
assert.ok(imported.id.startsWith("user:"));
stage(imported.id);
audio.request("/api/wavetables/select",{id:imported.id,oscillator:0,timbre:id});
utility.call("sad_scene_adopt",audio.call("sad_scene_pack"));
const document=utility.json("sad_scene_document");
assert.equal(document.wavetable_assets.length,1);
assert.ok(document.timbres.every(part=>!part.wavetable_assets));
const recalledUtility=new Core(module,48000),recalled=new Core(module,48000);
const scene=recalledUtility.call("sad_scene_prepare",document);
recalled.wasm.sad_wavetable_realtime_init();
stage(imported.id,recalled,recalledUtility);
stage("factory:wt-basic-morph:v1",recalled,recalledUtility);
stage(complex,recalled,recalledUtility);
recalled.call("sad_scene_load",scene);
assert.equal(recalled.request("/api/preset").wavetable.oscillators[0].asset_id,imported.id);
assert.equal(recalled.request("/api/preset").wavetable.count,4);
assert.equal(recalled.request("/api/preset").wavetable.oscillators[3].fine,-7);
assert.ok(recalled.process(128).every(Number.isFinite));
const packet=new Uint8Array(audio.wasm.sad_snapshot_capacity());
const size=audio.snapshot(packet),decoded=utility.json("sad_snapshot_import",packet.slice(0,size));
assert.equal(decoded.state.wavetable_positions.length,4);
assert.equal(decoded.state.rack_peaks.length,7);
const removed=audio.request("/api/wavetables/layers",{action:"remove",index:1,timbre:id});
assert.equal(removed.wavetable.count,3);assert.equal(removed.wavetable.oscillators[2].fine,-7);
assert.ok(audio.process(128).every(Number.isFinite));
console.log("wavetable: 96 built-ins / 13 packs, 1–4 waves, staging, Rack, edits, safe rejection and portable scene passed");
