// The shipping backend method, real worklet handler and real WASM. Only the
// browser message ports are driven in-process, without an audio device.
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import vm from 'node:vm';
import {performance} from 'node:perf_hooks';
export async function cacheScenario(folder,{repeats=4,expectCache=true,invalidation=true}={}){
 folder=resolve(folder);
 const {Core}=await import(pathToFileURL(resolve(folder,'core.mjs')));
 const {preparedWavetable,wavetableIds}=await import(pathToFileURL(resolve(folder,'wavetables.mjs')));
 const module=await WebAssembly.compile(await readFile(resolve(folder,'sad_pet.simd.wasm')));
 const utility=new Core(module,48000),pending=new Map();let Type,id=0,processor,preparations=0,transferred=0;
 class ProcessorBase { constructor(){this.port={postMessage:m=>{const p=pending.get(m.id);if(p){pending.delete(m.id);m.error?p.reject(Error(m.error)):p.resolve(m.result);}}};} }
 const source=(await readFile(resolve(folder,'worklet.mjs'),'utf8')).replace(/^import[^;]+;\s*/,'');
 vm.runInNewContext(source,{Core,sampleRate:48000,currentTime:0,currentFrame:0,AudioWorkletProcessor:ProcessorBase,registerProcessor:(_,c)=>Type=c,Uint8Array,Float32Array,Float64Array,ArrayBuffer,DataView,TextEncoder,TextDecoder,console});
 const audio={call:(type,data,transfer=[],path)=>new Promise((resolve,reject)=>{const n=++id;pending.set(n,{resolve,reject});processor.port.onmessage({data:{id:n,type,data,path}});})};
 const {BrowserBackend}=await import(pathToFileURL(resolve(folder,'browser-backend.mjs')));
 const backend=Object.create(BrowserBackend.prototype);backend.wavetableTail=Promise.resolve();backend.context={state:'suspended'};backend.audio=audio;
 backend.worker={call:async(type,data)=>{assert.equal(type,'wavetable-prepare');preparations++;const bytes=utility.prepareWavetable?utility.prepareWavetable(data):utility.call('sad_wavetable_prepare',data);transferred+=bytes.byteLength;return bytes;}};
 const preset=JSON.parse(await readFile('reports/wavetable/wasm-unison4/target-preset.json'));
 const ids=wavetableIds(preset);
 processor=new Type({processorOptions:{module}});
 const apply=()=>audio.call('request',preset,[],'/api/preset');
 await backend.stageWavetables(ids,apply);
 await audio.call('request',{playing:true,midi_gate:true},[],'/api/transport');
 for(const note of [48,51,55,58])await audio.call('request',{kind:'note_on',channel:0,note,velocity:100},[],'/api/note');
 processor.core.process(128);assert.equal(preparations,4);
 const preparedStart=preparations,bytesStart=transferred,cpu=process.threadCpuUsage(),startTime=performance.now();
 let peak=0;
 for(let i=0;i<repeats;i++){
  preset.wavetable.oscillators[0].position=.2+i*.03;
  await backend.stageWavetables(ids,apply);
  for(const x of processor.core.process(128)){assert.ok(Number.isFinite(x));peak=Math.max(peak,Math.abs(x));}
 }
 const used=process.threadCpuUsage(cpu),result={repeats,preparations:preparations-preparedStart,transferred_bytes:transferred-bytesStart,thread_cpu_ms:(used.user+used.system)/1000,wall_ms:performance.now()-startTime,peak};
 assert.ok(peak>1e-4);
 if(expectCache){assert.equal(result.preparations,0,'An unchanged bank was recompiled');assert.equal(result.transferred_bytes,0,'An unchanged bank was exported again');}
 if(invalidation){
  processor=new Type({processorOptions:{module}});
  const previous=preparations;
  await backend.stageWavetables(ids,apply);
  assert.equal(preparations-previous,4,'A stale browser header skipped missing audio banks');
  assert.equal(processor.core.request('/api/preset').wavetable.count,4);
 }
 return result;
}
if(import.meta.url===pathToFileURL(resolve(process.argv[1])).href){
 const result=await cacheScenario(process.argv[2]||'dist/web');console.log('Wavetable cache checks passed',result);
 const {BrowserBackend}=await import(pathToFileURL(resolve(process.argv[2]||'dist/web','browser-backend.mjs')));
 const backend=Object.create(BrowserBackend.prototype);let complete,resumes=0;const commands=[];
 backend.context={state:'suspended',resume:()=>{resumes++;return new Promise(resolve=>complete=resolve);}};
 backend.audio={call:async(type,data)=>{commands.push(data.kind);return {ok:true};}};
 globalThis.location={href:'http://sad.pet/'};
 const first=backend.resume();assert.equal(backend.resume(),first,'Concurrent gestures did not share the resume promise');
 const pending=backend.fetch('/api/note',{body:JSON.stringify({kind:'note_on',note:60,velocity:100})});
 const released=backend.fetch('/api/note',{body:JSON.stringify({kind:'note_off',note:60,velocity:0})});
 await Promise.resolve();await Promise.resolve();assert.equal(commands.length,0,'A note entered the engine before AudioContext resumed');
 backend.context.state='running';complete();await first;
 assert.equal((await pending).status,200);assert.equal((await released).status,200);
 assert.deepEqual(commands,['note_on','note_off']);assert.equal(resumes,1);
 console.log('Cold-start note ordering passed.');
}
