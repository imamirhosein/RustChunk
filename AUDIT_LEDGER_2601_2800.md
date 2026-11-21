# RustChunk Starling Audit Ledger — Entries 2601–2800

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active 200-file stage: **2601–2800**; completed **25/200 (2601–2625)**.
- One-time exceptional authorization: **2601–2700 only**, two 50-file sections and four 25-file checkpoints.
- Source selection: strict raw lexical path order from the complete pinned tree, excluding every path already in the full Index. The tree is untruncated; all 25 records match pinned paths, SHAs and byte sizes.
- Starling is a Java Edition 26.3-era reference snapshot, not proof of Java 1.20.1 / Forge 47.x compatibility.
- Stop after entry **2700**. Entries 2701+ are not authorized. No source file was modified; tests/examples were inspected only, not executed; no build/runtime/device/Forge/Aternos validation was performed.

## 50-file section 2601–2650

### Checkpoint 2601–2625 — COMPLETE

### 2601. `crates/horizon-world/src/use_remainder.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `357896b19b714da57936ff8534f7efba2b323906`
- Size: 1387 bytes
- Coverage: Complete through EOF: 40 logical lines; full file; no gaps.
- Purpose: Stack identity and count-based remainder creation.
- Dependencies / cross-references: StackRef; convert/convert_with; create/extra callbacks; shared world.
- Implementation / logic: Skips infinite materials and unchanged/increased counts; creates remainder after decrease; returns it if original empties, else calls extra.
- Findings: Callback can fail after creation; caller owns rollback/insertion.
- RustChunk relevance: Adaptable item-use remainder reference.
- Decision: adaptable; reference; target ItemStack/use-hook parity required
- 1.20.1 implications: Verify 1.20.1 pre-component ItemStack count and Forge use-hook identity.
- Follow-up checks: Compare empty/unchanged/decreased/increased/infinite cases and callback failure.
- RustChunk implementation mapping: ItemStack identity, item-use and inventory insertion.
- Evidence required before adoption: 1.20.1 ItemStack source, Forge 47 hooks and differential count/identity fixtures.
- Completion state: COMPLETE — 40 logical lines through EOF; pinned source SHA and 1387 byte size match; inspected, not modified; tests not executed.

### 2602. `crates/horizon-world/src/view_direction.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d33a142890865faeec8e59ef19aab2f97f3e711f`
- Size: 1105 bytes
- Coverage: Complete through EOF: 24 logical lines; full file; no gaps.
- Purpose: Pitch/yaw view vector and horizontal blocking-angle math.
- Dependencies / cross-references: Vec3/trig; libm acos; float degree conversion; head yaw.
- Implementation / logic: Builds view vector, projects source direction horizontally and computes acos; defines widened-f32-PI fallback.
- Findings: Sign and numeric widths explicit; zero horizontal delta and acos edges need target checks.
- RustChunk relevance: Adaptable combat orientation math.
- Decision: adaptable; reference; JVM trig and acos parity required
- 1.20.1 implications: Verify 1.20.1 Entity view vector and blocking-angle conversions.
- Follow-up checks: Compare cardinal angles, yaw wrap, zero delta and NaN/range edges.
- RustChunk implementation mapping: Core geometry/trig and combat orientation.
- Evidence required before adoption: 1.20.1 JVM source/bytecode and deterministic numeric fixtures on Android ABIs.
- Completion state: COMPLETE — 24 logical lines through EOF; pinned source SHA and 1105 byte size match; inspected, not modified; tests not executed.

### 2603. `crates/horizon-world/src/water_splash.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fd6b023bdd03b4c5dddd8341b72f6a1cba5528b1`
- Size: 4069 bytes
- Coverage: Complete through EOF: 115 logical lines; full file; no gaps.
- Purpose: Water-entry sound, bubble/splash particles and game event.
- Dependencies / cross-references: WaterSplashWorld; passenger/entity; velocity; sound; RandomSource; particle budget.
- Implementation / logic: Selects passenger velocity, computes volume/category, draws pitch, emits particles and dispatches event.
- Findings: Callback/RNG order observable; failure may follow already emitted effects.
- RustChunk relevance: Adaptable water-entry effects reference.
- Decision: adaptable; reference; random/event/particle parity required
- 1.20.1 implications: Verify 1.20.1 fluid transition, sound, particle, event and Forge hooks.
- Follow-up checks: Trace thresholds, pitch/particle RNG draws, order, loop bound and failures.
- RustChunk implementation mapping: Entity fluid transition with compatible RNG/event/sound/particle interface.
- Evidence required before adoption: 1.20.1 source, Forge 47 events and seeded differential traces.
- Completion state: COMPLETE — 115 logical lines through EOF; pinned source SHA and 4069 byte size match; inspected, not modified; tests not executed.

### 2604. `crates/horizon-world/src/world_movement.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ed247f44bf5fa272bdbabfd4ea3c3e3f75eae715`
- Size: 8725 bytes
- Coverage: Complete through EOF: 242 logical lines; full file; no gaps.
- Purpose: Block/entity/border collision aggregation, step resolution and ground reach.
- Dependencies / cross-references: block_collision_scan; BlockCollisionWorld; EntityMovementWorld; CollisionContext; AABB/VoxelShape.
- Implementation / logic: Collects entity shapes, scans blocks/border for step resolution, checks no-collision and approximates ground reach.
- Findings: Entity shapes captured once; border queried per bounds. Ground reach excludes entity colliders; scan errors fail closed.
- RustChunk relevance: Core collision architecture reference.
- Decision: rewrite required; reference; rebuild 1.20.1 collision and step pipeline
- 1.20.1 implications: Reconstruct 1.20.1 Entity.collide, CollisionGetter.noCollision and ServerPlayerGameMode ground checks plus Forge overrides.
- Follow-up checks: Compare axis order, entity selection, border, step choices, zero movement and ground reach.
- RustChunk implementation mapping: World collision service over chunks, entity query and border.
- Evidence required before adoption: 1.20.1 Mojang-mapped sources, Forge 47 patches and shape-query traces.
- Completion state: COMPLETE — 242 logical lines through EOF; pinned source SHA and 8725 byte size match; inspected, not modified; tests not executed.

### 2605. `crates/horizon-world/src/world_ray.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `302f043d6b09562f2c8210226de77718a0a383bd`
- Size: 6664 bytes
- Coverage: Complete through EOF: 194 logical lines; full file; no gaps.
- Purpose: Finite block/fluid ray clipping and fall-reset shape selection.
- Dependencies / cross-references: RayWorld; block/fluid state and shapes; modes; traversal; BlockHit; portal/player context.
- Implementation / logic: Traverses blocks under budget, compares block/fluid hits, adjusts interaction face and models reset-tag/portal shapes.
- Findings: Query order fixed; errors are not fabricated misses; reset logic depends on player/server/cooldown.
- RustChunk relevance: Raycast and portal-reset reference.
- Decision: rewrite required; reference; rebuild target ray clipping and portal reset
- 1.20.1 implications: Rebuild 1.20.1 ClipContext, VoxelShape, BlockGetter and fall-reset behavior.
- Follow-up checks: Compare modes, tie-breaks, miss face, traversal, interaction face and Forge hooks.
- RustChunk implementation mapping: Registry-backed raycast plus portal policy.
- Evidence required before adoption: 1.20.1 mappings, Forge patches, registry fixtures and ray differential traces.
- Completion state: COMPLETE — 194 logical lines through EOF; pinned source SHA and 6664 byte size match; inspected, not modified; tests not executed.

### 2606. `crates/horizon-world/src/worldgen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `109293a695b7f3bc8ecc7fc996ea29f19dc7c13a`
- Size: 9635 bytes
- Coverage: Complete through EOF: 301 logical lines; full file; no gaps.
- Purpose: Worldgen module surface and JSON/noise helpers for Java 26.3.
- Dependencies / cross-references: Worldgen modules; serde_json RawValue; horizon_core noise/density; NormalNoise.
- Implementation / logic: Declares modules and strict numeric/noise decoder, including decimal/exponent narrowing.
- Findings: Noise codec targets 26.3; terrain generation incomplete; schema is version-specific.
- RustChunk relevance: Subsystem map, not drop-in engine.
- Decision: rewrite required; version-specific; codec is Java 26.3
- 1.20.1 implications: Rebuild 1.20.1 NoiseSettings, DensityFunction and NoiseRouter codecs from target resources.
- Follow-up checks: Map modules to target classes; verify Gson numeric conversion and codecs.
- RustChunk implementation mapping: Use only as subsystem map; build target codec registry separately.
- Evidence required before adoption: 1.20.1 sources, vanilla worldgen JSON, Forge registries and codec fixtures.
- Completion state: COMPLETE — 301 logical lines through EOF; pinned source SHA and 9635 byte size match; inspected, not modified; tests not executed.

### 2607. `crates/horizon-world/src/worldgen/anchor.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `741a57dd059bb203f820cb7bf7f34796d1437035`
- Size: 3611 bytes
- Coverage: Complete through EOF: 102 logical lines; full file; no gaps.
- Purpose: Vertical generation anchors and worldgen Y context.
- Dependencies / cross-references: VerticalAnchor; WorldGenerationContext; JSON helpers; minY/height/sea level.
- Implementation / logic: Implements absolute, bottom/top/sea-relative anchors with wrapping arithmetic and bounded JSON alternatives.
- Findings: Runtime offsets unrestricted while codec bounds them; context combines minima/heights independently.
- RustChunk relevance: Adaptable utility reference.
- Decision: adaptable; reference; verify target anchor bounds and context
- 1.20.1 implications: Check 1.20.1 VerticalAnchor and context for standard/custom dimensions.
- Follow-up checks: Compare constants, overflow, alternative precedence and minY/height.
- RustChunk implementation mapping: Versioned worldgen vertical context and anchor codec.
- Evidence required before adoption: 1.20.1 sources, dimension JSON and Forge dimension tests.
- Completion state: COMPLETE — 102 logical lines through EOF; pinned source SHA and 3611 byte size match; inspected, not modified; tests not executed.

### 2608. `crates/horizon-world/src/worldgen/aquifer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `44500313b68dad4b94aa878bd9ec3808bf129037`
- Size: 2772 bytes
- Coverage: Complete through EOF: 95 logical lines; full file; no gaps.
- Purpose: Global fluid picker, disabled aquifer and FluidStatus.
- Dependencies / cross-references: BlockStateId; FluidPicker; GlobalFluidPicker; DisabledAquifer; density; air/lava.
- Implementation / logic: Selects fluid below level; picker chooses air/lava/default; disabled aquifer handles non-positive/NaN density.
- Findings: Hard-coded modern threshold and external registry IDs; target state identity matters.
- RustChunk relevance: Separation of global fluid policy.
- Decision: rewrite required; version-specific; target aquifer global-fluid behavior
- 1.20.1 implications: Rebuild 1.20.1 sea/lava levels, default fluid and dimension registry rules.
- Follow-up checks: Check thresholds, NaN, state-vs-block equality and codec.
- RustChunk implementation mapping: Target-version FluidPicker/Aquifer over immutable BlockState IDs.
- Evidence required before adoption: 1.20.1 Aquifer/NoiseBasedChunkGenerator sources and dimension fixtures.
- Completion state: COMPLETE — 95 logical lines through EOF; pinned source SHA and 2772 byte size match; inspected, not modified; tests not executed.

### 2609. `crates/horizon-world/src/worldgen/aquifer/config.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `984fe59e8bfb373fe793b93cf3594a8e2dc1936b`
- Size: 4521 bytes
- Coverage: Complete through EOF: 127 logical lines; full file; no gaps.
- Purpose: Six aquifer density roots and compiled sampler binding.
- Dependencies / cross-references: AquiferField; DensityRegistry; DensityCompiler; FunctionId; SamplerContext.
- Implementation / logic: Decodes required roots atomically and binds six samplers in fixed order through shared compiler cache.
- Findings: Root names/count and cache namespace may differ from target.
- RustChunk relevance: Adaptable registry binding, not target codec authority.
- Decision: rewrite required; version-specific; target aquifer roots differ
- 1.20.1 implications: Compare exact 1.20.1 NoiseRouter/Aquifer roots, names, defaults and registration.
- Follow-up checks: Verify fields, aliases, atomic failure and sampler reuse.
- RustChunk implementation mapping: Target density graph and worker-local sampler bundle.
- Evidence required before adoption: 1.20.1 NoiseRouter codec, built-in settings and graph fixtures.
- Completion state: COMPLETE — 127 logical lines through EOF; pinned source SHA and 4521 byte size match; inspected, not modified; tests not executed.

### 2610. `crates/horizon-world/src/worldgen/aquifer/noise_based.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d2b83c3665c8a25e49a9501b6564c38ca3b8062f`
- Size: 23530 bytes
- Coverage: Complete through EOF: 642 logical lines; full file; no gaps.
- Purpose: Stateful noise-based aquifer grid, pressure and fluid scheduling for 26.3.
- Dependencies / cross-references: AquiferSamplers; FluidPicker; PositionalRandom; DensityVolume; BlockRegistry; SamplerContext.
- Implementation / logic: Builds bounded cells, caches surface/status, derives positional-random points and evaluates floodedness, spread, lava and pressure.
- Findings: Neighborhood order, equal-distance insertion and RNG draws affect output; version-marked 26.3.
- RustChunk relevance: Algorithm reference only.
- Decision: rewrite required; version-specific; rebuild 1.20.1 aquifer and RNG
- 1.20.1 implications: Compare target grid, roots, random factory, pressure, fluid rules and scheduling.
- Follow-up checks: Seeded traces for fluid result, flag, RNG, caches, overflow and grid edges.
- RustChunk implementation mapping: Independent 1.20.1 Aquifer with target samplers and positional RNG.
- Evidence required before adoption: 1.20.1 Aquifer sources/settings, Forge fluid behavior and differential traces.
- Completion state: COMPLETE — 642 logical lines through EOF; pinned source SHA and 23530 byte size match; inspected, not modified; tests not executed.

### 2611. `crates/horizon-world/src/worldgen/bamboo.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ca63d9d85dfb9ef33ea97157155a3562dafa1571`
- Size: 5970 bytes
- Coverage: Complete through EOF: 156 logical lines; full file; no gaps.
- Purpose: Bamboo configured feature with podzol patch, trunk and caps.
- Dependencies / cross-references: FeatureBlockBindings; FeatureWorld; PlacementContext; Heightmap; RNG; block/tag registry.
- Implementation / logic: Checks origin survival, samples height and optional patch radius, scans podzol area, places trunk and ordered caps.
- Findings: RNG branch-dependent; writes can partially commit; replaceable tag captured at bind time.
- RustChunk relevance: Adaptable configured-feature reference.
- Decision: adaptable; reference; verify config, tags, RNG and writes
- 1.20.1 implications: Verify 1.20.1 BambooFeature config, state properties, podzol tag, heightmap and flags.
- Follow-up checks: Compare codec, draws, scan order, survival, cap sequence and failure.
- RustChunk implementation mapping: Version-bound feature with registry snapshot and chunk writes.
- Evidence required before adoption: 1.20.1 BambooFeature/BambooBlock, feature JSON and seeded traces.
- Completion state: COMPLETE — 156 logical lines through EOF; pinned source SHA and 5970 byte size match; inspected, not modified; tests not executed.

### 2612. `crates/horizon-world/src/worldgen/biome.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7847d679ba15de8d304d08213fc3a15fa8414598`
- Size: 5105 bytes
- Coverage: Complete through EOF: 153 logical lines; full file; no gaps.
- Purpose: Imported multi-noise biome index and worker resolver.
- Dependencies / cross-references: ClimateIndex; ClimateSampler; ClimateSearch; ParameterPoint; biome IDs; SamplerContext.
- Implementation / logic: Builds parameter-point index from custom quantized JSON and resolves samples, optionally preparing climate volume.
- Findings: Import format is Horizon-specific, not vanilla codec; preset builders/tables separate.
- RustChunk relevance: Adaptable spatial index architecture.
- Decision: adaptable; reference; regenerate target multi-noise tables/codecs
- 1.20.1 implications: Rebuild 1.20.1 quantization, ties, presets, holder identity and built-in tables.
- Follow-up checks: Check coordinate order/scale, offsets, ties, provenance and worker lifetime.
- RustChunk implementation mapping: Biome registry, target climate table, codec and per-worker resolver.
- Evidence required before adoption: 1.20.1 Climate/MultiNoiseBiomeSource sources and lookup fixtures.
- Completion state: COMPLETE — 153 logical lines through EOF; pinned source SHA and 5105 byte size match; inspected, not modified; tests not executed.

### 2613. `crates/horizon-world/src/worldgen/biome_chunk.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7654e06f73c01a11dddd2e70c7a90bb070b503cf`
- Size: 8022 bytes
- Coverage: Complete through EOF: 214 logical lines; full file; no gaps.
- Purpose: Quart biome chunk storage, atomic section fill, zoom and retrogen.
- Dependencies / cross-references: Heightmap; BiomeZoom; ChunkPos/BlockPos; RetrogenBiomeResolver; resolver callback.
- Implementation / logic: Stores 4x4x4 biome cells per section, clamps quart Y, commits filled sections atomically and resolves block biome via zoom.
- Findings: Failed section retains prior values; earlier sections committed. Registry order not used for present-biome sorting.
- RustChunk relevance: Adaptable storage/atomic fill design.
- Decision: adaptable; reference; verify target quart storage, sections and retrogen
- 1.20.1 implications: Rebuild 1.20.1 biome palette, heights, holder serialization, blending and retrogen.
- Follow-up checks: Compare indexing, negative Y, fill order, zoom, errors and save/network codec.
- RustChunk implementation mapping: ChunkSection biome data and region/retrogen resolver.
- Evidence required before adoption: 1.20.1 LevelChunkSection/PalettedContainer/BiomeManager/BelowZeroRetrogen and fixtures.
- Completion state: COMPLETE — 214 logical lines through EOF; pinned source SHA and 8022 byte size match; inspected, not modified; tests not executed.

### 2614. `crates/horizon-world/src/worldgen/biome_predicate.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `46554e1a9f274c2ee35c0cfeba573f99f8ec553e`
- Size: 3096 bytes
- Coverage: Complete through EOF: 81 logical lines; full file; no gaps.
- Purpose: Biome names/tags resolved to immutable holder-ID sets.
- Dependencies / cross-references: BiomeId; identifier; BiomePredicateBindings; BTreeMap/BTreeSet/Arc.
- Implementation / logic: Validates registry and tags, canonicalizes names and resolves singleton/tag/list into ID sets.
- Findings: Rejects duplicates/unknown members; IDs need same immutable snapshot as queries.
- RustChunk relevance: Adaptable registry-binding pattern.
- Decision: adaptable; reference; bind target biome tags and identity
- 1.20.1 implications: Bind Forge biome tags/reloads to versioned snapshot; confirm target holder codec.
- Follow-up checks: Check tag syntax, duplicates, unknowns, reload invalidation and identity mismatch.
- RustChunk implementation mapping: Biome registry snapshot and placed-feature predicate resolver.
- Evidence required before adoption: 1.20.1 biome holder codec and Forge tag lifecycle tests.
- Completion state: COMPLETE — 81 logical lines through EOF; pinned source SHA and 3096 byte size match; inspected, not modified; tests not executed.

### 2615. `crates/horizon-world/src/worldgen/biome_source.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bb2cf8990191ad571df49873842fee2eab135703`
- Size: 3043 bytes
- Coverage: Complete through EOF: 92 logical lines; full file; no gaps.
- Purpose: Fixed, checkerboard and End biome-source sampling.
- Dependencies / cross-references: FixedBiomeSource; CheckerboardBiomeSource; EndBiomeSource; DensitySampler; SamplerContext.
- Implementation / logic: Returns fixed biome, indexes checkerboard via masked shifts/modulo or samples End erosion outside center circle.
- Findings: Codec limits are separate; End sample uses chunk center and wrapped quart Y.
- RustChunk relevance: Adaptable source algorithm.
- Decision: adaptable; version-specific; verify target fixed/checkerboard/End behavior
- 1.20.1 implications: Check 1.20.1 scale, End radius/sample position, biome IDs and negative coordinates.
- Follow-up checks: Compare extremes, shift masking, circle boundary and erosion thresholds.
- RustChunk implementation mapping: Version-bound biome source with target registry handles.
- Evidence required before adoption: 1.20.1 Fixed/Checkerboard/End sources and deterministic fixtures.
- Completion state: COMPLETE — 92 logical lines through EOF; pinned source SHA and 3043 byte size match; inspected, not modified; tests not executed.

### 2616. `crates/horizon-world/src/worldgen/biome_source/codec.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ab78f4588ee1f08b4c26bfd440ec38e6e708a59e`
- Size: 10245 bytes
- Coverage: Complete through EOF: 262 logical lines; full file; no gaps.
- Purpose: Holder-aware biome-source codecs for variants and presets.
- Dependencies / cross-references: BiomeSourceCatalog; BiomeSet; ParameterPoint; biome/tag/preset registries; RawValue.
- Implementation / logic: Validates IDs/tags/presets, decodes source and inline multi-noise or preset alternatives, emits canonical JSON.
- Findings: Inline decode before preset fallback; full inline biome codec separate; precedence version-sensitive.
- RustChunk relevance: Adaptable codec composition.
- Decision: adaptable; reference; verify target holder codec and preset JSON
- 1.20.1 implications: 1.20.1 holder codecs, inline entries, tags and preset formats differ.
- Follow-up checks: Check alternatives, limits, unknown holders, fallback and round-trip errors.
- RustChunk implementation mapping: Version-specific biome source codec registry.
- Evidence required before adoption: 1.20.1 codecs, preset data, Forge registry fixtures and round-trip tests.
- Completion state: COMPLETE — 262 logical lines through EOF; pinned source SHA and 10245 byte size match; inspected, not modified; tests not executed.

### 2617. `crates/horizon-world/src/worldgen/biome_source/runtime.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5d78fd835e80308f3b58e7524134fa3f8cae51ef`
- Size: 6154 bytes
- Coverage: Complete through EOF: 170 logical lines; full file; no gaps.
- Purpose: Biome-source resolution and per-worker climate state.
- Dependencies / cross-references: BiomeSourceDefinition; source variants; climate/erosion; quantized presets; SamplerContext.
- Implementation / logic: Resolves definitions using external presets, exposes possible biomes and constructs source resolvers; multi-noise can prepare volume.
- Findings: Presets external rather than vanilla builders; worker lifetime preserves search tie behavior.
- RustChunk relevance: Adaptable resolver lifecycle.
- Decision: adaptable; reference; authenticate target presets and worker state
- 1.20.1 implications: Recreate 1.20.1 presets, biome IDs, erosion roots and resolver lifetime.
- Follow-up checks: Verify preset digest, dispatch, compiler namespace and scalar/volume equivalence.
- RustChunk implementation mapping: Biome source builder, authenticated preset corpus and worker sampler.
- Evidence required before adoption: 1.20.1 codecs/preset data and deterministic worker fixtures.
- Completion state: COMPLETE — 170 logical lines through EOF; pinned source SHA and 6154 byte size match; inspected, not modified; tests not executed.

### 2618. `crates/horizon-world/src/worldgen/biome_temperature.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7111214f4edd64a81d55edc8f88b9969edd3c2e9`
- Size: 6961 bytes
- Coverage: Complete through EOF: 195 logical lines; full file; no gaps.
- Purpose: Seeded biome temperature, altitude cooling and bounded cache.
- Dependencies / cross-references: LegacyRandom; SimplexNoise/NoiseStack; BiomeTemperatureSettings; BlockPos; map/queue cache.
- Implementation / logic: Builds fixed-seed noises, applies frozen modifier/height cooling and caches positions up to 1,024 entries.
- Findings: Cache omits sea level; NaN recomputes; hits do not reorder; seeds/float narrowing observable.
- RustChunk relevance: Adaptable temperature/cache reference.
- Decision: adaptable; reference; verify target temperature noise and float cache
- 1.20.1 implications: Compare 1.20.1 seeds, noise, altitude adjustment, thresholds and thread-local cache.
- Follow-up checks: Test frozen coordinates, rounding, altitude, NaN, eviction and sea-level key.
- RustChunk implementation mapping: Per-worker temperature cache and legacy RNG/noise.
- Evidence required before adoption: 1.20.1 Biome/TemperatureModifier/PerlinSimplexNoise and Java float fixtures.
- Completion state: COMPLETE — 195 logical lines through EOF; pinned source SHA and 6961 byte size match; inspected, not modified; tests not executed.

### 2619. `crates/horizon-world/src/worldgen/biome_weather.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7c58b4d23470bc86d3faeaa39029f1a0c8bbdae6`
- Size: 3347 bytes
- Coverage: Complete through EOF: 94 logical lines; full file; no gaps.
- Purpose: Registry-bound water-freezing and snow-placement checks.
- Dependencies / cross-references: BiomeTemperature; Precipitation; FeatureBlockBindings; FluidKind; BlockStateId; PlacementContext.
- Implementation / logic: Captures liquid/snow/air states and checks temperature, height, brightness, fluid, water edge and snow survival.
- Findings: Needs matching fluid kind and block state; edge test checks four neighbors; registry snapshot must match queries.
- RustChunk relevance: Adaptable environmental placement adapter.
- Decision: adaptable; reference; bind target fluids, snow, height and Forge hooks
- 1.20.1 implications: Rebuild 1.20.1 Biome.shouldFreeze/shouldSnow, SnowLayerBlock and Forge hooks.
- Follow-up checks: Compare brightness, query order, fluid identity, edge direction, height and survival.
- RustChunk implementation mapping: Weather placement linked to target temperature and registries.
- Evidence required before adoption: 1.20.1 weather sources, Forge 47 hooks and seeded fixtures.
- Completion state: COMPLETE — 94 logical lines through EOF; pinned source SHA and 3347 byte size match; inspected, not modified; tests not executed.

### 2620. `crates/horizon-world/src/worldgen/blending.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b54e3626c734201a77bdcaeea645c90e7780d6b`
- Size: 36234 bytes
- Coverage: Complete through EOF: 1031 logical lines; full file; no gaps.
- Purpose: Legacy chunk-border blending of height, density, biome, carving and structure starts.
- Dependencies / cross-references: BlendingData; OldChunk/OldWorld; DensityBlender; BiomeId/BiomeChunk; Noise/RNG; fastutil map.
- Implementation / logic: Extracts old border columns, caches samples, blends height/density/biomes and computes carving filter.
- Findings: Column/neighbour/map iteration affects floats; reads saved old chunk data, not regenerated terrain.
- RustChunk relevance: Strong upgrade architecture reference; target data needs rebuild.
- Decision: adaptable; reference; rebuild blending against 1.20.1 saved chunks
- 1.20.1 implications: Verify target BlendingData NBT, DataVersion, ranges, density cells, biome blending and structure starts.
- Follow-up checks: Compare corner fixtures, missing heightmaps, arrays, float sums and carving distance.
- RustChunk implementation mapping: Chunk upgrade/blender with serializer, old chunk, density, biome and carving.
- Evidence required before adoption: 1.20.1 Blender/BlendingData, pre-1.18 region fixtures and Java golden outputs.
- Completion state: COMPLETE — 1031 logical lines through EOF; pinned source SHA and 36234 byte size match; inspected, not modified; tests not executed.

### 2621. `crates/horizon-world/src/worldgen/block_blob.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ad866b8d55c01fdb764bbbbe004c7b6624af0427`
- Size: 5273 bytes
- Coverage: Complete through EOF: 140 logical lines; full file; no gaps.
- Purpose: Descending support search and randomized three-stage block blob.
- Dependencies / cross-references: FeatureBlockBindings; BlockPredicate; FeatureWorld; PlacementContext; RNG; BlockStateId.
- Implementation / logic: Finds allowed support, performs three randomized extents and places blocks within computed radius.
- Findings: Draws per radius and post-pass drift; work bounds and wrapping matter.
- RustChunk relevance: Adaptable configured-feature reference.
- Decision: adaptable; reference; verify blob RNG and update flags
- 1.20.1 implications: Compare 1.20.1 BlockBlobFeature descent, bounds, predicate and write flags.
- Follow-up checks: Trace RNG calls, support boundary, negative positions, traversal and failures.
- RustChunk implementation mapping: PlacedFeature with target predicate/RNG/chunk mutation.
- Evidence required before adoption: 1.20.1 BlockBlobFeature/BlockPredicate, JSON and seeded traces.
- Completion state: COMPLETE — 140 logical lines through EOF; pinned source SHA and 5273 byte size match; inspected, not modified; tests not executed.

### 2622. `crates/horizon-world/src/worldgen/block_column.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `72a95c73c86736a790b4022916946733a9453ba4`
- Size: 6495 bytes
- Coverage: Complete through EOF: 186 logical lines; full file; no gaps.
- Purpose: Multi-layer directional column and tip-priority truncation.
- Dependencies / cross-references: IntProvider; BlockStateProvider; BlockPredicate; FeatureBlockBindings; PlacementContext; Direction; RNG.
- Implementation / logic: Samples layer heights, tests allowed prefix, truncates selected end and places provider states in order.
- Findings: Heights sampled before predicate; zero total false; errors can leave partial writes.
- RustChunk relevance: Adaptable feature ordering reference.
- Decision: adaptable; reference; verify target layers, truncation and provider RNG
- 1.20.1 implications: Confirm 1.20.1 BlockColumnFeature codec, truncate policy, direction, RNG and flags.
- Follow-up checks: Test zero/negative/overflow heights, prefix, tip priority and provider draws.
- RustChunk implementation mapping: Configured feature/state provider and deterministic chunk mutation.
- Evidence required before adoption: 1.20.1 BlockColumnFeature, JSON and provider traces.
- Completion state: COMPLETE — 186 logical lines through EOF; pinned source SHA and 6495 byte size match; inspected, not modified; tests not executed.

### 2623. `crates/horizon-world/src/worldgen/block_feature.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `292b4019a7f04544644f2f7e681c5f692a3319d1`
- Size: 19632 bytes
- Coverage: Complete through EOF: 539 logical lines; full file; no gaps.
- Purpose: Registry-bound feature state/tag/property binding plus fill and replacement features.
- Dependencies / cross-references: BlockRegistry/BlockStateJsonCodec; FeatureBlockBindings; BoundTags; properties; biome/fluid predicates; providers.
- Implementation / logic: Builds block/tag maps and lazy tags, enforces registry identity, exposes predicate resolvers and decodes fill/replacement features.
- Findings: Tag names validated before lazy expansion; replacement targets ordered; registry pointer identity enforced.
- RustChunk relevance: Binding architecture; schema needs target/Forge rebuild.
- Decision: rewrite required; reference; bind target block/tag/property/biome/fluid registries
- 1.20.1 implications: Target states, tags, holders and feature serializers differ; Forge reload extensions need design.
- Follow-up checks: Compare laziness, unknown IDs, identity, replacement order, minY and write flags.
- RustChunk implementation mapping: Versioned registry snapshot shared by feature, predicate and property services.
- Evidence required before adoption: 1.20.1/Forge 47 registry sources, feature JSON, reload tests and state fixtures.
- Completion state: COMPLETE — 539 logical lines through EOF; pinned source SHA and 19632 byte size match; inspected, not modified; tests not executed.

### 2624. `crates/horizon-world/src/worldgen/block_pile.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2db19f9b45bf521c4ea517d1070c59d8af88f5c8`
- Size: 5685 bytes
- Coverage: Complete through EOF: 160 logical lines; full file; no gaps.
- Purpose: Randomized two-layer block pile with support/path validation.
- Dependencies / cross-references: BlockStateProvider; FeatureBlockBindings; FeatureWorld; PlacementContext; RNG; Direction.
- Implementation / logic: Checks min height, samples radii, tests candidates, requires air and validates dirt-path or sturdy support before provider writes.
- Findings: Random order matters; path branch draws boolean; write flag 260 needs target verification.
- RustChunk relevance: Adaptable placed-feature reference.
- Decision: adaptable; reference; verify dirt_path, support and provider RNG
- 1.20.1 implications: Compare 1.20.1 feature codec, dirt_path, sturdy face, random threshold, bounds and flags.
- Follow-up checks: Trace candidate draws, support branch, provider choice and flags.
- RustChunk implementation mapping: Target placed feature with provider, support predicate and mutation adapter.
- Evidence required before adoption: 1.20.1 BlockPileFeature/BlockStateProvider, feature JSON and seeded traces.
- Completion state: COMPLETE — 160 logical lines through EOF; pinned source SHA and 5685 byte size match; inspected, not modified; tests not executed.

### 2625. `crates/horizon-world/src/worldgen/block_predicate.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7d27de22e7157dc3118824cfac3fbad6420f4911`
- Size: 14702 bytes
- Coverage: Complete through EOF: 376 logical lines; full file; no gaps.
- Purpose: Composable recursive world-position predicate AST and evaluator.
- Dependencies / cross-references: VerticalAnchor; PlacementContext; PredicateBlockResolver; BlockStateId; BlockProperty; BiomeId; fluid sets.
- Implementation / logic: Decodes survival, sturdy face, property, unobstructed, biome/fluid/block/tag, height, volume and boolean predicates; evaluates under shared budget.
- Findings: Volume loops X/Z/Y; boolean forms short-circuit; unobstructed offset is decoded but ignored; abs wraps.
- RustChunk relevance: Adaptable predicate architecture; types/registry are versioned.
- Decision: adaptable; reference; rebuild target predicate set, bounds, order and Forge hooks
- 1.20.1 implications: Rebuild 1.20.1 serializers/world queries and retain Forge property, biome, fluid and survival callbacks.
- Follow-up checks: Compare type set, holder errors, offsets, recursion, order, early failures and negative heights.
- RustChunk implementation mapping: Typed predicate AST with immutable registry and world-query service.
- Evidence required before adoption: 1.20.1 BlockPredicate sources, Forge 47 hooks, registry fixtures and evaluation traces.
- Completion state: COMPLETE — 376 logical lines through EOF; pinned source SHA and 14702 byte size match; inspected, not modified; tests not executed.
