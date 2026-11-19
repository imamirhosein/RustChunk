# RustChunk Audit Continuity

Last reconciled: 2026-11-19
Repository: imamirhosein/RustChunk
Working branch: starling-1.20.1-port-audit

## Authoritative resume point

- Completed and committed entries: **1–1350**.
- Closed 200-file stage: **1001–1200** (200/200).
- Active 200-file stage: **1201–1400**.
- Completed in the active stage: **1201–1350** (150/200).
- Authorization state: **STOP** at the 1350 boundary. Section 1301–1350 is complete and verified. Do not inspect entry 1351 or later until explicit user authorization.
- Exact cadence: each 200-file stage has **four 50-file reporting sections**; each 50 has **two exact 25-file checkpoints**. Thus each 200 has eight 25-file checkpoints.
- A 25-file checkpoint is committed and re-read/verified before the next 25 begins.
- A 50-file boundary is the report/pause boundary. Do not ask permission between the two 25-file checkpoints inside an authorized 50. Stop after the 50 and wait for explicit approval.

## Next section (not yet authorized)

- Completed reporting section: **1301–1350** (50/50), consisting of committed checkpoints 1301–1325 and 1326–1350.
- Next section: **1351–1400**, checkpoints **1351–1375** and **1376–1400**.
- This next section is unauthorized. Do not inspect entry 1351 until the user explicitly authorizes it.
- After the 1351–1400 section is complete, globally revalidate and close stage 1201–1400 at 200/200.

## Stage layout from 1401 onward

- Stage 1401–1600: reporting sections 1401–1450, 1451–1500, 1501–1550 and 1551–1600.
- Each section comprises two exact 25-file checkpoints.
- Continue the same 4 × 50 / 2 × 25 pattern for every later 200-file stage.
- Stop after every 50-file section and obtain explicit approval before the next 50.

## Source pin and selection

- Upstream: starling-client/starling, ref `starling`.
- Pinned commit: `5432370d573642041585c69b40fb045b27133488`.
- Inspect only this exact snapshot.
- Build the recursive source tree and verify it is complete/not truncated.
- Select paths in deterministic lexical order, excluding every path already in the full Index. Never repeat or renumber a source path.
- Verify every source SHA and byte size against the pinned tree.

## Per-file inspection requirements

- Read every file fully through EOF.
- For files over 400 logical lines, use consecutive, gap-free ranges of no more than 400 lines each.
- Record canonical logical line count, byte size, SHA and exact coverage.
- Canonical line count: LF-terminated line count, plus one only when the blob does not end in LF; do not count a synthetic empty split item after a final LF.
- Each Ledger entry must contain exactly these 15 fields, each once: Source repository/ref; Source SHA; Size; Coverage; Purpose; Dependencies / cross-references; Implementation / logic; Findings; RustChunk relevance; Decision; 1.20.1 implications; Follow-up checks; RustChunk implementation mapping; Evidence required before adoption; Completion state.
- Decision primary labels: essential, adaptable, rewrite required, integrate, unnecessary, undetermined. Qualifiers: reference, version-specific, optional/future.
- Separate observed facts, interpretation, suspected defects and proposed target-version work. Do not claim a suspected defect is reproduced without evidence.

## Five-document checkpoint persistence

At every 25-file checkpoint, update together:
1. The applicable Ledger.
2. AUDIT_INDEX.md.
3. AUDIT_QA_LOG.md.
4. AUDIT_CONTINUITY.md.
5. AUDIT_PROTOCOL.md when state/rules need synchronization; keep it consistent with Continuity.

Validate sequential numbering, global path uniqueness, all 15 fields, pinned SHA/byte size, complete EOF coverage, Index/Ledger alignment and correct resume state. Commit the five documents together in one commit. Re-fetch all five from GitHub and verify saved contents/blob SHAs before calling the checkpoint complete.

Before each new chat continuation, re-fetch the latest branch head and read this file plus the latest audit documents. If context is uncertain, stop and reconcile rather than guessing.

## Main project boundary

- Main RustChunk development remains on `main`; Starling audit artifacts remain on the audit branch.
- The audit is research and compatibility assessment, not proof that features have been implemented.
- Reading embedded tests is not running them.
- Never claim build, unit-test, benchmark, APK build, Forge launch, Android-device validation, multiplayer join, LAN, UPnP or relay tests unless actually executed.
- Starling/Horizon is based on later Minecraft 26.3 behavior. Do not treat its registries, packets, IDs, assets or native mod API as Forge 1.20.1-compatible without separate target-era evidence.
- The Horizon native-mod build pipeline and app-library replacement are high-privilege mechanisms; do not treat them as normal Forge mod loading.

## Backup state

- `backup/audit-1000-unique-2026-11-18` -> `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`.
- `backup/audit-1000-final-2026-11-19` -> `4ba78eba6ebc2c89bb455a68a3cb9ddfe5f30e23`.
- Both backup refs must remain untouched. They are procedurally read-only; GitHub currently reports branch protection disabled.
- Next backup only after entries 1001–2000 are fully committed and globally verified. No backup after 25, 50, 100 or 200 entries.
- Do not recreate or alter the deleted archive branch. Do not place audit files on main.

## Existing completed work

- Entries 1–1000: completed and historically QA-reviewed in documented ranges.
- Entries 1001–1200: all ten former 20-file checkpoints completed and verified; stage closed.
- Entries 1201–1350: completed; entries 1201–1300 retain their historical 20-file checkpoint records; checkpoints 1301–1325 and 1326–1350 are recorded in the Ledger and QA log.
- Index at 1350: 1350 sequential rows, 1350 unique paths, zero duplicate paths.
- Active Ledger: 150 entries (1201–1350), exactly 15 required fields per entry.
- The 1281–1300 checkpoint includes mobile touch/input, mod-related systems, mounts, name tags, audio and native-mod extension files. Several native/Horizon-only capabilities were explicitly classified unnecessary or optional for the Forge 1.20.1 baseline; see entries 1284–1300.
- No claim is made that the Starling audit means RustChunk has built or launched Minecraft/Forge or passed an Android/server test.

## Historical duplicate corrections

- Entries 631, 632, 633, 754, 762 and 766 were corrected to unique paths: pick_block_data_fixture.rs, place_recipe_fixture.rs, player_sync_fixture.rs, raid_fixture.rs, random_tick_sections_probe.rs and recipe_book_fixture.rs.
- Entry 1089 was corrected to `crates/horizon-client/src/game/blockctl.rs`; entry 121 remains the only `entry.rs` occurrence.


## Work pacing and chat-report rule

- Work serially: one file or one independent check at a time. Finish it before starting another; do not parallelize unrelated reads or validations.
- For files over 400 lines, read consecutive ranges sequentially, each at most 400 lines. Do not leave a file half-read while moving to another.
- Keep operations bounded and avoid long, bundled actions that make the user think the session is stuck. Give only a brief current-item status when a delay makes that useful; do not send filler updates.
- Chat reports stay minimal: completed range/status, verification result or blocker, and exact next point/authorization. Do not volunteer file-by-file descriptions, architecture recaps, file-dependency explanations or internal change lists unless the user asks.
- Keep detailed evidence in the audit documents rather than repeating it in chat. At 25-file commits, a short confirmation is enough; after each 50-file section, report briefly and STOP for approval.
- On interruption or chat change, resume from the latest verified commit and this Continuity record; never repeat completed work or infer an unverified state.


## Latest verified checkpoint — 1301–1325

- Branch head before this checkpoint commit: `62695e4d2f1d41630f4ba90c792ee0569c7d3b4b`.
- Completed new entries: 1301–1325 inclusive (25/25).
- Index: 1325 sequential entries and 1325 unique source paths.
- Active Ledger: entries 1201–1325, 125 records; every entry has exactly 15 required fields.
- Current authorized section: 1301–1350. The second checkpoint 1326–1350 is already authorized; continue without an interim permission request.
- Stop after 1350 and wait for explicit authorization before 1351–1400.


## Latest verified checkpoint — 1326–1350

- Branch head before this checkpoint commit: `193fc62bd995034b159f82b526292b990b8174c5`.
- Completed new entries: 1326–1350 inclusive (25/25).
- Reporting section 1301–1350 is now complete (50/50).
- Index: 1350 sequential entries and 1350 unique source paths.
- Active Ledger: entries 1201–1350, 150 records; every entry has exactly 15 required fields.
- Current authorization state: STOP. The next range 1351–1400 requires a separate explicit user instruction.


## Duplicate-path correction — checkpoint 1326–1350

- Read-back QA found the initially recorded entry 1349 path `crates/horizon-client/src/lib.rs` duplicated entry 118. Entry 118 remains the only `lib.rs` audit entry.
- Corrected 1349 to `crates/horizon-client/src/locator_bar.rs` and added `crates/horizon-client/src/lod_net.rs` as entry 1350, fully reviewed to EOF.
- The final Index is 1350 sequential entries and 1350 unique source paths. The active Ledger contains 150 entries, 1201–1350, with exactly 15 fields per entry.
- Reporting section 1301–1350 remains complete. STOP at 1350; do not begin 1351–1400 until explicit user authorization.

## Latest verified checkpoint — 1351–1375

- Branch head before checkpoint work: 38dab3b1018f4b023c22dcb549da1b1024588f4a.
- Latest checkpoint documentation commit: 9098ab72dabbea4ca59d5ace32460e511b8127bb.
- Completed entries: 1351–1375 inclusive (25/25).
- Index: 1375 sequential entries and 1375 unique source paths.
- Active Ledger: entries 1201–1375, 175 records; each has exactly 15 required fields.
- QA: checkpoint 1351–1375 recorded; the symlink at 1351 and complete 10,360-line native_hooks.rs inspection are explicitly noted.
- Pinned upstream remains starling-client/starling @ 5432370d573642041585c69b40fb045b27133488.
- Authorization: the user's “start” instruction authorized the full section 1351–1400. Continue with 1376–1400 without asking for another permission at 1375.
- Next checkpoint: 1376–1400. Stop after entry 1400 and wait for explicit authorization for 1401–1450.
- No builds/tests/device checks were run. main and backups remain untouched; next backup remains governed by the existing 1000-entry cadence.
