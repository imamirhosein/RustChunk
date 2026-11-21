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
