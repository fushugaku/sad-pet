// Keep one compiled module for all workers; compile while its bytes arrive.
// An optional second engine built with WebAssembly SIMD can be preferred where
// the browser validates it; its arithmetic is the same IEEE float math, so both
// engines render identical samples. The baseline is loaded by default because
// the SIMD build measured slower in V8.
const SIMD_PROBE = new Uint8Array([
  0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 123, 3, 2, 1, 0, 10, 10, 1,
  8, 0, 65, 0, 253, 15, 253, 98, 11,
]);
export function supportsSimd() {
  try {
    return WebAssembly.validate(SIMD_PROBE);
  } catch {
    return false;
  }
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
export async function compileEngine(url, simdUrl) {
  if (simdUrl && supportsSimd()) {
    try {
      return await compile(simdUrl);
    } catch {
      // A host without the SIMD build, or a browser whose validator accepts
      // more than its compiler, falls back to the baseline engine.
    }
  }
  return compile(url);
}
