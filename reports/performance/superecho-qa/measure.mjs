import {Core} from './core.mjs';
onmessage=async()=>{let text='';try{
for(const [label,file] of [['Исходная','baseline.wasm'],['Оптимизированная','candidate.wasm']]){
 const module=await WebAssembly.compile(await(await fetch('./'+file)).arrayBuffer());
 const utility=new Core(module);utility.call('sad_prepare',utility.call('sad_factory',undefined,0));const source=utility.call('sad_pack');
 text+=label+' версия\n';postMessage({text,done:false});
 for(const [name,kinds,dirty] of [['Tape',['super_echo'],0],['BBD',['super_echo'],1],['2 × SuperEcho',['super_echo','super_echo'],0],['SuperEcho + AlphaVerb',['super_echo','alpha_verb'],0]]){
  const core=new Core(module);core.call('sad_load',source);const p=core.request('/api/preset'),s=core.request('/api/color-defaults').slot;
  p.pre_color_chain=[];p.source_color_chain=[];p.color_chain=kinds.map((kind,i)=>({...structuredClone(s),id:i+1,kind,enabled:true,mix:.7,settings:{...s.settings,super_echo_dirty:i===1?1:dirty,super_echo_time:.3,super_echo_feedback:.55,super_echo_shimmer:.6,super_echo_reverb:.4}}));
  core.request('/api/preset',p);core.request('/api/transport',{playing:true,midi_gate:false});
  for(let i=0;i<200;i++)core.process(128);
  const times=[];let energy=0;
  for(let i=0;i<750;i++){const start=performance.now();const samples=core.process(128);times.push(performance.now()-start);for(const v of samples)energy+=v*v;}
  times.sort((a,b)=>a-b);text+=`${name}: среднее ${(times.reduce((a,b)=>a+b,0)/times.length).toFixed(3)} мс; p95 ${times[Math.floor(times.length*.95)].toFixed(3)} мс; energy ${energy.toFixed(6)}\n`;
  postMessage({text,done:false});
 }
 text+='\n';
}
text+='Проверка завершена';postMessage({text,done:true});
}catch(error){postMessage({text:text+'\n'+error.stack,done:true});}};
