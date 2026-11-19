# RustChunk Starling Audit QA Log

This log records quality-control reviews of existing Starling audit entries. QA reviews do not create new source-audit entries, renumber files, or change the pinned upstream snapshot.

## Frozen baseline

- Repository: `imamirhosein/RustChunk`
- Pinned upstream source: `starling-client/starling`
- Pinned upstream ref: `starling`
- Pinned upstream commit: `5432370d573642041585c69b40fb045b27133488`
- Baseline RustChunk commit: `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`
- Original backup branch: `backup/audit-1000-unique-2026-11-18`
- The duplicate archive branch `archive/audit-1000-frozen-2026-11-18` was historically created from the baseline, then deleted by the repository owner; it is absent from the current branch list.
- The original backup remains at the baseline commit `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4` and is treated as read-only by this audit procedure. GitHub reports `protected: false`; it is not server-locked.

## QA batch 1–100 — 2026-11-18

### Scope and checks

- Reviewed existing ledger entries 1–100 in `AUDIT_LEDGER_1_400.md`; no source entry was renumbered and no new source entry was added.
- Verified the 100 entry numbers are sequential from 1 through 100 with no missing or duplicate numbers.
- Verified each entry contains each of the 15 required fields exactly once.
- Verified all 100 source paths exist in the pinned upstream tree and every recorded source blob SHA matches the corresponding blob SHA in that pinned tree.
- The upstream recursive tree response contained 5,749 entries and was not truncated.
- Compared the Index rows 1–100 with the ledger: all 100 entry numbers and source paths agree.
- Checked all 1,000 Index rows for repeated source paths: none were found.
- Retrieved the pinned source blobs for entries 1–100 and compared recorded line/byte metadata and coverage against the actual blob contents. All coverage ranges for this QA batch are contiguous and end at the canonical final line.
- Verified the Index decision tags for rows 1–100 use only the protocol's approved primary decisions and secondary qualifiers; no placeholder decision remains in this range.

### Corrections made on the active audit branch

1. Corrected off-by-one logical line counts for entries:
   - 1 — `README.md`: 146 → 145
   - 2 — `LICENSE`: 22 → 21
   - 3 — `THIRD_PARTY_NOTICES.md`: 300 → 299
   - 9 — `packaging/android/build_apk.py`: 381 → 380
   - 10 — `packaging/lowbudget/make.py`: 257 → 256
   - 11 — `research-lock.json`: 70 → 69
   - 12 — `roadmap/starling.md`: 188 → 187
   - 13 — `docs/horizon-parity.md`: 39 → 38
   - 14 — `docs/android-armv7.md`: 111 → 110
   - 15 — `docs/android-controls.md`: 79 → 78
   - 16 — `docs/android-accounts.md`: 76 → 75
   - 17 — `docs/android-diagnostics.md`: 97 → 96
   - 18 — `docs/native-mods.md`: 3,049 → 3,048
   - 19 — `docs/starling-native-mods.md`: 121 → 120
   - 20 — `docs/mods.md`: 493 → 492
2. Corrected the associated Coverage endpoints for entries 1, 3, 9, 18, 19 and 20. For entry 18, the consecutive ranges now end at line 3,048.
3. Aligned ledger and Index decision classifications for entries 34, 60, 85 and 95:
   - 34 — `crates/horizon-prepare/src/artifacts.rs`: `Adaptable; Rewrite required; Version-specific`
   - 60 — `crates/horizon-protocol/src/resolver/dns.rs`: `Adaptable`
   - 85 — `crates/horizon-client-input/src/key_names.rs`: `Adaptable; Version-specific`; replaced the Index placeholder `See ledger decision`
   - 95 — `crates/horizon-client/src/touch.rs`: `Adaptable; Rewrite required`
4. Extended `AUDIT_PROTOCOL.md` with QA rules: one exact 15-field set per entry, pinned SHA checks, canonical line counting, contiguous coverage ranges, approved decision vocabulary, and mandatory QA logging.

### Active-branch commits

- Ledger corrections: `8ff9e7410af29acd76ca30d0a87020821f28bebc`
- Index corrections: `1b28f54fc6bef7a162fb15cfcde771fbcb60c944`
- QA protocol rules: `148adc6d1a912a25d67fe36566012adbea41ca70`

### Verification after writes

- Re-read the updated ledger and Index from GitHub.
- Confirmed 400 ledger entries remain present and entries 1–100 still have all 15 required fields exactly once.
- Confirmed the four corrected decision rows agree between ledger and Index.
- Confirmed no `See ledger decision` placeholder remains in the Index.
- Confirmed the QA protocol contains the canonical line-count and decision-taxonomy rules.
- Confirmed the archive and original backup branches still point to baseline commit `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`; the archived ledger blob SHA remains unchanged.

### Limits of this QA result

- This is a structural, provenance, metadata, coverage and cross-document QA pass for entries 1–100. It is not a fresh exhaustive semantic re-audit of every source implementation claim.
- Reading source code or tests does not mean tests were run. No build, unit-test suite, Android APK, device test, Minecraft client launch, or multiplayer-server connection was executed as part of this QA batch.
- Do not treat a source-review completion marker as proof of runtime compatibility with Minecraft 1.20.1.

## Next QA batch

- Next range: **101–200**.
- Continue only on `starling-1.20.1-port-audit`.
- Do not modify `archive/audit-1000-frozen-2026-11-18` or `backup/audit-1000-unique-2026-11-18`.


## QA batch 101–200 — 2026-11-18

### Scope and checks

- Reviewed entries 101–200 in `AUDIT_LEDGER_1_400.md`; no new source-audit entries were created and no entry numbers were changed.
- Verified exactly 100 sequential ledger entries (101–200), each with all 15 required fields exactly once.
- Compared all 100 paths against the complete Index: all 100 entry numbers and paths match.
- Compared the batch against all four ledgers covering entries 1–1000: all 100 paths are unique within the batch and absent from entries 1–1000. The full existing set still contains 1,000 unique paths.
- Verified every path exists in the pinned Starling tree (5,749 tree entries; tree not truncated).
- Verified 99 recorded source SHAs matched the exact pinned Git tree. Entry 174 had a malformed SHA and was corrected to the exact tree SHA listed below.
- Fetched the source blobs and checked logical line counts and byte sizes. Source files in this batch end with LF; the trailing LF is not an additional logical line.
- Checked coverage endpoints. Long-file ranges remain consecutive and gap-free after correcting final endpoints.
- Audited Index decisions for this range and normalized 15 nonstandard descriptions/placeholders into the approved decision vocabulary. Explanatory wording remains in the reason column or after an em dash in the ledger.

### Corrections made on the active audit branch

1. Corrected logical line counts and Coverage endpoints for all 100 entries. For 97 files, the existing count included the synthetic empty split element after the final LF and was one too high.
2. Corrected two under-counted source files after re-reading their complete pinned blobs:
   - 167 — `benchmarks/entity-model-document-2026-10-14/generate_fixture.py`: 29 → 31 logical lines; 1,605 bytes.
   - 168 — `benchmarks/entity-model-document-2026-10-14/README.md`: 61 → 78 logical lines; 3,762 bytes.
3. Corrected entry 174 — `benchmarks/model-state-parse-2026-10-14.md`:
   - SHA corrected to `15a50e8ff7ece5a567909f18bd76d9ed5777793c`.
   - Logical line count corrected from 70 to 69; 3,671 bytes.
4. Added byte sizes from the pinned Git tree to the Size field for all 100 entries. Full logical line counts and byte sizes now use one consistent convention.
5. Normalized Decision labels in entries 109, 122, 126, 146, 153, 156, 157, 160, 161, 164, 166, 167, 169, 170 and 171; matching Index rows were updated.
6. Entry 167's findings now explicitly note that the script writes to the caller-supplied output path; callers should use a disposable destination.

### Verification after writes

- The archive and original backup branches were rechecked before the updates and both still pointed to baseline commit `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`.
- The connector exposes no branch-deletion operation. Therefore the duplicate archive branch could not be removed from this session; the original backup remains untouched and authoritative, while the archive duplicate remains present at the same baseline commit.
- GitHub reports `protected: false` for both backup branches; no server-side branch protection was enabled. Do not claim these branches are technically locked.
- No Rust build, benchmark, unit-test run, APK build, Android device test, client launch, or server connection was performed during this QA batch.
- Post-write verification completed: re-read all four ledger files and the Index; confirmed 1,000 unique existing paths, 100 new unique paths at 101–200, 100/100 SHA-and-byte matches against the pinned Git tree, 100/100 Index path matches, no field or decision-tag violations, and no coverage-range gaps or overlaps. The corrected SHA and line/byte metadata for entry 174 were also rechecked.

### Next QA batch

- Next range: **201–300**.
- Continue only on `starling-1.20.1-port-audit`.
- Keep the original backup branch unchanged. Do not modify the duplicate archive branch.


### Supplemental source-range verification — entry 111

- Re-read pinned `crates/horizon-world/src/nbt.rs` in exact consecutive ranges 1–400, 401–800 and 801–1,151.
- All three requests returned source SHA `b27bf4fda2c50ec7b81adb316aa0d12359e2dc56`.
- Ranges contain 400 + 400 + 351 = 1,151 logical lines, with no gaps or overlaps.
- Updated entry 111 Coverage to record these exact ranges. No source implementation claim or completion status was promoted by this re-read.


## QA batch 201–300 — 2026-11-18

### Scope and checks

- Reviewed entries 201–300 in `AUDIT_LEDGER_1_400.md`; this was a QA correction pass, not a new source-audit batch.
- Verified exactly 100 sequential ledger entries (201–300), each with all 15 required fields exactly once.
- Compared the Index rows 201–300 with the ledger: all 100 entry numbers and source paths agree.
- Compared the batch against entries 1–200 and all remaining ledgers through 1000: all 100 paths are unique within the batch and absent from the previous 200; the full 1,000-entry set still has 1,000 unique paths.
- Verified every source path exists in the pinned Starling tree at commit `5432370d573642041585c69b40fb045b27133488` (5,749 tree entries; not truncated).
- All 100 recorded source blob SHAs match the pinned Git tree exactly.
- All 100 recorded byte sizes match the pinned Git tree exactly.
- Fetched and parsed each of the 100 source JSON files. All 100 parse successfully; no invalid JSON was found.
- For all 100 source files, the prior recorded line count was one greater than the canonical logical line count because the terminal LF was counted as an additional blank line. Corrected all 100 line counts by one. No path, source SHA or byte size changed.
- Coverage entries describe complete JSON retrieval and contain no numeric chunk ranges; source blobs were retrieved in full for parse and line-count checks.
- Standardized all 100 Ledger Decision labels and matching Index decision cells to the approved taxonomy. The exact original descriptive assessment is retained alongside each Ledger decision, and contextual reasons remain in the Index reason column.

### Corrections made on the active audit branch

1. Corrected logical line counts for entries 201–300 (all 100 entries: prior count minus one). Byte sizes were retained because all matched the pinned Git tree.
2. Standardized 100 Decision labels:
   - Adaptable primary decision with Version-specific qualifier for reusable test methods, algorithms, data structures and benchmark patterns.
   - Added Reference qualifier where the original entry explicitly limited the artifact to reference use.
   - Used Unnecessary; Reference; Version-specific for the two component-model reports explicitly marked "Reference only" and incompatible with 1.20.1's legacy item-data model.
3. Kept original descriptive assessment text in every Ledger Decision line and preserved the detailed Index reason column. No technical conclusion was silently dropped.

### Verification

- All 100 JSON sources parsed successfully.
- All 100 SHAs and byte sizes match the pinned upstream tree.
- All 100 source paths are present in the pinned tree.
- 1,000 ledger entries remain sequential and globally path-unique.
- All 100 Index paths still match the Ledger; all 100 decisions use approved labels.
- No changes were made to either the original backup branch or the duplicate archive branch.
- No Rust build, unit-test suite, benchmark execution, APK build, Android device test, Minecraft launch, or multiplayer-server connection was performed.

### Next QA batch

- Next range: **301–400**.
- Continue only on `starling-1.20.1-port-audit`.
- Keep the original backup branch unchanged. The duplicate archive branch is still present because the available GitHub connection has no branch-deletion action.


## QA batch 301–400 — 2026-11-18

### Scope and checks

- Completed the full source review for ledger entries 301–400 (100 files), using the pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Verified exactly 100 sequential entries, 100 unique source paths, and no missing source paths in the pinned, non-truncated Git tree.
- Verified all 100 recorded source blob SHAs and byte sizes against the pinned Git tree.
- Retrieved all 100 source JSON artifacts and parsed each in full: all 100 are valid JSON.
- Recomputed logical line counts from the complete source text. The prior ledger count included the terminal newline as an extra blank line; corrected the count by one for each of the 100 entries. Byte sizes and source SHAs remain unchanged.
- Verified all 100 Ledger entries contain the 15 required audit fields exactly once.
- Verified the 100 Index rows match the Ledger entry numbers and source paths.
- Standardized decision labels in both Ledger and Index: reusable test/algorithm/architecture material is marked `Adaptable; Version-specific`; evidence-only artifacts are marked `Reference; Version-specific`. Each original descriptive assessment is preserved in the Ledger and each detailed Index reason remains intact.
- Reviewed native smoke reports as evidence, not as proof of release readiness. The reviewed runs are Linux x86_64 resident-scene/software-renderer checks; they do not establish Android ARM64 performance, 1.20.1 gameplay completeness, Windows/macOS release readiness, or Aternos multiplayer compatibility.
- Key remaining integration gaps recur across fluid/world binding, 1.20.1 registries and item/component schema, complete entity/player tick lifecycle, networking, renderer pixel parity and device/hardware validation.

### Verification limits

- No Rust build, unit-test suite, benchmark execution, APK build, Android device test, Minecraft launch, or multiplayer-server connection was performed in this audit pass.
- The smoke-test evidence and source parity artifacts are inputs for future implementation and test design, not proof that the corresponding complete subsystem exists in RustChunk.

### Corrections

1. Corrected logical line counts for entries 301–400 (100 entries, each prior count minus one).
2. Standardized all 100 decision labels in Ledger and Index while preserving the original descriptive decision text and Index reasons.
3. No source artifact, path, source SHA, or byte-size record was changed.

### Repository safety

- Active work branch: `starling-1.20.1-port-audit`.
- Stable backup branch: `backup/audit-1000-unique-2026-11-18`.
- The duplicate archive branch was deleted by the repository owner and is no longer present.
- The backup branch was not modified.

### Next range

- Next audit/QA range: **401–500**.


## QA batch 401–500 — 2026-11-18

### Scope and checks

- Reviewed exactly 100 existing entries (401–500); this was a QA correction pass, not a new source-audit batch. No entry was renumbered and no new source path was added.
- Verified the 100 entry numbers are sequential, each of the 15 required fields appears exactly once, and the 100 Index rows match the Ledger entry numbers and paths.
- Verified all 100 paths exist in the pinned Starling commit `5432370d573642041585c69b40fb045b27133488`. The recursive source tree contained 5,749 entries and was not truncated.
- Verified all 100 recorded source blob SHAs and byte sizes against the pinned Git tree.
- Retrieved all 100 source files in full: 94 JSON artifacts parsed successfully; the six Markdown/text files were read completely. This validates retrieval and parseability, not that any upstream test was rerun.
- Compared the batch with the complete 1,000-entry audit index/ledgers: the 1,000 paths remain unique, and the 401–500 paths are not repeated elsewhere.
- Checked the Minecraft/platform provenance in the parity artifacts: the scoped parity data targets Minecraft 26.3 and Linux x86_64; server measurements are dedicated-server workloads and cannot establish Android client FPS, Galaxy A22 performance or Minecraft 1.20.1 compatibility.
- Standardized all 100 Ledger Decision labels and matching Index decision cells to the approved primary/secondary taxonomy. Original descriptive assessments remain in the Ledger after the standardized label; detailed reasons remain in the Index.
- Rechecked all multi-range Coverage endpoints and corrected the seven terminal-line overcounts listed below.

### Corrections made on the active audit branch

1. Corrected final Coverage ranges to the actual final logical line:
   - 489 — `benchmarks/server-2026-10-17/earlier-002f14ce/walk-explore.json`: 401–696 → 401–695.
   - 492 — `benchmarks/server-2026-10-17/walk-explore.json`: 401–696 → 401–695.
   - 494 — `benchmarks/server-equal-mixed-2026-10-18/load.txt`: 401–703 → 401–702.
   - 495 — `benchmarks/server-equal-mixed-2026-10-18/mixed-equal.json`: 401–422 → 401–421.
   - 498 — `benchmarks/server-equal-work-2026-10-17/horizon.json`: 401–416 → 401–415.
   - 499 — `benchmarks/server-equal-work-2026-10-17/run-1-before-chunk-cache.json`: 401–694 → 401–693.
   - 500 — `benchmarks/server-equal-work-2026-10-17/run-2-chunk-cache.json`: 401–414 → 401–413.
2. Normalized all 100 decisions in Ledger and Index to the approved vocabulary:
   - Adaptable entries use `Adaptable; Version-specific`.
   - Test/method/reference entries use `Adaptable; Reference; Version-specific`.
   - Historical, context-only and server-only result artifacts use `Unnecessary; Reference; Version-specific`.
   - Entries 457–458 remain `Undetermined; Reference; Version-specific` because the stored-effect artifacts explicitly lack sufficient aggregate parity evidence.
3. Corrected entry 494's load-trace finding: the bot host samples range from 64–95% CPU, while sampled ThinkPad CPU values range from 2–100%; host-load variability remains a caveat.
4. Corrected entry 496's finding: the README compares Horizon and Vanilla only; it contains no Paper results. The previous "Paper often faster" statement was removed. The source reports a Vanilla watchdog stop at 100 bots and contains no client/Android measurements.
5. Clarified RustChunk relevance for server benchmark and load-trace entries 486–500 so server-side data is not presented as evidence of client FPS, chunk rendering or Android performance.

### Verification after writes

- Re-read the updated Ledger, Index and QA log from GitHub.
- Confirmed entries 401–500 remain sequential, all 15 fields are present exactly once, all 100 Ledger and Index paths match, and each standardized Ledger decision matches its Index decision cell.
- Confirmed all seven Coverage endpoints now terminate at their actual final logical lines.
- The original backup branch `backup/audit-1000-unique-2026-11-18` remains at `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`; it was not modified. GitHub reports `protected: false`, so it is procedurally read-only, not server-locked.
- The duplicate archive branch is absent from the current GitHub branch list, consistent with its deletion by the repository owner.
- No Rust build, unit-test suite, benchmark execution, APK build, Android device test, Minecraft client launch or multiplayer-server connection was performed.

### Limits of this QA result

- This is a provenance, structure, coverage, parseability and targeted claim-correction pass. It is not a fresh exhaustive semantic re-audit of every fixture value or upstream implementation, and reading benchmark artifacts is not running the benchmarks.
- None of these results establishes RustChunk runtime readiness for Minecraft 1.20.1 Forge, Android ARM64 or Aternos multiplayer.

### Next QA range

- Next range: **501–600**.
- Continue only on `starling-1.20.1-port-audit`; keep the original backup branch unchanged.


## QA batch 501–600 — 2026-11-18

### Scope and checks

- Reviewed exactly 100 existing entries (501–600) on the pinned Starling snapshot `5432370d573642041585c69b40fb045b27133488`. This was a QA correction pass, not a new source-audit batch; no entry number or source path was added, removed or renumbered.
- Verified the source tree contains 5,749 entries and is not truncated. All 100 indexed paths exist in the pinned tree; all recorded source blob SHAs and byte sizes match the corresponding pinned tree entries.
- Verified the 100 entries are sequential, every one of the 15 required fields occurs exactly once, and all 100 Index rows match Ledger numbering and source paths.
- Checked global uniqueness across all four ledgers (entries 1–1000): 1,000 entries and 1,000 unique source paths; no duplicate path was introduced by this batch.
- Retrieved and validated the data artifacts: 14 benchmark/observation JSON files and 8 Horizon content-template JSON files parsed successfully; all 16 records in `runs.jsonl` parsed as individual JSON objects. All 100 source files were fetched from the pinned raw source; long files were inspected using consecutive ranges no larger than 400 lines.
- Server benchmark files 501–510 are dedicated-server/bot measurements on Linux x86_64; they do not measure RustChunk client rendering, mobile FPS or Android ARM64 performance.
- Files 511–518 are loader-only JSON microbenchmarks, explicitly excluding full application startup and first-frame timing; entry 519 is one Linux x86_64/OpenJDK 25 tick-state observation, not a controlled Android benchmark.
- Entries 520–536 concern Horizon's HMOD/HNMOD tooling and templates. They are not Forge 1.20.1 mod support; only generic packaging/resource ideas may be considered after independent redesign and target-version validation.
- Entries 538–542 are dedicated-server anti-cheat code, not client runtime code. Entries 576–582 are a synthetic server bot or server chat-moderation filter; neither is a playable client implementation.
- Entries 543–572 cover Horizon's CPU-side asset/model decoding, blockstate, geometry, palette and item-model logic plus fixture harnesses. They can inform a native asset pipeline and differential tests, but use Minecraft 26.3 schemas/resources and do not establish GPU rendering, Android pixel parity or 1.20.1 compatibility.
- Entries 573–575 describe benchmark reporting and hardware telemetry. The report design can inform future diagnostics; the hardware probe does not itself provide complete Android GPU/display/memory detection.
- Entries 583–600 cover client-core state/codecs and utilities. Their architecture may be adaptable, but the catalogues, component forms, protocol integration and version-specific semantics must be rebuilt or verified against Minecraft Java 1.20.1.

### Corrections made on the active audit branch

1. Corrected the 22 Coverage endpoints that counted the synthetic trailing empty split element as one extra source line:
   - 503 `server-four-way-2026-10-19/load.txt`: 801–871 → 801–870.
   - 504 `server-four-way-2026-10-19/report-25-50.json`: 401–438 → 401–437.
   - 510 `server-vs-pumpkin-2026-10-31.json`: 1201–1498 → 1201–1497.
   - 513 `shared-json-five-way-20261017.json`: 401–454 → 401–453.
   - 514 `shared-json-five-way-final-20261017.json`: 401–583 → 401–582.
   - 518 `shared-json-variant-isolation-20261017.json`: 401–454 → 401–453.
   - 522 `cargo-horizon-mod/src/main.rs`: 401–402 → 401–401.
   - 540 `horizon-anticheat/src/lib.rs`: 401–427 → 401–426.
   - 541 `horizon-anticheat/src/prediction.rs`: 401–417 → 401–416.
   - 560 `horizon-assets/src/item_model.rs`: 1601–1757 → 1601–1756.
   - 568 `horizon-assets/src/math.rs`: 401–579 → 401–578.
   - 569 `horizon-assets/src/model.rs`: 401–529 → 401–528.
   - 572 `horizon-assets/src/resolve.rs`: 401–583 → 401–582.
   - 575 `horizon-bench/src/lib.rs`: 401–464 → 401–463.
   - 577 `horizon-bots/src/main.rs`: 1601–1661 → 1601–1660.
   - 580 `horizon-chat-filter/src/lib.rs`: 401–547 → 401–546.
   - 583 `horizon-client-core/src/block_entity_nbt.rs`: 801–1002 → 801–1001.
   - 585 `horizon-client-core/src/component.rs`: 401–716 → 401–715.
   - 590 `horizon-client-core/src/lang.rs`: 401–648 → 401–647.
   - 591 `horizon-client-core/src/nbt_path.rs`: 801–902 → 801–901.
   - 597 `horizon-client-core/src/scoreboard.rs`: 1601–1613 → 1601–1612.
   - 600 `horizon-client-core/src/text.rs`: 801–824 → 801–823.
2. Standardized all 100 Ledger Decision labels and matching Index decision cells to the approved primary/secondary vocabulary while retaining the original descriptive assessment after the Ledger separator and keeping explanatory reasons in the Index.
3. Reworded RustChunk relevance so server benchmarks, loader-only measurements, Horizon-only mod packaging, server anti-cheat/bot/chat-filter code, CPU asset/model utilities and client-core utilities are not misrepresented as working Android client features or Forge 1.20.1 support.
4. Clarified entry 516's purpose/scope: it measures the asset loader, not full application startup. Clarified entry 600's line count as 823 logical lines and retained the trust boundary that `open_file` actions are only allowed in locally created text, never untrusted server/LAN text.

### Verification after writes

- Re-read the updated Ledger, Index and QA log from GitHub.
- Confirmed 100 sequential entries, all 15 fields exactly once, 100/100 Ledger-to-Index path matches and 100/100 standardized decision matches.
- Confirmed each corrected Coverage endpoint matches the canonical logical line count from its complete pinned source.
- Confirmed all 100 source SHAs and sizes match the pinned Git tree; all 22 corrected Coverage ranges are consecutive and end at their source's final logical line.
- The primary backup `backup/audit-1000-unique-2026-11-18` remains unchanged at `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`. It remains procedural read-only only: GitHub reports `protected: false`. The duplicate archive branch is absent. A new consolidated backup is intentionally deferred until QA reaches 1000/1000.
- No Rust build, unit-test suite, benchmark execution, APK build, Android device test, Minecraft client launch or multiplayer-server connection was performed.

### Limits of this QA result

- This is a provenance, structure, parseability, coverage and targeted scope/claim QA pass; it is not a fresh exhaustive semantic re-audit of every implementation detail, and reading a test file is not running its tests.
- The source snapshot targets Minecraft 26.3; these entries do not prove RustChunk's Minecraft 1.20.1 vanilla, Forge, Android ARM64 or Aternos compatibility.

### Next QA range

- Next range: **601–700**.
- Continue only on `starling-1.20.1-port-audit`; do not modify the original backup branch.


## QA batch 601–700 — 2026-11-18

### Scope and provenance

- QA-reviewed exactly 100 existing entries (601–700) against pinned Starling commit `5432370d573642041585c69b40fb045b27133488`; no source path was added, removed or renumbered.
- The pinned source tree contains 5749 entries and is not truncated. All 100 paths exist in that tree and all 100 recorded Source SHAs match the pinned Git blob SHAs.
- Canonical byte counts were checked against the pinned tree and fetched source. Corrected entry 620 from 152,652 to 152,654 bytes.
- Corrected logical line counts for the three replacement fixtures: entry 631 97→96, entry 632 99→98, entry 633 73→72. Final-LF files do not count the synthetic empty split element as a source line.
- Rewrote long-file Coverage records as consecutive, gap-free ranges of at most 400 logical lines, ending at each source's canonical final line.
- Verified the Index has 1,000 sequential rows and 1,000 unique source paths; entries 601–700 match their Ledger paths. No duplicate path exists in the full index.

### Critical detail-alignment correction

- The three replacement fixture paths at 631–633 were correctly described, but the descriptive fields after entry 633 had not been re-aligned to the updated source-path sequence.
- Source repository/ref, Source SHA, Size and Coverage were already attached to the correct path. Starting at entry 634, the other 11 descriptive fields (Purpose through Completion state) were shifted by three source slots.
- Reconstructed entries 634–636 from their actual pinned source files (`cit.rs`, `cit_armor.rs`, `cit_assets.rs`), then reassigned the detailed fields from slots 634–697 to their correct source paths at 637–700. The Index decisions and reasons were realigned to match the corrected Ledger.
- This corrects the mapping for CIT, clouds, collisions, command-block UI, connected textures, containers, crash reports, datapacks, dynamic light, effects, Ely.by, shaders, environment, extras, fast-hash, fog, fonts, Fusion, the main renderer, Adreno workarounds, animation, GPU arenas, block lighting, compositor, culling, End sky, entity batching/outline/skinning, frame generation, HDR, late sky, lightning, LOD, native shader hooks, occlusion and OIT.
- The original displaced details from slots 698–700 are preserved below for QA 701–703. They belong to source entries 701–703, not to the current entries 698–700. Entries 701–800 have not been approved or modified in this pass.

### Decisions and scope

- Standardized all 100 Ledger Decision fields and matching Index decision cells to the protocol's primary labels (`essential`, `adaptable`, `rewrite required`, `integrate`, `unnecessary`, `undetermined`) with only approved secondary qualifiers (`reference`, `version-specific`, `optional/future`).
- Clarified that OptiFine/CIT, Fusion, dynamic lighting, BlueMap, Ely.by, frame generation, HDR, native shader packs and OIT are optional compatibility/renderer extensions, not prerequisites for the vanilla 1.20.1 baseline.
- Frame generation, HDR and OIT have no demonstrated support or performance benefit on the target Mali-G52 Android device; defer them until separate backend/device evidence exists.
- The section-occlusion graph and far-terrain LOD are different systems: visibility culling must not hide authoritative loaded chunks, and far tiles remain visual approximations only.

### Carry-forward details for QA 701–703

The original displaced detailed fields from slots 698–700 are preserved here. During QA 701–800, apply each record's details to its correct source path at 701–703 after checking SHA, size and actual contents:

### Preserved original shifted detail from slot 698 — carry forward to source entry 701
- Purpose: Shader-pack block atlas conversion
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Copies sprite arrays into a 2D atlas layout expected by shader packs.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional bridge, not needed for vanilla terrain arrays.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Atlas size and array split depend on GPU max texture limits.
- RustChunk implementation mapping: Atlas size and array split depend on GPU max texture limits.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### Preserved original shifted detail from slot 699 — carry forward to source entry 702
- Purpose: Asynchronous center-depth readback
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Copies the centre depth texel through a tiny target and reads it back later without blocking the frame.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Useful non-blocking readback pattern for optional integrations.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Depth formats, copy alignment and map lifetime depend on backend.
- RustChunk implementation mapping: Depth formats, copy alignment and map lifetime depend on backend.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### Preserved original shifted detail from slot 700 — carry forward to source entry 703
- Purpose: Shader-pack per-draw uniforms
- Dependencies / cross-references: Depends on the wgpu renderer, frame graph, shader/asset bindings and GPU adapter capabilities; optional pack paths additionally depend on the relevant shader-pack API.
- Implementation / logic: GPU/render implementation with allocation, pipeline, pass-order, readback or batching responsibilities; optional paths must retain a conservative fallback.
- Findings: Builds uniform slots for individual entity/block entity/particle draw runs.
- RustChunk relevance: Relevant to rendering/GPU architecture; optional renderer extensions must not become prerequisites for vanilla gameplay.
- Decision: Optional shader-pack bridge.
- 1.20.1 implications: Pinned Starling source is from the newer 26.3 line, not Minecraft 1.20.1. Rebuild versioned IDs, registries, packets, codecs, assets and observable behavior from exact 1.20.1 sources.
- Follow-up checks: Newer pack stage names and uniform schema do not match vanilla 1.20.1.
- RustChunk implementation mapping: Newer pack stage names and uniform schema do not match vanilla 1.20.1.
- Evidence required before adoption: Required before adoption: Java 1.20.1 golden/differential fixtures, malformed-input tests where relevant, and backend/device-specific validation for GPU code. None were run in this audit.
- Completion state: Source review and ledger entry complete; no code port, build, test execution or device validation claimed.

### Verification after write

- Re-read the updated Ledger, Index and QA log from GitHub and verify the active branch commit.
- Confirmed 100 sequential entries, all 15 fields exactly once per entry, 100/100 Index path and Decision matches, and the full Index at 1,000 unique paths.
- Confirmed all 100 Source SHAs match the pinned tree; corrected source bytes and logical line counts are recorded; long-file Coverage ranges are contiguous and end at the canonical final line.
- The original backup `backup/audit-1000-unique-2026-11-18` remains unchanged at `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`. GitHub reports `protected: false`, so it is procedurally read-only but not server-locked. The duplicate archive branch remains absent.
- No Rust tests, GPU tests, benchmark execution, APK build, Android device run, Minecraft launch or Forge/server connection was performed.

### Limits and next range

- This pass verifies source provenance, structure, coverage metadata and the detected alignment defect; it does not claim that tests ran or that Android graphics/performance passed.
- The three-entry alignment defect appears to continue into entries after 700. QA 701–800 must use the preserved details above and independently verify the source-to-description mapping before being accepted.
- Next QA range: **701–800**.


## QA batch 701–800 — 2026-11-19

### Scope and checks

- Reviewed exactly 100 existing entries (701–800). This is a corrective QA pass, not a new source-audit batch; no path was reselected or renumbered.
- The pinned Starling tree is complete (5,749 entries, not truncated). All 100 paths exist; all recorded SHA values and byte sizes match the pinned Git tree.
- Verified consecutive numbering, exactly one occurrence of each of the 15 standard fields, and 100/100 Ledger-to-Index path agreement.
- Re-read complete source files from the pinned commit and recorded canonical logical line counts and byte sizes. Long files were inspected to EOF in consecutive sections of at most 400 lines.
- Found that many existing descriptions in 701–800 had drifted onto neighbouring paths. Corrected Purpose, Dependencies, Implementation/logic, Findings, RustChunk relevance, Decision, 1.20.1 implications, follow-up checks and implementation mapping for all 100 records; updated matching Index decisions/reasons.
- Recalculated all 100 Size and Coverage fields from the pinned source. Logical lines exclude a synthetic empty split element after a final LF.
- Separated optional shader-pack/OIT and Horizon-only extensions from the base vanilla client. These files do not establish Forge 1.20.1 support.
- Clarified that entry 774 is Horizon HMOD/HNMOD enablement, not Forge mod discovery or execution.
- Identified important client foundations for later 1.20.1 work: resident sections (719), translucent sorting (731), weather (734), HUD/layout/overlays (743–748), lighting (755), far LOD (756–758), fluid meshing (763), model loading/baking (775, 777), options persistence (787–788), resource packs (794–795) and particles (797).
- Preserved the touch-control decision: keypad profiles (752) and custom button-image handling (791) are references only; RustChunk's touch editor/layout/icons remain independently designed.

### Target-version and device caveats

- Starling is pinned to a newer Minecraft 26.3 snapshot. Do not reuse 26.3 IDs, registries, packets, codecs, resource formats or shader assumptions as Java 1.20.1 facts.
- Horizon HMOD/HNMOD is unrelated to Forge 47.x.
- Adreno-specific shader shortcuts do not establish Mali-G52 correctness/performance.
- Far LOD must remain separate from authoritative near chunks, lighting and simulation; it cannot conceal missing near chunks.
- Offline name/UUID generation is not Microsoft authentication and does not grant online-mode server access.

### Verification after writes

- Re-read the updated Ledger, Index and QA log from GitHub.
- Confirmed 100 entries, exactly 15 fields each, 100/100 Ledger-to-Index path/decision matches, and source SHA/size pairs against the pinned upstream tree.
- Confirmed Coverage ends at each source's canonical logical final line.
- Original backup branch `backup/audit-1000-unique-2026-11-18` remains unchanged at `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4` (GitHub `protected=false`; procedural read-only only). The duplicate archive branch is absent.
- The agreed second backup is deferred until QA reaches 1,000/1,000.
- No Rust build, unit-test suite, benchmark execution, APK build, Android-device test, Minecraft launch or multiplayer-server connection was performed.

### Limits

- This is a complete-source metadata, structure, coverage, description and decision QA pass. Reading tests is not executing them.
- It does not establish RustChunk compatibility with Minecraft 1.20.1, Forge, Android ARM64/Mali or Aternos.

### Next QA range

- Next range: **801–900**.
- Continue only on `starling-1.20.1-port-audit`; do not modify the original backup branch.


## QA batch 801–900 — 2026-11-19

### Scope and verification

- Reviewed exactly 100 existing entries (801–900); no source paths were renumbered or reselected.
- Verified 100 sequential entries, 100 unique paths in the range, no duplicates against the full 1,000-entry Index, and all 100 paths present in the pinned Starling tree.
- Verified all 100 Source SHAs and byte sizes against the pinned tree, exactly one occurrence of each of the 15 required Ledger fields, and Ledger/Index path alignment.
- Corrected the Index decision column for all 100 entries to use the protocol's primary decision labels; explanatory text remains in the reason column.
- Repaired RustChunk implementation mappings across entries 801–858 so they point to the actual subsystem represented by each source file; corrected shifted/malformed mappings including the section-mask, server-terrain, settings, skins, sound, split-screen and weather records.

### Touch asset correction (859–900)

- Visually inspected all 42 PNGs from the pinned source commit. The files are compact 24×24 RGBA control glyphs; default/pressed pairs were checked against the actual filenames and rendered pixels.
- Corrected the shifted glyph descriptions and candidate destination mappings. The image records are now reference-only; they do not imply that RustChunk will copy or ship these assets.
- Inspected `bedrock-touch.json`: it pins Mojang/bedrock-samples revision `15701fa9c13923f12662984c32c124327e40e3d3`. Retrieved its referenced `LICENSE.md`, which states Mojang AB retains all rights and that the files are subject to the Minecraft EULA. This is not an open asset license; no Bedrock asset should be redistributed without EULA review/permission.
- RustChunk's touch layout/editor remains an independent, editable implementation; the PNGs may serve only as visual references pending provenance/license approval.

### Limits and backup

- Source provenance, byte sizes, ledger structure, decision labels and all 42 image pixels were checked. No Rust tests, build, benchmark, APK, Android-device run, Minecraft launch or Forge/server connection was performed.
- Original backup `backup/audit-1000-unique-2026-11-18` remains unchanged; no archive branch was recreated. The agreed second backup remains deferred until QA reaches 1,000/1,000.

### Next QA range

- Next range: **901–1000**.

## QA batch 901–1000 — 2026-11-19

### Scope and verification

- Reviewed exactly the final 100 existing entries (901–1000); no source path was renumbered or reselected.
- Verified 100/100 unique Ledger paths, all present in the pinned upstream tree; all 100 recorded source SHAs and byte sizes match commit `5432370d573642041585c69b40fb045b27133488`.
- Verified consecutive numbering, exactly one occurrence of each of the 15 required fields per entry, and 100/100 Ledger-to-Index path agreement. Index decisions were converted to the standard primary labels and synchronized with each Ledger Decision.
- Repaired the shifted description and asset mapping records in 901–913. Rewrote implementation mappings for code records 929–1000 so each QA fixture points to its own source basename; no production/runtime path is claimed or created.
- Corrected Coverage for all seven files over 400 logical lines: 945 (495), 946 (435), 966 (655), 971 (533), 974 (472), 992 (1,697) and 995 (1,430). Each was read to EOF in consecutive sections of at most 400 lines and the recorded ranges now match the actual inspected ranges.

### Touch image findings (901–928)

- Visually inspected all 28 PNGs from the pinned source blobs and decoded their IHDR dimensions. Twenty-seven are 24×24 RGBA; `stick.png` is the only 48×48 RGBA file; `stick_knob.png` is 24×24.
- Corrected glyph identities for the D-pad arrows/center, drop, inventory, jump, offhand, pause, pick-block, player list, sneak, joystick base/knob and use-item states. The center D-pad tile is not a standalone action button.
- These PNGs remain visual references only. No source image was copied into RustChunk. Asset redistribution rights and every runtime consumer/call-site have not been cleared; RustChunk should implement independent editable touch artwork/layouts.

### Target-version corrections

- Entry 935 is a narrow NeoForge/Horizon carried-content inspection probe, not a Forge 47.x loader or proof that required Forge mods can run.
- Entry 958's command parser fixture is retained as a useful generic differential-test pattern; its command tree, parser IDs and registry payload must be rebuilt for 1.20.1.
- Entry 995 covers both natural spawning and ordinary block-based mob spawners, which are in 1.20.1 scope. Only its Trial Spawner and Vault branches are excluded as 1.21-era features.
- Entry 967's generic entity-predicate harness may be adapted, but its modern variant-component schema is not accepted as 1.20.1 data. Entry 963 remains undetermined until the underlying cushion feature is confirmed against the target registry.
- Windows build customizations (929), destructive fixture generation (938), the newer creative tool component (961) and inline-object syntax (983) are not baseline RustChunk runtime features.

### Limits, backup and next action

- This batch is source/metadata/structure/coverage/decision QA only. No Rust build, unit tests, benchmark execution, APK build, Android-device test, Minecraft launch, Forge 1.20.1 client launch or Aternos/server connection was performed.
- The original backup branch `backup/audit-1000-unique-2026-11-18` remains unchanged at `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`; no archive branch was recreated.
- The second backup remains deferred. Next action: final global QA of entries 1–1000; only after that passes, create the second backup and compare its tree/content against the original backup.

## Global QA 1–1000 — 2026-11-19

### Final verification

- Re-read all four Ledger files from the working branch. Confirmed exactly 1,000 entries, numbering 1–1,000 with no gaps, and exactly 1,000 unique source paths.
- Verified all 1,000 paths exist in the pinned Starling tree at `5432370d573642041585c69b40fb045b27133488`; all 1,000 recorded source SHAs match the upstream Git tree. Every explicit byte-size field also matches the upstream tree.
- Verified exactly one occurrence of each of the 15 required fields in every entry (15,000 field checks; zero errors).
- Verified 1,000 Index rows and exact Ledger-to-Index path agreement. Corrected 20 decision rows where the Index had reversed or omitted the primary decision/qualifier; explanatory prose and quoted historical assessments were not misclassified as decisions.
- Verified all 80 files whose recorded logical line count exceeds 400 have continuous Coverage from line 1 through EOF, with no gap/overlap and no individual inspected range larger than 400 lines. Corrected the remaining inaccurate/ambiguous Coverage records, including `docs/native-mods.md`, `crates/horizon-protocol/src/item.rs`, `secure_chat.rs`, `item/hash/raw.rs` and `crates/horizon-client/src/touch.rs`.
- The final source tree contains 5,749 entries and the recursive tree response was not truncated.

### Backup discipline and execution limits

- Original backup `backup/audit-1000-unique-2026-11-18` remains unchanged at `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`. No archive branch was recreated.
- Global source/metadata/structure/Index/Coverage QA passed. The agreed second backup may now be created from the verified working branch; compare its tree and key audit file contents with the original backup before treating it as valid.
- No Rust tests, build, benchmark execution, APK build, Android-device run, Minecraft launch, Forge 1.20.1 client launch or multiplayer/Aternos connection was performed. Audit verification is not runtime compatibility evidence.

## Audit stage 1001–1200 started — 2026-11-19

- Re-audit starts from the pinned Starling commit `5432370d573642041585c69b40fb045b27133488` and continues in deterministic lexical path order after the last indexed path at entry 1000.
- The 200 source paths were selected from the pinned upstream tree after excluding all 1,000 paths already in the Index. The six corrective-closure paths from the previous stage are already indexed and are not counted again.
- New durable rule: checkpoint every 20 completed files. Each checkpoint writes 20 entries to the range Ledger, matching Index rows and this QA log, validates them, commits, and re-reads the saved state before the next 20 begins.
- Backup cadence: one backup per 1,000 newly completed entries. The current 1,000-entry backups remain unchanged; no backup is due at 1,200.
- Starting active-branch commit before this stage: `adfc618dc1bceccb9ba7a1055fd32a4ed2fcb7fd`. Pinned source baseline: `5432370d573642041585c69b40fb045b27133488`.

## Audit checkpoint 1001–1020 — 2026-11-19

- Completed full source review of exactly 20 new unique files (entries 1001–1020) at pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- The reviewed files comprise 20 text sources; all were read completely. The 723-line motion probe and 430-line scoreboard fixture were read in consecutive sections of at most 400 lines.
- Recorded source SHA, byte size, canonical logical line count, coverage and all 15 required fields for every entry.
- Key findings: recipe fixtures are useful as 1.20.1 parity-harness patterns but require rebuilt registries; scene collision/motion and section meshing are high-value candidates for terrain/player correctness; Crafter UI is later than 1.20.1 and excluded; server-files administration is outside the mobile client core; sound, spawn, sprint, skin, subtitle and tab-list fixtures require version-specific data and/or protocol reconstruction.
- Entry 1010 is server-administration tooling with a predictable temporary-directory cleanup pattern; it is not to be reused against user worlds/server directories. Entry 1017 hardcodes a local 26.3 resource path. Entry 1012 tests legacy skin normalization only, not profile signature authentication.
- Confirmed no source paths duplicate entries 1–1000 or each other; the checkpoint's 20 entries are sequential.
- This is source inspection only. No tests, build, benchmark, APK, Android device, Minecraft client, Forge client or server were executed.
- Next checkpoint: entries 1021–1040. Continue only after re-reading this saved checkpoint from GitHub.

### Checkpoint 1001–1020 post-commit correction

- Re-read the committed first checkpoint and compared all 20 Source SHAs and byte sizes again against the pinned source tree.
- Found and corrected one transcription error in entry 1013's SHA: the ledger had omitted the final two characters. Correct value: `327a8e9d1465a6fd80b6e236fe759417f7e42fe2`; pinned tree SHA and fetched file SHA both agree. The file size remains 6,621 bytes and 145 logical lines.
- This is a metadata correction only; source content, decision and implementation mapping were not changed. Checkpoint remains 20 complete entries.
## Audit checkpoint 1021–1040 — 2026-11-19

- Completed full source review of exactly 20 new unique files (entries 1021–1040) from pinned Starling commit 5432370d573642041585c69b40fb045b27133488.
- Recorded all 15 required fields, upstream SHA, byte size and canonical logical line count for every file. All 20 source texts were read to EOF. The merchant fixture (556 lines) was read as 1–400 and 401–556; the advancement tree/predicate module (1,805 lines) was read as five consecutive sections (1–400, 401–800, 801–1,200, 1,201–1,600, 1,601–1,805); ambient audio (479 lines) was read as 1–400 and 401–479.
- Version findings: text-display entities are relevant but require 1.20.1 metadata/protocol reconstruction; the tooltip-display data component is later than the target; trade tables and random-sequence assumptions are versioned; world-options packets must be rewritten for 1.20.1; advancement integration has newer triggers that must be excluded unless verified; ambient sound data must be translated from 1.20.1 biome effects rather than the newer attributes schema.
- Performance fixtures are methodology references only. The random-tick eligibility benchmark lacks an explicit semantic equality assertion; the resource-memory profile uses platform-specific RSS tools; save-batch benchmark verifies NBT equality but does not time disk I/O; sprite-cache fixture deletes a cache under the supplied root and must only run on a disposable copy.
- Account management (1038) is security-sensitive and coupled to Starling account/provider/platform code; no direct port is approved without independent threat-model and provider review.
- No tests, build, benchmark, APK, Android device, Minecraft client, Forge client, audio playback or server process were executed.
- Validated SHA/byte size for all 20 entries against the pinned upstream Git tree; all 15 fields occur exactly once; numbering is sequential; paths are unique against Index entries 1–1040.
- Next checkpoint: entries 1041–1060. Backup branches remain untouched; backup cadence remains every 1,000 newly completed audit entries only (next due after entry 2000).

### Checkpoint 1021–1040 reconciliation after concurrent duplicate write

- A second concurrent write had appended a duplicate set of entries 1021–1040 and placed the separate `src/game/advancements.rs` file at entry 1039. The duplicate block was removed; the canonical 20 entries 1021–1040 remain exactly once.
- The canonical entry 1039 is `crates/horizon-client/src/advancements.rs` (SHA `1483c1eebaca73e55886fcfef0028e5986a05dcc`, 66,532 bytes, 1,805 logical lines), which was read completely in five consecutive sections of at most 400 lines. The distinct `crates/horizon-client/src/game/advancements.rs` (SHA `18479955a0f183cd5f98d98290a6557c6cb4d7e2`) was not part of this checkpoint and remains eligible for its later lexical position; no review of it is claimed here.
- Updated Index entry 1039 path/decision to match the canonical ledger. Aligned all 20 Index rows with the canonical entries and decisions. The range now contains exactly one each of 1021–1040, with 20 unique paths.
- No additional backup was created. Existing backup branches remain untouched; the next backup remains due after entry 2000.

## Audit checkpoint 1041–1060 — 2026-11-19

- Completed exactly 20 new unique source reviews (entries 1041–1060) from pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Every source was read fully. The Android control-theme module (2,840 lines), Android control UI module (8,506 lines), animate-tick module (2,000 lines), arrows module (1,482 lines), and block-entity item module (662 lines) were read in consecutive sections no larger than 400 lines through EOF. The benchmark (616 lines), CLI command table (908 lines) and other long files were also fully read in bounded consecutive ranges.
- All 20 entries record the 15 required fields, source SHA, upstream byte size and canonical logical line count. The pinned tree SHA/size values were checked against all 20 paths before the write.
- Key 1.20.1 findings: touch UI/theme are architecture references requiring a RustChunk-owned editor/design; Android JNI names/lifecycle are package-specific; animate-tick includes many post-1.20.1 blocks and modern biome attributes; arrows require legacy item/protocol metadata adaptation; banner and block-entity item modules use modern data components and must be rebuilt around 1.20.1 NBT; beehive and block-pattern logic are relevant but need target Java fixtures; block prediction is high-value but sequence/ack packet semantics must be verified; block transformer registry is later than 1.20.1; block-contact is a Horizon/NeoForge extension, not Forge 1.20.1 support.
- Asset acquisition code (1041) verifies pinned Bedrock sample downloads, but this is integrity checking, not redistribution permission. No Bedrock artwork is authorized for bundling by this audit.
- Horizon CLI/MCP modules (1053–1055) are development/automation tooling, not mobile client runtime dependencies. The benchmark module is methodology only; no benchmark was run.
- All 20 paths are sequential (1041–1060), unique against entries 1–1040, and SHA/byte-size match the pinned source tree. Each entry has all 15 fields exactly once; matching Index rows have been appended.
- No tests, build, benchmark, APK, Android device, Minecraft client, Forge client, multiplayer server or Aternos connection were executed.
- Next checkpoint: entries 1061–1080. Backup branches remain untouched; next backup remains due only after entry 2000.

## Audit checkpoint 1041–1060 — 2026-11-19

### Scope and source verification

- Inspected exactly 20 new unique text files in deterministic pinned-tree order, entries 1041–1060, from `starling-client/starling@5432370d573642041585c69b40fb045b27133488`.
- Recorded every source SHA and byte size from the pinned tree. Each source file was read to its canonical EOF; no partial file is marked complete.
- Long files were read in consecutive sections of at most 400 lines: 1042 (2,840 lines; eight sections), 1043 (8,506 lines; 22 sections), 1045 (2,000 lines; five sections), 1046 (1,482 lines; four sections), 1052 (616 lines; two sections), 1053 (908 lines; three sections), and 1057 (662 lines; two sections).
- Classified app/UI/CLI systems separately from Minecraft simulation and protocol. Explicitly flagged Mojang asset licensing for 1041; modern 26.x ambience/item component/block-transform schemas for 1045, 1057 and 1060; Horizon-only Android UI/JNI/CLI/MCP and benchmark behavior; and target-version validation required for arrows, combat, bee, banner, auto-jump and block prediction logic.
- No Starling source code or asset was copied into RustChunk. No Rust test, build, benchmark, APK, Android device, Minecraft launch, Forge client or multiplayer/server test was executed.

### Durable checkpoint persistence

- Added 20 complete 15-field records to `AUDIT_LEDGER_1001_1200.md` and 20 matching decision/path rows to `AUDIT_INDEX.md`.
- Re-fetched the Ledger, Index and QA log from GitHub after writes. Verify: numbers 1041–1060 occur exactly once and sequentially; 20 paths are unique and not duplicated in the previous 1,040 entries; all source paths exist in the pinned tree; all Source SHAs and byte sizes match; every entry contains each of the 15 required fields exactly once; and all 20 Index paths/primary decisions match the Ledger.
- The earlier checkpoint reconciliation at 1021–1040 remains recorded; its duplicate block was removed and the canonical entry 1039 remains the 1,805-line `crates/horizon-client/src/advancements.rs` record.
- Both existing backup branches remain untouched. No backup was created at this 20-file checkpoint. The next backup is due only after entry 2000 has been fully audited and globally verified.

### Next checkpoint

- Continue with entries **1061–1080**. Do not restart or renumber the current 200-file stage.

## Audit checkpoint 1061–1080 — 2026-11-19

- Completed exactly 20 new unique source reviews (entries 1061–1080) from pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Every source was read through EOF. Long files were read in consecutive sections of at most 400 lines: cauldron (410), clock (628), crafting (2,497), creative (1,622), debug chart (644), debug entries (506), debug keys (490), debug overlay (711), dialog screen (552) and dialog (895).
- All 20 records contain the 15 standard fields, source SHA, byte size and canonical logical line count. Source metadata was checked against the pinned upstream tree before writing.
- Key version findings: crafting and creative inventory need 1.20.1 recipe/ItemStack/NBT reconstruction; cauldron metadata must not use modern components; /data operations need target Unicode/error/path parity; debug systems are optional and 26.3 catalogues are not target data; debug-world is a useful QA preset; native breathing hooks/custom armor are not vanilla requirements; Cushion applicability remains unverified.
- Server Dialogs are not part of Java 1.20.1: official Java 1.21.6 release notes introduce the feature. Entries 1079–1080 are excluded from the target baseline; local UI ideas do not imply server-dialog compatibility.
- Android exit reports are useful optional reliability tooling, but raw trace/log content can be sensitive and should remain local until explicit user-controlled redaction/export.
- No tests, builds, benchmarks, APKs, Android-device tests, Minecraft/Forge launches, server connections or audio/GPU execution were performed.
- The 20 entries are sequential (1061–1080), unique against entries 1–1060, all 15 fields occur exactly once, and SHA/byte sizes match the pinned upstream tree. Matching Index rows and this QA checkpoint are committed.
- Next checkpoint: entries 1081–1100. Backup branches remain untouched; next backup is due only after entry 2000.

### Checkpoint 1061–1080 Index normalization

- Rechecked the newly appended Index decision column against the Ledger. The first write had included explanatory text after the primary decision labels; normalized all 20 rows so the Index carries only the approved primary decision plus optional qualifiers, while the reason remains in the final column.
- Re-read and verified the corrected Index rows for entries 1061–1080. This is a documentation-format correction; the Ledger decisions and source reviews are unchanged.

## Audit continuation reconciliation and Coverage normalization — 2026-11-19

- Reconciled the concurrent duplicate append in `AUDIT_LEDGER_1001_1200.md`: retained one canonical copy of entries 1001–1080; removed the duplicate second copy of 1041–1060; preserved the already recorded, QA-logged entries 1061–1080. The resulting ledger contains 80 unique sequential entries 1001–1080.
- Revalidated the retained entries 1001–1080 against the pinned Starling tree: all 80 paths are unique; all 80 source paths exist; all 80 SHA and byte-size pairs match; every entry contains each of the 15 required fields exactly once; all 80 Index rows match the Ledger paths and primary decisions.
- Corrected the Index/Ledger qualifier alignment for entry 1056 and the Index primary decision rows for 1043 and 1056. The 1057 source byte size is the Git tree's canonical 26,500 bytes.
- Replaced generic whole-file Coverage statements with exact consecutive ranges of at most 400 lines for long files in the retained range, including entries 1006, 1007, 1026, 1038–1040, 1042–1043, 1045–1046, 1051–1053, 1057, 1063–1064, 1068–1069, 1073–1076 and 1079–1080. All recorded ranges are contiguous and end at each source file's canonical EOF.
- Verified both immutable backup refs remain unchanged. No backup was created at 1080; the next backup remains due only after entry 2000 is fully audited and globally verified.
- Next checkpoint: **1081–1100**. Continue only after the current 1001–1080 read-back checks remain green.

## Audit checkpoint 1081–1100 — 2026-11-19

### Scope and source verification

- Inspected exactly 20 new files one at a time in deterministic pinned-tree order from `starling-client/starling@5432370d573642041585c69b40fb045b27133488`.
- Every file was read through EOF. Files over 400 logical lines were read in contiguous sections no larger than 400 lines: 1081 (623; 2), 1084 (2,927; 8), 1085 (560; 2), 1087 (5,683; 15), 1092 (447; 2), 1094 (1,428; 4), 1095 (591; 2), 1098 (23,159; 58), 1099 (805; 2) and 1100 (2,341; 6).
- The missing 3,601–4,000 window in the first pass of 1098 was fetched before completion; final coverage is contiguous from line 1 through 23,159.
- All 20 paths exist in the pinned upstream tree; each Source SHA and byte size recorded in the Ledger matches that tree.
- Findings distinguish optional Horizon desktop features from 1.20.1 behavior. End Gateway, falling-block, Furnace, Superflat and advancement behavior have target relevance; modern Item Components, later entities, modern equipment assets and the monolithic Horizon coordinator require reconstruction/exclusion.
- No source code/assets were copied into RustChunk. No tests, builds, benchmarks, APKs, Android-device runs, Minecraft/Forge launches, multiplayer joins or GPU/audio executions were performed.

### Durable checkpoint persistence

- Appended exactly 20 complete 15-field entries (1081–1100), 20 matching Index rows and this dated QA record.
- Validated sequential numbering, unique paths against entries 1–1080, exactly one occurrence of each required field, upstream SHA/byte-size matches and Ledger/Index parity.
- Re-read the three committed files from GitHub after the checkpoint commit.
- Existing backups remain untouched. No backup is due here; the next backup is after entry 2000 is fully audited and globally verified.

### Next checkpoint

- Continue with entries **1101–1120**, one file at a time.

### Checkpoint 1081–1100 Decision-format correction — 2026-11-19

- Standardized the 20 new Ledger Decision fields to the canonical bold primary-label plus em-dash rationale format.
- The Index already contained the correct primary labels; this correction aligns the Ledger representation for exact parity checks. The 15 required fields were all present; no field was missing.
- No source review, SHA, byte-size, coverage or classification changed.
