# RustChunk: Starling engine and Minecraft 1.20.1 port audit

Date: 2026-11-16

Status: **research and input-comparison pipeline is working. No native 1.20.1 game port or APK has been built yet.**

## Decision and target

Use Starling as the native Rust engine base. Preserve its asynchronous chunk streaming, lighting, meshing and Vulkan renderer while bringing the gameplay and data layers back to **vanilla Minecraft Java 1.20.1**.

The first playable target is single-player vanilla 1.20.1. Forge 1.20.1 / 47.4.22 and mods are explicitly later work. Keep the old `main` branch and its signing identity untouched until the native 1.20.1 client is proven.

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

The game loop also limits chunk-mesh uploads per frame, with a larger budget while the view is initially arriving. Network chunk decoding happens away from the frame loop, and overdue chunk packets get a bounded per-frame budget on Android. This is a meaningful architecture advantage; we should preserve it unless measurements show a specific defect.

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

The current preparation tool downloads the official 1.20.1 version JSON, client/server JARs and Mojang's client/server ProGuard mappings. It checks SHA-1 and size values from Mojang's metadata, extracts the versioned server JAR and verifies the embedded SHA-256, then remaps client and server using SpecialSource 1.11.6 (BSD-3-Clause). SpecialSource reads Mojang ProGuard mappings directly; no intermediate mapping converter is needed.

The remapped JARs expose the expected named entry points:
- Client: `net/minecraft/client/Minecraft.class`
- Server: `net/minecraft/server/MinecraftServer.class`

The official 1.20.1 values used by the workflow are:
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

The 1.20.1 data generator does **not** emit `packets.json` or the modern item-component reports. The comparison therefore records those as absent, not as empty data. We need a 1.20.1-specific packet exporter and the older item representation; modern data components cannot be assumed.

### 3. Automated 1.20.1 ↔ 26.3 comparison passed

The GitHub Actions workflow [completed successfully](https://github.com/imamirhosein/RustChunk/actions/runs/43891125869), including unit tests, remapping, data generation, bytecode indexing and the baseline comparison. Its artifact contains only two small metadata JSON reports, not Minecraft JARs, mappings, generated game data or full class indexes:
- `minecraft-1.20.1-remap-audit`
- `minecraft-1.20.1-vs-starling-26.3-audit`

The comparison used the exact Starling-pinned client and server JAR SHA-1 values. Mojang's current 26.3 version JSON had been republished outside the lock's listed JSON hashes, so the tool accepted it only after verifying that both actual game JAR SHA-1 values still match the pinned Starling lock. The report marks this as a metadata republish with identical game JARs.

#### Code differences

Starling's class-index comparison found:

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

This confirms that reusing all 26.3 resources and generated world data unchanged would be wrong.

## Next engineering steps

1. **Decompile the verified 1.20.1 client/server JARs privately** with the pinned Vineflower version and Java 17. Decompiled game source must remain in the ignored `.local` workspace and must never be committed or uploaded.
2. **Create missing 1.20.1 exporters**, starting with packet registration and legacy item behavior; generate accurate version-specific tables from the 1.20.1 code/data.
3. **Build a port map** from each version-sensitive Java subsystem/class to Starling Rust files and existing parity tests. Prioritize protocol-independent single-player foundations, registries/blocks, chunk/world storage, world generation, entities and saves.
4. **Backport in small, testable subsystems**, with 1.20.1 as the oracle. Preserve the current chunk worker pipeline and renderer where behavior permits.
5. **Only after the 1.20.1 engine compiles and its world-generation/save tests pass**, build an ARM64 APK and test the Galaxy A22: fresh world, save/reopen, block interaction, render distance and high-speed elytra flight.

No native RustChunk 1.20.1 APK is ready to install. A successful research workflow is not proof of a playable port.

## Later: Forge

Forge 1.20.1 / 47.4.22 remains a separate milestone. Existing Forge JARs contain Java bytecode and expect the exact Minecraft/Forge classes, ModLauncher, transformers and often Mixin/coremod behavior. Starling's `.hmod` and `.hnmod` formats are not drop-in Forge compatibility. Do not advertise Forge or the user's modpack as supported until an actual compatibility runtime/bridge and staged real-mod tests pass.

## References

- Starling source at the pinned commit: https://github.com/starling-client/starling/tree/5432370d573642041585c69b40fb045b27133488
- Starling's version-port design: https://github.com/starling-client/starling/blob/starling/docs/version-ports.md
- Starling's pinned version lock: https://github.com/starling-client/starling/blob/starling/research-lock.json
- Chunk streaming implementation: https://github.com/starling-client/starling/blob/starling/crates/horizon-client-render/src/streaming.rs
- 1.20.1 Mojang metadata: https://github.com/sya-ri/minecraft-skills/blob/main/packages/data/data/java/version-details/1.20.1.json
- SpecialSource 1.11.6: https://github.com/md-5/SpecialSource/tree/SpecialSource-1.11.6
