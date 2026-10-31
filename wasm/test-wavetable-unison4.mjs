// Full note-entry and live-transition regression against the saved shipping
// binary. No warm-up exclusion, sample/time alignment or gain fitting.
import assert from "node:assert/strict";
import {readFile,writeFile,mkdir} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {createHash} from "node:crypto";
const [before,after,coreFolder,out]=process.argv.slice(2);
await mkdir(out,{recursive:true});
const paths=[before,after],bytes=await Promise.all(paths.map(p=>readFile(p)));
const modules=await Promise.all(bytes.map(b=>WebAssembly.compile(b)));
const {Core}=await import(pathToFileURL(resolve(coreFolder,"core.mjs")));
const preset=JSON.parse(await readFile("reports/wavetable/wasm-unison4/target-preset.json","utf8"));
const frames=16384,results=[];
function render(module,rate,kind){
 const p=structuredClone(preset),o=p.wavetable.oscillators;
 if(kind==="pm")o.forEach(x=>x.fm_mode=2);
 if(kind==="fm")o.forEach(x=>x.sync=false);
 if(kind==="paired")o.forEach((x,i)=>{x.fm_source=i^1;x.fm_mode=i%2?0:1;x.sync_source=i;});
 if(kind==="self")o.forEach((x,i)=>{x.fm_source=i;x.sync_source=i;});
 if(kind==="sync")o.forEach((x,i)=>{x.fm_mode=0;x.sync_source=i;});
 const core=new Core(module,rate);
 core.request("/api/preset",p);core.request("/api/transport",{playing:true,midi_gate:true});
 const timbre=core.request("/api/timbres").active;
 const note=(n,on)=>core.request("/api/note",{kind:on?"note_on":"note_off",channel:0,note:n,velocity:on?100:0});
 for(const n of [48,51,55,58])note(n,true);
 const output=new Float32Array(frames*2);
 for(let at=0;at<frames;at++){
  if(kind==="fanout"&&at===1001)core.request("/api/parameters",{wt_a_fm_source:0,wt_b_fm_source:0,wt_c_fm_source:0,wt_d_fm_source:0});
  if(kind==="fanout"&&at===1002)core.request("/api/parameters",{wt_a_fm_source:3,wt_b_fm_source:0,wt_c_fm_source:1,wt_d_fm_source:2});
  if(at%257===0)core.request("/api/parameters",{wt_a_position:.1+at/frames*.8,wt_d_position:.9-at/frames*.8,wt_a_fm_index:.2+at/frames*3.7,wt_c_fm_index:3.9-at/frames*3.7,wt_a_sync_ratio:1.1+at/frames*6.5,wt_d_voice_shift:.15-at/frames*.3});
  if(at===3001)note(48,false);if(at===4201)note(48,true);
  if(at===6001)core.request("/api/parameters",{wt_b_unison:3,wt_c_unison:8});
  if(at===8001)core.request("/api/parameters",{wt_b_unison:4,wt_c_unison:4,wt_a_fm_mode:2,wt_b_sync:1});
  if(at===9001)core.request("/api/wavetables/select",{id:"factory:wt-basic-morph:v1",oscillator:1,timbre});
  if(at===11001)core.request("/api/parameters",{wt_a_fm_mode:0,wt_c_fm_mode:0,wt_a_sync:0,wt_c_sync:0});
  if(at===12001)core.request("/api/wavetables/select",{id:preset.wavetable.oscillators[1].asset_id,oscillator:1,timbre});
  if(at===13001)core.request("/api/parameters",{wt_a_fm_mode:1,wt_c_fm_mode:1,wt_a_fm_source:2,wt_b_fm_source:0,wt_c_fm_source:1,wt_d_fm_source:3,wt_a_sync:1,wt_c_sync:1});
  output.set(core.process(1),at*2);
 }
 assert.ok(output.every(Number.isFinite));return output;
}
const hash=a=>createHash("sha256").update(new Uint8Array(a.buffer,a.byteOffset,a.byteLength)).digest("hex");
for(const rate of [44100,48000,96000])for(const kind of ["mixed","pm","fm","paired","self","sync","fanout"]){
 const waves=modules.map(m=>render(m,rate,kind));let error=0,energy=0,max=0;
 for(let i=0;i<waves[0].length;i++){const d=waves[1][i]-waves[0][i];error+=d*d;energy+=waves[0][i]**2;max=Math.max(max,Math.abs(d));}
 const bit_identical=hash(waves[0])===hash(waves[1]),error_dbr=10*Math.log10(Math.max(error,1e-60)/Math.max(energy,1e-60));
 results.push({rate,kind,frames,notes:4,waves:4,initial_unison:4,bit_identical,error_dbr,max_abs_error:max,sha256:waves.map(hash),passed:max<=1e-6&&error_dbr<=-100});
 if(rate===48000&&kind==="mixed")for(let i=0;i<2;i++)await writeFile(resolve(out,i?"after.f32":"before.f32"),new Uint8Array(waves[i].buffer));
 console.log(rate,kind,bit_identical?"identical":{error_dbr,max});
}
const report={scope:"Full shipping WASM before/after; four notes, four complex waves, U4. FM/sync ring, PM/sync ring, FM ring and compatibility pairs. Includes note entry/release/retrigger, live Position, Voice shift, index, sync ratio, U3/U8/U4 and routing/mode edits at exact frames. No time/gain fitting or discarded transients.",binaries:paths.map((path,i)=>({path,sha256:createHash("sha256").update(bytes[i]).digest("hex")})),criterion_error_dbr:-100,criterion_max_abs_error:1e-6,passed:results.every(x=>x.passed),results};
await writeFile(resolve(out,"audio.json"),JSON.stringify(report,null,2)+"\n");assert.ok(report.passed);
