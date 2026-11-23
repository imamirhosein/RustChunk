# RustChunk Starling Audit Ledger — Entries 2801–3000

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active 200-file stage: **2801–3000 — CLOSED (200/200)**.
- Exceptional authorization 2801–2900: **EXHAUSTED**. New exceptional authorization 2901–3000: **EXHAUSTED**.
- All eight 25-file checkpoints 2801–3000 are complete and committed; all four 50-file sections (2801–2850, 2851–2900, 2901–2950, 2951–3000) are closed.
- Final Ledger: **200 records (2801–3000)**, each with exactly 15 required fields. Index: **3,000 sequential unique paths** with path/Decision alignment. All 200 source SHAs and byte sizes match the pinned tree.
- Source selection: eligible source/documentation blobs in strict raw lexical order from the complete pinned recursive tree after excluding every previously indexed path. The tree is untruncated; every source was inspected through EOF.
- Starling/Horizon documentation and implementation target Minecraft Java 26.3; this audit is compatibility research, not evidence of Minecraft 1.20.1 / Forge 47.x support.
- No application source was modified. Tests/docs were inspected only; no tests, builds, benchmarks, APK, device, Forge, Minecraft or server validation was run.
- **STOP at 3000**. Entry 3001+ requires fresh explicit user authorization.


## Checkpoint 2801–2825 — COMPLETE

### 2801. `docs/README.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3febb38d83b939b3a8df56eaa80a79e312d45230`
- Size: 8240 bytes
- Coverage: Complete through EOF: 65 logical lines; full file; no gaps.
- Purpose: Developer documentation landing page linking build, launcher, server, parity, worldgen, rendering and API guides.
- Dependencies / cross-references: Cross-links README, AGENTS, architecture, launcher, worldgen, parity and API documents.
- Implementation / logic: Organizes Horizon developer docs by build/run, parity, world generation, simulation and profiling.
- Findings: No executable implementation. Its navigation and feature inventory describe a 26.3 desktop/server product.
- RustChunk relevance: Useful only as a documentation taxonomy and subsystem map.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Rebuild links and status around RustChunk Android, Rust client, Forge 1.20.1 and its actual milestones; do not copy 26.3 claims.
- Follow-up checks: Cross-check every linked target and map documented subsystems against RustChunk scope before using as project navigation.
- RustChunk implementation mapping: RustChunk docs index: architecture, Android integration, protocol, renderer, Forge bridge and test strategy.
- Evidence required before adoption: Current RustChunk tree, target 1.20.1 source map and verified implementation status for every linked feature.
- Completion state: COMPLETE — 65 logical lines through EOF; pinned source SHA and 8240 byte size match; inspected, not modified; tests not executed.

### 2802. `docs/agent-control.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fd400092553794c01b88d6b0dac9e0563ccb8f08`
- Size: 27268 bytes
- Coverage: Complete through EOF: 386 logical lines; full file; no gaps.
- Purpose: Design and inventory of launcher/game agent control surfaces, CLI, MCP, socket protocol, event tracing and repro recordings.
- Dependencies / cross-references: References horizon-ctl command table, launcher CLI/MCP, control socket, no-AI switch, input trace, recording/replay and tests.
- Implementation / logic: Documents local line-oriented JSON socket, CLI/MCP wrappers, protocol revisions, widget tree, destructive/network safeguards and input provenance.
- Findings: Local socket access grants broad control of the game; security relies on local-only binding/token and explicit user choice. Recording copies worlds and logs, albeit with scrub rules.
- RustChunk relevance: Control architecture is a useful optional developer/automation concept, not required for initial Android game operation.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Re-design for Android lifecycle, scoped permissions, accessibility/input APIs and RustChunk command set; do not transplant GPUI or Horizon-specific commands.
- Follow-up checks: Review socket authentication, command authorization, user-input interruption, redaction, world-copy limits and Android process death.
- RustChunk implementation mapping: Optional RustChunk diagnostic/control service and test harness; keep disabled unless explicitly enabled by the player.
- Evidence required before adoption: Threat model, Android sandbox behavior, local socket permissions, consent UX and end-to-end tests including malicious local clients.
- Completion state: COMPLETE — 386 logical lines through EOF; pinned source SHA and 27268 byte size match; inspected, not modified; tests not executed.

### 2803. `docs/animated-sprite-mips-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `60aed97db525f64638e5a8e14168e8619f7e3ce0`
- Size: 8412 bytes
- Coverage: Complete through EOF: 148 logical lines; full file; no gaps.
- Purpose: Performance study of caching generated mip chains for animated sprite frames.
- Dependencies / cross-references: References renderer gfx.rs, animation/mipmap loaders, resource reload snapshots and benchmark driver.
- Implementation / logic: Describes lazy bounded cache (8 MiB pixel capacity, 4096 metadata entries), fallback for non-admitted frames and paired microbenchmarks.
- Findings: Reported speedups apply to warmed CPU generation/reuse, not GPU upload, GPU memory or end-to-end FPS; test suite/full game parity were not all run.
- RustChunk relevance: Potential renderer optimization hypothesis; Android memory/cache pressure differs substantially.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Only consider after RustChunk renderer has equivalent animated texture/mipmap path; remeasure on Mali-G52/Android and constrain memory more tightly.
- Follow-up checks: Check cache invalidation, frame identity, resource reload, map overhead, allocation behavior and pixel-perfect upload stream parity.
- RustChunk implementation mapping: Future texture animation cache in RustChunk renderer, behind measured device-specific admission limits.
- Evidence required before adoption: Actual RustChunk renderer code, Android ARM64 benchmarks, memory telemetry, pixel/upload parity fixtures and cache-pressure tests.
- Completion state: COMPLETE — 148 logical lines through EOF; pinned source SHA and 8412 byte size match; inspected, not modified; tests not executed.

### 2804. `docs/anticheat.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `af35c1cf0b69126125323d6d5b80632a44b5f715`
- Size: 11749 bytes
- Coverage: Complete through EOF: 116 logical lines; full file; no gaps.
- Purpose: Explains Horizon dedicated-server movement, timing, ground and combat anti-cheat checks.
- Dependencies / cross-references: Depends on horizon-anticheat crate, LAN packet hub, server.toml thresholds, vanilla movement replay and server E2E bot harness.
- Implementation / logic: Defines violation levels/decay, thresholds for setback/alert/kick/ban, timer accounting, ground-claim filtering, movement prediction replay and attack ordering.
- Findings: Server-authoritative policy is sensitive to false positives; prediction excuses lag/unknown state and default actions vary by check. Horizon's server-side code is separate from client anti-cheat.
- RustChunk relevance: Not a client performance feature; server-side design may inform compatibility diagnostics only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Do not implement as client anti-cheat. RustChunk must remain protocol-correct and avoid making unauthorized movement claims; server enforcement belongs to server operators.
- Follow-up checks: Validate vanilla 1.20.1 packet/movement semantics only if a separate server project is ever scoped; examine Forge-mod movement and latency cases.
- RustChunk implementation mapping: No direct game-client mapping; at most a future server diagnostics research note.
- Evidence required before adoption: Separate explicit server-side scope, target-version packet oracle, false-positive dataset, consent and security review.
- Completion state: COMPLETE — 116 logical lines through EOF; pinned source SHA and 11749 byte size match; inspected, not modified; tests not executed.

### 2805. `docs/api/README.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ddbf2c72fa80d44feb948b966bec07e15c9dcd81`
- Size: 4898 bytes
- Coverage: Complete through EOF: 58 logical lines; full file; no gaps.
- Purpose: Overview of Horizon's seven public API surfaces, design principles and generated-reference workflow.
- Dependencies / cross-references: Links API pages for mods, shaders, agent control, servers, launcher, launch and data formats; generated docs derive from Rust source/tests.
- Implementation / logic: Classifies surfaces by audience, stability and reference; describes versioned contracts, safe defaults and generated documentation checks.
- Findings: All API contracts are Horizon-specific; API inventory includes experimental and preview surfaces, not general Minecraft/Forge standards.
- RustChunk relevance: Useful as a model for documenting stable internal interfaces, but not a reusable API implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Define RustChunk-specific APIs and stability only after actual contracts exist; distinguish Minecraft protocol/version data from RustChunk's own interfaces.
- Follow-up checks: Check generated docs are tied to source and tests; ensure API compatibility does not imply Forge compatibility.
- RustChunk implementation mapping: RustChunk architecture documentation and future internal API catalogue.
- Evidence required before adoption: RustChunk source interfaces, release policy, generated-reference tests and documented compatibility guarantees.
- Completion state: COMPLETE — 58 logical lines through EOF; pinned source SHA and 4898 byte size match; inspected, not modified; tests not executed.

### 2806. `docs/api/agent-control.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b34b123c42ecdef1360fdae00dfe8d4a48741c8c`
- Size: 3134 bytes
- Coverage: Complete through EOF: 57 logical lines; full file; no gaps.
- Purpose: Public API guide for driving a running Horizon game through socket, CLI and MCP.
- Dependencies / cross-references: References control socket, horizon-ctl commands, MCP server, generated reference and E2E scripts.
- Implementation / logic: Defines command counts, transport, stability level, generated command reference and missing compatibility/schema features.
- Findings: Describes a preview API with no complete reply schemas or protocol-version refusal yet.
- RustChunk relevance: Could guide optional automation API documentation, not the Android game core.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Any RustChunk control interface needs a smaller Android-aware contract, version negotiation and explicit capability scopes.
- Follow-up checks: Verify reply schemas, command version checks, socket access and transcript compatibility before adoption.
- RustChunk implementation mapping: Optional RustChunk CLI/diagnostic API spec.
- Evidence required before adoption: Implemented Android control transport, strict schemas, compatibility transcript tests and security review.
- Completion state: COMPLETE — 57 logical lines through EOF; pinned source SHA and 3134 byte size match; inspected, not modified; tests not executed.

### 2807. `docs/api/data-formats.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1adf0d40d78dfb1469e0da0509a0adafbb857e7d`
- Size: 3218 bytes
- Coverage: Complete through EOF: 42 logical lines; full file; no gaps.
- Purpose: Catalogues versioned and unversioned files written by Horizon, including mods, settings, telemetry and world folders.
- Dependencies / cross-references: Cross-references benchmark report, telemetry, content/native mod manifests, server settings, control endpoints and release signatures.
- Implementation / logic: Specifies schema/version fields, forward-reader rules and sample-file tests.
- Findings: Several settings/control formats are not versioned and sample coverage is incomplete; Horizon-specific formats are not Forge formats.
- RustChunk relevance: Versioning and robust-reader principles transfer to RustChunk metadata and caches.
- Decision: adaptable; reference
- 1.20.1 implications: Use explicit schema versions for RustChunk-owned manifests/cache metadata; leave vanilla world/save formats governed by 1.20.1.
- Follow-up checks: Test unknown fields, newer schema refusal, optional additions, backward compatibility and malformed data.
- RustChunk implementation mapping: RustChunk profile metadata, download manifests, renderer cache metadata and diagnostic reports.
- Evidence required before adoption: Concrete RustChunk schemas, migration policy, golden fixtures and reader tests.
- Completion state: COMPLETE — 42 logical lines through EOF; pinned source SHA and 3218 byte size match; inspected, not modified; tests not executed.

### 2808. `docs/api/launch-guide.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a1e5aae032786ef114a6625be2cf18d0952851cb`
- Size: 6538 bytes
- Coverage: Complete through EOF: 123 logical lines; full file; no gaps.
- Purpose: Step-by-step guide for third-party launchers to run Horizon and prepare its resources.
- Dependencies / cross-references: Depends on horizon-client/horizon-prepare launch APIs, research-lock pins, Java runtime, game files, session JSON and release archives.
- Implementation / logic: Describes owner gate, artifact validation, preparation/check phases, launch arguments, secure session transfer and exit codes.
- Findings: Explicit Microsoft ownership gate and no offline profile path; targets Minecraft 26.3 and Java 25. The process assumes desktop archives and external preparer.
- RustChunk relevance: Resource provenance, prepare/check separation and safe secret transfer are useful launcher concepts.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Rewrite for Android storage/runtime, RustChunk's 1.20.1 assets/libraries, Forge 47.x installation and permitted offline-profile behavior; never assume 26.3 resources.
- Follow-up checks: Verify license/compliance boundaries, checksums, Java selection, scoped storage, resumable downloads and startup error reporting.
- RustChunk implementation mapping: RustChunk resource preparation and instance-launch workflow.
- Evidence required before adoption: 1.20.1 version manifest, Forge installer outputs, Android filesystem rules, Java 17 runtime requirements and account-mode design.
- Completion state: COMPLETE — 123 logical lines through EOF; pinned source SHA and 6538 byte size match; inspected, not modified; tests not executed.

### 2809. `docs/api/launch-reference.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f8af03877aae23f933d8203c6cf630cfe43bac32`
- Size: 4002 bytes
- Coverage: Complete through EOF: 103 logical lines; full file; no gaps.
- Purpose: Generated reference for Horizon's launch command options, session JSON and exit codes.
- Dependencies / cross-references: Generated from crates/horizon-client/src/game/launch_api.rs; linked launch guide and launch API tests.
- Implementation / logic: Lists resources/game/home/player directories, window, session, quick-play options, launch-api JSON and exit codes.
- Findings: Contract is experimental and declares Minecraft 26.3; account session requires online token fields.
- RustChunk relevance: Useful as a model for a machine-readable launcher/game contract.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Design RustChunk-specific launch contract for Android and 1.20.1; do not copy option names or session schema without need.
- Follow-up checks: Test unknown options, path isolation, session redaction/deletion, error codes and version negotiation.
- RustChunk implementation mapping: RustChunk launcher-to-client process contract and launch diagnostics.
- Evidence required before adoption: RustChunk executable arguments, Android process model, offline/online account semantics and integration tests.
- Completion state: COMPLETE — 103 logical lines through EOF; pinned source SHA and 4002 byte size match; inspected, not modified; tests not executed.

### 2810. `docs/api/launch.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `610cda7c7f09c7003eff5b9ce286d7672be26448`
- Size: 7076 bytes
- Coverage: Complete through EOF: 125 logical lines; full file; no gaps.
- Purpose: Describes Horizon Launch API 0.1, resource preparation, session JSON and lifecycle.
- Dependencies / cross-references: References horizon-client, horizon-prepare, launch-guide, launch-reference, research-lock and data-format policy.
- Implementation / logic: Separates resource preparation/checking from client startup; specifies JSON progress, secure session handoff and process exit semantics.
- Findings: Experimental 26.3-only API; requires Microsoft-owned Java Edition account and does not download Mojang files.
- RustChunk relevance: Architectural separation of preparation and launch is adaptable; its account/resource assumptions are not.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Rebuild the API around Android installation, 1.20.1 version assets, Forge profile, offline profile support and Java 17.
- Follow-up checks: Check artifact hashes, preparer idempotence, interrupted writes, progress monotonicity, process exit and session lifetime.
- RustChunk implementation mapping: RustChunk resource-preparer/client-launch boundary.
- Evidence required before adoption: Verified 1.20.1 artifact set, Forge 47.x installation process, Android storage behavior and launch lifecycle tests.
- Completion state: COMPLETE — 125 logical lines through EOF; pinned source SHA and 7076 byte size match; inspected, not modified; tests not executed.

### 2811. `docs/api/launcher.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b100d97275f1ad7ee314a69b940cc85ea4d036e`
- Size: 2758 bytes
- Coverage: Complete through EOF: 57 logical lines; full file; no gaps.
- Purpose: Documents Horizon launcher CLI and MCP commands for installs, accounts, mods, modpacks and servers.
- Dependencies / cross-references: Generated command reference from launcher/src/cli.rs; links agent-control and launcher implementation.
- Implementation / logic: Defines command/tool inventory, confirm semantics, account/modpack workflows and JSON behavior.
- Findings: GPUI desktop launcher and Horizon mod formats are not Android UI or Forge mod management.
- RustChunk relevance: Command taxonomy and explicit confirmation patterns can inform RustChunk settings/diagnostics.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Do not port GPUI CLI or Horizon package manager wholesale; build Android-first profile and Forge management surfaces.
- Follow-up checks: Separate read-only from destructive operations; verify confirmation, structured errors and account privacy.
- RustChunk implementation mapping: Optional RustChunk command/diagnostic surface and UI action model.
- Evidence required before adoption: Android UI flow, RustChunk profile schema, Forge mod file semantics and integration tests.
- Completion state: COMPLETE — 57 logical lines through EOF; pinned source SHA and 2758 byte size match; inspected, not modified; tests not executed.

### 2812. `docs/api/mod-files.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2fd029a5f956cb6fb593e394c08c25fc01f4a2e2`
- Size: 3449 bytes
- Coverage: Complete through EOF: 70 logical lines; full file; no gaps.
- Purpose: Defines Horizon .hmod/.hnmod ZIP package paths, manifests, content/native code and size limits.
- Dependencies / cross-references: Depends on horizon_mods::Package/Content, native Bundle, horizon-native.json and horizon-mixin.toml.
- Implementation / logic: Specifies archive path safety, entry uniqueness, allowed top-level entries, manifest schema and per-file/aggregate caps.
- Findings: These are Horizon-native/content package formats, not Java Forge mod JARs; native bundles compile Rust code with player privileges.
- RustChunk relevance: Archive validation and strict path/size constraints are reusable security principles; package formats are not.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: RustChunk Forge support must install and preserve Forge JAR mods as Java artifacts; never reinterpret .hmod/.hnmod as Forge mods.
- Follow-up checks: Review ZIP traversal, duplicate entries, symlink/device rejection, compression limits, transactional installation and signatures.
- RustChunk implementation mapping: Secure Forge mod downloader/installer validation, separate from Horizon package support.
- Evidence required before adoption: Forge 47.x mod discovery rules, mod metadata, dependency resolution, file integrity and malicious archive tests.
- Completion state: COMPLETE — 70 logical lines through EOF; pinned source SHA and 3449 byte size match; inspected, not modified; tests not executed.

### 2813. `docs/api/mod-index.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b4fd4c0e32da037f55446d8dec1a8db6709d70c0`
- Size: 15817 bytes
- Coverage: Complete through EOF: 217 logical lines; full file; no gaps.
- Purpose: Documents Horizon's registry index, signed release metadata, revoked files and .hpack modpack format.
- Dependencies / cross-references: References horizon_mods::index, modpack, registry CI, GitHub releases, minisign, launcher and mod manifests.
- Implementation / logic: Defines project/version entries, file hashes/signatures, compatibility metadata, registry validation and pack overrides.
- Findings: Registry is for Horizon-specific .hmod/.hnmod/.hpack; release metadata and trust rules do not make its packages Forge-compatible.
- RustChunk relevance: Signed manifests, revocation and hash pinning are useful for RustChunk artifact distribution.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Create a separate RustChunk/Forge repository schema; keep Java mods as JARs and verify dependencies/loader/version constraints.
- Follow-up checks: Audit signature key rotation, revocation, immutable published assets, URL allowlists and override path restrictions.
- RustChunk implementation mapping: RustChunk mod catalog/download metadata and safe profile pack import.
- Evidence required before adoption: Forge mod metadata/dependency graph, official provider APIs, signing policy, hash verification and transactional install tests.
- Completion state: COMPLETE — 217 logical lines through EOF; pinned source SHA and 15817 byte size match; inspected, not modified; tests not executed.

### 2814. `docs/api/server-reference.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `baae92209b2617eef4c338b95c1a8f551d42f691`
- Size: 14118 bytes
- Coverage: Complete through EOF: 186 logical lines; full file; no gaps.
- Purpose: Generated reference for Horizon dedicated-server commands, server.toml and HTTP control endpoints.
- Dependencies / cross-references: Generated from horizon-server source: help, config template and API routes; references server supervisor and panel.
- Implementation / logic: Documents lifecycle commands, server properties mapping, backup/update controls, API routes, metrics and player moderation.
- Findings: Targets a Horizon 26.3 server, not Aternos or Forge 1.20.1; server config names and endpoints are not portable.
- RustChunk relevance: Useful for identifying generic server-management concepts and safe API documentation.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not build server-management features into the Android client based on this document; Aternos remains an external server.
- Follow-up checks: Only if RustChunk later ships a server tool, validate authentication, local binding, destructive confirmation and API versioning.
- RustChunk implementation mapping: No direct client implementation mapping.
- Evidence required before adoption: Separate server product scope, 1.20.1 server protocol/config evidence and security review.
- Completion state: COMPLETE — 186 logical lines through EOF; pinned source SHA and 14118 byte size match; inspected, not modified; tests not executed.

### 2815. `docs/api/server.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `02bffaedb5d8e08d5b50b2c581d770d04fcf395f`
- Size: 7480 bytes
- Coverage: Complete through EOF: 103 logical lines; full file; no gaps.
- Purpose: Overview and usage guide for Horizon's dedicated-server management API.
- Dependencies / cross-references: Depends on horizon-server commands, server.toml, /api/v1, MCP, Prometheus, SSE, RCON and vanilla management protocol.
- Implementation / logic: Explains server setup, process supervision, backup/restore, player moderation, metrics, events and external integrations.
- Findings: All concrete endpoints and protocol details are Horizon 26.3 server-specific; not a Minecraft client or Forge loader layer.
- RustChunk relevance: General monitoring, backup and API security concepts may be retained as future server-side reference.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Keep RustChunk focused on Android client/launcher; do not conflate client multiplayer connectivity with hosting a server.
- Follow-up checks: Any later server work must distinguish vanilla 1.20.1 server properties/protocol and provider-specific APIs.
- RustChunk implementation mapping: No direct implementation mapping in the current client.
- Evidence required before adoption: Explicit server scope, authoritative 1.20.1 server behavior and authenticated endpoint tests.
- Completion state: COMPLETE — 103 logical lines through EOF; pinned source SHA and 7480 byte size match; inspected, not modified; tests not executed.

### 2816. `docs/api/shaders-reference.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `601d6bbcf4c7e87b60335a86a5f2b0a218e7824f`
- Size: 22589 bytes
- Coverage: Complete through EOF: 292 logical lines; full file; no gaps.
- Purpose: Generated WGSL shader API reference for Horizon sky, vertex, material, weather and post stages.
- Dependencies / cross-references: Generated from horizon-shader-api; references renderer frame uniforms, resource bindings, manifest stages and naga validation.
- Implementation / logic: Defines typed inputs, frame offsets, texture/sampler bindings, helper functions, option overrides and resource budgets.
- Findings: API is Horizon-specific and the shader subsystem is marked not started/experimental; bindings and render passes are not vanilla 1.20.1 shader interfaces.
- RustChunk relevance: Typed shader contracts and generated reference/testing are useful design ideas only.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not adopt as a replacement for Minecraft 1.20.1 rendering/shader pipeline; first achieve vanilla/Forge rendering parity.
- Follow-up checks: Compare 1.20.1 render stages, GL requirements, Android Mali driver limits, memory/bandwidth and shader-pack compatibility.
- RustChunk implementation mapping: No current implementation mapping; possible future renderer extension specification.
- Evidence required before adoption: Working RustChunk renderer, 1.20.1 visual parity baseline, Android GPU capability matrix and shader validation tests.
- Completion state: COMPLETE — 292 logical lines through EOF; pinned source SHA and 22589 byte size match; inspected, not modified; tests not executed.

### 2817. `docs/api/shaders.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d50fbe48862c3eb4172f030ac28d5aa5137b3c79`
- Size: 14814 bytes
- Coverage: Complete through EOF: 215 logical lines; full file; no gaps.
- Purpose: Design and pack format for Horizon's custom WGSL shader API.
- Dependencies / cross-references: References horizon-shader-api crate, manifest, WGSL stage files, naga translation and renderer hooks.
- Implementation / logic: Describes shader pack stages, typed inputs, hot reload, debug overlay, budgets, .hshader package and opt-in behavior.
- Findings: Documentation itself states no shader packs currently ship; API differs from Java Edition 1.20.1 shader mods and common mod-loader ecosystems.
- RustChunk relevance: Optional architecture reference for a future Rust-owned shader extension.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Prioritize standard 1.20.1 renderer and Forge compatibility; no custom shader pack API in baseline milestone.
- Follow-up checks: Assess WGSL-to-Android GPU backend support, shader compilation limits, fallback behavior and interaction with Forge render hooks.
- RustChunk implementation mapping: Potential future RustChunk graphics extension, isolated from Forge mod compatibility.
- Evidence required before adoption: Renderer stability, Mali-G52 GPU tests, target-version shader pipeline and compatibility review.
- Completion state: COMPLETE — 215 logical lines through EOF; pinned source SHA and 14814 byte size match; inspected, not modified; tests not executed.

### 2818. `docs/api/stability.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a972790d99515d66f9b1f3ed5774754ede2bd51a`
- Size: 7206 bytes
- Coverage: Complete through EOF: 103 logical lines; full file; no gaps.
- Purpose: Defines Horizon API semantic versioning, experimental/preview/stable levels, deprecation and compatibility tests.
- Dependencies / cross-references: Cross-references every public API surface, generated references, sample files and CI compatibility suites.
- Implementation / logic: Specifies line-based semver, breaking/nonbreaking changes, deprecation windows and promotion evidence.
- Findings: Policy is project-specific and some stated guarantees are aspirational; should not be copied as a promise before RustChunk can test it.
- RustChunk relevance: Good governance model for RustChunk-owned APIs and metadata formats.
- Decision: adaptable; reference
- 1.20.1 implications: Adopt a smaller explicit stability policy after RustChunk's interfaces settle; separate Minecraft version support from RustChunk API version.
- Follow-up checks: Define compatibility fixtures, old-profile migration and deprecation behavior realistically for a small Android project.
- RustChunk implementation mapping: RustChunk API/manifest versioning policy.
- Evidence required before adoption: Maintainer capacity, release cadence, migration tests, documented contracts and actual CI enforcement.
- Completion state: COMPLETE — 103 logical lines through EOF; pinned source SHA and 7206 byte size match; inspected, not modified; tests not executed.

### 2819. `docs/benchmark-report.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8d97fc3fa12b9122ffb76d2b2639802117f60687`
- Size: 8946 bytes
- Coverage: Complete through EOF: 132 logical lines; full file; no gaps.
- Purpose: Specifies JSON benchmark report schema for game/launcher performance and opt-in telemetry upload.
- Dependencies / cross-references: References horizon-bench types, perf_bench, launcher benchmark, telemetry consent and local report storage.
- Implementation / logic: Defines versioned report fields, hardware/conditions, scalar statistics, frame histograms and privacy filtering.
- Findings: Schema includes Horizon-specific fields and planned metrics; hardware GPU memory is not always available and local paths are excluded from upload.
- RustChunk relevance: Strongly adaptable for reproducible FPS, frame pacing, chunk-load and memory measurements.
- Decision: adaptable; reference
- 1.20.1 implications: Design RustChunk benchmark schema around Android thermal state, Mali GPU, chunk loading, frame pacing and Forge mod count.
- Follow-up checks: Ensure comparable conditions, report confidence/variance, avoid uploading identifiers and distinguish client CPU/GPU measurements.
- RustChunk implementation mapping: RustChunk performance telemetry and device benchmark report format.
- Evidence required before adoption: Android instrumentation, privacy review, stable metric definitions, repeated runs and device-specific validation.
- Completion state: COMPLETE — 132 logical lines through EOF; pinned source SHA and 8946 byte size match; inspected, not modified; tests not executed.

### 2820. `docs/benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fcd6e819425d3a230106c7de42f2df32260732fe`
- Size: 14500 bytes
- Coverage: Complete through EOF: 96 logical lines; full file; no gaps.
- Purpose: Explains paired Horizon-versus-vanilla performance benchmark methodology and stress scenes.
- Dependencies / cross-references: Depends on perf_bench.py, graphics_bench, vanilla driver mod, benchmark report schema and pinned performance mods.
- Implementation / logic: Controls machine, world seed, paths, settings, warmup, repeats and statistics; documents caveats for readiness and render backends.
- Findings: Benchmarks are 26.3 desktop-oriented and include a Fabric driver mod; short scenes and differing graphics APIs limit interpretation.
- RustChunk relevance: Methodology transfers well to controlled RustChunk comparisons, but its measurements are not evidence about Android.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Create Android A/B tests on the actual Galaxy A22 class device, including chunk arrival, frame-time tails, thermal throttling and Forge mod packs.
- Follow-up checks: Control background load, battery/thermal state, render distance, seed, camera path, warmup, repeat count and chunk readiness criteria.
- RustChunk implementation mapping: RustChunk performance test harness and chunk-streaming regression suite.
- Evidence required before adoption: Actual device runs, same 1.20.1 world/Forge mods, repeated raw frame times, CPU/GPU telemetry and reproducible baselines.
- Completion state: COMPLETE — 96 logical lines through EOF; pinned source SHA and 14500 byte size match; inspected, not modified; tests not executed.

### 2821. `docs/biome-lookup.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `abb527f6528514ba0456f45dd930652f48fae50b`
- Size: 9839 bytes
- Coverage: Complete through EOF: 161 logical lines; full file; no gaps.
- Purpose: Documents block-to-quart biome selection, biome chunk storage/generation, retrogen retention and biome-source codecs.
- Dependencies / cross-references: References BiomeZoom, BiomeChunk, noise/climate routers, biome source catalog, native vanilla oracle and parity scripts.
- Implementation / logic: Describes coordinate selection, 64-value section storage, generation, below-zero retention, fixed/checkerboard/End/multi-noise sources and registry codecs.
- Findings: All implementation and parity counts target 26.3; full saved palettes, preset builders, blending and complete world generation remain pending.
- RustChunk relevance: Biome math, cache/state boundaries and differential-test methodology are highly relevant references.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild from 1.20.1 biome source/manager/chunk classes and Forge registry lifecycle; never reuse 26.3 preset tables or biome IDs.
- Follow-up checks: Check quart coordinate shifts, seed obfuscation, fuzzy selection, section indexing, retrogen semantics, codec defaults and random source behavior.
- RustChunk implementation mapping: RustChunk worldgen biome subsystem and 1.20.1 parity harness.
- Evidence required before adoption: 1.20.1 mapped source/JAR oracle, exact data packs/registries, target fixtures and full chunk/save integration tests.
- Completion state: COMPLETE — 161 logical lines through EOF; pinned source SHA and 9839 byte size match; inspected, not modified; tests not executed.

### 2822. `docs/biome-temperatures.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b27e91edc57ae128508124a6c8f45f3e43063929`
- Size: 2100 bytes
- Coverage: Complete through EOF: 33 logical lines; full file; no gaps.
- Purpose: Documents biome temperature, frozen modifier, height adjustment, precipitation and cached climate settings.
- Dependencies / cross-references: References Rust noise stack, per-biome worker cache, native biome methods and material-condition integration.
- Implementation / logic: Describes cache capacity/order, packed position keys, NaN eviction behavior and temperature/precipitation parity suite.
- Findings: Targets 26.3 and leaves full biome codecs, automatic thread-local ownership and world-dependent snow/ice placement pending.
- RustChunk relevance: Climate calculations and cache semantics are useful references, not portable target data.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Reimplement with 1.20.1 Biome/ClimateSettings and Java float/double semantics; check cache lifecycle under RustChunk workers.
- Follow-up checks: Test thresholds, modifiers, coordinate packing, sea-level changes, NaN/infinity and thread isolation.
- RustChunk implementation mapping: RustChunk biome climate and surface-weather logic.
- Evidence required before adoption: 1.20.1 source/bytecode, biome registry fixtures, seeded numeric parity and multithreaded tests.
- Completion state: COMPLETE — 33 logical lines through EOF; pinned source SHA and 2100 byte size match; inspected, not modified; tests not executed.

### 2823. `docs/block-state-codecs.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f56e48403051e672bc79752e4b9b658848997b10`
- Size: 1536 bytes
- Coverage: Complete through EOF: 25 logical lines; full file; no gaps.
- Purpose: Describes compact/full block-state JSON codecs over imported block registry identities.
- Dependencies / cross-references: References BlockStateJsonCodec, BlockRegistry, property-kind metadata and parity fixtures.
- Implementation / logic: Defines default-state compact encoding, full id/properties form and fallback rules for invalid/missing properties.
- Findings: Uses 26.3 registry/state metadata and JDK 25 digit handling; arbitrary malformed JSON/unpaired-surrogate behavior is outside tests.
- RustChunk relevance: Codec architecture and explicit property validation are useful, but data must be rebuilt.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Build 1.20.1 block-state/property registries and codec semantics; don't import 26.3 IDs or tables.
- Follow-up checks: Compare Gson/Jackson-like parsing semantics, duplicate fields, property fallback, integer narrowing and Forge-added properties.
- RustChunk implementation mapping: RustChunk block-state registry codec and world-data import.
- Evidence required before adoption: 1.20.1 registry dump, mapped codec source, Forge extension cases and malformed-input differential fixtures.
- Completion state: COMPLETE — 25 logical lines through EOF; pinned source SHA and 1536 byte size match; inspected, not modified; tests not executed.

### 2824. `docs/build-times.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `46881ad86914611751ccab5711e6243cc3888015`
- Size: 9646 bytes
- Coverage: Complete through EOF: 148 logical lines; full file; no gaps.
- Purpose: Documents a repeatable Cargo build-time measurement tool and its cold, incremental and no-change workloads.
- Dependencies / cross-references: References tools/build_times.py, Cargo profiles, crate graph, rustc timings and report outputs.
- Implementation / logic: Measures cold/dev/fast, no-change and controlled source edits; restores edited files and reports critical path/crate recompilation.
- Findings: Published timings are specific to a Windows/WSL Ryzen desktop and old commit/toolchain; not representative of Android builds or user's Windows 7 PC.
- RustChunk relevance: Measurement design can inform RustChunk CI and compile-time optimization.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use only after RustChunk workspace/toolchain stabilizes; measure Android cross-build and host build separately.
- Follow-up checks: Verify edit restoration on interruption, dirty-tree detection, CARGO jobs and reproducible target directories.
- RustChunk implementation mapping: RustChunk build performance dashboard/CI script.
- Evidence required before adoption: Current RustChunk workspace, supported host/NDK toolchains, reproducible runners and baseline reports.
- Completion state: COMPLETE — 148 logical lines through EOF; pinned source SHA and 9646 byte size match; inspected, not modified; tests not executed.

### 2825. `docs/carving.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9dfcde8e66b9dba87f22463091482f30f94d557f`
- Size: 7669 bytes
- Coverage: Complete through EOF: 126 logical lines; full file; no gaps.
- Purpose: Documents carving-mask traversal, ellipsoid geometry, seeded cave/canyon paths and biome-selected carver orchestration.
- Dependencies / cross-references: References CarvingMask, WorldCarver, CarverCatalog, NoiseBasedChunkGenerator, aquifers, biome source and parity scripts.
- Implementation / logic: Describes Y/Z/X packed mask, mutation-aware iteration, ordered interval application, seeded path RNG, 17x17 origin neighborhood and bounded work.
- Findings: 26.3-specific behavior; Gaussian provider codecs, blending, retrogen, full world generation and complete worlds remain pending.
- RustChunk relevance: Core worldgen algorithms and bounded traversal/error semantics are valuable research references.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild target-era carver codecs, random streams, masks and generation ordering from Minecraft 1.20.1; account for Forge hooks.
- Follow-up checks: Verify RNG draw order, coordinate wrapping, mask mutation, aquifer scheduling, bounds, error partial writes and target generation stage order.
- RustChunk implementation mapping: RustChunk 1.20.1 worldgen carvers and chunk generation pipeline.
- Evidence required before adoption: 1.20.1 Mojang-mapped carver sources, data-driven worldgen fixtures, Forge registry integration and seeded differential tests.
- Completion state: COMPLETE — 126 logical lines through EOF; pinned source SHA and 7669 byte size match; inspected, not modified; tests not executed.

## Checkpoint 2826–2850 — COMPLETE

### 2826. `docs/chat-filter.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b4c63b3d7b62e5322d49251f53e003b6dfe15c6b`
- Size: 3827 bytes
- Coverage: 1–67; complete EOF; 67 canonical logical lines, no gaps.
- Purpose: Documents the dedicated server's chat-filter policy and handling of rejected messages.
- Dependencies / cross-references: References server chat processing, moderation policy and player-facing message behavior.
- Implementation / logic: Describes filtering as a server-side decision before chat forwarding; the client receives only accepted chat.
- Findings: No client-side filtering implementation or 1.20.1/Forge integration is established by this guide.
- RustChunk relevance: Low relevance to launcher/rendering; useful only for future moderation UX or preserving server-authoritative chat behavior.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not filter or rewrite multiplayer chat in RustChunk; preserve the server's authoritative messages and signatures.
- Follow-up checks: If adding a client chat filter, verify signed-chat handling, message trust indicators and server policy boundaries.
- RustChunk implementation mapping: No baseline implementation; optional chat display preference only.
- Evidence required before adoption: 1.20.1 signed-chat behavior, Forge chat events, privacy and moderation requirements.
- Completion state: COMPLETE — 67 logical lines through EOF; pinned source SHA and 3827 byte size match; inspected, not modified; tests not executed.

### 2827. `docs/ci-runners.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5efb92ef42a873e7bbfdf1ce92623c029eb4c656`
- Size: 6071 bytes
- Coverage: 1–88; complete EOF; 88 canonical logical lines, no gaps.
- Purpose: Documents CI runner provisioning and build/test workflow assumptions for Starling.
- Dependencies / cross-references: References GitHub Actions, Rust toolchains, native dependencies, caches and platform-specific jobs.
- Implementation / logic: Describes runner setup, cache use and separation of host checks from platform builds.
- Findings: Runner image/toolchain versions and hardware are repository-specific; a green hosted job does not prove Android device behavior.
- RustChunk relevance: Useful for RustChunk CI design, especially reproducible cross-platform build jobs.
- Decision: adaptable; reference
- 1.20.1 implications: Define a minimal reproducible matrix for Rust host tests and Android ARM64 packaging; do not copy runner versions blindly.
- Follow-up checks: Verify NDK/JDK/Rust versions, Windows compatibility, cache keys, artifact provenance and actual Android target execution.
- RustChunk implementation mapping: RustChunk GitHub Actions workflow and reproducible build environment.
- Evidence required before adoption: Current project toolchain pins, Android SDK/NDK matrix, signed artifact hashes and clean-run results.
- Completion state: COMPLETE — 88 logical lines through EOF; pinned source SHA and 6071 byte size match; inspected, not modified; tests not executed.

### 2828. `docs/collision-geometry.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2ea44c5b1d2586f50ec2c37fcf32c9be4ad201f1`
- Size: 132833 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2109; complete EOF; 2109 canonical logical lines, no gaps.
- Purpose: Large technical record of collision shapes, world queries, entity movement, border geometry, fluids, fall, damage, inventory and item interactions.
- Dependencies / cross-references: Cross-references collision registry, TerrainChunk, native parity scripts, entity/world adapters and many scoped benchmark reports.
- Implementation / logic: Describes granular Rust implementations and Java-oracle comparisons for collision scanning, step movement, support blocks, fluids, entity movement, combat and item lifecycle; repeatedly distinguishes controlled fixtures from live gameplay.
- Findings: Targets 26.3 and JDK 25; many providers remain controlled and full live gameplay/client integration is explicitly pending. Matching fixture traces are not proof of complete gameplay parity.
- RustChunk relevance: Highly relevant research for RustChunk physics and collision architecture, but it is documentation evidence, not portable implementation.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild required behavior from 1.20.1 mapped classes and Forge hooks; preserve Java numeric order, callback side effects and packet-visible movement semantics.
- Follow-up checks: Prioritize 1.20.1 collision shapes, entity movement, fluid interaction and player travel; verify each with a native Java oracle before connecting live state.
- RustChunk implementation mapping: RustChunk physics/world adapter, block collision scanner, player movement and item state modules.
- Evidence required before adoption: 1.20.1 source/JAR oracle, Forge-added block/entity cases, seeded differential traces, live chunk/entity lifecycle tests and Android runtime validation.
- Completion state: COMPLETE — 2109 logical lines through EOF; pinned source SHA and 132833 byte size match; inspected, not modified; tests not executed.

### 2829. `docs/controllers.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `75240721c975d4a572c9dc3677e140f6dbe53adf`
- Size: 34712 bytes
- Coverage: 1–400, 401–583; complete EOF; 583 canonical logical lines, no gaps.
- Purpose: Describes controller input, mappings, menu navigation, on-screen keyboard, rumble, DualSense outputs and Android event handling.
- Dependencies / cross-references: References gilrs, vendored winit Android hooks, gamepad/android.rs, input modes, UI focus targets and HID/audio output layers.
- Implementation / logic: Uses Android KEYCODE_BUTTON and joystick axes before winit; maps gamepad actions to vanilla key mappings, provides analog movement, focus/pointer UI modes and optional controller feedback.
- Findings: Most controller stack is Horizon-specific; Android input hooks and mappings are tied to its vendored winit. DualSense HID/audio output is platform-sensitive and partly reference-only.
- RustChunk relevance: Relevant to Android external-controller support and accessible UI navigation; not a prerequisite for touch controls or Forge compatibility.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use Android native key/axis events and a thin Rust mapping layer; keep keyboard/touch working and add controller support only after core input is stable.
- Follow-up checks: Check Android API event delivery, axis calibration/deadzones, focus loss, device reconnect, key repeat, winit interception and controller vibration permissions.
- RustChunk implementation mapping: RustChunk input abstraction and optional gamepad navigation layer.
- Evidence required before adoption: Actual Android controller devices, event traces, controller focus tests, touch coexistence and Samsung device compatibility.
- Completion state: COMPLETE — 583 logical lines through EOF; pinned source SHA and 34712 byte size match; inspected, not modified; tests not executed.

### 2830. `docs/cosmetics.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9ff1e0d7d7acf1c478930a4d01f4198135e38efa`
- Size: 5410 bytes
- Coverage: 1–96; complete EOF; 96 canonical logical lines, no gaps.
- Purpose: Specifies optional client-only cosmetic items, JSON models, animation limits and a remote cosmetics service.
- Dependencies / cross-references: References player model bones, skin atlas, HTTP service, UUID lookup, cache and GPU model path.
- Implementation / logic: Defines opt-in cosmetics, bounded item geometry, cache TTL/backoff and rendering on player bones without changing game protocol.
- Findings: Adds UUID-based network requests and a new remote service; off-by-default behavior and cache/vertex limits are Horizon-specific.
- RustChunk relevance: Not needed for RustChunk's launcher, renderer stability or 1.20.1 Forge milestone.
- Decision: unnecessary; optional/future; version-specific
- 1.20.1 implications: Do not include in the baseline; cosmetic extensions would need separate privacy, trust and asset validation.
- Follow-up checks: Review UUID disclosure, service failure behavior, malformed model/texture limits and player visibility settings if revisited.
- RustChunk implementation mapping: No current implementation mapping; possible isolated cosmetic module later.
- Evidence required before adoption: Explicit product scope, privacy consent, model sandbox/limits, cache tests and target-version player model parity.
- Completion state: COMPLETE — 96 logical lines through EOF; pinned source SHA and 5410 byte size match; inspected, not modified; tests not executed.

### 2831. `docs/datapack-upgrade.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9015bc86d871c577cdefa07ccf2f3b39a6a6e2c9`
- Size: 6315 bytes
- Coverage: 1–86; complete EOF; 86 canonical logical lines, no gaps.
- Purpose: Documents migration of Minecraft 26.2 world-generation data packs to 26.3.
- Dependencies / cross-references: References horizon-port-pack, vanilla 26.2/26.3 registries, density functions, features, carvers and fidelity checks.
- Implementation / logic: Lists registry/codec rewrites and validation against both vanilla versions and generated terrain.
- Findings: The migration is specifically 26.2→26.3 and explicitly notes unsupported transformations; it does not cover 1.20.1 data packs.
- RustChunk relevance: No direct target value for RustChunk 1.20.1 except as an example of versioned datapack migration discipline.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not port these conversion rules; preserve 1.20.1 datapack schemas and Forge registry behavior.
- Follow-up checks: If a converter is ever needed, derive it from the exact source and target versions and test full generated chunks.
- RustChunk implementation mapping: No current implementation mapping.
- Evidence required before adoption: 1.20.1 datapack registry/codec definitions, representative packs, loss report and generated-world fidelity.
- Completion state: COMPLETE — 86 logical lines through EOF; pinned source SHA and 6315 byte size match; inspected, not modified; tests not executed.

### 2832. `docs/density-functions.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5357abeb3757f34c89e07bbb9dec6e46853b4e28`
- Size: 31782 bytes
- Coverage: 1–400, 401–502; complete EOF; 502 canonical logical lines, no gaps.
- Purpose: Documents compiled density samplers, graph compilation, random/noise handling, intervals, caches and scalar/bulk evaluation.
- Dependencies / cross-references: References horizon_core::density, DensityCompiler, SamplerContext, noise registries, world fields and Java oracle suites.
- Implementation / logic: Defines immutable sampler graphs, per-worker scratch/cache contexts, specialization, seeded noise, range propagation and explicit bounded evaluation semantics.
- Findings: Targets 26.3; serialized registries, complete graph coverage, concrete world-field producers and full seeded chunk generation remain pending. Synthetic metadata does not prove terrain parity.
- RustChunk relevance: Core world-generation architecture is relevant, especially deterministic noise and bounded bulk sampling.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild 1.20.1 density functions, random streams, codec graph and cache behavior from the target JAR; do not reuse 26.3 registries or presets.
- Follow-up checks: Verify scalar/bulk bit parity, RNG draw order, signed zero/NaN, interval ranges, cache identity, buffer tails and worker ownership.
- RustChunk implementation mapping: RustChunk worldgen density graph, sampler context and seeded noise modules.
- Evidence required before adoption: 1.20.1 mapped source/JAR, registry exports, deterministic seed fixtures, codec corpus, full chunk-generation tests and Forge datapack hooks.
- Completion state: COMPLETE — 502 logical lines through EOF; pinned source SHA and 31782 byte size match; inspected, not modified; tests not executed.

### 2833. `docs/design-system.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d150cc99380452cedaab03d55ada5f26011fb813`
- Size: 25755 bytes
- Coverage: 1–351; complete EOF; 351 canonical logical lines, no gaps.
- Purpose: Describes Horizon's typography, color tokens, spacing, components, launcher surfaces and multiple visual themes.
- Dependencies / cross-references: References horizon-theme, launcher, game settings UI, website CSS and generated tokens.
- Implementation / logic: Centralizes design tokens and defines list-based layouts, accessibility contrast, motion, controls and surface-specific themes.
- Findings: Strongly branded Horizon design language; launcher mockups and web-specific details are not a neutral architecture standard.
- RustChunk relevance: Useful as a UI-system reference, but RustChunk should retain its own identity and Android-native constraints.
- Decision: adaptable; reference
- 1.20.1 implications: Use design tokens and contrast/accessibility tests selectively; do not copy Horizon branding, fonts, sponsor elements or desktop layout wholesale.
- Follow-up checks: Check Android screen sizes, touch target dimensions, font licensing, reduced motion, dark mode and game-vs-launcher separation.
- RustChunk implementation mapping: RustChunk launcher settings/profile UI design system.
- Evidence required before adoption: RustChunk brand direction, Android accessibility tests, license review and visual validation on the target phone.
- Completion state: COMPLETE — 351 logical lines through EOF; pinned source SHA and 25755 byte size match; inspected, not modified; tests not executed.

### 2834. `docs/design/compositor.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `44e73244d3f450f064427a2a9fcdd884b5c41443`
- Size: 1108 bytes
- Coverage: 1–19; complete EOF; 19 canonical logical lines, no gaps.
- Purpose: Explains an optional offscreen frame compositor for multiple views in one window.
- Dependencies / cross-references: References renderer offscreen surfaces, same-device GPU surface copies and Renderer::take_back.
- Implementation / logic: Copies BGRA views into clipped positions and clears uncovered pixels; normal game path does not instantiate it.
- Findings: Part of an unfinished splitscreen series; no ordinary rendering path depends on it.
- RustChunk relevance: Future multi-instance/splitscreen research only; not relevant to initial single-player/multiplayer client.
- Decision: unnecessary; optional/future; version-specific
- 1.20.1 implications: Do not add to the baseline renderer; revisit only after single-view rendering and frame pacing are stable.
- Follow-up checks: Verify GPU device identity, surface format support, resize clipping, resource lifetime and synchronization.
- RustChunk implementation mapping: Possible future splitscreen renderer module.
- Evidence required before adoption: Working Android multi-window/splitscreen requirements, GPU surface-copy tests and actual multi-view frame benchmarks.
- Completion state: COMPLETE — 19 logical lines through EOF; pinned source SHA and 1108 byte size match; inspected, not modified; tests not executed.

### 2835. `docs/design/shared-audio.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9d4dc50090039c34bfaa15471cb5ff848de73633`
- Size: 1361 bytes
- Coverage: 1–24; complete EOF; 24 canonical logical lines, no gaps.
- Purpose: Describes sharing one audio output and decoded sample cache across multiple sound engines/listeners.
- Dependencies / cross-references: References Audio::share, owner IDs, world/dimension identity, listener positions and channel lifetime.
- Implementation / logic: Deduplicates nearby world sounds across listeners while retaining relative/looping channels and safe replacement IDs.
- Findings: Current game still uses one listener; player-session integration is not complete.
- RustChunk relevance: Optional splitscreen/multi-listener architecture, not needed for one-player client audio.
- Decision: unnecessary; optional/future; version-specific
- 1.20.1 implications: Keep one audio engine/listener in the first RustChunk milestone.
- Follow-up checks: If multi-listener support is introduced, test channel ownership, late decoder replies, world identity and output lifetime.
- RustChunk implementation mapping: No current mapping; future audio subsystem extension.
- Evidence required before adoption: A real multi-player session model, audio-device tests and concurrent listener lifecycle tests.
- Completion state: COMPLETE — 24 logical lines through EOF; pinned source SHA and 1361 byte size match; inspected, not modified; tests not executed.

### 2836. `docs/extras.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d614ecc3056151b63de03d112181ffa13f96106c`
- Size: 16780 bytes
- Coverage: 1–257; complete EOF; 257 canonical logical lines, no gaps.
- Purpose: Catalogues optional client extras such as dynamic lights, reconnect, inventory tweaks, minimap, HUD counters and music controls.
- Dependencies / cross-references: References renderer hooks, extras settings, inventory logic, reconnect policy, sound events and native mods.
- Implementation / logic: Most extras are disabled by default and use isolated hooks/settings; some moved to Horizon native mods with shared hook points.
- Findings: Features are 26.3-specific and many are independent of vanilla parity; some depend on a native mod ecosystem not equivalent to Forge.
- RustChunk relevance: Selective ideas may improve usability later, but extras must not distract from baseline Forge 1.20.1 support.
- Decision: adaptable; reference; optional/future
- 1.20.1 implications: Defer extras until launch, world loading, chunk stability and Forge compatibility are proven; consider reconnect and diagnostics first.
- Follow-up checks: Verify each extra independently for server-visible effects, performance overhead, conflicts with Forge mods and off-state no-op behavior.
- RustChunk implementation mapping: Optional RustChunk settings modules after core milestones.
- Evidence required before adoption: 1.20.1/Forge hooks, mod-conflict matrix, off-state performance tests and actual Android device measurements.
- Completion state: COMPLETE — 257 logical lines through EOF; pinned source SHA and 16780 byte size match; inspected, not modified; tests not executed.

### 2837. `docs/face-visibility.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `093695af657c603d52e1081bc5df40bad7dd92b2`
- Size: 2514 bytes
- Coverage: 1–41; complete EOF; 41 canonical logical lines, no gaps.
- Purpose: Describes neighboring block-model face occlusion and cached face subtraction for chunk meshing.
- Dependencies / cross-references: References FaceVisibility, block occlusion shapes, canonical face identities, resource-pack state and native parity suite.
- Implementation / logic: Uses ordered full-face, block-specific and partial-shape rules; bounded shape-pair cache; integrated with resident chunk meshing in the documented client.
- Findings: All state identities, tags and special cases are pinned to 26.3; GPU pixels, lighting and full gameplay are outside the comparison.
- RustChunk relevance: Directly relevant to chunk mesh efficiency and correct visibility; must be rebuilt for 1.20.1 block states and Forge registries.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Reimplement against 1.20.1 VoxelShape/occlusion semantics and Forge-added blocks; preserve conservative visibility for unknown neighbors.
- Follow-up checks: Test all six directions, partial shapes, glass/leaves/liquids, missing chunk boundaries, resource packs and dynamic block states.
- RustChunk implementation mapping: RustChunk chunk mesher's face-visibility stage.
- Evidence required before adoption: 1.20.1 registry/state exports, Forge modded block cases, face-mask differential tests and GPU render regression.
- Completion state: COMPLETE — 41 logical lines through EOF; pinned source SHA and 2514 byte size match; inspected, not modified; tests not executed.

### 2838. `docs/feature-placement.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3c65d395a9b0108058aa4324d9f9c3acc2acaffa`
- Size: 38623 bytes
- Coverage: 1–400, 401–638; complete EOF; 638 canonical logical lines, no gaps.
- Purpose: Documents placement modifiers, feature graphs, world mutation, resident feature regions, block providers, predicates and decoration scheduling.
- Dependencies / cross-references: References FeaturePlacer, TerrainRegion, biome ordering, random streams, feature catalog, block-state providers and many parity reports.
- Implementation / logic: Implements ordered bounded feature execution with shared random streams, explicit world query/mutation providers and staged feature composition.
- Findings: Targeted at 26.3; full datapack loading, complete registries, structures, postprocessing and complete generated worlds remain pending.
- RustChunk relevance: Highly relevant to worldgen and seeded chunk population, but target version changes many codecs and feature registries.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild the 1.20.1 feature/placement registry graph and Forge worldgen hooks; do not reuse 26.3 feature IDs or codec shapes.
- Follow-up checks: Verify modifier order, random draw consumption, partial writes, biome ordering, heightmaps, registry holder resolution and feature stages.
- RustChunk implementation mapping: RustChunk worldgen feature placement and chunk decoration pipeline.
- Evidence required before adoption: 1.20.1 vanilla+Forge feature registry, datapack fixtures, deterministic seeds, full chunk snapshots and retrogen tests.
- Completion state: COMPLETE — 638 logical lines through EOF; pinned source SHA and 38623 byte size match; inspected, not modified; tests not executed.

### 2839. `docs/generation-scan-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `89a4b2c0c4dc21305bb1cddbcdc5b94f445dca30`
- Size: 3013 bytes
- Coverage: 1–52; complete EOF; 52 canonical logical lines, no gaps.
- Purpose: Reports an optimization that reuses a completed generation scan until relevant state changes.
- Dependencies / cross-references: References Streaming::update, ChunkMap structural tokens, generation readiness and ignored microbenchmark.
- Implementation / logic: Uses structural membership/range/generation invalidation to avoid repeated scans while retaining bounded pending scheduling.
- Findings: Benchmark is a Linux desktop microbenchmark; synthetic chunk keys and a single workload do not establish Android FPS or end-to-end gains.
- RustChunk relevance: Potentially relevant to avoiding redundant chunk scheduler work, but only after profiling RustChunk's actual bottleneck.
- Decision: adaptable; reference
- 1.20.1 implications: Consider a generation-readiness gate only if RustChunk has the same repeated-scan pattern and correct invalidation signals.
- Follow-up checks: Benchmark on Android ARM64 and test all membership/range/state invalidation paths; avoid confusing this with actual chunk generation latency.
- RustChunk implementation mapping: Future chunk streaming scheduler optimization.
- Evidence required before adoption: RustChunk scheduler traces, invalidation tests, target-device CPU/frame-time measurements and correctness under chunk arrival/unload.
- Completion state: COMPLETE — 52 logical lines through EOF; pinned source SHA and 3013 byte size match; inspected, not modified; tests not executed.

### 2840. `docs/graphics.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2a7670d70e0db2c193b43fc546488a16290e450f`
- Size: 37764 bytes
- Coverage: 1–162; complete EOF; 162 canonical logical lines, no gaps.
- Purpose: Describes graphics settings, culling, frame pacing, HDR, LOD, TAA/upscaling, frame generation and desktop benchmark results.
- Dependencies / cross-references: References renderer passes, shaders, chunk meshes, GPU timestamps, distant terrain and graphics settings storage.
- Implementation / logic: Defines separate vanilla/Horizon settings, optional rendering optimizations and LOD sampling/refinement paths.
- Findings: Targets 26.3 and desktop GPUs; many capabilities are unavailable or costly on Android. LOD is approximate, shader packs disabled in places, and benchmark numbers are not transferable.
- RustChunk relevance: Relevant to renderer design and LOD research, but does not solve near-chunk loading or prove performance on Mali-G52.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Prioritize stable near chunks, correct frame pacing and Forge render compatibility before LOD/TAA/HDR/frame generation; avoid treating distant terrain as a fix for missing nearby ground.
- Follow-up checks: Measure on Android Mali GPU, verify GLES/Vulkan backend support, memory budgets, shader hooks, chunk readiness and visual correctness.
- RustChunk implementation mapping: RustChunk renderer settings and later optional distant terrain/graphics optimizations.
- Evidence required before adoption: Galaxy-class ARM64 device measurements, 1.20.1 render parity, Forge renderer compatibility and per-feature GPU/memory tests.
- Completion state: COMPLETE — 162 logical lines through EOF; pinned source SHA and 37764 byte size match; inspected, not modified; tests not executed.

### 2841. `docs/heightmaps.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `74d28b21cb423af853a2fa85e208a986413f38c2`
- Size: 3279 bytes
- Coverage: 1–57; complete EOF; 57 canonical logical lines, no gaps.
- Purpose: Documents six heightmap types, packed storage, update/prime order and raw data behavior.
- Dependencies / cross-references: References HeightmapState, HeightmapChunk, block tags, ProtoChunk parity fixtures and heightmap persistence.
- Implementation / logic: Stores 256 column heights in packed words; updates and priming follow native order and preserve raw padding when lengths match.
- Findings: 26.3 block tags and runtime bounds are version-specific; chunk lifecycle, generated terrain and lighting remain pending.
- RustChunk relevance: Heightmaps are fundamental to chunk generation, placement, world surface queries and rendering culling.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Implement exact 1.20.1 heightmap types/tags and packed encoding, with Forge block state extensions.
- Follow-up checks: Test all heightmap predicates, updates/removals, priming order, vertical bounds, raw imports and packed-word edge cases.
- RustChunk implementation mapping: RustChunk chunk storage and worldgen heightmap subsystem.
- Evidence required before adoption: 1.20.1 Heightmap source/JAR oracle, Forge tags/states, save/load fixtures and generated chunk integration.
- Completion state: COMPLETE — 57 logical lines through EOF; pinned source SHA and 3279 byte size match; inspected, not modified; tests not executed.

### 2842. `docs/horizon-ctl-commands.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fdb513bd220ecaefb71f4c8a45df4c0a3c7fa4bf`
- Size: 23184 bytes
- Coverage: 1–158; complete EOF; 158 canonical logical lines, no gaps.
- Purpose: Generated reference of game control commands, including world edits, player movement, multiplayer, screenshots and benchmarks.
- Dependencies / cross-references: References horizon-ctl command table, control socket, launcher MCP and agent-control protocol.
- Implementation / logic: Exposes structured observe/action commands with wait conditions, confirmations for destructive actions and optional agent controls.
- Findings: This is a large automation surface with powerful world/network/destructive actions; its schema is tied to Horizon 26.3 and desktop process model.
- RustChunk relevance: Optional debugging and automated regression testing can benefit from a narrow subset.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Do not implement broad agent control in the first release; consider read-only diagnostics and explicit user-driven test hooks later.
- Follow-up checks: Threat-model command auth, destructive confirmations, world backup, network actions, chat disclosure and Android process isolation.
- RustChunk implementation mapping: Optional RustChunk diagnostic/test API, not part of launcher core.
- Evidence required before adoption: Strict schemas, local-only authentication, user consent, audit logs, safe defaults and Android lifecycle tests.
- Completion state: COMPLETE — 158 logical lines through EOF; pinned source SHA and 23184 byte size match; inspected, not modified; tests not executed.

### 2843. `docs/hosting.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eb8b829bf8b78b4762f308a890bd99fa4d904f23`
- Size: 34932 bytes
- Coverage: 1–400, 401–605; complete EOF; 605 canonical logical lines, no gaps.
- Purpose: Documents Horizon's dedicated server installation, configuration, online/offline modes, management APIs, backups, hosting panel and benchmarks.
- Dependencies / cross-references: References horizon-server, server.toml, prepared 26.3 resources, RCON, GS4 query, Velocity forwarding, LOD channel and control API.
- Implementation / logic: Wraps a native 26.3 server engine with a supervisor, configuration translation to server.properties, account checks, optional offline mode and administration tools.
- Findings: Not a client launcher implementation; target version is 26.3 and offline-mode servers are explicitly unsafe on public networks. Aternos providers have their own control planes.
- RustChunk relevance: Useful to understand multiplayer compatibility boundaries and online/offline server behavior; RustChunk should connect to external servers, not host one initially.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not build a server host into RustChunk; focus on correct 1.20.1 client protocol, Forge handshake/mod compatibility and safe account modes.
- Follow-up checks: Verify 1.20.1 login/secure-chat/brand/Forge channels and offline-mode behavior against target servers; do not assume Horizon's private LOD channel exists.
- RustChunk implementation mapping: No server-engine mapping; use only as multiplayer protocol/security research.
- Evidence required before adoption: Actual 1.20.1 vanilla/Forge/Aternos connections, packet captures, login/session tests and server-side compatibility evidence.
- Completion state: COMPLETE — 605 logical lines through EOF; pinned source SHA and 34932 byte size match; inspected, not modified; tests not executed.

### 2844. `docs/known-issues.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `42c4e21c1752bc8d6e09a86a91abc664168da79b`
- Size: 359 bytes
- Coverage: 1–8; complete EOF; 8 canonical logical lines, no gaps.
- Purpose: Lists the current player-facing known issue for shader packs.
- Dependencies / cross-references: Generated from release tooling and links shader-pack behavior to the release notes.
- Implementation / logic: States shader packs are temporarily removed and will be rewritten later.
- Findings: A short 26.3 release note, not a technical implementation description; status can change between releases.
- RustChunk relevance: Only useful as an example of concise release-note disclosure.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: No implementation mapping; RustChunk should maintain its own truthful known-issues list.
- Follow-up checks: Confirm any renderer limitations against the actual RustChunk build before publishing.
- RustChunk implementation mapping: RustChunk release notes only.
- Evidence required before adoption: Verified issue reproduction, affected versions and confirmation that a fix has shipped.
- Completion state: COMPLETE — 8 logical lines through EOF; pinned source SHA and 359 byte size match; inspected, not modified; tests not executed.

### 2845. `docs/launcher-commands.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4ac90461043255eef2d0a3819c18b63b3edf1aa3`
- Size: 31393 bytes
- Coverage: 1–400, 401–800, 801–1064; complete EOF; 1064 canonical logical lines, no gaps.
- Purpose: Generated command reference for Horizon launcher operations: releases, profiles, accounts, Java, mods, worlds, packs, servers and telemetry.
- Dependencies / cross-references: Generated from launcher/src/cli.rs; references launcher MCP, instance manager, mod browser, accounts, preparation and game control.
- Implementation / logic: Defines a broad CLI with explicit confirmation for destructive commands, online/offline account selection, Java instances and modpack workflows.
- Findings: Desktop GPUI launcher, Microsoft owner-gate assumptions, Horizon-native mod formats and Java instance loaders differ from RustChunk Android and Forge 1.20.1 goals.
- RustChunk relevance: Strong reference for launcher feature decomposition and offline profile semantics, but not code or a directly reusable command set.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Design Android-first profile/instance management; separate account ownership, offline profile creation, game files, Java runtime and Forge installation.
- Follow-up checks: Verify profile data isolation, offline identity semantics, mod JAR handling, Java 17 selection, destructive confirmations and Android scoped storage.
- RustChunk implementation mapping: RustChunk launcher navigation, profile manager and Forge modpack/import workflow.
- Evidence required before adoption: 1.20.1 Forge install fixtures, Android profile lifecycle, online/offline compliance review and backup/restore tests.
- Completion state: COMPLETE — 1064 logical lines through EOF; pinned source SHA and 31393 byte size match; inspected, not modified; tests not executed.

### 2846. `docs/launcher-preparation.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `128814fae1be7aa8f6db252cc1f84731c1802ad3`
- Size: 6887 bytes
- Coverage: 1–101; complete EOF; 101 canonical logical lines, no gaps.
- Purpose: Describes resource preparation using original Java exporter classes, checksum manifests, isolated JVMs and batched execution, including Starling APK preparation.
- Dependencies / cross-references: References horizon-prepare, Java exporters, research-lock, APK prepare-kit, source pin keys and resource manifests.
- Implementation / logic: Compiles and validates pinned Java exporters, caches by source/dependency/compiler hashes, runs exporters in bounded workers and verifies repeatable resource output.
- Findings: Requires JDK 25 and 26.3 resources in this snapshot; it does not provide a 1.20.1 Forge installation pipeline. Exporter execution and Java toolchain footprint may be heavy on Android.
- RustChunk relevance: Highly relevant as a resource-preparation and integrity model; Android prepare-kit separation is especially worth studying.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Separate host-side resource extraction from Android runtime; adapt exporter inputs and hashes to 1.20.1, Forge 47.x and Java 17; keep Android package lean.
- Follow-up checks: Test manifest pinning, class/resource hash validation, cache invalidation, isolated-vs-batched byte equality and memory use on the host build machine.
- RustChunk implementation mapping: RustChunk resource manifest, host preparation tool and Android prepare-kit packaging.
- Evidence required before adoption: 1.20.1/Forge artifacts, exact Java 17 requirement, reproducible resource outputs, Android memory/storage limits and clean install tests.
- Completion state: COMPLETE — 101 logical lines through EOF; pinned source SHA and 6887 byte size match; inspected, not modified; tests not executed.

### 2847. `docs/material-conditions.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `044d9dd0ba34680f46c40daabd7c4875e0640318`
- Size: 5682 bytes
- Coverage: 1–88; complete EOF; 88 canonical logical lines, no gaps.
- Purpose: Documents compiled numeric world-surface conditions, worker caches, noise thresholds and registry-bound biome/height inputs.
- Dependencies / cross-references: References MaterialConditions, material rule libraries, density sources, biome tags and surface stage parity.
- Implementation / logic: Compiles per-worker condition trees with cache invalidation by column/block revisions and lazy secondary noise.
- Findings: 26.3 condition types and registries are not 1.20.1 contracts; live biome-tag reloads and some registry-owner validation remain pending.
- RustChunk relevance: Useful for the surface-rule evaluator and worker cache architecture once the correct 1.20.1 schemas are known.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild target-version condition codecs and arithmetic; keep worker cache lifetimes explicit and separate from client rendering.
- Follow-up checks: Test cache invalidation, seeded noise laziness, float/double conversion, tag reload and query-failure behavior.
- RustChunk implementation mapping: RustChunk worldgen surface-condition subsystem.
- Evidence required before adoption: 1.20.1 material-condition codecs, registry snapshots, seeded Java oracle and chunk surface integration.
- Completion state: COMPLETE — 88 logical lines through EOF; pinned source SHA and 5682 byte size match; inspected, not modified; tests not executed.

### 2848. `docs/material-rule-libraries.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `47a434d9f086b469565380d0ea53136b59719063`
- Size: 2282 bytes
- Coverage: 1–36; complete EOF; 36 canonical logical lines, no gaps.
- Purpose: Describes loading named material-rule resources and compiling density roots for chunk surface application.
- Dependencies / cross-references: References MaterialRuleLibrary, density compiler, condition/noise/biome registries and surface stage.
- Implementation / logic: Loads immutable named rule snapshots atomically, resolves forward references, detects cycles and lazily compiles density dependencies.
- Findings: 42 bundled rules and codec shapes are 26.3-specific; encoding, live reload and registry-owner checks remain pending.
- RustChunk relevance: Relevant as a worldgen resource graph design, not as target-version data.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Build a 1.20.1/Forge-specific rule catalog and dependency resolver from the actual target registry.
- Follow-up checks: Verify atomic failure, duplicate names, cycles, forward references, seeded density dependencies and reload semantics.
- RustChunk implementation mapping: RustChunk worldgen rule catalog and surface pipeline.
- Evidence required before adoption: 1.20.1 resource files, Forge registry lifecycles, dependency fixtures and generated chunk parity.
- Completion state: COMPLETE — 36 logical lines through EOF; pinned source SHA and 2282 byte size match; inspected, not modified; tests not executed.

### 2849. `docs/material-surfaces.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2f8ad65b12777351bf28f445759c2a12acb7fdbb`
- Size: 7590 bytes
- Coverage: 1–125; complete EOF; 125 canonical logical lines, no gaps.
- Purpose: Documents seeded material surfaces, clay bands, biome-specific extensions and chunk surface application.
- Dependencies / cross-references: References MaterialSurface, MaterialConditions, rule libraries, heightmaps, density compiler, biome queries and fluid postprocessing.
- Implementation / logic: Applies surface rules in X/Z order, updates heightmaps and fluid offsets, shares worker density context and retains partial writes on failure.
- Findings: Targets 26.3; full biome preset builders, feature/structure stages, nonempty blending, lighting and complete worlds remain pending.
- RustChunk relevance: Highly relevant to terrain surface generation, but requires a full 1.20.1 rewrite and integration with Forge worldgen.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Reimplement 1.20.1 surface rules and biome extensions only after density, heightmap and feature registries are pinned.
- Follow-up checks: Test seeded block outputs, write order, heightmaps, fluid postprocessing, biome neighborhoods and failure retention.
- RustChunk implementation mapping: RustChunk worldgen surface stage.
- Evidence required before adoption: 1.20.1 source/JAR oracle, Forge biome registry, deterministic world seeds, chunk snapshots and end-to-end generation.
- Completion state: COMPLETE — 125 logical lines through EOF; pinned source SHA and 7590 byte size match; inspected, not modified; tests not executed.

### 2850. `docs/media/brand/README.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5b64530b6373ad3364809d16ec53286f5b0e2c68`
- Size: 5265 bytes
- Coverage: 1–66; complete EOF; 66 canonical logical lines, no gaps.
- Purpose: Documents Horizon's icon, mark, social preview, generated platform icons and asset upload workflow.
- Dependencies / cross-references: References design-system tokens, xtask icon generation, Linux/macOS/Windows packaging and website assets.
- Implementation / logic: Defines editable SVG masters and generated PNG/ICO/ICNS/desktop-entry outputs, plus brand upload instructions.
- Findings: Brand assets, account names and platform paths are Horizon-specific; no launcher/runtime architecture is present.
- RustChunk relevance: No direct RustChunk technical value beyond general source-vs-generated asset discipline.
- Decision: unnecessary; reference
- 1.20.1 implications: Create RustChunk branding independently; if needed, keep source vector assets separate from generated platform icons.
- Follow-up checks: Verify icon licenses, Android adaptive icon requirements, app label and actual Samsung Gaming Hub classification independently.
- RustChunk implementation mapping: No current implementation mapping beyond future RustChunk icon asset pipeline.
- Evidence required before adoption: Original RustChunk brand assets, Android adaptive icon export, licensing and Samsung launcher/game classification tests.
- Completion state: COMPLETE — 66 logical lines through EOF; pinned source SHA and 5265 byte size match; inspected, not modified; tests not executed.

### 2851. `docs/media/entity-culling-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8a4c2724d5524b039f66f876e7dad7e8fcee89a0`
- Size: 1786 bytes
- Coverage: 1–33; complete EOF; 33 canonical logical lines, no gaps.
- Purpose: Reports a controlled entity-culling comparison with culling disabled, synchronous culling and asynchronous culling.
- Dependencies / cross-references: References renderer visibility queries, entity lists, worker scheduling and a fixed wall/armor-stand benchmark scene.
- Implementation / logic: Alternates six 10-second samples per mode and reports average FPS, 1% lows, CPU and P99 frame time.
- Findings: One synthetic scene on an old Intel HD 4000 driver; GPU timestamps were unavailable. It does not establish general performance or Android gains.
- RustChunk relevance: Useful as a benchmark design example for entity culling, but the result is hardware/workload-specific.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Implement culling only after RustChunk entity visibility correctness; measure on Mali-G52 and include crowded scenes and camera motion.
- Follow-up checks: Test occlusion correctness, async result staleness, worker contention, frame-time tails and GPU timing availability.
- RustChunk implementation mapping: Future RustChunk entity culling and renderer benchmark.
- Evidence required before adoption: Target-device repeated runs, diverse scenes, GPU/CPU timing, visibility parity and memory/worker overhead.
- Completion state: COMPLETE — 33 logical lines through EOF; pinned source SHA and 1786 byte size match; inspected, not modified; tests not executed.

### 2852. `docs/microsoft-account.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `496e4a30b1eda869cc4c4a56188c3bc5b656c4df`
- Size: 26259 bytes
- Coverage: 1–351; complete EOF; 351 canonical logical lines, no gaps.
- Purpose: Documents Microsoft device-code sign-in, Xbox/XSTS/Minecraft token exchange, ownership checks, token storage, session handoff, online login, skins and secure chat.
- Dependencies / cross-references: References launcher account module, keychain abstraction, Microsoft/Xbox/Minecraft HTTPS endpoints, Minecraft protocol handshake, session files and secure-chat signatures.
- Implementation / logic: Uses public-client OAuth device flow, entitlements/profile gate, keychain-backed refresh tokens, read-once session file and RSA/AES server login; offline names are permitted only behind an owning account in the described launcher.
- Findings: The owner gate is a product/legal policy decision, not a Minecraft protocol requirement. Offline names do not authenticate to online-mode servers; token storage and secure chat are sensitive.
- RustChunk relevance: Essential reference for correctly distinguishing online authentication, offline profiles and offline-mode server access; do not copy the Horizon owner-gate policy automatically.
- Decision: adaptable; reference; security-sensitive; version-specific
- 1.20.1 implications: Design RustChunk account modes transparently and in line with Minecraft terms; never fake Mojang session validation or claim offline profiles can join authenticated servers. Offline mode only works on servers explicitly configured for it.
- Follow-up checks: Review OAuth registration requirements, token storage on Android Keystore, token redaction, session file permissions/lifetime, signed chat, XSTS region/age errors and 1.20.1 authlib behavior.
- RustChunk implementation mapping: RustChunk account/profile manager, secure session handoff and multiplayer login state machine.
- Evidence required before adoption: Current Microsoft/Mojang developer requirements, Android secure storage, 1.20.1 authlib/protocol oracle, online/offline integration tests and compliance review.
- Completion state: COMPLETE — 351 logical lines through EOF; pinned source SHA and 26216 byte size match; inspected, not modified; tests not executed.

### 2853. `docs/mod-browser.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e69d8dd77c393c9295c3f9141e42a6b8a784964e`
- Size: 17834 bytes
- Coverage: 1–215; complete EOF; 215 canonical logical lines, no gaps.
- Purpose: Describes a mod browser that searches Modrinth and Horizon's registry, checks compatibility, installs packs and verifies downloads.
- Dependencies / cross-references: References launcher browser, Modrinth API, content/native Horizon mod formats, modpack importer, file checksums and game-side mod registry.
- Implementation / logic: Downloads only from allowed HTTPS hosts, verifies SHA-512/SHA-1, inspects JAR metadata/classes and classifies Java mods by how much data can be carried into Horizon-native packages.
- Findings: Horizon does not run Java mod code; its 'Works/Partly/Doesn't run' verdict is not Forge mod-loader compatibility. CurseForge API support is absent in this snapshot.
- RustChunk relevance: Safe download, dependency resolution and UI patterns transfer; its Java-to-native porting model is incompatible with RustChunk's goal of running Forge JARs.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Build a true Forge 1.20.1 mod browser/importer that installs original JARs and respects Forge dependencies; do not convert mods into .hmod/.hnmod or classify Java mods as 'partly run' using this method.
- Follow-up checks: Verify Modrinth/CurseForge file hashes, Forge 47.x loader metadata, required dependencies, client/server side, mod id collisions, malicious archive handling and update rollback.
- RustChunk implementation mapping: RustChunk Forge mod browser and profile mod installer.
- Evidence required before adoption: Forge 1.20.1/47.x mod metadata, dependency graph, authenticated provider APIs, file integrity tests and real modpack launch tests.
- Completion state: COMPLETE — 215 logical lines through EOF; pinned source SHA and 17830 byte size match; inspected, not modified; tests not executed.

### 2854. `docs/modpacks.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `32f12f92043e736af739a3d92cbd3b89f71cd6af`
- Size: 91487 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1243; complete EOF; 1243 canonical logical lines, no gaps.
- Purpose: Presents a full modpack inventory, carry-over, agent porting, Java oracle, coverage report and performance comparison pipeline.
- Dependencies / cross-references: References horizon-port-pack, Modrinth/CurseForge/Prism imports, NeoForge 1.21.1 oracle, Horizon content/native mod APIs, mod licenses and benchmarks.
- Implementation / logic: Normalizes pack manifests, inventories dependencies/licences, carries data/assets, ports code to native mods and checks each mod against a Java oracle; records partial coverage and measured results.
- Findings: This is a large, evolving design for replacing Java mod execution, primarily ATM10 1.21.1/NeoForge against Horizon 26.3. Its 10x claims are carefully limited and many mod features remain unported.
- RustChunk relevance: Pack manifest parsing, dependency ordering, local workspaces, licensing, coverage reporting and differential testing are useful. Its core porting strategy must not replace actual Forge support.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: RustChunk's first milestone should install and run Forge 1.20.1 JARs unchanged. Only later consider optional modpack conversion/analysis tools; keep user-owned mod files local and respect each license.
- Follow-up checks: Test .mrpack, CurseForge and launcher-instance imports, exact 1.20.1/Forge 47.x dependency graph, side restrictions, nested overrides, checksums, partial downloads, duplicate IDs and save preservation.
- RustChunk implementation mapping: RustChunk modpack importer, Forge profile resolver and coverage/compatibility report.
- Evidence required before adoption: Real Forge 1.20.1 packs, Forge installer and libraries, mod license review, original-JAR launch, multiplayer handshake and save migration fixtures.
- Completion state: COMPLETE — 1243 logical lines through EOF; pinned source SHA and 91487 byte size match; inspected, not modified; tests not executed.

### 2855. `docs/narrator.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `428751a7f096839e779b8c8c89bd686a7c9b3440`
- Size: 8022 bytes
- Coverage: 1–143; complete EOF; 143 canonical logical lines, no gaps.
- Purpose: Documents screen/chat narration, narrator modes, keyboard toggle and platform text-to-speech backends.
- Dependencies / cross-references: References narrator state machine, screen widget narration collection, translation keys, UI focus and OS speech APIs.
- Implementation / logic: Collects narration while drawing widgets, then speaks on a dedicated speech thread; keeps recent messages and degrades gracefully when TTS is unavailable.
- Findings: Backends listed are Windows SAPI, macOS AppKit and Linux flite; no Android TTS backend is documented. Some screen narration differs from vanilla.
- RustChunk relevance: Accessibility concepts and nonblocking speech-thread design are valuable for Android launcher/game UI.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Consider Android TextToSpeech as a future accessibility feature, with lifecycle-safe initialization and no frame-thread blocking.
- Follow-up checks: Test Android TTS availability, language/voice selection, focus navigation, touch and controller narration, screen transitions and interruption behavior.
- RustChunk implementation mapping: Optional RustChunk accessibility/narration subsystem.
- Evidence required before adoption: Android TTS integration, 1.20.1 narration strings, TalkBack coexistence, accessibility review and device tests.
- Completion state: COMPLETE — 143 logical lines through EOF; pinned source SHA and 8018 byte size match; inspected, not modified; tests not executed.

### 2856. `docs/native-hooks-neoforge.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9bcea3c9ef263fce8c7f7a677929618b72d1d375`
- Size: 8249 bytes
- Coverage: 1–132; complete EOF; 132 canonical logical lines, no gaps.
- Purpose: Proposes a stable hook catalogue modelled on NeoForge 26.3 event names, side, cancellation and payload semantics.
- Dependencies / cross-references: References native-hooks.json, horizon-mixins, vanilla call sites and NeoForge public event documentation.
- Implementation / logic: Defines hook naming/versioning, client/server/both sides, cancellation/result parameters and future capability hooks; explicitly says the proposal is not built.
- Findings: Proposal targets NeoForge 26.3 and is not evidence of Forge 47.x event names or actual implementation. Native Horizon hooks are not the Forge event bus.
- RustChunk relevance: A stable hook contract and explicit side/cancellation semantics are useful when designing compatibility bridges.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: For RustChunk, use actual Forge 47.x event bus and coremod/mixin semantics as applicable; do not transplant 26.3 hook names or assume Forge 1.20.1 APIs match.
- Follow-up checks: Map each required Forge 47.x event to the exact 1.20.1 vanilla method, verify event ordering, cancellation, side and mutable state with tests.
- RustChunk implementation mapping: RustChunk Forge integration bridge and mod event lifecycle documentation.
- Evidence required before adoption: Forge 47.x source/Javadocs, 1.20.1 call-site probes, real mod event tests and stable API compatibility policy.
- Completion state: COMPLETE — 132 logical lines through EOF; pinned source SHA and 8249 byte size match; inspected, not modified; tests not executed.

### 2857. `docs/noise-aquifer-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a68f18f2d915f3120df3c5c89a7a2dcce9cc2517`
- Size: 10171 bytes
- Coverage: 1–64; complete EOF; 64 canonical logical lines, no gaps.
- Purpose: Reports bulk density/noise generation optimizations, whole-terrain timing and worker scaling.
- Dependencies / cross-references: References DensityCompiler, Perlin bulk kernels, Beardifier, aquifer/surface work, chunk generation benchmarks and pinned Java parity.
- Implementation / logic: Dispatches unary/binary operations once per bulk buffer, reuses shared kernels and Perlin X-column permutations; includes alternating baseline/candidate runs and extensive differential tests.
- Findings: Measurements are Linux Ryzen 5 5600X desktop-specific, 26.3-specific and workload-scoped; whole terrain gains are distinct from individual noise-stage gains.
- RustChunk relevance: Strong optimization methodology and candidate algorithms for worldgen, but no evidence of Android gains or 1.20.1 parity.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Port only after 1.20.1 density/noise correctness; benchmark single and multi-worker paths on ARM64 and keep correctness gates separate from performance.
- Follow-up checks: Verify floating-point bit behavior, NaN/signed-zero, random draw order, bulk buffer tails, worker scaling, memory and thermal throttling.
- RustChunk implementation mapping: RustChunk density/noise bulk sampler and worldgen benchmark suite.
- Evidence required before adoption: 1.20.1 Java oracle, target-device repeatable runs, complete chunk parity, raw samples and per-stage CPU/GPU attribution.
- Completion state: COMPLETE — 64 logical lines through EOF; pinned source SHA and 10086 byte size match; inspected, not modified; tests not executed.

### 2858. `docs/noise-generation.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `055c322023a1625307258aa3a50acc032cf416f1`
- Size: 5699 bytes
- Coverage: 1–96; complete EOF; 96 canonical logical lines, no gaps.
- Purpose: Summarizes implemented Perlin/simplex noise, octave stacks, random streams and density volumes.
- Dependencies / cross-references: References horizon-core noise modules, Java client/server oracle suites, density registries and world-generation integration.
- Implementation / logic: Implements deterministic scalar/bulk samplers, derivative accumulation, normal-noise construction and legacy Nether biome initialization with exact RNG order.
- Findings: Targets 26.3 and explicitly states complete vanilla chunks, aquifers, surface rules, features and full gameplay remain incomplete.
- RustChunk relevance: Core worldgen math is relevant, but the target-version data and random streams must be rebuilt.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Reimplement 1.20.1 noise and random streams from the target JAR, keeping exact Java numeric and RNG semantics before performance optimization.
- Follow-up checks: Test both legacy/xoroshiro streams, permutation initialization, octave gaps, coordinate wrap, bulk/scalar differences, NaN payload policy and seeded chunk output.
- RustChunk implementation mapping: RustChunk worldgen noise and random modules.
- Evidence required before adoption: 1.20.1 source/JAR oracle, registry JSON, seeded numerical fixtures, Forge datapack tests and complete chunk-generation parity.
- Completion state: COMPLETE — 96 logical lines through EOF; pinned source SHA and 5699 byte size match; inspected, not modified; tests not executed.

### 2859. `docs/owners.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c678915912633c91fd7093dab8c2d0484b0070ef`
- Size: 4161 bytes
- Coverage: 1–88; complete EOF; 88 canonical logical lines, no gaps.
- Purpose: Maps vanilla server.properties fields to Horizon's server.toml configuration.
- Dependencies / cross-references: References horizon-server initialization, vanilla parser, server config, world folder and management settings.
- Implementation / logic: Imports settings, preserves vanilla world/player files and writes server.properties from a validated TOML model before startup.
- Findings: Server-hosting configuration only; it does not describe client profile or Forge launcher configuration.
- RustChunk relevance: No direct value for the RustChunk Android launcher; generic config migration/validation is reusable.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not implement a server configuration editor in the client milestone.
- Follow-up checks: If a separate server tool is ever planned, preserve unknown vanilla properties and round-trip the target version's parser.
- RustChunk implementation mapping: No current client implementation mapping.
- Evidence required before adoption: Explicit server scope, 1.20.1 server.properties schema, import/export round-trip tests.
- Completion state: COMPLETE — 88 logical lines through EOF; pinned source SHA and 4161 byte size match; inspected, not modified; tests not executed.

### 2860. `docs/parity/blocks.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4c5bbfb7663d631f98cae720db048d3d34c3dfc3`
- Size: 9606 bytes
- Coverage: 1–182; complete EOF; 182 canonical logical lines, no gaps.
- Purpose: Reports parity scope for random/scheduled block ticks, growth, TNT, explosions, hoppers, dispensers, comparators and weather.
- Dependencies / cross-references: References horizon-world level modules, block behaviour callbacks, block entities, worldgen features and Java parity harnesses.
- Implementation / logic: Implements many block behaviors with exact tick ordering, RNG streams, block-entity lifecycle and container interactions; lists remaining gaps and tests.
- Findings: 26.3 behavior and block set differ from 1.20.1; several known gaps remain (random stream propagation, explosion exposure, entities, save state).
- RustChunk relevance: High relevance to integrated-server/world simulation and Forge mod interactions, but not directly portable.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Use as a checklist of vanilla behavior domains; rebuild from 1.20.1 and expose the right Forge event hooks instead of copying 26.3 code.
- Follow-up checks: Check random tick eligibility, scheduled ticks, block entity removal, TNT persistence, hopper cooldowns, explosion exposure and all Forge-added blocks.
- RustChunk implementation mapping: RustChunk world simulation/block behavior and Forge integration tests.
- Evidence required before adoption: 1.20.1 source/JAR parity, Forge mod block fixtures, save/reload, multiplayer server tests and deterministic seed traces.
- Completion state: COMPLETE — 182 logical lines through EOF; pinned source SHA and 9606 byte size match; inspected, not modified; tests not executed.

### 2861. `docs/parity/meshing.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a0844dcf9f20611a48feaa39587c9fee99b9c6de`
- Size: 9287 bytes
- Coverage: 1–154; complete EOF; 154 canonical logical lines, no gaps.
- Purpose: Describes exact section-mesh comparison against the vanilla client, including fluids, lighting, tints and per-state facts.
- Dependencies / cross-references: References horizon-client-render mesher, vanilla ModelBlockRenderer/FluidRenderer oracle, block facts and biome blend fixtures.
- Implementation / logic: Matches quads as multisets and compares positions, UVs, colors and light; reports 841,775 matching quads across 94 scenes.
- Findings: The corpus is pinned to 26.3 and excludes GPU pixels, translucent sorting, block entities, animated frames and some culling modes.
- RustChunk relevance: Strong renderer-validation pattern for RustChunk's chunk mesh, fluids, lighting and resource-pack rendering.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Build a 1.20.1 renderer parity corpus before tuning; ensure Forge-added blocks and resource packs are represented and compare frame output on Android separately.
- Follow-up checks: Verify quad order/tolerance, alpha behavior, shader output, missing-neighbor borders, fluid faces, biome tint, translucent ordering and actual GPU pixels.
- RustChunk implementation mapping: RustChunk section mesher and renderer differential harness.
- Evidence required before adoption: 1.20.1 vanilla renderer oracle, Forge block models, ARM64 GPU captures, resource-pack corpus and visual regression fixtures.
- Completion state: COMPLETE — 154 logical lines through EOF; pinned source SHA and 9287 byte size match; inspected, not modified; tests not executed.

### 2862. `docs/performance/lighting-worker-results.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7c52b1276101db390d2ebf284e99dddadc52cc84`
- Size: 3134 bytes
- Coverage: 1–56; complete EOF; 56 canonical logical lines, no gaps.
- Purpose: Reports reduced worker-result copying when packaging and applying chunk-lighting results.
- Dependencies / cross-references: References light_chunk, apply_lighting, Section storage, Arc::make_mut, streaming stale-sequence guards and replayed edits.
- Implementation / logic: Returns only light buffers and moves them into current sections, preserving block/biome edits and snapshots; tests exact state before timing.
- Findings: Benchmarks are synthetic 24-section fixtures on Ryzen 5 5600X; excludes propagation, streaming, remeshing and whole-game FPS.
- RustChunk relevance: Potentially useful for RustChunk chunk/light worker ownership and reducing copies.
- Decision: adaptable; reference
- 1.20.1 implications: Consider move-based result ownership and copy-on-write only after profiling the actual Android chunk pipeline.
- Follow-up checks: Test stale result rejection, edits during worker execution, snapshot lifetime, light buffer reuse, allocation count and chunk unload races.
- RustChunk implementation mapping: RustChunk lighting worker result/application path.
- Evidence required before adoption: Android ARM64 allocation/frame-time profiles, concurrency tests, exact light parity and realistic loaded chunks.
- Completion state: COMPLETE — 56 logical lines through EOF; pinned source SHA and 3134 byte size match; inspected, not modified; tests not executed.

### 2863. `docs/performance/paletted-container-access-results.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `08783d2904a50877a8dfe969f3580791581798be`
- Size: 3865 bytes
- Coverage: 1–73; complete EOF; 73 canonical logical lines, no gaps.
- Purpose: Reports hot-path optimizations to block-section palette get/set and bulk unpack operations.
- Dependencies / cross-references: References BlockSection, PalettedContainer, PackedStorage, palette growth/free-list and NBT serialization.
- Implementation / logic: Avoids redundant palette lookups and dispatches palette variants outside per-cell loops while preserving encoded palette and NBT output.
- Findings: Microbenchmarks are synthetic and desktop-specific; improvements are local operation speedups, not end-to-end frame gains.
- RustChunk relevance: Directly relevant to chunk section storage and block lookup performance.
- Decision: adaptable; reference
- 1.20.1 implications: Use the same principle of validating indices once at the trusted section boundary, while retaining safe public APIs and 1.20.1-compatible palette formats.
- Follow-up checks: Test palette growth/collapse, free-slot reuse, global/local palette transitions, NBT roundtrip, malformed ids and real chunk workloads.
- RustChunk implementation mapping: RustChunk packed palette storage and chunk section get/set.
- Evidence required before adoption: Differential storage fixtures, Android benchmarks, save/network format compatibility and memory bounds.
- Completion state: COMPLETE — 73 logical lines through EOF; pinned source SHA and 3865 byte size match; inspected, not modified; tests not executed.

### 2864. `docs/performance/server-vs-pumpkin.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `300aaeee10afd595497cf0f7a19228552172c7c9`
- Size: 5966 bytes
- Coverage: 1–90; complete EOF; 90 canonical logical lines, no gaps.
- Purpose: Compares Horizon's dedicated server performance with Pumpkin under identical 26.3 protocol workloads.
- Dependencies / cross-references: References horizon-server, Pumpkin black-box binary, bot harness, tick query, chunk throughput and CPU/RSS measurements.
- Implementation / logic: Uses alternating server runs, same bots/seed/settings, multiple player counts and reports MSPT, P99, join latency and chunks per second.
- Findings: Server-only comparison on a remote Linux EPYC host; both runs use offline mode and protocol 777, not client rendering or Forge 1.20.1.
- RustChunk relevance: No direct launcher/client implementation value; the benchmark discipline may inform future server compatibility testing.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not add a dedicated server engine to RustChunk's Android client scope; focus on joining external servers correctly.
- Follow-up checks: If future server work is authorized, compare only identical protocol/version, world, bot load, online-mode state and CPU constraints.
- RustChunk implementation mapping: No current client mapping; optional future server benchmark methodology.
- Evidence required before adoption: Explicit server scope, Forge 1.20.1 server tests, controlled host resources and reproducible raw data.
- Completion state: COMPLETE — 90 logical lines through EOF; pinned source SHA and 5936 byte size match; inspected, not modified; tests not executed.

### 2865. `docs/performance/startup-stage-3.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b3258aada196364f1d859af81a2bed495f7bc3d1`
- Size: 8361 bytes
- Coverage: 1–61; complete EOF; 61 canonical logical lines, no gaps.
- Purpose: Documents progressive startup, core/rest resource loading, interactive loading menu and deferred world entry.
- Dependencies / cross-references: References resource worker, renderer creation, loading UI, event loop, pack overlays, renderer facts and memory measurements.
- Implementation / logic: Publishes core resources before simulation tail, queues Play/Connect, keeps resize/close responsive and resumes after worker completion.
- Findings: Horizon 26.3 desktop implementation; measurements were under concurrent compilation and show a significant progressive-title RSS increase requiring investigation.
- RustChunk relevance: Relevant to Android startup responsiveness and not blocking the UI while resources/chunks load.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use a staged startup state machine and loading progress on Android, but keep GPU work on the render thread and bound background memory.
- Follow-up checks: Test cancellation, lifecycle pause/resume, process death, resource generation replacement, memory peaks and no UI stalls on Galaxy A22 class devices.
- RustChunk implementation mapping: RustChunk startup coordinator, resource loader and loading screen.
- Evidence required before adoption: Android lifecycle tests, memory/thermal profiles, responsive loading UI, failure/cancel behavior and actual device timings.
- Completion state: COMPLETE — 61 logical lines through EOF; pinned source SHA and 8359 byte size match; inspected, not modified; tests not executed.

### 2866. `docs/play-with-friends.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ae80f3f96b1ab579963fec2faec75892b3118c3c`
- Size: 15549 bytes
- Coverage: 1–227; complete EOF; 227 canonical logical lines, no gaps.
- Purpose: Describes direct encrypted tunnels and optional relays for hosting a single-player world to friends.
- Dependencies / cross-references: References integrated server LAN listener, NAT-PMP/PCP/UPnP, STUN, SPAKE2, Ed25519, ChaCha20-Poly1305, relay protocol and invite codes.
- Implementation / logic: Opens LAN and tunnels unchanged vanilla protocol bytes through encrypted direct/relay connections; manages router mappings and privacy disclosures.
- Findings: Adds network exposure and router automation; protocol is Horizon-specific. This is for hosting worlds, not joining Aternos or ordinary servers.
- RustChunk relevance: Optional social hosting feature only; not required for RustChunk to connect to external servers.
- Decision: unnecessary; optional/future; security-sensitive
- 1.20.1 implications: Do not implement in the baseline. Aternos connectivity needs normal client TCP login, not NAT traversal or relay hosting.
- Follow-up checks: If revisited, audit SPAKE2 framing, replay/order handling, port mapping cleanup, relay metadata, rate limits and consent before opening inbound ports.
- RustChunk implementation mapping: No current implementation mapping; future optional LAN-sharing module.
- Evidence required before adoption: Independent security review, adversarial protocol tests, router compatibility, explicit opt-in and Android network policy.
- Completion state: COMPLETE — 227 logical lines through EOF; pinned source SHA and 15109 byte size match; inspected, not modified; tests not executed.

### 2867. `docs/portal-scan-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8156916ed8adfdbc95e2b7a65affd2cf2c271fb2`
- Size: 4402 bytes
- Coverage: 1–81; complete EOF; 81 canonical logical lines, no gaps.
- Purpose: Benchmarks a resident per-chunk section mask cache for nether-portal reach scans.
- Dependencies / cross-references: References entity portal collision checks, resident chunk sections, palettes, weak cache handles and chunk invalidation.
- Implementation / logic: Caches whether sections may contain portal blocks, compares cached answers to rescans and measures steady/cold/reload/edit workloads.
- Findings: Some edit-all/cold paths are dramatically slower due full-chunk rescans; benchmark is synthetic desktop and the cache adds bookkeeping.
- RustChunk relevance: Potential future optimization for repeated portal/entity collision queries, but not a current bottleneck without profiling.
- Decision: adaptable; reference
- 1.20.1 implications: Do not introduce broad chunk caches without measuring edit invalidation; if needed, use narrow per-chunk masks with explicit lifetime.
- Follow-up checks: Test cache invalidation on every block/light/resource update, chunk unload/reload, cold path and high block-edit frequency.
- RustChunk implementation mapping: Optional RustChunk portal/entity collision scan cache.
- Evidence required before adoption: Android measurements on realistic worlds, invalidation correctness and bounded cache memory.
- Completion state: COMPLETE — 81 logical lines through EOF; pinned source SHA and 4402 byte size match; inspected, not modified; tests not executed.

### 2868. `docs/pregeneration.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `13ceba610cd98564d5f4c534327fc3825e6ac8b3`
- Size: 3405 bytes
- Coverage: 1–50; complete EOF; 50 canonical logical lines, no gaps.
- Purpose: Describes ahead-of-time chunk generation over a square/circle with bounded streaming bands.
- Dependencies / cross-references: References Pregen, streaming Band, decoration order, lighting, postprocessing and save scheduling.
- Implementation / logic: Generates chunks in strips/bands, holds bounded in-flight memory and preserves deterministic decoration order independent of worker timing.
- Findings: Primarily a server/admin command, not a client render-distance solution; generated data and scheduler are Horizon 26.3-specific.
- RustChunk relevance: Bounded chunk scheduling and deterministic worker ordering are useful ideas for future worldgen; not a fix for client chunk arrival.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Keep pregeneration out of initial Android client scope; prioritize receiving/loading nearby chunks without visible holes.
- Follow-up checks: Check generation order, neighbor radius, save durability, cancellation/restart semantics and memory caps if worldgen is later implemented.
- RustChunk implementation mapping: Future RustChunk worldgen task scheduler only.
- Evidence required before adoption: 1.20.1 seeded chunk parity, bounded Android RAM, cancellation safety and realistic performance tests.
- Completion state: COMPLETE — 50 logical lines through EOF; pinned source SHA and 3405 byte size match; inspected, not modified; tests not executed.

### 2869. `docs/prepare-performance.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ec1df683d57c1907afeda441971c8218734d9a00`
- Size: 3253 bytes
- Coverage: 1–46; complete EOF; 46 canonical logical lines, no gaps.
- Purpose: Discusses resource preparation I/O bottlenecks, asset-index reuse, file cloning and immutable resource stores.
- Dependencies / cross-references: References horizon-prepare, Mojang asset index, exporters, ZIP CRC, FICLONE/clonefile, staging and cache markers.
- Implementation / logic: Shares parsed asset metadata per preparation, validates destination bytes, stages files before publishing and considers immutable cross-release stores.
- Findings: Clone support varies by filesystem; rename/process interruption is not a fully transactional publish. Proposed shared store is not implemented.
- RustChunk relevance: Useful for reducing repeated downloads/copies and protecting installed resources.
- Decision: adaptable; reference
- 1.20.1 implications: Design resumable, hash-verified resource preparation for 1.20.1/Forge and use Android-safe copy/rename behavior; do not assume desktop reflink APIs exist.
- Follow-up checks: Test interrupted downloads, corrupt cache repair, atomic replacement, duplicate asset references, disk exhaustion and concurrent launcher instances.
- RustChunk implementation mapping: RustChunk asset/library installer and version cache.
- Evidence required before adoption: Android scoped storage behavior, filesystem capability detection, pinned artifact hashes and crash-recovery tests.
- Completion state: COMPLETE — 46 logical lines through EOF; pinned source SHA and 3249 byte size match; inspected, not modified; tests not executed.

### 2870. `docs/profiling.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1bd7732edf3557d221f96b27e5e3cbe128452416`
- Size: 3922 bytes
- Coverage: 1–73; complete EOF; 73 canonical logical lines, no gaps.
- Purpose: Explains Java Flight Recorder profiling and careful interpretation of allocation, execution, GC and heap metrics.
- Dependencies / cross-references: References JDK 25 jcmd/JFR, custom JFR JSON summary tool, private local recordings and separate benchmark runs.
- Implementation / logic: Captures a bounded profile separately from timing, streams selected events and distinguishes observed allocation counters from sample counts and RSS.
- Findings: JFR/Java 25 desktop method is not directly available for the Rust Android client; raw recordings may contain private environment data.
- RustChunk relevance: The separation of profiling and timing runs and careful metric semantics are transferable.
- Decision: adaptable; reference
- 1.20.1 implications: Use Rust/Android profiling tools rather than JFR; retain private traces locally and distinguish allocated bytes, resident memory, CPU samples and GPU time.
- Follow-up checks: Check profiler overhead, thread spans, sampling bias, thermal state, memory metrics and privacy scrubbing.
- RustChunk implementation mapping: RustChunk profiling and performance investigation workflow.
- Evidence required before adoption: Android Studio/Perfetto or Rust profiling captures, reproducible workloads, privacy review and independent timing runs.
- Completion state: COMPLETE — 73 logical lines through EOF; pinned source SHA and 3922 byte size match; inspected, not modified; tests not executed.

### 2871. `docs/random-tick-eligibility-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `adc3acfe04eb1f0023c756b4ff9ae94ce1608ab9`
- Size: 3708 bytes
- Coverage: 1–68; complete EOF; 68 canonical logical lines, no gaps.
- Purpose: Benchmarks caching random-tick eligibility masks for resident chunk sections.
- Dependencies / cross-references: References random tick section selection, block palettes, weak chunk handles, edit invalidation and worker scheduling.
- Implementation / logic: Caches section masks and compares cached eligibility against original scanning under steady, cold, reload and block-edit workloads.
- Findings: Results are synthetic Ryzen/Linux measurements; no whole-game FPS gain is claimed and cache overhead/invalidations remain relevant.
- RustChunk relevance: Potentially useful for world simulation CPU reduction once RustChunk supports random ticks.
- Decision: adaptable; reference
- 1.20.1 implications: Profile actual 1.20.1 tick selection before adopting; preserve exact vanilla random tick eligibility and ordering.
- Follow-up checks: Test palette transitions, block edits, lighting changes, chunk unload, resource reload, cache memory and Android thermal behavior.
- RustChunk implementation mapping: Future RustChunk random tick scheduler cache.
- Evidence required before adoption: 1.20.1 parity oracle, realistic chunk workloads, edit-heavy tests and ARM64 device benchmark.
- Completion state: COMPLETE — 68 logical lines through EOF; pinned source SHA and 3708 byte size match; inspected, not modified; tests not executed.

### 2872. `docs/release-flow.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04241e8fc5125f2422050f058f2483acfb7a1913`
- Size: 44061 bytes
- Coverage: 1–400, 401–571; complete EOF; 571 canonical logical lines, no gaps.
- Purpose: Documents CI checks, parity gates, branch review, signed release builds, Nightly promotion, trusted reviewers and artifact cleanup.
- Dependencies / cross-references: References GitHub Actions workflows, release_train.py, parity harnesses, signing keys, CODEOWNERS and release assets.
- Implementation / logic: Uses staged CI, main-only signing jobs, isolated untrusted builds, signed SHA256SUMS, release blockers and controlled Nightly source tags.
- Findings: All runner identities, secrets, trusted reviewers, release channels and branch rules are specific to Horizon and change over time.
- RustChunk relevance: Release governance, supply-chain separation and artifact signing are useful for RustChunk once builds are reproducible.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Adopt signed artifacts, least-privilege CI and parity gates; do not copy Horizon's people, secrets, runner paths or release train configuration.
- Follow-up checks: Audit workflow permissions, fork PR isolation, signing key custody, reproducible source pins, checksum verification and Android APK signing.
- RustChunk implementation mapping: RustChunk GitHub Actions, release process and signed APK distribution.
- Evidence required before adoption: RustChunk-specific CODEOWNERS, threat model, isolated signing workflow, reproducible APK, key rotation and clean-room release verification.
- Completion state: COMPLETE — 571 logical lines through EOF; pinned source SHA and 44042 byte size match; inspected, not modified; tests not executed.

### 2873. `docs/research-lab.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `827246fd73a03a6cbdc06bd59c0d621ecb8326a1`
- Size: 5145 bytes
- Coverage: 1–84; complete EOF; 84 canonical logical lines, no gaps.
- Purpose: Describes a local Minecraft JAR inspection/decompilation workspace and dependency-free Rust research crates.
- Dependencies / cross-references: References research-lock.json, lab.py, decompiler, .local game assets, Rust parity runners and world-data/geometry suites.
- Implementation / logic: Pins official JARs and decompiler, keeps proprietary artifacts gitignored, generates inventories and compares Rust outputs to actual Java methods.
- Findings: The snapshot is 26.3; decompilation coverage does not prove semantic equivalence or recompilability. Private game data must not enter Git.
- RustChunk relevance: Strongly relevant as a reproducible, source-pinned reverse-engineering and parity workflow for RustChunk.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Build a separate 1.20.1/Forge 47.x research workspace with official artifact hashes; keep local JARs, extracted sources and test worlds out of Git.
- Follow-up checks: Verify source/bytecode mapping, decompiler version, artifact checksums, license restrictions, fixture provenance and parity scope.
- RustChunk implementation mapping: RustChunk research tooling, version pin and Java oracle harness.
- Evidence required before adoption: Pinned 1.20.1 client/server/Forge artifacts, clean clone reproduction, private-data checks and exact differential tests.
- Completion state: COMPLETE — 84 logical lines through EOF; pinned source SHA and 5145 byte size match; inspected, not modified; tests not executed.

### 2874. `docs/resource-packs.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bdabadbaeebf0788c62e329a165cedd32691b6de`
- Size: 15498 bytes
- Coverage: 1–105; complete EOF; 105 canonical logical lines, no gaps.
- Purpose: Lists resource-pack features and parity status for metadata, models, textures, fonts, sounds, GUI, shaders, entity models and connected textures.
- Dependencies / cross-references: References pack overlay builder, reload listeners, resource loaders, OptiFine/CIT compatibility, renderer and many parity harnesses.
- Implementation / logic: Builds ordered overlay roots and reloads all asset/render subsystems; documents feature-by-feature works/partial/no support with evidence.
- Findings: Targets 26.3; shader formats and many OptiFine/EMF extensions are partial or absent; 1.20.1 resource pack format differs.
- RustChunk relevance: Important reference for resource-pack compatibility and correct renderer reload behavior.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Start with vanilla 1.20.1 resource-pack semantics and Forge resource reload hooks; expand OptiFine features only after vanilla parity.
- Follow-up checks: Test pack format compatibility, overlays/filters, blockstate/model resolution, atlas/mipmap, fonts, sounds, animations, entity textures and resource reload lifecycle.
- RustChunk implementation mapping: RustChunk resource-pack manager and renderer asset pipeline.
- Evidence required before adoption: 1.20.1 pack-format specs, Forge reload listeners, representative packs, pixel/mesh parity and malformed ZIP tests.
- Completion state: COMPLETE — 105 logical lines through EOF; pinned source SHA and 15496 byte size match; inspected, not modified; tests not executed.

### 2875. `docs/reuse-pack-overlays.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `53e8bbc3a2e22644f2dea589015b7c72eb6fcfc4`
- Size: 4762 bytes
- Coverage: 1–46; complete EOF; 46 canonical logical lines, no gaps.
- Purpose: Describes persistent caching of completed resource-pack overlays using versioned markers and source identity stamps.
- Dependencies / cross-references: References packs::build_overlay, filesystem metadata, symlinks, ZIP source stamps, lock files and overlay invalidation.
- Implementation / logic: Uses marker identity and OS lock, validates source topology and materialized inputs, publishes marker only after successful stable build and reuses unchanged output.
- Findings: Unix metadata fast paths differ from non-Unix content hashing; external output edits are not detected and crash/rename publication is not fully transactional.
- RustChunk relevance: Useful for reducing resource-pack reload/startup cost after correctness and cache invalidation are established.
- Decision: adaptable; reference
- 1.20.1 implications: Use a simpler hash-verified cache for RustChunk first; only adopt metadata-based shortcuts after Windows/Android filesystem semantics are proven.
- Follow-up checks: Test same-size/mtime edits, symlink aliases, ZIP replacement, pack order, crash recovery, concurrent rebuild and stale output tampering.
- RustChunk implementation mapping: RustChunk resource-pack overlay cache.
- Evidence required before adoption: Android/Windows filesystem behavior, cryptographic input fingerprints, atomic publication and invalidation regression tests.
- Completion state: COMPLETE — 46 logical lines through EOF; pinned source SHA and 4758 byte size match; inspected, not modified; tests not executed.

### 2876. `docs/rust-parity.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4db6a8a91adb32ae9f1354c60b36a27e0e149b4`
- Size: 106605 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1595; complete EOF; 1595 canonical logical lines, no gaps.
- Purpose: Large running inventory of Rust ports and differential tests against the actual vanilla client/server JARs.
- Dependencies / cross-references: Cross-references nearly every world, gameplay, network, UI, rendering and persistence subsystem plus Python oracle tools and fixture reports.
- Implementation / logic: Describes scoped adapters that execute native Java methods and compare Rust traces, state, NBT, packets and GUI calls; records exact covered cases and pending gaps.
- Findings: The pinned snapshot is 26.3; many comparisons are fixture-scoped, controlled or incomplete. Passing harnesses are not proof of a playable client, Forge support or full gameplay parity.
- RustChunk relevance: Very valuable as a parity-ledger model and a map of subsystem interactions; the actual APIs/data are version-specific.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Use its evidence discipline and target-JAR oracle approach, but rebuild each contract for 1.20.1/Forge 47.x and avoid assuming the 26.3 Rust code ports directly.
- Follow-up checks: Prioritize login/registry, chunks, block states, worldgen, renderer, player movement, inventory, entities, saves and mod hooks; distinguish fixture coverage from end-to-end runtime tests.
- RustChunk implementation mapping: RustChunk 1.20.1 compatibility matrix, Java oracle harnesses and per-subsystem acceptance gates.
- Evidence required before adoption: Pinned 1.20.1 client/server/Forge artifacts, complete fixture corpus, deterministic parity outputs, real Forge mods and actual multiplayer/device tests.
- Completion state: COMPLETE — 1595 logical lines through EOF; pinned source SHA and 106605 byte size match; inspected, not modified; tests not executed.

### 2877. `docs/save-batch-entities.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `335867c84b0f431688c51ab166179aa529c2084e`
- Size: 4927 bytes
- Coverage: 1–90; complete EOF; 90 canonical logical lines, no gaps.
- Purpose: Reports batching block-entity/name collection for multi-chunk save snapshots.
- Dependencies / cross-references: References Game::with_entities, save::chunks_with_block_entities, chunk maps, async save queues and exact NBT serialization.
- Implementation / logic: Groups borrowed entries for requested chunks in one pass, clones selected payloads chunk-by-chunk and retains original map/chunk state.
- Findings: Synthetic populated fixtures on Ryzen/Linux; excludes writer queue, compression, region I/O and full save latency. Improvement is only 5–17% in multi-chunk snapshot stage.
- RustChunk relevance: Relevant to chunk unload/save performance and avoiding repeated whole-map scans.
- Decision: adaptable; reference
- 1.20.1 implications: Use only if RustChunk has repeated per-chunk scans during batch saves; keep save snapshots isolated from unload cleanup.
- Follow-up checks: Test duplicate chunks, negative coordinates, entity ordering, snapshot independence, NBT byte equality and asynchronous save failures.
- RustChunk implementation mapping: RustChunk save pipeline and block-entity batch snapshot helper.
- Evidence required before adoption: Real 1.20.1 world saves, Android storage, crash/reload tests, full NBT equality and realistic entity-heavy worlds.
- Completion state: COMPLETE — 90 logical lines through EOF; pinned source SHA and 4927 byte size match; inspected, not modified; tests not executed.

### 2878. `docs/scheduled-ticks.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e6faca658b9a55a06618e35e89cef749b70cc7a5`
- Size: 7160 bytes
- Coverage: 1–136; complete EOF; 136 canonical logical lines, no gaps.
- Purpose: Documents block/fluid scheduled tick queues, ordering, persistence and callback lifecycle.
- Dependencies / cross-references: References TickPriority, ScheduledTick, SavedTick, ChunkTicks, LevelTicks and vanilla tick codecs.
- Implementation / logic: Preserves first-request deduplication, chunk-local heap order, cross-chunk priority competition, callback deferral, area operations and saved tick packing.
- Findings: Targets 26.3; actual block/fluid behaviors, random ticks, neighbor updates, redstone and live world integration remain unimplemented. API adds Rust invariant errors not present in Java.
- RustChunk relevance: Core simulation and chunk persistence are relevant; exact scheduler ordering affects Forge gameplay.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild 1.20.1 scheduled tick semantics and callback routing; preserve Java comparator, RNG/order and saved NBT semantics before optimizing.
- Follow-up checks: Test duplicate requests, tie ordering across chunks, callback rescheduling, disabled/unloaded chunks, signed time overflow, saved tick codec and Forge block/fluid behavior.
- RustChunk implementation mapping: RustChunk block/fluid tick scheduler and chunk save/load adapters.
- Evidence required before adoption: 1.20.1 LevelTicks/ChunkTicks source oracle, Forge block/fluid callbacks, save/reload fixtures and deterministic world tests.
- Completion state: COMPLETE — 136 logical lines through EOF; pinned source SHA and 7160 byte size match; inspected, not modified; tests not executed.

### 2879. `docs/security.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `886f149d50e47388bc22cb4dac7cfa2f5a4587a5`
- Size: 8734 bytes
- Coverage: 1–109; complete EOF; 109 canonical logical lines, no gaps.
- Purpose: Explains source-control, agent, CI, signed update and release security controls.
- Dependencies / cross-references: References GitHub Actions, required checks, release environment, CODEOWNERS, GitHub App, Ed25519 signatures and update verification.
- Implementation / logic: Separates untrusted issue/report data from instructions, makes required checks unconditional, isolates signing keys and verifies signed checksums before updates.
- Findings: Several safeguards are marked not yet live in the snapshot; signed releases prove provenance, not benign code. First install still trusts GitHub checksum rather than signature.
- RustChunk relevance: Highly relevant to safe launcher downloads, mod installation, APK release and future automation.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Adopt least-privilege CI, immutable source pins, signed APK/update manifests, checksum validation and untrusted-content handling; do not copy Horizon's agent assumptions.
- Follow-up checks: Verify workflow permissions, fork PR isolation, signing key custody, key rotation, downgrade/rollback policy, initial install trust and mod archive validation.
- RustChunk implementation mapping: RustChunk CI/release pipeline, update verifier and mod download trust boundary.
- Evidence required before adoption: Threat model, signed artifacts, independent verification, protected release environment, malicious archive tests and key recovery procedure.
- Completion state: COMPLETE — 109 logical lines through EOF; pinned source SHA and 8734 byte size match; inspected, not modified; tests not executed.

### 2880. `docs/server-levels.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bba8048b33fc039e67bdd191e4fdd44176e8370f`
- Size: 6268 bytes
- Coverage: 1–95; complete EOF; 95 canonical logical lines, no gaps.
- Purpose: Describes one ServerLevel per dimension and switching level state through a shared Game instance.
- Dependencies / cross-references: References MinecraftServer.levels, Game fields, parked levels, server tick order, dimension travel, save folders and LAN hub.
- Implementation / logic: Swaps per-level state into one active Game while preserving shared server state and ticking only levels with players; closes idle non-overworld levels.
- Findings: Horizon-specific integrated/dedicated server architecture, 26.3 and incomplete cross-level entity/portal behavior.
- RustChunk relevance: Useful for understanding dimension-scoped state, but not required to connect to external servers.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not add a server-side simulation architecture to RustChunk's Android client; ensure the client correctly handles server dimension changes and chunk bounds.
- Follow-up checks: Check 1.20.1 Respawn/dimension registry packets, dimension-specific height, world border, player respawn and chunk cache invalidation.
- RustChunk implementation mapping: RustChunk remote dimension transition and per-dimension client cache only.
- Evidence required before adoption: 1.20.1 protocol oracle, multi-dimension server integration and cache/resource lifecycle tests.
- Completion state: COMPLETE — 95 logical lines through EOF; pinned source SHA and 6268 byte size match; inspected, not modified; tests not executed.

### 2881. `docs/server-next.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d4f90b143af2ed5f6a5e8492d379b2200d9b9311`
- Size: 35995 bytes
- Coverage: 1–400, 401–472; complete EOF; 472 canonical logical lines, no gaps.
- Purpose: Proposes a high-performance dedicated server with parallel ticking, network I/O pools, chunk caches, memory/startup targets and a negotiated Horizon fast path.
- Dependencies / cross-references: References Horizon server, Pumpkin/vanilla black-box benchmarks, level scheduler, LAN hub, packet encoding, worker pools and custom protocol capabilities.
- Implementation / logic: Separates exact vanilla-order parallel work from opt-in region ticking; proposes per-player I/O queues, prepared login frames, cached chunk packets and negotiated chunk-zstd/entity-batch/far-terrain capabilities.
- Findings: Dedicated-server roadmap, not client launcher; Horizon fast path is proprietary and only valid when negotiated. Benchmarks are on Mac mini/desktop and use offline bots.
- RustChunk relevance: Useful for performance architecture and a critical reminder that non-vanilla packet extensions require explicit negotiation.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not implement a server engine or proprietary fast-path protocol in RustChunk baseline; interoperate with vanilla/Forge 1.20.1 packet flow and ignore unknown optional channels safely.
- Follow-up checks: Verify 1.20.1 login/config/play ordering, chunk batching, movement checks, keepalives, compression and any server brand/custom payload behavior.
- RustChunk implementation mapping: No server implementation; remote client networking and packet processing only.
- Evidence required before adoption: Actual 1.20.1/Forge server packet captures, Aternos connections, negotiated capability tests and protocol compatibility matrix.
- Completion state: COMPLETE — 472 logical lines through EOF; pinned source SHA and 35995 byte size match; inspected, not modified; tests not executed.

### 2882. `docs/server.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7b90a75cf1a59a580ef25f7bd20eb57cfdc36ed1`
- Size: 55382 bytes
- Coverage: 1–400, 401–701; complete EOF; 701 canonical logical lines, no gaps.
- Purpose: Documents Horizon's dedicated server engine, supervisor, install packages, config, CLI/TUI, web panel, backups, update and control APIs.
- Dependencies / cross-references: References horizon-server, horizon-client --dedicated, server.properties, EULA, control socket, RCON, GS4 query, management WebSocket and release tooling.
- Implementation / logic: Separates supervisor from headless game engine; imports vanilla config, manages process lifecycle, backups/rollback and exposes local authenticated APIs.
- Findings: Targets Minecraft 26.3, is server hosting software and includes Horizon-specific APIs and assumptions; not a launcher for joining Aternos.
- RustChunk relevance: Only server lifecycle/security patterns are reusable; client should not absorb server supervisor functionality.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Keep RustChunk as a client/launcher; don't bundle a server panel or supervisor unless explicitly scoped later.
- Follow-up checks: For joining servers, validate vanilla 1.20.1 login, secure chat, compression, server list ping, keepalive and Forge handshake only.
- RustChunk implementation mapping: No current implementation mapping beyond future optional local integrated-server process management.
- Evidence required before adoption: Explicit product scope, 1.20.1 client-server integration tests and separate threat model if hosting is ever added.
- Completion state: COMPLETE — 701 logical lines through EOF; pinned source SHA and 55382 byte size match; inspected, not modified; tests not executed.

### 2883. `docs/shaders.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7174220c3bc3e55bf3e5a8c4a3266abe3830edb9`
- Size: 11536 bytes
- Coverage: 1–238; complete EOF; 238 canonical logical lines, no gaps.
- Purpose: Guides authors of Horizon's own WGSL shader-pack format and shader stages.
- Dependencies / cross-references: References horizon-shader-api, WGSL, naga, renderer stages, shader manifests, mod registry and pack validation.
- Implementation / logic: Defines sky/material/weather/post stages, resource budgets, hot reload, typed inputs, sprite tags, material IDs and option overrides.
- Findings: Horizon WGSL format is not Iris/OptiFine shader format; API is experimental and many features are distinct from Java 1.20.1 shader mod ecosystems.
- RustChunk relevance: Optional graphics extension only; not needed for baseline rendering or Forge mod loading.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Prioritize vanilla 1.20.1 rendering and Forge shader-mod compatibility; do not substitute Horizon WGSL packs for OptiFine/Iris.
- Follow-up checks: Check 1.20.1 shader-pack ecosystem, Android GLES/Vulkan capability, shader memory budgets and driver fallback only if a shader feature is later scoped.
- RustChunk implementation mapping: No baseline mapping; future optional shader extension.
- Evidence required before adoption: Working 1.20.1 renderer, Mali GPU shader support, target API design, compatibility and security tests.
- Completion state: COMPLETE — 238 logical lines through EOF; pinned source SHA and 11536 byte size match; inspected, not modified; tests not executed.

### 2884. `docs/shared-model-library-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ee483574c2dd9665ee98c4c09701311d1b622574`
- Size: 5588 bytes
- Coverage: 1–106; complete EOF; 106 canonical logical lines, no gaps.
- Purpose: Measures sharing a prepared model library between asset loading and item renderer setup.
- Dependencies / cross-references: References Assets::load, ItemRenderer::load_with_library, block model baking, item definitions and real prepared resources.
- Implementation / logic: Reuses immutable baked models to reduce duplicate item preparation; compares combined time, retained heap, peak RSS and parity hashes.
- Findings: 26.3 Linux Ryzen benchmark; combined startup improves 3.47–4.83%, but asset-only consumers retain ~6 MiB extra memory; no Android data.
- RustChunk relevance: Useful for avoiding duplicate model baking, but memory cost matters on low-RAM phones.
- Decision: adaptable; reference
- 1.20.1 implications: Consider shared immutable asset/model caches after measuring RustChunk's startup and RAM; do not assume speedup outweighs retained memory on Galaxy A22 class hardware.
- Follow-up checks: Measure cold/warm preparation, peak and retained memory, reload invalidation, item-only errors and exact model/mesh parity on Android.
- RustChunk implementation mapping: RustChunk asset preparation and model cache ownership.
- Evidence required before adoption: Actual 1.20.1 resource set, Forge resource reload tests, Android memory profiling and repeatable device benchmarks.
- Completion state: COMPLETE — 106 logical lines through EOF; pinned source SHA and 5588 byte size match; inspected, not modified; tests not executed.

### 2885. `docs/smp.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `229b4bbc6b9b7ca0feed32edd2baf9cdef9099d4`
- Size: 12492 bytes
- Coverage: 1–178; complete EOF; 178 canonical logical lines, no gaps.
- Purpose: Designs an optional server-only survival multiplayer mod with economy, trades, claims, teleports and native hooks.
- Dependencies / cross-references: References horizon-mods, native hook API, server commands, player inventories, world folder and custom persistence.
- Implementation / logic: Keeps rules in a server-only native mod; clients use vanilla menus/chat and server remains authoritative.
- Findings: This is Horizon native server modding, not Java Forge modding; it is a server-specific feature design, not a client launcher API.
- RustChunk relevance: Useful only as an example of server-authoritative feature separation and transactional persistence.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not add native SMP mods or server economies to RustChunk; Forge mods must execute through Forge's Java runtime and server channels.
- Follow-up checks: If a future client UI interacts with server features, use vanilla protocol and never assume a server has this private mod.
- RustChunk implementation mapping: No current implementation mapping.
- Evidence required before adoption: Explicit server-side scope and 1.20.1 Forge protocol/mod integration evidence.
- Completion state: COMPLETE — 178 logical lines through EOF; pinned source SHA and 12492 byte size match; inspected, not modified; tests not executed.

### 2886. `docs/starling-catalog.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9247eaf3c17ea3636c11a5baab2f316bdad8e144`
- Size: 2554 bytes
- Coverage: 1–34; complete EOF; 34 canonical logical lines, no gaps.
- Purpose: Describes an in-game catalog UI for mods, resource packs and shader packs, with paginated touch-friendly lists.
- Dependencies / cross-references: References starling-catalog, launcher browser, Modrinth APIs, Horizon registry and game options screens.
- Implementation / logic: Uses one scrollable finger-driven list, auto-fetches pages near the end and keeps rows on retry; install destinations vary by content type.
- Findings: Catalog excludes Java mods and uses Horizon-native/content formats; shader pack compatibility is delegated to conversion and phone performance.
- RustChunk relevance: Useful UI/UX and pagination model for a mobile mod/resource browser; package semantics are incompatible with Forge.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Reuse touch-list/pagination concepts but install original Forge 1.20.1 JARs and standard resource packs; never present Java mods as supported if they cannot load.
- Follow-up checks: Test Modrinth metadata, Forge loader/version filters, paging failure/retry, SHA-512 verification, storage permissions and offline browsing.
- RustChunk implementation mapping: RustChunk mobile mod catalog and resource-pack browser.
- Evidence required before adoption: Forge 47.x mod files, Modrinth/CurseForge API behavior, Android storage tests and actual mod launch.
- Completion state: COMPLETE — 34 logical lines through EOF; pinned source SHA and 2554 byte size match; inspected, not modified; tests not executed.

### 2887. `docs/starling-performance.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `14fc5da9a138d3967ce9dbf84ac1286a5ca40b53`
- Size: 8306 bytes
- Coverage: 1–125; complete EOF; 125 canonical logical lines, no gaps.
- Purpose: Records Android GPU/CPU measurements and ruled-out optimizations for Starling on a POCO F6 Pro.
- Dependencies / cross-references: References Vulkan renderer, F3 metrics, GPU timestamps, culling, LOD, upload budgets, ADPF and Android APK profiles.
- Implementation / logic: Finds resolution scaling affects pixel cost, LOD adds work rather than reducing near geometry, GPU occlusion culling can hide terrain, and chunk packet/mesh upload budgets can bound frame spikes.
- Findings: Measurements are POCO F6 Pro/Adreno 740/Vulkan, not Galaxy A22/Mali-G52/OpenGL ES; some hypotheses are untested and results are specific to scenes/settings.
- RustChunk relevance: Highly relevant performance methodology and caution: distant LOD cannot repair missing nearby chunks; near chunk readiness and bounded upload work must be measured first.
- Decision: adaptable; reference; device-specific
- 1.20.1 implications: Do not copy benchmark numbers. On RustChunk, prioritize timely nearby chunk arrival/mesh readiness, then profile upload budgets, CPU/GPU time and resolution on the actual Mali-G52 device; treat LOD as visual supplement only.
- Follow-up checks: Compare same world/camera/settings, release profile, warmup, frame-time distribution, chunk-ready count, upload queue backlog, GPU timestamps where supported and thermal state.
- RustChunk implementation mapping: RustChunk Android renderer telemetry, chunk streaming/mesh upload scheduler and performance regression suite.
- Evidence required before adoption: Galaxy A22 ARM64/Mali-G52/Android 13 actual runs, GLES backend captures, stable frame-time traces, visible-ground correctness and memory/thermal measurements.
- Completion state: COMPLETE — 125 logical lines through EOF; pinned source SHA and 8306 byte size match; inspected, not modified; tests not executed.

### 2888. `docs/structure-density.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `448d462c9efb715d80b7f1639d4024f01bbf12e7`
- Size: 2445 bytes
- Coverage: 1–43; complete EOF; 43 canonical logical lines, no gaps.
- Purpose: Documents structure terrain-adaptation density (Beardifier) for rigid pieces and jigsaw junctions.
- Dependencies / cross-references: References horizon_core::density::Beardifier, WorldField, structure pieces, junctions and vanilla parity runner.
- Implementation / logic: Computes beard adjustment kernels with ordered float accumulation, wrapped coordinates and scalar/bulk clipping semantics.
- Findings: Targets 26.3; live structure discovery, selection, placement and complete terrain generation remain separate or pending.
- RustChunk relevance: Relevant worldgen algorithm but dependent on 1.20.1 structure codecs and RNG.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild the structure density algorithm and bounds from 1.20.1 target classes; don't reuse 26.3 constants/structure registries.
- Follow-up checks: Test all beard modes, kernel boundaries, accumulation order, scalar/bulk parity, extreme coordinates and full structure terrain.
- RustChunk implementation mapping: RustChunk structure density and worldgen sampler.
- Evidence required before adoption: 1.20.1 Beardifier oracle, structure piece fixtures, seeded full-chunk comparison and Forge structure registry tests.
- Completion state: COMPLETE — 43 logical lines through EOF; pinned source SHA and 2445 byte size match; inspected, not modified; tests not executed.

### 2889. `docs/surface-generation-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `14a465402ebb6e224fc1c322e4a6d1471d23b504`
- Size: 3483 bytes
- Coverage: 1–15; complete EOF; 15 canonical logical lines, no gaps.
- Purpose: Reports constant surface-rule specialization timing across dimensions, seeds and worker counts.
- Dependencies / cross-references: References density bulk optimization, surface generation benchmark and complete terrain snapshots.
- Implementation / logic: Uses predeclared workloads, alternating paired samples and separate terrain/decoration timings; reports median percentage changes.
- Findings: Small desktop-only improvements, substantial paired noise and one End four-worker regression; historical incremental timings on Ryzen/Linux.
- RustChunk relevance: Useful benchmark methodology for worldgen optimization, not transferable performance results.
- Decision: adaptable; reference
- 1.20.1 implications: Require measured gains across representative RustChunk 1.20.1 workloads; don't accept one best-case percentage as general improvement.
- Follow-up checks: Use same seed/sites, worker count, full decorated snapshots, repeated alternating runs, warmups and per-dimension regressions.
- RustChunk implementation mapping: RustChunk worldgen benchmark suite and optimization acceptance gate.
- Evidence required before adoption: Actual Android/host ARM64 data, full 1.20.1 generated-chunk parity and statistically repeated workloads.
- Completion state: COMPLETE — 15 logical lines through EOF; pinned source SHA and 3483 byte size match; inspected, not modified; tests not executed.

### 2890. `docs/telemetry.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `69cda4d56477ba7eb73e1baa8b122b003c31b95d`
- Size: 13860 bytes
- Coverage: 1–129; complete EOF; 129 canonical logical lines, no gaps.
- Purpose: Specifies opt-in performance/crash/bug/benchmark collection, data minimization, outbox, upload and an always-on player-count ping.
- Dependencies / cross-references: References horizon-telemetry, consent.json, outbox, scrubber, telemetry server, benchmark report schema and launcher settings.
- Implementation / logic: Uses independent consent switches, schema versioning, bounded queues, deletion by install ID and scrubbed reports; a separate always-on 'I'm playing' ping runs without the four consent switches.
- Findings: The always-on ping is an exception to the stated 'nothing leaves without consent' principle; hardware/diagnostic fields and deletion/retention require careful user-facing disclosure.
- RustChunk relevance: Privacy and consent design are highly relevant, but the always-on ping should not be copied into RustChunk.
- Decision: adaptable; reference; privacy-sensitive
- 1.20.1 implications: Default all telemetry/network reporting off, make every outbound item separately opt-in, provide exact preview, local-only mode and deletion; no hidden player-count ping.
- Follow-up checks: Audit IP/server logs, install IDs, retention, crash stack scrubber, log attachments, consent versioning, deletion completeness and Android network background behavior.
- RustChunk implementation mapping: RustChunk privacy/telemetry settings and diagnostics export.
- Evidence required before adoption: Privacy threat model, data-flow inventory, opt-in UX, no-network tests when disabled, deletion verification and policy review.
- Completion state: COMPLETE — 129 logical lines through EOF; pinned source SHA and 13860 byte size match; inspected, not modified; tests not executed.

### 2891. `docs/terrain-filling.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8daf2804d30608c76ebab4ca82c32e691a6c7585`
- Size: 5146 bytes
- Coverage: 1–88; complete EOF; 88 canonical logical lines, no gaps.
- Purpose: Describes noise-stage terrain filling, aquifer selection, resident block sections, heightmaps and fluid postprocessing.
- Dependencies / cross-references: References TerrainChunk, density compiler, NoiseBasedAquifer, HeightmapState, section palettes and worldgen parity harness.
- Implementation / logic: Samples final density before writes, traverses Z/X/decreasing Y, preserves prior blocks on default-air skips, updates heightmaps and ordered fluid postprocessing offsets.
- Findings: 26.3; full biome assignment, structures, features, nonempty blending, lighting and chunk lifecycle remain pending. Scoped fixture parity is not complete world generation.
- RustChunk relevance: Core worldgen pipeline is directly relevant but must be reconstructed for 1.20.1.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Implement only after 1.20.1 density/aquifer/heightmap semantics and Forge registry lifecycle are verified; preserve partial writes and traversal order.
- Follow-up checks: Test seeded chunk snapshots, aquifer states, scheduled fluids, repeated fills, prefilled chunks, vertical subranges and Forge datapack overrides.
- RustChunk implementation mapping: RustChunk terrain generator noise-fill stage.
- Evidence required before adoption: 1.20.1 NoiseChunk/NoiseBasedChunkGenerator oracle, Forge datapacks, full chunk block/biome/entity/tick snapshots and live-world integration.
- Completion state: COMPLETE — 88 logical lines through EOF; pinned source SHA and 5146 byte size match; inspected, not modified; tests not executed.

### 2892. `docs/value-providers.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dfca9216fd14bb5327ee8ef2db1c94ed0f7b6bed`
- Size: 3772 bytes
- Coverage: 1–63; complete EOF; 63 canonical logical lines, no gaps.
- Purpose: Documents FloatProvider, IntProvider and HeightProvider codecs and seeded sampling.
- Dependencies / cross-references: References density/worldgen codecs, random sources, vertical anchors and vanilla Java provider classes.
- Implementation / logic: Preserves numeric narrowing, codec defaults, float operation order, random draw count, weighted choices and anchored height sampling.
- Findings: 26.3 provider kinds and bounds; Gaussian providers/canonical encoding remain pending, and host math differences are documented.
- RustChunk relevance: Highly relevant to data-driven worldgen and deterministic RNG, but target schemas differ.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Rebuild provider types/codecs and exact random draw order from 1.20.1, including Forge-added provider types.
- Follow-up checks: Test nested clamps, malformed definitions, zero/reversed ranges, signed zero, weighted lists, anchors and subsequent RNG state.
- RustChunk implementation mapping: RustChunk worldgen value/height provider codecs.
- Evidence required before adoption: 1.20.1 codec oracle, Forge registry extensions, seeded fixture corpus and deterministic chunk generation.
- Completion state: COMPLETE — 63 logical lines through EOF; pinned source SHA and 3772 byte size match; inspected, not modified; tests not executed.

### 2893. `docs/velocity-forwarding.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `17ac60a69cd215a9a466dcb5b768467450619d83`
- Size: 2794 bytes
- Coverage: 1–48; complete EOF; 48 canonical logical lines, no gaps.
- Purpose: Documents Velocity modern player-information forwarding for Horizon dedicated servers behind a proxy.
- Dependencies / cross-references: References login plugin messages, velocity:player_info, HMAC-SHA256, forwarding version and profile properties.
- Implementation / logic: Captures a real Velocity 4.3.1 exchange and validates HMAC, address, UUID, name and profile properties; unsupported versions/addresses/trailing bytes are refused.
- Findings: Server-side proxy forwarding only; version 1 observed and non-empty profile properties not observed. No vanilla reference parity.
- RustChunk relevance: Optional for proxy-hosted servers; not needed for ordinary Aternos connection.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Do not implement Velocity forwarding as a client feature; it belongs on a backend server. Client only responds to normal login plugin requests according to 1.20.1.
- Follow-up checks: If proxy compatibility issues arise, verify 1.20.1 login plugin request/response handling and do not leak or forge forwarding data.
- RustChunk implementation mapping: No client implementation mapping beyond generic login plugin message handling.
- Evidence required before adoption: Real 1.20.1 Forge client behind Velocity test, exact proxy version and authenticated packet captures.
- Completion state: COMPLETE — 48 logical lines through EOF; pinned source SHA and 2794 byte size match; inspected, not modified; tests not executed.

### 2894. `docs/vertical-anchors.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ce743463a634ad34be17ea603a7f0d42bb1c27fb`
- Size: 1306 bytes
- Coverage: 1–22; complete EOF; 22 canonical logical lines, no gaps.
- Purpose: Documents absolute, above-bottom, below-top and sea-level-relative worldgen anchors.
- Dependencies / cross-references: References WorldGenerationContext, HeightProvider and vanilla anchor codecs.
- Implementation / logic: Preserves wrapped integer resolution, codec range/truncation and independent min/height intersection semantics.
- Findings: 26.3 constants and codecs; this does not establish 1.20.1 height behavior.
- RustChunk relevance: Relevant to data-driven worldgen height providers; target constants must be re-derived.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Implement 1.20.1 anchor enum/codec and Java arithmetic exactly; connect to the target dimension type/height accessor.
- Follow-up checks: Test all anchors, negative min Y, tall dimensions, overflow, decimal truncation and disjoint generator bounds.
- RustChunk implementation mapping: RustChunk worldgen vertical anchor and height provider logic.
- Evidence required before adoption: 1.20.1 DimensionType/HeightProvider source oracle and tall-datapack fixtures.
- Completion state: COMPLETE — 22 logical lines through EOF; pinned source SHA and 1306 byte size match; inspected, not modified; tests not executed.

### 2895. `docs/visibility-benchmark.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5fafea0b4da6d9850bd53a2f138c97b4e155551a`
- Size: 4234 bytes
- Coverage: 1–77; complete EOF; 77 canonical logical lines, no gaps.
- Purpose: Benchmarks a section face-visibility flood-fill shortcut and stack allocation.
- Dependencies / cross-references: References chunk mesher, section opacity, face-pair masks, traversal stack and synthetic full-sized sections.
- Implementation / logic: Pre-resolves opacity, uses 4 KiB worker stack instead of heap allocation and early exits for fully open/closed or all-face-connected sections.
- Findings: Desktop Ryzen synthetic microbenchmark; some 4–7% gains are noise-sensitive and no distribution-weighted real-world aggregate exists.
- RustChunk relevance: Directly relevant to chunk meshing CPU cost and worker allocations.
- Decision: adaptable; reference
- 1.20.1 implications: Use only after proving exact 1.20.1/Forge face visibility; compare allocation reduction and actual mesh time on Mali device.
- Follow-up checks: Test all 64 face subsets, opacity states, mixed sections, stack limits, worst-case traversal and output equality.
- RustChunk implementation mapping: RustChunk chunk mesher visibility helper and worker stack policy.
- Evidence required before adoption: 1.20.1 block occlusion oracle, Forge block shapes, realistic section corpus, ARM64 benchmarks and mesh correctness.
- Completion state: COMPLETE — 77 logical lines through EOF; pinned source SHA and 4234 byte size match; inspected, not modified; tests not executed.

### 2896. `docs/windows-allocators.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `229813589dec4fa5ee74f5bc88bcbc1d8e95e60a`
- Size: 3826 bytes
- Coverage: 1–69; complete EOF; 69 canonical logical lines, no gaps.
- Purpose: Describes Windows Segment Heap experiments alongside an existing mimalloc allocator.
- Dependencies / cross-references: References Rust allocator, Windows executable manifest, winresource and Windows SDK validation.
- Implementation / logic: Adds optional Segment Heap manifest for Win32 heap allocations without replacing Rust mimalloc; requires rebuilding and measuring separate variants.
- Findings: Windows 10 2004+ feature; no Android relevance and older Windows versions untested.
- RustChunk relevance: Not relevant to Android target; may inform future Windows developer/launcher builds only.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Do not add Windows Segment Heap to Android build; keep allocator selection target-specific.
- Follow-up checks: If Windows packaging is later scoped, test actual manifest in executable and compare same workload/profile.
- RustChunk implementation mapping: No Android implementation mapping.
- Evidence required before adoption: Supported Windows OS matrix, manifest extraction, native library compatibility and repeatable memory/frame tests.
- Completion state: COMPLETE — 69 logical lines through EOF; pinned source SHA and 3826 byte size match; inspected, not modified; tests not executed.

### 2897. `docs/world-generation.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `68ffddf6e9b81adabe0c3e5f429a63caa5efd479`
- Size: 17614 bytes
- Coverage: 1–251; complete EOF; 251 canonical logical lines, no gaps.
- Purpose: Summarizes noise/density registry decoding, noise routers, aquifers, climate, biome source, tags and terrain generation parity.
- Dependencies / cross-references: References horizon_world::worldgen, density graph, NoiseRouter, aquifers, biome sources, tags, terrain filling and whole-chunk harnesses.
- Implementation / logic: Loads named immutable density graphs, compiles noise roots, samples climate and multi-noise biome tables, binds aquifers and compares many stages against actual JARs.
- Findings: Targets 26.3; complete structures/features, nonempty blending, full datapack coverage and complete generated worlds remain pending. Several bounded Rust API limits are explicit.
- RustChunk relevance: A major architectural map for worldgen; all IDs, codec schemas, seeds and constants require 1.20.1 reimplementation.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Build 1.20.1 worldgen from official target codecs and Forge registries in dependency order; do not copy 26.3 registries or import tables.
- Follow-up checks: Test noise seeds, codec defaults, density graph cycles, climate quantization, biome tie behavior, aquifer caches, datapack overrides and full chunk output.
- RustChunk implementation mapping: RustChunk worldgen architecture: noise, density, biome, aquifer, tags and terrain generation.
- Evidence required before adoption: 1.20.1 client/server JAR oracle, Forge datapack/registry fixtures, deterministic chunk snapshots and actual integrated/server gameplay.
- Completion state: COMPLETE — 251 logical lines through EOF; pinned source SHA and 17614 byte size match; inspected, not modified; tests not executed.

### 2898. `docs/world-height.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `31d87f9f30527b95863cdd6935e770a5c5bca4eb`
- Size: 5534 bytes
- Coverage: 1–75; complete EOF; 75 canonical logical lines, no gaps.
- Purpose: Describes per-dimension LevelShape, chunk columns, storage, lighting, meshing, saves and network height handling.
- Dependencies / cross-references: References DimensionType codecs, LevelShape/LevelShapes, Chunk, SaveDir, LightWorld, renderer layout and protocol registry.
- Implementation / logic: Carries shape with each world/chunk/save/light/mesh/network job rather than a process global; supports tall datapack dimensions and per-level registry data.
- Findings: 26.3 implementation; tall-world upper bounds and exact storage/lighting border semantics differ by version and Forge registry additions.
- RustChunk relevance: Important for chunk storage, remote dimension changes and correct render/lighting bounds.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Implement 1.20.1 dimension type height and per-dimension chunk layout; never assume a single global -64..320 range when Forge datapacks can alter dimensions.
- Follow-up checks: Test 1.20.1 dimension codecs, tall dimensions, save/load, lighting borders, mesh rebuilds, chunk packets and dimension switch cache invalidation.
- RustChunk implementation mapping: RustChunk per-dimension world/chunk layout and network chunk decoder.
- Evidence required before adoption: 1.20.1 DimensionType registry oracle, Forge datapacks, tall-world save/lighting/render tests and server integration.
- Completion state: COMPLETE — 75 logical lines through EOF; pinned source SHA and 5534 byte size match; inspected, not modified; tests not executed.

### 2899. `launcher/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `227b93e11533082cade936c3db0099bf65334b58`
- Size: 5441 bytes
- Coverage: 1–105; complete EOF; 105 canonical logical lines, no gaps.
- Purpose: Defines Horizon desktop launcher's separate Cargo workspace and platform dependencies.
- Dependencies / cross-references: References GPUI Fast, keyring backends, HTTP/TLS, ZIP/tar, modpack porting, telemetry, release signing and Windows/macOS/Linux platform crates.
- Implementation / logic: Keeps launcher separate from game build; uses platform-conditional secure stores, profile/config parsers, network and packaging dependencies.
- Findings: GPUI desktop dependencies and OS keychain crates are not Android UI/storage APIs; horizon-port-pack is a native mod porting pipeline, not Forge execution.
- RustChunk relevance: Useful as a dependency-boundary and separate-workspace reference, not as a RustChunk Android manifest.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Keep RustChunk launcher and client/core modules separated; select Android-compatible UI, secure storage, HTTP and filesystem crates rather than importing GPUI/keyring wholesale.
- Follow-up checks: Audit transitive native dependencies, Android NDK support, APK size, license, TLS, key storage and cross-compilation.
- RustChunk implementation mapping: RustChunk Cargo workspace, Android launcher crate and shared core dependency graph.
- Evidence required before adoption: Successful ARM64 Android cross-build, dependency/license audit, APK size and runtime storage/keychain tests.
- Completion state: COMPLETE — 105 logical lines through EOF; pinned source SHA and 5441 byte size match; inspected, not modified; tests not executed.

### 2900. `launcher/README.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `381a2aeddafc4478da382eb49782134da683bddc`
- Size: 42414 bytes
- Coverage: 1–400, 401–538; complete EOF; 538 canonical logical lines, no gaps.
- Purpose: Comprehensive desktop launcher guide covering installs, accounts, Java instances, modpacks, servers, updates, telemetry, MCP and settings.
- Dependencies / cross-references: References launcher backend/CLI, horizon-prepare, mod browser, Forge-like Java instances, account services, signed releases, telemetry and server supervisor.
- Implementation / logic: Separates launcher UI/CLI/MCP over shared backend; supports instances, profiles, offline names gated by owner account, Java runtime management, modpacks, packs, multiplayer, backups and launch options.
- Findings: Desktop GPUI and Horizon 26.3 workflow; offline profiles still require an owning account for launch and do not authenticate online-mode servers. Java mods are ported, not run through Forge.
- RustChunk relevance: Strong reference for launcher feature taxonomy, profile isolation, preparation, safe update and import/export flows; its mod/account model is not directly compatible with RustChunk goals.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Build Android-first profiles and instance isolation while running real Forge 1.20.1 JARs. Design offline mode transparently and lawfully; no false online authentication. Keep game data separate from launcher state.
- Follow-up checks: Verify Forge installer/runtime selection, modpack imports, Java 17 runtime, offline/online mode, secure token storage, scoped storage, backup/restore and update rollback on Android.
- RustChunk implementation mapping: RustChunk launcher screens, profile manager, resource preparation, Forge installer and account/session bridge.
- Evidence required before adoption: Actual 1.20.1 Forge launch, Aternos join, Samsung Android lifecycle/storage, account compliance review and full modpack regression tests.
- Completion state: COMPLETE — 538 logical lines through EOF; pinned source SHA and 42414 byte size match; inspected, not modified; tests not executed.

### 2901. `launcher/benchmarks/release-loading.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4c933e3c34abcb088de9e7eea821f14ee08cc594`
- Size: 2557 bytes
- Coverage: 1–46; complete EOF; 46 canonical logical lines, no gaps.
- Purpose: Documents a launcher-startup optimization that shares one fetched GitHub releases response between game-release and update parsing.
- Dependencies / cross-references: Launcher backend release retrieval, startup workers, JSON parsing, fallback lookup, update checks and benchmark test.
- Implementation / logic: Parses and decodes the shared response once, publishes game releases before fallback lookups, while manual/four-hour checks still fetch fresh data; reports synthetic paired median timings.
- Findings: The reported 2.003x is serial parse/decoding only; concurrent startup improvement is 7.7%. Mock tests omit HTTP/TLS, rate limits, install scans, GUI and end-to-end startup. Validation claims belong to upstream snapshot and were not reproduced here.
- RustChunk relevance: Useful measurement method for reducing duplicate network/JSON work; desktop backend architecture and numbers are not transferable to Android.
- Decision: adaptable; reference; benchmark-only
- 1.20.1 implications: Reuse request deduplication only where RustChunk startup actually duplicates metadata calls; use Android networking and avoid desktop-specific assumptions.
- Follow-up checks: Compare cold/warm startup, network latency, cancellation, stale cache behavior, update fallback, memory and rate-limit behavior on ARM64.
- RustChunk implementation mapping: RustChunk release metadata repository and startup orchestration.
- Evidence required before adoption: Reproducible ARM64 release-profile benchmark, real HTTP fixtures, equivalent parsed results, cancellation and update failure tests.
- Completion state: COMPLETE — 46 logical lines through EOF; pinned source SHA and 2557 byte size match; reviewed, not modified; tests not executed.

### 2902. `launcher/lang/de.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2c06fa518fd7f764b12a375be82f3166310da35c`
- Size: 72730 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: German launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source-key set, translation lookup and all desktop launcher screens.
- Implementation / logic: 914 string keys mapped to German text; placeholders retained consistently with other locale dictionaries.
- Findings: Valid JSON, 914 keys, no missing/extra keys versus first locale, no empty/non-string values. 124 values remain exactly English; locale is incomplete.
- RustChunk relevance: Localization coverage and placeholder preservation are relevant to RustChunk UI, but the vocabulary reflects Horizon desktop functionality.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Use as translation-process example only; create RustChunk-specific Persian and other locale catalogs from actual Android UI strings.
- Follow-up checks: Check ICU/plural behavior, bidi/RTL, font coverage, truncation, placeholder parity and stale keys in RustChunk.
- RustChunk implementation mapping: RustChunk Android localization resources; Persian-first UI.
- Evidence required before adoption: Complete Persian translations, placeholder tests, RTL screenshots on Android and translation review.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 72730 byte size match; reviewed, not modified; tests not executed.

### 2903. `launcher/lang/es.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `afe5716a941cef4f2855859624fa42e4ac350156`
- Size: 73669 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: Spanish launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source-key set and launcher i18n lookup.
- Implementation / logic: Maps the same 914 source keys to Spanish strings.
- Findings: Valid JSON and same 914-key set; no empty/non-string values; placeholders match German baseline. 28 strings are unchanged English.
- RustChunk relevance: Useful localization reference, but Horizon-specific copy is not directly reusable.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Build RustChunk translations against its own stable key set; avoid copying unreviewed Horizon phrasing.
- Follow-up checks: Test pluralization, interpolation, truncation and platform-specific terminology.
- RustChunk implementation mapping: RustChunk localization catalog.
- Evidence required before adoption: Native-speaker review, placeholder parity and Android UI rendering.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 73669 byte size match; reviewed, not modified; tests not executed.

### 2904. `launcher/lang/fr.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0faa28c9da66594fd88eede7613b784130ce7d26`
- Size: 73750 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: French launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source-key set and launcher i18n lookup.
- Implementation / logic: Maps 914 source keys to French text.
- Findings: Valid JSON, key set aligned and placeholders match; 116 strings are unchanged English. Some long sentences need UI-fit validation.
- RustChunk relevance: Localization model is reusable; Horizon copy and desktop labels are version-specific.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Use separate RustChunk translation catalog and validate long labels in narrow mobile layouts.
- Follow-up checks: Review French typography, apostrophes, pluralization, interpolation and Android text wrapping.
- RustChunk implementation mapping: RustChunk localization resources.
- Evidence required before adoption: Native-speaker review and screenshots across phone widths.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 73750 byte size match; reviewed, not modified; tests not executed.

### 2905. `launcher/lang/ja.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a8c9f020125fcab5bd753197d2e22b571d0a0f0c`
- Size: 82174 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: Japanese launcher UI localization dictionary.
- Dependencies / cross-references: Shared 914-key English source catalog and i18n resolver.
- Implementation / logic: Maps all source keys to Japanese strings, retaining interpolation markers.
- Findings: Valid JSON and key parity; placeholders match; 146 strings remain English. Several product/technical names are naturally untranslated but require contextual review.
- RustChunk relevance: Shows localization coverage needs and CJK text handling; desktop GPUI font behavior does not transfer.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Provide Japanese resources only if scoped; Android must bundle suitable CJK fonts and handle variable-width text.
- Follow-up checks: Check font fallback, glyph coverage, line breaks, button clipping and placeholder parity.
- RustChunk implementation mapping: RustChunk locale resources and font fallback policy.
- Evidence required before adoption: Real-device CJK screenshots, missing-glyph scan and native-speaker review.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 82174 byte size match; reviewed, not modified; tests not executed.

### 2906. `launcher/lang/pl.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `66d3ba2fd61c44de832f1e465ba86b8be613c0d7`
- Size: 71494 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: Polish launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source catalog and i18n resolver.
- Implementation / logic: Maps 914 source keys to Polish strings.
- Findings: Valid JSON, aligned keys and placeholders; 107 entries are identical to English. No empty values.
- RustChunk relevance: Useful localization structure, but all text must be scoped to RustChunk UI.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Do not import Horizon's catalog as RustChunk copy; maintain key parity with RustChunk's source strings.
- Follow-up checks: Test Polish plural forms, grammatical gender, interpolation and narrow layouts.
- RustChunk implementation mapping: RustChunk localization resources.
- Evidence required before adoption: Native-speaker review, plural tests and Android screenshots.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 71494 byte size match; reviewed, not modified; tests not executed.

### 2907. `launcher/lang/pt-br.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `659f24639e78be43b5a985ccd812bba1af0653a1`
- Size: 73839 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: Brazilian Portuguese launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source catalog and i18n resolver.
- Implementation / logic: Maps 914 source keys to Brazilian Portuguese.
- Findings: Valid JSON and key parity; placeholders match; 48 strings unchanged English.
- RustChunk relevance: Localization example; desktop launcher terminology needs adaptation.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Create RustChunk locale strings based on actual launcher workflows and preserve interpolation names.
- Follow-up checks: Check Brazilian terminology, pluralization, punctuation and mobile line wrapping.
- RustChunk implementation mapping: RustChunk localization catalog.
- Evidence required before adoption: Native-speaker review and Android screenshots.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 73839 byte size match; reviewed, not modified; tests not executed.

### 2908. `launcher/lang/ru.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0056523817afda97e5bdeb7350d4c81c7502b756`
- Size: 90400 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: Russian launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source catalog and i18n resolver.
- Implementation / logic: Maps 914 source keys to Russian strings.
- Findings: Valid JSON, key parity and placeholder parity; 84 English-identical values remain.
- RustChunk relevance: Localization reference; Cyrillic font and line behavior must be validated on Android.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Use RustChunk-owned strings and test font fallback for Cyrillic.
- Follow-up checks: Test plural categories, case/declension, interpolation and text overflow.
- RustChunk implementation mapping: RustChunk locale resources.
- Evidence required before adoption: Native-speaker review, plural tests and device screenshots.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 90400 byte size match; reviewed, not modified; tests not executed.

### 2909. `launcher/lang/tr.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `501c1d9bebf5144e933757b077cef583148eb926`
- Size: 73658 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: Turkish launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source catalog and i18n resolver.
- Implementation / logic: Maps 914 source keys to Turkish strings.
- Findings: Valid JSON, matching keys/placeholders; 154 values remain exactly English, the highest count among these locales.
- RustChunk relevance: Demonstrates locale completeness QA; Horizon text is not a RustChunk translation source.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Keep a translation completeness report for RustChunk and don't assume valid JSON means fully translated.
- Follow-up checks: Review Turkish terminology, casing, suffixes, interpolation and narrow layouts.
- RustChunk implementation mapping: RustChunk localization QA.
- Evidence required before adoption: Native-speaker review and untranslated-string budget.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 73658 byte size match; reviewed, not modified; tests not executed.

### 2910. `launcher/lang/uk.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `019d81ea3ce73cc28db64cc57dec0a8c4e905158`
- Size: 90896 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: Ukrainian launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source catalog and i18n resolver.
- Implementation / logic: Maps 914 source keys to Ukrainian strings.
- Findings: Valid JSON, aligned keys and placeholders; 94 strings unchanged English.
- RustChunk relevance: Localization reference with CJK/RTL/platform concerns separate.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Build RustChunk's own locale dictionary and test Cyrillic rendering.
- Follow-up checks: Check Ukrainian grammar/plurals, font fallback and mobile overflow.
- RustChunk implementation mapping: RustChunk localization resources.
- Evidence required before adoption: Native-speaker review and Android device screenshots.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 90896 byte size match; reviewed, not modified; tests not executed.

### 2911. `launcher/lang/zh.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `718ede889f3d6bbcec9b03731ca260995ce25968`
- Size: 69403 bytes
- Coverage: 1–916; complete EOF; 916 canonical logical lines, no gaps.
- Purpose: Simplified Chinese launcher UI localization dictionary.
- Dependencies / cross-references: Shared English source catalog and i18n resolver.
- Implementation / logic: Maps 914 source keys to Simplified Chinese strings.
- Findings: Valid JSON, same key set and placeholder parity; 97 entries unchanged English.
- RustChunk relevance: Useful CJK localization reference; desktop font assumptions do not transfer to Android.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Use RustChunk-owned locale keys and verify Chinese glyph/font fallback on target Android.
- Follow-up checks: Check CJK punctuation, fallback fonts, line heights, interpolation and clipping.
- RustChunk implementation mapping: RustChunk localization resources.
- Evidence required before adoption: Real Android CJK screenshots, font coverage and native-speaker review.
- Completion state: COMPLETE — 916 logical lines through EOF; pinned source SHA and 69403 byte size match; reviewed, not modified; tests not executed.

### 2912. `launcher/links.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `951275408c2a7ede144263a92b61caa1338d7bba`
- Size: 234 bytes
- Coverage: 1–1; complete EOF; 1 canonical logical lines, no gaps.
- Purpose: Stores launcher-owned community, sponsorship, website and official-server links.
- Dependencies / cross-references: Launcher UI links and server discovery/presentation.
- Implementation / logic: Small JSON object with Discord, sponsor, website, official server and prior official server values.
- Findings: All URLs and hostnames are Horizon-specific branding and infrastructure; one official server entry is a hostname, not a full URL.
- RustChunk relevance: No product functionality to port; avoid accidental branding, network routing or endorsement transfer.
- Decision: unnecessary; reference; branding-sensitive
- 1.20.1 implications: Create RustChunk-owned links only after official services exist; never copy Horizon's server or sponsor endpoints.
- Follow-up checks: Validate HTTPS, host allowlist, link opening policy, configuration updates and privacy implications.
- RustChunk implementation mapping: RustChunk about/help/community configuration.
- Evidence required before adoption: Verified RustChunk domains, ownership, TLS and explicit approval of any official server endpoint.
- Completion state: COMPLETE — 1 logical lines through EOF; pinned source SHA and 234 byte size match; reviewed, not modified; tests not executed.

### 2913. `launcher/src/browser/java_instance/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `35a3a5a7b8d34c8e302f0a3ec4e1b4f2548fc911`
- Size: 10700 bytes
- Coverage: 1–322; complete EOF; 322 canonical logical lines, no gaps.
- Purpose: Tests Java-instance mod/resource-pack selection, dependency installation, update/disable/remove lifecycle and target filtering.
- Dependencies / cross-references: Browser model, Modrinth source, downloader, Installed index and target loader/version.
- Implementation / logic: Fake source supplies projects, version metadata and bytes; tests choose newest compatible version, dependency order, checksum downloads, disabled update preservation and safe refusal.
- Findings: Tests hard-code Minecraft 1.21.1 with Fabric/Quilt/NeoForge and explicitly reject Forge dependencies; not evidence of 1.20.1 Forge compatibility.
- RustChunk relevance: Useful behavioral test design for RustChunk's mod browser and install transactions, but current compatibility policy conflicts with required Forge 1.20.1.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Build a separate 1.20.1 Forge-aware resolver; do not copy current loader filters or imply its tests cover Forge.
- Follow-up checks: Test Forge 47.x metadata, transitive dependencies, version conflicts, cycles, disabled-state preservation, checksum mismatch, rollback and partial downloads.
- RustChunk implementation mapping: RustChunk mod browser dependency planner and transactional installer.
- Evidence required before adoption: Real Forge 1.20.1 JARs, Modrinth/CurseForge metadata, malicious archive tests and launch validation.
- Completion state: COMPLETE — 322 logical lines through EOF; pinned source SHA and 10700 byte size match; reviewed, not modified; tests not executed.

### 2914. `launcher/src/browser/modrinth.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4efd143d2f8c66795debca8132ee97d3a259ca95`
- Size: 10615 bytes
- Coverage: 1–296; complete EOF; 296 canonical logical lines, no gaps.
- Purpose: Implements Modrinth v2 search/project/version API mapping for the launcher's catalog.
- Dependencies / cross-references: Browser data types, details facets, downloader, source registry and Modrinth REST endpoints.
- Implementation / logic: Builds facets, parses search hits/project/version metadata, normalizes checksums, required dependencies, primary file choice, and URL encoding; GET helpers provide search/project/versions.
- Findings: Modrinth Java mod selection is intentionally oriented to Horizon's 26.3 porting workflow; project_type handling excludes native shader/datapack paths and does not supply Forge runtime semantics.
- RustChunk relevance: Useful API adapter and checksum parsing reference, but cannot be treated as a Forge loader or 1.20.1 compatibility implementation.
- Decision: rewrite required; reference; API-version-sensitive
- 1.20.1 implications: Implement RustChunk's own Modrinth client with 1.20.1+Forge filters and explicit loader compatibility; preserve strong checksum verification and reject unsupported formats.
- Follow-up checks: Test API schema drift, null/malformed fields, pagination/limits, rate limits, SHA-512/SHA-1 fallback, dependency graphs and Forge loader facets.
- RustChunk implementation mapping: RustChunk catalog source adapter and mod metadata resolver.
- Evidence required before adoption: Live API fixtures for Minecraft 1.20.1 Forge, reproducible checksum verification and real install/launch tests.
- Completion state: COMPLETE — 296 logical lines through EOF; pinned source SHA and 10615 byte size match; reviewed, not modified; tests not executed.

### 2915. `launcher/src/browser/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d16e52a8bc0b56a69cb06e2bb5b1b91863993627`
- Size: 17378 bytes
- Coverage: 1–400, 401–451; complete EOF; 451 canonical logical lines, no gaps.
- Purpose: Unit tests for project support labels, version picking, Modrinth/CurseForge parsing, safe filenames and download validation.
- Dependencies / cross-references: Browser types, Modrinth/CurseForge parsers, source registry and downloader.
- Implementation / logic: Exercises game-version/loader filtering, DataPack kind conversion, project parsing, dependencies, checksums, host allowlists, file-name traversal refusal and live network smoke test (ignored).
- Findings: Explicit test asserts Java mods made for 1.20.1 are not installable because this Horizon port only accepts 1.21+; CurseForge is deliberately excluded from active sources. Live test requires network and was not run.
- RustChunk relevance: Critical compatibility evidence: the inspected browser's current Java mod policy does not meet RustChunk's Forge 1.20.1 goal.
- Decision: rewrite required; reference; version-specific; security-sensitive
- 1.20.1 implications: Treat filename/host/checksum refusal tests as design guidance; replace version policy and implement actual Forge metadata and installer support.
- Follow-up checks: Test 1.20.1 Forge versions, CurseForge API licensing/key flow, URL host validation, archive traversal, checksum/size limits and offline behavior.
- RustChunk implementation mapping: RustChunk mod catalog filtering, download trust boundary and compatibility tests.
- Evidence required before adoption: Real 1.20.1 Forge mod corpus, verified metadata, adversarial URL/archive tests and device launch.
- Completion state: COMPLETE — 451 logical lines through EOF; pinned source SHA and 17378 byte size match; reviewed, not modified; tests not executed.

### 2916. `launcher/src/build_import.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8cef7121679b9ca6be66c02bfc89dd945b3755dd`
- Size: 23099 bytes
- Coverage: 1–400, 401–579; complete EOF; 579 canonical logical lines, no gaps.
- Purpose: Imports a locally built Horizon client binary into a separately installed version folder, reusing a signed Horizon release/Nightly's game resources.
- Dependencies / cross-references: Backend release management, native_mods::copy_base, platform executable format, ZIP/TAR.GZ parsing and imported.json metadata.
- Implementation / logic: Finds target-platform executable inside direct files, archives or nested Actions artifacts; caps extracted program at 1 GiB, validates PE/Mach-O/ELF signature and embedded mod API mark, copies base to partial folder, then atomically renames.
- Findings: This is a native binary import pipeline for Horizon 26.3, not a Java Forge installer; it does not establish trust/signature for imported program (UI explicitly warns unsigned). Archive path/size/resource-exhaustion and symlink semantics need adversarial review.
- RustChunk relevance: Atomic import and bounded extraction patterns are useful; platform binary checks and native-mod APIs are not compatible with Android Java runtime installation.
- Decision: adaptable; reference; security-sensitive; platform-specific
- 1.20.1 implications: Use transactional install staging and explicit unsigned-build warning for RustChunk; replace executable validation with signed APK/JAR/runtime artifact verification and Android scoped-storage flow.
- Follow-up checks: Test ZIP/TAR bombs, duplicate entries, path traversal, symlink/hardlink handling, nested archives, size cap, interrupted rename, permissions and rollback on Android.
- RustChunk implementation mapping: RustChunk APK/runtime/Forge profile import and transactional update manager.
- Evidence required before adoption: Signed artifact provenance, archive fuzzing, Android filesystem tests, rollback and real Forge runtime preparation.
- Completion state: COMPLETE — 579 logical lines through EOF; pinned source SHA and 23099 byte size match; reviewed, not modified; tests not executed.

### 2917. `launcher/src/cli.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `42a53b9469732ea17627ad40c31e6319dce564c1`
- Size: 151986 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–3897; complete EOF; 3897 canonical logical lines, no gaps.
- Purpose: Defines a broad Clap CLI for account, Java/Rust runtimes, instances, modpacks, servers, telemetry, native mods, settings, world management and MCP-related launcher commands.
- Dependencies / cross-references: Backend, account/session, loaders, native mods, server management, instances, data packs, screenshots, multiplayer, world packs, release/import and CLI tool generation.
- Implementation / logic: Declares nested command tree and routes execution through backend functions; includes offline account names, Microsoft device-code sign-in, per-launch account selection, server EULA/offline flags, game args, instance operations, file imports/exports and shell wrappers.
- Findings: Desktop-first and Horizon 26.3 scope; native mod/port workflow is not Forge. Offline identities are explicitly for LAN/offline-mode contexts and do not authenticate online-mode servers. Shell pre/post-launch and wrappers are desktop OS features; many commands cannot transfer to Android as-is.
- RustChunk relevance: Command taxonomy and clear separation of parsing/dispatch from backend are useful; most commands and assumptions need an Android/1.20.1 redesign.
- Decision: rewrite required; reference; version-specific; security-sensitive
- 1.20.1 implications: Keep a small Android command/service layer only where useful; design lawful offline profiles transparently, preserve real Microsoft auth for online-mode, and isolate all external process execution.
- Follow-up checks: Audit token handling, device-code flow, offline-mode warnings, EULA gating, shell command injection boundaries, path validation, destructive actions, JSON output and command cancellation.
- RustChunk implementation mapping: RustChunk profile/account manager, Java/Forge preparation, diagnostics and optional developer CLI.
- Evidence required before adoption: Android runtime tests, online/offline server auth tests, EULA/account compliance, command injection tests and Forge 47.x multiplayer.
- Completion state: COMPLETE — 3897 logical lines through EOF; pinned source SHA and 151986 byte size match; reviewed, not modified; tests not executed.

### 2918. `launcher/src/cli_tools.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0f2a04914863bf4cd94d5d32f190fecd17278b4d`
- Size: 18849 bytes
- Coverage: 1–400, 401–506; complete EOF; 506 canonical logical lines, no gaps.
- Purpose: Generates MCP tools from Clap command definitions and classifies dangerous/outbound commands.
- Dependencies / cross-references: CLI command tree, Clap metadata, MCP session runner, confirmation gates and generated launcher command docs.
- Implementation / logic: Walks command tree, creates tool names/arguments, dispatches as child process with JSON output, keeps selected session tools separate, marks destructive commands as requiring confirm and open-world/network actions as user-initiated.
- Findings: Subprocess-based desktop bridge; the file explicitly excludes player-only commands and session handoff/token file. Tool safety depends on correct classification staying synchronized with command changes.
- RustChunk relevance: Useful policy pattern for any future RustChunk automation/API; do not expose unrestricted shell or account/session internals.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: If RustChunk later exposes agent tools, use explicit allowlists, confirmation for destructive actions, separate outbound actions and never expose authentication tokens.
- Follow-up checks: Test every command classification, newly added subcommands, argument serialization, confirm enforcement, session token isolation and subprocess failure/timeouts.
- RustChunk implementation mapping: RustChunk optional local automation API and permission model.
- Evidence required before adoption: Threat model, adversarial command tests, permission UX, token non-disclosure and Android sandbox tests.
- Completion state: COMPLETE — 506 logical lines through EOF; pinned source SHA and 18849 byte size match; reviewed, not modified; tests not executed.

### 2919. `launcher/src/collection.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `281f5f84281ecaa4cc5adcb2afdc3f4c5ef7779e`
- Size: 14849 bytes
- Coverage: 1–400, 401–414; complete EOF; 414 canonical logical lines, no gaps.
- Purpose: Implements desktop telemetry consent screen and collection settings UI.
- Dependencies / cross-references: horizon_telemetry consent/choices, install ID, data preview, delete workflow and first-run UI.
- Implementation / logic: Shows granular performance/crash/bug/benchmark choices, previews exact JSON, records consent and delete-my-data confirmation; includes an always-on “I'm playing” ping row with no switch.
- Findings: The always-on ping is outside the four opt-in categories and weakens the blanket impression that nothing leaves without consent; requires explicit privacy disclosure and policy review.
- RustChunk relevance: Consent preview, granular opt-in and deletion affordances are reusable; the always-on ping must not be copied.
- Decision: adaptable; reference; privacy-sensitive
- 1.20.1 implications: Default RustChunk telemetry and player-presence reporting off; each outbound channel separately opt-in with exact preview and deletion controls.
- Follow-up checks: Trace every outbound field/endpoint, install ID lifecycle, retention, IP/server logs, crash scrubber and delete completeness; test no-network mode.
- RustChunk implementation mapping: RustChunk privacy settings and diagnostics consent UI.
- Evidence required before adoption: Data-flow inventory, no-network test when disabled, deletion verification and user-facing privacy review.
- Completion state: COMPLETE — 414 logical lines through EOF; pinned source SHA and 14849 byte size match; reviewed, not modified; tests not executed.

### 2920. `launcher/src/console.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b12067139a3a7c98cc0c3c8644a8af2466cdc5e8`
- Size: 1799 bytes
- Coverage: 1–48; complete EOF; 48 canonical logical lines, no gaps.
- Purpose: Attaches a Windows CLI process to its parent console and restores standard handles.
- Dependencies / cross-references: Windows kernel32 AttachConsole/GetStdHandle/SetStdHandle and cfg(windows) platform boundary.
- Implementation / logic: Uses unsafe FFI to attach parent process console, captures and restores stdin/stdout/stderr handles with a guard.
- Findings: Windows-only desktop helper; no Android implementation and unsafe handle correctness depends on platform API semantics.
- RustChunk relevance: Not needed for Android launcher; demonstrates narrow platform-specific unsafe isolation.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Do not compile this into Android path; use Android logging/diagnostics APIs.
- Follow-up checks: Check Windows handle ownership, failure restoration, null/invalid handles and cfg isolation in cross-target builds.
- RustChunk implementation mapping: No Android mapping; optional Windows desktop CLI support only.
- Evidence required before adoption: Cross-compilation and Windows console attach/detach integration tests.
- Completion state: COMPLETE — 48 logical lines through EOF; pinned source SHA and 1799 byte size match; reviewed, not modified; tests not executed.

### 2921. `launcher/src/console_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1a37c94bfe841c9fa844cfe1a7ee78f7869f074a`
- Size: 20838 bytes
- Coverage: 1–400, 401–521; complete EOF; 521 canonical logical lines, no gaps.
- Purpose: Implements desktop console/log view with follow, search, copy, share and unshare actions.
- Dependencies / cross-references: Game log stream, selected instance latest.log, mclo.gs upload/delete, GPUI clipboard and scroll state.
- Implementation / logic: Reads current run log or selected instance log, scrubs via game_log::private, asks before upload, shows share URL and deletion control; supports both legacy and new UI paths.
- Findings: Log sharing uploads to a third party and anyone with link can read until deletion/unread; user confirmation exists, but log scrubbing and secret/path/chat redaction must be independently validated.
- RustChunk relevance: Console viewing, filtering and explicit share consent are useful; desktop GPUI and mclo.gs endpoint should not be inherited without review.
- Decision: adaptable; reference; privacy-sensitive
- 1.20.1 implications: Build Android log viewer with local-only default, redact tokens/paths/chat, require explicit upload consent and provide delete link when supported.
- Follow-up checks: Test scrubbing against OAuth/session tokens, IPs, usernames, local paths, chat, server addresses, failed upload and deletion lifecycle.
- RustChunk implementation mapping: RustChunk in-app log viewer and opt-in diagnostics exporter.
- Evidence required before adoption: Adversarial log fixtures, endpoint privacy review, Android share UX and verified remote deletion.
- Completion state: COMPLETE — 521 logical lines through EOF; pinned source SHA and 20838 byte size match; reviewed, not modified; tests not executed.

### 2922. `launcher/src/cull.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `52959066e6cd13a6f387ee49aa512ec07fa5e7df`
- Size: 12677 bytes
- Coverage: 1–355; complete EOF; 355 canonical logical lines, no gaps.
- Purpose: Virtualizes variable-height desktop lists so only nearby rows are rendered.
- Dependencies / cross-references: GPUI scroll handle, retained row measurements, estimated heights and viewport bounds.
- Implementation / logic: Plans visible ranges with spacers for hidden rows, records measured heights, handles scroll jumps/resizes/appends and reuses nearby rows.
- Findings: Tests/measurements target desktop GPUI; virtualization logic must be adapted to Android UI toolkit and touch scrolling.
- RustChunk relevance: Useful for long mod/server/world catalogs to reduce UI element count and memory.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Use equivalent Recycler/LazyColumn-style virtualization in RustChunk Android screens; don't port GPUI element code.
- Follow-up checks: Test variable row heights, fast fling, large jumps, dynamic insertion, accessibility focus and scroll restoration.
- RustChunk implementation mapping: RustChunk mobile mod/server/world list rendering.
- Evidence required before adoption: Android frame-time and memory profiling on target phone with 10k+ rows.
- Completion state: COMPLETE — 355 logical lines through EOF; pinned source SHA and 12677 byte size match; reviewed, not modified; tests not executed.

### 2923. `launcher/src/datapacks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ddc40cf83dc7e4fb0e4e8f322c1aa47a75d5d043`
- Size: 8834 bytes
- Coverage: 1–240; complete EOF; 240 canonical logical lines, no gaps.
- Purpose: Lists, validates, installs and removes datapack ZIPs/folders for a saved world; estimates feature coverage.
- Dependencies / cross-references: Minecraft world datapacks folder, datapacks.json, ZIP archive reader, trash service and pack metadata.
- Implementation / logic: Recognizes pack.mcmeta and data/ paths, chooses collision-free names, tracks disabled packs, removes through OS trash and counts resource kinds; labels only functions, function tags, predicates and item modifiers as currently executed.
- Findings: Its coverage model is intentionally narrow and current game-specific; 1.20.1 datapack formats and enabled/disabled semantics must be checked against target version. ZIP validation only checks basic structure.
- RustChunk relevance: World datapack management is relevant to RustChunk profile/world tooling; not central to Forge JAR loading.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Implement 1.20.1 pack format compatibility and safe install for datapacks separately from Forge mods; preserve original files and clearly show unsupported contents.
- Follow-up checks: Test malformed ZIP, zip bombs, duplicate names, path traversal, symlinks, datapacks.json corruption, 1.20.1 pack_format, reload and trash restore.
- RustChunk implementation mapping: RustChunk world datapack browser and coverage report.
- Evidence required before adoption: 1.20.1 DataPackLoader oracle, Forge datapack reload, adversarial archive tests and world save/restore.
- Completion state: COMPLETE — 240 logical lines through EOF; pinned source SHA and 8834 byte size match; reviewed, not modified; tests not executed.

### 2924. `launcher/src/datapacks/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f5695c70607d58a270d19913081a77403e78102c`
- Size: 4454 bytes
- Coverage: 1–138; complete EOF; 138 canonical logical lines, no gaps.
- Purpose: Tests datapack discovery, enabled state, adding, collision naming, removing and coverage summaries.
- Dependencies / cross-references: datapacks module, ZIP writer/reader, datapacks.json and trash seam.
- Implementation / logic: Creates temporary world folders and ZIP fixtures; verifies enabled/disabled state, pack filtering, resource-pack rejection, trash movement and coverage counts.
- Findings: Fixture pack_format 88 and examples are aligned with later game version, not 1.20.1; tests are source only and were not executed in this audit.
- RustChunk relevance: Useful testing pattern, but fixtures need replacement with actual 1.20.1 formats.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Create 1.20.1-specific fixtures and Forge reload tests; preserve collision/trash tests with Android storage semantics.
- Follow-up checks: Test actual 1.20.1 pack_format and data registries, disabled packs, folder packs, malformed archives and SAF/storage permission errors.
- RustChunk implementation mapping: RustChunk datapack test suite.
- Evidence required before adoption: Official 1.20.1 pack metadata, Forge datapack loading and real device filesystem tests.
- Completion state: COMPLETE — 138 logical lines through EOF; pinned source SHA and 4454 byte size match; reviewed, not modified; tests not executed.

### 2925. `launcher/src/datapacks_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `104fb5485e9429776f22d9175371dfe44ba7609d`
- Size: 10417 bytes
- Coverage: 1–268; complete EOF; 268 canonical logical lines, no gaps.
- Purpose: Adds a per-world datapack panel with picker/drop support, list, confirmation and trash removal.
- Dependencies / cross-references: GPUI UI, World model, datapacks service, file picker and external drag/drop.
- Implementation / logic: Opens/closes panel, refreshes list, copies selected/dropped ZIPs, displays add result and asks before moving a pack to trash.
- Findings: Desktop file picker and drag/drop UX do not translate directly to Android; multi-file add can partially mutate filesystem before collect returns first error.
- RustChunk relevance: Useful interaction flow and confirmation pattern; needs Android SAF picker and explicit partial-failure reporting.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Implement Android document picker/content URI handling and transactionally copy/validate each pack; show per-file result.
- Follow-up checks: Test canceled picker, mixed valid/invalid multi-select, duplicate names, partial failure cleanup, scoped storage and remove confirmation.
- RustChunk implementation mapping: RustChunk world datapack management UI.
- Evidence required before adoption: Android 13 SAF/URI permission tests, partial rollback and 1.20.1 datapack launch tests.
- Completion state: COMPLETE — 268 logical lines through EOF; pinned source SHA and 10417 byte size match; reviewed, not modified; tests not executed.

### 2926. `launcher/src/desktop.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5431c7b52a994bbccf892ff72296335738a48bd1`
- Size: 14261 bytes
- Coverage: 1–356; complete EOF; 356 canonical logical lines, no gaps.
- Purpose: Installs, repairs and removes Linux desktop-entry files and application icons for the Horizon launcher/game.
- Dependencies / cross-references: XDG data directories, Linux .desktop metadata, packaged icons, AppImage/wrapper executable paths and PATH lookup.
- Implementation / logic: Writes launcher/game desktop entries, quotes Exec paths, detects stale TryExec/Exec targets, follows moved binaries and avoids overwriting a working external entry.
- Findings: Linux desktop integration only; no Android package/game classification logic. XDG and .desktop semantics are irrelevant to Samsung Gaming Hub.
- RustChunk relevance: No Android implementation; useful only if RustChunk later ships a Linux desktop launcher.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Do not use this as a substitute for Android manifest/game-category integration; handle Android app classification with platform APIs.
- Follow-up checks: Test Android launcher manifest and Samsung Game Hub separately; Linux-only code must be cfg-gated in any shared workspace.
- RustChunk implementation mapping: No Android mapping; optional desktop distribution integration.
- Evidence required before adoption: Android manifest/category behavior on actual Samsung device; Linux packaging tests if desktop scope is added.
- Completion state: COMPLETE — 356 logical lines through EOF; pinned source SHA and 14261 byte size match; reviewed, not modified; tests not executed.

### 2927. `launcher/src/desktop_seams.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8767bfdb7c240cffdf3a1d4c660e31a8507e9e51`
- Size: 4878 bytes
- Coverage: 1–110; complete EOF; 110 canonical logical lines, no gaps.
- Purpose: Provides test seams for OS trash, opening folders and dispatching blocking work on its own thread.
- Dependencies / cross-references: Desktop filesystem integration, GPUI clipboard and platform trash service.
- Implementation / logic: Routes trash operations through configurable test folder or platform trash, opens folders, runs blocking closure on worker thread and converts panic to error.
- Findings: OS trash/clipboard/window integration is desktop-oriented; thread panic conversion is useful but platform seam must be redesigned for Android.
- RustChunk relevance: Potentially useful abstraction for safe file deletion and blocking work; not directly portable.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Implement Android SAF/MediaStore deletion and coroutine/worker scheduling; never permanently delete without user confirmation where recovery is expected.
- Follow-up checks: Test Android document URI permissions, trash availability, cancellation, worker panic/error propagation and scoped storage.
- RustChunk implementation mapping: RustChunk Android filesystem service and background task executor.
- Evidence required before adoption: Device-level SAF tests, recovery tests and lifecycle/cancellation checks.
- Completion state: COMPLETE — 110 logical lines through EOF; pinned source SHA and 4878 byte size match; reviewed, not modified; tests not executed.

### 2928. `launcher/src/dev_options.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a2ade17d7797113ded7311527eaa0a942d49eb13`
- Size: 2410 bytes
- Coverage: 1–62; complete EOF; 62 canonical logical lines, no gaps.
- Purpose: Renders development-only launcher version actions.
- Dependencies / cross-references: GPUI launcher state, release download and version selection.
- Implementation / logic: Provides dev-only click handler and settings section for switching/installing development builds.
- Findings: Desktop-only dev UI; no Forge or Android deployment semantics.
- RustChunk relevance: Low-value UI reference; release channel separation may be useful.
- Decision: unnecessary; reference; development-only
- 1.20.1 implications: Keep RustChunk development builds isolated from stable profiles; don't port the GPUI widget.
- Follow-up checks: Test debug/release channel separation, downgrade prevention and profile compatibility.
- RustChunk implementation mapping: RustChunk build channel settings if required.
- Evidence required before adoption: Signed test APK channel, rollback protection and profile migration tests.
- Completion state: COMPLETE — 62 logical lines through EOF; pinned source SHA and 2410 byte size match; reviewed, not modified; tests not executed.

### 2929. `launcher/src/export.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c1a1b9e45f741118c956396cf4e9ca068fae9bf2`
- Size: 10161 bytes
- Coverage: 1–286; complete EOF; 286 canonical logical lines, no gaps.
- Purpose: Exports an instance and selected worlds as a ZIP or Modrinth mrpack while excluding private player data.
- Dependencies / cross-references: Installed mod index, Modrinth file metadata, Minecraft NBT level.dat, world saves and archive writer.
- Implementation / logic: Copies selected settings/folders, builds mrpack index with linkable verified downloads, strips player UUID from level.dat when possible, omits playerdata/stats/advancements/session.lock and skips dotfiles.
- Findings: Export folder allowlist is Horizon-centric (`horizon-mods` etc.), not Forge's complete mod/config layout; if NBT redaction fails it exports original level.dat unchanged.
- RustChunk relevance: Useful export/import architecture and privacy-by-default rules; requires Forge 1.20.1 file list and robust NBT privacy guarantees.
- Decision: adaptable; reference; privacy-sensitive
- 1.20.1 implications: Build a Forge-aware export format that preserves mods/config/resourcepacks while excluding credentials, tokens and per-player secrets; never silently include raw player identity if redaction fails.
- Follow-up checks: Test ZIP traversal, symlinks, oversized files, world metadata redaction failure, unknown files, Forge configs, mod dependencies and round-trip imports.
- RustChunk implementation mapping: RustChunk instance/modpack export and backup service.
- Evidence required before adoption: 1.20.1 world NBT oracle, Forge modpack round-trip, privacy fixtures and archive fuzzing.
- Completion state: COMPLETE — 286 logical lines through EOF; pinned source SHA and 10161 byte size match; reviewed, not modified; tests not executed.

### 2930. `launcher/src/export/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3c4843d08f3ba6c4ce7e48bffad4f363cf3afb21`
- Size: 7597 bytes
- Coverage: 1–214; complete EOF; 214 canonical logical lines, no gaps.
- Purpose: Tests instance/world export content, private-data exclusion and mrpack links for Modrinth-hosted resources.
- Dependencies / cross-references: export module, ZIP archives, world level.dat NBT and Modrinth file source.
- Implementation / logic: Builds temp instance and world fixtures; verifies selected world inclusion, settings, exclusion of private player files, Modrinth-linked pack files and checksum/index structure.
- Findings: Tests target Horizon 26.3 and current folder allowlist; no 1.20.1 Forge modpack round-trip or Android storage test.
- RustChunk relevance: Useful privacy and archive test pattern; adapt fixtures to Forge and 1.20.1.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Create tests from actual RustChunk Forge profile layout, including mods, config, defaultconfigs, kubejs and world saves as appropriate.
- Follow-up checks: Test corrupt NBT fallback, identity leakage, zip-slip, symlink traversal, duplicate names, missing Modrinth source and large worlds.
- RustChunk implementation mapping: RustChunk export/import regression suite.
- Evidence required before adoption: Round-trip 1.20.1 Forge pack, privacy scan and Android filesystem tests.
- Completion state: COMPLETE — 214 logical lines through EOF; pinned source SHA and 7597 byte size match; reviewed, not modified; tests not executed.

### 2931. `launcher/src/export_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f13f224371b912ef40422915d42f8a19d3623da9`
- Size: 8621 bytes
- Coverage: 1–227; complete EOF; 227 canonical logical lines, no gaps.
- Purpose: Implements desktop instance export panel and asynchronous export workflow.
- Dependencies / cross-references: GPUI, instance selection, export module and filesystem path picker.
- Implementation / logic: Selects ZIP/mrpack format, worlds and output path, runs export asynchronously and displays result/error.
- Findings: Desktop GPUI file picker and layout are not Android UI/storage APIs.
- RustChunk relevance: Workflow concept is reusable, presentation code is not.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Use Android document picker/create-document and scoped storage; keep export service independent from UI.
- Follow-up checks: Test canceled picker, output URI, background task lifecycle, large file progress and failure cleanup.
- RustChunk implementation mapping: RustChunk mobile export screen and export service.
- Evidence required before adoption: Android SAF tests and large Forge modpack export/import.
- Completion state: COMPLETE — 227 logical lines through EOF; pinned source SHA and 8617 byte size match; reviewed, not modified; tests not executed.

### 2932. `launcher/src/frame.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4f46a424e1b242722b937357294ae1a879723702`
- Size: 38152 bytes
- Coverage: 1–400, 401–800, 801–941; complete EOF; 941 canonical logical lines, no gaps.
- Purpose: Composes the desktop Glass launcher frame, sidebar, topbar, account menu and instance list interactions.
- Dependencies / cross-references: GPUI, glass_ui, instance manager, account state, theme, scroll/hover state and navigation actions.
- Implementation / logic: Builds custom glass-style desktop shell, instance rows, menus, account selector, badges and action handlers; maintains hover and measurement state.
- Findings: Presentation-only desktop UI; tightly coupled to GPUI and Horizon's design system. Does not implement Android app lifecycle, game recognition or Forge execution.
- RustChunk relevance: Can inform information architecture but should not be transplanted as UI source.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Design a touch-first Android UI with native Compose/egui/selected Rust UI framework; keep UI separate from launcher/core services.
- Follow-up checks: Test small-screen layout, touch targets, accessibility, navigation, rotation, lifecycle and low-memory recovery on Galaxy A22 class hardware.
- RustChunk implementation mapping: RustChunk Android launcher navigation and profile screen design.
- Evidence required before adoption: Working Android screens, accessibility checks, touch/rotation tests and device performance.
- Completion state: COMPLETE — 941 logical lines through EOF; pinned source SHA and 38152 byte size match; reviewed, not modified; tests not executed.

### 2933. `launcher/src/game_app.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `82b46e8f49fb523e50bd119f8be8febc41f34ded`
- Size: 7293 bytes
- Coverage: 1–183; complete EOF; 183 canonical logical lines, no gaps.
- Purpose: Creates/repairs a macOS app bundle that links the selected installed version's game programs.
- Dependencies / cross-references: macOS bundle layout, filesystem metadata, plist XML and version directory.
- Implementation / logic: Creates bundle symlinks to game/ctl programs, updates only changed metadata and handles missing executable variants; XML-escapes plist strings.
- Findings: macOS-specific packaging; does not classify Android apps as games or influence Samsung Gaming Hub.
- RustChunk relevance: No Android implementation; packaging separation is conceptually useful.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Do not infer Android game classification from this; implement package manifest and Samsung game service compatibility separately.
- Follow-up checks: Check Android manifest category, intent filters, package behavior and actual Gaming Hub recognition.
- RustChunk implementation mapping: No direct Android mapping.
- Evidence required before adoption: Install APK on Samsung device and verify Gaming Hub detects it as a game, not just an icon.
- Completion state: COMPLETE — 183 logical lines through EOF; pinned source SHA and 7293 byte size match; reviewed, not modified; tests not executed.

### 2934. `launcher/src/game_log.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d671f27efb422cd5b08c51885f658d19cff7489`
- Size: 5696 bytes
- Coverage: 1–159; complete EOF; 159 canonical logical lines, no gaps.
- Purpose: Scrubs, truncates, uploads and deletes Minecraft logs shared through mclo.gs.
- Dependencies / cross-references: Launcher account names/offline names, home-directory paths, game log contents and mclo.gs paste API.
- Implementation / logic: Removes home path and whole-word player names, limits payload to 25k lines/10 MiB, uploads and stores deletion token for authenticated delete.
- Findings: Scrubber only knows configured names/home paths; other secrets, server IPs, access tokens and arbitrary paths may remain. Remote paste is readable by anyone with URL.
- RustChunk relevance: Privacy-sensitive feature; log viewer useful, third-party upload should remain explicit opt-in and local-only default.
- Decision: adaptable; reference; privacy-sensitive
- 1.20.1 implications: Implement robust Android log redaction and never upload automatically; keep logs local unless user explicitly chooses share.
- Follow-up checks: Fuzz scrubber with OAuth tokens, UUIDs, IPs, server addresses, Windows/Android paths, chat, mod configs and multiline secrets; test delete token lifecycle.
- RustChunk implementation mapping: RustChunk local diagnostics and optional user-confirmed log sharing.
- Evidence required before adoption: Adversarial secret corpus, independent redaction review, endpoint privacy review and verified deletion.
- Completion state: COMPLETE — 159 logical lines through EOF; pinned source SHA and 5696 byte size match; reviewed, not modified; tests not executed.

### 2935. `launcher/src/game_log/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b353df173abe400aac16ae299c367be2d26e9bc2`
- Size: 4109 bytes
- Coverage: 1–114; complete EOF; 114 canonical logical lines, no gaps.
- Purpose: Tests log search, privacy redaction, truncation and mclo.gs upload/delete API behavior.
- Dependencies / cross-references: game_log module and mocked HTTP server via environment override.
- Implementation / logic: Asserts names/home paths are removed, long logs send tail, and upload/delete requests use returned token.
- Findings: Fixtures cover only selected name/path cases; do not prove arbitrary token or server metadata redaction. Tests were not executed in this audit.
- RustChunk relevance: Useful baseline for RustChunk diagnostics tests; needs much broader sensitive-data fixtures.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Expand tests to include Minecraft auth/session artifacts and mod-specific secrets; preserve explicit consent before network transfer.
- Follow-up checks: Add property/fuzz tests, URL/token leakage checks, upload failure and expired/deleted paste cases.
- RustChunk implementation mapping: RustChunk log privacy test suite.
- Evidence required before adoption: Adversarial fixtures, fuzzing, no-network default and successful deletion proof.
- Completion state: COMPLETE — 114 logical lines through EOF; pinned source SHA and 4109 byte size match; reviewed, not modified; tests not executed.

### 2936. `launcher/src/game_tools.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `28e85e2ab880f68bd248f6a7bfa14ed3f89a33a3`
- Size: 7660 bytes
- Coverage: 1–218; complete EOF; 218 canonical logical lines, no gaps.
- Purpose: Exposes in-game Horizon MCP tools through a local child-process/stdin-stdout RPC channel.
- Dependencies / cross-references: horizon-launcher backend, horizon-client process, JSON-RPC-like tool requests and child stdin/stdout.
- Implementation / logic: Starts/connects to local game process, lists/calls prefixed tools, sends serialized requests and explains unreachable client state.
- Findings: Process and MCP protocol are Horizon-specific; local IPC authentication, framing, child lifecycle and untrusted tool execution need review before adapting.
- RustChunk relevance: Optional automation interface only; not required for Minecraft 1.20.1 Forge launcher functionality.
- Decision: unnecessary; reference; security-sensitive
- 1.20.1 implications: Do not include agent/MCP control in baseline RustChunk; if later requested, make it opt-in and local-only with strict allowlist.
- Follow-up checks: Test spoofed local endpoints, malformed/oversized frames, child exit, concurrent requests, token/session isolation and Android IPC permissions.
- RustChunk implementation mapping: Future optional RustChunk diagnostics/automation IPC, not baseline.
- Evidence required before adoption: Threat model, Android local IPC design, explicit permissions and adversarial protocol tests.
- Completion state: COMPLETE — 218 logical lines through EOF; pinned source SHA and 7660 byte size match; reviewed, not modified; tests not executed.

### 2937. `launcher/src/github.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `96222a8836040549d59ea0904f972bb7d3da61ce`
- Size: 8926 bytes
- Coverage: 1–234; complete EOF; 234 canonical logical lines, no gaps.
- Purpose: Fetches GitHub API JSON with ETag caching, rate-limit handling, optional token and Horizon-only mirror fallback.
- Dependencies / cross-references: ureq HTTP client, GitHub API, local cache, ETag metadata, optional HORIZON_GITHUB_TOKEN and mirror service.
- Implementation / logic: Uses cached ETag/response, handles 304, detects 403/429 rate limits, falls back to a mirror only for Horizon-owned repository and stores responses.
- Findings: Mirror URL and cache trust are Horizon-specific; token comes from environment and desktop cache paths; cached data integrity/provenance must be independently verified.
- RustChunk relevance: HTTP caching/rate-limit logic is useful, but mirror and token assumptions must not be reused blindly.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Use official GitHub endpoints or an explicitly trusted RustChunk mirror; verify downloaded artifacts with signed metadata and keep tokens in Android secure storage.
- Follow-up checks: Test poisoned cache, ETag mismatch, mirror substitution, rate-limit response variants, token leakage, TLS and offline behavior.
- RustChunk implementation mapping: RustChunk release/update metadata client.
- Evidence required before adoption: Signed release manifests, secure Android secret storage, cache integrity tests and trusted mirror ownership.
- Completion state: COMPLETE — 234 logical lines through EOF; pinned source SHA and 8926 byte size match; reviewed, not modified; tests not executed.

### 2938. `launcher/src/github/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b13440b7ff87d86cc94d5c7b88d67804b87454dd`
- Size: 8235 bytes
- Coverage: 1–278; complete EOF; 278 canonical logical lines, no gaps.
- Purpose: Tests GitHub mirror selection, ETag cache behavior, rate-limit handling and corrupt cache rejection.
- Dependencies / cross-references: github module, fake HTTP responses, local cache and mirror URL parser.
- Implementation / logic: Tests mirror limited to Horizon repository, 304 cache reuse, 403/429 detection, 404 no retry, stale cache fallback and corrupt/foreign cache rejection.
- Findings: Tests verify Horizon-specific mirror policy and do not validate RustChunk endpoint ownership or artifact signing; not executed here.
- RustChunk relevance: Useful adversarial HTTP/cache tests for RustChunk update system.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Adapt mirror tests to RustChunk's own infrastructure and ensure no untrusted cache can supply installable APKs.
- Follow-up checks: Test malicious mirror, cache tampering, stale signed manifest, downgrade and cross-repository confusion.
- RustChunk implementation mapping: RustChunk update client test suite.
- Evidence required before adoption: Signed APK/update metadata, poisoned-cache tests, offline behavior and TLS validation.
- Completion state: COMPLETE — 278 logical lines through EOF; pinned source SHA and 8235 byte size match; reviewed, not modified; tests not executed.

### 2939. `launcher/src/github_mods.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7b474337096ec5eafe6963712ff6ce182a54d035`
- Size: 24562 bytes
- Coverage: 1–400, 401–685; complete EOF; 685 canonical logical lines, no gaps.
- Purpose: Resolves GitHub repositories/releases/commits and packages Horizon-native Rust mods into .hnmod bundles for installation.
- Dependencies / cross-references: GitHub API, Horizon native-mod manifest/index, source archive, SHA-256, native_mods installer and pinned Horizon revision/hook points.
- Implementation / logic: Validates GitHub URL/ref, resolves release or commit, downloads .hnmod or codeload tarball, caps assets/archives, checks hashes/manifest pins, packages source and installs with origin metadata.
- Findings: This is Horizon-native Rust plugin code with full access to host computer; warning is explicit. It is not Forge Java mod support, and its native mod API/hook points are proprietary/version-bound.
- RustChunk relevance: Concepts such as pinned provenance, source review, checksums and bounded archives are reusable; plugin runtime is not.
- Decision: unnecessary; reference; security-sensitive; version-specific
- 1.20.1 implications: Do not port `.hnmod` or native hook system as a Forge substitute. RustChunk needs actual Forge loader/runtime and Java JAR handling.
- Follow-up checks: Audit archive traversal/symlink limits, checksum availability, API rate limits, native code permissions, dependency pinning and Android execution sandbox.
- RustChunk implementation mapping: No direct Forge mapping; possible future signed Rust extension plugin only if explicitly scoped.
- Evidence required before adoption: Threat model, Android ABI/native library compatibility, signed plugin manifests, source review and real Forge mod execution.
- Completion state: COMPLETE — 685 logical lines through EOF; pinned source SHA and 24562 byte size match; reviewed, not modified; tests not executed.

### 2940. `launcher/src/github_mods/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2fec216e2ba921a547053bee8646d96144303e99`
- Size: 13739 bytes
- Coverage: 1–400, 401–409; complete EOF; 409 canonical logical lines, no gaps.
- Purpose: Tests GitHub native-mod URL parsing, release/commit resolution, archive packing, source pin enforcement and install/update origin tracking.
- Dependencies / cross-references: github_mods module, GitHub API fixture, tar/ZIP archives, Horizon native registry manifest and local install folder.
- Implementation / logic: Covers invalid hosts/refs, rate limit, release fallback, commit pin requirements, source-only bundle packaging and update following.
- Findings: Tests only Horizon `.hnmod` and `horizon-mixin.toml` semantics; no Forge JAR classloading or Java compatibility.
- RustChunk relevance: Useful tests for trusted source pinning and archive validation, but native-mod model is not RustChunk's mod architecture.
- Decision: unnecessary; reference; security-sensitive
- 1.20.1 implications: Do not infer Forge compatibility from successful native-mod tests; create separate Java Forge fixture suite.
- Follow-up checks: Test archive traversal, symlink escape, malformed manifest, hash mismatch, stale commit, malicious build scripts and Android native code isolation.
- RustChunk implementation mapping: RustChunk provenance/archive security tests only.
- Evidence required before adoption: Forge 47.x mod JARs and actual client launch; malicious archive fuzzing.
- Completion state: COMPLETE — 409 logical lines through EOF; pinned source SHA and 13739 byte size match; reviewed, not modified; tests not executed.

### 2941. `launcher/src/github_mods_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4476d962b6b615e0f3a23263d8325799a5b2e78`
- Size: 21162 bytes
- Coverage: 1–400, 401–548; complete EOF; 548 canonical logical lines, no gaps.
- Purpose: Implements desktop UI for adding GitHub-hosted Horizon-native mods, reviewing resolved source and choosing/installing bundles.
- Dependencies / cross-references: GPUI forms, github_mods resolver, native_mods installer, release/ref picker and async background tasks.
- Implementation / logic: Lets user resolve URL, inspect origin/revision, choose among multiple mods, install and update/remove GitHub-native bundles.
- Findings: Desktop GPUI and Horizon native-mod ecosystem only; UI does not load Forge Java mods.
- RustChunk relevance: May inform source-review UX for a future plugin manager; not part of baseline.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Keep native extension review distinct from Forge mod management; do not label GitHub Rust mods as Forge mods.
- Follow-up checks: Test touch flow only if reused, stale async results, duplicate installs, install cancellation and malicious source warnings.
- RustChunk implementation mapping: Potential future RustChunk extension browser, separate from Forge mod browser.
- Evidence required before adoption: Android UX prototype, source provenance display and permission/security review.
- Completion state: COMPLETE — 548 logical lines through EOF; pinned source SHA and 21155 byte size match; reviewed, not modified; tests not executed.

### 2942. `launcher/src/glass.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `69f9ba96332e55fd2d4b83576d6fd9bb9179ac42`
- Size: 10954 bytes
- Coverage: 1–281; complete EOF; 281 canonical logical lines, no gaps.
- Purpose: Implements custom desktop glass visual primitives, backdrop capture and nine-slice image painting.
- Dependencies / cross-references: GPUI canvas/render image, Glass theme, scene backdrop model and texture sampling.
- Implementation / logic: Builds glass plates, tabs, overlays and nine-slice pieces using measured backdrop and image resources.
- Findings: Custom desktop renderer/UI toolkit; not a Minecraft renderer and no Android GLES implementation.
- RustChunk relevance: Pure presentation reference; no reason to import this graphics layer.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Do not confuse launcher glass UI with Minecraft game rendering; choose Android UI toolkit separately.
- Follow-up checks: If adapting aesthetic, test GPU cost, transparency, accessibility, contrast and Mali GLES performance.
- RustChunk implementation mapping: No core RustChunk renderer mapping; optional launcher visual style only.
- Evidence required before adoption: Android visual prototype and Mali GPU profiling.
- Completion state: COMPLETE — 281 logical lines through EOF; pinned source SHA and 10954 byte size match; reviewed, not modified; tests not executed.

### 2943. `launcher/src/glass_home.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fc354fbe2527cb09af33c41bdade0534cf4cf078`
- Size: 31205 bytes
- Coverage: 1–400, 401–792; complete EOF; 792 canonical logical lines, no gaps.
- Purpose: Composes desktop Glass Home screen, Play actions, profile/version selector, folder groups and join-friend flow.
- Dependencies / cross-references: GPUI, backend launch commands, instance manager, account/session, versions, folder grouping and theme.
- Implementation / logic: Builds Home tabs, profile selection, play group, folder picker, launch command construction and join-friend inputs.
- Findings: UI tied to desktop windowing and Horizon 26.3 profile semantics; no Forge installer or Android lifecycle.
- RustChunk relevance: Useful as product-flow reference, not portable code.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Design an Android-first Home screen with clear Forge profile selection, offline/online auth status and launch readiness checks.
- Follow-up checks: Test touch targets, profile changes, launch cancellation, account switching, server address validation and lifecycle resume.
- RustChunk implementation mapping: RustChunk home/profile selection screen and launch state model.
- Evidence required before adoption: Actual Android UX, 1.20.1 Forge profile launch and multiplayer join.
- Completion state: COMPLETE — 792 logical lines through EOF; pinned source SHA and 31205 byte size match; reviewed, not modified; tests not executed.

### 2944. `launcher/src/glass_home_lists.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f6f4364a37a85112e386f3ddb263251705985390`
- Size: 18184 bytes
- Coverage: 1–400, 401–465; complete EOF; 465 canonical logical lines, no gaps.
- Purpose: Renders desktop world and multiplayer server lists, row menus, add/drop interactions and destructive actions.
- Dependencies / cross-references: GPUI, worlds model, server list, drag/drop, filesystem and list action handlers.
- Implementation / logic: Builds world/server rows, context menus, add flows and danger confirmations; integrates dropped files and selection state.
- Findings: Desktop drag/drop and mouse-hover patterns do not map directly to Android touch/SAF.
- RustChunk relevance: List UX and confirmation patterns reusable; implementation is platform-specific.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Use Android touch list with explicit edit/delete confirmation and keyboard accessibility.
- Follow-up checks: Test large lists, server address parsing, duplicate worlds, touch long-press, accessibility and state refresh.
- RustChunk implementation mapping: RustChunk worlds/server list screens.
- Evidence required before adoption: Android touch tests, server ping/connect integration and large-list profiling.
- Completion state: COMPLETE — 465 logical lines through EOF; pinned source SHA and 18170 byte size match; reviewed, not modified; tests not executed.

### 2945. `launcher/src/glass_home_mods.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6955f749c0521d6b075ee5936bfb3765d49b11ff`
- Size: 8564 bytes
- Coverage: 1–226; complete EOF; 226 canonical logical lines, no gaps.
- Purpose: Renders installed Horizon-native mod rows and find-mods action on desktop Home.
- Dependencies / cross-references: GPUI, instance mod rows and GitHub native-mod browser.
- Implementation / logic: Displays native mod status and actions in per-instance list.
- Findings: This is Horizon-native Rust extension inventory, not Forge Java mod inventory.
- RustChunk relevance: Not a source for Forge mod listing; only generic row UX reusable.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Build Forge JAR inventory from mods folder and loader metadata; keep RustChunk-native plugins separate if ever added.
- Follow-up checks: Test mod metadata, duplicate mod IDs, disabled JARs, Forge dependency conflicts and profile-specific folders.
- RustChunk implementation mapping: RustChunk Forge mod list UI, redesigned around Java JAR metadata.
- Evidence required before adoption: Actual Forge 1.20.1 mod corpus and loader compatibility.
- Completion state: COMPLETE — 226 logical lines through EOF; pinned source SHA and 8564 byte size match; reviewed, not modified; tests not executed.

### 2946. `launcher/src/glass_home_packs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dbc189a8ed68d59e7341eb4f4b835ecc9d462de8`
- Size: 12851 bytes
- Coverage: 1–317; complete EOF; 317 canonical logical lines, no gaps.
- Purpose: Renders desktop resource/shader packs and screenshots with add/drop/remove controls.
- Dependencies / cross-references: GPUI, packs service, screenshot paths, drag/drop and file operations.
- Implementation / logic: Lists packs/screenshots, supports add, toggle/remove and open-folder actions.
- Findings: Desktop file paths, drag/drop and current pack formats are not Android SAF; shader pack format is Horizon-specific in places.
- RustChunk relevance: Pack management UX may be adapted; format support must remain Minecraft 1.20.1-compatible.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Implement standard 1.20.1 resource packs and compatible shader ecosystem; do not substitute Horizon WGSL shaders for Iris/OptiFine.
- Follow-up checks: Test pack metadata, zip safety, resource reload, shader compatibility and Android file picker.
- RustChunk implementation mapping: RustChunk resource-pack/screenshot screens.
- Evidence required before adoption: 1.20.1 resource pack reload, Mali GLES shader compatibility and Android SAF tests.
- Completion state: COMPLETE — 317 logical lines through EOF; pinned source SHA and 12851 byte size match; reviewed, not modified; tests not executed.

### 2947. `launcher/src/glass_home_tabs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f9705ee01a873288f41cd3ce50aa64d9b4761ec2`
- Size: 14982 bytes
- Coverage: 1–384; complete EOF; 384 canonical logical lines, no gaps.
- Purpose: Builds desktop Home overview tabs and formats release notes into sections.
- Dependencies / cross-references: GPUI HomeTab state, release body markdown and links.
- Implementation / logic: Renders overview, screenshots, patch log sections and strips references/thanks from release-note body.
- Findings: Desktop-only presentation; release-note formatting is not game or Forge behavior.
- RustChunk relevance: Optional UX reference only.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use a compact mobile release-notes view only if RustChunk needs it; preserve source links and avoid dropping attribution without reason.
- Follow-up checks: Test markdown parsing, link safety, accessibility and narrow screen layout.
- RustChunk implementation mapping: Optional RustChunk About/updates screen.
- Evidence required before adoption: Android screenshots, safe links and attribution review.
- Completion state: COMPLETE — 384 logical lines through EOF; pinned source SHA and 14982 byte size match; reviewed, not modified; tests not executed.

### 2948. `launcher/src/glass_instance.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f9fcbd5df57379b6626c04fd299755b4bcc19e43`
- Size: 37162 bytes
- Coverage: 1–400, 401–800, 801–1007; complete EOF; 1007 canonical logical lines, no gaps.
- Purpose: Implements desktop instance creation/edit screen, loader and version selection, icon, profile settings and side panel.
- Dependencies / cross-references: GPUI, instances service, loader registry, version lists, Minecraft version picker and theme.
- Implementation / logic: Builds instance wizard, selects Horizon or Java edition kind, loader/version and icons, creates/edits instance details and displays version list.
- Findings: Java instance flow is Fabric/Quilt-centric; no Forge 47.x path demonstrated. Desktop screen cannot be used on Android unchanged.
- RustChunk relevance: Instance isolation and wizard flow are reusable concepts; loader selection and UI require a new Forge-aware implementation.
- Decision: rewrite required; reference; version-specific; platform-specific
- 1.20.1 implications: Implement profile model with independent game dirs, Forge 1.20.1 installer/version resolution, Java 17 and mod folder per profile.
- Follow-up checks: Test profile isolation, Forge installer failure/rollback, Java runtime compatibility, version JSON libraries, resource reload and account selection.
- RustChunk implementation mapping: RustChunk profile manager and Forge instance wizard.
- Evidence required before adoption: Actual 1.20.1 Forge 47.4.5 install/launch, ARM64 Java 17 runtime, mods and Aternos connection.
- Completion state: COMPLETE — 1007 logical lines through EOF; pinned source SHA and 37146 byte size match; reviewed, not modified; tests not executed.

### 2949. `launcher/src/glass_instance_java.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4dea9fadc8cfbcf00fa9be461160a4627571bc8a`
- Size: 8774 bytes
- Coverage: 1–210; complete EOF; 210 canonical logical lines, no gaps.
- Purpose: Provides desktop Java Edition version and loader choice subpanels.
- Dependencies / cross-references: Minecraft version list, loader registry, Java instance state and GPUI controls.
- Implementation / logic: Renders Minecraft version list and loader selector with desktop state callbacks.
- Findings: Current loader selection is not proof of Forge support; surrounding instance model is Fabric/Quilt-oriented.
- RustChunk relevance: Only version-picker UX concept is reusable.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Add explicit Forge 1.20.1 loader support and installer state; don't silently fall back to Fabric/Quilt.
- Follow-up checks: Test Forge 47.x version list, installer metadata, Java 17 selection and failure recovery.
- RustChunk implementation mapping: RustChunk Forge version/loader picker.
- Evidence required before adoption: Real Forge profile creation and launch on Android.
- Completion state: COMPLETE — 210 logical lines through EOF; pinned source SHA and 8774 byte size match; reviewed, not modified; tests not executed.

### 2950. `launcher/src/glass_ui.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4d5816498330ea940744779c4250afbad8cc974d`
- Size: 18801 bytes
- Coverage: 1–400, 401–628; complete EOF; 628 canonical logical lines, no gaps.
- Purpose: Defines reusable desktop Glass UI primitives for text, buttons, rows, tabs, menus, popovers, lists and form fields.
- Dependencies / cross-references: GPUI, Horizon theme colors, hover tracking, keyboard state and custom component wrappers.
- Implementation / logic: Builds consistent styled primitives and interaction states over GPUI elements.
- Findings: Entire module is bound to desktop GPUI and Horizon theme; no Android accessibility/touch semantics.
- RustChunk relevance: Design-system concepts reusable, source code is not.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Choose Android-native touch/accessibility components and keep style tokens independent from renderer/game core.
- Follow-up checks: Test screen readers, contrast, focus traversal, touch size, localization expansion and low-end GPU rendering.
- RustChunk implementation mapping: RustChunk Android design system concepts only.
- Evidence required before adoption: Android UI implementation and device accessibility/performance review.
- Completion state: COMPLETE — 628 logical lines through EOF; pinned source SHA and 18800 byte size match; reviewed, not modified; tests not executed.

### 2951. `launcher/src/heads.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ce81478cd1fec981ae74d86a4d88f1c8becd9353`
- Size: 9544 bytes
- Coverage: 1–258; complete EOF; 258 canonical logical lines, no gaps.
- Purpose: Builds player head/face textures from Mojang profile skins with UUID-based fallback and cache.
- Dependencies / cross-references: Mojang profile/skin endpoints, account UUID, local image cache and launcher rendering.
- Implementation / logic: Parses UUID, downloads skin/profile textures, extracts face pixels, caches by skin identity and falls back to deterministic UUID-derived default skin when profile unavailable.
- Findings: Skin services and image handling are not Forge-specific; online profile requests and cache policy need privacy/offline review. Fallback does not imply authenticated identity.
- RustChunk relevance: Useful for offline-friendly avatar presentation and resilient skin caching, not required for game compatibility.
- Decision: adaptable; reference; privacy-sensitive
- 1.20.1 implications: Implement optional skin retrieval separately from account authentication; offline profile must never be presented as a Mojang-authenticated account.
- Follow-up checks: Test no-network fallback, malformed profile/skin data, cache poisoning, stale skin refresh, UUID normalization and Android storage.
- RustChunk implementation mapping: RustChunk profile avatar/skin cache.
- Evidence required before adoption: Android image decode, Mojang API behavior, privacy policy and offline-mode server tests.
- Completion state: COMPLETE — 258 logical lines through EOF; pinned source SHA and 9544 byte size match; reviewed, not modified; tests not executed.

### 2952. `launcher/src/home.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6812b3b5a6f3aa76eb4a75d8f6f4e31c3e522b5d`
- Size: 22764 bytes
- Coverage: 1–400, 401–579; complete EOF; 579 canonical logical lines, no gaps.
- Purpose: Composes desktop Home hero, launch actions, readiness preview, download progress and friend join controls.
- Dependencies / cross-references: GPUI, launcher shell actions, download state, backend preparation and Home navigation.
- Implementation / logic: Displays preparation steps in order, progress/time estimate and launch/friend actions; tests step ordering and estimated remaining time.
- Findings: Desktop GPUI workflow and Horizon preparation phases; not an Android launcher screen or Forge installer.
- RustChunk relevance: Readiness-step model and progress UX can be adapted; code itself is desktop-specific.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Build Android-first preparation checklist for Java 17, Forge 47.x, libraries, assets, mods and profile validation.
- Follow-up checks: Test interrupted downloads, resume, progress accuracy, cancellation, background lifecycle and low-memory recovery.
- RustChunk implementation mapping: RustChunk Home screen and launch readiness state machine.
- Evidence required before adoption: Actual 1.20.1 Forge profile preparation on ARM64 Android with reliable progress and resume.
- Completion state: COMPLETE — 579 logical lines through EOF; pinned source SHA and 22764 byte size match; reviewed, not modified; tests not executed.

### 2953. `launcher/src/home_tabs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e716fa69e9c440546c2919fd17bc259da1dcc6e6`
- Size: 5216 bytes
- Coverage: 1–132; complete EOF; 132 canonical logical lines, no gaps.
- Purpose: Selects and renders desktop Home tab configuration, including environment overrides.
- Dependencies / cross-references: Launcher environment config, HomeTab enum and GPUI shell.
- Implementation / logic: Reads optional environment override, chooses available tabs and splits display sections.
- Findings: Horizon desktop navigation and environment variables; no game runtime behavior.
- RustChunk relevance: Only tab organization concept reusable.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use RustChunk Android navigation settings independent from desktop environment variables.
- Follow-up checks: Test persisted tab state, process recreation and accessibility.
- RustChunk implementation mapping: RustChunk navigation model.
- Evidence required before adoption: Android navigation lifecycle tests.
- Completion state: COMPLETE — 132 logical lines through EOF; pinned source SHA and 5216 byte size match; reviewed, not modified; tests not executed.

### 2954. `launcher/src/i18n.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `abb4380459babf4af77cfcaccbda26122b62d6fc`
- Size: 6626 bytes
- Coverage: 1–203; complete EOF; 203 canonical logical lines, no gaps.
- Purpose: Loads launcher language catalogs, detects system locale, selects language and resolves translated strings with fallback.
- Dependencies / cross-references: Locale JSON files, serde, global atomic language selection, OnceLock translation table.
- Implementation / logic: Parses locale codes, maps display names, chooses system/explicit locale, caches table and resolves tr/trf interpolation with fallback.
- Findings: Global process-wide state and desktop locale assumptions may be unsuitable for Android configuration changes; source keys are Horizon UI copy.
- RustChunk relevance: Translation lookup/fallback architecture is useful; catalog and locale selection require Android-specific adaptation.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Use RustChunk-owned Persian-first resources; prefer explicit locale state and robust RTL-aware Android UI.
- Follow-up checks: Test unknown locale, system locale changes, fallback, interpolation, plural rules, bidi and concurrent locale changes.
- RustChunk implementation mapping: RustChunk localization engine and resource catalogs.
- Evidence required before adoption: Persian RTL screenshots, locale-switch lifecycle, placeholder/plural tests and Android font coverage.
- Completion state: COMPLETE — 203 logical lines through EOF; pinned source SHA and 6626 byte size match; reviewed, not modified; tests not executed.

### 2955. `launcher/src/i18n/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c9bf66b44d435ffa0623f8ad2eb60e6879bf72da`
- Size: 9348 bytes
- Coverage: 1–233; complete EOF; 233 canonical logical lines, no gaps.
- Purpose: Tests that all code-requested translation strings exist, language order is valid and placeholders are preserved.
- Dependencies / cross-references: i18n parser, source-code scanner, locale JSON files and interpolation syntax.
- Implementation / logic: Scans translation calls including multiline syntax while ignoring comments/quoted code, checks catalog coverage and placeholder validity, tests fallback and locale order.
- Findings: Tests use the inspected Horizon locale catalog; coverage scanner cannot establish translation quality or Android visual fit.
- RustChunk relevance: Strong catalog integrity test pattern; needs RustChunk source-key scan and Persian-specific checks.
- Decision: adaptable; reference; localization
- 1.20.1 implications: Adopt missing-key and placeholder CI checks for RustChunk; add RTL and pluralization validation beyond catalog parity.
- Follow-up checks: Test scanner against Rust syntax edge cases, dynamic keys, nested interpolation, duplicate keys and locale file drift.
- RustChunk implementation mapping: RustChunk localization CI.
- Evidence required before adoption: Full RustChunk catalog coverage, parser fixtures, Persian native review and device screenshots.
- Completion state: COMPLETE — 233 logical lines through EOF; pinned source SHA and 9348 byte size match; reviewed, not modified; tests not executed.

### 2956. `launcher/src/import.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ad0ca3c01f85bcfe672e010bb95453958d0d5452`
- Size: 8823 bytes
- Coverage: 1–236; complete EOF; 236 canonical logical lines, no gaps.
- Purpose: Recognizes dropped files and imports supported instance exports, modpacks, resource packs and shader packs.
- Dependencies / cross-references: browser download metadata, export layout, instance manager, packs service, ZIP reader and filesystem paths.
- Implementation / logic: Inspects file/container contents, identifies target import kind, validates relative components and imports into instance or pack locations.
- Findings: Horizon 26.3 format/loader model; import paths and resource/shader formats need 1.20.1 Forge adaptation. Archive extraction and symlink safety need deeper review.
- RustChunk relevance: Import classification and path containment are useful; exact format policy cannot be copied unchanged.
- Decision: adaptable; reference; security-sensitive; version-specific
- 1.20.1 implications: Create Forge-aware import routing for .mrpack, CurseForge ZIP, Prism/MultiMC instances and standalone JARs; validate every path and checksum.
- Follow-up checks: Fuzz archive names, zip bombs, symlinks, nested archives, duplicate entries, malformed manifests, partial extraction and SAF URI handling.
- RustChunk implementation mapping: RustChunk import dispatcher and profile import service.
- Evidence required before adoption: Actual Forge 1.20.1 modpacks, malicious archive suite, Android storage permissions and successful launch.
- Completion state: COMPLETE — 236 logical lines through EOF; pinned source SHA and 8823 byte size match; reviewed, not modified; tests not executed.

### 2957. `launcher/src/import/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7865cdba354226c6f6dc1d403cdeb932d500436e`
- Size: 7084 bytes
- Coverage: 1–209; complete EOF; 209 canonical logical lines, no gaps.
- Purpose: Tests import classification and round-trip of exported instance ZIPs while keeping extraction inside target folder.
- Dependencies / cross-references: import/export modules, ZIP fixtures, temp directories and path checks.
- Implementation / logic: Creates fixture archives, detects supported dropped-file types, imports export back as instance and asserts no archive path escapes profile root.
- Findings: Tests reflect current Horizon formats and do not prove 1.20.1 Forge modpack compatibility or Android URI handling.
- RustChunk relevance: Useful path traversal and round-trip test pattern.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Expand with Forge/CurseForge/Prism fixtures and adversarial ZIP entries before enabling user imports.
- Follow-up checks: Test ../ paths, absolute paths, Windows drive paths, symlinks, nested ZIP, decompression bombs, partial failure and duplicate names.
- RustChunk implementation mapping: RustChunk import regression suite.
- Evidence required before adoption: Adversarial archive fuzzing, Android SAF import and real Forge launch.
- Completion state: COMPLETE — 209 logical lines through EOF; pinned source SHA and 7084 byte size match; reviewed, not modified; tests not executed.

### 2958. `launcher/src/import_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d463f6b8ba92b6e8314b2bf4686c44976e493217`
- Size: 5477 bytes
- Coverage: 1–137; complete EOF; 137 canonical logical lines, no gaps.
- Purpose: Handles desktop drag/drop import flow and displays import result.
- Dependencies / cross-references: GPUI ExternalPaths, import service, selected instance and background tasks.
- Implementation / logic: Routes dropped files to import classifier, performs asynchronous import and displays success/error state.
- Findings: Desktop drag/drop and path-based access do not translate to Android SAF/content URIs.
- RustChunk relevance: Workflow concept reusable; UI is platform-specific.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Use Android document picker with explicit import preview and per-file results.
- Follow-up checks: Test canceled selection, multi-file partial failure, duplicate import, content URI lifetime and background interruption.
- RustChunk implementation mapping: RustChunk Android import screen.
- Evidence required before adoption: Android SAF tests and Forge modpack import.
- Completion state: COMPLETE — 137 logical lines through EOF; pinned source SHA and 5477 byte size match; reviewed, not modified; tests not executed.

### 2959. `launcher/src/installer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b48f528aed3215f740e504ae85862d48b9270373`
- Size: 19987 bytes
- Coverage: 1–400, 401–547; complete EOF; 547 canonical logical lines, no gaps.
- Purpose: Installs, updates and uninstalls the Windows desktop launcher, shortcuts and registry/uninstall entries.
- Dependencies / cross-references: Windows registry, shortcut target, executable paths, temporary staging and backend update lifecycle.
- Implementation / logic: Detects installer mode, chooses install folder, writes shortcuts/registry, stages/removes files and runs quiet update operations.
- Findings: Windows-only desktop installer; registry and shortcut APIs are irrelevant to Android APK installation. Unsafe FFI and filesystem ownership need Windows-specific review.
- RustChunk relevance: No direct Android code to reuse; installation/update transaction concepts are generic.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android PackageInstaller/OS-managed APK updates, signed APKs and rollback-safe app data migrations.
- Follow-up checks: Test Android signature continuity, split APK handling, version downgrade, app data migration, storage and update failure recovery.
- RustChunk implementation mapping: RustChunk APK distribution/update flow at platform boundary.
- Evidence required before adoption: Signed release APK, PackageInstaller tests on Android 13 and upgrade/rollback tests.
- Completion state: COMPLETE — 547 logical lines through EOF; pinned source SHA and 19987 byte size match; reviewed, not modified; tests not executed.

### 2960. `launcher/src/instance_icons.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3a8a34d7ed27865aa6965a6c8b6f10917ee8092a`
- Size: 7829 bytes
- Coverage: 1–197; complete EOF; 197 canonical logical lines, no gaps.
- Purpose: Loads and displays instance icons, loader logos and custom pictures in desktop instance rows.
- Dependencies / cross-references: GPUI RenderImage, browser image cache, instance metadata and loader logo assets.
- Implementation / logic: Selects user icon or loader logo, decodes/resizes image and fits it into desktop icon bounds.
- Findings: GPUI image representation and desktop scaling; no Minecraft rendering or Forge runtime behavior.
- RustChunk relevance: Icon selection logic is a minor UI reference.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android image loader and app-specific profile icon resources; do not couple to renderer.
- Follow-up checks: Test malformed image, oversized dimensions, cache eviction and Android bitmap memory pressure.
- RustChunk implementation mapping: RustChunk profile icon presentation.
- Evidence required before adoption: Android bitmap bounds, memory profiling and accessibility descriptions.
- Completion state: COMPLETE — 197 logical lines through EOF; pinned source SHA and 7829 byte size match; reviewed, not modified; tests not executed.

### 2961. `launcher/src/instance_mods.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bc897d419c7e3d869ee3fb260afac3e65d5f23df`
- Size: 26676 bytes
- Coverage: 1–400, 401–691; complete EOF; 691 canonical logical lines, no gaps.
- Purpose: Lists and toggles installed mods per instance, including Horizon-native mods and Java JAR files.
- Dependencies / cross-references: Instance filesystem, native mod registry, Java instance metadata, browser images, mod cache and UI list state.
- Implementation / logic: Tracks filesystem fingerprints, reloads asynchronously, displays mod rows, enables/disables JARs by renaming, and avoids duplicate native registry rows.
- Findings: Three TODO markers remain; Java JAR inventory is basic and does not prove Forge dependency resolution, mod metadata compatibility or safe toggling of locked/open files. Native mod handling is Horizon-specific.
- RustChunk relevance: A per-profile JAR inventory/toggle concept is useful, but Forge mod lifecycle must be rebuilt around actual loader metadata.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Implement robust Forge 1.20.1 mod list from mods folder, parse mod metadata/dependencies, support disabled suffix semantics only if Forge honors it, and avoid modifying active profile during game run.
- Follow-up checks: Inspect TODO sites; test fingerprint races, stale async reads, duplicate IDs, malformed JARs, dependency conflicts, locked files, disabled mods and Forge restart behavior.
- RustChunk implementation mapping: RustChunk Forge mod manager and per-profile mod state.
- Evidence required before adoption: Actual Forge 47.x JAR corpus, mod metadata parser, dependency graph, safe enable/disable and launch regression.
- Completion state: COMPLETE — 691 logical lines through EOF; pinned source SHA and 26676 byte size match; reviewed, not modified; tests not executed.

### 2962. `launcher/src/instances.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d5f56d1c21715bd3fa50d5c4c782832a7eb9f78d`
- Size: 26679 bytes
- Coverage: 1–400, 401–726; complete EOF; 726 canonical logical lines, no gaps.
- Purpose: Defines isolated launcher instances, profile metadata, creation/copy/edit/delete, version pinning and play-time accounting.
- Dependencies / cross-references: Serde instance config, Horizon backend version resolution, Java instance layout, modpacks, filesystem copy and launch command.
- Implementation / logic: Stores per-instance game folders, creates Horizon/Java instances, copies settings/packs, tracks selected profile and play time, follows or pins versions, and moves deleted profiles to trash.
- Findings: Java instance model is tied to Horizon loader support; create_java does not itself prove Forge install/launch. Process command and file copy paths require Android adaptation.
- RustChunk relevance: Per-profile isolation and pin/follow model are highly reusable, but data schema and filesystem model need Android design.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Build RustChunk profile isolation with independent 1.20.1 Forge libraries/mods/config/saves and explicit runtime selection.
- Follow-up checks: Test copy rollback, path collision, rename, delete/trash, selected profile recovery, version pin/follow, corrupted metadata and concurrent launches.
- RustChunk implementation mapping: RustChunk profile/instance data model.
- Evidence required before adoption: Real 1.20.1 Forge profile creation, Java 17 runtime, modpack isolation and Android storage tests.
- Completion state: COMPLETE — 726 logical lines through EOF; pinned source SHA and 26679 byte size match; reviewed, not modified; tests not executed.

### 2963. `launcher/src/instances/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `39923cf3bb4f1e0360ac640f52da20b498672def`
- Size: 17202 bytes
- Coverage: 1–400, 401–425; complete EOF; 425 canonical logical lines, no gaps.
- Purpose: Tests instance naming, copy behavior, version pinning, Java profile layout, mod browser loader targeting and deletion.
- Dependencies / cross-references: instances service, temp filesystem, browser/mod loader, play time and trash integration.
- Implementation / logic: Exercises unique names, profile copies, Java version isolation, play-time accounting, icon validation, mod install target and trash removal.
- Findings: Test data targets 1.21.1 Fabric/Quilt/NeoForge and Horizon versions; no Forge 1.20.1 end-to-end test.
- RustChunk relevance: Good profile isolation test patterns, but fixtures and loader assertions need 1.20.1 Forge replacement.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Build Forge-specific profile fixtures and ensure copies don't share mutable mods/config/saves unexpectedly.
- Follow-up checks: Test simultaneous profiles, crash during copy, path escape, loader mismatch, version pin, Forge installer and delete recovery.
- RustChunk implementation mapping: RustChunk instance regression tests.
- Evidence required before adoption: Forge 47.x profile launch and mod isolation on Android.
- Completion state: COMPLETE — 425 logical lines through EOF; pinned source SHA and 17202 byte size match; reviewed, not modified; tests not executed.

### 2964. `launcher/src/instances_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f72c9ae57f249108d27a04bc0dd9a6fb00378fd6`
- Size: 36511 bytes
- Coverage: 1–400, 401–800, 801–925; complete EOF; 925 canonical logical lines, no gaps.
- Purpose: Renders desktop instance list, creation/edit dialog, profile selection, version pinning and deletion confirmation.
- Dependencies / cross-references: GPUI, instances model, Java loader list, version selector, image cache and theme.
- Implementation / logic: Builds profile rows/dialogs, copy/new flows, version and icon selection, delete confirmation and played-time display.
- Findings: Desktop UI and current Fabric/Quilt loader choices; no Android controls or Forge installer progress flow.
- RustChunk relevance: Product flow is useful but should be redesigned as Android UI.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Build a touch-first RustChunk profile wizard that explicitly selects Minecraft 1.20.1, Forge 47.x and Java 17.
- Follow-up checks: Test orientation/rotation, accessibility, large profile list, duplicate names, cancel/rollback, background install and profile selection persistence.
- RustChunk implementation mapping: RustChunk Android profile manager screen.
- Evidence required before adoption: Device UI tests and successful Forge profile creation/launch.
- Completion state: COMPLETE — 925 logical lines through EOF; pinned source SHA and 36511 byte size match; reviewed, not modified; tests not executed.

### 2965. `launcher/src/internet.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e5ba460bf5f2f7be6732f43285fd58bccd30f224`
- Size: 29591 bytes
- Coverage: 1–400, 401–709; complete EOF; 709 canonical logical lines, no gaps.
- Purpose: Implements desktop Tailscale integration, account token handling, invite links and connectivity policy UI.
- Dependencies / cross-references: Tailscale local API, OAuth/auth tokens, invite flow, network reachability, GPUI settings and state polling.
- Implementation / logic: Reads local Tailscale status, stores/clears auth token, refreshes state, creates friend invites and changes network policy.
- Findings: Tailscale private-network feature is not required for Aternos/public servers; token handling and local API access are security-sensitive.
- RustChunk relevance: Optional private LAN/VPN connectivity idea only; not core launcher functionality.
- Decision: unnecessary; reference; security-sensitive
- 1.20.1 implications: Do not make Tailscale a dependency for joining Aternos; use ordinary Minecraft server address and protocol.
- Follow-up checks: If optional VPN support is later requested, review token storage, invite scope, device authorization and Android VPN permission requirements.
- RustChunk implementation mapping: No baseline mapping; optional networking integration only.
- Evidence required before adoption: Explicit feature scope, Android VPN APIs, token threat model and real connection tests.
- Completion state: COMPLETE — 709 logical lines through EOF; pinned source SHA and 29591 byte size match; reviewed, not modified; tests not executed.

### 2966. `launcher/src/java.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a3bd1492e3f73be1c3bd45e0a8acd43d5d411d59`
- Size: 9348 bytes
- Coverage: 1–275; complete EOF; 275 canonical logical lines, no gaps.
- Purpose: Discovers, selects and installs Mojang Java runtimes required by Minecraft versions.
- Dependencies / cross-references: Mojang version metadata, runtime manifests, OS/architecture rules, Java home folders and download verification.
- Implementation / logic: Reads Java major/component requirement, locates installed runtimes, chooses exact compatible major and downloads runtime component files.
- Findings: Desktop OS runtime selection and Mojang runtime archives; Android ARM64 Java runtime distribution is different. Minecraft 1.20.1 generally requires Java 17, not a generic newest runtime.
- RustChunk relevance: Runtime requirement resolution is important, but runtime acquisition must be Android-specific.
- Decision: rewrite required; reference; platform-specific; version-specific
- 1.20.1 implications: Bundle or provision a compatible ARM64 Java 17 runtime for Android; preserve version metadata and avoid selecting unsupported host runtime.
- Follow-up checks: Test 1.20.1 Java 17 selection, ARM64 ABI, JNI, native library paths, memory limits, TLS and runtime update integrity.
- RustChunk implementation mapping: RustChunk Android Java runtime manager.
- Evidence required before adoption: Real Android ARM64 Java 17 execution of Forge 47.4.5 and native library loading.
- Completion state: COMPLETE — 275 logical lines through EOF; pinned source SHA and 9348 byte size match; reviewed, not modified; tests not executed.

### 2967. `launcher/src/java/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d07e0b21fb343e20344a11f35e82d3645a1bed6`
- Size: 4397 bytes
- Coverage: 1–135; complete EOF; 135 canonical logical lines, no gaps.
- Purpose: Tests Java version requirement parsing, Mojang runtime layout discovery and exact-major selection.
- Dependencies / cross-references: java module, fake runtime home and Mojang metadata fixtures.
- Implementation / logic: Covers Java 8 historical versions, runtime manifest requirements, exact major choice and instance-owned Java override.
- Findings: Desktop runtime fixtures only; no Android Java 17 ARM64 runtime test.
- RustChunk relevance: Useful compatibility selection tests; adapt to Android runtime packages.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Add tests ensuring Forge 1.20.1 selects Java 17 and rejects Java 8/21 when the profile requires 17.
- Follow-up checks: Test component runtime metadata, missing files, wrong architecture and invalid JAVA_HOME.
- RustChunk implementation mapping: RustChunk Java runtime selection tests.
- Evidence required before adoption: Android ARM64 Java 17 package, JNI launch and Forge integration.
- Completion state: COMPLETE — 135 logical lines through EOF; pinned source SHA and 4397 byte size match; reviewed, not modified; tests not executed.

### 2968. `launcher/src/java_launch.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ef17c2e37f0759d3b43ce5c3d410210402c953ce`
- Size: 18593 bytes
- Coverage: 1–400, 401–489; complete EOF; 489 canonical logical lines, no gaps.
- Purpose: Resolves Java Edition version JSON, libraries, natives, assets, placeholders and final game process command.
- Dependencies / cross-references: Mojang version metadata, OS feature/rule filtering, library downloads, asset index, classpath, native extraction and Java process.
- Implementation / logic: Loads version/inheritance metadata, validates library rules, builds classpath, extracts native archives, resolves assets and fills JVM/game arguments including server host/port.
- Findings: This is the strongest reusable Java launch foundation in this batch, but Forge requires installer-generated version JSON, extra libraries, tweakers/launch targets, argument merging and mod-loader handshake. Desktop process execution is not Android runtime integration.
- RustChunk relevance: adaptable; reference; version-specific
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Port version JSON/library/asset semantics, then add a real Forge 47.x installer and launch profile; run through Android Java runtime rather than desktop Command.
- Follow-up checks: Test 1.20.1 inheritance, artifact/native classifiers, OS rules, checksum/size, classpath ordering, arg placeholders, Forge arguments, secure chat/login and Aternos join.
- RustChunk implementation mapping: RustChunk Java launch resolver, library/asset downloader and Forge profile launch bridge.
- Evidence required before adoption: Actual 1.20.1 Forge version JSON, Java 17 ARM64 runtime, classpath parity and real Forge multiplayer test.
- Completion state: COMPLETE — 489 logical lines through EOF; pinned source SHA and 18593 byte size match; reviewed, not modified; tests not executed.

### 2969. `launcher/src/java_launch/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `694a0847592ef5ea02dbd734bc9c8165d7d52d0e`
- Size: 16233 bytes
- Coverage: 1–400, 401–413; complete EOF; 413 canonical logical lines, no gaps.
- Purpose: Tests modern/legacy Java launch metadata, placeholder expansion, loader-profile inheritance and server address arguments.
- Dependencies / cross-references: java_launch module, synthetic version JSON, libraries/native archives, assets and argument resolver.
- Implementation / logic: Covers modern version rules, legacy natives/assets, inheritance cycle refusal, server host/port splitting and loader profile parent resolution.
- Findings: Fixtures are synthetic and primarily test generic loader profile inheritance; do not establish Forge 1.20.1 argument/installer compatibility or actual launch.
- RustChunk relevance: Useful base tests; add actual Forge-generated fixtures.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Create captured/official Forge 47.4.5 metadata fixtures and compare generated JVM/game args with vanilla/Forge launcher output.
- Follow-up checks: Test Forge launch target, argument merge order, library checksums, natives on Android, IPv6/server parsing and auth/session handoff.
- RustChunk implementation mapping: RustChunk Java/Forge launch regression suite.
- Evidence required before adoption: Real Forge 1.20.1 profile, Android runtime, actual modpack and multiplayer server tests.
- Completion state: COMPLETE — 413 logical lines through EOF; pinned source SHA and 16233 byte size match; reviewed, not modified; tests not executed.

### 2970. `launcher/src/language_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `977c783aa989d71ddcb43724070fe21b3a9d8c68`
- Size: 5130 bytes
- Coverage: 1–123; complete EOF; 123 canonical logical lines, no gaps.
- Purpose: Renders desktop language selector and system-language option.
- Dependencies / cross-references: GPUI, i18n Language model and launcher settings.
- Implementation / logic: Displays available language choices and persists selected language.
- Findings: Desktop presentation only; no Android RTL handling or accessibility proof.
- RustChunk relevance: Language choice flow can be reused conceptually.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Implement Android locale settings with Persian as primary language and correct RTL direction.
- Follow-up checks: Test system locale, manual override, process recreation, bidi and accessibility.
- RustChunk implementation mapping: RustChunk language settings screen.
- Evidence required before adoption: Persian Android screenshots and locale lifecycle tests.
- Completion state: COMPLETE — 123 logical lines through EOF; pinned source SHA and 5130 byte size match; reviewed, not modified; tests not executed.

### 2971. `launcher/src/launch.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9723b40bda69ca93c51f22a4ebb0a52021e206a3`
- Size: 19883 bytes
- Coverage: 1–400, 401–542; complete EOF; 542 canonical logical lines, no gaps.
- Purpose: Builds launch-time process plan with memory limits, environment variables, wrappers, pre/post commands and OS-specific execution.
- Dependencies / cross-references: Instance config, system memory detection, shell command parsing, environment variables and process Command.
- Implementation / logic: Computes memory cap from system RAM/explicit setting, resolves executable/wrapper, parses environment, creates pre/post scripts and composes platform-specific command plan.
- Findings: Desktop process/shell assumptions and OS memory sources are not Android; arbitrary shell hooks introduce code-execution risk. Must not pass untrusted profile text to shell.
- RustChunk relevance: Memory-budget and environment planning concepts are useful; execution must be Android-specific and tightly controlled.
- Decision: rewrite required; reference; security-sensitive; platform-specific
- 1.20.1 implications: Use Android memory APIs and bounded JVM heap; avoid arbitrary shell hooks in user-facing mobile profile settings.
- Follow-up checks: Test Android available memory, low-memory kill behavior, shell injection, environment escaping, wrapper paths, process cancellation and post-exit cleanup.
- RustChunk implementation mapping: RustChunk Android Java process launcher and memory budget manager.
- Evidence required before adoption: Galaxy A22 runtime profiling, safe argument construction, Java 17 stability and process lifecycle tests.
- Completion state: COMPLETE — 542 logical lines through EOF; pinned source SHA and 19883 byte size match; reviewed, not modified; tests not executed.

### 2972. `launcher/src/launch/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e59e70b8c14331920581111d6ded5fd3c259623e`
- Size: 9977 bytes
- Coverage: 1–303; complete EOF; 303 canonical logical lines, no gaps.
- Purpose: Tests launch memory settings, environment parsing, per-OS process plans and pre/post command ordering.
- Dependencies / cross-references: launch module, synthetic OS targets, instance variables and child process fixtures.
- Implementation / logic: Checks unset/default memory, user overrides, argument quoting, wrapper ordering, Linux flags and post-exit environment.
- Findings: Host OS tests and desktop command semantics only; no Android heap tuning or lifecycle validation.
- RustChunk relevance: Useful parser and memory-policy test concepts.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Add Android-specific tests for safe JVM args and memory allocation based on actual available RAM.
- Follow-up checks: Test boundary values, invalid env keys, shell metacharacters, Windows quoting, Android process death and low-memory device behavior.
- RustChunk implementation mapping: RustChunk launch configuration and process tests.
- Evidence required before adoption: Actual Galaxy A22 memory tests and Forge 1.20.1 launch.
- Completion state: COMPLETE — 303 logical lines through EOF; pinned source SHA and 9977 byte size match; reviewed, not modified; tests not executed.

### 2973. `launcher/src/launch_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `56784dd13529a805e1bc69343d7d4b255f96c3a9`
- Size: 19490 bytes
- Coverage: 1–400, 401–517; complete EOF; 517 canonical logical lines, no gaps.
- Purpose: Renders desktop launch settings panel for memory, wrappers, pre/post commands and Linux-specific toggles.
- Dependencies / cross-references: GPUI, launch config service, memory settings and desktop environment.
- Implementation / logic: Allows memory selection/typed values and Linux GameMode/MangoHud/dedicated GPU toggles.
- Findings: Desktop-only controls; shell hooks and Linux GPU options do not belong in Android UI.
- RustChunk relevance: Memory settings UX may be adapted to mobile.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Build simple Android heap preset/custom setting with safe upper bounds; omit desktop wrappers.
- Follow-up checks: Test slider bounds, persisted values, device memory, OOM and thermal throttling.
- RustChunk implementation mapping: RustChunk Android performance settings.
- Evidence required before adoption: Device memory profiling and stable frame-time/GC tests.
- Completion state: COMPLETE — 517 logical lines through EOF; pinned source SHA and 19490 byte size match; reviewed, not modified; tests not executed.

### 2974. `launcher/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6018e8a6f0f3c600b781d3e1a95d9d1a84a70639`
- Size: 947 bytes
- Coverage: 1–49; complete EOF; 49 canonical logical lines, no gaps.
- Purpose: Declares launcher library module exports and public crate surface.
- Dependencies / cross-references: All launcher modules and Cargo crate graph.
- Implementation / logic: Defines module visibility and feature compilation boundaries.
- Findings: Thin desktop launcher crate root; does not define Android workspace or RustChunk client/core module split.
- RustChunk relevance: Useful only as module organization reference.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Keep Android launcher UI crate separate from game core/runtime crates and expose only stable interfaces.
- Follow-up checks: Check feature flags, target-specific modules, crate dependency cycles and public API boundaries.
- RustChunk implementation mapping: RustChunk Cargo workspace/module architecture.
- Evidence required before adoption: Successful ARM64 Android cross-build and module boundary tests.
- Completion state: COMPLETE — 49 logical lines through EOF; pinned source SHA and 947 byte size match; reviewed, not modified; tests not executed.

### 2975. `launcher/src/line_edit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d3acb1becb1a9072dfb5974cb8c700ca9cadbd6c`
- Size: 22624 bytes
- Coverage: 1–400, 401–707; complete EOF; 707 canonical logical lines, no gaps.
- Purpose: Implements custom desktop text editing with caret, selection, undo/redo, word movement, paste and viewport reveal.
- Dependencies / cross-references: GPUI keyboard/mouse events, text buffer, cursor/selection ranges and rendering.
- Implementation / logic: Maintains UTF-8-safe byte boundaries, editing history, selection replacement, word navigation, max length and scroll-to-caret behavior; includes extensive tests.
- Findings: Desktop keyboard/mouse interaction model; no touch IME, Android selection handles, accessibility or composing-region support.
- RustChunk relevance: Text-editing invariants may inform future custom widget, but native Android text fields are safer.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Prefer Android native text input/IME for server addresses, profile names and search; avoid reimplementing keyboard composition.
- Follow-up checks: Test RTL/Persian text, combining marks, emoji graphemes, IME composition, selection handles, paste, undo and accessibility if custom editor considered.
- RustChunk implementation mapping: RustChunk Android forms/search fields using native IME.
- Evidence required before adoption: Real Android keyboard tests for Persian/RTL and Unicode grapheme behavior.
- Completion state: COMPLETE — 707 logical lines through EOF; pinned source SHA and 22624 byte size match; reviewed, not modified; tests not executed.

### 2976. `launcher/src/loaders.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d4cd9cb620d4ecef33fee9b653f6aff4cbfa5a70`
- Size: 8930 bytes
- Coverage: 1–242; complete EOF; 242 canonical logical lines, no gaps.
- Purpose: Resolves and installs Fabric and Quilt loader profiles for Java Edition instances.
- Dependencies / cross-references: Fabric/Quilt metadata APIs, Mojang version JSON, Maven libraries and source download/checksum helpers.
- Implementation / logic: Parses loader kind, lists versions, chooses stable release, builds profile IDs and downloads profile/library metadata with SHA-1 verification.
- Findings: The module explicitly supports only Fabric and Quilt; parse rejects all other loaders. Forge 47.x is not supported by this loader registry.
- RustChunk relevance: Critical incompatibility for RustChunk's Forge requirement; only the general loader-profile pattern is reusable.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Implement Forge 47.4.5 installer and profile metadata, not a fallback to Fabric/Quilt.
- Follow-up checks: Test Forge installer profile, libraries, launch arguments, Java 17 and mod handshake on 1.20.1.
- RustChunk implementation mapping: RustChunk Forge loader manager and profile generation.
- Evidence required before adoption: Actual Forge 1.20.1 Forge installer and real modded client launch.
- Completion state: COMPLETE — 242 logical lines through EOF; pinned source SHA and 8930 byte size match; reviewed, not modified; tests not executed.

### 2977. `launcher/src/loaders/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `83ce986f48f7aa822945660a2be1df82599ecb10`
- Size: 5985 bytes
- Coverage: 1–161; complete EOF; 161 canonical logical lines, no gaps.
- Purpose: Tests Fabric/Quilt version listing, Maven paths and loader profile installation.
- Dependencies / cross-references: loaders module, mocked source downloads and live loader metadata endpoint.
- Implementation / logic: Checks stable version parsing, Maven coordinate paths, checksum-pinned libraries and an ignored live loader install.
- Findings: All test cases are Fabric/Quilt-only; no Forge installer or 1.20.1 Forge profile is tested. Tests were not executed.
- RustChunk relevance: Useful loader metadata/checksum test style, not proof of target loader compatibility.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Add dedicated Forge installer fixtures and profile validation rather than extending these assumptions blindly.
- Follow-up checks: Test Forge 47.x metadata, library checksum/size, installer cancellation, profile inheritance and Java 17.
- RustChunk implementation mapping: RustChunk Forge installer regression suite.
- Evidence required before adoption: Real Forge installer artifacts and actual Android launch.
- Completion state: COMPLETE — 161 logical lines through EOF; pinned source SHA and 5985 byte size match; reviewed, not modified; tests not executed.

### 2978. `launcher/src/mac_update.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6c591de010accae43c12d6abab55eb36bb333f31`
- Size: 6243 bytes
- Coverage: 1–168; complete EOF; 168 canonical logical lines, no gaps.
- Purpose: Safely replaces a running macOS application bundle and cleans retired versions after process use ends.
- Dependencies / cross-references: macOS app bundle, lsof/process checks, executable signature and filesystem staging.
- Implementation / logic: Checks code signature and active users, stages replacement, retains rollback and deletes only an unambiguous retired bundle.
- Findings: macOS-only update lifecycle and lsof semantics; irrelevant to Android PackageInstaller.
- RustChunk relevance: Only staged replacement/rollback principle is generic.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Use Android OS-managed signed APK update and app data migration; do not self-replace a running APK.
- Follow-up checks: Test PackageInstaller signature continuity, process lifecycle, rollback and interrupted update.
- RustChunk implementation mapping: RustChunk APK update manager boundary.
- Evidence required before adoption: Android 13 PackageInstaller update/rollback tests.
- Completion state: COMPLETE — 168 logical lines through EOF; pinned source SHA and 6243 byte size match; reviewed, not modified; tests not executed.

### 2979. `launcher/src/mac_update/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eaffa1e86b02414779c56d43c04a6f0109aef0a5`
- Size: 12252 bytes
- Coverage: 1–356; complete EOF; 356 canonical logical lines, no gaps.
- Purpose: Tests safe macOS app bundle replacement, active-user detection, symlink preservation and rollback.
- Dependencies / cross-references: mac_update module, fake lsof output, symlinked app bundle and code-signature fixtures.
- Implementation / logic: Covers ambiguous process queries, exact temporary naming, concurrent cleanup, failed signature and rollback behavior.
- Findings: Tests are macOS-specific and do not exercise Android package updates.
- RustChunk relevance: Rollback test strategy can inspire Android update tests; implementation cannot be reused.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Create Android-specific update tests around signed APK identity, interrupted install and profile-data preservation.
- Follow-up checks: Test signature mismatch, downgrade, concurrent launch, update cancellation and app data migration.
- RustChunk implementation mapping: RustChunk APK update tests.
- Evidence required before adoption: Real PackageInstaller flows on Android 13 and signed release artifacts.
- Completion state: COMPLETE — 356 logical lines through EOF; pinned source SHA and 12252 byte size match; reviewed, not modified; tests not executed.

### 2980. `launcher/src/main.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e1cd86b118ad458b59b35e636ab9fb0e64b3b246`
- Size: 218289 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4800, 4801–5200, 5201–5354; complete EOF; 5354 canonical logical lines, no gaps.
- Purpose: Main desktop GPUI application: state, release/update flow, account lifecycle, launch orchestration and all major launcher screens.
- Dependencies / cross-references: GPUI windowing/rendering, backend jobs, account/session, instances, Java launch, native mod build, telemetry, game tools, theme and platform services.
- Implementation / logic: Owns app state and event handling, startup release fetch, update checks, sign-in/refresh, profile selection, prepare/play flows, Java client launch, Home/settings/versions pages and app lifecycle.
- Findings: Very large desktop-specific presentation and orchestration module. It launches Horizon-native builds and Java instances but does not provide Forge 1.20.1 support; global UI state tightly couples many services.
- RustChunk relevance: Useful to map user workflows and identify subsystem boundaries; not suitable as RustChunk architecture source code.
- Decision: rewrite required; reference; platform-specific; version-specific
- 1.20.1 implications: Separate RustChunk Android UI, profile services, Java/Forge preparation, runtime process and game core; use this only as a feature inventory.
- Follow-up checks: Audit auth/session lifecycle, background cancellation, update trust, lifecycle restoration, memory use, error handling and Forge-specific launch args.
- RustChunk implementation mapping: RustChunk application architecture and launch orchestration decomposition.
- Evidence required before adoption: ARM64 Android build, Java 17, Forge 47.x end-to-end launch, account/session and Aternos tests.
- Completion state: COMPLETE — 5354 logical lines through EOF; pinned source SHA and 218289 byte size match; reviewed, not modified; tests not executed.

### 2981. `launcher/src/markdown.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c8e1acfe37005fe8a327c6edb71f3cb8c221fa29`
- Size: 9769 bytes
- Coverage: 1–249; complete EOF; 249 canonical logical lines, no gaps.
- Purpose: Renders release-note markdown into desktop GPUI elements with selected formatting and link behavior.
- Dependencies / cross-references: pulldown-cmark parser, GPUI, theme and URL links.
- Implementation / logic: Parses markdown events and maps headings/lists/emphasis/code/links into styled UI elements.
- Findings: Desktop-only renderer; link policy and unsupported Markdown behavior need security/accessibility review.
- RustChunk relevance: Markdown parsing idea reusable, GPUI renderer is not.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Use Android-native Markdown renderer or safe text subset; allowlist external link schemes and preserve attribution.
- Follow-up checks: Test malformed markdown, nested lists, unsafe URL schemes, very long documents, RTL and accessibility.
- RustChunk implementation mapping: RustChunk update notes/help renderer.
- Evidence required before adoption: Android UI screenshots, link-safety tests and Persian markdown rendering.
- Completion state: COMPLETE — 249 logical lines through EOF; pinned source SHA and 9769 byte size match; reviewed, not modified; tests not executed.

### 2982. `launcher/src/mcp.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d910ae5b92131c6d3fac8a0c59c295ae07da4732`
- Size: 18877 bytes
- Coverage: 1–354; complete EOF; 354 canonical logical lines, no gaps.
- Purpose: Hosts the desktop launcher MCP server and exposes session tools for account, game, launcher and instance control.
- Dependencies / cross-references: CLI tool generator, game_tools IPC, backend jobs, account sign-in/session handoff and JSON protocol.
- Implementation / logic: Advertises tool schemas, dispatches calls, handles long-running account/play actions and bridges to in-game tools.
- Findings: Agent automation surface is optional and security-sensitive; session handoff can involve secret/token files. Not required for basic launcher or Forge compatibility.
- RustChunk relevance: Do not add to baseline; permission/allowlist architecture is a future optional idea.
- Decision: unnecessary; reference; security-sensitive
- 1.20.1 implications: Keep RustChunk free of always-on agent server; if explicitly added, require opt-in, local-only binding, allowlisted tools and explicit destructive confirmations.
- Follow-up checks: Test local port binding, origin/authentication, malformed JSON, oversized messages, token/session disclosure, command permissions and process shutdown.
- RustChunk implementation mapping: No baseline mapping; optional local automation layer.
- Evidence required before adoption: Threat model, Android IPC security and adversarial RPC tests.
- Completion state: COMPLETE — 354 logical lines through EOF; pinned source SHA and 18877 byte size match; reviewed, not modified; tests not executed.

### 2983. `launcher/src/microsoft.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6335259d0c22011f6dbe248c75f7a30e7dca6ad5`
- Size: 2848 bytes
- Coverage: 1–81; complete EOF; 81 canonical logical lines, no gaps.
- Purpose: Renders Microsoft sign-in branding/button visuals.
- Dependencies / cross-references: GPUI, Microsoft logo SVG and launcher theme.
- Implementation / logic: Embeds vector logo and creates a styled sign-in button.
- Findings: Presentation only; contains no OAuth/device-code/token logic.
- RustChunk relevance: No direct compatibility value beyond branding asset licensing.
- Decision: unnecessary; reference; branding-sensitive
- 1.20.1 implications: Use official Microsoft auth flow only when needed; don't copy brand assets without compliance review.
- Follow-up checks: Review Microsoft branding rules and test OAuth flow in account subsystem, not this file.
- RustChunk implementation mapping: No RustChunk implementation mapping.
- Evidence required before adoption: Branding permission and actual Android authentication integration.
- Completion state: COMPLETE — 81 logical lines through EOF; pinned source SHA and 2848 byte size match; reviewed, not modified; tests not executed.

### 2984. `launcher/src/minecraft_versions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0d21f00ab1032d501a4bea962fa2cb71475d4e51`
- Size: 6764 bytes
- Coverage: 1–173; complete EOF; 173 canonical logical lines, no gaps.
- Purpose: Loads and caches the Mojang Minecraft version manifest and classifies release/snapshot/old versions.
- Dependencies / cross-references: Mojang version manifest, local cache and source fetch helpers.
- Implementation / logic: Parses release list, filters versions, supports latest aliases and serves cached/offline list.
- Findings: Version manifest is generic but current UI/tests assume later version timeline; cache freshness and signature/provenance matter.
- RustChunk relevance: Useful version metadata component for RustChunk.
- Decision: adaptable; reference
- 1.20.1 implications: Use official Mojang manifest to resolve exact 1.20.1 and required metadata; pin hashes and preserve offline cached view.
- Follow-up checks: Test 1.20.1 presence, stale cache, malformed manifest, latest aliases, network failure and manifest hash provenance.
- RustChunk implementation mapping: RustChunk Minecraft version selector and metadata cache.
- Evidence required before adoption: Official manifest fixture, checksum verification and offline profile setup.
- Completion state: COMPLETE — 173 logical lines through EOF; pinned source SHA and 6764 byte size match; reviewed, not modified; tests not executed.

### 2985. `launcher/src/modpacks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `08d524170eee97284953e4d058524bb64aa2c090`
- Size: 17888 bytes
- Coverage: 1–400, 401–480; complete EOF; 480 canonical logical lines, no gaps.
- Purpose: Imports Java modpacks and converts compatible parts into Horizon-native mod profiles/resources.
- Dependencies / cross-references: Modrinth .mrpack, CurseForge ZIP parsing, Horizon port pack tool, browser sources, instance manager and process execution.
- Implementation / logic: Loads pack manifest, counts/filters mods, downloads portable JARs, creates isolated profile, runs Horizon porting tool and manages pack lifecycle.
- Findings: Horizon porting tool converts supported mods to native Rust mods; CurseForge-only JARs remain unsupported. It is not a Forge JAR runtime and cannot run arbitrary Forge 1.20.1 mods.
- RustChunk relevance: Modpack manifest parsing/profile isolation ideas are useful, but porting pipeline is incompatible with user's Forge requirement.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Build actual Forge modpack import that preserves Java JARs and uses Forge installer/dependency resolution; do not port them into native Rust hooks.
- Follow-up checks: Test .mrpack/CurseForge/Prism imports, Forge 47.x mods, missing downloads, optional dependencies, checksums, profile isolation and launch.
- RustChunk implementation mapping: RustChunk Forge modpack importer and profile manager.
- Evidence required before adoption: Real 1.20.1 Forge packs, mod dependencies and actual launch on Android.
- Completion state: COMPLETE — 480 logical lines through EOF; pinned source SHA and 17885 byte size match; reviewed, not modified; tests not executed.

### 2986. `launcher/src/modpacks_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c1cc5091014a11648230201032bf44ac3e3117e7`
- Size: 20443 bytes
- Coverage: 1–400, 401–536; complete EOF; 536 canonical logical lines, no gaps.
- Purpose: Renders desktop Modpacks tab, import/port progress, report and profile play/remove controls.
- Dependencies / cross-references: GPUI, modpacks service, Horizon port report, browser and async job state.
- Implementation / logic: Displays imported pack coverage and port statuses, runs background import/port, opens isolated pack profile and handles remove.
- Findings: Desktop UI and Horizon-native port report; does not establish Forge loader support.
- RustChunk relevance: Only progress/report presentation is reusable.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Design Android modpack import with explicit compatibility report showing supported/unsupported Forge JARs and no false claim of native conversion.
- Follow-up checks: Test progress cancellation, lifecycle restoration, partial failure, large pack memory and Forge-specific report accuracy.
- RustChunk implementation mapping: RustChunk modpack import UI and progress model.
- Evidence required before adoption: Android SAF import, Forge 1.20.1 modpack launch and truthful compatibility reporting.
- Completion state: COMPLETE — 536 logical lines through EOF; pinned source SHA and 20435 byte size match; reviewed, not modified; tests not executed.

### 2987. `launcher/src/mods_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c3a595775ab16ead83501a22d2826c3cee28796b`
- Size: 74109 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1809; complete EOF; 1809 canonical logical lines, no gaps.
- Purpose: Desktop Mods page for registry search, install/update/remove, native-mod builds and pack management.
- Dependencies / cross-references: Browser sources, modpacks, native_mods, GPUI state, loader target, project details and background jobs.
- Implementation / logic: Maintains search filters, project support checks, install/update queues, installed rows, native build actions and source selection.
- Findings: Eight TODO markers remain; current support/port model is Horizon native, and loader target is Fabric/Quilt. Not a Forge 1.20.1 mod manager.
- RustChunk relevance: Feature inventory and async state patterns useful, but implementation and compatibility logic require replacement.
- Decision: rewrite required; reference; version-specific; platform-specific
- 1.20.1 implications: Create a Forge-aware Android mod manager that manages original JARs, loader metadata, dependencies and per-profile toggles; keep native Rust extensions separate.
- Follow-up checks: Resolve all TODOs; test stale search responses, async cancellation, dependency conflicts, malformed mods, checksum, loader compatibility and concurrent profile changes.
- RustChunk implementation mapping: RustChunk Forge mod catalog and installed-mod management.
- Evidence required before adoption: Forge 47.x corpus, Android UI, dependency resolution and actual game launch.
- Completion state: COMPLETE — 1809 logical lines through EOF; pinned source SHA and 74089 byte size match; reviewed, not modified; tests not executed.

### 2988. `launcher/src/mods_project.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f8d3e37927eeea0d1fafe462a4a9010c407eff68`
- Size: 32300 bytes
- Coverage: 1–400, 401–800, 801–832; complete EOF; 832 canonical logical lines, no gaps.
- Purpose: Renders project detail page, icons, categories, filters, sorting and project checks on desktop Mods page.
- Dependencies / cross-references: Browser project metadata, GPUI images, sort/filter models and mod support checks.
- Implementation / logic: Builds project page and filtering controls, loads project icons, categories and compatible version information.
- Findings: UI depends on current Horizon browser support classification; does not add Forge compatibility.
- RustChunk relevance: Project detail UX can be adapted, not source code.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Build Android project detail page with explicit Minecraft 1.20.1 Forge support state from RustChunk's resolver.
- Follow-up checks: Test stale project fetches, malformed metadata, icon caching, sorting/filter interactions and accessibility.
- RustChunk implementation mapping: RustChunk mod project detail screen.
- Evidence required before adoption: Real Modrinth/CurseForge 1.20.1 Forge metadata and device UI tests.
- Completion state: COMPLETE — 832 logical lines through EOF; pinned source SHA and 32290 byte size match; reviewed, not modified; tests not executed.

### 2989. `launcher/src/mojang.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7185596dd189b2ee482399bf55a05cae1a1fd3ca`
- Size: 17112 bytes
- Coverage: 1–400, 401–441; complete EOF; 441 canonical logical lines, no gaps.
- Purpose: Downloads Mojang version files, libraries, assets and Java runtimes with manifest validation and cancellation.
- Dependencies / cross-references: Mojang manifests, source pin/checksum helpers, Java runtime layouts, OS/architecture rules and file system.
- Implementation / logic: Resolves manifest URLs, validates paths remain inside runtime directories, checks SHA-1/size, downloads concurrently and supports cancellation/resume.
- Findings: Desktop OS/native classifiers; Android ARM64 native classifiers and scoped storage differ. Mojang checksum semantics should be retained but upgraded where available.
- RustChunk relevance: Core resource download and verification design is valuable for RustChunk.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Use for official Minecraft 1.20.1 libraries/assets after implementing Android ARM64 natives and a Java 17 runtime.
- Follow-up checks: Test path traversal, manifest mismatch, wrong size/hash, cancellation/resume, concurrent writes, missing assets and Android native ABI selection.
- RustChunk implementation mapping: RustChunk Mojang artifact downloader and asset preparation.
- Evidence required before adoption: Pinned 1.20.1 manifest, verified libraries/assets, ARM64 Java 17 runtime and real launch.
- Completion state: COMPLETE — 441 logical lines through EOF; pinned source SHA and 17112 byte size match; reviewed, not modified; tests not executed.

### 2990. `launcher/src/mojang_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c46ac94c106b1e4684c9b98cda64581dc828a4e2`
- Size: 20223 bytes
- Coverage: 1–400, 401–604; complete EOF; 604 canonical logical lines, no gaps.
- Purpose: Tests Mojang downloads, runtime layouts, platform rules, path containment, checksum/size validation and cancellation/resume.
- Dependencies / cross-references: mojang module, fake HTTP sources, synthetic runtime manifests and temporary filesystem.
- Implementation / logic: Covers platform-specific native libraries, path containment, hash/size mismatch refusal, missing sounds, resumable downloads and asset failures.
- Findings: Fixtures target desktop platforms and do not prove Android ARM64 runtime/native handling; tests were not executed.
- RustChunk relevance: Strong security/download test pattern, but add Android target fixtures.
- Decision: adaptable; reference; security-sensitive
- 1.20.1 implications: Add ARM64 Android runtime/native fixtures and Forge library handling; preserve exact size/hash and path containment guarantees.
- Follow-up checks: Test archive extraction, symlink path escapes, concurrent downloads, cancel/resume corruption, malicious manifest URLs and Android storage.
- RustChunk implementation mapping: RustChunk download verification suite.
- Evidence required before adoption: Android ABI fixtures, fuzzed manifest and real Forge 1.20.1 preparation.
- Completion state: COMPLETE — 604 logical lines through EOF; pinned source SHA and 20223 byte size match; reviewed, not modified; tests not executed.

### 2991. `launcher/src/multiplayer_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `98813b1cf5353d5f3b015dc860771e0fda35f8bb`
- Size: 9758 bytes
- Coverage: 1–261; complete EOF; 261 canonical logical lines, no gaps.
- Purpose: Renders desktop saved multiplayer server list, icons, add/edit and count.
- Dependencies / cross-references: GPUI, server_list model, server image cache and text field.
- Implementation / logic: Loads saved servers, edits/adds server addresses and renders server rows.
- Findings: Desktop UI only; no Minecraft network protocol or connection implementation.
- RustChunk relevance: UX pattern for saved servers only.
- Decision: adaptable; reference; platform-specific
- 1.20.1 implications: Build Android server list with validation and a direct 1.20.1 Forge connect action; keep actual protocol in client networking layer.
- Follow-up checks: Test host/port parsing, IPv6, duplicate entries, invalid addresses, offline server list and Aternos connection.
- RustChunk implementation mapping: RustChunk multiplayer server list screen.
- Evidence required before adoption: Actual Aternos server ping/login/Forge handshake from Android.
- Completion state: COMPLETE — 261 logical lines through EOF; pinned source SHA and 9757 byte size match; reviewed, not modified; tests not executed.

### 2992. `launcher/src/native_log_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `81a86ac304d8e43c4e09c5c8df42301bb3aff963`
- Size: 14745 bytes
- Coverage: 1–390; complete EOF; 390 canonical logical lines, no gaps.
- Purpose: Displays logs from Horizon native mod builds and refreshes diagnostic snapshots.
- Dependencies / cross-references: native_mods build log files, GPUI clipboard/scroll and build state.
- Implementation / logic: Reads complete diagnostic log but renders tail, discards stale async reads after refresh and exposes copy/scroll UI.
- Findings: Specific to Rust native-mod build pipeline; not a Forge/Minecraft game log viewer.
- RustChunk relevance: General async log snapshot pattern can be reused.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Keep Forge game logs separate from native build logs; Android diagnostics should be local and bounded.
- Follow-up checks: Test large logs, concurrent refresh, file deletion, stale async completion and sensitive-data redaction.
- RustChunk implementation mapping: RustChunk local diagnostics viewer.
- Evidence required before adoption: Real Forge crash logs, Android storage and privacy tests.
- Completion state: COMPLETE — 390 logical lines through EOF; pinned source SHA and 14745 byte size match; reviewed, not modified; tests not executed.

### 2993. `launcher/src/native_mods.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b497bc6e3e330bc3b19df8dee73edba719334aaf`
- Size: 151404 bytes
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–3857; complete EOF; 3857 canonical logical lines, no gaps.
- Purpose: Builds and installs Horizon-native Rust mixin mods by compiling mod crates and injecting generated hooks into Horizon game binaries.
- Dependencies / cross-references: Cargo/Rust toolchain, Horizon lockfile, mixin manifests, syn/quote injection, native mod registry, game revision pins, per-profile build cache and launcher runtime.
- Implementation / logic: Resolves mod crates/dependencies, validates hook points and game revisions, generates Rust trait dispatch, compiles client/server builds, stages binaries, records hashes/build metadata and installs per profile.
- Findings: This is a proprietary Rust-native extension system requiring matching Horizon source/revision and Rust toolchain; it does not load Java Forge JARs or implement Forge ModLauncher/FML. It is not a substitute for Forge.
- RustChunk relevance: Useful only as an example of a separately scoped native extension pipeline and deterministic build cache; incompatible with target mod ecosystem.
- Decision: unnecessary; reference; version-specific; security-sensitive
- 1.20.1 implications: Do not port this to satisfy Forge support. RustChunk must launch original Java Forge 1.20.1 and load mods through Forge itself.
- Follow-up checks: Review compiler execution permissions, build-script sandboxing, dependency pinning, injected hook ABI, cache invalidation and untrusted mod source risk if a separate native plugin feature is ever scoped.
- RustChunk implementation mapping: No Forge mapping; possible independent RustChunk-native extension layer only by explicit product decision.
- Evidence required before adoption: Forge 47.x actual client, ModLauncher/FML handshake and mod JAR loading; separate native extension threat model.
- Completion state: COMPLETE — 3857 logical lines through EOF; pinned source SHA and 151404 byte size match; reviewed, not modified; tests not executed.

### 2994. `launcher/src/native_mods/api.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b890b50d928c32855929545c77e234a902d3464a`
- Size: 17629 bytes
- Coverage: 1–400, 401–447; complete EOF; 447 canonical logical lines, no gaps.
- Purpose: Generates and stages the Rust trait-mod API crate into a native mod build.
- Dependencies / cross-references: Horizon game crate, Cargo dependency graph, manifest and generated Rust entry point.
- Implementation / logic: Validates API folder, generates main.rs and dependency declarations, stages trait dispatch and tests dependency-cycle avoidance.
- Findings: API targets Horizon-native Rust game hooks and specific game crate; unrelated to Forge Java mod API.
- RustChunk relevance: No direct compatibility value for Forge.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Keep RustChunk native core separate from Java Forge mod API; do not pretend this is FML/ModLauncher integration.
- Follow-up checks: If a separate Rust plugin API is designed, specify stable ABI, permissions, versioning and sandbox boundary.
- RustChunk implementation mapping: No baseline Forge mapping.
- Evidence required before adoption: Independent plugin architecture and threat model; Forge runtime integration tests.
- Completion state: COMPLETE — 447 logical lines through EOF; pinned source SHA and 17629 byte size match; reviewed, not modified; tests not executed.

### 2995. `launcher/src/native_mods/crates.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b1995f7731fc58d5ed4d5e9ca3dc31120ea50800`
- Size: 32273 bytes
- Coverage: 1–400, 401–800, 801–804; complete EOF; 804 canonical logical lines, no gaps.
- Purpose: Resolves and validates native mod crate dependencies and post-build commands against Horizon's pinned Cargo lock.
- Dependencies / cross-references: Cargo manifests/lock, crate workspace, filesystem paths, symlink/junction checks and post-build subprocesses.
- Implementation / logic: Scans dependency tables, restricts sibling path dependencies to approved mod folders, rejects conflicting path sources and executes declared post-build steps.
- Findings: Arbitrary Cargo build scripts/post-build commands execute native code; security depends on trusted mod source and path/symlink checks. Horizon lock and APIs are version-specific.
- RustChunk relevance: Security validation patterns for any future code plugin pipeline, not Forge mod loading.
- Decision: unnecessary; reference; security-sensitive
- 1.20.1 implications: Do not compile user-provided native Rust mods in baseline Android launcher; if ever allowed, require explicit trust and sandboxing.
- Follow-up checks: Fuzz path dependencies, symlink/junction escape, malicious build.rs, post-build command injection, lockfile drift and build environment variables.
- RustChunk implementation mapping: No baseline mapping; optional separate Rust extension build sandbox.
- Evidence required before adoption: Android NDK build isolation, explicit user consent, signed source and adversarial build-script tests.
- Completion state: COMPLETE — 804 logical lines through EOF; pinned source SHA and 32273 byte size match; reviewed, not modified; tests not executed.

### 2996. `launcher/src/native_mods/dependencies.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `47e1fced8462ce6b101c3f93e0460231d2951b95`
- Size: 15478 bytes
- Coverage: 1–400, 401–447; complete EOF; 447 canonical logical lines, no gaps.
- Purpose: Parses and resolves semantic-version dependencies for Horizon native mods and rewrites their manifests atomically.
- Dependencies / cross-references: semver, native mod IDs, Cargo features and optional dependency metadata.
- Implementation / logic: Resolves dependency graph/order, checks version requirements and conflicting feature/name declarations, then rewrites dependency records.
- Findings: Resolver is for native Rust mods, not Forge mod dependency metadata; its version constraints and lifecycle differ.
- RustChunk relevance: Generic graph resolution can inform Forge dependency diagnostics, not direct reuse.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Implement Forge mod dependency metadata resolver using Forge's actual mandatory/optional dependency and version-range semantics.
- Follow-up checks: Test cycles, optional deps, incompatible versions, duplicate mod IDs, side restrictions and deterministic ordering.
- RustChunk implementation mapping: RustChunk Forge mod dependency checker, newly implemented against Forge semantics.
- Evidence required before adoption: Forge 47.x mod metadata fixtures and real modpack dependency graph.
- Completion state: COMPLETE — 447 logical lines through EOF; pinned source SHA and 15478 byte size match; reviewed, not modified; tests not executed.

### 2997. `launcher/src/native_mods/inject.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `698032dfb55afa40fac439734a1f92626f7b42f1`
- Size: 30800 bytes
- Coverage: 1–400, 401–758; complete EOF; 758 canonical logical lines, no gaps.
- Purpose: Generates Rust code for trait-backed mixin injections into Horizon game source/build.
- Dependencies / cross-references: syn/quote AST rewriting, Horizon hook-point manifests, generated API crate and native build pipeline.
- Implementation / logic: Transforms Rust types/lifetimes, creates dispatch wrappers and event/trait hooks, stages generated source and tests injection order/cancellation behavior.
- Findings: This is Rust AST injection into Horizon code, not Java bytecode transformation or Forge Mixin/ModLauncher. No reuse for Java Forge mods.
- RustChunk relevance: Potentially instructive about deterministic injection and hook validation only; technology is incompatible.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Use Forge's own Java transformation/loading pipeline; do not transplant Rust `syn` hooks into Minecraft 1.20.1.
- Follow-up checks: For any separate native extension, test ABI, generated code determinism, hook cancellation and target-version mismatch.
- RustChunk implementation mapping: No baseline Forge mapping.
- Evidence required before adoption: Actual Forge ModLauncher transformation and FML lifecycle tests.
- Completion state: COMPLETE — 758 logical lines through EOF; pinned source SHA and 30800 byte size match; reviewed, not modified; tests not executed.

### 2998. `launcher/src/native_mods/inject_cache.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `195192cd9e6b0d395f947c87bd0047b7724aec6a`
- Size: 3826 bytes
- Coverage: 1–108; complete EOF; 108 canonical logical lines, no gaps.
- Purpose: Builds cache keys and prunes generated mixin/API artifacts for native mod builds.
- Dependencies / cross-references: Native mod bundles, profile, manifest, generated API files and SHA-256.
- Implementation / logic: Hashes inputs excluding generated API overlays, snapshots build cache and removes stale entries.
- Findings: Cache semantics are for Rust mixin build pipeline, not Minecraft chunk/render cache or Forge mod loading.
- RustChunk relevance: No direct compatibility value.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Keep RustChunk runtime/asset cache separate; if adding build cache, use content-addressed inputs and explicit invalidation.
- Follow-up checks: Test cache key changes on every relevant input, stale cache removal, concurrent builds and corruption.
- RustChunk implementation mapping: No baseline mapping; possible Rust build cache only if native extensions are separately approved.
- Evidence required before adoption: Deterministic cache invalidation tests and concurrent build safety.
- Completion state: COMPLETE — 108 logical lines through EOF; pinned source SHA and 3826 byte size match; reviewed, not modified; tests not executed.

### 2999. `launcher/src/native_status.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e56433ab01d4535576c5fdb4bcf3ec93a0f8a098`
- Size: 16229 bytes
- Coverage: 1–400, 401–426; complete EOF; 426 canonical logical lines, no gaps.
- Purpose: Determines whether Horizon native mods are installed, enabled and built for the current game revision.
- Dependencies / cross-references: Native mod manifests, profile settings, installed build metadata and game version.
- Implementation / logic: Parses enabled native mods, computes build fingerprints and reports whether binary contains matching mod set/revision.
- Findings: Manifest examples hardcode Horizon game revision 26.3; this state model is not Forge mod loading or JAR status.
- RustChunk relevance: Build-state fingerprint idea is reusable for RustChunk profiles, but current semantics are proprietary.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Track Forge profile mod JAR hashes and loader/game version separately; do not reuse Horizon native mod state flags.
- Follow-up checks: Test changed JAR/config, disabled mods, profile switching, stale builds and Forge update migration.
- RustChunk implementation mapping: RustChunk profile compatibility/status summary.
- Evidence required before adoption: Actual Forge 1.20.1 mod list, hash/fingerprint consistency and launch readiness tests.
- Completion state: COMPLETE — 426 logical lines through EOF; pinned source SHA and 16229 byte size match; reviewed, not modified; tests not executed.

### 3000. `launcher/src/new_instance.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `59229fc2258fa32316bf2479e83458d27d6b687c`
- Size: 13933 bytes
- Coverage: 1–361; complete EOF; 361 canonical logical lines, no gaps.
- Purpose: Renders desktop new-instance wizard for Horizon and vanilla Java Edition profiles.
- Dependencies / cross-references: GPUI shell, instances service, Minecraft version list and Fabric/Quilt loader picker.
- Implementation / logic: Creates Java instances and selects vanilla/Fabric/Quilt; defaults and labels reflect current game version and loader list.
- Findings: The source explicitly exposes only Fabric and Quilt choices; no Forge 47.x option or installer workflow.
- RustChunk relevance: Profile wizard UX can be adapted; loader choices need replacement.
- Decision: rewrite required; reference; version-specific; platform-specific
- 1.20.1 implications: Implement Android new-profile wizard with Minecraft 1.20.1, Forge 47.4.5 and Java 17 as explicit choices.
- Follow-up checks: Test Forge installer success/failure, Java 17 selection, separate profile directories, cancel and resume.
- RustChunk implementation mapping: RustChunk profile creation screen and Forge setup state machine.
- Evidence required before adoption: Real Forge 1.20.1 profile creation and launch on ARM64 Android.
- Completion state: COMPLETE — 361 logical lines through EOF; pinned source SHA and 13933 byte size match; reviewed, not modified; tests not executed.

