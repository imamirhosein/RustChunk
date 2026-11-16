# Native Rust Engine Feasibility

Status: research branch, not a production architecture change.

## Goal

Evaluate a Minecraft client whose **game engine runs natively in Rust on Android**, rather than using Rust only as a launcher around the normal Java client. The required compatibility target remains **Minecraft 1.20.1 with Forge 47.4.x**, including the user's existing Java Forge modpack.

The current `main` branch is preserved as the Stage 21 launcher/JNI baseline. Build 46 (commit `d39584042189c18f3224b3bfb67cfde61161dc46`) completed successfully, but it only builds a Forge launch-plan diagnostic APK; it does not launch Forge or provide a native Minecraft engine.

## Findings

### 1. Existing Horizon Android work is a useful engine reference, not a drop-in Forge solution

The public Android port in `RxLauncherTeam/RxLauncher-Rust-Edition`, branch `android`, is a native Rust Android client based on Horizon. Its documented game target is Minecraft Java **26.3**. It supports Android 9+, Vulkan, ARM64 builds, Android touch controls and sandboxed native `.hmod` mods.

Its own Android feature documentation explicitly says Java/Fabric/Forge/Mixin code is not executed. Its native mod format is not compatible with Forge JARs. The Android README also says the published device footage is not a controlled performance benchmark. Therefore:
- Reuse its architecture and lessons only after reviewing source and license notices.
- Do not label it Forge-compatible.
- Do not assume its desktop performance figures apply to the Galaxy A22.

### 2. Horizon's Java bridge proposal does not solve our target

The upstream design discussion for a Java bridge describes running unchanged **Fabric 26.3** mods in a JVM beside the Rust engine. The design keeps world simulation and rendering in Rust and asks Java mods for their changes through a bridge.

A separate unofficial beta repository, `iaxivers/horizon-java-bridge`, now demonstrates the idea on desktop: unchanged Fabric 26.3 JARs run in an optional JVM while Horizon owns the game and renderer. Its published instructions target Windows/Linux, require a Microsoft account that owns Java Edition, and provide no offline path. It is not an Android build and it does not implement Forge 1.20.1.

The bridge idea is useful evidence that a Rust game engine can coexist with Java mod code. It does not supply the version-matched Minecraft 1.20.1 classes, Forge 47.x loader/events, or Android runtime integration our target needs.

### 3. Why unchanged Forge JARs cannot simply be loaded by Rust

Forge mods are Java bytecode. Typical mods depend on the exact Minecraft and Forge Java classes, Forge lifecycle/event APIs, ModLauncher, class transformation and often Mixin/coremod behavior. A Rust engine cannot execute those JARs merely by reading them.

There are only two broad ways to preserve ordinary Forge JARs:
1. Provide a JVM-compatible runtime and the expected Minecraft 1.20.1/Forge 47.x Java environment, then bridge mod behavior into the Rust engine.
2. Reimplement or translate the required Java/Forge runtime and APIs in a compatible native system. That is a much larger undertaking and still needs to preserve Java class-loading and transformation semantics.

A strict no-JVM client that executes existing Forge JARs unchanged is not a realistic first milestone. If unchanged Forge compatibility remains mandatory, an **optional Java compatibility runtime** is the practical starting point, while the actual game engine, simulation, chunk system and rendering remain Rust.

### 4. Target-device graphics capability

The target Galaxy A22 4G (SM-A225F, Mali-G52 MC2) has public device records showing Vulkan 1.1 support and successful Vulkan compute runs. This is encouraging for testing a Vulkan-based native engine, but it does not prove that Horizon Android is stable or fast on this phone. Only an on-device launch and gameplay test can establish that.

## Proposed architecture to investigate

- **Rust native engine:** Android lifecycle, input, world/chunk simulation, entity state, networking, save logic and Vulkan rendering.
- **Optional Java compatibility worker:** Java 17 runtime for Forge 1.20.1 / Forge 47.x mod bytecode and its class-loading/transformation pipeline.
- **Compatibility bridge:** explicit, versioned messages for registries, events, block/item/entity definitions, world changes, HUD/UI contributions and resource data. No direct Java rendering calls in the Rust render loop.
- **Mod compatibility tiers:** native Rust mods; data/assets-only imports; and Java Forge mods that pass bridge compatibility tests. Never advertise every Forge mod as supported without testing.
- **Server compatibility:** verify protocol, mod list, registries and login behavior against an actual matching 1.20.1 Forge server. Offline identity can only join servers that permit offline-mode or another compatible authentication flow.

## Ordered feasibility gates

1. **Android engine smoke test:** run an existing ARM64 Horizon Android build on the Galaxy A22; verify startup, Vulkan renderer, a new single-player world, touch controls, memory use, thermal behavior and a short session. Record logs and measured frame times.
2. **Version gate:** determine the amount of work needed to target the exact 1.20.1 game rules, assets, protocol and save behavior. A 26.3 engine cannot be assumed to accept 1.20.1 Forge mods.
3. **Java bridge prototype:** prove that a minimal Forge 1.20.1 mod can be loaded and communicate a simple registry/event change to a Rust test engine.
4. **Real mod gate:** test one small, representative mod, then expand to libraries and the user's actual mod list in dependency order.
5. **Aternos gate:** test the exact server-side modpack, protocol and authentication mode only after the client-side gates pass.

## Decision rule

Do not replace the working `main` branch or discard its stable signing identity until the native Android engine passes Gate 1 and the Forge bridge has a credible technical prototype. Keep the native investigation isolated on this branch. A successful APK build by itself is not proof of Vulkan gameplay, Forge loading, mod compatibility or Aternos connectivity.

## References

- Horizon project: https://github.com/horizon-rust/horizon
- Horizon Android port: https://github.com/RxLauncherTeam/RxLauncher-Rust-Edition/tree/android
- Horizon Android feature support: https://github.com/RxLauncherTeam/RxLauncher-Rust-Edition/blob/android/docs/android-features.md
- Horizon Java bridge beta (Fabric 26.3, desktop): https://github.com/iaxivers/horizon-java-bridge
- Horizon Java mod design discussion: https://github.com/horizon-rust/horizon/issues/10216
- Forge 1.20.x documentation: https://docs.minecraftforge.net/en/1.20.x/
- Target device Vulkan record: https://slions.net/resources/samsung-galaxy-a22.33/
