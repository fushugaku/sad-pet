// Canonical user waves persist independently of source WAV paths. Factory
// packs are compiled into the engine and need neither this store nor a fetch.
const database=()=>new Promise((resolve,reject)=>{
  const request=indexedDB.open("sad-pet-wavetables",1);
  request.onupgradeneeded=()=>request.result.createObjectStore("assets",{keyPath:"id"});
  request.onsuccess=()=>resolve(request.result);request.onerror=()=>reject(request.error);
});
const transaction=async(mode,action)=>{
  const db=await database();
  return new Promise((resolve,reject)=>{
    const tx=db.transaction("assets",mode),request=action(tx.objectStore("assets"));
    tx.oncomplete=()=>{db.close();resolve(request?.result);};
    tx.onerror=tx.onabort=()=>{db.close();reject(tx.error||new Error("Cannot save the wavetable library."));};
  });
};
export const userWavetables=()=>transaction("readonly",store=>store.getAll());
export const saveWavetable=asset=>transaction("readwrite",store=>store.put(asset));
