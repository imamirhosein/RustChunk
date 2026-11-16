# RustChunk

RustChunk is being redirected from the former Java/JNI launcher toward a native Rust Minecraft client, using Starling/Horizon as the engine base.

## Active direction

The active development branch is `starling-1.20.1-port-audit`. The original `main` branch remains preserved as the Stage 21 JNI/Forge launch-plan prototype; it is not the native game engine.

The upstream Starling source is pinned as a Git submodule at `upstream/starling`. Project-specific engine edits are kept as an ordered patch series under `patches/starling/` and applied by CI to that exact pin. Its source commit and license notes are documented in [docs/STARLING_UPSTREAM.md](docs/STARLING_UPSTREAM.md).

## Target order

1. Reproduce and verify the pinned Starling ARM64 Android build.
2. Port the native game rules and data from Minecraft 26.3 back to **vanilla 1.20.1**.
3. Prove local single-player world creation, movement, block interaction and save/reopen.
4. Build and test the playable ARM64 Android client on the Samsung Galaxy A22; measure chunk arrival, frame pacing, RAM and heat.
5. Only after Vanilla 1.20.1 works, investigate protocol 763 and the long-term Forge/Java compatibility bridge.

Forge/JAR mod support is not implemented in the current engine. Starling's `.hmod` and `.hnmod` formats are not drop-in replacements for Java Forge mods. The future bridge is a long-term goal, not current work.

## Current verified progress

- The official Mojang 1.20.1 client/server inputs and client/server ProGuard mappings are checksum-verified against the separate reviewed pins in `tools/minecraft_1_20_1_lock.json`.
- SpecialSource 1.11.6 remapped both JARs successfully.
- The remapped JARs contain the expected named client/server entry classes: `net.minecraft.client.Minecraft` and `net.minecraft.server.MinecraftServer`.
- The smoke-test report counted 7,436 client classes and 5,460 server classes.
- Starling's class-index tool successfully indexed 7,436 classes and 58,542 methods for 1.20.1.
- The 1.20.1 data generator produced 1,003 block entries, 66 registries, the command tree and two biome-parameter reports.
- The automated comparison against Starling 26.3 passed: 5,355 classes exist only in 26.3, 1,408 are absent there, 581 moved and 4,842 changed; 50,674 methods differ.
- The verified client and server were privately decompiled: 4,786 client and 3,568 server Java source files. These sources remain private and are not included in the workflow artifact.
- A dedicated Java 17 exporter has successfully generated the ordered 1.20.1 block-state registry in the exact `facts.json` schema consumed by Starling's `BlockRegistry`: 1,003 blocks and 24,135 states. Only counts and a SHA-256 summary are published, never the complete registry file.
- The private 1.20.1 facts file passed Starling's actual Rust `BlockRegistry` test in CI run [#43906779498](https://github.com/imamirhosein/RustChunk/actions/runs/43906779498).
- The 1.20.1 block-tag exporter completed successfully: it verified the client/server resources match and exported 170 tag files / 1,668 entries in the `data/minecraft/tags/blocks/` layout. Only the compact report was uploaded; the complete bundle stays private.
- The actual Rust `TagDefinitions` implementation passed a new integration test against the private 1.20.1 block facts and all 170 exported tags: no unresolved required references; all tags built (CI run [#43908914224](https://github.com/imamirhosein/RustChunk/actions/runs/43908914224)). This validates tag parsing/resolution, not yet the entire renderer/resource loader.
- A compatibility check found that all three block-tag IDs currently required by Starling's 26.3 renderer are absent from vanilla 1.20.1. We are not fabricating empty tags; the next step is to export the matching 1.20.1 per-state behavior from official Java and teach the Rust loader to consume it accurately.
- The pinned Starling ARM64 Android baseline also built and passed package/ABI verification in CI run [#43905001244](https://github.com/imamirhosein/RustChunk/actions/runs/43905001244). This artifact is Starling 26.3 Preview, not the 1.20.1 game.
- The Starling architecture and target version-sensitive subsystems have been mapped in [the engineering map](docs/STARLING_1_20_1_ENGINE_MAP.md).
- Preparation and decompilation now reject any primary 1.20.1 JAR or mapping whose SHA-1 differs from the reviewed target lock.

The new [ARM64 baseline workflow](.github/workflows/starling-native-baseline.yml) builds the exact pinned Starling source as a CI-only preview artifact. That proves the base engine build pipeline when it passes; it is still Minecraft 26.3 and **not** a RustChunk 1.20.1 APK.

**No native RustChunk Minecraft 1.20.1 APK is ready to install.** The successful baseline APK is only a pinned Starling 26.3 build and is not the target game. See [the port audit](docs/STARLING_1_20_1_PORT_AUDIT.md) for the current technical status.

## Previous JNI launcher

The previous Stage 1–21 launcher implementation is preserved for reference in [docs/LEGACY_JNI_LAUNCHER.md](docs/LEGACY_JNI_LAUNCHER.md). It prepared a Java/Forge launch plan but did not provide a native Minecraft engine or launch Forge on Android.


## CI runtime

The ordinary 1.20.1 preparation workflow is intentionally lightweight: verify/cache official 1.20.1 inputs, export only the data needed for the current port tasks, and run the real Rust compatibility tests. It no longer decompiles thousands of Java source files or repeats the broad 26.3 comparison on every small exporter/test change.

The complete class/version comparison and private Vineflower decompilation are available on demand through [the manual deep-audit workflow](.github/workflows/starling-deep-audit.yml). The upstream 26.3 Android baseline is also manual; it is not rebuilt just because a test patch changed.
