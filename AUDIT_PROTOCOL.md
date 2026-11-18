# RustChunk Starling Audit Protocol

This protocol governs the Starling source audit on branch `starling-1.20.1-port-audit`.

## Batch size and boundaries

- Each audit stage is one continuous batch of **exactly 200 new, unique source files** whenever at least 200 eligible files remain.
- Do not split a 200-file stage into 20-file or other smaller reporting stages.
- Example boundaries: 601–800, then 801–1000.
- Do not stop for approval or request a new start command in the middle of a batch.
- Report only after the full 200-entry batch is reviewed, recorded and verified.

## File selection and inspection

- Use the pinned Starling commit recorded by the audit, not the moving default branch.
- Continue in deterministic source-tree path order from the last audited path.
- A source path may appear only once in the audit. Before selecting new files, build the skip set from every path already present in the current index (currently entries 1–1000); do not give a path a second audit number.
- Corrective closure for entries 1–1000: six duplicate slots (631, 632, 633, 754, 762 and 766) were replaced with the six next unique paths after the former entry-1000 endpoint: `pick_block_data_fixture.rs`, `place_recipe_fixture.rs`, `player_sync_fixture.rs`, `raid_fixture.rs`, `random_tick_sections_probe.rs` and `recipe_book_fixture.rs`. Those six files are already audited and must be skipped by future batches.
- Each file must be inspected completely before moving on. For long files, read consecutive sections of at most 400 lines until the end of that file.
- If a source file cannot be read completely, record the exact limitation and do not mark it as fully reviewed.

## Ledger file layout

- Entries 1–400: `AUDIT_LEDGER_1_400.md`.
- Entries 401–600: `AUDIT_LEDGER_401_600.md`.
- Entries 601–800: `AUDIT_LEDGER_601_800.md`.
- Entries 801–1000: `AUDIT_LEDGER_801_1000.md`.
- `AUDIT_INDEX.md` is the navigation/decision summary; keep its ledger links and entry ranges synchronized with every continuation file.
- Name every future continuation `AUDIT_LEDGER_<first>_<last>.md` using its exact inclusive entry range; do not use an unqualified ledger filename.

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

## Batch completion checks

Before reporting a batch complete:
- Verify exactly 200 new sequential entry numbers.
- Verify no duplicate source paths against every previous entry.
- Verify all 15 fields are present for every new entry.
- Append all entries to the ledger file matching the audited entry range (use the range-specific naming convention above) and the matching rows to `AUDIT_INDEX.md`.
- Re-read both files and verify numbering, path agreement, missing rows and duplicate rows.
- Only then report the completed range and stop at the batch boundary.

The next batch after 801–1000 is **1001–1200**.


## QA review and canonical file-size accounting

- QA reviews of existing entries are separate from new source-audit stages. QA may be reported in 100-entry ranges; it must not renumber entries, reselect paths, or create duplicate source paths.
- For each QA range, verify sequential numbering, path uniqueness against the full index, exactly one occurrence of each of the 15 required fields, ledger/index path and decision alignment, and source SHA equality with the pinned upstream tree.
- Canonical logical line count is the number of LF-terminated lines, plus one only when the source blob does not end in LF. Do not count the synthetic empty split element after a final newline as a source line.
- Source byte size must match the pinned Git tree/blob metadata. Optional character counts must use one consistent UTF-8 decoding convention.
- Coverage ranges must not extend beyond the canonical logical line count. Multi-range reads must be consecutive, gap-free, and end exactly at the canonical final line.
- Primary Decision labels are **essential**, **adaptable**, **rewrite required**, **integrate**, **unnecessary**, and **undetermined**. Approved secondary qualifiers are **reference**, **version-specific**, and **optional/future**; they supplement a primary decision and do not replace it.
- The Decision field and Index decision column must use one or more primary decision labels. Do not use placeholders such as "See ledger decision" or sentiment wording such as "Highly relevant/adaptable" as decision labels.
- Every QA correction must be recorded in `AUDIT_QA_LOG.md`, including the baseline commit, affected entry numbers, exact correction and verification result.
- QA may correct active-branch records, but must never modify the frozen archive branch.
