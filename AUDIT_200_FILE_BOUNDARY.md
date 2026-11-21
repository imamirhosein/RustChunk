# RustChunk — 200-File Audit Boundary Register

- Repository: `imamirhosein/RustChunk`
- Branch: `starling-1.20.1-port-audit`
- Pinned source: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Pinned recursive tree: `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`)
- Entries **1–2000** passed global structural/provenance verification and final backup; entries **2001–2250** are complete.
- Active 200-file stage: **2201–2400**, progress **50/200**.
- Reporting section **2201–2250 is CLOSED (50/50)**. Both 25-file checkpoints are committed and read-back verified.
- The user has explicitly authorized section **2251–2300**. Checkpoint **2251–2275 is COMPLETE (25/25)**; checkpoint **2276–2300 remains AUTHORIZED** under the same grant.
- Continue through 2276–2300 under the same authorization; **STOP at 2300** and request fresh permission before entry 2301.

## Stage partition

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 2001–2050 | 2001–2025 — **COMPLETE** | 2026–2050 — **COMPLETE** | **CLOSED — 50/50** |
| 2051–2100 | 2051–2075 — **COMPLETE** | 2076–2100 — **COMPLETE** | **CLOSED — 50/50** |
| 2101–2150 | 2101–2125 — **COMPLETE** | 2126–2150 — **COMPLETE** | **CLOSED — 50/50** |
| 2151–2200 | 2151–2175 — **COMPLETE** | 2176–2200 — **COMPLETE** | **CLOSED — 50/50; AUTHORIZATION EXHAUSTED AT 2200** |

## Checkpoint 2001–2025

- Exact paths are the next 25 blobs after excluding Index entries 1–2000 from the complete pinned tree.
- All files were inspected one at a time through EOF. Files over 400 lines were read in consecutive, gap-free ranges no larger than 400 lines: supervisor.rs 1–899; update.rs 1–1200; check.rs 1–607; manifest.rs 1–1175; stages.rs 1–697.
- Each entry has exactly 15 required fields. Blob SHA and exact byte size match the pinned tree.
- Index: 2,025 sequential unique paths. Active Ledger: 25 records (2001–2025).
- Findings include server signal/supervisor/update/WebSocket issues and shader API issues: ImageGather not counted in texture budget; incomplete stage-key validation; symlink-following package traversal and late aggregate size checks; substring-based set_surface detection.
- Shader API and samples are Horizon API 0.1/Naga 30.0.1 and renderer-specific, not drop-in Minecraft 1.20.1 Java shader support.
- Tests/examples inspected only; none run. No build, benchmark, APK, Android, Forge, server join, Aternos or renderer validation. No app source changed.

## Historical authorization and next actions — snapshots from earlier checkpoints

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


## Checkpoint 2126–2150 — COMPLETE (25/25)

- All entries 2126–2150 were completely inspected in pinned-tree order; exact paths, SHAs and sizes match the full source tree.
- All 11 TTFs were fully Base64-decoded and parsed: SFNT directories/ranges, all table and whole-font checksums, metadata, cmap/metrics, GSUB/GPOS and glyph outlines. No metadata-only binary review was used.
- Three SIL OFL notices, one Hugeicons MIT notice and seven SVGs were fully read. SVGs were checked for executable/embedded/external resources.
- No font supports Arabic/Persian; provide Persian-capable font, shaping, bidi and fallback. Geist Mono is fixed-width Latin code/log face; Pixelify decorative. Preserve all notices with shipped assets.
- SVGs are simple 24×24 currentColor vectors, but Android vector renderer remains untested.
- Ledger 150 records (2001–2150), 15 fields each; Index 2,150 sequential unique paths.
- Section **2101–2150 CLOSED (50/50)**. Continue 2151–2175 and 2176–2200 under same authorization. **Do not stop at 2150; STOP only at 2200.**


## Checkpoint 2151–2175 — COMPLETE (25/25)

- All 25 next SVG source files were read completely and sequentially; exact SHA/size pairs match the pinned tree.
- Entire set uses one-line UTF-8 SVGs and simple 24×24 vector paths/circles. No script/foreignObject/embedded image/use, href/xlink or CSS URL external resources were found.
- Discord is the filled currentColor exception without explicit width/height; dashboard uses square line caps. Keep individual rendering attributes intact when converting.
- Hugeicons MIT notice from entry 2143 must accompany redistributed icon files. Android vector renderer compatibility is not proven.
- Active Ledger 175 records, 15 fields each; Index 2,175 sequential unique paths. Current section 2151–2200 is 25/50 complete; final half is pre-authorized.
- Continue 2176–2200 with no permission request at 2175. **STOP only at 2200.**


## Final authorized checkpoint 2176–2200 — COMPLETE (25/25)

- Entries 2176–2200 were fully inspected in raw Git path order; exact source paths, SHAs and sizes match the pinned complete tree.
- Seven Hugeicons Glass SVGs were completely read. PNG horizon-64 is 64×64 RGBA, 797 bytes; all four chunk CRCs pass, IEND ends at EOF and visual preview was inspected.
- Pixelarticons MIT notice was fully read (21 lines, Copyright © 2019 Gerrit Halfmann); sixteen Pixelarticons SVGs are completely read and separate from the Hugeicons package.
- SVGs are self-contained without active/external resources. PNG ownership/license remains unverified. Android SVG conversion/rendering was not tested.
- Seven later assets (2201–2207, `home.svg` through `package.svg`) were unintentionally fetched during a path-order mistake; they are excluded from Ledger/Index/counts and are not authorized complete files.
- Ledger **200 records**, Index **2,200 sequential unique paths**; all four 25-file checkpoints in the 2101–2200 window are complete. Stage **200/200 CLOSED**.
- **STOP at 2200. Authorization exhausted; fresh explicit permission is required before entry 2201.**


## Historical stage snapshot — after checkpoint 2201–2225 (superseded by current table below)

| 50-file section | Checkpoint 1 | Checkpoint 2 | State at that time |
|---|---|---|---|
| 2201–2250 | 2201–2225 — COMPLETE | 2226–2250 — AUTHORIZED | IN PROGRESS — 25/50 |
| 2251–2300 | NOT STARTED | NOT STARTED | NOT YET AUTHORIZED |
| 2301–2350 | NOT STARTED | NOT STARTED | NOT YET AUTHORIZED |
| 2351–2400 | NOT STARTED | NOT STARTED | NOT YET AUTHORIZED |

## Checkpoint 2201–2225 — COMPLETE (25/25)

- All 25 new paths are unique against the prior 2,200 Index entries; source SHA and exact byte size match the complete pinned tree.
- Every file was inspected one at a time through EOF. lib.rs was read in three consecutive chunks (1–400, 401–800, 801–877).
- Ledger: 225 records in 2001–2225; exactly 15 fields each. Index: 2,225 sequential unique paths; decisions align.
- The seven previously fetched assets 2201–2207 were re-fetched and reviewed within this explicit authorization; they are now included as complete entries.
- Continue immediately at 2226 under the same authorization. Preserve the next exact 25-file checkpoint, then STOP at 2250. No authorization exists for 2251+.


## Current stage partition — 2201–2400

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 2201–2250 | 2201–2225 — COMPLETE | 2226–2250 — COMPLETE | CLOSED — 50/50 |
| 2251–2300 | 2251–2275 — COMPLETE | 2276–2300 — COMPLETE | CLOSED — 50/50; authorization exhausted |
| 2301–2350 | NOT STARTED | NOT STARTED | NOT AUTHORIZED |
| 2351–2400 | NOT STARTED | NOT STARTED | NOT AUTHORIZED |

## Checkpoint 2226–2250 — COMPLETE (25/25)

- Each source was inspected individually through EOF; paths, SHAs and byte sizes align with the complete pinned tree. Long files were read in sequential, gap-free chunks no larger than 400 lines.
- New Ledger records 2226–2250 contain exactly 15 required fields. Combined Ledger count is 250 (2001–2250); Index is 2,250 sequential unique paths with Decisions aligned.
- Critical recorded findings: jobs pool can misreport active worker count after spawn failure; memory watcher Once cannot retry after spawn failure; chunkgen benchmark may accept truncated output; fixture-only comparator, recursive placement and numeric-normalization blind spots.
- No tests/examples executed; no app source changed; no runtime/build/device/game validation.
- Historical stop condition at this closed boundary: wait for the next explicit authorization. The user has now granted the next section 2251–2300; current state is shown in the table above. **STOP at 2300**, not at 2250.


## Index chronology reconciliation — 2026-11-21

- 70 adjacent transitions relative to the pinned GitHub recursive-tree API order were found among the 2,250 stable Index rows: 69 occur in entries 1–2000, and one occurs at entry 2033. No additional raw-tree-position regressions occur from entry 2034 through 2250. This is a historical ordering divergence, not a missing-file, duplicate-path or source-provenance defect: all 2,250 Index paths are unique and present in the pinned tree. The audit IDs are stable historical chronology and must not be renumbered because Ledger, QA and cross-document references depend on them. Do not describe the whole Index as a literal raw recursive-tree traversal.
- The latest indexed path is `crates/horizon-world/examples/probe/block_landing_fixture.rs`. A full-set check found **zero unindexed source paths lexically before or equal to it**; all **2,718** remaining unindexed blobs are after this boundary.
- From entry **2251** onward, select the next unindexed paths in deterministic lexical path order strictly after the latest indexed path, excluding the complete Index. Recheck uniqueness, pinned-tree membership, source SHA and byte size before each checkpoint. Preserve stable entry numbering and the 25/50/200 cadence.


## Checkpoint 2251–2275 — COMPLETE (25/25)

- All 25 unique files were inspected serially through EOF in deterministic lexical order after the complete Index; exact source SHAs and byte sizes match the full pinned tree.
- Long-file coverage: `block_update_fixture.rs` 1–400, 401–474; `collision_context_fixture.rs` 1–400, 401–450; `consumable_fixture.rs` 1–400, 401–413.
- Ledger 275 entries (2001–2275), exactly 15 fields each; Index 2,275 sequential unique paths and matching decisions.
- This checkpoint consists of differential mock fixtures for block properties, survival, push/update, collision, chunk stages, carving, climbing, combat/blocking, component patches and consumable/effect behavior. They are not production client code.
- Next checkpoint **2276–2300** is already authorized. Do not stop/request permission at 2275; **STOP at 2300**. No authorization exists for 2301+.


## Checkpoint 2276–2300 — COMPLETE (25/25)

- Each source was inspected individually through EOF; long files were read in consecutive, gap-free chunks no larger than 400 lines. Source paths, SHAs and sizes match the pinned recursive tree.
- Ledger contains 300 records (2001–2300), each with exactly 15 fields; Index contains 2,300 sequential unique paths with matching Decisions.
- Section 2251–2300 is closed (50/50); active 200-file stage 2201–2400 is 100/200 complete.
- Tests/examples inspected only; none executed. No app source changes or runtime/build/device/game validation.
- **STOP at 2300**. No authorization exists for 2301+ until the user explicitly grants it.
