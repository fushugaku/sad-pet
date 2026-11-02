// Repeatable local Core render timings, not a guarantee for a mobile AudioWorklet.
// node wasm/bench-oscillators.mjs /path/to/web [sad_pet.simd.wasm]
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {performance} from "node:perf_hooks";

const folder=resolve(process.argv[2]||"dist/web"),variant=process.argv[3]||"sad_pet.simd.wasm";
const {Core}=await import(pathToFileURL(resolve(folder,"core.mjs")));
const module=await WebAssembly.compile(await readFile(resolve(folder,variant)));
const cases=[
  {name:"1 RADIAS / 1 voice",count:1,voices:1},
  {name:"4 mixed / 1 voice",count:4,voices:1},
  {name:"8 mixed / 1 voice",count:8,voices:1},
  {name:"8 mixed / 4 voices",count:8,voices:4},
  {name:"8 mixed + group Playback/Ring / 4 voices",count:8,voices:4,groups:true},
  {name:"8 mixed + group cycle / 4 voices (audio-rate births)",count:8,voices:4,groups:true,births:true},
];
const result=[];
for(const item of cases){
  const core=new Core(module,48000);
  core.request("/api/timbres/add",{kind:"oscillators"});
  core.request("/api/timbres/remove",{id:0});
  const p=core.request("/api/preset"),template=structuredClone(p.oscillators.slots[0]);
  for(let i=0;i<item.count;i++)p.oscillators.slots[i]={...structuredClone(template),id:i+1,
    kind:["radias","wavefolder","pcm","wavetable"][i%4],level:.1,transpose:i};
  p.oscillators.next_id=item.count+1;p.params.filter=1;p.params.filter2=1;
  p.color_chain=[];p.pre_color_chain=[];
  if(item.groups){
    p.rack.modules.splice(1,0,{module:"grains",enabled:true});
    p.grain_capture={mode:"osc",slots:85};
    p.grain_groups={advanced:true,groups:Array.from({length:4},()=>({}))};
    p.interactions={version:1,routes:[
      {source:{kind:"osc",id:4},target:{kind:"osc",id:1},ring:.5,sync:"hard"},
      {source:{kind:"osc",id:1},target:{kind:"group",index:0},sync:item.births?"hard":"soft",scope:item.births?"both":"playback"},
      {source:{kind:"group",index:0},target:{kind:"osc",id:8},ring:.5,sync:"soft"},
      {source:{kind:"group",index:1},target:{kind:"group",index:2},ring:.5,sync:"hard",scope:"clock"},
    ]};
  }
  core.request("/api/preset",p);core.request("/api/transport",{playing:true,midi_gate:true});
  for(const note of [60,64,67,72].slice(0,item.voices))core.request("/api/note",{kind:"note_on",channel:0,note,velocity:100});
  for(let i=0;i<256;i++)core.process(128);
  const memory=core.wasm.memory.buffer,timings=[];let peak=0;
  for(let i=0;i<512;i++){
    const begin=performance.now(),audio=core.process(128);timings.push(performance.now()-begin);
    for(const v of audio){assert.ok(Number.isFinite(v));peak=Math.max(peak,Math.abs(v));}
  }
  assert.ok(peak>1e-5,`${item.name}: silent render`);
  assert.equal(core.wasm.memory.buffer,memory,"render grew memory");
  timings.sort((a,b)=>a-b);
  const state=core.request("/api/state");
  const packet=new Uint8Array(core.wasm.sad_snapshot_capacity()),size=core.snapshot(packet);
  const telemetry=core.json("sad_snapshot_import",packet.slice(0,size)).state;
  result.push({case:item.name,p50_ms:+timings[256].toFixed(3),p95_ms:+timings[486].toFixed(3),
    p99_ms:+timings[506].toFixed(3),worst_ms:+timings.at(-1).toFixed(3),
    deadline_ms:+(128/48).toFixed(3),linear_memory_MiB:memory.byteLength/1048576,
    live_grains:state.active_grains,dropped_births:telemetry.dropped_grain_births});
}
console.log(JSON.stringify({variant,rate:48000,frames:128,results:result},null,2));
