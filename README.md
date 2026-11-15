# RustChunk

An independent Android launcher and game-engine research project targeting Minecraft Java 1.20.1 and Forge 47.x.

## Current milestone: Stage 6 — Forge profile analysis and launch-spec generation

Stage 6 adds a read-only resolver for the imported Minecraft 1.20.1 Forge profile. It locates the actual Forge JSON inside the imported version directory, follows available \`inheritsFrom\` metadata, combines parent/child libraries and launch-argument arrays, checks declared library artifact files, checks the client JAR and asset index, and retains the profile-specific mods/config directory layout.

The resolver writes:

- \`minecraft/forge-launch-spec.json\`: structured profile metadata, library entries, inherited game/JVM arguments, paths, mod/config locations, and runtime candidates.
- \`minecraft/forge-readiness-report.txt\`: a readable audit that survives closing the app.

It also performs a limited Java-runtime inventory under \`minecraft/runtime/\`, looking for a \`bin/java\` executable and \`libjvm.so\`, and checks whether the shared library is a 64-bit little-endian AArch64 ELF file. This is only a file/architecture check; it does not prove the runtime can execute on Android.

The app button is labelled **Analyze Forge profile and prepare launch plan**. It does not launch Java or Minecraft.

## Zalith game-data import

The Android Storage Access Framework importer accepts a folder directly containing at least two of \`assets/\`, \`libraries/\`, and \`versions/\`; it also detects a wrapper folder containing a \`.minecraft/\` child.

The importer copies reusable game data into RustChunk's private \`files/minecraft/\` directory, including Minecraft assets, libraries, versions, mods, configs, resource packs, shader packs, KubeJS scripts, and selected mod data folders. It deliberately does not copy launcher account databases, authentication tokens, logs, crash reports, or launcher-specific private data.

The imported Zalith profile can keep its mods and config inside its own version directory. RustChunk therefore analyzes and records those paths without relocating files.

## Current capabilities and limitations

- Stage 2: resolves official Minecraft 1.20.1 metadata and builds a SHA-1-aware install plan.
- Stage 3: downloads base Minecraft files with four parallel workers, resume support, and SHA-1 verification.
- Stage 4: imports reusable game files from a user-selected Zalith game directory.
- Stage 5: persists import/inspection reports and audits game-data presence.
- Stage 6: resolves the imported Forge profile, available inheritance metadata, library paths, launch arguments, mod/config paths, and possible local Java runtime files.

**Not yet implemented:** Android-compatible Java runtime provisioning, complete Forge argument/rule substitution, native-library extraction for Android, a safe Java process bootstrap, actual Forge startup, graphics/runtime integration, and on-device Aternos/modpack compatibility testing. The generated launch specification is an intermediate input for those steps, not a runnable command.

A pure Rust game engine cannot directly load Java Forge mods. RustChunk must supply a compatible Java/Forge execution layer; this milestone prepares and audits the real imported profile rather than pretending the game is already runnable.

## Platform target

- Android ARM64 (arm64-v8a)
- Minecraft Java 1.20.1
- Forge 47.x compatibility target
- Android app-private storage under the RustChunk application directory

## Build and tests

Push to \`main\` or run RustChunk Android Build manually from GitHub Actions. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, builds the Android APK, verifies the persistent signing key and APK certificate, and uploads the artifact.
