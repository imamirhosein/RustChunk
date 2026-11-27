# RustChunk Starling Audit Ledger — entries 3601–3800

- Repository: `starling-client/starling`
- Pinned ref: `5432370d573642041585c69b40fb045b27133488`
- Target assessment: RustChunk Minecraft Java 1.20.1 / Forge 47.x / Android
- Authorized scope: exceptional 100-file grant, entries 3601–3700, dated 2026-11-27.
- Selection: first 100 raw-lexically sorted unindexed blobs from the complete pinned recursive tree, excluding all 3,600 existing Index paths.
- Checkpoint cadence: four sequential 25-file checkpoints. Commit all checkpoint records and audit documents in one Git commit, then re-fetch/read-back verify before the next checkpoint.
- Current registered state: **25 source entries (3601–3625)**; this file will grow only under the same 3601–3700 grant. Entries 3626–3700 remain authorized but not yet inspected.

### 3601. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaVillagerFarmingOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 6dc03ad2e82d7ac6149c1ce2546499cf1d1c936e
- **Size:** 1476 bytes
- **Coverage:** Complete EOF coverage (31 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaVillagerFarmingOracle.java
- **Dependencies / cross-references:** Minecraft SharedConstants/Bootstrap, BuiltInRegistries, Identifier, Block, CropBlock and FarmlandBlock; MC26.3 mappings.
- **Implementation / logic:** Bootstraps registries, resolves nine named blocks, iterates every possible state and emits state ID, CropBlock flag, max-age flag and FarmlandBlock flag.
- **Findings:** Javadoc claims coverage of non-CropBlock age properties, but implementation only tests CropBlock.isMaxAge; it does not inspect arbitrary age properties. Unknown identifiers may resolve to the registry default and be silently misrepresented.
- **RustChunk relevance:** Useful narrow vanilla oracle for villager crop maturity and block classification.
- **Decision:** adaptable; reference; version-specific; parity-test-oracle
- **1.20.1 implications:** Block identifiers, registry contents and farming behavior must be rechecked against 1.20.1; later crop types must not enter the target registry.
- **Follow-up checks:** Compare HarvestFarmland and all 1.20.1 crop-age properties, including cocoa and any non-CropBlock age property; add unknown-ID assertions.
- **RustChunk implementation mapping:** Create target-version parity fixtures for crop maturity and farmland classification; keep villager AI behavior in the Java compatibility layer until proven.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3602. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWaterSplashOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 47a85e02dd58c03601fb8f75b207f90ff829ca64
- **Size:** 5769 bytes
- **Coverage:** Complete EOF coverage (34 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWaterSplashOracle.java
- **Dependencies / cross-references:** Minecraft Entity/ArmorStand/ServerLevel, RandomSource legacy/Xoroshiro implementations, particles, sounds, GameEvent, Gson, Unsafe and reflective Entity fields.
- **Implementation / logic:** Allocates actor/world shells without constructors, injects position/dimensions/level/random source, intercepts particle/sound/event calls and supports injected failure or mid-query mutation.
- **Findings:** Constructor-free Unsafe allocation, reflective private-field access and fake server-world state are brittle; proxy only implements nextFloat/nextDouble; oracle is not a live world or client renderer.
- **RustChunk relevance:** Useful for deterministic splash effect, sound selection, event order and RNG continuation tests.
- **Decision:** adaptable; reference; version-specific; parity-test-oracle
- **1.20.1 implications:** Entity splash implementation, sounds, particles and RNG implementation differ by Minecraft version; port observations, not the Java shell.
- **Follow-up checks:** Compare 1.20.1 Entity.doWaterSplashEffect branch order, velocity thresholds, passenger handling, sound pitch/volume, particle offsets and random calls.
- **RustChunk implementation mapping:** Add isolated target-version event trace tests; native Rust may consume captured expected values but must not replace actual Forge entity hooks.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3603. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWeatherRenderOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** fccd3083e1e37609f43b791b9518479bc41df23c
- **Size:** 5247 bytes
- **Coverage:** Complete EOF coverage (87 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWeatherRenderOracle.java
- **Dependencies / cross-references:** WeatherEffectRenderer, VertexConsumer, RandomSource, Vec3, Unsafe allocation and reflective private methods/fields.
- **Implementation / logic:** Recreates the renderer's 32×32 column direction tables, injects them into an Unsafe instance, calls rain/snow column factories and prepareInstances, then captures vertex attributes through a proxy.
- **Findings:** Only CPU-side geometry is observed; no GPU upload, shader, blend/depth state, texture sampling or frame timing is tested. At the center table coordinate distance is zero, producing NaN direction components unless vanilla intentionally preserves that result.
- **RustChunk relevance:** Strong reference for weather column geometry and deterministic vertex parity, not rendering performance.
- **Decision:** adaptable; reference; version-specific; render-parity-oracle
- **1.20.1 implications:** Renderer class/method signatures, vertex APIs and weather formulas are version-specific and must be regenerated for 1.20.1.
- **Follow-up checks:** Check center-column NaN behavior against 1.20.1, camera offsets, rain/snow intensity, light, UVs, vertex winding and shader state.
- **RustChunk implementation mapping:** Port geometry fixtures into a CPU renderer test harness; keep GPU validation as a separate Android device gate.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3604. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldBorderOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 5014bac9264a45701cceb70cfdb2d6f0b420260e
- **Size:** 3600 bytes
- **Coverage:** Complete EOF coverage (43 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldBorderOracle.java
- **Dependencies / cross-references:** WorldBorder, Entity.position, AABB, Vec3, VoxelShape and VanillaFaceSupportOracle.emit; Gson, Unsafe and reflection.
- **Implementation / logic:** Applies center/size/limit/lerp/tick operations, emits status and interpolated bounds for raw float partial ticks, collision shape and configured entity/AABB proximity queries.
- **Findings:** Uses reflection to inject Entity.position and another oracle's private emit method; test input arrays and operation values are trusted; no network synchronization or client rendering is included.
- **RustChunk relevance:** Useful for border geometry and interpolation correctness tests.
- **Decision:** adaptable; reference; version-specific; parity-test-oracle
- **1.20.1 implications:** WorldBorder behavior and signatures must be checked against 1.20.1; this oracle is a later-version behavioral reference only.
- **Follow-up checks:** Compare 1.20.1 border size clamping, lerp expiry, negative/large coordinates, collision shape, warning range and entity proximity.
- **RustChunk implementation mapping:** Build a RustChunk border math test suite from target-version outputs; implement packet synchronization separately.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3605. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldBorderRenderOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 4c57fe6d666a499d46246a129222f6e48755b11c
- **Size:** 14957 bytes
- **Coverage:** Complete EOF coverage (262 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldBorderRenderOracle.java
- **Dependencies / cross-references:** WorldBorderRenderer, RenderSystem, RenderPearl GPU abstractions, DynamicGpuData, Hud, Minecraft/ClientLevel, texture manager, JOML, Unsafe and proxies.
- **Implementation / logic:** Installs stand-in GPU device/encoder/pass, captures buffer floats and draw side indices, drives border operations and frames at fixed time, captures transforms and vignette color.
- **Findings:** The GPU is fully stubbed; no actual Vulkan/OpenGL execution, shader output, texture correctness, synchronization or performance is established. Extensive Unsafe field injection and default-return proxies can conceal unmodeled API calls.
- **RustChunk relevance:** Useful CPU-side border-render state and vertex-buffer parity reference.
- **Decision:** adaptable; reference; version-specific; render-parity-oracle
- **1.20.1 implications:** Renderer/GPU APIs are later-version-specific and not transferable to 1.20.1 without reimplementation.
- **Follow-up checks:** Compare 1.20.1 border renderer, vertex layout, texture, transforms, alpha/tint, draw side selection and vignette; separately test real Android GPU output.
- **RustChunk implementation mapping:** Use as a design for deterministic render-state tests only; keep GPU backend and frame pacing independent.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3606. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldOptionsOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 9b31bcf257e1c0858ccf7126e0727cd64f091d5f
- **Size:** 2921 bytes
- **Coverage:** Complete EOF coverage (55 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldOptionsOracle.java
- **Dependencies / cross-references:** FriendlyByteBuf/Netty, game packet classes, Registries.GAME_RULE, Identifier, ResourceKey and Difficulty.
- **Implementation / logic:** Reads tab-separated cases, selects packet StreamCodec, encodes to a buffer, emits hex bytes and releases the buffer.
- **Findings:** Encoding only: no decode, server handling, acknowledgement or state mutation is tested. Input shape is unchecked and malformed set/value rows can index missing fields; exception paths can skip buffer release.
- **RustChunk relevance:** Useful protocol byte oracle for a small subset of world options packets.
- **Decision:** adaptable; reference; version-specific; protocol-test-oracle
- **1.20.1 implications:** Packet class names, codecs, registry identifiers and wire formats must be compared with 1.20.1; do not reuse 26.3 bytes blindly.
- **Follow-up checks:** Check 1.20.1 packet IDs/codecs, game-rule key serialization, empty collections, malformed rows and both encode/decode round trips.
- **RustChunk implementation mapping:** Add target-version golden packet tests in RustChunk's protocol compatibility layer; keep server-authoritative option state in the JVM/game.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3607. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 4e41b8ffd96d68655c26ea53c8c6e707eeaee6e1
- **Size:** 14328 bytes
- **Coverage:** Complete EOF coverage (217 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldOracle.java
- **Dependencies / cross-references:** Minecraft NBT codecs, RegionFile/RegionFileVersion, PalettedContainer, Strategy, BlockState registry, DFU Codec, Netty streams and filesystem.
- **Implementation / logic:** Creates edge-case NBT specimens and region chunks for compression IDs 1–4, emits palette fixtures over boundary cardinalities, exports registry states, then compares Rust-produced artifacts by decoding them through vanilla classes.
- **Findings:** Large 1.2 MB payloads and deep/mixed NBT fixtures are valuable stress cases; fixture semantics depend on 26.3 registry and palette behavior. Verify mode assumes Rust artifacts exist and does not itself validate production world saves.
- **RustChunk relevance:** High-value reference for world storage interoperability, NBT and palette edge cases.
- **Decision:** adaptable; reference; version-specific; serialization-test-oracle
- **1.20.1 implications:** NBT fundamentals persist but region versions, DataVersion, palette thresholds, registry IDs and chunk schema must be re-derived for 1.20.1.
- **Follow-up checks:** Rebuild all fixtures against 1.20.1/Forge, inspect compression IDs, network NBT, negative chunk coordinates, oversized data, unknown-field preservation and palette boundaries.
- **RustChunk implementation mapping:** Prioritize a Rust NBT/region/palette compatibility layer with JVM-produced golden fixtures; avoid using 26.3 block-state IDs.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3608. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldRayOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** ff7bddd9027492351b9713464eede767fa8d0229
- **Size:** 8144 bytes
- **Coverage:** Complete EOF coverage (78 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldRayOracle.java
- **Dependencies / cross-references:** BlockGetter.clip/ClipContext, BlockState, FluidState, WaterFluid, GameRules, tags/registries, ServerLevel, Entity and VoxelShape; Unsafe and Proxy.
- **Implementation / logic:** Builds custom block/fluid states with observable query hooks, supplies server or proxy world, selects entity collision context and records ray hit, direction, block position, inside flag and query order.
- **Findings:** Custom BlockState/Fluid subclasses and registry tag mutation are invasive; proxy world supports a constrained method subset; failure injection catches only its sentinel. No Forge-modified collision shapes are represented.
- **RustChunk relevance:** Useful for exact raycast ordering, shape mode and fluid handling tests.
- **Decision:** adaptable; reference; version-specific; parity-test-oracle
- **1.20.1 implications:** Block/fluid tags, collision context and ClipContext behavior must be rechecked against 1.20.1 Forge.
- **Follow-up checks:** Compare block modes, fluid modes, entity collision, portal/reset tags, empty/full/partial shapes, fluids and query order against 1.20.1.
- **RustChunk implementation mapping:** Use as target-version raycast oracle fixtures; preserve Forge event hooks and modded shape providers in JVM integration.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3609. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldSpawnOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 65199b489cf381032350671268302e3b6e6f85c4
- **Size:** 3458 bytes
- **Coverage:** Complete EOF coverage (61 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldSpawnOracle.java
- **Dependencies / cross-references:** Server data-pack resources, RegistryLayer, RegistryDataLoader, TagLoader, NoiseGeneratorSettings, biome preset lists, NoiseSpawnFinder and RandomState.
- **Implementation / logic:** Bootstraps vanilla server resources and world registries, builds overworld biome source/generator, then emits spawn X/Z and origin chunk per seed.
- **Findings:** Loads full server resources once but does not close MultiPackResourceManager; assumes vanilla pack and overworld preset; argument parsing and seed list are minimally validated.
- **RustChunk relevance:** Useful deterministic seed-to-spawn/world-origin oracle.
- **Decision:** adaptable; reference; version-specific; worldgen-test-oracle
- **1.20.1 implications:** Noise settings, biome presets and spawn algorithm are version-specific; 1.20.1's worldgen differs substantially from 26.3.
- **Follow-up checks:** Compare 1.20.1 NoiseSpawnFinder or equivalent spawn search, preset data, seed determinism, negative/large seeds and origin chunk.
- **RustChunk implementation mapping:** Add worldgen golden seed fixtures only after pinning 1.20.1 registries and data packs.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3610. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldgenOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** e42ba7edd3e2ba76d4d213536c74720c6905265e
- **Size:** 38042 bytes
- **Coverage:** Complete EOF coverage (387 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldgenOracle.java
- **Dependencies / cross-references:** NormalNoise, RandomState, DensityFunction compiler/sampler/cache/volume, registry ops and mapped registries, Climate, MultiNoiseBiomeSource, Aquifer, NoiseSettings, TagLoader, Gson/DFU, reflection.
- **Implementation / logic:** Builds temporary registries from JSON bundles, compiles/evaluates scalar and volume density samplers, probes climate/biome point and bulk lookup, tests index branch factors/cache state, settings clamps, aquifer state/cache traces and tag merging.
- **Findings:** Very broad and reflection-heavy oracle; later-version registry and density APIs; many fixtures and outputs are line-oriented and depend on exact field ordering. It is a reference harness, not a production generator or proof of Rust parity.
- **RustChunk relevance:** One of the strongest references for decomposing worldgen compatibility into testable subsystems.
- **Decision:** adaptable; reference; version-specific; worldgen-test-oracle
- **1.20.1 implications:** 1.20.1 noise router, density graph, biome source, aquifer and random APIs/data differ; all expected values must be regenerated.
- **Follow-up checks:** Split into smaller 1.20.1-specific oracles for noise/RNG, density compilation, climate, biome search, aquifer and tag reload; verify cache and boundary cases independently.
- **RustChunk implementation mapping:** Use only as a test taxonomy; implement worldgen through target-version Java/Forge bridge first, then native Rust parity modules.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3611. `mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldgenRandomOracle.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** d074834d021b73293ee78663a1619f6914645481
- **Size:** 4120 bytes
- **Coverage:** Complete EOF coverage (50 logical lines); full source SHA/size verified.
- **Purpose:** mods/horizon/src/test/java/dev/horizonlab/parity/VanillaWorldgenRandomOracle.java
- **Dependencies / cross-references:** RandomSource, LegacyRandomSource, XoroshiroRandomSource, WorldgenRandom and Gson.
- **Implementation / logic:** Parses JSON cases, warms source, wraps layers, applies root/seed/consume/fork/positional/slime/decoration/feature/large/salt and sampling operations, records outputs and root counter.
- **Findings:** The `random` variable can diverge from `root` after forks while the reported count remains root count; likely intentional but should be explicit. No input bounds or malformed-operation validation.
- **RustChunk relevance:** Directly relevant to deterministic chunk/worldgen RNG compatibility.
- **Decision:** adaptable; reference; version-specific; parity-test-oracle
- **1.20.1 implications:** RNG algorithms and seeding contracts vary across game versions; 1.20.1 uses its own legacy/modern source set and must be compared independently.
- **Follow-up checks:** Compare every method and counter against 1.20.1 WorldgenRandom/RandomSource, especially wrapper nesting, positional hash, fork and negative coordinates.
- **RustChunk implementation mapping:** Implement RNG adapters in Rust only after target-version golden vectors; keep Java bridge for calls tied to Forge's worldgen classes.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3612. `native-hooks.json`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 12163a3bb5551046fe622f959b382e61556fbecc
- **Size:** 123749 bytes
- **Coverage:** Complete EOF coverage (4302 logical lines); full source SHA/size verified.
- **Purpose:** native-hooks.json
- **Dependencies / cross-references:** Rust horizon_client native modules, hook injection build pipeline, native-mod API schema, NeoForge event naming and native-hooks consumers.
- **Implementation / logic:** JSON has 113 helpers and 386 hooks; parsed all objects and verified unique helper/hook names and targets, required name/target/mod_api.contract presence, contract distribution and optional side/cancellable metadata.
- **Findings:** No duplicate names/targets and no missing name/target/contract; however 126/386 hook entries omit `side`, and 287 omit `cancellable` (not necessarily invalid if optional defaults exist). Event coverage explicitly records many events with no NeoForge equivalent, so parity cannot be inferred from hook count.
- **RustChunk relevance:** High-value inventory for mapping native hook surface and identifying missing Forge event equivalents.
- **Decision:** adaptable; reference; version-specific; native-hook-contract-map
- **1.20.1 implications:** Hook names, native targets and event correspondences are tied to Horizon 26.3 and cannot be treated as Forge 1.20.1 API coverage.
- **Follow-up checks:** Validate JSON schema/default semantics, inspect every hook target against implementation, classify client/server/both, compare 1.20.1 Forge events and mark missing/cancel/replace/collect semantics.
- **RustChunk implementation mapping:** Build a RustChunk hook-coverage matrix from 1.20.1 Forge events; use this manifest only as a checklist, not as an implementation contract.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3613. `native-mods/prebuilt.txt`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 6346d2a528ed76504aea2d5c7bac673ca1b13c7d
- **Size:** 194 bytes
- **Coverage:** Complete EOF coverage (3 logical lines); full source SHA/size verified.
- **Purpose:** native-mods/prebuilt.txt
- **Dependencies / cross-references:** native-mods.yml build configuration and native-mods directory IDs.
- **Implementation / logic:** Commented format description followed by the single prebuilt mod identifier `wrench`.
- **Findings:** Only one prebuilt native mod is listed; this does not enumerate ordinary Java/Forge mods and does not imply Forge compatibility.
- **RustChunk relevance:** Useful packaging/configuration pattern for selecting optional native modules.
- **Decision:** reference; version-specific; packaging-manifest
- **1.20.1 implications:** The prebuilt native-mod mechanism is specific to Horizon/Starling and its native mod ABI.
- **Follow-up checks:** Trace native-mods.yml selection and build output; compare with RustChunk's JVM Forge mod loading and optional native extension policy.
- **RustChunk implementation mapping:** If RustChunk later supports native extensions, maintain a separate explicit allowlist; never mix it with Forge JAR metadata.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3614. `native-mods/wrench/horizon-native.json`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 3b74f67e640d04873322903a1f57def99e273de9
- **Size:** 132 bytes
- **Coverage:** Complete EOF coverage (7 logical lines); full source SHA/size verified.
- **Purpose:** native-mods/wrench/horizon-native.json
- **Dependencies / cross-references:** Wrench native mod build, Horizon native mod loader and mixin source bundle.
- **Implementation / logic:** Small JSON descriptor with game `26.3`, id `wrench`, schema 1, version 0.1.0 and pinned mixin digest.
- **Findings:** Explicit target game version is 26.3; this is not a Java Forge mod descriptor and cannot be loaded as a 1.20.1 Forge mod.
- **RustChunk relevance:** Useful only as an example of explicit native module versioning and source digest pinning.
- **Decision:** reference; version-specific; incompatible-with-1.20.1
- **1.20.1 implications:** Target game and native ABI are incompatible with RustChunk's 1.20.1/Forge 47.x scope.
- **Follow-up checks:** Keep the descriptor out of Forge mod resolution; inspect native mod schema and hash validation only if a separate RustChunk native ABI is designed.
- **RustChunk implementation mapping:** Do not load/copy this descriptor into the Forge mods folder; translate the provenance idea into a separate RustChunk extension manifest.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3615. `native-mods/wrench/mixins/wrench/horizon-mixin.toml`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 9f08c84af68bab904a08e03442e1638c48e897b0
- **Size:** 2264 bytes
- **Coverage:** Complete EOF coverage (70 logical lines); full source SHA/size verified.
- **Purpose:** native-mods/wrench/mixins/wrench/horizon-mixin.toml
- **Dependencies / cross-references:** Horizon native mod schema, Rust module files runtime.rs/native.rs/use.rs/list.rs/settings.rs/click.rs, Game::use_item and native_hooks functions.
- **Implementation / logic:** Declares priority 210 HEAD cancellable injection for use_item and three mod UI hook injections with explicit Rust bindings; comments explain priority interaction and unpinned hook-point stability.
- **Findings:** `use_item` is explicitly not a published hook point; injection is tied to private Horizon implementation and source commit and may fail after renames. Not a Java Mixin config despite the filename.
- **RustChunk relevance:** Useful example of native extension injection and explicit binding declarations, with high maintenance risk.
- **Decision:** reference; rewrite-required; version-specific; native-mod-injection
- **1.20.1 implications:** Neither target functions nor hook framework match Forge 1.20.1 Mixin/ModLauncher semantics.
- **Follow-up checks:** Inspect Wrench helper sources and injector validation; compare with Forge event/Mixin lifecycle only as a separate architecture exercise.
- **RustChunk implementation mapping:** Do not port these injections directly; use Forge events/Mixins for 1.20.1 compatibility and isolate any Rust native extension behind a versioned ABI.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3616. `packaging/PLAY.txt`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** a175c5d51b935035f2c1af4cc339d487631a8b9a
- **Size:** 2881 bytes
- **Coverage:** Complete EOF coverage (51 logical lines); full source SHA/size verified.
- **Purpose:** packaging/PLAY.txt
- **Dependencies / cross-references:** horizon-prepare, horizon-client, horizon-server, horizon-ctl, launcher download/update flow and Minecraft resources.
- **Implementation / logic:** Explains that the player supplies Minecraft Java Edition 26.3, uses Java 25, prepares assets/server jar, launches the client and operates an embedded server.
- **Findings:** The explicit Minecraft 26.3 and Java 25 prerequisites conflict with RustChunk's Minecraft 1.20.1 / Java 17 target; it describes a separate Rust game engine, not a Forge launcher.
- **RustChunk relevance:** Useful high-level documentation structure and user-facing preparation flow only.
- **Decision:** reference; rewrite-required; version-specific; packaging-documentation
- **1.20.1 implications:** Version and runtime instructions cannot be reused for 1.20.1 Forge; Java 17 and Forge 47.x must be specified instead.
- **Follow-up checks:** Keep a separate 1.20.1 setup guide covering Java 17, Forge 47.x, libraries, assets/legal boundaries and supported Android ABI.
- **RustChunk implementation mapping:** Use the section structure for RustChunk docs, rewriting every command, runtime, version and server statement.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3617. `packaging/android/AndroidManifest.xml`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** ba03ae9c028e9c609ac30d3875eae581fb1439d7
- **Size:** 9101 bytes
- **Coverage:** Complete EOF coverage (148 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/AndroidManifest.xml
- **Dependencies / cross-references:** LauncherActivity, GameActivity/StarlingActivity, DiagnosticsActivity, WorkService, GameFolderProvider, UpdateProvider, horizon_android JNI library and resources.
- **Implementation / logic:** Template manifest with substituted application ID/label; declares INTERNET, storage, install, foreground-service, notification, vibration and conditional audio permissions; sets isGame=true/appCategory=game; requires Vulkan feature; routes launcher/game/diagnostics and providers.
- **Findings:** Broad MANAGE_EXTERNAL_STORAGE and install-package permissions need least-privilege review; mandatory Vulkan feature can exclude devices without the declared feature; no target/min SDK values here because build tooling injects them.
- **RustChunk relevance:** Strong Android integration reference, including explicit game classification metadata relevant to Samsung Gaming Hub discovery.
- **Decision:** adaptable; rewrite-required; android-packaging
- **1.20.1 implications:** Must be adapted to RustChunk package/process/library names, API levels and renderer requirements; Vulkan must not be mandatory unless RustChunk truly requires it.
- **Follow-up checks:** Inspect build_apk.py SDK injection, runtime permission UX, scoped-storage alternative, API 33 behavior, Vulkan capability gating and manifest merge.
- **RustChunk implementation mapping:** Use `android:isGame=true` and `android:appCategory=game` as candidates in RustChunk manifest; keep Java bridge and Rust native engine process design separate.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3618. `packaging/android/bedrock-controls.json`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 2f97af93b8a6e96d1c36b1c2cb1a2ecfc7eb83b9
- **Size:** 15723 bytes
- **Coverage:** Complete EOF coverage (424 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/bedrock-controls.json
- **Dependencies / cross-references:** Build asset preparation, upstream GUI control images, Git object provenance and LICENSE.md.
- **Implementation / logic:** JSON revision plus 60 asset records: 59 PNGs and one license file; unique paths, required path/source/size/SHA256 fields present; aggregate declared payload 75,060 bytes; 59 records include git blob IDs.
- **Findings:** This is a manifest, not the image files themselves. The 59 PNGs were identified by metadata only and were not visually opened in this entry; no image may be marked visually reviewed based on this manifest.
- **RustChunk relevance:** Useful for deterministic asset pinning, integrity verification and mobile control asset inventory.
- **Decision:** adaptable; reference; android-controls-asset-manifest
- **1.20.1 implications:** Asset content, license and touch-control semantics must be re-evaluated for RustChunk; Bedrock-derived controls do not prove Minecraft Java control parity.
- **Follow-up checks:** When the actual PNG blobs are encountered, archive each under its own audit number, visually inspect it, verify Git/SHA-256/size and reconcile with this manifest.
- **RustChunk implementation mapping:** Adopt the manifest integrity pattern for RustChunk-owned assets, with independent touch layout and legal review.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3619. `packaging/android/build-server.txt`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 8f2b19fbd8903144696a7f00b0bdd9611ddcffad
- **Size:** 53 bytes
- **Coverage:** Complete EOF coverage (1 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/build-server.txt
- **Dependencies / cross-references:** Native build orchestration and the remote Cloudflare Worker service.
- **Implementation / logic:** Single URL pointing to `starling-compile.starling-client.workers.dev`.
- **Findings:** A single endpoint provides no local fallback, authentication, integrity or provenance details in this file; relying on it would make builds depend on an external service.
- **RustChunk relevance:** Useful only to identify a remote build dependency that must be isolated.
- **Decision:** reference; external-build-service; security-review-required
- **1.20.1 implications:** RustChunk should not depend on this endpoint or transmit source/build inputs to it.
- **Follow-up checks:** Inspect all callers, authentication, request payloads, returned artifact verification and service availability policy.
- **RustChunk implementation mapping:** Prefer reproducible local/GitHub Actions builds for RustChunk; pin and verify any external build artifact if ever used.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3620. `packaging/android/java/io/github/juanluci/starling/DiagnosticsActivity.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 2bdbe8637d0ab23354cc8251d723fd7935f2649d
- **Size:** 11197 bytes
- **Coverage:** Complete EOF coverage (222 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/java/io/github/juanluci/starling/DiagnosticsActivity.java
- **Dependencies / cross-references:** Android Activity/Dialog/Views, StartupRecovery, StarlingActivity.sharedFolder, native JNI makeReport, diagnostics resources/fonts and UpdateProvider content URI.
- **Implementation / logic:** Builds UI programmatically, lazily loads horizon_android on a background thread for report/export, displays logs in a selectable dialog, tracks latest ZIP and shares via content URI.
- **Findings:** Separate process and lazy native loading improve recovery resilience; latest report selection is lexicographic over numeric filenames and may misorder different digit lengths; report/export thread has no explicit lifecycle cancellation.
- **RustChunk relevance:** Useful recovery and diagnostics UX pattern for a native Android game.
- **Decision:** adaptable; reference; android-diagnostics
- **1.20.1 implications:** JNI library names, report schema, resources and process ownership must be replaced for RustChunk; no guarantee the game engine is safe to initialize in diagnostics.
- **Follow-up checks:** Test recovery after native load failure, report export/share permissions, large ZIP handling, activity destruction and numeric report ordering.
- **RustChunk implementation mapping:** Build a RustChunk diagnostics activity that does not initialize the renderer/game engine and exports bounded logs/crash metadata.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3621. `packaging/android/java/io/github/juanluci/starling/GameActivity.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 75a2b5ef05b1605de430fc04c8338854897c2d5d
- **Size:** 185 bytes
- **Coverage:** Complete EOF coverage (4 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/java/io/github/juanluci/starling/GameActivity.java
- **Dependencies / cross-references:** StarlingActivity and Android activity manifest registration.
- **Implementation / logic:** Declares final GameActivity extending StarlingActivity with no additional behavior.
- **Findings:** No independent lifecycle, renderer or JNI implementation exists in this file; all behavior is inherited.
- **RustChunk relevance:** Useful only as a naming/manifest separation pattern.
- **Decision:** reference; android-activity-shell
- **1.20.1 implications:** No Minecraft or Forge compatibility implications by itself.
- **Follow-up checks:** Inspect StarlingActivity and its JNI lifecycle before deciding whether a distinct game activity is needed.
- **RustChunk implementation mapping:** Keep RustChunk's Android Activity thin and delegate engine startup to a controlled Rust/JNI bridge.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3622. `packaging/android/java/io/github/juanluci/starling/GameFolder.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 71ffb3308f0f7a7dc376258b920cd73c9d2b2e98
- **Size:** 10511 bytes
- **Coverage:** Complete EOF coverage (276 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/java/io/github/juanluci/starling/GameFolder.java
- **Dependencies / cross-references:** Android DocumentsProvider caller, filesystem canonical paths, Files/NIO, PLAYER_FILES/LOGS allowlists and StarlingActivity shared-folder resolver.
- **Implementation / logic:** Validates path components and UTF-8 byte lengths, refuses symbolic links, restricts private root contents, maps canonical paths to IDs, creates collision-suffixed names and recursively deletes documents.
- **Findings:** Explicit path traversal and symlink defenses are valuable; filesystem checks are subject to TOCTOU races between validation and use, and canonicalization policy should be reviewed for shared-storage aliases.
- **RustChunk relevance:** Highly relevant for secure worlds/mods/screenshots/logs storage on Android.
- **Decision:** adaptable; security-sensitive; android-storage
- **1.20.1 implications:** Android scoped-storage and SAF behavior differ by API level; private/shared directory rules must match RustChunk's own storage model.
- **Follow-up checks:** Fuzz `..`, separators, NUL, overlong UTF-8, symlink swaps, concurrent rename/delete/create, and shared-folder relocation.
- **RustChunk implementation mapping:** Implement a Rust storage abstraction with SAF/DocumentFile bridge and canonical-root enforcement; keep user files distinct from caches/runtime libraries.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3623. `packaging/android/java/io/github/juanluci/starling/GameFolderProvider.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 0716f977a7a8434748abca7de88f38c2dcb6ed73
- **Size:** 9420 bytes
- **Coverage:** Complete EOF coverage (237 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/java/io/github/juanluci/starling/GameFolderProvider.java
- **Dependencies / cross-references:** GameFolder, DocumentsContract, MatrixCursor, ParcelFileDescriptor, MIME type map, manifest authority and MANAGE_DOCUMENTS permission.
- **Implementation / logic:** Implements roots/document/children queries, MIME inference, read/write opens with close notifications, create, rename and delete operations.
- **Findings:** Queries honor caller-supplied projections when constructing MatrixCursor but addRow/queryRoots unconditionally add every standard column; subset projections may trigger unknown-column errors. Write open checks existing file and relies on provider flags; error mapping is coarse.
- **RustChunk relevance:** Useful SAF integration pattern, but provider correctness and security need Android-specific tests.
- **Decision:** adaptable; security-review-required; android-storage-provider
- **1.20.1 implications:** Authority, root ID, permissions and MIME policy must be rewritten for RustChunk's storage paths and Android API targets.
- **Follow-up checks:** Test all projection subsets, query sorting, cancellation, read/write modes, concurrent changes, URI grants, rename/delete and symlink edge cases.
- **RustChunk implementation mapping:** Use a narrowly scoped RustChunk DocumentsProvider exposing only player-owned files; avoid exporting private auth/cache directories.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3624. `packaging/android/java/io/github/juanluci/starling/HorizonImport.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 4d42e4d2b81ee29c680a064c6cff5b8332ca733a
- **Size:** 11402 bytes
- **Coverage:** Complete EOF coverage (271 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/java/io/github/juanluci/starling/HorizonImport.java
- **Dependencies / cross-references:** Horizon Android package/authority, ACTION_OPEN_DOCUMENT_TREE, WorkService foreground hold, ContentResolver, ZipOutputStream, game import.rs inbox semantics.
- **Implementation / logic:** Picks Horizon's document tree, walks selected worlds/packs, streams files or zips directories at BEST_SPEED, excludes session.lock and atomically renames `.part` files into the inbox.
- **Findings:** Top-level names are sanitized, but nested document names are concatenated into ZIP entry paths without equivalent component validation; archive extraction safety is delegated to import.rs and not proven here. No explicit size/count limit or cancellation path.
- **RustChunk relevance:** Useful cross-launcher world/pack migration and foreground transfer pattern.
- **Decision:** adaptable; security-review-required; android-import
- **1.20.1 implications:** Horizon Android's package, provider schema and import format are product-specific; world formats still require target-version validation.
- **Follow-up checks:** Audit import.rs extraction against ZIP-slip, duplicate paths, symlinks, oversized archives, compression bombs and partial writes; validate nested names and cancellation.
- **RustChunk implementation mapping:** Implement SAF-based world/pack import with bounded streaming, safe ZIP path normalization, integrity checks and user-visible progress.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.

### 3625. `packaging/android/java/io/github/juanluci/starling/LauncherActivity.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 53b85373dc9a6c0d4b633a210cfd9cce99766537
- **Size:** 8434 bytes
- **Coverage:** Complete EOF coverage (205 logical lines); full source SHA/size verified.
- **Purpose:** packaging/android/java/io/github/juanluci/starling/LauncherActivity.java
- **Dependencies / cross-references:** GameActivity, DiagnosticsActivity, StartupRecovery, ContentResolver, OpenableColumns, `/proc/<pid>`, import.rs inbox consumer and package process model.
- **Implementation / logic:** Handles ACTION_VIEW/ACTION_SEND, copies URI content to `.part` on a worker thread, renames when complete, shows restart screen and waits briefly for old PID to exit before relaunch.
- **Findings:** Receive writes to a predictable `<name>.part` without exclusive creation, so concurrent same-name imports can collide; rename may replace a prior destination depending filesystem semantics. Completion callback runs even after copy failure and launches game anyway.
- **RustChunk relevance:** Useful separation of import routing and native game process restart.
- **Decision:** adaptable; security-review-required; android-launch-flow
- **1.20.1 implications:** Process names, activity lifecycle, import protocol and Android URI grants must be redesigned for RustChunk.
- **Follow-up checks:** Test concurrent same-name imports, overwrite behavior, provider permission revocation, large/partial files, stale `.part` cleanup and PID reuse/timeout.
- **RustChunk implementation mapping:** Use a dedicated import broker with unique temp files, atomic no-replace commit, content hash/size checks and explicit failure UX.
- **Evidence required before adoption:** undefined
- **Completion state:** COMPLETE — pinned source SHA and byte size verified; complete stated coverage read through EOF (files over 400 lines inspected in consecutive chunks no larger than 400 lines); static source review only; not executed.
