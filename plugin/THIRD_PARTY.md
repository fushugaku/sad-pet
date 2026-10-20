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
  The Windows installer includes Microsoft's original signed Evergreen Bootstrapper
  and Visual C++ Runtime installer. They run only when the corresponding runtime
  is missing/outdated and remain independently maintained Microsoft components.
  Download URLs, original signatures, versions and SHA-256 hashes are recorded in
  `Licenses/Microsoft-runtime-packages.json` in the Windows distribution.
- **Inno Setup 6** creates the Windows Setup/uninstaller. Its compiler is a build
  tool; generated installers include the Inno Setup installer engine.
  [License and installer distribution terms](https://github.com/jrsoftware/issrc/blob/main/license.txt).
- **Rust engine dependencies** are recorded in the workspace `Cargo.lock`,
  including serde, serde_json, hound, rtrb and base64. Preset compression uses
  flate2 with the pure Rust miniz_oxide backend; both offer MIT or Apache-2.0
  licensing.
  The plug-in dependency on
  `sad-pet` disables the standalone desktop feature, so it does not link CPAL,
  MIDI-device drivers, the HTTP server or the LAME encoder.
- **Sample sources** are inherited from SAD PET. Credits and usage boundaries are
  in `rust/assets/samples/SOURCES.txt`, included with each package. The Harvest
  Dawn cover is a new recording of a copyrighted composition by Jeremy Soule;
  it is not royalty-free music.
- **RADIAS Drive / Waveshaper reconstruction** is ported from radias-emulator's
  shared Rustias synthesis source at `6a7ce839d9c4e76f5568feffb47ec4527b1c526e`:
  `radias-synth-domain/src/waveshaper.rs`, `controller_shaper.rs`, the three used
  `fixed.rs` helpers, and the firmware-free `standalone_tables::shapers` formula.
  The original integer kernels and DEPTH transfers are retained in
  `rust/src/dsp/radias_waveshaper/`; a stereo adapter supplies host sample rate,
  MIDI pitch and the original four-sample coefficient slew. No firmware binary
  or reference recording is included in this port.

- **Rustias Resynth oscillator model** uses `radias-synth-domain` and
  `radias-synth-application`, pinned to
  [91cae3f57a868a80d32a2e6ebb113938efd65025](https://github.com/fushugaku/Rustias/tree/91cae3f57a868a80d32a2e6ebb113938efd65025).
  The OSC1 adapter and analytic table formulas are adapted from
  `radias-synth-infrastructure/src/circuit/primary.rs` and `standalone_tables.rs`.
  The shared fixed-point kernels, control transfers, phase initialization,
  feedback and four-frame coefficient slew are retained. SAD PET supplies its
  own host-rate 16-band analyzer, stereo adapter and oversampling/decimation.
  PCM uses the selected SAD PET sample; Audio In uses the analyzed audio bands.
  No hardware PCM ROM, firmware image or reference recording is distributed.
  The original Rustias source is maintained by the same repository owner; this
  notice does not assign it a different license.

This document records the added dependencies; it does not replace their license
texts or the repository's existing source and sample notices.

- **Web binary telemetry** uses rmp-serde/rmp (MIT), recorded in Cargo.lock.
  They run only in the web build and are not linked into native plug-ins.
