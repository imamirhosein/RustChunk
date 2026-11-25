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
