# RustChunk Starling Audit Ledger — Entries 3001–3200

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active 200-file stage: **3001–3200 — IN PROGRESS (25/200)**.
- One-time exceptional authorization **3001–3100 — ACTIVE (25/100 consumed)**; it covers sections 3001–3050 and 3051–3100, with 25-file commits and 50-file reports.
- Checkpoint **3001–3025 is complete**. Checkpoint 3026–3050 remains authorized under the same grant. Stop at 3100; entries 3101+ require fresh explicit authorization.
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
