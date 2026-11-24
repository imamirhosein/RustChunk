# RustChunk Starling Audit Ledger — Entries 3001–3200

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active 200-file stage: **3001–3200 — IN PROGRESS (100/200)**.
- Exceptional authorization **3001–3100 — EXHAUSTED (100/100)**; sections 3001–3050 and 3051–3100 are CLOSED (50/50 each).
- Checkpoints **3001–3025, 3026–3050, 3051–3075 and 3076–3100** are COMPLETE and read-back verified. **STOP at 3100**; entries 3101+ require fresh explicit authorization.
- Source selection: strict raw lexical path order from the complete pinned recursive tree after excluding every path already in the full Index. All selected paths are unique.
- Every source SHA and byte size matches the pinned tree. Canonical line counts and full EOF coverage are recorded. Long files were read in consecutive, gap-free ranges no larger than 400 lines.
- Starling/Horizon targets Minecraft 26.3 desktop behavior; this is compatibility research, not evidence of Minecraft 1.20.1 / Forge 47.x support.
- No RustChunk application source was modified. Embedded tests were inspected only; no tests, builds, benchmarks, APK, device, Forge, Minecraft, multiplayer or Aternos validation was run.

## Checkpoint 3001–3025 — COMPLETE

### 3001. `launcher/src/nightly.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4a5a066a1f35a1615fcffe9f9a45743f2bc883c9`
- Size: 5653 bytes
- Coverage: 1–157; complete EOF; 157 canonical logical lines, no gaps.
- Purpose: Implements Horizon's Nightly release version naming, display labels and cleanup after updating.
- Dependencies / cross-references: `backend::Release`, `backend::Settings`, instance metadata, signed nightly release marker and installed-version list.
- Implementation / logic: Derives `nightly-<date>-<commit>` IDs, recognizes dates/labels, updates the old Nightly instance and removes stale unselected/unpinned Nightly installs.
- Findings: Nightly is a Horizon release-channel workflow, not a Minecraft version or Forge loader. Cleanup preserves selected and instance-pinned builds; no target Android lifecycle is present.
- RustChunk relevance: Only the generic idea of keeping pinned versions while pruning stale downloads has value; Nightly itself is not required.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: RustChunk needs explicit Minecraft 1.20.1/Forge 47.x profile versioning and safe update rollback, not Horizon's Nightly release identifier.
- Follow-up checks: If adapting version cleanup, test selected profile, pinned worlds, failed update, rollback and orphaned libraries independently.
- RustChunk implementation mapping: No baseline mapping; optional download-cache retention policy only.
- Evidence required before adoption: Evidence of safe profile pinning, atomic update/rollback and Android storage lifecycle.
- Completion state: COMPLETE — 157 logical lines through EOF; pinned SHA and 5653-byte tree size match; reviewed, not modified; embedded tests inspected, not run.

### 3002. `launcher/src/other_launchers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cad1cb97b8b1e0d6addc0af9f8e18d67b3e3cf6b`
- Size: 20150 bytes
- Coverage: 1–400, 401–516; consecutive, gap-free; 516 logical lines through EOF.
- Purpose: Discovers Prism/MultiMC, official launcher, CurseForge, Modrinth and ATLauncher profiles and copies selected game data into a new Java instance.
- Dependencies / cross-references: `instances`, `loaders`, `sources::known_folders`, `minecraft_versions`, filesystem copy helpers and launcher-specific JSON/INI formats.
- Implementation / logic: Parses each launcher's metadata, identifies Minecraft/loader versions and game directories, filters shared-folder contents, then creates a profile and copies files.
- Findings: Forge is detected in metadata, but `loader()` delegates to Horizon's loader registry; paired tests confirm Forge returns `None`. CurseForge/Modrinth folders may not encode Minecraft version. Folder scanning and copying have no Android SAF model; source-folder immutability is intended but not a proof of symlink-safe traversal.
- RustChunk relevance: The import workflow and per-launcher metadata inventory are useful references; implementation must be redesigned for Android and actual Forge 1.20.1.
- Decision: rewrite required; reference; version-specific; security-sensitive
- 1.20.1 implications: RustChunk import must preserve `mods`, `config`, `saves`, resource packs and options while installing Forge 47.x itself; never silently label a Forge profile vanilla when loader parsing fails.
- Follow-up checks: Test Prism/MultiMC variants, launcher_profiles inheritance, Forge 47.4.5, missing version metadata, symlinks, path traversal, partial copy rollback and Android document providers.
- RustChunk implementation mapping: RustChunk profile-import service with allowlisted roots, SAF access, staged copy and explicit loader compatibility result.
- Evidence required before adoption: Real sample profiles from each supported launcher, safe-copy tests, Forge installer integration tests and Android storage permission tests.
- Completion state: COMPLETE — 516 logical lines read in 1–400 and 401–516; pinned SHA and 20150-byte size match; tests inspected, not run.

### 3003. `launcher/src/other_launchers/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3d2e0bcb66ef09b4d65ee3466fe544745a32e84`
- Size: 11241 bytes
- Coverage: 1–314; complete EOF; 314 canonical logical lines, no gaps.
- Purpose: Tests import parsing and copy behavior for Prism/MultiMC, official launcher, CurseForge, Modrinth and ATLauncher.
- Dependencies / cross-references: Parent `other_launchers.rs`, temporary filesystem fixtures, serde_json and `loaders::Kind`.
- Implementation / logic: Builds synthetic instance folders, validates profile metadata, game folder selection, shared-folder copy filtering, ATLauncher loader fields and error paths.
- Findings: The fixtures explicitly assert Forge metadata is detected but `loader(&found[1]) == None`; ATLauncher loader parsing also needs a nonempty version. Tests validate desktop filesystem behavior only.
- RustChunk relevance: The fixture-driven approach is useful for RustChunk's importer, but these tests are not portable evidence of Android or Forge launch compatibility.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Replace the Forge-negative expectation with real Forge 1.20.1 installer/profile fixtures and add Android storage-provider coverage.
- Follow-up checks: Add adversarial path/symlink tests, malformed JSON/INI cases, missing version manifests, duplicate profile IDs, partial copy failure and rollback.
- RustChunk implementation mapping: RustChunk importer test suite using isolated temp roots and actual Forge 47.x metadata fixtures.
- Evidence required before adoption: Passing target-specific tests on Android storage plus an end-to-end imported Forge profile launch.
- Completion state: COMPLETE — 314 logical lines through EOF; pinned SHA and 11241-byte size match; tests inspected, not executed.

### 3004. `launcher/src/owner_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b29ba90d0a272ffae5545d5ffc8b6e524f0976b`
- Size: 10802 bytes
- Coverage: 1–270; complete EOF; 270 canonical logical lines, no gaps.
- Purpose: Renders Horizon's Microsoft-account ownership gate and device-code sign-in flow.
- Dependencies / cross-references: `account`, `microsoft`, launcher sign-in state, settings notice flag, theme and GPUI shell.
- Implementation / logic: Determines whether the owner gate covers the window, starts/cancels Microsoft device-code sign-in, shows verification URL/code and persists notice dismissal.
- Findings: The gate requires an account owning Minecraft Java Edition and explicitly says offline names do not satisfy ownership; a screenshot-only fake code is environment-gated. This conflicts with RustChunk's required offline-friendly profile mode.
- RustChunk relevance: The account gate is a product policy, not a required Minecraft client subsystem; account handling may be an optional separate module.
- Decision: unnecessary; reference; privacy-sensitive; version-specific
- 1.20.1 implications: RustChunk must not make Microsoft ownership a prerequisite for offline profiles; online authentication, if implemented, must follow current Microsoft/Xbox/Minecraft services independently.
- Follow-up checks: Review authentication protocol, token storage, device-code expiry, cancellation, privacy and offline-mode separation before any optional online account feature.
- RustChunk implementation mapping: No baseline mapping; optional authentication module only, isolated from offline profile creation.
- Evidence required before adoption: Current official authentication specifications, secure Android token storage, consent UX and end-to-end online/offline mode tests.
- Completion state: COMPLETE — 270 logical lines through EOF; pinned SHA and 10802-byte size match; reviewed, not modified; tests not run.

### 3005. `launcher/src/packs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `506d566508a56bd931287d6427e6e8866789192a`
- Size: 12156 bytes
- Coverage: 1–344; complete EOF; 344 canonical logical lines, no gaps.
- Purpose: Lists, enables, disables, imports and removes resource packs and Horizon/standard shader packs for a game folder.
- Dependencies / cross-references: Filesystem, `options.txt`, Horizon `horizon-graphics.txt`, ZIP reader, JSON/TOML parsers and desktop trash seam.
- Implementation / logic: Reads pack metadata/icons from directories or ZIPs with a 4 MiB entry read cap; updates vanilla resourcePacks JSON and Horizon shader selection while preserving unrelated lines.
- Findings: Resource-pack metadata handling is partly vanilla-compatible, but shader manifest/`.hshader` support is Horizon-specific. `plain()` only walks text/translate/extra component fields. Line rewriting replaces the first matching key and appends a newline; malformed/duplicate options need target tests.
- RustChunk relevance: Resource-pack inventory and vanilla options semantics are useful; shader formats and filesystem UI assumptions are not directly portable.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Implement 1.20.1 resource-pack format/compatibility checks (including target pack format), safe ZIP handling and Android document import; do not adopt Horizon shader package semantics as Minecraft shaders.
- Follow-up checks: Test 1.20.1 `pack.mcmeta`, JSON text components, ZIP bombs/oversized entries, duplicate keys, invalid options, path names and SAF copy failure.
- RustChunk implementation mapping: RustChunk resource-pack manager; shader packs remain a separately scoped renderer feature.
- Evidence required before adoption: Minecraft 1.20.1 pack-format evidence, bounded archive tests, round-trip options tests and Android SAF integration.
- Completion state: COMPLETE — 344 logical lines through EOF; pinned SHA and 12156-byte size match; embedded tests inspected, not run.

### 3006. `launcher/src/packs/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `05a2c6b948b57635727baf6409831218e607a9a4`
- Size: 6230 bytes
- Coverage: 1–174; complete EOF; 174 canonical logical lines, no gaps.
- Purpose: Tests resource/shader pack discovery, descriptions, enable/disable persistence, duplicate naming and ZIP validation.
- Dependencies / cross-references: Parent `packs.rs`, temp filesystem, ZIP writer and JSON fixtures.
- Implementation / logic: Creates synthetic directories and ZIP archives, checks list order, icon extraction, toggling and invalid archive rejection.
- Findings: The fixture uses `pack_format:69`, a later Horizon-era value rather than Minecraft 1.20.1's resource-pack format. Shader tests are Horizon-specific; none were executed in this audit.
- RustChunk relevance: Test structure is reusable after replacing fixtures with exact 1.20.1 resource-pack metadata and Android-specific storage tests.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Use actual 1.20.1 resource pack metadata and add compatibility behavior rather than inheriting the later-version fixture.
- Follow-up checks: Add 1.20.1 positive/negative pack-format cases, malformed component JSON, large ZIP entry and directory traversal tests.
- RustChunk implementation mapping: RustChunk resource-pack unit/integration test suite.
- Evidence required before adoption: Verified 1.20.1 pack-format references and passing RustChunk tests on Android-compatible storage.
- Completion state: COMPLETE — 174 logical lines through EOF; pinned SHA and 6230-byte size match; tests inspected, not run.

### 3007. `launcher/src/packs_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3863c3c39984816314be66c3dc9060d64745d17b`
- Size: 12222 bytes
- Coverage: 1–328; complete EOF; 328 canonical logical lines, no gaps.
- Purpose: Renders Horizon's resource/shader pack tab, add/drop workflow, toggles and remove confirmation.
- Dependencies / cross-references: GPUI launcher shell, `horizon_launcher::packs`, desktop file picker/drop events and desktop image API.
- Implementation / logic: Caches pack lists, loads icons, adds files through picker/drop, disables actions while game runs and sends removed packs to desktop trash.
- Findings: Entire presentation layer is GPUI/desktop-specific; file picker and drag/drop behavior have no Android SAF equivalent. Shader-pack display is tied to Horizon's shader formats.
- RustChunk relevance: Only workflow concepts such as clear import feedback and confirmation are reusable.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Build a native Android pack-management screen and document-picker integration; keep core pack parsing independent of UI.
- Follow-up checks: Verify SAF permissions, lifecycle restoration, large icon memory, import cancellation and game-running state.
- RustChunk implementation mapping: No source-code mapping; Android pack-management UI should call a separate Rust core service.
- Evidence required before adoption: Android UI tests and SAF integration tests against 1.20.1 pack fixtures.
- Completion state: COMPLETE — 328 logical lines through EOF; pinned SHA and 12222-byte size match; reviewed, not modified; tests not run.

### 3008. `launcher/src/partner.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6692f382093f27483810047e1c061165ad717581`
- Size: 12338 bytes
- Coverage: 1–340; complete EOF; 340 canonical logical lines, no gaps.
- Purpose: Implements Horizon's Sculk Hosting partner advertisements, banners, cards, promo code and hide timer.
- Dependencies / cross-references: GPUI, bundled WebP assets, Horizon localization/theme and external browser URL.
- Implementation / logic: Loads bundled sponsor imagery, renders partner-labelled clickable ads and exposes a persistent hide action after a visibility threshold.
- Findings: This is advertising/partner integration and carries third-party branding, promo code and external navigation. No Minecraft compatibility or launcher-core value.
- RustChunk relevance: No baseline feature need; generic consent/hide behavior is not a reason to port the ad itself.
- Decision: unnecessary; reference; branding-sensitive
- 1.20.1 implications: Do not include third-party ads or sponsor assets in RustChunk baseline; any future monetization needs separate product and privacy review.
- Follow-up checks: Check licenses/brand permissions only if any asset or concept is independently proposed; otherwise exclude.
- RustChunk implementation mapping: No RustChunk implementation mapping.
- Evidence required before adoption: Explicit product approval, asset rights and privacy review would be required for any future advertising system.
- Completion state: COMPLETE — 340 logical lines through EOF; pinned SHA and 12338-byte size match; reviewed, not modified; tests inspected, not run.

### 3009. `launcher/src/perf.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `360329b0fee49e1a29a0789e10558914fbd30558`
- Size: 18304 bytes
- Coverage: 1–400, 401–512; consecutive, gap-free; 512 logical lines through EOF.
- Purpose: Collects launcher startup/frame/memory metrics and implements a repeatable desktop GUI benchmark command.
- Dependencies / cross-references: `horizon_bench`, telemetry outbox, GPUI frame loop, backend release fetch, process APIs and local benchmark directory.
- Implementation / logic: Records first-frame and UI timings, asynchronously probes hardware after first frame, saves local JSON and runs repeated GUI starts/page switching/scrolling/idle CPU tests.
- Findings: The benchmark launches the desktop GUI executable and probes desktop process RSS/CPU/power/load. `benchmark()` queues an upload payload through telemetry outbox; comments defer actual upload to consent policy. No measurement or upload was executed here.
- RustChunk relevance: Metric names and separation of local reports from consent-gated upload are useful; benchmark harness and probes need Android redesign.
- Decision: adaptable; reference; platform-specific; privacy-sensitive
- 1.20.1 implications: Use Android frame/jank, process memory and launch timing APIs; do not use desktop GUI relaunch harness or silently transmit diagnostics.
- Follow-up checks: Define consent and retention, validate telemetry outbox gating, add Android cold/warm launch and frame-jank scenarios, test missing probe values.
- RustChunk implementation mapping: RustChunk optional local performance recorder and Android benchmark harness.
- Evidence required before adoption: Android measurement validity, privacy/consent tests and reproducible benchmark runs on target hardware.
- Completion state: COMPLETE — 512 logical lines read in 1–400 and 401–512; pinned SHA and 18304-byte size match; tests inspected, not run.

### 3010. `launcher/src/plate.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9ed8e4111a9a3fad13fff25f2a79dd8e5f7e8cb5`
- Size: 8741 bytes
- Coverage: 1–250; complete EOF; 250 canonical logical lines, no gaps.
- Purpose: Draws pixel-stepped translucent plates and strokes for Horizon's desktop Night design system.
- Dependencies / cross-references: GPUI canvas/quad painting, Horizon night theme and geometry helpers.
- Implementation / logic: Computes fill/stroke rectangles for stepped corners and paints all quads in one canvas; embedded raster tests check overlap and edges.
- Findings: This is a presentation primitive built on GPUI rendering; it is not a game renderer or Minecraft terrain/chunk algorithm.
- RustChunk relevance: The shape math is a visual reference only and is not required for RustChunk's launcher or client runtime.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android-native UI components or a separately designed renderer; do not couple RustChunk's game renderer to this desktop UI primitive.
- Follow-up checks: If recreating the visual style, validate clipping, alpha blending, scaling and accessibility on Android independently.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android rendering/UI tests and explicit design approval.
- Completion state: COMPLETE — 250 logical lines through EOF; pinned SHA and 8741-byte size match; embedded tests inspected, not run.

### 3011. `launcher/src/process.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c175e34a31938d433e71e3c8dcd75543f2ef2716`
- Size: 909 bytes
- Coverage: 1–26; complete EOF; 26 canonical logical lines, no gaps.
- Purpose: Wraps process creation and suppresses Windows console windows for background commands.
- Dependencies / cross-references: Rust `std::process::Command`, Windows `CommandExt` and `rust_toolchain` priority flags.
- Implementation / logic: Creates commands and applies `CREATE_NO_WINDOW` on Windows; other targets leave command unchanged.
- Findings: Windows desktop-only behavior; Android process launching uses Android runtime/process constraints and cannot use this Windows flag.
- RustChunk relevance: No direct implementation value for Android.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android-specific process/JNI/runtime launch integration; keep OS-specific process flags behind target abstractions.
- Follow-up checks: Verify Android process lifecycle, child process restrictions and Java runtime invocation separately.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android process-launch design and target-device validation.
- Completion state: COMPLETE — 26 logical lines through EOF; pinned SHA and 909-byte size match; reviewed, not modified; tests not present.

### 3012. `launcher/src/profile_heads.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `df4bfbaeb72b2c93156e715b9d44d06fd3c6f1bb`
- Size: 5202 bytes
- Coverage: 1–153; complete EOF; 153 canonical logical lines, no gaps.
- Purpose: Caches account profile faces and asynchronously refreshes skin-derived thumbnails.
- Dependencies / cross-references: Account records, profile-face service, worker threads, message channel, GPUI image bake/cache and theme.
- Implementation / logic: Retains only active account UUIDs, tracks skin URL/pending/retry state, ignores stale results and caches scaled face images by size/scale.
- Findings: The retry/stale-result state model is broadly useful; account skin source and GPUI image rendering are platform/product-specific. Worker lifetime and cancellation need Android lifecycle handling.
- RustChunk relevance: Potential reference for asynchronous skin caching, not core game compatibility.
- Decision: adaptable; reference; platform-specific; privacy-sensitive
- 1.20.1 implications: Implement optional skin/avatar cache with explicit online account integration; offline player identity should not depend on a skin fetch.
- Follow-up checks: Test stale result races, retry backoff, account removal, memory bounds, cache invalidation and lifecycle cancellation.
- RustChunk implementation mapping: RustChunk optional profile-avatar repository and bounded image cache.
- Evidence required before adoption: Android lifecycle tests, cache quota policy and current skin/auth source documentation.
- Completion state: COMPLETE — 153 logical lines through EOF; pinned SHA and 5202-byte size match; embedded tests inspected, not run.

### 3013. `launcher/src/proxy.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d08b932b9c1ec5c3743b6bc6d62569481ec32d79`
- Size: 6362 bytes
- Coverage: 1–151; complete EOF; 151 canonical logical lines, no gaps.
- Purpose: Chooses a proxy for launcher downloads and coordinates proxy settings with the port-pack network layer.
- Dependencies / cross-references: Horizon settings, `horizon_port_pack::net`, ureq agent builder and environment proxy variables.
- Implementation / logic: Prefers saved proxy, bypasses local addresses/NO_PROXY, validates HTTP proxy syntax, caches settings behind a mutex and announces it to other download code.
- Findings: The module delegates parsing/direct-host rules to another crate and documents a separate no-proxy server API path. HTTP proxy only; SOCKS is refused with a redacted diagnostic. Environment variable semantics are desktop-shell-oriented.
- RustChunk relevance: Proxy support is useful for restricted networks, but requires one Android-wide network policy across Rust and Java/Forge downloads.
- Decision: adaptable; reference; security-sensitive; platform-specific
- 1.20.1 implications: Design proxy settings for Android and ensure every artifact path (Mojang libraries, Forge installer, mod downloads, metadata) consistently honors it without proxying loopback.
- Follow-up checks: Test HTTP/HTTPS, NO_PROXY, IPv4/IPv6 local hosts, credentials redaction, proxy changes during concurrent downloads and Java-side parity.
- RustChunk implementation mapping: RustChunk network configuration service shared by Rust downloader and Java/Forge resolver.
- Evidence required before adoption: Threat model, credential storage, proxy integration tests and end-to-end Forge dependency download through proxy.
- Completion state: COMPLETE — 151 logical lines through EOF; pinned SHA and 6362-byte size match; embedded tests inspected, not run.

### 3014. `launcher/src/rust_toolchain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5e4455002199a4e8c766cc871d8c78e73490fc4a`
- Size: 38462 bytes
- Coverage: 1–400, 401–800, 801–933; consecutive, gap-free; 933 logical lines through EOF.
- Purpose: Downloads and installs a pinned Rust 1.99.1 toolchain and Windows MinGW for Horizon native-mod builds.
- Dependencies / cross-references: Horizon backend download/build directories, static.rust-lang.org, WinLibs archive, SHA-256, tar/ZIP extraction, libc and process environment.
- Implementation / logic: Pins compiler component hashes per desktop host, unpacks toolchain, scrubs compiler environment variables, configures C linker dependencies and lowers build priority.
- Findings: Supported hosts are macOS ARM64, Linux x86_64 and Windows x86_64; Android ARM64 is absent. The toolchain exists for compiling Horizon native mods, not Java Forge mods. Archive extraction and compiler execution are high-privilege code paths requiring separate threat review.
- RustChunk relevance: No baseline role in RustChunk's Java Forge client; the general principle of pinned toolchain artifacts is a reference only.
- Decision: unnecessary; reference; security-sensitive; platform-specific
- 1.20.1 implications: Do not substitute Rust native-mod compilation for Forge's Java/ModLauncher/FML loading. RustChunk needs Rust Android NDK toolchain only for its own native engine, separately scoped.
- Follow-up checks: If any compiler downloader is later needed, audit archive path/symlink handling, checksum pin updates, process environment, executable provenance and Android target availability.
- RustChunk implementation mapping: No game/mod-loader mapping; possible separate Rust native-engine build pipeline only.
- Evidence required before adoption: Reproducible Android ARM64 build, signed toolchain manifest, archive extraction security review and independent native-engine architecture approval.
- Completion state: COMPLETE — 933 logical lines read in 1–400, 401–800 and 801–933; pinned SHA and 38462-byte size match; tests inspected, not run.

### 3015. `launcher/src/scene.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ef37512f7eb850c87ccab38d60b7dfc857ea85aa`
- Size: 46620 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1311; consecutive, gap-free; 1311 logical lines through EOF.
- Purpose: Recreates Horizon's animated pixel-sunset background and blurred backdrop for its glass UI.
- Dependencies / cross-references: GPUI canvas/images, deterministic RNG/noise, theme, `art::bake`, animation clock and glass backdrop.
- Implementation / logic: Builds static sky/halo/ridge images per window size, animates stars/clouds/sun/fireflies and computes blurred/saturated backdrop pixels; embedded tests compare deterministic pixel hashes to Horizon website output.
- Findings: This is desktop launcher decoration and GPUI rendering, not Minecraft world rendering, chunk loading, terrain meshing or a renderer backend. Per-window baked images and CPU blur are presentation costs, not game optimization evidence.
- RustChunk relevance: No direct value for RustChunk's chunk engine; at most a visual design reference.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Keep launcher UI graphics separate from Minecraft rendering and chunk scheduler; never treat this scene as evidence for Mali-G52 rendering performance.
- Follow-up checks: If visual concepts are recreated, profile Android GPU/CPU/memory independently and test lifecycle/window resize behavior.
- RustChunk implementation mapping: No baseline engine mapping.
- Evidence required before adoption: Android UI rendering profile and memory budget before adopting any animated background.
- Completion state: COMPLETE — 1311 logical lines read in 1–400, 401–800, 801–1200 and 1201–1311; pinned SHA and 46620-byte size match; tests inspected, not run.

### 3016. `launcher/src/screenshots.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d7c79fe1c73eaff497e9a729048124213af0c14`
- Size: 3778 bytes
- Coverage: 1–103; complete EOF; 103 canonical logical lines, no gaps.
- Purpose: Lists, finds and deletes screenshot files from a Java instance's screenshots folder.
- Dependencies / cross-references: Filesystem metadata, `SystemTime`, paths and desktop trash seam.
- Implementation / logic: Filters PNG/JPG/JPEG regular files, sorts newest-first by modified time and name, and resolves a filename or stem.
- Findings: The file list is simple and potentially reusable; delete is wired to desktop trash. It does not decode images or provide Android content URIs.
- RustChunk relevance: Screenshot indexing is a small optional profile-management feature, not needed for Forge launch correctness.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Use Android MediaStore/SAF or app-private screenshot paths and preserve permissions; Minecraft 1.20.1 screenshots are normally PNG.
- Follow-up checks: Test same-second ordering, missing metadata, permission revocation, external file deletion, URI lifetime and large folders.
- RustChunk implementation mapping: RustChunk optional screenshot index service.
- Evidence required before adoption: Android storage integration and target-device tests.
- Completion state: COMPLETE — 103 logical lines through EOF; pinned SHA and 3778-byte size match; embedded tests inspected, not run.

### 3017. `launcher/src/screenshots_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `be8007a95aaebfe2a8a411af6b69a99fbdc5bcc3`
- Size: 13470 bytes
- Coverage: 1–340; complete EOF; 340 canonical logical lines, no gaps.
- Purpose: Renders screenshot thumbnails, show-all list and image viewer with copy/open/reveal/delete actions.
- Dependencies / cross-references: GPUI, image decoder, clipboard, desktop path picker/open/reveal and `screenshots` service.
- Implementation / logic: Caches the list, displays first eight thumbnails, supports navigation, copies image bytes to clipboard and confirms trash deletion.
- Findings: Entire UI and file actions are desktop-specific; image decoding/copying can allocate large buffers; no Android SAF/content-URI behavior.
- RustChunk relevance: Only user-flow concepts are reusable; not a code-level Android implementation.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Build a native Android viewer using content URIs, bounded thumbnail decoding and scoped storage; keep optional to core launcher.
- Follow-up checks: Test large image memory, image orientation, URI revocation, clipboard restrictions, navigation after deletion and process recreation.
- RustChunk implementation mapping: No baseline mapping; optional Android screenshot viewer.
- Evidence required before adoption: Android UI/storage tests and memory profiling on target device.
- Completion state: COMPLETE — 340 logical lines through EOF; pinned SHA and 13470-byte size match; reviewed, not modified; tests not run.

### 3018. `launcher/src/scrollbar.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e2000d642c9bd72ea130f126bd9b2a267dd8d6ca`
- Size: 8143 bytes
- Coverage: 1–199; complete EOF; 199 canonical logical lines, no gaps.
- Purpose: Implements custom pointer-drag scrollbars for GPUI vertical scroll containers.
- Dependencies / cross-references: GPUI canvas, mouse events, ScrollHandle, hitboxes and theme.
- Implementation / logic: Computes thumb geometry, tracks drag offset and supports track-click centering with nested scroll propagation control.
- Findings: This is a desktop mouse-oriented control; Android touch/gesture scrolling and platform accessibility are not represented.
- RustChunk relevance: No need to port custom desktop scrollbar to Android.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android native scrolling and accessibility semantics; only retain geometry math if a custom touch scrollbar is deliberately required.
- Follow-up checks: Test touch drag, nested scrolling, screen readers, scaling and tiny viewport behavior if redesigned.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android gesture/accessibility tests.
- Completion state: COMPLETE — 199 logical lines through EOF; pinned SHA and 8143-byte size match; embedded tests inspected, not run.

### 3019. `launcher/src/server_list.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eedea391d235c4c8c09df8b976b23c5ea43da4dc`
- Size: 9866 bytes
- Coverage: 1–257; complete EOF; 257 canonical logical lines, no gaps.
- Purpose: Reads and updates Java Edition `servers.dat`, preserving hidden entries and unmodified NBT fields.
- Dependencies / cross-references: `horizon_world::nbt`, Base64 icon decoder, vanilla NBT fields and filesystem backup/rename.
- Implementation / logic: Reads uncompressed named NBT, lists visible servers, adds/removes entries, writes `servers.dat_new`, preserves old file as `.old` and manages a Horizon-specific official-server marker.
- Findings: Vanilla server-list NBT concepts are relevant, but `OFFICIAL_NAME`, address migration and marker are Horizon branding. Read errors collapse to an empty list in public `load`; save behavior and exact 1.20.1 NBT compatibility need tests.
- RustChunk relevance: Potentially useful for saved-server support, but separate Horizon-specific server promotion logic must be excluded.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Verify exact Minecraft 1.20.1 `servers.dat` schema and NBT library behavior; never insert Horizon's official server or addresses.
- Follow-up checks: Test 1.20.1 client round-trip, hidden Direct Connect entries, unknown fields, malformed/oversized NBT, write interruption and backup recovery.
- RustChunk implementation mapping: RustChunk saved-server repository and NBT serializer adapter.
- Evidence required before adoption: 1.20.1 client source/spec, cross-version fixtures, corruption recovery tests and Android atomic-file semantics.
- Completion state: COMPLETE — 257 logical lines through EOF; pinned SHA and 9866-byte size match; embedded tests inspected, not run.

### 3020. `launcher/src/server_list/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fc0d2b57b71cefb3195335596611a58c19becf43`
- Size: 13523 bytes
- Coverage: 1–400; complete EOF; 400 canonical logical lines, no gaps.
- Purpose: Tests server-list NBT reading/writing, hidden entries, icon preservation, official-server insertion and address migration.
- Dependencies / cross-references: Parent `server_list.rs`, NBT codec, temp filesystem and `backend::Links`.
- Implementation / logic: Builds vanilla-shaped NBT fixtures, checks unknown-field preservation, `.old` backup, visible ordering, hidden entry retention and official-server migration cases.
- Findings: Fixtures cover Horizon's official server branding/address policy and do not demonstrate compatibility against a real Minecraft 1.20.1 client. Tests were read only, not run.
- RustChunk relevance: Useful test patterns for NBT round-trip and unknown-field preservation after replacing product-specific cases.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Create target-era fixture files from Minecraft 1.20.1 and assert the actual client reads/writes them without losing metadata.
- Follow-up checks: Add malformed NBT, duplicate server addresses, non-ASCII names, max-size icons, interrupted save and actual client-produced fixture tests.
- RustChunk implementation mapping: RustChunk server-list serialization tests.
- Evidence required before adoption: Passing tests against real 1.20.1 `servers.dat` files and Android filesystem behavior.
- Completion state: COMPLETE — 400 logical lines through EOF; pinned SHA and 13523-byte size match; tests inspected, not executed.

### 3021. `launcher/src/servers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0762b2f5a5c19ba1f41a2a7dc9a769b027540fa3`
- Size: 36197 bytes
- Coverage: 1–400, 401–800, 801–1019; consecutive, gap-free; 1019 logical lines through EOF.
- Purpose: Manages Horizon-hosted servers by invoking the separate `horizon-server` supervisor, preparing game files, configuring worlds, backups and local control API.
- Dependencies / cross-references: Horizon backend version directories, `horizon-server` binary, prepared 26.3 game resources, filesystem locks, process APIs, HTTP control API and server config.
- Implementation / logic: Creates/updates server runtime, copies prepared game files, initializes worlds, manages EULA/whitelist/settings, polls supervisor API and handles backups/status/commands.
- Findings: This hosts Horizon's own 26.3 server runtime and prepared resource set; it is not a Minecraft Java 1.20.1 dedicated server and does not implement Forge server loading. It contains privileged filesystem/process actions and a bearer-token local API that need separate threat review.
- RustChunk relevance: RustChunk's goal is to join existing servers, not host a proprietary Horizon server; no baseline value.
- Decision: unnecessary; reference; security-sensitive; version-specific
- 1.20.1 implications: Do not use this server manager as a path to Forge compatibility or Aternos connectivity; implement standard Minecraft Java client networking separately.
- Follow-up checks: If a host feature is ever scoped, audit supervisor authentication, loopback binding, token file permissions, lock races, path validation, EULA flow and Java/Forge server compatibility.
- RustChunk implementation mapping: No baseline mapping; separate optional hosting product only.
- Evidence required before adoption: A real Forge 1.20.1 dedicated server, secure control-plane threat model and end-to-end interoperability evidence.
- Completion state: COMPLETE — 1019 logical lines read in 1–400, 401–800 and 801–1019; pinned SHA and 36197-byte size match; tests inspected, not run.

### 3022. `launcher/src/servers/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cc4e4be08b840d2f9ad998581eca60b0c2e93d99`
- Size: 10465 bytes
- Coverage: 1–279; complete EOF; 279 canonical logical lines, no gaps.
- Purpose: Tests Horizon server folder setup, argument generation, status parsing, control API token use and stopped-server deletion.
- Dependencies / cross-references: Parent `servers.rs`, temp directories, local TCP listener, `horizon-server` runtime fixtures and desktop trash seam.
- Implementation / logic: Exercises folder slugs, server init/config args, runtime layout, problem classification, JSON status parsing, bearer-token API calls and lock-protected deletion.
- Findings: Tests validate Horizon's own supervisor protocol and filesystem conventions, not vanilla Minecraft/Forge server startup or client multiplayer. Embedded tests were not executed.
- RustChunk relevance: The local HTTP mock and lock/deletion test patterns are references only.
- Decision: unnecessary; reference; security-sensitive; version-specific
- 1.20.1 implications: Do not interpret these fixtures as evidence of Forge server support; separate any future hosting test suite from RustChunk's client join tests.
- Follow-up checks: Add threat tests for token disclosure, unauthorized API callers, lock races, symlinked server roots and malformed status payloads only if hosting is separately approved.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Passing target-specific server control tests and a secure threat model, if hosting is approved.
- Completion state: COMPLETE — 279 logical lines through EOF; pinned SHA and 10465-byte size match; tests inspected, not run.

### 3023. `launcher/src/servers_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eb04956e2c26adc8b96c58582285d30e91799e34`
- Size: 119036 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201; consecutive, gap-free; 3201 logical lines through EOF.
- Purpose: Renders the desktop Servers page for creating, starting, monitoring, configuring, backing up and updating Horizon-hosted servers.
- Dependencies / cross-references: GPUI launcher UI, `servers` service, polling threads, control API, clipboard, local networking/Tailscale panel, desktop scrollbars and file actions.
- Implementation / logic: Maintains server-page state, polls status/logs, starts/stops jobs, edits settings, manages players/whitelist, displays LAN address, backups and update controls, and renders a create-server dialog.
- Findings: Large desktop-only GPUI page tightly coupled to Horizon's custom server supervisor and 26.3 runtime. LAN address is not public reachability; external access depends on VPN/router configuration. No Android/Forge/Aternos join flow.
- RustChunk relevance: No baseline value for the RustChunk launcher/client target; hosting UI is outside current scope.
- Decision: unnecessary; reference; platform-specific; version-specific
- 1.20.1 implications: Do not confuse a host/server administration panel with the Minecraft client multiplayer screen needed to join Aternos servers.
- Follow-up checks: If a future host feature is approved, separate service/API state from UI, threat-model commands, test lifecycle cancellation and verify external reachability securely.
- RustChunk implementation mapping: No baseline mapping; keep client multiplayer/server list as a separate subsystem.
- Evidence required before adoption: Product scope approval, Android UI architecture, real Forge server compatibility and network security review.
- Completion state: COMPLETE — 3201 logical lines read in eight consecutive 400-line-or-smaller ranges plus final line 3201; pinned SHA and 119036-byte size match; tests not run.

### 3024. `launcher/src/setup_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `651611cc9bc36eb229201f9a628096a94e9241a4`
- Size: 15532 bytes
- Coverage: 1–400, 401–418; consecutive, gap-free; 418 logical lines through EOF.
- Purpose: Implements Horizon's Windows setup/uninstall window and progress reporting.
- Dependencies / cross-references: GPUI, Horizon installer, Windows registry/shortcuts operations, settings, theme and desktop folder picker.
- Implementation / logic: Runs install/uninstall on worker thread, reports steps, selects install folder, toggles desktop shortcut/data deletion and launches installed executable.
- Findings: Explicitly Windows desktop setup; registry/shortcut assumptions and fixed window dimensions do not apply to Android APK installation or Android package lifecycle.
- RustChunk relevance: No direct code reuse for Android app installation.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: RustChunk installation/update is managed by Android package distribution and app-private storage, not a Windows setup wizard.
- Follow-up checks: If a desktop build is ever added, separately review installer rollback, data deletion confirmation and package signing.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android packaging/signing, Play/side-load install and update tests are the relevant evidence.
- Completion state: COMPLETE — 418 logical lines read in 1–400 and 401–418; pinned SHA and 15532-byte size match; tests not present.

### 3025. `launcher/src/shell.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3aed277fc8fbaa61fd45b033c4bda33f54e9ff5e`
- Size: 9515 bytes
- Coverage: 1–314; complete EOF; 314 canonical logical lines, no gaps.
- Purpose: Provides Horizon's GPUI Night design-system shell primitives: sidebar rows, tabs, account chip and buttons.
- Dependencies / cross-references: GPUI elements, Horizon theme/type scale, hover/focus tracking, pixel-cut plate helpers and keyboard navigation.
- Implementation / logic: Builds reusable desktop layout primitives with selected/hovered plates, icon buttons, text and three button styles.
- Findings: All primitives depend on GPUI and desktop pointer/keyboard interaction; no Android native UI bindings or game renderer integration.
- RustChunk relevance: Visual language may inform design, but source is not reusable in the Android launcher baseline.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android UI toolkit for launcher chrome and keep Minecraft rendering as a separate native/game subsystem.
- Follow-up checks: If the design is reused visually, test touch targets, focus/accessibility, scaling, RTL/Persian text and dark/light themes.
- RustChunk implementation mapping: No baseline source mapping; visual design reference only.
- Evidence required before adoption: Android design-system implementation and accessibility/RTL review.
- Completion state: COMPLETE — 314 logical lines through EOF; pinned SHA and 9515-byte size match; reviewed, not modified; tests not run.


## Checkpoint 3026–3050 — COMPLETE

### 3026. `launcher/src/shortcut.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `74c09f67507df3c87a535da9a66991442271439f`
- Size: 16487 bytes
- Coverage: 1–400, 401–473; consecutive, gap-free; 473 logical lines through EOF.
- Purpose: Creates and removes per-instance desktop shortcuts for Linux, macOS and Windows.
- Dependencies / cross-references: Desktop folder discovery, GPUI/OS desktop seams, Linux .desktop format, macOS app bundle, Windows Shell Link/COM.
- Implementation / logic: Sanitizes display names, embeds instance ID, writes desktop/menu entries, locates old shortcuts and moves them to trash; Windows uses IShellLinkW while a portable LNK serializer supports tests.
- Findings: All behavior targets desktop OS integration. `make_in` trashes old shortcuts before checking whether the target filename belongs to another file, so a collision error can occur after an old shortcut has already been moved. The fixed `.horizon-tmp` name can collide under concurrent writes. Portable LNK output is explicitly not a working Explorer target; Windows COM writer is required.
- RustChunk relevance: No Android desktop shortcut feature is needed for the core launcher; only per-profile launch command semantics are conceptually reusable.
- Decision: unnecessary; reference; platform-specific; security-sensitive
- 1.20.1 implications: Use Android launcher shortcuts/app shortcuts only if separately approved; do not port Linux/macOS/Windows shell formats.
- Follow-up checks: Test collision rollback ordering, concurrent writes, reserved Windows names, stale shortcut rename, partial write failure and Android app-shortcut permissions.
- RustChunk implementation mapping: No baseline mapping; optional Android App Shortcuts integration.
- Evidence required before adoption: Android launcher API behavior, atomic update/collision tests and explicit product requirement.
- Completion state: COMPLETE — 473 logical lines read in 1–400 and 401–473; pinned SHA and 16487-byte size match; embedded tests inspected, not run.

### 3027. `launcher/src/shortcut/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `87f270f71fd03be6ae1ea9234069fc36b598178b`
- Size: 8114 bytes
- Coverage: 1–225; complete EOF; 225 canonical logical lines, no gaps.
- Purpose: Tests desktop shortcut naming, Linux .desktop quoting, macOS bundle plist/script generation, Windows LNK structure and lifecycle.
- Dependencies / cross-references: Parent `shortcut.rs`, temporary filesystem, platform cfg and desktop trash seam.
- Implementation / logic: Checks sanitization, quoting, plist XML escaping, Shell Link binary offsets, shortcut create/find/rename/remove and collision refusal.
- Findings: These are desktop-format and filesystem tests; the Windows COM path is compiled/tested only on Windows. The test comments explicitly distinguish the portable LNK bytes from the shell-written shortcut used by Explorer.
- RustChunk relevance: Test organization and collision/rename cases are useful patterns, but not Android evidence.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: If Android app shortcuts are added, create a new test suite for Android shortcut intents and profile identity; do not reuse desktop fixtures as validation.
- Follow-up checks: Add tests for Android shortcut permission/launcher behavior, profile deletion, app update and duplicate labels.
- RustChunk implementation mapping: RustChunk optional Android shortcut test suite.
- Evidence required before adoption: Passing Android launcher integration tests on supported OS versions.
- Completion state: COMPLETE — 225 logical lines through EOF; pinned SHA and 8114-byte size match; tests inspected, not executed.

### 3028. `launcher/src/shortcut_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8f15deaaa1d3e37a7281064b64f4fcaa0c444706`
- Size: 2775 bytes
- Coverage: 1–73; complete EOF; 73 canonical logical lines, no gaps.
- Purpose: Adds a button and status message for making/removing an instance desktop shortcut.
- Dependencies / cross-references: GPUI Instance row, Horizon shortcut service, desktop executable path and localized strings.
- Implementation / logic: Caches whether selected instance has a shortcut and toggles make/remove while displaying success/error feedback.
- Findings: The page is a tiny GPUI desktop UI; it does not use Android pinned shortcuts or package-manager integration.
- RustChunk relevance: No direct implementation value for Android core.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Keep profile shortcut UI out of baseline unless Android App Shortcuts are explicitly required.
- Follow-up checks: If implemented, test selected profile changes, deletion, stale cache and Android launcher compatibility.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android launcher integration and UX approval.
- Completion state: COMPLETE — 73 logical lines through EOF; pinned SHA and 2775-byte size match; reviewed, not modified; tests not run.

### 3029. `launcher/src/signed.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9f2b439ca2319a38fec701346f3b6d42cee6ce07`
- Size: 6742 bytes
- Coverage: 1–148; complete EOF; 148 canonical logical lines, no gaps.
- Purpose: Verifies release asset digests against signed SHA256SUMS and built-in release keys, with pinned legacy hashes for pre-signature releases.
- Dependencies / cross-references: `horizon_release_sign`, release repository constant, GitHub release asset URLs, key list and legacy release manifest.
- Implementation / logic: Fetches checksum/signature assets, delegates signature and tag binding to the release-sign crate, refuses unsigned/unknown assets and separates Nightly signing domain from stable releases.
- Findings: Security architecture is useful, but trust roots, release repo, asset names and legacy pins are Horizon-specific. Full correctness depends on the separately audited signature crate/key-rotation policy; this file alone is not proof of secure RustChunk updates.
- RustChunk relevance: Strong reference for signed update manifests and fail-closed download verification.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: RustChunk APK/native-engine updates should use independent trusted keys, signed manifests, version/channel binding and rollback protection; Minecraft game libraries follow Mojang's own hashes and Forge installer metadata.
- Follow-up checks: Audit release-sign crate, key rotation/revocation, canonical manifest parsing, URL/tag binding, downgrade policy and Android package signature chain.
- RustChunk implementation mapping: RustChunk secure update verifier and signed release metadata, separate from Minecraft artifact resolver.
- Evidence required before adoption: Independent cryptographic review, key lifecycle documentation and tamper/rollback tests.
- Completion state: COMPLETE — 148 logical lines through EOF; pinned SHA and 6742-byte size match; embedded tests inspected, not run.

### 3030. `launcher/src/skin_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b23b90a0acd13a365e6993b7e7194ea576c253ad`
- Size: 9161 bytes
- Coverage: 1–244; complete EOF; 244 canonical logical lines, no gaps.
- Purpose: Renders a desktop settings panel to select offline-name skin/cape PNGs and choose Classic/Slim arms.
- Dependencies / cross-references: GPUI file picker, `accounts`, `skins` service, image baking and settings persistence.
- Implementation / logic: Validates the offline account still exists after file picker returns, stores skin/cape selection, caches scaled previews and offers reset controls.
- Findings: UI and file picker are GPUI desktop-specific. Skin/cape variables are Horizon's custom game integration, not a built-in Minecraft 1.20.1 Forge setting.
- RustChunk relevance: Offline skin editing could be an optional feature, but UI and game integration must be rebuilt.
- Decision: unnecessary; reference; platform-specific; version-specific
- 1.20.1 implications: If offline skins are supported, add an explicit compatible client-side implementation for Forge 1.20.1; vanilla/Forge will not automatically honor Horizon's private environment variables.
- Follow-up checks: Test stale account after dialog, large/invalid PNG, cancellation, Android SAF URI permission and actual in-game skin rendering.
- RustChunk implementation mapping: Optional RustChunk skin editor calling a bounded image-validation service and a separate game-side feature.
- Evidence required before adoption: Actual Forge 1.20.1 in-game skin rendering and Android document-picker tests.
- Completion state: COMPLETE — 244 logical lines through EOF; pinned SHA and 9161-byte size match; reviewed, not modified; tests not run.

### 3031. `launcher/src/skins.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a70933aa86677d6aeed079d6fb5a5fe2fe7ef0fd`
- Size: 6636 bytes
- Coverage: 1–183; complete EOF; 183 canonical logical lines, no gaps.
- Purpose: Validates, stores and projects custom offline-name skin/cape PNGs into Horizon's game process through environment variables.
- Dependencies / cross-references: Image crate PNG decoder, app data directory, offline account names, `HORIZON_OFFLINE_SKIN*` variables and local-player renderer hooks.
- Implementation / logic: Accepts skin dimensions 64×64/64×32 and cape 64×32/22×17, extracts front texels, stores per-name files and returns environment variables only when enabled files exist.
- Findings: The game-side contract is proprietary and requires Horizon game code to read the variables; it is not Minecraft/Forge standard behavior. `keep` reads the entire source file without a size cap and writes directly to destination. `files` lowercases the supplied name but does not itself validate path-safe names, so callers must enforce account-name constraints.
- RustChunk relevance: Skin image validation and per-profile selection are useful ideas, but implementation cannot be adopted without a game-side Forge integration and hardened file handling.
- Decision: rewrite required; reference; security-sensitive; version-specific
- 1.20.1 implications: Do not assume a Forge 1.20.1 client will render these environment-provided skins. Treat offline skin support as a separate mod/renderer feature.
- Follow-up checks: Add input byte/dimension limits, canonical offline-name validation, atomic writes, PNG decompression-bomb limits, stale-file cleanup and in-game Forge validation.
- RustChunk implementation mapping: Optional RustChunk skin asset store plus a separate Forge-compatible local-player skin feature.
- Evidence required before adoption: Security tests for untrusted PNGs/path names and real 1.20.1 Forge rendering proof.
- Completion state: COMPLETE — 183 logical lines through EOF; pinned SHA and 6636-byte size match; embedded tests inspected, not run.

### 3032. `launcher/src/skins/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `69f957b74f7de6ddd05134fd56bc6b89757016d7`
- Size: 4667 bytes
- Coverage: 1–124; complete EOF; 124 canonical logical lines, no gaps.
- Purpose: Tests skin/cape dimension validation, front-view pixel extraction, old-skin limb mirroring, environment-variable generation and forgotten-file cleanup.
- Dependencies / cross-references: Parent `skins.rs`, image PNG encoder and temporary filesystem.
- Implementation / logic: Uses coordinate-coded 64x64/64x32 fixtures to assert UV mapping and validates accepted/rejected image sizes and environment output.
- Findings: Tests validate Horizon's helper logic only; they do not load a real Minecraft 1.20.1 Forge renderer or prove offline skins appear in-game. No size-abuse or path traversal fixtures are present.
- RustChunk relevance: Good deterministic pixel-mapping test pattern for any separately implemented RustChunk skin feature.
- Decision: adaptable; reference; security-sensitive; version-specific
- 1.20.1 implications: Add actual target-version integration tests and adversarial image/path cases before adopting any skin pipeline.
- Follow-up checks: Test huge compressed PNG, invalid names, duplicate case names, corrupt file, atomic replacement and real Forge client rendering.
- RustChunk implementation mapping: RustChunk optional skin feature test suite.
- Evidence required before adoption: Passing image-security tests and Forge 1.20.1 in-game verification.
- Completion state: COMPLETE — 124 logical lines through EOF; pinned SHA and 4667-byte size match; tests inspected, not executed.

### 3033. `launcher/src/sky.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cd9b252e9a8c60fe6b2c85d8e292671f6e0a3a9f`
- Size: 24128 bytes
- Coverage: 1–400, 401–704; consecutive, gap-free; 704 logical lines through EOF.
- Purpose: Draws an animated pixel-art night/dawn sky behind Horizon's desktop launcher window.
- Dependencies / cross-references: GPUI canvas/images, Horizon theme tokens, baked cloud/skyline sprites, animation clock and accessibility reduced-motion state.
- Implementation / logic: Bakes fixed sprite layers by theme/scale, animates cloud strips and star brightness on timed beats, stops after focus loss or reduced-motion preference.
- Findings: This is decorative launcher artwork, not Minecraft's sky renderer, world renderer or chunk engine. It uses GPUI and small image atlases; the low-idle-cost claim applies only to this UI feature.
- RustChunk relevance: No relevance to chunk loading/FPS optimization; only a visual reference.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Keep launcher background animation independent from Minecraft rendering and avoid using its timings as game performance evidence.
- Follow-up checks: If recreating on Android, profile GPU/CPU/battery, honor system reduced-motion and test lifecycle pause/resume.
- RustChunk implementation mapping: No baseline game-engine mapping.
- Evidence required before adoption: Android UI performance and accessibility tests only if the decoration is explicitly requested.
- Completion state: COMPLETE — 704 logical lines read in 1–400 and 401–704; pinned SHA and 24128-byte size match; embedded tests inspected, not run.

### 3034. `launcher/src/sources.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5c2d31b0cdf6dbf406bbcf21880e0df9f29bd742`
- Size: 20657 bytes
- Coverage: 1–400, 401–543; consecutive, gap-free; 543 logical lines through EOF.
- Purpose: Finds a pinned Minecraft client JAR/version manifest in official or third-party launcher folders and links assets/libraries into Horizon's expected official layout.
- Dependencies / cross-references: Mojang version pins, SHA-1, Prism/MultiMC/Modrinth/ATLauncher/GDLauncher folder formats, filesystem symlinks/junctions, download callback and `horizon-prepare`.
- Implementation / logic: Resolves candidate folders, checks client JAR and version JSON SHA-1, fetches the pinned Mojang manifest if needed, then links client/libraries/assets into a normalized game directory.
- Findings: Pins and fixtures are for Minecraft 26.3. Loader JAR discovery only scans `<version>-*` names and tests document Fabric-profile cases; this does not install or configure Forge. Uses Unix symlinks/Windows junctions/hard links, not Android SAF; fixed `.partial` file replacement and symlink handling need Android-specific review.
- RustChunk relevance: The source-resolution model and verified artifact reuse are valuable, but must be version-aware and Android-storage-safe.
- Decision: adaptable; reference; security-sensitive; version-specific; platform-specific
- 1.20.1 implications: Implement a dedicated 1.20.1 resolver using Mojang's exact version metadata, Java 17, Forge 47.x installer-generated libraries and profile JSON. Do not infer Forge support from generic loader JAR search.
- Follow-up checks: Test Mojang manifest/client hashes, republished manifests, path/symlink attacks, partial download recovery, shared-library reuse, SAF and actual Forge profile launch.
- RustChunk implementation mapping: RustChunk artifact resolver and normalized game-directory preparation service, rewritten for Android and Forge.
- Evidence required before adoption: Pinned 1.20.1 manifest fixtures, Forge 47.4.5 install outputs, secure filesystem tests and Android launch evidence.
- Completion state: COMPLETE — 543 logical lines read in 1–400 and 401–543; pinned SHA and 20657-byte size match; tests inspected, not run.

### 3035. `launcher/src/sources_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `396f50881bb07d133d7dcd4226645dc599febd5b`
- Size: 20617 bytes
- Coverage: 1–400, 401–552; consecutive, gap-free; 552 logical lines through EOF.
- Purpose: Tests client-JAR discovery, Maven coordinate conversion, known launcher folders, symlink/hardlink normalization, manifest pinning and fallback downloads.
- Dependencies / cross-references: Parent `sources.rs`, temporary folders, fake client bytes, SHA-1, local fake Mojang fetch and platform filesystem links.
- Implementation / logic: Creates Prism/MultiMC/Modrinth/ATLauncher/GDLauncher fixtures and verifies pinned 26.3 client/manifest selection and linked library/assets behavior.
- Findings: Every fake manifest and client fixture is explicitly Minecraft 26.3. Tests cover desktop symlink/junction semantics and no actual Forge installer or Android SAF behavior.
- RustChunk relevance: Fixture-driven source-resolution tests are a useful pattern; all target-version metadata and platform assumptions must be replaced.
- Decision: rewrite required; reference; security-sensitive; version-specific
- 1.20.1 implications: Create exact Minecraft 1.20.1 fixtures and Forge 47.x install layout tests; preserve strict checksum verification while adapting storage.
- Follow-up checks: Add forged manifest/client, malicious symlink, interrupted copy, unsupported loader, SAF permission and Forge library-resolution cases.
- RustChunk implementation mapping: RustChunk artifact resolver integration test suite.
- Evidence required before adoption: Passing tests against official 1.20.1 metadata, real Forge installer outputs and Android storage.
- Completion state: COMPLETE — 552 logical lines read in 1–400 and 401–552; pinned SHA and 20617-byte size match; tests inspected, not executed.

### 3036. `launcher/src/specimen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ac7a49a99a82d12d246dbec36058907f9498d684`
- Size: 10725 bytes
- Coverage: 1–310; complete EOF; 310 canonical logical lines, no gaps.
- Purpose: Provides an internal-only design specimen page for checking Horizon's theme colors, typography, controls and contrast.
- Dependencies / cross-references: GPUI, Horizon theme, sky, plate/icon components and internal environment page selector.
- Implementation / logic: Renders sample text, controls, color swatches and contrast ratios for dark/light palettes; intentionally not reachable as a normal player page.
- Findings: This is a desktop design QA page, not a launcher feature or game rendering component. Text examples hardcode Horizon's 26.3/Fabric/Java 21 copy.
- RustChunk relevance: No baseline value beyond a possible design-system test concept.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Do not port the specimen page into RustChunk runtime; if a theme system is built, test its tokens independently.
- Follow-up checks: Add Persian/RTL and Android density/accessibility contrast cases only if RustChunk creates its own design system.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android design-system QA and RTL review if a UI theme is adopted.
- Completion state: COMPLETE — 310 logical lines through EOF; pinned SHA and 10725-byte size match; reviewed, not modified; tests not run.

### 3037. `launcher/src/switch_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `11547f37ecd243a954f771b296fd2238e415f021`
- Size: 11040 bytes
- Coverage: 1–293; complete EOF; 293 canonical logical lines, no gaps.
- Purpose: Implements account chip quick-switch and Settings rows for Microsoft accounts, offline names and the generated game name.
- Dependencies / cross-references: Account persistence, `Player` enum, Microsoft sign-in state, skin panel, GPUI menu and offline-name field.
- Implementation / logic: Switches selected player, lists Microsoft/offline identities, creates/forgets offline names and opens skin editor; shows account type and current identity.
- Findings: Offline names are explicitly described as not accepted by online servers. An offline identity is not a substitute for Microsoft/Xbox/Minecraft authentication on online-mode servers; the UI is desktop GPUI and depends on Horizon account semantics.
- RustChunk relevance: The separation of offline and authenticated identities is critical; its UX can be adapted, but online-server access must remain honest and protocol-compliant.
- Decision: adaptable; reference; authentication-sensitive; platform-specific
- 1.20.1 implications: RustChunk offline profile mode may support offline-mode servers only; joining ordinary online-mode/Aternos servers generally requires valid online authentication unless the server owner disables online mode.
- Follow-up checks: Test offline/online profile separation, UUID/name collision, server authentication errors, token lifecycle and account switching while game runs.
- RustChunk implementation mapping: RustChunk profile identity/authentication state machine and Android account UI.
- Evidence required before adoption: Real Microsoft authentication and multiplayer handshake tests; offline-mode server test kept separate.
- Completion state: COMPLETE — 293 logical lines through EOF; pinned SHA and 11040-byte size match; reviewed, not modified; tests not run.

### 3038. `launcher/src/tailscale.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c78be54a5714fed504b20a31cc3178132a48ba19`
- Size: 18734 bytes
- Coverage: 1–400, 401–507; consecutive, gap-free; 507 logical lines through EOF.
- Purpose: Integrates Tailscale CLI and API to share a hosted Horizon server with friends and restrict shared-device access to one port.
- Dependencies / cross-references: Tailscale CLI, API access token, system keychain seam, HTTP API, proxy agent, tailnet ACL/grants and local server address.
- Implementation / logic: Reads Tailscale status, builds MagicDNS/IP address, creates/revokes one-use device invitations and rewrites only recognized default ACL policy shapes using ETag concurrency.
- Findings: This is optional remote-hosting infrastructure, not Minecraft client multiplayer. It requires installed Tailscale CLI/API token and desktop keychain; ACL changes are high-impact network policy. `Node::address` falls back to IPv4 only when MagicDNS is disabled; no IPv6 fallback.
- RustChunk relevance: No baseline requirement: RustChunk must join public/private servers directly, not require Tailscale or host a server.
- Decision: unnecessary; reference; security-sensitive; platform-specific
- 1.20.1 implications: Do not make Tailscale a prerequisite for Aternos or ordinary multiplayer. Any optional VPN integration must never silently alter a user's ACL policy.
- Follow-up checks: Threat-model token storage, device invitation scope, ETag races, custom ACL preservation, API error redaction, IPv6-only tailnet and Android Tailscale integration.
- RustChunk implementation mapping: No baseline mapping; optional separately scoped private-network feature only.
- Evidence required before adoption: Security review, Android VPN integration approval and network interoperability tests.
- Completion state: COMPLETE — 507 logical lines read in 1–400 and 401–507; pinned SHA and 18734-byte size match; tests inspected, not run.

### 3039. `launcher/src/tailscale/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `98078ddbc8f7ed287d038a4ff1763aac31ae799b`
- Size: 10832 bytes
- Coverage: 1–270; complete EOF; 270 canonical logical lines, no gaps.
- Purpose: Tests Tailscale CLI status parsing, MagicDNS/IP formatting, invitation API, ETag handling and ACL policy rewrite safety.
- Dependencies / cross-references: Parent `tailscale.rs`, fake local TCP API, JSON fixtures and stand-in CLI script.
- Implementation / logic: Asserts read-only status command, invite lifecycle, token header, custom policy preservation, ACL/grants support and compare-and-swap `If-Match`.
- Findings: The tests use local stand-ins and do not call live Tailscale or test Android VPN behavior. They validate Horizon's optional server-sharing flow, not public Minecraft server joining.
- RustChunk relevance: Good API-mocking and policy-preservation test patterns if an optional network feature is approved.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Keep multiplayer core independent; if adding VPN, retain the rule that custom ACL policies are never overwritten.
- Follow-up checks: Add IPv6-only, malformed response, token leakage, ACL race, revoked device and Android VPN lifecycle tests.
- RustChunk implementation mapping: Optional RustChunk VPN integration test suite.
- Evidence required before adoption: Security review and Android VPN/device tests.
- Completion state: COMPLETE — 270 logical lines through EOF; pinned SHA and 10832-byte size match; tests inspected, not executed.

### 3040. `launcher/src/telemetry.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `06f9fee3e84f028a0616524e95e8f6968a10f9ec`
- Size: 17983 bytes
- Coverage: 1–400, 401–469; consecutive, gap-free; 469 logical lines through EOF.
- Purpose: Coordinates opt-in performance/crash/bug/benchmark telemetry, preview/delete actions, report scrubbing and live player-count pings.
- Dependencies / cross-references: `horizon_telemetry`, consent/outbox/upload, account/world scrubbing, backend logs, official server status and background threads.
- Implementation / logic: Persists choices, prunes disallowed queued items, previews exact payloads, scrubs names/worlds from reports, submits only allowed report kinds and deletes remote install-ID data when possible.
- Findings: The anonymous `I'm playing` ping is explicitly sent once per minute regardless of telemetry switches; live official-server player counts are also polled when endpoint exists. The declared never-sent list is a product assertion, not independently verified here. `delete_my_data` resets local ID without remote deletion when endpoint is off. Privacy-sensitive behavior must not be copied implicitly.
- RustChunk relevance: Telemetry is not required for RustChunk functionality; explicit opt-in, transparent preview and data deletion patterns are references only.
- Decision: unnecessary; reference; privacy-sensitive
- 1.20.1 implications: Default RustChunk telemetry should be absent or strictly opt-in; never add background pings outside consent. Do not send usernames, UUIDs, server addresses, chat, worlds or device identifiers without clear permission.
- Follow-up checks: Audit telemetry crate/server retention, consent migration, queue pruning, log scrubbing, background thread shutdown, deletion retry and endpoint-off semantics.
- RustChunk implementation mapping: No baseline mapping; optional privacy-reviewed telemetry subsystem only.
- Evidence required before adoption: Independent privacy/security review, data-flow inventory, consent tests and server-side deletion evidence.
- Completion state: COMPLETE — 469 logical lines read in 1–400 and 401–469; pinned SHA and 17983-byte size match; tests inspected, not run.

### 3041. `launcher/src/theme.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `093df3331d2bdd27f69bb0140eb7d8240eefc6f2`
- Size: 6988 bytes
- Coverage: 1–187; complete EOF; 187 canonical logical lines, no gaps.
- Purpose: Maps Horizon theme tokens and dark/light appearance into the desktop launcher's GPUI theme.
- Dependencies / cross-references: GPUI Hsla/WindowAppearance, Horizon theme crate, accent settings and design tokens.
- Implementation / logic: Builds color palette and spacing/motion constants from shared tokens and chooses system/pinned appearance.
- Findings: The adapter is GPUI-specific, but the idea of one semantic token source and contrast-checked states is generic.
- RustChunk relevance: Visual design architecture is a reference, not game-engine code.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: If RustChunk builds a launcher UI, define platform-neutral semantic tokens then adapt them to Android components; keep Minecraft renderer settings separate.
- Follow-up checks: Test light/dark, high contrast, color-blind states, dynamic type, RTL and Android system appearance.
- RustChunk implementation mapping: RustChunk UI design-token module, only if UI architecture chooses it.
- Evidence required before adoption: Android accessibility/contrast and visual regression tests.
- Completion state: COMPLETE — 187 logical lines through EOF; pinned SHA and 6988-byte size match; embedded tests inspected, not run.

### 3042. `launcher/src/themes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cb761aaace091cd88fa15762544de02d24a552d3`
- Size: 8260 bytes
- Coverage: 1–232; complete EOF; 232 canonical logical lines, no gaps.
- Purpose: Stores accent hue and validates/copies a custom Home background picture.
- Dependencies / cross-references: Horizon theme color math, serde settings, image crate, filesystem data directory and SHA-1 naming.
- Implementation / logic: Maps named/custom hue through HSL while preserving contrast thresholds; accepts PNG/JPEG/WebP up to 40 MiB and at least 64x64, stores content-derived filename and removes older background.
- Findings: Generic accent color math and bounded image selection can be reused conceptually. Home background is desktop UI; validation checks dimensions but does not fully decode pixels, and filesystem replacement/removal semantics need Android storage design.
- RustChunk relevance: Optional launcher UI feature; no Minecraft runtime relevance.
- Decision: adaptable; reference; platform-specific; security-sensitive
- 1.20.1 implications: Use Android content URIs/scoped storage and explicit image decode limits if adding a custom launcher background; no effect on Forge rendering.
- Follow-up checks: Test malformed image headers, huge dimensions with small compressed body, race during replacement, duplicate hash, storage permission loss and image decoder limits.
- RustChunk implementation mapping: Optional RustChunk theme settings and Android image asset repository.
- Evidence required before adoption: Android image-security tests, accessibility contrast checks and SAF integration.
- Completion state: COMPLETE — 232 logical lines through EOF; pinned SHA and 8260-byte size match; embedded tests inspected, not run.

### 3043. `launcher/src/themes/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `31f192f59570a46951c8a917281a93d138edf6bf`
- Size: 4783 bytes
- Coverage: 1–124; complete EOF; 124 canonical logical lines, no gaps.
- Purpose: Tests contrast preservation for accent hues and custom background image acceptance/rejection.
- Dependencies / cross-references: Parent `themes.rs`, Horizon theme palettes, image encoder and temp filesystem.
- Implementation / logic: Sweeps 720 custom hues across dark/light palettes, checks text/focus contrast, HSL round-trip, picture type/size, stale background removal and path traversal rejection.
- Findings: Tests cover Horizon's theme and image helper only; they do not test Android content URIs, image decoder resource limits or Persian/RTL rendering.
- RustChunk relevance: The contrast sweep and adversarial file-selection patterns are useful test references.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Port test intent only; use Android-specific storage and bounded decode fixtures.
- Follow-up checks: Add image bomb/oversized-dimension, malformed metadata, SAF revocation, Persian/RTL and accessibility contrast cases.
- RustChunk implementation mapping: RustChunk theme/image utility test suite.
- Evidence required before adoption: Passing Android storage/security and accessibility tests.
- Completion state: COMPLETE — 124 logical lines through EOF; pinned SHA and 4783-byte size match; tests inspected, not executed.

### 3044. `launcher/src/themes_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6bd99a13cb28abb63e0bd8126d08920963609a38`
- Size: 9973 bytes
- Coverage: 1–275; complete EOF; 275 canonical logical lines, no gaps.
- Purpose: Renders desktop Settings rows for accent swatches, draggable hue strip and custom Home background picker.
- Dependencies / cross-references: GPUI mouse events/canvas, Horizon themes, desktop file dialog and settings persistence.
- Implementation / logic: Updates preview theme during hue drag and saves once on release; selects/removes background through desktop path picker.
- Findings: Mouse drag and desktop file picker are not Android touch/SAF APIs; implementation is tightly coupled to GPUI state.
- RustChunk relevance: Only workflow concepts can be adapted.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android-native color picker and document picker if the feature is included; avoid implementing a desktop-like window.
- Follow-up checks: Test touch drag cancellation, rotation, state restore, permission revocation, RTL and large-screen layouts.
- RustChunk implementation mapping: No baseline mapping; optional Android settings screen.
- Evidence required before adoption: Android UI/SAF tests and accessibility review.
- Completion state: COMPLETE — 275 logical lines through EOF; pinned SHA and 9973-byte size match; reviewed, not modified; tests not run.

### 3045. `launcher/src/titlebar.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dbf0f966e7bba613ebbe6bdf43d887b0105419ff`
- Size: 13732 bytes
- Coverage: 1–400, 401–422; consecutive, gap-free; 422 logical lines through EOF.
- Purpose: Implements native/custom desktop window title bars, controls, resize handles and appearance bridging for Linux/macOS/Windows.
- Dependencies / cross-references: GPUI window handles, Linux client-side decorations, Windows window-control hit tests, macOS AppKit/Objective-C and raw-window-handle.
- Implementation / logic: Selects system decoration mode, draws Linux/Windows controls, applies macOS appearance and adds resize edges.
- Findings: All code is desktop window chrome; OS-specific unsafe AppKit handle access and CSD hit tests have no Android activity/window equivalent.
- RustChunk relevance: No code-level value for Android launcher or Minecraft rendering.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android activity/system bars and edge-to-edge APIs; do not port desktop titlebar behavior.
- Follow-up checks: If a desktop RustChunk port is later scoped, test window decorations per compositor/OS and review unsafe AppKit lifetimes.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android system-bar and edge-to-edge UI tests only if a custom desktop port is planned.
- Completion state: COMPLETE — 422 logical lines read in 1–400 and 401–422; pinned SHA and 13732-byte size match; reviewed, not modified; tests not run.

### 3046. `launcher/src/while_playing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d47d20ac40a2b625e558e135ceaf80d1f9115819`
- Size: 3377 bytes
- Coverage: 1–95; complete EOF; 95 canonical logical lines, no gaps.
- Purpose: Minimizes/hides the desktop launcher while a game runs and restores it after game exit.
- Dependencies / cross-references: GPUI window handle, launcher settings and desktop `WhilePlaying` mode.
- Implementation / logic: Reads current preference at launch, defers hide/minimize action and activates launcher window when game process closes.
- Findings: Window hide/minimize and reactivation are desktop-only; Android activity backgrounding and process lifecycle differ and are controlled by the OS.
- RustChunk relevance: Only the user preference concept is reusable.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android activity/task lifecycle and foreground-service rules where applicable; never assume launcher can force itself to foreground on game exit.
- Follow-up checks: Test activity recreation, process death, return from game, multi-window and Android background restrictions.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android lifecycle and task-stack integration tests.
- Completion state: COMPLETE — 95 logical lines through EOF; pinned SHA and 3377-byte size match; reviewed, not modified; tests not run.

### 3047. `launcher/src/worlds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3317ab1b82126d910830b66eca1a000cf73dc537`
- Size: 9656 bytes
- Coverage: 1–260; complete EOF; 260 canonical logical lines, no gaps.
- Purpose: Lists Minecraft single-player worlds from level.dat and backs up, copies, renames or deletes world folders.
- Dependencies / cross-references: Gzip/NBT codec, filesystem recursion, ZIP writer, chrono timestamps and desktop trash seam.
- Implementation / logic: Reads gzip/plain NBT, extracts mode/version/time/name/icon, creates vanilla-style backup ZIP excluding session.lock, copies world folder and rewrites LevelName.
- Findings: The metadata reader contains 26.3-specific `difficulty_settings.hardcore` handling plus legacy Data.hardcore. Backup/copy traversal treats symlinks as non-directories then `File::open`/`fs::copy` follows them, so a crafted symlink inside a world may pull data from outside the world into a backup/copy; requires explicit symlink rejection. Initial `fs::read(level.dat)` is unbounded before the 64 MiB decompression cap. Delete is desktop trash only.
- RustChunk relevance: World NBT/backup concepts are valuable for an optional single-player manager, but storage traversal and version schema need hardening.
- Decision: adaptable; reference; security-sensitive; version-specific
- 1.20.1 implications: Support 1.20.1 level.dat fields and Android app-private/SAF world directories; preserve vanilla backup format and reject symlinks/special files.
- Follow-up checks: Add symlink/special-file tests, bounded compressed input, malformed NBT, interrupted ZIP cleanup, world lock/running checks, path validation and 1.20.1 fixtures.
- RustChunk implementation mapping: RustChunk optional world repository and backup service, rewritten for Android storage and safe traversal.
- Evidence required before adoption: Security review of recursive file operations, NBT compatibility tests and Android world backup/restore tests.
- Completion state: COMPLETE — 260 logical lines through EOF; pinned SHA and 9656-byte size match; embedded tests inspected, not run.

### 3048. `launcher/src/worlds/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7949b8c83cf6c4d0eb34384f0510f7963a17dbdc`
- Size: 5345 bytes
- Coverage: 1–160; complete EOF; 160 canonical logical lines, no gaps.
- Purpose: Tests world listing, NBT metadata extraction, vanilla-style backup ZIP and copy naming/lock exclusion.
- Dependencies / cross-references: Parent `worlds.rs`, NBT encoder/decoder, gzip, ZIP reader and temporary filesystem.
- Implementation / logic: Builds synthetic gzip level.dat fixtures, checks mode/date/name/icon, verifies backup entry paths omit session.lock and copied worlds receive unique LevelName.
- Findings: Fixtures use Version.Name `26.3`; no 1.20.1-specific fixtures, symlink traversal, oversized level.dat or real-world lock behavior are covered.
- RustChunk relevance: Good deterministic world-backup test structure; needs target-version and filesystem-security coverage.
- Decision: adaptable; reference; security-sensitive; version-specific
- 1.20.1 implications: Replace/add fixtures from real Minecraft 1.20.1 worlds and test Android storage operations.
- Follow-up checks: Add symlink-to-outside, special file, corrupt/huge level.dat, active session.lock, interrupted backup and restore verification.
- RustChunk implementation mapping: RustChunk optional world-service tests.
- Evidence required before adoption: Passing tests against actual 1.20.1 world fixtures and Android storage.
- Completion state: COMPLETE — 160 logical lines through EOF; pinned SHA and 5345-byte size match; tests inspected, not executed.

### 3049. `launcher/src/worlds_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9a78fab218cfb48a5cd51e1719c206a0403a45a7`
- Size: 13312 bytes
- Coverage: 1–340; complete EOF; 340 canonical logical lines, no gaps.
- Purpose: Renders selected instance's worlds, Play/Backup/Copy/Delete/Data packs/Open folder actions and background loading.
- Dependencies / cross-references: GPUI, `worlds` service, async background executor, instance selection, desktop open-folder action and packs panel.
- Implementation / logic: Loads world list asynchronously, guards against instance switches, serializes one busy action and confirms deletion before trashing.
- Findings: Desktop GPUI presentation and file-open/trash APIs; async state patterns may inform a mobile UI but data operations need a safe core service.
- RustChunk relevance: Optional world-management UI, not required for Minecraft launch or multiplayer.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: If adding worlds UI, call a separately hardened world service and use Android storage/document APIs.
- Follow-up checks: Test instance switch during load, process recreation, action cancellation, large lists, busy state and confirmation persistence.
- RustChunk implementation mapping: No baseline mapping; optional Android worlds screen.
- Evidence required before adoption: Android UI/storage tests and hardened world-service integration.
- Completion state: COMPLETE — 340 logical lines through EOF; pinned SHA and 13312-byte size match; reviewed, not modified; tests not run.

### 3050. `launcher/tests/desktop.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0a9f33ff19320b6235ba09c1e14b4a06fdc390d7`
- Size: 2066 bytes
- Coverage: 1–55; complete EOF; 55 canonical logical lines, no gaps.
- Purpose: Linux-only integration test for preserving a distributor's desktop-entry executable wrapper across menu registration and removal.
- Dependencies / cross-references: Cargo-built launcher binary, Linux .desktop file, XDG_DATA_HOME and temporary filesystem.
- Implementation / logic: Invokes CLI desktop-entry command with and without wrapper override and checks generated/removal behavior.
- Findings: Compiled only on Linux and tests desktop package integration; no Android, Minecraft, Forge or client runtime coverage.
- RustChunk relevance: No direct value for Android launcher.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Android package identity, app shortcuts and update flows need separate tests.
- Follow-up checks: Test Android package update, app link, install/uninstall and data preservation only if packaging changes.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android package integration tests and APK install/update validation.
- Completion state: COMPLETE — 55 logical lines through EOF; pinned SHA and 2066-byte size match; test inspected, not run.


## Checkpoint 3051–3075 — COMPLETE

### 3051. `launcher/tests/fixtures/native-crate-demo/data/greeting.txt`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aa725498aa2b24d7573e8c58c7bae4021010f02a`
- Size: 36 bytes
- Coverage: 1–1; complete EOF; 1 canonical logical line, no gaps.
- Purpose: Fixture data file for a Horizon native Rust-mod bundle demo.
- Dependencies / cross-references: `horizon-mixin.toml`, `frame.rs`, `horizon_client::native_data!` and the native-mod bundle fixture.
- Implementation / logic: Contains a short greeting string embedded into the demo mod and read by its injected frame hook.
- Findings: Pure fixture content; its behavior depends on Horizon's native-mod compiler/bundle loader, not Minecraft's Java mod system.
- RustChunk relevance: No direct RustChunk runtime value.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not treat this fixture as a Forge mod asset or standard Minecraft resource path.
- Follow-up checks: If a native extension format is separately approved, test asset namespacing, bundle inclusion and path isolation.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Only an independently approved native plugin format and bundle-integrity tests.
- Completion state: COMPLETE — 1 logical line through EOF; pinned SHA and 36-byte size match; fixture inspected, not executed.

### 3052. `launcher/tests/fixtures/native-crate-demo/engine/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e1bacb1fb8ef02fe68c574f7713240b49ba49eea`
- Size: 183 bytes
- Coverage: 1–9; complete EOF; 9 canonical logical lines, no gaps.
- Purpose: Cargo manifest for the demo native Rust crate compiled separately from the Horizon game.
- Dependencies / cross-references: Cargo workspace, Horizon Cargo.lock, `crc32fast` and `bake.rs` binary target.
- Implementation / logic: Declares a non-publishable Rust 2024 crate with a single version-ranged dependency expected in Horizon's lockfile.
- Findings: The manifest belongs to a demonstration native-mod crate; edition 2024 and host dependency resolution are not evidence of Android NDK/Forge compatibility.
- RustChunk relevance: Separate Rust crate compilation can be a native-engine technique, but not a replacement for Java Forge loading.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: RustChunk's own native engine may use Cargo, but game mods for Forge 1.20.1 remain Java JARs with Forge metadata and lifecycle.
- Follow-up checks: Check Android target triples, dependency versions, reproducible builds, ABI and license policy if a Rust subcrate is introduced.
- RustChunk implementation mapping: No mod-loader mapping; optional Rust engine crate only.
- Evidence required before adoption: Android ARM64 NDK build and integration tests for any separately scoped native engine.
- Completion state: COMPLETE — 9 logical lines through EOF; pinned SHA and 183-byte size match; manifest inspected, no Cargo build run.

### 3053. `launcher/tests/fixtures/native-crate-demo/engine/data/salt.txt`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `06a710d47a022aa3500ad102c1f67e68f0d42e2f`
- Size: 16 bytes
- Coverage: 1–1; complete EOF; 1 canonical logical line, no gaps.
- Purpose: Static input embedded into the demo native Rust crate checksum function.
- Dependencies / cross-references: `include_bytes!`, `crate-demo-engine` and `bake.rs`.
- Implementation / logic: Provides a small fixed salt that the demo crate compiles into its binary and includes in CRC-32 output.
- Findings: A test fixture only; `include_bytes!` demonstrates compile-time resource embedding, not Minecraft mod resource packaging.
- RustChunk relevance: No direct compatibility value.
- Decision: unnecessary; reference
- 1.20.1 implications: If RustChunk embeds engine data, use explicit asset/resource packaging and checksums rather than adopting the fixture value.
- Follow-up checks: Test resource inclusion, deterministic hashes and cache invalidation if using compile-time assets.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Reproducible build and resource integrity tests.
- Completion state: COMPLETE — 1 logical line through EOF; pinned SHA and 16-byte size match; fixture inspected, not executed.

### 3054. `launcher/tests/fixtures/native-crate-demo/engine/src/bin/bake.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1529f15061ddf9efc53ce81eece355bc31b606bd`
- Size: 962 bytes
- Coverage: 1–22; complete EOF; 22 canonical logical lines, no gaps.
- Purpose: Post-build executable for the native-mod demo; increments a persistent cache counter and writes a generated data file.
- Dependencies / cross-references: Cargo binary target, `HORIZON_NATIVE_DATA_DIR`, `HORIZON_NATIVE_CACHE_DIR` and `crate_demo_engine::checksum`.
- Implementation / logic: Reads/writes build cache, computes a deterministic checksum and emits a generated file into the new game build's native data directory.
- Findings: This runs a separate native build step and writes to environment-provided paths; it is part of Horizon's proprietary native-mod pipeline, not Forge's Java installer lifecycle. Path trust and process sandboxing are not addressed by this fixture.
- RustChunk relevance: Post-build artifact generation is a possible native-engine build pattern, but should not run untrusted mod code in RustChunk baseline.
- Decision: unnecessary; reference; security-sensitive; version-specific
- 1.20.1 implications: Keep Forge installation/launch separate from optional Rust engine compilation. Any build hook must be sandboxed and constrained to app-owned directories.
- Follow-up checks: Threat-model executable provenance, environment path injection, cache isolation, concurrent builds, stale data and build failure cleanup.
- RustChunk implementation mapping: No baseline mapping; optional signed native-engine build stage only.
- Evidence required before adoption: Reproducible Android builds, sandbox design, path containment tests and signed toolchain verification.
- Completion state: COMPLETE — 22 logical lines through EOF; pinned SHA and 962-byte size match; fixture inspected, not executed.

### 3055. `launcher/tests/fixtures/native-crate-demo/engine/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4315ac5fd028cb53618de57c1431a16f3cf8f10d`
- Size: 454 bytes
- Coverage: 1–13; complete EOF; 13 canonical logical lines, no gaps.
- Purpose: Demo Rust crate logic used by the native-mod build fixture.
- Dependencies / cross-references: Cargo manifest, `include_bytes!` data and `crc32fast`.
- Implementation / logic: Embeds salt at compile time and exposes a checksum function consumed by the post-build binary and injected frame hook.
- Findings: This is a trivial native crate demonstration; it does not implement Minecraft APIs, Forge entrypoints or Java mod interoperability.
- RustChunk relevance: No direct game/mod compatibility value.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: RustChunk may use Rust for its own engine modules, but must still load Java Forge mods through Forge 47.x.
- Follow-up checks: If a native Rust module is planned, define a stable FFI boundary, panic strategy, ABI and memory ownership before porting patterns.
- RustChunk implementation mapping: No Forge mapping; optional native engine library only.
- Evidence required before adoption: ARM64 ABI/FFI tests and separation from Java Forge classloading.
- Completion state: COMPLETE — 13 logical lines through EOF; pinned SHA and 454-byte size match; fixture inspected, not executed.

### 3056. `launcher/tests/fixtures/native-crate-demo/frame.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8d5ed0305e1a27a01aaa8fc1b51bf54dc13b9b30`
- Size: 1281 bytes
- Coverage: 1–27; complete EOF; 27 canonical logical lines, no gaps.
- Purpose: Demo native injection hook that runs once on the first client frame and records bundle data, generated data and checksum proof.
- Dependencies / cross-references: Horizon hook registry (`frame`), native-data macros, `Game` APIs, demo engine crate and game log/filesystem.
- Implementation / logic: Uses a static AtomicBool to execute once, reads bundled greeting and post-build output, writes a proof file in mod-private data directory.
- Findings: The hook binds directly to Horizon's `crate::game::Game` internals and custom macro APIs; it is not a Fabric/Forge event or Mixin. The one-time flag is process-global, not per game instance.
- RustChunk relevance: The idea of an early client-frame instrumentation hook is relevant to performance diagnostics, but API and loader are proprietary.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: For Forge 1.20.1 diagnostics, use supported Forge client events/Mixins and keep instrumentation out of release builds.
- Follow-up checks: Verify hook timing, thread affinity, profile isolation, shutdown, multiple client instances and no filesystem writes outside app storage.
- RustChunk implementation mapping: RustChunk benchmark/diagnostic hook implemented through a target-compatible Forge client mod or controlled Java instrumentation.
- Evidence required before adoption: Real Forge 47.x hook ordering and Android client integration tests.
- Completion state: COMPLETE — 27 logical lines through EOF; pinned SHA and 1281-byte size match; fixture inspected, not executed.

### 3057. `launcher/tests/fixtures/native-crate-demo/horizon-mixin.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1885378a1ccba0667272b47143d1e1abf26f1293`
- Size: 762 bytes
- Coverage: 1–25; complete EOF; 25 canonical logical lines, no gaps.
- Purpose: Manifest fixture exercising every part of Horizon's native-mod bundle: named injection, helper source, separate crate and post-build step.
- Dependencies / cross-references: Horizon native-mod manifest parser, `native-hooks.json`, Rust crate build and `frame.rs`.
- Implementation / logic: Declares a named `frame` hook, versioned binding to internal `Game`, an engine crate and post-build binary.
- Findings: This TOML is not Forge `mods.toml`, `META-INF/mods.toml`, or a Java mod descriptor; its fields are Horizon-specific and do not create a Java Forge mod.
- RustChunk relevance: Useful only as a reference for a separate Rust extension format; not suitable for Forge compatibility.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not present Horizon's native-mixin manifest as Forge support. Forge 47.x requires its own metadata, ModLauncher/FML lifecycle and Java 17 runtime.
- Follow-up checks: Keep manifest namespaces unambiguous; test schema versioning, unknown fields, path containment and hook compatibility if an independent native extension format is approved.
- RustChunk implementation mapping: No Forge mapping; native engine plugin schema only if separately approved.
- Evidence required before adoption: Actual Forge metadata and loader tests, plus independent native-extension threat model.
- Completion state: COMPLETE — 25 logical lines through EOF; pinned SHA and 762-byte size match; fixture inspected, not executed.

### 3058. `launcher/tests/hidden_processes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c7da90ea9c7fa88a1e0d9d8ff2092e2b0ffbf7c2`
- Size: 3813 bytes
- Coverage: 1–116; complete EOF; 116 canonical logical lines, no gaps.
- Purpose: Windows-only integration tests for hidden child processes, stdin/stdout/stderr capture, exit status, Java exporter logs, CLI streams and GUI subsystem.
- Dependencies / cross-references: Windows process creation, console probe executable, backend job runner, Java JDK wrapper, MCP stdio and PE header parsing.
- Implementation / logic: Builds a probe, runs it hidden, verifies stream capture and failure status, checks GUI subsystem and stdio transport.
- Findings: Entire test module is `cfg(windows)` and validates Windows GUI/no-console behavior; no Android child-process or Java runtime restrictions are exercised.
- RustChunk relevance: General process-test patterns are useful, but Windows console suppression is irrelevant to Android.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: RustChunk needs Android Java 17 process lifecycle and log piping, not Windows `CREATE_NO_WINDOW` behavior.
- Follow-up checks: Test Android child process start/stop, stdout/stderr buffering, stdin, exit status, app background restrictions and crash cleanup.
- RustChunk implementation mapping: No direct implementation mapping; Android process integration suite should be separate.
- Evidence required before adoption: Target-device tests with the actual Java runtime and Android process limits.
- Completion state: COMPLETE — 116 logical lines through EOF; pinned SHA and 3813-byte size match; tests inspected, not executed.

### 3059. `launcher/tests/logs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `57d660a7240d2070d3de8a857274bcff39b3cb48`
- Size: 2292 bytes
- Coverage: 1–62; complete EOF; 62 canonical logical lines, no gaps.
- Purpose: CLI integration tests for reading the latest game log tail and newest crash report in text/JSON forms.
- Dependencies / cross-references: Horizon launcher CLI, `HORIZON_HOME`, logs/latest.log, crash-reports and temp filesystem.
- Implementation / logic: Checks missing-file errors, default 100-line tail, custom line count, JSON output path and newest crash report selection.
- Findings: The file layout is generic Java Edition, but CLI binary and desktop test harness are Horizon-specific; tests do not exercise Android log access or redaction.
- RustChunk relevance: Log-tail/crash-report behavior is useful for RustChunk diagnostics.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Expose app-private Minecraft logs and crash reports through Android UI/share controls while preserving privacy and bounded reads.
- Follow-up checks: Test large logs, UTF-8 boundaries, path traversal, permission changes, concurrent log writes, JSON stability and redaction.
- RustChunk implementation mapping: RustChunk diagnostics/log repository and crash-report viewer.
- Evidence required before adoption: Android filesystem tests, crash artifact integration and privacy review.
- Completion state: COMPLETE — 62 logical lines through EOF; pinned SHA and 2292-byte size match; tests inspected, not executed.

### 3060. `launcher/tests/owner_gate.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3a7869272cb721cb7e6768969b51d590a8ec6e01`
- Size: 3620 bytes
- Coverage: 1–89; complete EOF; 89 canonical logical lines, no gaps.
- Purpose: End-to-end tests enforce a Microsoft Minecraft Java ownership gate for both CLI play and MCP play.
- Dependencies / cross-references: Launcher binary, accounts.json offline identity, installed version fixture, owner gate and MCP JSON-RPC transport.
- Implementation / logic: Creates a profile with only an offline name and asserts both play entrypoints refuse before creating a game session, preserving the offline account.
- Findings: This is direct evidence that Starling deliberately blocks offline-name play without an owning Microsoft account. Porting this gate unchanged would violate RustChunk's offline-friendly requirement; bypassing online-mode authentication must not be confused with legitimate access to online-mode servers.
- RustChunk relevance: Negative compatibility requirement: do not inherit this product gate, but keep online-mode server authentication honest and distinct from offline-mode server support.
- Decision: rewrite required; reference; authentication-sensitive
- 1.20.1 implications: RustChunk may allow offline profiles for offline-mode servers; normal online-mode/Aternos servers generally require valid Microsoft/Xbox/Minecraft authentication unless the server owner disables online mode.
- Follow-up checks: Test no-owner offline launch policy, online-mode handshake failures, offline-mode server connection and no false promise that offline names join authenticated servers.
- RustChunk implementation mapping: RustChunk account/profile policy and play preflight; no owner gate for offline local play, with explicit server-auth requirements.
- Evidence required before adoption: Separate offline-mode and online-mode multiplayer integration tests and authentication security review.
- Completion state: COMPLETE — 89 logical lines through EOF; pinned SHA and 3620-byte size match; tests inspected, not executed.

### 3061. `launcher/tests/startup.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `776d8ef15de6426ed5186313252288a8c69e3ac4`
- Size: 3565 bytes
- Coverage: 1–88; complete EOF; 88 canonical logical lines, no gaps.
- Purpose: Ignored Windows integration test measures that hardware probes run after first frame and do not open console windows.
- Dependencies / cross-references: Windows launcher binary, PowerShell probe stand-in, startup timing JSON and environment-controlled telemetry/benchmark settings.
- Implementation / logic: Starts the desktop launcher, waits for exit, verifies seven probes occur after first-frame timestamp and hidden-console flag is false.
- Findings: Test is `cfg(windows)` and explicitly ignored unless an interactive Windows desktop/GPU is available; it does not measure Minecraft chunk loading or Android startup.
- RustChunk relevance: The principle of moving expensive hardware probes off the critical startup path is useful; the implementation is Windows/GPUI-specific.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: For RustChunk, avoid blocking launcher/game startup with device probing; gather Android GPU/RAM/renderer info asynchronously and only when needed.
- Follow-up checks: Benchmark Android cold/warm startup, probe scheduling, failure handling, permission restrictions and battery impact.
- RustChunk implementation mapping: RustChunk startup coordinator and optional Android device-info probe service.
- Evidence required before adoption: Measured Android startup traces and no-regression performance tests.
- Completion state: COMPLETE — 88 logical lines through EOF; pinned SHA and 3565-byte size match; ignored test inspected, not run.

### 3062. `launcher/tests/support/console_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1ff16a10ba4fc4bc3078a8b1177f87d12e862e40`
- Size: 2338 bytes
- Coverage: 1–73; complete EOF; 73 canonical logical lines, no gaps.
- Purpose: Builds a Windows console probe executable to report window visibility and verify redirected streams or simulated hardware probes.
- Dependencies / cross-references: Win32 kernel32/user32 APIs, environment variables, stdio, `rustc` test helper and process integration tests.
- Implementation / logic: Queries console window visibility, writes optional probe timestamp, sleeps to expose blocking, emits stdout/stderr/stdin and optional failure exit code.
- Findings: This helper uses Windows-specific APIs and is only test support; it is not safe or meaningful as an Android process probe.
- RustChunk relevance: No direct implementation value.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android-specific instrumentation and process metrics for RustChunk.
- Follow-up checks: No port; replace with Android instrumentation if equivalent startup/process tests are needed.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android process instrumentation tests.
- Completion state: COMPLETE — 73 logical lines through EOF; pinned SHA and 2338-byte size match; helper inspected, not executed.

### 3063. `launcher/tests/support/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b5a72d115ee813833413047a3bbbe8e5b2542d58`
- Size: 873 bytes
- Coverage: 1–29; complete EOF; 29 canonical logical lines, no gaps.
- Purpose: Compiles and caches a Windows console-probe helper for integration tests.
- Dependencies / cross-references: `rustc`, `OnceLock`, temp directory and `horizon_launcher::process`.
- Implementation / logic: Builds console_probe.rs once per test process and returns its executable path.
- Findings: Windows-specific test harness support; not application code or Android build tooling.
- RustChunk relevance: No direct implementation value.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: RustChunk Android tests should use Gradle/NDK instrumentation rather than compiling a Windows executable at test runtime.
- Follow-up checks: Ensure test helper artifacts are isolated and cleaned up in any future platform-specific test suite.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Android CI instrumentation harness.
- Completion state: COMPLETE — 29 logical lines through EOF; pinned SHA and 873-byte size match; helper inspected, not executed.

### 3064. `licenses/create-MIT.txt`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e19475ba8e768134097e3fbb4cc518fb3bfb1825`
- Size: 1445 bytes
- Coverage: 1–26; complete EOF; 26 canonical logical lines, no gaps.
- Purpose: Records MIT license and attribution for code portions ported from the Create Minecraft mod.
- Dependencies / cross-references: Create upstream repository, `kinetic_machines.rs` and related kinetic shape/number code.
- Implementation / logic: States that Create code outside assets is MIT-licensed and requires retaining notice; explicitly excludes Create assets, which are All Rights Reserved.
- Findings: Legal boundary is material if RustChunk reuses kinetic code or assets: MIT code notice must be preserved, while Create assets cannot be copied under this license.
- RustChunk relevance: No direct game implementation; legal provenance reference only.
- Decision: adaptable; reference; legal-sensitive
- 1.20.1 implications: Before porting any Create-derived code, verify exact source files and preserve required MIT notices; do not copy excluded textures/models/audio/assets.
- Follow-up checks: Perform source-level provenance and license review for every reused code/asset; inspect upstream changes and third-party notices.
- RustChunk implementation mapping: RustChunk third-party notices and source provenance register, only for independently approved reused code.
- Evidence required before adoption: Legal review, license compatibility audit and asset-rights confirmation.
- Completion state: COMPLETE — 26 logical lines through EOF; pinned SHA and 1445-byte size match; license text inspected, not modified.

### 3065. `licenses/iris-features-MIT.txt`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `22147ae0bbc274869d3a51c55c01c2d2c4a8b58f`
- Size: 1067 bytes
- Coverage: 1–21; complete EOF; 21 canonical logical lines, no gaps.
- Purpose: Contains the MIT license notice attributed to Riley-Neeb for Iris features.
- Dependencies / cross-references: Iris feature code provenance elsewhere in Starling and third-party license inventory.
- Implementation / logic: Provides standard MIT grant, copyright notice and warranty disclaimer.
- Findings: The file alone does not identify which RustChunk-relevant files, if any, are derived from Iris; do not infer that every Iris feature or asset is covered by this license.
- RustChunk relevance: Legal metadata only; no direct implementation value.
- Decision: unnecessary; reference; legal-sensitive
- 1.20.1 implications: Any reuse must be traced to the exact covered source files and preserve the copyright/license text; verify assets separately.
- Follow-up checks: Map license to source files, verify upstream provenance and confirm no non-MIT assets are included.
- RustChunk implementation mapping: RustChunk third-party license register only if a specific covered code portion is approved.
- Evidence required before adoption: File-level provenance map and legal review before reuse.
- Completion state: COMPLETE — 21 logical lines through EOF; pinned SHA and 1067-byte size match; license text inspected, not modified.

### 3066. `mods/horizon/src/bench/java/dev/horizonlab/bench/VanillaBench.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4b3bbf14b319c31656ca2edb7124ff540e46e484`
- Size: 16175 bytes
- Coverage: 1–371; complete EOF; 371 canonical logical lines, no gaps.
- Purpose: Dev-only benchmark driver for vanilla Fabric client frame/tick timings and chunk-load readiness across repeatable scenes.
- Dependencies / cross-references: Minecraft client API, Fabric `ClientModInitializer`, Mixin frame/tick hooks, script JSON, creative world, local/server connection and JSONL output.
- Implementation / logic: Runs deterministic teleport/load/settle/flight phases, waits until nearby chunks exist and rendered sections are complete and stable for one second, records frame/tick durations, chunk counts and renderer metadata.
- Findings: Benchmark is tied to Minecraft 26.3 and Fabric entrypoint/mixins; its `chunksAround` check measures a circular area inside render distance minus one and `hasRenderedAllSections`, then waits for loaded count stability. Because flight begins after chunks are ready, load latency is reported separately from steady-state frame time. Server mode requires operator permission for commands.
- RustChunk relevance: Highly relevant benchmark methodology for distinguishing chunk-load latency from steady-state FPS, but code/API and fixtures must be retargeted.
- Decision: adaptable; reference; version-specific; platform-specific
- 1.20.1 implications: Build a dedicated Minecraft 1.20.1 Forge 47.x client benchmark mod/harness on Java 17, with Android frame pacing/GPU metrics and scene scripts; do not ship this Fabric 26.3 driver.
- Follow-up checks: Add separate metrics for time-to-first-chunk, all-visible-chunks-ready, compiled/rendered sections, missing chunks during movement, frame-time percentiles, tick time, memory and renderer stalls; test both local and Aternos-like servers.
- RustChunk implementation mapping: RustChunk benchmark specification and target-specific Forge instrumentation module, not launcher production code.
- Evidence required before adoption: Reproducible 1.20.1 Forge scenes, validated chunk readiness definition, Android device traces and repeatable frame/tick reports.
- Completion state: COMPLETE — 371 logical lines through EOF; pinned SHA and 16175-byte size match; benchmark code inspected, not executed.

### 3067. `mods/horizon/src/bench/java/dev/horizonlab/bench/mixin/FrameHookMixin.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6c9ac676d10b96331411b6cc0a6115c8504d2e6c`
- Size: 593 bytes
- Coverage: 1–17; complete EOF; 17 canonical logical lines, no gaps.
- Purpose: Injects the vanilla benchmark driver at the head of `Minecraft.runTick` to sample frame timing/state.
- Dependencies / cross-references: Sponge Mixin, Minecraft 26.3 `Minecraft.runTick` and `VanillaBench.frame`.
- Implementation / logic: Calls benchmark frame callback once per game run tick before the client tick work.
- Findings: Mixin target and mapping are version-specific; this is not a Forge 1.20.1 patch target or Android renderer callback.
- RustChunk relevance: A frame hook is relevant to a custom benchmark, but must use the correct 1.20.1 Forge mappings and avoid double-counting frame/tick callbacks.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use a Forge 47.x-compatible Mixin/event hook on Java 17; confirm whether callback corresponds to rendered frame or game tick on target version.
- Follow-up checks: Verify injection point, callback frequency, render thread safety, mod conflicts and instrumentation overhead.
- RustChunk implementation mapping: RustChunk benchmark instrumentation hook in a dev-only Forge client mod.
- Evidence required before adoption: Runtime validation against Forge 1.20.1 mappings and frame pacing traces.
- Completion state: COMPLETE — 17 logical lines through EOF; pinned SHA and 593-byte size match; mixin inspected, not executed.

### 3068. `mods/horizon/src/bench/java/dev/horizonlab/bench/mixin/TickHookMixin.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7d88a3fec1c5e6e601567b1decb0deab3ef9b080`
- Size: 788 bytes
- Coverage: 1–23; complete EOF; 23 canonical logical lines, no gaps.
- Purpose: Times client tick duration around `Minecraft.tick` for benchmark reports.
- Dependencies / cross-references: Sponge Mixin, Minecraft 26.3 tick method and `VanillaBench.tick`.
- Implementation / logic: Records `System.nanoTime()` at method head and stores elapsed milliseconds at return while benchmark flight phase is active.
- Findings: Method mapping and lifecycle are version-specific; tick duration is distinct from frame duration and must not be conflated with FPS.
- RustChunk relevance: Useful benchmark metric separation, but hook must be ported to the exact Forge 1.20.1 client.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Record both render-frame time and client tick time in RustChunk diagnostics; use Forge-compatible mappings and avoid intrusive hooks in release builds.
- Follow-up checks: Test tick callback pairing on exceptions, pauses, game screens, integrated server load and mixin conflicts.
- RustChunk implementation mapping: RustChunk benchmark tick instrumentation.
- Evidence required before adoption: Forge 47.x hook tests and Android traces with tick/frame distributions.
- Completion state: COMPLETE — 23 logical lines through EOF; pinned SHA and 788-byte size match; mixin inspected, not executed.

### 3069. `mods/horizon/src/bench/resources/fabric.mod.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `789d9cfacd728d1db4cb2c1cc01edf4150f2d314`
- Size: 501 bytes
- Coverage: 1–21; complete EOF; 21 canonical logical lines, no gaps.
- Purpose: Declares the development benchmark mod as a client-only Fabric mod.
- Dependencies / cross-references: Fabric Loader, `VanillaBench` client entrypoint, mixin config and Minecraft version metadata.
- Implementation / logic: Registers benchmark entrypoint and mixins; dependency range pins Fabric Loader 0.19.5+ and Minecraft 26.3.
- Findings: Explicitly Fabric-only, Minecraft 26.3; not a Forge `mods.toml` or NeoForge metadata file.
- RustChunk relevance: No direct reuse for target Forge.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: RustChunk benchmark harness needs Forge 47.x metadata and Java 17; do not package this Fabric descriptor.
- Follow-up checks: Validate mod metadata against Forge 47.4.5 and ensure dev-only benchmark artifacts are excluded from production APK.
- RustChunk implementation mapping: No mapping; create independent Forge benchmark mod descriptor.
- Evidence required before adoption: Forge loader discovery and production artifact exclusion tests.
- Completion state: COMPLETE — 21 logical lines through EOF; pinned SHA and 501-byte size match; metadata inspected, not executed.

### 3070. `mods/horizon/src/bench/resources/horizon_bench.mixins.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `96415ac5168add1b1ff691bdacdc392a4bcc7e54`
- Size: 208 bytes
- Coverage: 1–12; complete EOF; 12 canonical logical lines, no gaps.
- Purpose: Registers the two benchmark Mixin hooks for the Fabric client benchmark mod.
- Dependencies / cross-references: Sponge Mixin, `FrameHookMixin`, `TickHookMixin` and Java compatibility level.
- Implementation / logic: Marks mixins required, sets package, `JAVA_25` compatibility and default injection requirement 1.
- Findings: Java 25 and target Minecraft 26.3 are incompatible with RustChunk's Java 17 / Minecraft 1.20.1 target; required injection failures would abort loading.
- RustChunk relevance: Only the concept of a minimal dev-only mixin manifest is reusable.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Create a new Forge 47.x/Java 17 benchmark manifest with mappings and injection points verified for 1.20.1.
- Follow-up checks: Check target class/method names, injection count, optional-mod conflicts and Java 17 bytecode compatibility.
- RustChunk implementation mapping: No direct mapping; new Forge benchmark manifest only.
- Evidence required before adoption: Forge 1.20.1 runtime load tests and bytecode/version verification.
- Completion state: COMPLETE — 12 logical lines through EOF; pinned SHA and 208-byte size match; metadata inspected, not executed.

### 3071. `mods/horizon/src/neoforge-bench/java/dev/horizonlab/neoforgebench/NeoForgeBench.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5992ed27a9604dbc77857a7b3e9e91c651a5ea32`
- Size: 11268 bytes
- Coverage: 1–275; complete EOF; 275 canonical logical lines, no gaps.
- Purpose: Dev-only NeoForge client benchmark driver using render-frame events for scripted chunk-load and steady-flight scenes.
- Dependencies / cross-references: NeoForge `@Mod` client distribution, `RenderFrameEvent.Pre`, Minecraft 1.21.1 client API, script JSON and JSONL output.
- Implementation / logic: Creates a creative world, teleports between scenes, waits for nearby chunks and compiled sections to stabilize, records frame times and renderer details, then stops the client.
- Findings: Targets NeoForge 1.21.1, not Forge 1.20.1; unlike VanillaBench it omits tick timings, server mode, setup/cleanup commands and per-scene settle override. It shares the chunk readiness method and one-second stability window.
- RustChunk relevance: The event-based benchmark structure is the closest loader-adjacent reference in this checkpoint, but cannot be assumed binary/source compatible with Forge 47.x.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use the benchmark state machine and chunk-ready metrics as a design reference; port to Forge 47.4.5, Minecraft 1.20.1 and Java 17, then add Android instrumentation.
- Follow-up checks: Verify event timing and client thread, add tick timing, remote server scenes, per-scene setup/cleanup, failure timeout, and Android thermal/battery controls.
- RustChunk implementation mapping: RustChunk dev-only Forge benchmark harness specification and implementation.
- Evidence required before adoption: Actual Forge 47.4.5 build/run on Android and reproducible chunk/FPS traces.
- Completion state: COMPLETE — 275 logical lines through EOF; pinned SHA and 11268-byte size match; benchmark inspected, not executed.

### 3072. `mods/horizon/src/neoforge-bench/resources/META-INF/neoforge.mods.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0fb0c30292c9854f7a55881ea3d88a01f5c9ba7c`
- Size: 402 bytes
- Coverage: 1–11; complete EOF; 11 canonical logical lines, no gaps.
- Purpose: NeoForge metadata for the dev-only benchmark driver.
- Dependencies / cross-references: NeoForge JavaFML loader, benchmark mod id and build/test tooling.
- Implementation / logic: Declares `javafml`, loaderVersion `[4,)`, MIT license and dev-only `horizon_bench` mod identity.
- Findings: Metadata is a NeoForge development mod descriptor; it does not define Forge 47.x compatibility or target Minecraft version by itself.
- RustChunk relevance: Useful descriptor concept only.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Create a dedicated Forge 47.x descriptor with correct `modLoader`, loader range and Minecraft dependency; never reuse this as-is.
- Follow-up checks: Verify exact Forge 47.4.5 metadata syntax, dependency ranges and that benchmark JAR is excluded from production.
- RustChunk implementation mapping: No direct mapping; new Forge-only dev descriptor.
- Evidence required before adoption: Forge 47.4.5 mod discovery and packaging tests.
- Completion state: COMPLETE — 11 logical lines through EOF; pinned SHA and 402-byte size match; metadata inspected, not executed.

### 3073. `mods/horizon/src/neoforge-oracle/java/dev/horizonlab/neoforge/FactsExporter.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8bd9e28e4c5dcb3456f22a778870590bdf866e58`
- Size: 23682 bytes
- Coverage: 1–400, 401–496; consecutive, gap-free; 496 logical lines through EOF.
- Purpose: Dev-only NeoForge server oracle exporting block/item/recipe/tag facts for mod-port parity research.
- Dependencies / cross-references: NeoForge `ServerStartedEvent`, Minecraft 1.21.1 registries/NBT/data components, loot engine, block states/shapes, JSON writer and namespace allowlist.
- Implementation / logic: Enumerates vanilla blocks and selected mod namespaces, records state properties/physics/shape/sounds/drops/placement, item components/armor, recipes and tags, then halts the server.
- Findings: Targets NeoForge 1.21.1 and its data-component APIs; not a client renderer/chunk benchmark. Uses reflection to inspect private BlockSetType/WoodType fields, catches broad Throwable and exits process on export failure; exports may be very large and server-mutating (places temporary blocks).
- RustChunk relevance: Could serve as a reference for a dev-only parity oracle when porting specific mods, but is not part of RustChunk launcher/runtime or Forge client loader.
- Decision: adaptable; reference; version-specific; security-sensitive
- 1.20.1 implications: For 1.20.1 mod compatibility, build a target-version oracle around Forge 47.x server registries and the 1.20.1 item/NBT model; do not copy 1.21.1 component assumptions.
- Follow-up checks: Bound output size, run in isolated disposable world, avoid private reflection where possible, record incomplete exports, compare version-specific schemas and ensure world state is restored.
- RustChunk implementation mapping: Separate RustChunk mod-compatibility research harness only if mod-port/parity work is approved.
- Evidence required before adoption: Disposable Forge 1.20.1 server tests, deterministic exports, output limits and reflection/security review.
- Completion state: COMPLETE — 496 logical lines read in 1–400 and 401–496; pinned SHA and 23682-byte size match; exporter inspected, not executed.

### 3074. `mods/horizon/src/neoforge-oracle/resources/META-INF/neoforge.mods.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `006875e950c4f1ab01433bd947ecbfb1c6ff9db4`
- Size: 394 bytes
- Coverage: 1–11; complete EOF; 11 canonical logical lines, no gaps.
- Purpose: NeoForge metadata for the dev-only mod parity facts exporter.
- Dependencies / cross-references: NeoForge JavaFML loader, `horizon_oracle` mod id and oracle build script.
- Implementation / logic: Declares JavaFML, loader range `[4,)`, Apache-2.0 license and development-only exporter identity.
- Findings: This is a NeoForge dev descriptor, not Forge 47.x metadata and not part of the client APK.
- RustChunk relevance: No direct runtime value.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: If parity research is approved, create a Forge 47.x-specific dev-only descriptor and keep it out of the shipped launcher/client.
- Follow-up checks: Verify mod metadata, licensing of exporter dependencies and no accidental production inclusion.
- RustChunk implementation mapping: No baseline mapping; optional dev oracle packaging only.
- Evidence required before adoption: Forge 1.20.1 discovery and release-artifact exclusion tests.
- Completion state: COMPLETE — 11 logical lines through EOF; pinned SHA and 394-byte size match; metadata inspected, not executed.

### 3075. `mods/horizon/src/test/java/dev/horizonlab/parity/EntityTranscript.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `09c343f165d31b27dfaab8009535ed564efb7133`
- Size: 8884 bytes
- Coverage: 1–181; complete EOF; 181 canonical logical lines, no gaps.
- Purpose: Builds a deterministic transcript of server-tracked non-player entities by replaying clientbound entity packets into a lightweight state model.
- Dependencies / cross-references: Minecraft 26.3 protocol packet classes, entity registries, `VecDeltaCodec`, attributes, metadata serializers, equipment, passengers and `VanillaProtocolClient` helpers.
- Implementation / logic: Tracks spawn/remove, entity data, equipment, attributes, passenger lists, position/rotation/motion/teleport and projectile power; names entities by type and spawn block instead of unstable server entity IDs.
- Findings: Uses 26.3 packet/API types and switch-pattern syntax; players are excluded from tracked entities, while self attributes are separate. This is a protocol/parity test helper, not production networking or Forge event handling.
- RustChunk relevance: Entity packet transcript is a strong reference for validating server-client protocol state and multiplayer entity synchronization, but needs a 1.20.1 protocol-763 rewrite.
- Decision: adaptable; reference; version-specific; network-sensitive
- 1.20.1 implications: For Minecraft 1.20.1, build packet transcript tests against protocol 763 and actual Forge/vanilla clientbound packets; keep entity IDs and player identity handling faithful to protocol semantics.
- Follow-up checks: Test spawn metadata pairing, relative movement codec, teleport, removal/reuse of entity IDs, passengers, attributes, players, modded entity data and packet ordering.
- RustChunk implementation mapping: RustChunk protocol/network test harness for entity synchronization, not launcher source.
- Evidence required before adoption: Protocol-763 fixtures, real 1.20.1 server packet captures and deterministic replay tests.
- Completion state: COMPLETE — 181 logical lines through EOF; pinned SHA and 8884-byte size match; test helper inspected, not executed.


## Checkpoint 3076–3100 — COMPLETE — exceptional authorization exhausted

### 3076. `mods/horizon/src/test/java/dev/horizonlab/parity/KnockbackClient.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ddc06af0f4fe64f90faac805626bebded748a785`
- Size: 26643 bytes
- Coverage: 1–400, 401–522; consecutive, gap-free; 522 logical lines through EOF.
- Purpose: Runs a minimal vanilla-protocol client against a local test server to record knockback velocity, movement and collision outcomes.
- Dependencies / cross-references: Minecraft 26.3 protocol packets, raw TCP socket/framing, offline UUID, entity movement, player input, attack/knockback packets and local parity scripts.
- Implementation / logic: Logs spawn/teleport/motion/velocity/position updates, sends movement/attack inputs, answers keep-alives and compares server-applied knockback against a transcript.
- Findings: Dev-only hand-written socket client uses offline authentication and current 26.3 packet APIs. Frame length is allocated directly from peer VarInt and compressed payload length is trusted; no production-grade bounds. Online-mode server auth is not satisfied by offline UUID.
- RustChunk relevance: Useful as a deterministic server-client movement/knockback parity harness, not as a production network stack or Forge loader.
- Decision: adaptable; reference; version-specific; network-sensitive
- 1.20.1 implications: Rebuild protocol packet handling for Minecraft 1.20.1 protocol 763 and Java 17; do not reuse raw frame parser without strict size/decompression limits.
- Follow-up checks: Add malicious frame/compression tests, teleport/relative-move ordering, movement validation, collision scenes, online-mode auth and Forge-modified packet coverage.
- RustChunk implementation mapping: RustChunk dev-only multiplayer movement/knockback parity harness.
- Evidence required before adoption: Protocol-763 packet captures, bounded network decoder tests and reproducible Forge 47.x server traces.
- Completion state: COMPLETE — 522 logical lines read in 1–400 and 401–522; pinned SHA and 26643-byte size match; test client inspected, not executed.

### 3077. `mods/horizon/src/test/java/dev/horizonlab/parity/LocatorBarClient.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9f964c197d40857e35e27894c6d04c72c18dbf82`
- Size: 38214 bytes
- Coverage: 1–400, 401–686; consecutive, gap-free; 686 logical lines through EOF.
- Purpose: Implements a configurable raw-protocol test client to observe locator/waypoint, entities, player state, chat, hooks, boats, chunk batches and tick-step packets.
- Dependencies / cross-references: Minecraft 26.3 protocol codecs, raw TCP socket, offline UUID, configuration/game packet states, entity registry, waypoint packets and transcript scripts.
- Implementation / logic: Performs handshake/login/configuration, tracks player/entity IDs and positions, records waypoint/chat/effect/boat/hook events, acknowledges keep-alive/ping/teleport/chunk batches and emits deterministic transcripts.
- Findings: This is a test harness using 26.3 packet classes and offline-mode UUIDs, not a production multiplayer client. Frame and decompressed lengths are peer-controlled allocations without strict upper bounds; `vanilla` brand is synthetic. It does not implement Forge mod handshake.
- RustChunk relevance: Strong reference for protocol-state parity and packet transcript testing; must be rewritten for 1.20.1 protocol 763 and hardened before any production networking.
- Decision: adaptable; reference; version-specific; network-sensitive
- 1.20.1 implications: Use the transcript scenarios to test RustChunk's 1.20.1 client protocol, but implement standard Forge/FML negotiation separately and authenticate online-mode servers correctly.
- Follow-up checks: Bound frame/decompression sizes, test packet state transitions, entity ID reuse, waypoint support, unknown payloads, disconnects, chat signatures and Forge handshake sequence.
- RustChunk implementation mapping: RustChunk dev-only protocol parity client and packet-state test harness.
- Evidence required before adoption: Protocol-763 captures, malformed packet tests, Forge 47.x handshake tests and online/offline authentication integration.
- Completion state: COMPLETE — 686 logical lines read in 1–400 and 401–686; pinned SHA and 38214-byte size match; harness inspected, not executed.

### 3078. `mods/horizon/src/test/java/dev/horizonlab/parity/OfflineLanHost.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a643805b790a7e66bd9e05cc8ee2a899ebb3df1f`
- Size: 1906 bytes
- Coverage: 1–42; complete EOF; 42 canonical logical lines, no gaps.
- Purpose: Starts an unchanged vanilla client and opens its integrated single-player world to LAN with authentication disabled for offline protocol test clients.
- Dependencies / cross-references: Minecraft integrated server, `MinecraftServer.MultiplayerScope.LAN`, test JVM property `horizon.lanPort` and `VanillaProtocolClient` scripts.
- Implementation / logic: Polls until the single-player server is ready, calls `setUsesAuthentication(false)` and publishes the world to LAN with commands enabled.
- Findings: Authentication is disabled only to enable a local test harness to connect using offline UUIDs. This is not an online-mode authentication bypass for public servers and must not be generalized to Aternos.
- RustChunk relevance: Useful for isolated offline-mode LAN regression tests, but dangerous if reused as a general multiplayer configuration.
- Decision: adaptable; reference; security-sensitive; version-specific
- 1.20.1 implications: RustChunk may test offline-mode LAN servers separately; ordinary online-mode/Aternos servers still require legitimate account authentication unless server owners disable it.
- Follow-up checks: Ensure test server binds only to intended LAN interface, is disposable, restores authentication defaults and cannot be enabled in production builds.
- RustChunk implementation mapping: RustChunk offline-mode local integration-test fixture only.
- Evidence required before adoption: Isolated local network tests and proof that production build cannot disable authentication silently.
- Completion state: COMPLETE — 42 logical lines through EOF; pinned SHA and 1906-byte size match; test host inspected, not executed.

### 3079. `mods/horizon/src/test/java/dev/horizonlab/parity/ParityData.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `34cb8d05e9a4a41f0b34ed3a9bb313c659e4a8a9`
- Size: 6475 bytes
- Coverage: 1–135; complete EOF; 135 canonical logical lines, no gaps.
- Purpose: Bootstraps vanilla registries/data pack, exposes registry/NBT codecs and creates a lightweight stand-in ServerLevel for parity tools.
- Dependencies / cross-references: Vanilla server data pack, RegistryLayer, TagLoader, ReloadableServerRegistries, NBT/ItemStack codecs, `Unsafe` and private Level fields.
- Implementation / logic: Loads vanilla registries and tags, applies components, creates RegistryOps, allocates an unconstructed ServerLevel and injects registryAccess; canonicalizes NBT with sorted keys and type markers.
- Findings: Dev-only test support uses `sun.misc.Unsafe`, reflective private-field writes and direct vanilla bootstrap; brittle across mappings/versions and unsuitable for production runtime.
- RustChunk relevance: Useful shared setup and canonical serialization approach for a Rust-vs-vanilla oracle harness.
- Decision: adaptable; reference; version-specific; security-sensitive
- 1.20.1 implications: For 1.20.1, use exact registries/NBT codecs and a dedicated test runtime; never depend on 26.3 private field names or Unsafe in launcher/game production.
- Follow-up checks: Version-pin registry setup, eliminate Unsafe where possible, test cyclic/large NBT and deterministic canonical encoding, isolate test classpath.
- RustChunk implementation mapping: RustChunk test-only vanilla oracle support module or JVM harness.
- Evidence required before adoption: Protocol/version-pinned registry fixtures and deterministic NBT parity tests.
- Completion state: COMPLETE — 135 logical lines through EOF; pinned SHA and 6475-byte size match; test support inspected, not executed.

### 3080. `mods/horizon/src/test/java/dev/horizonlab/parity/ParityFont.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3390b37f6737cbdd71028b324ac5a48a4bdcd597`
- Size: 5822 bytes
- Coverage: 1–132; complete EOF; 132 canonical logical lines, no gaps.
- Purpose: Loads vanilla font provider definitions and reports glyph metrics without stitching glyphs or using a GPU.
- Dependencies / cross-references: Minecraft font provider codecs, vanilla resource pack, GlyphProvider, GlyphSource, Font and font reference resolution.
- Implementation / logic: Loads space/default providers, resolves references with cycle detection, returns metrics-only BakedGlyphs and uses six-pixel fallback for missing glyphs.
- Findings: Uses Minecraft 26.3 font APIs and reflective `ClientPackSource`; it is a test oracle, not an Android font renderer. Metrics-only output cannot validate GPU atlas stitching or visual rasterization.
- RustChunk relevance: Useful for text-width/font-metric parity and localization layout tests; not a renderer optimization.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: If RustChunk needs accurate 1.20.1 UI/game text measurements, query target Minecraft font providers; do not infer glyph rasterization performance from this metrics-only oracle.
- Follow-up checks: Test fallback providers, cycles, Unicode/RTL shaping, resource-pack font overrides, atlas generation and actual Android rendering.
- RustChunk implementation mapping: RustChunk optional font-metric parity tests for target 1.20.1.
- Evidence required before adoption: 1.20.1 font-provider fixtures and pixel/metric comparisons on Android.
- Completion state: COMPLETE — 132 logical lines through EOF; pinned SHA and 5822-byte size match; test helper inspected, not executed.

### 3081. `mods/horizon/src/test/java/dev/horizonlab/parity/PrepareWorker.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `26104bbe1e2f351731d79d25ddf87bf630ec8c04`
- Size: 3709 bytes
- Coverage: 1–69; complete EOF; 69 canonical logical lines, no gaps.
- Purpose: Runs multiple parity oracle tasks in one long-lived JVM with separate server/client classpaths and JSON-lines control transport.
- Dependencies / cross-references: Vanilla Bootstrap, BuiltInRegistries, registry tag reload, reflection, JSON task input, classpath worker process and file descriptor stdout.
- Implementation / logic: Validates classpath classes, bootstraps once, resets registry tags between tasks, redirects task logs and invokes named oracle main classes reflectively.
- Findings: Test-only worker executes task-selected classes and writes task-selected log paths; reflection and process-exit behavior are unsuitable as a user-facing service. One JVM/classpath per worker is a useful isolation rule.
- RustChunk relevance: Useful for a separate parity CI harness, not a production launcher subsystem.
- Decision: adaptable; reference; security-sensitive; version-specific
- 1.20.1 implications: Keep any test worker isolated from the Android launcher and game process; use strict task allowlists and app-owned temp paths.
- Follow-up checks: Validate task JSON schema, class allowlist, path containment, worker crash recovery, registry reset completeness and stdout/stderr separation.
- RustChunk implementation mapping: RustChunk dev/test parity worker only, not app runtime.
- Evidence required before adoption: Isolated CI runner, sandbox/path tests and reproducible task execution.
- Completion state: COMPLETE — 69 logical lines through EOF; pinned SHA and 3709-byte size match; worker inspected, not run.

### 3082. `mods/horizon/src/test/java/dev/horizonlab/parity/SecureChatClient.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `49e4b2254cd1a04986494bf680fee8e5b0ad5d2c`
- Size: 35191 bytes
- Coverage: 1–400, 401–689; consecutive, gap-free; 689 logical lines through EOF.
- Purpose: Simulates multiple vanilla clients to verify secure-chat signatures, session keys, last-seen tracking, commands, acknowledgements and enforce-secure-profile behavior.
- Dependencies / cross-references: Minecraft 26.3 protocol packets, RSA profile keys, mock session-service HTTP endpoint, Cipher streams, SignedMessageChain, LastSeenMessagesTracker and signature validators.
- Implementation / logic: Creates mock service key pairs, joins offline/online test scenarios, signs chat/command bodies, validates incoming signatures and records signature/session/transcript outcomes.
- Findings: Uses mock access tokens/session service and synthetic player keys; it does not perform real Microsoft/Xbox authentication. Protocol packet types and chat-session details are 26.3-specific; raw compression framing allocates untrusted lengths.
- RustChunk relevance: High-value reference for testing secure chat and online-mode authentication semantics; not a production account/login implementation.
- Decision: adaptable; reference; authentication-sensitive; version-specific; network-sensitive
- 1.20.1 implications: RustChunk 1.20.1 secure chat support must follow protocol 763 and official account/profile-key flow; offline-mode test chat does not imply online-mode server access.
- Follow-up checks: Test profile-key expiry/signature, borrowed keys, secure-profile enforcement, last-seen chain, command argument signatures, compression limits and real authenticated login.
- RustChunk implementation mapping: RustChunk dev-only secure-chat protocol test suite.
- Evidence required before adoption: Protocol-763 secure-chat packet captures, official auth integration and cryptographic review.
- Completion state: COMPLETE — 689 logical lines read in 1–400 and 401–689; pinned SHA and 35191-byte size match; harness inspected, not executed.

### 3083. `mods/horizon/src/test/java/dev/horizonlab/parity/ServerPackClient.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ac59104d9b4eee372fdf215cae69a645fb51607a`
- Size: 17394 bytes
- Coverage: 1–378; complete EOF; 378 canonical logical lines, no gaps.
- Purpose: Simulates vanilla clients responding to server resource-pack offers during configuration and play.
- Dependencies / cross-references: Minecraft 26.3 resource-pack packets, raw TCP framing/compression, offline UUID, configuration/game protocol codecs and scripted actions.
- Implementation / logic: Joins a test server, records configuration packet runs and pack offer UUID/URL/hash/required/prompt, keeps connection alive during delay and sends selected ACCEPTED/DECLINED/FAILED actions.
- Findings: The harness records server offers and responses but does not download, hash-verify or apply the pack. It uses offline-mode identity and unbounded peer frame/decompression lengths; 26.3 packet API is not 1.20.1.
- RustChunk relevance: Useful to validate server resource-pack handshake and prompt/action sequencing, not a pack downloader or production network implementation.
- Decision: adaptable; reference; version-specific; network-sensitive
- 1.20.1 implications: Implement 1.20.1 protocol-763 resource-pack negotiation and separately verify downloaded pack hash/size, HTTPS policy, storage and user consent.
- Follow-up checks: Test required/optional prompts, delayed response, duplicate offers, URL/hash validation, redirect policy, decompression bounds, disconnect and 1.20.1 packet ordering.
- RustChunk implementation mapping: RustChunk resource-pack protocol integration test harness and separate secure downloader.
- Evidence required before adoption: Real Forge 47.x client/server traces and malicious URL/hash/size tests.
- Completion state: COMPLETE — 378 logical lines through EOF; pinned SHA and 17394-byte size match; harness inspected, not executed.

### 3084. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaActiveEffectTickOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b2863fad467e83cc197c1c6ab84956fe1b2b28c9`
- Size: 20514 bytes
- Coverage: 1–101; complete EOF; 101 canonical logical lines, no gaps.
- Purpose: Executes vanilla LivingEntity active-effect tick logic over synthetic entities, levels, random sources and observable callbacks.
- Dependencies / cross-references: MobEffect/MobEffectInstance, SynchedEntityData, particle/sound events, Unsafe allocation, reflection into private effect/blend fields and deterministic RNG.
- Implementation / logic: Models effect maps/hidden chains, tick callbacks, particles/sounds, random draws, mutations and concurrent modification/failure points; emits deterministic trace.
- Findings: Uses extensive Unsafe/private-field access and current 26.3 effect APIs; test oracle only. It does not run a real game world or Forge event bus.
- RustChunk relevance: Useful as a model for high-fidelity vanilla behavior parity tests, but unrelated to chunk renderer performance.
- Decision: adaptable; reference; version-specific; security-sensitive
- 1.20.1 implications: If RustChunk later ports game logic, use 1.20.1 effect semantics and a safe isolated oracle; do not embed Unsafe reflection in the launcher.
- Follow-up checks: Replace private-field assumptions with test adapters where possible, verify deterministic RNG and cover Forge event/modifier hooks.
- RustChunk implementation mapping: RustChunk dev-only status-effect parity suite, outside production launcher.
- Evidence required before adoption: Protocol/version-pinned vanilla traces and Forge event integration tests.
- Completion state: COMPLETE — 101 logical lines through EOF; pinned SHA and 20514-byte size match; oracle inspected, not executed.

### 3085. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaActiveEffectsOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0aa385cb0af9148237004760601f68e0c53b4150`
- Size: 8970 bytes
- Coverage: 1–44; complete EOF; 44 canonical logical lines, no gaps.
- Purpose: Tests vanilla LivingEntity active-effect storage, add/update/remove/clear and blend-factor behavior.
- Dependencies / cross-references: MobEffectInstance, AttributeMap, Unsafe allocation, private fields and synthetic Actor/ServerLevel.
- Implementation / logic: Builds controlled effect holders/instances, mutates identity-linked maps and records callbacks, hidden-effect chains and state after each operation.
- Findings: Dev-only 26.3 reflection harness; does not represent a supported public API or Forge-specific mod event behavior.
- RustChunk relevance: Reference for differential effect-storage tests only.
- Decision: adaptable; reference; version-specific; security-sensitive
- 1.20.1 implications: Use exact 1.20.1 effect lifecycle and Forge 47.x hooks if mod compatibility requires effect parity.
- Follow-up checks: Test duplicate holders, identity/hash collisions, hidden chain, callbacks, removal and private-field changes across mappings.
- RustChunk implementation mapping: RustChunk test-only effect parity oracle.
- Evidence required before adoption: Target 1.20.1 expected traces and real Forge behavior tests.
- Completion state: COMPLETE — 44 logical lines through EOF; pinned SHA and 8970-byte size match; oracle inspected, not executed.

### 3086. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaAmbientOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d20c5f8921381e5495d7786c6686f47426d9f772`
- Size: 12910 bytes
- Coverage: 1–249; complete EOF; 249 canonical logical lines, no gaps.
- Purpose: Runs vanilla local-player underwater, bubble-column and biome ambient sound handlers through a real SoundEngine over synthetic client scenes.
- Dependencies / cross-references: Minecraft ClientLevel/LocalPlayer, SoundEngine, ambient handlers, environment attributes, seeded random, brightness and block-column state.
- Implementation / logic: Scripts player position/eye-water/spectator/removal/light, biome ambient definitions and volume; records sound instances, fades, moodiness, delay and random state.
- Findings: High-fidelity but dev-only audio oracle for 26.3; it constructs synthetic ClientLevel/LocalPlayer via reflection and does not validate Android audio output/latency.
- RustChunk relevance: Useful for sound behavior parity if RustChunk ports gameplay/audio semantics; not directly relevant to chunk load or FPS.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Any audio parity work must target 1.20.1 sound handlers and Forge events; Android audio backend needs separate latency/device testing.
- Follow-up checks: Test underwater transitions, bubble columns, biome loop/mood, random determinism, sound category volume and Android audio lifecycle.
- RustChunk implementation mapping: RustChunk optional audio/gameplay parity tests.
- Evidence required before adoption: Target-version sound traces and Android device audio tests.
- Completion state: COMPLETE — 249 logical lines through EOF; pinned SHA and 12910-byte size match; oracle inspected, not executed.

### 3087. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaAnchorOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9d8c4ec7011574078f31c86d3f5fa11e560cb2fa`
- Size: 3829 bytes
- Coverage: 1–57; complete EOF; 57 canonical logical lines, no gaps.
- Purpose: Checks vanilla vertical-anchor resolution and codec behavior under varied world height/sea-level/generator contexts.
- Dependencies / cross-references: VerticalAnchor.CODEC, WorldGenerationContext, NoiseGeneratorSettings, NoiseRouter, DensityFunctions and synthetic LevelAccessor.
- Implementation / logic: Constructs generator or accessor contexts, resolves bottom/top/sea/absolute/above-bottom/below-top/relative-sea anchors and parses/encodes JSON forms.
- Findings: Version 26.3 world-generation codec oracle; not a client chunk scheduler or renderer test.
- RustChunk relevance: Useful only for world-generation parity.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use Minecraft 1.20.1 world-height and anchor codec semantics if RustChunk ports worldgen or mod data; this does not prove Forge client support.
- Follow-up checks: Test negative minY, height bounds, sea-level edge, malformed codec and retrogen contexts.
- RustChunk implementation mapping: RustChunk optional worldgen codec parity suite.
- Evidence required before adoption: Target 1.20.1 codec fixtures and modded dimension tests.
- Completion state: COMPLETE — 57 logical lines through EOF; pinned SHA and 3829-byte size match; oracle inspected, not executed.

### 3088. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaArrowOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4168db9aae4c2af847c470fa9d1c41033bf84614`
- Size: 19912 bytes
- Coverage: 1–233; complete EOF; 233 canonical logical lines, no gaps.
- Purpose: Runs real vanilla Arrow/AbstractArrow tick and collision behavior against a synthetic resident scene.
- Dependencies / cross-references: Arrow/AbstractArrow, block/fluid collision, projectile physics, DamageSources, game events, particles/sounds, inventory pickup and `VanillaSceneCollisionOracle`.
- Implementation / logic: Exercises flight, water/bubble inertia, fluid currents, sticking/despawn, fire/lava/water, TNT/campfires/candles, entity hit/deflection and pickup rules; records event order and velocity.
- Findings: Uses Unsafe allocation and hand-built ServerLevel/Entity/Inventory shells; test oracle for 26.3 only. Does not model a complete live server tick or Forge projectile modifications.
- RustChunk relevance: Useful for game-physics parity tests, not chunk loading optimization.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: If RustChunk implements any native gameplay/physics behavior, compare with 1.20.1 vanilla/Forge; otherwise preserve Java game behavior via Forge runtime.
- Follow-up checks: Test collision ordering, fluid tags, RNG, entity damage/deflection, pickup inventory and modded projectile hooks.
- RustChunk implementation mapping: RustChunk optional projectile/physics parity harness.
- Evidence required before adoption: Target 1.20.1 server/client traces and Forge mod interaction tests.
- Completion state: COMPLETE — 233 logical lines through EOF; pinned SHA and 19912-byte size match; oracle inspected, not executed.

### 3089. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaAttributeHealthOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `18c14f659076189e76932c88026f107599507f83`
- Size: 7785 bytes
- Coverage: 1–53; complete EOF; 53 canonical logical lines, no gaps.
- Purpose: Tests vanilla entity attribute limits, synced health, absorption and hunger regeneration/exhaustion.
- Dependencies / cross-references: AttributeMap, Attributes.MAX_HEALTH/MAX_ABSORPTION, FoodData, GameRules, SynchedEntityData and Unsafe/reflection.
- Implementation / logic: Builds synthetic ServerPlayer/Level, changes base/modifier values, runs health/heal/hurt/absorption/food operations and records packed dirty state.
- Findings: Dev-only 26.3 oracle uses private fields and Unsafe; not a supported game runtime extension.
- RustChunk relevance: Useful for gameplay state parity, unrelated to renderer/chunk scheduling.
- Decision: adaptable; reference; version-specific; security-sensitive
- 1.20.1 implications: Port only if RustChunk implements native gameplay state; use exact 1.20.1 health/food semantics and Forge events.
- Follow-up checks: Test modifier bounds, health sync, absorption, regeneration rule, difficulty, death and packet dirty flags.
- RustChunk implementation mapping: RustChunk optional entity-state parity suite.
- Evidence required before adoption: 1.20.1 expected traces and Forge event/packet tests.
- Completion state: COMPLETE — 53 logical lines through EOF; pinned SHA and 7785-byte size match; oracle inspected, not executed.

### 3090. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaAttributeMapQueriesOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `085644cae53611d11ed2f648dbfab391666c8bcc`
- Size: 9710 bytes
- Coverage: 1–92; complete EOF; 92 canonical logical lines, no gaps.
- Purpose: Tests AttributeMap lookup/cache/prototype behavior, modifier queries, reset and effect-triggered dirty refresh.
- Dependencies / cross-references: AttributeMap/AttributeSupplier, AttributeInstance private maps/cache, LivingEntity effect refresh, MobEffect and Unsafe/reflection.
- Implementation / logic: Reads and mutates supplier prototypes and live instances, tests modifiers, notifications, pending updates, duplicate/missing values and refresh behavior.
- Findings: Relies on private supplier/prototype/cache fields and 26.3 internals; not production-safe and not a chunk-performance benchmark.
- RustChunk relevance: Reference for modded attribute compatibility tests only.
- Decision: adaptable; reference; version-specific; security-sensitive
- 1.20.1 implications: If porting gameplay state, target 1.20.1 AttributeMap and Forge modifier events; avoid copying private-field reflection into app code.
- Follow-up checks: Test prototype isolation, cached values, modifier duplicates, refresh ordering, effect dirty flags and mapping stability.
- RustChunk implementation mapping: RustChunk dev-only attribute parity suite.
- Evidence required before adoption: 1.20.1 Forge traces and tests without private-field dependence where possible.
- Completion state: COMPLETE — 92 logical lines through EOF; pinned SHA and 9710-byte size match; oracle inspected, not executed.

### 3091. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaAttributeOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c10e9f8c4af5aeee19d002040ffebd1077bb1143`
- Size: 6439 bytes
- Coverage: 1–43; complete EOF; 43 canonical logical lines, no gaps.
- Purpose: Tests vanilla AttributeInstance modifier storage, operation ordering, dirty callbacks, packing and application.
- Dependencies / cross-references: AttributeInstance, AttributeModifier operations, AttributeMap, BuiltInRegistries and private field reflection.
- Implementation / logic: Creates attributes/modifiers, applies transient/permanent/replace/remove/copy/pack/unpack/many operations and records values, order and callback state.
- Findings: 26.3 dev oracle, uses private fields and synthetic attributes; no runtime game integration.
- RustChunk relevance: Useful for precise attribute math parity if native gameplay code is introduced.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use target 1.20.1 attribute operation semantics and modded attribute registry data; not a launcher or renderer feature.
- Follow-up checks: Test operation ordering, duplicate IDs, ranged bounds, serialization and Forge attribute registration.
- RustChunk implementation mapping: RustChunk dev-only attribute math parity suite.
- Evidence required before adoption: Target-version differential tests and Forge registry integration.
- Completion state: COMPLETE — 43 logical lines through EOF; pinned SHA and 6439-byte size match; oracle inspected, not executed.

### 3092. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaAttributeUpdatesOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bfdc96dd60067bd9b515c4981a13155050d49aa5`
- Size: 22260 bytes
- Coverage: 1–94; complete EOF; 94 canonical logical lines, no gaps.
- Purpose: Composes vanilla attribute updates with effect lifecycle, health sync, passenger propagation, waypoint tracking and server packets.
- Dependencies / cross-references: LivingEntity, AttributeMap, MobEffect, SynchedEntityData, ServerGamePacketListenerImpl, ServerWaypointManager, Unsafe and private fields.
- Implementation / logic: Models attribute dirty notifications, pending update sets, effect add/remove, health/absorption changes, waypoint manager transitions and outbound effect packets.
- Findings: Large synthetic 26.3 test oracle with Unsafe/private-field manipulation and many mocked server classes; not production code or direct chunk engine.
- RustChunk relevance: Useful for cross-system regression tests if RustChunk ports gameplay internals, but not relevant to launcher rendering/performance.
- Decision: adaptable; reference; version-specific; security-sensitive
- 1.20.1 implications: If native gameplay logic is ever implemented, validate 1.20.1 Forge attribute/effect synchronization; otherwise let the Java game/Forge runtime own it.
- Follow-up checks: Test packet order, passenger propagation, waypoint changes, health data dirty state, effect mutation during callbacks and concurrency.
- RustChunk implementation mapping: RustChunk optional deep gameplay parity harness only.
- Evidence required before adoption: Target 1.20.1 Forge integration traces and deterministic callback ordering tests.
- Completion state: COMPLETE — 94 logical lines through EOF; pinned SHA and 22260-byte size match; oracle inspected, not executed.

### 3093. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaAudioAttributesOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `33cc3877673eee93354547ca92c127361ccab440`
- Size: 6584 bytes
- Coverage: 1–112; complete EOF; 112 canonical logical lines, no gaps.
- Purpose: Exports vanilla audio environment attributes from dimension/biome/timeline/weather layers and music selection rules.
- Dependencies / cross-references: Vanilla server data pack, RegistryLayer, TagLoader, EnvironmentAttributeSystem, biome/dimension registries, ClockManager and Musics.
- Implementation / logic: Loads vanilla registries, evaluates background music choices for creative/underwater states, music volume, ambient loop/mood/additions at requested ticks.
- Findings: Data-pack oracle for 26.3; no real Android audio playback or performance instrumentation.
- RustChunk relevance: Useful for audio/world environment parity, not chunk rendering.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: If supporting world environment semantics natively, use 1.20.1 dimension/biome attributes and validate Forge datapack modifications.
- Follow-up checks: Test timeline boundaries, weather layers, dimension overrides, biome layers and audio category volume.
- RustChunk implementation mapping: RustChunk optional environment/audio parity tests.
- Evidence required before adoption: 1.20.1 data-pack fixtures and Android audio output tests.
- Completion state: COMPLETE — 112 logical lines through EOF; pinned SHA and 6584-byte size match; oracle inspected, not executed.

### 3094. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaAutoJumpOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dc55541977ba9ebdaed0e34b68b8843a9278b4fa`
- Size: 8377 bytes
- Coverage: 1–159; complete EOF; 159 canonical logical lines, no gaps.
- Purpose: Runs actual LocalPlayer.updateAutoJump against synthetic blocks/collision shapes and varied movement state.
- Dependencies / cross-references: LocalPlayer/ClientInput, collision shapes, movement speed attribute, jump boost, resident chunk scene and Unsafe/reflection.
- Implementation / logic: Loads test blocks, configures player box/rotation/ground/input/speed/effects, invokes auto-jump and records result, then restores scene blocks.
- Findings: Uses 26.3 LocalPlayer APIs and reflection; tests auto-jump physics, not chunk load scheduling or GPU rendering.
- RustChunk relevance: Useful for player movement parity; separate from the user's primary chunk/FPS bottleneck.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: If implementing custom movement or porting behavior, compare against Minecraft 1.20.1 and Forge; otherwise preserve vanilla movement in Java client.
- Follow-up checks: Test stairs/slabs/edges, collision bounds, jump boost, input, sprint/sneak and Forge movement-mod hooks.
- RustChunk implementation mapping: RustChunk optional movement parity test harness.
- Evidence required before adoption: Protocol/version-specific movement traces and Android input integration tests.
- Completion state: COMPLETE — 159 logical lines through EOF; pinned SHA and 8377-byte size match; oracle inspected, not executed.

### 3095. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaBasicFeatureOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1b566c2aea37959c4f5ad4ef2ae5586562680c3e`
- Size: 5845 bytes
- Coverage: 1–78; complete EOF; 78 canonical logical lines, no gaps.
- Purpose: Executes vanilla world-generation Features and PlacementModifiers in an observable mutable world with deterministic random.
- Dependencies / cross-references: Feature/PlacedFeature codecs, PlacementContext, WorldGenLevel proxy, BlockState registry and Legacy/Xoroshiro RNG.
- Implementation / logic: Decodes feature/placement JSON, records block reads/writes, injects failures/rejected writes and outputs resulting world state and RNG sequence.
- Findings: Dev-only 26.3 worldgen oracle; only a small fixed block set is exported and it does not test a live chunk-generation pipeline.
- RustChunk relevance: Useful for feature placement/worldgen parity tests, not direct client chunk scheduling.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: If RustChunk implements native worldgen, compare 1.20.1 feature codecs and random sequence; Forge mods may add feature types and placement modifiers.
- Follow-up checks: Test codec failures, random determinism, block-write flags, feature exceptions and modded registry extensions.
- RustChunk implementation mapping: RustChunk optional worldgen feature parity harness.
- Evidence required before adoption: Target 1.20.1 datapack fixtures and Forge registry/worldgen integration.
- Completion state: COMPLETE — 78 logical lines through EOF; pinned SHA and 5845-byte size match; oracle inspected, not executed.

### 3096. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaBiomeChunkOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9c203bd2f27fe256d60b8e8c159b3133be4abfe1`
- Size: 5204 bytes
- Coverage: 1–69; complete EOF; 69 canonical logical lines, no gaps.
- Purpose: Tests vanilla section-atomic biome filling and chunk biome reads, including BelowZeroRetrogen resolver behavior.
- Dependencies / cross-references: ProtoChunk, PalettedContainerFactory, biome IdMapper, BelowZeroRetrogen, BiomeResolver and `fillBiomesFromNoise`.
- Implementation / logic: Fills biome containers for each section, injects provider failures, toggles retrogen and reads biome coordinates back from sections/resolver.
- Findings: Directly relevant to biome data inside chunk generation, but not block-state chunk loading, network chunk decode, mesh building or rendering. Uses 26.3 APIs and synthetic biome holders.
- RustChunk relevance: High-value test pattern for a RustChunk biome/chunk-data implementation, limited to the biome portion of chunk lifecycle.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Port to Minecraft 1.20.1 chunk/biome codecs and retrogen semantics; do not treat successful biome fill as proof that visible chunks are ready/rendered.
- Follow-up checks: Test section atomicity, partial provider failure, retrogen transitions, coordinate quantization, palette serialization and 1.20.1 section layout.
- RustChunk implementation mapping: RustChunk dev-only biome container parity suite.
- Evidence required before adoption: Protocol/version-specific chunk fixtures, real 1.20.1 chunk NBT/network packets and Forge worldgen tests.
- Completion state: COMPLETE — 69 logical lines through EOF; pinned SHA and 5204-byte size match; oracle inspected, not executed.

### 3097. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaBiomeExport.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `03718e6a12654012b8c7139264de6cc9d4d922c2`
- Size: 1870 bytes
- Coverage: 1–32; complete EOF; 32 canonical logical lines, no gaps.
- Purpose: Exports vanilla multi-noise biome preset parameter points to deterministic JSON for worldgen parity.
- Dependencies / cross-references: MultiNoiseBiomeSourceParameterList, Climate.ParameterList, ResourceKey and Gson.
- Implementation / logic: Enumerates known presets, exports six climate parameter intervals, offset and biome identifier in sorted preset order.
- Findings: Data export helper for 26.3 presets; not a runtime biome source or renderer.
- RustChunk relevance: Useful fixture-generation approach for biome generation parity.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Generate the same export from Minecraft 1.20.1 and Forge datapack registries; presets can change between versions/modpacks.
- Follow-up checks: Verify stable ordering, complete preset coverage, modded biome keys and numeric precision.
- RustChunk implementation mapping: RustChunk test fixture exporter only.
- Evidence required before adoption: 1.20.1 exported fixture comparison and modded datapack test.
- Completion state: COMPLETE — 32 logical lines through EOF; pinned SHA and 1870-byte size match; exporter inspected, not executed.

### 3098. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaBiomeGenerationOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2246e960d25ef4d797399273eda48e64b6278f4b`
- Size: 8401 bytes
- Coverage: 1–72; complete EOF; 72 canonical logical lines, no gaps.
- Purpose: Invokes vanilla ChunkGenerator.doCreateBiomes with real noise/density registries, multi-noise/fixed/checkerboard/end sources and retrogen.
- Dependencies / cross-references: NoiseRouter, NormalNoise, DensityFunction registries, RandomState, NoiseBasedChunkGenerator, ProtoChunk, Blender and biome paletted sections.
- Implementation / logic: Reconstructs registry/noise inputs, fills biome sections repeatedly, tests retrogen and queries resulting biome manager positions.
- Findings: Directly relevant to biome generation and chunk section population; still does not cover block-state terrain generation, chunk scheduling, network transfer, meshing or render readiness. Uses 26.3 APIs and private reflective `doCreateBiomes`.
- RustChunk relevance: Strong reference for a target-version biome-generation oracle, not a drop-in Rust chunk engine.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Port to 1.20.1 NoiseRouter/DensityFunction/RandomState/ChunkGenerator APIs and compare with Forge 47.x datapack modifications.
- Follow-up checks: Test deterministic seeds, legacy/Xoroshiro RNG, noise registry loading, biome presets, retrogen, section boundaries and parallel generation.
- RustChunk implementation mapping: RustChunk dev-only biome-generation parity harness.
- Evidence required before adoption: 1.20.1 oracle outputs, deterministic multi-seed comparisons and Android chunk-generation profiling.
- Completion state: COMPLETE — 72 logical lines through EOF; pinned SHA and 8401-byte size match; oracle inspected, not executed.

### 3099. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaBiomePlacementOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c5cbf4f0019cad8de1aeddc3643079a91f3959db`
- Size: 6525 bytes
- Coverage: 1–80; complete EOF; 80 canonical logical lines, no gaps.
- Purpose: Tests biome-filter placement decisions and per-invocation context for PlacedFeature/FeaturePlacer.
- Dependencies / cross-references: BiomeGenerationSettings, PlacementModifier/BiomeFilter, FeaturePlacer, ChunkGenerator, WorldGenLevel proxy and deterministic RNG.
- Implementation / logic: Constructs root/copy/other placed features, scripts biome membership and failure points, records biome queries/feature writes and random progression.
- Findings: Worldgen placement oracle for 26.3; not a chunk renderer, networking or Android performance harness.
- RustChunk relevance: Useful for validating biome-aware feature placement in a future native worldgen implementation.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use exact 1.20.1 biome placement modifier and FeaturePlacer semantics; Forge modded biome registries must be included.
- Follow-up checks: Test nested placement, biome-filter membership identity, random draws, exceptions and placement context propagation.
- RustChunk implementation mapping: RustChunk optional worldgen placement parity tests.
- Evidence required before adoption: Target 1.20.1/Forge datapack fixtures and deterministic placement traces.
- Completion state: COMPLETE — 80 logical lines through EOF; pinned SHA and 6525-byte size match; oracle inspected, not executed.

### 3100. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaBiomeSourceCodecOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8e79ae90533ec0639c321bff61fecd357c826381`
- Size: 4293 bytes
- Coverage: 1–47; complete EOF; 47 canonical logical lines, no gaps.
- Purpose: Tests vanilla BiomeSource.CODEC decode/encode, possible-biome sets and resolver sampling for multi-noise/fixed/nether sources.
- Dependencies / cross-references: BiomeSource.CODEC, registry ops, biome/preset registries, DensitySampler and climate sampler.
- Implementation / logic: Loads biome registry fixtures, decodes source JSON, emits normalized canonical codec output and samples resolved biome at deterministic positions.
- Findings: Uses 26.3 registry/codec types and helper loaders; validates biome source semantics, not full chunk generation or renderer readiness.
- RustChunk relevance: Useful final piece of a biome-generation parity suite; no production code value.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Generate protocol/version-appropriate 1.20.1 biome source fixtures and validate Forge-modified biome registries; keep codecs in the game Java runtime unless native port is proven necessary.
- Follow-up checks: Test malformed codec, unknown registry key, deterministic resolver samples, numeric normalization and modded biome source types.
- RustChunk implementation mapping: RustChunk dev-only biome source codec parity harness.
- Evidence required before adoption: 1.20.1 codec fixtures, Forge registry integration and deterministic sample comparisons.
- Completion state: COMPLETE — 47 logical lines through EOF; pinned SHA and 4293-byte size match; oracle inspected, not executed.

### 3101. `native-mods/wrench/mixins/wrench/click.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: bb369892710f30646c0d4b99c06f68dc6f7d3f4a
- Size: 301 bytes
- Coverage: 1–7; EOF.
- Purpose: Wrench click-hook glue.
- Dependencies / cross-references: Wrench mixin declarations and native callback API.
- Implementation / logic: Forwards click handling to the Wrench native implementation.
- Findings: Starling native-mod hook, not Forge item/event registration.
- RustChunk relevance: Only the general dispatch pattern is relevant.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Implement any similar behavior through Forge 47.x APIs, not this hook.
- Follow-up checks: Test event order, cancellation and server authority if recreated.
- RustChunk implementation mapping: No direct mapping; optional input-event adapter.
- Evidence required before adoption: Forge 1.20.1 event integration tests.
- Completion state: COMPLETE — 7 lines through EOF; pinned SHA/301-byte size match; inspected only.

### 3102. `native-mods/wrench/mixins/wrench/list.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 485ef8f278cd26edfb915d5ed6477f5128d14228
- Size: 406 bytes
- Coverage: 1–10; EOF.
- Purpose: Central Wrench mixin registration list.
- Dependencies / cross-references: Wrench mixin modules and Starling bootstrap.
- Implementation / logic: Registers the feature's mixin hooks.
- Findings: Horizon-specific wiring, not Forge mod metadata.
- RustChunk relevance: Only centralized registration is a generic pattern.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Use the matching Forge/mixin bootstrap for 1.20.1.
- Follow-up checks: Verify mixin config and conflict behavior on Forge.
- RustChunk implementation mapping: No direct mapping.
- Evidence required before adoption: Forge 47.x launch with exact mixin configuration.
- Completion state: COMPLETE — 10 lines through EOF; pinned SHA/406-byte size match; inspected only.

### 3103. `native-mods/wrench/mixins/wrench/native.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 73d1a59752c9e38372c9cc2ad7f1fcd9353589ca
- Size: 8415 bytes
- Coverage: 1–210; EOF.
- Purpose: Server-side Wrench turn/take-apart interaction.
- Dependencies / cross-references: Starling world/player/block APIs, runtime rules and mixin hooks.
- Implementation / logic: Checks interaction context and mutates supported block states through the authoritative world path.
- Findings: Coupled to Starling native-mod APIs; rejects unsupported remote contexts; not a Forge feature implementation.
- RustChunk relevance: Server-authoritative mutation pattern is conceptually useful; concrete behavior is not portable.
- Decision: unnecessary; reference; server-authoritative; version-specific
- 1.20.1 implications: Forge implementation must use server-side events, protection checks and 1.20.1 block APIs.
- Follow-up checks: Test multiplayer, protections, reach, borders, unloaded chunks, drops and cancellation.
- RustChunk implementation mapping: Separate Forge-side feature or explicit server API; not renderer core.
- Evidence required before adoption: Dedicated-server Forge 47.x tests with protections and multiplayer.
- Completion state: COMPLETE — 210 lines through EOF; pinned SHA/8,415-byte size match; no tests run.

### 3104. `native-mods/wrench/mixins/wrench/runtime.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 45ecadef8071b9f4520bb5b9773378f1ad136840
- Size: 11897 bytes
- Coverage: 1–335; EOF.
- Purpose: Wrench block-state rotation and dismantling rules.
- Dependencies / cross-references: Minecraft block-state properties, directions and Wrench logic.
- Implementation / logic: Maps block properties/state variants to supported turns and take-apart operations.
- Findings: Rules are upstream-version and block-family specific, not a general Forge transform engine.
- RustChunk relevance: Could inspire a data-driven rule registry only.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: 1.20.1 and modded blocks require registry-aware state/property handling.
- Follow-up checks: Test directional, multipart, inventory and block-entity cases.
- RustChunk implementation mapping: No direct mapping; optional separate gameplay integration.
- Evidence required before adoption: Forge block-state fixture matrix and server behavior tests.
- Completion state: COMPLETE — 335 lines through EOF; pinned SHA/11,897-byte size match; inspected only.

### 3105. `native-mods/wrench/mixins/wrench/settings.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 54f314f44b2e285d2aff8f54e7d455d112315138
- Size: 274 bytes
- Coverage: 1–6; EOF.
- Purpose: Wrench feature settings bridge.
- Dependencies / cross-references: Wrench mixin and upstream settings/configuration types.
- Implementation / logic: Small feature-scoped configuration declaration.
- Findings: Does not define launcher or renderer preferences.
- RustChunk relevance: No direct value beyond feature-scoped configuration separation.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: RustChunk settings need an independent persisted schema.
- Follow-up checks: Test defaults, migration and client/server authority if recreated.
- RustChunk implementation mapping: No direct mapping.
- Evidence required before adoption: Versioned schema and migration tests.
- Completion state: COMPLETE — 6 lines through EOF; pinned SHA/274-byte size match; inspected only.

### 3106. `native-mods/wrench/mixins/wrench/use.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: ad9dde8244d48878a2609d096b16ecc97333b319
- Size: 470 bytes
- Coverage: 1–9; EOF.
- Purpose: Wrench use-interaction mixin hook.
- Dependencies / cross-references: Wrench mixins and Starling interaction callbacks.
- Implementation / logic: Thin hook forwards use interaction to Wrench behavior.
- Findings: Starling-specific lifecycle; not Forge's item-use event ordering.
- RustChunk relevance: Only separation of hook and behavior is reusable.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Implement through Forge 1.20.1 interaction APIs if needed.
- Follow-up checks: Test hand/offhand, cancellation, event order and remote server behavior.
- RustChunk implementation mapping: No direct mapping; optional Forge feature adapter.
- Evidence required before adoption: Dedicated-server Forge 47.x interaction tests.
- Completion state: COMPLETE — 9 lines through EOF; pinned SHA/470-byte size match; inspected only.

### 3107. `third_party/libmimalloc-sys/build.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 9206d04e17ab49cc98d5fc6fdd799aa10d0ac3be
- Size: 5125 bytes
- Coverage: 1–138; EOF.
- Purpose: Builds and configures bundled mimalloc native allocator.
- Dependencies / cross-references: Cargo features, cc, C sources, target OS/architecture and TLS model.
- Implementation / logic: Selects allocator options and target-specific C/link directives, including Android handling.
- Findings: Static allocator override/TLS affect process-wide allocation and may interact with embedded JVM/native libraries.
- RustChunk relevance: Only consider after profiling; allocator choice is not a direct FPS fix.
- Decision: adaptable; reference; platform-sensitive; allocator-sensitive
- 1.20.1 implications: Validate Android ARM64 NDK, TLS, shared-library loading and JVM coexistence.
- Follow-up checks: Benchmark allocation-heavy paths; test JNI, symbol collisions, OOM and shutdown.
- RustChunk implementation mapping: Optional allocator backend behind a build feature.
- Evidence required before adoption: Reproducible Android build, stress tests and JVM/JNI compatibility evidence.
- Completion state: COMPLETE — 138 lines through EOF; pinned SHA/5,125-byte size match; not built.

### 3108. `third_party/libmimalloc-sys/src/extended.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 90a767fcd476f9c3190c26923308772f8ee3c09d
- Size: 45143 bytes
- Coverage: 1–400, 401–800, 801–1200; EOF; 1201–1600 empty.
- Purpose: Extended mimalloc v3 C APIs.
- Dependencies / cross-references: mimalloc ABI, core FFI, cfg gates and raw pointers.
- Implementation / logic: Declares unsafe heap, arena, option, callback and statistics interfaces.
- Findings: Broad unsafe FFI; ABI/version and caller lifetime/thread-safety obligations are critical.
- RustChunk relevance: Relevant only if mimalloc is deliberately adopted; not a chunk engine.
- Decision: adaptable; reference; unsafe-ffi; allocator-sensitive
- 1.20.1 implications: Hide behind a narrow safe Rust facade and verify exact Android ABI/version.
- Follow-up checks: Audit pointer ownership, callbacks, threading, symbols and failure paths.
- RustChunk implementation mapping: Optional allocator subsystem; no game-engine mapping.
- Evidence required before adoption: Header/binding ABI comparison and Android ARM64 allocator stress tests.
- Completion state: COMPLETE — 1,200 lines in three consecutive 400-line chunks through EOF; pinned SHA/45,143-byte size match; not compiled.

### 3109. `third_party/libmimalloc-sys/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: ebd069b44110e06c877c7f48c6298310c4572983
- Size: 5610 bytes
- Coverage: 1–139; EOF.
- Purpose: Base Rust mimalloc FFI crate surface.
- Dependencies / cross-references: C ABI, allocator symbols, feature flags and extended module.
- Implementation / logic: Exposes no_std-compatible bindings and conditional allocator override symbols.
- Findings: Global override can affect JVM/native process allocations; bindings do not prove safety or speed.
- RustChunk relevance: Optional allocator dependency only after measured evidence.
- Decision: adaptable; reference; allocator-sensitive
- 1.20.1 implications: Do not make it default until shared-process/JVM and linker behavior is validated.
- Follow-up checks: Test symbol resolution, threads, JNI allocations and memory pressure.
- RustChunk implementation mapping: Optional Rust allocator facade behind feature gate.
- Evidence required before adoption: Target-device profiling and JVM/JNI integration tests.
- Completion state: COMPLETE — 139 lines through EOF; pinned SHA/5,610-byte size match; not built.

### 3110. `third_party/naga/build.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 30af3c964be7a7441c2b6f52dcd8e3f9ec74859d
- Size: 549 bytes
- Coverage: 1–12; EOF.
- Purpose: Declares Naga Cargo cfg aliases.
- Dependencies / cross-references: Cargo features and cfg_aliases build dependency.
- Implementation / logic: Maps feature combinations into conditional compilation aliases.
- Findings: Tied to this Naga manifest/revision; copying alone can break cfg consistency.
- RustChunk relevance: Useful build-configuration reference.
- Decision: adaptable; reference; build-configuration; version-specific
- 1.20.1 implications: Pin Naga and preserve its feature graph/Android cfg matrix.
- Follow-up checks: Compare manifest features and target-specific modules.
- RustChunk implementation mapping: RustChunk graphics dependency build configuration.
- Evidence required before adoption: Reproducible Android ARM64 Cargo build.
- Completion state: COMPLETE — 12 lines through EOF; pinned SHA/549-byte size match; not built.

### 3111. `third_party/naga/src/arena/handle.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 5b9c2441e449c95d2c6f0ba577128c40ec1232ae
- Size: 3241 bytes
- Coverage: 1–121; EOF.
- Purpose: Defines typed compact arena handles.
- Dependencies / cross-references: NonMaxU32, typed generics, arena and serde features.
- Implementation / logic: Wraps compact indices in typed handles with indexing/debug/serialization helpers.
- Findings: Handle validity belongs to its source arena; stale handles must not cross arena rebuilds.
- RustChunk relevance: Useful typed-ID pattern for chunks, meshes and GPU resources.
- Decision: adaptable; reference; handle-indexing
- 1.20.1 implications: Use distinct typed IDs and explicit lifecycle/generation boundaries.
- Follow-up checks: Test stale handles, reset, remapping and index exhaustion.
- RustChunk implementation mapping: RustChunk typed-ID/arena primitives.
- Evidence required before adoption: Property tests for invalidation and remapping.
- Completion state: COMPLETE — 121 lines through EOF; pinned SHA/3,241-byte size match; not tested.

### 3112. `third_party/naga/src/arena/handle_set.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 27f6630ec45be0098127378dc3aa5956534c2f57
- Size: 3689 bytes
- Coverage: 1–132; EOF.
- Purpose: Bitset-backed typed handle set.
- Dependencies / cross-references: Handle, bitset storage and arena size metadata.
- Implementation / logic: Supports insertion/removal/contains and iteration over possible handles.
- Findings: `clear_for_arena` clears members/reserves capacity but leaves the stored iteration bound unchanged; may stale `all_possible` after arena resize. Static suspicion, not reproduced.
- RustChunk relevance: Compact sets may help resource deduplication after fixing/testing the invariant.
- Decision: adaptable; reference; correctness-sensitive
- 1.20.1 implications: Define whether bounds follow a fixed arena or current generation.
- Follow-up checks: Test clear with larger/smaller arenas and iteration completeness.
- RustChunk implementation mapping: Potential bitset resource set with explicit arena-generation semantics.
- Evidence required before adoption: Property tests for resize, clear, insert/remove and iteration.
- Completion state: COMPLETE — 132 lines through EOF; pinned SHA/3,689-byte size match; suspected issue not reproduced.

### 3113. `third_party/naga/src/arena/handlevec.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: a25a82c934ccb75067392cf4ea12cdc2877bbb70
- Size: 2837 bytes
- Coverage: 1–106; EOF.
- Purpose: Typed-handle-indexed vector.
- Dependencies / cross-references: Handle and Vec arena iteration APIs.
- Implementation / logic: Wraps vector access/iteration with typed handles.
- Findings: Ordinary Vec behavior remains; handle-to-arena identity is caller invariant.
- RustChunk relevance: Useful dense metadata for fixed chunk slots or mesh handles.
- Decision: adaptable; reference; arena-storage
- 1.20.1 implications: Use only with guaranteed dense/stable index mapping.
- Follow-up checks: Test alignment, bounds, retain/clone and arena rebuild.
- RustChunk implementation mapping: RustChunk dense metadata arrays.
- Evidence required before adoption: Property tests for index alignment and lifecycle.
- Completion state: COMPLETE — 106 lines through EOF; pinned SHA/2,837-byte size match; not tested.

### 3114. `third_party/naga/src/arena/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 4f1f6b6a3e809d56470f6aa7a3c337d3a0bfe57f
- Size: 10666 bytes
- Coverage: 1–342; EOF.
- Purpose: Typed arena and span-aware storage primitives.
- Dependencies / cross-references: Handle/index types, Vec storage and source spans.
- Implementation / logic: Supports append/insert/index/retain/drain and span-aware iteration.
- Findings: `clear` empties data but appears not to clear parallel span metadata; later inserts may see stale spans. Static suspicion, not reproduced.
- RustChunk relevance: Potential arena foundation for transient meshes/IR, subject to metadata correctness.
- Decision: adaptable; reference; correctness-sensitive
- 1.20.1 implications: Reset semantics must keep data and metadata aligned when chunk slots are reused.
- Follow-up checks: Test clear-then-append, span lookup/iteration, retain and drain.
- RustChunk implementation mapping: RustChunk arena with explicit reset/generation invariants.
- Evidence required before adoption: Unit/property tests for data/span alignment after mutations.
- Completion state: COMPLETE — 342 lines through EOF; pinned SHA/10,666-byte size match; suspected issue not reproduced.

### 3115. `third_party/naga/src/arena/range.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 64a14c561f0f32f440470ac82288f49764e52330
- Size: 4208 bytes
- Coverage: 1–139; EOF.
- Purpose: Compact typed handle ranges.
- Dependencies / cross-references: Handle, non-max indices and arena iteration.
- Implementation / logic: Constructs and iterates handle ranges from bounds or arena sizes.
- Findings: Boundary construction depends on valid non-max indices and assertions.
- RustChunk relevance: Useful for contiguous chunk-section/resource batches.
- Decision: adaptable; reference; range-boundaries
- 1.20.1 implications: Use checked bounds and explicit inclusive/exclusive semantics.
- Follow-up checks: Test empty, max-valid, reversed and overflow-adjacent bounds.
- RustChunk implementation mapping: RustChunk range type for contiguous batches.
- Evidence required before adoption: Boundary/property tests.
- Completion state: COMPLETE — 139 lines through EOF; pinned SHA/4,208-byte size match; not tested.

### 3116. `third_party/naga/src/arena/unique_arena.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: d66684c7fc3846c559dfb5afaea50e8fb257844f
- Size: 8299 bytes
- Coverage: 1–265; EOF.
- Purpose: Deduplicating arena with span metadata.
- Dependencies / cross-references: Underlying arena/set, hashing/equality, handles and spans.
- Implementation / logic: Looks up/inserts unique values and supports replacement/removal.
- Findings: `replace` inserts then swap-removes while spans are separately stored; value/span association may stale. Static suspicion, not reproduced.
- RustChunk relevance: Could deduplicate immutable GPU state/descriptors after validation.
- Decision: adaptable; reference; correctness-sensitive
- 1.20.1 implications: Do not use replacement for long-lived IDs until metadata relocation is proven.
- Follow-up checks: Test replacing first/middle/last entries and span lookup after swap removal.
- RustChunk implementation mapping: Optional unique resource/descriptor arena.
- Evidence required before adoption: Property tests comparing values, handles and spans.
- Completion state: COMPLETE — 265 lines through EOF; pinned SHA/8,299-byte size match; suspected issue not reproduced.

### 3117. `third_party/naga/src/back/continue_forward.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: e25d9cf3b2225c4421385de432687c5c0db95b44
- Size: 12801 bytes
- Coverage: 1–313; EOF.
- Purpose: Forwards break/continue when switch is lowered to a synthetic loop.
- Dependencies / cross-references: Naga statement IR and nested loop/switch traversal.
- Implementation / logic: Tracks control-flow contexts and emits flags/exit actions to preserve intended outer-loop behavior.
- Findings: Targeted shader backend lowering workaround, not a general game scheduler.
- RustChunk relevance: Useful only if RustChunk adopts a shader IR needing the same lowering.
- Decision: adaptable; reference; control-flow-lowering
- 1.20.1 implications: Keep shader control flow separate from game/chunk scheduling.
- Follow-up checks: Test nested loops/switches and break/continue combinations against source IR.
- RustChunk implementation mapping: Optional shader IR lowering utility.
- Evidence required before adoption: Differential control-flow tests.
- Completion state: COMPLETE — 313 lines through EOF; pinned SHA/12,801-byte size match; not tested.

### 3118. `third_party/naga/src/back/dot/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 4cb3ef817db817d829ab157188042f14dc40312a
- Size: 34517 bytes
- Coverage: 1–400, 401–800, 801–897; EOF; 1201–1600 empty.
- Purpose: Graphviz DOT exporter for Naga module/IR/control flow.
- Dependencies / cross-references: Naga arenas, module/function validation info and Graphviz.
- Implementation / logic: Builds statement nodes/edges and expression dependency graphs.
- Findings: Calls are recorded, but `write_fun` asserts the call list is empty before the following edge-writing loop; function-call statements appear to panic. Not reproduced.
- RustChunk relevance: Potential dev-only shader/resource graph diagnostics.
- Decision: adaptable; reference; developer-tooling
- 1.20.1 implications: Keep diagnostics off Android render hot path and fix/verify call handling before reliance.
- Follow-up checks: Export a shader with function calls plus nested branch/loop fixtures.
- RustChunk implementation mapping: Optional developer graph exporter.
- Evidence required before adoption: Passing call-graph fixture and deterministic DOT snapshots.
- Completion state: COMPLETE — 897 lines in 400/400/97 chunks through EOF; pinned SHA/34,517-byte size match; not executed.

### 3119. `third_party/naga/src/back/glsl/conv.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: ebbc1df99e5c2b995be55669618d7180b14d31a4
- Size: 8819 bytes
- Coverage: 1–274; EOF.
- Purpose: Maps Naga scalar, builtin, storage qualifier, interpolation and image formats to GLSL.
- Dependencies / cross-references: Naga IR enums, GLSL options and backend errors.
- Implementation / logic: Small conversion helpers with explicit unsupported/error paths.
- Findings: Mesh/ray builtins include unimplemented paths; BGRA8 storage format returns unsupported error.
- RustChunk relevance: Relevant reference for GLES shader conversion, not a full renderer.
- Decision: adaptable; reference; GLES-sensitive; version-specific
- 1.20.1 implications: Use a tested GLSL ES profile and explicit feature rejection/fallback.
- Follow-up checks: Test all scalar widths, builtins, interpolation and storage formats on Mali.
- RustChunk implementation mapping: Naga GLSL conversion only if compatible revision is selected.
- Evidence required before adoption: Target Mali shader compile/link corpus.
- Completion state: COMPLETE — 274 lines through EOF; pinned SHA/8,819-byte size match; not compiled.

### 3120. `third_party/naga/src/back/glsl/features.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 366405c579f41b49e2e27c69bde6920fb67b8f05
- Size: 30681 bytes
- Coverage: 1–400, 401–670; EOF; 801–1200 empty.
- Purpose: Collects required GLSL features, checks version availability and emits extensions.
- Dependencies / cross-references: Naga IR/type analysis, GLSL options/version and extension rules.
- Implementation / logic: Scans IO, types, globals, images, subgroup ops and atomics into feature flags.
- Findings: Profile/extension availability is driver-specific; passing static checks does not guarantee a driver compile.
- RustChunk relevance: Strong reference for GLES capability negotiation.
- Decision: adaptable; reference; GLES-sensitive; version-specific
- 1.20.1 implications: Base capabilities on actual Android GLES extensions, not version labels alone.
- Follow-up checks: Test storage images, atomics, subgroup, multiview, multisampling, queries and early depth.
- RustChunk implementation mapping: RustChunk shader capability negotiation/precompile validation.
- Evidence required before adoption: Device extension enumeration and per-feature shader compile tests.
- Completion state: COMPLETE — 670 lines in 400/270 chunks through EOF; pinned SHA/30,681-byte size match; not compiled.

### 3121. `third_party/naga/src/back/glsl/keywords.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: aeaec5b03c060e81144120e573c6453a38892261
- Size: 13745 bytes
- Coverage: 1–400, 401–502; EOF.
- Purpose: GLSL 4.6/ES 3.2 reserved keywords and builtin names.
- Dependencies / cross-references: Naga KeywordSet/RacyLock and cited GLSL specs.
- Implementation / logic: Stores reserved identifiers and a lazily cached keyword set for name generation.
- Findings: Keyword corpus is tied to specification revisions; profile/compiler quirks still need validation.
- RustChunk relevance: Useful for preventing invalid generated shader identifiers.
- Decision: adaptable; reference; GLSL-version-specific
- 1.20.1 implications: Align reserved names with emitted GLSL ES profile and RustChunk internals.
- Follow-up checks: Test builtins, reserved words, case sensitivity and generated identifiers on target compiler.
- RustChunk implementation mapping: RustChunk shader identifier sanitizer or matching Naga revision.
- Evidence required before adoption: Automated keyword corpus and target-driver compile tests.
- Completion state: COMPLETE — 502 lines in 400/102 chunks through EOF; pinned SHA/13,745-byte size match; not compiled.

### 3122. `third_party/naga/src/back/glsl/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: d0bf71a336efd4f9e5526aea1b602300f513004e
- Size: 18520 bytes
- Coverage: 1–400, 401–505; EOF; 801–1200 empty.
- Purpose: Defines public GLSL backend API, options, reflection, errors and capabilities.
- Dependencies / cross-references: Naga backend traits, GLSL conv/features/keywords/writer and validation.
- Implementation / logic: Exposes writer/config, binding maps, pipeline options, reflection and supported profile metadata.
- Findings: Documents desktop 330–450 and ES 300/310; advanced capabilities can still be rejected or unimplemented.
- RustChunk relevance: Strong shader-backend reference; exact revision/profile must match target.
- Decision: adaptable; reference; GLES-sensitive; version-specific
- 1.20.1 implications: Select and test a concrete ES profile against Mali GLES 3.2 and required renderer shaders.
- Follow-up checks: Compile representative shaders and verify resource reflection/bindings.
- RustChunk implementation mapping: Optional Naga GLSL backend in graphics layer, separate from Forge logic.
- Evidence required before adoption: Mali-G52 GLES 3.2 compile/link and render parity.
- Completion state: COMPLETE — 505 lines in 400/105 chunks through EOF; pinned SHA/18,520-byte size match; not compiled.

### 3123. `third_party/naga/src/back/glsl/writer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: ff2174eeec88991809c3a625730fe8fa5f188c01
- Size: 203165 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4634; EOF; 4801–5200 empty.
- Purpose: Naga GLSL code generator for types, globals, entry points, expressions, statements, images, barriers and reflection.
- Dependencies / cross-references: Naga IR/validation, GLSL feature/conversion modules, continue forwarding, bounds policies and reflection.
- Implementation / logic: Writes desktop GLSL/ES; handles resources, varyings, workgroup init, math, image ops, atomics, subgroup and control flow.
- Findings: Ray-query/ray-pipeline and cooperative operations have unsupported paths; external images unimplemented; F16/64-bit integer literals error; `clamp_to_edge` remains TODO.
- RustChunk relevance: Highly relevant GLSL backend reference, but not chunk decode/scheduling/meshing or Forge execution.
- Decision: adaptable; reference; GLES-sensitive; version-specific
- 1.20.1 implications: Adopt only with pinned Naga/wgpu compatibility and a shader corpus covering Minecraft 1.20.1 renderer needs.
- Follow-up checks: Compile/link on Mali; exercise image bounds, barriers, atomics, subgroup and unsupported-feature diagnostics.
- RustChunk implementation mapping: Potential GLSL backend in graphics abstraction; keep Forge/Java compatibility separate.
- Evidence required before adoption: Target-device shader compile/link/render parity and dependency audit.
- Completion state: COMPLETE — 4,634 lines in eleven 400-line chunks plus 234 lines through EOF; pinned SHA/203,165-byte size match; not compiled.

### 3124. `third_party/naga/src/back/hlsl/conv.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 6b3abc625882d14319358bcd58c46a60026034fa
- Size: 9866 bytes
- Coverage: 1–257; EOF.
- Purpose: Maps Naga types, builtins, interpolation, atomics and formats to HLSL.
- Dependencies / cross-references: Naga IR, HLSL errors, type layout and name mapping.
- Implementation / logic: Converts scalar widths, matrices, storage formats and builtins to HLSL declarations.
- Findings: The mapping appears to swap `Rg32Uint`/`Rg32Sint` signedness (`int4`/`uint4`); likely issue, not reproduced.
- RustChunk relevance: Not needed for Android GLES; reference only for future D3D.
- Decision: unnecessary; reference; HLSL-only; D3D-specific
- 1.20.1 implications: Keep out of Android GLES baseline.
- Follow-up checks: If D3D is ever targeted, verify all storage formats with DXC/UAV tests.
- RustChunk implementation mapping: No current mapping; optional Windows/D3D backend.
- Evidence required before adoption: DXC tests for Rg32Uint/Rg32Sint and all format mappings.
- Completion state: COMPLETE — 257 lines through EOF; pinned SHA/9,866-byte size match; not compiled.

### 3125. `third_party/naga/src/back/hlsl/help.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 10631331a52e18cbe4a9d53f0812c96821db3881
- Size: 98541 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2356; EOF; 2401–2800 empty.
- Purpose: Generates HLSL helper wrappers for image query/load/sample, external YUV textures, constructors, matrix access and math.
- Dependencies / cross-references: Naga HLSL Writer, WrappedType registry, IR types, sampler heaps and texture metadata.
- Implementation / logic: Emits wrappers for GetDimensions, YUV conversion, clamping, matrix layout and numeric edge cases.
- Findings: D3D/HLSL-specific assumptions; not an Android GLES shader implementation.
- RustChunk relevance: No direct mobile GLES value; future D3D reference only.
- Decision: unnecessary; reference; HLSL-only; D3D-specific
- 1.20.1 implications: Keep out of Android GLES renderer; select backend explicitly by platform.
- Follow-up checks: If D3D is added, test DXC wrappers, texture dimensions, YUV, samplers and matrix access.
- RustChunk implementation mapping: No current mapping; optional future HLSL backend.
- Evidence required before adoption: DXC compilation, D3D resource binding and cross-backend shader parity.
- Completion state: COMPLETE — 2,356 lines in five 400-line chunks plus 356 lines through EOF; pinned SHA/98,541-byte size match; not compiled.

## Checkpoint 3101–3125 — COMPLETE

- 25 new source files inspected one at a time from the pinned snapshot after excluding all 3,100 prior Index paths.
- All source SHAs and byte sizes match the complete pinned tree (5,749 entries; untruncated). Canonical logical line counts and complete EOF coverage are recorded.
- Long files were read in consecutive, gap-free ranges no larger than 400 lines: 3108 (1,200), 3118 (897), 3120 (670), 3121 (502), 3122 (505), 3123 (4,634) and 3125 (2,356).
- All entries have exactly 15 required fields. Static concerns in HandleSet/Arena/UniqueArena and the DOT call assertion were not reproduced. HLSL entries 3124–3125 are not part of the Android GLES baseline.
- No tests/builds/benchmarks/APK/device/Forge/Minecraft/server/Aternos validation was run. No application source was changed.

### 3126. `third_party/naga/src/back/hlsl/keywords.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: a5ae4085af5059d017bab63e179fe1a3c0b550a6
- Size: 20941 bytes
- Coverage: 1–400, 401–800, 801–940; EOF; 1201–1600 confirmed empty.
- Purpose: Reserved HLSL keywords and builtin identifiers for name sanitization.
- Dependencies / cross-references: HLSL language/Shader Model reserved names and Naga identifier generation.
- Implementation / logic: Builds the keyword set used to avoid emitting reserved or colliding HLSL identifiers.
- Findings: Version/profile-specific reserved names; target compiler and Shader Model still need validation.
- RustChunk relevance: Reference only if a separate D3D/HLSL backend is intentionally added.
- Decision: unnecessary; reference; HLSL-only; D3D-specific
- 1.20.1 implications: Android GLES does not consume HLSL source.
- Follow-up checks: Compile generated identifiers with DXC for each targeted Shader Model if D3D is ever supported.
- RustChunk implementation mapping: No current mapping; optional future D3D shader-name sanitizer.
- Evidence required before adoption: DXC keyword corpus and generated shader compile tests.
- Completion state: COMPLETE — 940 logical lines through EOF; pinned SHA and 20,941-byte size match; not compiled.

### 3127. `third_party/naga/src/back/hlsl/mesh_shader.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: f0f72370e069599b9915ae1380f360d87bcd1e23
- Size: 17384 bytes
- Coverage: 1–400, 401–468; consecutive, gap-free; EOF; 801–1200 confirmed empty.
- Purpose: Generates HLSL mesh/task shader output and wrapper logic.
- Dependencies / cross-references: Naga mesh entry-point IR, HLSL writer, output bindings and Shader Model capabilities.
- Implementation / logic: Maps per-vertex/per-primitive outputs and task dispatch into HLSL mesh shader constructs and helper wrappers.
- Findings: Requires mesh-shader-capable D3D Shader Model/runtime; not available in the Android GLES baseline.
- RustChunk relevance: Only a platform-specific reference for a future Windows renderer.
- Decision: unnecessary; reference; HLSL-only; D3D-specific
- 1.20.1 implications: Forge 1.20.1 compatibility does not require HLSL mesh shaders; Android renderer must use GLES/Vulkan path.
- Follow-up checks: If D3D is added, test output limits, primitive indices, task dispatch and per-primitive semantics on DXC/runtime.
- RustChunk implementation mapping: No current mapping; future Windows/D3D renderer only.
- Evidence required before adoption: DXC compile and D3D mesh/task shader execution tests.
- Completion state: COMPLETE — 468 logical lines through EOF in 400/68 ranges; pinned SHA and 17,384-byte size match; not compiled.

### 3128. `third_party/naga/src/back/hlsl/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 3222c24a1922b02380da7cbe2d01f83bc553fdb3
- Size: 31234 bytes
- Coverage: 1–400, 401–800, 801–833; EOF; 1201–1600 confirmed empty.
- Purpose: Public HLSL backend API, options, errors, capabilities and pipeline configuration.
- Dependencies / cross-references: Naga IR, HLSL writer/helpers/keywords/mesh/ray/storage modules and Shader Model selection.
- Implementation / logic: Defines HLSL output options, binding/resource behavior, version capabilities and writer entry points.
- Findings: Capability declarations are not equivalent to runtime driver support; HLSL profile and DXC version must match.
- RustChunk relevance: Useful only for a future D3D backend; not a mobile renderer component.
- Decision: unnecessary; reference; HLSL-only; D3D-specific
- 1.20.1 implications: No Android GLES integration; keep backend selection platform-specific.
- Follow-up checks: If Windows support is added, validate each advertised capability against DXC and D3D device feature queries.
- RustChunk implementation mapping: No current mapping; future D3D graphics backend.
- Evidence required before adoption: DXC compile/link and D3D capability matrix.
- Completion state: COMPLETE — 833 logical lines through EOF in 400/400/33 ranges; pinned SHA and 31,234-byte size match; not compiled.

### 3129. `third_party/naga/src/back/hlsl/ray.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 0d69756362d0a09af73bbd8d373925d1ae11b883
- Size: 22831 bytes
- Coverage: 1–400, 401–564; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Implements HLSL ray-query and ray-tracing pipeline helper functions.
- Dependencies / cross-references: Naga ray query/pipeline IR, HLSL writer, acceleration structures and payload globals.
- Implementation / logic: Builds wrapper functions for ray initialization, proceed/confirm/terminate, intersection queries and trace operations.
- Findings: Ray tracing requires D3D ray-tracing-capable hardware/runtime and exact Shader Model support; wrappers are not portable to GLES.
- RustChunk relevance: No current mobile GLES use; conceptual reference for future ray features only.
- Decision: unnecessary; reference; HLSL-only; D3D-specific
- 1.20.1 implications: Do not couple Forge gameplay or Android renderer to this D3D backend.
- Follow-up checks: If D3D is targeted, validate ray flags, payload layout, query lifecycle and DXR runtime semantics.
- RustChunk implementation mapping: No current mapping; optional future D3D ray backend.
- Evidence required before adoption: DXC and DXR execution tests with invalid and valid ray descriptors.
- Completion state: COMPLETE — 564 logical lines through EOF in 400/164 ranges; pinned SHA and 22,831-byte size match; not compiled.

### 3130. `third_party/naga/src/back/hlsl/storage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 5d66af00087313a9892dde1f801b1a9c99a377e0
- Size: 27904 bytes
- Coverage: 1–400, 401–647; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Generates HLSL storage-buffer and storage-image declarations/access helpers.
- Dependencies / cross-references: Naga global/resource binding model, HLSL writer, image formats and binding arrays.
- Implementation / logic: Maps storage resources to HLSL register/space and typed/raw buffer/image access patterns.
- Findings: Resource binding arrays and typed UAV formats depend on D3D feature levels and DXC behavior; no Android GLES relevance.
- RustChunk relevance: Reference only for a separate D3D backend.
- Decision: unnecessary; reference; HLSL-only; D3D-specific
- 1.20.1 implications: Android uses GLES/Vulkan resource binding; do not reuse HLSL register mapping.
- Follow-up checks: If D3D is added, test storage formats, binding arrays, access modes, offsets and resource register collisions.
- RustChunk implementation mapping: No current mapping; future D3D resource backend.
- Evidence required before adoption: DXC reflection plus D3D resource-binding and UAV tests.
- Completion state: COMPLETE — 647 logical lines through EOF in 400/247 ranges; pinned SHA and 27,904-byte size match; not compiled.

### 3131. `third_party/naga/src/back/hlsl/writer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: af3736ff11f386532d4b930c67482cb12cfd782c
- Size: 216427 bytes
- Coverage: 1–400 through 4401–4800 in consecutive 400-line chunks, then 4801–5106; EOF; 5201–5600 confirmed empty.
- Purpose: Full HLSL shader code generator for types, resources, entry points, expressions, statements, images, atomics, mesh and ray operations.
- Dependencies / cross-references: Naga IR/validation, HLSL options, conv/features/keywords, helper wrappers, mesh/ray/storage modules.
- Implementation / logic: Emits HLSL translation units, resource declarations, helper functions and stage-specific control flow/operations.
- Findings: Backend-only unsupported paths and profile limits remain; successful source emission is not DXC/device validation. It is D3D-specific.
- RustChunk relevance: Not part of Android GLES baseline; may be reused only for future Windows renderer.
- Decision: unnecessary; reference; HLSL-only; D3D-specific
- 1.20.1 implications: Keep out of Android APK graphics path; Minecraft Forge Java compatibility is independent of this backend.
- Follow-up checks: If D3D is added, compile a representative shader corpus with pinned DXC and test every enabled Shader Model feature.
- RustChunk implementation mapping: No current mapping; optional Windows/D3D shader backend.
- Evidence required before adoption: DXC compile/link, reflection and D3D render parity across required shaders.
- Completion state: COMPLETE — 5,106 logical lines through EOF in twelve 400-line chunks plus 306 lines; pinned SHA and 216,427-byte size match; not compiled.

### 3132. `third_party/naga/src/back/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: fa67879226f37b584792e06cb0db59dc39e8ad43
- Size: 13959 bytes
- Coverage: 1–400; EOF; 401–800 confirmed empty.
- Purpose: Defines shared backend abstractions and task-dispatch limit metadata.
- Dependencies / cross-references: Naga backend modules, ShaderStage, TaskDispatchLimits and backend capability structures.
- Implementation / logic: Provides common backend-facing types used by GLSL/HLSL/MSL/SPIR-V implementations.
- Findings: Backend-neutral abstractions still carry version-specific semantics; task dispatch limits must be supplied by the selected GPU API/device.
- RustChunk relevance: Useful as a reference for a clean graphics-backend interface and explicit capability/limit separation.
- Decision: adaptable; reference; graphics-architecture
- 1.20.1 implications: RustChunk should derive limits from Android GLES/Vulkan device queries rather than hard-code desktop defaults.
- Follow-up checks: Map backend limits to queried device limits and validate dispatch dimensions/overflow.
- RustChunk implementation mapping: RustChunk graphics capability/limit interface shared across renderer backends.
- Evidence required before adoption: Device capability capture and per-backend limit tests.
- Completion state: COMPLETE — 400 logical lines through EOF; pinned SHA and 13,959-byte size match; inspected only.

### 3133. `third_party/naga/src/back/msl/keywords.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 0f35b424bf88c4c73aa73912681eb04fa475af56
- Size: 8477 bytes
- Coverage: 1–395; EOF.
- Purpose: Reserved Metal Shading Language identifiers and builtin names.
- Dependencies / cross-references: MSL specification names and Naga identifier generation.
- Implementation / logic: Defines reserved identifier list and cached keyword set for MSL code generation.
- Findings: The set is MSL-version-specific; compiler acceptance must be checked against the selected Metal toolchain.
- RustChunk relevance: Reference only for an Apple/Metal renderer.
- Decision: unnecessary; reference; MSL-only; Apple-specific
- 1.20.1 implications: Android Mali does not use Metal/Objective-C.
- Follow-up checks: If Apple support is added, test reserved names against the target Metal compiler.
- RustChunk implementation mapping: No current mapping; future Apple/Metal shader sanitizer.
- Evidence required before adoption: Metal compiler keyword corpus and shader compile tests.
- Completion state: COMPLETE — 395 logical lines through EOF; pinned SHA and 8,477-byte size match; not compiled.

### 3134. `third_party/naga/src/back/msl/mesh_shader.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: a172bde802d058dcac90632505cec7aeafb82ea7
- Size: 18432 bytes
- Coverage: 1–400, 401–438; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Builds Metal mesh/task shader output wrappers and output copy logic.
- Dependencies / cross-references: Naga mesh/task IR, MSL writer, output structures and Metal stage limits.
- Implementation / logic: Maps vertex/primitive output data into Metal mesh shader output structures and dispatch wrappers.
- Findings: Requires Metal mesh shader support and Apple-specific limits; not a mobile Android path.
- RustChunk relevance: No current Android use; future Apple backend reference only.
- Decision: unnecessary; reference; MSL-only; Apple-specific
- 1.20.1 implications: Do not use for Android GLES/Vulkan or Forge mod compatibility.
- Follow-up checks: If Apple target is added, validate max vertices/primitives, task dispatch and per-primitive output semantics.
- RustChunk implementation mapping: No current mapping; optional Apple renderer.
- Evidence required before adoption: Metal compile and runtime mesh/task output tests.
- Completion state: COMPLETE — 438 logical lines through EOF in 400/38 ranges; pinned SHA and 18,432-byte size match; not compiled.

### 3135. `third_party/naga/src/back/msl/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 45a3a8ac4ac3aba2d4751e2f2d2190f39b981a77
- Size: 34859 bytes
- Coverage: 1–400, 401–800, 801–883; consecutive; EOF; 1201–1600 confirmed empty.
- Purpose: Public MSL backend API, options, capabilities, resource mapping and pipeline configuration.
- Dependencies / cross-references: Naga IR, MSL writer, sampler/ray/mesh helpers, Metal language version and binding maps.
- Implementation / logic: Defines MSL writer options, binding maps, stage support and generated source entry points.
- Findings: Metal capability declarations and language versions must match actual Apple GPU/runtime; not portable to Android.
- RustChunk relevance: Reference only for a future Apple/Metal target.
- Decision: unnecessary; reference; MSL-only; Apple-specific
- 1.20.1 implications: No Android renderer integration; keep MSL separate from GLES/Vulkan backend selection.
- Follow-up checks: If Apple support is added, verify Metal language version, argument buffers, resource limits and stage features.
- RustChunk implementation mapping: No current mapping; future Apple graphics backend.
- Evidence required before adoption: Metal compile/link and runtime capability tests.
- Completion state: COMPLETE — 883 logical lines through EOF in 400/400/83 ranges; pinned SHA and 34,859-byte size match; not compiled.

### 3136. `third_party/naga/src/back/msl/ray.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 05ae6fc8c152b3a1dbd2daf144ec7732bae00f41
- Size: 24021 bytes
- Coverage: 1–400, 401–533; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Generates Metal ray-query and ray-tracing pipeline wrappers.
- Dependencies / cross-references: Naga ray IR, MSL writer, acceleration structures and Metal intersection query API.
- Implementation / logic: Maps ray query initialization/proceed/confirm and ray tracing operations to Metal-specific constructs.
- Findings: Ray support is Metal-runtime/device specific; behavior and available query APIs vary by Metal version.
- RustChunk relevance: No Android use; Apple-only reference.
- Decision: unnecessary; reference; MSL-only; Apple-specific
- 1.20.1 implications: Keep out of Android GLES/Vulkan baseline.
- Follow-up checks: If Apple is targeted, validate ray query lifecycle, flags, payload and acceleration-structure APIs.
- RustChunk implementation mapping: No current mapping; future Apple ray backend.
- Evidence required before adoption: Metal compile/runtime ray tracing tests.
- Completion state: COMPLETE — 533 logical lines through EOF in 400/133 ranges; pinned SHA and 24,021-byte size match; not compiled.

### 3137. `third_party/naga/src/back/msl/sampler.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 5dc5afdb0069c54874dbf0da179e3d3b4ca40b7c
- Size: 3986 bytes
- Coverage: 1–148; EOF.
- Purpose: Maps Naga sampler descriptions and inline sampler properties to Metal sampler syntax/objects.
- Dependencies / cross-references: Naga sampler descriptors, MSL writer and Metal filtering/addressing enums.
- Implementation / logic: Converts address modes, filters, comparison and sampler property combinations into MSL representation.
- Findings: Metal inline sampler restrictions and unsupported property combinations must be handled explicitly; not a GLES sampler implementation.
- RustChunk relevance: Conceptual sampler mapping only; Android must use GLES/Vulkan sampler APIs.
- Decision: unnecessary; reference; MSL-only; Apple-specific
- 1.20.1 implications: No direct 1.20.1 Forge or Android renderer reuse.
- Follow-up checks: If Apple support is added, test every address/filter/compare combination against Metal validation.
- RustChunk implementation mapping: No current mapping; optional Apple sampler adapter.
- Evidence required before adoption: Metal compile and sampler behavior tests.
- Completion state: COMPLETE — 148 logical lines through EOF; pinned SHA and 3,986-byte size match; not compiled.

### 3138. `third_party/naga/src/back/msl/writer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 119b41edb81acbdc05ea1de4b820dca56eafc776
- Size: 368991 bytes
- Coverage: 1–400 through 8001–8400 in consecutive 400-line chunks, then 8401–8588; EOF; 8801–9200 confirmed empty.
- Purpose: Full Metal Shading Language code generator for types, resources, entry points, expressions, images, atomics, mesh and ray operations.
- Dependencies / cross-references: Naga IR/validation, MSL options, helper/sampler/ray/mesh modules and Metal resource model.
- Implementation / logic: Emits MSL source, resource/argument-buffer declarations, shader-stage wrappers and helper/polyfill code.
- Findings: In external YUV wrapper, `plane2_coords` upper clamp appears to subtract `plane1_half_texel` rather than `plane2_half_texel`; suspected copy/paste bug, not reproduced. Some sampler properties are explicitly TODO/unsupported.
- RustChunk relevance: No Android use; large Apple-only backend. The suspected YUV issue matters only if MSL is adopted.
- Decision: unnecessary; reference; MSL-only; Apple-specific
- 1.20.1 implications: Keep out of Android GLES/Vulkan renderer.
- Follow-up checks: If Apple support is added, test external YUV crop bounds with differing plane dimensions and every sampler property.
- RustChunk implementation mapping: No current mapping; future Apple/Metal backend only.
- Evidence required before adoption: Metal compiler/runtime YUV fixtures with non-identical plane sizes and sampler matrix.
- Completion state: COMPLETE — 8,588 logical lines through EOF in twenty-one 400-line chunks plus 188 lines; pinned SHA and 368,991-byte size match; not compiled.

### 3139. `third_party/naga/src/back/pipeline_constants.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: b9e429cf32a03bab1cbc3e6175f7dc681d0a2f0a
- Size: 42155 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1217; consecutive; EOF; 1601–2000 confirmed empty.
- Purpose: Resolves pipeline overrides/specialization constants and compacts the module for a selected entry point.
- Dependencies / cross-references: Naga Module overrides, constant expressions, entry-point selection and type/value conversion.
- Implementation / logic: Evaluates supplied pipeline constants, substitutes defaults where appropriate, removes resolved overrides and prepares per-pipeline IR.
- Findings: Requires correct override IDs/types/defaults and entry-point reachability; malformed or missing values return validation errors.
- RustChunk relevance: Highly relevant to compiling shader variants/pipelines without rebuilding unrelated game data.
- Decision: adaptable; reference; pipeline-specialization
- 1.20.1 implications: Use pipeline constants for renderer variants only; Minecraft 1.20.1 mod shader packs may require exact compatibility with upstream Naga version.
- Follow-up checks: Test named/ID overrides, defaults, type mismatches, missing values, unused overrides and entry-point filtering.
- RustChunk implementation mapping: RustChunk shader pipeline cache key and specialization stage.
- Evidence required before adoption: Deterministic tests proving distinct override sets produce correct distinct shader modules.
- Completion state: COMPLETE — 1,217 logical lines through EOF in three 400-line chunks plus 17 lines; pinned SHA and 42,155-byte size match; not executed.

### 3140. `third_party/naga/src/back/spv/block.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: bb977e81fcaadd8bdbb18f4dcd7f318123f4f444
- Size: 188584 bytes
- Coverage: 1–400 through 4001–4400 in eleven consecutive 400-line chunks; EOF; 4401–4800 confirmed empty.
- Purpose: Core SPIR-V block/function lowering for expressions, statements, control flow, memory and shader operations.
- Dependencies / cross-references: Naga IR/validation, SPIR-V Writer, Instruction builder, image/index helpers, ray/mesh/subgroup modules and bounds policies.
- Implementation / logic: Lowers Naga function blocks into SPIR-V instructions, manages block/merge labels, locals, loads/stores, access chains, calls and structured control flow.
- Findings: Complex lowering depends on prior validation and exact Vulkan/SPIR-V capabilities; unsupported IR paths return errors or panic paths. No runtime Vulkan evidence from static review.
- RustChunk relevance: Strong candidate reference if RustChunk chooses Vulkan on Android; not a Minecraft chunk scheduler or Forge runtime.
- Decision: adaptable; reference; Vulkan-specific; correctness-critical
- 1.20.1 implications: Mali-G52 Vulkan support and driver extensions must be queried; choose Vulkan only after actual device tests.
- Follow-up checks: Compile/validate representative Minecraft shaders with SPIR-V Tools and run on target Mali driver; test bounds policies and control flow.
- RustChunk implementation mapping: Potential Naga SPIR-V backend within a Vulkan renderer abstraction.
- Evidence required before adoption: SPIR-V validation, Vulkan pipeline creation and render parity on target device.
- Completion state: COMPLETE — 4,400 logical lines through EOF in eleven consecutive 400-line chunks; pinned SHA and 188,584-byte size match; not compiled.

### 3141. `third_party/naga/src/back/spv/f16_polyfill.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: bd788592fa0ce2d08ae32bf006bd79e27000a4ad
- Size: 3181 bytes
- Coverage: 1–104; EOF.
- Purpose: Polyfills 16-bit float shader input/output when native SPIR-V storage capability is unavailable.
- Dependencies / cross-references: SPIR-V Writer, f16 types, input/output interface variables and conversion instructions.
- Implementation / logic: Creates f32-compatible interface values and conversion logic for f16 data paths.
- Findings: Polyfill changes precision/representation and must match Vulkan capability and shader interface rules.
- RustChunk relevance: Useful only for a Vulkan backend on devices lacking required 16-bit storage features.
- Decision: adaptable; reference; Vulkan-specific; precision-sensitive
- 1.20.1 implications: Query `StorageInputOutput16` and float16 features on the Android driver; don't assume Mali support from GPU name alone.
- Follow-up checks: Test f16 input/output round trips, NaN/subnormal behavior and interface layout with/without native capability.
- RustChunk implementation mapping: Optional f16 I/O fallback in Vulkan shader backend.
- Evidence required before adoption: SPIR-V validation and GPU readback precision tests.
- Completion state: COMPLETE — 104 logical lines through EOF; pinned SHA and 3,181-byte size match; not compiled.

### 3142. `third_party/naga/src/back/spv/helpers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: d3de254918a11586357925b7188785d7279812bd
- Size: 7467 bytes
- Coverage: 1–213; EOF.
- Purpose: Provides SPIR-V binding/storage/image helpers and string/word packing utilities.
- Dependencies / cross-references: SPIR-V enums, Naga resource bindings, instruction layout and type metadata.
- Implementation / logic: Maps address spaces/storage classes, formats and decorations; packs strings into SPIR-V words and handles wrapped globals.
- Findings: String chunk helper assumes a positive chunk limit; internal callers pass a fixed positive limit. Resource mapping must align with Vulkan descriptor rules.
- RustChunk relevance: Useful low-level reference if Naga SPIR-V backend is adopted.
- Decision: adaptable; reference; Vulkan-specific
- 1.20.1 implications: Use Vulkan descriptor layout and Android driver capability data; do not reuse raw mapping for GLES.
- Follow-up checks: Test string packing at empty, aligned, unaligned and >64KiB boundaries; verify all storage class/format mappings.
- RustChunk implementation mapping: Optional SPIR-V helper layer only with pinned Naga revision.
- Evidence required before adoption: SPIR-V Tools validation and mapping unit tests.
- Completion state: COMPLETE — 213 logical lines through EOF; pinned SHA and 7,467-byte size match; inspected only.

### 3143. `third_party/naga/src/back/spv/image.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: c185806cd33eab341f8ee7bed58ad346047e7380
- Size: 51731 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1383; consecutive; EOF; 1601–2000 confirmed empty.
- Purpose: Lowers SPIR-V image load/sample/query/store and storage-image atomic operations.
- Dependencies / cross-references: Naga image IR, SPIR-V Instruction builder, bounds-check policies, image formats and capability tracking.
- Implementation / logic: Builds sampled-image ops, LOD/gather, clamp-to-edge, dimension queries, bounds-restricted loads, stores and supported atomics.
- Findings: Storage-image atomic `Exchange` explicitly errors; BGRA8 writes require `StorageImageWriteWithoutFormat`; clamp-to-edge is restricted to level zero and validation assumptions.
- RustChunk relevance: Relevant to Vulkan texture/storage-image implementation, with explicit capability/format limits.
- Decision: adaptable; reference; Vulkan-specific; image-capability-sensitive
- 1.20.1 implications: Use only after target Vulkan format/atomic/extension queries; GLES renderer requires a separate backend.
- Follow-up checks: Test every image dimension/class, LOD, array/sample index, bounds policy, BGRA8 and atomic operation on target driver.
- RustChunk implementation mapping: Potential Vulkan image backend; not chunk streaming or Forge mod loading.
- Evidence required before adoption: SPIR-V validation plus Vulkan image read/write/sample/atomic GPU tests.
- Completion state: COMPLETE — 1,383 logical lines through EOF in three 400-line chunks plus 183 lines; pinned SHA and 51,731-byte size match; not compiled.

### 3144. `third_party/naga/src/back/spv/index.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: b61b645d44288a06e065ad7f065d69e3bac4f514
- Size: 26059 bytes
- Coverage: 1–400, 401–800; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Implements SPIR-V array/vector/matrix/runtime-array index bounds checks.
- Dependencies / cross-references: Naga GuardedIndex, runtime array length, resource binding maps, SPIR-V selection blocks and bounds policies.
- Implementation / logic: Resolves known indices statically; emits UMin restrictions or conditional loads for runtime checks; computes runtime array lengths.
- Findings: Restrict policy intentionally maps invalid indices to an arbitrary in-bounds element (not conventional signed clamp); zero-length sequences rely on validation invariants.
- RustChunk relevance: Important safety reference for GPU buffer/index access; policy semantics must be explicit.
- Decision: adaptable; reference; Vulkan-specific; bounds-safety-critical
- 1.20.1 implications: Renderer resource indexing must use the selected backend's exact bounds policy and validated lengths.
- Follow-up checks: Test negative/out-of-range, runtime arrays, binding arrays, zero-length validation and ReadZeroSkipWrite branches.
- RustChunk implementation mapping: RustChunk GPU buffer/array access validation in Vulkan backend.
- Evidence required before adoption: Property tests plus SPIR-V Tools and runtime GPU validation.
- Completion state: COMPLETE — 800 logical lines through EOF in two consecutive 400-line chunks; pinned SHA and 26,059-byte size match; not tested.

### 3145. `third_party/naga/src/back/spv/instructions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 0a22bc3dbfea0a18c5c5584a6342f41327fdcf15
- Size: 42471 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1410; consecutive; EOF; 1601–2000 confirmed empty.
- Purpose: Constructs low-level SPIR-V instruction objects for declarations, constants, memory, image, ray, arithmetic and control flow.
- Dependencies / cross-references: SPIR-V Op enum, Word operands, instruction layout serializer and backend modules.
- Implementation / logic: Provides typed constructors that assemble opcode, result/type IDs and operands before serialization.
- Findings: Branch weights are explicitly TODO; constructors rely on callers to supply operands in spec-defined order and valid IDs.
- RustChunk relevance: Useful as the low-level instruction-emission layer if using Naga SPIR-V backend, not as a standalone renderer.
- Decision: adaptable; reference; Vulkan-specific
- 1.20.1 implications: Pin SPIR-V/Naga versions and validate emitted module against the Vulkan target environment.
- Follow-up checks: Run SPIR-V Tools validation and constructor serialization tests for each opcode family used.
- RustChunk implementation mapping: Optional SPIR-V instruction builder only with the matching backend revision.
- Evidence required before adoption: Opcode/operand fixtures, SPIR-V validation and Vulkan pipeline creation.
- Completion state: COMPLETE — 1,410 logical lines through EOF in three 400-line chunks plus 210 lines; pinned SHA and 42,471-byte size match; not compiled.

### 3146. `third_party/naga/src/back/spv/layout.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 1bd1f43754e1f11ac284d7ad56805d656f66d4d1
- Size: 7273 bytes
- Coverage: 1–225; EOF.
- Purpose: Serializes SPIR-V physical header and logical instruction sections.
- Dependencies / cross-references: SPIR-V instruction vectors, physical/logical layout and reclaimable vectors.
- Implementation / logic: Writes magic/version/generator/bound/schema header and emits capabilities, extensions, entry points, declarations and function sections in required order.
- Findings: Correctness depends on the ID bound and instruction section ordering; `Instruction::to_words` serializes word count/opcode/type/result/operands.
- RustChunk relevance: Useful low-level module serialization reference for Vulkan backend.
- Decision: adaptable; reference; Vulkan-specific
- 1.20.1 implications: Keep the emitted SPIR-V version and generator metadata aligned with selected Vulkan environment.
- Follow-up checks: Validate word counts, ID bound, section ordering and header with SPIR-V Tools.
- RustChunk implementation mapping: Optional SPIR-V module serializer with pinned instruction model.
- Evidence required before adoption: Golden word streams and SPIR-V validation.
- Completion state: COMPLETE — 225 logical lines through EOF; pinned SHA and 7,273-byte size match; inspected only.

### 3147. `third_party/naga/src/back/spv/mesh_shader.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: c49f944eba59b766dffba50596f11f7a21fc2ef3
- Size: 41490 bytes
- Coverage: 1–400, 401–800, 801–1027; consecutive; EOF; 1201–1600 confirmed empty.
- Purpose: Generates SPIR-V mesh/task shader output variables and wrapper loops.
- Dependencies / cross-references: Naga mesh/task IR, SPIR-V Writer, output bindings, workgroup barriers and dispatch limits.
- Implementation / logic: Creates per-vertex/per-primitive output blocks, emits SetMeshOutputs, copies workgroup outputs and bounds task dispatch products.
- Findings: `mesh_shader_primitive_indices_clamp` option is documented as unimplemented; output handling assumes validated builtins/bindings and uses unwraps.
- RustChunk relevance: Potential Vulkan mesh/task shader reference only; feature support on Android Mali is uncertain and not baseline.
- Decision: adaptable; reference; Vulkan-specific; advanced-feature
- 1.20.1 implications: Query Vulkan mesh-shader extension/device support; don't assume GLES or Mali model name implies it.
- Follow-up checks: Validate output limits, primitive indices, task dispatch overflow and synchronization on actual hardware.
- RustChunk implementation mapping: Optional Vulkan mesh/task stage only if device extension exists.
- Evidence required before adoption: SPIR-V validation and Vulkan mesh/task execution tests.
- Completion state: COMPLETE — 1,027 logical lines through EOF in two 400-line chunks plus 227 lines; pinned SHA and 41,490-byte size match; not compiled.

### 3148. `third_party/naga/src/back/spv/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: c5c817fcdebbe7aee2d05df27961bd4401325c0a
- Size: 44420 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1229; consecutive; EOF; 1601–2000 confirmed empty.
- Purpose: Public SPIR-V backend API, Writer/Options, type model, capability declarations and module orchestration.
- Dependencies / cross-references: SPIR-V block/image/index/instruction/layout/mesh/ray/subgroup modules, Naga IR and Vulkan capability types.
- Implementation / logic: Defines Writer state, type deduplication, binding maps, shader options, capability checks and output assembly.
- Findings: `fake_missing_bindings` defaults to true although docs warn it may produce invalid SPIR-V; `mesh_shader_primitive_indices_clamp` is documented unimplemented despite default true. `supported_capabilities` is a static superset, not device detection.
- RustChunk relevance: Highly relevant if RustChunk chooses Vulkan; default options must be hardened and capabilities constrained to queried device support.
- Decision: adaptable; reference; Vulkan-specific; configuration-critical
- 1.20.1 implications: For production, provide complete binding maps, disable fake bindings, set explicit capabilities and validate on target Vulkan driver.
- Follow-up checks: Test missing bindings, exact capabilities, SPIR-V versions, workgroup zero-init, integer division checks and mesh index clamp behavior.
- RustChunk implementation mapping: Potential Naga SPIR-V backend integration behind Vulkan renderer; separate from Forge Java runtime.
- Evidence required before adoption: SPIR-V Tools validation and real Mali Vulkan pipeline tests with strict options.
- Completion state: COMPLETE — 1,229 logical lines through EOF in three 400-line chunks plus 29 lines; pinned SHA and 44,420-byte size match; not compiled.

### 3149. `third_party/naga/src/back/spv/ray/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: a7f672e6485c03d1b6d09ab79fbcf57e2de9c302
- Size: 16307 bytes
- Coverage: 1–400, 401–470; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Shared SPIR-V ray-query and ray-pipeline descriptor extraction/validation helpers.
- Dependencies / cross-references: Naga ray descriptor, RayFlag bits, SPIR-V writer and ray query/pipeline wrappers.
- Implementation / logic: Extracts ray flags, origin/direction and t ranges; optionally validates finite values, range order and mutually exclusive flag combinations.
- Findings: Invalid descriptors are guarded only when validation option is enabled; debug printf depends on non-semantic extension availability.
- RustChunk relevance: Only relevant if RustChunk Vulkan renderer deliberately supports ray queries/pipelines; not a Minecraft chunk renderer requirement.
- Decision: unnecessary; reference; Vulkan-specific; ray-tracing-only
- 1.20.1 implications: Do not require ray tracing for Android baseline; query optional Vulkan extension/device support first.
- Follow-up checks: Test NaN/Inf, negative tmin, tmin>tmax and conflicting ray flags with validation on/off.
- RustChunk implementation mapping: No current mapping; optional future Vulkan ray backend.
- Evidence required before adoption: SPIR-V validation and GPU ray-query tests on supported devices.
- Completion state: COMPLETE — 470 logical lines through EOF in 400/70 ranges; pinned SHA and 16,307-byte size match; not compiled.

### 3150. `third_party/naga/src/back/spv/ray/pipeline.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 57c67ac1191c347837f993dc450fe9b67a860dbe
- Size: 5050 bytes
- Coverage: 1–125; EOF; 401–800 confirmed empty.
- Purpose: Emits SPIR-V trace-ray wrapper functions for ray-tracing pipeline calls.
- Dependencies / cross-references: Naga RayPipelineFunction, acceleration structures, payload globals, shared ray descriptor validation and SPIR-V instructions.
- Implementation / logic: Builds a cached trace-ray function, validates descriptor when enabled, emits TraceRayKHR and skips invalid calls.
- Findings: SBT offset, stride and miss index are hard-coded to zero in this wrapper; this matches the limited current IR surface but is a pipeline capability constraint to verify.
- RustChunk relevance: No Android baseline value; ray tracing is optional and device-specific.
- Decision: unnecessary; reference; Vulkan-specific; ray-tracing-only
- 1.20.1 implications: Do not include ray tracing in initial Android renderer unless actual device/driver and shader requirements demand it.
- Follow-up checks: Test payload layout, SBT assumptions, validation branches and runtime trace behavior on a supported Vulkan device.
- RustChunk implementation mapping: No current mapping; optional future Vulkan ray-tracing module.
- Evidence required before adoption: SPIR-V validation plus Vulkan ray-tracing pipeline execution tests.
- Completion state: COMPLETE — 125 logical lines through EOF; pinned SHA and 5,050-byte size match; not compiled.

## Checkpoint 3126–3150 — COMPLETE

- 25 new source files inspected one at a time from the pinned Starling snapshot after excluding all 3,125 prior Index paths.
- All 25 source blob SHAs and byte sizes match the complete pinned tree (5,749 entries; untruncated). Canonical logical line counts and complete EOF coverage are recorded.
- Long files were read in consecutive, gap-free ranges no larger than 400 lines: 3126 (940), 3127 (468), 3128 (833), 3129 (564), 3130 (647), 3131 (5,106), 3134 (438), 3135 (883), 3136 (533), 3138 (8,588), 3139 (1,217), 3140 (4,400), 3143 (1,383), 3145 (1,410), 3147 (1,027), 3148 (1,229) and 3149 (470).
- Every new entry has exactly 15 required fields. The checkpoint covers HLSL, MSL and SPIR-V backends; HLSL/MSL are platform-specific, while SPIR-V is a possible Vulkan path subject to actual device capability and driver tests.
- Static concerns include MSL external-YUV plane clamping and SPIR-V default fake bindings / unimplemented mesh primitive-index clamp; none was reproduced by tests.
- No tests/builds/benchmarks/APK/device/Vulkan/Forge/Minecraft/server/Aternos validation was run. No RustChunk application source was changed.

### 3151. `third_party/naga/src/back/spv/ray/query.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 673c240797831d99458885ca15d4a25a614d4a3c
- Size: 55936 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1624; consecutive; EOF; 2001–2400 confirmed empty.
- Purpose: Lowers SPIR-V ray-query operations and tracks candidate/committed intersection state.
- Dependencies / cross-references: Naga RayQuery IR, SPIR-V writer, ray descriptor helpers and query initialization tracking.
- Implementation / logic: Emits initialize/proceed/confirm/terminate and intersection-property instructions, including candidate-vs-committed selection.
- Findings: Ray query requires Vulkan ray-query extension/device support; tracking and validation options affect emitted state handling. No runtime validation was performed.
- RustChunk relevance: Only a future optional Vulkan ray-query path; not required for Minecraft chunk rendering.
- Decision: unnecessary; reference; Vulkan-ray-query-only
- 1.20.1 implications: Do not make ray queries a baseline dependency for Android or Forge 1.20.1.
- Follow-up checks: Test candidate/committed triangle/AABB paths, proceed exhaustion, terminate-before-proceed and initialization tracking on supported Vulkan devices.
- RustChunk implementation mapping: No current mapping; optional future Vulkan ray-query backend.
- Evidence required before adoption: SPIR-V validation and real-device ray-query tests on an extension-capable GPU.
- Completion state: COMPLETE — 1,624 logical lines through EOF in four 400-line chunks plus 24 lines; pinned SHA and 55,936-byte size match; not compiled.

### 3152. `third_party/naga/src/back/spv/reclaimable.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: f0c799eb1f652df1114b0100cb0b24c5bd20894f
- Size: 2412 bytes
- Coverage: 1–76; EOF.
- Purpose: Defines reusable/reclaimable allocation storage for SPIR-V writer internals.
- Dependencies / cross-references: Vec buffers and SPIR-V writer lifecycle/reset code.
- Implementation / logic: Provides a reclaim trait and implementations intended to clear/reuse allocations between writer runs.
- Findings: The trait documentation says large buffers should shrink when much larger than current use, but the Vec implementation only calls `clear()`; capacity is retained. Static doc/implementation mismatch with possible memory retention after unusually large shaders.
- RustChunk relevance: Memory reuse matters for repeated shader compilation on a 6GB Android device, but retained peak capacity may increase memory pressure.
- Decision: adaptable; reference; memory-management-sensitive
- 1.20.1 implications: If adapted, set explicit capacity-retention thresholds for repeated shader compilation and release oversized buffers under memory pressure.
- Follow-up checks: Benchmark repeated large/small shader workloads and measure retained capacities after reset.
- RustChunk implementation mapping: RustChunk shader compilation buffer pool/reclaim policy.
- Evidence required before adoption: Allocation/capacity tests and Android memory-pressure profiling.
- Completion state: COMPLETE — 76 logical lines through EOF; pinned SHA and 2,412-byte size match; not tested.

### 3153. `third_party/naga/src/back/spv/selection.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: b217db637c37967e57a0eff4d46423dd9652629d
- Size: 9972 bytes
- Coverage: 1–260; EOF.
- Purpose: Builds SPIR-V structured selection/if-else control-flow blocks and merge handling.
- Dependencies / cross-references: SPIR-V Writer block state, labels, conditional branches and expression IDs.
- Implementation / logic: Creates selection merge blocks, conditional branches, tracks branch-local values and emits merge/phi behavior.
- Findings: Correctness depends on structured-control-flow invariants and all incoming values being represented; the helper is backend-internal.
- RustChunk relevance: Useful reference for shader compiler control flow, not game tick or chunk task scheduling.
- Decision: adaptable; reference; Vulkan-specific; control-flow-sensitive
- 1.20.1 implications: Only reuse as part of a pinned SPIR-V backend; do not transplant into RustChunk's job scheduler.
- Follow-up checks: Test nested if/else, empty branches, early exits and value merges with SPIR-V Tools validation.
- RustChunk implementation mapping: Optional shader backend control-flow helper.
- Evidence required before adoption: Differential tests against Naga IR and SPIR-V validation.
- Completion state: COMPLETE — 260 logical lines through EOF; pinned SHA and 9,972-byte size match; not executed.

### 3154. `third_party/naga/src/back/spv/subgroup.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: a0c35f1d64895909ee6742f929f50c39b1746bda
- Size: 9211 bytes
- Coverage: 1–223; EOF.
- Purpose: Lowers subgroup ballot, collective and gather operations to SPIR-V group-nonuniform instructions.
- Dependencies / cross-references: Naga subgroup IR, SPIR-V capabilities, scope IDs and instruction builder.
- Implementation / logic: Maps all/any, reductions/scans, broadcast/shuffle and quad operations to SPIR-V opcodes and capabilities.
- Findings: Some combinations are explicitly unimplemented; subgroup availability/size and operation support vary by Vulkan device and extension.
- RustChunk relevance: Potential GPU optimization for parallel chunk/mesh work only if target device supports the required subgroup operations.
- Decision: adaptable; reference; Vulkan-subgroup-sensitive
- 1.20.1 implications: Query subgroup size/stages/operations on the actual Mali Vulkan driver; always retain a fallback path.
- Follow-up checks: Test each subgroup op and type across supported stages, subgroup sizes and fallback behavior.
- RustChunk implementation mapping: Optional subgroup acceleration in Vulkan compute/mesh pipeline.
- Evidence required before adoption: Device feature queries, SPIR-V validation and GPU correctness/performance comparisons.
- Completion state: COMPLETE — 223 logical lines through EOF; pinned SHA and 9,211-byte size match; not compiled.

### 3155. `third_party/naga/src/back/spv/writer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: ea6a9530fa8632850c5ef11b449e4a1d09b0f926
- Size: 170102 bytes
- Coverage: 1–400 through 3601–4000 in ten consecutive 400-line chunks; EOF; 4001–4400 confirmed empty.
- Purpose: Top-level SPIR-V Writer lifecycle, module emission, types/constants/resources, options and output assembly.
- Dependencies / cross-references: SPIR-V block/image/index/instruction/layout/ray/mesh/subgroup helpers, Naga IR and Vulkan capability structures.
- Implementation / logic: Owns module IDs, capability/declaration sections, functions, entry points, binding decorations, options, reset/reuse and final binary emission.
- Findings: `set_options` appears not to update `fake_missing_bindings`, `ray_query_initialization_tracking`, `trace_ray_argument_validation` and `emit_int_div_checks`; changing these on a reused Writer may leave stale settings. Static concern, not reproduced.
- RustChunk relevance: Strong Vulkan backend reference; option reuse and memory behavior are important for repeated shader reloads on Android.
- Decision: adaptable; reference; Vulkan-specific; configuration-critical
- 1.20.1 implications: Use a pinned Naga revision, construct strict Options per pipeline, provide complete bindings and avoid relying on incomplete `set_options` until fixed/tested.
- Follow-up checks: Add tests that change each omitted option between writes on the same Writer; validate resulting SPIR-V and reset behavior.
- RustChunk implementation mapping: Potential Naga SPIR-V Writer inside an Android Vulkan renderer abstraction.
- Evidence required before adoption: SPIR-V Tools validation, option-transition tests and target Mali pipeline creation.
- Completion state: COMPLETE — 4,000 logical lines through EOF in ten consecutive 400-line chunks; pinned SHA and 170,102-byte size match; not compiled.

### 3156. `third_party/naga/src/back/wgsl/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 74c438aa8ffa7b952edfdbdb1c69204e6b1a3bd3
- Size: 2565 bytes
- Coverage: 1–96; EOF.
- Purpose: Public WGSL backend API and error/capability definitions.
- Dependencies / cross-references: Naga IR, WGSL writer, WGSL type formatting and enabled-feature analysis.
- Implementation / logic: Exposes WGSL source generation entry points, writer options and backend error variants.
- Findings: WGSL output support is not equivalent to WebGPU runtime availability; some Naga IR constructs are explicitly unsupported by the writer.
- RustChunk relevance: WGSL is a portable shader representation but not the existing Minecraft GLSL pipeline format.
- Decision: adaptable; reference; WGSL-backend-specific
- 1.20.1 implications: Only adopt if RustChunk intentionally uses a WGSL/WebGPU path; Android GLES/Vulkan renderer still needs a compiled backend.
- Follow-up checks: Test WGSL output against the selected WGSL parser/validator and target WebGPU implementation.
- RustChunk implementation mapping: Optional WGSL export/diagnostic backend; not Forge integration.
- Evidence required before adoption: Round-trip parse/validate tests and actual runtime shader compilation.
- Completion state: COMPLETE — 96 logical lines through EOF; pinned SHA and 2,565-byte size match; not compiled.

### 3157. `third_party/naga/src/back/wgsl/polyfill/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 970a83a53ce783a945dcf82aa44cf1f74e3cd106
- Size: 2320 bytes
- Coverage: 1–66; EOF.
- Purpose: Declares WGSL polyfill generation for operations not directly available in WGSL.
- Dependencies / cross-references: WGSL writer, Naga MathFunction/IR and helper generation.
- Implementation / logic: Provides inverse-matrix polyfill generation hooks for matrix sizes/types that need synthesized WGSL code.
- Findings: Polyfills change generated shader text and must preserve numerical behavior/precision; this module is a helper, not a full backend.
- RustChunk relevance: Potential reference for shader compatibility work if WGSL is used.
- Decision: adaptable; reference; shader-polyfill
- 1.20.1 implications: Minecraft 1.20.1 renderer shaders are not automatically WGSL-compatible; only use with a validated WGSL translation pipeline.
- Follow-up checks: Compare inverse results for 2x2/3x3/4x4 matrices, singular inputs and precision edge cases.
- RustChunk implementation mapping: Optional WGSL shader polyfill module.
- Evidence required before adoption: Numerical GPU tests against CPU reference and WGSL compiler validation.
- Completion state: COMPLETE — 66 logical lines through EOF; pinned SHA and 2,320-byte size match; not executed.

### 3158. `third_party/naga/src/back/wgsl/writer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 643a8a01ffbf1c970df8d157aa96ce4858310269
- Size: 86042 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2186; consecutive; EOF; 2401–2800 confirmed empty.
- Purpose: Generates WGSL source from Naga IR, including declarations, types, expressions, statements, entry points and enables.
- Dependencies / cross-references: Naga IR/validation, common WGSL type formatting, name sanitization, polyfills, image and subgroup operations.
- Implementation / logic: Emits WGSL module source, resources, bindings, entry-point IO, functions, overrides and generated helper code.
- Findings: Several valid Naga IR paths outside the WGSL subset use `unimplemented!()`/`unreachable!()` (notably subgroup/ray/cooperative combinations); switch fallthrough is explicitly unimplemented. Also the ray-pipeline enable scan appears duplicated, a harmless redundancy.
- RustChunk relevance: WGSL is useful for portable shader tooling but not a direct replacement for Minecraft's GLSL shader packs.
- Decision: adaptable; reference; WGSL-backend-specific; feature-limited
- 1.20.1 implications: Use only after deciding on WGSL/WebGPU and constraining input IR to the supported WGSL subset.
- Follow-up checks: Test every enabled IR feature, subgroup combinations, ray/cooperative constructs, switch fallthrough and generated extension directives.
- RustChunk implementation mapping: Optional WGSL export/backend separated from Android GLES/Vulkan runtime.
- Evidence required before adoption: WGSL parser/validator round-trip and target runtime shader tests.
- Completion state: COMPLETE — 2,186 logical lines through EOF in five 400-line chunks plus 186 lines; pinned SHA and 86,042-byte size match; not compiled.

### 3159. `third_party/naga/src/common/diagnostic_debug.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: efa3ec548b3db486f1a310e6237adf035e4b4606
- Size: 3985 bytes
- Coverage: 1–126; EOF.
- Purpose: Formats Naga IR types/scalars for debug logging.
- Dependencies / cross-references: Naga TypeContext, type arenas, handles and Debug formatting.
- Implementation / logic: Wraps IR values in `DiagnosticDebug` and implements contextual `Debug` output with WGSL-aware formatting when enabled.
- Findings: Lower fidelity by design; anonymous types/overrides may be represented by handles or placeholders.
- RustChunk relevance: Useful for RustChunk diagnostics around shader IR and resource types.
- Decision: adaptable; reference; developer-diagnostics
- 1.20.1 implications: No Minecraft 1.20.1 runtime dependency; keep logging feature-gated to avoid hot-path formatting.
- Follow-up checks: Test output with WGSL features enabled/disabled and anonymous types/overrides.
- RustChunk implementation mapping: RustChunk shader compiler debug formatting.
- Evidence required before adoption: Feature-matrix compile tests and diagnostic snapshot tests.
- Completion state: COMPLETE — 126 logical lines through EOF; pinned SHA and 3,985-byte size match; inspected only.

### 3160. `third_party/naga/src/common/diagnostic_display.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: d5ee7c07dc4fd39e9e9b273608fd733cdd11f9c0
- Size: 4262 bytes
- Coverage: 1–124; EOF.
- Purpose: Formats Naga IR types and validation rules for user-facing diagnostics.
- Dependencies / cross-references: GlobalCtx, TypeResolution, Rule, WGSL TypeContext and fmt::Display.
- Implementation / logic: Provides contextual display wrappers for types, scalars and type rules.
- Findings: Documentation states language-sensitive diagnostics are currently WGSL-only; GLSL-origin diagnostics may not use source-language notation.
- RustChunk relevance: Useful for readable shader errors, but source-language tagging is needed for multi-front-end correctness.
- Decision: adaptable; reference; developer-diagnostics
- 1.20.1 implications: RustChunk should preserve shader source language/label through parse, validation and compilation errors.
- Follow-up checks: Test GLSL/WGSL/SPIR-V source diagnostics and verify type notation matches originating language.
- RustChunk implementation mapping: RustChunk shader error reporting layer.
- Evidence required before adoption: Golden diagnostic tests per source language.
- Completion state: COMPLETE — 124 logical lines through EOF; pinned SHA and 4,262-byte size match; inspected only.

### 3161. `third_party/naga/src/common/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: e1fe06d5c0ad7c458890f340fb2aa17beb285f72
- Size: 357 bytes
- Coverage: 1–12; EOF.
- Purpose: Declares and re-exports shared language-specific diagnostic and WGSL utilities.
- Dependencies / cross-references: diagnostic_debug, diagnostic_display, predeclared and wgsl modules.
- Implementation / logic: Small module wiring for shared front/back-end helpers.
- Findings: No independent runtime logic; feature availability follows submodules.
- RustChunk relevance: Useful organizational pattern only.
- Decision: adaptable; reference; module-organization
- 1.20.1 implications: Keep feature gates consistent with Cargo features selected for Android.
- Follow-up checks: Compile with minimal and full shader feature sets.
- RustChunk implementation mapping: RustChunk shader-common module organization.
- Evidence required before adoption: Cargo feature-matrix builds.
- Completion state: COMPLETE — 12 logical lines through EOF; pinned SHA and 357-byte size match; inspected only.

### 3162. `third_party/naga/src/common/predeclared.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 0c2004f7fd36cbeec63ddadb65f49524c05022f2
- Size: 1002 bytes
- Coverage: 1–33; EOF.
- Purpose: Generates stable names for predeclared WGSL result struct types.
- Dependencies / cross-references: Naga PredeclaredType, scalar kind/width and vector size.
- Implementation / logic: Formats names for atomic compare-exchange, modf and frexp result structures.
- Findings: Generated names are internal WGSL identifiers; name collision/hygiene must be handled by the caller's Namer.
- RustChunk relevance: Useful if RustChunk uses Naga WGSL diagnostics or backend.
- Decision: adaptable; reference; WGSL-name-generation
- 1.20.1 implications: No direct Forge or Android renderer behavior.
- Follow-up checks: Test name stability across scalar widths and vector sizes, and collisions with user identifiers.
- RustChunk implementation mapping: Optional internal WGSL type-name generator.
- Evidence required before adoption: Name hygiene and WGSL parser tests.
- Completion state: COMPLETE — 33 logical lines through EOF; pinned SHA and 1,002-byte size match; inspected only.

### 3163. `third_party/naga/src/common/wgsl/diagnostics.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: a78343b60ab23b74e96a720224788b61d4d27984
- Size: 2397 bytes
- Coverage: 1–73; EOF.
- Purpose: Maps WGSL diagnostic severity/filtering rules to WGSL identifiers and Display output.
- Dependencies / cross-references: DiagnosticFilter, Severity and StandardFilterableTriggeringRule.
- Implementation / logic: Parses sentinel identifiers for error/warning/info/off and formats standard/user triggering rules.
- Findings: Only derivative_uniformity is represented as a standard rule here; lesser severities are not fully reported by Naga yet.
- RustChunk relevance: Useful only if RustChunk accepts or emits WGSL diagnostics.
- Decision: adaptable; reference; WGSL-diagnostics
- 1.20.1 implications: No direct Forge integration; feature-gate WGSL diagnostics.
- Follow-up checks: Test unknown/user rules, duplicate filters, severity precedence and parser spans.
- RustChunk implementation mapping: Optional WGSL diagnostic filter support.
- Evidence required before adoption: Parser tests and severity/reporting tests.
- Completion state: COMPLETE — 73 logical lines through EOF; pinned SHA and 2,397-byte size match; inspected only.

### 3164. `third_party/naga/src/common/wgsl/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: fe4cfac9d4cdb0275f9f19bbabe8859dcb915080
- Size: 236 bytes
- Coverage: 1–9; EOF.
- Purpose: Declares and re-exports WGSL type/source formatting helpers.
- Dependencies / cross-references: diagnostics, to_wgsl and types modules.
- Implementation / logic: Small module wiring for WGSL shared front/back functionality.
- Findings: No independent logic; availability depends on WGSL feature flags.
- RustChunk relevance: Only relevant if WGSL parsing/output is enabled.
- Decision: adaptable; reference; module-organization
- 1.20.1 implications: No direct Minecraft 1.20.1 runtime mapping.
- Follow-up checks: Compile with WGSL features both enabled and disabled.
- RustChunk implementation mapping: RustChunk shader-language common module.
- Evidence required before adoption: Cargo feature-matrix compile tests.
- Completion state: COMPLETE — 9 logical lines through EOF; pinned SHA and 236-byte size match; inspected only.

### 3165. `third_party/naga/src/common/wgsl/to_wgsl.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: c3fa20832c9a44898a109ca7691ffd7c27697f34
- Size: 14572 bytes
- Coverage: 1–398; EOF.
- Purpose: Maps Naga IR enums, builtins, scalar types, storage formats and address spaces to WGSL spellings.
- Dependencies / cross-references: MathFunction, BuiltIn, Scalar, StorageFormat, AddressSpace and WGSL grammar.
- Implementation / logic: Implements `ToWgsl`/`TryToWgsl` conversions and address-space/access string mapping.
- Findings: Naga-only builtins/types return None or extension spellings; output is not guaranteed standard WGSL for every IR variant.
- RustChunk relevance: Useful for WGSL shader translation/diagnostics, but not for GLSL shader pack execution.
- Decision: adaptable; reference; WGSL-language-mapping
- 1.20.1 implications: Keep WGSL extension support explicit and do not assume every Minecraft shader IR feature maps to WGSL.
- Follow-up checks: Test every enum mapping against a WGSL parser and target runtime.
- RustChunk implementation mapping: RustChunk WGSL type/builtin formatter if WGSL path is selected.
- Evidence required before adoption: Exhaustive mapping tests and WGSL parser validation.
- Completion state: COMPLETE — 398 logical lines through EOF; pinned SHA and 14,572-byte size match; inspected only.

### 3166. `third_party/naga/src/common/wgsl/types.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: a678a617f76744988ade3eb9bd921c884002799c
- Size: 17602 bytes
- Coverage: 1–400, 401–490; consecutive; EOF.
- Purpose: Formats Naga TypeInner/TypeResolution structures as WGSL type declarations and diagnostics.
- Dependencies / cross-references: TypeContext, GlobalCtx, UniqueArena, TypeResolution, predeclared types and scalar/address-space mappings.
- Implementation / logic: Formats vectors, matrices, arrays, binding arrays, pointers, images, samplers, atomics, structs and Naga extensions.
- Findings: Diagnostic TypeContext implementations explicitly may emit invalid WGSL for anonymous structs and override-sized arrays; backend implementation assumes validated modules.
- RustChunk relevance: Useful for shader diagnostics and WGSL generation, but requires Namer/hygiene and validation.
- Decision: adaptable; reference; WGSL-type-formatting
- 1.20.1 implications: Use only with validated 1.20.1 shader IR and a matching WGSL feature set.
- Follow-up checks: Test anonymous/named structs, override-sized arrays, storage access modes and non-WGSL extensions.
- RustChunk implementation mapping: RustChunk shader diagnostic/type formatter.
- Evidence required before adoption: WGSL parser tests and type-formatting snapshots.
- Completion state: COMPLETE — 490 logical lines through EOF in 400/90 ranges; pinned SHA and 17,602-byte size match; inspected only.

### 3167. `third_party/naga/src/compact/expressions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 021401d00ef9d33a7a83d348f905929812cee66a
- Size: 17278 bytes
- Coverage: 1–400, 401–481; consecutive; EOF.
- Purpose: Traces expression dependencies and adjusts expression handles after Naga IR compaction.
- Dependencies / cross-references: Expression arenas, HandleSet/HandleMap, constants, overrides, types and module/function maps.
- Implementation / logic: Walks expression operands in reverse arena order, marks referenced constants/types/globals and rewrites handles after compaction.
- Findings: Correctness relies on Naga's invariant that expressions refer only to earlier arena entries; missed handle variants could silently remove dependencies.
- RustChunk relevance: Useful model for pruning unused shader IR and reducing compile/cache memory.
- Decision: adaptable; reference; IR-compaction-critical
- 1.20.1 implications: Only compact validated modules and preserve exact arena ordering/handle remapping.
- Follow-up checks: Add coverage for every Expression variant, constants/overrides, image/ray/cooperative expressions and stale handles.
- RustChunk implementation mapping: RustChunk shader IR compaction stage if Naga IR is adopted.
- Evidence required before adoption: Property tests comparing module behavior before/after compaction and revalidation.
- Completion state: COMPLETE — 481 logical lines through EOF in 400/81 ranges; pinned SHA and 17,278-byte size match; not tested.

### 3168. `third_party/naga/src/compact/functions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 21d6a95ae07c9c265b81f05def6411ec4ea9db7c
- Size: 4242 bytes
- Coverage: 1–121; EOF.
- Purpose: Traces function calls and compacts function-local expressions/handles.
- Dependencies / cross-references: FunctionTracer, ExpressionTracer, FunctionMap, module maps and named expressions.
- Implementation / logic: Tracks called functions, arguments/results/local initializers, retains live expressions and adjusts statements after arena compaction.
- Findings: Uses assertions that temporary named-expression reuse storage is empty; correctness depends on call graph order and no recursion.
- RustChunk relevance: Useful reference for eliminating unused shader functions and reducing shader compilation work.
- Decision: adaptable; reference; IR-compaction-critical
- 1.20.1 implications: Only run after validation and maintain the upstream no-recursion/function ordering invariants.
- Follow-up checks: Test nested call graphs, unused functions, local initializers, named expressions and entry-point reachability.
- RustChunk implementation mapping: RustChunk shader module optimizer/compactor if Naga IR is adopted.
- Evidence required before adoption: Before/after validation and shader output equivalence tests.
- Completion state: COMPLETE — 121 logical lines through EOF; pinned SHA and 4,242-byte size match; inspected only.

### 3169. `third_party/naga/src/compact/handle_set_map.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: b3cbb909eedc6fd0504ce1e7729553d9ab2fc756
- Size: 4917 bytes
- Coverage: 1–132; EOF.
- Purpose: Maps old arena handles to compacted indices and adjusts contiguous ranges.
- Dependencies / cross-references: HandleSet, Handle, Range, Arena and NonMaxU32.
- Implementation / logic: Builds dense old-to-new maps, remaps handles/options and shrinks ranges to retained endpoints.
- Findings: `used`/`try_adjust` index directly and assume handles belong to the source arena; invalid/stale handles panic by design.
- RustChunk relevance: Potential reusable primitive for compact chunk/mesh/resource arenas if generation invariants are explicit.
- Decision: adaptable; reference; handle-remapping
- 1.20.1 implications: Do not use raw stale handles across asynchronous chunk unload/reload cycles.
- Follow-up checks: Test holes, empty ranges, removed endpoints, max indices and stale/out-of-arena handles.
- RustChunk implementation mapping: RustChunk arena remapping/compaction utility.
- Evidence required before adoption: Property tests comparing all retained old/new handle mappings.
- Completion state: COMPLETE — 132 logical lines through EOF; pinned SHA and 4,917-byte size match; inspected only.

### 3170. `third_party/naga/src/compact/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 4974cea96bdd6c83c541e5c17da46b6a5d36af93
- Size: 46647 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1300; consecutive; EOF; 1601–2000 confirmed empty.
- Purpose: Orchestrates removal of unused Naga module objects and remapping of all surviving handles.
- Dependencies / cross-references: ModuleTracer, FunctionTracer, TypeTracer, expression/statement tracers, HandleMap and arena retain/drain APIs.
- Implementation / logic: Marks entry-point reachability, traces calls/types/constants/overrides, compacts arenas, updates docs and rewrites handles.
- Findings: Panics are documented for invalid modules; the tandem type/expression pass depends on strict arena ordering and override dependency invariants.
- RustChunk relevance: Strong reference for shader IR dead-code elimination and memory reduction; not world/chunk data compaction.
- Decision: adaptable; reference; IR-compaction-critical
- 1.20.1 implications: Only compact validated shader modules and revalidate after mutation; keep compaction off live world-state data.
- Follow-up checks: Run all included compaction tests plus fuzzed modules with arrays/overrides/calls/mesh/ray and compare emitted shaders.
- RustChunk implementation mapping: Optional Naga IR compaction stage, not chunk storage compaction.
- Evidence required before adoption: Before/after validator and backend output equivalence; fuzz/property tests.
- Completion state: COMPLETE — 1,300 logical lines through EOF in three 400-line chunks plus 100 lines; pinned SHA and 46,647-byte size match; tests inspected, not run.

### 3171. `third_party/naga/src/compact/statements.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 4b9dd13c899a5a2f365557aff823b4a7a230698a
- Size: 18290 bytes
- Coverage: 1–400, 401–461; consecutive; EOF.
- Purpose: Traces expression/function dependencies from Naga statements and rewrites statement handles.
- Dependencies / cross-references: FunctionTracer, HandleMap, FunctionMap, Statement, RayQuery, subgroup and atomic IR.
- Implementation / logic: Walks nested blocks/branches/loops iteratively, marks used expressions/calls and adjusts all statement references after compaction.
- Findings: Every Statement variant must be handled in both tracing and adjustment; omission could retain/remove wrong expressions or leave stale handles.
- RustChunk relevance: Useful reference for safe shader IR graph traversal and handle remapping.
- Decision: adaptable; reference; IR-compaction-critical
- 1.20.1 implications: Keep the statement-variant match exhaustive when adopting a newer Naga revision.
- Follow-up checks: Add a test for every Statement variant, nested branches/loops, calls, ray/subgroup/cooperative operations and range adjustments.
- RustChunk implementation mapping: RustChunk shader IR liveness/compaction if Naga is adopted.
- Evidence required before adoption: Property tests and post-compaction validation for every statement family.
- Completion state: COMPLETE — 461 logical lines through EOF in 400/61 ranges; pinned SHA and 18,290-byte size match; not tested.

### 3172. `third_party/naga/src/compact/types.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: d06558b182c31deb39478faa0973b027b51d729f
- Size: 3575 bytes
- Coverage: 1–110; EOF.
- Purpose: Traces type dependencies and adjusts nested type handles during module compaction.
- Dependencies / cross-references: TypeTracer, HandleSet, ModuleMap, overrides and type arenas.
- Implementation / logic: Marks array bases, pending override lengths, pointer bases and struct member types; rewrites corresponding handles.
- Findings: Assumes all referenced types/overrides remain reachable and source module passed validation.
- RustChunk relevance: Reusable as part of Naga IR compaction, not general game data storage.
- Decision: adaptable; reference; IR-compaction
- 1.20.1 implications: Preserve type dependency ordering and override initialization references.
- Follow-up checks: Test nested arrays/structs/pointers/binding arrays and pending override sizes.
- RustChunk implementation mapping: RustChunk shader IR type compaction.
- Evidence required before adoption: Post-compaction validation and type graph equivalence tests.
- Completion state: COMPLETE — 110 logical lines through EOF; pinned SHA and 3,575-byte size match; inspected only.

### 3173. `third_party/naga/src/diagnostic_filter.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 16b8de6699de3f23f3b4aed96845491d99160e4f
- Size: 9517 bytes
- Coverage: 1–274; EOF.
- Purpose: Defines diagnostic severities, filter rules, conflict detection and parent-linked filter lookup.
- Dependencies / cross-references: WGSL front end, DiagnosticFilter arena, spans, severity and standard/user triggering rules.
- Implementation / logic: Stores module/construct-scoped filters, rejects conflicting duplicates and searches most-specific parent rules.
- Findings: Info/warning are logged but Naga notes lesser-severity diagnostics are not fully supported/reported yet; only derivative_uniformity is a standard rule here.
- RustChunk relevance: Useful if RustChunk accepts WGSL shader diagnostics; not part of Minecraft gameplay.
- Decision: adaptable; reference; diagnostic-policy
- 1.20.1 implications: Keep diagnostic filters language-scoped and don't silently treat warnings as fully implemented.
- Follow-up checks: Test nested filters, duplicate conflicts, unknown/user rules and severity overrides.
- RustChunk implementation mapping: RustChunk shader diagnostics configuration.
- Evidence required before adoption: Parser/validator tests for precedence, spans and reporting.
- Completion state: COMPLETE — 274 logical lines through EOF; pinned SHA and 9,517-byte size match; inspected only.

### 3174. `third_party/naga/src/error.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 43e0b0b50446904009312abc49da860cd37ac6f6
- Size: 6480 bytes
- Coverage: 1–196; EOF.
- Purpose: Wraps shader source, labels and parser/validation errors into displayable error diagnostics.
- Dependencies / cross-references: WGSL/GLSL/SPIR-V front ends, codespan-reporting, ShaderError and diagnostic buffers.
- Implementation / logic: Formats source-located parse/validation errors, supports color/no-color/stdout feature configurations and replaces control characters.
- Findings: `Error::source` for ShaderError returns `self.inner.source()` rather than `Some(self.inner.as_ref())`; this appears to skip the immediate wrapped parser/validation error in the standard error chain. Static concern, not tested.
- RustChunk relevance: Useful for RustChunk shader error reporting; error-chain visibility matters for logs and support.
- Decision: adaptable; reference; error-reporting
- 1.20.1 implications: Preserve original shader source/label and expose the underlying error correctly through `std::error::Error`.
- Follow-up checks: Test error-chain traversal, color feature combinations, malformed UTF-8/control chars and source spans.
- RustChunk implementation mapping: RustChunk shader compilation error wrapper.
- Evidence required before adoption: Error source-chain tests and golden diagnostic output.
- Completion state: COMPLETE — 196 logical lines through EOF; pinned SHA and 6,480-byte size match; inspected only.

### 3175. `third_party/naga/src/front/atomic_upgrade.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 549fcf69abd561fd831551a377ef4c56d68dd72f
- Size: 9774 bytes
- Coverage: 1–273; EOF.
- Purpose: Upgrades SPIR-V scalar globals/struct fields accessed atomically into Naga Atomic types.
- Dependencies / cross-references: SPIR-V front end, global/struct types, atomic operations, Upgrades sets and type arena.
- Implementation / logic: Tracks atomic global/field accesses, recursively rebuilds pointer/array/struct types and replaces global variable types.
- Findings: `Upgrades::is_empty` checks only `globals.is_empty()` and ignores `fields`; field-only upgrade sets are reported empty. This may be safe only if callers always insert the owning global too; static invariant needs verification.
- RustChunk relevance: Relevant if RustChunk imports SPIR-V shaders and needs Naga's atomic IR normalization.
- Decision: adaptable; reference; SPIR-V-front-end-specific; correctness-sensitive
- 1.20.1 implications: Use only with the matching SPIR-V front end; ensure field access tracking always marks its owning global or correct `is_empty`.
- Follow-up checks: Test atomic accesses to direct globals, nested struct fields, arrays, repeated fields and field-only Upgrades state.
- RustChunk implementation mapping: Optional SPIR-V atomic normalization stage if Naga front end is adopted.
- Evidence required before adoption: SPIR-V input corpus, type validation and unit tests for Upgrades::is_empty.
- Completion state: COMPLETE — 273 logical lines through EOF; pinned SHA and 9,774-byte size match; inspected, not tested.

## Checkpoint 3151–3175 — COMPLETE

- 25 source files inspected one at a time from the pinned Starling snapshot after excluding all 3,150 prior Index paths.
- All 25 source blob SHAs and byte sizes match the complete pinned tree (5,749 entries; untruncated). Canonical logical line counts and complete EOF coverage are recorded.
- Long files were read in consecutive, gap-free ranges no larger than 400 lines: 3151 (1,624), 3155 (4,000), 3158 (2,186), 3166 (490), 3167 (481), 3170 (1,300) and 3171 (461).
- Every new entry has exactly 15 required fields. Static concerns include SPIR-V Writer option updates, reclaimable buffer capacity retention, ShaderError error-chain behavior and atomic-upgrade emptiness tracking; none was reproduced by tests.
- No tests/builds/benchmarks/APK/device/Vulkan/Forge/Minecraft/server/Aternos validation was run. No RustChunk application source was changed.

### 3176. `third_party/naga/src/front/glsl/ast.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 5e1033944a1ee3a2ca9c474088bd666bb38e4a14
- Size: 12109 bytes
- Coverage: 1–400; EOF; 401–800 confirmed empty.
- Purpose: Defines the GLSL front-end AST and source-span-bearing syntax nodes.
- Dependencies / cross-references: GLSL lexer tokens, parser and AST visitor/context lowering.
- Implementation / logic: Represents declarations, expressions, statements, functions, types and control-flow constructs before lowering to Naga IR.
- Findings: AST correctness is only one stage; unsupported GLSL grammar/extension constructs are rejected by parser/lowering layers.
- RustChunk relevance: Useful reference if RustChunk needs to import GLSL shader packs through Naga.
- Decision: adaptable; reference; GLSL-front-end-architecture
- 1.20.1 implications: Minecraft 1.20.1 shader packs may use GLSL extensions beyond this front end's supported profile.
- Follow-up checks: Test source spans and every grammar node against representative 1.20.1 shader packs.
- RustChunk implementation mapping: Optional GLSL AST/parser integration in shader import pipeline.
- Evidence required before adoption: Parser fixtures, source-span checks and Naga validation.
- Completion state: COMPLETE — 400 logical lines through EOF; pinned SHA and 12,109-byte size match; inspected only.

### 3177. `third_party/naga/src/front/glsl/builtins.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 3b6a7e1018121b00ad46ea43f604ba6775d52b86
- Size: 112521 bytes
- Coverage: 1–400 through 2401–2800 in seven consecutive 400-line chunks, then 2801–2913; EOF; 3201–3600 confirmed empty.
- Purpose: Lowers GLSL builtin functions/macros, texture operations, atomics and image calls into Naga IR.
- Dependencies / cross-references: GLSL parser/context, builtin signatures, Naga expressions/statements, texture/image types and atomic upgrade tracking.
- Implementation / logic: Resolves overloads and constructs IR for math, geometric, relational, texture, image and atomic builtins.
- Findings: Image atomic builtins load the texel before emitting a separate atomic statement and return that earlier load as the old value; another invocation can write between the load and atomic, so the returned old value is not atomic. The source comment acknowledges the race. Correctness defect for concurrent image atomics.
- RustChunk relevance: Directly relevant to GLSL shader-pack compatibility; this semantic mismatch can break modded shaders using imageAtomic* under contention.
- Decision: rewrite required; reference; semantic-correctness-critical
- 1.20.1 implications: Do not adopt this path unchanged for 1.20.1 shader packs that use image atomics; implement an IR atomic result that returns the actual old value.
- Follow-up checks: Create concurrent imageAtomicAdd/Min/CompSwap fixtures and compare returned values against GLSL/Vulkan reference behavior.
- RustChunk implementation mapping: GLSL builtin lowering only after atomic return semantics are fixed; separate from chunk streaming.
- Evidence required before adoption: GPU concurrency tests with multiple invocations updating the same texel and validating returned old values.
- Completion state: COMPLETE — 2,913 logical lines through EOF in seven 400-line chunks plus 113 lines; pinned SHA and 112,521-byte size match; not compiled.

### 3178. `third_party/naga/src/front/glsl/context.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 87603a1f65bacfc6aef379f92a4da4d841f1886b
- Size: 61257 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600; consecutive; EOF; 1601–2000 confirmed empty.
- Purpose: Provides GLSL parser context, symbol/type resolution, expression lowering and statement construction.
- Dependencies / cross-references: GLSL AST, Type/Function/Variable declarations, Naga arenas, typifier and source spans.
- Implementation / logic: Maintains scopes, resolves symbols/types, creates local/global variables and lowers high-level expressions/statements into Naga IR.
- Findings: Large semantic bridge with many version/extension-specific assumptions; error recovery and type inference must remain aligned with parser/validator.
- RustChunk relevance: Central reference for importing GLSL shader packs into a Rust shader pipeline.
- Decision: adaptable; reference; GLSL-front-end-specific; correctness-sensitive
- 1.20.1 implications: Pin the exact Naga revision and test all GLSL extensions used by the selected Minecraft shader packs.
- Follow-up checks: Test scope shadowing, lvalues, swizzles, image/atomic access, precision qualifiers and invalid type paths.
- RustChunk implementation mapping: Optional GLSL-to-Naga IR frontend; no direct world/chunk runtime mapping.
- Evidence required before adoption: Shader corpus parse/lower/validate tests and source-span diagnostics.
- Completion state: COMPLETE — 1,600 logical lines through EOF in four consecutive 400-line chunks; pinned SHA and 61,257-byte size match; not compiled.

### 3179. `third_party/naga/src/front/glsl/error.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 8f5d573089781de619b6cdb2241d97a81ee71526
- Size: 9139 bytes
- Coverage: 1–248; EOF.
- Purpose: Defines GLSL parser/lowering error variants and source-location context.
- Dependencies / cross-references: GLSL parser, lexer, AST spans and Naga diagnostic reporting.
- Implementation / logic: Represents unsupported versions/extensions, invalid operands/types, parse failures and source spans.
- Findings: Errors are frontend-specific; some unsupported cases may be reported at lowering rather than lexical parsing.
- RustChunk relevance: Useful for clear shader-pack import diagnostics.
- Decision: adaptable; reference; parser-diagnostics
- 1.20.1 implications: Preserve original GLSL source and version in 1.20.1 shader error reports.
- Follow-up checks: Test malformed tokens, unsupported extensions, invalid overloads and precise source spans.
- RustChunk implementation mapping: RustChunk shader import error type and diagnostics adapter.
- Evidence required before adoption: Golden diagnostic tests for real shader-pack failures.
- Completion state: COMPLETE — 248 logical lines through EOF; pinned SHA and 9,139-byte size match; inspected only.

### 3180. `third_party/naga/src/front/glsl/functions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: a4480236529c581ac32f4e110a721525c0c3a70d
- Size: 65569 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1713; consecutive; EOF; 2001–2400 confirmed empty.
- Purpose: Handles GLSL function declarations/calls, overload selection, constructors, parameter modes and atomic access discovery.
- Dependencies / cross-references: GLSL context/types, function AST, Naga Function/Expression/Statement arenas and atomic upgrade map.
- Implementation / logic: Resolves overload candidates/conversions, builds function IR, handles in/out/inout arguments and tracks atomic pointer paths.
- Findings: Overload/constructor logic depends on exact GLSL conversion ranking; unsupported or ambiguous signatures must fail deterministically.
- RustChunk relevance: Relevant to modded GLSL shader-pack import and function-heavy shaders.
- Decision: adaptable; reference; overload-resolution-sensitive
- 1.20.1 implications: Use only with a pinned GLSL/Naga version and compatibility fixtures for actual 1.20.1 shader packs.
- Follow-up checks: Test ambiguous overloads, matrix/vector constructors, out/inout swizzles, recursion, atomic struct fields and extension functions.
- RustChunk implementation mapping: Optional GLSL function lowering layer.
- Evidence required before adoption: Overload resolution fixtures, Naga validation and shader output equivalence.
- Completion state: COMPLETE — 1,713 logical lines through EOF in four 400-line chunks plus 113 lines; pinned SHA and 65,569-byte size match; not compiled.

### 3181. `third_party/naga/src/front/glsl/lex.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 9337491b5dae57490f8122b0bb72ce302ec9595f
- Size: 11660 bytes
- Coverage: 1–306; EOF.
- Purpose: Tokenizes GLSL source into identifiers, literals, operators, punctuation and preprocessor/version markers.
- Dependencies / cross-references: GLSL source text, spans, numeric parsing and token enum.
- Implementation / logic: Scans characters, recognizes numeric/string/identifier forms and attaches source spans for parser diagnostics.
- Findings: Lexical support is profile-specific; extensions, numeric suffixes and preprocessor behavior need shader corpus coverage.
- RustChunk relevance: Useful if RustChunk imports GLSL shader packs directly.
- Decision: adaptable; reference; GLSL-lexer
- 1.20.1 implications: 1.20.1 packs may depend on legacy GLSL versions and vendor extensions.
- Follow-up checks: Test numeric edge cases, comments, escapes, line directives, malformed exponents and non-ASCII identifiers.
- RustChunk implementation mapping: Optional GLSL lexer if Naga frontend is selected.
- Evidence required before adoption: Lexer fuzzing and corpus token/span snapshots.
- Completion state: COMPLETE — 306 logical lines through EOF; pinned SHA and 11,660-byte size match; inspected only.

### 3182. `third_party/naga/src/front/glsl/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 2b6c278d89d5d5cabfd604686d0543fbeb19a75d
- Size: 7745 bytes
- Coverage: 1–248; EOF.
- Purpose: Public GLSL frontend API, options, version/profile selection and parse orchestration.
- Dependencies / cross-references: GLSL lexer/parser/context/errors, Naga module/entry point and validation capabilities.
- Implementation / logic: Configures shader stage/version, parses source and returns Naga IR or frontend diagnostics.
- Findings: Supported GLSL versions/extensions are explicit and do not guarantee support for every Minecraft shader-pack dialect.
- RustChunk relevance: Useful frontend entry point if RustChunk needs GLSL shader import.
- Decision: adaptable; reference; GLSL-frontend-specific
- 1.20.1 implications: Pin a version compatible with shader packs used alongside Minecraft 1.20.1 and validate extension support.
- Follow-up checks: Parse a representative shader-pack corpus across vertex/fragment/compute stages and legacy versions.
- RustChunk implementation mapping: Optional Naga GLSL frontend in shader import pipeline.
- Evidence required before adoption: Corpus parse+validate tests and target backend compile tests.
- Completion state: COMPLETE — 248 logical lines through EOF; pinned SHA and 7,745-byte size match; not compiled.

### 3183. `third_party/naga/src/front/glsl/offset.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 1a2d909be1cd1db151d230bf51909a7cb0ba585d
- Size: 7004 bytes
- Coverage: 1–190; EOF.
- Purpose: Calculates GLSL std140/std430 memory layout, member offsets, alignment and array/matrix stride.
- Dependencies / cross-references: GLSL layout qualifiers, Naga TypeInner, scalar/vector/matrix/array/struct types.
- Implementation / logic: Recursively computes alignment/size and builds explicit member/array/matrix layout metadata.
- Findings: Row-major matrix layout and unsupported layout combinations require explicit rejection or validation; silent fallback would corrupt uniform/storage data.
- RustChunk relevance: Important for shader uniform/storage buffer compatibility across GLSL shader packs.
- Decision: adaptable; reference; buffer-layout-critical
- 1.20.1 implications: Use exact std140/std430 rules for 1.20.1 shader packs and keep uniform/storage layouts distinct.
- Follow-up checks: Test vec3 padding, nested structs, arrays, matrices, row-major qualifiers and overflow boundaries against driver reflection.
- RustChunk implementation mapping: RustChunk shader uniform/storage layout calculator if Naga is adopted.
- Evidence required before adoption: Compare calculated offsets/strides with GL/Vulkan reflection and known layout fixtures.
- Completion state: COMPLETE — 190 logical lines through EOF; pinned SHA and 7,004-byte size match; inspected only.

### 3184. `third_party/naga/src/front/glsl/parser.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 959a9b8e60d57203a2fc5d778d6b4682e604ac1e
- Size: 16222 bytes
- Coverage: 1–400, 401–446; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Implements the main GLSL parser for translation units, declarations and statements.
- Dependencies / cross-references: Lexer/token stream, AST, declarations/expressions/functions parsers, context and error types.
- Implementation / logic: Consumes tokens, manages parser state/scopes and dispatches grammar productions into AST/context lowering.
- Findings: Grammar support is intentionally bounded by Naga's GLSL profile; unsupported constructs need precise errors rather than silent acceptance.
- RustChunk relevance: Useful if RustChunk needs direct GLSL shader-pack import.
- Decision: adaptable; reference; parser
- 1.20.1 implications: Test against the GLSL dialects and extension directives actually used by 1.20.1 modpacks.
- Follow-up checks: Fuzz nested expressions/statements, malformed declarations, version directives and recovery after parse errors.
- RustChunk implementation mapping: Optional GLSL parser frontend.
- Evidence required before adoption: Parser corpus tests, fuzzing and source-span verification.
- Completion state: COMPLETE — 446 logical lines through EOF in 400/46 ranges; pinned SHA and 16,222-byte size match; not compiled.

### 3185. `third_party/naga/src/front/glsl/parser/declarations.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: e3cb321c94426838470a212e2102cbadebebc0cc
- Size: 26483 bytes
- Coverage: 1–400, 401–684; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Parses GLSL declarations, qualifiers, layout attributes, structs, interfaces and variables.
- Dependencies / cross-references: GLSL AST/parser, type constructors, storage/interpolation qualifiers and context symbol table.
- Implementation / logic: Builds type/variable declarations, applies layout/binding/access qualifiers and inserts declarations into the frontend context.
- Findings: Unsupported qualifier/layout combinations and extension-specific declarations must be rejected; layout defaults can affect GPU memory correctness.
- RustChunk relevance: Relevant to shader packs with UBO/SSBO/image declarations and modded resources.
- Decision: adaptable; reference; declaration-lowering
- 1.20.1 implications: Match GLSL 1.20.1 shader-pack dialects and ensure descriptor/binding semantics are preserved.
- Follow-up checks: Test std140/std430, binding/location, readonly/writeonly, interface blocks, structs and duplicate declarations.
- RustChunk implementation mapping: Optional GLSL declaration lowering layer.
- Evidence required before adoption: Parser fixtures and GL/Vulkan reflection comparison.
- Completion state: COMPLETE — 684 logical lines through EOF in 400/284 ranges; pinned SHA and 26,483-byte size match; not compiled.

### 3186. `third_party/naga/src/front/glsl/parser/expressions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 8fdb989d6d292c33ee19b974c2cc277e054368fd
- Size: 21027 bytes
- Coverage: 1–400, 401–568; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Parses GLSL expressions with precedence, constructors, assignments, ternaries, calls and access chains.
- Dependencies / cross-references: Lexer/parser, AST expression nodes, function overload resolution and context lowering.
- Implementation / logic: Uses precedence-aware parsing and produces AST expressions for unary/binary/conditional/index/member/swizzle/call operations.
- Findings: Operator semantics and associativity are critical; malformed/ambiguous expressions must not be accepted with altered precedence.
- RustChunk relevance: Relevant for importing real GLSL shader packs.
- Decision: adaptable; reference; expression-semantics-sensitive
- 1.20.1 implications: Verify GLSL operator precedence and assignment behavior against 1.20.1 pack shaders.
- Follow-up checks: Differentially test precedence, short-circuit, ternary, comma, increment/decrement and compound assignment expressions.
- RustChunk implementation mapping: Optional GLSL expression parser.
- Evidence required before adoption: Parser fixtures plus Naga validation and shader output parity.
- Completion state: COMPLETE — 568 logical lines through EOF in 400/168 ranges; pinned SHA and 21,027-byte size match; not compiled.

### 3187. `third_party/naga/src/front/glsl/parser/functions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 441e99130a0ba9e4a0ac0b86e6dd5cf40f55a5d7
- Size: 25710 bytes
- Coverage: 1–400, 401–661; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Parses GLSL function prototypes/definitions, compound statements, loops, branches and switch cases.
- Dependencies / cross-references: GLSL parser/AST, context scopes, function overload resolution and Naga control-flow IR.
- Implementation / logic: Builds function bodies and nested control flow, tracks loop/switch terminators and parses parameter qualifiers.
- Findings: Control-flow parsing must preserve break/continue/return and switch fallthrough semantics; unsupported fallthrough paths can limit backend output.
- RustChunk relevance: Relevant for shader-pack function and control-flow compatibility.
- Decision: adaptable; reference; function-control-flow
- 1.20.1 implications: Test GLSL control flow from the selected 1.20.1 shader packs; do not assume every valid GLSL switch maps to all backends.
- Follow-up checks: Test nested loops/switches, break/continue, early return, do/while and out/inout parameters.
- RustChunk implementation mapping: Optional GLSL function/control-flow parser.
- Evidence required before adoption: Differential parse/validate tests and backend output comparison.
- Completion state: COMPLETE — 661 logical lines through EOF in 400/261 ranges; pinned SHA and 25,710-byte size match; not compiled.

### 3188. `third_party/naga/src/front/glsl/parser/types.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 1499845a8620f9e494789cd9b37342eb36d4e2c0
- Size: 17163 bytes
- Coverage: 1–400, 401–456; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Parses GLSL type names, scalar/vector/matrix/image/sampler and array type constructors.
- Dependencies / cross-references: GLSL token/type grammar, Naga TypeInner, image classes and type arena.
- Implementation / logic: Maps built-in GLSL type spellings and dimensions into Naga types, including extension-specific numeric types.
- Findings: Image format and extension types require qualifier/profile context; placeholder defaults must be finalized by declarations.
- RustChunk relevance: Relevant to GLSL shader-pack type import.
- Decision: adaptable; reference; GLSL-type-compatibility
- 1.20.1 implications: Use only with exact supported GLSL version and image/storage qualifier handling.
- Follow-up checks: Test every scalar/vector/matrix/image/sampler spelling and invalid dimension/format combinations.
- RustChunk implementation mapping: Optional GLSL type parser.
- Evidence required before adoption: Exhaustive type-name fixtures and Naga validator tests.
- Completion state: COMPLETE — 456 logical lines through EOF in 400/56 ranges; pinned SHA and 17,163-byte size match; not compiled.

### 3189. `third_party/naga/src/front/glsl/parser_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 6fdd1e39715df030b32db43599e7f8d1feaa52a9
- Size: 18556 bytes
- Coverage: 1–400, 401–800, 801–862; consecutive; EOF; 1201–1600 confirmed empty.
- Purpose: Contains GLSL parser/frontend regression fixtures and unit tests.
- Dependencies / cross-references: GLSL lexer/parser/context, expected Naga IR and diagnostic assertions.
- Implementation / logic: Exercises declarations, expressions, types, builtins, control flow and parser error cases.
- Findings: Tests are evidence of intended behavior only; they were inspected but not executed in this audit.
- RustChunk relevance: Useful as a future regression-test source when integrating Naga's GLSL frontend.
- Decision: adaptable; reference; test-suite
- 1.20.1 implications: Run the upstream test suite only after selecting the exact Naga revision/toolchain; add Minecraft shader-pack fixtures.
- Follow-up checks: Execute all parser tests plus real shader-pack corpus on the chosen host/Android build.
- RustChunk implementation mapping: RustChunk GLSL frontend regression suite.
- Evidence required before adoption: Passing CI tests and shader-pack parse/compile fixtures.
- Completion state: COMPLETE — 862 logical lines through EOF in two 400-line chunks plus 62 lines; pinned SHA and 18,556-byte size match; tests not run.

### 3190. `third_party/naga/src/front/glsl/token.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 78c23c56c29a0cb170755b967dca1043495aac68
- Size: 2379 bytes
- Coverage: 1–139; EOF.
- Purpose: Defines GLSL token kinds and token value/span structures.
- Dependencies / cross-references: GLSL lexer and parser.
- Implementation / logic: Enumerates identifiers, numeric literals, operators, punctuation and keyword tokens.
- Findings: Token enum completeness must match lexer/parser version and extension grammar.
- RustChunk relevance: Useful only as part of a pinned GLSL frontend.
- Decision: adaptable; reference; lexer-token-model
- 1.20.1 implications: Keep token kinds synchronized with supported GLSL versions and extensions.
- Follow-up checks: Exhaustive tokenization tests and unreachable token match checks.
- RustChunk implementation mapping: Optional GLSL lexer/parser token model.
- Evidence required before adoption: Compile-time exhaustive matches and lexer corpus tests.
- Completion state: COMPLETE — 139 logical lines through EOF; pinned SHA and 2,379-byte size match; inspected only.

### 3191. `third_party/naga/src/front/glsl/types.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: dffdf96e6ddf047d6067b55de3418598e5c3fa15
- Size: 12845 bytes
- Coverage: 1–372; EOF.
- Purpose: Defines GLSL frontend type descriptors for images, samplers and source-level types.
- Dependencies / cross-references: GLSL parser/type constructors, Naga scalar/vector/matrix/image types and storage formats.
- Implementation / logic: Stores source-language type information and maps it to Naga TypeInner representations.
- Findings: Source-level GLSL distinctions may be lost during conversion; preserve format/access/profile metadata until declarations are resolved.
- RustChunk relevance: Relevant to shader-pack import and resource reflection.
- Decision: adaptable; reference; GLSL-type-model
- 1.20.1 implications: Keep source GLSL type qualifiers available until Naga validation and backend binding generation.
- Follow-up checks: Test image dimensionality, multisample/array/shadow flags, storage format and scalar width.
- RustChunk implementation mapping: Optional GLSL source type model.
- Evidence required before adoption: Type mapping fixtures and backend reflection checks.
- Completion state: COMPLETE — 372 logical lines through EOF; pinned SHA and 12,845-byte size match; inspected only.

### 3192. `third_party/naga/src/front/glsl/variables.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 143af99dda55245c6967fce6aaaf89dec108a926
- Size: 27368 bytes
- Coverage: 1–400, 401–718; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Lowers GLSL global/local variables, storage classes, initializers, interface locations and resource bindings.
- Dependencies / cross-references: GLSL context/type/qualifier data, Naga global/local variables and binding metadata.
- Implementation / logic: Creates global/local variables, maps storage/interpolation/access qualifiers and handles initializers/entry-point IO.
- Findings: Binding/location defaults and storage access must match GLSL and target API; silent fallback risks resource mismatch.
- RustChunk relevance: Important for shader-pack resource and stage-interface compatibility.
- Decision: adaptable; reference; variable-lowering
- 1.20.1 implications: Validate UBO/SSBO/image/sampler bindings against the selected 1.20.1 shader pack and target GLES/Vulkan API.
- Follow-up checks: Test duplicate bindings, explicit locations, interface blocks, readonly/writeonly and initializer restrictions.
- RustChunk implementation mapping: Optional GLSL variable/resource lowering.
- Evidence required before adoption: GL/Vulkan reflection comparison and parser/validator tests.
- Completion state: COMPLETE — 718 logical lines through EOF in 400/318 ranges; pinned SHA and 27,368-byte size match; not compiled.

### 3193. `third_party/naga/src/front/interpolator.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 3fce4339bd409829b7d26690b0d5df5096b33931
- Size: 2299 bytes
- Coverage: 1–54; EOF.
- Purpose: Maps interpolation and sampling qualifiers to Naga interpolation metadata.
- Dependencies / cross-references: GLSL/WGSL frontends, interpolation/sampling enums and stage IO.
- Implementation / logic: Converts flat/linear/perspective and centroid/sample qualifiers into Naga interpolation/sampling values.
- Findings: Cross-language interpolation defaults differ; callers must supply correct source-language defaults.
- RustChunk relevance: Useful for preserving shader stage IO semantics.
- Decision: adaptable; reference; interpolation
- 1.20.1 implications: Check vertex/fragment interface interpolation for Minecraft shader packs.
- Follow-up checks: Test flat/noperspective/smooth, centroid/sample and integer varying defaults.
- RustChunk implementation mapping: RustChunk shader stage-interface mapping.
- Evidence required before adoption: Shader interface validation and GPU render tests.
- Completion state: COMPLETE — 54 logical lines through EOF; pinned SHA and 2,299-byte size match; inspected only.

### 3194. `third_party/naga/src/front/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: ee59429ed359ca7fc3c29c5356ee70cfc879dbf8
- Size: 13832 bytes
- Coverage: 1–400; EOF.
- Purpose: Defines shared frontend context, typifier, symbol table and type-resolution utilities.
- Dependencies / cross-references: Naga IR arenas, TypeResolution, GlobalCtx and frontend-specific parsers.
- Implementation / logic: Provides scope-aware symbol lookup, expression type resolution and invalidation helpers shared by GLSL/SPIR-V/WGSL frontends.
- Findings: Typifier invalidation is caller-sensitive after IR mutation; downstream dependent expressions must be recomputed when source types change.
- RustChunk relevance: Useful shared frontend architecture if RustChunk adopts Naga parsing/IR.
- Decision: adaptable; reference; frontend-core
- 1.20.1 implications: Keep the exact Naga validation/typifier lifecycle and invalidate dependent expressions after mutation.
- Follow-up checks: Test nested scope shadowing, symbol reuse, expression invalidation and type resolution after IR edits.
- RustChunk implementation mapping: RustChunk shader frontend context and symbol table if Naga is selected.
- Evidence required before adoption: Mutation/typifier consistency tests and parser integration tests.
- Completion state: COMPLETE — 400 logical lines through EOF; pinned SHA and 13,832-byte size match; inspected only.

### 3195. `third_party/naga/src/front/spv/convert.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: f44029320da0c37f405ae6d478aedf6ff8ff196b
- Size: 9753 bytes
- Coverage: 1–189; EOF.
- Purpose: Maps SPIR-V opcode/type enums to Naga IR operators and scalar/type metadata.
- Dependencies / cross-references: SPIR-V opcodes, Naga BinaryOperator/UnaryOperator/ScalarKind and conversion errors.
- Implementation / logic: Converts arithmetic, comparison, logical and scalar type instructions into Naga equivalents.
- Findings: `FOrd*` and `FUnord*` comparison opcodes appear to map to the same Naga comparison operator; unordered comparisons have different NaN truth behavior, so this may lose SPIR-V semantics. Several remainder/modulo opcodes also need explicit coverage review.
- RustChunk relevance: Critical if RustChunk imports SPIR-V shaders; incorrect comparison semantics can cause shader output differences.
- Decision: rewrite required; reference; SPIR-V-semantic-correctness
- 1.20.1 implications: Do not rely on this conversion unchanged for shader code that uses unordered float comparisons or remainder variants.
- Follow-up checks: Add NaN fixtures for every ordered/unordered comparison and signed/unsigned remainder/modulo opcode; compare against SPIR-V reference execution.
- RustChunk implementation mapping: SPIR-V frontend opcode conversion after semantic fixes.
- Evidence required before adoption: Differential SPIR-V execution tests and Naga IR validator tests.
- Completion state: COMPLETE — 189 logical lines through EOF; pinned SHA and 9,753-byte size match; not compiled.

### 3196. `third_party/naga/src/front/spv/error.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: c0c525d363285642db8683638b02ace0a3e7a91b
- Size: 7496 bytes
- Coverage: 1–198; EOF.
- Purpose: Defines SPIR-V frontend parse/translation error variants.
- Dependencies / cross-references: SPIR-V instruction parser, ID/type/constant lookup and source spans.
- Implementation / logic: Represents malformed modules, invalid operands, unsupported instructions/capabilities and type conversion failures.
- Findings: Some errors depend on validation being enabled; distinguish parser rejection from post-parse Naga validation.
- RustChunk relevance: Useful for robust SPIR-V shader import diagnostics.
- Decision: adaptable; reference; SPIR-V-diagnostics
- 1.20.1 implications: Keep malformed SPIR-V errors separate from Vulkan runtime/pipeline failures.
- Follow-up checks: Test invalid IDs, operand counts, unsupported opcodes/capabilities and byte-offset reporting.
- RustChunk implementation mapping: RustChunk SPIR-V import error model.
- Evidence required before adoption: Malformed-module corpus and source offset tests.
- Completion state: COMPLETE — 198 logical lines through EOF; pinned SHA and 7,496-byte size match; inspected only.

### 3197. `third_party/naga/src/front/spv/function.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: df96dc460d9f2493ee06a40d243c174fe16971c9
- Size: 30523 bytes
- Coverage: 1–400, 401–800; consecutive; EOF; 801–1200 confirmed empty.
- Purpose: Parses SPIR-V functions, entry points, function variables and function-level metadata into Naga IR.
- Dependencies / cross-references: SPIR-V module/type/constant lookup, Naga Function/EntryPoint, BlockContext and atomic upgrade tracking.
- Implementation / logic: Creates Naga functions, maps arguments/results/local variables, translates entry-point interfaces and normalizes coordinate/output behavior.
- Findings: Function and entry-point lowering relies on valid ID/type references and later Naga validation; no execution evidence from static review.
- RustChunk relevance: Relevant if RustChunk uses SPIR-V as shader input or Vulkan pipeline source.
- Decision: adaptable; reference; SPIR-V-function-lowering
- 1.20.1 implications: Pin the SPIR-V frontend version and validate every imported module before backend generation.
- Follow-up checks: Test entry-point IO, function calls, local variables, return values, builtins and atomic pointer upgrades.
- RustChunk implementation mapping: Optional SPIR-V frontend function lowering.
- Evidence required before adoption: SPIR-V Tools validation, Naga validation and Vulkan pipeline tests.
- Completion state: COMPLETE — 800 logical lines through EOF in two consecutive 400-line chunks; pinned SHA and 30,523-byte size match; not compiled.

### 3198. `third_party/naga/src/front/spv/image.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 591590be024b088f20f1b9320ecc7f33e2c6e5b8
- Size: 35363 bytes
- Coverage: 1–400, 401–800, 801–932; consecutive; EOF; 1201–1600 confirmed empty.
- Purpose: Translates SPIR-V image types, sampled images, image operations and image operands into Naga IR.
- Dependencies / cross-references: SPIR-V image opcodes, image type/format metadata, Naga ImageClass/StorageFormat and expression arena.
- Implementation / logic: Resolves dimensions, array/multisample/depth/storage formats, sampling/load/store/query and image operand masks.
- Findings: SPIR-V image operands/extensions and format capabilities are version-specific; unsupported operand combinations must return explicit errors.
- RustChunk relevance: Important for importing Minecraft shader modules with textures, storage images and sampler operations.
- Decision: adaptable; reference; SPIR-V-image-frontend
- 1.20.1 implications: Validate storage/image format and sampling semantics against the exact Vulkan target and shader pack.
- Follow-up checks: Test all dimensions, depth, multisample, array, gather, offsets, LOD, storage formats and image operand masks.
- RustChunk implementation mapping: Optional SPIR-V image frontend.
- Evidence required before adoption: SPIR-V Tools validation, Naga validation and GPU image read/write tests.
- Completion state: COMPLETE — 932 logical lines through EOF in two 400-line chunks plus 132 lines; pinned SHA and 35,363-byte size match; not compiled.

### 3199. `third_party/naga/src/front/spv/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: cd3e07b05ae3141e242b19537bdbbed9c0101721
- Size: 119189 bytes
- Coverage: 1–400 through 2801–3200 in eight consecutive 400-line chunks, then 3201–3275; EOF; 3601–4000 confirmed empty.
- Purpose: Top-level SPIR-V module parser and Naga frontend orchestration.
- Dependencies / cross-references: SPIR-V grammar/instructions, types/constants/decorations/capabilities, function/image/next_block modules, Naga IR and validation.
- Implementation / logic: Reads SPIR-V words, resolves IDs, constructs module types/constants/globals/entry points/functions and dispatches function block parsing.
- Findings: Capability and memory model handling are version-sensitive; parser acceptance is not proof that a Vulkan device supports emitted capabilities or semantics.
- RustChunk relevance: Major candidate for importing SPIR-V and building a Vulkan shader path on Android.
- Decision: adaptable; reference; SPIR-V-frontend; validation-critical
- 1.20.1 implications: Require strict capability validation and SPIR-V Tools validation; query Vulkan device extensions before pipeline creation.
- Follow-up checks: Test SPIR-V 1.0–1.6 modules, malformed IDs, capabilities, decorations, specialization constants, mesh/ray/subgroup and memory model cases.
- RustChunk implementation mapping: Optional Naga SPIR-V frontend in a Vulkan renderer pipeline.
- Evidence required before adoption: SPIR-V Tools validation, Naga validation and real Mali Vulkan pipeline creation.
- Completion state: COMPLETE — 3,275 logical lines through EOF in eight 400-line chunks plus 75 lines; pinned SHA and 119,189-byte size match; not compiled.

### 3200. `third_party/naga/src/front/spv/next_block.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 698ed37e89499a2ae7c34816ac815e84c246e7b1
- Size: 143572 bytes
- Coverage: 1–400 through 2401–2800 in seven consecutive 400-line chunks, then 2801–3129; EOF; 3201–3600 confirmed empty.
- Purpose: Parses SPIR-V basic blocks, structured control flow, barriers, subgroup and atomic instructions into Naga statements.
- Dependencies / cross-references: SPIR-V opcode stream, block/body maps, merge/loop metadata, expression emitter, Naga Statement/Expression and atomic upgrade.
- Implementation / logic: Builds nested if/switch/loop bodies, handles merge/continue labels, emits loads/stores/barriers/subgroup operations and atomic statements.
- Findings: `Op::CopyMemory` discards memory-access flags and lowers to plain load/store; atomic scope/memory-semantics operands are consumed but not represented; `ControlBarrier` ignores memory scope and unsupported execution scopes are only warned/dropped. These can lose synchronization/volatile semantics and require explicit validation.
- RustChunk relevance: Critical if RustChunk imports SPIR-V shaders or executes Vulkan compute workloads; synchronization correctness affects parallel rendering/compute.
- Decision: rewrite required; reference; SPIR-V-synchronization-semantics
- 1.20.1 implications: Do not use unchanged for concurrent GPU workloads until memory scopes, semantics and CopyMemory flags are preserved or proven safely representable by Naga IR.
- Follow-up checks: Create SPIR-V fixtures for volatile/copy memory, acquire/release, device/workgroup/subgroup scopes, atomic ordering and unsupported barrier scopes; compare execution.
- RustChunk implementation mapping: SPIR-V block parser after synchronization semantic review/fixes.
- Evidence required before adoption: SPIR-V Tools validation and Vulkan GPU concurrency/readback tests.
- Completion state: COMPLETE — 3,129 logical lines through EOF in seven 400-line chunks plus 329 lines; pinned SHA and 143,572-byte size match; not compiled.

## Checkpoint 3176–3200 — COMPLETE

- 25 source files inspected one at a time from the pinned Starling snapshot after excluding all 3,175 prior Index paths.
- All 25 source blob SHAs and byte sizes match the complete pinned tree (5,749 entries; untruncated). Canonical logical line counts and complete EOF coverage are recorded.
- Long files were read in consecutive, gap-free ranges no larger than 400 lines: 3177 (2,913), 3178 (1,600), 3180 (1,713), 3185 (684), 3186 (568), 3187 (661), 3189 (862), 3192 (718), 3197 (800), 3198 (932), 3199 (3,275) and 3200 (3,129).
- Every new entry has exactly 15 required fields. Major static concerns: GLSL image-atomic return value is a non-atomic pre-load; SPIR-V ordered/unordered float comparisons appear conflated; SPIR-V block lowering drops memory-access/synchronization operands. These are not reproduced by tests.
- No tests/builds/benchmarks/APK/device/Vulkan/Forge/Minecraft/server/Aternos validation was run. No RustChunk application source was changed.
