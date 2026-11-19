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

## 9. QA scope and claim discipline

- QA of existing entries is separate from new source-audit checkpoints. QA must not silently create, renumber or duplicate source entries.
- Structural QA checks sequential numbering, global path uniqueness, exact 15-field counts, source SHA/byte-size matches, canonical line counts, EOF coverage, Ledger/Index alignment and five-document consistency.
- Structural/provenance QA is not automatically a fresh exhaustive semantic re-audit. State the actual scope.
- Never claim a Rust build, unit test, benchmark, APK build, Forge launch, Android-device test, Minecraft multiplayer join, LAN host, UPnP or relay validation unless actually executed. Record unrun checks as pending.
- Preserve security findings. Do not treat Starling/Horizon native-mod build and app-library replacement mechanisms as ordinary Forge mod loading or adopt them without a separate security design and review.

## 10. Backup policy

- Create a new backup only after each additional **1,000 newly completed audit entries** have been committed and globally verified.
- The audit currently has entries 1–1300. The next backup is due only after entries **1001–2000** are complete and the entire 1–2000 audit passes global validation.
- Do not create backups at 200-entry stage boundaries, 100-entry points, 50-file reporting boundaries or 25-file checkpoints.
- Never modify an existing backup branch. Existing backup refs are procedurally read-only; GitHub currently reports branch protection disabled, so do not describe them as technically locked.
- Do not recreate or alter a user-deleted archive branch. Do not copy audit ledgers onto `main`.

## 11. Current authoritative resume state (2026-11-19)

- Repository: `imamirhosein/RustChunk`.
- Working branch: `starling-1.20.1-port-audit`.
- Branch head before this protocol normalization: `369cf95ab040b720a15af96ca514231ad03c3352`.
- Completed entries: **1–1300**.
- Closed stage: **1001–1200** (200/200).
- Active stage: **1201–1400**; completed **1201–1300** (100/200).
- Authorization state: **STOP**. No entry 1301 or later may be inspected until explicit user authorization.
- Next section after authorization: **1301–1350**, checkpoints **1301–1325** and **1326–1350**.
- Active Ledger: `AUDIT_LEDGER_1201_1400.md`; Index: `AUDIT_INDEX.md`; QA: `AUDIT_QA_LOG.md`; Continuity: `AUDIT_CONTINUITY.md`.
- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Index: 1300 sequential entries and 1300 unique paths. Active Ledger: 100 entries (1201–1300), each with exactly 15 required fields.
- Existing backups: `backup/audit-1000-unique-2026-11-18` at `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`; `backup/audit-1000-final-2026-11-19` at `4ba78eba6ebc2c89bb455a68a3cb9ddfe5f30e23`. Do not modify them.

## 12. Historical corrections preserved

- In entries 1–1000, duplicate slots 631, 632, 633, 754, 762 and 766 were replaced with unique paths: `pick_block_data_fixture.rs`, `place_recipe_fixture.rs`, `player_sync_fixture.rs`, `raid_fixture.rs`, `random_tick_sections_probe.rs` and `recipe_book_fixture.rs`.
- Entry 1089 was corrected to `crates/horizon-client/src/game/blockctl.rs`; entry 121 remains the only `entry.rs` occurrence.
- Preserve these corrections and all prior QA evidence; never repeat those paths.
