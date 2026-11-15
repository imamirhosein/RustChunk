# RustChunk

An independent Android launcher and game-engine research project targeting Minecraft Java 1.20.1 and Forge 47.x.

## Current milestone: Stage 10 — Java runtime execution diagnostics

Stage 8 hardens the selectable-profile UI: the supported Forge 1.20.1 profile is preselected explicitly, and the app recognizes it from its ID even when older metadata omits the support flag. Stage 7 added a catalogue for the version JSON profiles found under `minecraft/versions/`. The app identifies Forge, NeoForge, Fabric, Quilt, and Vanilla entries and lets the user select a profile. The analysis action is currently enabled only for Forge profiles inheriting Minecraft 1.20.1; other profile types and game versions are listed but are not yet launchable.

The selected profile is resolved by its exact profile ID rather than by choosing the first Forge folder. Its mods, config, generated launch specification, and readiness report are handled inside that selected profile's own version directory. An analysis of one profile no longer overwrites a global report belonging to another profile.

Stage 7 also corrects inherited library overrides and legacy argument inheritance. Desktop-only `natives-linux`, `natives-windows`, and `natives-macos` classifiers are separated from the Android artifact-presence audit rather than incorrectly treated as Android libraries. This does not mean Android native replacements have been installed; native mapping remains future work.

The **Import Android Java runtime folder** action copies a user-selected runtime directory to a separate `files/runtime/imported-*/` location. Files in `bin/` are marked executable after SAF copying. Android's Storage Access Framework does not guarantee preservation of symbolic links, so the copied runtime must be tested. Stage 10 adds **Test installed Java runtime**, which searches known runtime locations and actually executes each detected `bin/java -version` (up to eight candidates), with a 15-second timeout and a saved report at `files/runtime/runtime-test-report.txt`. This verifies Java startup only; it does not launch Forge or Minecraft.

## Reports and profile-specific data

For a selected profile such as `1.20.1 Forge 47.4.22`, RustChunk stores:
- `versions/<selected-profile>/rustchunk-launch-spec.json`
- `versions/<selected-profile>/rustchunk-readiness-report.txt`

The selected profile's mods and config are referenced in place. RustChunk does not move, merge, or duplicate profile-specific files.

The profile list is discovered from JSON metadata. A profile is shown even when it is not currently supported by the Forge analysis action. Full profile creation, editing, deletion, and independent game-directory migration are not implemented yet.

## Zalith game-data import

The Android Storage Access Framework importer accepts a folder directly containing at least two of `assets/`, `libraries/`, and `versions/`; it also detects a wrapper folder containing a `.minecraft/` child.

It copies reusable assets, libraries, versions, mods, configs, resource packs, shader packs, KubeJS scripts, and selected mod data folders. Launcher account databases, authentication tokens, logs, crash reports, and launcher-specific private data are not copied.

## Current capabilities and limitations

- Stage 2: resolves official Minecraft 1.20.1 metadata and builds a SHA-1-aware install plan.
- Stage 3: downloads base Minecraft files with four parallel workers, resume support, and SHA-1 verification.
- Stage 4: imports reusable game data from a user-selected Zalith game directory.
- Stage 5: persists import/inspection reports and audits game-data presence.
- Stage 6: resolves inherited Forge metadata, libraries, launch arguments, and local runtime candidates.
- Stage 7: lists installed profiles, selects an exact Forge 1.20.1 profile, stores reports per profile, and imports a user-selected Java runtime directory.
- Stage 8: fixes the profile-selection state so the Forge analysis action is enabled for the imported Minecraft 1.20.1 Forge profile.
- Stage 9: adds the standard Zalith `files/games/runtimes` directory to runtime discovery.
- Stage 10: adds actual Java executable testing with timeout handling and a persistent diagnostic report; also prevents overlapping actions during game-data import.

**Not yet implemented:** a reliable Java 17 runtime installer that preserves symlinks and includes required Android native support, complete argument substitution and rule evaluation, Android native-library extraction/mapping, a safe Java process bootstrap, actual Forge startup, graphics/runtime integration, and on-device modpack/Aternos compatibility testing. The launch specification is still an intermediate diagnostic file, not a runnable command.

A pure Rust game engine cannot directly load Java Forge mods. RustChunk still needs a compatible Java/Forge execution layer. This milestone prepares and audits profile-specific inputs; it does not launch Minecraft.

## Platform target

- Android ARM64 (arm64-v8a)
- Minecraft Java 1.20.1
- Forge 47.x compatibility target
- Android app-private storage under the RustChunk application directory

## Build and tests

Push to `main` or run RustChunk Android Build manually from GitHub Actions. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, builds the Android APK, verifies the persistent signing key and APK certificate, and uploads the artifact.
