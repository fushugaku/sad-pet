import hashlib
import re
import shutil
import subprocess
from pathlib import Path

root=Path.cwd()
out=Path('/tmp/sadpet-midi-navigation')
prefix=Path('/tmp/sadpet-visual-midi-script.html').read_text()+Path(__file__).with_name('qa-metrics.html').read_text()
for case in ['before','after']:
    target=out/case
    shutil.copytree(root/'dist/web',target,dirs_exist_ok=True)
    shutil.copytree(root/'rust/web',target,dirs_exist_ok=True)
    index=(target/'index.html').read_text()
    index=index.replace('<script src="./library.js"','<script src="./browser.js" defer></script>\n    <script src="./library.js"')
    index=index.replace('<script src="./clock.js"','<script src="./controller.js" defer></script>\n    <script src="./clock.js"')
    (target/'index.html').write_text(index.replace('RUST EDITION','WEBASSEMBLY'))
    shutil.copytree(root/'wasm/web',target,dirs_exist_ok=True)
    if case=='before':
        for name,source in [('midi.js','rust/web/midi.js'),('worklet.mjs','wasm/web/worklet.mjs')]:
            (target/name).write_bytes(subprocess.check_output(['git','show','4bfe108:'+source]))
    backend=target/'browser-backend.mjs'
    source=backend.read_text()
    for name in ['sad_pet.tc.wasm','sad_pet.simd.wasm']:
        if not (target/name).exists():source=source.replace(f'new URL("./{name}", import.meta.url)','undefined')
    backend.write_text(source)
    runtime=sorted(p for p in target.rglob('*') if p.is_file() and p.suffix in ['.js','.mjs','.css','.wasm'] and 'baseline' not in p.relative_to(target).parts)
    version=hashlib.sha256(b''.join(p.read_bytes() for p in runtime)).hexdigest()[:12]
    for path in [target/'index.html',*[p for p in runtime if p.suffix in ['.js','.mjs'] and 'vendor' not in p.relative_to(target).parts]]:
        text=re.sub(r'\?v=[a-f0-9]{12}','',path.read_text()).replace('__BUILD_VERSION__',version)
        text=re.sub(r'([\"\'])(\./[^\"\'\s?]+\.(?:mjs|js|css|wasm))\1',lambda m:f'{m[1]}{m[2]}?v={version}{m[1]}',text)
        path.write_text(text)
    index=(target/'index.html').read_text()
    (target/'verify-midi-navigation.html').write_text(index.replace('<head>','<head>'+prefix,1))
    print(case,version,target,flush=True)
