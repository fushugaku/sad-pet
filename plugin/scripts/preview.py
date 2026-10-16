#!/usr/bin/env python3
"""Browser QA of the plugin editor against the real native engine (no sound output)."""
import argparse
import ctypes as c
import json
import math
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
    args = parser.parse_args()
    lib = c.CDLL(str(args.engine))
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
    for effect in [False, True]:
        controller = lib.sad_control_new(effect)
        audio = lib.sad_control_prepare(controller, 48000)
        instances.append((controller, audio, (c.c_float * size)()))
    left, right = (c.c_float * 2048)(), (c.c_float * 2048)()
    incoming = (c.c_float * 2048)(*(0.15 * math.sin(i * 0.04) for i in range(2048)))

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
            controller, audio, values = instances[1 if "fx" in url.query else 0]
            if request.get("path") == "/api/state":
                lib.sad_control_get_parameters(controller, values, size)
                lib.sad_audio_begin(audio, values, size, 120.0, 0.0, False, False, 2048)
                lib.sad_audio_process(audio, incoming if "fx" in url.query else None, incoming if "fx" in url.query else None, left, right, 2048)
                lib.sad_audio_end(audio, 2048)
            response = take(lib.sad_control_request(controller, data, len(data)))
            body = json.dumps(response, separators=(",", ":")).encode()
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
                **{name: PROJECT / "rust/web" / name for name in ["app.js", "style.css", "library.js", "selection.js", "presets.js"]},
                **{name: ROOT / "web" / name for name in ["plugin-backend.js", "plugin.css"]},
            }
            file = paths.get(name)
            if file is None or not file.is_file():
                self.send_error(404)
                return
            body = file.read_bytes()
            if name == "index.html":
                for script_name in ["library", "selection", "presets", "app"]:
                    body = body.replace(f'    <script src="./{script_name}.js" defer></script>\n'.encode(), b"")
                body = body.replace(b"</head>", b'<link rel="stylesheet" href="./plugin.css" /><script type="module" src="./plugin-backend.js"></script></head>')
                endpoint = "/native?fx" if "fx" in url.query else "/native"
                script = f'<script>window.sadPetNativeRequest = async (request) => (await fetch({json.dumps(endpoint)}, {{method:"POST",headers:{{"Content-Type":"application/json"}},body:JSON.stringify(request)}})).json();</script>'
                body = body.replace(b"</head>", script.encode() + b"</head>")
            self.send_response(200)
            self.send_header("Content-Type", {".html": "text/html", ".js": "text/javascript", ".css": "text/css"}[file.suffix])
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

    server = HTTPServer(("127.0.0.1", args.port), Handler)
    print(f"Plugin editor: http://127.0.0.1:{args.port}/ (instrument), /?fx (effect)", flush=True)
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
