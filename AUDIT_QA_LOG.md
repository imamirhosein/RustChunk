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
