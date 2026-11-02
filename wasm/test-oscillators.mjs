// Real Rust/WASM: mixed slots, all interaction domains, prepared assets,
// scalar automation, telemetry and portable scene recall.
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {preparedWavetable,wavetableIds} from "./web/wavetables.mjs";
const folder=resolve(process.argv[2]||"dist/web");
const {Core}=await import(pathToFileURL(resolve(folder,"core.mjs")));
const variant=process.argv[3]||"sad_pet.wasm";
const module=await WebAssembly.compile(await readFile(resolve(folder,variant)));
const utility=new Core(module,48000),audio=new Core(module,48000);
const stage=(id,target=audio,source=utility)=>{
  const data=source.prepareWavetable({id}),bank=preparedWavetable(data);
  if(target.json("sad_wavetable_stage_begin",bank.header).cached)return;
  const bytes=new Uint8Array(data,bank.offset);let status=0;
  for(let at=0;at<bytes.length;at+=32768){
    const chunk=bytes.subarray(at,at+32768);
    new Uint8Array(target.wasm.memory.buffer,target.wasm.sad_wavetable_upload(),chunk.length).set(chunk);
    status=target.wasm.sad_wavetable_stage_chunk(chunk.length);assert.notEqual(status,2);
  }
  assert.equal(status,1);target.json("sad_wavetable_stage_finish");
};
audio.wasm.sad_wavetable_realtime_init();
const id=audio.request("/api/timbres/add",{kind:"oscillators"}).active;
audio.request("/api/timbres/update",{id:0,muted:true});
let p=audio.request("/api/preset");
assert.deepEqual(p.rack.modules.map(m=>m.module),["oscillators","filters","mixer","post_fx"]);
assert.equal(p.oscillators.slots.length,8);assert.equal(p.wavetable,undefined);
const template=structuredClone(p.oscillators.slots[0]);
for(let i=0;i<8;i++)p.oscillators.slots[i]={...structuredClone(template),id:i+1,kind:["radias","wavefolder","pcm","wavetable"][i%4],level:.12,transpose:i};
p.oscillators.next_id=9;
p.rack.modules.splice(1,0,{module:"grains",enabled:true});
p.grain_capture={mode:"osc",slots:0b01010101};p.grain_groups={advanced:true,groups:Array.from({length:4},()=>({}))};p.grain_scheduler="independent";
p.interactions={version:1,routes:[
  {source:{kind:"osc",id:4},target:{kind:"osc",id:1},ring:.5,sync:"hard",event:"natural_wrap"},
  {source:{kind:"group",index:0},target:{kind:"osc",id:2},sync:"soft",event:"group_clock_wrap"},
  {source:{kind:"group",index:1},target:{kind:"group",index:0},sync:"hard",scope:"both",event:"grain_launch"},
]};
for(const table of wavetableIds(p))stage(table);
audio.request("/api/preset",p);
audio.request("/api/transport",{playing:true,midi_gate:true});
audio.request("/api/note",{kind:"note_on",channel:0,note:60,velocity:100});
let peak=0;for(let block=0;block<40;block++)for(const v of audio.process(128)){assert.ok(Number.isFinite(v));peak=Math.max(peak,Math.abs(v));}
assert.ok(peak>1e-4,`mixed bank is silent: ${peak}`);
audio.request("/api/parameters",{osc8_wt_position:.73,osc1_native_control1:.3,osc_surf:.4,interaction1_ring:.25});
p=audio.request("/api/preset");assert.ok(Math.abs(p.oscillators.slots[7].wavetable.position-.73)<1e-5);
assert.equal(p.interactions.routes[0].source.id,4);assert.equal(p.interactions.routes[0].ring,.25);
const memory=audio.wasm.memory.buffer;
for(let i=0;i<16;i++){
  audio.request("/api/parameters",{osc_surf:i/16,interaction2_response:2+i,osc2_fine:i});
  assert.ok(audio.process(128).every(Number.isFinite));
}
assert.equal(audio.wasm.memory.buffer,memory,"scalar controls grew the prepared WASM heap");
p=audio.request("/api/preset");p.oscillators.order=[7,0,1,2,3,4,5,6];audio.request("/api/preset",p);
assert.equal(audio.request("/api/preset").interactions.routes[0].source.id,4);
const exported=utility.request("/api/wavetables/export",{id:template.asset_id});
const imported=utility.request("/api/wavetables/import",{wav:exported.wav,name:"Mixed bank portable asset"});
stage(imported.id);p.oscillators.slots[7].asset_id=imported.id;audio.request("/api/preset",p);
utility.call("sad_scene_adopt",audio.call("sad_scene_pack"));
const document=utility.json("sad_scene_document");assert.equal(document.wavetable_assets.length,1);
const recalledUtility=new Core(module,48000),recalled=new Core(module,48000);
const scene=recalledUtility.call("sad_scene_prepare",document);recalled.wasm.sad_wavetable_realtime_init();
for(const table of wavetableIds(document))stage(table,recalled,recalledUtility);
recalled.call("sad_scene_load",scene);
const saved=recalled.request("/api/preset");
assert.equal(saved.oscillators.slots[7].asset_id,imported.id);assert.deepEqual(saved.interactions,p.interactions);
assert.deepEqual(saved.oscillators.order,p.oscillators.order);assert.deepEqual(saved.grain_capture,p.grain_capture);
const packet=new Uint8Array(audio.wasm.sad_snapshot_capacity()),size=audio.snapshot(packet);
const decoded=utility.json("sad_snapshot_import",packet.slice(0,size));
assert.equal(decoded.state.oscillator_positions.length,8);assert.equal(decoded.state.rack_peaks.length,8);
assert.ok(decoded.state.knobs.osc8_wt_position);
assert.ok(recalled.process(128).every(Number.isFinite));
console.log(`${variant}: mixed oscillators: 8 stable slots, cross-type/group Ring+Sync, capture, automation, telemetry and portable user assets passed`);
