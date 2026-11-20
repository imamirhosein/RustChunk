# RustChunk Audit Continuity

Last reconciled: 2026-11-20
Repository: imamirhosein/RustChunk
Working branch: starling-1.20.1-port-audit

## Authoritative resume point

- Last reconciled: **2026-11-20**.
- Completed and committed entries: **1–1800**.
- Closed prior stages: **1–1000**, **1001–1200**, **1201–1400**, and **1401–1600**.
- Active 200-file stage: **1601–1800**, **200/200 complete**; this stage is closed.
- Closed reporting sections: **1601–1650**, **1651–1700**, **1701–1750** and **1751–1800**.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **200 entries** (1601–1800), each with exactly 15 fields.
- Index: **1,800 sequential entries and 1,800 unique paths**.
- **STOP at entry 1800.** The next section, **1801–1850**, is not authorized; wait for explicit user authorization before inspecting entry 1801.
- Preserve the 200 / 50 / 25 cadence: commit and read-back verify each 25-file checkpoint; report and stop at each 50-file boundary.
- Next backup remains due only after entries 1001–2000 are complete and globally verified. Existing backup refs remain untouched.

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
