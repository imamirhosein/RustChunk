# RustChunk Starling patches

Project-specific engine changes live here as ordered Git patches, not as uncommitted edits inside the submodule.

- `manifest.json` pins the exact clean upstream base commit and lists patches in application order.
- Add each patch file to `manifest.json` only after it has been reviewed against that base.
- CI runs `python tools/apply_starling_patches.py` before building the engine.
- The applier verifies the base SHA, requires a clean submodule, preflights the entire series, then applies it.
- Never add Mojang JARs, assets, generated private resources or decompiled source to this directory.

The current series is intentionally empty. That means the first ARM64 baseline build is unmodified Starling 26.3; it is not a 1.20.1 build.
