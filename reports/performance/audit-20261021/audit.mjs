import { readFile, writeFile } from 'node:fs/promises';
import { performance } from 'node:perf_hooks';
import { Core } from '../../../wasm/web/core.mjs';

const wasm = await WebAssembly.compile(await readFile('dist/web/sad_pet.wasm'));
const results = [];
const timed = fn => { const t=performance.now(); const value=fn(); return [value, performance.now()-t]; };
const stats = xs => { xs.sort((a,b)=>a-b); return {median_us:xs[xs.length>>1],p99_us:xs[Math.floor(xs.length*.99)]}; };
let [utility, initMs] = timed(()=>new Core(wasm));
results.push({name:'utility-init',ms:initMs,memoryMiB:utility.wasm.memory.buffer.byteLength/2**20});
utility.call('sad_prepare',utility.call('sad_factory',undefined,0));
const prepared=utility.call('sad_pack');

for (const name of ['stopped','default','dense','resynth4','drums-stopped','drums','4-timbres','4-muted-timbres']) {
  const [core, ms]=timed(()=>new Core(wasm));
  core.call('sad_load',prepared);
  const commands=[];
  if(name.startsWith('drums')) {
    const [,addMs]=timed(()=>core.request('/api/timbres/add',{kind:'drums'}));
    commands.push({command:'add drums',ms:addMs});
    core.request('/api/timbres/update',{id:0,muted:true});
  }
  if(name.startsWith('4-')) for(let i=1;i<4;i++){
    const [,addMs]=timed(()=>core.request('/api/timbres/add',{kind:'granular'}));
    commands.push({command:'add granular',ms:addMs});
  }
  if(name==='4-muted-timbres') for(let i=0;i<4;i++)core.request('/api/timbres/update',{id:i,muted:true});
  if(name==='dense'){
    core.request('/api/parameters',{grains:1,grain_size:.9});
    core.request('/api/transport',{playing:true,midi_gate:true});
    for(const note of [36,48,55,60,67,72,79,91])core.request('/api/note',{kind:'note_on',channel:0,note,velocity:100});
  }else{
    if(name==='resynth4'){
      const p=core.request('/api/preset');
      p.resynth.enabled=true;p.resynth.count=4;
      core.request('/api/preset',p);
    }
    core.request('/api/transport',{playing:!name.endsWith('stopped'),midi_gate:false});
  }
  for(let i=0;i<500;i++)core.process(128);
  const times=[];let peak=0;
  for(let i=0;i<1200;i++){
    const [audio,elapsed]=timed(()=>core.process(128));
    times.push(elapsed*1000);
    for(const v of audio)peak=Math.max(peak,Math.abs(v));
  }
  const packet=new Uint8Array(core.wasm.sad_snapshot_capacity());
  const publisher=[],consumer=[],clones=[],lens=[];
  let stateBytes=0,firstPacket;
  for(let i=0;i<101;i++){
    const [length,elapsed]=timed(()=>core.snapshot(packet));
    const [decoded,workerElapsed]=timed(()=>utility.json('sad_snapshot_import',packet.subarray(0,length)));
    const [,cloneElapsed]=timed(()=>structuredClone(decoded.state));
    core.wasm.sad_snapshot_ack(core.wasm.sad_snapshot_sequence(),1);
    if(i===0)firstPacket={bytes:length,publish_us:elapsed*1000,import_us:workerElapsed*1000};
    else{ publisher.push(elapsed*1000);consumer.push(workerElapsed*1000);clones.push(cloneElapsed*1000);lens.push(length); }
    stateBytes=JSON.stringify(decoded.state).length;
  }
  const result={name,initMs:ms,commands,memoryMiB:core.wasm.memory.buffer.byteLength/2**20,process:stats(times),peak,telemetry:{firstPacket,packetBytes:Math.max(...lens),stateJsonBytes:stateBytes,publish:stats(publisher),import:stats(consumer),clone:stats(clones)}};
  results.push(result);console.log(JSON.stringify(result));
}
await writeFile('target/perf/audit-20261021/audit.json',JSON.stringify(results,null,2));
