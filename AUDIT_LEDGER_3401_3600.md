# RustChunk Starling Audit Ledger — entries 3401–3600

- Repository: `starling-client/starling`
- Pinned ref: `5432370d573642041585c69b40fb045b27133488`
- Target assessment: RustChunk Minecraft Java 1.20.1 / Forge 47.x / Android
- Authorized scope: entries 3401–3600 under one continuous 200-file user grant dated 2026-11-26.
- Selection: first 200 raw-lexically sorted unindexed blobs from the complete pinned recursive tree, excluding all 3,400 existing Index paths.
- Checkpoint cadence: eight sequential 25-file checkpoints; commit and six-document read-back verification after each. No permission stop at 50-file subdivisions for this grant.
- At registration this Ledger contains **0 source entries**. Do not infer inspection from the registration commit.

### 3401. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaEntityParticlesOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 06b1eb53d10e4b715f25fca90f7dcad61f88833a
- Size: 4598 bytes
- Coverage: Text through EOF (78 logical lines).
- Purpose: Capture vanilla entity handleEntityEvent particle bursts for a fixed selection of entity classes and record spawned particle parameters and RNG continuation.
- Dependencies / cross-references: Minecraft client Entity/ClientLevel, ParticleOptions, BuiltInRegistries, Unsafe and reflective Entity fields; MC26.3 class/package surface.
- Implementation / logic: Allocates selected entity instances without constructors, injects a stand-in ClientLevel, feet position, dimensions and seeded RandomSource, invokes handleEntityEvent, intercepts addParticle and prints particle registry path, raw coordinate/velocity bits and next RNG long.
- Findings: Unsafe/reflection and constructor-free entity shells are brittle; entity selection includes later-version classes such as Nautilus; only body-particle events and a subset of types are represented; input entity names and array indices are unchecked.
- RustChunk relevance: Useful reference for differential event/particle tests where entity events trigger client-side particle effects.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Entity set, names and event implementations differ from 1.20.1; rebuild fixtures against 1.20.1 entity registry and event handlers; not a runtime adapter.
- Follow-up checks: Compare 1.20.1 Entity.handleEntityEvent cases, particle IDs, emission order, coordinate formulas and RNG consumption; validate fixture inputs and unknown entity handling.
- RustChunk implementation mapping: Port only the deterministic particle-event expectations into RustChunk's target-version tests; implement effects in native client runtime separately.
- Evidence required before adoption: Pinned source provenance and EOF read verified; require matched 1.20.1 oracle outputs and deterministic-seed regressions before using as an acceptance test.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3402. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaEntityTypesExporter.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 57205887172005e7828fe5b778ae493c3f2df7c8
- Size: 1948 bytes
- Coverage: Text through EOF (41 logical lines).
- Purpose: Export registered entity type dimensions, category, fire immunity and living-entity default attribute values to JSON.
- Dependencies / cross-references: SharedConstants, Bootstrap, BuiltInRegistries.ENTITY_TYPE/ATTRIBUTE and DefaultAttributes.
- Implementation / logic: Bootstraps vanilla registries, iterates every entity type, records dimensions and classification, and queries DefaultAttributes for types that have a supplier.
- Findings: Exporter relies on a raw generic cast for living entity attribute suppliers and assumes output argument exists; output schema does not include implementation behavior or runtime entity state.
- RustChunk relevance: Reference data for target entity metadata and registry-driven validation.
- Decision: adaptable; reference; parity-data-exporter
- 1.20.1 implications: Registry members, dimensions, entity types and attributes are highly version-specific; later entity types must not leak into the 1.20.1 target.
- Follow-up checks: Regenerate from a 1.20.1 vanilla/Forge baseline and compare every entity key, dimensions, category, fire immunity and attribute default.
- RustChunk implementation mapping: Use as a model for an offline parity-data exporter; do not ship generated later-version data as target data.
- Evidence required before adoption: Pinned SHA/size and full 41-line EOF coverage verified; require target-version export, schema checks and registry completeness comparison.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3403. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaEnvironmentOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: df1d7b02898a6e59807d4d0fdb1560f67a44992f
- Size: 5325 bytes
- Coverage: Text through EOF (98 logical lines).
- Purpose: Resolve vanilla environment attributes for dimension, biome, timelines and weather at supplied ticks/rain/thunder values and emit supported values.
- Dependencies / cross-references: MC26.3 EnvironmentAttributeSystem, biome/dimension registries, clocks, TagLoader, server data pack resource manager and JOML vectors.
- Implementation / logic: Loads vanilla data resources and world registries, builds constant and positional biome layers, adds timeline layers and weather layers where dimension permits, then queries all registered environment attributes at Vec3.ZERO.
- Findings: Only Float, Boolean, Enum and JOML vector values are serialized; unsupported types silently disappear; fixture parsing uses simple space splitting and hand-built JSON values; resource managers are not explicitly closed.
- RustChunk relevance: Reference for understanding environment-driven render/gameplay attribute layering and constructing a later differential harness.
- Decision: adaptable; reference; version-specific; parity-test-oracle
- 1.20.1 implications: EnvironmentAttribute/clock/timeline APIs are later-version architecture and not drop-in 1.20.1 APIs; derive equivalent 1.20.1 sky/weather/biome behavior independently.
- Follow-up checks: Check omitted value types, JSON escaping, malformed fixture handling, pack reload/resource closure and per-attribute layer precedence; compare relevant 1.20.1 environment/weather source.
- RustChunk implementation mapping: Map only the testing concept of layered environment inputs and target outputs into RustChunk tests; do not transplant MC26.3 API code.
- Evidence required before adoption: Pinned SHA/size and 98-line EOF coverage verified; require target-version behavior matrix and serialization completeness tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3404. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaExtraItemsOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 80b4f0b834d4dc91f3ef132c6d182d8057cc6b46
- Size: 8474 bytes
- Coverage: Text through EOF (29 logical lines).
- Purpose: Exercise real ItemStack.finishUsingItem behavior combining use remainders, item templates, cooldowns, inventory insertion/drop and callback mutation/failure.
- Dependencies / cross-references: ItemStack/data components, UseRemainder, ItemStackTemplate, UseCooldown, ItemCooldowns, ServerPlayer/ArmorStand stand-ins, Unsafe and reflection.
- Implementation / logic: Creates scripted custom items and actors, builds stacks through a reflected constructor, mutates global item component prototypes, traces finish/cooldown/inventory callbacks and aliasing, then captures states under injected failure or mutation.
- Findings: Highly invasive harness mutates built-in registry-holder component defaults and ItemStack.EMPTY, allocates actors unsafely, depends on a private ItemStack constructor/fields and modern components; shared global state may contaminate a longer-lived test JVM.
- RustChunk relevance: High-value behavioral reference for item consumption, remainders, cooldowns and inventory ownership/aliasing.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: UseRemainder, ItemStackTemplate and UseCooldown are modern data-component APIs; reconstruct the matching semantics for 1.20.1 Forge item use and container APIs.
- Follow-up checks: Run only in isolated JVM; audit restoration/global leakage; test remainder stack limits, creative vs survival, aliasing, inventory-full drop, cooldown grouping, exception order and target-version equivalents.
- RustChunk implementation mapping: Convert observed scenarios into RustChunk's isolated item-use regression tests; production inventory and item logic must use target-version design.
- Evidence required before adoption: Pinned SHA/size and 29-line EOF coverage verified; require isolated execution, state-leak checks and 1.20.1 matched outputs before adoption.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3405. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFaceSupportOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 4e816024b797c30dee4c7102eee5fb920fb29ea1
- Size: 5280 bytes
- Coverage: Text through EOF (56 logical lines).
- Purpose: Probe VoxelShape face extraction, nested face shapes, coordinate grids, bounds and block support predicates for synthetic geometries.
- Dependencies / cross-references: VoxelShape/CubeVoxelShape/ArrayVoxelShape, DiscreteVoxelShape implementations, Direction and SupportType; Unsafe/reflection into StateHolder.owner.
- Implementation / logic: Constructs voxel occupancy from explicit axes and bit cells or unions AABBs, optionally translates it, emits face geometry and calls FULL/CENTER/RIGID support tests against an instrumented Block.
- Findings: Fixture dimensions and occupied-cell count are assumed consistent; reflective owner wiring bypasses normal BlockState construction; nested face extraction can multiply output and is not bounded by the harness.
- RustChunk relevance: Useful for collision, support-face and shape-culling parity tests.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Voxel shape algorithms may be similar but exact classes, caches and edge behavior must be compared with 1.20.1; it is not Android rendering evidence.
- Follow-up checks: Test empty/full/cube/array shapes, nonuniform and degenerate coordinates, out-of-range inputs, translated bounds, nested face identity and support invocation counts.
- RustChunk implementation mapping: Use as a target-version shape oracle for collision/support regression tests; write Rust shape representation separately.
- Evidence required before adoption: Pinned SHA/size and 56-line EOF coverage verified; require 1.20.1 output comparisons and malformed-shape tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3406. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFaceVisibilityOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: c88bcc7ed22d63be3ef5704f31ee0d21162a327b
- Size: 3897 bytes
- Coverage: Text through EOF (68 logical lines).
- Purpose: Export block-state occlusion metadata and compare native skipRendering/shouldRenderFace results for pairs of block states with leaf cutout toggled.
- Dependencies / cross-references: BuiltInRegistries, Block.skipRendering, Block.shouldRenderFace, BlockTags, NBT/JSON; cross-calls VanillaHeightmapOracle and VanillaCollisionOracle.
- Implementation / logic: Classifies skip-rendering owner methods by a hard-coded class-name switch; export mode writes block/occlusion facts; comparison mode evaluates both face methods for each direction and toggles static LeavesBlock cutout state.
- Findings: Unknown override owners intentionally throw; static cutout flag is mutated without a restoration guard; relies on other test exporters and registry IDs; output captures logic but not GPU renderer behavior.
- RustChunk relevance: Useful for face-visibility and block-model culling parity, not a substitute for actual mesh/render tests.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Block classes, override ownership, tags, state IDs and cutout behavior can differ in 1.20.1; rebuild classification on the target registry.
- Follow-up checks: Exercise every override owner; restore static global cutout value in finally; test pair ordering, state IDs, tags, and compare culling decisions with 1.20.1.
- RustChunk implementation mapping: Use as a reference exporter/oracle for RustChunk's block-face culling test inputs and expected decisions.
- Evidence required before adoption: Pinned SHA/size and 68-line EOF coverage verified; require full target registry owner inventory and renderer-level validation.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3407. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFallDamageOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 7c16307dc20ad4749fda5c98d62a28b34fe2750e
- Size: 5243 bytes
- Coverage: Text through EOF (53 logical lines).
- Purpose: Trace Entity.checkFallDamage behavior, support-position handling, fallOn callback, ground-hit game event and reset/mutation order.
- Dependencies / cross-references: Entity, Marker, ServerLevel/BlockGetter stand-ins, BlockState and GameEvent; Unsafe/reflection for entity position and supporting-block field.
- Implementation / logic: Builds an Actor and Level shell, instruments water checks, block lookup, fallOn and HIT_GROUND event, sets initial fall distance/position/support and injects failure or mutation during callbacks.
- Findings: Only selected world callbacks are modeled; callback state changes are scripted; fake queried block state and marker entity do not represent full gameplay; private field names are version-sensitive.
- RustChunk relevance: Reference for fall damage, landing, support state and event-order regression tests.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Entity method signatures and landing semantics need direct comparison to 1.20.1; keep server/Forge callbacks separate from the mobile renderer.
- Follow-up checks: Compare grounded vs airborne, water, zero/negative/large fall distance, support position present/absent, fallOn mutation, callback failure and game-event ordering.
- RustChunk implementation mapping: Port scenarios and event trace into target-version movement/physics tests, not into production code from this oracle.
- Evidence required before adoption: Pinned SHA/size and 53-line EOF coverage verified; require target source comparison and deterministic event-order regressions.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3408. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFeatureCatalogOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: b6439ffa8cc7486bc853c60c82f05abc1018448d
- Size: 10049 bytes
- Coverage: Text through EOF (123 logical lines).
- Purpose: Load synthetic feature/placed-feature catalogs and invoke native feature placement in an observable mutable world.
- Dependencies / cross-references: Feature/PlacedFeature codecs, RegistryOps, MappedRegistry, TagLoader, PlacementModifier, FeaturePlacer, WorldGenLevel proxy; reflectively reuses VanillaPlacementOracle.context.
- Implementation / logic: Registers feature and placed-feature definitions plus tags, decodes either feature or wrapped placed root, constructs a restricted WorldGenLevel proxy, records reads/writes and random draws across repeated placements with failure/rejection injection.
- Findings: Registry lookup supports only selected registries and proxy only selected world methods; reflective context reuse couples it to another oracle; catches a narrow set of decode/runtime errors; synthetic world is not a loaded chunk engine.
- RustChunk relevance: Valuable fixture architecture for feature decoding, placement ordering, RNG and block-write differential tests.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Registry codecs and worldgen feature APIs are MC26.3-specific; 1.20.1 codec schemas, random algorithms and WorldGenRegion write semantics differ.
- Follow-up checks: Validate invalid references/tags, unexpected proxy calls, rejected writes, partial failures, repeated placement and random continuation using 1.20.1 registries.
- RustChunk implementation mapping: Reuse the test pattern to drive a 1.20.1 worldgen adapter test harness; keep codec/registry implementation target-specific.
- Evidence required before adoption: Pinned SHA/size and 123-line EOF coverage verified; require full proxy-method contract review and target-worldgen oracle comparison.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3409. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFeatureGraphOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: b210ea8a4d7f45d081ae54bf24f4c3535bc379c4
- Size: 6024 bytes
- Coverage: Text through EOF (79 logical lines).
- Purpose: Exercise native Feature codec and FeaturePlacer with a deterministic dummy world, scripted placements and state-write tracing.
- Dependencies / cross-references: Feature.DIRECT_CODEC, PlacementModifier, FeaturePlacer, RandomSource/LegacyRandomSource/XoroshiroRandomSource and WorldGenLevel proxy.
- Implementation / logic: Decodes direct feature definitions and optional modifiers, builds a deterministic proxy world with synthetic default blocks, invokes placer repeatedly, captures block reads/writes, rejected writes, errors and next RNG values.
- Findings: WorldGenLevel proxy supplies only a narrow method subset and returns procedural fake states; input/codec errors are only partly caught; output reflects the harness world, not generated chunks.
- RustChunk relevance: Useful for placement modifier order, failure and RNG regression inputs.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Feature codecs, PlacementModifier order and RNG implementation are version-sensitive; compare with Forge 1.20.1 worldgen pipeline.
- Follow-up checks: Test empty modifier lists, malformed definitions, modifier failures, read/write exceptions, rejected writes, repeated calls and RNG continuity against target version.
- RustChunk implementation mapping: Build a target-version feature placement oracle over controlled fake WorldGenLevel/ChunkGenerator boundaries.
- Evidence required before adoption: Pinned SHA/size and 79-line EOF coverage verified; require 1.20.1 exact-output comparison and expanded proxy-contract coverage.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3410. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFeaturePlacementOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 37727e3c3954b14b2de36d68d12e75aa2d6eb9b9
- Size: 3702 bytes
- Coverage: Text through EOF (53 logical lines).
- Purpose: Compare PlacementModifier chains and FeaturePlacer position emission for ordered fixture positions and optional random draws.
- Dependencies / cross-references: PlacementModifier codec, FeaturePlacer, ChunkGenerator, PlacementContext and Feature.DIRECT_CODEC; reflection into VanillaPlacementOracle.context.
- Implementation / logic: Decodes modifiers, provides a synthetic Feature that logs each visited position and configurable success result, and records random draws for each input position while injecting a selected failure call.
- Findings: Depends on private/reflected helper from another oracle and catches only IllegalArgumentException/Stop around placement; the custom feature is not a real world-writing feature.
- RustChunk relevance: Reference for modifier order, feature invocation count, boolean result propagation and random consumption.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modifier set, codec and invocation contracts in 1.20.1 differ from later code; preserve target vanilla ordering rather than reusing this implementation.
- Follow-up checks: Compare every supported modifier's order, position transform, conditional skip, seed behavior, exceptions and false/true aggregation.
- RustChunk implementation mapping: Translate test cases to Forge 1.20.1 placement pipeline without importing MC26.3 classes.
- Evidence required before adoption: Pinned SHA/size and 53-line EOF coverage verified; require target codec/placement parity tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3411. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFeatureRegionOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: e6acbf117ec370a4fb2b230cb2d35b270c37fe35
- Size: 20760 bytes
- Coverage: Text through EOF (248 logical lines).
- Purpose: Exercise native WorldGenRegion/ProtoChunk feature writes, chunk write zones, height gates, retrogen lifecycle, ticks, heightmaps and resident section data.
- Dependencies / cross-references: WorldGenRegion, ProtoChunk, PalettedContainerFactory, BelowZeroRetrogen, feature codecs, WorldGenTickAccess, block/fluid registries and extensive Unsafe/reflection.
- Implementation / logic: Creates a 3x3 resident protochunk map, routes real feature placement and read/write calls through WorldGenRegion/WorldGenLevel transport, records zone and Y-range permissions, upgrade lifecycle, ticks, height maps, palette sections and failure/rejection effects.
- Findings: Relies on private field names and Unsafe for WorldGenRegion state; limited 3x3 map can reject/out-of-range coordinates; reloads global fluid/block tags; test fixture fixes one biome and selected height/data conditions; high-complexity oracle is version-locked.
- RustChunk relevance: Highest relevance in this checkpoint for chunk-generation correctness, section writes, retrogen and chunk-stage consistency.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: MC26.3 chunk statuses, write radius, dimension height and retrogen transitions are not proof of 1.20.1 Forge behavior; map to actual 1.20.1 ProtoChunk/WorldGenRegion.
- Follow-up checks: Validate all edge chunks, negative coordinates, missing region chunks, min/max Y, write radius, retrogen stages, tick ordering, rejected writes, exceptions and global tag restoration.
- RustChunk implementation mapping: Use its scenarios to define a 1.20.1 chunk/worldgen parity suite; implement RustChunk chunk storage and generation independently.
- Evidence required before adoption: Pinned SHA/size and 248-line EOF coverage verified; require 1.20.1 reference outputs, isolated registry resets and full chunk-boundary tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3412. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFeatureSortOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 833ede52d7f2b75453f2f315fc7c581454b0e882
- Size: 2765 bytes
- Coverage: Text through EOF (45 logical lines).
- Purpose: Probe FeatureSorter step grouping and identity-based ordering of placed features referenced by biome feature lists.
- Dependencies / cross-references: FeatureSorter, Holder/HolderSet<PlacedFeature>, PlacedFeature, noop Feature codec and IdentityHashMap.
- Implementation / logic: Creates stable dummy placed-feature objects keyed by integer identity, builds per-biome step lists, runs FeatureSorter and records step order/indexMapping; detects cycle exceptions.
- Findings: Uses dummy payloads with no feature semantics and identity-keyed fixtures; catches only IllegalStateException; does not validate placement execution.
- RustChunk relevance: Useful for biome feature ordering/scheduling parity.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: FeatureSorter signatures and ordering semantics may change between MC26.3 and 1.20.1; preserve target registry identity and ordering rules.
- Follow-up checks: Test duplicate holder identity, equal-valued distinct objects, absent queries, empty biome steps, cycles and stable ordering on 1.20.1.
- RustChunk implementation mapping: Adopt test method for the target worldgen feature-step planner; keep data model native to 1.20.1.
- Evidence required before adoption: Pinned SHA/size and 45-line EOF coverage verified; require target vanilla comparison and duplicate-identity regressions.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3413. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFillLayerOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: fa375278646d4898adbf93649a1f123c8d4315bb
- Size: 5449 bytes
- Coverage: Text through EOF (76 logical lines).
- Purpose: Exercise the native fill-layer feature against a synthetic world and capture world-minimum queries, block reads/writes and random continuation.
- Dependencies / cross-references: Feature.DIRECT_CODEC, WorldGenLevel, BlockState registry, Legacy/Xoroshiro random sources; block-state export delegates to VanillaBlockStateCodecOracle.
- Implementation / logic: Decodes a feature definition, applies it at an origin across repeats, uses an instrumented WorldGenLevel proxy with fake block responses and write rejection/failure, and exports block-state facts including air IDs and height limit.
- Findings: Proxy provides a narrow API and synthetic state distribution; injected min/read/write failures are harness scenarios, not observed runtime faults; feature definition errors are not broadly handled.
- RustChunk relevance: Useful target test for fill-layer and minimum-height behavior in world generation.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Fill-layer algorithm, dimension minimum Y, air state IDs and random calls must be rechecked against 1.20.1; modern height semantics cannot be copied uncritically.
- Follow-up checks: Test negative minY, non-aligned bounds, air variants, rejection, failure order, repeat determinism and `getMinY` call behavior.
- RustChunk implementation mapping: Use as a target-version feature fixture and compare actual placed block states in RustChunk's chunk pipeline.
- Evidence required before adoption: Pinned SHA/size and 76-line EOF coverage verified; require 1.20.1 feature outputs and boundary-height tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3414. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFinalizeSpawnOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 4dd8b6ece3fe3c3d923e93f3c4f0c21be3d2ebd6
- Size: 33163 bytes
- Coverage: Text through EOF (527 logical lines).
- Purpose: Run the real Mob.finalizeSpawn path for mob types, spawn reasons, difficulty, local difficulty, structures, environment and mob packs, exporting full entity state and RNG continuation.
- Dependencies / cross-references: EntityType factories, Mob/ServerLevel, data-pack registries, attributes/components/brain, StructureManager/StructureStart, Bootstrap, Unsafe and reflective private fields.
- Implementation / logic: Bootstraps vanilla server registries/data, constructs stand-in ServerLevel/structure manager, wraps every EntityType factory to seed and track created entities, creates mobs and jockeys, calls finalizeSpawn with group data, then serializes each entity state and per-entity/level RNG draws.
- Findings: 527 logical lines; read sequentially as ranges 1–400 and 401–527. Mutates every global EntityType factory and relies on Unsafe/private field offsets; System.exit(0) terminates the process after execution; catches Throwable per case; not safe as an in-process reusable test helper.
- RustChunk relevance: Very high-value reference for mob spawning parity, RNG order, pack/group data, jockey/passenger creation, equipment, variants, effects, attributes and brain memories.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Many referenced mobs, spawn-reason values, registries and data components are MC26.3-specific; create a 1.20.1 Forge fixture from its own EntityType/Mob/SpawnPlacements and finalizeSpawn implementation.
- Follow-up checks: Run only in a disposable JVM; check factory mutation isolation, exception paths, factory restoration policy, all spawn reasons/difficulties, group data, jockeys, structure/floor/roof and per-entity RNG; compare with 1.20.1.
- RustChunk implementation mapping: Use as a specification for target spawn regression scenarios and golden outputs, not as a runtime spawner or factory wrapper.
- Evidence required before adoption: Pinned SHA/size and 527 logical lines verified; complete coverage recorded 1–400 and 401–527; execution and all 1.20.1 comparisons remain pending.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3415. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFireStarterSoundOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 276a0aae322b18b1c351c9dc45f6ae4437ae683d
- Size: 3186 bytes
- Coverage: Text through EOF (66 logical lines).
- Purpose: Capture the private FireChargeItem.playSound sound choice, volume/pitch bit patterns and RNG draw consumption for fixed input seeds.
- Dependencies / cross-references: FireChargeItem, Level/ServerLevel stand-in, RandomSource, ParityData and Unsafe/reflection into private playSound.
- Implementation / logic: Allocates a fake ServerLevel without constructor, seeds Level random and sound seed generator, reflectively invokes playSound at one fixed BlockPos and records sound identifier, volume/pitch bits and next RNG long.
- Findings: Tests only a private sound helper with one fixed position, not the whole fire-charge interaction or client/server effects; uses Unsafe/reflection and System.exit(0).
- RustChunk relevance: Reference for sound/RNG deterministic unit tests.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Sound events, helper visibility and exact pitch/randomization may differ in 1.20.1; the target use behavior must be inspected independently.
- Follow-up checks: Compare lit campfire/candle and placed-fire cases, RNG stream, event IDs, sound source and playback side in 1.20.1.
- RustChunk implementation mapping: Port only seed-based expected sound/RNG assertions into target item-use tests.
- Evidence required before adoption: Pinned SHA/size and 66-line EOF coverage verified; require target source comparison and tests for all use contexts.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3416. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFireworkTooltipOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 749a8c110fc009bcf0039da392968050ef4e1b0f
- Size: 1842 bytes
- Coverage: Text through EOF (40 logical lines).
- Purpose: Capture fireworks and firework-explosion component tooltip strings under normal tooltip display filtering.
- Dependencies / cross-references: ParityData item-stack SNBT decoder, Item.TooltipContext, TooltipProvider, TooltipDisplay, TooltipFlag and FIREWORKS/FIREWORK_EXPLOSION data components.
- Implementation / logic: For each input stack, reads tooltip display, invokes each visible component provider with NORMAL flag and appends component-produced text; comment lines are skipped.
- Findings: Captures provider strings only; does not model complete ItemStack tooltip assembly, ordering with other components, font/layout, tooltip context variants or rendering; relies on modern component types.
- RustChunk relevance: Reference for item component tooltip logic and hide/show behavior.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Firework component APIs and tooltip assembly changed across Minecraft versions; recreate equivalent 1.20.1 ItemStack tooltip pipeline and component visibility.
- Follow-up checks: Test hidden/visible components, invalid SNBT, empty providers, translation keys, order, advanced flag and other ItemStack tooltip contributions on 1.20.1.
- RustChunk implementation mapping: Use as a focused tooltip-component oracle only, behind a broader target ItemStack tooltip harness.
- Evidence required before adoption: Pinned SHA/size and 40-line EOF coverage verified; require full target tooltip composition comparisons.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3417. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFirstPersonOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 1ddaa64b7eaf098faa71b2cab91ce1e53735376c
- Size: 21567 bytes
- Coverage: Text through EOF (398 logical lines).
- Purpose: Drive first-person attack, block breaking, item use, swing state, hand-height animation, item swap and input tickers through scripted Minecraft client steps.
- Dependencies / cross-references: Minecraft, LocalPlayer, MultiPlayerGameMode, FirstPersonHandsAndItems, Inventory/PlayerEquipment, ClientPacketListener, ItemModelResolver, AttributeMap, Unsafe/reflection.
- Implementation / logic: Builds client/player/level/connection shells, scripts hits and use results, replays selected handleKeybinds/tick order for attack/use/destroy, applies input/rotation, updates swing/equipment tickers and first-person hand animation, and logs events and timer/position outputs.
- Findings: 398 logical lines; uses Unsafe and many private fields; Connection.send and renderer-facing objects are stubs, item/block/entity interaction results are scripted, Level block writes always succeed, and the code manually mirrors selected tick order rather than running the full rendered client.
- RustChunk relevance: Useful for deterministic input/use/attack/swing and first-person hand-state regressions; not evidence of GPU rendering, camera correctness or real server interaction.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Control flow, mappings, client lifecycle and hand animation differ in MC26.3; Android touch input and Forge 1.20.1 key bindings need a target implementation.
- Follow-up checks: Compare call order for attack/use overlap, held/released input, block destroy delays, miss reset, creative mode, haste/fatigue, equipment swap, swing animations and multiple ticks against 1.20.1.
- RustChunk implementation mapping: Use its scenario model to test RustChunk input state and first-person animation independently from rendering and networking.
- Evidence required before adoption: Pinned SHA/size and 398-line EOF coverage verified; require 1.20.1 source mapping, event-sequence regressions and real render/input integration tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3418. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFloatLogOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 7ffc321387c95aa373fdb0bd01a1d0655baec41f
- Size: 1681 bytes
- Coverage: Text through EOF (32 logical lines).
- Purpose: Stream the vanilla compiled logarithmic density sampler's output for a specified contiguous range of 32-bit float bit patterns.
- Dependencies / cross-references: UnaryFunction.LogSampler, DensitySampler/SamplerContext/DensityBuffer/DensityVolume and buffered stdout transport.
- Implementation / logic: Treats each input index as raw float bits, passes that float through vanilla LogSampler.sampleValue, canonicalizes output with Float.floatToIntBits and writes big-endian four-byte words in bounded 1 MiB blocks.
- Findings: Argument count is read through args[0]/args[1] before validation; start+remaining can overflow long; Float.floatToIntBits canonicalizes NaN payloads; output has no embedded range metadata and can be extremely large for malformed bounds.
- RustChunk relevance: Useful for exact density-function math and float edge-case golden data.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: The logarithmic density operation and density sampler APIs must be compared with 1.20.1; bit-level float behavior and NaN canonicalization matter for deterministic world generation.
- Follow-up checks: Validate argument count before access, use overflow-safe range bounds, cap output volume, test signed zero/subnormals/infinities/NaNs and verify stream byte order.
- RustChunk implementation mapping: Use as a bounded offline oracle generator; never run this potentially large sweep inside the Android client.
- Evidence required before adoption: Pinned SHA/size and 32-line EOF coverage verified; require safe range validation and target-version output comparisons.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3419. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFluidBoxOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 97b5fdaab63b2d042b10412600813ef4d0c6eab0
- Size: 4028 bytes
- Coverage: Text through EOF (25 logical lines).
- Purpose: Exercise the default entity fluid-interaction AABB and boat passenger-fluid box modification, including vehicle chains, boat status, callbacks and mutation.
- Dependencies / cross-references: Entity.getFluidInteractionBox, AbstractBoat/Boat, AABB, Marker stand-ins, private boat status field and reflected helper.
- Implementation / logic: Creates actor/boat/optional vehicle shells with scripted boxes/status; queries actor vehicle and boat under-water state, optionally invokes the boat modifier directly, and records returned bounds and callback count under injected failure/mutation.
- Findings: Unsafe shell objects and private status/helper reflection are version-sensitive; only selected vehicle types and simplified modifier callbacks are modeled; boat implementation package and status enum are later-version-specific.
- RustChunk relevance: Reference for entity/fluid collision bounds and passenger behavior.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Boat/entity classes and fluid interaction methods need 1.20.1 verification; do not infer target collision geometry from later API.
- Follow-up checks: Test no vehicle, boat, other vehicle, modifier vehicle, underwater/status states, null/degenerate AABBs, mutation and callback failure.
- RustChunk implementation mapping: Build target-version entity fluid-box tests over RustChunk collision geometry.
- Evidence required before adoption: Pinned SHA/size and 25-line EOF coverage verified; require target entity/boat AABB comparisons.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3420. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFluidFlowOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 4c41cd9344d8206d2cb06bf3d157b1adf12974fc
- Size: 4183 bytes
- Coverage: Text through EOF (29 logical lines).
- Purpose: Exercise native flowing-fluid current vectors and solid-face checks with independently scripted neighbor fluid/block states.
- Dependencies / cross-references: FlowingFluid.getFlow, private FlowingFluid.isSolidFace, FluidState codec, BlockGetter proxy, BlockState and BlockTags.BLOCKS_FLUID_FLOW.
- Implementation / logic: Exports registered fluid state codecs or constructs fixture block/fluid arrays, serves neighbor lookups through a proxy, injects query failures, and records raw vector bits or solid-face boolean.
- Findings: Proxy supports only getBlockState/getFluidState; fixture indexes and enum direction are unchecked; support face behavior is simulated from supplied booleans/face bits rather than a real block world.
- RustChunk relevance: Useful for fluid-current and block-face influence tests.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Flowing-fluid classes, codec schemas and neighbor rules must be matched to 1.20.1; fluid speed/current is gameplay/physics, separate from renderer.
- Follow-up checks: Test all six directions, water/lava and flowing/source states, falling fluids, solid-face tags, failures at each lookup and negative coordinates.
- RustChunk implementation mapping: Use as a differential fixture for RustChunk's target fluid simulation; keep world transport and tags version-native.
- Evidence required before adoption: Pinned SHA/size and 29-line EOF coverage verified; require 1.20.1 fluid lookup trace/output comparisons.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3421. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFluidOccupancyOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: aa43ea1c835e087164eb7d52b497e0d247c8be94
- Size: 4707 bytes
- Coverage: Text through EOF (24 logical lines).
- Purpose: Probe LevelReader.containsAnyLiquid using real ProtoChunk palette sections and block-state fluid occupancy.
- Dependencies / cross-references: ProtoChunk, PalettedContainerFactory, LevelReader/LevelHeightAccessor, BlockState registry and NBT data exporter.
- Implementation / logic: Builds a one-chunk palette-backed world shell, writes scripted block states directly into chunk sections, traces section/range/chunk queries and invokes containsAnyLiquid over supplied AABBs; exports state registry facts in NBT/JSON.
- Findings: Returns the same chunk for any requested coordinates; input height/section alignment and block IDs are assumed valid; direct writes bypass normal world write gates and trace/failure injection covers only selected queries.
- RustChunk relevance: Relevant to collision/fluid occupancy queries over chunk storage and palette data.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Chunk section indexing, world bounds and containsAnyLiquid semantics need explicit comparison with 1.20.1; one-chunk fixture does not establish cross-chunk handling.
- Follow-up checks: Test negative minY, top/bottom boundary, cross-section/cross-chunk AABBs, empty and waterlogged states, malformed bounds, and missing chunk cases.
- RustChunk implementation mapping: Use the scenario pattern for chunk-backed fluid occupancy tests after implementing 1.20.1 section storage.
- Evidence required before adoption: Pinned SHA/size and 24-line EOF coverage verified; require boundary and multi-chunk target tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3422. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFluidOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 7e1c08f8259340b099e9315dfbc5bb5e9dc17b54
- Size: 4468 bytes
- Coverage: Text through EOF (57 logical lines).
- Purpose: Export fluid-family relationships, all registered block-state fluid properties, codec round trips and height/camera-height calculations.
- Dependencies / cross-references: BuiltInRegistries.FLUID/BLOCK, FluidState.CODEC, FlowingFluid properties, BlockGetter proxy, Blocks and JsonOps.
- Implementation / logic: Enumerates all fluid pairs and block states, decodes/encodes fluid-state fixtures with canonical property ordering, then computes getHeight/getHeightForCamera from a scripted above-fluid/block lookup with call failure injection.
- Findings: Codec failures are partly skipped; hand fixture indexes are unchecked; output includes only selected properties and fake neighbor behavior; helper calls continue with one cumulative counter across camera/non-camera paths.
- RustChunk relevance: High relevance for target fluid representation and collision/flow heights.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Fluid state schema, family equivalence, amount/height semantics and camera height need 1.20.1/Forge comparison; avoid copying modern registry IDs.
- Follow-up checks: Check round-trip canonicalization, all registered states, source/falling/full flags, family equivalence, camera vs non-camera queries, failures and legacy block conversion.
- RustChunk implementation mapping: Reuse as a target-version fluid-state exporter/oracle generator for RustChunk's fluid engine and collision queries.
- Evidence required before adoption: Pinned SHA/size and 57-line EOF coverage verified; require target registry export and exact height tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3423. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFluidShapeOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: ac2d7fe10516580a339906341bd3de6e33a3dbd1
- Size: 3210 bytes
- Coverage: Text through EOF (27 logical lines).
- Purpose: Probe registered fluid VoxelShape values, cache identity, and cache population while the above-fluid neighbor changes.
- Dependencies / cross-references: FluidState.CODEC, FlowingFluid static shape cache, VoxelShape/Shapes, BlockGetter proxy and reflective private cache field.
- Implementation / logic: Clears caches for water/flowing water/lava/flowing lava, calls getShape for scripted state/above-state pairs, records shape object identity and boxes, and reports cache sizes with injected neighbor-query failures.
- Findings: Mutates global static fluid shape caches without restoring prior contents; only four built-in fluid objects are cleared/count-tracked; test identity IDs depend on encounter order; no custom-fluid coverage.
- RustChunk relevance: Useful for fluid collision/render-shape cache correctness and identity reuse.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: The cache key and shape logic may differ in 1.20.1; global cache ownership/lifecycle must be checked on target.
- Follow-up checks: Run isolated JVM cases, verify cache reset scope, repeated calls and changed neighbors, empty/full shapes, custom fluids and concurrency assumptions.
- RustChunk implementation mapping: Use as a target-only cache/shape test design; never carry MC26.3 cache internals into RustChunk.
- Evidence required before adoption: Pinned SHA/size and 27-line EOF coverage verified; require target implementation comparisons and isolation-safe cache reset.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3424. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFoodOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 8af1580e3a0c11c0d824a0a5c319588505cb54cf
- Size: 5579 bytes
- Coverage: Text through EOF (48 logical lines).
- Purpose: Exercise FoodData eat/exhaustion/setter/tick arithmetic and record healing/starvation callbacks under mutation/failure injection.
- Dependencies / cross-references: FoodData, FoodProperties, ServerPlayer/ServerLevel stand-ins, GameRules, DamageSources, Difficulty and Unsafe/reflection for private exhaustion/timer fields.
- Implementation / logic: Runs configured operation sequences against real FoodData, intercepts regeneration rule, difficulty, health/hurt/heal and starvation-source callbacks, and logs food level, saturation, exhaustion and timer after each operation.
- Findings: Fake actor heal/hurt callbacks only log and do not model full health/damage application; rule/difficulty/health are scripted; private fields and modern FoodData signatures are version-sensitive.
- RustChunk relevance: Useful for survival food, saturation, exhaustion, regeneration and starvation parity.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: FoodData behavior is target-critical but modern method names/rules may differ; compare exact 1.20.1 hunger tick ordering and difficulty/game-rule checks.
- Follow-up checks: Test all food/saturation bounds, exhaustion thresholds, peaceful/easy/normal/hard, natural regeneration, starvation thresholds, hurt state, callback failure and operation ordering.
- RustChunk implementation mapping: Translate operation fixtures into RustChunk's target survival-simulation tests, keeping real player health and server effects separate.
- Evidence required before adoption: Pinned SHA/size and 48-line EOF coverage verified; require exact 1.20.1 FoodData results and real health-effect tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3425. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFoodPropertiesOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 73050497e732d0e40af247733f18775297a4fc29
- Size: 5544 bytes
- Coverage: Text through EOF (26 logical lines).
- Purpose: Exercise FoodProperties.onConsume with real FoodData selection, sound playback, player/non-player actors, mutation callbacks and two deterministic RNG sources.
- Dependencies / cross-references: FoodProperties, Consumable component, Player/ArmorStand, FoodData, ServerLevel sound callback, ItemStack, RandomSource/XoroshiroRandomSource and Unsafe/reflection.
- Implementation / logic: Creates player or non-player actor with two food-state instances and two RNGs, configures position/state, invokes onConsume and traces getRandom/getFoodData/sound callbacks with injected failure or mutation; records final food values and RNG continuation.
- Findings: Relies on modern Consumable component and onConsume signature, Unsafe actor shells and scripted sound world; only selected sound IDs are accepted; does not run actual inventory/complete item consumption pipeline.
- RustChunk relevance: Reference for eating side effects, sound/RNG ordering and player vs non-player food behavior.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Consumable data component and FoodProperties consumption flow are not directly portable to 1.20.1; map onto 1.20.1 Item.use/finishUsingItem semantics.
- Follow-up checks: Compare player and non-player cases, always-edible/food-level behavior, saturation, sounds, positions, RNG selection/consumption, mutation and callback failures.
- RustChunk implementation mapping: Use as a target-version scenario definition for RustChunk survival/item-consumption tests.
- Evidence required before adoption: Pinned SHA/size and 26-line EOF coverage verified; require 1.20.1 consumption pipeline outputs and complete item-use tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3426. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFoodStorageOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 02d3595f53bae3e3a5a156126193e9400c1adbb5
- Size: 3697 bytes
- Coverage: Complete EOF coverage (24 logical lines).
- Purpose: Exercise FoodData save/load with numeric NBT coercions, invalid/default values, overwritten fields and read-after-write round trips.
- Dependencies / cross-references: FoodData, TagValueInput/TagValueOutput, CompoundTag numeric tags, HolderLookup, ProblemReporter and DataFix-independent NBT storage.
- Implementation / logic: Constructs numeric tags of each NBT numeric/array/string/list/compound kind, invokes readAdditionalSaveData or seeds private timer/exhaustion fields, writes back via addAdditionalSaveData, then reloads and records food/saturation/exhaustion/timer.
- Findings: Uses an empty registry lookup and discarding problem reporter; several tag fixtures use fixed payloads; malformed/missing fields may follow default coercion paths not representative of full world save.
- RustChunk relevance: Useful for persistence compatibility and state round-trip checks.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 FoodData save format predates TagValueInput/Output; preserve target key names and numeric conversion semantics from 1.20.1 source, not the newer API.
- Follow-up checks: Check missing/wrong tag types, numeric overflow/truncation, float NaN/infinity, unrelated-key preservation, duplicate/override behavior and round-trip stability against 1.20.1.
- RustChunk implementation mapping: Translate the data cases into target FoodData serialization tests and migration fixtures.
- Evidence required before adoption: Pinned SHA/size verified; require 1.20.1 save/read output comparisons and malformed-NBT regression set.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3427. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaFrameLimiterOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: d6e2fa11949ae9eb16e7c9122b79ac1961d799c8
- Size: 7473 bytes
- Coverage: Complete EOF coverage (171 logical lines).
- Purpose: Run real FramerateLimiter.limitDisplayFPS against a scripted virtual clock and inspect park/spin/interrupt timing and limiter state.
- Dependencies / cross-references: FramerateLimiter, Java ClassFile API bytecode transform, custom ClassLoader, System.nanoTime, LockSupport.parkNanos, Thread.interrupted/onSpinWait and reflection.
- Implementation / logic: Rewrites four static clock/scheduler calls in the client class to a test Clock, loads the transformed limiter class, sets private state and replays frame limits with deterministic clock-call, overshoot and interrupt sequences.
- Findings: Requires java.lang.classfile (newer JDK than RustChunk's Android/Java baseline); hard-codes exactly eight rewritten call sites; custom loader and class rewrite are brittle; Thread interruption semantics are simulated, not real OS scheduling.
- RustChunk relevance: High relevance to frame pacing and battery/thermal performance hypotheses.
- Decision: adaptable; reference; parity-test-oracle; JDK-specific
- 1.20.1 implications: The timing algorithm must be compared to 1.20.1's FramerateLimiter and Android runtime; this harness itself is not compatible with the current Java 17/Android target toolchain.
- Follow-up checks: Check all limiter transitions, negative/zero FPS, interruption, overshoot bounds, frame-limit changes and JDK-specific bytecode rewriting; compare on Android with measured frame pacing.
- RustChunk implementation mapping: Use its virtual-clock test concept for a Rust-native frame limiter test; never bring Java classfile machinery into the launcher.
- Evidence required before adoption: Pinned SHA/size and 171-line EOF coverage verified; require target-version algorithm comparison and real-device latency/power measurement.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3428. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaGallerySelectionOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 290425f1e20c3db6cc624c5ce7f16f0ba98bea73
- Size: 11613 bytes
- Coverage: Complete EOF coverage (155 logical lines).
- Purpose: Compare native blockstate model dispatch, weighted variant selection, model transforms and face-visibility totals against a Rust scene report.
- Dependencies / cross-references: BlockStateModelDispatcher codec, BlockStateModel roots, ModelBaker/QuadCollection, CuboidModel/ModelDiscovery, LegacyRandomSource, blockstate/model assets, VanillaCuboidModelOracle.map and VanillaHeightmapOracle.bindTags.
- Implementation / logic: Builds a synthetic resident-chunk scene and gallery overrides, parses blockstate/model assets, bakes model roots with a custom tag-producing Baker, compares collected model/transform signatures to expected Rust gallery selections, then verifies scene visible/culled face counts using Block.shouldRenderFace.
- Findings: Custom Baker returns empty geometry and records model tags instead of producing GPU quads; static LeavesBlock cutout flag is set and not restored; only selected blockstates/assets and one synthetic scene are exercised.
- RustChunk relevance: Very relevant to blockstate model choice and deterministic model selection in the renderer.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern model dispatcher, model geometry and resource package structure differ from 1.20.1/Forge; renderer model selection must be mapped to target asset format and random seeding.
- Follow-up checks: Test weighted variants, properties, multipart roots, missing resources, UV lock/rotations, leaf cutout restoration and actual face culling on target 1.20.1.
- RustChunk implementation mapping: Use as a model-selection differential test specification; GPU baking and mobile mesh generation remain separate Rust implementation tasks.
- Evidence required before adoption: Pinned SHA/size and 155-line EOF coverage verified; require target assets, full fixture coverage and rendered mesh/GPU comparisons.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3429. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaGameLoopOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 294ceacac5e454bbb767e6e259bbf1a241d71548
- Size: 19857 bytes
- Coverage: Complete EOF coverage (384 logical lines).
- Purpose: Drive the actual delta timer, tick-rate manager, GUI scale, frame limiter, mouse movement/scroll/turning and toggle-key mapping through JSON input sequences.
- Dependencies / cross-references: DeltaTracker.Timer, ServerTickRateManager, Window, FramerateLimitTracker, MouseHandler, Entity.turn, ToggleKeyMapping, Minecraft/Options; Unsafe and reflection.
- Implementation / logic: Creates an uninitialized Minecraft shell and minimal server/player stubs, then runs separate scripted operations for timer, tick rate, GUI/window scale, inactivity throttling, mouse turn/scroll and toggle key press/release; logs each native state transition.
- Findings: 384 lines; the harness invokes selected methods, not the complete Minecraft game loop; static Minecraft.instance and global options/time-sensitive fields are modified; uses System.exit(0); Util wall-clock paths and SDL/OS input delivery are not fully virtualized.
- RustChunk relevance: High relevance to the launcher input loop, timing, controls and UI scaling.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern timer/input APIs differ from 1.20.1 and real Android touch dispatch; this is a behavioral reference only, not Android input validation.
- Follow-up checks: Compare paused/frozen/tick-rate transitions, scale limits, inactivity throttle, mouse sensitivity/inversion/smoothing, discrete scrolling, key restore and timing determinism with 1.20.1.
- RustChunk implementation mapping: Derive RustChunk frame/tick/input state-machine tests and Android mapping tests from the scenarios.
- Evidence required before adoption: Pinned SHA/size and 384-line EOF coverage verified; require target algorithm tests and end-to-end Android input/frame-pacing measurements.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3430. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaGameplayFactsExporter.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 80dff0ad2a684296dae991f8130c8c1dd8017958
- Size: 5891 bytes
- Coverage: Complete EOF coverage (112 logical lines).
- Purpose: Export per-blockstate gameplay and selection facts including light, hardness, tool requirement, replacement, collision/outline shapes, map color, friction and sounds.
- Dependencies / cross-references: BlockState registry, EmptyBlockGetter, BlockPos, VoxelShape, SoundType and vanilla-generated item-component reports.
- Implementation / logic: Iterates the complete blockstate registry, queries native accessors at a fixed empty-world position, serializes outline AABBs/edges and other physics/gameplay fields, and streams JSON to a sibling partial file before moving it to the target.
- Findings: Exporter comments estimate a large output (about 56 MB); the fixed EmptyBlockGetter/BlockPos cannot express context-dependent world behavior; `Files.move(REPLACE_EXISTING)` is not explicitly ATOMIC_MOVE; item component reports are external, not included here.
- RustChunk relevance: Useful broad source of block metadata for collision, mining, lighting, sound and culling data.
- Decision: adaptable; reference; parity-data-exporter
- 1.20.1 implications: State IDs/properties and many accessors differ by target version; contextual hardness/shape and Forge hooks require 1.20.1-specific regeneration.
- Follow-up checks: Compare every exported field against 1.20.1, test context-dependent states, outline offset positions, shape serialization, atomic completion and size/streaming behavior.
- RustChunk implementation mapping: Build a target-version exporter or generate static parity datasets; do not embed its later-version output directly.
- Evidence required before adoption: Pinned SHA/size and 112-line EOF coverage verified; require target registry regeneration, output schema validation and contextual block tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3431. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaGenerationTickOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 4d956f5f069d66a832acee8854ce8dbd6b6970d0
- Size: 4130 bytes
- Coverage: Complete EOF coverage (48 logical lines).
- Purpose: Exercise WorldGenTickAccess routing and ProtoChunkTicks/LevelChunkTicks persistence, promotion, snapshots, queries and counts.
- Dependencies / cross-references: WorldGenTickAccess, ProtoChunkTicks, LevelChunkTicks, SavedTick/ScheduledTick, ChunkPos and TickPriority.
- Implementation / logic: Parses a command stream, maps chunk positions to tick containers, executes load/level/schedule/query/will/dump/snapshot/unpack/promote/remove/reset and injects lookup failures; serializes saved tick type/position/delay/priority and aggregate count.
- Findings: Uses synthetic object identities for 32 tick types, simple space splitting and unchecked indexes; failure flag rejects all future lookups until reset; only storage/routing is modeled, not server tick execution.
- RustChunk relevance: High relevance to chunk serialization, pending block/fluid ticks and chunk promotion during world load.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 tick containers and WorldGenTickAccess differ; target scheduled tick ordering, deduplication and promotion semantics must be revalidated.
- Follow-up checks: Test same-tick duplicates, priority/time ordering, negative positions/chunk packing, absent chunks, failure and save/load/promotion around clock boundaries.
- RustChunk implementation mapping: Use as a target-version test harness design for per-chunk tick queues.
- Evidence required before adoption: Pinned SHA/size and 48-line EOF coverage verified; require exact 1.20.1 scheduled tick ordering and persistence tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3432. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaGeometryOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 4241b656b3e7b1d7e3035ecb47a98a63702fd350
- Size: 18635 bytes
- Coverage: Complete EOF coverage (218 logical lines).
- Purpose: Provide a broad bit-exact oracle for native VoxelShape, AABB, Vec3, BlockPos traversal, collision/step-up, rotation and trigonometric operations.
- Dependencies / cross-references: VoxelShape/ArrayVoxelShape, Shapes/DiscreteVoxelShape, AABB/Vec3, BlockGetter traversal, Entity private collision methods, Mth sine table, ServerLevel/ArmorStand shells, Unsafe and reflection.
- Implementation / logic: Command-driven adapter constructs shapes and entities, invokes real vanilla geometry methods, captures raw floating-point bits, AABB collections, traversal order, hit results, step-up candidates, shape joins/optimization and vector math.
- Findings: Private method names/signatures and Unsafe entity shells are version brittle; step-motion uses scripted block/entity collision shape lists; input counts/dimensions are trusted; large trig/position outputs can be extensive.
- RustChunk relevance: Extremely relevant to chunk collision, block selection, movement, voxel culling and shape compatibility.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: MC26.3 shape and Entity collision code may diverge from 1.20.1; Android floating-point/compiler behavior must be verified separately.
- Follow-up checks: Build a 1.20.1 comparison matrix for negative/degenerate boxes, grid coordinate ordering, boolean joins, traversal boundaries, step-up ties, axis collision order and NaNs.
- RustChunk implementation mapping: Use as core target-version geometry test specification; implement Rust shape/collision primitives independently.
- Evidence required before adoption: Pinned SHA/size and 218-line EOF coverage verified; require exact target-version bitwise outputs and device-side movement/collision integration tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3433. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaGhostRecipeOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: bb0d74fcbfabacb51530e546c79f8c75c4ad9ec1
- Size: 8072 bytes
- Coverage: Complete EOF coverage (130 logical lines).
- Purpose: Run recipe-book ghost-slot population for crafting and furnace displays, including result/input slots and candidate stacks.
- Dependencies / cross-references: RecipeManager/RecipeDisplayEntry, CraftingRecipeBookComponent/FurnaceRecipeBookComponent.fillGhostRecipe, GhostSlots, ContextMap, VanillaMenuOracle and ParityData.
- Implementation / logic: Loads recipe definitions, creates fake inventory/menu/player, reflectively invokes each matching component's private fillGhostRecipe and serializes slot index/result flag and ghost candidate stacks for each recipe display.
- Findings: Relies on shared VanillaMenuOracle and ParityData initialization, Unsafe menu shells and reflected method/inner-slot access; catches Throwable per recipe; uses new RecipeDisplay system rather than actual menu click lifecycle.
- RustChunk relevance: Useful for recipe-book ghost UI and crafting/furnace slot guidance.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern RecipeDisplay/SlotDisplay APIs differ substantially from 1.20.1 recipe-book classes; target GUI flow must be rebuilt around RecipeHolder/recipe-book menu.
- Follow-up checks: Test shaped/shapeless fit, 2x2 vs 3x3, furnace fuel choices, empty fuel slot, multiple displays, missing ingredients and slot index mapping in 1.20.1.
- RustChunk implementation mapping: Translate expected ghost-slot sequences to RustChunk recipe-book UI tests.
- Evidence required before adoption: Pinned SHA/size and 130-line EOF coverage verified; require 1.20.1 recipe menu/recipe display comparison and actual GUI integration.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3434. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaGroundStateOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: b5a0335613bcce5330cb92264957961c5afedc9b
- Size: 4556 bytes
- Coverage: Complete EOF coverage (39 logical lines).
- Purpose: Trace Entity.checkSupportingBlock and setOnGroundWithMovement support-ground transitions against controlled support-block results.
- Dependencies / cross-references: Entity private onGround/onGroundNoBlocks/mainSupportingBlockPos fields, ServerLevel.findSupportingBlock, Marker/Actor and AABB.
- Implementation / logic: Allocates a test entity and fake level, supplies successive support BlockPos answers or failure, invokes private checkSupportingBlock or public ground-state overloads and records ground/horizontal/empty flags and main support position.
- Findings: Support detection itself is scripted; fake level does not perform voxel collision search; direct reflection into private flags bypasses regular movement lifecycle; support result list must be long enough for query count.
- RustChunk relevance: Relevant to grounded movement, edge support and stable walking over chunk/block boundaries.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: The 1.20.1 supporting-block algorithm and fields must be compared; no inference about actual collision-world lookup can be made from this test.
- Follow-up checks: Test support absent/present, consecutive changes, movement null/non-null, horizontal collision, callback failure and support position invalidation against target.
- RustChunk implementation mapping: Use for ground-state state-machine tests alongside target collision-world queries.
- Evidence required before adoption: Pinned SHA/size and 39-line EOF coverage verified; require full target support search and movement integration tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3435. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaHeightProviderOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: b3fb18aff99b387acab83bc9cc71112483bb6e36
- Size: 2162 bytes
- Coverage: Complete EOF coverage (32 logical lines).
- Purpose: Decode native HeightProvider definitions and sample deterministic heights under legacy and Xoroshiro random streams.
- Dependencies / cross-references: HeightProvider.CODEC, WorldGenerationContext, RandomSource/LegacyRandomSource/XoroshiroRandomSource and reflective VanillaAnchorOracle.context.
- Implementation / logic: Constructs world-generation context from fixture, parses a height provider JSON definition, samples it N times and emits the provider result plus next RNG values after every sample.
- Findings: Context helper is reflected from another oracle; catches only IllegalArgumentException on parsing/sampling; malformed count or definitions may abort; only supported context fields from helper are covered.
- RustChunk relevance: High relevance to biome/feature height selection and deterministic worldgen.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: HeightProvider codec schemas and random source consumption must be matched specifically to Forge 1.20.1.
- Follow-up checks: Compare every provider type, min/max build heights, uniform/trapezoid/biased-to-bottom distributions, invalid ranges and random draw counts.
- RustChunk implementation mapping: Use as a 1.20.1 height-provider golden-test driver.
- Evidence required before adoption: Pinned SHA/size and 32-line EOF coverage verified; require target provider codec and seeded output comparisons.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3436. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaHeightmapOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 6aab393dbba46a9052199fd4bbbd0a914d0a7a0d
- Size: 8717 bytes
- Coverage: Complete EOF coverage (145 logical lines).
- Purpose: Export block-state/tag facts and exercise all native Heightmap.Types through ProtoChunk writes, updates, priming, raw data and queries.
- Dependencies / cross-references: Heightmap, ProtoChunk/PalettedContainerFactory, block registry/state codec, BlockTag TagLoader, biome holders and BlockState IDs.
- Implementation / logic: Builds an in-memory ProtoChunk with requested vertical bounds, writes states to palette sections, primes/upgrades heightmaps, mutates raw packed data, queries highest/first-available heights and dumps long words; binds supplied block tags and exports state opacity predicates.
- Findings: Single chunk only; section writes assume aligned valid Y/state IDs; `bindTags` mutates global BLOCK registry tag state and is not restored; no section NBT/biome codec or disk save/load is exercised.
- RustChunk relevance: Very high relevance to chunk column metadata, surface detection, spawn placement and chunk persistence.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Heightmap types, tag names and height-section layout are version-specific; compare 1.20.1's heightmap update and palette packing precisely.
- Follow-up checks: Test negative minY, variable build height, all heightmap predicates, stale map updates, raw data length/bit packing, cross-section writes and tag reload isolation.
- RustChunk implementation mapping: Use as an acceptance oracle for RustChunk's heightmap/column-index implementation.
- Evidence required before adoption: Pinned SHA/size and 145-line EOF coverage verified; require 1.20.1 raw-word comparisons and persisted chunk round trips.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3437. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaHeldDropOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: b2cdb3383e63c21918652eb94d52d197adc70bf0
- Size: 6989 bytes
- Coverage: Complete EOF coverage (142 logical lines).
- Purpose: Run the actual client handleKeybinds drop path and MultiPlayerGameMode.dropItem for single/all drops across game modes and inventory edge cases.
- Dependencies / cross-references: Minecraft.handleKeybinds, MultiPlayerGameMode.dropItem, LocalPlayer/Inventory, KeyMapping, ServerboundPlayerActionPacket and Connection.
- Implementation / logic: Builds fake client/actor/controller/connection, invokes the real key handler for two presses while varying GameType, control key, hotbar slot and stack count, and records local count loss, action packets and swing count.
- Findings: Resets AIR/STONE component prototypes globally; uses Unsafe and private fields; deliberately aborts at first-person item-use boundary; no actual network send/server inventory reconciliation.
- RustChunk relevance: Useful for drop-key semantics and inventory mutation tests.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern inventory/packet/control flow may differ in 1.20.1; server acknowledgement and multiplayer synchronization are not modeled.
- Follow-up checks: Check empty stack, single vs all, spectator/creative/survival, selected slots, repeated presses, swing packet and serverbound action packet semantics on target.
- RustChunk implementation mapping: Use the sequence matrix to test RustChunk drop controls and target packet generation.
- Evidence required before adoption: Pinned SHA/size and 142-line EOF coverage verified; require 1.20.1 packet comparison and end-to-end multiplayer drop tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3438. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaHeldItemStackOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: ebcbc235d24d089721700771631dd71fc889726e
- Size: 3470 bytes
- Coverage: Complete EOF coverage (67 logical lines).
- Purpose: Capture which main/offhand ItemStacks the actual armed-entity render-state extraction sends to the item-model resolver for left/right-handed players.
- Dependencies / cross-references: ArmedEntityRenderState.extractArmedEntityRenderState, LocalPlayer, ItemModelResolver, ItemStackRenderState and item registry.
- Implementation / logic: Provides a custom player shell and resolver that records item IDs passed to updateForLiving, then runs native armed-entity extraction for configured hand/arm combinations.
- Findings: Before tests it binds every item registry prototype to empty component maps; output records item choice only, not transforms, swing pose, arm geometry, model baking or GPU draw; global registry mutation is not restored.
- RustChunk relevance: Useful for first-person held-item selection/order, which can affect visual hand correctness.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern render-state extraction is not the same as 1.20.1 ItemInHandRenderer; reconstruct target behavior and account for Forge item renderer hooks.
- Follow-up checks: Test all empty/nonempty main/offhand combinations, left/right main arm, item animation states and registry component prototype isolation.
- RustChunk implementation mapping: Use as a held-stack selection test case source, not a renderer implementation.
- Evidence required before adoption: Pinned SHA/size and 67-line EOF coverage verified; require 1.20.1 extraction mapping and rendered hand integration tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3439. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaHotbarOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 99233ab77f609e3474a5fe50dfd094e9f2efdd4c
- Size: 2375 bytes
- Coverage: Complete EOF coverage (50 logical lines).
- Purpose: Load creative hotbar NBT through HotbarManager and the game's DataFixer, then enumerate all toolbar groups and canonicalized stacks.
- Dependencies / cross-references: HotbarManager/Hotbar, NbtIo/TagParser, DataFixers, DataFixTypes.HOTBAR and ParityData item/registry lookup.
- Implementation / logic: Writes each input SNBT as hotbar.nbt in a temp directory, constructs HotbarManager with the vanilla DataFixer, loads each group and serializes canonical item stacks.
- Findings: Uses modern hotbar storage format and item stack fixes; temp directory is not deleted; catches Throwable and optional trace prints stack traces; does not test UI slot selection.
- RustChunk relevance: Relevant to saved creative toolbar compatibility and data migration.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Hotbar file layout and DataFixer version must match 1.20.1; do not reuse later-version migrated NBT as target source data.
- Follow-up checks: Test missing file, malformed SNBT, all nine groups, legacy DataVersion, item/component conversion and empty stacks using target DataFixer.
- RustChunk implementation mapping: Use as target-specific hotbar persistence migration tests.
- Evidence required before adoption: Pinned SHA/size and 50-line EOF coverage verified; require 1.20.1 DataFixer round trips and temp-file lifecycle checks.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3440. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaHudOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: f44a0efd6fea14257f76c48cdf08acb01e26c1ec
- Size: 42838 bytes
- Coverage: Complete EOF coverage (630 logical lines).
- Purpose: Capture native HUD extraction and draw calls for hotbar, health, crosshair, effects, titles, overlays, boss bars, chat, sidebar, jump/experience bars and item decorations.
- Dependencies / cross-references: Hud, BossHealthOverlay, ChatComponent, GuiGraphicsExtractor, DeltaTracker, Font/ParityFont, ItemStack renderer, Scoreboard and large Unsafe/reflection-built client/actor shells.
- Implementation / logic: Uses typed case JSON to populate HUD-facing player/game state, invokes real Hud methods and overlays, overrides GuiGraphicsExtractor to capture sprites/blits/fills/text/items, and serializes per-draw coordinates, colors, transforms, pipeline and text runs.
- Findings: 630 logical lines, read in consecutive 1–400 and 401–630 ranges. Uses unseeded HUD RandomSource, random boss UUIDs and Util wall-clock timestamps; static Minecraft.instance is changed in chat path without restoration; styles capture only color/bold/italic; item decorations can optionally delegate to untested super renderer; output accumulates all cases in memory; not pixel/GPU capture.
- RustChunk relevance: Extremely relevant to RustChunk HUD parity and identifying GUI draw order/layout differences, but it is a command/draw-call oracle rather than an image comparison.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: The HUD, font, overlay, scoreboard and graphics extraction classes are MC26.3-specific and not drop-in for 1.20.1/Android; target Forge overlays and GUI scaling need separate mapping.
- Follow-up checks: Seed random sources, virtualize Util time, test static singleton restoration, text style coverage, alpha/opacity, boss/chat state, output memory bounds and target draw order/coordinates across sizes.
- RustChunk implementation mapping: Use cases to specify native Rust HUD widget states and compare emitted Rust UI primitives with target-version Java oracle outputs.
- Evidence required before adoption: Pinned SHA/size and 630-line EOF coverage verified; complete ranges 1–400 and 401–630 recorded; require deterministic cases, 1.20.1 mapping and Android screenshot/render validation.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3441. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaInlineObjectOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 02bf729e9989d5d81035887c0b11fdcc14bbd81d
- Size: 4996 bytes
- Coverage: Complete EOF coverage (91 logical lines).
- Purpose: Inspect font atlas glyph color, shadow, style and quad vertex generation for the inline object replacement character.
- Dependencies / cross-references: Font.prepareText, GlyphSource/BakedGlyph, AtlasGlyphProvider.Instance, EffectGlyph, VertexConsumer, TextureAtlasSprite and ParityData.
- Implementation / logic: Builds a synthetic font provider and atlas sprite UV region, prepares a single formatted U+FFFC glyph with custom style/color/shadow, renders through a VertexConsumer proxy and records vertex positions/colors.
- Findings: Uses Unsafe/reflection to construct atlas internals; recorder ignores most VertexConsumer methods and assumes color/vertex callback ordering; no texture upload or GPU rasterization occurs.
- RustChunk relevance: Useful for glyph/icon replacement and custom inline object typography.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: The font glyph/provider/render pipeline differs in 1.20.1; inline object support should be rechecked against target TextRenderer/GlyphRenderTypes.
- Follow-up checks: Test color/shadow precedence, bidi/style runs, UV bounds, vertex order, GUI vs world flag and actual rendered pixels on device.
- RustChunk implementation mapping: Use as a test-only glyph geometry oracle for RustChunk custom text/icon rendering.
- Evidence required before adoption: Pinned SHA/size and 91-line EOF coverage verified; require target glyph source comparison and GPU screenshot validation.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3442. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaInteractionInputOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: c18f0133afc7e8f3ea372763d27649b02623c8d5
- Size: 10106 bytes
- Coverage: Complete EOF coverage (173 logical lines).
- Purpose: Run Minecraft.handleKeybinds input dispatch for attack/use/pick actions with scripted key clicks, held states and screen-open release.
- Dependencies / cross-references: Minecraft.handleKeybinds, KeyMapping, LocalPlayer, MultiPlayerGameMode, MouseHandler, ClientLevel and packet connection stubs.
- Implementation / logic: Constructs a synthetic client/player/level/controller, scripts attack/use/pick click counters and held buttons/right-click delay, then invokes the real key handler and records calls, using/destroying flags and resulting delay.
- Findings: GameMode effects and UI are scripted stubs; block is always STONE, entity interaction range always true, packets are discarded and swing always false; only handleKeybinds segment is covered.
- RustChunk relevance: High relevance to mapping touch/controller input to the vanilla action loop.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern handleKeybinds behavior, input key bindings and screen lifecycle must be compared to 1.20.1; Android gestures are not directly validated.
- Follow-up checks: Test click order, attack/use simultaneous inputs, PASS/fallback paths, pick block/entity, destroy hold/release, delay countdown, screen close and item-in-use behavior.
- RustChunk implementation mapping: Derive RustChunk input dispatch state-machine tests; connect Android touch events separately.
- Evidence required before adoption: Pinned SHA/size and 173-line EOF coverage verified; require 1.20.1 callback-order comparison and Android input integration.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3443. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaInteractionShapeOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 0c2520ccb59ac2931f28ed7706beb5a482fcdec8
- Size: 5541 bytes
- Coverage: Complete EOF coverage (35 logical lines).
- Purpose: Export registered block interaction shapes and exercise native clipWithInteractionOverride ray intersection for caller-supplied shapes.
- Dependencies / cross-references: BlockState.getInteractionShape, BlockGetter.clipWithInteractionOverride, Block/State registry, VoxelShape, NBT and VanillaCollisionOracle.grid.
- Implementation / logic: Verifies only an allowlist of block classes override getInteractionShape; exports every block state plus identity-deduplicated interaction shapes and state-to-shape mapping; runs ray clips against supplied positions/shapes.
- Findings: The allowlist hard-fails when a new override owner appears; static world proxy throws if interaction shape queries world; NBT IDs and shape dedup are version-dependent; no dynamic block entity/world collision path.
- RustChunk relevance: Useful for mouse block targeting and interaction raycasting accuracy.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Interaction shape override classes and ray clipping semantics must be audited against 1.20.1/Forge; mods can add overrides and custom shapes.
- Follow-up checks: Check all override owners, custom mod blocks, inside-hit semantics, face/nearest block, shape identity, world-query exceptions and negative coordinates.
- RustChunk implementation mapping: Use as target-version interaction-ray oracle for RustChunk touch hit tests.
- Evidence required before adoption: Pinned SHA/size and 35-line EOF coverage verified; require full Forge block-owner inventory and Android picking tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3444. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaInventoryPickupOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 7ed3e8845da51795901edbcc2d60dd0571a68557
- Size: 10217 bytes
- Coverage: Complete EOF coverage (42 logical lines).
- Purpose: Exercise native Inventory storage/pickup/removal/equipment and alias semantics with mutable stacks and DataComponentMap patches.
- Dependencies / cross-references: Inventory, EntityEquipment, ItemStack/PatchedDataComponentMap, component prototypes, EquipmentSlot mapping, Player infinite-materials and Unsafe/reflection.
- Implementation / logic: Builds two inventories and shared/separate equipment configurations, creates aliased stack references, runs many inventory operations and snapshots slots/patches/equipment before and after event-triggered mutation/failure.
- Findings: Global Inventory.EQUIPMENT_SLOT_MAPPING is cleared/rebuilt; AIR/STICK/SHIELD prototype components and ItemStack.EMPTY are changed; many private fields/maps/constructors and untyped casts are used; dense harness catches only selected exceptions; global state not restored.
- RustChunk relevance: Very high relevance to launcher item inventory correctness, stacking, selected slot and equipment mapping.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern EntityEquipment and component/slot layout differ from 1.20.1 Inventory; Forge mods may add equipment slots or stack-size components.
- Follow-up checks: Run isolated JVM; verify mapping restore, stack merge limits, duplicate aliases, EMPTY behavior, pick/add/remove return semantics, equipment slot order, max-size mutation and inventory replacement.
- RustChunk implementation mapping: Convert scenarios into target-version inventory acceptance tests; write Rust inventory structures independently.
- Evidence required before adoption: Pinned SHA/size and 42-line EOF coverage verified; require state-leak checks and complete 1.20.1 inventory operation comparison.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3445. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaInventoryTickOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 8ad5d1f6710acc0735a326cb02c4493912c8b62c
- Size: 6034 bytes
- Coverage: Complete EOF coverage (28 logical lines).
- Purpose: Exercise inventory and equipment ticking with actual ItemStack.inventoryTick callbacks under server/client level selection.
- Dependencies / cross-references: Inventory.tick, EntityEquipment.tick, ItemStack.inventoryTick, Item/ServerLevel/ClientLevel, Holder.direct and Unsafe/reflected ItemStack constructor.
- Implementation / logic: Creates scripted TickItem subclasses that record the slot, owner, level and call order, populates inventory/equipment aliases and executes inventory/equipment/both/stack modes with event-triggered state mutation/failure.
- Findings: The tick item is synthetic and asserts one expected server level; ClientLevel allocation may be unavailable in dedicated JAR; ItemStack.EMPTY is mutated globally; private constructor/fields are relied upon; no real game entity/tick world is run.
- RustChunk relevance: Useful for inventory tick ordering, side gating and per-equipment slot behavior.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 tick methods and client/server level branch must be verified; custom mod items may rely on side-sensitive callbacks.
- Follow-up checks: Test player/non-player owners, server/client/null level, main/offhand/armor/body/saddle slots, stack aliases, mutation mid-iteration and callback order on target.
- RustChunk implementation mapping: Use as target inventory-tick regression harness design.
- Evidence required before adoption: Pinned SHA/size and 28-line EOF coverage verified; require dedicated/client runtime comparison and modded item integration tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3446. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaInventoryUseOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 45f3d1191dcf2378e0c704c7da3907b3aa31f6e6
- Size: 8555 bytes
- Coverage: Complete EOF coverage (40 logical lines).
- Purpose: Exercise LivingEntity item-use lifecycle including onUseTick, releaseUsing, finishUsingItem, synchronized state, vibration and equipment notifications.
- Dependencies / cross-references: LivingEntity, Player/Inventory/PlayerEquipment, ItemStack components, SynchedEntityData, ServerLevel, GameEvent, Unsafe/reflection and custom Items.
- Implementation / logic: Builds custom items and server shells with mutable client-side state, invokes private updatingUsingItem or public update/complete/release/stop paths and records flags, use-item refs, remaining ticks, callbacks, inventory/equipment and dirty synchronized data.
- Findings: Hard-coded living-entity flag data ID 8 and private field names are fragile; fake levels/items and SynchedEntityData built manually; Items.AIR prototype and ItemStack.EMPTY are mutated; no real packet transport.
- RustChunk relevance: High relevance to eating/drinking/blocking/charging item lifecycle and synchronized animation state.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Use lifecycle and component hooks differ in 1.20.1; map behavior to `LivingEntity` use fields and Forge item callbacks from target source.
- Follow-up checks: Test client vs server branches, zero/negative remaining ticks, item identity changes, release query, finish replacement, mutation/failure, vibrations and dirty packet packing.
- RustChunk implementation mapping: Derive RustChunk item-use state transitions and network-sync test vectors, not production Java lifecycle code.
- Evidence required before adoption: Pinned SHA/size and 40-line EOF coverage verified; require 1.20.1 method/callback sequence comparisons and server/client tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3447. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaInventoryUseStartOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 8a8478e6974e41ab8d3d2731df6a06612600bdf6
- Size: 10838 bytes
- Coverage: Complete EOF coverage (42 logical lines).
- Purpose: Exercise the start/use flag path including item duration, initial hand selection, synced flag updates, equipment packets and ITEM_INTERACT_START events.
- Dependencies / cross-references: LivingEntity.startUsingItem/updatingUsingItem, Item.getUseDuration, PlayerEquipment, SynchedEntityData, ServerChunkCache tracking packets, UseEffects/KineticWeapon components, Unsafe/reflection.
- Implementation / logic: Extends the item-use harness with scripted use duration, client/server checks, ServerChunkCache equipment packet capture, item flag state, tracked entity ID, synced-data notifications and item-start vibration event.
- Findings: Hard-codes entity data accessor ID 8 and internal field/constructor shapes; synthetic ServerChunkCache and custom items only record packets/callbacks; Uses modern UseEffects/KineticWeapon; no actual connected client receives packets.
- RustChunk relevance: Useful for item start-use animation and server tracking/equipment sync.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern item component and packet update rules are later-version-specific; 1.20.1 start-use and equipment sync need source-level porting.
- Follow-up checks: Verify duration=0/negative, main/offhand, use effects, kinetic item flags, client-side no-packet vs server packet, tracking recipients, entity ID and failure order.
- RustChunk implementation mapping: Use its state/event vectors in target item-use network and animation tests.
- Evidence required before adoption: Pinned SHA/size and 42-line EOF coverage verified; require 1.20.1 network packet and item-start parity tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3448. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemAfterUseOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: a55f7172e0627629678898aefdb478408883824b
- Size: 7302 bytes
- Coverage: Complete EOF coverage (29 logical lines).
- Purpose: Exercise ItemStack.finishUsingItem composition with UseRemainder templates, UseCooldown groups, ItemCooldowns, actor infinite materials and callback replacement.
- Dependencies / cross-references: ItemStack.finishUsingItem, UseRemainder/ItemStackTemplate, UseCooldown, ItemCooldowns, Custom Item, Player/ArmorStand and DataComponentMap.
- Implementation / logic: Creates custom/vanilla stacks with remainder/cooldown components, calls finishUsingItem and records extra item creation, original stack identity, cooldown map/timing and callback order under injection.
- Findings: Modern components and reflection into ItemCooldowns fields; static AIR/STICK component prototypes mutated; fake actors do not model real inventory insertion/drop and the remainder output only records the result stack.
- RustChunk relevance: High relevance to item consumption, bowl/bottle/remainder and cooldown behavior.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: UseRemainder and UseCooldown modern APIs are not present in the same form in 1.20.1; derive equivalent logic from 1.20.1 ItemStack and Item use pipeline.
- Follow-up checks: Test no remainder, max stack override, damage-zero template, grouped cooldown duration, infinite materials, returned stack alias, callback failure/mutation and player vs non-player.
- RustChunk implementation mapping: Map the cases to target item consumption/remainder and cooldown tests.
- Evidence required before adoption: Pinned SHA/size and 29-line EOF coverage verified; require target-version finishUsingItem and inventory-return tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3449. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemBlockingOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 1dede3a19f1df8da3d43cf56b530ccc92292803d
- Size: 7909 bytes
- Coverage: Complete EOF coverage (48 logical lines).
- Purpose: Run LivingEntity.applyItemBlocking with BlocksAttacks component reductions, bypass tags, projectile pierce and attacker/source metadata.
- Dependencies / cross-references: BlocksAttacks, DamageSource/DamageType tags, ItemStack, LivingEntity/ServerPlayer/ArmorStand, Arrow, ServerLevel, Stats and Unsafe.
- Implementation / logic: Builds synthetic damage types and tag sets, configurable blocking items/components, damage source, attacker/direct/projectile entities and level, then invokes native applyItemBlocking and records blocked ratio, events, stats and stack damage.
- Findings: Uses later-version BlocksAttacks data component and custom damage types; fake actors and levels script most getters; does not apply actual final hurt/damage or simulate full combat/network flow.
- RustChunk relevance: Reference for shield/blocking angle and damage reduction algorithm.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 shield blocking behavior is implemented differently (item tags and shield logic); must compare exact 1.20.1 `isDamageSourceBlocked`/shield durability paths.
- Follow-up checks: Test projectile vs melee, pierce levels, angle boundaries, bypass tags, damage thresholds/reduction factors, item damage and stat callbacks in 1.20.1.
- RustChunk implementation mapping: Use only as a guide to construct target shield/blocking regression cases.
- Evidence required before adoption: Pinned SHA/size and 48-line EOF coverage verified; require target shield source comparison and real combat tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3450. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemCooldownsOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 68600af2f583eb316ffd208dbf3a7984894cdf40
- Size: 5464 bytes
- Coverage: Complete EOF coverage (37 logical lines).
- Purpose: Exercise ItemCooldowns storage, expiry ticks, group resolution, cooldown percentages/durations and UseCooldown application.
- Dependencies / cross-references: ItemCooldowns, UseCooldown, DataComponents, ItemStack, Actor.getCooldowns and Identifier-keyed cooldown map.
- Implementation / logic: Builds an isolated cooldown book, applies group/stack cooldowns, mutates clock/map/stack inside callbacks, calls tick/percent/duration/apply and captures callbacks and map boundaries.
- Findings: Uses private cooldown map and inner timing fields; callback mutation deliberately can throw ConcurrentModificationException; uses modern component grouping; no wall-clock or UI rendering is involved.
- RustChunk relevance: Useful for item cooldown timing and hotbar/UI lockout behavior.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 cooldown map/group behavior and item grouping API must be verified; modern UseCooldown component is not portable.
- Follow-up checks: Test exact expiry endpoints, partial-tick percent, zero/negative duration, grouping, remove during callback, mutation and multiple stack items on target.
- RustChunk implementation mapping: Translate timing cases to RustChunk's item cooldown service and target HUD overlay tests.
- Evidence required before adoption: Pinned SHA/size and 37-line EOF coverage verified; require 1.20.1 ItemCooldowns output comparisons and end-to-end UI cooldown display tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3451. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemDurabilityOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 0c2a98eaa338aa5f42e5c964ada39d4b0781bd11
- Size: 4520 bytes
- Coverage: Complete EOF coverage (23 logical lines).
- Purpose: Exercise native ItemStack durability mutations, break callbacks, creative-infinite handling and callback-triggered state mutation/failure.
- Dependencies / cross-references: ItemStack.hurtAndBreak/hurtWithoutBreaking/setDamageValue, DataComponents.MAX_DAMAGE/DAMAGE/UNBREAKABLE, ServerPlayer/PlayerAdvancements; Unsafe and private ItemStack fields.
- Implementation / logic: Creates a real ItemStack with controlled component patch and scripted ServerPlayer shell, then drives hurt, set, grow/shrink and query modes while recording snapshot state on each callback.
- Findings: Uses modern component data and private ItemStack constructor/fields; uses ItemStack stack seeded from AIR/SHIELD; callback event order includes virtual advancement and level access; tests do not cover enchantment-specific durability, damage predicates or all item subclasses.
- RustChunk relevance: High relevance to tool durability, damage bar, break/consume and item mutation correctness.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 uses component-based item stacks but its exact durability and break callback signatures differ; map behavior from target vanilla/Forge source.
- Follow-up checks: Compare negative/over-max damage, unbreakable, creative, count>1, player null/non-null, break callback mutation/failure and advancement side effects.
- RustChunk implementation mapping: Use the fixture matrix as target ItemStack durability regression cases; implement actual stack semantics natively.
- Evidence required before adoption: Pinned SHA/size and EOF 23 lines verified; require target 1.20.1 ItemStack results and item-specific durability integration.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3452. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemModelOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: cbbcd5e055a3cfab2173671550972afe0f502705
- Size: 31309 bytes
- Coverage: Complete EOF coverage (464 logical lines).
- Purpose: Decode all item definitions, discover model roots, bake native item models, resolve ItemModel variants/properties and capture complete ItemStackRenderState layers/quad data.
- Dependencies / cross-references: ClientItem.CODEC, ModelDiscovery/ModelBakery, SpecialModelRenderers, ItemModels, SpriteSources/atlases, NativeImage resources, ItemModelResolver and modern DataComponents.
- Implementation / logic: Loads vanilla resources from an extracted JAR, prepares registries/tags/default item component prototypes, decodes item model files and synthetic definitions, runs discovery/baking with real sprite images and captures per-case render layers, transformations, quads, UVs, tints, foil, special-model config, dimensions and swap animation data.
- Findings: 464 logical lines read in contiguous ranges 1–400 and 401–464. Mutates every registered item holder's default component map; global sprite/model caches; special renderer submit is intentionally a no-op; fake sprite getter supplies a dummy sprite for baking context; no texture atlas upload, GPU submission or pixel rendering; output writer closed only at normal completion; resources/image disposal and failure cleanup deserve review.
- RustChunk relevance: Extremely high relevance to native item rendering/model compatibility, layered item models, special items and item-state visual data.
- Decision: adaptable; reference; parity-test-oracle; renderer-reference
- 1.20.1 implications: ClientItem item-definition JSON and item model architecture are later-version systems, not direct 1.20.1 item model JSON; port conceptual pipeline only after target resource formats and Forge model hooks are mapped.
- Follow-up checks: Test missing/invalid assets, generated/paletted trim sprites, texture lifecycle, model discovery recursion, fallback/missing models, item owner/dimension properties, bundles, special renderer arguments, model transforms and actual GPU raster output on 1.20.1.
- RustChunk implementation mapping: Use as a detailed renderer test specification and resource-pipeline reference; keep Rust asset parser/model graph/baker independent and target-version aware.
- Evidence required before adoption: Pinned SHA/size and 464 logical lines verified; complete coverage 1–400 and 401–464; require 1.20.1 asset comparison, deterministic owner cases, native mesh baking and Android GPU image tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3453. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemParticlesOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: a6551eb5609479d3bc582339591d34c431dbd0af
- Size: 5290 bytes
- Coverage: Complete EOF coverage (34 logical lines).
- Purpose: Exercise LivingEntity.spawnItemParticles with controlled item stack/count/damage, positions, pitch/yaw, random source, RNG stream and particle sink.
- Dependencies / cross-references: LivingEntity.spawnItemParticles, ItemParticleOption/ParticleTypes.ITEM, Level.addParticle, RandomSource/LegacyRandomSource or XoroshiroRandomSource, ItemStack and Unsafe fields.
- Implementation / logic: Builds an entity shell and two fake server levels, injects position and random generator, invokes native particle emission, captures particle stack/coords/velocity bit patterns and post-call random values.
- Findings: Mutates AIR/STONE/BREAD global item prototype component maps; only three item identities and two seeded random sources are represented; `getRandom` is deliberately forbidden in one harness path; no actual particle engine, renderer or GPU is invoked.
- RustChunk relevance: Useful for dropped-item/hand effects and deterministic item particle parity.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: The 1.20.1 particle emission code and random source algorithm must be checked; particle engine throughput/performance remains unmeasured.
- Follow-up checks: Compare particle count, damage component propagation, spawn center, random velocities, random draw order, zero/negative count and client-side particle handling with 1.20.1.
- RustChunk implementation mapping: Use the expected event traces in RustChunk's target particle tests, not as a production particle engine.
- Evidence required before adoption: Pinned SHA/size and EOF 34 lines verified; require target-version seeded output comparisons and real mobile particle rendering/performance tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3454. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemStackStorageOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 34c5bfa86358ab40b7a055f811c154a1c743c12e
- Size: 6634 bytes
- Coverage: Complete EOF coverage (28 logical lines).
- Purpose: Exercise ItemStack storage primitives: prototypes, patch maps, stack identity/equality, count/pop time, copy, split, transmute, consume and grow/shrink/limit.
- Dependencies / cross-references: ItemStack, DataComponentMap/DataComponentPatch/PatchedDataComponentMap, DataComponentType, ItemStack.EMPTY, Inventory actor infinite-materials callback and Unsafe/reflection.
- Implementation / logic: Creates synthetic component types and controlled AIR/STICK/SHIELD prototypes, constructs three stack slots including canonical EMPTY, applies patch operations and logs full state, visible component values, patch additions/removals and identity/equality after each operation.
- Findings: Resets static ItemStack.EMPTY count/pop/patch and rebinds global built-in item component prototypes for every fixture; changes global JVM state without restoring; `copyAndClear`, consume and split semantics are modern component-map implementation; catches only selected exceptions.
- RustChunk relevance: Very high relevance to RustChunk item storage, inventory correctness, alias safety and serialization.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 ItemStack internals/components patch API differ by minor release; target Forge item components and canonical EMPTY handling must be verified.
- Follow-up checks: Isolate per process; test global state leakage, two slots sharing object, empty canonical identity, prototype changes, removed patch values, copy/split/transmute, max count, negative grow/shrink and infinite materials.
- RustChunk implementation mapping: Use as acceptance-test cases for native Rust ItemStack/Inventory structures; do not port Unsafe or internal Java representation.
- Evidence required before adoption: Pinned SHA/size and EOF 28 lines verified; require 1.20.1 golden state snapshots and alias/empty invariant tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3455. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemTemplateOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: fa3ad6a9a11361211635bc296afa1a1bb5c1eac6
- Size: 5291 bytes
- Coverage: Complete EOF coverage (39 logical lines).
- Purpose: Exercise ItemStackTemplate construction/from-stack/with-count/create/apply/validate operations and patch precedence against mutable prototypes.
- Dependencies / cross-references: ItemStackTemplate, ItemStack, DataComponentMap/DataComponentPatch, DataComponents.MAX_STACK_SIZE/MAX_DAMAGE/DAMAGE and `ItemStack.validateStrict`.
- Implementation / logic: Creates synthetic AIR/STICK/SHIELD prototypes, stacks and templates, applies template creation and component patches, changes prototypes and counts, and captures stack/template values, validity, patch contents and identity relationships.
- Findings: Rebinds global built-in prototypes and resets ItemStack.EMPTY; templates and components use modern API not found in the same form in 1.20.1; operation exceptions are narrowly handled; no thread or persistence lifecycle coverage.
- RustChunk relevance: Relevant to item stack construction, crafting output and item serialization/data patch layering.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Template API and component patch merge semantics need exact 1.20.1 mapping; treat this as later architecture rather than a drop-in facility.
- Follow-up checks: Test patch overriding/removing prototype values, null values, invalid count/max size, from empty/nonempty, from count, create/apply order and prototype mutation after template creation.
- RustChunk implementation mapping: Translate behavior into target stack construction/recipe output tests; use target primitives.
- Evidence required before adoption: Pinned SHA/size and EOF 39 lines verified; require target-version strict validation and template equivalence tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3456. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemUseCatalogOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 0c69903d3b3d5d34d5c62d592b4b8e1326fa8297
- Size: 4731 bytes
- Coverage: Complete EOF coverage (28 logical lines).
- Purpose: Export registered item usage defaults/duration/release implementations and query native ItemStack behavior after scripted durability, consumable, blocking, kinetic and instrument component changes.
- Dependencies / cross-references: BuiltInRegistries.ITEM/DATA_COMPONENT_TYPE, ItemStack, Item.getUseDuration/useOnRelease, Consumable, BlocksAttacks, KineticWeapon, InstrumentComponent and VanillaRegistries.
- Implementation / logic: Bootstraps vanilla registries and component initializers; export mode emits item registry IDs/classes/default component maps/instrument durations; case mode applies component/count operations and snapshots stack capability flags and use properties.
- Findings: Exports DataVersion 5023, confirming a later-version catalog; modern component types and item methods are not a 1.20.1 API contract; registry bootstrap and generated catalog schema are version-pinned.
- RustChunk relevance: Useful catalog methodology for item metadata, default components, use duration and capabilities.
- Decision: adaptable; reference; parity-data-exporter; parity-test-oracle
- 1.20.1 implications: Rebuild completely against 1.20.1 vanilla/Forge registries; do not import DataVersion 5023 or later components into the target item catalog.
- Follow-up checks: Validate catalog schema, every item, inherited vs overridden implementations, instrument references, optional/default components and item use/release semantics.
- RustChunk implementation mapping: Build a separate 1.20.1 target data exporter and compare with RustChunk item metadata.
- Evidence required before adoption: Pinned SHA/size and EOF 28 lines verified; require full 1.20.1 registry export and schema/version assertions.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3457. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemUseLifecycleOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: db0180ef07786eac91cf582148971dfce8e34ff2
- Size: 6793 bytes
- Coverage: Complete EOF coverage (38 logical lines).
- Purpose: Trace native LivingEntity updateUsingItem, completeUsingItem, releaseUsingItem and stopUsingItem lifecycle, including ticks, hand replacement, release queries, finish callbacks and finish vibration.
- Dependencies / cross-references: LivingEntity private useItem/useItemRemaining/recentKineticEnemies, Item callbacks onUseTick/useOnRelease/releaseUsing/finishUsingItem, ServerLevel and USE_EFFECTS/ KINETIC_WEAPON components.
- Implementation / logic: Builds custom items and server shells with private state injected, runs tick/update/complete/release/stop operations and captures callback sequence, hand/item identity, remaining ticks, client/server branch and injected failure/mutation.
- Findings: Uses Unsafe/private fields and manual synthetic actor/level; static EMPTY and modern UseEffects/KineticWeapon components; scripted callbacks do not establish packet delivery, actual audio/haptics or complete inventory consumption.
- RustChunk relevance: High relevance to item use animation/timing, eating/drinking, bow/crossbow-like charging and touch-button lifecycle.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern item-use state fields and components differ from 1.20.1; map to target LivingEntity/Item release callbacks and Forge extensions.
- Follow-up checks: Test duration expiry, early release, client/server cases, hand changes, replaced stack identity, finish result, vibration, kinetic tracking and callback failure at each step.
- RustChunk implementation mapping: Use its scenarios to define RustChunk input-to-item-use lifecycle state machine and synchronization tests.
- Evidence required before adoption: Pinned SHA/size and EOF 38 lines verified; require target-version callback trace and server/client item-use integration.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3458. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemUseOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 5421a1f613dff90d5d53ed9ef37a3a32603c3ecc
- Size: 4581 bytes
- Coverage: Complete EOF coverage (24 logical lines).
- Purpose: Probe native item blocking selection and use timers from scripted item durations and BLOCKS_ATTACKS component presence.
- Dependencies / cross-references: LivingEntity.getItemBlockingWith/isBlocking/getTicksUsingItem/getTicksUsingItem(partial), ItemStack BLOCKS_ATTACKS and custom Item.getUseDuration.
- Implementation / logic: Builds custom actor/items with controlled component delay and remaining ticks, then invokes blocking and timer methods while callbacks can mutate or fail.
- Findings: Only two custom items and one empty stack; fake Actor reports `isUsingItem` via scripted state; modern BLOCKS_ATTACKS component is not the 1.20.1 shield condition API.
- RustChunk relevance: Useful for blocking state and item-use progress UI.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Rebuild using 1.20.1 shield-use duration and blocking conditions; don't assume BLOCKS_ATTACKS component exists in target.
- Follow-up checks: Test no item/empty item, both hand candidates, duration <=0, remaining > duration, fractional partial tick, blocking delay and component removal.
- RustChunk implementation mapping: Use cases for target shield/blocking and hotbar animation tests.
- Evidence required before adoption: Pinned SHA/size and EOF 24 lines verified; require 1.20.1 shield-state comparison and actual first-person shield display.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3459. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemUseStartOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 24041611c08ae39817d34a6482c0ba6aa81123d3
- Size: 8097 bytes
- Coverage: Complete EOF coverage (39 logical lines).
- Purpose: Exercise inherited LivingEntity.startUsingItem including flags, selected hand, use duration, server tracking equipment packet, kinetic state and item-start game event.
- Dependencies / cross-references: LivingEntity.startUsingItem, Item.getUseDuration/onUseTick/useOnRelease/releaseUsing/finishUsingItem, ServerChunkCache.sendToTrackingPlayers, Synched flags, USE_EFFECTS/KINETIC_WEAPON, Unsafe.
- Implementation / logic: Builds paired ServerLevel/ServerChunkCache stand-ins, custom items and tracked Entity shell, then records calls to hand getters/setters, entity IDs, flags, duration, packets, vibration and kinetic enemy map.
- Findings: Hard-codes entity flag values and private fields; constructs synthetic packets but no connected client/network transport; modern use effect and kinetic components; `ServerPlayer`-like server and actor are minimal.
- RustChunk relevance: High relevance to item interaction start and network synchronization for animations.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Target 1.20.1 startUsingItem and equipment sync are different; verify hand state and packet order with Forge 47.x.
- Follow-up checks: Test client/server side, hand selected vs requested, zero/negative duration, tracking players, item stack replacement, packet contents and event order.
- RustChunk implementation mapping: Derive start-use state transition and packet test vectors for RustChunk.
- Evidence required before adoption: Pinned SHA/size and EOF 39 lines verified; require exact 1.20.1 packet/event sequence and real multiplayer tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3460. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaItemWireOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: cc95bb3110625958577fca971bd1ec25c5ef82b2
- Size: 7466 bytes
- Coverage: Complete EOF coverage (155 logical lines).
- Purpose: Capture ItemStack.OPTIONAL_STREAM_CODEC wire bytes for default and fixture component patches, added/removed component IDs and direct-holder/non-persistent values.
- Dependencies / cross-references: ItemStack.OPTIONAL_STREAM_CODEC, RegistryFriendlyByteBuf, DataComponentPatch, BuiltInRegistries.ITEM/DATA_COMPONENT_TYPE, ParityData SNBT decoder, Jukebox/painting/instrument/map components.
- Implementation / logic: Enumerates vanilla item component defaults, applies each component to a stick, deduplicates and caps 12 samples per type, serializes to hex and lists component type IDs in patch iteration order, then emits explicit fixtures including nonpersistent/direct holders, removals and EMPTY.
- Findings: 155 logical lines. Uses current registry numeric IDs, current DataComponentPatch iteration order and current serialization codec; caps examples at 12 per component type; helper catches encode RuntimeException and emits generic failure; this is a component-codec oracle, not a full client-server negotiation/packet test.
- RustChunk relevance: Very high relevance to network inventory/item synchronization and protocol compatibility.
- Decision: reference; parity-test-oracle; version-specific
- 1.20.1 implications: Do not use these bytes or component IDs for 1.20.1; 1.20.1 has different item stack wire representation. Recreate a separate 1.20.1 packet oracle; current data may reflect much newer component registry.
- Follow-up checks: Test all target ItemStack packets, registry ID remapping, custom Forge components, removed vs added values, holder codecs, malformed buffers and encode/decode symmetry.
- RustChunk implementation mapping: Use only as a method for building version-specific wire parity vectors; never copy byte strings/IDs into RustChunk target protocol.
- Evidence required before adoption: Pinned SHA/size and EOF 155 lines verified; require bidirectional 1.20.1/Forge encode-decode goldens and real server interoperability.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3461. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaJavaHashMapOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: c945b6596003826c5724388af2f63bc17ffddd27
- Size: 2250 bytes
- Coverage: Complete EOF coverage (19 logical lines).
- Purpose: Exercise the running JDK's HashMap insertion/removal/get/clear and fail-fast iterator/cursor behavior with controlled hashes and mutation sequence.
- Dependencies / cross-references: java.util.HashMap, custom comparable Key, Map.Entry iterator and ConcurrentModificationException semantics.
- Implementation / logic: Runs ordered operation streams against an actual JVM HashMap, exposes current key order and iterator progression, and captures iterator remove, stale iterator and missing-entry behavior.
- Findings: The class only tests the active Java runtime's HashMap behavior, not a Minecraft item cooldown collection; iteration order is intentionally not a stable contract and collision resize/treeification depends on JDK implementation.
- RustChunk relevance: Potentially useful only if RustChunk must reproduce exact Java collection iteration under compatibility constraints.
- Decision: reference; runtime-specific; parity-test-oracle
- 1.20.1 implications: Never assume Android/libcore Rust ordering equals desktop JDK HashMap; exact JDK behavior is not portable across Android/JDK releases.
- Follow-up checks: Run cases on chosen Java baseline and Samsung Android runtime only if order fidelity is genuinely needed; test resize/collision/tie-break/cursor invalidation.
- RustChunk implementation mapping: Prefer semantic maps in RustChunk; emulate Java HashMap only when a target behavior demonstrably exposes iteration order.
- Evidence required before adoption: Pinned SHA/size and EOF 19 lines verified; require runtime-specific test on Android and do not treat iteration sequence as language-independent.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3462. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaJavaIdentityMapOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 617a6c62d081df32a8f0fb4d027b67ce10357145
- Size: 3648 bytes
- Coverage: Complete EOF coverage (28 logical lines).
- Purpose: Exercise HashMap collision-bin behavior with custom comparable and non-comparable key classes, copy/save/swap and iterator operations while recording identity hashes.
- Dependencies / cross-references: java.util.HashMap, custom key types A/B/C/D with varied equals/hashCode/Comparable, System.identityHashCode and iterator APIs.
- Implementation / logic: Creates key objects with controlled logical IDs and hash collisions, persists maps/copies, swaps them, mutates through a cursor, and records iteration plus identity hash values.
- Findings: Despite filename `IdentityMap`, implementation uses `HashMap`, not `IdentityHashMap`; `equals` checks same concrete class plus id; identity hashes are logged but key equality is value-based. `Comparable<C>` compares modulo four and can return zero for distinct IDs.
- RustChunk relevance: A blind spot for any assumption about object identity semantics in Java compatibility; otherwise reference for HashMap collision behavior only.
- Decision: reference; runtime-specific; parity-test-oracle
- 1.20.1 implications: Do not port as identity-map semantics. Android ART HashMap internals and tie-break order can differ from current JVM; avoid observable order assumptions.
- Follow-up checks: Test separate same-class IDs, cross-class equal IDs, compareTo ties, hash collisions, copies and identity-hash output under actual device runtime.
- RustChunk implementation mapping: Use only to validate collision behavior if game-visible semantics require it; implement explicit identity keys in Rust when identity is intended.
- Evidence required before adoption: Pinned SHA/size and EOF 28 lines verified; require tests against Android runtime and confirm no code expects IdentityHashMap.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3463. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLanAuthOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: e9a28575090fe29b6734baac84ab58ab470d6532
- Size: 3616 bytes
- Coverage: Complete EOF coverage (60 logical lines).
- Purpose: Verify client login listener behavior when session-service authentication throws InvalidCredentialsException for LAN vs non-LAN server entries.
- Dependencies / cross-references: ClientHandshakePacketListenerImpl `handleHello` lambda, ServerData.Type, SessionService.joinServer, ServerboundKeyPacket, Crypt and Connection.
- Implementation / logic: Creates an Unsafe Minecraft client with synthetic user/services, SessionService dynamic proxy that throws authentication rejection, invokes the real private handshake callback in AUTHORIZING state and records whether a key packet is sent or the connection is disconnected for LAN and OTHER.
- Findings: Only the LAN enum case is specially accepted in the tested failure branch; the oracle does not implement general offline-mode login, remove authentication, alter server auth or prove any remote-server compatibility.
- RustChunk relevance: Directly relevant to LAN session behavior but not a mechanism for Aternos/offline account support.
- Decision: reference; parity-test-oracle; authentication-boundary
- 1.20.1 implications: Treat this as a narrow vanilla LAN exception only. It does not authorize offline accounts on public servers or permit joining online-mode Aternos without valid authentication.
- Follow-up checks: Check 1.20.1 handshake path, server data type classification, failed vs successful joinServer, packet/disconnect ordering and ensure all OTHER/remote servers retain authentication enforcement.
- RustChunk implementation mapping: Document as a non-feature for RustChunk account/auth design; keep legitimate authentication and LAN-specific behavior separate.
- Evidence required before adoption: Pinned SHA/size and EOF 60 lines verified; require 1.20.1 handshake comparison and real LAN/online authentication tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3464. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLanOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: b7c4c1b0f0ab05ec8a1e9cf6dab055a64e709ced
- Size: 2655 bytes
- Coverage: Complete EOF coverage (52 logical lines).
- Purpose: Exercise LAN multiplayer announcement formatting and received-ping parsing/list updates used by vanilla LAN world discovery.
- Dependencies / cross-references: LanServerPinger.createPingString/parseMotd/parseAddress, LanServerDetection.LanServerList, LanServer and InetAddress.
- Implementation / logic: Processes ping/MOTD/port, heard/IP/hex datagram and reset commands; records UTF-8 ping bytes, parsed fields and dirty server-list changes.
- Findings: 52 lines; tests codec/list handling only, does not open a UDP socket or measure broadcast discovery; tab splitting and hex/IP fields are trusted, and malformed UTF-8/duplicate announcements need additional tests.
- RustChunk relevance: Useful if RustChunk exposes local Wi-Fi/LAN world discovery UI.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Compare broadcast protocol and list semantics with 1.20.1; actual Android multicast/Wi-Fi permissions, broadcast locks and network interfaces are separate platform concerns.
- Follow-up checks: Test malformed datagrams, tabs in MOTD, invalid port, duplicate IP/port, expiry, interface changes, Android multicast lock and local subnet filtering.
- RustChunk implementation mapping: Use as the message-format and list-state oracle for a separate Android LAN discovery service.
- Evidence required before adoption: Pinned SHA/size and EOF 52 lines verified; require real device UDP/multicast tests and 1.20.1 message parity.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3465. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLevelShapeOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: e3ff99b24a9e80cfdb2f8e68b78c9cec68de95c6
- Size: 8183 bytes
- Coverage: Complete EOF coverage (133 logical lines).
- Purpose: Run BlockState.updateShape and canSurvive across synthetic neighboring cells while capturing scheduled block/fluid ticks.
- Dependencies / cross-references: BlockState.updateShape/canSurvive, LevelReader proxy, ScheduledTickAccess/LevelTickAccess, TagLoader for block/fluid tags, Voxel/world state and LegacyRandomSource.
- Implementation / logic: Loads tag JSON, creates a cell map of block states, responds to LevelReader queries via dynamic proxy, intercepts scheduled tick requests into a set and runs updateShape plus survival query for selected position/direction.
- Findings: Uses fixed world bounds minY=-64 height=384, fixed brightness/environment attributes, default AIR for absent cells, synthetic no-tick access, registry/tag mutation and narrow proxy method support; context-sensitive state behavior beyond supplied cells isn't represented.
- RustChunk relevance: High relevance to block neighbor updates and chunk-edge stability/correct block states.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: The world access API includes later EnvironmentAttributeReader and newer LevelReader methods; reconstruct target 1.20.1 neighbor-update semantics and dynamic Forge block hooks.
- Follow-up checks: Test six directions, absent neighbors, fluids, redstone, fences, doors, support changes, delayed tick type/priority and actual chunk border/loading behavior.
- RustChunk implementation mapping: Use as neighbor-update fixture/reference while implementing target chunk/block update scheduling.
- Evidence required before adoption: Pinned SHA/size and EOF 133 lines verified; require target-version updateShape outputs and multi-chunk Android integration tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3466. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLiveAttributeUpdatesOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: ca25e3d2b8545af032ff9044b0117669d853395c
- Size: 15313 bytes
- Coverage: Complete EOF coverage (197 logical lines).
- Purpose: Exercise live attribute updates, health/absorption synchronized data, effect add/update/remove callbacks, dirty refresh, attribute mutation and waypoint tracking.
- Dependencies / cross-references: AttributeMap/AttributeInstance, EntityDataAccessor/ SynchedEntityData, LivingEntity health and absorption, MobEffect callbacks, ServerWaypointManager/WaypointTransmitter and Unsafe.
- Implementation / logic: Creates two attribute maps and health data stores, scripted server/effect/manager contexts, invokes attribute and effect update paths, mutates maps during callbacks and records update/sync sets, dirty values, health data and waypoint transitions.
- Findings: 197 lines; shares static helpers/state with VanillaEntityAttributeOracle and VanillaLiveAttributesOracle; manually builds SynchedEntityData array and health accessor; fake manager tracks events only; no actual packet dispatch, real server tick or waypoint network transport.
- RustChunk relevance: High relevance to dynamic entity stats/health and server-client sync consistency.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern attribute/effect and waypoint APIs differ from 1.20.1; sync semantics and Forge attribute extension behavior must be re-established for target.
- Follow-up checks: Test effect callbacks and mutation failure, separate attribute maps, null/unknown holders, refresh dirty ordering, client vs server waypoint transitions, health accessor dirtiness and packet packing.
- RustChunk implementation mapping: Use cases to design RustChunk entity-state replication and stat-update tests; keep server authority distinct.
- Evidence required before adoption: Pinned SHA/size and EOF 197 lines verified; require 1.20.1 synchronized entity data comparison and real packet/client replication tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3467. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLiveAttributesOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 7a9449e97658558eabcc38b80bd4e28cedc79c20
- Size: 9277 bytes
- Coverage: Complete EOF coverage (150 logical lines).
- Purpose: Export and exercise native registered attribute suppliers, AttributeMap instance creation, values, base values, modifiers and pending update/sync collections.
- Dependencies / cross-references: BuiltInRegistries.ATTRIBUTE/ENTITY_TYPE, AttributeSupplier, AttributeMap/AttributeInstance, AttributeModifier, MobEffect and reflection into map/cache fields.
- Implementation / logic: Builds actual maps from registered entity suppliers, executes value/base/has/ensure/reset/define/add/update/permanent/remove/clear and effect-modifier operations, then records instance references, cached values, dirty flags and pending update/sync sets.
- Findings: 150 lines; static registry attribute catalog and helper are imported from another oracle; introspects private map, dirty and cached fields; iteration orders of maps/modifiers may be implementation-specific.
- RustChunk relevance: Useful for mob/player attribute calculation and synchronized attributes in vanilla compatibility.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Attribute identifiers, supplier defaults and modifier operations must be regenerated from 1.20.1; later attributes and modifier identifiers may not exist.
- Follow-up checks: Test every target entity supplier, attribute absent/default state, modifier duplicate/replace/remove, operation order, cache invalidation and update vs sync acknowledgements.
- RustChunk implementation mapping: Use as differential tests for RustChunk attribute model and modded attribute handling.
- Evidence required before adoption: Pinned SHA/size and EOF 150 lines verified; require 1.20.1 full registry export and target server/client modifier tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3468. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLivingAirOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 143d926cc365f7d3c9da44550b23a989aeaf8c78
- Size: 12486 bytes
- Coverage: Complete EOF coverage (84 logical lines).
- Purpose: Exercise native LivingEntity air movement, relative friction, climb handling, speed, input vector and friction calculation for Player and non-player actors.
- Dependencies / cross-references: LivingEntity.travelInAir/handleRelativeFrictionAndCalculateMovement/handleOnClimbable/getFrictionInfluencedSpeed; Entity.getInputVector; friction/jump/slow-fall effects, attributes, blocks and Unsafe.
- Implementation / logic: Creates scripted actor/player shells with controlled velocity, yaw, ground/climb/snow/boots/levitation/speed/friction/gravity inputs, fake Level/Block/BlockState and invokes selected private movement methods; records query order and float/vector bits.
- Findings: Only controlled queries are simulated; support/ground/climb/sprinting flags are scripted; player implementation is a minimal Survival shell; uses Unsafe/private fields and does not run a real movement tick or collision world.
- RustChunk relevance: Very high relevance to movement/physics and client/server movement parity.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Movement constants, attributes and algorithm differ in newer version; compare exactly with 1.20.1 Entity/LivingEntity and Forge patches; this does not validate Android touch input.
- Follow-up checks: Test negative/large velocity, friction boundaries, yaw, jump boosts, slow falling, powder snow, boots, collision flags, repeated movement and float-bit determinism.
- RustChunk implementation mapping: Use as movement oracle scenarios for RustChunk native physics; keep input sampling/collision resolution separate.
- Evidence required before adoption: Pinned SHA/size and EOF 84 lines verified; require target 1.20.1 seeded outputs and real multiplayer movement/server correction tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3469. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLivingDamageOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 77d8b1eb3f030d2a18e6729d6d1e9bb44d1bc656
- Size: 7259 bytes
- Coverage: Complete EOF coverage (60 logical lines).
- Purpose: Trace native LivingEntity.actuallyHurt damage application phases for players and non-player entities.
- Dependencies / cross-references: DamageSource, CombatTracker, Attributes.MAX_ABSORPTION, ServerLevel, GameEvent.ENTITY_DAMAGE, armor/magic absorption and Player exhaustion/stats.
- Implementation / logic: Overrides invulnerability, armor/magic absorption, health/absorption accessors, damage tracker and event callbacks; invokes actual protected actuallyHurt and records each stage, absorption reduction, health, exhaustion and stats.
- Findings: Most inputs and effects are scripted; only inner damage application is executed, not outer hurt cooldown/tag/knockback path; Player and ArmorStand shells are Unsafe stand-ins; target max-absorption attribute is modern.
- RustChunk relevance: Useful for damage application order, armor/resistance, absorption and survival exhaustion.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 has different maximum absorption APIs and may lack this exact attribute; reconstruct target damage calculation from matching source and Forge hooks.
- Follow-up checks: Test zero/negative/overkill, invulnerable, armor/magic reduction, absorption limits, lethal, player vs mob, damage event and attacker/target stats.
- RustChunk implementation mapping: Use scenarios for target combat/damage engine regression tests; avoid copying modern Attributes.MAX_ABSORPTION semantics.
- Evidence required before adoption: Pinned SHA/size and EOF 60 lines verified; require exact target-version damage/health results and complete combat outer-path integration.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3470. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLivingFallOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 18eebac2cf1b5798da54976b3750e54d5ea48da1
- Size: 6156 bytes
- Coverage: Complete EOF coverage (49 logical lines).
- Purpose: Exercise native fall-damage calculation/application and per-impulse fall-immunity context lifecycle.
- Dependencies / cross-references: LivingEntity.calculateFallDamage/causeFallDamage, EntityTypeTags.FALL_DAMAGE_IMMUNE, SAFE_FALL_DISTANCE/FALL_DAMAGE_MULTIPLIER, impulse grace fields and DamageSource.
- Implementation / logic: Calls native fall calculation or cause path, scripts successive immunity tag results, attribute factors, sounds, server hurt result and passenger propagation; tests reset/apply grace/ignore current impulse context operations.
- Findings: Uses private fields for impulse grace/impact position; fall sound callbacks scripted; ServerLevel shell and hurtServer return scripted; no real block landing or world physics in this oracle.
- RustChunk relevance: High relevance to landing damage and movement correctness.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Tag/attributes/impulse context may be later-version APIs; compare exact 1.20.1 fall formula and passenger rules before adoption.
- Follow-up checks: Test immunity tags, safe distance, damage multiplier, boundary values, passenger propagation, null impact, grace timers, server/client and hurt denial.
- RustChunk implementation mapping: Use as target fall/impulse regression scenarios; couple with the landing oracle and collision tests.
- Evidence required before adoption: Pinned SHA/size and EOF 49 lines verified; require target 1.20.1 fall outcomes and real server damage event tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3471. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLivingFluidOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: b37df7384bd1385e5d7ad442647fd04b7b531450
- Size: 16629 bytes
- Coverage: Complete EOF coverage (101 logical lines).
- Purpose: Exercise native LivingEntity fluid, water and lava travel, falling adjustment, jump-out, ridden float behavior and shared air movement helpers.
- Dependencies / cross-references: LivingEntity.travelInFluid/travelInWater/travelInLava/getFluidFallingAdjustedMovement/jumpOutOfFluid/floatInLiquidWhileRidden; FluidTags, WATER_MOVEMENT_EFFICIENCY, Fluid height and movement attributes.
- Implementation / logic: Uses fake Level/Block/State, scripted fluid heights and flags (water/lava/floatable, dolphin grace, sprint, vehicle, boots, levitation, collision/free), and invokes private fluid and movement methods with controlled random-free vector inputs; records query order, resulting velocity and position.
- Findings: 101 lines; combines fluid and air methods, uses scripted isFree/chunk/block responses, one fake friction state, two actor implementations; fluid tag and attribute set is later-version specific and does not simulate actual water collision geometry.
- RustChunk relevance: Very high relevance to swimming/lava movement and chunk-boundary fluid interactions.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern fluid movement helpers and WATER_MOVEMENT_EFFICIENCY attribute may differ from 1.20.1; rebuild fixtures from target vanilla/Forge code.
- Follow-up checks: Test water/lava/no-fluid, fall adjustment, jump threshold, depth-strider/boots equivalents, dolphin grace, ride floating, negative minY, free/collision failures and fluid-height transitions.
- RustChunk implementation mapping: Use as movement/physics scenario reference; implement target fluid travel independently of Java oracle.
- Evidence required before adoption: Pinned SHA/size and EOF 101 lines verified; require 1.20.1 movement outputs, fluid collision integration and Android device physics tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3472. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLivingHealthOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: e50c50b64f8062db27d0f4b47ba75dd01aaea690
- Size: 5641 bytes
- Coverage: Complete EOF coverage (43 logical lines).
- Purpose: Exercise native health setter/heal/absorption and synchronized health data semantics across active entity-data stores.
- Dependencies / cross-references: LivingEntity heal/setHealth/isHurt/isDeadOrDying/setAbsorptionAmount, Attribute.MAX_HEALTH/MAX_ABSORPTION, SynchedEntityData, health DataItem and Unsafe.
- Implementation / logic: Builds two independent SynchedEntityData stores with the real health accessor, injects dead/absorption state, runs operations and records health value, dirty/default bits, active-store switching and callbacks.
- Findings: Health/max absorption getters are scripted; two stores are synthetic; direct base health setter bypasses the override for fixture creation; no network pack/send/reception is exercised.
- RustChunk relevance: Useful to keep health bars, damage state and client-synced health consistent.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Verify exact 1.20.1 LivingEntity health/absorption and SynchedEntityData dirty semantics; modern MAX_ABSORPTION attribute may not exist.
- Follow-up checks: Test healing caps, zero/negative/max health, absorption, dead state, force-sync vs ordinary set, pack/dirty transitions, data store switching and callback failure.
- RustChunk implementation mapping: Use as target health state and sync regression fixtures; actual HUD and packet replication need separate tests.
- Evidence required before adoption: Pinned SHA/size and EOF 43 lines verified; require 1.20.1 health source comparison and connected client health update tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3473. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLivingHurtOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 0b43ba05b1bb8b2517a5f794781a4c80ab27914f
- Size: 12577 bytes
- Coverage: Complete EOF coverage (86 logical lines).
- Purpose: Invoke actual LivingEntity.hurtServer through a special MethodHandle and trace outer hurt lifecycle including cooldown, invulnerability, damage tags, shield blocking, effects, knockback, sounds and death.
- Dependencies / cross-references: LivingEntity.hurtServer, DamageSource/DamageTypeTags/DamageTypes, ServerLevel, BlocksAttacks, shield ItemStack, MobEffect.onMobHurt, Combat stats, PlayerAdvancements and private damage timers.
- Implementation / logic: Uses `MethodHandles.privateLookupIn/findSpecial` to call the base implementation, scripted Actor/ServerPlayer overrides to record branch decisions, state fields for cooldown/hurt time/source stamp and synthetic status effects.
- Findings: Unsafe/private fields and MethodHandles require module access; most world/stat/sound/knockback/helmet/blocking effects are recorded through stub overrides; modern tag/component logic; the method is invoked for a shell and no real server packet/combat world exists.
- RustChunk relevance: Very high relevance to combat and multiplayer damage acceptance tests.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 hurtServer signature and damage branch logic are version specific; shield logic then differs from modern BlocksAttacks; regenerate from 1.20.1 Forge code.
- Follow-up checks: Test every DamageTypeTag, cooldown bypass, last-source stamp, freeze/fire, sleeping, shield/helmet, resistance callbacks, player stats, no-impact/no-knockback and lethal transition with target code.
- RustChunk implementation mapping: Use as a target server combat event-order matrix; do not port the modern implementation or synthetic networking.
- Evidence required before adoption: Pinned SHA/size and EOF 86 lines verified; require target 1.20.1 hurtServer branch comparisons and real dedicated-server combat tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3474. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLivingLandingOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: b5f354b86d92097bc52ddb43669c2ff649fd3e85
- Size: 12568 bytes
- Coverage: Complete EOF coverage (90 logical lines).
- Purpose: Run native LivingEntity.checkFallDamage landing behavior for server/client levels, fluid updates, block changes, particles, fallOn and HIT_GROUND event.
- Dependencies / cross-references: LivingEntity.checkFallDamage, Level/ServerLevel, BlockState.isAir/Block.fallOn, BlockParticleOption, GameEvent.HIT_GROUND, support position and private climbable/position fields.
- Implementation / logic: Builds server/client stand-in levels and blocks/states, scripts water/fluid state, fall distance, ground flag, support/landing positions and injected mutation/failure in callbacks; captures native block state reads, particles, game events, fall distance and positional state.
- Findings: 90 lines with a large ClientLevel subclass whose unrelated methods throw UnsupportedOperationException; BlockState and Level values are synthetic; particle emission is captured at `sendParticles`, not drawn; callback ordering only covers configured methods.
- RustChunk relevance: High relevance to landing behavior and chunk block lookup consistency.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Target `checkFallDamage` in 1.20.1 may differ; block particle/event signatures, climbing support and fluid behavior require remapping.
- Follow-up checks: Test server vs client, air vs non-air, water/fluid transition, support absent/present, particle count/position, mutation during fallOn, HIT_GROUND order and chunk edge.
- RustChunk implementation mapping: Use as target landing/ground-state test specification; keep actual block/particle world integration separate.
- Evidence required before adoption: Pinned SHA/size and EOF 90 lines verified; require target 1.20.1 callback order and real block/particle rendering at chunk boundaries.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3475. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLivingMotionOracle.java`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: 1a05fa5781dbd21626be699a3188b3d095ce255a
- Size: 6262 bytes
- Coverage: Complete EOF coverage (42 logical lines).
- Purpose: Exercise LivingEntity jump power, jump boost, jump/flight gravity, applyGravity, water jump, downward water movement, flying speed and air drag.
- Dependencies / cross-references: LivingEntity.getJumpPower/jumpFromGround/getEffectiveGravity/jumpInLiquid/goDownInWater/getFlyingSpeed/getAirDrag; Entity.applyGravity/addDeltaMovement, attributes, effects and passenger/controller state.
- Implementation / logic: Creates an Unsafe entity shell with controlled velocity, effect amplifier, attributes, yaw/sprint/gravity/passenger/sync flags and invokes private/native methods, recording query order, result bits and velocity mutations.
- Findings: 42 lines; scripted passenger/player and effect state, no collision/world tick; uses modern GRAVITY/AIR_DRAG_MODIFIER attributes and methods; private amplifier access; no input sampling, packet or renderer integration.
- RustChunk relevance: High relevance to jumping, gravity, movement feel and vehicle-driven control.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Reconstruct target 1.20.1 gravity/jump constants and attributes; modern air drag/gravity architecture may not exist in target form.
- Follow-up checks: Test jump boost amplifier, block jump factor, no-gravity, sprint yaw impulse, slow falling, rider passenger controls, water up/down and velocity mutation/failure on target.
- RustChunk implementation mapping: Use as target movement parameter regression cases in RustChunk physics.
- Evidence required before adoption: Pinned SHA/size and EOF 42 lines verified; require 1.20.1 exact seeded movement outputs and real mobile movement/server correction tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static review only; no execution.

### 3476. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLocalInputOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 229abcd7020932cb4f28851d6d0563808e7f6fb4
- Size: 4020 bytes
- Coverage: Complete EOF coverage (54 logical lines).
- Purpose: InputKeyboard/LocalPlayer vector generation.
- Dependencies / cross-references: Native tick/modifyInput; scripted keys; captures vectors and flags.
- Implementation / logic: Synthetic input/player and later UseEffects branch; not touch/network.
- Findings: Android movement input reference.
- RustChunk relevance: Useful for key-state handling and player movement input.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Recheck 1.20.1 diagonal, crouch, sprint, flight and use slowdown.
- Follow-up checks: Golden vectors plus Android touch integration.
- RustChunk implementation mapping: Use as input-state regression cases.
- Evidence required before adoption: Require 1.20.1 vectors and device tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3477. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLoomScreenOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 105329c1d30a522569d06ceb65b3929ecf3c2e74
- Size: 15256 bytes
- Coverage: Complete EOF coverage (282 logical lines).
- Purpose: Loom/Crafter background and disabled-slot draw calls.
- Dependencies / cross-references: Native screen extraction with recording GuiGraphicsExtractor.
- Implementation / logic: Synthetic GUI internals/atlas; no framebuffer, pixels or GPU.
- Findings: GUI state/draw order.
- RustChunk relevance: Useful for native container-screen visual parity.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern graphics extraction differs from 1.20.1 Forge screens.
- Follow-up checks: Compare textures, slots, recipe states and scale.
- RustChunk implementation mapping: UI tests for Rust widgets.
- Evidence required before adoption: Require 1.20.1 and Android screenshots.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3478. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLootFillOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: b52bb1b7d6b1b9f85ba08b9450516b631deec8ce
- Size: 7633 bytes
- Coverage: Complete EOF coverage (109 logical lines).
- Purpose: Seeded loot table generation and 27-slot container fill.
- Dependencies / cross-references: Registry/data pack, LootTable/LootParams, RandomSource, Container.
- Implementation / logic: Actual loot generation over reduced server/player/location fixture; newer context/components.
- Findings: Loot/chest contents.
- RustChunk relevance: Useful for deterministic loot and inventory population.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Regenerate from 1.20.1 and its Forge loot modifiers.
- Follow-up checks: Test luck, conditions, stack split, full slots, RNG.
- RustChunk implementation mapping: Target loot golden tests.
- Evidence required before adoption: Require 1.20.1 loot corpus.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3479. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaLoreTooltipOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 462a8016a9fc890f8ac862434b2b70119b531cc5
- Size: 2609 bytes
- Coverage: Complete EOF coverage (58 logical lines).
- Purpose: ItemLore tooltip component display.
- Dependencies / cross-references: ItemLore provider, modern tooltip components/styles.
- Implementation / logic: Lore only; no full tooltip, localization, font wrap or render; modern component.
- Findings: Item lore UI.
- RustChunk relevance: Useful for item lore and tooltip compatibility.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 uses legacy display Lore path.
- Follow-up checks: Check flags, styles, localization and legacy NBT.
- RustChunk implementation mapping: Target tooltip fixtures.
- Evidence required before adoption: Require target tooltip and rendering tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3480. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMarkerParticlesOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: efcdaf18801d569937c9e0857b1a32c1e1072a6e
- Size: 4987 bytes
- Coverage: Complete EOF coverage (98 logical lines).
- Purpose: Marker particle target and ambient animateTick.
- Dependencies / cross-references: ClientLevel marker helpers, Player, ParticleOptions, RandomSource.
- Implementation / logic: Synthetic client level; only marker particles, no GPU; global prototype mutation.
- Findings: Debug particle feedback.
- RustChunk relevance: Useful for marker targeting and debug particle effects.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Compare target marker logic; unrelated to terrain chunks.
- Follow-up checks: Test game mode, range and seeded positions.
- RustChunk implementation mapping: Narrow particle test reference.
- Evidence required before adoption: Require 1.20.1 and device renderer results.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3481. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMaterialConditionOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: d5efa48bcbd8df2f91ccf4812bdbd00a5012b0fe
- Size: 22071 bytes
- Coverage: Complete EOF coverage (251 logical lines).
- Purpose: Material condition/rule codec and evaluator harness.
- Dependencies / cross-references: MaterialSystem codecs, biome/noise/density registries and context.
- Implementation / logic: 251 lines; private reflection, synthetic density/ore, global buffer arena; current-generation API.
- Findings: Surface rule test design.
- RustChunk relevance: Useful as a reference model for conditional surface generation.
- Decision: adaptable; reference; parity-test-oracle; version-specific
- 1.20.1 implications: Not 1.20.1 SurfaceRules; no direct data/code port.
- Follow-up checks: Compare target conditions, tags, anchors, caches and rule order.
- RustChunk implementation mapping: Port oracle concept only.
- Evidence required before adoption: Require target column goldens.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3482. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMaterialExtensionOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 2204fa6e08f73e81e222b9316b5e5f542bb521b7
- Size: 6788 bytes
- Coverage: Complete EOF coverage (67 logical lines).
- Purpose: Badlands and frozen ocean surface column helpers.
- Dependencies / cross-references: MaterialSystem private extensions, six noises, BlockColumn.
- Implementation / logic: Private reflective calls and limited mutable column fixture; no chunk pipeline.
- Findings: Surface extension behavior.
- RustChunk relevance: Useful for badlands and frozen-ocean material algorithm analysis.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Target 1.20.1 surface rules differ.
- Follow-up checks: Compare noise coordinates, bounds and writes.
- RustChunk implementation mapping: Target surface-rule cases.
- Evidence required before adoption: Require source-equivalent target outputs.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3483. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMaterialStageOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 8a2e81a713a53f56d878fe39d14f995e4dd8585f
- Size: 18691 bytes
- Coverage: Complete EOF coverage (181 logical lines).
- Purpose: ProtoChunk material surface, fill, aquifer and carving.
- Dependencies / cross-references: MaterialSystem, NoiseChunk, RandomState, ProtoChunk, BiomeSource, Aquifer, CarvingMask.
- Implementation / logic: 3x3 fixture chunks and real native methods; no world scheduler/light/persistence.
- Findings: World generation/chunk correctness.
- RustChunk relevance: Highly relevant to chunk surface, aquifer and carving behavior.
- Decision: adaptable; reference; parity-test-oracle; version-specific
- 1.20.1 implications: Current system differs from 1.20.1 SurfaceRules/pipeline.
- Follow-up checks: Compare negative Y, palettes, heightmaps, carvers, fluids.
- RustChunk implementation mapping: Blueprint target worldgen harness.
- Evidence required before adoption: Require 1.20.1 generated columns.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3484. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMaterialSurfaceOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: e3a7bb28739b30f197d219c9b67a9ffddffd8d57
- Size: 5514 bytes
- Coverage: Complete EOF coverage (75 logical lines).
- Purpose: Seeded clay bands and surface depth/noise lookup.
- Dependencies / cross-references: MaterialSystem bands, positional RandomSource and noises.
- Implementation / logic: Private fields; limited terracotta palette and no full terrain.
- Findings: Badlands palette determinism.
- RustChunk relevance: Useful for deterministic surface palette generation.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Rebuild against 1.20.1 badlands logic.
- Follow-up checks: Test seed, RNG, coordinate and palette edges.
- RustChunk implementation mapping: Target band tests.
- Evidence required before adoption: Require 1.20.1 seeded vectors.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3485. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMenuOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 3c7de61efcbb37db090b223f56953d9c52df414d
- Size: 25306 bytes
- Coverage: Read through EOF: lines 1–200 and 201–403 (403 logical lines).
- Purpose: Vanilla AbstractContainerMenu clicks across many screen/container classes.
- Dependencies / cross-references: Real menu subclasses, Inventory, RecipeManager, synthetic ServerPlayer/Level.
- Implementation / logic: 403 lines in ranges 1–200 and 201–403; Unsafe fake level/player; packets/world effects stubbed.
- Findings: Container/inventory operation reference.
- RustChunk relevance: Highly relevant to menu input and container transfer behavior.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Modern ContainerInput differs from 1.20.1 ClickType/Forge.
- Follow-up checks: Test all click modes, quick-move, output, limits, craft and resync.
- RustChunk implementation mapping: Target menu operation tests.
- Evidence required before adoption: Require 1.20.1 Forge menu/network tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3486. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMipmapOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 2ecc3c4853333dda933a64f30249179044ba0870
- Size: 3416 bytes
- Coverage: Complete EOF coverage (65 logical lines).
- Purpose: Texture PNG/mcmeta mip-level generation.
- Dependencies / cross-references: NativeImage, MipmapGenerator and strategy.
- Implementation / logic: Outputs mip RGBA; no GPU upload; review image close lifecycle and non-power-of-two cases.
- Findings: Texture preprocessing.
- RustChunk relevance: Directly relevant to texture filtering and mipmap preprocessing.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Target 1.20.1 alpha/mipmap behavior differs.
- Follow-up checks: Test rectangular/transparent/animated and resource cleanup.
- RustChunk implementation mapping: Offline mip golden tests.
- Evidence required before adoption: Require target mip bytes and GPU sampling.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3487. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMobAiOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: e959bc7b1b906cffccd8ded097c7c290be243edf
- Size: 72767 bytes
- Coverage: Read through EOF: lines 1–300, 301–500, 501–700, 701–900, 901–1100 and 1101–1206 (1206 logical lines).
- Purpose: Headless server mob Brain/GoalSelector/navigation/gossip/bee AI.
- Dependencies / cross-references: Real Mob/Brain/GoalSelector, ServerLevel, registries, paths and AI callbacks.
- Implementation / logic: 1206 lines in six ranges 1–300,301–500,501–700,701–900,901–1100,1101–1206; fixture world/time/light/biome; neutral finalizeSpawn RNG; Unsafe.
- Findings: AI behavior and tick test oracle.
- RustChunk relevance: Highly relevant to server-side mob behavior and tick simulation.
- Decision: adaptable; reference; parity-test-oracle; version-specific
- 1.20.1 implications: Current mob AI differs from 1.20.1; server authoritative.
- Follow-up checks: Compare deterministic goals/memories/pathfinding; profile actual server separately.
- RustChunk implementation mapping: Target mob AI traces/profile.
- Evidence required before adoption: Require 1.20.1 tick goldens and server data.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3488. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMobAttributesOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: c9d4362c40fd6b04d9b0886e11bcb65234ade817
- Size: 5847 bytes
- Coverage: Complete EOF coverage (97 logical lines).
- Purpose: Export entity dimensions, eye height, attributes, baby size and XP.
- Dependencies / cross-references: EntityType/Attribute registries, actual constructors on detached server fixture.
- Implementation / logic: Unsafe detached level, type-specific creation errors recorded; modern entity catalog.
- Findings: Entity metadata exporter.
- RustChunk relevance: Useful to regenerate entity metadata and collision/render bounds.
- Decision: adaptable; reference; parity-data-exporter
- 1.20.1 implications: Regenerate from 1.20.1 Forge including mods.
- Follow-up checks: Audit completeness, baby/adult, hitboxes and defaults.
- RustChunk implementation mapping: Target entity catalog.
- Evidence required before adoption: Require full 1.20.1 export.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3489. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMobDataExporter.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 2163d8d3c91f24d32da2f3296c009338b065051c
- Size: 9950 bytes
- Coverage: Complete EOF coverage (197 logical lines).
- Purpose: Export block pathfinding data, spawn placement and entity animation frames.
- Dependencies / cross-references: WalkNodeEvaluator, SpawnPlacements, registries and client JAR animation classes.
- Implementation / logic: Private reflection; exact client JAR needed; current DataVersion.
- Findings: Pathfinding/spawn/animation data.
- RustChunk relevance: Useful for spawn placement and mob navigation data.
- Decision: adaptable; reference; parity-data-exporter
- 1.20.1 implications: Regenerate for target 1.20.1 Forge/JAR.
- Follow-up checks: Verify JAR version, tags, entity coverage and clips.
- RustChunk implementation mapping: Target data exporters.
- Evidence required before adoption: Require version-locked data and playback tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3490. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaModelBakingOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 96ecc8b46d5209765f76701ddcd9b405fb869da5
- Size: 7034 bytes
- Coverage: Complete EOF coverage (106 logical lines).
- Purpose: Cuboid model baking and quad vertex/UV outputs.
- Dependencies / cross-references: ModelDiscovery/ModelBaker, FaceBakery and synthetic atlas.
- Implementation / logic: Fixture models/sprite; no resource pack, GPU or raster.
- Findings: Block/item mesh geometry.
- RustChunk relevance: Highly relevant to mesh geometry and texture coordinates.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Map to 1.20.1 Forge model loaders.
- Follow-up checks: Test rotations, winding, UV lock, tint and missing texture.
- RustChunk implementation mapping: Mesh-baker geometry oracle.
- Evidence required before adoption: Require target assets/pixel comparisons.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3491. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaModelChoiceOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 4cf8cb3297c7308dc7469f86f4d983c8225f9da3
- Size: 4087 bytes
- Coverage: Complete EOF coverage (65 logical lines).
- Purpose: WeightedVariants and MultiPartModel choice.
- Dependencies / cross-references: Variant/multipart model classes and seeded RandomSource.
- Implementation / logic: Dummy geometry leaves and limited properties; no rendering.
- Findings: Deterministic model choice.
- RustChunk relevance: Useful for deterministic blockstate model selection.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Target blockstate JSON/seed rules govern.
- Follow-up checks: Test weights, predicates and missing properties.
- RustChunk implementation mapping: Target resolver cases.
- Evidence required before adoption: Require target JSON goldens.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3492. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaModelResolutionOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: b198a1882d79dd76a32ea68c01f27f5af7794ef1
- Size: 4030 bytes
- Coverage: Complete EOF coverage (63 logical lines).
- Purpose: ModelDiscovery parent/texture slot/display transform resolution.
- Dependencies / cross-references: ResolvedModel, TextureSlots and ItemDisplayContext.
- Implementation / logic: Synthetic model graph only; no actual resource manager/atlas.
- Findings: Resource graph reference.
- RustChunk relevance: Useful for model resource graph and display transforms.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: 1.20.1 BlockModel/ModelBakery differs.
- Follow-up checks: Test parent cycle, missing slots, inheritance and context.
- RustChunk implementation mapping: Rust resource resolver tests.
- Evidence required before adoption: Require 1.20.1 assets.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3493. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMovementEmissionOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 6ad610cc7278bd9371cdc4cb32afe3ba77b46feb
- Size: 7192 bytes
- Coverage: Complete EOF coverage (68 logical lines).
- Purpose: Movement sound, step and vibration callback order.
- Dependencies / cross-references: Entity movement-emission helpers, BlockState, Level.
- Implementation / logic: Unsafe entity and fake world; no full tick/audio playback.
- Findings: Footstep/surface events.
- RustChunk relevance: Useful for entity movement feedback and sound events.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Recheck 1.20.1 and Forge sound/event hooks.
- Follow-up checks: Test silent block, threshold, water, climb and event order.
- RustChunk implementation mapping: Movement/audio test vectors.
- Evidence required before adoption: Require target traces and sound tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3494. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMovementRegistryOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: d25ff5d3657d431fead0bb2eed51e74c1a33be8c
- Size: 7348 bytes
- Coverage: Complete EOF coverage (61 logical lines).
- Purpose: Export friction/speed/jump/bounce/fall state properties.
- Dependencies / cross-references: BlockState registry, NBT codec and block/fluid tags.
- Implementation / logic: Global tag bind; current IDs/DataVersion; no full physics.
- Findings: Movement coefficient catalog.
- RustChunk relevance: Useful for surface-specific movement physics.
- Decision: adaptable; reference; parity-data-exporter; parity-test-oracle
- 1.20.1 implications: Regenerate with 1.20.1 blocks/mods.
- Follow-up checks: Check every state and contextual override.
- RustChunk implementation mapping: Target physics exporter.
- Evidence required before adoption: Require 1.20.1 registry tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3495. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMovementSupportOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: fe55429c214b8d3b969f38ae8a92e6cefad7de87
- Size: 5316 bytes
- Coverage: Complete EOF coverage (61 logical lines).
- Purpose: Entity support position and jump-factor queries.
- Dependencies / cross-references: Entity getOnPos/movementPos/jump factor, fake Level/BlockState.
- Implementation / logic: Private AABB/position and scripted world; not collision resolution.
- Findings: Feet/support coordinate math.
- RustChunk relevance: Useful for support-block detection and movement coordinates.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Compare 1.20.1 rounding and bounds.
- Follow-up checks: Test negative/border coords, slabs, fences, missing support.
- RustChunk implementation mapping: Physics coordinate tests.
- Evidence required before adoption: Require target integration.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3496. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMover.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 0b0d0ee14682472f9ceca3ddc261974ee041c98e
- Size: 3443 bytes
- Coverage: Complete EOF coverage (77 logical lines).
- Purpose: Interactive desktop vanilla client input and position driver.
- Dependencies / cross-references: Minecraft main, client executor, LocalPlayer and stdin commands.
- Implementation / logic: Starts live desktop client and sends movement/look/attack commands; not run, not production launcher.
- Findings: Manual vanilla baseline.
- RustChunk relevance: Useful as a desktop baseline input harness, not code to integrate.
- Decision: reference; runtime-driver; parity-test-oracle
- 1.20.1 implications: Not portable to RustChunk Android.
- Follow-up checks: Manual controlled 1.20.1 baseline; check server correction.
- RustChunk implementation mapping: Keep as optional desktop test tool.
- Evidence required before adoption: No execution; require manual run.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3497. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaMusicOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 19af845e50e2eb2a632a203bccdabbcc95890534
- Size: 6765 bytes
- Coverage: Complete EOF coverage (145 logical lines).
- Purpose: MusicManager/SoundEngine transitions.
- Dependencies / cross-references: MusicManager, SoundEngine, MusicInfo and test sound rig.
- Implementation / logic: Real manager on scripted rig; no Android speaker, focus or latency tests.
- Findings: Background music policy.
- RustChunk relevance: Useful for music scheduling behavior and audio lifecycle.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Map to 1.20.1 sound APIs; Android audio separate.
- Follow-up checks: Test pause, volume, end, delay and teardown.
- RustChunk implementation mapping: Rust audio state tests.
- Evidence required before adoption: Require 1.20.1 and device audio.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3498. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaNaturalSpawnOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: 8bb0129f28956f021f1bd8050ac46c9917b97dd0
- Size: 59328 bytes
- Coverage: Read through EOF: lines 1–220, 221–440, 441–660 and 661–889 (889 logical lines).
- Purpose: NaturalSpawner and spawner/trial/vault native ticks.
- Dependencies / cross-references: NaturalSpawner, SpawnState, ServerLevel, chunk map, spawner, trial/vault and registries.
- Implementation / logic: 889 lines in 4 ranges 1–220,221–440,441–660,661–889; controlled blocks/biomes/players/chunks; neutral finalizeSpawn RNG; no normal scheduler/network.
- Findings: Strong server spawn/mob-cap oracle blueprint.
- RustChunk relevance: Highly relevant to authoritative mob spawning and per-chunk caps.
- Decision: adaptable; reference; parity-test-oracle; version-specific
- 1.20.1 implications: Recreate NaturalSpawner for 1.20.1; trial/vault later-only systems not equivalents.
- Follow-up checks: Compare caps, player radius, placement, structures, spawner NBT and RNG.
- RustChunk implementation mapping: Target Forge server spawn tests.
- Evidence required before adoption: Require 1.20.1 spawn traces and real server metrics.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3499. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaNoiseOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: cf0cfb7d903c3fd96a8908dab1b961b4b204737f
- Size: 11817 bytes
- Coverage: Complete EOF coverage (149 logical lines).
- Purpose: Seeded Perlin/Simplex/Smeared/Normal noise and density raw bits.
- Dependencies / cross-references: Noise classes, RandomSource, DensityFunction and buffers.
- Implementation / logic: 149 lines; command parser/volume limits need hardening; current noise/RNG.
- Findings: Terrain determinism.
- RustChunk relevance: Highly relevant to deterministic terrain noise and density.
- Decision: adaptable; reference; parity-test-oracle
- 1.20.1 implications: Target noise and random implementations may differ.
- Follow-up checks: Test negative coords, NaN, signed zero, volume and cache.
- RustChunk implementation mapping: Target noise oracle design.
- Evidence required before adoption: Require target raw-bit outputs.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
### 3500. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaObjectOpenMapOracle.java`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: af075584f0d2df542383f107bac9674e1ee8c251
- Size: 5759 bytes
- Coverage: Complete EOF coverage (96 logical lines).
- Purpose: fastutil Object2ObjectOpenHashMap mutation/iteration/live entries.
- Dependencies / cross-references: fastutil map, custom key equals/hashCode, iterator and Map.Entry.
- Implementation / logic: Actual fastutil collection; order/probing dependency-specific; key equality is value-based not identity.
- Findings: Collection compatibility reference.
- RustChunk relevance: Only useful where exact fastutil map semantics are observable.
- Decision: reference; runtime-specific; parity-test-oracle
- 1.20.1 implications: Do not assume Rust HashMap matches fastutil behavior.
- Follow-up checks: Check dependency version, collisions, resize and entry mutation.
- RustChunk implementation mapping: Only port exposed semantic requirements.
- Evidence required before adoption: Require dependency/runtime tests.
- Completion state: COMPLETE — pinned SHA/size verified; full stated coverage read through EOF; static inspection only; not executed.
