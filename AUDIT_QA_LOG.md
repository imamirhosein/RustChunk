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
