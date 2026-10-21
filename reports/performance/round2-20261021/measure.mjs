import { readFile, writeFile } from 'node:fs/promises';
import { Core, TelemetryCore } from '../../../wasm/web/core.mjs';
const root='target/perf/round2-20261021';
const before=await WebAssembly.compile(await readFile(`${root}/baseline-web/sad_pet.wasm`));
const after=await WebAssembly.compile(await readFile(`${root}/candidate.wasm`));
const cores=[new Core(before),new Core(after)];
const results={initialization_ms:{before:[],after:[]},telemetry:{}};
const median=xs=>[...xs].sort((a,b)=>a-b)[xs.length>>1];
for(const core of cores) for(let i=0;i<16;i++)core.wasm.sad_init(48000);
for(let round=0;round<7;round++) {
  for(const index of round%2?[1,0]:[0,1]) {
    const times=[];
    for(let i=0;i<60;i++) {
      const start=performance.now(); cores[index].wasm.sad_init(48000);
      times.push(performance.now()-start);
    }
    results.initialization_ms[index?'after':'before'].push(median(times));
  }
}
for(const [name,module] of [['same_build',after],['before',before]]) {
  const legacy=new Core(module),light=name==='same_build'?new TelemetryCore(after):null;
  const audio=new Core(after);
  const initial={legacy_MiB:legacy.wasm.memory.buffer.byteLength/2**20,
    lightweight_MiB:light?.wasm.memory.buffer.byteLength/2**20};
  for(let i=1;i<4;i++)audio.request('/api/timbres/add',{kind:'granular'});
  for(let i=0;i<4;i++) {
    audio.request('/api/timbres/select',{id:i});audio.process(128);
    const bytes=new Uint8Array(audio.wasm.sad_snapshot_capacity());
    const n=audio.snapshot(bytes),packet=bytes.subarray(0,n);
    legacy.json('sad_snapshot_import_delta',packet);light?.snapshot(packet);
    audio.wasm.sad_snapshot_ack(audio.wasm.sad_snapshot_sequence(),1);
  }
  results.telemetry[name]={initial,four_timbres:{legacy_MiB:legacy.wasm.memory.buffer.byteLength/2**20,
    lightweight_MiB:light?.wasm.memory.buffer.byteLength/2**20}};
}
results.initialization_ms.median_before=median(results.initialization_ms.before);
results.initialization_ms.median_after=median(results.initialization_ms.after);
results.initialization_ms.reduction_percent=100*(1-results.initialization_ms.median_after/results.initialization_ms.median_before);
await writeFile(`${root}/measurements.json`,JSON.stringify(results,null,2)+'\n');
console.log(JSON.stringify(results,null,2));
