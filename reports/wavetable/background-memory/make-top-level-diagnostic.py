from pathlib import Path
import shutil,re
src=Path('/tmp/sadpet-wavetable-release/dist/web');pub=Path('/tmp/sadpet-wt-bg-deployed');out=Path('/tmp/sadpet-wt-public-top')
shutil.copytree(src,out,dirs_exist_ok=True,ignore=shutil.ignore_patterns('baseline','check-mobile-wt.html'))
for p in pub.iterdir():
 if p.is_file():shutil.copy2(p,out/p.name)
qa=(pub/'verify-wavetable-app.html').read_text()
markup=qa.split('<h1>',1)[1].split('<script type="module">',1)[0]
markup='<h2>'+markup.replace('</h1>','</h2>',1)
markup=re.sub(r'<details id="interface">.*?</details>','',markup,flags=re.S)
markup=markup.replace('id="','id="wtv-')
script=qa.split('<script type="module">',1)[1].split('</script>',1)[0]
script=script.replace("const $=id=>document.getElementById(id),sleep=ms=>new Promise(r=>setTimeout(r,ms)),frame=$('app');", "const $=id=>document.getElementById('wtv-'+id),sleep=ms=>new Promise(r=>setTimeout(r,ms));")
script=script.replace("frame.onload=()=>{ready=(async()=>{", "ready=(async()=>{")
script=script.replace(" win=frame.contentWindow;await win.sadPetState();", " win=window;while(!win.sadPetState)await sleep(10);await win.sadPetState();")
script=script.replace(" })().catch(error=>{$('status').textContent='Ошибка';$('result').textContent=error.stack;});};", " })().catch(error=>{$('status').textContent='Ошибка';$('result').textContent=error.stack;});")
script=script.replace(" $('run').disabled=true;", " document.querySelector('main').hidden=true;$('run').disabled=true;",1)
script=script.replace("scope:'Actual SAD PET index.html?engine=wavetable, production BrowserBackend, Rack, AudioWorklet and all real workers.", "scope:'Top-level SAD PET document with exact published runtime/WASM, production BrowserBackend, Rack, AudioWorklet and all real workers.")
assert 'frame.' not in script and "getElementById('wtv-'+id)" in script
(out/'verify-main.mjs').write_text(script)
index=(pub/'index.html').read_text()
head='<style>#wtv-toolbar{font:15px system-ui;background:#17191c;color:#eee;padding:18px;max-width:1240px;margin:auto}#wtv-toolbar h2{font-size:20px}#wtv-toolbar .controls{display:flex;gap:12px;align-items:center;flex-wrap:wrap}#wtv-toolbar button,#wtv-toolbar select{font:inherit;color:#eee;background:#303740;border:1px solid #647080;border-radius:6px;padding:8px;min-height:40px}#wtv-toolbar progress{width:100%}#wtv-toolbar pre{white-space:pre-wrap;overflow-wrap:anywhere;font-size:12px}#wtv-metrics{font-size:20px}</style><script>for(const t of ["pointerdown","keydown"])document.addEventListener(t,e=>{if(e.target.closest("#wtv-toolbar"))e.stopImmediatePropagation()},true)</script>'
index=index.replace('<head>','<head>'+head,1)
index=index.replace('<body>','<body><aside id="wtv-toolbar">'+markup+'</aside>',1)
index=index.replace('</body>','<script type="module" src="./verify-main.mjs"></script></body>')
assert 'id="wtv-run"' in index and 'id="wtv-toolbar"' in index
(out/'verify-main.html').write_text(index)
print('Top-level published-bundle QA ready:',out)
