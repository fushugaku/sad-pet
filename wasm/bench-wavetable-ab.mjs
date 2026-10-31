// Sequential AB/BA; do not run builds, audio tests or other benchmarks beside it.
import assert from "node:assert/strict";
import {mkdir,readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
import {spawn} from "node:child_process";
import {cpus,platform,release} from "node:os";
const [before,folder,out]=process.argv.slice(2);
assert.equal(typeof process.threadCpuUsage,"function","Use Node 24 for per-thread CPU timing.");
await mkdir(out,{recursive:true});
const cases=[{mode:"mixed",motion:false},{mode:"paired",motion:false},{mode:"mixed",motion:true}];
const runs=[];
for(const c of cases)for(let repeat=0;repeat<2;repeat++)for(const version of repeat?["after","before"]:["before","after"]){
 const dir=resolve(out,`${c.mode}-${c.motion?"motion":"held"}-${repeat}-${version}`);
 const binary=version==="before"?resolve(before):resolve(folder,"sad_pet.simd.wasm");
 await new Promise((done,reject)=>{
  const child=spawn(process.execPath,["wasm/bench-wavetable-target.mjs",binary,folder,dir],{
   env:{...process.env,WT_SECONDS:process.env.WT_SECONDS||"2",WT_ROUNDS:"1",WT_MODE:c.mode,WT_MOTION:c.motion?"1":"0"},stdio:["ignore","ignore","pipe"]});
  let errors="";child.stderr.on("data",b=>errors+=b);child.on("error",reject);child.on("exit",code=>code?reject(Error(errors)):done());
 });
 const data=JSON.parse(await readFile(resolve(dir,"cpu.json"),"utf8"));
 runs.push({version,repeat,...data});console.log(c.mode,c.motion?"motion":"held",repeat,version,data.thread_cpu.p50_ms);
}
const med=a=>{a.sort((a,b)=>a-b);return(a[Math.floor((a.length-1)/2)]+a[Math.floor(a.length/2)])/2;};
const summary=cases.map(c=>{
 const row={...c,notes:4,waves:4,unison:4};
 for(const version of ["before","after"]){
  const r=runs.filter(r=>r.mode===c.mode&&r.moving_positions===c.motion&&r.version===version);
  row[version]={};
  for(const [key,field]of[["wall","summary"],["thread_cpu","thread_cpu"]])row[version][key]={
   ...Object.fromEntries(["p50_ms","p95_ms","p99_ms"].map(k=>[k,med(r.map(x=>x[field][k]))])),
   blocks:r.reduce((a,x)=>a+x[field].blocks,0),over_budget:r.reduce((a,x)=>a+x[field].over_budget,0),max_ms:Math.max(...r.map(x=>x[field].max_ms))};
 }
 row.cpu_reduction_percent=100*(1-row.after.thread_cpu.p50_ms/row.before.thread_cpu.p50_ms);
 row.node_wall_p99_fits_quantum=row.after.wall.p99_ms<128/48;return row;
});
await writeFile(resolve(out,"comparison.json"),JSON.stringify({scope:"Production shipping WASM, Core.process(128) including JS ABI. Per-thread user+system CPU and wall clock measured around rendering only; assets/preparation, controls, signal validation and compilation excluded. Separate from browser AudioWorklet/playbackStats acceptance.",method:"Sequential AB/BA, two independent runs per version/case; summary p50/p95/p99 are medians of run percentiles. No V8 flags. No concurrent builds/tests from this task. Other user background processes left running.",machine:{cpu:cpus()[0].model,logical_cpus:cpus().length,platform:platform(),os:release(),arch:process.arch},node:process.version,sample_rate:48000,block:128,deadline_ms:128/48,summary,runs},null,2)+"\n");
