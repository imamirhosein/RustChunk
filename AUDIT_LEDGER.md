# Starling → RustChunk audit ledger

- Source repository: `starling-client/starling`
- Source ref: `starling`
- Source tree snapshot recorded for this audit: `5432370d573642041585c69b40fb045b27133488`
- Target repository: `imamirhosein/RustChunk`
- Target branch: `starling-1.20.1-port-audit`
- Scope: assess Starling's architecture and implementation for adaptation into a Rust-native Android Minecraft Java Edition 1.20.1 client. Starling's documented target is Java 26.3; Forge is deferred and is not part of the current acceptance criteria.
- Rule: a file is complete only after its full contents (or all consecutive ≤400-line chunks) have been read, analyzed, and recorded here. Claims in documentation remain claims until verified in implementation.
- Decision labels: **essential**, **adaptable**, **rewrite required**, **integrate**, **unnecessary**, **undetermined**.

## Current project scope — authoritative

- The current RustChunk milestone is a **Rust-native Minecraft Java Edition 1.20.1 client**, adapted from the Starling/Horizon Rust architecture. It is not a JVM launcher in this phase.
- **Forge support is deferred** until the vanilla 1.20.1 base client works; do not use old Forge-oriented wording in early audit entries as the current product requirement.
- Starling is pinned to Minecraft Java 26.3. Its version-specific game rules, assets, registries, world formats and packet schemas must be reconciled with 1.20.1; changing a version string is not a port.
- Minecraft Java 1.20.1 uses protocol 763 and has Handshake, Status, Login and Play states. It does **not** have the separate post-login Configuration state introduced in later protocol generations.


## Audit standard and backfill rule

- The original entries 1–120 were reviewed under the then-current 13-part standard. Their prior source analysis, decisions and completion claims are preserved; this backfill does not invalidate those reviews.
- The current standard adds two mandatory fields to every file: **RustChunk implementation mapping** and **Evidence required before adoption**. Entries 1–120 have now been brought into that same 15-part structure.
- Mapping language distinguishes current RustChunk code from planned native-engine destinations. At this branch snapshot, the RustChunk Cargo workspace contains only the JNI-oriented `rustchunk-engine` package (download, installer, workspace, Forge launch-plan and Java/JLI helpers); it does not yet contain a playable native Minecraft game, protocol, world or renderer crate. Planned destinations are not claims of implemented functionality.
- Evidence requirements are acceptance gates, not reports of tests already run. A source review alone does not satisfy them; each requirement must be met by version-pinned 1.20.1 evidence and the appropriate automated/device test.

## Completed files

### 1. `README.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `3cd9d680757217774270f477ecbfc9c80563e39f`
- Size: 146 lines; 17,521 characters; full file fetched and read.
- Coverage: lines 1–146, complete. No chunks omitted.
- Purpose: project overview, legal boundary, Android capabilities/status, mod formats, build prerequisites and workflow, Minecraft resource preparation, contribution/licensing information, and supported-device claims.
- **Architecture evidence stated by the README:**
  - Starling is described as an Android port/fork of Horizon, whose game implementation is rewritten in Rust; the Android app is positioned around Horizon rather than being a conventional JVM launcher.
  - It targets Minecraft Java 26.3 and explicitly requires user-owned game files; Minecraft code/assets/data are not bundled.
  - It describes two mod paths: native `.hnmod` code compiled on-device (ARM64 only) and data-only `.hmod` content mods.
  - It describes Android APK packaging via `packaging/android/build_apk.py`, and host-side preparation via `horizon-prepare`; source game inputs are checksum-pinned through `research-lock.json`.
  - It documents Android account sign-in plus offline profiles, server support, shader/resource packs, touch controls, diagnostics, and device-specific performance settings. These are project claims, not independently validated implementation findings.
- Dependencies / cross-references: `roadmap/starling.md`, `docs/horizon-parity.md`, `docs/android-armv7.md`, `docs/android-controls.md`, `docs/android-accounts.md`, `docs/android-diagnostics.md`, `docs/native-mods.md`, `docs/starling-native-mods.md`, `docs/mods.md`, `packaging/android/build_apk.py`, `packaging/lowbudget/make.py`, `research-lock.json`, `Cargo.toml`, `Cargo.lock`, `CONTRIBUTING.md`, `THIRD_PARTY_NOTICES.md`, and `LICENSE`. These are README links/references; actual implementation dependencies and call relationships remain to be checked in their respective files.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high as a system-level map and requirements inventory. Particularly useful topics to investigate are the Rust/native execution boundary, Android packaging, legal/user-supplied game-resource preparation, ABI/GPU limitations, input abstraction, account/profile handling, diagnostics, and separation of code mods from data mods.
- **Important incompatibility / risk:** the documented target is 26.3, not 1.20.1. The README is a product/requirements map, not evidence that Starling's game rules, registries, assets, protocol, world data or resource formats are already compatible with 1.20.1. The native/content mod formats are also Starling-specific and must not be assumed to be part of the initial RustChunk base client.
- Decision: **adaptable** for high-level Android product design, diagnostics, touch controls, resource acquisition/verification, and build organization; **rewrite required** for any feature whose implementation assumes Starling/Horizon internals or version 26.3; Forge compatibility is outside the current milestone and is deferred. **Forge classification:** current conclusion: Forge compatibility is outside the current milestone and deferred.
- Follow-up checks: validate each documented capability against source code; inspect Cargo workspace/crates and build pipeline; trace how 26.3 files are prepared and consumed; inspect account/auth and offline-mode semantics; verify the Rust-native 1.20.1 data, protocol, world and rendering requirements; compare supported ABIs and graphics APIs to RustChunk's target hardware.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: This is a requirements/navigation source for `README.md` and `docs/STARLING_1_20_1_ENGINE_MAP.md`; it does not map to a runtime module. Keep product scope, device constraints, resource ownership and milestone order synchronized there. No current native gameplay module is present.
- Evidence required before adoption: Before using any README claim as an implementation requirement, trace it to source, a pinned 1.20.1 input or a passing CI/device result. Confirm that 26.3 baseline claims, native-mod formats and future Forge goals are not presented as 1.20.1 capabilities.
- Completion state: complete documentation review; implementation claims not yet validated.

### 2. `LICENSE`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `e959a076e9fa4b8c1fcf990b2253a935da5bb5f4`
- Size: 22 lines; full file fetched and read.
- Coverage: complete; no chunks omitted.
- Purpose: standard MIT license text. It attributes the copyright to Ludovic Marie and Horizon contributors (2026), grants broad rights to use, modify, distribute, sublicense, and sell the licensed software, requires retaining the notice, and disclaims warranties and liability.
- Dependencies / cross-references: this file is the repository's top-level license declaration. It does not enumerate third-party dependency licenses or independently grant rights to Minecraft/Mojang assets. The README explicitly limits MIT coverage to original project work; `THIRD_PARTY_NOTICES.md` is the next source for component-specific attribution.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical due-diligence input before copying or adapting code. MIT is permissive, but the copyright notice and license text must be retained for substantial copied portions; dependencies and upstream code may have separate obligations.
- **Limitations:** a root MIT license alone does not establish the license of every vendored/generated/resource file or third-party dependency. Do not infer that game assets, Minecraft binaries, or third-party libraries are covered.
- Decision: **adaptable** as a licensing model reference; copying Starling/Horizon code requires preserving notices and checking each dependency's actual license. Minecraft files remain outside this license scope.

- 1.20.1 implications: the MIT license is version-independent, but it covers only the licensed project code; it does not grant rights to Minecraft 1.20.1 binaries/assets or third-party dependencies. Preserve notices for any code actually reused.

- Follow-up checks: No additional follow-up check is recorded in this entry.
- RustChunk implementation mapping: Maps to RustChunk root `LICENSE` and its future third-party notice/provenance records; no runtime subsystem. Any reused Starling/Horizon source must be traceable to its exact license and source revision.
- Evidence required before adoption: Require a per-file/per-dependency license inventory, preserved notices, confirmation of asset/JAR exclusion and review of any non-MIT component before accepting copied code or distributing a build.
- Completion state: complete license-text review; third-party notices and dependency metadata still require separate review.

### 3. `THIRD_PARTY_NOTICES.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `825d4bb50daf691d9049b3d4cfdbc62dbc285ec2`
- Size: 300 lines; 20,016 characters; full file fetched and read.
- Coverage: lines 1–300, complete.
- Purpose: detailed attribution and license inventory for third-party code, ports, assets, fonts, icons, fetched runtimes, and Horizon Android adaptations.
- **Components and material disclosures found:**
  - Vendored `wgpu-hal` and `naga` 30.0.1, each MIT OR Apache-2.0, with documented upstream fixes; `libmimalloc-sys`/mimalloc sources under MIT, including a portable-popcount change.
  - Significant Create 6.0.10 mechanical/processing system ports into Horizon game crates; Create code MIT, while Create assets are All Rights Reserved and fetched from Modrinth on the player's device rather than included.
  - Iris Features sky-shape logic, MIT; Pixelify Sans under SIL OFL 1.1; Pixelarticons and Hugeicons sets under MIT; Discord mark from Simple Icons under CC0, with trademark note.
  - AMD FidelityFX SDK 1.1.4 optical-flow/frame-interpolation shaders and related implementation, ported to WGSL/Rust under MIT with AMD attribution/trademark notice.
  - Loader logos (Fabric, Quilt, NeoForge, Forge) with differing logo licenses/marks; Microsoft sign-in branding copied from Microsoft's official symbol/branding guide. **Forge classification:** historical reference: loader logo/branding and licensing inventory only; this does not establish loader support.
  - Semantics from CIT Resewn Continuation and layout tables derived from FusionExamples, both MIT; BlueMap low-resolution map tile/settings behavior, MIT.
  - OpenJDK 25 runtimes are fetched on-device for preparation and deleted afterward; the fetched JDK is GPLv2 with Classpath Exception. The Starling C launcher is MIT.
  - Bedrock touch-button art is fetched from Mojang's samples repository and governed by Mojang's license/EULA; Starling ships fallback buttons of its own under MIT.
  - A large set of Horizon Android adaptations are explicitly identified: legacy GLES shader/render paths, diagnostics and performance recording, Android Vulkan/Adreno paths, Android Documents provider, touch controls/editor/keypad, provider-aware accounts/Ely.by, account wardrobe/skin upload, and HUD layout. The notice points to source files and upstream revision `fa548eb7abe91c194d3780e4580b9746323cd606` and states MIT attribution.
- Dependencies / cross-references: points to `third_party/wgpu-hal`, `third_party/naga`, `third_party/libmimalloc-sys`, `crates/horizon-client`, `crates/horizon-client-render`, `crates/horizon-optifine`, `crates/horizon-theme`, `crates/starling-account`, `launcher`, Android packaging Java/C sources, asset manifests, and preparation scripts. These are evidence of named locations, not yet a source-level dependency graph.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high. This is the clearest warning against treating the upstream repository as a single uniform MIT codebase. A port/reuse inventory must separate original MIT code, dual-licensed dependencies, GPL-with-exception runtimes, OFL fonts, CC0/MIT icon sets, trademark artwork, and Mojang/Create assets. It also shows how Starling deliberately downloads some proprietary/game-adjacent assets and runtimes to the device instead of bundling them.
- **Risks / limitations:** this document is a self-authored notice, not a substitute for validating each file, bundled license text, upstream commit, or redistribution terms. The notice records code adapted from Horizon Android; RustChunk should preserve attribution and inspect the relevant source history before reusing it. The Minecraft/Bedrock assets and Create assets must not be assumed redistributable with the app. GPLv2 + Classpath Exception for OpenJDK does not automatically license the Starling app itself, but the exact runtime distribution and build arrangement still require checking.
- Decision: **essential** as a compliance/attribution checklist; **adaptable** as a model for RustChunk's own third-party notices and on-device fetching policy. Individual libraries/features are not approved for reuse until their code and license files are checked.
- Follow-up checks: compare notices against `Cargo.toml`/`Cargo.lock` and actual vendored directories; inspect all referenced license files; trace the Create asset fetch; inspect JDK archive acquisition/checksum code; validate Mojang touch assets are never bundled; identify which Horizon Android adaptations exist in current Starling tree and their exact source diffs.

- 1.20.1 implications: license obligations do not change with Minecraft version. Any RustChunk reuse must trace the exact source file and dependency to its own license; Minecraft 1.20.1 and Mojang assets remain separately governed.
- RustChunk implementation mapping: Maps to RustChunk third-party provenance and packaging policy, alongside root `LICENSE`, dependency manifests and resource-download tooling. It is a compliance gate, not an engine module.
- Evidence required before adoption: Reconcile every declared component with actual source, lockfiles and bundled license text; verify remote asset/runtime acquisition, hashes, attribution and redistribution terms. The notice alone is insufficient evidence.
- Completion state: complete review of the notice document; individual licenses, code, and runtime distribution mechanisms remain open audits.

### 4. `CONTRIBUTING.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `b326f21b72a3fb5e6d10601b1ec0697070400c5d`
- Size: 48 lines; 5,087 characters; full file fetched and read.
- Coverage: complete; no chunks omitted.
- Purpose: defines the boundary between changes for Horizon (shared game logic) and Starling (Android-specific app/integration), developer setup, device-testing expectations, source/license policy, PR checks, and public-tree hygiene.
- **Key architecture/process rules:**
  - Shared gameplay, world generation, desktop-shared rendering and menus are directed upstream to Horizon and are expected to be verified against vanilla with a parity harness.
  - Android-only work is directed to Starling; named examples include APK packaging, Android entry point, touch controls, game-folder/import logic, and Android-only UI.
  - Build/install commands use the Python APK script; phone testing should name the actual device. A local `horizon-args.txt` can pass launch arguments/environment variables; `--control-socket` plus `HORIZON_CONTROL_PORT` exposes a control channel over ADB.
  - The guide imposes a project contribution policy: only a whitelist of permissive/public-domain licenses may be read/copied/adapted; several other license families and unclear/no-license projects are declared off limits even to read. It requires checking licenses at the exact commit and file headers, reporting provenance in PRs, retaining notices, and recording reuse in `THIRD_PARTY_NOTICES.md`.
  - Before PR, developers install Git hooks and run `tools/pr_ready.py`; this checks formatting, Clippy, relevant tests, Python tests and public-tree hygiene. Game workspace dependency scopes and history checks are described.
- Dependencies / cross-references: `README.md`, upstream Horizon contributing guide, `packaging/android/build_apk.py`, `packaging/lowbudget/make.py`, `hand.py`, `strings.json`, `pack/`, `crates/horizon-client/src/android_lib.rs`, touch/import modules, `horizon-ctl`, `tools/pr_ready.py`, `tools/check_public_tree.py`, `.githooks`, and `THIRD_PARTY_NOTICES.md`. These named paths need source inspection before relying on their behavior.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high for project boundaries and quality gates. A similar split could prevent Android-specific changes from contaminating a reusable Rust/game core. The public-tree check and explicit device-test reporting are valuable safeguards, especially because game assets/prepared resources should not enter version control.
- **Compatibility warning:** the stated build targets 26.3 and its NDK/SDK/JDK versions are not automatically transferable to a Rust-native 1.20.1 client. The license whitelist is Starling's own contributor policy, not a universal statement of copyright law; RustChunk should establish and document its own policy rather than silently assuming Starling's restrictions are legal requirements for every case.
- Decision: **adaptable** for core-vs-Android separation, device testing, PR gates, provenance records and public-tree protection. **Rewrite required** for toolchain/version policy and project-specific paths. The exact third-party policy is a candidate governance rule, not code to copy.
- Follow-up checks: inspect the named build and check scripts; inspect the repository's game/Android workspace boundaries; verify which tests and linters are actually run in CI; compare Starling's license policy with actual third-party notices for consistency.

- 1.20.1 implications: the contribution guide describes a 26.3-oriented toolchain and Horizon/Starling split. Its process ideas may transfer, but the target game version, Java resource preparation, build tools and RustChunk architecture must be specified independently for 1.20.1.
- RustChunk implementation mapping: Maps to RustChunk `CONTRIBUTING.md`, CI workflows, public-tree checks and patch-review rules. Its core/platform split can guide future engine crates; it is not a gameplay implementation.
- Evidence required before adoption: Require `cargo metadata`, formatting/lint/test results, public-tree scans, provenance/license checks and a successful ARM64 build; verify each workflow actually enforces the documented gates.
- Completion state: complete guide review; referenced scripts and source modules not yet validated.

### 5. `Cargo.toml`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `fb4a4a7d703c8ed35973c1690382d7ea2059d0eb`
- Size: 121 lines; 4,893 characters; full file fetched and read.
- Coverage: complete workspace manifest; no chunks omitted.
- Purpose: defines a large Rust workspace, shared package metadata, Clippy lint allowances, four local crates.io patches, and development/release/iteration build profiles.
- **Workspace structure observed:**
  - Edition 2024, declared workspace `rust-version = 1.85`, MIT package license, resolver 2.
  - Core/game-oriented members include `horizon-core`, world/protocol/items/commands, client/client-core/client-lib, split client input/render crates, Android NativeActivity crate, server, bots, friends/relay, asset preparation, shader/OptiFine compatibility, mods, account, theme, benchmark and release-signing crates.
  - `launcher`, telemetry server, mod registry, and vendored `wgpu-hal`, `naga`, `libmimalloc-sys`, and `winit` are explicitly excluded from the root workspace.
  - `horizon-android` is annotated as the game NativeActivity library and empty on non-Android targets; `horizon-mcp` is described as a shared Model Context Protocol component.
- **Pinned/local patches and rationale:** local `wgpu-hal` for Vulkan descriptor-pool/Windows driver, Metal HDR string handling and VSync-off fence behavior; local `winit` patch for Android raw input hook behavior; local `naga` patch for GLSL frontend shader-pack fixes; local `libmimalloc-sys` patch to avoid POPCNT on older MSVC CPUs. Comments say to drop patches when upstream releases contain fixes.
- **Build profile strategy:** dev builds retain line tables only; release uses thin LTO and one codegen unit; `fast` optimizes while linking faster; `quick` increases codegen units and disables LTO for iteration, with client/Android crates at opt-level 1.
- Dependencies / cross-references: direct workspace references to paths above, patch directories under `third_party`, README/CONTRIBUTING toolchain, `packaging/android/build_apk.py`, `crates/horizon-client/src/android_lib.rs`, and `THIRD_PARTY_NOTICES.md`. This manifest does not itself enumerate each crate's dependencies; per-crate manifests and Cargo.lock are needed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high for architecture mapping. It shows Starling is not just a thin Android launcher: most of the workspace is a Rust reimplementation of game/client/server systems, with Android as one target and separate input/render/core modules. The split crates, NativeActivity boundary, controlled local patches and fast/quick profiles are useful architectural/build ideas.
- **Major compatibility warning:** this structure is built around Horizon's 26.3 Rust game implementation. It is not a 1.20.1-ready workspace: game rules, registries, world data, protocol and renderer assumptions need a version-specific port. Forge/JVM support is deferred and is not a requirement for the current milestone. **Forge classification:** current conclusion: Forge/JVM support is deferred and is not a milestone requirement.
- **Notable configuration observation:** README/CONTRIBUTING demand Rust 1.90+, but this workspace manifest advertises `rust-version = 1.85`. That is not necessarily a build defect (toolchain may be pinned elsewhere or a dependency may need newer Rust), but it is a documentation/configuration discrepancy to resolve when the toolchain files are audited.
- Decision: **adaptable** for crate boundaries, Android-specific target isolation, local patch rationale tracking, and quick-vs-release profiles; **rewrite required** for version-specific game crates and preparation inputs; keep the crate-boundary pattern only where the 1.20.1 port validates it.
- Follow-up checks: inspect member crate manifests, Android NativeActivity source and JNI/NDK integration; verify `third_party` patch diffs and removal conditions; inspect toolchain files and CI to explain Rust 1.85 vs 1.90; map which crates are needed for Starling's phone app versus optional server/tools.

- 1.20.1 implications: workspace members, feature boundaries and dependency patches are built around the 26.3 implementation. The manifest is an architecture reference, not a 1.20.1-compatible workspace to copy wholesale; only crates proven version-neutral should be considered.
- RustChunk implementation mapping: Maps to the RustChunk Cargo workspace boundary. The current workspace has only `engine`; future native-game crates for core/world/protocol/render/input/preparation must be introduced deliberately rather than treating Starling's workspace as already present.
- Evidence required before adoption: Before adopting any crate boundary, inspect the actual dependency graph and feature tree, then pass host and `aarch64-linux-android` checks for the selected minimal workspace. Confirm no launcher/JVM dependency leaks into the native game core.
- Completion state: complete root manifest review; dependency graph and crate internals remain pending.

### 6. `Cargo.lock`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `5f5a3e785ef55cb1e4ae408b7d4a2a1368e07754`
- Size: 5,510 newline-terminated lines (5,511 entries when splitting on the final newline); 132,369 characters; 541 package records; 498 crates.io registry package source entries; no Git-source package entries; lock format version 4.
- Coverage: fetched and read sequentially in consecutive chunks: 1–400, 401–800, 801–1,200, 1,201–1,600, 1,601–2,000, 2,001–2,400, 2,401–2,800, 2,801–3,200, 3,201–3,600, 3,601–4,000, 4,001–4,400, 4,401–4,800, 4,801–5,200, and 5,201–5,510. The file ends with a newline. No intended ranges skipped.
- Purpose: Cargo's generated lockfile pins the exact resolved Rust dependency versions and checksums, plus the workspace package dependency edges. It is not intended for manual editing.
- **Project packages / dependency topology:** the file records Starling/Horizon packages including the Android NativeActivity crate, core/client/client-lib/client-core/client-input/client-render, world/protocol/items, rendering/shaders/OptiFine compatibility, account, asset preparation/catalog, mods, friends/relay, server and related tools. The locked workspace edges confirm that `horizon-client` and `horizon-client-lib` sit over a broad shared Rust game/client stack, while Android crate `horizon-android` depends on `horizon-client`, JNI 0.21.1 and winit.
- **Important ecosystem packages observed:** Android activity/NDK/JNI crates; winit 0.30.13; wgpu/wgpu-core/wgpu-hal/naga 30.0.1; audio via cpal/symphonia; input/controller/HID via gilrs/hidapi; networking/TLS via ureq/rustls; serialization via serde; crypto/auth primitives; ZIP/compression/image/font support; wasmparser and minisign verification for mods; Windows/macOS/Linux/Wayland/X11/Web dependency families included in the cross-platform resolution.
- **Cross references and patch relationship:** matches the root `Cargo.toml` workspace packages and local patches. The lockfile's `wgpu-hal`, `naga`, `libmimalloc-sys`, and `winit` path packages have no registry source/checksum entries, consistent with the root path patches. It also contains multiple versions of packages such as JNI, objc2, rustix, socket2 and platform libraries, which are separate resolved versions rather than necessarily conflicts.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high as a dependency inventory and reproducibility baseline. It identifies practical Rust-side building blocks for Android native entry, graphics, user input, audio, archive/resource parsing, networking, account/security logic, and mod verification. It also demonstrates that Starling's dependency footprint is a full game engine/runtime stack rather than a conventional launcher's small wrapper.
- **Limitations / risks:** a lockfile alone does not show which target-specific packages are compiled into a particular APK, enabled Cargo features, runtime behavior, binary size, license compatibility, or whether a dependency is actually required at runtime. Registry checksums establish package identity, not safety or suitability. Platform-specific entries should not be mistaken for Android shipping dependencies. The presence of mod-related crates does not imply Forge/FML support; the package graph shows Horizon's native/content mod infrastructure instead. **Forge classification:** current conclusion: the dependency graph does not establish Forge/FML support.
- Decision: **adaptable** as a shortlist/reference for specific Android Rust subsystems after reviewing manifests and licenses; **unnecessary** to copy wholesale. Every dependency must be justified against RustChunk's narrower Rust-native 1.20.1 objective.
- Follow-up checks: inspect per-crate manifests and Cargo features to identify the Android build graph; verify patched vendor crates and licenses; map which dependencies are needed for the 1.20.1 renderer/input/auth/archive pipeline; check minimum toolchain and MSRV;

- 1.20.1 implications: this lockfile pins the current Starling/Horizon dependency graph, not a validated RustChunk 1.20.1 graph. Re-resolve and test only the dependencies selected for RustChunk; do not copy this lockfile as a compatibility mechanism.

- Dependencies / cross-references: No separate dependency list is recorded in this entry.
- RustChunk implementation mapping: Use only as a dependency-selection reference for the future RustChunk workspace; do not copy Starling's lockfile. Current `engine/Cargo.toml` is a separate, small JNI/installer-oriented package.
- Evidence required before adoption: For each selected dependency, verify license, active feature path, transitive footprint, Android ARM64 compatibility and reproducible resolution. A successful lockfile parse is not proof that a dependency is needed or safe.
- Completion state: complete lockfile inventory review; feature activation, Android-specific resolution and runtime dependency necessity remain pending.

### 7. `.gitignore`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `29d00d0368c76edd3115dbca57ce0901e55860a9`
- Size: 42 lines; 760 characters; full file fetched and read.
- Coverage: complete.
- Purpose: excludes build output and caches (`.local/`, `target/`, Python caches/venvs); downloaded/decompiled game material and private profiling files (`*.jar`, `*.class`, JFR/heap profiles, decompiled folders, Minecraft/assets/run/logs/dist); runtime-generated game data (options, debug profiles, servers.dat, touch layout, resource packs, mod data, saves/backups/screenshots/crash reports); and release exporter output.
- **Cross references:** aligns with README/CONTRIBUTING claims that game files and prepared resources stay local and must not be committed; ignores the `.local` resource-preparation cache and `exporters` release-only bundle.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high for protecting user-owned Minecraft files, mod jars/classes, saves and private diagnostic data from accidental commits. It also separates reproducible source from generated outputs and runtime state.
- **Risks / limitations:** `.gitignore` does not untrack files already committed and is not a complete public-tree/security enforcement mechanism. Broad rules such as `*.jar` and `/assets/` can also hide legitimate project files; RustChunk should define narrow, reviewed exceptions rather than copy these patterns without checking its own folder layout. A separate CI/pre-commit scan is needed for robust protection.
- Decision: **adaptable** for generated/private-data protection, with paths and ignore breadth rewritten for RustChunk.
- Follow-up checks: inspect `tools/check_public_tree.py`, hooks and CI to see how they complement ignore rules; examine actual Starling asset/prepared-resource directories and packaging behavior.

- 1.20.1 implications: ignore rules are independent of protocol version, but must be adapted to RustChunk's own resource, save, mod and build paths. They do not prove that private Minecraft data is excluded from all commits; CI/public-tree checks are still needed.

- Dependencies / cross-references: No separate dependency list is recorded in this entry.
- RustChunk implementation mapping: Maps to RustChunk root `.gitignore` plus public-tree validation in CI. Adapt exclusions for private JARs, mappings, decompilation, generated facts, local worlds, logs and device profiles.
- Evidence required before adoption: Test ignore patterns against representative private inputs and verify the public-tree checker catches files that are force-added or placed outside ignored paths. Confirm no user worlds or Mojang files enter commits.
- Completion state: complete ignore-rule review; enforcement implementation remains pending.

### 8. `.github/workflows/android.yml`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `3638a951d8489e040d9a37d22dab86526a6bf4e8`
- Size: 191 lines; 9,829 characters; full workflow fetched and read.
- Coverage: complete, including both APK and release jobs.
- Purpose: GitHub Actions pipeline that builds four Android ABIs, produces preview/release APK artifacts and native symbol artifacts, and publishes a tagged release after all matrix builds succeed.
- **Triggers / targets:** pushes to `android`, `starling`, or `integration`; pull requests opened/synchronized/reopened; manual dispatch with `keep_symbols`, `release_key`, and optional native-mod IDs. Matrix covers ARM64, ARMv7, x86 and x86_64 Android targets.
- **Build environment and sequence:** Ubuntu latest; checkout; Python 3.12; Temurin JDK 25; Android build-tools 35.0.0, platform 36 and NDK 28.2.13676358; stable Rust with target matrix and cargo-ndk 3.5.4; per-ABI Rust cache; release/debug keystore handling; build via `packaging/android/build_apk.py`. Preview uses `quick` profile and `--preview`; release uses `fast`. Release build adjusts swap/worker count for memory limits.
- **Signing/release controls:** release signing is conditioned on official repository plus tag or explicit release-key dispatch and uses a protected `release` environment/secrets; ordinary builds use a debug key or throwaway fallback. APK artifacts have different retention (release-key test builds only one day; ordinary builds 90 days). The release job downloads all ABI artifacts, runs `release_assets.py`, publishes APKs and StarlingVisuals with release notes, then triggers native-mod builds.
- **Native-mod path:** dispatch input IDs are validated against a constrained name regex and an expected mod manifest path; `native_build.sh` generates native-mod APKs. The main job has a separate symbol upload, with note that native-mod builds do not produce the ordinary symbols library.
- **Permissions / concurrency:** top-level contents read; release job elevates contents write and actions write; PR concurrency is cancellable while branch/tag builds are not cancelled by a new run in the same way.
- Dependencies / cross-references: `packaging/android/build_apk.py`, `packaging/android/native_build.sh`, `packaging/android/release_assets.py`, release notes, native-mod paths, diagnostics docs, and game update offer implementation. Workflow also relies on GitHub secrets, environments/rulesets, Actions artifacts, gh CLI and pinned action major tags.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high as a CI/CD template: ABI matrix builds, separation of preview and release package identity/signatures, protected release secrets, symbol artifact retention, memory-aware build configuration, and post-matrix publishing are directly useful patterns for an Android Rust app.
- **Risks / limitations:** workflow results are not established by reading YAML; no run was executed or checked here. Several GitHub Actions are referenced by mutable major tags (e.g. `@v4`, `@v5`, `@v2`) rather than immutable commit SHAs, so supply-chain hardening could pin action commits. `ubuntu-latest` and stable Rust are moving targets; reproducibility depends on script/toolchain details. The documented four-ABI matrix may be overkill for RustChunk's specific devices and will increase build time. Do not copy release-secret assumptions without configuring environment protection.
- Decision: **adaptable** for matrix builds, preview/release separation, signing discipline, symbols, and release gating; **rewrite required** for RustChunk package IDs, supported ABIs, secrets, branch policy, toolchain versions, and Rust-native 1.20.1 packaging and resource preparation.
- Follow-up checks: inspect build/release scripts and actual CI history; verify signing behavior and package IDs; inspect environment protection and tag rulesets; assess whether RustChunk needs all four ABIs or only ARM64/ARMv7.

- 1.20.1 implications: the workflow builds the current Starling 26.3 app with its own Android SDK/NDK/JDK and ABI matrix. RustChunk needs its own 1.20.1 resource-preparation and runtime pipeline; copying this workflow would not supply a compatible game client.
- RustChunk implementation mapping: Maps to RustChunk Android CI/release workflows, especially `.github/workflows/android.yml` and the pinned Starling baseline workflow. Reuse matrix/signing/artifact ideas only after replacing Starling package and toolchain assumptions.
- Evidence required before adoption: Require a complete build for every declared RustChunk ABI, verify package identity/signature/symbol artifacts, and confirm release publication is gated on all required jobs and protected credentials.
- Completion state: complete workflow source review; execution and security settings not independently tested.

### 9. `packaging/android/build_apk.py`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: 3c215d5875d364e215ade3eeafbe55bd81238ab9
- Size: 381 lines; 17,919 characters; full file fetched and read.
- Coverage: lines 1–381, complete in one fetch (under the 400-line threshold).
- Purpose: main Python APK build/packaging orchestrator. It compiles the Rust Android NativeActivity library, builds small native Java/Javac launcher executables, prepares runtime assets, compiles Java Activities to DEX, links Android resources, assembles and aligns the APK, signs/verifies it, and optionally zips or installs it.
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
- Dependencies / cross-references: Android SDK tools (aapt2, d8, zipalign, apksigner, adb); NDK clang/llvm-strip/C++ runtime; Rust/Cargo/cargo-ndk; JDK javac/keytool; AndroidManifest.xml, Java Activity sources, native/java_launcher.c, tools/build_android_exporters.py, research-lock.json, exporter Java source and bundle, fonts/theme crate, icons, lowbudget pack, shaderpack, diagnostics docs, and Git metadata. The script's referenced helper/source files require their own audit.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: extremely high. It provides the concrete bridge between a Rust Android native library and a distributable Android app, including ABI mapping, SDK/NDK discovery, 16-KB page alignment, build-ID symbols, DEX integration, application variants, asset bundling, APK alignment and signing. Its approach demonstrates how a Rust game core can be hosted in Android's NativeActivity while Java handles Android system integration.
- **Major target mismatch:** this pipeline packages a native Horizon 26.3 implementation plus Java preparation tools; it does not package a RustChunk 1.20.1 game core. The NativeActivity/Rust library, ABI selection, APK alignment and signing pattern are relevant, but the preparation/export pipeline and version-specific resources must be rebuilt for 1.20.1. The bundled Java launcher executables are preparation helpers, not proof of a JVM Minecraft runtime.
- **Potential issues / items to verify:**
  - JDK/NDK/SDK versions and all package constants are Starling-specific; README/CONTRIBUTING also mention Rust 1.90 although Cargo.toml sets MSRV 1.85.
  - link_resources() performs a literal manifest string replacement for non-ARM64 Vulkan requirement; verify against AndroidManifest.xml and test each ABI.
  - assets() includes exporter inputs/bytecode but README says Minecraft jars/assets are not packaged; inspect generated contents and exporter bundle to confirm there is no player-owned game code/data leakage.
  - sign()'s release/debug behavior depends on environment variables and CI protection; no build/sign command was executed in this audit.
  - zip_folder() fixes timestamps and sorts files for repeatability, but archive reproducibility should still be tested across host platforms.
  - --dev and --preview are not declared mutually exclusive; current precedence chooses dev if both are supplied. --library-only stops before APK packaging as designed.
  - Script assumes all expected source/resource files exist; its missing SDK checks are explicit, but some later missing inputs surface as lower-level exceptions.
- Decision: **adaptable** as an APK orchestration pattern; **rewrite required** for RustChunk's 1.20.1 game-data preparation, engine integration, package identity and supported ABI set. Keep asset/legal boundaries and symbol/signing ideas.
- Follow-up checks: inspect native/java_launcher.c, Android manifest and Activities, exporter helper and manifests, install/update code, and release assets; verify all ABI outputs and exact Android package identity; map required source-owned 1.20.1 data, registry and resource inputs for the Rust-native engine.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to RustChunk's Android packaging/build pipeline (current Gradle app plus `engine` native library and CI workflows). The Starling Python orchestrator is a reference, not an existing RustChunk build script.
- Evidence required before adoption: Require a clean reproducible ARM64 APK build, package/ABI/native-library inspection, install and launch on the target Android version, plus a later test proving the packaged binary is the 1.20.1 engine rather than a launcher/bootstrap.
- Completion state: complete script review; helper scripts, referenced sources and actual build results remain pending.

### 10. `packaging/lowbudget/make.py`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: f226619fcac0e0d1d61e75273d29e0f5772bc36d
- Size: 257 lines; 12,852 characters; full file fetched and read.
- Coverage: complete in one fetch.
- Purpose: generates Starling's temporary low-budget setup resource pack with Pillow. It supplies the app's setup/menu appearance before the user's own game files and resources are prepared.
- **Generated assets and mechanics:**
  - Chalk/graphite aesthetic with fixed palette, 4x texture scaling, multiple button takes, hand-drawn edges, grain, scribble fills, pressed/disabled variants, and nine-slice metadata.
  - Generates button, slider, slider-handle, text-field and scroller textures; tiled paper-like menu backgrounds; header/footer separators.
  - Generates five hand-lettered bitmap font sheets using hand.py, a default Minecraft font JSON plus four Starling font variants, and Starling variant metadata.
  - Imports local strings.json to create the en_us language file; writes a pack.mcmeta with pack_format 97 and Starling skin metadata.
  - Deletes and recreates the output pack directory on each run, then emits the output path.
- Dependencies / cross-references: Python standard library (json/math/random/shutil/pathlib), Pillow image APIs, local hand.py and strings.json; output path packaging/lowbudget/pack. build_apk.py packages the generated folder as lowbudget.zip.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: low for game performance/runtime, but useful as an example of generating a small bootstrapping UI/resource theme without shipping Minecraft assets. A separate early-start UI theme can help make setup usable before full game resources exist.
- **Version/compatibility warning:** generated filenames, namespace, language keys, font provider schema, pack metadata and resource-pack formats are tailored to Starling's 26.3 target. They cannot be assumed compatible with the 1.20.1 resource-pack format or the 1.20.1 pack-loading lifecycle without adapting and testing.
- **Potential issues / verification points:** this script was inspected but not run; the generated pack was not validated in-game. It relies on Pillow and imported hand.py behavior, neither of which is part of this file's implementation. Image-noise determinism should be tested rather than inferred from seeding Python's random module. Rebuilding destroys the previous generated output directory, as expected for a derived artifact but worth guarding if RustChunk's output path could contain user data.
- Decision: **unnecessary** for the core Rust-native 1.20.1 client; **adaptable** only if RustChunk needs a pre-game setup theme or generated placeholder UI pack. Rewrite pack schema and resource identifiers for Minecraft 1.20.1.
- Follow-up checks: inspect hand.py and strings.json if setup theme is in scope; inspect pack output and test its compatibility with Minecraft 1.20.1; verify build_apk.py's ZIP integration.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: No first-release runtime mapping. At most this can inform an optional pre-game setup/placeholder UI asset generator under RustChunk packaging; it must not be confused with Minecraft 1.20.1 resources.
- Evidence required before adoption: Only adopt after a setup-theme requirement exists; generate twice and compare outputs, validate the pack schema/resource identifiers against 1.20.1 and confirm no Mojang or other restricted assets are bundled.
- Completion state: complete generator source review; output generation and runtime visual behavior untested.

### 11. `research-lock.json`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: 719c266401fe962746965fb33c07abbb91f8a656
- Size: 70 lines; complete JSON document fetched and read.
- Coverage: complete; no ranges omitted.
- Purpose: machine-readable pin set for the exact Minecraft research/preparation inputs and supporting analysis tools/runtime artifacts.
- **Pinned data:**
  - Minecraft version 26.3; primary and accepted republished version-manifest SHA-1 values; accepted asset-index SHA-1 values; note explaining that certain republished JSONs differ only in asset-index selection; client and server JAR SHA-1 values; Java major 25.
  - Vineflower 1.12.0 release URL and SHA-256.
  - rea-agents package version 6.4.0.
  - jadx-headless-mcp 0.7.1 release URL and SHA-256.
  - Mojang Java runtime manifests/checksums used by tools/datafix_worlds.py for old server jars/sample worlds: Linux legacy/alpha/beta/gamma and macOS ARM64 gamma, with stated fallback to gamma when a component is absent.
- Dependencies / cross-references: build_apk.py places this file in the phone-side preparation kit; README describes prepared resources checked against these pins; exporter/build tooling consumes it; datafix_worlds.py uses the listed runtimes for sample datafix worlds. Actual consumers and validation logic still need inspection.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high as a pattern for separating mutable external metadata from source code and pinning downloaded inputs by cryptographic digest. RustChunk needs a comparable manifest for Minecraft 1.20.1 client artifacts, required native libraries and the tools used to construct a reproducible profile.
- **Version/compatibility warning:** every Minecraft checksum here is for 26.3, not 1.20.1; these values are unusable as RustChunk's source-of-truth for 1.20.1. The listed JDK major 25 is also Starling's exporter/preparation requirement, not evidence that a 1.20.1 game runtime should run on Java 25; this JDK is used by Starling's preparation/export tooling, and the Rust-native RustChunk target has a separate toolchain/runtime boundary.
- **Security / reliability notes:** checksums let consumers detect unexpected content changes only if the expected digest itself is obtained/trusted. SHA-1 remains in Mojang's lock fields and is weaker than SHA-256 for collision resistance; use SHA-256 where the upstream metadata provides it, and preserve the official manifest chain where it does not. Release URLs plus hashes are more robust than URLs alone. This document contains no actual game binaries/assets or license grants.
- Decision: **essential** as a concept for external-input pinning and deterministic preparation; **rewrite required** for RustChunk's 1.20.1 source artifacts, preparation tools and supported platforms.
- Follow-up checks: inspect every reader of research-lock.json, verify how the hashes are checked and how republished manifests are handled; map Mojang metadata and exact 1.20.1 client/resource dependencies; inspect input verification and cache lifecycle.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to RustChunk's reviewed `tools/minecraft_1_20_1_lock.json` and the preparation/download verification path. It provides a model for pinning external artifacts, not a file to copy unchanged.
- Evidence required before adoption: Verify every URL, version field and digest against official 1.20.1 metadata; test rejection of altered JARs/mappings, stale pins, path traversal and incomplete downloads before any artifact is accepted.
- Completion state: complete lockfile-document review; verification consumers and exact 1.20.1 pins remain pending.

### 12. `roadmap/starling.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: a3742a8f994dd266355428b1621747f642d36e76
- Size: 188 lines; 12,976 characters; full file fetched and read.
- Coverage: complete in one fetch.
- Purpose: product roadmap with Now/Next/Later priorities and technical status notes, explicitly separating Starling Android work from Horizon's shared-game roadmap.
- **Capabilities described as done or substantially done:** phone-side download/preparation from Mojang's 26.3 files with fetched OpenJDK 25 and compiled exporters; keyboard/IME bridge; release update prompt; Bedrock touch controls fetched by pinned revision/hash; save-on-background and recovery/log handling; offline identity/skins; shader-pack translation and phone-specific shader work; plugin server dialogs, resource packs, custom armour/models/chat actions/movement modes; phone-aware settings.
- **Explicitly unfinished/next/later items:** shader-pack options screen, signed/secure chat for servers requiring it, Android physical-controller backend, Open-to-LAN/invites on Android, keeping up with Horizon, more game features exposed to mods, Horizon native mixins/rebuild on device, phone mod browser, and Android Discord Rich Presence.
- **Architecture / implementation clues:** native Horizon shader translator (GLSL → WGSL at load), background compilation/loading while the world renders without pack for a short time, experimental depth prepass, reduced render resolution, per-server movement profiles, Android keyboard text mirror, on-device fetched assets, update install through Android package installer, native mod rebuild tied to a particular Horizon revision.
- Dependencies / cross-references: Horizon roadmap; account/resource preparation crate; packaging/lowbudget and shader pack; renderer/shader crates; bedrock_touch.rs and its manifest; NativeActivity/IME bridge; player and graphics config files; update_offer implementation; server/plugin protocol handlers; mod-porting doc and horizon-mixins. These implementation claims require code verification.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high for a realistic feature inventory and separating platform-specific gaps from shared-core maturity. It identifies concrete mobile problems beyond simply compiling Rust: first-run resource provisioning, lifecycle/save behavior, IME, touch controls, GPU constraints, background pack compilation, signatures/updates, and plugin server behavior.
- **Critical documentation inconsistencies to investigate:** README lists physical controllers as 100% and says servers include secure chat with a signed-in account; this roadmap still lists Android physical controllers and secure chat as Next. README also describes native mods as 95%/compiled on-device while the roadmap places Horizon native mods under Later. Treat these statuses as unresolved until source and tests clarify scope or document dates/revisions.
- **Version/compatibility warning:** roadmap behavior is for Minecraft Java 26.3 through Horizon. Protocol packets, resource-pack schemas, chat signing rules, renderer assumptions, and mod formats differ across versions. The listed Horizon native mods are source mixins against a Horizon revision—not Java Forge mods and not binary compatibility with 1.20.1. **Forge classification:** historical comparison: Horizon-native mods are explicitly distinguished from Java Forge mods.
- Decision: **adaptable** for mobile feature sequencing and clear Done/Left records; **rewrite required** for the 1.20.1 feature plan and integration assumptions. The roadmap is a planning source, not implementation proof.
- Follow-up checks: compare roadmap claims with README and code; inspect the account, update, IME, touch, server protocol, shader pipeline, native-mod build flow and test reports; establish exact feature status for the RustChunk hardware and Rust-native 1.20.1 target.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to RustChunk `README.md` target order and `docs/STARLING_1_20_1_ENGINE_MAP.md` implementation gates. Use it as planning methodology only; do not copy Starling's 26.3 priorities.
- Evidence required before adoption: Every milestone must name a concrete artifact, reproducible test and pass condition. Verify status against CI/device evidence; no feature may be marked done from roadmap wording alone.
- Completion state: complete roadmap-document review; documented statuses and claims are not yet source-verified.

### 13. `docs/horizon-parity.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: 4783c14f65966eb407b51f89cea5c847c9758bf7
- Size: 39 lines; 5,249 characters; full file fetched and read.
- Coverage: complete.
- Purpose: describes Horizon's own Java-vanilla parity audit as of the revision Starling uses. It explicitly says the audit is measured by desktop harnesses, not Android, and that 100% means only 100% of the behavior currently compared by those harnesses.
- **Audit details stated:** subsystem scores are 0/0.25/0.5/0.75/1, with 1 meaning complete and verified against vanilla by differential tests. The page says it was last audited 2026-10-13; 281 harnesses pass on Linux, and the Mac mini runs them too. Categories include input/game loop, assets/resources, protocol/multiplayer, chunk storage/palettes, meshing, rendering, lighting, world generation, saves/region files, entities/physics, interaction/inventory, UI/chat, audio, and block ticking/behaviors. Large scenario counts are documented for many subsystems.
- **Specific constraints / known gaps:** multiplayer parity evidence includes offline-mode only; the page states Horizon itself cannot yet sign in to Microsoft and therefore cannot join online-mode servers; HTTPS skins may show defaults unless cached; pack icons, creative hotbar C/X wiring, and item tooltip attribute lines are listed as remaining differences.
- Dependencies / cross-references: README Android status and Horizon README/project status; tests/test_parity.py; vanilla differential harnesses; game-specific packet and rendering/test sources, none of which are inspected in this file.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high as a model for differential testing against the exact vanilla version. This is a strong idea for 1.20.1: compare packets, NBT/world data, recipes, inventory interactions, chunk generation, entities and UI outputs against vanilla 1.20.1 fixtures rather than relying only on launch success or FPS.
- **Critical caveats:** 100% is a scoped test metric, not total parity and not Android validation. The test date and current status need rechecking. The parity target is the Horizon version based on Minecraft 26.3, not Minecraft 1.20.1. The page's Microsoft-auth statement conflicts with README's Starling Android sign-in claims; likely app-specific integration has moved beyond Horizon core, but that explanation is not proven here and must be checked in source. README's 100% server row also should not be read as online-mode parity.
- Decision: **essential** as a testing philosophy and differential-harness model; the actual 26.3 test suite is **rewrite required** for the Rust-native 1.20.1 target.
- Follow-up checks: inspect parity harness definitions, CI invocation and latest results; locate how Starling adds Microsoft sign-in around Horizon; inspect the implementation of the listed gaps; define RustChunk's version-pinned golden fixtures and offline/online test matrix.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to a future RustChunk 1.20.1 parity specification and dedicated Java-oracle/Rust-probe tests. Current 1.20.1 exports and integration tests are partial inputs, not a complete parity suite.
- Evidence required before adoption: Require version-pinned vanilla 1.20.1 oracle outputs, deterministic fixtures and subsystem coverage; demonstrate that tests fail on a deliberate mismatch and that all reported passes correspond to executed CI runs.
- Completion state: complete parity-document review; harness claims and implementation not independently executed or validated.

### 14. `docs/android-armv7.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: 17cfb5fe88d6dceff4a5df1a73ba83f17ef06aa4
- Size: 111 lines; 6,621 characters; full file fetched and read.
- Coverage: complete.
- Purpose: documents ABI/build limitations, GPU fallbacks, ARM32 memory constraints, resource preparation, updater selection and native-mod ABI restrictions.
- **ABI/build findings:** ARM64, ARMv7, x86 and x86_64 are separate APK builds; release waits for all four. Android x86 has no supplied Android Java 25 runtime and therefore imports prepared .starling resources; x86/x86_64 are not physically tested. ARMv7 requires Android 8+ and uses a 32-bit launcher/runtime. The docs emphasize that CPU ABI support does not guarantee GPU-driver compatibility.
- **Graphics findings:** GLES-only shader/texture transformations are adopted from Horizon Android while Vulkan WGSL is stated unchanged; a BASE_VERTEX fallback rebases vertex buffer bindings; terrain textures are split into at most eight arrays and packs exceeding that are rejected; small GPU atlas limits use skyline packing while prioritizing GUI/fonts/dynamic slots; OpenGL panoramas use 256px box-filtered faces while Vulkan keeps full resolution. Block-sprite budget is 256 MiB by default, but does not cap total memory. ARM32 GPU arena growth is normally limited to 8 MiB steps.
- **Runtime/resource findings:** updater chooses an APK matching the process ABI; native mod binaries must match ABI and build/prebuilt native mods are ARM64-only. ARM32 preparation runs sequentially and caps each Java heap at 512 MiB, or 384 MiB below 1.5 GiB total RAM/when RAM cannot be read; the cap does not bound total process memory. APK includes compiled original exporter classes but not Minecraft jars/libraries/assets; host JDK 25 compiles and checks exporter classes, and pinned Minecraft inputs stay in ignored cache. ARM32 phone runtime lacks jdk.compiler, so it executes the shipped classes without javac. Android ARM32 runtime is pinned to EclipseJRE Java 25.0.4 after a reported floating-point issue in an earlier runtime.
- Dependencies / cross-references: build_apk.py, native launcher, Rust targets/NDK, renderer backends and shaders, texture/atlas allocators, updater ABI selection, native-mod service, preparation worker lanes, research-lock.json, exporter bundle/cache, diagnostics and PR #272.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high for choosing a supported ABI strategy, preventing bad-GPU assumptions, memory budgeting, renderer fallbacks, process-matched APK updates, and deciding where native mods may run. The texture/atlas/arena limits are particularly useful as concrete examples of device-aware rendering safeguards.
- **Compatibility / applicability:** much of the graphics code is forked from Horizon Android and targets Horizon's 26.3 render stack; it cannot be lifted directly into the RustChunk 1.20.1 renderer without matching renderer internals. ARM32 Java 25 exporter preparation is a Starling data-preparation detail, not a game-runtime requirement for RustChunk's Rust-native target. ABI/build patterns are adaptable, not version-independent implementation.
- **Limitations / risks:** this is documentation; claims about rendering fallbacks, runtime FP correctness, heap limits, and byte-for-byte preparation output require source/test verification. Per-ABI release failure blocking is a deliberate quality/release policy but could slow RustChunk releases if unsupported ABIs are included.
- Decision: **adaptable** for ABI-aware packaging/update selection, low-memory controls, GPU capability checks and resource budgets; **rewrite required** for RustChunk's 1.20.1 resource preparation and renderer integration.
- Follow-up checks: inspect actual shader/backend fallback code, heap/worker limits, updater ABI selection, Java runtime checksums, native-mod ABI gating and build matrix; test on real Mali/Adreno devices and each ABI actually supported.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to RustChunk Android ABI policy and packaging configuration. The current target is ARM64 Galaxy A22; ARMv7 support is optional and should not enter the first release merely because Starling documents it.
- Evidence required before adoption: Adopt ARMv7 only after selecting supported physical devices, building and installing the ABI-specific APK, verifying native dependencies and measuring memory/performance. Otherwise explicitly document ARM64-only support.
- Completion state: complete architecture-doc review; implementation and device tests not independently verified.

### 15. `docs/android-controls.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: bde1e376a8910d910a93566c492fca1413f5eaec
- Size: 79 lines; 4,554 characters; full file fetched and read.
- Coverage: complete in one fetch.
- Purpose: documents Starling's Android integration of Horizon Android's touch-control engine and visual layout editor, plus keyboard, pointer, physical input and IME behavior.
- **Control features:** MCPE and GoyControl layouts/presets; move/resize controls; independent dimensions, colors, borders, shapes, labels and event bindings; undo/redo, visibility and save confirmation; joystick/aim/sensitivity/gyro/pointer/button/inventory options. GoyControl supports static/pass-through/swipe/hold/toggle controls and up to three simultaneous events; MCPE has joystick/crosshair, joystick/tap and D-pad schemes.
- **Persistence and input details:** presets live under controls/, active choice in horizon-touch.json; physical D-pad/keypad profile supports rebinding and is persisted in horizon-keypad.json; Input device chooses Auto, Touch, Keyboard & Mouse or Controller, with Auto tracking the last-used device. Virtual Mouse switches between direct touch and relative pointer. Account selection is independent of control preset. Existing touch-layout.json is preserved but not automatically imported into the new preset format.
- **Touch/IME/lifecycle details:** 360-degree movement is optional/off by default; Look while interacting covers attack and use/place; vertical slider gestures scroll while horizontal drags adjust values; chat history/completion/cursor/paste and touch-slot gestures remain; foreground/focus changes release touch owners. Text fields use a shared focus/IME route; TextMirror adapters synchronize text/selection, while other widgets report focus bounds and use ordinary text events. Custom mod UIs must expose focus and handle text events; simply drawing a field does not make it functional. Focus reports expire per frame; explicit taps reopen keyboards hidden by Back. Server dialogs own Back, even when they disallow closing.
- **Asset integrity:** Bedrock button art is retrieved from a pinned manifest with size and SHA-256 checks and layered over prepared resource archives; legal provenance is separately described in THIRD_PARTY_NOTICES.md.
- Dependencies / cross-references: controls/ preset data, horizon-touch.json, horizon-keypad.json, touch-layout.json legacy data, Bedrock resource manifest, TextMirror, preedit focus reporting, ordinary text event handlers, game input/focus release, and THIRD_PARTY_NOTICES.md.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high for mobile input design: editable controls, separate preset persistence, input-mode arbitration, touch ownership cleanup on lifecycle changes, explicit widget focus/IME contract, pointer mode, and cryptographically verified artwork. Particularly useful for reducing UI/input coupling and making custom screen text fields behave correctly.
- **Version/compatibility limitations:** the control behavior is adapted from Horizon Android and integrates with Horizon's renderer/game input. It is not RustChunk's 1.20.1 input implementation; its actions need an adapter into the Rust-native game loop and the target version's player interaction semantics. Bedrock resource assets and schemas are not automatically reusable for RustChunk.
- **Potential inconsistency:** this document describes Controller as an input-device option, while the roadmap still lists physical controllers on Android as Next and README reports controllers as complete. It may distinguish a UI option from a working Android backend; source inspection is required before claiming controller support.
- Decision: **adaptable** for input architecture, editable layouts, lifecycle handling and IME focus contracts; **rewrite required** for Rust-native Minecraft input integration and version-specific UI hooks.
- Follow-up checks: inspect touch engine/editor/preset persistence, Android input event routing, IME/TextMirror/focus APIs, Bedrock touch manifest verification, and whether the Android controller backend actually works.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to the future Android input/touch adapter and the app's control-layout UI; keep input actions separate from Minecraft gameplay. Existing Kotlin UI is not proof that native game touch actions work.
- Evidence required before adoption: Require pointer-ownership/cancel/rotation/focus tests, GUI-scale checks and parity tests that map each touch action to vanilla 1.20.1 input/game-mode behavior on the target device.
- Completion state: complete document review; underlying implementation and physical-device behavior remain unverified.

### 16. `docs/android-accounts.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: 253bcebee21e4962b4f317f6a6d1c61d5078e9b9
- Size: 76 lines; 4,481 characters; full file fetched and read.
- Coverage: complete in one fetch.
- Purpose: documents Android account/profile selection, Microsoft device-code authorization, Ely.by credentials/2FA, offline profiles, secure token storage, profile-specific skins and first-run account setup.
- **Provider semantics:** Microsoft device-code flow requires Java entitlement for authenticated play; Ely.by supports username/email, password and optional 2FA but authenticates only to Ely.by-configured servers and does not confer Microsoft ownership; offline names are 1–16 ASCII letters/digits/underscores and work only where offline players are allowed. Profile switching is persisted; failed or cancelled auth should not replace a later selection.
- **Persistence/security details:** provider/name/UUID/selection metadata in private accounts/accounts.json; refresh/access tokens in Android Keystore-backed sealed storage; passwords and 2FA codes are not saved. Legacy Microsoft profiles remain readable. Equal UUIDs across providers get separate secret keys/status caches. Desktop without platform keystore keeps secrets in memory for the run.
- **Skin behavior:** per-profile skins in horizon-skins.json; fallback legacy selection in horizon-player.txt; supports 64x64/legacy 64x32 PNGs; Microsoft upload posts the chosen skin to Minecraft profile endpoint and strips ancillary PNG chunks by rewriting the image; Ely.by skin is public API/read-only and opens their website for changes; offline skin stays local. Capes not supported.
- **Setup flow:** resource-independent UI allows Ely.by/offline/Microsoft selection and prepared-file import before full game assets are installed; authentication and profile selection precede continued resource download/preparation.
- Dependencies / cross-references: Microsoft device-code OAuth and Minecraft entitlement APIs, Ely.by authentication/session/skin APIs, Android Keystore, private accounts directory, account secrets store, skin files and player config, installer/download path, resource-independent setup UI, THIRD_PARTY_NOTICES.md and Horizon Android provider model.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high because account/profile semantics are central to a launcher and user has offline-profile requirements. Useful safeguards include provider-separated UUID/secret state, no persistence of passwords/2FA, protected token storage, cancellation race handling, profile-specific skins, and keeping offline play distinct from ownership/authenticated online play.
- **Compatibility / safety caveats:** this document describes Starling's custom integration. vanilla 1.20.1 online-mode servers and Microsoft authentication have distinct session/entitlement requirements; Ely.by/offline identity should not be presented as Microsoft authentication or a way to bypass ownership. Microsoft entitlement and online-mode server behavior must be implemented and tested independently.
- Decision: **essential** as a set of account-security and profile-management requirements; **adaptable** as architecture, but **rewrite required** for RustChunk APIs, Rust-native profile/authentication and Android account integration, storage encryption and provider support.
- Follow-up checks: inspect account manager, device-code flow, Minecraft entitlement/session join, Ely.by auth/session, Keystore bridge, token rotation/revocation, account selection/cancel races, PNG normalization and setup flow; confirm what offline mode means for target servers.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to a future RustChunk account/profile/authentication boundary and Android account UI, not to the current JNI workspace or a Forge feature. Keep offline profile behavior separate from online session authentication.
- Evidence required before adoption: Verify online authentication and server-join tokens against the 1.20.1 flow, test offline/local-world behavior without misrepresenting it as authenticated online access, and review secret storage, log redaction and account switching.
- Completion state: complete account documentation review; implementation and security properties not yet independently verified.

### 17. `docs/android-diagnostics.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: f4d250d2449634f2f6ef8125ebdc2a6a4b021365
- Size: 97 lines; 5,780 characters; full file fetched and read.
- Coverage: complete in one fetch.
- Purpose: documents local performance/crash diagnostics, safe explicit export/share, startup recovery UI, Android exit history, symbol matching, bounded storage and transactional resource installation.
- **Performance recording:** opt-in one-second summaries of frame/CPU/GPU durations, phases, slow frames, worker stages, memory, thermal state, streaming queues, rendering counters and GPU adapter. Detailed GPU pass profiling reports p50/p95/p99/max from a rolling 15-sample window; timestamps/readback are asynchronous and unavailable data is not treated as zero.
- **Diagnostics export/privacy:** ZIP contains bounded log/crash tails, checkpoints, Android exit history and retained performance reports. Sharing grants access only to the selected ZIP; no automatic upload. Account files/worlds/config are excluded and credential/address patterns are redacted, but users must review archives because arbitrary mod/plugin log text cannot be fully scrubbed.
- **Crash/recovery:** startup checkpoint tracks launch/menu/world/in-world phases; missing clean exit is not proof of crash. On Android 11+, ApplicationExitInfo can classify exits; up to eight system traces of 1 MiB each are stored privately and only bounded crashed-thread summaries enter ZIPs. A separate renderer-independent diagnostics Activity can open before game resources/library; native export loads library lazily and degrades gracefully if it cannot load.
- **Native symbol tracing:** BuildId embedded in native game library; CI stores unstripped per-ABI library artifacts for 90 days; symbols can be matched to tombstone pc addresses using llvm-readelf/addr2line. Release libraries without line tables resolve functions but not source lines.
- **Resource bounds:** performance JSONL <=8 MiB/30 minutes, three retained reports, writer queue eight samples; diagnostic ZIP <=4 MiB/32 files/256 KiB per ordinary input, three ZIPs retained; export retains complete header/trailing complete rows and marks truncation.
- **Resource installation safety:** exclusive kernel lock serializes preparation; installer validates prepared schema/tables/assets, keeps prior resource folder until publication and restores on failure; retry resets loader generation. Documentation says resource preparation uses on-device Java exporters rather than old pre-generated tables; download concurrency does not eliminate exporter cost.
- Dependencies / cross-references: logger/panic reporter, performance recorder and GPU timers, Android ApplicationExitInfo, tombstone parser, BuildId link flags, CI symbol artifacts, diagnostics Activity/launcher shortcut, resource preparation lock, installer publication/rollback and tests in tools/test_android_diagnostics.py.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: extremely high for a mobile client expected to operate under constrained memory/GPU conditions and with many mods. Bounded local logs, opt-in sampling, renderer-independent recovery, symbol artifacts, explicit share permissions, privacy controls and transactional resource installation are practical reliability features. The p50/p95/p99 stage measurements can help distinguish CPU, GPU, streaming and memory bottlenecks.
- **Risks / limitations:** diagnostics can themselves consume resources; GPU profiling overhead is explicitly noted. Redaction is not guaranteed for arbitrary third-party logs. The described host test command and device build were not run here; this is documentation review only. Some diagnostic implementation is Starling-specific and will need adaptation to RustChunk's Android lifecycle, renderer and Rust game loop.
- Decision: **essential** as reliability, observability and privacy requirements; **adaptable** for data model, retention limits and recovery workflow; **rewrite required** for RustChunk crash capture and Android integration.
- Follow-up checks: inspect test_android_diagnostics.py and production writers/exporter/tombstone parser; inspect BuildId and CI artifact retention; inspect resource lock, validation and rollback; verify Android 11+ behavior and resource limits on low-RAM devices.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to RustChunk diagnostics, crash reporting and Android log collection. The current launcher status messages are not a complete game-engine diagnostics system.
- Evidence required before adoption: Require crash/restart/lifecycle tests, bounded log storage, secret/token/path redaction and privacy review; confirm Android diagnostics can be retrieved without exposing user worlds or credentials.
- Completion state: complete diagnostics-document review; implementation and tests not independently verified.

### 18. `docs/native-mods.md`
- Source repository/ref: `starling-client/starling` / `starling`.
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
  - The documented hooks cover server and client tick paths; input, GUI, screens, keyboard/IME, player actions, authentication-adjacent profile/game events, entities, blocks, fluids, containers, recipes, loot, advancements, commands, world lifecycle, resource reload, particles, sounds, camera, shaders, render passes, and dedicated-server helpers. This is a broad reimplementation of many vanilla/NeoForge-like extension points inside Horizon, not actual NeoForge/FML event buses. **Forge classification:** historical architecture comparison: NeoForge-like hook semantics are implemented inside Horizon, not through Forge/FML.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high as a study of extension-system design, especially version-pinned hooks, stable contracts vs raw injections, Rust trait API dispatch, build cache keys, reproducible toolchain acquisition, mod provenance, explicit consent, content/code separation, server/client side boundaries, and a potentially extensive gameplay API. Individual mobile features (input/render hooks, resource reload, bounded APIs, lifecycle and diagnostics) are useful design references.
- Scope assessment: Starling's native mods are compiled Rust code injected into a Rust rewrite of Minecraft 26.3. They are not Java bytecode mods or a generic JVM compatibility layer. Their NeoForge-like hook semantics are implemented by Horizon and must be ported/versioned before they can be used with a Rust-native 1.20.1 core. They are not required to reach the first vanilla 1.20.1 milestone; a native extension API can be reconsidered after the core client works. **Forge classification:** historical architecture comparison: NeoForge-like hook semantics are implemented inside Horizon, not through Forge/FML.
- Further risks / limits:
  - The build process described here targets desktop host toolchains; the documented workflow does not establish on-device Android compilation or compatibility with Galaxy A22/Mali-G52 resource constraints. It is resource-intensive (large source archive, Rust toolchain, many Cargo crates and multi-GB build cache).
  - Rust mod code and post-build tools execute unsandboxed; a package signature/checksum alone only proves identity/integrity, not safety.
  - API catalog entries have caveats and missing paths; many hooks are expensive per tick/frame and documentation explicitly says to keep handlers cheap. Some APIs are pending/draft and some game behavior is not fully parity-tested.
  - A single Hooks object serves both window and server-thread games and hooks may be reentrant; shared state requires synchronization. The guide also documents data-load hooks on arbitrary loading threads. This is a concurrency requirement, not a guarantee that careless mods are safe.
  - A mod may be client-only when joining another server; server-side hooks require the host/server to run the mod. Dedicated server exports are separate host builds and have no compatibility handshake yet (per doc).
- Decision: **unnecessary** for the first vanilla 1.20.1 milestone; **adaptable later** for extension API governance, versioned hook contracts, build reproducibility/cache design, code/content separation, opt-in trust and diagnostics practices. If the goal changes to a fully native Rust Minecraft fork with its own non-Forge mod ecosystem, this document becomes a major architectural reference, but that is a different product target. **Forge classification:** historical architecture comparison: NeoForge-like hook semantics are implemented inside Horizon, not through Forge/FML.
- Follow-up checks: inspect native-hooks.json and hook generator; inspect horizon-mixins security notes and manifest validator; trace Rust toolchain/source downloads and hash verification; inspect staging/cache/build scripts and native post-build runner; compare docs with real code and tests; verify which features are Android-specific vs desktop; defer native mod API design until the vanilla 1.20.1 core and stable public abstractions are implemented.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: No first-release implementation mapping. Starling's native-mod hook system is not Forge and is outside the vanilla 1.20.1 milestone; retain only as future extension-governance research.
- Evidence required before adoption: Do not adopt in the vanilla client. Any future native extension proposal must have a threat model, ABI/version contract, permission model, signature/trust policy, rollback tests and independent security review.
- Completion state: complete document review and line coverage verified; implementation claims, security behavior and build measurements are not independently tested in this audit.

### 19. `docs/starling-native-mods.md`
- Source repository/ref: `starling-client/starling` / `starling`.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high to the Galaxy A22 / limited local build-resource constraint. Remote compilation could avoid requiring Rust+NDK+multi-GB Cargo cache on the phone, permit asynchronous progress while playing, and deliver ABI-specific libraries. The APK-version stamp, hash-checked download, resumable transfer, cached builds, user-visible compile state, and rollback to bundled binary are reusable reliability ideas.
- Critical limitation for RustChunk: this system remotely compiles Starling's Rust mixins into a Rust Horizon 26.3 engine. It is a remote build path for Starling's native Rust mods, not a necessary part of the first RustChunk 1.20.1 milestone. It may be useful later only if RustChunk adopts a comparable native extension system.
- Decision: **adaptable** for remote build delivery, APK-bound cache invalidation, resumable verified downloads, rollback and explicit service trust; **unnecessary** for the base 1.20.1 client; consider only after a native Rust extension system is designed.
- Follow-up checks: inspect native_build.sh, the three Python service/gateway/sandbox scripts, native_mod_permissions.py, NativeMods.java and StarlingActivity; confirm runtime library loading order and JNI binding behavior; test failure/rollback and APK stamp logic; verify sandbox OS controls and clean-cache isolation; examine source/data filtering and transport privacy; verify SHA-256 provenance and gateway address trust chain.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: No first-release mapping. Remote native-mod build delivery/cache logic is unrelated to a playable vanilla 1.20.1 engine and does not implement Java Forge mod loading.
- Evidence required before adoption: Keep excluded until a separately approved native-extension design exists; before adoption require authenticated artifact provenance, checksum/signature verification, cache invalidation, rollback and hostile-package tests.
- Completion state: complete document review; no service, Android build, sandbox or native library was executed or security-tested in this audit.

### 20. `docs/mods.md`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: 6b6c04640da7292c3a4a60217347d5acd487ebd1
- Size: 493 lines; 30,867 characters; full file fetched and read in one request; final newline present.
- Coverage: lines 1–493, complete.
- Purpose: describes Horizon's mod model, supported mod types, enabling/loading, content mod authoring and schema, multiplayer compatibility checks, native code mods, registry publishing and artifact integrity.
- Two distinct mod systems:
  - Content mods (.hmod) carry data/assets only: blocks, items, recipes, loot, tags, models, textures, structures and supported registry records. They are loaded at game start and contain no executable code.
  - Native mods (.hnmod) are Rust/horizon-mixins bundles that alter Horizon source and are built into a per-mod-set executable; see native-mods.md.
  - Fabric and NeoForge JARs do not run as JARs: their Java code calls or patches Minecraft Java classes which Horizon does not have. Only suitable data/assets inside those archives can be carried/ported. This is explicit and relevant when deciding whether a future RustChunk data-import layer can use only the data portions of mod archives. **Forge classification:** historical importer capability/reference; the current Java Forge runtime remains deferred.
  - Mods are off by default; content changes require restart; two enabled content mods cannot own the same namespace; benchmarks exclude mods.
- Content-mod authoring and schemas:
  - cargo-horizon-mod scaffolds, builds, runs and checks content .hmod archives. Build validates manifest/content and rejects native Cargo project code in a content bundle. Registry checks include archive size/hash, manifest/content and optional minisign signature.
  - Manifest requires id/name/version/description/authors, API version, side, capability and optional namespaces. Unknown keys are errors. Content uses registries capability.
  - Blocks and items are declared through data/<id>/horizon/block and item JSON using vanilla templates. The schema includes declared state properties/defaults; deterministic state ordering; collision/outline shape boxes; block random-tick overrides; signs and connected pipe-like blocks; item data-component patches; crafting remainders; creative tabs; mob effects; entity types currently only based on snowball; and menu types/data/result slots.
  - Native code can register custom component types used by content item JSON; unsupported component ids and malformed fields fail rather than silently ignore. A copied vanilla item does not automatically inherit behavior tied to the vanilla item id (the document gives trident behavior as an example).
  - Content file namespace policy permits the mod's own id and explicitly listed namespaces, plus carefully scoped vanilla/convention tags; namespace conflicts leave the later mod out. Limits include up to 65,536 files, 16 MiB per file and 512 MiB total. Unknown tag members are dropped while preserving the rest.
  - Content data merging supports tags, add_to contributions for structure sets/template pools/processor lists, biome feature modifiers including NeoForge and older Forge biome-modifier paths, selected Lithostitched pool element types and enhanced terrain adaptation. Many kinds are explicitly not supported (e.g. add_spawns, add_carvers, mod-owned serializers and some biome modifier forms); unknown/malformed data has different skip/fail outcomes. **Forge classification:** historical importer capability/reference; the current Java Forge runtime remains deferred.
- Multiplayer/content synchronization:
  - Only side=both content mods are sent in the configuration phase and compared by id, version and SHA-256; mismatches/missing mods reject the client with named reasons. side=client mods stay private to the client; side=server stay private to server. A manifest with no side defaults to client, so cross-side content must explicitly declare both. Dev folder mods lack a file hash and compare by version.
  - Native mods are not included in the content mod list and the manifest lacks side; this is a distinct protocol and compatibility area.
- Registry and supply-chain model:
  - Initial project listing is human-reviewed against repository/source and license; later releases are not human-reviewed, only checked by automation (manifest, id/version, integrity, optional signature).
  - Registry computes/publishes SHA-256 and launcher refuses altered bytes. Authors can optionally sign releases with minisign/Ed25519; public key is pinned in registry and key changes are rejected. Revoked assets are disabled/refused. Unsigned local files remain allowed with an unsigned marker.
  - The registry's docs explicitly state that project/source review is once at listing, not repeated for each release. Thus hash/signature proves correspondence to an indexed/signed file, not that the code is benign or a new release was manually audited.
- Dependencies / cross-references: native-mods.md, modpacks.md, modpack authoring, mod-browser.md, api/mod-index.md, horizon_protocol::mod_list, mod-content loader and registry parser, cargo-horizon-mod CLI, horizon-rust/horizon-mods index and minisign verification. Their code and tests remain outside this file audit.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance:
  - Very high for a compatibility boundary. The project documentation explicitly states that Java Fabric/NeoForge JAR code cannot run in Horizon; only selected data/assets can be imported. For the current Rust-native 1.20.1 milestone, this means Starling's content importer is not a way to execute arbitrary Java mod code. **Forge classification:** historical importer capability/reference; the current Java Forge runtime remains deferred.
  - Potentially useful data-porting ideas: asset/data extraction from JARs, namespace handling, content validation, registry-based content model, resource pack/loot/recipe/tag/structure/biome data migration, client/server mod-side separation, server content hashes and clear mismatch errors.
  - Its registry and mod trust model offers useful design patterns: explicit opt-in, SHA-256, pinned signing keys, revoke lists, but it also makes clear that verification is not a sandbox or repeated human security review.
- Version and implementation caveats:
  - Schemas and data component names are for Horizon/Minecraft 26.3 and its custom loader, not automatically valid Forge 1.20.1 resource/data formats. Data ports need version-aware conversion and testing. **Forge classification:** historical importer capability/reference; the current Java Forge runtime remains deferred.
  - Content-only data cannot reproduce Java code behavior, Forge event subscribers, mixins, capabilities, custom networking, custom entities with code AI, or other logic simply because assets/JSON are copied. Native Rust mods cannot fill the Java Forge gap. **Forge classification:** historical importer capability/reference; the current Java Forge runtime remains deferred.
  - Some advertised entity/menu/block features remain explicitly incomplete; e.g. entity types are only snowball-template projectiles; menu slot behavior and some lifecycle hooks are still marked not yet.
- Decision: **unnecessary** for the base vanilla 1.20.1 milestone; **adaptable later** for a separate data-only import/porting layer and integrity/compatibility checks. This content importer does not implement the current 1.20.1 game core and does not execute Java mod code; Forge integration remains deferred. **Forge classification:** historical importer capability/reference; the current Java Forge runtime remains deferred.
- Follow-up checks: inspect mod-content parsing/validation, archive format and signatures; compare the listed data forms against actual 1.20.1 resource/data schemas; assess whether a later importer can safely extract data without claiming Java-code compatibility; inspect protocol mod-list handling and client/server side rules; create a version-aware importer only after the Forge runtime strategy is fixed. **Forge classification:** historical importer capability/reference; the current Java Forge runtime remains deferred.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps only to a possible future data-only content-import/validation layer. Starling `.hmod`/`.hnmod` packages are not equivalent to Java Forge JAR mods and must not be described as Forge compatibility.
- Evidence required before adoption: Require a 1.20.1-specific format specification, parser fuzzing/size/path limits, compatibility tests and a clear statement of unsupported Java/Forge APIs. Do not accept it as a substitute for Forge.
- Completion state: complete document review; no JAR import, registry validation, multiplayer handshake or Forge compatibility test was run in this audit. **Forge classification:** historical importer capability/reference; the current Java Forge runtime remains deferred.

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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: **critical**. This document describes the workflow RustChunk needs conceptually to turn a 26.3-based Rust game implementation into a version-accurate 1.20.1 implementation, and it exposes the key incompatibility in the current automated route: its assumption of modern unobfuscated releases. It supports a data-driven port plan rather than a simple version-string replacement.
- Decision: **adaptable** as the porting methodology (version pinning, provenance/hash verification, generated constants/tables, data diffs, oracle mapping, parity gates and ranked blockers); **rewrite/substantial adaptation required** for the current tooling and historical 1.20.1 input. Do not treat the 26.2→26.3 results as evidence of 1.20.1 compatibility.
- Follow-up checks: inspect `horizon-prepare` and its Java invocation, JAR extraction, asset export and data generation; inspect the generated version-constant and packet-ID pipeline; inspect `horizon-protocol`'s version adapter and packet tables; inspect datafix/world-data loading; inspect the code's pinned Java version and build toolchain; then independently verify 1.20.1's official version metadata, mappings, protocol, data version and formats before designing the port.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to RustChunk's reverse-port methodology in `docs/STARLING_1_20_1_ENGINE_MAP.md`, the pinned 1.20.1 JAR/mappings and per-subsystem port ledger. Starling's forward-port automation must not be run unchanged for this downgrade.
- Evidence required before adoption: Require exact 1.20.1 source/mapping hashes, class/method inventory coverage, generated-diff reproducibility and oracle-backed tests for every accepted subsystem; identify all unresolved/moved/changed symbols.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: low for the current Minecraft 1.20.1 engine port. It is useful only if RustChunk later adopts a similar separate SemVer policy for its launcher/application releases and needs safe upgrades from old launcher versions. It must not be mistaken for a mechanism to select, download or run Minecraft 1.20.1.
- Decision: **unnecessary** for the 1.20.1 game-engine port; **adaptable** as an optional launcher/application release-management pattern. No need to reproduce Horizon's historical bridge tags or its release workflow.
- Dependencies / cross-references: `.github/release-line.txt`; `.github/workflows/release.yml`; `.github/workflows/launcher.yml`; `tools/release_train.py`; `crates/horizon-server/src/update.rs`; `launcher/src/backend.rs`; `release-flow.md`; website release ordering and external updater consumers.
- Follow-up checks: none required for the Minecraft 1.20.1 compatibility investigation; revisit only when RustChunk's own update and release policy is designed.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to RustChunk's target-version/build identity, the 1.20.1 lock and generated version facts. Application-release semantics may remain separate; Starling historical bridge tags are not needed for the native engine.
- Evidence required before adoption: Confirm the game version, protocol 763, DataVersion 3465 and pack-format constants all come from one reviewed 1.20.1 source of truth; test that stale or mixed-version generated files are rejected.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high for understanding which layer Starling has actually implemented and for planning save compatibility. Its region/NBT/Anvil palette code may be a useful implementation reference. The DataFixer work suggests that reading/upgrading older saves is a separately handled subsystem; however, it does not prove that the game logic, registries, worldgen or runtime can execute at 1.20.1.
- 1.20.1 implications:
  - 1.20.1 data version and registry/state IDs differ from 26.3. Direct use of the 26.3 registry export or DATA_VERSION=5023 would misinterpret 1.20.1 data unless a version-aware data path is introduced.
  - DataFixers should be used in the correct direction and for the intended operation. The documented pipeline upgrades old worlds toward the pinned 26.3 schema; it is not a downgrade path and cannot be assumed to preserve semantics when targeting 1.20.1.
  - NBT and Anvil container mechanics are more reusable than the game-specific schema, but 26.3 list-wrapper/canonicalization assumptions and all block-state identities need comparison against 1.20.1 vanilla.
- Dependencies / cross-references: `crates/horizon-world`; its `datafix`, `nbt`, `region`, `chunk`, `registry` and fluid modules; `horizon-prepare` registry exporter; local vanilla oracle; `tools/test_datafix_parity.py`; world/tick/fluid parity suites; docs/block-state-codecs.md and docs/scheduled-ticks.md.
- Decision: **adaptable** for generic NBT/Anvil/palette mechanics and the methodology of DataFixer parity; **rewrite/version-gate required** for registries, DataVersion, block-state tables, chunk schema and any 1.20.1 world semantics. The crate itself is not evidence of a complete playable 1.20.1 engine.
- Follow-up checks: inspect `horizon-world/Cargo.toml`, `nbt.rs`, `region.rs`, `chunk.rs`, `registry.rs`, the DataFixer implementation and its version schema; inspect generated 26.3 registry artifacts and how `horizon-prepare` produces them. Compare actual 1.20.1 data version, registry exports, palettes, DataFixer schemas and world fixtures.
- RustChunk implementation mapping: Maps to the future native world-data layer plus `docs/STARLING_1_20_1_ENGINE_MAP.md`; current RustChunk block facts/tags are partial foundations, not a complete world implementation.
- Evidence required before adoption: Require byte-compatible 1.20.1 NBT/Anvil/chunk schemas, DataVersion 3465, palette/registry and world-data oracle fixtures, plus save-close-reopen tests before treating world support as accepted.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: extremely high for architecture and project-goal evaluation. It proves Starling contains a substantial native Rust reimplementation, not merely a themed launcher, but it also shows this reimplementation is a large and still incomplete Minecraft 26.3 port. Existing control/gameplay pieces can guide component boundaries and test design; they do not provide a drop-in Android Minecraft 1.20.1 runtime.
- 1.20.1 / device implications:
  - The client has pinned 26.3 assumptions in resources, components, registry IDs, models, assets, physics rules and vanilla oracles. Porting to 1.20.1 requires a version-matched dataset and code/API reconciliation throughout these components, not just changing a constant.
  - Running on Galaxy A22 ARM64/Mali is outside the platform/performance evidence in this document. Vulkan/NVIDIA/Linux-only constraints mean Android integration, renderer/backend, ABI, memory behavior and GPU support must be audited separately; no mobile support should be inferred from the Android launcher or other Android documentation.
  - This page does not show server multiplayer support in the client; an MCP/agent control socket is not a Minecraft server connection. The network implementation must be examined separately.
- Dependencies / cross-references: `horizon-client`; `horizon-prepare`; `horizon-assets`; `horizon-world`; `horizon-client-core`; `.local/horizon-client/26.3`; `research-lock.json`; `horizon-ctl`; `tools/test_native_scene_collision.py`; scene motion/health/hunger/sprint/crouch parity harnesses; model/blockstate/animation parity tests; `docs/rust-parity.md`; `docs/graphics.md`; Android and renderer-specific docs.
- Decision: **adaptable** as a Rust-native client architecture, resource-preparation design, scoped Java-oracle methodology and module decomposition; **major rewrite/version adaptation required** for 1.20.1 semantics and Android ARM64/Mali deployment. The current client is neither a ready-made 1.20.1 client nor proven Android game runtime.
- Follow-up checks: inspect `horizon-client` crate entry point, launch/render loop, `horizon-prepare` resource exporters, version constants, renderer backend/platform features, networking/protocol integration, worldgen/game loop and unfinished client lifecycle; inspect Android packaging and compare actual ABIs/GPU backend requirements. Verify 1.20.1 Java oracle/resources and differential tests separately.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to the future Rust-native game runtime and its resource-preparation boundary. Current `engine/src/lib.rs` exposes JNI operations for download/install/launch-plan work; it is not yet the native Minecraft gameplay client.
- Evidence required before adoption: Require an actual native client build that reaches a playable 1.20.1 single-player world, performs movement/block interaction and saves/reopens it. A successful JNI/JVM launch is not acceptance evidence.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high as a map for selecting the actual source files to inspect and for understanding cross-crate boundaries. It helps avoid treating Starling as a single monolith or assuming the launcher alone contains the game implementation.
- 1.20.1 implications: this index identifies the subsystems that must be version-audited, but does not describe their Minecraft version compatibility, runtime feature completeness or 1.20.1 behavior. The 40-area history is a recent churn map (through 2026-10-23), not a version-port plan and not an exhaustive list of every dependency.
- Decision: **adaptable** as an audit/navigation map and modular architecture reference; **insufficient by itself** to establish 1.20.1 support. No implementation decision should be made from this index without reading the actual referenced source and versioned data.
- Dependencies / cross-references: `crates/horizon-client`; `crates/horizon-client-lib`; `crates/horizon-world`; `crates/horizon-protocol`; `crates/horizon-client-render`; `crates/horizon-client-input`; `crates/horizon-client-core`; `crates/horizon-command`; `crates/horizon-items`; `tools/parity.py`; the harnesses named in the table; `AGENTS.md`.
- Follow-up checks: use the map to guide source-level audits of startup/resource loading, game loop, worldgen, save/load, packet processing, renderer and launcher integration; run no harness until its environment and 1.20.1 oracle are confirmed.
- RustChunk implementation mapping: Maps to RustChunk's authoritative architecture document `docs/STARLING_1_20_1_ENGINE_MAP.md`; treat the Starling document as navigation/evidence of upstream organization only.
- Evidence required before adoption: Trace each claimed subsystem to current RustChunk code or explicitly label it planned. Confirm dependencies and direction with source-level evidence; architecture prose alone cannot establish runtime behavior.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high for the legal/provenance design of obtaining game assets and code-derived facts. It establishes a clear distinction between open-source Rust implementation and player-supplied official Minecraft files. It does not show that the same preparation workflow accepts 1.20.1 or that any Minecraft files can be redistributed in RustChunk.
- 1.20.1 implications: if RustChunk targets 1.20.1, the permitted source of assets/JARs and the preparation path must be version-specific and must not assume the 26.3 checksums or existing data exporters are applicable. The code review must keep decompiled code and proprietary assets out of the RustChunk repository unless a separately verified legal basis allows otherwise. The page is a project policy summary, not a legal opinion.
- Decision: **adaptable** for user-owned asset preparation, public-source hygiene, signed release checksums and scoped parity gates; **no direct runtime code reuse decision** from this policy page. Reconfirm actual license and contribution constraints from the governing LICENSE/CONTRIBUTING files before any port or copying.
- Dependencies / cross-references: root LICENSE and CONTRIBUTING.md; THIRD_PARTY_NOTICES.md; `tools/check_public_tree.py`; `.github/workflows/ci.yml`; `.github/workflows/release.yml`; `crates/horizon-release-sign`; `horizon-prepare`; research-lock.json; docs/security.md; docs/mods.md and docs/native-mods.md.
- Follow-up checks: inspect the actual legal notices and the preparation/checksum implementation; verify Minecraft 1.20.1 ownership/manifest handling and local data-generation support independently. No reliance should be placed solely on this summary page for legal conclusions.
- RustChunk implementation mapping: Maps to RustChunk's resource-acquisition, build and source-hygiene documentation. Reuse process safeguards only where they match the current Kotlin/JNI app and future native build; do not inherit undocumented upstream assumptions.
- Evidence required before adoption: Require reproducible acquisition of user-owned inputs, verified hashes, clean public artifacts and CI checks. Independently validate every security/legal assertion against actual scripts and configuration.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high for selecting a reliable research workflow for a historical 1.20.1 target. 1.20.1-era JARs are from a different mapping/deobfuscation regime than the modern unobfuscated 26.3 assumption in the version-port design; therefore class-name mapping, remapping and decompiler quality become key risks.
- 1.20.1 implications: the counts and exact Vineflower/REA pipeline in this file apply only to 26.3. They do not establish complete 1.20.1 coverage, readable symbol names, successful decompilation, or an ability to rebuild Mojang's original client. We must separately verify 1.20.1's mapping artifacts, official version manifest, client/server JARs, Java baseline, and whether mappings/obfuscation can be applied to both sides consistently.
- Decision: **adaptable** as a bytecode research/decompilation workflow and as a warning to validate coverage; **rewrite/adapt required** for the 1.20.1 mapping and toolchain inputs. Decompilation is an analysis tool, not a route to redistribute or directly execute recovered Java source.
- Dependencies / cross-references: `tools/lab.py`; `tools/rea_vanilla.py`; `research-lock.json`; Vineflower; REA/JADX; `.local/minecraft/26.3/sources/`; local class inventories and coverage reports; `docs/version-ports.md`.
- Follow-up checks: inspect `tools/lab.py` and `tools/rea_vanilla.py`; inspect the actual 1.20.1 manifest's client/server mapping fields and JAR hashes; assess a remapped class inventory and decompiler report for 1.20.1 before depending on class-based diffing.
- RustChunk implementation mapping: Maps to the private 1.20.1 decompilation/class-index research workflow and its compact public reports; decompiled sources remain research inputs, not distributable application files.
- Evidence required before adoption: Require reviewed JAR/mapping SHA pins, class/method coverage report, mapping/decompiler diagnostics and private-workspace hygiene. Verify source coverage before using any decompiled method as an implementation oracle.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical as a method-level port-completeness methodology. Moving a substantial Java-version-specific Rust implementation toward 1.20.1 requires a traceable mapping from each 1.20.1 vanilla method to its Rust implementation, an explicit verdict when no direct method is needed, and parity evidence for behavior. This method ledger complements the class-level map in the version-port pipeline.
- 1.20.1 implications and caveats:
  - The existing method ledger is designed around the current pinned 26.3 game and 26.3 Java class/method names. It cannot simply be rebuilt using 26.3 source and claim 1.20.1 coverage.
  - 1.20.1 class/method names and bytecode differ materially, particularly due to the older obfuscation/mapping regime. A reliable 1.20.1 input requires mapping-aware JARs/decompilation and a new vanilla source oracle; class and method matches must be recalculated rather than carried over by names.
  - Structural and inferred matches can be false positives. Every important inferred mapping must be manually verified and backed by tests; test reachability does not automatically establish complete lifecycle coverage.
- Decision: **adaptable** as a core audit tool/concept (method mapping, verdict map, gap taxonomy and harness reachability); **rewrite/version-adapt required** for historical 1.20.1 JAR/mappings and its different Java API. The ledger is a planning/coverage instrument, not a game engine feature.
- Dependencies / cross-references: `tools/port_ledger.py`; `tools/vanilla_map.py`; `tools/port_map/*.tsv`; `tools/lab.py`; local Minecraft JAR and decompiled sources; `tools/parity.py`; Java parity oracles; Rust crates and doc comments; `tools/test_*_parity.py`.
- Follow-up checks: inspect `tools/port_ledger.py`, `tools/vanilla_map.py`, the verdict-map schema, class/method inventory generator and parity runner; then evaluate whether mapping-aware 1.20.1 bytecode can feed a trustworthy adapted ledger.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to a RustChunk 1.20.1 method/class port ledger connecting official Java methods to future Rust functions and parity probes. Existing `docs/STARLING_1_20_1_ENGINE_MAP.md` is the higher-level system map, not method-level proof.
- Evidence required before adoption: Require complete method inventory, explicit implemented/missing/rewrite verdicts, Rust source locations and reachable passing Java-oracle tests for accepted methods; no unmapped critical method may be silently counted as ported.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical as a quality-control pattern for any attempt to port Starling to 1.20.1. A version-matched vanilla oracle and isolated deterministic fixtures can prevent silent drift across protocol, worldgen, save, renderer and gameplay code. Dependency-aware test selection may also make a large port manageable, but it must never suppress necessary full audits.
- 1.20.1 implications: every vanilla oracle, fixture baseline, generated reference and cached pass must be regenerated or revalidated for 1.20.1. Reusing cached 26.3 oracle outputs would invalidate the port audit. Toolchain and JDK selection must also follow the 1.20.1 target rather than the current 26.3/JDK 25 assumptions.
- Decision: **adaptable** for differential testing, dependency fingerprinting, repeat/flake gates, deterministic scene fixtures and staged integration; **rewrite/version-adapt required** for a 1.20.1 vanilla oracle, fixtures, baselines, toolchains and expected data. Do not treat the existing 26.3 pass count as evidence of 1.20.1 parity.
- Dependencies / cross-references: `tools/parity.py`; `tools/parity_shim.py`; `tools/parity_reach.py`; `tools/gate.py`; `tools/parity_main.py`; `tools/supervise.py`; `tools/parity_hosts.json`; `lab.game_env`; Rust probe crates; Java oracle sources under `mods/horizon`; `.local/parity-runner/`; `shared-local/` and `PARITY_SHARED`.
- Follow-up checks: inspect parity runner, oracle shim, host scheduler, cache key/fingerprint logic, scope closure/reachability, baseline format and gate integration. Then construct a separate 1.20.1 oracle fixture matrix; never blend results from distinct Minecraft versions.
- RustChunk implementation mapping: Maps to a future 1.20.1 differential-test runner and CI gates. Reuse caching, deterministic fixtures and dependency-scope ideas only after replacing every 26.3 oracle and baseline.
- Evidence required before adoption: Require 1.20.1-specific fixtures, immutable oracle fingerprints, stable repeat runs, failure-on-drift tests and proof that cached results invalidate when source, data, toolchain or target-version inputs change.
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
  - No networking, game logic, OpenGL/Vulkan renderer, Java VM or Forge loader dependency is declared here. The Java runtime is invoked by the preparation implementation and must be inspected separately. **Forge classification:** current file-level conclusion: this manifest declares no Forge loader dependency.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: moderate-to-high as a compact reference for a native Rust resource-preparation tool that can target Android and perform SHA-1/SHA-256 verification and archive extraction.
- 1.20.1 implications: the dependency set itself is largely version-neutral, but the actual expected files, hashes, archive layout, data-generator invocation and generated outputs are determined in source code and repository pins. Updating only this manifest would not add 1.20.1 support.
- Decision: **adaptable** as a resource-preparer dependency pattern; **no direct version-port decision** can be made from the manifest alone.
- Dependencies / cross-references: `crates/horizon-prepare/src/main.rs`; `java.rs`; `artifacts.rs`; `lib.rs`; `check.rs`; root Cargo workspace; `research-lock.json`.
- Follow-up checks: inspect the preparer's command flow, Java version selection, expected official launcher files, archive verification, exporters, and resource version stamp; separately confirm Android runtime feasibility.
- RustChunk implementation mapping: Maps to the future RustChunk preparation/tooling package, separate from the current `rustchunk-engine` JNI launcher. Reuse only dependencies shown necessary for 1.20.1 artifact/data preparation.
- Evidence required before adoption: Run dependency/license/feature analysis and compile the selected preparation tool for its actual host/Android targets; prove it consumes the 1.20.1 lock and does not pull server/game-only dependencies unnecessarily.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high for a safe and repeatable local resource-preparation flow and for understanding how a launcher decides whether resources need rebuilding. The separation between core required data and optional user-asset-dependent extras is reusable.
- 1.20.1 implications:
  - The strict equality check against `lock.version` correctly prevents a directory prepared for 26.3 from being treated as current for 1.20.1, assuming RustChunk pins the target independently.
  - `SCHEMA` and `OUTPUT_VERSION` track the preparer's own layout/tool version, not the Minecraft data version; these should remain distinct from the Minecraft version pin.
  - Existence and stamp checks do not prove the actual data is 1.20.1-compatible or intact. The export/manifest/hash process must provide the authoritative content checks; this file alone is not a data-integrity boundary.
- Decision: **adaptable** as a preparation-completeness/stamp/version guard. It is mostly version-agnostic; exact 1.20.1 support depends on the manifest, lock reader, exporters, resources and checks in other modules.
- Dependencies / cross-references: `research-lock.json`; `artifacts::Lock`; `OUTPUT_VERSION`; `SCHEMA`; `horizon-resources.json`; `horizon-prepare.json`; `missing_extras`; launcher backend completion checks; resource commands `packs`, `panorama`, `fonts`, `languages`, `sounds`.
- Follow-up checks: inspect `main.rs`, `lib.rs`, `artifacts.rs`, the manifest writer and resource exporter implementations to verify that the stamp and Minecraft version are written only after all mandatory 1.20.1 artifacts are verified.
- RustChunk implementation mapping: Maps to a future resource-preparation validation/stamp stage and current 1.20.1 exporter/integration tests. It must validate generated artifacts before they are consumed by the native runtime.
- Evidence required before adoption: Test missing, stale, wrong-version, hash-mismatched and partially generated bundles; prove the completion stamp is written only after every mandatory 1.20.1 artifact passes validation.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high as a launcher/resource-ingestion architecture: it cleanly separates application version, target Minecraft version, the player's local official files, Java runtime selection, prepared resources, and launcher-facing progress/API contracts.
- 1.20.1 implications:
  - Switching the lock to 1.20.1 would alter the default resources folder and expected game version, but it is not enough by itself. `artifacts`, `java`, exporters, generated resource schemas and every game reader must accept the historical 1.20.1 client/server JAR and metadata.
  - 1.20.1 requires its matching Java baseline and matching client/server artifact hashes; do not carry over the 26.3/JDK25 assumptions without validating the 1.20.1 version manifest.
  - This CLI is a preparer for the native Rust reimplementation. It does not launch the original Java Minecraft client and does not establish a JVM/Forge runtime. **Forge classification:** current file-level conclusion: this CLI does not create a JVM/Forge runtime.
- Decision: **adaptable** as the preparation CLI and launch API; **version-specific implementation changes required** in the lock/artifact resolver, Java selection, exporters, and output resource contracts before it can prepare 1.20.1 resources.
- Dependencies / cross-references: `horizon_prepare::Options`; `artifacts::Lock`; `prepare_with_progress`; `add_assets`; `compile_exporters`; `check`; `launch_api`; `research-lock.json`; `horizon-resources.json`; launcher Launch API.
- Follow-up checks: inspect `java.rs`, `artifacts.rs`, `lib.rs`, exporters, output manifest/stamping, and the precise 1.20.1 version metadata/JDK requirement. Confirm whether the preparer's asset path expects 26.3-only layouts.
- RustChunk implementation mapping: Maps to a future RustChunk resource-import/preparation CLI or orchestration entry point; do not merge it conceptually with `engine/src/installer.rs`, which belongs to the current launcher workflow.
- Evidence required before adoption: Require end-to-end preparation from verified user-owned 1.20.1 inputs, correct progress/error reporting and an output manifest accepted by the Rust engine; test interrupted and repeated runs.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high. This is evidence Starling's **resource preparation** can launch Java from an app-private runtime on Android; it does not mean the Rust game itself runs in a JVM. The supplied runtime hook is likely important when preparing 1.20.1 resources on a phone.
- 1.20.1 implications:
  - The required Java major is supplied by `research-lock.json`; the current comments and surrounding docs use Java 25 for 26.3. The exact 1.20.1 Java requirement must be read from its official version metadata and pinned independently (do not assume 25).
  - This module's minimum-version check can select a newer runtime than requested. Compatibility of a newer JDK with the historic Minecraft 1.20.1 data generator/exporter and decompiler needs verification; an exact Java 17 runtime may be a safer baseline if 1.20.1 metadata says 17.
  - Android 64-bit devices such as the Galaxy A22 do not receive the 32-bit heap cap in this implementation; its 6 GB class memory profile still requires measured heap choices and careful coexistence with the renderer. The 384/512 MiB rules apply only to 32-bit Android.
  - `javac --release` requires a compiler capable of the selected release target. Exporter source compatibility and dependencies must also be checked for the older 1.20.1 Java toolchain.
- Decision: **adaptable** for Android-supplied Java, JDK discovery, controlled batch execution and timeout handling; **version/toolchain policy changes and device-specific memory validation required** for 1.20.1. No claim is made that the game engine needs Java at runtime; Java here supports vanilla exporters/data prep.
- Dependencies / cross-references: `research-lock.json` Java major; `Supplied`/`supply`; `find`/`find_runtime`; `horizon-prepare` `Options.java_home`; `horizon-prepare/src/lib.rs`; `artifacts.rs`; exporter compilation and vanilla datagen; Android launcher runtime provisioning.
- Follow-up checks: inspect the source of runtime provisioning and `artifacts.rs` to verify major selection; confirm 1.20.1's version JSON/JDK baseline; inspect the Java exporter compile path; measure heap/time on Android ARM64 and ensure no unbounded concurrent JVMs.
- RustChunk implementation mapping: Maps to an optional host/on-device Java helper used only for 1.20.1 extraction/export/oracle work, not to the Rust gameplay engine. Current `engine/src/jli.rs` tests Java runtime loading and must remain clearly labelled launcher/tooling.
- Evidence required before adoption: Prove the exact Java major required by each helper, bounded heap/concurrency/timeouts and ARM64 feasibility. Verify the native game reaches gameplay without launching a JVM in the first milestone.
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
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: **critical**. This is the main gate between the user's legitimately installed Minecraft files and the native Rust data preparation pipeline. It identifies a concrete incompatibility with the current 1.20.1 goal, not a hypothetical concern: the current code deliberately refuses mapped releases.
- Decision: **major adaptation required** for 1.20.1. Reuse the secure path handling, checksum verification, manifest validation, explicit user-provided game files and local-only artifact storage; replace or extend the blanket mappings rejection with a verified 1.20.1 remapping workflow and version-aware handling of libraries/server bundling.
- Dependencies / cross-references: `research-lock.json`; `Lock`; `GameFiles`; `Inputs`; `game_files`; `import_manifest`; `import`; `unpack_server`; `client_library_artifacts`; `safe_relative`; `launcher_library`; `verify`; `client_libraries`; `inventory`; `files::Import`; ZIP helpers; `pool::map`; `.local/minecraft/<version>`.
- Follow-up checks: inspect `lib.rs`/`java.rs` and all exporter consumers; verify the actual 1.20.1 version JSON's mappings, SHA pins, Java major, libraries and server archive; determine the mapping namespace needed by Java oracles and Rust-source correspondence; test client and server remapping independently and record source/output hashes.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to the 1.20.1 artifact-import/download verification boundary. Current `engine/src/downloader.rs` is a launcher downloader; any reuse must be reviewed for official metadata, safe paths and strict digest checks.
- Evidence required before adoption: Test official 1.20.1 manifest/JAR/library hashes, malicious archive paths, interrupted downloads, symlinks and cache corruption; verify mappings are handled intentionally and private game files never enter public artifacts.
- Completion state: complete source and unit-test review; no actual game artifacts were imported, checksum-verified, unpacked or remapped, and no tests/build were run.

### 35. `crates/horizon-prepare/src/lib.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `00ff58173199fa11d717243ed2614e9cfb002618`.
- Size: 76,227 bytes; 1,966 lines; fetched in consecutive ranges 1–400, 401–800, 801–1,200, 1,201–1,600 and 1,601–1,966.
- Coverage: lines 1–1,966, complete; all five ranges returned the same blob SHA; all included unit tests were read.
- Purpose: main resource-preparation engine. It imports verified game artifacts, runs vanilla Java data generators/exporters, extracts client assets/data, builds worldgen and gameplay resource bundles, tracks provenance/hashes and writes the resource manifest consumed by the Rust client.
- Structure and execution:
  - Declares the 18-stage progress protocol, resource manifest schema 7 and preparer output version 3. The output version is independent of the Minecraft game version; the launcher uses it to decide whether generated resources must be recreated.
  - Computes a SHA-256 fingerprint over preparer source files; hashes jar/source/output inputs and reuses a previous export only when relevant inputs and output hashes match.
  - Uses lock files for both shared `.local` scratch data and the destination resource directory. Android uses `rustix::flock` because the standard file-lock API is unavailable in the targeted Android toolchain; other platforms use standard file locks.
  - Imports/verifies client and server JARs/libraries through `artifacts`; obtains a compatible Java runtime through `java`; compiles or reuses the original Java exporter bundle; then runs server, client, datagen and file-copy lanes concurrently with bounded exporter workers.
  - Runs the vanilla server data generator through `net.minecraft.data.Main` and `--reports`; installs registry/command reports, per-item default component data and biome parameter reports. The creative-inventory and command registries are produced by additional Java exporters.
  - Exports block-state facts, movement/attributes/landing, damage/health, collision, face-visibility/occlusion, gameplay facts, entity types, block behaviors, client model-state bindings, entity models, mob data and creative tabs. These outputs form a significant versioned interface between Minecraft's Java data/API and the Rust crates.
  - Extracts every selected client JAR member under `assets/`, `data/` and `pack.png`, avoids path traversal, verifies existing ZIP members by uncompressed size and CRC, repairs damaged/missing files and preserves unchanged files. Optional indexed assets may override JAR placeholders.
  - Writes 19 worldgen resource bundles (noise, density, material rules, conditions, settings, carvers, biomes, presets, features, placed features, state providers, tags, processor lists, template pools, structures and structure sets), structure templates, block behavior/support/survival files and biome tables.
  - The final resource manifest stores version, client/server JAR SHA-256, exporter-source hashes, generated file hashes, data version, resource-file count and schema. The prep stamp is written only after all stages complete; on failure the previous stamp is removed.
- Test code read:
  - Tests exercise lock collision detection, datagen report installation/repair, progress monotonicity and stage count, output-version history, source hash reproducibility, Python-compatible line splitting, export-stamp invalidation, client archive extraction/recovery, independent ZIP clone reads and path traversal defense.
  - Tests establish unit-level contracts only; they were not executed. No full 26.3 preparation or live data generator was run during this audit.
- Critical 1.20.1 implications:
  - This is not merely a generic asset copier: it compiles and invokes many Java exporter classes that call 26.3 vanilla APIs, exports version-specific registries/behaviors/collision/models/entities/damage, and generates worldgen tables in formats consumed by the Rust engine. Their API names, constants and data structures must be reconciled with 1.20.1.
  - The importer refuses mapped release manifests before this stage (see file 34), so 1.20.1 cannot reach it until a mapping/remapping approach is implemented. After that, exporter source compatibility and oracle classes are a likely substantial work area.
  - `items.json` is generated from the modern report path `minecraft/components/item`; 1.20.1 predates the 1.20.5 data-component system, so this input format and Rust item model are not directly reusable as-is. Item defaults, stack metadata, and registries need a 1.20.1-specific data representation.
  - The 19 worldgen data-pack registries and report layout must be compared with the 1.20.1 data pack schema; new/renamed registries and features in 26.x may be absent, and 1.20.1 may require alternate generators or custom exporters.
  - `SCHEMA=7`, `OUTPUT_VERSION=3`, file list and data-version manifest should be versioned deliberately. Do not confuse a preparer output-layout bump with the Minecraft DataVersion. `data_version` is read from the client JAR's `version.json` world_version; this needs an actual 1.20.1 pin.
  - The client/server equality checks for several data resources may be valid for the current 26.3 workflow but must be checked against the 1.20.1 JARs and historical split between client/server packaged assets/data.
  - Its Android-aware lock path is useful, but multiple concurrent Java exporter lanes and heap requests up to 2G (creative inventory) can create high memory pressure. Android 64-bit currently has no SMALL_ANDROID heap cap; device-class memory behavior needs explicit testing and likely a separate policy.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: **critical**. This file is the bridge between version-specific vanilla truth and every large subsystem in the native Rust game. It pinpoints the major conversion effort: 1.20.1-compatible JAR mapping, exporter/oracle rewrite, data-format adaptation and resource-reader updates.
- Decision: **reuse the pipeline architecture**, especially provenance, content hashes, reproducible generation, output stamps, lock protection, local-only source data and explicit staged preparation; **substantial version-specific rewrite required** for Java oracles/exporters, modern item components, registries, worldgen, resource manifest contract and Android memory policy.
- Dependencies / cross-references: `artifacts.rs`; `java.rs`; `exporters.rs`; `sounds.rs`; `survival.rs`; `worker.rs`; `pool.rs`; `pyjson.rs`; `mods/horizon/src/test/java/dev/horizonlab/parity`; `horizon-world`; `horizon-assets`; `horizon-client`; `research-lock.json`; `horizon-resources.json`; `output-versions.txt`.
- Follow-up checks: inspect `exporters.rs`, `survival.rs`, `sounds.rs`, `worker.rs`, `pool.rs`, build-time source hashing; inspect the Java parity oracle classes actually compiled; compare 1.20.1's `version.json`, datagen reports, registries, item model and worldgen JSON with the current generated resource contract; design a separate resource schema for 1.20.1 where necessary.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to a future resource-export pipeline that supplies registry, block-state, biome and world-generation facts to native Rust. Existing 1.20.1 block facts/tag tests are proven partial pieces; the full exporter/runtime contract is not complete.
- Evidence required before adoption: Require deterministic exports from the pinned 1.20.1 Java oracle, schema/version stamps, full required-field validation and Rust consumer tests for every artifact type. Preserve private source/output hygiene.
- Completion state: complete source and test review; no exporter compilation, JVM process, data generation, resource extraction, preparation, build or test was executed.

### 36. `crates/horizon-protocol/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `377b6d997e84642646aca16b3762485ff14b73ff`.
- Size: 1,108 bytes; 36 lines; fetched in one complete range.
- Coverage: lines 1–36, complete.
- Purpose: defines the native Rust crate implementing the Minecraft Java Edition network protocol for Horizon.
- Dependency and version findings:
  - Package description explicitly says Minecraft Java Edition 26.3, protocol version 777. This makes the crate's current protocol target unambiguously 26.3.
  - Depends on `horizon-core` and `horizon-world`, so protocol code also relies on shared version constants and world types/data structures.
  - Uses zlib compression, JSON serialization, OS randomness, SHA-1, SHA-256/RSA-related signature handling and AES for connection encryption. Windows additionally depends on `ipconfig` for SRV redirect support.
  - Defines dedicated Cargo examples/probe targets for protocol and item wire behavior; it has no Java runtime or Forge dependency. **Forge classification:** current file-level conclusion: this protocol crate declares no Java runtime or Forge dependency.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical for a native Rust client intended to connect to ordinary Minecraft Java servers. This is a real protocol implementation, not a bridge to a Java process.
- 1.20.1 implications:
  - The crate description and current protocol adapter are 26.3/protocol 777. Minecraft Java 1.20.1 uses a different protocol generation; all state/direction packet IDs, packet layouts, login/configuration/play sequence, registries, entity metadata, chunk payloads, chat/security and compression/encryption behavior need a 1.20.1-specific implementation or adapter.
  - Dependencies are largely version-agnostic primitives, but `horizon-world` and `horizon-core` bind the crate to versioned world/data/protocol constants. Reusing these dependency declarations alone does not create 1.20.1 compatibility.
  - The README-style package description should be updated only after source code and tests have a genuine version-aware implementation; changing the wording is not a port.
- Decision: **adaptable** at the transport/serialization/cryptography library level; **rewrite/introduce a 1.20.1 protocol adapter** and reconcile shared core/world types. Protocol 777 must not be reused for 1.20.1.
- Dependencies / cross-references: `horizon-core`; `horizon-world`; protocol `adapter.rs`; packet modules; compression, crypto, login and connection handling; `protocol_probe`; `item_wire_probe`; vanilla protocol report/packet-ID generation.
- Follow-up checks: inspect `adapter.rs` and its version modules, packet ID table and packet bodies for handshake/login/configuration/play, then compare against an authoritative 1.20.1 protocol schema and a real vanilla 1.20.1 server handshake.
- RustChunk implementation mapping: Maps to a future protocol crate only after the single-player engine gates; the current RustChunk Cargo workspace has no Minecraft packet/protocol implementation. Multiplayer is deferred in the engineering map.
- Evidence required before adoption: Require a complete protocol-763 specification and codecs, verified packet/state inventory, golden byte vectors and successful real vanilla 1.20.1 server interoperability before claiming network support.
- Completion state: complete manifest review; no Cargo build, dependency resolution, packet probe, network connection or server test was run.

### 37. `crates/horizon-protocol/src/adapter.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `52b7439919e630c2e0d4fc3a4f6a768d7ee26f7c`.
- Size: 6,576 bytes; 184 lines; fetched in one complete range, including tests.
- Coverage: lines 1–184, complete.
- Purpose: first layer of protocol compatibility for older servers; maps packet IDs per protocol state and direction between Horizon's pinned protocol and an older server.
- Implementation:
  - `ServerVersion` recognizes only `Current` (26.3/protocol 777) and `V26_2` (protocol 776). It uses a separate version table for packet-ID translations in each state/direction.
  - Translates clientbound packet IDs from the old server to the pinned client's IDs, and outgoing serverbound IDs from the pinned client to the older server's IDs.
  - Returns `None` when a packet does not exist in the other version, allowing unsupported packets to be dropped/not sent.
  - `serverbound_body` rewrites only the VarInt packet ID and copies the payload bytes unchanged. It explicitly does **not** translate packet contents, field encodings, state transitions, registry data, chunk formats or other semantic changes.
  - Tests hardcode the 26.2↔26.3 ID examples and packet omissions, rather than deriving expected values from the same generated table.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high for server-connection strategy. It demonstrates a protocol adapter abstraction, but also makes a strict distinction between packet-ID renumbering and payload compatibility.
- 1.20.1 implications:
  - 1.20.1 is not the adjacent 26.2 protocol generation. Its packet structure and state machine are substantially different, and the current ID-table adapter cannot safely translate 1.20.1 by shifting IDs alone.
  - A usable 1.20.1 connection path needs a true 1.20.1 packet schema/codec or a separate protocol implementation, including handshake/status/login/configuration/play state differences, packet body changes, registries, chunk/section encodings, entity metadata, chat, encryption/compression and feature-specific behavior.
  - `ServerVersion::ALL`, protocol lookup and adapter routing are currently limited to two known versions. A new 1.20.1 adapter would need an explicit, independently validated entry and likely a broader abstraction than ID tables.
- Decision: **adaptable** as a multi-version routing and ID-map pattern; **insufficient and unsafe** as a 1.20.1 compatibility layer without a complete packet-body/state adapter.
- Dependencies / cross-references: `horizon-protocol` `PROTOCOL_VERSION`, `GAME_VERSION`, `State`, `Direction`, `buf::Reader`, `write_varint`, `ids`; `adapter/v26_2.rs`; connection handshake/status/server detection; protocol probes.
- Follow-up checks: inspect `adapter/v26_2.rs` and connection startup/state handling; build a full authoritative 1.20.1 packet table and body codecs from protocol data; test with a real vanilla 1.20.1 server instead of assuming 26.2 mappings generalize.
- RustChunk implementation mapping: Maps to a later protocol-version routing layer, not the current milestone. The 1.20.1 client should use a native protocol-763 implementation, not translate 26.x packet IDs as a shortcut.
- Evidence required before adoption: Require an authoritative 1.20.1 packet/state table and payload-level codec tests; demonstrate correct handshake/login/play transitions against a vanilla 1.20.1 server.
- Completion state: complete source and test review; tests and network interoperability were not run.

### 38. `crates/horizon-protocol/src/adapter/v26_2.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `a2b05a74f56078748ce9947306e0349f120b8ced`.
- Size: 4,406 bytes; 84 lines; fetched in one complete range.
- Coverage: lines 1–84, complete.
- Purpose: generated packet-ID translation tables for protocol 776 (Minecraft 26.2) against Horizon's pinned protocol 777 (26.3).
- Implementation findings:
  - Defines version metadata (`PROTOCOL=776`, `NAME="26.2"`) and lookup tables for clientbound/serverbound IDs in Handshake, Status, Login, Configuration and Play.
  - Tables are arrays indexed by the older wire packet ID for clientbound mapping, and by the pinned-version ID for serverbound mapping. `Option<u8>` represents absent/unmappable IDs; several packet IDs shift because packets were added/removed in 26.3.
  - Handshake/status/login entries are largely identity mappings; configuration and play tables contain the version-specific renumbering and omissions.
  - The file explicitly states it is generated by `tools/generate_packet_id_adapter.py` from two versions' packet reports and must not be edited manually. It contains IDs only, no packet body serializers/deserializers.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: useful evidence for a generated, state-aware packet ID layer. It can reduce manual errors for adjacent protocol versions when packet payloads remain compatible.
- 1.20.1 implications:
  - This map has no relevance as a direct mapping for 1.20.1. The packet inventory, state transitions and packet payloads have evolved far beyond the adjacent 26.2→26.3 gap.
  - A 1.20.1 adapter needs a complete packet list and schema for every state/direction, with independently generated ID mappings and separate payload codecs wherever packet structure changed. A numeric translation table alone is insufficient.
  - The `u8` entries also encode an assumption that this current table's packet IDs fit in one byte; any new version's ID representation and VarInt behavior must be checked rather than copied blindly.
- Decision: **adaptable** as a code-generation/output pattern; **not reusable as data** for 1.20.1. Generate a fresh 1.20.1 packet table from authoritative version-specific data only after its complete packet schemas and state transitions are understood.
- Dependencies / cross-references: `adapter.rs`; `tools/generate_packet_id_adapter.py`; 26.2 and 26.3 `packets.json` reports; `State`; `Direction`; `PROTOCOL_VERSION`; packet codecs and connection state machine.
- Follow-up checks: inspect the generator and packet report format, then derive/verify a full 1.20.1 packet inventory and real handshake/login/play flow. Add payload-level tests; do not use this file as a compatibility shim.
- RustChunk implementation mapping: No reusable packet-table data mapping. Only the generator/output pattern may be considered for a later protocol-763 table; the v26_2 IDs are not usable in RustChunk 1.20.1.
- Evidence required before adoption: Regenerate all IDs from a version-pinned 1.20.1 oracle and verify each state/direction against packet schemas and a real server trace. Reject any output still containing 26.2/26.3 identifiers.
- Completion state: complete generated-file review; no generator, packet table test, protocol connection or network interoperability run was performed.

### 39. `crates/horizon-protocol/src/packets/handshake.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `1686adb23f2e9d06e9c1b24637fb3fa906eedad8`.
- Size: 988 bytes; 39 lines; fetched in one complete range, including test.
- Coverage: lines 1–39, complete.
- Purpose: defines the client-to-server handshake packet (`ClientIntentionPacket`) and intent constants.
- Wire structure:
  - Fields are protocol number as VarInt, hostname as a bounded string (`StrN<255>`), port as unsigned short, and intent as VarInt.
  - Current intent constants are STATUS=1, LOGIN=2 and TRANSFER=3. A single `Intention` packet is registered in the Handshake state's serverbound packet enum.
  - The unit test encodes protocol 777, localhost:25565, login intent and checks the leading packet ID and protocol VarInt bytes; a packet round-trip is also asserted. Test not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high. This packet is the initial point where a client selects protocol version and asks a server for status or login; it is central to connecting a native client to a matching Minecraft Java server.
- 1.20.1 implications:
  - The core field ordering and VarInt/port/string structure is likely largely reusable, but the protocol number must be the 1.20.1 value and the accepted intent set must match that protocol's handshake exactly.
  - `TRANSFER=3` is a later protocol feature and must not be advertised/sent to a 1.20.1 server unless its handshake implementation explicitly supports it. The exact 1.20.1 packet ID and maximum host string interpretation should be confirmed against a version-matched reference.
  - A handshake packet alone cannot establish compatibility: status/login/configuration/play packet IDs, encryption/compression, registry negotiation and packet payload schemas must also match.
- Decision: **mostly adaptable** for the basic handshake field codec; **version-gate protocol constant and intent options** for 1.20.1. Do not carry forward protocol 777 or later-only transfer semantics.
- Dependencies / cross-references: `buf::StrN`; `UShort`; `VarInt`; `packet!`; `packet_enum!`; `ids::handshake::serverbound`; `State::Handshake`; protocol connection startup; `adapter.rs`.
- Follow-up checks: inspect the connection state machine and generated IDs; verify 1.20.1 handshake bytes against an authoritative protocol specification and a vanilla 1.20.1 server.
- RustChunk implementation mapping: Maps to the future protocol-763 Handshake packet codec and connection bootstrap. This is multiplayer work and not required for the first local single-player vertical slice.
- Evidence required before adoption: Verify exact 1.20.1 handshake bytes, protocol value 763, host/port/intent encoding and status/login selection against both golden vectors and a vanilla 1.20.1 server.
- Completion state: complete source/test review; no packet encoding test, handshake or network connection was executed.

### 40. `crates/horizon-protocol/src/packets/login.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `3e494a6a9d5a1336fe1231aaaba7967ba86914d7`.
- Size: 4,588 bytes; 134 lines; fetched in one complete range, including tests.
- Coverage: lines 1–134, complete.
- Purpose: defines packet body codecs and state-specific enums for the current protocol's Login state.
- Packet structures present:
  - Property, GameProfile (UUID, 16-character name and property list), Disconnect, Hello (server ID, public key, challenge, authenticate flag), LoginFinished (profile plus a session UUID), Compression, CustomQuery, and CookieRequest.
  - Serverbound messages include ServerHello (name and UUID), encryption Key (key and challenge bytes), CustomQueryAnswer, LoginAcknowledged and CookieResponse.
  - Packet IDs are imported from the generated `ids::login` tables; codecs use VarInt, UUID, bounded strings, byte arrays, optional values, identifiers and rest-of-packet payloads.
  - Tests round-trip the packet variants and check trailing-byte rejection and player-name length validation. The tests were read, not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical. Login is required for both offline and authenticated Java Edition server connections; its packet bodies and phase transitions must be compatible even before the client can enter Play.
- 1.20.1 implications:
  - This file is for the current 26.3 protocol and includes later login-era features such as cookie requests/responses and a separate login acknowledgement, as well as a LoginFinished body carrying a session UUID. These must not be assumed to exist in 1.20.1.
  - The 1.20.1 login state uses its own Login Start/Encryption Response/Plugin Request-Response/Set Compression/Login Success or Disconnect forms and a different transition into play; exact IDs, field order, bounds, optionality and session/profile fields must come from a 1.20.1 protocol reference or vanilla oracle.
  - A 1.20.1 port needs to reconcile offline identity and online authentication behavior with that older handshake/login flow. Merely mapping packet IDs while retaining these current packet bodies will produce wire-incompatible messages.
- Decision: **reuse low-level codecs and cryptographic primitives only where protocol semantics match**; **rewrite the Login state packet set and connection transition for 1.20.1**. Do not expose current cookie/acknowledgement/session fields to the 1.20.1 adapter by default.
- Dependencies / cross-references: `ids::login`; `buf` codecs; `crypto`; `packets::Empty`; login connection state machine; encryption/compression negotiation; authentication/offline UUID generation; `adapter.rs`.
- Follow-up checks: inspect `connection.rs`, crypto and status packets; verify the precise 1.20.1 Login Start, Encryption Request/Response, Plugin Request/Response, Set Compression, Login Success and Disconnect packet structures and phase transition against a real 1.20.1 server.


## Scope correction — 2026-11-17
The active RustChunk target is Minecraft Java Edition **1.20.1 implemented through the Rust-native Starling/Horizon architecture**. Forge is explicitly deferred and is not an acceptance requirement for the current phase. **Forge classification:** current project conclusion: Java mod-loader/Forge integration is deferred until the vanilla 1.20.1 Rust client works.
- The first batch's older Forge-oriented wording and any conclusion framed as suitability/unsuitability for Forge 1.20.1 must not be used as the current project decision. Those entries remain in the audit history but are superseded for scope purposes by this correction. **Forge classification:** current project conclusion: Java mod-loader/Forge integration is deferred until the vanilla 1.20.1 Rust client works.
- The current job is to determine how to adapt the 26.3-pinned Rust game implementation so it implements and runs the 1.20.1 Java Edition rules, data, resource formats, world behavior and network protocol. It is not to run the Java Minecraft client or provide a Forge runtime in this phase. **Forge classification:** current project conclusion: Java mod-loader/Forge integration is deferred until the vanilla 1.20.1 Rust client works.
- Java code/JARs may still be used locally as version-matched research/oracle/export tooling; that does not change the goal into a JVM game runtime. The Rust engine is the implementation under review.
- Native Rust mods, Java mod loaders and Forge integration are not blockers for the first 1.20.1 milestone. Their relevance can be reassessed only after the base 1.20.1 client is working. **Forge classification:** current project conclusion: Java mod-loader/Forge integration is deferred until the vanilla 1.20.1 Rust client works.
- From this point, classify each file by whether it is reusable, version-specific, requires rewrite, or unnecessary for the **Rust-native 1.20.1 port**. Prioritize the observed current blocker: `horizon-prepare/src/artifacts.rs` deliberately rejects manifests with client/server mappings; then audit the protocol, datagen/exporters, data schemas, engine behavior and platform/runtime integration.
- RustChunk implementation mapping: Maps to the future 1.20.1 Login state codec and authentication transition in the protocol crate; do not port modern cookies/configuration packets wholesale.
- Evidence required before adoption: Verify all 1.20.1 Login packet fields, online encryption/session flow, disconnect cases and Login-to-Play transition against official source and real server tests; confirm no separate Configuration state is introduced.
- Completion state: complete source and test review; no packet test, login, encryption exchange, compression negotiation or server interoperability run was performed.

### 41. `crates/horizon-protocol/src/lib.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `c0fc01d304ed13ce5541b6cc2415c3c33deec2a5`.
- Size: 2,546 bytes; 87 lines; fetched as a complete file.
- Coverage: lines 1–87, complete.
- Purpose: crate root and public API surface for the native Minecraft Java Edition protocol implementation.
- Findings:
  - Describes itself as the protocol implementation for whichever version is pinned in `horizon_core::version`; the package manifest and current generated constants resolve that to Java 26.3/protocol 777.
  - Exposes primitive codecs, packet framing, compression/encryption, typed packet modules and auxiliary synchronization/mod transport functionality. Client and future server share the crate.
  - Denies unsafe Rust crate-wide, except the dedicated ARMv8 AES-intrinsic module's local allowance.
  - Defines exactly five connection states: Handshake, Status, Login, Configuration and Play; directions are Clientbound and Serverbound.
  - `PROTOCOL_VERSION` aliases `horizon_core::version::PROTOCOL_VERSION`; `GAME_VERSION` aliases the core vanilla version. The protocol crate therefore inherits the core version pin rather than owning an independent version selection.
  - Defines one string-backed protocol error with explicit timeout and end-of-stream classifications and conversion from I/O errors.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- RustChunk relevance: critical; this is the top-level network API shared by the client. It shows a central design dependency on one global `horizon-core` version, while older-server compatibility is layered separately.
- 1.20.1 implications: simply changing core version constants would cause this crate to advertise a new protocol number while leaving packet definitions and all supporting schemas at 26.3. The crate root exposes modules for registries, items, entity data, chunks and packets which must be independently version-reconciled; protocol-state availability and transitions also need verification against 1.20.1.
- Decision: **adaptable** as the central protocol crate structure, shared state/direction/error model and safe Rust module boundaries; **substantial version-specific adaptation required** across nearly every exposed module. Keep version ownership centralized only if all modules select coherent 1.20.1 schemas.
- Dependencies / cross-references: `horizon-core::version`; `adapter`; `attachment_sync`; `buf`; `chunk`; `connection`; `crypto`; `entity_data`; `frame`; `ids`; `item`; `mod_list`; `mod_net`; `packets`; `registry`; `resolver`; `rsa`; `secure_chat`; `servers`; `text`.
- Follow-up checks: inspect each public module, especially connection state transitions, packet payload codecs, registry sync, chunk data, item wire encoding, entity metadata, encryption and server-list/status handling; validate against 1.20.1 protocol data.
- RustChunk implementation mapping: Maps to the future protocol crate root: state/direction/error types and module boundaries. No such native Minecraft protocol crate exists in the current RustChunk workspace.
- Evidence required before adoption: Require a complete 1.20.1 public API inventory, compile-time exclusion of later-only states/types and integration tests covering every supported handshake/login/play path.
- Completion state: complete source review; no build, test or live protocol exchange was run.

### 42. `crates/horizon-protocol/src/ids.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `892b6af9a53c105ad663c56c07db9fe3714a61b8`.
- Size: 22,066 bytes; 590 lines; fetched in consecutive ranges 1–400 and 401–590.
- Coverage: lines 1–590, complete; both ranges returned the same blob SHA.
- Purpose: generated packet-ID constants and ID-to-name tables for every connection state/direction of Minecraft Java protocol 777 (26.3).
- Findings:
  - File header states it is generated from the vanilla data generator's packet report by `tools/generate_packet_ids.py`; the file must not be edited manually.
  - Defines Handshake serverbound, Status both directions, Login both directions, Configuration both directions and Play both directions. No Handshake clientbound packet table is defined.
  - Enumerates 21 clientbound and 10 serverbound Configuration packets, 144 clientbound Play packets and 69 serverbound Play packets, in addition to the smaller earlier-state tables. IDs include later-version packets such as cookies, transfers, post effects, data components-era interactions, dialogs, code of conduct, waypoints and modern chat/session fields.
  - `NAMES` arrays are parallel to numeric IDs and power `name(state, direction, id)`. Invalid states/directions/negative or out-of-range IDs return `None`.
  - The module imports the common `State` and `Direction`, so all protocol consumers share this single generated ID namespace.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- RustChunk relevance: critical. These IDs determine packet framing/dispatch and every packet enum's encoding/decoding. This is a completely version-specific generated artifact.
- 1.20.1 implications:
  - No numeric ID from this file can be presumed valid for 1.20.1. It belongs to protocol 777 and contains packet kinds absent from 1.20.1; older protocol also uses a different login/configuration/play split and packet catalog.
  - A 1.20.1 port needs a fresh, authoritative packet report/ID inventory and version-specific generated constants and names, plus code changes for packet types that were added/removed or structurally different.
  - The generator's input format and ability to obtain packet data from a 1.20.1 mapped JAR must be validated; the current import workflow refuses mapped releases. Do not manually renumber this table or mechanically offset IDs.
- Decision: **do not reuse data**; **regenerate entirely for 1.20.1** using a verified 1.20.1 vanilla report. The generator pattern and `name()` lookup API are adaptable, provided all packet consumers switch atomically to coherent version-specific definitions.
- Dependencies / cross-references: `tools/generate_packet_ids.py`; vanilla datagen `packets.json`; `State`; `Direction`; packet enums in `packets/*.rs`; `adapter.rs`; protocol probes; `horizon-core::version`.
- Follow-up checks: inspect the generator and packet report schema, all consumers of numeric constants and `NAMES`, and the handshake/login/configuration transitions for 1.20.1. Confirm whether a unified `Configuration` state is even valid for the target version.
- RustChunk implementation mapping: Maps to generated protocol-763 packet IDs and reverse name tables in a future protocol crate. Do not check in Starling's 26.3 IDs as RustChunk data.
- Evidence required before adoption: Generate from a reviewed 1.20.1 packet report; test complete state/direction coverage, duplicate IDs/names, unknown-ID handling and parity with real vanilla packet captures.
- Completion state: complete source review; no packet report generation, protocol tests, codegen or network interoperability run was performed.

### 43. `crates/horizon-protocol/src/connection.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `a7e1d30600b073eecf7aa26d0ccb4b25ec5ee539`.
- Size: 27,464 bytes; 744 lines; fetched in consecutive ranges 1–400 and 401–744.
- Coverage: lines 1–744, complete; both ranges returned the same blob SHA; all unit tests read.
- Purpose: implements the byte transport and framed Minecraft packet connection, including TCP and in-process memory pipes, optional AES/CFB8 encryption, VarInt21 framing, zlib threshold compression, split reader/writer and connection counters.
- Transport behavior:
  - `Pipe` is a thread-safe in-memory duplex byte channel with close notification, optional read timeout and clone semantics; it supports vanilla-style LocalChannel communication between an integrated server and local player.
  - `Stream` abstracts TCP and Pipe. A `Connection` keeps paired stream handles for separate read and write sides, can be split into `ConnReader`/`ConnWriter`, and can be closed through a `Closer`.
  - TCP connect resolves all socket addresses, puts IPv4 first to match Java's default address preference, and uses a 250 ms stagger between attempts so another address can race while the first is pending. The first successful socket wins; reported failure uses the first attempted address's error.
  - `connect_server_with_address` applies the SRV resolver and returns the resolved host/port for a login handshake; status callers may preserve the original address separately.
- Packet framing and crypto:
  - Reader incrementally buffers incomplete frames, parses frame headers, inflates compressed payloads and decodes a VarInt packet ID plus payload. Empty compressed frames are skipped, matching the documented vanilla decoder behavior.
  - Encryption uses AES/CFB8 both directions with the shared 16-byte secret as key and IV. If encrypted bytes were read past a plain packet before encryption is enabled, the buffered tail is decrypted at activation.
  - Compression threshold is shared by reader/writer; negative values disable it. Writer frames packet bodies, optionally compresses, encrypts and writes; raw unframed APIs exist for legacy server-list ping behavior.
  - Counters track sent/received packets and bytes; read timeout and EOF map to explicit protocol error classifications.
- Unit tests read: IPv4-first selection and IPv6 fallback, fast fallback from dead address, deterministic error reporting, silent peer timeout/EOF, TCP compression+encryption, in-memory pipe packet exchange/close, packet sizes from one byte through 200 KB, pipe timeout/drop semantics, encrypted bytes buffered before activation and empty compressed-frame handling. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: very high. The transport/framing/encryption primitives are broadly reusable beneath a version-specific packet layer and offer an integrated-server in-memory transport.
- 1.20.1 implications:
  - TCP, socket resolution, raw streams, most AES/CFB8, packet-length framing and threshold compression are likely reusable protocol infrastructure, subject to parity verification with 1.20.1's packet limits and compression behavior.
  - This implementation uses the 26.3-era crate's `frame` API and packet assumptions. 1.20.1 connection phase handling (login success/transition directly to Play versus modern Configuration) must be implemented above it; transport alone does not make the packet stream compatible.
  - CFB8 key exchange and enabling point depend on the 1.20.1 login sequence. The connection primitive can likely support the older flow, but exact ordering and encryption boundary must be verified against a real server.
  - Address racing and SRV behavior are launcher/client features rather than Minecraft-version-specific gameplay, but the precise DNS behavior should be compared to the target client/version.
- Decision: **largely adaptable** as transport and packet framing infrastructure; **verify and adapt boundaries** for 1.20.1 frame/compression rules, crypto activation and connection-state orchestration. Do not confuse the in-memory Pipe with remote server networking.
- Dependencies / cross-references: `frame`; `crypto::Cfb8`; `buf::Reader`; `resolver`; `State` consumers; login packet flow; `ConnReader`; `ConnWriter`; `Closer`; `Counters`.
- Follow-up checks: inspect `frame.rs`, `crypto.rs`, `resolver.rs`, login/connection state machine and packet size limits; validate handshake/login/encryption/compression with a vanilla 1.20.1 server.
- RustChunk implementation mapping: Maps to future TCP transport, connection lifecycle, framing handoff and online-mode encryption/compression plumbing. The current downloader networking is not a Minecraft protocol connection.
- Evidence required before adoption: Test connect/disconnect/error lifecycle, handshake/login, encryption activation, negotiated compression and bounded packet sizes against a vanilla 1.20.1 server on desktop and Android.
- Completion state: complete source and test review; no connection unit tests or real TCP server interoperability were run.

### 44. `crates/horizon-protocol/src/buf.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `4b82513417a34119abc2d23f7517d5b83377835e`.
- Size: 22,954 bytes; 724 lines; fetched in consecutive ranges 1–400 and 401–724.
- Coverage: lines 1–724, complete; both ranges returned the same blob SHA; all tests and macros read.
- Purpose: generic wire-level buffer readers/writers and typed `Codec` building blocks modeled on Minecraft's FriendlyByteBuf, VarInt/VarLong, ByteBufCodecs and StreamCodec composition.
- Implemented primitives and structures:
  - Bounded Reader over packet bytes with big-endian signed/unsigned integers, floats, booleans, VarInt/VarLong, byte arrays, UTF-8 strings capped by Java UTF-16 code units, UUIDs, NBT, variable/fixed long arrays and remaining-byte access.
  - Writer for corresponding primitives, VarInts, strings/byte arrays, UUID, network NBT and long arrays. `write_varint` and `varint_size` are exported helpers.
  - Typed codecs include Bool/Byte/UByte/Short/UShort/Int/Long/Float/Double/VarInt/VarLong/Uuid, bounded strings and bytes, Rest, Nbt, Longs, List, Opt, OptVarInt, integer/double vectors, packed BlockPos/ChunkPos/SectionPos, BitSet and low-precision velocity vectors (`LpVec3`).
  - The `packet!` macro generates a Rust packet struct and a field-order codec by composing primitive/typed codecs. Packet structs derive Clone/Debug/PartialEq.
  - Network NBT read uses a 2 MiB quota. Writer emits a zero/End tag if the NBT writer cannot encode a tag within its configured default limits.
- Behavior/tests read:
  - VarInt/VarLong examples include negative and boundary values and overlong encodings; string tests cover UTF-8 byte length versus Java UTF-16 character count; positions test packed bit fields and negative boundaries.
  - BitSet writes omit trailing zero bytes; low-precision velocity codec quantizes, sanitizes NaN and clamps large values; angle packing mirrors Mth pack/unpack.
  - Tests cover list/optional/UUID round trips and representative position/codec examples. Tests read but not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical reusable foundation. Most primitive encodings are protocol mechanics independent of Minecraft version and are suitable for an older-version adapter, provided exact vanilla behavior is revalidated.
- 1.20.1 implications:
  - Big-endian fixed-width values, VarInt/VarLong, UUID, UTF-8, basic lists/options and many packed positions are likely reusable. String limits and Java UTF-16 counting are also broadly applicable.
  - `LpVec3`, section/position codecs and NBT wire representation must be checked against 1.20.1's exact method/packet usages; no assumption that 26.3's newer packet fields share these codecs is safe.
  - NBT itself remains a core 1.20.1 wire type, but integration with 26.3-specific `horizon_world::nbt` semantics, quota accounting and root-name/list changes should be verified against 1.20.1's NbtIo implementation.
  - Modern data-component/item payloads introduced after 1.20.1 must not be carried back into the old item wire model just because generic codecs compile.
  - Writer length/count casts are not uniformly validated before emitting VarInt counts; callers/codecs must enforce their own bounds where vanilla does. The protocol parser should also ensure packet payloads are fully consumed at the right decode boundary.
- Decision: **largely adaptable** for primitive codecs and macro composition; **method-by-method validation and selective replacement required** for special packed/vector/NBT codecs and version-specific structures.
- Dependencies / cross-references: `horizon_world::nbt`; `Reader`; `Writer`; `Codec`; `packet!`; `frame`; packet modules; `chunk.rs`; `item.rs`; `entity_data.rs`; NBT quota and Java's FriendlyByteBuf/ByteBufCodecs.
- Follow-up checks: compare each specialized codec's source behavior with 1.20.1 vanilla methods; inspect NBT read/write and packet decode strictness; add a 1.20.1 golden-byte corpus for boundary and malformed inputs.
- RustChunk implementation mapping: Maps to a future protocol codec library for VarInt, strings, arrays, NBT and typed fields. The current RustChunk engine has no Minecraft wire codec.
- Evidence required before adoption: Build a 1.20.1 golden-byte corpus for every primitive and special codec; test boundaries, malformed/truncated input, trailing bytes, overflows and parity with vanilla serialization.
- Completion state: complete source/macro/test review; no codec tests or 1.20.1 byte comparison was run.

### 45. `crates/horizon-protocol/src/frame.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `243a07c87fe0de3ba5bcbf405cc8ebe7faab2bfc`.
- Size: 9,473 bytes; 254 lines; fetched as a complete file, including tests.
- Coverage: lines 1–254, complete.
- Purpose: implements Minecraft's VarInt21 packet-length framing and zlib compression/decompression above the negotiated threshold.
- Protocol behavior:
  - Maximum frame payload is `(1<<21)-1` bytes; the length prefix must fit in at most three VarInt bytes and zero-length frames are rejected.
  - With compression disabled, frame content is the packet body. With compression enabled below threshold, the content begins with uncompressed-length VarInt 0 followed by raw body. At/above threshold it begins with the declared uncompressed length followed by zlib-compressed bytes.
  - Maximum uncompressed packet size is 8,388,608 bytes. Decoder rejects negative lengths, uncompressed payloads below threshold, declared sizes above maximum, malformed zlib streams, wrong decoded length and streams that do not terminate cleanly.
  - `Inflater` resets and reuses a zlib decompressor between frames. Noncanonical but legal VarInt encodings are read by the VarInt parser rather than rejected solely for width.
- Tests read: uncompressed/compressed round trips, below/at-threshold boundary, malformed size and stream length, reusable inflater after invalid frames, and noncanonical prefix handling. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high. This is generic transport framing and compression infrastructure shared beneath Minecraft packet schemas.
- 1.20.1 implications: packet framing, VarInt21 and zlib compression are core Java Edition wire mechanics that are likely reusable for 1.20.1, but the exact uncompressed-size cap, threshold semantics, handling of malformed/noncanonical prefixes and error behavior should be checked against the 1.20.1 NetworkCompression/PacketDecoder implementation. It does not solve differences in packet IDs or packet bodies.
- Decision: **likely reusable with targeted parity verification**. Keep framing separate from version-specific packet codecs and add 1.20.1 boundary vectors before trusting interoperability.
- Dependencies / cross-references: `buf::Reader`; `buf::write_varint`; `connection.rs`; `Inflater`; `flate2` zlib; packet-size limits.
- Follow-up checks: verify Java 1.20.1 VarInt21FrameDecoder/LengthFieldPrepender and CompressionDecoder/Encoder edge behavior, including empty frames and declared-size limits; test actual compressed traffic with vanilla 1.20.1.
- RustChunk implementation mapping: Maps to future protocol framing/compression below the 1.20.1 packet codecs; it is separate from world/chunk storage compression.
- Evidence required before adoption: Verify VarInt21 framing and zlib threshold behavior with byte vectors, empty/oversized frames, corrupt streams and real server traffic; enforce bounded allocation before decompression.
- Completion state: complete source/test review; tests and actual packet framing/compression were not run.

### 46. `crates/horizon-protocol/src/packets/mod.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `4ef8ac21adafb7f10770ad867cbe5873471ed108`.
- Size: 3,119 bytes; 87 lines; fetched as a complete file, including macro/test helper.
- Coverage: lines 1–87, complete.
- Purpose: declares packet modules and provides the `packet_enum!` macro that generates typed packet enums for each connection state/direction.
- Findings:
  - Modules cover Configuration, Handshake, Login, Play, Recipes and Status.
  - `packet_enum!` generates an enum with modeled packet variants and an `Unknown(id, raw_payload)` fallback. `id()` maps typed variants to generated numeric constants; `encode()` writes VarInt packet ID followed by the type's codec body.
  - Typed decode constructs a `Reader`, decodes the matching struct and rejects any unconsumed payload bytes. This is intended to reproduce vanilla's rejection of oversized packet bodies.
  - Unknown packet IDs are preserved as raw payload without attempting typed parsing, so unsupported packet bodies can be retained and potentially forwarded.
  - `Empty` is a zero-field packet codec. Test-only `round_trip` reads the ID prefix and validates typed round-trip behavior.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- RustChunk relevance: high. This macro is the central mechanism joining packet IDs to body codecs; its version correctness depends on the IDs and packet structs registered in each state.
- 1.20.1 implications:
  - Macro architecture is version-neutral, but the current module layout assumes a separate Configuration state and modern packet variants. Minecraft Java 1.20.1 has no separate post-login Configuration connection state; the target uses Handshake, Status, Login and Play, so this module's Configuration enum must be excluded and its compatible packets relocated to their 1.20.1 phases.
  - The unknown fallback is useful for resilience, but unknown packets cannot be semantically processed, and passing them through does not guarantee compatibility with protocol feature negotiation or required packet handling.
  - Strict trailing-byte rejection should be tested against actual 1.20.1 serializers/decoders to ensure no valid packet uses padding or intentionally ignored trailing fields.
- Decision: **adaptable** as typed-packet macro and unknown-packet representation; **regenerate/reorganize state-specific enums and IDs** for 1.20.1. Preserve strict decode guarantees only where the reference client does so.
- Dependencies / cross-references: `buf::Codec`; `Reader`; `Writer`; `ids.rs`; packet modules; `State`; `Direction`; `packet!`; `round_trip` tests.
- Follow-up checks: compare the generated macro's strictness and enum routing with Minecraft 1.20.1's connection dispatch; map packet modules to the target's actual states and typed packet catalog.
- RustChunk implementation mapping: Maps to typed packet dispatch for the 1.20.1 protocol states and directions. Reuse a macro/pattern only if it does not assume modern Configuration state or packet IDs.
- Evidence required before adoption: Require exhaustive packet-enum-to-ID mapping, round-trip codec tests, unknown-packet policy tests and compile/test evidence that state transitions match 1.20.1.
- Completion state: complete source/macro/test-helper review; no macro expansion, protocol tests or packet exchange was run.

### 47. `crates/horizon-protocol/src/packets/status.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `7df73880febd4e9f9548d93b469e69b2f414ae8d`.
- Size: 1,314 bytes; 50 lines; fetched as a complete file, including tests.
- Coverage: lines 1–50, complete.
- Purpose: defines server-list Status state packets: status JSON response and ping/pong timestamp exchange.
- Packet bodies:
  - Clientbound `StatusResponse` contains one UTF-8 string encoded with the general `Str` codec (the status JSON document).
  - Clientbound `PongResponse` and serverbound `PingRequest` use one signed 64-bit timestamp (`Long`).
  - Serverbound `StatusRequest` is an empty packet. Typed packet IDs are read from `ids::status`.
  - Tests round-trip all four packet forms, including a negative timestamp. Tests read but not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: moderate-to-high. Status ping is a useful minimal first connectivity test before attempting login; the body shapes are simple and mostly stable across Java Edition versions.
- 1.20.1 implications: The general status JSON string and ping/pong long bodies are likely reusable, but packet IDs must come from the 1.20.1 status packet table. Status JSON's `version.name` and `version.protocol` must report the actual target version/protocol, not 26.3. Server list favicon, sample players, secure-chat/previews or other JSON fields may differ and should be treated as data rather than hardcoded current-version assumptions.
- Decision: **body codecs largely adaptable**; **IDs and response metadata must be version-specific** for 1.20.1.
- Dependencies / cross-references: `ids::status`; `buf::Str`; `buf::Long`; `packets::Empty`; status response builder; `Connection` and server list ping code.
- Follow-up checks: inspect status packet construction/server-list code and protocol version reporting; compare JSON and status/ping flow with a vanilla 1.20.1 server.
- RustChunk implementation mapping: Maps to a future optional server-list ping path in the protocol/UI layer; it is not needed to create a local single-player world.
- Evidence required before adoption: Verify protocol-763 Status packet IDs, JSON version metadata and ping/pong round trips against a vanilla 1.20.1 server; test malformed response and timeout handling.
- Completion state: complete source/test review; no status ping or 1.20.1 server connection was run.

### 48. `crates/horizon-protocol/src/packets/configuration.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `48976429f92dd61b7c0c5a85377e2dec69be1f71`.
- Size: 13,382 bytes; 354 lines; fetched as a complete file, including all tests.
- Coverage: lines 1–354, complete.
- Purpose: defines packet codecs for the modern Minecraft Configuration state: registry synchronization, tags/features, custom payloads, keepalive/ping, resource packs, cookies, known packs, transfer, server links and dialogs/code-of-conduct; also defines client-information and response packets.
- Main packet structures:
  - Cookie request/response, custom payload, NBT disconnect, keepalive, integer ping/pong, registry entries with optional NBT and registry data lists.
  - Resource-pack pop/push with UUID, URL/hash/required flag/optional NBT prompt; cookies; transfer host/port; enabled features; registry tags with VarInt ID arrays; known-pack selection.
  - Later configuration features include code-of-conduct text, server links (known ID or custom NBT label), show-dialog NBT and custom dialog click action with a length-prefixed NBT payload capped at 65,536 bytes.
  - `ClientInformation` has language, view distance, chat visibility/colors, skin part bitfield, main-hand, text filtering, server-listing permission and particle status. Clientbound/serverbound enums route through the 26.3 configuration IDs.
- Tests read: all packet round trips, main-hand values, tag/registry optional data, known packs, links, dialogs and click-action payload's exact encoded shape (`None` emits a length followed by End tag). Tests not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical as evidence of substantial modern protocol schema, but the file as a whole is not the right state/module layout for Minecraft 1.20.1.
- 1.20.1 implications:
  - Minecraft Java 1.20.1 does not have the separate post-login Configuration connection state introduced in later protocol generations. This module's state machine placement is therefore structurally incompatible with the target.
  - Modern cookies, known-pack negotiation, update-enabled-features, transfer, dialogs, custom click actions, code-of-conduct, server links and post-26.x configuration packet set must not be sent to a 1.20.1 server.
  - Registry data, tags, client information, resource-pack requests, custom payloads and keepalives may still be needed in 1.20.1, but several are carried in different connection phases and can have different packet IDs and body shapes. For example, dimension/biome registry data belongs to older login/join-game-era structures rather than this modern generic Configuration registry synchronization packet.
  - Current `ClientInformation` includes modern fields (notably listing permission and particle status) and uses newer codec assumptions; its 1.20.1 predecessor must be reconstructed field-by-field from that version's vanilla client/server classes.
  - NBT, identifiers, lists and UUID primitive codecs can be reused selectively, but all packet structs/enums and their routing should be versioned separately.
- Decision: **do not reuse this module as a 1.20.1 Configuration state**. Extract only genuinely shared packet/data structures and primitive codecs; design 1.20.1 Status/Login/Play packets and move any compatible configuration-like messages into their historically correct state.
- Dependencies / cross-references: `ids::configuration`; `buf` codecs; `Tag`; `packets::Empty`; `packet!`/`packet_enum!`; `registry.rs`; `item.rs`; `connection.rs`; `State::Configuration`.
- Follow-up checks: inspect current login and play codecs, registry/chunk login payloads and state-transition handling; compare exact 1.20.1 clientbound Login/Join Game, Update Tags, Client Settings, Resource Pack, Keep Alive and Plugin Message structures against vanilla 1.20.1.
- RustChunk implementation mapping: No 1.20.1 Configuration-state mapping: protocol 763 goes from Login to Play without a separate post-login Configuration state. Reuse only independently verified primitive/data codecs in their actual 1.20.1 context.
- Evidence required before adoption: Prove the state machine has no Configuration state and test the full Login-to-Play transition, including registry/game-join data, against vanilla 1.20.1 source and a real server.
- Completion state: complete source/test review; no round-trip tests or 1.20.1 packet capture/interoperability run was performed.

### 49. `crates/horizon-protocol/src/packets/play.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `0da8053f315d53b739bf39823c8874f6aa17c5af`.
- Size: 141,345 bytes; 3,922 lines; fetched in consecutive ranges 1–400, 401–800, 801–1,200, 1,201–1,600, 1,601–2,000, 2,001–2,400, 2,401–2,800, 2,801–3,200, 3,201–3,600 and 3,601–3,922.
- Coverage: lines 1–3,922, complete; all ten ranges returned the same blob SHA; all packet enums, custom codecs and unit tests read.
- Purpose: the large typed packet catalogue for modern protocol Play state, including world/chunk, entity, player, inventory, recipe, command tree, advancement, sound, chat and serverbound interaction packets.
- Scope of implementation:
  - Shared structures include global spawn information, relative movement flags, registry-backed/inline holders, signed chat types, entity movement paths/deltas, biome chunk updates and map patches.
  - Clientbound packet families include entity spawn/movement/removal/metadata/equipment/passengers; block updates/events/destruction/section changes; block entities; chunk+light and chunk-biome payloads; login/respawn; player position, health, abilities, inventory, experience and attributes; containers/merchant offers; sounds/particles/explosions; scoreboard/team/boss-bar; advancements/statistics; commands; chat/player-info; tags and game events.
  - Serverbound families include movement, player actions/commands/input, use/interact, teleport acknowledgement, chat and signed commands/session update, container clicks and hashed item changes, recipe/trade actions, book/sign/structure/jigsaw/test-block operations, difficulty/rules, resource packs, plugins and misc admin/debug requests.
  - Custom serializers handle modern component-based item costs, hashed inventory stack synchronization, modern entity metadata/particle payloads, movement path/delta formats, modern chat signatures/last-seen state, map patch color data, command argument parser registries, advancement display/criteria and test-instance/waypoint structures.
  - Clientbound and serverbound enums register packet types against `ids::play` (the generated protocol 777 IDs). Several packets are imported from the modern Configuration and Login modules, reinforcing the modern phase model.
- Critical 1.20.1 mismatches found in the actual source:
  - Item wire format: current packet definitions use `item::Stack`, `Component`, `HashedSlot`, `DelimitedSlot` and typed component patch data. Java 1.20.1 predates the 1.20.5 data-component item serialization; its item stacks use the older count/item ID plus optional NBT tag representation. Current merchant cost, container click, creative slot, equipment, inventory, advancement icon and item-bearing metadata codecs cannot be carried over unchanged.
  - Join Game/Respawn: current `Login` and `SpawnInfo` serialize a modern structure (dimension type registry ID, dimension key, seed, game modes, last death, portal cooldown, sea level, levels, simulation distance, secure-chat flags). 1.20.1's login/join-game and respawn packet layouts contain older dimension codec/registry NBT and fields; these bodies must be separately implemented.
  - Entity/player metadata: packet variants and field groups have grown and changed. Current player-info actions include listing/order/hat and modern chat session data; entity metadata, attributes, damage, movement and particles contain modern 26.3-era fields/IDs. 1.20.1 needs version-specific metadata serializers and packet bodies.
  - Chat: this code includes indexed/packed signatures, last-seen updates, chat acknowledgement, signed command argument signatures and remote chat sessions. 1.20.1 has a different secure-chat/session packet flow; this cannot be adapted by reusing IDs alone.
  - Chunk/light: current `LevelChunkWithLight`, `ChunkBiomeData`, `ChunksBiomes`, `LightData` and `ChunkData` bind to the current chunk palette/biome/light encoding and modern registry assumptions. 1.20.1 has its own chunk data, heightmap, block-entity, biome, light-mask and section palette formats; inspect and implement independently.
  - The packet catalogue contains clearly later-era types/features: configuration acknowledgements, chunk batching, client tick end, player loaded, punch, add transient block, test-instance blocks, ticker state/step, post effects, transfer, waypoint, dialogs, cookies, code of conduct, new minecart movement, known-pack and custom click actions. These are not part of a plain 1.20.1 Play flow and must not be emitted.
  - Commands use a 26.3 `ARGUMENT_TYPES` ID table and parser property layouts, including dialog/swing-animation and later resource selector types. The 1.20.1 command argument registry and parser schema differ.
  - Many superficially familiar packets (block update, entity movement, health, sounds, teams, scoreboard, signs, books, interactions) still have different IDs, fields, enums or serialization rules across versions. A packet name match is not evidence of wire compatibility.
- Useful reusable foundations:
  - High-level packet model separation, typed codecs, explicit per-packet structs, unknown-packet fallback, bounds-checked readers and byte-level tests are useful architecture patterns.
  - Primitive field encodings such as VarInt, UUID, fixed-width numbers and selected positions can be reused if exact target-version behavior matches. Movement/position math and some packet semantics may inform implementation but require reference parity.
  - The tests provide examples of golden-byte vectors and adversarial/truncated-body rejection; the same style should be recreated against a 1.20.1 vanilla oracle.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: **critical**. This file is the bulk of the network protocol surface between the Rust game and a Java server; its current content is one of the largest version-specific obstacles to running 1.20.1.
- Decision: **do not port the current Play packet structs as a set**. Keep the typed packet design and generic codecs, but build a dedicated 1.20.1 Play packet catalogue with independently sourced IDs, payload schemas, registry assumptions and golden-byte/parity tests. Separate the reusable game concepts from their wire representations. The existing modern code may remain a reference for behavior only where method-level comparison proves equivalence.
- Dependencies / cross-references: `ids::play`; `buf`; `chunk::{ChunkData,LightData,Paletted}`; `entity_data`; `item::{Component,Slot,Stack,Hashed}`; `packets::configuration`; `packets::login::Property`; `packets::recipes`; `horizon-world::nbt`; `registry`; connection state machine and vanilla packet reports.
- Follow-up checks: inspect `chunk.rs`, `entity_data.rs`, `item.rs`, `registry.rs`, `packets/recipes.rs` and `packets/play.rs` consumers; obtain 1.20.1 serverbound/clientbound packet schemas and mappings; create separate 1.20.1 golden traces for login, join game, chunk load/light, inventory, entity metadata, movement, chat and common interactions.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to a future protocol-763 Play packet catalogue, split by direction and packet family. It is deferred until after the single-player runtime and must be sourced afresh, not copied from 26.3.
- Evidence required before adoption: Require a complete 1.20.1 packet inventory with payload schemas, IDs and golden vectors; test critical join, chunk, entity, inventory, movement and disconnect paths against vanilla.
- Completion state: complete source/test review; the numerous unit tests were read but not executed, and no 1.20.1 packet capture, server connection or byte-level comparison was performed.

### 50. `crates/horizon-protocol/src/chunk.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `f961b1b230dd57f05532844869db6a4c755166aa`.
- Size: 18,283 bytes; 522 lines; fetched in consecutive ranges 1–400 and 401–522.
- Coverage: lines 1–522, complete; both ranges returned the same blob SHA; all tests read.
- Purpose: packet codecs and data structures for serialized chunk columns/sections, block and biome palettes, heightmaps, block entities, and sky/block light layers.
- Implemented structures:
  - `Paletted` represents a registry-ID palette plus bit-packed storage. It has strategy-specific local/global palette thresholds for blocks and 4×4×4 biomes, packed long access, conversion from flat values, and safe handling of storage bit widths.
  - `Section` stores non-empty block count, a separate fluid count, block-state palette and biome palette.
  - `ChunkData` stores a collection of heightmap type IDs and packed long arrays, a length-prefixed section byte buffer, and block entity position/type/NBT records. Helpers serialize/deserialize sections by a caller-supplied count and global block/biome bit widths.
  - `LightData` stores four bitsets (sky/block present and empty masks) and arrays of 2048-byte light layers. Helper expands those masks into per-section optional sky/block arrays; nibble lookup reads low/high nibbles.
  - Bit storage packs `64/bits` values per long without crossing word boundaries; blocks contain 4096 entries and biome containers 64.
- Tests read: optimized palette construction matches the slow reference; bit storage, block/biome palette thresholds and global widths; invalid bit widths; chunk section/heightmap/block-entity round trips; light mask/layer/nibble interpretation. Tests not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: extremely high. This is a direct network-to-world boundary for receiving terrain and lighting. Incorrect chunk codec means corrupt/invisible terrain regardless of the renderer's performance.
- 1.20.1 implications:
  - Paletted block/biome container concepts, 4096/64 entry sizes, per-long packing and the general sky/block light mask model are likely reusable, but exact palette bit thresholds, registry global widths and IDs must match 1.20.1.
  - The current section codec includes both `non_empty` and `fluids` short counters. 1.20.1's historical section wire structure must be confirmed directly; do not assume the additional fluid count is present.
  - The current `Heightmaps` wire codec is a count of numeric type IDs and long arrays. 1.20.1 chunk packet heightmaps use the older NBT compound representation rather than this newer typed list format; that field requires a distinct 1.20.1 codec.
  - `ChunkData` section buffer shape, included biome containers, block entity info and section count depend on 1.20.1's dimension/build-height and registry rules. Current caller-supplied global widths and section counts must be derived from target version data.
  - Light layer masks are conceptually similar across releases, but mask bit indexing, trust-edges behavior, sky dimension rules, light section count and packet envelope must be checked against the target client/server.
  - Current code is bound to modern 26.3 registry IDs and resource exports. Matching serialized numeric palette values without a 1.20.1 registry is unsafe.
- Decision: **adapt the palette/storage and light primitives**, but **rewrite the chunk packet envelope, section structure, heightmap format and registry integration** for 1.20.1. Keep a dedicated version-specific chunk decoder rather than a shared struct with unchecked modern fields.
- Dependencies / cross-references: `buf::{BitSet,Longs,Bytes,VarInt}`; `horizon_world::nbt::Tag`; `packets::play::{LevelChunkWithLight,ChunksBiomes,LightUpdate}`; registry state/biome widths; world section dimensions; renderer chunk loader.
- Follow-up checks: inspect `registry.rs`, `packets/play.rs` chunk packet call sites and `horizon-world` palette representation; compare against 1.20.1 `LevelChunkPacketData`, `LevelChunkSection`, `Heightmap` and `ClientboundLightUpdatePacket` serializers, including actual captures for overworld/nether/end and negative/minimum-height chunks.
- RustChunk implementation mapping: Maps to the future network chunk decoder and its adapter into the 1.20.1 world/chunk model. This is distinct from local chunk generation/storage and is later multiplayer work.
- Evidence required before adoption: Compare chunk packet bytes/fields, section palettes, heightmaps, biomes, block entities and light arrays against a vanilla 1.20.1 server; test malformed counts and registry mismatches.
- Completion state: complete source/test review; tests, generated chunk packets and live 1.20.1 terrain/light reception were not run.

### 51. `crates/horizon-protocol/src/registry.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `6d7d528ac789dc0d99420d4197c515302b9331e8`.
- Size: 11,388 bytes; 310 lines; fetched as a complete file, including tests.
- Coverage: lines 1–310, complete.
- Purpose: stores built-in and server-synchronized registries, bidirectional name/ID lookup, synchronized registry data/tags and dimension properties used by the client/protocol.
- Built-in registry handling:
  - Parses the vanilla data generator `registries.json` report; for each registry it reads each entry's `protocol_id` and builds parallel names and reverse name→ID lookup.
  - Searches for the report beside prepared resources or in development datagen output.
  - `SyncedNames` resolves names first from built-in registries, then dynamic server-synced registries, then optional custom component names when a modern custom component registry was confirmed.
- Dynamic registry handling:
  - Accepts modern Configuration `RegistryData` packets and preserves entry names plus optional NBT data in arrival order; reverse lookup uses that order as the assigned ID.
  - Stores dialog registry tags from modern `UpdateTags` packets and a peer-confirmed custom-component-registry flag.
  - Exposes name/ID/data/length queries and a `DimensionType` parser that reads min_y, height, logical_height, has_skylight and modern default_clock data from synchronized NBT.
  - Vanilla defaults are hardcoded for Nether (0..256, no skylight, no clock), End (0..256, skylight and End clock) and other names (Overworld-like -64..320, skylight/Overworld clock). Section count is height/16. `ceil_log2` computes bits needed for a global palette.
- Tests read: parses built-in protocol IDs, dynamic registry arrival order, derives dimension vertical range/skylight/default clock, and checks ceil-log2 boundaries. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical. Registry IDs define the meaning of item/entity/block/biome/effect/sound IDs in packets and chunk palettes; wrong registries can make otherwise well-formed packets semantically invalid.
- 1.20.1 implications:
  - The built-in report is pinned to 26.3 and must be regenerated from a 1.20.1 vanilla reference. Every numeric ID used in packets, palettes, particles, entities, item stacks, sounds, argument parsers and attributes must follow 1.20.1 registry ordering.
  - The current dynamic registry design expects `RegistryData` packets in a separate Configuration state. 1.20.1 does not have that state; its dimension codec/registry information is delivered in the older Login/Join Game path (with later tags delivered in Play). The source and data model must be adapted to populate registries from those older packet structures.
  - Current custom component registry and dialog tags are modern 26.3 concepts and must not be required for 1.20.1. Components and dialog lookups need to be removed or version-gated in the target path.
  - `default_clock` is a modern dimension data feature, not a 1.20.1 dimension field. Basic min_y/height/logical_height/has_skylight handling is conceptually relevant, but the exact target dimension NBT schema and defaults must be sourced from 1.20.1.
  - A fallback `DimensionType::vanilla` that treats every unknown dimension as Overworld can silently misread modded/custom dimensions; the custom 1.20.1 dimension codec must be read rather than relying on this default.
- Decision: **adaptable** as registry name/ID lookup and data-storage architecture; **rebuild registry inputs and rewire dynamic registry acquisition** for 1.20.1's Login/Play packet flow. Exclude modern dialog/component/default-clock semantics from the target.
- Dependencies / cross-references: `registries.json`; `packets::configuration::{RegistryData,UpdateTags}`; `item::Names`; `item::custom`; `horizon-world::nbt`; `packets/play.rs`; `chunk.rs`; `horizon-prepare` datagen; `horizon-core::version`.
- Follow-up checks: inspect `Join Game`/login packet in the target version and data-driven dimension registry serialization; map all 1.20.1 built-in network registry reports; ensure no modern-only registry IDs leak into item/chunk/entity packet decoding.
- RustChunk implementation mapping: Maps to 1.20.1 block/biome/dimension registries shared by future protocol and world systems. Existing verified facts/tags provide only part of the required registry pipeline.
- Evidence required before adoption: Require complete version-pinned registry names/IDs/defaults and dynamic registry payloads; prove consistent IDs across resource export, local world and protocol decoding with Java-oracle tests.
- Completion state: complete source/test review; no registry reports, dynamic packets, 1.20.1 world registry or protocol tests were run.

### 52. `crates/horizon-protocol/src/entity_data.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `34f9fb132a3ea8daa1faffb60bf458f260aa615d`.
- Size: 27,001 bytes; 754 lines; fetched in consecutive ranges 1–400 and 401–754.
- Coverage: lines 1–754, complete; both ranges returned the same blob SHA; all entity metadata and particle codecs/tests read.
- Purpose: decodes/encodes `ClientboundSetEntityDataPacket` synchronized entity metadata values using serializer IDs, plus modern particles, position sources and resolvable player profiles.
- Implementation:
  - `EntityDataSerializers` table is hardcoded in 26.3 registration order with IDs 0–43: byte/int/long/float/string/components/optional components/item stack/boolean/rotations/positions/directions/optional entity references/block states/particles/villager data/pose/variant enums/optional global position/vector/quaternion/resolvable profile/humanoid arm/dye color.
  - Each metadata record is index byte, serializer VarInt, serializer-specific value; `0xFF` terminates the list. Unknown serializer IDs have no known value length, so decoder stops and discards the rest while retaining preceding entries.
  - Metadata values cover primitives, NBT components, item stacks, vectors/quaternions, optional positions/UUID/global positions, registry IDs and multi-ID values, particles/lists and modern profile/skin data.
  - Particle payloads are decoded by hardcoded 26.3 `particle_type` registry IDs and per-type payload schemas (block state, colors/scales/power, delay, trail, item, geyser, vibration, etc.). A negative sentinel is a Starling native-mod extension, not vanilla.
  - `Profile` decodes modern full/partial ResolvableProfile and skin patch data, including properties and body/cape/elytra texture overrides.
- Tests read: primitive/metadata round trips and `0xFF` terminator; unknown serializer truncation; vectors/quaternions big-endian floats; representative particle types and particle lists; optional global positions; full/partial resolvable profiles and preserving following metadata entries. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: critical. Entity metadata is required for mob/player state, equipment, poses, effects, particles, variants and other entity rendering/behavior. It is versioned both by serializer ID and nested registry/item codecs.
- 1.20.1 implications:
  - Serializer IDs and serializer roster are version-specific. The current 26.3 order includes many later types (numerous animal/copper variants, resolvable profile, quaternion/vector3, etc.) and cannot be applied to 1.20.1. A new 1.20.1 table and serializer codec set must be taken from that version's vanilla source.
  - The item serializer uses modern `Stack`/component data, whereas 1.20.1 metadata item values use the older item-stack/NBT wire format. Component-based items and `ResolvableProfile` skin patch fields are not direct 1.20.1 equivalents.
  - Particle IDs and payload shapes use current `minecraft:particle_type` order; numeric IDs and data-bearing particle variants must be regenerated for 1.20.1. Unknown IDs are dangerous because the decoder cannot know how many bytes to skip; it stops the metadata list rather than recovering subsequent entries.
  - Components as NBT and simple primitives are useful concepts, but exact metadata serializer values, optional sentinels, registry IDs and nested NBT need target-specific codecs.
  - The negative mod-particle sentinel is a Starling extension and cannot be sent to ordinary vanilla 1.20.1 servers. Keep it strictly outside vanilla wire data or omit it from the target path.
- Decision: **reuse the generic metadata-list mechanism and primitive codecs**, but **replace serializer IDs, value codecs, particle IDs/data, item/profile handling and registry bindings** with a 1.20.1-specific implementation. Do not share the current numeric serializer table.
- Dependencies / cross-references: `packets/play.rs::SetEntityData`; `buf`; `item::{Slot,Stack,Template}`; `packets/login::Property`; `Tag`; `registry` particle/entity/block/item IDs; `MOD_PARTICLE_ID`.
- Follow-up checks: extract 1.20.1 `EntityDataSerializers` registry order and serializer read/write methods; create exhaustive version-specific metadata and particle fixtures; verify unknown-ID handling and metadata termination against vanilla.
- RustChunk implementation mapping: Maps to future 1.20.1 entity metadata decoding and the native entity model/render bridge. It is not an existing RustChunk entity runtime.
- Evidence required before adoption: Extract the exact 1.20.1 serializer order/types and particle payloads; create exhaustive metadata fixtures, unknown-ID tests and vanilla comparison traces for representative entities.
- Completion state: complete source/test review; no metadata, particle or 1.20.1 wire tests were executed.

### 53. `crates/horizon-protocol/src/item.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `37ee9ee60dbcea5170b6a63105d901a93eb17a08`.
- Size: 72,127 bytes; 2,006 lines; fetched in consecutive ranges 1–400, 401–800, 801–1,200, 1,201–1,600, 1,601–2,000 and 2,001–2,006.
- Coverage: lines 1–2,006, complete; all six ranges returned the same blob SHA; all codecs, helpers and unit tests read.
- Purpose: implements the current 26.3 network ItemStack data-component patch format, optional/delimited item slots, item-stack templates, hashed inventory synchronization and many typed/opaque component codecs.
- Core wire model:
  - Declares 122 `minecraft:data_component_type` IDs (0–121) in 26.3 registry order. The list includes component types that did not exist in 1.20.1, such as item model, use/weapon/equipment/attack range, death protection, block transformers, many mob variant components, sign text and newer entity/animal-related components.
  - A current non-empty `Slot` is encoded as count VarInt, item registry ID VarInt, number of added components, number of removed component IDs, then each component ID/value and removed ID. Empty is count 0.
  - `DelimitedSlot` uses the modern component patch with each positive component value length-prefixed, intended for untrusted/forward-compatible creative-slot input. `Template` serializes nested item stacks with item/count followed by data-component counts/values.
  - `HashedSlot` is a separate modern format: presence boolean, item/count, added component IDs with 32-bit hashes, and removed component IDs. `WirePatch` preserves original patch bytes for untouched stacks and can query/decode a stored patch.
  - Nested stacks are bounded to depth 64. Bundles, charged projectiles, containers, pot decorations and use remainder can recursively contain stack templates.
- Component codec coverage:
  - `Component` models simple integer/unit/bool/text/NBT/identifier/lore/enchantments, potion data, profiles/skin patches, custom model data, tooltip display, writable/written books, nested stacks, fireworks, lodestone trackers, trims, stew effects, instruments, banner patterns and painting variants.
  - Many other 26.3 components are read with `skip.rs` and preserved as raw bytes; custom component types can be represented as NBT via `custom` support. Unknown component IDs are errors because their body length is not known in this non-delimited format.
  - Specialized readers include registry-holder IDs, potion effects/name, component profile, filters on book content, fireworks, adventure predicates, tool/weapon/consumable/equippable data, container contents, sign text, entity/block data, etc. Several are kept opaque to preserve byte identity.
- Tests read: empty and common stack round trips; potion/firework/book/bundle/charged-projectile/profile/trim/banner/instrument components; raw component preservation; unknown component refusal; hash/patch-related behavior; nesting limits; item-specific equipment and consumable/tool/weapon payloads; painting direct-vs-reference cases. Tests not executed.
- Critical 1.20.1 finding:
  - Minecraft 1.20.1 predates the data-component item overhaul (introduced in the 1.20.5 line). Its wire `Slot` representation is the older presence/item ID/count/optional-NBT stack format, not this count-first modern component patch.
  - Therefore `Slot`, `DelimitedSlot`, `Template`, `HashedSlot`, `WirePatch`, `Stack` component collections and the whole 122-ID component registry cannot be reused as the 1.20.1 item wire model.
  - 1.20.1 item metadata lives in NBT tags with older semantics; container clicks/creative slots/merchant offers/advancement icons/equipment/chunk block entities must use the old slot codec wherever they carry items. The new hashed-stack synchronization model is also later-version protocol behavior and must not be emitted to 1.20.1.
  - The generic concepts of item ID/count, NBT preservation, nested item containers, book pages, potion metadata, enchantments, trims and custom data may still be needed by the game, but need translation to the target's older NBT structure and older registry IDs, not modern component IDs.
  - Registry-dependent component payloads and item stack hashes depend on modern registry ordering. Reusing their numbers or hash calculation against 1.20.1 would misinterpret items.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: **critical / blocking for item, inventory and interaction networking**. This is an explicit structural incompatibility, not a small compatibility gap. The project needs a dedicated 1.20.1 item stack codec and item metadata model before normal inventory, crafting, containers and item use can interoperate.
- Decision: **do not adapt current item.rs by editing component IDs**. Implement a separate legacy 1.20.1 ItemStack/NBT codec and version-specific inventory synchronization. Reuse the safe Reader/Writer, NBT library, nesting protections and selected semantic helper concepts only where matching 1.20.1 behavior is demonstrated. Keep modern item components for a future 1.20.5+ version, not in the 1.20.1 path.
- Dependencies / cross-references: `component` table; `Component`; `Stack`; `Slot`; `DelimitedSlot`; `Template`; `HashedSlot`; `WirePatch`; `custom.rs`; `hash.rs`; `skip.rs`; `buf`; `registry`; `packets/play.rs` item-bearing packets; `packets/recipes.rs`; `entity_data.rs`.
- Follow-up checks: inspect `hash.rs`, `skip.rs`, `custom.rs` and Play packet consumers. Then source 1.20.1 FriendlyByteBuf/ItemStack read/write semantics and implement byte-golden tests for empty/ordinary/enchanted/damaged/named/potion/book/container/map/banner/shulker stacks; verify container clicks and creative slot packets with vanilla 1.20.1.

- 1.20.1 implications: No separate 1.20.1-specific implication is recorded in this entry. Treat the version-related caveats above as unverified until checked against Minecraft Java 1.20.1.
- RustChunk implementation mapping: Maps to a future legacy 1.20.1 ItemStack/NBT model shared by inventory, world drops and protocol. Do not reuse the modern Data Components wire model.
- Evidence required before adoption: Verify empty/non-empty slot and ItemStack/NBT encodings byte-for-byte against 1.20.1; test counts, unknown item IDs, nested NBT, limits and inventory packet round trips.
- Completion state: complete source and unit-test review; no item codec tests, 1.20.1 serialization, network interaction or game session was run.

### 54. `crates/horizon-protocol/src/item/custom.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `1ae7bd8166a692f4ccbefd1d9e2ec0d76ab4d06b`.
- Size: 6,301 bytes; 188 lines; fetched as a complete file.
- Coverage: lines 1–188, complete.
- Purpose: internal extension for registering and negotiating custom namespaced data-component types in the modern item data-component system.
- Implementation:
  - A process-wide `OnceLock` stores a starting numeric ID and sorted custom component names. `install` verifies a nonnegative/non-overflowing ID range, validates namespaced identifiers and forbids the `minecraft` namespace, sorts names, rejects duplicates and can run only once.
  - Provides numeric ID↔name lookup and enumerates installed custom component entries.
  - `validate` rejects NBT End, enforces network NBT quota/parse behavior, and checks that a value survives disk-NBT save/reload without normalization changes (including NaN/negative-zero canonicalization).
  - `permitted` recursively scans a Stack's added/removed component IDs, nested stack lists, tooltip hidden IDs and selected raw nested-stack component payloads. It disallows custom components unless the connection has negotiated them; if no custom types are installed it allows ordinary stacks.
  - Documentation explicitly says this is an internal transport seam, not a native-mod API.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: low for the active 1.20.1 milestone. It shows how Starling extends modern component registries without colliding with vanilla IDs, but the entire feature assumes the newer data-component item model.
- 1.20.1 implications:
  - Minecraft 1.20.1 predates item data components and this 26.3 custom component registry mechanism. There is no equivalent component-ID namespace to install in its normal vanilla item-stack wire format.
  - The recursive scanner's assumptions (`Stack.added`, `Stack.removed`, `Template`, component constants, TooltipDisplay and modern nested component payloads) are inseparable from the modern `item.rs` model. It cannot be adapted by changing `first` or names.
  - NBT canonicalization checks and strict identifier validation are general safety concepts that may be useful elsewhere, but the custom data-component negotiation itself should not be inserted into a vanilla 1.20.1 protocol flow.
- Decision: **unnecessary for the current 1.20.1 vanilla protocol milestone**. Defer or omit this feature until a future item extension design exists; reuse only generic NBT validation ideas if required by a separate 1.20.1 feature.
- Dependencies / cross-references: `item.rs` component registry and Stack model; `Component::Stacks`; `Template`; `horizon_world::identifier`; network NBT APIs and `NBT_QUOTA`; registry `SyncedNames`; negotiation state.
- Follow-up checks: inspect whether any current client startup path unconditionally calls `custom::install`, and ensure the future 1.20.1 branch does not require component negotiation or custom-component metadata to complete login or item decoding.
- RustChunk implementation mapping: No base 1.20.1 mapping. This modern custom data-component registry is not a vanilla 1.20.1 feature; only generic NBT validation ideas may inform a separately scoped extension later.
- Evidence required before adoption: Confirm all references to data-component registration/negotiation are absent from the vanilla 1.20.1 core. Any later extension requires a distinct spec and compatibility/security tests.
- Completion state: complete source review; no custom component registration, NBT validation, negotiation or protocol test was run.

### 55. `crates/horizon-protocol/src/packets/recipes.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `dcbe5ffa0e1e81369a5c9e163eb0e24676a6dd9e`.
- Size: 19,442 bytes; 634 lines; fetched in consecutive ranges 1–400 and 401–634.
- Coverage: lines 1–634, complete; both ranges returned the same blob SHA; tests read.
- Purpose: codecs for the modern recipe book display system, including recursive slot displays, recipe display variants, recipe book add/remove/settings, ghost recipes and place-recipe requests.
- Implementation:
  - `HolderSet` represents either a named registry tag or a list of item registry IDs.
  - `SlotDisplay` models empty/fuel/potion/component-filtered/item/item-stack/tag/dyed/smithing-trim/remainder/composite display nodes, with a nesting guard of 32 levels.
  - `RecipeDisplay` has five variants: shapeless, shaped, furnace, stonecutter and smithing. Shaped display validates that width×height equals ingredient count.
  - Recipe book add entries carry display ID, display payload, group, recipe-book category registry ID, optional ingredient requirements and flags. Separate codecs encode recipe display removals, four recipe-book (open/filtering) states, ghost recipe display and serverbound place-recipe (container/display ID/use-max).
  - The module's header explicitly states it encodes the 26.3 `minecraft:recipe_display` and `minecraft:slot_display` registries in their current ID order.
- Tests read: round trip for shaped display, tag and item stack, nested smithing display, trimming, component-filter and composite structures; not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: moderate for recipe UI/network planning, but high version mismatch. The Rust game may still need a recipe-book UI, yet these are modern protocol-specific display payloads, not the historic recipe representation.
- 1.20.1 implications:
  - The current wire model depends on later `recipe_display` and `slot_display` registries, their IDs, and modern display-specific variants such as `OnlyWithComponent`, `WithRemainder`, and richer registry-holder trim representations. They cannot be sent to 1.20.1.
  - The 1.20.1 recipe-book packet format uses the older recipe book packet model and recipe identifiers/collections rather than this modern display tree. Its add/remove/settings/ghost/place semantics must be reconstructed from the target version's vanilla packet classes.
  - `HolderSet`, generic recursive display nodes and primitive codecs may be useful domain structures for a future renderer/recipe UI, but the current `Stack` reference itself uses the modern data-component item format; it must not leak into legacy item wire encoding.
- Decision: **rewrite the recipe networking codecs for 1.20.1**. Retain only generic codec patterns and possibly recipe UI concepts after translating them into the older recipe model. Treat current recipe display/slot display IDs as unusable target-version data.
- Dependencies / cross-references: `item::{Stack,Template}`; `buf::{Codec,Reader,Writer}`; `horizon_world::nbt::Tag`; `packets/play.rs` recipe book variants; `ids::play`; recipe and item registries; `recipe_display`/`slot_display` data.
- Follow-up checks: inspect 1.20.1 serverbound/clientbound recipe-book packet classes and serializers; trace recipe definitions, ingredient encoding and recipe identifiers; implement and compare legacy recipe-book add/remove/settings/ghost/place packets against a vanilla 1.20.1 oracle.
- RustChunk implementation mapping: Maps to a future 1.20.1 recipe/recipe-book model and packet codecs, after the base gameplay/inventory model exists. Current recipe display IDs and structures are not reusable data.
- Evidence required before adoption: Compare 1.20.1 recipe serializers, ingredients, identifiers and recipe-book actions to vanilla source; require client/server golden vectors and functional book/update tests.
- Completion state: complete source/test review; no tests, packet traces or 1.20.1 recipe-book interoperability run was performed.

### 56. `crates/horizon-protocol/src/crypto.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `540aeb5eb9fb078b7e114dd211fc73f3711829d3`.
- Size: 11,763 bytes; 327 lines; fetched as a complete file.
- Coverage: lines 1–327, complete, including tests.
- Purpose: implements Minecraft connection AES/CFB8 encryption, secure random shared-secret generation, signed Java SHA-1 server hashes, offline-mode UUIDs and UUID formatting.
- Implementation:
  - `Cfb8` wraps the crate's AES-128 forward cipher with a 16-byte shift register; encryption/decryption preserve stream state across calls. It documents Minecraft's use of the shared secret as both key and IV.
  - Decryption uses parallel AES block lanes while preserving the byte-by-byte CFB8 register semantics.
  - `random_bytes` uses the OS cryptographic RNG; `generate_secret` returns a random 16-byte AES key.
  - `java_hex_digest` interprets a digest as a signed two's-complement BigInteger and formats lower-case hexadecimal exactly like Java, including negative values and no leading zeros.
  - `server_hash` computes SHA-1 over ISO-8859-1 server ID bytes (unrepresentable Unicode replaced with '?'), shared secret and encoded server public key.
  - Includes an internal MD5 implementation and `offline_uuid` using the Java name-based UUID v3 algorithm over `OfflinePlayer:<name>`; sets version and IETF variant bits. `uuid_string` formats the 8-4-4-4-12 UUID string.
- Tests read: FIPS-197 AES-128 vector; NIST CFB8 vector across split updates; AES lanes compared with single-block and Rust aes crate; CFB8 decryption against one-byte reference under different read cuts; signed Java hash vectors for Notch/jeb_/simon; RNG secrets differ; MD5 and offline UUID vectors. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high and largely version-independent. These are foundational cryptographic and identity primitives needed by Java Edition login, with an offline UUID helper for offline-mode identity.
- 1.20.1 implications:
  - AES-128/CFB8, secure random 16-byte secret, SHA-1 server hash and Java-compatible signed hexadecimal formatting are longstanding Java Edition connection mechanisms and are likely reusable, subject to confirming the exact 1.20.1 encryption exchange ordering and encoding.
  - Offline UUID v3 derivation is a general Minecraft Java convention and is likely reusable for offline-mode player identity. It is not equivalent to Microsoft authentication and does not grant online-mode access.
  - The newer client protocol's login packet flow may differ, but the low-level CFB8 primitive remains independent of whether the login packets use older or newer packet IDs.
  - The AES implementation relies on hardware/architecture-aware internals (`blocks.rs`); the ARM64 Android target needs actual performance/correctness validation. This file itself does not establish the safety of every platform backend.
- Decision: **largely adaptable/reusable** for 1.20.1 cryptographic primitives and offline UUID derivation. Keep key exchange and login state ordering version-specific; audit platform AES backend separately.
- Dependencies / cross-references: `crypto::blocks`; `LANES`; `getrandom`; `sha1`; `horizon-protocol::connection::enable_encryption`; `rsa.rs`; `packets/login.rs`; authentication/session service; offline account identity.
- Follow-up checks: inspect `blocks.rs` and `rsa.rs`; trace the 1.20.1 Encryption Request/Response sequence and signed server hash inputs against vanilla; verify ARM64 Android AES backend and OS RNG availability.
- RustChunk implementation mapping: Maps to future online-mode login cryptography and offline UUID derivation, not to local world simulation. Select only primitives whose exact 1.20.1 usage is verified.
- Evidence required before adoption: Run published algorithm vectors and 1.20.1 login/server-hash vectors; review RNG, key handling and ARM64 AES dispatch; verify successful online-mode login without leaking secrets.
- Completion state: complete source/test review; no crypto test, login exchange, RNG probe or ARM64 device test was executed.

### 57. `crates/horizon-protocol/src/rsa.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `479f4d64254b8037f4ef49fee0314dedd564a242`.
- Size: 35,306 bytes; 834 lines; fetched in consecutive ranges 1–400, 401–800 and 801–834.
- Coverage: lines 1–834, complete; all three ranges returned the same blob SHA; all RSA/DER/prime/signature tests read.
- Purpose: pure-Rust RSA key parsing, public-key encryption, private-key decryption/signing, PKCS#1 v1.5 signature verification, Montgomery modular arithmetic and RSA key-pair generation.
- Implementation:
  - Custom arbitrary-width positive integers are represented as little-endian 32-bit limbs. Operations include big-endian conversion, multiplication, small multiply/add, division by a small integer, compare/subtract and bit access.
  - Montgomery multiplication/exponentiation implements modular exponentiation for odd RSA moduli.
  - A minimal DER parser decodes SubjectPublicKeyInfo X.509 RSA public keys, PKCS#8 private keys and bare PKCS#1 private keys. It accepts the rsaEncryption OID and validates minimum modulus size/oddness.
  - `RsaPublicKey::encrypt` implements RSAES-PKCS1-v1_5 with random nonzero type-2 padding; `verify` implements RSASSA-PKCS1-v1_5 for SHA-1 and SHA-256 by checking the expected encoded DigestInfo block.
  - `RsaPrivateKey` can parse keys, generate RSA key pairs (Miller-Rabin with 40 rounds; public exponent 65537), decrypt PKCS#1 v1.5 blocks and sign SHA-1/SHA-256 messages. Signing checks its own output with the public verifier before returning it.
  - Public keys are encoded as X.509 SubjectPublicKeyInfo. The test support includes 1024-bit and 2048-bit DER vectors and known OpenSSL signatures.
- Tests read: Montgomery small-number vectors, X.509 key parsing, PKCS#1 encryption/decryption and random padding, generated-key DER round trips, primality checks, OpenSSL SHA-256 signature validation, SHA-1/SHA-256 signature generation parity, PKCS#8 and PKCS#1 key parsing. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high. RSA is used in online-mode login (encrypting the shared secret/challenge with the server key) and in signed-chat/profile-key validation and signing. The exact subfeatures required depend on the 1.20.1 account/authentication mode.
- 1.20.1 implications:
  - RSA/PKCS#1 v1.5, X.509 public key parsing, SHA-1/SHA-256 DigestInfo and PKCS#8 chat-key parsing are not tied to protocol 777 and are potentially reusable for 1.20.1 login and secure-chat support.
  - Packet placement, required key sizes, key trust chain, encryption request/response and signed-chat message format are version-specific and must be wired through the 1.20.1 Login/Play state, not the modern packet schemas.
  - Pure-Rust private exponentiation uses variable-time control flow (square-and-multiply branches on exponent bits) and big-integer arithmetic not designed as constant-time cryptography. Private-key decryption/signing should not be exposed to hostile timing observations without a security review; a vetted constant-time cryptographic implementation may be preferable for production.
  - The DER helper returns trailing input slices, while some nested parsers ignore them. Exact strictness/trailing-byte rejection and bounds should be fuzzed; malformed public/private key input is attacker-controlled in login/profile verification paths.
  - Key generation is substantial custom cryptographic code. The client generally needs to consume a server public key and a service-issued profile key; generating a server key pair is not required merely to connect to a server.
- Decision: **cryptographic standards and data model are adaptable**, but **version-specific login/chat wiring and security review required**. Prefer vetted constant-time implementations for private operations where feasible; do not treat test vectors as a complete security audit.
- Dependencies / cross-references: `crypto.rs`; `secure_chat.rs`; `packets/login.rs`; `packets/play.rs` chat/session packets; `rsa::SignatureHash`; `getrandom`; SHA-1/SHA-256; `Cfb8`; service-signed profile public keys.
- Follow-up checks: inspect `secure_chat.rs` and authentication call sites; determine 1.20.1's exact signed-chat/profile-key requirements, key sizes and trust chain; review private-key timing behavior, malformed DER handling and maximum key lengths; consider replacing private operations with a vetted crypto crate.
- RustChunk implementation mapping: Maps only to the cryptographic support needed by future online login/profile-key verification. Prefer vetted constant-time libraries over carrying custom private-key arithmetic without review.
- Evidence required before adoption: Require independent cryptographic review, standard RSA test vectors, key-size/format validation, constant-time assessment where applicable and a full 1.20.1 login/profile-key interoperability test.
- Completion state: complete source/test review; no RSA key generation, private operation, login exchange, signature verification test or security assessment was executed.

### 58. `crates/horizon-protocol/src/secure_chat.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `1c54eab44c70e4f1467e855d089efef51c8a4038`.
- Size: 41,559 bytes; 1,076 lines; fetched in consecutive ranges 1–400, 401–800 and 801–1,076.
- Coverage: lines 1–1,076, complete; all three ranges returned the same blob SHA; all validation, chain, last-seen and cache tests read.
- Purpose: implements modern secure-chat profile-key validation, message-signature payload construction, per-player signed message chains, incoming message validation, last-seen acknowledgement tracking, signature cache compression and local chat signing.
- Implementation:
  - A profile key contains expiry, DER public key and Mojang services signature. `signed_by` validates SHA1withRSA over profile UUID + expiry + public-key bytes; expiry and an 8-hour grace period are modeled separately.
  - `ChatSession::validate` parses the X.509 key and checks service signature. `data()` builds the modern player-info session tuple.
  - `signed_payload` constructs a versioned binary message payload from a version marker, sender UUID, session UUID, chain index, salt, timestamp in epoch seconds, UTF-8 content length/content and last-seen signatures.
  - `Chain` verifies incoming signatures using SHA256withRSA, validates sequence/order/timestamp and expires or breaks the chain on invalid content/signature/order. Unsigned fallback depends on secure-chat enforcement.
  - `MessageValidator` supports accept-unsigned, reject-all and key-based modes; key-based validation checks expiry/grace, message signature and that a message repeats the last accepted one or descends from the same sender/session chain.
  - Java `Arrays.hashCode(byte[])` and `LastSeenMessages.computeChecksum` behavior are reproduced. `LastSeenValidator` tracks pending messages and checks the 20-bit window, offset, acknowledgement validity and optional checksum.
  - `SignatureCache` maintains 128 recent signatures, packs known signatures by cache index and otherwise includes the full 256-byte signature.
  - `Encoder` signs the next local message chain link with SHA256withRSA and emits a fixed-size signature when the private key can produce one.
  - `LastSeenTracker` tracks the client's last 20 accepted/hidden/deleted messages and emits acknowledgement bitsets, offsets and checksums.
- Tests read: payload field byte layout and timestamp flooring; unsigned/enforced chain handling; checksum behavior; window bounds and invalid acknowledgements; packed signature cache; signing/decoding interop; last-seen tracker and validator agreement including holes; known external service/chat signature vectors; key expiry/grace and broken-chain behavior. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high for an eventual normal 1.20.1 multiplayer experience, since secure chat and profile public keys existed before 1.20.1. However, this code is a modern 26.3 implementation and is not a wire-compatible 1.20.1 chat subsystem without adaptation.
- 1.20.1 implications:
  - RSA/SHA1 profile-key trust validation, SHA256withRSA message signature verification, 256-byte signatures and the general principle of tracking last-seen signed messages are potentially relevant to 1.20.1.
  - The current signed payload explicitly includes a modern version marker, sender/session UUIDs and indexed chain link, while the Play packets in this tree carry modern chat/session updates, last-seen update structure, chat acknowledgement and signed argument data. The 1.20.1 message signature payload and packet sequence must be derived from that version's vanilla code; the current payload bytes cannot simply be reused.
  - Current LastSeen update is `(offset, 3-byte 20-bit acknowledged mask, checksum)` and signature caching uses a modern packed format. Validate the exact 1.20.1 representation/window semantics before sharing these structures.
  - Current Login/Play/session packet structures for initializing chat keys differ across protocol generations. The key service payload may retain a common shape, but the session registration and message-chain lifecycle must be version-specific.
  - The 5-minute server message expiry and 8-hour key expiry grace are hardcoded from the current vanilla implementation; they must be checked against 1.20.1's corresponding constants and not copied without verification.
- Decision: **adapt selected cryptographic primitives and abstract concepts**, but **rewrite 1.20.1 signed payload, login/player-info key exchange, chain protocol, last-seen updates and packet integration** from target-version vanilla. Keep modern 26.3 state machine isolated from the 1.20.1 path.
- Dependencies / cross-references: `rsa::{RsaPrivateKey,RsaPublicKey,SignatureHash}`; `packets::play::LastSeenUpdate`; modern Login/Play chat packets; profile public key/session; chat message signing; `crypto.rs`.
- Follow-up checks: inspect `packets/play.rs` chat structs and `packets/login.rs` against exact 1.20.1 source; establish profile key signature payload, message signing payload, session/index behavior, last-seen window, acknowledgement and enforcement modes using a 1.20.1 oracle/server.
- RustChunk implementation mapping: Maps to future 1.20.1 secure-chat signing, profile-key and message-chain handling in protocol/game UI. It must be rewritten for 1.20.1 rather than inherited from the modern implementation.
- Evidence required before adoption: Compare signed payloads, session/profile keys, last-seen acknowledgements and chat packet behavior against official 1.20.1 Java source; test valid, invalid, expired and unsigned-message cases with a server.
- Completion state: complete source/test review; no signature validation, chat signing, secure-chat session, or vanilla 1.20.1 message exchange was run.

### 59. `crates/horizon-protocol/src/resolver.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `f9ed010aa125f0588894d55e9cf2615a6f3d4635`.
- Size: 7,040 bytes; 209 lines; fetched as a complete file, including tests.
- Coverage: lines 1–209, complete.
- Purpose: resolves Minecraft server addresses, including the standard `_minecraft._tcp` SRV redirect for the default port, then resolves the target host to socket addresses.
- Implementation:
  - SRV lookup is attempted only for port 25565 and hostnames that are not IP literals or `localhost`. Query name is `_minecraft._tcp.<host>.`, avoiding a duplicate trailing dot for fully qualified names.
  - An SRV result replaces both the host and port for login connection. It deliberately performs no second SRV query for the target and does not fall back to the original host if the redirected target cannot resolve.
  - If no SRV record/query result is available, the original host/port is used for normal address resolution. Explicit non-default ports and IP literals bypass SRV.
  - `server_redirect` returns the selected host/port without resolving socket addresses, for display/use by server-list or launcher-facing code.
  - `NAME_SERVERS` can be supplied once through a callback. On Android the source is designed to query the active network's DNS servers from the Android activity on each lookup, so Wi-Fi/mobile network changes are reflected. Without a callback, the DNS module finds system name servers.
- Tests read: redirect and explicit port behavior; target resolution with no original hostname lookup; missing record fallback; IP/localhost bypass; FQDN trailing dot; no fallback from an SRV target that fails to resolve; direct literal TCP connection. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high for real multiplayer connectivity and especially useful on Android, where DNS configuration may not be exposed through a normal `resolv.conf` path.
- 1.20.1 implications:
  - Minecraft Java SRV discovery on the default server port is a long-standing client behavior and is likely reusable for 1.20.1. It is independent of packet schema/version numbers.
  - The login handshake must carry the resolved target host and port, while the UI/status display may preserve the originally entered hostname; this code explicitly separates that concern.
  - Android's active-network DNS provider hook is highly relevant to a native Android client; it must be connected to the actual Android network APIs and refreshed on network changes.
  - The DNS packet implementation and resolver timeout/error policy live in `resolver/dns.rs`, which still needs full review. Correct SRV parsing and DNS response validation are essential; never trust DNS target names as packet payload instructions.
- Decision: **largely adaptable/reusable** for 1.20.1 and Android. Keep the SRV selection rules and injected DNS provider; verify target-version behavior and ensure the Android host supplies current DNS servers.
- Dependencies / cross-references: `resolver::dns`; `NAME_SERVERS`; `dns::system_name_servers`; `dns::query_srv`; `connection::connect_server`; `connect_server_with_address`; Android activity/network DNS callback.
- Follow-up checks: inspect `resolver/dns.rs`, DNS response parsing, retry/timeout and name-server selection; verify 1.20.1 server redirect behavior against vanilla. Confirm Android integration fetches DNS servers from the current active network without stale caching.
- RustChunk implementation mapping: Maps to a later multiplayer server-address resolver and connection bootstrap. It is not used by the first offline single-player milestone.
- Evidence required before adoption: Test direct host resolution, SRV target/port selection, IPv4/IPv6, malformed DNS and timeouts on desktop and Android; compare SRV behavior with vanilla client.
- Completion state: complete source/test review; no live DNS query, SRV lookup, Android DNS callback or connection test was run.

### 60. `crates/horizon-protocol/src/resolver/dns.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `3266607c4cd93ae6210918d679960b9253d9841a`.
- Size: 13,214 bytes; 357 lines; fetched as a complete file.
- Coverage: lines 1–357, complete, including all parser and local DNS integration tests.
- Purpose: small synchronous DNS client for Minecraft SRV discovery, modeled on the Java JNDI resolver behavior described in its module header.
- Implementation:
  - Uses SRV type 33/IN queries with recursion desired. Generates a random 16-bit transaction ID and encodes an absolute DNS name with label and total-name length bounds.
  - Name-server discovery: Unix reads `nameserver` lines from `/etc/resolv.conf`; Windows enumerates active adapters via `ipconfig` and filters scoped link/site-local IPv6 values; other targets return no default servers. Android is expected to supply live active-network DNS servers through the callback in `resolver.rs`.
  - Queries configured servers sequentially, one UDP attempt per server with a 1-second timeout. If UDP response is truncated, retries the same server over length-prefixed TCP. A malformed response or server failure advances to the next configured server.
  - DNS response parsing checks transaction ID, response flag, truncation and response code; NXDOMAIN is treated as no redirect. It skips the question section and walks answer records until the first SRV answer, returning the first record's target/port as the selected endpoint.
  - Compressed domain-name pointers must point backwards and are bounded to 128 iterations to prevent loops; label data must be valid UTF-8. TCP framing uses the DNS two-byte length prefix.
- Tests read: exact recursive SRV query bytes, first SRV answer selection, empty/NXDOMAIN behavior, malformed/short/wrong-ID/forward-pointer replies, truncated UDP detection, Unix resolver parsing, Windows scoped IPv6 filtering, sequential name-server fallback and UDP-truncated→TCP-complete retry using local sockets. Not executed.
- Implementation / logic: Detailed implementation and behavior are documented in this entry's implementation-specific subsections above.
- Findings: File-specific risks, limitations, and compatibility observations are documented in the corresponding subsections above.
- RustChunk relevance: high for Android and desktop server discovery. It has no dependency on a particular Minecraft protocol version and is a compact cross-platform SRV implementation, with an explicit Android DNS injection seam.
- 1.20.1 implications:
  - DNS SRV discovery for Minecraft server addresses is a client networking behavior independent of packet IDs; the implementation can likely be reused unchanged if 1.20.1 client behavior matches the documented rule (SRV for default port 25565, not explicit alternate ports or IP literals).
  - The Android callback is particularly important: relying only on `/etc/resolv.conf` may not represent the active Android network. It must be wired to current system DNS servers and updated when connectivity changes.
  - First-answer selection and the one-try-per-server timeout policy are intended to match the JNDI client, but DNS implementations and SRV priorities/weights can differ; confirm the desired vanilla behavior rather than adding random load balancing or selecting an arbitrary record.
  - The parser does not implement DNSSEC; SRV results remain unauthenticated DNS data, as is typical for this client discovery path. Transport to the redirected server still needs the normal Minecraft connection/security policy.
- Decision: **largely reusable/adaptable** for 1.20.1, including query, UDP/TCP fallback and platform DNS-source abstraction. Verify parser edge behavior and Android active-network wiring; no protocol-version-specific rewrite is indicated by this file.
- Dependencies / cross-references: `resolver.rs`; `getrandom`; `ipconfig` on Windows; `std::net::{UdpSocket,TcpStream}`; `NAME_SERVERS`; `SERVER_TIMEOUT`; SRV query target `_minecraft._tcp`.
- Follow-up checks: inspect Android network integration that calls `set_name_servers`; validate DNS timeouts and malformed response handling on Android; run resolver comparisons and a real SRV-enabled 1.20.1 server connection. Review whether DNS answer owner name/class/priority/weight handling is sufficiently faithful to the Java resolver.
- RustChunk implementation mapping: Maps to the DNS SRV implementation used by the future server resolver. Keep it platform-aware and independent of the game state machine.
- Evidence required before adoption: Test compressed DNS names, UDP/TCP fallback, malformed/truncated responses, injected Android DNS servers and timeouts; verify behavior on a real SRV-enabled 1.20.1 server.
- Completion state: complete source/test review; no DNS query, TCP fallback test, Android resolver call or live SRV resolution was executed.

### 61. `crates/horizon-protocol/src/attachment_sync.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `532c3854124c11910f9a1cc3e44f59e25e03a184`.
- Size: 7,213 bytes; 212 lines; full source fetched.
- Coverage: lines 1–212, complete.
- Purpose: Defines the network message used to synchronize native-mod synced-attachment values from a Horizon server to clients observing the holder.
- Implementation / logic: The clientbound channel is `horizon:attachment_sync`. A message carries the holder kind and its ID/position, a replace flag, and a list of namespace/name/value changes. Holder kinds are Entity, Player, BlockEntity, Chunk, and Level. Values are encoded as CompoundTag in network NBT form; `None` is represented by `Tag::End` and means removal. The payload is capped at 1 MiB and names at 256 UTF-16 code units. The decoder bounds the count by the remaining bytes and rejects trailing bytes.
- Findings: The code uses a compact, holder-kind-versioned wire format and has no serverbound counterpart. The encoder rejects unknown holder kinds, but it does not semantically validate empty change sets, namespaces, or identifier syntax. Replace semantics clear the client's entire synced-holder cache before applying the delta. Tests cover round trips for all five holder kinds, removal, payload and name limits, NBT depth, and trailing bytes; they were not run.
- RustChunk relevance: Attachment wire-protocol concepts and the NBT update pattern may be useful, but the native-mod framework is not part of RustChunk's current target. Payload limits and malformed-input defenses are useful patterns for the client protocol.
- 1.20.1 implications: The Attachment API and this channel belong to version 26.3; vanilla Minecraft 1.20.1 does not provide them. They must not be ported as a vanilla packet or data format. If extension-state synchronization is needed later, design a separate message with holder IDs, limits, replacement/patch semantics, and version negotiation.
- Decision: **Unnecessary for the base 1.20.1 milestone; bounded NBT-payload handling is a reusable pattern for a future extension protocol.**
- Dependencies / cross-references: horizon_world::attachments::{AttachmentSync,Holder}, horizon_world::nbt, protocol::{Reader,Writer}; docs/native-mods.md, server attachment hook/sync lifecycle.
- Follow-up checks: Later, inspect the client attachment-cache lifecycle and server dispatcher. For the RustChunk core, retain only the NBT network bounds and trailing-byte policy in the design notes.
- RustChunk implementation mapping: No vanilla 1.20.1 runtime mapping. Starling's native-mod synced-attachment channel is a custom extension and is excluded from the base client; retain only generic bounded-NBT lessons.
- Evidence required before adoption: Confirm the base protocol implementation emits/accepts no such custom packet. Any future extension must have an explicit negotiated namespace, schema, limits and compatibility tests.
- Completion state: complete source review; no tests, build, runtime session or network interoperability test executed.

### 62. `crates/horizon-protocol/src/hash_ops.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `98b3f143740726279bf85846863ea74b5cc468dd`.
- Size: 13,130 bytes; 352 lines; full source fetched.
- Coverage: lines 1–352, complete.
- Purpose: Implements HashOps and Guava CRC32C to produce canonical hashes of codec-encoded data-component values.
- Implementation / logic: CRC32C is generated with the Castagnoli table. HashOps assigns byte tags to primitive types, UTF-16-based strings, booleans, empty/map/list values, byte/int/long arrays, lists, and maps. Primitive values are little-endian; map entries are sorted by unsigned key hash and then value hash, while list order is preserved. `from_nbt` converts NBT structures into HashOps values, and `bool_keys` converts selected byte values to Boolean codec hashes. RecordHash omits optional fields and hashes the remaining fields like a map.
- Findings: This is neither a hash of raw NBT nor SHA-256; it hashes the exact HashOps representation of a 26.3 Codec and is used for HashedPatchMap/Data Components. Floats and doubles use their raw bits; strings use UTF-16 length and code units. A wrong codec type or boolean-field interpretation changes the result. Tests assert standard CRC32C vectors, primitive byte payloads, map/list ordering, NBT conversion, and sample values emitted by vanilla 26.3; they were not run.
- RustChunk relevance: Useful for understanding how modern item-component packets carry component hashes, but it is not directly usable in the vanilla 1.20.1 core.
- 1.20.1 implications: Minecraft 1.20.1 predates this generation's Data Components and HashedStack. Do not use this implementation as an ItemStack/NBT hash or integrity mechanism for 1.20.1. If RustChunk needs a canonical hash, define it for the exact serializer and purpose; for file integrity, use SHA-256 or another explicitly selected algorithm, not this CRC32C scheme.
- Decision: **Version-specific and unsuitable for protocol 1.20.1; retain only as a reference for implementing the newer Codec-hash scheme.**
- Dependencies / cross-references: horizon_world::nbt::Tag; crates horizon-protocol::item/hash.rs and modern Data Component codecs/HashedPatchMap; Guava HashOps reference.
- Follow-up checks: When porting the 1.20.1 ItemStack, omit the Data Component hash path entirely and compare against that version's own serialization and NBT behavior.
- RustChunk implementation mapping: No base 1.20.1 item mapping. This CRC32C/Data Component hash path is not a replacement for legacy ItemStack/NBT serialization or integrity checks.
- Evidence required before adoption: Verify the 1.20.1 item/network model has no dependency on modern component hashes; add regression tests ensuring legacy NBT parsing and item packets do not call this path.
- Completion state: complete source review; no tests, build, runtime session or network interoperability test executed.

### 63. `crates/horizon-protocol/src/mod_list.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `9fc777efbe513d3bd4576f7df8c554b1e657384c`.
- Size: 11,594 bytes; 348 lines; full source fetched.
- Coverage: lines 1–348, complete.
- Purpose: Implements the content-mod list and compatibility handshake between a Horizon server and client when entering a world.
- Implementation / logic: The `horizon:mod_list` channel is bidirectional during the Configuration phase: the server sends its list and the client returns its own. Each entry contains an ID, version, optional file SHA-256, and Side (`Both`, `Server`, or `Client`). A `LOCAL` OnceLock stores the local-mod list once; `set_local` sorts and truncates it. The shared list includes only `Both` mods. `compare` checks the presence and version of shared mods, and compares SHA-256 when both sides provide a hash. `Difference.reason` builds a refusal message with at most six names per group. The codec caps the list at 128 entries, text at 64 characters, and rejects trailing bytes.
- Findings: Client-only and server-only mods intentionally do not block join compatibility. Folder-based development mods are matched by version only when no file hash is available. Encoding silently truncates the list to 128 entries and strings to 64 characters, and `set_local` also truncates; oversized mod IDs/versions or too many entries can therefore lose information silently. Comparison is based on matching IDs; this file does not enforce uniqueness or namespace-conflict rules. Tests cover role separation, round trips, caps, order independence, mismatches, folder development mods, and truncated refusal reasons; they were not run.
- RustChunk relevance: This is a useful pattern for content-identity compatibility and hash-based mod parity on modded networks. Horizon's mod-side semantics themselves are outside vanilla 1.20.1 and the RustChunk base milestone.
- 1.20.1 implications: A vanilla 1.20.1 server has no such custom handshake, and vanilla clients do not recognize `horizon:mod_list`. RustChunk must implement the vanilla Handshake/Login/Play flow first and must not insert this channel into the standard sequence. If modded sessions are needed later, the handshake should be opt-in and compatible with vanilla clients.
- Decision: **Unnecessary for the vanilla 1.20.1 core; file-hash parity is a reusable concept for a future mod-compatibility layer.**
- Dependencies / cross-references: horizon:mod_list, protocol Reader/Writer, game/mods.rs LOCAL setter, configuration custom payload, docs/mods.md and content identity checks.
- Follow-up checks: Later, inspect the server/client dispatch for this channel and `content_identity.rs`. For 1.20.1, first complete the vanilla packet IDs and handshake independently; do not assume a Configuration state.
- RustChunk implementation mapping: No vanilla protocol mapping. The Horizon content-mod list/handshake is custom and must not be sent to ordinary 1.20.1 servers; possible future mod parity is separate.
- Evidence required before adoption: Capture a vanilla 1.20.1 handshake/login/play session and prove no custom mod-list packet is emitted. Any future extension needs explicit negotiation and independent server compatibility tests.
- Completion state: complete source review; no tests, build, runtime session or network interoperability test executed.

### 64. `crates/horizon-protocol/src/mod_net.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `75ba8b153b477f80b18d9b5e9943fafd8584c2ce`.
- Size: 4,833 bytes; 147 lines; full source fetched.
- Coverage: lines 1–147, complete.
- Purpose: Encodes arbitrary messages exchanged between the server and client halves of a Horizon native mod during the Play state.
- Implementation / logic: The channel is `horizon:mod_net`; its payload contains a channel string in `mod_id:name` form and a data byte array. The `minecraft` and `horizon` namespaces are prohibited. The mod ID and name are each limited to 64 bytes and validated with different character allowlists. Data is capped at 30,000 bytes. The encoder checks the channel and size before serialization, and the decoder validates the channel, byte array, and trailing bytes. The module header states that the server advertises support with an empty message during Configuration and the client sends messages during Play.
- Findings: This layer validates only the wire shape and channel syntax. Authorization, rate limiting (the documentation mentions 32 messages per tick per player), and checking whether the mod is actually installed are responsibilities of the caller/dispatcher. `MAX_DATA=30,000` is below the 32,767-byte payload ceiling. `split_once` plus the character allowlist rejects additional colons in the name. Tests cover round trips, the channel allowlist, maximum data size, and trailing bytes; they were not run.
- RustChunk relevance: Worth studying for future client/server module messaging, but this is a Horizon-specific extension channel, not part of vanilla Minecraft Java.
- 1.20.1 implications: Minecraft 1.20.1 has custom-payload packets, but the `horizon:mod_net` channel and its framing are not part of the official game protocol. Custom-payload IDs and registration in the core client must follow protocol 1.20.1. Keep this channel absent or disabled for now so the vanilla connection flow is not modified unnecessarily.
- Decision: **Unnecessary for the base milestone; the codec design and strict validation are reusable for a future extension protocol.**
- Dependencies / cross-references: protocol Reader/Writer; custom payload packet schemas in packets/configuration.rs and packets/play.rs; native mod net hooks; docs/native-mods.md API 40.
- Follow-up checks: In a later network-integration audit, inspect caller-side rate limits, mod identity checks, and enforcement of server/client message direction.
- RustChunk implementation mapping: No first-release mapping. Native-mod networking is not Forge networking and is outside the vanilla protocol-763 client.
- Evidence required before adoption: Require a separately approved extension design, server/client schema, version negotiation, authentication, payload bounds and interoperability tests before implementing any custom mod channel.
- Completion state: complete source review; no tests, build, runtime session or network interoperability test executed.

### 65. `crates/horizon-protocol/src/quiet.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `dcf48dbee8b5b628aa7cc25e9f03a4eeb4c63238`.
- Size: 5,465 bytes; 166 lines; full source fetched.
- Coverage: lines 1–166, complete.
- Purpose: Builds and parses batched command messages that a native-mod client sends to a Horizon server over a dedicated channel for execution under the player's permissions without chat feedback.
- Implementation / logic: The channel is `horizon:quiet_commands`. Each batch contains a `mod_id` and command lines without a leading slash. Encoding allows at most 256 lines per payload and 256 characters per line, splits lines into multiple payloads capped at 32,767 bytes, and preserves order. Decoding reads the mod ID, count, and lines under the corresponding limits and rejects trailing bytes. The module header says the server advertises support with an empty Configuration payload; on other servers, the client falls back to chat commands.
- Findings: This file implements wire framing only; the server handler must validate the `mod_id`, player authentication, permissions, rate limits, and command dispatch. Encoding merely truncates `mod_id`; this file does not enforce an allowlist or namespace syntax. Line count and length are bounded, and payload size is calculated using UTF-8 byte length. An empty batch produces no payload. Tests cover batching, splitting by `MAX_LINES`/byte size, truncation, and malformed counts/strings/trailing bytes; they were not run.
- RustChunk relevance: Batched packets and bounded payloads are useful concepts, but this command-execution channel is Horizon-specific and is not needed by a vanilla client.
- 1.20.1 implications: In vanilla protocol 1.20.1, command execution uses the normal chat/command packets; `horizon:quiet_commands` is not part of the official protocol. Implementing this channel before the vanilla Login/Play connection works would break compatibility or expand the attack surface. If a native extension is needed later, it must have independent capability negotiation, server-side authorization, and rate limiting.
- Decision: **Unnecessary for the base milestone; batching and size checks are reusable for a future extension protocol.**
- Dependencies / cross-references: protocol Reader/Writer; horizon:quiet_commands custom payload registration; docs/mods.md command.run-quiet; server command dispatcher and player permissions.
- Follow-up checks: In the future, inspect the server caller for trust in mod IDs, per-player throttling, command-length semantics, and chat fallback. Do not implement an endpoint for this in the current milestone.
- RustChunk implementation mapping: No first-release mapping. This custom quiet/extension packet path is not part of vanilla 1.20.1; generic batching/size-limit ideas can be documented without porting the protocol.
- Evidence required before adoption: Verify no 1.20.1 packet or state depends on this channel; if reused as a generic helper, test exact limits, malformed data and backpressure independently of protocol code.
- Completion state: complete source review; no tests, build, runtime session or network interoperability test executed.

### 66. `crates/horizon-protocol/src/servers.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `466f868fa0a1a8496d65e4e85fbee4763b5a85f1`.
- Size: 8,679 bytes; 229 lines; full source fetched.
- Coverage: lines 1–229, complete.
- Purpose: Reads, writes, and prepends entries in `servers.dat` using an NBT layout similar to the vanilla server list, and parses host/port addresses.
- Implementation / logic: `ServerEntry` models the name, IP, base64 favicon, tri-state `acceptTextures`, and `acceptedCodeOfConduct` hash fields. `load` reads uncompressed NBT, removes hidden LAN/quick-play entries from the displayed list, and skips entries without an IP. `save` writes an unnamed root with a `servers` tag list. `prepend` inserts a new entry first while preserving other fields and unknown hidden entries. Writes go to `dat_tmp` before a rename. `parse_address` handles `host`, `host:port`, `[v6]:port`, and the default port 25565.
- Findings: The implementation uses NBT `JavaString`/`NamedTag`. If the file is damaged, `load` returns an empty list, while `prepend` returns an error and leaves the file untouched. `save` rewrites the entire list and drops unknown fields; `prepend` is designed to preserve them. The parser distinguishes bracketed IPv6 from unbracketed multi-colon hosts. Tests cover round trips, preservation of hidden/unknown fields, damaged-file safety, and address parsing; they were not run.
- RustChunk relevance: The `servers.dat` structure and preservation of server-list settings are directly relevant to RustChunk's launcher/client.
- 1.20.1 implications: Cross-check this against the exact Minecraft 1.20.1 schema and defaults; `acceptedCodeOfConduct` may be 26.3-only. Verify the NBT root/list keys and `ServerData` address behavior against the target version. Do not overwrite the file with empty data after a parse failure. Test replacement-by-rename behavior on Android, Linux, and Windows.
- Decision: **Adaptable after verifying the 1.20.1 schema and filesystem semantics.**
- Dependencies / cross-references: horizon_world::nbt::{read_named,write_named,Tag,JavaString}; local game server list screen; server status favicon and server resource pack preference.
- Follow-up checks: Inspect the `save`/`prepend` callers and server-list UI; compare the full vanilla 1.20.1 `servers.dat` schema, including fields and malformed-file behavior; test atomic rename on Android.
- RustChunk implementation mapping: Maps to the future server-list model/UI and persistent address metadata, not to a game-protocol primitive. It can be postponed until multiplayer UI is scheduled.
- Evidence required before adoption: Test server entry serialization, migration, invalid addresses and safe storage; verify user-visible status uses 1.20.1 protocol metadata and no credentials are stored in plain text.
- Completion state: complete source review; no tests, build, runtime session or network interoperability test executed.

### 67. `crates/horizon-protocol/src/text.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `cc44430fb66aadfa39ac018108bcf454bf1bbcf4`.
- Size: 7,829 bytes; 219 lines; full source fetched.
- Coverage: lines 1–219, complete.
- Purpose: Converts text `ComponentSerialization` received in NBT form into plain text and resolves translation keys through a language table.
- Implementation / logic: It unwraps plain/append literal strings, primitive numbers, lists, `{text:...}`, empty-key wrappers, translate + with + fallback, keybind, selector, score.name, and extra forms. Recursion stops after depth 32. `format_java` supports `%s`, `%n$s`, and `%%`. Literal and translatable helpers construct NBT tags.
- Findings: This implementation extracts plain text; it is not a full component renderer. Style, color, click, hover, insertion, font, and most other semantics are discarded. Selectors are not actually resolved, and score output includes only the name. Unknown translations display the fallback or the key itself. Unsupported placeholder formats may be left partially or become empty. Tests cover translation arguments, mixed NBT wrappers, ordered/sequential placeholders, percent escapes, and extra content; they were not run.
- RustChunk relevance: Useful as a plain-text fallback for status/chat display in RustChunk, but not as a replacement for a real text-component renderer.
- 1.20.1 implications: Minecraft 1.20.1 chat components use that version's codec/packet representations and specific JSON/NBT forms; this NBT component conversion and translation logic must be matched to the exact target serialization. For signed chat, flattening is sufficient only for display and must not normalize the signed payload. The UI should retain the rich component tree separately.
- Decision: **Adaptable only as a plain-text fallback; the 1.20.1 rich-component parser/renderer must be implemented separately.**
- Dependencies / cross-references: horizon_world::nbt::{JavaString,Tag}; language `en_us.json`; protocol chat/status packet handling and UI chat renderer.
- Follow-up checks: Compare component serialization and `TranslatableContents` in 1.20.1; test selectors, score, style inheritance, nested extras, malformed arguments, and signed-chat rendering.
- RustChunk implementation mapping: Maps to the future 1.20.1 text/component model used by chat, UI, titles and server messages. The existing renderer/font layer must supply shaping; this source is not a complete RTL text engine.
- Evidence required before adoption: Build JSON text-component fixtures from 1.20.1, including styles, nesting, translations and click/hover behavior; verify rendering, sanitization, Unicode and Persian/RTL shaping separately.
- Completion state: complete source review; no tests, build, runtime session or network interoperability test executed.

### 68. `crates/horizon-protocol/src/crypto/blocks.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `308a80a73022b3fa23bdcded18679ab33a912493`.
- Size: 7,623 bytes; 207 lines; full source fetched.
- Coverage: lines 1–207, complete.
- Purpose: Provides primitive AES-128 block encryption and fast CFB8 encryption for Minecraft connection encryption.
- Implementation / logic: `Blocks` stores an AES-128 key schedule and provides `encrypt_block`, `cfb8_encrypt`, and `encrypt_lanes`. On AArch64 builds with the compile-time `target_feature=aes`, it uses ARMv8 AESE/AESMC intrinsics in an eight-block pipeline. Other targets use RustCrypto `aes` with batches of 32 lanes and runtime AES-NI acceleration on x86/x86_64. CFB8 XORs each byte with `AES(register)` and appends the ciphertext to the end of the 16-byte shift register.
- Findings: The ARM64 section is the only unsafe code in this crate. Its safety relies on the assumption that `target_feature=aes` is guaranteed by the CPUs targeted by the binary; build target flags must match the actual minimum device. Enabling the AES feature for every ARM64 release could make the binary unusable on devices without the crypto extension. The custom key schedule uses a table-based S-box lookup indexed from key material; timing/cache side channels require review. The fallback depends on a vetted crate. This file contains no standalone tests; the algorithm and known-answer vectors must be tested at the wrapper/crypto-crate level.
- RustChunk relevance: Important for login-connection encryption. Android ARM64 can benefit from hardware AES, but feature detection and security matter more than benchmarking.
- 1.20.1 implications: Minecraft 1.20.1 also uses AES/CFB8 for online-mode login encryption, so the CFB8 semantics and key/IV handling are potentially reusable, but must not be copied without tests. Runtime ARM64 feature dispatch is safer than assuming compile-time AES support for broad Android distribution. Offline-mode login does not require this server-encryption step.
- Decision: **The CFB8 algorithm is adaptable; the custom ARM AES backend is usable only after security review and safe feature dispatch.**
- Dependencies / cross-references: protocol::crypto.rs, aes crate, ARMv8 AES/NEON intrinsics, RustCrypto AES, target CPU feature configuration.
- Follow-up checks: Inspect Android arm64-v8a build target flags and device crypto-extension support; test NIST AES vectors and CFB8 known-answer vectors; compare with a constant-time RustCrypto backend and assess the S-box key schedule.
- RustChunk implementation mapping: Maps to the future AES/CFB8 protocol encryption primitive or vetted replacement. It is not needed for local single-player and must not be accepted solely on architecture review.
- Evidence required before adoption: Compare encrypt/decrypt vectors with Java 1.20.1, test ARM64 feature dispatch and fallback paths, and review side-channel/constant-time properties before online-mode use.
- Completion state: complete source review; no tests, build, runtime session or network interoperability test executed.

### 69. `crates/horizon-protocol/src/item/hash.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `966f8860587406f7e8468398532da0b33cbc1be3`.
- Size: 29,855 bytes; 850 lines; full source fetched in consecutive ranges of at most 400 lines.
- Coverage: lines 1–850, complete.
- Purpose: Builds the representation and hashes for patched ItemStack components according to the 26.3 `HashedPatchMap.create` and `HashedStack.create` logic.
- Implementation / logic: The `Names` trait converts registry IDs into resource identifiers. Component code reconstructs the Codec shape from the wire representation and internal type, then hashes it with HashOps/CRC32C. Nested components such as enchantment maps, StackTemplate, potions, books, fireworks, banner patterns, trim, custom model data, and tooltip display have separate paths. A component is not hashed if its registry name or Codec form cannot be reconstructed. `Hashed::create` hashes added components and retains removed IDs; when a raw wire patch is available, it decodes that patch first to preserve the actual transmitted patch.
- Findings: This implementation directly depends on `DataComponentType`, 26.3 registry IDs, and modern Codecs. The result is neither SHA nor a file hash; it is CRC32C over the Codec representation. Unknown or unnamed components are silently omitted from the added-component hashes and, according to the module documentation, may cause the server to resynchronize the slot. The test suite fixes the hash values generated by vanilla 26.3 for dozens of components; those tests were not run in this audit.
- RustChunk relevance: Useful for understanding item-patch transfer semantics in the newer Horizon generation, but RustChunk 1.20.1 does not need this Data Component system.
- 1.20.1 implications: Minecraft 1.20.1 predates the Item Data Components introduced in 1.20.5; its ItemStack uses item ID/count and the traditional NBT tag. This file and the hashed-patch protocol must not be ported. Implement the legacy 1.20.1 ItemStack serialization and NBT independently. If a digest is needed, define separate, explicit semantics and an algorithm.
- Decision: **Version-specific and unusable for 1.20.1; unnecessary for the target protocol. The ItemStack+NBT path must be rewritten.**
- Dependencies / cross-references: hash_ops.rs, item.rs::{Component,Stack,WirePatch,Hashed}, registry name lookup, packets/play.rs ItemStack encoders and vanilla DataComponent Codec definitions.
- Follow-up checks: Compare the complete 1.20.1 ItemStack Codec and NBT layout against a vanilla oracle; keep all `HashedPatchMap` references outside the base-milestone scope.
- RustChunk implementation mapping: No 1.20.1 ItemStack mapping. This file hashes modern Data Component item content; RustChunk must use the legacy 1.20.1 item ID/count/NBT path.
- Evidence required before adoption: Prove the item codec and inventory model for 1.20.1 do not reference this hash scheme; validate legacy slot/NBT golden vectors instead.
- Completion state: complete source review; no tests or runtime interoperability were executed.

### 70. `crates/horizon-protocol/src/item/hash/exact.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `4441e7f6b52c4fa86add35c4cc8c56c40bb26539`.
- Size: 6,955 bytes; 170 lines; full source fetched in consecutive ranges of at most 400 lines.
- Coverage: lines 1–170, complete.
- Purpose: Hashes exact component predicates and nested predicates inside NBT data, especially container and bundle contents.
- Implementation / logic: It defines a Shape for each component to identify booleans, boolean lists, and fields containing ItemStacks, then traverses the NBT tree and converts it to HashOps. `components()` reads namespaced component keys, converts `!` keys to an empty-map hash, and converts glint-override values to Boolean. `predicate()` hashes predicate-level booleans and nested stack components with the corresponding shape.
- Findings: It depends on `DataComponentExactPredicate.CODEC` and the 26.3 `DataComponentPredicate` types. NBT alone cannot distinguish a boolean from a byte, so the Shape metadata is maintained manually and must be extended when a new Codec is added. Component names must use the `minecraft` namespace; components without a persistent Codec cannot be hashed. This file has no packet framing; it is part of the hashed-item-component logic.
- RustChunk relevance: Useful only for understanding newer-generation item predicates and the risk of confusing boolean and byte values in NBT.
- 1.20.1 implications: Vanilla 1.20.1 predicates and ItemStack NBT do not use the 26.3 Data Components model. Do not reuse this Shape table or HashOps mapping. If 1.20.1 item or loot predicates are needed, derive them from the actual 1.20.1 Codec/serializer and data structures.
- Decision: **Version-specific and unnecessary for the 1.20.1 core; predicate parsing for the target version must be rewritten separately.**
- Dependencies / cross-references: item/hash.rs, item/hash/gameplay.rs PREDICATE_BOOLS, HashOps, NBT Compound/Tag, DataComponentPredicate/ExactPredicate codecs.
- Follow-up checks: When designing 1.20.1 loot/item predicates, separately verify NBT matching semantics and the distinction between exact and partial predicates.
- RustChunk implementation mapping: No base mapping. The nested/exact Data Component predicate hash model is not the 1.20.1 vanilla item-predicate representation.
- Evidence required before adoption: If 1.20.1 loot/item predicates are later needed, derive their schema from 1.20.1 source and test exact-vs-partial NBT matching; do not accept modern hash tables.
- Completion state: complete source review; no tests or runtime interoperability were executed.

### 71. `crates/horizon-protocol/src/item/hash/gameplay.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `a2a248ff0c2a6ad12211c86a2667fcecebb9089c`.
- Size: 24,029 bytes; 620 lines; full source fetched in consecutive ranges of at most 400 lines.
- Coverage: lines 1–620, complete.
- Purpose: Reconstructs Codec-form representations and hashes gameplay item components from byte streams and registry names.
- Implementation / logic: It has specialized paths for components such as UseEffects, AttributeModifiers, Food, Consumable, UseCooldown, DamageResistant/Repairable, Tool, Weapon, Equippable, DeathProtection, BlocksAttacks, Piercing/KineticWeapon, SwingAnimation, Jukebox, and banner/trim-related data. Registry references are resolved through `Names`; enum IDs are converted to `StringRepresentable` names; default fields are omitted from the hash according to `optionalFieldOf`; and sound, holder forms, and BlockPredicate values are also reconstructed as Codec maps.
- Findings: Dispatch is based on fixed DataComponents IDs from generation 26.3. Many decoder helpers use `Option`; malformed streams, unsupported enum values, or unnamed registry IDs become `None`. Exact Codec defaults affect the hash, so omitting the wrong value causes a mismatch. This code does not mix modern components with legacy ItemStack data; it is not a 1.20.1 path.
- RustChunk relevance: This section shows which behavioral metadata Minecraft 26.3 stores in item components and is useful for documenting architectural differences.
- 1.20.1 implications: In 1.20.1, item behavior and metadata primarily come from ItemStack NBT, item classes/registries, and server code, not these Data Component schemas. Do not port this Codec map into the RustChunk base. If equivalent gameplay semantics are required, implement the 1.20.1 NBT handling and item behavior independently.
- Decision: **Version-specific and unusable for 1.20.1; the legacy implementation must be written separately.**
- Dependencies / cross-references: item/hash.rs, item/hash/raw.rs, item/hash/exact.rs, HashOps, Reader, Registry Names, modern component stream codecs.
- Follow-up checks: Extract ItemStack, FoodProperties, AttributeModifiers, and item-specific NBT independently from 1.20.1 source, then prepare separate golden vectors.
- RustChunk implementation mapping: No base mapping. These gameplay component hashes describe the modern item component model, not 1.20.1 ItemStack metadata.
- Evidence required before adoption: Require a 1.20.1-derived item behavior/NBT schema and golden fixtures for food, attributes and item-specific tags before building any equivalent feature.
- Completion state: complete source review; no tests or runtime interoperability were executed.

### 72. `crates/horizon-protocol/src/item/hash/raw.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `252f61fadd02d10945cd40f31470c3c89d4b2287`.
- Size: 53,132 bytes; 1,155 lines; full source fetched in consecutive ranges of at most 400 lines.
- Coverage: lines 1–1,155, complete.
- Purpose: Reconstructs Codec hashes from byte streams for Data Components that Horizon stores as `Component::Raw`.
- Implementation / logic: Readers consume the component stream and reconstruct the persistent Codec form. Registry references are converted to resource IDs; typed entity NBT is rebuilt with the correct ID; profiles, potion tails, trim/instrument/banner holders, container contents, pot decorations, bees, and sign text have separate paths. The `code` function dispatches by 26.3 kind IDs and rejects any trailing bytes. The file includes a large golden-vector test set comparing hexadecimal streams and hashes printed by vanilla 26.3.
- Findings: Both the code and its registry/test-fixture tables depend on the 26.3 ID order and Codecs. Some paths fall back to other component modules. Unknown, malformed, or registry-unresolved components yield `None`; raw bytes alone are insufficient to construct a hash. The tests cover many components but were not run in this audit. The file has 1,156 lines and was read completely in three consecutive ranges: 1–400, 401–800, and 801–1,156.
- RustChunk relevance: Useful for understanding the reverse-engineering method from `STREAM_CODEC` to `Codec` and for maintaining golden vectors, but the modern 26.3 schema itself must not be transferred to RustChunk 1.20.1.
- 1.20.1 implications: 1.20.1 does not have these Data Component registry/stream IDs. Copying the dispatch table or hash rules would misparse packets and item metadata. The actual 1.20.1 ItemStack/NBT serializer must be implemented for the base client. The method of extracting exact byte vectors from a vanilla oracle and rejecting trailing bytes can be retained.
- Decision: **Version-specific and unusable for 1.20.1; the golden-vector/oracle-testing method is reusable.**
- Dependencies / cross-references: item/hash.rs, item/hash/gameplay.rs, item/hash/exact.rs, item/skip.rs, HashOps, Item::Component::Raw, registry name tables, vanilla 26.3 StreamCodec and persistent Codec.
- Follow-up checks: Record the 1.20.1 vanilla StreamCodec/packet schema for ItemStack and NBT; create golden vectors for every item form and specify unknown/trailing-field behavior precisely.
- RustChunk implementation mapping: No direct runtime mapping. The reverse-engineering and golden-vector methodology may be reused for a future version-specific serializer, but none of the modern component IDs/data are portable.
- Evidence required before adoption: Build 1.20.1 byte vectors from official Java serializers and demonstrate exact round trips; ensure generated dispatch tables contain no modern-only component registry entries.
- Completion state: complete source review; no tests or runtime interoperability were executed.

### 73. `crates/horizon-protocol/src/item/skip.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `aa436cc31e7add4f2bfd21a45c9128fd417b44f2`.
- Size: 14,017 bytes; 425 lines; full source fetched in consecutive ranges of at most 400 lines.
- Coverage: lines 1–425, complete.
- Purpose: Precisely consumes byte streams for Data Components whose internal meaning Horizon does not decode, so that parsing the remainder of the ItemStack stays aligned.
- Implementation / logic: It uses a dispatch table based on the 26.3 DataComponents registry order. Depending on the component type, it consumes the appropriate VarInt, float, identifier, NBT, list, optional, holder/holderSet, sound, template, block-predicate, attribute-modifier, food/consumable, tool, equippable, blocks-attacks, kinetic-weapon, potion-effect, or firework representation. For components without a stream Codec, it reads the persistent NBT Codec. An unknown kind produces an error.
- Findings: This file is critical because the current item-data format has no separate length prefix for each component. If a skip routine consumes the wrong number of bytes, all following components and subsequent packets become misaligned. At the same time, an unknown kind cannot safely be skipped generically and immediately produces an error. The kind-ID table, nested-component forms, holder encoding, and registry order are all specific to 26.3. The file was read completely in two consecutive ranges, 1–400 and 401–426; no standalone tests were found in this file.
- RustChunk relevance: It provides an important defensive rule: skip unknown data only when its exact Codec length/schema is known; otherwise reject the connection instead of guessing.
- 1.20.1 implications: Minecraft 1.20.1 predates Data Components and does not have this component registry/network format. Do not use this skip table. Implement the legacy ItemStack serializer for the target version (item registry ID, count, optional NBT) and its NBT bounds. The only reusable rule is: never guess an unknown length.
- Decision: **Version-specific and unnecessary for 1.20.1; the legacy ItemStack decoder must be implemented separately.**
- Dependencies / cross-references: item.rs Component/Template/Reader, buf::Reader, horizon_world NBT, packet slot/item readers and 26.3 DataComponents registry order.
- Follow-up checks: During the 1.20.1 packet/item audit, extract the empty/non-empty Slot paths and NBT limits/terminator from vanilla source, then write malformed-NBT tests.
- RustChunk implementation mapping: No base mapping. The component skip table is specific to a wire schema absent from 1.20.1; unknown item data must be handled according to the 1.20.1 Slot/NBT grammar.
- Evidence required before adoption: Test empty/non-empty slot decoding, NBT boundaries, unknown item IDs and truncated payloads against 1.20.1 oracle vectors; reject malformed streams rather than guessing lengths.
- Completion state: complete source review; no tests or runtime interoperability were executed.

### 74. `crates/horizon-client/src/chunk_io.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `9613b3b265c39f7f97e6ec4aa7e8ec1a249a2f93`.
- Size: 5,730 bytes; 165 lines; full source fetched.
- Coverage: lines 1–165, complete.
- Purpose: Collects chunk load/save failures from multiple threads and turns them into UI toasts on the window thread.
- Implementation / logic: It logs load/save failures, uses `save::low_disk_space` to check storage space, and stores a low-space flag plus the latest failed chunk coordinates of each type in a global `Pending` mutex. `take()` atomically drains the queue with `std::mem::take`. Toast helpers build translated text and chunk coordinates. Server-originated low-disk-space messages are added to the same queue.
- Findings: Because only the latest chunk of each type is retained, repeated failures are summarized rather than counted individually. The `low_disk_space` flag remains sticky until `take()`. A poisoned mutex is recovered with `into_inner`. This code handles reporting and UI only; it does not persist chunks itself. Tests cover the newest failure, sticky warning, and toast timing; they were not run.
- RustChunk relevance: Useful for world-save reliability on Android: I/O failures and low-storage conditions should be reported to the UI from the storage thread without making the game thread construct toasts.
- 1.20.1 implications: Asynchronous failure reporting is not version-specific and can be adapted, but translations/messages and the integration point with `ServerChunkCache` must be implemented for the new client/server engine. RustChunk should report low storage and failures without blocking the tick/render thread.
- Decision: **Adaptable; the current code is tied to Horizon's save/toast systems and must be rewritten.**
- Dependencies / cross-references: horizon_client::save::low_disk_space, toasts::SystemToast, lang translation keys, HUD rgba, world region/chunk I/O and server writer thread.
- Follow-up checks: Audit `save.rs` and the region writer; establish thread ownership, failure propagation, retry policy, and Android storage-permission/space handling.
- RustChunk implementation mapping: Maps to future asynchronous chunk/world save-load error reporting and the Android UI notification channel. There is no current native world-storage worker in RustChunk.
- Evidence required before adoption: Inject disk-full, permission, corrupt-region and interrupted-write failures; verify errors reach the UI safely, are not lost across thread boundaries and never corrupt a previously saved world.
- Completion state: complete source review; no tests/build/runtime/device interoperability test was run.

### 75. `crates/horizon-client/src/content_identity.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `52f14173b4b52bb0d679b050859071bdcccd6579`.
- Size: 10,466 bytes; 253 lines; full source fetched.
- Coverage: lines 1–253, complete.
- Purpose: Computes a fingerprint for block-state registry order and registries added by content mods, so an incompatible client can be rejected before receiving network IDs.
- Implementation / logic: It reads state rows from `facts.json` and serializes only each state's ID and properties into a canonical-like JSON form, feeding its length and bytes into SHA-256. If `registries.json` exists, it parses the Builtin report and adds only non-`minecraft` mod entries from item, mob_effect, entity_type, and menu registries to the hash chain in network-ID order. If custom NBT components are registered, their names/type IDs and default values are included in the fingerprint. The result is cached by resource root using `OnceLock<Mutex<HashMap<PathBuf, hash>>>`.
- Findings: This fingerprint is designed for Horizon's modded-registry model and is associated with the `horizon:block_registry` channel/`horizon-content-blocks` marker; channel negotiation and disconnect handling are in other files. The hash is sensitive to state order, property values, registry IDs/names, component IDs, and defaults. The cache is keyed by PathBuf and does not reflect file changes under the same root during the process lifetime, so it can become stale if the root is mutable. Failure to read or parse `facts.json` is an error, but a missing or unreadable `registries.json` is treated through `if let Ok` as a block-only root. This fingerprint was not designed for 1.20.1.
- RustChunk relevance: Registry fingerprinting is a useful pattern for mod parity, especially when item/entity/block IDs in the protocol depend on registration order.
- 1.20.1 implications: Vanilla 1.20.1 has no such dedicated hash handshake; registry synchronization and IDs follow that version's protocol/registries. Avoid the custom Horizon channel in the base client. If content packs are added later, build the fingerprint from actual 1.20.1 registry reports and only from registries/metadata agreed by both sides; define canonical serialization, cache invalidation, and failure policy explicitly.
- Decision: **Adaptable only as an idea for future mod-content parity; the current implementation is version-specific and outside the base milestone.**
- Dependencies / cross-references: horizon_protocol::registry::Builtin; facts.json / registries.json generated by horizon-prepare; item::custom registry; horizon_world::item_defaults; channel horizon:block_registry.
- Follow-up checks: Later, trace the handshake caller on both client and server, the `facts.json`/`registries.json` generators, and whether the resource root can change after startup. Do not add any SHA handshake to vanilla login now.
- RustChunk implementation mapping: No base vanilla mapping. The current content-mod registry fingerprint/custom handshake is unnecessary for a vanilla 1.20.1 client; a future mod-content identity layer must be separately specified.
- Evidence required before adoption: Prove vanilla startup/world join does not depend on a custom fingerprint. For any later mod support, test deterministic registry hashes, ordering, collision handling and client/server mismatch rejection.
- Completion state: complete source review; no tests/build/runtime/device interoperability test was run.

### 76. `crates/horizon-client/src/player_sync.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `0d116535bbf9ee082ee028ba2490981a57e5f262`.
- Size: 5,037 bytes; 135 lines; full source fetched.
- Coverage: lines 1–135, complete.
- Purpose: Computes player movement/rotation packets sent to the server from the last-update baseline and transforms velocity during teleports using relative flags.
- Implementation / logic: `PositionSender` stores the last position, yaw/pitch, on-ground state, collision state, and tick reminder. On each update it sends a position packet if the squared distance exceeds `(2e-4)^2` or the reminder reaches 20 ticks; it selects a moved+rotated, position-only, rotation-only, or status-only packet. `told_rotation` tracks a look direction already sent in another packet, such as a teleport acknowledgement, so a later update resends it if it differs. `teleport_velocity` first rotates velocity by the relative rotation delta, then adds or replaces X/Y/Z components according to bit flags.
- Findings: This is a stateful packet scheduler. If its baseline is not synchronized after a teleport or a packet sent through another path, a movement packet may be skipped or duplicated. The position threshold and 20-tick reminder are intended to mirror vanilla behavior and must be checked against the exact 1.20.1 code. `told_rotation` supports Starling's custom steering/free-direction feature and may not be needed by vanilla. The included test is a regression test for a changed look direction after teleport; it was not run.
- RustChunk relevance: Highly relevant to a playable client and server joining; correct player movement and teleport handling are essential for multiplayer.
- 1.20.1 implications: 1.20.1 has the move-position/rotation/status packet family and relative teleport flags, but names, flags, thresholds, and ordering must come from that version's protocol and client source. Reset/update the baseline precisely on login, correction/teleport acknowledgement, respawn, and dimension changes. Do not include the Free Direction extension in the base milestone.
- Decision: **The baseline and delta-handling algorithm is adaptable; packet IDs, thresholds, and the steering extension must be validated or rewritten for 1.20.1.**
- Dependencies / cross-references: horizon_protocol::packets::play::{MovePlayer*,PlayerPosition,relative}, horizon_core::geometry::Vec3, local player tick/input, teleport acknowledgement path.
- Follow-up checks: Inspect the 1.20.1 `MovePlayer` and `ClientboundPlayerPosition` dispatch paths; produce a vanilla comparison trace for the stationary 20-tick heartbeat, on-ground/collision flags, and relative velocity rotation.
- RustChunk implementation mapping: Maps to future local-player simulation and, later, protocol-763 movement/teleport synchronization. The movement algorithm may inform simulation, but packet thresholds/flags are server-version-specific.
- Evidence required before adoption: Compare input-to-motion and movement/teleport packet traces against vanilla 1.20.1; test stationary heartbeat, collisions, relative teleport flags, mounted movement and on-ground state.
- Completion state: complete source review; no tests/build/runtime/device interoperability test was run.

### 77. `crates/horizon-client/src/client_lib.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `a9b81a864c5d4e80d198d77479b19ceeffcf0f32`.
- Size: 5,581 bytes; 187 lines; full source fetched.
- Coverage: lines 1–187, complete.
- Purpose: The client crate root that organizes game modules, Android input/control, renderer, shared core, items, commands, and server-side integrations into a public module surface.
- Implementation / logic: It applies crate-level `forbid(unsafe_code)`, aliases the crate as `horizon_client`, exposes many internal modules, and re-exports subsystems from other crates with `pub use`. Much of the renderer/UI/resources comes from `horizon_client_render`, geometry/game core from `horizon_client_core`, input from `horizon_client_input`, and command/item services from separate crates. The file does not execute game logic; it provides API/import organization.
- Findings: This file is a module index/aggregation root. The number and names of its modules show subsystem separation in Starling/Horizon, but do not by themselves prove the runtime architecture or boot flow. Many re-exports come from independent crates; the actual dependency graph is defined by the Cargo manifests/workspace. `forbid(unsafe_code)` applies only to this crate, not necessarily to its dependencies.
- RustChunk relevance: Useful for deriving subsystem boundaries and avoiding a monolith in RustChunk; neither the implementation nor its public API should be copied verbatim.
- 1.20.1 implications: RustChunk needs crate boundaries suited to the 1.20.1 core and Android platform. Keep the core protocol/registries/world model, renderer/input/platform, and game loop independent at first; add public re-exports after the APIs stabilize. This file is not tied to a particular 1.20.1 protocol detail, but its names and module contents are Horizon-specific.
- Decision: **A reference/adaptable example for crate organization; this file contains no runtime code to port directly.**
- Dependencies / cross-references: Cargo workspace manifests, horizon-client-render, horizon-client-core, horizon-client-input, horizon-command, horizon-items, horizon-server-files and modules declared in this root.
- Follow-up checks: Inspect the workspace Cargo.toml files and the entry point/game loop that consumes this crate; define the minimal RustChunk crate graph before moving subsystems.
- RustChunk implementation mapping: Maps as a crate-boundary reference for separating reusable client systems from application/platform startup. Do not assume a corresponding `client_lib` crate exists in RustChunk yet.
- Evidence required before adoption: Before creating a library boundary, show an acyclic `cargo metadata` dependency graph and compile/test the public API on host and Android; document which modules are actually shared.
- Completion state: complete source review; no tests/build/runtime/device interoperability test was run.

### 78. `crates/horizon-client/src/heartbeat.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `9f3f3a5a69aa8db7ca24ac9986b3ba8148c33dba`.
- Size: 2,697 bytes; 74 lines; full source fetched.
- Coverage: lines 1–74, complete.
- Purpose: Sends periodic anonymous pings to a telemetry backend for Horizon player counts and whether a LAN/friends world is open.
- Implementation / logic: After the client starts (but not for a dedicated server, benchmark, or automated run), a separate Heartbeat thread is created if a telemetry endpoint is configured and heartbeat has not been disabled. A random session ID is generated in memory only and sent in a Ping with the version and LAN flag. The default interval comes from `live::INTERVAL`; failures trigger exponential backoff up to 600 seconds and a one-time log. Changes to the LAN flag are stored in an AtomicBool and wake the thread with `unpark` for an earlier send.
- Findings: This feature is outside Minecraft gameplay/network protocol and is Horizon product telemetry only. It is enabled based on the endpoint and two environment-based opt-out mechanisms. The session identifier is kept in memory, but this file does not show a thread-loop termination/stop hook. Sending occurs on a separate thread so it does not block tick/render.
- RustChunk relevance: Not core to running the RustChunk client. It illustrates that optional telemetry should be separate from game networking and have a clear opt-out.
- 1.20.1 implications: A product heartbeat is not required for vanilla 1.20.1; this endpoint, format, and telemetry should be omitted. Any future RustChunk telemetry must be opt-in and disableable, send minimal data, and have a defined thread lifecycle/shutdown. It must not affect the Minecraft protocol.
- Decision: **Unnecessary for the base milestone and for Minecraft 1.20.1 compatibility.**
- Dependencies / cross-references: horizon_telemetry::live::{session_id,INTERVAL,Ping,send}, telemetry::version, telemetry endpoint config, LAN/friends world lifecycle.
- Follow-up checks: No port is needed. If telemetry is added in the future, review the privacy notice, consent, disable switch, shutdown lifecycle, and exact data sent.
- RustChunk implementation mapping: No runtime mapping. Anonymous player-count/LAN telemetry is not required for the first native 1.20.1 client and should be omitted unless RustChunk defines an explicit opt-in telemetry policy.
- Evidence required before adoption: For the initial release, verify no periodic telemetry request is made. Any later telemetry needs consent/disable tests, payload minimization, endpoint security and token/path redaction.
- Completion state: complete source review; no tests, builds, network downloads, resource reloads or Android device tests were executed.

### 79. `crates/horizon-client/src/reconnect.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `cd942aa7e095bc545b9100da5f933f54e45c2198`.
- Size: 17,547 bytes; 473 lines; full source fetched.
- Coverage: lines 1–400 and 401–473, complete.
- Purpose: Decides whether to retry a disconnected multiplayer connection and schedules backoff, while avoiding retries for permanent kicks/bans/refusals.
- Implementation / logic: `Tracker` stores the address, player name, LAN flag, attempt count, and time of a successful join. It allows five retries with delays of 5/10/20/40/60 seconds; a stable connection lasting at least 60 seconds resets the counter. `allowed` first classifies the disconnect translation key using a prefix table and falls back to an English keyword scan when no key is available. Kicks, bans, whitelist failures, incompatible/outdated versions, invalid sessions, online-mode refusals, and disabled multiplayer are not retried; timeouts, shutdowns, full servers, slow logins, and authentication-server downtime are retried. A retry preserves the same address, name, and LAN type.
- Findings: Separating the status key from localized text is more reliable because server messages may be resolved in different languages; the text fallback is limited to literal/OS errors and may be insufficient for non-English messages. Classification keys include Horizon-specific keys and authentication errors, so they must be synchronized with the exact target-version keys. Changing the address resets attempts; a kick or exhausted retry budget resets the tracker and forgets the address. Tests cover classification/localized keys, backoff, stable sessions, and address reset; they were not run.
- RustChunk relevance: Improves connection UX and resilience to network interruptions on Android; the implementation is relatively independent of the protocol version.
- 1.20.1 implications: Disconnect component keys and authentication flow for 1.20.1 must come from vanilla 1.20.1; do not copy 26.3 translation keys or Horizon offline-mode refusal assumptions verbatim. A new connection must not automatically loop after a kick, ban, or invalid session. Retry policy should use a reason code/component key where possible, not just text search.
- Decision: **The backoff logic is adaptable; the classification table and 1.20.1 login/disconnect integration must be rewritten or verified.**
- Dependencies / cross-references: net.rs disconnect/auth result, multiplayer connection lifecycle, localized component/translation keys, server list selected address, Retry UI scheduler.
- Follow-up checks: After auditing network/login, map all 1.20.1 disconnect reason keys, online-mode/session failures, and server-shutdown cases; verify that intentional disconnect/cancel clears the retry state.
- RustChunk implementation mapping: Maps to future multiplayer connection recovery and UI status handling. It is not needed for local single-player world creation.
- Evidence required before adoption: Test transient timeout, refused connection, server kick/ban, authentication failure, intentional cancel and app pause/resume; verify bounded backoff and no retry after permanent failure.
- Completion state: complete source review; no tests, builds, network downloads, resource reloads or Android device tests were executed.

### 80. `crates/horizon-client/src/server_packs.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `61695a6de4b0db492d100ad8d44f52ce9acb0421`.
- Size: 21,263 bytes; 644 lines; full source fetched.
- Coverage: lines 1–400 and 401–644, complete.
- Purpose: Manages server resource-pack prompts, download status, activation above user resource packs, resource reloads, removal, and status responses sent to the server.
- Implementation / logic: It maintains separate state models for Download (Requested/Pending/Done) and Activation (Inactive/Pending/Active), plus a removal reason. The three-state prompt (Pending/Allowed/Declined) comes from the `servers.dat acceptTextures` setting. `push` and `push_local` register a pack by UUID and replace any previous pack with the same ID. `tick` issues download requests and schedules ordered reloads only after downloads complete. The `downloaded` and `reloaded` callbacks advance state; a reload failure triggers a recovery reload to restore previously active packs. Status action codes are 0–7. The URL check in this file tests only the `http`/`https` scheme and enables SHA-1 checking only when the value contains 40 hexadecimal digits.
- Findings: State management/recovery is separated from download and resource reload and is callback-driven. If one pack fails, the remaining packs in that batch are discarded. If the first resource reload fails, previously active packs are reloaded; if recovery also fails, server packs are removed. `valid_url` is not a full URL parser and checks only the scheme prefix; SSRF, redirect, and path checks must occur in the download backend. An absent or malformed SHA-1 means there is no integrity check. `SUCCESSFULLY_LOADED`, `DECLINED`, `FAILED_DOWNLOAD`, `ACCEPTED`, `DOWNLOADED`, `INVALID_URL`, `FAILED_RELOAD`, and `DISCARDED` reflect the 26.3 model. Tests cover state transitions, recovery, pack replacement/status order, and URL/SHA-1 parsing; they were not run. The 645-line file was read completely in two ranges, 1–400 and 401–645.
- RustChunk relevance: Highly relevant to a real client: server resource packs are also an important part of 1.20.1 multiplayer, including textures/models/sounds and server compatibility. The state machine and recovery design are valuable.
- 1.20.1 implications: Compare resource-pack negotiation and packet-status semantics exactly with protocol 1.20.1, especially status-code enums and additions in newer versions. Audit the download backend, redirect policy, ZIP validation, cache path, and hash semantics independently. Secure downloads should enforce redirect/domain/IP checks and archive-traversal/ZIP-bomb checks; SHA-1 is only a compatibility checksum, not a publisher signature. Use the 1.20.1 resource-pack format and `server.properties` prompt behavior as the reference.
- Decision: **The state machine and recovery design are adaptable; status codes, the download/security pipeline, and pack-priority semantics must be verified or rewritten for 1.20.1.**
- Dependencies / cross-references: server_packs::download submodule, ServerboundResourcePackPacket actions, servers.dat acceptTextures, pack reload scheduler, local resourcepack source and resource reload callbacks.
- Follow-up checks: Fully audit `server_packs/download.rs` and the ZIP/resource manager; compare the packet-status enum with 1.20.1; inspect URL redirects, TLS, file size, archive traversal, SHA-1 policy, and preservation of user-pack priority.
- RustChunk implementation mapping: Maps to future 1.20.1 server resource-pack negotiation/download/reload, using the resource manager and protocol-763 packet layer. This is later multiplayer functionality.
- Evidence required before adoption: Test 1.20.1 pack-status packets, redirect/TLS handling, size and archive-traversal limits, hash validation, activation priority, reload failure and removal against a vanilla server.
- Completion state: complete source review; no tests, builds, network downloads, resource reloads or Android device tests were executed.

### 81. `crates/horizon-client-input/src/lib.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `f65082b645f15fbb73de046d0e1a459da7874087`.
- Size: 497 bytes; 16 lines; complete file fetched.
- Coverage: lines 1–16, complete.
- Purpose: The client-input crate root; it specifies which subsystems the input layer exposes.
- Implementation / logic: The crate applies `forbid(unsafe_code)` at its root and publicly exposes the clipboard, gamepad, input state, input mode, interaction input, key names, keybinds, keymapping, modifiers, mouse, and window modules. The file notes that `horizon-client` re-exports these modules under their legacy paths.
- Findings: The file defines only the module surface and unsafe-code policy; it does not implement an event loop or input mapping. Actual dependencies, backends, and settings live in child modules. The unsafe-code prohibition in this crate does not extend to external dependencies or other crates.
- RustChunk relevance: A separate input crate that is isolated from gameplay and UI is a useful pattern, particularly for maintaining Android and desktop backends.
- 1.20.1 implications: The overall architecture is independent of the Minecraft protocol; do not assume Horizon's module names or APIs can be reused verbatim. RustChunk should define a small platform-oriented input abstraction and connect it to game actions.
- Decision: **The crate structure is adaptable; this file itself is only an organizational reference.**
- Dependencies / cross-references: `crates/horizon-client/Cargo.toml`, `client_lib.rs` re-exports, and child modules listed above.
- Follow-up checks: Review each module's public API, platform feature gates, and actual Android/desktop backends in the following files in this audit.
- RustChunk implementation mapping: Maps to a future platform-neutral input crate boundary. Existing Android/Kotlin controls are application UI only; a Rust gameplay input crate is not yet present.
- Evidence required before adoption: Require architecture/dependency checks proving the input layer does not depend on renderer/game internals, plus host and Android tests for event translation and lifecycle.
- Completion state: complete source review; no tests or build were run.

### 82. `crates/horizon-client-input/src/input.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `f8b2c10b7f3576c5853274ffb09b25bad7a83621`.
- Size: 3,690 bytes; 112 lines; complete file fetched.
- Coverage: lines 1–112, complete.
- Purpose: Computes movement direction from the four directional keys, applies movement multipliers, and detects the operating-system quit shortcut.
- Implementation / logic: `keyboard_direction` creates -1/0/1 impulses for forward/back/left/right and normalizes diagonal vectors. `modify_input` applies a base multiplier of 0.98 to nonzero input, followed by item-use and sneak multipliers. `square_movement` normalizes the vector using a Java-style speed-clamping formula and deliberately propagates NaN in the same way as `Math.min`. `is_quit_shortcut_down` uses Super+Q on macOS and Alt+F4 on other systems.
- Findings: The comment explicitly identifies the model as 26.3 and excludes sprint eligibility, pose, and input lifecycle from scope. Zero movement returns before multipliers are applied. Rust `f32::min` and Java `Math.min` differ for NaN, and the implementation compensates for this. Tests cover only platform quit shortcuts; no movement-math tests are present in this file, and no tests were run.
- RustChunk relevance: Key direction, diagonal normalization, and movement-scalar semantics are part of player input and the gameplay core; this logic should remain independent of winit and the event source.
- 1.20.1 implications: Compare the movement algorithm with a 1.20.1 `LocalPlayer`/`Input` oracle. Movement multipliers and item-use, sneak, riding, and sprint behavior may differ in the target version. Desktop quit shortcuts are not needed on Android and should come from the platform abstraction.
- Decision: **The vector-normalization approach and multiplier concept are adaptable; the multiplier values and lifecycle logic must be reviewed against 1.20.1.**
- Dependencies / cross-references: winit KeyCode/ModifiersState; LocalPlayer.modifyInput semantics; key event state provider; sprint/pose/riding lifecycle not implemented here.
- Follow-up checks: Compare movement-input functions with vanilla 1.20.1; locate the item-use/sneaking calculations; inspect NaN/infinite input and add diagonal/analog-controller tests.
- RustChunk implementation mapping: Maps to future normalized movement/action input feeding vanilla 1.20.1 local-player simulation; this is not an existing game-logic function in RustChunk.
- Evidence required before adoption: Compare movement input, diagonal normalization, sneak/sprint/use modifiers and analog values against 1.20.1 vanilla fixtures; test NaN/infinity and focus-loss cleanup.
- Completion state: complete source review; no tests or build were run.

### 83. `crates/horizon-client-input/src/input_mode.rs`
- Source repository/ref: `starling-client/starling` / `starling`.
- Source SHA: `24dc84c50a2abe43ef0391c3f649c191ef2a7233`.
- Size: 5,349 bytes; 148 lines; complete file fetched.
- Coverage: lines 1–148, complete.
- Purpose: Selects the Touch/Pad/KeyboardMouse input mode and distinguishes a physical Android keyboard from a virtual keyboard and system keys.
- Implementation / logic: `InputMode::first` starts in Touch on platforms where touch is the primary input. On a phone, `after` switches modes according to the most recent input; on a touch laptop, it remains in Touch after the first touch. `android_keyboard` uses an Android keycode allowlist, `device != virtual keyboard`, `scan_code != 0`, and a non-gamepad source to detect physical typing. `held_modifiers` reconstructs SHIFT/CONTROL/ALT/SUPER by inspecting the set of held keys; the file notes that Android modifier changes are not received through winit.
- Findings: Physical-keyboard detection relies on a numeric Android keycode allowlist and source/device metadata, so it must be validated as APIs and device drivers change. Mode behavior intentionally differs between touch laptops and phones. `held_modifiers` derives state from held keys; correctness depends on key-up/cancel/focus-loss handling. Tests cover mode transitions, physical-vs-virtual keyboard detection, and modifier aggregation; they were not run.
- RustChunk relevance: Directly relevant to Galaxy/Android touchscreens, physical keyboards, and gamepads. Mode detection can select the touch overlay and prevent OS navigation keys from being mistaken for typing.
- 1.20.1 implications: Independent of the protocol, but it must be reviewed in the RustChunk Android backend against the Android NDK/Java key-event codes and the selected winit integration. Do not copy the current allowlist without checking the platform SDK; handle focus loss and key cancellation in the global lifecycle.
- Decision: **The mode-switching design and modifier reconstruction are adaptable; the Android keycode/source allowlist must be validated on target devices.**
- Dependencies / cross-references: winit KeyCode/ModifiersState; `crate::gamepad::android::from_pad`; Android key event source/device/scan-code mapping; touch overlay and keyboard event dispatch.
- Follow-up checks: Inspect the Android input-event bridge, key-up/focus-loss cleanup, device-specific scan codes, and real HID/gamepad devices.
- RustChunk implementation mapping: Maps to a future Android/desktop input-mode adapter that distinguishes touch, keyboard/mouse and controller sources. No equivalent native Rust mode manager is present yet.
- Evidence required before adoption: Validate Android key/source codes and physical-device behavior on the Galaxy A22 plus at least one HID device; test focus loss, virtual keyboard and input-mode switching.
- Completion state: complete source review; no tests or device input tests were run.

### 84. `crates/horizon-client-input/src/interaction_input.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 9457726434ea4fc943047a9d2351428846595c01.
- Size: 8,959 bytes; 276 lines; complete file fetched.
- Coverage: lines 1–276, complete.
- Purpose: Queues Attack/Use/Pick clicks between input events and ticks, then executes them in an order modeled on `Minecraft.handleKeybinds`.
- Implementation / logic: `InteractionInput` stores click counts for all three buttons. Each tick drains the queue and pins that tick's hit result. If an item is genuinely being used, clicks are discarded and a released use is released; otherwise attacks run first, then uses, then picks. It then checks held use against `use_delay`, runs `continue_attack`, and releases the hit result. `use_on_target` first tries an entity or block and falls back to the item if the action was not consumed. `release_on_screen_open` models paths where opening certain screens immediately releases use.
- Findings: Retaining one hit result for the whole tick is important: multiple clicks in a single tick should not acquire a different crosshair target after a block changes. Distinguishing active item use from a quick click depends on exact start/release lifecycle behavior. The Attack → Use → Pick order, discarding input during active use, and `continue_attack` after an instant attack are significant. Counters are `u32`, and incrementing on press can overflow during an input storm; in debug builds this may panic. Tests model five scenarios involving click order, use release, delay/digging, and a shared hit result; they were not run.
- RustChunk relevance: Very important for mobile and desktop: short taps may occur between ticks and must not be lost. Separating input-edge buffering from the gameplay tick is a suitable pattern for RustChunk.
- 1.20.1 implications: Comments and names refer to 26.3; behavior must be compared with 1.20.1 `Minecraft.handleKeybinds`, `startUseItem`, `MultiPlayerGameMode`, and hit-result logic. This is primarily gameplay lifecycle logic rather than wire protocol, but its outcomes become interaction/break/use packets; ordering and cancellation must be tested against a 1.20.1 server.
- Decision: **Tick buffering and a stable hit result are adaptable patterns; lifecycle behavior and edge cases must be rewritten/validated against 1.20.1 code.**
- Dependencies / cross-references: Minecraft.handleKeybinds, MultiPlayerGameMode startAttack/startDestroyBlock/releaseUsingItem, hitResult/pick lifecycle, screen set/release key mappings, player item-use state.
- Follow-up checks: Match the exact vanilla 1.20.1 input-tick order; test overflow/capacity, focus loss, touch cancellation, and multiple taps before a tick; inspect server acknowledgements for break/use synchronization.
- RustChunk implementation mapping: Maps to a future tick-boundary interaction queue for attack/use/pick actions, connecting input edges to 1.20.1 hit results and game-mode actions.
- Evidence required before adoption: Test taps shorter than one tick, multi-tap ordering, pointer cancellation, focus loss, queue overflow and exact vanilla 1.20.1 action/tick ordering.
- Completion state: complete source review; no tests or gameplay/server run were executed.

### 85. `crates/horizon-client-input/src/key_names.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: c6ba9d634bf247c73ba80a5e8f11b23a2c0b7599.
- Size: 9,297 bytes; 265 lines; complete file fetched.
- Coverage: lines 1–265, complete.
- Purpose: Tables mapping SDL scancodes and mouse-button IDs to Minecraft translation keys for displaying key names in the UI.
- Implementation / logic: The file explicitly states that its 26.3 `InputConstants` tables were generated by `VanillaOptionsTableExporter`. `KEYBOARD` contains SDL scancodes and `key.keyboard.*` localization keys; `MOUSE` contains button numbers 1–8 and `key.mouse.*` keys.
- Findings: The data is static/generated and contains no runtime logic or platform conversion. A scancode is not the same as a keycode or character text; keyboard layout and locale can make the displayed label differ from the character typed. The table includes rare, international, keypad, media, and Android soft keys. Because the source is a 26.3 exporter, completeness and naming/version parity with 1.20.1 are not guaranteed even if many names are stable. No tests are present in the file, and no tests were run.
- RustChunk relevance: A comparable mapping is useful for the keybind settings screen, display names for physical keys and mouse buttons, and keyboard UI.
- 1.20.1 implications: Translation keys and key enumeration must be extracted from the target version. Copying the entire 26.3 table could add later-version keys or omit required ones. SDL scancodes are relevant only if RustChunk actually uses an SDL backend; winit and Android may have different mappings.
- Decision: **Equivalent data can be generated/adapted; this 26.3 table must be regenerated for 1.20.1.**
- Dependencies / cross-references: SDL scancode definitions, winit keyboard backend, VanillaOptionsTableExporter, language resource keys, keybinding options UI.
- Follow-up checks: Extract the 1.20.1 table with an oracle; define separate mappings for winit/Android/SDL; check non-US layouts and physical-versus-logical key naming.
- RustChunk implementation mapping: Maps to generated key-name/translation data in a future input/settings layer; regenerate values from 1.20.1 rather than copying modern translation-key tables.
- Evidence required before adoption: Compare the complete 1.20.1 key/mouse name inventory with an oracle; test non-US layouts, unknown keys, Android mappings and stable saved identifiers.
- Completion state: complete source review; no tests or build were run.

### 86. `crates/horizon-client-input/src/keybinds.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: c1544a14f48e79ad2bd40ac30c5c732a4eb1545e.
- Size: 26,043 bytes; 626 lines; complete source fetched in consecutive ranges 1–400 and 401–626.
- Coverage: lines 1–626, complete.
- Purpose: Lists key mappings and their default/category/toggle metadata, persists and restores bindings, translates physical keys into actions, and defines keybind behavior in container UI.
- Implementation / logic: `MAPPINGS` is an ordered table of mapping names, defaults, and categories; `CATEGORIES` defines category order and `is_toggle` marks selected mappings as toggles. `KEYS` maps keyboard resource keys to winit `KeyCode` values. `key_code`/`key_name` provide bidirectional conversion; `keyboard_keys` sorts function keys first. `mouse_name`/`mouse_button` map mouse buttons. `learn_layout` is a no-op on Android and learns a layout label from winit events and `key_without_modifiers` on other platforms. `KeyBinds` stores values in a `BTreeMap`, handles conflicts/defaults, finds all bindings sharing a key, and translates physical keys into vanilla-default actions. Debug mappings are excluded from translation so they retain independent F3-modifier behavior. `container_mouse_key` and `container_hotbar_key` model offhand-swap and hotbar priority in container screens.
- Findings: The `MAPPINGS` table uses 26.3 defaults and categories and includes newer actions (such as friends/quickActions and debug bindings); it must be compared with 1.20.1 `Options.keyMappings`. `KEYS` is only a subset of key names. An unknown mapping returns `None` from `key_code`, and `translate` passes through a physical `KeyCode` when no name exists. `mouse_button` decodes only Left/Right/Middle/Back/Forward, while `mouse_name` also produces names for `Other`; round-tripping buttons 6+ is not guaranteed. `set` accepts only registered mappings but does not validate the key string. Reassigned default keys become inert, and multiple mappings can trigger from one physical input. Tests cover rebinding, conflicts, default-action removal, container priority, and display names; none were run.
- RustChunk relevance: Highly relevant to keyboard/mouse/touch/gamepad action mapping and preserving vanilla gameplay semantics when users customize keybinds. Normalizing input to default action IDs can keep game logic independent of physical keys.
- 1.20.1 implications: Extract the mapping catalogue, category order, toggles, mouse behavior, and persistence schema from 1.20.1 `Options`. Feed vanilla game code with logical actions rather than direct physical keys. Android touchscreen buttons should produce the same action identifiers, not synthetic key names inserted into OS input.
- Decision: **The KeyBinds abstraction and physical-key-to-action translation are adaptable; MAPPINGS/KEYS and container behavior must be regenerated or adapted for 1.20.1.**
- Dependencies / cross-references: winit KeyCode/MouseButton and KeyEvent; horizon_client_core::key_display; SDL/translation key names; Options.keyMappings ordering; screen container hotbar checks; debug_keys physical key handling.
- Follow-up checks: Compare the exact mapping list with 1.20.1; test keyboard layouts, persistence of unknown keys, other mouse-button IDs, duplicate bindings, focus loss, and Android touch-action routing.
- RustChunk implementation mapping: Maps to future keybind defaults, persistence and action translation in the input/settings layer. Existing app settings are not a replacement for vanilla 1.20.1 game keybinds.
- Evidence required before adoption: Verify defaults/categories/toggle behavior against 1.20.1 Options; test persistence/migration, duplicate bindings, unknown keys, mouse buttons and touch-action mappings.
- Completion state: complete source review; tests/build not executed.

### 87. `crates/horizon-client-input/src/keymapping.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 5f4388d90d1524b3a5923a2e45b2f2d5576b0b82.
- Size: 11,388 bytes; 308 lines; complete file fetched.
- Coverage: lines 1–308, complete.
- Purpose: Runtime state for held/click/toggle key mappings and their behavior across screen open/close, key repeat, controller toggles, and respawn.
- Implementation / logic: `KeyState` stores `is_down`, `click_count`, screen-triggered release state, the toggle option, keyboard origin, and `pad_toggled`. With `ToggleKeyMapping` enabled, `set_down` turns a press into a toggle and ignores the normal release; in hold mode it directly tracks the value. `click`/`consume_click` manage queued click edges. `should_set_on_ingame_focus` and release/reset/restore model the screen lifecycle. `ToggleKeys` has four slots for sneak, sprint, attack, and use, and handles Press/Repeat/Release actions, mouse buttons, pad toggles, screen open/close, and respawn.
- Findings: In this model, opening a screen blocks key presses but still accepts releases to prevent stuck keys. Toggle state is controlled by `shouldRestore` and keyboard origin. `pad_toggled` is a Horizon extension that makes controller sneak behave as a toggle even when the option is disabled. `ToggleKeys::key` also increments click count for Repeat, so winit repeat behavior must be aligned with vanilla event handling. `click_count` is an unsaturated `i32` and can overflow under a large event burst. Tests model screen lifecycle, toggle/hold, pad toggling, and respawn; they were not run.
- RustChunk relevance: Important for touch, gamepad, and keyboard controls. The state model prevents sneak/use/attack from remaining stuck when a menu opens. Physical events can be converted into key-action state, keeping the gameplay layer independent of hardware.
- 1.20.1 implications: Compare vanilla 1.20.1 `ToggleKeyMapping` and `KeyMapping` behavior with the target source; do not assume the four mappings or pad-specific toggle extension are part of 1.20.1. On Android, pointer cancellation/focus loss and controller disconnection must also produce Release/cleanup events.
- Decision: **The base state machine is adaptable; pad toggling and event/screen semantics must be aligned with the 1.20.1 and Android implementations.**
- Dependencies / cross-references: Options toggleKeySneak/toggleKeySprint/toggleKeyAttack/toggleKeyUse; KeyMapping and ToggleKeyMapping; input event dispatch; winit keyboard/mouse events; controller toggle-sneak extension.
- Follow-up checks: Match the exact 1.20.1 `KeyMapping`/`ToggleKeyMapping`; inspect repeat semantics, focus loss, pointer cancellation, controller disconnection, and click-counter bounds.
- RustChunk implementation mapping: Maps to future held/click/toggle key state management across screens, respawn and Android focus changes.
- Evidence required before adoption: Compare KeyMapping/ToggleKeyMapping behavior with 1.20.1; test key repeat, open/close screens, focus loss, pointer cancel, respawn and controller disconnect without stuck actions.
- Completion state: complete source review; no tests or build were run.

### 88. `crates/horizon-client-input/src/modifiers.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: f5e183660f04ceabc63923e5c4ddcfa5f0daae65.
- Size: 8,020 bytes; 232 lines; complete file fetched.
- Coverage: lines 1–232, complete.
- Purpose: Keeps OS modifier bits synchronized with the physical-key set and temporarily synthesizes modifiers for agent/controller shift-click.
- Implementation / logic: `set_key` adds/removes the left/right keys for each modifier in a HashSet and clears the corresponding bit only when neither side remains held. `hold` snapshots the previous physical-key and modifier-bit state, then activates the modifier synthetically; `restore` reinstates the previous state. `reconcile` clears keys when an OS modifier bit is off even though physical keys are still marked held, and releases `ToggleKeys` if the key was rebound to sneak/sprint and no controller hold is active.
- Findings: This module reconciles OS-level modifier state with the programmatically maintained physical-key set; it explicitly handles the right key remaining held when the left key is released. `reconcile` checks the current `KeyBinds` binding so that losing SHIFT does not accidentally release sneak when sneak has been rebound to another key; pad-held state and toggle options are also preserved. `hold`/`restore` snapshot an aggregate bit; if an independent modifier event arrives between them, `restore` can reinstate stale state. This API should therefore be used for short synchronous clicks or refined with a more precise state token. Tests cover left/right modifiers, synthetic hold/restore, and lost-release/rebinding/pad cases; they were not run.
- RustChunk relevance: Useful for Android modifier reconstruction and macro/controller-generated input. Preventing sticky modifiers and accidental sneak/sprint release is important for gameplay.
- 1.20.1 implications: Check `ToggleKeys` semantics against the target version; reconciliation and the physical-key set are primarily platform-input concerns. On Android, where winit modifier events may be incomplete, use actual key-down/up events and focus loss as the source of truth rather than the bit field alone.
- Decision: **Modifier reconciliation and paired-key logic are adaptable; snapshot restoration must be synchronous/serialized and tested against the 1.20.1 key lifecycle.**
- Dependencies / cross-references: winit ModifiersState/KeyCode; keybinds KeyBinds; keymapping ToggleKeys; Android key event bridge; controller agent/touch synthesized input.
- Follow-up checks: Test interleaving of real modifier events during hold/restore, IME/Android modifier state, focus loss, and key cancellation; inspect keybindings rebound to Shift/Control themselves.
- RustChunk implementation mapping: Maps to the future input platform adapter's modifier-state reconciliation, particularly synthetic touch/controller modifiers versus physical keyboard state.
- Evidence required before adoption: Test real modifier events interleaved with synthetic hold/restore, focus loss, Android IME, paired modifiers and exception/cancel paths; no modifier may remain stuck.
- Completion state: complete source review; tests/build not executed.

### 89. `crates/horizon-client-input/src/mouse.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 7e60b81c16afed63139f36ec416bd1a4f0791c57.
- Size: 19,239 bytes; 568 lines; complete source fetched in consecutive ranges 1–400 and 401–568.
- Coverage: lines 1–568, complete.
- Purpose: camera mouse-look, pointer delta accumulation, sensitivity curve, cinematic smoothing, invert/scoping behavior, scroll-wheel processing, spectator flight-speed scroll and macOS Control-click emulation.
- Implementation / logic: SmoothDouble accumulates target/remaining deltas with half-step smoothing; java_signum preserves signed zero/NaN semantics. ScrollWheelHandler accumulates fractional X/Y wheel offsets, resets accumulator on direction reversal and emits whole steps. Rotation implements Entity.turn with 0.15 degree/count and pitch clamp. MouseLook collects relative deltas while grabbed or absolute position deltas otherwise; ignores first motion after reset, aggregates per frame, optionally applies smooth-camera or scoping sensitivity, then inversion. sensitivity_scales uses (s*0.6F+0.2F)^3 and sens*8. on_scroll supports discrete wheel events and wheel sensitivity. Additional helpers invert degrees to mouse counts, cosine easing, center pointer after grab state transition and preserve Control-click-as-right-click through release.
- Findings: implementation is explicitly version 26.3. The sensitivity multiplication uses Java float constants widened to f64 to reproduce exact rounding; yaw/pitch are converted back to f32 before turn. smooth camera uses elapsed frame time and retains residual movement across later frames. Scroll fraction handling and direction reset matter for high-resolution touchpads. next_selection repeatedly wraps selected index but assumes limit > 0; limit 0 would loop indefinitely and should be guarded at call boundary or helper. mouse_position_after_grab_change divides by scale without checking zero/non-finite scale. macOS Control-click state is captured at press and retained to release even if Control key state changes. Tests cover vanilla sensitivity formula, scoping/inversion, inverse motion, cinematic smoothing, fractional scroll, pointer centering and Control-click. Tests not run.
- RustChunk relevance: valuable for accurate camera and scrolling on desktop, and for using relative deltas/virtual mouse-look from touch/gamepad/agent input; sensitivity consistency impacts multiplayer aiming and view direction.
- 1.20.1 implications: all sensitivity/cinematic behavior should be compared to the 1.20.1 MouseHandler/Entity.turn source rather than assumed compatible. Android generally uses touch-look rather than OS mouse grab; keep a platform-independent accumulated-delta API and place sensitivity/gesture handling in platform adapter. Pointer center semantics only apply when a real pointer grab changes.
- Decision: **core mouse delta aggregation, sensitivity curve and scroll accumulation are adaptable; 1.20.1 constants and platform-specific grab semantics require verification**.
- Dependencies / cross-references: winit pointer events, MouseHandler/Entity.turn behavior, sensitivity/smoothCamera/invertMouseX/invertMouseY/scoping options, spectator abilities, window DPI scale and macOS Control-click option.
- Follow-up checks: verify 1.20.1 exact mouse sensitivity and smoothing; guard positive scroll limit and nonzero scale; compare Android touch-look conversion and DPI/rotation with game coordinate space.
- RustChunk implementation mapping: Maps to future camera look/scroll input and desktop pointer handling; Android touch-look should feed a common camera-action API rather than assume desktop pointer grab.
- Evidence required before adoption: Compare sensitivity/smoothing/scroll semantics with 1.20.1 vanilla; test zero/invalid scale, DPI/rotation, touch-look conversion and focus/pointer-grab lifecycle.
- Completion state: complete source review; no tests/build/input-device runs executed.

### 90. `crates/horizon-client-input/src/window.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 71dcc3d59839a11eca7beca8852c74808a3c7435.
- Size: 33,729 bytes; 958 lines; complete source fetched in consecutive ranges 1–400, 401–800 and 801–958.
- Coverage: lines 1–958, complete.
- Purpose: Handles window sizing/GUI scale, scaled mouse position, inactivity FPS throttling, frame pacing, present mode/VSync, fullscreen state, and monitor video modes.
- Implementation / logic: `calculate_scale` uses a minimum GUI space of 320×240, a maximum scale, and Unicode even-scale handling. `gui_scaled_size` converts framebuffer dimensions using ceiling, while `scaled_pos` maps mouse coordinates into screen space. `throttle_reason` and `framerate_limit` select 10/30/60 FPS or the configured limit for minimized/AFK/menu states. With VSync enabled, `vsync_present_mode` selects FIFO; without VSync, it prefers Immediate/Mailbox/FIFO in that order. `FramerateLimiter` uses an injectable clock, tracks overshoot with an EMA, sleeps until near the deadline, then spins; `limit_on_grid` bases each frame slot on the prior deadline to prevent late wake-ups from drifting average frame rate or causing bursts after a long frame. `FullscreenState` separates requested and actual fullscreen and accepts compositor changes. VideoMode JSON read/write handles refresh rate/bit depth and per-monitor mode caching. The icon PNG is created from an embedded theme asset.
- Findings: The file explicitly targets 26.3. `FramerateLimiter` applies the cap only for FPS values below 260; dividing the frame period by framerate assumes a positive limit. `gui_scaled_size` and `scaled_pos` assume positive GUI scale/screen dimensions; zero scale would divide by zero. With Unicode enforcement enabled, `calculate_scale` may exceed `max_scale` by adding one to an odd result; this matches the current test but must be checked against target vanilla semantics. `VideoMode.read` manually parses JSON by searching property names and does not fully validate JSON grammar, duplicate keys, or complete input; a serde JSON parser would be safer. `from_winit` approximates total bit depth by dividing it among channels. `FramerateLimiter` has two modes: vanilla-like sleep/spin and grid pacing. `SleepingClock` replaces spinning with 50µs sleeps to reduce busy CPU use, with different granularity/latency. Extensive tests cover scaling, fullscreen transitions, FPS caps, timer overshoot, grid recovery, VSync, and monitor caching; they were not run.
- RustChunk relevance: FPS pacing, GUI scaling, and pointer scaling matter for Android performance and responsiveness. Grid deadlines may prevent frame-pacing drift, but the sleep/spin trade-off must be measured on ARM mobile devices and Mali GPUs.
- 1.20.1 implications: Window/MouseHandler/Options values must be compared with 1.20.1; there is no direct protocol dependency. Use the 1.20.1 game loop and frame limiter as the oracle, while RustChunk may build an independent Android timing backend. The mobile backend must account for display refresh rate, actual surface present mode, app background lifecycle, and battery/thermal state. Exclusive fullscreen and monitor switching differ substantially or are unavailable on Android.
- Decision: **GUI scaling and throttling concepts are adaptable; the FPS limiter needs Android-specific design and measurement; fullscreen/video-mode implementation is primarily desktop-only and must be adapted or omitted.**
- Dependencies / cross-references: winit Window/MonitorHandle/VideoModeHandle/Fullscreen, renderer surface capabilities, game loop end_frame, Options GUI scale/VSync/FPS cap/fullscreen, embedded PNG theme asset, system clock/sleep and thread interruption.
- Follow-up checks: validate positive limits/scales and strict JSON; measure CPU/battery/frame pacing on ARM64 Android; confirm winit/GPU present modes for GLES/Vulkan; verify 1.20.1 scale and throttle constants; skip desktop fullscreen assumptions on Android.
- RustChunk implementation mapping: Maps to future Android display/window abstraction, GUI scale and frame-pacing controls. Current app Activity is not the Starling window/game loop implementation.
- Evidence required before adoption: Measure frame pacing and input/GUI scale on the target device; test resize/rotation/background throttling/VSync and verify no busy-wait strategy harms thermals or battery.
- Completion state: complete source review; no tests/build, benchmark or Android device measurement performed.

### 91. `crates/horizon-client-input/src/clipboard.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 1326a3d0ffb9f6f069e0d90d6148d318f46ddac2.
- Size: 18,716 bytes; 535 lines; complete source fetched in consecutive ranges 1–400 and 401–535.
- Coverage: lines 1–535, complete.
- Purpose: Clipboard abstraction, keyboard-editing shortcuts, chat-text filtering, text-component formatting removal, and selection behavior for append-only text fields.
- Implementation / logic: `shortcut_letter` obtains A/C/V/X from the logical layout and falls back to the physical key for non-Latin logical text. `edit_shortcut` accepts only the edit modifier (Cmd on macOS, Ctrl elsewhere), without Shift/Alt. `types_text` makes an exception for AltGr, which Windows reports as Ctrl+Alt. `is_allowed_chat_character` and `filter_text` remove § and control characters and allow newlines only in multiline fields; `strip_formatting` removes recognized color/style codes. The Clipboard trait is implemented by `SystemClipboard`, Android platform hooks, and `MemoryClipboard`; the last copied text is retained in memory. Editable shortcuts implement SelectAll/Copy/Paste/Cut. `AppendSelection` keeps a full-value selection for append-only fields such as names/signs/book pages until the field ID or value changes.
- Findings: The file explicitly models 26.3 input/text and clipboard behavior. The Android backend uses `OnceLock` platform-function hooks, which cannot be replaced after being set. Loss of the desktop clipboard causes a system-backend reset; the copied fallback text is retained, but `get` uses it only when the system object is unavailable. `ContentNotAvailable` returns an empty string directly. `filter_text` removes newlines from chat but permits them in multiline fields; it does not remove C1 characters (including U+0080). `strip_formatting` removes only valid formatting codes and preserves §z. `append_limited` returns true whenever room remains, even if filtering makes the input empty or no character is appended; this may violate a “value changed” return contract. `AppendSelection` invalidates its selection using a snapshot of the entire value. Tests cover shortcuts/AltGr, layout fallback, filtering, edit fields, append-only behavior, and rejected paste; they were not run.
- RustChunk relevance: An Android clipboard bridge is needed for copy/paste. Chat sanitization, multiline book/sign fields, and key shortcuts also matter for a complete client. A platform-independent clipboard abstraction is appropriate.
- 1.20.1 implications: Extract the chat-character allowlist, formatting-code behavior, shortcut conditions, and sign/book constraints from 1.20.1. RustChunk on Android should use the native clipboard API with appropriate lifecycle/permissions; a static hook must not make UI calls from an invalid thread. Chat filtering must not over-restrict valid Unicode player text or protocol chat-component payloads.
- Decision: **The Clipboard trait and append-selection pattern are adaptable; filter/chat/shortcut rules and the Android bridge must be reviewed against 1.20.1 and the Android platform.**
- Dependencies / cross-references: winit Key/PhysicalKey; arboard desktop clipboard; Android platform clipboard callbacks; horizon_client::platform::set; EditBox/TextFieldHelper/ChatFormatting/StringUtil and sign/book text input.
- Follow-up checks: validate clipboard callbacks thread affinity and lifecycle; check append_limited return contract; test Unicode codepoint/UTF-16/byte cursor behavior, multi-line filters and 1.20.1 text length rules.
- RustChunk implementation mapping: Maps to a future clipboard/text-editing interface between Android APIs and in-game chat/book/sign fields. Existing UI clipboard code, if any, must be traced before reuse.
- Evidence required before adoption: Test Android copy/paste, Unicode and UTF-16 cursor positions, selection, multiline filters, formatting removal and 1.20.1 chat/book/sign length limits.
- Completion state: complete source review; no tests/build or Android clipboard session executed.

### 92. `crates/horizon-client-input/src/gamepad.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: cb36e0c3661155d910cc2b87d48eb6400f57a24d.
- Size: 63,778 bytes; 1,681 lines; complete source fetched in consecutive ranges 1–400, 401–800, 801–1200, 1201–1600 and 1601–1681.
- Coverage: lines 1–1,681, complete.
- Purpose: Backend-independent controller layer implementing button/action mapping, state and edges, deadzones/look curves, menu navigation, rumble, adaptive triggers, lightbar, and controller-specific settings.
- Implementation / logic: `Button` models 16 standard gilrs/SDL positions and provides labels/colors for Xbox/PlayStation/Nintendo. `PadState` stores buttons as a bitset plus stick/trigger values; it merges multiple sources and extracts press/release edges using trigger hysteresis. `ACTIONS` maps Bedrock-style defaults to vanilla key actions and pad-only hotbar actions. `PadSettings` stores controller settings in `horizon-controller.txt` next to options and serializes changed bindings. The module implements radial deadzone, `move_vector`, `java_move_vector` (approximation to Java's eight-direction movement), `look_delta`, `cursor_delta`, `focus_step`/`focus_step_rect`/`focus_step_vague`, `scroll_shift`/`refind`/`tab_step`/`slider_step`, a health watcher, and rumble pulse shapes. With the `controllers` feature, `TriggerFeel` maps to DualSense adaptive-trigger effects; the lightbar displays health/dimension, and `bow_power` derives force from draw ticks.
- Findings: The file states that controller support is a Horizon feature and vanilla 26.3 itself has no gamepad input; the entire feature must not be attributed to vanilla. UI glyphs and ACTIONS layout, defaults, pad-only controls, Steam Deck keyboard detection, server stick modes, and rumble/adaptive triggers are extensions. `java_move_vector` selects the nearest of eight directions using thresholds so movement on Java servers approximates boolean-direction packets; this differs from free analog movement and is selected by ServerStick mode. The settings parser is tolerant and defaults unknown/malformed values; it clamps floats to finite ranges. Radial deadzone normalization uses denominator `(1-deadzone)` and the public API assumes `deadzone<1`; `PadSettings` clamps it to at most 0.9, but direct callers must preserve this invariant. `PadState.merge` sums/clamps controller sources; edge hysteresis prevents trigger chatter. UI focus navigation for target rectangles uses a detailed geometry algorithm. Tests cover mapping, persistence, deadzone, edge hysteresis, menu focus, settings, rumble, lightbar, trigger effects, and movement sectors; none were run.
- RustChunk relevance: Controllers/gamepads can be a second input layer on Android/desktop. The state/edge/deadzone/action abstraction and eight-way Java movement are worth studying. Even if the first release supports only touch, an action-based API allows controllers to be added later without rewriting gameplay.
- 1.20.1 implications: Vanilla 1.20.1 has no built-in gamepad support; if added, this subsystem should remain a separate client-side input adapter. Network movement packets must still follow protocol 1.20.1; analog-stick input should map to that version's PlayerInput/command directions. Java eight-way conversion and server-stick options must match 1.20.1 client/server behavior and prediction, not introduce a custom protocol. Item-use, attack, sneak/sprint, hotbar, and focus events must connect to the same action IDs/gameplay tick.
- Decision: **PadState/Edges/action-map architecture and radial deadzone are adaptable; layout, tuning, navigation, and haptics should be rewritten/ported as optional features, not required parts of the 1.20.1 core.**
- Dependencies / cross-references: gilrs/SDL controller layout (documented in module), android submodule, horizon-controller.txt, horizon_dualsense optional feature, KeyBinds/KeyMapping/KeyboardInput, Player movement packet/remote prediction, UI focus rectangles, health/dimension/item state.
- Follow-up checks: If gamepad support is added to RustChunk, choose gilrs or the Android native event path, normalize HID axis maps, and handle disconnect `release_all` and focus loss. Validate radial-deadzone boundaries, analog-vs-eight-direction server behavior, frame-time units, rumble limits, and DualSense firmware behavior on real hardware.
- RustChunk implementation mapping: Optional future controller input backend; keep it separate from the first touchscreen/keyboard path. Do not make gamepad support a prerequisite for the native 1.20.1 core.
- Evidence required before adoption: Only adopt after choosing Android/desktop backend and validating HID mappings, axis calibration/deadzone, disconnect release-all, haptics policy and gameplay action parity.
- Completion state: complete source review; no tests, controller hardware, haptics, Android backend or network interoperability tests were executed.

### 93. `crates/horizon-client-input/src/gamepad/android.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 493a81f5afe27accf2d7cec4adb93fdeda63a1bf.
- Size: 12,201 bytes; 362 lines; complete file fetched.
- Coverage: lines 1–362, complete.
- Purpose: Converts Android InputDevice keycodes and MotionEvent axes into the same PadState abstraction that desktop obtains through gilrs.
- Implementation / logic: `from_pad` and `joystick` detect controller events using Android source masks. Android button keycodes are mapped to 16 Button positions; `pad_key` requires a controller source for D-pad input so keyboard arrow keys are not misclassified as D-pad. Axes include X/Y, right-stick Z/RZ, hat, and two trigger naming conventions. `AndroidPad` combines button events and the latest axis event into `PadState`; it inverts Y axes, zeros stick-rest noise below 0.1 and trigger noise below 0.05, and treats key-trigger events as full pull. Hat values map to D-pad directions at ±0.5. `AndroidPads` tracks multiple pads by device ID and exposes only the most recently used active pad to the game; key presses always take control, while motion takes control only when button/stick/trigger activity exceeds 0.5.
- Findings: Source classification and Android keycode values are platform-specific and must be matched to the Android SDK/NDK event API and the selected winit version. The AndroidPad stick-rest threshold is a noise filter applied before the radial gameplay deadzone. Triggers reported as keycodes are converted to full axis values and removed from the button bitset so the main Edges layer can apply trigger hysteresis. Multiple axes (LTRIGGER vs BRAKE, RTRIGGER vs GAS) are merged by taking the maximum. This file has no disconnect/remove_device/clear-state API in `AndroidPads`; if the event bridge does not report device removal elsewhere, the last buttons/axes can remain stale. The pad list is not pruned and the number of devices is not capped. Tests cover key mapping, source classification, stick axes, hat, trigger mapping, and last-active-pad switching; they were not run.
- RustChunk relevance: Highly relevant to physical Android controllers and conversion of input into a common PadState. This pattern can route touch emulation and gamepads through one action layer.
- 1.20.1 implications: This Android backend is not protocol-specific; its PadState must feed the movement/action semantics of the 1.20.1 game. Verify InputDevice constants against the target API level (Android 13/API 33 on the test device) and winit event handling. Explicitly handle controller disconnection, app pause/resume, multiple-controller selection, and gamepad-source masquerading.
- Decision: **The common-state adapter structure is adaptable; Android key/source codes and noise/takeover thresholds require device testing and a complete disconnect lifecycle.**
- Dependencies / cross-references: Android InputDevice.SOURCE_GAMEPAD/SOURCE_CLASS_JOYSTICK; KeyEvent.KEYCODE_BUTTON_*; MotionEvent AXIS_X/Y/Z/RZ/HAT/LTRIGGER/RTRIGGER/BRAKE/GAS; gamepad::PadState/Edges/PadSettings; horizon-android event bridge and game/pad/android.rs poller.
- Follow-up checks: Inspect disconnect/remove-device and pause/resume paths; validate HID axis maps and keycodes on several Android controllers; verify that zeroed axes are delivered to the active pad on focus loss and controller removal.
- RustChunk implementation mapping: Maps to a future Android `InputDevice`/`MotionEvent` adapter feeding the common input-action state. This is not implemented by the current JNI downloader/installer crate.
- Evidence required before adoption: Validate keycodes/source masks/axis values on physical Android controllers, including device removal, app pause/resume, noisy axes and release-all; test on the target API level.
- Completion state: complete source review; no Android device tests or controller hardware tests were executed.

### 94. `crates/horizon-client/src/control.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 96a8976a479b8639e26585285211f8bd12b862e6.
- Size: 24,849 bytes; 689 lines; complete source fetched in consecutive ranges 1–400 and 401–689.
- Coverage: lines 1–689, complete.
- Purpose: local control socket for scripts/horizon-ctl to send text-line commands to a running game, with Unix socket or authenticated localhost TCP transport; also frame timing/performance statistics used by control replies.
- Implementation / logic: Stream abstracts UnixStream/TcpStream; uses_tcp selects TCP on non-Unix or .json endpoint. TCP endpoint metadata stores port/token/remote flag in a private file; client reads it, connects to localhost and sends auth TOKEN. Server writes a random localhost port + token to endpoint path and starts listener/client threads. TCP first line is authenticated with same_token bytewise comparison; clients send each command as a line and wait for one JSON reply from game thread. Requests enter mpsc channel and Server::try_recv exposes them to the main/game thread. Server::drop marks closed, wakes listener and removes endpoint. Helpers encode/decode base64 screenshots. FrameStats records frame/CPU/GPU/phase durations and static/dynamic entity cache counters, with percentiles and low-1% FPS summaries.
- Findings: TCP server is bound to IPv4 loopback even when HORIZON_CONTROL_PORT is fixed; adb port forwarding is the documented mobile path. Unix endpoint permissions set 0600 only via OpenOptionsExt on Unix. random_token is constructed from RandomState hasher keys + time + PID; RandomState is not specified as a cryptographic CSPRNG, so use getrandom/OS CSPRNG for bearer tokens. authenticate reads at most 256 bytes but does not reject a line that reaches the cap without newline; serve uses BufRead::lines with no maximum command-line size. No concurrent-client or request-rate cap exists; listener spawns a thread per accepted client and mpsc::channel is unbounded. Each serve thread waits synchronously for a game-thread reply without timeout. File permission behavior and Windows ACL semantics need verification. base64/unbase64 round-trip tests exist, but unbase64 does not validate RFC 4648 length modulo 4 or canonical padding bits; an invalid one-character input may decode to an empty byte vector instead of None. FrameStats caps millis/cpu/phases at LIMIT but record_gpu extends the full supplied slice whenever the current length is below LIMIT and can exceed the cap; summary also assumes useful positive frame durations for avg_fps. Tests cover TCP/auth, base64, token differences, remote marker and stats counters; none were run.
- RustChunk relevance: not required for vanilla gameplay or server connectivity. Could become a useful optional diagnostics/automation bridge for remote Android control and performance reports, but it expands attack surface and should not be embedded into the first playable client.
- 1.20.1 implications: independent of Minecraft protocol. If ever included, keep it on loopback/explicit adb-forward use, authenticate with OS-random token, cap command length/client count/queue/response wait, validate JSON/base64 strictly and dispatch only allowlisted commands on game thread. Never expose an unauthenticated remote listener or accept arbitrary commands from LAN.
- Decision: **unnecessary for the base 1.20.1 client; optional tooling only after a separate security hardening pass**.
- Dependencies / cross-references: std Unix/TCP sockets, mpsc/threading, serde_json, horizon client renderer StaticCacheFrameStats, logging/private_home path, screenshot reply and external horizon-ctl client.
- Follow-up checks: audit command dispatcher/allowlist in game.rs; replace token source with CSPRNG; enforce bounded lines/requests/connections/timeouts; strict RFC 4648 decoding and bounded GPU stats; verify file permissions and shutdown behavior on Android.
- RustChunk implementation mapping: No production mapping. A control socket is optional developer/test tooling only and must not be exposed as an unauthenticated game control surface.
- Evidence required before adoption: Keep absent from the first release. Before any adoption require CSPRNG authentication, loopback/ADB-only binding, bounded messages/queues/timeouts, command allowlist and adversarial protocol tests.
- Completion state: complete source review; no socket server was launched and no security tests/build were executed.

### 95. `crates/horizon-client/src/touch.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: acc7dbd7b2ef2a1020ab7f0895c92e018e669be9.
- Size: approximately 111,426 bytes; 2,848 lines; complete source fetched in seven consecutive ranges: 1–400, 401–800, 801–1,200, 1,201–1,600, 1,601–2,000, 2,001–2,400 and 2,401–2,848.
- Coverage: lines 1–2,848, complete.
- Purpose: Full phone touch-control state machine: touch joystick/D-pad, buttons, tap/hold interaction, mouse-look, hotbar, multi-finger gestures, layout editor, JSON layout persistence, and conversion of touch to mouse events for menus.
- Implementation / logic: `LayoutConfig` stores responsive layout positions relative to screen width/height, control sizes, and opacity. A backward-compatible JSON parser supports Show On/Ghost/Off, Tap/Crosshair, Sneak Hold/Toggle, fixed/floating sticks, D-pad, gyro, three-finger stats, and custom/mod buttons. Layout calculations use framebuffer size and GUI scale to position the joystick, buttons, extra controls, hotbar, and close target. `TouchControls` claims an independent role for each finger: Stick, Look, Button, DPad, or Spent; edge actions are sent to the game loop. A view tap is separated from HoldStart/HoldStop for break/use at HOLD=300ms; drag beyond the slop threshold moves the camera. In Tap mode, a quick tap targets the point under the finger; Crosshair mode displays Attack/Use buttons. Pause activates on release, while EDIT_HOLD=800ms opens the editor. The D-pad converts eight directions to a stick vector and a double-tap up triggers Sprint. In UI screens, `TouchPointer` maps the first finger to a cursor/mouse and distinguishes tap, vertical scroll, sideways drag, scrollbar drag, and hold-to-drag; cancellation/background sends a synthetic Release to prevent stuck mouse buttons. `LayoutEditor` supports drag/resize/visibility/opacity, mode changes, and custom-button deletion.
- Findings: This module is touch-native and is not part of vanilla 1.20.1; its design is intentionally Bedrock-style. Per-finger ownership prevents a finger touching one button from unexpectedly taking control of another. `reset()` releases fingers but preserves toggle-sneak (so screen toggling matches vanilla); `clear_sneak()` must be called on respawn. The three-finger chord is active only when enabled and uses CHORD=350ms; it neutralizes actions started by newly participating fingers. `LayoutConfig` supports legacy `visible` and old movement names and ignores unknown keys; custom buttons are capped at 64 and persisted mod layouts at 256. `Shape::hits` uses a circle for Round and bounding boxes with 1.15 tolerance for Square/Wide; ordinary buttons in `button_at` are circles only, while custom Shape is used for mod controls. `stick_tilt` assumes a positive radius and `look_scale` divides by height, so window height must be positive. `slot_at` uses GUI-scaled hotbar geometry. In `TouchPointer`, only the first finger acts as the pointer and other fingers are ignored; `cancel()` during app background sends Release only, not a click. Layout depends on aspect ratio, density, and GUI scale; defaults in this file target 20:9 phones. Extensive tests cover joystick/sprint, fixed stick, chord, custom buttons, pointer gestures, pause long press, JSON compatibility, layout editor, sneak modes, D-pad, mod controls, hotbar, tap/hold/look/crosshair, gyro, and F3 chips; no tests were run in this audit.
- RustChunk relevance: Highly relevant to running 1.20.1 on mobile touchscreens. The strongest ideas are per-pointer ownership, buffering events into renderer-independent actions, separating touch camera gestures from tap/hold interaction, persistent layout settings, and cancelling/resetting all gestures during screen/background lifecycle changes.
- 1.20.1 implications: Port this module as a separate Android input/UI adapter, not as part of the protocol. Actions must connect to vanilla 1.20.1 `LocalPlayer`/`GameMode` and PlayerInput/interaction packets. In particular, validate Tap/Crosshair differences, attack/use sequencing, sneak toggles, break-hold duration, relative camera motion, hotbar bounds, and screen mouse behavior against the 1.20.1 source or oracle. Since RustChunk must first bring up the base 1.20.1 client, its first version can use simpler touch input and defer the layout editor, gyro, mod buttons, Bedrock-style D-pad, and three-finger stats.
- Decision: **The touch-event state machine and ownership/action abstraction are highly adaptable; layout/UI gestures and Bedrock-specific interactions must be rewritten as a separate Android layer and matched to vanilla 1.20.1.**
- Dependencies / cross-references: winit Touch/WindowEvent/MouseInput; settings CustomAction/CustomButton; LayoutConfig JSON storage at touch-layout.json; own_buttons registry; horizon-android event bridge; game/touch.rs action consumer; GameMode interaction and MouseHandler; GUI-scaled hotbar.
- Follow-up checks: Inspect all Action consumers in `game/touch.rs`; review screen/background/rotation lifecycle in the Android activity; test multi-touch cancellation/pointer IDs and GUI scaling across several aspect ratios; compare Tap/Crosshair interactions with a 1.20.1 oracle and NBT/world-interaction path.
- RustChunk implementation mapping: High-priority future Android touch adapter. The planned engine map already identifies Android input as a separate platform layer; Starling pointer ownership/action buffering can inform it, not be copied blindly.
- Evidence required before adoption: On the Galaxy A22, test simultaneous joystick/look/action fingers, cancellation, rotation, overlays and GUI scaling; compare each emitted action with vanilla 1.20.1 movement and interaction behavior.
- Completion state: complete source review; no tests, build, touch-device validation or gameplay/server interaction test was executed.

### 96. `crates/horizon-client/src/bedrock_touch.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 77d85e24594dca8f276ddf1028983d0992478930.
- Size: 376 lines; complete file fetched.
- Coverage: lines 1–376, complete.
- Purpose: Fetches touch-control sprite assets from Mojang's Bedrock sample repository and converts selected assets into Starling sprites for displaying touch buttons.
- Implementation / logic: The embedded `assets/bedrock-touch.json` manifest specifies the repository/revision, file list, Git blob SHA-1 values, and license file. `present` checks a marker containing the revision and all generated glyph names to decide whether the assets are ready. Fetching occurs only when assets are missing and `resources/assets/minecraft` exists. An HTTP agent with 15-second connect and 30-second read timeouts downloads files from `raw.githubusercontent.com` at the pinned revision; each SHA-1 is compared using Git blob format, i.e. the hash of `"blob <length>\0" + bytes`. Plate/plate_pressed images are decoded and their icon regions converted to grayscale; Starling glyphs are stamped as bitmaps onto the plate and pressed-plate images and encoded as RGBA PNGs. The license file is stored beside the assets, and the marker is written last. `fetch_logged` logs errors and treats network failure as non-fatal.
- Findings: This file is an asset pipeline only; it contains no game-input/gameplay logic. The pinned revision and Git blob SHA-1 verification help detect silent file changes; SHA-1 here identifies content as a Git blob and is not a security signature or publisher authentication. The manifest is embedded in the binary and `present` reads only the marker. If PNGs are deleted or corrupted after the marker is written, `present` may still return true unless another resource validator checks them. An intermediate fetch failure can leave partial files, but because the marker is written last, the next run retries the fetch. This file does not limit maximum download size/bytes or define redirect policy; the repository/revision/source come from the fixed build manifest, so the main exposure is the endpoint and pinned remote assets. The documented precedence is: player resource packs override Starling and Bedrock assets. Normal tests cover Git blob hashing, sprite-name parity, and glyph rasterization; the actual network test is marked `#[ignore]` and was not run.
- RustChunk relevance: Not required for the Minecraft 1.20.1 core. Pinning asset revisions, verifying assets before use, and making optional-asset downloads non-fatal could be useful for Android launcher/skin/control assets.
- 1.20.1 implications: Bedrock assets are proprietary to that edition and are not part of the vanilla 1.20.1 protocol or resources. The RustChunk base should use Minecraft 1.20.1 textures/resource-pack assets, with any additional assets optional and subordinate to resource-pack precedence. Do not copy Mojang sample assets into the APK or repository without reviewing the license and confirming the actual need.
- Decision: **Unnecessary for the 1.20.1 core; pinned revisions, hash verification, and optional asset fallback are adaptable patterns, but the Bedrock touch-sprite pack itself is not directly portable.**
- Dependencies / cross-references: embedded assets/bedrock-touch.json; ureq HTTP client; SHA-1 Git blob hashing; PNG encoder/decoder; horizon_client::hud decode helpers and touch_sprite_names; resource packs and Starling texture namespace.
- Follow-up checks: Review manifest content/revision/license and asset-distribution policy. If adapted, add size/count limits, redirect/TLS policy, and validation that marker contents match the actual files.
- RustChunk implementation mapping: No use of Bedrock sprite assets in the 1.20.1 base. Only the idea of pinned, verified, optional external assets may inform a future RustChunk-owned control-art pipeline.
- Evidence required before adoption: Verify all included art is original or properly licensed; enforce revision/hash/size/redirect checks if fetching any asset. Confirm no Bedrock/Mojang sample files are bundled without permission.
- Completion state: complete source review; no network fetch or ignored integration test was run.

### 97. `crates/horizon-client/src/osk.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: b494059445a3b6ae5db6afcd1e159d6cf226899d.
- Size: 417 lines; complete source fetched in consecutive ranges 1–400 and 401–417.
- Coverage: lines 1–417, complete.
- Purpose: Horizon virtual keyboard for controller-based focus navigation, with simple output for typing into the active text field.
- Implementation / logic: Two fixed layouts, LETTERS and SYMBOLS, each have five rows and a total width of 10 key units. Keyboard state includes row/column, symbol mode, one-shot shift, and caps lock. The D-pad wraps horizontally and, when moving vertically, selects the key with the nearest horizontal center. Pressing Char, Space, Backspace, or Done produces Output::Text/Backspace/Enter; Shift and Layer only change keyboard state. Gamepad mappings are South=press, West=backspace, North=space, Start=enter, East=close, LeftStick/LeftTrigger=shift, and bumpers=symbol layer. Rendering uses its own UI theme and draws focused/lit keys and controller glyph-style hints.
- Findings: This is a QWERTY virtual keyboard specifically for controller navigation, not an Android IME. Its letter layout is fixed and Latin-based, with only a few accented characters (é/è/à/ç/ù/ü/ö/ä/ß/ñ) in the symbol layer. This file has no input-method, composition, prediction, locale-layout, Unicode IME-event, or clipboard integration. The Output API leaves it to the caller to apply text to the focused field; validation and field-length limits are outside this module. Shift is one-shot; pressing Shift twice enables caps lock, and a third press unlocks it. Tests cover row widths, navigation wrapping, shift/caps, symbols, and output-button mapping; they were not run.
- RustChunk relevance: A useful fallback if gamepad navigation is added to the launcher or in-game UI, but it should not replace the native Android keyboard on touchscreen phones.
- 1.20.1 implications: The Minecraft protocol does not depend on this UI. RustChunk 1.20.1 should obtain typed text from Android IME/`commitText` and composing-text events; this virtual keyboard is only an optional controller fallback after a text field gains focus. Its output should feed the 1.20.1 TextField/Sign/Book/Chat behavior, not a raw chat packet directly.
- Decision: **Unnecessary for the 1.20.1 gameplay core; an optional gamepad OSK fallback is adaptable, while Android IME should remain the primary text-input path.**
- Dependencies / cross-references: gamepad::{Button,GlyphStyle}; ui_theme::{Ui,Face,color}; language keys horizon.osk.* and gui.done; focused UI text field caller and gamepad focus navigation.
- Follow-up checks: Inspect callers that apply Output to the focused field. Separately audit Android native IME composition/cursor/selection/RTL and the input connection; if ported, review locale layouts and codepoint/UTF-16 length handling.
- RustChunk implementation mapping: No first-release game mapping. Android IME should be the primary text-entry path; a controller-specific on-screen keyboard is optional future UI.
- Evidence required before adoption: Test native IME composition, commit/delete, cursor/selection, multiline and Persian/RTL text in the app and game text fields; verify no duplicate or lost input on resume.
- Completion state: complete source review; no tests/build or Android IME session was run.

### 98. `crates/horizon-client/src/android_control_keyboard.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 6c8ae28734715a89fd3af69c5f77a2c7afb579ac.
- Size: 451 lines; complete source fetched in consecutive ranges 1–400 and 401–451.
- Coverage: lines 1–451, complete.
- Purpose: Android control-editor variant of the controller on-screen keyboard, with touch hit-testing and hardware KeyCode output for editors that accept key bindings.
- Implementation / logic: The file header states that the code is an MIT adaptation from Horizon Android and that `THIRD_PARTY_NOTICES.md` and `LICENSE` must be reviewed. The fixed LETTERS/SYMBOLS layout resembles the OSK and has five rows of 10 units. Keyboard state tracks focus row/column, shift, caps, and symbol mode. D-pad movement wraps; presses produce characters, Backspace, Enter, and mode changes. Unlike `osk.rs`, Output includes `Key(KeyCode)` in addition to Text/Backspace/Enter/Close. Controller mappings include South/West/North/Start/East, shift, layer, and D-pad. `touch(width,height,at_top,p)` hit-tests key cells using the same draw geometry and fixed 554×268 dimensions, then performs the press. Rendering uses `android_control_theme` and displays controller-button hints.
- Findings: This file largely duplicates the keyboard state and navigation in `osk.rs`, but adds touch hit-testing and KeyCode output for the Android control-configuration editor. Touch geometry depends on fixed drawing constants; changes to UNIT/GAP/KEY_H or layout dimensions must be reflected in hit-testing at the same time. The keyboard is fixed QWERTY/Latin and has no IME/native composition or locale-specific input. `Output::Key(KeyCode)` provides only hardware-key identity and does not determine character text; the caller must interpret it in the correct editor context. The header explicitly attributes the adaptation to Horizon Android/MIT; any code transfer must comply with LICENSE and THIRD_PARTY_NOTICES. Tests cover touch rectangles, row width, navigation, shift/caps, symbol mode, and output mapping; they were not run.
- RustChunk relevance: Relevant to the Android control/layout editor and keybind remapping, not core gameplay. It could provide a fallback for selecting physical/virtual keys in an editor; Android IME remains more suitable for free text entry.
- 1.20.1 implications: This layer is protocol-agnostic and does not change the 1.20.1 packet stack. RustChunk may keep a simple keyboard overlay for the touch-control editor, but it must not replace the system IME or locale-aware text input. KeyCode mappings must match the winit/platform adapter and 1.20.1 KeyMapping names.
- Decision: **Optional for the Android control editor; touch hit-testing/output design is adaptable, but the keyboard layout is duplicated and Latin-only. Attribution/license compliance is required.**
- Dependencies / cross-references: winit::keyboard::KeyCode; gamepad::{Button,GlyphStyle}; android_control_theme::{Ui,Face,color}; Android control editor text/key binding fields; LICENSE and THIRD_PARTY_NOTICES.md.
- Follow-up checks: Inspect consumers of Output::Key and Text; unify duplicated OSK logic in the RustChunk design; validate geometry values against actual touch-event scaling; review license attribution before copying.
- RustChunk implementation mapping: Optional future control-layout editor only; a duplicate Latin-only virtual keyboard is not required for the native engine and must not replace Android IME.
- Evidence required before adoption: Before adoption, unify with the primary text editor, test touch hit targets/scaling and physical-key selection, and validate Unicode/RTL text paths plus license provenance.
- Completion state: complete source review; no tests/build or Android editor interaction was run.

### 99. `crates/horizon-client/src/android_control_text.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 47fdcbe32fa87df752346b5bb7dc3cb42306edd1.
- Size: 79 lines; complete file fetched.
- Coverage: lines 1–79, complete.
- Purpose: Shortens catalog descriptions to fit a specified UI width, accounting for ellipsis and kerning.
- Implementation / logic: `fit` traverses `char_indices` in one pass and computes `advance(previous,current)` for the text glyphs. At each position it precomputes the ellipsis width using kerning from the last glyph; it records the last UTF-8 byte index where `prefix + "..."` fits within the requested width. If the whole text fits, it returns it unchanged; otherwise it returns the substring up to the best character boundary plus an ellipsis, or an empty string. Complexity is linear, and the `advance` callback is invoked exactly four times per character.
- Findings: The function uses byte indices from `char_indices`, so slicing is safe at UTF-8 boundaries; tests cover mixed Latin/Cyrillic/CJK/emoji text. Ellipsis width is computed using the final actual prefix character so kerning is respected. With negative/very small width and no prefix that fits with an ellipsis, the result is empty. This is a generic text-fitting helper only; it does not perform complex shaping, ligatures, bidi, grapheme-cluster handling, or line wrapping, and relies on the `advance` callback for actual glyph metrics. The header attributes an MIT adaptation from Horizon Android; attribution and notice must be retained. Tests compare it with the old quadratic algorithm and cover Unicode/kerning and callback counts; they were not run.
- RustChunk relevance: Useful for fitting UI labels, option descriptions, and resource names into limited mobile widths; the algorithm is simple and independent of the Minecraft protocol.
- 1.20.1 implications: This has no packet or gameplay dependency. Use RustChunk's own font metrics/rendering backend for `advance`; for right-to-left text, shaping, or grapheme-level truncation, use a real text-layout engine rather than this helper.
- Decision: **The linear helper is adaptable for simple labels; it is not a replacement for a text-layout engine for rich/RTL text or shaping. MIT attribution is required.**
- Dependencies / cross-references: font advance callback; UI catalog description rendering; UTF-8 Rust str slicing; THIRD_PARTY_NOTICES.md and LICENSE.
- Follow-up checks: Inspect the caller/source font metrics; in RustChunk, ensure truncation occurs at grapheme boundaries for combined emoji and RTL text.
- RustChunk implementation mapping: Maps only to simple label truncation in future mobile settings/control UI. It is not a substitute for RustChunk's text shaping, bidi or grapheme-aware layout.
- Evidence required before adoption: Test measurement against the actual font backend and grapheme clusters, emoji and Persian/RTL strings; ensure truncation never splits a composed character or corrupts shaping.
- Completion state: complete source review; no tests or build were run.

### 100. `crates/horizon-client/src/net.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 9406989b095c1c4b42f480aada781e5833a0a807.
- Size: 1,766 lines; complete source fetched in five consecutive ranges: 1–400, 401–800, 801–1,200, 1,201–1,600 and 1,601–1,766.
- Coverage: lines 1–1,766, complete.
- Purpose: Main multiplayer-session layer: connection thread, handshake/login, RSA/AES encryption, compression, configuration/registry synchronization, play-packet forwarding, keep-alive, ping/status, transfer/cookies, and disconnect-reason classification.
- Implementation / logic: `Session` starts a dedicated network thread and sends events to the game thread through a channel. A Mutex-protected packet writer remains available to the game thread, while the Configuration state prevents play packets from being sent during reconfiguration. Connections can use a TCP server, an integrated-server memory Pipe, a LAN-selected endpoint, a friend-invite tunnel, or a transfer after server handoff. The client first sends a handshake with the protocol version and LOGIN/TRANSFER intent, then Hello with the profile UUID/name. If the server requests encryption, the RSA public key is parsed from DER, a shared secret is generated, the online-mode session hash is sent to the session service, and the secret/challenge are then RSA-encrypted; AES encryption begins after the response. The compression threshold is applied during login. After LoginFinished, LoginAcknowledged is sent and Configuration begins: ClientInformation, `minecraft:brand`, known-pack response, registries/tags/features, resource packs, code-of-conduct, dialogs, server links, and cookies are processed, followed by FinishConfiguration and a Joined event. During Play, KeepAlive is answered immediately on the network thread, but Ping is sent to the game thread so Pong does not overtake prior updates/teleport acknowledgements. ConnectionStartConfiguration emits a reconfiguration event, sends an acknowledgement, and gates play traffic. A disconnect event includes the reason text/key and a `from_server` flag for auto-reconnect decisions. Status ping and pre-1.7 legacy ping are also provided.
- Findings: The file is tightly coupled to `PROTOCOL_VERSION` and Horizon protocol 26.3 packet schemas; status/configuration/play/login packet IDs and registry models cannot be retained for 1.20.1. Friend relay/tunnel, Horizon custom content identity, mod_list, quiet commands, mod_net, the far-LOD channel, dialogs/code-of-conduct, and transfer/cookies are modern or Horizon-specific and must be omitted from the vanilla base or deferred to a separate phase. The most valuable reusable architecture is the network thread + typed Event channel + single writer lock + game-thread event consumption. `Session::start` discards thread-spawn errors with `.ok()`; if the thread is not created, Session returns without reporting a disconnect and without an active connection. The Event channel uses unbounded `mpsc::channel`; if game-thread polling stops or packets flood in, the queue can grow. `send` drops packets while configuring; real lifecycle call sites must guarantee that required packets are not lost during this interval. Play packet-decode errors become `DecodeError` and the loop continues; for 1.20.1, only genuinely skippable packets should be ignored, while malformed critical packets should disconnect. The fixed read timeout is 30 seconds, explained in relation to keep-alive cadence; connect timeout is 10 seconds. TCP host resolution/SRV is done in the Connection layer, except for friend relay where the endpoint is already concrete. `Reason::component/tag` extracts a translation key separately from the text so reconnect classification is language-independent. Round-trip ping is measured only after a Status response; `ping_legacy` uses the legacy 0xFE/MC|PingHost packet and reads the UTF-16 response length as a u16. `json_text` and component flattening produce plain text, not rich chat rendering. The file contains many tests for status/login/keepalive/configuration/transfer/cookies/reconnect/auth/drop behavior; none were run.
- RustChunk relevance: This is the most important networking-architecture reference for a real client. Thread ownership, separation of network I/O from the game thread, typed events, serialized connection writing, immediate keep-alive, ping ordering, compression/encryption, and disconnect classification are essential to a stable multiplayer client.
- 1.20.1 implications: Rewrite the state machine and packet codecs entirely for Minecraft Java 1.20.1. **Protocol 763 has no separate Configuration connection state in 1.20.1; after Login it enters Play directly.** Registry/dimension data uses the older Login/Join Game structures and Play messages from that version, so do not import the Configuration state machine or packets introduced in 1.20.2 and later. Extract packet IDs, Login Start, Encryption Request/Response, Plugin Request/Response, Set Compression, Login Success, Join Game, Resource Pack, KeepAlive/Ping, Player Info, and Respawn/Dimension exactly from 1.20.1. 1.20.1 predates the newer Data Components system and has a different ItemStack/NBT model. First implement vanilla online/offline Handshake/Status/Login/Play without a Configuration state, friend relay, Horizon channels, or content mods; Forge remains deferred. Online-mode login requires a valid session service; an offline UUID is only for servers configured for offline mode and does not create Microsoft authentication or entitlement. On Android, verify socket read/write timeouts, cancellation, DNS/SRV, background lifecycle, and thread shutdown on real ARM64 hardware.
- Decision: **Thread/event/writer architecture and the high-level state-machine structure are adaptable; all wire packet IDs/schemas and 26.3 registry/auth extensions must be rewritten for 1.20.1.**
- Dependencies / cross-references: horizon_protocol connection/packets/crypto/rsa/registry/text/servers; horizon_auth AuthSession/session join; horizon_friends relay; content_identity and mod_list custom channels; lod_net; reconnect reason classifier; client event polling and game-thread packet handling; resource-pack/dialog/code-of-conduct UI.
- Follow-up checks: Audit `AuthSession`/`join_server` and DNS/SRV/Connection connect methods; extract exact 1.20.1 handshake/login encryption/compression/keep-alive/registry/resource-pack packet vectors (without assuming a Configuration state); fix spawn-failure handling and use a bounded event queue; define fatal-versus-skippable packet-decode policy; test reconfiguration races, Android disconnect/cancellation, and thread shutdown.


## 100-file checkpoint review — 2026-11-18

- Re-fetched all 100 source paths from `starling-client/starling` at ref `starling`. Every source SHA matched the SHA recorded in its retained audit entry.
- Rechecked recorded source line counts and coverage metadata. Corrected the inaccurate line counts for entries 4, 5, 7, 8, 41, 46, 47, 51, 54 and 61–80; corrected the final chunk ranges for entries 79 and 80.
- The previous ledger contained a duplicate full 1–100 set and duplicate short records for files 9–20. This checkpoint keeps one canonical entry for each file number 1–100 and removes the redundant copies and the now-obsolete pending/integrity notes.
- Rechecked key claims against source for `artifacts.rs`, `play.rs`, `item.rs`, `touch.rs` and `net.rs`. In particular, the networking entry now explicitly records that 1.20.1 has no separate Configuration state.
- Source hash/coverage verification is not a substitute for executed tests. No build, automated test suite, Android device run, or live Minecraft server connection was performed during this checkpoint.

### Documentation re-review — 2026-11-18

- Re-reviewed all 100 ledger entries as one document and standardized their headings and 13 common fields. Existing file-specific subsections and evidence were retained.
- Where the earlier entry used custom subsections instead of consolidated `Implementation / logic` or `Findings` fields, the standardized fields point back to those existing subsections; no unsupported implementation detail was added.
- Translated all Persian-script content in entries 61–100 into English. A full-file scan found no remaining Persian-script characters.
- Classified every Forge/NeoForge/FML mention in entries 1–40 as a historical reference or current project/file-level conclusion. The original evidence was retained.
- Created `AUDIT_INDEX.md` with 100 numbered rows, split into four parts of 25 entries each. Each row records the source path, a normalized decision label, and a one-line summary derived from the ledger's Decision field.
- Final structural checks: exactly one entry for each number 1–100; all 100 have source SHA and all 13 common fields; all headings use `### N. \`path\``; no duplicate `##` headings; no Persian-script text; all 100 index rows are in sequence and partitioned 25/25/25/25.
- Revalidated all 100 source paths and blob SHAs against the complete pinned Starling Git tree at `5432370d573642041585c69b40fb045b27133488`; no missing paths or SHA mismatches were found.
- Verification boundary: this pass revalidated the ledger and source identities, but did not repeat a fresh line-by-line semantic read of all 100 source files. Existing semantic notes remain the findings from their recorded original source audits. No tests, build, Android device run, or live-server run was performed.
- RustChunk implementation mapping: Maps to a future protocol-763 multiplayer session/state machine after the local single-player gate. Current `engine/src/lib.rs` JNI functions/downloaders are not a Minecraft network client.
- Evidence required before adoption: Require end-to-end vanilla 1.20.1 server tests for handshake, login/encryption, Login-to-Play, keepalive, packets, disconnect and reconnect; confirm no later-only Configuration state or custom Horizon extensions.
- Completion state: complete source review; no tests, build, live server connection, online authentication or Android network run was executed.

### 101. `crates/horizon-client-core/Cargo.toml`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 8809a5216455ace851044f69a633e8adaabbc576.
- Size: 15 lines; complete file fetched and read.
- Coverage: lines 1–15, complete; no chunks omitted.
- Purpose: Cargo manifest for the lowest-level client-core crate, which owns client text, NBT/SNBT, translations, scoreboard, boss bars, game rules, identifiers, registries, particle/sound tables, and related data utilities.
- Implementation / logic: Declares package horizon-client-core, version 0.1.0, workspace edition/license, and Rust 1.90 minimum. Its only direct dependencies are the sibling horizon-world crate and serde_json with raw_value support. Workspace lints are inherited. The crate boundary and public module surface are defined separately in src/lib.rs (reviewed as entry 114).
- Findings: This is intentionally a very small dependency layer: it does not directly depend on winit, wgpu, a Java runtime, network transport, or Forge. Its reliance on horizon-world means the supposedly low-level client types still inherit the world crate's NBT, registry and version-pinning decisions. The manifest does not state which game version the data APIs represent; that is determined by the implementation and generated resources, not by the package version 0.1.0.
- RustChunk relevance: High as a dependency-layer pattern. Keeping text/components, identifiers, NBT paths, game-rule and registry-facing client data below rendering and gameplay can reduce cyclic dependencies and let the Android input/render stack depend on a small core.
- 1.20.1 implications: Package structure and serde_json are reusable, but the actual module APIs, registries, game rules, text serialization and NBT assumptions must be generated or implemented from Minecraft Java 1.20.1. Do not infer 1.20.1 compatibility from the crate name or its minimal dependency list.
- Decision: Adaptable as a minimal client-core crate boundary; version-specific module implementations must be validated or rewritten for 1.20.1.
- Dependencies / cross-references: horizon-world; serde_json; src/lib.rs; horizon-client-lib; generated resource/registry data; workspace Cargo.toml and lockfile.
- Follow-up checks: Inspect the modules re-exported by src/lib.rs and trace their callers; identify every DATA_VERSION-bound table and compare its source schema with 1.20.1; verify that the core remains free of renderer/gameplay dependencies.
- RustChunk implementation mapping: Maps to a future minimal native client-core crate for shared text/NBT/identifiers/registries and version-neutral utilities. The current workspace does not yet contain this crate.
- Evidence required before adoption: Use `cargo metadata` to confirm dependency direction and no renderer/platform cycles; test the versioned data tables against 1.20.1 oracles and compile the crate independently.
- Completion state: Complete manifest review; no Cargo resolution, build, test, or runtime execution was performed.

### 102. `crates/horizon-client-input/Cargo.toml`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 4ac6be76c312a1ef612a207c73db5236c898a633.
- Size: 27 lines; complete file fetched and read.
- Coverage: lines 1–27, complete; no chunks omitted.
- Purpose: Cargo manifest for the client input crate: key bindings/names, mouse, controllers, clipboard and window/input options.
- Implementation / logic: Declares horizon-client-core as its local core dependency, winit 0.30.13, horizon-theme and PNG for the window icon, plus optional horizon-dualsense behind the controllers feature. Desktop-only clipboard integration uses arboard under cfg(not(target_os = android)); the manifest comment says Android uses the platform clipboard through clipboard.rs. The controllers feature enables only the optional DualSense crate at this layer.
- Findings: The separation gives platform input a distinct crate, but winit and the platform conditionals are still significant implementation choices. There is no single Android controller backend dependency in this manifest; Android-specific event translation is implemented in child source modules. The optional feature controls DualSense effects here, while the higher-level client manifest also controls gilrs/controller discovery. A build with inconsistent feature forwarding between the client and input crates could therefore produce partial controller support. This file contains no tests.
- RustChunk relevance: High. A dedicated input crate can normalize touchscreen, hardware keyboard, mouse and controller sources into action-oriented state without making gameplay depend directly on Android UI or desktop windowing.
- 1.20.1 implications: Input abstractions are largely protocol-independent, but key names/default bindings, repeat/toggle semantics and logical action mappings must be extracted from 1.20.1. The Android clipboard, touch and device event paths need target-API validation; winit 0.30.13 should not be copied without checking the chosen RustChunk platform stack.
- Decision: Adaptable as a platform-input boundary; key tables and backends require target-specific validation, and controller support should remain optional.
- Dependencies / cross-references: horizon-client-core; winit; horizon-theme; png; optional horizon-dualsense; arboard on non-Android; src/lib.rs; gamepad/android.rs; clipboard.rs; keybinds.rs.
- Follow-up checks: Trace feature forwarding from horizon-client and horizon-client-lib; inspect the Android input bridge and clipboard implementation; test focus loss, pointer cancellation, controller disconnect and physical-versus-virtual keyboard classification on real devices.
- RustChunk implementation mapping: Maps to a future platform-neutral input crate, not the current Android Activity. Keep controller support feature-gated and optional.
- Evidence required before adoption: Verify feature forwarding and target-specific dependency trees; run host and `aarch64-linux-android` builds, and exercise keyboard/touch lifecycle tests before adding optional controller dependencies.
- Completion state: Complete manifest review; no dependency resolution, build, device input test, or Cargo test was executed.

### 103. `crates/horizon-client-lib/Cargo.toml`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 9f290d8f6b156871e5f599801aebdd2c60d87f53.
- Size: 232 lines; complete file fetched and read.
- Coverage: lines 1–232, complete; no chunks omitted.
- Purpose: Manifest for horizon-client-lib, the reusable client systems library, control tool, and parity-probe host. It deliberately points its library target at ../horizon-client/src/client_lib.rs rather than a conventional src/lib.rs.
- Implementation / logic: Disables automatic bins, examples and tests, then explicitly declares the library path, horizon-ctl binary, numerous benchmarks/probes/examples and integration-test targets. The dependency graph spans client-core/input/render, world, protocol, assets, items, commands, threads, shaders, HDR, theme, MCP, server files, server anticheat/chat filter, friends, narrator, telemetry, account sign-in, preparation and mod-index infrastructure. Wgpu is configured with Vulkan/Metal/WGSL and naga IR; platform-specific Windows DX12 and non-macOS GLES variants are added. Default features enable audio and controllers; audio forwards to render, and controllers forward to gilrs, DualSense and input. A separate rng-trace feature propagates into horizon-core.
- Findings: The package is a broad integration layer, not a small pure client library. It includes dependencies whose responsibilities also serve a dedicated server or launcher (server files, anticheat, chat filtering, preparation and MCP), which can enlarge the graph and make feature auditing important. Some native platform dependencies are conditionally compiled, but actual APK inclusion cannot be inferred from this manifest alone. The unusual library path and disabled Cargo auto-discovery mean that adding files under examples/tests does not automatically register them; the explicit target list must be maintained. Default features pull audio and controller stacks unless consumers opt out. The manifest's wgpu version and shader capabilities are generation-specific. This is a dependency declaration, not evidence that every listed subsystem is active at runtime.
- RustChunk relevance: High as a map of the integration layer and as a warning about dependency growth. RustChunk should decide deliberately whether to separate a reusable client library, native Android entry-point crate, diagnostics/control CLI, and parity tools instead of compiling all of them into every target.
- 1.20.1 implications: The overall layering can be reused, but the large dependency graph and all protocol/game-facing consumers need a 1.20.1 compatibility pass. Renderer backend features must match Android's GLES/Vulkan capability and the target Mali device; desktop DX12/Metal features should not leak into the Android artifact. Account/session and server utilities must obey the selected 1.20.1 online/offline authentication design. None of the Horizon version 26.3 dependencies establishes Forge support.
- Decision: Adaptable as an integration/workspace pattern; major dependency, feature and game-version pruning is required before using it as RustChunk's 1.20.1 client library.
- Dependencies / cross-references: All local crates listed in this manifest, especially horizon-client-core/input/render, horizon-world, horizon-protocol, horizon-items, horizon-prepare, horizon-server-files, horizon-mods, horizon-mcp and horizon-threads; client_lib.rs; horizon-client/Cargo.toml; workspace Cargo.toml.
- Follow-up checks: Generate Android-specific Cargo feature trees for default and no-default-features builds; determine which server-only crates are linked into the APK; inspect client_lib.rs module wiring and every manually registered target; audit wgpu backend selection, JNI/native dependencies, minimum Rust toolchain and binary size.
- RustChunk implementation mapping: Maps to a future application-facing client library composing core, world, input and renderer; current `rustchunk-engine` is an installer/JNI package and is not equivalent.
- Evidence required before adoption: Require a minimal acyclic crate graph, Android feature-tree inspection and build tests showing only needed game subsystems are linked into the target APK.
- Completion state: Complete manifest review; no cargo metadata/tree, dependency resolution, APK build, test, benchmark or runtime link inspection was executed.

### 104. `crates/horizon-client-render/Cargo.toml`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: d340024b704df413438f51c7d70fe36ce5e7f328.
- Size: 82 lines; complete file fetched and read.
- Coverage: lines 1–82, complete; no chunks omitted.
- Purpose: Manifest for the client world-data/rendering/screens layer, described as chunks, meshing, light, streaming, renderer, HUD, menus and settings, with gameplay kept above it.
- Implementation / logic: Depends on client-core/input, world, protocol, assets, commands, items, shaders, theme, threads, HDR, cosmetics and related UI/render crates. Uses wgpu 30.0.1, winit 0.30.13, naga with WGSL/GLSL/HLSL output, glam, bytemuck, fonts, ZIP/JSON, image and compression libraries. Audio is optional through cpal and the audio feature; test-support is a separate feature. Windows adds DX12 and junction support; non-macOS targets add GLES; Unix adds rustix.
- Findings: The manifest is broad for a rendering crate: it includes world, protocol, item, command and telemetry dependencies, so the package boundary is not equivalent to a standalone graphics backend. The declared backends imply Vulkan/Metal as primary options, Windows DX12, and GLES on non-macOS targets; the exact backend selected at runtime is implemented elsewhere. The feature graph needs to stay aligned with horizon-client-lib and horizon-client, especially audio and test-support. Package-level description says gameplay does not reach into this crate; that layering claim should be checked through actual imports rather than assumed from Cargo metadata. This file contains no tests.
- RustChunk relevance: High for separating renderer, world presentation and screens from simulation/gameplay. It also exposes the need to measure renderer-backend features and optional audio on Android rather than copying desktop defaults.
- 1.20.1 implications: Rendering infrastructure such as wgpu, font rasterization, asset loading and UI layering is version-independent in principle. However, block models, chunk meshing inputs, lighting, resource formats, registry data, protocol-derived world state and screens are tied to the target version and must be reimplemented or validated against 1.20.1. GLES/Vulkan availability and memory use must be checked on the Android target GPU.
- Decision: Adaptable as a render/UI crate boundary; version-specific world, mesh, lighting, asset and packet consumers must be ported, and Android backend selection must be verified.
- Dependencies / cross-references: horizon-client-core/input; horizon-world; horizon-protocol; horizon-assets; horizon-items; horizon-shaders/shader-api; horizon-command; wgpu; winit; crates/horizon-client-render/src/lib.rs; crates/horizon-client-render/src/chunk.rs; horizon-client-lib.
- Follow-up checks: Trace imports to verify the no-gameplay dependency rule; inspect renderer startup and adapter selection; validate GLES/Vulkan on Android; inspect audio feature forwarding and pack-overlay filesystem behavior; compare all world/render facts and chunk formats with 1.20.1.
- RustChunk implementation mapping: Maps to a future renderer/UI crate. The architecture map proposes preserving Starling's wgpu/chunk pipeline while adapting all 1.20.1-facing mesh/light/model data; RustChunk has no independent renderer crate yet.
- Evidence required before adoption: Require a correct 1.20.1 block/model/biome/light input contract, headless render-data tests, Vulkan/GLES device validation and Android frame/RAM measurements before adoption.
- Completion state: Complete manifest review; no Cargo build, shader compilation, renderer startup, Android GPU test or performance benchmark was run.

### 105. `crates/horizon-client/Cargo.toml`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 1d575779193dbc9aa41adad2e08d243097bf1409.
- Size: 163 lines; complete file fetched and read.
- Coverage: lines 1–163, complete; no chunks omitted.
- Purpose: Manifest for the native Horizon game executable and the gameplay/startup crate; it registers the horizon-client binary and the client integration tests.
- Implementation / logic: Declares horizon-client-lib with default features disabled, and separately includes client-core/input/render plus assets, items, world, protocol, server/shared services, shaders, mods, telemetry, narrator and related crates. Android adds winit's android-native-activity feature, JNI 0.21.1 and starling-catalog. Windows adds URI handling, DX12 and winresource build support; macOS adds objc2 AppKit/Foundation; non-macOS targets add GLES. Default features enable audio/controllers and forward those into client-lib/input/render; optional allocator, RNG-trace and Windows heap features are declared. The binary path is src/main.rs; three explicit integration tests are registered.
- Findings: This crate is the native executable/platform-entry boundary, while much reusable functionality is delegated to horizon-client-lib. Dependencies are repeated across the executable and library manifests, so feature alignment and duplicate target-conditional declarations need to be maintained. Android is implemented as a NativeActivity/winit + JNI path, not a conventional Java Activity wrapper in this Cargo manifest. The manifest declares many Horizon server/mod/telemetry features as dependencies even though feature flags may disable some code paths; only a target-specific dependency/build graph can prove what lands in an APK. The minimum Rust version comes from the workspace and is not independently pinned here. No Java Forge/FML runtime is declared.
- RustChunk relevance: Very high as the main executable manifest and the clearest place to inspect Android target composition. It informs how to isolate the launcher shell, Android activity/JNI glue, reusable client runtime and optional features.
- 1.20.1 implications: The native entry and Android activity approach are adaptable, but startup/gameplay implementations and all protocol/world/render dependencies must target 1.20.1. RustChunk should avoid linking Horizon-only mods, catalogs, server facilities or 26.3 assets into the vanilla base unless each is intentionally retained. The declared GLES/Vulkan/DX12/Metal feature layout must be reduced and verified for Android.
- Decision: Adaptable as the native executable and Android platform boundary; the dependency graph, features and game-facing implementation require substantial 1.20.1-specific pruning and rewrite.
- Dependencies / cross-references: horizon-client-lib; client-core/input/render; horizon-world/protocol; JNI; winit Android NativeActivity; starling-catalog; src/lib.rs; src/main.rs; Android android.rs and entry.rs; workspace Cargo configuration.
- Follow-up checks: Inspect build.rs, Android packaging and Gradle/NDK integration; trace which optional features actually compile into the APK; verify how JNI/native library naming and activity lifecycle are wired; check the APK's runtime/API/ABI requirements and the separation between game and launcher.
- RustChunk implementation mapping: Maps to the future native game executable/platform composition crate. Do not treat current `engine/Cargo.toml` or JNI launch-plan exports as the gameplay executable.
- Evidence required before adoption: Require a native Rust build and playable single-player 1.20.1 vertical slice; audit feature/link outputs to prove the build does not route through Java/JVM for game simulation.
- Completion state: Complete manifest review; no Cargo resolution, binary build, Android APK, JNI launch or integration test was performed.

### 106. `crates/horizon-world/Cargo.toml`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 0eb0442297cdb05372808e97a8a9e287ae3cd7ba.
- Size: 36 lines; complete file fetched and read.
- Coverage: lines 1–36, complete; no chunks omitted.
- Purpose: Manifest for horizon-world, the vanilla world-data layer covering NBT, regions, chunk storage, registries and world-generation parity utilities.
- Implementation / logic: Declares library dependencies on libm pinned to 0.2.16, log, serde/serde_json, SHA-2, postcard 1.1.3, horizon-core, flate2 with zlib-rs/runtime detection, lz4_flex with checked safe codecs, and xxhash-rust with xxh32. Registers horizon-world and horizon-worldgen-parity binaries plus the world_probe example whose source is reviewed as entry 120. Workspace lints are inherited.
- Findings: The manifest avoids direct graphics/windowing dependencies and is a clean data/world layer in the dependency graph. The serialization/compression primitives are broadly reusable, but actual DataVersion, chunk section, registry, tick and worldgen schemas are fixed by source constants and generated data elsewhere. The package includes production binaries and a large parity probe target, so a library-only consumer must be deliberate about Cargo target selection. No tests are declared explicitly here; tests may still be embedded in module source files.
- RustChunk relevance: Very high. World/NBT/region responsibilities should be separate from renderer and launcher code, with the source-of-truth data version made explicit.
- 1.20.1 implications: NBT and Anvil storage are structurally reusable, but all DataVersion checks, generated registries, block-state palettes, tick types, biome/worldgen schemas and data-fixing assumptions require a 1.20.1 implementation. The pinned library versions and native compression dependencies must be evaluated for Android toolchain support and binary size.
- Decision: Adaptable as a dedicated world-data crate; the versioned world formats and resource pipeline must be rebuilt or verified for 1.20.1.
- Dependencies / cross-references: horizon-core; NBT, compression, region, chunk and registry modules; DATA_VERSION; horizon-worldgen-parity; examples/probe/main.rs; cargo workspace and Android target configuration.
- Follow-up checks: Audit DATA_VERSION and world registry generation; inspect Android builds of zlib-rs/lz4 and xxhash; compare Anvil external chunk compression and 1.20.1 section/data-version schemas; decide whether probe binaries/examples are excluded from release builds.
- RustChunk implementation mapping: Maps to a future world-data/storage crate, with 1.20.1 NBT, Anvil, palette, registry and worldgen schemas. Current verified registry facts are only inputs to that crate.
- Evidence required before adoption: Require chunk/NBT/region round-trip tests, DataVersion 3465 validation, 1.20.1 registry/worldgen oracle fixtures and save/reopen tests on host before Android integration.
- Completion state: Complete manifest review; no Cargo build, compression test, world load/save, or Android target build was performed.

### 107. `packaging/android/java/io/github/juanluci/starling/NativeMods.java`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 30d7c46dc06572c0708a75ffa9227a8a86e26b05.
- Size: 183 lines; complete file fetched and read.
- Coverage: lines 1–183, complete; no chunks omitted.
- Purpose: Android Java bridge for selecting and managing downloaded Horizon native-mod builds, tracking first-frame startup failures, exposing mod-requested Android permissions, and selecting a remote native build server.
- Implementation / logic: Uses the app-private files/native directory. chosen() reads the current build key, rejects empty keys, slash-containing keys and keys beginning with a dot, verifies the library exists, checks the requested game version against the build's for file, and checks an APK-install stamp against the build's apk file. It then counts starts and moves the build aside after two starts without a first-frame confirmation. started() clears the counter. permissions() reads build.json and groups each mod ID by requested permission in insertion order; permissionsDeclined/declinePermissions store the user's choice. failed() sets the build aside with a notice. isBuildServer() checks an HTTPS host/optional-port regex; chooseBuildServer() stores or removes the configured address. Small metadata files use best-effort UTF-8 read/write helpers.
- Findings: The selected native library is tied to both game version and exact APK lastUpdateTime, which prevents loading a build made for another install even when versionName/versionCode match. The two-start guard provides a rollback path after native crashes before first frame. Permission requests are sourced from build.json, not independently derived in this Java layer; malformed JSON becomes an empty permission set. The permission names/mod IDs are not allowlisted here, so the Activity/requesting code must validate supported Android permission identifiers and explain the mapping to the user. Build-server validation is regex-based: it restricts to HTTPS host syntax but does not fully validate DNS labels, IPv6, port range or URL parsing. Metadata writes are non-atomic and silently ignore I/O errors; a lost/corrupt starts or notice file can affect rollback/UX. The key rejects slash and leading-dot forms but does not visibly validate a strict generated-key alphabet; build key creation and app-private path trust need to be checked. This file manages Horizon Rust native-mod libraries, not Java Forge/FML mod loading.
- RustChunk relevance: Low for the first vanilla 1.20.1 milestone, but relevant if RustChunk later adopts a native extension system or needs guarded loading of downloaded native libraries on Android. The install-stamp and startup rollback pattern is worth studying.
- 1.20.1 implications: None of these mod-build files is part of Minecraft's vanilla 1.20.1 client protocol. Do not carry native-mod selection or permission behavior into the vanilla base by default. If RustChunk adds native extensions, design a separate trust/signature/update policy, strict build-key validation, permission allowlist, atomic metadata writes and crash recovery. This mechanism is not a substitute for a Forge runtime.
- Decision: Unnecessary for the vanilla 1.20.1 base; the APK-stamp/startup-rollback pattern is adaptable later, while the current native-mod integration is Horizon-specific.
- Dependencies / cross-references: StarlingActivity; native_builds.rs; native_build.sh; native_mod_permissions.py; build.json; STARLING_APK; android manifest permissions; app-private files/native layout.
- Follow-up checks: Inspect StarlingActivity permission prompting and library loading order; validate permission strings against Android manifest/runtime permission mappings; inspect build-key generation and remote build-server parsing on the Rust side; review signature/integrity verification for downloaded .so files and atomic metadata recovery.
- RustChunk implementation mapping: No first-release mapping. Starling's Android NativeMods manager is native-extension plumbing, not a Forge mod loader; RustChunk's Forge launch-plan code is a separate legacy launcher path.
- Evidence required before adoption: Keep excluded from the vanilla native APK. Any later native extension feature requires a separate ABI/trust/permission/rollback design and security tests; it does not prove Forge JAR compatibility.
- Completion state: Complete source review; no Android build, native library load, permission prompt, build-server connection or device test was executed.

### 108. `packaging/android/native_mod_permissions.py`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: b043eccdf8fa7ec2ce0e1412dfc8c356180382e9.
- Size: 58 lines; complete file fetched and read.
- Coverage: lines 1–58, complete; no chunks omitted.
- Purpose: Build-time source scanner that guesses which Android permissions a native mod needs, then feeds the permission list into the build metadata consumed by NativeMods.java.
- Implementation / logic: Defines one permission rule, microphone, as a compiled regular expression matching source/API markers for CPAL input streams/devices, AAudio input direction, OpenSL ES recording and Android AudioRecord/MediaRecorder. Scans Rust, C/C++, header, Java and Kotlin files recursively under each supplied mod folder in sorted path order. Reads source as UTF-8 with replacement for invalid bytes, records matching permission names in a set and stops early when every configured permission has been found. The CLI prints each folder and its sorted permission list or a dash.
- Findings: This is a heuristic text search, not a compiler, call-graph or permission analysis. It can miss aliased/wrapped/dynamically invoked APIs and can report false positives from comments, dead code, vendored samples or unrelated source files. It only knows the microphone permission today; new permission types require additional patterns and manifest/runtime request wiring. It does not check whether the Android permission is declared in the APK manifest, whether a mod actually executes the code path, or whether the user granted the permission. File read errors are not handled explicitly beyond replacement decoding; no tests are included in this file.
- RustChunk relevance: Low for the vanilla core. Useful only if RustChunk later offers native plugin/mod compilation that must declare Android capabilities before installation or execution.
- 1.20.1 implications: This script is independent of Minecraft's 1.20.1 protocol and must not be mistaken for Java Forge mod permission discovery. If a future RustChunk native extension system is added, use explicit capability manifests or compiler-assisted analysis rather than relying solely on text grep.
- Decision: Unnecessary for the vanilla 1.20.1 base; the idea of predeclaring native-extension permissions is adaptable, but this scanner is heuristic and Horizon-specific.
- Dependencies / cross-references: NativeMods.java; native_build.sh; build.json; Android manifest permission declarations; native-mod source layout; cpal/AAudio/OpenSL ES/Android media APIs.
- Follow-up checks: Compare scanner patterns with actual native-mod APIs and Android manifest requests; test false-positive/false-negative cases; verify recursive scan boundaries and symlink behavior; decide whether a future RustChunk extension API should use an explicit permission declaration file.
- RustChunk implementation mapping: No first-release mapping. Permission scanning for Starling native modules is not Java Forge permission discovery and is irrelevant without an approved native extension ABI.
- Evidence required before adoption: Do not add to the vanilla build. If a native extension system is later designed, validate scanner accuracy with positive/negative fixtures, symlink/path boundaries and manifest/runtime permission tests.
- Completion state: Complete source review; no script execution, sample-mod scan, Android build or runtime permission request was performed.

### 109. `crates/horizon-client-render/src/chunk.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 923a2a0832b84ae93a03c431d1288cb70443400b.
- Size: 1,076 lines; complete file fetched in consecutive ranges 1–400, 401–600, 601–800 and 801–1,076.
- Coverage: lines 1–1,076, complete; the initially large middle response was re-fetched in smaller consecutive ranges to ensure no gap.
- Purpose: Resident client-render chunk/column data model, including packed block and biome palettes, shared sections, block/sky light, heightmaps, block entities, saved entities, structures and memory accounting.
- Implementation / logic: Defines block-palette width strategy and a process-wide configurable block bit width, shares uniform sections through Arc, and uses copy-on-write for mutations. A ColumnLayout describes min Y and section count. Section stores block and biome PalettedContainers plus a Light value that can be uniform or packed. Chunk stores coordinates, min_y, Arc sections, lazily initialized WORLD_SURFACE and MOTION_BLOCKING heightmaps, light/fresh/generated flags, block entities/names, post-processing positions, saved extras, entities, structure references and starts. Index helpers map local block/section/biome coordinates. set() mutates via Arc::make_mut and invalidates cached heightmaps only when a write could change a relevant column height; replace_blocks() rebuilds storage and resets both heightmap caches. The module also tracks unique/shared allocation sizes in ColumnCensus.
- Findings: The resident representation is optimized for memory and mutation sharing, not a serialized protocol model. Heightmap correctness depends on all section writes passing through the invalidating methods. The section_blocks() helper computes its index from global MIN_Y rather than the chunk's stored min_y; because Chunk supports arbitrary ColumnLayout, callers using non-default layouts need validation that this mapping is intentional. Light outside a column returns zero below min_y and full sky above max_y. Biome lookup clamps quart Y to the column. The global block-bit strategy is mutable process-wide and should not change while existing containers rely on it. Shared Arc sections reduce memory until writes trigger copy-on-write, so memory reports must distinguish shared and unique allocations. The module includes modern world-height/layout and registry assumptions; no tests were executed as part of this audit.
- RustChunk relevance: Very high for mobile memory use. Uniform-section sharing, packed palettes, Arc copy-on-write, cached heightmaps and explicit memory census are useful approaches for large render distances on constrained devices.
- 1.20.1 implications: The underlying chunk/section/palette concepts are reusable, but section count/min_y, biome container dimensions, light encoding, palette bit widths, heightmap predicates and block/biome IDs must match the actual 1.20.1 dimension and protocol rules. In particular, do not assume the same vertical layout for Overworld, Nether and End; validate each dimension's min_y and height. Chunk packet decoding and save NBT must be version-specific and remain separate from this resident cache.
- Decision: Highly adaptable as a memory-efficient resident chunk representation; global layout, section indexing, light/heightmap semantics and registry data require validation or rewrite for 1.20.1.
- Dependencies / cross-references: horizon-world palette/storage and registry types; ColumnLayout; Light; BlockStateId; horizon-client-render chunk map, mesher, lighting and streaming; MIN_Y; memory diagnostics.
- Follow-up checks: Trace every caller of section_blocks() and test custom min_y layouts; compare dimension section ranges and heightmap predicates with 1.20.1; test concurrent read/COW mutation rules; verify cache invalidation for every block mutation path; benchmark memory and lookup costs on Android ARM64.
- RustChunk implementation mapping: Maps to the future resident render-chunk representation and chunk-streaming worker interfaces described in `docs/STARLING_1_20_1_ENGINE_MAP.md`; no corresponding RustChunk runtime struct exists yet.
- Evidence required before adoption: Compare section range/min_y, palettes, biome containers, heightmaps, light and block-entity layouts with 1.20.1 Java; add memory/COW/concurrency tests and verify on target ARM64.
- Completion state: Complete source review; included source was read but tests, renderer build, memory benchmark, Android device run and gameplay chunk load were not executed.

### 110. `crates/horizon-world/src/chunk.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 9272a3460116dad42ac378cd9f4c76a390fdca4a.
- Size: 999 lines; complete file fetched in five consecutive ranges: 1–200, 201–400, 401–600, 601–800 and 801–999.
- Coverage: lines 1–999, complete; no chunks omitted.
- Purpose: World-storage chunk primitives: NBT palette containers, saved chunk metadata/ticks, block-state descriptors, and runtime packed BlockSection storage. The file explicitly says these operations do not execute block behavior, lighting, heightmaps, neighbor updates or scheduled ticks.
- Implementation / logic: PalettedContainer reads/writes palette and packed-long-array NBT, validates palette indexes, preserves unrelated extra compound fields and can compact storage. BlockState converts between names/properties and NBT. ChunkData validates chunk xPos/zPos, exact DATA_VERSION, section list and duplicate section Y, and reads/writes saved block/fluid ticks constrained to the chunk. BlockSection maps registry BlockStateIds to a compact palette with per-slot counts, a free list, last-lookup cache and PackedStorage; set() grows/reuses palette IDs, fill_run() writes vertical runs and counts added air-to-non-air transitions, and compact() rebuilds first-use palette order. Embedded tests compare optimized state operations with a baseline over randomized edit streams and cover palette growth/collapse, invalid data and freed-slot reuse.
- Findings: This is storage-level machinery, not a complete gameplay chunk or a data-fixer. ChunkData rejects any DataVersion other than the crate's exact constant and does not convert older chunk formats; 1.20.1 saves therefore cannot be assumed to load. Section Y is stored as an NBT Byte and conversion to i8 explicitly rejects values outside that range. Palette storage validates data lengths/indices, while unrelated NBT fields are retained for round-trip preservation. BlockSection state IDs are tied to the loaded BlockRegistry order. fill_run() optimizes a top-down contiguous write and updates counts; its returned air count is important to callers and must be cross-checked against the exact caller traversal. Tests are extensive but were read only, not run.
- RustChunk relevance: Very high for efficient chunk/world persistence. Palette compaction, packed storage, free-slot tracking and baseline-equivalence tests are useful implementation patterns, while the explicit storage-versus-simulation boundary should be preserved.
- 1.20.1 implications: Rebuild DATA_VERSION handling, chunk NBT field names, section Y/range, palette packing format, biome container and tick schemas from 1.20.1. Add a deliberate DataFixer/migration policy for old saves rather than simply relaxing the version equality check. Runtime BlockStateIds must come from the 1.20.1 registry snapshot and remain consistent with packets, generated resources and disk storage.
- Decision: Adaptable as storage algorithms and palette machinery; the existing data version, registry IDs and serialized chunk layout require a 1.20.1-specific rewrite/adapter.
- Dependencies / cross-references: horizon-world NBT/registry/compression/tick_data; horizon-core PackedStorage and position types; DATA_VERSION; client-render Chunk/Section; world save/load and region storage.
- Follow-up checks: Compare palette packing and section NBT against 1.20.1 vanilla ChunkSerializer; inspect DATA_VERSION and datafix requirements; verify fill_run callers and non-air counts; add round-trip fixtures from genuine 1.20.1 chunks; test missing/unknown block states and tick lists.
- RustChunk implementation mapping: Maps to the future persisted world/chunk model in the world-data crate, including 1.20.1 Anvil/NBT section/tick schemas; keep it separate from renderer-resident chunk caches.
- Evidence required before adoption: Require byte/semantic parity with 1.20.1 ChunkSerializer, palette packing and DataVersion 3465; test malformed data, migration policy and save/reopen round trips.
- Completion state: Complete source review; embedded tests were read but not run; no world save/load, datafix, build or benchmark was executed.

### 111. `crates/horizon-world/src/nbt.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: b27bf4fda2c50ec7b81adb316aa0d12359e2dc56.
- Size: 1,152 lines; complete file fetched in consecutive ranges 1–200, 201–400, 401–600, 601–800, 801–1,000 and 1,001–1,152.
- Coverage: lines 1–1,152, complete; an omitted portion of the first large response was fetched separately as lines 601–800.
- Purpose: Java Edition NBT and SNBT implementation for file and network payloads, with lossless UTF-16 strings, streaming list reads, resource-specific limits and version-generation mixed-list wrappers.
- Implementation / logic: JavaString stores UTF-16 code units, including unpaired surrogates. Tag models all standard NBT tag types and stores float/double bit patterns. Reader validates byte, node and depth limits; decodes Java modified UTF-8; handles named-file roots and unnamed network roots; keeps the last duplicate compound key; and can stream selected lists through a visitor rather than retaining their elements. Limits differ for defaults, large state tables, saved data and structure templates. Writer separates disk and network behavior: oversized modified-UTF strings become empty on disk but are rejected for network output; disk NaNs are canonicalized while network output preserves the bit pattern. SNBT parser handles compounds, lists, typed arrays, quoted/bare strings, booleans and suffixed numbers, with a depth cap of 512. Tests cover UTF behavior, mixed-list wrappers, duplicate keys, NaN handling, streaming, truncation, limits and SNBT.
- Findings: This is a broad and security-sensitive parser/writer. The mixed-list wrapper convention unwraps one-entry compounds whose key is empty; it is described as a 26.3 behavior and must not be treated as an unconditional property of every 1.20.1 NBT list, where normal NBT list elements remain homogeneous. The saved-data limits allow up to 1 GiB and unlimited nodes, trading memory protection for compatibility with large vanilla/modded saves; callers must still constrain decompressed input and allocate budgets deliberately. SNBT typed-array conversion uses a helper that maps nonnumeric tag variants to zero rather than producing an error; verify this against vanilla TagParser semantics before reusing it. The parser returns consumed byte length rather than requiring the entire input to be consumed, so callers must decide whether trailing text is permitted. Tests exist in the file but were not executed.
- RustChunk relevance: Critical. NBT is used by chunk saves, region files, entities, block entities, item stacks and many Minecraft data structures. Lossless Java strings, modified UTF-8, safe length/depth handling, separate disk/network behavior and streaming large tables are particularly valuable.
- 1.20.1 implications: The standard NBT tag IDs and Anvil serialization concepts apply, but verify exact 1.20.1 read/write quirks, root-name behavior, modified UTF-8, NaN handling, limits and SNBT grammar against that version. Keep any 26.3 mixed-list wrapper convention behind a clearly versioned adapter; do not silently unwrap legitimate empty-name compounds in generic 1.20.1 NBT. ItemStack and chunk layouts must use 1.20.1-era NBT rather than modern Data Components.
- Decision: Essential reusable subsystem for a Java client, but version-sensitive edge semantics and custom mixed-list behavior must be separated and validated before use in RustChunk 1.20.1.
- Dependencies / cross-references: horizon-world compression, chunk, registry, region, block-entity and item serialization; NbtAccounter/NbtIo/TagParser equivalents; BlockTableFile streaming; network protocol NBT codecs.
- Follow-up checks: Compare byte-for-byte against 1.20.1 NbtIo/FriendlyByteBuf and TagParser; confirm whether mixed-list wrappers are actually required by target data; make invalid typed-array elements return errors if vanilla does; test trailing input policy, UTF edge cases and decompression/node budget interaction.
- RustChunk implementation mapping: Maps to an essential future shared NBT/SNBT crate used by world saves, entities, block entities, items and protocol. Current block-fact tests do not establish complete NBT support.
- Evidence required before adoption: Run byte-for-byte tests against 1.20.1 NbtIo/TagParser for all tag types, modified UTF-8, root names, arrays, lists, malformed lengths, limits and network/file variants.
- Completion state: Complete source review; embedded tests were read but not run; no fuzzing, NBT corpus round-trip, build or 1.20.1 oracle comparison was performed.

### 112. `crates/horizon-world/src/region.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 2e6e7f9a7f69ac3b0d2c3edb15a392d12f1c119c.
- Size: 538 lines; complete file fetched in consecutive ranges 1–200, 201–400 and 401–538.
- Coverage: lines 1–538, complete; no chunks omitted.
- Purpose: Anvil region-file reader, in-place writer/clearer and non-overwriting exporter, including external chunk files (.mcc).
- Implementation / logic: Uses 4,096-byte sectors and an 8 KiB header with 1,024 chunk slots. Parses r.<x>.<z>.mca paths and validates region coordinates. RegionFile reads offset/timestamp tables, tracks allocated sectors, reads inline or external compressed chunk records, allocates first-fit free sector runs, writes chunks, clears entries and syncs data. Chunk payloads are compressed NBT with a 64 MiB compressed-size cap. RegionWriter writes a whole replacement region to a temporary sibling, syncs it, then publishes it with a non-overwriting hard link; Drop removes temporary/unpublished output and external files on failure. Export preserves untouched compressed records/timestamps and applies explicit overrides/deletions.
- Findings: The sector/header layout is the conventional Anvil shape, so this module is less coupled to gameplay than the chunk serializer. The reader intentionally accepts short headers as zero-filled and drops certain invalid offsets; a full sector-overlap/range consistency audit is not visible in this file. An offset whose first sector begins exactly at EOF is not dropped by the greater-than check and will fail later when the payload is read; verify whether this matches the intended vanilla recovery policy. In-place writes allocate new sectors before freeing old ones, but errors after external-file publication or during header update can leave orphaned data or an in-memory state requiring reopen/recovery. RegionWriter relies on hard-link publication, which must be verified on the Android target filesystem. No unit-test module is present in this file.
- RustChunk relevance: High for durable world saving and portability. Its separation between immutable export and in-place updates, external chunk support, sector allocation and crash-safe temporary publication provides valuable design material.
- 1.20.1 implications: Anvil region headers, sector allocation and external chunk records are broadly applicable, but compression identifiers, external-stub behavior, NBT root semantics, size caps and file recovery behavior must be compared with 1.20.1 RegionFile. Android filesystem support for hard links/rename/sync semantics must be tested. Preserve Minecraft's expected .mca/.mcc compatibility; avoid Horizon-only metadata.
- Decision: Adaptable as a region-storage implementation; verify 1.20.1 compression/repair semantics and replace or supplement hard-link publication if Android filesystem support is insufficient.
- Dependencies / cross-references: horizon-world compression and NBT; ChunkRecord; ChunkPos; world save/load; .mca and .mcc files; Android app-private storage.
- Follow-up checks: Compare malformed header/overlap handling with 1.20.1 RegionFile; test sectors crossing EOF, duplicate/overlapping offsets and external files; inject failures at every write/header/sync point; test hard-link and rename behavior on Android/Linux/Windows; validate all compression methods.
- RustChunk implementation mapping: Maps to future Anvil region storage and save publication in the world-data crate, including external `.mcc` chunks where 1.20.1 requires them.
- Evidence required before adoption: Test region headers/sectors/compression/external chunks and recovery from interrupted writes; run fault injection and Android filesystem tests for atomicity, permissions and storage exhaustion.
- Completion state: Complete source review; no test module is included in this file, and no filesystem fault-injection, world save/load, build or Android storage test was run.

### 113. `crates/horizon-world/src/registry.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: be511976b7379dedf755f7570f5869ffeae4ce45.
- Size: 312 lines; complete file fetched and read.
- Coverage: lines 1–312, complete; no chunks omitted.
- Purpose: Runtime block-state registry and validated per-state fact tables, binding names/properties and default states to version-pinned numeric IDs.
- Implementation / logic: BlockRegistry loads NBT or streamed facts.json, validates DATA_VERSION, ordered states and defaults, shares property strings to reduce allocation, and stores state lookups by a 64-bit hash with a collision fallback map. It exposes ID/state/default lookup, property value enumeration and strict resolution that fills omitted default properties. BlockTable checks that a fact table's states match the registry in exact ID order; BlockTableFile streams and verifies root/direct-child states lists so very large tables are not materialized as complete NBT trees.
- Findings: Registry IDs are positional and tied to the exact generated data order; using an otherwise similar registry with a different order corrupts every ID-based chunk, packet and fact table. The facts.json representation and DATA_VERSION are exporter-specific and must be regenerated together. The in-memory index uses Rust DefaultHasher and explicitly assumes stable keys across processes; its algorithm is not a persistence format and Rust does not promise DefaultHasher's implementation across compiler releases. Hash collisions are handled by the fallback map, but determinism should not be relied upon across Rust versions. BlockTableFile streams the logical state list but still reads the entire file into a byte vector first, so memory pressure remains proportional to compressed/uncompressed table size. No unit-test module is present in this file.
- RustChunk relevance: Critical for correct block state identity. Registry order, default-property resolution and fact-table identity checks must be consistent across world storage, chunk decoding, meshing, collision and gameplay.
- 1.20.1 implications: Build the registry and per-state tables from authoritative 1.20.1 data and preserve exact registry/state order. Do not reuse 26.3 IDs, facts.json schema or DATA_VERSION. Any future mod content changes may alter dynamic registries and must be reconciled with server-assigned network IDs separately from local offline registry tables.
- Decision: Essential concept for the client core, but the current registry data and exporters are version-specific and require a 1.20.1 registry pipeline.
- Dependencies / cross-references: horizon-world chunk and NBT; DATA_VERSION; facts.json generation; BlockState/BlockStateId; BlockTableFile; horizon-prepare exporters; client-render chunk and mesher.
- Follow-up checks: Locate DATA_VERSION and facts.json generator; compare 1.20.1 block-state and default-state ordering; test collisions and cross-version reproducibility; evaluate streaming from a bounded reader instead of reading entire tables into memory; audit packet registry synchronization.
- RustChunk implementation mapping: Maps to the core 1.20.1 registry/state tables that feed world storage, rendering and later protocol decoding. Existing block facts/BlockRegistry tests are partial validated components only.
- Evidence required before adoption: Prove exact 1.20.1 block/state ordering, defaults and registry IDs from official source; test all 1,003 block records/24,135 states and collision/lookup consistency in every consumer.
- Completion state: Complete source review; no build, data import, generated-registry validation or tests were executed.

### 114. `crates/horizon-client-core/src/lib.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 4c4231359c562a86d53c87cb3ca91b7fbda82e7b.
- Size: 29 lines; complete file fetched and read.
- Coverage: lines 1–29, complete; no chunks omitted.
- Purpose: Public module root for the low-level client-core crate.
- Implementation / logic: Forbids unsafe code and exposes modules for block-entity NBT, boss bars, text components, fastutil ordering, game rules, identifiers, key display, language tables, NBT paths/values, particles, patterns, potions, registries, scoreboard, SNBT, sound sources, text, UUIDs and name/player helpers. Defines a common Result alias over boxed dynamic errors. The module-level comment describes this layer as below commands and gameplay and says horizon-client re-exports the old module paths.
- Findings: The file is an API surface and module index only; it does not implement these subsystems or prove their version compatibility. The public modules are broad, so changing an internal module or moving a type can be a downstream API break. The generic boxed error alias is convenient but loses some compile-time error specificity at crate boundaries. No tests are present in this root file.
- RustChunk relevance: Useful for defining a stable low-level public API, grouping text/NBT/registry/utility modules and enforcing a no-unsafe policy in data-oriented code.
- 1.20.1 implications: Keep the layer conceptually, but derive module names and public types from the target-version implementation. Text components, potions, registries, NBT, game rules and block-entity data must match 1.20.1; Horizon's API surface is not a substitute for those semantics.
- Decision: Adaptable as a module-boundary/reference only; the underlying module implementations must be audited individually for 1.20.1.
- Dependencies / cross-references: horizon-client-core/Cargo.toml; all public child modules; horizon-client/src/client_lib.rs; horizon-client/src/lib.rs; horizon-client-lib.
- Follow-up checks: Inspect module-level dependencies and re-export paths; identify version constants and generated tables used by registries/potions/game rules; keep stable APIs separate from protocol-specific DTOs.
- RustChunk implementation mapping: Maps to a future low-level client-core public API grouping versioned data and shared utilities. No equivalent crate root currently exists in RustChunk.
- Evidence required before adoption: Demonstrate clean module boundaries, no unsafe/platform dependency leakage and versioned table coverage; build and test the core independently of Android and rendering.
- Completion state: Complete source review; no module tests, Cargo build or downstream API compilation were run.

### 115. `crates/horizon-client-render/src/lib.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 4abc362410c18ceb8723f0ed6f33e330c525f807.
- Size: 155 lines; complete file fetched and read.
- Coverage: lines 1–155, complete; no chunks omitted.
- Purpose: Public module root for the client rendering/world-presentation/screens crate.
- Implementation / logic: Forbids unsafe code, declares a large public module surface for assets, atlases, audio, auth UI, bidi, biome, block/entity rendering, chunks, meshing, light, LOD, models, fonts, HUD, menus, packs, particles, resource streaming, UI, world presentation and related features. Keeps sprite_cache private. Re-exports horizon_world::json_folder, Result and selected modules/types from horizon-client-core, horizon-client-input, horizon-command and horizon-items. Adds Android controls and keypad modules.
- Findings: This is a large aggregation root, with many modules exposed publicly; it does not itself establish which are actually reachable in the running renderer. The comment states gameplay builds on this crate and that nothing here reaches gameplay, but this architectural claim must be checked through imports because the manifest includes protocol/world/command dependencies. Re-exporting numerous foreign modules creates a broad compatibility surface and may couple UI to lower-level crate names. No tests are included in this file.
- RustChunk relevance: High as a reference for separating renderer/screens from simulation and using re-exports to provide a coherent application-facing API. It also illustrates the maintenance cost of a very wide public module root.
- 1.20.1 implications: Rendering/UI organization is reusable, but every version-facing subsystem—biomes, models, block/entity renderers, light, LOD inputs, assets, chat and screens—must consume 1.20.1-compatible data. A RustChunk first milestone should expose only the modules needed for vanilla rendering rather than port every Starling feature.
- Decision: Adaptable as a rendering/UI module boundary; prune the broad API and reimplement version-specific modules for 1.20.1.
- Dependencies / cross-references: horizon-client-render/Cargo.toml; horizon-world; horizon-client-core; horizon-client-input; horizon-command; horizon-items; horizon-client-lib; renderer startup and game crate imports.
- Follow-up checks: Trace actual import direction between rendering, gameplay and protocol; identify unused or feature-gated modules; map Android controls/keypad lifecycle; verify that no renderer module calls into gameplay through callbacks or global state.
- RustChunk implementation mapping: Maps to a future render/world-presentation API surface. It should expose stable render inputs without pulling protocol/storage implementation into UI.
- Evidence required before adoption: Verify dependency direction with `cargo metadata`; test the public API with 1.20.1 scene fixtures and prove mobile backends compile without desktop-only modules.
- Completion state: Complete source review; no Cargo build, renderer test or Android UI run was performed.

### 116. `crates/horizon-client/examples/probe/main.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 09cc5bca60ce394bcfd1ce3c3bd46475dc003ef3.
- Size: 210 lines; complete file fetched and read.
- Coverage: lines 1–210, complete; no chunks omitted.
- Purpose: Central registry/entry point for the horizon-client parity-probe executable, replacing many individually linked example binaries.
- Implementation / logic: Aliases horizon-client-lib as horizon_client, imports a shared dispatch module and declares fixture/probe modules for audio, camera, input, world rendering, block/entity behavior, menus, inventories, chat, lighting, particles, text, options, sounds and many other client features. A probes! macro registers the executable probe names. The header explains that parity harness scripts invoke the selected probe as if it were an individual example and that one binary avoids repeatedly linking the large client crate.
- Findings: This file mostly lists modules; individual probe implementations and fixture logic live in the child modules and dispatch.rs. The parity harness explicitly excludes this registry file from its dependency fingerprint so adding a new probe does not rerun unrelated harness preparation. The module list and probe names are evidence of the 26.3 behavior surface Starling tests, not evidence that those features already work or pass tests. No probes are executed by merely compiling or reading this file.
- RustChunk relevance: High as a test/build scaling technique. A shared probe binary can reduce repeated code generation and link time when a client has many parity fixtures, which is relevant on constrained build runners.
- 1.20.1 implications: The harness architecture can be reused, but fixtures must be tied to 1.20.1 vanilla outputs and must not reuse 26.3 expectations. RustChunk should keep probe inventory separate from the actual client startup path and from production APK targets.
- Decision: Adaptable as a parity-test harness registry; the listed probes and fixture semantics are version-specific and need 1.20.1 replacements.
- Dependencies / cross-references: horizon-client-lib; dispatch.rs; all declared probe modules; tools/parity.py; tools/test_*_parity.py; tools/oracle_harness.py; Cargo example target.
- Follow-up checks: Inspect dispatch.rs and representative probe fixtures; confirm probe target is excluded from release builds; identify oracle version and expected output format for each probe; measure binary link/build time against separate examples.
- RustChunk implementation mapping: Maps to a future client parity-probe registry/runner for gameplay and client behavior. The existing `tests/` directory currently tests exporters and repository tooling, not a full client runtime.
- Evidence required before adoption: Require each probe to identify its 1.20.1 Java oracle, deterministic inputs and expected output; prove probe targets are excluded from release APKs and that failures propagate to CI.
- Completion state: Complete source review; no probe binary build, parity harness run, test or benchmark was executed.

### 117. `crates/horizon-client/src/bin/horizon-ctl/main.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: b20c846daf1a9e830063d7c94dc96f927d527c3c.
- Size: 695 lines; complete file fetched and read in two consecutive ranges: 1–400 and 401–695.
- Coverage: lines 1–695, complete; the second range includes the full test module.
- Purpose: Command-line control client for a running Horizon game, scripts and MCP/agent workflows, communicating with the game's local/remote control socket.
- Implementation / logic: Defines structured failures/exit codes for game errors, usage, unreachable connection and no-AI denial. Client connects through horizon_client::control::connect, reads newline-delimited JSON replies, caches the socket in Session and reconnects after unreachable errors. Session routes CLI commands to local command handlers or wire messages, supports launch/wait-ready, entity interaction, held keys, logs/crash reports, screenshots, scripts, absolute file paths and MCP service mode. split_line implements shell-like quoting, escapes and comments. The executable supports global socket/timeout/pretty flags, human-readable output for logs/help, script keep-going behavior and no-AI refusal. Embedded tests cover log tail/crash selection, command-line splitting, unreachable games, no-AI restrictions and screenshot path handling.
- Findings: The control socket is a separate automation interface, not the Minecraft server protocol. It provides powerful scripted and MCP control of the running game, so the no-AI gate and command dispatcher are a meaningful security boundary. Numeric arguments use f64 parsing and pass values into Duration::from_secs_f64 in timeout/sleep/hold/wait paths; non-finite or out-of-range values should be rejected before conversion to avoid a panic. The script loop silently breaks on an input read error rather than reporting it, which can make a truncated script appear successful. Remote screenshot payloads are base64-decoded and written to the requested path; path and payload size policy should be reviewed in the transport layer. The file contains tests but they were read, not executed.
- RustChunk relevance: Optional tooling only. A local control interface could be useful for automated regression tests and device testing, but it should not be part of the initial vanilla client or expose unrestricted gameplay/file operations by default.
- 1.20.1 implications: The CLI is independent of Minecraft's wire protocol, but any game commands it forwards must use the RustChunk command/action API and enforce permissions at the running-client boundary. A future tool interface should have explicit authentication, capability scoping, bounded requests/replies, path restrictions and a disabled-by-default policy for external agents.
- Decision: Unnecessary for the vanilla 1.20.1 gameplay core; adaptable later as a separately secured developer/test-control tool after the runtime and command API stabilize.
- Dependencies / cross-references: horizon-client control socket; commands, launch and mcp modules; horizon_core::no_ai; logging; JSON line protocol; screenshot/base64 helpers; Cargo target registered in horizon-client-lib.
- Follow-up checks: Reject NaN/infinity/overflow numeric values before Duration conversion; make script read errors nonzero; inspect server-side command allowlist and no-AI gate; test remote socket authentication, screenshot size/path bounds and MCP capability scoping.
- RustChunk implementation mapping: No production mapping. A local control CLI may later support deterministic device/regression testing, but it is not required for the vanilla gameplay core.
- Evidence required before adoption: Keep absent until a stable command API exists. Before adoption require strict allowlisting/authentication, bounded numeric/input parsing, no unsafe remote exposure and end-to-end tests on host/device.
- Completion state: Complete source review; embedded tests were read but not run; no control server, MCP session, script, screenshot, build or live game command was executed.

### 118. `crates/horizon-client/src/lib.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 3699d714d7f5684c82f9401eee44f07cd291c0b2.
- Size: 14 lines; complete file fetched and read.
- Coverage: lines 1–14, complete; no chunks omitted.
- Purpose: Library root for the gameplay/startup crate above the reusable client library.
- Implementation / logic: Re-exports horizon-client-lib public items, aliases the current crate as horizon_client, exposes entry, game, mod_api, diagnostics and performance modules, and includes the Android module only on Android. The crate forbids unsafe code on non-Android targets but deliberately does not apply that crate-level prohibition on Android, where JNI/native-activity hooks may need unsafe interoperability.
- Findings: This is a thin composition boundary rather than the runtime entry point itself. The conditional unsafe policy is a security-sensitive exception: Android-specific unsafe code must remain tightly scoped and reviewed, while all other targets retain the prohibition. Re-exporting the entire client library creates a broad and potentially unstable public surface. No tests are included in this root.
- RustChunk relevance: High for designing a small platform-aware startup crate above a reusable library, especially when Android JNI glue differs from desktop startup.
- 1.20.1 implications: This structure can be adapted, but the game/entry/diagnostics APIs and JNI lifecycle must target RustChunk's 1.20.1 engine. Keep unsafe Android JNI code isolated in a small module and expose safe interfaces to the rest of the client.
- Decision: Adaptable as a thin platform composition layer; Android unsafe/JNI boundary and all game-specific modules require independent implementation and review.
- Dependencies / cross-references: horizon-client-lib; android.rs; entry.rs; game.rs; mod_api.rs; diagnostics.rs; performance.rs; src/main.rs; Android activity/JNI bridge.
- Follow-up checks: Inspect android.rs and entry.rs; audit every unsafe block behind the Android cfg; trace NativeActivity pause/resume/surface/destroy lifecycle; narrow public re-exports if the client library API is not intended to be stable.
- RustChunk implementation mapping: Maps to the future thin native application/platform composition layer. Current `engine/src/lib.rs` is a JNI bridge to download/installer/Forge/JLI routines, not the native game library.
- Evidence required before adoption: Require lifecycle tests for JNI/NativeActivity or chosen entry path, safe/isolated unsafe boundaries, correct initialization order and a 1.20.1 native gameplay launch; distinguish launcher test results.
- Completion state: Complete source review; no Android compile, JNI launch, lifecycle test or gameplay run was performed.

### 119. `crates/horizon-client/src/main.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: ed5f7f5c16a4d8c90cdc8cf9e3985f3ef14eee7f.
- Size: 20 lines; complete file fetched and read.
- Coverage: lines 1–20, complete; no chunks omitted.
- Purpose: Minimal executable entry point for the Horizon native Minecraft Java 26.3 client.
- Implementation / logic: Forbids unsafe code, selects horizon_threads::Allocator as the global allocator unless system-allocator or alloc-census is active, and selects CensusAllocator when alloc-census is enabled without system-allocator. Embeds the 27-byte MOD_API_LEVEL marker from horizon_mods::api_level::MARK so the launcher can inspect the mod API level without running the game. main() keeps the marker alive with black_box and delegates startup to horizon_client::entry::main().
- Findings: The file intentionally contains almost no game logic; startup, Android activity callbacks and the runtime are elsewhere. Allocator selection is compile-feature-dependent and can materially affect memory profiling/performance; the 27-byte array length also constrains the mod API marker's compile-time representation. The source comment explicitly pins this executable to Minecraft Java 26.3. No tests are present in this file.
- RustChunk relevance: High as a minimal native executable design. Separating entry/allocator configuration from client startup keeps the binary root easy to inspect and lets profiling builds swap allocators deliberately.
- 1.20.1 implications: Retain a minimal main and deliberate allocator policy, but remove or replace the Horizon mod API marker unless RustChunk defines its own extension ABI. The game-version identity must be 1.20.1 and should be shared with resource/protocol manifests rather than being inferred from a display string.
- Decision: Adaptable as a minimal executable/bootstrap pattern; the 26.3 mod marker and allocator feature set are Horizon-specific and must be reconsidered.
- Dependencies / cross-references: horizon_threads allocator; horizon_mods API level marker; horizon_client entry::main; horizon-client Cargo features; launcher version inspection.
- Follow-up checks: Inspect entry::main and Android launch bridge; verify allocator feature precedence and memory-accounting overhead; define RustChunk's own version/build identity marker and decide whether a native extension API exists.
- RustChunk implementation mapping: Maps to a future minimal native game executable/bootstrap and deliberate allocator configuration. Current RustChunk exposes a JNI cdylib; it does not yet contain a native game `main` entry point.
- Evidence required before adoption: Require host and Android native builds, allocator feature tests/benchmarks and proof the executable starts the 1.20.1 game runtime without launching a Java game process.
- Completion state: Complete source review; no executable build, allocator benchmark or runtime launch was performed.

### 120. `crates/horizon-world/examples/probe/main.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 56dbc88433ffb014d86a70186685a2f3dc55d4fb.
- Size: 365 lines; complete file fetched and read.
- Coverage: lines 1–365, complete; no chunks omitted.
- Purpose: Central module registry for the horizon-world parity-probe executable, combining a large set of world, storage, entity, block, movement and world-generation fixtures into one binary.
- Implementation / logic: Imports a shared dispatch macro and an args helper, defines shared support modules for common data, decoration storage, placement queries and shapes, then declares and registers roughly 170 probe/fixture modules. The list spans effects, attributes, biome/chunk/worldgen, block state/collision/support/survival, entities, fluids, inventory/items, player travel/combat, structures, ticks, world borders, ray queries and deterministic random behavior. The header explains that one probe binary avoids repeatedly instantiating/linking the crate's generic-heavy code in many separate examples.
- Findings: Like the client probe registry (entry 116), this file mostly enumerates modules; the actual assertions, input fixtures and output formats are in child modules and dispatch.rs. It includes datafix_world, chunk dumps and many version-specific parity fixtures, but the list alone does not prove any test has passed. Building this example can still be expensive because it links a large set of probes, though it is designed to reduce repeated builds across separate examples. The module list is a testing inventory, not production world runtime code.
- RustChunk relevance: High as a test infrastructure pattern. A single dispatchable probe binary can lower repeated code-generation/link costs and provide a stable interface for parity automation across many world subsystems.
- 1.20.1 implications: The one-binary registry approach is reusable, but all fixtures, oracle values, datafix cases, worldgen behavior and serialization expectations must be sourced from 1.20.1. Do not treat 26.3 probe outputs as 1.20.1 truth.
- Decision: Adaptable as a parity harness organization pattern; every child fixture must be replaced or versioned for 1.20.1.
- Dependencies / cross-references: horizon-world crate; dispatch.rs; support/common; support/decoration_storage; support/placement_queries; support/shapes; all declared fixture modules; tools/parity.py; horizon-worldgen-parity; Cargo example target.
- Follow-up checks: Inspect dispatch.rs and representative chunk/NBT/worldgen fixtures; identify how vanilla oracle outputs are generated and version-pinned; verify probes are not included in production APK; measure compile/link time and output determinism.
- RustChunk implementation mapping: Maps to a future world-data parity-probe binary/registry, validating chunk, NBT, block, entity and worldgen subsystems against 1.20.1 Java outputs. It is test-only, not production gameplay code.
- Evidence required before adoption: Require representative probes for each world subsystem, exact 1.20.1 oracle provenance, deterministic output and CI failure on mismatch; prove the test binary is never bundled into the release APK.
- Completion state: Complete source review; no probe build, parity run, datafix test, world generation, benchmark or Android runtime execution was performed.


## Batch checkpoint — entries 101–120 — 2026-11-18

- Completed full source reviews for entries 101–120. All 20 paths and blob SHAs match the pinned Starling source tree snapshot `5432370d573642041585c69b40fb045b27133488`.
- Files longer than 400 lines were read in consecutive chunks with no gaps; the 1,076-line render chunk, 999-line world chunk, 1,152-line NBT and 695-line control CLI files received complete-range coverage.
- Added each file's source identity, size/coverage, purpose, implementation, findings, RustChunk relevance, 1.20.1 implications, decision, dependencies, follow-up checks and completion state.
- Extended `AUDIT_INDEX.md` to all 120 reviewed files, split into five parts of 25, 25, 25, 25 and 20 entries.
- Risks recorded for follow-up include the render chunk section-index/min_y relationship, SNBT typed-array invalid-value behavior, registry hash stability assumptions, region-file failure recovery and Android hard-link support, native-mod permission validation, and finite numeric bounds in horizon-ctl.
- Tests found in source were read but not run. No Cargo build, APK build, Android device run, filesystem fault-injection, parity probe or live Minecraft connection was executed.

## Backfill checkpoint — entries 1–120 — 2026-11-18

- All 120 earlier source reviews and their original findings/decisions are preserved.
- Added the two current-standard fields to every entry 1–120: 120 implementation mappings and 120 evidence gates (240 new field records total).
- The mappings explicitly mark future native gameplay/protocol/world/render/input modules as planned, not implemented; the current RustChunk JNI/installer/Forge launch-plan code is distinguished from the native game target.
- Evidence gates are requirements only. This documentation change did not run Cargo tests, build an APK, test a device or establish Forge/multiplayer support.

### 121. `crates/horizon-client/src/entry.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 7bc456b6ffdc89d055d8428db37dc371d5c44233.
- Size: 246 lines; complete source file fetched and reviewed.
- Coverage: lines 1–246, complete; no gaps between ranges.
- Purpose: Shared desktop/Android startup orchestration, including early hook installation, logging/crash setup, platform arguments and runtime launch.
- Implementation / logic: Installs trait mods before lower-level services can invoke them; configures allocator and startup telemetry; reads desktop arguments or Android-supplied arguments; selects game home, player directory or dedicated-server folder. It initializes logging/crash/RAM-limit services, runs the post-log platform callback, installs native renderers and world/client hooks, adjusts Unix open-file limits when possible, then enters the game runtime. It classifies usage versus runtime errors, flushes logs and uses platform-specific shutdown.
- Findings: Startup ordering is essential because several global hook registries are one-shot. Android arguments come from the Activity bridge, not OS argv. Dedicated-server setup is not needed by an initial client-only release. File-limit adjustment is best-effort. No local tests exist.
- RustChunk relevance: High as a bootstrap-order and error-lifecycle reference.
- 1.20.1 implications: Structure is reusable, but Horizon mod hooks, telemetry and server branches are product-specific. RustChunk should initialize only its platform, 1.20.1 resources/registries, diagnostics and game runtime.
- Decision: Adaptable as startup architecture; mod integration and server branches require redesign or omission for vanilla 1.20.1.
- Dependencies / cross-references: crash, game runtime, logging, RAM limit, mod_api, allocator, telemetry, native renderer, world/client hooks, Android args/finish bridge and Unix file limits.
- Follow-up checks: Map every side effect to RustChunk; decide which initializers are one-shot; test arguments, error classification, Android lifecycle, shutdown and file-descriptor needs.
- RustChunk implementation mapping: Candidate for executable/bootstrap module, not protocol or renderer.
- Evidence required before adoption: Desktop/Android launch, duplicate-init, argument and shutdown tests; proof all required 1.20.1 resources are initialized before use.
- Completion state: Complete source review; no RustChunk implementation, compile or runtime launch was performed.

### 122. `crates/horizon-client/src/android.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 546c37dc4c03642b6bb3f95bfae21a7183ab1ea4.
- Size: 1,386 lines; complete source file fetched and reviewed.
- Coverage: lines 1–1,386 in consecutive ranges 1–300, 301–550, 551–800, 801–1,050, 1,051–1,134 and 1,135–1,386, complete; no gaps between ranges.
- Purpose: Android NativeActivity/JNI bridge for storage, lifecycle, input, IME, sensors, clipboard, file picking, links, native-library loading, system statistics and frame scheduling.
- Implementation / logic: Retains AndroidApp/JNI/activity handles and one-shot process arguments. Provides private/shared folder selection and migration helpers; maps Android key, motion, pointer and gamepad events into client input; maintains mouse state and a bounded event queue; mirrors text-field/IME composition and selection; exposes clipboard, file picker, folder opening, browser links and file hooks. It handles vibration, gyroscope, DNS, foreground-work hooks, optional native-mod library loading, heap/thermal probes, Android Performance Hint Manager and native-window frame-rate requests. Startup configures environment/platform hooks, diagnostics, preparation/account resume and input before entering the game. Shutdown finishes the Activity then terminates the process because the event loop cannot be recreated in place.
- Findings: Large unsafe FFI/JNI boundary with object-lifetime and thread-affinity assumptions. Correctness depends on Android-owned lifetimes, valid JNI attachment, native-window lifetime and a single process event loop. Newer Android APIs are dynamically resolved and may be absent. Shared-storage migration needs interruption/retry testing. Mouse/controller state must reset on focus loss, pause, pointer cancel and device removal. Native-mod loading is not Forge. No device run was performed.
- RustChunk relevance: Very high for Android NativeActivity, IME, touch, sensors, clipboard, storage and lifecycle integration.
- 1.20.1 implications: Mostly protocol-independent, but input must feed 1.20.1 movement/interaction. Remove Starling-specific telemetry, accounts, native mods and folder assumptions unless RustChunk independently needs them.
- Decision: Highly relevant as an Android architecture reference; implement a smaller RustChunk bridge with explicit unsafe/JNI review and device validation.
- Dependencies / cross-references: android-activity/winit, JNI, input/touch/gamepad, platform hooks, diagnostics, storage helpers, NativeMods Java bridge, system stats, account/preparer and Android packaging.
- Follow-up checks: Audit unsafe blocks/JNI signatures; trace Activity/surface lifecycle; test IME/RTL, multi-touch cancel, controller disconnect, storage migration, symbol fallbacks and thermal/frame-rate APIs.
- RustChunk implementation mapping: Candidate for platform/android module; port required services only, not native-mod loading/account renewal by default.
- Evidence required before adoption: Android ABI/API compile, repeated launch/close, lifecycle, touch/IME/controller/clipboard/storage tests and real-device frame pacing checks.
- Completion state: Complete source review; no APK build, JNI compile, device test or Android performance measurement was run.

### 123. `crates/horizon-client/src/diagnostics.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 543befa72bfae5d207397fd3558144e88b7c2ce9.
- Size: 606 lines; complete source file fetched and reviewed.
- Coverage: lines 1–606 in consecutive ranges 1–300, 301–400, 401–480, 481–567 and 568–606, complete; no gaps between ranges.
- Purpose: Bounded local crash/startup diagnostics and explicit diagnostic ZIP export, separated from opt-in performance recording.
- Implementation / logic: Stores session JSON with build/device/ABI/start phase/clean-exit/context and previous-session data; writes updates through a bounded sync-channel worker so disk operations stay off the frame thread. Heartbeat uses nonblocking lock/send operations. It supports checkpoints, handled/fatal errors, flush, previous-unclean detection and summaries. A redactor removes selected credential terms, URLs, addresses, private paths and domains. Explicit export allowlists logs/crash/performance/exit summaries, limits file count and size, keeps complete JSONL rows, summarizes binary tombstones, writes a manifest and retains three archives. Tests cover redaction, archive limits, JSONL, worker/heartbeat and exit labels.
- Findings: The bounded async design is useful, but redaction is best-effort pattern filtering, not a guarantee arbitrary log text is safe. New token formats or paths can evade it. JSON persistence uses temp plus rename without visible fsync. Worker send/write failures are often ignored to avoid blocking the frame thread, so diagnostics may be incomplete. Archive metadata overhead should be checked against the exact cap. Tests were read but not run.
- RustChunk relevance: High for a mobile client needing crash/OOM/thermal/GPU diagnostics without blocking rendering.
- 1.20.1 implications: Protocol-independent. Replace Starling identifiers/Android exit integration and define RustChunk privacy, retention, export contents and consent.
- Decision: Adaptable diagnostics architecture; redaction, durability and Android exit collection need security/privacy review.
- Dependencies / cross-references: serde_json, zip, logging, crash metadata, performance records, tombstone summarizer, Android exit reports and share provider.
- Follow-up checks: Fuzz redaction; test symlinks, duplicate names, malformed JSONL, concurrent writers and archive limits; decide on fsync; document exactly what leaves the device.
- RustChunk implementation mapping: Candidate for separate diagnostics subsystem called by startup, renderer, memory monitor and Android recovery; not Minecraft protocol or gameplay.
- Evidence required before adoption: Redaction tests, interrupted-write tests, bounded-memory log-flood tests and Android crash/low-memory integration tests; inspect exports before sharing.
- Completion state: Complete source review; tests were read but not run; no export or Android crash test was performed.

### 124. `crates/horizon-client-render/src/chunk_map.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 484bd96756d5d21493e879a10abd2146c793d5b4.
- Size: 386 lines; complete source file fetched and reviewed.
- Coverage: lines 1–386, complete; no gaps between ranges.
- Purpose: Resident chunk-column membership map with version tokens and a bounded change journal for consumers caching loaded chunk keys.
- Implementation / logic: Stores chunk coordinates to Arc<Chunk>, tracks fresh columns and uses an AtomicU64 revision source. Insert/remove update membership revision only when membership changes; replacing payload under an existing key does not. A 256-entry VecDeque retains recent Added/Removed deltas. deltas_since returns changes only when the revision belongs to the map and remains in the retained window; otherwise consumers rebuild. clear records Reset; fresh-key helpers track chunks awaiting entity placement. It implements LightWorld through resident chunks. Tests cover interleaved maps, expired history, reset and fresh-key behavior.
- Findings: Separates membership changes from payload/light updates and avoids repeated full key scans. The bounded journal falls back to rebuild when history is incomplete. Revision wrap is not explicitly guarded. Fresh-key iteration follows fast-hash order and is not persistent/deterministic across runs.
- RustChunk relevance: High for thousands of loaded chunks at long render distances; deltas can reduce per-tick work for rendering, streaming and lighting.
- 1.20.1 implications: Structure is version-independent, but membership should be dimension/level-scoped and follow packet load/unload, dimension transition and reset, not in-place block edits.
- Decision: Adaptable as resident chunk membership bookkeeping; revision lifetime, reset semantics and ordering assumptions need tests.
- Dependencies / cross-references: Chunk, fast hash map/set, Arc, LightWorld, streaming, mesh workers and entity placement.
- Follow-up checks: Trace delta consumers; test revision wrap or use map-local generation tokens; test dimension reset/load-unload races; compare journal replay with full snapshots under bursts.
- RustChunk implementation mapping: Candidate for world/chunk_store membership bookkeeping and notifications to meshing/streaming workers; not packet decoding or persisted identity.
- Evidence required before adoption: Property tests against full snapshots, stress beyond 256 changes, dimension/reset tests and benchmarks with thousands of chunks.
- Completion state: Complete source review; tests were read but not run; no compile, concurrency stress or render-distance benchmark was performed.

### 125. `crates/horizon-client-render/src/mesher.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: a5aff352590f344ef169db790386b7256e6ece4f.
- Size: 2,028 lines; complete source file fetched and reviewed.
- Coverage: lines 1–2,028 in consecutive ranges 1–350, 351–700, 701–1,050, 1,051–1,200, 1,201–1,400, 1,401–1,550, 1,551–1,750 and 1,751–2,028, complete; no gaps between ranges.
- Purpose: Main block-section mesher converting resident chunk states, baked models, biome tints and light into GPU terrain geometry.
- Implementation / logic: Defines compact vertex layouts (fixed-point UVs, atlas layer, light and packed color/HDR/fluid flags), SectionMesh buffers/bounds and per-state MeshTables. MeshTables derive state facts from registry/model/render assets: solidity, permeability, shade/emission, tints, block entities, fluids, face occlusion and animated layers. Padded loads a section plus two-block borders from a 3x3 chunk neighborhood; MeshScratch reuses per-worker buffers. Meshing performs hidden-cell/visibility flood fills, gathers block-entity/light references, tessellates fluids, loads baked models, applies offsets and neighbor culling, optional leaf culling and AO, samples biome colors, applies connected textures/overlays, splits compact-CTM faces and writes solid/translucent vertices.
- Findings: Highly version/asset-coupled renderer. Registry IDs directly index MeshTables, so mismatched facts can produce wrong states. Padded borders/missing-neighbor behavior affect AO/fluid seams, requiring neighbor mesh invalidation on chunk arrival/unload. Mesh vertex buffers can grow substantially even if worker scratch is bounded. Custom HDR/material flags in vertex alpha are Horizon shader conventions, not vanilla semantics. Tests cover UV packing, visibility, tint, leaves, fluids and geometry; not run.
- RustChunk relevance: Critical. Meshing controls chunk correctness and CPU/RAM use at long render distances. Border sampling, scratch reuse, compact vertices, visibility flood fill and separate solid/translucent buffers are valuable leads.
- 1.20.1 implications: Rebuild state facts from 1.20.1 block definitions, model JSON, render shapes, light, biome colors, fluids and assets. Verify dimension section ranges, AO/culling, offsets and fluid geometry. Exclude 26.3 names, HDR, Fusion and OptiFine integrations from vanilla baseline unless deliberately added as optional support.
- Decision: Essential rendering subsystem concept; reimplement against 1.20.1 model/registry/biome/light/shader data rather than copy as-is.
- Dependencies / cross-references: Chunk/Section/chunk_map; BlockFacts/MeshTables; baked model catalog; biome zoom; FaceVisibility; fluid/leaf culling; lighting.rs; connected textures; wgpu terrain shaders.
- Follow-up checks: Build minimal 1.20.1 cube/slab/stair/fluid mesher; golden vertex/image captures; test neighbor arrival/unload and border seams; compare visibility flood fill; profile allocations/mesh size on Android.
- RustChunk implementation mapping: Candidate for render/chunk_mesher after chunk decoding, registry and model baking; emits GPU geometry, not packet decoding or chunk persistence.
- Evidence required before adoption: 1.20.1 golden fixtures for blocks/models/fluids, pixel/vertex parity, neighbor tests, malformed-resource handling and Android CPU/RAM/GPU benchmarks.
- Completion state: Complete source review; tests were inspected but not run; no compile, shader build, visual comparison or GPU execution was performed.

### 126. `crates/horizon-client-render/src/mesher/lighting.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 84677087b5f715bb67274480ec6db061749cabfa.
- Size: 507 lines; complete source file fetched and reviewed.
- Coverage: lines 1–507, complete; no gaps between ranges.
- Purpose: Per-vertex block/sky light, directional face shade and ambient-occlusion calculations for baked model quads, including partial faces.
- Implementation / logic: Packs block/sky values into light coordinates; handles emissive blocks, per-quad emission, max/smooth blending, cardinal face shade, gray/scale operations and Java-compatible integer narrowing. An adjacency table defines corner sampling for six faces. Quad geometry is classified as partial/cubic; flat lighting samples the block or neighbor; AO samples corners/diagonals/permeable states and weights partial faces. Tests cover sample fallback, blending, float-floor parity, color math, emission and face sampling.
- Findings: Small arithmetic/remapping changes can create visible seams. gray contains a platform-specific x86-64 floor workaround and needs ARM64 validation. weighted_blend intentionally narrows/masks values; clamping would alter parity. AO requires padded block/light neighbors and correct state facts. Tests were read but not run.
- RustChunk relevance: Critical subcomponent affecting smooth light, AO, partial models and mobile visual quality.
- 1.20.1 implications: Verify exact 1.20.1 BlockModelLighter, ModelBlockRenderer, LightCoordsUtil and Mth arithmetic. 26.3 model facts and shader packing are not presumed identical.
- Decision: Highly adaptable as algorithm reference; constants, state facts, float narrowing and shader packing need 1.20.1 parity tests and ARM64 validation.
- Dependencies / cross-references: mesher Info/Padded; baked ModelQuad geometry; block/sky light; emission/permeability/solid/shade properties; terrain vertex and shader lightmap.
- Follow-up checks: Compare helpers with 1.20.1 source; golden AO vectors for six faces/partial shapes; test negative geometry, emissive blocks and missing-neighbor borders.
- RustChunk implementation mapping: Candidate for render/lighting helpers called by chunk mesher; keep GPU packing distinct from stored/network light representation.
- Evidence required before adoption: 1.20.1 golden vectors for flat/AO/light/emission and visual parity on Android ARM64; verify CPU output against shader decoding.
- Completion state: Complete source review; tests were read but not run; no oracle, ARM64 math or shader test was performed.

### 127. `crates/horizon-client-render/src/world.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 3bd8f012e2597e6be18d980e11ee894f62ee7fc2.
- Size: 634 lines; complete source file fetched and reviewed.
- Coverage: lines 1–634, complete; no gaps between ranges.
- Purpose: Resident client-world state joining chunk storage with remesh/relight queues, block entities, weather, height queries and gameplay bookkeeping.
- Implementation / logic: DirtySections stores section bit masks by column. World owns dimension shape, ChunkMap, random state, revision, air ID, dirty geometry/light, light edits, loaded chunks/entities, modified columns, block-entity tickers, POIs, clocks/weather and optional network/path logs. set uses Arc copy-on-write; on change it updates block-entity/watch/path/network logs, marks affected sections in the 3x3x3 AO neighborhood and queues incremental light edits or marks nearby columns for relighting. Queries provide heightmaps, block/light reads, palette-assisted may_hold, tickers and saved lock-code retention.
- Findings: More than a passive chunk store: it combines renderer state with some simulation bookkeeping. Dirty propagation is optimized, but wrong dimension min_y/section/border math can leave stale mesh/light. Attachments, watched writes and mod-specific fields are not needed in minimal vanilla. Tests cover dirty-neighborhood marking and may_hold; not run.
- RustChunk relevance: Very high for coordinating loaded chunks, renderer invalidation, light recomputation and gameplay notifications without rescanning every tick.
- 1.20.1 implications: Preserve storage/render/gameplay separation, but derive dimension layouts, heightmaps, block entities, POIs and weather from 1.20.1. Keep Horizon attachment/native-mod tracking out of vanilla base.
- Decision: Adaptable as resident-world orchestration; ownership and invalidation should be simplified and verified against 1.20.1.
- Dependencies / cross-references: Chunk/ColumnLayout/LevelShape; ChunkMap/SectionMask; BlockStateId/NBT; light engine/streaming; block-entity manager; biome/POI/heightmaps; game loop.
- Follow-up checks: Draw state-ownership graph; test block writes at chunk/section corners and min/max Y; compare dirty propagation to brute-force 3x3x3; compare incremental/full relight; test unload while jobs run.
- RustChunk implementation mapping: Candidate for world/resident_world coordinating loaded chunks, dirty mesh sections, light updates and reads; packets/interactions call explicit methods.
- Evidence required before adoption: 1.20.1 Overworld/Nether/End tests, property-based dirty-region tests, incremental/full-light equality and multi-thread load/unload tests plus Android profiles.
- Completion state: Complete source review; embedded tests were read but not run; no world integration or relight execution was performed.

### 128. `crates/horizon-client-render/src/chunk/block_entity.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 9f50876e831d035f10430918a04b9c0ad94ae7af.
- Size: 755 lines; complete source file fetched and reviewed.
- Coverage: lines 1–755, complete; no gaps between ranges.
- Purpose: Plain client-side data models for block entities stored in chunks; behavior lives in separate feature systems.
- Implementation / logic: Defines typed models for chest, furnace, brewing, signs, jukebox, beacon, lectern, sculk, banners, crafter, spawner, beehive, command block, campfire, skull, end gateway and later entities. Uses ItemStack/NBT/Text/UUID types from shared crates. ModBlockEntity stores ID, saved NBT, item slots and menu data; its revision changes on mutable dereference, clones get new identities, and update-tag merges replace only supplied changed keys. Additional models describe mod containers, fluid tanks and energy buffers. Embedded tests check revision changes and idempotent update-tag merging.
- Findings: Plain-data/behavior separation is sound, but the enum mixes 1.20-era types with later trial-spawner/vault/shelf/creaking-heart and native-mod entities. It is not a clean 1.20.1 type list. Revision increments on mutable dereference even if no change; a retained mutable reference can be changed repeatedly without another revision. merge_update_tag is partial merge, not whole-compound replacement. Tests cover only mod entity revisions; not run.
- RustChunk relevance: High for separating block-entity payloads from rendering/behavior. Typed models can optimize common entities while preserving unknown NBT.
- 1.20.1 implications: Derive exact entity types/fields from 1.20.1 save and update-packet behavior. Keep newer/unknown entities opaque only under an explicit preservation policy; keep Forge/modded data out of vanilla base.
- Decision: Adaptable as typed-data architecture; enum/extension types must be rebuilt for 1.20.1 and revision semantics made safer.
- Dependencies / cross-references: client-core Text/Who; items ItemStack/BannerLayer; horizon-world NBT/SculkSpreader; block entity manager/tickers; renderers/gameplay; native mod API.
- Follow-up checks: Enumerate 1.20.1 block entity types/packet fields; inspect save/load and unknown-tag preservation; consider immutable snapshots or mutation guards; verify signs/inventories/update tags.
- RustChunk implementation mapping: Candidate for world/block_entities typed data plus separate renderer lookup; ticking/interaction belongs to gameplay.
- Evidence required before adoption: 1.20.1 fixtures for each supported type, NBT round trips, update-tag merge/replace, unknown-type retention and mutation revision tests.
- Completion state: Complete source review; embedded tests were read but not run; no serializer, renderer or gameplay integration was built.

### 129. `crates/horizon-world/src/compression.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: d1cd863c212a9e4b29c313f79f8c109fc9ce88bc.
- Size: 363 lines; complete source file fetched and reviewed.
- Coverage: lines 1–363 in consecutive ranges 1–200 and 201–363, complete; no gaps between ranges.
- Purpose: Compression/decompression codecs used by Anvil region records, including lz4-java block framing.
- Implementation / logic: Maps compression IDs 1–4 to Gzip, Zlib, None and Lz4. Decode accepts a maximum output size and can append to an existing Vec. Gzip uses MultiGzDecoder, Zlib uses streaming Decompress and detects no-progress/truncated streams, and None checks the size limit. LZ4 expects LZ4Block framing (not LZ4 frame), validates methods/lengths, checks xxHash32 with Java seed/mask and implements Java fast block token/end behavior. Encoder emits 65,536-byte raw/compressed blocks and a terminator. Tests compare Java-LZ4 cases and cover round trips, limits and corruption.
- Findings: Output budgets and checksums are valuable for untrusted world files. LZ4 framing is lz4-java block stream, not generic LZ4 frame. Zlib returns at StreamEnd without visibly requiring all input consumed; Gzip accepts concatenated members; LZ4 returns at a terminator without visibly rejecting following bytes. The presence of ID 4 here does not prove vanilla 1.20.1 accepts it. Tests read but not run.
- RustChunk relevance: High for region files and protection from decompression bombs/corrupt data.
- 1.20.1 implications: Confirm exact 1.20.1 Anvil compression IDs/header/trailing-byte behavior. Implement vanilla-required codecs first; optional modded compression must be explicit. Coordinate decompression budgets with NBT limits.
- Decision: Adaptable as bounded compression layer; LZ4 variant and codec-ID table require version verification.
- Dependencies / cross-references: flate2; lz4_flex; xxhash-rust; RegionFile/ChunkRecord; NBT limits; Anvil region header.
- Follow-up checks: Compare IDs with 1.20.1 RegionFile; collect real fixtures; test truncated header/payload, checksum mismatch, trailing garbage, concatenated gzip and exact-limit boundaries.
- RustChunk implementation mapping: Candidate for world/region_compression below chunk NBT; separate from network packet compression.
- Evidence required before adoption: Byte-for-byte 1.20.1 region fixtures, decompression-bomb/fuzz tests and Android ARM64 benchmarks.
- Completion state: Complete source review; embedded tests were read but not run; no compression corpus, fuzzing, build or Android benchmark was performed.

### 130. `crates/horizon-world/src/datafix/mod.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: a3dfa02f5e19ad1368fa8b6b448c0dd19dc45a8c.
- Size: 386 lines; complete source file fetched and reviewed.
- Coverage: lines 1–386 in consecutive ranges 1–200 and 201–386, complete; no gaps between ranges.
- Purpose: DataFixerUpper orchestration for migrating older world/entity/player/item data into current Starling/Horizon 26.3 schema.
- Implementation / logic: Defines schema types and version-keyed fix registry; stores last-read record remainder so unknown fields can survive rewrites; applies read hooks, ordered fixes and write hooks; exposes updates for chunks, entity/POI tags, player, hotbar and saved data. Chunk fixes receive dimension/generator context. Item/name hooks normalize historical forms. CURRENT comes from Horizon DATA_VERSION; OLDEST_SUPPORTED is -1.
- Findings: Architecture is useful, but each fix/key/schema targets 26.3. A read hook converts older ItemStacks to Data Components at a later key, which must never run in a live 1.20.1 client. update_chunk_to attaches temporary context and removes it after normal return; panic cleanup is not guaranteed. Equal-key fix order matters. OLDEST_SUPPORTED=-1 is not proof every old world is recoverable.
- RustChunk relevance: Useful only if older-save import/upgrade is required; not needed for connecting or loading a current 1.20.1 world.
- 1.20.1 implications: 1.20.1 is the target schema, not 26.3. Any migration must implement the chain to DataVersion 3465 with correct schemas; never apply 26.3 component/registry transformations to 1.20.1 saves.
- Decision: Reference for migration architecture only; current 26.3 fix pipeline is unsuitable for base and must be independently versioned if legacy imports are required.
- Dependencies / cross-references: schema; dynamic NBT helpers; block_state_data/chunk_flatten/chunk_leaves; fix modules; items; files; world_upgrader; NBT Tag; DATA_VERSION.
- Follow-up checks: Define import version matrix; obtain exact 1.20.1 DFU chain; test unknown-field preservation/idempotence and panic/I/O recovery; separate options migrations from world data.
- RustChunk implementation mapping: Not needed in live client path; future offline world/data migration tool only.
- Evidence required before adoption: Real source/target fixtures, exact 1.20.1 DFU chain evidence, unknown-tag round trips and interrupted-migration recovery.
- Completion state: Complete source review; no migration execution or fault-injection test was performed.

### 131. `crates/horizon-world/src/datafix/block_state_data.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 9d5f578abeee550ea0d95876015760a4ad41bd5e.
- Size: 200 lines; complete source file fetched and reviewed.
- Coverage: lines 1–200, complete; no gaps between ranges.
- Purpose: Generated lookup table converting pre-1.13 legacy block IDs/data into flattened block-state identities during DataFixer migrations.
- Implementation / logic: Models name plus sorted properties and NBT tags. A OnceLock loads block_state_data.txt, recording state instances, a 4096-entry legacy ID/data map, defaults and lookups from historical name/property forms to flattened states. Helpers expose legacy tag/name/ID conversions.
- Findings: Historical migration data, not the 1.20.1 runtime registry. ID range, order and fallbacks are specific to pre-1.13 conversion. include_str creates a compile-time dependency on generated data. Invalid/out-of-range IDs have different fallback behavior by helper. No local tests are defined.
- RustChunk relevance: Low for first milestone; relevant only if importing very old worlds.
- 1.20.1 implications: 1.20.1 already uses flattened states. Do not use this table for runtime registry IDs, packets, palettes or meshing.
- Decision: Unnecessary for 1.20.1 base; retain only as historical migration reference if pre-1.13 import becomes a requirement.
- Dependencies / cross-references: block_state_data.txt; generator script; chunk_flatten.rs; DataFixer schema; NBT; legacy 1.12 arrays.
- Follow-up checks: Decide if pre-1.13 import is in scope; review table provenance/license and generator; if adopted test defaults and invalid-ID fallbacks.
- RustChunk implementation mapping: No live-client use; possible dependency of an offline legacy-save converter.
- Evidence required before adoption: Explicit pre-1.13 import requirement, verified data provenance/license and representative migration fixtures.
- Completion state: Complete source review; no generator or migration test was run.

### 132. `crates/horizon-world/src/datafix/chunk_flatten.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 671fc9f73231ce256a2ce49c623605666d1932e2.
- Size: 772 lines; complete source file fetched and reviewed.
- Coverage: lines 1–772 in consecutive ranges 1–250, 251–500 and 501–772, complete; no gaps between ranges.
- Purpose: Historical ChunkPalettedStorageFix upgrading 1.12-era chunk arrays into flattened palettes and migrating block-entity state into block states.
- Implementation / logic: Builds legacy state tables from block_state_data, adds lookup maps for doors, beds, banners, skulls, pots and note blocks, reads old Blocks bytes plus Data/Add nibbles, reconstructs numeric ID/data, applies neighbor/virtual-block fixes, converts each 4096-cell section to a palette and packed BlockStates long array, removes legacy arrays and writes UpgradeData.
- Findings: Specialized ancient-world migration, not a current chunk decoder. It assumes historical section range/count and legacy IDs; mappings/order depend on generated 1.12/1.13 data. Not a 1.20.1 palette parser. No test module is present.
- RustChunk relevance: Very low for first milestone; migration case study only.
- 1.20.1 implications: 1.20.1 chunks are already palette-based with dimension-dependent vertical sections; legacy arrays do not describe current packets or saves.
- Decision: Unnecessary for 1.20.1 runtime; keep only if pre-1.13 world import becomes explicit.
- Dependencies / cross-references: block_state_data.rs; generated table; DataFixer schema/fixes; NBT helpers; legacy 1.12 arrays and TileEntities.
- Follow-up checks: Decide legacy import matrix; if required compare with vanilla ChunkPalettedStorageFix and test special mappings/output.
- RustChunk implementation mapping: No role in chunk decode/store/renderer; standalone legacy-save import only.
- Evidence required before adoption: Agreed legacy-save matrix, oracle migration fixtures, special mapping tests and safe new-copy output.
- Completion state: Complete source review; no migration test/build/conversion was run.

### 133. `crates/horizon-world/src/datafix/world_upgrader.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 45e4545fbbd9d252c0d116842a4aa8544399a092.
- Size: 598 lines; complete source file fetched and reviewed.
- Coverage: lines 1–598 in consecutive ranges 1–300 and 301–598, complete; no gaps between ranges.
- Purpose: Background Optimize World worker scanning region/entity/POI files across dimensions and optionally rewriting chunks/caches.
- Implementation / logic: Enumerates vanilla/custom dimensions, locates region files and chunk positions; processes entity, POI and chunk data; reports progress/cancellation; applies DataFixer and tag modifiers; rewrites changed records using Zlib. Recreate mode writes to sibling new_ folders and replaces originals only after successful conversion. Optional cache erasure removes heightmaps, light-on flag and section light arrays. Worker thread catches panics and updates status.
- Findings: Recreate mode avoids replacing originals until region conversion succeeds, but remove/rename failures need recovery tests. Non-recreate mode can rewrite earlier chunks before a later failure. Thread spawn errors are discarded with .ok(), potentially leaving progress with no worker. Unreadable regions are logged/skipped. Fixer/current version is 26.3-specific.
- RustChunk relevance: Optional future world-maintenance tool, not needed for current-world loading or multiplayer.
- 1.20.1 implications: Never run this 26.3 upgrader on RustChunk 1.20.1 worlds. Any 1.20.1 upgrader needs its own source/target matrix. First milestone should load/save same-version data without forward conversion.
- Decision: Unnecessary for first 1.20.1 milestone; transactional background conversion is a future-tool pattern only.
- Dependencies / cross-references: datafix/mod.rs/files.rs/progress.rs; RegionFile/ChunkRecord; compression/NBT; dimension/generator metadata; Optimize World UI.
- Follow-up checks: Review world_upgrader_tests.rs/files.rs; fault-inject temp write/sync/remove/rename; report spawn failure; define cancellation/backup/recovery; only add if older-save import is in scope.
- RustChunk implementation mapping: No live world-loop role; future world/tools/optimize_world job only.
- Evidence required before adoption: 1.20.1 save fixtures, crash-safe region replacement tests, cancellation/thread-start failure tests and a user-visible recovery policy.
- Completion state: Complete source review; referenced tests were not run; no world upgrade or filesystem fault injection was performed.

### 134. `crates/horizon-world/src/datafix/tests.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 99d813dd216dac35908374ba1696fc47f7569a0f.
- Size: 81 lines; complete source file fetched and reviewed.
- Coverage: lines 1–81, complete; no gaps between ranges.
- Purpose: Representative DataFixer tests for no-op current data and selected migrations from 1.20-era saves into newer 26.3 schema.
- Implementation / logic: Parses SNBT and checks current chunk data unchanged, 1.20 ItemStacks converted into Data Components, legacy Level fields moved into newer difficulty/game-rule/spawn structures, and historical block-state tags rewritten. The ItemStack fixture uses DataVersion 3465.
- Findings: Tests document migrations to 26.3, not correctness of the live 1.20.1 serializer. Component output is unsuitable for 1.20.1 runtime. The limited cases do not prove full DFU parity. Tests were read but not run.
- RustChunk relevance: Useful test-design pattern: compact SNBT fixtures and assertions over nested fields make migrations auditable.
- 1.20.1 implications: Preserve 1.20.1 ItemStack/Level structures. Only an optional migration tool should apply a chain whose target is 1.20.1.
- Decision: Reference-only for migration-test structure; expected 26.3 output is not a 1.20.1 oracle.
- Dependencies / cross-references: datafix/mod.rs; dynamic helpers; SNBT parser; DataVersion 3465 fixture; ItemStack/Level/block-state fixes.
- Follow-up checks: Create no-op round-trip fixtures from genuine 1.20.1 worlds; compare migration outputs with target-version vanilla DFU.
- RustChunk implementation mapping: Methodology for future world/datafix_1_20_1 tests only; not live runtime conversion.
- Evidence required before adoption: Canonical 1.20.1 fixture files with hashes, no-op round-trip tests and target-version migration golden outputs.
- Completion state: Complete test-source review; tests were read but not run; no migration oracle was executed.

### 135. `crates/horizon-world/examples/probe/datafix_world.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: f2e119d42e9d911e8993ddf1fd0162a766c412a2.
- Size: 55 lines; complete source file fetched and reviewed.
- Coverage: lines 1–55, complete; no gaps between ranges.
- Purpose: CLI parity probe copying a world folder and running DataFixer conversion on the copy.
- Implementation / logic: Accepts input world, output world and optional target DataVersion. Removes an existing output directory recursively, copies input while skipping session.lock, then calls upgrade_band for a target or upgrade_world for current conversion. Prints resulting version or failure/usage code.
- Findings: Test tool, not runtime loader. Removing an existing output path is unsafe for user-facing defaults; caller must guarantee it is disposable and distinct from source. Copy is recursive but not an atomic directory transaction and has no backup recovery. Default target is 26.3.
- RustChunk relevance: Low shipped-client value; useful as offline migration regression harness.
- 1.20.1 implications: Do not run against 1.20.1 saves unless target/fixer chain are specifically 1.20.1. Normal opening should not rewrite data to 26.3.
- Decision: Unnecessary in production; adaptable as development-only runner with safer destination safeguards.
- Dependencies / cross-references: horizon-world datafix/files; upgrade_band/upgrade_world; NBT; parity scripts; world fixtures.
- Follow-up checks: Inspect upgrade_band; reject source/destination equality or ancestry; use temp output and safe publish; preserve fixture copies.
- RustChunk implementation mapping: Possible host-side world_migration_probe only; never auto-run on world open.
- Evidence required before adoption: Source overwrite protection tests, repeatability on copied 1.20.1 fixtures and semantic oracle comparison.
- Completion state: Complete source review; executable was not run and no world folder was copied or changed.

### 136. `crates/horizon-client/examples/probe/dispatch.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 8754e944d96be5f133bfcd522cb715c79e3a562c.
- Size: 50 lines; complete source file fetched and reviewed.
- Coverage: lines 1–50, complete; no gaps between ranges.
- Purpose: Shared dispatcher and argument adapter for the single client parity-probe binary.
- Implementation / logic: A probes macro generates name-to-module dispatch and main; invokes the chosen probe, converts Termination to ExitCode and strips the probe name so each module sees an individual-example interface. Provides String and OsString iterators.
- Findings: Test harness only. Argument removal assumes probe name is argv index 1 and invocation goes through the dispatcher. Single-binary compilation may reduce repeated linking, but actual tests/expected values live in child probes. No probe was run.
- RustChunk relevance: Useful if many integration/parity examples make separate client binaries expensive to link.
- 1.20.1 implications: Dispatcher is reusable, but probe logic and expected results must target 1.20.1.
- Decision: Adaptable as development-only test harness utility; no production dependency.
- Dependencies / cross-references: client probe main.rs; probe modules; ExitCode/Termination; parity scripts/oracle harness.
- Follow-up checks: Verify Cargo registration and APK exclusion; test empty/unknown names and OsString paths; version-label fixtures.
- RustChunk implementation mapping: Candidate for tools/client_probe in workspace, not Android client binary.
- Evidence required before adoption: CI probe on 1.20.1 fixtures and build-time comparison showing a single-binary benefit.
- Completion state: Complete source review; no probe build, invocation or parity run was performed.

### 137. `crates/horizon-world/examples/probe/dispatch.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 80b875eef798f920c989730c36bb382c30321d8f.
- Size: 41 lines; complete source file fetched and reviewed.
- Coverage: lines 1–41, complete; no gaps between ranges.
- Purpose: Shared dispatcher for the horizon-world parity-probe binary.
- Implementation / logic: A probes macro generates module dispatch and main, strips probe name from arguments, and prints usage/exit code 2 for unknown names. Exposes String arguments only, unlike client dispatcher which also exposes OsString.
- Findings: Harness plumbing only; it does not prove world-generation, region or NBT tests passed. String-only paths may be insufficient for non-UTF8 filesystem names.
- RustChunk relevance: Useful if many world/NBT/chunk fixtures need one compiled probe.
- 1.20.1 implications: Dispatcher is version-independent, but probes/fixtures must target 1.20.1 and stay out of release APK.
- Decision: Adaptable as development/test harness only; not shipping runtime.
- Dependencies / cross-references: world probe main/modules; Cargo example; CI/parity scripts.
- Follow-up checks: Test no/unknown args, non-UTF8 paths, exit-code propagation and release-target exclusion.
- RustChunk implementation mapping: Candidate for host-side world_probe Cargo example; never game-loop invoked.
- Evidence required before adoption: CI for representative 1.20.1 NBT/chunk/worldgen probes and proof it is not linked into Android release.
- Completion state: Complete source review; no probe executable or tests were run.

### 138. `crates/horizon-world/examples/probe/chunk_dump.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 11725bc709ed1d79d613e66c575c6be593e3e9a8.
- Size: 506 lines; complete source file fetched and reviewed.
- Coverage: lines 1–506 in consecutive ranges 1–250 and 251–506, complete; no gaps between ranges.
- Purpose: Host-side whole-chunk world-generation parity dumper for blocks, quart biomes, block entities, scheduled ticks and postprocessing metadata.
- Implementation / logic: Loads registry from facts.json, canonicalizes NBT, loads folder datapacks, optionally synthesizes old chunks for blending, then creates an OverworldGenerator from seed/dimension. Environment options select decoration order, terrain/structures/full phases, structure scans/pieces, saved starts, datapacks, old chunks and isolated decoration. It generates target rectangle plus margin/neighborhood, decorates in controlled order and emits stable dumps of state IDs, quart biomes, block rows, block-entity NBT, block/fluid ticks and postprocessing marks.
- Findings: Strong oracle method because it compares more than screenshots. This is not live renderer or network chunk decoder. registry() leaks a Registry Box for process lifetime; acceptable for short probe but not library code. Canonical output casts float/double values to integers, potentially losing fractions. Datapack/saved-start behavior is probe-specific. No probe was run.
- RustChunk relevance: High only if local/integrated world generation is implemented; multiplayer-first chunk decode/render is higher priority.
- 1.20.1 implications: Retarget resources, registries, worldgen, RNG, dimensions and expected output to 1.20.1; do not reuse 26.3 facts/dumps. Test local generation separately from server-delivered chunks.
- Decision: Adaptable as host-side worldgen parity tool; current generator/resources/fixtures are 26.3-specific.
- Dependencies / cross-references: horizon-world generator; horizon-core position/random; BlockRegistry; facts.json/worldgen resources; BlendingData; datapack overrides; ProtoChunk; oracle scripts.
- Follow-up checks: Replace lossy float/double canonicalization; test resource pinning/order-sensitive decoration; create 1.20.1 seed/dimension fixtures; separate generated-chunk from network chunk tests.
- RustChunk implementation mapping: Development-only worldgen_probe; no first-milestone multiplayer role.
- Evidence required before adoption: Matching 1.20.1 dumps across seeds/dimensions, decoration-order fixtures, block-entity/tick parity and CI with pinned resources.
- Completion state: Complete source review; no generator execution, oracle comparison or benchmark was performed.

### 139. `crates/horizon-core/src/no_ai.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: 18b3de0f2a11da736931e93c9a327490a68b20b9.
- Size: 98 lines; complete source file fetched and reviewed.
- Coverage: lines 1–98, complete; no gaps between ranges.
- Purpose: Persistent opt-out switch disabling Horizon AI/agent-control surfaces across game and launcher.
- Implementation / logic: Reads noAi:true from horizon-extras.txt beside options.txt; missing file means off. with() rewrites only this key while retaining other lines. set() writes when enabling, does not create an absent file when disabling and propagates write errors. check() returns a fixed refusal when enabled. Tests cover parsing, line preservation and default-off behavior.
- Findings: Product/agent safety control only, unrelated to Minecraft gameplay/protocol. Simple line parser and direct fs::write rather than atomic replacement; interruption can truncate settings. is_on treats unreadable/missing data as off, so read failure is not fail-closed. Tests read but not run.
- RustChunk relevance: Low for initial client; relevant only if RustChunk later has agent-control or remote automation.
- 1.20.1 implications: No effect on vanilla compatibility, chunk rendering or Forge. Do not import merely because Starling has it.
- Decision: Unnecessary for vanilla 1.20.1 base; retain as reference only if developer/agent control is introduced.
- Dependencies / cross-references: horizon-extras.txt; launcher settings; horizon-ctl/MCP; agent-control docs.
- Follow-up checks: No port now; future automation needs capability scopes, authentication, per-action confirmation and atomic settings.
- RustChunk implementation mapping: No shipping game module; possible future devtools preference only.
- Evidence required before adoption: Explicit agent-control requirement, security review, cross-process consistency and fail-closed tests.
- Completion state: Complete source review; tests read but not run; no settings or automation interface were changed.

### 140. `crates/horizon-core/src/random/worldgen.rs`
- Source repository/ref: starling-client/starling / starling.
- Source SHA: aa16ab2492e194ef9b2d0dd0c76b0a199a1c25f8.
- Size: 204 lines; complete source file fetched and reviewed.
- Coverage: lines 1–204, complete; no gaps between ranges.
- Purpose: Adapts legacy and Xoroshiro random sources to Minecraft world-generation bit-sampling and call-count semantics.
- Implementation / logic: A sealed WorldgenSource exposes version-specific bit extraction for LegacyRandom and XoroshiroRandom. WorldgenRandom supports wrapped Java-style nextBits/nextInt(bound)/nextLong/nextFloat/nextDouble/boolean/Gaussian with call counting, or plain mode delegating to the source for fork/positional streams. It implements decoration, feature, large-feature and salted-large-feature seed formulas; a positional adapter wraps streams and slime_chunk_random handles legacy slime seed mixing.
- Findings: Deterministic world generation depends on exact RNG, call sequence, bit extraction, overflow, fork semantics and Gaussian cache. set_seed does not clear the adapter Gaussian cache (documented as matching native behavior); plain mode delegates methods rather than applying wrapper behavior, so wrong wrapping changes output. No local test module; parity tests live elsewhere and were not run.
- RustChunk relevance: Critical only if RustChunk generates terrain/features locally. Multiplayer-first client does not need it to receive/render server chunks.
- 1.20.1 implications: Verify exact 1.20.1 LegacyRandom/Xoroshiro selection, seed formulas, positional forks, call counts and Gaussian cache. Do not assume 26.3 call sites/constants apply.
- Decision: Adaptable RNG architecture; exact source selection and worldgen semantics need 1.20.1 validation/versioning.
- Dependencies / cross-references: horizon-core random LegacyRandom/XoroshiroRandom/PositionalRandom; BlockPos; biome/noise/carver/structure/feature/decorate; chunk_dump probe.
- Follow-up checks: Trace RNG use in 1.20.1 worldgen; add known-answer tests for bounds, forks, negative coordinates, Gaussian cache and call counts; compare chunk dumps across dimensions/seeds.
- RustChunk implementation mapping: Not needed for remote multiplayer-only first milestone; candidate for worldgen/random if integrated/local generation is added later.
- Evidence required before adoption: 1.20.1 oracle vectors for each RNG entry point and matching biome/terrain/structure/decoration output over positive/negative coordinates and multiple seeds.
- Completion state: Complete source review; no RNG vectors, worldgen oracle, benchmark or build was executed.

## Batch checkpoint — entries 121–140 — 2026-11-18

- Completed full source reviews for entries 121–140. All 20 source paths and blob SHAs were obtained from the pinned Starling source snapshot.
- Long files were read in consecutive ranges without gaps: Android bridge (1,386), diagnostics (606), mesher (2,028), world state (634), block-entity data (755), DataFixer orchestration (386), legacy chunk flattening (772), world upgrader (598) and chunk dump probe (506).
- Every entry includes an explicit RustChunk implementation mapping and evidence-required-before-adoption field, separating production client needs from optional migration tools, product tooling and test-only probes.
- Extended AUDIT_INDEX.md to 140 sequential rows in six sections of 25, 25, 25, 25, 25 and 15 entries.
- Main architectural leads: Android platform bridge, bounded/asynchronous diagnostics, chunk membership deltas, worker-oriented section meshing/lighting, resident-world invalidation, typed block-entity data and bounded region compression.
- Explicit exclusions from vanilla 1.20.1 first milestone: Starling 26.3 DataFixer output, pre-1.13 block-state flattening, Optimize World migration UI, Starling no-AI/agent switch, Horizon native-mod block entities and Starling product-specific native-mod loading.
- Tests present in source were read but not executed. No Cargo build, RustChunk code change, Android APK/device test, shader/render run, migration, worldgen probe or live Minecraft connection was performed.

### 141. `.claude/skills/vanilla-parity/SKILL.md`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 4c0714d92320e0f36507b7d51855ec74f33ab5c3.
- Size: 77 lines.
- Coverage: lines 1–77, complete.
- Purpose: Agent procedure for diagnosing vanilla-parity reports through a generated port ledger and tracing vanilla methods to Rust implementations.
- Implementation / logic: Rebuilds the ledger from the vanilla 26.3 jar/decompiled lab; uses issue/where/class/what/gaps commands; requires reading both vanilla and Rust functions because the generated map may be wrong; preserves operation order, constants, RNG draws and Java math; distinguishes integrated-server logic from client-only effects; runs ledger/map checks and area tests before PR.
- Findings: Strong review method, especially the warning that generated mapping is a lead rather than proof and the need to check both execution paths. The jar path, tools, method map and target version are Horizon 26.3-specific. Its stated mapping error rate is a claim in this document, not independently measured here.
- RustChunk relevance: High as a parity-review process; the existing mapping data is not reusable evidence for 1.20.1.
- 1.20.1 implications: Rebuild the map against pinned 1.20.1 vanilla sources and RustChunk paths; do not import 26.3 verdicts or method paths. Include protocol 763 and the correct client/server seams in validation.
- Decision: Adaptable as an engineering process; rewrite all version-specific data, scripts and paths. No runtime dependency.
- Dependencies / cross-references: tools/port_ledger.py; tools/port_map/*.tsv; docs/port-ledger.md; docs/architecture.md; AGENTS.md; horizon_core random/math; vanilla jar and decompilation lab.
- Follow-up checks: Inspect the generator/schema; validate map accuracy against 1.20.1 methods; define RustChunk seam rules; attach executable parity tests to each accepted port.
- RustChunk implementation mapping: Candidate devtool and contributor workflow only; no shipped module. Replace the 26.3 commands and mappings.
- Evidence required before adoption: Hash-pinned 1.20.1 vanilla inputs; verified method-to-Rust samples; paired source evidence; client/server seam checks; repeatable 1.20.1 parity tests in CI.
- Completion state: Full source review completed; no ledger command, decompilation, RustChunk test or CI job was run.

### 142. `.dockerignore`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 458330c93359f503d2f949860f658ff2e82ca46b.
- Size: 8 lines.
- Coverage: lines 1–8, complete.
- Purpose: Excludes build output, local state, JavaScript dependencies, benchmarks, packs and mods from Docker build contexts.
- Implementation / logic: Ignores target/, .local/, node_modules directories, benchmarks/, packs/ and mods/; the comment says the relay needs Rust sources only.
- Findings: Useful context-size and privacy safeguard, but ignored paths may contain required inputs in another project. This file does not identify the Dockerfile or prove the relay has no dependency on excluded content.
- RustChunk relevance: Moderate for reducing build context and preventing local/game data from being uploaded; RustChunk's build inputs may differ.
- 1.20.1 implications: No game semantics. Confirm user-owned 1.20.1 files and test fixtures are not required inside any Docker context before copying these exclusions.
- Decision: Adaptable as a Docker hygiene pattern; rewrite the paths for RustChunk's actual build.
- Dependencies / cross-references: Dockerfile and relay build; target/; .local/; node_modules; benchmarks; packs; mods.
- Follow-up checks: Locate all Docker invocations; build from a clean context with ignored paths absent; confirm user assets are neither required nor included.
- RustChunk implementation mapping: Build/deployment tooling only; no game crate or Android runtime mapping.
- Evidence required before adoption: Successful clean Docker build and a context audit showing all required source/manifests/generated files remain available while private game data is excluded.
- Completion state: Full source review completed; no Docker image was built.

### 143. `.gitattributes`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 585f03f060b9faa24a5625f663c43497ca049f62.
- Size: 12 lines.
- Coverage: lines 1–12, complete.
- Purpose: Enforces LF line endings for byte-compared generated documentation, CSS and Rust source; marks generated Rust files for GitHub language statistics.
- Implementation / logic: Applies text/eol=lf to selected paths and patterns, and linguist-generated to generated Rust outputs.
- Findings: Prevents Windows autocrlf from creating false generated-file diffs. The exact path list does not prove all generated files are covered or that the generators are deterministic.
- RustChunk relevance: Useful for consistent generated protocol/resource tables and clean cross-platform diffs, especially on Windows.
- 1.20.1 implications: Version-independent source-control behavior; apply it to RustChunk's actual 1.20.1 generated artifacts, not Starling paths.
- Decision: Adaptable; rewrite the path list for RustChunk generators and generated outputs.
- Dependencies / cross-references: Git core.autocrlf; generated mod API files; theme tokens; command/API docs; associated generators and check modes.
- Follow-up checks: Inventory generated outputs; test generator checks on Windows and Linux; avoid marking hand-maintained source as generated.
- RustChunk implementation mapping: Repository configuration for generated protocol tables, registries and API files; no runtime mapping.
- Evidence required before adoption: Windows/Linux checkout test with identical generated bytes and passing generator check on both platforms.
- Completion state: Full source review completed; no cross-platform checkout or generator test was run.

### 144. `.githooks/pre-commit`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: abc64ac4e8daa0f4a46fa60e501dffeb5607f403.
- Size: 12 lines.
- Coverage: lines 1–12, complete.
- Purpose: Pre-commit hook that finds a working Python 3 executable and runs the public-tree checker.
- Implementation / logic: Tries python3, python, then py using a Python-version check; replaces itself with tools/check_public_tree.py on the first working interpreter; exits with an error if none is found.
- Findings: Handles the Windows Store python3 stub and multiple Python command names. It only applies when hooks are installed and enabled; hooks can be bypassed, so CI is still required. The checker behavior is outside this wrapper.
- RustChunk relevance: Useful local protection against committing private game assets, secrets and build products.
- 1.20.1 implications: The checker policy must explicitly exclude user-owned 1.20.1 binaries/assets and prepared local files.
- Decision: Adaptable as a local safeguard, not sufficient as sole enforcement.
- Dependencies / cross-references: tools/check_public_tree.py; Python 3; Git hook installation/configuration; public-tree policy.
- Follow-up checks: Inspect the checker and hook installer; test python3/python/py fallback, missing Python, executable mode and bypass behavior.
- RustChunk implementation mapping: Repository developer workflow only; optionally installed by a RustChunk setup script.
- Evidence required before adoption: Windows/Linux hook tests, rejection tests for private game data/secrets, and equivalent required CI checks.
- Completion state: Full hook-source review completed; hook and checker were not executed.

### 145. `.githooks/pre-push`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 375b535c0f6415f27b2524bf211b2e4d4b840ad4.
- Size: 12 lines.
- Coverage: lines 1–12, complete.
- Purpose: Pre-push hook that runs the public-tree checker in its pre-push mode.
- Implementation / logic: Uses the same python3/python/py fallback as pre-commit, then executes tools/check_public_tree.py with --pre-push; errors if Python 3 cannot be found.
- Findings: Adds a second local checkpoint before publishing. It shares the installation and bypass limitations of all local Git hooks; exact pre-push behavior is implemented by the checker, not this file.
- RustChunk relevance: Useful to prevent accidental publication of private Minecraft data and local build outputs.
- 1.20.1 implications: The protected-file policy should cover user-owned 1.20.1 assets and prepared resources; no protocol or gameplay behavior is present.
- Decision: Adaptable as a local pre-push safeguard; CI must independently enforce the policy.
- Dependencies / cross-references: tools/check_public_tree.py --pre-push; Python 3; Git hook setup; public-tree policy.
- Follow-up checks: Inspect the checker's pre-push branch; test allowed/rejected file classes; verify CI catches the same violations when hooks are bypassed.
- RustChunk implementation mapping: Repository workflow only; no production crate.
- Evidence required before adoption: Supported-OS hook tests and CI tests proving prohibited game files/secrets cannot be pushed through a hook bypass.
- Completion state: Full hook-source review completed; no hook or push test was executed.

### 146. `.github/actionlint.yaml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: af951fe125aacb21b2a7227da9e08d4cf61ce4f6.
- Size: 4 lines.
- Coverage: lines 1–4, complete.
- Purpose: Configures actionlint's recognized self-hosted-runner labels for a maintainer Mac mini.
- Implementation / logic: Declares the horizon-game and ludovics-mac-mini labels under the self-hosted-runner configuration key.
- Findings: Lint configuration only; it does not provision runners or secure them. Its comment says the runner holds the maintainer's Minecraft files, making runner trust and workflow routing security-sensitive.
- RustChunk relevance: Low unless RustChunk introduces self-hosted GitHub Actions runners; otherwise GitHub-hosted runners avoid this extra boundary.
- 1.20.1 implications: Never schedule untrusted fork code on a runner containing local 1.20.1 files, credentials or signing material.
- Decision: Reference-only unless a self-hosted runner is introduced; rewrite labels and security assumptions.
- Dependencies / cross-references: actionlint; workflows using runs-on labels; GitHub runner configuration and access policy.
- Follow-up checks: Validate the config with the chosen actionlint version; inspect workflow triggers and runner groups before adding private runners.
- RustChunk implementation mapping: Optional CI lint configuration only; no client code mapping.
- Evidence required before adoption: Passing actionlint validation and a threat model proving fork PRs cannot execute on runners with private game files or secrets.
- Completion state: Full configuration review completed; actionlint and runner behavior were not tested.

### 147. `.github/CODEOWNERS`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 5567eb184c61306a7e646e99bf03dd4d4c50353b.
- Size: 17 lines.
- Coverage: lines 1–17, complete.
- Purpose: Assigns a maintainer as code owner for workflow, CI/release, signing, update/download and packaging paths.
- Implementation / logic: Uses a broad .github rule and explicit paths for CI scripts, release training, signing, update server, launcher signature/backend/link files, Cargo manifests and packaging.
- Findings: A useful ownership boundary for sensitive supply-chain code. The file comment explicitly says code-owner review applies only once branch protection requires it; CODEOWNERS alone does not enforce review or show whether bypass is possible.
- RustChunk relevance: High for protecting signing, update, resource acquisition and Android packaging logic from unreviewed changes.
- 1.20.1 implications: Include version-pinned resource acquisition, auth/session, package signing, native loading and build/release automation in the RustChunk sensitive path inventory.
- Decision: Adaptable ownership pattern; rewrite paths/reviewers and enable matching protected-branch rules.
- Dependencies / cross-references: GitHub CODEOWNERS syntax; branch protection/rulesets; tools/ci/protect_pipeline.sh; docs/security.md; release/updater paths.
- Follow-up checks: Audit coverage of every sensitive RustChunk path; verify reviewer access; inspect branch rules, required approvals and administrator bypass.
- RustChunk implementation mapping: Repository governance for resource verification, auth, release, Android packaging and future native-extension boundaries; no runtime dependency.
- Evidence required before adoption: Active protected-branch policy, a test PR requiring owner approval, complete sensitive-path coverage and reviewer succession plan.
- Completion state: Full CODEOWNERS review completed; branch-protection enforcement was not inspected.

### 148. `.github/ISSUE_TEMPLATE/bug_report.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 1c0aa78a7cc60a1ff30d62c69e361bc56d7db834.
- Size: 54 lines.
- Coverage: lines 1–54, complete.
- Purpose: Structured player bug report capturing reproduction, phone model, Android/version context, feature area and logs.
- Implementation / logic: Requires the report description and phone model; offers Android and Starling version inputs, a dropdown for setup/singleplayer/server/shaders/resource packs/mods/touch controls, and log attachment guidance for latest, previous and rotated logs.
- Findings: Device and log context helps reproduce mobile problems. Android/version fields are optional, and logs can contain account, server or filesystem information; the form has no redaction/privacy warning of its own.
- RustChunk relevance: High for reproducing GPU-, device-, chunk- and input-related failures on Android.
- 1.20.1 implications: Ask for RustChunk build and Minecraft 1.20.1 environment separately; request mod-loader/mod-list details only when relevant to the later milestone.
- Decision: Adaptable issue-intake pattern; rewrite labels, log path, supported feature list and privacy guidance.
- Dependencies / cross-references: GitHub issue forms; Android log location/rotation; diagnostics redaction/export; release version.
- Follow-up checks: Verify actual log path and rotation; inspect redaction; test required/optional form behavior and attachment privacy.
- RustChunk implementation mapping: Repository bug-triage form mapped to device, renderer, chunk, network and Android issue categories; no runtime module.
- Evidence required before adoption: Test submissions, confirmed log path, verified redaction/retention and privacy review for session/server data.
- Completion state: Full form-source review completed; no live issue or log-redaction test was performed.

### 149. `.github/ISSUE_TEMPLATE/config.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: c24e7816f4876a8d172ddf0645e97ee50e1e472f.
- Size: 6 lines.
- Coverage: lines 1–6, complete.
- Purpose: Keeps blank issues enabled and routes desktop/shared-game issues to Horizon's repository.
- Implementation / logic: Sets blank_issues_enabled to true and defines a single contact link with its destination and explanation.
- Findings: Simple issue routing can separate Android-specific and shared-game ownership, but the Horizon URL is Starling-specific and should not be copied into RustChunk without a real relationship.
- RustChunk relevance: Useful only if RustChunk later separates core and Android issue ownership; otherwise one tracker is simpler.
- 1.20.1 implications: Route issues to the team actually responsible for the 1.20.1 client; no game-version behavior is implemented here.
- Decision: Adaptable issue-routing pattern; existing destination is not reusable as-is.
- Dependencies / cross-references: GitHub issue-form config; shared core/Android ownership; external issue tracker.
- Follow-up checks: Decide whether multiple trackers are needed; verify destinations and preserve a private route for security reports.
- RustChunk implementation mapping: GitHub repository configuration only; no runtime code mapping.
- Evidence required before adoption: Confirmed ownership split, working destination URLs and an end-to-end report-routing test.
- Completion state: Full configuration review completed; no live routing test was performed.

### 150. `.github/ISSUE_TEMPLATE/native-build.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: f98b500a428848fb48516388fee38e4f6ba71f9e.
- Size: 25 lines.
- Coverage: lines 1–25, complete.
- Purpose: Issue form requesting a Starling native-mod build for a specified Starling version and list of native-mod identifiers.
- Implementation / logic: Provides an explanation that a submitted request starts a build, then requires a version and space-separated mod IDs; a separate workflow consumes the fields.
- Findings: The form is only an input surface; it does not validate identifiers, authorize code or establish trust. The format and workflow are Starling-native-mod-specific.
- RustChunk relevance: None for the vanilla 1.20.1 first milestone. It must not be mistaken for Forge mod support.
- 1.20.1 implications: Forge is deferred; this form does not implement Forge or Java mod compatibility.
- Decision: Unnecessary for the current vanilla client; future reference only if an independently designed native-extension service is approved.
- Dependencies / cross-references: .github/workflows/native-mods.yml; native-mods/; docs/starling-native-mods.md; GitHub issue forms.
- Follow-up checks: No adoption now. For any future extension service, inspect trust, sandboxing, supported ABI/version matrix and artifact verification.
- RustChunk implementation mapping: No current runtime/UI mapping; possible future developer request interface only after an extension system is designed.
- Evidence required before adoption: Approved threat model, vetted source allowlist, isolated reproducible builds, signed artifacts, compatibility matrix and explicit user consent.
- Completion state: Full form-source review completed; no issue was submitted or build requested.

### 151. `.github/pull_request_template.md`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 8ca6fe2f50fbb91faccd29f7ff83bc090d800d92.
- Size: 40 lines.
- Coverage: lines 1–40, complete.
- Purpose: PR checklist for rationale, change type, vanilla reference, external code provenance, self-check evidence and screenshots.
- Implementation / logic: Prompts for parity/bug/performance/feature/tooling/docs classification; asks for vanilla classes/methods and harness, exact external repository commit and license details, pr_ready output and before/after screenshots where visible. It also asks AI agents to disclose authorship and requests maintainer edit permission.
- Findings: Good review and provenance prompts, but checkboxes are not enforcement. The 26.3 parity target and script names are version-specific, and a self-check block can be asserted without CI validation.
- RustChunk relevance: High for traceable changes, license review, test evidence and visual regression.
- 1.20.1 implications: Replace 26.3 references with a pinned 1.20.1 oracle; Forge-specific labels should wait for the deferred Forge phase.
- Decision: Adaptable PR process; rewrite version-specific fields and make required checks enforceable in CI.
- Dependencies / cross-references: CONTRIBUTING.md; tools/pr_ready.py; tools/rea_vanilla.py; parity harness; license allowlist; GitHub labels.
- Follow-up checks: Inspect PR-ready script and label automation; determine which fields are actually validated; review fork PR and screenshot privacy behavior.
- RustChunk implementation mapping: Repository review workflow covering protocol, world, renderer, Android, assets and future mod code; no runtime module.
- Evidence required before adoption: A test PR proving version-specific references, provenance and CI results are validated rather than merely requested.
- Completion state: Full template review completed; no PR or self-check workflow was executed.

### 152. `.github/release-line.txt`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 3b04cfb60da13a716867848ebeb2191a164887d9.
- Size: 2 lines.
- Coverage: lines 1–2, complete.
- Purpose: Stores a single release-line value, currently 0.2.
- Implementation / logic: Plain-text marker with one value; this file contains no parser or definition of the value's meaning.
- Findings: The value alone does not show whether it means an application release line, compatibility channel, protocol generation or another selector. Assigning semantics without tracing consumers would be guesswork.
- RustChunk relevance: Potentially useful only if RustChunk has a separate release-line concept; it must not be confused with Minecraft 1.20.1.
- 1.20.1 implications: No demonstrated game-version meaning. Keep Minecraft version, client release and release channel separate if all are needed.
- Decision: Undetermined pending consumer tracing; do not copy the value yet.
- Dependencies / cross-references: Any release/build scripts, workflow expressions, update metadata or docs that read this path; none is established by this file alone.
- Follow-up checks: Search the pinned tree for references to this path; inspect parser, validation, release docs and migration behavior.
- RustChunk implementation mapping: No module assigned; possible release automation metadata only if an actual consumer and requirement are found.
- Evidence required before adoption: Identified consumers, documented semantics, validation/default behavior and a release-pipeline test showing the effect of changing the value.
- Completion state: Full file review completed; consumer tracing and release automation were not run.

### 153. `.github/trusted-reviewers.txt`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 7513add1f64b451e1089b40c82f37608756d04d2.
- Size: 11 lines.
- Coverage: lines 1–11, complete.
- Purpose: Allowlist of GitHub logins for nightly-hold selection and issue/PR triage commands.
- Implementation / logic: Lists one login per line with comments recording who added it and when; comments define a paused marker to suspend listed reviewers while keeping the maintainer's own commands available.
- Findings: A simple revocable allowlist, but its security depends on consuming scripts correctly enforcing membership and on protected review for changes. This file does not implement identity verification, parsing, authorization or audit logging.
- RustChunk relevance: Only relevant if RustChunk adds repository moderation commands; it is not a Minecraft player/account allowlist.
- 1.20.1 implications: No gameplay or protocol impact. Keep automation identities distinct from Minecraft accounts and server operator permissions.
- Decision: Reference-only for possible future repository moderation tooling; no client need.
- Dependencies / cross-references: docs/release-flow.md; tools/ci/nightly_set.py; tools/ci/triage_command.py; CODEOWNERS; GitHub issue/PR permission scopes.
- Follow-up checks: Inspect the consumer scripts for exact matching, bot handling, pause/revoke behavior and audit output; verify this file is owner-protected.
- RustChunk implementation mapping: Optional repository automation config only; never part of Android app or game login.
- Evidence required before adoption: Allow/deny/paused/revoked identity tests, least-privilege tokens, protected changes and auditable command logs.
- Completion state: Full allowlist review completed; consumer scripts and GitHub permission enforcement were not run.

### 154. `.github/workflows/cancel-on-close.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 28343115f3580a9d63a0363bdc6ca5a6b37a6530.
- Size: 32 lines.
- Coverage: lines 1–32, complete.
- Purpose: Cancels queued or running workflow runs for a pull request after it is closed or merged.
- Implementation / logic: Uses pull_request_target on closed events with actions:write; checks out no source and runs no PR code. It queries unfinished statuses, filters by branch and head repository, de-duplicates run IDs and best-effort cancels them.
- Findings: Avoiding checkout/execution is an important pull_request_target mitigation. Matching both branch and head repository avoids collisions from same-named branches in different forks. The API request asks for at most 100 runs per status, so unusually large result sets may be incomplete.
- RustChunk relevance: Useful for freeing CI capacity from stale PR jobs, especially expensive Android/parity runs.
- 1.20.1 implications: No game semantics; ensure cancellation targets obsolete 1.20.1 jobs without affecting unrelated workflows or branches.
- Decision: Adaptable CI-cost control, subject to security and pagination validation.
- Dependencies / cross-references: GitHub pull_request_target; Actions API; gh CLI; pull-request head ref/repository; actions:write permission.
- Follow-up checks: Review expression/shell handling and pagination; test same-repo and fork PRs with colliding branch names; confirm cancellation scope.
- RustChunk implementation mapping: GitHub CI automation only; no shipped game code.
- Evidence required before adoption: Threat-model review, least-privilege permission confirmation, fork/collision integration tests and proof only intended PR runs are cancelled.
- Completion state: Full workflow-source review completed; no workflow dispatch or cancellation was performed.

### 155. `.github/workflows/ci.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 84c4414e59f859b11671ab8cbc0f69a39de2e1b4.
- Size: 97 lines.
- Coverage: lines 1–97, complete.
- Purpose: General CI for selected branch pushes and PRs, covering public-tree hygiene, Python research tooling, Rust formatting, lint and core tests.
- Implementation / logic: Triggers on selected branches, PR changes and manual dispatch; cancels superseded PR runs; disables telemetry in tests; selects hosted or restricted self-hosted runners; detects Rust changes; checks public-tree files and PR ranges; runs Python tests, installs Rust plus system headers, caches dependencies, then runs fmt, Clippy and workspace library/binary/integration/doc tests.
- Findings: Strong layered CI with private-game-file protection and fork PR isolation from self-hosted runners. Rust checks may be skipped when change detection says no Rust changed, while Python and public-tree checks still run. This workflow does not itself run vanilla parity, Android APK release builds or device tests. Action references use version tags, not immutable commit SHAs.
- RustChunk relevance: Very high as a base CI pattern: change detection, reproducible lint/test gates, private-data exclusion and split release/device testing.
- 1.20.1 implications: Rewrite branch/path triggers and add 1.20.1 protocol, NBT, chunks, renderer and server-interoperability parity gates. Passing general CI is not proof of 1.20.1 compatibility.
- Decision: Adaptable as a CI foundation; rewrite triggers, scripts, runner policy, caches and version-specific test coverage.
- Dependencies / cross-references: tools/ci/rust_changes.py; tools/check_public_tree.py; tests/; Cargo workspace; rustfmt; Clippy; Rust tests; telemetry config; Android and parity workflows.
- Follow-up checks: Inspect change detection for missed inputs; verify other workflows run parity and APK/device tests; review action pinning and self-hosted runner protections; verify telemetry stays disabled in tests.
- RustChunk implementation mapping: Workspace CI for host-side 1.20.1 protocol/world/render tests; keep Android APK and physical-device tests in separate jobs.
- Evidence required before adoption: CI tests for Rust and non-Rust changes; fork PR isolation; private-file rejection; passing 1.20.1 fixtures/parity; Android build and device gates for release candidates.
- Completion state: Full workflow-source review completed; no workflow run, Cargo command, CI check or APK build was performed.

### 156. `.github/workflows/claim-command.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 9c6ddfc290dbd8682a9b3f93fddfdb74bfcf9b73.
- Size: 81 lines.
- Coverage: lines 1–81, complete.
- Purpose: Handles /claim and /unclaim issue comments and periodically releases old claims without a linked open PR.
- Implementation / logic: Triggers on issue comments, schedule and manual dispatch; restricts command execution to the canonical Horizon repo; ignores bot comments; serializes by issue number; sparsely checks out main scripts/list; passes user comment data through environment variables; grants issue-write permission for claim/release operations.
- Findings: Runs trusted main-branch scripts rather than PR code and avoids interpolating comment text into shell source. Actual command parser, reviewer authorization, linked-PR detection and stale-age logic live in Python scripts not reviewed here. Prefix filtering in YAML is not proof of secure parsing.
- RustChunk relevance: Low for the client; optional contributor coordination only if issue traffic warrants it. It must not affect in-game permissions.
- 1.20.1 implications: No game behavior. Keep repository moderation separate from Minecraft authentication and server operator permissions.
- Decision: Unnecessary for the vanilla client; possible future repository automation after script review.
- Dependencies / cross-references: tools/ci/claim_command.py; tools/ci/nightly_set.py; .github/trusted-reviewers.txt; GitHub issue-comment/schedule events and issue APIs.
- Follow-up checks: Inspect parser and authorization; test malformed prefixes, bots, forks, duplicate claims, stale claims with/without PRs and API rate limits.
- RustChunk implementation mapping: No shipping module; optional GitHub contributor workflow only.
- Evidence required before adoption: Parser/authorization tests, minimum token scopes, audit trail, abuse/rate-limit handling and protection from releasing active claims.
- Completion state: Full workflow-source review completed; called scripts and live GitHub behavior were not run.

### 157. `.github/workflows/mod-registry.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 0f252bb68206c90800fffe8a74b167c8492915de.
- Size: 35 lines.
- Coverage: lines 1–35, complete.
- Purpose: Dedicated CI for the Horizon mod-registry workspace when its crates or shared mod checks change.
- Implementation / logic: Runs on main pushes and PRs touching the mod registry, shared mods crate or workflow; installs stable Rust with fmt/Clippy; caches the registry workspace; runs locked formatting, lint and tests.
- Findings: Isolating a small workspace reduces unrelated CI work, but path filters must include all inputs that affect it. This does not build native mods or prove Minecraft mod compatibility.
- RustChunk relevance: Potentially useful if RustChunk later has a separate resource/mod metadata service; not required for the vanilla client.
- 1.20.1 implications: A future registry must distinguish data packs, resource packs, Forge mods and any Rust extension format, and identify exact 1.20.1 compatibility. This workflow establishes none of those.
- Decision: Adaptable CI isolation pattern; the current mod-registry feature is unnecessary for the first milestone.
- Dependencies / cross-references: crates/horizon-mod-registry; crates/horizon-mods; Cargo.lock; rustfmt; Clippy; test suite; GitHub path filters.
- Follow-up checks: Inspect manifests and indirect inputs; test path-filter coverage; verify Android dependencies if this service is ever shipped on-device.
- RustChunk implementation mapping: Future optional metadata/mod service CI only; no first-milestone game crate.
- Evidence required before adoption: Defined 1.20.1 registry schema and ownership, path-trigger tests, reproducible locked builds and explicit compatibility tests.
- Completion state: Full workflow-source review completed; no registry build, lint or test was run.

### 158. `.github/workflows/mojang-republish.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 11c68c4437687e39eeadad92a2c7f98deca02ca0.
- Size: 34 lines.
- Coverage: lines 1–34, complete.
- Purpose: Hourly detector for Mojang version-manifest republishing that changes pinned metadata not yet recorded in research-lock.json.
- Implementation / logic: Runs hourly or manually with a dry-run input; only runs in the canonical Horizon repository; grants contents:read and issues:write; invokes a Python checker to inspect the public manifest and optionally print proposed issue changes.
- Findings: Useful supply-chain alerting pattern for detecting upstream metadata changes that may break preparation. Behavior depends on the Python script, which was not reviewed; this workflow does not automatically update pinned hashes or establish artifact authenticity.
- RustChunk relevance: High if RustChunk pins Mojang manifests or 1.20.1 downloads; alerts can protect reproducible acquisition inputs.
- 1.20.1 implications: Pin the exact 1.20.1 metadata and downloaded file hashes; investigate republished metadata before accepting any change.
- Decision: Adaptable supply-chain alert; rewrite checks around RustChunk's pinned 1.20.1 acquisition manifest.
- Dependencies / cross-references: tools/ci/mojang_republish.py; research-lock.json; Mojang public version manifest; GitHub issues API; workflow_dispatch input.
- Follow-up checks: Inspect hash/URL comparison, duplicate issue suppression, malformed manifest behavior and dry-run equivalence; identify all 1.20.1 pinned artifacts.
- RustChunk implementation mapping: Repository resource-preparation provenance workflow only; not part of runtime launch.
- Evidence required before adoption: Tests for unchanged/republished/missing/malformed entries; exact hash comparison; dry-run parity; issue deduplication and least-privilege permissions.
- Completion state: Full workflow-source review completed; no external manifest query or script execution was performed.

### 159. `.github/workflows/native-hook-stamp.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: 3ca07a4732ac8019fbfbacfe627ec29a04d5d10d.
- Size: 56 lines.
- Coverage: lines 1–56, complete.
- Purpose: Opens an automation PR assigning numbers to pending native-hook API fragments after they land on main.
- Implementation / logic: Triggers on main changes to hook fragment JSON; checks out main with history; runs a stamping script; exits if no diff; configures the GitHub bot; creates/refreshes an automation branch; commits tracked changes; pushes with force-with-lease and opens a PR. Runs only in the canonical Horizon repository with contents and pull-request write permissions.
- Findings: Makes generated numbering reviewable and separates it from the fragment submission. Correctness depends on the fragment schema and stamping script, neither reviewed here. Force-with-lease helps branch safety but race behavior and automation ownership still need tests.
- RustChunk relevance: None for vanilla gameplay. Could be a pattern for a future versioned Rust-native extension API.
- 1.20.1 implications: Starling native-hook IDs are not Forge APIs or Minecraft packet IDs. Any future extension API must have its own identifiers and compatibility contract.
- Decision: Unnecessary in the current milestone; adaptable only if a future extension API requires generated IDs.
- Dependencies / cross-references: tools/stamp_native_hooks.py; tools/native-hook-fragments/*.json; hook API schema; GitHub Actions bot; automation branch/PR API.
- Follow-up checks: Inspect schema and numbering algorithm; test deterministic order, duplicate/collision detection, idempotence, branch races and generated-output checks.
- RustChunk implementation mapping: No current runtime module; possible future devtools workflow after an extension ABI is defined.
- Evidence required before adoption: Versioned API schema, deterministic generator tests, collision tests, protected automation branch and reviewable generated-output PR.
- Completion state: Full workflow-source review completed; stamping script and workflow were not executed.

### 160. `.github/workflows/native-mods.yml`
- Source repository/ref: starling-client/starling / starling; audited from pinned snapshot 5432370d573642041585c69b40fb045b27133488.
- Source SHA: d7d5bdf5d054c42c98e9f856252169badd6731bb.
- Size: 280 lines.
- Coverage: lines 1–280, complete.
- Purpose: Builds Starling game-library variants with selected native mods and publishes shared libraries plus metadata as release assets.
- Implementation / logic: Accepts issue-form or manual requests; reads version/mod identifiers from environment; normalizes IDs, validates version syntax and tag existence, checks each requested mod manifest, selects Azure VM or hosted runner, checks out tooling/source without persisted credentials, installs Android/Rust/Java build tools, builds ARM64 library variants, uploads artifacts and publishes a prerelease. Separate jobs start/deallocate the VM and attempt to keep it alive for queued builds.
- Findings: This is a privileged native-code build-and-publish pipeline, not evidence of Forge support. It compiles selected code into a library players download; trust, build isolation, artifact integrity/signing, scoped permissions and VM cleanup are critical. The called packaging script, client downloader, signature checks and VM failure paths were not examined or executed. The dated VM transition in comments does not prove current cloud configuration.
- RustChunk relevance: Low for the vanilla base client and high-risk for any later native extension distribution. Only isolated build-orchestration concepts may be useful; Starling's native-mod format/library replacement is not a Forge adapter.
- 1.20.1 implications: No demonstrated 1.20.1/Forge compatibility. Do not load arbitrary native code in the first milestone. A later extension system needs its own ABI, exact build binding, signatures and explicit trust.
- Decision: Unnecessary for the current vanilla milestone; retain as security/CI reference only if a separately approved native-extension system is designed.
- Dependencies / cross-references: native-mods/ and prebuilt.txt; per-mod mixin manifests; packaging/android/native_build.sh; game/native_builds.rs; docs/starling-native-mods.md; Android NDK; cargo-ndk; Java 25; Azure OIDC/VM; GitHub Actions artifacts/releases.
- Follow-up checks: Inspect native_build.sh, build metadata and client downloader; verify hashes/signatures, source allowlist, tag immutability, reproducibility, artifact retention and VM cleanup on cancellation/failure.
- RustChunk implementation mapping: No current shipping component. If an extension system is approved later, reuse only high-level isolated build orchestration; independently design the format, loader, ABI, trust and signed artifact protocol.
- Evidence required before adoption: Formal extension threat model, reproducible source-to-artifact builds, reviewed allowlist, signed and client-verified artifacts, version/ABI matrix, isolated runners and tested cleanup/revocation.
- Completion state: Full workflow-source review completed; no workflow dispatch, native build, publication, VM operation, signature check or client download was performed.

## Batch checkpoint — entries 141–160 — 2026-11-18

- Completed full source review of entries 141–160 from pinned Starling tree snapshot 5432370d573642041585c69b40fb045b27133488; all 20 paths and blob SHAs were checked against that tree.
- Source files ranged from 2 to 280 lines; every file was read in full and no multi-chunk files were needed.
- Every new entry uses the 15-part standard, including explicit RustChunk implementation mapping and evidence required before adoption.
- Main findings: the parity-ledger process is valuable only after rebuilding a 1.20.1 oracle; Git hooks/public-tree checks are useful but require CI; CODEOWNERS/trusted reviewers need protected GitHub permissions; general CI patterns are adaptable but do not prove game parity; Starling native-mod workflows are out of scope for the vanilla milestone and are not Forge support.
- The release-line value remains undetermined pending consumer tracing. The native-mod build pipeline is a security-sensitive future reference, not a current implementation candidate.
- Tests/checkers/workflows were read but not executed. No Cargo build, actionlint run, Docker build, hook test, native-mod build, artifact publication, Azure VM operation, RustChunk gameplay change, Android APK build or device test was performed.

## Batch checkpoint — entries 161–170 — 2026-11-18

- Completed full source review for entries 161–170 from pinned Starling snapshot `5432370d573642041585c69b40fb045b27133488`; all ten paths and blob SHAs were checked.
- Files longer than 400 lines were read in consecutive chunks of no more than 400 lines.
- Findings cover privileged PR workflow safety, resource/server benchmark patterns, synthetic model fixtures and suspicious renderer counters.
- These files document Starling 26.3 behavior; nothing here establishes RustChunk 1.20.1 gameplay or Android performance.
- No workflow, benchmark, Cargo build, test, APK or device run was executed.

### 161. `.github/workflows/overlap-note.yml`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `bfa6a0446695ccaeaeda0c8b621225157488d51f`
- Size: 44 lines; full file fetched and read.
- Coverage: Complete; all 44 lines read.
- Purpose: Workflow that posts pull-request overlap notes for changes targeting main.
- Implementation / logic: Uses pull_request_target and per-PR concurrency; checks out only a trusted main-branch script with persisted credentials disabled; fetches PR heads as Git objects for diff analysis; permissions are contents:read and pull-requests:write. It does not checkout or execute PR-head code.
- Findings: The trusted-script boundary is important, but pull_request_target and optional self-hosted runner make token scope, runner isolation and untrusted diff handling security-critical. The invoked overlap script was not reviewed here; YAML alone does not prove safe behavior.
- RustChunk relevance: Optional PR-review automation for intersecting RustChunk changes; not a client/runtime subsystem.
- 1.20.1 implications: No Minecraft semantics. RustChunk must keep privileged workflows isolated from PR-controlled scripts and build files.
- Decision: Adaptable; Security reference
- Dependencies / cross-references: GitHub Actions event payload; trusted main-branch overlap script; GitHub PR API; optional self-hosted runner.
- Follow-up checks: Inspect the called script, fork PR behavior, head-ref fetch and diff flags. Test opened/synchronize/reopened PRs, hostile diff paths, least-privilege token and runner isolation.
- RustChunk implementation mapping: Possible future `.github/workflows/` review helper. No RustChunk runtime or gameplay component depends on it.
- Evidence required before adoption: Require adversarial fork-PR tests, diff-only behavior, no checkout/exec of PR head, minimum token permissions and verified runner isolation.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 162. `.github/workflows/pack-bench.yml`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `3fb1e1671d7a7faaa88fa0cb55b8427a58effbce`
- Size: 62 lines; full file fetched and read.
- Coverage: Complete; all 62 lines read.
- Purpose: Workflow comparing resource-pack load performance on a PR branch against the base branch.
- Implementation / logic: Runs manually or for pack-bench path changes; installs Linux dependencies and Rust; runs a client-render pack-load benchmark on the branch, then uses a base worktree and the branch's harness against base code; publishes a summary with read-only workflow permissions.
- Findings: Useful isolated comparison pattern, but the benchmark binary, renderer crate and test inputs are Starling-specific. YAML does not establish output equivalence or fairness of cache state.
- RustChunk relevance: Adaptable to a future RustChunk resource/model-load benchmark once a native resource manager exists; low priority before single-player gameplay.
- 1.20.1 implications: Fixtures/schema are 26.3. RustChunk needs 1.20.1-compatible resources and separate Android storage/memory measurements.
- Decision: Adaptable; Optional/future
- Dependencies / cross-references: GitHub Actions; Rust toolchain/cache; horizon-client-render pack-load example; Linux packages; base worktree and report script.
- Follow-up checks: Inspect benchmark example and pack licensing; verify base/branch use identical fixtures and warm/cold policy; add repetitions and target Android measurements.
- RustChunk implementation mapping: Future host benchmark in RustChunk tools or a resource crate; no native renderer benchmark target currently exists.
- Evidence required before adoption: Require semantic output equality, pinned 1.20.1 fixtures, deterministic runs, cold/warm separation and Galaxy A22 load/RAM measurements.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 163. `.github/workflows/parity-main.yml`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4840b98be6afec567393ff49d9f448e21a4625f5`
- Size: 78 lines; full file fetched and read.
- Coverage: Complete; all 78 lines read.
- Purpose: Main-branch integration workflow for changed-scope parity tests and failure bisection.
- Implementation / logic: Runs on main pushes/manual dispatch; serializes on a persistent self-hosted macOS runner; checks out the exact commit, prepares vanilla data, calculates a last-green base, runs selected parity scopes, bisects failures, comments and uploads logs.
- Findings: Self-hosted persistent state (`clean:false`, `.local`/target caches) risks stale or compromised evidence. Continue-on-error means release gating must explicitly interpret the result. Workflow was not executed.
- RustChunk relevance: Strong pattern for a future 1.20.1 parity gate; current RustChunk CI covers selected exporters/registry/tag tests, not full gameplay parity.
- 1.20.1 implications: All inherited oracles are 26.3. Rebuild source/data fixtures for 1.20.1; a green 26.3 run is not evidence of 1.20.1 support.
- Decision: Adaptable; Rewrite required
- Dependencies / cross-references: GitHub Actions; self-hosted macOS runner; parity_main.py/parity.py; vanilla-data prep; Git bisect; log artifacts.
- Follow-up checks: Inspect parity scripts, cache keys, last-green selection and bisect. Verify branch protection actually blocks on failure; test runner cleanup and isolation.
- RustChunk implementation mapping: Future GitHub Actions 1.20.1 parity gate after native Rust gameplay and Java oracle probes exist.
- Evidence required before adoption: Require pinned 1.20.1 oracle, explicit scopes/dependencies, cache fingerprinting, deterministic reruns, tested bisect/failure handling and enforced branch protection.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 164. `.github/workflows/pr-labels.yml`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d596166740105a266c8ab763321e7eb467aeda05`
- Size: 40 lines; full file fetched and read.
- Coverage: Complete; all 40 lines read.
- Purpose: Workflow that labels pull requests from trusted repository metadata and changed paths.
- Implementation / logic: Uses pull_request_target but checks out workflow/script from main, not PR head. Reads event data as input and applies labels through GitHub API with contents:read and pull-requests:write; credentials are not persisted.
- Findings: The key security rule is not executing PR code. Label mapping/API behavior is in a separate script not reviewed here; titles, body and paths remain untrusted strings.
- RustChunk relevance: Optional RustChunk triage automation; not a game feature or evidence of a test pass.
- 1.20.1 implications: No version dependence. Labels must describe RustChunk work and never imply compatibility or completion.
- Decision: Adaptable; Security reference
- Dependencies / cross-references: GitHub Actions; trusted main-branch label script; event payload and changed-files API.
- Follow-up checks: Inspect script for shell injection and malformed fields; test hostile PR titles, bodies, filenames and fork events; verify label allowlist.
- RustChunk implementation mapping: Optional workflow-only addition; no native engine code depends on labels.
- Evidence required before adoption: Require adversarial input tests, no PR-head execution, strict label allowlist, minimal token and fork-safe behavior.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 165. `AGENTS.md`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `2578901a07556cf0432bec11e7fbcb46ef73618c`
- Size: 214 lines; full file fetched and read.
- Coverage: Complete; all 214 lines read.
- Purpose: Repository-level agent/developer instructions and navigation guide for Starling's Rust-native Minecraft 26.3 codebase.
- Implementation / logic: Requires reading CONTRIBUTING first; describes project boundaries, crate/tool navigation, coding/testing practices, licensing restrictions, private-data hygiene, source ownership and validation commands.
- Findings: Strong governance reference, but paths, crates, version, Java/toolchain and commands are Starling-specific. License restrictions are Starling's policy, not automatically RustChunk policy. Instructions are not proof that CI/hooks enforce them.
- RustChunk relevance: Highly adaptable as a RustChunk root agent guide for native 1.20.1 scope, provenance, review completeness, Android target and truthful reporting.
- 1.20.1 implications: Replace all 26.3, Java25 and Starling paths/commands. State Forge/JAR mods are deferred and JNI installer code is not the native game engine.
- Decision: Adaptable; Rewrite required
- Dependencies / cross-references: CONTRIBUTING.md; Cargo workspace; architecture/version-port docs; parity probes; hooks/CI; licensing/public-tree checks.
- Follow-up checks: Trace every cited path and command against RustChunk; choose its own contribution/license policy; verify CI enforcement; include Galaxy A22/API33 and 1.20.1 oracle rules.
- RustChunk implementation mapping: Future RustChunk root AGENTS.md; Starling-specific crate paths must not be presented as existing RustChunk paths.
- Evidence required before adoption: Require each instruction to map to a real path/tool or be labelled policy; run listed commands in clean CI; verify private-data protection and README consistency.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 166. `bench/bench.py`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1f463bca5b77f53ab5195c322098b8cb7df8cec8`
- Size: 458 lines; full file fetched and read.
- Coverage: Complete; read in consecutive ranges 1–200, 201–400 and 401–458.
- Purpose: Python harness for dedicated-server load benchmarks of Horizon, vanilla 26.3 and Pumpkin with optional bot clients.
- Implementation / logic: Creates disposable runs, configures offline/creative local servers, optionally copies a prepared world, launches bots in staggered joins, waits for readiness and samples TPS/MSPT, process RSS/CPU, network bytes and join times. Produces JSON/Markdown; it is a server-load benchmark, not client FPS.
- Findings: Process/pipe cleanup needs review: exceptional paths can leave child processes/reader threads; captured pipes can block if not drained; ps-based accounting is not a Windows implementation. Console parsing is version/locale-sensitive.
- RustChunk relevance: Optional server stress methodology only; not needed for first single-player milestone.
- 1.20.1 implications: Server commands, bot behavior and protocol are 26.3/Horizon/Pumpkin-specific. A later test needs protocol-763 server/bots.
- Decision: Optional/future; Adaptable methodology
- Dependencies / cross-references: Python standard library; Horizon/Pumpkin/vanilla server binaries; horizon-bots; ps; isolated worlds/logs.
- Follow-up checks: Test timeout/crash/bot-failure cleanup; drain pipes; verify disposable world isolation, OS support, units and repeated load metrics.
- RustChunk implementation mapping: Future developer-only tools/server_load_bench.py after RustChunk gains a protocol-763 client; current RustChunk has no native game server/network client.
- Evidence required before adoption: Require real protocol-763 server/bot, no process leaks/deadlocks, deterministic world, exact version pins and reproducible TPS/network measurements.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 167. `benchmarks/entity-model-document-2026-10-14/generate_fixture.py`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `624a7abe1787406ea3d154723ac784c46714f83f`
- Size: 29 lines; full file fetched and read.
- Coverage: Complete; all 29 lines read.
- Purpose: Generator for a large synthetic entity-model JSON fixture used to test shared-document loading.
- Implementation / logic: Generates a deterministic resource-like document with boat/chest-boat, minecart, armor/player and filler layers; expands parts/quads/normals to a target file size and reports fixture shape.
- Findings: The fixture is synthetic and includes test identifiers such as test_wood; size alone does not prove every real schema branch is represented.
- RustChunk relevance: Adaptable host-side parser/model-loader stress fixture; not shipping resource data.
- 1.20.1 implications: Model/layer structure is 26.3-specific. Generate separate 1.20.1 fixtures from official data and label synthetic extensions.
- Decision: Adaptable; Test-only reference
- Dependencies / cross-references: Python JSON/path libraries; entity-model README/results; Starling model-document schema.
- Follow-up checks: Regenerate twice and compare hashes; inspect schema branch coverage, counts/depth and real 1.20.1 compatibility.
- RustChunk implementation mapping: Possible host-side fixture generator under RustChunk tools; no native model/resource loader exists in current workspace.
- Evidence required before adoption: Require deterministic fixture, explicit version/schema, parser boundary tests and parity with independent 1.20.1 resource oracle.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 168. `benchmarks/entity-model-document-2026-10-14/README.md`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `eeb8f66730ae16f36f9849475fac0ba0e2ebee29`
- Size: 61 lines; full file fetched and read.
- Coverage: Complete; all 61 lines read.
- Purpose: Methodology/results for comparing repeated per-model JSON parsing with a shared parsed entity-model document.
- Implementation / logic: Describes synthetic 2.47 MB document with 458 layers and 42 selected models; warmups, alternating order and 15 sample pairs. Equality compares parts, poses, polygons, normals, vertices and children. Reports 1434.974 ms vs 33.420 ms (~42.94×) for the isolated path.
- Findings: Speedup is only for synthetic parsing/selection; textures, full startup and GPU are excluded. Test commands in the note were not run in this audit.
- RustChunk relevance: Strong method: prove equality, warm up, alternate ordering, preserve raw samples and scope performance claims.
- 1.20.1 implications: Fixture and parser are 26.3-specific. Rebuild with 1.20.1 resources and measure on A22; do not generalize 42.94× to total game performance.
- Decision: Adaptable; Reference
- Dependencies / cross-references: generate_fixture.py; results.json; model loader; benchmark harness and Cargo tests.
- Follow-up checks: Inspect comparator/test sources; run with actual 1.20.1 models; separate cold/warm and allocation costs; measure A22 RAM/thermal impact.
- RustChunk implementation mapping: Future benchmark docs beside RustChunk 1.20.1 model parser tests; no such native engine component currently.
- Evidence required before adoption: Require field equality, source/input hashes, repeatable raw samples, cold/warm separation and target-device measurements.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 169. `benchmarks/entity-model-document-2026-10-14/results.json`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1ef0c6576a1c731871b629c57e14ead554791d57`
- Size: 44 lines; full file fetched and read.
- Coverage: Complete; all 44 lines read.
- Purpose: Raw timing samples and summary for the shared entity-model parsing experiment.
- Implementation / logic: Stores 15 baseline timings, 15 optimized timings, three warmups, 2,473,695-byte fixture, 42 selected models, medians and approximately 42.94× computed speedup.
- Findings: Raw values are consistent with the README medians, but there is no independent fixture SHA, tested source commit or hardware/toolchain metadata. It is not a whole-client startup/FPS result.
- RustChunk relevance: Useful raw-sample/report schema only, not a RustChunk target.
- 1.20.1 implications: Data belongs to Starling 26.3 and a synthetic model document.
- Decision: Reference; Adaptable schema
- Dependencies / cross-references: Companion README/generator; tested parser implementations; benchmark runner.
- Follow-up checks: Any RustChunk report should include fixture/source hashes, commit, device, toolchain, cache state and independently recalculated summaries.
- RustChunk implementation mapping: Potential future RustChunk benchmark artifact in benchmarks/ for 1.20.1 resource parsing; current engine does not consume it.
- Evidence required before adoption: Require provenance-bound inputs/code, independent statistic recomputation, output equality and Android measurements.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

### 170. `benchmarks/graphics/after-4080.json`
- Source repository/ref: starling-client/starling / starling / pinned snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `223449dd8c1849185eb6a3f57ecf3d593ae6de6a`
- Size: 547 lines; full file fetched and read.
- Coverage: Complete; read in consecutive ranges 1–400 and 401–547.
- Purpose: Post-change graphics benchmark report for Horizon on RTX4080 at render distances 8/16/32.
- Implementation / logic: Records fixed-seed four-scene 12-second flights, Vulkan, 1280×720/offscreen, section counts, upload bytes, FPS and frame diagnostics; identifies commit 9006566b.
- Findings: RD32 throughput falls sharply. At RD16 drawn_sections is zero despite nonzero upload bytes and frame activity; trace the counter semantics. Single snapshot, no repeated statistical samples.
- RustChunk relevance: Renderer telemetry/regression-triage reference only; no reusable mobile FPS target.
- 1.20.1 implications: Starling 26.3, RTX4080/Vulkan desktop/offscreen does not represent 1.20.1 Mali-G52/OpenGL ES.
- Decision: Adaptable methodology; Raw values not reusable
- Dependencies / cross-references: tools/graphics_bench.py; Horizon renderer counters; fixed world/seed; NVIDIA driver.
- Follow-up checks: Inspect counter implementation and benchmark harness; reproduce both commits with repeated conditions; investigate RD16 zero counter.
- RustChunk implementation mapping: Future RustChunk GPU benchmark records after native renderer exists; no RustChunk renderer currently produces these metrics.
- Evidence required before adoption: Require 1.20.1 fixtures, valid draw/submit counters, repeated runs, driver/API metadata and Galaxy A22 measurements.
- Completion state: Complete source review. No upstream workflow dispatch, benchmark run, parity rerun, Cargo build, APK build or device test was performed as part of this audit.

## Batch checkpoint — entries 171–180 — 2026-11-18

- Completed source review for entries 171–180 from pinned Starling snapshot `5432370d573642041585c69b40fb045b27133488`; each selected path and blob SHA was checked.
- The long performance JSONs were read in consecutive ranges of no more than 400 lines.
- The 26.3 software-renderer baseline and M4 Pro/RTX5080 raw performance reports provide benchmark formats only. Zero/near-zero drawn-section counters require tracing; none of these desktop numbers establishes Android performance.
- No benchmark was run and no RustChunk build/APK/device test was performed.

### 171. `benchmarks/graphics/after-lavapipe.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `946f1c27a730fff503e10b8d611d3a8e5fab35f7`
- Size: 547 lines; full file fetched and read.
- Coverage: Complete; read in consecutive ranges 1–400 and 401–547.
- Purpose: Post-change Horizon graphics benchmark using Mesa llvmpipe/lavapipe software Vulkan.
- Implementation / logic: Records four fixed scenes at RD8/16/32, 1600×1000, Mailbox present mode, Vulkan/llvmpipe, section/upload counters and per-flight FPS/frame metrics.
- Findings: RD32 software rendering drops to roughly 3–5 FPS. RD16 drawn_sections is zero despite uploads and frame activity. Software Vulkan is a CPU/fallback stress test, not a physical GPU result; not directly comparable to RTX4080 due different size/present mode.
- RustChunk relevance: Adaptable host CPU/software-render fallback stress scenario, low priority for mobile release.
- 1.20.1 implications: Linux llvmpipe is not Samsung Mali hardware or a 1.20.1 renderer.
- Decision: Reference; Adaptable test method
- Dependencies / cross-references: graphics benchmark harness; Mesa llvmpipe/lavapipe; Horizon renderer counters and fixed test scene.
- Follow-up checks: Trace zero-section metric; repeat identical runs and record CPU timings; validate fallback path separately from GPU hardware.
- RustChunk implementation mapping: Optional host diagnostic after RustChunk renderer exists; not an Android benchmark.
- Evidence required before adoption: Require correct counter semantics, pinned software driver, repeated fixed-scene tests and separate physical Galaxy A22 validation.
- Completion state: Complete source review; no upstream benchmark/workflow was run, no RustChunk build/APK/device test was performed.

### 172. `benchmarks/graphics/baseline-4080.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4919084242681a3cd76184f852114fe9e051bfee`
- Size: 393 lines; full file fetched and read.
- Coverage: Complete; all 393 lines read.
- Purpose: Pre-change Horizon graphics baseline on RTX4080.
- Implementation / logic: Captures four-scene short flights and render distances 8/16/32 under Vulkan; linked to baseline commit 27a32cc3 with section counters, uploaded bytes and frame performance.
- Findings: RD16/RD32 drawn-section counter is at or near zero in some scenes, so it is not yet a reliable visual-completeness metric. No repeated confidence interval data.
- RustChunk relevance: Historical before-state template for renderer regression tests only.
- 1.20.1 implications: 26.3 RTX4080/Vulkan desktop results do not establish a 1.20.1 or phone baseline.
- Decision: Reference; Version-specific
- Dependencies / cross-references: tools/graphics_bench.py; Horizon baseline commit; renderer/GPU metrics.
- Follow-up checks: Pair only with after-4080 after checking identical inputs/settings and counter meanings; repeat with source and driver pinned.
- RustChunk implementation mapping: Future RustChunk baseline/after pair after a native renderer exists.
- Evidence required before adoption: Require same source/assets/driver/resolution/scenes, repeated measurements and verified draw/submit counters.
- Completion state: Complete source review; no upstream benchmark/workflow was run, no RustChunk build/APK/device test was performed.

### 173. `benchmarks/graphics/baseline-lavapipe.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `72ea8253ca620a940812a3faa39335b8fe923050`
- Size: 393 lines; full file fetched and read.
- Coverage: Complete; all 393 lines read.
- Purpose: Pre-change Horizon graphics baseline on Mesa llvmpipe/lavapipe software Vulkan.
- Implementation / logic: Records four benchmark scenes at RD8/16/32 with software Vulkan at 1600×1000/Mailbox, FPS and renderer counters.
- Findings: RD16/RD32 drawn-section counts are around one; this cannot be treated as a visual-completeness check. It is a historical software-render snapshot, not a physical GPU measurement.
- RustChunk relevance: Reference-only for host software-renderer regression methodology.
- 1.20.1 implications: Linux llvmpipe cannot predict Android Mali-G52 or 1.20.1 rendering.
- Decision: Reference; Version-specific
- Dependencies / cross-references: graphics benchmark harness; Mesa software Vulkan; Horizon renderer baseline.
- Follow-up checks: Check counter definitions and matching after-lavapipe conditions; preserve CPU timings and repeat runs.
- RustChunk implementation mapping: Optional host software-renderer baseline after RustChunk renderer exists.
- Evidence required before adoption: Require same scene/driver/tool versions, correct counters and separate ARM64 device tests.
- Completion state: Complete source review; no upstream benchmark/workflow was run, no RustChunk build/APK/device test was performed.

### 174. `benchmarks/model-state-parse-2026-10-14.md`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `15a50e8ff7ece5a5679097f18bd76d9ed5777793c`
- Size: 70 lines; full file fetched and read.
- Coverage: Complete; all 70 lines read.
- Purpose: Benchmark report for reducing duplicate model-state JSON parsing with shared BTreeMap reuse.
- Implementation / logic: Uses synthetic input with 843 block schemas and 53,952 states, warmups and 15 samples; reports 96.080 ms repeated parse vs 73.585 ms shared parse (~1.306×).
- Findings: This isolates parse/state binding; excludes full startup, model bake, textures and GPU. Note says real-resource benefit is smaller than synthetic results.
- RustChunk relevance: Useful optimization method: eliminate duplicate parsing only after output equality is verified.
- 1.20.1 implications: Schema is 26.3-specific. Regenerate 1.20.1 inputs and measure memory tradeoff on Galaxy A22.
- Decision: Adaptable; Reference
- Dependencies / cross-references: Model-state parser; state registry; synthetic fixtures; Cargo benchmark path.
- Follow-up checks: Inspect implementation and equality tests; build 1.20.1 fixtures; capture CPU and allocations/RAM on target phone.
- RustChunk implementation mapping: Future native Rust model-state/resource parser; current JNI installer has no equivalent parser.
- Evidence required before adoption: Require exact state-map equality, source/input pins, repeatable measurements, cache separation and A22 memory/thermal data.
- Completion state: Complete source review; no upstream benchmark/workflow was run, no RustChunk build/APK/device test was performed.

### 175. `benchmarks/overlay-real-packs-2026-10-15.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `f664be6c44eac16944c27ee30b0704f5f3fb2194`
- Size: 55 lines; full file fetched and read.
- Coverage: Complete; all 55 lines read.
- Purpose: Benchmark results/metadata for overlay lookup using three published resource packs.
- Implementation / logic: Records environment/toolchain, 26.3 manifest, pack URLs/versions/ZIP sizes/SHA-512, asset counts and uncompressed sizes; compares original, cold overlay and cache-hit paths, with output equivalence checked outside the timing loop.
- Findings: Only measures overlay lookup on Linux x86_64/NVMe/warm storage; not full pack load, bake, upload or rendering. Pack files are not included; third-party licensing still applies.
- RustChunk relevance: Adaptable methodology for cache-hit/cold lookup and content provenance in a future resource-pack manager.
- 1.20.1 implications: 26.3 pack schema is version-specific. RustChunk needs 1.20.1 pack semantics and A22 filesystem measurements; do not redistribute unlicensed packs.
- Decision: Adaptable; Reference
- Dependencies / cross-references: Starling overlay/resource manager; published pack metadata and hashes; Linux cache/storage.
- Follow-up checks: Trace cache key/invalidation and output comparator; verify pack revisions/licensing; repeat with 1.20.1 packs and Android cold/warm storage.
- RustChunk implementation mapping: Future native resource manager benchmark; no such resource loader exists in current RustChunk.
- Evidence required before adoption: Require cache correctness, semantic equality, legal input provenance, versioned pack fixture and Android I/O/memory measurements.
- Completion state: Complete source review; no upstream benchmark/workflow was run, no RustChunk build/APK/device test was performed.

### 176. `benchmarks/perf/horizon-m4pro-1080p-rd16.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `563707b4d975f5bc31e6fa1cb03e68faf3ebcebc`
- Size: 759 lines; full file fetched and read.
- Coverage: complete; read consecutively in lines 1–400 and 401–759.
- Purpose: Horizon 26.3 performance report at render distance 16.
- Implementation / logic: Records app/build identity, OS/CPU/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, RSS/footprint and 12 raw frame records. Apple M4 Pro/macOS, Metal, 1920×1080 offscreen; scene-average FPS 568/333/412/357; peak RSS about 1.89 GB.
- Findings: Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with means computed from raw flights within rounding; each frame histogram sums to its recorded frame count. This verifies JSON consistency only, not the original run or tested binary.
- RustChunk relevance: Adaptable benchmark schema and controlled-scene method; desktop data are not an Android performance target.
- 1.20.1 implications: All results are Minecraft 26.3. Mac data use Metal/offscreen and Windows data use high-end RTX/Vulkan; none predicts Mali-G52/Android 13 or proves 1.20.1 correctness.
- Decision: Reference; Adaptable methodology.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; Horizon control/fly API; 26.3 world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, with target-device scenes and matching settings; measure visible chunk arrival, raw frame times, RAM, temperature and battery. Keep platform/API datasets separate.
- RustChunk implementation mapping: Future benchmark schema and instrumentation under RustChunk tools/native renderer; no current RustChunk frame loop or chunk-streaming engine exists.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural JSON checks; no benchmark run, RustChunk build, APK or phone test was performed.

### 177. `benchmarks/perf/horizon-m4pro-1080p-rd32.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `bfe22246200b7b4b02a61c345c31ff17cb0330b7`
- Size: 759 lines; full file fetched and read.
- Coverage: complete; read consecutively in lines 1–400 and 401–759.
- Purpose: Horizon 26.3 performance report at render distance 32.
- Implementation / logic: Records app/build identity, OS/CPU/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, RSS/footprint and 12 raw frame records. Same M4 Pro/Metal/offscreen setup at RD32; average FPS 326/165/210/190; peak RSS about 2.54 GB.
- Findings: Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with means computed from raw flights within rounding; each frame histogram sums to its recorded frame count. This verifies JSON consistency only, not the original run or tested binary.
- RustChunk relevance: Adaptable benchmark schema and controlled-scene method; desktop data are not an Android performance target.
- 1.20.1 implications: All results are Minecraft 26.3. Mac data use Metal/offscreen and Windows data use high-end RTX/Vulkan; none predicts Mali-G52/Android 13 or proves 1.20.1 correctness.
- Decision: Reference; Adaptable methodology.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; Horizon control/fly API; 26.3 world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, with target-device scenes and matching settings; measure visible chunk arrival, raw frame times, RAM, temperature and battery. Keep platform/API datasets separate.
- RustChunk implementation mapping: Future benchmark schema and instrumentation under RustChunk tools/native renderer; no current RustChunk frame loop or chunk-streaming engine exists.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural JSON checks; no benchmark run, RustChunk build, APK or phone test was performed.

### 178. `benchmarks/perf/horizon-m4pro-1080p-rd8.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `181008314bde7caf60fef69bbd2727c69e1964e2`
- Size: 759 lines; full file fetched and read.
- Coverage: complete; read consecutively in lines 1–400 and 401–759.
- Purpose: Horizon 26.3 performance report at render distance 8.
- Implementation / logic: Records app/build identity, OS/CPU/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, RSS/footprint and 12 raw frame records. Same M4 Pro/Metal/offscreen setup at RD8; average FPS 540/364/410/466; peak RSS about 1.54 GB; recorded load average is high.
- Findings: Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with means computed from raw flights within rounding; each frame histogram sums to its recorded frame count. This verifies JSON consistency only, not the original run or tested binary.
- RustChunk relevance: Adaptable benchmark schema and controlled-scene method; desktop data are not an Android performance target.
- 1.20.1 implications: All results are Minecraft 26.3. Mac data use Metal/offscreen and Windows data use high-end RTX/Vulkan; none predicts Mali-G52/Android 13 or proves 1.20.1 correctness.
- Decision: Reference; Adaptable methodology.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; Horizon control/fly API; 26.3 world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, with target-device scenes and matching settings; measure visible chunk arrival, raw frame times, RAM, temperature and battery. Keep platform/API datasets separate.
- RustChunk implementation mapping: Future benchmark schema and instrumentation under RustChunk tools/native renderer; no current RustChunk frame loop or chunk-streaming engine exists.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural JSON checks; no benchmark run, RustChunk build, APK or phone test was performed.

### 179. `benchmarks/perf/horizon-win-9800x3d-5080-rd16.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `017c9cbec6d786b71b156ea7c70e8aee2b94ab6a`
- Size: 774 lines; full file fetched and read.
- Coverage: complete; read consecutively in lines 1–400 and 401–774.
- Purpose: Horizon 26.3 performance report at render distance 16.
- Implementation / logic: Records app/build identity, OS/CPU/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, RSS/footprint and 12 raw frame records. Windows 11 Ryzen 7 9800X3D/RTX5080, Vulkan, 1280×720; mean FPS 1712/2424/2201/1961; peak working set about 1.46 GB.
- Findings: Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with means computed from raw flights within rounding; each frame histogram sums to its recorded frame count. This verifies JSON consistency only, not the original run or tested binary.
- RustChunk relevance: Adaptable benchmark schema and controlled-scene method; desktop data are not an Android performance target.
- 1.20.1 implications: All results are Minecraft 26.3. Mac data use Metal/offscreen and Windows data use high-end RTX/Vulkan; none predicts Mali-G52/Android 13 or proves 1.20.1 correctness.
- Decision: Reference; Adaptable methodology.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; Horizon control/fly API; 26.3 world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, with target-device scenes and matching settings; measure visible chunk arrival, raw frame times, RAM, temperature and battery. Keep platform/API datasets separate.
- RustChunk implementation mapping: Future benchmark schema and instrumentation under RustChunk tools/native renderer; no current RustChunk frame loop or chunk-streaming engine exists.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural JSON checks; no benchmark run, RustChunk build, APK or phone test was performed.

### 180. `benchmarks/perf/horizon-win-9800x3d-5080-rd32.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `82e5776686fc2b765a58ee3d21f9ca6c07e934d8`
- Size: 774 lines; full file fetched and read.
- Coverage: complete; read consecutively in lines 1–400 and 401–774.
- Purpose: Horizon 26.3 performance report at render distance 32.
- Implementation / logic: Records app/build identity, OS/CPU/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, RSS/footprint and 12 raw frame records. Same Windows/RTX5080 Vulkan setup at RD32; mean FPS 1164/1389/1189/1086; peak working set about 2.04 GB.
- Findings: Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with means computed from raw flights within rounding; each frame histogram sums to its recorded frame count. This verifies JSON consistency only, not the original run or tested binary.
- RustChunk relevance: Adaptable benchmark schema and controlled-scene method; desktop data are not an Android performance target.
- 1.20.1 implications: All results are Minecraft 26.3. Mac data use Metal/offscreen and Windows data use high-end RTX/Vulkan; none predicts Mali-G52/Android 13 or proves 1.20.1 correctness.
- Decision: Reference; Adaptable methodology.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; Horizon control/fly API; 26.3 world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, with target-device scenes and matching settings; measure visible chunk arrival, raw frame times, RAM, temperature and battery. Keep platform/API datasets separate.
- RustChunk implementation mapping: Future benchmark schema and instrumentation under RustChunk tools/native renderer; no current RustChunk frame loop or chunk-streaming engine exists.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural JSON checks; no benchmark run, RustChunk build, APK or phone test was performed.

## Batch checkpoint — entries 181–190 — 2026-11-18

- Completed full source review for entries 181–190 at the pinned source snapshot `5432370d573642041585c69b40fb045b27133488`; all paths and source blob SHAs were checked.
- All long JSONs were read consecutively in chunks of at most 400 lines; internal frame-summary/histogram consistency checks passed.
- The Mac report explicitly identifies loaded/offscreen/mismatched-size conditions. Mac and Windows performance records are 26.3 host data, not Android A22 evidence.
- No benchmark, RustChunk build, APK or device test was performed.

### 181. `benchmarks/perf/horizon-win-9800x3d-5080-rd8.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4afe983607ebf1d0f245bf15468181230af1e15b`
- Size: 774 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–774.
- Purpose: Horizon 26.3 performance report at render distance 8.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. Windows 11 Ryzen 7 9800X3D/RTX5080, Vulkan, 1280×720; average FPS 2942/3495/3346/3376; peak working set about 1.17 GB.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. 
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Horizon uses Metal/offscreen on Mac or Vulkan on high-end Windows RTX hardware. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

### 182. `benchmarks/perf/macmini-m4pro-loaded-2026-11-01.md`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `9e259be38425ee498ee7da5716f6b3274558e1e6`
- Size: 69 lines; full file fetched and read.
- Coverage: Complete; all 69 lines read.
- Purpose: Interpretation report comparing Horizon, vanilla 26.3 and vanilla with performance mods on a heavily loaded Mac mini M4 Pro.
- Implementation / logic: Documents three-repeat RD8/16/32 tests, four scenes, load averages 15–114 across 14 cores, offscreen rendering, actual size mismatch (Horizon 1920×1080 vs vanilla 1920×930), and startup/FPS/memory tables plus QoS/graphics differences.
- Findings: The report explicitly says it is not a fair player comparison: system contention differs; vanilla draws 14% fewer pixels; the compare script rejects different window sizes but the table was manually produced with that guard skipped. The main value is its cautionary analysis.
- RustChunk relevance: Reference-only for benchmark caveats; raw performance ranking should not guide RustChunk engine choices.
- 1.20.1 implications: 26.3/M4 Pro/macOS offscreen results cannot infer 1.20.1 or Galaxy A22 performance.
- Decision: Reference; Limited comparison
- Dependencies / cross-references: Adjacent Mac perf JSONs; tools/perf_bench.py; docs/benchmark.md; macOS QoS/footprint measurements.
- Follow-up checks: Keep these caveats tied to the associated JSONs. For a fair comparison, repeat with identical actual dimensions, controlled load and visible presentation.
- RustChunk implementation mapping: Maps to future RustChunk benchmark report caveats; no runtime code component.
- Evidence required before adoption: Require matched render dimensions, controlled load, comparable APIs/presentation, repeated runs and full condition matching before cross-client comparison.
- Completion state: Complete source review; no benchmark was run and no RustChunk build/APK/device test was performed.

### 183. `benchmarks/perf/vanilla-m4pro-1080p-rd16.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `11e07fd5fc551d1cb4be7ae62b0893c5ff3cbdc6`
- Size: 708 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–708.
- Purpose: Vanilla Minecraft 26.3 performance report at render distance 16.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. Vanilla 26.3 plus Fabric benchmark driver, M4 Pro/OpenGL/offscreen; mean FPS 45.5/23.3/51.3/44.3; peak RSS ~2.55 GB.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. 
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Vanilla uses OpenGL on Mac/Windows and includes a Fabric benchmark driver. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

### 184. `benchmarks/perf/vanilla-m4pro-1080p-rd32.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `40dff2a3fd0368a796344f41bb2a372922cfbbc3`
- Size: 708 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–708.
- Purpose: Vanilla Minecraft 26.3 performance report at render distance 32.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. Same Mac/OpenGL/offscreen setup at RD32; mean FPS 81.4/67.8/44.1/28.7; peak RSS ~2.96 GB.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. 
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Vanilla uses OpenGL on Mac/Windows and includes a Fabric benchmark driver. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

### 185. `benchmarks/perf/vanilla-m4pro-1080p-rd8.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `5b6895d515431c4ce4d5382db6b371b3950af492`
- Size: 708 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–708.
- Purpose: Vanilla Minecraft 26.3 performance report at render distance 8.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. Same Mac/OpenGL/offscreen setup at RD8; mean FPS 42.2/66.4/49.9/45.9; peak RSS ~1.88 GB; high sample variance.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. 
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Vanilla uses OpenGL on Mac/Windows and includes a Fabric benchmark driver. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

### 186. `benchmarks/perf/vanilla-mods-m4pro-1080p-rd16.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ffe027750cc27ffc7da36fed874144dbe6ed5fc8`
- Size: 724 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–724.
- Purpose: Vanilla Minecraft 26.3 performance report at render distance 16.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. 26.3 Fabric plus Sodium/Lithium/FerriteCore/ImmediatelyFast on M4 Pro/OpenGL/offscreen; mean FPS 46.2/61.9/75.5/35.9; peak RSS ~2.57 GB.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. The listed Fabric mods are for 26.3, not a Forge 1.20.1 mod set.
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Vanilla uses OpenGL on Mac/Windows and includes a Fabric benchmark driver. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

### 187. `benchmarks/perf/vanilla-mods-m4pro-1080p-rd32.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `7d3c160902647e01a3adbce06e300b137eaf705f`
- Size: 724 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–724.
- Purpose: Vanilla Minecraft 26.3 performance report at render distance 32.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. Same 26.3 mod set at RD32; mean FPS 91.8/113.9/119.1/56.9; peak RSS ~4.08 GB.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. The listed Fabric mods are for 26.3, not a Forge 1.20.1 mod set.
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Vanilla uses OpenGL on Mac/Windows and includes a Fabric benchmark driver. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

### 188. `benchmarks/perf/vanilla-mods-m4pro-1080p-rd8.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `eecf394e4fb02b56f7d426bc1eab0a5baa4067bc`
- Size: 724 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–724.
- Purpose: Vanilla Minecraft 26.3 performance report at render distance 8.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. Same 26.3 mod set at RD8; mean FPS 33.2/47.3/34.9/67.0; peak RSS ~2.02 GB.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. The listed Fabric mods are for 26.3, not a Forge 1.20.1 mod set.
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Vanilla uses OpenGL on Mac/Windows and includes a Fabric benchmark driver. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

### 189. `benchmarks/perf/vanilla-mods-win-9800x3d-5080-rd16.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `36bb115554375f00b8cc5710fa1e743b62493be3`
- Size: 707 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–707.
- Purpose: Vanilla Minecraft 26.3 performance report at render distance 16.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. Windows 11 Ryzen 9800X3D/RTX5080 OpenGL with 26.3 Fabric performance mods; mean FPS 1282/1746/1815/1500; peak working set ~2.50 GB.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. The listed Fabric mods are for 26.3, not a Forge 1.20.1 mod set.
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Vanilla uses OpenGL on Mac/Windows and includes a Fabric benchmark driver. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

### 190. `benchmarks/perf/vanilla-mods-win-9800x3d-5080-rd32.json`
- Source repository/ref: starling-client/starling / starling; snapshot `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `8ad8074e14af2226251e7b922867a189b0583499`
- Size: 707 lines; full file fetched and read.
- Coverage: complete; read consecutively in ranges 1–400 and 401–707.
- Purpose: Vanilla Minecraft 26.3 performance report at render distance 32.
- Implementation / logic: Stores app/mod metadata, machine/OS/GPU/backend, seed 1234, simulation distance 12, window/presentation state, three repetitions of village/forest/ocean/mountain scenes, startup/chunk loading, memory and 12 raw frame records. Same Windows modded vanilla setup at RD32; mean FPS 926/929/1017/645; peak working set ~2.96 GB.
- Findings: Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw frame records within rounding, and each histogram totals to the recorded frame count. This validates JSON consistency, not original run execution or game correctness. The listed Fabric mods are for 26.3, not a Forge 1.20.1 mod set.
- RustChunk relevance: Reference baseline and report schema only; useful to preserve raw frames, chunk-arrival, memory and hardware metadata.
- 1.20.1 implications: Minecraft 26.3 only. Vanilla uses OpenGL on Mac/Windows and includes a Fabric benchmark driver. None predicts Mali-G52/Android 13 or validates 1.20.1.
- Decision: Reference; Version-specific.
- Dependencies / cross-references: tools/perf_bench.py; docs/benchmark.md; 26.3 client/driver and world data; OS/GPU memory samplers.
- Follow-up checks: Re-run only after a playable RustChunk 1.20.1 build exists, using the target device and fixed scenes/settings; measure visible chunk arrival, raw frames, RAM, temperature and battery. Keep host/platform/API results separate.
- RustChunk implementation mapping: Future performance benchmark tooling/instrumentation after a native client/renderer exists; current JNI installer has no frame loop or chunk streaming engine.
- Evidence required before adoption: Exact build/resource hashes, repeated target-device runs, validated raw records, explicit presentation conditions, chunk-visible latency and thermal/memory acceptance.
- Completion state: Complete source review and structural report checks; no benchmark run, RustChunk build/APK or phone test was performed.

