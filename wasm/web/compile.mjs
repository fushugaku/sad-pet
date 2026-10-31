// Keep one compiled module for all workers; compile while its bytes arrive.
// Variant engines built with extra WebAssembly features are preferred where
// the browser validates them. DSP quality is retained; scalar/SIMD numerical
// parity is checked by the engine's render corpus. The tail-call engine
// (sad_pet.tc.wasm) runs the translated firmware block to block without a
// dispatch loop; the SIMD engine (sad_pet.simd.wasm) weighs grain reads four
// taps at a time.
const SIMD_PROBE = new Uint8Array([
  0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 123, 3, 2, 1, 0, 10, 10, 1,
  8, 0, 65, 0, 253, 15, 253, 98, 11,
]);
// (func (return_call 0))
const TAIL_CALL_PROBE = new Uint8Array([
  0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 6, 1, 4, 0,
  18, 0, 11,
]);
function validates(bytes) {
  try {
    return WebAssembly.validate(bytes);
  } catch {
    return false;
  }
}
export function supportsSimd() {
  return validates(SIMD_PROBE);
}
export function supportsTailCalls() {
  return validates(TAIL_CALL_PROBE);
}
async function compile(url) {
  const response = await fetch(url);
  if (!response.ok) throw new Error("Cannot download the WebAssembly engine.");
  // Some engines reject parameters such as "; charset=utf-8" for WASM.
  const mime = response.headers.get("content-type")?.trim();
  if (typeof WebAssembly.compileStreaming === "function" && mime === "application/wasm")
    return WebAssembly.compileStreaming(response);
  // Static hosts with a generic MIME type and older webviews retain byte loading.
  return WebAssembly.compile(await response.arrayBuffer());
}
export async function compileEngine(url, simdUrl, tailCallUrl) {
  const variants = [
    [tailCallUrl, supportsTailCalls],
    [simdUrl, supportsSimd],
  ];
  for (const [variant, supported] of variants) {
    if (!variant || !supported()) continue;
    try {
      return await compile(variant);
    } catch {
      // A host without that build, or a browser whose validator accepts
      // more than its compiler, falls back to the next engine.
    }
  }
  return compile(url);
}
