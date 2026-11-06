import concurrent.futures
import datetime
import hashlib
import json
import re
import subprocess
import urllib.request
from pathlib import Path

base = "https://fushugaku.github.io/sad-pet/"
commit = "8d85a82858ef079ffd316be288ff7b13aefa6bb4"
assets = {name: "rust/web/" + name for name in ["app.js", "control.js", "midi-targets.js", "modulation.js", "panels.js", "rack.js", "sequence.js", "midi.js", "controller.js"]}
assets["xl3-feedback.mjs"] = "wasm/web/xl3-feedback.mjs"

def fetch(url):
    request = urllib.request.Request(url, headers={"Cache-Control": "no-cache"})
    with urllib.request.urlopen(request, timeout=45) as response:
        return response.read().decode()

def normalise(text):
    return re.sub(r"\?v=[0-9a-f]{12}", "", text)

html = fetch(base)
versions = sorted(set(re.findall(r"v=([0-9a-f]{12})", html)))
assert len(versions) == 1, versions
version = versions[0]

def check(item):
    asset, source = item
    published = normalise(fetch(base + asset + "?v=" + version))
    expected = normalise(subprocess.check_output(["git", "show", commit + ":" + source], text=True))
    return {"asset": asset, "source": source, "matches_commit": published == expected, "sha256": hashlib.sha256(published.encode()).hexdigest()}

with concurrent.futures.ThreadPoolExecutor(max_workers=5) as pool:
    results = list(pool.map(check, assets.items()))
record = {"checked_at": datetime.datetime.now(datetime.timezone.utc).isoformat(), "url": base, "commit": commit, "version": version, "assets": results}
Path(__file__).with_name("public-verification.json").write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps(record, indent=2))
assert all(result["matches_commit"] for result in results), "Published assets do not match the MIDI fix"
