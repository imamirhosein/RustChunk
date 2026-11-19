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
- Size: 2411 bytes; 50 logical lines
- Coverage: Complete text file reviewed, lines 1–50.
- Purpose: Differential fixture for team gameplay rules: friendly-fire permission, alliance, pushability, invisibility and name-tag visibility.
- Dependencies / cross-references: Uses Scoreboard, CollisionRule, Visibility and NameView; JSON case input/output.
- Implementation / logic: Creates two teams from case flags, resolves own/other/viewer teams, computes invisibility and NameView context, then calls teams_allow_damage, teams_allied, pushable_by and name_shown.
- Findings: Isolates rule evaluation; omits packet synchronization, team membership lifecycle, collision resolution and rendering. Invalid enum names panic via expect.
- RustChunk relevance: Useful for multiplayer/team correctness. Pure rules/test pattern is reusable; data and edge semantics require Java 1.20.1 checking.
- Decision: **adaptable; reference; version-specific — keep the focused team-rule oracle pattern and rebuild cases against the 1.20.1 team model.**
- 1.20.1 implications: Reconfirm 1.20.1 team collision/visibility enum values, friendly-fire semantics, spectator handling and scoreboard membership updates.
- Follow-up checks: Test missing/same/different teams, all collision rules, spectator invisibility, camera/vehicle name visibility, HUD-hidden and distance thresholds.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/team_rules_fixture.rs (proposed, not created); runtime mapping is scoreboard state plus entity push/name-tag policy.
- Evidence required before adoption: Java 1.20.1 scoreboard/team goldens and multiplayer packet-order tests; collision/name-tag device render checks separate.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1022. `crates/horizon-client/examples/probe/text_display_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f1f58762586cd1d9dc32ea78c75f4c41a050e4d6`
- Size: 1535 bytes; 37 logical lines
- Coverage: Complete text file reviewed, lines 1–37.
- Purpose: Decoder-level parity fixture for text-display entity metadata: reads wire entity-data packets and emits interpreted state or rotation matrix.
- Dependencies / cross-references: Uses current horizon_protocol Play packet IDs/decoder, SetEntityData, TextDisplayData and serde_json.
- Implementation / logic: Hex-decodes wire bytes, decodes current SET_ENTITY_DATA packet, applies metadata values, then serializes display state/alignment/plain text or computes the four-angle matrix.
- Findings: Text display entities exist in target era, but packet ID, metadata serializer indices and current TextDisplayData structure are snapshot-specific. No entity lifecycle.
- RustChunk relevance: Useful metadata-to-display-state and transform test pattern; protocol translation must be fully versioned.
- Decision: **adaptable; reference; version-specific — preserve fixture concept, rebuild wire decoder and metadata schema from Java 1.20.1.**
- 1.20.1 implications: Use 1.20.1 entity metadata index/type mapping and text component format; compare interpolation, billboard, alignment, opacity, background and line width.
- Follow-up checks: Create Java 1.20.1 vectors for full/partial metadata, null text, billboard modes, rotations, scale, alignment, shadow, see-through, opacity and background.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/text_display_fixture.rs (proposed); production is versioned entity metadata decoder feeding text display model.
- Evidence required before adoption: Authoritative 1.20.1 SetEntityData packet/metadata vectors and Java-derived state/matrix outputs, including malformed and partial updates.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1023. `crates/horizon-client/examples/probe/text_display_render_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2a539e4252095695ffed9ceee6f4dd3d041f37cc`
- Size: 2925 bytes; 70 logical lines
- Coverage: Complete text file reviewed, lines 1–70.
- Purpose: Headless rendering oracle for text-display content: converts metadata wire cases into laid-out text quads, pose matrices and ARGB colors.
- Dependencies / cross-references: Uses HudAtlas/HudBatch, font layout, TextDisplayData, Lines, background color and current protocol metadata decoding.
- Implementation / logic: Loads HUD atlas; decodes entity data; applies display metadata; wraps text; draws glyph/background quads; serializes pose and quad corners/colors.
- Findings: Bypasses GPU and serializes only part of quad data. Atlas/font, serializer indices and text schema are snapshot-specific; no world-space billboard/depth validation.
- RustChunk relevance: Valuable text-quad oracle. Keep separate from packet decoder and from actual mobile GPU validation.
- Decision: **adaptable; reference; version-specific — reuse text-quad oracle shape; rebuild metadata/font/atlas inputs for 1.20.1.**
- 1.20.1 implications: Verify 1.20.1 metadata, text wrapping, billboard/transform and renderer resources; do not import newer HUD sprite identifiers.
- Follow-up checks: Compare line wrapping, glyph UVs, background alpha, shadow, see-through, alignment, transform origin, billboard modes and depth with Java 1.20.1; add Android/Mali image comparison.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/text_display_render_fixture.rs (proposed); production mapping is 1.20.1 text-display renderer and world-space submission.
- Evidence required before adoption: Java 1.20.1 golden quad/pose vectors and screenshot tests at multiple GUI scales; Mali-G52 validation.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1024. `crates/horizon-client/examples/probe/toggle_keys_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d05acdd5c07cbe4805a997ca07b8725ef7961520`
- Size: 2052 bytes; 56 logical lines
- Coverage: Complete text file reviewed, lines 1–56.
- Purpose: Deterministic state-machine fixture for toggle mappings of sneak, sprint, attack and use across keyboard/button/lifecycle events.
- Dependencies / cross-references: Uses ToggleKeys, KeyAction, four toggle options and scripted JSON event lists.
- Implementation / logic: Replays option changes, key press/release/repeat, button state, screen-open/close and respawn; emits down-state and pending click counts.
- Findings: Covers discrete toggles but not analog, touch pointer IDs, ACTION_CANCEL, focus loss or Android lifecycle callbacks. Input indices are trusted.
- RustChunk relevance: Good regression model for consistent key-toggle behavior across keyboard and on-screen controls.
- Decision: **adaptable; reference — reuse event-sequence test structure and map Android touch into the same canonical key contract.**
- 1.20.1 implications: Verify 1.20.1 toggle/hold semantics and screen transitions; touch cancellation remains platform-specific.
- Follow-up checks: Add focus-loss, duplicate press, repeat, pointer-up/cancel, multi-touch, screen transition, pause/resume and respawn cases.
- RustChunk implementation mapping: QA-only candidate tests/input/starling/toggle_keys_probe.rs (proposed); runtime mapping is input state machine shared by keyboard and editable touch controls.
- Evidence required before adoption: Java 1.20.1 input traces plus Android multi-pointer/lifecycle tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1025. `crates/horizon-client/examples/probe/tooltip_display_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7cb3ade954f587ccf3a3a289cc19401a020f1063`
- Size: 2808 bytes; 70 logical lines
- Coverage: Complete text file reviewed, lines 1–70.
- Purpose: Probe for tooltip visibility policy and styled item-name rendering based on the newer tooltip_display item data component.
- Dependencies / cross-references: Uses TooltipDisplay, ItemStack, component decoding and NBT Tag; tab-separated visibility flags and custom-name text.
- Implementation / logic: Builds synthetic sword stack, writes hide-tooltip/hidden-components data, asks TooltipDisplay visibility and emits custom-name style spans.
- Findings: The minecraft:tooltip_display component and hidden-components schema are newer than vanilla 1.20.1. Custom-name subcase is conceptually useful but not a target tooltip model.
- RustChunk relevance: Only the general split between item-name styling and tooltip-section visibility is reusable.
- Decision: **unnecessary; adaptable; version-specific — exclude tooltip_display component behavior from the 1.20.1 baseline; independently test legacy HideFlags/custom names.**
- 1.20.1 implications: Rebuild tooltip suppression from 1.20.1 ItemStack/NBT/HideFlags; do not add the later component to vanilla item protocol.
- Follow-up checks: Build Java 1.20.1 vectors for HideFlags bits, attributes/enchantments/durability/lore/name, creative mode and custom-name styles.
- RustChunk implementation mapping: No runtime mapping for TooltipDisplay; separate legacy parity fixture proposed under tests/parity/starling/legacy_tooltip_visibility.rs.
- Evidence required before adoption: Java 1.20.1 tooltip goldens and NBT item fixtures; confirm no later tooltip_display component in vanilla baseline.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1026. `crates/horizon-client/examples/probe/trading_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `36804fba9e92be2326438781135f24759085c66f`
- Size: 20564 bytes; 556 logical lines
- Coverage: Complete text file reviewed in consecutive sections of at most 400 lines, lines 1–556; final section ends at EOF.
- Purpose: Merchant-trading differential harness covering villager/wandering-trader offers, price modifiers, gossip, restock, menu, XP, NBT and RNG.
- Dependencies / cross-references: Uses Items, Enchantments, Trades, RandomSequences, Gossips, MerchantOffer/Session/Trader, merchant menu, entity NBT, recipes/brewing/banner data, legacy RNG and PackOverlay.
- Implementation / logic: Loads tables; builds merchants from profession/variant/level or wandering trader sets; applies uses/demand/gossip/XP/restock; tests price/restock/NBT; replays menu select/click/tick/close and tracks inventory/offers/XP/orbs/random draws.
- Findings: Test transport with snapshot assumptions around trade-set names, random sequence IDs, profession/variant catalogues, merchant level logic and menu. Not full village/server tick simulation; positional inputs may panic.
- RustChunk relevance: High-value gameplay reference for merchant interaction, inventory transactions, pricing and persistence.
- Decision: **adaptable; reference; version-specific — keep merchant state/replay and NBT comparison strategy; rebuild 1.20.1 trade pools, gossip, restock and menu behavior.**
- 1.20.1 implications: Use target release's villager professions/levels/biomes, wandering trader offers, gossip, demand/special prices, restock timing, trade XP and ItemStack NBT. Do not assume current random sequence IDs match.
- Follow-up checks: Create Java 1.20.1 goldens for profession/level/variant, wandering trader, uses/demand/gossip/hero, restock boundaries, menu click modes, inventory overflow, XP orbs and villager NBT.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/trading_fixture.rs (proposed); production spans merchant data, gossip, menus, inventory, RNG and entity persistence.
- Evidence required before adoption: Pinned 1.20.1 trade/loot/profession data, Java oracle traces, NBT round-trip cases and multiplayer menu packet tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1027. `crates/horizon-client/examples/probe/translucency_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8ea42c1dce681b848e7b880e031f59c4f29415b8`
- Size: 1697 bytes; 47 logical lines
- Coverage: Complete text file reviewed, lines 1–47.
- Purpose: Oracle for translucent quad centers, camera-distance sorting order and cached sort-state updates.
- Dependencies / cross-references: Uses SortState, Sorter and quad_center with quad positions, section origin and camera positions.
- Implementation / logic: Computes quad centers, initializes state, updates cache for cameras and independently sorts centers using camera-relative coordinates; emits float bits, indices and update flags.
- Findings: Tests ordering/cache only, not draw submission, blend/depth, cross-section ordering or visuals. Separate cache sorter and local sorter distinction matters.
- RustChunk relevance: Directly relevant to terrain transparency sorting and artifacts.
- Decision: **adaptable; reference — preserve deterministic quad-order and cache-invalidation tests; adapt precision and renderer integration.**
- 1.20.1 implications: Compare exact 1.20.1 sort algorithm, center precision, camera origin and invalidation thresholds; do not infer blend/depth from fixture.
- Follow-up checks: Test equal-distance ties, threshold movement, section-origin changes, chunk unload/reload, large counts and nonfinite input rejection.
- RustChunk implementation mapping: QA-only candidate tests/rendering/starling/translucency_fixture.rs; runtime mapping is translucent quad sort/cache.
- Evidence required before adoption: Java 1.20.1 sorted-index goldens plus glass/water/ice screenshots and Mali blending tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1028. `crates/horizon-client/examples/probe/trident_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bf4e1392d7d79a9a1ad6c4ea83dd0fa084204a53`
- Size: 5183 bytes; 130 logical lines
- Coverage: Complete text file reviewed, lines 1–130.
- Purpose: Headless block-collision parity probe for thrown tridents: projectile step, block hit, in-ground state, shake, gravity and air drag.
- Dependencies / cross-references: Uses Assets/RenderAssets, World::test_scene, BlockFacts, collision binding, arrows::clip_block_stop, Vec3 and JSON operations.
- Implementation / logic: Builds radius-1 scene, edits block states, replays ticks, clips movement against block shapes, stops/pulls back on hit or advances and applies drag/gravity on miss.
- Findings: Covers block collision only. Owner, pickup, touch, target, ignite and velocity inputs are ignored; no entity hits, fluids, loyalty, pickup eligibility, network or authority. Synthetic scene is small.
- RustChunk relevance: Useful targeted projectile/collision regression for RustChunk.
- Decision: **adaptable; reference; version-specific — retain collision-step probe pattern and rebuild projectile rules/data against Java 1.20.1.**
- 1.20.1 implications: Reconstruct 1.20.1 trident movement, block clip, drag/gravity, shake, pickup, loyalty, riptide and entity-hit semantics.
- Follow-up checks: Add block corner/edge and chunk-border, entity interception, fluids, owner/pickup, loyalty return, riptide, unloaded chunks and seeded precision cases.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/trident_probe.rs; runtime mapping is projectile physics/collision.
- Evidence required before adoption: Java 1.20.1 golden trajectories/collision shapes and server-authoritative hit/pickup tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1029. `crates/horizon-client/examples/probe/vehicle_container_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `48942be5b184ec2466e300f72d875cf38b3a0441`
- Size: 1314 bytes; 33 logical lines
- Coverage: Complete text file reviewed, lines 1–33.
- Purpose: Table-driven mapping of vehicle entity type to container menu, slot count and title key.
- Dependencies / cross-references: Uses containers::vehicle_menu and vehicle_menu_name with one entity-type ID per line.
- Implementation / logic: Trims entity type ID, resolves vehicle menu and outputs menu ID, size and title key or error.
- Findings: Narrow lookup only; no open/close packet, slot layout, item transfer or persistence. Entity mapping is snapshot-specific.
- RustChunk relevance: Useful inventory UX and multiplayer consistency check for chest boats/minecarts and hopper minecarts.
- Decision: **adaptable; reference; version-specific — retain table-driven mapping test and rebuild entity/menu catalogue for 1.20.1.**
- 1.20.1 implications: Confirm target vehicle types, menu IDs, sizes and title keys; exclude later vehicle entities.
- Follow-up checks: Test chest boat, chest minecart, hopper minecart, ordinary boat/minecart, invalid IDs, localization and full menu transfers.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/vehicle_container_fixture.rs; runtime mapping is vehicle metadata plus container menu factory.
- Evidence required before adoption: 1.20.1 Java entity/menu mappings and server packet vectors for opening, closing and transfers.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1030. `crates/horizon-client/examples/probe/weather_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b85dd3e097c1631060b239144ed004893eeebcff`
- Size: 2402 bytes; 67 logical lines
- Coverage: Complete text file reviewed, lines 1–67.
- Purpose: Headless rain/snow weather-column sampling, deterministic randomness, parameter-cache and quad-generation fixture.
- Dependencies / cross-references: Uses weather::column_random, rain_column, snow_column, ParameterCache, column_quad and camera/light inputs.
- Implementation / logic: Computes rain/snow column extents/UV/light, asserts cached parameters equal uncached columns, emits vertex positions/UV/color/light.
- Findings: Tests geometry/cache only, not precipitation particle simulation, biome/weather transitions, sound, fog, GPU blending or device visuals. Positional inputs are trusted.
- RustChunk relevance: Useful weather rendering/cache regression.
- Decision: **adaptable; reference; version-specific — retain deterministic column/cache model and rebuild visual/data constants for 1.20.1.**
- 1.20.1 implications: Compare 1.20.1 rain/snow density, heights, randomization, UV, light, alpha and render distance.
- Follow-up checks: Test weather transitions, world-height bounds, camera, light/cache invalidation, chunk borders and Android/Mali image output.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/weather_fixture.rs; runtime mapping is weather column preparation and precipitation renderer.
- Evidence required before adoption: Java 1.20.1 weather vectors and screenshots in daylight/night, rain/snow and multiple render distances.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1031. `crates/horizon-client/examples/probe/world_border_render_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `535118959241217cf1933f18098f2640e3f0aa97`
- Size: 5373 bytes; 111 logical lines
- Coverage: Complete text file reviewed, lines 1–111.
- Purpose: Parity harness for world-border state and visual rendering: interpolation, bounds, wall geometry/transforms and warning vignette.
- Dependencies / cross-references: Uses WorldBorder/BorderStatus, BorderRenderer/BorderRenderState, GUI vignette and JSON operations.
- Implementation / logic: Replays tick/lerp/size/center/warning/invalidate/frame operations; captures interpolated bounds/status, prepares renderer, serializes vertices/transforms/sides and vignette color.
- Findings: Headless extraction only, not GPU texture/blend. Fixed 427×240 vignette viewport; assumes valid scripts.
- RustChunk relevance: Useful world-border interpolation and rendering reference.
- Decision: **adaptable; reference; version-specific — retain deterministic border-state/vertex oracle; rebuild constants and packet mapping for 1.20.1.**
- 1.20.1 implications: Verify target packet updates, interpolation timing, max size, texture offsets, warning settings and vignette semantics.
- Follow-up checks: Test expand/shrink/stationary, invalid sizes, warning thresholds, camera on border, renderer invalidation, partial tick and GUI scale.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/world_border_render_fixture.rs; runtime spans world-border state, render extraction and HUD vignette.
- Evidence required before adoption: Java 1.20.1 state/packet goldens and vertex/screenshots at multiple viewports; Android GPU blending.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1032. `crates/horizon-client/examples/probe/world_options_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5bd3026bc5aa664274b96b7a93cce53d49ec8fd5`
- Size: 1501 bytes; 40 logical lines
- Coverage: Complete text file reviewed, lines 1–40.
- Purpose: Packet codec fixture for difficulty change/lock and game-rule set/value messages.
- Dependencies / cross-references: Uses current protocol ChangeDifficulty, LockDifficulty, SetGameRule, GameRuleValues, Reader and Play packet encoders.
- Implementation / logic: Builds current-version packet structs from tab-separated inputs, encodes bytes, removes packet ID varint and emits payload hex.
- Findings: Strictly wire codec fixture; packet IDs and GameRuleEntry/SetGameRule structures are version-specific. Does not test state transition/server application.
- RustChunk relevance: Useful packet fixture pattern only.
- Decision: **rewrite required; reference; version-specific — rewrite packet structs, IDs and payloads from Java 1.20.1 protocol.**
- 1.20.1 implications: Verify which difficulty/game-rule packets exist in 1.20.1; later GameRuleValues messages cannot be assumed.
- Follow-up checks: Generate Java 1.20.1 packet bytes, malformed varint/trailing-byte tests, direction and protocol-state constraints.
- RustChunk implementation mapping: QA-only candidate tests/protocol/v1_20_1/world_options_fixture.rs; production mapping is dedicated 1.20.1 packet module.
- Evidence required before adoption: Authoritative protocol IDs/schemas and client/server packet-byte vectors.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1033. `crates/horizon-client/examples/random_tick_eligibility_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bc5a8ea560245e5a9a51177ff30686a180bbcc54`
- Size: 5670 bytes; 150 logical lines
- Coverage: Complete text file reviewed, lines 1–150.
- Purpose: Microbenchmark comparing baseline and cached selection of random-ticking sections; explicitly selection cost only, not block simulation.
- Dependencies / cross-references: Uses BlockFacts, Chunk/Section, FxHashMap, RandomTickSections and prepared resources.
- Implementation / logic: Creates sparse/dense 17x17 chunk fixtures; measures steady/cold/reload/edit workloads over repeated ticks, alternates order and reports timing distributions.
- Findings: Does not simulate random-tick block behavior. Black-boxes result counts but does not assert baseline/cache semantic equality; synthetic workloads may dominate.
- RustChunk relevance: Useful cache benchmark methodology for chunk ticking.
- Decision: **adaptable; reference; optional/future — retain benchmark idea; add semantic equality checks before using performance numbers.**
- 1.20.1 implications: Rebuild random-ticking facts for 1.20.1; verify chunk eligibility, light gating and cache invalidation from target rules.
- Follow-up checks: Assert baseline equals cached selection for all workloads; test edits/removals/reloads/stale handles; run release benchmark on Android and end-to-end tick.
- RustChunk implementation mapping: Optional candidate benches/random_tick_eligibility.rs; runtime mapping is chunk random-tick eligibility cache, not block simulation.
- Evidence required before adoption: 1.20.1 Java/random-tick oracle parity and repeatable Android ARM64/Mali benchmark; no benchmark run here.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1034. `crates/horizon-client/examples/resource_memory.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f85a65f8aa74885b83d93591f833f286ab56f819`
- Size: 5564 bytes; 155 logical lines
- Coverage: Complete text file reviewed, lines 1–155.
- Purpose: Resource-loader memory profiler reporting live/peak counted heap and process RSS after each initialization stage.
- Dependencies / cross-references: Uses custom GlobalAlloc counter plus Assets, BlockFacts/Behaviours, Items, Recipes, Biomes, LightTables, MeshTables, collisions, MotionResources and VanillaTerrain; calls ps/macOS footprint.
- Implementation / logic: Counts allocations, loads startup stages, reports live delta/peak/RSS, sprite pixel memory and state count; retains loaded resources to report steady state.
- Findings: Unsafe allocator plumbing and OS-specific ps/footprint; cannot capture GPU/native allocations completely; concurrent allocations blur phase attribution.
- RustChunk relevance: Useful low-memory Android budgeting methodology; not ready Android profiler.
- Decision: **adaptable; reference; version-specific — adapt staged memory profiling, replace platform-specific RSS and validate 1.20.1 resources.**
- 1.20.1 implications: Resource tables/sizes are snapshot-specific; no counts from this benchmark establish Galaxy A22 memory behavior.
- Follow-up checks: Use Android PSS/native/GPU APIs, measure peak during reload, clean and modded 1.20.1, cold/warm profiles.
- RustChunk implementation mapping: Optional profiling tool tools/profile_android_resource_memory (proposed); not a production module.
- Evidence required before adoption: Measured Android ARM64 PSS/native/GPU memory profiles with repeatable 1.20.1 resource loads.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1035. `crates/horizon-client/examples/save_batch_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `30d7629ca1de49903c8d6828abf2863c005642d2`
- Size: 9699 bytes; 260 logical lines
- Coverage: Complete text file reviewed, lines 1–260.
- Purpose: Synthetic benchmark of batch chunk snapshot preparation for world saving, optionally NBT-encoding, with exact serialized-byte equality guard.
- Dependencies / cross-references: Uses Chunk/ChunkExtras, BlockEntity, item stacks/NBT, PalettedContainer, Biomes, BlockRegistry and save::chunks_with_block_entities; creates temporary resource fixture.
- Implementation / logic: Builds 256 chunk village/storage workloads with chest/furnace/sign/block-entity data and NBT overlays; compares old per-chunk filter with batch helper; asserts exact NBT bytes; measures alternating medians; optional NBT timing.
- Findings: Measures in-memory snapshots and optional encoding only; excludes disk I/O/compression/GPU/autosave. Temp directory predictable by process ID and removed recursively; requires isolation.
- RustChunk relevance: Useful save optimization pattern with strong exact-output guard.
- Decision: **adaptable; reference; optional/future — preserve equality guard/benchmark method; rebuild 1.20.1 data and harden temp isolation.**
- 1.20.1 implications: Recreate target Chunk/BlockEntity/ItemStack/Anvil/NBT structures; current DATA_VERSION and data model are snapshot-specific.
- Follow-up checks: Use unique secure temp dirs, cleanup on errors, test zero/large density/unknown entities, validate exact 1.20.1 output and separately measure Android storage.
- RustChunk implementation mapping: Optional candidate benches/save_batch.rs; runtime mapping is snapshot assembly before chunk save.
- Evidence required before adoption: 1.20.1 NBT golden/round-trip, crash recovery and Android storage I/O benchmarks.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1036. `crates/horizon-client/examples/shared_json_startup_profile.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c6ab30374a40dc91e8784ae0cf02ce350f1a7b76`
- Size: 3542 bytes; 76 logical lines
- Coverage: Complete text file reviewed, lines 1–76.
- Purpose: Process-isolated profile of block-fact/behavior table startup across baseline/shared/deferred parsing modes.
- Dependencies / cross-references: Uses Assets, BlockFacts, Behaviours, JSON parser and Linux /proc/self/status VmRSS.
- Implementation / logic: Loads assets, times resource fact/behavior parsing variants, reports phase/total time, state count, parse/facts/behavior subphase and optional retained JSON/projection.
- Findings: Requires separate processes for peak RSS; deferred mode runs phases contiguously unlike actual startup; Linux VmRSS omits Android GPU/native usage; one mode compares historical PR snapshot.
- RustChunk relevance: Useful startup optimization methodology, not a direct port decision or benchmark result.
- Decision: **adaptable; reference; optional/future — reuse process-isolated profiling with Android memory sampling and 1.20.1 data.**
- 1.20.1 implications: Facts/behavior schema differs in 1.20.1; require identical outputs before comparing loaders.
- Follow-up checks: Add hash/semantic equality for modes, separate cold/warm, Android PSS/native memory and interrupted reload tests.
- RustChunk implementation mapping: Optional candidate tools/profile_android_startup; runtime mapping is resource preparation/startup.
- Evidence required before adoption: Same-output hashes and repeated Android ARM64 profiles using exact 1.20.1 assets.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1037. `crates/horizon-client/examples/sprite_cache_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `70f86b4a5c815e84c4e93d22e0278822d659baee`
- Size: 2112 bytes; 59 logical lines
- Coverage: Complete text file reviewed, lines 1–59.
- Purpose: Cold/warm sprite disk-cache parity gate comparing decoded/cache-loaded pixels, layers, mips and animation blends.
- Dependencies / cross-references: Uses Assets/RenderAssets and deletes horizon-sprites.cache under supplied ROOT.
- Implementation / logic: Loads assets, removes cache, measures cold decode/write, hashes sprite/animation state, reloads warm cache and compares digest.
- Findings: Mutates supplied resource root by deleting cache; unsafe against user live resources. DefaultHasher is not stable artifact checksum; does not test corruption recovery/concurrency/GPU upload.
- RustChunk relevance: Useful asset-cache cold/warm regression concept; must run on disposable copy.
- Decision: **adaptable; reference; optional/future — keep parity test but isolate directory and use stable digest or byte equality.**
- 1.20.1 implications: Rebuild cache key/serialization for 1.20.1; include game version, source hash, pack inputs, renderer format and ABI in invalidation.
- Follow-up checks: Run only on temp resources; test missing/truncated/stale cache, atomic writes, concurrent readers, animation parity and GPU upload.
- RustChunk implementation mapping: QA-only candidate tests/assets/sprite_cache_parity.rs; runtime mapping is sprite resource cache validation.
- Evidence required before adoption: Stable digest or byte comparison, corrupt-cache recovery and target Android filesystem/renderer tests.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1038. `crates/horizon-client/src/accounts.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `314fcfe1d29b78f078abd5740c93907d831e9984`
- Size: 21092 bytes; 597 logical lines
- Coverage: Complete text file reviewed in consecutive sections of at most 400 lines, lines 1–597; final section ends at EOF.
- Purpose: In-game profile/account orchestration for Microsoft, Ely.by and offline profiles, device-bound secret storage, session renewal, skin upload and official game-file downloads.
- Dependencies / cross-references: Uses Starling account crate, SecretStore/Endpoints/SignIn, Android Keystore platform seal/open and foreground-work hooks, auth session, private directories, network file pinning and async threads.
- Implementation / logic: Maintains shared account/file state; seals secrets where supported; selects/adds/removes offline/online profiles; cancellable sign-in/resume; periodically renews sessions; uploads skins; downloads pinned client/server/libraries/assets privately and writes completion marker.
- Findings: Security-sensitive launcher/account functionality, not simulation. Closely coupled to provider/network endpoints, Starling account crate, platform hooks, UI and file pins. Copying wholesale imports unreviewed auth surface. Offline name/UUID does not authenticate online mode.
- RustChunk relevance: Architecture reference for private storage, token handling, cancellable work and offline-profile separation; requires independent security/provider scope.
- Decision: **adaptable; reference; optional/future — preserve reviewed security requirements and abstractions only; do not port without threat model/provider decision.**
- 1.20.1 implications: 1.20.1 game files/libraries/Java runtime differ; online session and protocol auth must match target; offline profile never grants online-mode server access.
- Follow-up checks: Threat-model storage/removal, audit OAuth device flow/token revocation, verify provider terms, validate artifact hashes/atomic installation and test cancellation/reinstall/keystore failure.
- RustChunk implementation mapping: No direct file mapping; future account module proposed under engine/auth with Android Keystore bridge and offline profiles isolated from online credentials.
- Evidence required before adoption: Independent auth design, provider terms, keystore review, credential redaction, artifact integrity/rollback and end-to-end online/offline login validation.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1039. `crates/horizon-client/src/game/advancements.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `18479955a0f183cd5f98d98290a6557c6cb4d7e2`
- Size: 94062 bytes; 2341 logical lines
- Coverage: Complete text file reviewed in consecutive sections of at most 400 lines, lines 1–2341; final section ends at EOF.
- Purpose: Game-side advancement integration: tree/progress loading, world/entity/item predicates, gameplay criteria, rewards/progress, remote sync, toast and chat announcement.
- Dependencies / cross-references: Depends on Game/world/player/mob/boat/guest state, advancement Tree/Progress/EntityView/ItemView, tags/registries/loot overlays, items/inventory/effects/health/movement/commands/recipes, protocol UpdateAdvancements, HUD/text/audio and per-player JSON save.
- Implementation / logic: Defines world predicate adapter for dimension/biome/structure/block/fluid/sky/light/tags; materializes entity/item views; prefetches/indexes advancement data; loads/saves progress; evaluates inventory/tick/location/damage/item-use/block/entity/recipe/loot/trade/dimension/effect criteria; tracks fall/Nether/levitation marks; grants rewards/announcements; supports grant/revoke commands and queued toasts.
- Findings: Large integration module, not standalone engine. Has newer trigger hooks such as spear_mobs, bee_nest_destroyed, default_block_use and any_block_use; do not assume 1.20.1 support. Predicate adapter has assumptions (fixed 200-block sky scan, damage tags from current tables); remote updates use current protocol types. Tightly coupled to Game internals.
- RustChunk relevance: Very high-level gameplay reference for advancement JSON/progress, criterion dispatch, reward pipeline, trigger instrumentation, multiplayer sync and UI notifications.
- Decision: **adaptable; reference; version-specific — port concepts selectively; rebuild trigger set, predicate semantics, packet adapter, save format and game hooks for Java 1.20.1.**
- 1.20.1 implications: Reconstruct 1.20.1 advancement JSON/criteria/loot rewards, player predicate fields, tags, trigger set, progress JSON/DataVersion and ClientboundUpdateAdvancements schema; exclude later triggers unless verified.
- Follow-up checks: Build trigger-to-source map for every 1.20.1 vanilla criterion; test OR/AND requirements, partial progress, reset/removal, hidden/display-less, rewards, remote sync, item/entity predicates, dimension/effects/damage and toast queue.
- RustChunk implementation mapping: Do not port as one file; proposed split: engine/advancements/tree, progress, predicates, triggers and UI toasts, connected only after event-level parity.
- Evidence required before adoption: Java 1.20.1 source and golden fixtures for every trigger family, progress JSON round-trip, packet bytes, multiplayer sync and GUI screenshots.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.

### 1040. `crates/horizon-client/src/ambience.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `98cfe132d091a3b2a5e47c2d2be6e27cba1523fe`
- Size: 18126 bytes; 479 logical lines
- Coverage: Complete text file reviewed in consecutive sections of at most 400 lines, lines 1–479; final section ends at EOF.
- Purpose: Local-player ambient sound handlers: biome loops/cross-fades, mood/cave ambience, random additions, underwater transitions/subsounds and bubble-column sounds.
- Dependencies / cross-references: Uses SoundEngine/SoundInstance/Sounds, LegacyRandom, ambient/player categories, JSON dimension/biome tables, player position/eye height/spectator/underwater flags, light sampler and bubble-column lookup.
- Implementation / logic: Parses loop/mood/additions; resolves biome sound overriding dimension defaults; finds bubble columns in adjusted player AABB; tracks underwater/bubble/first-tick state; plays enter/exit and subsounds; cross-fades biome loops; samples additions chances and accumulates mood from brightness/random positions.
- Findings: Good separation of audio policy from backend. But loader expects modern attributes.minecraft:audio/ambient_sounds in biome/dimension JSON; 1.20.1 uses older biome effects ambient_sound/mood_sound/additions_sound schema. No Android playback/focus/latency validation.
- RustChunk relevance: Relevant to immersive audio and deterministic sound scheduling; bubble-column and underwater patterns map to gameplay. Sound definitions, schema and RNG sequencing require versioning.
- Decision: **adaptable; reference; version-specific — keep handler/state-machine ideas, rewrite resource loading and compare loop/mood/addition behavior with Java 1.20.1.**
- 1.20.1 implications: Translate 1.20.1 biome effects ambient_sound/mood_sound/additions_sound into internal model; validate cave mood, underwater, bubble columns, spectator behavior and sound categories.
- Follow-up checks: Test biome cross-fade, none/one/many additions, mood/light thresholds, underwater transitions/rare additions, bubble-column AABB edges and first tick; Android AudioFocus/device tests.
- RustChunk implementation mapping: QA-only candidate tests/parity/starling/ambience_fixture.rs; production is audio policy/state machine plus separate Android backend.
- Evidence required before adoption: Java 1.20.1 biome JSON/sound event tables and deterministic traces, plus target-device playback/focus/latency evidence.
- Completion state: FULL TEXT REVIEW COMPLETE — source read in full. No tests, build, benchmark, APK or device execution was performed.


## Checkpoint 1021–1040

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
