# RustChunk Starling Audit Ledger — Entries 2601–2800

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active 200-file stage: **2601–2800**; completed **150/200 (2601–2750)**.
- Exceptional authorization 1 (**2601–2700**) is closed. Exceptional authorization 2 (**2701–2800**) is active and covers the next four 25-file checkpoints.
- Source selection: strict raw lexical path order from the complete pinned tree, excluding every path already in the full Index. The tree is untruncated; all 25 records match pinned paths, SHAs and byte sizes.
- Starling is a Java Edition 26.3-era reference snapshot, not proof of Java 1.20.1 / Forge 47.x compatibility.
- Stop after entry **2800**. Entries 2801+ are not authorized. No source file was modified; tests/examples were inspected only, not executed; no build/runtime/device/Forge/Aternos validation was performed.

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

## 50-file section 2601–2650

### Checkpoint 2626–2650 — COMPLETE

### 2626. `crates/horizon-world/src/worldgen/blue_ice.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3b2e3f0c29a8a82bedf1ed7d818d33597faf9d0`
- Size: 5156 bytes
- Coverage: Complete through EOF: 156 logical lines; full file; no gaps.
- Purpose: Blue-ice feature validates an underwater origin and performs 200 randomized replacement attempts.
- Dependencies / cross-references: Block states/tags for water, packed ice, blue ice and replaceable states; FeatureWorld; RandomSource.
- Implementation / logic: Checks origin water/sea level and packed-ice adjacency, writes origin, then samples dy and triangular dx/dz offsets for 200 attempts; places only at replaceable positions adjacent to blue ice.
- Findings: Random draw order, direction order, tag contents and write flags are observable; writes can partially remain after errors.
- RustChunk relevance: Adaptable configured-feature algorithm.
- Decision: adaptable; verify target BlueIceFeature configuration, RNG and flags
- 1.20.1 implications: Compare 1.20.1 origin checks, attempt count, state tags and Java random draws.
- Follow-up checks: Seeded trace for offsets, neighbors, sea-level boundary and partial failures.
- RustChunk implementation mapping: Target feature with immutable block/tag bindings and compatible RNG.
- Evidence required before adoption: 1.20.1 BlueIceFeature source, feature JSON, tags and Java placement fixtures.
- Completion state: COMPLETE — 156 logical lines through EOF; pinned source SHA and 5156 byte size match; inspected, not modified; tests not executed.

### 2627. `crates/horizon-world/src/worldgen/bonus_chest.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ce3d399d09b0684cd4bfb2e30b9fe2c24d2b5ce2`
- Size: 3779 bytes
- Coverage: Complete through EOF: 102 logical lines; full file; no gaps.
- Purpose: Bonus-chest feature permutes candidate positions, checks terrain, places chest and cardinal torches.
- Dependencies / cross-references: Fisher-Yates random permutations; MotionBlockingNoLeaves heightmap; collision and survival callbacks; chest loot table.
- Implementation / logic: Shuffles X and Z candidates, searches for valid surface, writes chest with flags 2, assigns spawn-bonus loot table and places surviving torches.
- Findings: Permutation/query order determines location; chest block entity and loot initialization belongs to host; failure may follow chest placement.
- RustChunk relevance: Adaptable placement and integration reference.
- Decision: adaptable; verify target heightmap, chest loot and torch order
- 1.20.1 implications: Compare 1.20.1 BonusChestFeature shuffle, heightmap, collision, loot and torch behavior.
- Follow-up checks: Seeded fixtures for permutation, first valid site, chest BE/loot seed and torch sides.
- RustChunk implementation mapping: Feature adapter plus chest block-entity/loot initializer.
- Evidence required before adoption: 1.20.1 BonusChestFeature, chest loot sources, heightmap and deterministic fixtures.
- Completion state: COMPLETE — 102 logical lines through EOF; pinned source SHA and 3779 byte size match; inspected, not modified; tests not executed.

### 2628. `crates/horizon-world/src/worldgen/bulk_section.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9628ac5108b71362dbb259adfd7a253bf16aa114`
- Size: 4241 bytes
- Coverage: Complete through EOF: 113 logical lines; full file; no gaps.
- Purpose: Provides bounded direct section access for high-volume worldgen mutation.
- Dependencies / cross-references: TerrainChunk sections; packed SectionPos; block-state IDs; section cache; exclusive write ownership.
- Implementation / logic: Caches last/previous section handles, maps block positions to section keys, reads absent/out-of-range as air and performs raw section writes.
- Findings: Bypasses heightmaps, POI, lighting, neighbors and block entities; caller must apply side effects. Packed-key aliasing and section layout need parity.
- RustChunk relevance: Low-level optimization candidate with strict side-effect boundary.
- Decision: adaptable; require target packed-coordinate and exclusive-write parity
- 1.20.1 implications: Compare 1.20.1 SectionPos packing, LevelChunkSection layout and empty-section semantics.
- Follow-up checks: Test negative/extreme coords, out-of-range Y, sparse sections, cache hits and delayed side effects.
- RustChunk implementation mapping: Private generation write layer below TerrainRegion with exclusive ownership/effect handling.
- Evidence required before adoption: 1.20.1 SectionPos/LevelChunkSection sources, Forge hooks and state/heightmap/POI/light fixtures.
- Completion state: COMPLETE — 113 logical lines through EOF; pinned source SHA and 4241 byte size match; inspected, not modified; tests not executed.

### 2629. `crates/horizon-world/src/worldgen/carver.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ca5feb3eb90e0d20808cade561311cac15f37fa`
- Size: 11318 bytes
- Coverage: Complete through EOF: 296 logical lines; full file; no gaps.
- Purpose: Decodes and runs cave-carver configuration including rooms, tunnels and branches.
- Dependencies / cross-references: HeightProvider/IntProvider; predicates; LegacyRandom; carver context; carve budget and block mask.
- Implementation / logic: Validates parameters, samples starts and room chance, seeds tunnels from random longs and traces yaw/pitch paths with split branches under bounds.
- Findings: Random draw/branch order is critical; work limits cap starts/volume; partial writes persist on errors. Codec/constants are version-specific.
- RustChunk relevance: Algorithmic reference, not target drop-in.
- Decision: rewrite required; rebuild 1.20.1 CaveWorldCarver and RNG
- 1.20.1 implications: Compare 1.20.1 codec, probability, start count, tunnel/room formulas, branch seeding and mask semantics.
- Follow-up checks: Seeded draw/path/room/floor/aquifer traces and every error boundary.
- RustChunk implementation mapping: Target carver config decoder, random adapter, path generator and mask writer.
- Evidence required before adoption: 1.20.1 CaveWorldCarver, Forge carver patches, vanilla JSON and golden carve masks.
- Completion state: COMPLETE — 296 logical lines through EOF; pinned source SHA and 11318 byte size match; inspected, not modified; tests not executed.

### 2630. `crates/horizon-world/src/worldgen/carver/canyon.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `33f745f54dafeca9e832ee44b8f9b0e562be27e4`
- Size: 6804 bytes
- Coverage: Complete through EOF: 161 logical lines; full file; no gaps.
- Purpose: Canyon carver config, width profile and segmented path generation.
- Dependencies / cross-references: Width/thickness providers; LegacyRandom; world height; carver context and mask.
- Implementation / logic: Decodes shape parameters, samples origin/width profile by world height and steers canyon path with yaw/pitch randomization.
- Findings: Width indexing depends on minY/height; RNG and path steering are observable; errors leave prior mask writes.
- RustChunk relevance: Adaptable decomposition; target algorithm required.
- Decision: rewrite required; rebuild 1.20.1 CanyonWorldCarver
- 1.20.1 implications: Compare 1.20.1 codec fields, vertical profile, bounds, step count and RNG.
- Follow-up checks: Seeded fixtures for negative minY, height extremes, smoothing, path and failures.
- RustChunk implementation mapping: Target canyon config, height profile, path walker and carve mask.
- Evidence required before adoption: 1.20.1 CanyonWorldCarver, Forge patches, configured-carver JSON and golden masks.
- Completion state: COMPLETE — 161 logical lines through EOF; pinned source SHA and 6804 byte size match; inspected, not modified; tests not executed.

### 2631. `crates/horizon-world/src/worldgen/carver/generation.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `40cd5cfd3df42bfaa8b1d34eb4d9400052a1b246`
- Size: 6359 bytes
- Coverage: Complete through EOF: 167 logical lines; full file; no gaps.
- Purpose: Schedules configured carvers over neighboring origin chunks and returns a local carve mask.
- Dependencies / cross-references: Carver catalogue; biome sampling; 17×17 origins; seed derivation; upgrade height policy.
- Implementation / logic: Visits neighboring origins X then Z, resolves biome and carver order, derives per-carver large-feature seeds and trims height during upgrades.
- Findings: No live datapack/tag reload; fixed biome sample position and origin/carver order matter. Mask is only applied by a caller.
- RustChunk relevance: Useful orchestration boundary reference.
- Decision: rewrite required; target carver registry, biome sample and seed order
- 1.20.1 implications: Compare 1.20.1 carver stage, 17×17 scan, biome location, seed formula and retrogen clipping.
- Follow-up checks: Trace every origin, selected biome, carver ordinal, seed and mask.
- RustChunk implementation mapping: Target carver scheduler, registry holders, biome provider and random factory.
- Evidence required before adoption: 1.20.1 ChunkGenerator/WorldGenRegion sources, Forge hooks and multi-chunk mask fixtures.
- Completion state: COMPLETE — 167 logical lines through EOF; pinned source SHA and 6359 byte size match; inspected, not modified; tests not executed.

### 2632. `crates/horizon-world/src/worldgen/carving.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d627050140f8ca6ef140f01a91b61b14053d8b6a`
- Size: 5558 bytes
- Coverage: Complete through EOF: 146 logical lines; full file; no gaps.
- Purpose: Applies carve masks and handles replacement fluids, surface soil and postprocessing.
- Dependencies / cross-references: CarvingBlocks state/tag bindings; mask; aquifer; surface material worker; chunk writes.
- Implementation / logic: Precomputes per-state carve flags, scans marked columns top-down, applies soil rules, resolves fluid replacements and marks postprocessing.
- Findings: Column order and soil state persistence affect output; writes before failures remain; target tags/surface rules can differ.
- RustChunk relevance: Adaptable carving-mask stage.
- Decision: adaptable; verify 1.20.1 mask, surface repair and fluid ticks
- 1.20.1 implications: Compare 1.20.1 CarvingContext replacement, tags, surface repair and fluid schedule.
- Follow-up checks: Test mask packing, overlap, null aquifer, surface transitions and error cases.
- RustChunk implementation mapping: Target mask applier tied to block tags, aquifer and chunk mutation.
- Evidence required before adoption: 1.20.1 carver/surface sources, block tags, aquifer fixtures and postprocess traces.
- Completion state: COMPLETE — 146 logical lines through EOF; pinned source SHA and 5558 byte size match; inspected, not modified; tests not executed.

### 2633. `crates/horizon-world/src/worldgen/cave.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `18b06f9da5977253eecf2bef82a0f0852a2f1765`
- Size: 28292 bytes
- Coverage: Complete through EOF: 700 logical lines; full file; no gaps.
- Purpose: Implements lake, monster-room and geode configured features.
- Dependencies / cross-references: FeatureWorld/PlacementContext; block state/tag bindings; biome/fluid queries; RNG; chest/spawner BEs.
- Implementation / logic: Lake forms ellipsoid union and validates edges; room builds shell/interior, chests and spawner; geode samples shell layers, distribution points, crack/noise and crystals.
- Findings: Loop order, RNG, edge checks, block-entity setup and partial writes are critical. Treat each serializer independently.
- RustChunk relevance: Useful feature-family reference.
- Decision: adaptable; rebuild and validate each feature separately for 1.20.1
- 1.20.1 implications: Compare 1.20.1 LakeFeature, MonsterRoom/DungeonFeature and GeodeFeature codecs, states, loot/spawner, fluid edges and random.
- Follow-up checks: Golden ellipsoid/shell/spawner/geode placements and all rejection boundaries.
- RustChunk implementation mapping: Separate target feature implementations sharing registry, placement and RNG infrastructure.
- Evidence required before adoption: 1.20.1 LakeFeature/DungeonFeature/GeodeFeature sources, vanilla JSON, Forge BE hooks and goldens.
- Completion state: COMPLETE — 700 logical lines through EOF; pinned source SHA and 28292 byte size match; inspected, not modified; tests not executed.

### 2634. `crates/horizon-world/src/worldgen/chorus_plant.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `54ca29b90c2851e50d93b4bf29124e11f0ef559d`
- Size: 7143 bytes
- Coverage: Complete through EOF: 204 logical lines; full file; no gaps.
- Purpose: Generates chorus branches and terminal flowers with derived connection states.
- Dependencies / cross-references: Chorus/flower blockstate properties; end-stone support; replaceability; bounded recursion; RandomSource.
- Implementation / logic: Validates origin/support, builds 64 connection states, grows vertical stem and evaluates horizontal branches in fixed order under offset/depth bounds.
- Findings: Branch order, RNG placement and exact property encoding determine topology; connection table assumes specific properties.
- RustChunk relevance: Adaptable derived-state feature.
- Decision: adaptable; verify target chorus state table and branch RNG
- 1.20.1 implications: Compare 1.20.1 ChorusPlantFeature/Block support, state properties and branch order.
- Follow-up checks: Seeded fixtures for straight stem, every branch combination, rejected neighbors and terminal flower.
- RustChunk implementation mapping: Generate target connection table from target BlockStateDefinition.
- Evidence required before adoption: 1.20.1 ChorusPlantFeature/Block sources, blockstate definitions and deterministic growth fixtures.
- Completion state: COMPLETE — 204 logical lines through EOF; pinned source SHA and 7143 byte size match; inspected, not modified; tests not executed.

### 2635. `crates/horizon-world/src/worldgen/climate.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `92c8bb080d6c6f5383893af8beb2a32b410faf7f`
- Size: 3914 bytes
- Coverage: Complete through EOF: 118 logical lines; full file; no gaps.
- Purpose: Quantizes climate dimensions and samples density climate values in quart space or bulk volumes.
- Dependencies / cross-references: Climate parameter types; DensitySampler; SamplerContext; volume coordinate transforms.
- Implementation / logic: Maps six climate dimensions into scaled integers, handles offset dimension, samples at quart coordinates and supports prepared volume stepping.
- Findings: Float multiply/cast and negative-coordinate shifts are observable; prepared-volume bounds return None where target may rely on invariant.
- RustChunk relevance: Adaptable climate data and sampling API.
- Decision: adaptable; validate target climate scale and coordinates
- 1.20.1 implications: Compare 1.20.1 quantization, seven-value parameter, quart-to-block conversion and sample ordering.
- Follow-up checks: Test edge floats, negative coordinates, offset and scalar-vs-bulk equivalence.
- RustChunk implementation mapping: Target Climate value/volume API on 1.20.1 DensityFunction.
- Evidence required before adoption: 1.20.1 Climate/Climate.Sampler sources, NoiseRouter and deterministic samples.
- Completion state: COMPLETE — 118 logical lines through EOF; pinned source SHA and 3914 byte size match; inspected, not modified; tests not executed.

### 2636. `crates/horizon-world/src/worldgen/climate/codec.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8ea0d00d64d29e4119822e4e9ae9f1720321ed85`
- Size: 2227 bytes
- Coverage: Complete through EOF: 59 logical lines; full file; no gaps.
- Purpose: Decodes bounded climate intervals and offset values.
- Dependencies / cross-references: RawValue JSON; float parser; six climate keys; min/max interval.
- Implementation / logic: Accepts scalar, 2-element range and min/max object; validates bounds/order and requires six axes plus offset.
- Findings: NaN, signed zero, token narrowing and accepted schema may differ from Mojang codec.
- RustChunk relevance: Small codec reference only.
- Decision: rewrite required; target 1.20.1 Climate.Parameter codec
- 1.20.1 implications: Compare target scalar/range formats, bounds, required fields and numeric casts.
- Follow-up checks: Round-trip vanilla JSON and fuzz invalid lengths/ranges/NaN/infinity/signed zero.
- RustChunk implementation mapping: Version-specific climate parameter codec under target biome source.
- Evidence required before adoption: 1.20.1 Climate.Parameter source and vanilla biome data acceptance corpus.
- Completion state: COMPLETE — 59 logical lines through EOF; pinned source SHA and 2227 byte size match; inspected, not modified; tests not executed.

### 2637. `crates/horizon-world/src/worldgen/climate/index.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6a602ed924b903499ac514bdf504f81b35b9e878`
- Size: 11492 bytes
- Coverage: Complete through EOF: 342 logical lines; full file; no gaps.
- Purpose: Constructs and searches custom multidimensional climate parameter index.
- Dependencies / cross-references: Seven-dimensional points; interval distance/fitness; branching tree; flattened links; cached previous match.
- Implementation / logic: Builds bounded partitions across climate axes, flattens for query, prunes by interval distance and retains a prior result for faster repeated searches.
- Findings: Partition algorithm, tie-breaks and prior-result shortcut can diverge from vanilla R-tree selection. High-risk semantic difference despite performance potential.
- RustChunk relevance: Performance architecture only until selection parity is proven.
- Decision: rewrite required; do not adopt without vanilla tie parity
- 1.20.1 implications: Compare 1.20.1 Climate.RTree insertion, fitness, ties, search pruning and iteration order.
- Follow-up checks: Exhaustive Java/Rust queries including equal-distance ties and previous-query order effects.
- RustChunk implementation mapping: Keep target Climate.RTree authoritative unless proven identical.
- Evidence required before adoption: 1.20.1 Climate.RTree source and exhaustive biome selection fixtures.
- Completion state: COMPLETE — 342 logical lines through EOF; pinned source SHA and 11492 byte size match; inspected, not modified; tests not executed.

### 2638. `crates/horizon-world/src/worldgen/decoration.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3e2e92b33cf4c809fcb7c0b85a4caf6e01cd6269`
- Size: 7170 bytes
- Coverage: Complete through EOF: 180 logical lines; full file; no gaps.
- Purpose: Coordinates structure and biome decoration feature steps across neighboring chunks.
- Dependencies / cross-references: 3×3 biome region; feature identities/sort; WorldgenRandom/Xoroshiro; structure callback; current-feature marker.
- Implementation / logic: Collects nearby biome features, sorts/deduplicates identities, interleaves structure work with steps, seeds feature execution and shares placer.
- Findings: Biome aggregation, structure timing, feature sort and RNG reseeding are behavior-critical; error can leave current-feature marker set.
- RustChunk relevance: Adaptable stage orchestration, target semantics control.
- Decision: rewrite required; rebuild target WorldGenRegion decoration and seeding
- 1.20.1 implications: Compare 1.20.1 applyBiomeDecoration, FeatureSorter, WorldgenRandom and structure/feature order.
- Follow-up checks: Golden 3×3 collection, feature sorting, step/RNG and error cleanup traces.
- RustChunk implementation mapping: Target decoration scheduler integrated with FeatureSorter and structure manager.
- Evidence required before adoption: 1.20.1 WorldGenRegion/FeatureSorter/WorldgenRandom, Forge hooks and seeded chunk fixtures.
- Completion state: COMPLETE — 180 logical lines through EOF; pinned source SHA and 7170 byte size match; inspected, not modified; tests not executed.

### 2639. `crates/horizon-world/src/worldgen/decoration_order.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ea9e8e023d7cf751049e60d892e1935be1195b95`
- Size: 16676 bytes
- Coverage: Complete through EOF: 420 logical lines; full file; no gaps.
- Purpose: Emulates chunk decoration ticket scheduler and neighboring readiness order.
- Dependencies / cross-references: Chunk tickets/status levels; hash positions; grid/heap; concurrency and completion predicates.
- Implementation / logic: Tracks nearby tickets, caps concurrent tasks, applies FULL/LIGHT thresholds, waits for neighbor rings and deterministically unblocks lower-ranked chunks.
- Findings: Scheduler is version-specific; hash iteration and tie-breaking emulation matter. Included tests/goldens inspected, not run.
- RustChunk relevance: High-risk concurrency reference, not safe drop-in.
- Decision: rewrite required; derive dependencies from 1.20.1 ChunkStatus pipeline
- 1.20.1 implications: Compare 1.20.1 ChunkMap sorter, tickets, status transitions and light/full dependencies.
- Follow-up checks: JFR/trace with varied request order, cancellation, failed futures and neighbor completions.
- RustChunk implementation mapping: Explicit target chunk-status dependency graph and bounded scheduler.
- Evidence required before adoption: 1.20.1 ChunkMap/ChunkTaskPriorityQueueSorter/ChunkStatus sources and stress traces.
- Completion state: COMPLETE — 420 logical lines through EOF; pinned source SHA and 16676 byte size match; inspected, not modified; tests not executed.

### 2640. `crates/horizon-world/src/worldgen/delta.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `884a103bdb0d43c1eef9ff6ce95046fff0000bb3`
- Size: 6345 bytes
- Coverage: Complete through EOF: 202 logical lines; full file; no gaps.
- Purpose: Places Nether delta rim/content with randomized diamond-like sampling.
- Dependencies / cross-references: Block state and tag bindings; IntProviders; FeatureWorld; RandomSource.
- Implementation / logic: Samples sizes and rim probability, iterates a Manhattan-style footprint, checks block classes/neighbors and writes rim or content blocks.
- Findings: Traversal/RNG order, hard-coded block exclusions, neighbor relations and flags determine output.
- RustChunk relevance: Adaptable configured-feature shape.
- Decision: adaptable; verify 1.20.1 delta codec, block checks and RNG
- 1.20.1 implications: Compare 1.20.1 DeltaFeature fields, exclusions, neighbor tests and update flags.
- Follow-up checks: Seeded footprint boundary and all excluded-neighbor/provider cases.
- RustChunk implementation mapping: Target feature over block tags/properties and Java random providers.
- Evidence required before adoption: 1.20.1 DeltaFeature source, JSON, Nether tags and placement goldens.
- Completion state: COMPLETE — 202 logical lines through EOF; pinned source SHA and 6345 byte size match; inspected, not modified; tests not executed.

### 2641. `crates/horizon-world/src/worldgen/density.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6cd2c7df70d76188d16268c4032503eb00899be2`
- Size: 24432 bytes
- Coverage: Complete through EOF: 578 logical lines; full file; no gaps.
- Purpose: Builds named density graph and decodes arithmetic, spatial, noise, interpolation and spline operators.
- Dependencies / cross-references: DensityRegistry/graph; named holders and forward references; noise registry; spline points; compiler.
- Implementation / logic: Loads named graphs atomically, resolves forward references/cycles and decodes arithmetic/unary, gradient, noise/shift, caches/blends, interpolation, top-surface, range/interval selection, distance and spline nodes.
- Findings: Depth/node/spline bounds are enforced; raw number tokens retain negative zero and avoid double rounding. Operator catalogue and named-noise rules differ from target. Tests present but not run.
- RustChunk relevance: Core graph architecture reference; target codecs must be rebuilt.
- Decision: rewrite required; rebuild 1.20.1 DensityFunctions and NoiseRouter
- 1.20.1 implications: Compare complete 1.20.1 operator catalogue, cache/interpolation nodes, spline semantics, holder lookup and evaluation order.
- Follow-up checks: Per-operator golden samples, forward/cycle refs, signed zero, splines, negative positions and interpolation.
- RustChunk implementation mapping: Target DensityFunction AST/codec/compiler for 1.20.1 NoiseRouter and aquifer.
- Evidence required before adoption: 1.20.1 DensityFunctions/NoiseRouter sources, worldgen JSON, Forge registries and Java sample goldens.
- Completion state: COMPLETE — 578 logical lines through EOF; pinned source SHA and 24432 byte size match; inspected, not modified; tests not executed.

### 2642. `crates/horizon-world/src/worldgen/disk.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f088c616fd8f49e65a75edc17cd4818413036232`
- Size: 5974 bytes
- Coverage: Complete through EOF: 168 logical lines; full file; no gaps.
- Purpose: Disk feature writes provider-generated circular layers and marks postprocessing above exposed runs.
- Dependencies / cross-references: BlockStateProvider; BlockPredicate; radius/half-height IntProviders; FeatureWorld; postprocessing.
- Implementation / logic: Iterates columns X then Z and Y top-down, checks predicate, writes states and marks positions above contiguous surface runs.
- Findings: Traversal, provider RNG, predicate timing, flag 2 and postprocessing are observable; work bound rejects pathological input.
- RustChunk relevance: Adaptable disk algorithm.
- Decision: adaptable; verify target DiskFeature codec and write behavior
- 1.20.1 implications: Compare 1.20.1 DiskFeature fields, bounds, loop order, replacement predicate and postprocess marks.
- Follow-up checks: Seeded disks on negative/positive coords, boundary radii, provider outputs and write failures.
- RustChunk implementation mapping: Target disk feature with provider/predicate and chunk postprocessing service.
- Evidence required before adoption: 1.20.1 DiskFeature source, vanilla JSON and deterministic mutation traces.
- Completion state: COMPLETE — 168 logical lines through EOF; pinned source SHA and 5974 byte size match; inspected, not modified; tests not executed.

### 2643. `crates/horizon-world/src/worldgen/dripstone.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7a11af7f5ca00b791aa7143adb29fc73b023ff3e`
- Size: 27815 bytes
- Coverage: Complete through EOF: 800 logical lines; full file; no gaps.
- Purpose: Implements speleothem-cluster and large-dripstone features.
- Dependencies / cross-references: Dripstone states; floor/ceiling scan; providers; RNG; heightmap and wind offsets.
- Implementation / logic: Cluster searches floor/ceiling and samples column heights/pools; large dripstone samples radius, bluntness, height scale and wind, adjusts roots and fills tapered columns.
- Findings: RNG, scan direction, float geometry, wind, fluid exclusions and writes matter. Modern speleothem type/schema is not a 1.20.1 drop-in.
- RustChunk relevance: Algorithmic reference only.
- Decision: rewrite required; rebuild 1.20.1 DripstoneClusterFeature/LargeDripstoneFeature
- 1.20.1 implications: Compare target codecs, pointed-dripstone state transitions, floor/ceiling detection, shape formula and RNG.
- Follow-up checks: Seeded cave cases with water/lava, thin roof, short columns, wind and provider extremes.
- RustChunk implementation mapping: Target dripstone feature pair and pointed-dripstone state transition service.
- Evidence required before adoption: 1.20.1 DripstoneClusterFeature/LargeDripstoneFeature sources, JSON and seeded geometry goldens.
- Completion state: COMPLETE — 800 logical lines through EOF; pinned source SHA and 27815 byte size match; inspected, not modified; tests not executed.

### 2644. `crates/horizon-world/src/worldgen/end_gateway.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b1c11f21b5dbd2d3f52ef0d8809c7f307687ffe1`
- Size: 4377 bytes
- Coverage: Complete through EOF: 123 logical lines; full file; no gaps.
- Purpose: Places fixed End gateway portal structure and optionally configures exit position.
- Dependencies / cross-references: Air/bedrock/gateway states; exit vector; exact flag; world writes and block-entity callback.
- Implementation / logic: Fills 3×5×3 volume in fixed coordinate order and configures center gateway block entity when exit is supplied.
- Findings: `exact` applies to BE configuration; config callback follows write call even if mutation reports false. Partial structure can remain after errors.
- RustChunk relevance: Adaptable structure feature with BE integration risk.
- Decision: adaptable; verify target EndGatewayFeature and BE lifecycle
- 1.20.1 implications: Compare 1.20.1 gateway geometry, exact exit semantics, flags and BE update timing.
- Follow-up checks: Test optional exit, exact true/false, existing gateway, no-op write and callback errors.
- RustChunk implementation mapping: Gateway feature plus target block entity configuration/update service.
- Evidence required before adoption: 1.20.1 EndGatewayFeature/BlockEntity sources, structure JSON and mutation traces.
- Completion state: COMPLETE — 123 logical lines through EOF; pinned source SHA and 4377 byte size match; inspected, not modified; tests not executed.

### 2645. `crates/horizon-world/src/worldgen/end_podium.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3fafc591cc3c72c921ac5f1e36b5fb0a1fd6eb53`
- Size: 6245 bytes
- Coverage: Complete through EOF: 174 logical lines; full file; no gaps.
- Purpose: Builds End exit podium, bedrock base, portal and cardinal torches.
- Dependencies / cross-references: Bedrock/end-stone/air/portal; wall torch facing states; active flag; destroy-without-entity callback.
- Implementation / logic: Iterates bounded 9×34×9 volume, classifies inner/outer distance and Y layers, optionally clears/replaces blocks, then writes central bedrock and torches.
- Findings: Active/inactive paths differ; distance tests, traversal, destroy-without-drops and torch orientations matter.
- RustChunk relevance: Adaptable fixed-structure feature.
- Decision: adaptable; verify 1.20.1 EndPodiumFeature and replacement semantics
- 1.20.1 implications: Compare active default, distance thresholds, portal/bedrock/torch layout and destroyBlock behavior.
- Follow-up checks: Golden state maps active/inactive with pre-existing blocks and boundary radii.
- RustChunk implementation mapping: Target podium feature with state-orientation mapping and worldgen replacement hooks.
- Evidence required before adoption: 1.20.1 EndPodiumFeature, torch state definitions, portal resources and structure snapshots.
- Completion state: COMPLETE — 174 logical lines through EOF; pinned source SHA and 6245 byte size match; inspected, not modified; tests not executed.

### 2646. `crates/horizon-world/src/worldgen/end_spike.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e76982853c83e65b66f415601e8a35af34d04994`
- Size: 8826 bytes
- Coverage: Complete through EOF: 243 logical lines; full file; no gaps.
- Purpose: Generates ten End obsidian pillars, cages and crystals.
- Dependencies / cross-references: World seed; LegacyRandom; shuffled spike sizes; block states; crystal entity and beam target.
- Implementation / logic: Derives layout from seed, shuffles sizes, places intersecting pillars/cages, spawns crystals and writes fire/bedrock in ordered steps.
- Findings: Seed truncation, shuffle, chunk loops, bar orientation and entity-before-block ordering are observable; included tests not run.
- RustChunk relevance: Useful deterministic structure reference.
- Decision: adaptable; verify 1.20.1 SpikeFeature layout and crystal spawn order
- 1.20.1 implications: Compare 1.20.1 SpikeFeature/SpikeCacheLoader seed, pillar sizes, guard bars and crystal properties.
- Follow-up checks: Golden layouts across seeds and chunk borders; spawn failure and bar property fixtures.
- RustChunk implementation mapping: Seeded spike service with target random, state property and entity transaction.
- Evidence required before adoption: 1.20.1 SpikeFeature/SpikeCacheLoader/EndCrystal sources and deterministic layouts.
- Completion state: COMPLETE — 243 logical lines through EOF; pinned source SHA and 8826 byte size match; inspected, not modified; tests not executed.

### 2647. `crates/horizon-world/src/worldgen/feature.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4595d509990699ebad91d45e4f394a0e4a23a03f`
- Size: 6946 bytes
- Coverage: Complete through EOF: 186 logical lines; full file; no gaps.
- Purpose: Basic void-start platform, End island and End platform features plus shared FeatureWorld interface.
- Dependencies / cross-references: Block registry; FeatureWorld block/scan/write; RandomSource; mutation flags.
- Implementation / logic: Decodes basic feature kinds and writes platform/island shapes using bounded loops and random disk radii.
- Findings: Void platform uses absolute coordinates with a fixed offset from 8; verify target coordinate origin carefully. Island RNG order/flags also need parity.
- RustChunk relevance: Minimal feature API and algorithm reference.
- Decision: adaptable; verify coordinate origin, End island RNG and flags
- 1.20.1 implications: Compare 1.20.1 VoidStartPlatformFeature, EndIslandFeature, EndPlatformFeature loops and local/global coordinates.
- Follow-up checks: Test chunks positive/negative XZ, central offsets, radius reduction, existing states and flags.
- RustChunk implementation mapping: Basic feature module over target registry and world adapter.
- Evidence required before adoption: 1.20.1 feature sources/mappings, configured JSON and exact coordinate/RNG fixtures.
- Completion state: COMPLETE — 186 logical lines through EOF; pinned source SHA and 6946 byte size match; inspected, not modified; tests not executed.

### 2648. `crates/horizon-world/src/worldgen/feature_catalog.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4db25111464b60cde7c5b264447e1359d08f0ffd`
- Size: 12064 bytes
- Coverage: Complete through EOF: 304 logical lines; full file; no gaps.
- Purpose: Builds immutable feature resource snapshot and expands named references/tags.
- Dependencies / cross-references: Resource maps; tag definitions; feature/placed-feature definitions; cycle detection; bounded JSON nodes.
- Implementation / logic: Indexes named resources, resolves placed-feature holders/tags, recursively expands selected nested refs and selector sets, detects cycles and empty tags.
- Findings: Not a full datapack live-reload registry; only selected nested fields expand; validation is reachability-based; pale-moss special case is version-specific.
- RustChunk relevance: Adaptable immutable snapshot and cycle-safe expansion.
- Decision: adaptable; rebuild target feature holder/tag and reload lifecycle
- 1.20.1 implications: Compare 1.20.1 configured/placed holder codecs, HolderSet/tag handling and Forge datapack reload semantics.
- Follow-up checks: Test missing/cycles, unused malformed values, empty tags, nested selectors, pale moss and atomic reload.
- RustChunk implementation mapping: Versioned target feature resource loader and registry snapshot.
- Evidence required before adoption: 1.20.1 FeatureUtils/RegistryOps/HolderSet, Forge reload sources and vanilla resources.
- Completion state: COMPLETE — 304 logical lines through EOF; pinned source SHA and 12064 byte size match; inspected, not modified; tests not executed.

### 2649. `crates/horizon-world/src/worldgen/feature_graph.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `014ea3f42f18e9a797dd8b4eeb94a717d7374cfa`
- Size: 48314 bytes
- Coverage: Complete through EOF: 1106 logical lines; full file; no gaps.
- Purpose: Typed configured-feature graph, bounded decoder and recursive execution engine.
- Dependencies / cross-references: Feature/PlacedFeature definitions; codecs; placement modifiers/FeaturePlacer; providers/predicates; RNG and shared budgets.
- Implementation / logic: Decodes broad feature catalogue and composites, enforces depth/list/node limits and evaluates basic, nested, sequence, overlay, weighted, boolean, random and chance paths.
- Findings: Short-circuiting, overlay aggregation, RNG, nested placer reuse and mutation/error boundaries are observable. Catalogue is newer and includes non-1.20.1 types.
- RustChunk relevance: Architecture reference, not target registry.
- Decision: rewrite required; build target 1.20.1/Forge serializer catalogue and graph
- 1.20.1 implications: Compare all 1.20.1 feature serializers/modifiers/composites and Forge-added types with exact execution order.
- Follow-up checks: Exhaustive codec corpus and seeded execution traces for each variant, error and budget.
- RustChunk implementation mapping: Target-version feature AST/codec dispatch, placer, RNG and Forge extension registry.
- Evidence required before adoption: 1.20.1 configured/placed registries, serializers, Forge hooks, vanilla JSON and Java golden traces.
- Completion state: COMPLETE — 1106 logical lines through EOF; pinned source SHA and 48314 byte size match; inspected, not modified; tests not executed.

### 2650. `crates/horizon-world/src/worldgen/feature_region.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1b8f02aeab5f424589ba5284c922af49aebe8ff1`
- Size: 24625 bytes
- Coverage: Complete through EOF: 683 logical lines; full file; no gaps.
- Purpose: Resident multi-chunk terrain region with feature query and mutation adapter.
- Dependencies / cross-references: TerrainChunk; biome region; block/fluid/tag registry; proto ticks; heightmaps; write zone and upgrade height.
- Implementation / logic: Sorts chunks X/Z, indexes grid/sparse, exposes block/biome/tick/height queries and wraps TerrainRegion in TerrainFeatureAccess enforcing write zone and upgrade-height policy.
- Findings: Storage writes separated from world effects; missing chunks and ungenerated biomes fail explicitly. Tick namespace/order, lazy heightmaps, upgrade status and host callbacks need target lifecycle parity.
- RustChunk relevance: Important region/access layering reference.
- Decision: adaptable; rebuild against 1.20.1 WorldGenRegion/ProtoChunk and Forge side effects
- 1.20.1 implications: Compare WorldGenRegion constructor, write radius, upgrade limits, biome status, ticks, heightmaps, block entities, POI/light and neighbor updates.
- Follow-up checks: Test sparse/grid, duplicate/missing chunks, negative coords, minY, write boundaries, live upgrade state and callback failure.
- RustChunk implementation mapping: Target WorldGenRegion abstraction over ChunkAccess/ProtoChunk with effects/tick scheduler.
- Evidence required before adoption: 1.20.1 WorldGenRegion/ProtoChunk/ChunkAccess, Forge patches, saved fixtures and deterministic tick/write traces.
- Completion state: COMPLETE — 683 logical lines through EOF; pinned source SHA and 24625 byte size match; inspected, not modified; tests not executed.

## 50-file section 2651–2700

### Checkpoint 2651–2675 — COMPLETE

### 2651. `crates/horizon-world/src/worldgen/feature_sort.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `82d4ef6a073c0cbcb108ecf17b70908a18170ca7`
- Size: 4147 bytes
- Coverage: Complete through EOF: 116 logical lines; full file; no gaps.
- Purpose: Builds a deterministic feature execution order from biome generation settings.
- Dependencies / cross-references: FeatureIdentity; per-biome feature steps; dependency graph; ordered maps/sets and topological sorting.
- Implementation / logic: Collects feature identities across steps, adds precedence edges for adjacent features in each step and performs a stable topological order using the original step/index priority.
- Findings: Duplicate feature appearances and step order affect edge creation; cycles or invalid graph bounds must fail explicitly. Stable tie-breaking is essential for worldgen parity.
- RustChunk relevance: Useful ordering architecture; exact sort behavior must match 1.20.1.
- Decision: adaptable; verify vanilla FeatureSorter graph and tie ordering
- 1.20.1 implications: Compare 1.20.1 FeatureSorter construction, identity deduplication, graph edges, priority queue and cycle handling.
- Follow-up checks: Build fixtures with repeated features across steps, shared dependencies, equal-priority nodes and cycles; compare final order to Java.
- RustChunk implementation mapping: Target FeatureSorter with registry-holder identity and deterministic graph ordering.
- Evidence required before adoption: 1.20.1 FeatureSorter source/mappings, biome generation settings and Java golden feature orders.
- Completion state: COMPLETE — 116 logical lines through EOF; pinned source SHA and 4147 byte size match; inspected, not modified; tests not executed.

### 2652. `crates/horizon-world/src/worldgen/fluid_predicate.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1394445c99d2acc8e5c303fd7e98d2878f98ea37`
- Size: 3664 bytes
- Coverage: Complete through EOF: 107 logical lines; full file; no gaps.
- Purpose: Resolves fluid identifiers and fluid tags into immutable predicate sets.
- Dependencies / cross-references: Fluid registry snapshot; fluid identifier canonicalization; tag membership; FluidKind identity.
- Implementation / logic: Decodes one fluid, a tag reference or a collection of entries, resolving names and tags against the supplied registry into a compact set used by placement predicates.
- Findings: Unknown names/tags and registry mismatches are explicit errors; the set is valid only for the exact registry snapshot used during binding.
- RustChunk relevance: Adaptable registry-binding utility.
- Decision: adaptable; verify target FluidTag/HolderSet codec and Forge registry lifecycle
- 1.20.1 implications: Compare 1.20.1 fluid predicate serialization, tag expansion, holder identity and Forge fluid registration/reload behavior.
- Follow-up checks: Test missing IDs/tags, empty/duplicate lists, tag reload, namespace canonicalization and stale registry handles.
- RustChunk implementation mapping: Versioned fluid registry snapshot and shared predicate resolver for worldgen and block conditions.
- Evidence required before adoption: 1.20.1 Fluid/TagKey/HolderSet sources, Forge fluid registry hooks, vanilla tags and reload fixtures.
- Completion state: COMPLETE — 107 logical lines through EOF; pinned source SHA and 3664 byte size match; inspected, not modified; tests not executed.

### 2653. `crates/horizon-world/src/worldgen/fungus.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d560b41b587a7790c5e4a32c4ec6045e58907eeb`
- Size: 10697 bytes
- Coverage: Complete through EOF: 300 logical lines; full file; no gaps.
- Purpose: Generates huge crimson/warped fungi including stems, caps and hanging vegetation.
- Dependencies / cross-references: Huge fungus configuration; crimson/warped block states and tags; support predicates; bounded world writes; RandomSource.
- Implementation / logic: Validates substrate and clearance, chooses a configured height, builds trunk and cap geometry with variant-specific states, and conditionally grows shroomlight/vines or weeping vegetation.
- Findings: Shape dimensions, block tests, branch order and RNG consumption determine the result. State-property and tag assumptions are registry-specific; partial placement remains after an error.
- RustChunk relevance: Adaptable feature family with version-sensitive block-state behavior.
- Decision: adaptable; verify target HugeFungusFeature config, cap geometry, tags and RNG
- 1.20.1 implications: Compare 1.20.1 HugeFungusFeature and Nether vegetation states, height distribution, cap bounds, survival tests and placement flags.
- Follow-up checks: Seeded fixtures for crimson/warped variants, low/high height, invalid substrate, cap edges, vines and world-boundary failures.
- RustChunk implementation mapping: Separate target HugeFungusFeature with immutable state/tag bindings and Java-compatible random stream.
- Evidence required before adoption: 1.20.1 HugeFungusFeature sources, blockstate definitions, configured-feature JSON, Forge tags and golden placement maps.
- Completion state: COMPLETE — 300 logical lines through EOF; pinned source SHA and 10697 byte size match; inspected, not modified; tests not executed.

### 2654. `crates/horizon-world/src/worldgen/generator.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `52cf5afbb08345d4538834de7567e8bd31af4b60`
- Size: 185496 bytes
- Coverage: Complete through EOF: 4575 logical lines; full file; no gaps.
- Purpose: Implements the main world-generation orchestration and terrain-generation pipeline for the pinned modern snapshot.
- Dependencies / cross-references: Noise settings/router and density graph; biome sources; chunk/section access; worldgen random; structures, carvers, surface/material stages and feature scheduling.
- Implementation / logic: Coordinates generation setup and per-chunk stages, prepares noise and biome samplers, produces density/terrain columns, applies structures and carvers, transitions chunk generation through later terrain and decoration stages, and handles worker/context state.
- Findings: This is a central version-specific engine, not an isolated utility. Stage sequencing, seeded randomness, worker ownership, chunk status transitions and side effects are coupled; the pinned implementation is from a later Java Edition and contains schemas/paths unavailable in 1.20.1.
- RustChunk relevance: Architectural map for the worldgen engine; requires a target-version rewrite and differential validation.
- Decision: rewrite required; architecture reference only; rebuild the full pipeline for 1.20.1/Forge 47
- 1.20.1 implications: Map every generation stage against 1.20.1 ChunkGenerator, NoiseBasedChunkGenerator, NoiseChunk, WorldGenRegion, ChunkStatus, carver and decoration implementations; Forge patches may change callbacks.
- Follow-up checks: Create per-stage seed/position golden traces and compare terrain density, block states, heightmaps, biome assignments, carver masks, structure placement, feature order and failures against 1.20.1 Java.
- RustChunk implementation mapping: Target-version WorldGenCoordinator with explicit chunk-stage state, target NoiseRouter/DensityFunction, biome samplers, structure/carver/material/feature services and Forge adapters.
- Evidence required before adoption: Complete Mojang-mapped 1.20.1 source set, Forge 47 patch set, vanilla worldgen datapacks and deterministic Java golden chunk fixtures.
- Completion state: COMPLETE — 4575 logical lines through EOF; pinned source SHA and 185496 byte size match; inspected, not modified; tests not executed.

### 2655. `crates/horizon-world/src/worldgen/generator_blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7498b9e6672c476835ec34414904fc3ab18f9b56`
- Size: 6584 bytes
- Coverage: Complete through EOF: 168 logical lines; full file; no gaps.
- Purpose: Provides block-state identities and common block classifications used by terrain generation.
- Dependencies / cross-references: Block registry snapshot; canonical block states; air/solid/fluid/water/stone classifications; generation bindings.
- Implementation / logic: Resolves the generator’s commonly used block-state handles and exposes predicates/default states needed by terrain and material generation.
- Findings: Cached handles are tied to one registry snapshot; hard-coded classifications and modern registry identities must not leak into a reload or target-version registry.
- RustChunk relevance: Useful binding pattern, but IDs and predicates are target-version specific.
- Decision: adaptable; rebuild state bindings against 1.20.1 registries
- 1.20.1 implications: Compare the exact 1.20.1 block constants, tags, fluid states and registry lifecycle, including Forge-added blocks and state properties.
- Follow-up checks: Verify every required block exists, default states match, tags reload safely and cross-registry IDs are rejected.
- RustChunk implementation mapping: Immutable target BlockStateBindings shared by generator, material and feature subsystems.
- Evidence required before adoption: 1.20.1 Blocks/BlockState/FluidState sources, Forge registry lifecycle, blockstate definitions and registry identity tests.
- Completion state: COMPLETE — 168 logical lines through EOF; pinned source SHA and 6584 byte size match; inspected, not modified; tests not executed.

### 2656. `crates/horizon-world/src/worldgen/generator_decoration.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4222bc33e46eee214472fb88b4edf4062d215787`
- Size: 57948 bytes
- Coverage: Complete through EOF: 1481 logical lines; full file; no gaps.
- Purpose: Implements later world-generation decoration and feature dispatch over generated chunk regions.
- Dependencies / cross-references: Biome generation settings; placed-feature definitions; feature ordering; placement modifiers; generation region and random state.
- Implementation / logic: Builds and executes decoration-related work for a chunk, connecting biome feature lists, placement chains, feature execution and chunk-region access while preserving stage ordering.
- Findings: Feature and placement schemas are snapshot-specific; feature ordering and RNG reseeding are compatibility-critical. Exact callback and chunk-write semantics need target Java/Forge comparison.
- RustChunk relevance: Architectural reference for decoration subsystem boundaries.
- Decision: rewrite required; rebuild target biome decoration/feature dispatch
- 1.20.1 implications: Compare 1.20.1 WorldGenRegion.applyBiomeDecoration, FeatureSorter, PlacedFeature and placement modifier chains, including Forge decoration hooks.
- Follow-up checks: Golden traces for neighboring biome feature collection, sort order, per-feature seeds, placement emissions and block-write effects.
- RustChunk implementation mapping: Target decoration stage using 1.20.1 FeatureSorter, placed-feature registry, placement chain and shared mutation adapter.
- Evidence required before adoption: 1.20.1 WorldGenRegion/FeatureSorter/PlacedFeature sources, vanilla datapacks, Forge 47 patches and seeded decoration fixtures.
- Completion state: COMPLETE — 1481 logical lines through EOF; pinned source SHA and 57948 byte size match; inspected, not modified; tests not executed.

### 2657. `crates/horizon-world/src/worldgen/generator_structures.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `59025a1e5d34318a0a0dbf8e9ca077e8cd8f1ce7`
- Size: 34854 bytes
- Coverage: Complete through EOF: 841 logical lines; full file; no gaps.
- Purpose: Coordinates structure starts, references and generation-stage integration.
- Dependencies / cross-references: Structure/StructureSet registries; biome source; chunk positions; structure placement and pieces; terrain/height queries; chunk status.
- Implementation / logic: Resolves applicable structure sets and placement candidates, obtains or reuses starts/references and coordinates structure generation against neighboring chunks and terrain.
- Findings: Structure placement salts, spacing/separation, biome eligibility, start reuse, reference limits and piece write ordering are target-version-sensitive; persistent start/reference data must match chunk serialization.
- RustChunk relevance: Useful structural architecture map; not a portable implementation.
- Decision: rewrite required; rebuild 1.20.1 Structure/StructureSet and chunk persistence integration
- 1.20.1 implications: Compare 1.20.1 StructurePlacement, StructureState, StructureManager, StructureStart, StructurePiece and ChunkSerializer plus Forge structure hooks.
- Follow-up checks: Test region boundaries, negative chunk coordinates, placement salts, biome rejection, duplicate references, failed starts and saved/reloaded structures.
- RustChunk implementation mapping: Target structure registry and placement service integrated with ChunkAccess/StructureManager and target NBT persistence.
- Evidence required before adoption: 1.20.1 structure classes/mappings, vanilla structure sets/templates, Forge 47 structure patches and saved-chunk golden fixtures.
- Completion state: COMPLETE — 841 logical lines through EOF; pinned source SHA and 34854 byte size match; inspected, not modified; tests not executed.

### 2658. `crates/horizon-world/src/worldgen/height_provider.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6849f3a3a968ff7627255e4765a23dcc97182ef6`
- Size: 5721 bytes
- Coverage: Complete through EOF: 149 logical lines; full file; no gaps.
- Purpose: Decodes and samples world-generation height providers.
- Dependencies / cross-references: VerticalAnchor; WorldGenerationContext; RandomSource; provider bounds and height arithmetic.
- Implementation / logic: Supports constant, uniform, biased-to-bottom and very-biased-to-bottom style height sampling against generation bounds and anchor resolution.
- Findings: Random draw sequence, inclusive/exclusive endpoints, vertical-anchor bounds and overflow behavior affect placed features and carvers.
- RustChunk relevance: Adaptable provider structure.
- Decision: adaptable; verify target HeightProvider codec and random bounds
- 1.20.1 implications: Compare 1.20.1 HeightProvider provider catalogue, codec fields, anchor limits and sample algorithms.
- Follow-up checks: Seeded endpoint tests for minimum/maximum bounds, negative minY, height one, invalid intervals and Java random consumption.
- RustChunk implementation mapping: Target HeightProvider enum and codec over the 1.20.1 VerticalAnchor/WorldGenerationContext.
- Evidence required before adoption: 1.20.1 HeightProvider/VerticalAnchor sources, vanilla provider JSON and Java sampling fixtures.
- Completion state: COMPLETE — 149 logical lines through EOF; pinned source SHA and 5721 byte size match; inspected, not modified; tests not executed.

### 2659. `crates/horizon-world/src/worldgen/iceberg.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3a7409ad3177676e6015fc080333c1b78088a3ca`
- Size: 13967 bytes
- Coverage: Complete through EOF: 390 logical lines; full file; no gaps.
- Purpose: Generates ocean icebergs with randomized iceberg geometry and water/ice cleanup.
- Dependencies / cross-references: Packed ice/ice/snow block states; ocean height and water queries; shape noise; RandomSource; block mutation.
- Implementation / logic: Chooses iceberg position and dimensions, constructs an irregular tapered mass, places ice/snow layers and performs surrounding water/air cleanup based on geometric and block-state tests.
- Findings: Random draw order, noise samples, negative-coordinate handling and cleanup traversal are observable. Modern state/tag assumptions and geometry constants require target parity.
- RustChunk relevance: Adaptable feature architecture, but geometry and RNG are high-risk.
- Decision: adaptable; verify target IcebergFeature dimensions, cleanup and RNG
- 1.20.1 implications: Compare 1.20.1 IcebergFeature source for height/radius distribution, shape equations, snow cap, water carving and block-write order.
- Follow-up checks: Seeded fixtures across shoreline, deep ocean, negative coordinates, narrow/wide forms, water cleanup and block obstruction.
- RustChunk implementation mapping: Target IcebergFeature with 1.20.1 block states, ocean-height service, compatible noise and random stream.
- Evidence required before adoption: 1.20.1 IcebergFeature source/mappings, configured-feature JSON, block tags and exact seeded block snapshots.
- Completion state: COMPLETE — 390 logical lines through EOF; pinned source SHA and 13967 byte size match; inspected, not modified; tests not executed.

### 2660. `crates/horizon-world/src/worldgen/live.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d4a499fcb3a613d96e50dba02e0301d9377647ec`
- Size: 13713 bytes
- Coverage: Complete through EOF: 347 logical lines; full file; no gaps.
- Purpose: Represents and merges live data-pack world-generation overrides over prepared vanilla bundles.
- Dependencies / cross-references: Pack file paths and namespaces; resource IDs; dimension/type JSON; biome/density/noise/feature/carver/structure bundles; tag bundles.
- Implementation / logic: Classifies relevant pack files, applies highest-pack replacement semantics to entries, accumulates tag files in pack order, resolves dimension height and filters unsupported new biomes from tags/climate points.
- Findings: The implementation explicitly describes 26.3 folder naming and currently leaves added biomes out; tags have additive/replacement semantics distinct from ordinary entries. This is not a complete datapack reload/registry implementation.
- RustChunk relevance: Useful override layering reference only; target datapack integration needs a rebuild.
- Decision: rewrite required; version-specific; rebuild 1.20.1 datapack overlay and registry reload
- 1.20.1 implications: Compare 1.20.1 ResourceManager/RegistryOps/HolderSet/tag reload and world preset/dimension codec behavior; Forge adds lifecycle hooks.
- Follow-up checks: Test pack priority, tag replace/append, malformed and unreadable resources, dimension bounds, newly added biome references and atomic reload failures.
- RustChunk implementation mapping: Target PackResources loader and staged immutable registry/data bundle with explicit reload lifecycle.
- Evidence required before adoption: 1.20.1 datapack/registry source and data resources, Forge 47 reload hooks and layered-pack integration fixtures.
- Completion state: COMPLETE — 347 logical lines through EOF; pinned source SHA and 13713 byte size match; inspected, not modified; tests not executed.

### 2661. `crates/horizon-world/src/worldgen/long_set.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `55b2b01303f08c0ea03d00868ecb1e93a77062b6`
- Size: 2656 bytes
- Coverage: Complete through EOF: 83 logical lines; full file; no gaps.
- Purpose: Provides a compact long-integer set for deduplicating packed world coordinates.
- Dependencies / cross-references: 64-bit packed positions; hash table/capacity management; deterministic membership and insertion semantics.
- Implementation / logic: Stores long keys in a primitive-oriented set with insertion, membership and growth behavior intended to avoid boxing and reduce allocation.
- Findings: Hash mixing, sentinel values, resize thresholds and duplicate insertion results can affect memory and collision performance; ordering must not be assumed.
- RustChunk relevance: Potential allocation optimization, independent of Minecraft semantics if key behavior is preserved.
- Decision: adaptable; retain only after overflow/collision and benchmark verification
- 1.20.1 implications: Confirm packed BlockPos/ChunkPos key domain and sentinel compatibility with 1.20.1 coordinates and dimension bounds.
- Follow-up checks: Test zero, negative values, min/max longs, collisions, resize boundaries, repeated inserts and remove/contains behavior if supported.
- RustChunk implementation mapping: Internal primitive LongSet for position caches and feature/structure deduplication.
- Evidence required before adoption: Source-level contract, randomized differential tests against HashSet<Long> and target packed-position range fixtures; benchmarks before adoption.
- Completion state: COMPLETE — 83 logical lines through EOF; pinned source SHA and 2656 byte size match; inspected, not modified; tests not executed.

### 2662. `crates/horizon-world/src/worldgen/material_condition.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `59572a042d7428bc284a525b9850a7fc5239a5a8`
- Size: 26951 bytes
- Coverage: Complete through EOF: 748 logical lines; full file; no gaps.
- Purpose: Defines material-condition evaluation state and compiled condition graph used by surface rules.
- Dependencies / cross-references: Material condition AST; density/noise samplers; biome and block queries; per-worker caches; surface/material context.
- Implementation / logic: Provides condition definitions and evaluation plumbing for block/biome/noise/height/water/stone-depth and composite predicates, including worker-local evaluation state and bounded caching.
- Findings: Condition evaluation order, short-circuit behavior, cache key identity and world-column context are observable. The material-rule schema is later than 1.20.1 in places.
- RustChunk relevance: Architectural reference for a compiled surface-condition engine.
- Decision: rewrite required; rebuild 1.20.1 SurfaceRules condition types and evaluation context
- 1.20.1 implications: Compare complete 1.20.1 SurfaceRules.ConditionSource and SurfaceRules.Context, including preliminary surface level and updateXZ/vertical scan semantics.
- Follow-up checks: Golden truth tables across Y, biome, noise, water, stone depth, floor/ceiling and repeated-context cache reuse.
- RustChunk implementation mapping: Target SurfaceRules condition AST/compiler with per-column context and target density/biome samplers.
- Evidence required before adoption: 1.20.1 SurfaceRules source and vanilla noise settings; Java reference traces for all condition types and evaluation order.
- Completion state: COMPLETE — 748 logical lines through EOF; pinned source SHA and 26951 byte size match; inspected, not modified; tests not executed.

### 2663. `crates/horizon-world/src/worldgen/material_condition/codec.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aa159bd1e584aaef18e58eaacf70f82eeb297fe6`
- Size: 18913 bytes
- Coverage: Complete through EOF: 468 logical lines; full file; no gaps.
- Purpose: Decodes and encodes material-condition JSON definitions.
- Dependencies / cross-references: Raw JSON; condition type identifiers; density/noise holders; biome tags; vertical anchors; named condition references.
- Implementation / logic: Maps material-condition serializers into a typed definition graph, parses nested composites and typed parameters, and preserves references to named condition/noise resources.
- Findings: Accepted type names and field shapes are snapshot-specific; decode depth/node limits and alternative precedence must match target codecs.
- RustChunk relevance: Useful codec architecture; target serializers require reconstruction.
- Decision: rewrite required; rebuild target SurfaceRules condition codecs
- 1.20.1 implications: Inventory every 1.20.1 SurfaceRules condition serializer and exact codec fields; distinguish Forge additions from vanilla.
- Follow-up checks: Round-trip vanilla surface-rule JSON, fuzz invalid/missing fields, unknown holders, nested depth, duplicate names and codec error precedence.
- RustChunk implementation mapping: Versioned target MaterialCondition decoder/encoder registered with 1.20.1 density, biome and noise holders.
- Evidence required before adoption: 1.20.1 SurfaceRules codec sources, vanilla worldgen JSON, Forge serializer registration and accepted/rejected JSON fixtures.
- Completion state: COMPLETE — 468 logical lines through EOF; pinned source SHA and 18913 byte size match; inspected, not modified; tests not executed.

### 2664. `crates/horizon-world/src/worldgen/material_condition/density.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f48a41354dd2eb3b4d5fe35543307c25ed4856d8`
- Size: 12464 bytes
- Coverage: Complete through EOF: 314 logical lines; full file; no gaps.
- Purpose: Compiles density-backed material conditions and exposes efficient samples to surface evaluation.
- Dependencies / cross-references: Density graph/compiler; density volume; sampler context; column/vertical coordinates; per-worker caches.
- Implementation / logic: Binds density-function references to samplers and supports contextual or precomputed evaluation for surface-condition queries.
- Findings: The sampling coordinate transform and cache lifetime affect material decisions; target DensityFunction nodes and surface Y semantics differ.
- RustChunk relevance: Adaptable compiled-sampler boundary.
- Decision: adaptable; verify target density sampling and cache parity
- 1.20.1 implications: Compare 1.20.1 SurfaceRules.Context density access, NoiseChunk sampler positions and density cache invalidation.
- Follow-up checks: Compare scalar and bulk samples at negative/positive coordinates, minY, preliminary surface and section boundaries.
- RustChunk implementation mapping: Target density-condition binding over 1.20.1 DensityFunction and per-column surface context.
- Evidence required before adoption: 1.20.1 NoiseChunk/SurfaceRules sources and deterministic density sample fixtures.
- Completion state: COMPLETE — 314 logical lines through EOF; pinned source SHA and 12464 byte size match; inspected, not modified; tests not executed.

### 2665. `crates/horizon-world/src/worldgen/material_condition/library.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `52bd246a89e725df7f1ad27d12dce0914770e294`
- Size: 14031 bytes
- Coverage: Complete through EOF: 367 logical lines; full file; no gaps.
- Purpose: Builds a named material-condition/rule library with reference resolution and cycle detection.
- Dependencies / cross-references: Named condition/rule resources; density and noise registries; recursive graph builders; cache and depth/node bounds.
- Implementation / logic: Loads named definitions, resolves forward references, rejects cycles and compiles reusable condition/rule graphs against shared noise/density resources.
- Findings: Reference resolution and compile ordering are important; partial resource visibility and cache sharing can differ from vanilla/Forge registry bootstrapping.
- RustChunk relevance: Adaptable graph-building and validation architecture.
- Decision: adaptable; rebuild catalogue and references from target SurfaceRules
- 1.20.1 implications: Compare 1.20.1 surface-rule named holder availability, built-in rule graph construction and datapack resource replacement.
- Follow-up checks: Test forward refs, unknown names, recursive cycles, duplicate definitions, cache reuse and deterministic compile failures.
- RustChunk implementation mapping: Target SurfaceRules registry/compiler constructed atomically from target datapacks and density/noise holders.
- Evidence required before adoption: 1.20.1 SurfaceRules bootstrap/source, built-in settings and datapack reload fixtures.
- Completion state: COMPLETE — 367 logical lines through EOF; pinned source SHA and 14031 byte size match; inspected, not modified; tests not executed.

### 2666. `crates/horizon-world/src/worldgen/material_condition/noise.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `68849f17823d4f32726df888a6a36da102e0266c`
- Size: 4088 bytes
- Coverage: Complete through EOF: 118 logical lines; full file; no gaps.
- Purpose: Resolves named noises and supplies deterministic noise values to material conditions.
- Dependencies / cross-references: Noise registry; positional random seeding; noise sampler; worker-local lookup cache.
- Implementation / logic: Binds named noise identifiers to samplers and evaluates them at requested world coordinates with a cached resolver.
- Findings: Noise identity, seed derivation, coordinate scaling and float conversions affect surface selection; modern noise definitions may differ.
- RustChunk relevance: Adaptable named-noise binding utility.
- Decision: adaptable; verify target noise registry, positional RNG and scale
- 1.20.1 implications: Compare 1.20.1 SurfaceRules noise conditions and NoiseSettings/NormalNoise registry and seeding.
- Follow-up checks: Seeded sample fixtures across negative coordinates, scale boundaries and cache reuse.
- RustChunk implementation mapping: Target noise sampler registry tied to target world seed and NoiseRouter.
- Evidence required before adoption: 1.20.1 NoiseSettings/NormalNoise/SurfaceRules sources and Java sample fixtures.
- Completion state: COMPLETE — 118 logical lines through EOF; pinned source SHA and 4088 byte size match; inspected, not modified; tests not executed.

### 2667. `crates/horizon-world/src/worldgen/material_condition/rule.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c1b4b898990845e6ac3b4fb57e95441f9ed6e32b`
- Size: 15106 bytes
- Coverage: Complete through EOF: 400 logical lines; full file; no gaps.
- Purpose: Defines and evaluates compiled material rules, including sequence and ore-vein/surface variants.
- Dependencies / cross-references: MaterialRule AST; MaterialCondition; density roots; noise samplers; block-state bindings; column context.
- Implementation / logic: Compiles rule alternatives and sequences, evaluates conditions in order, selects block states and applies specialized ore-vein and surface-variant decisions.
- Findings: Short-circuit and compile timing are observable; conditions may be compiled even for branches not ultimately selected. Some modern rule variants and constants are absent in 1.20.1.
- RustChunk relevance: Architectural reference, but target rule catalogue and evaluation order must be rebuilt.
- Decision: rewrite required; reconstruct 1.20.1 SurfaceRules.RuleSource and RuleSource evaluation
- 1.20.1 implications: Compare 1.20.1 SurfaceRules rule types, sequence semantics, block-state providers, bandlands/ore vein behavior and Forge surface-rule injection.
- Follow-up checks: Seeded column fixtures for every rule type, conditional order, sequence fallthrough, ore-vein thresholds and error propagation.
- RustChunk implementation mapping: Target SurfaceRules typed AST/compiler and column mutator with 1.20.1 conditions and block registry.
- Evidence required before adoption: 1.20.1 SurfaceRules source, vanilla noise settings and Java golden surface block columns.
- Completion state: COMPLETE — 400 logical lines through EOF; pinned source SHA and 15106 byte size match; inspected, not modified; tests not executed.

### 2668. `crates/horizon-world/src/worldgen/material_extensions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8fb5735ace7a769b388e8ab6192691de2d8c1db1`
- Size: 7660 bytes
- Coverage: Complete through EOF: 229 logical lines; full file; no gaps.
- Purpose: Adds specialized material/surface extension behavior for badlands and frozen-ocean terrain.
- Dependencies / cross-references: Biome IDs; noise samplers; height/material context; clay bands and ice block states; chunk column writer.
- Implementation / logic: Applies badlands pillar and frozen-ocean iceberg-like extensions using biome/height/noise inputs and ordered block placement.
- Findings: These are later-version extensions, not automatically part of 1.20.1 surface rules; added effects and block placement order alter terrain if copied.
- RustChunk relevance: Reference only for optional extension architecture.
- Decision: rewrite required; only adopt if target-version behavior includes the corresponding extension
- 1.20.1 implications: Compare 1.20.1 SurfaceRules and biome surface settings; do not import later extension behavior as vanilla parity.
- Follow-up checks: Verify target biomes, configured noise, clay-band/ice geometry, seeded column results and Forge biome modifiers.
- RustChunk implementation mapping: Optional version-gated material extension registry outside the vanilla 1.20.1 baseline.
- Evidence required before adoption: 1.20.1 Mojang source and datapack data proving each behavior, plus Java golden columns and explicit feature/version gating.
- Completion state: COMPLETE — 229 logical lines through EOF; pinned source SHA and 7660 byte size match; inspected, not modified; tests not executed.

### 2669. `crates/horizon-world/src/worldgen/material_stage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `121573ba498f7e6aaa6b0196093bef07df2cf6b2`
- Size: 15122 bytes
- Coverage: Complete through EOF: 426 logical lines; full file; no gaps.
- Purpose: Runs the surface material stage over generated chunk columns.
- Dependencies / cross-references: Material rules/conditions; biome and density samplers; block-state registry; material extensions; chunk sections and fluid postprocessing.
- Implementation / logic: Traverses columns and vertical positions, evaluates compiled rules, applies material variants and extension logic and records postprocessing where required.
- Findings: Column/vertical traversal direction, context update, bottom bound and side-effect ordering matter. A modern hard-coded lower scan bound is not evidence for 1.20.1 dimension limits.
- RustChunk relevance: Core architecture reference for a separate surface-material stage.
- Decision: rewrite required; rebuild the 1.20.1 surface replacement stage
- 1.20.1 implications: Compare 1.20.1 NoiseBasedChunkGenerator surface build, SurfaceSystem.buildSurface, SurfaceRules.Context and dimension minY handling.
- Follow-up checks: Golden columns across Overworld/Nether/End and custom dimensions; compare preliminary surface, water depth, bedrock, fluid postprocessing and rule short-circuiting.
- RustChunk implementation mapping: Target SurfaceSystem and per-column SurfaceRules evaluator with Forge rule injection.
- Evidence required before adoption: 1.20.1 SurfaceSystem/SurfaceRules/NoiseBasedChunkGenerator source, datapack settings and Java block-column goldens.
- Completion state: COMPLETE — 426 logical lines through EOF; pinned source SHA and 15122 byte size match; inspected, not modified; tests not executed.

### 2670. `crates/horizon-world/src/worldgen/material_surface.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c463ec06367bcca85bbb8548953f78cd9c997716`
- Size: 5283 bytes
- Coverage: Complete through EOF: 142 logical lines; full file; no gaps.
- Purpose: Builds and samples layered clay bands used by surface extensions.
- Dependencies / cross-references: LegacyRandom; clay-band block states; positional coordinates; noise/offset selection.
- Implementation / logic: Creates a randomized palette and thickness sequence of clay bands, then resolves a band for a given vertical or noise-derived coordinate.
- Findings: Band initialization order, random draws, thickness boundaries and indexing behavior are deterministic terrain inputs.
- RustChunk relevance: Adaptable small deterministic surface-data utility.
- Decision: adaptable; verify target ClayBands generation and indexing
- 1.20.1 implications: Compare 1.20.1 ClayBands initialization, random seed, palette distribution, thickness and block lookup.
- Follow-up checks: Golden band arrays across world seeds and boundary-Y queries, including wrap/negative positions.
- RustChunk implementation mapping: Target ClayBands table generated from the exact 1.20.1 seed and random implementation.
- Evidence required before adoption: 1.20.1 ClayBands source and deterministic seed-to-array fixtures.
- Completion state: COMPLETE — 142 logical lines through EOF; pinned source SHA and 5283 byte size match; inspected, not modified; tests not executed.

### 2671. `crates/horizon-world/src/worldgen/ore_feature.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d69874d76fd445dcf434c1a0eb304ec612e96f5a`
- Size: 14811 bytes
- Coverage: Complete through EOF: 370 logical lines; full file; no gaps.
- Purpose: Implements ore-vein and scattered ore feature placement.
- Dependencies / cross-references: BlockStateProvider; target/replace predicates; height and biome conditions; RandomSource; section-level writes.
- Implementation / logic: Builds irregular ore veins from interpolated ellipsoids along a sampled line, evaluates replace rules and writes eligible blocks; supports scattered ore-style placement.
- Findings: Random stream, ellipsoid overlap, candidate iteration, state-provider sampling and direct section mutation order determine ore distribution; bypassed side effects need explicit handling.
- RustChunk relevance: Adaptable ore feature architecture.
- Decision: adaptable; verify target OreFeature/ScatteredOreFeature geometry and replacement behavior
- 1.20.1 implications: Compare 1.20.1 OreFeature and ScatteredOreFeature, vein length/radius equations, replace predicates, discard chance, and setBlock flags.
- Follow-up checks: Seeded golden placements across vein lengths, radii, air exposure, negative coordinates, biome checks and chunk boundaries.
- RustChunk implementation mapping: Target OreFeature and ScatteredOreFeature over target block provider, predicate and chunk mutation APIs.
- Evidence required before adoption: 1.20.1 ore feature sources, configured-feature JSON, Forge ore-generation hooks and Java placement goldens.
- Completion state: COMPLETE — 370 logical lines through EOF; pinned source SHA and 14811 byte size match; inspected, not modified; tests not executed.

### 2672. `crates/horizon-world/src/worldgen/overrides.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0a20987a32580fbf097f54871e8ca8be8e5f4073`
- Size: 19142 bytes
- Coverage: Complete through EOF: 478 logical lines; full file; no gaps.
- Purpose: Collects and merges live datapack overrides for world-generation resource bundles and tags.
- Dependencies / cross-references: Pack path classifier; namespaces/resource IDs; dimension/type JSON; biome, noise, density, feature, carver and structure bundles; tag packs.
- Implementation / logic: Processes packs low-to-high, replaces ordinary entries with the highest pack, appends tag contributions in pack order, resolves dimension height and removes unsupported new biome references from tags and climate points.
- Findings: The folder-kind catalogue explicitly uses Java 26.3 names and leaves new biomes out; dimension validation and tag replace semantics must be target-version exact. Embedded tests were inspected, not run.
- RustChunk relevance: Adaptable pack-merging architecture; target codecs and reload lifecycle require rebuild.
- Decision: rewrite required; version-specific; rebuild 1.20.1 datapack overlay and registry reload
- 1.20.1 implications: Compare 1.20.1 datapack directory names, ResourceManager ordering, RegistryOps, HolderSet, tag replace semantics, dimension codecs and Forge reload hooks.
- Follow-up checks: Fixtures for low/high pack override, additive/replace tags, malformed/unreadable files, custom dimension bounds, biome additions and failed atomic reload.
- RustChunk implementation mapping: Target PackResources/ResourceManager layer producing immutable 1.20.1 registry bundles and staged reload.
- Evidence required before adoption: 1.20.1 ResourceManager/RegistryDataLoader/HolderSet sources, official data folders, Forge 47 reload patches and layered-pack fixtures.
- Completion state: COMPLETE — 478 logical lines through EOF; pinned source SHA and 19142 byte size match; inspected, not modified; tests not executed.

### 2673. `crates/horizon-world/src/worldgen/placement.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dd774f3160d235164b6985f672ef4b373e8afe00`
- Size: 20333 bytes
- Coverage: Complete through EOF: 505 logical lines; full file; no gaps.
- Purpose: Decodes and applies placed-feature position modifiers.
- Dependencies / cross-references: HeightProvider/IntProvider; block predicates; HeightmapType; PlacementContext; biome noise; RandomSource.
- Implementation / logic: Supports biome/block filters, environment scans, heightmap and surface thresholds, water depth, count/noise counts, square/cuboid offsets, height ranges, rarity/chance, fixed and randomly selected placement.
- Findings: Modifier order, random draw consumption, emission order, inclusive bounds, short-circuiting and work budgets are observable. The catalogue includes newer/custom modifiers and bounds not automatically equivalent to 1.20.1.
- RustChunk relevance: Core placement modifier reference; target serializers and semantics need exact rebuild.
- Decision: rewrite required; rebuild the 1.20.1 PlacementModifier catalogue and execution
- 1.20.1 implications: Compare every 1.20.1 PlacementModifier serializer and runtime method, including environment scan limits, count providers, heightmap names and Forge-added modifiers.
- Follow-up checks: Seeded traces for each modifier, chained ordering, duplicate positions, boundary heights, failure after partial emission and work-limit behavior.
- RustChunk implementation mapping: Target PlacementModifier AST/codec and Java-compatible sequential execution over world query adapters.
- Evidence required before adoption: 1.20.1 PlacementModifier source/mappings, vanilla placed-feature JSON, Forge placement extensions and Java golden position streams.
- Completion state: COMPLETE — 505 logical lines through EOF; pinned source SHA and 20333 byte size match; inspected, not modified; tests not executed.

### 2674. `crates/horizon-world/src/worldgen/placement/chain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `63d526256a858643484cfa958fcbb107663d3920`
- Size: 9889 bytes
- Coverage: Complete through EOF: 270 logical lines; full file; no gaps.
- Purpose: Executes ordered placement-modifier chains with reusable pending buffers.
- Dependencies / cross-references: PlacementModifier; PlacementContext; FeatureIdentity; RandomSource; pending and emitted-position buffers.
- Implementation / logic: Applies one modifier to all current positions before descendants, preserves emitted sibling order, shares random/world context and supports root biome checks with a bounded work budget.
- Findings: Buffer contents persist after errors, so reusing a placer after failure can retain partial state. Nested feature callbacks intentionally do not inherit the root top-feature context.
- RustChunk relevance: Useful execution architecture, but native callback ordering must be proven.
- Decision: adaptable; verify Java 1.20.1 PlacedFeature modifier chain order and failure cleanup
- 1.20.1 implications: Compare 1.20.1 PlacedFeature.placeWithContext and modifier nesting, random stream sharing and biome-filter context.
- Follow-up checks: Fixtures with branching modifiers, nested features, partial callback failure, reused placer, equal sibling ordering and budget exhaustion.
- RustChunk implementation mapping: Target placed-feature executor with explicitly scoped biome context and deterministic queue semantics.
- Evidence required before adoption: 1.20.1 PlacedFeature source/mappings, Forge hooks and seeded modifier chain traces.
- Completion state: COMPLETE — 270 logical lines through EOF; pinned source SHA and 9889 byte size match; inspected, not modified; tests not executed.

### 2675. `crates/horizon-world/src/worldgen/placement/context.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4a9ea0464123e0ac9e6447b9c8e248d040f852ec`
- Size: 14852 bytes
- Coverage: Complete through EOF: 381 logical lines; full file; no gaps.
- Purpose: Defines world-query and mutation contracts used by feature placement.
- Dependencies / cross-references: Block/chunk/section access; heightmaps; biome/fluid queries; block entities; scheduled ticks; postprocessing; generation bounds.
- Implementation / logic: Exposes typed callbacks for survival, support, collision, state/tag lookup, biome/weather, fluids, block-entity and entity setup, write permissions, chunk/section writes and tick/postprocessing scheduling.
- Findings: The trait carefully separates raw section writes from ChunkAccess/WorldGenRegion effects; many callbacks default to unavailable or no-op, so host adapters must implement required operations explicitly.
- RustChunk relevance: Important interface boundary for separating Rust feature logic from Minecraft world mutation.
- Decision: adaptable; rebuild adapters to match 1.20.1 ChunkAccess/WorldGenRegion and Forge side effects
- 1.20.1 implications: Compare each callback to 1.20.1 BlockGetter/LevelWriter/WorldGenRegion/ProtoChunk and relevant block-entity, POI, lighting and tick APIs.
- Follow-up checks: Contract tests for every callback, absent chunk/state, write-zone denial, update flags, raw-vs-normal write side effects and error propagation.
- RustChunk implementation mapping: Target MinecraftWorldgenAccess adapter over ChunkAccess/ProtoChunk/WorldGenRegion with explicit effect scheduling.
- Evidence required before adoption: 1.20.1 Mojang-mapped world/chunk interfaces, Forge 47 patches and callback-level integration fixtures.
- Completion state: COMPLETE — 381 logical lines through EOF; pinned source SHA and 14852 byte size match; inspected, not modified; tests not executed.


### 2676. `crates/horizon-world/tests/movement_properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e6e84bdc4bc1c60672e3502e56c3b3dd64372fc5`
- Size: 4933 bytes
- Coverage: Complete through EOF: 144 logical lines; full file; no gaps.
- Purpose: Movement-property registry decoding and tag-binding regression tests.
- Dependencies / cross-references: MovementProperties; BlockRegistry; MovementTags; RegistryMovementWorld; BlockPos; NBT Tag.
- Implementation / logic: Builds a two-state NBT fixture and checks malformed/version-mismatched definitions, tag rebinding, coefficient bit preservation and distinct provider/data errors.
- Findings: Tests preserve negative-zero speed factor and distinguish missing-chunk provider errors from unknown-state data errors. They are fixtures only and do not prove target Java/Forge parity.
- RustChunk relevance: Reference for designing movement-property validation and world-query error contracts.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Pinned DATA_VERSION and registry schema are later-version inputs; 1.20.1 block movement properties and Forge-added tags require independent reconstruction.
- Follow-up checks: Compare all movement coefficients, default values, tag reloads, unknown states and provider errors with 1.20.1/Forge 47.
- RustChunk implementation mapping: RustChunk-owned movement-property codec and immutable registry-bound snapshot tests.
- Evidence required before adoption: 1.20.1 source/mappings, Forge 47 tag lifecycle, malformed NBT corpus and seeded numeric differential fixtures.
- Completion state: COMPLETE — 144 logical lines through EOF; pinned source SHA and 4933-byte size match; inspected, not modified; tests not executed.

### 2677. `crates/horizon-world/tests/native_load_hooks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `626d18f3267215825cbc7f92f7ff50dd981b8044`
- Size: 2009 bytes
- Coverage: Complete through EOF: 54 logical lines; full file; no gaps.
- Purpose: Native loot/item hook installation and callback dispatch tests.
- Dependencies / cross-references: native_load_hooks; loot; item_defaults; serde_json; NativeLootHooks.
- Implementation / logic: Checks no-op behavior before install, first successful installation, rejection of a second install, and callback effects on loot tables, item components, loot functions and conditions.
- Findings: The test asserts single-install semantics and first-callback retention; it does not exercise concurrent installation, callback reentrancy, poisoned state or Android dynamic-library loading.
- RustChunk relevance: Reference only for optional native-mod hook lifecycle; this is not Forge event registration.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Starling native hook API is not Forge 47's mod/event lifecycle and must not be treated as a Forge mod loader.
- Follow-up checks: Inspect the implementation's synchronization and failure behavior; separately map any needed hooks to Forge 47 event buses and lifecycle.
- RustChunk implementation mapping: Keep any native hook bridge isolated behind a RustChunk-owned optional extension API; do not expose it as Forge compatibility.
- Evidence required before adoption: Pinned implementation review, thread/race tests, Android ARM64 loader evidence and Forge 47 event integration tests.
- Completion state: COMPLETE — 54 logical lines through EOF; pinned source SHA and 2009-byte size match; inspected, not modified; tests not executed.

### 2678. `crates/horizon-world/tests/placement_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ca9610ceb6d7336b7b975afa866ba2b28fc5318b`
- Size: 1917 bytes
- Coverage: Complete through EOF: 58 logical lines; full file; no gaps.
- Purpose: Placement-modifier negative-extent and world-query work-budget tests.
- Dependencies / cross-references: PlacementModifier; IntProvider; PlacementContext; WorldGenerationContext; HeightmapType; LegacyRandom.
- Implementation / logic: Checks a negative cuboid Y extent emits no positions even with huge XZ extents, and an extreme column height returns WorkLimit after a bounded number of queries.
- Findings: The test expects a 65,536-query ceiling for the layered scan. It does not execute the test or establish the same bound/semantics in Java 1.20.1.
- RustChunk relevance: Useful regression scenarios for bounded worldgen scans and malformed provider values.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Placement modifier catalogue and worldgen height semantics are snapshot-specific; compare against 1.20.1 serializers and execution.
- Follow-up checks: Verify negative/zero extents, extreme heights, cancellation/error propagation and exact query counts against target sources.
- RustChunk implementation mapping: Add bounded 1.20.1 placement scan tests around a RustChunk world-query adapter.
- Evidence required before adoption: 1.20.1 PlacementModifier sources, seeded oracle traces and boundary/overflow corpus.
- Completion state: COMPLETE — 58 logical lines through EOF; pinned source SHA and 1917-byte size match; inspected, not modified; tests not executed.

### 2679. `crates/horizon-world/tests/placement_queries.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e49bfd8ae5a18d45f359ec83fa2060bdb1123e33`
- Size: 1578 bytes
- Coverage: Complete through EOF: 45 logical lines; full file; no gaps.
- Purpose: Surface-water-depth placement query ordering and failure short-circuit test.
- Dependencies / cross-references: PlacementModifier::SurfaceWaterDepth; PlacementContext; HeightmapType; LegacyRandom.
- Implementation / logic: Records OceanFloor then WorldSurface query order, forces the second query to fail, and checks no emission or random draw occurs.
- Findings: The test pins callback order and random-stream non-consumption on a query failure; only a mock context is covered.
- RustChunk relevance: Useful deterministic test shape for worldgen callback ordering.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Heightmap names and modifier behavior must be checked against 1.20.1 and Forge additions.
- Follow-up checks: Compare query order, failure point, output positions and random state against Java 1.20.1.
- RustChunk implementation mapping: Use a scripted RustChunk placement context with recorded query and RNG traces.
- Evidence required before adoption: 1.20.1 placement source and seeded Java-vs-Rust trace corpus.
- Completion state: COMPLETE — 45 logical lines through EOF; pinned source SHA and 1578-byte size match; inspected, not modified; tests not executed.

### 2680. `crates/horizon-world/tests/retrogen_biomes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c83b77d93b1278e395ebff66dc522dc457512cb6`
- Size: 3409 bytes
- Coverage: Complete through EOF: 117 logical lines; full file; no gaps.
- Purpose: Retrogen biome selection precedence and section-atomic fill tests.
- Dependencies / cross-references: RetrogenBiomeResolver; BiomeChunk; ChunkPos; BiomeFillError; RefCell.
- Implementation / logic: Checks candidate biome before old-column fallback, error short-circuiting, cave-biome retention, and that a failed section fill does not partially replace previously stored columns/sections.
- Findings: The fixture covers selected candidate/old-column paths and one failure point (the 66th resolver call); it is not a complete 1.20.1 retrogen pipeline.
- RustChunk relevance: Reference for safe biome upgrade transactions and preservation of pre-existing chunk data.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Retrogen behavior, biome quart coordinates, section layout and data version differ by Minecraft version; verify against 1.20.1 chunk/biome upgrade logic.
- Follow-up checks: Test all section boundaries, negative coordinates, mixed retained/new cave biomes and failures at every column index.
- RustChunk implementation mapping: Implement retrogen as staged per-section updates with explicit 1.20.1 biome registry and quart-coordinate rules.
- Evidence required before adoption: 1.20.1 chunk upgrade sources, real saved-chunk fixtures and failure-injection tests.
- Completion state: COMPLETE — 117 logical lines through EOF; pinned source SHA and 3409-byte size match; inspected, not modified; tests not executed.

### 2681. `crates/horizon-world/tests/rule_test_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3f614590d9767ff9140403d28e05fd344d27b75`
- Size: 5126 bytes
- Coverage: Complete through EOF: 146 logical lines; full file; no gaps.
- Purpose: Rule-test recursive decode budgets, tag binding and matter-phase semantics tests.
- Dependencies / cross-references: RuleTest; FeatureBlockBindings; BlockRegistry; NBT Tag; BTreeMap/BTreeSet; LegacyRandom.
- Implementation / logic: Builds synthetic block registries and tests recursion/branching limits, unknown and duplicate canonical tags, air/liquid/rail/chain matter phases, inversion and invalid enum inputs.
- Findings: Tests depend on a synthetic registry and a later-version custom matter_phase predicate; they do not prove 1.20.1 rule serializer or block-material parity.
- RustChunk relevance: Reference for bounded recursive decoders and explicit registry/tag validation.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Custom matter-phase predicates and block classifications are not automatically vanilla 1.20.1 or Forge 47 behavior.
- Follow-up checks: Rebuild target RuleTest serializers; test depth/node limits, namespace canonicalization and block material flags against target registries.
- RustChunk implementation mapping: RustChunk 1.20.1 rule-test AST with bounded decode context and versioned block-property snapshots.
- Evidence required before adoption: 1.20.1/Forge 47 predicate sources, canonical registry data and malformed nested JSON corpus.
- Completion state: COMPLETE — 146 logical lines through EOF; pinned source SHA and 5126-byte size match; inspected, not modified; tests not executed.

### 2682. `crates/horizon-world/tests/state_provider_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `23553e6b78d9345806cac3abcc2df5636fd38098`
- Size: 3904 bytes
- Coverage: Complete through EOF: 87 logical lines; full file; no gaps.
- Purpose: Block-state provider recursion, aggregate budgets, tags and catalogue resolution tests.
- Dependencies / cross-references: BlockStateProvider; StateProviderBindings; StateProviderCatalog; BlockRegistry; serde_json; BTreeMap/BTreeSet.
- Implementation / logic: Checks recursive depth and list caps, aggregate predicate/provider node budget, shared-tag expansion cap, duplicate tag names, cycles, invalid definitions and direct/alias resolution.
- Findings: Boundary tests deliberately accept a list of 8,192 entries and reject 8,193; resource catalog fixtures are synthetic and version-bound.
- RustChunk relevance: Useful patterns for bounded data-driven block-state providers and registry snapshot binding.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Provider types, JSON schemas and tag contents must be rebuilt for Java 1.20.1 and Forge 47 dynamic registries.
- Follow-up checks: Compare every 1.20.1 provider serializer, recursive limits, tag expansion and cycle detection with target resources.
- RustChunk implementation mapping: Versioned RustChunk BlockStateProvider decoder using one shared work-budget object per root decode.
- Evidence required before adoption: 1.20.1 provider sources, Forge registry fixtures and adversarial recursion/branching corpus.
- Completion state: COMPLETE — 87 logical lines through EOF; pinned source SHA and 3904-byte size match; inspected, not modified; tests not executed.

### 2683. `crates/horizon-world/tests/tick_data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7060b57a4d1e6d01429d9925ae4c8ce1cc4e0cdb`
- Size: 3533 bytes
- Coverage: Complete through EOF: 117 logical lines; full file; no gaps.
- Purpose: Saved block/fluid tick NBT codec, coordinate filtering and atomic chunk update tests.
- Dependencies / cross-references: tick_data; SavedTick; TickPriority; TickTypeId; ChunkData; NamedTag; NBT Tag.
- Implementation / logic: Exercises numeric narrowing/coercion, invalid list/type handling, chunk-coordinate filtering, tick unpack/pack, failed writes without mutation and preservation of unrelated NBT fields.
- Findings: Fixtures include NaN/infinity and extreme numeric inputs; expected priority/coordinate conversions are pinned to this snapshot. No runtime tick scheduler or 1.20.1 codec is exercised.
- RustChunk relevance: Strong reference for saved scheduled-tick persistence and transactional chunk edits.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Tick NBT field names, numeric conversions and priority enum mapping must match 1.20.1 saved data and Forge tick types.
- Follow-up checks: Compare malformed/legacy tick tags, negative chunk coordinates, duplicate ticks, priority ordering and unknown fields with 1.20.1.
- RustChunk implementation mapping: RustChunk versioned tick-data codec plus atomic ChunkData mutation; keep scheduler separate.
- Evidence required before adoption: 1.20.1 TickListChunk/ProtoChunk sources, real chunk NBT corpus and differential round-trip tests.
- Completion state: COMPLETE — 117 logical lines through EOF; pinned source SHA and 3533-byte size match; inspected, not modified; tests not executed.

### 2684. `crates/horizon-world/tests/weighted_provider.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `574e1bb4c39bfd9b2cd79fd675d32cf826b295c4`
- Size: 2292 bytes
- Coverage: Complete through EOF: 53 logical lines; full file; no gaps.
- Purpose: Weighted-list validation, aggregate decoder budgets and optional zero-weight sampling tests.
- Dependencies / cross-references: WeightedList; OptionalWeightedList; IntProvider; HeightProvider; LegacyRandom.
- Implementation / logic: Checks invalid/overflowing total weights, 8,192-entry cap, shared nested node budget and zero-weight optional choices without random draws.
- Findings: The exact allocation and aggregate-node thresholds are implementation policy, not evidence of vanilla 1.20.1 limits. Embedded tests were not executed.
- RustChunk relevance: Reference for safe weighted sampling and deterministic no-draw cases.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Provider schemas, weight overflow and random draw behavior must be compared with 1.20.1 Java implementations.
- Follow-up checks: Verify zero-weight semantics, total overflow, nested budgets and seeded draw counts against 1.20.1.
- RustChunk implementation mapping: RustChunk weighted provider with checked totals and target-specific decode budget.
- Evidence required before adoption: 1.20.1 provider codecs, Java seeded traces and adversarial allocation tests.
- Completion state: COMPLETE — 53 logical lines through EOF; pinned source SHA and 2292-byte size match; inspected, not modified; tests not executed.

### 2685. `crates/horizon-world/tests/world_io.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c7cb2887e2cc22552385e0752ae3c96d47ae1397`
- Size: 15656 bytes
- Coverage: Complete through EOF: 413 logical lines; ranges 1–400, 401–413; contiguous, no gaps.
- Purpose: Region-file sector allocation, external chunk records, chunk NBT edits and compression regression tests.
- Dependencies / cross-references: RegionFile/RegionWriter; ChunkRecord; ChunkData; PalettedContainer; Compression; NBT Limits; filesystem and temporary directories.
- Implementation / logic: Covers external-chunk threshold, negative coordinates, timestamps, in-place sector reuse, writer rollback, collision with existing destinations, corrupt headers/payloads, block edits and compression tails; includes one ignored microbenchmark.
- Findings: The fixture explicitly documents and accepts overlapping sector allocations between chunks (two entries survive), a potential corruption hazard requiring independent review. Tests also pin behavior where bytes after the decoded NBT root are ignored; comments identify Java 26.3 behavior. The benchmark is ignored and was not run.
- RustChunk relevance: High-value test scenarios for Anvil region I/O, crash-safe writes and chunk NBT preservation; not implementation proof.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Region-file allocation and NBT/compression behavior must be checked against 1.20.1 RegionFile/RegionFileStorage and Forge patches; do not inherit 26.3 assumptions.
- Follow-up checks: Review overlapping-sector handling, header validation, external .mcc lifecycle, truncation, trailing compressed data and crash recovery against 1.20.1.
- RustChunk implementation mapping: RustChunk region storage with sector-overlap rejection, transactional writes and a versioned chunk NBT codec.
- Evidence required before adoption: 1.20.1 Java RegionFile/RegionFileStorage sources, malformed .mca corpus, fault-injected writes and round-trip differential tests.
- Completion state: COMPLETE — 413 logical lines through EOF; pinned source SHA and 15656-byte size match; inspected, not modified; tests not executed.

### 2686. `crates/horizon-world/tests/world_movement.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3937448a0c0dfab42ce3d7794cc758b476065731`
- Size: 4315 bytes
- Coverage: Complete through EOF: 132 logical lines; full file; no gaps.
- Purpose: Movement collision-scan candidate budget and full-sweep behavior tests.
- Dependencies / cross-references: scan_cells; resolve_world_movement; BlockCollisionWorld; EntityMovementWorld; CollisionContext; AABB/VoxelShape.
- Implementation / logic: Uses open-air mock worlds to show diagonal movement beyond the candidate budget is still fully scanned and ordinary movement results remain budget-independent.
- Findings: The test explicitly asserts that a 40×40-block sweep proceeds despite candidate_budget=10; the budget is not a hard cap on movement work. This may preserve a vanilla walk but exposes a potential unbounded-work/performance risk for extreme movement requests.
- RustChunk relevance: Critical performance and robustness test for collision scanning on mobile; test-only evidence, not proof of production exploitability.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Reconcile exact 1.20.1 collision walk with Forge entity/block hooks; target may require safe movement bounds without changing normal collision semantics.
- Follow-up checks: Trace caller-supplied movement bounds, maximum legitimate velocity/teleport/knockback, chunk residency and work amplification; compare with Java 1.20.1.
- RustChunk implementation mapping: Separate a validated movement-distance policy from collision candidate accounting; preserve normal Java axis/order semantics.
- Evidence required before adoption: 1.20.1 Entity.collide sources, Forge movement hooks, extreme-coordinate tests and ARM64 profiling.
- Completion state: COMPLETE — 132 logical lines through EOF; pinned source SHA and 4315-byte size match; inspected, not modified; tests not executed.

### 2687. `crates/horizon-world/tests/world_ray_limits.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cc999867bce020da8f9865954bb8c6e1aa566d32`
- Size: 2818 bytes
- Coverage: Complete through EOF: 101 logical lines; full file; no gaps.
- Purpose: Ray traversal visit-limit, non-finite input and world-provider error tests.
- Dependencies / cross-references: clip_world; RayWorld; RayError; TraversalError; RayBlockMode; RayFluidMode; Vec3.
- Implementation / logic: Checks visit-budget exhaustion is returned as an error rather than a miss, non-finite rays fail before world reads, and provider errors remain distinguishable.
- Findings: Tests use a minimal scripted world with fluid mode disabled; they do not cover full block/fluid shape selection or target-version hit ordering.
- RustChunk relevance: Reference for bounded ray traversal and explicit query-error semantics.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Ray traversal, voxel clipping and interaction-face semantics must be reconstructed from 1.20.1 ClipContext/VoxelShape.
- Follow-up checks: Compare visit budget, NaN/infinity, tie-breaking, fluid modes and world-query failures with 1.20.1.
- RustChunk implementation mapping: RustChunk ray service with finite-input validation and distinct traversal/world error types.
- Evidence required before adoption: 1.20.1 ray/shape sources, Forge query hooks and differential ray corpus.
- Completion state: COMPLETE — 101 logical lines through EOF; pinned source SHA and 2818-byte size match; inspected, not modified; tests not executed.

### 2688. `crates/starling-account/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `65ffb123011d8b4e968584856b5896d8f890eca0`
- Size: 764 bytes
- Coverage: Complete through EOF: 20 logical lines; full file; no gaps.
- Purpose: Cargo package and dependency definition for Starling's account, file-download and Android Java-runtime subsystem.
- Dependencies / cross-references: Workspace edition/license/MSRV; ureq; serde/serde_json; sha1/sha2; lzma-rs; tar; getrandom.
- Implementation / logic: Declares a non-published workspace crate and its HTTP, serialization, hashing, archive and random-number dependencies.
- Findings: No Android Keystore provider or platform-specific dependency is declared here; secure persistence must be supplied by the host SecretStore. The dependency set does not establish compatibility with the RustChunk toolchain.
- RustChunk relevance: Reference for separating account/download/runtime responsibilities into RustChunk-owned crates.
- Decision: adaptable; reference; dependency baseline only
- 1.20.1 implications: Workspace edition and rust-version are inherited; pin and audit exact crate versions/features for Android ARM64 and the target build toolchain.
- Follow-up checks: Inspect lockfile resolution, TLS backend, Android ABI builds, archive parser behavior and SecretStore host implementation.
- RustChunk implementation mapping: Split RustChunk account/session, verified downloads and runtime installation behind independent platform interfaces.
- Evidence required before adoption: Pinned Cargo.lock, dependency license/security review, Android ARM64 build and Keystore integration tests.
- Completion state: COMPLETE — 20 logical lines through EOF; pinned source SHA and 764-byte size match; inspected, not modified; tests not executed.

### 2689. `crates/starling-account/src/ely.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `39f5a23e5e5bbc96002506ed6a5e19bcf5d18a7d`
- Size: 10210 bytes
- Coverage: Complete through EOF: 260 logical lines; full file; no gaps.
- Purpose: Ely.by Yggdrasil authentication, refresh, profile validation and skin/cape lookup.
- Dependencies / cross-references: ureq; Account/Secrets/Provider; SecretStore; getrandom; JSON helpers; Ely.by auth and texture endpoints.
- Implementation / logic: Sends password with optional six-digit OTP convention, creates a random client token, validates returned UUID/name/access token, refreshes sessions and fetches texture metadata.
- Findings: Redirects are disabled and password is request-scoped; profile UUID/name are validated. Texture URLs are accepted from the response without scheme/host validation, and texture lookup failures silently leave skin/cape absent. Ely.by credentials are still transmitted to a third-party auth service.
- RustChunk relevance: Optional third-party account-provider reference; not Microsoft authentication or Forge account handling.
- Decision: adaptable; optional; third-party provider
- 1.20.1 implications: Ely.by/Yggdrasil session semantics differ from Microsoft online-mode and may not authenticate to Mojang online-mode servers.
- Follow-up checks: Verify current provider API/terms, URL validation, token expiry/refresh semantics and server-side compatibility; isolate credentials and opt-in UX.
- RustChunk implementation mapping: Optional provider adapter behind a RustChunk auth-provider trait; never treat Ely token as Microsoft/Mojang session.
- Evidence required before adoption: Current provider documentation, security review, TLS/redirect tests and target server authentication traces.
- Completion state: COMPLETE — 260 logical lines through EOF; pinned source SHA and 10210-byte size match; inspected, not modified; tests not executed.

### 2690. `crates/starling-account/src/files.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7e0b504d09039e0dd34175a56b950ada077a1efb`
- Size: 15664 bytes
- Coverage: Complete through EOF: 436 logical lines; ranges 1–400, 401–436; contiguous, no gaps.
- Purpose: Pinned Minecraft manifest, client/server/library/asset downloader with hash verification, retry and progress reporting.
- Dependencies / cross-references: research-lock.json pins; Mojang piston-meta/resources endpoints; ureq Agent; SHA-1; filesystem; four scoped download workers; cancellation/progress callbacks.
- Implementation / logic: Accepts only pinned version manifests, checks client/server hashes, filters library OS rules, validates asset object hashes, writes .part files and downloads concurrently with retries.
- Findings: Response bodies are read without a size cap; manifest library paths are filtered with substring `..` but not validated component-by-component against absolute/rooted paths; duplicate library destinations are not deduplicated. Pinned metadata narrows exposure but does not replace path and resource limits. Cancellation is checked between queued files, not during each library/asset response.
- RustChunk relevance: Useful design reference for Mojang-layout file acquisition, resumable verified downloads and progress; pinned Starling lock is not RustChunk's target version.
- Decision: adaptable; reference; harden before reuse
- 1.20.1 implications: research-lock pins Java 26.3; RustChunk requires 1.20.1 manifest/client/libraries/assets and Forge 47 artifacts. SHA-1 compatibility is required by Mojang metadata but should not be the only trust boundary for RustChunk-owned artifacts.
- Follow-up checks: Add bounded streaming reads, path-component containment, destination deduplication, declared-size checks and cancellation-aware streams; verify 1.20.1 manifest and Forge libraries.
- RustChunk implementation mapping: RustChunk verified downloader with immutable target-version manifest, strict root-relative path validator and bounded worker queue.
- Evidence required before adoption: Pinned 1.20.1 manifest, malicious path/oversized-body corpus, interrupted-download tests and Android filesystem tests.
- Completion state: COMPLETE — 436 logical lines through EOF; pinned source SHA and 15664-byte size match; inspected, not modified; tests not executed.

### 2691. `crates/starling-account/src/files/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9287183e4ce9f8300dc7481bbce8354d159a0dfe`
- Size: 9149 bytes
- Coverage: Complete through EOF: 244 logical lines; full file; no gaps.
- Purpose: Local mock-server tests for manifest pinning, verified download layout, retries and resumability.
- Dependencies / cross-references: files::{download,Pins,Sources,Wanted,Progress}; local TCP listener; SHA-1; temporary filesystem.
- Implementation / logic: Builds synthetic client/server jars, libraries and asset index, simulates dropped connections, checks hashes and launcher paths, and verifies repeat downloads reuse intact files.
- Findings: All payloads are made-up and the HTTP server is a test harness; tests do not fetch Mojang files or establish official 1.20.1/Forge library parity.
- RustChunk relevance: Reference for deterministic downloader integration tests without network dependency.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Test fixture schema and path/OS cases must be expanded for 1.20.1 and Forge 47's additional libraries/natives.
- Follow-up checks: Add traversal/absolute path, duplicate destination, oversized response, invalid hash, cancellation and ARM64 filesystem tests.
- RustChunk implementation mapping: RustChunk local mock HTTP test harness for the target-version verified downloader.
- Evidence required before adoption: Executed tests on host and Android ARM64, 1.20.1/Forge fixture manifests and adversarial path/stream tests.
- Completion state: COMPLETE — 244 logical lines through EOF; pinned source SHA and 9149-byte size match; inspected, not modified; tests not executed.

### 2692. `crates/starling-account/src/java.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2e5ee1c0ad9a6029b2b281ed7b26315ab8a04a72`
- Size: 13832 bytes
- Coverage: Complete through EOF: 352 logical lines; full file; no gaps.
- Purpose: Downloads and installs pinned Android OpenJDK 25 runtimes for ARM64, ARMv7 and x86_64.
- Dependencies / cross-references: files::agent/retrying/may_pass; SHA-256; lzma-rs; tar; filesystem staging and rename.
- Implementation / logic: Streams a fixed-size archive to a .part file while hashing, verifies exact size/SHA-256, decompresses XZ, validates tar entry paths and stages runtime extraction before rename.
- Findings: The existing runtime directory is removed before the new archive is fully extracted, so a failed update can destroy a working runtime. Expanded tar size/entry count are not bounded; Unix symlinks are allowed and rely on tar::unpack_in protections. Installed marker checks only the marker hash, not the runtime binaries.
- RustChunk relevance: Android runtime provisioning architecture is relevant, but Starling's JDK 25 and publishers are not a drop-in choice for RustChunk's Java/Forge target.
- Decision: adaptable; reference; harden before reuse
- 1.20.1 implications: Forge 1.20.1 needs a compatible Java runtime (normally Java 17); JDK 25 is not the required baseline. ARM ABI, JNI and ART integration need independent verification.
- Follow-up checks: Use Java 17-compatible Android runtime; stage beside old install and swap only after validation; bound decompression/entries; test symlink and rollback cases.
- RustChunk implementation mapping: RustChunk runtime manager with version/ABI manifest, transactional install and post-extraction executable/library validation.
- Evidence required before adoption: Pinned Java 17 Android ARM64 build, SHA provenance, bounded archive corpus, rollback tests and device JVM launch.
- Completion state: COMPLETE — 352 logical lines through EOF; pinned source SHA and 13832-byte size match; inspected, not modified; tests not executed.

### 2693. `crates/starling-account/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `228fad98798c90cdfdd94f17d69a671d1d75fab9`
- Size: 31492 bytes
- Coverage: Complete through EOF: 875 logical lines; ranges 1–400, 401–800, 801–875; contiguous, no gaps.
- Purpose: Microsoft OAuth device-code sign-in, Xbox Live/XSTS exchange, Minecraft ownership/profile checks, session refresh and secret-store abstraction.
- Dependencies / cross-references: Microsoft consumers OAuth; Xbox Live; XSTS; Minecraft Services; serde_json; SecretStore; profiles/ely/files/java/skins modules; account mutex.
- Implementation / logic: Runs device-code polling, token exchange, ownership validation and profile loading; stores tokens separately from account metadata, supports refresh, account CRUD and cancellable background sign-in.
- Findings: `Accounts::load` silently converts read/JSON errors into an empty account list; profile UUID/name and device-code response fields are weakly validated; HTTP response bodies are unbounded and invalid JSON becomes Null. Session/activation hold a global account mutex during network refresh; account metadata and secret-store mutations are not one transaction. Android Keystore behavior is only a host-supplied trait contract.
- RustChunk relevance: Core account architecture reference, but it assumes licensed Microsoft ownership for online-mode authentication; not an offline authentication solution.
- Decision: adaptable; reference; harden and separate providers
- 1.20.1 implications: Minecraft 1.20.1 online-mode requires valid Microsoft/Mojang session semantics; Forge does not replace authentication. Offline profiles only work on servers configured for offline mode.
- Follow-up checks: Validate UUID/name/verification URL and response sizes; preserve corrupt account files with explicit errors; shorten lock scope; add transaction/recovery tests and Android Keystore integration.
- RustChunk implementation mapping: RustChunk AuthProvider interface, encrypted platform SecretStore, versioned account metadata and cancellable login/session state machine.
- Evidence required before adoption: Microsoft API approval/flow evidence, Keystore implementation, malformed response corpus, atomic persistence fault tests and online/offline server integration.
- Completion state: COMPLETE — 875 logical lines through EOF; pinned source SHA and 31492-byte size match; inspected, not modified; tests not executed.

### 2694. `crates/starling-account/src/profiles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `24b47d6e886ad7501b80ddf76dec4b7da02398da`
- Size: 11336 bytes
- Coverage: Complete through EOF: 330 logical lines; full file; no gaps.
- Purpose: Provider-aware Microsoft/Ely/offline profile selection, activation, addition and removal.
- Dependencies / cross-references: Accounts; Player; Account/Secrets; SecretStore; account_lock; Ely auth; filesystem metadata.
- Implementation / logic: Maintains signed-in and offline profile lists, validates offline names, refreshes a selected account before activation and serializes profile mutations under a global lock.
- Findings: Activation holds the global account mutex across network refresh. `remove_profile_if` treats any non-offline key as a secret-store deletion key without first confirming that it belongs to a listed account; cancellation is checked before mutation but not during Ely authentication.
- RustChunk relevance: Useful separation of offline profile identity from authenticated sessions; selection metadata alone does not create a valid online-mode session.
- Decision: adaptable; reference; harden before reuse
- 1.20.1 implications: Offline profiles can only join offline-mode servers; Microsoft token identity and Forge server auth must remain separate.
- Follow-up checks: Test concurrent selection/removal, storage failures, unknown keys, provider collisions and cancellation during provider network calls.
- RustChunk implementation mapping: RustChunk profile manager with explicit Offline/Microsoft/Ely states and no implicit auth downgrade.
- Evidence required before adoption: Persistence fault tests, concurrency tests, Java 1.20.1 session contract and separate online/offline multiplayer tests.
- Completion state: COMPLETE — 330 logical lines through EOF; pinned source SHA and 11336-byte size match; inspected, not modified; tests not executed.

### 2695. `crates/starling-account/src/skins.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `09bdf1c58cc5805e95f8909554871ff94750028f`
- Size: 10755 bytes
- Coverage: Complete through EOF: 261 logical lines; full file; no gaps.
- Purpose: Microsoft skin upload, profile refresh and Ely.by skin-site guidance.
- Dependencies / cross-references: Account/Secrets/Provider; SecretStore; multipart HTTP; Minecraft profile API; PNG signature and size checks.
- Implementation / logic: Checks a 1 MiB limit and PNG signature, verifies the selected account/provider, uploads multipart variant+image and stores the refreshed profile texture URL.
- Findings: PNG validation checks only the signature and size, not complete PNG structure or 64×64/64×32 dimensions; the service may reject malformed images. Profile texture URLs are accepted from the response without host/scheme validation.
- RustChunk relevance: Optional account UX reference, not part of core Forge compatibility.
- Decision: adaptable; optional; harden before reuse
- 1.20.1 implications: Skin upload requires a valid Microsoft-owned profile; Ely.by uses a separate website and does not expose the same upload API.
- Follow-up checks: Validate PNG structure/dimensions, URL allowlist, selected-account races and response handling; verify current Mojang/Ely APIs.
- RustChunk implementation mapping: Optional RustChunk skin service behind provider-specific capability interfaces.
- Evidence required before adoption: PNG adversarial corpus, official endpoint documentation, URL validation tests and provider integration tests.
- Completion state: COMPLETE — 261 logical lines through EOF; pinned source SHA and 10755-byte size match; inspected, not modified; tests not executed.

### 2696. `crates/starling-account/src/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `808a98da8ff61f576f93cc4cd206cfaa6f46721a`
- Size: 17477 bytes
- Coverage: Complete through EOF: 473 logical lines; ranges 1–400, 401–473; contiguous, no gaps.
- Purpose: Local mock tests for the Microsoft device-code/Xbox/XSTS/Minecraft login chain, refresh, cancellation and secret redaction.
- Dependencies / cross-references: Endpoints; SignIn; SecretStore/MemoryStore; local TCP mock; account/session helpers.
- Implementation / logic: Simulates pending OAuth polls, network drops, Xbox/XSTS errors, ownership/profile responses, refresh, persistence and cancellation; checks tokens are excluded from Debug/account metadata.
- Findings: Mocks use synthetic tokens and responses and a one-thread local server; they do not validate live Microsoft approval, real token claims, TLS endpoints or Minecraft 1.20.1 server login.
- RustChunk relevance: Reference for offline deterministic authentication tests and secret-redaction assertions.
- Decision: adaptable; reference; test-only
- 1.20.1 implications: Expand to actual RustChunk Java 1.20.1 session and Forge 47 multiplayer integration; do not infer server compatibility from mock OAuth success.
- Follow-up checks: Add malformed/oversized response, profile validation, cancellation during each HTTP step, concurrent account operations and real online/offline server tests.
- RustChunk implementation mapping: RustChunk fake endpoint server and SecretStore test doubles for provider adapters.
- Evidence required before adoption: Executed host/Android tests, Microsoft-approved app configuration and controlled online-mode/offline-mode server joins.
- Completion state: COMPLETE — 473 logical lines through EOF; pinned source SHA and 17477-byte size match; inspected, not modified; tests not executed.

### 2697. `crates/starling-catalog/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `24277d037ae077569fc29a97e65768596ab76627`
- Size: 848 bytes
- Coverage: Complete through EOF: 22 logical lines; full file; no gaps.
- Purpose: Cargo package and dependency definition for Starling's Android-facing mod/resource/shader catalog.
- Dependencies / cross-references: horizon-mods index; horizon-port-pack; image; serde/serde_json; sha1/sha2; ureq; zip.
- Implementation / logic: Declares a non-published Rust 2021 crate with PNG/JPEG/WebP/GIF image support, HTTP and ZIP processing.
- Findings: Dependencies and feature flags are inherited from a later-version Starling/Horizon workspace; they do not establish Forge 47 mod compatibility or Android packaging suitability.
- RustChunk relevance: Reference for an optional in-game catalog, separate from the game runtime and mod loader.
- Decision: adaptable; optional; dependency baseline only
- 1.20.1 implications: RustChunk Forge 1.20.1 mods are Java/Forge artifacts; Horizon's native/content mod catalog is a different ecosystem.
- Follow-up checks: Audit resolved dependency versions/licenses, Android ABI builds, archive safety and explicit Forge/Modrinth compatibility metadata.
- RustChunk implementation mapping: Optional RustChunk catalog crate with Forge 1.20.1/47.x filters and isolated verified downloads.
- Evidence required before adoption: Pinned lockfile, Android build, ZIP adversarial corpus and real Forge 47 mod metadata/installation tests.
- Completion state: COMPLETE — 22 logical lines through EOF; pinned source SHA and 848-byte size match; inspected, not modified; tests not executed.

### 2698. `crates/starling-catalog/src/launcher/browser.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `00adc08534a5fbf47e850099f973a7c9ddc370a0`
- Size: 279 bytes
- Coverage: Complete through EOF: 4 logical lines; full file; no gaps.
- Purpose: Build-time path shim documenting the desktop browser module's alternate source location.
- Dependencies / cross-references: `crates/starling-catalog/src/lib.rs` path attribute; launcher/src/browser.rs; horizon-mixins workspace scanner.
- Implementation / logic: Contains only a module-level comment explaining that the file is not compiled and exists because the mixin scanner expects a file at this path.
- Findings: No browser implementation exists in this four-line stub; functionality resides outside the audited path. Do not infer catalog behavior from this shim.
- RustChunk relevance: Repository/build-layout reference only.
- Decision: unnecessary; placeholder/build shim only
- 1.20.1 implications: No direct Minecraft 1.20.1 behavior; source path and workspace scanner assumptions are repository-specific.
- Follow-up checks: If restructuring, verify the scanner and `#[path]` target together; do not port this placeholder as functionality.
- RustChunk implementation mapping: Keep RustChunk module paths explicit; avoid fake implementation files unless a build tool demonstrably requires them.
- Evidence required before adoption: Workspace scanner behavior and compile graph for the target RustChunk workspace.
- Completion state: COMPLETE — 4 logical lines through EOF; pinned source SHA and 279-byte size match; inspected, not modified; tests not executed.

### 2699. `crates/starling-catalog/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aeeab2676206c3724b96833613e46c4d1eaf7654`
- Size: 15635 bytes
- Coverage: Complete through EOF: 432 logical lines; ranges 1–400, 401–432; contiguous, no gaps.
- Purpose: Android adapter for Horizon's mod/resource/shader catalog and installation flow.
- Dependencies / cross-references: launcher/src/browser.rs via path attribute; Registry; Modrinth; Installed; horizon-mods; horizon-port-pack; image/ZIP; filesystem.
- Implementation / logic: Provides mod/resource/shader shelves, search/pagination, shader-pack downloads/validation, installed-state cleanup, project removal and installation result classification; data-pack APIs are explicit stubs.
- Findings: Registry search materializes all matching mods before paging; shader-registry errors are silently converted to an empty result; `set_game_home` is a one-shot OnceLock that silently ignores later roots. Shader install removes old files best-effort and may leave an installed file untracked if checksum metadata is absent after rename. Catalog code targets Horizon's ecosystem, not Forge Java mods.
- RustChunk relevance: Optional catalog architecture and verified-install workflow; not a mod loader or proof of Forge mod compatibility.
- Decision: rewrite required; optional; ecosystem-specific adapter
- 1.20.1 implications: GAME and support metadata are inherited from Horizon's later-version browser; Forge 47 mods require separate loader/version/dependency handling.
- Follow-up checks: Audit launcher browser and installed modules separately, validate home/path lifecycle, pagination errors, ZIP limits and failure rollback; keep data packs out until real world-target support.
- RustChunk implementation mapping: RustChunk optional Forge Mod Browser service backed by explicit Modrinth Forge 1.20.1 metadata and transactional file installation.
- Evidence required before adoption: Forge 47 metadata matrix, Android ZIP bomb/path corpus, atomic install/rollback tests and real mod discovery/launch checks.
- Completion state: COMPLETE — 432 logical lines through EOF; pinned source SHA and 15635-byte size match; inspected, not modified; tests not executed.

### 2700. `crates/starling-catalog/src/list.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ff31c47e661441ce814d4d260017c7b401f3bcc6`
- Size: 7634 bytes
- Coverage: Complete through EOF: 229 logical lines; full file; no gaps.
- Purpose: UI-independent catalog pagination feed, stale-request rejection and touch scrolling helpers.
- Dependencies / cross-references: Project; Range; PAGE; Feed/Ask/Tail; Scroller/Finger.
- Implementation / logic: Tracks list generations and source offsets, deduplicates projects by source/id, models loading/more/end/failure states and translates pointer movement into bounded scroll offsets and visible row ranges.
- Findings: `rows_shown` has no guard for zero/negative/non-finite row height or non-finite inputs; `near_end` likewise assumes finite geometry. The 12-unit slop is UI-scale-specific. Fixed PAGE=20 assumes the source paging contract.
- RustChunk relevance: Reusable UI-independent feed/scroller concepts; touch constants and source paging policy need adaptation.
- Decision: adaptable; reference; UI utility
- 1.20.1 implications: No Minecraft protocol or Forge semantics; Android density, viewport and catalog pagination must be integrated separately.
- Follow-up checks: Test zero/negative/NaN/infinite geometry, stale/duplicate pages, short pages, offset overflow and multi-touch lifecycle.
- RustChunk implementation mapping: RustChunk catalog state model with validated viewport geometry and source-specific page-size adapters.
- Evidence required before adoption: Property tests/fuzzing for Feed and Scroller, Android touch-density tests and pagination integration tests.
- Completion state: COMPLETE — 229 logical lines through EOF; pinned source SHA and 7634-byte size match; inspected, not modified; tests not executed.

### 2701. `crates/horizon-world/src/worldgen/placement/layer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ac794bc5546d05e7585fc59ae92c340de3b51537`
- Size: 2323 bytes
- Coverage: Complete through EOF: 68 logical lines; full file; no gaps.
- Purpose: Layered configured-feature placement modifier that selects vertical candidates using a layer layout and local terrain checks.
- Dependencies / cross-references: Layer configuration and integer providers; FeatureWorld/PlacementContext height and block queries; RandomSource.
- Implementation / logic: Iterates the configured placement count, draws horizontal offsets within the candidate layer, samples MotionBlocking height and searches down for the required ground/air relationship. Counter and position updates control yielded positions.
- Findings: No confirmed defect established. Decrement/check order and random-draw sequence are behaviorally significant; malformed or extreme layer settings need validation at the data boundary.
- RustChunk relevance: Useful reference for deterministic terrain-aware placement modifiers and for keeping placement queries behind a world adapter.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Later Starling snapshot; vanilla 1.20.1 layer codecs, layer count rules, random sequence and heightmap semantics must be recovered independently.
- Follow-up checks: Differential-test layer ordering, offsets, minY, empty layers, zero counts and negative-coordinate columns against 1.20.1 Java.
- RustChunk implementation mapping: Implement a target-version LayerPlacement codec and bounded iterator over the RustChunk placement context; retain Java RNG draw order.
- Evidence required before adoption: Official 1.20.1 LayerPlacement source, serialized feature fixtures, seeded Java/Rust traces and boundary tests.
- Completion state: COMPLETE — 68 logical lines through EOF; pinned source SHA and 2323-byte size match; inspected, not modified; tests not executed.

### 2702. `crates/horizon-world/src/worldgen/projected.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `63be87cfbbe4d88da2cc12ad742a76de2c7281e9`
- Size: 10179 bytes
- Coverage: Complete through EOF: 277 logical lines; full file; no gaps.
- Purpose: Projected configured features, including patchy-square projection and stepped-column clustering.
- Dependencies / cross-references: Integer providers; FeatureWorld and PlacementContext; block-state providers; height and surface queries; RandomSource.
- Implementation / logic: Decodes bounded feature parameters, samples candidate columns, projects positions toward terrain or air, and places selected states. The stepped-column path derives cluster positions and per-column heights from bounded reach/count providers.
- Findings: The projection/drop loop has an order-sensitive boundary: decrement and stop-test order make zero/one worth checking. This is a parity candidate, not a reproduced defect. Parameter bounds and random draw order are version-specific.
- RustChunk relevance: Algorithmic reference for projected ores/features and clustered vertical placement, not a drop-in 1.20.1 implementation.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Configured-feature codecs, projection rules, heightmap behavior and random calls may differ from 1.20.1. Reconstruct the exact target serializer and algorithms.
- Follow-up checks: Compare drop limits 0/1/maximum, negative coordinates, minY, failed surface searches and random draws against Mojang 1.20.1.
- RustChunk implementation mapping: Build separate 1.20.1 patchy-square and stepped-column placement implementations behind RustChunk's feature registry.
- Evidence required before adoption: Official 1.20.1 Java implementations, target data-pack JSON, seeded placement traces and chunk-boundary tests.
- Completion state: COMPLETE — 277 logical lines through EOF; pinned source SHA and 10179-byte size match; inspected, not modified; tests not executed.

### 2703. `crates/horizon-world/src/worldgen/replace_blobs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `96fbd9198cca383a97c65e23af02fe78a96e630b`
- Size: 5710 bytes
- Coverage: Complete through EOF: 170 logical lines; full file; no gaps.
- Purpose: Configured feature that replaces a connected blob of a selected block state around an origin.
- Dependencies / cross-references: BlockStateId and provider/predicate bindings; FeatureWorld block reads/writes; integer radius provider; RandomSource; bounded work counter.
- Implementation / logic: Decodes target state and radius, searches for a matching seed, then explores bounded three-dimensional shell positions and replaces matching states while consuming a work budget.
- Findings: The explicit work budget is a useful safety feature. No confirmed defect was established; exact shell order, radius distribution and replacement flags remain parity-sensitive.
- RustChunk relevance: Strong reference for bounded blob replacement and avoiding unbounded terrain scans.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Feature schema and seed-selection behavior must match the 1.20.1 ReplaceBlobFeature implementation and block-state registry.
- Follow-up checks: Check zero/max radius, no matching seed, boundary clipping, budget exhaustion, repeated positions and deterministic order against target Java.
- RustChunk implementation mapping: Implement a 1.20.1 codec plus a budgeted frontier/shell iterator using RustChunk's versioned block-state IDs.
- Evidence required before adoption: 1.20.1 ReplaceBlobFeature source, seeded differential fixtures and tests for budget exhaustion and chunk edges.
- Completion state: COMPLETE — 170 logical lines through EOF; pinned source SHA and 5710-byte size match; inspected, not modified; tests not executed.

### 2704. `crates/horizon-world/src/worldgen/root_system.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `38a9362ea9fe561cfcec85e8e8077fd20573404e`
- Size: 8912 bytes
- Coverage: Complete through EOF: 230 logical lines; full file; no gaps.
- Purpose: Root-system configured feature that places tree-root blocks, rooted dirt and hanging roots around a tree origin.
- Dependencies / cross-references: Tree/feature configuration; block-state providers; block predicates and tags; FeatureWorld and PlacementContext; RandomSource.
- Implementation / logic: Decodes root radius, vertical-space tests, attempts, root-column height and hanging-root parameters. It performs bounded root and hanging-root attempts, tests available space and places configured states.
- Findings: Individual parameters and loops are bounded, but there is no single aggregate work budget across all attempts and column scans. The worst-case product should be measured before accepting untrusted data-pack inputs.
- RustChunk relevance: Useful reference for target-version tree-root ecology and feature-specific placement rules.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Later-version tree/root behavior. Rooted-dirt, hanging-root, tree decorators, tags and height limits must be compared with 1.20.1 registries and feature codecs.
- Follow-up checks: Verify attempt limits, maximum vertical scan, chance order, block replacement, tree-feature interactions and writes at minY/maxY.
- RustChunk implementation mapping: Create a 1.20.1 RootSystemFeature adapter and add a shared per-feature work budget if target pack inputs require it.
- Evidence required before adoption: 1.20.1 configured-feature and tree-decorator sources, official feature JSON, seeded traces and worst-case work tests.
- Completion state: COMPLETE — 230 logical lines through EOF; pinned source SHA and 8912-byte size match; inspected, not modified; tests not executed.

### 2705. `crates/horizon-world/src/worldgen/router.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aed50353d3b272329243c0b606d7e49b58b7df51`
- Size: 6018 bytes
- Coverage: Complete through EOF: 168 logical lines; full file; no gaps.
- Purpose: Defines and compiles the modern noise-router channels used by the later terrain-generation pipeline.
- Dependencies / cross-references: Density-function samplers and compiler/cache; climate and noise settings; sampler context; registry/resource bindings.
- Implementation / logic: Stores temperature, vegetation, continents, erosion, depth, ridges, chunk-surface-level and final-density samplers and wires them into a compiled router with shared sampling context.
- Findings: The structure exposes later-version channels and compilation assumptions. No one-to-one compatibility with the 1.20.1 NoiseRouter is established.
- RustChunk relevance: Architecture reference for separating named density channels from sampling and compilation.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Minecraft 1.20.1 has its own NoiseRouter field graph, noise parameter registries and density-function semantics. A 26.3 router cannot be adapted by renaming fields.
- Follow-up checks: Reconstruct the complete 1.20.1 NoiseRouter constructor graph, holders, random-state wiring, blender and aquifer dependencies.
- RustChunk implementation mapping: Build a 1.20.1-specific router schema and compile only after the target density-function registry has been imported and verified.
- Evidence required before adoption: Official 1.20.1 NoiseRouter/RandomState sources, noise settings JSON, registry exports and sample-by-sample Java/Rust comparisons.
- Completion state: COMPLETE — 168 logical lines through EOF; pinned source SHA and 6018-byte size match; inspected, not modified; tests not executed.

### 2706. `crates/horizon-world/src/worldgen/rule_test.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5202baa31b1a356022ca51b4078e444b2753af18`
- Size: 8432 bytes
- Coverage: Complete through EOF: 220 logical lines; full file; no gaps.
- Purpose: Implements recursive block rule tests and composite predicates used by the later world-generation feature system.
- Dependencies / cross-references: Block predicates and state IDs; tag bindings; height access; matter-phase/property helpers; RandomSource; shared decode depth/work accounting.
- Implementation / logic: Decodes true/not/all/any/state-set/random-state/height/matter-phase variants and evaluates them against a world query context. Recursive and list decoding uses explicit depth, item and work budgets.
- Findings: Probability values are parsed without an explicit normalized finite [0,1] check; this may be intentional codec parity but needs target confirmation. Matter-phase is an added abstraction, not automatically vanilla semantics.
- RustChunk relevance: Useful reference for bounded recursive rule ASTs and separating decode-time validation from runtime block queries.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: The target uses 1.20.1 block predicates and legacy block-state/NBT semantics. Later matter-phase rules and serialized variants cannot be assumed compatible.
- Follow-up checks: Check NaN/infinite/out-of-range probabilities, empty all/any lists, tag resolution, random draw counts and height boundaries against 1.20.1 Java codecs.
- RustChunk implementation mapping: Use a target-version rule-test enum with bounded recursive decode context and explicit adapters for optional mod-provided predicates.
- Evidence required before adoption: Official 1.20.1 predicate codecs, tag snapshots, seeded Java traces and malformed recursive JSON tests.
- Completion state: COMPLETE — 220 logical lines through EOF; pinned source SHA and 8432-byte size match; inspected, not modified; tests not executed.

### 2707. `crates/horizon-world/src/worldgen/sculk.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `79cbfb4e0373ddd99a3e4f3a064068796c890b29`
- Size: 26288 bytes
- Coverage: Complete through EOF: 727 logical lines; 1–400, 401–727; no gaps.
- Purpose: Implements sculk-related world-generation behavior, including cursor growth, spread, decay and placement around sculk catalysts.
- Dependencies / cross-references: Sculk block-state/tag bindings; block predicates; feature/world and placement contexts; RandomSource; bounded cursor and charge configuration.
- Implementation / logic: Decodes sculk growth parameters, charge limits, attempts, rounds and cursor data, then processes cursor spread, decay and growth with bounded per-operation loops.
- Findings: Per-field bounds and cursor limits constrain individual operations, but the whole feature has no single aggregate work budget. Cursor-merging and update ordering are behaviorally important and require differential tests; no runtime defect was reproduced.
- RustChunk relevance: Potential reference for sculk simulation and procedural feature placement, especially deterministic work queues and bounded cursors.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Sculk sensor/catalyst behavior, block tags, charge rules and game events must match the 1.20.1 implementation. Later block states and registry IDs are not portable.
- Follow-up checks: Compare cursor merge/decay, charge consumption, spread-attempt ordering, maximum rounds and emitted block updates against 1.20.1 source and fixtures.
- RustChunk implementation mapping: Reimplement the 1.20.1 Sculk vein/catalyst feature using explicit target-version state/tag data and a shared operation budget.
- Evidence required before adoption: 1.20.1 SculkVeinFeature/SculkPatchFeature/catalyst sources, target tags, seeded event traces and worst-case work tests.
- Completion state: COMPLETE — 727 logical lines through EOF; pinned source SHA and 26288-byte size match; inspected, not modified; tests not executed.

### 2708. `crates/horizon-world/src/worldgen/settings.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `770f12bd66dc960cfceae0e86915018047a13612`
- Size: 1949 bytes
- Coverage: Complete through EOF: 45 logical lines; full file; no gaps.
- Purpose: Defines and validates the noise-generation vertical settings, including minimum Y and generation height.
- Dependencies / cross-references: Integer range parsing; noise router and generator settings; vertical build-height constants.
- Implementation / logic: Parses minY and height, enforces supported bounds and 16-block alignment, and exposes a clamped height range for generation.
- Findings: The allowed vertical interval and clamp behavior are properties of the pinned later-version engine. Edge cases where the requested and supported intervals do not overlap need explicit tests.
- RustChunk relevance: Small reference for validating generator settings before allocating chunks or noise columns.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Minecraft 1.20.1 world-height bounds and dimension-type rules differ from the later snapshot; copying these constants would break target parity.
- Follow-up checks: Compare minY/height limits, alignment, clamping, inverted ranges and custom dimension types against 1.20.1.
- RustChunk implementation mapping: Create a dedicated 1.20.1 Dimension/NoiseSettings validator using official dimension-type and generator constraints.
- Evidence required before adoption: 1.20.1 DimensionType/NoiseSettings sources, vanilla dimension fixtures and min/max-height tests.
- Completion state: COMPLETE — 45 logical lines through EOF; pinned source SHA and 1949-byte size match; inspected, not modified; tests not executed.

### 2709. `crates/horizon-world/src/worldgen/simple_block.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a732765c5a8dc65a665d59e545e81f5078add355`
- Size: 3637 bytes
- Coverage: Complete through EOF: 109 logical lines; full file; no gaps.
- Purpose: Implements the simple-block configured feature: sample a block-state provider, check survival, place the block and optionally schedule a tick.
- Dependencies / cross-references: BlockStateProvider and StateProviderCatalog; BlockPlacement; FeatureWorld; PlacementContext; RandomSource; registry identity.
- Implementation / logic: Requires provider and placement bindings to share the same registry instance, samples an optional state, checks block survival, performs placement and schedules an optional tick.
- Findings: The explicit shared-registry identity check is a sound invariant. Placement flags, survival callbacks and tick delay must still be checked against target Java behavior.
- RustChunk relevance: Reusable structural pattern for composing data-driven providers with a transactional placement helper.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Codec and block survival rules must map to the 1.20.1 SimpleBlockFeature/BlockStateProvider and Forge block-state behavior.
- Follow-up checks: Verify empty provider behavior, survival failure, update flags, tick scheduling, random sequence and registry lifetime.
- RustChunk implementation mapping: Implement a 1.20.1 simple-block feature around RustChunk's versioned providers and placement adapter.
- Evidence required before adoption: 1.20.1 SimpleBlockFeature source, block-state provider fixtures and Java/Rust placement traces.
- Completion state: COMPLETE — 109 logical lines through EOF; pinned source SHA and 3637-byte size match; inspected, not modified; tests not executed.

### 2710. `crates/horizon-world/src/worldgen/snow_freeze.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `776336925ee5216b565ec346b82b10a0c01d1180`
- Size: 4150 bytes
- Coverage: Complete through EOF: 115 logical lines; full file; no gaps.
- Purpose: Performs the freeze-water and place-snow pass across a 16×16 feature footprint.
- Dependencies / cross-references: FeatureBlockBindings and snowy-state variants; FeatureWorld/PlacementContext biome, height, block and set-block queries; HeightmapType.
- Implementation / logic: For each column it samples MotionBlocking height, resolves the biome, tests freezing below and snow placement at the surface, writes ice/snow and switches compatible ground states to snowy variants.
- Findings: The pass relies on the supplied heightmap and biome predicates and does not independently model all support/replaceability rules. This is a parity question, not a confirmed defect.
- RustChunk relevance: Reference for batching climate-driven surface updates across a chunk-sized area.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 snow/ice placement, biome temperature and snow-support rules must be recovered exactly; block IDs and snowy-state sets are versioned.
- Follow-up checks: Compare snowy blocks, freezing position, light/temperature checks, support conditions, update flags and border columns with 1.20.1.
- RustChunk implementation mapping: Implement the target SnowAndFreezeFeature using 1.20.1 biome predicates and an explicit validated state mapping.
- Evidence required before adoption: 1.20.1 SnowAndFreezeFeature and Biome temperature methods, block-state registry export and surface-world fixtures.
- Completion state: COMPLETE — 115 logical lines through EOF; pinned source SHA and 4150-byte size match; inspected, not modified; tests not executed.

### 2711. `crates/horizon-world/src/worldgen/spawn_finder.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `12d0cf5305e0c8e2dc9f9fbdbaf052db3ecfc117`
- Size: 4459 bytes
- Coverage: Complete through EOF: 128 logical lines; full file; no gaps.
- Purpose: Finds a candidate world-spawn column by minimizing the climate fitness of configured spawn-target points.
- Dependencies / cross-references: DensitySampler and SamplerContext; climate Parameter quantization; deterministic math helpers and seeded noise targets.
- Implementation / logic: Scores the origin, performs coarse then fine radial searches up to a fixed radius, and chooses the column with lowest summed squared distance from climate intervals, using distance-to-origin as a tie-break.
- Findings: The public function accepts an empty target list; fitness then remains at its sentinel value and the result is not a meaningful climate match. Callers/data may guarantee non-empty targets, so this remains a boundary-validation candidate.
- RustChunk relevance: Reference for deterministic, density-driven spawn selection and bounded radial search.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Spawn-target fields, climate quantization, density noise and search constants belong to the target NoiseBasedChunkGenerator implementation.
- Follow-up checks: Enforce or prove non-empty targets; compare coarse/fine ring visitation, tie-breaking, negative coordinates and exact math/RNG behavior with 1.20.1.
- RustChunk implementation mapping: Recreate 1.20.1 spawn-target parsing and candidate search with an explicit non-empty validated target set.
- Evidence required before adoption: Official 1.20.1 NoiseBasedChunkGenerator/SpawnTargetPoint sources, target noise settings and deterministic spawn fixtures.
- Completion state: COMPLETE — 128 logical lines through EOF; pinned source SHA and 4459-byte size match; inspected, not modified; tests not executed.

### 2712. `crates/horizon-world/src/worldgen/speleothem.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aeb114db37c220493a1c83a8aeaf748400da6b40`
- Size: 7950 bytes
- Coverage: Complete through EOF: 210 logical lines; full file; no gaps.
- Purpose: Generates pointed dripstone-like formations, surrounding base patches and waterlogged pointed states.
- Dependencies / cross-references: FeatureBlockBindings and block predicates; BlockRegistry state lookup; Direction and RandomSource; FeatureWorld and PlacementContext.
- Implementation / logic: Builds vertical-direction/thickness/waterlogged state variants, decodes chance values, selects floor or ceiling support, spreads base blocks in cardinal directions and places pointed segments.
- Findings: The sequence and strictness of chance comparisons determine the Java random stream. Variant lookup and fallback behavior depend on a complete target registry; no defect was reproduced.
- RustChunk relevance: Reference for state-derived feature variants and multi-step cave formations.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Later Speleothem/Dripstone state properties and feature JSON must be mapped to 1.20.1 pointed-dripstone properties and placement rules.
- Follow-up checks: Compare chance defaults and comparison operators, RNG draw order, base replacement, waterlogging, floor/ceiling selection and missing-state handling.
- RustChunk implementation mapping: Implement a target-version dripstone-style feature with precomputed validated 1.20.1 state transitions.
- Evidence required before adoption: 1.20.1 DripstoneClusterFeature/PointedDripstoneFeature sources, official block-state table and seeded placement tests.
- Completion state: COMPLETE — 210 logical lines through EOF; pinned source SHA and 7950-byte size match; inspected, not modified; tests not executed.

### 2713. `crates/horizon-world/src/worldgen/spike.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fd9cec61470447608ccd292cb0568e697f2c0244`
- Size: 6782 bytes
- Coverage: Complete through EOF: 187 logical lines; full file; no gaps.
- Purpose: Builds randomized tapered spike structures with mirrored lower sections and discontinuous root columns.
- Dependencies / cross-references: Block predicates; FeatureBlockBindings; FeatureWorld/PlacementContext; RandomSource; block-state registry.
- Implementation / logic: Searches downward for a valid support point under a work budget, chooses randomized height and base radius, places tapered layers and lower mirrored blocks, then descends root columns with a second work bound.
- Findings: Root termination uses an absolute Y threshold of 50 rather than the world's minY. This is compatible only with intended dimension assumptions; other dimension profiles need a target-specific rule.
- RustChunk relevance: Reference for deterministic procedural spike geometry and explicit loop budgets.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: End-spike configuration, dimension-height assumptions and state replacement must match 1.20.1's End feature.
- Follow-up checks: Verify placement only in intended dimension, minY behavior, randomized height/radius, lower-layer symmetry, work-budget exhaustion and seeded RNG order.
- RustChunk implementation mapping: Implement an EndSpikeFeature from 1.20.1 Java source with a dimension-aware vertical floor and checked geometry.
- Evidence required before adoption: 1.20.1 SpikeFeature/End island generation sources, End dimension type and seeded geometry fixtures.
- Completion state: COMPLETE — 187 logical lines through EOF; pinned source SHA and 6782-byte size match; inspected, not modified; tests not executed.

### 2714. `crates/horizon-world/src/worldgen/spring.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9249b04485d5cb903ff9bf7324a2c6bbc2829d0b`
- Size: 3821 bytes
- Coverage: Complete through EOF: 116 logical lines; full file; no gaps.
- Purpose: Places a configured fluid spring after checking its supporting blocks and exact neighbor rock/hole counts.
- Dependencies / cross-references: FluidState and FluidBlockBindings; block predicates and air states; FeatureWorld/PlacementContext; scheduled fluid ticks.
- Implementation / logic: Decodes a fluid state and validity set, checks above/below support, counts valid-rock and air neighbors, writes the fluid block and schedules its fluid tick.
- Findings: rock_count and hole_count are parsed as unrestricted integers; impossible values silently make placement fail rather than rejecting the feature definition. The block write and tick scheduling are separate operations.
- RustChunk relevance: Reference for fluid-feature checks and ordering of block mutation versus scheduled fluid updates.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: The 1.20.1 SpringFeature codec, neighbor tests, fluid-state model and tick priority need exact reconstruction.
- Follow-up checks: Validate count ranges, neighbor order, water/lava mapping, update flags, tick scheduling failure and target RNG behavior.
- RustChunk implementation mapping: Build a target-version SpringFeature codec that validates counts and uses RustChunk's fluid/tick transaction boundary.
- Evidence required before adoption: 1.20.1 SpringFeature/FluidState sources, target feature JSON, fluid registry export and failure-injection tests.
- Completion state: COMPLETE — 116 logical lines through EOF; pinned source SHA and 3821-byte size match; inspected, not modified; tests not executed.

### 2715. `crates/horizon-world/src/worldgen/state_provider.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4e0c9d6e602ab45a16e9f77d8f82cc0d0de63f5b`
- Size: 27397 bytes
- Coverage: Complete through EOF: 695 logical lines; 1–400, 401–695; no gaps.
- Purpose: Implements the data-driven block-state provider family, registry bindings, state selection and state-property transformations.
- Dependencies / cross-references: BlockRegistry and BlockStateJsonCodec; block/fluid/biome predicates; tags; integer and weighted providers; RandomSource; world/placement query traits.
- Implementation / logic: Binds block defaults and tags, decodes direct or typed providers, evaluates rule-based/noise/simple/weighted/random/rotated/copy-properties/randomized-int nodes, and lazily builds state-rotation/property tables.
- Findings: MAX_TAG_BLOCKS is 1<<20 while nearby comments describe much smaller tag expectations; array and aggregate limits must be reviewed as one resource policy. Decoder layers also have multiple per-call budgets whose total cost should be measured.
- RustChunk relevance: Core architectural reference for RustChunk's data-driven block-state generation and registry-bound immutable providers.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Provider types, JSON schemas, block properties, tag expansion and RNG semantics must be rebuilt from 1.20.1/Forge 47 sources.
- Follow-up checks: Verify provider variants, direct-versus-typed exclusivity, aggregate budgets, tag expansion, cycle/alias behavior and exact rotation/property tables against 1.20.1.
- RustChunk implementation mapping: Create a target-version StateProvider module using one shared decode/work budget and immutable 1.20.1 registry bindings.
- Evidence required before adoption: 1.20.1 BlockStateProvider codecs, Forge registry/tag snapshots, Java random traces and adversarial provider/resource fixtures.
- Completion state: COMPLETE — 695 logical lines through EOF; pinned source SHA and 27397-byte size match; inspected, not modified; tests not executed.

### 2716. `crates/horizon-world/src/worldgen/state_provider/catalog.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e8316713ae9c1752d67cad09885aeb315e5a37f3`
- Size: 2560 bytes
- Coverage: Complete through EOF: 64 logical lines; full file; no gaps.
- Purpose: Stores named block-state provider resources and resolves provider holders lazily against a shared registry binding.
- Dependencies / cross-references: BlockStateProvider decoder; StateProviderBindings; identifier normalization; serde_json RawValue and ordered maps.
- Implementation / logic: Normalizes and deduplicates resource identifiers, caps entry count, stores raw JSON without decoding unused providers, and resolves references through shared depth/work budgets.
- Findings: The item-count cap does not constrain total JSON bytes or per-resource size. Large individual resources can still consume substantial parse and memory work before provider-level budgets apply.
- RustChunk relevance: Good reference for immutable resource snapshots and lazy resolution of named data-pack resources.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Resource location rules and named provider serialization differ in 1.20.1; integrate only after target registry/tag lifecycle is designed.
- Follow-up checks: Add total-byte/per-resource limits, test alias cycles and duplicate canonical IDs, and compare resource-location normalization with 1.20.1.
- RustChunk implementation mapping: Implement a 1.20.1 named-provider catalog backed by bounded immutable resource bundles.
- Evidence required before adoption: Target provider resource schemas, 1.20.1 ResourceLocation rules, pack-size limits and adversarial cycle/size tests.
- Completion state: COMPLETE — 64 logical lines through EOF; pinned source SHA and 2560-byte size match; inspected, not modified; tests not executed.

### 2717. `crates/horizon-world/src/worldgen/state_provider/noise.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ee8b7c4604b4a2c5b7157c4147d6510463b54323`
- Size: 8231 bytes
- Coverage: Complete through EOF: 231 logical lines; full file; no gaps.
- Purpose: Implements noise, noise-threshold and dual-noise block-state provider variants.
- Dependencies / cross-references: Noise/NoiseStack; Density/Noise decoders; seeded LegacyRandom and WorldgenRandom; BlockStateId and provider bindings.
- Implementation / logic: Decodes seed, noise settings, scale and candidate states, then selects states from one- or two-noise samples with optional threshold/chance or variety behavior.
- Findings: Float parsing and the custom long-number parser reproduce later codec behavior. Exact parsing, precision and random-draw order are compatibility-sensitive; no parity with Java 1.20.1 was demonstrated.
- RustChunk relevance: Reference for deterministic noise-driven state choice and isolating variant-specific providers.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: The target uses 1.20.1 noise parameters and Java random implementations; later NoiseStack composition cannot be reused directly.
- Follow-up checks: Differential-test decimal/exponent seeds, float boundaries, zero/maximum variety, noise sampling precision and every random draw.
- RustChunk implementation mapping: Implement 1.20.1 noise-based provider variants over the verified target noise registry and LegacyRandom adapter.
- Evidence required before adoption: 1.20.1 NoiseProvider codecs, NoiseSettings/RandomState sources, official provider JSON and seeded sample traces.
- Completion state: COMPLETE — 231 logical lines through EOF; pinned source SHA and 8231-byte size match; inspected, not modified; tests not executed.

### 2718. `crates/horizon-world/src/worldgen/state_provider/properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cac52e69e712611d4b1930587bb17d3b88731964`
- Size: 8605 bytes
- Coverage: Complete through EOF: 206 logical lines; full file; no gaps.
- Purpose: Builds compact lookup tables for block-state property identities, copying, reading and replacing Cartesian property values.
- Dependencies / cross-references: BlockRegistry; BlockStateId; integer-property allowlist; ordered property/state maps.
- Implementation / logic: Groups states by block, validates Cartesian property ordering, calculates strides and property identities, then uses indexed tables for copy and set/get operations.
- Findings: Correctness depends on the source registry's state ordering and exact property-identity families. Cartesian validation does not prove another target registry uses the same order.
- RustChunk relevance: Useful data-structure and lookup optimization reference for high-frequency block-state property operations.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: All state IDs and property order must come from the 1.20.1 registry export. The later registry snapshot is not portable.
- Follow-up checks: Compare every property family, integer-valued property and Cartesian state order with the 1.20.1 BlockState registry.
- RustChunk implementation mapping: Generate RustChunk property tables from the verified 1.20.1 registry and retain a checked slow path for unmatched property families.
- Evidence required before adoption: 1.20.1 block-state registry, exhaustive Java/Rust property-copy comparisons and malformed Cartesian-table tests.
- Completion state: COMPLETE — 206 logical lines through EOF; pinned source SHA and 8605-byte size match; inspected, not modified; tests not executed.

### 2719. `crates/horizon-world/src/worldgen/structure.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2883880d94c0ed8f23ed9452fe203290acc09d1d`
- Size: 14056 bytes
- Coverage: Complete through EOF: 409 logical lines; 1–400, 401–409; no gaps.
- Purpose: Shared structure-template resources, block-state transforms, bounding boxes and processor/template lookup infrastructure.
- Dependencies / cross-references: NBT codec and limits; BlockRegistry/BlockStateId; processor lists; filesystem prepared-file reader; gzip decoder; rotation and bounding-box helpers.
- Implementation / logic: Builds rotation/mirror/full-block/entity tables, lazily loads gzip-compressed NBT templates and processor resources, caches templates and derives jigsaw/fluid/waterlogging state metadata.
- Findings: StateTransforms filters malformed array elements and indexes short inner arrays without checking lengths, which can panic on malformed input. Gzip data is expanded into a Vec before NBT limits are applied. Template ID path containment relies on downstream file handling and needs proof.
- RustChunk relevance: Core architecture reference for structure templates, processors, block transforms and cached immutable resources.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Structure template NBT, state transforms, processor registries, block-entity IDs and paths must be reconstructed from 1.20.1 data and serialization.
- Follow-up checks: Validate transform tuples/state IDs before indexing, bound compressed/decompressed sizes, canonicalize template paths and test corrupt gzip/NBT inputs.
- RustChunk implementation mapping: Create a 1.20.1 structure-resource loader with validated transform tables, bounded decompression and path-contained template lookup.
- Evidence required before adoption: Official 1.20.1 StructureTemplate/StructurePlaceSettings sources, target transform export, NBT corpora and adversarial archive/path tests.
- Completion state: COMPLETE — 409 logical lines through EOF; pinned source SHA and 14056-byte size match; inspected, not modified; tests not executed.

### 2720. `crates/horizon-world/src/worldgen/structure/end_city.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b440fc832988ea65f6d2ef82f1d648095a6021a0`
- Size: 16710 bytes
- Coverage: Complete through EOF: 467 logical lines; 1–400, 401–467; no gaps.
- Purpose: Procedurally assembles End City towers, bridges, houses and ship pieces, then handles structure markers during placement.
- Dependencies / cross-references: StructureResources and templates; BoundingBox/Rotation/Mirror; LegacyPiece/TemplatePiece; ProcessorList; entity and loot-table NBT; RandomSource.
- Implementation / logic: Builds bounded recursive sections with a maximum generation depth, computes transformed template offsets and rejects intersecting piece groups. Post-processing creates shulkers, item frames with elytra and End City chest loot from marker names.
- Findings: Generation depth is capped, but geometry depends on exact template dimensions and transform math. Marker handling uses prefix matching; random helper failures are collapsed to zero. These are parity/robustness candidates, not confirmed defects.
- RustChunk relevance: Reference for multi-piece structure assembly, collision grouping and marker-driven entity/loot placement.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: End City templates, marker names, entity NBT and loot semantics must match Java 1.20.1 assets and structure-piece code.
- Follow-up checks: Compare template offsets, collision tags, ship probability, marker handling, chunk clipping and loot/entity initialization against 1.20.1.
- RustChunk implementation mapping: Implement target EndCityStructure through the generic 1.20.1 template-piece engine and target structure templates.
- Evidence required before adoption: 1.20.1 EndCityPieces/EndCityStructure sources, official NBT templates, seeded piece-layout fixtures and chunk-split placement tests.
- Completion state: COMPLETE — 467 logical lines through EOF; pinned source SHA and 16710-byte size match; inspected, not modified; tests not executed.

### 2721. `crates/horizon-world/src/worldgen/structure/enhanced_terrain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04f14e9698fc66674d5c687d63eac50085a0604f`
- Size: 13465 bytes
- Coverage: Complete through EOF: 358 logical lines; full file; no gaps.
- Purpose: Adapts external structure-library terrain kernels, including YUNG's API and MoogsStructureLib-style piece/junction rules.
- Dependencies / cross-references: TerrainKernel/KernelPiece/KernelJunction; StructureStart and PoolPiece; jigsaw projection; bounding-box and chunk-position geometry.
- Implementation / logic: Decodes kernel radius/distance, carving/burying actions, padding, offsets and optional vertical bands, then collects affected structure-piece bounds and terrain junctions near a chunk.
- Findings: Explicitly a mod-library extension, not vanilla structure terrain behavior. Numeric casts and bounding-box arithmetic deserve checked handling, but adoption is optional and no compatibility claim is established.
- RustChunk relevance: Potential future extension point for compatibility with specific structure mods; not part of the baseline vanilla world generator.
- Decision: undetermined; reference; optional/future
- 1.20.1 implications: No direct Forge 47 integration follows from this later Horizon extension. Each target mod's mixins/API and exact terrain-kernel contract must be separately identified.
- Follow-up checks: Identify target mods and licenses, validate each kernel algorithm and bound, and prove how Forge 1.20.1 injects equivalent terrain influence.
- RustChunk implementation mapping: Keep an optional adapter interface for externally supplied structure-terrain kernels; do not wire it into the vanilla core by default.
- Evidence required before adoption: Exact target mod versions, source/licensing review, Forge 1.20.1 mixin behavior, bounded geometry tests and parity fixtures.
- Completion state: COMPLETE — 358 logical lines through EOF; pinned source SHA and 13465-byte size match; inspected, not modified; tests not executed.

### 2722. `crates/horizon-world/src/worldgen/structure/features.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `73eecc80818c7d384a10b3dcbde686904ac8095d`
- Size: 8395 bytes
- Coverage: Complete through EOF: 231 logical lines; full file; no gaps.
- Purpose: Implements template and fossil configured features, including weighted template rotation and fossil/overlay placement.
- Dependencies / cross-references: StructureResources and templates; ProcessorList/PlaceSettings; WeightedList; FeatureWorld/PlacementContext; heightmap and random queries.
- Implementation / logic: Decodes weighted template entries and processor lists, chooses a rotation, positions templates, finds fossil placement height across the base footprint and places paired base/overlay structures within a bounded box.
- Findings: An explicitly empty rotation array survives decoding and later produces a zero-bound random request, which returns an error rather than choosing a rotation. Empty/malformed fossil arrays and corner thresholds also require strict validation.
- RustChunk relevance: Reference for reusable template/fossil feature orchestration and processor-list application.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Target 1.20.1 template/fossil codecs, templates, processor order, heightmap and NBT transformations differ.
- Follow-up checks: Test empty rotation lists, mismatched/empty fossil pairs, extreme dimensions, corner-count thresholds and exact processor/RNG order.
- RustChunk implementation mapping: Rebuild TemplateFeature/FossilFeature on the 1.20.1 structure template/processor layer with explicit codec validation.
- Evidence required before adoption: 1.20.1 TemplateFeature/FossilFeature sources, official structure NBT data, target processor registries and seeded placement tests.
- Completion state: COMPLETE — 231 logical lines through EOF; pinned source SHA and 8395-byte size match; inspected, not modified; tests not executed.

### 2723. `crates/horizon-world/src/worldgen/structure/formations.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d011966a7b1dc21e7b6332796ad073f5f0464ebb`
- Size: 14178 bytes
- Coverage: Complete through EOF: 405 logical lines; 1–400, 401–405; no gaps.
- Purpose: Implements the external Formations mod's simple and pieced structures using pool elements, height placement and jigsaw children.
- Dependencies / cross-references: Pools/PoolElement and JigsawConfig; StructureResources; StartContext/NoiseColumn; heightmap; FeatureBlockBindings; RandomSource.
- Implementation / logic: Chooses an element and rotation inside a chunk, samples five terrain columns for surface/ceiling/water/lava/underground placement, computes its vertical position and optionally expands jigsaw children within a distance box.
- Findings: Mod-specific format. max_depth and max_distance_from_center are cast to i32 without explicit range checks; large/negative settings can produce invalid geometry or excessive assembly work.
- RustChunk relevance: Optional integration example for structure mods exposing a pool-based placement format.
- Decision: undetermined; reference; optional/future
- 1.20.1 implications: Not a vanilla 1.20.1 structure type. The matching Formations version, serializer and jigsaw mixin behavior must be proven separately.
- Follow-up checks: Validate depth/distance ranges, dimension clipping, five-column averaging, random order and exact target Formations compatibility.
- RustChunk implementation mapping: Keep a plugin adapter for supported structure-mod placement definitions, isolated from vanilla structure generation.
- Evidence required before adoption: Exact Formations 1.20.1 source/version, Forge mixin hooks, sample data packs and bounded jigsaw differential tests.
- Completion state: COMPLETE — 405 logical lines through EOF; pinned source SHA and 14178-byte size match; inspected, not modified; tests not executed.

### 2724. `crates/horizon-world/src/worldgen/structure/fortress.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `521dac872c9f5c332078d3aeae82e97382d7aeb5`
- Size: 33739 bytes
- Coverage: Complete through EOF: 935 logical lines; 1–400, 401–800, 801–935; no gaps.
- Purpose: Generates and places Nether Fortress bridge/castle pieces, corridors, rooms, spawners, chests and terrain supports.
- Dependencies / cross-references: LegacyPiece/PieceFrame; BoundingBox/orientation helpers; RandomSource/LegacyRandom; BlockStateId palettes; block and entity/loot placement.
- Implementation / logic: Uses weighted bridge/castle piece lists, collision tests and a pending-piece worklist bounded by horizontal distance and depth, then shifts the structure vertically and emits each piece's block geometry and spawner/chest markers.
- Findings: The helper converts random-bound errors to zero. One branch requests a random bound of 10 minus current depth while the generator permits depth up to 30; if depth reaches 10 or more, that bound is non-positive and the helper silently returns zero. Reachability and impact need differential/reproduction before calling this a defect.
- RustChunk relevance: Important reference for deterministic legacy structure assembly, piece serialization and chunk-wise block placement.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Fortress piece geometry, weights, depth limits, random sequence, block palettes and loot/spawner behavior require exact Java 1.20.1 parity.
- Follow-up checks: Prove whether the dynamic bound can be non-positive on reachable paths; compare each piece factory, collision, y-shift, NBT save and chunk post-process with 1.20.1.
- RustChunk implementation mapping: Implement a 1.20.1 NetherFortressStructure/PieceBuilder with checked RNG bounds and explicit deterministic pending-work limits.
- Evidence required before adoption: Official 1.20.1 NetherFortressPieces sources, seeded structure-layout corpus, serialized-piece fixtures and chunk-split block/loot tests.
- Completion state: COMPLETE — 935 logical lines through EOF; pinned source SHA and 33739-byte size match; inspected, not modified; tests not executed.

### 2725. `crates/horizon-world/src/worldgen/structure/jigsaw.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7ba24ee02a9bdc5c8a56ae308ad011c238d9b598`
- Size: 59932 bytes
- Coverage: Complete through EOF: 1579 logical lines; 1–400, 401–800, 801–1200, 1201–1579; no gaps.
- Purpose: Implements structure pool elements, jigsaw blocks, pool decoding, vanilla/YUNG/Lithostitched candidate rules, structure assembly and piece placement.
- Dependencies / cross-references: PoolElement/StructureResources and templates; ProcessorList; NBT; FeatureWorld/PlacementContext; RandomSource; JigsawContext; biome/height queries; enhanced-terrain adapters.
- Implementation / logic: Decodes nested pool elements and weighted pools, resolves target/fallback pools, attaches jigsaw blocks with rotation/projection rules, tracks free-space boxes and depth-prioritized work, creates piece/junction records and places templates or feature elements.
- Findings: Pool weights are not range-checked before weight expansion into repeated Vec entries; aggregate i32 weight sums can overflow. Nested list elements and pool/resource sizes have no shared decode/work budget, and assembly has no global piece-count budget. These are resource-exhaustion candidates for content-pack input, not reproduced runtime failures.
- RustChunk relevance: Core architecture reference for procedural structure assembly, template pools and modded pool selection.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 jigsaw schemas, random draw order, pool fallback/alias behavior, projection, processors, junctions and saved NBT all require target-specific reconstruction. YUNG/Lithostitched semantics are optional mod integrations.
- Follow-up checks: Validate positive bounded weights, checked total-weight arithmetic, nested depth, pool-byte and piece-count budgets, alias cycles, free-space collision, max depth/distance and saved-piece round trips.
- RustChunk implementation mapping: Build a target 1.20.1 JigsawPlacement core first; add explicit per-mod strategy adapters and shared budgets rather than copying later extension rules.
- Evidence required before adoption: 1.20.1 JigsawPlacement/StructureTemplatePool Java sources, pool/processor resources, target YUNG/Lithostitched versions, seeded Java/Rust layouts and adversarial size/depth tests.
- Completion state: COMPLETE — 1579 logical lines through EOF; pinned source SHA and 59932-byte size match; inspected, not modified; tests not executed.

### 2726. `crates/horizon-world/src/worldgen/structure/legacy.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d8c13764531a432a51a20e651c44bb940d1409aa`
- Size: 37060 bytes
- Coverage: Complete through EOF: 1088 logical lines; 1–400, 401–800, 801–1088; no gaps.
- Purpose: Shared legacy structure-piece helpers for template transforms, oriented placement, block support and entity/container marker handling.
- Dependencies / cross-references: StructureTemplate and PlaceSettings; BoundingBox/Rotation/Mirror; BlockRegistry; NBT; FeatureWorld/PlacementContext; random source.
- Implementation / logic: Provides block-state transformation tables and placement helpers for piece frames, bounds, supports, containers, dispensers and entity markers; uses chunk/build-bound-aware writes through shared structure context.
- Findings: Several bounding-box and offset operations use i32 coordinate arithmetic, while volume loops rely on caller-provided piece bounds. Internal template dimensions normally constrain them, but malformed or extreme template geometry needs checked dimensions and write-volume limits. Some missing-state paths fall back to air, potentially hiding bad registry data.
- RustChunk relevance: Useful common foundation for structure-piece placement and transform semantics; concepts can be retained with target-version block/processor adapters.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: 1.20.1 structure transforms, palettes, block entities, jigsaw metadata and processor order differ. The later Starling tables and fallback behavior are not target parity.
- Follow-up checks: Check overflow at extreme world coordinates, negative chunk clipping, empty bounds, malformed templates and every 1.20.1 mirror/rotation mapping.
- RustChunk implementation mapping: Build a target-version shared StructurePiece/PlacementContext layer with checked bounds, validated states and transactional NBT/entity writes.
- Evidence required before adoption: Official 1.20.1 StructurePiece/StructureTemplate sources, transform fixtures, saved-piece NBT and chunk-edge differential placements.
- Completion state: COMPLETE — 1088 logical lines through EOF; pinned source SHA and 37060-byte size match; inspected, not modified; tests not executed.

### 2727. `crates/horizon-world/src/worldgen/structure/mansion.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `943f650afcd945667536e136470fab36639292d2`
- Size: 38669 bytes
- Coverage: Complete through EOF: 982 logical lines; 1–400, 401–800, 801–982; no gaps.
- Purpose: Procedurally plans and emits Woodland Mansion rooms, corridors, walls, roofs and multi-level wings.
- Dependencies / cross-references: Legacy structure-piece frame and template helpers; rotation/mirror; room graph/grid; block palettes; random source; chunk clipping.
- Implementation / logic: Builds a bounded 11×11 room layout, connects rooms and corridors, assigns floor/roof/wall pieces, and emits the corresponding oriented block geometry and markers.
- Findings: The room grid and piece loops are fixed-size and bounded; no confirmed defect was established. Geometry and random choices are tightly coupled to the pinned structure palette and native layout order.
- RustChunk relevance: Reference for translating complex procedural structures into reusable room graphs and chunk-clipped piece emitters.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Mansion room layouts, palette choices, placement seeds and block geometry must be checked against Java 1.20.1; the pinned later snapshot is not interchangeable.
- Follow-up checks: Compare seeded room graphs, connector validity, rotations, roof/wall geometry, marker placement and chunk-split output with 1.20.1.
- RustChunk implementation mapping: Implement a Java-parity 1.20.1 WoodlandMansionPieces generator over the target legacy-piece framework.
- Evidence required before adoption: Official 1.20.1 MansionPieces sources, vanilla structure templates/palettes, seeded layout corpus and per-chunk block diffs.
- Completion state: COMPLETE — 982 logical lines through EOF; pinned source SHA and 38669-byte size match; inspected, not modified; tests not executed.

### 2728. `crates/horizon-world/src/worldgen/structure/mineshaft.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `082afd0cde17d3666828cce2aca29e1794279ee2`
- Size: 29582 bytes
- Coverage: Complete through EOF: 813 logical lines; 1–400, 401–800, 801–813; no gaps.
- Purpose: Generates Mineshaft rooms, corridors, crossings, stairs and support structures, including rails, chains, spawners and chest minecarts.
- Dependencies / cross-references: LegacyPiece/PieceFrame; bounding boxes and orientation; block-state properties; random source; entity, loot and chunk post-processing APIs.
- Implementation / logic: Creates and connects piece types with distance/depth constraints, derives local geometry and support pillars, and emits per-piece blocks, rails, cobwebs, entities and loot markers.
- Findings: The shared placed-spider flag is checked and updated through separate Mutex guard operations; if chunk post-processing can run concurrently, two pieces could race and duplicate a cave spider. This is unverified until the scheduler/threading contract is traced. Coordinate absolute-value and bounding arithmetic also deserve extreme-coordinate checks.
- RustChunk relevance: Core legacy-structure reference with useful examples of graph expansion, connector bounds and chunk-wise entity placement.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 MineshaftPieces weights, room geometry, minecart loot and random sequence must be reconstructed; later entity/block states are version-specific.
- Follow-up checks: Trace post-process concurrency around the spider flag; compare every piece factory, bounds, support geometry, loot/entity placement and serialized state against 1.20.1.
- RustChunk implementation mapping: Rebuild the 1.20.1 Mineshaft piece graph using deterministic single-owner placement state and checked geometry.
- Evidence required before adoption: Official 1.20.1 MineshaftPieces sources, multichunk seeded fixtures, concurrency/lifecycle proof and saved-piece NBT tests.
- Completion state: COMPLETE — 813 logical lines through EOF; pinned source SHA and 29582-byte size match; inspected, not modified; tests not executed.

### 2729. `crates/horizon-world/src/worldgen/structure/monument.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ababac969b6da5ceb94cba95b8023c4e547c9f6c`
- Size: 50190 bytes
- Coverage: Complete through EOF: 1452 logical lines; 1–400, 401–800, 801–1200, 1201–1452; no gaps.
- Purpose: Builds Ocean Monument room graphs and emits room, wing, roof, frame, pillar and guardian-related structure geometry.
- Dependencies / cross-references: LegacyPiece/room definitions; bounding boxes and rotations; water/fill helpers; block palettes; random source; chunk bounds and entity placement.
- Implementation / logic: Assembles a fixed 5×5×3 room grid, attaches room types and child pieces, then places water-filled or solid block regions, frames, pillars and chunk-local entities.
- Findings: The room grid is bounded. Water/fill helpers iterate complete local boxes even when a large portion is outside the target chunk; writes may no-op after clipping, leaving avoidable repeated work across chunks. This is a bounded performance concern, not a correctness defect established here.
- RustChunk relevance: Important reference for deterministic room-graph structure generation and multichunk placement.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 monument room topology, elder guardian spawn rules, palettes and water behavior must be compared exactly.
- Follow-up checks: Measure repeated out-of-chunk scans, clip iteration to chunk intersections, compare seeded room graphs, entity counts and each piece's block output against Java 1.20.1.
- RustChunk implementation mapping: Implement a target 1.20.1 OceanMonumentPieces graph and chunk-intersection-aware placement routines.
- Evidence required before adoption: Official 1.20.1 OceanMonumentPieces source, structure palettes/templates, seeded layout records and chunk-by-chunk block/entity comparisons.
- Completion state: COMPLETE — 1452 logical lines through EOF; pinned source SHA and 50190-byte size match; inspected, not modified; tests not executed.

### 2730. `crates/horizon-world/src/worldgen/structure/nether_fossil.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b80b8c1a86b69d60fdc9d1c04ce01ce2ca15376b`
- Size: 4015 bytes
- Coverage: Complete through EOF: 117 logical lines; full file; no gaps.
- Purpose: Places Nether Fossil templates by locating the floor and selecting a fossil variant for the target chunk region.
- Dependencies / cross-references: Structure template resources; chunk/build-height queries; height/column sampling; positional random source; template placement and processor lists.
- Implementation / logic: Searches down from the candidate origin toward sea level using column data, selects from a fixed fossil-template set, and expands post-processing placement across each intersecting chunk.
- Findings: Whole-template placement across chunk boundaries repeats bounded template traversal in each intersecting chunk by design. The placement index converts `y - min_y` to usize after the signed i32 subtraction; at extreme coordinate endpoints that subtraction can overflow before conversion. The later snapshot also includes dried-ghast-specific behavior not part of vanilla 1.20.1; it must not be carried into the target without explicit evidence. No runtime failure was reproduced.
- RustChunk relevance: Compact reference for floor finding, positional randomness and multichunk template emission.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: The 1.20.1 fossil template set, floor search, dimension height and marker/entity behavior differ from this later snapshot.
- Follow-up checks: Compare floor-search bounds and negative Y, template selection seed, chunk overlap behavior and all fossil marker/block-entity effects against 1.20.1.
- RustChunk implementation mapping: Implement the 1.20.1 NetherFossilFeature with target templates and bounded chunk-clipped template placement.
- Evidence required before adoption: Official 1.20.1 NetherFossilFeature source, target NBT templates and seeded chunk-split placement fixtures.
- Completion state: COMPLETE — 117 logical lines through EOF; pinned source SHA and 4015-byte size match; inspected, not modified; tests not executed.

### 2731. `crates/horizon-world/src/worldgen/structure/ocean_ruin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bf15dc372a7c1982e8b1023fe0b0c041e60bf84a`
- Size: 11856 bytes
- Coverage: Complete through EOF: 360 logical lines; full file; no gaps.
- Purpose: Generates warm/cold Ocean Ruin pieces and handles ruin clustering, placement depth and archaeology markers.
- Dependencies / cross-references: Legacy template pieces and processors; ocean-floor heightmap; positional random source; biome/config parameters; block entity and drowned entity handling.
- Implementation / logic: Chooses ruin templates/configuration, creates cluster pieces and adds archaeology processors, then performs chunk post-processing and handles chest/brushable-block/drowned markers.
- Findings: Drowned marker handling uses a hard-coded sea-level value of 63 to choose water/air rather than querying the active dimension sea level; this is an Overworld-specific assumption that needs target-era confirmation. Width/coordinate arithmetic and config probabilities also need validation at their input boundary.
- RustChunk relevance: Reference for template-based feature placement, archaeology processing and biome/config-driven structure variants.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 ocean ruin serializers, suspicious sand/gravel, marker rules and sea-level behavior must be recovered from the target Java implementation.
- Follow-up checks: Verify marker sea-level behavior in custom dimensions, probability codec bounds, chunk clipping, processor ordering and exact 1.20.1 templates.
- RustChunk implementation mapping: Rebuild OceanRuinStructure with the 1.20.1 config/processor schema and dimension-aware marker logic.
- Evidence required before adoption: Official 1.20.1 OceanRuinPieces/Structure sources, target archaeology block states and marker/seed fixtures.
- Completion state: COMPLETE — 360 logical lines through EOF; pinned source SHA and 11856-byte size match; inspected, not modified; tests not executed.

### 2732. `crates/horizon-world/src/worldgen/structure/piece_limited.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b3a6d7de213d3f527ac6d87501f551ed159ce7b`
- Size: 39316 bytes
- Coverage: Complete through EOF: 1067 logical lines; 1–400, 401–800, 801–1067; no gaps.
- Purpose: Implements a PieceLimited jigsaw structure assembler for external structure-mod extensions such as Repurposed Structures and Moog's.
- Dependencies / cross-references: Jigsaw/pool elements; StructureResources; bounding boxes and free-space tracking; height/biome queries; random source; mod-specific config.
- Implementation / logic: Decodes placement/depth/distance constraints, retries candidate starts, checks piece bounds and free-space, places pool pieces and child jigsaws while limiting attempts and depth.
- Findings: Mod-specific rather than vanilla structure behavior. Several i64-to-i32 casts are unchecked; weight totals can overflow; candidate/pool arrays and total pieces lack a shared budget. The retry path caches start-box/center-derived values before choosing another start, so free-space removal may use stale geometry. These are candidates requiring exact caller/config tracing. The documented X-coordinate use in a source-Z calculation is intentionally preserved behavior, not automatically a fix target.
- RustChunk relevance: Optional adapter reference for selected modded jigsaw formats, with useful examples of bounded retry and free-space management.
- Decision: undetermined; reference; optional/future
- 1.20.1 implications: No automatic Forge 47 compatibility follows from this later Horizon extension; each supported 1.20.1 mod version and mixin contract is separate.
- Follow-up checks: Trace config ranges/casts, check weight sums and total piece limits, reproduce retries with changed start positions, and prove the source-Z behavior against the target mod.
- RustChunk implementation mapping: Keep this outside vanilla core; add a feature-gated per-mod adapter with checked bounds, a shared work budget and differential tests.
- Evidence required before adoption: Exact target mod source/version and license, Forge 1.20.1 mixin/API traces, adversarial pool configs and deterministic retry fixtures.
- Completion state: COMPLETE — 1067 logical lines through EOF; pinned source SHA and 39316-byte size match; inspected, not modified; tests not executed.

### 2733. `crates/horizon-world/src/worldgen/structure/processor.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `141ff91427175677a3087d4f251266da356b2593`
- Size: 84820 bytes
- Coverage: Complete through EOF: 2170 logical lines; 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2170; no gaps.
- Purpose: Defines the structure processor framework, vanilla processors, custom processor codecs and the ordered per-block transformation pipeline.
- Dependencies / cross-references: BlockInfo/NBT; StructureTemplate and PlaceSettings; BlockRegistry; block predicates/tags; random source; height/biome/world queries; processor lists.
- Implementation / logic: Decodes processor variants and chains, then transforms or filters template blocks using location, random, block-state, water, entity/loot and chunk-context rules. Includes recursive JSON-to-NBT conversion and shape/edge processing.
- Findings: Several resource boundaries are incomplete: pick/list counts and NBT arrays can be large, nested processor wrappers need a shared depth/work budget, and JSON-to-NBT conversion has no aggregate node/byte cap. Dense shape buffers derive allocation from bounding-box volume. Some probability/weight fields and numeric casts need finite/range checks; extreme coordinate arithmetic can overflow. These are input-hardening candidates, not reproduced runtime failures.
- RustChunk relevance: Central architectural reference for data-driven template processors and their ordering, but not reusable as a target-version codec bundle.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Processor registry names, NBT shape, loot/entity handling and block-state semantics differ across versions and Forge integrations. The 26.3 processor surface cannot be copied into 1.20.1.
- Follow-up checks: Add shared limits for processor depth/count, JSON/NBT bytes/nodes, pick arrays and shape volume; validate all probabilities/weights/casts; compare every vanilla processor's seeded behavior and pipeline order against 1.20.1.
- RustChunk implementation mapping: Build a 1.20.1 StructureProcessorType/StructureProcessor layer with bounded decode context, explicit target codecs and preserved Java random call order.
- Evidence required before adoption: Official 1.20.1 processor serializers and implementations, target processor registry/resources, NBT corpus, adversarial limits tests and seeded Java/Rust diffs.
- Completion state: COMPLETE — 2170 logical lines through EOF; pinned source SHA and 84820-byte size match; inspected, not modified; tests not executed.

### 2734. `crates/horizon-world/src/worldgen/structure/processor/litho.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ba006fd6367e2ec1a0dc17a52c8cf5bd057d3b7`
- Size: 32659 bytes
- Coverage: Complete through EOF: 841 logical lines; 1–400, 401–800, 801–841; no gaps.
- Purpose: Implements Lithostitched-specific structure processors, weighted block replacement and recursive structure conditions.
- Dependencies / cross-references: Processor framework; block-state providers; structure block info; world/biome queries; registry/tag bindings; random source.
- Implementation / logic: Decodes extension processors and conditions, tests context predicates and selects weighted states or applies conditional structure transformations.
- Findings: The module is explicitly third-party extension behavior. Its seeding contract is recorded as open; current FNV salt/world-seed/position composition is not proved equivalent to the target mod. Condition recursion has no independent depth/node budget, and weighted state totals use unchecked i32 accumulation. Probability values need finite [0,1] checks.
- RustChunk relevance: Optional example of a mod-specific processor adapter and weighted transformation framework.
- Decision: undetermined; reference; optional/future
- 1.20.1 implications: Lithostitched versions and mixin behavior for Forge 1.20.1 must be identified; this later implementation is not vanilla or drop-in support.
- Follow-up checks: Resolve exact seed contract, cap nested condition depth/nodes and weighted entries, use checked totals, and differential-test against the target Lithostitched release.
- RustChunk implementation mapping: Isolate Lithostitched behind an optional 1.20.1 adapter; do not register these processor types in vanilla core.
- Evidence required before adoption: Target Lithostitched source/license, Forge 1.20.1 mixin and serializer map, seed traces and adversarial nested/weight fixtures.
- Completion state: COMPLETE — 841 logical lines through EOF; pinned source SHA and 32659-byte size match; inspected, not modified; tests not executed.

### 2735. `crates/horizon-world/src/worldgen/structure/processor/pillars.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f0c05bab38f0c981249f776634720178922fbbcb`
- Size: 35561 bytes
- Coverage: Complete through EOF: 942 logical lines; 1–400, 401–800, 801–942; no gaps.
- Purpose: Implements pillar and terrain-aware structure processors, including cached block/tag properties, height searches and placement adjustment.
- Dependencies / cross-references: Structure processor framework; BlockRegistry and block predicates/tags; heightmaps and placement context; integer/float providers; random source.
- Implementation / logic: Builds reusable block classification sets and runs processors that inspect ground, fluids, pillars and terrain around candidate structure blocks before changing, replacing or rejecting placement.
- Findings: Some spatial arithmetic subtracts i32 coordinates before converting to i64, so extreme coordinates can overflow before the wide operation. HeightRandomizer/YungRandomizer accept unbounded entry arrays and cast f64 probabilities to f32 without enforcing finite [0,1] values. `weighted_blocks` casts i64 weights to i32 without range/sign validation and its aggregate selection total is not checked; malformed weights can distort selection. Vertical scans are bounded mainly by dimension limits. These are configuration-hardening risks, not reproduced failures.
- RustChunk relevance: Useful structure-processor patterns for world-aware placement and cached block classification.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Pillar processor formats and many block/tag semantics are Horizon/mod-specific; exact target 1.20.1 processor availability must be established.
- Follow-up checks: Use checked wide coordinate differences, cap processor arrays and work, validate probabilities and compare every accepted target processor with 1.20.1/mod implementations.
- RustChunk implementation mapping: Retain the generic block-classification cache where useful; port only verified 1.20.1 processors behind explicit adapters.
- Evidence required before adoption: Target processor serializers, official block tags/states, extreme-coordinate tests and seeded placement comparisons.
- Completion state: COMPLETE — 942 logical lines through EOF; pinned source SHA and 35561-byte size match; inspected, not modified; tests not executed.

### 2736. `crates/horizon-world/src/worldgen/structure/processor/sea_level.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4f7df2b9ff32dec5dc85ac0f407af01e1f41f97d`
- Size: 17156 bytes
- Coverage: Complete through EOF: 448 logical lines; 1–400, 401–448; no gaps.
- Purpose: Provides sea-level structure processors carried from Better Ocean Monuments and Repurposed Structures, including flooding, waterlogging, keeping world blocks and mineshaft sky-view behavior.
- Dependencies / cross-references: Structure processor framework; BlockRegistry/FluidState; PlacementContext sea level and heightmaps; chunk bounds; post-processing and scheduled block ticks.
- Implementation / logic: Converts plain air below sea level to water, waterlogs template blocks, marks edge fluids for post-processing, copies selected world blocks into templates and filters exposed mineshaft pieces while preserving grounded rails.
- Findings: The module is explicitly sourced from external mod processors, not vanilla. `MineshaftSkyView` reads y-1 without a checked lower-bound step, and the implementation relies on chunk/build-height and `% 16` edge assumptions that need dimension/negative-coordinate tests.
- RustChunk relevance: Optional reference for structure processors that consult live world fluid, sea level and chunk-edge behavior.
- Decision: undetermined; reference; optional/future
- 1.20.1 implications: Better Ocean Monuments/Repurposed Structures 1.20.1 versions and their exact processor semantics must be traced independently.
- Follow-up checks: Verify y at min build height, negative X/Z edge behavior, waterlogged state fallback, post-processing/tick scheduling and target mod serializer names.
- RustChunk implementation mapping: Implement only as feature-gated 1.20.1 processors after proving the source mod's target code and Forge registration.
- Evidence required before adoption: Exact target mod versions/source, Forge processor registrations, custom-dimension fixtures and chunk-edge fluid/tick tests.
- Completion state: COMPLETE — 448 logical lines through EOF; pinned source SHA and 17156-byte size match; inspected, not modified; tests not executed.

### 2737. `crates/horizon-world/src/worldgen/structure/ruined_portal.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4f87cdd1093ce1ded591933c9b20e98a531e3528`
- Size: 14210 bytes
- Coverage: Complete through EOF: 423 logical lines; 1–400, 401–423; no gaps.
- Purpose: Generates Ruined Portal structure variants, terrain placement and the associated block/loot post-processing behavior.
- Dependencies / cross-references: Legacy structure pieces/templates; portal configuration; rotation/mirror; block predicates and heightmaps; random source; loot and block entities.
- Implementation / logic: Chooses portal templates and placement settings, adjusts vertical placement against terrain, and applies portal-specific cleanup, chest/fire/obsidian and marker behavior during chunk placement.
- Findings: Portal setup probabilities and weighted-variant totals are accepted without a complete finite/range/positive-total validation path; malformed inputs can create zero/negative or non-finite selection totals. Chunk-to-block coordinate multiplication and surface/index arithmetic also need checked bounds. Placement still depends on exact template dimensions, terrain sampling and per-piece random order; no runtime failure or 1.20.1 parity was established.
- RustChunk relevance: Reference for template-based Ruined Portal placement and target-version post-processing boundaries.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 ruined-portal configs, templates, portal-frame rules, loot markers and height placement must be rebuilt from target Java/resources.
- Follow-up checks: Compare all portal variants, vertical placement modes, air pocket/terrain checks, chest/fire handling and chunk-split output against Java 1.20.1.
- RustChunk implementation mapping: Implement 1.20.1 RuinedPortalStructure using target templates and verified placement/marker semantics.
- Evidence required before adoption: Official 1.20.1 RuinedPortalStructure/Piece sources, vanilla NBT templates, seeded placement and per-chunk block/loot fixtures.
- Completion state: COMPLETE — 423 logical lines through EOF; pinned source SHA and 14210-byte size match; inspected, not modified; tests not executed.

### 2738. `crates/horizon-world/src/worldgen/structure/simple.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d05aab20ff964b30c0bbb258809c315e31f5465b`
- Size: 22914 bytes
- Coverage: Complete through EOF: 623 logical lines; 1–400, 401–623; no gaps.
- Purpose: Implements simple vanilla structure pieces such as shipwreck, buried treasure, igloo and swamp hut, including template and marker placement.
- Dependencies / cross-references: Structure templates and legacy piece helpers; processors; heightmaps; block/entity/loot NBT; random source; chunk bounds.
- Implementation / logic: Selects piece-specific placement transforms and heights, places template or handcrafted geometry, and processes marker blocks for chests, entities, loot and support.
- Findings: Shipwreck size validation checks width and height but the depth dimension needs target-source comparison. Template and marker handling must preserve exact chunk ownership and random calls; no defect was reproduced.
- RustChunk relevance: Useful catalog of simple structure-piece patterns but requires target-version reconstruction.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Shipwreck geometry, buried-treasure placement, igloo/hut contents, marker names and loot/entity NBT are 1.20.1-specific.
- Follow-up checks: Check all three template dimensions and overflow, structure bounding-box clipping, marker handling, chunk-split chest/loot ownership and seeded 1.20.1 outputs.
- RustChunk implementation mapping: Implement each verified 1.20.1 simple structure piece against the target template/processor subsystem.
- Evidence required before adoption: Official 1.20.1 ShipwreckPieces/BuriedTreasure/Igloo/SwampHut sources, vanilla template assets and seeded block/entity/loot fixtures.
- Completion state: COMPLETE — 623 logical lines through EOF; pinned source SHA and 22914-byte size match; inspected, not modified; tests not executed.

### 2739. `crates/horizon-world/src/worldgen/structure/start.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e2bdd443085bab493acdecaab7c4b9649c5868ee`
- Size: 60848 bytes
- Coverage: Complete through EOF: 1579 logical lines; 1–400, 401–800, 801–1200, 1201–1579; no gaps.
- Purpose: Defines structure sets, placement algorithms and structure starts, including random-spread/concentric-ring placement and biome/terrain validity checks.
- Dependencies / cross-references: Structure registry/resources; weighted structure sets; RandomState/positional random; biome and density samplers; StructureStart/pieces; chunk/coordinate math.
- Implementation / logic: Decodes structure-set placements and exclusions, chooses candidate chunks, evaluates biome/noise conditions, creates starts and serializes structure references and piece data.
- Findings: Spacing/separation and concentric-ring parameters are not consistently range-validated before division, subtraction and random-bound use. Weight sums and zone reach use unchecked i32 arithmetic; radius scans can be O((2r+1)^2) without a strict input cap; count-based ring allocation can be huge and spread can be zero. These are untrusted worldgen-config risks, not reproduced crashes.
- RustChunk relevance: Core reference for the structure scheduling and placement architecture; the validation boundaries are as important as the algorithms.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 StructurePlacement, StructureSet, placement seeds, biome checks and saved start schema differ and must be reconstructed.
- Follow-up checks: Add positive spacing/separation and bounded radius/count checks, checked weight sums, cap scans, validate spread and compare candidate chunks/seeds with 1.20.1.
- RustChunk implementation mapping: Build a 1.20.1-specific StructurePlacement/StructureSet pipeline with checked codecs and deterministic Java random adapters.
- Evidence required before adoption: Official 1.20.1 StructurePlacement/StructureSet Java sources, structure-set JSON, saved NBT, boundary/adversarial configs and seeded chunk-selection diffs.
- Completion state: COMPLETE — 1579 logical lines through EOF; pinned source SHA and 60848-byte size match; inspected, not modified; tests not executed.

### 2740. `crates/horizon-world/src/worldgen/structure/start_checks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f182706a8e234a376cb07e7f90f140fcbb9aae64`
- Size: 18628 bytes
- Coverage: Complete through EOF: 462 logical lines; 1–400, 401–462; no gaps.
- Purpose: Implements additional biome/terrain placement checks for modded jigsaw structures.
- Dependencies / cross-references: Jigsaw structure starts; biome and noise-column queries; heightmap/build-height context; third-party Repurposed Structures and Moog configuration.
- Implementation / logic: Evaluates biome radius, terrain radius/range and vertical constraints around candidate starts before allowing the modded structure to generate.
- Findings: This is mod-extension behavior, not vanilla structure placement. Radius/range values are cast to i32 and can drive quadratic world-query loops without tight bounds; height differences can overflow. Exact callers and target mod data constraints are not proven.
- RustChunk relevance: Optional adapter example for extra structure-start predicates.
- Decision: undetermined; reference; optional/future
- 1.20.1 implications: No generic Forge 1.20.1 compatibility can be inferred; each target mod's start-check contract and mixin hook is separate.
- Follow-up checks: Trace all callers/codecs, clamp radius and ranges, use checked height arithmetic, and compare exact target mod 1.20.1 behavior.
- RustChunk implementation mapping: Keep extra checks behind explicit per-mod strategy interfaces, with bounded query budgets.
- Evidence required before adoption: Exact supported mod versions, target source/mixin hooks, serializer data and adversarial radius/height tests.
- Completion state: COMPLETE — 462 logical lines through EOF; pinned source SHA and 18628-byte size match; inspected, not modified; tests not executed.

### 2741. `crates/horizon-world/src/worldgen/structure/stronghold.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7ef5b651d5e3ac5607112128054ad305bbb721f6`
- Size: 43852 bytes
- Coverage: Complete through EOF: 1173 logical lines; 1–400, 401–800, 801–1173; no gaps.
- Purpose: Generates Stronghold ring positions and assembles the connected portal-room piece graph.
- Dependencies / cross-references: Structure placement/start; piece factories and collision boxes; random source; portal-room and silverfish marker state; chunk post-processing.
- Implementation / logic: Computes concentric ring candidates, searches valid structure starts and recursively expands rooms/corridors/stairs until portal-room placement, then places blocks and markers.
- Findings: Ring count/distance/spread parameters need validation: zero spread can divide by zero, count can allocate large vectors, and distance/radius arithmetic can overflow. The outer portal-room search loop has no explicit attempt cap. Portal-room/chest flags use separate mutex check/set sections that may race if generation is concurrent. These need caller/threading and reachability verification.
- RustChunk relevance: Important deterministic structure-graph reference and example of a complex start-placement algorithm.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 stronghold ring math, piece weights, portal-room generation and random calls must match Mojang Java exactly.
- Follow-up checks: Validate placement parameters and cap ring allocation/search attempts; prove post-process concurrency; compare seeded ring candidates and piece graphs to 1.20.1.
- RustChunk implementation mapping: Reimplement 1.20.1 StrongholdStructure/StrongholdPieces with bounded search and deterministic single-owner placement flags.
- Evidence required before adoption: Official 1.20.1 Stronghold/StrongholdPieces source, saved-start fixtures, seeded graph traces, concurrent chunk schedule tests.
- Completion state: COMPLETE — 1173 logical lines through EOF; pinned source SHA and 43852-byte size match; inspected, not modified; tests not executed.

### 2742. `crates/horizon-world/src/worldgen/structure/template.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `51a49449fca2ee41c5a072e7c4dfe9c545087362`
- Size: 19348 bytes
- Coverage: Complete through EOF: 539 logical lines; 1–400, 401–539; no gaps.
- Purpose: Decodes, stores and serializes structure-template blocks, palettes, entities and local-coordinate transforms.
- Dependencies / cross-references: NBT Tag/Compound; BlockRegistry/BlockStateJsonCodec; palette and block-state IDs; rotation/mirror; block/entity NBT.
- Implementation / logic: Loads palette variants and block records, resolves state IDs, stores block/entity local positions, applies transforms and emits NBT or transformed template data for placement.
- Findings: Palette/block/entity arrays have no aggregate entry/byte limit before allocation; palette-by-block processing can multiply CPU work. Invalid/missing state data may be coerced to air or zero coordinates, potentially hiding malformed templates. Size/position transforms use i32 arithmetic and need checked bounds; save-time deduplication can be quadratic.
- RustChunk relevance: Core reusable structure-template data model, but target-version serialization and validation must be rewritten.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 structure NBT palette layout, block entity fields, entity tags, processors and transforms differ from the later source schema.
- Follow-up checks: Add compressed/decompressed byte and palette/block/entity caps, reject malformed state/position vectors, use checked transforms and benchmark palette deduplication.
- RustChunk implementation mapping: Implement 1.20.1 StructureTemplate NBT codecs with bounded allocation and a verified transform pipeline.
- Evidence required before adoption: Official 1.20.1 StructureTemplate Java sources, real vanilla structure NBT corpus, malformed/oversized fixtures and Java/Rust round-trip diffs.
- Completion state: COMPLETE — 539 logical lines through EOF; pinned source SHA and 19348-byte size match; inspected, not modified; tests not executed.

### 2743. `crates/horizon-world/src/worldgen/structure/temple.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c328f0f553df3dd6acd1943a59c708b784bf48fa`
- Size: 25852 bytes
- Coverage: Complete through EOF: 751 logical lines; 1–400, 401–751; no gaps.
- Purpose: Generates Desert Pyramid and Jungle Temple geometry, traps, chests and suspicious-sand archaeology details.
- Dependencies / cross-references: LegacyPiece and bounding-box helpers; block-state properties; random source; chunk clipping; loot and block-entity storage.
- Implementation / logic: Places fixed geometric rooms, corridors, traps and redstone-like components, registers chest/dispenser markers, and places suspicious-sand contents and related block entities at configured coordinates.
- Findings: Most geometry loops are fixed and bounded. Correctness depends on non-empty piece/bounds invariants and coordinate-specific chunk gates; no confirmed defect was established. Randomized chest/trap details and archaeology block entity semantics are version-specific.
- RustChunk relevance: Reference for hand-authored legacy structures and marker-driven container/block-entity placement.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 temple geometry, suspicious sand, loot, block entity and redstone/trap semantics must be compared; later archaeology behavior is not proof of parity.
- Follow-up checks: Verify non-empty piece union assumptions, chunk ownership of each chest/dispenser, loot initialization, suspicious sand NBT and seeded trap layout against 1.20.1.
- RustChunk implementation mapping: Implement target 1.20.1 DesertPyramidPiece and JungleTemplePiece with validated bounds and deferred block-entity writes.
- Evidence required before adoption: Official 1.20.1 DesertPyramid/JungleTemple source and structure fixtures, block/entity/loot diffs and negative-chunk tests.
- Completion state: COMPLETE — 751 logical lines through EOF; pinned source SHA and 25852-byte size match; inspected, not modified; tests not executed.

### 2744. `crates/horizon-world/src/worldgen/tags.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9c22abc769429e4b1b69ddc2699fe2a7a6dfed55`
- Size: 13687 bytes
- Coverage: Complete through EOF: 369 logical lines; full file; no gaps.
- Purpose: Parses tag resources, merges pack-priority tag definitions and resolves required/optional tag dependencies.
- Dependencies / cross-references: Raw JSON parsing; identifier normalization; block/item/registry lookup; dependency graph sorting; Java-compatible hash bucket ordering.
- Implementation / logic: Decodes string/object tag entries, applies replace/append semantics, constructs dependency edges, resolves nested tags and produces values plus missing-reference diagnostics.
- Findings: The build cap limits the number of tag definitions but not per-tag entries, total JSON bytes or identifier/source lengths. Dependency reachability is repeatedly recursive and Java bucket ordering performs quadratic hash grouping; a hostile large tag corpus can consume CPU/memory. Recursion has a depth cap but no aggregate graph-work budget.
- RustChunk relevance: Useful general design for layered tag merging, optional references and deterministic dependency resolution.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: 1.20.1 tag codecs, replace behavior, optional entries, load order and HashMap iteration semantics need target Java comparison.
- Follow-up checks: Cap file bytes, values per tag and total edges; use iterative/budgeted graph traversal; compare duplicate, cycle, collision and pack-priority behavior against 1.20.1.
- RustChunk implementation mapping: Implement a target-version tag loader with source-aware merging, bounded graph resolution and Java-compatible order only where proven necessary.
- Evidence required before adoption: Official 1.20.1 TagLoader/TagEntry sources, datapack tag corpus, adversarial graph/size tests and deterministic ordering fixtures.
- Completion state: COMPLETE — 369 logical lines through EOF; pinned source SHA and 13687-byte size match; inspected, not modified; tests not executed.

### 2745. `crates/horizon-world/src/worldgen/terrain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `783a7981b8a7fd9a499ea961de7b6c1fd416e045`
- Size: 34817 bytes
- Coverage: Complete through EOF: 973 logical lines; 1–400, 401–800, 801–973; no gaps.
- Purpose: Stores pre-light terrain chunks and applies bulk density/aquifer noise filling, block storage updates, heightmaps, ticks and post-processing marks.
- Dependencies / cross-references: DensityVolume/DensitySampler and SamplerContext; NoiseBasedAquifer/FluidPicker; BlockSection/BlockRegistry; Heightmap; ProtoChunkTicks; worldgen feature queries.
- Implementation / logic: Maintains section storage and non-air counts, primes and incrementally updates heightmaps, scans columns, stores scheduled ticks/post-processing offsets and fills chunk columns from a prepared density volume in descending Y order.
- Findings: `fill_noise` computes `volume.size()` and resizes thread-local scratch storage before `fill_noise_inner` validates 16×16 dimensions and build-height fit. A malformed/oversized density volume can therefore force a large allocation before rejection. The per-dimension checks inside the later function do not protect that earlier allocation. Scratch retains the largest allocation per thread. Failure after writes intentionally has no rollback.
- RustChunk relevance: Core engine architecture for chunk terrain storage and density/aquifer integration; its ordering and memory accounting are useful.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 ProtoChunk statuses, heightmaps, retrogen, noise density and aquifer interfaces differ; this later implementation cannot be transplanted wholesale.
- Follow-up checks: Validate dimensions and checked volume size before scratch resize; cap bytes/thread-local retention; test invalid volumes, partial failures, heightmaps, ticks and Java 1.20.1 density/aquifer parity.
- RustChunk implementation mapping: Design a target-version TerrainChunk/NoiseFill stage with pre-allocation validation, bounded scratch buffers and explicit partial-write semantics.
- Evidence required before adoption: Official 1.20.1 ProtoChunk/NoiseBasedChunkGenerator/NoiseChunk/Aquifer sources, target chunk NBT and density/aquifer differential tests.
- Completion state: COMPLETE — 973 logical lines through EOF; pinned source SHA and 34817-byte size match; inspected, not modified; tests not executed.

### 2746. `crates/horizon-world/src/worldgen/tree.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7145d41fa54084ed095ebc1c980c8b5dff47fc10`
- Size: 29440 bytes
- Coverage: Complete through EOF: 844 logical lines; 1–400, 401–800, 801–844; no gaps.
- Purpose: Coordinates configured tree generation: decodes shared tree settings, places mangrove roots, trunk and foliage, runs decorators and updates leaf distance/neighbor shapes.
- Dependencies / cross-references: Trunk/FoliagePlacer and TreeDecorator modules; block-state providers and TreeBlocks; FeatureWorld/PlacementContext; nested feature execution; RandomSource.
- Implementation / logic: Shares a bounded decoder budget across components, computes tree height/clearance, creates a tracked set of placed positions, executes decorators, builds a voxel occupancy shape and propagates leaf distances through a bucketed breadth-first pass.
- Findings: The leaf/shape pass permits each bounding dimension up to 512, then allocates a dense `size[0]*size[1]*size[2]` bit vector; the maximum accepted volume can reach 134,217,728 cells, a large allocation for mobile devices. Root placement and voxel processing need an aggregate operation budget in addition to per-component bounds.
- RustChunk relevance: Core reference for deterministic tree feature orchestration, component sharing and leaf-distance updates.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 tree feature fields, root/mangrove behavior, leaf distance and block-state properties differ from the later content set.
- Follow-up checks: Cap total shape volume/bytes rather than each dimension alone; bound total tree work and compare tree placement, roots, leaf decay distances and update-shape order against 1.20.1.
- RustChunk implementation mapping: Implement a 1.20.1 TreeFeature coordinator with checked voxel volume, shared work budget and target-version trunk/foliage/decorator modules.
- Evidence required before adoption: Official 1.20.1 TreeFeature/MangroveRootPlacer sources, tree configured-feature JSON, seeded placements and leaf-distance block-state diffs.
- Completion state: COMPLETE — 844 logical lines through EOF; pinned source SHA and 29440-byte size match; inspected, not modified; tests not executed.

### 2747. `crates/horizon-world/src/worldgen/tree/decorator.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b0aaff4b52a1ef2556db77f5c9eabf8375074eb2`
- Size: 27935 bytes
- Coverage: Complete through EOF: 699 logical lines; 1–400, 401–699; no gaps.
- Purpose: Implements tree decorators for vines, cocoa, beehives, attached blocks, pale moss, ground placement and shelf mushrooms.
- Dependencies / cross-references: TreeFeature position sets; BlockStateProvider; nested FeatureDefinition; TreeBlocks; world/height queries; random source; block entities.
- Implementation / logic: Decodes a capped decorator list, runs each decorator in sequence and applies deterministic random choices around trunk/root/leaf positions, including bounded vine drops, attached-block exclusion zones, beehive occupants and nested ground patches.
- Findings: The list is deserialized into a vector before the 256-entry cap is checked. `place_on_ground` accepts tries/radius/height up to i32::MAX, allowing extreme loops and coordinate/range overflow; its workload needs a much tighter cap. The pale-moss hanger loop has no explicit step budget, and attached-leaf blacklist volume can multiply with leaf count.
- RustChunk relevance: Good example of modular tree post-processing and preserving random-draw order.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Several decorators are later-version or mod-specific (pale moss/shelf mushrooms); 1.20.1 has a different decorator registry and block set.
- Follow-up checks: Cap raw JSON item count before allocation, bound tries/radius/height and total blacklist writes, add a hanger work cap, then compare each vanilla decorator's random order to 1.20.1.
- RustChunk implementation mapping: Rebuild only the 1.20.1 decorator registry, with a shared tree work budget and isolated optional future decorators.
- Evidence required before adoption: Official 1.20.1 tree decorator serializers, target block-state/tag data, adversarial large decorators and seeded Java/Rust traces.
- Completion state: COMPLETE — 699 logical lines through EOF; pinned source SHA and 27935-byte size match; inspected, not modified; tests not executed.

### 2748. `crates/horizon-world/src/worldgen/tree/fallen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ac4ba25ec009019930ed34d5b0af8f7c8f3086d4`
- Size: 6296 bytes
- Coverage: Complete through EOF: 182 logical lines; full file; no gaps.
- Purpose: Implements the later `minecraft:fallen_tree` feature with a stump, optional horizontal log and separate stump/log decorators.
- Dependencies / cross-references: Tree trunk provider and IntProvider; TreeDecorator/Lists; TreeBlocks; FeatureWorld/PlacementContext; positional random source; post-processing marks.
- Implementation / logic: Decodes a bounded log-length provider, places a stump, samples a horizontal direction and start offset, searches a short vertical adjustment window, checks ground support and places a bounded run of log blocks.
- Findings: The log length is decoded in 0..16 then reduced by two; values 0 or 1 result in an empty log range while the stump still succeeds. This may be intentional codec behavior; no correctness defect was established. Placement uses fixed small loops.
- RustChunk relevance: Optional reference for adding fallen-tree worldgen as a separate feature without overloading normal tree placement.
- Decision: undetermined; reference; optional/future
- 1.20.1 implications: Fallen-tree feature and likely block/content assumptions are not part of the vanilla 1.20.1 baseline; verify whether the target includes an equivalent mod/content.
- Follow-up checks: Confirm length 0/1 semantics against the source codec, check support/vertical search at build bounds and verify marker/post-processing placement.
- RustChunk implementation mapping: Keep behind an optional feature registry; do not include in the initial 1.20.1 vanilla tree feature set.
- Evidence required before adoption: Target 1.20.1 source or exact mod version, feature JSON, tree block-state mappings and seeded placement fixtures.
- Completion state: COMPLETE — 182 logical lines through EOF; pinned source SHA and 6296-byte size match; inspected, not modified; tests not executed.

### 2749. `crates/horizon-world/src/worldgen/tree/foliage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `74028ab9e69944fa262c037610fb3c3930edfc5b`
- Size: 23169 bytes
- Coverage: Complete through EOF: 631 logical lines; 1–400, 401–631; no gaps.
- Purpose: Implements foliage placer variants and the deterministic placement of leaf rows, hanging extensions and special canopy shapes.
- Dependencies / cross-references: TreeFeature/Run position sets; BlockStateProvider; TreeBlocks/Axis; trunk/foliage attachments; RandomSource and world writes.
- Implementation / logic: Decodes bounded radius/offset and variant-specific heights/chances, then creates blob, fancy, bush, acacia, spruce, pine, mega, random-spread, cherry and poplar foliage geometries in native random-draw order.
- Findings: Most ordinary radii/heights are bounded, but RandomSpread permits foliage height up to 512 and uses `next_int_bound(leaf_radius)` even though the shared radius codec allows zero; zero can produce an invalid random bound. Some row and coordinate arithmetic uses wrapping operations. Variant availability and state properties are later-version-specific.
- RustChunk relevance: Useful algorithm catalog for canopy geometry and deterministic Java-compatible random sequencing.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 foliage placer registry/types and leaf properties differ; later cherry/poplar and other variants need target evidence.
- Follow-up checks: Test zero radius and invalid-bound handling, maximum height/attempts, double-trunk geometry, negative coordinates and exact seeded leaf sets against 1.20.1.
- RustChunk implementation mapping: Implement only the target 1.20.1 FoliagePlacer types behind verified codecs, with checked bounds and preserved random-call order.
- Evidence required before adoption: Official 1.20.1 foliage placer sources, configured tree JSON, block-state/property registry and seeded Java/Rust leaf-position traces.
- Completion state: COMPLETE — 631 logical lines through EOF; pinned source SHA and 23169-byte size match; inspected, not modified; tests not executed.

### 2750. `crates/horizon-world/src/worldgen/tree/trunk.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `741f0e2a7d4f6b2b1120932602e4cd3ab25d9e1c`
- Size: 32465 bytes
- Coverage: Complete through EOF: 811 logical lines; 1–400, 401–800, 801–811; no gaps.
- Purpose: Implements trunk placer variants, trunk geometry, branch generation and foliage attachment points for configured trees.
- Dependencies / cross-references: TreeFeature/Run; TreeDecoder and IntProvider; BlockStateProvider and TreeBlocks; Axis/Direction; FeatureWorld/PlacementContext; RandomSource.
- Implementation / logic: Decodes straight, forked, giant, jungle, dark-oak, fancy, bending, upward-branching, cherry and poplar placers; writes logs, generates branches and returns foliage attachment coordinates.
- Findings: Base trunk height is bounded by its codec, but upward-branching extra length/step providers permit values up to i32::MAX and cherry branch geometry depends on iterative Manhattan movement and coordinate arithmetic. The outer tree-height gate limits some loops, but a shared work/coordinate bound is still needed. No concrete runtime exploit was reproduced.
- RustChunk relevance: Core reference for modular trunk generation and deterministic branch/attachment algorithms.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 has its own trunk placer registry, codecs and supported tree variants; newer cherry/poplar/upwards-branching content must not be assumed available.
- Follow-up checks: Tighten extra branch provider maxima, use checked coordinate/Manhattan arithmetic, test zero/maximum tree heights and compare every supported trunk variant's seeded geometry with 1.20.1.
- RustChunk implementation mapping: Implement a versioned 1.20.1 TrunkPlacer enum with a shared tree work budget and target-compatible providers.
- Evidence required before adoption: Official 1.20.1 trunk placer Java sources, vanilla tree feature JSON, block-state mappings and seeded Java/Rust branch/attachment diffs.
- Completion state: COMPLETE — 811 logical lines through EOF; pinned source SHA and 32465-byte size match; inspected, not modified; tests not executed.

### 2751. `crates/horizon-world/src/worldgen/tree_blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cffbf0d0e150faf089d48b429ab743d4b2e74bc0`
- Size: 16621 bytes
- Coverage: Complete through EOF: 471 logical lines; 1–400, 401–471.
- Purpose: Builds immutable block-state classifications, default-state lookup and tree-specific property/coordinate helpers for vegetation features.
- Dependencies / cross-references: BlockRegistry and BlockStateId; FeatureBlockBindings; tag/property/fluid bindings; StateProperties; Direction; JavaHashMap; tree, foliage, decorator and vegetation modules.
- Implementation / logic: Builds per-state flag and leaf-distance arrays from tags, block IDs, fluid/property metadata; maps state IDs to block identity; exposes predicates/property mutations, axis/direction helpers, wrapped coordinate operations, PosSet and Fisher–Yates shuffle.
- Findings: `same_block` compares optional vector lookups, so two invalid state IDs would compare as the same missing block; callers appear to rely on valid registry IDs. PosSet documents that Java HashMap tree-bin tie-breaking uses identity hashes, while this port substitutes a deterministic tuple order; collision-heavy bucket iteration may therefore diverge from vanilla iteration and affect random-draw placement order. Coordinate helpers intentionally wrap i32 arithmetic.
- RustChunk relevance: Core shared registry-bound cache for tree, vegetation, fluid, leaf-decay and block-state placement logic.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Later tag membership, block properties, fluid states and block defaults are not a 1.20.1 data source; the tree registry and available content differ.
- Follow-up checks: Validate state IDs at the boundary; compare every flag/property against 1.20.1 registries; test invalid IDs; construct adversarial Java-hash collisions and compare iteration order, while explicitly deciding whether vanilla identity-hash nondeterminism is emulated or replaced.
- RustChunk implementation mapping: Implement a 1.20.1 BlockClassifications cache keyed by the exact target registry snapshot; keep coordinate arithmetic and PosSet behavior behind compatibility-tested helpers.
- Evidence required before adoption: Official 1.20.1 block/tag/property/fluid registries, LeavesBlock distance semantics, Java HashMap implementation behavior and seeded Java/Rust set-order fixtures.
- Completion state: COMPLETE — 471 logical lines through EOF; pinned SHA and 16,621-byte size match; inspected only, not modified; embedded tests not executed.

### 2752. `crates/horizon-world/src/worldgen/underwater_magma.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0729fa3f080c2ea07acaaf5074895827e5973d48`
- Size: 6781 bytes
- Coverage: Complete through EOF: 187 logical lines; full file; no gaps.
- Purpose: Implements underwater magma placement by scanning a water column, finding its floor and sampling occluded positions around that floor.
- Dependencies / cross-references: FeatureBlockBindings and water predicates; FeatureWorld block reads/writes; BoundOcclusionFaces; Direction; RandomSource; BlockStateId.
- Implementation / logic: Validates configured probability, search range and radius; scans for non-water ceiling/floor; traverses a cubic radius around the floor and probabilistically places magma where the target and neighboring faces meet occlusion conditions.
- Findings: The configured radius ceiling of 64 still permits 129³ = 2,146,689 candidate cells for one placement. The explicit WorkLimit check at index 8,388,608 is above the maximum normal candidate count and therefore does not constrain any accepted radius. Cube dimensions and total count use wrapping i32 arithmetic; a center close to coordinate limits can wrap endpoints and produce invalid sizes/counts. No runtime failure was reproduced.
- RustChunk relevance: Useful reference for the 1.20.1 underwater magma feature and face-occlusion based placement; the resource budget needs redesign for mobile generation.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: The feature exists in vanilla-era world generation, but target codecs, water predicates, build-height handling and occlusion APIs must be taken from 1.20.1 rather than this later implementation.
- Follow-up checks: Use checked i64 bounds/volume; impose a lower mobile work budget before any world query; test radius 0/64, i32-edge coordinates, search range 0/512, waterlogged states and random-draw order against 1.20.1.
- RustChunk implementation mapping: Implement a target-version UnderwaterMagmaFeature with checked cube traversal and a shared generation work budget.
- Evidence required before adoption: Official 1.20.1 UnderwaterMagmaFeature source/codec, configured-feature JSON, block occlusion rules and seeded placement differential fixtures.
- Completion state: COMPLETE — 187 logical lines through EOF; pinned SHA and 6,781-byte size match; inspected only, not modified; tests not executed.

### 2753. `crates/horizon-world/src/worldgen/value_provider.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ba081e3e9c6a1fe0f2d7c39780e55df28be99868`
- Size: 10329 bytes
- Coverage: Complete through EOF: 275 logical lines; full file; no gaps.
- Purpose: Defines scalar float and integer distribution providers used by world-generation configuration and sampling.
- Dependencies / cross-references: WeightedList; JSON RawValue/Object helpers; identifier decoding; RandomSource and InvalidBound; density-provider utilities.
- Implementation / logic: Decodes constant, uniform, trapezoid, biased, clamped, weighted and clamped-normal providers; tracks nested integer-provider depth/count; exposes bounds and samples using native random draw order.
- Findings: Float inputs are parsed to f32 without explicit finite checks; very large JSON numeric values can become infinities, and clamped-normal deviation is not range-checked. Integer trapezoid validates plateau only against the upper range and uses wrapping range/bound arithmetic; extreme full-width bounds can produce invalid random bounds or distorted results. Integer nesting has depth 256 and a shared node count 65,536, but FloatProvider has no equivalent explicit limits. These are input-validation risks, not reproduced crashes.
- RustChunk relevance: Core reusable worldgen data primitive; integer/float distributions affect biome, terrain, feature and structure determinism.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 provider registries/codecs and supported provider types must be matched exactly; later-only distribution types or parameter semantics cannot be assumed compatible.
- Follow-up checks: Reject non-finite floats; validate deviation and plateau domains; use checked width/bound computations; cap raw JSON collection sizes before allocation; test full i32 range, zero-width, negative plateau, nested-provider limits and exact RNG consumption.
- RustChunk implementation mapping: Implement 1.20.1 IntProvider/FloatProvider enums with target codec validation, checked arithmetic and explicit decode/work budgets.
- Evidence required before adoption: Official 1.20.1 provider codecs/samplers, Mojang random source implementations and fixtures for every supported distribution and edge bound.
- Completion state: COMPLETE — 275 logical lines through EOF; pinned SHA and 10,329-byte size match; inspected only, not modified; tests not executed.

### 2754. `crates/horizon-world/src/worldgen/vegetation.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `03685244a88fe37afff8745fdd16953ec12630b2`
- Size: 31455 bytes
- Coverage: Complete through EOF: 847 logical lines; 1–400, 401–800, 801–847.
- Purpose: Implements vegetation patches, coral trees/claws, huge mushrooms and multiface/sculk growth placement.
- Dependencies / cross-references: FeatureBlockBindings; BlockPredicate and BlockStateProvider; FeatureWorld/PlacementContext; nested placed-feature graph; TreeDecoder/TreeBlocks/PosSet; IntProvider; random and direction helpers.
- Implementation / logic: Decodes patch geometry and ground replacement, performs nested vegetation placement, builds coral branching, grows huge mushroom caps/stems and performs multiface attachment/spread with waterlogging, support and post-processing rules.
- Findings: VegetationPatchFeature accepts an xz_radius IntProvider without a feature-specific range cap; sampled radii drive nested square loops and coordinate stepping in ground_patch. HugeMushroomFeature accepts foliage_radius as an unrestricted integer and uses it directly in quadratic loops and negated range endpoints. Extreme values can create enormous work or coordinate overflow; i32::MIN also makes negation unsafe. By contrast, multiface search_range is capped at 64. No runtime failure was reproduced.
- RustChunk relevance: Contains multiple target-relevant vanilla vegetation algorithms, but this combined module also includes later-version behavior and needs decomposition by target registry.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: 1.20.1 has vegetation patches, coral, mushroom and multiface concepts, but feature fields, block properties and sculk/vegetation content must be checked against target registries; later-only types cannot be ported blindly.
- Follow-up checks: Add checked radius/foliage-radius limits and aggregate candidate budgets before loops; test negative, zero, maximum and i32-edge radii; verify nested placement accounting, fluid/attachment rules, block flags and seeded placements against target-era Java.
- RustChunk implementation mapping: Split into versioned 1.20.1 feature implementations; expose a shared bounded patch iterator and keep optional later content out of the vanilla registry.
- Evidence required before adoption: Official 1.20.1 vegetation patch/coral/mushroom/multiface Java sources and codecs, feature JSON, block-state/tag data and seeded block-position/RNG traces.
- Completion state: COMPLETE — 847 logical lines through EOF; pinned SHA and 31,455-byte size match; inspected only, not modified; tests not executed.

### 2755. `crates/horizon-world/src/worldgen/vines.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5e5530ee90ab35450308043fefe64a5e6551b2d2`
- Size: 2092 bytes
- Coverage: Complete through EOF: 68 logical lines; full file; no gaps.
- Purpose: Places a single vine block with the first valid attachment in the native direction priority order.
- Dependencies / cross-references: FeatureBlockBindings and block registry; FeatureWorld/PlacementContext; Direction and BlockStateId; attachment query.
- Implementation / logic: Resolves the default vine state and five directional state variants, requires air at the placement position, then checks up/north/south/west/east support in order and writes the first matching state.
- Findings: The implementation assumes the default vine state exists and unwraps its registry lookup after resolving the ID; that is an internal registry invariant. It also requires all five directional property combinations to be present. No independent work or loop bound concern; no failure was reproduced.
- RustChunk relevance: Small reusable vanilla behavior; direction order and block-state property resolution are important for parity.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Vines are present in 1.20.1, but registry state combinations, property names and the direction-priority behavior must be confirmed against the target version.
- Follow-up checks: Test missing default state, each missing face state, all six support outcomes, negative coordinates and exact first-match priority against 1.20.1.
- RustChunk implementation mapping: Implement a narrow 1.20.1 vine feature using validated state handles and the target's exact attachment query order.
- Evidence required before adoption: Official 1.20.1 VineBlock/Vines feature source and block-state registry fixtures.
- Completion state: COMPLETE — 68 logical lines through EOF; pinned SHA and 2,092-byte size match; inspected only, not modified; tests not executed.

### 2756. `crates/horizon-world/src/worldgen/weighted.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c5149c8dcb36723aee6e62fa2ec0ea9be4221d7d`
- Size: 3814 bytes
- Coverage: Complete through EOF: 105 logical lines; full file; no gaps.
- Purpose: Provides ordered weighted choices with vanilla-style bounded random draw consumption, plus a zero-total optional form.
- Dependencies / cross-references: DecodeError/Object/RawValue; RandomSource and InvalidBound; shared value-provider decoder.
- Implementation / logic: Stores entries, cumulative weights and a checked positive i32 total; rejects negative weights and sums that overflow; retains zero-weight entries; selects by partitioning cumulative totals after one bounded draw. OptionalWeightedList allows a zero total and returns None without drawing.
- Findings: The decoded JSON array is materialized as a Vec before its 8,192-entry cap is checked, so the cap does not protect the initial allocation. The public constructor similarly receives an already-built Vec. Weight totals are otherwise checked and zero-weight entries are intentionally retained; no sampling defect was reproduced.
- RustChunk relevance: Reusable weighted-choice utility for loot/worldgen-like algorithms; random draw count and stable entry order matter.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: 1.20.1 uses weighted provider/list codecs with its own validation and RNG call contracts; compatibility must be measured rather than inferred from a later helper.
- Follow-up checks: Enforce raw JSON/byte and item-count budgets before allocation; keep checked sum semantics; test all-zero, singleton, zero-weight prefix/suffix, max total and exact Java random draw parity.
- RustChunk implementation mapping: Implement target-version weighted list with bounded decode before materialization and explicitly preserved random consumption.
- Evidence required before adoption: Official 1.20.1 weighted list/provider codecs and seeded sampling traces for boundary weights.
- Completion state: COMPLETE — 105 logical lines through EOF; pinned SHA and 3,814-byte size match; inspected only, not modified; tests not executed.

### 2757. `crates/horizon-world/src/worldgen/write_zone.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6f5075d77299377b6ac0545c3b786accc35800e4`
- Size: 1495 bytes
- Coverage: Complete through EOF: 45 logical lines; full file; no gaps.
- Purpose: Defines horizontal chunk write-zone checks and the vertical write-height restriction used during below-zero retrogen/upgrade generation.
- Dependencies / cross-references: ChunkPos and BlockPos; TerrainFeatureAccess; generation-step write policy; retrogen state.
- Implementation / logic: GenerationWriteZone compares wrapped absolute chunk-coordinate deltas to a caller radius. UpgradeWriteHeight stores a min/max range and provides a fixed BELOW_ZERO interval of -64..-1.
- Findings: Both constructors accept unvalidated values. `wrapping_abs(i32::MIN)` remains negative, so an extreme coordinate delta can incorrectly satisfy a positive radius. UpgradeWriteHeight computes `min_y + height - 1` with wrapping arithmetic, allowing invalid or inverted ranges for negative/overflowing heights. Existing region tests exercise normal bounds, not these integer extremes.
- RustChunk relevance: Relevant to limiting retrogen writes so feature generation does not modify already-generated sections/chunks; the policy concept is reusable.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: 1.20.1 below-zero retrogen and chunk status rules are target-specific; constants/ranges and write gating should match the 1.20.1 upgrade path exactly.
- Follow-up checks: Use checked i64 deltas and validate nonnegative bounded radius/height; test i32 coordinate extremes, negative radius/height, zero height, boundary inclusion and center-chunk policy.
- RustChunk implementation mapping: Implement target-version GenerationWriteZone/UpgradeWriteHeight as validated policy types owned by chunk retrogen orchestration.
- Evidence required before adoption: Official 1.20.1 BelowZeroRetrogen, WorldGenRegion and chunk-status sources plus boundary tests at negative chunk coordinates and build-height edges.
- Completion state: COMPLETE — 45 logical lines through EOF; pinned SHA and 1,495-byte size match; inspected only, not modified; tests not executed.

### 2758. `crates/horizon-world/tests/biome_data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `51be40cfc1fc2fcc70948d7bf3b1d6e07c595df2`
- Size: 2789 bytes
- Coverage: Complete through EOF: 91 logical lines; full file; no gaps.
- Purpose: Regression tests for multi-noise biome import, scalar/prepared-volume sampling consistency, ties and out-of-volume errors.
- Dependencies / cross-references: MultiNoiseBiomeSource; ClimateSampler; DensitySampler and SamplerContext; GradientSampler/ConstantSampler; BiomeError.
- Implementation / logic: Builds a two-biome fixture, compares scalar and prepared sampling at selected positions, checks volume-boundary failures and rejects malformed quantized coordinate dimensions, non-integer offsets and invalid identifiers.
- Findings: The fixture compares a prepared volume sampled through its original resolver with one passed to a second resolver and expects the resolver-specific result; this is a targeted behavior contract whose rationale should be preserved when ported. It covers only a small fixture, not all ties or climate dimensions. No tests were executed.
- RustChunk relevance: Useful test specification for target biome parameter quantization and prepared sampling boundaries; not application implementation.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 biome source serialization and climate sampling must be sourced from its own Java code; the later test fixture does not prove target parity.
- Follow-up checks: Port the assertions into the target test harness, expand all six climate axes, exact tie cases, negative/edge coordinates and volume ownership/caching behavior.
- RustChunk implementation mapping: Use as a compact acceptance-test template for MultiNoiseBiomeSource import, scalar/prepared sampling and volume boundaries after a 1.20.1 implementation exists.
- Evidence required before adoption: A 1.20.1 MultiNoiseBiomeSource/Climate.Sampler reference plus matching Java output fixtures and execution of the ported Rust tests.
- Completion state: COMPLETE — 91 logical lines through EOF; pinned SHA and 2,789-byte size match; inspected only; embedded tests not executed.

### 2759. `crates/horizon-world/tests/biome_predicate_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5910ebbcf386e7a9734cef172d40d21809e5e731`
- Size: 1537 bytes
- Coverage: Complete through EOF: 39 logical lines; full file; no gaps.
- Purpose: Regression tests for biome holder identity, tag resolution, duplicate rejection and predicate-list size bounds.
- Dependencies / cross-references: BiomePredicateBindings; BiomeId; BTreeMap registry/tag fixtures; serde_json.
- Implementation / logic: Checks valid empty and populated tags, duplicate elimination, rejection of tag-string arrays, an 8,193-item selector, duplicate biome IDs/canonical names and unknown tag members.
- Findings: The test covers the binding layer's accepted/rejected identity cases but does not execute any biome placement predicate or test large tag-file import bounds. It is a source-level regression specification only.
- RustChunk relevance: Useful validation checklist for resource-registry identity and bounded biome-holder expansion; not a runtime component.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: The exact registry holder and tag semantics differ between Starling 26.3 and 1.20.1.
- Follow-up checks: Port to target tests; extend with duplicate canonical aliases, unresolved tag references, empty selectors, maximum accepted list and actual predicate evaluation.
- RustChunk implementation mapping: Port holder identity, duplicate canonical name, tag resolution and selector-size cases into the target biome predicate loader's automated test suite.
- Evidence required before adoption: Official 1.20.1 biome registry/tag loaders, holder-set codec and executed Rust unit tests.
- Completion state: COMPLETE — 39 logical lines through EOF; pinned SHA and 1,537-byte size match; inspected only; tests not executed.

### 2760. `crates/horizon-world/tests/biome_region.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `13afd9533b5ebba81257d2d2c87fcea3d38a7b9f`
- Size: 1418 bytes
- Coverage: Complete through EOF: 40 logical lines; full file; no gaps.
- Purpose: Tests resident biome chunk uniqueness, filled-biome tracking, replacement updates and world-coordinate lookup at extreme Y values.
- Dependencies / cross-references: BiomeChunk and BiomeRegion; ChunkPos; BiomeRegionError.
- Implementation / logic: Builds and fills a negative-coordinate biome chunk, rejects duplicate/mismatched chunks, verifies present-biome set after fill/replacement, checks i32 Y extrema and missing-chunk errors.
- Findings: Coverage is one compact fixture; it does not test large/sparse regions, chunk-boundary coordinates in X/Z or retrogen visibility. The test has not been executed.
- RustChunk relevance: Useful regression reference for chunk-local biome storage and negative-coordinate routing; not production code.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Biome container/storage format and retrogen semantics must match 1.20.1.
- Follow-up checks: Port tests for floor-divided negative coordinates, region overlap, all biome quart boundaries, sparse/chunk lookup and retrogen status gates.
- RustChunk implementation mapping: Use as a target-version biome-container regression fixture for chunk uniqueness, fill/replacement updates and negative-coordinate lookup.
- Evidence required before adoption: Official 1.20.1 LevelChunk biome storage, QuartPos conversion and executed boundary fixtures.
- Completion state: COMPLETE — 40 logical lines through EOF; pinned SHA and 1,418-byte size match; inspected only; tests not executed.

### 2761. `crates/horizon-world/tests/block_collision_scan.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `41343e27ad64deb9a3f900d50428846293326e3e`
- Size: 9539 bytes
- Coverage: Complete through EOF: 306 logical lines; full file; no gaps.
- Purpose: Tests collision scan visit budgets, early-stop behavior, invalid bounds, 3D cursor ordering, skipped corners and section-level shape reachability.
- Dependencies / cross-references: BlockCollisionScan/BlockCollisionWorld; Aabb, BlockPos, ChunkPos, VoxelShape; CollisionContext and BlockStateId.
- Implementation / logic: Uses failing and recording world adapters to assert errors fuse the iterator, budgets count skipped candidates, invalid/excessive domains fail before world queries, cells follow Cursor3D order and contained sections suppress outer-cell reads.
- Findings: Tests exercise synthetic-world adapters and document expected query ordering; they do not run against Minecraft's live collision registry/world or validate entity collision parity. No test was executed.
- RustChunk relevance: High-value acceptance criteria for the engine's bounded block-collision traversal; this is test code, not an algorithm to ship.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Minecraft 1.20.1 entity collision and section shape behavior must be verified independently; later tests use Starling's adapter surface.
- Follow-up checks: Port fixtures to 1.20.1 collision shapes; include chunk-edge, moving-piston, entity-specific shapes, negative coordinates and actual visit-budget enforcement.
- RustChunk implementation mapping: Port the visit-budget, invalid-bounds, cursor-order and section-boundary assertions into the target collision-scan test suite.
- Evidence required before adoption: Official 1.20.1 CollisionGetter/LevelChunk collision sources, target collision-shape tables and executed integration tests.
- Completion state: COMPLETE — 306 logical lines through EOF; pinned SHA and 9,539-byte size match; inspected only; tests not executed.

### 2762. `crates/horizon-world/tests/block_entities.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `244c6f95f29c05f751a7d925993059e2f78b6e8a`
- Size: 2827 bytes
- Coverage: Complete through EOF: 73 logical lines; full file; no gaps.
- Purpose: Tests active/pending block-entity save state, invalid pending NBT rejection, snapshot independence and floating coordinate conversion.
- Dependencies / cross-references: ProtoBlockEntities; BlockPos/ChunkPos; NBT Tag and position_from_tag.
- Implementation / logic: Checks failed pending data does not replace current entries, failed active serialization does not fall back to stale pending data, save snapshots do not alias mutable pending tags and float/double coordinates floor then saturate to Java int semantics.
- Findings: Only a small set of synthetic tags and coordinate edge values are covered; no real block-entity type load/save round-trip is tested. Tests were not executed.
- RustChunk relevance: Useful correctness specification for block-entity lifecycle and NBT persistence; not a runtime module.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 block-entity NBT fields, chunk ownership and coordinate decoding must match target Java semantics.
- Follow-up checks: Port to target NBT tests; add valid/invalid block-entity IDs, chunk-local coordinates, duplicate entries, entity removal and actual chunk save/reload.
- RustChunk implementation mapping: Port the active/pending block-entity save-state and Java coordinate conversion assertions into target NBT persistence tests.
- Evidence required before adoption: Official 1.20.1 BlockEntity/ChunkSerializer sources and executed NBT round-trip fixtures.
- Completion state: COMPLETE — 73 logical lines through EOF; pinned SHA and 2,827-byte size match; inspected only; tests not executed.

### 2763. `crates/horizon-world/tests/block_predicate_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fc7e07ef63b8b682e5cd580bff41feb46c573c00`
- Size: 2938 bytes
- Coverage: Complete through EOF: 92 logical lines; full file; no gaps.
- Purpose: Tests shared evaluation/decode budgets for block predicates and environment scans, including early exits and extreme coordinate volumes.
- Dependencies / cross-references: BlockPredicate; PlacementModifier::EnvironmentScan; PlacementContext; LegacyRandom; PlacementQueryError.
- Implementation / logic: Checks max-step scan budget without consuming RNG, nested predicate-tree decode/evaluation limits, and an oversized volume's early-false shortcut versus work-limit rejection.
- Findings: These are carefully constructed unbounded-world fakes; they do not exercise real chunk queries or feature scheduler integration. The embedded Rust tests were inspected but not run.
- RustChunk relevance: Strong hardening and regression checklist for bounded placement predicates; test-only source.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: The 1.20.1 predicate codec and placement-modifier semantics must be sourced from target Java; limits may be RustChunk policy rather than vanilla semantics.
- Follow-up checks: Port tests for nested AllOf/AnyOf/Not/Volume, world-query counts, boundary Y, early exit and no-RNG-consumption behavior.
- RustChunk implementation mapping: Use as adversarial tests for target block-predicate decode/evaluation budgets and environment-scan RNG invariance.
- Evidence required before adoption: Official 1.20.1 BlockPredicate and EnvironmentScan source plus executed unit/property tests.
- Completion state: COMPLETE — 92 logical lines through EOF; pinned SHA and 2,938-byte size match; inspected only; tests not executed.

### 2764. `crates/horizon-world/tests/carver_generation.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `688daea45e4cc257ee582d7c6f25ec2b32684e50`
- Size: 3032 bytes
- Coverage: Complete through EOF: 89 logical lines; full file; no gaps.
- Purpose: Tests carver-mask generation origin order, height interval differences during upgrading, error stop behavior and carver-holder identity.
- Dependencies / cross-references: generate_carving_mask/CarverCatalog; WorldGenerationContext; ChunkPos; MaskGenerationError.
- Implementation / logic: Checks 17×17 origin visitation and wrapped chunk coordinates at i32 extremes, different upgrade mask ceilings, invalid bounds before source calls, fail-fast source errors and duplicate alias holder identity.
- Findings: A compact fixture does not test the actual cave/ravine carver algorithm, density sampling, mask serialization or parallel scheduling. Tests not executed.
- RustChunk relevance: Useful deterministic-order and boundary regression criteria for worldgen carving; not implementation source.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 carver origin enumeration, retrogen height rules and holder registry semantics require direct target evidence.
- Follow-up checks: Port tests against 1.20.1 CarvingMask and WorldGenRegion; add actual carver shapes, seed traces, build-height boundaries and failure cleanup.
- RustChunk implementation mapping: Port carver origin visitation, retrogen-height and fail-fast source assertions into target carver-generation tests.
- Evidence required before adoption: Official 1.20.1 WorldGenRegion/carver source, target retrogen rules and executed Java/Rust origin-order fixtures.
- Completion state: COMPLETE — 89 logical lines through EOF; pinned SHA and 3,032-byte size match; inspected only; tests not executed.

### 2765. `crates/horizon-world/tests/carving_stage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3af6d45709ba62e5d34917bc65ff377705b56141`
- Size: 5207 bytes
- Coverage: Complete through EOF: 172 logical lines; full file; no gaps.
- Purpose: Tests carving-mask application order, uncarvable block filtering, aquifer calls, surface restoration and post-processing marks across failures.
- Dependencies / cross-references: CarvingMask; TerrainChunk/TerrainAquifer; CarvingBlocks; MaterialRule; TopMaterialInputs; NBT BlockRegistry fixtures.
- Implementation / logic: Uses a synthetic terrain chunk and recording aquifer to check filter order, exact positions sampled, behavior when an aquifer returns air/none or errors, restored surface blocks and scheduled post-processing.
- Findings: This is a narrow three-block fixture; it does not establish full cave/carver geometry or target version parity. Embedded tests not executed.
- RustChunk relevance: Useful failure-order and partial-mutation criteria for the carving stage; not code to adopt.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 carver-to-aquifer pipeline and surface-rule ordering must be compared directly with target source.
- Follow-up checks: Port with varied heightmaps, fluids, uncarvable tags, aquifer errors and multiple mask columns; verify partial writes and postprocessing under 1.20.1.
- RustChunk implementation mapping: Port aquifer/filter ordering, failure restoration and post-processing assertions into target chunk-carving integration tests.
- Evidence required before adoption: Official 1.20.1 CarvingContext/Carver/Aquifer and chunk carving order; executed integration fixtures.
- Completion state: COMPLETE — 172 logical lines through EOF; pinned SHA and 5,207-byte size match; inspected only; tests not executed.

### 2766. `crates/horizon-world/tests/collision_data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `85b41d98ebe834b2909ac49996d274c93d3e2245`
- Size: 13226 bytes
- Coverage: Complete through EOF: 341 logical lines; full file; no gaps.
- Purpose: Tests collision-shape NBT loading, canonical identity restoration, block-state mappings and streamed table decoding.
- Dependencies / cross-references: CachedCollisionRegistry; BlockRegistry/BlockSection; VoxelShape; NBT Tag/BlockTableFile.
- Implementation / logic: Covers empty/full/context-only cached shapes, invalid table references and malformed geometry, storage sharing without identity collapse, canonical shape IDs, strict registry-scan binding and tree-versus-streamed table parity.
- Findings: Synthetic NBT fixtures validate importer contracts but not actual 1.20.1 collision table assets, modded states, shape compression or runtime query performance. Tests were not run.
- RustChunk relevance: Useful asset-validation suite for collision-shape cache import; test-only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: The target's collision serialization and block-state registry differ; the later canonical identity metadata cannot be assumed to exist in 1.20.1 assets.
- Follow-up checks: Port strict parsing/identity tests to the target table format; add corrupt/truncated files, duplicate states, large shape tables and actual collision cache fixtures.
- RustChunk implementation mapping: Use as importer regression cases for target collision-shape NBT tables, canonical shape identity and streamed metadata validation.
- Evidence required before adoption: Official 1.20.1 block collision/state registry data, NBT table format and executed import tests.
- Completion state: COMPLETE — 341 logical lines through EOF; pinned SHA and 13,226-byte size match; inspected only; tests not executed.

### 2767. `crates/horizon-world/tests/decoration_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2ba2eb885b5690c176b1c3efcc41e58dfd390284`
- Size: 1380 bytes
- Coverage: Complete through EOF: 35 logical lines; full file; no gaps.
- Purpose: Tests feature-decoration rejection of duplicate biome identities and invalid/unbounded neighborhood extents before chunk reads or callbacks.
- Dependencies / cross-references: FeatureDecorationPlan; DecorationWorld; DecorationBiome; FeatureIdentity; DecorationError and FeatureSortError.
- Implementation / logic: Builds a world adapter that panics if chunks or generating-state callbacks are queried, then checks duplicate biome IDs and extreme chunk coordinates return deterministic errors before those calls.
- Findings: Only two boundary coordinates and a fake empty plan are used; it does not cover real neighborhood radii or placement ordering. Tests not run.
- RustChunk relevance: Small but valuable preflight/work-limit regression specification; test-only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Target feature sorting/decorating neighborhood and chunk radius differ in 1.20.1 and need source verification.
- Follow-up checks: Port with normal and maximum valid neighborhoods, sparse chunks, negative coordinates and callback-state cleanup.
- RustChunk implementation mapping: Port duplicate biome identity and preflight work-limit assertions into target feature-decoration tests.
- Evidence required before adoption: Official 1.20.1 WorldGenRegion feature decoration and biome feature sorting sources; executed tests.
- Completion state: COMPLETE — 35 logical lines through EOF; pinned SHA and 1,380-byte size match; inspected only; tests not executed.

### 2768. `crates/horizon-world/tests/edge_backoff.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4e69992050952b9bcf02618519c17d1ff569afc5`
- Size: 3138 bytes
- Coverage: Complete through EOF: 109 logical lines; full file; no gaps.
- Purpose: Tests edge-backoff collision probing limits, non-converging movement, zero-budget short-circuits and NaN fall-distance behavior.
- Dependencies / cross-references: back_off_from_edge; EdgeBackoffWorld/EdgeBackoffInput; Aabb and Vec3; EdgeBackoffError.
- Implementation / logic: Uses a fake movement world to verify infinite horizontal displacement exhausts a finite query budget, zero budget is tolerated only when no probe is needed, and grounded/NaN cases preserve native short-circuit behavior.
- Findings: Infinite/NaN cases and a tiny fake world do not validate real player movement, collision shape sampling or server-authoritative movement. Tests were not executed.
- RustChunk relevance: Useful bounded-query regression criteria for entity movement; not a replacement for a collision engine.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 Entity.collide/edge-backoff logic and player movement rules must be checked against vanilla/Forge target sources.
- Follow-up checks: Port finite/infinite/NaN/zero-motion cases; include actual collision geometry, server-authority and multiplayer correction.
- RustChunk implementation mapping: Use as edge-case regression fixtures for target entity movement collision probing, query budgets and NaN/infinite movement semantics.
- Evidence required before adoption: Official 1.20.1 Entity/Player movement and collision sources, plus executed simulation and server-client tests.
- Completion state: COMPLETE — 109 logical lines through EOF; pinned SHA and 3,138-byte size match; inspected only; tests not executed.

### 2769. `crates/horizon-world/tests/feature_catalog.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `769ce8bbed9776803e24d895e1f00efeb7b28adc`
- Size: 2231 bytes
- Coverage: Complete through EOF: 62 logical lines; full file; no gaps.
- Purpose: Tests feature catalog canonicalization, duplicate resource IDs, reference cycles, missing references and shared expansion budgets.
- Dependencies / cross-references: FeatureCatalog; TagDefinitions; feature/placed-feature resource maps; DecodeError.
- Implementation / logic: Verifies duplicate canonical keys fail, unselected resource cycles are rejected at lookup, inline no-op definitions remain usable, unknown zero-weight references fail and repeated shared references cannot expand into an oversized tree.
- Findings: Two synthetic resource graphs do not cover full data-pack merge priority, registry aliases or all feature/placement types. Tests were not executed.
- RustChunk relevance: Useful data-pack loading acceptance checklist; test-only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Starling catalog resolution and 1.20.1 datapack registry/resource reload behavior differ materially.
- Follow-up checks: Port duplicate/cycle/unknown-reference and expansion-limit tests; add data-pack priority, tags, disabled feature and reload-generation cases.
- RustChunk implementation mapping: Port canonical resource ID, dependency-cycle, unknown-reference and graph-expansion cases into the target datapack feature catalog tests.
- Evidence required before adoption: Official 1.20.1 RegistryOps/feature resource loading behavior and executed pack reload tests.
- Completion state: COMPLETE — 62 logical lines through EOF; pinned SHA and 2,231-byte size match; inspected only; tests not executed.

### 2770. `crates/horizon-world/tests/feature_graph_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `63da02b19ed8ac009057cad0c4ac89ef3ef4f1bd`
- Size: 3186 bytes
- Coverage: Complete through EOF: 89 logical lines; full file; no gaps.
- Purpose: Tests shared work budgets across nested feature graphs, inline decode depth and multiplicative placement-modifier expansion without consuming extra RNG before rejection.
- Dependencies / cross-references: FeatureDefinition/PlacedFeatureDefinition; FeatureExecutionError; FeatureWorld/PlacementContext; PlacementModifier and IntProvider; LegacyRandom.
- Implementation / logic: Builds no-op graph nodes and panic-on-world-query adapters to verify oversized graphs and nested JSON stop at shared limits, modifier multiplication is budgeted, and random state remains unchanged on preflight failure.
- Findings: Synthetic no-op graphs do not exercise stateful world writes or all feature graph node types; tests not run.
- RustChunk relevance: Valuable defense-in-depth validation for bounded configured/placed feature graphs; test-only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 configured/placed feature codecs and scheduler semantics must be compared with Mojang code; Rust-side work limits are implementation safeguards.
- Follow-up checks: Port graph depth/node/expansion checks and RNG invariance; test recursive structures, mixed placement chains, state mutations and rollback behavior.
- RustChunk implementation mapping: Use as adversarial target configured/placed-feature tests for shared graph budgets, depth limits and no-random-draw preflight errors.
- Evidence required before adoption: Official 1.20.1 ConfiguredFeature/PlacedFeature and placement modifier code, plus executed adversarial unit tests.
- Completion state: COMPLETE — 89 logical lines through EOF; pinned SHA and 3,186-byte size match; inspected only; tests not executed.

### 2771. `crates/horizon-world/tests/feature_placement.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8eb9c1af658a1dcc38495791e629d4721d5607f6`
- Size: 1402 bytes
- Coverage: Complete through EOF: 41 logical lines; full file; no gaps.
- Purpose: Tests placement-modifier chain limits and multiplicative count expansion under a shared feature-placement work budget.
- Dependencies / cross-references: FeaturePlacer/PlacementModifier; WorldGenerationContext; IntProvider; LegacyRandom.
- Implementation / logic: Checks a 1,025-modifier chain fails before consuming random numbers or invoking the feature, and two large count modifiers stop at the shared work cap.
- Findings: This narrowly tests chain and multiplicative count bounds; no individual modifier semantics, real feature callback or world adapter is exercised. Not run.
- RustChunk relevance: Useful bounded scheduler specification; test-only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Target 1.20.1 placement modifier list, order and count semantics require direct comparison.
- Follow-up checks: Port chain and multiplicative-count tests, then add each 1.20.1 modifier's output order, random draw and failure handling.
- RustChunk implementation mapping: Port modifier-chain length and multiplicative count budget assertions into the target placement pipeline tests.
- Evidence required before adoption: Official 1.20.1 PlacementModifier codecs/PlacementContext and executed RNG fixtures.
- Completion state: COMPLETE — 41 logical lines through EOF; pinned SHA and 1,402-byte size match; inspected only; tests not executed.

### 2772. `crates/horizon-world/tests/feature_region.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6b6f3e52da86ebae24e4274f9a5c48d56a9525eb`
- Size: 20027 bytes
- Coverage: Complete through EOF: 528 logical lines; 1–400, 401–528.
- Purpose: Tests terrain-region chunk routing, negative/sparse coordinates, column scans, write callbacks, post-processing, block/fluid/tag bindings, retrogen write limits, biome visibility and heightmap-update parity.
- Dependencies / cross-references: TerrainRegion/TerrainFeatureAccess; TerrainChunk; FeatureWorld/PlacementContext; registry, heightmaps, fluids, tags, retrogen and biome storage.
- Implementation / logic: Uses synthetic registries/chunks and callbacks to verify region construction validation, negative coordinate routing, scan_column parity, callback-controlled mutations/flags, post-processing section indexing, binding snapshot identity, retrogen write-zone/height policy, live upgrade transitions, biome reads and fast-vs-per-write heightmap equality.
- Findings: The largest test file here remains an artificial adapter fixture: it does not execute real generation stages, disk chunk serialization, concurrent feature writes or target-version Java parity. No embedded tests were run.
- RustChunk relevance: High-value integration specification for chunk/feature world access and retrogen gating; test-only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Starling's chunk stages, biome quart semantics, callbacks and retrogen state differ from 1.20.1/Forge; individual rules must be remapped.
- Follow-up checks: Port as integration tests over the target chunk representation; add actual Java NBT round-trip, sparse-neighbor generation, concurrent/scheduled decoration and full retrogen transition tests.
- RustChunk implementation mapping: Port as integration tests over target terrain regions for chunk routing, callbacks, retrogen, biome reads, fluids and heightmaps.
- Evidence required before adoption: Official 1.20.1 WorldGenRegion/LevelChunk/BelowZeroRetrogen/Heightmap/biome container source and executed end-to-end tests.
- Completion state: COMPLETE — 528 logical lines through EOF; pinned SHA and 20,027-byte size match; inspected only; tests not executed.

### 2773. `crates/horizon-world/tests/feature_sort_bounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `84bbfb05e63334ee569cb22bd3b860757d5eddd7`
- Size: 1285 bytes
- Coverage: Complete through EOF: 34 logical lines; full file; no gaps.
- Purpose: Tests feature dependency sorting with very long chains, bounded work, cycle detection and preservation of empty/repeated per-step groups.
- Dependencies / cross-references: build_features_per_step; FeatureIdentity and FeatureSortError.
- Implementation / logic: Checks 50,000 sequential features are handled iteratively, 65,536 duplicate references exceed the work limit, opposite biome orders and repeated identities produce cycles, and empty step groups remain in output.
- Findings: Only small synthetic dependency structures are used; resource registry ordering and real configured-feature dependencies are not integrated. Tests not executed.
- RustChunk relevance: Useful test-only acceptance criteria for scalable deterministic feature sorting.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 feature ordering and biome step collection require direct target-era comparison.
- Follow-up checks: Port iterative traversal and duplicate/cycle tests, then add real biome-feature graphs, cross-biome ordering and deterministic topological tie cases.
- RustChunk implementation mapping: Use as graph-algorithm regression fixtures for long dependency lists, cycle detection and stable per-step grouping.
- Evidence required before adoption: Official 1.20.1 FeatureSorter and biome feature-step sources with executed graph fixtures.
- Completion state: COMPLETE — 34 logical lines through EOF; pinned SHA and 1,285-byte size match; inspected only; tests not executed.

### 2774. `crates/horizon-world/tests/material_rule_specialization.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d70cd04175183adc1dd139472784a0afd6c9ede1`
- Size: 5701 bytes
- Coverage: Complete through EOF: 189 logical lines; full file; no gaps.
- Purpose: Tests material-rule specialization, constant biome-branch short-circuiting, eager/lazy density binding order and preservation of dynamic query errors.
- Dependencies / cross-references: MaterialConditions/MaterialRule/MaterialCondition; MaterialSurfaceQueries; DensitySampler; BlockPos and XoroshiroRandom.
- Implementation / logic: Verifies constant biome predicates avoid unnecessary biome/temperature queries while keeping binding order, empty biome lists match nothing, unreachable child binding failures do not partially add a rule, and dynamic temperature queries run/rethrow errors on every evaluation.
- Findings: Synthetic query counters verify call contracts but do not compare the specialized rule compiler against 1.20.1 surface-rule semantics or real noise samplers. Tests not run.
- RustChunk relevance: Useful regression specification for optimizing surface rules without changing observable query order; test-only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 SurfaceRules and density function optimization are different and must not inherit later optimizer assumptions.
- Follow-up checks: Port against 1.20.1 SurfaceRules; assert identical block output, random calls, sampler bind/query order and error propagation before/after specialization.
- RustChunk implementation mapping: Port density-binding order, constant-branch short-circuit and dynamic query error assertions into target surface-rule evaluator tests.
- Evidence required before adoption: Official 1.20.1 SurfaceRules, DensityFunction and biome-temperature code with executed differential tests.
- Completion state: COMPLETE — 189 logical lines through EOF; pinned SHA and 5,701-byte size match; inspected only; tests not executed.

### 2775. `crates/horizon-world/tests/material_stage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa17bf0e9677731a544d67f6b2ea66131353a293`
- Size: 9891 bytes
- Coverage: Complete through EOF: 295 logical lines; full file; no gaps.
- Purpose: Tests material surface/top-material stages preserving caller density-cache and world-field context across success, binding failure, partial chunk failure and biome-query failure.
- Dependencies / cross-references: build_surface/top_material; MaterialRule/MaterialCondition; TerrainChunk; SamplerContext/CachingDensitySampler/WorldFields; MaterialExtensions; MaterialSurfaceQueries.
- Implementation / logic: Uses counting/failing density samplers and synthetic terrain to check cache identity and world-field retention through success/failure, partial block writes/postprocessing, repeated biome queries and errors; also ensures possible-biome shortcuts do not suppress required queries.
- Findings: These are controlled tests for context/error semantics, not full terrain-noise output or target-version surface-rule parity. Tests not executed.
- RustChunk relevance: Useful regression suite for retaining sampler caches and error semantics during surface-stage optimization; test-only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 noise/surface pipeline, cache lifecycle and extension behavior differ; the same assertions need target-specific fixtures.
- Follow-up checks: Port context preservation, failure transactionality, biome query ordering and no-stale-cache assertions to the target material/terrain stage.
- RustChunk implementation mapping: Port caller-cache retention, world-field continuity, partial failure and biome-query behavior into target surface/terrain stage integration tests.
- Evidence required before adoption: Official 1.20.1 NoiseBasedChunkGenerator/SurfaceRules/RandomState and executed cache/error parity tests.
- Completion state: COMPLETE — 295 logical lines through EOF; pinned SHA and 9,891-byte size match; inspected only; tests not executed.

### 2776. `crates/starling-catalog/tests/catalog.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `23303f922e0bf811364fa8ee82dff5af6bd48821`
- Size: 6205 bytes
- Coverage: Complete through EOF: 187 logical lines; full file; no gaps.
- Purpose: Catalog Project/Source/Kind helpers; serde_json, ZIP and temp files.
- Dependencies / cross-references: Synthetic API responses/ZIP files test metadata mapping, shader detection, install-list reconciliation and removal.
- Implementation / logic: Synthetic fixtures only; no live service, real archives or concurrent filesystem behavior.
- Findings: Narrow catalog metadata test checklist, not game code.
- RustChunk relevance: No direct 1.20.1 gameplay behavior; packs must match target loader/layout.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Add corrupt/oversized ZIPs, bad manifests, concurrent updates and Forge metadata.
- Follow-up checks: Port selected assertions to RustChunk catalog tests.
- RustChunk implementation mapping: Target pack metadata, install layout and executed archive/filesystem tests.
- Evidence required before adoption: Executed RustChunk catalog tests against real 1.20.1/Forge pack metadata, corrupt and oversized archives, traversal-safe paths, concurrent install/remove and rollback; verify no server-only catalog dependencies ship in the Android client.
- Completion state: COMPLETE — 187 logical lines through EOF; pinned source SHA and 6,205 byte size match; inspected only; tests not executed.

### 2777. `crates/starling-catalog/tests/list.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c7e187fb90f4e100ee20eb1221f3e22bee3d9872`
- Size: 11313 bytes
- Coverage: Complete through EOF: 329 logical lines; full file; no gaps.
- Purpose: Feed/Scroller/Tail helpers; Project; PAGE/SLOP constants.
- Dependencies / cross-references: Synthetic pages test offsets, deduplication, stale-request rejection, retries, visible rows and touch transitions.
- Implementation / logic: No real GPUI event loop, Android touch dispatch or live network timing.
- Findings: Potential catalog UI checklist, not engine subsystem.
- RustChunk relevance: No 1.20.1 semantics; use Android UI contracts.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Port stale/retry cases and test rotation, slow network and pointer cancellation.
- Follow-up checks: Use as launcher async-list test cases.
- RustChunk implementation mapping: RustChunk UI framework and executed device/emulator tests.
- Evidence required before adoption: Executed Android UI tests on the selected RustChunk UI framework for pagination, stale-request rejection, retries, scrolling, rotation, slow-network behavior, pointer cancellation and accessibility.
- Completion state: COMPLETE — 329 logical lines through EOF; pinned source SHA and 11,313 byte size match; inspected only; tests not executed.

### 2778. `launcher/build.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bc2ea1f12eb473908cac7f98366062affb02faa3`
- Size: 672 bytes
- Coverage: Complete through EOF: 15 logical lines; full file; no gaps.
- Purpose: Tracks packaging/icons/horizon.ico and invokes winresource only for Windows target.
- Dependencies / cross-references: CARGO_MANIFEST_DIR; winresource; Horizon ICO.
- Implementation / logic: Emits rerun-if-changed and embeds executable icon on Windows.
- Findings: Desktop packaging only; no Android, renderer or game role.
- RustChunk relevance: No RustChunk engine reuse; at most desktop packaging reference.
- Decision: unnecessary; version-specific
- 1.20.1 implications: No 1.20.1 behavior; Android packaging differs.
- Follow-up checks: Revisit only for a future RustChunk Windows release.
- RustChunk implementation mapping: Keep icon embedding target-specific.
- Evidence required before adoption: Defined desktop release target and build requirements.
- Completion state: COMPLETE — 15 logical lines through EOF; pinned source SHA and 672 byte size match; inspected only; tests not executed.

### 2779. `launcher/src/account.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ac46ddc2866bd9f434ba1b18b03d044f5fdbb8e9`
- Size: 81480 bytes
- Coverage: Complete through EOF: 2135 logical lines; 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2135; no gaps.
- Purpose: OAuth device flow; Xbox/XSTS; Minecraft Services; entitlement/profile; token renewal; account storage; offline-name and session handoff.
- Dependencies / cross-references: ureq/proxy; serde_json; Microsoft/Xbox/Minecraft; SecretStore/session locks; keychain; AEAD/SHA-256; backend.
- Implementation / logic: Polls OAuth, exchanges Xbox/XSTS, gets Minecraft profile, checks ownership, stores secrets and writes game session file.
- Findings: Default launch requires Java Edition ownership; offline names are gated unless hidden developer override. Fallback key is machine identity plus compiled constant; local permissions still matter. No attack reproduced.
- RustChunk relevance: Do not port Microsoft-only identity/owner gate; secret isolation/session handoff are references only.
- Decision: unnecessary; reference; version-specific
- 1.20.1 implications: Forge 47/1.20.1 authlib, UUID/session and offline server rules need independent design.
- Follow-up checks: Separate offline/online identities; threat-model storage, permissions, key rotation, response caps, proxy and session cleanup.
- RustChunk implementation mapping: Keep offline profiles first-class; optional Microsoft provider must not gate offline play.
- Evidence required before adoption: 1.20.1 authlib/session, Forge launch args, server online-mode and credential-storage review.
- Completion state: COMPLETE — 2135 logical lines through EOF; pinned source SHA and 81,480 byte size match; inspected only; tests not executed.

### 2780. `launcher/src/account/session.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dcd007443c3839db8833ba4759ea4dd7a06e2c5b`
- Size: 12846 bytes
- Coverage: Complete through EOF: 356 logical lines; full file; no gaps.
- Purpose: Serializes refresh/sign-in/sign-out and reports account state without exposing credentials.
- Dependencies / cross-references: Account/Secrets/SecretStore; filesystem locks; process mutex; timeout/status enums.
- Implementation / logic: Acquires process and cross-process locks, reads saved state and refreshes credentials before handoff.
- Findings: Lock errors fail closed; busy status is bounded; tightly coupled to Horizon keychain.
- RustChunk relevance: Concurrency reference only, not core offline identity.
- Decision: adaptable; reference; optional/future
- 1.20.1 implications: No game behavior; locking must match Android storage/process model.
- Follow-up checks: Test contention, process death, stale lock files and refresh/signout races.
- RustChunk implementation mapping: Use provider-neutral operation lock only if RustChunk adds shared credential refresh.
- Evidence required before adoption: Android filesystem/keystore and multi-process tests.
- Completion state: COMPLETE — 356 logical lines through EOF; pinned source SHA and 12,846 byte size match; inspected only; tests not executed.

### 2781. `launcher/src/account/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b9780413c67d40078d5736cd4b6f99669533a81`
- Size: 77183 bytes
- Coverage: Complete through EOF: 2086 logical lines; 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2086; no gaps.
- Purpose: Tests OAuth/ownership, offline names, secret stores, keyring, file sealing and session lifecycle.
- Dependencies / cross-references: account.rs/session.rs; fake endpoints; MemoryStore/FileStore; mock keyring; temp dirs.
- Implementation / logic: Mock replies test success/failure, persistence fallback, account switching, refresh and session handoff.
- Findings: Does not prove Android Keystore, Forge launch or RustChunk storage; not run.
- RustChunk relevance: Test-only; do not inherit the paid-owner requirement for offline names.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Platform keychain tests do not cover Android or target offline identity.
- Follow-up checks: Test offline profiles, UUID consistency, session cleanup, crash recovery and provider failures.
- RustChunk implementation mapping: Create tests beside RustChunk profile/session and Android storage adapter.
- Evidence required before adoption: RustChunk profile semantics, Android Keystore design and executed tests.
- Completion state: COMPLETE — 2086 logical lines through EOF; pinned source SHA and 77,183 byte size match; inspected only; tests not executed.

### 2782. `launcher/src/agents.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2baf198c618c7f8d8945c152f5d78f2c937c6b85`
- Size: 7793 bytes
- Coverage: Complete through EOF: 214 logical lines; full file; no gaps.
- Purpose: Builds client configs, locates CLIs and self-tests the MCP server.
- Dependencies / cross-references: serde_json; PATH/home; process wrapper; MCP command.
- Implementation / logic: Serializes JSON/TOML and add commands, shell-quotes args and starts MCP child.
- Findings: wait_with_output has no visible deadline or output cap; potential hang/memory use, not reproduced.
- RustChunk relevance: Optional developer tooling only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Android agent bridge needs another process/permission model.
- Follow-up checks: Add explicit permission, process isolation, timeout and output limits if ever implemented.
- RustChunk implementation mapping: Exclude desktop MCP from RustChunk core.
- Evidence required before adoption: Concrete agent use case, secure process design and bounded tests.
- Completion state: COMPLETE — 214 logical lines through EOF; pinned source SHA and 7,793 byte size match; inspected only; tests not executed.

### 2783. `launcher/src/agents_page.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b40bbd25e3b1c4aaa3802adf43b9375d79742fec`
- Size: 10454 bytes
- Coverage: Complete through EOF: 265 logical lines; full file; no gaps.
- Purpose: Renders No AI, MCP selector, config copy/add and connection-test feedback.
- Dependencies / cross-references: GPUI; agents; backend settings/executable; process helper.
- Implementation / logic: Starts add/test subprocesses only after click.
- Findings: GPUI-bound; add subprocess has no explicit deadline here.
- RustChunk relevance: Not reusable in Android UI; explicit-action pattern is general.
- Decision: unnecessary; version-specific
- 1.20.1 implications: No 1.20.1 semantics; Android process rules differ.
- Follow-up checks: Keep future automation opt-in and permissioned; do not port GPUI widgets.
- RustChunk implementation mapping: Implement native Android settings only if scope requires.
- Evidence required before adoption: Android UI, permission model and subprocess lifecycle tests.
- Completion state: COMPLETE — 265 logical lines through EOF; pinned source SHA and 10,454 byte size match; inspected only; tests not executed.

### 2784. `launcher/src/art.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f0ab08cd34ef4cbabbdf5bcb49e596062383a202`
- Size: 1639 bytes
- Coverage: Complete through EOF: 37 logical lines; full file; no gaps.
- Purpose: Converts RGBA texels to scaled premultiplied BGRA GPUI RenderImage.
- Dependencies / cross-references: gpui::RenderImage; image::RgbaImage/Frame; Arc.
- Implementation / logic: Maps output pixels to source texels and premultiplies alpha.
- Findings: Assumes nonzero dimensions and adequate texel slice; invalid caller can underflow/panic.
- RustChunk relevance: No engine reuse; generic image resampling only.
- Decision: unnecessary; version-specific
- 1.20.1 implications: Not Android GLES or 1.20.1 rendering.
- Follow-up checks: Validate dimensions/overflow/buffer length if reused.
- RustChunk implementation mapping: Keep artwork separate from chunk mesh/render path.
- Evidence required before adoption: RustChunk image contract, bounds tests and device benchmark.
- Completion state: COMPLETE — 37 logical lines through EOF; pinned source SHA and 1,639 byte size match; inspected only; tests not executed.

### 2785. `launcher/src/backend.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `033750209092de07802512c30c437022a3bfbce2`
- Size: 213664 bytes
- Coverage: Complete through EOF: 5146 logical lines; 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4800, 4801–5146; no gaps.
- Purpose: Coordinates launcher state, settings/data paths, releases, instances/files, process launch and downloads/updates.
- Dependencies / cross-references: GPUI; account/session; Java prep; release metadata; process/proxy/download; filesystem/config; Horizon version modules.
- Implementation / logic: Exposes UI/CLI operations to prepare/launch game, persist settings and coordinate child processes/files.
- Findings: Broad responsibilities and desktop assumptions create high coupling; archive/path/process/update behavior needs independent threat review. No flaw reproduced.
- RustChunk relevance: Architecture map only; split storage, Java, metadata, instance and process services.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Forge 1.20.1 differs in libraries, launch args, mappings and runtime; no direct carry-over.
- Follow-up checks: Trace call sites; review traversal, archive extraction, limits, rollback, child lifecycle and secret redaction.
- RustChunk implementation mapping: Rebuild separate Forge profile, Java 17, sandbox, downloader and process lifecycle services.
- Evidence required before adoption: Pinned 1.20.1/Forge sources, Android tests and end-to-end launch.
- Completion state: COMPLETE — 5146 logical lines through EOF; pinned source SHA and 213,664 byte size match; inspected only; tests not executed.

### 2786. `launcher/src/backend/java_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3f46123b8d415673729b03963bc469f10d5cc2ef`
- Size: 3108 bytes
- Coverage: Complete through EOF: 85 logical lines; full file; no gaps.
- Purpose: Verifies JAVA_HOME/PATH and child java/javac selection.
- Dependencies / cross-references: backend java/with_java; process wrapper; rustc probe; fake JRE/JDK.
- Implementation / logic: Creates fake JRE 21/JDK 25 and invokes child assertion.
- Findings: Horizon Java 25 fixture; not 1.20.1 validation and not run.
- RustChunk relevance: Test-only subprocess environment idea.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 needs verified Java 17.
- Follow-up checks: Test Java 17 selection/environment in Forge preparation and launch.
- RustChunk implementation mapping: Use target-version tests in RustChunk Java module.
- Evidence required before adoption: Forge 47 requirements and executed Android subprocess tests.
- Completion state: COMPLETE — 85 logical lines through EOF; pinned source SHA and 3,108 byte size match; inspected only; tests not executed.

### 2787. `launcher/src/backend/release_loading_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d7f839bdefef7c7fa12dbb9ecf8849f440ee1fa3`
- Size: 8292 bytes
- Coverage: Complete through EOF: 240 logical lines; full file; no gaps.
- Purpose: Exercises release parsing and asset selection successes/failures.
- Dependencies / cross-references: Backend release helpers; synthetic JSON; filesystem/error fixtures.
- Implementation / logic: Checks metadata parsing, selection and failure handling.
- Findings: No Mojang 1.20.1 manifests, Forge Maven metadata or real downloads; not run.
- RustChunk relevance: Checklist for target release resolver tests.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1/Forge metadata graphs and classifiers differ.
- Follow-up checks: Add Mojang manifest, Forge installer, libraries, assets and checksum fixtures.
- RustChunk implementation mapping: Use in RustChunk version/Forge resolver tests.
- Evidence required before adoption: Official 1.20.1 manifest, Forge metadata and executed resolver tests.
- Completion state: COMPLETE — 240 logical lines through EOF; pinned source SHA and 8,292 byte size match; inspected only; tests not executed.

### 2788. `launcher/src/blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e291a34e9a518985bf3a7dbb0fc2c760d1a24262`
- Size: 6489 bytes
- Coverage: Complete through EOF: 194 logical lines; full file; no gaps.
- Purpose: Defines asset source, icons/text, panel edges, badges, tabs and image slots.
- Dependencies / cross-references: GPUI; horizon_theme icons; theme/translations.
- Implementation / logic: Serves embedded SVG bytes and builds reusable GPUI visual elements.
- Findings: Pure presentation layer with GPUI types; no world block/registry logic.
- RustChunk relevance: No engine reuse; visual ideas can be recreated in Android UI.
- Decision: unnecessary; version-specific
- 1.20.1 implications: No 1.20.1 behavior; UI stack differs.
- Follow-up checks: Keep UI assets/widgets outside engine; check asset licensing if reused.
- RustChunk implementation mapping: Recreate visuals in native Android frontend; no GPUI in core.
- Evidence required before adoption: RustChunk UI framework, licenses and device layout/render tests.
- Completion state: COMPLETE — 194 logical lines through EOF; pinned source SHA and 6,489 byte size match; inspected only; tests not executed.

### 2789. `launcher/src/browser.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2a40719248d97675138b978565536f92f027584d`
- Size: 15072 bytes
- Coverage: Complete through EOF: 432 logical lines; 1–400, 401–432; no gaps.
- Purpose: Defines content-browser model/UI for Modrinth, Horizon registry and optional CurseForge.
- Dependencies / cross-references: Catalog Source/Project/Version; Horizon registry/pack installer; downloads; GPUI; game/loader metadata.
- Implementation / logic: Routes search/details to provider adapters and applies Horizon game-version/loader constraints.
- Findings: GAME 26.3 and native .hmod model; loader labels are not Forge 47 support evidence.
- RustChunk relevance: Provider-adapter and catalog-flow reference only.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: RustChunk needs Forge 1.20.1 loader/dependency matching and .jar placement.
- Follow-up checks: Define Forge filters, dependencies, side restrictions, artifact verification and rollback.
- RustChunk implementation mapping: Build separate RustChunk Forge 1.20.1 browser; do not reuse Horizon package model.
- Evidence required before adoption: Current provider APIs, Forge manifests and executed install/launch tests.
- Completion state: COMPLETE — 432 logical lines through EOF; pinned source SHA and 15,072 byte size match; inspected only; tests not executed.

### 2790. `launcher/src/browser/curseforge.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `68002c5eded40ae6d505b8e7789f692a1e9d3471`
- Size: 8167 bytes
- Coverage: Complete through EOF: 238 logical lines; full file; no gaps.
- Purpose: Implements optional CurseForge key discovery, search, project/version and dependency parsing.
- Dependencies / cross-references: CurseForge API; runtime/build key; catalog types; download agent; GAME constant.
- Implementation / logic: Maps class IDs, queries endpoints, parses versions/loaders and records dependencies and SHA-1 URLs.
- Findings: option_env! can embed a build-time key in binary; digit-prefix heuristic and SHA-1-only metadata need review.
- RustChunk relevance: Provider boundary adaptable; credentials/default version need redesign.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Default GAME 26.3; request 1.20.1/Forge explicitly.
- Follow-up checks: Keep private keys out of public binaries; test pagination, malformed versions, unavailable files and dependency graph.
- RustChunk implementation mapping: Implement explicit target-version/Forge adapter with bounded parse and verified downloads.
- Evidence required before adoption: Current API contract, secret plan and executed 1.20.1 metadata/download tests.
- Completion state: COMPLETE — 238 logical lines through EOF; pinned source SHA and 8,167 byte size match; inspected only; tests not executed.

### 2791. `launcher/src/browser/details.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b7b8c3fc0e90e4cd28a8ea337ccefbee90a4a7ce`
- Size: 11955 bytes
- Coverage: Complete through EOF: 366 logical lines; full file; no gaps.
- Purpose: Renders project details, versions, dependencies, installed status and install actions.
- Dependencies / cross-references: GPUI; Project/Version/Source; catalog/download; theme/translations.
- Implementation / logic: Builds detail view and delegates download/install to provider helpers.
- Findings: Bound to Horizon UI/provider model; not portable to Android frontend.
- RustChunk relevance: No engine reuse; behavior may inform a distinct Android screen.
- Decision: unnecessary; version-specific
- 1.20.1 implications: No 1.20.1 behavior; loader rules must come from provider adapters.
- Follow-up checks: Design Android details with target match, dependency tree, verification and rollback.
- RustChunk implementation mapping: Keep presentation separate from provider/catalog service.
- Evidence required before adoption: RustChunk UI design, Forge metadata and device tests.
- Completion state: COMPLETE — 366 logical lines through EOF; pinned source SHA and 11,955 byte size match; inspected only; tests not executed.

### 2792. `launcher/src/browser/download.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `47af5375c89fa7901b0c7a891db505f313240920`
- Size: 7739 bytes
- Coverage: Complete through EOF: 203 logical lines; full file; no gaps.
- Purpose: HTTP download config, URL/host restrictions, safe filenames, size limits and checksum verification.
- Dependencies / cross-references: ureq; URL parser; source metadata; SHA-1/256/512; paths.
- Implementation / logic: Validates host/name, downloads to .part, checks size and verifies hashes.
- Findings: Fixed filename.part may collide for concurrent same-name downloads and leave stale partial data; potential, not reproduced.
- RustChunk relevance: Adapt security pattern after unique temp files/path containment/cancellation.
- Decision: adaptable; reference; optional/future
- 1.20.1 implications: 1.20.1 artifacts have other metadata/hosts; integrity logic remains useful.
- Follow-up checks: Unique temp names, atomic finalize, redirect checks, cancellation cleanup and concurrency tests.
- RustChunk implementation mapping: Central RustChunk downloader with allowlists, artifact identity, caps and atomic rename.
- Evidence required before adoption: Android FS semantics, provider hosts and executed corruption/interruption/concurrency tests.
- Completion state: COMPLETE — 203 logical lines through EOF; pinned source SHA and 7,739 byte size match; inspected only; tests not executed.

### 2793. `launcher/src/browser/horizon.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `43f7c0c2c9355e8168a735e6e272b77fc3a5666e`
- Size: 30657 bytes
- Coverage: Complete through EOF: 815 logical lines; 1–400, 401–800, 801–815; no gaps.
- Purpose: Implements registry source, signature loading, metadata validation, revocations and dependencies.
- Dependencies / cross-references: Registry docs/signatures; SHA-256; Ed25519; source types; download/pack installer.
- Implementation / logic: Validates registry and hashes/revocations, resolves entries/dependencies and delegates installation.
- Findings: Horizon trust/package formats are specific; partial mutations possible during multi-file update; no flaw reproduced.
- RustChunk relevance: Signed-metadata principle is reference; trust root/installer require redesign.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Horizon .hmod is not Forge 1.20.1 JAR loading.
- Follow-up checks: Design key rotation, revocation, cycles, hashes, atomic transactions and native-code trust.
- RustChunk implementation mapping: Use signed metadata only for RustChunk-controlled content; keep Forge resolver separate.
- Evidence required before adoption: Key custody/threat model, Forge format and executed tamper/rollback tests.
- Completion state: COMPLETE — 815 logical lines through EOF; pinned source SHA and 30,657 byte size match; inspected only; tests not executed.

### 2794. `launcher/src/browser/horizon/pack.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f6c6f880cc163a531441a6e6793971b0037cb1f7`
- Size: 11584 bytes
- Coverage: Complete through EOF: 305 logical lines; full file; no gaps.
- Purpose: Installs/updates packs, dependencies, mods, overrides and manifest.
- Dependencies / cross-references: Registry/project/version; downloads/checksums; overrides; pack.json; dependency/revocation.
- Implementation / logic: Resolves requirements, replaces files sequentially, copies overrides on first install and writes manifest last.
- Findings: Failure after some replacements can leave partial pack; fresh-install tests do not prove update rollback. Not reproduced.
- RustChunk relevance: Checklist only; do not transplant installer.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Forge modpacks have loader/config/override dependencies; Horizon .hmod assumptions differ.
- Follow-up checks: Stage everything, preflight, atomic swap/rollback, safe extraction and injected failure tests.
- RustChunk implementation mapping: Separate RustChunk Forge pack installer with recovery journal.
- Evidence required before adoption: Forge 47 pack rules, Android rename behavior and rollback tests.
- Completion state: COMPLETE — 305 logical lines through EOF; pinned source SHA and 11,584 byte size match; inspected only; tests not executed.

### 2795. `launcher/src/browser/horizon/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0b33132be41fb1c290639f0095c95a4b4ece3409`
- Size: 13700 bytes
- Coverage: Complete through EOF: 361 logical lines; full file; no gaps.
- Purpose: Tests signature, checksum, revocation, dependencies, pack installs and failures.
- Dependencies / cross-references: Horizon registry/pack; fake signatures/registry; temp dirs; mock downloads.
- Implementation / logic: Synthetic fixtures verify signature acceptance, hashes, dependencies, revocations and install paths.
- Findings: Horizon package model only; no Forge JARs/live provider/complete rollback coverage; not run.
- RustChunk relevance: Test-only reference for trust/failure scenarios.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: No direct 1.20.1 compatibility; Forge manifests need separate tests.
- Follow-up checks: Port tamper scenarios after defining trust model; add traversal/rollback/Forge dependencies.
- RustChunk implementation mapping: Add target provider tests; don't use fixtures as production metadata.
- Evidence required before adoption: RustChunk trust-root design, Forge metadata and executed adversarial installer tests.
- Completion state: COMPLETE — 361 logical lines through EOF; pinned source SHA and 13,700 byte size match; inspected only; tests not executed.

### 2796. `launcher/src/browser/images.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `feb4aa1b79272049ac34e675a58ce1c9242f6259`
- Size: 18951 bytes
- Coverage: Complete through EOF: 472 logical lines; 1–400, 401–472; no gaps.
- Purpose: Fetches project/gallery images over HTTPS, scales, caches PNGs, prunes old files and extracts icons from mod JAR metadata.
- Dependencies / cross-references: Modrinth/CurseForge/Fabric/Forge/Horizon image hosts; ureq; SHA-1 URL cache key; image decoder; ZIP/JSON/TOML.
- Implementation / logic: Restricts URL prefixes, limits download to 8 MiB, decoded dimensions to 8192 and allocation to 256 MiB, uses HTTP validators and keeps cache under 128 MiB.
- Findings: Fixed .part path for each URL/size can collide on concurrent writes; icon path check rejects '..' but is not full canonical-path policy. No race reproduced.
- RustChunk relevance: Useful bounded thumbnail/cache design for launcher artwork, not game textures.
- Decision: adaptable; reference; optional/future
- 1.20.1 implications: Not gameplay/render pipeline code; Android decoder and memory pressure need device tests.
- Follow-up checks: Use unique temp files, strict URL/redirect validation, memory/dimension limits, cancellation and concurrency tests.
- RustChunk implementation mapping: Build dedicated RustChunk thumbnail cache separate from chunk texture/mesh renderer.
- Evidence required before adoption: Android decoder/GPU constraints, provider CDN policy and executed decompression-bomb/cache-race tests.
- Completion state: COMPLETE — 472 logical lines through EOF; pinned source SHA and 18,951 byte size match; inspected only; tests not executed.

### 2797. `launcher/src/browser/installed.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c4d33270290272ab186c3e856adf56ab0eee9716`
- Size: 14090 bytes
- Coverage: Complete through EOF: 384 logical lines; full file; no gaps.
- Purpose: Tracks installed catalog files; plans dependencies; installs, updates, removes files and imports data packs.
- Dependencies / cross-references: Project/Version/Source; mod-browser.json; Java/Horizon installers; downloader/checksum; filesystem.
- Implementation / logic: Loads installed entries, resolves bounded dependencies, downloads steps and updates ownership manifest.
- Findings: Malformed manifest rows are silently dropped; fixed .part path and stepwise filesystem writes mean later failure can leave partial changes; potential recovery issue.
- RustChunk relevance: Use ownership manifest idea only; rewrite planner and filesystem transaction.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Horizon mods go to horizon-mods; Forge JARs belong in each Forge instance mods directory.
- Follow-up checks: Preflight dependency tree, stage artifacts, atomic swap/rollback, unique temp files and corrupt-manifest recovery.
- RustChunk implementation mapping: Implement instance-scoped Forge inventory and transactional dependency installer.
- Evidence required before adoption: Forge 47 metadata, Android storage semantics and failure-injection tests.
- Completion state: COMPLETE — 384 logical lines through EOF; pinned source SHA and 14,090 byte size match; inspected only; tests not executed.

### 2798. `launcher/src/browser/installed/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `152ff03c305ad6e83d695bc52bfebf98426a9e78`
- Size: 15772 bytes
- Coverage: Complete through EOF: 430 logical lines; 1–400, 401–430; no gaps.
- Purpose: Tests install/update/remove, dependency order, checksum rejection, user-file protection, ported mod clashes, legacy shader rows and data packs.
- Dependencies / cross-references: installed.rs/java.rs; fake Source; download::save; SHA-512; temporary dirs; synthetic ZIP/JAR.
- Implementation / logic: Verifies dependency-first install, update, removal, checksum mismatch, user-file preservation, namespace clash and datapack import.
- Findings: Horizon-specific layout and Java-port assumptions; no Forge JAR loading or full rollback injection. Not run.
- RustChunk relevance: Test-only reference for ownership/dependency/corruption cases.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Horizon content is not Forge 1.20.1 mod-loading.
- Follow-up checks: Port dependency/checksum/user-file tests and inject failure at each filesystem boundary.
- RustChunk implementation mapping: Add target tests beside RustChunk Forge instance manager/datapack importer.
- Evidence required before adoption: Forge 47 artifacts, Android path semantics and executed failure-injection tests.
- Completion state: COMPLETE — 430 logical lines through EOF; pinned source SHA and 15,772 byte size match; inspected only; tests not executed.

### 2799. `launcher/src/browser/java.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4ac7ae084209d8566c4cf12941c975ea2460ea75`
- Size: 13051 bytes
- Coverage: Complete through EOF: 351 logical lines; full file; no gaps.
- Purpose: Checks JARs, caches results by checksum, extracts portable content and installs it under horizon-mods.
- Dependencies / cross-references: horizon_port_pack check/verdict; Source/Project/Version; installed entries; checksum cache; ZIP/filesystem.
- Implementation / logic: Downloads JAR, checks it, stores verdict/carry data, resolves required mods and copies supported assets into Horizon folders.
- Findings: Loader list mentions Forge/Fabric/Quilt/NeoForge, but this translates data rather than executing Java mods. Some nested dependency errors are skipped; replacement removes old folder before rename. Not reproduced.
- RustChunk relevance: Not a Forge loader. JAR-scanning/report concepts only.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Forge 47 needs original Java mods loaded by Forge, not ported into Horizon data.
- Follow-up checks: Use Forge metadata/dependency graph; stage replacement before deleting old install; expose dependency failure.
- RustChunk implementation mapping: Implement Forge-native catalog/installer without horizon_port_pack/horizon-mods.
- Evidence required before adoption: Forge 47 mod lifecycle, metadata and executed rollback/dependency tests.
- Completion state: COMPLETE — 351 logical lines through EOF; pinned source SHA and 13,051 byte size match; inspected only; tests not executed.

### 2800. `launcher/src/browser/java_instance.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa35c0b202cea6b835a19a5bac05358df5b12165`
- Size: 9234 bytes
- Coverage: Complete through EOF: 250 logical lines; full file; no gaps.
- Purpose: Models instance version/loader, filters versions, plans dependencies and installs/disables/updates/removes mods/resource packs.
- Dependencies / cross-references: Installed Entry/Step; Project/Query/Source/Version; loader metadata; filesystem rename; filters.
- Implementation / logic: Constrains search to instance version/loader, prefers exact loader, accepts Fabric on Quilt, installs dependency-first and renames disabled mods with suffix.
- Findings: Uses Horizon install helpers; project IDs deduplicate dependency graph and file/manifest updates are sequential. Partial changes possible.
- RustChunk relevance: Adaptable target-filtering/per-instance state pattern, not drop-in installer.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Bind installs to Minecraft 1.20.1 + Forge 47.x; keep original JARs in instance mods folder.
- Follow-up checks: Test strict Forge filters, dependency cycles/identity, disabled state across update and transactionality.
- RustChunk implementation mapping: Add instance-scoped mod inventory/filter service; Forge JAR loading remains authoritative.
- Evidence required before adoption: Forge 47 metadata, provider loader behavior, Android filesystem and executed install tests.
- Completion state: COMPLETE — 250 logical lines through EOF; pinned source SHA and 9,234 byte size match; inspected only; tests not executed.
