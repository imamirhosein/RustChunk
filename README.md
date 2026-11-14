# RustChunk

An independent Android-native Rust launcher and game-engine research project targeting Minecraft Java 1.20.1 and incremental Forge 47.x compatibility.

## Current milestone: Stage 2 — Official metadata and installation planning

Stage 2 adds a real Android network path from the Kotlin UI into the Rust core.

When the user taps **Resolve Minecraft 1.20.1 metadata**, RustChunk:

1. Fetches Mojang's version manifest over HTTPS and selects the 1.20.1 release.
2. Fetches and SHA-1 validates the Minecraft version metadata and asset index.
3. Resolves the client JAR, Android ARM64-compatible library artifacts selected by metadata rules, and all asset object URLs.
4. Creates a JSON installation plan with paths, URLs, sizes, and SHA-1 hashes.
5. Persists the manifest, version metadata, asset index, and plan inside the app-private Minecraft directory.

The plan is saved at:

`files/minecraft/instances/forge-1.20.1/install-plan.json`

The app only fetches metadata at this stage. It does **not** download the Minecraft client JAR, library JARs, assets, Forge installer, or Java runtime, and it does not launch Minecraft yet. OS-version regular-expression rules and native classifier selection are not implemented in this stage.

## Current platform target

- Android ARM64 (`arm64-v8a`)
- Java 17 runtime target
- Minecraft Java 1.20.1
- Forge 47.x compatibility target, implemented incrementally
- App-private storage under the RustChunk application directory

## APK updates

GitHub Actions caches the Android debug signing key. The first Stage 2 APK establishes that persistent key; subsequent APK builds should be installable as updates without uninstalling the app. The Stage 1 APK may need to be uninstalled once because it was built before persistent signing was configured. This debug key is for test builds only, not production releases.

## Build and tests

Push to `main` or run **RustChunk Android Build** manually from GitHub Actions. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, packages the debug APK, and uploads it as a workflow artifact.

Stage 2 includes Rust unit tests for download-plan construction, platform library rules, SHA-1 checks, safe paths, and atomic metadata writes.
