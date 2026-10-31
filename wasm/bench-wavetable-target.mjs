// Exact target: four notes, four distinct waves, U4, FM and sync together.
import assert from "node:assert/strict";
import {readFile,writeFile,mkdir} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {performance} from "node:perf_hooks";
import {createHash} from "node:crypto";
import {Session} from "node:inspector";
const [modulePath,folder,out]=process.argv.slice(2);
const profile=process.argv.includes("--profile");
const seconds=Number(process.env.WT_SECONDS||2),rounds=Number(process.env.WT_ROUNDS||3);
await mkdir(out,{recursive:true});
const bytes=await readFile(modulePath),module=await WebAssembly.compile(bytes);
const {Core}=await import(pathToFileURL(resolve(folder,"core.mjs")));
const preset=JSON.parse(await readFile("reports/wavetable/wasm-unison4/target-preset.json","utf8"));
const mode=process.env.WT_MODE||"mixed",motion=process.env.WT_MOTION==="1";
if(mode==="paired")preset.wavetable.oscillators.forEach((o,i)=>{o.fm_source=i^1;o.fm_mode=i%2?0:1;o.sync_source=i;});
if(mode==="pm")preset.wavetable.oscillators.forEach(o=>o.fm_mode=2);
const notes=(process.env.WT_NOTES||"48,51,55,58").split(",").map(Number);
const core=new Core(module,48000);
core.request("/api/preset",preset);core.request("/api/transport",{playing:true,midi_gate:true});
for(const note of notes)core.request("/api/note",{kind:"note_on",channel:0,note,velocity:100});
for(let i=0;i<375;i++)core.process(128);
const inspector=profile?new Session():null;
const post=(name,args={})=>new Promise((resolve,reject)=>inspector.post(name,args,(e,r)=>e?reject(e):resolve(r)));
if(profile){inspector.connect();await post("Profiler.enable");await post("Profiler.setSamplingInterval",{interval:100});await post("Profiler.start");}
const runs=[],all=[],cpus=[];let peak=0,energy=0;
for(let r=0;r<rounds;r++){
 const times=[],cpuTimes=[];
 for(let i=0;i<Math.ceil(seconds*48000/128);i++){
  if(motion){const t=(i%375)/374;core.request("/api/parameters",{wt_a_position:.1+t*.8,wt_b_position:.9-t*.8,wt_c_position:.2+t*.5,wt_d_position:.8-t*.6});}
  const cpu=process.threadCpuUsage(),at=performance.now(),a=core.process(128);const t=performance.now()-at,used=process.threadCpuUsage(cpu),c=(used.user+used.system)/1000;times.push(t);all.push(t);cpuTimes.push(c);cpus.push(c);
  for(const x of a){assert.ok(Number.isFinite(x));peak=Math.max(peak,Math.abs(x));energy+=x*x;}
 }
 runs.push({...stats(times),thread_cpu:stats(cpuTimes)});
}
if(profile){const {profile:p}=await post("Profiler.stop");await writeFile(resolve(out,"render.cpuprofile"),JSON.stringify(p));inspector.disconnect();}
assert.ok(peak>1e-4&&energy>1e-3,"the target was silent");
function stats(a){a.sort((a,b)=>a-b);return{blocks:a.length,p50_ms:a[Math.floor(a.length*.5)],p95_ms:a[Math.floor(a.length*.95)],p99_ms:a[Math.floor(a.length*.99)],max_ms:a.at(-1),over_budget:a.filter(t=>t>128/48).length};}
const result={scope:"Shipping WASM Core.process(128), 4 held notes × 4 complex waves × U4; all four linear FM operators and hard sync, reciprocal rings; includes JS ABI boundary, excludes preparation and validation; Node/V8 wall timing, separate from AudioWorklet/device acceptance.",sample_rate:48000,block:128,deadline_ms:128/48,notes:4,waves:4,unison:4,oversampling:{fm:8,sync:32},node:process.version,v8:process.versions.v8,module_sha256:createHash("sha256").update(bytes).digest("hex"),profiled:profile,warmup_audio_seconds:1,audio_seconds_per_round:seconds,rounds,peak,summary:stats(all),runs};
Object.assign(result,{scope:`Shipping WASM Core.process(128), four notes × four complex waves × U4; ${mode==="paired"?"compatibility pairs B→A / D→C with four self-sync clocks":"four FM operators and four hard-sync clocks in independent reciprocal rings"}. Includes JS ABI boundary; excludes preparation, control requests and validation. Node/V8 thread CPU + wall timing, separate from browser AudioWorklet acceptance.`,mode,moving_positions:motion,note_numbers:notes,thread_cpu:stats(cpus)});
await writeFile(resolve(out,"cpu.json"),JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify(result));
