# RustChunk

An independent Android-native Rust game-engine research project targeting Minecraft Java 1.20.1 compatibility.

## Current milestone: Stage 0

Stage 0 verifies the Android build pipeline and the JNI boundary between a native Rust library and the Android app. It is a build/bridge smoke test only; it does **not** run Minecraft yet.

- Android package: `dev.rustchunk.app`
- Android ABI: `arm64-v8a`
- Rust library: `engine`
- Android app: `app`
- Build: GitHub Actions workflow `.github/workflows/android.yml`

## Build

Push to `main` or run **RustChunk Android Build** manually from the Actions tab. The workflow runs Rust unit tests, cross-compiles the Rust shared library for Android ARM64, packages the Android app, and uploads a debug APK artifact.

## Long-term target

- Minecraft Java 1.20.1 world/client behavior
- Forge 47.x compatibility, developed incrementally
- Rust-native engine components where compatibility can be maintained
- Android touch input and device-specific graphics/performance work

Forge compatibility and Minecraft runtime integration are not implemented in Stage 0. They will be developed and tested in later milestones.
