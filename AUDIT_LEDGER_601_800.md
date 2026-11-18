# Starling → RustChunk Audit Ledger — Entries 601–620

- Source repository: `starling-client/starling`.
- Source ref: `5432370d573642041585c69b40fb045b27133488` (pinned snapshot; do not follow the moving default branch).
- Target project: RustChunk, Minecraft Java 1.20.1-first audit.
- Selection rule: next 20 unique source paths after entry 600; paths already covered by entries 1–600 were skipped, not duplicated.
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
- Decision: **Highly adaptable formatting helper; tighten and differential-test parsing.**
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
- Decision: **Reusable only as a local discriminator; do not use as persistent identity.**
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
- Decision: **Adapt the compositor smoke-test pattern; do not treat it as Android renderer validation.**
- 1.20.1 implications: A 1.20.1 client needs the same presentation correctness but must also handle Android surface loss/recreation, lifecycle pause/resume, GLES/Vulkan device capabilities and actual game frame targets.
- Follow-up checks: Run on desktop and Android separately; test surface lost/outdated, zero and huge dimensions, orientation changes, app background/foreground and frame pacing.
- RustChunk implementation mapping: Candidate test/example under a renderer crate; keep it independent from world, chunk and protocol code.
- Evidence required before adoption: Captured frames or pixel comparisons, resize/lifecycle test logs and device-specific GPU validation; none were run in this audit.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 604. `crates/horizon-client-render/examples/pack_load_bench.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `1bca9ce16a8fd8e34607246cb8487e7b36bb5f89`
- Size: 15027 bytes; 436 lines.
- Coverage: Complete file reviewed (lines 1–436).
- Purpose: Builds deterministic synthetic resource-pack fixtures and times pack overlay, model parsing, sprite processing, texture atlas and cache-related work.
- Dependencies / cross-references: Exercises resource-pack ZIP generation, JSON/model fixtures and renderer resource-loading paths.
- Implementation / logic: Generates controlled packs with namespace, HD textures, models and OptiFine-style connected-texture data; runs several load/toggle/cache stages and reports medians over a small number of samples.
- Findings: Fixtures are synthetic and do not represent a complete 1.20.1 vanilla or modded pack. The custom ZIP writer uses test-oriented shortcuts (including zero CRC handling); scale is caller-controlled and large values can cause excessive allocations or exceed archive-entry count assumptions.
- RustChunk relevance: A benchmark harness can expose resource-loading regressions, but its measurements cannot be used as real-world load-time or Android memory claims.
- Decision: **Adapt the benchmark structure; replace fixtures and validation with real 1.20.1 assets.**
- 1.20.1 implications: Use the 1.20.1 client resource-pack formats, texture dimensions, model variants, namespace overrides and actual mod packs. Android storage and RAM constraints require separate measurements.
- Follow-up checks: Compare synthetic and real packs, validate ZIP CRC/limits, bound CLI scale, measure cold/warm caches and run on the target ARM device.
- RustChunk implementation mapping: Add a standalone benchmark harness for RustChunk's resource-pack pipeline after the parser and atlas implementations stabilize.
- Evidence required before adoption: Repeatable benchmark output, real resource-pack corpus, validated archive integrity and device memory/latency traces; none were run here.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 605. `crates/horizon-client-render/src/android_controls.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `27a7179f72783855d7a45449abee5d3a2bdb43b2`
- Size: 115065 bytes; 2974 lines.
- Coverage: Complete file reviewed (lines 1–2974).
- Purpose: Implements the Android touchscreen control overlay: movement controls, virtual buttons, tap/crosshair interaction, gesture state, HUD drawing and conversion to game actions.
- Dependencies / cross-references: Uses the `horizon-client-input` action/key abstractions, settings/layout definitions, block/item/entity facts and renderer/HUD primitives. Several helper types are declared in adjacent Android control modules.
- Implementation / logic: Tracks touches and pointer ownership; supports short taps, long holds, camera drag, joystick and D-pad movement, double-tap crouch, sprint toggle, shield and item-dependent use/aim behavior. It generates visual button geometry and routes events to the game input layer.
- Findings: This is a large Horizon-specific mobile UX layer, not Minecraft's game-rule or protocol implementation. Some tap behaviors are selected through hard-coded item/block identities, and camera-ray helpers rely on sensible camera/aspect inputs. The source contains tests, but they were not executed.
- RustChunk relevance: Potentially valuable for RustChunk's Android usability and for solving input ergonomics, but it should remain a separately scoped mobile UI subsystem so it does not contaminate the vanilla 1.20.1 engine core.
- Decision: **Useful optional Android UX reference; rebuild and validate against RustChunk's input model.**
- 1.20.1 implications: Preserve 1.20.1 interaction semantics, controls and item use rules. Re-check item identities and actions against the 1.20.1 registry rather than importing newer hard-coded tables.
- Follow-up checks: Test multiple fingers, touch cancellation, pointer handoff, inventory transitions, item changes during hold, portrait/landscape, camera FOV and real device input latency.
- RustChunk implementation mapping: Candidate crate/module `android-controls`, depending on a version-neutral input action interface and RustChunk's own 1.20.1 item/block classification.
- Evidence required before adoption: Golden gesture tests, action traces, no stuck key-up states, and hands-on Galaxy A22 tests at multiple orientations; none run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 606. `crates/horizon-client-render/src/android_controls/custom.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `b7f6c2b4674865893967346ac99f13e3444e9a53`
- Size: 25024 bytes; 633 lines.
- Coverage: Complete file reviewed (lines 1–633).
- Purpose: Defines customizable touchscreen buttons and the custom GoyControl/MCPE input-binding behavior.
- Dependencies / cross-references: Builds on Android touch layout settings, key names, game action tokens and keyboard/mouse input dispatch.
- Implementation / logic: Validates up to three bindings for a custom button, manages press/release and latching across touch pointers, supports button labels and custom tokens, and exposes inventory utility actions.
- Findings: GoyControl and MCPE modes are Starling/Horizon product choices, not vanilla Minecraft 1.20.1 formats. Binding ownership and release behavior are correctness-critical; duplicate or overlapping action tokens must not leave a key held.
- RustChunk relevance: Useful if RustChunk is to offer editable mobile controls, but it does not help establish core Minecraft protocol or Forge compatibility.
- Decision: **Optional feature architecture; do not port as part of the core 1.20.1 engine.**
- 1.20.1 implications: The action list and supported hardware bindings must be derived from RustChunk's Android input backend and the target game's available actions.
- Follow-up checks: Test simultaneous pointers, two buttons bound to one action, cancel/lift ordering, latches, mode switches, keyboard shortcuts and saving/restoring custom bindings.
- RustChunk implementation mapping: An optional button-binding layer above the shared input action interface; keep key down/up ownership explicit and reference-counted where needed.
- Evidence required before adoption: Deterministic multi-touch event tests and Android hardware/IME validation; source tests were only read, not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 607. `crates/horizon-client-render/src/android_controls/editor.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `861113a33be451577064ff53c463de3ac625c51f`
- Size: 37286 bytes; 999 lines.
- Coverage: Complete file reviewed (lines 1–999).
- Purpose: Provides the touchscreen layout editor: move/resize controls, choose bindings, edit labels, undo/redo and manage saved profiles.
- Dependencies / cross-references: Consumes layout/settings definitions, custom binding logic, keyboard events and UI dialog/render helpers.
- Implementation / logic: Maintains an editable copy of the active layout, captures key input, manipulates button placement/scale, saves or cancels edits and provides profile/preset operations.
- Findings: This is UI/product functionality, not game compatibility. Character capture maps a limited ASCII/US-style set rather than arbitrary Unicode/IME input; save errors are not consistently surfaced to the user in every event path.
- RustChunk relevance: Could reduce the friction of configuring mobile controls, but is lower priority than getting RustChunk's game loop, world and renderer working.
- Decision: **Optional editor reference; defer until the Android control layer exists.**
- 1.20.1 implications: Target 1.20.1 does not define this editor. RustChunk would own the UX, file schema, international keyboard support and migration.
- Follow-up checks: Test undo/redo boundaries, cancel restoration, failed saves, duplicate profiles, key capture on non-US keyboards, IME composition and rotation while editing.
- RustChunk implementation mapping: Keep in `android-controls/editor` as an optional presentation component separated from persisted settings and input dispatch.
- Evidence required before adoption: UI interaction tests and Android IME/rotation tests; no such execution was performed in this review.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 608. `crates/horizon-client-render/src/android_controls/layout.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `95b539b048f22a5c7f46d4f7d4dea506d08b3c74`
- Size: 86349 bytes; 2326 lines.
- Coverage: Complete file reviewed (lines 1–2326).
- Purpose: Defines and persists touchscreen control layouts, control styles, placement, presets and profile migration.
- Dependencies / cross-references: Shared types for `android_controls.rs`, `custom.rs` and `editor.rs`; reads/writes Horizon touch profile files.
- Implementation / logic: Provides mode/style enums, clamped placements, layout sanitization, migration from older 10/12-control layouts to a 14-slot layout, custom-button validation, per-profile directories and separate MCPE/GoyControl presets.
- Findings: The schema and profile names are Horizon-specific. Sanitization and migration are useful patterns, but the preset list, action catalog, default placement and on-disk paths are not vanilla Minecraft contracts. User profile data must not be silently discarded on parse/save errors.
- RustChunk relevance: Potentially reusable for RustChunk's mobile control configurator, but not a prerequisite for the game engine. Porting the whole schema would create unnecessary coupling to another launcher's UX.
- Decision: **Reuse validation/migration patterns only; define a RustChunk-owned settings schema.**
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
- Decision: **High-value architecture; regenerate the complete data bundle from Minecraft 1.20.1.**
- 1.20.1 implications: Build the registry, tags, facts, default states, biome/resource registries and client assets against exact 1.20.1 artifacts and DataVersion. Never mix 26.3 IDs or resources with 1.20.1.
- Follow-up checks: Trace bundle generation and consumers, validate required tags and registry bounds, test pack-only reloads, server registry updates and data-version mismatch rejection.
- RustChunk implementation mapping: Split RustChunk's `game-data-1_20_1` from reloadable renderer assets, each with explicit version identity.
- Evidence required before adoption: Reproducible extraction from pinned 1.20.1 client/server artifacts, complete registry diffs, pack reload tests and asset integrity checks; not performed here.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 610. `crates/horizon-client-render/src/atlas_sources.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `4ab90033c7e921617008475980e828382b9ca303`
- Size: 17705 bytes; 497 lines.
- Coverage: Complete file reviewed (lines 1–497).
- Purpose: Implements resource-pack atlas sprite source decoding and generation for directory, single, filter, unstitch and paletted-permutation sources.
- Dependencies / cross-references: Uses JSON codecs, pack path validation, image/texture handling and sprite atlas inputs from the renderer.
- Implementation / logic: Resolves namespaces and source paths, preserves source order, applies filters/removals, slices source textures into regions and generates palette variants before atlas packing.
- Findings: The general source pipeline is reusable, but codec details and resource paths must be checked against the 1.20.1 `SpriteSourceList`. Numeric and image-dimension inputs are not universally range/finite-checked at this layer; image buffer shape assumptions need defensive validation.
- RustChunk relevance: A resource-pack atlas pipeline is essential for faithful textures and mod packs, including resource-pack overrides. This is one of the more reusable renderer subsystems once version codecs are corrected.
- Decision: **Strong reusable algorithmic reference; revalidate codecs and harden untrusted pack inputs.**
- 1.20.1 implications: Use 1.20.1 atlas JSON and sprite source semantics; do not inherit later palette or namespace behavior without a version oracle.
- Follow-up checks: Differential-test each source type, ordering, duplicates, malformed paths, huge dimensions, non-finite coordinates, palette mismatch and pack priority.
- RustChunk implementation mapping: Implement in RustChunk's renderer asset pipeline behind a versioned 1.20.1 sprite-source codec.
- Evidence required before adoption: Java 1.20.1 atlas output comparison and hostile resource-pack tests; source-level tests were not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 611. `crates/horizon-client-render/src/audio.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `8c4b0ed8fc401932a4b30a3d614be44abccc246b`
- Size: 75037 bytes; 1916 lines.
- Coverage: Complete file reviewed (lines 1–1916).
- Purpose: Implements an audio engine with decoded sample caching, asynchronous decode, channel/source management, spatial attenuation/panning, device handling and event playback.
- Dependencies / cross-references: Uses Symphonia/Vorbis decoding, cpal output, mixer state, sound event data, game listener/world state and platform-specific audio behavior.
- Implementation / logic: Decodes and caches samples, schedules playback on mixer channels, handles pitch/gain/loop/positional sources, computes distance and stereo behavior, detects device changes and supports disabled/virtual output modes. It also includes duplicate-sound suppression and split-screen listener handling.
- Findings: This is a substantial replacement for the usual OpenAL route, not a small renderer helper. Audio parity depends on 1.20.1 sound categories, events, random selection, attenuation, streaming/music, subtitles, listener updates, sample conversion and device behavior. Code comments state specific OpenAL comparisons, but no test or audio output was executed here.
- RustChunk relevance: Can inform a future native audio backend and performance strategy, especially if RustChunk targets Android. It should not precede a working game loop and a validated baseline audio path.
- Decision: **Architectural reference only; defer a full audio-engine replacement until core gameplay works.**
- 1.20.1 implications: Rebuild the sound event catalog and exact 1.20.1 sound semantics; Android routing, latency, pause/resume and hardware differences need target-device validation.
- Follow-up checks: Compare waveforms/levels with Java 1.20.1, test multi-channel loads, streaming, loops, device unplug/replug, sample-rate conversion, suspend/resume and Android routes.
- RustChunk implementation mapping: Separate `audio-decode`, `audio-mixer` and platform output backend; keep game sound events and listener integration versioned.
- Evidence required before adoption: Measured output/latency, reference audio comparisons, stress/memory tests and real Galaxy A22 testing; none run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 612. `crates/horizon-client-render/src/auth.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `1752be1b9f3544fd98791352cb027aae254d789c`
- Size: 35613 bytes; 911 lines.
- Coverage: Complete file reviewed (lines 1–911).
- Purpose: Handles authenticated account sessions, session-file handoff, Minecraft Services discovery, server join/verify, account status and secure chat profile key acquisition.
- Dependencies / cross-references: Uses serde JSON, ureq HTTP, `horizon_protocol` UUID/RSA/secure-chat types, ban notice parsing and skin/profile services.
- Implementation / logic: Loads short-lived private-permission session files and deletes them after reading, stores bearer tokens in memory, discovers service endpoints, joins/verifies servers, retries temporary unavailable joins once and fetches/rotates signed chat keys.
- Findings: Security hygiene is strong in intent: tokens are hidden from Debug and not written/logged, and session files are age/permission checked and deleted. `parse_instant` is handwritten and checks broad month/day ranges but not real calendar dates (e.g. February 31); malformed dates may be accepted. UUID parsing is permissive about hyphen placement. Provider routing includes Ely.by, which is not part of vanilla account behavior.
- RustChunk relevance: Online-mode server support requires correct auth/session behavior, but it is sensitive and independent from offline/local gameplay. Keep it isolated from rendering and avoid persisting secrets.
- Decision: **Security-sensitive reference; reimplement against 1.20.1 services and harden date/UUID parsing.**
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
- Decision: **Reusable parsing/UI pattern; verify exact 1.20.1 moderation schema and localization keys.**
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
- Decision: **Valuable RTL reference; validate with a full Arabic/Persian Unicode corpus and 1.20.1 behavior.**
- 1.20.1 implications: Target 1.20.1 font assets, bidi rules and ICU/Java shaping behavior must be matched. Do not assume all current Rust Unicode crate versions reproduce the JDK's exact Unicode data.
- Follow-up checks: Test Persian letters, Arabic letters/marks, lam-alef, mixed Latin/numbers, punctuation mirroring, formatting spans, emoji, control characters and truncated lines.
- RustChunk implementation mapping: Keep a version-neutral text layout stage in the UI/render text crate, separated from protocol chat component parsing.
- Evidence required before adoption: Golden screenshots and expected visual-order strings from Java 1.20.1 for Arabic, Persian and mixed-direction messages; not executed.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 615. `crates/horizon-client-render/src/biome.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `a04f3471cdd485d9cafc71b97409e0db4d4e64f3`
- Size: 27273 bytes; 714 lines.
- Coverage: Complete file reviewed (lines 1–714).
- Purpose: Loads biome definitions, spawn settings, colours, features and datapack replacements; computes temperature, grass tint and spatially blended biome colours.
- Dependencies / cross-references: Uses worldgen biome JSON, prepared grass/foliage/dry-foliage colormaps, legacy random/simplex noise and world block positions.
- Implementation / logic: Builds builtin biome tables, overlays datapack additions/replacements while preserving builtin IDs, produces sorted registry order, samples colour maps, applies grass modifiers and computes a 6×6×6 Gaussian interpolation around biome quart cells.
- Findings: The data tables and IDs are from the pinned newer Starling version and cannot be reused for 1.20.1. `parse_entries` casts u64 values to u32 without range checks, `parse_costs` accepts defaulted numeric values without finite/range validation, and `get` assumes the biome table is non-empty.
- RustChunk relevance: Biome data directly affects chunk colour, temperature, precipitation, mobs and world generation. The replacement/registry architecture is more valuable than the 26.3 catalog itself.
- Decision: **High-value architecture and algorithms; rebuild 1.20.1 biome registry/data and harden parsing.**
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
- Decision: **Adapt the batched storage algorithm only behind a separate, explicitly strict bulk-edit API.**
- 1.20.1 implications: Match 1.20.1 chunk section/heightmap/light behavior and server/client update semantics. Any block entity, fluid, POI, neighbor or entity-path side effects must be explicitly added by the caller where required.
- Follow-up checks: Differential-test overlap order, negative coordinates, tall dimensions, y-boundaries, tracked states, heightmaps, lighting, block entities and extreme integer inputs.
- RustChunk implementation mapping: Separate `World::edit_runs_strict` from normal block mutation; return a complete change summary and require caller-declared side-effect handling.
- Evidence required before adoption: Java 1.20.1 fill/replace parity, chunk serialization tests, height/light correctness and stress benchmarks; source tests not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 617. `crates/horizon-client-render/src/block_entities.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `eb418a9ec7515b8750374f9aaf9ee0e805ebe961`
- Size: 29456 bytes; 737 lines.
- Coverage: Complete file reviewed (lines 1–737).
- Purpose: Stores block entities, their attachments and indexes, and maintains the ordered live/pending block-entity ticker list.
- Dependencies / cross-references: Uses block entity variants, fast hash maps/sets, world attachments and block-state ticker classifications.
- Implementation / logic: Models pending ticker promotion, rebind on replacement, cancellation/removal, chunk-load order, `take`/`put` during ticks, mod entity indexes and structure/test bounding-box lists. `chunk_map_order` emulates fastutil open-addressing iteration order.
- Findings: Correctness depends on observable Java fastutil iteration order and exact ticker lifecycle. The implementation uses its own token/slot bookkeeping and mod-specific attachments/containers; a state transition to non-ticking expects the caller's block-entity removal path. Runtime IDs and registries are target-version-specific.
- RustChunk relevance: Highly relevant to server/client world fidelity: block entities and ticker ordering affect chests, furnaces, signs, containers, animations and modded blocks.
- Decision: **Strong lifecycle reference; port only after reproducing 1.20.1 load/tick/update ordering.**
- 1.20.1 implications: Build the ticking table from 1.20.1 block registry/state definitions and verify `LevelChunk`/`Level` lifecycle against target sources; mod attachment APIs are optional future scope.
- Follow-up checks: Differential-test chunk load ordering, insert/replace/remove/take/put/retain, state changes, slots freed during iteration, chunk unload/reload and attachment persistence.
- RustChunk implementation mapping: Core world-state `BlockEntityStore` plus separate ticker scheduler; keep renderer indexes as derived caches, not the authoritative map.
- Evidence required before adoption: Java 1.20.1 ticker-order fixtures and save/load/update parity; no runtime tests run here.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 618. `crates/horizon-client-render/src/block_entity_anim.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `acef6e51f1ed80772fbb445203fdb9c848fb1d0b`
- Size: 35311 bytes; 1027 lines.
- Coverage: Complete file reviewed (lines 1–1027).
- Purpose: Tracks client-side block-entity animation state for chests, enchanting books, shulkers, bells, conduits, skulls, decorated pots and End gateways.
- Dependencies / cross-references: Uses block entity renderer `BlockAnim`, core trigonometry/random sources, world positions, entities and block registry state.
- Implementation / logic: Implements per-kind tick state, trigger events, interpolation values, shulker collision/push bounds, bell raid resonance, conduit activation/particles, skull animation, decorated-pot wobble and End gateway beam/cooldown state.
- Findings: The general separation between tick state and render extraction is useful. Several ticker classifications include later additions (copper chests and other newer states) and must be filtered to 1.20.1. Random source selection, event timing and side effects must match the client vs integrated server responsibilities exactly.
- RustChunk relevance: Useful for smooth block-entity visuals and accurate client ticking; animation correctness is separate from block entity storage and should consume authoritative game events/state.
- Decision: **Adapt per-entity state/interpolation patterns; rebuild the ticker catalog and event behavior for 1.20.1.**
- 1.20.1 implications: Use only target-version block entities and properties. Validate that the client does not simulate server-only world effects and that random streams do not desynchronize unrelated features.
- Follow-up checks: Compare tick-by-tick chests/books/shulkers/bells/conduits/skulls/gateways with Java 1.20.1, including event timing, negative coordinates, entity queries and pause/resume.
- RustChunk implementation mapping: Client render-state animation module keyed by block position; feed from a distinct 1.20.1 block entity ticker/event pipeline.
- Evidence required before adoption: Scripted deterministic input traces and Java reference animation frames; no tests or gameplay run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 619. `crates/horizon-client-render/src/block_entity_cem.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `7048c97bea6013fc03109c6c6bb32f1d0e8d3ca0`
- Size: 25735 bytes; 630 lines.
- Coverage: Complete file reviewed (lines 1–630).
- Purpose: Loads OptiFine/EMF custom entity model `.jem` files for block entities and reshapes baked vanilla model parts.
- Dependencies / cross-references: Uses OptiFine CEM file discovery/JSON parsing, entity model bones and `block_entity_render::ModelPart` structures.
- Implementation / logic: Maps supported CEM names to renderer model parts, applies replacement/attachment geometry, scales bones, handles shared files across banner/conduit/pot/book models and chooses custom atlas textures.
- Findings: This is an optional resource-pack compatibility layer, not a vanilla Minecraft requirement. It supports a finite hard-coded set of model names; some renderer targets and model names reflect newer blocks. Custom model loading is user-supplied input and must be hardened against oversized/deep models and invalid numeric geometry.
- RustChunk relevance: Potential future compatibility feature for OptiFine/EMF resource packs, but far below the priority of vanilla 1.20.1 model and texture rendering.
- Decision: **Optional compatibility feature; defer and scope to the subset of formats actually required.**
- 1.20.1 implications: Do not let CEM determine base block-entity models. Rebuild vanilla model parts from 1.20.1 resources and layer CEM replacement only after the renderer is correct.
- Follow-up checks: Test malformed/deep `.jem`, unknown parts, attach vs replace, shared files, texture lookup, model recursion and 1.20.1 supported CEM names.
- RustChunk implementation mapping: Separate `resource-pack-compat/cem` extension that transforms already-loaded RustChunk base models.
- Evidence required before adoption: OptiFine/EMF compatibility corpus, 1.20.1 pack comparison and recursion/size safety tests; not run.
- Completion state: Full artifact reviewed and recorded; no build, test suite, server session, APK or Android device run was executed in this audit.

### 620. `crates/horizon-client-render/src/block_entity_render.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA:  `b6b8c188042308d9220a6fad3b5d6bd17af618f3`
- Size: 152652 bytes; 4035 lines.
- Coverage: Complete file reviewed (lines 1–4035).
- Purpose: Implements block-entity and associated special-model rendering: chest variants, shulkers, banners, skulls, bells, decorated pots, conduits, books, signs, End portal/gateway beams and newer copper golem/chest models.
- Dependencies / cross-references: Consumes baked entity-model data, model-part geometry, animation state, world/light queries, sprites/emissive layers, sign text, CEM shapes, break overlays and optional static GPU geometry caching.
- Implementation / logic: Classifies block states into render kinds, transforms and draws model parts, calculates diffuse/packed light, text/beam/portal geometry, applies animation, supports CEM replacement and maintains per-section CPU snapshots for static models plus a sign-text cache.
- Findings: This is a large renderer module and its model catalog, state classification, textures, block list and render behavior are pinned to the newer 26.3 snapshot. Features such as copper chests/statues are not part of 1.20.1. Static caching is sensitive to section membership, block state, light, atlas revision, pose and sprite alpha; the companion cache tests are in a separate `block_entity_render_tests.rs` file. No renderer test was executed.
- RustChunk relevance: Important reference for block-entity visuals and potential chunk/render cache design, but importing it wholesale before a 1.20.1 world/renderer baseline would bring substantial version-specific state and assets.
- Decision: **Use as architectural/rendering reference; rebuild the target-version model catalog and keep the cache as a separately validated optimization.**
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
