# RustChunk Audit Continuity

Last reconciled: 2026-11-19
Repository: `imamirhosein/RustChunk`
Working branch: `starling-1.20.1-port-audit`

## Authoritative resume point

- **Completed and committed entries:** 1–1200.
- **Most recently closed 200-file stage:** 1001–1200 (200/200).
- **Current state:** no active source-audit half-stage; the next stage has not started.
- **Next stage, when explicitly authorized:** 1201–1400.
- **First half:** 1201–1300, consisting of five checkpoints: 1201–1220, 1221–1240, 1241–1260, 1261–1280 and 1281–1300. After 1300 is committed and verified, stop and report 100/200. Do not begin 1301 until the user gives a new explicit instruction.
- **Second half after that instruction:** 1301–1400, consisting of checkpoints 1301–1320, 1321–1340, 1341–1360, 1361–1380 and 1381–1400.
- **Next source path to verify:** `crates/horizon-client/src/game/boat_authority_fixture.rs`.
- **Range Ledger for the completed stage:** `AUDIT_LEDGER_1001_1200.md`.
- **Navigation/decision index:** `AUDIT_INDEX.md`.
- **Checkpoint and QA history:** `AUDIT_QA_LOG.md`.
- **Rules:** `AUDIT_PROTOCOL.md`.

The full Index has 1200 sequential rows and 1200 unique source paths. The 1001–1200 Ledger has exactly 200 entries, each containing all 15 required fields. Do not repeat or renumber any existing entry.

## Pinned source

- Upstream repository: `starling-client/starling`.
- Pinned upstream commit: `5432370d573642041585c69b40fb045b27133488`.
- Always use this pinned commit for source inspection and source SHA/byte-size comparisons. Do not silently switch to the moving default branch.

## Mandatory work cadence

1. A 200-file stage is the accounting and final-closure unit.
2. A stage is processed as two 100-file halves; each half contains exactly five 20-file checkpoints.
3. A new stage-start instruction authorizes only its first 100 files. Finish the five checkpoints (without asking for approval between them), commit and verify each one, then stop after the 100th file and report the midpoint.
4. Wait for a separate explicit user instruction before beginning the second 100 files. Once authorized, complete its five checkpoints without approval pauses between them.
5. Inspect every file completely before moving to the next. For files over 400 logical lines, fetch consecutive, gap-free sections of at most 400 lines through EOF.
6. Record exactly the 15 standard Ledger fields for each file. Verify pinned source SHA, byte size, canonical line count and complete Coverage.
7. After every 20 files, update the applicable Ledger, Index, QA Log and this Continuity file; synchronize Protocol when required. Commit the checkpoint records together in one checkpoint commit.
8. Re-read the committed records from GitHub, verify blob SHAs and content, then proceed to the next 20 files. Report briefly after each verified checkpoint.
9. The 100-file midpoint is a pause/report boundary only: it does not close the 200-file stage, split the Ledger into two ranges, or trigger a backup.
10. Close the full 200-file stage only after all ten checkpoints are committed and globally verified.

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
- Next backup is due only after entries 1001–2000 are fully committed and globally verified. Do not create backups after a 20-file checkpoint, a 100-file half-stage or a 200-file stage.
- GitHub reports branch protection disabled for the backup refs; immutability is enforced by this workflow rule, not by a GitHub protection lock.

## Closed stage 1001–1200

- All ten 20-file checkpoints from 1001–1020 through 1181–1200 are committed; their five-document read-backs were verified.
- The full Index has 1200 sequential entries and 1200 unique source paths; there are zero duplicate paths.
- Historical duplicate entry 1089 is corrected to `crates/horizon-client/src/game/blockctl.rs`; entry 121 is the only `entry.rs` occurrence.
- The source files 1181–1200 were inspected to EOF; source blob SHA and byte size matched the pinned tree.
- No build, test suite, APK, Forge launch, Android device run, Minecraft multiplayer join or server operation was performed. Embedded tests were inspected only.

## Historical duplicate-path corrections

- For entries 1–1000, duplicate slots 631, 632, 633, 754, 762 and 766 were replaced with the six next unique paths: `pick_block_data_fixture.rs`, `place_recipe_fixture.rs`, `player_sync_fixture.rs`, `raid_fixture.rs`, `random_tick_sections_probe.rs` and `recipe_book_fixture.rs`. Keep these paths in the skip set.
- For the 1001–1200 stage, `crates/horizon-client/src/entry.rs` was found at entries 121 and 1089. Entry 1089 was replaced by `crates/horizon-client/src/game/blockctl.rs` (pinned SHA `94b3853a94ceb4d905e477434553f0a84b4730e5`, 7136 bytes, 179 logical lines). No other entry number changed. Keep `blockctl.rs` in the skip set.
- Global Index uniqueness is restored at 1200/1200 entries.

## Protocol amendment — 2026-11-19

The user approved a new 100+100 work cadence while preserving the 200-file stage boundary. The authoritative rule is in `AUDIT_PROTOCOL.md`: complete five 20-file checkpoints, stop at the first 100 files, report 100/200 and wait for a separate user instruction before starting the second 100. The second half is another five uninterrupted 20-file checkpoints. Backup cadence remains 1,000 newly completed entries.
