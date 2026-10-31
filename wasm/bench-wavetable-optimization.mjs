// Shipping SIMD WASM timing in Node/V8, separate from native thread CPU.
import assert from "node:assert/strict";
import {readFile,writeFile} from "node:fs/promises";
import {pathToFileURL} from "node:url";
import {resolve} from "node:path";
import {createHash} from "node:crypto";
import {performance} from "node:perf_hooks";
const [before,folder,patches,report]=process.argv.slice(2);
const {Core}=await import(pathToFileURL(resolve(folder,"core.mjs")));
const paths=[resolve(before),resolve(folder,"sad_pet.simd.wasm")];
const bytes=await Promise.all(paths.map(p=>readFile(p)));
const modules=await Promise.all(bytes.map(b=>WebAssembly.compile(b)));
const presets=JSON.parse(await readFile(patches,"utf8"));
const rate=48000,block=128,seconds=.75,rounds=2,runs=[];
const unison=process.argv.includes("--unison2")?2:4;
const routed=process.argv.includes("--routed"),onlyAfter=process.argv.includes("--after-only");
const focused=process.argv.includes("--focused");
const cases=routed?(focused?[["mixed",2]]:[["mixed",2],["mixed",3],["mixed",4]]):focused?[["fm",4],["mixed",3]]:unison===2?[["fm",4],["sync",4],["mixed",4],["mixed",3]]:[["off",8],["fm",4],["fm",8],["sync",4],["mixed",4]];
for(const [mode,notes] of cases){
  for(let repeat=0;repeat<rounds;repeat++)for(const index of onlyAfter?[1]:repeat%2?[1,0]:[0,1]){
    const core=new Core(modules[index],rate),p=structuredClone(presets[mode]);
    p.wavetable.engine_revision=routed?3:2;
    for(const o of p.wavetable.oscillators){if(!routed){delete o.fm_source;delete o.sync_source;}o.unison=unison;}
    core.request("/api/preset",p);core.request("/api/transport",{playing:true,midi_gate:true});
    for(let i=0;i<notes;i++)core.request("/api/note",{kind:"note_on",channel:0,note:48+Math.floor(i/4)*12+[0,3,7,10][i%4],velocity:100});
    for(let i=0;i<Math.ceil(rate*.5/block);i++)core.process(block);
    const times=[];let peak=0;
    for(let i=0;i<Math.ceil(rate*seconds/block);i++){
      const start=performance.now(),output=core.process(block);times.push(performance.now()-start);
      for(const value of output){assert.ok(Number.isFinite(value));peak=Math.max(peak,Math.abs(value));}
    }
    times.sort((a,b)=>a-b);
    runs.push({version:index?"after":"before",mode,notes,waves:4,unison,repeat,peak,
      p50_ms:times[Math.floor(times.length*.5)],p95_ms:times[Math.floor(times.length*.95)],p99_ms:times[Math.floor(times.length*.99)],max_ms:times.at(-1)});
    global.gc?.();
  }
  console.log(mode,notes);
}
const median=a=>{a.sort((a,b)=>a-b);return(a[Math.floor((a.length-1)/2)]+a[Math.floor(a.length/2)])/2;};
const summary=[];
for(const [mode,notes] of cases){
  const row={mode,notes,waves:4,unison};
  for(const version of onlyAfter?["after"]:["before","after"]){const rows=runs.filter(r=>r.mode===mode&&r.notes===notes&&r.version===version);row[version]=Object.fromEntries(["p50_ms","p95_ms","p99_ms","max_ms"].map(k=>[k,median(rows.map(r=>r[k]))]));}
  if(!onlyAfter)row.p50_reduction_percent=100*(1-row.after.p50_ms/row.before.p50_ms);row.after_p99_fits_deadline=row.after.p99_ms<block/rate*1000;summary.push(row);
}
await writeFile(report,JSON.stringify({scope:"Production shipping SIMD WASM in Node/V8, Core.process(128) including its JS boundary/output view. Preparation/compilation and signal validation excluded. Wall timing, median of two run percentiles; excludes the browser AudioWorklet/device scheduler. Separate from native thread CPU.",method:onlyAfter?"After only; four-node feedback with independent sync ring":"Sequential alternating AB/BA",node:process.version,v8:process.versions.v8,sample_rate:rate,block,deadline_ms:block/rate*1000,audio_seconds_per_run:seconds,warmup_audio_seconds:.5,rounds,binaries:paths.map((path,i)=>({path,sha256:createHash("sha256").update(bytes[i]).digest("hex")})).filter((_,i)=>!onlyAfter||i===1),summary,runs},null,2)+"\n");
