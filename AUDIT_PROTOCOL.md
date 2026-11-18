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
- A source path may appear only once in the audit. Skip paths already present in entries 1–800; do not give them a second audit number.
- Each file must be inspected completely before moving on. For long files, read consecutive sections of at most 400 lines until the end of that file.
- If a source file cannot be read completely, record the exact limitation and do not mark it as fully reviewed.

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
- Append all entries to `AUDIT_LEDGER_601_800.md` (or the next continuation ledger) and the matching rows to `AUDIT_INDEX.md`.
- Re-read both files and verify numbering, path agreement, missing rows and duplicate rows.
- Only then report the completed range and stop at the batch boundary.

The next batch after 601–800 is **801–1000**.
