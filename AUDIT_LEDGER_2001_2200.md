# RustChunk Starling Audit Ledger — Entries 2001–2200

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 2001–2200 (200 unique source files; one shared Ledger for this stage).
- Current progress: **25/200**. Checkpoint **2001–2025 complete and closed**; checkpoint **2026–2050 authorized**.
- One-time expanded authorization: entries **2001–2100 only**, comprising sections 2001–2050 and 2051–2100. The second 50-file section is pre-authorized; no mid-section permission request is needed at 2050.
- Cadence unchanged: exact checkpoints of 25 files, boundary reports at every 50 files, and a stop at **2100**. Entries 2101–2200 are not authorized.
- Each entry contains exactly the 15 required fields from AUDIT_PROTOCOL.md. Files over 400 lines were inspected through EOF in consecutive, gap-free ranges no larger than 400 lines.
- Embedded tests/examples are inspected only; they are not executed.


### 2001. `crates/horizon-server/src/signals.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `11222257c8d8f5f2a232a8016c48cf6e67600096`
- Size: 1,360 bytes; 41 logical lines
- Coverage: 1–41 (EOF)
- Purpose: Collects Ctrl-C, SIGTERM, SIGHUP and Windows console shutdown requests for the dedicated-server supervisor.
- Dependencies / cross-references: AtomicU32; libc::signal; windows_sys::SetConsoleCtrlHandler; supervisor.rs.
- Implementation / logic: Each OS callback increments a process-global atomic counter; requests() returns its value.
- Findings: Registration failures are ignored. A signal during run() setup but before supervise() snapshots the counter can be mistaken as already handled; the u32 counter can theoretically wrap.
- RustChunk relevance: Host-side server lifecycle only, not Android NativeActivity or client gameplay.
- Decision: unnecessary; OS stop-signal handler; dedicated-server-only
- 1.20.1 implications: No 1.20.1 gameplay, networking, renderer or Forge functionality.
- Follow-up checks: Test all platform signals and registration failures; preserve requests arriving before supervise starts; consider wrap-safe sequencing.
- RustChunk implementation mapping: No direct Android client mapping; only a separate desktop/server companion might need host signal handling.
- Evidence required before adoption: Cross-platform process tests, setup-time signal race tests and registration-error coverage.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2002. `crates/horizon-server/src/supervisor.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9367778a505f27c4d0ec9a8e8db5049b618da5c5`
- Size: 34,905 bytes; 899 logical lines
- Coverage: 1–200; 201–400; 401–600; 601–800; 801–899 (EOF)
- Purpose: Runs and supervises the dedicated server, manages console/control services, crash restarts, automatic backups and failed-update rollback.
- Dependencies / cross-references: API Shared/Phase; config; engine; backup; signals; update; management, RCON, Query, panel, events, Discord and horizon-bench.
- Implementation / logic: Loads server config/resources, locks the server folder, starts host endpoints and samplers, launches the game, pumps logs, handles stop/restart, syncs config and limits crash restarts to five per ten minutes.
- Findings: Early stop-signal race; failed program/resource resolution inside restart loop is silently ignored and stale paths reused; unbounded console mpsc channel; log pump stops on read/UTF-8 error and may stop draining child pipes; thread spawn failures are often ignored.
- RustChunk relevance: Server process manager, not native Minecraft client loop, Android lifecycle or chunk pipeline.
- Decision: unnecessary; dedicated-server process supervisor; host-only
- 1.20.1 implications: No 1.20.1 client or Forge compatibility evidence; targets newer Horizon server engine.
- Follow-up checks: Preserve early stop requests; log reload failures; bound/backpressure console input; keep draining child pipes after decode errors; surface thread-spawn errors; test crash/stop/update races.
- RustChunk implementation mapping: Do not include in Android client. Crash-backoff and rollback ideas only apply to a distinct server companion.
- Evidence required before adoption: Cross-platform process lifecycle/race tests, large console/log tests, update failure injection and child cleanup.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2003. `crates/horizon-server/src/update.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `191cc39c1faafc27073810d87c61a26a6dfb8ea6`
- Size: 49,142 bytes; 1,200 logical lines
- Coverage: 1–200; 201–400; 401–600; 601–800; 801–1000; 1001–1200 (EOF)
- Purpose: Finds, downloads, verifies, unpacks, installs, switches, rolls back and prunes versioned server releases.
- Dependencies / cross-references: GitHub release API; horizon-release-sign keys/legacy hashes; SHA-256; ZIP/tar; Install symlink/junction layout; backup; Config; engine; supervisor API.
- Implementation / logic: Selects platform assets, validates signed checksum or legacy digest, downloads and hashes archives, unpacks staging release, switches current/previous and asks supervisor to restart or roll back.
- Findings: Download and expanded archive sizes/counts are unbounded; Windows switch removes old junction before creating new (not atomic); replacing a non-current target removes it before staged rename; release tags permit characters unsafe on Windows; previous marker is plain text without strict tag validation.
- RustChunk relevance: Signature/checksum and staged rollback are useful only for a separate RustChunk updater; this is server updater and does not load Forge mods.
- Decision: adaptable; signed release/update workflow; server updater
- 1.20.1 implications: Asset set is Linux/macOS/Windows, not Android ARM64 RustChunk; no 1.20.1/Forge update support.
- Follow-up checks: Validate portable tag syntax and previous marker; cap download and expanded archive bytes/files; safe extraction; make Windows switch recoverable; preserve existing target until new release is committed; test interruptions.
- RustChunk implementation mapping: Adapt signature validation/staged updates only in a separately designed Android updater with package/ABI/signing rules.
- Evidence required before adoption: Malicious tag/path, disk exhaustion, archive-bomb, interrupted switch/rollback, signature replay and cross-platform update tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2004. `crates/horizon-server/src/websocket.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9567a6bcebae23184ed6d076e239d0cdfa7d61a9`
- Size: 12,710 bytes; 352 logical lines
- Coverage: 1–352 (EOF)
- Purpose: Implements server-side RFC6455 WebSocket text messages over TCP or rustls TLS for Horizon management RPC.
- Dependencies / cross-references: TcpStream; rustls ServerConnection; base64/SHA-1 handshake key; horizon-server management RPC.
- Implementation / logic: Provides shared plain/TLS Link, 64KiB text message aggregation, client-mask decode, ping/pong, close echo and unmasked server frames.
- Findings: Does not validate RSV bits, canonical extended lengths, 64-bit high bit, control-frame fragmentation/size rules or close payload. Socket::new removes read timeout, allowing idle connection/handshake indefinitely absent higher-layer limits. Close does not enforce a terminal socket state. Tests do not cover full RFC conformance.
- RustChunk relevance: Potential management transport only; not Minecraft TCP packet framing or needed for ordinary public servers.
- Decision: adaptable; WebSocket framing and TLS I/O reference; management-only
- 1.20.1 implications: No Forge 1.20.1 packet framing/mod negotiation; protocol is Horizon server management.
- Follow-up checks: Add RFC6455 conformance/fuzz tests; enforce handshake/read/idle lifetime; prohibit frames after close.
- RustChunk implementation mapping: Keep separate from horizon-protocol and gameplay networking.
- Evidence required before adoption: RFC6455/TLS/timeout, malformed-frame, fragmentation, slow-client and resource-exhaustion tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2005. `crates/horizon-shader-api/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `588808a6bd9951b6a5af57cd9ef4535dec136024`
- Size: 733 bytes; 21 logical lines
- Coverage: 1–21 (EOF)
- Purpose: Declares horizon-shader-api dependencies for WGSL shader-pack parsing, validation, translation and packaging.
- Dependencies / cross-references: Workspace Rust settings; bytemuck; PNG; Naga 30.0.1; serde; JSON; SHA-256; TOML; ZIP.
- Implementation / logic: Requires Rust 1.85 and enables Naga WGSL input plus SPIR-V, MSL and HLSL outputs.
- Findings: Naga 30.0.1 and features are tied to current Horizon/wgpu dependency graph; should not be copied blindly.
- RustChunk relevance: Potential future user-shader feature; manifest itself does not integrate Android or 1.20.1 rendering.
- Decision: adaptable; shader tooling dependency manifest; version-specific
- 1.20.1 implications: No 1.20.1 game logic or shader inputs.
- Follow-up checks: Compare required Naga and wgpu versions; review dependency size, Android support and licences.
- RustChunk implementation mapping: Candidate for separate shader SDK with dependencies aligned to RustChunk renderer.
- Evidence required before adoption: ARM64 dependency build and shader backend parity tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2006. `crates/horizon-shader-api/packs/golden_hour/README.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `943bce85a7f66884a116f066f718a328237484bf`
- Size: 198 bytes; 4 logical lines
- Coverage: 1–4 (EOF)
- Purpose: Documents Golden Hour example shader pack and validation command.
- Dependencies / cross-references: Neighbouring manifest, WGSL stages and horizon-shader CLI.
- Implementation / logic: Describes one sky stage, two post passes and five options; points to horizon-shader check.
- Findings: Concise example docs, but depend on Horizon shader API/CLI.
- RustChunk relevance: Documentation pattern only; no gameplay/render implementation.
- Decision: adaptable; example shader-pack documentation; sample-only
- 1.20.1 implications: API 0.1 is Horizon-specific, not verified Minecraft 1.20.1 shader interface.
- Follow-up checks: Keep docs synchronized with actual manifest and generated API reference.
- RustChunk implementation mapping: Potential README for a future RustChunk shader SDK.
- Evidence required before adoption: Real pack validation and generated-doc consistency test.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2007. `crates/horizon-shader-api/packs/golden_hour/common.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `687e76f4b7215245548cb9c6dc2fb65099af94cc`
- Size: 446 bytes; 13 logical lines
- Coverage: 1–13 (EOF)
- Purpose: Provides WGSL helpers for luminance, Overworld daylight and low-sun weighting.
- Dependencies / cross-references: Frame sun_direction/dimension; manifest shared files.
- Implementation / logic: Computes Rec.709 luminance, daylight only when dimension == 0 and a smooth low-sun factor.
- Findings: Generic math is portable; dimension ID and frame semantics are API-specific.
- RustChunk relevance: Math sample only; game dimensions/time must come from target engine.
- Decision: adaptable; reusable WGSL helper examples; sample-only
- 1.20.1 implications: 26.3 dimension/time semantics cannot be assumed for 1.20.1.
- Follow-up checks: Verify dimension IDs, sun vector and rain/daylight semantics; compare against vanilla oracle.
- RustChunk implementation mapping: Reuse only generic WGSL math after adapting input contract.
- Evidence required before adoption: Shader compile and image comparisons day/night, Nether/End, rain and horizon angles.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2008. `crates/horizon-shader-api/packs/golden_hour/horizon-shaders.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b67cff3b1a4b4c4ebf28056002841e98098e207`
- Size: 1,127 bytes; 69 logical lines
- Coverage: 1–69 (EOF)
- Purpose: Declares Golden Hour pack identity, shared WGSL, sky/post stages, five options and GPU budget.
- Dependencies / cross-references: Manifest schema 1/API 0.1; sky/common/post WGSL files.
- Implementation / logic: Defines a sky stage, two post passes and bounded float/bool options.
- Findings: Readable declarative composition; relies on Horizon stage/resource rules.
- RustChunk relevance: Can inform future shader pack layout/settings only.
- Decision: adaptable; shader pack manifest example; API-specific
- 1.20.1 implications: Not a vanilla 1.20.1 resource pack or OptiFine/Iris format.
- Follow-up checks: Design RustChunk shader API separately; test allowed keys, paths and option bounds.
- RustChunk implementation mapping: Reference only after RustChunk stage contract exists.
- Evidence required before adoption: Manifest schema and renderer image tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2009. `crates/horizon-shader-api/packs/golden_hour/post/glow.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e27b98244da44f9a78a23567361d4d6550b4021a`
- Size: 1,016 bytes; 25 logical lines
- Coverage: 1–25 (EOF)
- Purpose: Implements soft brightness halo with twelve screen-space texture samples.
- Dependencies / cross-references: PostInput; previous; linear_sampler; depth_at; luminance; frame.resolution.
- Implementation / logic: Thresholds bright pixels, reduces sky glow, samples two rings of offsets and adds halo.
- Findings: Bounded tap count; reversed-depth and sampler assumptions are renderer-specific.
- RustChunk relevance: Useful low-cost sample if user WGSL passes are supported later.
- Decision: adaptable; post-process glow sample; renderer-specific
- 1.20.1 implications: No Java shader-pack or gameplay feature.
- Follow-up checks: Validate depth detection, tap count, resolution scaling and Mali-G52 GPU cost.
- RustChunk implementation mapping: Optional effect only after compatible RustChunk post inputs exist.
- Evidence required before adoption: Shader compiler, multi-resolution image and ARM64 GPU timing tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2010. `crates/horizon-shader-api/packs/golden_hour/post/grade.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `809b0877ea1dc7c49db19da94126e24decd6e7b1`
- Size: 871 bytes; 19 logical lines
- Coverage: 1–19 (EOF)
- Purpose: Applies sunrise/sunset warmth, saturation adjustment, highlight shoulder and optional vignette.
- Dependencies / cross-references: PostInput; frame sun/rain/resolution; daylight, sun_low and luminance; overrides.
- Implementation / logic: Modulates RGB warmth by sun/day/rain, applies rational highlight curve, desaturates and darkens edges.
- Findings: Small composable effect; Frame semantics/thresholds are Horizon-specific.
- RustChunk relevance: Color grading example, not required for core gameplay.
- Decision: adaptable; color-grading sample; renderer-specific
- 1.20.1 implications: Does not reproduce exact 1.20.1 vanilla post-processing or Java shader APIs.
- Follow-up checks: Compare tone mapping/day/rain and HDR clipping against vanilla 1.20.1.
- RustChunk implementation mapping: Optional example after RustChunk shader frame interface is defined.
- Evidence required before adoption: Screenshot parity day/sunset/rain/night vision/brightness.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2011. `crates/horizon-shader-api/packs/golden_hour/sky.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dbf1e0ab56f955b5e1fd93a2b7a25783396c1ce5`
- Size: 679 bytes; 14 logical lines
- Coverage: 1–14 (EOF)
- Purpose: Warms the sky around low sun and deepens daytime zenith blue.
- Dependencies / cross-references: SkyInput; frame.sun_direction; frame.rain; daylight/sun_low; sun_glow override.
- Implementation / logic: Computes ray/sun alignment, horizon proximity and daylight/rain weighting.
- Findings: Assumes renderer later draws sun, moon, stars and clouds over result.
- RustChunk relevance: Possible sky sample only; not needed for base game.
- Decision: adaptable; sky shader sample; renderer-specific
- 1.20.1 implications: Sky order/sun vectors differ by renderer and need target validation.
- Follow-up checks: Verify layering, sunrise, dimensions, fog and weather against 1.20.1.
- RustChunk implementation mapping: Example only after 1.20.1 sky renderer contract exists.
- Evidence required before adoption: Sky captures by dimension/time/weather and overlay ordering.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2012. `crates/horizon-shader-api/src/bin/horizon-shader.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b6130282dd38c701e6e9d10342d18aebe830b7a7`
- Size: 9,202 bytes; 307 logical lines
- Coverage: 1–307 (EOF)
- Purpose: CLI creates shader packs from templates, checks packs and writes deterministic .hshader archives.
- Dependencies / cross-references: Pack/check/package API; TOML; WGSL validation; SHA-256; filesystem/CLI args.
- Implementation / logic: Implements new, check and pack commands; writes templates and reports stage/errors.
- Findings: Argument handling permissive: unknown/missing flags may be ignored; --name only replaces quote and not newline/backslash; pack can overwrite output directly, non-atomically.
- RustChunk relevance: Developer tooling if RustChunk supports user shader packs; not game client logic.
- Decision: adaptable; shader authoring and validation CLI; host tooling
- 1.20.1 implications: Targets Horizon API 0.1, not 1.20.1 shader compatibility.
- Follow-up checks: Use typed CLI parsing; reject unknown/duplicate/missing args; TOML-escape names; atomic/no-clobber output.
- RustChunk implementation mapping: Optional host developer tool, not Android runtime.
- Evidence required before adoption: CLI fuzz, TOML injection, no-overwrite and pack round-trip tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2013. `crates/horizon-shader-api/src/check.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa5194a140095fada93cee04ff8e6055325c7f30`
- Size: 21,353 bytes; 607 logical lines
- Coverage: 1–200; 201–400; 401–600; 601–607 (EOF)
- Purpose: Checks shader stage signatures/API rules, validates WGSL via Naga, applies option sets and translates to SPIR-V, MSL and HLSL.
- Dependencies / cross-references: Pack/Manifest; StageKind composition/prelude; Naga parser/validator/backends; PNG decoder; resource-budget constants.
- Implementation / logic: Composes stages, forbids user entrypoints/bindings/reserved names, checks options, decodes PNGs, validates and compiles for backends at default/min/max options.
- Findings: cost() claims samples/gathers/loads but counts only ImageSample and ImageLoad: ImageGather escapes MAX_TEXTURE_READS. Translation returns on first problem set. Up to eight 2048×2048 decoded textures can consume substantial memory.
- RustChunk relevance: Strong pattern for validating untrusted WGSL if RustChunk later supports third-party shader packs; not needed for baseline renderer.
- Decision: adaptable; WGSL validation and backend translation; security-sensitive
- 1.20.1 implications: Naga 30 and stage resources are tied to Horizon and require adapting to RustChunk wgpu/Naga.
- Follow-up checks: Count ImageGather; cap source/manifest/aggregate decoded texture memory; adversarial parser/PNG/backend tests.
- RustChunk implementation mapping: Separate optional shader validation module after 1.20.1 renderer API stabilizes.
- Evidence required before adoption: Shader fuzzing, resource exhaustion, backend matrix and ImageGather budget regression.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2014. `crates/horizon-shader-api/src/inputs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `64aded5373c65f0fa859b34e491ef4bdcf6f0f52`
- Size: 14,837 bytes; 312 logical lines
- Coverage: 1–312 (EOF)
- Purpose: Defines Rust Frame uniform with generated WGSL fields and Sky/Material/Vertex/Weather/Post input contracts.
- Dependencies / cross-references: bytemuck; renderer uniform upload; StageKind; reference generator.
- Implementation / logic: Macros map Rust types to WGSL; exposes Frame::WGSL/offsets and stage-specific input structs.
- Findings: 49-field Frame includes camera/sun/moon/dimension/weather/lighting/reflection/shadow/mod/temporal semantics specific to current renderer; tests do not assert each field offset/alignment vs WGSL.
- RustChunk relevance: Important architecture reference only; not drop-in contract for 1.20.1 renderer.
- Decision: rewrite required; renderer frame and stage input contract; version-specific
- 1.20.1 implications: Frame semantics/IDs must derive from 1.20.1 and target renderer.
- Follow-up checks: Design minimal 1.20.1 Frame; assert every Rust offset/alignment vs WGSL; document derived fields.
- RustChunk implementation mapping: Create RustChunk-specific Frame only after 1.20.1 pipelines exist.
- Evidence required before adoption: Layout ABI tests, Java parity, shader compile and ARM64 renderer tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2015. `crates/horizon-shader-api/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5a22f97e876c0f4152532379b77a3e5c790b8357`
- Size: 736 bytes; 20 logical lines
- Coverage: 1–20 (EOF)
- Purpose: Declares and re-exports public horizon-shader-api modules and types.
- Dependencies / cross-references: check, inputs, manifest, package, reference, stages.
- Implementation / logic: Exports check/Problem/Report, Frame, manifest types, Pack and stage composition APIs.
- Findings: Small cohesive public surface; inherits version-specific internals.
- RustChunk relevance: Crate boundary pattern for separate RustChunk shader SDK.
- Decision: adaptable; shader API crate exports; module structure
- 1.20.1 implications: No Minecraft 1.20.1 protocol/game logic.
- Follow-up checks: Keep public API narrow; expose only invariants enforced by target implementation.
- RustChunk implementation mapping: Possible facade for future shader SDK, not game core.
- Evidence required before adoption: Public API integration tests and semver review.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2016. `crates/horizon-shader-api/src/manifest.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c2efb6cecbc86be3b8a89cda91f629e5f4d3d5ea`
- Size: 43,268 bytes; 1,175 logical lines
- Coverage: 1–200; 201–400; 401–600; 601–800; 801–1000; 1001–1175 (EOF)
- Purpose: Parses and validates shader manifest stages, files, options, settings pages, sprite tags, materials, textures, lights, reflection, shadows and post targets.
- Dependencies / cross-references: TOML deserialization; pack path/id validators; Stage/PackOption; shader composition.
- Implementation / logic: Uses deny_unknown_fields and validates schema/API, ids, stage combinations, paths, counts, ranges, options, page reachability and declarations.
- Findings: Stage-key validation omits Weather for fog, scale/output and sky-only clouds/sun/stars; Vertex is omitted from one sky-only check. Misplaced keys can be accepted/ignored. Some free-form lists lack per-list count/length caps.
- RustChunk relevance: Potential shader manifest foundation, but models Horizon renderer and not standard Java shader packs.
- Decision: adaptable; strict shader manifest schema; validation gaps
- 1.20.1 implications: Stage contracts/options/resources are Horizon-specific; no OptiFine/Iris compatibility.
- Follow-up checks: Use explicit allowlists per stage kind; test misplaced Weather/Vertex keys; bound free-form lists and duplicates.
- RustChunk implementation mapping: Adapt validation concepts into independent RustChunk schema after stage design.
- Evidence required before adoption: Manifest fuzzing, stage-key matrix and renderer integration.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2017. `crates/horizon-shader-api/src/package.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7458639b2af0d81862903f5b18d98f97cffea2d7`
- Size: 8,000 bytes; 212 logical lines
- Coverage: 1–212 (EOF)
- Purpose: Reads folders or ZIP .hshader packs, validates files/sizes and deterministically packages checked content.
- Dependencies / cross-references: Manifest; BTreeMap; ZIP reader/writer; SHA-256; filesystem traversal.
- Implementation / logic: Accepts folder/.hshader; caps ZIP entries at 512, file sizes at 256KiB WGSL/1MiB others; validates paths/UTF-8/required files; sorted ZIP output.
- Findings: ZIP entries are buffered individually before 8MiB aggregate check, allowing excess memory. Folder traversal follows symlink dirs/files without root containment, count or aggregate-size cutoff before reads; may escape pack root/exhaust memory.
- RustChunk relevance: Only useful if RustChunk accepts downloaded third-party shader packs.
- Decision: adaptable; shader package reader and archive safety; security-sensitive
- 1.20.1 implications: .hshader is custom, not Minecraft resource pack/OptiFine/Iris.
- Follow-up checks: Reject symlinks and canonical paths outside root; enforce count/aggregate limits while reading/decompressing; ZIP bomb and loop tests.
- RustChunk implementation mapping: Adapt archive safeguards into future SDK, not gameplay milestone.
- Evidence required before adoption: Symlink escape, directory cycle, archive-bomb, memory-bound and determinism tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2018. `crates/horizon-shader-api/src/reference.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7b2f705f54b673ea2b8dcee5cae5a581db9f80f7`
- Size: 7,861 bytes; 133 logical lines
- Coverage: 1–133 (EOF)
- Purpose: Generates shader API Markdown from stage definitions, Frame/input structs, resources and limits.
- Dependencies / cross-references: Frame offsets; Struct metadata; StageKind; MAX constants; reference test.
- Implementation / logic: Builds stage/input/resource/helper/Frame offset/option/limit reference tables.
- Findings: Source-generated docs reduce drift, but helper prose is partly manual and may diverge.
- RustChunk relevance: Useful docs-maintenance pattern for RustChunk contracts.
- Decision: adaptable; generated shader API reference; documentation tooling
- 1.20.1 implications: Describes Horizon API 0.1, not 1.20.1 truth.
- Follow-up checks: Test helper descriptions vs implementation; run doc consistency; escape Markdown.
- RustChunk implementation mapping: Use generated reference after RustChunk API exists.
- Evidence required before adoption: Generated-doc test and helper behavior tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2019. `crates/horizon-shader-api/src/stages.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7ddcd7897b814c2a9aa286abf2769dc26d93b57d`
- Size: 29,102 bytes; 697 logical lines
- Coverage: 1–200; 201–400; 401–600; 601–697 (EOF)
- Purpose: Defines shader stages, bind-group resources, WGSL preludes/helpers, renderer entrypoints and composition of shared/stage files.
- Dependencies / cross-references: Frame/input contracts; Manifest/Pack; bind group 3; stage WGSL; Naga compose/check.
- Implementation / logic: Builds stage-specific preludes, texture/post-target declarations, helpers/entrypoints and composes shared/per-stage/vertex-material source with origin mapping.
- Findings: Bindings and entrypoints tightly coupled to current renderer. calls_set_surface() counts raw `set_surface(` substring rather than parsing WGSL; comments/spacing can cause false results and affect surface-target allocation. LabPBR/geometry helpers are Horizon-specific.
- RustChunk relevance: Only for future custom WGSL stage API; not core world/chunk code.
- Decision: rewrite required; renderer stage composition and WGSL preludes; version-specific
- 1.20.1 implications: Direct reuse imports 26.3 renderer and binding assumptions.
- Follow-up checks: Replace substring detection with Naga AST analysis; specify 1.20.1 stage resources/bind groups/entrypoints and golden composition tests.
- RustChunk implementation mapping: Build separate RustChunk shader composer after 1.20.1 renderer stabilizes.
- Evidence required before adoption: AST usage tests, WGSL compile, backend builds and screenshot/GPU tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2020. `crates/horizon-shader-api/templates/material/horizon-shaders.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8cb7737e64e408daea637013a078a24e3610258c`
- Size: 266 bytes; 18 logical lines
- Coverage: 1–18 (EOF)
- Purpose: Starter manifest for material shader pack with float contrast option.
- Dependencies / cross-references: Schema1/API0.1; material.wgsl; contrast option.
- Implementation / logic: Defines placeholders, material stage and bounded float option 0.5–2.0.
- Findings: Minimal template; Horizon-specific keys.
- RustChunk relevance: Could inspire RustChunk user shader templates.
- Decision: adaptable; material shader starter manifest; sample-only
- 1.20.1 implications: No vanilla 1.20.1 or Java shader compatibility.
- Follow-up checks: Generate from final schema; test TOML escaping.
- RustChunk implementation mapping: Optional future SDK template.
- Evidence required before adoption: Template generation/parser tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2021. `crates/horizon-shader-api/templates/material/material.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ef9d89a2ba91bf0afefecb15c4fc8ad8a9180d34`
- Size: 390 bytes; 9 logical lines
- Coverage: 1–9 (EOF)
- Purpose: Starter WGSL material applies albedo, tint and lightmap with configurable contrast exponent.
- Dependencies / cross-references: MaterialInput, Surface, lightmap and contrast override.
- Implementation / logic: Raises lightmap channels to contrast power and multiplies albedo/tint.
- Findings: Horizon tint/lightmap semantics.
- RustChunk relevance: Sample only if RustChunk exposes material shader function.
- Decision: adaptable; material shader starter; sample-only
- 1.20.1 implications: 1.20.1 material/lighting output must be validated against Java.
- Follow-up checks: Compare alpha, AO, face shade, block/sky light and gamma.
- RustChunk implementation mapping: Optional example, not engine dependency.
- Evidence required before adoption: Golden image/material parity and GPU tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2022. `crates/horizon-shader-api/templates/post/horizon-shaders.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e135ee838bf720a3a94ef52d76c562b9c5569035`
- Size: 309 bytes; 22 logical lines
- Coverage: 1–22 (EOF)
- Purpose: Starter manifest for one post stage, saturation option and declared GPU budget.
- Dependencies / cross-references: Schema1/API0.1; post.wgsl; saturation; budget.gpu_ms.
- Implementation / logic: Declares full-screen post stage and float slider 0–2.
- Findings: Budget is declarative/overlay-oriented, not measured enforcement.
- RustChunk relevance: Could be sample for optional post SDK.
- Decision: adaptable; post shader starter manifest; sample-only
- 1.20.1 implications: No game compatibility; Horizon post pass.
- Follow-up checks: Decide custom post support; measure/enforce or label budget informational.
- RustChunk implementation mapping: Optional template only after target post interface.
- Evidence required before adoption: GPU budget tests and image snapshots.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2023. `crates/horizon-shader-api/templates/post/post.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d4d1793796fe3fa0b921050c9fe37e63765b08c0`
- Size: 418 bytes; 9 logical lines
- Coverage: 1–9 (EOF)
- Purpose: Starter post shader adjusts saturation by mixing luminance with input color.
- Dependencies / cross-references: PostInput, luminance, saturation override and previous color.
- Implementation / logic: Computes grayscale luminance and mixes with RGB preserving alpha.
- Findings: Assumes Horizon pass pixel and linear color semantics.
- RustChunk relevance: Basic sample if RustChunk adds post pipeline.
- Decision: adaptable; post shader starter example; sample-only
- 1.20.1 implications: No Java shader compatibility or target color-space proof.
- Follow-up checks: Test gamma/linear, HDR, alpha and color management.
- RustChunk implementation mapping: Optional shader fixture.
- Evidence required before adoption: Image comparison and color-space tests.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2024. `crates/horizon-shader-api/templates/sky/horizon-shaders.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5939f25fdf3e70d7455e0673d729edcfed1b6863`
- Size: 236 bytes; 18 logical lines
- Coverage: 1–18 (EOF)
- Purpose: Starter manifest for sky stage with configurable sun glow.
- Dependencies / cross-references: Schema1/API0.1; sky.wgsl; glow override.
- Implementation / logic: Declares one sky stage and float glow 0–1.
- Findings: Assumes Horizon sky pass order.
- RustChunk relevance: Useful only if RustChunk supports custom sky.
- Decision: adaptable; sky shader starter manifest; sample-only
- 1.20.1 implications: 1.20.1 sky order/time/dimensions need target validation.
- Follow-up checks: Test layering, sunrise, dimensions, fog and weather.
- RustChunk implementation mapping: Optional future SDK template.
- Evidence required before adoption: Sky captures by dimension/time/weather.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.

### 2025. `crates/horizon-shader-api/templates/sky/sky.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b5aaaa7addece2f96f7dd90bbf4369d3e6f537d3`
- Size: 411 bytes; 9 logical lines
- Coverage: 1–9 (EOF)
- Purpose: Starter sky shader adds sun-aligned warm halo to incoming color.
- Dependencies / cross-references: SkyInput; sun_direction; daylight/sun_low; glow override.
- Implementation / logic: Uses direction/sun dot product and falloff to tint sky near sun.
- Findings: Depends on normalized sun vector and later sky overlays.
- RustChunk relevance: Possible extension sample, not base gameplay.
- Decision: adaptable; sky shader starter example; sample-only
- 1.20.1 implications: Target 1.20.1 sky interface/composition need independent validation.
- Follow-up checks: Test sun position, fog, weather, sunrise/sunset and overlay order.
- RustChunk implementation mapping: Optional sample after RustChunk sky API defined.
- Evidence required before adoption: Golden screenshots and stage-order tests on desktop/Android.
- Completion state: COMPLETE — full content read through EOF; source SHA and exact byte size match the pinned recursive tree; embedded tests/examples were inspected only and not executed.
