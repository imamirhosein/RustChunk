# Starling → RustChunk audit ledger

- Source repository: `starling-client/starling`
- Source ref: `starling`
- Source tree snapshot recorded for this audit: `5432370d573642041585c69b40fb045b27133488`
- Target repository: `imamirhosein/RustChunk`
- Target branch: `starling-1.20.1-port-audit`
- Scope: assess Starling's architecture and implementation for lessons or components relevant to a Rust-based Android launcher/client targeting Minecraft Java 1.20.1 Forge. Starling's documented target is Java 26.3; no compatibility with Forge 1.20.1 is assumed.
- Rule: a file is complete only after its full contents (or all consecutive ≤400-line chunks) have been read, analyzed, and recorded here. Claims in documentation remain claims until verified in implementation.
- Decision labels: **essential**, **adaptable**, **rewrite required**, **integrate**, **unnecessary**, **undetermined**.

## Completed files

### 1. `README.md`
- **Source SHA:** `3cd9d680757217774270f477ecbfc9c80563e39f`
- **Size:** 146 lines; 17,521 characters; full file fetched and read.
- **Coverage:** lines 1–146, complete. No chunks omitted.
- **Purpose/content:** project overview, legal boundary, Android capabilities/status, mod formats, build prerequisites and workflow, Minecraft resource preparation, contribution/licensing information, and supported-device claims.
- **Architecture evidence stated by the README:**
  - Starling is described as an Android port/fork of Horizon, whose game implementation is rewritten in Rust; the Android app is positioned around Horizon rather than being a conventional JVM launcher.
  - It targets Minecraft Java 26.3 and explicitly requires user-owned game files; Minecraft code/assets/data are not bundled.
  - It describes two mod paths: native `.hnmod` code compiled on-device (ARM64 only) and data-only `.hmod` content mods.
  - It describes Android APK packaging via `packaging/android/build_apk.py`, and host-side preparation via `horizon-prepare`; source game inputs are checksum-pinned through `research-lock.json`.
  - It documents Android account sign-in plus offline profiles, server support, shader/resource packs, touch controls, diagnostics, and device-specific performance settings. These are project claims, not independently validated implementation findings.
- **Direct references / likely dependencies:** `roadmap/starling.md`, `docs/horizon-parity.md`, `docs/android-armv7.md`, `docs/android-controls.md`, `docs/android-accounts.md`, `docs/android-diagnostics.md`, `docs/native-mods.md`, `docs/starling-native-mods.md`, `docs/mods.md`, `packaging/android/build_apk.py`, `packaging/lowbudget/make.py`, `research-lock.json`, `Cargo.toml`, `Cargo.lock`, `CONTRIBUTING.md`, `THIRD_PARTY_NOTICES.md`, and `LICENSE`. These are README links/references; actual implementation dependencies and call relationships remain to be checked in their respective files.
- **RustChunk relevance:** high as a system-level map and requirements inventory. Particularly useful topics to investigate are the Rust/native execution boundary, Android packaging, legal/user-supplied game-resource preparation, ABI/GPU limitations, input abstraction, account/profile handling, diagnostics, and separation of code mods from data mods.
- **Important incompatibility / risk:** the documented target is 26.3, while RustChunk's target is 1.20.1 Forge. The README gives no evidence that Starling supports Forge, Forge's mod-loading lifecycle, Java bytecode mods, or the 1.20.1 protocol/runtime. Starling's native/content mod formats must not be treated as Forge compatibility. A Rust rewrite of the game is also a fundamentally different execution model from launching the original Forge Java runtime.
- **Preliminary decision:** **adaptable** for high-level Android product design, diagnostics, touch controls, resource acquisition/verification, and build organization; **rewrite required** for any feature whose implementation assumes Starling/Horizon internals or version 26.3; Forge compatibility remains **undetermined** and must be investigated explicitly.
- **Follow-up checks:** validate each documented capability against source code; inspect Cargo workspace/crates and build pipeline; trace how 26.3 files are prepared and consumed; inspect account/auth and offline-mode semantics; determine whether any Java/FML/Forge bridge exists (none is established by this README); compare supported ABIs and graphics APIs to RustChunk's target hardware.
- **Completion state:** complete documentation review; implementation claims not yet validated.

## Pending first batch (selected order)

2. `LICENSE`
3. `THIRD_PARTY_NOTICES.md`
4. `CONTRIBUTING.md`
5. `Cargo.toml`
6. `Cargo.lock`
7. `.gitignore`
8. `.github/workflows/android.yml`
9. `packaging/android/build_apk.py`
10. `packaging/lowbudget/make.py`
11. `research-lock.json`
12. `roadmap/starling.md`
13. `docs/horizon-parity.md`
14. `docs/android-armv7.md`
15. `docs/android-controls.md`
16. `docs/android-accounts.md`
17. `docs/android-diagnostics.md`
18. `docs/native-mods.md`
19. `docs/starling-native-mods.md`
20. `docs/mods.md`


### 2. `LICENSE`
- **Source SHA:** `e959a076e9fa4b8c1fcf990b2253a935da5bb5f4`
- **Size:** 22 lines; full file fetched and read.
- **Coverage:** complete; no chunks omitted.
- **Purpose/content:** standard MIT license text. It attributes the copyright to Ludovic Marie and Horizon contributors (2026), grants broad rights to use, modify, distribute, sublicense, and sell the licensed software, requires retaining the notice, and disclaims warranties and liability.
- **Dependencies / scope:** this file is the repository's top-level license declaration. It does not enumerate third-party dependency licenses or independently grant rights to Minecraft/Mojang assets. The README explicitly limits MIT coverage to original project work; `THIRD_PARTY_NOTICES.md` is the next source for component-specific attribution.
- **RustChunk relevance:** critical due-diligence input before copying or adapting code. MIT is permissive, but the copyright notice and license text must be retained for substantial copied portions; dependencies and upstream code may have separate obligations.
- **Limitations:** a root MIT license alone does not establish the license of every vendored/generated/resource file or third-party dependency. Do not infer that game assets, Minecraft binaries, or third-party libraries are covered.
- **Decision:** **adaptable** as a licensing model reference; copying Starling/Horizon code requires preserving notices and checking each dependency's actual license. Minecraft files remain outside this license scope.
- **Completion state:** complete license-text review; third-party notices and dependency metadata still require separate review.
