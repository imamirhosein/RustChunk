# RustChunk — Starling 1.20.1 Port Audit

Date: 2026-11-16

Status: source-level audit completed; no 1.20.1 port or APK has been built yet.

## Decision

Use **Starling as the engine base**, not the old RustChunk Java/JNI launcher. Keep the successful Starling chunk-streaming and Vulkan renderer architecture intact while bringing the game simulation and version data back to **Minecraft Java 1.20.1**.

The first deliverable is **vanilla 1.20.1 single-player**, without Forge or mods. Forge 1.20.1 / 47.4.22 remains a later compatibility target. Do not replace the protected `main` branch or the existing signing identity during this research and porting phase.

## Exact upstream snapshot inspected

- Repository: https://github.com/starling-client/starling
- Branch: `starling`
- Commit inspected: `5432370d573642041585c69b40fb045b27133488`
- Current upstream README identifies Starling 0.8.0 as the Android release.
- Root license: MIT for Starling's original work. Third-party components retain their own licenses; preserve `LICENSE` and `THIRD_PARTY_NOTICES.md` and review each third-party component before copying or changing it.
- The repository tree at this commit contains 5,749 entries, including a substantial Rust workspace, Android packaging, tests, parity tooling, documentation, and third-party components.

## What Starling actually is

Starling is not a Java launcher or a renderer wrapped around the original Java game. It is the Android client for Horizon: the Minecraft Java game logic has been reimplemented in Rust. Its Android app starts a Rust `NativeActivity` library; it does not run the Minecraft Java client in a JVM.

Important workspace areas:

| Area | Responsibility |
|---|---|
| `crates/horizon-client` | Game loop, player, single-player and client-side gameplay |
| `crates/horizon-world` | World state, blocks, entities, chunk/world simulation and generation |
| `crates/horizon-client-render` | Chunk streaming, lighting, meshing, Vulkan/GPU rendering, HUD |
| `crates/horizon-protocol` | Multiplayer protocol and packet codecs |
| `crates/horizon-client-input` | Input handling |
| `crates/horizon-threads` | Shared worker/job pool |
| `crates/horizon-android` and `packaging/android` | Android entry points, activities, controls, packaging and Java-to-Android glue |

The renderer uses `wgpu`, with Vulkan enabled for Android and GLES available as a fallback. The Android library is loaded through NativeActivity/winit and JNI hooks; those are Android integration hooks, not a JVM running Minecraft.

## Why its chunk performance is worth preserving

The source confirms a real asynchronous chunk pipeline in `crates/horizon-client-render/src/streaming.rs`:

- Jobs include loading saved chunks, generating chunks, decorating 3×3 regions, finishing chunks, lighting, meshing and freeing completed meshes.
- Work is queued by priority. The nearest columns are processed first.
- When the player's centre or loading band changes, queued priorities are recalculated. This avoids a large batch of old distant work blocking chunks newly needed around the player.
- Workers operate on immutable snapshots rather than forcing the game/render thread to perform all generation and meshing itself.
- The game loop bounds mesh uploads per frame; it uses a larger upload allowance while the view is still arriving, rather than allowing an unlimited burst to stall a frame.
- Network chunk decoding is also off the frame. On Android, overdue chunk packets have a bounded per-frame budget.

This is a strong architecture to retain. We should not rewrite the streamer just to port the game version. First bring the correct 1.20.1 game data and behavior into the engine, then measure the phone and tune only where the evidence shows a bottleneck.

## The main version mismatch

The inspected Starling source is pinned to **Minecraft 26.3**, not 1.20.1:

- `research-lock.json` pins 26.3 and Java 25.
- `crates/horizon-core/src/version.rs` reports protocol 777, data version 5023 and Java 25.
- `crates/horizon-protocol/src/ids.rs` is generated for protocol 777.
- The Android setup prepares the resources for the pinned 26.3 game.

For Minecraft Java 1.20.1, the target facts are:

| Fact | 1.20.1 target | Current Starling |
|---|---:|---:|
| Protocol | 763 | 777 |
| World/data version | 3465 | 5023 |
| Resource pack format | 15 | 97.1 |
| Data pack format | 15 | 121.0 |
| Java major used by that game | 17 | 25 |

These are not cosmetic labels. They affect packet IDs and codecs, save metadata, registries, data/resource formats, game rules and preparation tools. Merely changing the version string or protocol constant would not create a 1.20.1 client.

Starling can read and upgrade many older world-save formats, but that does **not** mean it runs the old game's rules. An older save opened by a 26.3 engine is upgraded and then played as 26.3. RustChunk must target 1.20.1 behavior and data, not just accept an old save.

## First engineering blocker: Mojang mappings

The existing version-port tooling is valuable, but it explicitly supports unobfuscated releases (26.1 and later):

- `tools/version_port.py prepare_download()` stops if the version manifest contains `client_mappings`.
- `tools/lab.py prepare()` independently stops if either `client_mappings` or `server_mappings` is present, with the message that older releases need a remapping stage.
- Minecraft 1.20.1 is an older obfuscated release and its version metadata provides client and server obfuscation mappings.

Therefore, **the first code change should not be inside the renderer or game loop**. It should extend the research/preparation pipeline to:
1. Fetch 1.20.1's official version JSON and verify the manifest and JAR SHA-1 values.
2. Fetch and verify the matching client and server mapping files.
3. Remap the client and server game JARs into a consistent named form for analysis and Java parity oracles. Keep all game JARs, mappings and decompiled sources in a private/ignored working area; never commit or package them.
4. Run the 1.20.1 server data generator and create the version facts, registry reports, packet reports and class inventories.
5. Produce a machine-readable 26.3 ↔ 1.20.1 difference report before changing gameplay code.

The current toolchain already has a version-port pipeline that compares protocol facts, registries, blocks, item components, commands, data packs, assets and Java class/method inventories. It creates a candidate worktree and can run parity harnesses. That pipeline is designed for newer versions; it needs the mapping/remapping stage and older-JDK handling before it can be used for 1.20.1.

## Second engineering blocker: Java 17 preparation

The game engine itself is Rust and does not need a JVM during play. However, Starling currently uses Java 25 for its 26.3 preparation/export pipeline, and the Android build packages a Java launcher plus the exporter bundle.

Minecraft 1.20.1 uses Java 17. We must audit and retarget the preparation/exporter pipeline to Java 17, including the Android preparation runtime and exporter compatibility. The Rust compiler/toolchain is a separate requirement: the upstream workspace uses Rust 2024 and current crates require Rust 1.90. Do not confuse the Java game version with the Rust compiler version.

## Other version-sensitive systems to audit

1. **Game logic:** newer blocks, items, mobs, commands, components, effects and gameplay behavior must be reconciled against the 1.20.1 oracle. In particular, the current code contains later-version item/component and registry assumptions.
2. **World generation:** use 1.20.1's own generator data, codecs, carvers, features, structures, biomes, dimensions and seeds. Do not reuse 26.3 worldgen JSON unchanged.
3. **Save format:** write 1.20.1's data version and NBT/chunk/entity structure. Validate new-world save/reopen and region files.
4. **Protocol:** regenerate protocol 763 packet IDs and update codecs/registries before claiming 1.20.1 server compatibility.
5. **Resources and UI:** use the 1.20.1 assets, sounds, models, blockstates, translations, language and pack formats.
6. **Android:** preserve touch controls, storage, lifecycle and diagnostics, but verify the renderer and memory policy on the actual Galaxy A22 / Mali-G52 device.

There is one documentation discrepancy to resolve during the port: `docs/horizon-parity.md` describes a complete 26.3 world-generation parity audit, while the opening lines of `docs/world-generation.md` still say complete terrain generation is pending. Treat the audit and its harness results as evidence to recheck at the pinned commit, not as an unconditional guarantee.

## Build environment and device implications

The current Android build instructions use Rust 1.90+, cargo-ndk, Android NDK r28c, Android platform 36/build-tools 35, JDK 25 for the current exporter/build setup and Python 3.10+. The Android app supports ARM64, and its README says it has also run on some Mali-G52 devices. That is encouraging for the Galaxy A22, but the user's own A22 test remains the deciding test.

The user's Windows 7 PC does not need to compile the project. The port and APK can be built by GitHub Actions once the source and workflow are prepared. A successful CI APK build alone is not a pass: the phone must boot it, create and reopen a 1.20.1 world, and pass the chunk-flight test.

## M1 groundwork: official 1.20.1 mappings

The 1.20.1 version metadata contains separate client and server mapping files. The current upstream pipeline rejects these fields, so the backport needs a remapping stage before decompilation and class indexing.

The verified 1.20.1 metadata values are:

| Input | SHA-1 |
|---|---|
| Client JAR | `0c3ec587af28e5a785c0b4a7b8a30f9a8f78f838` |
| Client mappings | `6c48521eed01fe2e8ecdadbd5ae348415f3c47da` |
| Server bundler JAR | `84194a2f286ef7c14ed7ce0090dba59902951553` |
| Server mappings | `0b4dba049482496c507b2387a73a913230ebbd76` |

The mappings are ProGuard format. An important correction from source inspection: **SpecialSource 1.11.6 can parse ProGuard mappings directly** through its own `JarMapping.parseProguardLine` path. We do not need an intermediate ProGuard-to-SRG converter. The earlier converter prototype was discarded so we do not keep an unnecessary translation layer.

SpecialSource 1.11.6 is published on Maven Central under BSD-3-Clause. The preparation script resolves the tool and its declared dependencies through Maven, then runs its main class with the official Mojang mappings.

Current groundwork in this branch:

- `tools/prepare_minecraft_1_20_1.py` resolves version 1.20.1 through Mojang's official version manifest; verifies the manifest JSON, client/server JARs and both mapping files against Mojang's SHA-1 and size metadata; extracts the bundled server game JAR and verifies its embedded SHA-256; remaps client and server JARs with SpecialSource; and checks for the expected named Minecraft entry classes.
- `tests/test_prepare_minecraft_1_20_1.py` tests server-bundle parsing and hash validation, version-fact validation and file hashing without downloading game files.
- `.github/workflows/starling-port-tools.yml` runs the unit tests on changes. Its manual `workflow_dispatch` path performs the real official download/remap smoke test and exports only a metadata report. The Minecraft JARs, mappings and remapped JARs remain in the runner's ignored working directory and are not uploaded as artifacts.

**Not yet proven:** the script has not yet completed its manual GitHub Actions run against the full Mojang 1.20.1 files. Until that run succeeds, remapping is an implementation attempt, not a validated result. A successful remap also does not mean Horizon has been ported to 1.20.1; it only unlocks the next analysis stage.

Sources:
- Mojang version metadata values for 1.20.1: https://github.com/sya-ri/minecraft-skills/blob/main/packages/data/data/java/version-details/1.20.1.json
- SpecialSource 1.11.6 release source: https://github.com/md-5/SpecialSource/tree/SpecialSource-1.11.6
- SpecialSource 1.11.6 Maven Central metadata and BSD-3-Clause license: https://central.sonatype.com/artifact/net.md-5/SpecialSource/1.11.6

## Ordered RustChunk milestones

### M0 — Freeze a known-good engine baseline
Record the exact Starling commit and release, keep upstream attribution and notices, and make sure an unchanged ARM64 build can be reproduced. No version changes yet.

### M1 — Make the 1.20.1 research pipeline work
Add official-mapping download/verification/remapping and Java 17 support. Produce the verified 1.20.1 facts, generated reports, and class/method/registry/packet diff. This is the next concrete engineering milestone.

### M2 — Backport vanilla 1.20.1 behavior
Regenerate version constants, packet IDs, registry/data tables and assets. Use a 1.20.1 Java oracle and parity tests to backport the engine subsystems in dependency order. Keep the asynchronous chunk streaming and GPU renderer unless tests show they need changes.

### M3 — First playable 1.20.1 Android build
On the Galaxy A22, prove: app startup, title menu, new single-player world, world generation, chunk rendering, block interaction, save, reopen, and high-speed movement/elytra without the visible world falling behind. No Forge or mods in this milestone.

### M4 — Multiplayer 1.20.1
Verify the 763 protocol, login mode, registries and chunk packets against a real matching vanilla 1.20.1 server. Offline identity is only for single-player or servers that explicitly allow offline-mode authentication.

### M5 — Forge compatibility research
Only after M3 succeeds, investigate Forge 1.20.1 / 47.4.22 separately. Forge JARs are Java code; the current Starling `.hmod` and `.hnmod` formats are not drop-in Forge support. Do not advertise the user's Forge mods as supported until a real bridge and representative mod tests pass.

## Acceptance rule

Do not call RustChunk "Minecraft 1.20.1" until the single-player game logic, save format, assets, world generation and target data are actually 1.20.1. Do not call it Forge-compatible until Forge 47.4.22 starts and the user's real mod set passes staged tests.

## References

- Starling source at the inspected commit: https://github.com/starling-client/starling/tree/5432370d573642041585c69b40fb045b27133488
- Starling version-port design: https://github.com/starling-client/starling/blob/starling/docs/version-ports.md
- Current version lock: https://github.com/starling-client/starling/blob/starling/research-lock.json
- Current version constants: https://github.com/starling-client/starling/blob/starling/crates/horizon-core/src/version.rs
- Current packet IDs: https://github.com/starling-client/starling/blob/starling/crates/horizon-protocol/src/ids.rs
- Chunk streaming implementation: https://github.com/starling-client/starling/blob/starling/crates/horizon-client-render/src/streaming.rs
- Android build instructions: https://github.com/starling-client/starling/blob/starling/README.md#build-from-source
- Mojang announcement on removing obfuscation: https://www.minecraft.net/en-us/article/removing-obfuscation-in-java-edition
- 1.20.1 facts: https://minecraft.wiki/w/Java_Edition_1.20.1
- Forge 1.20.1 / 47.4.22 release listing: https://files.minecraftforge.net/net/minecraftforge/forge/index_1.20.1.html
- Forge mod file metadata documentation: https://docs.minecraftforge.net/en/1.20.1/gettingstarted/modfiles/
