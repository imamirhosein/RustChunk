# RustChunk Starling Audit Ledger — Entries 2801–3000

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active 200-file stage: **2801–3000**; completed **75/200 (2801–2875)**.
- One-time exceptional authorization covers exactly **2801–2900**. Checkpoints **2801–2825**, **2826–2850** and **2851–2875** are complete; section **2801–2850** is closed and section **2851–2900** is 25/50 complete. Continue **2851–2875** and **2876–2900** under the same explicit 100-file grant without another permission request. **STOP at 2900**.
- Source selection: first 50 eligible source/documentation blobs in strict raw lexical order from the complete pinned recursive tree after excluding all 2,800 paths in the Index. The tree is untruncated; all 25 selected paths, SHAs and byte sizes match the pin.
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

