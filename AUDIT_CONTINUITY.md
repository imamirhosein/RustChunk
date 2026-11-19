# RustChunk Audit Continuity

Last reconciled: 2026-11-19
Repository: imamirhosein/RustChunk
Working branch: starling-1.20.1-port-audit

## Authoritative resume point

- Completed and committed entries: 1–1260.
- Most recently closed 200-file stage: 1001–1200 (200/200).
- Active 200-file stage: 1201–1400.
- Active half-stage: first half, 1201–1300.
- First-half progress: 60/100 (60%); checkpoints 1201–1220, 1221–1240 and 1241–1260 are committed and read-back verified.
- Completed active checkpoints: 1201–1220, 1221–1240, 1241–1260.
- Next checkpoint: 1261–1280.
- First-half schedule: 1201–1220, 1221–1240, 1241–1260, 1261–1280, 1281–1300. After 1300 is committed and globally verified, stop and report 100/200. Do not start 1301 until a separate explicit user instruction.
- Second half after separate authorization: 1301–1320, 1321–1340, 1341–1360, 1361–1380, 1381–1400.
- Next source path: `crates/horizon-client/src/game/launch_api.rs`.
- Active Ledger: `AUDIT_LEDGER_1201_1400.md` (one file for both 100-file halves).
- Previous Ledger: `AUDIT_LEDGER_1001_1200.md`.
- Index: `AUDIT_INDEX.md`; QA history: `AUDIT_QA_LOG.md`; rules: `AUDIT_PROTOCOL.md`.

The full Index contains 1260 sequential rows and 1260 unique source paths. The active Ledger contains 60 entries (1201–1260), each with exactly 15 fields. Never repeat or renumber paths.

## Pinned source

- Upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Recursive tree SHA matched the pin and was not truncated. Always use this pinned revision for source inspection and SHA/byte-size checks.

## Mandatory cadence

1. A 200-file stage remains the accounting and closure boundary.
2. Each stage has two 100-file halves, each five exact 20-file checkpoints.
3. Stage-start authorizes only the first 100; complete five checkpoints without approval pauses, then stop/report 100/200.
4. Wait for a separate explicit instruction before the second half; after authorization finish its five checkpoints without approval pauses.
5. Read each file completely; files over 400 logical lines need contiguous, gap-free ranges <=400.
6. Record all 15 fields; verify SHA, byte size, canonical line count and Coverage.
7. After every 20 files, update Ledger, Index, QA, Continuity and Protocol when current state/skip set changes; commit all together and read back all five.
8. Verify global path uniqueness and Ledger/Index alignment.
9. The 100-file midpoint is report/pause only; it does not split the Ledger or trigger backup.
10. Close the 200-file stage only after ten committed and verified checkpoints.

## Decision vocabulary

Primary: essential, adaptable, rewrite required, integrate, unnecessary, undetermined.
Qualifiers: reference, version-specific, optional/future.
Reading tests is not running them. Never claim build, APK, Forge launch, Android-device validation or server join unless actually executed.

## Backup state

- `backup/audit-1000-unique-2026-11-18` -> `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`
- `backup/audit-1000-final-2026-11-19` -> `4ba78eba6ebc2c89bb455a68a3cb9ddfe5f30e23`
- Both backup refs are read-only by workflow rule and must never be updated, moved or deleted.
- Next backup only after entries 1001–2000 are fully committed and globally verified. No backup after a 20, 100 or 200 file boundary.
- GitHub branch protection is disabled for these refs; immutability is procedural.

## Closed stage 1001–1200

- Ten 20-file checkpoints through 1181–1200 committed and verified.
- Index globally validated with 1200 sequential, unique paths.
- Historical duplicate 1089 corrected to `blockctl.rs`; entry 121 is the sole `entry.rs`.
- No build, APK, Forge, device or server test was run.

## Active stage 1201–1260

- Checkpoints 1201–1220, 1221–1240 and 1241–1260 are committed and five-document read-backs were verified.
- Index has 1260 sequential rows, 1260 unique paths and zero duplicates.
- First 60 of the authorized first 100 are complete; no entries 1261+ have been started.
- Long files: kinetic_machines.rs 1024; kinetics.rs 1442; lan_actions.rs 5365; lan_entities.rs 853; lan_mobs.rs 545; lan_particles.rs 884; lan_screen.rs 1879; lan_stations.rs 879; lan_trading.rs 465; lan_vehicles.rs 564; lan.rs 2982. All were read in contiguous ranges through EOF.
- lan_far.rs canonical line count is 79 (not 98); pinned SHA and byte size were verified.
- Next path: `crates/horizon-client/src/game/launch_api.rs`, entry 1261.
- One Ledger file `AUDIT_LEDGER_1201_1400.md` contains both halves.
- After two more verified checkpoints, pause at 1300 and await a separate second-half instruction.

## Historical duplicate corrections

- Entries 1–1000 duplicate slots 631, 632, 633, 754, 762 and 766 were replaced with six unique paths: `pick_block_data_fixture.rs`, `place_recipe_fixture.rs`, `player_sync_fixture.rs`, `raid_fixture.rs`, `random_tick_sections_probe.rs` and `recipe_book_fixture.rs`.
- Entry 1089 corrected from duplicate `entry.rs` to `crates/horizon-client/src/game/blockctl.rs` (SHA `94b3853a94ceb4d905e477434553f0a84b4730e5`; 7136 bytes; 179 lines); entry 121 remains sole `entry.rs`.
