"use strict";
(() => {
  const LETTERS=["A","B","C","D"];
  const DEFAULT = "factory:wt-basic-morph:v1";
  const defaults = () => ({enabled:true,asset_id:DEFAULT,position:0,transpose:0,fine:0,level:1,pan:.5,
    phase:0,phase_random:0,unison:1,detune:10,spread:1,voice_shift:0,position_env:0,position_env_depth:0,surf_depth:1,step:false,key_waves:0,free_phase:false,position_lfo:0,position_lfo_depth:0,fm_mode:0,fm_index:0,sync:false,sync_ratio:1});
  const percent = v => `${Math.round(v*100)}%`;
  const signed = (v,unit,digits=1) => `${v>0?"+":""}${v.toFixed(digits)} ${unit}`;
  const node = (tag,cls,text) => {
    const e=document.createElement(tag); if(cls)e.className=cls; if(text!==undefined)e.textContent=text; return e;
  };
  const base64 = async file => {
    const bytes=new Uint8Array(await file.arrayBuffer());
    let text=""; for(let i=0;i<bytes.length;i+=16384) text+=String.fromCharCode(...bytes.subarray(i,i+16384));
    return btoa(text);
  };
  let auditionGeneration=Date.now()*1000;
  class Editor {
    constructor(root,hooks) {
      this.root=root; this.hooks=hooks; this.tables=[]; this.previews=new Map(); this.loaded=new Map(); this.active=0;
      root.innerHTML='<div class="section-heading"><h2>Wavetable <span>A–D</span></h2><button type="button" class="wt-add">+ Wave</button></div><div class="wt-tabs" role="tablist" aria-label="Wavetable oscillator"></div><div class="wt-oscillators"></div>';
      this.tabs=[0,1,2,3].map(i=>{
        const tab=node("button","",LETTERS[i]);tab.type="button";tab.role="tab";
        tab.id=`wt-tab-${i}`;tab.setAttribute("aria-controls",`wt-osc-${i}`);
        tab.onclick=()=>{this.active=i;this.sync();};
        tab.onkeydown=e=>{if(!["ArrowLeft","ArrowRight","Home","End"].includes(e.key))return;
          e.preventDefault();this.active=e.key==="Home"?0:e.key==="End"?this.hooks.read().wavetable.count-1:(this.active+(e.key==="ArrowLeft"?-1:1)+this.hooks.read().wavetable.count)%this.hooks.read().wavetable.count;this.sync();this.tabs[this.active].focus();};
        root.querySelector(".wt-tabs").append(tab);return tab;
      });
      this.tools=new window.SadWavetableTools(this);
      this.buildMotion();
      this.cards=[0,1,2,3].map(i=>this.card(i));
      this.add=root.querySelector(".wt-add");this.add.onclick=()=>this.layers("add");
      this.observer=new ResizeObserver(()=>this.drawAll());this.cards.forEach(c=>this.observer.observe(c.canvas));
      this.ready=this.catalog().catch(error=>this.error(0,error));
    }
    motion() {const s=this.hooks.read()?.wavetable;return s?(s.motion??={surf:0,xy_on:false,x:.5,y:.5}):{surf:0,xy_on:false,x:.5,y:.5};}
    editMotion(field,value) {const s=this.motion();this.motionInteraction=performance.now()+150;s[field]=field==="xy_on"?!!value:value;this.hooks.parameter(`wt_${field}`,Number(value));this.drawMotion();}
    buildMotion() {
      const root=node("div","wt-motion");root.innerHTML='<div class="wt-performance-knobs knob-grid"></div><div class="wt-xy-wrap"><button type="button" class="wt-xy-on" aria-pressed="false">XY mix</button><div class="wt-xy" role="group" aria-label="Mix waves A B C D"><span>A</span><span>B</span><span>C</span><span>D</span><button type="button" class="wt-puck" aria-label="XY position, use arrow keys"></button></div></div>';
      this.root.querySelector(".wt-tabs").before(root);this.motionRoot=root;
      for(const [field,label,min,max,reset] of [["surf","SURF",-1,1,0],["x","X",0,1,.5],["y","Y",0,1,.5]])this.hooks.knob([`wt_${field}`,label,percent,min,max,`Wavetable ${label}`],root.querySelector(".wt-performance-knobs"),{get:()=>this.motion()[field],set:v=>this.editMotion(field,v),reset:()=>reset,ariaScale:1});
      const on=root.querySelector(".wt-xy-on"),pad=root.querySelector(".wt-xy");on.onclick=()=>this.editMotion("xy_on",!this.motion().xy_on);on.dataset.controlKey="wt_xy_on";
      window.SadMidiLearn?.attach(on,"wt_xy_on","Wavetable XY",v=>this.editMotion("xy_on",v>=.5));
      const move=e=>{const rect=pad.getBoundingClientRect();this.editMotion("xy_on",true);this.editMotion("x",Math.max(0,Math.min(1,(e.clientX-rect.left)/rect.width)));this.editMotion("y",Math.max(0,Math.min(1,(e.clientY-rect.top)/rect.height)));};
      pad.onpointerdown=e=>{if(e.button!==0)return;e.preventDefault();pad.setPointerCapture(e.pointerId);move(e);};pad.onpointermove=e=>{if(pad.hasPointerCapture(e.pointerId))move(e);};pad.onpointerup=e=>{if(pad.hasPointerCapture(e.pointerId))pad.releasePointerCapture(e.pointerId);};
      root.querySelector(".wt-puck").onkeydown=e=>{if(!["ArrowLeft","ArrowRight","ArrowUp","ArrowDown"].includes(e.key))return;e.preventDefault();this.editMotion("xy_on",true);const field=["ArrowLeft","ArrowRight"].includes(e.key)?"x":"y",delta=["ArrowLeft","ArrowUp"].includes(e.key)?-.02:.02;this.editMotion(field,Math.max(0,Math.min(1,this.motion()[field]+delta*(e.shiftKey?5:1))));};
    }
    drawMotion() {if(!this.motionRoot)return;const s=this.liveMotion&&performance.now()>(this.motionInteraction??0)?this.liveMotion:this.motion(),pad=this.motionRoot.querySelector(".wt-xy"),puck=this.motionRoot.querySelector(".wt-puck");this.motionRoot.querySelector(".wt-xy-on").setAttribute("aria-pressed",String(!!s.xy_on));pad.classList.toggle("is-on",!!s.xy_on);puck.style.left=`${s.x*100}%`;puck.style.top=`${s.y*100}%`;puck.setAttribute("aria-valuetext",`X ${Math.round(s.x*100)}%, Y ${Math.round(s.y*100)}%`);pad.dataset.count=this.hooks.read()?.wavetable?.count??1;}
    settings(i) {return this.hooks.read()?.wavetable?.oscillators[i];}
    async catalog() {
      const catalog=await this.hooks.request("/api/wavetables");this.tables=catalog.tables;this.sync();
      if(catalog.storage_error)this.error(this.active,new Error(`Cannot read saved wavetables: ${catalog.storage_error}`));
    }
    groups() {
      const packs=new Map();for(const table of this.tables) {
        if(!packs.has(table.pack_name))packs.set(table.pack_name,[]);
        packs.get(table.pack_name).push([table.id,table.name]);
      }
      return [...packs].map(([label,choices])=>({label,choices}));
    }
    card(i) {
      const name=LETTERS[i],card=node("div","wt-oscillator");card.id=`wt-osc-${i}`;
      card.setAttribute("aria-label",`Wavetable ${name}`);
      card.innerHTML=`<div class="wt-osc-heading"><strong>${name}</strong><span class="wt-pack"></span><button type="button" class="power wt-power" aria-label="Wavetable ${name} enabled"><svg aria-hidden="true"><use href="#power-glyph" /></svg></button><button type="button" class="wt-remove" aria-label="Remove wave ${name}">×</button></div><div class="wt-picker"><button type="button" class="wt-previous" aria-label="Previous wavetable for ${name}">‹</button><button type="button" class="wt-table" aria-label="Wavetable ${name} table"></button><button type="button" class="wt-next" aria-label="Next wavetable for ${name}">›</button></div><div class="wt-wave"><canvas role="img" aria-label="Wavetable ${name} frames and current position"></canvas><output class="wt-frame"></output></div><div class="knob-grid wt-knobs"></div><div class="wt-selectors"><label>Unison<select class="wt-unison" aria-label="Wavetable ${name} unison voices">${Array.from({length:8},(_,n)=>`<option value="${n+1}">${n+1} ${n?"voices":"voice"}</option>`).join("")}</select></label><label>Position ENV<select class="wt-env" aria-label="Wavetable ${name} position envelope"><option value="0">Off</option>${Array.from({length:8},(_,n)=>`<option value="${n+1}">ENV ${n+1}${n===0?" · Amp":""}</option>`).join("")}</select></label></div><div class="knob-grid wt-unison-knobs"></div><details class="wt-phase"><summary>Phase</summary><div class="knob-grid wt-phase-knobs"></div></details><div class="wt-file-actions"><button type="button" class="wt-import">Import WAV</button><button type="button" class="wt-export">Export WAV</button><input type="file" accept=".wav,audio/wav" hidden></div><p class="wt-status" role="status" hidden></p><p class="wt-error" role="alert" hidden></p>`;
      this.root.querySelector(".wt-oscillators").append(card);
      const result={root:card,canvas:card.querySelector("canvas"),picker:card.querySelector(".wt-table"),loading:false,request:0};
      const knob=(field,label,format,min,max,bank,hostField=field,step)=>this.hooks.knob(
        [`wt_${name.toLowerCase()}_${hostField}`,label,format,min,max,`Wavetable ${name} ${label.toLowerCase()}`],card.querySelector(bank),{
          get:()=>this.settings(i)?.[field]??defaults()[field],set:v=>this.edit(i,field,v,hostField),reset:()=>defaults()[field],
          ariaScale:1,...(step?{step,quantize:v=>Math.round(v/step)*step}:{})});
      for(const [field,label,format,min,max,step] of [
        ["position","POSITION",v=>`${(v*100).toFixed(1)}%`,0,1], ["transpose","TRANSPOSE",v=>signed(v,"st"),-48,48,1],
        ["fine","FINE",v=>signed(v,"ct"),-100,100], ["level","LEVEL",percent,0,1],
        ["pan","PAN",v=>Math.abs(v-.5)<.005?"C":`${Math.round(Math.abs(v-.5)*200)}${v<.5?"L":"R"}`,0,1],
      ]) knob(field,label,format,min,max,".wt-knobs",field,step);
      knob("detune","DETUNE",v=>`${v.toFixed(1)} ct`,0,100,".wt-unison-knobs");
      knob("spread","SPREAD",percent,0,1,".wt-unison-knobs");
      knob("voice_shift","VOICE SHIFT",v=>signed(v*100,"%",1),-1,1,".wt-unison-knobs");
      knob("position_env_depth","ENV DEPTH",v=>signed(v*100,"%",0),-1,1,".wt-unison-knobs","env_depth");
      knob("phase","START PHASE",v=>`${Math.round(v*360)}°`,0,1,".wt-phase-knobs");
      knob("phase_random","RANDOM",percent,0,1,".wt-phase-knobs");
      const motion=node("details","wt-phase wt-local-motion");motion.innerHTML=`<summary>Motion</summary><div class="wt-selectors"><label>Frames<select class="wt-step" aria-label="Wavetable ${name} frame scanning"><option value="0">Smooth</option><option value="1">Step</option></select></label><label>Phase<select class="wt-free" aria-label="Wavetable ${name} phase mode"><option value="0">Retrigger</option><option value="1">Free-running</option></select></label></div><label class="wt-lfo-label">Position LFO<select class="wt-lfo" aria-label="Wavetable ${name} position LFO"><option value="0">Off</option>${Array.from({length:8},(_,n)=>`<option value="${n+1}">LFO ${n+1}</option>`).join("")}</select></label><div class="knob-grid wt-motion-knobs"></div>`;
      card.querySelector(".wt-file-actions").before(motion);
      knob("surf_depth","SURF DEPTH",v=>signed(v*100,"%",0),-1,1,".wt-motion-knobs");
      knob("position_lfo_depth","LFO DEPTH",v=>signed(v*100,"%",0),-1,1,".wt-motion-knobs","lfo_depth");
      card.querySelector(".wt-lfo").onchange=e=>this.edit(i,"position_lfo",+e.target.value,"lfo");
      knob("key_waves","KEY → WAVES",v=>signed(v*100,"% / 5 oct",0),-1,1,".wt-motion-knobs");
      for(const [selector,field] of [[".wt-step","step"],[".wt-free","free_phase"]]){const select=card.querySelector(selector),host=`wt_${name.toLowerCase()}_${field}`;select.onchange=()=>this.edit(i,field,select.value==="1");select.dataset.controlKey=host;window.SadMidiLearn?.attach(select,host,`Wavetable ${name} ${field}`,v=>this.edit(i,field,v>=.5));}
      const sources=LETTERS.map((letter,n)=>`<option value="${n}">${letter}</option>`).join("");
      const interaction=node("details","wt-phase wt-interaction");interaction.innerHTML=`<summary>FM / Sync</summary><div class="wt-selectors"><label>Modulation<select class="wt-fm-mode" aria-label="Wavetable ${name} modulation mode"><option value="0">Off</option><option value="1">Linear FM</option><option value="2">Phase modulation</option></select></label><label>FM source<select class="wt-fm-source" aria-label="Wavetable ${name} FM source">${sources}</select></label><label>Hard sync<select class="wt-sync" aria-label="Wavetable ${name} hard sync"><option value="0">Off</option><option value="1">On</option></select></label><label>Sync source<select class="wt-sync-source" aria-label="Wavetable ${name} sync source">${sources}</select></label></div><div class="knob-grid wt-interaction-knobs"></div>`;
      interaction.querySelector(".wt-fm-source").title="Operator signal before Level, pan and XY. Feedback connections use one internal sample of delay.";
      interaction.querySelector(".wt-sync-source").title="Reset from this wave's base clock when selecting itself, or from the selected wave's cycle crossings.";
      card.querySelector(".wt-file-actions").before(interaction);
      knob("sync_ratio","SYNC RATIO",v=>`×${v.toFixed(2)}`,1,8,".wt-interaction-knobs");
      knob("fm_index","INDEX",v=>v.toFixed(2),0,4,".wt-interaction-knobs");
      card.querySelector(".wt-sync").onchange=e=>this.edit(i,"sync",e.target.value==="1");
      card.querySelector(".wt-fm-mode").onchange=e=>this.edit(i,"fm_mode",+e.target.value);
      for(const [selector,key] of [[".wt-fm-source","fm_source"],[".wt-sync-source","sync_source"]]){
        const select=card.querySelector(selector),host=`wt_${name.toLowerCase()}_${key}`;
        select.onchange=e=>this.edit(i,key,+e.target.value);select.dataset.controlKey=host;
        window.SadMidiLearn?.attach(select,host,`Wavetable ${name} ${key}`,unit=>this.edit(i,key,Math.round(unit*3)));
      }
      card.querySelector(".wt-remove").onclick=()=>this.layers("remove",i);
      card.querySelector(".wt-power").onclick=()=>this.edit(i,"enabled",!this.settings(i)?.enabled);
      card.querySelector(".wt-unison").onchange=e=>this.edit(i,"unison",+e.target.value);
      card.querySelector(".wt-env").onchange=e=>this.edit(i,"position_env",+e.target.value,"env");
      for(const [selector,key,min,max,field] of [[".wt-unison","unison",1,8,"unison"],[".wt-env","env",0,8,"position_env"],[".wt-power","enabled",0,1,"enabled"]]) {
        const control=card.querySelector(selector),host=`wt_${name.toLowerCase()}_${key}`;control.dataset.controlKey=host;
        window.SadMidiLearn?.attach(control,host,`Wavetable ${name} ${key}`,unit=>this.edit(i,field,field==="enabled"?unit>=.5:Math.round(min+unit*(max-min)),key));
      }
      result.picker.onchange=()=>{const id=result.picker.value;window.SadLazySelect.setValue(result.picker,this.settings(i)?.asset_id);this.choose(i,id);};
      card.querySelector(".wt-previous").onclick=()=>this.adjacent(i,-1);
      card.querySelector(".wt-next").onclick=()=>this.adjacent(i,1);
      const input=card.querySelector('input[type="file"]');
      card.querySelector(".wt-import").onclick=()=>input.click();
      input.onchange=()=>{if(input.files[0])this.importFile(i,input.files[0]);input.value="";};
      card.querySelector(".wt-export").onclick=()=>this.export(i);
      const tools=node("div","wt-file-actions wt-tool-actions");tools.innerHTML='<button type="button" class="wt-edit-table">Edit table</button><button type="button" class="wt-maker-open">Wave Maker</button>';
      card.querySelector(".wt-file-actions").after(tools);tools.querySelector(".wt-edit-table").onclick=()=>this.tools.edit(i);tools.querySelector(".wt-maker-open").onclick=()=>this.tools.maker(i);
      card.ondragover=e=>{if(e.dataTransfer?.types.includes("Files")){e.preventDefault();card.classList.add("is-drop");}};
      card.ondragleave=e=>{if(!card.contains(e.relatedTarget))card.classList.remove("is-drop");};
      card.ondrop=e=>{e.preventDefault();card.classList.remove("is-drop");if(e.dataTransfer.files[0])this.importFile(i,e.dataTransfer.files[0]);};
      return result;
    }
    edit(i,field,value,hostField=field) {
      const s=this.settings(i);if(!s||this.layerBusy||i>=(this.hooks.read()?.wavetable?.count??2))return;s[field]=value;
      this.hooks.parameter(`wt_${LETTERS[i].toLowerCase()}_${hostField}`,Number(value));this.sync();
    }
    async layers(action,index) {
      if(this.layerBusy)return;const context=this.hooks.read();let changed=false;
      this.layerBusy=true;this.error(index??this.active,null);this.sync();
      try {await this.hooks.layers(action,index);changed=true;}catch(error){if(this.hooks.read()===context)this.error(index??this.active,error);}
      finally{this.layerBusy=false;this.sync();}
      if(changed&&this.hooks.read()===context) {
        const tab=this.tabs[this.active],target=tab.getClientRects().length?tab:this.cards[this.active].picker;
        target.focus({preventScroll:true});
      }
    }
    async adjacent(i,direction) {
      const at=this.tables.findIndex(t=>t.id===this.settings(i)?.asset_id);
      if(this.tables.length)this.choose(i,this.tables[(at+direction+this.tables.length)%this.tables.length].id);
    }
    status(i,text="") {const p=this.cards[i].root.querySelector(".wt-status");p.textContent=text;p.hidden=!text;}
    error(i,error) {const p=this.cards[i].root.querySelector(".wt-error");p.textContent=error?.message??String(error??"");p.hidden=!p.textContent;}
    async choose(i,id) {
      const card=this.cards[i],original=this.settings(i);if(!original||original.asset_id===id||card.loading)return;
      card.loading=true;this.status(i,`Loading ${this.tables.find(t=>t.id===id)?.name??"wavetable"}…`);this.error(i,null);this.sync();
      try {await this.hooks.select(i,id);if(this.settings(i)===original)original.asset_id=id;}
      catch(error){if(this.settings(i)===original)this.error(i,error);}finally{card.loading=false;this.status(i);this.sync();}
    }
    preview(id) {
      if(!this.previews.has(id))this.previews.set(id,this.hooks.request("/api/wavetables/preview",{id}).catch(error=>{this.previews.delete(id);throw error;}));
      return this.previews.get(id);
    }
    async audition(id,notify) {
      if(this.auditionId===id)return this.stopAudition();
      this.stopAudition();const generation=++auditionGeneration;this.auditionId=id;this.auditionNotify=notify;notify("loading");
      try {await this.hooks.request("/api/wavetables/audition",{id,generation});
        if(this.auditionId!==id||generation!==auditionGeneration)return;notify("playing");
        this.auditionTimer=setTimeout(()=>{if(this.auditionId===id){this.auditionId=null;notify("idle");}},800);
      }catch(error){if(generation===auditionGeneration){notify("idle");this.error(this.active,error);this.auditionId=null;}}
    }
    stopAudition() {
      if(!this.auditionId)return;this.auditionId=null;clearTimeout(this.auditionTimer);this.auditionNotify?.("idle");
      this.hooks.request("/api/wavetables/audition",{id:null,generation:++auditionGeneration}).catch(()=>{});
    }
    sync() {
      const settings=this.hooks.read()?.wavetable;this.root.hidden=!settings;
      if(this.dialog&&this.settings(this.dialog.waveIndex)!==this.dialog.waveSettings)this.dialog.close();
      if(!settings){this.stopAudition();return;}
      this.drawMotion();
      const count=settings.count??2;this.active=Math.min(this.active,count-1);
      this.root.querySelector(".wt-oscillators").dataset.count=String(count);
      this.add.disabled=count>=4||this.layerBusy;this.add.title=count>=4?"Maximum four waves":"Add a wave";
      this.tabs.forEach((tab,i)=>{tab.hidden=i>=count;tab.setAttribute("aria-selected",String(i===this.active));tab.tabIndex=i===this.active?0:-1;});
      this.cards.forEach((card,i)=>{
        card.root.hidden=i>=count;if(i>=count)return;
        const s=this.settings(i);card.root.querySelector(".wt-remove").disabled=count<=1||this.layerBusy;card.root.classList.toggle("is-inactive-tab",i!==this.active);card.root.classList.toggle("is-off",!s.enabled);
        card.root.querySelector(".wt-power").setAttribute("aria-pressed",String(s.enabled));
        for(const [selector,value] of [[".wt-unison",s.unison],[".wt-env",s.position_env],[".wt-step",Number(s.step??false)],[".wt-free",Number(s.free_phase??false)],[".wt-lfo",s.position_lfo??0],[".wt-sync",Number(s.sync??false)],[".wt-fm-mode",s.fm_mode??0],[".wt-fm-source",s.fm_source<4?s.fm_source:i^1],[".wt-sync-source",s.sync_source<4?s.sync_source:i] ]) {
          const select=card.root.querySelector(selector);if(document.activeElement!==select)select.value=String(value);
        }
        for(const select of card.root.querySelectorAll(".wt-fm-source,.wt-sync-source"))for(const option of select.options)option.disabled=Number(option.value)>=count;
        const table=this.tables.find(t=>t.id===s.asset_id);
        card.root.querySelector(".wt-pack").textContent=table?.pack_name??"";
        window.SadLazySelect.configure(card.picker,s.asset_id,this.groups(),{play:(id,notify)=>this.audition(id,notify),stop:()=>this.stopAudition()});
        for(const button of card.root.querySelectorAll(".wt-picker button,.wt-import"))button.disabled=card.loading;
        card.picker.setAttribute("aria-busy",String(card.loading));
        if(card.asset!==s.asset_id){
          card.asset=s.asset_id;card.preview=null;const id=s.asset_id;
          this.preview(id).then(preview=>{if(this.settings(i)?.asset_id===id){card.preview=preview;this.draw(i);}}).catch(error=>{if(card.asset===id)this.error(i,error);});
        }
        this.draw(i);
      });
    }
    update(positions,motion) {this.positions=positions;this.liveMotion=motion;this.drawMotion();this.drawAll();}
    drawAll(){this.cards?.forEach((_,i)=>this.draw(i));}
    draw(i) {
      const card=this.cards[i];if(!card?.preview||this.root.hidden||card.root.hidden||document.hidden||this.root.classList.contains("rack-folded"))return;
      const canvas=card.canvas,width=canvas.clientWidth,height=canvas.clientHeight;if(!width||!height)return;
      const ratio=Math.min(2,window.devicePixelRatio||1),position=this.positions?.[i]??{current:this.settings(i)?.position??0,min:this.settings(i)?.position??0,max:this.settings(i)?.position??0};
      const signature=[card.asset,width,height,ratio,position.current,position.min,position.max].join(":");if(signature===card.drawn)return;card.drawn=signature;
      canvas.width=Math.round(width*ratio);canvas.height=Math.round(height*ratio);
      const c=canvas.getContext("2d");c.scale(ratio,ratio);c.clearRect(0,0,width,height);
      const waves=card.preview.waves,frames=waves.length,now=Math.max(0,Math.min(1,position.current)),count=Math.min(10,frames);
      const path=(wave,depth,color,lineWidth)=>{
        const left=12+depth*width*.11,right=width-12-(1-depth)*width*.11,center=height*.59-depth*height*.23,amplitude=height*.23;
        c.strokeStyle=color;c.lineWidth=lineWidth;c.beginPath();
        wave.forEach((v,k)=>{const x=left+k/(wave.length-1)*(right-left),y=center-v*amplitude;k?c.lineTo(x,y):c.moveTo(x,y);});c.stroke();
      };
      for(let f=count-1;f>=0;f--){const index=count===1?0:Math.round(f/(count-1)*(frames-1));path(waves[index],count===1?0:f/(count-1),"#39473c",1);}
      const frame=now*(frames-1),a=Math.floor(frame),b=Math.min(frames-1,a+1),t=frame-a;
      path(waves[a].map((v,k)=>v+(waves[b][k]-v)*t),now,"#c4f486",1.7);
      const span=width-24,y=height-13;c.fillStyle="#303831";c.fillRect(12,y,span,2);
      const low=Math.max(0,Math.min(1,position.min)),high=Math.max(0,Math.min(1,position.max));
      c.fillStyle="#8cdcd155";c.fillRect(12+low*span,y-4,Math.max(2,(high-low)*span),10);
      c.fillStyle="#8cdcd1";c.fillRect(11+now*span,y-5,2,12);
      card.root.querySelector(".wt-frame").textContent=`${Math.round(frame)+1} / ${frames}`;
    }
    async importFile(i,file) {
      const original=this.settings(i);
      this.error(i,null);
      try {
        if(file.size>16*1024*1024)throw new Error("The wavetable WAV exceeds 16 MiB.");
        const wav=await base64(file),info=await this.hooks.request("/api/wavetables/import",{wav,inspect:true});
        if(this.settings(i)!==original)return;
        const input={wav,name:file.name.replace(/\.wav$/i,"")};
        if(info.channels===1&&info.frame_size) return await this.applyImport(i,{...input,frame_size:info.frame_size});
        this.importDialog(i,input,info);
      }catch(error){if(this.settings(i)===original)this.error(i,error);}
    }
    async applyImport(i,input) {
      const original=this.settings(i);
      const imported=await this.hooks.request("/api/wavetables/import",input);this.previews.set(imported.id,Promise.resolve(imported));
      await this.catalog();if(this.settings(i)===original)await this.choose(i,imported.id);
    }
    importDialog(i,input,info) {
      this.dialog?.close();const dialog=node("dialog","wt-import-dialog");this.dialog=dialog;
      dialog.waveIndex=i;dialog.waveSettings=this.settings(i);
      dialog.innerHTML='<form method="dialog"><div class="wt-dialog-heading"><h2>Import wavetable</h2><button type="button" aria-label="Close import">×</button></div><p class="wt-import-name"></p><div class="wt-selectors"><label>Frame size<select aria-label="Import frame size"></select></label><label class="wt-channel">Channel<select aria-label="Import stereo channel"><option value="left">Left</option><option value="right">Right</option><option value="mix">Mix</option></select></label></div><p class="wt-import-frames" role="status"></p><canvas class="wt-import-wave" role="img" aria-label="Imported table preview"></canvas><p class="wt-error" role="alert" hidden></p><div class="wt-dialog-actions"><button type="button" class="wt-preview" aria-pressed="false">Preview</button><button type="button" class="wt-cancel">Cancel</button><button type="button" class="wt-apply">Apply</button></div></form>';
      dialog.querySelector(".wt-import-name").textContent=input.name;
      const period=dialog.querySelector('[aria-label="Import frame size"]'),channel=dialog.querySelector('[aria-label="Import stereo channel"]'),apply=dialog.querySelector(".wt-apply"),error=dialog.querySelector(".wt-error");
      for(const size of [128,256,512,1024,2048,4096]) {
        const option=node("option","",String(size));option.value=String(size);option.disabled=info.samples_per_channel%size!==0||info.samples_per_channel/size>256||info.samples_per_channel<size;period.append(option);
      }
      period.value=String(info.frame_size??([2048,1024,512,256,128,4096].find(size=>info.samples_per_channel%size===0&&info.samples_per_channel/size<=256&&info.samples_per_channel>=size)??2048));
      dialog.querySelector(".wt-channel").hidden=info.channels===1;
      let previewGeneration=0,previewId=null,temporaryGeneration=0;
      const audition=dialog.querySelector(".wt-preview");audition.disabled=true;
      audition.onclick=()=>previewId&&this.audition(previewId,status=>{audition.textContent=status==="loading"?"Loading…":status==="playing"?"Stop":"Preview";audition.setAttribute("aria-pressed",String(status!=="idle"));});
      const values=()=>({...input,frame_size:+period.value,...(info.channels===2?{channel:channel.value}:{})});
      const preview=async()=>{
        const frames=info.samples_per_channel/+period.value,valid=Number.isInteger(frames)&&frames>=1&&frames<=256;
        apply.disabled=!valid;dialog.querySelector(".wt-import-frames").textContent=valid?`${frames} ${frames===1?"frame":"frames"}`:"Choose a frame size that gives 1–256 complete frames.";
        this.stopAudition();previewId=null;audition.disabled=true;
        if(!valid)return;const generation=++previewGeneration;temporaryGeneration=++auditionGeneration;
        try {const p=await this.hooks.request("/api/wavetables/import",{...values(),preview:true,generation:temporaryGeneration});if(generation!==previewGeneration)return;
          const canvas=dialog.querySelector("canvas"),c=canvas.getContext("2d");canvas.width=640;canvas.height=160;c.clearRect(0,0,640,160);
          c.strokeStyle="#c4f486";c.beginPath();p.waves[0].forEach((v,k)=>{const x=8+k/(p.waves[0].length-1)*624,y=80-v*65;k?c.lineTo(x,y):c.moveTo(x,y);});c.stroke();
          previewId=p.id;audition.disabled=false;error.hidden=true;
        }catch(e){if(generation===previewGeneration){error.textContent=e.message;error.hidden=false;apply.disabled=true;}}
      };
      period.onchange=channel.onchange=preview;
      apply.onclick=async()=>{apply.disabled=true;try{await this.applyImport(i,values());dialog.close();}catch(e){error.textContent=e.message;error.hidden=false;apply.disabled=false;}};
      const close=()=>dialog.close();dialog.querySelector('.wt-dialog-heading button').onclick=close;dialog.querySelector('.wt-cancel').onclick=close;
      const previous=document.activeElement;dialog.onclose=()=>{previewGeneration++;this.stopAudition();this.hooks.request("/api/wavetables/preview-clear",{generation:temporaryGeneration}).catch(()=>{});dialog.remove();if(this.dialog===dialog)this.dialog=null;if(previous?.isConnected)previous.focus({preventScroll:true});};
      document.body.append(dialog);dialog.showModal();period.focus();preview();
    }
    async export(i) {
      try {
        const id=this.settings(i).asset_id;
        if(window.sadPetNative) {
          const name=(this.tables.find(t=>t.id===id)?.name??"Wavetable").replace(/[\\/:*?"<>|]/g,"_");
          await window.sadPetNative.dialog("/api/wavetables/export",{id},
            {mode:"save",title:"Export wavetable",filename:`${name}.wav`,patterns:"*.wav"});
          return;
        }
        const result=await this.hooks.request("/api/wavetables/export",{id});
        const bytes=Uint8Array.from(atob(result.wav),c=>c.charCodeAt(0));await this.hooks.download(new Blob([bytes],{type:"audio/wav"}),result.name);
      }catch(error){this.error(i,error);}
    }
  }
  window.SadWavetable={Editor,nextGeneration:()=>++auditionGeneration,defaults:()=>({schema_version:2,engine_revision:3,seed:"6292746119127516500",count:1,
    oscillators:LETTERS.map((_,i)=>({...defaults(),enabled:i===0,fm_source:i^1,sync_source:i}))})};
})();
