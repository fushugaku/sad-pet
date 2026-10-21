import {readFile,writeFile} from 'node:fs/promises';
import {Core} from '../../../wasm/web/core.mjs';
const root='target/perf/round3-20261021';
const modules=await Promise.all(['baseline-web/sad_pet.wasm','candidate.wasm'].map(async f=>WebAssembly.compile(await readFile(`${root}/${f}`))));
const median=values=>[...values].sort((a,b)=>a-b)[values.length>>1];
const results=[];
for(const rate of [44100,48000,96000,192000]) {
 const runs=[[],[]];let steps;
 for(let round=0;round<8;round++) for(const index of round%2?[1,0]:[0,1]) {
  const core=new Core(modules[index],rate), times=[];
  core.json('sad_timbre_prepare_begin',{kind:'drums'});
  for(let step=0;step<1000;step++) {
   core.process(128);
   const t=performance.now(),result=core.json('sad_timbre_prepare_step');
   times.push((performance.now()-t)*1000);
   if(result.done)break;
  }
  if(index)steps=times.length;
  if(round)runs[index].push(Math.max(...times));
  globalThis.gc?.();
 }
 const before=median(runs[0]),after=median(runs[1]);
 results.push({sample_rate:rate,before_max_step_us:before,after_max_step_us:after,
   reduction_percent:100*(1-after/before),steps,preparation_audio_time_ms:steps*128/rate*1000,runs});
}
await writeFile(`${root}/add-measurements.json`,JSON.stringify(results,null,2)+'\n');
console.log(results.map(({runs,...rest})=>rest));
