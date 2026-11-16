# RustChunk

RustChunk is being redirected from the former Java/JNI launcher toward a native Rust Minecraft client, using Starling/Horizon as the engine base.

## Active direction

The active development branch is `starling-1.20.1-port-audit`. The original `main` branch remains preserved as the Stage 21 JNI/Forge launch-plan prototype; it is not the native game engine.

The upstream Starling source is pinned as a Git submodule at `upstream/starling`. Its exact commit and license notes are documented in [docs/STARLING_UPSTREAM.md](docs/STARLING_UPSTREAM.md).

## Target order

1. Reproduce and understand the Starling engine architecture.
2. Port the native game rules/data from Minecraft 26.3 back to **vanilla 1.20.1**.
3. Build and test a playable ARM64 Android client on the Samsung Galaxy A22.
4. Validate world creation, saving/reopening, chunk streaming and high-speed elytra flight.
5. Only after vanilla 1.20.1 works, investigate Forge 1.20.1 compatibility.

Forge/JAR mod support is not implemented in the current engine. Starling's `.hmod` and `.hnmod` formats are not drop-in replacements for Java Forge mods.

## Current verified progress

- The official Mojang 1.20.1 client/server inputs and client/server ProGuard mappings were downloaded and checksum-verified by GitHub Actions.
- SpecialSource 1.11.6 remapped both JARs successfully.
- The remapped JARs contain the expected named client/server entry classes: `net.minecraft.client.Minecraft` and `net.minecraft.server.MinecraftServer`.
- The smoke-test report counted 7,436 client classes and 5,460 server classes.
- The next stage is running the 1.20.1 data generator and then adapting Starling's version-preparation and parity tooling.

**This is not yet a playable Minecraft 1.20.1 build. No RustChunk APK from this native port is ready to install.** See [the port audit](docs/STARLING_1_20_1_PORT_AUDIT.md) for the current technical status.

## Previous JNI launcher

The previous Stage 1–21 launcher implementation is preserved for reference in [docs/LEGACY_JNI_LAUNCHER.md](docs/LEGACY_JNI_LAUNCHER.md). It prepared a Java/Forge launch plan but did not provide a native Minecraft engine or launch Forge on Android.
