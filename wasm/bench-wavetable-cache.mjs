// Control-path CPU, separate from the render-only quantum benchmark.
import {writeFile,mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {cacheScenario} from './test-wavetable-cache.mjs';
const [before,after,out]=process.argv.slice(2);await mkdir(out,{recursive:true});
const runs=[];
for(const version of ['before','after','after','before']){
 const result=await cacheScenario(version==='before'?before:after,{repeats:8,expectCache:version==='after',invalidation:false});
 runs.push({version,...result});console.log(version,result);
}
await writeFile(resolve(out,'comparison.json'),JSON.stringify({scope:'Eight repeated full-preset edits on the same four-wave U4 FM+sync patch, each followed by a real 128-frame render. Shipping backend method + worklet handler + WASM; browser ports driven in-process. Total control/preparation/render thread CPU, not AudioWorklet callback timing. Initial bank load excluded.',method:'AB/BA, no concurrent builds or audio benchmarks from this task; unchanged bank compilation and transfer counts measured directly.',runs},null,2)+'\n');
