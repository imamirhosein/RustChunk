# RustChunk Starling Audit Ledger — Entries 2801–3000

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active 200-file stage: **2801–3000**; completed **25/200 (2801–2825)**.
- One-time exceptional authorization covers exactly **2801–2900**. The first checkpoint **2801–2825** is registered; continue to **2826–2850** under the same grant, then stop at the 50-file boundary **2850** and report. Do not inspect 2851+ until the user authorizes the next 50-file section.
- Source selection: first 25 eligible source/documentation blobs in strict raw lexical order from the complete pinned recursive tree after excluding all 2,800 paths in the Index. The tree is untruncated; all 25 selected paths, SHAs and byte sizes match the pin.
- Starling/Horizon documentation and implementation target Minecraft Java 26.3; this audit is compatibility research, not evidence of Minecraft 1.20.1 / Forge 47.x support.
- No application source was modified. Documentation/tests were inspected only; no tests, builds, benchmarks, APK, device, Forge, Minecraft or server validation was run.

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

