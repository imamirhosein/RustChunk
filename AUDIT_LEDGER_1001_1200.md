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
