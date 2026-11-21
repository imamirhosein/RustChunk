# RustChunk Starling Audit Ledger — Entries 2001–2200

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 2001–2200 (200 unique source files; one shared Ledger for this stage).
- Current progress: **75/200**. Checkpoints **2001–2025** and **2026–2050** are complete; reporting section **2001–2050 is closed**.
- One-time expanded authorization: entries **2001–2100 only**, comprising sections 2001–2050 and 2051–2100. Both 50-file sections 2001–2050 and 2051–2100 are covered by the user's one-time authorization; no mid-section permission request is needed at 2050.
- Cadence unchanged: exact checkpoints of 25 files, boundary reports at every 50 files; continue through 2100 then stop. Entries 2101–2200 are not authorized.
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

### 2026. `crates/horizon-shader-api/tests/check.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9cd377627b0b455a9fb264184cc79f85bb679c9d`
- Size: 9294 bytes; 261 logical lines
- Coverage: 1–261 (EOF)
- Purpose: Tests horizon-shader-api's WGSL validation, stage composition, diagnostics, texture budgets and material/vertex behavior.
- Dependencies / cross-references: Pack, Pack::from_files, check, Problem, StageKind, Naga WGSL validation.
- Implementation / logic: The shader-pack check suite exercises valid packs, diagnostics, stage composition, texture-read budgets and material/vertex interfaces.
- Findings: The budget case generates 70 textureLoad calls; no case exercises ImageGather, leaving the checker accounting gap unguarded. Tests are not a runtime proof.
- RustChunk relevance: Useful as test design only; it exposes a missing texture-read budget regression case, not a runtime feature.
- Decision: reference; WGSL checker test coverage; test-only
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Add ImageGather/GatherCmp budget cases, malformed packs, oversized files, deep calls and helper-chain regression coverage; retain diagnostics assertions.
- RustChunk implementation mapping: Do not port test code; copy the missing negative cases into RustChunk shader-checker tests after a checker exists.
- Evidence required before adoption: Budget tests must cover sample/gather/load/store combinations and demonstrate deterministic diagnostics; no execution evidence exists yet.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2027. `crates/horizon-shader-api/tests/reference.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6f9709123efd171b166dc526a34a50640c8d081f`
- Size: 4531 bytes; 125 logical lines
- Coverage: 1–125 (EOF)
- Purpose: Checks WGSL stage preludes, Frame layout parity, sample pack compilation and generated reference documentation.
- Dependencies / cross-references: Frame, StageKind, Pack, compose, generated `docs/api/shaders-reference.md`, Naga.
- Implementation / logic: Compiles representative stage preludes, compares Frame WGSL size/member offsets to Rust offsets, checks sample-stage composition and generated-doc freshness.
- Findings: Layout checks size and offsets but not a complete field-type/interface contract; Naga validation does not establish Android backend correctness.
- RustChunk relevance: Useful as a pattern for checking Rust/WGSL layout and source diagnostics; no renderer execution.
- Decision: reference; Naga shader-layout tests; test-only
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Assert every Frame member's WGSL type/alignment as well as offset/size; compare SPIR-V/Vulkan and Android backend output.
- RustChunk implementation mapping: Use compile-time/host tests for Rust GPU structures, then device-specific backend and rendering comparisons.
- Evidence required before adoption: Field-by-field ABI comparison, target-backend compilation and Android GPU validation are required before relying on layout parity.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2028. `crates/horizon-shader-api/tests/samples/schema-1/common.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8fef8baf9e3866e2261fadcdfdda7ded3630be8b`
- Size: 92 bytes; 3 logical lines
- Coverage: 1–3 (EOF)
- Purpose: Provides a minimal shared WGSL luminance function for the schema-1 sample pack.
- Dependencies / cross-references: WGSL built-ins and shared shader include list in the sample manifest.
- Implementation / logic: Defines Rec.709 luminance as a weighted dot product over RGB.
- Findings: Math is generic, but color-space and input-linearization assumptions belong to the calling renderer.
- RustChunk relevance: Generic luminance math may be reused only after matching linear-color conventions.
- Decision: adaptable; generic luminance math; sample-only
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Check against the target renderer's linear/sRGB contract and known vanilla luminance fixtures.
- RustChunk implementation mapping: Reuse only the weighted RGB formula if RustChunk's renderer input is already linear and matches the same convention.
- Evidence required before adoption: Vanilla color-space reference images and unit tests for known RGB values.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2029. `crates/horizon-shader-api/tests/samples/schema-1/horizon-shaders.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `77211047f2dc68fb687ae8e66fdcae8d7671ad83`
- Size: 1213 bytes; 76 logical lines
- Coverage: 1–76 (EOF)
- Purpose: Provides the schema-1 sample manifest consumed by the parser and pack tests.
- Dependencies / cross-references: Schema-1 Manifest parser, sample WGSL files, reflection/shadow/material/tag/option schema.
- Implementation / logic: Declares sample pack identity, shared WGSL, sky/material/post stages, options, reflection, shadow, tags and GPU budget.
- Findings: Header claims every schema-1 key, but this fixture omits several optional schema features covered elsewhere by tests; it is not a complete conformance fixture.
- RustChunk relevance: Useful as a sample fixture pattern, not as a Minecraft resource-pack schema.
- Decision: reference; schema-1 manifest fixture; sample-only
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Make the fixture genuinely complete or narrow its header claim; test every schema key and reject undocumented keys.
- RustChunk implementation mapping: Use only as a test fixture after a separate RustChunk shader manifest is specified.
- Evidence required before adoption: Schema conformance suite proving the sample covers every claimed key and version.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2030. `crates/horizon-shader-api/tests/samples/schema-1/material.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eed97590e73580486564e8793b5be07ad2eba5e4`
- Size: 564 bytes; 12 logical lines
- Coverage: 1–12 (EOF)
- Purpose: Implements the sample material function for terrain-like surfaces and optional fluid reflections.
- Dependencies / cross-references: MaterialInput, Surface, Horizon WGSL prelude, reflection texture/sampler and frame flags.
- Implementation / logic: Combines albedo, tint and lightmap, adds luminance modulation and optionally samples a reflection texture for fluid faces.
- Findings: Uses Horizon MaterialInput/Surface fields and reflection bindings; fluid/face IDs and lighting semantics are not Minecraft 1.20.1 contracts.
- RustChunk relevance: Surface composition is a renderer feature that must be rebuilt around the target game's actual data.
- Decision: rewrite required; material stage function; Horizon-specific
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Reconstruct MaterialInput/Surface and fluid/face semantics from 1.20.1; test opaque, cutout, translucent and fluids.
- RustChunk implementation mapping: Rebuild against a target-version `MaterialInput` contract; never copy the WGSL fields or IDs directly.
- Evidence required before adoption: 1.20.1 Java/client source, modded-fluid integration fixtures and pixel-level render comparisons.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2031. `crates/horizon-shader-api/tests/samples/schema-1/post/grade.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8daa1eab00ccb5d1ec0bcf74cfa7656d43ace094`
- Size: 293 bytes; 8 logical lines
- Coverage: 1–8 (EOF)
- Purpose: Implements sample post-process saturation and tone mapping.
- Dependencies / cross-references: PostInput, luminance helper, saturation and tonemap options.
- Implementation / logic: Mixes luminance with source color using saturation and applies a tone-map branch.
- Findings: The declared choice has three values, but `tonemap > 0` selects the same branch for both nonzero choices; Soft and Filmic are not distinguished.
- RustChunk relevance: Color-grading math is an optional renderer feature, not vanilla gameplay behavior.
- Decision: adaptable; post-process color grading; renderer-specific
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Implement each tonemap choice distinctly or reduce the declared choices; test linear/sRGB and alpha preservation.
- RustChunk implementation mapping: Optional post-process pass once color-space and render graph are stable.
- Evidence required before adoption: Distinct outputs for all declared choices plus image comparison on target backends.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2032. `crates/horizon-shader-api/tests/samples/schema-1/post/vignette.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `84f7261cbc8c267a4e28766a5f362e40a9c06651`
- Size: 287 bytes; 8 logical lines
- Coverage: 1–8 (EOF)
- Purpose: Implements sample post-process vignette darkening.
- Dependencies / cross-references: PostInput, vignette and samples options, frame UV/color.
- Implementation / logic: Darkens the image toward the corners behind a boolean option.
- Findings: The manifest declares `samples`, but this shader never reads it; UV/color conventions are renderer-specific.
- RustChunk relevance: Vignette is optional post-processing and unrelated to gameplay correctness.
- Decision: adaptable; vignette post-process; renderer-specific
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Remove or consume the unused samples option; compare edge falloff and UV origin against renderer images.
- RustChunk implementation mapping: Optional post-process shader only after renderer UV conventions are defined.
- Evidence required before adoption: Image comparison with option on/off, multiple resolutions and UV origins.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2033. `crates/horizon-shader-api/tests/samples/schema-1/README.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a8ad4583b073b0ed8d14e9b48552c64919ac005f`
- Size: 89 bytes; 1 logical lines
- Coverage: 1–1 (EOF)
- Purpose: Documents which tests consume the schema-1 sample files.
- Dependencies / cross-references: tests/schema.rs and tests/reference.rs in horizon-shader-api.
- Implementation / logic: Identifies the schema-1 fixture as input to schema and stage-reference tests.
- Findings: Documentation is one line and does not describe the broader schema or prove compatibility.
- RustChunk relevance: Documentation fixture only.
- Decision: reference; shader fixture documentation; test-only
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Keep the README synchronized with the test fixture and actual schema.
- RustChunk implementation mapping: Use as a reference for shader feature documentation.
- Evidence required before adoption: Docs/test fixture diff and generated documentation check.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2034. `crates/horizon-shader-api/tests/samples/schema-1/sky.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `55ca0ac1d6e0516a908ab8c857b7614d12c75d92`
- Size: 65 bytes; 3 logical lines
- Coverage: 1–3 (EOF)
- Purpose: Implements the sample sky stage as a passthrough.
- Dependencies / cross-references: SkyInput and Horizon sky-stage WGSL prelude.
- Implementation / logic: Returns the supplied sky input color unchanged.
- Findings: Only demonstrates function shape; sky color, cloud, sun and fog composition are Horizon-specific.
- RustChunk relevance: Demonstrates a trivial stage signature only.
- Decision: adaptable; sky pass passthrough; sample-only
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Test sky composition, weather, fog and overlays against 1.20.1 screenshots before reuse.
- RustChunk implementation mapping: Use only to test shader stage interfaces after the sky pipeline is specified.
- Evidence required before adoption: Target-version sky pass capture and weather/fog ordering tests.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2035. `crates/horizon-shader-api/tests/schema.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7a5a9970973112b6e8b4bb7e64c39ab48d8c3353`
- Size: 23055 bytes; 664 logical lines
- Coverage: 1–400; 401–664 (EOF)
- Purpose: Exercises the schema-1 parser, validation rules, textures, package creation and read-back.
- Dependencies / cross-references: Manifest, Pack, PageItem, OptionKind, ZIP/PNG helpers and test temp dirs.
- Implementation / logic: Covers schema keys, validation failures, options, material/tag/light rules, pages, textures, PNG decoding, ZIP packaging and round-trip behavior.
- Findings: Tests use PID-derived temporary directory names and remove them recursively; collision/isolation should be considered. No tests were run.
- RustChunk relevance: Useful as validation-test ideas; tests do not establish cross-version compatibility.
- Decision: reference; manifest and package validation tests; test-only
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Add isolation-safe temp dirs and package corruption/resource-limit cases; run only with separate authorization.
- RustChunk implementation mapping: Translate selected parser edge cases into RustChunk tests; never import the fixture's Horizon schema as the game's resource format.
- Evidence required before adoption: Adversarial manifest/ZIP/PNG tests, temp isolation and clean test-run evidence.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2036. `crates/horizon-shader-api/tests/sdk.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ef69a88a010922b3fc4fcc4f5724f70e63ec2bb4`
- Size: 3477 bytes; 92 logical lines
- Coverage: 1–92 (EOF)
- Purpose: Exercises the shader SDK CLI and template/package workflow end to end.
- Dependencies / cross-references: Pack, check, horizon-shader executable, CLI commands and temporary filesystem.
- Implementation / logic: Exercises example pack validation, template generation, CLI check JSON and package round-trip through the built executable.
- Findings: Subprocess output is captured without an explicit timeout or output cap; tests are host-only and were inspected, not executed.
- RustChunk relevance: Useful as host-side CLI test organization, not Android app functionality.
- Decision: reference; shader CLI integration tests; host-only
- 1.20.1 implications: No direct Minecraft 1.20.1/Forge gameplay integration is demonstrated; sample and test contracts are Horizon-specific.
- Follow-up checks: Add subprocess timeout, output bounds and deterministic locale/platform tests; inspect CLI failure cleanup.
- RustChunk implementation mapping: Mirror host-tool integration tests in RustChunk's independent shader SDK, with controlled temp paths and process limits.
- Evidence required before adoption: Host CI on supported OSes, timeout/output cap tests and CLI exit-code evidence.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2037. `crates/horizon-shaders/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fe81875160424f3e6795bc7676a5fcd103c18a20`
- Size: 727 bytes; 21 logical lines
- Coverage: 1–21 (EOF)
- Purpose: Declares the separate GLSL shader-pack reader/translator crate and its dependencies.
- Dependencies / cross-references: Workspace Rust settings, horizon-core, Naga 30.0.1, half, PNG and ZIP.
- Implementation / logic: Declares the separate `horizon-shaders` crate with Naga GLSL/WGSL input, PNG and ZIP dependencies under Rust 1.85.
- Findings: Naga 30.0.1 and this dependency graph are tied to the pinned Starling workspace, not automatically suitable for Android or Java 1.20.1.
- RustChunk relevance: Dependency choices are research input only; build/size/Android support must be checked independently.
- Decision: reference; GLSL shaderpack dependencies; version-specific
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Check licenses, dependency graph, Naga backend support, native Android build footprint and ABI impact before adoption.
- RustChunk implementation mapping: Choose versions only after Android ARM64 builds and dependency/licence audit; don't copy the manifest wholesale.
- Evidence required before adoption: Android ARM64 dependency builds, APK size/memory measurements, licences and backend compatibility.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2038. `crates/horizon-shaders/examples/pack_report.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b686b63b029a8f85f3a065251a35dc3566bdf5c`
- Size: 18442 bytes; 505 logical lines
- Coverage: 1–400; 401–505 (EOF)
- Purpose: Reports which programs in a shader pack translate under synthetic Horizon pipeline inputs.
- Dependencies / cross-references: horizon-shaders Pack/Properties/Macros, GameInputs, load_post, Naga SPIR-V writer.
- Implementation / logic: Discovers packs, loads programs for dimension folders, groups translation failures, optionally prints programs, stress variants and SPIR-V results.
- Findings: `--example N` prints at most one sample per cause; invalid `--dims` tokens are silently dropped; backend check examines only the first entry point; synthetic game inputs approximate rather than prove actual pipeline compatibility.
- RustChunk relevance: Useful for offline shader-pack diagnostics, not runtime proof of the Android Vulkan pipeline.
- Decision: reference; shader-pack diagnostic report tool; host-only
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Make --example behavior match help, reject invalid dimensions/arguments, test all entry points and derive real pipeline inputs.
- RustChunk implementation mapping: An offline troubleshooting utility could be designed later; not part of the Android render loop.
- Evidence required before adoption: Real packs and real pipeline input captures; every entry point and stress dimension verified.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2039. `crates/horizon-shaders/src/biomes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c9ede57e0a33a9281cb7486a2c019a31278c9e26`
- Size: 5318 bytes; 159 logical lines
- Coverage: 1–159 (EOF)
- Purpose: Provides shader-pack biome constants, category inference and precipitation values.
- Dependencies / cross-references: horizon-core version, biome registry identifiers and shader custom uniform constants.
- Implementation / logic: Builds CAT/PPT/BIOME constants from an ordered biome list, maps legacy names, infers categories by name substrings and applies the 0.15 precipitation threshold.
- Findings: Substring categories can misclassify modded/new names; exact biome registry ordering, legacy replacements, categories and temperature rule must come from 1.20.1 data.
- RustChunk relevance: Potential shader compatibility extension only after exact 1.20.1 registry/category mappings.
- Decision: rewrite required; biome category and precipitation constants; version-specific
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Use the exact 1.20.1 biome registry order, legacy map, category data and precipitation oracle; include modded biome fallback rules.
- RustChunk implementation mapping: Implement a data-driven 1.20.1 adapter from vanilla registry data and verify with a Java oracle.
- Evidence required before adoption: Java 1.20.1 registry oracle for every biome and precipitation threshold, including modded biomes.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2040. `crates/horizon-shaders/src/block_ids.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ebcf6d992c0d0f2731c5b3fac20e5f2b7ce1734`
- Size: 5927 bytes; 151 logical lines
- Coverage: 1–151 (EOF)
- Purpose: Parses shader-pack block/entity/item ID declarations and optional block-state filters.
- Dependencies / cross-references: Properties parser, resource names and shader-pack block/entity/item configuration files.
- Implementation / logic: Parses `block.<id>` style names and conjunctive state filters, prefers more filters, and exports plain entity/item IDs.
- Findings: No identifier or ID-range validation; malformed/empty filters can be accepted; `%tag` expansion is absent and note wording is block-specific for all kinds.
- RustChunk relevance: Useful for shaderpack-defined IDs after connecting to real 1.20.1 block-state/registry data.
- Decision: adaptable; block state and entity/item ID rules; optional shaderpack extension
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Validate resource identifiers, signed/range-bounded IDs and filter grammar; implement tag expansion and registry-backed matching.
- RustChunk implementation mapping: Map names through 1.20.1 registries and blockstate properties; keep separate from core vanilla IDs.
- Evidence required before adoption: Registry and blockstate differential fixtures, malformed identifier fuzzing and tag semantics tests.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2041. `crates/horizon-shaders/src/condition.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e872f415fde53fbea100dc9c35edb9ac228f4e2d`
- Size: 13963 bytes; 406 logical lines
- Coverage: 1–400; 401–406 (EOF)
- Purpose: Evaluates preprocessor conditional arithmetic and nested conditional directives.
- Dependencies / cross-references: Macros lexer/expander, C-like operators, conditional directive stack.
- Implementation / logic: Evaluates C-like `#if` arithmetic, defined(), nested conditionals and macro-expanded identifiers.
- Findings: No short-circuit for &&/||; macro-expanded float values can be evaluated with integer division; #ifdef/#ifndef ignores extra tokens and can accept a missing name. Integer precision/overflow also diverges from C.
- RustChunk relevance: Potential shaderpack conditional compatibility, but requires a real C/GLSL differential test suite.
- Decision: adaptable; GLSL #if evaluator; optional shaderpack extension
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Implement short-circuit, macro float semantics, strict ifdef syntax, integer overflow semantics and differential tests against a C preprocessor.
- RustChunk implementation mapping: Separate the shaderpack preprocessor from Minecraft protocol/game simulation; adopt only after differential conformance.
- Evidence required before adoption: Differential tests against a reference C/GLSL preprocessor over a broad condition corpus and adversarial resource limits.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2042. `crates/horizon-shaders/src/config.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2fbf536fe43403a96859464d4a149bb93f7f1e86`
- Size: 20257 bytes; 575 logical lines
- Coverage: 1–400; 401–575 (EOF)
- Purpose: Reads shader-pack render-target and shadow-map configuration from preprocessed GLSL.
- Dependencies / cross-references: Preprocessed source, Properties/Pack, GLSL constant declarations and comment origins.
- Implementation / logic: Reads DRAWBUFFERS/RENDERTARGETS comments, colortex formats/clear/mipmap settings and shadow constants after preprocessing.
- Findings: Shadow settings do not consistently validate declared GLSL types, non-finite/range values can pass silently, and DRAWBUFFERS accepts out-of-range single digits 8/9.
- RustChunk relevance: Potential optional render-target config reader, separate from chunk/world core.
- Decision: adaptable; render-target and shadow config parser; renderer-specific
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Validate all supported GLSL types, finite numeric ranges, buffer index bounds and malformed directive fallback; test against OptiFine/Iris fixtures.
- RustChunk implementation mapping: Keep shaderpack render-target settings in an optional rendering subsystem with explicit GPU capability checks.
- Evidence required before adoption: Known shader-pack corpus comparisons, invalid-range fuzzing and device render-target capability validation.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2043. `crates/horizon-shaders/src/core_shader.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8a4f96db86e7efff3ba27705f6b3372a044daa99`
- Size: 27257 bytes; 653 logical lines
- Coverage: 1–400; 401–653 (EOF)
- Purpose: Translates vanilla resource-pack core vertex/fragment GLSL into validated Naga modules.
- Dependencies / cross-references: Naga GLSL frontend/validator, Glue, include callback, Horizon core shader uniform blocks.
- Implementation / logic: Expands includes, strips comments, rewrites uniforms/samplers/attributes/varyings and validates paired vertex/fragment modules through Naga.
- Findings: Hardcoded 26.3 uniform blocks and samplers; angle-bracket include paths are not traversal-normalized; fragment/vertex interfaces compare locations but not types/interpolation; line-oriented declaration parsing omits complex GLSL forms.
- RustChunk relevance: Potential reference for a future Minecraft core shader bridge, but full 1.20.1 shader contracts must be reconstructed.
- Decision: rewrite required; vanilla core GLSL translation; Minecraft 26.3-specific
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Add path normalization for all include forms, full GLSL declaration parsing, varying type/interpolation checks and 1.20.1 vanilla shader fixtures.
- RustChunk implementation mapping: Treat the code as a structural reference only; build a new 1.20.1 shader bridge with exact vanilla uniform/binding contracts.
- Evidence required before adoption: 1.20.1 vanilla core-shader source corpus, include security tests, varying ABI checks and cross-backend output.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2044. `crates/horizon-shaders/src/custom.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `671750e516c3c4d0727d09c666efce250a8e8f39`
- Size: 28364 bytes; 720 logical lines
- Coverage: 1–400; 401–720 (EOF)
- Purpose: Parses and evaluates custom shader-pack uniforms and variables, including vector math and stateful functions.
- Dependencies / cross-references: Properties, Macros, biome constants, builtin GameInputs and frame delta.
- Implementation / logic: Parses file-ordered custom uniforms/variables, vector/scalar arithmetic, builtin calls, biome constants, deterministic random and smooth state.
- Findings: `if`/`ifb` eagerly evaluate every branch, `in()` with no arguments can panic, Val::vector accepts unchecked component counts, and expression depth/argument counts are uncapped.
- RustChunk relevance: Potential optional shaderpack custom-uniform runtime; correctness and resource limits need fixing first.
- Decision: adaptable; custom expression evaluator; optional shaderpack extension
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Make conditional functions lazy, validate arity and component bounds, cap expression depth/arguments and define divide-by-zero behavior.
- RustChunk implementation mapping: Keep custom expressions sandboxed and bounded in a separate optional subsystem.
- Evidence required before adoption: Fuzzed expressions, no-panic guarantees, bounded CPU/memory, eager-vs-lazy behavior fixtures and Iris 1.11.7 differential evidence.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2045. `crates/horizon-shaders/src/discover.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7d01d41b33a01a7c762fffb1e77a69abf8ba079a`
- Size: 2888 bytes; 82 logical lines
- Coverage: 1–82 (EOF)
- Purpose: Discovers available shader-pack folders and ZIP files in a directory.
- Dependencies / cross-references: Filesystem read_dir, Path, Pack::open and ZIP/folder metadata.
- Implementation / logic: Lists visible folders and ZIP files, probes each with Pack::open and sorts by case-insensitive name.
- Findings: read_dir errors are silently converted to an empty result; probing every ZIP reads the full archive into memory; symlink folders/ZIPs can point outside the selected directory.
- RustChunk relevance: Potential launcher UI shaderpack discovery, with safe path and archive handling.
- Decision: adaptable; shader-pack discovery; host-only
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Report unreadable directories distinctly, bound pack probe sizes/counts and reject symlink escapes.
- RustChunk implementation mapping: Provide a sandboxed shader-pack browser/selector; never let pack paths escape the selected import location.
- Evidence required before adoption: Permission/symlink tests, large directory stress, unreadable folder behavior and bounded archive probes.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2046. `crates/horizon-shaders/src/lang.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `394727ce8b3edccbdb1daac12eafa0f6947c8c3e`
- Size: 2655 bytes; 81 logical lines
- Coverage: 1–81 (EOF)
- Purpose: Loads shader-pack .lang localization files with English fallback.
- Dependencies / cross-references: Pack::list/read_text and locale file names under `lang/`.
- Implementation / logic: Loads English then the selected locale case-insensitively, preserving section formatting codes and allowing locale entries to override English.
- Findings: Simple key=value parsing has no escape/continuation handling and duplicate keys silently use the last value; this is not Minecraft's general resource-language system.
- RustChunk relevance: Localization fallback pattern only; Minecraft resource translation has a broader format.
- Decision: adaptable; localization fallback utility; portable
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Test duplicate locale files, malformed UTF-8, empty keys, newline handling and target resource-bundle precedence.
- RustChunk implementation mapping: Use as a small locale-overlay helper only, not as the client localization backend without adaptation.
- Evidence required before adoption: Locale precedence against the target resource loader and malformed/duplicate translation fixtures.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2047. `crates/horizon-shaders/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ef16db4b58441a4369e5fec34723a44048e790a9`
- Size: 3367 bytes; 70 logical lines
- Coverage: 1–70 (EOF)
- Purpose: Defines the public module/export surface of the horizon-shaders crate.
- Dependencies / cross-references: All crate modules: pack, options, properties, preprocess, translate, post, textures, custom, etc.
- Implementation / logic: Exports pack, options, properties, GLSL translation, custom uniforms, textures, render-target settings, localization and post-program types.
- Findings: The crate documentation states it knows nothing of wgpu and targets a Horizon clean-room shader-pack specification; this export surface is not a Minecraft client API.
- RustChunk relevance: Useful architecture map for optional shader support only.
- Decision: reference; shaderpack crate public API map; module-boundary only
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Keep this module map synchronized when features are added; no independent implementation adoption is implied.
- RustChunk implementation mapping: Treat exports as an index of optional Starling shaderpack components, not a RustChunk crate architecture.
- Evidence required before adoption: Explicit architecture decision document and a separately scoped, tested shader subsystem.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2048. `crates/horizon-shaders/src/macros.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `48038cd8efebfb21ad955ece411d94562eb4ae00`
- Size: 18751 bytes; 489 logical lines
- Coverage: 1–400; 401–489 (EOF)
- Purpose: Implements object-like and function-like shader macros plus renderer/version feature defines.
- Dependencies / cross-references: Macros BTreeMap, lex, token parsing, properties and Horizon/Iris compatibility constants.
- Implementation / logic: Provides MC_VERSION, render-stage and Iris feature macros, object/function macro expansion, token pasting and recursion hiding.
- Findings: Expansion depth/output are unbounded; function macro arity is not validated; supported features named only as required are not defined as IRIS_FEATURE_*; defaults hardcode MC 26.3/Iris 1.11.7.
- RustChunk relevance: Potential shaderpack preprocessor layer, after conformance and resource-bound hardening.
- Decision: adaptable; GLSL macro compatibility layer; optional shaderpack extension
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Define supported feature semantics for both optional and required declarations; validate macro names/arity and cap recursive expansion/output.
- RustChunk implementation mapping: Keep a bounded macro-expansion layer for optional shaderpack compatibility, isolated from normal game resource parsing.
- Evidence required before adoption: Macro corpus differential tests, bounded recursive-expansion tests, required/optional feature tests and exact target-version defines.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2049. `crates/horizon-shaders/src/options.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a4e95299951ae5b91664a7dceef68900bca44b1f`
- Size: 22952 bytes; 580 logical lines
- Coverage: 1–400; 401–580 (EOF)
- Purpose: Discovers player-selectable shader options, loads chosen values and rewrites shader source lines.
- Dependencies / cross-references: Pack, Macros, Properties, Values and all shaderpack source files/includes.
- Implementation / logic: Discovers switches/value options across sources and one-level includes, parses saved choices and applies them to source text.
- Findings: switches_on misses switches tested only via `#if defined`; option_texts follows only one include level; public apply accepts arbitrary unvalidated replacement strings, while only Pack::with_chosen filters values.
- RustChunk relevance: Potential option UI and persisted choices, but source rewriting must be strictly validated.
- Decision: adaptable; shaderpack option discovery and apply; optional shaderpack extension
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Follow includes transitively by normalized path, align switches_on with #if defined detection and validate persisted replacements at every public entry point.
- RustChunk implementation mapping: Implement choices in a safe immutable source overlay or typed AST, not unrestricted textual substitution.
- Evidence required before adoption: Options corpus spanning direct/transitive includes and all #if forms; malicious saved-choice strings must not alter unrelated source.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

### 2050. `crates/horizon-shaders/src/pack.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `483f5112612527514914f521a54e258968c4e323`
- Size: 12244 bytes; 345 logical lines
- Coverage: 1–345 (EOF)
- Purpose: Opens shader packs from a folder or ZIP and reads normalized files and chosen source values.
- Dependencies / cross-references: Filesystem APIs, ZIP archive, Arc-backed in-memory bytes, normalize(), options::apply and Values.
- Implementation / logic: Opens a `shaders/` folder or ZIP, normalizes logical paths, lists files and applies chosen values while reading source text.
- Findings: ZIP input and decompressed entry sizes are unbounded; folder reads follow symlinks beyond the pack root; recursive file enumeration has no file/depth caps. Lexical path normalization alone does not prevent symlink escape.
- RustChunk relevance: Pack abstraction is relevant, but must be rebuilt with bounded ZIP reads and symlink-safe filesystem access.
- Decision: rewrite required; pack folder/ZIP resource reader; security-sensitive
- 1.20.1 implications: This crate targets the pinned Starling shaderpack/renderer behavior around Minecraft 26.3 and Naga 30.0.1; no direct Java 1.20.1 or Forge compatibility is established.
- Follow-up checks: Bound archive bytes, file count and expanded entry size; canonicalize/reject symlinks and enforce pack-root containment on every read.
- RustChunk implementation mapping: Rebuild resource loading with bounded decompression, canonical path checks and symlink-safe root confinement.
- Evidence required before adoption: ZIP-bomb, symlink escape, TOCTOU, duplicate-entry, huge-file and path-traversal tests; strict memory/file-count caps.
- Completion state: COMPLETE — full source content read serially through EOF; exact source SHA and byte size match the pinned recursive tree; tests/examples were inspected only and not executed.

## Checkpoint 2026–2050 closeout

- Entries 2026–2050 complete (25/25); active Ledger now holds 50 entries for 2001–2050.
- All 25 source files were reviewed one at a time through EOF; files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines.
- Exact path, SHA and byte size for all 25 entries match the complete pinned source tree. All 15 required fields occur exactly once per entry.
- The 50-file section 2001–2050 is closed. Continue with 2051–2075 under the user's pre-authorized 100-file exception; next source path is `crates/horizon-shaders/src/post.rs`.
- Continue to 2100, close that section, then stop. Entries 2101+ remain unauthorized.



### 2051. `crates/horizon-shaders/src/post.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `024981f94513c17613a072beb895f04dc251c28b`
- Size: 75190 bytes; 1957 logical lines
- Coverage: 1–400; 401–800; 801–1200; 1201–1600; 1601–1957 (EOF)
- Purpose: Coordinates the shaderpack's setup, gbuffers, shadow, deferred, composite, final and compute passes; resolves fallback programs and pass resources.
- Dependencies / cross-references: Pack, Properties, Values, preprocess, translate/translate_geometry, texture loader and Horizon renderer interfaces.
- Implementation / logic: Parses program enable expressions, custom image/buffer declarations, dimensions, blend and flip state, then builds ordered render-pass and geometry/compute descriptions.
- Findings: Condition evaluation errors are fail-open (treated as enabled). Buffer/image extents and relative scales lack complete finite/product/device-limit checks; image names and buffer indices are weakly validated. Geometry-stage settings appear incompletely consulted. Panic capture can hide internal defects.
- RustChunk relevance: Optional shaderpack render-graph manager only; not launcher, chunk engine or game protocol.
- Decision: rewrite required; Horizon shaderpack render-pass manager; renderer-specific
- 1.20.1 implications: Pass names, frame uniforms, buffer formats and draw semantics are Horizon-specific and provide no 1.20.1/Forge proof.
- Follow-up checks: Fail closed on condition errors; enforce checked byte/pixel/GPU limits; validate identifiers/indices; parse all geometry settings; report internal panics distinctly.
- RustChunk implementation mapping: Only consider an isolated shaderpack subsystem after RustChunk's own 1.20.1 frame graph and renderer exist.
- Evidence required before adoption: Actual 1.20.1 pack corpus, OpenGL/Iris oracle, resource-limit fuzzing and Android GPU validation.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2052. `crates/horizon-shaders/src/preprocess.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3e47d00aa1cd89b7ffccb173be3e9e074ba22c13`
- Size: 19240 bytes; 489 logical lines
- Coverage: 1–400; 401–489 (EOF)
- Purpose: Preprocesses shaderpack GLSL: include expansion, macros, conditionals, version/extensions, comments and source-origin tracking.
- Dependencies / cross-references: Pack file reader, Macros, condition evaluator, option Values and source Origin records.
- Implementation / logic: Walks active source lines, expands included files/macros and preserves line/file mappings with include-depth and selected macro-line limits.
- Findings: Depth is capped but aggregate include fan-out/output size is not; source IDs cast to u16 unchecked. Unclosed comments and malformed #version can be silently accepted; include delimiter grammar is loose; line joining may skew source origins.
- RustChunk relevance: Possible GLSL preprocessor layer for an optional compatibility module only.
- Decision: adaptable; GLSL preprocessor and origin tracking; optional shaderpack subsystem
- 1.20.1 implications: Dialect semantics must be verified against the chosen GLSL/OptiFine/Iris version; not Java or Forge preprocessing.
- Follow-up checks: Bound total bytes/lines/includes, check source-id conversion, enforce strict directive grammar and add malformed-comment diagnostics.
- RustChunk implementation mapping: Keep distinct from Minecraft assets/registries; use bounded include graph and explicit source map.
- Evidence required before adoption: Differential reference-preprocessor corpus, include fan-out/cycle stress and source-line accuracy tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2053. `crates/horizon-shaders/src/properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6768d083ad6ddbc7b45b831e899d98978aa2dd69`
- Size: 8903 bytes; 251 logical lines
- Coverage: 1–251 (EOF)
- Purpose: Parses shaderpack properties-like key/value records, comments, continuations and conditional define directives.
- Dependencies / cross-references: Macros, conditional evaluator, property entries and parser notes.
- Implementation / logic: Builds ordered properties, processes conditional and define/undef lines and records duplicate-key replacement notes.
- Findings: Input bytes, line/entry counts and continuation length are uncapped; u32 line counting is unchecked; duplicate vector replacement can become quadratic; function-like property macros are not modeled.
- RustChunk relevance: Possible utility for an optional shaderpack configuration layer.
- Decision: adaptable; shaderpack properties parser; optional extension
- 1.20.1 implications: Not Java .properties nor the Minecraft resource-language parser; only Starling's shaderpack dialect.
- Follow-up checks: Cap bytes/lines/entries, check line counters, test continuation grammar and diagnose unsupported macros.
- RustChunk implementation mapping: Implement separately from launcher config and game resources.
- Evidence required before adoption: Actual 1.20.1 pack fixtures, malformed-input and duplicate-heavy stress tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2054. `crates/horizon-shaders/src/screens.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `170b94ed863ddcf507b999d4859d022dfe68e5be`
- Size: 8697 bytes; 232 logical lines
- Coverage: 1–232 (EOF)
- Purpose: Parses shaderpack settings screens, sub-screens, columns, sliders and profile references.
- Dependencies / cross-references: Properties and PackOption values/profile resolution.
- Implementation / logic: Builds nested screen items and uses a visited set for profile-cycle traversal.
- Findings: Columns permit zero/huge values; sliders are not fully type-checked; profile values are not verified against allowed option values; missing screen links and total item counts need validation/caps.
- RustChunk relevance: Could inform an optional shader settings UI, not core launcher/profile UI.
- Decision: adaptable; shader settings screen schema; optional extension
- 1.20.1 implications: Horizon settings schema is not Minecraft 1.20.1 or Forge config UI.
- Follow-up checks: Validate column and slider ranges, missing links, allowed profile values and nested item counts.
- RustChunk implementation mapping: Keep shader controls isolated from launcher accounts, game profiles and mod configs.
- Evidence required before adoption: Real-pack screen fixtures, invalid-reference/profile-cycle tests and large nested-screen stress.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2055. `crates/horizon-shaders/src/textures.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cfeefb3ccf16f33814f09269b4aec2ed71dc3c7f`
- Size: 21911 bytes; 578 logical lines
- Coverage: 1–400; 401–578 (EOF)
- Purpose: Loads shaderpack custom/noise textures and PNG/raw 2D/3D payloads, with resource naming and metadata.
- Dependencies / cross-references: Pack files, resource identifiers, PNG decoder and GPU texture descriptors.
- Implementation / logic: Parses texture declarations/metadata, resolves resources and decodes texture data to typed image descriptions.
- Findings: Decoded PNG allocation has no explicit aggregate memory cap; raw 3D dimension products may be huge; multiplications need checked arithmetic. Dynamic/atlas prefixes bypass normal path validation; mcmeta flags use brittle substring searches.
- RustChunk relevance: Potential optional resource decoder after secure archive and renderer abstractions exist.
- Decision: rewrite required; shaderpack texture and image loader; security-sensitive
- 1.20.1 implications: Minecraft 1.20.1 atlas, animation, resource locations and GPU formats differ; not a drop-in asset loader.
- Follow-up checks: Validate every path prefix, cap decoded bytes and dimension products, use checked arithmetic and real JSON parsing, reject duplicate names and incompatible formats.
- RustChunk implementation mapping: Build a bounded decode pipeline with per-image and aggregate budgets.
- Evidence required before adoption: PNG-bomb/3D overflow, path traversal, mcmeta fuzz and Android format/memory tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2056. `crates/horizon-shaders/src/translate/anonymous_structs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dcccc6434aa7608961733821ff809cb81f307169`
- Size: 1592 bytes; 44 logical lines
- Coverage: 1–44 (EOF)
- Purpose: Names anonymous GLSL structs for downstream parsing.
- Dependencies / cross-references: GLSL lexer and shader translator.
- Implementation / logic: Tokenizes source, identifies anonymous structs and inserts generated type names.
- Findings: Pattern-based rewrite rather than GLSL AST; unusual declarators, nested structures and generated-name collisions need broader tests.
- RustChunk relevance: Small compiler-front-end technique only.
- Decision: adaptable; anonymous GLSL struct naming; optional translator helper
- 1.20.1 implications: Only relevant to an optional GLSL shaderpack path.
- Follow-up checks: Test nested/array/interface-block declarations, comments/strings and collision-free names.
- RustChunk implementation mapping: Use only inside a bounded GLSL front end, preferably after typed parsing.
- Evidence required before adoption: GLSL reference fixtures and line/source preservation tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2057. `crates/horizon-shaders/src/translate/atomic_vectors.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f9b76a9f3a0f06fcce1b0fecf8fe0ae7044cb920`
- Size: 12936 bytes; 349 logical lines
- Coverage: 1–349 (EOF)
- Purpose: Lowers shared ivec/uvec values used by atomics to scalar shared arrays plus getter/setter helpers.
- Dependencies / cross-references: GLSL lexer, shared declarations, atomic built-ins and Naga translator.
- Implementation / logic: Finds atomic operands, splits vector storage into scalar components and rewrites component/vector/compound operations.
- Findings: First-argument resource detection is shallow; declaration shape and component bounds are weakly validated; generated helper names may collide; malformed missing-semicolon cases can produce bad rewrites.
- RustChunk relevance: Narrow compute-shader compatibility technique only.
- Decision: adaptable; shared-vector atomic lowering; optional GLSL compute translator
- 1.20.1 implications: Naga/backend atomic support and memory semantics need device proof; no game feature.
- Follow-up checks: Use AST resource resolution, validate vector width/indexes, generate collision-free names and diagnose malformed statements.
- RustChunk implementation mapping: Isolate in optional shader translator; never mix with chunk scheduling or protocol.
- Evidence required before adoption: GLSL oracle, expression fuzzing, atomic memory tests and Android GPU execution.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2058. `crates/horizon-shaders/src/translate/geometry.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e5e1b1ae21a50308f4f5960f588c9d3a9060f28c`
- Size: 29538 bytes; 688 logical lines
- Coverage: 1–400; 401–688 (EOF)
- Purpose: Emulates a GLSL geometry stage by combining vertex/geometry logic into a generated vertex stage and preserving the fragment stage.
- Dependencies / cross-references: Preprocessed stages, interface_blocks, GbuffersInputs, vertex-pull/index contract and main translator.
- Implementation / logic: Pulls each input triangle's vertices, runs the original vertex stage, emulates EmitVertex/EndPrimitive strips and gates memory writes to one primary invocation.
- Findings: Relies on exact unindexed triangle draws and instance semantics; geometry output arrays are explicitly unsupported; max_vertices expects a simple literal. Token-based write gating and generated u16 file-offset mapping need stronger checks.
- RustChunk relevance: Possible optional fallback if RustChunk later supports shaderpacks on a backend without native geometry shaders.
- Decision: rewrite required; geometry-stage emulation; renderer/backend-specific
- 1.20.1 implications: Not a Minecraft 1.20.1 game primitive; assumes Horizon draw topology and renderer inputs.
- Follow-up checks: Validate strip winding/parity, multiple primitives, limits, early returns, nested helper writes and origin overflow against native geometry behavior.
- RustChunk implementation mapping: Do not use in first client implementation; consider only after renderer draw contract is stable.
- Evidence required before adoption: Native OpenGL geometry oracle, Vulkan/Metal captures, draw topology tests and target GPU limits.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2059. `crates/horizon-shaders/src/translate/image_parameters.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f20871ba43958503d0c9bc72d6f0349e3278ae75`
- Size: 10743 bytes; 331 logical lines
- Coverage: 1–331 (EOF)
- Purpose: Specializes functions with opaque image parameters into concrete calls against bound global images.
- Dependencies / cross-references: GLSL lexer, image declarations, function signatures/calls and Naga translator.
- Implementation / logic: Creates function variants keyed by function and bound image names, substitutes resource identifiers and removes opaque parameters.
- Findings: Overloads selected by name/argument count only; ambiguous overloads fail. Image handles must be a single identifier; dynamic expressions unsupported. A 256-specialization cap exists, but recursion/cycle and resource-identity handling need tests.
- RustChunk relevance: Optional GLSL compute/image lowering only.
- Decision: adaptable; storage-image parameter specialization; optional translator helper
- 1.20.1 implications: No Java 1.20.1, Forge or client integration; Naga binding rules vary by backend.
- Follow-up checks: Resolve overloads by full types, verify resources are actual bound globals, detect recursion and test arrays/nested calls.
- RustChunk implementation mapping: Keep inside a separate GLSL-to-Naga bridge with typed bindings and quotas.
- Evidence required before adoption: GLSL oracle for image parameters, overload/recursion tests and storage-format backend validation.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2060. `crates/horizon-shaders/src/translate/integer_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a735cf0dde8ac0efa94480ce34f8033fc9e2dddf`
- Size: 3143 bytes; 91 logical lines
- Coverage: 1–91 (EOF)
- Purpose: Resolves a narrow GLSL clamp overload where an int value meets two unsigned literal bounds.
- Dependencies / cross-references: GLSL lexer and translator preprocessing pass.
- Implementation / logic: Builds a shallow token type map and inserts uint(value) only for the recognized exact form.
- Findings: Type discovery is lexical/global, not scope-aware; same-name declarations suppress rewriting; only one literal call shape is covered.
- RustChunk relevance: Small compatibility workaround, not a type checker.
- Decision: adaptable; targeted signed/unsigned clamp disambiguation; optional translator helper
- 1.20.1 implications: No direct gameplay semantics; exact GLSL overload behavior requires version-specific confirmation.
- Follow-up checks: Test shadowing, parameters, macros, constructors, negative values and prefer typed AST resolution.
- RustChunk implementation mapping: Never put in game logic; if needed, keep as versioned GLSL pass.
- Evidence required before adoption: Reference compiler outputs and Naga validation for accepted/rejected cases.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2061. `crates/horizon-shaders/src/translate/interface_blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f7ac121cddec2489e29c8dde005e5f76fc8be594`
- Size: 18476 bytes; 494 logical lines
- Coverage: 1–400; 401–494 (EOF)
- Purpose: Flattens GLSL in/out interface blocks into individually named varyings for Naga.
- Dependencies / cross-references: GLSL lexer, stage declarations, geometry translator and Stage scanner.
- Implementation / logic: Finds global blocks, rewrites instance.member and array accesses to generated member varyings; leaves whole-instance uses intact.
- Findings: Pattern-based block/member parser and scope inference; generated names can collide; whole-instance uses remain unflattened; cross-stage type/location/interpolation matching is deferred.
- RustChunk relevance: Useful front-end concept for an optional GLSL shaderpack compiler only.
- Decision: adaptable; GLSL interface-block flattening; optional translator helper
- 1.20.1 implications: Requires GLSL version/backend conformance; no direct Minecraft/Forge interface.
- Follow-up checks: Test nested shadowing, arrays, member layouts, name collisions and linked-stage mismatches; use typed IR.
- RustChunk implementation mapping: Parse blocks to typed interface records and verify exact location/type/interpolation.
- Evidence required before adoption: Reference GLSL compiler, linked-stage ABI corpus and adversarial scope tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2062. `crates/horizon-shaders/src/translate/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `efc0cca990a54ad55c6cc04e1692b94fca7345e7`
- Size: 132688 bytes; 3114 logical lines
- Coverage: 1–400; 401–800; 801–1200; 1201–1600; 1601–2000; 2001–2400; 2401–2800; 2801–3114 (EOF)
- Purpose: Orchestrates GLSL 110–460-to-Naga translation for vertex, fragment and compute stages.
- Dependencies / cross-references: scan, rewrite, geometry, interface_blocks, image_parameters, ordering, integer_bounds, atomic_vectors, shadowed_types, Naga and Horizon renderer structures.
- Implementation / logic: Collects interfaces/resources, generates bindings and built-ins, lowers varyings and validates Naga modules while exporting program/workgroup metadata.
- Findings: Per-stage Naga validation is not full linked-pipeline ABI validation; varying type/interpolation and sampler conflict checks are incomplete. Generated array products need caps; workgroup values use spec minima not queried device limits; numeric casts and AlphaTest parsing lack strict finite/range/grammar checks.
- RustChunk relevance: Large architecture reference for an optional shader translator, tightly bound to Horizon's renderer.
- Decision: rewrite required; GLSL-to-Naga shader translator; renderer-specific
- 1.20.1 implications: Horizon uniforms, bindings, draw model and frame contract are not a Minecraft 1.20.1/Forge client implementation.
- Follow-up checks: Add cross-stage ABI linking, exact resource reconciliation, bounded arrays/resources, real adapter workgroup checks and strict numeric grammar.
- RustChunk implementation mapping: If later needed, design separate typed IR and per-version renderer adapter; do not port monolithically.
- Evidence required before adoption: Full real-pack corpus, OpenGL/Iris oracle, Naga backend tests and Android pixel/device validation.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2063. `crates/horizon-shaders/src/translate/ordering.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2d3a3a62832dea3b324d7e172f706068d8db9943`
- Size: 17537 bytes; 460 logical lines
- Coverage: 1–400; 401–460 (EOF)
- Purpose: Detects image load/store assumptions and injects pre-dispatch snapshots or shared-memory barriers.
- Dependencies / cross-references: GLSL lexer, shared variables, atomic calls and compute translator.
- Implementation / logic: Heuristically tracks image loads/stores in functions/loops and inserts barrier after shared atomic statements.
- Findings: Not a formal control-flow/call-graph or memory-model analysis; helper recursion/complex scopes may be missed. Any early return in hz_main suppresses all barrier insertion.
- RustChunk relevance: Specialized portability layer for compute shaderpacks only.
- Decision: adaptable; compute image ordering and barrier rewrite; optional translator pass
- 1.20.1 implications: OpenGL/Vulkan/Metal ordering differs; cannot assume correctness on Android Mali or vanilla renderer.
- Follow-up checks: Use control-flow analysis, diagnose unsafe return paths and test adversarial kernels across backends.
- RustChunk implementation mapping: Separate from chunk scheduling; use explicit memory scopes if shaderpack support is pursued.
- Evidence required before adoption: OpenGL reference kernels, Vulkan/Metal validation and Android inter-invocation tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2064. `crates/horizon-shaders/src/translate/rewrite.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d99776362335fe4167ea743bc58ac8c057a98c5d`
- Size: 36421 bytes; 775 logical lines
- Coverage: 1–400; 401–775 (EOF)
- Purpose: Rewrites GLSL tokens for Naga and injects Horizon compatibility built-ins.
- Dependencies / cross-references: Stage scanner metadata, sampler/function metadata, uniforms, lexer and Naga GLSL frontend.
- Implementation / logic: Rewrites legacy texture functions, built-ins, sampler arguments, bool uniforms, matrices, array declarations and selected return/discard constructs.
- Findings: Token-level rewriting rather than full AST; depends on inferred scopes/declaration shapes, generated symbols and limited compatibility substitutions; source/token sizes uncapped.
- RustChunk relevance: Potential part of an optional shaderpack compiler, not client core.
- Decision: rewrite required; GLSL source compatibility rewrite; renderer-specific
- 1.20.1 implications: Injected matrices/uniforms encode Horizon render state, not general Minecraft 1.20.1 shader ABI.
- Follow-up checks: Move fragile rewrites to AST/typed IR; cap source/token sizes; test macro-expanded scopes, name collisions and diagnostics.
- RustChunk implementation mapping: Do not port into launcher; implement versioned shader lowering separately if needed.
- Evidence required before adoption: GLSL reference compilers 1.20/3.30/4.60, fuzzed scopes and backend output tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2065. `crates/horizon-shaders/src/translate/scan.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b10d88a11a11eeeb8ba7111cf8e1d543b44f4e34`
- Size: 33105 bytes; 870 logical lines
- Coverage: 1–400; 401–800; 801–870 (EOF)
- Purpose: Scans GLSL declarations, uniforms, images, SSBOs, attributes, varyings, structs and built-ins.
- Dependencies / cross-references: GLSL lexer, Preprocessed origins and translator lowering passes.
- Implementation / logic: Classifies global statements and layout/storage declarations, builds stage metadata and detects built-in resource usage.
- Findings: Token-shape grammar is incomplete; is_ident validates only the first char; SSBO binding/declarator parsing and alpha-cut inference are heuristic; no resource-count caps.
- RustChunk relevance: Potential shader parser reference only.
- Decision: rewrite required; GLSL stage declaration scanner; optional compiler front end
- 1.20.1 implications: Horizon GLSL contracts are distinct from Minecraft block registries and renderer interfaces.
- Follow-up checks: Use grammar/AST, validate identifiers/layouts, reject duplicate bindings, cap resources and make alpha-test semantic.
- RustChunk implementation mapping: Lower to typed stage records and perform linked-stage validation.
- Evidence required before adoption: GLSL grammar corpus, malformed declaration fuzzing and stage-link checks.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2066. `crates/horizon-shaders/src/translate/shadowed_types.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `19866acd7ec01597bdb74a1ba1b8aae6ab86533f`
- Size: 4055 bytes; 106 logical lines
- Coverage: 1–106 (EOF)
- Purpose: Renames a variable that shadows its struct type name for Naga parser compatibility.
- Dependencies / cross-references: GLSL lexer and downstream translation.
- Implementation / logic: Detects struct name redeclarations and suffixes inferred value uses with `_hzv`.
- Findings: Global token heuristic lacks robust scope modelling; same identifiers in other functions/nested scopes may be renamed incorrectly; identifier validation is weak.
- RustChunk relevance: Small parser workaround for optional shaderpack path.
- Decision: adaptable; struct-name shadowing rewrite; optional translator helper
- 1.20.1 implications: No Minecraft or Forge version contract.
- Follow-up checks: Test nested scopes, parameters/locals, multiple functions and generated-name collision cases.
- RustChunk implementation mapping: Prefer symbol resolution in a typed AST over global text substitution.
- Evidence required before adoption: Reference GLSL parser comparison and scope-fuzz tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2067. `crates/horizon-shaders/src/values.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f283b75a9435388147057638c719970335a8760b`
- Size: 16765 bytes; 410 logical lines
- Coverage: 1–400; 401–410 (EOF)
- Purpose: Stores shader option choices, switches, profile inheritance/cost and applies selections to #define/const lines.
- Dependencies / cross-references: Properties, PackOption definitions, preprocessor and shader option UI.
- Implementation / logic: Parses lists/profiles, caps numeric options and rewrites declarations with chosen values.
- Findings: Public set/parse_list accepts arbitrary raw strings and apply inserts values into source without option-domain validation; newline injection possible. Inheritance has depth cap but no visited set/breadth cap; NaN/inf caps and narrow parser are concerns.
- RustChunk relevance: Could inform an optional shader settings UI only after validation.
- Decision: rewrite required; shader option values and profiles; input-validation sensitive
- 1.20.1 implications: Separate from launcher accounts, Minecraft profiles and Forge mod config.
- Follow-up checks: Validate identifiers/values at every public boundary, reject control chars, detect graph cycles and require finite numeric caps.
- RustChunk implementation mapping: Use typed enums/ranges and immutable validated choices; never insert untrusted raw strings.
- Evidence required before adoption: Malicious choice injection, profile-graph fuzzing, nonfinite numeric tests and real-pack conformance.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2068. `crates/horizon-shaders/tests/packs/post-test/README.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c3d124eecc7f91b91c7d42030eeb78e9b3624346`
- Size: 385 bytes; 6 logical lines
- Coverage: 1–6 (EOF)
- Purpose: Documents the local Horizon post-processing test pack.
- Dependencies / cross-references: Post-test shader fixtures, properties and Horizon runtime selection.
- Implementation / logic: Explains copying into shaderpacks and activating through HORIZON_SHADER_PACK=post-test.
- Findings: Explicitly says it is written for Horizon, not another pack's code; it is not third-party compatibility evidence.
- RustChunk relevance: Test fixture documentation only.
- Decision: reference; Horizon post-processing test README; test fixture only
- 1.20.1 implications: No 1.20.1, OptiFine/Iris or Forge compatibility evidence.
- Follow-up checks: Keep docs aligned with files and tests that consume this fixture.
- RustChunk implementation mapping: No runtime port; use only as a test design reference.
- Evidence required before adoption: Third-party 1.20.1 pack corpus and rendered-image comparison.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2069. `crates/horizon-shaders/tests/packs/post-test/shaders/composite.fsh`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9261e9faeab7cc147df8e7b1c318e550cbed094a`
- Size: 653 bytes; 23 logical lines
- Coverage: 1–23 (EOF)
- Purpose: Sample composite fragment pass reads color/depth and writes tinted color plus view-space depth to MRT outputs.
- Dependencies / cross-references: colortex0, depthtex0, gbufferProjectionInverse, far and DRAWBUFFERS 02.
- Implementation / logic: Reconstructs view position from depth and emits color plus normalized distance.
- Findings: Horizon-only; projection/depth/far and DRAWBUFFERS contracts are renderer-specific.
- RustChunk relevance: Illustrates MRT post-processing only.
- Decision: reference; Horizon composite fragment fixture; test-only
- 1.20.1 implications: Legacy GLSL 1.20 syntax; output targets and uniforms not 1.20.1 contracts.
- Follow-up checks: Compare reconstruction near/far values and validate attachment mapping.
- RustChunk implementation mapping: Use as concept only; define independent RustChunk shader ABI.
- Evidence required before adoption: Reference depth captures, MRT checks and pixel comparisons.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2070. `crates/horizon-shaders/tests/packs/post-test/shaders/composite.vsh`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6dbee8d1ff41c92d889d9c434666211c798158fe`
- Size: 147 bytes; 8 logical lines
- Coverage: 1–8 (EOF)
- Purpose: Sample fullscreen vertex stage for the first post-processing pass.
- Dependencies / cross-references: ftransform, gl_TextureMatrix[0], gl_MultiTexCoord0 and texcoord varying.
- Implementation / logic: Transforms quad vertices and forwards transformed UVs.
- Findings: Depends on compatibility built-ins and Horizon quad/matrix state; draw geometry and winding are not specified here.
- RustChunk relevance: Fullscreen pass example only.
- Decision: reference; Horizon fullscreen vertex fixture; test-only
- 1.20.1 implications: Not directly portable; target renderer must provide these built-ins and UVs.
- Follow-up checks: Test UV orientation/origin, viewport scaling and matrix values.
- RustChunk implementation mapping: Use explicit fullscreen geometry/bindings after renderer conventions are fixed.
- Evidence required before adoption: Target backend capture for UV mapping.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2071. `crates/horizon-shaders/tests/packs/post-test/shaders/composite1.fsh`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5207ad758aedae2504174a7e1682b6ebadc2d58f`
- Size: 744 bytes; 25 logical lines
- Coverage: 1–25 (EOF)
- Purpose: Sample second composite fragment pass renders depth fog bands on half-screen and adds noise.
- Dependencies / cross-references: colortex0, colortex2, noisetex, viewWidth/viewHeight and RENDERTARGETS 0.
- Implementation / logic: Samples color/depth/noise, blends fog on right half and writes output location zero.
- Findings: Horizon-specific uniforms; fixed 256 noise scale may behave differently by resolution; GLSL 330 compatibility.
- RustChunk relevance: Second post-process/diagnostic sample only.
- Decision: reference; Horizon second composite fixture; test-only
- 1.20.1 implications: No direct mapping to Minecraft post-process chain.
- Follow-up checks: Test noise tiling/resolution, UV origin and target binding.
- RustChunk implementation mapping: Keep as renderer fixture concept, not code port.
- Evidence required before adoption: Multi-resolution captures and texture dependency validation.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2072. `crates/horizon-shaders/tests/packs/post-test/shaders/composite1.vsh`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `12ef6da1f00017575e58f06cd1b816b87dd46dae`
- Size: 133 bytes; 8 logical lines
- Coverage: 1–8 (EOF)
- Purpose: Sample fullscreen vertex stage for composite1.
- Dependencies / cross-references: ftransform, gl_MultiTexCoord0 and texcoord varying.
- Implementation / logic: Writes fullscreen position and passes untransformed UV.
- Findings: Different UV path than composite.vsh without explaining why; runtime conventions decide correctness.
- RustChunk relevance: Stage wiring fixture only.
- Decision: reference; Horizon composite1 vertex fixture; test-only
- 1.20.1 implications: Renderer-specific; no 1.20.1 evidence.
- Follow-up checks: Confirm UV-space convention and orientation with captures.
- RustChunk implementation mapping: Use only for stage-interface tests.
- Evidence required before adoption: A/B UV captures on target renderer.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2073. `crates/horizon-shaders/tests/packs/post-test/shaders/final.fsh`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5efbf50df86febf53f8acf99e8031f91252da0dc`
- Size: 276 bytes; 12 logical lines
- Coverage: 1–12 (EOF)
- Purpose: Sample final fragment stage applying a vignette.
- Dependencies / cross-references: colortex0, texcoord and gl_FragColor.
- Implementation / logic: Samples composed color, computes centered UV distance and multiplies by vignette.
- Findings: Does not clamp the factor; if UV domain differs, color may become negative; legacy GLSL 1.20.
- RustChunk relevance: Minimal final post-process fixture only.
- Decision: reference; Horizon final vignette fragment fixture; test-only
- 1.20.1 implications: Color-space/alpha/output contract is Horizon-specific.
- Follow-up checks: Check UV bounds, clamp policy, alpha preservation and linear/sRGB output.
- RustChunk implementation mapping: Use only as a simple shader test after color space is defined.
- Evidence required before adoption: Reference images across aspect ratios and color-space tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2074. `crates/horizon-shaders/tests/packs/post-test/shaders/final.vsh`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6dbee8d1ff41c92d889d9c434666211c798158fe`
- Size: 147 bytes; 8 logical lines
- Coverage: 1–8 (EOF)
- Purpose: Sample fullscreen vertex stage for final vignette.
- Dependencies / cross-references: ftransform, gl_TextureMatrix[0], gl_MultiTexCoord0 and texcoord.
- Implementation / logic: Writes fullscreen position and matrix-transformed UVs.
- Findings: Relies on Horizon matrix state; confirm identity/intended transform for this pass.
- RustChunk relevance: Fullscreen fixture pattern only.
- Decision: reference; Horizon final vertex fixture; test-only
- 1.20.1 implications: No portable Minecraft/Forge shader contract.
- Follow-up checks: Verify final-pass texture matrix and UV orientation.
- RustChunk implementation mapping: No direct code reuse; concept only.
- Evidence required before adoption: Pass-by-pass UV/matrix capture.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

### 2075. `crates/horizon-shaders/tests/packs/post-test/shaders/shaders.properties`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `510ada3a9311b21ddbc1699906dfb62b0ef185b3`
- Size: 78 bytes; 2 logical lines
- Coverage: 1–2 (EOF)
- Purpose: Declares the composite1 program property and its NO_SPLIT switch condition.
- Dependencies / cross-references: Properties parser, Values and post program discovery.
- Implementation / logic: Contains one program enable rule: program.composite1.enabled = !NO_SPLIT.
- Findings: Only a two-line fixture; does not test malformed directives, nested includes or profile resolution.
- RustChunk relevance: Shader pass toggle example only.
- Decision: reference; Horizon shader pass property fixture; test-only
- 1.20.1 implications: Horizon post-chain semantics, not Minecraft config.
- Follow-up checks: Add enabled/disabled, missing option and malformed-expression cases.
- RustChunk implementation mapping: Represent future pass enablement as typed boolean expression, separate from game settings.
- Evidence required before adoption: Real-pack fixtures and conditional evaluator tests.
- Completion state: COMPLETE — full source content read serially through EOF; coverage is contiguous with no gaps; exact source SHA and byte size match the pinned recursive tree; embedded tests/examples inspected only, not executed.

## Checkpoint 2051–2075 — complete (25/25)

- Entries 2051–2075 are the next 25 blobs after excluding Index entries 1–2050 from the complete pinned tree.
- All 25 files were reviewed one at a time through EOF; long files were fetched in contiguous, gap-free chunks of no more than 400 logical lines. Exact path, SHA and byte size match the pinned tree.
- Each new Ledger entry contains the 15 required fields exactly once. The active Ledger now contains **75 entries (2001–2075)**.
- Checkpoint 2051–2075 is closed. Continue directly to **2076–2100** under the user's advance authorization; at 2100 close the 50-file section and stop. Entries 2101+ remain unauthorized.
- No tests or examples were executed; no application source was changed and no runtime/build/device compatibility validation was performed.
