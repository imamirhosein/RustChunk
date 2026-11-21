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

### 2426. `crates/horizon-world/src/block_render.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `63efd595f8d56a57b06f970b386c3076b8279121`
- Size: 9461 bytes
- Coverage: Complete through EOF: 257 logical lines; full file; no gaps.
- Purpose: Neighbor face visibility and render-face culling
- Dependencies / cross-references: horizon-world block registry/properties, collision/support, fluids, dynamic world callbacks and pinned tags/NBT.
- Implementation / logic: Precomputes per-state face IDs from cached occlusion shapes; applies same-block/leaves/liquid/bars/mangrove skip rules and bounded face-pair cache.
- Findings: Requires every registry state to have render definition and cached shape; cache clears at 4096 pairs. Opaque face visibility only, not translucent sorting, fluids, meshing or lighting.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test canonical empty/full shape identity, bars masks, liquid families and cache rollover; compare with Java 1.20.1 renderer.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 257 logical lines through EOF; exact pinned SHA and 9461-byte size verified; inspected, not changed; tests not executed.


### 2427. `crates/horizon-world/src/block_state_properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `56e959ca0464f4ff660da8dbe7ff6a29522dbe53`
- Size: 14206 bytes
- Coverage: Complete through EOF: 418 logical lines; ranges 1–400, 401–418, each <=400 lines.
- Purpose: Block-state property registry and decoded access
- Dependencies / cross-references: horizon-world block registry/properties, collision/support, fluids, dynamic world callbacks and pinned tags/NBT.
- Implementation / logic: Builds immutable property descriptors from versioned registry data and exposes state-bound lookup/conversion.
- Findings: 418 lines read 1–400/401–418. Registry/schema completeness and IDs trusted at binding boundaries; properties parsed from string snapshots, not live Forge state.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Validate all 1.20.1 property domains/defaults, unknown/missing keys and malformed values against Java.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 418 logical lines through EOF; exact pinned SHA and 14206-byte size verified; inspected, not changed; tests not executed.


### 2428. `crates/horizon-world/src/block_support.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a2fe7811c0884c9e7cca605e7076b5c8e7d8934e`
- Size: 8310 bytes
- Coverage: Complete through EOF: 217 logical lines; full file; no gaps.
- Purpose: Full/center/rigid face support and cached/dynamic dispatch
- Dependencies / cross-references: horizon-world block registry/properties, collision/support, fluids, dynamic world callbacks and pinned tags/NBT.
- Implementation / logic: Computes support flags from voxel face shapes and dispatches uncached states to DynamicSupport callbacks.
- Findings: Cached geometry models context-free branch only; moving-piston/block-entity behavior remains callback-owned. Unsupported dynamic states return error.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Compare support combinations with Java 1.20.1; test registry identity and moving-piston context.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 217 logical lines through EOF; exact pinned SHA and 8310-byte size verified; inspected, not changed; tests not executed.


### 2429. `crates/horizon-world/src/block_survival.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f5c6d56a9d476acea54199d78c35efd4fc442ceb`
- Size: 50928 bytes
- Coverage: Complete through EOF: 1220 logical lines; ranges 1–400, 401–600, 601–800, 801–1200, 1201–1220, each <=400 lines.
- Purpose: Block survival rules for plants, attachments, fluids and support
- Dependencies / cross-references: horizon-world block registry/properties, collision/support, fluids, dynamic world callbacks and pinned tags/NBT.
- Implementation / logic: Builds typed rules from properties/tags, evaluates fire/cactus/mushroom/vines/crops/doors/chorus/plants/scaffolding through world/support/collision queries.
- Findings: 1220 lines read 1–400/401–600/601–800/801–1200/1201–1220. CactusFlower reads same below-position twice; Farmland may read above twice after solid test. Duplicate world callbacks can diverge for stateful providers. Registry/tag completeness required.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Confirm duplicate query ordering against Java; add stateful-world query tests and compare every rule/tag/fluid condition with 1.20.1.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1220 logical lines through EOF; exact pinned SHA and 50928-byte size verified; inspected, not changed; tests not executed.


### 2430. `crates/horizon-world/src/blocking_item_damage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `466532f87ea140c0f2e9abcabb140be00aa09cd5`
- Size: 1301 bytes
- Coverage: Complete through EOF: 40 logical lines; full file; no gaps.
- Purpose: Blocking component durability dispatch
- Dependencies / cross-references: horizon-world entity/block/fluid/collision interfaces; horizon-core geometry and position.
- Implementation / logic: Applies blocking item damage for players, awards item-used on server, computes damage and calls hurt/break.
- Findings: Stack identity and server-player state are callback-owned; ordering is contractual. Does not decode components or sounds.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test client/server/non-player branches, callback failures and award/hurt order.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 40 logical lines through EOF; exact pinned SHA and 1301-byte size verified; inspected, not changed; tests not executed.


### 2431. `crates/horizon-world/src/blocking_rules.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c990b991a96677404e3c5dba90e2903b88610eaa`
- Size: 2660 bytes
- Coverage: Complete through EOF: 95 logical lines; full file; no gaps.
- Purpose: Blocking damage reduction, delay/cooldown and item damage arithmetic
- Dependencies / cross-references: horizon-world entity/block/fluid/collision interfaces; horizon-core geometry and position.
- Implementation / logic: Resolves angular/type-gated reductions, sums/clamps damage and converts seconds to ticks.
- Findings: NaN/signed-zero explicit; round uses f64 intermediate on f32 input; angle is caller-supplied radians.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Differential-test half-tick rounding, NaN, signed zero and negative damage.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 95 logical lines through EOF; exact pinned SHA and 2660-byte size verified; inspected, not changed; tests not executed.


### 2432. `crates/horizon-world/src/bundle_weight.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cf8715676a1416fc092f3ffae66b9ec7c7d6a896`
- Size: 6470 bytes
- Coverage: Complete through EOF: 191 logical lines; full file; no gaps.
- Purpose: Bundle Fraction arithmetic, recursive weight and contents validation
- Dependencies / cross-references: horizon-world entity/block/fluid/collision interfaces; horizon-core geometry and position.
- Implementation / logic: Uses checked rational arithmetic, gcd, lazy cached weight and stack-count validation.
- Findings: Weight cached in Rc OnceCell and permanently captures first ItemWeights view; mutations to that view leave stale weight. Fraction constructor does not fully normalize immediately.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test cache invalidation contract, nested bundles/cycles, negative counts, overflow and Java Fraction edges.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 191 logical lines through EOF; exact pinned SHA and 6470-byte size verified; inspected, not changed; tests not executed.


### 2433. `crates/horizon-world/src/chunk_status.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `69ed423859e5ad514c2cfb136ddb0f5a2f4c3da0`
- Size: 3279 bytes
- Coverage: Complete through EOF: 100 logical lines; full file; no gaps.
- Purpose: Chunk status identities, ordering, parsing and heightmap selection
- Dependencies / cross-references: horizon-world entity/block/fluid/collision interfaces; horizon-core geometry and position.
- Implementation / logic: Defines ten ordered generation statuses, parent/index/name conversions and NBT codec.
- Findings: by_name maps unknown names to Empty; parse/from_nbt return errors. Status assignment does not execute generation or lighting.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test aliases, unknown namespaces and final heightmaps with Java 1.20.1.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 100 logical lines through EOF; exact pinned SHA and 3279-byte size verified; inspected, not changed; tests not executed.


### 2434. `crates/horizon-world/src/climbing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d0af2e1d5240dd96543f8f6f8ea38de9810e0604`
- Size: 4931 bytes
- Coverage: Complete through EOF: 137 logical lines; full file; no gaps.
- Purpose: Climbing tags, ladders, open trapdoors and last-climbable position
- Dependencies / cross-references: horizon-world entity/block/fluid/collision interfaces; horizon-core geometry and position.
- Implementation / logic: Builds state cache from tags and runs spectator/fall-flying/trapdoor/ladder checks in native callback order.
- Findings: Only positive result updates last-climbable position; negative checks preserve old value. Facing limited to horizontal directions.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test callback order, stale last position and trapdoor orientation.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 137 logical lines through EOF; exact pinned SHA and 4931-byte size verified; inspected, not changed; tests not executed.


### 2435. `crates/horizon-world/src/collision_context.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `edcb0d51631e2360dbbb942f3c51d3a747785da5`
- Size: 5362 bytes
- Coverage: Complete through EOF: 170 logical lines; full file; no gaps.
- Purpose: Entity, placement and position-only collision contexts
- Dependencies / cross-references: horizon-world entity/block/fluid/collision interfaces; horizon-core geometry and position.
- Implementation / logic: Captures entity identity/y/descending/held item/living/placement/fluid flags; implements above-shape and fluid support callbacks.
- Findings: with_position(None,y) ignores y and uses -MAX; can_stand_on_fluid calls entity callback before same-family test.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Compare context factories, empty-stack semantics, y threshold and callback side effects.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 170 logical lines through EOF; exact pinned SHA and 5362-byte size verified; inspected, not changed; tests not executed.


### 2436. `crates/horizon-world/src/collision.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f22f314845baa3b9a650ac065f1af1a763d14c7c`
- Size: 12724 bytes
- Coverage: Complete through EOF: 311 logical lines; full file; no gaps.
- Purpose: Cached voxel collision shape registry and canonical identity
- Dependencies / cross-references: horizon-world item components, durability, NBT and copy-on-write maps.
- Implementation / logic: Loads versioned shape NBT, validates geometry metadata, deduplicates grids and maps states to cached shapes.
- Findings: Context-free branch only; uncached states must invoke block behavior. Older exports without canonical identity retain local aliases only.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Validate canonical empty/full identity, duplicate grids, metadata and state mapping.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 311 logical lines through EOF; exact pinned SHA and 12724-byte size verified; inspected, not changed; tests not executed.


### 2437. `crates/horizon-world/src/combat_rules.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4d8aa424f7f604785d8de75396cd1037f6a39bdf`
- Size: 1719 bytes
- Coverage: Complete through EOF: 51 logical lines; full file; no gaps.
- Purpose: Armor/toughness and enchantment-protection arithmetic
- Dependencies / cross-references: horizon-world item components, durability, NBT and copy-on-write maps.
- Implementation / logic: Applies vanilla armor formula, optional weapon effectiveness and magic protection clamp.
- Findings: NaN/signed-zero behavior explicit; depends on correct weapon/server-level callbacks.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test extreme armor/toughness/damage, NaN and enchantment effectiveness.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 51 logical lines through EOF; exact pinned SHA and 1719-byte size verified; inspected, not changed; tests not executed.


### 2438. `crates/horizon-world/src/component_durability.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5bf576c08b8f8314af11652899bcf078a8e8f7d5`
- Size: 5855 bytes
- Coverage: Complete through EOF: 185 logical lines; full file; no gaps.
- Purpose: Data-component durability prototype, patch and stack behavior
- Dependencies / cross-references: horizon-world item components, durability, NBT and copy-on-write maps.
- Implementation / logic: Models max-damage/damage/unbreakable, patch persistence, clamps, break prediction and count changes.
- Findings: Raw components remain while stack empty; grow/shrink wrap integers. Only one component family, not full ItemStack.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test empty stack, unbreakable/removal, zero max damage and overflow.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 185 logical lines through EOF; exact pinned SHA and 5855-byte size verified; inspected, not changed; tests not executed.


### 2439. `crates/horizon-world/src/component_patch.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `675ddce1dc384c602b0022bfe2dc7f84d84cec08`
- Size: 6731 bytes
- Coverage: Complete through EOF: 196 logical lines; full file; no gaps.
- Purpose: Shared component prototypes and ordered copy-on-write patches
- Dependencies / cross-references: horizon-world item components, durability, NBT and copy-on-write maps.
- Implementation / logic: Implements prototype/patch/map lookup, set/remove/apply/restore and ordered iteration.
- Findings: set mutates patch before returning MissingPrototype for unsanitized removal without prototype; partial mutation documented, needs native exception parity.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test malformed patches, return values, iteration/key order, clone sharing and errors.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 196 logical lines through EOF; exact pinned SHA and 6731-byte size verified; inspected, not changed; tests not executed.


### 2440. `crates/horizon-world/src/conditions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `da22c072d9225f8ad7c4ad63db4efa6bac55fbaf`
- Size: 6047 bytes
- Coverage: Complete through EOF: 161 logical lines; full file; no gaps.
- Purpose: NeoForge data-pack load conditions
- Dependencies / cross-references: horizon-world damage, status, tag catalog, combat and entity callbacks.
- Implementation / logic: Evaluates mod_loaded/true/not/and/or; unknown nested condition types exclude file.
- Findings: Missing/non-array `values` in `and` becomes empty iterator and passes via vacuous all; malformed shape may be accepted instead of rejected by codec.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test absent/wrong-type values, empty and/or, malformed not and NeoForge 1.20.1/47.x codecs.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 161 logical lines through EOF; exact pinned SHA and 6047-byte size verified; inspected, not changed; tests not executed.


### 2441. `crates/horizon-world/src/consumable.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1e504afbe000831dd421ba26e2b78ceee787aac5`
- Size: 6959 bytes
- Coverage: Complete through EOF: 208 logical lines; full file; no gaps.
- Purpose: Consumable timing, particle/sound emission and item consumption
- Dependencies / cross-references: horizon-world damage, status, tag catalog, combat and entity callbacks.
- Implementation / logic: Implements use timing, sound volume/pitch, particles, overrides, listeners, effects, criteria and stack shrink.
- Findings: emit unconditionally draws nextBoolean, triangleFloat and nextFloat before choosing Eat/Drink. Java conditional path likely draws only selected animation values; likely over-consumes RNG.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Verify exact Java call order; split draws by animation and test next RNG state for Eat vs Drink.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 208 logical lines through EOF; exact pinned SHA and 6959-byte size verified; inspected, not changed; tests not executed.


### 2442. `crates/horizon-world/src/consume_effect.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8f0bdd4a794a278d6c2a222686d2b2776a29695b`
- Size: 3944 bytes
- Coverage: Complete through EOF: 107 logical lines; full file; no gaps.
- Purpose: Apply/remove/clear status and play-sound consume effects
- Dependencies / cross-references: horizon-world damage, status, tag catalog, combat and entity callbacks.
- Implementation / logic: Executes probability, copies instances, resolves tag sets, removes effects and plays sounds.
- Findings: NaN probability passes because random>=NaN is false; usual Java random<probability predicate returns false. Confirm source oracle.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test NaN/infinities and <0/0/1/>1 probabilities, RNG consumption and tag rebinding.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 107 logical lines through EOF; exact pinned SHA and 3944-byte size verified; inspected, not changed; tests not executed.


### 2443. `crates/horizon-world/src/damage_immunity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c71be64da3be696afcf78e885da78b2806067627`
- Size: 2668 bytes
- Coverage: Complete through EOF: 73 logical lines; full file; no gaps.
- Purpose: Entity/living/player/server-player damage immunity ordering
- Dependencies / cross-references: horizon-world damage, status, tag catalog, combat and entity callbacks.
- Implementation / logic: Implements removed/invulnerability/bypass/fire/fall/resistance and player dimension/load checks.
- Findings: Player rule loop returns on first matching tag; Drowning/Fall/Fire/Freezing order matters and Fall tag is also used in base immunity.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test overlapping tags, bypass/creative and server dimension/load order.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 73 logical lines through EOF; exact pinned SHA and 2668-byte size verified; inspected, not changed; tests not executed.


### 2444. `crates/horizon-world/src/damage_reduction.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0bcfce5dd6c99dd46513fb38ebb36f3a2344ba6a`
- Size: 2926 bytes
- Coverage: Complete through EOF: 78 logical lines; full file; no gaps.
- Purpose: Armor, resistance, enchantment and bypass orchestration
- Dependencies / cross-references: horizon-world damage, status, tag catalog, combat and entity callbacks.
- Implementation / logic: Applies armor wear, armor/toughness, resistance stats, enchantment bypass and protection callbacks.
- Findings: Resistance amplifier wraps i32; resisted-stat award uses distinct victim/attacker server-player lookups.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test bypass combinations, amplifier overflow, negative/NaN damage and stat attribution.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 78 logical lines through EOF; exact pinned SHA and 2926-byte size verified; inspected, not changed; tests not executed.


### 2445. `crates/horizon-world/src/damage_source.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6af30f736746e95ced18f1b126c4d3fea451627a`
- Size: 10031 bytes
- Coverage: Complete through EOF: 286 logical lines; full file; no gaps.
- Purpose: Version-checked damage type/tag catalog and DamageSource identity
- Dependencies / cross-references: horizon-world damage, status, tag catalog, combat and entity callbacks.
- Implementation / logic: Loads definitions/tags and exposes bypass/hurt/immunity tags, direct/causing entities, weapon/position and scaling.
- Findings: Immutable catalog snapshot; unresolved required tags fail. LivingNonPlayer intentionally excludes Player.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Validate complete 1.20.1 damage registry/tags, scaling and virtual causing-entity dispatch.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 286 logical lines through EOF; exact pinned SHA and 10031-byte size verified; inspected, not changed; tests not executed.


### 2446. `crates/horizon-world/src/datafix/block_state_data.txt`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a0c7e7f9bffd3dadb57446e1c7a3ec55e69c9988`
- Size: 450617 bytes
- Coverage: Complete through EOF: 1703 logical lines; ranges 1–400, 401–800, 801–1200, 1201–1600, 1601–1703, each <=400 lines.
- Purpose: Generated legacy 1.12 block-state to 1.13+ mapping table
- Dependencies / cross-references: horizon-world datafix schemas, NBT, region files, JSON, chunk palettes and legacy 26.3 resources.
- Implementation / logic: Maps legacy numeric block/data IDs to modern states and aliases in static initializer order.
- Findings: 1703 lines read 1–400/401–800/801–1200/1201–1600/1601–1703. 1695 rows; IDs strictly increasing/no duplicates; no malformed columns/IDs/property order; max 81 aliases per row. Generated from vanilla 26.3, not 1.20.1.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Pin generator/source version; do not treat as 1.20.1 runtime registry or Forge block-ID table.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1703 logical lines through EOF; exact pinned SHA and 450617-byte size verified; inspected, not changed; tests not executed.


### 2447. `crates/horizon-world/src/datafix/chunk_leaves.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `07d59cbdff11c81c184cd2b3ea019a48dd7f7293`
- Size: 15888 bytes
- Coverage: Complete through EOF: 488 logical lines; ranges 1–400, 401–488, each <=400 lines.
- Purpose: Legacy LeavesFix and trapped-chest datafix
- Dependencies / cross-references: horizon-world datafix schemas, NBT, region files, JSON, chunk palettes and legacy 26.3 resources.
- Implementation / logic: Rewrites packed section leaf distances with fastutil-compatible iteration and converts trapped-chest IDs.
- Findings: 488 lines read 1–400/401–488. Trapped-chest local x/y/z pass to get_index without explicit bounds; malformed coordinates can alias another local index. Bad palette/storage can make section skippable.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test out-of-range block-entity coords, malformed palette/storage, duplicate section Ys, fastutil order and packed-bit boundaries.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 488 logical lines through EOF; exact pinned SHA and 15888-byte size verified; inspected, not changed; tests not executed.


### 2448. `crates/horizon-world/src/datafix/dynamic.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `69cca1453bccd2781d96d1ae052afeb54d28d76c`
- Size: 8307 bytes
- Coverage: Complete through EOF: 264 logical lines; full file; no gaps.
- Purpose: DataFixerUpper Dynamic/NbtOps number, map, list and conversion helpers
- Dependencies / cross-references: horizon-world datafix schemas, NBT, region files, JSON, chunk palettes and legacy 26.3 resources.
- Implementation / logic: Implements Java Number casts/strings, NBT streams, field edits, identifiers and ceil-log2.
- Findings: Number narrowing follows Java-style casts; stream clones lists and expands arrays. String conversions may be lossy for unpaired UTF-16.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Test NaN/infinity, long/double narrowing, unpaired UTF-16, malformed arrays and Number.toString.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 264 logical lines through EOF; exact pinned SHA and 8307-byte size verified; inspected, not changed; tests not executed.


### 2449. `crates/horizon-world/src/datafix/files.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fbf63fd7f2450b0b7dcfc85af14b46153196bbe5`
- Size: 42765 bytes
- Coverage: Complete through EOF: 1147 logical lines; ranges 1–200, 201–400, 401–600, 601–800, 801–1000, 1001–1147, each <=400 lines.
- Purpose: World file-layout migration and world-upgrade orchestration
- Dependencies / cross-references: horizon-world datafix schemas, NBT, region files, JSON, chunk palettes and legacy 26.3 resources.
- Implementation / logic: Moves legacy files, extracts level.dat data, upgrades regions/saved data/player JSON and stamps DataVersion last for retry safety.
- Findings: 1147 lines read 1–200/201–400/401–600/601–800/801–1000/1001–1147. files_with_suffix/subdirectories suppress read_dir errors into empty lists, silently skipping inaccessible world data. Legacy chunk_x±8 ranges may overflow i32. Upgrade targets 26.3, not a 1.20.1 loader.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Propagate filesystem enumeration errors; validate coordinate bounds/staging; keep 26.3 migration separate from 1.20.1 save format.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1147 logical lines through EOF; exact pinned SHA and 42765-byte size verified; inspected, not changed; tests not executed.


### 2450. `crates/horizon-world/src/datafix/fixes_a.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e47cf15e4d0f13c3c9d4fea67b2edcb69b57cebe`
- Size: 36422 bytes
- Coverage: Complete through EOF: 1116 logical lines; ranges 1–200, 201–400, 401–600, 601–800, 801–1000, 1001–1116, each <=400 lines.
- Purpose: Datafixes from Minecraft 1.20.2 through 1.20.6
- Dependencies / cross-references: horizon-world datafix schemas, NBT, region files, JSON, chunk palettes and legacy 26.3 resources.
- Implementation / logic: Registers sign, random sequence, scoreboard, mob effect, block rename, projectile, horse, position, bee, banner, potion, component and trade migrations.
- Findings: 1116 lines read 1–200/201–400/401–600/601–800/801–1000/1001–1116. Multiple fixes share same version/substep keys and order is semantic; chested-horse slot conversion subtracts unchecked numeric values.
- RustChunk relevance: Reference/core behavior for a Rust Android Minecraft engine; pinned upstream is 26.3 and must not be mistaken for target 1.20.1 behavior.
- Decision: adaptable; reference
- 1.20.1 implications: Verify version-bound rules against vanilla 1.20.1 and Forge 47.x; 26.3 datafixes/resources are not directly portable to 1.20.1.
- Follow-up checks: Preserve registration order; test version keys and malformed slots; do not apply 1.20.2–1.20.6 fixes to 1.20.1.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep 26.3 datafix migration and oracle assets separate from Android runtime.
- Evidence required before adoption: Canonical 1.20.1 Java sources/data, version-hashed fixtures, malformed-input and differential tests, Android ARM64 evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1116 logical lines through EOF; exact pinned SHA and 36422-byte size verified; inspected, not changed; tests not executed.


## Checkpoint 2426–2450 — COMPLETE (25/25); section 2401–2450 CLOSED

- All 25 files inspected individually through EOF; paths, SHAs and byte sizes match pinned source tree.
- Long-file coverage: block_state_properties.rs 1–400/401–418; block_survival.rs 1–400/401–600/601–800/801–1200/1201–1220; block_state_data.txt 1–400/401–800/801–1200/1201–1600/1601–1703; chunk_leaves.rs 1–400/401–488; files.rs 1–200/201–400/401–600/601–800/801–1000/1001–1147; fixes_a.rs 1–200/201–400/401–600/601–800/801–1000/1001–1116.
- Key issues: duplicate survival queries in CactusFlower/Farmland; probable consumable RNG overdraw; NaN consume-effect probability inversion; malformed NeoForge and condition can pass vacuously; filesystem enumeration errors may silently skip world data; legacy coordinate range arithmetic can overflow.
- Data table 2446: 1695 rows, IDs strictly increasing/no duplicates, no malformed columns/IDs/property order; generated from vanilla 26.3, not 1.20.1.
- Active Ledger now has 50 records (2401–2450), every entry exactly 15 fields. Index has 2,450 sequential unique paths and matching Decisions.
- Tests/examples inspected only, none executed. No application source changed, no build/APK/device/runtime validation. No backup at 2450.
- Section 2401–2450 CLOSED (50/50). Continue 2451–2475 under existing 2401–2500 grant; report at 2500 and STOP.
