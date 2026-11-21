# RustChunk Audit Continuity

Last reconciled: 2026-11-20
Repository: imamirhosein/RustChunk
Working branch: starling-1.20.1-port-audit

## Authoritative resume point

- Last reconciled: **2026-11-21**.
- Completed source entries: **1–2250**.
- Active 200-file stage: **2201–2400**, progress **50/200**.
- Closed reporting section: **2201–2250 (50/50)**, checkpoints **2201–2225** and **2226–2250** both complete and read-back verified.
- The one-time user authorization for **2201–2250** is exhausted. **STOP before entry 2251**; obtain fresh explicit authorization.
- Active Ledger `AUDIT_LEDGER_2001_2200.md` contains **250 records (2001–2250)** with exactly 15 fields; Index contains **2,250 sequential unique paths**.
- Pinned upstream: `starling-client/starling` @ `${pinned}`; complete recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`).
- Seven previously fetched assets 2201–2207 were re-fetched and included under the new 2201–2250 authorization; all are now legitimate completed entries.
- Entry 2214 remains a custom Horizon star with unresolved separate asset-license provenance. The jobs/threads/memory issues and chunkgen benchmark verifier defect from 2226–2250 are documented.
- Existing final backup still covers entries **1–2000** only. No backup at 2250; next backup only after entry 3000 and global verification.
- No application source changed. Tests/examples were inspected only, not run; no build/benchmark/APK/device/renderer/Minecraft/Forge/server/Aternos validation.

## Source pin and selection

- Upstream: starling-client/starling, ref `starling`.
- Pinned commit: `5432370d573642041585c69b40fb045b27133488`.
- Inspect only this exact snapshot.
- Build the recursive source tree and verify it is complete/not truncated.
- Select paths in deterministic lexical order, excluding every path already in the full Index. Never repeat or renumber a source path.
- Verify every source SHA and byte size against the pinned tree.

## Per-file inspection requirements

- Read every file fully through EOF.
- For files over 400 logical lines, use consecutive, gap-free ranges of no more than 400 lines each.
- Record canonical logical line count, byte size, SHA and exact coverage.
- Canonical line count: LF-terminated line count, plus one only when the blob does not end in LF; do not count a synthetic empty split item after a final LF.
- Each Ledger entry must contain exactly these 15 fields, each once: Source repository/ref; Source SHA; Size; Coverage; Purpose; Dependencies / cross-references; Implementation / logic; Findings; RustChunk relevance; Decision; 1.20.1 implications; Follow-up checks; RustChunk implementation mapping; Evidence required before adoption; Completion state.
- Decision primary labels: essential, adaptable, rewrite required, integrate, unnecessary, undetermined. Qualifiers: reference, version-specific, optional/future.
- Separate observed facts, interpretation, suspected defects and proposed target-version work. Do not claim a suspected defect is reproduced without evidence.

## Five-document checkpoint persistence

At every 25-file checkpoint, update together:
1. The applicable Ledger.
2. AUDIT_INDEX.md.
3. AUDIT_QA_LOG.md.
4. AUDIT_CONTINUITY.md.
5. AUDIT_PROTOCOL.md when state/rules need synchronization; keep it consistent with Continuity.

Validate sequential numbering, global path uniqueness, all 15 fields, pinned SHA/byte size, complete EOF coverage, Index/Ledger alignment and correct resume state. Commit the five documents together in one commit. Re-fetch all five from GitHub and verify saved contents/blob SHAs before calling the checkpoint complete.

Before each new chat continuation, re-fetch the latest branch head and read this file plus the latest audit documents. If context is uncertain, stop and reconcile rather than guessing.

## Main project boundary

- Main RustChunk development remains on `main`; Starling audit artifacts remain on the audit branch.
- The audit is research and compatibility assessment, not proof that features have been implemented.
- Reading embedded tests is not running them.
- Never claim build, unit-test, benchmark, APK build, Forge launch, Android-device validation, multiplayer join, LAN, UPnP or relay tests unless actually executed.
- Starling/Horizon is based on later Minecraft 26.3 behavior. Do not treat its registries, packets, IDs, assets or native mod API as Forge 1.20.1-compatible without separate target-era evidence.
- The Horizon native-mod build pipeline and app-library replacement are high-privilege mechanisms; do not treat them as normal Forge mod loading.

## Backup state

- `backup/audit-1000-unique-2026-11-18` -> `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`.
- `backup/audit-1000-final-2026-11-19` -> `4ba78eba6ebc2c89bb455a68a3cb9ddfe5f30e23`.
- Both backup refs must remain untouched. They are procedurally read-only; GitHub currently reports branch protection disabled.
- Next backup only after entries 1001–2000 are fully committed and globally verified. No backup after 25, 50, 100 or 200 entries.
- Do not recreate or alter the deleted archive branch. Do not place audit files on main.

## Existing completed work

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


## Final authorized checkpoint — entries 2176–2200 — 2026-11-21

- Repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; complete recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23`.
- Entries **2176–2200 (25/25)** read fully in raw Git tree order; exact paths, blob SHAs and sizes match.
- Seven Glass SVGs, Horizon PNG, Pixelarticons MIT notice and sixteen Pixel SVGs were completely inspected. PNG CRCs are valid and preview was reviewed. Pixelarticons and Hugeicons MIT notices are distinct.
- SVGs contain no active/external content. Android renderer/conversion untested. Horizon PNG ownership/license unresolved.
- Seven later assets (2201–2207, `home.svg` through `package.svg`) were inadvertently fetched in an earlier ordering mistake; they are excluded from Ledger/Index and completion counts.
- Ledger 200 records with 15 fields each; Index 2,200 sequential unique paths. No app source change or runtime testing.
- Authorization 2101–2200 exhausted. Stop before entry 2201 and ask for fresh explicit permission.

## Latest verified checkpoint — 2201–2225 — 2026-11-21

- Entries 2201–2225 (25/25) inspected individually through EOF in deterministic lexical path order after excluding the full Index skip set. Exact paths, SHAs and sizes match the pinned tree.
- Ledger has 225 records (2001–2225) with 15 fields per record; Index has 2,225 sequential unique paths.
- Checkpoint 2226–2250 remains within the same one-time authorization. Continue without a pause at 2225; STOP at 2250.
- No app source change or runtime validation. No backup at this checkpoint.

## Latest verified checkpoint — 2226–2250 — 2026-11-21

- Entries 2226–2250 (25/25) inspected serially through EOF; SHA/size align with the pinned tree. Long files are covered contiguously in chunks <=400 lines.
- Ledger contains 250 records (2001–2250), 15 fields each; Index has 2,250 sequential unique paths with Decisions aligned.
- 50-file section 2201–2250 is closed. Authorization exhausted. Stop before 2251 and request fresh permission.
- No application source change, tests or runtime validation. No backup at this boundary.
