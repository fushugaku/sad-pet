"use strict";
(() => {
  const el = (tag, cls, text) => {
    const node = document.createElement(tag);
    if (cls) node.className = cls;
    if (text !== undefined) node.textContent = text;
    return node;
  };
  const percent = v => `${Math.round(v * 100)}%`;
  const signed = (v, unit) => `${v > 0 ? "+" : ""}${v.toFixed(1)} ${unit}`;
  const reorder=(parent,nodes)=>{let at=parent.firstElementChild;for(const node of nodes){if(node===at)at=at.nextElementSibling;else parent.insertBefore(node,at);}};
  const TYPES = [["radias", "RADIAS"], ["wavefolder", "Wavefolder"], ["pcm", "PCM"], ["wavetable", "Wavetable"]];
  const WAVES = [["saw", "Saw"], ["pulse", "Pulse"], ["triangle", "Triangle"], ["sine", "Sine"], ["formant", "Formant"], ["noise", "Noise"], ["audio_in", "Audio In"]];
  const MODES = [["wave", "Wave"], ["cross", "Cross"], ["unison", "Unison"], ["vpm", "VPM"]];
  const FOLDS = [["triangle", "Triangle"], ["sine", "Sine"], ["soft", "Soft"], ["asymmetric", "Asymmetric"], ["rectify", "Rectify"]];
  const select = (label, choices, change, cls = "") => {
    const wrap = el("label", cls), control = el("select");
    wrap.append(el("span", "", label), control);
    control.setAttribute("aria-label", label);
    for (const [value, text] of choices) {
      const option = el("option", "", text); option.value = String(value); control.append(option);
    }
    control.onchange = () => change(control.value);
    return { wrap, control };
  };
  const button = (label, action, cls = "") => {
    const node = el("button", cls, label); node.type = "button"; node.onclick = action; return node;
  };
  const transport = () => ({enabled:false, window_seconds:.04, rolling:true, seed:1396786256, rate:1});
  const slotDefault = () => ({id:0, kind:"radias", enabled:true, level:.65, pan:.5, transpose:0, fine:0,
    native:{enabled:true, model:"radias", waveform:"saw", mode:"wave", control1:0, control2:0,
      level:.7, pan:.5, transpose:0, fine:0, cross_source:0, cross_input:"analysis", fold_shape:"triangle", interaction:"off", interaction_source:0},
    wavetable:{...window.SadWavetable.defaults().oscillators[0], fm_source:0, sync_source:0},
    asset_id:"factory:wt-basic-morph:v1"});
  const defaults = () => ({version:1, next_id:2, slots:Array.from({length:8}, (_,i)=>({...slotDefault(), id:i===0?1:0})),
    order:[0,1,2,3,4,5,6,7], motion:{surf:0,xy_on:false,x:.5,y:.5}, xy_slots:[1,0,0,0]});

  class Editor {
    constructor(root, hooks) {
      this.root = root; this.hooks = hooks; this.active = 0;
      this.tables = []; this.previews = new Map(); this.loaded = new Map();
      root.innerHTML = '<div class="section-heading"><h2>Oscillators <span>8 slots</span></h2><button type="button" class="osc-add">+ Oscillator</button></div><div class="osc-tabs" role="tablist" aria-label="Oscillators"></div><div class="osc-motion"></div><div class="osc-slots"></div>';
      this.tools = new window.SadWavetableTools(this);
      this.cards = Array.from({length:8}, (_,i)=>this.card(i));
      this.tabs = this.cards.map((card,i)=>{
        const tab = button(`OSC ${i+1}`, ()=>{this.active=i;this.sync();});
        tab.id = `osc-tab-${i}`;tab.role = "tab";tab.setAttribute("aria-controls",card.root.id);
        tab.onkeydown = e => {
          if (!["ArrowLeft","ArrowRight","Home","End"].includes(e.key)) return;
          e.preventDefault();const order=this.order(),at=order.indexOf(this.active);
          this.active=order[e.key==="Home"?0:e.key==="End"?order.length-1:(at+(e.key==="ArrowLeft"?-1:1)+order.length)%order.length];
          this.sync();this.tabs[this.active].focus();
        };
        root.querySelector(".osc-tabs").append(tab);return tab;
      });
      root.querySelector(".osc-add").onclick = () => this.add();
      this.buildMotion();
      this.observer = new ResizeObserver(()=>this.drawAll());
      this.cards.forEach(c=>this.observer.observe(c.canvas));
      this.ready = this.catalog().catch(e=>this.error(0,e));
    }
    bank() { return this.hooks.read()?.oscillators; }
    slot(i) { return this.bank()?.slots[i]; }
    order() { return (this.bank()?.order??[]).filter(i=>this.slot(i)?.id); }
    settings(i) { const slot=this.slot(i);if(!slot?.id||slot.kind!=="wavetable")return null;slot.wavetable.asset_id=slot.asset_id;return slot.wavetable; }
    change() { this.hooks.structure();this.sync();this.hooks.changed?.(); }
    edit(i, field, value, group="") {
      const slot=this.slot(i);if(!slot?.id)return;
      const target=group?slot[group]:slot;target[field]=value;
      let numeric=typeof value==="boolean"?Number(value):value;
      const options=field==="waveform"?WAVES:field==="mode"?MODES:field==="fold_shape"?FOLDS:null;
      if(options)numeric=options.findIndex(([key])=>key===value);
      this.hooks.parameter(`osc${i+1}_${group==="wavetable"?"wt_":group==="native"?"native_":""}${field}`,numeric);
      this.sync();
    }
    add() {
      const bank=this.bank(),i=bank?.slots.findIndex(s=>!s.id);if(i===undefined||i<0)return;
      bank.slots[i]={...slotDefault(),id:bank.next_id++};this.active=i;this.change();
      this.tabs[i].focus({preventScroll:true});
    }
    remove(i) {
      const bank=this.bank(),id=this.slot(i)?.id;if(!id)return;
      for(const route of this.hooks.read().interactions?.routes??[]) {
        if((route.source.kind==="osc"&&route.source.id===id)||(route.target.kind==="osc"&&route.target.id===id))Object.assign(route,routeDefault(),{enabled:false});
      }
      for(const slot of bank.slots){
        if(slot.native.cross_source===i+1)slot.native.cross_source=0;
        if(slot.wavetable.fm_source===i){slot.wavetable.fm_mode=0;slot.wavetable.fm_index=0;}
        if(slot.wavetable.sync_source===i)slot.wavetable.sync=false;
      }
      for(const track of this.hooks.read().sequencer?.tracks??[])if(track.target.kind==="mixed_oscillator"&&track.target.index===i)track.target={kind:"parameter",parameter:""};
      bank.xy_slots=bank.xy_slots.map(x=>x===id?0:x);bank.slots[i]=slotDefault();
      this.active=this.order()[0]??0;this.change();
    }
    move(i,delta) {
      const order=this.bank().order,live=this.order(),other=live[live.indexOf(i)+delta];if(other===undefined)return;
      const a=order.indexOf(i),b=order.indexOf(other);[order[a],order[b]]=[order[b],order[a]];this.change();
    }
    card(i) {
      const root=el("article","osc-slot wt-oscillator");root.id=`osc-slot-${i}`;root.setAttribute("aria-labelledby",`osc-tab-${i}`);
      root.innerHTML=`<div class="osc-heading"><strong>OSC ${i+1}</strong><div class="osc-type"></div><button type="button" class="power osc-power" aria-label="OSC ${i+1} enabled"><svg aria-hidden="true"><use href="#power-glyph" /></svg></button><button type="button" class="osc-left" aria-label="Move OSC ${i+1} left">‹</button><button type="button" class="osc-right" aria-label="Move OSC ${i+1} right">›</button><button type="button" class="osc-remove" aria-label="Remove OSC ${i+1}">×</button></div><div class="osc-common knob-grid"></div><div class="osc-native"><div class="osc-pcm-source"><button type="button" class="osc-pcm-file" aria-label="OSC ${i+1} sample"></button><button type="button" class="osc-pcm-import">Import</button></div><div class="osc-native-selects osc-selects"></div><div class="osc-native-knobs knob-grid"></div><details class="osc-transport"><summary>Phase transport</summary><div class="osc-transport-selects osc-selects"></div></details></div><div class="osc-wt"><div class="wt-picker"><button type="button" class="wt-table" aria-label="OSC ${i+1} wavetable"></button></div><div class="wt-wave"><canvas role="img" aria-label="OSC ${i+1} wavetable frames"></canvas><output class="wt-frame"></output></div><div class="osc-wt-knobs knob-grid"></div><details class="osc-wt-details"><summary>Unison / Motion / FM</summary><div class="osc-wt-selects osc-selects"></div><div class="osc-wt-extra knob-grid"></div></details><div class="wt-file-actions"><button type="button" class="wt-import">Import</button><button type="button" class="wt-export">Export</button><button type="button" class="wt-edit-table">Edit table</button><button type="button" class="wt-maker-open">Wave Maker</button><input type="file" accept=".wav,.mp3,audio/*" hidden></div></div><p class="wt-status" role="status" hidden></p><p class="wt-error" role="alert" hidden></p>`;
      this.root.querySelector(".osc-slots").append(root);
      const card={root,canvas:root.querySelector("canvas"),picker:root.querySelector(".wt-table"),loading:false,selects:[]};
      const addSelect=(parent,field,label,choices,group="",convert=v=>v)=>{
        const s=select(`OSC ${i+1} ${label}`,choices,v=>this.edit(i,field,convert(v),group));
        // The visible caption is short; the accessible name includes the slot.
        s.wrap.firstChild.textContent=label;root.querySelector(parent).append(s.wrap);
        card.selects.push({control:s.control,field,group});
        const key=`osc${i+1}_${group==="native"?"native_":"wt_"}${field}`;s.control.dataset.controlKey=key;
        window.SadMidiLearn?.attach(s.control,key,`OSC ${i+1} ${label}`,unit=>this.edit(i,field,convert(choices[Math.round(unit*(choices.length-1))][0]),group));
        return s;
      };
      const type=select(`OSC ${i+1} type`,TYPES,v=>{this.slot(i).kind=v;if(v==="wavefolder"&&!["saw","pulse","triangle","sine"].includes(this.slot(i).native.waveform))this.slot(i).native.waveform="sine";this.change();});
      type.wrap.firstChild.hidden=true;root.querySelector(".osc-type").append(type.wrap);card.type=type.control;
      const knob=(field,label,format,min,max,group,bank,reset,step)=>this.hooks.knob(
        [`osc${i+1}_${group==="wavetable"?"wt_":group==="native"?"native_":""}${field}`,label,format,min,max,`OSC ${i+1} ${label.toLowerCase()}`],root.querySelector(bank),{
          get:()=> (group?this.slot(i)?.[group]:this.slot(i))?.[field]??reset,
          set:v=>this.edit(i,field,v,group),reset:()=>reset,ariaScale:1,...(step?{step,quantize:v=>Math.round(v/step)*step}:{})});
      for(const [f,l,fmt,min,max,d,step] of [["transpose","PITCH",v=>signed(v,"st"),-48,48,0,1],["fine","FINE",v=>signed(v,"ct"),-100,100,0],["level","LEVEL",percent,0,1,.65],["pan","PAN",v=>Math.abs(v-.5)<.005?"C":`${Math.round(Math.abs(v-.5)*200)}${v<.5?"L":"R"}`,0,1,.5]])knob(f,l,fmt,min,max,"",".osc-common",d,step);
      const rootNote=select(`OSC ${i+1} sample root`,Array.from({length:128},(_,n)=>[n,`${["C","C♯","D","D♯","E","F","F♯","G","G♯","A","A♯","B"][n%12]}${Math.floor(n/12)-1}`]),v=>{this.hooks.setSourceRoot(+v);this.sync();});
      rootNote.wrap.firstChild.textContent="Root";rootNote.wrap.classList.add("osc-pcm-root");root.querySelector(".osc-native-selects").append(rootNote.wrap);card.rootNote=rootNote.control;
      addSelect(".osc-native-selects","waveform","Wave",WAVES,"native");
      addSelect(".osc-native-selects","mode","Mode",MODES,"native");
      addSelect(".osc-native-selects","fold_shape","Fold",FOLDS,"native");
      addSelect(".osc-native-selects","cross_source","Cross source",[[0,"Off"],...Array.from({length:8},(_,n)=>[n+1,`OSC ${n+1}`])],"native",Number);
      knob("control1","CONTROL 1",percent,0,1,"native",".osc-native-knobs",0);
      knob("control2","CONTROL 2",percent,0,1,"native",".osc-native-knobs",0);
      const transportSelect=(field,label,choices,convert)=>{
        const s=select(`OSC ${i+1} ${label}`,choices,v=>{(this.slot(i).native.transport??=transport())[field]=convert(v);this.change();});
        s.wrap.firstChild.textContent=label;root.querySelector(".osc-transport-selects").append(s.wrap);card.selects.push({control:s.control,field,group:"transport"});
      };
      transportSelect("enabled","Playback",[[0,"Auto on Sync"],[1,"Window"]],v=>v==="1");
      transportSelect("rolling","Window",[[1,"Rolling"],[0,"Freeze"]],v=>v==="1");
      transportSelect("window_seconds","Length",[[.005,"5 ms"],[.01,"10 ms"],[.02,"20 ms"],[.04,"40 ms"],[.08,"80 ms"],[.16,"160 ms"],[.25,"250 ms"]],Number);
      transportSelect("rate","Read rate",[[.125,"⅛×"],[.25,"¼×"],[.5,"½×"],[1,"1×"],[2,"2×"],[4,"4×"],[8,"8×"]],Number);
      for(const [f,l,fmt,min,max,d] of [["position","POSITION",percent,0,1,0],["detune","DETUNE",v=>`${v.toFixed(1)} ct`,0,100,10],["spread","SPREAD",percent,0,1,1],["phase","PHASE",v=>`${Math.round(v*360)}°`,0,1,0]])knob(f,l,fmt,min,max,"wavetable",".osc-wt-knobs",d);
      for(const [f,l,choices,convert] of [
        ["unison","Unison",Array.from({length:8},(_,n)=>[n+1,`${n+1} ${n?"voices":"voice"}`]),Number],
        ["env","Position ENV",[[0,"Off"],...Array.from({length:8},(_,n)=>[n+1,`ENV ${n+1}`])],Number],
        ["lfo","Position LFO",[[0,"Off"],...Array.from({length:8},(_,n)=>[n+1,`LFO ${n+1}`])],Number],
        ["step","Frames",[[0,"Smooth"],[1,"Step"]],v=>v==="1"],
        ["free_phase","Phase",[[0,"Retrigger"],[1,"Free"]],v=>v==="1"],
        ["fm_mode","Modulation",[[0,"Off"],[1,"Linear FM"],[2,"PM"]],Number],
        ["fm_source","FM source",Array.from({length:8},(_,n)=>[n,`OSC ${n+1}`]),Number],
      ]){
        const actual=f==="env"?"position_env":f==="lfo"?"position_lfo":f;
        const s=select(`OSC ${i+1} ${l}`,choices,v=>{
          this.slot(i).wavetable[actual]=convert(v);this.hooks.parameter(`osc${i+1}_wt_${f}`,Number(convert(v)));this.sync();
        });s.wrap.firstChild.textContent=l;root.querySelector(".osc-wt-selects").append(s.wrap);card.selects.push({control:s.control,field:actual,group:"wavetable"});
      }
      for(const [f,key,l,min,max,d] of [["phase_random","phase_random","RANDOM",0,1,0],["voice_shift","voice_shift","VOICE SHIFT",-1,1,0],["surf_depth","surf_depth","SURF",-1,1,1],["position_env_depth","env_depth","ENV DEPTH",-1,1,0],["position_lfo_depth","lfo_depth","LFO DEPTH",-1,1,0],["key_waves","key_waves","KEY → WAVES",-1,1,0],["fm_index","fm_index","FM INDEX",0,4,0]]){
        this.hooks.knob([`osc${i+1}_wt_${key}`,l,percent,min,max,`OSC ${i+1} ${l.toLowerCase()}`],root.querySelector(".osc-wt-extra"),{
          get:()=>this.slot(i)?.wavetable[f]??d,set:v=>{this.slot(i).wavetable[f]=v;this.hooks.parameter(`osc${i+1}_wt_${key}`,v);},reset:()=>d,ariaScale:1});
      }
      const sample=root.querySelector(".osc-pcm-file");sample.onchange=async()=>{
        const id=sample.value;sample.disabled=true;this.error(i,null);
        try{await this.hooks.selectSource(id);}catch(error){this.error(i,error);}finally{sample.disabled=false;this.sync();}
      };
      root.querySelector(".osc-pcm-import").onclick=()=>this.hooks.importSource();
      const power=root.querySelector(".osc-power");power.onclick=()=>this.edit(i,"enabled",!this.slot(i).enabled);power.dataset.controlKey=`osc${i+1}_enabled`;
      window.SadMidiLearn?.attach(power,`osc${i+1}_enabled`,`OSC ${i+1} enabled`,unit=>this.edit(i,"enabled",unit>=.5));
      root.querySelector(".osc-remove").onclick=()=>this.remove(i);
      root.querySelector(".osc-left").onclick=()=>this.move(i,-1);root.querySelector(".osc-right").onclick=()=>this.move(i,1);
      card.picker.onchange=()=>{const id=card.picker.value;window.SadLazySelect.setValue(card.picker,this.slot(i)?.asset_id);this.choose(i,id);};
      const file=root.querySelector('input[type="file"]');root.querySelector(".wt-import").onclick=()=>file.click();
      file.onchange=()=>{if(file.files[0])this.importFile(i,file.files[0]);file.value="";};
      root.querySelector(".wt-export").onclick=()=>this.export(i);root.querySelector(".wt-edit-table").onclick=()=>this.tools.edit(i);root.querySelector(".wt-maker-open").onclick=()=>this.tools.maker(i);
      root.ondragover=e=>{if(this.slot(i)?.kind==="wavetable"&&e.dataTransfer?.types.includes("Files")){e.preventDefault();root.classList.add("is-drop");}};
      root.ondragleave=()=>root.classList.remove("is-drop");root.ondrop=e=>{e.preventDefault();root.classList.remove("is-drop");if(e.dataTransfer.files[0]&&this.slot(i)?.kind==="wavetable")this.importFile(i,e.dataTransfer.files[0]);};
      return card;
    }
    buildMotion() {
      const root=this.root.querySelector(".osc-motion");
      for(const [f,l,min,max,d] of [["surf","SURF",-1,1,0],["x","X",0,1,.5],["y","Y",0,1,.5]])this.hooks.knob([`osc_${f}`,l,percent,min,max,`Oscillator ${l}`],root,{
        get:()=>this.bank()?.motion[f]??d,set:v=>{this.bank().motion[f]=v;this.hooks.parameter(`osc_${f}`,v);},reset:()=>d,ariaScale:1});
      const xy=button("XY mix",()=>{this.bank().motion.xy_on=!this.bank().motion.xy_on;this.hooks.parameter("osc_xy_on",Number(this.bank().motion.xy_on));this.sync();},"osc-xy-on");root.append(xy);
      this.xy=Array.from({length:4},(_,i)=>{
        const s=select(`XY ${"ABCD"[i]}`,[],v=>{this.bank().xy_slots[i]=+v;this.change();});root.append(s.wrap);return s.control;
      });
    }
    sync() {
      const bank=this.bank();this.root.hidden=!bank;
      if(this.dialog&&this.settings(this.dialog.waveIndex)!==this.dialog.waveSettings)this.dialog.close();
      if(!bank){this.stopAudition();return;}
      const order=this.order();if(!order.includes(this.active))this.active=order[0]??0;
      this.root.querySelector(".osc-add").disabled=order.length>=8;
      this.root.querySelector(".osc-xy-on").setAttribute("aria-pressed",String(bank.motion.xy_on));
      const choices=[[0,"Off"],...order.map(i=>[this.slot(i).id,`OSC ${i+1}`])];
      this.xy.forEach((s,i)=>fill(s,choices,bank.xy_slots[i]));
      reorder(this.root.querySelector(".osc-slots"),bank.order.map(i=>this.cards[i].root));
      reorder(this.root.querySelector(".osc-tabs"),bank.order.map(i=>this.tabs[i]));
      this.cards.forEach((card,i)=>{
        const slot=this.slot(i),tab=this.tabs[i];card.root.hidden=!slot.id;tab.hidden=!slot.id;
        tab.setAttribute("aria-selected",String(i===this.active));tab.tabIndex=i===this.active?0:-1;
        if(!slot.id)return;
        card.root.classList.toggle("is-inactive-tab",i!==this.active);card.root.classList.toggle("is-off",!slot.enabled);
        card.root.querySelector(".osc-power").setAttribute("aria-pressed",String(slot.enabled));card.type.value=slot.kind;
        card.root.querySelector(".osc-left").disabled=order.indexOf(i)===0;card.root.querySelector(".osc-right").disabled=order.indexOf(i)===order.length-1;
        card.root.querySelector(".osc-native").hidden=slot.kind==="wavetable";
        card.root.querySelector(".osc-wt").hidden=slot.kind!=="wavetable";
        card.root.querySelector(".osc-transport").hidden=slot.kind!=="radias"||!["noise","audio_in","formant"].includes(slot.native.waveform);
        for(const {control,field,group} of card.selects){
          const target=group==="transport"?(slot.native.transport??transport()):slot[group];let value=target[field];if(typeof value==="boolean")value=Number(value);
          if(document.activeElement!==control)control.value=String(value??0);
          if(field==="waveform"){control.closest("label").hidden=slot.kind==="pcm";for(const option of control.options)option.disabled=slot.kind==="wavefolder"&&!["saw","pulse","triangle","sine"].includes(option.value);}
          if(field==="fold_shape")control.closest("label").hidden=slot.kind!=="wavefolder";
          if(field==="mode")control.closest("label").hidden=slot.kind!=="radias";
          if(field==="cross_source")control.closest("label").hidden=slot.kind!=="radias"||slot.native.mode!=="cross";
        }
        const pcm=card.root.querySelector(".osc-pcm-source");pcm.hidden=slot.kind!=="pcm";
        card.rootNote.closest("label").hidden=slot.kind!=="pcm";if(document.activeElement!==card.rootNote)card.rootNote.value=String(Math.round(this.hooks.sourceRoot()));
        if(slot.kind==="pcm"){
          const source=this.hooks.source(),picker=card.root.querySelector(".osc-pcm-file");
          window.SadLazySelect.configure(picker,source?.sample_id??"",this.hooks.sourceGroups(),{play:(id,notify)=>this.hooks.previewSource(id,notify),stop:()=>this.hooks.stopSourcePreview()});
          if(!source?.sample_id)picker.textContent=source?.sample_name??"Choose sample";
        }
        if(slot.kind!=="wavetable")return;
        this.settings(i);window.SadLazySelect.configure(card.picker,slot.asset_id,this.groups(),{play:(id,notify)=>this.audition(id,notify),stop:()=>this.stopAudition()});
        card.picker.disabled=card.loading;card.picker.setAttribute("aria-busy",String(card.loading));
        if(card.asset!==slot.asset_id){card.asset=slot.asset_id;card.preview=null;const id=slot.asset_id;
          this.preview(id).then(p=>{if(this.slot(i)?.asset_id===id){card.preview=p;this.draw(i);}}).catch(e=>{if(card.asset===id)this.error(i,e);});}
        this.draw(i);
      });
    }
    update(positions) {
      this.positions=positions;this.drawAll();
      const source=this.hooks.source(),signature=JSON.stringify([source?.sample_id,source?.sample_name,this.hooks.sourceGroups().length]);
      if(signature!==this.sourceSignature){this.sourceSignature=signature;this.sync();}
    }
  }
  // Share the actual library, import, Wave Maker and preview implementation.
  for(const method of ["catalog","groups","choose","preview","audition","stopAudition","status","error","draw","drawAll","importFile","applyImport","importDialog","export"])
    Editor.prototype[method]=window.SadWavetable.Editor.prototype[method];

  const key = n => n.kind==="osc"?`osc:${n.id}`:n.index===undefined?n.kind:`${n.kind}:${n.index}`;
  const parse = value => {const [kind,id]=value.split(":");return kind==="osc"?{kind,id:+id}:id===undefined?{kind}:{kind,index:+id};};
  const routeDefault = () => ({source:{kind:"none"},target:{kind:"none"},enabled:true,tap:"generation",event:"auto",bridge:{kind:"auto"},ring:0,sync:"off",scope:"playback",strength:1,response:5,reset_phase:0,divide:1,include_forced:false});
  const fill = (control, choices, value) => {
    const signature=JSON.stringify(choices);
    if(control.dataset.choices!==signature){control.replaceChildren(...choices.map(([v,text])=>{const o=el("option","",text);o.value=String(v);return o;}));control.dataset.choices=signature;}
    if(document.activeElement!==control)control.value=String(value);
  };
  const mode = r => r.sync==="hard"?(r.ring>0?"ring_hard":"hard"):r.sync==="soft"?(r.ring>0?"ring_sync":"sync"):r.ring>0?"ring":"off";
  const modeValues = [["off","Off"],["ring","Ring"],["sync","Sync"],["hard","Hard Sync"],["ring_sync","Ring + Sync"],["ring_hard","Ring + Hard Sync"]];

  class Connections {
    constructor(root,hooks) {
      this.root=root;this.hooks=hooks;this.rows=new Map();this.nodeCards=new Map();
      root.innerHTML='<div class="section-heading"><h2>Connections <span>Ring / Sync</span></h2><button type="button" class="connection-add">+ Connection</button></div><div class="connection-list"></div><p class="connection-empty muted">Choose a source and target to connect their sound or phase.</p>';
      root.querySelector(".connection-add").onclick=()=>this.add();
      const capture=el("div","grain-capture");capture.innerHTML='<span>Capture</span><div class="capture-mode"></div><div class="capture-slots" role="group" aria-label="Oscillators captured by Granular"></div>';
      const choose=select("Granular capture",[["chain","Chain"],["osc","Selected OSC"]],v=>{const p=hooks.read();(p.grain_capture??={mode:"chain",slots:255}).mode=v;hooks.structure();this.sync();});choose.wrap.firstChild.hidden=true;
      capture.querySelector(".capture-mode").append(choose.wrap);this.captureMode=choose.control;this.capture=capture;
      document.querySelector("#grain-engine > .section-heading").after(capture);
    }
    nodes() {
      const p=this.hooks.read(),nodes=[];
      for(const i of p?.oscillators?.order??[]){const s=p.oscillators.slots[i];if(s.id)nodes.push([`osc:${s.id}`,`OSC ${i+1} · ${TYPES.find(t=>t[0]===s.kind)?.[1]??s.kind}`]);}
      for(const [kind,name] of [["synth","Synth"],["wavetable","WT"],["resynth","Resynth"],["tb303","303 OSC"]]){
        const s=p?.[kind];if(!s||(kind==="resynth"&&!s.enabled&&!p.rack?.modules.some(m=>m.module==="resynth")))continue;
        if(kind==="tb303")nodes.push(["tb303_vco","303 VCO"]);
        for(let i=0;i<(s.count??1);i++)nodes.push([`${kind}:${i}`,`${name} ${i+1}`]);
      }
      const granular=p.rack?p.rack.modules.some(m=>m.module==="grains"):p.bus?!!p.bus.granular:p.synth?!!p.synth.granular:p.drums?!!p.drums.granular:true;
      if(granular){for(let i=0;i<4;i++)nodes.push([`group:${i}`,`Grain ${"ABCD"[i]}`]);nodes.push(["grain_wet","Grain Wet"]);}return nodes;
    }
    routes() {return (this.hooks.read().interactions??={version:1,routes:[]}).routes;}
    changed(index, structural=true, field=null) {
      const routes=this.routes(),r=routes[index];
      if(r.enabled){
        for(let i=0;i<routes.length;i++){
          const other=routes[i];if(i===index||!other.enabled||key(other.target)!==key(r.target))continue;
          if(r.ring>0)other.ring=0;
          if(r.sync!=="off"&&other.sync!=="off"){
            if(other.scope==="both"&&r.scope!=="both")other.scope=r.scope==="clock"?"playback":"clock";
            else if(r.scope==="both"||r.scope===other.scope)other.sync="off";
          }
        }
      }
      if(r.enabled&&(r.ring>0||r.sync!=="off")&&(r.source.kind==="group"||r.target.kind==="group"))this.hooks.groups().advanced=true;
      if(r.sync!=="off"&&((r.target.kind==="group"&&r.scope!=="playback")||(r.source.kind==="group"&&(r.event==="group_clock_wrap"||(r.event==="auto"&&r.tap==="generation")))))this.hooks.read().grain_scheduler="independent";
      // Topology and replacement masters are committed together.
      if(structural)this.hooks.structure();else this.hooks.parameter(`interaction${index+1}_${field}`,Number(field==="phase"?r.reset_phase:r[field]));
      this.sync();
    }
    add() {
      const routes=this.routes(),nodes=this.nodes(),sources=nodes.filter(n=>n[0]!=="grain_wet");
      if(!sources.length)return;
      let index=routes.findIndex(r=>r.source.kind==="none"&&r.target.kind==="none");if(index<0)index=routes.length;if(index>=96)return;
      const r=routeDefault();r.source=parse(sources[0][0]);r.target=parse(sources.find(n=>n[0].startsWith("group:"))?.[0]??sources[1]?.[0]??sources[0][0]);
      routes[index]=r;this.changed(index);this.rows.get(index)?.root.querySelector("select").focus();
    }
    row(index, inline=false, caption=`Connection ${index+1}`) {
      const root=el("div",inline?"connection osc-connection":"connection");root.dataset.route=String(index);root.innerHTML='<div class="connection-main"></div><details class="connection-details"><summary>Phase / Routing</summary><div class="connection-options osc-selects"></div><div class="connection-knobs knob-grid"></div></details>';
      const result={root,controls:{},updates:[],inline,caption};const read=()=>this.routes()[index]??routeDefault();
      const add=(field,label,choices,parent,change)=>{
        const s=select(label,choices,v=>{change(v);this.changed(index);});
        if(inline)s.control.setAttribute("aria-label",`${caption} ${label.toLowerCase()}`);
        root.querySelector(parent).append(s.wrap);result.controls[field]=s.control;return s.control;
      };
      add("source","Source",[],".connection-main",v=>{const r=read();r.source=parse(v);r.event="auto";});
      add("mode","Mode",modeValues,".connection-main",v=>{const r=read();r.ring=v.includes("ring")?(r.ring||1):0;r.sync=v.includes("hard")?"hard":v.includes("sync")?"soft":"off";});
      add("target","Target",[],".connection-main",v=>{const r=read();r.target=parse(v);if(r.target.kind!=="group")r.scope="playback";});
      if(inline)result.controls.target.closest("label").hidden=true;
      const enabled=button("",()=>{read().enabled=!read().enabled;this.changed(index);},"power connection-power");enabled.innerHTML='<svg aria-hidden="true"><use href="#power-glyph" /></svg>';enabled.setAttribute("aria-label",`${caption} enabled`);root.querySelector(".connection-main").append(enabled);
      const remove=button("×",()=>{this.routes()[index]={...routeDefault(),enabled:false};this.hooks.structure();this.sync();},"connection-remove");remove.setAttribute("aria-label",`Remove ${caption.toLowerCase()}`);root.querySelector(".connection-main").append(remove);
      add("scope","Group phase",[["playback","Playback"],["clock","Clock"],["both","Playback + Clock"]],".connection-options",v=>read().scope=v);
      add("event","Sync event",[["natural_wrap","Natural cycle"],["audio_zero_cross","Audio zero crossing"],["group_clock_wrap","Group clock"],["grain_launch","Grain launch"]],".connection-options",v=>read().event=v);
      add("tap","Source tap",[["generation","Generation"],["output","Output · feedback"]],".connection-options",v=>read().tap=v);
      add("bridge","Voices",[["auto","Auto"],["same_voice","Same note"],["mix","Mix"],...Array.from({length:33},(_,i)=>[`voice:${i}`,`Voice ${i+1}`]),...Array.from({length:16},(_,i)=>[`band:${i}`,`Band ${i+1}`])],".connection-options",v=>read().bridge=parse(v));
      add("include_forced","Forced events",[[0,"Ignore"],[1,"Include"]],".connection-options",v=>read().include_forced=v==="1");
      for(const [field,label,min,max,d,fmt,step] of [["ring","RING",0,1,1,percent],["strength","SYNC",0,1,1,percent],["response","RESPONSE",.1,1000,5,v=>`${v.toFixed(1)} ms`],["phase","PHASE",0,1,0,v=>`${Math.round(v*360)}°`],["divide","DIVIDE",1,32,1,v=>`÷${Math.round(v)}`,1]]){
        result.updates.push(this.hooks.knob([`interaction${index+1}_${field}`,label,fmt,min,max,`${caption} ${label.toLowerCase()}`],root.querySelector(".connection-knobs"),{
          register:!inline,get:()=>{const r=read();return field==="phase"?r.reset_phase:r[field];},set:v=>{const r=read();if(field==="phase")r.reset_phase=v;else r[field]=v;this.changed(index,false,field);},reset:()=>d,ariaScale:1,...(step?{step,quantize:Math.round}:{})}));
      }
      return result;
    }
    syncRow(row,r,nodes,targets) {
      row.root.classList.toggle("is-off",!r.enabled);
      fill(row.controls.source,nodes,key(r.source));fill(row.controls.target,targets,key(r.target));
      fill(row.controls.mode,modeValues,mode(r));row.controls.scope.value=r.scope;row.controls.scope.closest("label").hidden=r.target.kind!=="group";
      fill(row.controls.event,[["auto","Auto"],...(r.source.kind==="group"?[["group_clock_wrap","Group clock"],["grain_launch","Grain launch"],["audio_zero_cross","Audio zero crossing"]]:r.source.kind==="grain_wet"?[["audio_zero_cross","Audio zero crossing"]]:[["natural_wrap","Natural cycle"],["audio_zero_cross","Audio zero crossing"]])],r.event);
      row.controls.tap.value=r.tap;
      fill(row.controls.bridge,[["auto","Auto"],["mix","Mix"],...(["osc","synth","wavetable"].includes(r.source.kind)?[["same_voice","Same note"],...Array.from({length:33},(_,i)=>[`voice:${i}`,`Voice ${i+1}`])]:r.source.kind==="resynth"?Array.from({length:16},(_,i)=>[`band:${i}`,`Band ${i+1}`]):[])],key(r.bridge));
      row.controls.include_forced.value=String(Number(r.include_forced));
      row.root.querySelector(".connection-power").setAttribute("aria-pressed",String(r.enabled));
      for(const update of row.updates)update.paint();
    }
    syncNodeCards(nodes,targets,routes) {
      const p=this.hooks.read(),bank=p.oscillators;
      const cards=[...Array.from({length:8},(_,i)=>({
        key:`osc-slot:${i}`,card:document.getElementById(`osc-slot-${i}`),
        target:bank?.slots[i]?.id?{kind:"osc",id:bank.slots[i].id}:null,caption:`OSC ${i+1}`,masters:2,
      })),...Array.from({length:4},(_,i)=>({
        key:`group:${i}`,card:document.getElementById(`grain-group-page-${i+1}`),
        target:targets.some(([id])=>id===`group:${i}`)?{kind:"group",index:i}:null,caption:`Grain ${"ABCD"[i]}`,masters:3,
      }))];
      for(const {key:cardKey,card,target,caption,masters} of cards){
        let view=this.nodeCards.get(cardKey);
        if(!card||!target){if(view)view.root.hidden=true;continue;}
        if(!view){
          const root=el("section",`osc-interactions${target.kind==="group"?" grain-interactions":""}`);root.setAttribute("aria-label",`${caption} Ring / Sync`);
          root.innerHTML='<div class="osc-interaction-heading"><strong>Ring / Sync</strong><button type="button">+ Source</button></div><div class="osc-interaction-list"></div>';
          if(target.kind==="group")card.prepend(root);else card.querySelector(".osc-common").after(root);
          view={root,rows:new Map(),target,wantDraft:false,draft:this.freshDraft(target)};this.nodeCards.set(cardKey,view);
          root.querySelector("button").setAttribute("aria-label",`Add source to ${caption}`);
          root.querySelector("button").onclick=()=>{view.wantDraft=true;view.draft=this.freshDraft(view.target);this.sync();view.draftControls?.source.focus();};
          const draftRoot=el("div","osc-connection-draft osc-selects");
          const commit=()=>{
            const d=view.draft;if(d.source==="none"||d.mode==="off")return;
            if(!this.nodes().some(([id])=>id===key(view.target)))return;
            const all=this.routes();let index=all.findIndex(r=>r.source.kind==="none"&&r.target.kind==="none");if(index<0)index=all.length;if(index>=96)return;
            const r=routeDefault();r.source=parse(d.source);r.target={...view.target};r.scope=d.scope;r.ring=d.mode.includes("ring")?1:0;r.sync=d.mode.includes("hard")?"hard":d.mode.includes("sync")?"soft":"off";
            all[index]=r;view.wantDraft=false;view.draft=this.freshDraft(view.target);this.changed(index);
          };
          const source=select(`${caption} new source`,[],v=>{view.draft.source=v;commit();});source.wrap.firstChild.textContent="Source";
          const interaction=select(`${caption} new mode`,modeValues,v=>{view.draft.mode=v;commit();});interaction.wrap.firstChild.textContent="Mode";
          draftRoot.append(source.wrap,interaction.wrap);view.draftRoot=draftRoot;view.draftControls={source:source.control,mode:interaction.control};
          if(target.kind==="group"){
            const scope=select(`${caption} new group phase`,[["playback","Playback"],["clock","Clock"],["both","Playback + Clock"]],v=>{view.draft.scope=v;commit();});scope.wrap.firstChild.textContent="Group phase";
            draftRoot.append(scope.wrap);view.draftControls.scope=scope.control;
          }
        }
        if(key(view.target)!==key(target)){view.target=target;view.wantDraft=false;view.draft=this.freshDraft(target);}
        view.root.hidden=false;
        const incoming=routes.flatMap((r,index)=>key(r.target)===key(target)?[index]:[]),live=new Set(incoming),list=view.root.querySelector(".osc-interaction-list");
        for(const [index,row] of view.rows)if(!live.has(index))row.root.remove();
        for(const index of incoming){
          let row=view.rows.get(index);if(!row){row=this.row(index,true,`${caption} connection ${index+1}`);view.rows.set(index,row);}
          if(row.root.parentElement!==list)list.append(row.root);this.syncRow(row,routes[index],nodes,targets);
          if(target.kind==="group"){
            const scope=row.controls.scope.closest("label"),main=row.root.querySelector(".connection-main");
            if(scope.parentElement!==main)main.insertBefore(scope,row.root.querySelector(".connection-power"));
          }
        }
        const draft=incoming.length===0||view.wantDraft,free=routes.length<96||routes.some(r=>r.source.kind==="none"&&r.target.kind==="none");
        if(draft){
          if(view.draftRoot.parentElement!==list)list.append(view.draftRoot);
          fill(view.draftControls.source,[["none","Off"],...nodes],view.draft.source);fill(view.draftControls.mode,modeValues,view.draft.mode);
          if(view.draftControls.scope)view.draftControls.scope.value=view.draft.scope;
          view.draftControls.source.disabled=view.draftControls.mode.disabled=!free;
        }else view.draftRoot.remove();
        const add=view.root.querySelector(".osc-interaction-heading button");add.hidden=incoming.length===0;add.disabled=draft||incoming.length>=masters||!free;
      }
    }
    freshDraft(target) {
      const playback=this.routes().some(r=>r.enabled&&key(r.target)===key(target)&&r.sync!=="off"&&r.scope!=="clock");
      return {source:"none",mode:"off",scope:target.kind==="group"&&playback?"clock":"playback"};
    }
    update() {
      for(const view of this.nodeCards.values())for(const row of view.rows.values())if(row.root.isConnected)for(const update of row.updates)update.paint();
    }
    sync() {
      const p=this.hooks.read();if(!p)return;
      this.root.hidden=!!p.control||!p.rack?.modules.some(m=>m.module==="connections");
      const nodes=this.nodes(),targets=nodes.filter(n=>n[0]!=="grain_wet"),routes=this.routes();let count=0;
      routes.forEach((r,i)=>{
        if(r.source.kind==="none"&&r.target.kind==="none"){this.rows.get(i)?.root.remove();return;}
        count++;let row=this.rows.get(i);if(!row){row=this.row(i);this.rows.set(i,row);}
        if(!row.root.isConnected)this.root.querySelector(".connection-list").append(row.root);
        this.syncRow(row,r,nodes,targets);
      });
      for(const [i,row] of this.rows)if(i>=routes.length)row.root.remove();
      this.root.querySelector(".connection-empty").hidden=count>0;this.root.querySelector(".connection-add").disabled=count>=96||!targets.length;
      this.syncNodeCards(nodes,targets,routes);
      const modules=p.rack?.modules??[],gi=modules.findIndex(m=>m.module==="grains"),oi=modules.findIndex(m=>m.module==="oscillators");
      const eligible=oi>=0&&gi>oi&&!modules.slice(oi+1,gi).some(m=>["sample","synth","wavetable","drums","sampler","vocal","tb303","mixer"].includes(m.module));
      this.capture.hidden=!eligible;if(!eligible)return;
      const capture=p.grain_capture??{mode:"chain",slots:255};this.captureMode.value=capture.mode;
      const area=this.capture.querySelector(".capture-slots");area.hidden=capture.mode!=="osc";
      const signature=JSON.stringify([p.oscillators.order,p.oscillators.slots.map(s=>s.id)]);
      if(area.dataset.slots!==signature){area.replaceChildren();for(const i of p.oscillators.order){if(!p.oscillators.slots[i].id)continue;
        const b=button(`OSC ${i+1}`,()=>{const c=(this.hooks.read().grain_capture??={mode:"osc",slots:255});c.slots^=1<<i;this.hooks.structure();this.sync();},"capture-slot");b.dataset.slot=i;area.append(b);}
        area.dataset.slots=signature;}
      for(const b of area.children)b.setAttribute("aria-pressed",String(!!(capture.slots&(1<<+b.dataset.slot))));
    }
  }
  window.SadOscillators={Editor,Connections,defaults};
})();
