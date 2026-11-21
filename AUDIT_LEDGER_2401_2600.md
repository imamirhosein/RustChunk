# RustChunk Starling Audit Ledger — Entries 2401–2600

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active stage: **2401–2600**; checkpoint **2401–2425 (25/200)**.
- One-time expanded authorization: **2401–2500 (100 files)**; checkpoints 25, reports 50. No authorization for 2501+.
- Exact source is gitlink `upstream/starling` at `5432370d573642041585c69b40fb045b27133488`; do not follow moving branch.
- Entries selected in lexical path order after excluding all 2,400 Index paths. Pinned tree complete/untruncated.
- Every entry has exactly 15 fields; paths, SHAs and sizes match pinned tree.
- Tests/examples inspected only; none executed. No application source changed or build/runtime/device validation.


### 2401. `crates/horizon-world/examples/probe/view_direction_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0c5f483df83202fef9da22363dae719ec9c7ea36`
- Size: 1933 bytes
- Coverage: Complete through EOF: 59 logical lines; full file; no gaps.
- Purpose: View vector and blocking-angle fixture
- Dependencies / cross-references: horizon-core geometry/random/position; horizon-world view, border, chunk, ray and worldgen; private JSON/NBT inputs.
- Implementation / logic: Runs acos, pitch/yaw view-vector, blocking-angle and fallback probes.
- Findings: JSON modes/vectors are unwrapped; malformed hex, short vectors or unknown modes panic. No camera/Android input integration.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Validate malformed modes/vectors and NaN/infinity boundaries; compare yaw/pitch and float bits with Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 59 logical lines through EOF; exact pinned SHA and 1933-byte size match; inspected, not modified; tests not executed.


### 2402. `crates/horizon-world/examples/probe/water_splash_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8d6eabc798fc7221725edb6333376a3e98b5ed54`
- Size: 5682 bytes
- Coverage: Complete through EOF: 183 logical lines; full file; no gaps.
- Purpose: Water splash particles, sounds, passenger and RNG fixture
- Dependencies / cross-references: horizon-core geometry/random/position; horizon-world view, border, chunk, ray and worldgen; private JSON/NBT inputs.
- Implementation / logic: Scripted WaterSplashWorld traces callbacks, mutations, failures, particles and Legacy/Xoroshiro draws.
- Findings: Passenger/mutation fields are trusted; unknown passenger panics. Fixed mock entity model and hard-coded budget/RNG checks.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test invalid passenger IDs and each callback failure/mutation; compare particle/sound/event order and RNG with Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 183 logical lines through EOF; exact pinned SHA and 5682-byte size match; inspected, not modified; tests not executed.


### 2403. `crates/horizon-world/examples/probe/world_border_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ab0b8795b90af1726b8d689c5f4f3a4cf7de854b`
- Size: 3413 bytes
- Coverage: Complete through EOF: 91 logical lines; full file; no gaps.
- Purpose: World border geometry and interpolation fixture
- Dependencies / cross-references: horizon-core geometry/random/position; horizon-world view, border, chunk, ray and worldgen; private JSON/NBT inputs.
- Implementation / logic: Applies center/size/limit/lerp/tick operations and emits bounds, collision, distance and containment.
- Findings: Operation arrays unwrapped; limit narrows i64 to i32; fixed defaults, not packet/timing integration.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test malformed ops, extreme coordinates/limits and lerp boundaries against Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 91 logical lines through EOF; exact pinned SHA and 3413-byte size match; inspected, not modified; tests not executed.


### 2404. `crates/horizon-world/examples/probe/world_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fd89e767b43c471e393e7a764fe3b38d2e997623`
- Size: 5507 bytes
- Coverage: Complete through EOF: 150 logical lines; full file; no gaps.
- Purpose: Palette NBT and block-state registry round-trip fixture
- Dependencies / cross-references: horizon-core geometry/random/position; horizon-world view, border, chunk, ray and worldgen; private JSON/NBT inputs.
- Implementation / logic: Verifies palette slots, compaction, BlockSection NBT and 4096-cell mutation against vanilla-generated data.
- Findings: Filename-derived count trusted (zero causes modulo panic); IDs narrow to u32; assumes fixed state naming/layout.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Validate filename/count and malformed NBT; compare palette packing, compaction and registry defaults with Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 150 logical lines through EOF; exact pinned SHA and 5507-byte size match; inspected, not modified; tests not executed.


### 2405. `crates/horizon-world/examples/probe/world_ray_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ded8645c7aad9b2fab0dcdeaa6a402f9c3823351`
- Size: 6637 bytes
- Coverage: Complete through EOF: 195 logical lines; full file; no gaps.
- Purpose: World block/fluid ray clipping fixture
- Dependencies / cross-references: horizon-core geometry/random/position; horizon-world view, border, chunk, ray and worldgen; private JSON/NBT inputs.
- Implementation / logic: Builds scripted voxel shapes and queries clip_world across collider/outline/visual/fall-reset modes.
- Findings: Mode indexes fixed arrays unchecked; unknown cells default to state zero; mock flags do not represent live chunks/entities.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Validate mode bounds/missing cells; test portals, fluids, negative coords and query budgets with Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 195 logical lines through EOF; exact pinned SHA and 6637-byte size match; inspected, not modified; tests not executed.


### 2406. `crates/horizon-world/examples/probe/worldgen_random_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1db24cf8adeb781044ec61a6225a248060a16498`
- Size: 5280 bytes
- Coverage: Complete through EOF: 134 logical lines; full file; no gaps.
- Purpose: Worldgen random, seed and positional-fork fixture
- Dependencies / cross-references: horizon-core geometry/random/position; horizon-world view, border, chunk, ray and worldgen; private JSON/NBT inputs.
- Implementation / logic: Exercises Legacy/Xoroshiro, decoration/feature seeds, slime RNG, forks and UTF-16 hashes.
- Findings: Inputs narrow i64 to i32/u16; unknown ops/layers panic; selected paths only.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test overflow, invalid bounds/bits, UTF-16 surrogate cases and nested forks against Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 134 logical lines through EOF; exact pinned SHA and 5280-byte size match; inspected, not modified; tests not executed.


### 2407. `crates/horizon-world/examples/structure_blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ba2eea0b8880eb970fa4c562d56272ddd376839d`
- Size: 4276 bytes
- Coverage: Complete through EOF: 103 logical lines; full file; no gaps.
- Purpose: Structures-only chunk-generation CLI
- Dependencies / cross-references: horizon-core geometry/random/position; horizon-world view, border, chunk, ray and worldgen; private JSON/NBT inputs.
- Implementation / logic: Loads versioned worldgen/block resources, generates 3×3 proto-chunk regions and dumps structures/block entities.
- Findings: Registry intentionally leaked for process lifetime; cx±1/cz±1 can overflow at i32 edges; external resource bundle required.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test extreme chunk coords/resource hashes; compare seeded structures and loot metadata with Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 103 logical lines through EOF; exact pinned SHA and 4276-byte size match; inspected, not modified; tests not executed.


### 2408. `crates/horizon-world/src/active_effect_tick.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b2de37baf05bb7c5ccc4f769635c9b20a6ea96e2`
- Size: 7464 bytes
- Coverage: Complete through EOF: 207 logical lines; full file; no gaps.
- Purpose: Active status-effect server/client tick loop
- Dependencies / cross-references: horizon-world status effects, health, attributes and sync hooks; JavaHashMap/ObjectOpenMap/ObjectOpenSet.
- Implementation / logic: Ticks/blends effects, expires/removes entries, notifies updates and emits client particles.
- Findings: Server unwraps current effect after cursor advancement; callback mutation/cursor invalidation needs parity. Client RNG bounds unwrap.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test concurrent modification, expiry cadence, particle visibility and RNG ordering against Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 207 logical lines through EOF; exact pinned SHA and 7464-byte size match; inspected, not modified; tests not executed.


### 2409. `crates/horizon-world/src/active_effects.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `94c74ad24a10b5a30b7b0f465b01e38826475d4e`
- Size: 5754 bytes
- Coverage: Complete through EOF: 173 logical lines; full file; no gaps.
- Purpose: Active status-effect storage and lifecycle
- Dependencies / cross-references: horizon-world status effects, health, attributes and sync hooks; JavaHashMap/ObjectOpenMap/ObjectOpenSet.
- Implementation / logic: Implements add/update/force-add/remove, effect callbacks and JavaHashMap storage.
- Findings: Some mutations precede callbacks; callback errors do not roll back. Public direct map mutations bypass lifecycle hooks.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test callback order/partial state, replacement, hidden blend and Java map iteration semantics.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 173 logical lines through EOF; exact pinned SHA and 5754-byte size match; inspected, not modified; tests not executed.


### 2410. `crates/horizon-world/src/attachments.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `118486c11e726fc2c8da0bbdbf31be70c6c73c25`
- Size: 24237 bytes
- Coverage: Complete through EOF: 695 logical lines; ranges 1–400, 401–695; no gaps.
- Purpose: Typed attachments, NBT persistence, death-copy and client sync
- Dependencies / cross-references: horizon-world status effects, health, attributes and sync hooks; JavaHashMap/ObjectOpenMap/ObjectOpenSet.
- Implementation / logic: Stores raw/decoded namespaced values, lazily decodes codecs, preserves unknown NBT and coalesces sync changes.
- Findings: 695 lines read 1–400/401–695. apply_sync defers codec validation until read; identity is namespace/name without serialized type ID.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test malformed envelopes, cross-type key collisions, decode/remove, unknown NBT preservation and sync/death-copy semantics.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 695 logical lines through EOF; exact pinned SHA and 24237-byte size match; inspected, not modified; tests not executed.


### 2411. `crates/horizon-world/src/attribute_health.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d4db63012347104d51b6fe7987aba34d0b5a9da6`
- Size: 5135 bytes
- Coverage: Complete through EOF: 139 logical lines; full file; no gaps.
- Purpose: Attribute-backed health and absorption adapter
- Dependencies / cross-references: horizon-world status effects, health, attributes and sync hooks; JavaHashMap/ObjectOpenMap/ObjectOpenSet.
- Implementation / logic: Binds max-health/max-absorption to SyncedFloat and base absorption; delegates heal/set/dead/hurt calculations.
- Findings: Base LivingEntity absorption only; Player synchronized absorption and virtual dispatch excluded; missing attributes returns None.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Add Player binding and virtual dispatch tests; compare clamps, NaN and sync behavior.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 139 logical lines through EOF; exact pinned SHA and 5135-byte size match; inspected, not modified; tests not executed.


### 2412. `crates/horizon-world/src/attribute_map.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4f8cc457f6e7ce8081ee282fa461ba8f4f0018b8`
- Size: 5943 bytes
- Coverage: Complete through EOF: 184 logical lines; full file; no gaps.
- Purpose: Live AttributeMap and dirty/sync queue semantics
- Dependencies / cross-references: horizon-world status effects, health, attributes and sync hooks; JavaHashMap/ObjectOpenMap/ObjectOpenSet.
- Implementation / logic: Routes queries to instances or suppliers; implements lazy creation and modified update/sync scheduling.
- Findings: Callback failures preserve queued updates and may leave partial state; None keys pass to suppliers; reentrancy matters.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test reentrant callbacks, null keys and failure between update/sync insertion.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 184 logical lines through EOF; exact pinned SHA and 5943-byte size match; inspected, not modified; tests not executed.


### 2413. `crates/horizon-world/src/attribute_updates.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f97f6477f592b48cb01424d41c6310775686d254`
- Size: 2310 bytes
- Coverage: Complete through EOF: 57 logical lines; full file; no gaps.
- Purpose: LivingEntity attribute-change side effects
- Dependencies / cross-references: horizon-world status effects, health, attributes and sync hooks; JavaHashMap/ObjectOpenMap/ObjectOpenSet.
- Implementation / logic: Dispatches health/absorption clamp, dimension refresh and server waypoint tracking.
- Findings: Waypoint path preserves hook order; max values cast f64 to f32 before comparison.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test NaN/infinite/narrowing and server/client call order.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 57 logical lines through EOF; exact pinned SHA and 2310-byte size match; inspected, not modified; tests not executed.


### 2414. `crates/horizon-world/src/attributes/entity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b7ffd392593a3a55cfab6337d3de2706db7010d8`
- Size: 9966 bytes
- Coverage: Complete through EOF: 277 logical lines; full file; no gaps.
- Purpose: Registered attribute catalog and lazy entity defaults
- Dependencies / cross-references: attribute catalog, modifier maps, registered defaults, effect attributes and identity/hash queues.
- Implementation / logic: Imports versioned JSON definitions/default suppliers; creates instances and dirty/sync state.
- Findings: DataVersion checked but entity IDs not canonicalized; AttributeId indexes assume catalog-origin IDs.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Validate entity IDs, schema, range/NaN and complete 1.20.1 registry ordering.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 277 logical lines through EOF; exact pinned SHA and 9966-byte size match; inspected, not modified; tests not executed.


### 2415. `crates/horizon-world/src/attributes/live.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f1faece1f5ff13e6f46ef747201a30614a35cf2d`
- Size: 12951 bytes
- Coverage: Complete through EOF: 416 logical lines; ranges 1–400, 401–416; no gaps.
- Purpose: Live attributes backed by native-order maps and queues
- Dependencies / cross-references: attribute catalog, modifier maps, registered defaults, effect attributes and identity/hash queues.
- Implementation / logic: Owns catalog keys, owner-scoped handles, nodes and identity allocator; integrates stored effect modifiers.
- Findings: 416 lines read 1–400/401–416. Clones copy owner and allocator state; divergent mutation can create colliding logical handles.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test clone divergence, duplicate allocator sequences, foreign handles, hash collisions and callback reentry.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 416 logical lines through EOF; exact pinned SHA and 12951-byte size match; inspected, not modified; tests not executed.


### 2416. `crates/horizon-world/src/attributes/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6778db4299b2bb3ecea2977bd92cdf5f88001df2`
- Size: 10791 bytes
- Coverage: Complete through EOF: 350 logical lines; full file; no gaps.
- Purpose: Attribute modifier operations and cached values
- Dependencies / cross-references: attribute catalog, modifier maps, registered defaults, effect attributes and identity/hash queues.
- Implementation / logic: Implements modifier identity, operation groups, add/remove/replace/apply and dirty observers.
- Findings: Modifier::new accepts empty path; add_or_update_transient retains old operation bucket. Verify Java parity.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Reject empty ResourceLocation paths; test normalization, stale buckets and floating-point order.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 350 logical lines through EOF; exact pinned SHA and 10791-byte size match; inspected, not modified; tests not executed.


### 2417. `crates/horizon-world/src/attributes/operation_map.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `faa15209446efb386de513c75a4b57a84bd95dac`
- Size: 3133 bytes
- Coverage: Complete through EOF: 104 logical lines; full file; no gaps.
- Purpose: Open-addressed attribute operation buckets
- Dependencies / cross-references: attribute catalog, modifier maps, registered defaults, effect attributes and identity/hash queues.
- Implementation / logic: Implements hashing, slot iteration, rehash, backward-shift deletion and copy-for-replace.
- Findings: Capacity uses f64 ceil/next_power_of_two; extreme lengths may overflow but allocation constrains them. Slot order is compatibility-critical.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Stress collisions, wraparound clusters, growth/shrink and float accumulation order against Java fastutil.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 104 logical lines through EOF; exact pinned SHA and 3133-byte size match; inspected, not modified; tests not executed.


### 2418. `crates/horizon-world/src/bin/worldgen_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eaf585023aaded9e38ae0ab7ab08221f090ea97a`
- Size: 31491 bytes
- Coverage: Complete through EOF: 915 logical lines; ranges 1–200, 201–400, 401–600, 601–800, 801–915; no gaps.
- Purpose: Worldgen density/noise/aquifer/climate/tag CLI parity harness
- Dependencies / cross-references: horizon-core density/noise/random; horizon-world worldgen registry/router/aquifer/climate/tags/settings.
- Implementation / logic: Provides stdin commands for noise/router, density volumes, aquifers, tags, climate index and settings with bit-level traces.
- Findings: 915 lines read 1–200/201–400/401–600/601–800/801–915. CLI indexes args/bundle keys without full arity/range validation (e.g. settings_create).
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Add parser/arity/bounds tests and malformed bundle cases; compare traces/RNG with Java 1.20.1.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 915 logical lines through EOF; exact pinned SHA and 31491-byte size match; inspected, not modified; tests not executed.


### 2419. `crates/horizon-world/src/block_collision_scan.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f514e38f00e0ad07856c5b8f384ff5007ecaa3c1`
- Size: 14123 bytes
- Coverage: Complete through EOF: 370 logical lines; full file; no gaps.
- Purpose: Lazy block-collision scan with chunk caching and candidate budget
- Dependencies / cross-references: horizon-core geometry; horizon-world collision context, registry, dynamic support, fluids, pistons and chunk providers.
- Implementation / logic: Walks padded AABB cells in cursor order, caches chunk/section reachability and yields collider shapes.
- Findings: Wrapping arithmetic intentional; budget counts skipped cursor cells. Correctness relies on may_reach_out never false-negative.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test AABB/i32 edges, chunk transitions, missing chunks, pistons, large shapes and budget.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 370 logical lines through EOF; exact pinned SHA and 14123-byte size match; inspected, not modified; tests not executed.


### 2420. `crates/horizon-world/src/block_collision.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `46a061e2de35fe042eec5aef32cac5c393f68dd3`
- Size: 12617 bytes
- Coverage: Complete through EOF: 312 logical lines; full file; no gaps.
- Purpose: Cached/dynamic/contextual block collision dispatch
- Dependencies / cross-references: horizon-core geometry; horizon-world collision context, registry, dynamic support, fluids, pistons and chunk providers.
- Implementation / logic: Precomputes state flags/shapes and binds scaffolding, liquid, powder-snow, dynamic support and piston rules.
- Findings: PistonBlockBindings errors discarded with .ok(), conflating invalid metadata with no pistons.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Preserve error distinction; test incomplete registries and contextual/dynamic shapes.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 312 logical lines through EOF; exact pinned SHA and 12617-byte size match; inspected, not modified; tests not executed.


### 2421. `crates/horizon-world/src/block_entities.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1ee8a90e0cef90fdd9dadbb8e69fe457baff920a`
- Size: 3816 bytes
- Coverage: Complete through EOF: 99 logical lines; full file; no gaps.
- Purpose: Proto-chunk pending/active block-entity storage
- Dependencies / cross-references: horizon-world registry, NBT, collision, movement properties, FeatureWorld/PlacementContext and block-state tables.
- Implementation / logic: Resolves numeric NBT coordinates, relocates X/Z into owning chunk, tracks pending/active and serializes active entities via callback.
- Findings: Missing/wrong-type coordinates default to zero; Long narrows to i32; pending_mut can alter coordinates without rekeying.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test malformed/overflow coordinates, negative chunks and pending/active precedence.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 99 logical lines through EOF; exact pinned SHA and 3816-byte size match; inspected, not modified; tests not executed.


### 2422. `crates/horizon-world/src/block_entity_push.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `986d12b6520de647a1c9145c23c8e31b3e0ba768`
- Size: 2751 bytes
- Coverage: Complete through EOF: 75 logical lines; full file; no gaps.
- Purpose: Block replacement collision expansion and entity displacement
- Dependencies / cross-references: horizon-world registry, NBT, collision, movement properties, FeatureWorld/PlacementContext and block-state tables.
- Implementation / logic: Computes newly occupied shape, queries entities in bounds and moves each upward with collision resolution.
- Findings: Selection uses added-shape bounds; entity boxes re-read before each teleport; depends on stable handles/query order.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test multi-entity movement, collision clipping and query ordering.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 75 logical lines through EOF; exact pinned SHA and 2751-byte size match; inspected, not modified; tests not executed.


### 2423. `crates/horizon-world/src/block_landing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `60f5296d88dd97f4bbce656d56478ede8d745b95`
- Size: 16331 bytes
- Coverage: Complete through EOF: 464 logical lines; ranges 1–400, 401–464; no gaps.
- Purpose: Block landing, fall damage, bounce, turtle-egg and farmland reactions
- Dependencies / cross-references: horizon-world registry, NBT, collision, movement properties, FeatureWorld/PlacementContext and block-state tables.
- Implementation / logic: Builds per-state rules and runs server/client damage, sound, block-event and entity-push sequences.
- Findings: 464 lines read 1–400/401–464. Farmland RNG draw precedes entity gating; hook contracts control event order; definitions strict for all block names.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Test RNG/event order, spawn protection, mob griefing, entity classes and NaN/infinite fall distance.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 464 logical lines through EOF; exact pinned SHA and 16331-byte size match; inspected, not modified; tests not executed.


### 2424. `crates/horizon-world/src/block_placement.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cb18e55f5fa8fef881e554e00f4ecdcb5d62329a`
- Size: 13802 bytes
- Coverage: Complete through EOF: 376 logical lines; full file; no gaps.
- Purpose: Double-plant, waterlogging and pale-moss-carpet placement
- Dependencies / cross-references: horizon-world registry, NBT, collision, movement properties, FeatureWorld/PlacementContext and block-state tables.
- Implementation / logic: Precomputes placement kinds, double-plant transitions and 162 pale-moss states; updates neighbors and schedules ticks.
- Findings: State IDs checked; relative coords wrap i32 by design; assumes complete pale-moss registry.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Validate 1.20.1 pale-moss state set; test neighbor/RNG order, obstruction, waterlogging and i32 edges.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 376 logical lines through EOF; exact pinned SHA and 13802-byte size match; inspected, not modified; tests not executed.


### 2425. `crates/horizon-world/src/block_properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `98e06f7cbdb61c329e8136bc17f676ad7c4de2af`
- Size: 8772 bytes
- Coverage: Complete through EOF: 246 logical lines; full file; no gaps.
- Purpose: Block-definition flags, solidity, replaceability and occlusion cache
- Dependencies / cross-references: horizon-world registry, NBT, collision, movement properties, FeatureWorld/PlacementContext and block-state tables.
- Implementation / logic: Loads DataVersion-checked properties, validates definitions and caches occlusion faces.
- Findings: Every state needs definitions and occlusion shape; face mask assumes Direction discriminants fit u8.
- RustChunk relevance: Core world/gameplay architecture or reference harness for 1.20.1; pinned code targets 26.3 and requires version-specific verification.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish complete vanilla 1.20.1, registry/data parity, protocol 763, Forge 47.x or Aternos compatibility.
- Follow-up checks: Validate 1.20.1 definitions, face mapping and force-solid precedence.
- RustChunk implementation mapping: Port behind RustChunk-owned 1.20.1 interfaces; keep fixtures/CLI outside Android release dependencies; preserve upstream and third-party licenses.
- Evidence required before adoption: Version-hashed Java 1.20.1 inputs, malformed/positive/negative corpus, executed differential traces, Android ARM64 evidence and separate Forge 47.x tests.
- Completion state: COMPLETE — 246 logical lines through EOF; exact pinned SHA and 8772-byte size match; inspected, not modified; tests not executed.


## Checkpoint 2401–2425 — COMPLETE (25/25)

- All 25 files read individually through EOF; path/SHA/size match pinned submodule tree.
- Long files: attachments.rs 1–400/401–695; live.rs 1–400/401–416; worldgen_parity.rs 1–200/201–400/401–600/601–800/801–915; block_landing.rs 1–400/401–464.
- Flags: trusted fixture/CLI selectors; partial callback mutations; live attribute clone/allocator assumptions; empty modifier path accepted; unchecked CLI arity; may_reach_out false-negative risk; swallowed piston-binding errors; missing block-entity coords default zero; strict registry completeness.
- Index has 2,425 sequential unique paths; all new entries have 15 fields and aligned Decisions.
- No tests run, source changes, builds, APK/device checks or backup.
- Continue 2426–2450 under same authorization; report at 2450 and continue through 2500, then STOP.
