# RustChunk Audit Continuity

Last reconciled: 2026-11-19
Repository: `imamirhosein/RustChunk`
Working branch: `starling-1.20.1-port-audit`

## Authoritative resume point

- **Completed and committed entries:** 1–1220.
- **Most recently closed 200-file stage:** 1001–1200 (200/200).
- **Active 200-file stage:** 1201–1400.
- **Active half-stage:** first half, 1201–1300.
- **First-half progress:** 20/100 (20%); checkpoint 1201–1220 is committed and read-back verified.
- **Completed checkpoints in the active stage:** 1201–1220.
- **Next checkpoint to start:** 1221–1240.
- **Checkpoint schedule for first half:** 1201–1220, 1221–1240, 1241–1260, 1261–1280, 1281–1300. After 1300 is committed and globally verified, stop and report 100/200. Do not begin 1301 until a separate explicit user instruction.
- **Second half after its separate authorization:** 1301–1320, 1321–1340, 1341–1360, 1361–1380 and 1381–1400.
- **Next source path to inspect:** `crates/horizon-client/src/game/ghasts.rs`.
- **Range Ledger for active stage:** `AUDIT_LEDGER_1201_1400.md` (one file for both 100-file halves).
- **Completed prior Ledger:** `AUDIT_LEDGER_1001_1200.md`.
- **Navigation/decision index:** `AUDIT_INDEX.md`.
- **Checkpoint and QA history:** `AUDIT_QA_LOG.md`.
- **Rules:** `AUDIT_PROTOCOL.md`.

The full Index has 1220 sequential entries and 1220 unique source paths. The new Ledger has exactly 20 entries (1201–1220), each with all 15 required fields. Do not repeat or renumber any existing entry.

## Pinned source

- Upstream repository: `starling-client/starling`.
- Pinned upstream commit: `5432370d573642041585c69b40fb045b27133488`.
- Recursive source tree was re-fetched; tree was not truncated and its SHA matched the pinned commit.
- Always use this pinned commit for source inspection and source SHA/byte-size comparisons. Do not silently switch to the moving default branch.

## Mandatory work cadence

1. A 200-file stage is the accounting and final-closure unit.
2. Each stage is processed as two consecutive 100-file halves; each half contains exactly five 20-file checkpoints.
3. A new stage-start instruction authorizes only its first 100 files. Complete five checkpoints, commit/read-back verify each, then stop after the 100th file and report 100/200.
4. Wait for a separate explicit user instruction before beginning the second half. Once authorized, complete its five checkpoints without approval pauses between them.
5. Inspect every file completely before moving to the next. For files over 400 logical lines, fetch consecutive, gap-free sections of at most 400 lines through EOF.
6. Record exactly 15 standard Ledger fields for each file; verify pinned source SHA, byte size, canonical line count and complete Coverage.
7. After every 20 files, update the applicable Ledger, Index, QA Log and this Continuity file; synchronize Protocol when the governing resume state changes. Commit all records together in one checkpoint commit.
8. Re-read all committed documents from GitHub, verify blob SHAs and contents, then proceed to the next 20 files. Report briefly after each verified checkpoint.
9. The 100-file midpoint is pause/report only; it does not close the stage, split the Ledger range or trigger a backup.
10. Close the 200-file stage only after all ten checkpoints are committed and globally verified.

## Decision vocabulary

Primary labels: `essential`, `adaptable`, `rewrite required`, `integrate`, `unnecessary`, `undetermined`.

Optional qualifiers: `reference`, `version-specific`, `optional/future`.

Never confuse Starling/Horizon snapshot-specific features or data with Minecraft Java 1.20.1 behavior. Reading a test is not running it. Never claim builds, tests, APKs, Android-device validation, Forge launches or server joins unless they were actually executed.

## Backup state

- Original backup ref: `backup/audit-1000-unique-2026-11-18`.
- Original backup commit: `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`.
- Second backup ref: `backup/audit-1000-final-2026-11-19`.
- Second backup commit: `4ba78eba6ebc2c89bb455a68a3cb9ddfe5f30e23`.
- Both existing backups are read-only by working rule: never update, move or delete them.
- Next backup remains due only after entries 1001–2000 are fully committed and globally verified. Do not create backups after 20-file checkpoints, 100-file halves or 200-file stages.
- GitHub reports branch protection disabled for the backup refs; immutability is enforced by this workflow rule, not by a GitHub protection lock.

## Closed stage 1001–1200

- All ten 20-file checkpoints from 1001–1020 through 1181–1200 are committed and their five-document read-backs were verified.
- The full Index has 1200 sequential entries and 1200 unique paths; there are zero duplicate paths.
- Historical duplicate entry 1089 is corrected to `crates/horizon-client/src/game/blockctl.rs`; entry 121 remains the only `entry.rs` occurrence.
- No build, test suite, APK, Forge launch, Android device run, multiplayer join or server operation was performed in that stage.

## Active checkpoint 1201–1220

- Source files 1201–1220 were inspected completely to EOF, in deterministic path order after entry 1200. Each source SHA and byte size matches the pinned upstream Git tree.
- The first source after entry 1200 is `crates/horizon-client/src/game/events.rs`, not the previously written `boat_authority_fixture.rs`. That stale next-path note has been corrected; `events.rs` is now entry 1201.
- Entry paths 1201–1220 are recorded in `AUDIT_LEDGER_1201_1400.md` and in the Index. All 20 entries contain the 15 standard fields.
- The Index contains 1220 sequential entries, 1220 unique paths and zero duplicate paths.
- Files longer than 400 lines were read using contiguous, gap-free ranges no larger than 400 lines: explosion.rs (1416), fireworks.rs (736), fishing.rs (1570), frames.rs (661), functions.rs (1079) and game_tests.rs (1650).
- Embedded tests in fans.rs, fireworks.rs, fishing.rs, fov.rs, game_mode.rs and game_tests.rs were inspected only, not executed.
- No source tests, builds, APKs, Forge launches, Android device runs, multiplayer joins, LAN hosting or UPnP/relay operations were performed.
- Decisions retain core vanilla mechanics (explosions, falling damage, fangs, fireworks, fishing, frames, datapacks and game modes) for a 1.20.1 rewrite/adaptation; Horizon agent/Android/friends features and Create extras stay outside baseline; GameTest remains test-only.
- Next checkpoint is 1221–1240 starting at `crates/horizon-client/src/game/ghasts.rs`. Continue through 1300 without asking for approval between 20-file checkpoints, then stop and await the separate second-half instruction.

## Historical duplicate-path corrections

- For entries 1–1000, duplicate slots 631, 632, 633, 754, 762 and 766 were replaced with the six next unique paths: `pick_block_data_fixture.rs`, `place_recipe_fixture.rs`, `player_sync_fixture.rs`, `raid_fixture.rs`, `random_tick_sections_probe.rs` and `recipe_book_fixture.rs`. Keep these paths in the skip set.
- For entries 1001–1200, duplicate entry 1089 was replaced with `crates/horizon-client/src/game/blockctl.rs` (pinned SHA `94b3853a94ceb4d905e477434553f0a84b4730e5`, 7136 bytes, 179 logical lines); entry 121 remains the sole `entry.rs` record. Keep `blockctl.rs` in the skip set.
- All paths in the full Index through 1220 are unique.

## Protocol amendment — 2026-11-19

The user approved a 100+100 work cadence with 200 files remaining the accounting boundary. For stage 1201–1400, only 1201–1300 is currently authorized. Stop at 1300 after five committed and verified 20-file checkpoints, then wait for a new explicit instruction before starting 1301–1400. The single Ledger file for the entire stage is `AUDIT_LEDGER_1201_1400.md`. Backup cadence remains every 1000 newly completed entries.
