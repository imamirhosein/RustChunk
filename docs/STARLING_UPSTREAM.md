# Starling upstream source pin

RustChunk uses the public Starling repository as its native engine reference through a Git submodule:

- Repository: https://github.com/starling-client/starling
- Submodule path: `upstream/starling`
- Tracking branch for future reviewed updates: `starling`
- **Pinned source commit:** `5432370d573642041585c69b40fb045b27133488`
- Release noted at the time of the audit: Starling 0.8.0
- Starling's original code license: MIT. Third-party code keeps its own license.

The gitlink pins an exact commit, so a normal checkout does not silently move to the latest upstream code. Update the pointer only after reviewing the upstream diff, version pin, changelog, third-party notices and relevant Android build changes.

The submodule is deliberately used instead of copying hundreds of upstream source files into RustChunk. This preserves upstream attribution and makes the baseline commit easy to audit. RustChunk-specific changes should be recorded as reviewable patches or commits in RustChunk's own maintained engine fork/branch; do not edit the submodule's working tree and assume those edits persist.

The pinned Starling engine targets Minecraft Java Edition 26.3. Its Android port is not a Minecraft 1.20.1 engine. The goal of the current port branch is to adapt the version-specific gameplay/data layer to 1.20.1 while retaining the asynchronous chunk streamer and renderer architecture where tests permit.

Key source areas to inspect:

- `crates/horizon-client`: client game loop, gameplay and single-player
- `crates/horizon-world`: world simulation, chunks, blocks, entities and world generation
- `crates/horizon-client-render`: chunk streaming, lighting, meshing and rendering
- `crates/horizon-protocol`: network packet definitions and codecs
- `crates/horizon-android` and `packaging/android`: Android activity and packaging

Keep `LICENSE` and `THIRD_PARTY_NOTICES.md` with every derived distribution. Do not assume the root MIT license overrides the licenses of dependencies or vendored components.
