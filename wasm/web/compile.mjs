// Keep one compiled module for all workers; compile while its bytes arrive.
export async function compileEngine(url) {
  const response = await fetch(url);
  if (!response.ok) throw new Error("Cannot download the WebAssembly engine.");
  // Some engines reject parameters such as "; charset=utf-8" for WASM.
  const mime = response.headers.get("content-type")?.trim();
  if (typeof WebAssembly.compileStreaming === "function" && mime === "application/wasm")
    return WebAssembly.compileStreaming(response);
  // Static hosts with a generic MIME type and older webviews retain byte loading.
  return WebAssembly.compile(await response.arrayBuffer());
}
