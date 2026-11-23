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
