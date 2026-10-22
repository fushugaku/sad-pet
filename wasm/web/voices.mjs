// Imported voicebanks (.sadvoice files) persist in their own IndexedDB store;
// the vocal render worker loads every one of them at startup.
const NAME = "sad-pet-voices";

function database() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(NAME, 1);
    request.onupgradeneeded = () =>
      request.result.createObjectStore("voices", { keyPath: "id" });
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}

function run(mode, action) {
  return database().then(
    (db) =>
      new Promise((resolve, reject) => {
        const transaction = db.transaction("voices", mode);
        const request = action(transaction.objectStore("voices"));
        transaction.oncomplete = () => {
          db.close();
          resolve(request?.result);
        };
        transaction.onerror = () => {
          db.close();
          reject(transaction.error);
        };
      }),
  );
}

export const listVoices = () => run("readonly", (store) => store.getAll());
export const saveVoice = (voice) => run("readwrite", (store) => store.put(voice));
export const removeVoice = (id) => run("readwrite", (store) => store.delete(id));

/** The JSON header of a .sadvoice file: name, author, language, units. */
export function voiceInfo(buffer) {
  const bytes = new Uint8Array(buffer);
  if (new TextDecoder().decode(bytes.subarray(0, 8)) !== "SADVOICE")
    throw new Error("Not a SAD PET voicebank");
  const view = new DataView(buffer);
  const length = view.getUint32(12, true);
  const header = JSON.parse(new TextDecoder().decode(bytes.subarray(16, 16 + length)));
  return header.info || {};
}

/** A stable id from a voice name: letters, digits and dashes. */
export function voiceId(name, taken) {
  const base =
    name
      .normalize("NFKC")
      .toLowerCase()
      .replace(/[^\p{L}\p{N}]+/gu, "-")
      .replace(/^-|-$/g, "")
      .slice(0, 40) || "voice";
  let id = base;
  for (let n = 2; taken.has(id); n++) id = `${base}-${n}`;
  return id;
}

/** Files of a .zip (stored or deflate), for voicebanks shared as archives. */
export async function unzip(buffer) {
  const bytes = new Uint8Array(buffer);
  const view = new DataView(buffer);
  let end = bytes.length - 22;
  while (end >= 0 && view.getUint32(end, true) !== 0x06054b50) end--;
  if (end < 0) throw new Error("Not a zip file");
  const count = view.getUint16(end + 10, true);
  let at = view.getUint32(end + 16, true);
  const files = [];
  const utf8 = new TextDecoder();
  let shiftJis = null;
  try {
    shiftJis = new TextDecoder("shift_jis");
  } catch {
    shiftJis = null;
  }
  for (let i = 0; i < count; i++) {
    if (view.getUint32(at, true) !== 0x02014b50) throw new Error("Damaged zip file");
    const flags = view.getUint16(at + 8, true);
    const method = view.getUint16(at + 10, true);
    const size = view.getUint32(at + 20, true);
    const nameLength = view.getUint16(at + 28, true);
    const extra = view.getUint16(at + 30, true);
    const comment = view.getUint16(at + 32, true);
    const local = view.getUint32(at + 42, true);
    const raw = bytes.subarray(at + 46, at + 46 + nameLength);
    // Japanese banks are often zipped with Shift-JIS names and no UTF-8 flag.
    let name = utf8.decode(raw);
    if (!(flags & 0x800) && name.includes("�") && shiftJis) name = shiftJis.decode(raw);
    at += 46 + nameLength + extra + comment;
    if (name.endsWith("/")) continue;
    const start =
      local + 30 + view.getUint16(local + 26, true) + view.getUint16(local + 28, true);
    const data = bytes.subarray(start, start + size);
    let content;
    if (method === 0) content = data.slice().buffer;
    else if (method === 8)
      content = await new Response(
        new Blob([data]).stream().pipeThrough(new DecompressionStream("deflate-raw")),
      ).arrayBuffer();
    else continue;
    files.push({ path: name.replace(/\\/g, "/"), data: content });
  }
  return files;
}
