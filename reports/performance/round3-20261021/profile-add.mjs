import {readFile,writeFile} from 'node:fs/promises';
import {Core} from '../../../wasm/web/core.mjs';
const binary=process.argv[2],out=process.argv[3];
const module=await WebAssembly.compile(await readFile(binary));
const results=[];
for(const rate of [44100,48000,96000,192000]) {
 const core=new Core(module,rate), times=[],pages=[];
 core.json('sad_timbre_prepare_begin',{kind:'drums'});
 for(let stage=0;stage<3000;stage++) {
  core.process(128);
  const memory=core.wasm.memory.buffer.byteLength,t=performance.now();
  const result=core.json('sad_timbre_prepare_step');
  times.push((performance.now()-t)*1000);pages.push((core.wasm.memory.buffer.byteLength-memory)/65536);
  if(result.done)break;
 }
 results.push({rate,steps:times.length,us:times,max_us:Math.max(...times),grew_pages:pages});
}
await writeFile(out,JSON.stringify(results,null,2)+'\n');
console.log(results.map(({rate,steps,max_us})=>({rate,steps,max_us})));
