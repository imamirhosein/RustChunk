# RustChunk Starling Audit Ledger — Entries 1001–1200

- Source repository/ref: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1001–1200 (200 new unique source files)
- Persistence rule: write, validate and commit every 20 completed entries.
- Backup rule: create the next backup only after every additional 1,000 audit entries; the next backup is due after entry 2000, not entry 1200.
- Each entry contains the 15 required fields defined in `AUDIT_PROTOCOL.md`.
- Long text files are read completely in consecutive sections of at most 400 lines. Tests are not claimed as executed unless actually run.

### 1001. `crates/horizon-client/examples/probe/recipe_book_render_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `de1c34fce92332c933cc3e0df62378a8c8a9684f`
- Size: 10364 bytes; 264 logical lines
- Coverage: Complete text file reviewed, lines 1–264.
- Purpose: Headless rendering oracle for the crafting/furnace recipe-book screen; converts JSON cases into the actual draw calls produced by the client recipe-book renderer.
- Dependencies / cross-references: Uses Items and Recipes catalogs, Book/Entry data, HUD atlas/font metrics, language translations, recipe-book-screen drawing types, pack overlay and serde_json input cases.
- Implementation / logic: Loads item/recipe resources, builds the book and atlas, constructs tabs/search/buttons/pages/overlay/ghost slots from each case, calls draw(), then serializes blits, sprites, fills, highlights, text, item calls and tooltips including rounded pose matrices.
- Findings: It is a test transport, not a screen controller or resource loader for the game. Unknown recipe type falls back to Smoker; page/overlay indices are trusted and may panic on malformed fixtures. Displayed draw calls depend on this snapshot's atlas, recipe catalogue and localized strings.
- RustChunk relevance: Useful differential-test shape for recipe-book UI, render ordering, ghost slots, tooltips and layout. The architecture is adaptable; its recipe IDs, category tabs, item catalog and draw output are not 1.20.1 evidence.
- Decision: **adaptable; reference; version-specific — retain the oracle pattern and rebuild recipe inputs, categories and expected draw calls for Java 1.20.1.**
- 1.20.1 implications: 1.20.1 recipe-book types and clientbound recipe displays differ from the 26.3 snapshot. Reconstruct the 1.20.1 recipe categories/search/filter semantics and GUI resources; don't import later recipe/display IDs.
- Follow-up checks: Compare crafting, furnace, blast-furnace and smoker books against a Java 1.20.1 oracle; add invalid page/overlay bounds tests and localized text/font atlas parity.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/recipe_book_render_fixture.rs` (proposed, not created); production touch/UI must call a RustChunk-owned renderer with 1.20.1 data.
- Evidence required before adoption: Pinned 1.20.1 recipe registry/display data, golden draw-call fixtures for narrow/wide screens, filters, pages, overlays, ghosts and tooltips, plus Android text/atlas screenshots.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 264 logical lines. No test, build or UI execution was performed.

### 1002. `crates/horizon-client/examples/probe/recipe_book_view_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ee915d9d8298d1efcfdb4b0cb803317c0a047fe`
- Size: 5667 bytes; 131 logical lines
- Coverage: Complete text file reviewed, lines 1–131.
- Purpose: Headless behavioral oracle for recipe-book contents: which tabs are visible, which collections appear, page counts, and which recipes are selected/craftable.
- Dependencies / cross-references: Uses Items, Recipes, RecipeIndex, Book/ClientBook/Display/Tab, stack parsing, inventory/menu slot accounting, item max-stack rules, creative names and item tooltip generation.
- Implementation / logic: Replays add/remove/replace display-history batches; accounts inventory and crafting/furnace slots; chooses book type and grid; filters recipes by tab, fit, search tooltip matching and craftability; emits tab visibility, pages and per-recipe selected/craftable markers.
- Findings: The fixture models the client recipe display history and local stock accounting but trusts fixture indices and type strings. Registry IDs, display IDs, recipe categories and tooltip search results are snapshot-specific.
- RustChunk relevance: Strong reusable test pattern for recipe-book state, inventory accounting, category visibility and search. Not a standalone recipe engine and does not prove 1.20.1 recipe parity.
- Decision: **adaptable; reference; version-specific — keep the state-transition and differential harness pattern; rebuild recipe/display registries for 1.20.1.**
- 1.20.1 implications: Rebuild the 1.20.1 recipe-book packet/display history, recipe IDs, categories and ingredient matching against Java. Do not reuse later snapshot display IDs or catalogue tables.
- Follow-up checks: Test history replacement/removal, duplicate display IDs, empty collections, search strings, grid sizes, stack limits and filtering against Java 1.20.1.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/recipe_book_view_fixture.rs` (proposed, not created); eventual logic belongs in a versioned recipe-book model separate from UI rendering.
- Evidence required before adoption: Java 1.20.1 output fixtures for display history, each book type, inventory/slot accounting, search/tooltips, page boundaries and craftable state.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 131 logical lines. No tests were executed.

### 1003. `crates/horizon-client/examples/probe/recipe_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `993e21117553df0bd19589ecf40888b5e81fb5f7`
- Size: 3669 bytes; 92 logical lines
- Coverage: Complete text file reviewed, lines 1–92.
- Purpose: Command-line differential harness for recipe lookup and station output: crafting, cooking, stonecutting and smithing.
- Dependencies / cross-references: Loads Items, Recipes, StationRecipes and Enchantments from prepared resources; uses parse_stack/canon_stack and a tab-separated input format.
- Implementation / logic: Reads operation rows, builds crafting grids and calls recipe lookup/remainders; calls cooking lookup for furnace-like recipe IDs/results/XP/time; enumerates stonecutter outputs; computes smithing results; writes canonical stack forms.
- Findings: The harness is intentionally thin and depends on prepared data correctness. Cooking recipe IDs are normalized with a fixed `minecraft:` prefix, and input indexing is trusted; malformed fixture rows can fail.
- RustChunk relevance: Core-relevant recipe parity pattern for survival gameplay and crafting stations. The typed dispatch and canonical stack comparison are reusable, while every recipe/resource table must be generated from 1.20.1.
- Decision: **adaptable; reference; version-specific — use as a Java-oracle harness template; do not reuse the newer recipe catalogue.**
- 1.20.1 implications: 1.20.1 uses older recipe/serializer and smithing-template behavior than 26.3. Rebuild recipe IDs, ingredient predicates, remainders, cooking metadata and smithing transformations from 1.20.1.
- Follow-up checks: Compare shaped/shapeless recipes, remainders, cooking XP/time, stonecutter ordering and smithing template/base/addition cases against a 1.20.1 Java oracle.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/recipe_fixture.rs` (proposed, not created); runtime recipe matching belongs in the 1.20.1 data/recipe subsystem.
- Evidence required before adoption: Version-pinned recipe JSON and registry IDs, golden outputs for all station types, edge cases for empty/invalid stacks and deterministic ordering.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 92 logical lines. No tests were executed.

### 1004. `crates/horizon-client/examples/probe/remote_attack_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f41b7a5693e8c4a02a9e09d3852210fb2c55bcf0`
- Size: 1948 bytes; 52 logical lines
- Coverage: Complete text file reviewed, lines 1–52.
- Purpose: Differential probe for the attacking player's client-side remote-player hit reaction, especially post-hit slowdown and sprint cancellation.
- Dependencies / cross-references: Uses AttackStrength, attacker slowdown constant, JSON case inputs and a VanillaRemoteAttackOracle script.
- Implementation / logic: Loads ticker/speed/damage or spin/knockback/sprint/velocity/target fields; computes scaled attack strength; only applies horizontal velocity slowdown and clears sprint for player targets when the shared attack-strength predicate allows it.
- Findings: Deliberately excludes mob targets because the fixture notes Entity.hurtClient is false for those in this path. It does not simulate the server's authoritative damage/knockback or the full combat pipeline.
- RustChunk relevance: Useful focused test for a 1.20.1 player-hit client reaction; the mathematical test shape is portable. Constants and attack-strength semantics must be verified against the target Java version.
- Decision: **adaptable; reference; version-specific — retain as a narrow regression test pattern, not as the complete combat implementation.**
- 1.20.1 implications: Validate attack cooldown scaling, sprint reset and horizontal slowdown against 1.20.1 Player.attack behavior. Later combat changes must not be inferred from this one function.
- Follow-up checks: Add boundary cases around cooldown strength, critical/sweep/spin damage representations, non-player targets, zero/negative knockback and velocity components.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/remote_attack_probe.rs` (proposed, not created); integrate verified behavior into RustChunk's combat/input simulation only after Java parity.
- Evidence required before adoption: Golden Java 1.20.1 vectors for attack strength and resulting attacker state; server-side combat tests kept separate.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 52 logical lines. No test execution was performed.

### 1005. `crates/horizon-client/examples/probe/scene_collision_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7f22a11735b24e96022e8a4a6d36096ea9807166`
- Size: 4258 bytes; 118 logical lines
- Coverage: Complete text file reviewed, lines 1–118.
- Purpose: Headless transport that loads actual client assets/chunks and exercises world collision and movement resolution against a controlled scene.
- Dependencies / cross-references: Depends on Assets/RenderAssets, World::test_scene, collision binding, SceneCollisions, AABB/Vec3, EntityIdentity, CollisionContext and world_movement APIs.
- Implementation / logic: Validates a 1..16 chunk radius, loads resources and collision bindings, writes scene metadata, reads serialized movement/AABB/entity cases, resolves movement with step height and checks unobstructed positions before/after movement.
- Findings: Uses a fixed test subject identity, a very high candidate budget, and a synthetic scene rather than a complete loaded world. The fixture directory is expected to be prepared externally; it does not persist chunks or exercise server authority.
- RustChunk relevance: High-value parity harness for near-chunk collision, step-up logic and anti-void movement correctness. It is one of the most directly useful tests for RustChunk's known chunk/ground reliability target.
- Decision: **adaptable; reference; version-specific — adapt this harness with Java 1.20.1 collision shapes, block states and movement oracle outputs.**
- 1.20.1 implications: Collision shapes, step rules, world bounds and block-state registry data must match 1.20.1. This fixture cannot make distance-LOD geometry authoritative or replace near-chunk collision.
- Follow-up checks: Cover slab/stair/fence/wall/trapdoor/fluid/entity collision, chunk borders, unloaded chunks, step-height edges and collision candidate-budget exhaustion; use deterministic scenes.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/scene_collision_probe.rs` (proposed, not created); production mapping is the world collision/movement subsystem, independent of far-LOD rendering.
- Evidence required before adoption: Java 1.20.1 golden movement vectors and collision boxes, deterministic synthetic chunks, plus Android device tests for ground continuity and chunk-boundary movement.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 118 logical lines. No runtime test was executed.

### 1006. `crates/horizon-client/examples/probe/scene_motion_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `43564d3bcf2d2eebdaed9b9d82bd0f8c75c5d51a`
- Size: 30534 bytes; 723 logical lines
- Coverage: Complete source read in consecutive sections, maximum 400 lines per section.
- Purpose: Large differential harness that replays sequential player-motion operations in one resident synthetic world and snapshots player, attributes, health, fluids, portal state, movement flags and emitted events after every operation.
- Dependencies / cross-references: Uses Assets/World/collisions/MotionResources, PlayerMotion/MotionWorld, health and food rules, attributes/modifiers, movement statistics, portal state, block registry, fluid states and JSON fixtures.
- Implementation / logic: Builds a test scene, creates a player with stable attribute/instance identities, replays a broad operation vocabulary (pose, controlled tick, movement, jump, sprint, health/food, hurt/fall, fluids/air, powder snow, portal, effects, blocks, modifiers) and serializes deterministic state/event snapshots.
- Findings: The fixture encodes explicit oracle assumptions: one player, test scene radius 1, no armor (armor-hurt events filtered), player attribute identities injected, block effects only enabled for snow_tick, and a simplified portal transition model. It is a differential transport, not a complete live simulation.
- RustChunk relevance: Very high-value reference for RustChunk's player physics and state consistency; it reveals which state fields and event ordering need parity tests. The test scaffold is reusable but version-specific data/semantics are not.
- Decision: **adaptable; reference; version-specific — preserve the deterministic replay/snapshot technique and rebuild all cases and expected semantics for Java 1.20.1.**
- 1.20.1 implications: 1.20.1 differs from the pinned 26.3 snapshot in entity/player metadata, effects, fluid behavior, movement rules and portal mechanics. Later effects/fields must be filtered; armor and multiplayer interactions need independent coverage.
- Follow-up checks: Partition tests by movement, attributes, food/health, fluids, powder snow, portal, effects and block mutation; validate every event/state transition against Java 1.20.1, and add armor/vehicle/multi-entity scenarios absent here.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/scene_motion_probe.rs` (proposed, not created); production mapping spans player physics, health, attributes and fluid simulation, not a single module.
- Evidence required before adoption: Pinned Java 1.20.1 oracle vectors per operation and event order, registry/effect maps, plus deterministic integration tests in loaded chunks.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in consecutive sections 1–400 and 401–723 (723 logical lines). No tests were executed.

### 1007. `crates/horizon-client/examples/probe/scoreboard_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4a2534b328204d6cb79b8f9548fef6cbac2b8d45`
- Size: 16998 bytes; 430 logical lines
- Coverage: Complete source read in consecutive sections, maximum 400 lines per section.
- Purpose: Comprehensive scoreboard differential fixture covering objectives, score holders/values, display slots, teams, player memberships, criteria, number formats and NBT save/reload.
- Dependencies / cross-references: Uses Scoreboard, criteria registries, components, NumberFormat, team visibility/collision policies, NBT read/write and optional vanilla-produced NBT for a reload comparison.
- Implementation / logic: Replays objective/team/score operations; dumps streamed/listed order, tracked holders, slots and membership; packs NBT, encodes/decodes it, reloads the scoreboard and emits canonical JSON and hex.
- Findings: The fixture has test-only trust assumptions: several args/fields use unwrap/expect/panic; malformed hex in unhex is converted to zero bytes. The newer NumberFormat and component representations are version-specific and may not exist in 1.20.1.
- RustChunk relevance: Scoreboard semantics and NBT persistence are useful for multiplayer HUD parity; the deterministic pack/reload oracle is adaptable. The newer number-format schema must not leak into the 1.20.1 baseline.
- Decision: **adaptable; reference; version-specific — retain the scoreboard/NBT parity method but rebuild criteria, packet data and persisted schema for 1.20.1.**
- 1.20.1 implications: Reconstruct 1.20.1 objectives, criteria, display slots, teams and score NBT from Java; drop later NumberFormat/score-display features unless present in the target version.
- Follow-up checks: Add 1.20.1 golden cases for duplicate objectives, read-only criteria, team transfer, display slots, score reset/lock, integer overflow, invalid NBT and pack/unpack stability.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/scoreboard_fixture.rs` (proposed, not created); runtime mapping is protocol scoreboard packets plus HUD state and world-save metadata.
- Evidence required before adoption: Java 1.20.1 scoreboard state/packet/NBT golden vectors and malformed-input tests; no test is claimed executed here.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in consecutive sections 1–400 and 401–430 (430 logical lines). No tests were executed.

### 1008. `crates/horizon-client/examples/probe/screen_layout_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1103f36476e797c9c9a15b5f945cdd0bf067438a`
- Size: 5310 bytes; 138 logical lines
- Coverage: Complete text file reviewed, lines 1–138.
- Purpose: Headless GUI draw oracle for Loom and Crafter screens; converts screen-layout draw calls into canonical JSON for comparison with vanilla GUI output.
- Dependencies / cross-references: Uses banner layer/color/loading and screen_layout LoomView/CrafterView/ScreenDraw, serde_json, resource path and draw pose/color/atlas helpers.
- Implementation / logic: Builds loom inputs including pattern choices, banner layers, dye, scroll and mouse state; alternatively builds crafter disabled-slot/powered state; serializes gradients, blits, sprites, banner layers, tooltips, fills and atlas UVs.
- Findings: The fixture merges two screens but their target-version status differs. Loom exists in 1.20.1; Crafter was introduced after 1.20.1 and must be excluded from the 1.20.1 baseline. Draw output also depends on newer atlas IDs/assets.
- RustChunk relevance: Loom layout and banner rendering are useful UI parity references. Crafter-specific code/cases are future-version only and must not be treated as vanilla 1.20.1 behavior.
- Decision: **adaptable; reference; version-specific — keep the Loom draw-oracle pattern; explicitly exclude Crafter from the 1.20.1 baseline.**
- 1.20.1 implications: Rebuild Loom textures, atlas IDs, pattern data and layout from 1.20.1. Do not port the Crafter branch as a vanilla 1.20.1 feature.
- Follow-up checks: Compare Loom width/height, pattern pages, scroll bounds, dyes, banner layers and mouse hover in Java 1.20.1. Keep Crafter tests in a separately version-gated suite.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/loom_screen_fixture.rs` (proposed, not created); no Crafter runtime mapping for 1.20.1.
- Evidence required before adoption: Java 1.20.1 Loom screen golden images/draw calls, asset/license verification and input/scroll edge cases.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 138 logical lines. No tests were executed.

### 1009. `crates/horizon-client/examples/probe/section_mesh_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `90210e31c0337fc6d654d36a388d406d360f3f4d`
- Size: 9884 bytes; 251 logical lines
- Coverage: Complete text file reviewed, lines 1–251.
- Purpose: Headless parity fixture for terrain section meshing; builds synthetic 3×3 chunk scenes and emits canonical per-quad geometry, sprite names, ARGB colors and packed light coordinates.
- Dependencies / cross-references: Uses Assets/RenderAssets, BlockFacts, Biomes, Chunk/Light data, MeshTables, Mesher, visibility and biome zoom tables, block-state registry and JSON scene cases.
- Implementation / logic: Constructs chunks, light/biome samples and block placements; runs Mesher::mesh for selected sections; validates quad shape and sprite-layer consistency; serializes four vertices per quad and can export per-state mesh facts.
- Findings: The fixture hardcodes several rendering modes off (shader-pack lighting, separate AO, translucent layers, leaf culling) and uses only solid/translucent outputs. It depends on the pinned model/atlas/biome/state corpus, not a self-contained 1.20.1 renderer.
- RustChunk relevance: Extremely relevant to RustChunk terrain rendering and the user's chunk/FPS goal: gives a way to compare geometry, tint, emissive/light values and per-state facts. Must be version-adapted and not copy current registry facts.
- Decision: **adaptable; reference; version-specific — prioritize as a parity-harness design; rebuild BlockFacts, sprite/model tables, biome data and expected geometry from Java 1.20.1.**
- 1.20.1 implications: 1.20.1 model baking, AO, tint, leaves, fluid faces, vertex color/light packing and block-state registries must be compared to target Java. The current fixture intentionally omits several rendering extensions.
- Follow-up checks: Add tests for model rotations, UV lock, block offsets, faces, AO, leaves, fluid surfaces, translucent ordering, biome tint, emission, chunk borders and light updates; verify mobile GPU output separately.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/section_mesh_fixture.rs` (proposed, not created); production mapping is the block-model/mesh/section renderer with data sourced from the 1.20.1 preparer.
- Evidence required before adoption: Java 1.20.1 golden vertex/UV/color/light outputs and image comparisons for deterministic scenes; independent Mali-G52 GPU tests before performance claims.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 251 logical lines. No tests/build or GPU execution was performed.

### 1010. `crates/horizon-client/examples/probe/server_files_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `85b33a764232487c5fbaa26f8d69f127d0313960`
- Size: 13902 bytes; 367 logical lines
- Coverage: Complete text file reviewed, lines 1–367.
- Purpose: Differential fixture for server-side file behavior: server.properties, EULA text, operators, whitelist, player/IP bans and scripted list mutations.
- Dependencies / cross-references: Uses Horizon server property parsing/storage, EULA parser, user-list types, UUID parsing, entropy, filesystem and Java-like line/Latin-1 normalization.
- Implementation / logic: Copies each case into a temp work directory, loads/stores properties and EULA, applies op/whitelist/ban/pardon operations to four JSON lists, prints normalized settings and serialized lists, then removes temp data.
- Findings: This is server-administration logic, not Minecraft client runtime. It includes newer management-server/code-of-conduct/transfer-related properties not in 1.20.1. The temporary directory name is PID plus case name and is recursively removed before use; safe only for isolated test fixtures, not production/user data.
- RustChunk relevance: Low priority for RustChunk's client target. Classic properties, EULA parsing and JSON user-list formats can inform separate server tooling, but do not belong in the mobile game client core.
- Decision: **unnecessary; reference; version-specific — exclude the server-file administration fixture from the RustChunk client baseline; reuse only generic parsing ideas if a separate server-management tool is ever planned.**
- 1.20.1 implications: Do not implement dedicated-server property administration in the client. Any future helper must use the exact 1.20.1 property/list schema and must never mutate actual user worlds/server folders through a parity fixture.
- Follow-up checks: If retained as tooling, replace the predictable temp-root deletion pattern with an isolated unique temp directory; add symlink/race and cleanup tests, and isolate secret-bearing property fixtures.
- RustChunk implementation mapping: No production module. Optional isolated test tool only under a separate server-tools scope (proposed; not created); never target user game or server directories.
- Evidence required before adoption: Separate product decision plus Java 1.20.1 server property schema and safe filesystem behavior; security tests for temp-directory isolation before any reuse.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 367 logical lines. No test or server process was executed.

### 1011. `crates/horizon-client/examples/probe/situational_music_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `898893c09b574c08b3af4a33847dfb3b958a88bc`
- Size: 1766 bytes; 47 logical lines
- Coverage: Complete text file reviewed, lines 1–47.
- Purpose: Differential harness for selecting situational background music and computing the music volume from client world/screen conditions.
- Dependencies / cross-references: Uses MusicTables, Situation, BackgroundMusic, Music and sound-event name normalization; consumes compact whitespace-separated oracle inputs.
- Implementation / logic: Loads music tables, derives in-world/background/End/boss/credits/creative/underwater flags, calls Situation.music() and volume(), then prints selected normalized sound, delay bounds, replace flag and float bits.
- Findings: The harness is concise but depends entirely on newer snapshot music table contents and flag precedence. Several input fields are positional and parsed without robust validation.
- RustChunk relevance: Useful for client ambience and audio policy parity, but not required to prove networking or basic world rendering. Selection logic can be compared to 1.20.1 vanilla.
- Decision: **adaptable; reference; version-specific — reuse the deterministic test model but source music definitions and situational rules from 1.20.1.**
- 1.20.1 implications: Rebuild music event IDs, delays and conditions for 1.20.1; exclude new dimension/effect/boss situations that do not exist in the target release.
- Follow-up checks: Test credits/menu/world priority, dimension changes, boss music, creative, underwater, empty/missing music tables and category volume against Java 1.20.1.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/situational_music_fixture.rs` (proposed, not created); production mapping belongs to audio selection, not the Android playback backend.
- Evidence required before adoption: 1.20.1 music JSON/event registry and Java oracle selection/volume outputs; separate Android audio focus/latency tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 47 logical lines. No audio playback was executed.

### 1012. `crates/horizon-client/examples/probe/skin_image_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e85be6a16b7ea96e31cbed022c4fc53826d121fd`
- Size: 1216 bytes; 32 logical lines
- Coverage: Complete text file reviewed, lines 1–32.
- Purpose: Headless fixture for decoding base64 PNG player skins and transforming legacy skin dimensions/layouts into normalized RGBA data or refusing invalid images.
- Dependencies / cross-references: Uses base64 decode, image decode, skins::process_legacy_skin and a line-oriented file input/output protocol.
- Implementation / logic: Reads each nonempty base64 input, decodes PNG bytes, decodes image dimensions/RGBA, calls process_legacy_skin and writes base64 RGBA or `refused`.
- Findings: The fixture tests legacy skin normalization only; it does not verify online texture signature validation, profile/session trust, URL fetching, caching or rendering on Android. Image size/dimension limits are enforced downstream and should be confirmed.
- RustChunk relevance: Useful for player skin compatibility and for keeping image decoding deterministic; secondary to core world rendering but needed for multiplayer polish.
- Decision: **adaptable; reference; version-specific — retain the image-normalization test pattern and verify accepted legacy dimensions against 1.20.1 behavior.**
- 1.20.1 implications: Use 1.20.1 skin layout acceptance and profile texture rules; do not treat PNG decode success as proof of signed Mojang texture authenticity.
- Follow-up checks: Test classic 64×32 and 64×64 layouts, malformed base64/PNG, huge dimensions, decompression-bomb limits, alpha channels and signed/unsigned profile handling separately.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/skin_image_fixture.rs` (proposed, not created); production mapping is skin decode/normalization behind a bounded image loader.
- Evidence required before adoption: Java 1.20.1 accepted/rejected skin fixtures, memory-limit tests and independent signature/URL policy tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 32 logical lines. No image decode test was executed.

### 1013. `crates/horizon-client/examples/probe/sound_engine_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `327a8e9d1465a6fd80b6e236fe759417f7e42fe2`
- Size: 6621 bytes; 145 logical lines
- Coverage: Complete text file reviewed, lines 1–145.
- Purpose: Replay harness for the client SoundEngine state machine, recording what a fake OpenAL-like backend would hold after sound operations.
- Dependencies / cross-references: Uses SoundEngine, SoundRegistry, SoundInstance, attenuation, cart sound contexts, legacy RNG and shared audio_common fake backend.
- Implementation / logic: Replays play/replay/queue/delayed/fade/stop/matching/finish/gain/volume/refresh/pause/resume/player/cart/ride/freeze/tick/forget commands and serializes instance/channel/backend state after each step.
- Findings: The fake backend is a useful deterministic boundary but cannot prove OpenAL/AAudio behavior. Sound event catalogues, source categories, cart behavior and operation timing come from the pinned newer snapshot.
- RustChunk relevance: Good architecture and regression model for the client audio manager; helps isolate sound scheduling/state from device playback. Requires a target-specific registry and separate Android backend tests.
- Decision: **adaptable; reference; version-specific — preserve the fake-backend state-machine testing strategy; rebuild 1.20.1 sound events, categories and call order.**
- 1.20.1 implications: 1.20.1 sound event mappings, subtitles, category volumes and special loops must be regenerated; later events or behavior cannot be imported by name alone.
- Follow-up checks: Add deterministic cases for replacement, delayed play, pause/freeze, category gain, loop lifetimes, minecart removal, listener underwater/removed, channel exhaustion and backend failure.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/sound_engine_fixture.rs` (proposed, not created); runtime audio should be an interface with Android implementation separate from sound policy.
- Evidence required before adoption: Java 1.20.1 sound engine event/loop golden traces and Android device playback/latency/focus tests on target hardware.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 145 logical lines. No test or audio backend was executed.

### 1014. `crates/horizon-client/examples/probe/sound_event_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6b52dfc9cb7b06666a3300476ab845feeec8acdd`
- Size: 3093 bytes; 78 logical lines
- Coverage: Complete text file reviewed, lines 1–78.
- Purpose: Probe for sound-event registry resolution, including selected sound file, sentinel state, subtitle, weight, stream/preload flags, attenuation and RNG progression.
- Dependencies / cross-references: Uses SoundRegistry, SoundEngine, SoundInstance, legacy RNG and normalized event identifiers.
- Implementation / logic: Loads and lists all registered names, creates a deterministic sound instance per input, resolves it, emits event/subtitle and sound metadata, computes effective volume/pitch/attenuation range and next RNG value, then releases the instance.
- Findings: The registered event list and file references are data-version specific. The output only inspects resolution; it does not decode/play audio or verify actual resource files and codecs.
- RustChunk relevance: Useful compact oracle for sound-resource resolution and deterministic choice, but must remain separate from playback/device correctness.
- Decision: **adaptable; reference; version-specific — keep the registry-resolution test pattern while rebuilding the event/sound JSON for Java 1.20.1.**
- 1.20.1 implications: Reconstruct 1.20.1 sound event identifiers, weighted entries, subtitles, streaming/preload and attenuation values. Later registry entries must be excluded.
- Follow-up checks: Test missing/empty/intentional-empty events, weighted random selection, file-vs-event references, invalid resource paths and deterministic RNG across seeds.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/sound_event_fixture.rs` (proposed, not created); production mapping is sound registry loading and audio event resolution.
- Evidence required before adoption: Pinned Java 1.20.1 sounds.json plus oracle output vectors; resource existence/format validation and Android playback tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 78 logical lines. No sound was played.

### 1015. `crates/horizon-client/examples/probe/spawn_rules_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f5cdc53d77901d81d9e32f5525e3fb887c2bf63b`
- Size: 7590 bytes; 234 logical lines
- Coverage: Complete text file reviewed, lines 1–234.
- Purpose: Differential fixture for mob spawn placement and spawn-predicate checks using a minimal single-column world.
- Dependencies / cross-references: Uses Assets, BlockFacts, Items block tags, MobData/MobTypes and spawn_rules traits; models block states, biome tags, light, dimension, sea/surface level, seed, difficulty and nearby player position.
- Implementation / logic: Parses per-case key/value inputs and optional column block states, builds a minimal SpawnLevel implementation, calls is_spawn_position_ok and check_spawn_rules with a seeded legacy RNG, then records result and RNG progression.
- Findings: The fixture covers many useful predicates but only a one-column synthetic world; it includes a `trial_spawner` reason introduced after 1.20.1 and relies on data/tag tables from the newer snapshot. Unknown reason strings fall back to Natural, which can hide fixture typos.
- RustChunk relevance: Useful for parity of spawn placement, light/dimension/biome/player-distance checks and deterministic RNG. The single-column adapter is a test oracle, not full spawn simulation or cap management.
- Decision: **adaptable; reference; version-specific — retain natural/spawner predicate harness design; remove Trial Spawner and rebuild 1.20.1 spawn data and rule reasons.**
- 1.20.1 implications: 1.20.1 natural spawning, spawner/reinforcement/patrol/structure/event/command reasons need their target Java semantics; trial-spawner is outside baseline. Actual mob caps and chunk-level spawn scheduling require separate tests.
- Follow-up checks: Add 1.20.1 golden cases for valid/invalid ground, block/sky light, dimension, biome/block tags, peaceful difficulty, player reach, surface slime checks, unsupported types and random-call order.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/spawn_rules_fixture.rs` (proposed, not created); runtime mapping is mob spawning predicates and later chunk-level spawn scheduling.
- Evidence required before adoption: Java 1.20.1 spawn-rule source/data and seeded oracle outputs; full chunk/cap integration fixtures and server-authoritative checks.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 234 logical lines. No tests were executed.

### 1016. `crates/horizon-client/examples/probe/sprint_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5c161da4f373093a44eb6db291d51d31df7f72c2`
- Size: 2958 bytes; 85 logical lines
- Coverage: Complete text file reviewed, lines 1–85.
- Purpose: Focused state-machine fixture for client sprint input and start/stop transitions.
- Dependencies / cross-references: Uses SprintState, Conditions, Keys, JSON scripted flags and input bit masks.
- Implementation / logic: Initializes trigger time, previous forward/shift state and sprinting; replays per-step keys/environment conditions into SprintState::tick and records state changes, trigger times, direction bit patterns and setter calls.
- Findings: The test cleanly isolates sprint transition ordering but maps many environment flags manually. It uses `analog: None`, so controller/analogue movement is not covered; the conditions list includes later movement flags.
- RustChunk relevance: Directly relevant to responsive movement on Android and remote-player state parity. Test structure is reusable, but target conditions/input semantics need 1.20.1 comparison.
- Decision: **adaptable; reference; version-specific — retain as a deterministic sprint transition test model; reconstruct the 1.20.1 condition set.**
- 1.20.1 implications: Compare sprint key double-tap, hunger, swimming, flight, crouching, collision, passenger and item-use restrictions against Java 1.20.1; exclude later-only predicates.
- Follow-up checks: Test key press/release sequencing, simultaneous shift/forward, hunger threshold, underwater states, analog sticks, controller input and server correction.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/sprint_probe.rs` (proposed, not created); production mapping is player input/sprint state, with Android touch mapped into the same key contract.
- Evidence required before adoption: Java 1.20.1 sprint-state oracle traces, input cancellation/lifecycle tests and Android multi-pointer/controller vectors.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 85 logical lines. No test was executed.

### 1017. `crates/horizon-client/examples/probe/squid_client_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b0b0aa772fcab39d2a68bed56e4f0516436a021d`
- Size: 1794 bytes; 51 logical lines
- Coverage: Complete text file reviewed, lines 1–51.
- Purpose: Small client-side animation probe for squid movement, testing the local visual animation step while preserving network-authoritative velocity.
- Dependencies / cross-references: Uses MobTypes, Mob, Vec3 and aquatic::squid_cycle_event/squid_client_step; reads JSON rows and emits yaw/animation values.
- Implementation / logic: Loads a hardcoded `.local/horizon-client/26.3` resource root, builds a squid mob with supplied water/velocity/body yaw/animation state, optionally resets cycle animation, runs squid_client_step and asserts velocity unchanged.
- Findings: The velocity-preservation assertion is a valuable authority boundary. The fixed 26.3 resource path harms portability/reproducibility; species data and animation parameters must be sourced for 1.20.1.
- RustChunk relevance: Relevant to mob visual fidelity and correct separation of client animation from server velocity. Only the state-update/test approach is reusable.
- Decision: **adaptable; reference; version-specific — retain the velocity authority invariant, replace the hardcoded newer resource root and use 1.20.1 squid data.**
- 1.20.1 implications: Rebuild squid animation timing and entity metadata from Java 1.20.1. Do not use later entity catalog data; never overwrite velocity received from the server during client-only animation.
- Follow-up checks: Make resource root an explicit argument; test in-water/out-of-water, cycle reset and repeated steps; assert position/velocity/network snapshots remain unchanged.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/squid_client_probe.rs` (proposed, not created); runtime mapping is client-only squid animation.
- Evidence required before adoption: Java 1.20.1 client animation traces and explicit invariance tests for packet-owned velocity; loaded-resource reproducibility.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 51 logical lines. No test was executed.

### 1018. `crates/horizon-client/examples/probe/storage_menu_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2298e1360dfc640f53c31dad3a1e5dd21ebe1d25`
- Size: 1248 bytes; 37 logical lines
- Coverage: Complete text file reviewed, lines 1–37.
- Purpose: Tiny parity fixture mapping block IDs to the container menu the server should open and whether the menu permits inserting a shulker box.
- Dependencies / cross-references: Uses only `block_menu_type` and line-oriented input/output; behavior is based on known block menu types and shulker-box restriction.
- Implementation / logic: Reads one block identifier per line, trims it, resolves its block-menu type, emits menu type plus accepts/refuses shulker box, or `error` for unknown blocks.
- Findings: The fixture is intentionally narrow and does not model all slot layouts, inventories or menu packets. The shulker-box-in-shulker-box restriction and most container mappings are in 1.20.1 scope, but mapping tables must be rebuilt from target registries.
- RustChunk relevance: Low-cost but relevant multiplayer/container correctness test; can prevent invalid inventory actions and client/server menu mismatch.
- Decision: **adaptable; reference — keep the simple table-driven parity idea and generate the block-to-menu mapping from 1.20.1.**
- 1.20.1 implications: Confirm 1.20.1 menu IDs and supported containers; later-added containers and block IDs must be excluded. Validate the restriction against the server's actual slot mayPlace rule.
- Follow-up checks: Test every 1.20.1 container block, unknown IDs, renamed/legacy IDs and shulker nesting across all valid slots; later add packet/menu state tests.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/storage_menu_fixture.rs` (proposed, not created); production mapping is block menu resolution and slot validation.
- Evidence required before adoption: 1.20.1 registry/menu IDs and Java server slot acceptance oracle, plus inventory packet interaction tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 37 logical lines. No test was executed.

### 1019. `crates/horizon-client/examples/probe/subtitle_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8f690f1303aca1f516547db816a7573ea346b354`
- Size: 5296 bytes; 121 logical lines
- Coverage: Complete text file reviewed, lines 1–121.
- Purpose: Headless differential fixture for the subtitle overlay: replays sound events and frames and serializes text/fill draw calls with pose transforms.
- Dependencies / cross-references: Uses SubtitleOverlay/SubtitleFrame, HudAtlas, language data, GUI background color, pose stack helpers and sound event callbacks.
- Implementation / logic: Processes time, play and frame operations; registers sounds only when listening, computes range/looping state, extracts subtitle calls, applies push/pop transforms and emits text/fill/stratum draw commands.
- Findings: The test covers camera-relative subtitle presentation, expiration and listener visibility but does not render through an Android canvas/GPU. Text component styling is flattened to plain strings in the serialized output, and sound keys/data are versioned.
- RustChunk relevance: Useful reference for readable audio accessibility/UI and subtitle lifetime. Needs 1.20.1 sound subtitle mapping and richer text parity where styles matter.
- Decision: **adaptable; reference; version-specific — retain overlay timing/pose harness; rebuild sound keys, text runs and resource labels from 1.20.1.**
- 1.20.1 implications: Verify 1.20.1 subtitle keys and sound event metadata; do not import later sounds or UI defaults. Preserve camera-space transform and background-opacity semantics only if Java oracle agrees.
- Follow-up checks: Test listening toggles, looping/non-looping sounds, range cutoff, time expiry, concurrent subtitles, pose stack nesting and styled text; add Android screenshot/golden tests.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/subtitle_fixture.rs` (proposed, not created); runtime mapping is audio subtitle extraction plus HUD overlay drawing.
- Evidence required before adoption: Java 1.20.1 subtitle/sound fixtures and rendered device screenshots at multiple GUI scales; text styling parity tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 121 logical lines. No tests were executed.

### 1020. `crates/horizon-client/examples/probe/tab_list_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ea822fbaa15fa54a08791f9a6053206a7fcbbe7`
- Size: 8868 bytes; 200 logical lines
- Coverage: Complete text file reviewed, lines 1–200.
- Purpose: Headless parity fixture for the multiplayer player list: constructs teams/scores/player identities/skins/header/footer and serializes per-frame GUI draw calls.
- Dependencies / cross-references: Uses Scoreboard, TabList/TabInputs/TabPlayer, HUD font atlas, language, component/text conversion, skin profile parsing/signatures/download states and JSON cases.
- Implementation / logic: Builds teams/objective/scores, parses player UUID/game mode/latency/display/list-order/hat/texture data, resolves local/remote skin download state, computes row background and emits fill/sprite/text/blit calls for visible frames.
- Findings: The fixture mixes stable player-list presentation with later fields such as list-order and show-hat behavior; current NumberFormat and skin texture structures are also tied to the pinned newer protocol. It simulates download states rather than networking.
- RustChunk relevance: Important multiplayer UX reference for tab list, latency, game mode, teams, score objective, profile skin and overlay rendering. The display model and packet fields must be adapted to 1.20.1.
- Decision: **adaptable; reference; version-specific — keep the presentation-test architecture, but reconstruct 1.20.1 PlayerInfo, scoreboard, skin and ordering semantics.**
- 1.20.1 implications: Do not assume 1.20.1 sends modern list-order/show-hat fields or modern number-format data. Rebuild player-list update actions and skin profile handling from exact 1.20.1 protocol/source.
- Follow-up checks: Test ordering/tie-breaks, hidden/unlisted players, latency, game mode, local profile, signed/unsigned/invalid textures, pending/failed downloads, scoreboard hearts/integer display and header/footer wrapping.
- RustChunk implementation mapping: QA-only candidate `tests/parity/starling/tab_list_fixture.rs` (proposed, not created); production mapping spans protocol PlayerInfo, scoreboard state, skin resolver and HUD tab-list renderer.
- Evidence required before adoption: Java 1.20.1 PlayerInfo packet oracle, valid profile texture/signature cases, scoreboard fixtures and Android screenshots with multiple GUI scales.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full, 200 logical lines. No tests were executed.

### 1021. `crates/horizon-client/examples/probe/team_rules_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9a2d716ffc42f1c1b2e0b575d39fdb890465c5df`
- Size: 2,411 bytes; 50 logical lines
- Coverage: Complete text file read to EOF, lines 1–50.
- Purpose: Small differential fixture for scoreboard-team rules: friendly fire, alliances, entity pushability, invisibility and name-tag visibility.
- Dependencies / cross-references: Uses `scoreboard::{CollisionRule, NameView, Scoreboard, Visibility}`; the inputs represent two teams plus own/other/viewer roles, spectator/HUD/camera/vehicle flags and squared distance.
- Implementation / logic: Creates two teams from the case, sets friendly-fire/invisibility/collision/name-tag policies, resolves the referenced teams, computes `invisible_to`, then serializes team damage/alliance/push/name-visibility answers.
- Findings: Deterministic logic-only probe. It does not exercise the server's authoritative damage or collision packets, and it has no entity/world simulation. Enum names and defaults are code-level contracts that need 1.20.1 oracle confirmation.
- RustChunk relevance: Useful, inexpensive parity harness for multiplayer name visibility and team interactions; suitable for a focused RustChunk scoreboard test, not a complete PvP implementation.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Team rules exist in 1.20.1, but rebuild team collision, visibility and friendly-fire behavior against Java 1.20.1; ignore any new team properties absent from that version.
- Follow-up checks: Compare all collision-rule pairs, friendly-fire combinations, spectator and invisibility cases, name-tag range boundaries, HUD-hidden/camera/vehicle cases and unassigned teams against Java 1.20.1.
- RustChunk implementation mapping: Proposed `tests/parity/starling/team_rules_fixture.rs`; production behavior remains in RustChunk scoreboard/player-collision code. No file created.
- Evidence required before adoption: Java 1.20.1 scoreboard team and entity push/name visibility oracle cases, including boundary-distance vectors. No test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 50 logical lines; source-only review, no runtime or multiplayer test executed.

### 1022. `crates/horizon-client/examples/probe/text_display_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f1f58762586cd1d9dc32ea78c75f4c41a050e4d6`
- Size: 1,535 bytes; 37 logical lines
- Coverage: Complete text file read to EOF, lines 1–37.
- Purpose: Wire-to-model fixture for text-display entity metadata; accepts an encoded entity-data packet and reports the resulting display state or transform matrix.
- Dependencies / cross-references: Depends on `horizon_protocol` Play clientbound IDs and decoder, `SetEntityData`, `TextDisplayData`, serde JSON and the snapshot's entity metadata serializer table.
- Implementation / logic: Hex-decodes the packet payload, decodes `SET_ENTITY_DATA`, applies metadata values to a default TextDisplayData, and either returns its matrix for four supplied angles or serializes display state with plain text and alignment.
- Findings: Text displays were introduced before 1.20.1, but metadata indices/serializer types and protocol packet IDs are version-specific. The fixture uses current Protocol 777 and therefore cannot be used as a 1.20.1 wire decoder unchanged.
- RustChunk relevance: Useful separation of protocol metadata decoding from display-state interpretation; good candidate for two focused tests (1.20.1 metadata codec and text-display model).
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Rebuild SetEntityData ID, metadata field indices, serializer codecs, interpolation/rotation and alignment semantics from Java 1.20.1. The Text Display entity exists in the target version, but packet fields must match 1.20.1.
- Follow-up checks: Use 1.20.1 encoded metadata golden vectors for text, line width, background, opacity, alignment, billboard, brightness and transforms; cover absent/default and partial metadata updates.
- RustChunk implementation mapping: Proposed `tests/parity/starling/text_display_fixture.rs`; runtime decode belongs to RustChunk's 1.20.1 protocol adapter and entity render path. No file created.
- Evidence required before adoption: Java 1.20.1 entity metadata packets and TextDisplay entity source/oracle vectors. No packet test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 37 logical lines; no protocol or rendering test executed.

### 1023. `crates/horizon-client/examples/probe/text_display_render_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2a539e4252095695ffed9ceee6f4dd3d041f37cc`
- Size: 2,925 bytes; 70 logical lines
- Coverage: Complete text file read to EOF, lines 1–70.
- Purpose: Headless text-display render fixture; turns entity metadata into wrapped text lines, pose matrices and canonical HUD quads.
- Dependencies / cross-references: Depends on HudAtlas/font metrics, TextDisplayData, SetEntityData decoding, line splitting, background-color calculation and HudBatch vertex format.
- Implementation / logic: Loads the atlas, decodes each supplied wire packet, applies metadata, splits text at the configured line width, computes background and calls `text_display::draw`; converts batch vertices to four-corner quads with ARGB colors and pose.
- Findings: Output tests the CPU draw-command/quad construction, not a GPU renderer. It serializes a subset of vertex data and relies on the current atlas, fonts and protocol serializer mappings.
- RustChunk relevance: Useful differential-rendering harness for text display once a 1.20.1 metadata decoder and font/resource set are available.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Retain the text measurement and quad-oracle pattern, but rebuild metadata packet, text styling, font glyph metrics, background defaults and transforms from 1.20.1 resources.
- Follow-up checks: Compare empty/multiline/styled text, line-width wrapping, alignment, opacity/background, billboard pose, ARGB conversion and vertex ordering at multiple GUI scales against Java 1.20.1.
- RustChunk implementation mapping: Proposed `tests/parity/starling/text_display_render_fixture.rs`; production renderer remains RustChunk's own HUD/entity render layer. No file created.
- Evidence required before adoption: Java 1.20.1 TextDisplayRenderer oracle, exact font/atlas resources and deterministic metadata cases. No rendering test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 70 logical lines; source-only review, no GPU/device run.

### 1024. `crates/horizon-client/examples/probe/toggle_keys_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d05acdd5c07cbe4805a997ca07b8725ef7961520`
- Size: 2,052 bytes; 56 logical lines
- Coverage: Complete text file read to EOF, lines 1–56.
- Purpose: Event-sequence probe for toggle-style key mappings: sneak, sprint, attack and use behavior across key/button input and screen lifecycle transitions.
- Dependencies / cross-references: Depends on `ToggleKeys`, `KeyAction`, four option flags and a manually encoded operation stream.
- Implementation / logic: Replays option changes, key press/release/repeat, button transitions, screen open/close and respawn; emits each key's down state and pending click count after every event.
- Findings: Pure input state machine, but its key-slot numbering and action details are specific to the source client. It does not test Android touch pointer IDs, controller analog state or OS cancellation.
- RustChunk relevance: Good compact test pattern for separating toggle semantics from platform-specific input dispatch.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Verify exact 1.20.1 toggle options and repeat/press semantics. RustChunk should map touchscreen controls to the same canonical action state, while keeping touch pointer identity/cancel handling separate.
- Follow-up checks: Test press-release-repeat sequences, option toggled while held, focus loss, screen open/close, respawn, duplicate pointer/button events and multi-touch cancellation; compare keyboard behavior with Java 1.20.1.
- RustChunk implementation mapping: Proposed `tests/input/toggle_keys_probe.rs`; canonical key and touch mapping remains RustChunk-owned. No file created.
- Evidence required before adoption: Java 1.20.1 key mapping/ToggleKeyMapping behavior and separate Android ACTION_CANCEL/multi-pointer tests. No test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 56 logical lines; no input or Android-device test executed.

### 1025. `crates/horizon-client/examples/probe/tooltip_display_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7cb3ade954f587ccf3a3a289cc19401a020f1063`
- Size: 2,808 bytes; 70 logical lines
- Coverage: Complete text file read to EOF, lines 1–70.
- Purpose: Fixture for tooltip visibility and styled item-name output, including hide-tooltip and hidden-attribute controls.
- Dependencies / cross-references: Uses `TooltipDisplay`, `ItemStack`, modern item-data components and SNBT/NBT tag construction; also exercises component parsing and styled text spans.
- Implementation / logic: Reads tab-separated cases, optionally parses a custom-name component, constructs a test stack, calls `TooltipDisplay::for_stack` and `shows`, then serializes style flags for name glyphs.
- Findings: Central `minecraft:tooltip_display` component and `hidden_components` representation are from the modern data-component item model and are not the 1.20.1 ItemStack/NBT schema. Reusing this fixture directly would misrepresent legacy tooltip rules.
- RustChunk relevance: Only the high-level tests—hidden tooltip, hidden attribute lines and styled custom name—are conceptually useful; the schema and implementation need a separate legacy 1.20.1 design.
- Decision: **rewrite required; reference; version-specific**
- 1.20.1 implications: Do not carry the modern tooltip-display component into vanilla 1.20.1. Reconstruct item tooltip suppression and attribute visibility from 1.20.1 item NBT/flags and the target client implementation.
- Follow-up checks: Build 1.20.1 tests for custom name formatting, hide flags, attribute modifiers, enchantments, durability, lore, advanced/debug tooltip and creative mode.
- RustChunk implementation mapping: Proposed legacy fixture `tests/parity/starling/legacy_tooltip_fixture.rs`; no modern component runtime mapping.
- Evidence required before adoption: Java 1.20.1 ItemStack tooltip source plus golden outputs for old NBT/hide flags. No tooltip test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 70 logical lines; version-specific component schema flagged; no test executed.

### 1026. `crates/horizon-client/examples/probe/trading_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `36804fba9e92be2326438781135f24759085c66f`
- Size: 20,564 bytes; 556 logical lines
- Coverage: Complete text file read to EOF, lines 1–556; consecutive sections of at most 400 lines were used.
- Purpose: Differential harness for merchant gameplay: generates villager/wandering-trader offers, applies demand/gossip/hero modifiers, restocks merchants, opens trade menus, moves inventory stacks and round-trips villager NBT.
- Dependencies / cross-references: Depends on Items, Enchantments, Recipes, Brewing, Banners, MobTypes, RandomSequences, Gossip, MerchantOffer/Session/TradeEnv, Menu click logic, entity NBT and vanilla oracle text.
- Implementation / logic: Loads trade/recipe/item resources; builds merchant offer sets by profession/variant/level and deterministic random sequences; supports generation, price, restock, NBT and trade cases with selection/click/tick/close operations; emits offers, inventory, XP, gossip, event orbs and next RNG.
- Findings: Large test transport, not a merchant implementation. Inputs and resource tables are snapshot-specific; villager trade pools, professions, variants, trade pricing, random sequence IDs and newer NBT fields must all be rebuilt for 1.20.1. It also trusts several structured fixture fields and uses a synthetic player/origin.
- RustChunk relevance: Useful as a comprehensive parity-test outline for merchant trading once the legacy 1.20.1 trading data and menus are implemented; not appropriate to copy as a monolithic runtime module.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 villager professions/levels/trade pools, wandering-trader offers, demand/reputation/hero pricing, restock timing, trade menu and villager NBT using target version sources. Keep RNG sequence order exact.
- Follow-up checks: Split into deterministic sub-suites for offer generation, special prices, gossip, restock boundaries, inventory/click/result slots, XP/level events and villager NBT round-trip; compare to Java 1.20.1. Include bounds/invalid case handling.
- RustChunk implementation mapping: Proposed tests under `tests/parity/starling/trading/` split by feature; runtime trade/session remains RustChunk-owned. No files created.
- Evidence required before adoption: Java 1.20.1 VillagerTrades, WanderingTrader, MerchantOffers, MerchantMenu, GossipContainer and entity NBT oracle outputs; pinned target trade data and RNG vectors. No test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 556 logical lines read consecutively (1–400, 401–556); no trade, build or server test executed.

### 1027. `crates/horizon-client/examples/probe/translucency_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8ea42c1dce681b848e7b880e031f59c4f29415b8`
- Size: 1,697 bytes; 47 logical lines
- Coverage: Complete text file read to EOF, lines 1–47.
- Purpose: Pure CPU fixture for translucent quad center calculation, distance sorting and sort-cache invalidation as the camera moves.
- Dependencies / cross-references: Depends on `gfx::translucency::{SortState, Sorter, quad_center}` and floating-point inputs for quad vertices/camera/origin.
- Implementation / logic: Reads cases, calculates each quad center, creates a SortState, runs sorter for each camera frame, records sort indices and whether a cached sort update occurred, and serializes center bit patterns.
- Findings: Tests only quad ordering and cache behavior; no wgpu draw, blending, depth buffer, transparency artifacts or GPU synchronization is exercised. The cache threshold/state semantics are source-specific.
- RustChunk relevance: Strong, small parity harness for translucent chunk geometry ordering; suitable for CPU-side RustChunk renderer regression tests.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Confirm Java 1.20.1 translucent quad sorting center formula, distance ordering and camera-relative coordinates; do not inherit later GPU-specific optimizations without visual parity.
- Follow-up checks: Test equal-distance ties, camera crossing, negative chunk origin, degenerate quads, stable ordering and cached-vs-forced resort; compare indices and verify screenshots separately on Android Mali.
- RustChunk implementation mapping: Proposed `tests/render/translucency_sort_fixture.rs`; production sort implementation belongs in RustChunk's renderer. No file created.
- Evidence required before adoption: Java 1.20.1 translucent sorting oracle vectors plus GPU screenshots for blend/depth behavior. No sort or GPU test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 47 logical lines; no build, sort run or GPU/device validation.

### 1028. `crates/horizon-client/examples/probe/trident_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bf4e1392d7d79a9a1ad6c4ea83dd0fa084204a53`
- Size: 5,183 bytes; 130 logical lines
- Coverage: Complete text file read to EOF, lines 1–130.
- Purpose: Resident-scene probe for thrown-trident block collision, `AbstractArrow` tick movement, block-hit stop position, in-ground state and shake duration.
- Dependencies / cross-references: Depends on Assets/RenderAssets, World::test_scene, collision binding/BlockFacts, arrow block clipping, Vec3 bit parsing and JSON operations.
- Implementation / logic: Builds a small world scene and writes its descriptor; replays block placement/property edits and projectile operations; on each tick decrements shake, clips block collision, pulls back/stops/sticks on hit, or advances position and applies drag/gravity on miss.
- Findings: Only the arrow's block collision/movement segment is modeled. Owner, pickup, entity hit, enchantments, loyalty, return-to-owner, fluid drag, server authority and network replication are ignored. Uses newer resources/model gallery.
- RustChunk relevance: Useful narrow projectile collision parity test, but not a full trident gameplay or multiplayer implementation.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Rebuild projectile collision, drag/gravity, in-ground/shake timing and block-shape behavior against Java 1.20.1. Exclude later trident data/components and test the full loyalty/riptide behavior separately if targeted.
- Follow-up checks: Compare hit/miss and boundary intersections against Java 1.20.1; then add separate tests for entity collisions, pickup/owner restrictions, loyalty return, water and enchantments. Keep server-authoritative hit resolution distinct.
- RustChunk implementation mapping: Proposed `tests/parity/starling/trident_probe.rs`; runtime mapping is RustChunk projectile collision and entity gameplay. No file created.
- Evidence required before adoption: Java 1.20.1 AbstractArrow/ThrownTrident oracle traces, target block shapes and exact RNG/float vectors. No projectile test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 130 logical lines; no projectile or multiplayer test executed.

### 1029. `crates/horizon-client/examples/probe/vehicle_container_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `48942be5b184ec2466e300f72d875cf38b3a0441`
- Size: 1,314 bytes; 33 logical lines
- Coverage: Complete text file read to EOF, lines 1–33.
- Purpose: Small entity-type to menu mapping fixture for container vehicles, including menu kind, slot count and display-name translation key.
- Dependencies / cross-references: Depends on `containers::vehicle_menu` and `vehicle_menu_name`, entity type identifiers and menu naming rules.
- Implementation / logic: Reads one entity type per line, resolves vehicle menu and size, returns title key or `error` for unsupported types.
- Findings: Fixture does not model inventory contents, click semantics, packet flow or vehicle entity behavior. Exact supported vehicle types and sizes vary by version; later minecart/boat forms must be checked against 1.20.1.
- RustChunk relevance: Useful narrow client/server agreement test for opening chest-boat/chest-minecart/hopper-minecart containers if those target entities are supported in the exact version.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Rebuild entity type allowlist, menu identifiers, slot counts and title keys from Java 1.20.1; exclude any vehicle type introduced later.
- Follow-up checks: Verify chest boat, chest minecart and hopper minecart menu type/size/title mapping plus unknown entity IDs against Java 1.20.1; add packet/open/close and inventory synchronization cases separately.
- RustChunk implementation mapping: Proposed `tests/parity/starling/vehicle_container_fixture.rs`; production mapping resides in RustChunk entity/container systems. No file created.
- Evidence required before adoption: Java 1.20.1 entity classes `ChestBoat`, `Minecart`, `HopperMinecart`, container menu constructors and translation keys. No test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 33 logical lines; mapping inspected; no vehicle/menu test executed.

### 1030. `crates/horizon-client/examples/probe/weather_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b85dd3e097c1631060b239144ed004893eeebcff`
- Size: 2,402 bytes; 67 logical lines
- Coverage: Complete text file read to EOF, lines 1–67.
- Purpose: Headless weather-render fixture for deterministic rain/snow column placement, UV offsets, light coordinates and emitted quad vertices.
- Dependencies / cross-references: Depends on `weather` column RNG/geometry, parameter cache, camera position, radius/intensity and float-bit serialization.
- Implementation / logic: Replays text cases, seeds per-column legacy random from X/Z, computes rain or snow column, asserts ParameterCache equals direct generation, computes camera-facing quad and serializes positions/UV/color/light as exact bits.
- Findings: CPU-side geometry and random sequence only; no GPU blending, texture sampling, weather audio or biome/weather transition is tested. Exact column range, brightness and snow/rain rules are snapshot-sensitive.
- RustChunk relevance: Good deterministic render-parity shape for precipitation, but target data, formulas and screenshot validation must be rebuilt for 1.20.1.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Use 1.20.1 precipitation rules and climate/light behavior; confirm column geometry, tick interpolation and legacy RNG sequence from target Java sources. Keep the parameter cache only if measured beneficial on Android.
- Follow-up checks: Test rain/snow, negative coordinates, camera offsets, light extremes, partial ticks, radius edges, cache equivalence and biome/weather switches against Java 1.20.1; run a separate Mali screenshot/performance pass.
- RustChunk implementation mapping: Proposed `tests/parity/starling/weather_fixture.rs`; production renderer belongs in RustChunk's render/weather subsystem. No file created.
- Evidence required before adoption: Java 1.20.1 WeatherEffectRenderer/LevelRenderer oracle values, resource textures and Android GPU screenshots. No test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 67 logical lines; no weather or GPU test executed.

### 1031. `crates/horizon-client/examples/probe/world_border_render_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `535118959241217cf1933f18098f2640e3f0aa97`
- Size: 5,373 bytes; 111 logical lines
- Coverage: Complete text file read to EOF, lines 1–111.
- Purpose: Headless parity fixture for world-border state and visual output: interpolation, bounds, fade, render geometry, texture transforms and vignette warning.
- Dependencies / cross-references: Uses WorldBorder/BorderStatus, BorderRenderer/BorderRenderState, hud vignette draw extraction, camera/render distance/depth and float-bit canonicalization.
- Implementation / logic: Replays tick/lerp/size/center/warning/invalidate/frame operations, records border bounds and dimensions/status, extracts render state, prepares border vertices/transforms/sides and computes vignette color.
- Findings: Pure state/render preparation; no GPU shader/blending or actual screenshot. The test assumes fixed vignette viewport 427×240 and newer renderer representation; this must be parameterized/normalized for target screen dimensions.
- RustChunk relevance: Useful world-border animation and renderer oracle design; split logical world-border state from GPU-specific geometry.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Rebuild interpolation/status/tick/border-distance semantics from Java 1.20.1 and separately derive border vertex generation/texture animation from the target renderer. Avoid fixed screenshot viewport assumptions.
- Follow-up checks: Test shrinking/growing/stationary, negative center, max-size clamps, warning distance/time, partial tick interpolation, rebuild invalidation and vignette at multiple GUI sizes against Java 1.20.1.
- RustChunk implementation mapping: Proposed `tests/parity/starling/world_border_render_fixture.rs`; production mapping is RustChunk world-border state plus render overlay. No file created.
- Evidence required before adoption: Java 1.20.1 WorldBorder and WorldBorderRenderer oracle vectors; target renderer screenshot comparisons on Android. No test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 111 logical lines; no renderer or device test executed.

### 1032. `crates/horizon-client/examples/probe/world_options_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5bd3026bc5aa664274b96b7a93cce53d49ec8fd5`
- Size: 1,501 bytes; 40 logical lines
- Coverage: Complete text file read to EOF, lines 1–40.
- Purpose: Packet-payload fixture for difficulty changes, difficulty lock and game-rule set/value messages.
- Dependencies / cross-references: Depends on the source snapshot's Play packet types/IDs and `Reader`/packet encoder.
- Implementation / logic: Translates tab-separated inputs into Serverbound ChangeDifficulty/LockDifficulty/SetGameRule or Clientbound GameRuleValues packets, then strips the packet ID and outputs body bytes.
- Findings: Protocol 777 packet catalogue and payload schemas are not 1.20.1. Game-rule update packet shape, direction, field order, IDs and supported rules must be rebuilt; output is not a reusable protocol vector.
- RustChunk relevance: Only the idea of small packet body golden-vector fixtures is reusable. Actual bytes must be generated independently for 1.20.1.
- Decision: **rewrite required; reference; version-specific**
- 1.20.1 implications: Implement the dedicated 1.20.1 packet IDs and schemas from authoritative 1.20.1 protocol sources; do not reuse Protocol 777 enum variants or outputs.
- Follow-up checks: Compare set/ack game rule, difficulty change/lock if supported, serverbound/clientbound direction and malformed VarInt/body length against a Java 1.20.1 protocol oracle.
- RustChunk implementation mapping: Proposed `tests/protocol/v1_20_1/world_options_packets.rs`; no source packet types reused.
- Evidence required before adoption: Java 1.20.1 protocol packet ID/schema references and exact encoded payload vectors. No packet test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 40 logical lines; source-only review; no packet encoding test executed.

### 1033. `crates/horizon-client/examples/random_tick_eligibility_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bc5a8ea560245e5a9a51177ff30686a180bbcc54`
- Size: 5,670 bytes; 150 logical lines
- Coverage: Complete text file read to EOF, lines 1–150.
- Purpose: Development benchmark for random-tick section eligibility selection only; compares a full per-chunk scan with a cached `RandomTickSections` path across sparse/dense synthetic chunks and edit patterns.
- Dependencies / cross-references: Depends on prepared block registry and BlockFacts, Chunk/Section palette storage, fast hash map, cache invalidation and `Instant`/`black_box` timing.
- Implementation / logic: Loads registry/facts, chooses active/inactive states, builds 17×17 lit chunk fixtures, then times baseline/cached selection for steady/cold/reload/1%-edit/all-edit workloads with alternating order and warmup samples.
- Findings: Explicitly measures selection cost, not block simulation, random-tick execution, chunk generation, GPU or total frame time. The fixed 17×17 scene and target-specific facts can bias results; `grass_block` is assumed available/random-ticking.
- RustChunk relevance: Useful methodology for measuring cache overhead and invalidation behavior; not a gameplay implementation. Any speedup must be reproduced on the actual RustChunk target and phone.
- Decision: **adaptable; reference; optional/future**
- 1.20.1 implications: Rebuild BlockFacts and chunk state corpus from 1.20.1; keep Java random-tick section eligibility as the correctness oracle. Do not use its 26.3 resource output as 1.20.1 performance evidence.
- Follow-up checks: First assert baseline and cached selected sections are identical for each workload, then run repeated release benchmarks on desktop and Android ARM64/Mali with warmed/cold cases and confidence summaries. No run was performed here.
- RustChunk implementation mapping: Optional dev benchmark `benches/random_tick_eligibility.rs` (proposed only); production cache belongs in RustChunk chunk/tick pipeline.
- Evidence required before adoption: 1.20.1 Java section random-tick eligibility oracle plus repeatable Rust release timing on Galaxy A22 and representative desktop. No benchmark executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 150 logical lines; benchmark source reviewed only; no measurements collected.

### 1034. `crates/horizon-client/examples/resource_memory.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f85a65f8aa74885b83d93591f833f286ab56f819`
- Size: 5,564 bytes; 155 logical lines
- Coverage: Complete text file read to EOF, lines 1–155.
- Purpose: Development memory profiler for resource-loader startup; records counting-allocator live/peak heap and process RSS after each loader stage.
- Dependencies / cross-references: Depends on Assets, BlockFacts/Behaviours, Items, Recipes, Biomes, LightTables, MeshTables, Models, Collisions, MotionResources, VanillaTerrain, system allocator and external `ps`/optional macOS `footprint` command.
- Implementation / logic: Installs a custom global counting allocator, loads resources in game startup order, records live/peak allocation deltas and RSS after registry/assets/facts/items/recipes/biomes/light/mesh/collision/motion/worldgen, then keeps resources alive to process exit.
- Findings: Developer-only executable, not runtime logic. RSS from `ps` is platform-specific and allocator live bytes do not equal Android PSS/RSS or total native/GPU memory. Optional `footprint` is macOS-specific. Allocation accounting relies on the global allocator wrapper and only process allocations it observes.
- RustChunk relevance: Useful to identify heavy preparation/runtime resources and compare startup loader stages, but RustChunk memory budgets need Android-native and GPU measurements in addition to allocator counts.
- Decision: **adaptable; reference; optional/future**
- 1.20.1 implications: Resource schemas, state counts and worldgen inputs must be 1.20.1. Use Android `/proc`/PSS or native profiler for device memory; do not copy `ps`/macOS assumptions into the app.
- Follow-up checks: Run isolated processes with disposable 1.20.1 prepared resources; compare live heap, peak, RSS/PSS, GPU texture memory and post-drop retention. Add low-memory scenarios and device measurements. No profiler was run.
- RustChunk implementation mapping: Proposed developer tool `tools/profile_resource_memory.rs` or Android diagnostic hook; not a production game module.
- Evidence required before adoption: Repeatable 1.20.1 data and measurements on target ARM64 device, with OS PSS/native heap/GPU memory and desktop reference. No measurements collected.
- Completion state: FULL TEXT REVIEW COMPLETE — 155 logical lines; profiler source inspected, no profiling or device run.

### 1035. `crates/horizon-client/examples/save_batch_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `30d7629ca1de49903c8d6828abf2863c005642d2`
- Size: 9,699 bytes; 260 logical lines
- Coverage: Complete text file read to EOF, lines 1–260.
- Purpose: Development benchmark for building save snapshots of many chunks with block entities/names; optionally times NBT serialization and always validates exact encoded-byte equality between baseline and optimized snapshot methods.
- Dependencies / cross-references: Depends on Chunk/BlockEntity/ItemStack, ChunkExtras, palette containers, save NBT encoder, BlockRegistry/Biomes and PNG fixtures; it creates a deterministic temp directory under the OS temp root.
- Implementation / logic: Builds 256 synthetic chunks with village/storage block entities and saved NBT overlays, compares old per-chunk entity/name filtering against `chunks_with_block_entities`, asserts matching names and byte-identical NBT, then runs alternating warmup/21-sample median timing.
- Findings: Does not time compression or disk IO, GPU or full world-save workflow. `DATA_VERSION` and component/NBT shapes are snapshot-specific. It deletes a predictable process-specific temp directory at start/end; safe only in an isolated temp root, not a user world. Benchmark can be CPU- and allocator-sensitive.
- RustChunk relevance: Useful architecture and correctness gate for batching save snapshots, especially exact NBT equality before performance claims. Not a production save API itself.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Rebuild chunk extras, block entity NBT, item representation and DataVersion from Java 1.20.1; keep target save layout and chunk serialization separate from modern data-component schema.
- Follow-up checks: Run exact NBT comparison first for 1/16/64/256 chunk batches, preserve unmodelled block entities and ordering, then benchmark snapshot-only and snapshot+serialization on desktop and Android storage separately. Inspect temp path safety before any run.
- RustChunk implementation mapping: Proposed `benches/save_batch.rs` and `tests/save_snapshot_parity.rs`; no files created. Production mapping is RustChunk save/chunk snapshot pipeline.
- Evidence required before adoption: Java 1.20.1 chunk NBT golden files, block entity/item NBT vectors and repeated release benchmarks. No benchmark or filesystem test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 260 logical lines; benchmark source reviewed in full; no benchmark or save operation executed.

### 1036. `crates/horizon-client/examples/shared_json_startup_profile.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c6ab30374a40dc91e8784ae0cf02ce350f1a7b76`
- Size: 3,542 bytes; 76 logical lines
- Coverage: Complete text file read to EOF, lines 1–76.
- Purpose: Development profile comparing alternative load paths for the shared block-facts and gameplay-behaviour JSON tables; collects phase time, total time, state count and Linux RSS.
- Dependencies / cross-references: Depends on Assets, BlockFacts loaders/projections, horizon-world Behaviours loader, JSON parser and `/proc/self/status`.
- Implementation / logic: Supports baseline/pr607/shared/deferred/deferred-held modes, times JSON parse/facts projection/behaviour creation separately in deferred modes, optionally retains parsed JSON/projection and prints state count plus RSS.
- Findings: Only a development profiler and comparator; the `pr607` path is a historical same-build reference, not an authority for 1.20.1. `/proc/self/status` is Linux-specific and measures process RSS, not GPU or Android PSS.
- RustChunk relevance: Useful pattern for evaluating whether shared parsed input reduces startup CPU and peak memory, provided the compared outputs are semantically equal.
- Decision: **adaptable; reference; optional/future**
- 1.20.1 implications: Rebuild the data formats and block/behaviour loaders from 1.20.1. Port the measurement method but add Android-specific memory telemetry; do not carry over 26.3 state counts or performance conclusions.
- Follow-up checks: Before timing, compare every BlockFacts and Behaviours output for semantic equality; measure cold/warm independent process runs and peak RSS/PSS on desktop and Android. No execution occurred.
- RustChunk implementation mapping: Optional tooling `tools/profile_shared_resource_load.rs` (proposed); production implementation remains in RustChunk prepare/resource loaders.
- Evidence required before adoption: 1.20.1 generated data, output equality assertions, repeatable process timings and Android memory measurements. No profiler executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 76 logical lines; no profiler or device run executed.

### 1037. `crates/horizon-client/examples/sprite_cache_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `70f86b4a5c815e84c4e93d22e0278822d659baee`
- Size: 2,112 bytes; 59 logical lines
- Coverage: Complete text file read to EOF, lines 1–59.
- Purpose: Parity gate for disk-backed sprite cache: computes a digest over decoded render pixels, sprite-layer metadata, mipmaps and animation blends, then checks cold-load and warm-cache reload are identical.
- Dependencies / cross-references: Depends on Assets/RenderAssets, sprite cache file `horizon-sprites.cache`, PNG decoders, mip/animation data and deterministic digest inputs.
- Implementation / logic: Loads Assets, deletes `ROOT/horizon-sprites.cache`, reloads to decode/write cache, computes digest, reloads warm cache, computes second digest and reports parity/timings/cache size.
- Findings: **Destructive to the supplied resource root**: it removes the cache file before testing. Must not be run against user data/shared source/resources; only use a disposable copy. Digest uses `DefaultHasher`, so it is a same-build parity checksum, not a stable cross-toolchain cryptographic identity. It does not prove Android filesystem durability.
- RustChunk relevance: Useful cache correctness test pattern, but cache schema and decoded content are version-specific; cleanup strategy must be safer in RustChunk.
- Decision: **adaptable; reference; optional/future**
- 1.20.1 implications: Regenerate cache format and invalidation key for 1.20.1. Run only against a disposable cloned resource root, and prefer a content hash (SHA-256) for reproducible cross-process records.
- Follow-up checks: Use a copy of resources, verify cold decode and warm cache arrays byte-identical, test truncated/corrupt/stale/version-mismatched cache recovery, and confirm the original source/cache remains untouched. No run performed.
- RustChunk implementation mapping: Proposed `tests/assets/sprite_cache_parity.rs`; production cache belongs in RustChunk asset preparation/render subsystem. No file created.
- Evidence required before adoption: 1.20.1 resource sprites/animations, deterministic cache validation vectors and isolated temp-copy test. Android filesystem/cache eviction tests needed before shipping. No test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 59 logical lines; source only; cache was not deleted and no parity test run.

### 1038. `crates/horizon-client/src/accounts.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `314fcfe1d29b78f078abd5740c93907d831e9984`
- Size: 21,092 bytes; 597 logical lines
- Coverage: Complete text file read to EOF, lines 1–597; consecutive sections of at most 400 lines were used.
- Purpose: Client account/session coordinator for switchable Microsoft, Ely.by and offline profiles; manages platform-sealed credentials, current auth session, background renewal, profile/skin actions and downloading official game files.
- Dependencies / cross-references: Depends on external `starling-account` crate, platform keystore/seal/open APIs, private filesystem/logging, auth session global, language, background WorkService, async threads/cancellation, Mojang file manifests/pins and own-skin upload subsystem.
- Implementation / logic: Provides in-game account detection; private account/files directories; keystore-backed per-profile secret files with atomic `.part` rename; current session handoff; State/Files models; lazy saved profile cache; player name/key/skin and cloud status; background renewal; select/add/remove/offline and Ely sign-in; Microsoft device-code polling/cancel; sign-out; verified official client/server/libraries/assets download with progress and cancel.
- Findings: Security-sensitive integration layer. Credentials are only encrypted when platform can seal; desktop fallback is memory-only. It mixes account UX with official game-file downloads, uses a global mutex/state and background threads, and trusts platform `seal/open`. It does not itself implement OAuth cryptography; that is delegated to starling-account. Download source/pins target Starling's own 26.3 build. Ely provider and cloud skin routes are external dependencies.
- RustChunk relevance: Some requirements are relevant to a launcher (profiles, private storage, cancellation, auth-state transitions, verified resumable downloads), but RustChunk's immediate user requirement is offline profiles. Do not force a Microsoft/Ely sign-in into offline single-player; online-mode servers still require valid server-compatible authentication. Separate account provider, download manager and game auth interfaces.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Rewrite all provider endpoints, version metadata and download pins for Minecraft 1.20.1. For Android, independently security-review Keystore encryption, backup exclusions, token revocation, crash-safe writes and lifecycle cancellation. Offline profile identity must not be presented as Microsoft authentication or an online-mode credential.
- Follow-up checks: Design a minimal offline-profile path first; then separately test Microsoft device-code auth, account switch/cancel races, token renewal, reinstall/key loss, concurrent sign-in/download, corrupt sealed file, path traversal, `.part` recovery, hash mismatch, network interruption and private data permissions. Review Ely.by terms/availability before adopting.
- RustChunk implementation mapping: RustChunk launcher account/profile service plus Android credential-store adapter; do not transplant the game-internal UI/state singleton wholesale. Proposed only, no code changes.
- Evidence required before adoption: Threat model, reviewed OAuth/API provider specs, Android Keystore tests, cryptographic review, 1.20.1 version manifest/hash sources, account lifecycle tests and privacy review. No auth/download/device test executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 597 logical lines read consecutively (1–400, 401–597); security-sensitive source reviewed only, no credentials accessed or network authentication run.

### 1039. `crates/horizon-client/src/advancements.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1483c1eebaca73e55886fcfef0028e5986a05dcc`
- Size: 66,532 bytes; 1,805 logical lines
- Coverage: Complete text file read to EOF, lines 1–1,805; consecutive sections of at most 400 lines were used.
- Purpose: Advancement data and progress subsystem: loads the advancement tree, indexes criteria by trigger/recipe/inventory candidate items, tracks and serializes progress, evaluates loot/entity/item/location/block/damage predicates and computes visibility/layout of the advancement tree.
- Dependencies / cross-references: Depends on lazy JSON, data-pack overlay, biome/block/fluid/structure/tag world queries, item/enchantment/potion registries, entity/player/equipment/effects views, loot condition JSON, NBT/JSON progress, recipes, and the tree-layout algorithm.
- Implementation / logic: Parses advancement JSON and criteria, merges data-pack overrides, builds trigger and recipe indexes, builds InventoryIndex from item tags, converts saved progress, evaluates predicate families recursively, evaluates recipe/arrow/item-entity/damage triggers, and performs Walker tree layout/visibility. Contains tests for index filtering, lock-code behavior, passenger/damage predicates, advancement progress/layout and date serialization; one ignored test needs prepared resources.
- Findings: Several predicate shapes and entity keys are explicitly from 26.x (`minecraft:components`, `minecraft:predicates`, keyed `EntityPredicate` namespace names, potion/variant/component models). Item predicates use data components, whereas 1.20.1 uses legacy item NBT and older predicate JSON. Includes newer mob/type-specific examples such as sulfur cube and modern damage/biome/tag sets. Some functions may support older syntax via aliases, but cannot be treated as complete 1.20.1 semantics. The ignored test references local 26.3 data.
- RustChunk relevance: Very useful conceptual source for advancement tree/progress/indexing and UI layout algorithms. It is not a drop-in 1.20.1 predicate or criterion evaluator; separate legacy parser/evaluator is required and only common algorithms should be reused.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Rebuild advancement JSON schemas, trigger registry, entity/item/location predicate format, NBT item predicates, damage source, recipe/loot condition fields, data-pack overrides and progress save schema from Java 1.20.1. Exclude modern component/variant predicates and 26.x-only trigger keys.
- Follow-up checks: Create Java 1.20.1 golden fixtures for all advancement trigger families, default requirements, completion/revoke, parent/visibility/layout, inventory/recipe indexing equivalence, nested predicates, tags/ranges, damage source and save/load. Run any ignored tests only with a disposable pinned 1.20.1 resource tree.
- RustChunk implementation mapping: Candidate legacy modules: `advancements/tree.rs`, `progress.rs`, `predicates.rs`, `triggers.rs`, `layout.rs` and parity fixtures; do not transplant the 26.x module wholesale. Proposed structure only.
- Evidence required before adoption: Java 1.20.1 Advancement/AdvancementProgress/EntityPredicate/ItemPredicate/LocationPredicate sources, exact advancement JSON and tag registries, NBT progress vectors and differential outputs. No tests executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 1,805 logical lines read in consecutive sections (1–400, 401–800, 801–1,200, 1,201–1,600, 1,601–1,805); source-only, no test/build/device run.

### 1040. `crates/horizon-client/src/ambience.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `98cfe132d091a3b2a5e47c2d2be6e27cba1523fe`
- Size: 18,126 bytes; 479 logical lines
- Coverage: Complete text file read to EOF, lines 1–479; consecutive sections of at most 400 lines were used.
- Purpose: Player-local ambient sound system coordinating biome/dimension loops, ambient additions, cave mood sounds, underwater enter/exit/loop/subsounds and bubble-column inside sounds.
- Dependencies / cross-references: Depends on SoundEngine/Sounds backend, ambient/music sound event registry, LegacyRandom, JSON ambient tables, dimension/biome data, player position/eye/fluid/spectator state, world light sampling, bubble-column block lookup and sound instance IDs.
- Implementation / logic: Loads `minecraft:audio/ambient_sounds` attributes from dimension_type and biome JSON; resolves biome over dimension defaults; computes bubble-column overlap from player AABB; ticks underwater transitions/delay/random rare additions; maintains cross-fading biome loops; evaluates additions and mood buildup from light/RNG; includes tests for 40-tick loop fade and dark cave mood timing.
- Findings: Uses the newer `minecraft:audio/ambient_sounds` environment-attribute schema rather than the 1.20.1 BiomeEffects ambient loop/mood/additions JSON fields. Bubble columns and underwater sounds exist in 1.20.1, but exact AABB scan order, spectator behavior, RNG draw order, delay reset and loop cross-fade must be checked against target source. Existing tests use fake audio backend and synthetic resource tables, not actual audio playback.
- RustChunk relevance: Good architecture reference for stateful client ambience, sound event scheduling, fades and deterministic RNG. Rebuild the resource adapter and compare tick order for Java 1.20.1; Android audio backend must be tested independently.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Read ambient data from 1.20.1 biome effects and sound definitions; do not require the 26.x environment attribute system. Validate underwater/bubble/biome mood/additions against 1.20.1 `LocalPlayer` and ambient handler logic.
- Follow-up checks: Build a deterministic 1.20.1 audio event oracle covering biome transition, loop cross-fade, light-dependent mood timer, random additions, enter/exit water, underwater delay, bubble-column first tick and spectator exclusions. Run fake backend parity and separate Android audio focus/latency tests.
- RustChunk implementation mapping: Candidate `audio/ambience.rs` plus resource adapter for 1.20.1 BiomeEffects; platform output remains RustChunk audio backend. No runtime code copied.
- Evidence required before adoption: Java 1.20.1 biome effect JSON, sound event definitions, ambient handler source, exact RNG vectors, fake-backend golden outputs and Android audio tests. No test or playback executed.
- Completion state: FULL TEXT REVIEW COMPLETE — 479 logical lines read consecutively (1–400, 401–479); fake-backend tests inspected only, no audio/device validation.

### 1041. `crates/horizon-client/src/android_control_assets.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f7351faf306f47ed098541b2c2abee1358e4c061`
- Size: 3893 bytes; 100 logical lines
- Coverage: Complete text file reviewed, lines 1–100.
- Purpose: Fetches a curated set of Android touch-control bitmap resources from a pinned Mojang Bedrock samples revision, verifying each downloaded file before accepting it.
- Dependencies / cross-references: Manifest entries, HTTPS client, redirect policy, bounded timeout/response size, SHA-256, cache/output directory and the referenced Bedrock samples repository/license.
- Implementation / logic: Downloads listed files into an app-controlled control-asset directory; validates expected size and digest, rejects invalid/oversized responses and follows a pinned source revision rather than an unpinned moving branch.
- Findings: The download/verification boundary is a sound generic pattern, but the source images are Mojang Bedrock sample assets and are not an open asset pack. Successful hash verification establishes integrity, not redistribution rights. Do not ship or redistribute the bitmaps without EULA/legal clearance.
- RustChunk relevance: Useful for secure, reproducible acquisition of optional control assets. RustChunk should draw its own editable touch icons by default and treat external images as a separately gated, user-selected resource.
- Decision: **adaptable; reference; version-specific — reuse the integrity and pinned-revision download pattern, not the source artwork or Bedrock-specific asset set.**
- 1.20.1 implications: No Minecraft 1.20.1 game data is involved. Keep UI assets independent of the Java-version data pack; do not conflate Bedrock sample provenance with permission to bundle assets.
- Follow-up checks: Review manifest provenance and Mojang EULA scope; test hash mismatch, truncated file, redirect, timeout, oversized response, path traversal and interrupted download. Provide an independent RustChunk icon set.
- RustChunk implementation mapping: Possible future isolated Android resource-fetch module; no source images copied. The RustChunk touch editor should use independently authored vector/bitmap artwork.
- Evidence required before adoption: Written asset-rights clearance before bundling any Mojang image, plus HTTP/path-safety tests and verification that no unpinned remote content is accepted.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1042. `crates/horizon-client/src/android_control_theme.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8ce860eb542ca12f63539c06c8905139688a6103`
- Size: 103070 bytes; 2840 logical lines
- Coverage: Complete source read in consecutive sections of at most 400 lines, 1–2840, ending at EOF.
- Purpose: Horizon Android visual-theme and drawing toolkit for panels, typography, control-editor screens, color palettes, borders, rounded shapes, layout helpers and font/atlas-backed text.
- Dependencies / cross-references: HudAtlas/HudBatch and font faces, display density/scale, Android viewport dimensions, palette/theme settings, UI geometry helpers and touch-control editor screens.
- Implementation / logic: Defines reusable drawing primitives and theme/layout helpers; resolves text metrics and fit, computes mobile density/scale, paints rounded panels/buttons/borders/swatches and supports multiple Horizon control-editor pages and visual styles.
- Findings: The file is primarily Horizon product UI, not Minecraft simulation or Java GUI parity. It is tightly coupled to Horizon-specific visual identity, font atlas, screen dimensions and touch-control state. Rendering helpers may be useful, but the theme, labels, hard-coded panel layouts and asset assumptions are not a RustChunk design specification.
- RustChunk relevance: Useful reference for a reusable mobile drawing abstraction, text fitting, palette separation, rounded controls and density-aware layout. RustChunk should keep its own visual design and separate renderer-independent input state from presentation.
- Decision: **adaptable; reference; optional/future — selectively reuse generic UI primitive ideas after separating them from Horizon-specific theme and screen code.**
- 1.20.1 implications: No 1.20.1 game behavior is encoded here. Do not let launcher theme resources or fixed mobile editor dimensions become dependencies of the Minecraft engine.
- Follow-up checks: Extract a minimal UI API independent of Starling types; test narrow/large aspect ratios, density changes, font fallback, clipping, color contrast and accessibility; author independent RustChunk themes.
- RustChunk implementation mapping: Future RustChunk Android UI layer, separate from engine/render world modules; do not port this file wholesale.
- Evidence required before adoption: A standalone RustChunk UI design and device screenshots across target aspect ratios/densities, plus proof that the UI layer builds without Horizon crates.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1043. `crates/horizon-client/src/android_control_ui.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `08e1d02ecc64605adca0f90f60170fc59d76977c`
- Size: 344501 bytes; 8506 logical lines
- Coverage: Complete source read in consecutive sections of at most 400 lines, 1–8506, ending at EOF.
- Purpose: Large Android touch-input and control-editor implementation: gameplay virtual mouse/keyboard, on-screen control hub, editable layouts, presets, custom buttons, property inspector and save/undo workflows.
- Dependencies / cross-references: android_controls settings/layout/editor models; android_control_theme Ui/HudAtlas/HudBatch; keyboard/mouse state; screen/menu lifecycle; filesystem-backed presets; IME and viewport scaling; touch phases and pointer ownership.
- Implementation / logic: Maps Android pointers to mouse/key actions; tracks pointer ownership, press/release/cancel, scrolling, gesture, virtual cursor and simultaneous button state; draws the floating control hub/editor; supports move/resize, grid snapping, sliders, custom bindings, preset save/load/reset, undo/redo and property edits.
- Findings: Highly relevant to the editable touch-control requirement, but this is a large Horizon-specific UI/controller integration, not a portable drop-in. It mixes rendering, input routing, persistence and editor state; fixed UI geometry and preset path assumptions require redesign. It contains valuable tests for pointer ownership, cancel, simultaneous keys and screen changes, but reading those tests does not run them.
- RustChunk relevance: One of the strongest architecture references for RustChunk's touch editor and virtual mouse. Preserve the separation between pointer ownership, key state, gesture and layout editing; do not adopt Horizon styles, fixed dimensions or persistence path directly.
- Decision: **adaptable; reference; version-specific — use as a behavioral/reference design and build a smaller RustChunk-owned touch subsystem with explicit Android lifecycle contracts.**
- 1.20.1 implications: Independent of Minecraft protocol version, but the generated key/mouse actions must map to the 1.20.1 client input model. Touch cancellation and pause/menu transitions must never leave stuck keys or clicks.
- Follow-up checks: Design separate modules for pointer capture, input state, editable layout, custom bindings, persistence and rendering; test ACTION_CANCEL, focus loss, multiple pointers, pointer-up outside bounds, screen transitions, rotation/resize, preset corruption and storage permission.
- RustChunk implementation mapping: Proposed RustChunk Android modules: touch/input_router, touch/layout_model, touch/editor and UI renderer. No direct file copy or implementation is claimed.
- Evidence required before adoption: Deterministic input traces, Android multi-pointer/lifecycle tests on the Galaxy A22 class of device, persisted layout round-trip tests and screenshots of independently designed controls.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1044. `crates/horizon-client/src/android_lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `de10b15153c41bacf31f09490bfd6a283a3cebff`
- Size: 4693 bytes; 137 logical lines
- Coverage: Complete text file reviewed, lines 1–137.
- Purpose: JNI boundary for the Android native activity: connects Java/Kotlin Activity lifecycle, shared storage, keyboard/IME input, pointer capture, diagnostics and native game startup.
- Dependencies / cross-references: JNI/JNIEnv, Android Activity class and method signatures, Java file/URI/string objects, shared storage API, IME bridge and Horizon client initialization.
- Implementation / logic: Exports native JNI entry points; retrieves the app's storage directory, sends text to and from the native client, translates IME actions/key events, handles pointer-capture loss and wraps native startup with a panic boundary that can emit diagnostics.
- Findings: The platform bridge is a narrow, useful architecture reference, but JNI symbol names, Java class/method signatures, package and storage behavior are application-specific. A panic catch is useful but must not mask partially initialized state or leave JNI references/resources inconsistent.
- RustChunk relevance: Useful for RustChunk's Android-native boundary, particularly IME and pointer lifecycle contracts. Rebuild it against RustChunk's package/activity and isolate platform bridge from gameplay logic.
- Decision: **adaptable; reference; version-specific — retain the JNI boundary and panic/IME integration pattern while generating RustChunk-specific entry points.**
- 1.20.1 implications: Game protocol behavior is not implemented here. The JNI layer must pass inputs into the 1.20.1 engine without embedding version-specific game logic.
- Follow-up checks: Check package-qualified JNI signatures, local/global reference lifetimes, UI-thread affinity, rotation/recreation, keyboard hide/show, pointer-capture loss, crash reporting privacy and ABI behavior.
- RustChunk implementation mapping: Future `android/native_bridge` or `app/src/main/java/...` integration layer in RustChunk; no exact JNI class or package is reused.
- Evidence required before adoption: Android ARM64 build/load test, Java/Kotlin-to-Rust lifecycle tests, IME/pointer tests and crash-path validation on a physical device.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1045. `crates/horizon-client/src/animate_tick.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `408af1630dc5f52294bbf0192dbb47a9b24da161`
- Size: 77461 bytes; 2000 logical lines
- Coverage: Complete source read in consecutive sections of at most 400 lines, 1–2000, ending at EOF.
- Purpose: Implements client-side block/fluid ambient animation ticks and campfire block-entity particle ticks, including deterministic particle positions, probabilities, sounds and biome ambient particles.
- Dependencies / cross-references: BlockFacts/Fluid facts, BlockRegistry/state properties, block tags and behavior JSON, Biomes, World, BlockCollisionRegistry, model offsets, sky/rain/height/weather inputs, LegacyRandom and ParticleEngine.
- Implementation / logic: Classifies block states into animation kinds; loads tags/behavior and biome ambient-particle data; simulates torch/furnace/campfire/leaves/portal/redstone/dripstone/fire/cauldron-adjacent and other ambient effects; emits particles/sounds and computes campfire cooking smoke.
- Findings: The architecture is a high-value client rendering/gameplay reference and uses deterministic RNG ordering. However it includes post-1.20.1 blocks and attributes (copper torches, pale oak, firefly bush, creaking heart, potent sulfur, dried ghast, poplar leaves etc.) and expects the 26.3 biome `attributes.minecraft:visual/ambient_particles` schema. Newer block tags and particle IDs must not be accepted as 1.20.1 facts.
- RustChunk relevance: Very relevant to particle/ambient effects and block visual fidelity; can inform a modular 1.20.1 animateTick system and help keep particles/sounds consistent with block states.
- Decision: **adaptable; reference; version-specific — preserve dispatch/RNG/particle architecture selectively, rebuilding the full block classification, tags, biome particle data and effects against Java 1.20.1.**
- 1.20.1 implications: Only target-era blocks and particle types belong in the baseline. Rebuild biome ambient particles and sound behavior from 1.20.1 resources; newer ambient environment attributes cannot be parsed as target data.
- Follow-up checks: Create a block-state-to-animate-trigger table from Java 1.20.1; add golden RNG/particle/sound vectors for every supported block; remove later block branches; test unloaded-neighbor/chunk and particle-budget behavior.
- RustChunk implementation mapping: Proposed `engine/client_effects/animate_tick` and particle event interface; production integration should consume 1.20.1 block facts and a platform-independent particle/audio queue.
- Evidence required before adoption: Pinned Java 1.20.1 source/resource mapping, deterministic per-block particle/sound oracle traces and Android GPU/audio tests. No tests or particle rendering were executed in this audit.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1046. `crates/horizon-client/src/arrows.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `146dc64d39b0fe7092daca2dc2cd8ec6fc1a8f13`
- Size: 53941 bytes; 1482 logical lines
- Coverage: Complete source read in consecutive sections of at most 400 lines, 1–1482, ending at EOF.
- Purpose: Projectile simulation and collision for arrows, tipped/spectral arrows and thrown tridents, including block/entity hits, water/lava, bubble columns, fire, pickup, piercing and hit events.
- Dependencies / cross-references: Projectile/ProjectileKind, EntityRules and target AABBs, World/BlockFacts/BlockCollisionRegistry, FluidResources/entity-fluid and flow APIs, LegacyRandom, Mob/Who, potion and ItemStack data, game rules and block contact events.
- Implementation / logic: Defines arrow state/pickup and event output; clips projectile movement against block shapes and entities; updates rotation, velocity, drag/gravity, water inertia, piercing, stuck/despawn/owner-ignore rules, entity-hit outcomes, trident bounce/deflection, TNT/campfire/candle reactions, fluid flow and fire extinguishing.
- Findings: A broad and valuable physics implementation, but it is coupled to current ItemStack data components (e.g. intangible projectile), current entity/fluid APIs and newer sound/particle registries. Some paths explicitly encode modern state, and projectile collision against a synthetic scene is not proof of server-authoritative hit parity. Need check the entire arrow/trident lifecycle against 1.20.1 rather than copy wholesale.
- RustChunk relevance: Highly relevant to survival/combat and physics; especially useful for exact block clip, swept collision, arrow pickup and fluid behavior. Separate collision/trajectory math from protocol/item metadata and game-side damage authority.
- Decision: **adaptable; reference; version-specific — retain the projectile state-machine/test approach and reimplement target-version item, metadata, hit and fluid contracts.**
- 1.20.1 implications: Rebuild 1.20.1 arrow/trident entity metadata, pickup NBT, item usage/ammo, projectile packet sync, fluid collision/flow and sound/particle IDs. Later component names must not enter legacy ItemStack/NBT.
- Follow-up checks: Build Java 1.20.1 trajectories for bow/crossbow/tipped/spectral/trident, owner-ignore, piercing, entity/block hit, refusal/deflection, water/lava/rain, bubble column, pickup and despawn; fuzz collision boundaries and unloaded chunks.
- RustChunk implementation mapping: Proposed `engine/projectiles` with versioned ammo/metadata adapter, collision/trajectory core and emitted effects; no one-file port.
- Evidence required before adoption: Java 1.20.1 golden trajectory and hit vectors, server-client packet traces, target AABB/block-shape parity and multiplayer integration tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1047. `crates/horizon-client/src/attack_strength.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ef151292c8ce67c3011f1c0b512c38e0593f7102`
- Size: 7538 bytes; 197 logical lines
- Coverage: Complete text file reviewed, lines 1–197.
- Purpose: Player attack cooldown-strength math and related client-side attack feedback, including attack-speed attribute use and hit-strength reset behavior.
- Dependencies / cross-references: Player attribute values/modifiers, tick counter, last attack tick and the client combat/attack event path.
- Implementation / logic: Computes elapsed ticks and normalized attack strength from attack speed (20 ticks/sec), clamps the result and supplies the scaling used by attack feedback/strength display.
- Findings: The core cooldown calculation is small and understandable; its constants/formula are a good candidate for a 1.20.1 differential check. It is not the whole combat algorithm: critical hits, sweep, knockback, invulnerability, shields and server damage are outside this file.
- RustChunk relevance: Directly relevant to responsive combat feel and accurate attack cooldown display.
- Decision: **adaptable; reference; version-specific — reuse the formula only after matching Java 1.20.1 attack-strength behavior and attribute defaults.**
- 1.20.1 implications: Check target attack-speed attributes and 1.20.1 Player attack strength scale/reset semantics; do not infer server damage from client cooldown alone.
- Follow-up checks: Golden cases for default and modified attack speed, partial ticks, zero/negative/large speed, boundary at full cooldown and repeated attacks; integrate with separate combat tests.
- RustChunk implementation mapping: Proposed `engine/player/combat/attack_cooldown`; independent of Android UI and server-side damage resolution.
- Evidence required before adoption: Java 1.20.1 attack-strength vectors and player attribute fixtures; no combat tests were executed in this audit.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1048. `crates/horizon-client/src/attribute_updates.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4de046ab2ee03c853d230450e70bfd4c3730693`
- Size: 5236 bytes; 147 logical lines
- Coverage: Complete text file reviewed, lines 1–147.
- Purpose: Applies pending player attribute/health updates and reconciles derived movement state after changes.
- Dependencies / cross-references: PlayerMotion, health/world update traits, live attribute world and waypoint-related state.
- Implementation / logic: Consumes pending health/attribute changes, recomputes derived movement data and updates associated state such as waypoints after the player attribute set changes.
- Findings: The attribute/health synchronization seam is useful, but waypoint behavior and current trait graph are Horizon integration details. The file alone does not define authoritative attribute packet parsing or the full 1.20.1 attribute catalogue.
- RustChunk relevance: Useful for ensuring speed/jump/health values take effect consistently after equipment/effects or server updates.
- Decision: **adaptable; reference; version-specific — keep the recomputation boundary but wire it to a dedicated 1.20.1 attribute model and packet adapter.**
- 1.20.1 implications: Rebuild entity attribute IDs, default values, modifiers and update packet semantics from 1.20.1; omit Horizon-only waypoint side effects unless independently needed.
- Follow-up checks: Test attribute add/remove/replace, modifier operation ordering, equipment/effect changes, health clamping and update timing against Java 1.20.1.
- RustChunk implementation mapping: Proposed `engine/player/attributes` update coordinator feeding movement/health subsystems; no direct trait dependency on Horizon world state.
- Evidence required before adoption: 1.20.1 attribute registry/packet golden vectors and deterministic movement/health recalculation tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1049. `crates/horizon-client/src/auto_jump.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bd7bdcbad08703810f846d5ab559b1d4e7c0c4e8`
- Size: 8784 bytes; 235 logical lines
- Coverage: Complete text file reviewed, lines 1–235.
- Purpose: Implements geometry-based automatic jump detection and the decision to trigger a jump while moving toward an obstacle.
- Dependencies / cross-references: Player movement vector/ground and collision state, AABB/collision shapes and nearby block/world queries.
- Implementation / logic: Probes ahead/sideways for a traversable obstacle, checks supporting/ceiling collision conditions and jump-boost-adjusted height, then decides whether to request a jump.
- Findings: The pure geometry/decision structure is useful for mobile movement controls, but collision and shape helpers are coupled to the current world model. Needs edge checks for stairs/slabs/fences, fluids, low ceilings and entity obstruction.
- RustChunk relevance: Relevant to smooth touch movement and avoiding unnecessary jump presses.
- Decision: **adaptable; reference; version-specific — preserve the obstacle-probe concept, but reproduce Java 1.20.1 auto-jump conditions over the target collision system.**
- 1.20.1 implications: Reconstruct the exact 1.20.1 auto-jump setting, player pose, max step, collision shape and jump boost behavior. Do not let it override explicit touch/keyboard input.
- Follow-up checks: Differential tests for stepable blocks, slabs/stairs, fences/walls, trapdoors, low ceilings, water, ladders, head collision, airborne/vehicle/spectator and intentional user jump.
- RustChunk implementation mapping: Proposed `engine/player/movement/auto_jump` using the 1.20.1 collision-query interface; user input takes precedence.
- Evidence required before adoption: Java 1.20.1 movement vectors and collision shapes, setting toggle tests and Android touch-control interaction tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1050. `crates/horizon-client/src/banner.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e50e213f1bf0ee8b3053b9e65de4136c920fd418`
- Size: 10221 bytes; 280 logical lines
- Coverage: Complete text file reviewed, lines 1–280.
- Purpose: Banner, decorated banner and shield pattern presentation/data handling, including pattern layers, item metadata conversion, Loom categories and texture-facing rendering information.
- Dependencies / cross-references: Current ItemStack component data, banner pattern registry/color mapping, Loom category tables, texture atlas/pattern resource mapping and tooltip presentation.
- Implementation / logic: Reads the current banner-pattern component representation, resolves pattern IDs and dye colors, builds ordered layers/decoration data for banner/shield and exposes pattern catalog information for UI rendering.
- Findings: The current file relies on data-component-era item metadata and newer registry/catalogue assumptions. Minecraft 1.20.1 stores banner patterns and related item data via legacy NBT (BlockEntityTag/Patterns); direct component serialization is not compatible. Loom itself exists in 1.20.1, but registry order/resource IDs must be rebuilt.
- RustChunk relevance: Banner and shield visuals matter for inventory and world rendering; layer ordering and pattern texture concepts can be reused after NBT migration.
- Decision: **rewrite required; reference; version-specific — reimplement the banner data bridge for 1.20.1 legacy NBT while retaining only the ordered-layer rendering model.**
- 1.20.1 implications: Use 1.20.1 Pattern NBT entries and dye IDs, legacy BlockEntityTag handling and target banner pattern resources. Do not introduce modern `banner_patterns` components into the vanilla 1.20.1 item model.
- Follow-up checks: Golden cases for every 1.20.1 banner pattern, color, order, shield decoration, empty/malformed NBT, inventory tooltip and Loom selection; compare texture outputs with Java.
- RustChunk implementation mapping: Proposed `engine/items/banner_nbt` for legacy parsing plus shared render-layer model; no direct modern ItemStack component adapter.
- Evidence required before adoption: Java 1.20.1 item/NBT fixtures, registry/pattern texture mapping and screenshot/golden layer parity.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1051. `crates/horizon-client/src/beehive.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `11b47818839abbb9c75ed7a0ffd5951d6842cf0a`
- Size: 14860 bytes; 461 logical lines
- Coverage: Complete source read in consecutive sections of at most 400 lines, 1–461, ending at EOF.
- Purpose: Beehive and bee-nest gameplay logic: bee occupants, NBT storage, honey/pollination state, entering/leaving, emergency release and honey-related interactions.
- Dependencies / cross-references: Bee/entity data, BeehiveBlockEntity NBT, flowers/tags, weather/time/night checks, block states and honey level, campfire/smoke path, sounds/particles and World update hooks.
- Implementation / logic: Loads bee occupant data, simulates bee entry/exit and hive capacity, tracks nectar/pollen/honey progress and cooldowns, releases bees under normal/emergency conditions, updates hive block state and emits effects.
- Findings: Beehive mechanics are relevant to 1.20.1 and largely predate the pinned snapshot. However current entity serialization/data structures and some tags/resources may be modern; verify exact 1.20.1 NBT fields, capacity/timing, weather/night and release ordering.
- RustChunk relevance: Useful survival-gameplay subsystem with block entity persistence and entity behavior; should be ported as state machine plus world events rather than copied with Horizon types.
- Decision: **adaptable; reference; version-specific — retain the hive state-machine structure after rebuilding NBT, bee entity and tag contracts for Java 1.20.1.**
- 1.20.1 implications: Reconstruct 1.20.1 bee occupant NBT, max occupants, nectar/cooldown behavior, flower tags, honey level and emergency release semantics from target Java.
- Follow-up checks: Test load/save roundtrip, full hive, entering/leaving, no flower, rain/night, smoke/soul campfire, fire, emergency release, honey harvest and chunk unload/reload.
- RustChunk implementation mapping: Proposed `engine/world/block_entities/beehive` plus bee entity AI/event interfaces; all persistence goes through a versioned 1.20.1 NBT adapter.
- Evidence required before adoption: Java 1.20.1 beehive NBT/state traces, deterministic tick tests and multiplayer block-entity update vectors.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1052. `crates/horizon-client/src/benchmark.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cf2bf02a31b29287c752dd9af3ce7de252e5670e`
- Size: 23777 bytes; 616 logical lines
- Coverage: Complete source read in consecutive sections of at most 400 lines, 1–616, ending at EOF.
- Purpose: In-client benchmark scenario runner for repeatable scene setup, camera movement, frame/tick sampling and summary/reporting.
- Dependencies / cross-references: World setup and seed/markers, chunk loading, camera/player controls, frame statistics, render/tick timing, GPU-pass instrumentation, benchmark state and platform memory/temperature data.
- Implementation / logic: Defines scenario configuration and stage progression; prepares reproducible benchmark worlds, waits for chunk readiness, drives camera trajectories, collects frame-time/tick-time/CPU/GPU and optional memory/device signals, then emits a summary.
- Findings: Useful benchmarking harness architecture, but scenarios, renderer hooks and device metric collection are Horizon-specific. Some GPU timing paths are backend-dependent and can be absent; synthetic benchmark data does not establish actual target-device performance unless executed on that device.
- RustChunk relevance: Good basis for a RustChunk repeatable performance test suite, particularly to catch regressions in chunk loading, rendering and memory on Android.
- Decision: **adaptable; reference; optional/future — reuse staged scenario/reporting ideas after integrating RustChunk-native metrics; do not import Horizon benchmark results.**
- 1.20.1 implications: Build target scenarios around Minecraft 1.20.1 world data, target renderer, mobile chunk streaming and the Galaxy A22/Mali-G52 device; metric availability must be explicit.
- Follow-up checks: Create deterministic scene seeds and readiness barriers, label unsupported GPU counters, measure p50/p95/p99 frame/tick, memory and thermal state, and compare cold/warm/reload scenarios.
- RustChunk implementation mapping: Future `tools/bench` or in-game RustChunk diagnostics subsystem; not part of gameplay engine itself.
- Evidence required before adoption: Repeated release-build runs on target Android ARM64/Mali device, raw CSV/trace artifacts, stable scene state and thermal-aware comparisons.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1053. `crates/horizon-client/src/bin/horizon-ctl/commands.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3ac2b1546141cb56ab233ee6be1c5d72fbde92b7`
- Size: 63753 bytes; 908 logical lines
- Coverage: Complete source read in consecutive sections of at most 400 lines, 1–908, ending at EOF.
- Purpose: Single source-of-truth command catalogue for the Horizon desktop control CLI and its generated help, Markdown reference and MCP tool schemas.
- Dependencies / cross-references: Command grouping/parameter type definitions, per-command flags, tool naming, JSON schema generation, line construction, destructive/network command annotations and CLI/MCP consumers.
- Implementation / logic: Declares command names, usage, summaries, descriptions, examples and typed parameters across screenshots, UI, world editing, multiplayer, account, performance, touch layout, recording and scripting; derives overview/detail/Markdown, MCP schemas and argument-to-command lines.
- Findings: This is development/automation tooling, not the in-game command parser nor gameplay engine. It exposes powerful operations including connect, account sign-in/files, world edits, LAN, recordings and scripts; generated MCP annotations help surface risk but do not themselves enforce permissions. It is desktop/Horizon CLI-specific.
- RustChunk relevance: Useful model for a typed command registry and generated docs/tool schemas if RustChunk later needs developer automation. Not needed for the mobile client baseline, and importing the command set would expose unrelated and privileged functionality.
- Decision: **unnecessary; reference; optional/future — exclude the Horizon CLI command surface from the core client; borrow the typed registry idea only for a separately scoped developer tool.**
- 1.20.1 implications: No Minecraft 1.20.1 protocol semantics are implemented here. Any future tooling must target RustChunk's actual feature set and separate local-only, destructive and network actions.
- Follow-up checks: If a CLI is later approved, define explicit allowlists, capability checks, confirmation for destructive/network/account actions, narrow file roots and independent MCP exposure review.
- RustChunk implementation mapping: No RustChunk runtime mapping. Optional future `tools/rustchunk-cli` command registry, independent from the mobile engine and not created.
- Evidence required before adoption: Separate product requirement, threat model for command capabilities and tests proving destructive/network/account actions cannot run without explicit permission.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1054. `crates/horizon-client/src/bin/horizon-ctl/launch.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3034fe41d36a3865f70381ad8d687c7f65ec4cb7`
- Size: 12734 bytes; 360 logical lines
- Coverage: Complete text file reviewed, lines 1–360.
- Purpose: Implements the Horizon desktop CLI launch path: resolves installed versions and prepared resources, starts the game process and connects to its local control channel.
- Dependencies / cross-references: Version/install manifests, Java executable and game arguments, resource-preparation stamps, working directory, OS process APIs, stdout/stderr log forwarding and control-socket readiness.
- Implementation / logic: Selects/validates a launch target, builds process command/environment, configures stdin/stdout/stderr, spawns the client, waits for readiness/control connection and forwards process exit/log state.
- Findings: The process orchestration and readiness/timeout patterns are generally useful, but the exact binary layout, manifests, Java args, control socket and OS behavior belong to Horizon's desktop CLI. It does not provide Android Activity lifecycle or an Android-native game launch.
- RustChunk relevance: Potential future desktop development harness reference; not part of RustChunk Android client implementation. The Android app should launch through its own lifecycle/native integration.
- Decision: **adaptable; reference; optional/future — retain generic readiness/process-supervision ideas only if a desktop RustChunk development CLI is later required.**
- 1.20.1 implications: The target 1.20.1 Java runtime/library set differs from the pinned Horizon version; this is not a 1.20.1 launch recipe or proof of successful Java/game execution.
- Follow-up checks: If used for a separate desktop tool, audit process cancellation, socket permissions, inherited environment, working-directory bounds, log redaction and child shutdown/timeout behavior.
- RustChunk implementation mapping: No Android runtime mapping; optional future `tools/rustchunk-cli/launch` for developer testing only.
- Evidence required before adoption: A separately scoped desktop CLI requirement and process lifecycle tests on supported desktop OSes; no game launch was executed in this source review.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1055. `crates/horizon-client/src/bin/horizon-ctl/mcp.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `888238e35c64923fba6b59e1e4c32bedb75344cd`
- Size: 10250 bytes; 256 logical lines
- Coverage: Complete text file reviewed, lines 1–256.
- Purpose: Serves the Horizon CLI as a Model Context Protocol stdio endpoint, exposing typed game/control actions and a screenshot tool to automation clients.
- Dependencies / cross-references: commands.rs registry/schema/line_for, JSON-RPC framing, MCP initialize/tools/list/tools/call methods, CLI command execution and screenshot file access.
- Implementation / logic: Reads line-delimited JSON-RPC, handles initialize and tool listing/call, validates arguments via generated schemas, maps tool calls to CLI commands and returns text or screenshot content.
- Findings: This is optional developer automation, not a mobile game dependency. The endpoint can invoke high-impact commands; argument schema is not a substitute for authorization, consent or filesystem/network sandboxing. MCP protocol revision and tool behavior are versioned.
- RustChunk relevance: Could support future automated RustChunk testing or agent-assisted diagnostics, but should be isolated from the production Android client and designed with explicit capabilities.
- Decision: **unnecessary; reference; optional/future — exclude MCP server support from the base mobile client; consider a separate restricted testing adapter later.**
- 1.20.1 implications: No Minecraft 1.20.1 game behavior is implemented. If future tools drive a target client, commands and result schemas must match RustChunk's own APIs and permissions.
- Follow-up checks: Threat-model tool exposure, enforce action capabilities/consent, restrict paths, redact secrets and test malformed/oversized JSON-RPC, concurrency and process termination.
- RustChunk implementation mapping: No production mapping. Optional future `tools/rustchunk-mcp` isolated test/diagnostic process, not created.
- Evidence required before adoption: Separate automation requirement, security review and tests for capability enforcement, secret redaction, path/network boundaries and tool lifecycle.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1056. `crates/horizon-client/src/block_contact.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d2955e06260176427e71cd833d77146cd26d4049`
- Size: 6654 bytes; 201 logical lines
- Coverage: Complete text file reviewed, lines 1–201.
- Purpose: Optional Horizon/NeoForge mod integration hook for block `stepOn`, `inside` and fall-on events, allowing mod blocks to react to entities.
- Dependencies / cross-references: Horizon/NeoForge extension API, thread-local current level, block contact events, atomic detection flag, block registry/name/state and Entity/Mob context.
- Implementation / logic: Provides a scoped thread-local level hook and a registered callback; emits contact events while ticking block interactions; records whether the current level contains a mod block and clamps incoming fall damage to finite nonnegative values.
- Findings: This is an extension/mod compatibility hook, not vanilla client block behavior. It depends on NeoForge/Horizon mod lifecycle and block event contracts, which do not exist in the RustChunk baseline. A future mod bridge would need a separate architecture and security boundary.
- RustChunk relevance: May inform a later plugin callback interface, but no direct relevance to the vanilla 1.20.1 milestone; do not introduce hidden thread-local hooks into core world simulation.
- Decision: **unnecessary; version-specific — exclude from the vanilla RustChunk core; reconsider only as a separately scoped mod/plugin extension.**
- 1.20.1 implications: Does not implement Forge 47.x and is not evidence of running Forge mods. Forge event/lifecycle/transform semantics require separate work.
- Follow-up checks: If a future mod API is scoped, define explicit event ownership, callback isolation, panic/failure handling, reentrancy/thread model and server-authoritative rules.
- RustChunk implementation mapping: No current RustChunk mapping. Future plugin API could expose typed block-contact events behind a separate extension crate; not created.
- Evidence required before adoption: A concrete mod-support product requirement, architecture/security review and parity tests against actual target Forge 1.20.1 mods.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1057. `crates/horizon-client/src/block_entity_item.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2d84fa5808a3c36f4e27f14029def9959aa730b1`
- Size: 26500 bytes; 662 logical lines
- Coverage: Complete source read in consecutive sections of at most 400 lines, 1–662, ending at EOF.
- Purpose: Converts block-entity saved NBT into item data on creative pick-block and restores block-entity state when placing the item; also handles spawn-egg data, spawner entity display and operator-only warnings.
- Dependencies / cross-references: block_entity_nbt canonicalization, ItemStack data components, NBT Compound/Tag/JavaString, container/loot/waxed/sign/banner/skull/decorated-pot/beehive data and op permissions.
- Implementation / logic: Splits block-entity NBT into implicit item components and remaining block_entity_data; maps container slots, loot table, sign sides, bees, banner patterns, wax, pot decorations and profile; on placement merges permitted custom data, applies component defaults and canonicalizes; handles spawn-egg entity_data and spawner metadata.
- Findings: This is deeply tied to the 26.3 data-component item model and includes later block entities such as shelf and trial spawner. Vanilla 1.20.1 uses the legacy ItemStack tag/BlockEntityTag, with different item/BE serialization and spawn-egg/spawner details. The pick/place round-trip idea is valuable, but this component schema is not portable.
- RustChunk relevance: High-value behavior for creative pick-block, block placement, container contents and entity/spawner data preservation. Must be rebuilt around the 1.20.1 NBT schema rather than copying the component table.
- Decision: **rewrite required; reference; version-specific — reimplement the pick/place round-trip as a version-specific 1.20.1 legacy NBT adapter.**
- 1.20.1 implications: Do not emit `minecraft:block_entity_data`, `minecraft:container`, `minecraft:banner_patterns` or other modern components as vanilla 1.20.1 item data. Reconstruct `BlockEntityTag`, item tag and operator restrictions from target Java source.
- Follow-up checks: Build round-trip fixtures for chest/container, furnace, beacon, sign, banner, skull/profile, beehive, command block/spawner, decorated pot only if present in target, spawn eggs and invalid/mismatched BE IDs; verify op gating and NBT canonicalization.
- RustChunk implementation mapping: Proposed `engine/items/block_entity_tag_1_20_1` adapter separate from modern item components; no direct port of parts()/component mapping.
- Evidence required before adoption: Java 1.20.1 pick-block/place-item golden NBT vectors, inventory/item-stack codecs, permission tests and save/load round-trips.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1058. `crates/horizon-client/src/block_pattern.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `291cdd5bd4fa88f5dcbb8238edfbf3ba4804316d`
- Size: 10532 bytes; 281 logical lines
- Coverage: Complete text file reviewed, lines 1–281.
- Purpose: Generic block-pattern matching and consumption for structured constructions such as Wither, snow golem and iron golem patterns.
- Dependencies / cross-references: BlockRegistry/state properties, World block queries, pattern definitions/orientations, block collision/shape or state predicates and world mutation/removal hooks.
- Implementation / logic: Defines pattern blocks/predicates and orientation transforms; searches candidate origins/directions, matches required arrangement and returns matched positions/state; consumes or transforms blocks for the construction behavior.
- Findings: The pattern-matching abstraction is broadly reusable, but pattern catalogue, block predicates, orientations and placement/removal semantics must match the target version. It should not be used to assume modern constructs exist in 1.20.1.
- RustChunk relevance: Useful generic algorithm for golem/wither construction and future multiblock structures; separate pure pattern matching from world mutation and entity spawning.
- Decision: **adaptable; reference; version-specific — reuse the generic orientation/match model after rebuilding only the 1.20.1 structure definitions and world update order.**
- 1.20.1 implications: Confirm all target 1.20.1 patterns, block states and spawn rules; exclude structures and block-state variants introduced later.
- Follow-up checks: Golden Java 1.20.1 arrangements for every orientation, partial/malformed pattern, nearby obstruction, snow/golem variants, wither skull order and block-consumption order.
- RustChunk implementation mapping: Proposed `engine/world/patterns` generic matcher plus versioned 1.20.1 construction definitions and game-event hooks.
- Evidence required before adoption: Java 1.20.1 source/data and deterministic world fixtures proving match result, entity spawn and block mutation order.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1059. `crates/horizon-client/src/block_predictions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f09284331e9356782b02e26c86395c34659a4b3a`
- Size: 6151 bytes; 179 logical lines
- Coverage: Complete text file reviewed, lines 1–179.
- Purpose: Reconciles client-side predicted block changes with authoritative server block updates and sequence acknowledgements.
- Dependencies / cross-references: World/chunk block state, client block interaction sequence IDs, server acknowledgement/update packets, player movement/teleport state and pending-prediction storage.
- Implementation / logic: Tracks predicted block states by sequence and position; on authoritative acknowledgement or update, confirms/removes matching predictions, restores or applies server state and suppresses invalid rewinds around teleport/respawn transitions.
- Findings: Core concept is essential for responsive block placement/breaking without visual disagreement. Current sequence/packet integration is Horizon version-specific; ordering, duplicate updates, sequence wrap and chunk unload handling need target protocol validation.
- RustChunk relevance: Directly relevant to the user's concern about near chunks and blocks appearing/disappearing: predicted state reconciliation must never corrupt authoritative chunk data or hide missing chunk loads.
- Decision: **adaptable; reference; version-specific — preserve the prediction/reconciliation state machine, but rebuild packet sequence/ack rules for Java 1.20.1.**
- 1.20.1 implications: Check exact 1.20.1 block-change acknowledgement packet, sequence counter and handling of single/multi-block updates, failed interactions and teleport. Do not rely on newer packet ordering.
- Follow-up checks: Test ack success/failure, update before ack, multi-block update, duplicate/stale sequence, wraparound, chunk unload/reload, dimension change, teleport and server correction with deterministic traces.
- RustChunk implementation mapping: Proposed `engine/world/block_predictions` as a client prediction layer sitting above authoritative chunk storage; maintain explicit source-of-truth separation.
- Evidence required before adoption: Java 1.20.1 client/server packet traces, deterministic race/order fixtures and Android walk/place/break tests at chunk boundaries.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.

### 1060. `crates/horizon-client/src/block_transform.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4cd1d6f94492a7c25846f85c53ef1de023e34b32`
- Size: 9777 bytes; 269 logical lines
- Coverage: Complete text file reviewed, lines 1–269.
- Purpose: Data-driven block transformation registry for item interactions such as axe stripping, hoe tilling and shovel path creation.
- Dependencies / cross-references: Modern block-transformer registry/resource JSON, block-state property changes, tool/item categories, sounds, particles and loot/result state.
- Implementation / logic: Loads transformer definitions, resolves eligible block state/tool operations, applies state transformation and emits interaction feedback from data tables.
- Findings: This is a newer registry-driven system from the pinned 26.3 snapshot. Vanilla 1.20.1 performs axe/hoe/shovel transformations through hard-coded item interaction maps/rules rather than this data-driven transformer registry. Current IDs, data schema and sound/particle tables cannot be adopted directly.
- RustChunk relevance: The data-driven table architecture could help organize target-version interactions, but actual target rules must come from 1.20.1 item classes and block states.
- Decision: **rewrite required; reference; version-specific — reconstruct 1.20.1 axe/hoe/shovel interactions independently; use the registry concept only as an internal table design if useful.**
- 1.20.1 implications: Do not expect `minecraft:block_transformer` resources in 1.20.1. Rebuild axe stripping, wax removal, hoe tilling and shovel path actions, correct block predicates, waterlogging and tool damage/hand swing semantics.
- Follow-up checks: Create Java 1.20.1 differential cases for all axe/hoe/shovel transformations, invalid states, waterlogged blocks, tool durability, interaction ordering and sound/particle outputs.
- RustChunk implementation mapping: Proposed `engine/items/tool_interactions_1_20_1` with explicit target-version rules, not a loader for current transformer JSON.
- Evidence required before adoption: 1.20.1 Java item interaction source and golden block-state/tool durability/event vectors.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, builds, benchmarks, APKs or Android device runs were executed.
