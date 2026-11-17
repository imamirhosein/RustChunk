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

### 7. `.gitignore`
- **Source SHA:** `29d00d0368c76edd3115dbca57ce0901e55860a9`
- **Size:** 1 lines; full file fetched and read.
- **Coverage:** complete.
- **Purpose/content:** excludes build output and caches (`.local/`, `target/`, Python caches/venvs); downloaded/decompiled game material and private profiling files (`*.jar`, `*.class`, JFR/heap profiles, decompiled folders, Minecraft/assets/run/logs/dist); runtime-generated game data (options, debug profiles, servers.dat, touch layout, resource packs, mod data, saves/backups/screenshots/crash reports); and release exporter output.
- **Cross references:** aligns with README/CONTRIBUTING claims that game files and prepared resources stay local and must not be committed; ignores the `.local` resource-preparation cache and `exporters` release-only bundle.
- **RustChunk relevance:** high for protecting user-owned Minecraft files, mod jars/classes, saves and private diagnostic data from accidental commits. It also separates reproducible source from generated outputs and runtime state.
- **Risks / limitations:** `.gitignore` does not untrack files already committed and is not a complete public-tree/security enforcement mechanism. Broad rules such as `*.jar` and `/assets/` can also hide legitimate project files; RustChunk should define narrow, reviewed exceptions rather than copy these patterns without checking its own folder layout. A separate CI/pre-commit scan is needed for robust protection.
- **Decision:** **adaptable** for generated/private-data protection, with paths and ignore breadth rewritten for RustChunk.
- **Follow-up checks:** inspect `tools/check_public_tree.py`, hooks and CI to see how they complement ignore rules; examine actual Starling asset/prepared-resource directories and packaging behavior.
- **Completion state:** complete ignore-rule review; enforcement implementation remains pending.

### 8. `.github/workflows/android.yml`
- **Source SHA:** `3638a951d8489e040d9a37d22dab86526a6bf4e8`
- **Size:** 1 lines; full workflow fetched and read.
- **Coverage:** complete, including both APK and release jobs.
- **Purpose/content:** GitHub Actions pipeline that builds four Android ABIs, produces preview/release APK artifacts and native symbol artifacts, and publishes a tagged release after all matrix builds succeed.
- **Triggers / targets:** pushes to `android`, `starling`, or `integration`; pull requests opened/synchronized/reopened; manual dispatch with `keep_symbols`, `release_key`, and optional native-mod IDs. Matrix covers ARM64, ARMv7, x86 and x86_64 Android targets.
- **Build environment and sequence:** Ubuntu latest; checkout; Python 3.12; Temurin JDK 25; Android build-tools 35.0.0, platform 36 and NDK 28.2.13676358; stable Rust with target matrix and cargo-ndk 3.5.4; per-ABI Rust cache; release/debug keystore handling; build via `packaging/android/build_apk.py`. Preview uses `quick` profile and `--preview`; release uses `fast`. Release build adjusts swap/worker count for memory limits.
- **Signing/release controls:** release signing is conditioned on official repository plus tag or explicit release-key dispatch and uses a protected `release` environment/secrets; ordinary builds use a debug key or throwaway fallback. APK artifacts have different retention (release-key test builds only one day; ordinary builds 90 days). The release job downloads all ABI artifacts, runs `release_assets.py`, publishes APKs and StarlingVisuals with release notes, then triggers native-mod builds.
- **Native-mod path:** dispatch input IDs are validated against a constrained name regex and an expected mod manifest path; `native_build.sh` generates native-mod APKs. The main job has a separate symbol upload, with note that native-mod builds do not produce the ordinary symbols library.
- **Permissions / concurrency:** top-level contents read; release job elevates contents write and actions write; PR concurrency is cancellable while branch/tag builds are not cancelled by a new run in the same way.
- **Dependencies / cross-references:** `packaging/android/build_apk.py`, `packaging/android/native_build.sh`, `packaging/android/release_assets.py`, release notes, native-mod paths, diagnostics docs, and game update offer implementation. Workflow also relies on GitHub secrets, environments/rulesets, Actions artifacts, gh CLI and pinned action major tags.
- **RustChunk relevance:** very high as a CI/CD template: ABI matrix builds, separation of preview and release package identity/signatures, protected release secrets, symbol artifact retention, memory-aware build configuration, and post-matrix publishing are directly useful patterns for an Android Rust app.
- **Risks / limitations:** workflow results are not established by reading YAML; no run was executed or checked here. Several GitHub Actions are referenced by mutable major tags (e.g. `@v4`, `@v5`, `@v2`) rather than immutable commit SHAs, so supply-chain hardening could pin action commits. `ubuntu-latest` and stable Rust are moving targets; reproducibility depends on script/toolchain details. The documented four-ABI matrix may be overkill for RustChunk's specific devices and will increase build time. Do not copy release-secret assumptions without configuring environment protection.
- **Decision:** **adaptable** for matrix builds, preview/release separation, signing discipline, symbols, and release gating; **rewrite required** for RustChunk package IDs, supported ABIs, secrets, branch policy, toolchain versions, and Forge/JVM packaging.
- **Follow-up checks:** inspect build/release scripts and actual CI history; verify signing behavior and package IDs; inspect environment protection and tag rulesets; assess whether RustChunk needs all four ABIs or only ARM64/ARMv7.
- **Completion state:** complete workflow source review; execution and security settings not independently tested.

### 9. packaging/android/build_apk.py
- **Source SHA:** 3c215d5875d364e215ade3eeafbe55bd81238ab9
- **Size:** 381 lines; 17,919 characters; full file fetched and read.
- **Coverage:** lines 1–381, complete in one fetch (under the 400-line threshold).
- **Purpose/content:** main Python APK build/packaging orchestrator. It compiles the Rust Android NativeActivity library, builds small native Java/Javac launcher executables, prepares runtime assets, compiles Java Activities to DEX, links Android resources, assembles and aligns the APK, signs/verifies it, and optionally zips or installs it.
- **Important build configuration:** application ID io.github.juanluci.starling; release name/version 0.8.0/code 13; dev and preview use distinct application-ID suffixes and labels; min SDK 26, target SDK 35, compile platform 36, build-tools 35.0.0, NDK 28.2.13676358, Java major 25. Four ABI mappings cover ARM64, ARMv7, x86 and x86_64.
- **Build pipeline / implementation details:**
  - build_library() adds NDK/SDK environment variables, records Git commit/dirty state in build env when available, adds 16-KB-page and SHA-1 BuildId linker flags, then runs cargo ndk ... build -p horizon-android with selected profile and ABI.
  - build_launchers() compiles native/java_launcher.c twice (java and JAVAC variants) for the selected ABI and includes the matching NDK libc++_shared.so.
  - assets() runs tools/build_android_exporters.py; packages research pins, exporter source/test Java classes and built exporter classes into prepare-kit.zip; copies diagnostic fonts; deterministically zips the setup skin and Starling Visuals shader pack; also copies the visuals ZIP beside the APK for release publishing.
  - compile_java() compiles Android Java sources with javac --release 8, archives classes, and invokes D8 with min API 26.
  - link_resources() substitutes package/label placeholders into the manifest, changes a Vulkan feature declaration to optional for non-default ABI, creates icon/theme resources, extracts dark palette constants from a Rust source file with regex, compiles and links via aapt2. Debuggable mode is enabled when no release keystore env is present.
  - add_code() inserts DEX and native libraries under lib/<abi>/, compressed so Android extracts them at install.
  - sign() uses a supplied keystore or generates a local debug key, signs with apksigner and runs apksigner verification.
  - CLI options include profile, no-build, install, zip, keep-symbols, library-only, dev/preview, and ABI.
- **Dependencies / references:** Android SDK tools (aapt2, d8, zipalign, apksigner, adb); NDK clang/llvm-strip/C++ runtime; Rust/Cargo/cargo-ndk; JDK javac/keytool; AndroidManifest.xml, Java Activity sources, native/java_launcher.c, tools/build_android_exporters.py, research-lock.json, exporter Java source and bundle, fonts/theme crate, icons, lowbudget pack, shaderpack, diagnostics docs, and Git metadata. The script's referenced helper/source files require their own audit.
- **RustChunk relevance:** extremely high. It provides the concrete bridge between a Rust Android native library and a distributable Android app, including ABI mapping, SDK/NDK discovery, 16-KB page alignment, build-ID symbols, DEX integration, application variants, asset bundling, APK alignment and signing. Its approach demonstrates how a Rust game core can be hosted in Android's NativeActivity while Java handles Android system integration.
- **Major target mismatch:** this pipeline packages a native Horizon 26.3 implementation plus preparation Java tools; it does not bundle a Java Edition 1.20.1 client runtime or Forge. It cannot be copied as-is for the Forge requirement. RustChunk may instead need a Java runtime distribution/management layer, Forge installation/mod libraries, and an Android activity/JNI layer that launches the JVM, unless the project changes its compatibility goal.
- **Potential issues / items to verify:**
  - JDK/NDK/SDK versions and all package constants are Starling-specific; README/CONTRIBUTING also mention Rust 1.90 although Cargo.toml sets MSRV 1.85.
  - link_resources() performs a literal manifest string replacement for non-ARM64 Vulkan requirement; verify against AndroidManifest.xml and test each ABI.
  - assets() includes exporter inputs/bytecode but README says Minecraft jars/assets are not packaged; inspect generated contents and exporter bundle to confirm there is no player-owned game code/data leakage.
  - sign()'s release/debug behavior depends on environment variables and CI protection; no build/sign command was executed in this audit.
  - zip_folder() fixes timestamps and sorts files for repeatability, but archive reproducibility should still be tested across host platforms.
  - --dev and --preview are not declared mutually exclusive; current precedence chooses dev if both are supplied. --library-only stops before APK packaging as designed.
  - Script assumes all expected source/resource files exist; its missing SDK checks are explicit, but some later missing inputs surface as lower-level exceptions.
- **Decision:** **adaptable** as an APK orchestration pattern; **rewrite required** for RustChunk's Java/Forge/JVM packaging route and desired ABI/version support. Keep asset/legal boundaries and symbol/signing ideas.
- **Follow-up checks:** inspect native/java_launcher.c, Android manifest and Activities, exporter helper and manifests, install/update code, and release assets; verify all ABI outputs and exact Android package identity; map required runtime files for Forge 1.20.1.
- **Completion state:** complete script review; helper scripts, referenced sources and actual build results remain pending.

### 10. packaging/lowbudget/make.py
- **Source SHA:** f226619fcac0e0d1d61e75273d29e0f5772bc36d
- **Size:** 257 lines; 12,852 characters; full file fetched and read.
- **Coverage:** complete in one fetch.
- **Purpose/content:** generates Starling's temporary low-budget setup resource pack with Pillow. It supplies the app's setup/menu appearance before the user's own game files and resources are prepared.
- **Generated assets and mechanics:**
  - Chalk/graphite aesthetic with fixed palette, 4x texture scaling, multiple button takes, hand-drawn edges, grain, scribble fills, pressed/disabled variants, and nine-slice metadata.
  - Generates button, slider, slider-handle, text-field and scroller textures; tiled paper-like menu backgrounds; header/footer separators.
  - Generates five hand-lettered bitmap font sheets using hand.py, a default Minecraft font JSON plus four Starling font variants, and Starling variant metadata.
  - Imports local strings.json to create the en_us language file; writes a pack.mcmeta with pack_format 97 and Starling skin metadata.
  - Deletes and recreates the output pack directory on each run, then emits the output path.
- **Dependencies / references:** Python standard library (json/math/random/shutil/pathlib), Pillow image APIs, local hand.py and strings.json; output path packaging/lowbudget/pack. build_apk.py packages the generated folder as lowbudget.zip.
- **RustChunk relevance:** low for game performance/runtime, but useful as an example of generating a small bootstrapping UI/resource theme without shipping Minecraft assets. A separate early-start UI theme can help make setup usable before full game resources exist.
- **Version/compatibility warning:** generated filenames, namespace, language keys, font provider schema, pack metadata and resource-pack formats are tailored to Starling's 26.3 target. They cannot be assumed compatible with the 1.20.1 resource-pack format or Forge's pack-loading lifecycle without adapting and testing.
- **Potential issues / verification points:** this script was inspected but not run; the generated pack was not validated in-game. It relies on Pillow and imported hand.py behavior, neither of which is part of this file's implementation. Image-noise determinism should be tested rather than inferred from seeding Python's random module. Rebuilding destroys the previous generated output directory, as expected for a derived artifact but worth guarding if RustChunk's output path could contain user data.
- **Decision:** **unnecessary** for the core Rust/Forge engine; **adaptable** only if RustChunk needs a pre-game setup theme or generated placeholder UI pack. Rewrite pack schema and resource identifiers for Minecraft 1.20.1.
- **Follow-up checks:** inspect hand.py and strings.json if setup theme is in scope; inspect pack output and test its compatibility with Minecraft 1.20.1; verify build_apk.py's ZIP integration.
- **Completion state:** complete generator source review; output generation and runtime visual behavior untested.

### 11. research-lock.json
- **Source SHA:** 719c266401fe962746965fb33c07abbb91f8a656
- **Size:** 70 lines; complete JSON document fetched and read.
- **Coverage:** complete; no ranges omitted.
- **Purpose/content:** machine-readable pin set for the exact Minecraft research/preparation inputs and supporting analysis tools/runtime artifacts.
- **Pinned data:**
  - Minecraft version 26.3; primary and accepted republished version-manifest SHA-1 values; accepted asset-index SHA-1 values; note explaining that certain republished JSONs differ only in asset-index selection; client and server JAR SHA-1 values; Java major 25.
  - Vineflower 1.12.0 release URL and SHA-256.
  - rea-agents package version 6.4.0.
  - jadx-headless-mcp 0.7.1 release URL and SHA-256.
  - Mojang Java runtime manifests/checksums used by tools/datafix_worlds.py for old server jars/sample worlds: Linux legacy/alpha/beta/gamma and macOS ARM64 gamma, with stated fallback to gamma when a component is absent.
- **Dependencies / cross-references:** build_apk.py places this file in the phone-side preparation kit; README describes prepared resources checked against these pins; exporter/build tooling consumes it; datafix_worlds.py uses the listed runtimes for sample datafix worlds. Actual consumers and validation logic still need inspection.
- **RustChunk relevance:** very high as a pattern for separating mutable external metadata from source code and pinning downloaded inputs by cryptographic digest. RustChunk needs a comparable manifest for Minecraft 1.20.1, Forge artifacts, Java runtimes, native libraries and any tools used to construct a reproducible profile.
- **Version/compatibility warning:** every Minecraft checksum here is for 26.3, not 1.20.1; these values are unusable as RustChunk's source-of-truth for 1.20.1. The listed JDK major 25 is also Starling's exporter/preparation requirement, not evidence that Forge 1.20.1 should run on Java 25 (Forge/Minecraft 1.20.1 normally needs a compatible Java 17 runtime; verify exact Forge requirements before implementation).
- **Security / reliability notes:** checksums let consumers detect unexpected content changes only if the expected digest itself is obtained/trusted. SHA-1 remains in Mojang's lock fields and is weaker than SHA-256 for collision resistance; use SHA-256 where the upstream metadata provides it, and preserve the official manifest chain where it does not. Release URLs plus hashes are more robust than URLs alone. This document contains no actual game binaries/assets or license grants.
- **Decision:** **essential** as a concept for external-input pinning and deterministic preparation; **rewrite required** for RustChunk's 1.20.1/Forge artifacts, supported platforms and Java runtime requirements.
- **Follow-up checks:** inspect every reader of research-lock.json, verify how the hashes are checked and how republished manifests are handled; map Mojang metadata for 1.20.1 and Forge installer/profile dependencies; inspect runtime download verification and cache lifecycle.
- **Completion state:** complete lockfile-document review; verification consumers and exact 1.20.1 pins remain pending.

### 12. roadmap/starling.md
- **Source SHA:** a3742a8f994dd266355428b1621747f642d36e76
- **Size:** 188 lines; 12,976 characters; full file fetched and read.
- **Coverage:** complete in one fetch.
- **Purpose/content:** product roadmap with Now/Next/Later priorities and technical status notes, explicitly separating Starling Android work from Horizon's shared-game roadmap.
- **Capabilities described as done or substantially done:** phone-side download/preparation from Mojang's 26.3 files with fetched OpenJDK 25 and compiled exporters; keyboard/IME bridge; release update prompt; Bedrock touch controls fetched by pinned revision/hash; save-on-background and recovery/log handling; offline identity/skins; shader-pack translation and phone-specific shader work; plugin server dialogs, resource packs, custom armour/models/chat actions/movement modes; phone-aware settings.
- **Explicitly unfinished/next/later items:** shader-pack options screen, signed/secure chat for servers requiring it, Android physical-controller backend, Open-to-LAN/invites on Android, keeping up with Horizon, more game features exposed to mods, Horizon native mixins/rebuild on device, phone mod browser, and Android Discord Rich Presence.
- **Architecture / implementation clues:** native Horizon shader translator (GLSL → WGSL at load), background compilation/loading while the world renders without pack for a short time, experimental depth prepass, reduced render resolution, per-server movement profiles, Android keyboard text mirror, on-device fetched assets, update install through Android package installer, native mod rebuild tied to a particular Horizon revision.
- **Dependencies / references:** Horizon roadmap; account/resource preparation crate; packaging/lowbudget and shader pack; renderer/shader crates; bedrock_touch.rs and its manifest; NativeActivity/IME bridge; player and graphics config files; update_offer implementation; server/plugin protocol handlers; mod-porting doc and horizon-mixins. These implementation claims require code verification.
- **RustChunk relevance:** very high for a realistic feature inventory and separating platform-specific gaps from shared-core maturity. It identifies concrete mobile problems beyond simply compiling Rust: first-run resource provisioning, lifecycle/save behavior, IME, touch controls, GPU constraints, background pack compilation, signatures/updates, and plugin server behavior.
- **Critical documentation inconsistencies to investigate:** README lists physical controllers as 100% and says servers include secure chat with a signed-in account; this roadmap still lists Android physical controllers and secure chat as Next. README also describes native mods as 95%/compiled on-device while the roadmap places Horizon native mods under Later. Treat these statuses as unresolved until source and tests clarify scope or document dates/revisions.
- **Version/compatibility warning:** roadmap behavior is for Minecraft Java 26.3 through Horizon. Protocol packets, resource-pack schemas, chat signing rules, renderer assumptions, and mod formats differ across versions. The listed Horizon native mods are source mixins against a Horizon revision—not Forge Java mods and not binary compatibility with 1.20.1.
- **Decision:** **adaptable** for mobile feature sequencing and clear Done/Left records; **rewrite required** for the 1.20.1 feature plan and integration assumptions. The roadmap is a planning source, not implementation proof.
- **Follow-up checks:** compare roadmap claims with README and code; inspect the account, update, IME, touch, server protocol, shader pipeline, native-mod build flow and test reports; establish exact feature status for the RustChunk hardware and Forge 1.20.1 target.
- **Completion state:** complete roadmap-document review; documented statuses and claims are not yet source-verified.

### 13. docs/horizon-parity.md
- **Source SHA:** 4783c14f65966eb407b51f89cea5c847c9758bf7
- **Size:** 39 lines; 5,249 characters; full file fetched and read.
- **Coverage:** complete.
- **Purpose/content:** describes Horizon's own Java-vanilla parity audit as of the revision Starling uses. It explicitly says the audit is measured by desktop harnesses, not Android, and that 100% means only 100% of the behavior currently compared by those harnesses.
- **Audit details stated:** subsystem scores are 0/0.25/0.5/0.75/1, with 1 meaning complete and verified against vanilla by differential tests. The page says it was last audited 2026-10-13; 281 harnesses pass on Linux, and the Mac mini runs them too. Categories include input/game loop, assets/resources, protocol/multiplayer, chunk storage/palettes, meshing, rendering, lighting, world generation, saves/region files, entities/physics, interaction/inventory, UI/chat, audio, and block ticking/behaviors. Large scenario counts are documented for many subsystems.
- **Specific constraints / known gaps:** multiplayer parity evidence includes offline-mode only; the page states Horizon itself cannot yet sign in to Microsoft and therefore cannot join online-mode servers; HTTPS skins may show defaults unless cached; pack icons, creative hotbar C/X wiring, and item tooltip attribute lines are listed as remaining differences.
- **Dependencies / cross-references:** README Android status and Horizon README/project status; tests/test_parity.py; vanilla differential harnesses; game-specific packet and rendering/test sources, none of which are inspected in this file.
- **RustChunk relevance:** very high as a model for differential testing against the exact vanilla version. This is a strong idea for 1.20.1: compare packets, NBT/world data, recipes, inventory interactions, chunk generation, entities and UI outputs against vanilla/Forge fixtures rather than relying only on launch success or FPS.
- **Critical caveats:** 100% is a scoped test metric, not total parity and not Android validation. The test date and current status need rechecking. The parity target is the Horizon version based on Minecraft 26.3, not Minecraft 1.20.1. The page's Microsoft-auth statement conflicts with README's Starling Android sign-in claims; likely app-specific integration has moved beyond Horizon core, but that explanation is not proven here and must be checked in source. README's 100% server row also should not be read as online-mode parity.
- **Decision:** **essential** as a testing philosophy and differential-harness model; the actual 26.3 test suite is **rewrite required** for 1.20.1/Forge.
- **Follow-up checks:** inspect parity harness definitions, CI invocation and latest results; locate how Starling adds Microsoft sign-in around Horizon; inspect the implementation of the listed gaps; define RustChunk's version-pinned golden fixtures and offline/online test matrix.
- **Completion state:** complete parity-document review; harness claims and implementation not independently executed or validated.

### 14. docs/android-armv7.md
- **Source SHA:** 17cfb5fe88d6dceff4a5df1a73ba83f17ef06aa4
- **Size:** 111 lines; 6,621 characters; full file fetched and read.
- **Coverage:** complete.
- **Purpose/content:** documents ABI/build limitations, GPU fallbacks, ARM32 memory constraints, resource preparation, updater selection and native-mod ABI restrictions.
- **ABI/build findings:** ARM64, ARMv7, x86 and x86_64 are separate APK builds; release waits for all four. Android x86 has no supplied Android Java 25 runtime and therefore imports prepared .starling resources; x86/x86_64 are not physically tested. ARMv7 requires Android 8+ and uses a 32-bit launcher/runtime. The docs emphasize that CPU ABI support does not guarantee GPU-driver compatibility.
- **Graphics findings:** GLES-only shader/texture transformations are adopted from Horizon Android while Vulkan WGSL is stated unchanged; a BASE_VERTEX fallback rebases vertex buffer bindings; terrain textures are split into at most eight arrays and packs exceeding that are rejected; small GPU atlas limits use skyline packing while prioritizing GUI/fonts/dynamic slots; OpenGL panoramas use 256px box-filtered faces while Vulkan keeps full resolution. Block-sprite budget is 256 MiB by default, but does not cap total memory. ARM32 GPU arena growth is normally limited to 8 MiB steps.
- **Runtime/resource findings:** updater chooses an APK matching the process ABI; native mod binaries must match ABI and build/prebuilt native mods are ARM64-only. ARM32 preparation runs sequentially and caps each Java heap at 512 MiB, or 384 MiB below 1.5 GiB total RAM/when RAM cannot be read; the cap does not bound total process memory. APK includes compiled original exporter classes but not Minecraft jars/libraries/assets; host JDK 25 compiles and checks exporter classes, and pinned Minecraft inputs stay in ignored cache. ARM32 phone runtime lacks jdk.compiler, so it executes the shipped classes without javac. Android ARM32 runtime is pinned to EclipseJRE Java 25.0.4 after a reported floating-point issue in an earlier runtime.
- **Dependencies / references:** build_apk.py, native launcher, Rust targets/NDK, renderer backends and shaders, texture/atlas allocators, updater ABI selection, native-mod service, preparation worker lanes, research-lock.json, exporter bundle/cache, diagnostics and PR #272.
- **RustChunk relevance:** very high for choosing a supported ABI strategy, preventing bad-GPU assumptions, memory budgeting, renderer fallbacks, process-matched APK updates, and deciding where native mods may run. The texture/atlas/arena limits are particularly useful as concrete examples of device-aware rendering safeguards.
- **Compatibility / applicability:** much of the graphics code is forked from Horizon Android and targets Horizon's 26.3 render stack; it cannot be lifted into a 1.20.1 Forge client without matching renderer internals. ARM32 Java 25 exporter preparation is unrelated to the likely Java 17 requirement for Forge 1.20.1. ABI/build patterns are adaptable, not version-independent implementation.
- **Limitations / risks:** this is documentation; claims about rendering fallbacks, runtime FP correctness, heap limits, and byte-for-byte preparation output require source/test verification. Per-ABI release failure blocking is a deliberate quality/release policy but could slow RustChunk releases if unsupported ABIs are included.
- **Decision:** **adaptable** for ABI-aware packaging/update selection, low-memory controls, GPU capability checks and resource budgets; **rewrite required** for RustChunk's Forge/JVM preparation and renderer integration.
- **Follow-up checks:** inspect actual shader/backend fallback code, heap/worker limits, updater ABI selection, Java runtime checksums, native-mod ABI gating and build matrix; test on real Mali/Adreno devices and each ABI actually supported.
- **Completion state:** complete architecture-doc review; implementation and device tests not independently verified.

### 15. docs/android-controls.md
- **Source SHA:** bde1e376a8910d910a93566c492fca1413f5eaec
- **Size:** 79 lines; 4,554 characters; full file fetched and read.
- **Coverage:** complete in one fetch.
- **Purpose/content:** documents Starling's Android integration of Horizon Android's touch-control engine and visual layout editor, plus keyboard, pointer, physical input and IME behavior.
- **Control features:** MCPE and GoyControl layouts/presets; move/resize controls; independent dimensions, colors, borders, shapes, labels and event bindings; undo/redo, visibility and save confirmation; joystick/aim/sensitivity/gyro/pointer/button/inventory options. GoyControl supports static/pass-through/swipe/hold/toggle controls and up to three simultaneous events; MCPE has joystick/crosshair, joystick/tap and D-pad schemes.
- **Persistence and input details:** presets live under controls/, active choice in horizon-touch.json; physical D-pad/keypad profile supports rebinding and is persisted in horizon-keypad.json; Input device chooses Auto, Touch, Keyboard & Mouse or Controller, with Auto tracking the last-used device. Virtual Mouse switches between direct touch and relative pointer. Account selection is independent of control preset. Existing touch-layout.json is preserved but not automatically imported into the new preset format.
- **Touch/IME/lifecycle details:** 360-degree movement is optional/off by default; Look while interacting covers attack and use/place; vertical slider gestures scroll while horizontal drags adjust values; chat history/completion/cursor/paste and touch-slot gestures remain; foreground/focus changes release touch owners. Text fields use a shared focus/IME route; TextMirror adapters synchronize text/selection, while other widgets report focus bounds and use ordinary text events. Custom mod UIs must expose focus and handle text events; simply drawing a field does not make it functional. Focus reports expire per frame; explicit taps reopen keyboards hidden by Back. Server dialogs own Back, even when they disallow closing.
- **Asset integrity:** Bedrock button art is retrieved from a pinned manifest with size and SHA-256 checks and layered over prepared resource archives; legal provenance is separately described in THIRD_PARTY_NOTICES.md.
- **Dependencies / references:** controls/ preset data, horizon-touch.json, horizon-keypad.json, touch-layout.json legacy data, Bedrock resource manifest, TextMirror, preedit focus reporting, ordinary text event handlers, game input/focus release, and THIRD_PARTY_NOTICES.md.
- **RustChunk relevance:** high for mobile input design: editable controls, separate preset persistence, input-mode arbitration, touch ownership cleanup on lifecycle changes, explicit widget focus/IME contract, pointer mode, and cryptographically verified artwork. Particularly useful for reducing UI/input coupling and making custom screen text fields behave correctly.
- **Version/compatibility limitations:** the control behavior is adapted from Horizon Android and integrates with Horizon's renderer/game input. It is not an Android Forge input implementation and would need a bridge into Minecraft 1.20.1's input/event system. Bedrock resource assets and schemas are not automatically reusable for RustChunk.
- **Potential inconsistency:** this document describes Controller as an input-device option, while the roadmap still lists physical controllers on Android as Next and README reports controllers as complete. It may distinguish a UI option from a working Android backend; source inspection is required before claiming controller support.
- **Decision:** **adaptable** for input architecture, editable layouts, lifecycle handling and IME focus contracts; **rewrite required** for Forge/Minecraft input integration and version-specific UI hooks.
- **Follow-up checks:** inspect touch engine/editor/preset persistence, Android input event routing, IME/TextMirror/focus APIs, Bedrock touch manifest verification, and whether the Android controller backend actually works.
- **Completion state:** complete document review; underlying implementation and physical-device behavior remain unverified.

### 16. docs/android-accounts.md
- **Source SHA:** 253bcebee21e4962b4f317f6a6d1c61d5078e9b9
- **Size:** 76 lines; 4,481 characters; full file fetched and read.
- **Coverage:** complete in one fetch.
- **Purpose/content:** documents Android account/profile selection, Microsoft device-code authorization, Ely.by credentials/2FA, offline profiles, secure token storage, profile-specific skins and first-run account setup.
- **Provider semantics:** Microsoft device-code flow requires Java entitlement for authenticated play; Ely.by supports username/email, password and optional 2FA but authenticates only to Ely.by-configured servers and does not confer Microsoft ownership; offline names are 1–16 ASCII letters/digits/underscores and work only where offline players are allowed. Profile switching is persisted; failed or cancelled auth should not replace a later selection.
- **Persistence/security details:** provider/name/UUID/selection metadata in private accounts/accounts.json; refresh/access tokens in Android Keystore-backed sealed storage; passwords and 2FA codes are not saved. Legacy Microsoft profiles remain readable. Equal UUIDs across providers get separate secret keys/status caches. Desktop without platform keystore keeps secrets in memory for the run.
- **Skin behavior:** per-profile skins in horizon-skins.json; fallback legacy selection in horizon-player.txt; supports 64x64/legacy 64x32 PNGs; Microsoft upload posts the chosen skin to Minecraft profile endpoint and strips ancillary PNG chunks by rewriting the image; Ely.by skin is public API/read-only and opens their website for changes; offline skin stays local. Capes not supported.
- **Setup flow:** resource-independent UI allows Ely.by/offline/Microsoft selection and prepared-file import before full game assets are installed; authentication and profile selection precede continued resource download/preparation.
- **Dependencies / references:** Microsoft device-code OAuth and Minecraft entitlement APIs, Ely.by authentication/session/skin APIs, Android Keystore, private accounts directory, account secrets store, skin files and player config, installer/download path, resource-independent setup UI, THIRD_PARTY_NOTICES.md and Horizon Android provider model.
- **RustChunk relevance:** very high because account/profile semantics are central to a launcher and user has offline-profile requirements. Useful safeguards include provider-separated UUID/secret state, no persistence of passwords/2FA, protected token storage, cancellation race handling, profile-specific skins, and keeping offline play distinct from ownership/authenticated online play.
- **Compatibility / safety caveats:** this document describes Starling's custom integration. Forge 1.20.1 servers and Microsoft authentication have their own launcher/session requirements; Ely.by/offline identity should not be presented as Microsoft authentication or a way to bypass ownership. Microsoft entitlement and online-mode server behavior must be implemented and tested independently.
- **Decision:** **essential** as a set of account-security and profile-management requirements; **adaptable** as architecture, but **rewrite required** for RustChunk APIs, Java/Forge launch arguments, storage encryption and provider support.
- **Follow-up checks:** inspect account manager, device-code flow, Minecraft entitlement/session join, Ely.by auth/session, Keystore bridge, token rotation/revocation, account selection/cancel races, PNG normalization and setup flow; confirm what offline mode means for target servers.
- **Completion state:** complete account documentation review; implementation and security properties not yet independently verified.

### 17. docs/android-diagnostics.md
- **Source SHA:** f4d250d2449634f2f6ef8125ebdc2a6a4b021365
- **Size:** 97 lines; 5,780 characters; full file fetched and read.
- **Coverage:** complete in one fetch.
- **Purpose/content:** documents local performance/crash diagnostics, safe explicit export/share, startup recovery UI, Android exit history, symbol matching, bounded storage and transactional resource installation.
- **Performance recording:** opt-in one-second summaries of frame/CPU/GPU durations, phases, slow frames, worker stages, memory, thermal state, streaming queues, rendering counters and GPU adapter. Detailed GPU pass profiling reports p50/p95/p99/max from a rolling 15-sample window; timestamps/readback are asynchronous and unavailable data is not treated as zero.
- **Diagnostics export/privacy:** ZIP contains bounded log/crash tails, checkpoints, Android exit history and retained performance reports. Sharing grants access only to the selected ZIP; no automatic upload. Account files/worlds/config are excluded and credential/address patterns are redacted, but users must review archives because arbitrary mod/plugin log text cannot be fully scrubbed.
- **Crash/recovery:** startup checkpoint tracks launch/menu/world/in-world phases; missing clean exit is not proof of crash. On Android 11+, ApplicationExitInfo can classify exits; up to eight system traces of 1 MiB each are stored privately and only bounded crashed-thread summaries enter ZIPs. A separate renderer-independent diagnostics Activity can open before game resources/library; native export loads library lazily and degrades gracefully if it cannot load.
- **Native symbol tracing:** BuildId embedded in native game library; CI stores unstripped per-ABI library artifacts for 90 days; symbols can be matched to tombstone pc addresses using llvm-readelf/addr2line. Release libraries without line tables resolve functions but not source lines.
- **Resource bounds:** performance JSONL <=8 MiB/30 minutes, three retained reports, writer queue eight samples; diagnostic ZIP <=4 MiB/32 files/256 KiB per ordinary input, three ZIPs retained; export retains complete header/trailing complete rows and marks truncation.
- **Resource installation safety:** exclusive kernel lock serializes preparation; installer validates prepared schema/tables/assets, keeps prior resource folder until publication and restores on failure; retry resets loader generation. Documentation says resource preparation uses on-device Java exporters rather than old pre-generated tables; download concurrency does not eliminate exporter cost.
- **Dependencies / references:** logger/panic reporter, performance recorder and GPU timers, Android ApplicationExitInfo, tombstone parser, BuildId link flags, CI symbol artifacts, diagnostics Activity/launcher shortcut, resource preparation lock, installer publication/rollback and tests in tools/test_android_diagnostics.py.
- **RustChunk relevance:** extremely high for a mobile client expected to operate under constrained memory/GPU conditions and with many mods. Bounded local logs, opt-in sampling, renderer-independent recovery, symbol artifacts, explicit share permissions, privacy controls and transactional resource installation are practical reliability features. The p50/p95/p99 stage measurements can help distinguish CPU, GPU, streaming and memory bottlenecks.
- **Risks / limitations:** diagnostics can themselves consume resources; GPU profiling overhead is explicitly noted. Redaction is not guaranteed for arbitrary third-party logs. The described host test command and device build were not run here; this is documentation review only. Some diagnostic implementation is Rust-specific and will need an Android/JVM/Forge bridge.
- **Decision:** **essential** as reliability, observability and privacy requirements; **adaptable** for data model, retention limits and recovery workflow; **rewrite required** for JVM/Forge crash capture and integration.
- **Follow-up checks:** inspect test_android_diagnostics.py and production writers/exporter/tombstone parser; inspect BuildId and CI artifact retention; inspect resource lock, validation and rollback; verify Android 11+ behavior and resource limits on low-RAM devices.
- **Completion state:** complete diagnostics-document review; implementation and tests not independently verified.

### 18. docs/native-mods.md
- Source SHA: 8236ea99847bdfb5d4f64679565e8d96e7527440
- Size: 3,049 lines; 328,314 characters; full-file verification confirms the same SHA and final newline.
- Coverage: all lines 1–3,049 read without gaps. Ranges were fetched consecutively: 1–400; 401–800; 801–850; 851–900; 901–950; 951–1,000; 1,001–1,050; 1,051–1,100; 1,101–1,150; 1,151–1,200; 1,201–1,250; 1,251–1,300; 1,301–1,350; 1,351–1,400; 1,401–1,450; 1,451–1,500; 1,501–1,550; 1,551–1,600; 1,601–1,650; 1,651–1,700; 1,701–1,750; 1,751–1,800; 1,801–1,850; 1,851–1,900; 1,901–1,950; 1,951–2,000; 2,001–2,050; 2,051–2,100; 2,101–2,150; 2,151–2,200; 2,201–2,250; 2,251–2,300; 2,301–2,350; 2,351–2,400; 2,401–2,450; 2,451–2,500; 2,501–2,550; 2,551–2,600; 2,601–2,650; 2,651–2,700; 2,701–2,750; 2,751–2,800; 2,801–2,850; 2,851–2,900; 2,901–2,950; 2,951–3,000; 3,001–3,049. Long tool responses were re-fetched in smaller windows where initial output had been truncated; the full-file verification confirms the current source SHA and exact line count.
- Purpose: extensive technical manual for Starling/Horizon native Rust mods: supported packaging formats, manifest compatibility, Rust mixin injection, generated hook API, trait-based API, build/install lifecycle, trust model, dedicated-server output, mod networking, data/content integration, rendering hooks, gameplay events and server helpers.
- Mod formats and compatibility:
  - Code bundles use .hnmod with mixins/<id>/ Rust/TOML and optional data/crate files; their metadata pins an exact Horizon source revision and/or a named hook API contract. Content data uses .hmod; a combined .hmod can contain both content and native code under two manifests with the same mod id. A native bundle is not a WebAssembly mod; a bundle with native code plus mod.wasm is refused. GitHub link installs resolve to an exact release or commit, retain origin metadata and use SHA-256 for release assets.
  - The bundle has strict file-count/unpacked-size limits (512 files and 64 MiB for registry bundles; local folder builds do not have those limits). Manifests and paths are validated before compilation. Dependencies declared by native crates must be present at a matching version in Horizon's Cargo.lock; the lockfile is treated as an allowlist and builds use --locked.
  - Hook catalog API is versioned (documented through API 359 in this file, with some pending/draft entries and explicit gaps). Stable named hook contracts can permit compatibility by hook API version; arbitrary raw Rust target injections pin the exact Horizon revision because they can break at any source change.
- Implementation model:
  - The launcher compiles native code into a generated Horizon executable, not as a plugin loaded into a JVM. horizon-mixins stages modifications against the pinned Horizon source; it applies source injections and builds the game. Trait mods link into that executable, expose Hooks implementations, share a process-wide instance across the window and server threads, and use Host/LevelHost interfaces and typed events.
  - The documented hooks cover server and client tick paths; input, GUI, screens, keyboard/IME, player actions, authentication-adjacent profile/game events, entities, blocks, fluids, containers, recipes, loot, advancements, commands, world lifecycle, resource reload, particles, sounds, camera, shaders, render passes, and dedicated-server helpers. This is a broad reimplementation of many vanilla/NeoForge-like extension points inside Horizon, not actual NeoForge/FML event buses.
  - Data/content integration includes conditional datapack files, recipe filters, mod-defined ingredient/effect/loot kinds, menu types, item/block declarations, block entities, fluids, energy, attachments, advancement triggers, mod messages, and post-build programs. Data may be compiled into the game or materialized after build into the version data directory; attachment persistence/sync has explicit holder and codec rules.
- Build/install workflow and operational cost:
  - The launcher downloads its own pinned Rust toolchain (the doc names Rust 1.99.1 and SHA-256-pinned components), separate Cargo home, a pinned horizon-mixins revision, and a source archive of the exact Horizon revision. It does not use the player's system Rust; it strips hostile environment overrides and handles host-specific compiler/dependency requirements.
  - Mixin staging tracks changed files and Cargo timestamps so incremental builds recompile only touched crates. Cache identity includes source revision, mixins revision, Rust version, profile/features and bundle contents. Performance/dev/release profiles trade compilation speed against runtime optimization. The docs report measured desktop builds from several minutes and multi-GB target directories; those are dated desktop measurements, not Android performance or RustChunk measurements.
  - It installs a separate native-* version folder tied to its Horizon base/revision and mod set, preserving the original instance and worlds. Builds are serialized; post-build steps run with a defined environment; local build logs are retained and are not automatically uploaded. A separate dedicated-server export builds an engine binary for the host OS/architecture.
- Security/trust:
  - Native code is explicitly unsandboxed and has full access to the player's computer. Static bundle validation, checksum verification, registry review and source pinning do not sandbox malicious code.
  - The player must explicitly accept --trust-native-code after a warning and should personally read the source; the guide explicitly says an agent must not provide that consent. This is a critical security design principle for RustChunk if it ever executes arbitrary native extensions.
  - Registry source review and package checks help provenance but do not remove runtime trust. Build-time post-build executables are also native programs and must be treated as trusted code. Mod messages from clients are described as untrusted player input and require validation.
- Dependencies / cross-references: horizon-mixins, Horizon Cargo.lock, horizon-client / horizon-client-render / horizon-world, native-hooks.json and tools/native-hook-fragments, packaging/manifests and registry index, mod-api generator/xtask, Host and LevelHost implementations, renderer internals, launcher Rust toolchain/download/cache/build modules, build logs, content docs, data formats, SMP server, THIRD_PARTY_NOTICES.md, and the referenced security notes. The document names these concrete implementation surfaces; those sources and their actual behavior still need separate inspection.
- RustChunk relevance: high as a study of extension-system design, especially version-pinned hooks, stable contracts vs raw injections, Rust trait API dispatch, build cache keys, reproducible toolchain acquisition, mod provenance, explicit consent, content/code separation, server/client side boundaries, and a potentially extensive gameplay API. Individual mobile features (input/render hooks, resource reload, bounded APIs, lifecycle and diagnostics) are useful design references.
- Critical incompatibility with RustChunk's goal: Starling's native mods are compiled Rust code injected into a Rust rewrite of Minecraft 26.3. They are not .jar files, Java bytecode mods, Forge mods, FML mods, or a JVM compatibility layer. Their NeoForge-like hook semantics are deliberately implemented by Horizon and do not make a Forge 1.20.1 mod loadable. Adopting this native-mod framework would change the product's mod ecosystem and would not satisfy the stated requirement to run the user's existing Forge 1.20.1 mods or connect to servers with those mods. A Rust launcher can still use selected design ideas while retaining a Java 17 + Minecraft 1.20.1 + Forge runtime.
- Further risks / limits:
  - The build process described here targets desktop host toolchains; the documented workflow does not establish on-device Android compilation or compatibility with Galaxy A22/Mali-G52 resource constraints. It is resource-intensive (large source archive, Rust toolchain, many Cargo crates and multi-GB build cache).
  - Rust mod code and post-build tools execute unsandboxed; a package signature/checksum alone only proves identity/integrity, not safety.
  - API catalog entries have caveats and missing paths; many hooks are expensive per tick/frame and documentation explicitly says to keep handlers cheap. Some APIs are pending/draft and some game behavior is not fully parity-tested.
  - A single Hooks object serves both window and server-thread games and hooks may be reentrant; shared state requires synchronization. The guide also documents data-load hooks on arbitrary loading threads. This is a concurrency requirement, not a guarantee that careless mods are safe.
  - A mod may be client-only when joining another server; server-side hooks require the host/server to run the mod. Dedicated server exports are separate host builds and have no compatibility handshake yet (per doc).
- Decision: **unnecessary** as a direct mod runtime for RustChunk's Forge target; **adaptable** for extension API governance, versioned hook contracts, build reproducibility/cache design, code/content separation, opt-in trust and diagnostics practices. If the goal changes to a fully native Rust Minecraft fork with its own non-Forge mod ecosystem, this document becomes a major architectural reference, but that is a different product target.
- Follow-up checks: inspect native-hooks.json and hook generator; inspect horizon-mixins security notes and manifest validator; trace Rust toolchain/source downloads and hash verification; inspect staging/cache/build scripts and native post-build runner; compare docs with real code and tests; verify which features are Android-specific vs desktop; explicitly decide whether RustChunk keeps the Forge/JVM runtime (the current target) or abandons Forge before borrowing any core architecture.
- Completion state: complete document review and line coverage verified; implementation claims, security behavior and build measurements are not independently tested in this audit.

### 19. docs/starling-native-mods.md
- Source SHA: d84abf4d4497bea2c4ef343287825256bce16477
- Size: 121 lines; 9,246 characters; full file fetched and read in one request; final newline present.
- Coverage: lines 1–121, complete.
- Purpose: documents how Starling enables Horizon native Rust mods on Android despite not compiling the Rust game on the phone. It describes remote compilation, verified library download, runtime library substitution, Android permission handling, service-side build isolation, GitHub runner mode, and alternate build-server trust.
- Android build lifecycle:
  - User imports .hnmod into horizon-native-mods. The Mods screen tracks the current APK/native build, enabled mod set and next launch selection.
  - The app sends enabled mods' source/manifests to a build service; content assets/data that only the game consumes are omitted. User can continue playing while the remote job builds.
  - The service returns a native game library; the client verifies its SHA-256 and stores it at files/native/current. Interrupted downloads resume.
  - At next startup StarlingActivity loads the replacement library rather than the APK's default game library; NativeActivity still loads the APK library entry point, which finds the replacement already loaded via STARLING_GAME_LIBRARY. The activity and game share one library, so Java native methods bind to the replacement.
  - A stamp beside the replacement records APK lastUpdateTime. A changed/reinstalled APK invalidates the native build; a build that fails twice before first frame is sidelined and the APK's own game is used. "Play without native mods" returns to the bundled library. This is a safety/rollback pattern, not proof of all crash cases being handled.
- Server build implementation described:
  - native_build.sh compiles horizon-mixins at a pinned commit, downloads the exact source revision and copies mod bundles, adjusts Android module identity to avoid ambiguous module paths, applies mixins into a persistent stage, then invokes build_apk.py with fast profile. Output includes stripped library, optionally APK, and build.json.
  - Permission scanner native_mod_permissions.py examines each mod's source and records permissions in build.json. Current documented capability is microphone; Android asks the player through the standard permission dialog after first frame, and "Not now" is remembered for that build. The mod source is not rewritten.
  - mod_build_server.py serializes one job at a time, caches matching version+mod-set results, and has a gateway. The gateway wakes a sleeping build machine, applies per-address/global daily quotas and failure accounting, and returns completed results.
  - The build sandbox is described as having no network, no access to host files or keys, no privilege escalation, and a disposable overlay/cache so one build cannot alter a later player's build. Mod build code executes (build scripts/macros), making this isolation boundary security-critical.
  - An optional GitHub Actions route runs the same validation and sandbox on public runners; the gateway uses a narrowly scoped token to trigger only the public mod-builds repository workflow. Runner job token has no permissions; result artifact is SHA-256 checked. Logs and run/artifact metadata are public temporarily (one day).
  - Build-server address is read from a protected Starling branch file before compile, with last-known/built-in fallback; a player may type an alternate HTTPS server manually. The alternate server receives mod sources/manifests and returns a library that executes as the game with access to worlds and signed-in account.
- Security assessment:
  - SHA-256 download verification detects corruption/change relative to the service-provided digest; it does not establish who built the library or whether it is safe. The documentation itself says an alternate build server's returned library cannot be checked by the app; its warning is only a warning.
  - Native code has the game's full access on the phone. The only authorization boundary for an untrusted native mod/library is the user's informed consent plus trust in the service and its build controls. Sandboxing build scripts does not sandbox the resulting native library at runtime.
  - The gateway/build service is a software supply-chain component: server address distribution, source transfer, build cache isolation, job quotas, GitHub token scope, artifact provenance and the service binary's trust all matter. Public logs/artifacts can expose mod source/build information.
  - The source scanner for Android permissions is a policy mechanism whose completeness must be validated; native code can potentially access platform APIs through other mechanisms unless constrained by the binary/build model. Do not assume static permission scanning is a sandbox.
- Dependencies / cross-references: import.rs; game/native_builds.rs; game/setup/fishing.rs; game/build_mining.rs; NativeMods.java; StarlingActivity; AndroidManifest.xml; android_lib.rs; build_apk.py; native_build.sh; native_mod_permissions.py; mod_build_server.py; mod_build_gateway.py; mod_build_once.py; mod_build_sandbox.sh; build.yml; base.yml; versions.json; protected Starling branch; and SHA-256 artifact metadata. Their implementation, signing, sandbox enforcement, and update/fallback behavior still require source-level verification.
- RustChunk relevance: very high to the Galaxy A22 / limited local build-resource constraint. Remote compilation could avoid requiring Rust+NDK+multi-GB Cargo cache on the phone, permit asynchronous progress while playing, and deliver ABI-specific libraries. The APK-version stamp, hash-checked download, resumable transfer, cached builds, user-visible compile state, and rollback to bundled binary are reusable reliability ideas.
- Critical limitation for RustChunk: this system remotely compiles Starling's Rust mixins into a Rust Horizon 26.3 engine. It does not compile, package or load Java Forge 1.20.1 mods, does not provide a JVM, and cannot substitute for Forge's Java mod loader. A RustChunk remote builder would still need to prepare the correct Forge/JVM runtime and native dependencies if RustChunk keeps its Forge target.
- Decision: **adaptable** for remote build delivery, APK-bound cache invalidation, resumable verified downloads, rollback and explicit service trust; **rewrite required** for any Android JVM/Forge pipeline; **unnecessary** to reproduce the Horizon native-mod runtime unless RustChunk abandons Forge and becomes a native Rust game fork.
- Follow-up checks: inspect native_build.sh, the three Python service/gateway/sandbox scripts, native_mod_permissions.py, NativeMods.java and StarlingActivity; confirm runtime library loading order and JNI binding behavior; test failure/rollback and APK stamp logic; verify sandbox OS controls and clean-cache isolation; examine source/data filtering and transport privacy; verify SHA-256 provenance and gateway address trust chain.
- Completion state: complete document review; no service, Android build, sandbox or native library was executed or security-tested in this audit.

### 20. docs/mods.md
- Source SHA: 6b6c04640da7292c3a4a60217347d5acd487ebd1
- Size: 493 lines; 30,867 characters; full file fetched and read in one request; final newline present.
- Coverage: lines 1–493, complete.
- Purpose: describes Horizon's mod model, supported mod types, enabling/loading, content mod authoring and schema, multiplayer compatibility checks, native code mods, registry publishing and artifact integrity.
- Two distinct mod systems:
  - Content mods (.hmod) carry data/assets only: blocks, items, recipes, loot, tags, models, textures, structures and supported registry records. They are loaded at game start and contain no executable code.
  - Native mods (.hnmod) are Rust/horizon-mixins bundles that alter Horizon source and are built into a per-mod-set executable; see native-mods.md.
  - Fabric and NeoForge JARs do not run as JARs: their Java code calls or patches Minecraft Java classes which Horizon does not have. Only suitable data/assets inside those archives can be carried/ported. This is explicit and directly relevant to RustChunk's Forge requirement.
  - Mods are off by default; content changes require restart; two enabled content mods cannot own the same namespace; benchmarks exclude mods.
- Content-mod authoring and schemas:
  - cargo-horizon-mod scaffolds, builds, runs and checks content .hmod archives. Build validates manifest/content and rejects native Cargo project code in a content bundle. Registry checks include archive size/hash, manifest/content and optional minisign signature.
  - Manifest requires id/name/version/description/authors, API version, side, capability and optional namespaces. Unknown keys are errors. Content uses registries capability.
  - Blocks and items are declared through data/<id>/horizon/block and item JSON using vanilla templates. The schema includes declared state properties/defaults; deterministic state ordering; collision/outline shape boxes; block random-tick overrides; signs and connected pipe-like blocks; item data-component patches; crafting remainders; creative tabs; mob effects; entity types currently only based on snowball; and menu types/data/result slots.
  - Native code can register custom component types used by content item JSON; unsupported component ids and malformed fields fail rather than silently ignore. A copied vanilla item does not automatically inherit behavior tied to the vanilla item id (the document gives trident behavior as an example).
  - Content file namespace policy permits the mod's own id and explicitly listed namespaces, plus carefully scoped vanilla/convention tags; namespace conflicts leave the later mod out. Limits include up to 65,536 files, 16 MiB per file and 512 MiB total. Unknown tag members are dropped while preserving the rest.
  - Content data merging supports tags, add_to contributions for structure sets/template pools/processor lists, biome feature modifiers including NeoForge and older Forge biome-modifier paths, selected Lithostitched pool element types and enhanced terrain adaptation. Many kinds are explicitly not supported (e.g. add_spawns, add_carvers, mod-owned serializers and some biome modifier forms); unknown/malformed data has different skip/fail outcomes.
- Multiplayer/content synchronization:
  - Only side=both content mods are sent in the configuration phase and compared by id, version and SHA-256; mismatches/missing mods reject the client with named reasons. side=client mods stay private to the client; side=server stay private to server. A manifest with no side defaults to client, so cross-side content must explicitly declare both. Dev folder mods lack a file hash and compare by version.
  - Native mods are not included in the content mod list and the manifest lacks side; this is a distinct protocol and compatibility area.
- Registry and supply-chain model:
  - Initial project listing is human-reviewed against repository/source and license; later releases are not human-reviewed, only checked by automation (manifest, id/version, integrity, optional signature).
  - Registry computes/publishes SHA-256 and launcher refuses altered bytes. Authors can optionally sign releases with minisign/Ed25519; public key is pinned in registry and key changes are rejected. Revoked assets are disabled/refused. Unsigned local files remain allowed with an unsigned marker.
  - The registry's docs explicitly state that project/source review is once at listing, not repeated for each release. Thus hash/signature proves correspondence to an indexed/signed file, not that the code is benign or a new release was manually audited.
- Dependencies / cross-references: native-mods.md, modpacks.md, modpack authoring, mod-browser.md, api/mod-index.md, horizon_protocol::mod_list, mod-content loader and registry parser, cargo-horizon-mod CLI, horizon-rust/horizon-mods index and minisign verification. Their code and tests remain outside this file audit.
- RustChunk relevance:
  - Very high for a compatibility boundary. The project documentation explicitly states that Java Fabric/NeoForge JAR code cannot run in Horizon; only selected data/assets can be imported. For RustChunk targeting 1.20.1 Forge, this means Starling's mod ecosystem is not a route to load the existing ~70 Forge JARs.
  - Potentially useful data-porting ideas: asset/data extraction from JARs, namespace handling, content validation, registry-based content model, resource pack/loot/recipe/tag/structure/biome data migration, client/server mod-side separation, server content hashes and clear mismatch errors.
  - Its registry and mod trust model offers useful design patterns: explicit opt-in, SHA-256, pinned signing keys, revoke lists, but it also makes clear that verification is not a sandbox or repeated human security review.
- Version and implementation caveats:
  - Schemas and data component names are for Horizon/Minecraft 26.3 and its custom loader, not automatically valid Forge 1.20.1 resource/data formats. Data ports need version-aware conversion and testing.
  - Content-only data cannot reproduce Java code behavior, Forge event subscribers, mixins, capabilities, custom networking, custom entities with code AI, or other logic simply because assets/JSON are copied. Native Rust mods cannot fill the Java Forge gap.
  - Some advertised entity/menu/block features remain explicitly incomplete; e.g. entity types are only snowball-template projectiles; menu slot behavior and some lifecycle hooks are still marked not yet.
- Decision: **unnecessary** as RustChunk's runtime mod loader; **adaptable** for a separate data-only import/porting layer and integrity/compatibility checks. A true Forge 1.20.1 target requires retaining/launching a compatible Java+Forge runtime or explicitly changing the product goal; it cannot be achieved by copying Horizon's .hmod/.hnmod system.
- Follow-up checks: inspect mod-content parsing/validation, archive format and signatures; compare the listed supported data forms against real Forge 1.20.1 mod JARs; assess how much of the current Forge pack is data/assets vs executable behavior; inspect protocol mod-list handling and client/server side rules; create a version-aware importer only after the Forge runtime strategy is fixed.
- Completion state: complete document review; no JAR import, registry validation, multiplayer handshake or Forge compatibility test was run in this audit.


### 21. `docs/version-ports.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `31c77d4d63bb45c648b8ec24134ad6141dd2628c`.
- Size: 33,526 bytes; 435 lines; complete file fetched in consecutive ranges 1–400 and 401–435.
- Coverage: lines 1–435, complete; both ranges returned the same blob SHA.
- Purpose: describes Horizon's automated process for moving its Rust implementation from one Minecraft Java release to the next: pinning, official-jar preparation, data generation, class indexing/diff, regeneration, parity harnesses, issue/report generation and release switch.
- Current-version assumptions and pinned facts:
  - The documented baseline is Minecraft 26.3: world/data version 5023, protocol 777, resource pack 97.1, data pack 121.0, Java 25; generated constants and protocol tables derive from that pin.
  - The pipeline assumes modern releases since 26.1 are unobfuscated. It explicitly stops when a version JSON contains client mappings; a 1.20.1-era target therefore cannot be assumed to pass its current preparation/decompilation route unchanged.
  - The demonstrated port is only 26.2 ↔ 26.3. Its replay claims that modern adjacent-version switch can be regenerated and parity-checked; it does not establish a long-range port to 1.20.1.
- Pipeline dependencies and mechanics:
  - `research-lock.json` pins version JSON and client/server JAR hashes, Java major and decompiler/tool versions. The player's official launcher installation is the source of the game files used by `horizon-prepare`; vanilla JARs and decompiled sources are development inputs and are not shipped.
  - Server bundler extraction, Mojang SHA-1 checks, server entry SHA-256 checks, `net.minecraft.data.Main --reports`, Java exporter programs, and class inventories are prerequisites for generated resources and facts.
  - Generated version facts, packet IDs, entity tracking, command ordering and selected DataFixer tables are version-pinned outputs. Other game data/assets are exported from the player's own version at preparation time.
  - The diff includes protocol packets, registries, block states, item components, commands, data packs, assets and changed class/method signatures. A vanilla-class-to-Rust-module map uses Java oracle imports, parity harness scopes and Rust doc comments.
  - Regeneration builds a candidate branch and re-runs generators; parity harnesses distinguish pass, oracle compilation breakage and actual behavior mismatch. Rank-1 items include protocol/save-format blockers and broken generators/exporters. The documented switch requires no rank-1/2 work, all relevant harnesses passing, and a playtest.
  - The pipeline stores downloads/decompilation locally, retains a small rolling version window and forbids public release of decompiled code. It expects Java, Cargo, tools, local vanilla data and a Mac mini; the scheduled daily agent is described as written but not installed.
- Important target-1.20.1 implications:
  - This is the strongest documented blueprint found so far for a disciplined version port, not a ready-made 1.20.1 port. Its modern release assumptions, version lock, JDK, unobfuscated bytecode indexing, generated schemas and parity oracle must be independently checked against 1.20.1.
  - A 26.3 → 1.20.1 change is not an adjacent-version update: packet IDs/states, registry contents and numeric IDs, save/DataFixer history, data/resource-pack formats, block/item components, entity metadata and Java APIs may differ across many intervening versions. The documented 26.2 → 26.3 diff does not cover that cumulative gap.
  - The pipeline currently supports one Minecraft version per build. Supporting 1.20.1 requires selecting/pinning the actual 1.20.1 client/server assets, using its correct Java baseline (to be verified from its version metadata), handling its mapping/decompilation regime, and building a 1.20.1-specific set of generated outputs/oracles and compatibility tests.
  - The player's official launcher copy and license restrictions are central; the document does not authorize bundling Mojang game code/assets into RustChunk. Legal asset acquisition and local preparation need their own design review.
  - Its principle—diff vanilla data/code, map changes to Rust modules, regenerate data-driven outputs, and prove behavior against version-matched vanilla oracles—is highly reusable. The existing tools themselves may need substantial changes or replacement for a historical 1.20.1 target.
- Dependencies / cross-references: `research-lock.json`; `tools/version_port.py`; `tools/lab.py`; `tools/gen_version_consts.py`; `tools/generate_packet_ids.py`; `tools/gen_block_state_data.py`; `tools/class_index.py`; `tools/vanilla_map.py`; `tools/port_regen.py`; `tools/port_harness.py`; `tools/port_report.py`; `tools/parity.py`; `tools/ci/vanilla-data.sh`; `crates/horizon-core/src/version.rs`; `crates/horizon-protocol/src/ids.rs`; `crates/horizon-world/src/datafix/`; Java parity oracles; `horizon-prepare`; docs/release-flow.md.
- RustChunk relevance: **critical**. This document describes the workflow RustChunk needs conceptually to turn a 26.3-based Rust game implementation into a version-accurate 1.20.1 implementation, and it exposes the key incompatibility in the current automated route: its assumption of modern unobfuscated releases. It supports a data-driven port plan rather than a simple version-string replacement.
- Decision: **adaptable** as the porting methodology (version pinning, provenance/hash verification, generated constants/tables, data diffs, oracle mapping, parity gates and ranked blockers); **rewrite/substantial adaptation required** for the current tooling and historical 1.20.1 input. Do not treat the 26.2→26.3 results as evidence of 1.20.1 compatibility.
- Follow-up checks: inspect `horizon-prepare` and its Java invocation, JAR extraction, asset export and data generation; inspect the generated version-constant and packet-ID pipeline; inspect `horizon-protocol`'s version adapter and packet tables; inspect datafix/world-data loading; inspect the code's pinned Java version and build toolchain; then independently verify 1.20.1's official version metadata, mappings, protocol, data version and formats before designing the port.
- Completion state: complete document review; no version-port script, download, decompilation, generator, parity harness, candidate build or release switch was executed.


### 22. `docs/versioning.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `261f462e35f6797c97b4108e8572734ecdf4326a`.
- Size: 12,096 bytes; 149 lines; fetched in one complete range (1–400 returned all 149 lines).
- Coverage: lines 1–149, complete.
- Purpose: documents Horizon application release numbering and migration from dated game tags to SemVer, including separate game/launcher release streams, nightly ranking, update bridges for old installs, and irreversible release-channel transitions.
- Important distinction: this file versions Horizon builds and launcher releases; it does not specify the Minecraft Java version embedded in the game build. The Minecraft game version is pinned independently (e.g. by `research-lock.json`, covered in `docs/version-ports.md`).
- Mechanics and constraints:
  - Game and launcher use distinct SemVer tags/streams; the game since 2026-10-28 ships Nightlies only, while launcher tags remain on a SemVer line.
  - Release workflows inject build versions through environment variables and check executable-reported versions; Cargo package versions are not necessarily the final release version.
  - Legacy installs require bridge releases because old clients compare tag strings/numbers with incompatible ordering rules. Some migration steps and tags are explicitly irreversible.
  - Release sequencing depends on release-train automation, stable/latest promotion rules, old launcher/server updater behavior and external fleet/Kira consumers.
- RustChunk relevance: low for the current Minecraft 1.20.1 engine port. It is useful only if RustChunk later adopts a similar separate SemVer policy for its launcher/application releases and needs safe upgrades from old launcher versions. It must not be mistaken for a mechanism to select, download or run Minecraft 1.20.1.
- Decision: **unnecessary** for the 1.20.1 game-engine port; **adaptable** as an optional launcher/application release-management pattern. No need to reproduce Horizon's historical bridge tags or its release workflow.
- Dependencies / cross-references: `.github/release-line.txt`; `.github/workflows/release.yml`; `.github/workflows/launcher.yml`; `tools/release_train.py`; `crates/horizon-server/src/update.rs`; `launcher/src/backend.rs`; `release-flow.md`; website release ordering and external updater consumers.
- Follow-up checks: none required for the Minecraft 1.20.1 compatibility investigation; revisit only when RustChunk's own update and release policy is designed.
- Completion state: complete document review; no release, tag, workflow, update or migration command was executed.


### 23. `docs/world-data.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `59b6049b77df9e7c1faa1c545a6707cb3d234446`.
- Size: 9,261 bytes; 147 lines; fetched in one complete range.
- Coverage: lines 1–147, complete.
- Purpose: describes the implemented portion of Horizon's Rust world/save layer: region files, NBT, chunk palettes, block-state registry identities, saved tick lists, DataFixer upgrades and a small fluid-state adapter.
- Hard version boundary:
  - The document explicitly pins implementation to Minecraft Java 26.3 and DataVersion 5023. It states chunk access requires 5023 and registry data is exported from the pinned vanilla version.
  - Worldgen, lighting, networking and rendering are not implemented by this crate; scheduled tick callbacks are also not implemented. It is not a complete game/world engine.
  - The implementation imports 35,723 block-state identities/defaults; it expressly does not import block implementations, collision/outline shapes or behavior.
- Storage behavior and reported validation:
  - NBT supports all tag types, named/unnamed roots, Java modified UTF-8 edge cases and 26.3 heterogeneous list wrappers; it applies explicit application limits and documents canonicalization differences.
  - Region support includes gzip, zlib, uncompressed and LZ4-java block framing, with validation of headers, lengths, sector overlap, external chunks and negative-coordinate chunk math.
  - Paletted sections handle block and biome arrays, uniform sections, packed words, registry-resolved numeric state IDs and compaction.
  - Data migration applies file-level and level.dat fixes, then server DataFixers across chunks, entities, POI, players and related save files. Documentation claims server fixes are ported from 1.8 onward, while client-file fixes are not. This is a save migration capability, not implementation of old-version gameplay.
  - Test scope is synthetic and differential against vanilla classes/JARs. It does not establish full generated-world lifecycle, playable integration or performance improvement.
- RustChunk relevance: very high for understanding which layer Starling has actually implemented and for planning save compatibility. Its region/NBT/Anvil palette code may be a useful implementation reference. The DataFixer work suggests that reading/upgrading older saves is a separately handled subsystem; however, it does not prove that the game logic, registries, worldgen or runtime can execute at 1.20.1.
- 1.20.1 implications:
  - 1.20.1 data version and registry/state IDs differ from 26.3. Direct use of the 26.3 registry export or DATA_VERSION=5023 would misinterpret 1.20.1 data unless a version-aware data path is introduced.
  - DataFixers should be used in the correct direction and for the intended operation. The documented pipeline upgrades old worlds toward the pinned 26.3 schema; it is not a downgrade path and cannot be assumed to preserve semantics when targeting 1.20.1.
  - NBT and Anvil container mechanics are more reusable than the game-specific schema, but 26.3 list-wrapper/canonicalization assumptions and all block-state identities need comparison against 1.20.1 vanilla.
- Dependencies / cross-references: `crates/horizon-world`; its `datafix`, `nbt`, `region`, `chunk`, `registry` and fluid modules; `horizon-prepare` registry exporter; local vanilla oracle; `tools/test_datafix_parity.py`; world/tick/fluid parity suites; docs/block-state-codecs.md and docs/scheduled-ticks.md.
- Decision: **adaptable** for generic NBT/Anvil/palette mechanics and the methodology of DataFixer parity; **rewrite/version-gate required** for registries, DataVersion, block-state tables, chunk schema and any 1.20.1 world semantics. The crate itself is not evidence of a complete playable 1.20.1 engine.
- Follow-up checks: inspect `horizon-world/Cargo.toml`, `nbt.rs`, `region.rs`, `chunk.rs`, `registry.rs`, the DataFixer implementation and its version schema; inspect generated 26.3 registry artifacts and how `horizon-prepare` produces them. Compare actual 1.20.1 data version, registry exports, palettes, DataFixer schemas and world fixtures.
- Completion state: complete document review; no world tool, DataFixer, world conversion, vanilla oracle or parity suite was run.


### 24. `docs/native-client.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `51d9245e4d9d4a7b467004d6c07db10d59065d51`.
- Size: 54,334 bytes; 857 lines; fetched in consecutive ranges 1–400, 401–800 and 801–857.
- Coverage: lines 1–857, complete; all three ranges returned the same blob SHA.
- Purpose: documents the Rust-native playable client, its setup and resource preparation, implemented gameplay/rendering slices, agent-control API, operating-system target, and extensive scoped Java-parity evidence.
- Version and platform constraints:
  - The client is explicitly a Rust rewrite targeting Minecraft Java Edition 26.3 and consumes the matching prepared resources/data. It does not directly execute Minecraft's Java client.
  - Source preparation requires current stable Rust (1.90+), JDK 25 and the user's own official 26.3 client/server files; data and assets are exported locally and kept private. The preparer and all generated resources are version-stamped and verified against repository pins.
  - The documented playable target is Linux x86-64 with an NVIDIA GPU/Vulkan. macOS and Windows are stated out of scope for this client; Android/ARM64 is not claimed as a supported target in this document.
- Implemented functionality described:
  - A worker-based chunk streamer; chunk lighting and model meshing; block models, blockstates and biome tints; water/lava rendering; day/sky/fog timeline; item models; mining/tool hardness; loot tables, placement, crafting; Anvil saving/reloading; selected player movement, health, hunger, sprint/crouch and attribute logic.
  - The `horizon-ctl` interface supports observation, movement and actions, widgets/options, screenshots and an MCP stdio service. This is an agent-control layer, not the Minecraft multiplayer protocol itself.
  - The client uses data exported from the pinned client and server JARs, including resource models, collision shapes, registries, loot, recipes and other runtime resources. Generic engine code and vanilla-derived version data are intertwined through preparation and generated resources.
- Explicit incomplete areas and qualification of the word playable:
  - Full authoritative player tick and LocalPlayer lifecycle are incomplete; entity-dependent scene queries, full entity lifecycle, death/respawn, vehicles, effects, item use, network broadcasts and the complete client/server tick remain unfinished.
  - World-generation integration is not established by this document; many scene/physics comparisons are controlled fixtures and use empty equipment/no nearby entities or other restricted actor domains.
  - Neighbor updates, block drops, block-entity lifecycle, audiovisual playback and networking still need consumers in the described client. A mesh refresh currently rebuilds the scene and explicitly claims no performance improvement.
  - Render/model tests validate data decoding, CPU geometry or renderer startup; pixel parity, hardware performance and physical-GPU validation on target devices remain separate. Linux llvmpipe smoke checks are not evidence of Android/Mali performance.
  - The document's own scope statements repeatedly distinguish parity of a narrowly controlled adapter from complete gameplay parity.
- RustChunk relevance: extremely high for architecture and project-goal evaluation. It proves Starling contains a substantial native Rust reimplementation, not merely a themed launcher, but it also shows this reimplementation is a large and still incomplete Minecraft 26.3 port. Existing control/gameplay pieces can guide component boundaries and test design; they do not provide a drop-in Android Minecraft 1.20.1 runtime.
- 1.20.1 / device implications:
  - The client has pinned 26.3 assumptions in resources, components, registry IDs, models, assets, physics rules and vanilla oracles. Porting to 1.20.1 requires a version-matched dataset and code/API reconciliation throughout these components, not just changing a constant.
  - Running on Galaxy A22 ARM64/Mali is outside the platform/performance evidence in this document. Vulkan/NVIDIA/Linux-only constraints mean Android integration, renderer/backend, ABI, memory behavior and GPU support must be audited separately; no mobile support should be inferred from the Android launcher or other Android documentation.
  - This page does not show server multiplayer support in the client; an MCP/agent control socket is not a Minecraft server connection. The network implementation must be examined separately.
- Dependencies / cross-references: `horizon-client`; `horizon-prepare`; `horizon-assets`; `horizon-world`; `horizon-client-core`; `.local/horizon-client/26.3`; `research-lock.json`; `horizon-ctl`; `tools/test_native_scene_collision.py`; scene motion/health/hunger/sprint/crouch parity harnesses; model/blockstate/animation parity tests; `docs/rust-parity.md`; `docs/graphics.md`; Android and renderer-specific docs.
- Decision: **adaptable** as a Rust-native client architecture, resource-preparation design, scoped Java-oracle methodology and module decomposition; **major rewrite/version adaptation required** for 1.20.1 semantics and Android ARM64/Mali deployment. The current client is neither a ready-made 1.20.1 client nor proven Android game runtime.
- Follow-up checks: inspect `horizon-client` crate entry point, launch/render loop, `horizon-prepare` resource exporters, version constants, renderer backend/platform features, networking/protocol integration, worldgen/game loop and unfinished client lifecycle; inspect Android packaging and compare actual ABIs/GPU backend requirements. Verify 1.20.1 Java oracle/resources and differential tests separately.
- Completion state: complete document review; no client build, gameplay session, renderer, Java oracle, test suite, Android package or hardware performance run was executed in this audit.


### 25. `docs/architecture.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `838ff3bcf985a689c0732ccb9e80deb89ed4d4b8`.
- Size: 10,769 bytes; 69 lines; fetched in one complete range.
- Coverage: lines 1–69, complete.
- Purpose: an index of the 40 most frequently changed feature areas over a 30-day window, showing their source modules, crate locations and related parity checks.
- Architecture facts:
  - The executable `horizon-client` is a thin entry point; gameplay and reusable library code are split across `horizon-client`, `horizon-client-lib` and supporting crates.
  - World persistence and DataFixers sit in `horizon-world` and client save modules; world generation and server simulation span `horizon-world` and `horizon-client`; multiplayer networking is split between game remote/LAN code and `horizon-protocol`.
  - Rendering, input, registries/items, commands, assets, launcher and mods have separate crates/modules. The table maps game loop, worldgen, network, renderer, chunks, physics, player, inventory, mobs, audio and many more systems to focused tests.
  - The document recommends asking the parity runner for affected harnesses rather than relying on historical test counts. It notes the reusable probe library does not compile all gameplay code, so a source file's dep-info alone can miss relevant harnesses.
- RustChunk relevance: high as a map for selecting the actual source files to inspect and for understanding cross-crate boundaries. It helps avoid treating Starling as a single monolith or assuming the launcher alone contains the game implementation.
- 1.20.1 implications: this index identifies the subsystems that must be version-audited, but does not describe their Minecraft version compatibility, runtime feature completeness or 1.20.1 behavior. The 40-area history is a recent churn map (through 2026-10-23), not a version-port plan and not an exhaustive list of every dependency.
- Decision: **adaptable** as an audit/navigation map and modular architecture reference; **insufficient by itself** to establish 1.20.1 support. No implementation decision should be made from this index without reading the actual referenced source and versioned data.
- Dependencies / cross-references: `crates/horizon-client`; `crates/horizon-client-lib`; `crates/horizon-world`; `crates/horizon-protocol`; `crates/horizon-client-render`; `crates/horizon-client-input`; `crates/horizon-client-core`; `crates/horizon-command`; `crates/horizon-items`; `tools/parity.py`; the harnesses named in the table; `AGENTS.md`.
- Follow-up checks: use the map to guide source-level audits of startup/resource loading, game loop, worldgen, save/load, packet processing, renderer and launcher integration; run no harness until its environment and 1.20.1 oracle are confirmed.
- Completion state: complete document review; no test selection, build or parity command was executed.


### 26. `docs/how-horizon-is-built.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `91a037b8d46468cf3e77acfd24a23b1837312bcf`.
- Size: 6,430 bytes; 41 lines; fetched in one complete range.
- Coverage: lines 1–41, complete.
- Purpose: summarizes Horizon's development, parity/review/CI process, distribution security, mod trust model and legal boundary around Minecraft game files.
- Key claims and constraints:
  - At the document's 2026-10-25 snapshot, it reports 326 harnesses comparing Horizon with real Minecraft 26.3, PR checks and selected parity runs; the text explicitly warns that passing all scoped harnesses does not prove complete equivalence.
  - It describes an open-source MIT project whose releases are built from public code, with signed SHA-256 manifests and launcher/server verification, while noting platform signing/notarization gaps.
  - It states that Horizon contains no Minecraft code/assets/data and does not download them; the player's own copy of Minecraft Java Edition 26.3 is prepared locally. A public-tree check is intended to prevent committing Mojang game files or decompiled code.
  - The project claims its post-2026-10-24 implementation is written from Minecraft 26.3 and public documentation, with strict third-party license constraints. Native mods execute with the user's rights and are not sandboxed at runtime.
- RustChunk relevance: very high for the legal/provenance design of obtaining game assets and code-derived facts. It establishes a clear distinction between open-source Rust implementation and player-supplied official Minecraft files. It does not show that the same preparation workflow accepts 1.20.1 or that any Minecraft files can be redistributed in RustChunk.
- 1.20.1 implications: if RustChunk targets 1.20.1, the permitted source of assets/JARs and the preparation path must be version-specific and must not assume the 26.3 checksums or existing data exporters are applicable. The code review must keep decompiled code and proprietary assets out of the RustChunk repository unless a separately verified legal basis allows otherwise. The page is a project policy summary, not a legal opinion.
- Decision: **adaptable** for user-owned asset preparation, public-source hygiene, signed release checksums and scoped parity gates; **no direct runtime code reuse decision** from this policy page. Reconfirm actual license and contribution constraints from the governing LICENSE/CONTRIBUTING files before any port or copying.
- Dependencies / cross-references: root LICENSE and CONTRIBUTING.md; THIRD_PARTY_NOTICES.md; `tools/check_public_tree.py`; `.github/workflows/ci.yml`; `.github/workflows/release.yml`; `crates/horizon-release-sign`; `horizon-prepare`; research-lock.json; docs/security.md; docs/mods.md and docs/native-mods.md.
- Follow-up checks: inspect the actual legal notices and the preparation/checksum implementation; verify Minecraft 1.20.1 ownership/manifest handling and local data-generation support independently. No reliance should be placed solely on this summary page for legal conclusions.
- Completion state: complete document review; no license review beyond this document, checksum verification, build, release or asset import was performed in this file review.


### 27. `docs/decompilation-status.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `80d39be68e65a2abe57d3da350a63a7ed7979d0a`.
- Size: 2,958 bytes; 37 lines; fetched in one complete range.
- Coverage: lines 1–37, complete.
- Purpose: records the scope and limitations of the local decompilation of pinned Minecraft 26.3 client/server JARs, plus a second query path using REA/JADX against bytecode.
- Verified 26.3 snapshot:
  - The document says it was verified 2026-10-03 with official 26.3 artifacts, Vineflower 1.12.0 and OpenJDK 25.
  - It reports client coverage of 11,383 classes / 98,739 methods / 7,301 expected and emitted Java files, and dedicated server coverage of 7,762 classes / 72,269 methods / 5,037 files, with zero missing files or detected failed-decompilation markers.
  - Third-party JARs were references only; runtime/native libraries and original build environment were not recovered. The full source/report outputs stay local and gitignored.
  - It explicitly warns that emitted-file coverage and absence of detected markers do not prove semantic correctness or rebuildability; original bytecode and differential harnesses remain the behavior reference.
- Bytecode query tooling: REA/JADX can inspect classes, methods, callers, overloads, bridge methods and lambda call sites where Vineflower text is ambiguous. Tool versions are pinned; both routes are reading aids, not substitutes for bytecode/oracle verification.
- RustChunk relevance: high for selecting a reliable research workflow for a historical 1.20.1 target. 1.20.1-era JARs are from a different mapping/deobfuscation regime than the modern unobfuscated 26.3 assumption in the version-port design; therefore class-name mapping, remapping and decompiler quality become key risks.
- 1.20.1 implications: the counts and exact Vineflower/REA pipeline in this file apply only to 26.3. They do not establish complete 1.20.1 coverage, readable symbol names, successful decompilation, or an ability to rebuild Mojang's original client. We must separately verify 1.20.1's mapping artifacts, official version manifest, client/server JARs, Java baseline, and whether mappings/obfuscation can be applied to both sides consistently.
- Decision: **adaptable** as a bytecode research/decompilation workflow and as a warning to validate coverage; **rewrite/adapt required** for the 1.20.1 mapping and toolchain inputs. Decompilation is an analysis tool, not a route to redistribute or directly execute recovered Java source.
- Dependencies / cross-references: `tools/lab.py`; `tools/rea_vanilla.py`; `research-lock.json`; Vineflower; REA/JADX; `.local/minecraft/26.3/sources/`; local class inventories and coverage reports; `docs/version-ports.md`.
- Follow-up checks: inspect `tools/lab.py` and `tools/rea_vanilla.py`; inspect the actual 1.20.1 manifest's client/server mapping fields and JAR hashes; assess a remapped class inventory and decompiler report for 1.20.1 before depending on class-based diffing.
- Completion state: complete document review; no JAR decompilation, bytecode query, mapping application or build was executed.


### 28. `docs/port-ledger.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `4251e0bf526660edf64ac59e3ed4131528139517`.
- Size: 11,611 bytes; 147 lines; fetched in one complete range.
- Coverage: lines 1–147, complete.
- Purpose: describes a method-level mapping ledger that correlates vanilla Java methods/bytecode calls with Rust functions, source spans, known gaps and parity harnesses.
- Matching model:
  - The builder reads the official pinned game JAR for exact bytecode call targets/constants/IDs and local decompiled client/server Java sources for line-level source references. It writes derived reports locally, not into the repository.
  - Mapping confidence levels include explicit `port` doc comments, name/class matches, manually curated verdict-map rows, mentions, linked callees, structural state similarity, rare-constant/ID inference, trivial methods and missing methods.
  - It detects possible seam/client/drift/class/callee/stale/map/unported gaps. The document stresses a gap is a lead that must be checked against both Java and Rust code, not a verdict.
  - Parity harnesses are cross-linked to method mappings, but a mapped method is not considered behaviorally proven merely because a Rust location was found. Uncovered method pairs can be ranked for new tests.
  - Branch checking compares changed Rust files with the baseline and lists new/closed gaps; test code, fixtures, probes and examples are excluded from being counted as ports.
- RustChunk relevance: critical as a method-level port-completeness methodology. Moving a substantial Java-version-specific Rust implementation toward 1.20.1 requires a traceable mapping from each 1.20.1 vanilla method to its Rust implementation, an explicit verdict when no direct method is needed, and parity evidence for behavior. This method ledger complements the class-level map in the version-port pipeline.
- 1.20.1 implications and caveats:
  - The existing method ledger is designed around the current pinned 26.3 game and 26.3 Java class/method names. It cannot simply be rebuilt using 26.3 source and claim 1.20.1 coverage.
  - 1.20.1 class/method names and bytecode differ materially, particularly due to the older obfuscation/mapping regime. A reliable 1.20.1 input requires mapping-aware JARs/decompilation and a new vanilla source oracle; class and method matches must be recalculated rather than carried over by names.
  - Structural and inferred matches can be false positives. Every important inferred mapping must be manually verified and backed by tests; test reachability does not automatically establish complete lifecycle coverage.
- Decision: **adaptable** as a core audit tool/concept (method mapping, verdict map, gap taxonomy and harness reachability); **rewrite/version-adapt required** for historical 1.20.1 JAR/mappings and its different Java API. The ledger is a planning/coverage instrument, not a game engine feature.
- Dependencies / cross-references: `tools/port_ledger.py`; `tools/vanilla_map.py`; `tools/port_map/*.tsv`; `tools/lab.py`; local Minecraft JAR and decompiled sources; `tools/parity.py`; Java parity oracles; Rust crates and doc comments; `tools/test_*_parity.py`.
- Follow-up checks: inspect `tools/port_ledger.py`, `tools/vanilla_map.py`, the verdict-map schema, class/method inventory generator and parity runner; then evaluate whether mapping-aware 1.20.1 bytecode can feed a trustworthy adapted ledger.
- Completion state: complete document review; no ledger build, bytecode scan, gap report, harness or branch check was run.


### 29. `docs/parity-workflow.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `45ab857edfa8662e62d30d0a87bfaa8dac798433`.
- Size: 23,390 bytes; 182 lines; fetched in one complete range.
- Coverage: lines 1–182, complete.
- Purpose: describes Starling/Horizon's differential parity runner, dependency tracing, caching, multi-host scheduling, test isolation, change-based harness selection, flaky-test gates and integration workflow.
- Runner and test architecture:
  - `tools/parity.py` executes named or dependency-selected harnesses, can run all from scratch, force fresh vanilla oracle runs, repeat tests and run on multiple hosts.
  - A tracer captures Python harness modules, Java oracle/source dependencies, read repository files and Rust probe dep-info; cached passes remain valid only while all dependency fingerprints, pinned vanilla artifacts and toolchains remain unchanged.
  - `parity_shim.py` records/replays Java/Javac oracle output and named input file contents, with a stated caveat that files written outside the named paths are not tracked.
  - Rust probes are consolidated into per-crate probe binaries; optional Rust scopes narrow dependencies only when the declared scope is closed. Reachability selection is approximate and falls back to all linked sources when uncertain or over budget.
  - Harnesses use scratch game/launcher/XDG homes, explicit game environments and virtual-screen modes. Remote runners have distinct trust, CPU/memory and host-exclusion boundaries; remote results are not CI results.
  - The gate runs public-tree validation, formatting, Clippy, Rust/Python tests and affected parity harnesses; new/changed harnesses must pass three consecutive runs to address flakiness. Worktree integration gates the combined branch and can bisect failures.
  - The workflow separates deterministic component fixtures from player-facing LAN/integration checks. Skipped cases are not counted as passes; named-only suites are not selected automatically.
- Version boundary and limitations:
  - Harnesses and Java oracles in this system compare against the repository-pinned Minecraft 26.3 artifacts. The described tooling and reported test counts do not demonstrate behavior against Minecraft 1.20.1.
  - Parity coverage is only as broad as each oracle, fixture, Rust scope and dependency graph. The document explicitly warns that narrow fixtures do not become complete game launches, that skip outcomes are not passes, and that some caches cannot observe all external writes.
- RustChunk relevance: critical as a quality-control pattern for any attempt to port Starling to 1.20.1. A version-matched vanilla oracle and isolated deterministic fixtures can prevent silent drift across protocol, worldgen, save, renderer and gameplay code. Dependency-aware test selection may also make a large port manageable, but it must never suppress necessary full audits.
- 1.20.1 implications: every vanilla oracle, fixture baseline, generated reference and cached pass must be regenerated or revalidated for 1.20.1. Reusing cached 26.3 oracle outputs would invalidate the port audit. Toolchain and JDK selection must also follow the 1.20.1 target rather than the current 26.3/JDK 25 assumptions.
- Decision: **adaptable** for differential testing, dependency fingerprinting, repeat/flake gates, deterministic scene fixtures and staged integration; **rewrite/version-adapt required** for a 1.20.1 vanilla oracle, fixtures, baselines, toolchains and expected data. Do not treat the existing 26.3 pass count as evidence of 1.20.1 parity.
- Dependencies / cross-references: `tools/parity.py`; `tools/parity_shim.py`; `tools/parity_reach.py`; `tools/gate.py`; `tools/parity_main.py`; `tools/supervise.py`; `tools/parity_hosts.json`; `lab.game_env`; Rust probe crates; Java oracle sources under `mods/horizon`; `.local/parity-runner/`; `shared-local/` and `PARITY_SHARED`.
- Follow-up checks: inspect parity runner, oracle shim, host scheduler, cache key/fingerprint logic, scope closure/reachability, baseline format and gate integration. Then construct a separate 1.20.1 oracle fixture matrix; never blend results from distinct Minecraft versions.
- Completion state: complete document review; no test, runner, cache, host, gate, harness, or integration command was executed.


### 30. `crates/horizon-prepare/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `edc8c6e78c7698980a70b3f7b2e90344f5eec99f`.
- Size: 792 bytes; 26 lines; fetched in one complete range.
- Coverage: lines 1–26, complete.
- Purpose: declares the `horizon-prepare` Rust package, dependencies, target-specific filesystem crates and executable entry point.
- Dependency findings:
  - Rust 2021 package; package-level dependencies include CRC32, regex, SHA-1, SHA-256 and ZIP/deflate support. These fit local archive parsing, integrity checks and asset/JAR preparation rather than containing a Java game runtime.
  - Android target includes `rustix` with filesystem support; Linux/macOS use `libc`. This indicates the preparer crate has an Android compilation path at the manifest level, but it does not establish that the full client/game engine or all its runtime dependencies are Android-ready.
  - No networking, game logic, OpenGL/Vulkan renderer, Java VM or Forge loader dependency is declared here. The Java runtime is invoked by the preparation implementation and must be inspected separately.
- RustChunk relevance: moderate-to-high as a compact reference for a native Rust resource-preparation tool that can target Android and perform SHA-1/SHA-256 verification and archive extraction.
- 1.20.1 implications: the dependency set itself is largely version-neutral, but the actual expected files, hashes, archive layout, data-generator invocation and generated outputs are determined in source code and repository pins. Updating only this manifest would not add 1.20.1 support.
- Decision: **adaptable** as a resource-preparer dependency pattern; **no direct version-port decision** can be made from the manifest alone.
- Dependencies / cross-references: `crates/horizon-prepare/src/main.rs`; `java.rs`; `artifacts.rs`; `lib.rs`; `check.rs`; root Cargo workspace; `research-lock.json`.
- Follow-up checks: inspect the preparer's command flow, Java version selection, expected official launcher files, archive verification, exporters, and resource version stamp; separately confirm Android runtime feasibility.
- Completion state: complete manifest review; no Cargo resolution, build, target compilation or dependency audit was executed.


### 31. `crates/horizon-prepare/src/check.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `ae95ea85ea4298334dbd0ee7f2c38d9b914fb59c`.
- Size: 10,928 bytes; 279 lines; fetched in one complete range.
- Coverage: lines 1–279, complete, including all unit tests.
- Purpose: implements the `horizon-prepare --check` resource validation path and its stamp lifecycle, and identifies optional resource groups that need separate preparation commands.
- Behavior:
  - Reads the root `research-lock.json`, then checks `horizon-resources.json` for the expected schema and exact pinned Minecraft version string.
  - Requires a `horizon-prepare.json` stamp whose `output_version` matches the current preparer's `OUTPUT_VERSION`.
  - Checks presence of built-in packs, six valid non-placeholder square title-panorama PNG faces plus overlay, Unifont zip, language metadata and sounds metadata.
  - Preparation removes the stamp before writing and stamps only after completion. A missing/old stamp or mismatching Minecraft version leads to a suggested full prepare command.
  - The implementation expressly does not run Java or inspect the game files during `--check`; it tests file presence and version stamps, not resource contents. Its tests cover missing folders, complete resources, per-extra gaps, invalid panorama dimensions, stamp removal and lock waiting.
- RustChunk relevance: high for a safe and repeatable local resource-preparation flow and for understanding how a launcher decides whether resources need rebuilding. The separation between core required data and optional user-asset-dependent extras is reusable.
- 1.20.1 implications:
  - The strict equality check against `lock.version` correctly prevents a directory prepared for 26.3 from being treated as current for 1.20.1, assuming RustChunk pins the target independently.
  - `SCHEMA` and `OUTPUT_VERSION` track the preparer's own layout/tool version, not the Minecraft data version; these should remain distinct from the Minecraft version pin.
  - Existence and stamp checks do not prove the actual data is 1.20.1-compatible or intact. The export/manifest/hash process must provide the authoritative content checks; this file alone is not a data-integrity boundary.
- Decision: **adaptable** as a preparation-completeness/stamp/version guard. It is mostly version-agnostic; exact 1.20.1 support depends on the manifest, lock reader, exporters, resources and checks in other modules.
- Dependencies / cross-references: `research-lock.json`; `artifacts::Lock`; `OUTPUT_VERSION`; `SCHEMA`; `horizon-resources.json`; `horizon-prepare.json`; `missing_extras`; launcher backend completion checks; resource commands `packs`, `panorama`, `fonts`, `languages`, `sounds`.
- Follow-up checks: inspect `main.rs`, `lib.rs`, `artifacts.rs`, the manifest writer and resource exporter implementations to verify that the stamp and Minecraft version are written only after all mandatory 1.20.1 artifacts are verified.
- Completion state: complete source and unit-test review; unit tests were read but not executed.


### 32. `crates/horizon-prepare/src/main.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `55ce8e1f306532fd2613e98ab63f115b9e52305e`.
- Size: 16,104 bytes; 369 lines; fetched in one complete range.
- Coverage: lines 1–369, complete, including argument parsing and unit tests.
- Purpose: CLI entry point for importing the player's official Minecraft Java installation and preparing Horizon's version-pinned resources.
- Behavior and dependencies:
  - Reads `research-lock.json` from a resolved root; that lock governs the Minecraft version and Java requirement. Horizon's own application `VERSION` is a separate build/release value, not the Minecraft game version.
  - Supports imports via an official launcher game directory or explicit version JSON, client JAR, libraries and assets directory; accepts a separately supplied server JAR and a Java home. Imported files are verified by the internal artifact/lock logic.
  - Exposes `--prepare`, `--check`, `--launch-api`, `compile-exporters`, and optional asset commands for sounds, panorama, fonts, built-in packs and languages. Asset commands can be rerun independently; `--complete` adds sounds and enforces the readiness check.
  - Can emit progress as line-oriented JSON (`message`, `fraction`, `error`) for launcher integration. `--launch-api` reports the preparer's API, pinned Minecraft/Java versions, resource layout and supported options.
  - It does not itself contain the game data exporters; it delegates to the `horizon_prepare` library. Java may be needed for exporters/preparation, while `--check` intentionally avoids Java and game-file reads.
  - The source's tests verify progress JSON compatibility and that advertised Launch API options are present in help output; they were read but not run.
- RustChunk relevance: very high as a launcher/resource-ingestion architecture: it cleanly separates application version, target Minecraft version, the player's local official files, Java runtime selection, prepared resources, and launcher-facing progress/API contracts.
- 1.20.1 implications:
  - Switching the lock to 1.20.1 would alter the default resources folder and expected game version, but it is not enough by itself. `artifacts`, `java`, exporters, generated resource schemas and every game reader must accept the historical 1.20.1 client/server JAR and metadata.
  - 1.20.1 requires its matching Java baseline and matching client/server artifact hashes; do not carry over the 26.3/JDK25 assumptions without validating the 1.20.1 version manifest.
  - This CLI is a preparer for the native Rust reimplementation. It does not launch the original Java Minecraft client and does not establish a JVM/Forge runtime.
- Decision: **adaptable** as the preparation CLI and launch API; **version-specific implementation changes required** in the lock/artifact resolver, Java selection, exporters, and output resource contracts before it can prepare 1.20.1 resources.
- Dependencies / cross-references: `horizon_prepare::Options`; `artifacts::Lock`; `prepare_with_progress`; `add_assets`; `compile_exporters`; `check`; `launch_api`; `research-lock.json`; `horizon-resources.json`; launcher Launch API.
- Follow-up checks: inspect `java.rs`, `artifacts.rs`, `lib.rs`, exporters, output manifest/stamping, and the precise 1.20.1 version metadata/JDK requirement. Confirm whether the preparer's asset path expects 26.3-only layouts.
- Completion state: complete source and test review; no preparation, Java invocation, asset import, build or test was executed.


### 33. `crates/horizon-prepare/src/java.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `b33229cbf56d422449a8c62672b27cd706fb9b97`.
- Size: 21,900 bytes; 609 lines; fetched in consecutive ranges 1–400 and 401–609.
- Coverage: lines 1–609, complete; both ranges returned the same blob SHA; unit tests included.
- Purpose: discovers/provisions Java tools, applies memory options, builds platform classpaths and runs Java/Javac processes used by vanilla exporters and datagen.
- Java discovery and execution:
  - A caller can inject a `Supplied` Java/Javac pair and environment through a process-wide `OnceLock`. The module comments identify Starling on Android as the case where ordinary system Java is unavailable and the app supplies its private runtime.
  - Discovery order is supplied runtime, `JAVA_HOME`, JDKs found through `PATH`, macOS `java_home`, launcher/Minecraft-provided homes, then common JDK installation directories. It validates tool versions by invoking `-version`, handles Java 8's `1.8` format and avoids pairing a standalone old `java` with a different JDK's `javac`.
  - `find` requires `javac` and `java`; `find_runtime` only requires `java` for precompiled exporters. Both accept a requested major from the pinned lock and currently accept versions greater than or equal to it.
  - `javac` invokes `--release <major> -proc:none`; Java commands use short-batch options `-XX:TieredStopAtLevel=1` and `-XX:+UseSerialGC`. Each process has a one-hour timeout, null stdin and combined output to a log.
  - Android 32-bit alone uses `SMALL_ANDROID` and reads `/proc/meminfo` to cap exporter heaps at 384 or 512 MiB. Android 64-bit does not activate this cap. Requested heap sizes are clamped only when larger than the computed maximum.
  - It resolves launcher-managed Java runtimes, including macOS bundle layout, and normalizes Windows verbatim drive paths.
- Android/RustChunk relevance: very high. This is evidence Starling's **resource preparation** can launch Java from an app-private runtime on Android; it does not mean the Rust game itself runs in a JVM. The supplied runtime hook is likely important when preparing 1.20.1 resources on a phone.
- 1.20.1 implications:
  - The required Java major is supplied by `research-lock.json`; the current comments and surrounding docs use Java 25 for 26.3. The exact 1.20.1 Java requirement must be read from its official version metadata and pinned independently (do not assume 25).
  - This module's minimum-version check can select a newer runtime than requested. Compatibility of a newer JDK with the historic Minecraft 1.20.1 data generator/exporter and decompiler needs verification; an exact Java 17 runtime may be a safer baseline if 1.20.1 metadata says 17.
  - Android 64-bit devices such as the Galaxy A22 do not receive the 32-bit heap cap in this implementation; its 6 GB class memory profile still requires measured heap choices and careful coexistence with the renderer. The 384/512 MiB rules apply only to 32-bit Android.
  - `javac --release` requires a compiler capable of the selected release target. Exporter source compatibility and dependencies must also be checked for the older 1.20.1 Java toolchain.
- Decision: **adaptable** for Android-supplied Java, JDK discovery, controlled batch execution and timeout handling; **version/toolchain policy changes and device-specific memory validation required** for 1.20.1. No claim is made that the game engine needs Java at runtime; Java here supports vanilla exporters/data prep.
- Dependencies / cross-references: `research-lock.json` Java major; `Supplied`/`supply`; `find`/`find_runtime`; `horizon-prepare` `Options.java_home`; `horizon-prepare/src/lib.rs`; `artifacts.rs`; exporter compilation and vanilla datagen; Android launcher runtime provisioning.
- Follow-up checks: inspect the source of runtime provisioning and `artifacts.rs` to verify major selection; confirm 1.20.1's version JSON/JDK baseline; inspect the Java exporter compile path; measure heap/time on Android ARM64 and ensure no unbounded concurrent JVMs.
- Completion state: complete source and unit-test review; tests and actual Java/Javac commands were not executed.


### 34. `crates/horizon-prepare/src/artifacts.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `69f47232438e887e8960db27ce22aba91f775c71`.
- Size: 66,697 bytes; 1,650 lines; fetched in consecutive ranges 1–400, 401–800, 801–1,200, 1,201–1,600 and 1,601–1,650.
- Coverage: lines 1–1,650, complete; all five ranges returned the same blob SHA; all unit-test code was reviewed.
- Purpose: validates, locates, imports and reuses the player's official Minecraft version manifest, client/server JARs, client libraries and server-bundled libraries into `.local/minecraft/<version>`.
- Lock and manifest model:
  - `Lock::read` obtains the Minecraft version, `java_major`, SHA pins and accepted republished-manifest/asset-index hashes from `research-lock.json`; validates the version identifier against path traversal; and resolves the version-specific local directory.
  - The imported manifest must have the exact pinned version ID, client/server SHA-1 values and a Java major matching the lock. Mojang-republished manifest hashes can be explicitly allow-listed.
  - If an alternate launcher rewrites Mojang's JSON, it is rejected unless the repository already has an accepted Mojang manifest copy. The error explains how to obtain the original manifest. This prevents trusting launcher-specific metadata as a substitute for the pinned official manifest.
- Launcher discovery and files:
  - Reads official launcher layout, ATLauncher layout, CurseForge's official-launcher Install folder, and instance folders whose parent launcher can be found. It supports explicitly named inputs for custom launchers and Prism's alternative Maven path for native classifiers.
  - Maps OS names and architecture aliases (including ARM64), applies the version JSON's library `rules`, filters x64/ARM native classifier variants, and skips only selected optional Netty native transports when absent. Other missing libraries fail.
  - All manifest/bundle paths pass `safe_relative`; absolute paths, backslashes, `..` and Windows drive-style paths are rejected. Library artifacts are checked against SHA-1; inner server bundle entries and libraries use SHA-256.
- Server JAR handling:
  - Accepts a supplied player-downloaded server JAR and verifies its pinned SHA-1. If it contains `META-INF/versions.list`, the implementation expects exactly one bundled server whose ID equals the lock, extracts the inner game JAR and server libraries, and checks their SHA-256 values against the bundle lists. If it lacks the bundler marker, it treats the supplied JAR as the server game itself.
  - Writes class inventories for client and server and a `prepared.json` inventory of the copied jars/libraries. It does not ship these JARs in the release.
- Reuse and integrity:
  - Every invocation can reuse previously imported files only after validating manifest, client/server JAR hashes, inner server JAR hash and server/client library hashes. It can repair missing import metadata and reuse a verified sibling version tree while skipping corrupted siblings.
  - The extensive tests construct synthetic ZIP/bundle fixtures for corruption, wrong target pins, alternate launcher layouts, changed manifests, absent optional Netty transports, path traversal and ARM-vs-x64 native selection. These tests were read, not executed.
- Critical 1.20.1 blocker:
  - `import` explicitly rejects any version manifest containing `downloads.client_mappings` or `downloads.server_mappings` with the message that the workflow targets unobfuscated releases and older releases need a remapping stage.
  - Minecraft Java 1.20.1 belongs to the historical obfuscated/mapped release regime; this path therefore cannot be assumed to import 1.20.1 successfully as written. A mapping/remapping stage or a carefully verified mapped-JAR strategy is required before inventory, exporters, class indexing and parity oracles can work with stable class/method names.
  - The import process otherwise appears designed to support version-specific checksums and data, but its server bundle assumptions, library classifiers and manifest rules must be validated against the actual 1.20.1 manifest and server archive, rather than inferred from the newer 26.3 example.
- RustChunk relevance: **critical**. This is the main gate between the user's legitimately installed Minecraft files and the native Rust data preparation pipeline. It identifies a concrete incompatibility with the current 1.20.1 goal, not a hypothetical concern: the current code deliberately refuses mapped releases.
- Decision: **major adaptation required** for 1.20.1. Reuse the secure path handling, checksum verification, manifest validation, explicit user-provided game files and local-only artifact storage; replace or extend the blanket mappings rejection with a verified 1.20.1 remapping workflow and version-aware handling of libraries/server bundling.
- Dependencies / cross-references: `research-lock.json`; `Lock`; `GameFiles`; `Inputs`; `game_files`; `import_manifest`; `import`; `unpack_server`; `client_library_artifacts`; `safe_relative`; `launcher_library`; `verify`; `client_libraries`; `inventory`; `files::Import`; ZIP helpers; `pool::map`; `.local/minecraft/<version>`.
- Follow-up checks: inspect `lib.rs`/`java.rs` and all exporter consumers; verify the actual 1.20.1 version JSON's mappings, SHA pins, Java major, libraries and server archive; determine the mapping namespace needed by Java oracles and Rust-source correspondence; test client and server remapping independently and record source/output hashes.
- Completion state: complete source and unit-test review; no actual game artifacts were imported, checksum-verified, unpacked or remapped, and no tests/build were run.
