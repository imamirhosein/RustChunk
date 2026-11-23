# RustChunk Audit Continuity

Last reconciled: 2026-11-23
Repository: imamirhosein/RustChunk
Branch: starling-1.20.1-port-audit

## Authoritative resume point

- Completed entries: **1–3000**.
- Stage **2601–2800 is CLOSED (200/200)**. Stage **2801–3000 is CLOSED (200/200)**.
- Sections **2801–2850** and **2851–2900** are **CLOSED (50/50)**; section **2901–2950 is CLOSED (50/50)**. Checkpoints 2901–2925 and 2926–2950 are committed and read-back verified. Section **2951–3000 is CLOSED (50/50)**. Checkpoints 2951–2975 and 2976–3000 are committed and read-back verified.
- The user's one-time exceptional authorization **2901–3000 (100 files) is EXHAUSTED**. **STOP at 3000**; 3001+ requires fresh explicit authorization.
- Active Ledger `AUDIT_LEDGER_2801_3000.md`: **200 records (2801–3000)**, each with exactly 15 fields. Index: **3,000 sequential unique paths**, no duplicates and Ledger/Index path/Decision alignment.
- Pinned source `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`; use this immutable snapshot only.
- The complete recursive source tree is untruncated (5,749 entries). New paths were selected from the complete tree after excluding the full Index.
- Global structural/provenance QA for entries 1–3000 is complete on 2026-11-23. All 3,000 source SHA/byte-size pairs match the pinned tree after the entry 174 correction; all 15 fields, primary Decision labels, Index alignment and recorded Coverage/Completion ranges were reconciled. Tests/examples were inspected only, not executed. No application source changes, builds, APK/device/renderer/Minecraft/Forge/server/Aternos validation.

## Global QA closeout — entries 1–3000 — COMPLETE (2026-11-23)

- 3,000 sequential unique Index paths; all Ledger/Index paths align.
- All 3,000 source paths exist in the complete pinned tree; all source SHA/byte-size pairs match after correcting entry 174.
- All 3,000 Ledger entries have exactly 15 fields, with no undefined placeholders. Primary Decision labels are first and align with Index rows.
- Entry 2542's missing 401–600 range was re-fetched; entry 1429's long-file coverage was expanded to explicit ranges.
- 28 shifted records and the separate entry 1397 Evidence/Completion alignment were corrected; two undefined evidence values were filled.
- No app source changed. No build, APK, Android-device, renderer, Minecraft/Forge or multiplayer validation was performed.

## Source pin and selection

- Upstream: starling-client/starling, ref `starling`.
- Pinned commit: `5432370d573642041585c69b40fb045b27133488`.
- Inspect only this exact snapshot.
- Build the recursive source tree and verify it is complete/not truncated.
- Select paths in deterministic lexical order, excluding every path already in the full Index. Never repeat or renumber a source path.

- Index order reconciliation: 70 adjacent transitions relative to the pinned GitHub recursive-tree API order were found among the 2,450 stable Index rows: 69 occur in entries 1–2000, and one occurs at entry 2033. No additional raw-tree-position regressions occur from entry 2034 through 2450. This is a historical ordering divergence, not a missing-file, duplicate-path or source-provenance defect: all 2,450 Index paths are unique and present in the pinned tree. The audit IDs are stable historical chronology and must not be renumbered because Ledger, QA and cross-document references depend on them. Do not describe the whole Index as a literal raw recursive-tree traversal.
- Ordering correction at the 2700 boundary: a complete pinned-tree scan found 75 unindexed Rust paths lexically before/equal to `crates/starling-catalog/src/list.rs`. Entries 2701–2750 cover the first 50 in raw lexical order; **25 remain** in that earlier-path set. The next globally sorted unindexed path is `crates/horizon-world/src/worldgen/tree_blocks.rs` (entry 2751). Always sort the full remaining source set after excluding every indexed path.
- Verify every source SHA and byte size against the pinned tree.

## Per-file inspection requirements

- Read every file fully through EOF.
- For files over 400 logical lines, use consecutive, gap-free ranges of no more than 400 lines each.
- Record canonical logical line count, byte size, SHA and exact coverage.
- Canonical line count: LF-terminated line count, plus one only when the blob does not end in LF; do not count a synthetic empty split item after a final LF.
- Each Ledger entry must contain exactly these 15 fields, each once: Source repository/ref; Source SHA; Size; Coverage; Purpose; Dependencies / cross-references; Implementation / logic; Findings; RustChunk relevance; Decision; 1.20.1 implications; Follow-up checks; RustChunk implementation mapping; Evidence required before adoption; Completion state.
- Decision primary labels: essential, adaptable, rewrite required, integrate, unnecessary, undetermined. Qualifiers: reference, version-specific, optional/future.
- Separate observed facts, interpretation, suspected defects and proposed target-version work. Do not claim a suspected defect is reproduced without evidence.

## Six-document checkpoint persistence

At every 25-file checkpoint, update together:
1. The applicable Ledger.
2. AUDIT_INDEX.md.
3. AUDIT_QA_LOG.md.
4. AUDIT_CONTINUITY.md.
5. AUDIT_PROTOCOL.md whenever current state or a governing rule needs synchronization.
6. AUDIT_200_FILE_BOUNDARY.md while the 200-file stage is active.

Validate sequential numbering, global path uniqueness, all 15 fields, pinned SHA/byte size, complete EOF coverage, Index/Ledger alignment and correct resume state. Commit all six documents together in one commit. Re-fetch all six from GitHub and verify saved contents/blob SHAs before calling the checkpoint complete.

Before each new chat continuation, re-fetch the latest branch head and read this file plus the latest audit documents. If context is uncertain, stop and reconcile rather than guessing.

## Main project boundary

- Main RustChunk development remains on `main`; Starling audit artifacts remain on the audit branch.
- The audit is research and compatibility assessment, not proof that features have been implemented.
- Reading embedded tests is not running them.
- Never claim build, unit-test, benchmark, APK build, Forge launch, Android-device validation, multiplayer join, LAN, UPnP or relay tests unless actually executed.
- Starling/Horizon is based on later Minecraft 26.3 behavior. Do not treat its registries, packets, IDs, assets or native mod API as Forge 1.20.1-compatible without separate target-era evidence.
- The Horizon native-mod build pipeline and app-library replacement are high-privilege mechanisms; do not treat them as normal Forge mod loading.

## Backup state

- Global structural/provenance QA for entries **1–3000** is complete on 2026-11-23.
- Current preserved backup refs, confirmed from the GitHub branch list:
  - `backup/audit-1000-final-2026-11-19` -> `4ba78eba6ebc2c89bb455a68a3cb9ddfe5f30e23`.
  - `backup/audit-2000-final-2026-11-21` -> `f0d7c32d0bdcf70fa30a88e54f049fe08dfca283`.
- Historical `backup/audit-1000-unique-2026-11-18` is absent from the current branch list; do not recreate it.
- The final 3,000-entry snapshot branch is `backup/audit-3000-final-2026-11-23`; it must point exactly to the verified global-QA commit.
- Backup refs are procedurally read-only; GitHub reports branch protection disabled. Do not modify the existing 1,000- or 2,000-entry backup refs.
- The next backup is due only after entries **3001–4000** are authorized, completed, committed and globally verified.

## Historical completed-work snapshot — entries 1–1800

This is a preserved snapshot from the earlier 1–1800 boundary; the authoritative current counts and resume point are at the top of this file.

- Entries 1–1000: complete and historically QA-reviewed.
- Entries 1001–1200, 1201–1400 and 1401–1600: complete; prior stages closed.
- Entries 1601–1800: complete in the active Ledger; reporting sections 1601–1650, 1651–1700, 1701–1750 and 1751–1800 are closed.
- Index: 1,800 sequential entries and 1,800 unique paths.
- Active Ledger: 200 entries (1601–1800), each with exactly 15 required fields.
- Source SHAs, byte sizes and complete EOF coverage for entries 1776–1800 are recorded in the active Ledger and were verified against the pinned source tree.
- No source or application code was modified. Embedded tests were inspected only, not run. No build, tests, benchmark, APK, device, Forge or server validation was performed.

## Historical duplicate corrections

- Entries 631, 632, 633, 754, 762 and 766 were corrected to unique paths: pick_block_data_fixture.rs, place_recipe_fixture.rs, player_sync_fixture.rs, raid_fixture.rs, random_tick_sections_probe.rs and recipe_book_fixture.rs.
- Entry 1089 was corrected to `crates/horizon-client/src/game/blockctl.rs`; entry 121 remains the only `entry.rs` occurrence.


## Work pacing and chat-report rule

- Work serially: one file or one independent check at a time. Finish it before starting another; do not parallelize unrelated reads or validations.
- For files over 400 lines, read consecutive ranges sequentially, each at most 400 lines. Do not leave a file half-read while moving to another.
- Keep operations bounded and avoid long, bundled actions that make the user think the session is stuck. Give only a brief current-item status when a delay makes that useful; do not send filler updates.
- Chat reports stay minimal: completed range/status, verification result or blocker, and exact next point/authorization. Do not volunteer file-by-file descriptions, architecture recaps, file-dependency explanations or internal change lists unless the user asks.
- Keep detailed evidence in the audit documents rather than repeating it in chat. At 25-file commits, a short confirmation is enough; after each 50-file section, report briefly and STOP for approval.
- On interruption or chat change, resume from the latest verified commit and this Continuity record; never repeat completed work or infer an unverified state.


## Latest verified checkpoint — 1301–1325

- Branch head before this checkpoint commit: `62695e4d2f1d41630f4ba90c792ee0569c7d3b4b`.
- Completed new entries: 1301–1325 inclusive (25/25).
- Index: 1325 sequential entries and 1325 unique source paths.
- Active Ledger: entries 1201–1325, 125 records; every entry has exactly 15 required fields.
- Current authorized section: 1301–1350. The second checkpoint 1326–1350 is already authorized; continue without an interim permission request.
- Stop after 1350 and wait for explicit authorization before 1351–1400.


## Latest verified checkpoint — 1326–1350

- Branch head before this checkpoint commit: `193fc62bd995034b159f82b526292b990b8174c5`.
- Completed new entries: 1326–1350 inclusive (25/25).
- Reporting section 1301–1350 is now complete (50/50).
- Index: 1350 sequential entries and 1350 unique source paths.
- Active Ledger: entries 1201–1350, 150 records; every entry has exactly 15 required fields.
- Current authorization state: STOP. The next range 1351–1400 requires a separate explicit user instruction.


## Duplicate-path correction — checkpoint 1326–1350

- Read-back QA found the initially recorded entry 1349 path `crates/horizon-client/src/lib.rs` duplicated entry 118. Entry 118 remains the only `lib.rs` audit entry.
- Corrected 1349 to `crates/horizon-client/src/locator_bar.rs` and added `crates/horizon-client/src/lod_net.rs` as entry 1350, fully reviewed to EOF.
- The final Index is 1350 sequential entries and 1350 unique source paths. The active Ledger contains 150 entries, 1201–1350, with exactly 15 fields per entry.
- Reporting section 1301–1350 remains complete. STOP at 1350; do not begin 1351–1400 until explicit user authorization.

## Latest verified checkpoint — 1351–1375

- Branch head before checkpoint work: 38dab3b1018f4b023c22dcb549da1b1024588f4a.
- QA checkpoint record commit: 9098ab72dabbea4ca59d5ace32460e511b8127bb; protocol update commit: e07a228ee304b36fe0002a394103ad64c010c6f7.
- Completed entries: 1351–1375 inclusive (25/25).
- Index: 1375 sequential entries and 1375 unique source paths.
- Active Ledger: entries 1201–1375, 175 records; each has exactly 15 required fields.
- QA: checkpoint 1351–1375 recorded; the symlink at 1351 and complete 10,360-line native_hooks.rs inspection are explicitly noted.
- Pinned upstream remains starling-client/starling @ 5432370d573642041585c69b40fb045b27133488.
- Authorization: the user's “start” instruction authorized the full section 1351–1400. Continue with 1376–1400 without asking for another permission at 1375.
- Next checkpoint: 1376–1400. Stop after entry 1400 and wait for explicit authorization for 1401–1450.
- No builds/tests/device checks were run. main and backups remain untouched; next backup remains governed by the existing 1000-entry cadence.

- Final read-back QA: Index/Ledger paths and decisions align for all 25 new entries; the full Index remains sequential and unique, and all 15 Ledger fields are present exactly once.

## Previous stage closure — 1400 complete

- Entries 1–1400 were complete before the current stage began.
- Prior 1201–1400 Ledger/Index alignment, 15-field count and global unique-path checks were verified.
- Prior final QA/alignment commit: `90d23fa8ccd599d4c8726b08849689e903f374fc`; branch head before this checkpoint: `6f6235a17c7ccf8abc7033400c81c5b44089840b`.

## Latest verified boundary — 1450

- Completed entries: **1401–1450 inclusive (50/50)**.
- Checkpoint 1401–1425 and checkpoint 1426–1450 are both complete, committed and read-back verified.
- Active Ledger: `AUDIT_LEDGER_1401_1600.md`, 50 entries, each with the 15 required fields exactly once.
- Index: **1,450 sequential entries and 1,450 unique paths**.
- All 50 source SHAs, byte sizes, logical line counts and EOF coverages align with the pinned upstream tree.
- Index/Ledger path and Decision alignment verified for entries 1401–1450.
- **STOP.** Next proposed section is 1451–1500; wait for explicit user authorization before inspecting it.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- No tests/builds/device/server validation was run. `main` and backups remain untouched; next backup remains at the 1000-entry cadence.


## Latest verified checkpoint — 1451–1475

- Completed entries: **1451–1475 inclusive (25/25)** from the pinned source snapshot.
- All 25 files were read completely through EOF. Files over 400 lines were read in consecutive ranges no larger than 400; exact ranges and logical line counts are in the Ledger.
- Source SHAs and byte sizes match the pinned tree. All 25 Ledger records contain the 15 required fields exactly once.
- Index: **1,475 sequential entries and 1,475 unique paths**; Index/Ledger path alignment for 1451–1475 is verified. A semantic field-value shift was corrected and Decision values realigned to Index in the 1476–1500 QA transaction.
- Active Ledger: **75 entries**, 1401–1475.
- The user's authorization covers **1451–1500**. Continue with **1476–1500**, then stop for authorization.
- No source or application code changed. Embedded tests were inspected but not run. No build/tests/APK/device/Forge/server validation was performed.
- Checkpoint commit: `69e45d02a0aee19f701fa8e6b5c6361766833746`; all five-document read-backs were verified.
- `main` and backups remain untouched; next backup remains at the 1000-entry cadence.


## Closed reporting section — 1451–1500 — 2026-11-19

- Completed entries: **1451–1500 inclusive (50/50)** in two 25-file checkpoints.
- Active Ledger: **100 entries**, 1401–1500, each with exactly 15 required fields.
- Index: **1,500 sequential entries and 1,500 unique source paths**.
- The 1451–1475 field-value shift was corrected; Index and Ledger decisions align for 1451–1500.
- Source SHAs, byte sizes and full EOF coverage are recorded and verified for the new checkpoint.
- No source/application code changed. Embedded tests were inspected but not run. No build, test, APK, device, Forge or server validation occurred.
- **STOP at 1500.** Next proposed section: 1501–1550; wait for explicit user authorization.
- Active 200-file stage 1401–1600 is **100/200 complete**. Backup remains due only after entries 1001–2000 are complete and globally verified. Main and backup refs remain untouched.


## Latest verified checkpoint — 1501–1525 — 2026-11-19

- Completed entries: **1501–1525 inclusive (25/25)**.
- Active 200-file stage: **1401–1600**, now **125/200 complete**.
- Section **1501–1550** is authorized; checkpoint **1501–1525** is complete and **1526–1550** remains within authorization.
- Active Ledger: **125 entries**, 1401–1525, 15 required fields per entry.
- Index: **1,525 sequential entries and 1,525 unique source paths**.
- All 25 source SHAs/byte sizes match the pinned tree; complete EOF coverage is recorded. Index/Ledger path and Decision alignment is verified for 1501–1525.
- Correction recorded: Tadpole is 1.20.1 content; its later brain redesign is excluded, while target goal AI must be retained. Copper Golem/Happy Ghast branches are excluded.
- No application/source code changed. Embedded tests were inspected but not run. No build/test/APK/device/Forge/server validation.
- Commit/read-back SHA will be added after the transaction.
- Next checkpoint: **1526–1550** (authorized). Stop at 1550 and wait for explicit approval before 1551–1600.
- `main` and backup refs remain untouched; no backup is due.


## Closed reporting section — 1501–1550 — 2026-11-19

- Completed entries: **1501–1550 inclusive (50/50)** in two 25-file checkpoints: 1501–1525 and 1526–1550.
- Active 200-file stage: **1401–1600**, now **150/200 complete**.
- Active Ledger: **150 entries**, 1401–1550, each with exactly 15 required fields.
- Index: **1,550 sequential entries and 1,550 unique source paths**.
- All 50 source SHAs/byte sizes match the pinned source tree; exact logical line counts and contiguous EOF coverage are recorded. Index/Ledger path and Decision alignment is checked for 1501–1550.
- The prior version note remains: Tadpole itself is 1.20.1 content; only its newer brain redesign and other later species branches are excluded.
- No source/application code changed. Embedded tests were inspected but not run. No build, test, APK, device, Forge or server validation was performed.
- Checkpoint 1501–1525 commit: `f01cc32175cc1e8d2f6fdda573e214232d2b1eb4`.
- Checkpoint 1526–1550 commit SHA will be added after commit/read-back.
- **STOP at 1550.** Next section 1551–1600 requires explicit user authorization. `main` and backup refs remain untouched; no backup is due at this boundary.

## Latest verified checkpoint — 1551–1575 — 2026-11-19

- Completed entries: **1551–1575 inclusive (25/25)** from pinned source `5432370d573642041585c69b40fb045b27133488`.
- Active 200-file stage: **1401–1600**, now **175/200 complete**.
- Active 50-file section: **1551–1600**. The first checkpoint, **1551–1575**, is complete; the second checkpoint, **1576–1600**, remains covered by the same explicit user authorization. Do not ask for intermediate authorization.
- Active Ledger: **175 entries**, 1401–1575, each with exactly 15 fields. Index: **1,575 sequential entries and 1,575 unique paths**.
- All 25 files were read completely through EOF. Exact line counts, contiguous coverage, source SHAs and byte sizes are in the Ledger and match the pinned tree.
- Index/Ledger path and Decision alignment is verified for 1551–1575.
- Findings: target-era mob behaviors remain candidates for adaptation after 1.20.1 parity checks; Happy Ghast, Breeze/Creaking Brain and Nautilus are excluded. Mob physics and A* navigation are core references but are not yet implemented or benchmarked.
- No source/application code changed. Embedded tests were inspected but not run. No build, tests, benchmark, APK, device, Forge or server validation was performed.
- Checkpoint commit SHA will be added after atomic five-document commit and read-back.
- **Next: inspect 1576–1600**, serially and with maximum 400-line chunks. Stop at **1600** and wait for fresh authorization.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.

## Closed reporting section — 1551–1600 — 2026-11-19

- Completed entries: **1551–1600 inclusive (50/50)** in two 25-file checkpoints: 1551–1575 and 1576–1600.
- Active 200-file stage: **1401–1600**, now **200/200 complete**.
- Active Ledger: `AUDIT_LEDGER_1401_1600.md`, **200 entries** (1401–1600), each with exactly 15 semantically aligned fields.
- Index: **1,600 sequential entries and 1,600 unique source paths**.
- All 50 files in this section were read through EOF, with maximum 400-line contiguous chunks. Source SHAs/sizes and coverage match the pinned tree.
- QA correction: the 1551–1575 Ledger records had 15 labels but omitted a separate Purpose and shifted subsequent semantic values. This transaction repaired them using the corresponding Index descriptions as Purpose and re-aligned Dependencies through Evidence. Paths, SHA, sizes, coverage and Decisions were preserved.
- Target-era behavior remains a candidate for exact 1.20.1 reconstruction; Spear, Sulfur Cube, Copper Golem transport and Trial Spawner are excluded. Spawn/selection/finalization are server-side; renderer, skinning, random textures and spatial indexing are optional architecture references without measured performance claims.
- No source/application code changed. Embedded tests were inspected but not run. No build, tests, benchmark, APK, device, Forge or server validation was performed.
- Checkpoint 1551–1575 commit: `ce4fd9f8711ce9cbab367e3ca637df130771d012`. This transaction closes 1576–1600 after five-document commit/read-back verification.
- **STOP at 1600.** Do not inspect 1601 or later until fresh explicit user authorization.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.

## Latest verified checkpoint — 1601–1625 — 2026-11-19

- Completed entries: **1601–1625 inclusive (25/25)** from pinned source `5432370d573642041585c69b40fb045b27133488`.
- Active 200-file stage: **1601–1800**, now **25/200 complete**.
- Active 50-file section: **1601–1650**. Checkpoint **1601–1625** is complete; checkpoint **1626–1650** remains covered by the same explicit user authorization. Do not request intermediate permission.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **25 entries** (1601–1625), each with exactly 15 fields. The previous Ledger 1401–1600 remains closed at 200/200.
- Index: **1,625 sequential entries and 1,625 unique source paths**. Index/Ledger path and Decision alignment is checked for 1601–1625.
- All 25 files were read through EOF in serial order. Files over 400 lines were read in contiguous chunks no larger than 400; SHAs, byte sizes and logical line coverage match the pinned source tree.
- Findings: target-era mob behavior and player physics need strict 1.20.1 parity; Starling Native Mod API/hooks are not Forge; content has only partial Forge/NeoForge biome-modifier handling; multiplayer friend relay/status protocol are Starling-specific. No Android performance claim is made.
- No source/application code changed. Embedded tests were inspected but not run. No build, unit tests, benchmark, APK, device, Forge or server validation was performed.
- Checkpoint commit SHA is recorded after atomic five-document commit/read-back below.
- **Next: inspect 1626–1650**, serially and with maximum 400-line chunks. Stop at **1650** and wait for fresh authorization.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.

## Latest verified checkpoint — 1626–1650 — 2026-11-19

- Completed entries: **1626–1650 inclusive (25/25)** from pinned source `5432370d573642041585c69b40fb045b27133488`.
- Active 200-file stage: **1601–1800**, now **50/200 complete**.
- Reporting section: **1601–1650**, now **50/50 closed** in two checkpoints: 1601–1625 and 1626–1650.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **50 entries** (1601–1650), each with exactly 15 fields. Index: **1,650 sequential entries and 1,650 unique paths**.
- The 25 files 1626–1650 were read through EOF; files over 400 lines were read in contiguous chunks no larger than 400. Exact source SHA, size and coverage are in the Ledger.
- Exact next-25 path order was verified against the pinned recursive tree; Index/Ledger path and Decision values align for 1626–1650.
- Findings: Starling native hooks/data are not Forge/FML; telemetry/overlay are diagnostics, not measured gains; ram_limit and random_tick_sections merit Android profiling; save.rs is 26.3 and its schema must be rebuilt for 1.20.1 while bounded I/O/LOD architecture may be studied separately.
- No source/application code changed. Embedded tests were inspected but not run. No build, unit test, benchmark, APK, device, Forge or server validation was performed.
- **STOP at 1650.** Do not inspect 1651 or later until fresh explicit user authorization.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.

## Latest verified checkpoint — 1651–1675 — 2026-11-20

- Completed entries: **1651–1675 inclusive (25/25)** from pinned source `5432370d573642041585c69b40fb045b27133488`.
- Active 200-file stage: **1601–1800**, now **75/200 complete**.
- Active 50-file section: **1651–1700**. Checkpoint **1651–1675** is complete; checkpoint **1676–1700** remains covered by the user's current authorization. Do not ask for intermediate authorization.
- Active Ledger: **75 entries**, 1601–1675, exactly 15 fields per entry. Index: **1,675 sequential entries and 1,675 unique paths**.
- All 25 files 1651–1675 were read completely through EOF. Exact contiguous coverage, line counts, SHA and byte sizes are recorded in the Ledger and match the pinned source tree.
- Index/Ledger path and Decision alignment is verified for entries 1651–1675.
- Findings: several target-era gameplay references remain candidates for adaptation after 1.20.1 parity checks; spear and Starling-specific release/account systems are excluded from the baseline. Tick membership and ticket levels are optimization references, not tested improvements.
- No source/application code changed. Embedded tests were inspected but not run. No build, tests, benchmark, APK, device, Forge or server validation was performed.
- Checkpoint commit SHA will be added after atomic five-document commit and read-back.
- **Next: inspect entry 1676**, the next unindexed blob in pinned tree order. Complete checkpoint 1676–1700; stop at **1700** and wait for fresh authorization.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.

## Latest verified checkpoint — 1676–1700 — 2026-11-20

- Completed entries: **1676–1700 inclusive (25/25)** from pinned source 5432370d573642041585c69b40fb045b27133488.
- Active 200-file stage: **1601–1800**, now **100/200 complete**.
- Section **1651–1700** is closed; both checkpoints 1651–1675 and 1676–1700 are complete.
- Active Ledger: **100 entries**, 1601–1700, exactly 15 fields each. Index: **1,700 sequential entries and 1,700 unique paths**.
- All 25 files 1676–1700 were read completely through EOF; contiguous coverage, line counts, SHA and byte sizes match the pinned source tree.
- Index/Ledger path and Decision alignment is verified for entries 1676–1700.
- Findings: worldgen and far LOD are high-value references but need strict 1.20.1 parity and isolation from gameplay; several UI, account, custom protocol and launcher modules are Starling-specific. Far mesh/lighting tests are relevant to the user's visual chunk-loading issue, but were not run.
- No source/application code changed. No build/test/benchmark/APK/device/Forge/server validation was performed.
- Checkpoint commit SHA will be added after atomic five-document commit and read-back.
- **STOP at entry 1700** and wait for fresh explicit authorization before 1701.
- main and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.

## Latest verified checkpoint — 1701–1725 — 2026-11-20

- Completed entries: **1701–1725 inclusive (25/25)** from pinned source `5432370d573642041585c69b40fb045b27133488`.
- Active 200-file stage: **1601–1800**, now **125/200 complete**.
- Active reporting section: **1701–1750**. Checkpoint **1701–1725** is complete; checkpoint **1726–1750** remains covered by the same explicit user authorization. Continue without another permission request mid-section.
- Active Ledger: **125 entries**, 1601–1725, each with exactly 15 fields. Index: **1,725 sequential entries and 1,725 unique paths**.
- All 25 files were read fully through EOF in serial order; files above 400 logical lines were read in contiguous ranges no larger than 400. Source SHA, byte size and coverage match the pinned tree.
- Index paths and Decisions align with the new Ledger entries.
- Findings: command parser/Brigadier, selectors, SNBT and block parsing are valuable architecture references, but modern item components, protocol parser IDs and newer command forms require 1.20.1 reconstruction. Tick and palette parity fixtures are useful test designs, not executed results.
- No source/application code changed. Embedded tests were inspected only, not run. No build, tests, benchmark, APK, device, Forge or server validation was performed.
- Checkpoint documentation blobs prepared for five-document commit and read-back.
- **Next: inspect entry 1726**. Complete checkpoint 1726–1750; **STOP at 1750** and wait for fresh explicit authorization.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.


## Latest verified checkpoint — 1726–1750 — 2026-11-20

- Completed entries: **1726–1750 inclusive (25/25)** from pinned source `5432370d573642041585c69b40fb045b27133488`.
- Active 200-file stage: **1601–1800**, now **150/200 complete**.
- Reporting section **1701–1750** is closed; both checkpoints 1701–1725 and 1726–1750 are complete, committed and read-back verified.
- Active Ledger: **150 entries**, 1601–1750, each with exactly 15 fields. Index: **1,750 sequential entries and 1,750 unique paths**.
- All 25 files were read fully through EOF in serial order; files above 400 logical lines were read in contiguous ranges no larger than 400. Source SHA, byte size and coverage match the pinned tree.
- Index paths and Decisions align with the new Ledger entries.
- Findings: density graph/sampler architecture and parity harnesses are useful references, but much of this tree targets Java 26.3; biome/End-island logic and newer graph semantics require reconstruction from 1.20.1 sources. Sparse/plane caches and mod terrain kernels are optional extensions, not proven vanilla behavior.
- Embedded tests and parity tools were inspected only, not executed. No source/application code changed. No build, tests, benchmark, APK, device, Forge or server validation was performed.
- Checkpoint documentation committed in one five-document transaction and read back from GitHub.
- **STOP at entry 1750**. The next proposed section is **1751–1800**; wait for explicit user authorization before inspecting it.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.


## Latest verified checkpoint — 1751–1775 — 2026-11-20

- Completed entries: **1751–1775 inclusive (25/25)** from pinned source `5432370d573642041585c69b40fb045b27133488`.
- Active 200-file stage: **1601–1800**, now **175/200 complete**.
- Reporting section **1751–1800** is active; checkpoint **1751–1775** is complete, committed and read-back verified; **1776–1800** remains covered by the same explicit authorization.
- Active Ledger: **175 entries**, 1601–1775, each with exactly 15 fields. Index: **1,775 sequential entries and 1,775 unique paths**.
- All 25 files were read fully through EOF in serial order; files above 400 logical lines were read in contiguous ranges no larger than 400. Source SHA, byte size and coverage match the pinned tree.
- Index paths and Decisions align with the new Ledger entries.
- Findings: geometry/collision, noise, density and palette implementations are substantial architecture references but the core explicitly targets Java 26.3 and is not a playable game. Android ARM64 math, 1.20.1 noise/RNG behavior and PalettedContainer thresholds all require independent target verification.
- Embedded tests and benchmarks were inspected only, not executed. No source/application code changed. No build, tests, benchmark, APK, device, Forge or server validation was performed.
- **Next: continue at entry 1776** under the current authorization; finish through **1800**, then stop for fresh authorization.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.


## Latest verified checkpoint — 1776–1800 — 2026-11-20

- Completed entries: **1776–1800 inclusive (25/25)** from pinned source `5432370d573642041585c69b40fb045b27133488`.
- Active 200-file stage: **1601–1800**, now **200/200 complete and closed**.
- Reporting section **1751–1800** is closed; checkpoints **1751–1775** and **1776–1800** are complete, committed and read-back verified.
- Active Ledger: **200 entries**, 1601–1800, each with exactly 15 fields. Index: **1,800 sequential entries and 1,800 unique paths**.
- All 25 files were read through EOF serially; files above 400 logical lines were read in consecutive chunks of at most 400. Source SHA, byte size and coverage match the pinned tree.
- Index paths and Decisions align with the new Ledger entries.
- Findings: RNG, packed storage, tick scheduler and coordinate code are important 1.20.1 research references, but this source contains Java 26.3 constants and version-specific behavior. Cosmetics and DualSense are optional Horizon subsystems; no Android HID/CPAL implementation is established.
- Embedded tests/hardware examples were inspected only, not executed. No source/application code changed. No build, tests, benchmarks, APK, device, Forge or server validation was performed.
- **STOP at entry 1800**. The next section is **1801–1850**; wait for fresh explicit authorization before inspecting it.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.



## Latest authoritative resume state — checkpoint 1801–1825 — 2026-11-20

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Completed entries: **1–1825**; all 1,825 Index paths are sequential and unique.
- Active 200-file stage: **1801–2000**, progress **25/200**.
- Active reporting section: **1801–1850**; checkpoint **1801–1825** complete; **1826–1850** remains covered by the same explicit user authorization. Do not ask for mid-section permission.
- Active Ledger: `AUDIT_LEDGER_1801_2000.md`, **25 entries** (1801–1825), each with exactly 15 fields. Progress register: `AUDIT_200_FILE_BOUNDARY.md`.
- All 25 files were read through EOF serially; files above 400 lines were read in contiguous chunks no larger than 400. SHAs and byte sizes match the pinned tree; Index paths and Decisions align with Ledger.
- Key findings: DualSense output/HDR are optional platform-specific branches; friends is peer-hosting, not standard outbound Minecraft protocol; router mapping needs explicit consent/security hardening; STUN discloses public IP; block facts/decoder offer useful architecture but need target 1.20.1 exports and strict shape validation.
- Tests/examples were inspected only, not executed. No source/application code changed. No build, tests, benchmark, APK, Android device, Forge launch or server join validation was performed.
- After the atomic checkpoint commit, read back the Ledger, Index, QA Log, Continuity, Protocol and Boundary register individually and verify consistency.
- **Next: inspect 1826–1850** under existing authorization. Finish the remaining 25, then **STOP at 1850**. Do not inspect 1851+ without fresh explicit authorization.
- main and existing backup refs remain untouched. No backup is due; next backup only after entries 1001–2000 and global verification are complete.


## Corrected checkpoint 1801–1825 registration — 2026-11-20

- Active stage 1801–2000: 25/200 formally registered in corrected canonical order.
- `crates/horizon-friends/src/lib.rs` inserted at 1812 after discovery during complete-tree reconciliation; first 25 now end at `crates/horizon-items/src/blocks/decode.rs` (1825). `blocks/decode_tests.rs` is reassigned to 1826.
- The initial 25-file candidate enumeration omitted one pinned path; final numbering is repaired and the correction retained in QA.
- The six checkpoint docs are read back after commit; the second authorized checkpoint is 1826–1850. Stop at 1850; 1851+ is unauthorized.


## Authoritative closeout — 1801–1850 complete — 2026-11-20

- Overall indexed coverage: **1–1850**, sequential and unique.
- Active 200-file stage: **1801–2000**, progress **50/200**.
- Section **1801–1850 COMPLETE/CLOSED**; both 25-file checkpoints separately committed and GitHub read-back verified.
- Correct order derives from complete pinned tree after excluding entries 1–1800. The initial candidate list omitted `crates/horizon-friends/src/lib.rs`; it was inserted at 1812 and later entries shifted. `blocks/decode_tests.rs` is 1826. `horizon-mod-registry/src/storage.rs` returned 404 and is absent from pinned tree. Final 50 paths match the tree; initial source reading crossed corrected checkpoint boundary and is disclosed in QA.
- Active Ledger `AUDIT_LEDGER_1801_2000.md`: 50 records, 15 fields each. Index: 1,850 sequential unique paths.
- All 50 files read serially through EOF; long files in continuous <=400-line chunks; source SHA/size verified. Tests/examples inspected only. No tests/build/benchmark/APK/device/Forge/server validation; no application code changes.
- **STOP at 1850.** Section 1851–1900 unauthorized without new explicit permission. main/backups untouched. Backup only after all entries 1001–2000 and global verification.

## Authoritative resume state — checkpoint 1851–1875 — 2026-11-20

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`).
- Completed Index coverage: **1–1875**, sequential and unique.
- Active 200-file stage: **1801–2000**, progress **75/200**.
- Active 50-file section: **1851–1900**, progress **25/50**. Checkpoint **1851–1875** is committed/read-back verified; **1876–1900 remains authorized** by the same explicit user grant.
- Active Ledger `AUDIT_LEDGER_1801_2000.md`: 75 entries (1801–1875), exactly 15 fields each. Index has 1,875 sequential unique paths.
- All 25 files 1851–1875 were fetched serially through EOF; large files used contiguous chunks <=400 lines; binary fixtures were fetched in full. SHA/size match pinned tree. Tests/examples inspected only, not run.
- Findings: Horizon package/index/content formats are not Forge 1.20.1 drop-ins; security review required for URL parsing, ZIP expansion, modpack extraction and registry governance. Narrator has no Android cfg/backend; speech queues are unbounded. UI narration logic is a possible accessibility reference only.
- No application source changes, tests/build/benchmark/APK/device/Forge/server validation. `main` and backup refs untouched; no backup until 1001–2000 complete and global verification passes.
- **Next: inspect 1876–1900** under existing authorization; finish and STOP at 1900. Do not inspect 1901+ without fresh explicit authorization.


## Read-back integrity correction — 2026-11-20

- Post-checkpoint read-back caught two inherited Ledger field-joining defects (entries 1823 and 1847) and three inherited Index row joins (660, 995, 1660). The missing newlines were restored without changing decisions, paths or source evidence.
- After correction: Ledger 75 entries, exactly 15 distinct fields each; Index 1,875 sequential unique rows. Six audit documents re-read after corrective commit.
- Continue 1876–1900 under existing authorization; stop at 1900.


## Authoritative resume state — section 1851–1900 closed — 2026-11-20

- Repository `imamirhosein/RustChunk`, branch `starling-1.20.1-port-audit`.
- Pinned source `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`).
- Index coverage **1–1900**, sequential and unique. Active Ledger coverage **1801–1900**, 100 entries, 15 fields each.
- Active 200-file stage **1801–2000: 100/200**. Section **1801–1850 CLOSED**, section **1851–1900 CLOSED (50/50)**. Next section **1901–1950 LOCKED**.
- Findings: OptiFine-format parser may support optional resource-pack visuals, especially CTM/CEM, but needs 1.20.1 parity and Android integration. CIT includes later item-component semantics; must use 1.20.1 NBT. CTM/file traversal need resource bounds and symlink containment. `properties.rs` has a UTF-8 boundary panic risk on escaped multibyte characters. Desktop narrator backends do not provide Android TTS.
- Tests/benchmarks/examples were inspected only, not run. No source code changes, build, APK, device, Forge, server or renderer validation.
- `main` and backup refs untouched. No backup until entries 1001–2000 complete and global 1–2000 verification passes.
- **STOP at 1900.** Do not inspect 1901+ until new explicit authorization.


## Latest verified checkpoint — 1901–1925 — 2026-11-20

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Index coverage: **1–1925**, sequential and unique. Active Ledger: **125 entries** (1801–1925), 15 fields each.
- Current 200-file stage: **1801–2000**, progress **125/200**. Section 1901–1950 is authorized; checkpoint 1901–1925 closed; 1926–1950 remains authorized.
- All 25 files were read serially through EOF; long files were read in contiguous chunks <=400 lines; source SHA/size match the pinned tree. Index paths and Decisions align with Ledger.
- Key findings: the Horizon port-pack is a separate 1.21.1→26.3 conversion tool, not a Forge 1.20.1 runtime loader. Some ideas can inform optional pack tooling/resource packs, but density/worldgen/ItemStack and folder mod outputs need 1.20.1-specific rewrites. JAR/API/archive bounds and redirects need follow-up hardening.
- Tests/examples inspected only, not run. No build/test/benchmark/APK/device/Forge/server/renderer validation. No app source changes.
- **Next: inspect 1926–1950** under the existing authorization. Then close and STOP at 1950. Do not inspect 1951+ without fresh explicit authorization.


## Authoritative closeout — section 1901–1950 — 2026-11-20

- Overall Index coverage: **1–1950**, sequential and unique.
- Active 200-file stage **1801–2000: 150/200 complete**.
- Section **1901–1950 COMPLETE/CLOSED (50/50)**; both checkpoints 1901–1925 and 1926–1950 were separately committed and read back from GitHub.
- Active Ledger `AUDIT_LEDGER_1801_2000.md`: 150 records (1801–1950), exactly 15 fields each. Index: 1,950 sequential unique paths, paths/Decisions aligned.
- Source entries 1926–1950 were read serially through EOF; files >400 lines used gap-free chunks <=400; SHA/size matched pinned tree. Tests/examples inspected only; no test/build/benchmark/APK/device/Forge/server/renderer validation; no application code changed.
- Findings: the port-pack upgrade/worldgen files target 26.2→26.3 and contain hard-coded/lossy transformations; Prepare is host-side Java/JDK exporter and asset-import infrastructure, not the Android game process; protocol probes use newer protocol and are not Forge 1.20.1 validation; Relay is optional friend-session service.
- **STOP at 1950.** Section 1951–2000 remains locked pending fresh explicit user authorization. main/backups untouched; backup only after all entries 1001–2000 and global verification.


## Latest verified checkpoint — entries 1951–1975 — 2026-11-20

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- The exact 25 paths were the next 25 blobs after excluding Index entries 1–1950; each was inspected serially through EOF, and every recorded SHA/size matches the pinned tree.
- Active stage 1801–2000 is **175/200 complete**. Index has **1,975** sequential unique paths; Ledger has **175** entries (1801–1975), each with exactly 15 fields; new paths/Decisions align.
- Section **1951–2000 remains authorized**. Checkpoint 1951–1975 is now closed; next is 1976–2000 under the same permission.
- Findings include relay limits/concurrency hardening, safe checksum path validation, dedicated-server-only file/settings modules, Java-properties/Gson parity limits, weak entropy fallback for management secret, and server-panel polling/backend-auth follow-up.
- Tests/examples inspected only; none run. No build, benchmark, APK, device, Forge, server join, Aternos or renderer validation. No application code changed.
- **STOP at 2000** and ask permission for full verification 1–2000, then final backup. No backup is due before that global verification.


## Final checkpoint — entries 1976–2000 — 2026-11-20

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Entries 1976–2000 were the next 25 blobs after excluding Index entries 1–1975. Each file was inspected serially through EOF; files over 400 lines were read in contiguous, gap-free chunks no larger than 400 lines. Recorded SHA and size match the pinned tree.
- Active stage 1801–2000 is **200/200 complete**; all four 50-file sections are closed. Ledger holds **200 entries** (1801–2000), each with exactly 15 fields; Index has **2,000** sequential unique paths and the final Ledger entries align.
- The `horizon-server` subsystem audited in this final segment is dedicated-server management for Minecraft Java 26.3, not the RustChunk Android Minecraft client. No Forge 1.20.1/Aternos/client support was demonstrated.
- Tests/examples inspected only; none executed. No build, benchmark, APK, Android, Forge, server join, Aternos or renderer validation. No application source was modified.
- **STOP at 2000.** Wait for explicit permission to perform global verification of entries 1–2000. Create the final backup only after that verification passes; no incremental backup was created.


## Global verification and final backup — 2026-11-21

- Working branch: `starling-1.20.1-port-audit`; completed audit coverage: **1–2000**.
- Reconciled all nine source Ledgers with the 2,000-row Index: sequential numbering, unique paths, path equality, exact required-field counts, Index/Ledger decision alignment, and pinned-source provenance all passed after documentation-only corrections.
- Pinned Starling snapshot remains `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`, recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`). All 2,000 source SHAs and byte sizes match that tree.
- Corrections: exact byte sizes added to 25 legacy Size fields (entries 1–20 and 96–100); 97 Ledger Decision fields across 1201–1800 were aligned to the existing Index classifications while retaining their explanatory text. Protocol checkpoint transaction wording and the current state were synchronized.
- Stage 1801–2000 is complete and all four 50-file sections are closed. Global verification passed. Final backup branch: `backup/audit-2000-final-2026-11-21`.
- `main`, `backup/audit-1000-unique-2026-11-18` and `backup/audit-1000-final-2026-11-19` remain unchanged. No application source was modified.
- Scope limit: structural/provenance/metadata/cross-document QA only; not a fresh semantic re-review, and no tests/build/benchmark/APK/device/Forge/server/Aternos/renderer validation was performed.
- **STOP at 2000.** No entry 2001+ is authorized; obtain a new explicit user instruction before starting another source section.


## Verified checkpoint — entries 2026–2050 — 2026-11-21

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Entries **2026–2050 inclusive (25/25)** were inspected serially through EOF in exact source-tree order. Eight long files were read in contiguous chunks of at most 400 lines. SHA and exact byte sizes match the pinned tree.
- Active Ledger `AUDIT_LEDGER_2001_2200.md` now contains **50 entries (2001–2050)**, each with exactly 15 fields. Index contains **2,050 sequential unique paths**; checkpoint paths and Decisions match the Ledger.
- Section **2001–2050 is closed**. The user's one-time authorization already covers **2051–2100**; continue with **2051–2075** and **2076–2100** without an intermediate permission request. Then **STOP at 2100** and ask before 2101.
- Main findings: shaderpack compatibility is a separate optional subsystem; several correctness/resource issues remain in conditional evaluation, configuration parsing, custom expression evaluation, macro expansion, options discovery and ZIP/folder resource loading. No evidence of 1.20.1/Forge compatibility was found in this segment.
- Tests/examples were inspected only, not run. No app source changed and no build/test/benchmark/APK/device/Forge/server/Aternos/render validation was performed.
- No backup is due at this 25/50/100-file checkpoint. Next backup only after 1,000 additional entries (entry 3000) and global verification. `main` and prior backup refs remain untouched.
- Next source path, verified from the pinned tree after excluding entries 1–2050: `crates/horizon-shaders/src/post.rs`.


## Verified checkpoint — entries 2051–2075 — 2026-11-21

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; full recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Entries **2051–2075 inclusive (25/25)** were reviewed serially, one source file at a time, through EOF. Long files were read in contiguous chunks of at most 400 lines. Paths, SHA and exact byte sizes match the pinned tree.
- Active Ledger holds **75 entries (2001–2075)**, each with exactly 15 required fields. Index holds **2,075 sequential unique paths**; new path/Decision rows match.
- Checkpoint **2051–2075 is closed**. The user's one-time authorization already covers **2076–2100**; continue without another permission request. At 2100 close section 2051–2100 and **STOP** before 2101.
- Key findings: optional Horizon shaderpack pipeline lacks several aggregate resource bounds and fail-closed conditions; texture decoding needs strict memory caps; GLSL stage emulation and translation are token-heuristic and Horizon-ABI-specific; device workgroup limits and option text validation need hardening; test pack is Horizon-only.
- Tests/examples were inspected only. No tests/builds/benchmarks/APK/device/Minecraft/Forge/server/Aternos/renderer validation was performed, and no application code changed.
- No backup at this checkpoint. Next backup only after 1,000 additional entries (entry 3000) and global verification. `main` and existing backup refs remain untouched.


## Verified checkpoint — entries 2076–2100 — 2026-11-21

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Entries **2076–2100 inclusive (25/25)** were reviewed one file at a time through EOF in exact tree order. Long files were read in contiguous chunks of at most 400 lines. Paths, SHAs and byte sizes match the pinned tree.
- Active Ledger: **100 entries (2001–2100)**, all with exactly 15 required fields. Index: **2,100 sequential unique paths**; new path/Decision rows align.
- Section **2051–2100 is CLOSED (50/50)**. The one-time authorization for 2001–2100 is fully consumed. **STOP at 2100**; do not inspect 2101 until new permission.
- Main findings: Horizon shader fixture packs do not prove client compatibility; telemetry is a separate opt-in host service. Notable static risks: proxy trust, container privilege, blocking SQLite in async handlers, unbounded export/history work and report idempotency races.
- Tests/examples were inspected only. No tests/builds/benchmarks/APK/device/Minecraft/Forge/server/Aternos/renderer validation was performed. No application source changed.
- No backup at 25/50/100/200-file checkpoints. Next backup only after 1,000 additional completed entries (entry 3000) and global verification. Existing `main` and backups remain untouched.


## Verified checkpoint — entries 2101–2125 — 2026-11-21

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; complete recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`).
- Entries **2101–2125 (25/25)** were inspected serially, one source file at a time, through EOF. Large files were read in contiguous, gap-free chunks no larger than 400 lines. Exact paths, SHAs and byte sizes match the pinned tree.
- Active Ledger contains **125 entries (2001–2125)**, each with the exact 15 fields. Index contains **2,125 sequential unique paths**; the new path/Decision entries align.
- The user explicitly authorized a second exceptional window **2101–2200**. Complete the next three 25-file checkpoints 2126–2150, 2151–2175 and 2176–2200 without pausing at 2150. **STOP only at 2200** and ask before 2201.
- Key findings: telemetry code is host-service infrastructure, not Minecraft gameplay; the live ping bypasses consent; fallback random IDs are predictable; outbox/consent/upload operations have race windows; scrub is best-effort; detailed hardware probes are not Android/Mali-ready. No evidence of 1.20.1 Forge client compatibility was established.
- Tests/examples were inspected only; none executed. No build/test/benchmark/APK/device/Minecraft/Forge/server/Aternos/renderer validation was performed; no application code changed.
- No backup at this checkpoint. Next backup only after 1,000 additional completed entries (entry 3000) and global verification. `main` and existing backups remain untouched.


## Verified checkpoint — entries 2126–2150 — 2026-11-21

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; complete recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`).
- Entries **2126–2150 (25/25)** are complete in exact tree order. All 11 TTF binaries were fully Base64-decoded to exact byte lengths and parsed; SFNT table ranges/checksums, whole-font checksums, naming/metrics, cmap/GSUB/GPOS and glyph outline records were reviewed. This is complete binary reading, not metadata-only.
- Three OFL font notices, one Hugeicons MIT notice and seven SVGs were fully read as UTF-8 through EOF. All 25 SHAs/byte sizes match the source tree.
- No reviewed font supports Arabic/Persian. RustChunk needs a Persian-capable face, shaping/bidi and fallback. Geist Mono is a fixed-pitch Latin code/log candidate; Pixelify is decorative only.
- Fonts are SIL OFL 1.1; keep license/copyright, do not sell font files alone, and keep derivatives under OFL. Bricolage contains Cyrillic merged from Geist, so retain both notices. Hugeicons SVG subset is MIT.
- Seven SVGs are compact 24×24 currentColor assets with no active/external resources; Android rendering remains unvalidated.
- Active Ledger 150 records (2001–2150); Index 2,150 sequential unique paths. No app source changed and no tests/runtime validation performed.
- Section **2101–2150 closed**. Continue 2151–2175 and 2176–2200 without pausing at 2150; **STOP at 2200** and ask before 2201.


## Verified checkpoint — entries 2151–2175 — 2026-11-21

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Entries **2151–2175 (25/25)** were read serially and completely through EOF. All are one-line SVGs; exact path, SHA and byte length match the pinned tree.
- The files are self-contained vector glyphs using a 24×24 viewBox. No active script, embedded images/use, external links or CSS URL references were found. Discord uses filled currentColor without explicit width/height; dashboard paths use square line caps. Other symbols are conventional stroked currentColor paths/circles.
- The set covers common launcher actions/statuses and external-brand mark (Discord). The adjacent Hugeicons MIT notice must accompany redistributed icon files. SVG-to-Android rendering remains unvalidated.
- Active Ledger has **175 records (2001–2175)**, each with 15 fields; Index has **2,175 sequential unique paths**. No app source changed; no tests/runtime validation performed.
- Continue at **2176–2200** under the same user authorization. **Do not stop/request permission at 2175; STOP only at 2200** and ask before 2201.


## Historical final checkpoint — entries 2176–2200 — 2026-11-21 (superseded by current state above)

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; complete recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23`.
- Entries **2176–2200 (25/25)** read fully in raw Git tree order; exact paths, blob SHAs and sizes match.
- Seven Glass SVGs, Horizon PNG, Pixelarticons MIT notice and sixteen Pixel SVGs were completely inspected. PNG CRCs are valid and preview was reviewed. Pixelarticons and Hugeicons MIT notices are distinct.
- SVGs contain no active/external content. Android renderer/conversion untested. Horizon PNG ownership/license unresolved.
- Historical status at this checkpoint: seven later assets (2201–2207, `home.svg` through `package.svg`) had been fetched but were not yet authorized/audited, so they were excluded from counts at that time. After the user authorized 2201–2250, those files were re-fetched and completed as entries 2201–2207; they are now present in Ledger and Index.
- Ledger 200 records with 15 fields each; Index 2,200 sequential unique paths. No app source change or runtime testing.
- Authorization 2101–2200 exhausted. Stop before entry 2201 and ask for fresh explicit permission.

## Historical checkpoint snapshot — 2201–2225 — 2026-11-21 (superseded by 2226–2250)

- Entries 2201–2225 (25/25) inspected individually through EOF in deterministic lexical path order after excluding the full Index skip set. Exact paths, SHAs and sizes match the pinned tree.
- Ledger has 225 records (2001–2225) with 15 fields per record; Index has 2,225 sequential unique paths.
- Checkpoint 2226–2250 remains within the same one-time authorization. Continue without a pause at 2225; STOP at 2250.
- No app source change or runtime validation. No backup at this checkpoint.

## Closed section snapshot — 2226–2250 — 2026-11-21 (prior to the next authorization)

- Entries 2226–2250 (25/25) inspected serially through EOF; SHA/size align with the pinned tree. Long files are covered contiguously in chunks <=400 lines.
- Ledger contains 250 records (2001–2250), 15 fields each; Index has 2,250 sequential unique paths with Decisions aligned.
- 50-file section 2201–2250 is closed. Authorization exhausted. Stop before 2251 and request fresh permission.
- No application source change, tests or runtime validation. No backup at this boundary.

## Current authorization update — 2251–2300 — 2026-11-21

- The user has explicitly authorized the next standard 50-file section **2251–2300**.
- Checkpoints **2251–2275** and **2276–2300** are both authorized; complete and verify the first 25-file checkpoint, then continue without asking at 2275.
- **STOP at 2300**. Do not inspect 2301+ without fresh explicit permission.


- Entries 2251–2275 were inspected serially through EOF in deterministic lexical order after the Index boundary; SHA and exact sizes match. Three files over 400 lines were read in contiguous chunks of at most 400 lines.
- Ledger 275 entries (2001–2275), exactly 15 fields; Index 2,275 sequential unique paths with Decisions aligned.
- Next checkpoint 2276–2300 is already authorized; proceed without pausing at 2275, then STOP at 2300.

## Closed section — 2251–2300 — 2026-11-21

- Entries 2251–2300 are complete: both 25-file checkpoints are committed and read back. Ledger has 300 records (2001–2300), exactly 15 fields each; Index has 2,300 sequential unique paths.
- Section 2251–2300 is closed (50/50); authorization exhausted. Stop before 2301 and request fresh permission.
- No app source changed; no tests run or runtime/build/device/game validation.


## Ledger partition correction — 2026-11-21

- The physical Ledger boundary was corrected: entries 2201–2300 were previously appended to `AUDIT_LEDGER_2001_2200.md` despite belonging to the separate 200-file stage 2201–2400.
- The closed Ledger now holds only entries 2001–2200 (200 records). New `AUDIT_LEDGER_2201_2400.md` holds entries 2201–2300 (100 records); it is the active Ledger for stage 2201–2400.
- Entry content, numbering, paths, Decisions and source provenance were preserved. Index numbering remains 1–2300. Historical notes describing the pre-correction combined Ledger remain historical snapshots.
- Current resume state remains 1–2300 complete; 2301–2350 is authorized, and the stop boundary is 2350.

## Latest verified checkpoint — 2301–2325 — 2026-11-21

- Entries 2301–2325 (25/25) inspected individually through EOF. Files 2313 and 2316 were read in contiguous chunks <=400 lines. All source SHAs and sizes match the pinned recursive tree.
- Active Ledger: 125 records (2201–2325), exactly 15 fields; Index: 2,325 sequential unique paths with matching Decisions.
- The one-time authorization through 2400 remains active. Continue 2326–2350 without pausing; **STOP only at 2400**.


## Latest verified checkpoint — 2326–2350 — 2026-11-21

- Entries 2326–2350 (25/25) inspected individually through EOF. Files 2332 (598), 2342 (735) and 2344 (828) were read in contiguous ranges <=400 lines. Source SHAs/sizes match the pinned tree.
- Active Ledger: 150 records (2201–2350), exactly 15 fields; Index: 2,350 sequential unique paths with matching Decisions.
- Section 2301–2350 is closed. One-time authorization through 2400 remains active for 2351–2375 and 2376–2400; STOP at 2400.


## Latest verified checkpoint — 2351–2375 — 2026-11-21

- Entries 2351–2375 (25/25) inspected through EOF. Four long files (2362, 2363, 2371 and 2373) were read in contiguous chunks <=400 lines. SHA/size match pinned tree.
- Active Ledger: 175 records (2201–2375), exactly 15 fields; Index: 2,375 sequential unique paths with Decisions aligned.
- 2376–2400 remains covered by the one-time authorization. It was re-derived with the in-flight 2351–2375 paths excluded; no overlap. STOP at 2400.


## Latest verified checkpoint — 2376–2400 — 2026-11-21

- Entries 2376–2400 (25/25) were inspected individually through EOF; 2376 (530), 2380 (407), 2381 (483) and 2397 (672) were read in contiguous chunks no larger than 400 lines. Source SHAs/sizes match the pinned tree.
- Ledger contains 200 records (2201–2400), exactly 15 fields each; Index has 2,400 sequential unique paths with path/Decision alignment.
- `use_flag_fixture.rs` was absent and excluded; actual 2398 path `use_flag_reaction_fixture.rs` was verified and recorded.
- Both exceptional authorizations are exhausted. **STOP at 2400**; obtain fresh permission before 2401. No new backup at this boundary.


## Latest verified checkpoint — 2401–2425 — 2026-11-21

- All 25 files read through EOF; long-file ranges: attachments.rs 1–400/401–695; live.rs 1–400/401–416; worldgen_parity.rs 1–200/201–400/401–600/601–800/801–915; block_landing.rs 1–400/401–464.
- Ledger 25 records with 15 fields; Index 2,425 unique sequential paths.
- Key flags: swallowed piston binding errors, empty modifier path, unchecked CLI selectors, callback partial mutation and coordinate defaults.
- Section 2401–2450 is closed. Next checkpoint 2451–2475 is authorized under the same one-time 2401–2500 grant; report at 2500 and stop there.


## Latest verified checkpoint — 2426–2450 — 2026-11-21

- All 25 files read through EOF; long files chunked <=400 lines: block_state_properties.rs, block_survival.rs, block_state_data.txt, chunk_leaves.rs, files.rs and fixes_a.rs.
- Ledger 50 records; Index 2,450 unique sequential paths; 15 fields per entry.
- Key flags: duplicate survival queries; probable consumable RNG overdraw; NaN consume-effect predicate; malformed condition vacuous truth; suppressed filesystem enumeration errors; legacy coordinate overflow.
- Next 2451–2475 under existing exception; report at 2500 and stop.


## Latest verified checkpoint — 2451–2475 — 2026-11-21

- All 25 files read through EOF; large files chunked <=400 lines. Index and pinned-tree SHAs/sizes matched.
- Ledger 75 records; Index 2,475 unique sequential paths; 15 fields per entry.
- Key flags: UUID half-zero handling, origin tick zero, legacy mapping typo, JSON parser edge cases, panic-unsafe thread-local mark, region-upgrader short-circuit.
- Next 2476–2500 under current grant; report and stop at 2500.


## Latest verified checkpoint — 2476–2500 — 2026-11-21

- All 25 files read through EOF; long files entity_fluid.rs 1–400/401–454 and inventory_use.rs 1–400/401–485.
- Ledger 100 records; Index 2,500 unique sequential paths; exactly 15 fields per entry.
- Key flags: unchecked FluidState amount, partial inventory pickup false result, fractional stack-size truncation, durability Damage default assumption.
- **STOP at 2500**; exceptional authorization is exhausted. Request fresh permission before 2501.


## Latest verified checkpoint — 2501–2525 — 2026-11-21

- All 25 files inspected individually through EOF, in lexical order after excluding the 2,500 existing Index paths. SHA and byte sizes match the complete pinned source tree.
- Long-file ranges: java_hash_map.rs 1–400/401–791; bonemeal.rs 1–400/401–800/801–1200/1201–1256; data.rs 1–400/401–800/801–1200/1201–1222; data/cache.rs 1–400/401–800/801–1166; fluids.rs 1–400/401–800/801–833; growth.rs 1–400/401–462; growth_tests.rs 1–400/401–792; hooks.rs 1–400/401–749; interact.rs 1–400/401–800/801–1037; mod.rs 1–400/401–800/801–1200/1201–1266.
- Active Ledger 125 entries with exactly 15 fields; Index 2,525 sequential unique paths and matching Decisions.
- User grant covers 2501–2600. Next checkpoint 2526–2550 is authorized; report at 2550, then continue to 2600 and STOP.
- Independent structural/provenance QA of entries 2401–2500 passed; full-stage 2401–2600 QA is due only after entry 2600.
- Tests/examples inspected only, none run. No source changes/build/APK/device/runtime validation or backup at 2525.


## Latest verified checkpoint — 2526–2550 — 2026-11-21

- Re-read all 25 sources from the beginning, individually and through EOF. Pinned recursive source tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is untruncated; exact paths, SHAs and byte sizes match. Long files were read in contiguous, gap-free 200-line chunks.
- Ledger entries 2401–2550: 150 records, 15 required fields each. Index: 2,550 sequential unique paths with matching Decisions. Six-document commit and full read-back verification are the remaining checkpoint-registration steps at the time this note was prepared.
- Important risks: paired placement partial writes, portal multi-write refusal, entity/block-entity host callbacks, ordered redstone/shape dispatch, registry-order dependence, tests that skip when resources are absent, scheduler/update budget error handling, modern-only block families and weather height/sea-level assumptions.
- `redstone.rs` corrected to 59,404 bytes / 1,507 lines; prior interim estimate was inaccurate.
- **STOP at 2550.** The latest user instruction supersedes previous historical notes that said the 2501–2600 grant covered both 50-file sections. A separate explicit authorization is required for 2551–2600. After that authorized range completes, recheck all 200 entries 2401–2600 independently.
- Tests/examples not executed. No product source, build, APK or runtime verification. No backup at 2550.


## Registration correction — 2526–2550 — 2026-11-21

- The first commit's read-back showed that 25 Index rows had been added but their Ledger entries were missing. The checkpoint was not considered complete at that point.
- A corrective six-document commit now adds the full 25 Ledger records and retains the matching 2,550 Index rows. The final checkpoint state is accepted only after all six documents are fetched from the corrective commit and their structure, path/Decision alignment and pinned source provenance are verified.
- User's active stop boundary remains 2550. No 2551+ source is authorized yet.


## Final register alignment correction — 2526–2550 — 2026-11-21

- Corrected read-back confirmed 150 Ledger records and 2,550 sequential unique Index rows with exactly 15 fields per Ledger record. One Decision wording mismatch at 2536 was found and normalized in the Index.
- Re-run full 2401–2550 path/Decision and pinned-tree SHA/size verification after the alignment correction. The stop remains 2550; wait for separate authorization before 2551.


## Communication preference — 2026-11-21

Keep progress updates extremely short. Report only the current range and the next range (for example, “Now at 2551–2575. Next: 2576–2600.”). Do not describe which source file is being reviewed or what it contains unless the user asks.


## Latest checkpoint — 2551–2575 — 2026-11-21

- Entries 2551–2575 complete through EOF; pinned paths, SHAs and byte sizes match.
- Ledger: 175 entries, 15 fields each. Index: 2,575 sequential unique paths.
- Long-file reads: `loot.rs` 1–2298 in chunks of at most 400 lines; `object_open_map.rs` 1–400/401–412; `piston_push.rs` 1–400/401–483.
- User pre-authorized 2576–2600. Continue without asking again; stop at 2600, then run independent QA across all 200 entries 2401–2600.
- Tests/examples not executed. No product source changes or runtime validation.


## Latest checkpoint — 2576–2600 — 2026-11-21

- Entries 2576–2600 complete individually through EOF; pinned path/SHA/byte-size checks pass.
- Ledger: 200 entries, 15 fields each. Index: 2,600 sequential unique paths.
- All four 50-file sections are closed. Stop source inspection at 2600.
- Perform independent second-pass QA across all 200 entries 2401–2600 before final acceptance.
- Tests/examples not executed; no application source changes or runtime validation.


## Final full-stage QA — 2401–2600 — 2026-11-21

- **200/200 complete and verified.** Ledger 200 records, each exactly 15 fields. Index 2,600 sequential unique paths.
- All 200 pinned-tree paths, SHAs and byte sizes match; all Ledger entries align with Index paths and Decisions; Coverage/Completion line counts agree.
- The active range was reordered and renumbered in strict raw lexical path order; final Ledger/Index alignment and source provenance were rechecked.
- Stop at 2600. A new explicit authorization is required for 2601+.
- Tests/examples not executed. No product source changes or runtime validation.


## Latest verified checkpoint — 2601–2625 — 2026-11-21

- All 25 files were inspected individually through EOF at the pinned Starling snapshot; paths, SHAs and byte sizes match the untruncated tree.
- Long-file reads were contiguous and gap-free: `aquifer/noise_based.rs` 1–400/401–642; `blending.rs` 1–400/401–800/801–1031; `block_feature.rs` 1–400/401–539.
- New Ledger: 25 records with exactly 15 fields each. Index: 2,625 sequential unique paths and matching Decisions.
- The one-time grant remains active for 2626–2650 and 2651–2700. Keep ordinary 25-file commits and 50-file reporting boundaries. At 2650 continue under the same grant; stop exactly at 2700.
- Tests/examples not executed. No product source change or build/runtime/device/Forge/Aternos validation.


## Latest verified checkpoint — 2626–2650 — 2026-11-21

- All 25 source files inspected individually through EOF at the pinned Starling snapshot; paths, SHAs and byte sizes checked against the untruncated tree.
- Long-file coverage was contiguous: `cave.rs` 1–400/401–700; `density.rs` 1–400/401–578; `decoration_order.rs` 1–400/401–420; `dripstone.rs` 1–400/401–800; `feature_graph.rs` 1–400/401–800/801–1106; `feature_region.rs` 1–400/401–683.
- Ledger and Index now cover 1–2650; active Ledger has 50 records, each with exactly 15 fields.
- Exceptional grant remains active for 2651–2700. Stop exactly at 2700; 2701+ requires a fresh explicit grant.
- Tests/examples not executed; no source changes or build/runtime/device/Forge/Aternos validation.


## Latest verified checkpoint — 2651–2675 — 2026-11-21

- All 25 source files were inspected individually through EOF at the pinned Starling snapshot; paths, SHAs and byte sizes match the untruncated tree.
- Long-file ranges were contiguous and gap-free: `generator.rs` through line 4575; `generator_decoration.rs` through 1481; `generator_structures.rs` through 841; `material_condition.rs` through 748; `material_condition/codec.rs` through 468; `material_stage.rs` through 426; `overrides.rs` through 478; `placement.rs` through 505.
- Ledger and Index now cover 1–2675. The active Ledger contains 75 entries with exactly 15 fields each.
- The remaining exceptional authorization is exactly 2676–2700. Stop at 2700; do not inspect 2701+.
- Tests/examples not executed; no product source changes or build/runtime/device/Forge/Aternos validation.


## Latest verified checkpoint — 2676–2700 — 2026-11-22

- Entries 2676–2700 were inspected individually through EOF at pinned Starling commit `5432370d573642041585c69b40fb045b27133488`; all 25 paths, SHAs and byte sizes match the complete untruncated tree.
- Long-file coverage was contiguous and gap-free: `world_io.rs` 1–400/401–413; `files.rs` 1–400/401–436; account `lib.rs` 1–400/401–800/801–875; account `tests.rs` 1–400/401–473; catalog `lib.rs` 1–400/401–432.
- Ledger: 100 records (2601–2700), exactly 15 fields each. Index: 2,700 sequential unique paths; all 25 new paths and Decisions align.
- Both 50-file sections 2601–2650 and 2651–2700 are closed. The exceptional 100-file grant is exhausted. **STOP at 2700; no inspection or preparation of 2701+ without a new explicit grant.**
- Tests/examples were inspected only; none executed. No RustChunk application source changed; no build, APK, device, renderer, Minecraft, Forge, multiplayer or Aternos validation. No backup is due before 3000/global verification.


- Final read-back QA caught and corrected one SHA transcription in entry 2682; the corrected SHA now matches the pinned tree. All six audit documents are re-committed together and re-verified after this correction.

## Latest verified checkpoint — 2701–2725 — 2026-11-22

- Entries **2701–2725 inclusive (25/25)** were read through EOF from pinned Starling `5432370d573642041585c69b40fb045b27133488`; source SHAs and byte sizes match the complete untruncated recursive tree.
- The full-tree candidate scan corrected the prior ordering blind spot: 75 unindexed Rust paths sorted before/equal to the previous last Index path; entries 2701–2750 now cover the first 50, with **25 remaining**. The next candidate is `crates/horizon-world/src/worldgen/tree_blocks.rs` (2751).
- Ledger: **150 entries (2601–2750)**; Index: **2,750 sequential unique paths**. All new entries have exactly 15 fields and their paths/Decisions align with the Index.
- Long files were read sequentially in contiguous chunks no larger than 400 lines; exact ranges are recorded in the Ledger and QA Log.
- User authorization covers **2701–2800**. Section **2701–2750 is closed**; continue with **2751–2775** without another permission request. Keep 25-file commits and 50-file reports; **STOP only after 2800**.
- No product source changed. Tests/examples were inspected only; no tests, build, APK, device, Forge, Minecraft, multiplayer or Aternos validation was run.

## Latest verified checkpoint — 2726–2750 — 2026-11-22

- Entries **2726–2750 inclusive (25/25)** were read through EOF from pinned Starling `5432370d573642041585c69b40fb045b27133488`; source SHAs and byte sizes match the complete untruncated recursive tree.
- The active 200-file stage is **150/200 complete**. Ledger: **150 records (2601–2750)**; Index: **2,750 sequential unique paths**. All new entries contain exactly 15 fields and align with Index paths/Decisions.
- The previous lexical-order blind spot is reduced from 75 to **25 unindexed Rust paths** before/equal to `crates/starling-catalog/src/list.rs`; the next candidate is `crates/horizon-world/src/worldgen/tree_blocks.rs` (2751).
- Sections 2601–2650, 2651–2700 and 2701–2750 are closed. The current one-time authorization still covers 2751–2800; continue at **2751–2775** and **STOP only after 2800**.
- Long files were reviewed in consecutive ranges no larger than 400 lines. Tests/examples were inspected only; no tests, build, APK, device, Forge, Minecraft, multiplayer or Aternos validation was run. No application source changed.

 
## Latest verified checkpoint — 2751–2775 — 2026-11-22

- Entries **2751–2775 inclusive (25/25)** were read through EOF from pinned Starling `5432370d573642041585c69b40fb045b27133488`; all source SHAs and byte sizes match the complete untruncated recursive tree, and the canonical EOF line counts match the recorded Coverage.
- The active 200-file stage is **175/200 complete**. Ledger: **175 records (2601–2775)**; Index: **2,775 sequential unique paths**. Each new record has exactly 15 fields and its path/Decision aligns with the Index.
- The 75-path lexical-order blind spot found at entry 2701 is now exhausted: entries 2701–2775 cover all 75 unindexed Rust paths sorting before/equal to the prior Index cutoff. Choose the next 25 from the complete pinned tree after excluding all 2,775 indexed paths.
- Sections 2601–2650, 2651–2700, 2701–2750 are closed. Section **2751–2800** is 25/50 complete; checkpoint **2776–2800** is already authorized by the same exceptional grant. **STOP only after entry 2800.**
- Entries 2726–2750 were re-fetched and independently checked for SHA, size, EOF line counts and exactly 15 Ledger fields; all checks passed. The findings in entries 2730, 2735 and 2737 were refined in the Ledger during this checkpoint transaction.
- Tests/examples were inspected only; no tests, build, APK, device, Forge, Minecraft, multiplayer or Aternos validation was run. No application source changed.


## Ledger structure correction — checkpoint 2751–2775 — 2026-11-22

- GitHub read-back of the first checkpoint commit found a field-value alignment defect in the 25 newly authored Ledger entries. The Index decisions and source provenance were correct, but the Ledger's Decision/implementation/evidence/completion values were shifted by one field in the generated draft.
- All 25 records were corrected and the test-only records 2758–2775 were given explicit RustChunk test-suite mappings. The correction preserves source SHA, byte size, canonical line count, coverage and source findings.
- The corrective six-document transaction must be committed and re-read from GitHub before treating 2751–2775 as closed or continuing to 2776.

## Closed reporting section — 2751–2800 — 2026-11-23

- Entries **2751–2800 inclusive (50/50)** are complete in two 25-file checkpoints.
- Active Ledger `AUDIT_LEDGER_2601_2800.md`: **200 entries (2601–2800)**. Index: **2,800 sequential unique paths**.
- The 2776–2800 checkpoint passed a second structural/provenance check for all source SHA/size/EOF counts, all 15 Ledger fields and Index path/Decision alignment. Entry 2764's SHA transcription was corrected.
- Stage **2601–2800 is CLOSED (200/200)**. The exceptional grant is exhausted. **STOP at 2800; 2801+ requires fresh explicit authorization.**
- No tests/builds/APK/device/Forge/Minecraft/server validation; no product source changed.


## Latest verified checkpoint — 2801–2825 — 2026-11-23

- Entries **2801–2825 inclusive (25/25)** were inspected individually through EOF from pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Exact source paths, SHAs and byte sizes match the complete untruncated recursive tree. Canonical logical line counts and EOF coverage are recorded in the Ledger.
- New Ledger `AUDIT_LEDGER_2801_3000.md`: 25 records, each with exactly 15 required fields. Index: 2,825 sequential unique paths; new paths and Decisions align.
- Section **2801–2850** is 25/50 complete. Continue **2826–2850** and report at 2850, then continue **2851–2875** and **2876–2900** under the same exceptional authorization. **STOP at 2900**.
- This was source/documentation review only. No tests/builds/benchmarks/APK/device/Forge/Minecraft/server validation; no application source changes.


## Latest verified checkpoint — 2826–2850 — 2026-11-23

- Entries **2826–2850 inclusive (25/25)** were inspected individually through EOF from pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Exact source paths, SHAs and byte sizes match the complete untruncated recursive tree. Large-file coverage is consecutive and gap-free in ranges no larger than 400 lines.
- Active Ledger `AUDIT_LEDGER_2801_3000.md`: 50 records, each with exactly 15 fields. Index: 2,850 sequential unique paths; new paths and Decisions align.
- Section **2801–2850 is closed**. Continue **2851–2875** and **2876–2900** under the same explicit exceptional authorization; **STOP at 2900**.
- Documentation/source review only. No tests/builds/benchmarks/APK/device/Forge/Minecraft/server validation; no application source changes.


## Latest verified checkpoint — 2851–2875 — 2026-11-23

- Entries **2851–2875 inclusive (25/25)** were inspected individually through EOF from pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Exact source paths, SHAs and byte sizes match the complete untruncated recursive tree. Large-file coverage is consecutive and gap-free in ranges no larger than 400 lines.
- Active Ledger `AUDIT_LEDGER_2801_3000.md`: 75 records, each with exactly 15 fields. Index: 2,875 sequential unique paths; new paths and Decisions align.
- Continue **2876–2900** under the same explicit exceptional authorization; **STOP at 2900**.
- Documentation/source review only. No tests/builds/benchmarks/APK/device/Forge/Minecraft/server validation; no application source changes.


## Final checkpoint under exceptional authorization — 2876–2900 — 2026-11-23

- Entries **2876–2900 inclusive (25/25)** were inspected individually through EOF from pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Exact source paths, SHAs and byte sizes match the complete untruncated recursive tree. Large-file coverage is consecutive and gap-free in ranges no larger than 400 lines.
- Active Ledger `AUDIT_LEDGER_2801_3000.md`: 100 records, each with exactly 15 fields. Index: 2,900 sequential unique paths; new paths and Decisions align.
- The one-time exceptional authorization **2801–2900 is exhausted**. Stage 2801–3000 is 100/200 complete; both authorized 50-file sections are closed. **STOP at 2900**; 2901+ needs fresh explicit permission.
- Documentation/Cargo-manifest source review only. No tests/builds/benchmarks/APK/device/Forge/Minecraft/server validation; no application source changes.


## Checkpoint 2901–2925 — COMPLETE — 2026-11-23

- 25 files 2901–2925 inspected through EOF from pinned Starling `5432370d573642041585c69b40fb045b27133488`; paths selected from full untruncated tree after excluding 2,900 prior paths.
- Exact source SHA and byte size verified against pinned tree. Canonical logical lines and gap-free EOF coverage recorded in Ledger.
- Active stage Ledger now contains 125 records (2801–2925), exactly 15 fields each; Index contains 2,925 sequential unique paths.
- Locale JSON consistency check: 10 locales, 914 shared keys each, valid JSON, placeholders aligned; English-identical entries noted per locale in findings.
- The user's exceptional authorization 2901–3000 remains active; next checkpoint is 2926–2950. Continue without asking for another grant.
- No app source modifications, tests/builds/benchmarks/APK/device/Forge/Minecraft/server/Aternos validation.


## Checkpoint 2926–2950 — COMPLETE — 2026-11-23

- 25 files 2926–2950 inspected through EOF from pinned Starling `5432370d573642041585c69b40fb045b27133488`; selected from full untruncated tree after excluding 2,925 prior Index paths.
- Exact SHA/size provenance checked against the pinned tree; canonical logical line counts and complete contiguous EOF coverage recorded. Long files used consecutive <=400-line chunks.
- Active stage Ledger now contains 150 records (2801–2950), exactly 15 fields each; Index contains 2,950 sequential unique paths. Section 2901–2950 is closed (50/50).
- The exceptional authorization 2901–3000 remains active. Continue 2951–2975 then 2976–3000 without asking; stop at 3000.
- No application source changes, tests/builds/benchmarks/APK/device/Forge/Minecraft/server validation.


## Checkpoint 2951–2975 — COMPLETE — 2026-11-23

- 25 files 2951–2975 inspected through EOF from pinned Starling `5432370d573642041585c69b40fb045b27133488`; selected from full untruncated tree after excluding 2,950 prior Index paths.
- Exact SHA/size provenance checked against pinned tree; canonical logical line counts and complete gap-free EOF coverage recorded. Long files were read in consecutive <=400-line chunks.
- Active Ledger now contains 175 records (2801–2975), exactly 15 fields each; Index contains 2,975 sequential unique paths.
- Exceptional authorization 2901–3000 remains active; continue 2976–3000 without asking, then stop at 3000.
- No application source changes, tests/builds/benchmarks/APK/device/Forge/Minecraft/server validation.


## Final checkpoint 2976–3000 — COMPLETE — 2026-11-23

- Entries **2976–3000 (25/25)** inspected through EOF from pinned Starling `5432370d573642041585c69b40fb045b27133488`; selected from full untruncated tree after excluding all 2,975 previous Index paths.
- Exact SHA/size provenance checked against the pinned tree. Canonical logical line counts and complete gap-free EOF coverage recorded. All long files were read in consecutive <=400-line ranges.
- Active Ledger now contains **200 records (2801–3000)**, exactly 15 fields each. Index contains **3,000 sequential unique paths**, no duplicates and Ledger/Index path/Decision alignment.
- Final compatibility finding: Starling's Java loader support is Fabric/Quilt only; its native-mod compiler/injector is a separate Horizon-specific Rust extension mechanism, not Forge. RustChunk must implement actual Forge 47.x and Java 17 support independently.
- Exceptional authorization **2901–3000 is EXHAUSTED**. Stage **2801–3000 is CLOSED (200/200)**. Both sections 2901–2950 and 2951–3000 are closed. **STOP at 3000**; fresh authorization required for 3001+.
- No application source changes, tests/builds/benchmarks/APK/device/renderer/Minecraft/Forge/server/Aternos validation.
