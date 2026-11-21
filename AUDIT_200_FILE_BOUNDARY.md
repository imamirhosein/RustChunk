# RustChunk — 200-File Audit Boundary Register

- Repository: `imamirhosein/RustChunk`
- Branch: `starling-1.20.1-port-audit`
- Pinned source: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Pinned recursive tree: `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`)
- Entries through 2000 passed global structural/provenance verification and final backup; entries 2001–2050 are now complete.
- Current stage: **2001–2200**, progress **125/200**.
- One-time authorizations: **2001–2100** and **2101–2200**. For the second window, preserve 25-file commits but do not stop or request permission at 2150; stop only at 2200.

## Stage partition

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 2001–2050 | 2001–2025 — **COMPLETE** | 2026–2050 — **COMPLETE** | **CLOSED — 50/50** |
| 2051–2100 | 2051–2075 — **COMPLETE** | 2076–2100 — **COMPLETE** | **CLOSED — 50/50** |
| 2101–2150 | 2101–2125 — **COMPLETE** | 2126–2150 — PRE-AUTHORIZED | IN PROGRESS — 25/50; CONTINUE AT 2126 |
| 2151–2200 | 2151–2175 — PRE-AUTHORIZED | 2176–2200 — PRE-AUTHORIZED | PRE-AUTHORIZED — STOP ONLY AT 2200 |

## Checkpoint 2001–2025

- Exact paths are the next 25 blobs after excluding Index entries 1–2000 from the complete pinned tree.
- All files were inspected one at a time through EOF. Files over 400 lines were read in consecutive, gap-free ranges no larger than 400 lines: supervisor.rs 1–899; update.rs 1–1200; check.rs 1–607; manifest.rs 1–1175; stages.rs 1–697.
- Each entry has exactly 15 required fields. Blob SHA and exact byte size match the pinned tree.
- Index: 2,025 sequential unique paths. Active Ledger: 25 records (2001–2025).
- Findings include server signal/supervisor/update/WebSocket issues and shader API issues: ImageGather not counted in texture budget; incomplete stage-key validation; symlink-following package traversal and late aggregate size checks; substring-based set_surface detection.
- Shader API and samples are Horizon API 0.1/Naga 30.0.1 and renderer-specific, not drop-in Minecraft 1.20.1 Java shader support.
- Tests/examples inspected only; none run. No build, benchmark, APK, Android, Forge, server join, Aternos or renderer validation. No app source changed.

## Authorization and next actions

- Section 2001–2050 is complete and closed. Continue 2051–2100 under the user's already-issued one-time authorization.
- At 2050, report the first 50-file section, then continue directly into pre-authorized 2051–2100; retain checkpoints 2051–2075 and 2076–2100.
- Section 2051–2100 was closed at entry 2100. The user has now issued a second one-time exception for **2101–2200**.
- Complete all four 25-file checkpoints through entry 2200. **Do not stop at 2150**; close the exceptional window and STOP at 2200. Ask before entry 2201.

## Rules

- One source file at a time; complete through EOF; long files in contiguous chunks no larger than 400 lines.
- Verify each path, SHA and byte size against the pinned tree.
- Exactly 15 Ledger fields per record; sequential unique Index paths; Ledger/Index decisions align.
- Never claim tests/build/benchmark/APK/device/Forge/server/Aternos/renderer validation unless actually performed.
- Do not modify app source, `main` or existing backups.
- Next backup only after 1,000 additional completed entries (entry 3000) and global verification.


## Checkpoint 2026–2050 — CLOSED

- Completed entries 2026–2050 (25/25), bringing the active Ledger to 50 entries (2001–2050) and the global Index to 2,050 sequential unique paths.
- All source files were read one at a time through EOF; long files were inspected in sequential, gap-free chunks of no more than 400 lines. Exact SHA and byte size match the pinned tree.
- All 15 required Ledger fields occur exactly once; new Index rows align by entry number, path and Decision.
- The shaderpack reader/translator is a distinct optional rendering extension, not the Minecraft Java game client. The reviewed implementation has unresolved bounds, symlink, parser and cross-version correctness requirements.
- Continue at **2051–2075**; the next 50-file section **2051–2100 is pre-authorized**. Keep 25-file commits. Close and STOP at **2100**.


## Checkpoint 2051–2075 — COMPLETE (25/25)

- Entries 2051–2075 are complete; active Ledger has 75 records (2001–2075) and global Index has 2,075 sequential unique paths.
- All 25 source files were reviewed one at a time through EOF. Long-file coverage was contiguous and gap-free in chunks of no more than 400 lines; source SHA and byte size match the pinned recursive tree.
- Exactly 15 required fields per new Ledger entry; Index paths and Decisions align.
- Findings center on optional shaderpack behavior: incomplete allocation/output caps, fail-open shader conditions, heuristic GLSL parsing and cross-stage contracts, geometry-emulation assumptions, weak image/texture resource limits, and unvalidated option text substitution. The test fixture identifies itself as Horizon-only.
- Continue at **2076–2100** under pre-authorization; retain the 25-file checkpoint. Close section 2051–2100 at **50/50** and **STOP at 2100**. Ask before 2101.


## Checkpoint 2076–2100 — CLOSED

- Entries 2076–2100 are complete; active Ledger has 100 records (2001–2100), global Index has 2,100 sequential unique paths.
- All 25 sources were read serially through EOF. Long files were read in consecutive chunks no larger than 400 lines; SHA and exact byte size align with the pinned tree.
- Each Ledger record has exactly 15 required fields; new Index paths and Decision labels align.
- The reviewed shader test packs are Horizon-only. Telemetry is a separate opt-in Rust 1.90 backend; it is not part of the Android Minecraft client. Static risks include proxy trust, container privilege, blocking DB operations and unbounded analytics work.
- The one-time 100-file authorization is consumed. **STOP at 2100** and ask before any file 2101+.


## Second exceptional window — checkpoint 2101–2125 COMPLETE

- Completed entries 2101–2125 (25/25); active Ledger now has 125 records (2001–2125) and the global Index has 2,125 sequential unique paths.
- Every source was read serially through EOF; long-file coverage is contiguous and no range exceeds 400 lines. SHA and exact byte sizes match the complete pinned tree.
- Exact 15 required fields per Ledger entry; new Index paths and Decision labels align.
- Next checkpoints 2126–2150, 2151–2175 and 2176–2200 are all covered by the user's explicit one-time authorization. **Do not stop/request permission at 2150. STOP only at 2200.**
