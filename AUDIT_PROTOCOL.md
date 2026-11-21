# RustChunk Starling Audit Protocol

This is the single authoritative protocol for the Starling source audit on `starling-1.20.1-port-audit`. If an older note, heading, QA entry or Ledger preamble conflicts with this file, this protocol and the matching `AUDIT_CONTINUITY.md` govern future work. Historical QA records remain historical evidence and must not be silently rewritten.

## 1. Scope and pinned source

- Project: `imamirhosein/RustChunk`; audit branch: `starling-1.20.1-port-audit`. Do not put audit ledgers or audit-only changes on `main`.
- Upstream: `starling-client/starling`, ref `starling`; pinned commit: `5432370d573642041585c69b40fb045b27133488`.
- Inspect only this exact pinned snapshot, never the moving upstream branch.
- The audit assesses which Starling/Horizon architecture is reusable, adaptable, version-specific, unnecessary or requires rewriting for RustChunk's Minecraft Java 1.20.1 / Forge 47.x target. Starling 26.3 behavior, registries, IDs, packets and assets are not proof of 1.20.1 compatibility.

## 2. Authoritative 200 / 50 / 25 cadence

- The accounting and closure unit is **200 new, unique source files**.
- Every 200-file stage has **four consecutive 50-file reporting sections**.
- Every 50-file section contains **exactly two 25-file checkpoints**.
- One complete 200-file stage therefore contains four 50-file sections and eight 25-file checkpoints.
- The **25-file checkpoint** is the durable Git commit and verification unit. Complete exactly 25 new files, update all six audit documents together, commit them together, re-fetch them from GitHub and verify the committed contents before inspecting the next 25 files.
- The **50-file boundary** is the user-facing report and authorization boundary. After the two 25-file checkpoints are committed and verified, report the completed section, identify the exact next 50-file range, and **STOP**. Do not inspect the next section until the user explicitly authorizes it.
- Do not ask for permission between the two 25-file checkpoints inside an already authorized 50-file section. Do not cross a 50-file boundary on assumed or implied permission.
- Do not change the 200 / 50 / 25 cadence unless the user explicitly changes it.

## 3. Historical stage state after checkpoint 1451–1475 (superseded by current state in Section 12)

- Entries 1–1475 are complete. The 200-file stage 1401–1600 is 75/200 complete.
- Reporting section **1401–1450** is closed.
- The user explicitly authorized section **1451–1500**. Checkpoint **1451–1475** is complete and checkpoint **1476–1500** remains authorized.
- Continue through entry 1500, then STOP and wait for explicit authorization before section 1501–1550.
- The following section 1551–1600 also requires fresh authorization at the preceding 50-file boundary.
- Preserve historical records; do not re-audit or renumber completed paths.

## 4. Deterministic source selection and uniqueness

- Fetch the recursive tree for the pinned upstream commit and confirm it is complete and not truncated.
- Select source files in deterministic lexical path order from that pinned tree, excluding every path already present in the complete `AUDIT_INDEX.md`.
- A source path may appear exactly once in the full audit. Never assign a second entry number to a previously audited path. Never renumber an earlier entry to conceal duplication.
- Verify each selected path exists in the pinned tree and obtain its exact blob SHA and byte size there.
- Keep the full Index as the durable skip set, including all historical duplicate corrections recorded in the ledgers and QA log.

## 5. Complete file inspection

- Read each selected source file completely, from the first logical line through EOF, before marking it complete or moving on.
- Files longer than 400 logical lines must be read in consecutive, gap-free ranges of at most 400 lines (for example 1–400, 401–800, then onward through EOF).
- Record exact read ranges in Coverage. No omitted lines, overlaps, skipped tails or inferred completion.
- If a file cannot be fetched or read completely, record the exact limitation. Do not mark it COMPLETE or count it among the 25 files.
- Inspect embedded tests and examples as source evidence, but reading a test is not running it.

## 6. Evidence and the exact 15 Ledger fields

Every entry must contain each of these 15 fields exactly once:
1. Source repository/ref
2. Source SHA
3. Size
4. Coverage
5. Purpose
6. Dependencies / cross-references
7. Implementation / logic
8. Findings
9. RustChunk relevance
10. Decision
11. 1.20.1 implications
12. Follow-up checks
13. RustChunk implementation mapping
14. Evidence required before adoption
15. Completion state

For every entry:
- Verify source SHA and byte size against the pinned tree/blob.
- Record canonical logical line count and complete EOF coverage.
- Canonical logical line count is the number of LF-terminated lines, plus one only when the blob does not end in LF. Do not count the synthetic empty split element after a final LF.
- Multi-range coverage must be consecutive, gap-free and end at the canonical final line.
- Separate observed source behavior from interpretation, suspected defects, target-version implications and proposed RustChunk design. Do not claim a suspected issue was reproduced unless it was.
- Primary Decision vocabulary: **essential**, **adaptable**, **rewrite required**, **integrate**, **unnecessary**, **undetermined**. Secondary qualifiers may include **reference**, **version-specific**, **optional/future**. Qualifiers do not replace a primary decision. Index decisions must agree with the Ledger.

## 7. Six-document checkpoint transaction

- The applicable Ledger is named for its own 200-file stage (`AUDIT_LEDGER_<first>_<last>.md`). Never append entries from a new stage to a previously closed stage's Ledger. If a historical cross-stage append is discovered, split at the exact entry boundary without renumbering or altering entry evidence, then verify the new partition against the Index.

At the end of every exact 25-file checkpoint, update these six documents together:
1. The applicable `AUDIT_LEDGER_<first>_<last>.md`.
2. `AUDIT_INDEX.md`.
3. `AUDIT_QA_LOG.md`.
4. `AUDIT_CONTINUITY.md`.
5. `AUDIT_PROTOCOL.md`, whenever current state or a governing rule needs synchronization; keep it consistent with Continuity.
6. `AUDIT_200_FILE_BOUNDARY.md`, while a 200-file stage is active; it is part of the same atomic six-document checkpoint transaction.

Before committing, verify:
- Exactly 25 new sequential entry numbers were added.
- All 25 paths are unique against the entire previous Index and within the checkpoint.
- All 25 entries contain all 15 required fields exactly once.
- Source SHAs and byte sizes match the pinned upstream tree.
- Coverage ranges are complete, consecutive and end at EOF.
- Ledger and Index paths, numbering and decisions align.
- The Index is globally sequential and contains no repeated paths.
- Active Ledger entry count and Continuity resume state are correct.

Commit the six-document checkpoint as one Git commit. Re-fetch all six documents from GitHub and verify their saved contents/blob SHAs and expected state. Do not call a checkpoint durable until read-back succeeds.

## 8. Reporting and cross-chat continuity

- Never leave more than one 25-file checkpoint inspected but uncommitted.
- After the first 25 files of an authorized 50, commit and verify that checkpoint, then continue to the second 25 without crossing the 50-file boundary.
- After the second 25, run the 50-file boundary checks, report the exact result and next range, then STOP for explicit user authorization.
- `AUDIT_CONTINUITY.md` is the durable cross-chat resume record. Read it and the latest branch head before every continuation. Update it in each 25-file checkpoint commit.
- If a new chat or lost context makes the state uncertain, re-fetch branch head and all five audit documents before source work. Never guess a resume point from conversational memory alone.
- Historical QA entries describing old cadence remain historical evidence, not current instructions.

## 9. Work pacing and concise communication

- Work **serially, one item at a time**. Inspect one source file or perform one independent validation step to completion before starting another. Do not run unrelated file reads, checks or investigations in parallel.
- For a long source file, read its ranges sequentially in order, each no larger than 400 logical lines. Do not interleave multiple files or leave a file unfinished while switching to another.
- Keep each tool operation and work unit small and bounded. Avoid large multi-action runs that leave the user watching one unchanged response for a long time.
- After each completed unit, preserve the exact state needed to resume. A 25-file checkpoint is still committed as one transaction after its 25 files are individually completed; do not create partial or unverified checkpoint claims.
- When a task is taking long enough that the user may reasonably think it is stuck, provide a brief status update identifying only the current item. Do not send repetitive progress narration or filler.
- User-facing reports must be **very short**. Normally include only: completed range/status, whether its required verification succeeded or what blocked it, and the exact next point or required user authorization.
- Do not volunteer file-by-file summaries, architecture explanations, descriptions of which file reads which other file, implementation plans, or lists of internal changes. Provide those details only when the user asks.
- Keep detailed evidence in the Ledger/QA/Continuity records, not in long chat reports. Do not repeat information already recorded in the prior report.
- At a 25-file checkpoint, a compact confirmation is sufficient. At a 50-file boundary, give one concise report and STOP for explicit authorization. Never continue across the 50-file boundary on implied permission.
- If interrupted or moved to another chat, resume from the latest verified Git commit and Continuity record. Do not restart a completed range or guess what was done.

## 10. QA scope and claim discipline

- QA of existing entries is separate from new source-audit checkpoints. QA must not silently create, renumber or duplicate source entries.
- Structural QA checks sequential numbering, global path uniqueness, exact 15-field counts, source SHA/byte-size matches, canonical line counts, EOF coverage, Ledger/Index alignment and six-document consistency.
- Structural/provenance QA is not automatically a fresh exhaustive semantic re-audit. State the actual scope.
- Never claim a Rust build, unit test, benchmark, APK build, Forge launch, Android-device test, Minecraft multiplayer join, LAN host, UPnP or relay validation unless actually executed. Record unrun checks as pending.
- Preserve security findings. Do not treat Starling/Horizon native-mod build and app-library replacement mechanisms as ordinary Forge mod loading or adopt them without a separate security design and review.

## 11. Backup policy

- Create a new backup only after each additional **1,000 newly completed audit entries** have been committed and globally verified.
- Entries **1–2000** have passed global verification on 2026-11-21. The single final backup branch is `backup/audit-2000-final-2026-11-21`; no incremental backups were created.
- Do not create backups at 200-entry stage boundaries, 100-entry points, 50-file reporting boundaries or 25-file checkpoints.
- Never modify an existing backup branch. Existing backup refs are procedurally read-only; GitHub currently reports branch protection disabled, so do not describe them as technically locked.
- Do not recreate or alter a user-deleted archive branch. Do not copy audit ledgers onto `main`.

## 12. Current authoritative resume state (2026-11-21)

- Repository: `imamirhosein/RustChunk`; working branch: `starling-1.20.1-port-audit`.
- Completed source entries: **1–2375**.
- Active 200-file stage: **2201–2400**, progress **175/200**.
- Sections **2201–2250**, **2251–2300** and **2301–2350** are closed (50/50 each); section **2351–2400** is in progress (25/50).
- The user's one-time expanded authorization covers **2301–2400**. Checkpoints through **2351–2375** are complete; **2376–2400** remains authorized. Continue without pausing; **STOP at 2400** and request fresh permission before 2401.
- Closed Ledger `AUDIT_LEDGER_2001_2200.md`: 200 records (2001–2200). Active Ledger `AUDIT_LEDGER_2201_2400.md`: **175 records (2201–2375)** with exactly 15 fields each. Index: **2,375 sequential unique paths**.
- Pinned upstream `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; full recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete. Future selection excludes all Index paths plus any current in-flight checkpoint paths; select the next lexical paths after the pending boundary to avoid duplicates.
- Entries 1–2000 have final backup `backup/audit-2000-final-2026-11-21`. Next backup only after entry 3000 and global verification.
- Tests/examples inspected only; none run. No build, benchmark, APK, device, renderer, Minecraft, Forge, server or Aternos validation. No application source change.

## 13. One-time expanded authorization — 2001–2100 — 2026-11-21

- The user explicitly expanded this one reporting window to **100 source entries**, covering adjacent sections **2001–2050** and **2051–2100**.
- This is not a permanent change to the 200 / 50 / 25 protocol. Keep 25-file checkpoint commits and report at every 50-file boundary.
- At 2050, report completion of 2001–2050 and continue the already-authorized 2051–2100 without another permission request.
- At 2100, close the section and STOP; get fresh explicit permission before inspecting entry 2101 or later.

## 13. Historical corrections preserved

- In entries 1–1000, duplicate slots 631, 632, 633, 754, 762 and 766 were replaced with unique paths: `pick_block_data_fixture.rs`, `place_recipe_fixture.rs`, `player_sync_fixture.rs`, `raid_fixture.rs`, `random_tick_sections_probe.rs` and `recipe_book_fixture.rs`.
- Entry 1089 was corrected to `crates/horizon-client/src/game/blockctl.rs`; entry 121 remains the only `entry.rs` occurrence.
- Preserve these corrections and all prior QA evidence; never repeat those paths.


## Duplicate-path correction record — 2026-11-19

- QA detected that entry 1349 initially duplicated `crates/horizon-client/src/lib.rs` at historical entry 118. Preserve entry 118; the duplicate slot was corrected.
- Final entry 1349 is `crates/horizon-client/src/locator_bar.rs`; final entry 1350 is `crates/horizon-client/src/lod_net.rs`.
- Do not repeat `lib.rs` at entry 1349. The final Index must contain 1350 sequential entry numbers and 1350 unique paths before the 1301–1350 reporting section is considered closed.

## 14. Authorization and resume update — 2026-11-19

- The user authorized section **1401–1450**; both checkpoints 1401–1425 and 1426–1450 are now complete and committed.
- The 50-file section is closed at **1450**.
- **No authorization exists yet for 1451–1500.** Wait for explicit user approval before inspecting any file in that range.
- Each future 50-file section must contain two exact 25-file checkpoints; update all five audit documents together after each checkpoint.
- Inspect files serially, with consecutive chunks of at most 400 lines for larger files. Keep user reports very short.
- Do not alter `main`, existing backup refs or deleted archive branch. No backup is due at 1450; next backup is at the 1000-entry cadence.



## 14. Authorization and resume update — checkpoint 1451–1475 — 2026-11-19

- The user explicitly authorized section **1451–1500**.
- Checkpoint **1451–1475** is documented in all five audit documents and committed/read-back verified.
- Continue with the already-authorized checkpoint **1476–1500** without requesting intermediate permission.
- After 1500, stop and wait for explicit user authorization before 1501–1550.
- Preserve the exact 200 / 50 / 25 cadence, serial file inspection and maximum 400-line chunk rule.
- No build/test/device/server validation and no backup is due at this checkpoint.


## Current authoritative resume state — section 1451–1500 closed — 2026-11-19

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Completed entries: **1–1500**.
- Active 200-file stage: **1401–1600**, completed **100/200**.
- Closed reporting section: **1451–1500**, both 25-file checkpoints complete.
- Active Ledger: `AUDIT_LEDGER_1401_1600.md`, 100 entries (1401–1500), each with 15 required fields. Index: 1,500 sequential unique paths.
- The semantic field-value shift in 1451–1475 was corrected; decisions align between Ledger and Index for 1451–1500.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- **Next proposed section 1501–1550 is not authorized. Stop and wait for explicit user approval.**
- Preserve 200 / 50 / 25 cadence, serial review and maximum 400-line chunks. Do not modify main or backup refs.
- No tests/builds/device/server validation were performed. No backup is due; next backup remains after 1001–2000 complete and globally verified.


## Current authorized section and resume state — 1501–1525 — 2026-11-19

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Completed entries: **1–1525**.
- Active 200-file stage: **1401–1600**, completed **125/200**.
- Section **1501–1550** is authorized. Checkpoint **1501–1525** is complete; proceed to **1526–1550** without intermediate permission.
- Stop at **1550**. Section **1551–1600** requires new explicit authorization.
- Active Ledger: `AUDIT_LEDGER_1401_1600.md`, 125 entries (1401–1525), each with 15 fields. Index: 1,525 sequential unique paths.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Preserve four 50-file sections per 200 and two 25-file checkpoints per section; inspect serially in chunks no larger than 400 lines.
- No build/test/device/server validation. No backup due; next backup after 1001–2000 complete and verified. Do not modify `main` or backups.


## Current authoritative resume state — section 1501–1550 closed — 2026-11-19

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Completed entries: **1–1550**.
- Active 200-file stage: **1401–1600**, completed **150/200**.
- Closed section: **1501–1550**; both 25-file checkpoints are complete and committed.
- Active Ledger: `AUDIT_LEDGER_1401_1600.md`, 150 entries (1401–1550), each with 15 required fields. Index: 1,550 sequential unique paths.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- **STOP at 1550. Section 1551–1600 is not authorized and requires explicit user approval before inspecting any source in that range.**
- Preserve the four 50-file sections per 200-file stage, two 25-file checkpoints per section, serial file review and maximum 400-line chunks.
- No tests/builds/device/server validation was performed. No backup is due; next backup after entries 1001–2000 are completed and globally verified. Do not modify `main` or backups.

## Latest authoritative resume state — checkpoint 1551–1575 — 2026-11-19

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1575**.
- Active 200-file stage: **1401–1600**, completed **175/200**.
- Active reporting section: **1551–1600**. Checkpoint **1551–1575** is complete and committed; **1576–1600 is already authorized** by the same user instruction. Proceed without asking for mid-section permission.
- Active Ledger: `AUDIT_LEDGER_1401_1600.md`, **175 entries** (1401–1575), with exactly 15 fields each. Index: **1,575 sequential unique paths**.
- The 25 files 1551–1575 were read through EOF in serial order; each file above 400 lines was read in continuous chunks no larger than 400. SHA/size/coverage are in the Ledger.
- **Next source file is entry 1576**, the next unindexed blob in the pinned tree. Verify the remaining source-tree order and no duplicate paths before reading.
- Preserve exact 200 / 50 / 25 cadence, serial review, complete EOF and max 400-line chunks. After entries 1576–1600, update all five audit docs in one atomic commit, read all five back individually and verify branch HEAD.
- **STOP at entry 1600** and wait for new explicit authorization before 1601 or any later source file.
- No source/application code changed. Embedded tests are inspected only, not run. No build/test/benchmark/APK/device/Forge/server validation has been performed.
- `main` and existing backup refs remain untouched. No backup is due; next backup only after entries 1001–2000 are complete and globally verified.

## Latest authoritative resume state — section 1551–1600 closed — 2026-11-19

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1600**.
- Active 200-file stage: **1401–1600**, now **200/200 complete**.
- Closed reporting section: **1551–1600**, both 25-file checkpoints complete.
- Active Ledger: `AUDIT_LEDGER_1401_1600.md`, **200 entries** (1401–1600), with the 15 required fields correctly aligned. Index: **1,600 sequential unique paths**.
- QA correction: entries 1551–1575 were semantically re-aligned in the 1576–1600 transaction. Index descriptions now populate Purpose; prior values occupy the correct Dependencies, Implementation, Findings, Relevance, Implications, Follow-up, Mapping and Evidence fields. Source SHA/size/coverage/path/Decision were preserved.
- The 25 files 1576–1600 were read through EOF serially, in chunks no larger than 400 lines; pinned SHAs, byte sizes and contiguous coverage are recorded.
- **STOP at entry 1600. Entry 1601 or later must not be inspected until fresh explicit user authorization.**
- Preserve exact 200 / 50 / 25 cadence, serial inspection and maximum 400-line chunks. On next authorization, verify branch HEAD and current Index/tree order before entry 1601.
- No source/application code changed. Embedded tests were inspected only, not run. No build, unit test, benchmark, APK, Android device, Forge launch, Minecraft join, LAN, UPnP or relay validation has been performed.
- `main` and backup refs remain untouched. No backup is due; backup only after entries 1001–2000 are complete and globally verified.

## Latest authoritative resume state — checkpoint 1601–1625 — 2026-11-19

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1625**.
- Active 200-file stage: **1601–1800**, completed **25/200**.
- Active reporting section: **1601–1650**. Checkpoint **1601–1625** is complete; checkpoint **1626–1650** is already authorized by the same user instruction. Proceed without asking for mid-section permission.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **25 entries** (1601–1625), with exactly 15 fields each. Previous Ledger `AUDIT_LEDGER_1401_1600.md` is closed. Index: **1,625 sequential unique paths**.
- The 25 files 1601–1625 were read through EOF serially; files above 400 logical lines were read in consecutive chunks of at most 400. SHA/size/coverage are recorded in the active Ledger.
- **Next source is entry 1626**, verified as the next unindexed blob in pinned tree order. Recheck the next 25 paths and duplicate status before reading.
- Preserve exact 200 / 50 / 25 cadence, serial inspection, complete EOF and maximum 400-line chunks. After entries 1626–1650, update all five audit docs in one atomic commit, read all five back individually and verify branch HEAD.
- **STOP at entry 1650** and wait for new explicit authorization before 1651 or any later source file.
- No source/application code changed. Embedded tests are inspected only, not run. No build/test/benchmark/APK/device/Forge/server validation has been performed.
- `main` and backup refs remain untouched. No backup is due; next backup only after entries 1001–2000 are complete and globally verified.

## Latest authoritative resume state — checkpoint 1626–1650 — 2026-11-19

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1650**.
- Active 200-file stage: **1601–1800**, completed **50/200**.
- Reporting section **1601–1650 is closed** (checkpoints 1601–1625 and 1626–1650).
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **50 entries** (1601–1650), exactly 15 fields each. Index: **1,650 sequential unique paths**.
- The 25 files 1626–1650 were read completely through EOF, in exact pinned tree order. Files above 400 lines were read in gap-free chunks of at most 400; SHA/size/coverage are recorded.
- **STOP at entry 1650.** Do not inspect 1651 or later until fresh explicit user authorization. The next checkpoint after authorization is 1651–1675; do not request permission mid-section once 1651–1700 is authorized.
- Preserve 200 / 50 / 25 cadence, serial file inspection, full EOF and max 400-line chunks. After each 25 files, commit all five audit docs atomically, read each back individually and verify branch HEAD.
- No source/application code changed. Embedded tests are inspected only, not run. No build/test/benchmark/APK/device/Forge/server validation has been performed.
- `main` and backup refs remain untouched. No backup is due; next backup only after entries 1001–2000 are complete and globally verified.

## Latest authoritative resume state — checkpoint 1651–1675 — 2026-11-20

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1675**.
- Active 200-file stage: **1601–1800**, completed **75/200**.
- Active reporting section: **1651–1700**. Checkpoint **1651–1675** is complete and committed; **1676–1700 is already authorized** by the same user instruction. Proceed without asking for mid-section permission.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **75 entries** (1601–1675), with exactly 15 fields each. Index: **1,675 sequential unique paths**.
- The 25 files 1651–1675 were read through EOF serially; files above 400 lines were read in continuous chunks no larger than 400. SHA/size/coverage are in the Ledger.
- **Next source file is entry 1676**, the next unindexed blob in pinned tree order. Verify remaining source-tree order and no duplicate paths before reading.
- Preserve exact 200 / 50 / 25 cadence, serial review, complete EOF and max 400-line chunks. After entries 1676–1700, update all five audit docs in one atomic commit, read all five back individually and verify branch HEAD.
- **STOP at entry 1700** and wait for fresh explicit authorization before 1701 or any later source file.
- No source/application code changed. Embedded tests are inspected only, not run. No build/test/benchmark/APK/device/Forge/server validation has been performed.
- `main` and existing backup refs remain untouched. No backup is due; next backup only after entries 1001–2000 are complete and globally verified.

## Latest authoritative resume state — checkpoint 1676–1700 — 2026-11-20

- Repository: imamirhosein/RustChunk; branch: starling-1.20.1-port-audit.
- Pinned source: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488.
- Completed entries: **1–1700**.
- Active 200-file stage: **1601–1800**, completed **100/200**.
- Section **1651–1700** is closed. Both 25-file checkpoints 1651–1675 and 1676–1700 are complete.
- Active Ledger: AUDIT_LEDGER_1601_1800.md, **100 entries** (1601–1700), exactly 15 fields each. Index: **1,700 sequential unique paths**.
- The 25 files 1676–1700 were read through EOF serially; files above 400 lines were read in continuous chunks no larger than 400. SHA/size/coverage are in the Ledger.
- **STOP at entry 1700**. Do not inspect entry 1701 or later until the user gives fresh explicit authorization.
- Preserve exact 200 / 50 / 25 cadence, serial review, complete EOF and max 400-line chunks. At the next authorized checkpoint, update all five audit docs atomically, read all five back individually and verify branch HEAD.
- No source/application code changed. Embedded tests were inspected only, not run. No build/test/benchmark/APK/device/Forge/server validation has been performed.
- main and existing backup refs remain untouched. No backup is due; next backup only after entries 1001–2000 are complete and globally verified.

## Latest verified checkpoint — 1701–1725 — 2026-11-20

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1725**.
- Active 200-file stage: **1601–1800**, completed **125/200**.
- Section **1701–1750** is authorized. Checkpoint **1701–1725** is complete; continue with **1726–1750** under the same authorization.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **125 entries** (1601–1725), exactly 15 fields each. Index: **1,725 sequential unique paths**.
- The 25 files 1701–1725 were read through EOF serially; ranges for files over 400 lines are consecutive and no larger than 400. SHA/size/coverage match the pinned tree. Index paths/Decisions align with the Ledger.
- Keep all tests/examples as inspected-only unless explicitly run later; no tests/builds/device/server validation was performed for this checkpoint.
- After checkpoint 1726–1750, update and read back all five audit documents in one commit; **STOP at 1750**.
- Do not modify `main` or backup refs. No backup is due before entries 1001–2000 pass global verification.


## Latest verified checkpoint — 1726–1750 — 2026-11-20

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1750**.
- Active 200-file stage: **1601–1800**, completed **150/200**.
- Reporting section **1701–1750 is closed**; checkpoints **1701–1725** and **1726–1750** are complete.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **150 entries** (1601–1750), exactly 15 fields each. Index: **1,750 sequential unique paths**.
- The 25 files 1726–1750 were read through EOF serially; ranges for files over 400 lines are consecutive and no larger than 400. SHA/size/coverage match the pinned tree. Index paths/Decisions align with the Ledger.
- Density graph, sampler and parity architectures are research references only. Java 26.3-specific biome/End-island algorithms and newer density nodes require target-era reconstruction. Tests were inspected only and not run.
- No source/application code changed. No build/test/benchmark/APK/device/Forge/server validation was performed.
- **STOP at entry 1750**. Do not inspect 1751–1800 until fresh explicit user authorization.
- Do not modify `main` or backup refs. No backup is due; next backup only after entries 1001–2000 are complete and globally verified.


## Latest verified checkpoint — 1751–1775 — 2026-11-20

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1775**.
- Active 200-file stage: **1601–1800**, completed **175/200**.
- Reporting section **1751–1800** is authorized. Checkpoint **1751–1775** is complete; continue with **1776–1800** under the same authorization.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **175 entries** (1601–1775), exactly 15 fields each. Index: **1,775 sequential unique paths**.
- The 25 files 1751–1775 were read through EOF serially; ranges for files over 400 lines are consecutive and no larger than 400. SHA/size/coverage match the pinned tree. Index paths/Decisions align with the Ledger.
- Findings: geometry/collision, density/noise and palette code are useful architecture references, but the pinned core is explicitly Java 26.3 and not a playable client. 1.20.1 RNG, terrain, palette, Android ARM64 math and full game integration require separate verification.
- Embedded tests/benchmarks were inspected only and not run. No build, tests, APK, device, Forge or server validation was performed.
- After checkpoint 1776–1800, commit/read back all five audit documents and **STOP at 1800**.
- Do not modify `main` or backup refs. No backup is due before entries 1001–2000 pass global verification.


## Latest verified checkpoint — 1776–1800 — 2026-11-20

- Repository: `imamirhosein/RustChunk`; branch: `starling-1.20.1-port-audit`.
- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Completed entries: **1–1800**.
- Active 200-file stage **1601–1800 is complete (200/200) and closed**.
- Reporting section **1751–1800 is closed**; checkpoints **1751–1775** and **1776–1800** are complete.
- Active Ledger: `AUDIT_LEDGER_1601_1800.md`, **200 entries** (1601–1800), exactly 15 fields each. Index: **1,800 sequential unique paths**.
- The 25 files 1776–1800 were read through EOF serially; ranges for files over 400 lines are consecutive and no larger than 400. SHA/size/coverage match the pinned tree. Index paths/Decisions align with the Ledger.
- Findings: coordinate, RNG, packed storage and tick subsystems are substantial research references, but exact 1.20.1 random/tick/palette semantics need target-source reconstruction. Cosmetics are optional third-party features; DualSense HID/CPAL has no proven Android backend.
- Embedded tests and hardware examples were inspected only and not run. No build/test/benchmark/APK/device/Forge/server validation was performed.
- **STOP at entry 1800**. Do not inspect **1801–1850** until fresh explicit user authorization.
- Do not modify `main` or backup refs. No backup is due before entries 1001–2000 are complete and globally verified.



## 13. Authoritative stage 1801–2000 register and checkpoint artifacts — 2026-11-20

This section supersedes older stage-specific Ledger names and the six-document list for the current 1801–2000 stage only; historical checkpoint records remain unchanged.

- Active Ledger: `AUDIT_LEDGER_1801_2000.md`; do not append new-stage records to the closed `AUDIT_LEDGER_1601_1800.md`.
- The explicit 200-file boundary/progress artifact is `AUDIT_200_FILE_BOUNDARY.md`. It records all four 50-file sections, all eight 25-file checkpoints, authorization state, completion state and stop boundary.
- At each 25-file checkpoint in 1801–2000, update and commit **six documents in one transaction**: the active Ledger, `AUDIT_INDEX.md`, `AUDIT_QA_LOG.md`, `AUDIT_CONTINUITY.md`, `AUDIT_PROTOCOL.md`, and `AUDIT_200_FILE_BOUNDARY.md`.
- Read back each of those six files individually from GitHub after updating the branch; verify entry counts, all 15 Ledger fields, Index uniqueness/sequential numbering, path/Decision alignment, boundary status and matching HEAD.
- The 200-file stage is 1801–2000. Its 50-file sections are 1801–1850, 1851–1900, 1901–1950 and 1951–2000; each contains two 25-file checkpoints.
- User authorization currently covers **1801–1850 only**. Checkpoint 1801–1825 is complete; 1826–1850 is covered by the same authorization. **STOP at 1850** and obtain fresh explicit authorization before inspecting 1851 or later.
- `AUDIT_200_FILE_BOUNDARY.md` must mark later sections/checkpoints as locked/not authorized until the corresponding user permission arrives; never infer permission from a progress record.
- Preserve serial source reading, complete EOF, maximum 400-line contiguous chunks, source SHA/size verification, no test execution and no application code changes. Keep main and existing backup refs untouched.
- No backup is due until all entries 1001–2000 are complete and global verification of entries 1–2000 has passed.


## Corrected checkpoint ordering note — 2026-11-20

For the 1801–1850 section, derive the canonical path sequence by excluding only entries 1–1800 from the pinned recursive tree, then take the next 50 blobs. Do not exclude already registered but potentially misnumbered records from 1801 onward when reconciling this section. The missing `crates/horizon-friends/src/lib.rs` belongs at 1812. Checkpoint 1801–1825 ends at `crates/horizon-items/src/blocks/decode.rs`; `blocks/decode_tests.rs` belongs at 1826. The correction must be committed/read back as a checkpoint transaction.


## 15. Stage 1801–2000 progress after section 1801–1850 — 2026-11-20

- Entries 1801–1850 complete and closed (50/200). Corrected order and the omitted `horizon-friends/src/lib.rs` at 1812 are recorded in QA/Continuity; nonexistent `horizon-mod-registry/src/storage.rs` is not an entry.
- Both 25-file checkpoints committed as separate six-document transactions; all six docs were read back after each commit.
- Section 1851–1900 is locked; do not inspect it without fresh explicit user authorization.
- Preserve serial review, full EOF, <=400-line contiguous chunks, source SHA/size verification, exactly 15 Ledger fields, no unverified test/build claims and no app source changes.

## 16. Checkpoint 1851–1875 closeout and authorization state — 2026-11-20

- Active stage 1801–2000: **75/200 complete**. Entries 1801–1850 are closed; checkpoint 1851–1875 is complete.
- Section 1851–1900 is authorized and active (25/50). Continue serially at 1876 through 1900 under the same grant; stop at 1900 and obtain fresh explicit permission before 1901.
- At this checkpoint the six-document transaction is mandatory: active Ledger, `AUDIT_INDEX.md`, `AUDIT_QA_LOG.md`, `AUDIT_CONTINUITY.md`, this Protocol and `AUDIT_200_FILE_BOUNDARY.md`. Read back each separately after commit and verify HEAD, 15 fields per Ledger entry, sequential unique Index paths and matching decisions.
- Findings from 1851–1875: Horizon custom package/content/index and native bundle formats are not Forge 1.20.1 equivalents. Registry/index logic needs URL, ZIP bomb, revocation and trust-governance hardening before any reuse. Modpack extraction needs symlink-safe writes. Narrator code lacks Android target support and uses unbounded queues; UI narration remains an optional accessibility reference.
- Keep one source fetch at a time, complete EOF, contiguous chunks <=400, pinned SHA/size verification, no test execution, no app source changes, and no unsupported validation claims. `main` and backups remain unchanged. Backup only after entries 1001–2000 and global verification 1–2000.


## 17. Post-commit read-back correction — 2026-11-20

- If read-back detects malformed Ledger field joins or Index row joins, correct the document formatting transparently in a documentation-only follow-up commit, retain original content/decisions, and read back all six checkpoint documents again.
- For checkpoint 1851–1875, two inherited Ledger joins (1823, 1847) and three inherited Index joins (660, 995, 1660) were repaired. Final counts: 75 Ledger entries with 15 fields each; 1,875 sequential unique Index paths.


## 18. Closeout of authorized section 1851–1900 — 2026-11-20

- Entries 1876–1900 completed as the second 25-file checkpoint; section 1851–1900 is now closed at 50/50.
- Six-document atomic update required: Ledger, Index, QA, Continuity, Protocol and Boundary. Commit all six together, read each back individually and verify 100 Ledger entries/15 fields each, 1,900 sequential unique Index paths, source SHA/size and matching Ledger/Index paths/decisions.
- Do not inspect 1901–1950 without fresh explicit user authorization. Stop at 1900.
- Main/backup refs remain untouched; no source changes or test/build/device/Forge/server/renderer claims. Backup only after the full 1001–2000 range and global verification.


## 19. Current stage state after checkpoint 1901–1925 — 2026-11-20

- Active 200-file stage 1801–2000 is **125/200 complete**.
- Authorized section: **1901–1950**. Checkpoint 1901–1925 is closed; checkpoint 1926–1950 remains authorized by the same user permission.
- Active Ledger `AUDIT_LEDGER_1801_2000.md` contains 125 entries (1801–1925), exactly 15 fields each. Index contains 1,925 sequential unique paths.
- At checkpoint 1926–1950, update/commit all six audit artifacts atomically and read each back individually. Stop at 1950; 1951+ remains locked pending fresh permission.
- All existing rules remain: one file at a time, complete EOF, <=400-line gap-free chunks, pinned SHA/size, no tests claimed executed, and no application code changes.


## 20. Closeout of authorized section 1901–1950 — 2026-11-20

- Entries 1926–1950 are complete as the second 25-file checkpoint; section 1901–1950 is closed at 50/50.
- Six-document atomic update required and completed: active Ledger, Index, QA Log, Continuity, Protocol and Boundary register. Read each back individually; verify 150 Ledger entries/15 fields each, 1,950 sequential unique Index paths, path/Decision alignment and source SHA/size.
- Do not inspect 1951–2000 without fresh explicit user authorization. Stop at 1950.
- Main and backup refs remain untouched; no source/application code changes or test/build/device/Forge/server/renderer claims. Backup only after entries 1001–2000 and global 1–2000 verification.


## 21. Checkpoint 1951–1975 — 2026-11-20

- Entries 1951–1975 are complete as the first 25-file checkpoint of section 1951–2000.
- All 25 paths are the next blobs after excluding Index entries 1–1950 from the complete pinned tree. Every file was inspected serially through EOF; files over 400 lines were fetched in consecutive, gap-free chunks <=400 lines.
- Six-document checkpoint update and read-back is required and is being committed together: Ledger, Index, QA Log, Continuity, Protocol and Boundary register.
- Ledger now holds 175 entries (1801–1975), exactly 15 fields each; Index has 1,975 sequential unique paths. SHA/size must match the complete pinned tree and Ledger paths/Decisions must match Index.
- The user's authorization for section 1951–2000 continues. Proceed with entries 1976–2000 one file at a time. Do not stop at 1975 or ask for intermediate permission.
- At entry 2000, close section 1951–2000 and STOP. Ask the user before starting global verification of entries 1–2000 and creating the final backup.
- Embedded tests/examples inspected only; none executed. No build/device/Forge/server/renderer claims. No application source code changes.


## 22. Final checkpoint 1976–2000 — 2026-11-20 (historical stop state; superseded by section 23)

- Entries 1976–2000 are complete as the second 25-file checkpoint of section 1951–2000.
- They are exactly the next 25 blobs after excluding Index entries 1–1975 from the complete pinned tree. Each file was inspected sequentially through EOF; files >400 lines were read in contiguous, gap-free chunks <=400 lines.
- Updated and committed all six audit documents together: Ledger, Index, QA Log, Continuity, Protocol and Boundary register.
- Ledger contains 200 entries (1801–2000), exactly 15 required fields per entry; Index contains 2,000 sequential unique paths; path/Decision alignment and source SHA/size must be read-back verified against the complete pinned tree.
- Section **1951–2000 is CLOSED (50/50)**. Stage **1801–2000 is COMPLETE (200/200)**. All prior 50-file sections are also closed.
- **STOP at 2000.** Do not start global verification of all entries 1–2000 until the user explicitly authorizes that phase. After global verification passes, request/obtain permission as needed for the final backup; no incremental backups.
- Embedded tests/examples inspected only; none executed. No build/APK/device/Forge/server/Aternos/renderer validation; no application source changes.


## 23. Global verification and final backup — 2026-11-21

- Global structural/provenance QA passed for entries **1–2000**: 2,000 sequential Ledger entries and Index rows; 2,000 unique paths; exact 15-field sets; Ledger/Index path and decision agreement; all source paths, SHAs and byte sizes match the complete pinned Starling tree.
- Documentation corrections: added missing exact byte sizes to entries 1–20 and 96–100; made the existing Index Decision classifications explicit in 97 Ledger Decision fields across entries 1201–1800, preserving the original explanations.
- Corrected the governing checkpoint documentation to state the six-document transaction (including the active-stage Boundary register) and synchronized the authoritative current state to 1–2000.
- Final backup branch: `backup/audit-2000-final-2026-11-21`, pointing to the post-verification audit commit. Verify the branch ref and read back its audit documents before reporting completion.
- This verification is structural, metadata, provenance and cross-document QA; it is not a fresh semantic source re-review or runtime compatibility evidence. No tests/builds/benchmarks/APK/device/Forge/server/Aternos/renderer checks were performed.
- No application source changed. `main` and both earlier 1,000-entry backup refs remain unchanged. **STOP at 2000**; a new source range requires fresh explicit user authorization.


## 14. Historical resume snapshot — after checkpoint 2026–2050 — 2026-11-21 (superseded by Section 12)

- Repository: `imamirhosein/RustChunk`; working branch: `starling-1.20.1-port-audit`.
- Completed source entries: **1–2050**.
- Active 200-file stage: **2001–2200**, progress **50/200**.
- Reporting section **2001–2050 is closed**. Checkpoints 2001–2025 and 2026–2050 are committed and read-back verified.
- The user's one-time expanded authorization covers the next section **2051–2100** as well. Keep exact 25-file checkpoint transactions and report at the 50-file boundary; do not request permission at 2050.
- Next checkpoint: **2051–2075**; next source path, derived from the complete pinned tree after excluding entries 1–2050: `crates/horizon-shaders/src/post.rs`.
- Continue through checkpoint 2076–2100, then **STOP at 2100** and request explicit authorization before 2101.
- **2101–2200 is not authorized.**
- Active Ledger: `AUDIT_LEDGER_2001_2200.md`, 50 entries. Index: **2,050 sequential unique paths**.
- Pinned upstream remains `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Next backup only after 1,000 additional completed entries (entry 3000) and global verification. No backup at 25/50/100/200-file boundaries.
- No tests/builds/benchmarks/APK/device/Minecraft/Forge/server/Aternos/renderer validation is implied. No application source changed.


## 15. Historical resume snapshot — after checkpoint 2051–2075 — 2026-11-21 (superseded by Section 12)

- Repository: `imamirhosein/RustChunk`; working branch: `starling-1.20.1-port-audit`.
- Completed source entries: **1–2075**.
- Active 200-file stage: **2001–2200**, progress **75/200**.
- Section **2001–2050 is closed**; checkpoint **2051–2075 is complete**. All six audit documents are updated together per checkpoint.
- The user's one-time expanded authorization covers entries **2001–2100**, with exact 25-file checkpoints and two 50-file reporting sections. Continue next at **2076–2100** without asking for permission.
- At entry **2100**, close section **2051–2100** and **STOP**. Entries 2101–2200 remain unauthorized.
- Active Ledger: `AUDIT_LEDGER_2001_2200.md`, 75 records. Index: **2,075 sequential unique paths**.
- Pinned upstream remains `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete (`truncated:false`).
- Next backup only after 1,000 additional completed entries (entry 3000) and global verification. No incremental backups at checkpoint boundaries.
- Tests/examples inspected only; no tests/builds/benchmarks/APK/device/Minecraft/Forge/server/Aternos/renderer validation. No application source changed.


## 16. Historical resume snapshot — after checkpoint 2076–2100 — 2026-11-21 (superseded by Section 12)

- Repository: `imamirhosein/RustChunk`; working branch: `starling-1.20.1-port-audit`.
- Completed source entries: **1–2100**.
- Active 200-file stage: **2001–2200**, progress **100/200** (one-time 100-file exception completed).
- Sections **2001–2050** and **2051–2100** are closed; all four 25-file checkpoints were committed and read-back verified.
- **STOP at 2100.** Entries 2101–2200 are not authorized. Obtain fresh explicit permission for the next 50-file section; the exceptional authorization does not roll over.
- Active Ledger: `AUDIT_LEDGER_2001_2200.md`, 100 records. Index: **2,100 sequential unique paths**.
- Pinned source remains `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`; recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is complete.
- Next backup only after 1,000 additional completed entries (entry 3000) and global verification. No incremental backups at section checkpoints.
- Tests/examples inspected only; no tests/builds/benchmarks/APK/device/Minecraft/Forge/server/Aternos/renderer validation. No application source changed.


## 17. Historical authorization record — entries 2101–2200 — 2026-11-21

- The user explicitly authorizes **all entries 2101–2200** as one exceptional 100-file window. This is a specific exception to the standard 50-file STOP/approval boundary; it does not permanently change the default 200 / 50 / 25 protocol.
- Preserve exact 25-file checkpoint transactions: 2101–2125, 2126–2150, 2151–2175, 2176–2200. At each checkpoint, update and commit the six audit documents together, re-fetch all six and verify structure/provenance/Index alignment before proceeding.
- At **2150 do not stop and do not ask for permission**. Continue directly with 2151–2175 and 2176–2200 under this same explicit authorization.
- At **2200**, close the 100-file exceptional window, report the checkpoint, and **STOP**. Entries 2201+ require fresh explicit permission.
- Latest verified state at this checkpoint: completed entries **1–2125**; active Ledger has **125 entries (2001–2125)**; Index has **2,125 sequential unique paths**. Branch HEAD before this checkpoint transaction was `0966527cdee32a1a5de0a97287aeb7d7491b08c3`.
- Tests/examples inspected only, not executed. No build, benchmark, APK, Android device, Forge, Minecraft multiplayer, Aternos or renderer validation is implied. No app source changed.
- Backup policy unchanged: no backup at 25/50/100/200-file checkpoint boundaries. Next backup only after 1,000 additional completed entries (entry 3000) and global verification.


## 18. Complete binary asset inspection — 2026-11-21

- A binary file is not complete merely because its extension, Git blob SHA, byte size or metadata is known. Never mark a binary asset COMPLETE after metadata-only inspection.
- Fetch the entire blob using Base64, decode every byte and prove the decoded length equals the exact pinned-tree byte size; verify the Git blob SHA. If bytes are missing/corrupt/unavailable, stop that entry, record the limitation and do not count it toward the 25-file checkpoint.
- For TrueType/OpenType SFNT, inspect signature and full table directory; validate table bounds; calculate every table checksum (handling head.checkSumAdjustment correctly) and whole-font checksum; inspect name, head/maxp/OS2, cmap coverage, hhea/hmtx metrics, loca/glyf outline records and GDEF/GSUB/GPOS scripts/features/lookups. Validate glyph references, contour/coordinate ranges and composite components. Derive language support from cmap/shaping, not filenames.
- For other binary formats, use a format-specific complete parser/validator; byte count/SHA alone proves provenance, not semantic integrity.
- Text remains full UTF-8 read-through-EOF in contiguous <=400-line ranges. For SVG, inspect full XML, dimensions/viewBox, every element/attribute, scripts/foreignObject/embedded resources, href/xlink and CSS URL references. Do not infer Android rendering compatibility without conversion/device validation.
- Binary Coverage must state exact decoded byte count and structures parsed; logical line count is not applicable. If complete semantic parsing is impossible, mark incomplete and do not count it.
- Keep static integrity distinct from Android rendering, runtime compatibility and performance.

## 19. Historical resume snapshot — after checkpoint 2126–2150 — 2026-11-21 (superseded by Section 12)

- Completed entries **1–2150**; active stage **2001–2200**, progress **150/200**.
- Active Ledger: **150 records (2001–2150)**, each with exactly 15 fields. Index: **2,150 sequential unique paths**.
- The second one-time authorization covers **2101–2200**. Continue checkpoints 2151–2175 and 2176–2200; do not ask or stop at 2150. **STOP at 2200** and ask before 2201.
- Checkpoint 2126–2150 comprises 11 fully decoded/parsed TTF files, three complete OFL texts, one complete Hugeicons MIT notice and seven complete SVG files; all paths/SHAs/sizes match the complete pinned tree.
- No reviewed font supports Arabic/Persian. Bricolage merged Cyrillic requires Geist attribution; font assets are OFL 1.1, Hugeicons subset MIT.
- No build/tests/benchmark/APK/device/Android renderer/Minecraft/Forge/multiplayer/Aternos validation; no application source changed.
- No incremental backup. Next backup only after 1,000 further completed entries (entry 3000) and global verification.


## 20. Historical resume snapshot — after checkpoint 2151–2175 — 2026-11-21 (superseded by Section 12)

- Completed source entries: **1–2175**; active stage **2001–2200**, progress **175/200**.
- Active Ledger contains **175 entries (2001–2175)** with exactly 15 required fields each. Index contains **2,175 sequential unique paths**.
- The second one-time authorization covers entries **2101–2200**. Final checkpoint **2176–2200** remains authorized; do not pause or ask at 2175. **STOP at 2200** and ask before 2201.
- Checkpoint 2151–2175 consists of 25 fully read SVG icon files with exact pinned SHA/size provenance. They are self-contained 24×24 vectors with no active/external resources. Preserve the adjacent Hugeicons MIT notice.
- SVG source integrity is not Android renderer validation. Conversion and target-device display remain required before adopting assets.
- No builds/tests/benchmarks/APK/device/Android renderer/Minecraft/Forge/server/Aternos validation; no application source changed.
- No incremental backup at this checkpoint; next backup only after 1,000 further completed entries (entry 3000) and global verification.


## 21. Historical final resume snapshot — after entry 2200 — 2026-11-21 (superseded by Section 12)

- Completed source entries **1–2200**. Active stage **2001–2200: 200/200 complete and closed**.
- Ledger contains **200 records (2001–2200)** with exactly 15 required fields; Index contains **2,200 sequential unique paths**.
- Both one-time authorization windows (2001–2100 and 2101–2200) are consumed. **STOP before entry 2201.** Require fresh explicit user permission before any further source inspection or audit.
- Entry 2184 is Pixelarticons' distinct MIT notice (Copyright © 2019 Gerrit Halfmann), separate from Hugeicons' MIT notice at 2143. Entry 2183 PNG has no dedicated license found at the icon root; ownership/redistribution permission remains unverified.
- Seven assets beyond entry 2200 were unintentionally fetched during the path-order correction. They are excluded from Ledger/Index and are not authorized or complete; do not inspect/adopt without fresh permission.
- No tests/builds/benchmarks/APK/device/Android renderer/Minecraft/Forge/server/Aternos validation; no application source changed.
- No incremental backup at entry 2200; the 1–2000 backup remains the only final backup pending later authorized global verification.


## 22. Historical checkpoint record — entries 2201–2225 — 2026-11-21

- Completed entries 2201–2225 (25/25) in deterministic lexical path order after excluding all paths in the full Index. Each file was read individually through EOF; SHA and exact byte size match the pinned recursive tree.
- Ledger AUDIT_LEDGER_2001_2200.md contains 225 entries (2001–2225) with exactly 15 required fields. Index contains 2,225 sequential unique paths.
- The 18 SVG entries are static UI glyphs. Pixelarticons' MIT notice is entry 2184; entry 2214 (star.svg) is separately identified as custom Horizon artwork and needs independent licensing confirmation.
- Theme entries contain unvalidated public float inputs and only source-level tests; the Windows GDI boot painter is not an Android backend. The unsafe heap-census allocator needs independent soundness review.
- The user has authorized the complete 50-file window 2201–2250. Preserve commits at 2201–2225 and 2226–2250. Do not stop/request permission at 2225; STOP only at 2250.
- No tests/builds/benchmarks/APK/device/renderer/Minecraft/Forge/server/Aternos validation; no application source changed. No backup at this checkpoint.


## 23. Historical closed reporting section record — entries 2201–2250 — 2026-11-21

- The user authorized exactly one regular 50-file section, **2201–2250**, with the standard two 25-file checkpoint commits. Both checkpoints are complete and verified.
- Ledger contains **250 records (2001–2250)** with all 15 required fields exactly once. Index contains **2,250 sequential unique paths**; Ledger/Index Decisions align.
- Entries 2226–2250 include thread-pool, cross-platform thread and memory watcher facilities plus examples/fixtures for attributes, effects, worldgen, biome, collision, block-entity and landing behavior. The benchmark verifier flaw and fixture-oracle limitations are documented in Ledger/QA.
- Tests/examples were inspected only; none were run. No application source change or runtime validation.
- **STOP at 2250.** The authorization is exhausted; 2251–2300 requires fresh explicit permission. Preserve 25-file commits and 50-file reporting boundaries by default.


## 24. Current one-time authorization — entries 2251–2300 — 2026-11-21

- The user explicitly authorized the next standard 50-file section **2251–2300** in the current conversation after the discrepancy review.
- Preserve the normal two 25-file checkpoints: **2251–2275** and **2276–2300**. Complete the first checkpoint, commit and read back all six audit documents, then continue directly to the second without another permission request.
- At entry **2300**, close section **2251–2300** and STOP. Do not inspect entry 2301 without fresh explicit permission.
- This authorization does not change the normal 200/50/25 cadence and does not authorize any later section.


## 25. Index chronology and source-tree order reconciliation — 2026-11-21

70 adjacent transitions relative to the pinned GitHub recursive-tree API order were found among the 2,250 stable Index rows: 69 occur in entries 1–2000, and one occurs at entry 2033. No additional raw-tree-position regressions occur from entry 2034 through 2250. This is a historical ordering divergence, not a missing-file, duplicate-path or source-provenance defect: all 2,250 Index paths are unique and present in the pinned tree. The audit IDs are stable historical chronology and must not be renumbered because Ledger, QA and cross-document references depend on them. Do not describe the whole Index as a literal raw recursive-tree traversal.

The current latest path is `crates/horizon-world/examples/probe/block_landing_fixture.rs`. A full-set comparison found zero unindexed paths lexically before/equal to this boundary; all 2,718 remaining unindexed blobs are later. For the already-authorized **2251–2300** section, select the next paths in deterministic lexical order strictly after that path, excluding the full Index, then verify exact path, source SHA, size and uniqueness. Keep the standard 25/50/200 protocol.


## 26. Checkpoint 2251–2275 — 2026-11-21

- Completed entries **2251–2275 (25/25)** serially through EOF in deterministic lexical order after excluding the full Index. The 25 paths, source SHAs and byte sizes match the complete pinned recursive tree.
- Ledger `AUDIT_LEDGER_2001_2200.md` now has **275 records (2001–2275)**, each with exactly 15 required fields. Index has **2,275 sequential unique paths** with matching path/Decision rows.
- Three long fixtures (`block_update_fixture.rs` 474 lines, `collision_context_fixture.rs` 450, `consumable_fixture.rs` 413) were read in contiguous, gap-free chunks no greater than 400 lines.
- Findings concern mock-world assumptions, trusted corpus inputs, unbounded/unchecked fixture operations, incomplete enchantment paths and synthetic registry constraints. These examples are oracle transports, not production features. No example/test was executed.
- The second checkpoint **2276–2300** is already covered by the same user authorization. Continue without asking at 2275. **STOP at 2300**, then request new permission before 2301.
- No app source changed; no tests/builds/benchmarks/APK/device/renderer/Minecraft/Forge/server/Aternos validation. No backup at this checkpoint.

## 26. Closed reporting section — entries 2251–2300 — 2026-11-21

- Both 25-file checkpoints **2251–2275** and **2276–2300** are complete, committed across all six audit documents, and read-back verified.
- Ledger contains **300 records (2001–2300)** with exactly 15 fields each. Index contains **2,300 sequential unique paths**, no duplicates and matching Decisions.
- Entries 2276–2300 cover damage/effect/entity/collision/fluid/movement/inventory/decoration/feature fixtures; comparator, mock-world and versioned-data caveats are recorded in Ledger/QA.
- Tests/examples inspected only; none run. No application source change or runtime/build validation.
- **STOP at 2300.** Authorization exhausted; entry 2301–2350 requires fresh explicit user authorization.


## 28. Decision taxonomy correction and authorization — 2026-11-21

- Decisions 2276–2300 are normalized to the approved primary+qualifier pair adaptable; reference. Reference alone is not a primary Decision; fixture-only/no-shipment limits remain in supporting text.
- decoration_region_fixture.rs has 303 canonical lines, re-read in ranges 1–200 and 201–303.
- User authorization covers 2301–2350, including both 25-file checkpoints. Continue without pausing at 2325; STOP at 2350 and request permission before 2351.


## 29. Ledger partition correction — 2026-11-21

- The prior `AUDIT_LEDGER_2001_2200.md` physically contained entries 2001–2300. Entries 2201–2300 belong to the separate 200-file stage 2201–2400 and have been moved, unchanged, into `AUDIT_LEDGER_2201_2400.md`.
- Current partition: `AUDIT_LEDGER_2001_2200.md` = entries 2001–2200 (200 records, closed); `AUDIT_LEDGER_2201_2400.md` = entries 2201–2300 (100 records, active).
- The 300 entry numbers, source paths, Decisions, SHA/size/coverage evidence and Index rows are unchanged. All 300 records pass the 15-field and Ledger/Index alignment checks.
- Earlier QA/Continuity/Protocol statements that the old Ledger held 225, 275 or 300 records are historical snapshots of the pre-correction layout, not the current file partition.
- Current authorization remains 2301–2350; do not pause at 2325 and STOP at 2350.

## 30. One-time expanded authorization 2301–2400 and checkpoint 2301–2325 — 2026-11-21

- The user extended the current exceptional window to **2301–2400 (100 files total, one time only)**. This does not permanently change the 200/50/25 cadence.
- Checkpoint **2301–2325 is COMPLETE (25/25)**. Checkpoints **2326–2350**, **2351–2375** and **2376–2400** remain covered by the same grant. Do not pause for permission at 2325, 2350 or 2375. **STOP at 2400** and request explicit permission before 2401.
- Active Ledger `AUDIT_LEDGER_2201_2400.md` contains **125 records (2201–2325)** with exactly 15 required fields; Index contains **2,325 sequential unique paths** with matching Decisions.
- All 25 files were read through EOF; 2313 (410 lines) and 2316 (436 lines) were read in contiguous chunks no larger than 400 lines. Source SHAs and sizes match the complete pinned tree.
- These files are test/fixture transports for fluid, food, heightmaps, interaction shapes, inventory and item use; they are not shipped runtime implementations. Tests/examples were inspected, not executed.
- No application source changed; no build, benchmark, APK, device, renderer, Minecraft, Forge, multiplayer or Aternos validation was performed.


## 32. Checkpoint 2326–2350 — 2026-11-21

- Entries **2326–2350 (25/25)** inspected individually through EOF in deterministic lexical path order after the full Index; source paths, SHAs and byte sizes match the complete pinned tree.
- Active Ledger `AUDIT_LEDGER_2201_2400.md` has **150 records (2201–2350)** with exactly 15 required fields; Index has **2,350 sequential unique paths** and matching Decisions.
- Long files: `live_attribute_updates_fixture.rs` 1–400/401–598; `material_condition_fixture.rs` 1–400/401–735; `material_stage_fixture.rs` 1–400/401–800/801–828.
- These are synthetic fixtures. Important limitations include invalid equal-identity JavaKey comparison, empty-sequence panic paths, fixed mock registries/worlds, trusted indexes/casts and private material-stage resource snapshots. No example or test was executed.
- Reporting section **2301–2350 is CLOSED (50/50)**. The user's one-time authorization still covers **2351–2400**. Continue 2351–2375 and 2376–2400 without another permission request at 2375; **STOP at 2400**.
- No application source changed; no build, benchmark, APK, device, renderer, Minecraft, Forge, server or Aternos validation.


## 33. Checkpoint 2351–2375 — 2026-11-21

- Entries **2351–2375 (25/25)** were inspected individually through EOF in deterministic lexical order after excluding the full Index. Exact path, source SHA and size match the pinned tree.
- Active Ledger `AUDIT_LEDGER_2201_2400.md` contains **175 records (2201–2375)** with exactly 15 required fields; Index contains **2,375 sequential unique paths** and matching Decisions.
- Long files: `registered_player_combat_fixture.rs` 1–400/401–609; `registry_collision_scan_fixture.rs` 1–400/401–591; `server_player_combat_fixture.rs` 1–400/401–629; `simple_block_fixture.rs` 1–400/401–773.
- Important limitations: mock player/server/network adapters; fixed arrays and trusted indexes; unavailable heightmap paths in replace/scattered-ore mocks; PlayerFood unwraps stored-path callback errors; PlayerTravel hardcodes a fluid query; ServerPlayerDamage leaves armor modifier unchanged; SimpleBlock uses a hard-coded world-height context/private registry bundle.
- The one-time authorization still covers **2376–2400**. Do not ask for permission before this final checkpoint; **STOP at 2400**.
- No tests/examples executed, no application source changed and no runtime/build/device/Minecraft/Forge/Aternos validation.
