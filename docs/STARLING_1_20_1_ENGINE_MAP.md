# RustChunk: Starling → Minecraft 1.20.1 engineering map

Date: 2026-11-16

Status: source architecture inspected; 1.20.1 inputs indexed and decompiled in a private CI workspace. The actual 1.20.1 game port has not started compiling. This document is the implementation map, not a claim that the port is complete.

## Goal and scope boundary

The first deliverable is a **native Rust, single-player Vanilla Minecraft Java 1.20.1 client**. The Android target is ARM64 on the Samsung Galaxy A22 (SM-A225F, Mali-G52 MC2).

The existing Starling chunk worker pipeline and wgpu renderer are valuable and should be preserved where their data contracts permit. The first pass must not spend effort on Forge, Java mods, multiplayer protocol translation, shaders beyond baseline support, or launcher cosmetics.

The long-term Java-mod compatibility bridge remains an architectural objective, not a current milestone. Keep a clean boundary around native game systems so a future optional Java 17 worker can exchange versioned registry, content, event and world-change messages with Rust. Do not implement or carry a JVM in the first playable Vanilla milestone.

## Confirmed source and build facts

- RustChunk branch: `starling-1.20.1-port-audit`.
- Starling submodule: `upstream/starling`.
- Pinned Starling commit: `5432370d573642041585c69b40fb045b27133488` (Starling 0.8.0, Minecraft 26.3).
- Starling uses Rust for the game and a NativeActivity Android entry point. Its Android crate is `crates/horizon-android`; the client startup enters through `crates/horizon-client/src/android_lib.rs` and `android.rs`.
- The Android package is built by `packaging/android/build_apk.py`, requiring Java 25, Android SDK platform 36, build-tools 35.0.0, NDK 28.2.13676358, Rust stable for `aarch64-linux-android`, and cargo-ndk 3.5.4.
- The upstream build script produces a separately installed `Starling Preview` package when called with `--preview`. It remains a 26.3 baseline and is **not** a RustChunk 1.20.1 game.

A new RustChunk CI workflow builds this exact pinned ARM64 baseline as a reproducibility gate. Its APK is an engineering artifact only; do not install it as the 1.20.1 test build.

## Port map: reuse vs replace

| System | Starling source of truth | 1.20.1 action | First-pass priority |
|---|---|---|---|
| Android lifecycle and native entry | `crates/horizon-android`, `crates/horizon-client/src/android_lib.rs`, `android.rs` | Reuse the NativeActivity/winit/JNI boundary; later change package name, label, shared folder and app identity for RustChunk | Baseline |
| Rust game loop and single-player | `crates/horizon-client/src/entry.rs`, `game.rs`, `game/` | Keep architecture; backport 1.20.1 gameplay and integrated-server behavior against the decompiled 1.20.1 source | Critical |
| Worker pool and chunk streaming | `crates/horizon-threads`, `crates/horizon-client-render/src/streaming.rs` | Preserve queue/worker design; adapt task inputs, chunk status, generation output and prioritization only where 1.20.1 requires it | Preserve |
| Renderer, meshing and GPU upload | `crates/horizon-client-render/src/gfx.rs`, `chunk.rs`, `mesher.rs`, `light.rs` | Preserve wgpu/Vulkan path; replace version-dependent block/model/biome/atlas inputs and validate shader/API assumptions | High |
| World and chunk storage | `crates/horizon-world/src/chunk.rs`, `region.rs`, `nbt/`, `level/` | Port 1.20.1 section layout, height/build limits, palettes, block entities, ticks, DataVersion 3465 and save paths | Critical |
| Registry and block/item definitions | `crates/horizon-world/src/registry/`, `crates/horizon-items`, `crates/horizon-client-core` | Rebuild from 1.20.1 registry/data and legacy item/NBT model; do not reuse modern component tables as if they were 1.20.1 | Critical |
| World generation | `crates/horizon-world/src/worldgen/`, `crates/horizon-client/src/vanillagen.rs` | Backport 1.20.1 noise settings, density functions, biome sources, carvers, features, structures and decoration data; compare deterministically to Java 1.20.1 | Critical |
| Resource preparation/export | `crates/horizon-prepare`, `mods/horizon/src/test/java/dev/horizonlab/parity` | Add a version-specific 1.20.1 preparation profile and exporters compiled against the 1.20.1 client/server libraries | Critical |
| Network protocol | `crates/horizon-protocol` | Later replace protocol 777 / packet tables with protocol 763 and 1.20.1 packet bodies; not a prerequisite for a local single-player vertical slice | Deferred |
| Version facts | `crates/horizon-core/src/version.rs`, `tools/gen_version_consts.py` | Generate version id, data/protocol and pack-format constants from the verified 1.20.1 client jar; update dependent tests and generated tables together | Critical |
| Mods | `crates/horizon-mods`, native `.hmod/.hnmod` | No implementation now. Keep this system separate from any future Java Forge compatibility worker | Deferred |

## Important dependency findings

### 1. Version constants are centralized, but that is not the port

`tools/gen_version_consts.py` can generate `horizon-core/src/version.rs` from a selected client jar. This is useful, but changing the constants alone would leave 26.3 registries, world rules, Java exporter classes, packet tables, item definitions and world-generation data in place. A constant-only version switch must never be treated as a 1.20.1 port.

### 2. The current preparation/exporter pipeline is pinned to 26.3

`research-lock.json` identifies Minecraft 26.3 and Java 25. `horizon-prepare/src/exporters.rs` compiles the Java helper classes under `mods/horizon/src/test/java` against the selected game libraries. Its current exporter list includes block behavior, entity types, command parsing, creative tabs, structures, block-state bindings, entity models and mob data.

Those exporters are version-sensitive. They must not be assumed to compile or produce equivalent results against 1.20.1. Add a separately pinned 1.20.1 source profile and a tested exporter set; retain the official JARs and decompiled sources only in ignored private workspaces.

The 1.20.1 server data generator already verified by RustChunk emits five JSON reports, but no `packets.json` and no modern item-component reports. For the first single-player milestone, prioritize block/biome/registry and world-generation outputs. Build the 1.20.1 packet exporter later for multiplayer. Model 1.20.1 items using their actual legacy representation rather than fabricating empty modern component data.

### 3. The existing version-port automation runs forward

Starling's `tools/version_port.py` and `docs/version-ports.md` describe bringing a newer Minecraft release into a currently pinned Horizon tree (for example, current → next release). Our direction is the reverse: 26.3 → 1.20.1. Reuse its verified download, indexing, diff and report ideas, but do not run its forward-port worktree/regeneration sequence unchanged for this downgrade.

### 4. Single-player is the first useful vertical slice

A local single-player world exercises the integrated game/server simulation, chunk generation, lighting, rendering, player movement and persistence without requiring a remote 1.20.1 packet connection. Therefore the first acceptance flow should be:

1. Start the native client and reach the title screen.
2. Create a fresh 1.20.1 world with a known seed (initially a controlled flat/small test world, then normal Overworld).
3. Spawn, move and look around; no void holes or delayed ground under the player.
4. Break and place blocks; check block state and lighting updates.
5. Save, close, reopen and verify the same world.
6. Extend to normal terrain, caves, structures, mobs and high-speed elytra flight.

Protocol 763, Forge and Aternos connectivity are later gates, not shortcuts around this local test.

## Source ownership and patching rule

`upstream/starling` is a pinned Git submodule. Editing files inside a checked-out submodule does not make those changes part of the RustChunk parent commit. Until RustChunk has a maintained Starling fork, keep all project-specific changes as a reviewable patch series under `patches/starling/`. The applier `tools/apply_starling_patches.py` checks the exact base SHA, rejects dirty submodules, preflights every patch, then applies them in manifest order before CI compiles. The series is currently empty. Never silently move the submodule pointer or modify upstream source without recording the base commit and patch order.

Preserve Starling's MIT license and `THIRD_PARTY_NOTICES.md`; dependency licenses remain separate. Do not commit Mojang JARs, game assets, generated private resources or decompiled source.

## Implementation gates

| Gate | Deliverable | Pass condition |
|---|---|---|
| E0 | Reproducible pinned ARM64 Starling build | **Passed** in CI run #43905001244; exact pin, Preview package identity and ARM64 native library verified |
| E1 | 1.20.1 preparation and first registry bridge | **Block-registry gate passed** in CI run #43906779498: official 1.20.1 inputs verify, facts are exported privately, and Starling's Rust BlockRegistry test accepts the file. Other exporters remain |
| E2 | Core 1.20.1 world model | Correct registries, blocks, legacy items, chunk format and DataVersion 3465; Rust tests compare with Java oracle |
| E3 | 1.20.1 local simulation/worldgen | Known-seed chunks and gameplay probes match Java 1.20.1; save/reopen round-trip passes |
| E4 | Desktop/headless vertical slice | A local 1.20.1 world can be created, entered, modified, saved and reopened before Android debugging |
| E5 | ARM64 Android client | APK starts and completes the same flow on the target device; measure frame times, RAM, heat and chunk arrival |
| E6 | Performance acceptance | Move/fly through generated terrain at speed without visible missing terrain; compare chunk throughput and frame pacing to the baseline |
| E7 | Later compatibility work | Only after E0–E6: protocol 763 multiplayer, then a separately scoped Java/Forge compatibility bridge |

## Immediate next work

1. E0 is closed: the pinned Starling ARM64 baseline builds and its package/ABI checks pass. Do not install that baseline as a 1.20.1 test client.
2. The 1.20.1 block facts exporter, real Rust BlockRegistry test, block-tag client/server equality check, and the actual Rust TagDefinitions resolution test all pass. The tag test resolves 170 tags / 1,668 entries with no missing required references. Preserve both private bundles and the test-only Starling patch.
3. The 1.20.1 target is missing all three 26.3 renderer-required tags for heightmap motion / no-leaves motion / fluid flow. Do not synthesize empty tags. Export per-state equivalents from official 1.20.1 predicates and add a Java-oracle parity test; then adapt Rust `Assets::load` and the mesher to consume those facts.
4. Finish fluid-tag extraction and validate its references against `FluidKind`. Then continue block behavior, collision/shape, legacy item/NBT, biome and world-generation exports. Keep full generated facts and decompiled source private.
5. Begin the actual maintained engine backport in dependency order (version constants, registry/data contracts, chunk storage and worldgen), preserving the renderer/streaming pipeline where its data contract still fits.
6. Only after a desktop/headless 1.20.1 single-player world can be created, modified, saved and reopened should the ARM64 1.20.1 APK be built and tested on the Galaxy A22.

No 1.20.1 APK is ready for installation. Every milestone must be labelled by what it actually proves.
