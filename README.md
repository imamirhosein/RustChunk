# RustChunk

An independent Android-native Rust game-engine research project targeting Minecraft Java 1.20.1 compatibility.

## Current milestone: Stage 1 — Minecraft workspace

Stage 1 moves beyond the Android/Rust smoke test. The Android app calls the Rust native library, which initializes an app-private Minecraft directory layout.

The workspace contains shared Minecraft directories (`assets`, `libraries`, `versions`, `runtime`, `profiles`, `instances`, and `logs`) plus a Forge 1.20.1 instance with directories for mods, configs, resource packs, shader packs, saves, screenshots, logs, and crash reports.

A default `profile.properties` file is created only when one does not already exist. Reopening the app does not overwrite an existing profile file.

- Android package: `dev.rustchunk.app`
- Android ABI: `arm64-v8a`
- Rust library: `engine`
- Android app: `app`
- Build: GitHub Actions workflow `.github/workflows/android.yml`

This stage does **not** download Minecraft, install Forge, launch Java, or load mods yet.

## Build

Push to `main` or run **RustChunk Android Build** manually from the Actions tab. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, packages the Android app, and uploads a debug APK artifact.

## Long-term target

- Minecraft Java 1.20.1 world/client behavior
- Forge 47.x compatibility, developed incrementally
- Rust-native engine components where compatibility can be maintained
- Android touch input and device-specific graphics/performance work

Forge compatibility and Minecraft runtime integration are not implemented yet. They will be developed and tested in later milestones.
