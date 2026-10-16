# Third-party components

- **JUCE 8.0.16-security**, pinned to commit
  `a74b024c188339fdfe07b73088e6f6961ff66709`, provides VST3/AU wrappers and the
  embedded WebKit/WebView2 editor. JUCE offers AGPLv3 or JUCE commercial licensing.
  This integration does not select a new license for SAD PET. Distributing a
  combined binary requires satisfying the applicable JUCE license. Preserve the
  original `LICENSE.md` from the JUCE checkout; the packaging script includes it.
  [JUCE source and license](https://github.com/juce-framework/JUCE/tree/8.0.16-security).
- **Microsoft WebView2** is used for the Windows editor. The SDK loader is linked
  statically; the separately installed Microsoft Edge WebView2 Runtime renders
  the interface. [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/).
- **Rust engine dependencies** are recorded in the workspace `Cargo.lock`,
  including serde, serde_json, hound, rtrb and base64. The plug-in dependency on
  `sad-pet` disables the standalone desktop feature, so it does not link CPAL,
  MIDI-device drivers, the HTTP server or the LAME encoder.
- **Sample sources** are inherited from SAD PET. Credits and usage boundaries are
  in `rust/assets/samples/SOURCES.txt`, included with each package. The Harvest
  Dawn cover is a new recording of a copyrighted composition by Jeremy Soule;
  it is not royalty-free music.

This document records the added dependencies; it does not replace their license
texts or the repository's existing source and sample notices.
