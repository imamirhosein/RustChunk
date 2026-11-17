# RustChunk Audit Ledger

## Audit rules

- A file is **complete** only after its entire current contents have been fetched and reviewed.
- Record the exact repository, ref/commit, path, total line count, and reviewed line ranges.
- Large files must be reviewed in consecutive, non-overlapping ranges with no gaps.
- A fetch, grep result, excerpt, or partial range is not a completed review.
- Do not move past an unfinished file in a sequential audit batch.
- Findings are classified as: confirmed, potential/unverified, or no issue found in reviewed scope.
- Re-review a file if its blob SHA changes.
- Do not claim tests/builds passed unless their result is directly verified.

## Current audit baseline

- Repository: `imamirhosein/RustChunk`
- Branch: `starling-1.20.1-port-audit`
- Branch head at ledger creation: `cf3b951b97defc33cdedffd48573df3ebfa68370`
- Upstream submodule pin previously established: `starling-client/starling`, commit `5432370d573642041585c69b40fb045b27133488`.
- This ledger is a new source-of-truth going forward. Historical summaries below are not upgraded to verified status unless exact evidence is available.

## Verified complete files

These two files were explicitly confirmed in the prior audit conversation as fully fetched and reviewed. Their exact historical blob SHAs and line counts were not preserved in the available audit record, so those fields remain unknown and must not be fabricated.

| File | Reviewed scope | Status | Notes |
|---|---:|---|---|
| `crates/horizon-client/src/game/input_trace.rs` | Entire file (reported 685 lines) | Complete — historical confirmation | Blob SHA not recorded |
| `crates/horizon-client/src/game/item_use.rs` | Entire file (reported 449 lines) | Complete — historical confirmation | Blob SHA not recorded |

## Previously reported historical coverage — not yet independently reconstructed

The following are historical summary claims, not a per-file verified ledger. Do not count them as individually proven complete until file paths, exact revisions, and complete reviewed ranges are reconstructed.

- `horizon-world`: previously reported 334 Rust files fully reviewed.
- `horizon-client-render`: multiple major renderer files and submodules reviewed; no complete file ledger is available here.
- `horizon-protocol`: previously reported partial coverage (roughly 26 of 37 Rust files); exact coverage is uncertain.
- `native_hooks.rs`: only several separated ranges are evidenced; not complete.
- Other large client/game files were inspected in partial ranges or targeted snippets.

## The 100-file batch after `input_mode.rs`

- Batch target: 100 consecutive Rust files under `crates/horizon-client/src/game/`, after `input_mode.rs`, on the pinned Starling commit above.
- Previous status report: only `input_trace.rs` and `item_use.rs` can be confidently certified as fully reviewed from the available record. This does **not** establish that these two are members of the requested 100-file sequence; membership must be reconstructed from the pinned tree and the exact ordering used.
- Several other files were fetched in partial ranges or searched selectively, but there is no reliable file-by-file completion ledger for them.
- Batch status: **OPEN / NOT VERIFIED COMPLETE**.
- Do not advance beyond this batch until the exact 100-file sequence is reconstructed and each member is either fully reviewed or explicitly marked blocked with reason.

## Finding register

No findings are promoted to confirmed solely from the old conversational summaries. Candidate concerns mentioned historically (including bounds, panic paths, resource limits, and input validation) remain **potential/unverified** until the relevant full file and call paths are rechecked.

| ID | Area | Status | Required next action |
|---|---|---|---|
| CAND-001 | `blockctl.rs` fill volume / empty argument handling | Potential, unverified | Review full file, callers, bounds and arithmetic |
| CAND-002 | `attachment_sync.rs` stale/drop behavior | Potential, unverified | Review full file and lifecycle callers |
| CAND-003 | `cosmetics.rs` texture dimensions / RGBA length | Potential, unverified | Review full file and texture parsing/validation |
| CAND-004 | `command_minecarts.rs` permission and entity conversion | Potential, unverified | Review full file and trust boundary |
| CAND-005 | Other previously mentioned files | Unverified | Reconstruct evidence before making claims |

## Session log

### 2026-11-17 — ledger initialization

- Inspected the RustChunk branch head and repository tree.
- Confirmed no root `AUDIT_LEDGER.md` existed on the branch before this commit.
- No source code was changed by this ledger initialization.
- No build or test result is claimed.
