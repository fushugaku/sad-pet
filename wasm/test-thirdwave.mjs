// Shipping WASM: shared modulation, interaction modes, offline tools,
// capture streaming, key splits and declarative motion-library recall.
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
function stage(id,to=audio,from=utility) {
  const bytes=from.call("sad_wavetable_prepare",{id}),prepared=preparedWavetable(bytes);
  if(to.json("sad_wavetable_stage_begin",prepared.header).cached)return;
  const payload=new Uint8Array(bytes,prepared.offset);
  for(let offset=0;offset<payload.length;offset+=32768){const chunk=payload.subarray(offset,offset+32768);new Uint8Array(to.wasm.memory.buffer,to.wasm.sad_wavetable_upload(),chunk.length).set(chunk);assert.notEqual(to.wasm.sad_wavetable_stage_chunk(chunk.length),2);}
  to.json("sad_wavetable_stage_finish");
}
stage("factory:wt-basic-morph:v1");
const part=audio.request("/api/timbres/add",{kind:"wavetable"}).active;
audio.request("/api/timbres/update",{id:0,muted:true});
for(let n=1;n<4;n++)audio.request("/api/wavetables/layers",{action:"add",timbre:part});
let p=audio.request("/api/preset");
assert.equal(p.wavetable.engine_revision,3);
p.wavetable.oscillators.forEach((o,i)=>Object.assign(o,{enabled:true,level:.2,position:.5,surf_depth:i%2?-1:1,key_waves:.1,free_phase:i===3}));
p.envs[1].wave={enabled:true,beats:false,times:[1,.4,.3,.4,.1,.1],levels:[1,.2,.8,.4,.2,0],curves:[0,0,0,0,0,0],loop_on:true,loop_start:1,loop_end:3};
Object.assign(p.wavetable.oscillators[0],{position:.1,position_env:2,position_env_depth:.7});
audio.request("/api/preset",p);
audio.request("/api/transport",{playing:true,midi_gate:true});
const note=(pitch,velocity=100,kind="note_on")=>audio.request("/api/note",{kind,channel:0,note:pitch,velocity});
note(60);for(let n=0;n<75;n++)audio.process(128);note(72);for(let n=0;n<38;n++)audio.process(128);
let view=audio.request("/api/state");assert.ok(view.wavetable_positions[0].max-view.wavetable_positions[0].min>.1,`per-note Wave ENV collapsed: ${JSON.stringify(view.wavetable_positions[0])}`);
audio.request("/api/parameters",{wt_surf:.2,wt_xy_on:1,wt_x:.8,wt_y:.3,wt_a_unison:3,wt_a_voice_shift:.12,wt_c_step:1,lfo1_poly:1,wt_b_lfo:1,wt_b_lfo_depth:.2,wt_a_fm_mode:1,wt_a_fm_index:.7,wt_c_sync:1,wt_c_sync_ratio:2.75});
const memory=audio.wasm.memory.buffer;
for(let mode=0;mode<=2;mode++){audio.request("/api/parameters",{wt_a_fm_mode:mode});for(let n=0;n<12;n++)assert.ok(audio.process(128).every(Number.isFinite));}
// Every carrier/source pair, including reciprocal feedback and independent
// FM/sync sources, must survive realtime automation and scene recall.
for(const carrier of ["a","b","c","d"])for(let source=0;source<4;source++){
  audio.request("/api/parameters",{[`wt_${carrier}_fm_mode`]:1,[`wt_${carrier}_fm_index`]:.7,[`wt_${carrier}_fm_source`]:source,[`wt_${carrier}_sync`]:1,[`wt_${carrier}_sync_source`]:(source+1)%4});
  assert.ok(audio.process(128).every(Number.isFinite));
}
for(const [i,carrier] of ["a","b","c","d"].entries())audio.request("/api/parameters",{[`wt_${carrier}_fm_mode`]:i%2+1,[`wt_${carrier}_fm_source`]:(i+3)%4,[`wt_${carrier}_sync_source`]:(i+1)%4});
for(let n=0;n<20;n++)assert.ok(audio.process(128).every(Number.isFinite));
assert.equal(audio.wasm.memory.buffer,memory);
view=audio.request("/api/state");assert.equal(view.wavetable_motion.xy_on,true);assert.ok(view.knobs.wt_x);assert.ok(view.knobs.wt_a_position);assert.ok(view.knobs.wt_a_voice_shift);assert.ok(view.wavetable_positions[0].max-view.wavetable_positions[0].min>.1);

// Shared capture reserves its entire bounded storage before audio. Reading
// is at most 32 KiB per message, never a base64/JSON export on the worklet.
const capture=audio.request("/api/capture/start",{seconds:.1});
for(let n=0;n<38;n++)audio.process(128);
const take=audio.request("/api/capture/status");assert.equal(take.ready,true);assert.equal(take.frames,4800);
const pcm=new Uint8Array(take.frames*8);
for(let offset=0;offset<pcm.length;offset+=32768){const size=audio.wasm.sad_capture_chunk(take.generation,offset);assert.ok(size>0&&size<=32768);pcm.set(new Uint8Array(audio.wasm.memory.buffer,audio.wasm.sad_wavetable_upload(),size),offset);}
assert.ok(new Float32Array(pcm.buffer).some(v=>Math.abs(v)>1e-4));
assert.equal(audio.wasm.sad_capture_chunk(capture.generation+1,0),-1);
assert.throws(()=>audio.request("/api/capture/wav"),/bounded/);
assert.equal(audio.wasm.memory.buffer,memory);

function sineWav(){const buffer=Buffer.alloc(44+48000*8);buffer.write("RIFF",0);buffer.writeUInt32LE(buffer.length-8,4);buffer.write("WAVEfmt ",8);buffer.writeUInt32LE(16,16);buffer.writeUInt16LE(3,20);buffer.writeUInt16LE(2,22);buffer.writeUInt32LE(48000,24);buffer.writeUInt32LE(384000,28);buffer.writeUInt16LE(8,32);buffer.writeUInt16LE(32,34);buffer.write("data",36);buffer.writeUInt32LE(48000*8,40);for(let i=0;i<48000;i++){const x=Math.sin(i/48000*Math.PI*2*220)*.65;buffer.writeFloatLE(x,44+i*8);buffer.writeFloatLE(x*.5,48+i*8);}return buffer.toString("base64");}
const wav=sineWav(),count=utility.request("/api/wavetables").tables.length;
const made=utility.request("/api/wavetables/make",{wav,name:"Captured spectrum",generation:100,settings:{method:"spectral",frames:16,channel:"mix"}});
assert.equal(made.frames,16);assert.equal(utility.request("/api/wavetables").tables.length,count);
assert.throws(()=>utility.request("/api/wavetables/commit",{id:made.id,generation:99}),/preview/);
const saved=utility.request("/api/wavetables/commit",{id:made.id,generation:100});assert.equal(saved.id,made.id);
const cycles=utility.request("/api/wavetables/make",{wav,name:"Detected cycles",generation:101,settings:{method:"cycles",frames:8,channel:"left"}});
utility.request("/api/wavetables/commit",{id:cycles.id,generation:101});
const edited=utility.request("/api/wavetables/edit",{id:cycles.id,name:"Vintage order",generation:102,settings:{order:[7,0,3],bits:8,phase_steps:128,locals:[{frame:1,phase:.25,gains:[{harmonic:1,gain:.5}]}]}});
assert.equal(edited.frames,3);assert.notEqual(edited.id,cycles.id);
utility.request("/api/wavetables/commit",{id:edited.id,generation:102});
assert.throws(()=>utility.request("/api/wavetables/edit",{id:cycles.id,name:"Invalid",generation:103,settings:{order:[99]}}));
assert.equal(utility.request("/api/wavetables").tables.length,count+3);
stage(edited.id);audio.request("/api/wavetables/select",{id:edited.id,oscillator:3,timbre:part});

// Range edits affect shared MIDI routing, including generic Rack sources.
audio.request("/api/timbres/update",{id:part,key_low:72,key_high:84,velocity_low:64});
audio.request("/api/midi-bytes",[0x90,60,100]);for(let n=0;n<200;n++)audio.process(128);
const rejectedPeak=Math.max(...audio.process(128).map(Math.abs));assert.ok(rejectedPeak<1e-6,`a rejected key reached the split part: ${rejectedPeak}; ${JSON.stringify(audio.request("/api/timbres").timbres.map(t=>({id:t.id,low:t.key_low,high:t.key_high,velocity:t.velocity_low})))}`);
audio.request("/api/midi-bytes",[0x90,72,100]);assert.ok(Array.from({length:50},()=>audio.process(128)).some(x=>x.some(v=>Math.abs(v)>1e-4)));

const catalog=utility.request("/api/sequencer/catalog");assert.equal(catalog.motion_patterns.length,8);assert.equal(catalog.filter_presets.length,6);
p=audio.request("/api/preset");p.sequencer.patterns=[catalog.motion_patterns[0]];p.sequencer.tracks=catalog.motion_patterns[2].tracks;p.sequencer.enabled=true;
Object.assign(p.params,catalog.filter_presets[0].params);audio.request("/api/preset",p);
assert.equal(audio.request("/api/preset").sequencer.patterns[0].name,"Slow Surf");
utility.call("sad_scene_adopt",audio.call("sad_scene_pack"));const scene=utility.json("sad_scene_document");assert.equal(scene.wavetable_assets.length,1);
assert.ok(scene.timbres.some(t=>t.meta?.key_low===72&&t.meta?.velocity_low===64));
assert.ok(scene.timbres.some(t=>t.preset.sequencer.patterns?.[0]?.name==="Slow Surf"));
const recalled=new Core(module,48000),recalledUtility=new Core(module,48000),packed=recalledUtility.call("sad_scene_prepare",scene);
recalled.wasm.sad_wavetable_realtime_init();stage(edited.id,recalled,recalledUtility);stage("factory:wt-basic-morph:v1",recalled,recalledUtility);recalled.call("sad_scene_load",packed);
assert.ok(recalled.request("/api/timbres").timbres.some(t=>t.key_low===72&&t.velocity_low===64));
const recalledPreset=recalled.request("/api/preset");assert.ok(Math.abs(recalledPreset.wavetable.oscillators[0].voice_shift-.12)<1e-6);assert.equal(recalledPreset.sequencer.patterns[0].name,"Slow Surf");
recalledPreset.wavetable.oscillators.forEach((o,i)=>{assert.equal(o.fm_source,(i+3)%4);assert.equal(o.sync_source,(i+1)%4);assert.equal(o.fm_mode,i%2+1);});
recalled.request("/api/transport",{playing:true,midi_gate:true});recalled.request("/api/midi-bytes",[0x90,72,100]);assert.ok(Array.from({length:50},()=>recalled.process(128)).some(x=>x.some(v=>Math.abs(v)>1e-4)));
console.log("thirdwave: shared ENV/LFO, Surf/Step/Keys/XY, unison Voice shift, FM/PM/sync, bounded capture, Maker/editor/vintage, splits, filter pairs and motion recall passed");
