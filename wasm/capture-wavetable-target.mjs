// Listening pair and unaligned/unfitted numerical null test of the same stream.
import assert from "node:assert/strict";
import {readFile,writeFile,mkdir} from "node:fs/promises";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const [before,after,folder,out]=process.argv.slice(2);
await mkdir(out,{recursive:true});
const {Core}=await import(pathToFileURL(resolve(folder,"core.mjs")));
const preset=JSON.parse(await readFile("reports/wavetable/wasm-unison4/target-preset.json","utf8"));
const rate=48000,frames=153600,gain=16;
const modules=await Promise.all([before,after].map(async p=>WebAssembly.compile(await readFile(p))));
const waves=modules.map(module=>{
 const core=new Core(module,rate);core.request("/api/preset",preset);core.request("/api/transport",{playing:true,midi_gate:true});
 const notes=[48,51,55,58];for(const note of notes)core.request("/api/note",{kind:"note_on",channel:0,note,velocity:100});
 const audio=new Float32Array(frames*2);
 for(let at=0;at<frames;at+=128){
  if(at>=24000&&at<115200){const t=(at-24000)/(115200-24000);core.request("/api/parameters",{wt_a_position:.27+t*.6,wt_b_position:.4+t*.4,wt_c_position:.53-t*.4,wt_d_position:.66-t*.4});}
  if(at===115200)for(const note of notes)core.request("/api/note",{kind:"note_off",channel:0,note,velocity:0});
  audio.set(core.process(128),at*2);
 }assert.ok(audio.every(Number.isFinite));return audio;
});
let error=0,energy=0,max=0,peak=0;
for(let i=0;i<waves[0].length;i++){const d=waves[0][i]-waves[1][i];error+=d*d;energy+=waves[0][i]**2;max=Math.max(max,Math.abs(d));peak=Math.max(peak,Math.abs(waves[0][i]),Math.abs(waves[1][i]));}
assert.ok(peak*gain<1,"listening gain would clip");
for(let index=0;index<2;index++){
 const data=Buffer.alloc(44+frames*8);data.write("RIFF",0);data.writeUInt32LE(data.length-8,4);data.write("WAVEfmt ",8);data.writeUInt32LE(16,16);data.writeUInt16LE(3,20);data.writeUInt16LE(2,22);data.writeUInt32LE(rate,24);data.writeUInt32LE(rate*8,28);data.writeUInt16LE(8,32);data.writeUInt16LE(32,34);data.write("data",36);data.writeUInt32LE(frames*8,40);
 for(let i=0;i<waves[index].length;i++)data.writeFloatLE(waves[index][i]*gain,44+i*4);
 await writeFile(resolve(out,index?"after.wav":"before.wav"),data);
}
const error_dbr=10*Math.log10(Math.max(error,1e-60)/energy);
const result={scope:"Same complete 3.2 second note-on, held four-note FM/sync chord, moving all four table positions, note-off/release. Numerical comparison on raw float output, no fitting. Listening WAVs have the same fixed gain of 16 (+24.08 dB), with no clipping.",rate,frames,gain,peak,error_dbr,max_abs_error:max,passed:max<1e-6&&error_dbr<-100};
await writeFile(resolve(out,"listening.json"),JSON.stringify(result,null,2)+"\n");assert.ok(result.passed);console.log(result);
