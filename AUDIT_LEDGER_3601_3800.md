# RustChunk Starling Audit Ledger — entries 3601–3800

- Repository: `starling-client/starling`
- Pinned ref: `5432370d573642041585c69b40fb045b27133488`
- Target assessment: RustChunk Minecraft Java 1.20.1 / Forge 47.x / Android
- Authorized scope: exceptional 100-file grant, entries 3601–3700, dated 2026-11-27.
- Selection: first 100 raw-lexically sorted unindexed blobs from the complete pinned recursive tree, excluding all 3,600 existing Index paths.
- Checkpoint cadence: four sequential 25-file checkpoints. Commit all checkpoint records and audit documents in one Git commit, then re-fetch/read-back verify before the next checkpoint.
- Current registered state: **100 source entries (3601–3700)**; this file will grow only under the same 3601–3700 grant. Entries 3626–3700 remain authorized but not yet inspected.

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

### 3626. `packaging/android/java/io/github/juanluci/starling/StarlingActivity.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 2104c5ffbae19e76b75d6810e2f04f7cdafbdcfe
- **Size:** 42794 bytes
- **Coverage:** Complete EOF coverage (1003 logical lines).
- **Purpose:** Android NativeActivity bridge for library loading, storage, window/input, IME, controllers, gyroscope, clipboard, file picker, browser, DNS, updates and Keystore secrets.
- **Dependencies / cross-references:** Android NativeActivity/system services; NativeMods, StartupRecovery, TextMirror, HorizonImport, GameFolderProvider, WorkService, UpdateProvider and horizon_android JNI.
- **Implementation / logic:** Loads selected native build or APK library; configures storage before NativeActivity starts; forwards IME/pointer/controller/sensor events; exposes import, folder, DNS, refresh-rate, update and AES-GCM Keystore methods.
- **Findings:** Tightly coupled to Starling JNI/process contracts. onDestroy kills process directly; openFolder uses string-prefix path checks; openLink accepts any HTTPS host; secret envelope lacks version/AAD; arbitrary native library loading depends on NativeMods verification outside this file. MANAGE_EXTERNAL_STORAGE is high privilege.
- **RustChunk relevance:** Core Android/JNI bridge reference for a Rust native game.
- **Decision:** adaptable; reference; version-specific
- **1.20.1 implications:** Java/runtime/library ABI and all class names need a RustChunk-specific rewrite; no 1.20.1 Forge compatibility is supplied.
- **Follow-up checks:** Audit NativeMods and android_lib.rs; fuzz folder paths; test Keystore invalidation, URI grants, process restart and API 33 lifecycle.
- **RustChunk implementation mapping:** Create a narrow Java bridge with SAF storage, explicit JNI lifecycle and a versioned encrypted-account envelope.
- **Evidence required before adoption:** Require API 33 storage/URI tests, JNI lifecycle/restart tests and AndroidKeyStore round-trip/invalidation tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3627. `packaging/android/java/io/github/juanluci/starling/StartupRecovery.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 1c35bed686462463dcf64cb8eb84eba46f3571c0
- **Size:** 2474 bytes
- **Coverage:** Complete EOF coverage (49 logical lines).
- **Purpose:** Persists startup attempts and offers recovery after interruption during native startup, renderer initialization or resource loading.
- **Dependencies / cross-references:** Android Context/filesDir, AtomicFile, JSONObject and launcher/diagnostics lifecycle.
- **Implementation / logic:** Creates a marker before native loading, removes it after first frame/recovery offer, updates current-session.json atomically and inspects phase/clean-exit flags.
- **Findings:** Marker write uses plain FileOutputStream; interrupted() performs one InputStream.read and assumes the full JSON arrives in one read; marker errors are mostly ignored.
- **RustChunk relevance:** Valuable crash recovery pattern when native startup can fail before UI initialization.
- **Decision:** adaptable; reference; android-recovery
- **1.20.1 implications:** Phase names and file locations are Starling-specific.
- **Follow-up checks:** Use readFully loop and atomic marker; test truncated JSON and process death at each phase.
- **RustChunk implementation mapping:** Add a versioned recovery journal before Rust/JNI or JVM startup; keep diagnostics independent.
- **Evidence required before adoption:** Require process-kill fault injection at every startup phase and verify one-time recovery.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3628. `packaging/android/java/io/github/juanluci/starling/TextMirror.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 06c46b53382981b64bc0da6174f3a91ae4845f0d
- **Size:** 13224 bytes
- **Coverage:** Complete EOF coverage (336 logical lines).
- **Purpose:** Bridges Android IME to a game-rendered text field using an invisible EditText and code-point-aware selection/length.
- **Dependencies / cross-references:** Android EditText/InputConnection/InputMethodManager/TextWatcher/InputFilter; StarlingActivity and native callbacks.
- **Implementation / logic:** Mirrors text and selection, suppresses echo during game updates, batches edits, reports Unicode code-point cursor/anchor and routes game-specific keys.
- **Findings:** Good UTF-16/code-point handling; OEM IME callback order and generation semantics need tests; only sendKeyEvent is explicitly wrapped.
- **RustChunk relevance:** Highly relevant to chat, search and server-address input on Android.
- **Decision:** adaptable; reference; android-input-bridge
- **1.20.1 implications:** Native callbacks and field flags must be recreated; no Forge-specific logic.
- **Follow-up checks:** Test emoji, combining marks, selection reversal, composing spans, password/multiline, hardware keyboard and OEM IMEs.
- **RustChunk implementation mapping:** Build a standalone Java IME adapter connected to RustChunk text-input protocol.
- **Evidence required before adoption:** Require Unicode and IME instrumentation tests on API 33 and multiple keyboards.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3629. `packaging/android/java/io/github/juanluci/starling/UpdateProvider.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** f564a65403f43acf4c54386008200ccf9101a9aa
- **Size:** 3411 bytes
- **Coverage:** Complete EOF coverage (90 logical lines).
- **Purpose:** Exposes downloaded update APKs and diagnostic ZIPs read-only through ContentProvider.
- **Dependencies / cross-references:** Android ContentProvider/Uri/ParcelFileDescriptor/MatrixCursor; private updates/performance directories and manifest grants.
- **Implementation / logic:** Restricts names by APK/ZIP regex, resolves fixed private directories, opens read-only and returns display name/size.
- **Findings:** Narrow read-only boundary is good; query ignores projection; getType checks path shape before full validation; symlink rejection is not explicit.
- **RustChunk relevance:** Useful for update delivery and controlled diagnostic export.
- **Decision:** adaptable; reference; android-content-provider
- **1.20.1 implications:** Authority, paths and APK policy must be RustChunk-specific.
- **Follow-up checks:** Test malformed/encoded URIs, projection subsets, symlinked files, grant revocation and APK signature verification.
- **RustChunk implementation mapping:** Create a read-only provider with exact allowlists and signature/hash checks.
- **Evidence required before adoption:** Require provider permission tests and APK signature/version validation.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3630. `packaging/android/java/io/github/juanluci/starling/WorkService.java`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 5dcf3bd4c3b849a21e68e39ff3c0673ab665294d
- **Size:** 4380 bytes
- **Coverage:** Complete EOF coverage (115 logical lines).
- **Purpose:** Keeps the game process alive with an ongoing notification during browser sign-in, downloads or imports.
- **Dependencies / cross-references:** Android Service/Notification/ServiceInfo; StarlingActivity and HorizonImport; manifest dataSync type.
- **Implementation / logic:** Tracks named work reasons, starts foreground service while any reason remains, shows newest reason and stops after last release.
- **Findings:** Reason map is process-local; service start failures are swallowed; Android 14+ dataSync limits and notification policy need review.
- **RustChunk relevance:** Useful for long-running work Android may otherwise interrupt.
- **Decision:** adaptable; reference; android-foreground-service
- **1.20.1 implications:** Service type, target SDK and limits must match RustChunk current Android targets.
- **Follow-up checks:** Test overlapping reasons, fast start/stop, process death and API 33/34+ restrictions.
- **RustChunk implementation mapping:** Use persisted WorkManager/foreground-service state with clear progress.
- **Evidence required before adoption:** Require lifecycle/race tests on API 33 and newer Android.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3631. `packaging/android/mod_build_gateway.py`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 5283430a3047d72e50dc6595f449342b0d506c9e
- **Size:** 39128 bytes
- **Coverage:** Complete EOF coverage (1010 logical lines).
- **Purpose:** HTTPS gateway wakes Azure build VM, applies quotas, caches native-mod builds, dispatches GitHub Actions and serves verified results.
- **Dependencies / cross-references:** Python HTTP/threads/urllib/zipfile; Azure managed identity/ARM; Caddy; mod_build_server; GitHub Actions API and artifact store.
- **Implementation / logic:** Tracks jobs, hashes request bodies, enforces quotas, wakes/deallocates VM, dispatches workflows, downloads artifacts and verifies SHA-256/size.
- **Findings:** Artifact verification and quotas are useful; address() trusts X-Forwarded-For so proxy overwrite is security-critical; connections are not consistently explicitly closed; random-token endpoints and GitHub credentials need threat review.
- **RustChunk relevance:** Architecture reference for optional remote compilation and artifact caching; not required for Forge JAR loading.
- **Decision:** reference; version-specific; remote-build-gateway
- **1.20.1 implications:** Compiles Horizon native mods, not Forge mods; does not solve JVM/Forge compatibility.
- **Follow-up checks:** Inspect Caddy/firewall/systemd/token scopes; test XFF spoofing, short bodies, socket lifecycle, artifact ZIP limits and cache eviction.
- **RustChunk implementation mapping:** Prefer reproducible local/GitHub Actions builds; isolate any remote service from worlds/accounts and sign artifacts.
- **Evidence required before adoption:** Require threat model, deployment review, HTTP fuzzing, quota-spoof tests and signed artifact verification.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3632. `packaging/android/mod_build_once.py`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 1c6f116e6d95310edd4f2bf22d32681ad037390d
- **Size:** 2762 bytes
- **Coverage:** Complete EOF coverage (70 logical lines).
- **Purpose:** Runs one native-mod build on an ephemeral GitHub runner using the same validation and sandbox as the persistent service.
- **Dependencies / cross-references:** mod_build_server split_body/unpack/why_failed; mod_build_sandbox; subprocess and GitHub Actions.
- **Implementation / logic:** Reads request body, validates bundles, invokes sandbox with timeout, captures logs, emits status and moves library to artifact folder.
- **Findings:** Fixed jobs/once workspace and no finally cleanup can leave partial files after malformed input or timeout; CLI args and local artifact metadata are minimally checked.
- **RustChunk relevance:** Useful one-request-per-run CI pattern.
- **Decision:** reference; version-specific; build-orchestration
- **1.20.1 implications:** Only builds Starling .hnmod native modules for Horizon 26.3.
- **Follow-up checks:** Test malformed body, timeout cleanup, artifact contents, log truncation and runner reuse assumptions.
- **RustChunk implementation mapping:** Build each optional native extension in a fresh ephemeral runner with provenance metadata.
- **Evidence required before adoption:** Require workflow isolation and artifact integrity tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3633. `packaging/android/mod_build_sandbox.sh`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 2abbd7aa2ad3e7951eb776b2e61330f6b005f565
- **Size:** 3119 bytes
- **Coverage:** Complete EOF coverage (87 logical lines).
- **Purpose:** Runs third-party native build scripts in Linux namespaces with network disabled and reduced filesystem/capabilities.
- **Dependencies / cross-references:** unshare/mount/pid namespaces, sudo/timeout/setpriv, overlayfs, Rust/Android/Cargo caches and native_build.sh.
- **Implementation / logic:** Checks out pinned source, creates private proc/tmp/home, mounts toolchains read-only, gives per-job overlay cache and drops UID/GID/groups/capabilities.
- **Findings:** Strong isolation intent; shared Cargo cache is writable across builds and may be poisoned; trusted-base mode writes directly to base; no seccomp profile visible; cleanup paths need adversarial tests.
- **RustChunk relevance:** Important threat-model reference for any future third-party native build system.
- **Decision:** reference; version-specific; build-sandbox
- **1.20.1 implications:** Linux build isolation only; unrelated to Android Forge runtime loading.
- **Follow-up checks:** Test cache poisoning, symlink/mount attacks, timeout cleanup, environment leakage and network namespace enforcement.
- **RustChunk implementation mapping:** Use disposable VM/container per build, read-only toolchains and isolated writable caches.
- **Evidence required before adoption:** Require malicious build-script tests proving no cross-build or host-secret access.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3634. `packaging/android/mod_build_server.py`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 317c8053a58864fdccee755f5af4138180cf2b3a
- **Size:** 13930 bytes
- **Coverage:** Complete EOF coverage (349 logical lines).
- **Purpose:** Accepts native-mod bundles, validates ZIP contents, queues builds, compiles through sandbox and serves status/library artifacts.
- **Dependencies / cross-references:** Python stdlib, ZIP parser, filesystem, worker queue, versions.json and native build scripts.
- **Implementation / logic:** Enforces size/count/mod limits, validates archive paths and IDs, extracts allowed Rust/TOML/data/text, deduplicates by version/commit/mod hashes and runs one worker.
- **Findings:** Duplicate ZIP names overwrite in map; failed unpack may leave partial work; identical concurrent submissions can race over same work directory; binds 0.0.0.0 although comments assume gateway-only; Content-Length read is not robustly validated.
- **RustChunk relevance:** Reference for safe archive intake and async job queues.
- **Decision:** reference; version-specific; mod-build-service
- **1.20.1 implications:** Starling .hnmod service is not a Forge mod loader.
- **Follow-up checks:** Test concurrent identical jobs, cleanup, duplicate ZIP entries, malformed/short bodies, direct-port firewall and ranges.
- **RustChunk implementation mapping:** Use JVM/Forge to load Forge JARs; borrow only validated queue/archive patterns.
- **Evidence required before adoption:** Require fuzzing, concurrency stress, firewall review and adversarial ZIP tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3635. `packaging/android/native/java_launcher.c`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 746233c33abd3af4a2f1bcf26d1eca1c1cad4dc3
- **Size:** 2953 bytes
- **Coverage:** Complete EOF coverage (71 logical lines).
- **Purpose:** Provides Android java/javac shims that load runtime libjli and call JLI_Launch.
- **Dependencies / cross-references:** Android linker/dlopen/dlsym, libc++_shared, STARLING_JAVA_HOME and JDK libjli.
- **Implementation / logic:** Finds own APK library folder, loads libc++_shared and libjli, resolves JLI_Launch and invokes Java or javac entry args.
- **Findings:** Hard-codes Java version 25; JLI_Launch is internal ABI and signature may vary across JDK builds.
- **RustChunk relevance:** Useful reference for packaging Java runtime tools on Android.
- **Decision:** reference; version-specific; android-java-launcher
- **1.20.1 implications:** Java 25 is incompatible with RustChunk Java 17 / Forge 1.20.1; requires JDK 17 bridge.
- **Follow-up checks:** Inspect runtime distribution, JLI symbol/signature, ABI and launch flags.
- **RustChunk implementation mapping:** Use a tested Java 17 runtime/launcher bridge; do not copy version 25 values.
- **Evidence required before adoption:** Require java/javac 17 version checks and Forge bootstrap on Android ARM64.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3636. `packaging/android/native_build.sh`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 32d68763dc863fbf22ffa9ff787b4a191bbfc57a
- **Size:** 7312 bytes
- **Coverage:** Complete EOF coverage (153 logical lines).
- **Purpose:** Builds Starling with selected native mods by applying pinned horizon-mixins, adapting bundles and invoking Android APK build.
- **Dependencies / cross-references:** Cargo, pinned horizon-mixins, git archive, rsync, sed, Python build_apk and native_mod_permissions, Android SDK/NDK.
- **Implementation / logic:** Validates ABI/variant, installs pinned CLI, exports source, creates plan stubs, retargets bundles, adapts code, hashes inputs, applies mixins and builds library/APK plus build.json.
- **Findings:** Build key omits explicit Rust/NDK/SDK toolchain versions; grep/sed anchors and regex adaptation are brittle; cached target artifacts are retained intentionally.
- **RustChunk relevance:** Reference for native extension build provenance and pipeline.
- **Decision:** reference; version-specific; native-build-pipeline
- **1.20.1 implications:** Horizon 26.3 native injections, not Forge 47.x or Java Mixin.
- **Follow-up checks:** Pin toolchain versions in key; verify staged diff, bundle hashes, source revision and clean/reused cache equivalence.
- **RustChunk implementation mapping:** Build RustChunk native engine and Java17/Forge in separate reproducible jobs.
- **Evidence required before adoption:** Require clean-room reproducibility and complete build-key coverage.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3637. `packaging/android/native_mod_adapt.py`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 935678e132f0bf8d2fefec05e98bb6468f447a4b
- **Size:** 7648 bytes
- **Coverage:** Complete EOF coverage (167 logical lines).
- **Purpose:** Rewrites older Horizon native mod manifests, module paths and Rust call sites to fit changed crate layout/API.
- **Dependencies / cross-references:** Python pathlib/re, source crate roots, horizon-mixin manifests and Rust source files.
- **Implementation / logic:** Discovers top-level modules, rewrites target/file paths, applies argument additions and regex replacements, then edits manifests and Rust files in place.
- **Findings:** Not a Rust parser: delimiter counting ignores strings/comments; regexes can rewrite unintended code or comments and silently change semantics.
- **RustChunk relevance:** Migration automation reference only; high-risk source transformation.
- **Decision:** reference; version-specific; source-rewriter
- **1.20.1 implications:** Paths and APIs are Horizon-specific and cannot map Forge Java mods.
- **Follow-up checks:** Add fixture before/after tests, parser-aware edits, diff review and idempotence checks.
- **RustChunk implementation mapping:** Prefer explicit migration patches; never auto-rewrite third-party Forge JAR contents.
- **Evidence required before adoption:** Require deterministic golden transforms and human-reviewed diffs.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3638. `packaging/android/release-notes.md`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** d21f12e85389d0f7f19bcd912acd2525d7cd3c98
- **Size:** 10455 bytes
- **Coverage:** Complete EOF coverage (113 logical lines).
- **Purpose:** Documents Starling 0.8.0 and earlier releases, APK ABIs, controls, account profiles, offline profiles, shaders, native/content mods and diagnostics.
- **Dependencies / cross-references:** Starling/Horizon release process, .hnmod/.hmod, catalog, account providers and Android APK variants.
- **Implementation / logic:** User-facing changelog grouped by install, new features, mods/shaders, fixes and historical release sections; contains external links and product claims.
- **Findings:** Release notes are self-reported claims, not runtime evidence; describe Horizon 26.3 and native mods, not 1.20.1 Forge or Aternos compatibility.
- **RustChunk relevance:** Useful changelog structure and user communication reference.
- **Decision:** reference; version-specific; release-documentation
- **1.20.1 implications:** Feature names, versions, auth and mod formats are not transferable without separate implementation.
- **Follow-up checks:** Trace relevant claims to source/tests; verify offline identity and account behavior independently.
- **RustChunk implementation mapping:** Use transparent changelog separating implemented, tested and planned features.
- **Evidence required before adoption:** Require code, tests and release artifact evidence for every RustChunk claim.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3639. `packaging/android/release_assets.py`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 49de32baa084f5535a1d884db0616f90e38eb816
- **Size:** 944 bytes
- **Coverage:** Complete EOF coverage (25 logical lines).
- **Purpose:** Checks all target ABI APKs exist before creating a shared visuals ZIP.
- **Dependencies / cross-references:** build_apk ROOT/TARGETS/zip_folder and packaging/shaderpacks/starling-visuals.
- **Implementation / logic:** Normalizes version prefix, checks expected APK filenames for all ABIs and zips shared visuals.
- **Findings:** Presence-only gate; does not verify signatures, package/version metadata, ABI contents, file hashes or ZIP reproducibility.
- **RustChunk relevance:** Useful multi-ABI release completeness gate.
- **Decision:** adaptable; reference; release-packaging
- **1.20.1 implications:** Targets and filenames must be RustChunk-specific.
- **Follow-up checks:** Add signature, manifest, ABI, checksum and deterministic ZIP verification.
- **RustChunk implementation mapping:** Use after CI has produced every supported RustChunk ABI artifact.
- **Evidence required before adoption:** Require per-ABI install smoke tests and signing verification.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3640. `packaging/android/res/values-v31/themes.xml`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** e009fd3b8005a5f9192bb62bef3c8b0b7de510c6
- **Size:** 503 bytes
- **Coverage:** Complete EOF coverage (9 logical lines).
- **Purpose:** Defines Android 12+ fullscreen game theme and splash background/icon.
- **Dependencies / cross-references:** values-v31 resources, Starling.Game theme, mipmap icon and first-frame gating.
- **Implementation / logic:** Overrides fullscreen theme with dark splash background and app icon for API 31+.
- **Findings:** Visual startup polish only; does not guarantee first frame or prevent native startup failure.
- **RustChunk relevance:** Useful Android 12 splash integration reference.
- **Decision:** adaptable; reference; android-splash-theme
- **1.20.1 implications:** Colors/resources and target API must match RustChunk.
- **Follow-up checks:** Test cold start, splash transition, first-frame delay and API 31 theme merge.
- **RustChunk implementation mapping:** Add values-v31 splash theme; keep recovery and first-frame synchronization separate.
- **Evidence required before adoption:** Require cold-start screenshots and API 31+ launch tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3641. `packaging/android/res/values/diagnostics.xml`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 7fac6de8046763279def8fffc767bbd8241c07d5
- **Size:** 963 bytes
- **Coverage:** Complete EOF coverage (14 logical lines).
- **Purpose:** Provides resource strings for diagnostics, recovery, logs, ZIP export/share, restart and failure states.
- **Dependencies / cross-references:** DiagnosticsActivity, StartupRecovery and Android resource IDs.
- **Implementation / logic:** Defines 12 strings for normal/recovery intros, actions, progress, success, failure and empty-report states.
- **Findings:** Includes reminder to review report before sharing; no localization or placeholder validation here.
- **RustChunk relevance:** Useful diagnostics copy structure.
- **Decision:** adaptable; reference; android-diagnostics
- **1.20.1 implications:** Resource IDs and wording must be rewritten/localized.
- **Follow-up checks:** Check IDs, format placeholders, accessibility length and translations.
- **RustChunk implementation mapping:** Keep diagnostic copy in Android resources rather than hard-coded Java.
- **Evidence required before adoption:** Require resource linking and UI accessibility checks.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3642. `packaging/android/res/values/folders.xml`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** df72f7b6e950ad21913fe886144aac69b33663ee
- **Size:** 163 bytes
- **Coverage:** Complete EOF coverage (3 logical lines).
- **Purpose:** Defines fallback message when no file manager can open a game folder.
- **Dependencies / cross-references:** StarlingActivity.openFolder and open_folder_failed resource.
- **Implementation / logic:** One string directs user to Android Files and the Starling entry.
- **Findings:** Assumes Files app exists and the game folder is visible there.
- **RustChunk relevance:** Small but useful storage failure UX pattern.
- **Decision:** adaptable; reference; android-storage-copy
- **1.20.1 implications:** Folder name and Android Files location must be RustChunk-specific.
- **Follow-up checks:** Test devices without DocumentsUI and denied shared-storage access.
- **RustChunk implementation mapping:** Provide a clear SAF fallback for RustChunk worlds/mods folder.
- **Evidence required before adoption:** Require permission-denied and no-file-manager UI tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3643. `packaging/android/res/values/themes.xml`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** e5ed0b8c627d200b19b471324299da438ce98917
- **Size:** 236 bytes
- **Coverage:** Complete EOF coverage (5 logical lines).
- **Purpose:** Defines the base fullscreen black game activity theme.
- **Dependencies / cross-references:** Android resources and Starling.Game manifest theme.
- **Implementation / logic:** Declares one style inheriting Theme.Black.NoTitleBar.Fullscreen.
- **Findings:** Minimal theme only; no renderer, cutout, edge-to-edge or system-bar behavior.
- **RustChunk relevance:** Useful minimal base theme for a fullscreen game activity.
- **Decision:** adaptable; reference; android-theme
- **1.20.1 implications:** Theme parent and API compatibility must match RustChunk SDK.
- **Follow-up checks:** Verify resource merge with values-v31, edge-to-edge, cutouts and keyboard insets.
- **RustChunk implementation mapping:** Use a minimal base theme with API-qualified overrides.
- **Evidence required before adoption:** Require manifest/resource merge and multi-API screenshot tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3644. `packaging/android/res/xml/shortcuts.xml`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** dda4f27994351c2079928f35fae4cf2f1e3d076d
- **Size:** 474 bytes
- **Coverage:** Complete EOF coverage (7 logical lines).
- **Purpose:** Declares launcher shortcut that opens diagnostics directly.
- **Dependencies / cross-references:** Manifest shortcuts metadata, DiagnosticsActivity, application ID placeholder and diagnostics_title.
- **Implementation / logic:** One enabled static shortcut with info icon and explicit VIEW target package/class.
- **Findings:** Only one shortcut; relies on manifest alias and launcher support.
- **RustChunk relevance:** Useful direct recovery entry point when game cannot start.
- **Decision:** adaptable; reference; android-app-shortcut
- **1.20.1 implications:** Application ID and class must be rewritten; diagnostics must not initialize game.
- **Follow-up checks:** Test cold-start shortcut, package replacement and launcher compatibility.
- **RustChunk implementation mapping:** Expose diagnostics shortcut through RustChunk launcher alias if needed.
- **Evidence required before adoption:** Require cold-start shortcut test after simulated native crash.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3645. `packaging/icons/hicolor/128x128/apps/horizon.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 82aa505087f9619dd9fd2c21a0cf07b4c6191d3a
- **Size:** 1651 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** 128x128 Linux application icon variant for Horizon/Starling branding.
- **Dependencies / cross-references:** hicolor icon theme packaging and shared icon artwork.
- **Implementation / logic:** PNG rendered at 128x128; dark rounded square, purple horizontal band and pale geometric H/pause-like central mark; no visible text.
- **Findings:** Visual asset only; no Android adaptive-icon metadata or attribution embedded.
- **RustChunk relevance:** Branding reference only.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Do not reuse Starling/Horizon mark as RustChunk identity without licensing review.
- **Follow-up checks:** Compare variants and inspect license/provenance.
- **RustChunk implementation mapping:** Create distinct RustChunk adaptive and legacy icons.
- **Evidence required before adoption:** Require original artwork or documented permission and Samsung launcher tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; PNG rendered and visually reviewed; static asset review only; not reused.

### 3646. `packaging/icons/hicolor/16x16/apps/horizon.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 034e64bcab27b1dc968cac4bc9644d2a6c2b182e
- **Size:** 280 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** 16x16 hicolor icon variant.
- **Dependencies / cross-references:** Linux hicolor theme and shared Horizon artwork.
- **Implementation / logic:** PNG rendered at 16x16; tiny dark icon with simplified pale central mark; no visible text.
- **Findings:** Purple band and fine geometry are barely distinguishable at this size.
- **RustChunk relevance:** Small-icon legibility reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Not an Android launcher icon specification or RustChunk asset.
- **Follow-up checks:** Review pixel contrast and compare adjacent sizes.
- **RustChunk implementation mapping:** Design RustChunk small icon separately from adaptive layers.
- **Evidence required before adoption:** Require device-density rendering and small-icon contrast review.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; PNG rendered and visually reviewed; static asset review only; not reused.

### 3647. `packaging/icons/hicolor/24x24/apps/horizon.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 10729734634614a9d04b478820452a0cc66e7ae5
- **Size:** 390 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** 24x24 hicolor icon variant.
- **Dependencies / cross-references:** Linux hicolor theme and shared Horizon artwork.
- **Implementation / logic:** PNG rendered at 24x24; dark rounded square with centered pale geometric mark and subtle horizontal accent; no text.
- **Findings:** Small-scale details are reduced.
- **RustChunk relevance:** Size-specific icon readability reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Not a RustChunk identity asset or Android density set.
- **Follow-up checks:** Compare pixel alignment and contrast with adjacent sizes.
- **RustChunk implementation mapping:** Generate RustChunk icons from a vector master.
- **Evidence required before adoption:** Require size-by-size review and launcher tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; PNG rendered and visually reviewed; static asset review only; not reused.

### 3648. `packaging/icons/hicolor/256x256/apps/horizon.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 5610f66051464788a1329ab905533400cca20890
- **Size:** 3428 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** 256x256 high-resolution hicolor icon variant.
- **Dependencies / cross-references:** Linux hicolor theme and shared Horizon artwork.
- **Implementation / logic:** PNG rendered at 256x256; dark rounded-square background, broad purple stripe and two pale vertical strokes forming central emblem; no text.
- **Findings:** Brand asset is not proof of Gaming Hub recognition behavior.
- **RustChunk relevance:** Reference for scalable icon geometry and contrast.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Do not adopt as RustChunk branding; Android adaptive layers are separate.
- **Follow-up checks:** Check alpha edges, corner radius, contrast and license.
- **RustChunk implementation mapping:** Create a RustChunk vector master and export adaptive/legacy densities.
- **Evidence required before adoption:** Require licensed artwork and Samsung Gaming Hub tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; PNG rendered and visually reviewed; static asset review only; not reused.

### 3649. `packaging/icons/hicolor/32x32/apps/horizon.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 3603cbe0fa7130493960a71c591b582d389983e1
- **Size:** 424 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** 32x32 hicolor icon variant.
- **Dependencies / cross-references:** Linux hicolor theme and shared Horizon artwork.
- **Implementation / logic:** PNG rendered at 32x32; compact dark square with pale central glyph and horizontal accent; no text.
- **Findings:** Main mark remains visible but fine details are reduced.
- **RustChunk relevance:** Small desktop icon legibility reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Not a RustChunk icon or Android adaptive substitute.
- **Follow-up checks:** Compare antialiasing and centering with adjacent sizes.
- **RustChunk implementation mapping:** Generate RustChunk density resources from a master vector.
- **Evidence required before adoption:** Require actual device launcher rendering and contrast review.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; PNG rendered and visually reviewed; static asset review only; not reused.

### 3650. `packaging/icons/hicolor/48x48/apps/horizon.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 63f1d4e176df7d214e7b5c1b29ffc30cb405e8f0
- **Size:** 616 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** 48x48 hicolor icon variant.
- **Dependencies / cross-references:** Linux hicolor theme and shared Horizon artwork.
- **Implementation / logic:** PNG rendered at 48x48; rounded dark square with pale central two-stroke mark and purple band; no text.
- **Findings:** More legible than smallest variants but remains desktop hicolor artwork.
- **RustChunk relevance:** Multi-size icon consistency reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Branding and Android packaging differ; no direct reuse in RustChunk.
- **Follow-up checks:** Verify pixel alignment and license provenance.
- **RustChunk implementation mapping:** Use as a size comparison checklist for RustChunk artwork.
- **Evidence required before adoption:** Require original artwork and Samsung/Android icon tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; PNG rendered and visually reviewed; static asset review only; not reused.

### 3651. `packaging/icons/hicolor/512x512/apps/horizon.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 9bf6b676468876407f1c55cdda0f34636d01a4e0
- **Size:** 7618 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** 512x512 hicolor Horizon application icon.
- **Dependencies / cross-references:** Linux hicolor packaging and Horizon icon family.
- **Implementation / logic:** PNG rendered at 512x512; dark rounded-square field with a broad purple horizontal band; unlike the paired vector/ICNS forms, no central vertical glyph is visible.
- **Findings:** Visual variant differs from other icon representations; not Android adaptive artwork.
- **RustChunk relevance:** Brand comparison only.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Do not reuse as RustChunk identity without rights review.
- **Follow-up checks:** Compare all platform icon variants and provenance.
- **RustChunk implementation mapping:** Create a distinct RustChunk icon family.
- **Evidence required before adoption:** Require original artwork and launcher tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3652. `packaging/icons/hicolor/64x64/apps/horizon.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 1add8b2d610b44bd2b16ec3816882154c7d000e2
- **Size:** 797 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** 64x64 hicolor Horizon icon.
- **Dependencies / cross-references:** Linux hicolor icon theme and shared artwork.
- **Implementation / logic:** PNG rendered at 64x64; dark rounded tile, two pale vertical bars with muted lower halves and a narrow purple horizontal line.
- **Findings:** Simple two-bar emblem; no text; desktop asset only.
- **RustChunk relevance:** Small icon legibility reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Not RustChunk artwork or Android adaptive icon.
- **Follow-up checks:** Compare with 16/24/32/48/128/256/512 variants.
- **RustChunk implementation mapping:** Generate device-density RustChunk icons from vector master.
- **Evidence required before adoption:** Require original artwork and device rendering.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3653. `packaging/icons/hicolor/scalable/apps/horizon.svg`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 94fb593fc7a57ff5199a5a230a04883698d5b538
- **Size:** 537 bytes
- **Coverage:** Complete EOF coverage (10 logical lines).
- **Purpose:** Scalable SVG application icon.
- **Dependencies / cross-references:** Linux hicolor scalable icon theme; SVG title Horizon and clipping path.
- **Implementation / logic:** SVG source reviewed and rendered at 256x256; dark rounded tile, two pale vertical strokes above the center line, dimmer lower strokes and a purple horizontal stripe.
- **Findings:** Vector geometry is clear and small; no text in visible mark. Distinct from 512 PNG band-only variant.
- **RustChunk relevance:** Useful vector-master comparison.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Not RustChunk identity; licensing and derivative rights need review.
- **Follow-up checks:** Compare SVG output against raster variants and inspect asset license.
- **RustChunk implementation mapping:** Create a RustChunk-owned SVG/vector master.
- **Evidence required before adoption:** Require rights review and visual parity tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3654. `packaging/icons/horizon.icns`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 4c691e990bb37a7dea5734ad5ee75db6c4c94324
- **Size:** 40332 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Apple ICNS icon container for Horizon.
- **Dependencies / cross-references:** macOS icon format with 11 embedded PNG chunks.
- **Implementation / logic:** Decoded all 11 chunks (icp4/icp5/icp6/ic07/ic08/ic09/ic10/ic11/ic12/ic13/ic14) and rendered each; variants range from tiny glyphs to 1024px tile with pale two-bar mark and purple band.
- **Findings:** Multiple sizes are embedded in one file; some @2x chunks duplicate dimensions; high-res mark is more detailed than the standalone 512 PNG.
- **RustChunk relevance:** Useful multi-resolution packaging reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** macOS asset only; not Android launcher metadata or RustChunk artwork.
- **Follow-up checks:** Check all embedded chunks, color profile, alpha and license provenance.
- **RustChunk implementation mapping:** Generate RustChunk icon containers only from original vector artwork.
- **Evidence required before adoption:** Require macOS icon rendering and documented asset rights.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3655. `packaging/icons/horizon.ico`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** b580493d67639d431db4dacecd4720b9f27b7b90
- **Size:** 7704 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Windows ICO icon container for Horizon.
- **Dependencies / cross-references:** Windows ICO directory and seven embedded PNG images.
- **Implementation / logic:** Decoded and rendered all seven embedded PNGs at 16, 24, 32, 48, 64, 128 and 256px; dark rounded tile with pale two-bar glyph and purple horizontal accent.
- **Findings:** A single ICO packages seven raster sizes; small versions simplify the mark and larger variants retain the rounded tile.
- **RustChunk relevance:** Useful Windows multi-resolution packaging reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Windows desktop icon only; not RustChunk artwork.
- **Follow-up checks:** Verify all embedded sizes and compare with ICNS/SVG/raster assets.
- **RustChunk implementation mapping:** Create a RustChunk ICO only if Windows desktop launcher is built.
- **Evidence required before adoption:** Require Windows shell rendering and asset rights.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3656. `packaging/linux/appimage.sh`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** d1e32ac123e70c14f8e116a1e5ad6f8fa220ae9a
- **Size:** 6714 bytes
- **Coverage:** Complete EOF coverage (143 logical lines).
- **Purpose:** Builds a Linux x86_64 AppImage for the Horizon launcher and packages non-system shared libraries.
- **Dependencies / cross-references:** Bash, ldd, patchelf, AppImage tool 1.9.1, desktop entry, hicolor icons and launcher binary.
- **Implementation / logic:** Copies launcher/assets, walks ldd dependencies, excludes system C/graphics/session/audio libraries, patches runpaths, downloads AppImage tool with pinned SHA256, builds and smoke-checks version/desktop entry.
- **Findings:** Explicit x86_64 and glibc baseline; APPIMAGETOOL override bypasses the built-in pinned download hash; ldd-based dependency discovery is host-specific.
- **RustChunk relevance:** Useful packaging and dependency-boundary reference only.
- **Decision:** adaptable; reference; linux-packaging
- **1.20.1 implications:** Linux desktop, not Android; no Forge/JVM support.
- **Follow-up checks:** Test AppImage on clean minimum-glibc systems, verify custom tool path and dependency closure.
- **RustChunk implementation mapping:** Keep Android packaging separate; use similar pinned-tool and artifact smoke-check principles.
- **Evidence required before adoption:** Require clean Linux VM install/launch tests and tool hash verification.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3657. `packaging/linux/horizon-launcher.desktop`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 2e0000e64606acbbdc2b775ad6860b94f91dc303
- **Size:** 261 bytes
- **Coverage:** Complete EOF coverage (10 logical lines).
- **Purpose:** Declares the Horizon launcher as a Linux desktop application in the Game category.
- **Dependencies / cross-references:** Desktop Entry specification and AppImage launcher executable/icon.
- **Implementation / logic:** Names app Horizon, describes a Minecraft Java Edition 26.3 launcher, runs horizon-launcher and sets Categories=Game.
- **Findings:** Desktop classification only; the product description is 26.3-specific and not Android Gaming Hub evidence.
- **RustChunk relevance:** Useful desktop game-category metadata pattern.
- **Decision:** adaptable; reference; linux-desktop-entry
- **1.20.1 implications:** Linux-only; application ID and description must be RustChunk-specific.
- **Follow-up checks:** Validate desktop entry and icon lookup on GNOME/KDE.
- **RustChunk implementation mapping:** Use Android manifest game metadata for Android rather than copying desktop file.
- **Evidence required before adoption:** Require desktop-entry validation and Linux shell tests if desktop target exists.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3658. `packaging/linux/horizon.desktop`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 1331e5c4accc202a38821d52cd4e4c01f3c3ce11
- **Size:** 205 bytes
- **Coverage:** Complete EOF coverage (10 logical lines).
- **Purpose:** Declares the Horizon game client desktop entry, hidden from normal app menus.
- **Dependencies / cross-references:** Desktop Entry specification and horizon-client binary.
- **Implementation / logic:** Runs horizon-client, uses Horizon icon, Categories=Game and NoDisplay=true.
- **Findings:** Hidden secondary entry is not the launcher entry; description targets Java Edition 26.3.
- **RustChunk relevance:** Desktop packaging reference only.
- **Decision:** reference; version-specific; linux-desktop-entry
- **1.20.1 implications:** Linux-only and version-specific.
- **Follow-up checks:** Check desktop association and launcher/client separation.
- **RustChunk implementation mapping:** Keep RustChunk client entry separate only if a Linux target is added.
- **Evidence required before adoption:** Require desktop shell tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3659. `packaging/lowbudget/hand.py`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 6ca89081bafebd6185fe505d88d9589cbba1011e
- **Size:** 11665 bytes
- **Coverage:** Complete EOF coverage (195 logical lines).
- **Purpose:** Defines a procedural handwriting glyph set and renders text with per-character jitter, slant, size, baseline and pressure variation.
- **Dependencies / cross-references:** Python math/random and Pillow drawing API; make.py imports these functions.
- **Implementation / logic:** Stores vector-like strokes for ASCII letters/digits/punctuation, smooths with Catmull-Rom, densifies paths, applies seeded random variation and draws rounded line strokes.
- **Findings:** Deterministic seed enables repeatable texture generation; no file I/O in this module; unsupported characters fall back to question mark; kerning is only fixed advance spacing.
- **RustChunk relevance:** Useful reference for generating stylized low-resolution UI font atlases.
- **Decision:** adaptable; reference; procedural-font-generator
- **1.20.1 implications:** Glyph set and visual style are custom resource-pack content, not a Minecraft 1.20.1 runtime font engine.
- **Follow-up checks:** Inspect make.py for atlas dimensions/seeds; test all glyphs, unsupported Unicode and baseline/advance metrics.
- **RustChunk implementation mapping:** Use only as inspiration for a RustChunk-owned font generator with licensed glyph design.
- **Evidence required before adoption:** Require generated atlas visual review, Unicode fallback tests and license review.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3660. `packaging/lowbudget/pack/assets/minecraft/font/default.json`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** cde82c0eb789dd60f2ac7f7f4be9063189b2a72e
- **Size:** 467 bytes
- **Coverage:** Complete EOF coverage (30 logical lines).
- **Purpose:** Defines the low-budget pack default bitmap font and four Starling-specific font variants.
- **Dependencies / cross-references:** Minecraft resource-pack font provider schema, hand_0.png and Starling custom font variant loader.
- **Implementation / logic:** Two providers: space advance 4 and bitmap atlas hand_0 at height 10/ascent 8 with six 16-character rows; custom starling:variants lists hand_1 through hand_4.
- **Findings:** Valid JSON; custom top-level starling:variants key is not standard vanilla font schema and requires Starling code; glyph atlas is ASCII-oriented.
- **RustChunk relevance:** Useful example of bitmap font provider and custom variants.
- **Decision:** adaptable; reference; version-specific; font-resource
- **1.20.1 implications:** Resource-pack font schema and custom key are version-specific; verify 1.20.1 format.
- **Follow-up checks:** Check vanilla 1.20.1 provider schema, codepoint mapping, missing glyph and variant loader.
- **RustChunk implementation mapping:** Rebuild a 1.20.1-compatible bitmap font pack and custom variant selector.
- **Evidence required before adoption:** Require 1.20.1 resource reload tests and glyph coverage.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3661. `packaging/lowbudget/pack/assets/minecraft/lang/en_us.json`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 97dc50741ae051ef67958879d0339086133eaa35
- **Size:** 10886 bytes
- **Coverage:** Complete EOF coverage (244 logical lines).
- **Purpose:** English localization for low-budget options, controls, accessibility, resource packs and Horizon controller features.
- **Dependencies / cross-references:** Minecraft language resource format and Starling/Horizon custom option keys.
- **Implementation / logic:** Valid JSON with 242 keys, 16 format placeholders and no duplicate literal keys; contains graphics/render-distance/FPS and Horizon controller labels.
- **Findings:** Contains later-version keys and custom Horizon options; strings such as Low Budget Settings lol and Way Too Wide are informal product copy; no translation coverage.
- **RustChunk relevance:** Useful key organization and wording reference.
- **Decision:** adaptable; reference; version-specific; localization-resource
- **1.20.1 implications:** Many keys differ from 1.20.1 and must be mapped to Forge target keys.
- **Follow-up checks:** Compare every key with 1.20.1 language registry and placeholder argument counts.
- **RustChunk implementation mapping:** Create a RustChunk-specific locale file with explicit version mapping.
- **Evidence required before adoption:** Require resource reload and placeholder formatting tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3662. `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_0.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** f644128716d56725ede497da47462fbf0fb5c3a1
- **Size:** 12956 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Primary handwritten bitmap font atlas.
- **Dependencies / cross-references:** default.json bitmap provider and hand.py glyph generator.
- **Implementation / logic:** PNG rendered; transparent atlas with white hand-drawn punctuation, digits, uppercase and lowercase ASCII glyphs in a grid.
- **Findings:** Custom font artwork; unsupported Unicode is not represented in this atlas.
- **RustChunk relevance:** Useful UI font atlas reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Not a drop-in 1.20.1 font without matching provider schema and glyph metrics.
- **Follow-up checks:** Inspect glyph grid dimensions, alpha and missing codepoints; compare against provider chars rows.
- **RustChunk implementation mapping:** Generate a RustChunk-owned atlas and codepoint map.
- **Evidence required before adoption:** Require visual glyph review, font reload tests and asset rights.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3663. `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_1.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 470be6b7c43370d22edfd2a0d511e7da4fc6784a
- **Size:** 13189 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Handwritten bitmap font variation 1.
- **Dependencies / cross-references:** Starling font variants list and hand.py deterministic glyph generator.
- **Implementation / logic:** PNG rendered; same ASCII glyph grid with slightly varied stroke shapes, slant and spacing.
- **Findings:** Variation is subtle and intended to avoid repeated identical glyph appearance.
- **RustChunk relevance:** Useful procedural font variation reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires Starling custom variant loader and target-version font mapping.
- **Follow-up checks:** Compare all glyph cell boundaries and metrics against default.json.
- **RustChunk implementation mapping:** Use deterministic seeded variants only in a RustChunk-owned font system.
- **Evidence required before adoption:** Require atlas visual and metrics tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3664. `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_2.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** b9f321fa646a0f89d7d8beebc4306b046a084b7d
- **Size:** 12959 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Handwritten bitmap font variation 2.
- **Dependencies / cross-references:** Starling font variants list and hand.py generator.
- **Implementation / logic:** PNG rendered; white handwritten ASCII glyphs on transparent atlas with alternate stroke jitter and shape.
- **Findings:** Small glyph-level differences from hand_0/1; same overall grid and style.
- **RustChunk relevance:** Useful font variation reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires custom loader and 1.20.1 schema check.
- **Follow-up checks:** Compare codepoint alignment and metrics across variants.
- **RustChunk implementation mapping:** Use as design inspiration only.
- **Evidence required before adoption:** Require glyph-grid and resource reload tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3665. `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_3.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** ed6d005088ea8bae9e351d69c28c60cb05fe275d
- **Size:** 13148 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Handwritten bitmap font variation 3.
- **Dependencies / cross-references:** Starling font variants list and hand.py generator.
- **Implementation / logic:** PNG rendered; same handwritten ASCII atlas with changed per-glyph slant, wobble and stroke thickness.
- **Findings:** Variation is subtle; no Unicode glyph coverage beyond atlas map.
- **RustChunk relevance:** Useful font variation reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires custom loader and 1.20.1 mapping.
- **Follow-up checks:** Check all rows align with default provider char map.
- **RustChunk implementation mapping:** Use only with a RustChunk-owned font pipeline.
- **Evidence required before adoption:** Require visual and codepoint alignment tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3666. `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_4.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 91bacf1717c4ffeec4b266174fac037e1e9df9c9
- **Size:** 13153 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Handwritten bitmap font variation 4.
- **Dependencies / cross-references:** Starling font variants list and hand.py generator.
- **Implementation / logic:** PNG rendered; alternate hand-drawn ASCII atlas with small stroke, slant and spacing changes.
- **Findings:** Variation set is custom art; no direct Java/Forge compatibility.
- **RustChunk relevance:** Useful font variation reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Needs custom variant selection and 1.20.1 font schema.
- **Follow-up checks:** Verify metrics and unsupported character fallback.
- **RustChunk implementation mapping:** Use only as inspiration for custom RustChunk font assets.
- **Evidence required before adoption:** Require asset license and resource reload tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3667. `packaging/lowbudget/pack/assets/minecraft/textures/gui/footer_separator.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 23200ec05988269bf8fad133446e6bbc56204476
- **Size:** 157 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Low-budget GUI footer separator texture.
- **Dependencies / cross-references:** Minecraft GUI resource-pack textures.
- **Implementation / logic:** PNG rendered; very thin horizontal pale gray line with subtle pixel variation/soft edges on transparent background.
- **Findings:** Tiny single-purpose divider; no text.
- **RustChunk relevance:** Useful subtle UI separator reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Texture path and UI usage differ by Minecraft version.
- **Follow-up checks:** Inspect dimensions/alpha and compare with header separator.
- **RustChunk implementation mapping:** Create RustChunk UI divider in its own style.
- **Evidence required before adoption:** Require in-game GUI scaling tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3668. `packaging/lowbudget/pack/assets/minecraft/textures/gui/header_separator.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 70df76ea6e379083f3190cdc3a4853d723ca84a8
- **Size:** 150 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Low-budget GUI header separator texture.
- **Dependencies / cross-references:** Minecraft GUI resource-pack textures.
- **Implementation / logic:** PNG rendered; thin horizontal light gray/white separator with slight pixel variation on transparency.
- **Findings:** Visually close to footer separator but a distinct blob and file.
- **RustChunk relevance:** Useful UI divider reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Texture path and UI usage differ by version.
- **Follow-up checks:** Compare dimensions/alpha and exact pixel difference from footer.
- **RustChunk implementation mapping:** Create version-independent UI divider asset for RustChunk.
- **Evidence required before adoption:** Require GUI scaling tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3669. `packaging/lowbudget/pack/assets/minecraft/textures/gui/menu_background.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** f2022746015d315d38dd5d04ab83561dab52c23a
- **Size:** 9801 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Low-budget menu background texture.
- **Dependencies / cross-references:** Minecraft GUI resource-pack background texture.
- **Implementation / logic:** PNG rendered; dark charcoal field with very subtle mottled/noise texture and no visible text or objects.
- **Findings:** Texture is low contrast and intended as a quiet menu backdrop.
- **RustChunk relevance:** Useful menu surface texture reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Must be checked against 1.20.1 background scaling and GUI layers.
- **Follow-up checks:** Inspect tiling, alpha and contrast on light/dark displays.
- **RustChunk implementation mapping:** Create a RustChunk-owned low-noise menu background.
- **Evidence required before adoption:** Require actual UI screenshot and accessibility contrast review.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3670. `packaging/lowbudget/pack/assets/minecraft/textures/gui/menu_list_background.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 0bbddd06e28b0bcaf882a7df7203d13a70af1521
- **Size:** 9530 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Low-budget menu list background texture.
- **Dependencies / cross-references:** Minecraft GUI resource-pack list background texture.
- **Implementation / logic:** PNG rendered; dark charcoal mottled/noise surface, visually close to menu_background but used for list panels.
- **Findings:** No visible text; distinction from menu background is subtle.
- **RustChunk relevance:** Useful panel texture reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Resource location and GUI composition are version-specific.
- **Follow-up checks:** Compare pixel values, tiling and list panel contrast.
- **RustChunk implementation mapping:** Create a separate RustChunk panel texture only if needed.
- **Evidence required before adoption:** Require UI screenshot and contrast checks.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3671. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** a1808f7e5081c4d41f9a7df7b09f66bca100135e
- **Size:** 61321 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Primary low-budget GUI button sprite.
- **Dependencies / cross-references:** button.png.mcmeta nine-slice configuration and Minecraft GUI sprite loader.
- **Implementation / logic:** PNG rendered; very wide dark mottled button with thin irregular pale outline and transparent exterior.
- **Findings:** Requires matching nine-slice metadata; visual style may be noisy at small scale.
- **RustChunk relevance:** Useful nine-slice button skin reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Sprite metadata schema and widget IDs may not exist in 1.20.1.
- **Follow-up checks:** Verify dimensions, border inset, hover/disabled state and metadata interpretation.
- **RustChunk implementation mapping:** Implement RustChunk buttons in its own UI system or target-version resource pack.
- **Evidence required before adoption:** Require 1.20.1 resource-pack render tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3672. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines).
- **Purpose:** Defines nine-slice scaling for the primary button sprite.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser and button.png.
- **Implementation / logic:** JSON specifies 200x20 nine_slice, borders left/right 6 and top/bottom 5, stretch_inner true.
- **Findings:** Valid structure; this same blob SHA is also used by button_1.png.mcmeta; version support must be checked.
- **RustChunk relevance:** Useful nine-slice metadata reference.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Likely tied to newer sprite scaling schema; verify 1.20.1 parser support.
- **Follow-up checks:** Test all border regions and narrow/wide button sizes.
- **RustChunk implementation mapping:** Use only after confirming target-version metadata schema.
- **Evidence required before adoption:** Require 1.20.1 resource reload and scaling tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3673. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_1.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** bad89171c4cfa1578e59f231e4861f6cc466e1f1
- **Size:** 58780 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Alternate low-budget GUI button sprite variant 1.
- **Dependencies / cross-references:** button_1.png.mcmeta identical nine-slice metadata blob and Minecraft sprite loader.
- **Implementation / logic:** PNG rendered; wide dark textured button with thin irregular pale border; interior texture and edge noise differ from button.png.
- **Findings:** Variant is visually distinct but shares the same nine-slice geometry; custom UI state mapping is not included here.
- **RustChunk relevance:** Useful button state/variant reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires target-version sprite registry and metadata support.
- **Follow-up checks:** Map which GUI state uses this variant and test hover/pressed/disabled states.
- **RustChunk implementation mapping:** Create RustChunk-owned button states and consistent nine-slice geometry.
- **Evidence required before adoption:** Require in-game state transition screenshots.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3674. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_1.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines).
- **Purpose:** Defines nine-slice scaling for button variant 1.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser and button_1.png.
- **Implementation / logic:** Same JSON blob as button.png.mcmeta: 200x20 nine_slice, 6px horizontal and 5px vertical borders, stretch_inner true.
- **Findings:** Identical content and SHA to entry 3672; duplicate blob reuse is expected, while paths remain unique.
- **RustChunk relevance:** Useful metadata deduplication reference.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target-version parser support remains unverified.
- **Follow-up checks:** Check why button_2 has no matching metadata in this range; inspect later files for its sidecar.
- **RustChunk implementation mapping:** Reuse one shared metadata template only if target schema supports it.
- **Evidence required before adoption:** Require resource reload and scaling tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; static review only; not executed.

### 3675. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_2.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 2666fa2ba9e43a2a7ca3338d01d4ecc3bd964644
- **Size:** 55408 bytes
- **Coverage:** Complete binary image/vector/container; all embedded representations rendered and visually inspected.
- **Purpose:** Alternate low-budget GUI button sprite variant 2.
- **Dependencies / cross-references:** Minecraft GUI sprite system; no sidecar metadata appears in this 25-file range.
- **Implementation / logic:** PNG rendered; wide dark mottled button with thin pale irregular outline and a slightly different interior pattern.
- **Findings:** The matching sidecar is confirmed at audit entry 3676; it was outside this entry's 3651–3675 checkpoint.
- **RustChunk relevance:** Useful additional button skin reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Sprite registration/scaling must be verified against 1.20.1.
- **Follow-up checks:** Search full tree for button_2 metadata and state mapping.
- **RustChunk implementation mapping:** Do not integrate until texture metadata and widget state are mapped.
- **Evidence required before adoption:** Require 1.20.1 resource-pack render test.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3676. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_2.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice scaling metadata for button_2.png.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser and button_2 texture.
- **Implementation / logic:** Valid JSON: 200x20 nine_slice, 6px left/right and 5px top/bottom borders, stretch_inner true.
- **Findings:** Confirms button_2 has a sidecar; content SHA is shared by other button-state metadata files.
- **RustChunk relevance:** Useful reusable metadata template.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Newer sprite metadata schema needs 1.20.1 validation.
- **Follow-up checks:** Test border preservation at narrow/wide widths and map to state.
- **RustChunk implementation mapping:** Use only if target resource pack parser supports nine_slice.
- **Evidence required before adoption:** Require 1.20.1 resource reload and scaling tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3677. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_3.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 8e717353b0adfeb088ae119ba1d18fbe759a3660
- **Size:** 59783 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Fourth normal-state button texture variant.
- **Dependencies / cross-references:** button_3.png.mcmeta and Minecraft GUI sprite loader.
- **Implementation / logic:** PNG rendered; wide dark mottled button with thin irregular pale outline and transparent exterior; interior grain differs from other normal variants.
- **Findings:** Visual state variant only; metadata is a separate source file.
- **RustChunk relevance:** Useful button skin reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Sprite registry and nine-slice behavior must match 1.20.1.
- **Follow-up checks:** Compare with button/button_1/button_2 and inspect all GUI states.
- **RustChunk implementation mapping:** Build RustChunk-owned normal button texture variants.
- **Evidence required before adoption:** Require in-game 1.20.1 UI render test.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3678. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_3.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice scaling metadata for button_3.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser and button_3 texture.
- **Implementation / logic:** Valid JSON: 200x20 nine_slice, 6px horizontal and 5px vertical borders, stretch_inner true.
- **Findings:** Identical content/SHA to other button metadata files; shared blob reuse is expected.
- **RustChunk relevance:** Useful nine-slice reference.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target-version parser support unverified.
- **Follow-up checks:** Test repeated reuse and all border corners.
- **RustChunk implementation mapping:** Use shared template only after target schema confirmation.
- **Evidence required before adoption:** Require resource reload and scale tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3679. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** bb879590a36723ccfeeef9765da14e0a51b39b4b
- **Size:** 46306 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Disabled-state button texture.
- **Dependencies / cross-references:** button_disabled.png.mcmeta and GUI disabled widget state.
- **Implementation / logic:** PNG rendered; wide dark textured button with pale outline and substantially darker interior than active button variants.
- **Findings:** Disabled visual state is encoded in artwork; actual widget state mapping is outside this file.
- **RustChunk relevance:** Useful disabled-state styling reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Widget sprite IDs and state selection differ by game version.
- **Follow-up checks:** Compare contrast and legibility against enabled/highlighted states.
- **RustChunk implementation mapping:** Create distinct RustChunk disabled state with accessible contrast.
- **Evidence required before adoption:** Require disabled-state screenshot and accessibility contrast test.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3680. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice scaling metadata for disabled button texture.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid JSON: 200x20 nine_slice, borders 6/5 and stretch_inner true.
- **Findings:** Same metadata blob as normal and highlighted button variants.
- **RustChunk relevance:** Useful scaling template.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target-version parser support unverified.
- **Follow-up checks:** Verify disabled texture scales without distorting border.
- **RustChunk implementation mapping:** Use shared metadata only if supported by target.
- **Evidence required before adoption:** Require 1.20.1 UI scaling test.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3681. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_1.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 1323629b71cfbd34875be97881e03efe1809163c
- **Size:** 49000 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Disabled-state button texture variant 1.
- **Dependencies / cross-references:** button_disabled_1.png.mcmeta and widget state variants.
- **Implementation / logic:** PNG rendered; dark mottled interior with irregular light border, slightly different grain from disabled base.
- **Findings:** Variant mapping and selection are not defined in this file.
- **RustChunk relevance:** Useful disabled texture variation reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires target sprite registration.
- **Follow-up checks:** Compare state mapping, border thickness and readability.
- **RustChunk implementation mapping:** Keep state variants in RustChunk-owned assets.
- **Evidence required before adoption:** Require UI state rendering tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3682. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_1.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice scaling metadata for disabled button variant 1.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid JSON with 200x20 nine_slice and 6/5 borders.
- **Findings:** Same blob SHA/content as all other button state sidecars.
- **RustChunk relevance:** Useful metadata deduplication pattern.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target-version parser support unverified.
- **Follow-up checks:** Test narrow and wide scaling.
- **RustChunk implementation mapping:** Use shared metadata template only after target check.
- **Evidence required before adoption:** Require resource reload tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3683. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_2.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 1ee071c636833d55c9cee7d21ace714bb449c1a0
- **Size:** 46092 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Disabled-state button texture variant 2.
- **Dependencies / cross-references:** button_disabled_2.png.mcmeta and widget state variants.
- **Implementation / logic:** PNG rendered; dark textured wide button with pale irregular outline; texture grain differs slightly from other disabled variants.
- **Findings:** Artwork only; state selection is external.
- **RustChunk relevance:** Useful UI state texture reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Target widget registry is version-specific.
- **Follow-up checks:** Compare contrast and nine-slice edges.
- **RustChunk implementation mapping:** Use as visual inspiration only.
- **Evidence required before adoption:** Require in-game UI test.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3684. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_2.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice metadata for disabled button variant 2.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid JSON with 200x20 dimensions, 6px horizontal/5px vertical borders and stretch_inner.
- **Findings:** Shared blob SHA/content with button metadata family.
- **RustChunk relevance:** Useful shared metadata pattern.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Version-specific scaling schema.
- **Follow-up checks:** Verify target parser and corner preservation.
- **RustChunk implementation mapping:** Reuse only after 1.20.1 compatibility check.
- **Evidence required before adoption:** Require resource reload and scaling tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3685. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_3.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 5b2d578852a2c0c0a7bd9b7ecfaaf806a4809d67
- **Size:** 46159 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Disabled-state button texture variant 3.
- **Dependencies / cross-references:** button_disabled_3.png.mcmeta and widget disabled state.
- **Implementation / logic:** PNG rendered; dark textured interior with a thin pale frame and a distinct grain pattern.
- **Findings:** Variant art does not prove a matching state is used at runtime.
- **RustChunk relevance:** Useful visual state reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires target widget/state mapping.
- **Follow-up checks:** Test contrast and hover/disabled distinction.
- **RustChunk implementation mapping:** Create explicit RustChunk state assets.
- **Evidence required before adoption:** Require actual UI state screenshots.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3686. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_3.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice metadata for disabled button variant 3.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid JSON with 200x20 nine_slice, 6/5 borders and stretch_inner true.
- **Findings:** Same blob as all other button sidecars.
- **RustChunk relevance:** Useful metadata deduplication pattern.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target version may not recognize schema.
- **Follow-up checks:** Test parser and scaling.
- **RustChunk implementation mapping:** Use only after 1.20.1 verification.
- **Evidence required before adoption:** Require resource reload tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3687. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 1efcc21db9607546246a6246fef3917394213741
- **Size:** 50632 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Highlighted/active button texture.
- **Dependencies / cross-references:** button_highlighted.png.mcmeta and highlighted widget state.
- **Implementation / logic:** PNG rendered; brighter gray diagonal hatch texture inside a pale irregular rectangular outline.
- **Findings:** Represents a highlighted state; animation or focus behavior is not included.
- **RustChunk relevance:** Useful hover/focus skin reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** State mapping and sprite schema are version-specific.
- **Follow-up checks:** Test focus visibility and color contrast.
- **RustChunk implementation mapping:** Design distinct RustChunk hover/focus texture and state.
- **Evidence required before adoption:** Require keyboard/controller/touch focus screenshots.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3688. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice metadata for highlighted button.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid JSON with 200x20 nine_slice and 6/5 border values.
- **Findings:** Same shared blob as other button states.
- **RustChunk relevance:** Useful nine-slice template.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target-version support unverified.
- **Follow-up checks:** Test border preservation on focus state.
- **RustChunk implementation mapping:** Reuse only after target schema check.
- **Evidence required before adoption:** Require 1.20.1 resource tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3689. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_1.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 69677683354f60b4857e3cb5fabb9b202759f04d
- **Size:** 45077 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Highlighted button texture variant 1.
- **Dependencies / cross-references:** button_highlighted_1.png.mcmeta and widget state variants.
- **Implementation / logic:** PNG rendered; diagonal lighter hatch pattern with a pale outline; one edge has a visibly different transparent/empty section.
- **Findings:** Variant may represent different visual state/asset crop; exact mapping is not shown.
- **RustChunk relevance:** Useful alternate focus texture reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires matching target sprite registry.
- **Follow-up checks:** Compare alpha bounds and state usage.
- **RustChunk implementation mapping:** Keep state mapping explicit in RustChunk UI.
- **Evidence required before adoption:** Require state-specific UI screenshot tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3690. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_1.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice metadata for highlighted variant 1.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid JSON with 200x20 dimensions and 6/5 borders.
- **Findings:** Same shared metadata blob as other button variants.
- **RustChunk relevance:** Useful metadata reuse pattern.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target parser support unverified.
- **Follow-up checks:** Test alpha edge and nine-slice scaling.
- **RustChunk implementation mapping:** Use shared metadata only after target check.
- **Evidence required before adoption:** Require 1.20.1 resource reload tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3691. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_2.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** bf5d1071e6af3dc92c8283ffd2013afea7498167
- **Size:** 49611 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Highlighted button texture variant 2.
- **Dependencies / cross-references:** button_highlighted_2.png.mcmeta and GUI highlighted state.
- **Implementation / logic:** PNG rendered; diagonal gray hatch texture with a thin pale irregular frame.
- **Findings:** Texture alone does not prove runtime focus semantics.
- **RustChunk relevance:** Useful UI state reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Sprite IDs and state selection are version-specific.
- **Follow-up checks:** Check texture alignment and contrast at GUI scales.
- **RustChunk implementation mapping:** Use only as visual inspiration.
- **Evidence required before adoption:** Require in-game focus-state test.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3692. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_2.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice metadata for highlighted variant 2.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid 200x20 nine_slice JSON with 6/5 border values.
- **Findings:** Same blob SHA/content as button state metadata family.
- **RustChunk relevance:** Useful shared metadata pattern.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target version compatibility unverified.
- **Follow-up checks:** Verify border behavior and JSON schema.
- **RustChunk implementation mapping:** Reuse only after 1.20.1 parser test.
- **Evidence required before adoption:** Require resource reload tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3693. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_3.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** ad0d55b50a5569274287af5e06d323158602210f
- **Size:** 49197 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Highlighted button texture variant 3.
- **Dependencies / cross-references:** button_highlighted_3.png.mcmeta and GUI highlighted state.
- **Implementation / logic:** PNG rendered; diagonal light hatch texture and irregular pale outline; some edge transparency differs from other variants.
- **Findings:** State variant mapping is not contained in texture file.
- **RustChunk relevance:** Useful focus/highlight texture reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires target sprite/state mapping.
- **Follow-up checks:** Inspect alpha bounds, scale and contrast.
- **RustChunk implementation mapping:** Use explicit RustChunk focus assets.
- **Evidence required before adoption:** Require UI screenshot and accessibility checks.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3694. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_3.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice metadata for highlighted variant 3.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid 200x20 nine_slice JSON with 6px horizontal and 5px vertical borders.
- **Findings:** Shared content SHA across button variants.
- **RustChunk relevance:** Useful metadata deduplication pattern.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target schema needs 1.20.1 validation.
- **Follow-up checks:** Test all state textures with shared metadata.
- **RustChunk implementation mapping:** Reuse only after compatibility check.
- **Evidence required before adoption:** Require resource reload tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3695. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/scroller.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 18c8cd4dfd173b892ec2d105ea838018dcbf57f7
- **Size:** 2292 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Vertical scroller thumb texture.
- **Dependencies / cross-references:** scroller.png.mcmeta and Minecraft GUI widget sprite loader.
- **Implementation / logic:** PNG rendered; narrow vertical dark textured thumb with pale irregular side border and transparent center/edges.
- **Findings:** Tiny asset designed for nine-slice stretching; contrast may be low on dark background.
- **RustChunk relevance:** Useful scroll control skin reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Widget texture IDs and metadata parser are version-specific.
- **Follow-up checks:** Test minimum thumb height, alpha and touch hit target independently.
- **RustChunk implementation mapping:** Build RustChunk scroller visuals and hit targets separately.
- **Evidence required before adoption:** Require touch/keyboard scroll screenshots and accessibility tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3696. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/scroller.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** d8d19dc35878dee344c686787e632ebfb3449dbe
- **Size:** 240 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice metadata for scroller thumb.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid JSON: 6x32 nine_slice with 2px left/right and 4px top/bottom borders, stretch_inner true.
- **Findings:** Uses a distinct shared metadata blob from the 200x20 button family.
- **RustChunk relevance:** Useful small vertical nine-slice example.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target schema must be validated for 1.20.1.
- **Follow-up checks:** Test very short/tall scroller dimensions.
- **RustChunk implementation mapping:** Use only if target parser supports nine_slice.
- **Evidence required before adoption:** Require resource reload and scroll-state tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3697. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/scroller_background.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** f08f91b8368fe35d60ad8517584152d2c05f58c5
- **Size:** 2151 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Vertical scroller track background texture.
- **Dependencies / cross-references:** scroller_background.png.mcmeta and GUI scroller widget.
- **Implementation / logic:** PNG rendered; narrow vertical dark textured track with thin pale side edges and transparent portions.
- **Findings:** Low contrast; hit area and visual track are separate concerns.
- **RustChunk relevance:** Useful scroller track texture reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Requires target sprite and metadata support.
- **Follow-up checks:** Test track/thumb contrast and scaling.
- **RustChunk implementation mapping:** Implement scroll track and hit testing independently.
- **Evidence required before adoption:** Require actual touch scroll UI tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3698. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/scroller_background.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** d8d19dc35878dee344c686787e632ebfb3449dbe
- **Size:** 240 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice metadata for scroller background.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser.
- **Implementation / logic:** Valid JSON: 6x32 nine_slice, 2px horizontal and 4px vertical borders.
- **Findings:** Same blob SHA/content as scroller thumb metadata.
- **RustChunk relevance:** Useful shared scroller metadata pattern.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target-version support unverified.
- **Follow-up checks:** Verify both scroller textures preserve borders at all sizes.
- **RustChunk implementation mapping:** Reuse shared metadata only after 1.20.1 test.
- **Evidence required before adoption:** Require resource reload and UI scaling tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.

### 3699. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/slider.png`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** c68f0015867d6d1b6e8096032c8541a9bf2d46aa
- **Size:** 47583 bytes
- **Coverage:** Complete binary PNG; rendered and visually inspected.
- **Purpose:** Slider track texture for low-budget GUI settings.
- **Dependencies / cross-references:** slider.png.mcmeta and Minecraft GUI slider widget.
- **Implementation / logic:** PNG rendered; wide dark mottled horizontal track with thin pale irregular outline and transparent exterior.
- **Findings:** Track only; knob/handle and drag behavior are not included in this source file.
- **RustChunk relevance:** Useful slider track visual reference.
- **Decision:** adaptable; reference; media-image
- **1.20.1 implications:** Target sprite IDs and slider state assets may differ in 1.20.1.
- **Follow-up checks:** Inspect associated knob assets and test focus/drag states.
- **RustChunk implementation mapping:** Create RustChunk slider track/handle with independent touch hit targets.
- **Evidence required before adoption:** Require touch drag, keyboard adjustment and GUI screenshot tests.
- **Completion state:** COMPLETE — pinned blob SHA/size verified; image rendered and visually reviewed; static asset review only; not reused.

### 3700. `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/slider.png.mcmeta`
- **Source repository/ref:** `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- **Source SHA:** 52d0899489886054cbec548d1028e59138a6d05e
- **Size:** 242 bytes
- **Coverage:** Complete EOF coverage (16 logical lines); JSON parsed and reviewed.
- **Purpose:** Nine-slice scaling metadata for slider track.
- **Dependencies / cross-references:** Minecraft GUI sprite metadata parser and slider.png.
- **Implementation / logic:** Valid JSON: 200x20 nine_slice, 6px left/right and 5px top/bottom borders, stretch_inner true.
- **Findings:** Shared metadata blob reused by button/slider state assets.
- **RustChunk relevance:** Useful nine-slice metadata reference.
- **Decision:** adaptable; reference; version-specific; sprite-metadata
- **1.20.1 implications:** Target parser support is not proven for Minecraft 1.20.1.
- **Follow-up checks:** Test stretched slider lengths and border integrity.
- **RustChunk implementation mapping:** Use shared metadata only if 1.20.1 schema supports it.
- **Evidence required before adoption:** Require resource reload and slider interaction tests.
- **Completion state:** COMPLETE — pinned source SHA/size verified; full stated coverage read through EOF; JSON parsed; static review only; not executed.
