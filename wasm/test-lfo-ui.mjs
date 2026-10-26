// The LFO card's helpers: synced divisions with triplets and dots, the shaped
// cycle the scope previews, and the readouts of the new controls.
import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import vm from "node:vm";

const app = await readFile(new URL("../rust/web/app.js", import.meta.url), "utf8");
const start = app.indexOf("// Synced cycle lengths in beats");
const end = app.indexOf("// Route destinations the grain engine owns");
assert.ok(start > 0 && end > start, "LFO helpers moved");
const context = vm.createContext({
  percent: (v) => `${Math.round(v * 100)}%`,
  signedPercent: (v) => `${v > 0 ? "+" : ""}${Math.round(v * 100)}%`,
  envTime: (s) => (s < 1 ? `${Math.round(s * 1000)} ms` : `${s.toFixed(2)} s`),
});
vm.runInContext(
  `${app.slice(start, end)}; Object.assign(globalThis, { LFO_DIVISIONS, lfoDivision, lfoWave, lfoPreview, lfoControlText, lfoShapeName, lfoHasPhase });`,
  context,
);
const { LFO_DIVISIONS, lfoDivision, lfoWave, lfoPreview, lfoControlText, lfoShapeName, lfoHasPhase } =
  context;

// Every old option survives, in order of length, with triplets and dots between.
const beats = [...LFO_DIVISIONS].map(([b]) => b);
assert.deepEqual([...beats].sort((a, b) => a - b), beats);
for (const old of [0.25, 0.5, 1, 2, 4, 8, 16, 32, 64, 128]) assert.equal(lfoDivision(old)[0], old);
// Presets hold f32 values: a stored triplet still picks its option.
assert.equal(lfoDivision(Math.fround(1 / 3))[1], "1/8T");
assert.equal(lfoDivision(Math.fround(0.75))[1], "1/8.");
assert.equal(lfoDivision(Math.fround(8 / 3))[1], "1 bar T");
assert.equal(lfoDivision(100)[1], "32 bars");
assert.equal(lfoDivision(64)[1], "16 bars");

// The preview follows the engine's shapes.
const close = (a, b, label) => assert.ok(Math.abs(a - b) < 1e-9, `${label}: ${a} vs ${b}`);
close(lfoWave({ waveform: "sine" }, 0.25), 1, "plain sine");
close(lfoWave({ waveform: "sine", shape: 0.6 }, 0.55), 1, "skewed sine peak");
close(lfoWave({ waveform: "triangle", shape: 1 }, 0.3), -0.4, "triangle to saw up");
close(lfoWave({ waveform: "triangle", shape: -1 }, 0.3), 0.4, "triangle to saw down");
close(lfoWave({ waveform: "saw", shape: 1 }, 0.5), (2 * 0.5) / (0.5 + 0.5 / 16) - 1, "bent saw");
assert.equal(lfoWave({ waveform: "square", shape: 0.5 }, 0.74), 1);
assert.equal(lfoWave({ waveform: "square", shape: 0.5 }, 0.75), -1);
assert.equal(lfoPreview({ waveform: "random" }), null);
assert.equal(lfoPreview({ waveform: "envelope" }), null);

const plain = [...lfoPreview({ waveform: "sine" }, 80)];
assert.equal(plain.length, 81);
close(plain[20], 1, "a quarter in");
const shifted = lfoPreview({ waveform: "sine", phase: 0.25 }, 80);
close(shifted[0], 1, "phase starts the cycle a quarter on");
const uni = lfoPreview({ waveform: "triangle", unipolar: true }, 80);
assert.ok(Math.min(...uni) >= 0 && Math.max(...uni) <= 1);
const soft = lfoPreview({ waveform: "square", smooth: 1 }, 80);
const jump = (v) => Math.max(...v.slice(1).map((x, i) => Math.abs(x - v[i])));
assert.equal(jump(lfoPreview({ waveform: "square" }, 80)), 2);
assert.ok(jump(soft) < 0.3, `smoothed square still jumps ${jump(soft)}`);

// Readouts: the shape knob names what it does, in its wave's terms.
assert.equal(lfoShapeName("sine"), "SYMMETRY");
assert.equal(lfoShapeName("square"), "WIDTH");
assert.equal(lfoShapeName("smooth_random"), "CURVE");
assert.equal(lfoControlText({ waveform: "square", shape: 0.5 }, "shape"), "75%");
assert.equal(lfoControlText({ waveform: "saw", shape: -0.3 }, "shape"), "-30%");
assert.equal(lfoControlText({ phase: 0.25 }, "phase"), "90°");
assert.equal(lfoControlText({ smooth: 0.4 }, "smooth"), "40%");
assert.equal(lfoControlText({ delay: 0.5 }, "delay"), "1.25 s");
assert.equal(lfoControlText({ fade: 0.2 }, "fade"), "80 ms");
assert.ok(lfoHasPhase("saw") && !lfoHasPhase("random") && !lfoHasPhase("envelope"));

console.log("LFO card: divisions with triplets and dots, shaped previews and readouts passed.");
