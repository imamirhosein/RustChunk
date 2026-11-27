# RustChunk — Starling audit protocol

## Purpose
Audit Starling to determine what can be reused, adapted, replaced or newly implemented so the native engine can run **Minecraft Java 1.20.1 Vanilla**. The audit is preparatory engineering work, not the project deliverable. Do not drift into Forge, mods, multiplayer or launcher cosmetics before the Vanilla single-player milestone.

## Fixed sources
- RustChunk repo: `imamirhosein/RustChunk`; working branch: `starling-1.20.1-port-audit`.
- Historical audit baseline: Starling commit `5432370d573642041585c69b40fb045b27133488` (the earlier 26.3 snapshot).
- Re-audit source: user fork `imamirhosein/starling`, fixed commit `ff303502a4f7982551a0e0d3622d9268efe9e829`. Do not use a moving branch/ref for file review. Verify this commit and complete source tree before each audit session.
- The four `archive/unverified-v0.8-audit-*` branches are historical read-only backups. Do not change them. Keep `main` untouched.

## Re-audit method — reuse prior work
Existing entries 1–3700 are the historical baseline; their numbers and order are stable and must not be renumbered.
For each existing entry, compare its prior source path and blob SHA with the fixed v0.9.0 tree:
1. **Same path + same SHA:** no re-reading/re-analysis. Carry forward the previous review as unchanged; record status only when the checkpoint is updated.
2. **Same path + different SHA:** inspect the complete new file (EOF; files >400 lines in consecutive chunks of at most 400), compare against prior findings and update that existing entry.
3. **Path absent in new tree:** mark the historical entry as removed from the target snapshot. Do not erase its prior review.
4. **Possible rename/move:** SHA/path matches are candidates only. Do not infer a rename without checking. Keep old path history; queue the new path as an addition unless its relationship is verified.
5. **New paths:** inventory additions separately from old entries. Do not silently renumber or insert them among historical entries. They are not part of the 1–3700 recheck; report/queue them for explicit scope after this pass.
Recheck the prior 3700 entries in their existing order, starting at entry 1. The old review is a comparison baseline, not evidence that changed files were reviewed in the new snapshot.

## Inspection quality
- Confirm the recursive source tree is complete and not truncated; verify every selected path, SHA and byte size against the pinned tree.
- Read changed files fully. No partial reading unless chunked consecutively with no gaps and EOF confirmed.
- Inspect visual assets with rendering when available; do not claim visual review otherwise. Videos are marked for user review unless actually played.
- Each ledger entry keeps the existing 15-field schema. Distinguish source claims from independently verified evidence.
- Static inspection does not establish build, test, APK, device, Minecraft launch, Forge, multiplayer or server results.

## Work cadence and permission
- Work serially in groups of 50 existing audit entries; use the 25-file subranges as internal progress checkpoints if needed.
- At each 50-entry boundary, report briefly: unchanged count, changed/re-read count, removed count, material findings, and next range. Stop and wait for explicit authorization before starting the next 50.
- Do not create new audit documents, manifests, status trackers or backup branches unless genuinely required and approved. Update the existing Ledger/Index entry in place; keep historical QA log append-only and update continuity only at the report boundary.
- No automatic continuation across a user authorization boundary.

## Existing records
- `AUDIT_INDEX.md`: stable numbering and prior paths/decisions; historical baseline.
- `AUDIT_LEDGER_*.md`: detailed prior source review; preserve chronology and update the existing entry only as needed for re-audit.
- `AUDIT_QA_LOG.md`: historical QA evidence, append-only. Do not rewrite prior QA as if it occurred on the new source.
- `AUDIT_MEDIA_ARCHIVE.md`: actual image/video records. Historical backfill is incomplete; do not claim complete media coverage.
- `AUDIT_CONTINUITY.md`: current state and resume point only.
This protocol is the sole current rulebook. Historical instructions inside old ledger preambles/QA/boundary notes are superseded for the new pass.

## Safety and project boundary
Audit-only changes belong on the audit branch. Do not edit RustChunk implementation, patches, workflows, private Minecraft inputs or submodule contents during this source review. No deletions outside audit-administration documents. Never modify `main` or the backup refs. Before a Git write, verify the expected branch HEAD; after a commit, fetch/read back changed documents and confirm the branch/backup/main refs.
