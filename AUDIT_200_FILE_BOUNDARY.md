# RustChunk — 200-File Audit Boundary Register

- Repository `imamirhosein/RustChunk`; branch `starling-1.20.1-port-audit`.
- Pinned upstream `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`; submodule `upstream/starling`.
- Entries 1–2000 globally verified/backed up; entries 2001–2775 inspected and committed; active 200-file stage is **2601–2800 (175/200)** after checkpoint 2751–2775.
- Sections **2601–2650**, **2651–2700** and **2701–2750** are closed. Section **2751–2800** is 25/50 complete: checkpoint 2751–2775 is complete, and 2776–2800 remains authorized under the same one-time grant.

- Historical note: the 2401–2600 stage was fully QA-verified and closed after its lexical-order correction. The earlier 2601–2700 grant is also closed; the active 2601–2800 stage and current exceptional grant 2701–2800 are stated below.
- Keep 25-file commits, 50-file reports, max 400-line chunks and one source file at a time. No source changes; next backup after 3000/global verification.

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


## Current stage partition — 2401–2600

## Historical authorization snapshot — superseded by the current boundary below

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 2401–2450 | 2401–2425 — COMPLETE | 2426–2450 — COMPLETE | CLOSED — 50/50 |
| 2451–2500 | 2451–2475 — COMPLETE | 2476–2500 — COMPLETE | CLOSED — 50/50 |
| 2501–2550 | NOT AUTHORIZED | NOT AUTHORIZED | WAIT FOR FRESH PERMISSION |
| 2551–2600 | NOT AUTHORIZED | NOT AUTHORIZED | WAIT FOR FRESH PERMISSION |

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


## Decision correction and next authorization — 2026-11-21

- Section 2251–2300 is closed (50/50); stale header status corrected.
- Decisions 2276–2300 standardized to adaptable; reference in Ledger/Index. Fixture-only limits remain in supporting text.
- decoration_region_fixture.rs is 303 canonical lines, read in ranges 1–200 and 201–303.
- Historical authorization snapshot superseded by the later one-time extension through 2400. Current state: 2301–2350 closed; 2351–2400 authorized; stop at 2400.


## Ledger partition correction — 2026-11-21

- Entries 2201–2300 had been stored in the prior stage's `AUDIT_LEDGER_2001_2200.md`. The Ledger was split at entry 2201 without changing any entry content or number.
- Closed Ledger: `AUDIT_LEDGER_2001_2200.md` — 2001–2200 (200/200). Active Ledger: `AUDIT_LEDGER_2201_2400.md` — 2201–2300 (100/200).
- Index remains 2,300 sequential unique paths. All 300 Ledger records pass exact 15-field and path/Decision alignment checks.
- Historical authorization snapshot superseded by the later explicit extension through 2400. Current state: 2301–2350 closed; 2351–2400 authorized; stop at 2400.

## Checkpoint 2301–2325 — COMPLETE (25/25)

- All 25 files were inspected individually through EOF in deterministic lexical order after the full Index boundary. Exact paths, SHAs and byte sizes match the complete pinned tree.
- Long-file coverage: `inventory_pickup_fixture.rs` 1–400 and 401–410; `inventory_use_start_fixture.rs` 1–400 and 401–436.
- Active Ledger: 125 records (2201–2325), exactly 15 fields each. Index: 2,325 sequential unique paths with matching Decisions.
- These are synthetic test transports for fluid/food/heightmaps/interaction shapes/inventory/item use, not production game systems. Several trust fixture IDs/arrays, narrow numeric values or use fixed mock inventories. Enchantment durability changes are not covered by the durability fixture.
- The one-time expanded authorization covers **2301–2400**. Section 2301–2350 is now closed; continue 2351–2375 and 2376–2400 without pausing at 2375. **STOP at 2400**; no authorization exists for 2401+.


## Checkpoint 2326–2350 — COMPLETE (25/25)

- All 25 files were inspected individually through EOF in deterministic lexical order after the full Index. Exact paths, SHAs and byte sizes match the complete pinned tree.
- Long-file coverage: `live_attribute_updates_fixture.rs` 1–400/401–598; `material_condition_fixture.rs` 1–400/401–735; `material_stage_fixture.rs` 1–400/401–800/801–828.
- Active Ledger: 150 records (2201–2350), exactly 15 fields each. Index: 2,350 sequential unique paths with matching Decisions.
- These are mock/differential fixtures for item, Java maps, level, living entity, material stage, movement and object-map behavior. The invalid equal-identity comparator, empty-sequence panic paths, unchecked indexes/casts and synthetic registry/world assumptions are recorded in Ledger/QA.
- Section **2301–2350 is CLOSED (50/50)**. The user's one-time authorization remains active for **2351–2400**, covering two 25-file checkpoints. Do not pause at 2375; **STOP at 2400**. No authorization exists for 2401+.


## Checkpoint 2351–2375 — COMPLETE (25/25)

- All 25 files were inspected individually through EOF in deterministic lexical order after the full Index boundary. Paths, SHAs and sizes match the pinned tree.
- Long-file coverage: `registered_player_combat_fixture.rs` 1–400/401–609; `registry_collision_scan_fixture.rs` 1–400/401–591; `server_player_combat_fixture.rs` 1–400/401–629; `simple_block_fixture.rs` 1–400/401–773.
- Active Ledger: 175 records (2201–2375), exactly 15 fields each. Index: 2,375 sequential unique paths and matching Decisions.
- Fixtures are synthetic reference transports for piston/player/portal/registry/placement/simple-block/container behavior. Their fixed mocks, trusted input indexes, unavailable height branches and no-op/panic adapters are documented in Ledger/QA; they do not prove full vanilla 1.20.1 or Forge runtime behavior.
- The one-time authorization covers the remaining checkpoint **2376–2400**. It was re-derived after explicitly excluding the 25 in-flight paths; no overlap with 2351–2375. Complete it and **STOP at 2400**. No authorization exists for 2401+.


## Checkpoint 2376–2400 — COMPLETE (25/25); stage 2201–2400 CLOSED

- All 25 source files were inspected individually through EOF in deterministic lexical path order after the full Index. Paths, SHAs and exact byte sizes match the complete pinned tree.
- Long-file coverage: `state_provider_fixture.rs` 1–400/401–530; `stored_consume_fixture.rs` 1–400/401–407; `stored_effect_attributes_fixture.rs` 1–400/401–483; `tree_feature_fixture.rs` 1–400/401–672.
- Active Ledger: 200 entries (2201–2400), exactly 15 fields each. Index: 2,400 sequential unique paths and matching Decisions.
- These are synthetic fixtures/helpers, not production gameplay. Limitations include comparator equality, stubbed effect notifications, private resource snapshots, unchecked helper conversions, fixed mock positions/dimensions and corpus-specific tick counts.
- `use_flag_fixture.rs` was not in the pinned tree and was excluded; actual 2398 is `use_flag_reaction_fixture.rs`.
- The one-time authorization through 2400 is exhausted. **STOP before 2401**; no authorization exists for later paths.
- No tests/examples executed, no application source changed, and no build/benchmark/APK/device/renderer/Minecraft/Forge/server/Aternos validation. No new backup at this boundary.


## Checkpoint 2401–2425 — COMPLETE (25/25)

- All 25 files inspected through EOF in lexical order after excluding Index 1–2400; SHAs/sizes match pinned tree.
- Long-file ranges <=400: attachments.rs 1–400/401–695; live.rs 1–400/401–416; worldgen_parity.rs 1–200/201–400/401–600/601–800/801–915; block_landing.rs 1–400/401–464.
- Active Ledger 25 records, 15 fields; Index 2,425 sequential unique paths.
- 2401–2500 exception active; 2426–2450 authorized. Report at 2450, continue to 2500, then stop.
- No source changes, tests or build/runtime validation; no backup at 2425.


## Checkpoint 2426–2450 — COMPLETE (25/25); section 2401–2450 CLOSED

- All 25 files read through EOF; paths/SHAs/sizes match pinned tree.
- Long files <=400: block_state_properties.rs 1–400/401–418; block_survival.rs 1–400/401–600/601–800/801–1200/1201–1220; block_state_data.txt 1–400/401–800/801–1200/1201–1600/1601–1703; chunk_leaves.rs 1–400/401–488; files.rs 1–200/201–400/401–600/601–800/801–1000/1001–1147; fixes_a.rs 1–200/201–400/401–600/601–800/801–1000/1001–1116.
- Active Ledger 50 entries with 15 fields; Index 2,450 sequential unique paths.
- Authorization 2401–2500 remains active; continue 2451–2475; report at 2500 and stop.
- No source changes, tests or build/runtime validation; no backup at 2450.


## Checkpoint 2451–2475 — COMPLETE (25/25)

- All 25 files inspected through EOF in lexical order after excluding Index/in-flight paths; SHAs/sizes match pinned tree.
- Long files <=400 lines: fixes_b, fixes_c, fixes_cliffs, fixes_flat, fixes_legacy, fixes_mid, fixes_old, flat_tables, items, json, schema_flat, schema and entity_fluid.
- Active Ledger 75 entries with 15 fields; Index 2,475 sequential unique paths.
- One-time authorization 2401–2500 remains active; next 2476–2500 authorized; report and stop at 2500.
- No source changes, tests/build/runtime validation or backup at 2475.


## Checkpoint 2476–2500 — COMPLETE (25/25); one-time grant exhausted

- All 25 files inspected through EOF; path/SHA/size match pinned tree.
- Long files <=400: entity_fluid.rs 1–400/401–454; inventory_use.rs 1–400/401–485.
- Active Ledger 100 entries with 15 fields; Index 2,500 sequential unique paths.
- One-time authorization 2401–2500 fully exhausted. STOP at 2500; 2501+ requires fresh permission.
- No source changes, tests/build/runtime validation or backup at 2500.


## Stage 2401–2600 — authorization and section register

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 2401–2450 | 2401–2425 — COMPLETE | 2426–2450 — COMPLETE | CLOSED — 50/50 |
| 2451–2500 | 2451–2475 — COMPLETE | 2476–2500 — COMPLETE | CLOSED — 50/50 |
| 2501–2550 | 2501–2525 — COMPLETE | 2526–2550 — COMPLETE | CLOSED — 50/50 |
| 2551–2600 | 2551–2575 — COMPLETE | 2576–2600 — COMPLETE | CLOSED — 50/50 |

- The earlier 2550 stop note below records the prior boundary decision. It is superseded by the user's later explicit advance authorization for 2551–2600.
- After the separately authorized 2551–2600 section is complete, perform independent structural/provenance QA across all 200 Ledger entries 2401–2600.


## Registration correction — 2526–2550 — 2026-11-21

- The first commit's post-commit check found the 25 new Index rows without their Ledger records. That commit was not accepted as completion.
- The corrective atomic six-document commit supplies the missing 25 full Ledger entries. The boundary is accepted only after reading back all six documents from the corrective commit and verifying 150 Ledger records, 2,550 unique sequential Index rows and all 25 path/Decision links.
- At the time of this correction, work stopped at 2550 pending a separate grant. The later advance authorization for 2551–2600 is recorded in the current register below.


## Authoritative current register — 2026-11-21

This table supersedes earlier in-file OPEN/AUTHORIZED snapshots for the same stage.

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 2401–2450 | 2401–2425 — COMPLETE | 2426–2450 — COMPLETE | CLOSED — 50/50 |
| 2451–2500 | 2451–2475 — COMPLETE | 2476–2500 — COMPLETE | CLOSED — 50/50 |
| 2501–2550 | 2501–2525 — COMPLETE | 2526–2550 — COMPLETE | CLOSED — 50/50 |
| 2551–2600 | 2551–2575 — COMPLETE | 2576–2600 — COMPLETE | CLOSED — 50/50 |

**Both checkpoints 2551–2575 and 2576–2600 are complete. Their entries have been normalized to strict raw lexical path order; independent global QA passed. Stop at 2600.**


## Checkpoint 2551–2575 — COMPLETE

- Ledger: 175 entries. Index: 2,575 sequential unique paths.
- The 2551–2575 checkpoint is closed. The remaining 2576–2600 checkpoint is pre-authorized.
- Stop at entry 2600, then run the full 2401–2600 structural/provenance QA.


## Checkpoint 2576–2600 — COMPLETE

- Ledger: 200 entries. Index: 2,600 sequential unique paths.
- All four 50-file sections in 2401–2600 are closed.
- Source inspection stops at 2600. Complete independent second-pass QA across all 200 entries before final acceptance.


## Final QA — 2401–2600 — COMPLETE

- 200 Ledger entries, exact 15-field schemas, and 2,600 sequential unique Index paths.
- All 200 source path/SHA/byte-size values match the pinned, untruncated tree; Ledger and Index paths/Decisions align; Coverage and Completion line counts match.
- The active 2401–2600 entries and Index rows are normalized to strict raw lexical path order with corrected sequential numbering.
- Stage closed at 200/200. Stop at 2600; fresh authorization is required for 2601+.


## Active stage 2601–2800 — exceptional extension in progress

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 2601–2650 | 2601–2625 — **COMPLETE** | 2626–2650 — **COMPLETE** | **CLOSED — 50/50** |
| 2651–2700 | 2651–2675 — **COMPLETE** | 2676–2700 — **COMPLETE** | **CLOSED — 50/50** |
| 2701–2750 | 2701–2725 — **COMPLETE** | 2726–2750 — **COMPLETE** | **CLOSED — 50/50** |
| 2751–2800 | 2751–2775 — **COMPLETE** | 2776–2800 — **PRE-AUTHORIZED** | **IN PROGRESS — 25/50** |

- The user explicitly renewed one exceptional grant for exactly **2701–2800**. It covers both remaining 25-file checkpoints; do not stop for permission at 2775.
- Keep the normal six-document atomic commit/read-back process after each 25-file checkpoint. The active Ledger is `AUDIT_LEDGER_2601_2800.md`; current Ledger count is **175 entries (2601–2775)** and the Index has **2,775 sequential unique paths**.
- The complete-tree scan at 2701 found 75 previously unindexed Rust paths lexically before/equal to the old Index cutoff; entries 2701–2775 now cover all **75** of those paths. Continue selecting from the full pinned tree after excluding the complete current Index.
- **STOP after entry 2800.** Entry 2801+ is outside this grant. Keep `main` and all backup refs unchanged. No tests/build/runtime claims unless actually executed.

## Checkpoint 2676–2700 — COMPLETE (25/25)

- All 25 source paths were selected as the next strict raw-lexical unindexed candidates and inspected individually through EOF at the pinned Starling snapshot; path, SHA and exact Git tree byte size match.
- Long files were read in consecutive, gap-free chunks no larger than 400 lines: `world_io.rs` 1–400/401–413; account `files.rs` 1–400/401–436; account `lib.rs` 1–400/401–800/801–875; account `tests.rs` 1–400/401–473; catalog `lib.rs` 1–400/401–432.
- Ledger now has 100 entries (2601–2700), all with exactly 15 required fields. Index has 2,700 sequential unique paths and aligned Decisions.
- Section 2651–2700 is closed at 50/50. The one-time exceptional authorization for 2601–2700 is fully consumed.
- **STOP at 2700. Entry 2701+ is not authorized.** No tests/examples were run; no product source or runtime validation was performed.


## Provenance read-back correction — entry 2682

- Corrected the Ledger's source SHA transcription to `23553e6b78d9345806cac3abcc2df5636fd38098`; pinned tree size remains 3,904 bytes and Coverage/Completion remain 87 logical lines.
- The correction is persisted atomically with the other audit documents. No source file changed and no test was run.

## Checkpoint 2701–2725 — COMPLETE (25/25)

- All 25 paths are the first globally sorted unindexed Rust blobs after excluding the full 2,700-row Index from the complete pinned tree. This corrects the prior omission of 75 unindexed paths that sorted before/equal to the old last Index path; no historical numbering was changed.
- All 25 files were read through EOF at pinned source `5432370d573642041585c69b40fb045b27133488`; each SHA and exact byte size matches the tree. Long files use consecutive, gap-free ranges no larger than 400 lines: sculk.rs 1–400/401–727; state_provider.rs 1–400/401–695; structure.rs 1–400/401–409; end_city.rs 1–400/401–467; formations.rs 1–400/401–405; fortress.rs 1–400/401–800/801–935; jigsaw.rs 1–400/401–800/801–1200/1201–1579.
- Ledger now has **125 records (2601–2725)** and the Index has **2,725 sequential unique paths**. All 15 fields, paths and Decisions align.
- The 2701–2750 section is 25/50 complete; **2726–2750 is already authorized**. The same exceptional authorization also covers 2751–2800. **STOP only at 2800.**
- No source code or runtime validation was performed; embedded tests were inspected only.

## Checkpoint 2726–2750 — COMPLETE (25/25)

- All 25 paths are exactly the next globally sorted unindexed Rust blobs after excluding the complete 2,725-row Index from the untruncated pinned tree.
- All 25 files were read through EOF at pinned source `5432370d573642041585c69b40fb045b27133488`; each source SHA and exact byte size matches the tree. Long-file reads were sequential and gap-free in chunks no larger than 400 lines; exact ranges are recorded in the QA Log.
- Ledger now has **150 entries (2601–2750)** and the Index has **2,750 sequential unique paths**. All 15 fields, paths and Decisions align.
- Section **2701–2750 is CLOSED (50/50)**. The same exceptional authorization covers 2751–2800; continue with 2751–2775 without another permission request. **STOP only at 2800.**
- No product source code changed; embedded tests/examples were not executed and no runtime validation was performed.


## Checkpoint 2751–2775 — COMPLETE (25/25)

- All 25 paths are the next strict raw-lexical unindexed Rust candidates after excluding the complete 2,750-row Index from the complete pinned tree. The 75-path earlier-lexical-order blind spot identified at entry 2701 is now fully exhausted; no historical numbering was changed.
- All 25 files were read through EOF at pinned source `5432370d573642041585c69b40fb045b27133488`; each source SHA, exact byte size and canonical logical line count matches the complete tree and EOF coverage. Long files were read sequentially in contiguous chunks no larger than 400 lines.
- Ledger now has **175 records (2601–2775)** and the Index has **2,775 sequential unique paths**. Each new record has exactly 15 required fields; paths and Decisions align.
- The 2726–2750 checkpoint was re-fetched and its 25 files re-verified for source SHA, byte size, EOF line count and Ledger field structure. Content-level finding refinements were made in Ledger entries 2730, 2735 and 2737.
- Section **2751–2800** remains authorized under the same exceptional grant; continue with **2776–2800** without another permission request. **STOP only after entry 2800.**
- No source code changed and no tests/examples were executed; no build or runtime validation was performed.
