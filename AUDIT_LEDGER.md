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

### 4. `CONTRIBUTING.md`
- **Source SHA:** `b326f21b72a3fb5e6d10601b1ec0697070400c5d`
- **Size:** 1 lines; full file fetched and read.
- **Coverage:** complete; no chunks omitted.
- **Purpose/content:** defines the boundary between changes for Horizon (shared game logic) and Starling (Android-specific app/integration), developer setup, device-testing expectations, source/license policy, PR checks, and public-tree hygiene.
- **Key architecture/process rules:**
  - Shared gameplay, world generation, desktop-shared rendering and menus are directed upstream to Horizon and are expected to be verified against vanilla with a parity harness.
  - Android-only work is directed to Starling; named examples include APK packaging, Android entry point, touch controls, game-folder/import logic, and Android-only UI.
  - Build/install commands use the Python APK script; phone testing should name the actual device. A local `horizon-args.txt` can pass launch arguments/environment variables; `--control-socket` plus `HORIZON_CONTROL_PORT` exposes a control channel over ADB.
  - The guide imposes a project contribution policy: only a whitelist of permissive/public-domain licenses may be read/copied/adapted; several other license families and unclear/no-license projects are declared off limits even to read. It requires checking licenses at the exact commit and file headers, reporting provenance in PRs, retaining notices, and recording reuse in `THIRD_PARTY_NOTICES.md`.
  - Before PR, developers install Git hooks and run `tools/pr_ready.py`; this checks formatting, Clippy, relevant tests, Python tests and public-tree hygiene. Game workspace dependency scopes and history checks are described.
- **Dependencies / cross-references:** `README.md`, upstream Horizon contributing guide, `packaging/android/build_apk.py`, `packaging/lowbudget/make.py`, `hand.py`, `strings.json`, `pack/`, `crates/horizon-client/src/android_lib.rs`, touch/import modules, `horizon-ctl`, `tools/pr_ready.py`, `tools/check_public_tree.py`, `.githooks`, and `THIRD_PARTY_NOTICES.md`. These named paths need source inspection before relying on their behavior.
- **RustChunk relevance:** high for project boundaries and quality gates. A similar split could prevent Android-specific changes from contaminating a reusable Rust/game core. The public-tree check and explicit device-test reporting are valuable safeguards, especially because game assets/prepared resources should not enter version control.
- **Compatibility warning:** the stated build targets 26.3 and NDK/SDK/JDK versions are not automatically transferable to a 1.20.1 Forge client. The license whitelist is Starling's own contributor policy, not a universal statement of copyright law; RustChunk should establish and document its own policy rather than silently assuming Starling's restrictions are legal requirements for every case.
- **Decision:** **adaptable** for core-vs-Android separation, device testing, PR gates, provenance records and public-tree protection. **Rewrite required** for toolchain/version policy and project-specific paths. The exact third-party policy is a candidate governance rule, not code to copy.
- **Follow-up checks:** inspect the named build and check scripts; inspect the repository's game/Android workspace boundaries; verify which tests and linters are actually run in CI; compare Starling's license policy with actual third-party notices for consistency.
- **Completion state:** complete guide review; referenced scripts and source modules not yet validated.

### 5. `Cargo.toml`
- **Source SHA:** `fb4a4a7d703c8ed35973c1690382d7ea2059d0eb`
- **Size:** 1 lines; full file fetched and read.
- **Coverage:** complete workspace manifest; no chunks omitted.
- **Purpose/content:** defines a large Rust workspace, shared package metadata, Clippy lint allowances, four local crates.io patches, and development/release/iteration build profiles.
- **Workspace structure observed:**
  - Edition 2024, declared workspace `rust-version = 1.85`, MIT package license, resolver 2.
  - Core/game-oriented members include `horizon-core`, world/protocol/items/commands, client/client-core/client-lib, split client input/render crates, Android NativeActivity crate, server, bots, friends/relay, asset preparation, shader/OptiFine compatibility, mods, account, theme, benchmark and release-signing crates.
  - `launcher`, telemetry server, mod registry, and vendored `wgpu-hal`, `naga`, `libmimalloc-sys`, and `winit` are explicitly excluded from the root workspace.
  - `horizon-android` is annotated as the game NativeActivity library and empty on non-Android targets; `horizon-mcp` is described as a shared Model Context Protocol component.
- **Pinned/local patches and rationale:** local `wgpu-hal` for Vulkan descriptor-pool/Windows driver, Metal HDR string handling and VSync-off fence behavior; local `winit` patch for Android raw input hook behavior; local `naga` patch for GLSL frontend shader-pack fixes; local `libmimalloc-sys` patch to avoid POPCNT on older MSVC CPUs. Comments say to drop patches when upstream releases contain fixes.
- **Build profile strategy:** dev builds retain line tables only; release uses thin LTO and one codegen unit; `fast` optimizes while linking faster; `quick` increases codegen units and disables LTO for iteration, with client/Android crates at opt-level 1.
- **Dependencies / cross-references:** direct workspace references to paths above, patch directories under `third_party`, README/CONTRIBUTING toolchain, `packaging/android/build_apk.py`, `crates/horizon-client/src/android_lib.rs`, and `THIRD_PARTY_NOTICES.md`. This manifest does not itself enumerate each crate's dependencies; per-crate manifests and Cargo.lock are needed.
- **RustChunk relevance:** very high for architecture mapping. It shows Starling is not just a thin Android launcher: most of the workspace is a Rust reimplementation of game/client/server systems, with Android as one target and separate input/render/core modules. The split crates, NativeActivity boundary, controlled local patches and fast/quick profiles are useful architectural/build ideas.
- **Major compatibility warning:** this structure is built around Horizon's 26.3 Rust game implementation; it is not evidence of Forge support. Forge 1.20.1 mods expect the Java/FML ecosystem; porting Starling's native game core would not by itself load Forge bytecode mods. RustChunk's target may require a different boundary: retain the original Java/Forge runtime and build Android integration around it, unless a deliberate incompatibility with Forge is accepted.
- **Notable configuration observation:** README/CONTRIBUTING demand Rust 1.90+, but this workspace manifest advertises `rust-version = 1.85`. That is not necessarily a build defect (toolchain may be pinned elsewhere or a dependency may need newer Rust), but it is a documentation/configuration discrepancy to resolve when the toolchain files are audited.
- **Decision:** **adaptable** for crate boundaries, Android-specific target isolation, local patch rationale tracking, and quick-vs-release profiles; **rewrite required** if adopting the Horizon gameplay code as the base for a 1.20.1 Forge-compatible launcher/client.
- **Follow-up checks:** inspect member crate manifests, Android NativeActivity source and JNI/NDK integration; verify `third_party` patch diffs and removal conditions; inspect toolchain files and CI to explain Rust 1.85 vs 1.90; map which crates are needed for Starling's phone app versus optional server/tools.
- **Completion state:** complete root manifest review; dependency graph and crate internals remain pending.

### 6. `Cargo.lock`
- **Source SHA:** `5f5a3e785ef55cb1e4ae408b7d4a2a1368e07754`
- **Size:** 5,510 newline-terminated lines (5,511 entries when splitting on the final newline); 132,369 characters; 541 package records; 498 crates.io registry package source entries; no Git-source package entries; lock format version 4.
- **Coverage:** fetched and read sequentially in consecutive chunks: 1–400, 401–800, 801–1,200, 1,201–1,600, 1,601–2,000, 2,001–2,400, 2,401–2,800, 2,801–3,200, 3,201–3,600, 3,601–4,000, 4,001–4,400, 4,401–4,800, 4,801–5,200, and 5,201–5,510. The file ends with a newline. No intended ranges skipped.
- **Purpose/content:** Cargo's generated lockfile pins the exact resolved Rust dependency versions and checksums, plus the workspace package dependency edges. It is not intended for manual editing.
- **Project packages / dependency topology:** the file records Starling/Horizon packages including the Android NativeActivity crate, core/client/client-lib/client-core/client-input/client-render, world/protocol/items, rendering/shaders/OptiFine compatibility, account, asset preparation/catalog, mods, friends/relay, server and related tools. The locked workspace edges confirm that `horizon-client` and `horizon-client-lib` sit over a broad shared Rust game/client stack, while Android crate `horizon-android` depends on `horizon-client`, JNI 0.21.1 and winit.
- **Important ecosystem packages observed:** Android activity/NDK/JNI crates; winit 0.30.13; wgpu/wgpu-core/wgpu-hal/naga 30.0.1; audio via cpal/symphonia; input/controller/HID via gilrs/hidapi; networking/TLS via ureq/rustls; serialization via serde; crypto/auth primitives; ZIP/compression/image/font support; wasmparser and minisign verification for mods; Windows/macOS/Linux/Wayland/X11/Web dependency families included in the cross-platform resolution.
- **Cross references and patch relationship:** matches the root `Cargo.toml` workspace packages and local patches. The lockfile's `wgpu-hal`, `naga`, `libmimalloc-sys`, and `winit` path packages have no registry source/checksum entries, consistent with the root path patches. It also contains multiple versions of packages such as JNI, objc2, rustix, socket2 and platform libraries, which are separate resolved versions rather than necessarily conflicts.
- **RustChunk relevance:** high as a dependency inventory and reproducibility baseline. It identifies practical Rust-side building blocks for Android native entry, graphics, user input, audio, archive/resource parsing, networking, account/security logic, and mod verification. It also demonstrates that Starling's dependency footprint is a full game engine/runtime stack rather than a conventional launcher's small wrapper.
- **Limitations / risks:** a lockfile alone does not show which target-specific packages are compiled into a particular APK, enabled Cargo features, runtime behavior, binary size, license compatibility, or whether a dependency is actually required at runtime. Registry checksums establish package identity, not safety or suitability. Platform-specific entries should not be mistaken for Android shipping dependencies. The presence of mod-related crates does not imply Forge/FML support; the package graph shows Horizon's native/content mod infrastructure instead.
- **Decision:** **adaptable** as a shortlist/reference for specific Android Rust subsystems after reviewing manifests and licenses; **unnecessary** to copy wholesale. Every dependency must be justified against RustChunk's narrower objective and 1.20.1 Forge constraint.
- **Follow-up checks:** inspect per-crate manifests and Cargo features to identify the Android build graph; verify patched vendor crates and licenses; map which deps are needed for renderer/input/auth/archive/Forge integration; check minimum toolchain and MSRV; assess whether any existing Rust library can interoperate with a retained Java/Forge runtime.
- **Completion state:** complete lockfile inventory review; feature activation, Android-specific resolution and runtime dependency necessity remain pending.
