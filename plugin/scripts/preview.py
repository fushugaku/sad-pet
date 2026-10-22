#!/usr/bin/env python3
"""Browser QA of the plugin and app editors against the real native engine (no sound output).

/?app stands in for the SAD PET app: native file dialogs open --open files or save
into <library>/Exports, and Record writes a short tone as the take."""
import argparse
import ctypes as c
import json
import math
import os
import struct
import tempfile
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent


class Bytes(c.Structure):
    _fields_ = [("data", c.c_void_p), ("length", c.c_size_t)]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine", type=Path, default=PROJECT / "target/release/libsad_pet_plugin.dylib")
    parser.add_argument("--port", type=int, default=8766)
    parser.add_argument("--library", type=Path, help="App library and presets folder (default: a new temporary folder)")
    parser.add_argument("--open", type=Path, nargs="*", default=[], help="Files the app's open dialogs return")
    args = parser.parse_args()
    library = (args.library or Path(tempfile.mkdtemp(prefix="sad-pet-app-preview-"))).resolve()
    os.environ["SAD_PET_LIBRARY_DIR"] = str(library / "Library")
    os.environ["SAD_PET_PRESET_DIR"] = str(library / "Presets")
    exports = library / "Exports"
    exports.mkdir(parents=True, exist_ok=True)
    lib = c.CDLL(str(args.engine))
    lib.sad_control_new_standalone.restype = c.c_void_p
    lib.sad_control_new.argtypes = [c.c_bool]
    lib.sad_control_new.restype = c.c_void_p
    lib.sad_control_prepare.argtypes = [c.c_void_p, c.c_uint32]
    lib.sad_control_prepare.restype = c.c_void_p
    lib.sad_control_request.argtypes = [c.c_void_p, c.c_char_p, c.c_size_t]
    lib.sad_control_request.restype = Bytes
    lib.sad_parameters.restype = Bytes
    lib.sad_bytes_free.argtypes = [c.c_void_p, c.c_size_t]
    lib.sad_audio_free.argtypes = [c.c_void_p]
    lib.sad_control_free.argtypes = [c.c_void_p]
    lib.sad_control_get_parameters.argtypes = [c.c_void_p, c.POINTER(c.c_float), c.c_size_t]
    lib.sad_audio_begin.argtypes = [c.c_void_p, c.POINTER(c.c_float), c.c_size_t, c.c_float, c.c_double, c.c_bool, c.c_bool, c.c_size_t]
    lib.sad_audio_process.argtypes = [c.c_void_p] + [c.POINTER(c.c_float)] * 4 + [c.c_size_t]
    lib.sad_audio_end.argtypes = [c.c_void_p, c.c_size_t]

    def take(result):
        data = c.string_at(result.data, result.length)
        lib.sad_bytes_free(result.data, result.length)
        return json.loads(data)

    size = len(take(lib.sad_parameters()))
    instances = []
    for controller in [lib.sad_control_new(False), lib.sad_control_new(True), lib.sad_control_new_standalone()]:
        audio = lib.sad_control_prepare(controller, 48000)
        instances.append((controller, audio, (c.c_float * size)()))
    recorder = {"next": 0, "active": None, "take": None}
    outputs = {"names": ["Preview speakers", "Preview headphones"], "current": "Preview speakers"}
    left, right = (c.c_float * 2048)(), (c.c_float * 2048)()
    incoming = (c.c_float * 2048)(*(0.15 * math.sin(i * 0.04) for i in range(2048)))

    def tone(path, seconds):
        frames = int(48000 * seconds)
        samples = b"".join(struct.pack("<hh", v, v) for v in (int(8000 * math.sin(i * 0.05)) for i in range(frames)))
        header = struct.pack("<4sI4s4sIHHIIHH4sI", b"RIFF", 36 + len(samples), b"WAVE", b"fmt ", 16, 1, 2, 48000, 192000, 4, 16, b"data", len(samples))
        path.write_bytes(header + samples)

    def recording_state():
        active, finished = recorder["active"], recorder["take"]
        current = active or finished or {"id": 0, "source": "output"}
        return {"active": bool(active), "available": bool(finished) and not active, "id": current["id"],
                "seconds": 1.0 if finished and not active else 0.0, "source": current["source"], "error": None}

    def app_request(request):
        """What the app's native layer answers itself (see plugin/native/Standalone.cpp)."""
        data = request.get("data") if isinstance(request.get("data"), dict) else {}
        data.pop("path", None)
        data.pop("paths", None)
        request["data"] = data
        path = request.get("path", "").split("?")[0]
        dialog = request.pop("dialog", None)
        if dialog:
            if dialog.get("mode") == "save":
                data["path"] = str(exports / Path(dialog.get("filename") or "export").name)
            else:
                patterns = [p.strip().lstrip("*").lower() for p in dialog.get("patterns", "").split(";") if p.strip()]
                chosen = [str(f.resolve()) for f in args.open if not patterns or f.suffix.lower() in patterns]
                if not chosen:
                    return {"status": 200, "data": {"cancelled": True}}
                if dialog.get("multiple"):
                    data["paths"] = chosen
                else:
                    data["path"] = chosen[0]
            return None
        if path == "/api/app/audio-outputs":
            return {"status": 200, "data": {"outputs": outputs["names"], "output": outputs["current"]}}
        if path == "/api/app/audio-output":
            if data.get("output") not in outputs["names"]:
                return {"status": 400, "data": {"error": "This audio output is no longer connected."}}
            outputs["current"] = data["output"]
            return {"status": 200, "data": {"ok": True}}
        if path in ("/api/app/audio-settings", "/api/app/log"):
            if path == "/api/app/log":
                print("editor:", data.get("message"), flush=True)
            return {"status": 200, "data": {"ok": True}}
        if path == "/api/recording":
            if data.get("active") and not recorder["active"]:
                recorder["next"] += 1
                recorder["active"] = {"id": recorder["next"], "source": "input" if data.get("source") == "input" else "output"}
            elif not data.get("active") and recorder["active"]:
                file = library / f"take-{recorder['active']['id']}.wav"
                tone(file, 1.0)
                recorder["take"] = {**recorder["active"], "file": file}
                recorder["active"] = None
            return {"status": 200, "data": recording_state()}
        if path == "/api/library/take":
            finished = recorder["take"]
            if not finished or finished["id"] != data.get("id"):
                return {"status": 400, "data": {"error": "The recording is no longer available."}}
            data.update(path=str(finished["file"]), source=finished["source"])
            encoded = json.dumps(request).encode()
            response = take(lib.sad_control_request(instances[2][0], encoded, len(encoded)))
            if response["status"] == 200:
                finished["file"].unlink(missing_ok=True)
                recorder["take"] = None
            return response
        return None

    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_POST(self):
            url = urlparse(self.path)
            if url.path != "/native":
                self.send_error(404)
                return
            data = self.rfile.read(int(self.headers.get("Content-Length", "0")))
            request = json.loads(data)
            app = "app" in url.query
            controller, audio, values = instances[2 if app else 1 if "fx" in url.query else 0]
            polled = str(request.get("path", "")).split("?")[0] == "/api/state"
            if polled:
                lib.sad_control_get_parameters(controller, values, size)
                lib.sad_audio_begin(audio, values, size, 120.0, 0.0, False, False, 2048)
                live = incoming if "fx" in url.query or app else None
                lib.sad_audio_process(audio, live, live, left, right, 2048)
                lib.sad_audio_end(audio, 2048)
            response = app_request(request) if app else None
            if response is None:
                encoded = json.dumps(request).encode()
                response = take(lib.sad_control_request(controller, encoded, len(encoded)))
                if app and polled and response["status"] == 200:
                    response["data"]["recording"] = recording_state()
                    response["data"].update(audio_device=outputs["current"], audio_input=response["data"]["input"]["active"])
            body = json.dumps(response, separators=(",", ":")).encode()
            if app and any(key in text for text in (data, body) for key in (b'"binary"', b'"base64"', b'"sample_wav"')):
                print("AUDIO CROSSED THE EDITOR BOUNDARY:", request.get("path"), flush=True)
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_GET(self):
            url = urlparse(self.path)
            name = url.path.rsplit("/", 1)[-1] or "index.html"
            paths = {
                "index.html": PROJECT / "rust/web/index.html",
                **{name: PROJECT / "rust/web" / name for name in ["app.js", "style.css", "library.js", "selection.js", "presets.js", "sequence.js", "panels.js"]},
                **{name: ROOT / "web" / name for name in ["plugin-backend.js", "plugin.css", "standalone-backend.js", "native-library.js", "standalone.css"]},
            }
            file = paths.get(name)
            if file is None or not file.is_file():
                self.send_error(404)
                return
            body = file.read_bytes()
            if name == "index.html":
                for script_name in ["library", "selection", "presets", "sequence", "app"]:
                    body = body.replace(f'    <script src="./{script_name}.js" defer></script>\n'.encode(), b"")
                stylesheet, backend = ("standalone.css", "standalone-backend.js") if "app" in url.query else ("plugin.css", "plugin-backend.js")
                body = body.replace(b"</head>", f'<link rel="stylesheet" href="./{stylesheet}" /><script type="module" src="./{backend}"></script></head>'.encode())
                endpoint = "/native?app" if "app" in url.query else "/native?fx" if "fx" in url.query else "/native"
                script = f'<script>window.sadPetNativeRequest = async (request) => (await fetch({json.dumps(endpoint)}, {{method:"POST",headers:{{"Content-Type":"application/json"}},body:JSON.stringify(request)}})).json();</script>'
                body = body.replace(b"</head>", script.encode() + b"</head>")
            self.send_response(200)
            self.send_header("Content-Type", {".html": "text/html", ".js": "text/javascript", ".css": "text/css"}[file.suffix])
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

    server = HTTPServer(("127.0.0.1", args.port), Handler)
    print(f"Plugin editor: http://127.0.0.1:{args.port}/ (instrument), /?fx (effect), /?app (standalone app)", flush=True)
    print(f"App library: {library}", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
        for controller, audio, _ in instances:
            lib.sad_audio_free(audio)
            lib.sad_control_free(controller)


if __name__ == "__main__":
    main()
