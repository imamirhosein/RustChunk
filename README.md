# RustChunk

An independent Android-native Rust launcher and game-engine research project targeting Minecraft Java 1.20.1 and incremental Forge 47.x compatibility.

## Current milestone: Stage 5 — Persistent import report and game-data inspection

Stage 5 builds on the Zalith game-data importer and adds a report saved inside RustChunk's private Minecraft folder. The report survives closing and reopening the app and contains the import result, files copied, copied data size, detected Forge profile folders, and the destination.

The app displays the last completed import report on startup and adds an inspection action that checks the files currently present in RustChunk storage without importing them again. The inspection reports:

- Presence of the core `assets/`, `libraries/`, and `versions/` folders
- Minecraft 1.20.1 Forge profile folders and their version JSON files
- Other installed version/loader folders
- Presence of Forge libraries, mods, and config folders
- Number of JAR files currently inside `mods/`

These are file-presence checks only. They do not prove that Forge launches or that the complete modpack is compatible.

## Zalith game-data import

The Android Storage Access Framework importer accepts a folder directly containing at least two of `assets/`, `libraries/`, and `versions/`; it also detects a wrapper folder containing a `.minecraft/` child.

The importer copies reusable game data into RustChunk's private `files/minecraft/` directory, including Minecraft assets, libraries, versions, mods, configs, resource packs, shader packs, KubeJS scripts, and selected mod data folders. It deliberately does not copy launcher account databases, authentication tokens, logs, crash reports, or launcher-specific private data.

On some Android versions the folder picker cannot access another app's private `Android/data/` directory. In that case, first copy the game directory to a normal folder such as `Download/RustChunkImport/`.

## APK update signing

The Stage 4 build created and saved the stable signing key at `.ci-signing/rustchunk-debug.keystore`. Its certificate SHA-256 fingerprint is pinned in the build workflow.

Stage 5 no longer silently creates a replacement key when the GitHub Actions cache misses. It fails before building if the original key cannot be restored or if its fingerprint differs. The produced APK is also checked against the same expected certificate before upload. This prevents publishing an APK that would break in-place updates.

## Current capabilities and limitations

- Stage 2: resolves official Minecraft 1.20.1 metadata and builds a SHA-1-aware install plan.
- Stage 3: downloads base Minecraft files with four parallel workers, resume support, and SHA-1 verification.
- Stage 4: imports reusable game files from a user-selected Zalith game directory.
- Stage 5: saves the import result and inspects imported file presence after app restart.

This is not yet a runnable Forge launcher. Forge installation, Java runtime provisioning, native extraction, full Forge argument generation, mod-loader startup, and actual Aternos/modpack compatibility remain future milestones and must be tested on-device.

## Platform target

- Android ARM64 (arm64-v8a)
- Java 17 runtime target
- Minecraft Java 1.20.1
- Forge 47.x compatibility target
- App-private storage under the RustChunk application directory

## Build and tests

Push to main or run RustChunk Android Build manually from GitHub Actions. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, builds the Android APK, verifies the persistent signing key and APK certificate, and uploads the artifact.
