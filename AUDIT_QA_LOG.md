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

## Cross-chat continuity record established — 2026-11-19

- Added `AUDIT_CONTINUITY.md` as the authoritative resume note for new chats and interrupted sessions.
- Updated `AUDIT_PROTOCOL.md` so every verified 20-file checkpoint must update Ledger, Index, QA Log and Continuity together in one commit, followed by GitHub read-back verification.
- Added the Continuity link to the Index and removed its duplicate 1001–1200 ledger link.
- Resume state at creation: entries 1–1100 committed; current stage 1001–1200 is 100/200 complete; next checkpoint 1101–1120.
- Pinned Starling baseline remains `5432370d573642041585c69b40fb045b27133488`. Existing backup refs remain unchanged; next backup remains due only after entry 2000 is fully audited and globally verified.
- This setup commit changes documentation only. No source files were audited, no entry numbers were added, and no backup refs were modified.

## Audit checkpoint 1101–1120 — 2026-11-19

- Inspected exactly 20 new unique files in pinned-tree order; all were read through EOF.
- Files over 400 logical lines were read consecutively in sections no larger than 400 lines: 1101 (439; 2), 1103 (2,247; 6), 1105 (1,713; 5), 1106 (632; 2), 1110 (895; 3), 1111 (706; 2), 1112 (524; 2), 1115 (926; 3), 1116 (650; 2), 1117 (722; 2), 1118 (841; 3).
- All 20 paths exist in the pinned upstream tree and all SHAs/byte sizes match. New paths are unique against prior Index entries and one another.
- Every new Ledger entry contains all 15 required fields exactly once; all Index rows match their Ledger paths and decisions.
- Findings distinguish vanilla 1.20.1 behavior from Create 6.0.10, Horizon-only APIs and later-version data/features.

### Historical duplicate path — corrected and closed

- The full Index initially contained `crates/horizon-client/src/entry.rs` at both entries 121 and 1089. The duplicate at 1089 has now been replaced without changing any other entry number.
- Replacement: `crates/horizon-client/src/game/blockctl.rs`, from pinned source commit `5432370d573642041585c69b40fb045b27133488`; source SHA `94b3853a94ceb4d905e477434553f0a84b4730e5`; 7136 bytes; 179 logical lines.
- The replacement was fully read, lines 1–179 to EOF. Pinned-tree SHA and byte size match. Findings classify it as Horizon-only scripted test tooling, not vanilla 1.20.1 gameplay; no tests or builds were run.
- This is the first eligible unindexed source path immediately after the former 1120 endpoint (`crates/horizon-client/src/game/block_items.rs`). It is now indexed at slot 1089 and must remain in the full skip set so it is not audited again as an ordinary next entry.
- Global Index validation now reports 1120 numbered rows and 1120 distinct source paths, with no duplicate paths.
- Corrective commit: `31158be7259f80ff8e5c30665c539c8fcba3e145` (parent checkpoint commit `03bf34e57055632975deb0185b10363b807a1b51`); GitHub branch-head and five-document read-back verification passed. Entry 121 remains the sole audited occurrence of `crates/horizon-client/src/entry.rs`.
- Resume the deterministic source-tree scan at `crates/horizon-client/src/game/boat_authority_fixture.rs` for the next checkpoint, after rebuilding the skip set from the corrected full Index.

### Limits

- No source was copied. No Rust tests/builds/benchmarks/APKs/device runs/Minecraft or Forge launches/server joins/GPU/audio/UI tests were performed; source tests were read only.

### Durable checkpoint

- Appended exactly 20 complete 15-field entries (1101–1120), 20 Index rows and this QA record.
- Continuity is updated to 120/200 and records the duplicate-path hold; Protocol status is updated through 1120.
- Pre-commit checks cover sequential numbering, 15 fields and pinned SHA/byte sizes. The checkpoint was committed as `03bf34e57055632975deb0185b10363b807a1b51`; all five documents were re-read from the branch and their updated content and blob SHAs were verified. The separate duplicate correction is recorded above.
- Existing backups are untouched; next backup only after entries 1001–2000 are fully audited and globally verified.

### Next action

- Checkpoint 1121–1140 is complete; continue with 1141–1160 from `crates/horizon-client/src/game/code_of_conduct.rs`.

### Checkpoint 1121–1140 — 2026-11-19

- Pinned source: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488; the recursive Git tree SHA matches the pinned commit.
- Exactly 20 new source files were reviewed completely, with no path overlap against entries 1–1120. All 20 blob SHAs and byte sizes match pinned Git-tree metadata.
- Complete line coverage: 1121 (51), 1122 (535), 1123 (458), 1124 (1634), 1125 (1164), 1126 (793), 1127 (67), 1128 (352), 1129 (242), 1130 (136), 1131 (192), 1132 (91), 1133 (290), 1134 (138), 1135 (1599), 1136 (283), 1137 (183), 1138 (183), 1139 (263), 1140 (139). Long files were read in consecutive, gap-free chunks of at most 400 lines.
- Every new Ledger entry contains exactly the 15 required fields. Index validation before commit: 1140 sequential rows, 1140 unique paths and zero duplicates.
- The corrective replacement at entry 1089 (blockctl.rs) remains in the skip set; entry.rs occurs only at entry 121.
- Decisions separate vanilla 1.20.1 vehicles/books/archaeology/campfires/chat from later Breeze/bundle behavior and Create/Horizon-only modules. Create chute support remains undetermined pending the target 1.20.1 mod list.
- Tests, builds, APKs, Forge launches, Android device runs and server joins were not performed. Embedded source tests were read only.
- No backup branch was created or modified. The next backup remains due only after entries 1001–2000 are fully committed and globally verified.

### Durable checkpoint

- Appended exactly 20 complete 15-field Ledger entries and 20 matching Index rows; QA, Continuity and Protocol were updated together.
- Continuity advances stage 1001–1200 to 140/200 (70%); next checkpoint 1141–1160 starts at crates/horizon-client/src/game/code_of_conduct.rs.
- Post-commit read-back of all five documents and blob-SHA verification are required before considering the checkpoint saved.

### Checkpoint 1141–1160 — 2026-11-19

- Pinned source: `starling-client/starling` @ 5432370d573642041585c69b40fb045b27133488; recursive Git tree SHA matched the requested source commit.
- Exactly 20 new and previously unindexed Rust files were read completely to EOF. Every source blob SHA and byte size matches pinned Git-tree metadata.
- Complete logical-line coverage: 1141 (298), 1142 (777), 1143 (1117), 1144 (291), 1145 (194), 1146 (3926), 1147 (92), 1148 (542), 1149 (906), 1150 (286), 1151 (1130), 1152 (903), 1153 (1037), 1154 (347), 1155 (256), 1156 (229), 1157 (288), 1158 (419), 1159 (108), 1160 (771). Files longer than 400 lines were read in consecutive, gap-free chunks of at most 400 lines.
- Each Ledger record contains exactly the 15 required fields. Pre-commit Index validation: 1160 sequential numbered rows, 1160 unique paths and zero duplicate paths against entries 1–1140 or within this checkpoint.
- The replacement at entry 1089 (`blockctl.rs`) remains in the skip set; `entry.rs` remains unique at entry 121.
- The command engine, execute chain, command blocks, NBT data and item-slot commands are important feature areas but require target 1.20.1 packet, registry, permission and legacy NBT rebuilding. Test harness, server pregeneration, dedicated user-list commands and later command roots are excluded from the client baseline.
- No source tests, builds, APKs, Forge launches, Android device runs, multiplayer joins or server operations were run. Embedded tests were inspected only.
- Existing backup branches were not modified. Next backup remains due only after entries 1001–2000 are complete and globally verified.

### Durable checkpoint

- Appended exactly 20 complete Ledger entries (1141–1160) with 15 fields each and 20 matching Index rows; QA, Continuity and Protocol updated in the same checkpoint operation.
- Stage 1001–1200 advances to 160/200 (80%). The next checkpoint is 1161–1180, starting at `crates/horizon-client/src/game/commands/test_locate.rs`.
- Commit and post-commit read-back are required before treating this checkpoint as saved.

### Checkpoint 1161–1180 — 2026-11-19

- Pinned source: `starling-client/starling` @ 5432370d573642041585c69b40fb045b27133488; the recursive Git tree SHA matches the pinned commit.
- Exactly 20 new and previously unindexed Rust files were reviewed completely to EOF. All 20 source blob SHAs and byte sizes match pinned Git-tree metadata.
- Complete logical-line coverage: 1161 (353), 1162 (489), 1163 (402), 1164 (1731), 1165 (213), 1166 (389), 1167 (1615), 1168 (362), 1169 (255), 1170 (485), 1171 (98), 1172 (8), 1173 (7615), 1174 (486), 1175 (605), 1176 (1371), 1177 (576), 1178 (493), 1179 (292), 1180 (187). Files longer than 400 lines were read in consecutive, gap-free chunks of at most 400 lines.
- Every new Ledger entry contains exactly the 15 required fields. Pre-commit Index validation: 1180 sequential numbered rows, 1180 unique source paths and no duplicates against entries 1–1160 or within this checkpoint.
- The historical correction at entry 1089 (`blockctl.rs`) remains in the skip set; `entry.rs` occurs only at entry 121.
- Decisions separate 1.20.1 commands, composter/consume, creatures, crossbow and debug-stick behavior from later test-instance/copper-golem/Creaking/cushion/cosmetics features and Horizon control tooling. The huge creature module is a core feature area but must be split and rewritten for the target version.
- No source tests, builds, APKs, Forge launches, Android device runs, multiplayer joins or server operations were performed. Embedded tests were inspected only.
- Existing backup branches were not changed. The next backup remains due only after entries 1001–2000 have been committed and globally verified.

### Durable checkpoint

- Appended exactly 20 complete 15-field Ledger entries (1161–1180) and 20 matching Index rows; QA, Continuity and Protocol were updated in the same checkpoint operation.
- Stage 1001–1200 advances to 180/200 (90%). Next checkpoint is 1181–1200, beginning at `crates/horizon-client/src/game/decorated_pot.rs`.
- Post-commit read-back of all five documents and blob-SHA verification are required before considering this checkpoint saved.

### Checkpoint 1181–1200 — 2026-11-19

- Pinned source: `starling-client/starling` @ 5432370d573642041585c69b40fb045b27133488; the recursive Git tree SHA matches the pinned commit. Each of the 20 new paths was absent from the previous Index and each blob SHA/byte size matches the pinned tree.
- Exactly 20 source files were read completely to EOF. Complete logical-line coverage: 1181 (159), 1182 (2227), 1183 (86), 1184 (595), 1185 (388), 1186 (118), 1187 (295), 1188 (182), 1189 (1247), 1190 (47), 1191 (1176), 1192 (2270), 1193 (4836), 1194 (107), 1195 (1178), 1196 (1788), 1197 (694), 1198 (736), 1199 (315), 1200 (24). Every file longer than 400 lines was read in consecutive, gap-free chunks of no more than 400 lines.
- Ledger validation: exactly 15 required fields for each of entries 1181–1200. Index validation: 1200 sequential numbered rows, 1200 unique source paths and zero duplicate paths across the full Index.
- This closes the 1001–1200 stage: 200/200 numbered entries are complete, committed and re-read. Entry 1089 remains correctly replaced by `blockctl.rs`; `entry.rs` occurs only at entry 121.
- Decisions distinguish decorated pots, dispenser/dropper, End/dragon, enchantment, entity abstraction and event loop features from Horizon dedicated hosting/management/telemetry, custom dialogs, Discord, diagnostics, optional dynamic lighting and test-only fixtures.
- The dragon source is 4836 logical lines/191786 bytes; the End source is 1788 logical lines (corrected count). Both were read to EOF in consecutive chunks. The earlier progress note that called End 1588 lines was an arithmetic slip; the canonical coverage and tree metadata here are correct.
- No source tests, builds, APKs, Forge launches, Android device runs, multiplayer joins or server operations were performed. Embedded tests were inspected only.
- Existing backup branches were not modified. The next backup remains due only after entries 1001–2000 are complete and globally verified; 200/200 for this stage is not a backup boundary.

### Durable checkpoint

- Appended exactly 20 complete Ledger entries (1181–1200) with 15 fields each and 20 matching Index rows. QA, Continuity and Protocol were updated together.
- Stage 1001–1200 is now **200/200 (100%) complete**. This closes the authorized 200-file stage; no files from 1201 onward were started.
- The five updated documents must be fetched again from GitHub and their blob SHAs, Index uniqueness and Ledger field counts verified before this checkpoint is considered saved.


## Protocol amendment — 2026-11-19: 100-file midpoint pause

- Governance-only change; no new Starling source file was inspected, no audit entry was added or renumbered, and no existing Ledger or Index row was altered.
- The 200-file stage remains the accounting and closure boundary. Each stage is now divided into two 100-file halves, each containing five exact 20-file checkpoints.
- Complete the first five checkpoints, commit/read-back verify each checkpoint, then stop at 100/200 and wait for a separate explicit user instruction before beginning the second half.
- After the second-half instruction, complete its five checkpoints without approval pauses between them; close the 200-file stage only after the tenth checkpoint and global QA.
- The 100-file midpoint does not split the Ledger range and does not trigger a backup. Backup cadence remains once per 1,000 newly completed entries.
- Current state remains closed at entry 1200. The next stage 1201–1400 has not started; its first half will be 1201–1300 and the second half will be 1301–1400. Neither existing backup branch was modified.


### Checkpoint 1201–1220 — 2026-11-19

- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`. Recursive Git tree was re-fetched, not truncated, and matched the pinned commit. The previous Index had 1200 sequential rows and 1200 unique paths.
- Source order was resumed after entry 1200 (`crates/horizon-client/src/game/event_loop.rs`). The correct next path is `crates/horizon-client/src/game/events.rs` (entry 1201); the stale Continuity/Protocol note naming `boat_authority_fixture.rs` was incorrect and has been corrected. The next path after entry 1220 is `crates/horizon-client/src/game/ghasts.rs`.
- Exactly 20 new source files were inspected completely to EOF in deterministic path order. Each recorded blob SHA and byte size matches the pinned tree. Canonical logical lines and byte sizes:
- 1201 `crates/horizon-client/src/game/events.rs` — 362 lines / 12482 bytes / SHA `034c79c9073147c62fec454765b9fa95676ab522`
- 1202 `crates/horizon-client/src/game/execution_role.rs` — 24 lines / 865 bytes / SHA `e8892dd471d2a0a0021441c9a8f5641a8e4e4fbd`
- 1203 `crates/horizon-client/src/game/explosion.rs` — 1416 lines / 58320 bytes / SHA `1ef3a557dab0866fbad28bc7aaedd008565cadaf`
- 1204 `crates/horizon-client/src/game/extras.rs` — 277 lines / 11167 bytes / SHA `a66d0a619ad748d7c2d5d854a4bee0720bcaec8b`
- 1205 `crates/horizon-client/src/game/falling_damage.rs` — 182 lines / 7175 bytes / SHA `1f3a2055a4fc2214eab75afb0b3a1031e8c3f7d6`
- 1206 `crates/horizon-client/src/game/falling_trees.rs` — 243 lines / 9833 bytes / SHA `a9b4eadd016f8b783f89a5201b0e97f00a5a28a9`
- 1207 `crates/horizon-client/src/game/fangs.rs` — 230 lines / 7989 bytes / SHA `b8e237004839a5f71b17ffc9f72ce30d420fa348`
- 1208 `crates/horizon-client/src/game/fans.rs` — 379 lines / 15129 bytes / SHA `b7a236e59e4d0786975078e52ed86cea9d680f28`
- 1209 `crates/horizon-client/src/game/files_offer.rs` — 282 lines / 10887 bytes / SHA `fd5fae697bf39b99437687400efddb6da9246054`
- 1210 `crates/horizon-client/src/game/fireworks.rs` — 736 lines / 29482 bytes / SHA `3e3858eb094b666c4325b3d2d5e8951521e3ab9e`
- 1211 `crates/horizon-client/src/game/fishing.rs` — 1570 lines / 61717 bytes / SHA `f9e3e61c399cbae365d04dd7dc6d8187ab3f5737`
- 1212 `crates/horizon-client/src/game/fov.rs` — 249 lines / 10185 bytes / SHA `c4e039b06f01d779ee1c9e3f4dacb9fa36d3f3f4`
- 1213 `crates/horizon-client/src/game/foxes.rs` — 166 lines / 7311 bytes / SHA `c1242cc86052b98d9eea139780afbd6277a07003`
- 1214 `crates/horizon-client/src/game/frames.rs` — 661 lines / 26519 bytes / SHA `7521dd155513be73374e9438507194bce8d7ed1e`
- 1215 `crates/horizon-client/src/game/friends.rs` — 322 lines / 12510 bytes / SHA `f2480ec76aea698444d83dec9d7f09244682f0cc`
- 1216 `crates/horizon-client/src/game/functions.rs` — 1079 lines / 43012 bytes / SHA `76305d0e4daa91a6d11e02f7bfff26e6fe363e82`
- 1217 `crates/horizon-client/src/game/funnels.rs` — 378 lines / 14510 bytes / SHA `dc6e0591d69bf133a6f4d0ed5805e742aaf14494`
- 1218 `crates/horizon-client/src/game/game_mode.rs` — 241 lines / 9347 bytes / SHA `afc2db79aacc4a81fe9ec8782e6e4933999e3360`
- 1219 `crates/horizon-client/src/game/game_tests.rs` — 1650 lines / 56315 bytes / SHA `43cdaaab472dd04803bae02fb12ee69392a7451e`
- 1220 `crates/horizon-client/src/game/gamemode_switcher.rs` — 91 lines / 3246 bytes / SHA `5d0c98048e15ec67e399e3f6fc654c35fae17786`
- Long-file coverage was consecutive and gap-free, at no more than 400 lines per fetch: `explosion.rs` 1–400, 401–600, 601–800, 801–1000, 1001–1200, 1201–1416; `fireworks.rs` 1–400, 401–736; `fishing.rs` 1–400, 401–800, 801–1200, 1201–1570; `frames.rs` 1–200, 201–400, 401–600, 601–661; `functions.rs` 1–400, 401–800, 801–1079; `game_tests.rs` 1–400, 401–800, 801–1200, 1201–1600, 1601–1650. Every shorter file was read in one complete range ending at EOF.
- Ledger validation: exactly 20 entries (1201–1220), sequential, with each of the 15 required fields present exactly once. Index validation: 1220 sequential rows, 1220 unique source paths, zero duplicate paths; new rows align with the Ledger paths and decision labels.
- The new Ledger `AUDIT_LEDGER_1201_1400.md` holds both halves of the 200-file stage. First-half progress after this checkpoint is 20/100; next checkpoint is 1221–1240. The second half 1301–1400 is not authorized yet.
- Findings separate vanilla-relevant explosions, falling damage, Evoker Fangs, fireworks, fishing, item frames, datapacks/functions and game modes from Horizon agent events, Horizon extras/storage/friends, Create-specific falling trees/fans/funnels, and test-only GameTest orchestration. Target-version data, NBT, commands and packet behavior require 1.20.1 evidence before implementation.
- Embedded source tests were inspected only; none were run. No build, APK, Forge launch, Android device run, Minecraft multiplayer join, LAN host, UPnP mapping or relay session was performed.
- Existing backup branches were not modified. Backup remains due only after entries 1001–2000 are fully committed and globally verified.

- Durable checkpoint documents updated together: new range Ledger, Index, QA Log, Continuity and Protocol. Read-back and blob SHA checks required before starting entry 1221.


### Checkpoint 1221–1240 — 2026-11-19

- Pinned source starling-client/starling @ 5432370d573642041585c69b40fb045b27133488. Recursive tree SHA matched the pin and response was not truncated. Prior Index contained 1220 sequential rows and 1220 unique paths.
- Exactly 20 new source files 1221–1240 were read fully to EOF. Blob SHAs and byte sizes match pinned tree:
- 1221 crates/horizon-client/src/game/ghasts.rs — 172 lines; 7007 bytes; SHA b8cc40e5b6498c21a5c8195e19ce15c3a98105b2.
- 1222 crates/horizon-client/src/game/glowing.rs — 53 lines; 2139 bytes; SHA 049040f97491f69e388e72320779727bd3745119.
- 1223 crates/horizon-client/src/game/golems.rs — 396 lines; 16247 bytes; SHA 60c757be8b9938c37f6bd587ad5815d1f49bdcd9.
- 1224 crates/horizon-client/src/game/growth.rs — 124 lines; 5079 bytes; SHA c52e7394e44a132ab01a264595db63ace9902c44.
- 1225 crates/horizon-client/src/game/hand.rs — 990 lines; 41204 bytes; SHA 3374297c13e4039acfc99c747938e734c7acf2dd.
- 1226 crates/horizon-client/src/game/headless.rs — 98 lines; 3260 bytes; SHA 587bd9b2e449d67410e6d9518c961519438b98e1.
- 1227 crates/horizon-client/src/game/ime.rs — 151 lines; 6539 bytes; SHA 51a270608beb251429d96849ab8ee454fe2dcfbb.
- 1228 crates/horizon-client/src/game/imports.rs — 804 lines; 28209 bytes; SHA d9ab95a0132ddf881d5ff36418f84f800b646871.
- 1229 crates/horizon-client/src/game/initial_spawn.rs — 367 lines; 14855 bytes; SHA ae6956ab40ebf14d5a8a62496a767fd0688adf5b.
- 1230 crates/horizon-client/src/game/input_mode.rs — 90 lines; 3695 bytes; SHA 018916e4d19c6fd7b512956ee5dff6021f26a7db.
- 1231 crates/horizon-client/src/game/input_trace.rs — 684 lines; 22937 bytes; SHA dec41557ad4e3968dcdc810944862c3314320cee.
- 1232 crates/horizon-client/src/game/integrated.rs — 1329 lines; 53468 bytes; SHA baedd4a77d6b62dc337158bbaf451f8689cb040b.
- 1233 crates/horizon-client/src/game/interactions.rs — 132 lines; 4886 bytes; SHA ddf1d967eb21ee7f1673434439aaadd431a7e334.
- 1234 crates/horizon-client/src/game/item_draw.rs — 681 lines; 26733 bytes; SHA 19776b1660bfe859d1737c9476c7db3a5ddc3489.
- 1235 crates/horizon-client/src/game/item_particles.rs — 130 lines; 5317 bytes; SHA 52072f7aaca615cfb8f699fb808cb751d17766a0.
- 1236 crates/horizon-client/src/game/item_use.rs — 448 lines; 19407 bytes; SHA 7b5b46006ac40b458901f34385f680eafbfacef0.
- 1237 crates/horizon-client/src/game/jigsaw_blocks.rs — 141 lines; 4770 bytes; SHA 4085e80887fb73f5bcf3095dae495a2285d13b7e.
- 1238 crates/horizon-client/src/game/jigsaw_edit.rs — 331 lines; 10795 bytes; SHA 119e3cea3187755fc3926f5f00d0fcbe793fee50.
- 1239 crates/horizon-client/src/game/jukebox.rs — 410 lines; 16555 bytes; SHA 03d2f5852cab025beb0364116b7b4a4dfd6788f5.
- 1240 crates/horizon-client/src/game/key_focus.rs — 518 lines; 19617 bytes; SHA a87464b19721a98c309e4650d642e8ad9fba2afb.
- Long-file coverage was contiguous and gap-free, each range <=400 lines: hand.rs 1–400,401–800,801–990; imports.rs 1–400,401–800,801–804; input_trace.rs 1–400,401–684; integrated.rs 1–400,401–800,801–1200,1201–1329; item_draw.rs 1–400,401–681; item_use.rs 1–400,401–448; jukebox.rs 1–200,201–400,401–410; key_focus.rs 1–400,401–518. Shorter files were fetched once to EOF.
- Ledger: 20 sequential entries, each 15 fields exactly once. Index: 1240 sequential rows, 1240 unique paths, zero duplicates; new paths/decisions align with Ledger.
- Range Ledger 1201–1400 now has 40 entries. First-half authorized progress is 40/100. Next checkpoint 1241–1260 begins at crates/horizon-client/src/game/kinetic_machines.rs. Do not start second half 1301–1400 before separate user instruction after first half.
- Happy Ghast and Copper Golem are later than 1.20.1; snow/iron golem mechanics remain relevant. Hand combat, item use/render, jukebox and item feedback are important but require 1.20.1 data/NBT/packet mapping. Spawn/integrated-server behavior is server-side; remote client obeys server authority. Create extras and Horizon platform/agent systems stay outside baseline.
- Potential hardening observations: Interaction dimensions need finite/bounded validation; importer behind Horizon UI needs separate archive/path security review; WorldRequest save-folder input needs boundary validation. These are review observations, not reproduced vulnerabilities.
- Embedded tests were inspected, not executed. No build, APK, Forge launch, Android device run, multiplayer join, LAN host, UPnP or relay action was performed.
- Existing backup refs untouched. Backup remains due only after entries 1001–2000 are committed and globally verified.



### Checkpoint 1241–1260 — 2026-11-19

- Pinned source: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`. The recursive tree was not truncated and its SHA matched the pinned commit. Before this checkpoint, the Index had 1240 sequential rows and 1240 unique paths.
- Exactly 20 new source files (1241–1260) were inspected completely to EOF in deterministic source-tree order. Every blob SHA and byte size matches the pinned source tree:
- 1241 `crates/horizon-client/src/game/kinetic_machines.rs` — 1024 logical lines; 41962 bytes; SHA `4e229a928f7f87fc44a58607230d2d61eb1175c4`.
- 1242 `crates/horizon-client/src/game/kinetics.rs` — 1442 logical lines; 61786 bytes; SHA `6b71bc90ba550a480a28568fa4edbd783030fbfe`.
- 1243 `crates/horizon-client/src/game/lan_actions.rs` — 5365 logical lines; 235964 bytes; SHA `e567c82690937f4f2cfa2da5ebd3d595c2b9ea6e`.
- 1244 `crates/horizon-client/src/game/lan_attributes.rs` — 336 logical lines; 13076 bytes; SHA `8c082d982a3f986590767e335024056a9ddc97e4`.
- 1245 `crates/horizon-client/src/game/lan_camera.rs` — 262 logical lines; 10043 bytes; SHA `8c64cf2bb8ac075c2869bfda37a4071862d157d3`.
- 1246 `crates/horizon-client/src/game/lan_chat.rs` — 242 logical lines; 9907 bytes; SHA `a1d8720b9530afd27b89e23340f85423733df8b0`.
- 1247 `crates/horizon-client/src/game/lan_crafting.rs` — 729 logical lines; 29151 bytes; SHA `9ab819fffe7c50c8d4136a86e58f29e6aad15856`.
- 1248 `crates/horizon-client/src/game/lan_entities.rs` — 853 logical lines; 34957 bytes; SHA `fc5cce5f058b622e7df773f3bae50db8260006ff`.
- 1249 `crates/horizon-client/src/game/lan_far.rs` — 79 logical lines; 3080 bytes; SHA `84bb02ad267bbb8c3df3f5ee6f12a23bfd28eb62`.
- 1250 `crates/horizon-client/src/game/lan_mobs.rs` — 545 logical lines; 20265 bytes; SHA `5b83f8e0b179aef3b6553b9ba27f45a51a010b39`.
- 1251 `crates/horizon-client/src/game/lan_mount_menu.rs` — 232 logical lines; 9211 bytes; SHA `d63ac07471237491dca2d51370162ea8c14fb762`.
- 1252 `crates/horizon-client/src/game/lan_particles.rs` — 884 logical lines; 33025 bytes; SHA `20efad5dfb17ba21a70b6dece9285a02d5000947`.
- 1253 `crates/horizon-client/src/game/lan_pvp.rs` — 490 logical lines; 19326 bytes; SHA `539b2f478e2e56fb00318e70975d3d40e9d39979`.
- 1254 `crates/horizon-client/src/game/lan_quiet.rs` — 176 logical lines; 6209 bytes; SHA `e893ed5610d690c81d941755047a4b9d94dca130`.
- 1255 `crates/horizon-client/src/game/lan_screen.rs` — 1879 logical lines; 75611 bytes; SHA `56eb7d47602dc4e717afc7c3fcd984cbf28da22f`.
- 1256 `crates/horizon-client/src/game/lan_stations.rs` — 879 logical lines; 35160 bytes; SHA `b18a64b0b56b95c010d22c8a554c4f968073627e`.
- 1257 `crates/horizon-client/src/game/lan_trading.rs` — 465 logical lines; 17384 bytes; SHA `bf94369c80ff12ca680aaaeebcee1e6816e67ee2`.
- 1258 `crates/horizon-client/src/game/lan_vehicles.rs` — 564 logical lines; 22898 bytes; SHA `d3cf0cafea56f0da1df387f2b0572009b88d7025`.
- 1259 `crates/horizon-client/src/game/lan_waypoints.rs` — 98 logical lines; 4063 bytes; SHA `4481d49fb94b42542e9701e2950db204565e24ce`.
- 1260 `crates/horizon-client/src/game/lan.rs` — 2982 logical lines; 129988 bytes; SHA `dda3e7a1da90209d6e40b40eed21018b5c90fc25`.
- Canonical line-count correction: `lan_far.rs` has 79 logical lines, as confirmed by the complete source read; earlier planning notes stating 98 were incorrect. Its SHA and 3080-byte size match the tree.
- Long-file coverage was contiguous, gap-free, and no range exceeded 400 lines: `kinetic_machines.rs` 1–400, 401–600, 601–800, 801–1024; `kinetics.rs` 1–400, 401–800, 801–1200, 1201–1400, 1401–1442; `lan_actions.rs` 1–400 through 4801–5200, then 5201–5365; `lan_entities.rs` 1–400, 401–800, 801–853; `lan_mobs.rs` 1–400, 401–545; `lan_particles.rs` 1–400, 401–800, 801–884; `lan_screen.rs` 1–400, 401–800, 801–1200, 1201–1600, 1601–1879; `lan_stations.rs` 1–400, 401–800, 801–879; `lan_trading.rs` 1–400, 401–465; `lan_vehicles.rs` 1–400, 401–564; `lan.rs` 1–400, 401–800, 801–1000, 1001–1200, 1201–1400, 1401–1600, 1601–2000, 2001–2200, 2201–2400, 2401–2600, 2601–2800 and 2801–2982. Shorter files were fully read through EOF, including `lan_attributes.rs` 1–120 and 121–336, `lan_camera.rs` 1–120 and 121–262, and `lan_chat.rs` 1–120 and 121–242.
- Ledger validation: exactly 20 new sequential entries, each with all 15 fields exactly once. Index validation: 1260 sequential rows, 1260 unique source paths and zero duplicates; new paths and decisions align with the Ledger.
- The stage Ledger `AUDIT_LEDGER_1201_1400.md` now contains 60 entries (1201–1260). The authorized first half is 60/100 complete; next checkpoint is 1261–1280 starting at `crates/horizon-client/src/game/launch_api.rs`. Do not begin 1301–1400 before the user separately authorizes the second half after 1300.
- Main version boundary: Create kinetic machinery/network code (1241–1242) is mod-specific, not vanilla; LAN host packet/action/Hub code (1243 and 1254–1260, plus parts of 1244–1253) is Horizon server infrastructure. Useful target-client references include entity spawn/metadata/relationships (1248), attribute sync (1244), secure chat (1246), camera/vehicle/particle packet families and menu synchronization (1245, 1247, 1251–1253, 1256–1258); each needs exact 1.20.1 packet/registry reconstruction.
- Newer-only systems identified include modern reach attributes, later projectile/entity/particle fields, Nautilus, mace PvP, 26.3 WorldOptions/locator behavior, Horizon `horizon:lod` and `horizon:quiet_commands`; do not treat these as vanilla 1.20.1 support.
- Embedded tests were inspected only, not executed. No build, APK, Forge launch, Android-device run, Minecraft server join, LAN host, port mapping, LOD worker or relay action was performed.
- Both existing backup branches remain untouched. Next backup remains due only after entries 1001–2000 are fully committed and globally verified.


### Checkpoint 1261–1280 — 2026-11-19

- Source pinned to starling-client/starling @ 5432370d573642041585c69b40fb045b27133488; recursive tree SHA matched and was not truncated. All 20 selected files were present, unique against the previous Index, and their Git blob SHA and byte sizes match the pinned tree.
- Files audited completely to EOF:
- 1261 `crates/horizon-client/src/game/launch_api.rs` — 271 lines, 9924 bytes, SHA `6ec77eb621fcef70836333a89d0f2a26b345cfbd`; Coverage: 1–271 (EOF).
- 1262 `crates/horizon-client/src/game/leads.rs` — 680 lines, 28659 bytes, SHA `26d4ecc1073bba00674e6e981e7dca08d5e887b5`; Coverage: 1–400, 401–680 (contiguous to EOF).
- 1263 `crates/horizon-client/src/game/level_particles.rs` — 439 lines, 18814 bytes, SHA `c1a5a97c4171a2156a9da221b5b5d2f3d5f6e784`; Coverage: 1–400, 401–439 (contiguous to EOF).
- 1264 `crates/horizon-client/src/game/levels.rs` — 731 lines, 30120 bytes, SHA `4bdbdce35c39239d0549dce17382cb2771bcb7d4`; Coverage: 1–400, 401–731 (contiguous to EOF).
- 1265 `crates/horizon-client/src/game/lightning.rs` — 736 lines, 29736 bytes, SHA `08c7ebc8f3a7b566300c423e85ed757f9fd59966`; Coverage: 1–400, 401–736 (contiguous to EOF).
- 1266 `crates/horizon-client/src/game/llamas.rs` — 159 lines, 6356 bytes, SHA `be216e2daec2744d5dbfc196b80d6e43f5fe7dda`; Coverage: 1–159 (EOF).
- 1267 `crates/horizon-client/src/game/locator_bar.rs` — 126 lines, 5035 bytes, SHA `aa69f7e7d790b020678c1f8400c5009f905e77f9`; Coverage: 1–126 (EOF).
- 1268 `crates/horizon-client/src/game/loom.rs` — 385 lines, 13356 bytes, SHA `5d8c7ae771567a8ae2b6bdacb995467e6bf56649`; Coverage: 1–385 (EOF).
- 1269 `crates/horizon-client/src/game/mace.rs` — 646 lines, 26195 bytes, SHA `46e97e950f40e1af2c500f5e867494d5a4612a14`; Coverage: 1–400, 401–646 (contiguous to EOF).
- 1270 `crates/horizon-client/src/game/machine_entities.rs` — 292 lines, 11443 bytes, SHA `eaf66d87764429413ae79d8c716f3068ea5febc0`; Coverage: 1–292 (EOF).
- 1271 `crates/horizon-client/src/game/machines.rs` — 1841 lines, 69842 bytes, SHA `878471ede9add0549e68e73a5ba78921596b350c`; Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1841 (contiguous to EOF).
- 1272 `crates/horizon-client/src/game/machines/crafter.rs` — 404 lines, 15545 bytes, SHA `20e146575a7b97f8bf2fac9824bbd416226c822b`; Coverage: 1–400, 401–404 (contiguous to EOF).
- 1273 `crates/horizon-client/src/game/machines/native_energy.rs` — 275 lines, 10748 bytes, SHA `80b11473904e3b47558fcd46d11d53241cbb58a7`; Coverage: 1–275 (EOF).
- 1274 `crates/horizon-client/src/game/machines/native_fluids.rs` — 673 lines, 26851 bytes, SHA `46c4fba0e8bfa4e347e9af97f5411cd380cde7d6`; Coverage: 1–400, 401–673 (contiguous to EOF).
- 1275 `crates/horizon-client/src/game/machines/native_items.rs` — 279 lines, 10273 bytes, SHA `84228d1727093b63a62f5d65a8e483b15b8a483a`; Coverage: 1–279 (EOF).
- 1276 `crates/horizon-client/src/game/maps.rs` — 1962 lines, 76657 bytes, SHA `39ee40df18751ee751569f799ec3960c05bf80e1`; Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1962 (contiguous to EOF).
- 1277 `crates/horizon-client/src/game/memory.rs` — 179 lines, 7363 bytes, SHA `303b5e616efa821ef5cd12dd60ef1da7b29a3b67`; Coverage: 1–179 (EOF).
- 1278 `crates/horizon-client/src/game/minecarts.rs` — 2895 lines, 122348 bytes, SHA `dd2bfe751f4cd9445ee4720c9bbeeea951f6bb52`; Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–2895 (contiguous to EOF).
- 1279 `crates/horizon-client/src/game/mob_blocks.rs` — 782 lines, 33450 bytes, SHA `6c4106c832d80207afccb66e2f6df5c78b6e78fb`; Coverage: 1–400, 401–782 (contiguous to EOF).
- 1280 `crates/horizon-client/src/game/mob_gear.rs` — 227 lines, 8737 bytes, SHA `4e7a2a0fd1f5c97c591034fd0d02fcb21270625a`; Coverage: 1–227 (EOF).
- Long-file coverage was consecutive and gap-free, each read range at most 400 lines: leads.rs (1–400, 401–680); level_particles.rs (1–400, 401–439); levels.rs (1–400, 401–731); lightning.rs (1–400, 401–736); mace.rs (1–400, 401–646); machines.rs (1–400, 401–800, 801–1200, 1201–1600, 1601–1841); crafter.rs (1–400, 401–404); native_fluids.rs (1–400, 401–673); maps.rs (1–400, 401–800, 801–1200, 1201–1600, 1601–1962); minecarts.rs (1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–2895); mob_blocks.rs (1–400, 401–782). All other files were read once from line 1 through EOF.
- Ledger validation: entries 1261–1280 are sequential and each has all 15 required fields exactly once. Index validation: 1280 sequential rows, 1280 unique paths, zero duplicates. Previous 1260 entries were preserved.
- First-half progress for stage 1201–1400: 80/100 (four of five 20-file checkpoints). Next checkpoint 1281–1300 starts at the next unindexed source path. After it is committed and globally verified, stop at 100/200 and await separate authorization for 1301–1350 under the user's new proposed 50-file cadence; do not begin 1301 yet.
- 1.20.1 findings: core target behavior includes leads, level particles, dimensions, lightning, llamas, loom, hopper/dispenser/dropper, maps, classic minecart behavior, mob equipment rendering and spawn/raid gear. Later features excluded include locator bar, mace, copper golem, crafter and experimental minecart improvements. Horizon-native energy/fluid/item APIs and Create-specific machine entities are not substitutes for Forge capabilities.
- Tests embedded in source were inspected only, not executed. No build, APK, Forge launch, Android-device test, multiplayer join, LAN host, UPnP or relay action was performed.
- Backup branches remain untouched. Backup cadence remains only after entries 1001–2000 are complete and globally verified.



### Checkpoint 1281–1300 — 2026-11-19

- Pinned upstream: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488; recursive tree matched the pin and was not truncated. Exactly 20 new paths were read completely to EOF; every path was unique against the prior 1280 Index rows, and each source Git SHA/byte size matches the pinned tree.
- Completed files:
- 1281 `crates/horizon-client/src/game/mob_index.rs` — 90 lines, 3216 bytes, SHA `052a292348f361155e100078d07b0bfcabef2a12`; 1–90 (EOF).
- 1282 `crates/horizon-client/src/game/mob_looting.rs` — 201 lines, 8461 bytes, SHA `11866dae86d99a8b17d409c5ae56425d5edf9d85`; 1–201 (EOF).
- 1283 `crates/horizon-client/src/game/mob_sounds.rs` — 136 lines, 5601 bytes, SHA `e74ee36e98b6be895c2d10626b363728c846498e`; 1–136 (EOF).
- 1284 `crates/horizon-client/src/game/mobile_controls.rs` — 2035 lines, 82783 bytes, SHA `3dbb3f39d4a598818876fe0138e97229c3fcec60`; 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2035 (contiguous to EOF).
- 1285 `crates/horizon-client/src/game/mobile_controls/mobile_platform.rs` — 86 lines, 2696 bytes, SHA `df2067cbf3bdd3d4a757a19567da95760b8b460f`; 1–86 (EOF).
- 1286 `crates/horizon-client/src/game/mod_api_host.rs` — 271 lines, 9587 bytes, SHA `39830ea73f84a0bb43cf2aba041ac2e228294af4`; 1–271 (EOF).
- 1287 `crates/horizon-client/src/game/mod_api_host_generated.rs` — 543 lines, 17834 bytes, SHA `4f2278e112fe230caea776f68420805d569045a0`; 1–400, 401–543 (contiguous to EOF).
- 1288 `crates/horizon-client/src/game/mod_assets.rs` — 203 lines, 7364 bytes, SHA `2bdc37dea236ba216723c2a73e5b0c2d062dfc8a`; 1–203 (EOF).
- 1289 `crates/horizon-client/src/game/mod_catalog.rs` — 1010 lines, 34967 bytes, SHA `856be7f82e3e0fd5247e2f63b10fecb4b0f0178f`; 1–400, 401–800, 801–1010 (contiguous to EOF).
- 1290 `crates/horizon-client/src/game/mods.rs` — 700 lines, 29203 bytes, SHA `655897741a6273125f22b91515b56009af21241a`; 1–400, 401–700 (contiguous to EOF).
- 1291 `crates/horizon-client/src/game/mount_menu.rs` — 634 lines, 23717 bytes, SHA `1b967d7db3f8222dd4b6d87bfc627d744e8f1fa0`; 1–400, 401–634 (contiguous to EOF).
- 1292 `crates/horizon-client/src/game/mounts.rs` — 1086 lines, 43042 bytes, SHA `a4dc01b1caa464b592d6ac179ae1997e8952568a`; 1–400, 401–800, 801–1086 (contiguous to EOF).
- 1293 `crates/horizon-client/src/game/moving_pistons.rs` — 167 lines, 6974 bytes, SHA `654ade5381ccd5cfd4fa585741fd2d43740c38bf`; 1–167 (EOF).
- 1294 `crates/horizon-client/src/game/music.rs` — 328 lines, 13096 bytes, SHA `4b0a0a064eeca8aab274c8ab64aab5350298631b`; 1–328 (EOF).
- 1295 `crates/horizon-client/src/game/names.rs` — 757 lines, 30307 bytes, SHA `048b855ce2bef59ea8b861c8b954549eded52c56`; 1–400, 401–757 (contiguous to EOF).
- 1296 `crates/horizon-client/src/game/narrator.rs` — 340 lines, 13726 bytes, SHA `38d2a892998c292f4421e56a2c7c9a14a387a9dc`; 1–340 (EOF).
- 1297 `crates/horizon-client/src/game/native_biome_colors.rs` — 42 lines, 1930 bytes, SHA `466fc4be3daab5a1cd11c1ca359a92dc8b9bea20`; 1–42 (EOF).
- 1298 `crates/horizon-client/src/game/native_block_entity_draw.rs` — 766 lines, 28650 bytes, SHA `582b140b97ddc56ac88156011690e23e7e6db3f9`; 1–400, 401–766 (contiguous to EOF).
- 1299 `crates/horizon-client/src/game/native_builds.rs` — 1424 lines, 56971 bytes, SHA `0e9714064b94183a1c8129b91ca8b55f3ec750a3`; 1–400, 401–800, 801–1200, 1201–1424 (contiguous to EOF).
- 1300 `crates/horizon-client/src/game/native_claimed_items.rs` — 343 lines, 12962 bytes, SHA `98aac6fc0e8ac0448cc0ad85b7c054824055d567`; 1–343 (EOF).
- Long-file coverage was contiguous and gap-free, every range no larger than 400 lines: mobile_controls.rs (1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2035); mod_api_host_generated.rs (1–400, 401–543); mod_catalog.rs (1–400, 401–800, 801–1010); mods.rs (1–400, 401–700); mount_menu.rs (1–400, 401–634); mounts.rs (1–400, 401–800, 801–1086); names.rs (1–400, 401–757); native_block_entity_draw.rs (1–400, 401–766); native_builds.rs (1–400, 401–800, 801–1200, 1201–1424). All other files were read once from line 1 through EOF.
- Ledger validation: entries 1281–1300 are sequential and each contains all 15 required fields exactly once. Index validation: 1300 sequential rows, 1300 unique paths, zero duplicates. Prior 1280 rows preserved.
- Stage 1201–1400 first 100-file section is now complete: 100/200 overall (50% of the 200-file stage), 100/100 in the first half. Stop here. Do not begin entry 1301 until the user explicitly authorizes the next 50-file section.
- Protocol updated to the new cadence: each 200-file stage has four 50-file reporting sections; each 50 contains two exact 25-file checkpoints. Stop and await approval after each 50. For the already-running 1201–1400 stage, 1201–1300 was completed under the old 20-file rule; the next section is 1301–1350 with checkpoints 1301–1325 and 1326–1350. After 1350, wait for approval before 1351–1400.
- Target implications: mob indexing is a performance optimization; Android touch/input, mobile IME and virtual mouse are core mobile UX; horse-family mounts, name tags, piston transitions and music are target-era systems to rewrite. Native mod host/generated API, native build server and custom item claims are proprietary high-privilege extension systems; do not port them as Forge substitutes. Remote native library compilation is explicitly rejected for RustChunk absent a formal, authenticated supply-chain design.
- Potential issue to verify before port: duplicate native_key_event dispatch in mobile_controls.rs; no claim is made that it has been reproduced or tested.
- Embedded tests were inspected only, not run. No build, APK, Forge launch, Android device, multiplayer server, LAN, UPnP or relay action was performed.
- Backup branches untouched. Backup cadence remains only after entries 1001–2000 are fully committed and globally verified.



## Protocol normalization — 2026-11-19

### Authoritative cadence clarification

- This is a protocol/document-consistency amendment only. It creates no new source-audit entries and does not renumber or reclassify any existing entry.
- The controlling cadence is four consecutive 50-file reporting sections per 200-file stage.
- Each 50-file section consists of two exact 25-file checkpoints. A complete 200-file stage therefore contains eight 25-file checkpoints.
- After each 25-file checkpoint, update the applicable Ledger, Index, QA Log, Continuity and Protocol together in one commit; re-fetch and verify all five documents from GitHub.
- After the second 25-file checkpoint completes a 50-file section, run the 50-file boundary checks, report and stop for explicit user authorization. Do not ask for approval between the two 25-file checkpoints inside an already authorized 50.
- The active stage 1201–1400 has 1201–1300 complete (100/200). Those historical entries were recorded under the former 20-file checkpoint cadence and remain unchanged. The next section is 1301–1350, with checkpoints 1301–1325 and 1326–1350; it remains unauthorized until the user explicitly starts it.
- At 1350, stop and wait before 1351–1400. For later stages, apply 4 × 50 / 2 × 25 from the first entry.
- Backup policy is unchanged: next backup only after entries 1001–2000 are complete, committed and globally verified. No backup at a 25-, 50-, 100- or 200-entry boundary.
- The Index currently contains 1300 sequential entries and 1300 unique paths. The active Ledger contains entries 1201–1300 with exactly 15 required fields each. No source files beyond entry 1300 were started as part of this amendment.
- No source build, unit tests, APK, Forge launch, Android-device validation or multiplayer/server test was performed as part of this policy/document amendment.


## Work pacing and concise-report amendment — 2026-11-19

- Policy-only amendment; no source entry was inspected, added, renumbered or reclassified.
- Work is now performed serially, one file or independent check at a time. Unrelated reads and validations must not be parallelized. Long files are read in sequential, contiguous ranges of at most 400 logical lines.
- Keep operations small and bounded. If a delay could make the user reasonably think work is stuck, provide one brief status update about the current item; avoid repetitive progress narration.
- Chat reports are intentionally minimal: completed range/status, verification result or blocker, and the exact next point or required authorization. Do not volunteer file-by-file details, architecture explanations, file dependency descriptions or internal change lists unless requested.
- Detailed evidence belongs in the audit documents. A 25-file checkpoint needs only a compact confirmation in chat; a 50-file reporting boundary requires a brief report and STOP for explicit authorization.
- On interruption/chat transition, resume only from the latest verified commit and Continuity record. Do not repeat completed work or infer an unverified state.


## Source-audit checkpoint 1301–1325 — 2026-11-19

- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Exactly 25 new source paths (1301–1325) were selected in pinned-tree lexical order after entry 1300; none duplicates an earlier Index path.
- Each source file was read completely through EOF. Files over 400 logical lines were read in consecutive, gap-free ranges of at most 400 lines. The recorded source SHA and byte size match the pinned tree; logical line counts and EOF coverage were checked.
- The Ledger has 25 sequential new entries, each with exactly the 15 required fields. Index rows 1301–1325 match the Ledger paths and decisions.
- Global Index validation: 1325 sequential entry numbers, 1325 unique source paths, zero duplicates.
- The 25-file checkpoint is the first half of the authorized 50-file section 1301–1350. Checkpoint 1301–1325 is committed and read-back verified; continue within the already authorized section with 1326–1350, then report and STOP at 1350.
- Findings in this range include client rendering/UX subsystems, several server-only LAN encoders/host features, and later-version extensions. All 26.3/Horizon-specific packet structures and content were kept separate from 1.20.1 compatibility claims.
- Embedded tests were inspected as source only, not executed. No build, unit tests, benchmark, APK, Forge launch, Android-device validation, Minecraft join, LAN host, UPnP or relay test was run.
- Existing backups and main branch were not modified. Next backup remains due only after entries 1001–2000 are completed and globally verified.


## Source-audit checkpoint 1326–1350 — 2026-11-19

- Pinned upstream: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`.
- Exactly 25 new unique source files (1326–1350) were inspected in pinned-tree lexical order after entry 1325. Every selected file was read fully to EOF; long-file ranges were consecutive, gap-free and no larger than 400 logical lines.
- Source SHA and byte size for all 25 entries match the pinned recursive tree. The Ledger records canonical logical line counts and EOF coverage.
- The 25 Ledger entries contain all 15 required fields exactly once; Index rows 1326–1350 align with their Ledger paths and decisions.
- The active stage 1201–1400 now has 150/200 entries complete. Reporting section 1301–1350 is closed after checkpoints 1301–1325 and 1326–1350.
- At this 50-file boundary, stop. Do not inspect entry 1351 or later until the user explicitly authorizes section 1351–1400 (checkpoints 1351–1375 and 1376–1400).
- This range is weighted toward LAN/server infrastructure. Server-only modules were not classified as client features; later-version waypoint/locator and packet structures were not treated as 1.20.1 compatibility evidence. Shared landing/movement architecture and optional in-process chunk handoff were identified separately.
- Embedded tests were inspected only, not executed. No build, unit test, benchmark, APK, Forge launch, Android-device validation, Minecraft join, LAN host, UPnP or relay test was run.
- The audit branch alone was updated. `main` and both existing backup branches remain untouched. The next backup remains due only after entries 1001–2000 are fully committed and globally verified.


## Duplicate-path correction for checkpoint 1326–1350 — 2026-11-19

- Read-back QA detected that the initially committed entry 1349, `crates/horizon-client/src/lib.rs`, duplicated the already-audited source path at entry 118. Entry 118 remains unchanged and authoritative.
- Corrected entry 1349 to `crates/horizon-client/src/locator_bar.rs`, which had been fully inspected and recorded at 1350 in the initial checkpoint commit.
- Added the next eligible, previously unaudited pinned-tree path as entry 1350: `crates/horizon-client/src/lod_net.rs`. It was read completely in ranges 1–400 and 401–555; its pinned SHA is `c9cd80ba2e22a031de4a29cd15bc33c66afcfbfa` and its byte size is 19954.
- No other entry number or source path was changed. The completed reporting section remains 1301–1350 (50/50).
- Final validation after this correction: Index entries 1–1350 are sequential and every source path is unique; Ledger entries 1201–1350 contain 150 entries, each with all 15 required fields exactly once; Index/Ledger path and decision alignment is verified.
- This was a documentation/path-uniqueness correction. Embedded tests remain inspected only, not run; no build, APK, Forge launch, Android device, server join or LAN test was performed.

## Source-audit checkpoint 1351–1375 — 2026-11-19

- Completed entries 1351–1375 inclusive (25/25) from the pinned Starling tree.
- Entry 1351 is a Git symlink (CLAUDE.md, mode 120000) whose exact 9-byte blob points to AGENTS.md; the content endpoint follows the link, so the symlink blob was checked separately to avoid miscounting target text.
- The largest file, native_hooks.rs, was read in 26 consecutive chunks covering all 10,360 lines through EOF. Other files over 400 lines were read in contiguous chunks, including their final partial chunks.
- Index read-back: entries 1–1375 are sequential and all 1,375 source paths are unique.
- Ledger read-back: entries 1201–1375 contain 175 records; all 15 required fields appear exactly once per record. Index/Ledger path and decision alignment was checked.
- This is a source review only. Embedded tests were inspected, not run; no build, unit test, benchmark, APK, Forge launch, Android-device validation, Minecraft join, or server test was performed.
- RustChunk decision remains that Horizon's native Rust mod API is not Forge 1.20.1 compatibility. Its validation and architecture patterns are references only; these files do not provide Java classloading, Forge/FML, ModLauncher, Mixin, or JVM execution.
- Only the audit branch is changed. main and backup branches remain untouched. No backup is due at this checkpoint.
- The user has authorized the full section 1351–1400. Continue with checkpoint 1376–1400, then stop at 1400.

## Source-audit checkpoint 1376–1400 — 2026-11-19

- Completed entries 1376–1400 inclusive (25/25); the authorized 50-file section 1351–1400 is now complete.
- Largest files were read in contiguous chunks through EOF: native_server.rs (1,217 lines), nether_portal.rs (1,396), pad_menu.rs (1,594), and persist.rs (1,879). All files over 400 lines include their final partial chunk.
- All 25 source SHAs match the pinned Starling tree; byte sizes and logical line counts were checked.
- Index read-back: 1–1400 sequential, 1,400 unique paths, no duplicates.
- Ledger read-back: 1–1400 contains 200 records; all 15 required fields appear exactly once per record. Entries 1376–1400 align with the Index paths.
- Compatibility decisions distinguish native-mod-only APIs, later-version-only features, optional controller/diagnostic features and core 1.20.1 behavior requiring version-specific adaptation. In particular, Nautilus and Ominous Item Spawner are excluded from the 1.20.1 baseline.
- Source review only: embedded tests were inspected, not run. No build, unit test, benchmark, APK, Forge launch, Android-device validation, Minecraft join or server test was performed.
- Only the audit branch was changed. main and backup branches remain untouched. The next backup is still governed by the 1000-entry cadence; no backup was taken here.
- STOP at entry 1400. The next section is 1401–1450 and requires explicit user authorization.

### Final ledger/index alignment recheck — 2026-11-19

- Corrected the 1376–1400 field alignment before closing the checkpoint.
- Re-read all 200 Ledger entries: each of the 15 required fields occurs exactly once.
- Re-read Index and compared every Ledger path and Decision: all 25 entries 1376–1400 match exactly.
- Index remains 1–1400 sequential with 1,400 unique paths. No source or application code was modified.

## Source-audit checkpoint 1401–1425 — 2026-11-19

- Completed entries 1401–1425 inclusive (25/25) from the pinned Starling tree at `5432370d573642041585c69b40fb045b27133488`.
- Every selected file was read completely through EOF. Files over 400 logical lines were read in consecutive, gap-free ranges no larger than 400 lines; ambiguous ranges in `potions.rs` were verified by slicing the complete pinned blob.
- All 25 source blob SHAs and byte sizes match the complete, non-truncated pinned recursive tree. Canonical logical line counts and EOF coverage are recorded in `AUDIT_LEDGER_1401_1600.md`.
- All 25 Ledger entries contain the 15 required fields exactly once. Index rows 1401–1425 match each Ledger path and Decision.
- Global Index check: 1,425 sequential entry numbers and 1,425 unique source paths; no repeated path was introduced.
- The 25-file checkpoint closes the first half of the authorized 50-file section 1401–1450. Continue with 1426–1450 without asking for intermediate permission; stop at 1450.
- Key 1.20.1 findings include remote chunk decoding/order, synchronized mob metadata, remote object lifecycle, secure chat and core gameplay systems needing version-specific rewrites. Later-only Potent Sulfur and Quick Actions were excluded; Create processing is a bundled native simulation, not Forge mod loading.
- Embedded tests were inspected as source only, not run. No build, unit test, benchmark, APK, Forge launch, Android-device validation, Minecraft join, LAN, UPnP or relay test was run.
- Only the audit branch is changed. `main` and both existing backup refs remain untouched. No backup is due; the next backup remains after entries 1001–2000 are committed and globally verified.

## Source-audit checkpoint and section closure 1426–1450 — 2026-11-19

- Completed entries 1426–1450 inclusive (25/25). This closes the authorized 50-file section 1401–1450 (50/50) within the active 200-file stage 1401–1600.
- Every source was read completely through EOF. Files over 400 logical lines were read in consecutive, gap-free chunks of at most 400 lines. Ambiguous range responses in `sculk.rs` were resolved using exact slices from its complete pinned blob.
- Source SHA, byte size and logical line count for all 25 files are recorded in `AUDIT_LEDGER_1401_1600.md` and match the pinned upstream tree.
- All 50 Ledger records for 1401–1450 contain the 15 required fields exactly once. Index rows 1426–1450 match the Ledger paths and Decision values.
- Global Index check: 1,450 sequential entries and 1,450 unique paths; no duplicate source path.
- Key RustChunk findings: the very large `remote.rs` is central to 1.20.1 multiplayer protocol mapping; chunk/projectile/entity/scoreboard/resource-pack paths must be rebuilt against the exact target protocol. Incremental save scheduling and Android lifecycle behavior are useful design references. Create native simulations, later server links/shelves and developer control commands are not substitutes for Forge compatibility.
- Embedded tests were inspected only, not run. No build, unit test, benchmark, APK, Forge launch, Android-device validation, Minecraft join, LAN, UPnP or relay test was run.
- Only the audit branch was changed. `main` and existing backup refs remain untouched. No backup was taken; the next backup remains due only after entries 1001–2000 are complete and globally verified.
- STOP at 1450. The next proposed section is 1451–1500 and requires explicit user authorization before any file in that range is inspected.


## Source-audit checkpoint 1451–1475 — 2026-11-19

- Completed entries 1451–1475 inclusive (25/25) from the pinned Starling tree at `5432370d573642041585c69b40fb045b27133488`.
- Every source was read completely through EOF. Files over 400 logical lines were re-read in consecutive, gap-free chunks of at most 400 lines; final partial chunks and EOF probes were checked.
- Pinned source SHA and byte sizes are recorded for all 25 files; canonical line counts and coverage are in the Ledger.
- All 25 new Ledger entries contain the 15 required fields exactly once. Index rows 1451–1475 match their Ledger paths and Decision values.
- Global Index validation: 1–1475 sequential with 1,475 unique paths. Active Ledger: 75 entries (1401–1475), each containing all 15 fields.
- Compatibility findings: sign editing, sniffers, spawners, water splash, containers and structure blocks need 1.20.1/legacy-data adaptation; spear, copper chest, trial-spawner and TestInstanceBlock paths are later-version content. Desktop split-screen is excluded from Android baseline. The station engine is reference only, not Forge support.
- Embedded tests were inspected as source only, not run. No build, unit test, benchmark, APK, Forge launch, Android device, Minecraft join, LAN, UPnP or relay test was run.
- Only the audit branch is changed. `main` and backups remain untouched. No backup is due; next backup remains after entries 1001–2000 are globally verified.
- The user's authorization covers section 1451–1500. Continue with 1476–1500; stop at 1500 for fresh authorization.


## Source-audit checkpoint 1476–1500 — 2026-11-19

- Completed entries 1476–1500 inclusive (25/25) from the pinned Starling tree at `5432370d573642041585c69b40fb045b27133488`.
- All sources were read completely through EOF. Files over 400 logical lines were inspected in consecutive, gap-free chunks no larger than 400 lines; final partial chunks reached EOF.
- Source SHA, byte size, canonical line count and complete coverage are recorded and verified against the non-truncated pinned recursive tree.
- Each new Ledger record has the 15 required fields exactly once. Index rows 1476–1500 match Ledger paths and decisions.
- Semantic correction to checkpoint 1451–1475: the initial write had shifted values under fields 9–14 by one position. The 15 field names were present, but values were mislabelled. Values are now realigned; descriptive relevance notes were added; Index decisions were corrected. The full 1451–1500 range was checked after correction.
- Global Index: 1–1500 sequential, 1,500 unique paths. Active Ledger: 100 entries (1401–1500), all 15 required fields each.
- Android touch/IME and touch inventory are strong mobile references but need a RustChunk platform bridge and authoritative 1.20.1 menu handling. Display entities, TNT, totem, trident, trading and tutorial have target-era counterparts requiring adaptation. Sulfur Cube, GameTest/TestInstance, Trial Spawner/Vault and associated visuals are later-version features. Transient-block rendering is a custom protocol concept and must not enter vanilla 1.20.1 protocol. UI agent control and APK auto-update need separate security design/release gating.
- Embedded tests were inspected only, not executed. No build, unit test, benchmark, APK, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was run.
- Only the audit branch was updated. `main` and backups remain untouched. No backup is due; next backup remains after entries 1001–2000 are complete and globally verified.
- Section 1451–1500 is closed. Stop at 1500 and wait for explicit authorization before 1501–1550.


## Source-audit checkpoint 1501–1525 — 2026-11-19

- Completed entries 1501–1525 inclusive (25/25) from pinned Starling tree `5432370d573642041585c69b40fb045b27133488`.
- Every source file was read through EOF. Files over 400 logical lines were read in consecutive, gap-free chunks no larger than 400 lines; final partial chunks and EOF probes are recorded in the Ledger.
- All 25 source blob SHAs and byte sizes match the non-truncated pinned recursive tree. Exact logical line counts and coverage are recorded per Ledger entry.
- Each of the 25 new Ledger records contains exactly the 15 required fields once. Index rows 1501–1525 match corresponding Ledger paths and Decision values.
- Findings: Horizon world-content manifests/restart are not a Forge loader; `horizon:lod` is custom protocol, not vanilla 1.20.1; menu/merchant/mob engines need target reconstruction. Wind Charge/Breeze, released bundle behavior and Armadillo are later than vanilla 1.20.1. Mobile touch-independent references in menu, animation, world-loading and AI are architectural only.
- Version correction: Tadpole itself is present in Minecraft 1.20.1. The later Copper Golem/Happy Ghast branches and this snapshot's redesigned Tadpole brain are excluded; target 1.20.1 Tadpole goal behavior must be retained/rebuilt, not removed.
- Embedded tests were inspected as source only, not run. No Rust build, unit test, benchmark, APK, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- Only the audit branch was updated. `main` and backup refs remain untouched. No backup is due; next backup remains after entries 1001–2000 are complete and globally verified.
- Authorization covers section 1501–1550. Continue with checkpoint 1526–1550, then stop at 1550 for fresh authorization.


## Source-audit checkpoint 1526–1550 — 2026-11-19

- Completed entries 1526–1550 inclusive (25/25) from the pinned Starling source tree `5432370d573642041585c69b40fb045b27133488`.
- All source files were read fully through EOF. Files over 400 logical lines were read in consecutive, gap-free chunks of at most 400 lines; final partial chunks and empty EOF probes are recorded in the Ledger.
- All 25 source blob SHAs and byte sizes match the non-truncated pinned recursive tree. Logical line counts and complete coverage are recorded for each entry.
- All 25 new Ledger records contain the 15 required fields exactly once. Index rows 1526–1550 match the corresponding Ledger paths and Decision values.
- Compatibility findings: vanilla 1.20.1 mob behavior such as bees, equines, dolphins, Endermen, doors, combat tracking, farm animals, loot and spawn finalization requires target-era data/RNG/NBT mapping. Breeze, Copper Golem, Creaking/Heart and later entities/reasons are excluded. CEM is optional resource-pack rendering, not Forge support. The mob-data loader must consume generated 1.20.1 exports, not source-snapshot exports.
- Embedded tests were inspected as code only; none were executed. No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was run.
- Only the audit branch was updated. `main` and backup refs remain untouched. No backup is due; the next backup remains after entries 1001–2000 are complete and globally verified.
- Reporting section 1501–1550 is closed. Stop at 1550 and wait for explicit authorization before inspecting 1551–1600.

## Source-audit checkpoint 1551–1575 — 2026-11-19

- Completed entries 1551–1575 inclusive (25/25) from pinned Starling source tree `5432370d573642041585c69b40fb045b27133488`.
- All 25 source files were read through EOF. Files over 400 logical lines were inspected in consecutive, gap-free chunks no larger than 400 lines; exact logical line counts and ranges are recorded in the Ledger.
- All 25 blob SHAs, byte sizes and paths match the pinned source tree. Ledger coverage reaches EOF for each file; the first and last chunk boundaries are contiguous.
- Each of the 25 new Ledger records has exactly the 15 required fields. Index rows 1551–1575 match Ledger paths and Decision values.
- Key 1.20.1-compatible references include Fox, Frog, Goat, Iron/Snow Golem, Guardian, Creeper/Spider/Silverfish, Illager/Illusioner, Llama/Trader Llama, Blaze/Ghast, Panda, Parrot and Phantom behavior. Every one still needs target-source/data/RNG/NBT parity checks before adoption.
- Happy Ghast, Breeze/Creaking Brain, and Nautilus are later-version content and are excluded from the 1.20.1 baseline. Illager crossbow, equipment break, gear, leash, mob physics and spawn/finalization-related paths require legacy ItemStack/NBT or other target-specific rewrites.
- `living.rs`, `nav.rs` and `path.rs` are core physics/navigation references. Their algorithms are potentially important for movement correctness and CPU cost, but no optimization or performance result is claimed; Android profiling remains unperformed. `keyframes.rs` is optional client rendering, not gameplay or Forge support.
- Embedded tests were inspected as source only; none were executed. No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- No source/application code changed. Only the audit documentation is included in this checkpoint.
- Only `starling-1.20.1-port-audit` is to be updated. `main` and backup refs remain untouched. No backup is due; the next backup remains after entries 1001–2000 are complete and globally verified.
- Checkpoint 1551–1575 is closed. Continue with 1576–1600 under the user's already-issued authorization; stop at entry 1600 for fresh authorization.

## Source-audit checkpoint 1576–1600 — 2026-11-19

- Completed entries 1576–1600 inclusive (25/25) from pinned Starling source tree `5432370d573642041585c69b40fb045b27133488`.
- All 25 source files were read through EOF. Files over 400 logical lines were read in consecutive, gap-free chunks no larger than 400 lines; exact line counts and coverage are recorded in the Ledger.
- All 25 paths, blob SHAs and byte sizes match the pinned source tree. New Ledger entries contain exactly the 15 required fields once, with semantic field alignment.
- Index now contains 1,600 sequential rows and 1,600 unique paths. The active Ledger contains 200 entries (1401–1600), each with exactly 15 fields; Index paths and Decision values align with the Ledger.
- Ledger semantic correction: final QA found entries 1551–1575 had 15 labels but the separate Purpose value was omitted, shifting semantic values from Purpose onward and leaving Evidence as `undefined`. Those 25 records have been rebuilt: the Index description is now Purpose; prior values are correctly assigned to Dependencies, Implementation, Findings, Relevance, Implications, Follow-up, Mapping and Evidence. Source SHA, size, coverage, path and Decision were preserved.
- Target-era references include Piglin/Hoglin, Polar Bear, Rabbit, Raider, Ravager, Shulker, Sniffer, Strider, tameables and Wandering Trader. SpawnRules/SpawnSelection/FinalizeSpawn and ordinary Monster Spawner are server/world-authoritative systems; they are useful for local-host parity but do not supersede a remote server.
- Later-only content excluded from 1.20.1 includes Spear/Kinetic Weapon, Sulfur Cube, Copper Golem chest transport, Trial Spawner/Trial Omen and their identifiers. Later species/model branches must also be filtered from render, sync and species registries.
- `render.rs`, `skin.rs`, `random_textures.rs` and `seen_grid.rs` are optional renderer/performance architecture references. No FPS gain, CPU/memory improvement or GPU compatibility is claimed; Android profiling and visual parity remain unperformed.
- Embedded tests were inspected as source only; none were executed. No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- No source/application code changed. Only audit documentation was updated.
- Only `starling-1.20.1-port-audit` was updated. `main` and backup refs remain untouched. No backup is due; next backup remains after entries 1001–2000 are complete and globally verified.
- Reporting section 1551–1600 is closed. Stop at entry 1600 and wait for fresh explicit authorization before 1601–1650.

## Source-audit checkpoint 1601–1625 — 2026-11-19

- Completed entries 1601–1625 inclusive (25/25) from pinned Starling source tree `5432370d573642041585c69b40fb045b27133488`.
- All 25 source files were read fully through EOF. Files over 400 logical lines were inspected in consecutive, gap-free chunks no larger than 400 lines; exact logical line counts and ranges are recorded in `AUDIT_LEDGER_1601_1800.md`.
- All 25 source paths, blob SHAs and byte sizes match the pinned recursive source tree. Exact logical line counts and full EOF coverage are recorded; Index paths and Decision values match the Ledger for 1601–1625.
- Reviewed domains include undead/villager/Warden AI, Wither/Witch, mob world queries, player movement/fluids/environment, multiplayer UI, music and Starling native mod hooks/content.
- Compatibility findings: Starling Native Mod API/generated hooks/Host/native client hooks are not Forge ModLoader/FML or Forge event APIs. Content loading has partial Forge/NeoForge biome-modifier parsing only; full Java mod loading and custom serializers remain unresolved. Friend relay and multiplayer ping paths are Starling/Horizon-specific until checked against 1.20.1.
- Player physics, fluid interaction, environment damage and villager/Warden Brain are core references, but modern attributes/events/data (including restitution/bounciness and Nautilus breathing) must be filtered or rebuilt for 1.20.1. No Android profiling or performance claim is made.
- Optional headwear/music UI and ignored integration tests are references only. Embedded tests were inspected; none were run.
- No Rust build, unit test, benchmark, APK, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay validation was performed.
- No source/application code changed; only audit documentation is included in this checkpoint.
- Only `starling-1.20.1-port-audit` is to be updated. `main` and backup refs remain untouched. No backup is due; next backup remains after entries 1001–2000 are complete and globally verified.
- Checkpoint 1601–1625 is closed. Continue with 1626–1650 under the user's existing authorization; stop at entry 1650 for fresh authorization.

## Source-audit checkpoint 1626–1650 — 2026-11-19

- Completed entries 1626–1650 inclusive (25/25) from pinned Starling source tree `5432370d573642041585c69b40fb045b27133488`.
- Exact next-25 path order was recomputed from the non-truncated recursive tree after Index entries 1–1625; all 25 paths match the next unindexed blobs in order.
- All 25 files were read through EOF. Files over 400 logical lines were read in consecutive, gap-free ranges no larger than 400 lines; exact coverage is in the Ledger.
- All 25 blob SHAs and byte sizes match the pinned source tree; logical line counts and EOF coverage are recorded. Each new Ledger record has exactly 15 semantically aligned fields; Index paths and Decision values match the Ledger.
- Findings: Starling native hooks/data are not Forge/FML; telemetry/overlay are diagnostics, not measured gains; ram_limit and random_tick_sections merit Android profiling; save.rs is explicitly 26.3 and its schema must be rebuilt for 1.20.1 while bounded I/O/LOD architecture may be studied separately.
- Projectile/raid/patrol/portal/pose behavior is version-sensitive and often world/server-authoritative; remote multiplayer must consume server packets rather than simulate a competing world.
- Embedded tests were inspected only; none were executed. No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- No source/application code changed; only audit documentation was updated.
- Only `starling-1.20.1-port-audit` is to be updated. `main` and backup refs remain untouched. No backup is due; next backup remains after entries 1001–2000 are complete and globally verified.
- Checkpoint 1626–1650 closes reporting section 1601–1650 (50/50). Stop at entry 1650 and wait for fresh explicit authorization before 1651–1700.

## Source-audit checkpoint 1651–1675 — 2026-11-20

- Completed entries 1651–1675 inclusive (25/25) from pinned Starling source tree `5432370d573642041585c69b40fb045b27133488`.
- All 25 source files were read through EOF in serial order. Files above 400 logical lines were inspected in consecutive, gap-free chunks no larger than 400 lines; exact ranges, logical line counts, SHA and byte sizes are recorded in the active Ledger.
- All 25 blob SHAs and byte sizes match the pinned source tree. Index paths and decisions match the corresponding Ledger entries; each new Ledger entry contains exactly 15 required fields.
- Important 1.20.1 references: spectator menu, sprint state, workstation logic, structure block entities/loot, display entities, tick-chunk membership, ticket levels and world creation UI. All still require exact target-version source/data/RNG/NBT/packet parity before adoption.
- Excluded from the 1.20.1 baseline: kinetic spear and Starling-specific release/account systems. Scoreboard command code, social screen, screen narrator, shadows, server pack download, and title/account integrations require target-version rewrites or optional separation.
- `tick_chunk_membership.rs` and `ticket_levels.rs` are potential core CPU optimization references; no performance improvement is claimed. Android profiling and differential tests have not been run.
- Embedded tests were inspected as source only; none were executed. No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- No source/application code changed. Only audit documentation is included in this checkpoint.
- Only `starling-1.20.1-port-audit` is to be updated. `main` and backup refs remain untouched. No backup is due; the next backup remains after entries 1001–2000 are complete and globally verified.
- Checkpoint 1651–1675 is closed. Continue with 1676–1700 under the user's already-issued authorization; stop at entry 1700 for fresh authorization.

## Source-audit checkpoint 1676–1700 — 2026-11-20

- Completed entries 1676–1700 inclusive (25/25) from pinned Starling source tree 5432370d573642041585c69b40fb045b27133488.
- All 25 files were read through EOF serially. Files above 400 lines were inspected in contiguous, gap-free chunks of at most 400 lines; exact ranges, logical line counts, SHA and byte sizes are in the Ledger.
- All blob SHAs and byte sizes match the pinned source tree. Index paths/decisions match Ledger entries; each new entry contains exactly 15 fields.
- Important 1.20.1 references include flat world settings, world spawn/respawn, vanilla generation/LOD, structure mob finalization and far-chunk meshing/lighting regression tests. LOD is speculative rendering only and must never alter authoritative gameplay chunks.
- Horizon/Starling-specific UI, account/onboarding, custom item-component protocol tests and launcher API are not direct vanilla/Forge behavior. LAN bot tests are tied to the pinned newer packet protocol and were not executed.
- Embedded tests were inspected as source only; none were run. No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- No source/application code changed. Only audit documentation is included in this checkpoint.
- Only starling-1.20.1-port-audit is updated. main and backup refs remain untouched. No backup is due; next backup remains after entries 1001–2000 are complete and globally verified.
- Section 1651–1700 is closed. Stop at entry 1700 and wait for fresh explicit authorization before 1701.

## Source-audit checkpoint 1701–1725 — 2026-11-20

- Completed entries 1701–1725 inclusive (25/25) from pinned Starling source tree 5432370d573642041585c69b40fb045b27133488.
- All 25 files were read through EOF in serial order. Files above 400 logical lines were inspected in consecutive, gap-free chunks of at most 400 lines; exact ranges, logical line counts, SHA and byte sizes are recorded in the active Ledger.
- All 25 blob SHAs and byte sizes match the pinned recursive tree. The Index now contains 1,725 sequential entries and 1,725 unique paths. The active Ledger contains 125 entries (1601–1725), each with exactly 15 fields; new Index paths and Decisions align with the Ledger.
- Command parsing is a substantial reusable architecture reference: Brigadier dispatch, selector parsing, SNBT grammar, block-state parsing, completion and differential tests. However, the pinned source includes newer item data components, command nodes, protocol parser IDs and Clock/Timeline features; these must be reconstructed from 1.20.1/Forge 47.x evidence.
- Core parity fixtures for scheduled ticks and paletted-container wire/storage behavior are relevant to RustChunk testing, but were only inspected. No embedded tests or examples were executed.
- Windows Segment Heap manifest and most hosted-game commands are not applicable to the Android baseline. No application/source code was changed.
- No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- Only starling-1.20.1-port-audit is updated. main and backup refs remain untouched. No backup is due; the next backup remains after entries 1001–2000 are complete and globally verified.
- Checkpoint 1701–1725 is complete and committed with the audit documents. Continue with 1726–1750 under the user's existing authorization; stop at 1750 for fresh authorization.

## Source-audit checkpoint 1726–1750 — 2026-11-20

- Completed entries 1726–1750 inclusive (25/25) from pinned Starling source tree 5432370d573642041585c69b40fb045b27133488.
- All 25 files were read through EOF in serial order. Files above 400 logical lines were inspected in consecutive, gap-free chunks of at most 400 lines; exact ranges, logical line counts, source SHA and byte sizes are recorded in the active Ledger.
- All 25 blob SHAs and byte sizes match the pinned recursive tree. The Index now contains 1,750 sequential entries and 1,750 unique paths. The active Ledger contains 150 entries (1601–1750), each with exactly 15 fields; Index paths and Decisions align with the Ledger for the new checkpoint.
- This half-section covers density graph compilation, sampler primitives, noise/RNG/geometry parity tools, carving, scheduled ticks, interpolation, interval arithmetic and related structures. These are architecture and test-harness references, not evidence of 1.20.1 parity.
- The biome zoom and End-island files explicitly target Java 26.3. Several density features add sparse/plane cache paths or mod-library terrain kernels; keep these distinct from authoritative 1.20.1 world generation until independently verified.
- Embedded tests and parity programs were inspected only; none were executed. No source/application code was modified.
- No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- Only starling-1.20.1-port-audit is updated. main and backup refs remain untouched. No backup is due; the next backup remains after entries 1001–2000 are complete and globally verified.
- Checkpoint 1726–1750 is complete and committed with the audit documents. Reporting section 1701–1750 is now closed (50/50). **STOP at entry 1750**; wait for explicit authorization before inspecting 1751–1800.

## Source-audit checkpoint 1751–1775 — 2026-11-20

- Completed entries 1751–1775 inclusive (25/25) from pinned Starling source tree 5432370d573642041585c69b40fb045b27133488.
- All 25 files were read through EOF in serial order. Files above 400 logical lines were inspected in consecutive, gap-free chunks of at most 400 lines; exact ranges, logical line counts, source SHA and byte sizes are recorded in the active Ledger.
- All 25 blob SHAs and byte sizes match the pinned recursive tree. The Index now contains 1,775 sequential entries and 1,775 unique paths. The active Ledger contains 175 entries (1601–1775), each with exactly 15 fields; Index paths and Decisions align with the new records.
- This checkpoint covers newer density spline/surface/noise code, world fields, geometry/collision and shape operations, Java/HotSpot math, Perlin/Simplex/normal noise and the 26.3 paletted-container implementation. The core library explicitly states it is not a playable game; terrain generation and parity coverage remain incomplete.
- Java 26.3 strategy thresholds, newer math/runtime assumptions and noise/biome semantics must not be treated as 1.20.1 behavior. The paletted container is a high-value design reference but needs a target-era rewrite.
- Embedded tests and microbenchmarks were inspected only; none were executed. No source/application code was modified.
- No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- Only starling-1.20.1-port-audit is updated. main and backup refs remain untouched. No backup is due; the next backup remains after entries 1001–2000 are complete and globally verified.
- Checkpoint 1751–1775 is complete and committed with the audit documents. Continue with 1776–1800 under the user's existing authorization; **STOP at 1800** for fresh authorization.

## Source-audit checkpoint 1776–1800 — 2026-11-20

- Completed entries 1776–1800 inclusive (25/25) from pinned Starling source tree 5432370d573642041585c69b40fb045b27133488.
- All 25 files were read through EOF in serial order. Files above 400 logical lines were inspected in consecutive, gap-free chunks of at most 400 lines; exact ranges, logical line counts, source SHA and byte sizes are recorded in the active Ledger.
- All 25 blob SHAs and byte sizes match the pinned recursive tree. The Index now contains 1,800 sequential entries and 1,800 unique paths. The active Ledger contains 200 entries (1601–1800), each with exactly 15 fields; Index paths and Decisions align with the new records.
- This checkpoint covers positional RNG, packed storage, generation/level tick scheduling, block coordinates and optional cosmetics/DualSense modules. Java 26.3 constants and RNG/tick semantics require reconstruction from the 1.20.1/Forge target.
- The cosmetics crates are an optional Horizon-specific external service/custom model format, not Minecraft protocol or Forge items. DualSense HID/CPAL support has no declared Android backend and was not tested on Android or hardware.
- Tests and hardware examples were inspected only; none were executed. No application/source code was changed.
- No Rust build, unit test, benchmark, APK build, Forge launch, Android device validation, Minecraft join, LAN, UPnP or relay test was performed.
- Only starling-1.20.1-port-audit is updated. main and backup refs remain untouched. No backup is due; the next backup remains after entries 1001–2000 are complete and globally verified.
- Checkpoint 1776–1800 is complete and committed with the audit documents. Reporting section 1751–1800 is now closed (50/50). **STOP at entry 1800**; wait for fresh explicit authorization before inspecting 1801 or later.


## Source-audit checkpoint 1801–1825 — 2026-11-20

- Completed entries 1801–1825 inclusive (25/25) from pinned Starling source tree 5432370d573642041585c69b40fb045b27133488.
- All 25 files were fetched serially and read through EOF. Files above 400 logical lines were read in consecutive, gap-free chunks no larger than 400 lines; exact coverage is recorded in the new active Ledger.
- All source blob SHAs and byte sizes were checked against the untruncated pinned recursive tree. Index now contains 1,825 sequential unique paths; the new Ledger contains 25 entries, each with exactly 15 fields; paths and Decisions align.
- Findings: DualSense crates are optional hardware-specific output; horizon-friends is a separate invite/direct/relay peer-hosting system, not the standard Aternos client transport. Router port mapping and STUN introduce consent/privacy/security concerns; invite decoding lacks an evident endpoint-count cap. HDR code is platform-specific; block-facts ingestion/shape decoding is architecturally useful but the facts schema and banner/component formats are not 1.20.1 drop-ins.
- Embedded decoder tests and examples were inspected only; none were run. No build, tests, benchmark, APK, device, Forge or server validation was performed.
- No application/source code changed. Only audit documentation is committed.
- First 50-file section 1801–1850 remains authorized; next checkpoint 1826–1850 is authorized. Do not inspect 1851 or later. Stop at 1850 for new authorization.
- Only starling-1.20.1-port-audit is updated; main and backup refs remain untouched. No backup is due before entries 1001–2000 and global verification are complete.


## Corrected checkpoint registration — entries 1801–1825 — 2026-11-20

- Recomputed source order from the complete pinned tree after excluding entries 1–1800. The initial list had omitted `crates/horizon-friends/src/lib.rs`; inserted at 1812. The corrected first 25 paths end at `crates/horizon-items/src/blocks/decode.rs` (1825).
- Rebuilt this checkpoint's Index and Ledger numbering from the canonical pinned order. The previous entry `blocks/decode_tests.rs` moves to 1826 and will be committed with the second checkpoint.
- The six checkpoint documents are corrected together. `storage.rs` under horizon-mod-registry/src does not exist in the pinned tree; the prior 404 was confirmed by the recursive tree.
- Files were read serially to EOF; long files were covered in contiguous chunks <=400 lines. The missing lib was reviewed fully (230 lines). This correction records that the first candidate enumeration was incomplete rather than concealing it.
- This repaired checkpoint contains 25 Ledger entries, exactly 15 fields each; Index numbering and paths 1801–1825 are sequential and unique.
- Tests/examples inspected only; none run. No build/device/Forge/server validation. No application source changed.


## Source-audit checkpoint 1826–1850 — 2026-11-20

- Completed corrected canonical checkpoint 1826–1850 (25/25) from the pinned Starling tree. These are exactly the next 25 blobs after checkpoint 1801–1825 when excluding entries 1–1800.
- All 25 files were fetched serially and read completely through EOF. Files over 400 lines were covered in contiguous gap-free chunks no larger than 400 lines; pinned SHA and byte size match the complete source tree.
- Active Ledger now contains 50 records 1801–1850, exactly 15 fields each. Index contains 1,850 sequential, unique paths; Ledger paths and Decisions align with Index, and all 50 paths match the pinned tree's next 50 blobs.
- Tree-reconciliation history: `horizon-friends/src/lib.rs` was inserted at 1812 after omission from the first candidate enumeration; subsequent records shifted by one. `blocks/decode_tests.rs` is 1826. The attempted `horizon-mod-registry/src/storage.rs` fetch returned 404 and the pinned tree confirms no such path; it was not indexed. Final order matches the tree exactly. Original review sequence crossed corrected checkpoint boundary, explicitly disclosed in the QA record.
- Findings: modern bundle/cooldown/enchantment/item-component schemas require 1.20.1 rewrites; loot engine has incomplete context/predicate behavior; random sequences and recipe filters require target parity; MCP and mod-registry service crates are developer/backend tooling only.
- Tests/examples inspected only, not executed. No build, benchmark, APK, Android device, Forge launch or server-join validation. No application source changed; audit documentation only.
- Section 1801–1850 is **COMPLETE/CLOSED (50/50)**. Do not inspect 1851 or later without new explicit user authorization. main and backups untouched; no backup until entries 1001–2000 and global verification are complete.


## Source-audit checkpoint 1851–1875 — 2026-11-20

- Completed canonical entries 1851–1875 (25/25) from the pinned Starling recursive tree. The exact sequence was recomputed by excluding indexed paths 1–1850 and taking the next 25 blobs.
- All 25 files were fetched serially and reviewed completely. Long files were read in contiguous chunks of at most 400 lines: submit.rs (1–1056), content.rs (1–2486), index.rs (1–695), registry.rs (1–689), native.rs (1–464), registry/tests.rs (1–408), manifest.rs (1–627), narrator/lib.rs (1–422), screen.rs (1–418), speech/linux.rs (1–206). Other text files were read to EOF; four ZIP fixtures were fetched as complete binary blobs and their pinned SHA/size checked.
- Every source blob SHA and byte size matches the untruncated pinned tree. Active Ledger now contains 75 entries (1801–1875), each with exactly 15 fields. Index contains 1,875 sequential unique paths; new paths and Decisions align with Ledger.
- Findings: Horizon's custom .hmod/.hpack/.hshader and mod API-level formats are not Forge 1.20.1 formats; content declarations require a full target-version rewrite. Registry/index trust logic offers design patterns but has URL parsing, ZIP expansion, revocation and repository governance follow-ups. Modpack extraction needs symlink-safe writes and duplicate-entry handling. The narrator subsystem has no Android backend: Linux code is cfg(target_os=linux), not Android; queues are unbounded in speech dispatch. Screen narration is a UI accessibility reference only.
- Four binary fixtures (HMOD, HPACK, HSHADER, public-key/signature/index fixtures) were inspected as source test assets; no fixture was executed. Embedded tests/examples were inspected only, not run.
- No application source changed. No build, unit test, benchmark, APK, Android device, Forge launch, Minecraft join, server or renderer validation was performed.
- This checkpoint is committed with the six required audit documents in one transaction; each will be read back individually from GitHub and cross-checked.
- Section 1851–1900 remains authorized. **Continue at 1876**; after completing 1876–1900, STOP at 1900 and request fresh authorization. Do not inspect 1901+.
- `main` and existing backup refs remain untouched. No backup is due; backup only after entries 1001–2000 and global verification 1–2000.


## Documentation integrity correction — checkpoint 1851–1875 — 2026-11-20

- Read-back QA found two inherited Ledger formatting joins that collapsed adjacent field labels: entries 1823 (`RustChunk implementation mapping` joined to `Evidence required before adoption`) and 1847 (`Purpose` joined to `Dependencies / cross-references`). Inserted the missing line breaks; both entries now have all 15 distinct required fields. No source findings or decisions changed.
- Index read-back found three inherited adjacent-row joins at entries 660, 995 and 1660 (`||` without a line break). Restored the row boundaries; numbering and path content were unchanged. The index now contains 1,875 parseable sequential rows and 1,875 unique paths.
- This is a documentation-only corrective commit following the checkpoint commit. The six audit documents are read back again after the correction.


## Source-audit checkpoint 1876–1900 — 2026-11-20

- Completed canonical entries 1876–1900 (25/25), recomputed as the next 25 blobs in the complete pinned recursive tree after excluding indexed paths 1–1875.
- All 25 source files were fetched serially and read through EOF. Files over 400 lines were inspected in contiguous chunks no larger than 400 lines: CEM expr.rs (1–997), CEM model.rs (1–1234), CIT (1–1363), CTM (1–879), CTM connect.rs (1–657), Fusion texture.rs (1–418). Other text files were read to EOF. SHA and byte sizes match the pinned tree.
- Findings: the OptiFine-format crate has high potential for optional resource-pack visuals, particularly CTM neighbour masks and CEM expression/model parsing, but must be validated against Minecraft/Forge 1.20.1. CIT currently incorporates post-1.20.5/1.21.5 component semantics and needs a 1.20.1 NBT-specific adapter. CTM property discovery/read needs file/byte limits. Filesystem traversal follows symlinks without proving file targets remain inside the resource root. Properties parser has a UTF-8 boundary panic risk when an escaped multibyte character is scanned.
- The narrator backends remain desktop-specific (Linux Flite, macOS AppKit, Windows SAPI); none is an Android implementation. CEM benchmark/example code and embedded unit tests were inspected only, not executed.
- Active Ledger now contains 100 entries (1801–1900), exactly 15 fields each. Index has 1,900 sequential unique paths; new paths/decisions match Ledger.
- No application source changed. No build/test/benchmark/APK/Android device/Forge launch/Minecraft join/server/renderer validation was performed.
- The authorized section 1851–1900 is now **CLOSED — 50/50**. **STOP at 1900**. Entries 1901–1950 remain locked pending fresh explicit user authorization.
- `main` and existing backups remain untouched. No backup is due; backup only after entries 1001–2000 and global verification 1–2000.


## Source-audit checkpoint 1901–1925 — 2026-11-20

- Completed entries 1901–1925 inclusive (25/25) from pinned Starling source tree 5432370d573642041585c69b40fb045b27133488; exact path order equals the next 25 blobs after excluding Index entries 1–1900.
- All 25 files were read serially through EOF. Files over 400 logical lines were read in consecutive, gap-free chunks no larger than 400 lines. Every recorded source blob SHA and byte size matches the complete pinned recursive tree.
- Active Ledger contains 125 records (1801–1925), each with exactly 15 fields. Index contains 1,925 sequential unique paths; new Ledger paths and Decisions align with Index.
- Findings: Horizon's port-pack is a standalone modpack conversion/checking tool for newer Horizon content formats, not a Forge 1.20.1 mod loader. Its OptiFine/ETF rules may be optional resource-pack references; its 1.21.1→26.3 carry, worldgen, density and resource transformations require target-version rewrites. JAR/network/archive paths have remaining size/redirect/hash hardening checks, while resourcepacks has stronger path/size/no-overwrite controls. Class census/check verdicts do not prove mods execute.
- Embedded tests/examples and harness references were inspected only; none were run. No build, benchmark, APK, Android device, Forge launch, Minecraft join or renderer validation was performed. No application/game source code changed; audit documents only.
- User authorization covers section 1901–1950. Checkpoint 1901–1925 is now closed; 1926–1950 remains authorized. Stop at 1950 for fresh authorization.


## Source-audit checkpoint 1926–1950 — 2026-11-20

- Completed entries 1926–1950 inclusive (25/25) from pinned Starling source tree 5432370d573642041585c69b40fb045b27133488; paths match exactly the next 25 blobs after excluding Index entries 1–1925.
- All 25 files were read serially through EOF. Long files were read in consecutive, gap-free chunks no larger than 400 logical lines. Every recorded blob SHA and byte size matches the complete pinned recursive tree.
- Active Ledger now contains 150 records (1801–1950), exactly 15 fields each. Index contains 1,950 sequential unique paths; new Ledger paths and Decisions align with Index.
- Findings: upgrade/features, density/settings and the large worldgen mapper are explicit 26.2→26.3 migration code with hard-coded/version-specific/lossy transforms. Horizon Prepare is desktop Java-exporter/asset staging infrastructure; it uses JVM tools and large heap, so it is not an Android runtime component. SHA-1 local asset imports are carefully staged, while several helper readers still lack size/depth bounds. Protocol examples are diagnostic/test harnesses for the pinned new protocol and do not prove Forge 1.20.1 compatibility. Horizon Relay is optional friend-tunnel infrastructure, not public-server client transport.
- Embedded tests/examples were inspected only; none were run. No build, benchmark, APK, Android device, Forge launch, Minecraft join or renderer validation was performed. No game/application source code changed; audit documentation only.
- Section 1901–1950 is **COMPLETE/CLOSED (50/50)**. Do not inspect 1951 or later without fresh explicit authorization. main and existing backup refs remain untouched; backup only after entries 1001–2000 and global verification 1–2000.


## Source-audit checkpoint 1951–1975 — 2026-11-20

- Completed entries 1951–1975 inclusive (25/25) from the pinned Starling source tree 5432370d573642041585c69b40fb045b27133488; these are exactly the next 25 blobs after excluding Index entries 1–1950.
- All 25 files were inspected one at a time through EOF. Files over 400 logical lines were read in consecutive, gap-free chunks no larger than 400 lines.
- Recorded source blob SHA values and byte sizes were checked against the complete recursive tree aa193ad1daa40ab307340c4a61dc6b0176bd0e23 (`truncated:false`).
- Active Ledger now contains 175 records (1801–1975), each with exactly 15 fields. Index contains 1,975 sequential unique paths; newly appended Ledger paths and Decisions align with Index.
- Findings: Horizon Relay is optional friend-session transport, not public Minecraft server transport; rate/configuration and bridge concurrency require hardening. Release-signing uses tag-bound Ed25519/SHA-256 but checksum asset paths need validation. Server-files/server-manager crates target dedicated-server behavior and include Java-properties/Gson compatibility utilities; input limits, weak entropy fallback for management secret, and unbounded cache/list concerns need follow-up. The admin panel uses textContent and same-origin requests, but backend authorization must be verified and polling can overlap.
- Tests/examples were inspected only; none were run. No build, benchmark, APK, Android device, Forge launch, Minecraft join, Aternos or renderer validation was performed. No application/game source code changed; audit documents only.
- Section 1951–2000 remains authorized by the user's current permission. Checkpoint 1951–1975 is closed; continue with 1976–2000, then stop at 2000 for explicit permission before global verification and final backup.


## Source-audit checkpoint 1976–2000 — 2026-11-20

- Completed entries 1976–2000 inclusive (25/25), closing the authorized section 1951–2000 (50/50) and the active 200-file stage 1801–2000 (200/200).
- The 25 paths are exactly the next 25 blobs after excluding Index entries 1–1975 from the complete pinned Starling recursive tree aa193ad1daa40ab307340c4a61dc6b0176bd0e23 (`truncated:false`).
- All files were inspected one at a time through EOF. Files over 400 logical lines were read in contiguous, gap-free chunks no larger than 400 lines. Source SHA and byte sizes recorded in the Ledger match the pinned tree.
- Active Ledger now contains 200 records (1801–2000), each with exactly 15 fields. Index contains 2,000 sequential unique paths; the 25 new Ledger paths and Decisions align with Index.
- Findings: the remaining `horizon-server` crate is a dedicated-server supervisor/admin subsystem explicitly targeting Minecraft Java 26.3, not a mobile Minecraft client. It includes HTTP/API/panel/management WebSocket/MCP, GS4 query and RCON services, backups, settings, OS service integration and docs. Security follow-ups include plaintext HTTP on remotely reachable API/panel binds, unbounded API/management connections and notification queues, backup symlink/ZIP extraction limits, weak entropy fallback reused for secrets, path containment and strict version parity. The management schema advertises more operations than its method dispatcher visibly handles.
- Embedded tests/examples were inspected only; none were run. No build, benchmark, APK, Android device, Forge launch, Minecraft join, Aternos or renderer validation was performed. No application/game source code changed; audit documents only.
- All sections in stage 1801–2000 are closed: 1801–1850, 1851–1900, 1901–1950 and 1951–2000 (50/50 each).
- **STOP at entry 2000.** Do not begin global verification of entries 1–2000 or create the final backup until the user explicitly authorizes the next phase. No incremental backup was made.


## Global QA 1–2000 — 2026-11-21

### Scope and results

- Re-read all nine source Ledgers and the complete Index from `starling-1.20.1-port-audit`. Confirmed exactly 2,000 Ledger records and 2,000 Index rows, numbered 1–2,000 without gaps; all 2,000 source paths are unique in each document and match by entry number.
- Verified all 2,000 paths exist in the pinned Starling recursive tree `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (`truncated:false`; 5,749 tree entries). All 2,000 recorded source SHAs match the exact pinned tree blob SHAs.
- Verified every one of the 15 required fields occurs exactly once in every record (30,000 required-field checks; zero missing or duplicate required fields).
- Compared all 2,000 Index classifications against their Ledger decisions. Corrected 97 Decision fields across entries 1201–1800 by making the already-recorded Index classification explicit at the start of the Ledger field while preserving the original explanatory text. Existing Global QA for entries 1–1000 remains the authority for the earlier prose-based decision summaries; entries 1001–1200 and 1801–2000 already had matching classification prefixes.
- Compared every Size byte count with the pinned tree. Added the exact tree byte sizes to the 25 legacy entries that had only character counts or descriptive Size text (entries 1–20 and 96–100). After correction, all 2,000 Size byte counts match the pinned tree.
- Confirmed all four 50-file sections in stage 1801–2000 are closed (1801–1850, 1851–1900, 1901–1950 and 1951–2000). QA Log, Continuity, Protocol and Boundary state were reconciled to the final 1–2000 closeout.
- Confirmed the working branch head before this QA correction was `c82d50f9457f85856b15d8c36cc6bbe8c81fb982`; no application/game source was changed. The global verification changed audit documentation only.

### Limits and backup

- This is a structural, provenance, metadata and cross-document verification of the 1–2000 audit. It is not a fresh semantic re-review of all 2,000 upstream implementations.
- Earlier source review and checkpoint records document full-file/EOF inspection and contiguous <=400-line chunks for long files. This global pass did not fetch every upstream file again solely to recalculate line counts.
- No tests, examples, builds, benchmarks, APKs, Android-device tests, Minecraft launches, Forge 1.20.1 client runs, server joins, Aternos sessions or renderer validations were executed.
- The final backup branch is `backup/audit-2000-final-2026-11-21`; it is created only after this verification commit and is read back against the verified working branch. Existing `main` and 1,000-entry backup refs remain read-only.


## Source-audit checkpoint 2001–2025 — 2026-11-21

- Completed entries 2001–2025 (25/25), the next lexically ordered source blobs after Index entries 1–2000 in the complete pinned Starling tree.
- All 25 files were inspected one at a time through EOF. Files over 400 logical lines were read in contiguous, gap-free ranges no larger than 400 lines: supervisor.rs 1–899; update.rs 1–1200; check.rs 1–607; manifest.rs 1–1175; stages.rs 1–697.
- Every entry contains all 15 required Ledger fields exactly once. Each exact source blob SHA and byte size matches the complete pinned recursive tree.
- Findings: server supervisor signal/setup race, silent reload failures and unbounded console queue; updater lacks download/unpack bounds and has non-atomic Windows switch; WebSocket RFC6455 checks/idle timeout gaps; shader checker omits ImageGather from texture-read budget; manifest omits some stage-key validation; package loader follows symlinks and checks aggregate memory too late; stage composer uses substring detection for set_surface.
- The shader API and examples are Horizon API 0.1/Naga 30.0.1 and renderer-specific, not drop-in Minecraft 1.20.1 Java shader support.
- Tests/examples were inspected only; none were run. No build, benchmark, APK, Android, Forge, Minecraft join, Aternos or renderer validation. No app source changed.
- One-time authorization covers entries 2001–2100 (two 50-file sections); checkpoint 2001–2025 is closed, 2026–2050 and 2051–2100 remain authorized. Stop at 2100; 2101+ is unauthorized.


## Checkpoint 2026–2050 — 2026-11-21

- Completed entries **2026–2050 (25/25)** in exact pinned-tree order, serially one file at a time.
- All files were read through EOF. Long-file coverage was contiguous and gap-free: `schema.rs` 1–400, 401–664; `pack_report.rs` 1–400, 401–505; `condition.rs` 1–400, 401–406; `config.rs` 1–400, 401–575; `core_shader.rs` 1–400, 401–653; `custom.rs` 1–400, 401–720; `macros.rs` 1–400, 401–489; `options.rs` 1–400, 401–580. Every other file was within one 400-line read.
- Every new Ledger record contains the 15 required fields exactly once. All paths, source SHAs and exact byte sizes match the complete pinned recursive tree. Ledger and Index numbering/path/Decision entries align for this checkpoint.
- Findings: the conditional evaluator lacks short-circuit and has macro-float/strictness gaps; render-target parser has type/range and output-index gaps; core-shader translator is 26.3-specific and validates varying locations but not full type/interpolation parity; custom expression evaluation eagerly evaluates if/ifb and can panic on in() with no args; macro expansion lacks depth/output bounds and supported required features are not consistently defined; option discovery misses transitive includes and some #if-defined switches; pack reader has unbounded ZIP/decompression and symlink-escape exposure.
- Shader API 0.1 WGSL examples/tests and the separate GLSL shaderpack crate are Horizon-specific, with 26.3/Iris 1.11.7 assumptions and Naga 30.0.1 dependencies. They are not drop-in Minecraft Java 1.20.1 or Forge shader support.
- Tests/examples were inspected only; none were executed. No build, benchmark, APK, Android, Forge, Minecraft join, Aternos or renderer validation was performed. No application source changed.
- Section **2001–2050 is closed**. The user pre-authorized **2051–2100** in this one-time 100-entry window; proceed without another request at 2050. Stop at 2100. Next backup only after 1,000 additional entries (3000) and global verification.


## Checkpoint 2051–2075 — 2026-11-21

- Completed entries **2051–2075 (25/25)** serially in pinned-tree order. Each file was read through EOF; every long file was read in contiguous, gap-free sections no longer than 400 logical lines.
- Source SHA and exact byte size for all 25 entries match the full pinned recursive tree. Ledger records contain exactly 15 required fields each; Index has 2,075 sequential unique paths and the new paths/Decision labels align.
- Main observations: shaderpack post-processing accepts conditions/resources without enough allocation limits; preprocessing lacks aggregate include/output bounds; texture decoding and raw 3D data need strict memory limits; geometry emulation is tied to triangle draw assumptions; GLSL translation is token-based and has incomplete linked-stage ABI/device-limit checks; public option APIs can substitute unvalidated text; compute ordering uses heuristics, not a formal memory model.
- The post-test pack explicitly says it is authored for Horizon. It is not evidence of third-party Minecraft 1.20.1/Forge compatibility.
- Tests/examples were inspected only; none executed. No build, benchmark, APK, Android, Forge, Minecraft join, Aternos or renderer validation was performed. No application source changed.
- Checkpoint **2051–2075 is complete**. Continue immediately through **2076–2100** under the existing one-time authorization; stop at 2100 and request permission before 2101.


## Checkpoint 2076–2100 — 2026-11-21

- Completed entries **2076–2100 (25/25)** serially in pinned-tree order. All files were read through EOF; long files were read in contiguous chunks no larger than 400 lines.
- Exact source SHA and byte size for all 25 entries match the complete pinned tree. The active Ledger now has **100 entries (2001–2100)** with exactly 15 required fields each; global Index has **2,100 sequential unique paths** and the new path/Decision rows align.
- Findings: the real-pack harness is ignored and relies on external packs; it does not validate Android/MobileGlues. The telemetry server is an independent Rust 1.90 host service, not the client. Static risks include Docker runtime running as root, proxy-header trust, synchronous SQLite under Tokio handlers, unbounded aggregate/export/history work, unchecked counter casts and telemetry report retry/idempotency races.
- Privacy boundary: documentation describes opt-in telemetry, in-memory IP rate-limit data, retention of raw hardware/report rows and protected export endpoints. This is design/documentation evidence, not proof of deployment. RustChunk must not silently add telemetry.
- Tests/examples were inspected only; none executed. No build, benchmark, APK, Android, Forge, Minecraft join, Aternos or renderer validation was performed. No application source changed.
- Section **2051–2100 is closed (50/50)**. The one-time authorization is consumed. **STOP at 2100** and request new explicit permission before 2101.


## Checkpoint 2101–2125 — 2026-11-21

- Completed entries **2101–2125 (25/25)** serially in pinned-tree order. Every source was read to EOF; oversized sources used contiguous chunks no larger than 400 lines.
- Exact SHA and byte size for every source match the complete recursive tree. Ledger holds **125 entries (2001–2125)** with exactly 15 fields each; Index has **2,125 sequential unique paths** and the new path/Decision rows align.
- Findings: the telemetry host service binds all interfaces and performs synchronous SQLite work in Tokio tasks; raw benchmark/hardware aggregation can materialize large datasets; benchmark validity checks are permissive; shared telemetry uses time-derived fallback pseudonyms if OS randomness fails; the live presence ping is explicitly outside per-kind consent; the outbox is shared by game and launcher without inter-process locking; scrub is heuristic and caller-invoked rather than enforced; hardware detection does not target Android/Mali and collects detailed system traits.
- Consent-gating tests verify disabled/withdrawn payload upload but do not cover live ping, which is separately enabled by default features and documented as no-consent. This is a policy blocker for direct adoption.
- Tests/examples inspected only; none run. No build, benchmark, APK, Android, Forge, game join, Aternos or renderer validation was performed. No app source changed.
- Continue through **2126–2150**, then **2151–2175** and **2176–2200** under the same explicit 100-file exception. **Do not stop/request permission at 2150; stop at 2200.**


## Checkpoint 2126–2150 — 2026-11-21

- Completed entries **2126–2150 (25/25)** in pinned-tree order. All 25 paths, blob SHAs and byte sizes match the complete recursive source tree at `5432370d573642041585c69b40fb045b27133488`.
- All 11 TTFs were fully fetched in Base64 and decoded to exact expected lengths. The complete SFNT directories/ranges, table checksums, whole-font checksums, metadata, cmap, GSUB/GPOS and glyph outline/component records were inspected. No binary file was treated as metadata-only.
- Three OFL font notices, one Hugeicons MIT notice and seven SVGs were fully read through EOF. SVGs were checked for active, embedded and external resource references.
- Typography: Bricolage faces have 703 glyphs/661 mappings; Geist 975/728 (Geist Mono 1,159/889); Pixelify 578/577. No family has Arabic/Persian cmap coverage. Geist Mono fixed-pitch flag and hmtx advances confirm 600-unit spacing glyphs and zero-advance combining marks.
- Licensing: fonts are SIL OFL 1.1; retain copyright/license, do not sell the fonts alone, and keep derivatives under OFL. Bricolage's merged Cyrillic contribution requires Geist attribution. Hugeicons SVG subset is MIT; retain its copyright/permission notice.
- SVGs are 24×24 currentColor vectors without active/external references. Android renderer compatibility is not proven and requires conversion/device validation.
- Ledger has **150 entries (2001–2150)** with exactly 15 fields each; Index has **2,150 sequential unique paths**, and the newly added decisions align.
- Tests/examples inspected only, not executed. No app source change, build, APK, Android device, renderer, Minecraft, Forge, server or Aternos validation.
- Section **2101–2150 is closed (50/50)**. Continue 2151–2175 and 2176–2200 under existing one-time authorization. **Do not stop/ask at 2150; stop only at 2200.**


## Checkpoint 2151–2175 — 2026-11-21

- Completed entries **2151–2175 (25/25)** in exact pinned-tree order; every file was inspected serially and completely through EOF as UTF-8. Exact blob SHAs and byte lengths match the complete recursive tree.
- The entire set comprises 25 self-contained one-line SVGs in the Hugeicons glass icon directory. Root/viewBox, dimensions, fill/stroke semantics, all path counts and tags were read; no script, foreignObject, iframe, image/use embedding, href/xlink or CSS URL external reference was present.
- Visual symbols: copy, four-panel dashboard, delete, Discord filled brand mark, download, earth, favourite, folder, game controller, globe, image, information, chain link, minus, horizontal/vertical overflow, parcel, edit, play, plus, puzzle, refresh, search, adjustment sliders and stop.
- Two integration variations were noted: Discord uses fill=currentColor and omits explicit width/height; dashboard path caps are square. Do not assume every icon shares identical SVG attributes.
- The adjacent Hugeicons MIT notice (entry 2143) remains part of the distribution obligation. SVG source inspection does not prove Android VectorDrawable support or renderer output; conversion and device validation are still required.
- Ledger now has **175 records (2001–2175)**, each with exactly 15 fields; Index has **2,175 sequential unique paths**, new Decisions aligned. No app source changed; no tests/build/APK/device/renderer/Minecraft/Forge/Aternos validation.
- Checkpoint **2151–2175 is complete**. The user's one-time authorization still covers 2176–2200; continue immediately and **STOP only at 2200**, not at 2175.


## Checkpoint 2176–2200 — 2026-11-21 — FINAL AUTHORIZED CHECKPOINT

- Completed entries **2176–2200 (25/25)** in exact raw Git tree order. All 25 paths, blob SHAs and exact sizes match the complete pinned tree.
- Seven Hugeicons Glass SVGs were read completely through EOF. The Horizon PNG was fully decoded (797 bytes): signature, 64×64 RGBA IHDR, IHDR/bKGD/IDAT/IEND sequence, four CRC32s and IEND-at-EOF were validated; a preview was inspected.
- Pixelarticons' separate MIT notice was fully read (1,072 bytes/21 lines; Copyright © 2019 Gerrit Halfmann). Sixteen Pixelarticons SVG files were completely read and use this distinct notice, not Hugeicons' MIT notice.
- All SVGs are self-contained with no script, embedded image/use or external URL references. Glass uses stroked currentColor line art; Pixel uses filled pixel-grid currentColor shapes. Android SVG renderer/conversion was not tested.
- No dedicated license for the Horizon PNG was found in the icon directory; ownership and redistribution permission need confirmation before use.
- Source-order correction incident: seven assets beyond the authorized boundary (entries **2201–2207**, `home.svg` through `package.svg`) had been fetched unintentionally. They are excluded from Ledger, Index and completion counts; no adoption decision is made for them.
- Ledger has **200 entries (2001–2200)**, each with exactly 15 fields. Index has **2,200 sequential unique paths** and decisions align. All four 25-file checkpoints in 2101–2200 are complete.
- No application source changed; no tests/builds/APKs/benchmarks/target-device/renderer/Minecraft/Forge/server/Aternos validation.
- The user's one-time authorization **2101–2200 is exhausted**. Stop before 2201 and request fresh explicit permission.


## Audit checkpoint 2201–2225 — 2026-11-21

- Completed entries **2201–2225 (25/25)** in deterministic lexical path order after excluding all existing Index paths. Each was read individually through EOF; exact source SHA and byte size match the complete pinned recursive tree.
- Entries 2201–2218 are 18 static Pixelarticons-style SVG glyphs; the distinct MIT notice remains entry 2184. Entry 2214 (star.svg) is explicitly marked in the crate registry as custom Horizon artwork, not Pixelarticons; its asset-license provenance remains unresolved.
- Entries 2219–2222 cover Glass/OKLab, the central UI token/asset registry, Night/Dawn and generated CSS. Public float parameters need validation; embedded tests were not executed.
- Entry 2223 is a platform dependency manifest. Entry 2224 is a Windows-only GDI boot painter; Entry 2225 is an unsafe profiling allocator with cross-thread guard, checked-arithmetic, Android thread-name and self-measurement concerns.
- The seven previously fetched out-of-range assets 2201–2207 have now been re-fetched and reviewed as part of the newly authorized 2201–2250 range. They are counted only now.
- All 25 Ledger entries contain the exact 15 required fields; the global Index has 2,225 sequential unique paths and the new Ledger has 225 records (2001–2225). No application source changed.
- No tests/examples were executed; no build, benchmark, APK, Android device, renderer, Minecraft, Forge, multiplayer server or Aternos validation was performed.
- The 200-file stage **2201–2400** is 25/200 complete. Current 50-file section **2201–2250** is 25/50 complete; checkpoint **2226–2250** remains authorized. Continue without asking at 2225 and **STOP at 2250**.


## Audit checkpoint 2226–2250 — 2026-11-21

- Completed entries **2226–2250 (25/25)** in deterministic lexical path order after excluding the full existing Index set. Every source was read individually through EOF; long files were read in consecutive, gap-free chunks of at most 400 lines. SHA and exact byte size match the complete pinned tree.
- Thread infrastructure findings: pool construction can report one worker after every spawn failed; registration removal can block behind a slow source callback; memory watcher Once startup cannot recover from initial thread-spawn failure; u64::MAX shares the no-limit sentinel; OS signal/priority/path handling needs per-platform verification.
- `chunkgen_bench.rs` has a truncated-output verifier flaw: expected candidates left active at EOF may still be treated as matched. Do not rely on benchmark pass results before fixing this.
- Differential fixtures cover effects, attributes, vertical anchors, basic feature placement, biome chunk/generation/source/temperature/placement, collision scanning, block entities and block landing. They are mock/test transports and are not production implementations. Risks include invalid equal-key comparator ordering, recursive nested placement without a depth guard, f64 normalization losing large integer precision and many trusted-fixture unwrap/index operations.
- All 25 entries contain exactly the 15 required Ledger fields. The new Ledger has **250 entries (2001–2250)**; global Index has **2,250 sequential unique paths**, no duplicates, and matching path/Decision rows.
- Tests/examples were inspected only; none were run. No app source changed; no build/benchmark/APK/device/renderer/Minecraft/Forge/server/Aternos validation.
- Section **2201–2250 is closed at 50/50**. Next range **2251–2300** requires fresh explicit authorization.


## Reconciliation correction — 2201–2207 registration and historical authorization snapshots — 2026-11-21

- At the 2176–2200 checkpoint, entries 2201–2207 had been fetched prematurely and were correctly excluded from that checkpoint's counts. That was a historical snapshot, not their final audit disposition.
- After the user authorized 2201–2250, all seven files were re-fetched one at a time, read through EOF, and added to Ledger/Index as entries 2201–2207. Their exact SHA and byte sizes were rechecked against the pinned recursive tree; all seven rows are unique and sequential. They are no longer excluded or pending.
- The old Protocol sections that say STOP at 2100 or 2200 describe dated states at those points. They are now explicitly labeled historical; current state is Section 12 and the new authorization is Section 24.
- The read-back selection script previously used a flawed numeric pattern and selected only 4 of 25 new entries. The corrected numeric interval selected all 25 entries 2226–2250; all 25 were checked for exactly 15 fields, Ledger/Index decision alignment, source SHA and byte size. The final corrected read-back passed.
- Current authorization: **2251–2300**, with checkpoints 2251–2275 and 2276–2300. Continue without asking at 2275; stop at 2300.


## Index chronology and tree-order reconciliation — 2026-11-21

- 70 adjacent transitions relative to the pinned GitHub recursive-tree API order were found among the 2,250 stable Index rows: 69 occur in entries 1–2000, and one occurs at entry 2033. No additional raw-tree-position regressions occur from entry 2034 through 2250. This is a historical ordering divergence, not a missing-file, duplicate-path or source-provenance defect: all 2,250 Index paths are unique and present in the pinned tree. The audit IDs are stable historical chronology and must not be renumbered because Ledger, QA and cross-document references depend on them. Do not describe the whole Index as a literal raw recursive-tree traversal.
- The latest indexed path is `crates/horizon-world/examples/probe/block_landing_fixture.rs`. A full-set check found **zero unindexed source paths lexically before or equal to it**; all **2,718** remaining unindexed blobs are after this boundary.
- From entry **2251** onward, select the next unindexed paths in deterministic lexical path order strictly after the latest indexed path, excluding the complete Index. Recheck uniqueness, pinned-tree membership, source SHA and byte size before each checkpoint. Preserve stable entry numbering and the 25/50/200 cadence.


## Audit checkpoint 2251–2275 — 2026-11-21

- Completed entries **2251–2275 (25/25)** serially in deterministic lexical order strictly after the last indexed path, excluding all 2,250 existing Index paths. The complete pinned recursive tree is untruncated; exact paths, SHA and byte sizes match.
- Source mix: block properties/support/survival, block push/update, block-state codec, chunk stage/status, terrain bulk access, carvers/carving masks, climbing/collision, combat/blocking rules, component durability/patches and consumable/effect fixtures.
- These are mock differential-test transports, not shipped runtime features. Important blind spots: fixed registry cardinalities (block support), mock assumptions (always-loaded chunk/no entities in block updates), missing enchantment effects (blocking durability), unbounded/unchecked fixture schemas, empty-array modulo/index panics, unchecked i32/u8 casts, unbounded carving loops and collision parser assumptions.
- `block_update_fixture.rs` (474), `collision_context_fixture.rs` (450) and `consumable_fixture.rs` (413) were read to EOF in consecutive gap-free chunks each no greater than 400 lines.
- All 25 Ledger entries contain exactly 15 required fields. Ledger now contains **275 records (2001–2275)**; global Index contains **2,275 sequential unique paths** and matching Decisions.
- Tests/examples inspected only; none run. No application source changes and no build/benchmark/APK/device/renderer/Minecraft/Forge/server/Aternos validation.
- The next checkpoint **2276–2300** remains covered by the same explicit authorization. Do not stop/request permission at 2275; **STOP at 2300**.


## Audit checkpoint 2276–2300 — 2026-11-21

- Completed entries **2276–2300 (25/25)** after the existing 2,275 Index entries, in deterministic lexical path order. Every file was inspected serially through EOF; long files used contiguous chunks no larger than 400 lines. Exact source SHA and size match the pinned recursive tree.
- Effect notification and sync fixtures repeat the JavaKey tie-break `<=` defect: equal identities can compare as Less. Do not trust their comparator/corpus as an ordering oracle until corrected.
- Damage reduction stubs armor effectiveness and enchantment protection; dynamic support, fluids, movement and entity collision use scripted/mock worlds. `entity_fluid_fixture` assumes non-empty pushed samples; extra-item inventory is fixed at 43 slots; many fixtures trust IDs/numeric fields.
- Decoration/feature-region coverage is useful for 1.20.1 regression scenarios (retrogen, write zones, heightmaps, chunk ticks and seeded placement), but uses Horizon DATA_VERSION data and resident synthetic chunks. It does not prove async chunk streaming, rendering or Forge compatibility.
- All 25 entries have exactly 15 fields; Ledger now has **300 records (2001–2300)**; Index has **2,300 sequential unique paths** with matching Decisions.
- Tests/examples were inspected only; none were run. No application source changed; no build, benchmark, APK, device, renderer, Minecraft, Forge, server or Aternos validation.
- Section **2251–2300 is closed (50/50)**. Next range **2301–2350** requires fresh explicit authorization.


## Corrective review — Decision taxonomy and next authorization — 2026-11-21

- Read-back found Decisions 2276–2300 used reference as a standalone label, conflicting with Protocol: reference is a qualifier and requires an approved primary Decision.
- Corrected all 25 Ledger and Index Decisions to adaptable; reference. Fixture-only/no-shipment limits remain in supporting Findings and RustChunk relevance.
- decoration_region_fixture.rs is 303 canonical lines and was re-read in contiguous ranges 1–200 and 201–303; the previous 493-line estimate is corrected.
- Section 2251–2300 is closed (50/50). The user has authorized 2301–2350; both checkpoints 2301–2325 and 2326–2350 are covered. Do not pause at 2325. STOP at 2350 and request permission before 2351.
- All 2,300 Index paths remain sequential/unique; entries 2276–2300 paths, source SHAs and sizes match the pinned tree. No tests/examples executed; no application source changed.


## Corrective review — split the 2001–2200 and 2201–2400 Ledgers — 2026-11-21

- A document-boundary error was found: the closed `AUDIT_LEDGER_2001_2200.md` physically contained entries 2001–2300 (300 records), so entries 2201–2300 had been appended to the prior stage's Ledger.
- Split the file at the exact `### 2201.` boundary. `AUDIT_LEDGER_2001_2200.md` now contains only entries 2001–2200 (200 records); the new `AUDIT_LEDGER_2201_2400.md` contains entries 2201–2300 (100 records) for the active 2201–2400 stage.
- All 300 existing entry blocks were transferred without changing entry numbers, source paths, source SHAs, byte sizes, coverage, findings or Decision values. No Index rows were renumbered or rewritten.
- Structural read-back checks: 200 + 100 records; sequential ranges with no gaps; all 300 records have the 15 required fields exactly once; 300 unique source paths; every Ledger path and primary Decision aligns with the matching Index row.
- Global Index remains 2,300 sequential unique paths. Reporting sections 2201–2250 and 2251–2300 remain closed; user authorization for 2301–2350 remains active, with STOP at 2350.
- Historical QA notes reporting 300 records in the old Ledger describe the pre-correction physical layout and are retained as historical snapshots. No upstream source or RustChunk application code changed. Tests/examples were not run; no build, APK, device, Minecraft, Forge, server or Aternos validation was performed.