#!/usr/bin/env python3
"""Run the filesystem frontend and native audio backend as one foreground job."""
import argparse
import http.client
import json
import mimetypes
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlsplit
import urllib.request
import webbrowser

ROOT = Path(__file__).resolve().parents[2]
WEB = ROOT / "rust" / "web"
MAX_BODY = 128 * 1024 * 1024
ASSETS = {"/": "index.html", "/index.html": "index.html", "/app.js": "app.js",
          "/library.js": "library.js", "/selection.js": "selection.js",
          "/presets.js": "presets.js", "/style.css": "style.css"}
CSP = ("default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; "
       "img-src 'self' data:; object-src 'none'; base-uri 'none'; frame-ancestors 'none'")


class Frontend(BaseHTTPRequestHandler):
    def log_message(self, *_args):
        pass

    def reply(self, status, body, mime="application/json", extra=()):
        self.send_response(status)
        self.send_header("Content-Type", mime)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.send_header("X-Content-Type-Options", "nosniff")
        if mime.startswith("text/html"):
            self.send_header("Content-Security-Policy", CSP)
        for key, value in extra:
            self.send_header(key, value)
        self.end_headers()
        if self.command != "HEAD":
            self.wfile.write(body)

    def handle_request(self):
        allowed = {f"127.0.0.1:{self.server.server_port}", f"localhost:{self.server.server_port}"}
        origin = self.headers.get("Origin")
        if self.headers.get("Host") not in allowed or (origin and origin not in {f"http://{host}" for host in allowed}):
            self.reply(403, b'{"error":"Local origin required"}')
            return
        path = urlsplit(self.path).path
        if path.startswith("/api/"):
            try:
                length = int(self.headers.get("Content-Length", 0))
                if not 0 <= length <= MAX_BODY or self.headers.get("Transfer-Encoding"):
                    self.reply(413, b'{"error":"File exceeds 128 MB or uses an unsupported transfer encoding"}')
                    return
                body = self.rfile.read(length) if length else None
                headers = {key: self.headers[key] for key in ("Content-Type", "X-File-Name", "X-Library-Id") if key in self.headers}
                if origin:
                    headers["Origin"] = f"http://127.0.0.1:{self.server.backend_port}"
                connection = http.client.HTTPConnection("127.0.0.1", self.server.backend_port, timeout=180)
                try:
                    connection.request(self.command, self.path, body, headers)
                    response = connection.getresponse()
                    data = response.read()
                    extra = [(key, value) for key, value in response.getheaders() if key.lower() == "content-disposition"]
                    self.reply(response.status, data, response.getheader("Content-Type", "application/json"), extra)
                finally:
                    connection.close()
            except (OSError, ValueError, http.client.HTTPException) as error:
                self.reply(502, json.dumps({"error": f"Backend unavailable: {error}"}).encode())
        elif self.command in ("GET", "HEAD"):
            if path == "/favicon.ico":
                self.reply(200, b"", "image/x-icon")
            elif path in ASSETS:
                file = WEB / ASSETS[path]
                mime = mimetypes.guess_type(file.name)[0] or "application/octet-stream"
                self.reply(200, file.read_bytes(), f"{mime}; charset=utf-8")
            else:
                self.reply(404, b'{"error":"Route not found"}')
        else:
            self.reply(405, b'{"error":"Method not allowed"}')

    def do_GET(self):
        self.handle_request()

    def do_HEAD(self):
        self.handle_request()

    def do_POST(self):
        self.handle_request()


def port(value):
    try:
        number = int(value)
        if 1 <= number <= 65535:
            return number
    except ValueError:
        pass
    raise argparse.ArgumentTypeError("port must be between 1 and 65535")


def stop_children(children):
    # Each child owns a session/process group, including cargo's compiler children.
    # Signal all groups before waiting, so neither service is left running alone.
    for child in reversed(children):
        try:
            os.killpg(child.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
    deadline = time.monotonic() + 4
    for child in reversed(children):
        try:
            child.wait(timeout=max(0, deadline - time.monotonic()))
        except subprocess.TimeoutExpired:
            pass
    for child in reversed(children):
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        child.wait()


def run(args):
    if args.frontend_port == args.backend_port:
        raise RuntimeError("Frontend and backend need different ports.")
    for number in (args.frontend_port, args.backend_port):
        with socket.socket() as probe:
            try:
                probe.bind(("127.0.0.1", number))
            except OSError as error:
                raise RuntimeError(f"Port {number} is busy. Stop its application or choose --frontend-port / --backend-port.") from error

    stopped = threading.Event()
    exit_signal = 0

    def handle_signal(number, _frame):
        nonlocal exit_signal
        exit_signal = number
        stopped.set()

    for number in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        signal.signal(number, handle_signal)
    children = []

    def start(command):
        child = subprocess.Popen(command, cwd=ROOT, start_new_session=True)
        children.append(child)
        return child

    try:
        print("Building Rust audio backend…", flush=True)
        build = start(["cargo", "build", "--release", "--locked"])
        while build.poll() is None and not stopped.wait(0.1):
            pass
        if stopped.is_set():
            return 128 + exit_signal
        if build.returncode:
            return build.returncode
        children.remove(build)
        command = [str(ROOT / "target" / "release" / "sad-pet"), "serve", "--no-open", "--port", str(args.backend_port)]
        if args.no_audio:
            command.append("--no-audio")
        backend = start(command)
        deadline = time.monotonic() + 20
        while not stopped.is_set():
            if backend.poll() is not None:
                raise RuntimeError(f"Backend exited during startup ({backend.returncode}).")
            try:
                with urllib.request.urlopen(f"http://127.0.0.1:{args.backend_port}/api/state", timeout=0.5) as response:
                    json.load(response)
                break
            except (OSError, ValueError):
                if time.monotonic() >= deadline:
                    raise RuntimeError("Backend did not become ready.")
                stopped.wait(0.1)
        if stopped.is_set():
            return 128 + exit_signal
        frontend = start([sys.executable, str(Path(__file__).resolve()), "--frontend", "--frontend-port", str(args.frontend_port), "--backend-port", str(args.backend_port)])
        url = f"http://127.0.0.1:{args.frontend_port}/"
        print(f"Frontend  {url}  (PID {frontend.pid})\nBackend   http://127.0.0.1:{args.backend_port}/  (PID {backend.pid})\nCtrl+C stops both services.", flush=True)
        if not args.no_open:
            # Give the frontend its bind step before opening the user's browser.
            while not stopped.wait(0.05):
                if frontend.poll() is not None:
                    break
                try:
                    with urllib.request.urlopen(url, timeout=0.5):
                        webbrowser.open(url)
                    break
                except OSError:
                    pass
        while not stopped.wait(0.1):
            for label, child in (("Frontend", frontend), ("Backend", backend)):
                if child.poll() is not None:
                    print(f"{label} stopped ({child.returncode}).", flush=True)
                    return child.returncode if child.returncode >= 0 else 128 - child.returncode
        return 128 + exit_signal
    finally:
        print("Stopping frontend and backend…", flush=True)
        stop_children(children)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--frontend-port", type=port, default=os.environ.get("SAD_FRONTEND_PORT", os.environ.get("EVIL_FRONTEND_PORT", "3214")))
    parser.add_argument("--backend-port", type=port, default=os.environ.get("SAD_BACKEND_PORT", os.environ.get("EVIL_BACKEND_PORT", "3215")))
    parser.add_argument("--no-open", action="store_true", help="do not open the browser")
    parser.add_argument("--no-audio", action="store_true", help="disable native audio for automated tests")
    parser.add_argument("--frontend", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.frontend:
        with ThreadingHTTPServer(("127.0.0.1", args.frontend_port), Frontend) as server:
            server.backend_port = args.backend_port
            server.serve_forever()
        return 0
    try:
        return run(args)
    except (OSError, RuntimeError) as error:
        print(f"dev: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
