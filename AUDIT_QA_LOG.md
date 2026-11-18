# RustChunk Starling Audit QA Log

This log records quality-control reviews of existing Starling audit entries. QA reviews do not create new source-audit entries, renumber files, or change the pinned upstream snapshot.

## Frozen baseline

- Repository: `imamirhosein/RustChunk`
- Pinned upstream source: `starling-client/starling`
- Pinned upstream ref: `starling`
- Pinned upstream commit: `5432370d573642041585c69b40fb045b27133488`
- Baseline RustChunk commit: `9b3b697e3c80019025d857a5e3dd0b6e20d0a5c4`
- Frozen archive branch: `archive/audit-1000-frozen-2026-11-18`
- Original backup branch: `backup/audit-1000-unique-2026-11-18`
- Both branches were verified at the baseline commit before QA. The archive branch was created from that exact commit and has not been modified by QA.
- GitHub reports `protected: false` for both branches. They are treated as read-only by this audit procedure; server-enforced branch protection was not enabled by the available repository action.

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
