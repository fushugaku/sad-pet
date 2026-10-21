import assert from "node:assert/strict";
import { compileEngine } from "./web/compile.mjs";

const wasm = Uint8Array.of(0, 97, 115, 109, 1, 0, 0, 0);
const originalFetch = globalThis.fetch;
const streaming = WebAssembly.compileStreaming;
const compile = WebAssembly.compile;
let streamed = 0, buffered = 0;
WebAssembly.compileStreaming = async response => { streamed++; return streaming(response); };
WebAssembly.compile = async bytes => { buffered++; return compile(bytes); };
try {
  const response = mime => new Response(wasm, { headers: { "content-type": mime } });
  globalThis.fetch = async () => {
    const result = response("application/wasm");
    result.arrayBuffer = () => { throw new Error("Streaming waited for the full file"); };
    return result;
  };
  assert.ok(await compileEngine("engine.wasm") instanceof WebAssembly.Module);
  assert.equal(streamed, 1);
  assert.equal(buffered, 0);
  globalThis.fetch = async () => response("application/octet-stream");
  assert.ok(await compileEngine("engine.wasm") instanceof WebAssembly.Module);
  assert.equal(buffered, 1);
  globalThis.fetch = async () => response("application/wasm; charset=utf-8");
  assert.ok(await compileEngine("engine.wasm") instanceof WebAssembly.Module);
  assert.equal(buffered, 2);
  WebAssembly.compileStreaming = undefined;
  globalThis.fetch = async () => response("application/wasm");
  assert.ok(await compileEngine("engine.wasm") instanceof WebAssembly.Module);
  assert.equal(buffered, 3);
  globalThis.fetch = async () => new Response("Not found", { status: 404 });
  await assert.rejects(compileEngine("engine.wasm"), /Cannot download/);
  WebAssembly.compileStreaming = streaming;
  globalThis.fetch = async () => new Response("invalid WASM", { headers: { "content-type": "application/wasm" } });
  await assert.rejects(compileEngine("engine.wasm"), WebAssembly.CompileError);
} finally {
  globalThis.fetch = originalFetch;
  WebAssembly.compileStreaming = streaming;
  WebAssembly.compile = compile;
}
console.log("WASM loader: streamed compilation, generic MIME/older webview fallback and download/compile errors passed.");
