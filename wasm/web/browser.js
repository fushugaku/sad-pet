"use strict";
(() => {
  let backend;
  const ready = import("./browser-backend.mjs").then(
    async ({ BrowserBackend }) => {
      backend = new BrowserBackend();
      await backend.ready;
      return backend;
    },
  );
  // Keep the native panel's API contract without a server or a fetch override.
  window.sadPetFetch = (...args) =>
    ready.then((engine) => engine.fetch(...args));
  // State already lives on this thread. Avoid encoding and parsing the entire
  // preset through a synthetic HTTP Response at every UI tick.
  window.sadPetState = () => ready.then((engine) => engine.snapshot());
  window.sadPetAudio = {
    status: () => backend?.status || "Loading WebAssembly…",
  };
  window.sadPetResume = () => backend?.resume();
  // Resume synchronously during the gesture, including keyboard note entry.
  for (const event of ["pointerdown", "keydown"])
    document.addEventListener(event, () => backend?.resume(), {
      capture: true,
    });
  ready.catch((error) => {
    const notice = document.getElementById("notice");
    notice.hidden = false;
    notice.textContent = `Cannot start SAD PET: ${error.message}`;
  });
})();
