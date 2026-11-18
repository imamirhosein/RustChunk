# Starling → RustChunk Audit Ledger — Entries 801–1000

- Source repository: `starling-client/starling`.
- Pinned source snapshot: `5432370d573642041585c69b40fb045b27133488` (source metadata comes from this exact tree, not the moving default branch).
- Batch: exactly 200 new, unique paths after entry 800, selected in deterministic path order; 4 previously audited paths were skipped during selection.
- Target: Minecraft Java 1.20.1-first RustChunk audit.
- Inspection: 130 text files were read fully; files longer than 400 lines were read in consecutive sections no larger than 400 lines. 70 PNG files could not be read as binary through the UTF-8-only GitHub connector and are explicitly metadata-only, not content-reviewed.
- Execution: no tests, build, benchmark, APK, server connection, Forge client or Android device execution is claimed by this audit.

### 801. `crates/horizon-client-render/src/post_glsl.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ca8f3efccb892c76a2a8e313bbc72afcbc15d8a5`
- Size: 12083 bytes; 316 lines
- Coverage: Complete text file reviewed, lines 1–316; long files were read in consecutive sections of at most 400 lines.
- Purpose: Converts selected post-effect GLSL into WGSL-compatible source.
- Dependencies / cross-references: Shader-pack resources, Naga parsing, sampler bindings.
- Implementation / logic: Transforms supported syntax/macros/uniforms before WGSL validation.
- Findings: Not a complete GLSL implementation; unsupported extensions and implicit semantics remain.
- RustChunk relevance: Optional shader-pack bridge; never a baseline render dependency.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/post_glsl.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 802. `crates/horizon-client-render/src/potent_sulfur.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `68bd32501b754ac927bc676b1a4072a5f1456758`
- Size: 4745 bytes; 119 lines
- Coverage: Complete text file reviewed, lines 1–119; long files were read in consecutive sections of at most 400 lines.
- Purpose: Implements a newer Potent Sulfur effect/content path.
- Dependencies / cross-references: Newer registry, item/block/effect data.
- Implementation / logic: Maps this feature into renderer-facing state.
- Findings: Not a vanilla 1.20.1 feature or registry entry.
- RustChunk relevance: Exclude from 1.20.1 baseline.
- Decision: Exclude from the Minecraft 1.20.1 baseline; reconsider only if target-version evidence or an explicit extension requires it.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/potent_sulfur.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.nd targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 803. `crates/horizon-client-render/src/preedit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `64ac1fb18562379cd5283f91b5324ceb90207d76`
- Size: 11350 bytes; 317 lines
- Coverage: Complete text file reviewed, lines 1–317; long files were read in consecutive sections of at most 400 lines.
- Purpose: Tracks IME pre-edit text and cursor/composition state.
- Dependencies / cross-references: Platform text events, UTF-8 strings and GUI widgets.
- Implementation / logic: Maintains composing text and commits/cancels composition.
- Findings: UTF-8 offsets, graphemes and Java UTF-16 indices are not interchangeable.
- RustChunk relevance: Useful Android text-input reference; port with explicit Unicode index mapping.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-r
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.ender/src/preedit.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 804. `crates/horizon-client-render/src/providers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6dccea6782ae431a77222048e5a05aad5f0665a6`
- Size: 4406 bytes; 122 lines
- Coverage: Complete text file reviewed, lines 1–122; long files were read in consecutive sections of at most 400 lines.
- Purpose: Decodes font/resource provider definitions.
- Dependencies / cross-references: Resource-pack JSON codecs and font/image assets.
- Implementation / logic: Maps provider JSON to bitmap/space/Unihex/TTF-style providers.
- Findings: Provider variants and constraints are version-specific; bound dimensions and image sizes.
- RustChunk relevance: Rebuild codec set from 1.20.1; preserve generic glyph-provider interface.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapp
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.ing: RustChunk module corresponding to `crates/horizon-client-render/src/providers.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 805. `crates/horizon-client-render/src/screen.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1497b36fa06a16305af57adefb1daae942a6880d`
- Size: 3048 bytes; 69 lines
- Coverage: Complete text file reviewed, lines 1–69; long files were read in consecutive sections of at most 400 lines.
- Purpose: WGSL shader for screen-space GUI primitives.
- Dependencies / cross-references: GUI draw list, texture bindings, screen uniform.
- Implementation / logic: Transforms GUI vertices/UVs and samples textures.
- Findings: Must match bind layout, alpha and scaling exactly.
- RustChunk relevance: Generic renderer pattern; validate separately on Android GPU.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks w
- RustChunk implementation mapping: here applicable; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/screen.wgsl`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 806. `crates/horizon-client-render/src/screen_draw.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `727b7ca1e1c11cba5d2c6b6a067bf13f95bdb61a`
- Size: 45157 bytes; 1268 lines
- Coverage: Complete text file reviewed, lines 1–1268; long files were read in consecutive sections of at most 400 lines.
- Purpose: Defines GUI draw commands and screen-space composition helpers.
- Dependencies / cross-references: Text/font, HUD atlas, screen transforms and clipping.
- Implementation / logic: Emits ordered sprite/fill/text/blit/scissor/tooltip draw operations.
- Findings: Ordering and nested scissor are observable; batch only across equivalent state.
- RustChunk relevance: High-value GUI foundation; rebuild Java 1.20.1 geometry and scaling.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: 
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/screen_draw.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 807. `crates/horizon-client-render/src/screen_effects.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `81addea96c7cca12db7fd52d86e6959f4dc4a1ef`
- Size: 16265 bytes; 434 lines
- Coverage: Complete text file reviewed, lines 1–434; long files were read in consecutive sections of at most 400 lines.
- Purpose: Builds full-screen visual overlays such as fire, portal, underwater and spyglass.
- Dependencies / cross-references: Camera, status effects, textures and screen shader.
- Implementation / logic: Selects effect quads, transforms, opacity and timing.
- Findings: Effect list and ordering are version-specific.
- RustChunk relevance: Reuse composition architecture; regenerate effect catalogue from 1.20.1.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping:
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant. RustChunk module corresponding to `crates/horizon-client-render/src/screen_effects.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 808. `crates/horizon-client-render/src/sculk.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1ce8a1a8e8d9e51be6d6492de9ecc45a854b76c2`
- Size: 30555 bytes; 858 lines
- Coverage: Complete text file reviewed, lines 1–858; long files were read in consecutive sections of at most 400 lines.
- Purpose: Maps sculk-related block and vibration visual state.
- Dependencies / cross-references: Block tags/states, vibration events, Warden/sculk events.
- Implementation / logic: Produces sculk effect/render state and associated events.
- Findings: Post-1.20.1 states may be included; data table must be filtered.
- RustChunk relevance: Adapt 1.20.1 sculk subset only; keep game logic separate from rendering.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device chec
- RustChunk implementation mapping: ks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 809. `crates/horizon-client-render/src/section_mask.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7a0e2e52894383644164a9acacdaf39ac867de08`
- Size: 4933 bytes; 164 lines
- Coverage: Complete text file reviewed, lines 1–164; long files were read in consecutive sections of at most 400 lines.
- Purpose: Compact 256-bit section visibility/occupancy mask.
- Dependencies / cross-references: Section indexing, mesher and chunk scheduler.
- Implementation / logic: Bit-set insert/remove/contains/iteration over four words.
- Findings: Index boundary and shift behavior require explicit bounds.
- RustChunk relevance: Small reusable utility; align index to target chunk height.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/sculk.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 810. `crates/horizon-client-render/src/server_terrain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `377bd68f7bccf108af6d619159e58578d17e9f02`
- Size: 37115 bytes; 997 lines
- Coverage: Complete text file reviewed, lines 1–997; long files were read in consecutive sections of at most 400 lines.
- Purpose: Connects server-provided far terrain tiles to renderer caches.
- Dependencies / cross-references: Network extension messages, far tiles and GPU uploads.
- Implementation / logic: Decodes/updates distant terrain separately from resident chunks.
- Findings: Horizon-specific extension, not vanilla protocol; stale tiles must never replace nearby real chunks.
- RustChunk relevance: Optional far-terrain layer only after near-chunk correctness.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fix
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/section_mask.rs`, or an isolated optional/tooling module if the feature is not vanilla core.tures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 811. `crates/horizon-client-render/src/settings.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1ccdbe942ab8bcc9f1873c7f6703bc67347abc09`
- Size: 39912 bytes; 1059 lines
- Coverage: Complete text file reviewed, lines 1–1059; long files were read in consecutive sections of at most 400 lines.
- Purpose: Defines client/renderer settings, ranges, persistence and UI.
- Dependencies / cross-references: Options file, graphics settings, controls and platform paths.
- Implementation / logic: Loads, validates, edits and serializes many option values.
- Findings: Large set of LOD/FSR/frame-gen/native-pack settings are Horizon-only.
- RustChunk relevance: Reuse versioned settings pattern; split vanilla options from RustChunk extensions.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: 26.3 tables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 812. `crates/horizon-client-render/src/shader_options.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04a7705fd73420c86f1eca084bddb5c63d57bac3`
- Size: 13530 bytes; 391 lines
- Coverage: Complete text file reviewed, lines 1–391; long files were read in consecutive sections of at most 400 lines.
- Purpose: Parses and displays shader-pack option definitions.
- Dependencies / cross-references: Shader metadata, typed option values and UI widgets.
- Implementation / logic: Stores option defaults/ranges and routes changes to shader settings.
- Findings: User pack metadata is untrusted and can request invalid ranges/rebuilds.
- RustChunk relevance: Optional shader compatibility feature; not required for Vanilla.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Cre
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/settings.rs`, or an isolated optional/tooling module if the feature is not vanilla core.ate Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 813. `crates/horizon-client-render/src/shader_pack_list.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3e88014f0131d8067aedc53a45f2c3dfdc28f8dd`
- Size: 11758 bytes; 327 lines
- Coverage: Complete text file reviewed, lines 1–327; long files were read in consecutive sections of at most 400 lines.
- Purpose: Discovers/selects shader packs and maintains pack list state.
- Dependencies / cross-references: Filesystem roots, metadata, settings and shader loader.
- Implementation / logic: Enumerates packs and tracks selected/incompatible state.
- Findings: File traversal, symlinks and pack compatibility need hardening.
- RustChunk relevance: Defer; optional shader-pack manager.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: t import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 814. `crates/horizon-client-render/src/signs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8c24b32928b25f8a0cc5c56149d78e0480e379ad`
- Size: 10690 bytes; 297 lines
- Coverage: Complete text file reviewed, lines 1–297; long files were read in consecutive sections of at most 400 lines.
- Purpose: Builds sign block-entity text and render data.
- Dependencies / cross-references: Sign NBT/state, front/back text, font and block-entity renderer.
- Implementation / logic: Resolves lines/styles/glow and creates face/text draw data.
- Findings: NBT/components and side fields must match 1.20.1 exactly.
- RustChunk relevance: High-value 1.20.1 block-entity render feature; rebuild target NBT schema.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: 
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 815. `crates/horizon-client-render/src/skins.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `209ee8c18198c0616289817cfdb3b0eebbc1e939`
- Size: 60707 bytes; 1508 lines
- Coverage: Complete text file reviewed, lines 1–1508; long files were read in consecutive sections of at most 400 lines.
- Purpose: Resolves, decodes, caches and applies player/entity skins.
- Dependencies / cross-references: Profile services, skin URLs, texture decoding and player model.
- Implementation / logic: Loads skin data, updates player textures and manages cache.
- Findings: Concurrency, stale profiles and unbounded cache growth risk; later metadata may differ.
- RustChunk relevance: Optional account appearance layer; offline fallback must work.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/signs.rs`, or an isolated optional/tooling module if the feature is not vanilla core.case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 816. `crates/horizon-client-render/src/sound_engine.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `45cd4a2f0cf6e2650696600e6d18c5986c96143b`
- Size: 57469 bytes; 1638 lines
- Coverage: Complete text file reviewed, lines 1–1638; long files were read in consecutive sections of at most 400 lines.
- Purpose: Runtime mixer, source/channel tracking, spatial playback and audio output.
- Dependencies / cross-references: Sound event registry, codec decode, listener state and platform backend.
- Implementation / logic: Maintains sources, loops, pitch/gain, distance attenuation, pause/stop and output device state.
- Findings: Large replacement for conventional audio backend; source/event catalogue includes later-version data.
- RustChunk relevance: Architecture reference only; defer full backend replacement until core loop stable.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: 
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 817. `crates/horizon-client-render/src/sound_events.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9b06ff310f437676197e1174755d2e75faeccdb3`
- Size: 20140 bytes; 553 lines
- Coverage: Complete text file reviewed, lines 1–553; long files were read in consecutive sections of at most 400 lines.
- Purpose: Loads sound-event names, subtitles and weighted sound variants.
- Dependencies / cross-references: Pack sound JSON, language strings and audio assets.
- Implementation / logic: Builds registry of event variants and referenced sound resources.
- Findings: Snapshot's event ids/data are 26.3-specific.
- RustChunk relevance: Regenerate exact 1.20.1 sounds.json and subtitle keys.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and 
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/sound_engine.rs`, or an isolated optional/tooling module if the feature is not vanilla core.device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 818. `crates/horizon-client-render/src/sound_preview.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `babb802dd33d5e78cdaabc8683e42783d41d6b5c`
- Size: 7283 bytes; 179 lines
- Coverage: Complete text file reviewed, lines 1–179; long files were read in consecutive sections of at most 400 lines.
- Purpose: Plays selected sounds for settings/pack preview.
- Dependencies / cross-references: Sound engine, event registry and GUI lifecycle.
- Implementation / logic: Starts/stops a temporary preview source.
- Findings: Rapid selection/screen close can leave source lifetime bugs.
- RustChunk relevance: Optional UI utility; keep preview token separate from gameplay audio.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: nchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 819. `crates/horizon-client-render/src/spans.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8bd8cab0e92f185a018139aaa689cc827fa48be8`
- Size: 24032 bytes; 687 lines
- Coverage: Complete text file reviewed, lines 1–687; long files were read in consecutive sections of at most 400 lines.
- Purpose: Represents styled text spans and run operations.
- Dependencies / cross-references: Text components, font layout and GUI draw commands.
- Implementation / logic: Stores character/style runs and provides merge/slice/iteration.
- Findings: Unicode indexing and bidi reordering can break formatting if byte offsets are used.
- RustChunk relevance: Useful generic text model; implement Java-compatible styles and Unicode mapping.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/sound_preview.rs`, or an isolated optional/tooling module if the feature is not vanilla core., edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 820. `crates/horizon-client-render/src/split_layout.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6331d37caef8c7f7078d8d5346ace5e3586947c9`
- Size: 28342 bytes; 758 lines
- Coverage: Complete text file reviewed, lines 1–758; long files were read in consecutive sections of at most 400 lines.
- Purpose: Computes split-screen viewports and layout rectangles.
- Dependencies / cross-references: Display size, orientation, player count and split settings.
- Implementation / logic: Partitions output area into per-player screen rectangles.
- Findings: Split-screen is Horizon-only and not first-release mobile scope.
- RustChunk relevance: Defer; optional multi-view rendering feature.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: ng 26.3 tables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 821. `crates/horizon-client-render/src/split_map.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f958a0c2337379803c4af70799b696b0b958bd7c`
- Size: 6529 bytes; 166 lines
- Coverage: Complete text file reviewed, lines 1–166; long files were read in consecutive sections of at most 400 lines.
- Purpose: Maps players/views to split-screen regions.
- Dependencies / cross-references: Split layout and per-player view state.
- Implementation / logic: Associates player indices with allocated viewports.
- Findings: Product-specific; reassignment/removal handling requires lifecycle tests.
- RustChunk relevance: Defer until single-player render/input is stable.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: 
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/split_layout.rs`, or an isolated optional/tooling module if the feature is not vanilla core.Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 822. `crates/horizon-client-render/src/split_profiles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7f0945be47ff16719ac9230daa1e62d0e49c074b`
- Size: 24936 bytes; 691 lines
- Coverage: Complete text file reviewed, lines 1–691; long files were read in consecutive sections of at most 400 lines.
- Purpose: Defines split-screen profiles and presets.
- Dependencies / cross-references: Split settings schema, input slots and persistent state.
- Implementation / logic: Selects/migrates profile settings for multiple local players.
- Findings: Not vanilla Minecraft behavior; preset data is Horizon-specific.
- RustChunk relevance: Reuse profile-store patterns only; no need to import schema.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: t import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/split_profiles.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 823. `crates/horizon-client-render/src/split_profiles/store.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0c90007f90a1dd336ce1d18f70bad2b2cee01ed0`
- Size: 2984 bytes; 90 lines
- Coverage: Complete text file reviewed, lines 1–90; long files were read in consecutive sections of at most 400 lines.
- Purpose: Persists split-screen profile data.
- Dependencies / cross-references: Serde/JSON and filesystem storage.
- Implementation / logic: Loads/saves profile records.
- Findings: Atomic save/corrupt data handling must be checked.
- RustChunk relevance: Reusable storage pattern, not the feature itself.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit..
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/split_profiles/store.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 824. `crates/horizon-client-render/src/split_ui.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `91af553b876a7b5b8611307bea57e9c4d51d5083`
- Size: 62018 bytes; 1630 lines
- Coverage: Complete text file reviewed, lines 1–1630; long files were read in consecutive sections of at most 400 lines.
- Purpose: Routes UI interactions and draw layout for split-screen players.
- Dependencies / cross-references: Split layout/profiles, HUD and per-player input focus.
- Implementation / logic: Composes per-view GUI and focus state.
- Findings: Optional product layer with complicated multi-focus semantics.
- RustChunk relevance: Defer from first RustChunk release.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/split_ui.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 825. `crates/horizon-client-render/src/sprite_cache.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c440e6ccf946f6f70114cd8afc454c9b4c1c8af1`
- Size: 12310 bytes; 345 lines
- Coverage: Complete text file reviewed, lines 1–345; long files were read in consecutive sections of at most 400 lines.
- Purpose: Caches processed sprite/image data for resource packs.
- Dependencies / cross-references: Pack paths, file metadata, image decoder and atlas builder.
- Implementation / logic: Stores/loads derived sprite representations using metadata cache keys.
- Findings: Path/size/mtime can miss same-size content replacement; cache must be versioned and bounded.
- RustChunk relevance: Useful optimization only with stronger content-hash invalidation.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.his is not proof of 1.20.1 compatibility.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/sprite_cache.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 826. `crates/horizon-client-render/src/sprites.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `264c816c0e0d59cc9027340e8091dab3861b973a`
- Size: 56438 bytes; 1319 lines
- Coverage: Complete text file reviewed, lines 1–1319; long files were read in consecutive sections of at most 400 lines.
- Purpose: Loads textures, atlas sprite sources, animations and UV mappings.
- Dependencies / cross-references: Atlas JSON, images, pack overlays and GPU atlas.
- Implementation / logic: Decodes sprite source types and builds atlas/animation frames.
- Findings: Codec variants, alpha and image bounds are version-specific.
- RustChunk relevance: Core renderer feature; rebuild from 1.20.1 atlas assets and source codecs.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.ot proof of 1.20.1 compatibility.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/sprites.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 827. `crates/horizon-client-render/src/static_entity.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bb7c58f8387772fdc5ca3d1e7f5950bb3bb3e7da`
- Size: 1056 bytes; 21 lines
- Coverage: Complete text file reviewed, lines 1–21; long files were read in consecutive sections of at most 400 lines.
- Purpose: Transforms static entity vertices using camera-relative precision.
- Dependencies / cross-references: Entity vertex buffers and view-projection uniforms.
- Implementation / logic: Applies position transforms while retaining large-world precision.
- Findings: Coordinate-space mismatch can cause double camera offsets.
- RustChunk relevance: Reusable shader pattern; validate RustChunk coordinate origin.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/static_entity.wgsl`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 828. `crates/horizon-client-render/src/streaming.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f89a6f4438d4e9bade145672a0c3e69bdd31ebb9`
- Size: 173677 bytes; 4045 lines
- Coverage: Complete text file reviewed, lines 1–4045; long files were read in consecutive sections of at most 400 lines.
- Purpose: Schedules chunk generation, lighting, mesh compilation, updates, unload and worker results.
- Dependencies / cross-references: Chunk/section state, async workers, mesher, lighting and resource snapshots.
- Implementation / logic: Uses bounded queues, distance priority/reranking, per-column sequences, nonblocking mailbox, stale-result rejection, neighbor-light readiness and mesh pooling.
- Findings: A stale worker result or LOD substitute must never replace nearby authoritative chunk data; near residency policy is critical.
- RustChunk relevance: Highest-value architectural reference for RustChunk chunk gaps/void-under-feet issue; port concepts, not code wholesale.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/streaming.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 829. `crates/horizon-client-render/src/structure_block.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0951f2e5628365b9e74aa63d5170947a54c26dcf`
- Size: 17509 bytes; 496 lines
- Coverage: Complete text file reviewed, lines 1–496; long files were read in consecutive sections of at most 400 lines.
- Purpose: Implements Structure Block state/data and actions.
- Dependencies / cross-references: Structure NBT/templates, block states and UI controls.
- Implementation / logic: Loads/saves structure data and handles placement/scan/save modes.
- Findings: Template/rotation semantics must match 1.20.1.
- RustChunk relevance: Useful lower-priority vanilla feature; target NBT and placement behavior must be rebuilt.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/structure_block.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 830. `crates/horizon-client-render/src/system_stats.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `761762896ac28b9c4b03bfdf2d670c92c7d6b688`
- Size: 13188 bytes; 341 lines
- Coverage: Complete text file reviewed, lines 1–341; long files were read in consecutive sections of at most 400 lines.
- Purpose: Collects process/system/graphics stats for diagnostics.
- Dependencies / cross-references: Android/Linux procfs/sysfs and renderer adapter data.
- Implementation / logic: Reads CPU/memory/GPU details where available.
- Findings: OEM/kernel nodes may be missing or permission-restricted; not a stable gameplay dependency.
- RustChunk relevance: Optional diagnostics service only.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/system_stats.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 831. `crates/horizon-client-render/src/tab_list.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6e5b580ffe6a5f6fce638890263bdda66dc55eb7`
- Size: 23564 bytes; 689 lines
- Coverage: Complete text file reviewed, lines 1–689; long files were read in consecutive sections of at most 400 lines.
- Purpose: Renders multiplayer player list and row status/skin details.
- Dependencies / cross-references: Player-info packets, teams, latency, skins and HUD atlas.
- Implementation / logic: Sorts and renders player rows, skins and latency indicators.
- Findings: Later profile fields may not exist in 1.20.1; packet model must be target-version-specific.
- RustChunk relevance: Useful multiplayer UI architecture; rebuild from 1.20.1 player-info data.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/tab_list.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 832. `crates/horizon-client-render/src/telemetry.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `89fd1f324e319fdd9fbaa2d6199db53d39c4c859`
- Size: 21065 bytes; 585 lines
- Coverage: Complete text file reviewed, lines 1–585; long files were read in consecutive sections of at most 400 lines.
- Purpose: Collects and scrubs optional diagnostic telemetry.
- Dependencies / cross-references: Settings/consent, event schemas and platform metadata.
- Implementation / logic: Filters and serializes telemetry events.
- Findings: Identifiers, paths and tokens must be scrubbed; telemetry is not gameplay-critical.
- RustChunk relevance: Do not port by default; separate opt-in diagnostics and privacy review.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/telemetry.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 833. `crates/horizon-client-render/src/terrain.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `905b4547fceacb851201bc8489a6f45c5bec5ba6`
- Size: 39721 bytes; 796 lines
- Coverage: Complete text file reviewed, lines 1–796; long files were read in consecutive sections of at most 400 lines.
- Purpose: Main terrain shader for lighting, fog, sky, cutout/translucency, emissive and HDR.
- Dependencies / cross-references: WGSL pipeline, sprites/lightmap, terrain vertices, fog/HDR uniforms and sky fan.
- Implementation / logic: Transforms terrain data and computes light, visibility, fog, emission, sky/sunrise and fragment output.
- Findings: Tightly bound to newer renderer pipelines/26.3 assets and shader features; desktop shader success would not prove Mali/GLES compatibility.
- RustChunk relevance: Architecture/algorithm reference; build a minimal 1.20.1 terrain path first.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/terrain.wgsl`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.e; on-device evidence for Android/GPU/audio paths.

### 834. `crates/horizon-client-render/src/test_block.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1de50dcc802bd936b3ab6899d4b0077484eaa74c`
- Size: 6566 bytes; 202 lines
- Coverage: Complete text file reviewed, lines 1–202; long files were read in consecutive sections of at most 400 lines.
- Purpose: Implements Test Block state/actions.
- Dependencies / cross-references: Test block entity data and test framework.
- Implementation / logic: Models test-block modes and controls.
- Findings: Confirm target fields against 1.20.1 source before reuse.
- RustChunk relevance: Optional test tooling feature; lower runtime priority.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/test_block.rs`, or an isolated option
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.al/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 835. `crates/horizon-client-render/src/test_instance_block.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `69239bca4b3fc9079b7e77b4bf085c432d3623c4`
- Size: 8978 bytes; 265 lines
- Coverage: Complete text file reviewed, lines 1–265; long files were read in consecutive sections of at most 400 lines.
- Purpose: Implements Test Instance block behavior.
- Dependencies / cross-references: Modern test framework and test instance block entity.
- Implementation / logic: Stores and dispatches instance/test state.
- Findings: Feature is newer than 1.20.1.
- RustChunk relevance: Exclude from 1.20.1 baseline.
- Decision: Exclude from the Minecraft 1.20.1 baseline; reconsider only if target-version evidence or an explicit extension requires it.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/test_instance
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant._block.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 836. `crates/horizon-client-render/src/testgen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cded7ae47a1b44aef1a8f8db8b35593c39ad2a39`
- Size: 6396 bytes; 186 lines
- Coverage: Complete text file reviewed, lines 1–186; long files were read in consecutive sections of at most 400 lines.
- Purpose: Generates source/fixtures for test harnesses.
- Dependencies / cross-references: Declarative test inputs and Rust generation.
- Implementation / logic: Converts test descriptions to generated code/fixtures.
- Findings: Generated output must be deterministic and version-pinned.
- RustChunk relevance: Reusable test tooling only.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Can
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.didate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/testgen.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 837. `crates/horizon-client-render/src/title_edition.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e27cf042f7d89c6c77eee61f64b5bc446408dc94`
- Size: 17645 bytes; 490 lines
- Coverage: Complete text file reviewed, lines 1–490; long files were read in consecutive sections of at most 400 lines.
- Purpose: Builds title edition/build branding.
- Dependencies / cross-references: Client metadata, title UI and translations.
- Implementation / logic: Selects label text for title screen.
- Findings: Branding is Horizon-specific, not a Minecraft feature.
- RustChunk relevance: Do not port Starling branding; RustChunk owns its own identity.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed 
- RustChunk implementation mapping: in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/title_edition.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 838. `crates/horizon-client-render/src/trims.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ea835284028b2bd59fb9ca0562d028366c2852aa`
- Size: 12989 bytes; 355 lines
- Coverage: Complete text file reviewed, lines 1–355; long files were read in consecutive sections of at most 400 lines.
- Purpose: Loads armor trims and builds material/pattern render layers.
- Dependencies / cross-references: Item/model registry, trim assets and atlas.
- Implementation / logic: Maps trim patterns/materials to armor texture layers.
- Findings: Armor trims exist in 1.20 but this snapshot's tables/assets may be newer.
- RustChunk relevance: Useful 1.20.1 feature after rebuilding exact trim registries.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/trim
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.s.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 839. `crates/horizon-client-render/src/ui.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `241d51c5a485caa28fe1effafee087dd6113eb34`
- Size: 81677 bytes; 2363 lines
- Coverage: Complete text file reviewed, lines 1–2363; long files were read in consecutive sections of at most 400 lines.
- Purpose: Top-level GUI screen routing, event dispatch and draw orchestration.
- Dependencies / cross-references: Input, screen models, menus, HUD, chat, inventory and rendering.
- Implementation / logic: Routes key/mouse/text/scroll actions, updates screens and produces draw commands.
- Findings: Large Horizon-specific UI integration; bulk port introduces coupling and lifecycle/input-focus bugs.
- RustChunk relevance: Use as navigation reference; implement minimal 1.20.1 screens independently.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create 
- RustChunk implementation mapping: Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 840. `crates/horizon-client-render/src/vault.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ca5c08a7f527f2d26a239fba77b50884af08fbbb`
- Size: 26576 bytes; 747 lines
- Coverage: Complete text file reviewed, lines 1–747; long files were read in consecutive sections of at most 400 lines.
- Purpose: Implements Vault block-entity key/reward lifecycle.
- Dependencies / cross-references: Trial Chamber blocks, loot tables, player keys and NBT.
- Implementation / logic: Tracks players, reward state, cooldown and loot ejection.
- Findings: Trial Chamber/Vault are 1.21-era features, not 1.20.1.
- RustChunk relevance: Exclude from 1.20.1 baseline.
- Decision: Exclude from the Minecraft 1.20.1 baseline; reconsider only if target-version evidence or an explicit extension requires it.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applica
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/ui.rs`, or an isolated optional/tooling module if the feature is not vanilla core.ble; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 841. `crates/horizon-client-render/src/weather.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f15ae028dcacb2e910c976da49cff00aef9707bb`
- Size: 19713 bytes; 564 lines
- Coverage: Complete text file reviewed, lines 1–564; long files were read in consecutive sections of at most 400 lines.
- Purpose: Builds weather/cloud/rain/snow visual state.
- Dependencies / cross-references: World time, biome climate, precipitation and renderer.
- Implementation / logic: Computes weather strength and render descriptors.
- Findings: Climate and biome data in snapshot are version-specific.
- RustChunk relevance: Rebuild Java 1.20.1 weather transitions and biome rules.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Crea
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/vault.rs`, or an isolated optional/tooling module if the feature is not vanilla core.te Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 842. `crates/horizon-client-render/src/widgets.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0e6482b5f3f41abe58dae255afd35b841b1907d0`
- Size: 40243 bytes; 1200 lines
- Coverage: Complete text file reviewed, lines 1–1200; long files were read in consecutive sections of at most 400 lines.
- Purpose: Provides reusable UI widgets for menus/settings.
- Dependencies / cross-references: Screen draw lists, mouse/key/text input and font metrics.
- Implementation / logic: Buttons, sliders, toggles, lists and text controls.
- Findings: Widget style/focus/IME behavior is Horizon-specific.
- RustChunk relevance: Reuse generic widget patterns; implement RustChunk screen-specific UX.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: 1.20.1 sources; do not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/widgets.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 843. `crates/horizon-client-render/src/world_post.wgsl`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d3e6f7296336745cd915a03f82eca4439069f93a`
- Size: 8018 bytes; 165 lines
- Coverage: Complete text file reviewed, lines 1–165; long files were read in consecutive sections of at most 400 lines.
- Purpose: Post-processing shader for world color/depth outputs.
- Dependencies / cross-references: World render targets, HDR/post-chain and final composition.
- Implementation / logic: Samples/composites post effects and final color.
- Findings: Coupled to Horizon HDR/FSR/native shader stack.
- RustChunk relevance: Optional after baseline renderer; no dependency for Vanilla game.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit. not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/world_post.wgsl`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 844. `crates/horizon-client-render/src/worldborder.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa2380d74bfe8bc0fe7ece28f5896c762f18e7f4`
- Size: 9107 bytes; 266 lines
- Coverage: Complete text file reviewed, lines 1–266; long files were read in consecutive sections of at most 400 lines.
- Purpose: Computes and draws Minecraft world-border behavior.
- Dependencies / cross-references: World border center/size/warnings and renderer.
- Implementation / logic: Interpolates size/center and calculates distance/visual warning.
- Findings: Core logic is vanilla but interpolation/limits must match 1.20.1.
- RustChunk relevance: Reusable vanilla behavior; compare exact formulas.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/worldborder.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 845. `crates/horizon-client-render/src/zip.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a2fc726100ef54e36675aa8ddefb6faf3bdc59d7`
- Size: 29431 bytes; 703 lines
- Coverage: Complete text file reviewed, lines 1–703; long files were read in consecutive sections of at most 400 lines.
- Purpose: Reads ZIP resource-pack containers and file entries.
- Dependencies / cross-references: ZIP central directory, Stored/Deflate decoders and overlay filesystem.
- Implementation / logic: Parses entries and reads requested file streams.
- Findings: Archive-bomb limits, CRC and normalized paths must be consistently enforced across methods.
- RustChunk relevance: Important security-sensitive pack subsystem; harden before use.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.e; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/src/zip.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 846. `crates/horizon-client-render/tests/sunrise_shader.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7dbd3a093e3f27124c1618e72bfa968c105f012c`
- Size: 7404 bytes; 189 lines
- Coverage: Complete text file reviewed, lines 1–189; long files were read in consecutive sections of at most 400 lines.
- Purpose: Validates extracted sunrise WGSL and contains ignored host-GPU parity test.
- Dependencies / cross-references: Naga, WGSL source, wgpu and sunrise-fan geometry.
- Implementation / logic: Checks shader syntax and optionally compares optimized sector lookup against full fan search.
- Findings: GPU test is ignored and was not run; host GPU success would not imply Android Mali compatibility.
- RustChunk relevance: Adopt test structure only; no execution was performed.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client-render/tests/sunrise_shader.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 847. `crates/horizon-client/assets/bedrock-touch.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `be48cf9766423707c149cace98501f5a1c86daa3`
- Size: 3852 bytes; 131 lines
- Coverage: Complete text file reviewed, lines 1–131; long files were read in consecutive sections of at most 400 lines.
- Purpose: Provenance manifest for touch icons sourced from Mojang/bedrock-samples.
- Dependencies / cross-references: Pinned upstream repo/revision, per-file SHA/size and license metadata.
- Implementation / logic: Lists icon names, upstream relative paths, SHA-1 and byte count; pins license file hash.
- Findings: Hash pinning doesn't replace checking the actual license text and matching each included asset.
- RustChunk relevance: Keep as provenance only; do not assume reuse rights without license review.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/bedrock-touch.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 848. `crates/horizon-client/assets/horizon/lang/de_de.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `06a62a5fd8217d9d9f6e94b731a506590a1821bf`
- Size: 25803 bytes; 340 lines
- Coverage: Complete text file reviewed, lines 1–340; long files were read in consecutive sections of at most 400 lines.
- Purpose: German Horizon UI translation table.
- Dependencies / cross-references: JSON key/value strings and placeholder formats.
- Implementation / logic: 338 keys; comparison with English base leaves 289 keys missing; shared placeholders matched.
- Findings: Partial localization with substantial English fallback.
- RustChunk relevance: Optional UI localization; not a Java 1.20.1 game language file.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/de_de.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 849. `crates/horizon-client/assets/horizon/lang/en_us.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d6f1112fee20a296db733e2952d640fc462ec322`
- Size: 44704 bytes; 629 lines
- Coverage: Complete text file reviewed, lines 1–629; long files were read in consecutive sections of at most 400 lines.
- Purpose: English base strings for Horizon-specific UI.
- Dependencies / cross-references: JSON strings, placeholders and UI consumers.
- Implementation / logic: 627 keys used as base for locale comparison.
- Findings: Reference for Horizon UI only, not for Minecraft 1.20.1 vanilla strings.
- RustChunk relevance: Use only as a seed for RustChunk-owned UI keys.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/en_us.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 850. `crates/horizon-client/assets/horizon/lang/es_es.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4aa7a80cbb3cfcc228a51c5753ce1f523fb8b126`
- Size: 15680 bytes; 228 lines
- Coverage: Complete text file reviewed, lines 1–228; long files were read in consecutive sections of at most 400 lines.
- Purpose: Spanish Horizon UI translations.
- Dependencies / cross-references: JSON strings and placeholder patterns.
- Implementation / logic: 226 keys; 401 absent vs English; shared placeholder formats match.
- Findings: Incomplete localization; fallback will be common.
- RustChunk relevance: Optional UI locale; not a vanilla game language asset.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/es_es.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 851. `crates/horizon-client/assets/horizon/lang/fr_fr.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `34600a6008894330d4644bdd53907c23ea1dfad4`
- Size: 31656 bytes; 408 lines
- Coverage: Complete text file reviewed, lines 1–408; long files were read in consecutive sections of at most 400 lines.
- Purpose: French Horizon UI translations.
- Dependencies / cross-references: JSON strings and placeholder patterns.
- Implementation / logic: 406 keys; 221 absent vs English; shared placeholders match.
- Findings: Partial localization; long labels may affect layout.
- RustChunk relevance: Optional UI locale; keep separate from 1.20.1 language data.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/fr_fr.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 852. `crates/horizon-client/assets/horizon/lang/ms_my.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `db2e0b14ea04242cb1201b03a1208057e6216094`
- Size: 44853 bytes; 629 lines
- Coverage: Complete text file reviewed, lines 1–629; long files were read in consecutive sections of at most 400 lines.
- Purpose: Malay Horizon UI translations.
- Dependencies / cross-references: JSON strings and placeholder patterns.
- Implementation / logic: 627 keys; key count matches English and shared placeholder sequences match; semantic quality was not human-reviewed.
- Findings: Coverage appears complete by key count, but key parity is not proof of translation quality.
- RustChunk relevance: Optional locale; require human review before release.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/ms_my.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 853. `crates/horizon-client/assets/horizon/lang/pl_pl.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d5455a3ffd6881905353f11227ec8cbf0de3b196`
- Size: 26418 bytes; 358 lines
- Coverage: Complete text file reviewed, lines 1–358; long files were read in consecutive sections of at most 400 lines.
- Purpose: Polish Horizon UI translations.
- Dependencies / cross-references: JSON strings and placeholder patterns.
- Implementation / logic: 356 keys; 271 absent vs English; shared placeholders match.
- Findings: Partial localization; inflection and long UI text need QA.
- RustChunk relevance: Optional locale; target version strings are not included.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/pl_pl.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.nd targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 854. `crates/horizon-client/assets/horizon/lang/pt_br.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c674a2cf5b0c859be59ffa67ea8ca91d2554a9ac`
- Size: 11224 bytes; 144 lines
- Coverage: Complete text file reviewed, lines 1–144; long files were read in consecutive sections of at most 400 lines.
- Purpose: Brazilian Portuguese Horizon UI translations.
- Dependencies / cross-references: JSON strings and placeholder patterns.
- Implementation / logic: 142 keys; 485 absent vs English; high fallback reliance.
- Findings: Very incomplete localization.
- RustChunk relevance: Optional locale; fill strings before presenting as complete.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lan
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.g/pt_br.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 855. `crates/horizon-client/assets/horizon/lang/ru_ru.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9d4067feb3480c7f0fbd0ae241514bd5be9e4747`
- Size: 32316 bytes; 358 lines
- Coverage: Complete text file reviewed, lines 1–358; long files were read in consecutive sections of at most 400 lines.
- Purpose: Russian Horizon UI translations.
- Dependencies / cross-references: JSON strings and placeholder patterns.
- Implementation / logic: 356 keys; 271 absent vs English; shared placeholders match.
- Findings: Partial localization; long labels/plurals need QA.
- RustChunk relevance: Optional locale.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module correspo
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.nding to `crates/horizon-client/assets/horizon/lang/ru_ru.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 856. `crates/horizon-client/assets/horizon/lang/tr_tr.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `979eb3593c79ab74ebd018e5a9870896244bb558`
- Size: 16915 bytes; 223 lines
- Coverage: Complete text file reviewed, lines 1–223; long files were read in consecutive sections of at most 400 lines.
- Purpose: Turkish Horizon UI translations.
- Dependencies / cross-references: JSON strings and placeholder patterns.
- Implementation / logic: 221 keys; 406 absent vs English; shared placeholders match.
- Findings: Incomplete localization; Turkish I/İ casing needs correct case mapping.
- RustChunk relevance: Optional locale.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Ca
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.ndidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/tr_tr.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 857. `crates/horizon-client/assets/horizon/lang/uk_ua.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6571ce05c05ee1aae6d92ec6fadc0f5984b817ec`
- Size: 44733 bytes; 460 lines
- Coverage: Complete text file reviewed, lines 1–460; long files were read in consecutive sections of at most 400 lines.
- Purpose: Ukrainian Horizon UI translations.
- Dependencies / cross-references: JSON strings and placeholder patterns.
- Implementation / logic: 458 keys; 169 absent vs English; shared placeholders match.
- Findings: Partial localization and plural QA needed.
- RustChunk relevance: Optional locale.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/uk_ua.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 858. `crates/horizon-client/assets/horizon/lang/zh_cn.json`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a771828ac8eeab0f16b67e5861366cd8b1920609`
- Size: 25029 bytes; 358 lines
- Coverage: Complete text file reviewed, lines 1–358; long files were read in consecutive sections of at most 400 lines.
- Purpose: Simplified Chinese Horizon UI translations.
- Dependencies / cross-references: JSON strings, CJK glyphs and placeholders.
- Implementation / logic: 356 keys; 271 absent vs English; shared placeholders match.
- Findings: Partial localization; CJK font fallback/width are relevant.
- RustChunk relevance: Optional locale; not game 1.20.1 translation content.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/assets/horizon/lang/zh_cn.json`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 859. `crates/horizon-client/assets/touch/attack.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `79faf6d241ae5ee171bae6688b4f128130b26e9c`
- Size: 602 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates attack touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 860. `crates/horizon-client/assets/touch/attack_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `38d3a7829ae1c095282b33af094e383041cbc755`
- Size: 575 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 861. `crates/horizon-client/assets/touch/camera.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `824d477a99cb3219f85e6db735658212aa8f0a49`
- Size: 564 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates camera touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 862. `crates/horizon-client/assets/touch/camera_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3fea6e9509ffb522c36fe3a16029d38203e908f9`
- Size: 583 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 863. `crates/horizon-client/assets/touch/chat.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ba3c6f61e32ad1afcbac8a427f9c8f863de5b6e`
- Size: 550 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates chat touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 864. `crates/horizon-client/assets/touch/chat_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a9ab6257a24fd6e2ec2205fa94bf20b05c559221`
- Size: 535 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 865. `crates/horizon-client/assets/touch/command.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ca8a7a102cce924ff264e318947c1a8b3b650388`
- Size: 616 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates command touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 866. `crates/horizon-client/assets/touch/command_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `386626cf3e69fcf187f30a067aa193eab78dc0bf`
- Size: 624 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 867. `crates/horizon-client/assets/touch/custom_apple.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `34ff5bcfba7ed2f5c0bdfa14f35077e9d97fe5c8`
- Size: 522 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 868. `crates/horizon-client/assets/touch/custom_apple_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b4e36d0dc91749fa7d5219d127fd815606acc041`
- Size: 505 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 869. `crates/horizon-client/assets/touch/custom_blank.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `522651c96acaf48c2a0e94604a38c162ce66ff08`
- Size: 601 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 870. `crates/horizon-client/assets/touch/custom_check.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e580d1c226adf447773355a1b2465280cffa897d`
- Size: 598 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 871. `crates/horizon-client/assets/touch/custom_check_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ee5ec45cd3cd8c0927a7a23a715544dfd328ce0b`
- Size: 579 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 872. `crates/horizon-client/assets/touch/custom_cross.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `647c6928c1610cd8894e361fd1d55a4968c5ae47`
- Size: 570 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 873. `crates/horizon-client/assets/touch/custom_cross_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4e8bd7d6f405e1e98589a9d1a724d4b57a3db9c`
- Size: 566 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 874. `crates/horizon-client/assets/touch/custom_crown.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b8ba35dab5368346a27d14384a8b8b19412b4213`
- Size: 449 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 875. `crates/horizon-client/assets/touch/custom_crown_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `26bba25125bfe5dad894101d8224344b6817f4fc`
- Size: 460 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 876. `crates/horizon-client/assets/touch/custom_fire.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f88fc5c7bc9fd8324cdda295bcf401b3f5aa4fb9`
- Size: 487 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 877. `crates/horizon-client/assets/touch/custom_fire_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `856cbc999e82a9054819fbb19922ccc8946a4cfd`
- Size: 496 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 878. `crates/horizon-client/assets/touch/custom_flag.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f0b6a99efd9ec0f3438d57032e2b9ed13d45d045`
- Size: 509 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 879. `crates/horizon-client/assets/touch/custom_flag_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f0616274106927cb8f7f6aff82046129931320bf`
- Size: 504 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 880. `crates/horizon-client/assets/touch/custom_heart.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0fbeade3f9bb4d8f4e5ddfce89e1f4c06941e6c1`
- Size: 540 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 881. `crates/horizon-client/assets/touch/custom_heart_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `443c392d8f2ca214cd3637a5e563b5a226ab7a96`
- Size: 536 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 882. `crates/horizon-client/assets/touch/custom_home.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9415e3d52f0335fec61d20ef50c551a8253a32e8`
- Size: 513 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 883. `crates/horizon-client/assets/touch/custom_home_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `201aa268f0a344d80c707bca013f52148efd4446`
- Size: 521 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 884. `crates/horizon-client/assets/touch/custom_music.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ab86a70d58c889b5b7b317575048a5e39801f2fb`
- Size: 449 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 885. `crates/horizon-client/assets/touch/custom_music_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f196a8d868bd2bc63fb07adb05e60412cdcb9832`
- Size: 467 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 886. `crates/horizon-client/assets/touch/custom_pickaxe.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `724b4e760953fe8170e3a033349902ee6bbd62db`
- Size: 543 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 887. `crates/horizon-client/assets/touch/custom_pickaxe_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2ef6b2696bdb7fc66b3ffa03b2617b84424338bc`
- Size: 541 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 888. `crates/horizon-client/assets/touch/custom_plus.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `415511aa7c2af05b7c16af721d8773672b5509a4`
- Size: 584 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 889. `crates/horizon-client/assets/touch/custom_plus_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7f7cc6d4cbef951080cb9784ed46e26679d47d3f`
- Size: 550 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 890. `crates/horizon-client/assets/touch/custom_potion.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `696db28ebc5fabdd544f35d8bc196f4c6a8f1a0d`
- Size: 532 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 891. `crates/horizon-client/assets/touch/custom_potion_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `01be75a85a46b9447ce405968cb1d7d40dc616ae`
- Size: 511 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 892. `crates/horizon-client/assets/touch/custom_shield.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8a403456a5bde86d5918917848a284b2c87a699e`
- Size: 483 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 893. `crates/horizon-client/assets/touch/custom_shield_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `244f6d16f2f659b3ae14033811d601986d43b907`
- Size: 478 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 894. `crates/horizon-client/assets/touch/custom_skull.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ce5c65f575db8b2df6c6516b853929c0904d51ac`
- Size: 485 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 895. `crates/horizon-client/assets/touch/custom_skull_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `699a6d594e043850c547659e87a8e1023e6bb53d`
- Size: 496 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 896. `crates/horizon-client/assets/touch/custom_star.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `269a0e50fa10f5679648bcb772920c1c20c29330`
- Size: 542 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates custom-control icon, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 897. `crates/horizon-client/assets/touch/custom_star_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aa0b91c2514aaf30b8af14091d618986e5616135`
- Size: 533 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 898. `crates/horizon-client/assets/touch/debug.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `faad2881a263b93865b1780ed141090791d9608f`
- Size: 558 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates debug touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 899. `crates/horizon-client/assets/touch/debug_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0f629fb2140c3f61fad4a9c280632a0bfe001e2d`
- Size: 535 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 900. `crates/horizon-client/assets/touch/dpad_down.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `391a6ed48699db000ab1fc2c1434f4477741b59f`
- Size: 556 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates directional-pad asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 901. `crates/horizon-client/assets/touch/dpad_down_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `70dca324fc172dbbd8cb0a76792e1cfe25a4dabd`
- Size: 567 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 902. `crates/horizon-client/assets/touch/dpad_left.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `11a9c354a9f8ac02c118ffc3a9c5348bbf7a0cb8`
- Size: 576 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates directional-pad asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 903. `crates/horizon-client/assets/touch/dpad_left_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e33fd86923acb98ca87c656fcc11a8353cda2fc8`
- Size: 571 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 904. `crates/horizon-client/assets/touch/dpad_middle.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `522651c96acaf48c2a0e94604a38c162ce66ff08`
- Size: 601 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates directional-pad asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 905. `crates/horizon-client/assets/touch/dpad_right.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `26d43a0159c9f44350e192e0ca878a58a77253d5`
- Size: 611 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates directional-pad asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 906. `crates/horizon-client/assets/touch/dpad_right_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1798cb3315d482b447bd40540bd0a4fa811ba31a`
- Size: 597 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 907. `crates/horizon-client/assets/touch/dpad_up.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `295f1b96e997f22c2a475afc848780cc6c548d71`
- Size: 574 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates directional-pad asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 908. `crates/horizon-client/assets/touch/dpad_up_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0c58a3cb6df9322a0f7affcfe147747862ef043a`
- Size: 574 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 909. `crates/horizon-client/assets/touch/drop.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a0c8530f60c8ad5472770d555dc671ecfe248d6a`
- Size: 569 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates drop touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 910. `crates/horizon-client/assets/touch/drop_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `90d75c4edc457af91fa35fb5eade67e7e306bd70`
- Size: 568 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 911. `crates/horizon-client/assets/touch/inventory.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f106e135a0fc2a32c7861c118620577e40d12439`
- Size: 573 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates inventory touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 912. `crates/horizon-client/assets/touch/inventory_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `718533f2cdbbc44174a1cbf0e3e4506b34bdcd5b`
- Size: 577 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 913. `crates/horizon-client/assets/touch/jump.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5944b88fe417295285ad940b4b67ad4b02ed8f72`
- Size: 588 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates jump touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 914. `crates/horizon-client/assets/touch/jump_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3438df2de9a62ba8ce29ec43c20ea5aeba89eccc`
- Size: 573 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 915. `crates/horizon-client/assets/touch/offhand.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a62a9fca036746ed2499f4d85f393de25647da30`
- Size: 562 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates offhand touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 916. `crates/horizon-client/assets/touch/offhand_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b26cc4223ae05e1e2c4db86f400d8f8e401a2563`
- Size: 560 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 917. `crates/horizon-client/assets/touch/pause.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9624460d13e8922c867eb13444deab85ca8bed7f`
- Size: 504 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pause touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 918. `crates/horizon-client/assets/touch/pause_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c24ef84b1e845af59216b846a6d5d1fe82be8c8f`
- Size: 474 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 919. `crates/horizon-client/assets/touch/pick.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3e06c223b3cc107d099e4428ec62e9b61e8c66c0`
- Size: 600 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pick touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 920. `crates/horizon-client/assets/touch/pick_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ed28e7eff080ed53d3f3bf6f45ca8fee4a3593bb`
- Size: 594 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 921. `crates/horizon-client/assets/touch/players.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eead5ca582b2fd9549a8b3f1bc2e184e0b0fe944`
- Size: 507 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates players touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 922. `crates/horizon-client/assets/touch/players_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fefbc9c72255833f7b2f8ba37e4213da3073ed7f`
- Size: 507 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 923. `crates/horizon-client/assets/touch/sneak.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `61f43780e4a8570e5501dd7aacf7c2155ca72806`
- Size: 555 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates sneak touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 924. `crates/horizon-client/assets/touch/sneak_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4efa8db2b5666cf86b1a93551ac07b085fe5ae22`
- Size: 548 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 925. `crates/horizon-client/assets/touch/stick.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `776a9f780ff1baef7a574abfc157643c0a60d76c`
- Size: 1966 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates joystick base, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 926. `crates/horizon-client/assets/touch/stick_knob.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `522651c96acaf48c2a0e94604a38c162ce66ff08`
- Size: 601 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates joystick knob, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 927. `crates/horizon-client/assets/touch/use.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `047f073865438c7873c5c2dc147530470d008750`
- Size: 542 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates use touch-control asset, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- RustChunk implementation mapping: PNG file bytes and visual inspection, pair
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.ed-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 928. `crates/horizon-client/assets/touch/use_pressed.png`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `748b448fd778374098bda97204700ba9f209ce0b`
- Size: 530 bytes; PNG, line count not applicable
- Coverage: Metadata only: source path, Git blob SHA and byte count; PNG binary/pixel data was not retrieved.
- Purpose: Android touch-control PNG asset; its filename indicates pressed-state variant, but its visual appearance was not inspected.
- Dependencies / cross-references: Path is under the Android touch-control asset directory; exact call site and image dimensions remain unverified.
- Implementation / logic: Only repository tree metadata was read. The connector rejects PNG bytes because it accepts UTF-8 text; no signature, IHDR, dimensions, alpha or pixels were decoded.
- Findings: Visual role, normal/pressed matching, transparency, dimensions and license/provenance remain unverified. Three files share identical blob SHA: custom_blank.png, dpad_middle.png and stick_knob.png.
- RustChunk relevance: May support a future Android control overlay, but inclusion or redistribution must wait for visual and license checks.
- Decision: **Metadata-only; do not approve reuse yet.**
- 1.20.1 implications: Not Minecraft Java 1.20.1 content; optional launcher UI asset only.
- Follow-up checks: Obtain bytes through a binary-capable route; inspect dimensions/alpha/pixels, compare state pairs, identify consumers and verify license.
- RustChunk implementation mapping: Pending visual inspection; if suitable, place under `android-controls/assets` with content hash and source/license metadata.
- Evidence required before adoption: PNG file bytes and visual inspection, paired-state check, consumer trace and provenance/license verification.
- Completion state: NOT FULLY REVIEWED — binary bytes/pixels unavailable through UTF-8-only GitHub connector; source-tree SHA and size recorded only.

### 929. `crates/horizon-client/build.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `de50c33f11df818cef3e1089e106b8cc27cee384`
- Size: 2038 bytes; 38 lines
- Coverage: Complete text file reviewed, lines 1–38; long files were read in consecutive sections of at most 400 lines.
- Purpose: Build hook for embedded desktop resources and platform linker settings.
- Dependencies / cross-references: Cargo build script, compile target and Windows linker.
- Implementation / logic: Embeds icon/resource metadata and applies selected Windows-only link settings.
- Findings: Windows branches do not directly solve Android Gradle/NDK packaging.
- RustChunk relevance: Keep platform build hooks separate; do not transplant desktop flags to Android.
- Decision: Potentially useful as a source/test/architecture reference for the subsystem described above; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: See Findings above; adapt algorithms only after target-version comparison.
- Follow-up checks: Rebuild versioned IDs, registries, packet/data schemas and assets from the exact Minecraft 1.20.1 sources; do not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/build.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 930. `crates/horizon-client/examples/chunk_load_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `47e839faf9d8edd05b90df0a794c1ce34543da6a`
- Size: 11952 bytes; 314 lines
- Coverage: Complete text file reviewed, lines 1–314; long files were read in consecutive sections of at most 400 lines.
- Purpose: Benchmarks chunk generation, decoration, lighting and mesh stages.
- Dependencies / cross-references: Assets, worldgen, chunk model, worker pool and mesher.
- Implementation / logic: Times pipeline stages over controlled worker counts and checksums.
- Findings: Synthetic fixtures/newer asset version and desktop timing are not Galaxy A22 performance evidence.
- RustChunk relevance: Reuse stage-separated harness; rebuild 1.20.1 fixtures.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/chunk_load_bench.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 931. `crates/horizon-client/examples/collision_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `212d6e17211ad0d0026fc1eb81ff98e234d7c1b6`
- Size: 3776 bytes; 104 lines
- Coverage: Complete text file reviewed, lines 1–104; long files were read in consecutive sections of at most 400 lines.
- Purpose: Benchmarks collision/AABB query throughput.
- Dependencies / cross-references: Collision shapes, geometry and deterministic random queries.
- Implementation / logic: Runs synthetic cases and validates a result checksum.
- Findings: Synthetic query mix may not represent real movement/world geometry.
- RustChunk relevance: Reuse benchmark methodology only after collision parity.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.of of 1.20.1 compatibility.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/collision_bench.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 932. `crates/horizon-client/examples/gameplay_decode_profile.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2d2ec4799b4e8d5777356b316be8f84628295b5c`
- Size: 1450 bytes; 34 lines
- Coverage: Complete text file reviewed, lines 1–34; long files were read in consecutive sections of at most 400 lines.
- Purpose: Profiles JSON gameplay-facts decode and typed conversion.
- Dependencies / cross-references: Serde JSON, gameplay resource schema and profile timers.
- Implementation / logic: Separates raw parse from later validation/typed passes.
- Findings: Desktop profile and snapshot schema do not establish Android performance.
- RustChunk relevance: Use as profiling tool only; rebuild target facts schema.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/gameplay_decode_profile.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 933. `crates/horizon-client/examples/gen_light_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `101e2d8136246cc2735514c774f0e98dc6f33fc2`
- Size: 12895 bytes; 352 lines
- Coverage: Complete text file reviewed, lines 1–352; long files were read in consecutive sections of at most 400 lines.
- Purpose: Benchmarks worldgen/decoration/lighting stages.
- Dependencies / cross-references: Worldgen assets, chunk workers and light engine.
- Implementation / logic: Times generation and light stages with optional simulation controls.
- Findings: Newer data and potentially long-running runs; no benchmark was run.
- RustChunk relevance: Use as harness pattern, not as measured claim.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/gen_light_bench.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 934. `crates/horizon-client/examples/json_load_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9397cd89e6afe7293e03bc8a168c7663ef67abe0`
- Size: 4378 bytes; 125 lines
- Coverage: Complete text file reviewed, lines 1–125; long files were read in consecutive sections of at most 400 lines.
- Purpose: Compares JSON load strategies and error parity.
- Dependencies / cross-references: Filesystem read APIs and serde_json.
- Implementation / logic: Benchmarks reader vs byte/text parse and compares failures.
- Findings: I/O benefits depend on Android storage/allocation; no result was run.
- RustChunk relevance: Evaluate only with target fixtures and Android profiling.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/json_load_bench.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 935. `crates/horizon-client/examples/mod_port_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `25e8ab5beb6e6129d4566f22d6af32197ba85b57`
- Size: 13234 bytes; 326 lines
- Coverage: Complete text file reviewed, lines 1–326; long files were read in consecutive sections of at most 400 lines.
- Purpose: Reports current mod/loader compatibility surface.
- Dependencies / cross-references: Modern NeoForge concepts, registries, recipes, loot and tags.
- Implementation / logic: Collects probe data for block/item/recipe/tag support.
- Findings: Targets newer mod API and does not prove Forge 1.20.1 compatibility.
- RustChunk relevance: Future research aid only; no Forge support conclusion.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/mod_port_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 936. `crates/horizon-client/examples/model_library_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c7e6d3e5b7513dbc212d42417e7911802d872d47`
- Size: 5317 bytes; 131 lines
- Coverage: Complete text file reviewed, lines 1–131; long files were read in consecutive sections of at most 400 lines.
- Purpose: Benchmarks model library load and optional allocation counts.
- Dependencies / cross-references: Model catalog, renderer assets and allocator instrumentation.
- Implementation / logic: Loads models repeatedly and reports duration/allocations.
- Findings: Desktop allocation instrumentation isn't an Android baseline.
- RustChunk relevance: Use to locate hot paths after model parity.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/model_library_bench.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 937. `crates/horizon-client/examples/model_states_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2ab1aa033535c5f1785dfa32c31eff1c100e7c5f`
- Size: 10864 bytes; 292 lines
- Coverage: Complete text file reviewed, lines 1–292; long files were read in consecutive sections of at most 400 lines.
- Purpose: Benchmarks model state resolution with synthetic fixture.
- Dependencies / cross-references: Block-state registry and model mapping.
- Implementation / logic: Compares repeated resolution with precomputed facts.
- Findings: Synthetic state corpus and newer registry sizes are not representative of 1.20.1.
- RustChunk relevance: Adapt benchmark method; rebuild target model state table.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/model_states_bench.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 938. `crates/horizon-client/examples/old_world_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3650f2c9cd5eca15af5a599110d90c7d3be8805c`
- Size: 3014 bytes; 81 lines
- Coverage: Complete text file reviewed, lines 1–81; long files were read in consecutive sections of at most 400 lines.
- Purpose: Converts old region/world fixtures for migration tests.
- Dependencies / cross-references: MCA files, NBT chunks and height/blending data.
- Implementation / logic: Reads region chunks and writes transformed region fixtures.
- Findings: Deletes `.mca` files under its destination before writing; destructive outside a disposable directory.
- RustChunk relevance: Fixture-only tool; never run on user worlds.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/old_world_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 939. `crates/horizon-client/examples/portal_scan_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `702ef4d4510b901a77bfdadfcc895a13403d756b`
- Size: 10219 bytes; 262 lines
- Coverage: Complete text file reviewed, lines 1–262; long files were read in consecutive sections of at most 400 lines.
- Purpose: Benchmarks portal section indexing against full entity scan.
- Dependencies / cross-references: Chunk section map, portal data and synthetic entities.
- Implementation / logic: Compares cached index and scan time with checksum.
- Findings: Synthetic workload doesn't predict real portals/player session load.
- RustChunk relevance: Optional optimization if profiling proves portal scan hot.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/portal_scan_bench.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage. paths.

### 940. `crates/horizon-client/examples/probe/ambient_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3fd507902130d8c6b121e0ecf5d1ea40b66528af`
- Size: 7189 bytes; 185 lines
- Coverage: Complete text file reviewed, lines 1–185; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for ambient environment output against Java oracle.
- Dependencies / cross-references: World time, biome/environment state and renderer/audio parameters.
- Implementation / logic: Serializes controlled ambient values.
- Findings: Current resource data must be regenerated for 1.20.1.
- RustChunk relevance: Use as a parity harness template.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/ambient_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage. evidence for Android/GPU/audio paths.

### 941. `crates/horizon-client/examples/probe/arrow_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fbc6a82355a13fe2ebb5198fd6e96741cef5fe19`
- Size: 17132 bytes; 371 lines
- Coverage: Complete text file reviewed, lines 1–371; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for arrow projectile movement, collision and events.
- Dependencies / cross-references: Projectile physics, collision world, RNG and particles/sounds.
- Implementation / logic: Ticks scripted seeded arrows and logs projectile-visible state.
- Findings: Item/effect and entity data may be newer; collision order is observable.
- RustChunk relevance: Rebuild with 1.20.1 arrow/tipped-arrow semantics.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/arrow_probe.rs`, or an
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant. isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 942. `crates/horizon-client/examples/probe/audio_attributes_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4bbb3a23992b6a3805c99c3d8fdfdf4a3f5a0a5a`
- Size: 2892 bytes; 85 lines
- Coverage: Complete text file reviewed, lines 1–85; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for sound attributes and default parameters.
- Dependencies / cross-references: Sound engine/channel/category config.
- Implementation / logic: Serializes selected audio source settings.
- Findings: Sound groups/defaults depend on target version.
- RustChunk relevance: Useful deterministic audio test input.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/audio_attributes_fixture.rs`, or an isolated optional/tooling mo
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.dule if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 943. `crates/horizon-client/examples/probe/audio_common/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b177eba540f9a633ec4071166d42b84a2737b3f`
- Size: 5628 bytes; 176 lines
- Coverage: Complete text file reviewed, lines 1–176; long files were read in consecutive sections of at most 400 lines.
- Purpose: Shared fake audio backend and audio result formatting.
- Dependencies / cross-references: SoundEngine source/channel API and deterministic logger.
- Implementation / logic: Implements fake backend, descriptions and flushing for audio fixture tools.
- Findings: Fake backend tests internal scheduling only, not actual audio output.
- RustChunk relevance: Reuse in tests; keep real platform audio separate.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module cor
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.responding to `crates/horizon-client/examples/probe/audio_common/mod.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 944. `crates/horizon-client/examples/probe/auto_jump_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c15565850bde1d0e7ade78d58a8ab930c20f8b63`
- Size: 4650 bytes; 119 lines
- Coverage: Complete text file reviewed, lines 1–119; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for auto-jump decisions during player movement.
- Dependencies / cross-references: Player input, collision, ground/step state.
- Implementation / logic: Replays movement conditions and records auto-jump outcomes.
- Findings: Thresholds and collision geometry must match 1.20.1.
- RustChunk relevance: High-value movement parity harness.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed i
- RustChunk implementation mapping: n this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/auto_jump_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 945. `crates/horizon-client/examples/probe/block_entity_anim_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a7bdfab7bb7d705b26f7e0215c246c48b1878e9a`
- Size: 20522 bytes; 495 lines
- Coverage: Complete text file reviewed, lines 1–495; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for block-entity animation tick/render values.
- Dependencies / cross-references: Block entity ticker, animation state, block registry and RNG.
- Implementation / logic: Runs controlled animations and serializes per-tick state.
- Findings: Contains later-added entity/block states beyond 1.20.1; use version-filtered cases.
- RustChunk relevance: Use as test transport; rebuild target entity list.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where appl
- RustChunk implementation mapping: icable; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/block_entity_anim_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 946. `crates/horizon-client/examples/probe/block_entity_render_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4e437eceb3783601020a8eb3a87db4c0609689fe`
- Size: 17488 bytes; 435 lines
- Coverage: Complete text file reviewed, lines 1–435; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for block-entity render geometry and text output.
- Dependencies / cross-references: Model catalog, block state, sprite atlas, world and light.
- Implementation / logic: Builds render cases and serializes quads/text output.
- Findings: Uses current newer model/asset catalog; not an automatic 1.20.1 oracle.
- RustChunk relevance: Rebuild models and states for target.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edg
- RustChunk implementation mapping: e-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 947. `crates/horizon-client/examples/probe/block_entity_tickers_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d0cb2d8398e386aaa123dff69b9ef63ebe3c331a`
- Size: 3913 bytes; 103 lines
- Coverage: Complete text file reviewed, lines 1–103; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for block-entity ticker registration/order.
- Dependencies / cross-references: Block state, block entity store and chunk loading.
- Implementation / logic: Creates ticker cases and logs update order/events.
- Findings: Registry list is version-pinned.
- RustChunk relevance: Use only after reproducing 1.20.1 ticker order.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where appl
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/block_entity_render_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.icable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 948. `crates/horizon-client/examples/probe/block_particles_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0e58d99ea24274e53e7fd432f941675e4c468421`
- Size: 2320 bytes; 63 lines
- Coverage: Complete text file reviewed, lines 1–63; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for particles emitted by block updates/interactions.
- Dependencies / cross-references: Particle engine, block state and random source.
- Implementation / logic: Replays scripted cases and emits particle records.
- Findings: Particle ids/resources depend on version.
- RustChunk relevance: Rebuild target particle set.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: 
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 949. `crates/horizon-client/examples/probe/block_tint_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `23724453051b88d948cb8a4f283762dae5eee1d3`
- Size: 1374 bytes; 37 lines
- Coverage: Complete text file reviewed, lines 1–37; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for block and biome tint colors.
- Dependencies / cross-references: Biome tint tables, block facts and coordinates.
- Implementation / logic: Queries color outputs at fixed positions.
- Findings: Color maps/current biomes are from newer assets.
- RustChunk relevance: Regenerate 1.20.1 colormaps.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/block_particles_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 950. `crates/horizon-client/examples/probe/block_transform_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1431b2838e9215fd7fda31225ea64b73695e7d03`
- Size: 3209 bytes; 98 lines
- Coverage: Complete text file reviewed, lines 1–98; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for transformed block model/shape geometry.
- Dependencies / cross-references: Block state model, transform and mesher.
- Implementation / logic: Outputs transformed boxes/faces for scripted states.
- Findings: State/model catalogue is version-specific.
- RustChunk relevance: Use as parity harness with 1.20.1 states.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was exe
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/block_tint_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.cuted in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 951. `crates/horizon-client/examples/probe/breaking_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c5a816ccdc88949c06bff80cadc20d1fde0c2fcb`
- Size: 2995 bytes; 78 lines
- Coverage: Complete text file reviewed, lines 1–78; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for destroy-crack overlay geometry/UV.
- Dependencies / cross-references: Block model catalogue, mesh tables and breaking sprites.
- Implementation / logic: Generates breaking geometry and serializes quads/stage light.
- Findings: Requires target-version models and crack sprite rules.
- RustChunk relevance: Adapt after base mesher and model parity.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/block_transform_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core., edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 952. `crates/horizon-client/examples/probe/camera_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e8cffd88a16ca7b093fd284ecfe71c6acb8cbcc5`
- Size: 11718 bytes; 351 lines
- Coverage: Complete text file reviewed, lines 1–351; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for camera transforms, FOV, projection, bobbing and clipping.
- Dependencies / cross-references: Camera, entity pose, voxel shapes, ray clip and use-item state.
- Implementation / logic: Runs JSON-line operations and prints bit-level numeric camera outputs.
- Findings: Exact numeric behavior is visible; source uses current model/input types.
- RustChunk relevance: High priority for first-person correctness; rebuild 1.20.1 behavior.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/camera_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 953. `crates/horizon-client/examples/probe/cauldron_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1f26dd3030ccdfe9f073e4125c88e8b3cd3cbde6`
- Size: 5292 bytes; 157 lines
- Coverage: Complete text file reviewed, lines 1–157; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for cauldron interactions and state changes.
- Dependencies / cross-references: Block state, item stack, interaction and sounds.
- Implementation / logic: Runs scripted cauldron fill/drain/wash cases.
- Findings: Item/tag table must be the 1.20.1 version.
- RustChunk relevance: Good vanilla interaction parity tool.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/cauldron_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 954. `crates/horizon-client/examples/probe/chat_screen_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `db334797dbbe43c6bb8b9aa6ee8f4cd364df54d6`
- Size: 5238 bytes; 131 lines
- Coverage: Complete text file reviewed, lines 1–131; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for chat UI actions and draw output.
- Dependencies / cross-references: Chat screen, text, history, input and GUI renderer.
- Implementation / logic: Replays text/key/mouse operations and serializes screen state.
- Findings: Newer chat components/signing flags can appear; separate UI and protocol semantics.
- RustChunk relevance: Rebuild screen and signed-chat behavior for 1.20.1.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.scribed above; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/chat_screen_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 955. `crates/horizon-client/examples/probe/chat_screen_fixture/registries.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `42be0d206c812b51a8eb9aa55a0758850c1539e1`
- Size: 3506 bytes; 86 lines
- Coverage: Complete text file reviewed, lines 1–86; long files were read in consecutive sections of at most 400 lines.
- Purpose: Registry fixtures used by chat/command tests.
- Dependencies / cross-references: Command/registry ids and text resources.
- Implementation / logic: Provides a fixed lookup set for fixture transport.
- Findings: Registry contents are target-version data.
- RustChunk relevance: Regenerate from 1.20.1 command/registry tables.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/chat_screen_fixture/registries.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 956. `crates/horizon-client/examples/probe/chest_pair_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2deec575437fb0b9f04ed2466842b1aa0e2cf7f8`
- Size: 2516 bytes; 74 lines
- Coverage: Complete text file reviewed, lines 1–74; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for single/double chest adjacency and facing.
- Dependencies / cross-references: Block state properties and neighbor world lookup.
- Implementation / logic: Creates paired chest states and emits resulting model selection.
- Findings: Chest state logic should follow 1.20.1 block updates.
- RustChunk relevance: Useful block-entity adjacency test.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/chest_pair_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 957. `crates/horizon-client/examples/probe/cloud_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4ba072dbbd2d60f7b84e0fde7c1ec3160a5635d6`
- Size: 1352 bytes; 39 lines
- Coverage: Complete text file reviewed, lines 1–39; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for cloud geometry sample values.
- Dependencies / cross-references: Cloud mesh generator and weather/cloud parameters.
- Implementation / logic: Serializes generated cloud face points.
- Findings: Custom cloud rendering may differ from vanilla.
- RustChunk relevance: Use only as a test if RustChunk adopts the matching algorithm.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/cloud_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 958. `crates/horizon-client/examples/probe/command_parse_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c54e55bebb6d82214dbd4297219cb3c0a767f46c`
- Size: 11533 bytes; 263 lines
- Coverage: Complete text file reviewed, lines 1–263; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for command parsing and Brigadier argument handling.
- Dependencies / cross-references: Command dispatcher, argument codecs and registry data.
- Implementation / logic: Parses scripted commands and serializes success/error/arguments.
- Findings: Includes argument types and registry keys newer than 1.20.1.
- RustChunk relevance: Rebuild command tree from 1.20.1; don't expose newer commands.
- Decision: Exclude from the Minecraft 1.20.1 baseline; reconsider only if target-version evidence or an explicit extension requires it.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/command_parse_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 959. `crates/horizon-client/examples/probe/container_screen_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e82b5bdf5e3c1076ce71022ae7fed5f25bc3982e`
- Size: 3382 bytes; 86 lines
- Coverage: Complete text file reviewed, lines 1–86; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for container screen mouse/slot interaction.
- Dependencies / cross-references: Menu slot layout, mouse coordinates and screen input.
- Implementation / logic: Replays clicks/drags and outputs selected slot/actions.
- Findings: Menu click semantics must match Java 1.20.1.
- RustChunk relevance: Useful inventory UI parity case.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/container_screen_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 960. `crates/horizon-client/examples/probe/container_title_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6f7ab15a8d62a6be029ccc4a43b226e5f9d64a3b`
- Size: 3520 bytes; 98 lines
- Coverage: Complete text file reviewed, lines 1–98; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for container titles/custom NBT names.
- Dependencies / cross-references: Block-entity custom name and text components.
- Implementation / logic: Serializes displayed title output.
- Findings: Modern text components may differ from 1.20.1 NBT display name.
- RustChunk relevance: Adapt to 1.20.1 custom-name rules.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/container_title_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 961. `crates/horizon-client/examples/probe/creative_mining_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6cbc1815400019e66cdaa343d2d8e095ef6647fb`
- Size: 1625 bytes; 49 lines
- Coverage: Complete text file reviewed, lines 1–49; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for creative block-breaking permission.
- Dependencies / cross-references: Item rules/components and player game mode.
- Implementation / logic: Checks if item permits breaking a target block in creative.
- Findings: Uses newer `can_destroy_blocks_in_creative` component, not 1.20.1 item schema.
- RustChunk relevance: Do not port the component; implement 1.20.1 creative behavior.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/creative_mining_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 962. `crates/horizon-client/examples/probe/creative_search_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f3bae3649189082603254d946a220a7182a82969`
- Size: 2227 bytes; 56 lines
- Coverage: Complete text file reviewed, lines 1–56; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for creative inventory tab/search results.
- Dependencies / cross-references: Creative tabs, item registry and search filtering.
- Implementation / logic: Builds case-based result ordering.
- Findings: Tabs and items are pinned to newer registry data.
- RustChunk relevance: Regenerate target 1.20.1 creative inventory.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/creative_search_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage..

### 963. `crates/horizon-client/examples/probe/cushion_placement_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `45fbdbd720402258e949fd3816bc4fa0c5838fc3`
- Size: 2632 bytes; 89 lines
- Coverage: Complete text file reviewed, lines 1–89; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for cushion placement/placement context.
- Dependencies / cross-references: Block placement rules, collision shape and item use.
- Implementation / logic: Replays placement cases and resulting block states.
- Findings: Verify the named block/feature exists in target registry; may be a newer/mod-specific feature.
- RustChunk relevance: Do not assume baseline support without 1.20.1 registry evidence.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/cushion_placement_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 964. `crates/horizon-client/examples/probe/emitter_particles_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `949b306ffdfd33c938e24a6a2fb2748007d49f6a`
- Size: 2633 bytes; 74 lines
- Coverage: Complete text file reviewed, lines 1–74; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for emitter particle creation/tick events.
- Dependencies / cross-references: Particle provider registry, entity/block events and RNG.
- Implementation / logic: Runs seeded emitter cases and serializes output.
- Findings: Particle types and providers are version-specific.
- RustChunk relevance: Rebuild target particle providers.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/emitter_particles_p
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.robe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 965. `crates/horizon-client/examples/probe/enchanting_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7536f0e5b1137c7ba709c4ce3dd10f7362d8a85e`
- Size: 2037 bytes; 52 lines
- Coverage: Complete text file reviewed, lines 1–52; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for enchanting offers and selection.
- Dependencies / cross-references: Enchanting menu, item enchantability and RNG.
- Implementation / logic: Creates seeded offers and logs selection behavior.
- Findings: Enchantments/weights and item data must match 1.20.1.
- RustChunk relevance: Useful vanilla UI/gameplay parity tool.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: 
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.RustChunk module corresponding to `crates/horizon-client/examples/probe/enchanting_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 966. `crates/horizon-client/examples/probe/entity_anim_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b446f30859c17a8c8c53ffefd19c2cfb5b36000d`
- Size: 26816 bytes; 655 lines
- Coverage: Complete text file reviewed, lines 1–655; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for entity model animation transforms.
- Dependencies / cross-references: Entity species/model list, pose and animation state.
- Implementation / logic: Runs deterministic animation cases and serializes transforms.
- Findings: Includes later species (Creaking/Sniffer/Happy Ghast etc.) beyond 1.20.1.
- RustChunk relevance: Use as harness architecture; filter to 1.20.1 mobs.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device check
- RustChunk implementation mapping: s where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 967. `crates/horizon-client/examples/probe/entity_components_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2b1c3233f9f63a344c170ad1f14c0bf5fc411085`
- Size: 2740 bytes; 89 lines
- Coverage: Complete text file reviewed, lines 1–89; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for advancement entity predicates and species variant components.
- Dependencies / cross-references: Advancement predicates, entity views and species variant data.
- Implementation / logic: Builds entity component views and evaluates predicates.
- Findings: Modern entity component variant schema is newer than 1.20.1.
- RustChunk relevance: Rebuild target advancement predicates; do not import later component model.
- Decision: Exclude from the Minecraft 1.20.1 baseline; reconsider only if target-version evidence or an explicit extension requires it.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/entity_anim_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 968. `crates/horizon-client/examples/probe/entity_death_loot_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `654d4cc4d6e5f52ca6785c035d4c46b898239966`
- Size: 13284 bytes; 322 lines
- Coverage: Complete text file reviewed, lines 1–322; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for death loot and dropped item events.
- Dependencies / cross-references: Loot tables, entity state, RNG and ItemStack serialization.
- Implementation / logic: Runs seeded death cases and records drops.
- Findings: New item components/species can be present; target loot data must be rebuilt.
- RustChunk relevance: Adapt to 1.20.1 death/loot ordering.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 orac
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/entity_components_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.le fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 969. `crates/horizon-client/examples/probe/entity_particles_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cdb0344df440413483938478e8020ce74cd22140`
- Size: 1893 bytes; 51 lines
- Coverage: Complete text file reviewed, lines 1–51; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for entity-triggered particles.
- Dependencies / cross-references: Entity events, particle options and RNG.
- Implementation / logic: Runs entity particle cases and logs spawn values.
- Findings: Event/particle registry is version-dependent.
- RustChunk relevance: Regenerate 1.20.1 particle map.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: ables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 970. `crates/horizon-client/examples/probe/environment_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `27846fe824d8c05f1dfb93f862dc99dbf05f8edf`
- Size: 3458 bytes; 80 lines
- Coverage: Complete text file reviewed, lines 1–80; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for environment state consumed by gameplay/rendering.
- Dependencies / cross-references: Dimension, biome, weather/time and player context.
- Implementation / logic: Supplies controlled environmental state and serializes derived values.
- Findings: Biome/environment data comes from pinned snapshot.
- RustChunk relevance: Use to build 1.20.1 environment parity tests.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/entity_particles_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 971. `crates/horizon-client/examples/probe/finalize_spawn_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `38b9748799f7369a6223ec98b0d87f3f3dbf0b36`
- Size: 18460 bytes; 533 lines
- Coverage: Complete text file reviewed, lines 1–533; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for `finalizeSpawn` variant/attribute behavior.
- Dependencies / cross-references: Mob types, spawn reason/difficulty, biome, random stream and entity state.
- Implementation / logic: Creates seeded mobs and logs finalized state/draw order.
- Findings: Contains species added after 1.20.1 (e.g. newer aquatic/undead variants); filter carefully.
- RustChunk relevance: High-value test pattern; rebuild only target species.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: ssets from the exact Minecraft 1.20.1 sources; do not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/finalize_spawn_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 972. `crates/horizon-client/examples/probe/fire_starter_sound_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `18c1b487a20e93faa9848cf2f5356f12da328ca7`
- Size: 1012 bytes; 32 lines
- Coverage: Complete text file reviewed, lines 1–32; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for fire-charge/fire-starter sound and pitch RNG.
- Dependencies / cross-references: Item use, sound registry and LegacyRandom.
- Implementation / logic: Feeds seed values and logs sound, pitch and next RNG value.
- Findings: Small but random-call order is observable and version-bound.
- RustChunk relevance: Adapt to 1.20.1 fire charge semantics.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/fire_starter_sound_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 973. `crates/horizon-client/examples/probe/firework_tooltip_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3857768370e07339cf84f60f95c4f07b684bf081`
- Size: 1325 bytes; 30 lines
- Coverage: Complete text file reviewed, lines 1–30; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for firework tooltip text.
- Dependencies / cross-references: Item stack/firework data and tooltip text.
- Implementation / logic: Serializes tooltip rows for selected firework cases.
- Findings: Modern component schema may differ; 1.20.1 uses target NBT forms.
- RustChunk relevance: Rebuild target NBT/tooltip mapping.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/firework_tooltip_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 974. `crates/horizon-client/examples/probe/first_person_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cb4c38e5848150ef263f0b639921ee731d19816a`
- Size: 15521 bytes; 472 lines
- Coverage: Complete text file reviewed, lines 1–472; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for first-person hand/action/attack/destroy behavior.
- Dependencies / cross-references: Camera, item use, attack cooldown and block destruction.
- Implementation / logic: Replays actions and emits first-person state.
- Findings: Exact timing and item categories must match target 1.20.1.
- RustChunk relevance: High priority for player feel and visual parity.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/first_person_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 975. `crates/horizon-client/examples/probe/frame_limiter_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b11338afc8ef052505e6b87aad94dc9f263fdb0b`
- Size: 3505 bytes; 107 lines
- Coverage: Complete text file reviewed, lines 1–107; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for timer/frame limiter behavior.
- Dependencies / cross-references: Clock abstraction, client timer and scheduler.
- Implementation / logic: Runs scripted time intervals and outputs tick/partial/delay.
- Findings: Desktop sleeps are not an Android scheduler; lifecycle and refresh rate differ.
- RustChunk relevance: Reuse deterministic clock design; implement platform-specific pacing.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/frame_limiter_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 976. `crates/horizon-client/examples/probe/game_loop_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `598f480e61d4eb85b42c52ce9a7279e8cef59dd6`
- Size: 10388 bytes; 311 lines
- Coverage: Complete text file reviewed, lines 1–311; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for game loop/tick/window/input scheduling.
- Dependencies / cross-references: Client timer, tick loop, mouse/keyboard and window events.
- Implementation / logic: Replays scripted events and logs tick/render order.
- Findings: Desktop window lifecycle differs from Android activity lifecycle.
- RustChunk relevance: Match 1.20.1 tick behavior while adapting Android lifecycle.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/game_loop_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 977. `crates/horizon-client/examples/probe/ghost_recipe_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04035a57c65c4318f4f1ffc699b026b17f275226`
- Size: 3178 bytes; 74 lines
- Coverage: Complete text file reviewed, lines 1–74; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for recipe ghost/display slots.
- Dependencies / cross-references: Recipe manager, item ingredients and inventory UI.
- Implementation / logic: Serializes recipe preview/display output.
- Findings: Recipe and item data are version-specific.
- RustChunk relevance: Rebuild from 1.20.1 recipe schema.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/ghost_recipe_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 978. `crates/horizon-client/examples/probe/held_drop_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1275021bb03713a1ff6e06bd4725f316bc7f12a3`
- Size: 1355 bytes; 38 lines
- Coverage: Complete text file reviewed, lines 1–38; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for dropping selected/held items.
- Dependencies / cross-references: Player inventory, selected slot and drop interaction.
- Implementation / logic: Runs drop-one/drop-stack inputs and records inventory/result.
- Findings: Item stack serialization and action order must match 1.20.1.
- RustChunk relevance: Use as core inventory parity case.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/held_drop_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 979. `crates/horizon-client/examples/probe/held_item_stack_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4a5765151e23aa3fd0fa8637c48a376573bfeaac`
- Size: 1263 bytes; 38 lines
- Coverage: Complete text file reviewed, lines 1–38; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for held item stack semantics.
- Dependencies / cross-references: Player hand/slot inventory and ItemStack.
- Implementation / logic: Reads selected held stack and serializes state.
- Findings: New component fields can differ by version.
- RustChunk relevance: Implement target ItemStack empty/count/copy semantics.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/held_item_stack_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 980. `crates/horizon-client/examples/probe/hotbar_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4e07764bfa2bbc2456ec2221425c1931214b5e5`
- Size: 1797 bytes; 46 lines
- Coverage: Complete text file reviewed, lines 1–46; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for hotbar contents and selected index.
- Dependencies / cross-references: Player inventory and NBT conversion.
- Implementation / logic: Loads cases and emits slot/selection output.
- Findings: Inventory/slot serialization is versioned.
- RustChunk relevance: Adapt hotbar indices and NBT to 1.20.1.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/hotbar_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 981. `crates/horizon-client/examples/probe/hud_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5fc77309b0380c29f078620a5bed4d0c89f12031`
- Size: 9839 bytes; 277 lines
- Coverage: Complete text file reviewed, lines 1–277; long files were read in consecutive sections of at most 400 lines.
- Purpose: Headless HUD fixture for draw calls and game state.
- Dependencies / cross-references: HUD atlas, health/hotbar inputs, text and draw encoder.
- Implementation / logic: Builds crosshair/hotbar/health/XP and overlay draw commands.
- Findings: Some newer HUD indicators/effects may be included; target atlas and behavior needed.
- RustChunk relevance: High-value visual parity test; rebuild exact 1.20.1 draw list.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/hud_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.-device evidence for Android/GPU/audio paths.

### 982. `crates/horizon-client/examples/probe/hud_fixture/kinds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6157a01a5c6818886e6776c0310fb0d7f64f3415`
- Size: 12563 bytes; 332 lines
- Coverage: Complete text file reviewed, lines 1–332; long files were read in consecutive sections of at most 400 lines.
- Purpose: Specialized HUD overlay fixture implementations.
- Dependencies / cross-references: Boss bar/title/scoreboard/status effects and HUD draw commands.
- Implementation / logic: Generates overlay-specific outputs and state values.
- Findings: Overlay set and game fields may contain later additions.
- RustChunk relevance: Use only 1.20.1 overlay types and ordering.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/hud_fixture/kinds.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 983. `crates/horizon-client/examples/probe/inline_object_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6e8c3b2eaaad4608cefd946bca1915523da3d2ec`
- Size: 2303 bytes; 69 lines
- Coverage: Complete text file reviewed, lines 1–69; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for inline sprite/object text rendering.
- Dependencies / cross-references: Text components and font provider object glyphs.
- Implementation / logic: Serializes text with inline objects and style runs.
- Findings: Inline-object resource syntax may be a Horizon extension, not a vanilla 1.20.1 component.
- RustChunk relevance: Treat as optional extension until target spec confirms.
- Decision: Exclude from the Minecraft 1.20.1 baseline; reconsider only if target-version evidence or an explicit extension requires it.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device 
- RustChunk implementation mapping: checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/inline_object_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 984. `crates/horizon-client/examples/probe/input_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ae66f77a65984e83ebfe63f8ed3b27ca806b08f5`
- Size: 1441 bytes; 43 lines
- Coverage: Complete text file reviewed, lines 1–43; long files were read in consecutive sections of at most 400 lines.
- Purpose: Small deterministic probe for input state transitions.
- Dependencies / cross-references: Input actions and key/mouse state.
- Implementation / logic: Replays basic action sequence and emits logical state.
- Findings: Platform bindings should not leak into game actions.
- RustChunk relevance: Useful input test pattern.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/input_probe.rs`, or an isolated optional/t
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.ooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 985. `crates/horizon-client/examples/probe/interaction_input_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `938c3736e3b9f58ea46b115ca7598c88ed0ab838`
- Size: 4124 bytes; 135 lines
- Coverage: Complete text file reviewed, lines 1–135; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for interaction action mapping and key/button state.
- Dependencies / cross-references: Input mode, keybinds, interactions and UI focus.
- Implementation / logic: Replays press/release/action operations.
- Findings: Must handle touch cancel and multiple pointers to avoid stuck key states.
- RustChunk relevance: Important for Android control correctness.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk modu
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.le corresponding to `crates/horizon-client/examples/probe/interaction_input_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 986. `crates/horizon-client/examples/probe/lan_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3be556e3b60dd71ff2478b55b00730c38ed82ac6`
- Size: 1736 bytes; 46 lines
- Coverage: Complete text file reviewed, lines 1–46; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for LAN server discovery.
- Dependencies / cross-references: UDP broadcast/multicast and server advertisement fields.
- Implementation / logic: Sends/receives local discovery messages and serializes found servers.
- Findings: Network permission/multicast behavior differs across Android devices.
- RustChunk relevance: Optional multiplayer convenience feature.
- Decision: Keep optional and isolated from the Vanilla 1.20.1 core; do not make it a launch or gameplay dependency.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applica
- RustChunk implementation mapping: ble; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/lan_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 987. `crates/horizon-client/examples/probe/light_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `41c2ba7848984e15502b8b51c9a8dfcee15ed769`
- Size: 13157 bytes; 333 lines
- Coverage: Complete text file reviewed, lines 1–333; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for light-engine parity against saved Vanilla chunks.
- Dependencies / cross-references: Java-produced chunk data, block/sky light arrays and incremental edits.
- Implementation / logic: Loads chunks, applies light operations and reports differences.
- Findings: Strong correctness oracle pattern; no test was run during this audit.
- RustChunk relevance: Very high value for 1.20.1 lighting validation; recreate target fixtures.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/light_parity.rs`, or an isolated optional/tooling module if 
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 988. `crates/horizon-client/examples/probe/lore_tooltip_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `350d9be8191ff5193d974644fa96ae0a13d49f3f`
- Size: 1861 bytes; 53 lines
- Coverage: Complete text file reviewed, lines 1–53; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for item lore tooltip text.
- Dependencies / cross-references: Item lore data, text components and tooltip layout.
- Implementation / logic: Serializes styled tooltip rows.
- Findings: New component schema may not match 1.20.1 display.Lore NBT.
- RustChunk relevance: Adapt to target NBT and tooltip ordering.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/lore_tooltip_f
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.ixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 989. `crates/horizon-client/examples/probe/marker_particles_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8dc48c45c7a142fc46c4c3483dff39cb4f589b87`
- Size: 3631 bytes; 98 lines
- Coverage: Complete text file reviewed, lines 1–98; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for marker particle state.
- Dependencies / cross-references: Particle registry, RNG and world position.
- Implementation / logic: Runs deterministic marker particle cases and logs state.
- Findings: Particle definitions are target-version data.
- RustChunk relevance: Regenerate target particle set.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/marker_particles_probe.rs`, or an isolated
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant. optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 990. `crates/horizon-client/examples/probe/menu_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2fc108897cab1f1540ab4797cda0dae91f3b139a`
- Size: 9555 bytes; 248 lines
- Coverage: Complete text file reviewed, lines 1–248; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for menu state/actions and draw output.
- Dependencies / cross-references: Menu screens, text, input and screen draw commands.
- Implementation / logic: Replays menu interactions and serializes current state.
- Findings: Horizon-specific screens should not be mixed with vanilla screen parity.
- RustChunk relevance: Use as generic UI test harness; target menu set only.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/menu_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 991. `crates/horizon-client/examples/probe/mipmap_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa94f4e9027a3019934dcfd8b7a4b8ee81af3188`
- Size: 1855 bytes; 47 lines
- Coverage: Complete text file reviewed, lines 1–47; long files were read in consecutive sections of at most 400 lines.
- Purpose: Probe for mipmap generation and alpha coverage.
- Dependencies / cross-references: Image loader, mipmap levels and texture filtering.
- Implementation / logic: Creates mipmaps and records sample results.
- Findings: Filtering/alpha behavior affects foliage/cutout textures and needs Java parity.
- RustChunk relevance: Use to validate texture pipeline before tuning performance.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/mipmap_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 992. `crates/horizon-client/examples/probe/mob_ai_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f4947016a0a6ad471bc65c35b874c66fd3a6ff8d`
- Size: 66543 bytes; 1697 lines
- Coverage: Complete text file reviewed, lines 1–1697; long files were read in consecutive sections of at most 400 lines.
- Purpose: Large deterministic mob AI parity harness.
- Dependencies / cross-references: Mob types, world/block/biome/light/collision, brains/goals, events and LegacyRandom.
- Implementation / logic: Builds scenarios and logs every AI-visible field, goals, navigation, movement, events and RNG state per tick.
- Findings: Includes newer systems/species (Creaking heart, Copper Golem, Happy Ghast, Nautilus etc.) outside 1.20.1; simulation order/data are snapshot-specific.
- RustChunk relevance: High value test harness; rebuild target 1.20.1 mob subset, never transplant its newer species list.
- Decision: Strong candidate for RustChunk adaptation, but rebuild target data and require the 1.20.1 parity evidence listed below before adoption.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candid
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.ate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/mob_ai_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 993. `crates/horizon-client/examples/probe/mob_attributes_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f1fcec701e4ca4d4a9181323ec290241fa109d51`
- Size: 2217 bytes; 56 lines
- Coverage: Complete text file reviewed, lines 1–56; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for mob dimensions and attributes.
- Dependencies / cross-references: Mob type registry and entity attribute model.
- Implementation / logic: Prints dimensions, scale, max health, speed, attack and related attributes.
- Findings: Values come from newer registry data and include later attributes.
- RustChunk relevance: Regenerate only 1.20.1 mob attribute tables.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: 
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/mob_attributes_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 994. `crates/horizon-client/examples/probe/music_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `02aecb373b9c546eaeb6b3f73bedbb45ace6e8f5`
- Size: 4288 bytes; 107 lines
- Coverage: Complete text file reviewed, lines 1–107; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for MusicManager sequencing over SoundEngine.
- Dependencies / cross-references: Music event data, category volume, seeded RNG and fake audio backend.
- Implementation / logic: Replays situation/tick/stop/frequency/toast operations and serializes channel state.
- Findings: Music list and timing policy are versioned; fake backend isn't actual audio output.
- RustChunk relevance: Useful scheduling parity harness; rebuild 1.20.1 music events.
- Decision: Use as architecture or test-harness reference only; no direct port is accepted until the listed 1.20.1 evidence is satisfied.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: 
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/music_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: FULL TEXT REVIEW COMPLETE — complete source file was read and documented. No tests, build, benchmark, APK, server join, Forge client or Android device run was executed in this stage.

### 995. `crates/horizon-client/examples/probe/natural_spawn_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d783d339ffffeef4c236d999598224b78eb8d8a3`
- Size: 52918 bytes; 1430 lines
- Coverage: Complete text file reviewed, lines 1–1430; long files were read in consecutive sections of at most 400 lines.
- Purpose: Large natural spawn, spawner, trial-spawner and vault parity fixture.
- Dependencies / cross-references: Biome spawn tables, block/structure tags, collisions, entity counts, loot and SNBT.
- Implementation / logic: Builds synthetic world and replays spawn selection/spawner/trial/vault ticks and events.
- Findings: Contains Trial Spawner and Vault (1.21-era), plus newer species; these are not 1.20.1 baseline features.
- RustChunk relevance: Adapt natural spawning and ordinary spawner only; exclude Trial/Vault from 1.20.1.
- Decision: Exclude from the Minecraft 1.20.1 baseline; reconsider only if target-version evidence or an explicit extension requires it.
- 1.20.1 implications: Rebuild this file's data and semantics against the exact Minecraft 1.20.1 source; the pinned Starling snapshot is not itself a 1.20.1 contract.
- Follow-up checks: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- RustChunk implementation mapping: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/natural_spawn_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Evidence required before adoption: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths where relevant.
- Completion state: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 996. `crates/horizon-client/examples/probe/options_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `97c76c030f0c5d5a907f7bbbe7418e7537c6d497`
- Size: 1462 bytes; 40 lines
- Coverage: Complete text file reviewed, lines 1–40; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for options.txt load/save and keybind parity.
- Dependencies / cross-references: Options parser/serializer and keybind mappings.
- Implementation / logic: Loads option examples, normalizes Horizon render-distance extension and prints vanilla-like serialization.
- Findings: Horizon permits render distance 64 then normalizes above 32 for vanilla comparison; this is a deliberate extension.
- RustChunk relevance: Implement exact 1.20.1 Options defaults/ranges and preserve unknown keys deliberately.
- Decision: Potentially useful as a source/test/architecture reference for the subsystem described above; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: See Findings above; adapt algorithms only after target-version comparison.
- Follow-up checks: Rebuild versioned IDs, registries, packet/data schemas and assets from the exact Minecraft 1.20.1 sources; do not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/options_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 997. `crates/horizon-client/examples/probe/pack_resources_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `80dbf34964f17dfe3d0cfa46b054540a986a84a7`
- Size: 3716 bytes; 92 lines
- Coverage: Complete text file reviewed, lines 1–92; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for pack overlay and resource loader resolution.
- Dependencies / cross-references: Pack discovery/enabled order, assets paths, lang, sound, font and atlas.
- Implementation / logic: Builds overlay and queries files/language/sounds/font/atlas providers.
- Findings: Resource formats/order must match 1.20.1; silent parse fallback can hide malformed packs.
- RustChunk relevance: Important resource-pack parity harness; regenerate target resource codecs.
- Decision: Potentially useful as a source/test/architecture reference for the subsystem described above; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: See Findings above; adapt algorithms only after target-version comparison.
- Follow-up checks: Rebuild versioned IDs, registries, packet/data schemas and assets from the exact Minecraft 1.20.1 sources; do not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/pack_resources_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 998. `crates/horizon-client/examples/probe/pack_screen_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `73912074f14a79d904f2f8eb0df6d03ee66c777f`
- Size: 7240 bytes; 177 lines
- Coverage: Complete text file reviewed, lines 1–177; long files were read in consecutive sections of at most 400 lines.
- Purpose: Headless fixture for resource-pack screen actions and draw output.
- Dependencies / cross-references: PackScreen, selection state, text, atlas and input operations.
- Implementation / logic: Applies click/scroll/type/erase actions and serializes draw commands.
- Findings: Screen labels/compatibility levels and UI structure can be Horizon-specific.
- RustChunk relevance: Reuse UI model test pattern; implement 1.20.1 pack format labels.
- Decision: Potentially useful as a source/test/architecture reference for the subsystem described above; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: See Findings above; adapt algorithms only after target-version comparison.
- Follow-up checks: Rebuild versioned IDs, registries, packet/data schemas and assets from the exact Minecraft 1.20.1 sources; do not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/pack_screen_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 999. `crates/horizon-client/examples/probe/pack_selection_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `91c58734363e90253d294ead837d3981e634dbbd`
- Size: 3690 bytes; 100 lines
- Coverage: Complete text file reviewed, lines 1–100; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for pack selection ordering and allowed moves.
- Dependencies / cross-references: SelectionModel, required/fixed flags and compatibility.
- Implementation / logic: Replays activate/up/down and outputs selectable flags and committed order.
- Findings: Pack compatibility policy must be versioned and required/server packs must not be removable.
- RustChunk relevance: Useful core resource-pack selection tests.
- Decision: Potentially useful as a source/test/architecture reference for the subsystem described above; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: See Findings above; adapt algorithms only after target-version comparison.
- Follow-up checks: Rebuild versioned IDs, registries, packet/data schemas and assets from the exact Minecraft 1.20.1 sources; do not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/pack_selection_fixture.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths.

### 1000. `crates/horizon-client/examples/probe/particle_providers_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aa47f683e1b9aa6d995915793190fffc957b22d5`
- Size: 9284 bytes; 249 lines
- Coverage: Complete text file reviewed, lines 1–249; long files were read in consecutive sections of at most 400 lines.
- Purpose: Fixture for ParticleEngine provider creation and tick outputs.
- Dependencies / cross-references: Particle type registry, world, block collision, assets, RNG and sound events.
- Implementation / logic: Creates particles from cases, ticks them and logs positions/velocities/quads/events.
- Findings: Includes newer particle types and a fixed local resources path; catalogue/asset paths must be rebuilt for 1.20.1.
- RustChunk relevance: Use as harness architecture; filter to supported 1.20.1 providers.
- Decision: Potentially useful as a source/test/architecture reference for the subsystem described above; this is not proof of 1.20.1 compatibility.
- 1.20.1 implications: See Findings above; adapt algorithms only after target-version comparison.
- Follow-up checks: Rebuild versioned IDs, registries, packet/data schemas and assets from the exact Minecraft 1.20.1 sources; do not import Starling 26.3 tables unchanged.
- RustChunk implementation mapping: Create Java 1.20.1 oracle fixtures, edge-case tests and device checks where applicable; no test was executed in this audit.
- Evidence required before adoption: Candidate mapping: RustChunk module corresponding to `crates/horizon-client/examples/probe/particle_providers_probe.rs`, or an isolated optional/tooling module if the feature is not vanilla core.
- Completion state: Pinned 1.20.1 source comparison, deterministic fixture outputs and targeted regression evidence; on-device evidence for Android/GPU/audio paths.

## Batch result

- Entries added: 200 unique paths (801–1000).
- Text files read fully: 130.
- PNG files metadata-only, NOT fully reviewed: 70.
- Four prior paths were skipped during deterministic selection; none were counted twice.
- All source SHAs and byte sizes are pinned to the Starling snapshot listed above.
- No build, test, benchmark, APK, Forge, server or Android device execution was performed.
