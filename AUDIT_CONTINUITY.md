# RustChunk Starling audit — current handover

## Goal
Use Starling's native Rust engine architecture as a basis for a playable **Minecraft Java 1.20.1 Vanilla single-player client**, first reaching menu/world, then prove movement, block interaction, save/reopen, and measure Android performance. Forge/mods/multiplayer come later.

## Branch and source
- Repo: `imamirhosein/RustChunk`
- Working branch: `starling-1.20.1-port-audit`
- Current branch base after cleanup: `11ce45ab531023c136b4d6a259813c5a37ce1a04`
- Fixed re-audit source: `imamirhosein/starling @ ff303502a4f7982551a0e0d3622d9268efe9e829`
- Historical reviewed source: `5432370d573642041585c69b40fb045b27133488`
- Prior audit entries 1–3700 are preserved and numbered. Start re-audit at entry 1; do not renumber.
- Four v0.8 backup refs and `main` are read-only and must remain untouched.

## Ready-to-start state
- Cleanup of audit administration is in progress; source-file re-audit has **not started**.
- Before entry 1: verify fixed source commit, recursive tree completeness, old Index/ledger consistency, then compare old path/SHA against new pinned tree.
- Same path + same SHA: carry forward prior finding without rereading.
- Changed path/SHA: reread the complete file and revise the existing entry.
- Removed path: preserve old entry and mark removed from new source.
- Additions: inventory and queue; do not insert/renumber into 1–3700.
- Re-audit old entries serially, groups of 50; report and stop at each boundary for user approval.

## Audit records
- Protocol: `AUDIT_PROTOCOL.md` (current rules).
- Index: `AUDIT_INDEX.md` (historic numbering/path baseline).
- Ledgers: `AUDIT_LEDGER_*.md` (previous reviews; keep entries).
- QA: `AUDIT_QA_LOG.md` (append-only historical evidence).
- Media: `AUDIT_MEDIA_ARCHIVE.md` (historical archive; backfill incomplete).
- The redundant boundary register has been removed. No new trackers/manifests are to be created.

## Verified limitations
Previous audit was static source/media review only. No complete Minecraft 1.20.1 Vanilla game port, APK, device launch or Forge compatibility has been established by that audit. Do not conflate Starling 26.3 baseline builds or data exporters with a playable 1.20.1 client.

## Next action
Finish/read-back verify the audit-document cleanup. Then, as the first audit action, verify source tree and reconcile entries 1–50 against the fixed commit. Do not start source entry 1 until this readiness gate is complete.
