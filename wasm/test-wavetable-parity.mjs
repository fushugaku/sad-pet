// Sample-timed four-wave input stream: compare shipping scalar/SIMD and
// arbitrary host blocks. Compilation here happens before audio rendering;
// test-wavetable.mjs separately exercises the cache-only worklet transfer.
import assert from "node:assert/strict";
import {readFile,writeFile} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const folder=resolve(process.argv[2]||"dist/web");
const unison4=process.argv[4]==="unison4";
const advanced=process.argv[4]==="thirdwave"||unison4;
const {Core}=await import(pathToFileURL(resolve(folder,"core.mjs")));
const modules=await Promise.all(["sad_pet.wasm","sad_pet.simd.wasm"].map(async name=>
  WebAssembly.compile(await readFile(resolve(folder,name)))));
const utility=new Core(modules[0]);
utility.request("/api/timbres/add",{kind:"wavetable"});
const preset=unison4?JSON.parse(await readFile(new URL("../reports/wavetable/wasm-unison4/target-preset.json",import.meta.url),"utf8")):utility.request("/api/preset");
preset.wavetable.count=4;
const ids=["basic-morph","vowel-sweep","additive-glass","phase-spiral"];
if(!unison4)preset.wavetable.oscillators.forEach((o,i)=>Object.assign(o,{
  asset_id:`factory:wt-${ids[i]}:v1`,enabled:true,level:.28,pan:.2+i*.2,
  unison:i+1,detune:7.3,spread:.7,position:.13+i*.2,phase:.12,phase_random:.5,
}));
const note=(kind,note)=>({kind,channel:0,note,velocity:kind==="note_on"?100:0});
const events=[
  [0,"/api/note",note("note_on",48)], [0,"/api/note",note("note_on",55)],
  [0,"/api/note",note("note_on",60)], [0,"/api/note",note("note_on",64)],
  [117,"/api/parameters",{wt_c_position:.9,wt_d_position:.2}],
  [511,"/api/note",{kind:"pitch_bend",channel:0,value:10001}],
  [997,"/api/parameters",{wt_d_transpose:7,wt_c_unison:4}],
  [1741,"/api/parameters",{wt_a_level:.41,wt_c_env:2,wt_c_env_depth:.7}],
  [2403,"/api/note",note("note_off",55)], [3001,"/api/note",note("note_on",67)],
  [4097,"/api/parameters",{wt_b_enabled:0,wt_c_pan:.2}],
  [6003,"/api/parameters",{wt_b_enabled:1,wt_b_unison:8}],
];
if(advanced){
  preset.envs[1].wave={enabled:true,beats:true,times:[.03,.1,.15,.1,.05,.1],levels:[1,.2,.7,.3,.1,0],curves:[.5,-.5,0,.3,0,0],loop_on:true,loop_start:1,loop_end:3};
  events.push(
    [281,"/api/parameters",{wt_surf:.15,wt_xy_on:1,wt_x:.3,wt_y:.7,wt_a_voice_shift:.12,wt_d_voice_shift:-.08,wt_a_step:1,lfo1_poly:1,wt_c_lfo:1,wt_c_lfo_depth:.2}],
    [1457,"/api/parameters",{wt_a_fm_mode:1,wt_a_fm_index:.7,wt_c_key_waves:-.2,wt_d_free_phase:1}],
    [3607,"/api/parameters",{wt_a_fm_mode:2,wt_a_fm_index:1.2,wt_d_voice_shift:.18}],
    [7201,"/api/parameters",{wt_a_sync:1,wt_a_sync_ratio:2.75}],
    [7807,"/api/parameters",{wt_a_fm_source:3,wt_b_fm_mode:1,wt_b_fm_index:.6,wt_b_fm_source:0,wt_c_fm_mode:2,wt_c_fm_index:.5,wt_c_fm_source:1,wt_d_fm_mode:1,wt_d_fm_index:.4,wt_d_fm_source:2,wt_a_sync_source:2,wt_b_sync:1,wt_b_sync_source:3}],
  );events.sort((a,b)=>a[0]-b[0]);
}
const frames=8192;
function render(module,rate,block) {
  const core=new Core(module,rate);
  core.request("/api/preset",preset);
  core.request("/api/transport",{playing:true,midi_gate:true});
  const output=new Float32Array(frames*2);
  let at=0,event=0;
  while(at<frames) {
    while(event<events.length&&events[event][0]===at) {
      core.request(events[event][1],events[event][2]);event++;
    }
    const end=Math.min(frames,at+block,events[event]?.[0]??frames);
    output.set(core.process(end-at),at*2);at=end;
  }
  assert.ok(output.every(Number.isFinite));
  assert.ok(output.some(x=>Math.abs(x)>1e-4),"four-wave stream is silent");
  return output;
}
function difference(a,b) {
  let max=0;
  for(let i=0;i<a.length;i++)max=Math.max(max,Math.abs(a[i]-b[i]));
  return max;
}
const results=[];
for(const rate of advanced?[48000]:[44100,48000,96000]) {
  const reference=render(modules[0],rate,128);
  for(const block of advanced?[16,128,2048]:[16,32,64,128,256,512,1024,2048]) {
    const scalar=difference(reference,render(modules[0],rate,block));
    const simd=difference(reference,render(modules[1],rate,block));
    assert.ok(scalar<=1e-6,`block-dependent scalar output: ${rate}/${block}: ${scalar}`);
    assert.ok(simd<=(unison4?1e-6:2e-5),`SIMD output differs: ${rate}/${block}: ${simd}`);
    results.push({sample_rate:rate,block,scalar_max_abs:scalar,simd_max_abs:simd});
  }
}
const report={scope:unison4?"Four complex waves, four notes, unison 4, full FM/sync rings followed by live PM/parameter changes and U8 fallback; shipping scalar/SIMD and host blocks":advanced?"Four waves with Voice shift, Wave ENV, poly LFO, Surf/Step/Keys/XY, FM→PM→sync→arbitrary sources/feedback; shipping WASM scalar/SIMD and host blocks, excludes native backend":"Four waves, shipping WASM scalar/SIMD, sample-timed notes/bend/controls; excludes native backend",frames,events:events.length,passed:true,results};
if(process.argv[3])await writeFile(process.argv[3],JSON.stringify(report,null,2)+"\n");
console.log(`wavetable: ${results.length} scalar/SIMD and host-block comparisons passed`);
