# RustChunk Starling Audit Protocol

This protocol governs the Starling source audit on branch `starling-1.20.1-port-audit`.

## Stage, half-stage and checkpoint boundaries

- The accounting and closure boundary remains **exactly 200 new, unique source files** per stage, whenever at least 200 eligible files remain.
- Split each 200-file stage into **two consecutive halves of 100 files**. Each half contains exactly **five checkpoints of 20 files**.
- The first half is the only part authorized by a new stage-start instruction. Complete its five 20-file checkpoints without asking for approval between them. After the 100th file is committed and verified, stop, report **“half-stage complete: 100/200”**, identify the five completed checkpoints and the exact next file/range, and wait for a new explicit user instruction before starting files 101–200 of that stage.
- After that new instruction, complete the second half as five more 20-file checkpoints. Do not stop for approval between those five checkpoints. Close the 200-file stage only after file 200 is committed and globally revalidated.
- A 100-file midpoint is a **pause/report boundary only**. It does not close the 200-file stage, renumber entries, create a new ledger range, or trigger a backup.
- Every 20-file checkpoint remains durable: after its 20 files are fully inspected, append the complete 15-field entries to the applicable range Ledger, append matching Index rows, append a dated checkpoint record to `AUDIT_QA_LOG.md`, and update `AUDIT_CONTINUITY.md`. Keep `AUDIT_PROTOCOL.md` synchronized when a governing rule or durable state changes. Validate all five documents, commit the checkpoint documents together in one checkpoint commit, and re-read the committed files from GitHub. Never leave more than 20 inspected files uncommitted.
- After every checkpoint read-back, report briefly. At the fifth checkpoint of the first 100-file half, report and pause for the user's next instruction. At the tenth checkpoint, report that the full 200-file stage is complete.

## File selection and inspection

- Use the pinned Starling commit recorded by the audit, not the moving default branch.
- Continue in deterministic source-tree path order from the last audited path.
- Before selecting new files, build the skip set from every path already present in the current Index (currently entries 1–1200). A source path may appear only once in the audit; never assign a second audit number to an existing path.
- Corrective closure for entries 1–1000: six duplicate slots (631, 632, 633, 754, 762 and 766) were replaced with the six next unique paths after the former entry-1000 endpoint: `pick_block_data_fixture.rs`, `place_recipe_fixture.rs`, `player_sync_fixture.rs`, `raid_fixture.rs`, `random_tick_sections_probe.rs` and `recipe_book_fixture.rs`. Those six files are already audited and must be skipped by future batches.
- Corrective closure for the 1001–1200 stage: duplicate entry 1089 was replaced, without renumbering other entries, by `crates/horizon-client/src/game/blockctl.rs` (pinned SHA `94b3853a94ceb4d905e477434553f0a84b4730e5`; 179 logical lines). This replacement is already audited and must be skipped.
- The next source path after entry 1200 is `crates/horizon-client/src/game/boat_authority_fixture.rs`; verify it against the pinned tree and full Index before assigning entry 1201.
- Inspect each file completely before moving on. For files longer than 400 logical lines, read consecutive, gap-free sections of at most 400 lines until EOF.
- If a source file cannot be read completely, record the exact limitation and do not mark it fully reviewed.

## Ledger file layout

- Entries 1–400: `AUDIT_LEDGER_1_400.md`.
- Entries 401–600: `AUDIT_LEDGER_401_600.md`.
- Entries 601–800: `AUDIT_LEDGER_601_800.md`.
- Entries 801–1000: `AUDIT_LEDGER_801_1000.md`.
- Entries 1001–1200: `AUDIT_LEDGER_1001_1200.md`.
- Keep `AUDIT_INDEX.md` as the navigation/decision summary; keep its ledger links and entry ranges synchronized with every continuation file.
- `AUDIT_CONTINUITY.md` is the durable cross-chat resume record. It must always match the latest verified checkpoint, current half-stage pause state, and branch state.
- Name each future continuation `AUDIT_LEDGER_<first>_<last>.md` using its exact inclusive entry range. The 100-file midpoint does not split a 200-entry ledger into two new ledgers.

## Ledger and evidence

- Every entry must contain all 15 standard fields:
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
- Record source SHA and size from the pinned source tree.
- Distinguish adaptable architecture from Starling/Horizon-only features and from Minecraft 1.20.1 behavior. Never treat 26.3 registries, IDs, packets or assets as 1.20.1 data.
- Reading a test is not running it. Never claim a test, build, APK, Forge client, server connection or device validation succeeded unless it was actually executed.

## Checkpoint and half-stage completion checks

Before reporting a 20-file checkpoint complete:
- Verify exactly 20 new sequential entry numbers.
- Verify no duplicate source paths against every previous entry and the current checkpoint.
- Verify all 15 fields are present exactly once for every new entry.
- Verify source SHA, byte size and complete Coverage against the pinned upstream tree.
- Verify Ledger and Index path/decision alignment.
- Update the applicable Ledger, Index, QA Log and Continuity; synchronize Protocol if the governing state changed.
- Commit the checkpoint documents together in one commit, re-read the committed documents from GitHub, and verify the saved blob SHAs/content before proceeding to the next 20 files.

At the end of the first 100-file half:
- Verify exactly 100 new sequential entries across five verified 20-file checkpoints.
- Verify full path uniqueness against the entire Index and exactly 15 fields per entry.
- Confirm the 200-file stage remains open at 100/200.
- Record the exact next entry number, next source path and second-half checkpoint ranges in Continuity.
- Report the midpoint and stop. Do not start the second half without a new explicit user instruction.

At the end of the second 100-file half:
- Verify exactly 200 new sequential entries across ten verified 20-file checkpoints.
- Run the full-stage uniqueness, field-count, source-SHA/size, Coverage and Ledger/Index alignment checks.
- Report the 200-file stage complete only after the tenth checkpoint has been committed and globally revalidated.

## Current continuation state (2026-11-19)

- Entries **1001–1200** are committed. The 1001–1200 stage is complete with 1200 sequential Index entries and 1200 unique source paths.
- Historical entry 1089 remains corrected to `crates/horizon-client/src/game/blockctl.rs`; entry 121 is the only `entry.rs` occurrence.
- Do not start entry 1201 until the user authorizes the next stage. Under the new cadence, the next stage is 1201–1400: first half 1201–1300 (five 20-file checkpoints), then stop and await a separate instruction before 1301–1400 (five more checkpoints).
- Both existing backup refs remain untouched. The next backup is due only after entries 1001–2000 are fully committed and globally verified. Read `AUDIT_CONTINUITY.md` before each new chat.

## QA review and canonical file-size accounting

- QA reviews of existing entries are separate from new source-audit stages. QA may be reported in 100-entry ranges; it must not renumber entries, reselect paths, or create duplicate source paths.
- For each QA range, verify sequential numbering, path uniqueness against the full Index, exactly one occurrence of each of the 15 required fields, Ledger/Index path and decision alignment, and source SHA equality with the pinned upstream tree.
- Canonical logical line count is the number of LF-terminated lines, plus one only when the source blob does not end in LF. Do not count the synthetic empty split element after a final newline as a source line.
- Source byte size must match the pinned Git tree/blob metadata. Optional character counts must use one consistent UTF-8 decoding convention.
- Coverage ranges must not extend beyond the canonical logical line count. Multi-range reads must be consecutive, gap-free, and end exactly at the canonical final line.
- Primary Decision labels are **essential**, **adaptable**, **rewrite required**, **integrate**, **unnecessary**, and **undetermined**. Approved secondary qualifiers are **reference**, **version-specific**, and **optional/future**; they supplement a primary decision and do not replace it.
- The Decision field and Index decision column must use one or more primary decision labels. Do not use placeholders such as "See ledger decision" or sentiment wording such as "Highly relevant/adaptable" as decision labels.
- Every QA correction and every 20-file audit checkpoint must be recorded in `AUDIT_QA_LOG.md`, including the pinned baseline commit, inclusive entry numbers, source-count, validation results and resulting commit SHA.
- **Backup cadence:** create a new backup branch only after each additional **1,000 newly completed audit entries**, not after 200 entries, 100-entry half-stages or 20-file checkpoints. The next backup is due only after entries 1001–2000 are fully committed and globally verified. Never modify an existing backup branch.
