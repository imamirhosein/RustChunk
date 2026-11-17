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
| CAND-006 | `integrated.rs` / `world_loading.rs`: save folder is joined to `current_dir()/saves` before any visible component validation; `WorldRequest::new` also directly joins it | Potential, unverified | Trace all `open_integrated` callers and world-list folder provenance; inspect path validation at the true boundary before classifying |
| CAND-007 | `interactions.rs`: remote metadata floats for hitbox width/height are accepted without finite/nonnegative validation | Potential, unverified | Trace packet decoder and `ray_box`/AABB behavior for NaN, infinity, negative, and extreme values |
| CAND-008 | `jigsaw_blocks.rs`: packet-supplied `levels` is forwarded as `JigsawQuery.max_depth` without a local bound | Potential, unverified | Trace protocol type, `VanillaTerrain::jigsaw_pieces`, and JigsawConfig depth/placement caps; determine whether a malicious permitted guest can force excessive generation |

## Session log

### 2026-11-17 — ledger initialization

- Inspected the RustChunk branch head and repository tree.
- Confirmed no root `AUDIT_LEDGER.md` existed on the branch before this commit.
- No source code was changed by this ledger initialization.
- No build or test result is claimed.


## Reconstructed exact 100-file batch

Source ordering: recursive Git tree returned by GitHub for `starling-client/starling` at commit `5432370d573642041585c69b40fb045b27133488`; only Rust blobs directly beneath `crates/horizon-client/src/game/` and its descendants, lexicographically ordered as returned by the Git tree. The batch is the next 100 Rust files after `input_mode.rs`. Tree response reported `truncated=false` and 361 Rust files in this game subtree.

| # | File | Status |
|---:|---|---|
| 1 | `crates/horizon-client/src/game/input_trace.rs` | Complete (historical; no blob SHA recorded) |
| 2 | `crates/horizon-client/src/game/integrated.rs` | Complete (1–1330; SHA `baedd4a77d6b62dc337158bbaf451f8689cb040b`) |
| 3 | `crates/horizon-client/src/game/interactions.rs` | Complete (1–133; SHA `ddf1d967eb21ee7f1673434439aaadd431a7e334`) |
| 4 | `crates/horizon-client/src/game/item_draw.rs` | Complete (1–682; SHA `19776b1660bfe859d1737c9476c7db3a5ddc3489`) |
| 5 | `crates/horizon-client/src/game/item_particles.rs` | Complete (1–131; SHA `52072f7aaca615cfb8f699fb808cb751d17766a0`) |
| 6 | `crates/horizon-client/src/game/item_use.rs` | Complete (historical; no blob SHA recorded) |
| 7 | `crates/horizon-client/src/game/jigsaw_blocks.rs` | Pending |
| 8 | `crates/horizon-client/src/game/jigsaw_edit.rs` | Complete (1–332; SHA `119e3cea3187755fc3926f5f00d0fcbe793fee50`) |
| 9 | `crates/horizon-client/src/game/jukebox.rs` | Pending |
| 10 | `crates/horizon-client/src/game/key_focus.rs` | Pending |
| 11 | `crates/horizon-client/src/game/kinetic_machines.rs` | Pending |
| 12 | `crates/horizon-client/src/game/kinetics.rs` | Pending |
| 13 | `crates/horizon-client/src/game/lan.rs` | Pending |
| 14 | `crates/horizon-client/src/game/lan_actions.rs` | Pending |
| 15 | `crates/horizon-client/src/game/lan_attributes.rs` | Pending |
| 16 | `crates/horizon-client/src/game/lan_camera.rs` | Pending |
| 17 | `crates/horizon-client/src/game/lan_chat.rs` | Pending |
| 18 | `crates/horizon-client/src/game/lan_crafting.rs` | Pending |
| 19 | `crates/horizon-client/src/game/lan_entities.rs` | Pending |
| 20 | `crates/horizon-client/src/game/lan_far.rs` | Pending |
| 21 | `crates/horizon-client/src/game/lan_mobs.rs` | Pending |
| 22 | `crates/horizon-client/src/game/lan_mount_menu.rs` | Pending |
| 23 | `crates/horizon-client/src/game/lan_particles.rs` | Pending |
| 24 | `crates/horizon-client/src/game/lan_pvp.rs` | Pending |
| 25 | `crates/horizon-client/src/game/lan_quiet.rs` | Pending |
| 26 | `crates/horizon-client/src/game/lan_screen.rs` | Pending |
| 27 | `crates/horizon-client/src/game/lan_stations.rs` | Pending |
| 28 | `crates/horizon-client/src/game/lan_trading.rs` | Pending |
| 29 | `crates/horizon-client/src/game/lan_vehicles.rs` | Pending |
| 30 | `crates/horizon-client/src/game/lan_waypoints.rs` | Pending |
| 31 | `crates/horizon-client/src/game/launch_api.rs` | Pending |
| 32 | `crates/horizon-client/src/game/leads.rs` | Pending |
| 33 | `crates/horizon-client/src/game/level_particles.rs` | Pending |
| 34 | `crates/horizon-client/src/game/levels.rs` | Pending |
| 35 | `crates/horizon-client/src/game/lightning.rs` | Pending |
| 36 | `crates/horizon-client/src/game/llamas.rs` | Pending |
| 37 | `crates/horizon-client/src/game/locator_bar.rs` | Pending |
| 38 | `crates/horizon-client/src/game/loom.rs` | Pending |
| 39 | `crates/horizon-client/src/game/mace.rs` | Pending |
| 40 | `crates/horizon-client/src/game/machine_entities.rs` | Pending |
| 41 | `crates/horizon-client/src/game/machines.rs` | Pending |
| 42 | `crates/horizon-client/src/game/machines/crafter.rs` | Pending |
| 43 | `crates/horizon-client/src/game/machines/native_energy.rs` | Pending |
| 44 | `crates/horizon-client/src/game/machines/native_fluids.rs` | Pending |
| 45 | `crates/horizon-client/src/game/machines/native_items.rs` | Pending |
| 46 | `crates/horizon-client/src/game/maps.rs` | Pending |
| 47 | `crates/horizon-client/src/game/memory.rs` | Pending |
| 48 | `crates/horizon-client/src/game/minecarts.rs` | Pending |
| 49 | `crates/horizon-client/src/game/mob_blocks.rs` | Pending |
| 50 | `crates/horizon-client/src/game/mob_gear.rs` | Pending |
| 51 | `crates/horizon-client/src/game/mob_index.rs` | Pending |
| 52 | `crates/horizon-client/src/game/mob_looting.rs` | Pending |
| 53 | `crates/horizon-client/src/game/mob_sounds.rs` | Pending |
| 54 | `crates/horizon-client/src/game/mobile_controls.rs` | Pending |
| 55 | `crates/horizon-client/src/game/mobile_controls/mobile_platform.rs` | Pending |
| 56 | `crates/horizon-client/src/game/mod_api_host.rs` | Pending |
| 57 | `crates/horizon-client/src/game/mod_api_host_generated.rs` | Pending |
| 58 | `crates/horizon-client/src/game/mod_assets.rs` | Pending |
| 59 | `crates/horizon-client/src/game/mod_catalog.rs` | Pending |
| 60 | `crates/horizon-client/src/game/mods.rs` | Pending |
| 61 | `crates/horizon-client/src/game/mount_menu.rs` | Pending |
| 62 | `crates/horizon-client/src/game/mounts.rs` | Pending |
| 63 | `crates/horizon-client/src/game/moving_pistons.rs` | Pending |
| 64 | `crates/horizon-client/src/game/music.rs` | Pending |
| 65 | `crates/horizon-client/src/game/names.rs` | Pending |
| 66 | `crates/horizon-client/src/game/narrator.rs` | Pending |
| 67 | `crates/horizon-client/src/game/native_biome_colors.rs` | Pending |
| 68 | `crates/horizon-client/src/game/native_block_entity_draw.rs` | Pending |
| 69 | `crates/horizon-client/src/game/native_builds.rs` | Pending |
| 70 | `crates/horizon-client/src/game/native_claimed_items.rs` | Pending |
| 71 | `crates/horizon-client/src/game/native_components.rs` | Pending |
| 72 | `crates/horizon-client/src/game/native_criteria.rs` | Pending |
| 73 | `crates/horizon-client/src/game/native_data_maps.rs` | Pending |
| 74 | `crates/horizon-client/src/game/native_entities.rs` | Pending |
| 75 | `crates/horizon-client/src/game/native_entity_refs.rs` | Pending |
| 76 | `crates/horizon-client/src/game/native_explosions.rs` | Pending |
| 77 | `crates/horizon-client/src/game/native_falling.rs` | Pending |
| 78 | `crates/horizon-client/src/game/native_hooks.rs` | Pending |
| 79 | `crates/horizon-client/src/game/native_inventories.rs` | Pending |
| 80 | `crates/horizon-client/src/game/native_item_container.rs` | Pending |
| 81 | `crates/horizon-client/src/game/native_item_energy.rs` | Pending |
| 82 | `crates/horizon-client/src/game/native_item_fluids.rs` | Pending |
| 83 | `crates/horizon-client/src/game/native_lifecycle.rs` | Pending |
| 84 | `crates/horizon-client/src/game/native_menus.rs` | Pending |
| 85 | `crates/horizon-client/src/game/native_mob_draw.rs` | Pending |
| 86 | `crates/horizon-client/src/game/native_mobs.rs` | Pending |
| 87 | `crates/horizon-client/src/game/native_motion.rs` | Pending |
| 88 | `crates/horizon-client/src/game/native_mounts.rs` | Pending |
| 89 | `crates/horizon-client/src/game/native_net.rs` | Pending |
| 90 | `crates/horizon-client/src/game/native_panel.rs` | Pending |
| 91 | `crates/horizon-client/src/game/native_panel_recipes.rs` | Pending |
| 92 | `crates/horizon-client/src/game/native_projectiles.rs` | Pending |
| 93 | `crates/horizon-client/src/game/native_quick_move.rs` | Pending |
| 94 | `crates/horizon-client/src/game/native_recipes.rs` | Pending |
| 95 | `crates/horizon-client/src/game/native_server.rs` | Pending |
| 96 | `crates/horizon-client/src/game/native_shaders.rs` | Pending |
| 97 | `crates/horizon-client/src/game/native_ticks.rs` | Pending |
| 98 | `crates/horizon-client/src/game/native_visuals.rs` | Pending |
| 99 | `crates/horizon-client/src/game/native_world_columns.rs` | Pending |
| 100 | `crates/horizon-client/src/game/native_world_edit.rs` | Pending |

Batch count: 100. Complete: 2 (historical confirmation only). Pending: 98. Do not advance beyond this list until all 100 rows are complete or explicitly blocked.


### 2026-11-17 — file #2: integrated.rs

- Source: `starling-client/starling`, pinned commit `5432370d573642041585c69b40fb045b27133488`.
- Entire file fetched and reviewed in consecutive ranges: 1–180, 181–360, 361–540, 541–720, 721–900, 901–1080, 1081–1260, 1261–1330.
- Total: 1,330 lines. Blob SHA: `baedd4a77d6b62dc337158bbaf451f8689cb040b`.
- Result: no confirmed defect established in this pass. One path-boundary concern is recorded as CAND-006 and must be resolved by tracing callers and validation before closing it.
- Dependency cross-check: `world_loading.rs` lines 1–180 show `WorldRequest::new` directly joins the supplied folder to `current_dir()/saves`; that snippet alone does not establish exploitability because caller provenance is not yet traced.
- No source code changed; only the audit ledger was updated. No build/test claimed.


### 2026-11-17 — file #3: interactions.rs

- Source: pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Entire file fetched and reviewed: lines 1–133 (single complete fetch).
- Blob SHA: `ddf1d967eb21ee7f1673434439aaadd431a7e334`.
- Result: normal hitbox geometry and vehicle-position update are straightforward; two synced floats are assigned without validation. Recorded as CAND-007 pending verification of protocol input handling and ray/AABB robustness. No confirmed exploit or crash established.
- No source code changed; only the audit ledger was updated. No build/test claimed.


### 2026-11-17 — file #4: item_draw.rs

- Source: pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Entire file fetched and reviewed in consecutive ranges: 1–180, 181–360, 361–540, 541–682.
- Total: 682 lines. Blob SHA: `19776b1660bfe859d1737c9476c7db3a5ddc3489`.
- Result: no confirmed defect established in this pass. Reviewed item environment values, date formatter, use-duration derivation, item resolution, GPU/CPU submission and dropped-item culling. Potential numeric edge cases in pack-derived bundle fullness/use duration were not established as defects and are not promoted to findings.
- No source code changed; only the audit ledger was updated. No build/test claimed.


### 2026-11-17 — file #5: item_particles.rs

- Source: pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Entire file fetched and reviewed: lines 1–131.
- Total: 131 lines. Blob SHA: `52072f7aaca615cfb8f699fb808cb751d17766a0`.
- Result: no confirmed defect found in the reviewed particle/model selection, remote equipment-break handling, or break-sound resolution paths.
- No source code changed; only the audit ledger was updated. No build/test claimed.


### 2026-11-17 — file #7: jigsaw_blocks.rs

- Source: pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Entire file fetched and reviewed: lines 1–142.
- Total: 142 lines. Blob SHA: `4085e80887fb73f5bcf3095dae495a2285d13b7e`.
- Result: permission checks require creative mode and permission level 2 before guest jigsaw edits/generation. Packet `levels` is passed to the jigsaw query without a local clamp; recorded as CAND-008 pending review of downstream caps and protocol constraints. No confirmed issue yet.
- No source code changed; only the audit ledger was updated. No build/test claimed.


### 2026-11-17 — file #8: jigsaw_edit.rs

- Source: pinned Starling commit `5432370d573642041585c69b40fb045b27133488`.
- Entire file fetched and reviewed: lines 1–332.
- Total: 332 lines. Blob SHA: `119e3cea3187755fc3926f5f00d0fcbe793fee50`.
- Result: UI slider clamps generation depth to 0–20; identifier validation and packet ordering (set block then generate) are explicit. No confirmed defect found in this file. This does not resolve CAND-008's server-side/downstream depth-bound question.
- No source code changed; only the audit ledger was updated. No build/test claimed.
