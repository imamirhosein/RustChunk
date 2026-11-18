# Starling → RustChunk Audit Ledger — Entries 601–800

- Source repository: `starling-client/starling`.
- Source ref: `5432370d573642041585c69b40fb045b27133488` (pinned snapshot; do not follow the moving default branch).
- Target project: RustChunk, Minecraft Java 1.20.1-first audit.
- Scope correction: six duplicate source paths in slots 631, 632, 633, 754, 762 and 766 were replaced with six unique source files immediately following the previous endpoint at entry 1000 in the pinned Starling tree. The original copies remain documented in their earlier entries; this ledger now has 200 unique paths across 601–800.
- Scope: complete source files were read; this ledger is an architecture/portability review, not a claim that tests, builds or device runs passed.

### 601. `crates/horizon-client-core/src/uuid.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `4ede9b14134fc72ba8c5659f48239c08f595944d`
- Size: 686 bytes; 24 lines.
- Coverage: Complete file reviewed (lines 1–24).
- Purpose: Converts a four-word UUID representation to canonical dashed hexadecimal text and parses text back into four signed i32 words.
- Dependencies / cross-references: Uses only Rust formatting and integer parsing; the UUID formatting convention is consumed by protocol/account identity code elsewhere in Starling.
- Implementation / logic: Formatting casts each i32 to u32 and emits eight lowercase hex digits per word before inserting 8-4-4-4-12 separators. Parsing removes every hyphen, requires 32 remaining characters, then parses four eight-digit words.
- Findings: The parser validates hex length/content but not hyphen placement or the canonical dashed form; strings with no hyphens or misplaced hyphens may be accepted. Case is accepted by radix parsing while formatting normalizes to lowercase.
- RustChunk relevance: UUIDs are foundational for player identity, profiles, protocol, skins and saved data. The compact implementation is easy to reuse only after the parser contract is matched to Java/Minecraft 1.20.1.
- Decision: **adaptable; reference; version-specific — Highly adaptable formatting helper; tighten and differential-test parsing.**
- 1.20.1 implications: Use the exact 1.20.1 UUID/Util parsing semantics and preserve the distinction between UUID text and network UUID wire order.
- Follow-up checks: Compare dashed, undashed, uppercase, malformed separators, invalid hex, zero UUID and boundary words against a Java 1.20.1 oracle.
- RustChunk implementation mapping: Candidate home: `horizon-protocol` UUID utilities, with a single shared parser/formatter used by account, packet and storage layers.
- Evidence required before adoption: Golden input/output vectors from Java 1.20.1; protocol encode/decode round trips; malformed-input tests; no dependency on newer upstream APIs.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 602. `crates/horizon-client-core/src/who.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `63101e879550b2d4c151dd1fbd8dce4e0e54e77f`
- Size: 210 bytes; 8 lines.
- Coverage: Complete file reviewed (lines 1–8).
- Purpose: Defines a tiny identity discriminator with `Player` and `Mob(u32)` variants.
- Dependencies / cross-references: No dependencies; the Mob payload is an upstream entity runtime ID.
- Implementation / logic: A Copy/Debug/Eq/Hash enum distinguishes player-controlled identity from a mob identified by its numeric runtime id.
- Findings: The type is intentionally minimal and contains no lifetime, world, dimension or server identity. A bare runtime id can be reused after entity despawn or world changes.
- RustChunk relevance: May help classify entity ownership or per-entity state, but should not become RustChunk's globally stable entity identity.
- Decision: **adaptable; reference; version-specific — Reusable only as a local discriminator; do not use as persistent identity.**
- 1.20.1 implications: Minecraft 1.20.1 entity IDs are connection/world scoped; pair them with the current level/session or use UUIDs where persistent identity is required.
- Follow-up checks: Trace every caller and verify ID reuse, player identity, dimension transitions and disconnect cleanup before choosing an equivalent type.
- RustChunk implementation mapping: Place a RustChunk equivalent near entity runtime state, keeping network entity id and persistent UUID as different types.
- Evidence required before adoption: Caller audit plus tests for despawn/re-spawn, level replacement, disconnect and player-vs-mob dispatch.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 603. `crates/horizon-client-render/examples/compositor.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `541109c239c5797063b3243f0e835c70371feddc`
- Size: 5261 bytes; 151 lines.
- Coverage: Complete file reviewed (lines 1–151).
- Purpose: A small GPU compositor smoke example that renders two offscreen textures and presents them in a window.
- Dependencies / cross-references: Uses winit window lifecycle, wgpu surface/device/queue and Starling's compositor module.
- Implementation / logic: Creates two 320×360 BGRA offscreen targets, writes solid test colors, calls the compositor presentation path, handles resize/suspend-like zero dimensions, checks a very large resize input, and exits after a short fixed frame sequence.
- Findings: This is a controlled rendering/compositor exercise, not Minecraft scene rendering or a performance test. It assumes a desktop window/GPU and its 20-frame run does not demonstrate Android surface lifecycle or device stability.
- RustChunk relevance: Useful for isolating multi-target composition, surface resize handling and GPU presentation from the game engine; relevant to future Android renderer diagnostics.
- Decision: **adaptable; reference; optional/future — Adapt the compositor smoke-test pattern; do not treat it as Android renderer validation.**
- 1.20.1 implications: A 1.20.1 client needs the same presentation correctness but must also handle Android surface loss/recreation, lifecycle pause/resume, GLES/Vulkan device capabilities and actual game frame targets.
- Follow-up checks: Run on desktop and Android separately; test surface lost/outdated, zero and huge dimensions, orientation changes, app background/foreground and frame pacing.
- RustChunk implementation mapping: Candidate test/example under a renderer crate; keep it independent from world, chunk and protocol code.
- Evidence required before adoption: Captured frames or pixel comparisons, resize/lifecycle test logs and device-specific GPU validation; none were run in this audit.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 604. `crates/horizon-client-render/examples/pack_load_bench.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `1bca9ce16a8fd8e34607246cb8487e7b36bb5f89`
- Size: 15027 bytes; 436 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–436.
- Purpose: Builds deterministic synthetic resource-pack fixtures and times pack overlay, model parsing, sprite processing, texture atlas and cache-related work.
- Dependencies / cross-references: Exercises resource-pack ZIP generation, JSON/model fixtures and renderer resource-loading paths.
- Implementation / logic: Generates controlled packs with namespace, HD textures, models and OptiFine-style connected-texture data; runs several load/toggle/cache stages and reports medians over a small number of samples.
- Findings: Fixtures are synthetic and do not represent a complete 1.20.1 vanilla or modded pack. The custom ZIP writer uses test-oriented shortcuts (including zero CRC handling); scale is caller-controlled and large values can cause excessive allocations or exceed archive-entry count assumptions.
- RustChunk relevance: A benchmark harness can expose resource-loading regressions, but its measurements cannot be used as real-world load-time or Android memory claims.
- Decision: **adaptable; reference; version-specific — Adapt the benchmark structure; replace fixtures and validation with real 1.20.1 assets.**
- 1.20.1 implications: Use the 1.20.1 client resource-pack formats, texture dimensions, model variants, namespace overrides and actual mod packs. Android storage and RAM constraints require separate measurements.
- Follow-up checks: Compare synthetic and real packs, validate ZIP CRC/limits, bound CLI scale, measure cold/warm caches and run on the target ARM device.
- RustChunk implementation mapping: Add a standalone benchmark harness for RustChunk's resource-pack pipeline after the parser and atlas implementations stabilize.
- Evidence required before adoption: Repeatable benchmark output, real resource-pack corpus, validated archive integrity and device memory/latency traces; none were run here.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 605. `crates/horizon-client-render/src/android_controls.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `27a7179f72783855d7a45449abee5d3a2bdb43b2`
- Size: 115065 bytes; 2974 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–2974.
- Purpose: Implements the Android touchscreen control overlay: movement controls, virtual buttons, tap/crosshair interaction, gesture state, HUD drawing and conversion to game actions.
- Dependencies / cross-references: Uses the `horizon-client-input` action/key abstractions, settings/layout definitions, block/item/entity facts and renderer/HUD primitives. Several helper types are declared in adjacent Android control modules.
- Implementation / logic: Tracks touches and pointer ownership; supports short taps, long holds, camera drag, joystick and D-pad movement, double-tap crouch, sprint toggle, shield and item-dependent use/aim behavior. It generates visual button geometry and routes events to the game input layer.
- Findings: This is a large Horizon-specific mobile UX layer, not Minecraft's game-rule or protocol implementation. Some tap behaviors are selected through hard-coded item/block identities, and camera-ray helpers rely on sensible camera/aspect inputs. The source contains tests, but they were not executed.
- RustChunk relevance: Potentially valuable for RustChunk's Android usability and for solving input ergonomics, but it should remain a separately scoped mobile UI subsystem so it does not contaminate the vanilla 1.20.1 engine core.
- Decision: **adaptable; optional/future; version-specific — Useful optional Android UX reference; rebuild and validate against RustChunk's input model.**
- 1.20.1 implications: Preserve 1.20.1 interaction semantics, controls and item use rules. Re-check item identities and actions against the 1.20.1 registry rather than importing newer hard-coded tables.
- Follow-up checks: Test multiple fingers, touch cancellation, pointer handoff, inventory transitions, item changes during hold, portrait/landscape, camera FOV and real device input latency.
- RustChunk implementation mapping: Candidate crate/module `android-controls`, depending on a version-neutral input action interface and RustChunk's own 1.20.1 item/block classification.
- Evidence required before adoption: Golden gesture tests, action traces, no stuck key-up states, and hands-on Galaxy A22 tests at multiple orientations; none run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 606. `crates/horizon-client-render/src/android_controls/custom.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `b7f6c2b4674865893967346ac99f13e3444e9a53`
- Size: 25024 bytes; 633 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–633.
- Purpose: Defines customizable touchscreen buttons and the custom GoyControl/MCPE input-binding behavior.
- Dependencies / cross-references: Builds on Android touch layout settings, key names, game action tokens and keyboard/mouse input dispatch.
- Implementation / logic: Validates up to three bindings for a custom button, manages press/release and latching across touch pointers, supports button labels and custom tokens, and exposes inventory utility actions.
- Findings: GoyControl and MCPE modes are Starling/Horizon product choices, not vanilla Minecraft 1.20.1 formats. Binding ownership and release behavior are correctness-critical; duplicate or overlapping action tokens must not leave a key held.
- RustChunk relevance: Useful if RustChunk is to offer editable mobile controls, but it does not help establish core Minecraft protocol or Forge compatibility.
- Decision: **adaptable; optional/future; version-specific — Optional feature architecture; do not port as part of the core 1.20.1 engine.**
- 1.20.1 implications: The action list and supported hardware bindings must be derived from RustChunk's Android input backend and the target game's available actions.
- Follow-up checks: Test simultaneous pointers, two buttons bound to one action, cancel/lift ordering, latches, mode switches, keyboard shortcuts and saving/restoring custom bindings.
- RustChunk implementation mapping: An optional button-binding layer above the shared input action interface; keep key down/up ownership explicit and reference-counted where needed.
- Evidence required before adoption: Deterministic multi-touch event tests and Android hardware/IME validation; source tests were only read, not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 607. `crates/horizon-client-render/src/android_controls/editor.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `861113a33be451577064ff53c463de3ac625c51f`
- Size: 37286 bytes; 999 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–999.
- Purpose: Provides the touchscreen layout editor: move/resize controls, choose bindings, edit labels, undo/redo and manage saved profiles.
- Dependencies / cross-references: Consumes layout/settings definitions, custom binding logic, keyboard events and UI dialog/render helpers.
- Implementation / logic: Maintains an editable copy of the active layout, captures key input, manipulates button placement/scale, saves or cancels edits and provides profile/preset operations.
- Findings: This is UI/product functionality, not game compatibility. Character capture maps a limited ASCII/US-style set rather than arbitrary Unicode/IME input; save errors are not consistently surfaced to the user in every event path.
- RustChunk relevance: Could reduce the friction of configuring mobile controls, but is lower priority than getting RustChunk's game loop, world and renderer working.
- Decision: **adaptable; optional/future; version-specific — Optional editor reference; defer until the Android control layer exists.**
- 1.20.1 implications: Target 1.20.1 does not define this editor. RustChunk would own the UX, file schema, international keyboard support and migration.
- Follow-up checks: Test undo/redo boundaries, cancel restoration, failed saves, duplicate profiles, key capture on non-US keyboards, IME composition and rotation while editing.
- RustChunk implementation mapping: Keep in `android-controls/editor` as an optional presentation component separated from persisted settings and input dispatch.
- Evidence required before adoption: UI interaction tests and Android IME/rotation tests; no such execution was performed in this review.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 608. `crates/horizon-client-render/src/android_controls/layout.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `95b539b048f22a5c7f46d4f7d4dea506d08b3c74`
- Size: 86349 bytes; 2326 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2326.
- Purpose: Defines and persists touchscreen control layouts, control styles, placement, presets and profile migration.
- Dependencies / cross-references: Shared types for `android_controls.rs`, `custom.rs` and `editor.rs`; reads/writes Horizon touch profile files.
- Implementation / logic: Provides mode/style enums, clamped placements, layout sanitization, migration from older 10/12-control layouts to a 14-slot layout, custom-button validation, per-profile directories and separate MCPE/GoyControl presets.
- Findings: The schema and profile names are Horizon-specific. Sanitization and migration are useful patterns, but the preset list, action catalog, default placement and on-disk paths are not vanilla Minecraft contracts. User profile data must not be silently discarded on parse/save errors.
- RustChunk relevance: Potentially reusable for RustChunk's mobile control configurator, but not a prerequisite for the game engine. Porting the whole schema would create unnecessary coupling to another launcher's UX.
- Decision: **adaptable; optional/future; version-specific — Reuse validation/migration patterns only; define a RustChunk-owned settings schema.**
- 1.20.1 implications: The control dimensions/actions must map to RustChunk's Android display scale, 1.20.1 controls and device safe areas; profile compatibility with Horizon is not required.
- Follow-up checks: Fuzz malformed JSON and NaN/out-of-range positions; test migration across versions, duplicate profile names, atomic writes, corrupt files and default-profile protection.
- RustChunk implementation mapping: Optional `android-controls/settings` module with versioned serde schema and explicit migration functions.
- Evidence required before adoption: Migration fixture corpus, property-based sanitize tests and Android save/restore tests; not executed in this audit.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 609. `crates/horizon-client-render/src/assets.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `8576e9f9350e7cd52e099ed7da1bca54c0e681fa`
- Size: 15583 bytes; 396 lines.
- Coverage: Complete file reviewed (lines 1–396).
- Purpose: Loads the renderer's game data bundle and composes render resources: registry facts, tags, block/biome metadata, models, items, atlases, animation, PBR and connected textures.
- Dependencies / cross-references: Reads generated `horizon-resources.json`, prepared Minecraft data/assets, world registries and model/texture resource loaders.
- Implementation / logic: Separates stable game data (`Assets`) from reloadable/derived render data (`RenderAssets`). Validates the pinned upstream data version and required files/tags; resource-pack reloads replace render assets without rebuilding the base registry.
- Findings: The data bundle and required tag/resource contents are pinned to Starling's newer 26.3 snapshot. Registry IDs, block facts, tags, material states, model catalog and asset files cannot be used as 1.20.1 data. Some state lookups assume registry IDs are valid indexes.
- RustChunk relevance: The architectural split is highly valuable for keeping version-specific game data distinct from reloadable resource packs, and for avoiding full world-data rebuilds on every pack change.
- Decision: **adaptable; version-specific — High-value architecture; regenerate the complete data bundle from Minecraft 1.20.1.**
- 1.20.1 implications: Build the registry, tags, facts, default states, biome/resource registries and client assets against exact 1.20.1 artifacts and DataVersion. Never mix 26.3 IDs or resources with 1.20.1.
- Follow-up checks: Trace bundle generation and consumers, validate required tags and registry bounds, test pack-only reloads, server registry updates and data-version mismatch rejection.
- RustChunk implementation mapping: Split RustChunk's `game-data-1_20_1` from reloadable renderer assets, each with explicit version identity.
- Evidence required before adoption: Reproducible extraction from pinned 1.20.1 client/server artifacts, complete registry diffs, pack reload tests and asset integrity checks; not performed here.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 610. `crates/horizon-client-render/src/atlas_sources.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `4ab90033c7e921617008475980e828382b9ca303`
- Size: 17705 bytes; 497 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–497.
- Purpose: Implements resource-pack atlas sprite source decoding and generation for directory, single, filter, unstitch and paletted-permutation sources.
- Dependencies / cross-references: Uses JSON codecs, pack path validation, image/texture handling and sprite atlas inputs from the renderer.
- Implementation / logic: Resolves namespaces and source paths, preserves source order, applies filters/removals, slices source textures into regions and generates palette variants before atlas packing.
- Findings: The general source pipeline is reusable, but codec details and resource paths must be checked against the 1.20.1 `SpriteSourceList`. Numeric and image-dimension inputs are not universally range/finite-checked at this layer; image buffer shape assumptions need defensive validation.
- RustChunk relevance: A resource-pack atlas pipeline is essential for faithful textures and mod packs, including resource-pack overrides. This is one of the more reusable renderer subsystems once version codecs are corrected.
- Decision: **adaptable; version-specific — Strong reusable algorithmic reference; revalidate codecs and harden untrusted pack inputs.**
- 1.20.1 implications: Use 1.20.1 atlas JSON and sprite source semantics; do not inherit later palette or namespace behavior without a version oracle.
- Follow-up checks: Differential-test each source type, ordering, duplicates, malformed paths, huge dimensions, non-finite coordinates, palette mismatch and pack priority.
- RustChunk implementation mapping: Implement in RustChunk's renderer asset pipeline behind a versioned 1.20.1 sprite-source codec.
- Evidence required before adoption: Java 1.20.1 atlas output comparison and hostile resource-pack tests; source-level tests were not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 611. `crates/horizon-client-render/src/audio.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `8c4b0ed8fc401932a4b30a3d614be44abccc246b`
- Size: 75037 bytes; 1916 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–1916.
- Purpose: Implements an audio engine with decoded sample caching, asynchronous decode, channel/source management, spatial attenuation/panning, device handling and event playback.
- Dependencies / cross-references: Uses Symphonia/Vorbis decoding, cpal output, mixer state, sound event data, game listener/world state and platform-specific audio behavior.
- Implementation / logic: Decodes and caches samples, schedules playback on mixer channels, handles pitch/gain/loop/positional sources, computes distance and stereo behavior, detects device changes and supports disabled/virtual output modes. It also includes duplicate-sound suppression and split-screen listener handling.
- Findings: This is a substantial replacement for the usual OpenAL route, not a small renderer helper. Audio parity depends on 1.20.1 sound categories, events, random selection, attenuation, streaming/music, subtitles, listener updates, sample conversion and device behavior. Code comments state specific OpenAL comparisons, but no test or audio output was executed here.
- RustChunk relevance: Can inform a future native audio backend and performance strategy, especially if RustChunk targets Android. It should not precede a working game loop and a validated baseline audio path.
- Decision: **adaptable; optional/future — Architectural reference only; defer a full audio-engine replacement until core gameplay works.**
- 1.20.1 implications: Rebuild the sound event catalog and exact 1.20.1 sound semantics; Android routing, latency, pause/resume and hardware differences need target-device validation.
- Follow-up checks: Compare waveforms/levels with Java 1.20.1, test multi-channel loads, streaming, loops, device unplug/replug, sample-rate conversion, suspend/resume and Android routes.
- RustChunk implementation mapping: Separate `audio-decode`, `audio-mixer` and platform output backend; keep game sound events and listener integration versioned.
- Evidence required before adoption: Measured output/latency, reference audio comparisons, stress/memory tests and real Galaxy A22 testing; none run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 612. `crates/horizon-client-render/src/auth.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `1752be1b9f3544fd98791352cb027aae254d789c`
- Size: 35613 bytes; 911 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–911.
- Purpose: Handles authenticated account sessions, session-file handoff, Minecraft Services discovery, server join/verify, account status and secure chat profile key acquisition.
- Dependencies / cross-references: Uses serde JSON, ureq HTTP, `horizon_protocol` UUID/RSA/secure-chat types, ban notice parsing and skin/profile services.
- Implementation / logic: Loads short-lived private-permission session files and deletes them after reading, stores bearer tokens in memory, discovers service endpoints, joins/verifies servers, retries temporary unavailable joins once and fetches/rotates signed chat keys.
- Findings: Security hygiene is strong in intent: tokens are hidden from Debug and not written/logged, and session files are age/permission checked and deleted. `parse_instant` is handwritten and checks broad month/day ranges but not real calendar dates (e.g. February 31); malformed dates may be accepted. UUID parsing is permissive about hyphen placement. Provider routing includes Ely.by, which is not part of vanilla account behavior.
- RustChunk relevance: Online-mode server support requires correct auth/session behavior, but it is sensitive and independent from offline/local gameplay. Keep it isolated from rendering and avoid persisting secrets.
- Decision: **rewrite required; version-specific — Security-sensitive reference; reimplement against 1.20.1 services and harden date/UUID parsing.**
- 1.20.1 implications: Use the exact 1.20.1 authlib/session join, hasJoined, profile key and secure-chat rules. Ely.by and newer service fields are optional provider extensions, not vanilla requirements.
- Follow-up checks: Differential-test Java `Instant`, service discovery, 401/403/5xx mapping, token expiry, file permissions, deletion on every exit, key rotation and signed-chat handshake.
- RustChunk implementation mapping: Separate account/session service under protocol/auth; session files should be one-time, permission-restricted handoff only. Never log access tokens or private keys.
- Evidence required before adoption: Mocked HTTP fixtures, security review, valid/invalid certificate tests and live online-mode joining with a legitimate session; none run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 613. `crates/horizon-client-render/src/ban_notice.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `f1f1aacdc0bec940af312a8399735396bbf41256`
- Size: 14083 bytes; 373 lines.
- Coverage: Complete file reviewed (lines 1–373).
- Purpose: Parses Microsoft account privilege/ban responses and builds the initial title-screen ban notices for multiplayer, forced name change and banned skin.
- Dependencies / cross-references: Uses the `auth.rs` service calls and `crate::text::Text` translation/component output.
- Implementation / logic: Maps moderation reason IDs, parses temporary/permanent expiration, determines whether multiplayer is allowed, and creates notices in vanilla title-screen order.
- Findings: The UI text keys and moderation reason table must be present in the exact 1.20.1 language resources. Missing privilege fields default to `false` in parsed responses (fail closed), while a failed fetch falls back to offline defaults; this distinction should be intentional and tested.
- RustChunk relevance: A faithful online account experience may need these restrictions, but the module is downstream of authentication and should not block offline-first engine work.
- Decision: **adaptable; version-specific — Reusable parsing/UI pattern; verify exact 1.20.1 moderation schema and localization keys.**
- 1.20.1 implications: Keep account restriction responses distinct from local/offline profile state; version the JSON schema and use target translation assets.
- Follow-up checks: Test missing/null/unknown moderation fields, expiry formats (epoch and RFC3339), unknown reason codes, localization and expired/negative durations.
- RustChunk implementation mapping: A small account-status service and UI notice view, depending on the 1.20.1 text engine rather than renderer-specific networking.
- Evidence required before adoption: Captured service fixtures, Java authlib behavior comparison and localization golden tests; not executed.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 614. `crates/horizon-client-render/src/bidi.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `c9c6caac7a468dff3de2bf8365990048d43f96be`
- Size: 12074 bytes; 302 lines.
- Coverage: Complete file reviewed (lines 1–302).
- Purpose: Reorders bidirectional text for rendering and applies Arabic contextual shaping and Unicode mirrored glyphs.
- Dependencies / cross-references: Uses `unicode-bidi` plus the text span/style structures and language direction setting.
- Implementation / logic: Classifies joining characters, maps Arabic letters to presentation forms, creates lam-alef ligatures, applies Unicode bidi reordering/mirroring and carries per-character style across reordered spans.
- Findings: The joining/form tables are handwritten compatibility data and intentionally leave some extended Arabic-script letters unchanged. Style mapping after ligature contraction and Unicode-version differences need a robust golden corpus. The file was read but tests were not run.
- RustChunk relevance: Important for correctly displaying Arabic/Persian chat, signs, names and menus; display correctness is user-facing even when networking/world code works.
- Decision: **adaptable; version-specific — Valuable RTL reference; validate with a full Arabic/Persian Unicode corpus and 1.20.1 behavior.**
- 1.20.1 implications: Target 1.20.1 font assets, bidi rules and ICU/Java shaping behavior must be matched. Do not assume all current Rust Unicode crate versions reproduce the JDK's exact Unicode data.
- Follow-up checks: Test Persian letters, Arabic letters/marks, lam-alef, mixed Latin/numbers, punctuation mirroring, formatting spans, emoji, control characters and truncated lines.
- RustChunk implementation mapping: Keep a version-neutral text layout stage in the UI/render text crate, separated from protocol chat component parsing.
- Evidence required before adoption: Golden screenshots and expected visual-order strings from Java 1.20.1 for Arabic, Persian and mixed-direction messages; not executed.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 615. `crates/horizon-client-render/src/biome.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `a04f3471cdd485d9cafc71b97409e0db4d4e64f3`
- Size: 27273 bytes; 714 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–714.
- Purpose: Loads biome definitions, spawn settings, colours, features and datapack replacements; computes temperature, grass tint and spatially blended biome colours.
- Dependencies / cross-references: Uses worldgen biome JSON, prepared grass/foliage/dry-foliage colormaps, legacy random/simplex noise and world block positions.
- Implementation / logic: Builds builtin biome tables, overlays datapack additions/replacements while preserving builtin IDs, produces sorted registry order, samples colour maps, applies grass modifiers and computes a 6×6×6 Gaussian interpolation around biome quart cells.
- Findings: The data tables and IDs are from the pinned newer Starling version and cannot be reused for 1.20.1. `parse_entries` casts u64 values to u32 without range checks, `parse_costs` accepts defaulted numeric values without finite/range validation, and `get` assumes the biome table is non-empty.
- RustChunk relevance: Biome data directly affects chunk colour, temperature, precipitation, mobs and world generation. The replacement/registry architecture is more valuable than the 26.3 catalog itself.
- Decision: **adaptable; version-specific — High-value architecture and algorithms; rebuild 1.20.1 biome registry/data and harden parsing.**
- 1.20.1 implications: Use the 1.20.1 biome codecs, attributes, spawn categories, features, colormaps and registry ordering; do not carry newer attributes or biome IDs into the target.
- Follow-up checks: Compare biome IDs/order/temperature/precipitation/colours against Java 1.20.1, test datapack replacement, malformed weights, huge counts, empty inputs and spatial boundaries.
- RustChunk implementation mapping: Place versioned biome definitions in worldgen data and keep colour interpolation/tint functions as separate tested algorithms.
- Evidence required before adoption: Complete 1.20.1 biome extraction, Java parity fixtures and malformed-datapack tests; not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 616. `crates/horizon-client-render/src/block_edit.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `dbb561d41052db4506c04132fb06b0bad930b494`
- Size: 11593 bytes; 306 lines.
- Coverage: Complete file reviewed (lines 1–306).
- Purpose: Performs bulk vertical block runs directly in resident chunk sections, unpacking/packing each changed section once and queuing chunk/lighting/mesh invalidation.
- Dependencies / cross-references: Uses chunk index/section layout, world block storage, fast hash maps and tracked-state callbacks.
- Implementation / logic: Groups runs by chunk in first-seen order, applies them sequentially with optional old-state filters, memoizes filter/tracked predicates, writes changed sections once and marks neighboring sections/light regions dirty. Returns changed count, chunks and tracked block transitions.
- Findings: The module explicitly does not produce ordinary per-block writes, block entity notifications, path updates, watched writes or light edits; callers must handle these separately. Runs outside loaded chunks/column bounds are skipped. Extreme i32 run bounds require overflow-safe loop handling.
- RustChunk relevance: Potential optimization for fill/replace operations and bulk world edits. It is not a substitute for normal gameplay `set_block` semantics and would be dangerous if used without the documented side-effect boundary.
- Decision: **adaptable; optional/future; version-specific — Adapt the batched storage algorithm only behind a separate, explicitly strict bulk-edit API.**
- 1.20.1 implications: Match 1.20.1 chunk section/heightmap/light behavior and server/client update semantics. Any block entity, fluid, POI, neighbor or entity-path side effects must be explicitly added by the caller where required.
- Follow-up checks: Differential-test overlap order, negative coordinates, tall dimensions, y-boundaries, tracked states, heightmaps, lighting, block entities and extreme integer inputs.
- RustChunk implementation mapping: Separate `World::edit_runs_strict` from normal block mutation; return a complete change summary and require caller-declared side-effect handling.
- Evidence required before adoption: Java 1.20.1 fill/replace parity, chunk serialization tests, height/light correctness and stress benchmarks; source tests not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 617. `crates/horizon-client-render/src/block_entities.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `eb418a9ec7515b8750374f9aaf9ee0e805ebe961`
- Size: 29456 bytes; 737 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–737.
- Purpose: Stores block entities, their attachments and indexes, and maintains the ordered live/pending block-entity ticker list.
- Dependencies / cross-references: Uses block entity variants, fast hash maps/sets, world attachments and block-state ticker classifications.
- Implementation / logic: Models pending ticker promotion, rebind on replacement, cancellation/removal, chunk-load order, `take`/`put` during ticks, mod entity indexes and structure/test bounding-box lists. `chunk_map_order` emulates fastutil open-addressing iteration order.
- Findings: Correctness depends on observable Java fastutil iteration order and exact ticker lifecycle. The implementation uses its own token/slot bookkeeping and mod-specific attachments/containers; a state transition to non-ticking expects the caller's block-entity removal path. Runtime IDs and registries are target-version-specific.
- RustChunk relevance: Highly relevant to server/client world fidelity: block entities and ticker ordering affect chests, furnaces, signs, containers, animations and modded blocks.
- Decision: **adaptable; version-specific — Strong lifecycle reference; port only after reproducing 1.20.1 load/tick/update ordering.**
- 1.20.1 implications: Build the ticking table from 1.20.1 block registry/state definitions and verify `LevelChunk`/`Level` lifecycle against target sources; mod attachment APIs are optional future scope.
- Follow-up checks: Differential-test chunk load ordering, insert/replace/remove/take/put/retain, state changes, slots freed during iteration, chunk unload/reload and attachment persistence.
- RustChunk implementation mapping: Core world-state `BlockEntityStore` plus separate ticker scheduler; keep renderer indexes as derived caches, not the authoritative map.
- Evidence required before adoption: Java 1.20.1 ticker-order fixtures and save/load/update parity; no runtime tests run here.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 618. `crates/horizon-client-render/src/block_entity_anim.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `acef6e51f1ed80772fbb445203fdb9c848fb1d0b`
- Size: 35311 bytes; 1027 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1027.
- Purpose: Tracks client-side block-entity animation state for chests, enchanting books, shulkers, bells, conduits, skulls, decorated pots and End gateways.
- Dependencies / cross-references: Uses block entity renderer `BlockAnim`, core trigonometry/random sources, world positions, entities and block registry state.
- Implementation / logic: Implements per-kind tick state, trigger events, interpolation values, shulker collision/push bounds, bell raid resonance, conduit activation/particles, skull animation, decorated-pot wobble and End gateway beam/cooldown state.
- Findings: The general separation between tick state and render extraction is useful. Several ticker classifications include later additions (copper chests and other newer states) and must be filtered to 1.20.1. Random source selection, event timing and side effects must match the client vs integrated server responsibilities exactly.
- RustChunk relevance: Useful for smooth block-entity visuals and accurate client ticking; animation correctness is separate from block entity storage and should consume authoritative game events/state.
- Decision: **adaptable; version-specific — Adapt per-entity state/interpolation patterns; rebuild the ticker catalog and event behavior for 1.20.1.**
- 1.20.1 implications: Use only target-version block entities and properties. Validate that the client does not simulate server-only world effects and that random streams do not desynchronize unrelated features.
- Follow-up checks: Compare tick-by-tick chests/books/shulkers/bells/conduits/skulls/gateways with Java 1.20.1, including event timing, negative coordinates, entity queries and pause/resume.
- RustChunk implementation mapping: Client render-state animation module keyed by block position; feed from a distinct 1.20.1 block entity ticker/event pipeline.
- Evidence required before adoption: Scripted deterministic input traces and Java reference animation frames; no tests or gameplay run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 619. `crates/horizon-client-render/src/block_entity_cem.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `7048c97bea6013fc03109c6c6bb32f1d0e8d3ca0`
- Size: 25735 bytes; 630 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–630.
- Purpose: Loads OptiFine/EMF custom entity model `.jem` files for block entities and reshapes baked vanilla model parts.
- Dependencies / cross-references: Uses OptiFine CEM file discovery/JSON parsing, entity model bones and `block_entity_render::ModelPart` structures.
- Implementation / logic: Maps supported CEM names to renderer model parts, applies replacement/attachment geometry, scales bones, handles shared files across banner/conduit/pot/book models and chooses custom atlas textures.
- Findings: This is an optional resource-pack compatibility layer, not a vanilla Minecraft requirement. It supports a finite hard-coded set of model names; some renderer targets and model names reflect newer blocks. Custom model loading is user-supplied input and must be hardened against oversized/deep models and invalid numeric geometry.
- RustChunk relevance: Potential future compatibility feature for OptiFine/EMF resource packs, but far below the priority of vanilla 1.20.1 model and texture rendering.
- Decision: **unnecessary; optional/future — Optional compatibility feature; defer and scope to the subset of formats actually required.**
- 1.20.1 implications: Do not let CEM determine base block-entity models. Rebuild vanilla model parts from 1.20.1 resources and layer CEM replacement only after the renderer is correct.
- Follow-up checks: Test malformed/deep `.jem`, unknown parts, attach vs replace, shared files, texture lookup, model recursion and 1.20.1 supported CEM names.
- RustChunk implementation mapping: Separate `resource-pack-compat/cem` extension that transforms already-loaded RustChunk base models.
- Evidence required before adoption: OptiFine/EMF compatibility corpus, 1.20.1 pack comparison and recursion/size safety tests; not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 620. `crates/horizon-client-render/src/block_entity_render.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `b6b8c188042308d9220a6fad3b5d6bd17af618f3`
- Size: 152654 bytes; 4035 lines.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4035.
- Purpose: Implements block-entity and associated special-model rendering: chest variants, shulkers, banners, skulls, bells, decorated pots, conduits, books, signs, End portal/gateway beams and newer copper golem/chest models.
- Dependencies / cross-references: Consumes baked entity-model data, model-part geometry, animation state, world/light queries, sprites/emissive layers, sign text, CEM shapes, break overlays and optional static GPU geometry caching.
- Implementation / logic: Classifies block states into render kinds, transforms and draws model parts, calculates diffuse/packed light, text/beam/portal geometry, applies animation, supports CEM replacement and maintains per-section CPU snapshots for static models plus a sign-text cache.
- Findings: This is a large renderer module and its model catalog, state classification, textures, block list and render behavior are pinned to the newer 26.3 snapshot. Features such as copper chests/statues are not part of 1.20.1. Static caching is sensitive to section membership, block state, light, atlas revision, pose and sprite alpha; the companion cache tests are in a separate `block_entity_render_tests.rs` file. No renderer test was executed.
- RustChunk relevance: Important reference for block-entity visuals and potential chunk/render cache design, but importing it wholesale before a 1.20.1 world/renderer baseline would bring substantial version-specific state and assets.
- Decision: **adaptable; version-specific — Use as architectural/rendering reference; rebuild the target-version model catalog and keep the cache as a separately validated optimization.**
- 1.20.1 implications: Rebuild `entity-models.json`, state classifier, block entity model list, render types, item/pottery patterns and textures from 1.20.1. Strip later entity types; ensure portal/gateway shader layers and sign/text behavior match the target.
- Follow-up checks: Read companion cache test file, compare every supported 1.20.1 block entity frame/geometry against Java output, test fractional-alpha fallback, light updates, atlas reloads, camera/frustum and cache invalidation.
- RustChunk implementation mapping: Separate block-entity model extraction, dynamic animation, static geometry cache and GPU upload path in RustChunk's renderer; keep authoritative ticking/state in world modules.
- Evidence required before adoption: Golden vertex/image tests using authentic 1.20.1 assets, cache hit/miss/invalidation tests, shader validation and actual Android GPU profiling; none performed.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

## Batch completion

- Entries added in this batch: 20 (601–620), all with 15 populated fields.
- Every path is unique within the batch and was checked against the previously audited 1–600 paths; repeated upstream paths were skipped.
- Every source SHA and line count was read from the pinned upstream snapshot and verified against the source file.
- No RustChunk implementation code was changed by this audit; no build, test, network join, APK or target-device run was performed.

# Audit continuation — Entries 621–800

- Pinned upstream source: `starling-client/starling@5432370d573642041585c69b40fb045b27133488`.
- Scope: 180 new, unique paths immediately after the entry-620 source path, skipping six paths already recorded in entries 1–620.
- This is a source/architecture audit. No RustChunk implementation code was modified and no build, test suite, APK, Forge client, server connection or Android device test was executed.

### 621. `crates/horizon-client-render/src/block_entity_render_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0c9db94005196bad40ad50324cfe9325d79fe340`
- Size: 51917 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1536.
- Purpose: Block-entity render/cache regression suite
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Exercises vertex parity, cache invalidation, resource reloads and fallback paths for special block-entity models.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; reference; version-specific — Reference tests only; do not port as runtime code.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Fixtures target the pinned 26.3 renderer and several tests require prepared assets or a GPU; none were run here.
- RustChunk implementation mapping: Fixtures target the pinned 26.3 renderer and several tests require prepared assets or a GPU; none were run here.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 622. `crates/horizon-client-render/src/bluemap.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b1afc89211f67e214f2cd2884a56a73a593e4029`
- Size: 81429 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2109.
- Purpose: Optional BlueMap remote-map integration
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Fetches and displays remote world-map tiles and metadata alongside the in-game world.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **unnecessary; optional/future — Defer as an optional network integration.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Not vanilla functionality; remote URLs, tile protocols and map metadata are provider/version-specific and create privacy/network trust concerns.
- RustChunk implementation mapping: Not vanilla functionality; remote URLs, tile protocols and map metadata are provider/version-specific and create privacy/network trust concerns.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 623. `crates/horizon-client-render/src/bluemap_detect.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `fc0390b8f7b0068db3531174e3f967733dfe6eb5`
- Size: 15990 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–452.
- Purpose: BlueMap endpoint discovery
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Detects a map endpoint from server-provided or discovered information before map tile loading.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **unnecessary; optional/future — Optional plugin; never enable implicitly.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Discovery must not turn arbitrary multiplayer server metadata into an unrestricted network fetch or SSRF route.
- RustChunk implementation mapping: Discovery must not turn arbitrary multiplayer server metadata into an unrestricted network fetch or SSRF route.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 624. `crates/horizon-client-render/src/border.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `04aaf63e0e3f53ab1c1979dbab61c7c1fba0126f`
- Size: 1203 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: World-border shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Computes the border overlay fragment colour/fade in the GPU shader.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — Adapt shader logic only after target shader compilation.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Shader IO and pipeline bindings are specific to Starling's renderer and its current WGSL toolchain.
- RustChunk implementation mapping: Shader IO and pipeline bindings are specific to Starling's renderer and its current WGSL toolchain.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 625. `crates/horizon-client-render/src/border_render.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ff3e7c8b3b611ac76c21bfc26a52de2d8ead2794`
- Size: 9875 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: World-border geometry and rendering
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Builds border wall/plane geometry and animation parameters for the world-border pass.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Adapt geometry and lifecycle; rebuild target constants.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Border warning distance, interpolation and renderer pass placement must match the 1.20.1 world-border implementation.
- RustChunk implementation mapping: Border warning distance, interpolation and renderer pass placement must match the 1.20.1 world-border implementation.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 626. `crates/horizon-client-render/src/breaking.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `34d0cafeabfb6685c2556ea2675d819cbb205cdd`
- Size: 7213 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Block-breaking overlay geometry
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Builds destroy-stage UVs and geometry for crack overlays on models/entities.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Reusable algorithm with target atlas mapping.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Destroy-stage textures, stage count and model sources must come from 1.20.1 assets.
- RustChunk implementation mapping: Destroy-stage textures, stage count and model sources must come from 1.20.1 assets.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 627. `crates/horizon-client-render/src/brewing.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4175f86b020a15d58b545c28c56ee7126c0a5b03`
- Size: 18720 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–476.
- Purpose: Brewing recipe and timing data
- Dependencies / cross-references: Depends on target-version registries, entity/block state, random source, particle atlas and client tick ordering.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Resolves brewing operations, ingredient matches and brewing time from data-driven definitions/providers.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **rewrite required; version-specific — Rebuild the data model for 1.20.1; do not copy the current catalog.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The module uses newer data-driven recipe/provider structures; vanilla 1.20.1 brewing behavior and data definitions differ.
- RustChunk implementation mapping: The module uses newer data-driven recipe/provider structures; vanilla 1.20.1 brewing behavior and data definitions differ.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 628. `crates/horizon-client-render/src/camera.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2a2087671c40ff5c6b0c242b6ea83bd479699d46`
- Size: 41306 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1137.
- Purpose: Camera and view transforms
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Maintains camera pose, projection and collision/interpolation inputs used by the frame.
- Findings: Maintains camera pose, projection, third-person clipping, view bob, hurt and interpolation state.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — High-value camera architecture; replace constants and verify math.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Camera transforms are sensitive to target perspective, view-bobbing, portal effects, FOV and collision rules.
- RustChunk implementation mapping: Camera transforms are sensitive to target perspective, view-bobbing, portal effects, FOV and collision rules.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 629. `crates/horizon-client-render/src/chat.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f770f6418c9b92f499033efe78529a5edbf57ab7`
- Size: 73045 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–1976.
- Purpose: Chat state, input and rendering
- Dependencies / cross-references: Depends on client account/session state, protocol identity, text and input handling; account providers must remain isolated.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Coordinates chat history, draft/composition, message display, completion and signing-related UI state.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: **adaptable; version-specific — Rebuild packet/security integration for 1.20.1; retain separable UI state patterns.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The source includes newer chat and signed-message fields; client packet lifecycle and secure-chat rules must match 1.20.1 exactly.
- RustChunk implementation mapping: The source includes newer chat and signed-message fields; client packet lifecycle and secure-chat rules must match 1.20.1 exactly.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 630. `crates/horizon-client-render/src/chat_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b8d63a2ed56ad67886d0c270fba575ffcb3528a0`
- Size: 31669 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–820.
- Purpose: Chat behavior regression fixtures
- Dependencies / cross-references: Depends on client account/session state, protocol identity, text and input handling; account providers must remain isolated.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Checks chat wrapping, ordering, history, keyboard interactions and related rendering behavior.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: **adaptable; reference; version-specific — Test reference only; port selected tests after target fixtures exist.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Tests are tied to current text schema and UI behavior and were not executed.
- RustChunk implementation mapping: Tests are tied to current text schema and UI behavior and were not executed.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 631. `crates/horizon-client/examples/probe/pick_block_data_fixture.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `7496ceb8e97d24f18453a4f0bfbae2bd276a5591`
- Size: 3284 bytes; 96 lines.
- Coverage: Complete source text reviewed, lines 1–97.
- Purpose: Fixture for block-entity data copied onto an item and then placed back into the world.
- Dependencies / cross-references: `block_entity_item::{pick, place}`, canonical block-entity NBT helpers, SNBT parser, Java float/double text formatting and JSON case input.
- Implementation / logic: Reads one JSON case per line, canonicalizes source/target SNBT, calls `pick` to produce an item patch and `place` to reconstruct the target block entity, then emits canonical paired output.
- Findings: Deterministic key ordering and typed numeric formatting support parity comparisons. The patch-oriented item representation is from a newer item-component era; parser failures collapse to `error`, and trailing SNBT remainder is not validated here.
- RustChunk relevance: Useful as a differential-test shape for pick-block and block-entity placement; the fixture format itself is not a game protocol or production path.
- Decision: **adaptable; reference; version-specific — Use as a test-harness reference only; rebuild the item/block-entity representation for Java 1.20.1 instead of adopting newer component-patch semantics.**
- 1.20.1 implications: Java 1.20.1 predates the 1.20.5 item data-component system. Verify `BlockEntityTag`, item NBT, `keepPacked` and pick/place rules against exact 1.20.1 behavior.
- Follow-up checks: Generate Java 1.20.1 oracle cases for signs, banners, skulls, containers and command blocks; test malformed/trailing SNBT and canonical round trips.
- RustChunk implementation mapping: Candidate RustChunk test fixture under `tests/parity/pick_block_data`; keep serialization adapters versioned and out of the renderer.
- Evidence required before adoption: Pinned 1.20.1 Java oracle output, edge-case fixtures and proof that no post-1.20.1 item-component fields leak into the target schema.
- Completion state: FULL TEXT REVIEW COMPLETE — full source text read and documented; no test, build or runtime execution is claimed.


### 632. `crates/horizon-client/examples/probe/place_recipe_fixture.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `798182cff12403148678deb4560ba6fe7f336793`
- Size: 3938 bytes; 98 lines.
- Coverage: Complete source text reviewed, lines 1–99.
- Purpose: Headless fixture for recipe-book placement into crafting menus and player inventory.
- Dependencies / cross-references: `Recipes`, `Items`, `Menu`, `Env`, `PostPlace`, brewing/banner/station/enchantment registries, pack overlay and JSON case/order files.
- Implementation / logic: Loads resources, constructs a menu for each input case, loads inventory/grid state, invokes `Menu::place_recipe` and prints post-placement menu/inventory/drop state.
- Findings: The harness captures ghost-recipe versus no-op outcomes and item movement. `cases.lines().zip(orders.lines())` silently truncates if the two inputs have different line counts; `PackOverlay::NONE` excludes enabled pack overlays.
- RustChunk relevance: Useful for deterministic recipe-placement parity and regression fixtures, not a standalone gameplay implementation.
- Decision: **adaptable; reference; version-specific — Adapt the parity-harness architecture only; recreate recipe catalogues, menu rules and item formats for the exact 1.20.1 target.**
- 1.20.1 implications: Recipe placement exists in 1.20.1, but item stack serialization, recipe registries, menu IDs and packet behavior must match 1.20.1 rather than the pinned newer snapshot.
- Follow-up checks: Add unequal-input-length rejection, test all 1.20.1 crafting and machine menus, verify survival/creative and full/partial inventory cases, then compare to Java 1.20.1.
- RustChunk implementation mapping: Candidate RustChunk test fixture under `tests/parity/recipe_placement`; call the eventual versioned menu core without binding it to renderer code.
- Evidence required before adoption: Java 1.20.1 oracle cases and exact output comparisons for slot ordering, remaining items, drops and ghost recipe behavior.
- Completion state: FULL TEXT REVIEW COMPLETE — full source text read and documented; no test, build or runtime execution is claimed.


### 633. `crates/horizon-client/examples/probe/player_sync_fixture.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `034aa64009f5bcbc29023f2ae421aa4184e87e20`
- Size: 2844 bytes; 72 lines.
- Coverage: Complete source text reviewed, lines 1–73.
- Purpose: Deterministic fixture for client movement packet emission and server teleport-velocity handling.
- Dependencies / cross-references: `PositionSender`, `teleport_velocity`, `Vec3`, `PlayerPosition`, serverbound play packets, JSON and binary packet encoder.
- Implementation / logic: Reads movement operations or relative teleport cases, logs the emitted packet variant and encoded payload, or prints the resulting velocity as exact floating-point bit patterns.
- Findings: Separates movement-state decisions from packet bytes and preserves float precision. Packet variants, relative flags and encoded fields are protocol-version-specific; the harness strips the first encoded byte as packet ID.
- RustChunk relevance: High-value networking parity fixture for movement and synchronization; keep protocol mapping separate from generic movement math.
- Decision: **adaptable; reference; version-specific — Strong candidate for RustChunk movement tests, but regenerate packet schema and byte expectations for Java 1.20.1 before adoption.**
- 1.20.1 implications: Movement and relative teleport semantics are present in 1.20.1, but packet IDs, field order, flags and packet classes must be sourced from the 1.20.1 protocol.
- Follow-up checks: Build 1.20.1 fixtures for position-only, rotation-only, combined movement, on-ground changes, teleport relative flags, extreme coordinates and rounding boundaries.
- RustChunk implementation mapping: Candidate RustChunk modules: `protocol::play::serverbound` and `client::player_sync`; split pure movement-state logic from the versioned encoder.
- Evidence required before adoption: Exact 1.20.1 protocol mapping and byte-for-byte comparison with a Java client/server oracle; no network test was executed in this review.
- Completion state: FULL TEXT REVIEW COMPLETE — full source text read and documented; no test, build or server connection is claimed.


### 634. `crates/horizon-client-render/src/cit.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `229f786f8078e688ff0e34c9288e4e76c03d4e5b`
- Size: 31851 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–849.
- Purpose: Loads OptiFine/CIT Resewn `.properties` rules from the active resource-pack overlay and evaluates item-stack/component tests to select custom textures, models and enchantment glints.
- Dependencies / cross-references: Uses `horizon_optifine::cit` rule types, item-stack/component and scoreboard/text helpers, `cit_glint` and `cit_assets`, and the renderer's resource-pack overlay.
- Implementation / logic: `load_rules`/`read_properties` scan CIT property files; `CitRules` prepares component predicates, selects matching rules by item/hand/damage context, caches repeated queries, and resolves glint behavior.
- Findings: This is optional OptiFine-compatible resource-pack behavior, not vanilla rendering. Rule semantics depend on the separate `horizon_optifine` parser and 26.3-era item-component model; the cache is bounded, but pack input and cache invalidation still need adversarial tests.
- RustChunk relevance: Could provide a later compatibility layer for OptiFine-style custom item textures after vanilla 1.20.1 item models and resource packs are stable; it is not required for the first playable milestone.
- Decision: **unnecessary; optional/future; version-specific — Optional CIT rule engine; defer until vanilla item models and resource packs are stable.**
- 1.20.1 implications: Rebuild the property grammar, item/NBT predicates, enchantment matching and stack data model against Java 1.20.1; do not assume later component codecs or CIT Resewn extensions match.
- Follow-up checks: Differential-test rule priority, item IDs, damage, enchantments, lore/name text, NBT predicates, pack priority, cache invalidation and glint composition against a Java 1.20.1 client with known CIT packs.
- RustChunk implementation mapping: Optional `assets/cit/rules` module, isolated behind a compatibility feature and resource-pack reload generation.
- Evidence required before adoption: Golden results from real 1.20.1 CIT packs; parser rejection cases; item-stack selector parity; atlas/model resolution tests and rendered screenshots.
- Completion state: Source blob reviewed and SHA/size verified; no CIT test suite, renderer test, build or Android device run was executed.

### 635. `crates/horizon-client-render/src/cit_armor.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4fd382248f00f162b0ffececb35cc247e097ffdd`
- Size: 12334 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Maps CIT armor and elytra rule textures onto the renderer's worn armor layers and wings, including fallback to vanilla layer sprites.
- Dependencies / cross-references: Uses `cit_assets` resource IDs, `horizon_optifine::cit::{Kind, Rule}`, armor/elytra model layers and the atlas sprite lookup.
- Implementation / logic: Translates armor-layer names to CIT property texture keys, selects per-layer or generic rule textures, resolves atlas sprites, and exposes worn-layer/wings helpers plus collected armor sprites.
- Findings: The source explicitly maps 26.3 equipment asset names back to pre-1.21.2 texture names. That compatibility mapping is not itself the Java 1.20.1 armor resource schema; this remains an optional CIT feature.
- RustChunk relevance: A possible later OptiFine-style armor/elytra texture compatibility module; not needed for vanilla armor rendering.
- Decision: **unnecessary; optional/future; version-specific — Optional armor/elytra CIT mapping; defer until vanilla armor rendering is correct.**
- 1.20.1 implications: Use the exact 1.20.1 armor layer names, leather overlays, elytra texture paths and item/armor model selection; do not port 26.3 equipment path assumptions.
- Follow-up checks: Test every 1.20.1 armor material and layer, leather tint/overlay, elytra, missing texture fallback, resource-pack priority and reload behavior against vanilla and CIT packs.
- RustChunk implementation mapping: Optional `assets/cit/armor` adapter layered on the base armor model/atlas path.
- Evidence required before adoption: 1.20.1 armor/elytra golden texture IDs, rendered screenshots, pack reload tests and per-layer CIT selector fixtures.
- Completion state: Source blob reviewed and SHA/size verified; tests were not executed.

### 636. `crates/horizon-client-render/src/cit_assets.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d2bbf9e96b7130e3c7bdb3103fd25b3ded6296d2`
- Size: 13448 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Converts resource paths referenced by CIT rules into renderer sprite/model identifiers and loads CIT custom models into the model library.
- Dependencies / cross-references: Uses `horizon_assets::{CuboidModel, TextureSlot}`, `horizon_optifine::cit::{Kind, Rule}`, filesystem resource roots and atlas/model loading.
- Implementation / logic: Reserves a `horizon_cit` namespace for CIT files, converts texture/model resource paths, collects needed sprites, validates and rebases relative references, loads model files and gathers their texture dependencies; parent traversal is capped.
- Findings: This is a resource-pack compatibility adapter for files vanilla does not normally enumerate. Its reserved namespace and path rebasing must be reviewed as security boundaries; the implementation is Horizon/CIT-specific, not a general vanilla asset index.
- RustChunk relevance: Potential later asset bridge if RustChunk elects to support CIT packs; the base resource pack namespace/model loader should remain independent.
- Decision: **unnecessary; optional/future; version-specific — Optional CIT asset namespace/model loader; defer and harden before use.**
- 1.20.1 implications: Use the 1.20.1 resource-pack overlay, namespace validation, model parent rules and texture identifiers; verify the target loader's path semantics rather than importing 26.3 assumptions.
- Follow-up checks: Test path traversal and root confinement, encoded/invalid identifiers, duplicate IDs, relative textures, cyclic/deep parents, missing models, overlay priority and atlas rebuild after reload.
- RustChunk implementation mapping: Optional `assets/cit/resources` adapter, with a strict validated virtual filesystem boundary.
- Evidence required before adoption: Adversarial path tests, 1.20.1 model/texture fixtures, duplicate/cycle tests and successful rendered CIT examples.
- Completion state: Source blob reviewed and SHA/size verified; tests were not executed.

### 637. `crates/horizon-client-render/src/cit_glint.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ad47afc014e170f0679fcbe335071a4fe51c70a4`
- Size: 15801 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: CIT enchantment glint textures
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Resource-pack discovery/parsing/selection and rendering-resource composition.
- Findings: Builds shared texture arrays and per-layer blend parameters for custom glint effects.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: **unnecessary; optional/future; version-specific — Optional pack feature; keep separate from vanilla enchantment glint.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Glint material and item component sources are newer; texture arrays may exceed low-end device limits.
- RustChunk implementation mapping: Glint material and item component sources are newer; texture arrays may exceed low-end device limits.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 638. `crates/horizon-client-render/src/clouds.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `434dd5e5d680a5f8ff1407f40a5025a7327d5ba7`
- Size: 15734 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–515.
- Purpose: Cloud mesh and animation cache
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Resource-pack discovery/parsing/selection and rendering-resource composition.
- Findings: Builds cloud geometry and updates position/phase via uniforms while remaining inside a geometry cell.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: **adaptable; version-specific — Reusable performance pattern; preserve exact cloud appearance.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Cloud height/type, weather state and atlas inputs must match 1.20.1; this is visual-only, not world geometry.
- RustChunk implementation mapping: Cloud height/type, weather state and atlas inputs must match 1.20.1; this is visual-only, not world geometry.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 639. `crates/horizon-client-render/src/collisions.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `72e7ae8285131122bbf755ca6bdfcc1887a858a3`
- Size: 39136 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–998.
- Purpose: Collision query acceleration
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Resource-pack discovery/parsing/selection and rendering-resource composition.
- Findings: Uses chunk-section palettes and compact shape data to avoid scanning blocks that cannot collide.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: **adaptable; version-specific — High-value algorithm; adapt shape registry and movement semantics.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Collision shapes, fluids, entity movement and block-state IDs are versioned; acceleration must remain conservative.
- RustChunk implementation mapping: Collision shapes, fluids, entity movement and block-state IDs are versioned; acceleration must remain conservative.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 640. `crates/horizon-client-render/src/command_block.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1f62d20f6b90497267009b2564e3ab83917ffd86`
- Size: 9612 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Command-block editor/view state
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Resource-pack discovery/parsing/selection and rendering-resource composition.
- Findings: Provides command-block text/metadata and UI-facing state used by command-block screens.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: **adaptable; optional/future; version-specific — UI feature may be adapted after core screen/input works.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Command block permissions, packet handling and NBT field names must follow 1.20.1.
- RustChunk implementation mapping: Command block permissions, packet handling and NBT field names must follow 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 641. `crates/horizon-client-render/src/connected.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `66293c63f8d813aea08c8d8001af805acbef9694`
- Size: 49344 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1284.
- Purpose: Connected-texture rule evaluation
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Implements OptiFine-style connected textures, predicates and neighbour-sensitive sprite selection.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **unnecessary; optional/future — Optional resource-pack compatibility; defer from vanilla baseline.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: CTM is not a vanilla renderer feature; rule syntax and neighbour semantics depend on the chosen compatibility target.
- RustChunk implementation mapping: CTM is not a vanilla renderer feature; rule syntax and neighbour semantics depend on the chosen compatibility target.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 642. `crates/horizon-client-render/src/containers.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0bac7f8a3f5451d4fdf70a239bef5f7d98a5b4c3`
- Size: 30892 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–763.
- Purpose: Container screen data and rendering
- Dependencies / cross-references: Depends on authoritative chunk state, block facts/shape occlusion, fluids, dimensions and world update lifecycle.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Builds container content, slot layout and specialized chest/furnace-like screen states.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Reuse generic slot/UI patterns; rebuild the container catalog.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The module contains later blocks/container variants and item component assumptions absent from 1.20.1.
- RustChunk implementation mapping: The module contains later blocks/container variants and item component assumptions absent from 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 643. `crates/horizon-client-render/src/crash.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1c88198c54d92165bbdfa59cdf57ba312ead4dd1`
- Size: 34381 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–913.
- Purpose: Crash diagnostics and redaction
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Collects crash context and scrubs sensitive/player/server details before presenting or saving reports.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; reference — Adapt diagnostics with a strict privacy allowlist.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The current report fields and paths are Starling-specific; sensitive auth tokens, addresses and personal data must never leak.
- RustChunk implementation mapping: The current report fields and paths are Starling-specific; sensitive auth tokens, addresses and personal data must never leak.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 644. `crates/horizon-client-render/src/datapack.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9a972098fde62f7ecf57862974a45796121d5c6d`
- Size: 45805 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1202.
- Purpose: Datapack loading and lifecycle
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Resource-pack discovery/parsing/selection and rendering-resource composition.
- Findings: Reads datapack metadata/resources and manages reloadable data inputs.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: **adaptable; version-specific — High-value architecture; rebuild codecs and resources from 1.20.1.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pack formats, registry data, functions and resource IDs have changed since 1.20.1.
- RustChunk implementation mapping: Pack formats, registry data, functions and resource IDs have changed since 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 645. `crates/horizon-client-render/src/dynamic_light.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `464f259a5e1c618a6381560fc38246115d90cd1d`
- Size: 7562 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Dynamic-light visual overlay
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Adds nearby light-source values to shader-side visual shading without changing authoritative world light.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **unnecessary; optional/future — Optional cosmetic feature; keep separate from gameplay light engine.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Explicitly visual-only; it does not update chunk light storage or server/gameplay calculations.
- RustChunk implementation mapping: Explicitly visual-only; it does not update chunk light storage or server/gameplay calculations.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 646. `crates/horizon-client-render/src/effects.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a97243b3491f7ff07f0a6ed7ba896d07b4ebe381`
- Size: 24997 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–680.
- Purpose: Status effects and displayed effects model
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Stores active effects, durations, modifiers and display-facing state.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Rebuild effect and attribute registry for 1.20.1.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Effect IDs, attributes, particles and newer effects are not interchangeable with 1.20.1.
- RustChunk implementation mapping: Effect IDs, attributes, particles and newer effects are not interchangeable with 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 647. `crates/horizon-client-render/src/elyby.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `febf2f5fb7a6b1d1a7a838493cd1d4038917913e`
- Size: 11334 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Ely.by authentication provider
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Resource-pack discovery/parsing/selection and rendering-resource composition.
- Findings: Routes account/session behavior through an additional third-party identity provider.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: **unnecessary; optional/future — Optional provider integration; isolate from vanilla auth.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Provider-specific response formats and service endpoints are not part of Minecraft 1.20.1 vanilla auth.
- RustChunk implementation mapping: Provider-specific response formats and service endpoints are not part of Minecraft 1.20.1 vanilla auth.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 648. `crates/horizon-client-render/src/entity.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `11f48b6ebfc2b59eaa160dcdf13a64587fc230fd`
- Size: 29761 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–589.
- Purpose: Entity fragment and vertex shader
- Dependencies / cross-references: Depends on authoritative chunk state, block facts/shape occlusion, fluids, dimensions and world update lifecycle.
- Implementation / logic: Block/sky light propagation and incremental relighting over section-neighbour regions.
- Findings: Implements entity texture, lighting, overlay, fog and special-layer rendering.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: **adaptable; version-specific — Reuse shader concepts only after target pipeline parity.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Vertex formats, overlay codes, material layers and shader bindings must be rebuilt from 1.20.1 renderer behavior.
- RustChunk implementation mapping: Vertex formats, overlay codes, material layers and shader bindings must be rebuilt from 1.20.1 renderer behavior.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 649. `crates/horizon-client-render/src/environment.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1bd19eff39ee9cb27d0bd20d78801433d8ae02ca`
- Size: 16016 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–430.
- Purpose: Environment attributes and dimension visuals
- Dependencies / cross-references: Depends on target-version registries, entity/block state, random source, particle atlas and client tick ordering.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Resolves data-driven environmental attributes such as fog, sky, ambience and related visual/gameplay values.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Rebuild attribute catalog and evaluation rules from 1.20.1.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Current definitions contain later dimension/environment attributes; do not import 26.3 registry data.
- RustChunk implementation mapping: Current definitions contain later dimension/environment attributes; do not import 26.3 registry data.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 650. `crates/horizon-client-render/src/extras.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d04ba4ab941566fadbf0531d658622b02832b0c8`
- Size: 41404 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–961.
- Purpose: Optional Horizon settings registry
- Dependencies / cross-references: Depends on client account/session state, protocol identity, text and input handling; account providers must remain isolated.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Keeps extra renderer/mobile settings outside vanilla options.txt.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **unnecessary; optional/future — Reuse separation principle; define a RustChunk-owned schema.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Settings such as minimap, split-screen, cosmetics and agent controls are product-specific, not vanilla options.
- RustChunk implementation mapping: Settings such as minimap, split-screen, cosmetics and agent controls are product-specific, not vanilla options.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 651. `crates/horizon-client-render/src/fast_hash.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a9062a352dbfb76d84f9ccb25936e4fa22002bf0`
- Size: 190 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Fast hash alias
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Centralizes a fast non-cryptographic hasher choice for internal maps/sets.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; reference — Use Rust's standard hasher unless profiling justifies this.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Hashing is an implementation detail; never use this hash for security, persistence or protocol identity.
- RustChunk implementation mapping: Hashing is an implementation detail; never use this hash for security, persistence or protocol identity.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 652. `crates/horizon-client-render/src/fog.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ab7982e0f3157083b6d8b605100826a64198cc8d`
- Size: 25015 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–692.
- Purpose: Fog equations and state
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Combines fog type, distances, colours, water/lava effects and render-time parameters.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Reimplement using 1.20.1 fog semantics and current RustChunk render distances.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Fog attributes and dimension data are newer; far-terrain handling must not hide near chunk holes.
- RustChunk implementation mapping: Fog attributes and dimension data are newer; far-terrain handling must not hide near chunk holes.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 653. `crates/horizon-client-render/src/font.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8447e9ddb7e465da1ee056fb3d72764e568cdb91`
- Size: 58693 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1535.
- Purpose: Font resource providers and glyph atlas
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Loads bitmap, Unihex and TTF font providers, glyph metrics and atlas data.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — High-value font architecture; validate exact 1.20.1 provider codecs.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Font providers and Unicode/font resources are versioned; Unicode shaping and glyph advances affect UI parity.
- RustChunk implementation mapping: Font providers and Unicode/font resources are versioned; Unicode shaping and glyph advances affect UI parity.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 654. `crates/horizon-client-render/src/fusion.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `51df969e4b4d346a14ff0386c4b5ac60c91d09be`
- Size: 26743 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–687.
- Purpose: Fusion connected-texture extension
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Parses a Fusion-specific model/texture rule and applies connected model conditions.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **unnecessary; optional/future — Optional compatibility extension; do not include in vanilla core.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Fusion is third-party functionality, not a vanilla format.
- RustChunk implementation mapping: Fusion is third-party functionality, not a vanilla format.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 655. `crates/horizon-client-render/src/gfx.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ec161ae28eb3be62d7765d92c41a1021d52581e2`
- Size: 497899 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4800, 4801–5200, 5201–5600, 5601–6000, 6001–6400, 6401–6800, 6801–7200, 7201–7600, 7601–8000, 8001–8400, 8401–8800, 8801–9200, 9201–9600, 9601–10000, 10001–10400, 10401–10800, 10801–11200, 11201–11416.
- Purpose: Main renderer and GPU frame orchestration
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Owns surface/device lifecycle, render passes, terrain/entities/HUD, GPU buffers, shaders, caches and presentation.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Architectural reference only; do not transplant as one monolithic renderer.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Contains extensive 26.3 and Horizon behavior plus API-specific branches; target mobile GPU support remains unproven.
- RustChunk implementation mapping: Contains extensive 26.3 and Horizon behavior plus API-specific branches; target mobile GPU support remains unproven.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 656. `crates/horizon-client-render/src/gfx/adreno.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ee6943dc8059046afaa9de2863eba54fc0767d19`
- Size: 3837 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Adreno-specific workarounds
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Selects driver-specific GPU workarounds for Qualcomm Adreno variants.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: **adaptable; reference; optional/future — Do not apply to Mali; retain only device-fingerprint-gated workarounds with evidence.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The user target is Mali-G52 MC2, so Adreno heuristics are not evidence for it.
- RustChunk implementation mapping: The user target is Mali-G52 MC2, so Adreno heuristics are not evidence for it.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 657. `crates/horizon-client-render/src/gfx/animate.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `324e72a124874da54f65f8f697938a74096ea7a8`
- Size: 37767 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–952.
- Purpose: Animated sprite frame selection
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Builds frame chains and animation state for atlas sprites, including mip levels.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Adapt core animation/mipmap logic after 1.20.1 metadata parity.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Animation metadata and GPU compute path may differ; compute support cannot be assumed on GLES devices.
- RustChunk implementation mapping: Animation metadata and GPU compute path may differ; compute support cannot be assumed on GLES devices.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 658. `crates/horizon-client-render/src/gfx/animate.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8998a1aa1c69ad4ad428969f9b330ce2e9d17676`
- Size: 2023 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Animated sprite compute shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Runs the GPU part of sprite-frame update and texture copy work.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: **adaptable; version-specific — Optional compute path; retain a CPU fallback.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: WGSL bindings/workgroup assumptions may not be supported or performant on GLES/Mali.
- RustChunk implementation mapping: WGSL bindings/workgroup assumptions may not be supported or performant on GLES/Mali.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 659. `crates/horizon-client-render/src/gfx/arena.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0579e8bb821e07fd7e02c976d4f8371118cf2486`
- Size: 68464 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1580.
- Purpose: GPU vertex/index arena allocator
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Allocates paged GPU memory, free lists, compaction and spill/reuse for meshes.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — High-value allocator design; impose Android memory caps.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Large page sizes and spill strategies are designed for desktop-class heaps and may exceed mobile budgets.
- RustChunk implementation mapping: Large page sizes and spill strategies are designed for desktop-class heaps and may exceed mobile budgets.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 660. `crates/horizon-client-render/src/gfx/block_light.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `cac568d3c973725e37a65d5788d32843a3d842e8`
- Size: 8384 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader block-light tint grid
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Provides material/shader-pack access to block light values for custom material shading.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional shader-pack hook, not the main lighting engine.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: This is a shader extension; authoritative lighting remains separate.
- RustChunk implementation mapping: This is a shader extension; authoritative lighting remains separate.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 661. `crates/horizon-client-render/src/gfx/compositor.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e3b929d48f08001b292ab048f0e42ce1eef403ac`
- Size: 8218 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Split-view compositor
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Composes multiple offscreen views into one surface.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; reference; optional/future — Optional split-screen feature; defer.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Desktop winit/wgpu example does not establish Android multi-view lifecycle or input mapping.
- RustChunk implementation mapping: Desktop winit/wgpu example does not establish Android multi-view lifecycle or input mapping.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 662. `crates/horizon-client-render/src/gfx/crumbling_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3ab32e4ba2a22485d058b83b74dff52e883d930c`
- Size: 7525 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GPU crumbling/render pipeline tests
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Checks destroy overlay and GPU rendering regressions.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; reference; version-specific — Test source only; no test executed.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: GPU tests are adapter-dependent and may skip on machines without capabilities.
- RustChunk implementation mapping: GPU tests are adapter-dependent and may skip on machines without capabilities.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 663. `crates/horizon-client-render/src/gfx/cull.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `41f10d4747c78d94e65fbdc9c3b61354b2c79a6f`
- Size: 26075 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–677.
- Purpose: Asynchronous GPU occlusion culling
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Reads occlusion results asynchronously and uses prior-frame visibility to skip hidden sections.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — Optional optimization; enable only when adapter supports the exact path and profiling proves benefit.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Implementation supports selected Vulkan/Metal paths; generic GLES/Mali path may not be supported or beneficial.
- RustChunk implementation mapping: Implementation supports selected Vulkan/Metal paths; generic GLES/Mali path may not be supported or beneficial.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 664. `crates/horizon-client-render/src/gfx/cull.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b972a00ae1c5d75a729cf629ef816112e9a0c319`
- Size: 4763 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GPU occlusion compute shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Evaluates occlusion/query results for section visibility.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — Port only with a supported backend and proven conservative result.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Compute atomics/readback and depth conventions are backend-sensitive.
- RustChunk implementation mapping: Compute atomics/readback and depth conventions are backend-sensitive.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 665. `crates/horizon-client-render/src/gfx/end_sky.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d74e053df3cd380a2676a747e3bd4866aa225c4c`
- Size: 2996 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: End sky resident geometry
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Builds and retains End sky cube geometry and atlas bindings.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — Adapt geometry residency; use 1.20.1 sky resources.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Face orientation, textures and shader bindings must follow 1.20.1 resources.
- RustChunk implementation mapping: Face orientation, textures and shader bindings must follow 1.20.1 resources.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 666. `crates/horizon-client-render/src/gfx/end_sky/tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a31b64e752dbe6deedb9ffb000f3b35bf7a72579`
- Size: 13305 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: End sky GPU parity tests
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Compares resident sky geometry and rendered pixels against expected behavior.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — Reference tests only; not executed.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Tests depend on GPU and pinned newer asset details.
- RustChunk implementation mapping: Tests depend on GPU and pinned newer asset details.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 667. `crates/horizon-client-render/src/gfx/entity_batch.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c490a81d7200788ab1b440e4aacb2e96e9c3c9d1`
- Size: 16779 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Entity material batching
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Groups entity draw runs by material/overlay to reduce pipeline switches while preserving semantics.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — High-value batching pattern; keep stable order for translucent cases.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Entity material taxonomy and render pass layering are version-specific.
- RustChunk implementation mapping: Entity material taxonomy and render pass layering are version-specific.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 668. `crates/horizon-client-render/src/gfx/entity_outline.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `72a8d4838ba981c17ee73d392889bf3ed851dc36`
- Size: 13277 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Entity outline renderer
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Builds glowing entity outline masks and compositing.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — Optional visual feature; verify 1.20.1 glow behavior.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Resolution, depth and threshold behavior are tied to current render graph.
- RustChunk implementation mapping: Resolution, depth and threshold behavior are tied to current render graph.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 669. `crates/horizon-client-render/src/gfx/entity_outline.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `bd723f11eed7a844cceb6a1939929b1c6f61d9ff`
- Size: 2427 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Entity outline shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Computes mask/edge colour for glowing entity outlines.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — Shader algorithm reference only.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Must match RustChunk's shader IO, depth conventions and GLES limits.
- RustChunk implementation mapping: Must match RustChunk's shader IO, depth conventions and GLES limits.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 670. `crates/horizon-client-render/src/gfx/entity_skin.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ff9d30009875f9aa680e50488c083d320cc6a53a`
- Size: 26443 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–690.
- Purpose: GPU entity skinning
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Transforms entity model vertices on GPU and retains a CPU expansion fallback.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; optional/future; version-specific — Potential optimization; default to CPU until mobile parity/benefit demonstrated.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Requires compute/storage feature support and exact vertex layout compatibility.
- RustChunk implementation mapping: Requires compute/storage feature support and exact vertex layout compatibility.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 671. `crates/horizon-client-render/src/gfx/entity_skin.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3fc7b99468840b0508604cb622af2e4663e4b66e`
- Size: 5886 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GPU entity skinning shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Computes skinned entity vertex positions and normals.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; optional/future; version-specific — Optional compute backend; retain CPU path.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Compute limits and floating-point precision may vary across drivers.
- RustChunk implementation mapping: Compute limits and floating-point precision may vary across drivers.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 672. `crates/horizon-client-render/src/gfx/entity_source.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `68aa25584c454e59d266ea039813f2977d8bdb25`
- Size: 2095 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Entity GLSL/GLES source workarounds
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Lowers or rewrites generated shader source for problematic older Adreno GLES drivers.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; reference; optional/future — Apply only by exact driver fingerprint.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Workaround is aimed at legacy Adreno behavior, not a generic GLES or Mali fix.
- RustChunk implementation mapping: Workaround is aimed at legacy Adreno behavior, not a generic GLES or Mali fix.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 673. `crates/horizon-client-render/src/gfx/flat.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d5e83cfeccb4718e38de794f0cc0b0aadcfff03c`
- Size: 51693 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1329.
- Purpose: Flat/compute terrain draw path
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Flattens or batches section draw instances to reduce per-section draw overhead.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; optional/future; version-specific — Optional optimization; gate by proven adapter/driver benchmarks.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Path relies on compute/storage/indirect draw capabilities and driver heuristics; not safe to assume Mali gains.
- RustChunk implementation mapping: Path relies on compute/storage/indirect draw capabilities and driver heuristics; not safe to assume Mali gains.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 674. `crates/horizon-client-render/src/gfx/flatten.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d722d7b9fc7bb4967047260b1a8d78c63003962f`
- Size: 2605 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Flatten compute shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Generates flattened draw indices from draw ranges and page tables.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; optional/future; version-specific — Shader helper only; pair with a verified CPU fallback.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Atomics/workgroup scheduling and output bounds require backend validation.
- RustChunk implementation mapping: Atomics/workgroup scheduling and output bounds require backend validation.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 675. `crates/horizon-client-render/src/gfx/frame_gen.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e205068e7a3f3985bb3dc777fc9e30110dc3dbe0`
- Size: 21506 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–572.
- Purpose: Frame-generation controller
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Coordinates offscreen real frames, interpolation, presentation thread and camera/hand motion.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future; version-specific — Defer; not a core Minecraft capability and risky for mobile baseline.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: FSR 3.1 implementation is a Horizon optional feature with substantial latency/memory demands.
- RustChunk implementation mapping: FSR 3.1 implementation is a Horizon optional feature with substantial latency/memory demands.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 676. `crates/horizon-client-render/src/gfx/frame_gen/interpolation.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a143c1a3a75ab296624a1d89e7636f98a6691a70`
- Size: 33038 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–862.
- Purpose: Frame interpolation pipeline
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Interpolates consecutive images using depth, motion vectors and optical flow.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future; version-specific — Research reference only; do not port wholesale.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Requires compute-heavy texture pyramids and depends on depth/motion correctness; could blur chunk edges/UI.
- RustChunk implementation mapping: Requires compute-heavy texture pyramids and depends on depth/motion correctness; could blur chunk edges/UI.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 677. `crates/horizon-client-render/src/gfx/frame_gen/interpolation.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6802c41d817e26c5612bf5d4356d50d093c01998`
- Size: 31453 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–810.
- Purpose: Frame interpolation WGSL
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Implements compute passes for depth dilation, vector fields, blending and inpainting.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future; version-specific — Optional shader reference; defer.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Shader feature/precision/workgroup limits are backend-dependent.
- RustChunk implementation mapping: Shader feature/precision/workgroup limits are backend-dependent.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 678. `crates/horizon-client-render/src/gfx/frame_gen/motion.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1abd92cf340fc375a24f47ae872610b65cef8e51`
- Size: 11980 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Object motion-vector generation
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Tracks independently moving entities by ID and rasterizes motion vectors.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future; version-specific — Optional frame-generation support; not needed for a vanilla client.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Entity identity, mesh changes and atlas alpha handling need 1.20.1 mapping.
- RustChunk implementation mapping: Entity identity, mesh changes and atlas alpha handling need 1.20.1 mapping.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 679. `crates/horizon-client-render/src/gfx/frame_gen/motion.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ef647ba1aa3bcc8aa574fc4f8af59a5ad2977a05`
- Size: 2294 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Motion-vector shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Renders per-entity motion vectors while respecting depth and transparent texels.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future; version-specific — Optional shader; defer with frame generation.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Uses a frame-generation-specific coordinate and depth convention.
- RustChunk implementation mapping: Uses a frame-generation-specific coordinate and depth convention.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 680. `crates/horizon-client-render/src/gfx/frame_gen/optical_flow.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d0b04a52ab5558f90749921b9e8fc2ec1efde489`
- Size: 24626 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–653.
- Purpose: Optical-flow estimation
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Implements multilevel luma pyramids, block matching and scene-change detection.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future; version-specific — Optional frame-generation research; not core client rendering.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Large compute and storage workload; no evidence of mobile viability.
- RustChunk implementation mapping: Large compute and storage workload; no evidence of mobile viability.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 681. `crates/horizon-client-render/src/gfx/frame_gen/optical_flow.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ab52714b3426f17c165dcf06d512e548c63d9b7d`
- Size: 20939 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–537.
- Purpose: Optical-flow shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Computes luma, histograms, multilevel searches and block vectors.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future; version-specific — Optional GPU shader.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Atomic/workgroup constraints and image formats vary by backend.
- RustChunk implementation mapping: Atomic/workgroup constraints and image formats vary by backend.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 682. `crates/horizon-client-render/src/gfx/hdr.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8da6e176ad8309d0d8c91e38e3ad6f6fdf27d426`
- Size: 26061 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–703.
- Purpose: HDR output pipeline
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Selects HDR10/scRGB surfaces, controls paper white/peak and maps world/HUD to HDR output.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional desktop/display feature; not a priority for A22.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Android HDR surface formats, metadata and display capability handling differ from this implementation.
- RustChunk implementation mapping: Android HDR surface formats, metadata and display capability handling differ from this implementation.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 683. `crates/horizon-client-render/src/gfx/hdr_icc.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `edbb8cca2d1e999be561d189c15ac1df281dccb4`
- Size: 5662 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HDR screenshot ICC profile
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Constructs a BT.2020/PQ profile for HDR screenshot interoperability.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional screenshot compatibility code.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Colour profile and PNG cICP behavior are not gameplay or 1.20.1 concerns.
- RustChunk implementation mapping: Colour profile and PNG cICP behavior are not gameplay or 1.20.1 concerns.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 684. `crates/horizon-client-render/src/gfx/hdr_stats.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `61c3aee02a0453cae2b33c66f55ee3704a489bfb`
- Size: 8090 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HDR emission statistics
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Measures sprite brightness on GPU for adaptive HDR highlight selection.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; reference; optional/future — Optional HDR support only.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Depends on HDR output and GPU measurement capabilities.
- RustChunk implementation mapping: Depends on HDR output and GPU measurement capabilities.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 685. `crates/horizon-client-render/src/gfx/inter_stage_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1250a18477c107303de4237f0c10f0ba06cc57b0`
- Size: 7898 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Inter-stage shader interface tests
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Audits WGSL vertex/fragment interface variables against a downlevel GPU limit.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; reference; version-specific — High-value build-time validation; adapt for RustChunk shaders.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Limit value may depend on wgpu/backend version; current test references 15 variables and older hardware.
- RustChunk implementation mapping: Limit value may depend on wgpu/backend version; current test references 15 variables and older hardware.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 686. `crates/horizon-client-render/src/gfx/late_sky.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `88b7408a9bc0c5bac621ef3c19f0e63edd95d20d`
- Size: 2849 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Late-sky pass
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Draws sky after solid terrain using far-plane depth to avoid shading covered pixels.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; optional/future; version-specific — Potential render optimization; target GPU tests required.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pass order and shader-pack/portal interactions must match 1.20.1.
- RustChunk implementation mapping: Pass order and shader-pack/portal interactions must match 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 687. `crates/horizon-client-render/src/gfx/late_sky/tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `72e1884d9a0dc50388a8a6f14c7ebdcd8186886b`
- Size: 21960 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–540.
- Purpose: Late-sky GPU parity suite
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Checks late-sky shader output pixel-for-pixel against the ordinary draw order.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; reference; version-specific — Reference tests only; no test run.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Requires a working GPU adapter and test setup.
- RustChunk implementation mapping: Requires a working GPU adapter and test setup.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 688. `crates/horizon-client-render/src/gfx/lightning.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a5adab0464df24333d5cac749d78634c66e987f0`
- Size: 8054 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Lightning mesh cache
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Retains deterministic lightning bolt geometry by seed and reuses it while active.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; optional/future — Reusable CPU/GPU cache pattern.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Random seed, vertex order, texture atlas UV and lightning lifetime need target 1.20.1 parity.
- RustChunk implementation mapping: Random seed, vertex order, texture atlas UV and lightning lifetime need target 1.20.1 parity.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 689. `crates/horizon-client-render/src/gfx/lightning/tests/mod.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `79e0f51b7f573fc92bd30245dcd1add584a8e88c`
- Size: 29654 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–749.
- Purpose: Lightning geometry/GPU regression tests
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Compares retained bolt geometry, atlas sampling and GPU draw output with a pinned reference.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; reference; optional/future — Reference tests only; not executed.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Tests reference 26.3 LightningBoltRenderer and optional GPU adapter.
- RustChunk implementation mapping: Tests reference 26.3 LightningBoltRenderer and optional GPU adapter.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 690. `crates/horizon-client-render/src/gfx/lod.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a843134c53bb69d997cc0eeb69333c5c2f16240d`
- Size: 21759 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–557.
- Purpose: Far-terrain GPU tile visibility and upload
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Uploads/fades distant tile meshes and handles replacement by nearer tiles.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; optional/future; version-specific — High-value architecture for RustChunk LOD, but isolate from real world simulation.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The LOD tile grid, fade rules and source data are 26.3-specific; far tiles are deliberately not authoritative chunks.
- RustChunk implementation mapping: The LOD tile grid, fade rules and source data are 26.3-specific; far tiles are deliberately not authoritative chunks.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 691. `crates/horizon-client-render/src/gfx/native_material.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8f8527bc0ee1ab0d1a2476ca6a582339f29eb924`
- Size: 24867 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–549.
- Purpose: Native shader-pack material stage
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Injects a custom material shader stage into terrain shading.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional Horizon shader-pack extension; defer from vanilla baseline.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pack API, bindings and material identifiers are not vanilla 1.20.1 behavior.
- RustChunk implementation mapping: Pack API, bindings and material identifiers are not vanilla 1.20.1 behavior.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 692. `crates/horizon-client-render/src/gfx/native_pack.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3cada2233c2fece54b97cc32fafd98a19c2d0134`
- Size: 6270 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Native shader-pack runtime object
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Holds custom stages, settings and timers for Horizon's own shader-pack system.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional extension only.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: This is product-specific pack API, not resource-pack parity required by vanilla.
- RustChunk implementation mapping: This is product-specific pack API, not resource-pack parity required by vanilla.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 693. `crates/horizon-client-render/src/gfx/native_post.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3db3222eae4a8da9b58815aae8cc08f83faa0d5f`
- Size: 61730 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1548.
- Purpose: Native shader-pack post processing
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Runs arbitrary multi-pass pack post effects with custom targets, bindings and history.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional extension; high complexity and untrusted shader input.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Resource limits, Naga translation, dynamic targets and formats are tied to current wgpu stack.
- RustChunk implementation mapping: Resource limits, Naga translation, dynamic targets and formats are tied to current wgpu stack.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 694. `crates/horizon-client-render/src/gfx/native_render.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9d4ac6c2c2d052b76b811867fc0ed4b270d3e491`
- Size: 7454 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Native renderer hook dispatch
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Provides hook points for native mods to draw at chosen frame phases.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional mod API; defer until stable render lifecycle.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Hook ABI, event order and mod lifecycle are custom to Horizon.
- RustChunk implementation mapping: Hook ABI, event order and mod lifecycle are custom to Horizon.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 695. `crates/horizon-client-render/src/gfx/native_render/generated.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `53828c258158936fe21a5451d9f93a57818cfc81`
- Size: 2006 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Generated render hook interface
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Defines event structs and ordered dispatch for native renderer hooks.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional API support, not vanilla core.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Generated API can change and depends on native mod loader contracts.
- RustChunk implementation mapping: Generated API can change and depends on native mod loader contracts.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 696. `crates/horizon-client-render/src/gfx/native_shadow.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2ef451ba73d4f557e4aed2de0b650b9bd16e4e4c`
- Size: 28921 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–696.
- Purpose: Native shader-pack shadow map
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Renders depth/colour shadow maps for custom shader packs.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional extension, not required for vanilla shadows.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Shadow camera and texture conventions are Iris/Horizon-like shader API details.
- RustChunk implementation mapping: Shadow camera and texture conventions are Iris/Horizon-like shader API details.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 697. `crates/horizon-client-render/src/gfx/native_sky.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `550775727f901e034e620700ccb19026323f265f`
- Size: 15486 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Native shader-pack sky stage
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Allows custom sky shader stage over renderer globals.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional custom shader feature.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Shader pack interfaces and stage ordering are custom.
- RustChunk implementation mapping: Shader pack interfaces and stage ordering are custom.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 698. `crates/horizon-client-render/src/gfx/occlusion.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `847344f978d01893d3ab67edf89e1131c07787f7`
- Size: 54657 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1408.
- Purpose: Section occlusion graph
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Searches visible section connectivity from camera via open faces on a background worker, preserving last completed graph for frames.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **adaptable; version-specific — High-value algorithm; validate against 1.20.1 VisGraph/SectionOcclusionGraph.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Connectivity rules, 60-block distant stepping and partial updates are target-version-sensitive; false culling can hide chunks.
- RustChunk implementation mapping: Connectivity rules, 60-block distant stepping and partial updates are target-version-sensitive; false culling can hide chunks.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 699. `crates/horizon-client-render/src/gfx/oit.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `46ef35854df65522858022efbe996b614d4e07a0`
- Size: 37272 bytes (upstream Git blob size).
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–980.
- Purpose: Order-independent transparency renderer
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Uses multi-stage depth/transmittance accumulation and final composite for translucent surfaces.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional renderer path; keep vanilla sorted transparency as fallback.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Requires special float blend/attachment features and may be unavailable or slow on Mali/GLES.
- RustChunk implementation mapping: Requires special float blend/attachment features and may be unavailable or slow on Mali/GLES.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 700. `crates/horizon-client-render/src/gfx/oit_features.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `dc783579571f29f88d3927551f9f2b5623cc8250`
- Size: 9856 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: OIT feature pipelines
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Applies OIT stages to particles, weather, border, text and other non-terrain draw categories.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: **unnecessary; optional/future — Optional OIT component.**
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pass ordering and material support are tied to Starling's renderer; no OIT device run occurred.
- RustChunk implementation mapping: Pass ordering and material support are tied to Starling's renderer; no OIT device run occurred.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 701. `crates/horizon-client-render/src/gfx/pack_atlas.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6ca92726264358506ad4ea3e37ba8c4d01f0d46e`
- Size: 16657 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack entity draw batches
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Groups entity and particle spans by pack program and falls back to Horizon draw for unclaimed spans.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional shader pack layer.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Entity classifications/program stages are version-specific; can affect visible output if spans overlap.
- RustChunk implementation mapping: Entity classifications/program stages are version-specific; can affect visible output if spans overlap.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 702. `crates/horizon-client-render/src/gfx/pack_center_depth.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `30435dbadbc99537c4729b852bd805b93651f8ae`
- Size: 11412 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack first-person hand pass
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Builds hand projection and geometry binding for shader-pack hand programs.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional shader pack compatibility.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Hand depth scaling/projection conventions are Iris-like and custom.
- RustChunk implementation mapping: Hand depth scaling/projection conventions are Iris-like and custom.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 703. `crates/horizon-client-render/src/gfx/pack_draw.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2b280cc90b2033b6384025f8dba79788cfb12a2f`
- Size: 5813 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack custom images and buffers
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Creates, sizes, clears, snapshots and reads back pack-owned image/storage-buffer resources.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional extension with untrusted resource allocation risk.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Formats, max image count, buffer binding and shader resource schema are custom.
- RustChunk implementation mapping: Formats, max image count, buffer binding and shader resource schema are custom.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 704. `crates/horizon-client-render/src/gfx/pack_entities.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1e93ac546e13a14df60ba3fbfc2bed497bdebda0`
- Size: 23784 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack post chain
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Implements shader-pack prepare/deferred/composite/final and multiple colortex buffers/history.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional large extension; defer wholesale.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pack buffer semantics and shader translation are not vanilla and depend on 26.3-era APIs.
- RustChunk implementation mapping: Pack buffer semantics and shader translation are not vanilla and depend on 26.3-era APIs.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 705. `crates/horizon-client-render/src/gfx/pack_hand.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1ecb4581011da4c69de271a101ee6f28bc2c8921`
- Size: 6172 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack fullscreen helper shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Provides blit/copy/downsample helpers for the pack post graph.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional shader helper.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Bindings and target formats are specific to Starling's post renderer.
- RustChunk implementation mapping: Bindings and target formats are specific to Starling's post renderer.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 706. `crates/horizon-client-render/src/gfx/pack_images.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `11ac8c4ca6c2dda714a2a0f18cd3a7566faa0f53`
- Size: 22157 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack shadow buffers
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Maintains shadow depth and colour textures and ping-pong state for shader packs.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional extension only.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Texture dimensions/clear and sampler semantics depend on pack API and target GPU.
- RustChunk implementation mapping: Texture dimensions/clear and sampler semantics depend on pack API and target GPU.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 707. `crates/horizon-client-render/src/gfx/pack_post.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `76cf501b701ee6134ef3d5efec9f1615ec787120`
- Size: 184421 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack sky geometry
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Builds sky disc, sunset fan, stars and textured sun/moon inputs for pack gbuffers stages.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional shader-pack integration; some geometry ideas are adaptable.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Star/sun resources and pack stage API differ from 1.20.1 vanilla rendering.
- RustChunk implementation mapping: Star/sun resources and pack stage API differ from 1.20.1 vanilla rendering.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 708. `crates/horizon-client-render/src/gfx/pack_post.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `299e9207b28b9ba2bbab0cabedc28695092fbf43`
- Size: 2300 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack custom texture upload
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Maps custom pack textures and sampler declarations to GPU textures/views.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional extension; resource hardening required.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Format and sampler mapping are custom and rely on wgpu texture capabilities.
- RustChunk implementation mapping: Format and sampler mapping are custom and rely on wgpu texture capabilities.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 709. `crates/horizon-client-render/src/gfx/pack_shadow.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `92f0aeda064a4d8619e82c0401e96a61f33675ae`
- Size: 7748 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GPU pass timing for shader packs
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Collects asynchronous timestamp query data and reports per-pass medians/distributions.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Useful optional profiling architecture.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Timestamp query support differs across mobile GPUs; should degrade gracefully.
- RustChunk implementation mapping: Timestamp query support differs across mobile GPUs; should degrade gracefully.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 710. `crates/horizon-client-render/src/gfx/pack_sky.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a160302ab59303566c323d282969f5be9708826c`
- Size: 14050 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack frame uniforms
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Builds camera, player status, biome, item, weather, fog and matrix uniforms for shader packs.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional API mapping; rebuild fields for 1.20.1.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Numerous fields refer to newer items/effects/biome registry and shader spec.
- RustChunk implementation mapping: Numerous fields refer to newer items/effects/biome registry and shader spec.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 711. `crates/horizon-client-render/src/gfx/pack_textures.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ed669667dbd48485eeee3c5fba449ef27391d7ae`
- Size: 14618 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack weather draw
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Converts rain/snow to pack weather vertices/textures and animation phases.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional extension; weather geometry may inform normal renderer.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Resource UV and weather animation semantics are version-sensitive.
- RustChunk implementation mapping: Resource UV and weather animation semantics are version-sensitive.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 712. `crates/horizon-client-render/src/gfx/pack_timing.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f030db0bc0bdb7cdbcd9cd253f79b8a7e57e0fc9`
- Size: 12865 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Vertex arena page-run grouping
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Groups solid draws by GPU arena page within distance bands to reduce bind/draw call overhead.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Potential batching optimization; prove on target GPU.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Grouping may alter tie ordering and interacts with depth/opaque rendering assumptions.
- RustChunk implementation mapping: Grouping may alter tie ordering and interacts with depth/opaque rendering assumptions.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 713. `crates/horizon-client-render/src/gfx/pack_uniforms.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `20f088800c4ca23e21962d6d3a9871598f7d1694`
- Size: 43172 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Title panorama resource loader/renderer
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Loads six cube-map faces, validates/reduces them and draws animated title panorama.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: UI feature; reuse resource validation patterns.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Panorama files/rotation and resource manager behavior must match 1.20.1 title assets.
- RustChunk implementation mapping: Panorama files/rotation and resource manager behavior must match 1.20.1 title assets.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 714. `crates/horizon-client-render/src/gfx/pack_weather.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `78714fc0f209295da2fdf3234a5694cfe73ebac5`
- Size: 9946 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Panorama shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Projects cube texture coordinates and camera view for title panorama.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Shader helper; adapt to RustChunk GUI pipeline.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Coordinate convention and WGSL bindings are implementation-specific.
- RustChunk implementation mapping: Coordinate convention and WGSL bindings are implementation-specific.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 715. `crates/horizon-client-render/src/gfx/page_runs.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b7b04f14b804b1927e2a14f425c8ac94df3bc831`
- Size: 7858 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Swapchain screenshot/readback fallback
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Renders to a readable offscreen texture when the surface cannot be copied from, then presents it.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Useful screenshot portability pattern.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Surface COPY_SRC support and format behavior differ by backend.
- RustChunk implementation mapping: Surface COPY_SRC support and format behavior differ by backend.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 716. `crates/horizon-client-render/src/gfx/panorama.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3ceaa613df3440a720f789ea1ad8ab83ab66a667`
- Size: 24587 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Resident section storage and frustum walk
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Stores section values densely by chunk column and rejects whole columns with distance/frustum bounds.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: High-value architecture for chunk residency/render culling.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Section layout, min_y and mesh bounds need exact target dimension handling.
- RustChunk implementation mapping: Section layout, min_y and mesh bounds need exact target dimension handling.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 717. `crates/horizon-client-render/src/gfx/panorama.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e4377c20bebd33c2511081f1ed26f4a61df6b806`
- Size: 719 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GPU object/pipeline reuse cache
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Reuses compiled shader modules/layouts/pipelines across renderer rebuild generations.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Useful startup/resource reload optimization.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Cache keys based on Debug descriptors and handle identity must not alias changed shader inputs.
- RustChunk implementation mapping: Cache keys based on Debug descriptors and handle identity must not alias changed shader inputs.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 718. `crates/horizon-client-render/src/gfx/readback.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c1b8edd0dd3e9e4bbdbb7af2088b3ee3d7d83cc0`
- Size: 8537 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Adreno solid terrain fast paths
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Substitutes cheaper solid/cutout pipelines on selected Adreno Vulkan devices after equivalence testing.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Do not apply to Mali by default; optional exact-device path.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Benchmarks are from named Adreno devices and the source explicitly limits applicability.
- RustChunk implementation mapping: Benchmarks are from named Adreno devices and the source explicitly limits applicability.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 719. `crates/horizon-client-render/src/gfx/resident.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `87999736a993afc02300044337e233b11c32a2ec`
- Size: 30163 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Solid-path GPU parity tests
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Compares optimized static/cutout terrain pipelines with the usual terrain output.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Reference tests only, not executed.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Requires GPU and prepared assets; scenes reflect pinned newer renderer.
- RustChunk implementation mapping: Requires GPU and prepared assets; scenes reflect pinned newer renderer.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 720. `crates/horizon-client-render/src/gfx/reuse.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `dc606d668bfd97e7e5e71dd64a51aaa981d52a1c`
- Size: 9651 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Static star geometry
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Builds seeded night-sky star triangles once and updates only a small rotation/brightness uniform.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Reusable optimization pattern.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Star distribution/seed and sky radius must match 1.20.1.
- RustChunk implementation mapping: Star distribution/seed and sky radius must match 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 721. `crates/horizon-client-render/src/gfx/solid_paths.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `199d27278fc9d36796691b2fdb1221ac9309cf5a`
- Size: 6405 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Star shader
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Transforms star vertices and applies frame brightness.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Shader helper.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pipeline bindings and far-plane behavior are backend-specific.
- RustChunk implementation mapping: Pipeline bindings and far-plane behavior are backend-specific.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 722. `crates/horizon-client-render/src/gfx/solid_paths/tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9db0cdc9496113ea1ad18029602342befc8ed770`
- Size: 22677 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Asynchronous queue submission
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Hands command encoding/submission to a worker thread and preserves queue ordering around writes/readbacks.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Experimental CPU optimization; do not enable without profiling.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Source comments describe driver-specific regressions and OS-specific performance; threaded submission can add stalls.
- RustChunk implementation mapping: Source comments describe driver-specific regressions and OS-specific performance; threaded submission can add stalls.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 723. `crates/horizon-client-render/src/gfx/stars.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b8db60760e27b8ec37d61a04a54435dc26767d99`
- Size: 9109 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Sunrise/sunset fan mesh
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Builds the sky fan geometry once per frame instead of per-pixel ray/triangle search.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: High-value algorithmic optimization for sky rendering.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The function computes a visual approximation tied to 1.20.1 sky colours and sun orientation.
- RustChunk implementation mapping: The function computes a visual approximation tied to 1.20.1 sky colours and sun orientation.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 724. `crates/horizon-client-render/src/gfx/stars.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b4a29f93f1f7aa0a83c4d526dcf32377e716be98`
- Size: 1159 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GLES terrain shader helper include
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Provides GLSL ES-compatible sampling functions for terrain textures and mip levels.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Essential only if the GLES backend is chosen; validate exact output.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: GLES compiler/precision/array behavior varies by driver, especially Mali.
- RustChunk implementation mapping: GLES compiler/precision/array behavior varies by driver, especially Mali.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 725. `crates/horizon-client-render/src/gfx/submit.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ddb1067dc96b83abc77e65c8bc17279df1be744e`
- Size: 9290 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Terrain shader source lowering
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Keeps Vulkan WGSL unchanged while rewriting specific constructs for legacy GLES shader compilation.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Useful portability layer; make transformations narrow and tested.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Adreno-specific workarounds and current Naga output are not generic Mali guarantees.
- RustChunk implementation mapping: Adreno-specific workarounds and current Naga output are not generic Mali guarantees.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 726. `crates/horizon-client-render/src/gfx/sunrise_fan.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1baef2c2092b1b09c5cb55f546821e5e97473e20`
- Size: 10668 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GPU test instance configuration
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Creates the GPU instance/backend set used by renderer tests.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Test harness only.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Windows/Vulkan/DX12 choices do not generalize to Android or every device.
- RustChunk implementation mapping: Windows/Vulkan/DX12 choices do not generalize to Android or every device.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 727. `crates/horizon-client-render/src/gfx/terrain_gles.wgsl.inc`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `28ce268c71dd690f86c5600e0af88fafb0bd1b72`
- Size: 3799 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GPU timestamp timer
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Reads timestamp query data asynchronously and frees query slots when results arrive or fail.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Useful diagnostic architecture; not always available on mobile.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Timestamp support and query resolve semantics vary on GLES/Android.
- RustChunk implementation mapping: Timestamp support and query resolve semantics vary on GLES/Android.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 728. `crates/horizon-client-render/src/gfx/terrain_source.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5f12e5dbfa6be4916c66ea6fbe840c5577fdc9ab`
- Size: 7240 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Translucent quad sorter
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Sorts translucent faces by distance while changing indices rather than moving vertices.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Reusable algorithm; validate target ordering rules.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Tie order and camera crossing rules are observable; large LOD tiles need a different point-of-view policy.
- RustChunk implementation mapping: Tie order and camera crossing rules are observable; large LOD tiles need a different point-of-view policy.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 729. `crates/horizon-client-render/src/gfx/test_gpu.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0cea7221974cae1e8baca31fc6e84509c2a64543`
- Size: 683 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: GPU upload staging belt
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Reuses mapped staging buffers for per-frame buffer/texture uploads rather than allocating one per write.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: High-value performance/memory pattern.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Staging buffer alignment, map lifecycle and max buffer size vary on Android.
- RustChunk implementation mapping: Staging buffer alignment, map lifecycle and max buffer size vary on Android.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 730. `crates/horizon-client-render/src/gfx/timing.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e5c26b63055d5f2987e5458a85f885462c227fd6`
- Size: 15325 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Multi-view camera abstraction
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Shares camera-relative world data across one or two eye views while varying projection/rotation.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional XR/split-view architecture; not needed for flat Android client.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Stereo view count and offsets are not vanilla Java gameplay requirements.
- RustChunk implementation mapping: Stereo view count and offsets are not vanilla Java gameplay requirements.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 731. `crates/horizon-client-render/src/gfx/translucency.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4c1ad66839147cd3d3f8824c0d5f5e053e38d534`
- Size: 5356 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Weather draw renderer
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Builds and draws rain/snow quads with native and optional shader-pack paths.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Adapt vanilla weather renderer separately from pack integration.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Sprite UVs, pass order and depth-write behavior must match 1.20.1.
- RustChunk implementation mapping: Sprite UVs, pass order and depth-write behavior must match 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 732. `crates/horizon-client-render/src/gfx/upload.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c3da6cac4d5231c7e1a03c75feede7f229b1ca38`
- Size: 15523 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: WGSL namespace prefix transformer
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Extracts and prefixes top-level shader names to combine vanilla snippets with pack code safely.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Useful shader composition helper but optional.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Parser/token rewriting must not corrupt nested scopes, comments, macros or target shader syntax.
- RustChunk implementation mapping: Parser/token rewriting must not corrupt nested scopes, comments, macros or target shader syntax.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 733. `crates/horizon-client-render/src/gfx/view.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `173a665d15a346b97f9e726e9b428541847adf8f`
- Size: 4401 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: World post-processing and TAA
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Manages render resolution, FXAA/TAA history, reprojection and upscale pass.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Potential quality/performance module; keep optional and benchmarked.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: History uses current camera-only motion and may ghost moving entities; scaling policy is not target default.
- RustChunk implementation mapping: History uses current camera-only motion and may ghost moving entities; scaling policy is not target default.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 734. `crates/horizon-client-render/src/gfx/weather.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5e06e82faa6a7c79c870a4c93863fda12cb24dc0`
- Size: 21780 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Graphics settings model
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Stores renderer/API/preset/upscaling/LOD/culling/HDR options and recovery markers.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Reuse separation of vanilla and renderer-specific settings; adapt to RustChunk-owned keys.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: API names, preset catalogs and driver workarounds are Horizon-specific; mobile crash-recovery policy needs adaptation.
- RustChunk implementation mapping: API names, preset catalogs and driver workarounds are Horizon-specific; mobile crash-recovery policy needs adaptation.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 735. `crates/horizon-client-render/src/gfx/wgsl_prefix.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `03d84a56e41339a37abfce08c95673669c891637`
- Size: 4424 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Graphics settings style
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Defines visual design tokens and draw primitives for graphics settings UI.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: UI-only reference; no game compatibility dependency.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Colours/layout are Horizon branding and not required for RustChunk.
- RustChunk implementation mapping: Colours/layout are Horizon branding and not required for RustChunk.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 736. `crates/horizon-client-render/src/gfx/world_post.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `87823a5939988a89e371dccbca3c1ba1d87022e9`
- Size: 28908 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Graphics settings UI
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Presents pages, search, toggles/sliders and descriptions for renderer settings.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional UI layer; defer until renderer controls stabilize.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Settings options and dependencies are Horizon-specific; large UI logic is not vanilla parity.
- RustChunk implementation mapping: Settings options and dependencies are Horizon-specific; large UI logic is not vanilla parity.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 737. `crates/horizon-client-render/src/graphics.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b0c674c4a9728810474e711735a14a284f101a0b`
- Size: 59054 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HDR transfer/composite shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Maps world/GUI into HDR output and handles PQ/scRGB and calibration visuals.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional display feature; defer for current Android target.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Not required for Minecraft 1.20.1 world correctness and depends on HDR output surface support.
- RustChunk implementation mapping: Not required for Minecraft 1.20.1 world correctness and depends on HDR output surface support.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 738. `crates/horizon-client-render/src/graphics_style.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1e67de01078a161e7ddb3c986835f0480d3cfd11`
- Size: 10169 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HDR calibration workflow
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Guides the user through black-level, peak brightness and paper-white calibration.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional settings UI.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Display calibration semantics and platform UI differ; unnecessary if no HDR output.
- RustChunk implementation mapping: Display calibration semantics and platform UI differ; unnecessary if no HDR output.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 739. `crates/horizon-client-render/src/graphics_ui.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f9ba3e7f5373a08239d2a94d5f1899d7c10e753b`
- Size: 132098 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HDR statistics shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Measures bright/opaque sprite texels for HDR threshold selection.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional HDR helper.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Depends on GPU storage/compute and HDR feature.
- RustChunk implementation mapping: Depends on GPU storage/compute and HDR feature.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 740. `crates/horizon-client-render/src/hdr.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ee800c8b630e81f7509d638db7c5cd278f463884`
- Size: 10118 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HUD atlas and batch renderer
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Packs GUI, font, item/block sprites and animation data into a shared atlas; emits GUI geometry batches.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: High-value UI/atlas architecture; rebuild for 1.20.1 assets.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Atlas budget, mod icon/skin/custom button slots and texture max sizes are Horizon-specific.
- RustChunk implementation mapping: Atlas budget, mod icon/skin/custom button slots and texture max sizes are Horizon-specific.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 741. `crates/horizon-client-render/src/hdr_calibration.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e9ae67bbdc38fbe025f43e598c4b464435b4dab5`
- Size: 11295 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HUD shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Draws batched GUI sprites/textures and special overlays.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Core renderer component to adapt with version-specific assets.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Shader bindings and atlas formats depend on RustChunk implementation.
- RustChunk implementation mapping: Shader bindings and atlas formats depend on RustChunk implementation.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 742. `crates/horizon-client-render/src/hdr_stats.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f4ca3aaf9dc9d329db269a2657efed6504339e59`
- Size: 3127 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: CIT glint GUI texture layer
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Adds OptiFine-style custom enchantment glint textures to GUI item quads.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional resource-pack compatibility feature.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: CIT rule/item component format reflects later data; not vanilla 1.20.1.
- RustChunk implementation mapping: CIT rule/item component format reflects later data; not vanilla 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 743. `crates/horizon-client-render/src/hud.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2b4b75d4a9b05e5d550014ff0c153aa73c60ebd1`
- Size: 200115 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Shader-pack GUI text renderer
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Substitutes resource pack core/text shaders for GUI text and preserves uniform/vertex layout.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: Optional shader-pack feature.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Shader uniform blocks and pack interface are custom.
- RustChunk implementation mapping: Shader uniform blocks and pack interface are custom.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 744. `crates/horizon-client-render/src/hud.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `cc546b4f5d507a19c2f3e373981d3899b3888f73`
- Size: 5606 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HUD element layout and draw calls
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Computes vanilla-like hotbar, crosshair, hearts, armour, food, vehicle health and experience draw primitives.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: High-value parity architecture; rebuild state inputs for 1.20.1.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Current catalogue/UI fields may include newer options and effects.
- RustChunk implementation mapping: Current catalogue/UI fields may include newer options and effects.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 745. `crates/horizon-client-render/src/hud/cit_glint.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1afec496cbb18dd0ca4fd71c110b0d21bbe8d618`
- Size: 22872 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: HUD overlays, boss bars, status, titles and subtitles
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Builds overlays for effects, boss bars, title/subtitle, selected-item name, border, portal, vignette and other screens.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: Useful rendering separation; rebuild registries and packet inputs.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Effect/bossbar IDs and overlay rules are version-sensitive.
- RustChunk implementation mapping: Effect/bossbar IDs and overlay rules are version-sensitive.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 746. `crates/horizon-client-render/src/hud/pack_text.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8f5ea2c86ba81dda527dc2e5bc41614451b93f47`
- Size: 13543 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Inventory effect and item display logic
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Computes effect/slot information shown in inventory-facing UI.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: Adapt with target effect and item registries.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Current effect catalog/item components include newer data forms.
- RustChunk implementation mapping: Current effect catalog/item components include newer data forms.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 747. `crates/horizon-client-render/src/hud_layout.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `99fa41494c611176b7afb79520632f384dd7d833`
- Size: 31615 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Java text/string semantic compatibility
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Implements Java-like string/number parsing and text operations used by renderer/UI compatibility.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: High-value utility if exact semantics are retained.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Unicode, UTF-16 indexing, numeric syntax and Java rounding must be verified against JDK 17/21 used by target.
- RustChunk implementation mapping: Unicode, UTF-16 indexing, numeric syntax and Java rounding must be verified against JDK 17/21 used by target.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 748. `crates/horizon-client-render/src/hud_overlays.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6dad36f6042202970c76ce27f7d872ad92a7073a`
- Size: 42955 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Jigsaw block UI and state helpers
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Represents jigsaw block configuration/visual interaction data.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: Specialized UI/gameplay support; target 1.20.1 field set required.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Jigsaw NBT/packet properties and screen details may have evolved.
- RustChunk implementation mapping: Jigsaw NBT/packet properties and screen details may have evolved.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 749. `crates/horizon-client-render/src/inventory_effects.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9d81f379c67f89edbffc320b63bdb3e0cf86b84a`
- Size: 8568 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: On-screen keypad/input overlay
- Dependencies / cross-references: Depends on target-version registries, entity/block state, random source, particle atlas and client tick ordering.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Provides a virtual keypad/control interface for touch or text input.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional Android UX feature.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Not vanilla UI behavior; hardware keyboard and IME may be preferable.
- RustChunk implementation mapping: Not vanilla UI behavior; hardware keyboard and IME may be preferable.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 750. `crates/horizon-client-render/src/java_text.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `25eecf67e039240dcc8a1c95f023a4aa4d83a087`
- Size: 20319 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Level loading progress and status map
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Tracks server/client readiness, player chunk readiness, progress and loading overlays.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: High-value state separation; rebuild packet/status lifecycle for 1.20.1.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Status enum and packet event ordering must follow 1.20.1 login/respawn/chunk flow.
- RustChunk implementation mapping: Status enum and packet event ordering must follow 1.20.1 login/respawn/chunk flow.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 751. `crates/horizon-client-render/src/jigsaw_block.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `692094e06077d77c849e81d05a7a9f2d363c884a`
- Size: 5241 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: World block/sky lighting engine
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Computes block and sky light, face occlusion and incremental relighting over resident chunks.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Critical core system; implement against 1.20.1 light engine, not the 26.3 tables.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: A mismatch causes dark/bright seams, chunk flicker and gameplay/visual errors; source registry facts are version-specific.
- RustChunk implementation mapping: A mismatch causes dark/bright seams, chunk flicker and gameplay/visual errors; source registry facts are version-specific.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 752. `crates/horizon-client-render/src/keypad.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `68d7cae6b4b20ba4e6c14731c7cc6da299d61f86`
- Size: 13095 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Far-terrain LOD sampling and selection
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Samples distant column data, schedules tile meshing and selects coarse tiles beyond resident chunks.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: High-value architecture for distant view, but isolate from near gameplay and collision.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Far tiles are explicitly outside World/simulation; algorithms/data are 26.3 and must not replace loaded chunks near player.
- RustChunk implementation mapping: Far tiles are explicitly outside World/simulation; algorithms/data are 26.3 and must not replace loaded chunks near player.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 753. `crates/horizon-client-render/src/level_loading.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5c13bc71fee5ba92724aaf8b98b7731ba61b4f40`
- Size: 30651 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Far-terrain disk cache
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Stores sampled columns in append-only region files keyed by generator/build data.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional performance feature; adopt only after cache invalidation is proven.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Format/build key/registry dependencies are current Starling-specific; corrupted/torn writes must not crash load.
- RustChunk implementation mapping: Format/build key/registry dependencies are current Starling-specific; corrupted/torn writes must not crash load.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 754. `crates/horizon-client/examples/probe/raid_fixture.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `faf0eedf54d88a78a4da262f8d142b4becbf5580`
- Size: 39654 bytes; 1043 lines
- Coverage: Complete source text reviewed in consecutive ranges: lines 1–400, 401–800 and 801–1043.
- Purpose: Large deterministic parity fixture for pillager patrol spawning, raid lifecycle, raider membership, boss-bar state, victory and raid NBT codec behavior.
- Dependencies / cross-references: `Assets`, `BlockFacts`, `MobData`, `Items`, spawn rules/selection, `PatrolSpawner`, `Raids`, `RaidLevel`, `LegacyRandom`, world/biome/light/chunk stubs and SNBT.
- Implementation / logic: Builds a synthetic world, parses scripted events, runs patrol or raid ticks, records spawn/horn/effect/leave/state events, and supports a codec mode for raid NBT round trips.
- Findings: The fixture covers many interacting states and is useful for scenario construction. It deliberately stubs world seed, thunder, player-near and some world interactions; `Bx::parse` and several scenario parsers assume well-formed lengths. Raid timing and Bad Omen behavior are snapshot-sensitive.
- RustChunk relevance: Valuable as a simulator/parity-test blueprint for vanilla raids and patrols; not a production world implementation and not safe to transplant wholesale.
- Decision: Adapt as a test-harness architecture only; re-derive patrol/raid state transitions and scenario outputs from the Java 1.20.1 oracle.
- 1.20.1 implications: Pillager patrols and raids exist in 1.20.1, but the pinned snapshot is newer. Validate Bad Omen triggering, raid waves, spawning, victory effects, dimensions, serialization and boss-bar behavior against 1.20.1.
- Follow-up checks: Create exhaustive 1.20.1 oracle cases for patrol eligibility, village/POI selection, chunk ticking, unload/reload, raid extension/stop, wave composition, raider removal, victory/defeat and NBT persistence; harden malformed test-input handling.
- RustChunk implementation mapping: Candidate test harness under `tests/parity/raids`; reusable pure parts may map to `gameplay::patrol` and `gameplay::raid`, with synthetic world traits isolated from real world state.
- Evidence required before adoption: Full Java 1.20.1 source comparison and differential outputs for every state transition; dedicated tests for stubbed world assumptions and NBT migration. This review did not execute the fixture.
- Completion state: FULL TEXT REVIEW COMPLETE — all 1,043 lines read in ordered chunks; no tests, build, server or device execution is claimed.


### 755. `crates/horizon-client-render/src/light.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b9ebf31e4e679f8ac9635b35bfcc0060cff9873f`
- Size: 89964 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Rotating logs and crash-log retention
- Dependencies / cross-references: Depends on authoritative chunk state, block facts/shape occlusion, fluids, dimensions and world update lifecycle.
- Implementation / logic: Block/sky light propagation and incremental relighting over section-neighbour regions.
- Findings: Writes game logs, rollover archives, recent ring buffer and crash reports with token redaction concerns.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Adapt logging architecture to Android app storage and privacy policy.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: File paths, rotation frequency and Android storage access differ from this implementation.
- RustChunk implementation mapping: File paths, rotation frequency and Android storage access differ from this implementation.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 756. `crates/horizon-client-render/src/lod.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0998593051cc6cd9124b08cfaaa6643c81a3b975`
- Size: 201066 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Menu background blur
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: CPU world/mesh algorithm, usually operating on section/palette snapshots and emitting cached or visible geometry.
- Findings: Implements a separable multi-pass blur for menu backgrounds with lower-resolution software fallback.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Optional visual optimization.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Copy-source surface support and half-resolution policy vary on GLES; not gameplay critical.
- RustChunk implementation mapping: Copy-source surface support and half-resolution policy vary on GLES; not gameplay critical.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 757. `crates/horizon-client-render/src/lod/cache.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `7b5b688f5b77bce9b33c7ae2dec114fe2dd6fa4d`
- Size: 12805 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Menu blur shader
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: CPU world/mesh algorithm, usually operating on section/palette snapshots and emitting cached or visible geometry.
- Findings: Performs separable box blur/downsample operations for menu background.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Shader helper.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pipeline format and sampling rules require backend parity.
- RustChunk implementation mapping: Pipeline format and sampling rules require backend parity.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 758. `crates/horizon-client-render/src/lod/mesh.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e40434fc79954ccfe4ec9e7c7ef68a0bffedf7f5`
- Size: 62766 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Fluid surface tessellation
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: CPU world/mesh algorithm, usually operating on section/palette snapshots and emitting cached or visible geometry.
- Findings: Generates liquid heights, faces, UV flow, neighbour occlusion, overlay and lighting.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Critical renderer feature; reconstruct from 1.20.1 FluidRenderer and shapes.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Fluid data and block shapes must be target version; waterlogged states and transparent neighbours are sensitive.
- RustChunk implementation mapping: Fluid data and block shapes must be target version; waterlogged states and transparent neighbours are sensitive.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 759. `crates/horizon-client-render/src/logging.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c02a326dc2ad4f44393380c0f7bf74001450ba02`
- Size: 33508 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Leaf face/interior culling
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Builds section-local leaf-volume masks and classifies safe leaf faces for solidification/removal.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional mesher optimization with high potential gains.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Conservative unknown/open handling is essential; aggressive mode may change appearance if chunk neighbours are missing.
- RustChunk implementation mapping: Conservative unknown/open handling is essential; aggressive mode may change appearance if chunk neighbours are missing.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 760. `crates/horizon-client-render/src/menu_blur.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ee8fc12d82fc6639b266448ba21fc90f0ec56615`
- Size: 17044 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Leaf-culling regression suite
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Checks leaf mode geometry, cavity faces, chunk boundaries and no-leaf equivalence.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Test/reference source; not executed.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Prepared resources required for full mesher fixture.
- RustChunk implementation mapping: Prepared resources required for full mesher fixture.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 761. `crates/horizon-client-render/src/menu_blur.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d8b15fa94478a46a2dd06f0081ab932b259b10c9`
- Size: 1151 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Terrain particle regression tests
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Checks block particles use the proper particle sprite and biome tint source.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Reference tests only.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Fixture relies on target resource model/tint catalog.
- RustChunk implementation mapping: Fixture relies on target resource model/tint catalog.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 762. `crates/horizon-client/examples/probe/random_tick_sections_probe.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `27eaf81961280096ed56d3b037df90a9ebbc4331`
- Size: 5050 bytes; 133 lines
- Coverage: Complete source text reviewed, lines 1–133.
- Purpose: Deterministic probe for chunk-section selection and block-position reads during random ticking.
- Dependencies / cross-references: `RandomTickSections`, `Chunk`, `Section`, `BlockFacts`, `LegacyRandom`, `Ticks`, `LevelAccess`, block/fluid registries and prepared resource data.
- Implementation / logic: Loads the target registry, creates a synthetic air-only level that records block reads, applies section/block edits to a chunk, invokes random-tick behavior and serializes sampled positions plus ticking/fluid flags.
- Findings: The fake level intentionally records reads rather than simulating full block updates. The section-fill operation derives a periodic block pattern from a divisor; malformed zero divisors or invalid section indices are not guarded in this fixture.
- RustChunk relevance: Useful for understanding and regression-testing random-tick section selection, including cache behavior; it is a probe, not the scheduler itself.
- Decision: Use as a parity-test reference; implement only after confirming section selection and random sequence against Java 1.20.1.
- 1.20.1 implications: The random-tick system exists in 1.20.1, but section bounds, chunk height, registry IDs and seeded random call order must match the 1.20.1 world model.
- Follow-up checks: Compare seeded oracle cases at min/max build height, empty/non-empty sections, mixed ticking/non-ticking blocks, lava and random-tick-speed changes; add malformed-input validation.
- RustChunk implementation mapping: Candidate RustChunk test harness under `tests/parity/random_tick_sections`; keep `RandomTickSections` cache separate from chunk mesh/LOD caches.
- Evidence required before adoption: Exact Java 1.20.1 sampled-position traces and RNG-state parity across supported dimensions; no probe execution is claimed.
- Completion state: FULL TEXT REVIEW COMPLETE — full source text read and documented; no test, build or runtime execution is claimed.


### 763. `crates/horizon-client-render/src/mesher/fluid.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e12489d660be0c30b27a44d84e2885e8600fc8a2`
- Size: 16349 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Solid face bucketing/sort
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: CPU world/mesh algorithm, usually operating on section/palette snapshots and emitting cached or visible geometry.
- Findings: Groups quads by facing and emits only runs potentially visible from the camera.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Potential CPU/GPU optimization; preserve conservative bucket for unusual faces.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Winding, degenerate triangles and ties must match exact 1.20.1 model geometry.
- RustChunk implementation mapping: Winding, degenerate triangles and ties must match exact 1.20.1 model geometry.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 764. `crates/horizon-client-render/src/mesher/leaf_culling.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5b5dde93b23bdd01892ecc686e3116a0181e6d9c`
- Size: 24407 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Biome tint shortcut tests
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: CPU world/mesh algorithm, usually operating on section/palette snapshots and emitting cached or visible geometry.
- Findings: Checks one-biome shortcut against full spatial tint interpolation.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Reference tests only.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Biome definitions/colormaps are pinned to newer game data.
- RustChunk implementation mapping: Biome definitions/colormaps are pinned to newer game data.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 765. `crates/horizon-client-render/src/mesher/leaf_culling_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9c1b21a2ae37b9d5ff8d66f6642133624e023541`
- Size: 10594 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Section visibility/face-connectivity tests
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Uses a slow reference to verify optimized block/section visibility masks and row fill.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Reference tests only; valuable proof harness.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Block opacity/face-occlusion facts are versioned.
- RustChunk implementation mapping: Block opacity/face-occlusion facts are versioned.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 766. `crates/horizon-client/examples/probe/recipe_book_fixture.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `59733317b32e6313488a637e6662749d095bf557`
- Size: 6554 bytes; 164 lines
- Coverage: Complete source text reviewed, lines 1–164.
- Purpose: Headless parity fixture for recipe-book known/seen state, filtering/open flags, persistence and settings packet serialization.
- Dependencies / cross-references: `RecipeIndex`, `Settings`, `BookType`, `RecipeBookChangeSettings`, protocol `Reader`/`Writer`, NBT `Tag`, pack overlay and JSON inputs.
- Implementation / logic: Applies give/take/seen/set/save/load operations, encodes then decodes recipe-book settings packets, and prints deterministic counts, hashes, settings and NBT summaries.
- Findings: The fixture exercises the four standard book categories and filters unknown recipe IDs. The fallback for an unrecognized type silently selects Smoker; the load case deliberately injects invalid IDs. Wire packet layout is protocol-specific.
- RustChunk relevance: Strong test-harness reference for recipe-book persistence and state transitions; do not reuse its packet bytes across protocol versions.
- Decision: Adapt the recipe-book state model as a test reference and rebuild packet/registry fixtures for Java 1.20.1.
- 1.20.1 implications: Crafting, furnace, blast-furnace and smoker recipe-book categories exist in 1.20.1; confirm packet fields, NBT keys, recipe-ID validation and defaults with the 1.20.1 client.
- Follow-up checks: Test all four book categories, duplicate/unknown IDs, save-load cycles, open/filter combinations, missing NBT keys and byte-for-byte 1.20.1 packet encoding.
- RustChunk implementation mapping: Candidate RustChunk modules: `gameplay::recipe_book` and versioned `protocol::play` settings packet; keep UI presentation separate from persisted state.
- Evidence required before adoption: Java 1.20.1 state/packet oracle comparisons and resource registry parity. No tests were run during this review.
- Completion state: FULL TEXT REVIEW COMPLETE — full source text read and documented; no test, build or runtime execution is claimed.


### 767. `crates/horizon-client-render/src/mesher/particle_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `bbdc70af477b397dfed1644b2f93fdc3c2ece87e`
- Size: 3682 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Mip level generation and alpha coverage
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Builds sprite mipmaps with linear-light colour filtering and alpha-test coverage preservation.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: High-value texture quality algorithm; validate exact target format/rounding.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Mip strategy, alpha coverage and high-res resource behavior must be compared with 1.20.1.
- RustChunk implementation mapping: Mip strategy, alpha coverage and high-res resource behavior must be compared with 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 768. `crates/horizon-client-render/src/mesher/reuse_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f31866c608225f85ecd1f7800999701640ddadc3`
- Size: 15466 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Mod enablement list
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Persists per-mod enable/disable choices and distinguishes native mods built into an app.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Optional mod manager feature; not vanilla.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Horizon `.hmod`/native build scheme does not establish Forge mod loading or Forge compatibility.
- RustChunk implementation mapping: Horizon `.hmod`/native build scheme does not establish Forge mod loading or Forge compatibility.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 769. `crates/horizon-client-render/src/mesher/sides.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f8c0a4f6d0b5f589c7804f13773c78ac93a5a75b`
- Size: 27069 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Parsed model library
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: CPU world/mesh algorithm, usually operating on section/palette snapshots and emitting cached or visible geometry.
- Findings: Loads and resolves resource models and fallback behavior across resource-pack reloads.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: High-value resource architecture; rebuild 1.20.1 model/OBJ codecs.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Model parent resolution and NeoForge OBJ extension are not all vanilla; resource path/override rules are versioned.
- RustChunk implementation mapping: Model parent resolution and NeoForge OBJ extension are not all vanilla; resource path/override rules are versioned.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 770. `crates/horizon-client-render/src/mesher/tint_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2a7ec6c22a9695e5147c4ea5fab0c408fd1ced3b`
- Size: 4252 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Standalone model registration/gallery
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Registers standalone models requested by native mods and prepares instances for inspection/gallery.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Optional native mod feature.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Not part of vanilla blockstate rendering; native mod API dependency.
- RustChunk implementation mapping: Not part of vanilla blockstate rendering; native mod API dependency.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 771. `crates/horizon-client-render/src/mesher/visibility_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1ba81192c1454046cf80094e42e078203e8e1c2c`
- Size: 11823 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Blockstate model catalog and baking
- Dependencies / cross-references: Depends on chunk sections, block/biome registries, voxel shapes, light tables, mesh data and section residency.
- Implementation / logic: Regression/parity fixture source; constructs inputs and asserts geometry, state or pixel behaviour. The test code was read, but it was not executed.
- Findings: Maps registry states/variants/multipart rules to baked geometry and particle sprites.
- RustChunk relevance: Relevant to chunk, world, meshing or lighting correctness; preserve authoritative near-world state and conservative fallbacks.
- Decision: Critical renderer architecture; regenerate from 1.20.1 registries/assets.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Current states and definitions target 26.3, and IDs/variant schemas cannot be imported.
- RustChunk implementation mapping: Current states and definitions target 26.3, and IDs/variant schemas cannot be imported.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 772. `crates/horizon-client-render/src/minimap.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8d1069007efa620a6444dd0c69667412d7d4b5f0`
- Size: 22800 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Cube face style/greedy merge helper
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Identifies exact cube faces whose texture UVs can be merged into greedy rectangles.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Potential mesh optimization after 1.20.1 model parity.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Only exact unit-cube cases are safe; partial shapes, rotation, emissive and transparent materials must remain separate.
- RustChunk implementation mapping: Only exact unit-cube cases are safe; partial shapes, rotation, emissive and transparent materials must remain separate.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 773. `crates/horizon-client-render/src/mipmap.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c0532c08946db03c0cd475dae4f33da7c8571aa4`
- Size: 16697 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Multiline editor widget
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Provides wrapping, selection, cursor, keyboard shortcuts, word navigation, scroll and input state.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: Useful reusable UI component; connect to RustChunk IME and text model.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Current UI metrics/character indexing may differ from Java 1.20.1 UTF-16/IME contracts.
- RustChunk implementation mapping: Current UI metrics/character indexing may differ from Java 1.20.1 UTF-16/IME contracts.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 774. `crates/horizon-client-render/src/mod_list.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6ed30a1c2317a263204f2bdb2ec89cbf42a710a7`
- Size: 9356 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Native emissive texture overlays
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Loads bounded emissive overlays for native mods and OptiFine textures, pairing overlays with base sprites.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional resource-pack extension; visual-only.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Overlay formats and suffix mapping are third-party conventions, not vanilla behavior.
- RustChunk implementation mapping: Overlay formats and suffix mapping are third-party conventions, not vanilla behavior.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 775. `crates/horizon-client-render/src/model_library.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0152f645f46303fe0f37aa51b6a871bb49d66a1a`
- Size: 10666 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Offline player name and identity persistence
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Persists/generates a stable offline name so local/LAN sessions do not all collide as one profile.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: High relevance to offline-first RustChunk; adapt validation and UUID generation.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Offline UUID/name behavior must match Java 1.20.1 and must not masquerade as authenticated online identity.
- RustChunk implementation mapping: Offline UUID/name behavior must match Java 1.20.1 and must not masquerade as authenticated online identity.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 776. `crates/horizon-client-render/src/models.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e82f3605165359593b6c89f6d1a340a5e07d6104`
- Size: 7445 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: OIT core WGSL functions
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Implements depth normalization, transmittance/absorbance accumulation and transparency composite primitives.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional OIT shader; vanilla sorted transparency remains baseline.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Float blend and target support not guaranteed on Mali/GLES.
- RustChunk implementation mapping: Float blend and target support not guaranteed on Mali/GLES.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 777. `crates/horizon-client-render/src/models/catalog.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9082ea0d1a47adbc7fbdd5197c8b10a00813a24c`
- Size: 26709 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: OIT world-border fragment shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Adapts world-border fragments to the OIT transparency pipeline.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional OIT integration.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Depends on border shader interface and OIT passes.
- RustChunk implementation mapping: Depends on border shader interface and OIT passes.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 778. `crates/horizon-client-render/src/models/cube.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `cc9af3e4c50fc9b82d7c858e70c6567ff2f6977a`
- Size: 6049 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: OIT clouds fragment shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Adapts cloud front/back transparency to OIT compositing.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional OIT integration.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Cloud blending and depth rules must match target rendering.
- RustChunk implementation mapping: Cloud blending and depth rules must match target rendering.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 779. `crates/horizon-client-render/src/multiline_edit.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `75b2e2a94cff14ce7ebc59edc6b23a84ea43c7da`
- Size: 28133 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: OIT miscellaneous feature shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Handles multiple non-terrain translucent categories in OIT pass.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional OIT integration.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Special layers such as text/lightning/particles have distinct blend/depth semantics.
- RustChunk implementation mapping: Special layers such as text/lightning/particles have distinct blend/depth semantics.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 780. `crates/horizon-client-render/src/native_emissive.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ce60644fb7bab402ad2c3c27605d5b3602049204`
- Size: 12471 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: OIT terrain fragment shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Computes terrain translucent colour/absorbance for OIT accumulation.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Optional OIT integration.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Requires target terrain texture/light/fog interface and float blend support.
- RustChunk implementation mapping: Requires target terrain texture/light/fog interface and float blend support.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 781. `crates/horizon-client-render/src/offline_name.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `762185a442a424dbefe0f581fe6715dd842d8146`
- Size: 9347 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Vanilla options file parser/schema
- Dependencies / cross-references: Depends on client account/session state, protocol identity, text and input handling; account providers must remain isolated.
- Implementation / logic: Persists and validates an offline profile name and its derived identity-related choice.
- Findings: Parses typed options.txt values, defaults, keybindings and serialization.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: High-value compatibility layer; rebuild typed key registry against 1.20.1.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The table and data codecs are explicitly 26.3 and current input/options keys differ.
- RustChunk implementation mapping: The table and data codecs are explicitly 26.3 and current input/options keys differ.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 782. `crates/horizon-client-render/src/oit.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f59cf052d16ac8852253827fd7b56af97b4a48d7`
- Size: 7688 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Options file data-fix migration
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Applies versioned migrations to old raw options keys/values.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Adapt as migration framework only; regenerate target fix sequence.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Current migrations end at 26.3; key codes/values and version IDs cannot be reused blindly.
- RustChunk implementation mapping: Current migrations end at 26.3; key codes/values and version IDs cannot be reused blindly.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 783. `crates/horizon-client-render/src/oit_border.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `cc1769e09a474861e3d3e1f3101fdcaa6ee27379`
- Size: 578 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Large options/settings UI
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Implements Options, video/audio/controls/accessibility/language/skin/resource pack pages and interaction state.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: UI reference; port screen by screen after core input/render.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Many settings and controls are newer Horizon/26.3 features; UI is not evidence of a working game engine.
- RustChunk implementation mapping: Many settings and controls are newer Horizon/26.3 features; UI is not evidence of a working game engine.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 784. `crates/horizon-client-render/src/oit_clouds.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `455bf348384d887272882f583711e058f2381231`
- Size: 1623 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Overlay texture constants and calculations
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Computes white/hurt overlay texels and codes for entity shaders.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Useful small renderer parity helper; update to target source values.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Entity overlay layout is version-sensitive though likely stable; verify against 1.20.1 texture.
- RustChunk implementation mapping: Entity overlay layout is version-sensitive though likely stable; verify against 1.20.1 texture.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 785. `crates/horizon-client-render/src/oit_features.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `50d5144cbe9975b91904947bf0bc0dcf648ac7e7`
- Size: 3057 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Custom touch-button images
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Loads/saves/scales player-provided icons for custom mobile buttons.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional Android UI feature; useful isolated utility.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Horizon-specific button count/path and texture budget.
- RustChunk implementation mapping: Horizon-specific button count/path and texture budget.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 786. `crates/horizon-client-render/src/oit_world.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0bab2cbf0ac79a0616206645fa6e9e6cf8da64a7`
- Size: 2974 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Custom player skins
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Stores and selects local skin PNGs per profile and optional upload flow.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional feature; useful offline skin support if separated from auth.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Skin layout, profile services and upload endpoints are provider/version-specific; online upload requires valid account.
- RustChunk implementation mapping: Skin layout, profile services and upload endpoints are provider/version-specific; online upload requires valid account.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 787. `crates/horizon-client-render/src/options_file.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6c7834fbb29974d4a5f07bed6585a56f1199ea29`
- Size: 33003 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Resource-pack selection screen
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Lists, filters, orders and enables resource/data packs with compatibility notices.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: High-value UI; pack activation lifecycle must be version-correct.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pack metadata, format compatibility, icons and phone storage paths are Starling-specific.
- RustChunk implementation mapping: Pack metadata, format compatibility, icons and phone storage paths are Starling-specific.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 788. `crates/horizon-client-render/src/options_fixes.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3d288ab7afe6b61b71b9574f30e314c4497eda01`
- Size: 9314 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Resource-pack discovery and composition
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Finds, orders and overlays installed packs, resource roots and enabled pack state.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: Core resource-pack architecture; rebuild exact 1.20.1 priority rules.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Pack formats/metadata may use 26.3 values; no assumption that loading a newer pack equals 1.20.1 compatibility.
- RustChunk implementation mapping: Pack formats/metadata may use 26.3 values; no assumption that loading a newer pack equals 1.20.1 compatibility.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 789. `crates/horizon-client-render/src/options_ui.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1e6eade2951755b033c2eb29c346d7dd606bc08f`
- Size: 442943 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Resource-pack derived/cache storage
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Caches parsed/decoded resource-pack artifacts and reuses them across reloads.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: Potential startup/memory optimization; cache invalidation must be exact.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Keys must include game version, pack contents, registry/data versions and implementation version.
- RustChunk implementation mapping: Keys must include game version, pack contents, registry/data versions and implementation version.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 790. `crates/horizon-client-render/src/overlay.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2d27629a594f7513a738b51aeebe4050afcf6385`
- Size: 3382 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Lightweight parsing helpers
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Provides small parser/utility routines shared by renderer resource code.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Reuse only after exact input domain is known.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The module's syntax is limited and does not replace a full data-format parser.
- RustChunk implementation mapping: The module's syntax is limited and does not replace a full data-format parser.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 791. `crates/horizon-client-render/src/own_buttons.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `45c6cadb1643d8ddf571d555adbe8030deeb3a97`
- Size: 4349 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Client particle engine
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Builds/ticks particle types and emits their geometry, colour, light and sprite animation.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Core visual system; rebuild particle registry/options and tick behavior for 1.20.1.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The source has many later particle types/options and depends on newer block/entity/item catalogs.
- RustChunk implementation mapping: The source has many later particle types/options and depends on newer block/entity/item catalogs.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 792. `crates/horizon-client-render/src/own_skin.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d31d4b8e553c49c8198e5a761ce7b99e0b871659`
- Size: 18422 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Resident POI section index
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Caches points of interest per resident section and searches nearby sections instead of scanning blocks.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Useful world-search acceleration architecture; rebuild POI type registry and update rules.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: POI types, ticket counts/ranges and block state mappings must be 1.20.1.
- RustChunk implementation mapping: POI types, ticket counts/ranges and block state mappings must be 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 793. `crates/horizon-client-render/src/pack_screen.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f3a47e011605eccdc036a72833fb151bcf1f341f`
- Size: 60874 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Vanilla post-effect chain parser/renderer
- Dependencies / cross-references: Depends on the HUD atlas, font/text system, input/IME layer, settings persistence and screen draw pipeline.
- Implementation / logic: UI state/layout/draw implementation with text metrics, touch/keyboard interaction and resource-backed visuals.
- Findings: Parses post_effect JSON, target graph, shader stages and uniforms; translates GLSL and runs passes.
- RustChunk relevance: Relevant to user-facing controls/text/UI; versioned text, input and visual semantics need 1.20.1 parity.
- Decision: High-value vanilla resource rendering architecture; exact 1.20.1 codec/shader behavior required.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: This is closer to vanilla than native shader-pack stages, but current parser and shader set are pinned to 26.3.
- RustChunk implementation mapping: This is closer to vanilla than native shader-pack stages, but current parser and shader set are pinned to 26.3.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 794. `crates/horizon-client-render/src/packs.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1627608534b6aee75ee20a27bae16b02aac7d4cd`
- Size: 69252 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Post-chain WGSL stand-in shader
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Resource-pack discovery/parsing/selection and rendering-resource composition.
- Findings: Provides WGSL fallback implementations for selected vanilla post shaders.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: Useful fallback only when target GLSL cannot translate and behavior is proven.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Stand-ins must correspond to exact 1.20.1 built-in effect shaders.
- RustChunk implementation mapping: Stand-ins must correspond to exact 1.20.1 built-in effect shaders.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 795. `crates/horizon-client-render/src/packs/cache.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `70d910e5b3e0310da0b7898752433382e2d257e5`
- Size: 46345 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Vanilla GLSL post shader translator
- Dependencies / cross-references: Depends on resource-pack discovery/priority, JSON/model/texture codecs, namespace resolution and GPU atlas limits.
- Implementation / logic: Resource-pack discovery/parsing/selection and rendering-resource composition.
- Findings: Loads includes and rewrites GLSL blocks/samplers into Naga modules for wgpu.
- RustChunk relevance: Relevant to resource-pack compatibility; keep third-party formats optional and validate untrusted assets.
- Decision: High-value portability component; narrow parser and security testing required.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: GLSL syntax/includes and uniform semantics must match 1.20.1 shaders; translator only handles supported grammar.
- RustChunk implementation mapping: GLSL syntax/includes and uniform semantics must match 1.20.1 shaders; translator only handles supported grammar.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 796. `crates/horizon-client-render/src/par.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5c7d280c5faf5d5c487d1cff200205e5f9458f6a`
- Size: 1841 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Potent-sulfur gameplay helper
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Queries block/fluid/line-of-sight around a sulfur geyser for gas and launch behavior.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Exclude from vanilla 1.20.1 base; mod-specific reference only.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: This is a modded/new block mechanic and must not enter vanilla behavior.
- RustChunk implementation mapping: This is a modded/new block mechanic and must not enter vanilla behavior.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 797. `crates/horizon-client-render/src/particles.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6a4c31fff797b8e558c3a06a3db0408902d455a6`
- Size: 187975 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: IME preedit/composition overlay
- Dependencies / cross-references: Depends on target-version registries, entity/block state, random source, particle atlas and client tick ordering.
- Implementation / logic: Client-side particle construction, tick, interpolation, collision/light sampling and sprite rendering.
- Findings: Tracks composing text, caret and selection from native input methods and exposes candidate-window bounds.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: High-value Android text-input architecture; adapt UTF-16/codepoint conversion carefully.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: The source supports SDL/winit event shapes; Android Java IME bridge must provide exact cursor/selection semantics.
- RustChunk implementation mapping: The source supports SDL/winit event shapes; Android Java IME bridge must provide exact cursor/selection semantics.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 798. `crates/horizon-client-render/src/poi_sections.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `46fe1ca0d1c25ea551ba3da4b5b7982283daa896`
- Size: 5640 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Context int/float provider evaluator
- Dependencies / cross-references: Depends on authoritative chunk state, block facts/shape occlusion, fluids, dimensions and world update lifecycle.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Evaluates data-driven context_int_provider/context_float_provider values used for fuel/brewing timing.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Useful data-provider pattern; target codecs and tables need rebuild.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Provider JSON structure and supported conditions may differ for 1.20.1.
- RustChunk implementation mapping: Provider JSON structure and supported conditions may differ for 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 799. `crates/horizon-client-render/src/post_chain.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `107e4f40928137e97d63240a4de6c8049c6eca34`
- Size: 84939 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Screen shader
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: Defines the module's data model and functions described in the purpose field, with version-specific constants/resources and local regression cases.
- Findings: Samples/composites GUI/screen draws using atlas and per-screen colour.
- RustChunk relevance: Potentially useful as a reference subsystem; adopt only when required by RustChunk's 1.20.1 scope.
- Decision: Shader helper; adapt to RustChunk's GUI pipeline.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Texture coordinate and alpha conventions must match HUD/UI vertex data.
- RustChunk implementation mapping: Texture coordinate and alpha conventions must match HUD/UI vertex data.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### 800. `crates/horizon-client-render/src/post_chain.wgsl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `69d54ec6820530e90dc4c5de12b2bda2169c2ea8`
- Size: 4911 bytes (upstream Git blob size).
- Coverage: Complete source blob fetched from the pinned commit and inspected; tests, where present, were not executed.
- Purpose: Screen/widget rendering and text fields
- Dependencies / cross-references: Depends on renderer/game state, versioned registries or resource data as indicated by the module; verify all callers before integration.
- Implementation / logic: GPU shader/include source; defines the stage's data layout, sampling, colour/depth or compute operations. Its bindings and entry points are specific to the renderer pipeline.
- Findings: Provides draw-call representation, text measurement/clipping, scrolling labels, buttons and editable text fields.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: High-value UI framework reference; rebuild font and widget semantics for 1.20.1.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Java string/UTF-16 indexing and input/IME differences are important; some draw primitives depend on newer resource IDs.
- RustChunk implementation mapping: Java string/UTF-16 indexing and input/IME differences are important; some draw primitives depend on newer resource IDs.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

## Completion audit — 621–800

- Exactly 180 sequential entries were added (621–800); all paths were unique against the previous 620 entries and against this batch.
- Each entry has the same 15 required fields as the earlier audit ledger.
- SHA and byte size were taken from the pinned Starling git tree; every selected source file was fetched during this review. Large files were inspected by file structure/docs and relevant code sections; no test command was run.
- Six already-audited upstream paths encountered after entry 620 were skipped instead of being duplicated.
