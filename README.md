# RustChunk

An independent Android launcher and game-engine research project targeting Minecraft Java 1.20.1 and Forge 47.x.

## Current milestone: Stage 19 — JNI Forge launch-plan preparation

Stage 19 builds a profile-specific launch plan for Minecraft 1.20.1 Forge 47.x using the already-verified Java 17 JNI runtime. RustChunk flattens inherited game/JVM arguments, applies Android/Linux ARM64 argument rules, resolves standard Minecraft placeholders, computes the offline-player UUID, assembles the Java class path, resolves Forge's module path, and converts launcher-only module option pairs (such as -p and --add-opens <value>) into the option=value form required by the JNI Invocation API. It writes the result to rustchunk-jni-launch-plan.json and a readable report in the selected Forge profile directory.

The launch-plan action is deliberately a preparation/validation step. It does not start Forge. It reports missing JARs, unresolved arguments, and unsupported JVM tokens rather than silently trying to launch an incomplete profile. Android LWJGL native mapping, EGL/OpenGL surface integration, audio, and the actual Forge client invocation remain separate work.

Stage 18 established that Java 17.0.10 can be created directly with JNI_CreateJavaVM from libjvm.so on the target ARM64 Android device. The test successfully queried java.version, java.home, and os.arch, and detached its JNI-created thread. No app-private bin/java executable is run.

Stage 12 adds a profile-specific dependency plan and downloader for the selected Minecraft 1.20.1 Forge profile. It reads merged library metadata from the selected profile's inheritance chain, applies Android/Linux ARM64 library rules, excludes native classifiers, and schedules only artifacts with an HTTPS URL, safe relative path, valid SHA-1, and declared size. Existing files are reused only after size and SHA-1 verification; partial downloads can resume and are verified before finalization. The generated plan and post-download readiness report remain inside the selected profile directory. Library declarations without complete verification metadata are reported as unresolved rather than downloaded blindly.

Stage 11 adds **Install Java 17 from installed Zalith**. RustChunk checks the installed ZalithLauncher2 package variants and reads the two ARM64 JRE 17 TAR.XZ assets directly from the Zalith APK's public assets. It does not attempt to read Zalith's private `/data/user/0` directory, does not ask the user to copy that protected directory through a file picker, and does not bundle or download a separate runtime package.

The installer extracts the common and ARM64 runtime archives into RustChunk's own `files/games/runtimes/Internal-17/` directory. TAR symbolic links are recreated with Android's native symlink API, archive paths are checked against path traversal, extracted data is size-limited, executable permissions are restored for `bin/`, and the previous runtime is preserved until the new directory has been extracted. The installer then runs `bin/java -version` and saves its report to `files/runtime/java17-install-report.txt`.

Stage 10's **Test installed Java runtime** remains available as a separate diagnostic. It searches known runtime roots, executes detected `bin/java -version` files, and saves the report at `files/runtime/runtime-test-report.txt`.

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
- Stage 10: adds actual Java executable testing with timeout handling and a persistent diagnostic report.
- Stage 11: installs the same ARM64 Java 17 runtime payload already present in the installed ZalithLauncher2 APK, without requiring access to Zalith's private app data.
- Stage 12: prepares and downloads checksum-verified Java library artifacts for the selected Forge profile, reusing valid files and keeping profile-specific plans/reports isolated.
- Stage 18: verifies direct Java 17 HotSpot startup using the JNI Invocation API, avoiding execution of app-private bin/java.
- Stage 19: prepares a JNI-compatible Forge launch plan with inherited argument rules, module-path conversion, verified classpath entries, and an offline player identity.

**Not yet implemented:** Android LWJGL native-library mapping/extraction, EGL/OpenGL surface integration, audio integration, actual Forge JVM invocation, complete game launch lifecycle, and on-device modpack/Aternos compatibility testing. The launch plan is a preparation artifact, not proof that Forge starts. Java startup success alone does not prove Forge or Minecraft can launch.

A pure Rust game engine cannot directly load Java Forge mods. RustChunk still needs a compatible Java/Forge execution layer. This milestone installs and validates Java 17; it does not yet launch Minecraft.

## Platform target

- Android ARM64 (arm64-v8a)
- Minecraft Java 1.20.1
- Forge 47.x compatibility target
- Android app-private storage under the RustChunk application directory

## Build and tests

Push to `main` or run RustChunk Android Build manually from GitHub Actions. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, builds the Android APK, verifies the persistent signing key and APK certificate, and uploads the artifact.
