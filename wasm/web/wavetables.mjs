// A scene or a single preset names tables; canonical user audio stays in
// the utility worker, and only prepared bank bytes cross to AudioWorklet.
export function wavetableIds(value) {
  const ids = new Set();
  const visit = (part) => {
    if (!part || typeof part !== "object") return;
    for (const oscillator of (part.wavetable?.oscillators || []).slice(0,part.wavetable?.count??4))
      if (typeof oscillator.asset_id === "string") ids.add(oscillator.asset_id);
    if (part.preset) visit(part.preset);
    if (part.state) visit(part.state);
    if (Array.isArray(part.timbres)) part.timbres.forEach(visit);
  };
  visit(value);
  return [...ids];
}
export function preparedWavetable(buffer) {
  if (!(buffer instanceof ArrayBuffer) || buffer.byteLength < 4)
    throw new Error("Incomplete prepared wavetable.");
  const size = new DataView(buffer).getUint32(0, true);
  if (size > 4096 || size + 4 >= buffer.byteLength)
    throw new Error("Invalid prepared wavetable header.");
  return {header:JSON.parse(new TextDecoder().decode(new Uint8Array(buffer,4,size))),
    prepared:buffer,offset:size+4};
}
