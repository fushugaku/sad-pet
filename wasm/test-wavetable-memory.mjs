// Production worker protocol + real WASM roles, not heap-growth deltas.
import assert from 'node:assert/strict';
import {readFile,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import vm from 'node:vm';
import {createHash} from 'node:crypto';
import {performance} from 'node:perf_hooks';
const folder=resolve(process.argv[2]||'dist/web'),out=process.argv[3];
const legacy=process.argv.includes('--legacy'),reportOnly=process.argv.includes('--report-only');
const {Core,TelemetryCore,VocalCore}=await import(pathToFileURL(resolve(folder,'core.mjs')));
const {preparedWavetable}=await import(pathToFileURL(resolve(folder,'wavetables.mjs')));
const moduleBytes=await readFile(resolve(folder,'sad_pet.simd.wasm'));
const module=await WebAssembly.compile(moduleBytes);
const source=(await readFile(resolve(folder,'worker.mjs'),'utf8')).replace(/^import .* from "\.\/core.mjs[^"]*";\s*/, '');
function worker(){
 let id=0;const pending=new Map();
 const self={postMessage:m=>{const key=m.id??(m.type==='snapshot'?'snapshot':null),p=pending.get(key);if(p){pending.delete(key);m.error?p.reject(Error(m.error)):p.resolve(m.result);}}};
 vm.runInNewContext(source,{self,Core,TelemetryCore,VocalCore,setTimeout,clearTimeout,performance,TextEncoder,TextDecoder,ArrayBuffer,Uint8Array,Float32Array,Int16Array,DataView,console});
 return (type,data,path,extra={})=>new Promise((resolve,reject)=>{const n=++id;pending.set(type==='snapshot'?'snapshot':n,{resolve,reject});self.onmessage({data:{...extra,id:n,type,data,path}});});
}
const utility=worker(),telemetry=worker(),vocal=worker(),audio=new Core(module,48000);
await utility('init',{module,rate:48000,wavetableWarmup:!legacy});await telemetry('init',{module,rate:48000,telemetry:true});await vocal('vocal-init',{module,rate:48000});
// Legacy worker has no memory RPC: its original, direct-instance profile is
// kept separately; this test exercises the current production protocol.
const memory=async()=>({utility:(await utility('memory')).linear_bytes,audio:audio.wasm.memory.buffer.byteLength,telemetry:(await telemetry('memory')).linear_bytes,vocal:(await vocal('memory')).linear_bytes});
const points=[{stage:'created',...await memory()}];
if(!legacy){
 const prepared=await utility('memory');
 const initFactoryReads=prepared.factory_reads.calls;
 assert.ok(prepared.warmup.frames>=65536);
 assert.ok(prepared.warmup.snapshots>0);
 assert.ok(prepared.warmup.peak>1e-4,'Warm-up did not exercise the sounding graph');
 const state=await utility('request',undefined,'/api/state');
 assert.equal(state.playing,false);assert.deepEqual(state.notes,[],'Warm-up leaked held notes');
 const defaults=await utility('wavetable-startup');
 assert.ok(defaults.rack.modules.some(m=>m.module==='wavetable'));
 assert.equal((await utility('memory')).factory_reads.calls,initFactoryReads,'Startup metadata compiled a bank');
}

if(!legacy){
 await vocal('vocal-near',0);await vocal('vocal-jobs',[]);
 assert.equal((await vocal('memory')).linear_bytes,0,'An unused Vocal worker instantiated WASM');
}
audio.wasm.sad_wavetable_realtime_init();
if(process.argv.includes('--legacy-startup')) {
 const scene=await utility('startup',{generation:1});audio.call('sad_scene_load',scene);
}
points.push({stage:process.argv.includes('--legacy-startup')?'legacy-startup':'wavetable-rack-ready',...await memory()});
const preset=JSON.parse(await readFile('reports/wavetable/wasm-unison4/target-preset.json'));
let maxTransfer=0;
async function stage(id){
 const raw=await utility('wavetable-prepare',{id}),bank=preparedWavetable(raw);maxTransfer=Math.max(maxTransfer,raw.byteLength);
 const state=audio.json('sad_wavetable_stage_begin',bank.header);
 if(!state.cached){const src=new Uint8Array(raw,bank.offset);for(let at=0;at<src.length;at+=32768){const part=src.subarray(at,at+32768);new Uint8Array(audio.wasm.memory.buffer,audio.wasm.sad_wavetable_upload(),part.length).set(part);audio.wasm.sad_wavetable_stage_chunk(part.length);}audio.json('sad_wavetable_stage_finish');}
 points.push({stage:id,bank_bytes:raw.byteLength,...await memory()});
 return createHash('sha256').update(new Uint8Array(raw)).digest('hex');
}
// Include the actual entry sequence: a default single-wave Rack first,
// interleaved table-browser metadata, then the demanding four-wave patch.
const initial=structuredClone(preset);
initial.wavetable.count=1;
for(let i=0;i<4;i++)Object.assign(initial.wavetable.oscillators[i],{
 asset_id:'factory:wt-basic-morph:v1',enabled:i===0,unison:1,fm_mode:0,fm_index:0,sync:false,
});
const initialBankHash=await stage('factory:wt-basic-morph:v1');audio.request('/api/preset',initial);
await utility('request',undefined,'/api/wavetables');
await utility('request',{id:'factory:wt-basic-morph:v1'},'/api/wavetables/preview');
for(const o of preset.wavetable.oscillators)await stage(o.asset_id);
audio.request('/api/preset',preset);audio.request('/api/transport',{playing:true,midi_gate:true});
for(const note of [48,51,55,58])audio.request('/api/note',{kind:'note_on',channel:0,note,velocity:100});
let maxSnapshot=131072;
async function snapshot(){
 maxSnapshot=Math.max(maxSnapshot,audio.wasm.sad_snapshot_capacity());
 const buffer=new Uint8Array(maxSnapshot),length=audio.snapshot(buffer),sequence=audio.wasm.sad_snapshot_sequence();
 const result=await telemetry('snapshot',null,null,{buffer:buffer.buffer,length,sequence});
 assert.ok(!result?.error);audio.wasm.sad_snapshot_ack(sequence,1);
}
let peak=0;for(let i=0;i<375;i++){
 for(const x of audio.process(128)){assert.ok(Number.isFinite(x));peak=Math.max(peak,Math.abs(x));}
 if(i%19===0)await snapshot();
}
points.push({stage:'playing',...await memory()});
const stableMemory=audio.wasm.memory.buffer.byteLength,reads=audio.factoryReads?.calls;
for(let i=0;i<375;i++){
 if(i%31===0)audio.request('/api/parameters',{wt_a_position:i/375,wt_d_position:1-i/375});
 audio.process(128);if(i%19===0)await snapshot();
}
points.push({stage:'moving-positions',...await memory()});
const recompiled=await utility('wavetable-prepare',{id:'factory:wt-basic-morph:v1'});
assert.equal(createHash('sha256').update(new Uint8Array(recompiled)).digest('hex'),initialBankHash,
 'Recycling a compile buffer changed the prepared bank bytes');
points.push({stage:'reused-compile-buffer',...await memory()});
assert.equal(audio.wasm.memory.buffer.byteLength,stableMemory,'Sustained playback grew linear memory');
assert.equal(audio.factoryReads?.calls,reads,'Rendering called the ROM imports');
if(!legacy)assert.equal(reads,0,'The realtime instance decompressed factory data');
assert.ok(peak>1e-4,'The target was silent');
let rom=0;const catalog=JSON.parse(await readFile('rust/assets/wavetables/manifest.json'));
let index=0;for(const pack of catalog.packs)for(const table of pack.tables){
 if(legacy)continue;
 const sections=WebAssembly.Module.customSections(module,`sadpet.wt.${index++}`);
 assert.equal(sections.length,1);assert.equal(sections[0].byteLength,table.packed_bytes);rom+=sections[0].byteLength;
}
rom += WebAssembly.Module.customSections(module,"sadpet.wt.info")[0]?.byteLength || 0;
const peakLinear=Math.max(...points.map(p=>p.utility+p.audio+p.telemetry+p.vocal));
const jsPools=3*maxSnapshot+4*16384*2*4;
const accounted=peakLinear+rom+maxTransfer+jsPools;
const result={scope:'Wavetable Rack and production worker roles, including initial single-wave Rack, table-browser metadata and target staging; 4 notes / 4 different 64-frame complex waves / U4 / all FM+sync. Absolute WASM linear memories of audio, utility, telemetry and Vocal roles, shared lossless table ROM counted once, largest transfer and worklet snapshot/record pools. Unrelated legacy sample-preset startup, browser DOM, compiled-code cache, JS object overhead and process RSS are outside this WT audio-data budget; --legacy-startup records that separate whole-app high-water diagnostic.',budget_bytes:128*1048576,points,peak_linear_bytes:peakLinear,shared_rom_bytes:rom,max_transfer_bytes:maxTransfer,worklet_pool_bytes:jsPools,accounted_audio_data_bytes:accounted,accounted_audio_data_mib:accounted/1048576,realtime_factory_reads:reads,stable_playback_memory:true,peak,passed:accounted<=128*1048576};
if(out)await writeFile(out,JSON.stringify(result,null,2)+'\n');console.log(JSON.stringify(result,null,2));
if(!reportOnly)assert.ok(result.passed,'Production target exceeds the absolute 128 MiB audio-data budget');
// A first real Vocal command must still initialize its renderer.
const voices=await vocal('vocal',{type:'voices'});assert.ok(Array.isArray(voices));assert.ok((await vocal('memory')).linear_bytes>0);
// The smaller initial reservation still holds the four sounding 64-frame
// banks plus four replacements. No audio-memory growth during this transition.
function silentBank(core,frames,number){
 const hash=number.toString(16).padStart(64,'0');
 const state=core.json('sad_wavetable_stage_begin',{layout_revision:2,compiler_revision:2,kernel_revision:1,content_hash:hash,frames});
 assert.equal(state.cached,false);
 new Uint8Array(core.wasm.memory.buffer,core.wasm.sad_wavetable_upload(),32768).fill(0);
 const bytes=frames===64?4530176:11829248;let status=0;
 for(let at=0;at<bytes;at+=32768)status=core.wasm.sad_wavetable_stage_chunk(Math.min(32768,bytes-at));
 assert.equal(status,1,'The complete bank must fit');core.json('sad_wavetable_stage_finish');
}
audio.wasm.sad_wavetable_prune();
for(let i=1;i<=4;i++)silentBank(audio,64,i);
assert.equal(audio.wasm.memory.buffer.byteLength,stableMemory,'Four-wave replacement grew the realtime arena');
// Retain the original ability to load four maximum-size user banks. The
// optional reserve is control-side preparation, outside the 64-frame target.
const maximum=new Core(module,48000);maximum.wasm.sad_wavetable_realtime_init();
for(let i=5;i<=8;i++)silentBank(maximum,256,i);
console.log('Arena checks passed: eight 64-frame banks without growth; four 256-frame banks at full precision.');

if(!legacy){
 const packets=new Core(module,48000),id=catalog.packs[0].tables[0].id;
 const original=packets.call('sad_wavetable_prepare',{id});
 packets.prepareWavetable({id:preset.wavetable.oscillators[0].asset_id});
 assert.deepEqual(new Uint8Array(packets.prepareWavetable({id})),new Uint8Array(original),
  'Direct worker export changed the original bank packet bytes');
 const renderSilentChange=preload=>{
  const core=new Core(module,48000),target=structuredClone(preset);
  for(const o of target.wavetable.oscillators){o.phase_random=0;o.free_phase=false;}
  if(preload){const old=structuredClone(target);for(const o of old.wavetable.oscillators)o.asset_id='factory:wt-basic-morph:v1';core.request('/api/preset',old);}
  core.request('/api/preset',target);core.request('/api/transport',{playing:true,midi_gate:true});
  for(const note of [48,51,55,58])core.request('/api/note',{kind:'note_on',channel:0,note,velocity:100});
  const frames=new Float32Array(8192);
  for(let i=0;i<32;i++)frames.set(core.process(128),i*256);
  return frames;
 };
 assert.deepEqual(renderSilentChange(true),renderSilentChange(false),
  'A silent table replacement leaked the unplayed previous table into the new attack');
 console.log('Silent table replacement matches the fresh target bit-for-bit.');
 // Missing packaged data must return through Rust, preserving the worker's
 // registry borrow and allowing the next operation to recover normally.
 const broken=Buffer.from(moduleBytes),at=broken.indexOf('sadpet.wt.0');
 assert.ok(at>=0);broken[at+7]='x'.charCodeAt(0);
 const recovery=new Core(await WebAssembly.compile(broken),48000);
 assert.throws(()=>recovery.call('sad_wavetable_prepare',{id:catalog.packs[0].tables[0].id}),/Missing or invalid packaged wavetable payload/);
 assert.throws(()=>recovery.prepareWavetable({id:catalog.packs[0].tables[0].id}),/Missing or invalid packaged wavetable payload/);
 assert.ok(recovery.request('/api/preset').params);
 assert.ok(recovery.prepareWavetable({id:catalog.packs[0].tables[1].id}).byteLength>10000);
 console.log('Missing-ROM recovery passed.');
}
