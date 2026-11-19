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
- The **25-file checkpoint** is the durable Git commit and verification unit. Complete exactly 25 new files, update all five audit documents together, commit them together, re-fetch them from GitHub and verify the committed contents before inspecting the next 25 files.
- The **50-file boundary** is the user-facing report and authorization boundary. After the two 25-file checkpoints are committed and verified, report the completed section, identify the exact next 50-file range, and **STOP**. Do not inspect the next section until the user explicitly authorizes it.
- Do not ask for permission between the two 25-file checkpoints inside an already authorized 50-file section. Do not cross a 50-file boundary on assumed or implied permission.
- Do not change the 200 / 50 / 25 cadence unless the user explicitly changes it.

## 3. Transition for the active stage 1201–1400

- Entries 1201–1300 are already complete and committed. They were historically recorded using the former 20-file checkpoint cadence. Preserve those entries and historical QA records; do not renumber, split, rewrite or re-audit them merely to make old checkpoint sizes match the new cadence.
- The first 100 entries of the active stage (1201–1300) are complete. For stage accounting, these cover the first two 50-file sections even though historical Git commits within that completed range were made at 20-file intervals.
- The next section is **1301–1350**, checkpoints **1301–1325** and **1326–1350**. It is currently unauthorized. Do not inspect source file 1301 or later until the user explicitly says to begin.
- After 1301–1350 is completed, committed and verified, report 150/200 for the active stage and STOP. Wait for separate explicit authorization before 1351–1400.
- After authorization, complete 1351–1375 and 1376–1400, then report 200/200 only after global validation.
- Starting with stage 1401–1600, apply the new cadence from the beginning: 1401–1450, 1451–1500, 1501–1550, 1551–1600. Each section contains two exact 25-file checkpoints and ends with a stop for authorization.

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

## 7. Five-document checkpoint transaction

At the end of every exact 25-file checkpoint, update these five documents together:
1. The applicable `AUDIT_LEDGER_<first>_<last>.md`.
2. `AUDIT_INDEX.md`.
3. `AUDIT_QA_LOG.md`.
4. `AUDIT_CONTINUITY.md`.
5. `AUDIT_PROTOCOL.md`, whenever current state or a governing rule needs synchronization; keep it consistent with Continuity.

Before committing, verify:
- Exactly 25 new sequential entry numbers were added.
- All 25 paths are unique against the entire previous Index and within the checkpoint.
- All 25 entries contain all 15 required fields exactly once.
- Source SHAs and byte sizes match the pinned upstream tree.
- Coverage ranges are complete, consecutive and end at EOF.
- Ledger and Index paths, numbering and decisions align.
- The Index is globally sequential and contains no repeated paths.
- Active Ledger entry count and Continuity resume state are correct.

Commit the five-document checkpoint as one Git commit. Re-fetch all five documents from GitHub and verify their saved contents/blob SHAs and expected state. Do not call a checkpoint durable until read-back succeeds.

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
- Structural QA checks sequential numbering, global path uniqueness, exact 15-field counts, source SHA/byte-size matches, canonical line counts, EOF coverage, Ledger/Index alignment and five-document consistency.
- Structural/provenance QA is not automatically a fresh exhaustive semantic re-audit. State the actual scope.
- Never claim a Rust build, unit test, benchmark, APK build, Forge launch, Android-device test, Minecraft multiplayer join, LAN host, UPnP or relay validation unless actually executed. Record unrun checks as pending.
- Preserve security findings. Do not treat Starling/Horizon native-mod build and app-library replacement mechanisms as ordinary Forge mod loading or adopt them without a separate security design and review.

## 11. Backup policy

- Create a new backup only after each additional **1,000 newly completed audit entries** have been committed and globally verified.
- The audit currently has entries 1–1350. The next backup is due only after entries **1001–2000** are complete and the entire 1–2000 audit passes global validation.
- Do not create backups at 200-entry stage boundaries, 100-entry points, 50-file reporting boundaries or 25-file checkpoints.
- Never modify an existing backup branch. Existing backup refs are procedurally read-only; GitHub currently reports branch protection disabled, so do not describe them as technically locked.
- Do not recreate or alter a user-deleted archive branch. Do not copy audit ledgers onto `main`.

## 12. Current authoritative resume state (2026-11-19)

- Repository: `imamirhosein/RustChunk`.
- Working branch: `starling-1.20.1-port-audit`.
- Branch head before this duplicate-path correction: `2e5b2ddabd92cebb81a01226f56ec61833c9bac3`.
- Completed entries: **1–1350**.
- Closed stage: **1001–1200** (200/200).
- Active stage: **1201–1400**; completed **1201–1350** (150/200).
- Authorization state: **STOP** at the 1350 boundary. Section **1301–1350** is complete and verified.
- Next section requires explicit authorization: **1351–1400**, checkpoints **1351–1375** and **1376–1400**.
- Active Ledger: `AUDIT_LEDGER_1201_1400.md`; Index: `AUDIT_INDEX.md`; QA: `AUDIT_QA_LOG.md`; Continuity: `AUDIT_CONTINUITY.md`.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Index: 1350 sequential entries and 1350 unique paths. Active Ledger: 150 entries (1201–1350), each with exactly 15 required fields.
- Existing backups: `backup/audit-1000-unique-2026-11-18` at `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`; `backup/audit-1000-final-2026-11-19` at `4ba78eba6ebc2c89bb455a68a3cb9ddfe5f30e23`. Do not modify them.

## 13. Historical corrections preserved

- In entries 1–1000, duplicate slots 631, 632, 633, 754, 762 and 766 were replaced with unique paths: `pick_block_data_fixture.rs`, `place_recipe_fixture.rs`, `player_sync_fixture.rs`, `raid_fixture.rs`, `random_tick_sections_probe.rs` and `recipe_book_fixture.rs`.
- Entry 1089 was corrected to `crates/horizon-client/src/game/blockctl.rs`; entry 121 remains the only `entry.rs` occurrence.
- Preserve these corrections and all prior QA evidence; never repeat those paths.


## Duplicate-path correction record — 2026-11-19

- QA detected that entry 1349 initially duplicated `crates/horizon-client/src/lib.rs` at historical entry 118. Preserve entry 118; the duplicate slot was corrected.
- Final entry 1349 is `crates/horizon-client/src/locator_bar.rs`; final entry 1350 is `crates/horizon-client/src/lod_net.rs`.
- Do not repeat `lib.rs` at entry 1349. The final Index must contain 1350 sequential entry numbers and 1350 unique paths before the 1301–1350 reporting section is considered closed.

## 14. Authorization and resume update — 2026-11-19

- The former STOP state at entry 1350 was superseded by the user's explicit instruction to start the next section after the 1350 boundary.
- Authorized section: 1351–1400 (50 files), split into checkpoints 1351–1375 and 1376–1400.
- Checkpoint 1351–1375 is complete and verified. Continue serially with 1376–1400; no additional approval is required at 1375.
- Stop at 1400. Do not begin 1401–1450 until the user explicitly authorizes that next 50-file section.
- Latest verified checkpoint commit at time of this update: b5c3d05705b67aaf72eb167aba660a4dff038cbb (continuity update; subsequent protocol commit will advance branch head).
- Keep the established cadence: four 50-file reporting sections per 200-entry boundary; each 50-file section has two 25-file checkpoints. Report briefly after each 25-file checkpoint; after the second checkpoint of a 50-file section, stop and ask before the next 50-file section.
- Inspect files serially, not concurrently. Files up to 400 lines may be read whole; larger files must be read in contiguous chunks of at most 400 lines through EOF. Verify pinned-tree SHA, byte size and logical line count. Treat symlinks by their Git blob target text, not by following their target contents as if they were the link.
- Preserve the 15 required Ledger fields, unique source paths in Index, and checkpoint QA/Continuity updates. Never claim tests or builds were run unless actually executed.
- Keep user-facing reports extremely short; omit implementation recaps and dependency explanations unless requested. Do not alter main or backup branches. Backups remain due only at the established 1000-entry cadence.
