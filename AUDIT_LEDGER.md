# Starling → RustChunk audit ledger

- Source repository: `starling-client/starling`
- Source ref: `starling`
- Source tree snapshot recorded for this audit: `5432370d573642041585c69b40fb045b27133488`
- Target repository: `imamirhosein/RustChunk`
- Target branch: `starling-1.20.1-port-audit`
- Scope: assess Starling's architecture and implementation for lessons or components relevant to a Rust-based Android launcher/client targeting Minecraft Java 1.20.1 Forge. Starling's documented target is Java 26.3; no compatibility with Forge 1.20.1 is assumed.
- Rule: a file is complete only after its full contents (or all consecutive ≤400-line chunks) have been read, analyzed, and recorded here. Claims in documentation remain claims until verified in implementation.
- Decision labels: **essential**, **adaptable**, **rewrite required**, **integrate**, **unnecessary**, **undetermined**.

## Completed files

### 1. `README.md`
- **Source SHA:** `3cd9d680757217774270f477ecbfc9c80563e39f`
- **Size:** 146 lines; 17,521 characters; full file fetched and read.
- **Coverage:** lines 1–146, complete. No chunks omitted.
- **Purpose/content:** project overview, legal boundary, Android capabilities/status, mod formats, build prerequisites and workflow, Minecraft resource preparation, contribution/licensing information, and supported-device claims.
- **Architecture evidence stated by the README:**
  - Starling is described as an Android port/fork of Horizon, whose game implementation is rewritten in Rust; the Android app is positioned around Horizon rather than being a conventional JVM launcher.
  - It targets Minecraft Java 26.3 and explicitly requires user-owned game files; Minecraft code/assets/data are not bundled.
  - It describes two mod paths: native `.hnmod` code compiled on-device (ARM64 only) and data-only `.hmod` content mods.
  - It describes Android APK packaging via `packaging/android/build_apk.py`, and host-side preparation via `horizon-prepare`; source game inputs are checksum-pinned through `research-lock.json`.
  - It documents Android account sign-in plus offline profiles, server support, shader/resource packs, touch controls, diagnostics, and device-specific performance settings. These are project claims, not independently validated implementation findings.
- **Direct references / likely dependencies:** `roadmap/starling.md`, `docs/horizon-parity.md`, `docs/android-armv7.md`, `docs/android-controls.md`, `docs/android-accounts.md`, `docs/android-diagnostics.md`, `docs/native-mods.md`, `docs/starling-native-mods.md`, `docs/mods.md`, `packaging/android/build_apk.py`, `packaging/lowbudget/make.py`, `research-lock.json`, `Cargo.toml`, `Cargo.lock`, `CONTRIBUTING.md`, `THIRD_PARTY_NOTICES.md`, and `LICENSE`. These are README links/references; actual implementation dependencies and call relationships remain to be checked in their respective files.
- **RustChunk relevance:** high as a system-level map and requirements inventory. Particularly useful topics to investigate are the Rust/native execution boundary, Android packaging, legal/user-supplied game-resource preparation, ABI/GPU limitations, input abstraction, account/profile handling, diagnostics, and separation of code mods from data mods.
- **Important incompatibility / risk:** the documented target is 26.3, while RustChunk's target is 1.20.1 Forge. The README gives no evidence that Starling supports Forge, Forge's mod-loading lifecycle, Java bytecode mods, or the 1.20.1 protocol/runtime. Starling's native/content mod formats must not be treated as Forge compatibility. A Rust rewrite of the game is also a fundamentally different execution model from launching the original Forge Java runtime.
- **Preliminary decision:** **adaptable** for high-level Android product design, diagnostics, touch controls, resource acquisition/verification, and build organization; **rewrite required** for any feature whose implementation assumes Starling/Horizon internals or version 26.3; Forge compatibility remains **undetermined** and must be investigated explicitly.
- **Follow-up checks:** validate each documented capability against source code; inspect Cargo workspace/crates and build pipeline; trace how 26.3 files are prepared and consumed; inspect account/auth and offline-mode semantics; determine whether any Java/FML/Forge bridge exists (none is established by this README); compare supported ABIs and graphics APIs to RustChunk's target hardware.
- **Completion state:** complete documentation review; implementation claims not yet validated.

## Pending first batch (selected order)

2. `LICENSE`
3. `THIRD_PARTY_NOTICES.md`
4. `CONTRIBUTING.md`
5. `Cargo.toml`
6. `Cargo.lock`
7. `.gitignore`
8. `.github/workflows/android.yml`
9. `packaging/android/build_apk.py`
10. `packaging/lowbudget/make.py`
11. `research-lock.json`
12. `roadmap/starling.md`
13. `docs/horizon-parity.md`
14. `docs/android-armv7.md`
15. `docs/android-controls.md`
16. `docs/android-accounts.md`
17. `docs/android-diagnostics.md`
18. `docs/native-mods.md`
19. `docs/starling-native-mods.md`
20. `docs/mods.md`


### 2. `LICENSE`
- **Source SHA:** `e959a076e9fa4b8c1fcf990b2253a935da5bb5f4`
- **Size:** 22 lines; full file fetched and read.
- **Coverage:** complete; no chunks omitted.
- **Purpose/content:** standard MIT license text. It attributes the copyright to Ludovic Marie and Horizon contributors (2026), grants broad rights to use, modify, distribute, sublicense, and sell the licensed software, requires retaining the notice, and disclaims warranties and liability.
- **Dependencies / scope:** this file is the repository's top-level license declaration. It does not enumerate third-party dependency licenses or independently grant rights to Minecraft/Mojang assets. The README explicitly limits MIT coverage to original project work; `THIRD_PARTY_NOTICES.md` is the next source for component-specific attribution.
- **RustChunk relevance:** critical due-diligence input before copying or adapting code. MIT is permissive, but the copyright notice and license text must be retained for substantial copied portions; dependencies and upstream code may have separate obligations.
- **Limitations:** a root MIT license alone does not establish the license of every vendored/generated/resource file or third-party dependency. Do not infer that game assets, Minecraft binaries, or third-party libraries are covered.
- **Decision:** **adaptable** as a licensing model reference; copying Starling/Horizon code requires preserving notices and checking each dependency's actual license. Minecraft files remain outside this license scope.
- **Completion state:** complete license-text review; third-party notices and dependency metadata still require separate review.

### 3. `THIRD_PARTY_NOTICES.md`
- **Source SHA:** `825d4bb50daf691d9049b3d4cfdbc62dbc285ec2`
- **Size:** 300 lines; 20,016 characters; full file fetched and read.
- **Coverage:** lines 1–300, complete.
- **Purpose/content:** detailed attribution and license inventory for third-party code, ports, assets, fonts, icons, fetched runtimes, and Horizon Android adaptations.
- **Components and material disclosures found:**
  - Vendored `wgpu-hal` and `naga` 30.0.1, each MIT OR Apache-2.0, with documented upstream fixes; `libmimalloc-sys`/mimalloc sources under MIT, including a portable-popcount change.
  - Significant Create 6.0.10 mechanical/processing system ports into Horizon game crates; Create code MIT, while Create assets are All Rights Reserved and fetched from Modrinth on the player's device rather than included.
  - Iris Features sky-shape logic, MIT; Pixelify Sans under SIL OFL 1.1; Pixelarticons and Hugeicons sets under MIT; Discord mark from Simple Icons under CC0, with trademark note.
  - AMD FidelityFX SDK 1.1.4 optical-flow/frame-interpolation shaders and related implementation, ported to WGSL/Rust under MIT with AMD attribution/trademark notice.
  - Loader logos (Fabric, Quilt, NeoForge, Forge) with differing logo licenses/marks; Microsoft sign-in branding copied from Microsoft's official symbol/branding guide.
  - Semantics from CIT Resewn Continuation and layout tables derived from FusionExamples, both MIT; BlueMap low-resolution map tile/settings behavior, MIT.
  - OpenJDK 25 runtimes are fetched on-device for preparation and deleted afterward; the fetched JDK is GPLv2 with Classpath Exception. The Starling C launcher is MIT.
  - Bedrock touch-button art is fetched from Mojang's samples repository and governed by Mojang's license/EULA; Starling ships fallback buttons of its own under MIT.
  - A large set of Horizon Android adaptations are explicitly identified: legacy GLES shader/render paths, diagnostics and performance recording, Android Vulkan/Adreno paths, Android Documents provider, touch controls/editor/keypad, provider-aware accounts/Ely.by, account wardrobe/skin upload, and HUD layout. The notice points to source files and upstream revision `fa548eb7abe91c194d3780e4580b9746323cd606` and states MIT attribution.
- **Dependencies / cross-references:** points to `third_party/wgpu-hal`, `third_party/naga`, `third_party/libmimalloc-sys`, `crates/horizon-client`, `crates/horizon-client-render`, `crates/horizon-optifine`, `crates/horizon-theme`, `crates/starling-account`, `launcher`, Android packaging Java/C sources, asset manifests, and preparation scripts. These are evidence of named locations, not yet a source-level dependency graph.
- **RustChunk relevance:** very high. This is the clearest warning against treating the upstream repository as a single uniform MIT codebase. A port/reuse inventory must separate original MIT code, dual-licensed dependencies, GPL-with-exception runtimes, OFL fonts, CC0/MIT icon sets, trademark artwork, and Mojang/Create assets. It also shows how Starling deliberately downloads some proprietary/game-adjacent assets and runtimes to the device instead of bundling them.
- **Risks / limitations:** this document is a self-authored notice, not a substitute for validating each file, bundled license text, upstream commit, or redistribution terms. The notice records code adapted from Horizon Android; RustChunk should preserve attribution and inspect the relevant source history before reusing it. The Minecraft/Bedrock assets and Create assets must not be assumed redistributable with the app. GPLv2 + Classpath Exception for OpenJDK does not automatically license the Starling app itself, but the exact runtime distribution and build arrangement still require checking.
- **Decision:** **essential** as a compliance/attribution checklist; **adaptable** as a model for RustChunk's own third-party notices and on-device fetching policy. Individual libraries/features are not approved for reuse until their code and license files are checked.
- **Follow-up checks:** compare notices against `Cargo.toml`/`Cargo.lock` and actual vendored directories; inspect all referenced license files; trace the Create asset fetch; inspect JDK archive acquisition/checksum code; validate Mojang touch assets are never bundled; identify which Horizon Android adaptations exist in current Starling tree and their exact source diffs.
- **Completion state:** complete review of the notice document; individual licenses, code, and runtime distribution mechanisms remain open audits.
