// First-note cost after a silent preset change, separately from held CPU.
import {readFile,writeFile} from 'node:fs/promises';
import {pathToFileURL} from 'node:url';
import {resolve} from 'node:path';
import {performance} from 'node:perf_hooks';
import {createHash} from 'node:crypto';
const [binary,folder,out]=process.argv.slice(2);
const {Core}=await import(pathToFileURL(resolve(folder,'core.mjs')));
const {preparedWavetable,wavetableIds}=await import(pathToFileURL(resolve(folder,'wavetables.mjs')));
const moduleBytes=await readFile(binary),module=await WebAssembly.compile(moduleBytes);
const preset=JSON.parse(await readFile('reports/wavetable/wasm-unison4/target-preset.json'));
const play=core=>{core.request('/api/transport',{playing:true,midi_gate:true});for(const note of [48,51,55,58])core.request('/api/note',{kind:'note_on',channel:0,note,velocity:100});};
const warm=new Core(module);warm.request('/api/preset',preset);play(warm);for(let i=0;i<750;i++)warm.process(128);
const utility=new Core(module),audio=new Core(module);audio.wasm.sad_wavetable_realtime_init();
const stage=id=>{
 const raw=utility.prepareWavetable({id}),b=preparedWavetable(raw);
 if(audio.json('sad_wavetable_stage_begin',b.header).cached)return;
 const bytes=new Uint8Array(raw,b.offset);
 for(let i=0;i<bytes.length;i+=32768){const part=bytes.subarray(i,i+32768);new Uint8Array(audio.wasm.memory.buffer,audio.wasm.sad_wavetable_upload(),part.length).set(part);audio.wasm.sad_wavetable_stage_chunk(part.length);}
 audio.json('sad_wavetable_stage_finish');
};
const initial=structuredClone(preset);initial.wavetable.count=1;
for(let i=0;i<4;i++)Object.assign(initial.wavetable.oscillators[i],{asset_id:'factory:wt-basic-morph:v1',enabled:i===0,unison:1,fm_mode:0,fm_index:0,sync:false});
for(const id of wavetableIds(initial))stage(id);audio.request('/api/preset',initial);
for(const id of wavetableIds(preset))stage(id);audio.request('/api/preset',preset);play(audio);
const times=[];
for(let i=0;i<32;i++){
 const cpu=process.threadCpuUsage(),t=performance.now();audio.process(128);const used=process.threadCpuUsage(cpu);
 times.push({block:i,wall_ms:performance.now()-t,cpu_ms:(used.user+used.system)/1000});
}
const result={scope:'Diagnostic: warmed compiled graph, Basic Morph startup then target, no rendered samples before the first four notes. First 32 quanta, including the initial bank transition; separate from steady-state AB/BA.',binary,module_sha256:createHash('sha256').update(moduleBytes).digest('hex'),times};
await writeFile(out,JSON.stringify(result,null,2)+'\n');console.log(times);
