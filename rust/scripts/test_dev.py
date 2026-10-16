#!/usr/bin/env python3
"""Exercise the real two-process launcher and API proxy using disposable ports."""
import io
import json
import os
from pathlib import Path
import re
import signal
import socket
import struct
import subprocess
import tempfile
import time
import unittest
import urllib.error
import urllib.request
import wave

ROOT = Path(__file__).resolve().parents[2]


def free_port():
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


def reachable(port):
    with socket.socket() as sock:
        sock.settimeout(0.2)
        return sock.connect_ex(("127.0.0.1", port)) == 0


class DevelopmentStack(unittest.TestCase):
    def launch(self):
        front, back = free_port(), free_port()
        while back == front:
            back = free_port()
        log = tempfile.TemporaryFile(mode="w+")
        process = subprocess.Popen([str(ROOT / "dev.sh"), "--no-open", "--no-audio",
                                    "--frontend-port", str(front), "--backend-port", str(back)],
                                   cwd="/tmp", stdout=log, stderr=subprocess.STDOUT,
                                   start_new_session=True)

        def cleanup():
            if process.poll() is None:
                process.terminate()
                process.wait(timeout=8)
            log.close()

        self.addCleanup(cleanup)
        deadline = time.monotonic() + 45
        while time.monotonic() < deadline:
            try:
                with urllib.request.urlopen(f"http://127.0.0.1:{front}/api/state", timeout=0.3) as response:
                    state = json.load(response)
                break
            except (OSError, ValueError):
                if process.poll() is not None:
                    log.seek(0)
                    self.fail(log.read())
                time.sleep(0.05)
        else:
            self.fail("dev stack did not start")
        log.seek(0)
        pids = {name: int(pid) for name, pid in re.findall(r"(Frontend|Backend)\s+http[^\n]*\(PID (\d+)\)", log.read())}
        self.assertEqual(set(pids), {"Frontend", "Backend"})
        self.assertNotEqual(pids["Frontend"], pids["Backend"])
        return process, front, back, state, pids

    def assert_stopped(self, process, front, back, pids):
        process.wait(timeout=8)
        self.assertFalse(reachable(front))
        self.assertFalse(reachable(back))
        for pid in pids.values():
            with self.assertRaises(ProcessLookupError):
                os.kill(pid, 0)

    def test_proxy_assets_audio_upload_origin_and_ctrl_c_cleanup(self):
        process, front, back, state, pids = self.launch()
        root = f"http://127.0.0.1:{front}"
        for name in ("library.js", "selection.js", "presets.js"):
            with urllib.request.urlopen(root + "/" + name) as response:
                self.assertEqual(response.read(), (ROOT / "rust/web" / name).read_bytes())
                self.assertEqual(response.headers["Cache-Control"], "no-store")
        with urllib.request.urlopen(f"http://127.0.0.1:{back}/api/state") as response:
            self.assertEqual(json.load(response)["instance_id"], state["instance_id"])
        data = io.BytesIO()
        with wave.open(data, "wb") as writer:
            writer.setnchannels(1)
            writer.setsampwidth(2)
            writer.setframerate(8000)
            writer.writeframes(b"\x00\x01" * 800)
        request = urllib.request.Request(root + "/api/wav", data=data.getvalue(), headers={
            "Content-Type": "audio/wav", "X-File-Name": "dev-import.wav",
            "X-Library-Id": "audio:dev-integration", "Origin": root})
        with urllib.request.urlopen(request) as response:
            self.assertEqual(json.load(response)["id"], "audio:dev-integration")
        request = urllib.request.Request(root + "/api/transport", data=b'{"playing":true}', headers={
            "Content-Type": "application/json", "Origin": "https://example.invalid"})
        with self.assertRaises(urllib.error.HTTPError) as caught:
            urllib.request.urlopen(request)
        self.assertEqual(caught.exception.code, 403)
        caught.exception.close()
        process.send_signal(signal.SIGINT)
        self.assert_stopped(process, front, back, pids)
        self.assertEqual(process.returncode, 130)

    def test_child_exit_stops_its_sibling(self):
        process, front, back, _state, pids = self.launch()
        os.kill(pids["Frontend"], signal.SIGTERM)
        self.assert_stopped(process, front, back, pids)
        self.assertEqual(process.returncode, 143)

    def test_recording_preserves_live_parameter_changes_and_exports_both_formats(self):
        _process, front, _back, state, _pids = self.launch()
        root = f"http://127.0.0.1:{front}"

        def post(path, data):
            request = urllib.request.Request(root + path, json.dumps(data).encode(),
                                             {"Content-Type": "application/json", "Origin": root})
            with urllib.request.urlopen(request, timeout=10) as response:
                return json.load(response)

        post("/api/transport", {"playing": True})
        post("/api/recording", {"active": True})
        time.sleep(0.2)
        preset = state["preset"]
        preset["params"]["master_volume"] = 0
        post("/api/preset", preset)
        time.sleep(0.25)
        take = post("/api/recording", {"active": False})
        self.assertTrue(take["available"])
        self.assertFalse(take["active"])
        self.assertGreater(take["seconds"], 0.4)
        self.assertEqual(post("/api/recording", {"active": False})["id"], take["id"])
        with urllib.request.urlopen(root + "/api/recording.wav?id=" + take["id"]) as response:
            wav = response.read()
            self.assertEqual(response.headers["Content-Type"], "audio/wav")
        offset = 12
        samples = None
        while offset + 8 <= len(wav):
            chunk = wav[offset:offset + 4]
            size = struct.unpack_from("<I", wav, offset + 4)[0]
            if chunk == b"data":
                samples = struct.unpack_from(f"<{size // 4}f", wav, offset + 8)
                break
            offset += 8 + size + (size & 1)
        self.assertIsNotNone(samples)
        self.assertAlmostEqual(len(samples) / 2 / state["sample_rate"], take["seconds"])
        self.assertGreater(max(map(abs, samples[:len(samples) // 3])), 0.01)
        self.assertLess(max(map(abs, samples[-4000:])), 0.0001)
        with urllib.request.urlopen(root + "/api/recording.mp3?id=" + take["id"]) as response:
            mp3 = response.read()
            self.assertEqual(response.headers["Content-Type"], "audio/mpeg")
            self.assertGreater(len(mp3), 8000)
            self.assertTrue(any(mp3[i] == 255 and mp3[i + 1] == 251 for i in range(len(mp3) - 1)))
        with urllib.request.urlopen(root + "/api/state") as response:
            current = json.load(response)
        self.assertTrue(current["playing"])
        self.assertEqual(current["recording"]["id"], take["id"])

    def test_sigterm_stops_both(self):
        process, front, back, _state, pids = self.launch()
        process.terminate()
        self.assert_stopped(process, front, back, pids)
        self.assertEqual(process.returncode, 143)

    def test_busy_port_is_left_untouched(self):
        with socket.socket() as busy:
            busy.bind(("127.0.0.1", 0))
            busy.listen()
            front, back = busy.getsockname()[1], free_port()
            result = subprocess.run([str(ROOT / "dev.sh"), "--no-open", "--frontend-port", str(front),
                                     "--backend-port", str(back)], capture_output=True, text=True, timeout=10)
            self.assertEqual(result.returncode, 1)
            self.assertIn(f"Port {front} is busy", result.stderr)
            self.assertTrue(reachable(front))
            self.assertFalse(reachable(back))


if __name__ == "__main__":
    unittest.main(verbosity=2)
