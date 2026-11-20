# RustChunk — 200-File Audit Boundary Register

- Repository: `imamirhosein/RustChunk`
- Branch: `starling-1.20.1-port-audit`
- Source: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Pinned tree: `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` (complete, not truncated)
- Stage: 1801–2000; current formally registered progress **25/200**.
- Authorization: only section 1801–1850 is authorized. Stop at 1850.

## Stage partition

| 50-file section | Checkpoint 1 | Checkpoint 2 | State |
|---|---|---|---|
| 1801–1850 | 1801–1825 — COMPLETE (corrected) | 1826–1850 — AUTHORIZED; registration pending | ACTIVE |
| 1851–1900 | 1851–1875 — LOCKED | 1876–1900 — LOCKED | NOT AUTHORIZED |
| 1901–1950 | 1901–1925 — LOCKED | 1926–1950 — LOCKED | NOT AUTHORIZED |
| 1951–2000 | 1951–1975 — LOCKED | 1976–2000 — LOCKED | NOT AUTHORIZED |

## Corrected checkpoint 1801–1825

- Canonical paths are the next 25 blobs in the pinned tree after excluding entries 1–1800.
- `crates/horizon-friends/src/lib.rs` is 1812; initial omission was corrected. This checkpoint now ends at `crates/horizon-items/src/blocks/decode.rs`.
- `crates/horizon-items/src/blocks/decode_tests.rs` is entry 1826 in the next checkpoint.
- 25 entries, 25 unique paths, 15 fields per Ledger entry; source blob SHA/size verified.
- Files read completely through EOF; long files use contiguous chunks <=400 lines.
- `crates/horizon-mod-registry/src/storage.rs` is absent from the pinned tree and is not an entry.
- Tests/examples inspected only; no tests/build/APK/device/Forge/server validation.

## Next authorized work

Complete the corrected second checkpoint 1826–1850, then close and stop at 1850. Do not inspect 1851+ without new explicit authorization.

## Rules

- One source file at a time; no parallel reads.
- Full EOF coverage; chunks <=400 logical lines, contiguous.
- Match every path, SHA and byte size to the full pinned tree.
- Exactly 15 Ledger fields; unique sequential Index paths.
- No application source changes, no main/backups modification.
- Backup only after 1001–2000 and global 1–2000 verification.
