import concurrent.futures
import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess
import urllib.request

base='https://fushugaku.github.io/sad-pet/'
commit='4631fad14e6a579ef41a9c7ebd8d388375ec6381'
assets={'midi.js':'rust/web/midi.js','worklet.mjs':'wasm/web/worklet.mjs',
        'app.js':'rust/web/app.js','midi-targets.js':'rust/web/midi-targets.js',
        'xl3-feedback.mjs':'wasm/web/xl3-feedback.mjs'}

def fetch(url):
    request=urllib.request.Request(url,headers={'Cache-Control':'no-cache'})
    with urllib.request.urlopen(request,timeout=45) as response:
        return response.read().decode()

def normalise(text):
    return re.sub(r'\?v=[0-9a-f]{12}','',text)

versions=sorted(set(re.findall(r'v=([0-9a-f]{12})',fetch(base))))
assert len(versions)==1,versions
version=versions[0]

def check(item):
    asset,source=item
    actual=normalise(fetch(base+asset+'?v='+version))
    expected=normalise(subprocess.check_output(['git','show',commit+':'+source],text=True))
    return {'asset':asset,'source':source,'matches_commit':actual==expected,
            'sha256':hashlib.sha256(actual.encode()).hexdigest()}

with concurrent.futures.ThreadPoolExecutor(max_workers=5) as pool:
    results=list(pool.map(check,assets.items()))
record={'checked_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'commit':commit,'url':base,'version':version,'assets':results}
Path(__file__).with_name('public-verification.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record,indent=2))
assert all(item['matches_commit'] for item in results),'The public app still serves earlier MIDI code'
