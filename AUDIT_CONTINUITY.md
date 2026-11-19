# RustChunk Audit Continuity

Last reconciled: 2026-11-19
Repository: `imamirhosein/RustChunk`
Working branch: `starling-1.20.1-port-audit`

## Authoritative resume point

- **Completed and committed entries:** 1–1120.
- **Current 200-file stage:** 1001–1200.
- **Completed checkpoints in this stage:** 1001–1020, 1021–1040, 1041–1060, 1061–1080, 1081–1100, 1101–1120.
- **Current stage progress:** 120 of 200 entries (60%).
- **Next checkpoint to start:** **1121–1140**, after corrective closure for entry 1089.
- **Next checkpoint number:** 7 of 10 (held pending duplicate closure).
- **Range Ledger:** `AUDIT_LEDGER_1001_1200.md`.
- **Navigation/decision index:** `AUDIT_INDEX.md`.
- **Checkpoint and QA history:** `AUDIT_QA_LOG.md`.
- **Rules:** `AUDIT_PROTOCOL.md`.

The Ledger currently contains exactly 120 entries for 1001–1120. The Index also contains rows through 1120. Do not repeat or renumber any of these entries.

## Pinned source

- Upstream repository: `starling-client/starling`.
- Pinned upstream commit: `5432370d573642041585c69b40fb045b27133488`.
- Always use this pinned commit for source inspection and source SHA/byte-size comparisons. Do not silently switch to the moving default branch.

## Checkpoint workflow (mandatory)

1. The authorized stage is a continuous 200-file range.
2. Divide it into ten checkpoints of exactly 20 new, unique source files.
3. Inspect every file completely before moving to the next file. For files over 400 logical lines, fetch consecutive, gap-free sections of at most 400 lines until EOF.
4. Record exactly the 15 standard Ledger fields for each file.
5. After every 20 files, update the range Ledger, Index, QA Log, and this Continuity file.
6. Validate sequential numbers, unique source paths against all previous entries, all 15 fields exactly once, pinned source SHA and byte size, exact Coverage, and Ledger/Index agreement.
7. Commit all four updated records together in one checkpoint commit.
8. Re-read the committed files from GitHub and confirm the commit before starting the next checkpoint.
9. Continue through the already-authorized 200-file stage without asking for another approval between 20-file checkpoints. Report briefly after each verified checkpoint.

## Decision vocabulary

Primary labels: `essential`, `adaptable`, `rewrite required`, `integrate`, `unnecessary`, `undetermined`.

Optional qualifiers: `reference`, `version-specific`, `optional/future`.

Never confuse Starling/Horizon snapshot-specific features or data with Minecraft Java 1.20.1 behavior. Reading a test is not running it. Never claim builds, tests, APKs, Android-device validation, Forge launches or server joins unless they were actually executed.

## Backup state

- Original backup ref: `backup/audit-1000-unique-2026-11-18`.
- Original backup commit: `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`.
- Second backup ref: `backup/audit-1000-final-2026-11-19`.
- Second backup commit: `4ba78eba6ebc2c89bb455a68a3cb9ddfe5f30e23`.
- Both existing backups are read-only by working rule: never update, move, or delete them.
- Next backup is due only after entries 1001–2000 are fully committed and globally verified. Do not create backups after a 20-file checkpoint or after the 200-file stage.

GitHub currently reports branch protection disabled for the backup refs, so immutability is enforced by this workflow rule, not by a GitHub protection lock.

## Recovery note

The previous long chat branch was interrupted while reviewing the large Horizon/Android source area. The activity display had shown animation, arrows, attribute updates, auto-jump, touch, beehive and benchmark reviews; later GitHub checkpoints supersede that transient activity view. The authoritative completion point is the committed Ledger/Index/QA state at entry 1100, not an incomplete activity message or an uncommitted file-read position.

## Current task

- Stage 1001–1200: 120/200 numbered entries complete (60%); checkpoint 1101–1120 is committed and read-back verified.
- The historical duplicate at entry 1089 is corrected: it now records `crates/horizon-client/src/game/blockctl.rs`; entry 121 remains the only occurrence of `crates/horizon-client/src/entry.rs`.
- The full Index now contains 1120 rows and 1120 unique source paths. Keep `blockctl.rs` in the skip set; it is a corrective replacement in slot 1089, not a future new entry.
- Resume checkpoint **1121–1140** at the next eligible pinned-tree path: `crates/horizon-client/src/game/boat_authority_fixture.rs`. Rebuild the skip set from the full corrected Index before selecting subsequent files.
- Both existing backups remain untouched. The next backup is due only after entries 1001–2000 are fully committed and globally verified.

## Historical duplicate-path correction — 2026-11-19

- Duplicate found: `crates/horizon-client/src/entry.rs` at entries 121 and 1089.
- Corrective replacement: entry 1089 is now `crates/horizon-client/src/game/blockctl.rs` (pinned SHA `94b3853a94ceb4d905e477434553f0a84b4730e5`, 7136 bytes, 179 logical lines), read fully to EOF.
- No other entry numbers changed. Global Index path uniqueness is restored: 1120 entries, 1120 unique paths, zero duplicates.
- The replacement path is included in the skip set. Next scan starts at `crates/horizon-client/src/game/boat_authority_fixture.rs`.
- Backups were not modified.

