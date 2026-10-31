// Actual shipping WASM before/after, with the same migrated legacy patches.
// Run after build.py; native/CPU measurements use wavetable_opt separately.
import assert from "node:assert/strict";
import {readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {createHash} from "node:crypto";
const [before,folder,patches,report]=process.argv.slice(2);
const routed=process.argv[6]==="routing";
const {Core}=await import(pathToFileURL(resolve(folder,"core.mjs")));
const paths=[resolve(before),resolve(folder,"sad_pet.wasm"),resolve(folder,"sad_pet.simd.wasm")];
const bytes=await Promise.all(paths.map(p=>readFile(p)));
const modules=await Promise.all(bytes.map(b=>WebAssembly.compile(b)));
const presets=JSON.parse(await readFile(patches,"utf8"));
function render(module,preset,rate,notes){
  const core=new Core(module,rate),p=structuredClone(preset);
  p.wavetable.engine_revision=routed?3:2;
  if(!routed)for(const o of p.wavetable.oscillators){delete o.fm_source;delete o.sync_source;}
  const activate={};
  if(routed)p.wavetable.oscillators.forEach((o,i)=>{const l="abcd"[i];activate[`wt_${l}_fm_mode`]=o.fm_mode;activate[`wt_${l}_sync`]=+o.sync;o.fm_mode=0;o.sync=false;});
  core.request("/api/preset",p);core.request("/api/transport",{playing:true,midi_gate:true});
  const note=(i,on)=>core.request("/api/note",{kind:on?"note_on":"note_off",channel:0,note:48+Math.floor(i/4)*12+[0,3,7,10][i%4],velocity:on?100:0});
  for(let i=0;i<notes;i++)note(i,true);
  // The compatibility renderer's entry crossfade intentionally changed when
  // inactive/non-paired sources stopped falling back to B/D. Compare the
  // actual routed DSP after that transition has settled.
  if(routed){for(let i=0;i<2048;i++)core.process(1);core.request("/api/parameters",activate);for(let i=0;i<Math.ceil(rate*.5);i++)core.process(1);}
  const output=new Float32Array(12288*2);
  for(let at=0;at<12288;at++){
    if(at%128===0){const t=at/12288;core.request("/api/parameters",{wt_a_position:.1+t*.8,wt_d_position:.9-t*.7,wt_a_fm_index:.2+t*3.7,wt_c_fm_index:3.9-t*3.7,wt_a_sync_ratio:1.1+t*6.5,wt_d_voice_shift:.15-t*.3});}
    if(!routed&&at===1301)note(0,false);if(!routed&&at===2583)note(0,true);
    if(at===4097)core.request("/api/parameters",{wt_b_unison:2,wt_c_unison:8});
    if(at===7001)core.request("/api/parameters",{wt_a_fm_mode:2,wt_b_sync:1});
    if(at===10007)core.request("/api/parameters",{wt_a_fm_mode:0,wt_c_fm_mode:0,wt_b_sync:0});
    output.set(core.process(1),at*2);
  }
  assert.ok(output.every(Number.isFinite));return output;
}
const results=[];
for(const rate of [44100,48000,96000])for(const mode of routed?["fm","pm","sync","mixed"]:["off","fm","pm","sync","mixed"])for(const notes of [1,8]){
  const reference=render(modules[0],presets[mode],rate,notes);
  for(const [index,variant] of [[1,"scalar"],[2,"simd"]]){
    const actual=render(modules[index],presets[mode],rate,notes);let max=0,error=0,energy=0;
    for(let i=0;i<reference.length;i++){const d=actual[i]-reference[i];max=Math.max(max,Math.abs(d));error+=d*d;energy+=reference[i]*reference[i];}
    const dbr=10*Math.log10(Math.max(error,1e-60)/Math.max(energy,1e-60));
    const passed=max<=1e-5&&dbr<=-90;
    results.push({rate,mode,notes,waves:4,initial_unison:4,variant,max_abs_error:max,error_dbr:dbr,passed});
  }
  global.gc?.();
}
const output={scope:routed?"WASM routed operators before/after shared phase-advance optimization, scalar and SIMD; four-node feedback with independent sync ring, 24 held-note cases at 44.1/48/96 kHz with live controls/unison/mode edits. Both engines start with 2048 clean frames, activate the routes, then settle entry and downstream filters for 0.5 audio seconds. Note restarts are tested separately because their entry changed intentionally to remove incorrect fallback operators. No time/gain fitting.":"Shipping WASM engine revision 2 before vs revision 3 after, scalar and SIMD. Thirty identical sample-timed legacy patches at 44.1/48/96 kHz; no time/gain fitting. Separate from native comparisons and from independent alias measurements.",frames:12288,warmup:routed?{clean_frames:2048,routed_audio_seconds:.5,scope:"Held voices only; intentional new-note entry change is excluded"}:null,binaries:paths.map((path,i)=>({path,sha256:createHash("sha256").update(bytes[i]).digest("hex")})),criterion_max_abs_error:1e-5,criterion_error_dbr:-90,passed:results.every(r=>r.passed),results};
await writeFile(report,JSON.stringify(output,null,2)+"\n");
assert.ok(output.passed,JSON.stringify(results.filter(r=>!r.passed)));
console.log(`wavetable optimization: ${results.length} shipping scalar/SIMD before/after dynamic comparisons passed`);
