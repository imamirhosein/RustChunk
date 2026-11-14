# RustChunk

An independent Android-native Rust launcher and game-engine research project targeting Minecraft Java 1.20.1 and incremental Forge 47.x compatibility.

## Current milestone: Stage 3 — Resumable base-game downloads

Stage 3 adds an actual downloader for the saved Mojang installation plan.

The Android UI can now:

1. Download Minecraft 1.20.1's client JAR, applicable library artifacts, and asset objects.
2. Run four simultaneous transfers to balance speed and mobile-device resource use.
3. Resume interrupted transfers from .part files when the HTTP server supports byte ranges.
4. Validate every completed file against its expected SHA-1 before moving it into its final path.
5. Skip files already present and verified, including valid completed partial files.
6. Show transfer progress, completed file count, total bytes, and the current path.
7. Request cancellation while preserving partial files for a later resume.

Downloads are stored in app-private storage under:

files/minecraft/

The 1.20.1 installation plan must first be generated using the Resolve Minecraft 1.20.1 metadata button. The downloader deduplicates entries that resolve to the same content-addressed path.

## What Stage 3 does not do

This stage downloads the base files listed in Mojang's Minecraft 1.20.1 metadata. It does not install Forge, download or configure a Java runtime, extract platform natives, load Forge mods, or launch the game. Forge 47.x / Aternos / the user's modpack remain future compatibility milestones and must be tested independently.

## Platform target

- Android ARM64 (arm64-v8a)
- Java 17 runtime target
- Minecraft Java 1.20.1
- Forge 47.x compatibility target, implemented incrementally
- App-private storage under the RustChunk application directory

## APK updates

GitHub Actions caches the Android debug signing key. Subsequent APK builds should be installable as updates without uninstalling the app. This debug key is for test builds only, not production releases.

## Build and tests

Push to main or run RustChunk Android Build manually from GitHub Actions. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, packages the debug APK, and uploads it as a workflow artifact.
