# RustChunk: Starling engine and Minecraft 1.20.1 port audit

Date: 2026-11-16

Status: **official 1.20.1 inputs verified, indexed, compared and privately decompiled. Starling architecture has been inspected. No native 1.20.1 game port or APK has been built yet.**

## Decision and target

Use Starling as the native Rust engine base. Preserve its asynchronous chunk streaming, lighting, meshing and wgpu renderer while bringing the gameplay and data layers back to **vanilla Minecraft Java 1.20.1**.

The first playable target is single-player vanilla 1.20.1. Forge 1.20.1 / 47.4.22 and Java mods are explicitly later work. Their future compatibility bridge remains a long-term design goal, but no Forge/JVM work is in the current milestone. Keep the old `main` branch and its signing identity untouched until the native 1.20.1 client is proven.

## Pinned upstream source

- Repository: https://github.com/starling-client/starling
- Branch: `starling`
- Exact commit: `5432370d573642041585c69b40fb045b27133488`
- Release noted in the inspected README: Starling 0.8.0
- The original Starling code is MIT-licensed. Third-party dependencies keep their own licenses; preserve `LICENSE` and `THIRD_PARTY_NOTICES.md`.

RustChunk pins this exact source as the `upstream/starling` Git submodule. The submodule workflow [passed](https://github.com/imamirhosein/RustChunk/actions/runs/43889253288). The pin lets us audit upstream updates rather than silently inheriting a moving branch.

## What Starling actually contains

Starling is not a Java launcher. Its Minecraft client/gameplay implementation runs natively in Rust; the Android app enters a Rust NativeActivity library. Java code is used for Android integration and temporary resource preparation, not as a JVM running the Minecraft game.

| Upstream area | Responsibility |
|---|---|
| `crates/horizon-client` | Game loop, player, gameplay and single-player |
| `crates/horizon-world` | World state, blocks, entities, chunks and world generation |
| `crates/horizon-client-render` | Chunk streaming, lighting, meshing, wgpu rendering |
| `crates/horizon-protocol` | Network protocol and packet codecs |
| `crates/horizon-client-input` | Input |
| `crates/horizon-threads` | Worker/job pool |
| `crates/horizon-android`, `packaging/android` | Android activity, lifecycle, controls and packaging |

### Why its chunk system is worth preserving

Inspection of `crates/horizon-client-render/src/streaming.rs` confirms that chunk loading is a worker-based pipeline. It queues saved-chunk reads, terrain generation, 3×3 decoration regions, chunk finishing, lighting, meshing and mesh freeing. Jobs are prioritized by distance from the player, and their queue priority is recalculated when the player moves. Workers use immutable snapshots so the game/render thread does not do all the expensive work itself.

The game loop also limits chunk-mesh uploads per frame, with a larger budget while the view is initially arriving. Network chunk decoding happens away from the frame loop, and overdue chunk packets get a bounded per-frame budget on Android. This is a meaningful architecture advantage; preserve it unless measurements show a specific defect.

## The version gap is substantial

The pinned Starling engine targets Minecraft **26.3**, not 1.20.1.

| Fact | Minecraft 1.20.1 | Starling baseline 26.3 |
|---|---:|---:|
| Protocol | 763 | 777 |
| World/data version | 3465 | 5023 |
| Java major for the game | 17 | 25 |
| Resource pack format | 15 | 97.1 |
| Data pack format | 15 | 121.0 |

These affect real behavior: packet registration, registries, blocks/items, save format, resource/data formats, world generation, entity behavior and commands. Opening an old save in a newer engine is not the same as running the old game's rules. Changing only version constants would be incorrect.

## Verified work completed

### 1. Official 1.20.1 inputs verified and remapped

The current preparation tool resolves the official 1.20.1 version JSON, then checks the client/server JAR and client/server ProGuard mapping SHA-1 values against the separate reviewed target lock. It also checks download sizes from Mojang's metadata, extracts the versioned server JAR and verifies the embedded SHA-256, then remaps client and server using SpecialSource 1.11.6 (BSD-3-Clause). SpecialSource reads Mojang ProGuard mappings directly; no intermediate mapping converter is needed.

The remapped JARs expose the expected named entry points:
- Client: `net/minecraft/client/Minecraft.class`
- Server: `net/minecraft/server/MinecraftServer.class`

The reviewed 1.20.1 target lock is stored separately in `tools/minecraft_1_20_1_lock.json`. Preparation and decompilation reject any mismatch against its four primary input hashes. The pinned values are:
- Client JAR SHA-1: `0c3ec587af28e5a785c0b4a7b8a30f9a8f78f838`
- Client mappings SHA-1: `6c48521eed01fe2e8ecdadbd5ae348415f3c47da`
- Server bundler SHA-1: `84194a2f286ef7c14ed7ce0090dba59902951553`
- Server mappings SHA-1: `0b4dba049482496c507b2387a73a913230ebbd76`

### 2. Minecraft 1.20.1 data generation and bytecode inventory passed

Using Java 17, the official 1.20.1 server data generator produced five JSON reports in the private runner workspace (about 9.18 MB). The report contains:
- 1,003 block definitions and 24,135 block states
- 66 registries and 5,067 registry entries
- 1,663 command-tree nodes
- Overworld and Nether biome parameter reports

The remapped 1.20.1 client/server code was indexed with Starling's own `tools/class_index.py`: **7,436 classes and 58,542 methods** (7,103 `net/*`, 333 `com/*`).

The 1.20.1 data generator does **not** emit `packets.json` or the modern item-component reports. The comparison therefore records those as absent, not as empty data. A 1.20.1-specific packet exporter is still needed for multiplayer; the old item representation must be modelled from 1.20.1 itself.

### 3. A version-specific block-registry exporter is now part of the pipeline

Starling's Rust `BlockRegistry` consumes an ordered `facts.json` shape: each row carries the block identifier, air and fluid flags, a display descriptor and the full block-state descriptor; the file also carries each block's default state ID. RustChunk now has a separate Java 17 exporter for Minecraft 1.20.1 that reads the verified remapped official game classes and writes this schema privately. It does not reuse Starling's Java 25 / 26.3 exporter. The workflow checks the state descriptors, unique state identities, default-state references and the locked counts (1,003 blocks / 24,135 states). Only a compact summary and hash are uploaded; the full facts file remains in ignored `.local`.

The exporter compiled and executed successfully. Its verified output summary was:
- 1,003 blocks and 24,135 block states
- 3 air states, 8,365 fluid-containing states, 23,774 states with properties
- 6,203,510-byte private facts file; SHA-256 `c0258fa8719e754cd67b46509a0c4c310d84677e99f468f17bef0d6a195c6edf`

The follow-up workflow [#43906779498](https://github.com/imamirhosein/RustChunk/actions/runs/43906779498) passed end-to-end: pinned input verification, comparison, private decompilation, exporter execution, all Python unit tests, and the real Starling Rust `BlockRegistry` test. That test loaded the private 1.20.1 facts file through `BlockRegistry::from_facts_json_with_flags` and passed the existing tree-built registry round-trip check.

This confirms that the 1.20.1 block-state export matches the data contract consumed by Starling's Rust registry. It does **not** yet mean Starling's game loop, world generator, chunk storage, items, or renderer have been ported to 1.20.1.

This is the first version-specific data bridge into a concrete Starling engine subsystem, not yet a full engine integration.

## Pinned Starling Android build baseline

The new RustChunk baseline workflow [#43905001244](https://github.com/imamirhosein/RustChunk/actions/runs/43905001244) completed successfully. It built the exact pinned Starling 0.8.0 source for ARM64 and passed the package and native-library checks:
- Android package: `io.github.juanluci.starling.preview`
- Native library: `lib/arm64-v8a/libhorizon_android.so`
- APK size: 34,608,516 bytes
- APK SHA-256: `b293d3eff23611e339509a9d34798d4eafe28911c19d4881f3a4d6ccb60e5ad1`

This proves the pinned Starling source can be built and packaged by RustChunk CI for ARM64. It is the upstream Starling 26.3 Preview APK, **not** a RustChunk 1.20.1 client; it has not been validated on the Galaxy A22 and is not a test APK for the target version.

### 4. Automated 1.20.1 ↔ 26.3 comparison passed

The latest branch run [completed successfully](https://github.com/imamirhosein/RustChunk/actions/runs/43891766193), including unit tests, remapping, data generation, class indexing, baseline comparison and private decompilation. Its artifact contains only metadata reports, not Minecraft JARs, mappings, game data or decompiled source.

Mojang's current 26.3 version JSON had been republished outside the lock's listed JSON hashes, so the comparison tool accepted it only after verifying that both actual game JAR SHA-1 values still match the pinned Starling lock. The report marks this as a metadata republish with identical game JARs.

#### Code differences

| Measure | Result |
|---|---:|
| Classes in 1.20.1 | 7,436 |
| Classes in 26.3 | 11,383 |
| Added in 26.3 | 5,355 |
| Removed since 1.20.1 | 1,408 |
| Moved | 581 |
| Changed | 4,842 |
| Changed methods | 50,674 |

These are class-index comparison counts, not a claim that every changed class needs a manual rewrite. They show that the version gap is broad and needs a dependency-aware backport.

#### Data and registry differences

| Measure | 1.20.1 | 26.3 |
|---|---:|---:|
| Block definitions | 1,003 | 1,286 |
| Block states | 24,135 | 35,723 |
| Registries | 66 | 95 |
| Registry entries | 5,067 | 7,053 |
| Command-tree nodes | 1,663 | 2,474 |
| JSON reports | 5 | 1,666 |
| `packets.json` | absent | present |
| Item-component report files | 0 | 1,658 |

#### Resource and world-generation files

| Resource group | 1.20.1 files | 26.3 files | Added in 26.3 | Removed since 1.20.1 | Changed CRC |
|---|---:|---:|---:|---:|---:|
| Assets | 7,590 | 11,603 | 5,235 | 1,222 | 2,559 |
| Data | 5,887 | 9,883 | 9,045 | 5,049 | 378 |
| World-generation data | 844 | 1,144 | 502 | 202 | 334 |

#### 5. Private decompilation completed

The pinned Vineflower 1.12.0 workflow successfully decompiled both verified, remapped JARs using Java 17:
- Client: 4,786 Java source files, 22,360,918 bytes.
- Server: 3,568 Java source files, 16,889,671 bytes.
- The decompiler used four threads and a 4 GB heap.

**Privacy and reproducibility:** these files stayed in the ignored private `.local` workspace on the GitHub runner and were not committed or uploaded. The runner workspace is ephemeral; a future job that needs the actual sources must repeat the verified preparation/decompile steps. The small workflow artifact contains metadata only.

## Architecture findings that affect implementation

1. **The submodule is source, not integration.** RustChunk's current `engine` workspace is still the former JNI launcher core; Starling lives at `upstream/starling` and is not yet a member of the RustChunk Cargo workspace. The pinned source has a separate Android build entry point in `crates/horizon-android`.
2. **The renderer is reusable only behind version-correct data.** The chunk stream, worker pool, wgpu rendering, meshing and upload scheduling are promising to preserve. Their block-state IDs, models, lighting inputs, biome data and chunk contracts must match 1.20.1.
3. **The preparation pipeline is version-sensitive.** Starling's current `research-lock.json` pins 26.3/Java 25. Its Java exporters are compiled against that game's libraries. A distinct 1.20.1 preparation/exporter profile is required; do not feed 1.20.1 data into an unmodified 26.3 resource pipeline.
4. **The version-port script is forward-facing.** Upstream `tools/version_port.py` documents a new-release flow from the currently pinned game to a newer candidate. The RustChunk task is a backport from 26.3 to 1.20.1; reuse tools selectively, but do not assume the forward-port worktree/regeneration sequence handles a downgrade.
5. **Single-player comes before remote protocol.** Local play is the best vertical slice for world, game simulation, player, renderer and save behavior. Protocol 763 and 1.20.1 packet bodies are needed for remote servers later, but not to prove a local world can run.

See [STARLING_1_20_1_ENGINE_MAP.md](STARLING_1_20_1_ENGINE_MAP.md) for the module-by-module implementation map and gates.

## Next engineering steps

1. **E0 — Reproducible Android base build: passed.** The exact pinned Starling source built and packaged as an ARM64 Preview APK in RustChunk CI. This verifies the Android toolchain and source integration only; the resulting APK is still Starling 26.3, not 1.20.1.
2. **E1 — 1.20.1 preparation and block-registry bridge: first gate passed.** Official inputs are pinned; the 1.20.1 exporter emits the exact ordered facts schema; Starling's real Rust `BlockRegistry` accepts it in CI. Other version-specific data (block behavior, biomes, resources and world generation) is still outstanding.
3. **E2 — Core world model: not started.** Backport version facts, chunk format/DataVersion 3465, block behavior, legacy items and registry consumers, with tests against the Java 1.20.1 oracle.
4. **E3 — Local single-player vertical slice: not started.** Create a deterministic flat/test world, spawn, move, break/place blocks, save and reopen. Expand into normal Overworld terrain, lighting, entities, structures and gameplay after the small world passes.
5. **E4 — Android and performance: not started.** First prove the desktop/headless 1.20.1 slice; then build the actual RustChunk ARM64 client and test on the Galaxy A22, including frame timing, memory, temperature and high-speed chunk arrival.
6. **E5 — Later networking/mods: deferred.** Only after Vanilla works, add protocol 763 for multiplayer; the optional Java 17/Forge bridge remains a later long-term objective. No Forge work is scheduled in the current milestone.

No native RustChunk 1.20.1 APK is ready to install. A successful research or baseline build workflow is not proof of a playable 1.20.1 port.

## Later: Forge

Forge 1.20.1 / 47.4.22 remains a long-term compatibility objective, not a current task. Existing Forge JARs contain Java bytecode and expect exact Minecraft/Forge classes, ModLauncher, transformers and often Mixin/coremod behavior. Starling's `.hmod` and `.hnmod` formats are not drop-in Forge compatibility. Keep the future bridge boundary in mind, but do not let it delay the native Vanilla engine.

## References

- Starling source at the pinned commit: https://github.com/starling-client/starling/tree/5432370d573642041585c69b40fb045b27133488
- Starling's version-port design: https://github.com/starling-client/starling/blob/starling/docs/version-ports.md
- Starling's pinned version lock: https://github.com/starling-client/starling/blob/starling/research-lock.json
- Chunk streaming implementation: https://github.com/starling-client/starling/blob/starling/crates/horizon-client-render/src/streaming.rs
- 1.20.1 Mojang metadata: https://github.com/sya-ri/minecraft-skills/blob/main/packages/data/data/java/version-details/1.20.1.json
- SpecialSource 1.11.6: https://github.com/md-5/SpecialSource/tree/SpecialSource-1.11.6
