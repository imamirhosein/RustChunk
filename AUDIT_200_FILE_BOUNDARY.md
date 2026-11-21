# RustChunk — 200-File Audit Boundary Register

- Repository: `imamirhosein/RustChunk`
- Branch: `starling-1.20.1-port-audit`
- Pinned source: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Pinned recursive tree: `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`)
- Entries through 2000 are complete; global structural/provenance verification and final 1–2000 backup are preserved.
- Current stage: **2001–2200**, progress **25/200**.
- One-time authorization: **2001–2100** only (two adjacent 50-file sections); 2101–2200 is locked.

## Stage partition

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 2001–2050 | 2001–2025 — **COMPLETE** | 2026–2050 — AUTHORIZED | IN PROGRESS — 25/50 |
| 2051–2100 | 2051–2075 — PRE-AUTHORIZED | 2076–2100 — PRE-AUTHORIZED | AUTHORIZED IN ADVANCE — 0/50 |
| 2101–2150 | LOCKED | LOCKED | NOT AUTHORIZED |
| 2151–2200 | LOCKED | LOCKED | NOT AUTHORIZED |

## Checkpoint 2001–2025

- Exact paths are the next 25 blobs after excluding Index entries 1–2000 from the complete pinned tree.
- All files were inspected one at a time through EOF. Files over 400 lines were read in consecutive, gap-free ranges no larger than 400 lines: supervisor.rs 1–899; update.rs 1–1200; check.rs 1–607; manifest.rs 1–1175; stages.rs 1–697.
- Each entry has exactly 15 required fields. Blob SHA and exact byte size match the pinned tree.
- Index: 2,025 sequential unique paths. Active Ledger: 25 records (2001–2025).
- Findings include server signal/supervisor/update/WebSocket issues and shader API issues: ImageGather not counted in texture budget; incomplete stage-key validation; symlink-following package traversal and late aggregate size checks; substring-based set_surface detection.
- Shader API and samples are Horizon API 0.1/Naga 30.0.1 and renderer-specific, not drop-in Minecraft 1.20.1 Java shader support.
- Tests/examples inspected only; none run. No build, benchmark, APK, Android, Forge, server join, Aternos or renderer validation. No app source changed.

## Authorization and next actions

- Complete 2026–2050 under current authorization. Commit and read back all six audit documents at 2050.
- At 2050, report the first 50-file section, then continue directly into pre-authorized 2051–2100; retain checkpoints 2051–2075 and 2076–2100.
- Close section 2051–2100 and **STOP at 2100**. Ask before entry 2101.
- The 100-file authorization is a one-time exception only; future sections return to the default 50-file boundary approval rule.

## Rules

- One source file at a time; complete through EOF; long files in contiguous chunks no larger than 400 lines.
- Verify each path, SHA and byte size against the pinned tree.
- Exactly 15 Ledger fields per record; sequential unique Index paths; Ledger/Index decisions align.
- Never claim tests/build/benchmark/APK/device/Forge/server/Aternos/renderer validation unless actually performed.
- Do not modify app source, `main` or existing backups.
- Next backup only after 1,000 additional completed entries (entry 3000) and global verification.
