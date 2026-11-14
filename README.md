# RustChunk

An independent Android-native Rust launcher and game-engine research project targeting Minecraft Java 1.20.1 and incremental Forge 47.x compatibility.

## Current milestone: Stage 4 — Zalith game-data import and stable APK signing

Stage 4 adds an Android Storage Access Framework folder importer for an existing Minecraft game directory. The selected folder must directly contain at least two of `assets/`, `libraries/`, and `versions/`; a wrapper folder containing a `.minecraft/` child is also detected.

The importer copies reusable game data into RustChunk's private `files/minecraft/` directory:

- Minecraft assets, libraries, and version files
- Mods and mod configuration folders
- Resource packs, shader packs, KubeJS scripts, and supported mod data folders
- Basic game preferences and server list files

It deliberately does not copy launcher account databases, authentication tokens, logs, crash reports, or launcher-specific private data. It detects 1.20.1 Forge version folders after import when they are present.

The Android folder picker may prevent access to another app's private `Android/data/` directory on some Android versions. In that case, the source game directory must first be exported/copied to a folder that Android allows the user to select.

## APK update signing

The previous workflow attempted to cache `~/.android/debug.keystore`, but the Stage 3 build log showed that this path did not exist at cache-save time. Therefore that workflow did **not** preserve its signing identity.

Stage 4 now creates a dedicated signing key at `.ci-signing/rustchunk-debug.keystore`, caches that exact file, explicitly configures Gradle to sign debug APKs with it, and prints/verifies the certificate during the build. The key is not committed to the public repository.

The first Stage 4 APK will have a new signing identity compared with the Stage 3 APK. A one-time uninstall of the Stage 3 app is therefore required before installing Stage 4. Subsequent APKs built from this workflow should retain the same signature and install as normal updates, provided the GitHub Actions cache is retained.

## Current capabilities and limitations

- Stage 2: resolves official Minecraft 1.20.1 metadata and builds a SHA-1-aware install plan.
- Stage 3: downloads base Minecraft files with four parallel workers, resume support, and SHA-1 verification.
- Stage 4: imports reusable game files from a user-selected Zalith game directory.

This is not yet a runnable Forge launcher. Forge installation, Java runtime provisioning, native extraction, full Forge argument generation, mod-loader startup, and actual Aternos/modpack compatibility remain future milestones and must be tested on-device.

## Platform target

- Android ARM64 (arm64-v8a)
- Java 17 runtime target
- Minecraft Java 1.20.1
- Forge 47.x compatibility target
- App-private storage under the RustChunk application directory

## Build and tests

Push to main or run RustChunk Android Build manually from GitHub Actions. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, builds the Android APK, verifies APK signing, and uploads the artifact.
