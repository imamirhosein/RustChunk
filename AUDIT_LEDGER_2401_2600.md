# RustChunk Starling Audit Ledger — Entries 2401–2600

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Active stage: **2401–2600**; completed entries **2401–2575 (175/200)**.
- Latest checkpoints **2526–2550** and **2551–2575** are complete and recorded below; active stage progress is **175/200**.
- The user has explicitly pre-authorized **2551–2600**. Checkpoint 2551–2575 is complete; continue with 2576–2600 without another permission request. **STOP at 2600**.
- Exact source is gitlink `upstream/starling` at `5432370d573642041585c69b40fb045b27133488`; do not follow the moving branch.
- New entries are selected in deterministic lexical path order after excluding all 2,550 Index paths; pinned recursive tree is complete and untruncated.
- Every entry has exactly the 15 required fields; source path, blob SHA and byte size are checked against the pinned tree.
- Tests/examples inspected only; none executed. No application source changed; no build, benchmark, APK, device, renderer, Minecraft, Forge, server or Aternos validation.

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

### 2451. `crates/horizon-world/src/datafix/fixes_b.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `708d266c8ffecbf5c9b0c020eac8787fb21fc3b1`
- Size: 55416 bytes
- Coverage: Complete through EOF: 1471 logical lines; consecutive ranges 1–400,401–800,801–1200,1201–1471; each <=400 lines.
- Purpose: Modern entity/item/world datafix registrations and migrations
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Registers ordered schema-version fixes for entities, item components, worldgen settings, attributes, structures and related saved data.
- Findings: Multiple fixes share version keys and order is semantic. UUID conversion helpers require both long halves nonzero; malformed/valid UUIDs with one zero half may be lost. Some unknown IDs collapse to fallback names.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Differential-test UUID half-zero cases, duplicate version registrations, unknown IDs and all migration steps with matching Java DataFixer versions; do not use post-1.20.1 fixes in target runtime.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1471 logical lines through EOF; pinned SHA and 55416-byte size verified; inspected, not modified; tests not executed.


### 2452. `crates/horizon-world/src/datafix/fixes_c.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9ef351d38a09b332f1ad782e7079b207611062f2`
- Size: 34539 bytes
- Coverage: Complete through EOF: 1043 logical lines; consecutive ranges 1–400,401–800,801–1043; each <=400 lines.
- Purpose: World/chunk/biome and entity datafixes for later schemas
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Registers world-gen settings, biome containers, chunk status, POI, structure and entity migration steps.
- Findings: Version-specific assumptions include fixed biome container layouts and old tick/height encodings; malformed lists may be replaced by defaults. Some keys are repeated across ordered substeps.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test chunk/biome layouts at every version boundary, missing sections, invalid palettes and preservation of unknown NBT.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1043 logical lines through EOF; pinned SHA and 34539-byte size verified; inspected, not modified; tests not executed.


### 2453. `crates/horizon-world/src/datafix/fixes_cliffs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c9861c4079ba3b71eb8877c79f46cee9294fc876`
- Size: 57632 bytes
- Coverage: Complete through EOF: 1623 logical lines; consecutive ranges 1–400,401–800,801–1200,1201–1623; each <=400 lines.
- Purpose: Chunk/height/biome/structure migration around 1.18-era world changes
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Migrates old chunk levels, heightmaps, biome containers, carving masks, structures, ticks and status metadata.
- Findings: 1623 lines read 1–400/401–800/801–1200/1201–1623. Tick conversion skips packed position <=0, which may discard valid local origin (0,0,0); confirm native semantics. Coordinate differences use abs and may hit i32::MIN edge behavior.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test origin tick, negative chunk coords, extreme coordinate deltas, packed biome/heightmap boundaries and old/new section formats against Java.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1623 logical lines through EOF; pinned SHA and 57632-byte size verified; inspected, not modified; tests not executed.


### 2454. `crates/horizon-world/src/datafix/fixes_flat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `29f24b0efa2b9ac0e16734d195545050c5520051`
- Size: 42176 bytes
- Coverage: Complete through EOF: 1224 logical lines; consecutive ranges 1–400,401–600,601–800,801–1200,1201–1224; each <=400 lines.
- Purpose: Flat-world, structure, item and block-name legacy migrations
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Registers older DataFixer steps for flat generators, block/item renames, chunk layouts, structures and legacy metadata.
- Findings: 1224 lines read in 1–400/401–600/601–800/801–1200/1201–1224. Unknown flat structure names can fall back to a default config; palette and biome conversions assume known version schemas.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test unknown structure IDs, malformed flat-layer strings, chunk palettes and migration boundaries; keep legacy mappings separate from 1.20.1 registry.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1224 logical lines through EOF; pinned SHA and 42176-byte size verified; inspected, not modified; tests not executed.


### 2455. `crates/horizon-world/src/datafix/fixes_legacy.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2bb5f08a9664a52471e8ba682ca109c3dc372a09`
- Size: 28146 bytes
- Coverage: Complete through EOF: 787 logical lines; consecutive ranges 1–400,401–787; each <=400 lines.
- Purpose: Legacy pre-flattening item/entity/block-entity and UUID migrations
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Handles old entity IDs, equipment, block IDs, spawn eggs, item stacks, UUID strings, signs, inventories and block-entity layouts.
- Findings: 787 lines read 1–400/401–787. UUID parser accepts variable component widths and masks overflow rather than enforcing canonical UUID groups; recursive Riding chains can exhaust stack on adversarial depth.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test malformed UUID widths/overflow, deeply nested Riding chains, unknown entity/block-entity IDs and exact 1.12 conversion behavior.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 787 logical lines through EOF; pinned SHA and 28146-byte size verified; inspected, not modified; tests not executed.


### 2456. `crates/horizon-world/src/datafix/fixes_mid.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `54a4f51c0af5f830e3955b560997d2adb9f19cf8`
- Size: 62365 bytes
- Coverage: Complete through EOF: 1815 logical lines; consecutive ranges 1–400,401–800,801–1200,1201–1600,1601–1815; each <=400 lines.
- Purpose: Mid-era chunk, entity, biome, palette and packed-storage datafixes
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Migrates block/biome containers, UUIDs, entity tags, worldgen settings, heightmaps and packed bit-storage formats.
- Findings: 1815 lines read 1–400/401–800/801–1200/1201–1600/1601–1815. UUID-from-longs returns None when either half is zero; may drop valid UUIDs. Packed-storage alignment and boundary behavior need exact 1.18-era oracle tests.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test UUID half-zero values, non-power-of-two bit widths, packed-long final-word boundaries, empty/short arrays and chunk palette round trips.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1815 logical lines through EOF; pinned SHA and 62365-byte size verified; inspected, not modified; tests not executed.


### 2457. `crates/horizon-world/src/datafix/fixes_old.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `82a5a62f3a4395be4d5bd0276ff9f9e8d9d01ccf`
- Size: 24301 bytes
- Coverage: Complete through EOF: 714 logical lines; consecutive ranges 1–400,401–714; each <=400 lines.
- Purpose: Old DataFixer migrations for entities, signs, spawners and world data
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Implements early fixes for entity names/UUIDs, block entities, riding/passengers, spawn data, stats, POI and old chunk metadata.
- Findings: 714 lines read 1–400/401–714. Several conversions silently default unknown enum/variant values; recursive entity tree transformations assume finite depth and well-formed compounds.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test malformed legacy tags, unknown enum values, nested passenger depth and id/version boundaries with Java oracle.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 714 logical lines through EOF; pinned SHA and 24301-byte size verified; inspected, not modified; tests not executed.


### 2458. `crates/horizon-world/src/datafix/flat_tables.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cea46f64c62f6112ddbc0f0848abb91aac757878`
- Size: 97674 bytes
- Coverage: Complete through EOF: 2791 logical lines; consecutive ranges 1–400,401–800,801–1200,1201–1600,1601–2000,2001–2200,2201–2400,2401–2791; each <=400 lines.
- Purpose: Static legacy flattening, block/item/entity and biome mapping tables
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Provides static lookup tables used by legacy block/item/entity/biome and statistics datafixers.
- Findings: 2791 lines checked in <=400-line ranges; static tuple tables parsed for duplicate keys, none found in checked tuple maps. Contains apparent `minecraft:enhanting_table` misspelling in legacy item-to-block-entity mapping; verify if source historical typo or broken lookup.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Verify enchanting-table mapping against original Java tables and test all legacy aliases/IDs; preserve historical order and do not use as target-version registry.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 2791 logical lines through EOF; pinned SHA and 97674-byte size verified; inspected, not modified; tests not executed.


### 2459. `crates/horizon-world/src/datafix/items.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a5ce49972fbd2b253257b008fceb8740f879c79a`
- Size: 46538 bytes
- Coverage: Complete through EOF: 1411 logical lines; consecutive ranges 1–400,401–600,601–800,801–1000,1001–1200,1201–1411; each <=400 lines.
- Purpose: ItemStack, item-name, component, profile and map-decoration datafixes
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Converts legacy stacks/tags into modern component patches, migrates books, banners, block predicates, maps, equipment, containers and entity/block-entity payloads.
- Findings: 1411 lines read in 1–400/401–600/601–800/801–1000/1001–1200/1201–1411. Malformed map-decoration IDs can fall back to empty key instead of leaving stack untouched; typo `enhanting_table` appears in legacy mapping path.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test malformed map decorations, unknown component IDs, duplicate map keys, item count/damage edge cases and enchanting-table item mapping against Java.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 1411 logical lines through EOF; pinned SHA and 46538-byte size verified; inspected, not modified; tests not executed.


### 2460. `crates/horizon-world/src/datafix/json.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4cab8ff1d2e4ae0e9c1ca054b17737efbf8d1551`
- Size: 20988 bytes
- Coverage: Complete through EOF: 644 logical lines; consecutive ranges 1–400,401–644; each <=400 lines.
- Purpose: Gson-compatible JSON parser, stable writer and JSON/NBT conversion
- Dependencies / cross-references: horizon-world DataFixer schemas, NBT, region files, JSON, registry snapshots and historical version tables.
- Implementation / logic: Implements strict/lenient JSON reading, UTF-16 strings, numeric exact-long parsing, stable key ordering and NBT conversion.
- Findings: 644 lines read 1–400/401–644. Strict quoted parser appears to accept escaped apostrophe; UTF-16 escape conversion rejects lone surrogates; lenient top-level null may return early despite trailing input. Exact-long exponent arithmetic has extreme i64 overflow risk.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test strict/lenient grammar, lone surrogate escapes, trailing tokens after null, extreme exponents and Java Gson/NbtOps parity.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 644 logical lines through EOF; pinned SHA and 20988-byte size verified; inspected, not modified; tests not executed.


### 2461. `crates/horizon-world/src/datafix/progress.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `de254d8c3e43cbc9cba7c5824362ecd516744118`
- Size: 9344 bytes
- Coverage: Complete through EOF: 282 logical lines; full file; no gaps.
- Purpose: Concurrent world-upgrade progress, cancellation and file-fixer statistics
- Dependencies / cross-references: DataFixer schema walkers, progress/cancellation, text component migration and world-upgrader filesystem pipeline.
- Implementation / logic: Stores atomic totals/status/type/progress and per-dimension progress behind Mutex for UI polling.
- Findings: 282 lines; relaxed atomics are used consistently for independent counters; poisoned dimension mutex silently drops updates and reads as zero.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test concurrent polling, cancellation visibility, overflow and poisoned-mutex fallback with Java screen semantics.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 282 logical lines through EOF; pinned SHA and 9344-byte size verified; inspected, not modified; tests not executed.


### 2462. `crates/horizon-world/src/datafix/schema_flat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `12730c255521ac21c330e3dfec7ed346e59bcac6`
- Size: 20043 bytes
- Coverage: Complete through EOF: 508 logical lines; consecutive ranges 1–400,401–508; each <=400 lines.
- Purpose: Pre-1.14 DataFixer schema walkers and untyped entity-tree marking
- Dependencies / cross-references: DataFixer schema walkers, progress/cancellation, text component migration and world-upgrader filesystem pipeline.
- Implementation / logic: Walks legacy entity/block-entity/chunk/spawner schemas and marks unknown trees to preserve untyped data.
- Findings: 508 lines read 1–400/401–508. MARKING thread-local bool is set/reset manually; panic during walk can leave it true for later work on same thread. Nested mark_untyped calls can also clear outer state.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Use RAII/thread-local nesting counter; test panic recovery, nested calls and unknown entity-list preservation.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 508 logical lines through EOF; pinned SHA and 20043-byte size verified; inspected, not modified; tests not executed.


### 2463. `crates/horizon-world/src/datafix/schema.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9f5ab287a4c7f8bb360e47b894ec0ac6d048ce75`
- Size: 37580 bytes
- Coverage: Complete through EOF: 947 logical lines; consecutive ranges 1–400,401–600,601–800,801–947; each <=400 lines.
- Purpose: Versioned DataFixer schema registry and typed traversal
- Dependencies / cross-references: DataFixer schema walkers, progress/cancellation, text component migration and world-upgrader filesystem pipeline.
- Implementation / logic: Defines entity/item/block-entity/structure/biome/component schema traversal and version-gated known-ID tables.
- Findings: 947 lines read 1–400/401–600/601–800/801–947. Schema registration and repeated key order are critical; recursive tree walking assumes finite NBT depth. Unknown IDs intentionally remain untyped.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test all schema version cutovers, unknown IDs, deeply nested NBT, component/tag renames and DataVersion transitions.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 947 logical lines through EOF; pinned SHA and 37580-byte size verified; inspected, not modified; tests not executed.


### 2464. `crates/horizon-world/src/datafix/text.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `56c4e8da8261a92477c8e48681c4267ba1aca0b5`
- Size: 8042 bytes
- Coverage: Complete through EOF: 252 logical lines; full file; no gaps.
- Purpose: 1.21.5 text-component and hover/click event migrations
- Dependencies / cross-references: DataFixer schema walkers, progress/cancellation, text component migration and world-upgrader filesystem pipeline.
- Implementation / logic: Unflattens JSON text strings, rewrites legacy hover events, stringifies boolean style flags and changes hover/click fields.
- Findings: 252 lines; URI validator approximates java.net.URI and chat-character filter uses UTF-16 units; malformed legacy hover payloads are converted to display text rather than rejected.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test URI authority/escape edge cases, lone surrogates, malformed hover actions and 1.21.5 schema behavior.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 252 logical lines through EOF; pinned SHA and 8042-byte size verified; inspected, not modified; tests not executed.


### 2465. `crates/horizon-world/src/datafix/world_upgrader_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a8bc69d9ac7085c0a691748ea5a28f6e9a2ae61c`
- Size: 10245 bytes
- Coverage: Complete through EOF: 298 logical lines; full file; no gaps.
- Purpose: World-upgrader region, progress, cancellation and cache-erasure tests
- Dependencies / cross-references: DataFixer schema walkers, progress/cancellation, text component migration and world-upgrader filesystem pipeline.
- Implementation / logic: Builds temporary worlds/region files and tests conversion, skip, POI defaults, cache removal, recreation, cancellation and thread join.
- Findings: 298 lines; test explicitly confirms a current/skipped chunk short-circuits the rest of that region-file pass, so an older later chunk remains unconverted. Mixed-version chunks can therefore leave stale data unless native behavior is intentional.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Validate mixed-version chunks in one region and compare Java loop semantics; add regression test ensuring every old chunk is visited if required.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 298 logical lines through EOF; pinned SHA and 10245-byte size verified; inspected, not modified; tests not executed.


### 2466. `crates/horizon-world/src/dynamic_support.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5a3e12afd84312d8a11aadbd5abd2e16867650e9`
- Size: 9917 bytes
- Coverage: Complete through EOF: 254 logical lines; full file; no gaps.
- Purpose: Dynamic support/collision shapes for bamboo, dripstone, scaffolding, powder snow, shulkers and moving pistons
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Selects state-specific shape, position-seeded offsets and live block-entity/piston callbacks; exposes support and collision shape paths.
- Findings: Shulker collision uses snapshot bounding box while support uses open/closed state callback; Direction discriminants index fixed six-element shape array.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test all six directions, open/closed snapshots, position-seeded offsets and moving-piston absence with Java 1.20.1.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 254 logical lines through EOF; pinned SHA and 9917-byte size verified; inspected, not modified; tests not executed.


### 2467. `crates/horizon-world/src/edge_backoff.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4303b2d7436977993c53c24b884f060a45d633ee`
- Size: 3609 bytes
- Coverage: Complete through EOF: 108 logical lines; full file; no gaps.
- Purpose: Player/self crouch edge-backoff movement reduction
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Reduces horizontal movement in 0.05 increments while collision probes indicate unsupported ground.
- Findings: 108 lines; max_up_step is not validated for negative/NaN values; query budget exhaustion returns error after earlier world callbacks have already occurred.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test zero/negative/NaN step, all mover types, diagonal reduction, budget exhaustion and exact collision probe order.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 108 logical lines through EOF; pinned SHA and 3609-byte size verified; inspected, not modified; tests not executed.


### 2468. `crates/horizon-world/src/effect_attribute_modifiers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `53aaac1629f0a72ddda2734b4fdb020480728162`
- Size: 4740 bytes
- Coverage: Complete through EOF: 139 logical lines; full file; no gaps.
- Purpose: Status-effect attribute modifier template creation and reentrant traversal
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Adds/removes permanent modifiers through ObjectOpenMap cursor while callbacks may mutate definitions.
- Findings: Re-reads indexed entry after remove callback in add_modifiers; remove_modifiers does not re-read because it has no later use. Cursor invalidation is surfaced explicitly.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test callback replacement/rehash, removal errors, amplifier overflow and identity/order parity.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 139 logical lines through EOF; pinned SHA and 4740-byte size verified; inspected, not modified; tests not executed.


### 2469. `crates/horizon-world/src/effect_notifications.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cea64b21de47c4d999685e00fe8845d00de5f2cf`
- Size: 8779 bytes
- Coverage: Complete through EOF: 246 logical lines; full file; no gaps.
- Purpose: Effect eligibility, attribute refresh and server-player passenger notifications
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Implements immunity-tag precedence, update/remove packets, attribute modifier hooks and pending-update traversal.
- Findings: Immunity tags are exclusive priority order (Infested before Oozing before Poison/Regeneration). Packet send loops query current entity ID per passenger; stable connection captured before virtual call.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test overlapping immunity tags, passenger list mutation, callback errors, attribute-set rehash and packet ordering.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 246 logical lines through EOF; pinned SHA and 8779-byte size verified; inspected, not modified; tests not executed.


### 2470. `crates/horizon-world/src/effect_particles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `60494dcf062dfe293d161060a6e9b64faecb25a1`
- Size: 4553 bytes
- Coverage: Complete through EOF: 125 logical lines; full file; no gaps.
- Purpose: Registered status-effect particle factories and option identity
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Loads versioned particle catalog; creates fresh color particles and reuses registered constant particle Arc identity.
- Findings: Catalog validates DataVersion and ambient alpha; equality is pointer identity, so fresh equal-color options remain unequal by design.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test duplicate IDs, invalid colors, constant identity reuse and synchronized-list equality with Java.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 125 logical lines through EOF; pinned SHA and 4553-byte size verified; inspected, not modified; tests not executed.


### 2471. `crates/horizon-world/src/effect_sync.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `50d8c61342843a7dfa590caf752490f457ca56e4`
- Size: 6721 bytes
- Coverage: Complete through EOF: 208 logical lines; full file; no gaps.
- Purpose: Effect particle, ambient, invisibility and glowing synchronized state
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Builds particle list, computes all-ambient, updates shared flags and mirrors client/server glowing logic.
- Findings: Flag reads sign-extend u8 through i8 before bit tests; shift uses wrapping_shl to mirror Java int shift masking. Empty effects clear particles/invisibility but ambient flag is not rewritten on that branch.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test signed high bits, shift counts, empty-effect ambient state and repeated virtual level/client queries.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 208 logical lines through EOF; pinned SHA and 6721-byte size verified; inspected, not modified; tests not executed.


### 2472. `crates/horizon-world/src/entity_collision.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8ef42b648946f81a2f12d086a2fb38b09786560f`
- Size: 2694 bytes
- Coverage: Complete through EOF: 77 logical lines; full file; no gaps.
- Purpose: EntityGetter collision-shape selection from ordered entity snapshot
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Inflates AABB, filters source/spectator/nonintersecting candidates and asks provider collision policy in native order.
- Findings: No explicit candidate budget; broadphase may return supersets, but snapshot handles must remain available through callbacks.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test source-null semantics, vehicle relationships, candidate ordering and pathological dense entity sets.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 77 logical lines through EOF; pinned SHA and 2694-byte size verified; inspected, not modified; tests not executed.


### 2473. `crates/horizon-world/src/entity_fluid_tick.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `22e6fd4d26ce3c67acab376b993aa8104b43f6fb`
- Size: 2383 bytes
- Coverage: Complete through EOF: 62 logical lines; full file; no gaps.
- Purpose: Entity water-entry splash, fall reset and water/lava current application
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Runs fluid interaction update, captures water/lava membership, handles splash transitions and applies current forces.
- Findings: is_pushed_by_fluid queried twice by design; water/lava membership captured before splash callbacks, preserving native phase ordering.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test callback mutation between the two push checks, splash transitions, fast-lava and current ordering.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 62 logical lines through EOF; pinned SHA and 2383-byte size verified; inspected, not modified; tests not executed.


### 2474. `crates/horizon-world/src/entity_fluid.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a4c64e357070dcf09f8aa52abf8f91da38444932`
- Size: 16671 bytes
- Coverage: Complete through EOF: 454 logical lines; consecutive ranges 1–400,401–454; each <=400 lines.
- Purpose: Entity fluid immersion trackers, loaded-chunk checks and current accumulation
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Scans loaded chunks/sections in Z-X-Y order, tracks fluid heights/eyes, maps current to first matching tag and applies player average vs mob direction.
- Findings: 454 lines read 1–400/401–454. Query budget counts chunk, section and fluid cells; tracker/current writes persist if later provider call fails. Float NaN behavior uses Java-compatible max helper.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test missing chunks, negative sections, NaN surfaces, overlapping fluid tags, query-budget exhaustion and partial-state semantics.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 454 logical lines through EOF; pinned SHA and 16671-byte size verified; inspected, not modified; tests not executed.


### 2475. `crates/horizon-world/src/entity_move.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `25d7aef14bfd8a4c17a6212ebd697dd9afd9cefe`
- Size: 8093 bytes
- Coverage: Complete through EOF: 214 logical lines; full file; no gaps.
- Purpose: Entity.move collision, support, fall reset, movement history and emission ordering
- Dependencies / cross-references: horizon-world entity, effect, collision, movement, fluid, attribute and dynamic support callbacks; horizon-core geometry.
- Implementation / logic: Coordinates piston limits, stuck multiplier, edge backoff, collision resolution, position/history, flags, fall damage, restitution and movement emission.
- Findings: No-physics branch sets position/record/flags then returns without later fall/ground/emission callbacks. MovementHistory compresses oldest two at 100 entries; exact +0.0 check distinguishes negative zero.
- RustChunk relevance: Reference/core architecture for Rust Android Minecraft engine; upstream snapshot targets 26.3 and is not a drop-in 1.20.1 implementation.
- Decision: adaptable; reference
- 1.20.1 implications: Validate against vanilla 1.20.1 and Forge 47.x. Later-version DataFixer tables/migrations must not be imported into the 1.20.1 runtime or save upgrader blindly.
- Follow-up checks: Test no-physics callback order, negative-zero movement, 100-entry history compression, piston/shulker movers and client authority.
- RustChunk implementation mapping: Port only version-neutral algorithms behind RustChunk-owned 1.20.1 interfaces; keep DataFixer/migration data version-pinned and outside ordinary gameplay runtime.
- Evidence required before adoption: Java 1.20.1 sources/data, version-hashed inputs, malformed and differential test corpus, Android ARM64 runtime evidence and Forge 47.x integration.
- Completion state: COMPLETE — 214 logical lines through EOF; pinned SHA and 8093-byte size verified; inspected, not modified; tests not executed.


## Checkpoint 2451–2475 — COMPLETE (25/25)

- All 25 files inspected individually through EOF; paths, SHAs and sizes match pinned source tree.
- Long-file ranges <=400: fixes_b.rs 1–400/401–800/801–1200/1201–1471; fixes_c.rs 1–400/401–800/801–1043; fixes_cliffs.rs 1–400/401–800/801–1200/1201–1623; fixes_flat.rs 1–400/401–600/601–800/801–1200/1201–1224; fixes_legacy.rs 1–400/401–787; fixes_mid.rs 1–400/401–800/801–1200/1201–1600/1601–1815; fixes_old.rs 1–400/401–714; flat_tables.rs 1–400/401–800/801–1200/1201–1600/1601–2000/2001–2200/2201–2400/2401–2791; items.rs 1–400/401–600/601–800/801–1000/1001–1200/1201–1411; json.rs 1–400/401–644; schema_flat.rs 1–400/401–508; schema.rs 1–400/401–600/601–800/801–947; entity_fluid.rs 1–400/401–454.
- Key risks: DataFixer migration files span multiple historical and post-1.20.1 versions; UUID half-zero handling; possible loss of origin tick; apparent enchanting-table typo; malformed JSON/UTF-16 compatibility; manual thread-local mark reset; current-chunk short-circuit in region-upgrader tests; entity/fluid callbacks and movement need Java parity.
- Active Ledger now has **75 records (2401–2475)**, each with exactly 15 fields. Index has **2,475 sequential unique paths** and matching Decisions.
- Tests/examples inspected only, none executed. No application source changed, no build/APK/device/runtime validation. No backup at 2475.
- Continue with **2476–2500** under the same one-time authorization; report and STOP at 2500.

### 2476. `crates/horizon-world/src/entity_obstruction.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aac2306b65193e71be487eb7d6cad57bd27cdea5`
- Size: 2524 bytes
- Coverage: Complete through EOF: 75 logical lines; full file; no gaps.
- Purpose: Entity obstruction and suffocation query adapter
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Queries collision/obstruction state around entity bounds and delegates entity/world-specific predicates.
- Findings: AABB query and entity-state callbacks are ordered; no explicit candidate budget or unloaded-chunk policy in this adapter.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test unloaded sections, moving shapes, large entities and callback order against Java 1.20.1.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 75 logical lines through EOF; pinned SHA and 2524-byte size verified; inspected, not modified; tests not executed.


### 2477. `crates/horizon-world/src/extra_items.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `69c2f04927ffa7492d9e87d54416ab54e75db4e7`
- Size: 925 bytes
- Coverage: Complete through EOF: 26 logical lines; full file; no gaps.
- Purpose: Additional item IDs and small registry helpers
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Defines compact extra item mappings used by item/entity interactions.
- Findings: Static identifiers are version-bound and assume registry IDs are supplied by matching data.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Verify every identifier and registry binding against 1.20.1.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 26 logical lines through EOF; pinned SHA and 925-byte size verified; inspected, not modified; tests not executed.


### 2478. `crates/horizon-world/src/fall_damage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d3a1dca4265ec7ea9c60a293759a8d0b12d4d4a2`
- Size: 1826 bytes
- Coverage: Complete through EOF: 49 logical lines; full file; no gaps.
- Purpose: Fall distance, damage and landing reset arithmetic
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Computes fall damage and applies landing-related reset/mitigation callbacks.
- Findings: Float NaN/infinity and negative fall distance follow Rust/Java comparison/cast behavior; callback ordering controls damage and reset.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test NaN/infinite/negative distance, water/hay/slime/bounce and exact damage formula with Java 1.20.1.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 49 logical lines through EOF; pinned SHA and 1826-byte size verified; inspected, not modified; tests not executed.


### 2479. `crates/horizon-world/src/fluid_box.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5632b652988680dba02ee25632fba22571765608`
- Size: 3808 bytes
- Coverage: Complete through EOF: 126 logical lines; full file; no gaps.
- Purpose: Fluid voxel boxes and surface-height geometry
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Builds per-fluid shape boxes and derives height/surface from amount and falling state.
- Findings: Assumes amount 0–8 and canonical fluid shape registry; callers can construct invalid FluidState unless constructor validates.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test every amount/falling combination, empty/source/flowing states and shape bounds.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 126 logical lines through EOF; pinned SHA and 3808-byte size verified; inspected, not modified; tests not executed.


### 2480. `crates/horizon-world/src/fluid_flow.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `272a51dae2831a85d29f6d16b44ad5fe5b05adc7`
- Size: 3743 bytes
- Coverage: Complete through EOF: 108 logical lines; full file; no gaps.
- Purpose: Fluid horizontal flow direction and vector accumulation
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Computes neighbor-level gradients, falling-fluid influence and normalized flow vector.
- Findings: Neighbor reads and fluid-family comparisons are callback-driven; tie and zero-vector handling need exact float parity.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test all direction ties, falling/source boundaries, same-family tags and exact float bits.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 108 logical lines through EOF; pinned SHA and 3743-byte size verified; inspected, not modified; tests not executed.


### 2481. `crates/horizon-world/src/fluid_occupancy.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4432eca3b3337aed1935ce4a2918eeaf14adff45`
- Size: 5690 bytes
- Coverage: Complete through EOF: 140 logical lines; full file; no gaps.
- Purpose: Fluid presence in entity AABB and loaded chunk/section scans
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Bounds the scan to loaded chunks/sections, samples fluid states and checks tags/height intersection.
- Findings: Uses state IDs as indexes into registry presence arrays; invalid IDs can panic. Chunk/section iteration and query budget are callback-sensitive.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test negative sections, chunk borders, unloaded chunks, invalid state IDs and max-Y edges.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 140 logical lines through EOF; pinned SHA and 5690-byte size verified; inspected, not modified; tests not executed.


### 2482. `crates/horizon-world/src/fluid_shape.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7df0a3b2d513ef48346dd2b85c27a49a380e8256`
- Size: 1707 bytes
- Coverage: Complete through EOF: 47 logical lines; full file; no gaps.
- Purpose: Precomputed fluid shape cache
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Creates indexed voxel shapes for fluid kind, falling flag and amount.
- Findings: Indexing assumes amount <=8; invalid states can exceed the fixed table.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Add constructor/range validation and test all cache indices.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 47 logical lines through EOF; pinned SHA and 1707-byte size verified; inspected, not modified; tests not executed.


### 2483. `crates/horizon-world/src/fluid.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ab03d6a52926cf2a982a9d0bcc76ed2f661fa333`
- Size: 10855 bytes
- Coverage: Complete through EOF: 322 logical lines; full file; no gaps.
- Purpose: Fluid state parsing, registry bindings, height and family semantics
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Decodes fluid state properties, maps block states to fluid amount/falling and exposes same-family/tag/shape behavior.
- Findings: FluidState::new is public and accepts any u8 amount; FluidShapeCache indexes directly from it, so amount >8 can panic/out-of-bounds. from_json also defaults missing falling to true for nonempty fluid; verify default-state semantics.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Validate amount 0–8 at construction, empty/source invariants and missing-property defaults; compare all fluid states with Java 1.20.1.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 322 logical lines through EOF; pinned SHA and 10855-byte size verified; inspected, not modified; tests not executed.


### 2484. `crates/horizon-world/src/food_properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `284179237984d44a6e065fd6d766f54b104e368d`
- Size: 2866 bytes
- Coverage: Complete through EOF: 86 logical lines; full file; no gaps.
- Purpose: Food component properties, effects and nutrition access
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Reads food nutrition/saturation/always-eat/effect entries from item components.
- Findings: Effect probability and numeric values are data-driven; malformed component shapes rely on decoder defaults/errors.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test fractional nutrition, NaN probability, effect ordering and missing/default components.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 86 logical lines through EOF; pinned SHA and 2866-byte size verified; inspected, not modified; tests not executed.


### 2485. `crates/horizon-world/src/food_storage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0401e8158b05e32659ff7856d567875fe9f2829a`
- Size: 1990 bytes
- Coverage: Complete through EOF: 47 logical lines; full file; no gaps.
- Purpose: FoodData hunger, saturation and exhaustion storage
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Stores hunger/saturation/exhaustion/tick values and applies bounded food updates.
- Findings: Float comparisons and exhaustion carry are compatibility-sensitive; fields are public and may be externally inconsistent.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test saturation caps, exhaustion thresholds, negative values and tick overflow.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 47 logical lines through EOF; pinned SHA and 1990-byte size verified; inspected, not modified; tests not executed.


### 2486. `crates/horizon-world/src/food.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `77c931d6f0ef710c78517d9901286ee586529e79`
- Size: 4590 bytes
- Coverage: Complete through EOF: 135 logical lines; full file; no gaps.
- Purpose: Food consumption, nutrition, effects and statistics orchestration
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Applies food properties to player food data, emits effect/stat/criterion callbacks and handles always-eat.
- Findings: Effect RNG/callback order and food-level checks are external hooks; a callback error may leave earlier food mutation committed.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test full hunger/always-eat, effect failure, RNG order and server/client statistic order.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 135 logical lines through EOF; pinned SHA and 4590-byte size verified; inspected, not modified; tests not executed.


### 2487. `crates/horizon-world/src/free_space.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0d0b48d0fdc7255eb37245f255312f77a8f305fd`
- Size: 1495 bytes
- Coverage: Complete through EOF: 43 logical lines; full file; no gaps.
- Purpose: Entity free-space and spawn-position collision check
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Tests candidate entity bounds against block/entity collision provider.
- Findings: Only supplied candidate positions and collision provider policy are covered; no chunk-loading guarantee.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test boundary touching, fluids, entities and unloaded chunk policy.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 43 logical lines through EOF; pinned SHA and 1495-byte size verified; inspected, not modified; tests not executed.


### 2488. `crates/horizon-world/src/ground_support.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3d761d2d5088cea4bca7694022cbefe9e02de29`
- Size: 3583 bytes
- Coverage: Complete through EOF: 98 logical lines; full file; no gaps.
- Purpose: Entity ground support, stepping and sturdy-face checks
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Combines on-ground/step state, below-block shape and collision support callbacks.
- Findings: Ground support may use a snapshot of entity position and block state; moving platform and unloaded-chunk behavior remain external.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test step-height, partial shapes, moving pistons and entity position mutation.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 98 logical lines through EOF; pinned SHA and 3583-byte size verified; inspected, not modified; tests not executed.


### 2489. `crates/horizon-world/src/heightmap.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `517902dbbb8759b9972a03eafb5956b16ee9f46e`
- Size: 9321 bytes
- Coverage: Complete through EOF: 280 logical lines; full file; no gaps.
- Purpose: Packed chunk heightmap types, updates, reads and NBT serialization
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Maintains motion-blocking/world-surface/ocean-floor heightmaps using packed long arrays and block predicates.
- Findings: Packed bit width and section/minY offsets are version-sensitive; malformed or short long arrays may decode with fallback values rather than hard error.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test negative minY, section boundaries, non-power-of-two packing, empty sections and round-trip NBT with Java 1.20.1.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 280 logical lines through EOF; pinned SHA and 9321-byte size verified; inspected, not modified; tests not executed.


### 2490. `crates/horizon-world/src/identifier.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9c335e613071bef39c58d615abfbef08165477a2`
- Size: 3148 bytes
- Coverage: Complete through EOF: 103 logical lines; full file; no gaps.
- Purpose: Resource identifier parsing, validation and canonical formatting
- Dependencies / cross-references: horizon-world entity/fluid/food/heightmap/collision registries and world callbacks; horizon-core geometry/random.
- Implementation / logic: Parses optional namespace, validates namespace/path character sets and emits canonical IDs.
- Findings: Validation must match Java ResourceLocation exactly; empty path and uppercase handling are compatibility-sensitive.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test empty namespace/path, uppercase, multiple colons, Unicode, slash and invalid characters against Java 1.20.1.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 103 logical lines through EOF; pinned SHA and 3148-byte size verified; inspected, not modified; tests not executed.


### 2491. `crates/horizon-world/src/interaction_shapes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `faf7b84a3c15f96429345eacf8939f48a8aa875b`
- Size: 1408 bytes
- Coverage: Complete through EOF: 36 logical lines; full file; no gaps.
- Purpose: Complete per-state interaction-shape registry
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Loads interaction shape table through CachedCollisionRegistry and requires canonical identities and every state bound.
- Findings: Strictly rejects partial tables; shape lookup returns error for missing state. No contextual interaction shapes are supported by design.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Validate all 1.20.1 states and compare ray-hit faces with vanilla interaction shapes.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 36 logical lines through EOF; pinned SHA and 1408-byte size verified; inspected, not modified; tests not executed.


### 2492. `crates/horizon-world/src/inventory_pickup.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8d9545cd339628cf724cdf3eeb854c7c7ba20db9`
- Size: 5167 bytes
- Coverage: Complete through EOF: 159 logical lines; full file; no gaps.
- Purpose: Player inventory pickup, stack merge and infinite-material behavior
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Finds selected/offhand/main slots, merges compatible stacks, copies damaged items and consumes source stack in place.
- Findings: Return value after partial pickup appears wrong: loop exits on final no-progress iteration and compares against that iteration's count, so a partially inserted stack can return false. Damaged stack is copy-cleared before target slot validation.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test partial pickup across multiple slots, final no-space remainder, invalid explicit slot, damaged item and infinite materials; compare Java return value and mutation order.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 159 logical lines through EOF; pinned SHA and 5167-byte size verified; inspected, not modified; tests not executed.


### 2493. `crates/horizon-world/src/inventory_tick.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aaa75a63910db998b8de7f43ba4fcd8ab75bb42c`
- Size: 2641 bytes
- Coverage: Complete through EOF: 87 logical lines; full file; no gaps.
- Purpose: Inventory and equipment item ticking
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Ticks pop-time, invokes item tick only for server level and walks inventory/equipment in native order.
- Findings: Level is fetched separately for each nonempty stack; equipment reads each entry when reached to preserve callback mutation visibility.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test callback changes to later equipment slots, client/server split and level lookup count/order.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 87 logical lines through EOF; pinned SHA and 2641-byte size verified; inspected, not modified; tests not executed.


### 2494. `crates/horizon-world/src/inventory_use.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d37ea341a798de0d1754074e554488f9844897e9`
- Size: 14957 bytes
- Coverage: Complete through EOF: 485 logical lines; ranges 1–400, 401–485, each <=400 lines.
- Purpose: Living item-use lifecycle bound to inventory, synchronized flags and callbacks
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Binds UseLifecycleWorld and StartUseWorld to shared StackRef, inventory and synced flag storage; handles use/release/finish/start, equipment and client synchronization.
- Findings: 485 lines read 1–400/401–485. set_hand_item mutates inventory before on_equip callback; flags observer errors leave the flag mutation committed. Reentrancy and same-stack aliasing are core contracts.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test callback failure/reentry at every flag/equipment/use boundary, client sync hand changes, stack replacement and exact event order.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 485 logical lines through EOF; pinned SHA and 14957-byte size verified; inspected, not modified; tests not executed.


### 2495. `crates/horizon-world/src/item_after_use.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `01b4d84df7e95e93b339adeb0cf0dd9f88a5b22c`
- Size: 2548 bytes
- Coverage: Complete through EOF: 69 logical lines; full file; no gaps.
- Purpose: Finish-use remainder and cooldown application
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Copies original stack before finish handler, derives remainder/cooldown from copy, applies remainder conversion then cooldown.
- Findings: Remainder conversion may mutate result and emit extra item before cooldown; callback failures preserve preceding mutations.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test finish handler replacement, remainder overflow/infinite materials and cooldown order.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 69 logical lines through EOF; pinned SHA and 2548-byte size verified; inspected, not modified; tests not executed.


### 2496. `crates/horizon-world/src/item_blocking.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d2561df5e6af9d402ac27cb46a7b5583a7385c8c`
- Size: 3094 bytes
- Coverage: Complete through EOF: 83 logical lines; full file; no gaps.
- Purpose: Item blocking component dispatch and blocked-damage side effects
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Checks item/component bypass, piercing arrows, source angle, damage, hand and attacker blocking callback.
- Findings: Direct entity is queried once for piercing and again after hurt; callback mutation may change entity identity between phases. Native virtual-call order must be preserved.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test direct-entity mutation, projectile state changes, angle fallback and blocked-item damage callback ordering.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 83 logical lines through EOF; pinned SHA and 3094-byte size verified; inspected, not modified; tests not executed.


### 2497. `crates/horizon-world/src/item_cooldowns.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6f2785c32f2f6fe6ffe6836c7270a7c59387619b`
- Size: 4388 bytes
- Coverage: Complete through EOF: 136 logical lines; full file; no gaps.
- Purpose: Item cooldown map, expiry, progress and group selection
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Stores start/end ticks in JavaHashMap, computes progress, expires through cursor and calls started/ended observers.
- Findings: Duration zero yields 0/0 NaN in percent; negative/overflowing duration relies on Java int wrap. Observer mutation may invalidate cursor.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test zero/negative duration, tick overflow, callback map mutation and Java HashMap iteration.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 136 logical lines through EOF; pinned SHA and 4388-byte size verified; inspected, not modified; tests not executed.


### 2498. `crates/horizon-world/src/item_defaults.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d15f6baef62968626e71f1a2a7de7fb156a808db`
- Size: 7661 bytes
- Coverage: Complete through EOF: 166 logical lines; full file; no gaps.
- Purpose: Vanilla and modded item default component loading
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Reads items.json, applies native ModifyDefaultComponents hook and sanitizes max stack size.
- Findings: stack_size uses as_f64 then casts to i64, truncating fractional JSON values (e.g. 2.9→2) although vanilla ExtraCodecs.intRange expects integer; verify and reject non-integral values.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test fractional, huge, negative and string stack sizes for data-pack and mod hooks against Java codec.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 166 logical lines through EOF; pinned SHA and 7661-byte size verified; inspected, not modified; tests not executed.


### 2499. `crates/horizon-world/src/item_durability.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9f48955fcc71e159e0c1f1cb9d1b2543e42e967c`
- Size: 5777 bytes
- Coverage: Complete through EOF: 191 logical lines; full file; no gaps.
- Purpose: ItemStack durability, damage, break and enchantment pipeline
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Models damageable stack, processes enchantment durability, updates damage, shrinks broken stack and emits callbacks.
- Findings: is_damageable requires explicit damage component Some, while vanilla may determine damageability from maxDamage>0 and !unbreakable and default missing Damage to zero. This could make pristine damageable items non-damageable.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test absent Damage component, default zero, unbreakable, maxDamage=0 and break callback order against Java 1.20.1.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 191 logical lines through EOF; pinned SHA and 5777-byte size verified; inspected, not modified; tests not executed.


### 2500. `crates/horizon-world/src/item_particles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `70a9e0f6bb36d1fac0f0e0693e2a251fffd77bd1`
- Size: 2672 bytes
- Coverage: Complete through EOF: 73 logical lines; full file; no gaps.
- Purpose: Eating item particle positions, velocities and random draw order
- Dependencies / cross-references: horizon-world inventory, item stack/component, cooldown, synced data and player equipment/lifecycle modules.
- Implementation / logic: Captures item template once, draws velocity/position, rotates by pitch/yaw and emits particles at eye position.
- Findings: Uses actor's direct random field rather than virtual getRandom; repeated pitch/yaw/position/eye/level callbacks per particle are order-sensitive.
- RustChunk relevance: Core Android Minecraft world/entity/inventory behavior; pinned upstream is 26.3 and needs 1.20.1-specific validation.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against vanilla 1.20.1 and Forge 47.x. Do not infer target compatibility from 26.3 implementation or resource data.
- Follow-up checks: Test exact RNG draws and callback order for multiple particles, stack mutation and client/server behavior.
- RustChunk implementation mapping: Port only behind RustChunk-owned 1.20.1 APIs; preserve shared stack identity, callback order, registry version and native-mod boundaries.
- Evidence required before adoption: Java 1.20.1 source/oracle, exact RNG/callback differential tests, malformed input tests and Android ARM64/Forge 47.x runtime validation.
- Completion state: COMPLETE — 73 logical lines through EOF; pinned SHA and 2672-byte size verified; inspected, not modified; tests not executed.


## Checkpoint 2476–2500 — COMPLETE (25/25); one-time 100-file authorization EXHAUSTED

- All 25 files inspected individually through EOF; exact path/SHA/size matches pinned tree.
- Long-file coverage: fluid.rs 1–322; entity_fluid.rs 1–400/401–454; inventory_use.rs 1–400/401–485.
- Important findings: FluidState::new accepts amount >8 but shape cache indexes a fixed 0–8 table; inventory pickup can report false after partial insertion; item default stack-size codec truncates fractions; item durability may require a damage component that vanilla defaults to zero; validate all with Java 1.20.1.
- Active Ledger now has **100 records (2401–2500)**, exactly 15 fields each. Index has **2,500 sequential unique paths** and matching Decisions.
- Tests/examples inspected only, none executed. No application source changed, no build/APK/device/runtime validation. No backup at 2500.
- Exceptional authorization 2401–2500 is fully consumed. **STOP at 2500**; entry 2501+ requires fresh explicit permission. Next backup only after 3000 and global verification.
### 2501. `crates/horizon-world/src/item_stack_refs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a516c93d661e37669aca5b0a6481c0d7d9d743d8`
- Size: 2518 bytes
- Coverage: Complete through EOF: 72 logical lines; full file; no gaps.
- Purpose: Shared-reference wrapper for mutable item-stack data.
- Dependencies / cross-references: ItemStackData, std::rc::Rc, std::cell::RefCell and the item-stack storage layer.
- Implementation / logic: Clones share one Rc<RefCell<ItemStackData>>; read/write expose borrow guards, same_reference tests pointer identity, and empty() reuses a shared empty allocation.
- Findings: The canonical empty allocation is shared while write() is exposed; an accidental mutation can contaminate every empty handle. Conflicting RefCell borrows panic, and Rc<RefCell> is neither Send nor Sync. Pointer identity must not be confused with value equality.
- RustChunk relevance: Useful for modelling mutable stack aliasing and callbacks that observe the same stack object; the ownership model must fit RustChunk's threading boundaries.
- Decision: adaptable; reference
- 1.20.1 implications: 1.20.1 ItemStack semantics predate the 1.20.5 component system used by this source. Preserve identity only where Java callers rely on it; do not import the 26.3 component payload.
- Follow-up checks: Prove the empty sentinel cannot be mutated through any public path; test aliased read/write, re-entrant callback borrow conflicts, value equality versus reference equality, and cross-thread ownership requirements.
- RustChunk implementation mapping: Implement a RustChunk-owned stack handle with explicit identity and guarded mutation; keep the empty sentinel immutable or return a protected empty value. Select thread-confined Rc or synchronized Arc only after defining the client threading model.
- Evidence required before adoption: Java 1.20.1 ItemStack reference/alias tests, empty-stack mutation tests, re-entrant mod callback tests and Android ARM64 thread-safety evidence.
- Completion state: COMPLETE — 72 logical lines through EOF; pinned SHA and 2518-byte size match; inspected, not modified; tests not executed.


### 2502. `crates/horizon-world/src/item_stack_storage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b5092a62929481df783377c0816837d945eff10`
- Size: 4190 bytes
- Coverage: Complete through EOF: 142 logical lines; full file; no gaps.
- Purpose: Core mutable item-stack value operations and consumption/splitting semantics.
- Dependencies / cross-references: StackRef, ItemStackData, item identity, component map, actor callbacks and registry air identity.
- Implementation / logic: Implements count, empty, copy, copy-with-count, split, grow, shrink, consume, same-item/component checks and transmutation through component patches.
- Findings: Arithmetic intentionally uses wrapping integer operations, but public split/grow/shrink inputs are not range-validated. Empty is determined by air or non-positive count. copy_with_count and negative-count paths need an explicit Java oracle; direct public data access can bypass invariants.
- RustChunk relevance: Stack count and consumption are central to inventory, item use, crafting and mod interactions.
- Decision: adaptable; reference; version-sensitive
- 1.20.1 implications: The source uses post-1.20.5 component patches; 1.20.1 stores item data through older tag/damage and item metadata mechanisms. Arithmetic and EMPTY semantics still need exact target-version matching.
- Follow-up checks: Differentially test split(0), negative split, i32::MIN, count overflow, empty/non-empty copies, infinite-consumption actor behavior and component equality against Java 1.20.1.
- RustChunk implementation mapping: Separate version-neutral count/identity operations from the target-version ItemStack data representation; centralize bounds and empty normalization in RustChunk's own type.
- Evidence required before adoption: Java 1.20.1 ItemStack source, boundary-value tests, stack aliasing tests and inventory/mod callback traces.
- Completion state: COMPLETE — 142 logical lines through EOF; pinned SHA and 4190-byte size match; inspected, not modified; tests not executed.


### 2503. `crates/horizon-world/src/item_template.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b623b8c4c3ce1113794fd7c5ada4f84f3eb24159`
- Size: 6634 bytes
- Coverage: Complete through EOF: 196 logical lines; full file; no gaps.
- Purpose: Immutable item prototype/count template with optional component-patch overrides.
- Dependencies / cross-references: ItemStackData, StackRef, component patch/map model, item registry and validation rules.
- Implementation / logic: Constructs templates from stack or item data, resolves effective components, applies patches into a new stack, supports count changes and compares value or reference identity.
- Findings: Strict validation is separate from construction; non-positive counts, missing component defaults and damaged-item stack rules require caller/version checks. Failed application returns a shared empty StackRef, linking to the mutable-empty risk in entry 2501.
- RustChunk relevance: Template snapshots can simplify recipe outputs, inventory transfers and entity-held stacks, provided target-version component storage is not copied.
- Decision: adaptable; reference; component-version-specific
- 1.20.1 implications: 26.3 data components and validation defaults are not equivalent to Java 1.20.1 ItemStack NBT/tag behavior.
- Follow-up checks: Test count zero/negative, stack maximum and damageability validation, component removal/fallback semantics, empty template application and mutation of returned stacks.
- RustChunk implementation mapping: Use a RustChunk-owned immutable stack descriptor only for stable item/count metadata; route target-version tags, damage and enchantments through a 1.20.1 codec.
- Evidence required before adoption: Java 1.20.1 ItemStack constructors/copyWithCount, item defaults, damage, tag serialization and inventory round-trip tests.
- Completion state: COMPLETE — 196 logical lines through EOF; pinned SHA and 6634-byte size match; inspected, not modified; tests not executed.


### 2504. `crates/horizon-world/src/item_use.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2ffef560f35fe0a89274e671f94cd24742c87a16`
- Size: 2148 bytes
- Coverage: Complete through EOF: 62 logical lines; full file; no gaps.
- Purpose: Item-use duration, blocking delay and ticks-using-item calculations.
- Dependencies / cross-references: ItemUseWorld callbacks, item type and duration providers, item stack state and use timing.
- Implementation / logic: Computes blocking delay and blocking state from the active item, and derives elapsed use ticks from duration and remaining-use ticks.
- Findings: Some blocking paths read use_item() more than once, including around duration callbacks. If the held item changes during a callback, the method can combine values from different stack snapshots; this may be intentional Java virtual-call order and must be preserved or disproved.
- RustChunk relevance: Affects blocking, food consumption, bows/crossbows and all timed item actions.
- Decision: adaptable; reference
- 1.20.1 implications: Target version item-use methods and callback sequence must be sourced from Java 1.20.1; do not infer parity from a 26.3 interface.
- Follow-up checks: Instrument getUseItem/getUseDuration callback traces while changing the held item; verify remaining-tick subtraction and Java int overflow behavior.
- RustChunk implementation mapping: Represent the Java call sequence explicitly in RustChunk's player/item-use state machine, avoiding eager snapshots where vanilla re-queries.
- Evidence required before adoption: Java 1.20.1 ItemStack use-duration source and ordered callback differential traces.
- Completion state: COMPLETE — 62 logical lines through EOF; pinned SHA and 2148-byte size match; inspected, not modified; tests not executed.


### 2505. `crates/horizon-world/src/item_use_catalog.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `943099692feff6e9e0947a9c5f7ec6eb72329de8`
- Size: 8701 bytes
- Coverage: Complete through EOF: 239 logical lines; full file; no gaps.
- Purpose: JSON-backed item-use and instrument catalogue with indexed item lookup.
- Dependencies / cross-references: serde_json, item-use stack model, item class identifiers, instrument definitions and versioned gameplay facts.
- Implementation / logic: Validates schema/version and contiguous IDs, builds indexed item and class data, and exposes durations, stackability and instrument metadata.
- Findings: Duration values can be negative, NaN or infinite through numeric/raw-bit fields; fallback casts then inherit Rust float-to-int behavior. The catalogue's classes, item names and data schema are tied to the pinned 26.3 resource export.
- RustChunk relevance: Useful pattern for separating immutable item metadata from per-stack runtime state; exact catalogue is not a target-compatible resource.
- Decision: reference; version-specific; do not ship
- 1.20.1 implications: Do not load 26.3 registry IDs/classes or use-duration facts in a 1.20.1 client. 1.20.1 instrument and item tables differ.
- Follow-up checks: Reject or deliberately normalize non-finite/negative durations; test raw f32-bit inputs, ID gaps/duplicates, missing classes and exact Java rounding.
- RustChunk implementation mapping: Generate a version-hashed RustChunk 1.20.1 item-use table from the target game data; keep loader/schema validation separate from runtime item logic.
- Evidence required before adoption: Pinned Java 1.20.1 item and instrument registries, extracted duration values, malformed catalogue corpus and executed differential tests.
- Completion state: COMPLETE — 239 logical lines through EOF; pinned SHA and 8701-byte size match; inspected, not modified; tests not executed.


### 2506. `crates/horizon-world/src/item_use_lifecycle.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f0f15096faff8db2c5cc6d06f97a60787f5f9943`
- Size: 5520 bytes
- Coverage: Complete through EOF: 172 logical lines; full file; no gaps.
- Purpose: Timed item-use lifecycle: ticking, release, completion and stop.
- Dependencies / cross-references: UseState, ItemUseWorld trait callbacks, player hand inventory, level side effects and vibration.
- Implementation / logic: Implements the held-item state machine, repeatedly queries hand/item identity and dispatches release/finish/stop callbacks with ordered mutations.
- Findings: Callback errors propagate after earlier mutations, so the lifecycle is not transactional. Several operations re-read player state after callbacks; changing hand/item during a callback affects subsequent branches. This ordering is observable and must not be simplified.
- RustChunk relevance: Affects eating, blocking, bows, crossbows, tridents, shields and native mods overriding item-use behavior.
- Decision: adaptable; reference
- 1.20.1 implications: 1.20.1 item-use lifecycle is applicable conceptually, but class and item-specific conditions differ from 26.3.
- Follow-up checks: Trace every state mutation and callback error boundary; test client/server stop behavior, hand swaps mid-tick, release without using flag, and finish-item replacement.
- RustChunk implementation mapping: Build an explicit RustChunk item-use state machine with ordered callback points and documented partial-commit semantics.
- Evidence required before adoption: Java 1.20.1 LivingEntity/Player item-use source, deterministic event traces and mod callback fault-injection tests.
- Completion state: COMPLETE — 172 logical lines through EOF; pinned SHA and 5520-byte size match; inspected, not modified; tests not executed.


### 2507. `crates/horizon-world/src/item_use_start.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `376b1d25707982e4f77ac1eca3095d88f097b5e2`
- Size: 1748 bytes
- Coverage: Complete through EOF: 54 logical lines; full file; no gaps.
- Purpose: Start of the active item-use lifecycle and initial use synchronization.
- Dependencies / cross-references: UseState, ItemUseWorld callbacks, active hand/item state, level side and vibration.
- Implementation / logic: Checks hand/item eligibility, records the used stack, queries duration and client/server state, then sets flags, vibration and kinetic state.
- Findings: State is assigned before later fallible callbacks; an error can leave a partially initialized use session. Item identity can change between the initial check and subsequent duration lookup.
- RustChunk relevance: Needed for shields, food, bows and modded items that begin timed use.
- Decision: adaptable; reference
- 1.20.1 implications: Compare exact 1.20.1 callback order and client synchronization packets; the 26.3 callback contract is not proof of protocol behavior.
- Follow-up checks: Inject failures at each callback; test hand replacement during getUseDuration, zero/negative duration and client/server flag order.
- RustChunk implementation mapping: Port as a narrow state-transition function that emits ordered effects rather than mutating unrelated player subsystems directly.
- Evidence required before adoption: Java 1.20.1 startUsingItem source, packet/state traces and tests for callback mutation/error paths.
- Completion state: COMPLETE — 54 logical lines through EOF; pinned SHA and 1748-byte size match; inspected, not modified; tests not executed.


### 2508. `crates/horizon-world/src/java_hash_map.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fc580706b6b8541ead93b8a690bf36ef4907830e`
- Size: 26283 bytes
- Coverage: Complete through EOF: 791 logical lines; 1–400/401–791; no gaps.
- Purpose: Java-compatible hash map including treeified bins and iterator removal.
- Dependencies / cross-references: Java hash/equality traits, hash spreading, red-black tree operations, Arc/RwLock identity and fail-fast cursor state.
- Implementation / logic: Implements bucket arrays, resize/split, collision chains, tree bins, put/get/remove/clear, map copy and a cursor that can remove the current entry.
- Findings: The map has extensive custom collision/tree balancing logic and no executed differential evidence here. copied() computes capacity from ceil(size/0.75) before next_power_of_two; verify this against the exact Java HashMap putMapEntries formula, especially sizes divisible by three. Tie-break identity and iterator mod-count behavior are compatibility-sensitive.
- RustChunk relevance: Potentially useful only where Java iteration/bucket order is externally observable; ordinary Rust collections are safer otherwise.
- Decision: reference; compatibility-only; do not ship
- 1.20.1 implications: Java 1.20.1 runtime collection implementation/version must be the oracle. HashMap iteration order is not a stable API and may vary with capacity and insertion sequence.
- Follow-up checks: Differentially test capacity thresholds, hash collisions/treeification/untreeification, resize splits, equal identity hashes, replacement modCount, iterator remove and fail-fast behavior.
- RustChunk implementation mapping: Prefer std collections for ordinary maps; isolate a Java-compatible map only behind a tested compatibility adapter where observable ordering requires it.
- Evidence required before adoption: Exact 1.20.1 JDK HashMap source, randomized Java-vs-Rust operation traces and exhaustive bucket/tree boundary tests.
- Completion state: COMPLETE — 791 logical lines through EOF; pinned SHA and 26283-byte size match; inspected, not modified; tests not executed.


### 2509. `crates/horizon-world/src/json_folder.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `66e511d11fdeda7a84c1011eb5a81d078df1a832`
- Size: 8598 bytes
- Coverage: Complete through EOF: 214 logical lines; full file; no gaps.
- Purpose: Filesystem-backed JSON resource folder with cached resource IDs and bytes.
- Dependencies / cross-references: std::fs, PathBuf, resource-location parsing, JSON consumers and a global Weak cache.
- Implementation / logic: Recursively enumerates directories, sorts resource names, reads matching JSON files and shares immutable prepared data through a path-keyed cache.
- Findings: Path::is_dir follows symlinks, so recursive traversal can escape the intended root or follow a directory cycle. Nested read_dir errors are logged and skipped, potentially producing an incomplete resource view without failing the load. Cache keys use non-canonical paths, so aliases can duplicate entries.
- RustChunk relevance: Useful for resource-pack/data-driven loading and avoiding repeated directory scans; unsafe traversal is not suitable for untrusted packs as-is.
- Decision: adaptable; reference; filesystem-hardening required
- 1.20.1 implications: 1.20.1 resource-pack path semantics and pack priority differ; no direct 26.3 resource identifiers or data should be reused.
- Follow-up checks: Test symlink-to-parent cycles, symlink escape, unreadable nested directories, invalid UTF-8 names, relative/absolute aliases and duplicate resource IDs.
- RustChunk implementation mapping: Implement a root-confined resource provider with explicit symlink policy, canonicalized cache keys, deterministic precedence and surfaced enumeration errors.
- Evidence required before adoption: Java 1.20.1 resource manager behavior, adversarial filesystem fixture corpus and Android scoped-storage tests.
- Completion state: COMPLETE — 214 logical lines through EOF; pinned SHA and 8598-byte size match; inspected, not modified; tests not executed.


### 2510. `crates/horizon-world/src/lazy_json.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b5347ee4786b0928e4a44bd3d7f25c6c7476f132`
- Size: 2642 bytes
- Coverage: Complete through EOF: 84 logical lines; full file; no gaps.
- Purpose: Lazy JSON Value parsing with cached decoded Value.
- Dependencies / cross-references: serde_json::Value and OnceLock.
- Implementation / logic: Stores serialized JSON text and lazily parses it once; a constructed Value is serialized immediately and an already-populated cache is cloned with the wrapper.
- Findings: The Value constructor performs a serialize-then-parse representation round-trip, adding allocation and work. Clone duplicates cached Values when initialized. The cache is per-object and read() before initialization returns an owned parsed value without populating the lock.
- RustChunk relevance: Useful as a small adapter for large resource JSON consumed by several systems, but it should be justified by memory profiles.
- Decision: adaptable; reference
- 1.20.1 implications: No Minecraft-version logic; serde_json number/object behavior still needs preserving for 1.20.1 resource formats.
- Follow-up checks: Test large integers, float formatting, object ordering, concurrent first reads and clone-before/after-initialization memory costs.
- RustChunk implementation mapping: Use borrowed typed projections or Arc<Value> for shared parsed resources where possible; retain lazy parsing only when measured startup/memory gains justify it.
- Evidence required before adoption: Numerical round-trip corpus, concurrent access tests and Android heap/startup measurements.
- Completion state: COMPLETE — 84 logical lines through EOF; pinned SHA and 2642-byte size match; inspected, not modified; tests not executed.


### 2511. `crates/horizon-world/src/level/bonemeal.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d4e53aa260b808b697d2cbd858ee9ccf305c9fe6`
- Size: 50968 bytes
- Coverage: Complete through EOF: 1256 logical lines; 1–400/401–800/801–1200/1201–1256; no gaps.
- Purpose: Bonemeal growth behavior for crops, plants, trees, mushrooms, fungi and water plants, plus template placement.
- Dependencies / cross-references: LevelAccess, LiveFeatures, LevelFeatureWorld, block tags/properties, legacy random, structure templates and scheduled ticks.
- Implementation / logic: Implements validity checks and mutation for many plant kinds, configured/placed feature execution, template processors, tree/flower growth and water/grass spread.
- Findings: Several random bound calls silently fall back to zero on failure. place_template converts placement errors with .ok() into None, making a placement failure indistinguishable from missing feature resources. Some helper loops fetch state before checking loaded/build-height conditions. Growth kinds and resource feature names include post-1.20.1 content.
- RustChunk relevance: World growth is core Minecraft behavior; useful as a coverage map and algorithm reference, but not a drop-in module.
- Decision: adaptable; worldgen-version-specific; reference
- 1.20.1 implications: 1.20.1 has older plant/feature sets, registry contents and structure processors. Exclude newer plants/tree types and regenerate feature data from the target client.
- Follow-up checks: Differentially test every growth kind, RNG call sequence, unloaded/build-height edges, structure placement errors, malformed feature resources and failure rollback.
- RustChunk implementation mapping: Port target-version growth rules behind RustChunk's world interface; separate per-block vanilla logic from versioned LiveFeatures and template placement.
- Evidence required before adoption: Java 1.20.1 block/feature sources, target-version placed/configured feature data, deterministic RNG traces and worldgen differential fixtures.
- Completion state: COMPLETE — 1256 logical lines through EOF; pinned SHA and 50968-byte size match; inspected, not modified; tests not executed.


### 2512. `crates/horizon-world/src/level/buckets.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `25026bfc1eb408b0be0f440e120f733d43a225f6`
- Size: 8404 bytes
- Coverage: Complete through EOF: 204 logical lines; full file; no gaps.
- Purpose: Bucket pickup and emptying behavior for fluid, powder snow and mob buckets.
- Dependencies / cross-references: FluidKind/FluidState, LiquidBlockContainer interfaces, level dimension, block replacement and pickup/place handlers.
- Implementation / logic: Selects resulting bucket item, checks replaceability/container rules, handles Nether water evaporation, places fluid or powder snow and emits sound/game events.
- Findings: BucketContent::Fluid(FluidKind::Empty) is accepted by the public API and falls through the non-water branch, which constructs a lava source. The waterlogged-container branch ignores place_liquid's boolean result and reports success regardless. bucket_pickup delegates waterlogged validation to pickup_block, so the flag itself is checked there.
- RustChunk relevance: Bucket behavior is common gameplay and a good candidate for a narrowly isolated interaction adapter.
- Decision: adaptable; reference; API-validation required
- 1.20.1 implications: Validate only actual 1.20.1 bucket contents and container APIs; the current 26.3 enum/resource set includes additional mod-facing content.
- Follow-up checks: Reject Empty and invalid flowing contents; test failed container placement, shift-click adjacent placement, Nether evaporation, barriers, powder snow and source-only pickup.
- RustChunk implementation mapping: Represent valid bucket contents as a validated enum that cannot encode Empty; propagate container placement failure and keep event ordering explicit.
- Evidence required before adoption: Java 1.20.1 BucketItem/MobBucketItem/SolidBucketItem source, all vanilla bucket interaction tests and modded fluid-container integration tests.
- Completion state: COMPLETE — 204 logical lines through EOF; pinned SHA and 8404-byte size match; inspected, not modified; tests not executed.


### 2513. `crates/horizon-world/src/level/classes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1a27da89d92ee069e2270849abdaeeb281fc36bb`
- Size: 11956 bytes
- Coverage: Complete through EOF: 340 logical lines; full file; no gaps.
- Purpose: Mapping Java block class names and marker interfaces into compact class/interface sets.
- Dependencies / cross-references: Block registry exports, class-chain data and fixed bitsets for classes/interfaces.
- Implementation / logic: Converts exact Java class-name strings to enum variants, stores class ancestry in a 320-bit set and marker interfaces in a compact mask.
- Findings: The enum includes classes introduced after 1.20.1 (including Creaking/CopperGolemStatue-era entries), so its universe is not the 1.20.1 block hierarchy. Unknown class names are not represented and dispatch silently lacks that class behavior.
- RustChunk relevance: Compact ancestry dispatch is useful, but must be generated against the exact target registry and inheritance hierarchy.
- Decision: reference; version-specific; do not ship
- 1.20.1 implications: Do not reuse class numbers/names or assume every 26.3 block exists in 1.20.1; registry and Java superclass relationships differ.
- Follow-up checks: Generate the exact 1.20.1 class/interface universe; check every registered block chain, unknown class handling and enum capacity.
- RustChunk implementation mapping: Use version-generated class metadata with explicit unknown-class diagnostics and stable RustChunk-owned IDs.
- Evidence required before adoption: Java 1.20.1 decompiled class hierarchy, target registry export, exhaustive class-chain comparison and Forge 47.x mod block tests.
- Completion state: COMPLETE — 340 logical lines through EOF; pinned SHA and 11956-byte size match; inspected, not modified; tests not executed.


### 2514. `crates/horizon-world/src/level/creaking.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `07633c978ab43910cb3ce3beca13915a2bea430d`
- Size: 3297 bytes
- Coverage: Complete through EOF: 83 logical lines; full file; no gaps.
- Purpose: Creaking Heart state changes based on nearby logs, dimension and day/night.
- Dependencies / cross-references: LevelAccess time/dimension, block class data, log tags, axis properties and block updates.
- Implementation / logic: Checks adjacent pale-oak log structure, derives facing/axis and chooses uprooted/awake/dormant state using a time window.
- Findings: The block and its pale-oak/Creaking behavior are absent from Minecraft 1.20.1. The time calculation also uses Euclidean remainder for negative day time, unlike Java remainder semantics.
- RustChunk relevance: No direct gameplay relevance to the 1.20.1 port; can be kept as an example of data-driven block-state transitions for future versions.
- Decision: not applicable to 1.20.1; do not ship
- 1.20.1 implications: Do not port the block or its class dispatch into RustChunk 1.20.1.
- Follow-up checks: Exclude from the target feature list; if a later version is targeted, compare negative day-time and log-neighbor behavior to that version's Java source.
- RustChunk implementation mapping: No 1.20.1 implementation mapping. Keep version-specific features out of the core vanilla registry.
- Evidence required before adoption: Target 1.20.1 registry absence; separate version-upgrade plan before any later-version adoption.
- Completion state: COMPLETE — 83 logical lines through EOF; pinned SHA and 3297-byte size match; inspected, not modified; tests not executed.


### 2515. `crates/horizon-world/src/level/data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7b98278e4e80c22084babf85858c29a4a7e66609`
- Size: 41113 bytes
- Coverage: Complete through EOF: 1222 logical lines; 1–400/401–800/801–1200/1201–1222; no gaps.
- Purpose: Loads block properties, names, tags, state facts, collision shapes, fluids and gameplay metadata.
- Dependencies / cross-references: BlockRegistry, resource JSON/NBT exports, StateTable, BlockSet, VoxelShape, class mappings and typed gameplay inputs.
- Implementation / logic: Builds block/state maps, named blocks, tags, shape tables, dynamic support and fluid tag masks; supports JSON fallback and typed gameplay projections.
- Findings: Mandatory named IDs and class lists include post-1.20.1 content. Several unknown/missing fields silently default to false/zero/empty or skip rows; shape dedup searches existing shapes linearly (potential quadratic work). Registry IDs and casts assume bounded tables.
- RustChunk relevance: The data-loading boundary is central to any native world simulation; architecture is useful but data and validation policy must be RustChunk-owned.
- Decision: reference; rewrite registry for 1.20.1
- 1.20.1 implications: All tables, class names, block IDs, fluid tags and facts must come from exact 1.20.1/Forge 47.x sources, not the pinned 26.3 export.
- Follow-up checks: Add strict schema/version checks, required-field validation, duplicate detection, shape-count caps and benchmark hash/map-based shape deduplication.
- RustChunk implementation mapping: Create a version-hashed 1.20.1 RegistrySnapshot with fail-closed parsing and separate optional Forge extension layers.
- Evidence required before adoption: Version-locked Minecraft 1.20.1 vanilla and Forge 47.x exports, malformed resource corpus, complete registry counts and shape parity tests.
- Completion state: COMPLETE — 1222 logical lines through EOF; pinned SHA and 41113-byte size match; inspected, not modified; tests not executed.


### 2516. `crates/horizon-world/src/level/data/cache.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ebb2b93d849a4ef8b7cdccf3c3fb1a1690ffbd98`
- Size: 42064 bytes
- Coverage: Complete through EOF: 1166 logical lines; 1–400/401–800/801–1166; no gaps.
- Purpose: Persistent cache for block registry and resolved behavior snapshots.
- Dependencies / cross-references: Registry snapshot codec, SHA-256, LZ4, filesystem timestamps, path hashing and global cache-directory configuration.
- Implementation / logic: Computes source stamps/digests, encodes bounded snapshots, verifies magic/version/checksum and restores registry/behavior data; writes through temporary files and rename.
- Findings: Cache loading uses fs::read before validating the maximum cache-file size, so an oversized file can allocate memory before the guard. Source-stamp reuse keys on path/length/mtime rather than content; same-size edits with preserved timestamps can return stale data. Executable identity is also based on metadata rather than binary contents.
- RustChunk relevance: Persistent precomputed registry data can reduce Android startup time, but cache corruption and stale-source acceptance are release-critical.
- Decision: adaptable; cache-hardening required; reference
- 1.20.1 implications: Use RustChunk-owned cache versioning and include the exact 1.20.1/Forge data/runtime identity. Do not trust Starling's cache key as proof of target-version compatibility.
- Follow-up checks: Check metadata/file length before allocation, hash source contents or use robust generation IDs, fsync durability as needed and test truncated/oversized/corrupted snapshots and same-length same-mtime changes.
- RustChunk implementation mapping: Implement a bounded, content-addressed cache with atomic replace, schema version, source provenance and safe fallback to rebuild.
- Evidence required before adoption: Fuzzed cache codec, large/corrupt-file tests, source mutation tests preserving length/mtime, Android filesystem crash/restart tests and startup benchmarks.
- Completion state: COMPLETE — 1166 logical lines through EOF; pinned SHA and 42064-byte size match; inspected, not modified; tests not executed.


### 2517. `crates/horizon-world/src/level/falling.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `21c68a705f08e6aa4c1e71943ecb3cbe3b8de4eb`
- Size: 5389 bytes
- Coverage: Complete through EOF: 137 logical lines; full file; no gaps.
- Purpose: Falling-block spawn, landing, break and pointed-dripstone damage decisions.
- Dependencies / cross-references: LevelAccess, fluid states, block survival/replacement, piston and collision checks, LevelEvent and caller-owned falling entity.
- Implementation / logic: Converts source blocks to fluid legacy states, emits falling-block events, finds stalactite chains and resolves placement/break/concrete conversion on landing.
- Findings: The module returns Landing outcomes but does not simulate entity motion, gravity, collision or item drops. Stalactite traversal queries states before validating loaded/build-height status; callers must handle Broken/Continue outcomes correctly.
- RustChunk relevance: Useful for implementing vanilla falling blocks and dripstone interactions in the native world engine.
- Decision: adaptable; reference; entity-integration required
- 1.20.1 implications: 1.20.1 block IDs, tags, entity physics and game events must be used; Forge may add falling-block subclasses.
- Follow-up checks: Test chunk boundaries, build-height edges, waterlogged landing, piston landing, concrete conversion, damage distance and caller drop/spawn behavior.
- RustChunk implementation mapping: Keep block-state transition rules in RustChunk world code and delegate entity physics to a separate entity simulation; make outcome handling exhaustive.
- Evidence required before adoption: Java 1.20.1 FallingBlockEntity/PointedDripstone sources, entity-physics traces and Android/Forge collision tests.
- Completion state: COMPLETE — 137 logical lines through EOF; pinned SHA and 5389-byte size match; inspected, not modified; tests not executed.


### 2518. `crates/horizon-world/src/level/feature_world.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c23c7eed0f7a3b3807d3f0a7236de930400c674a`
- Size: 12663 bytes
- Coverage: Complete through EOF: 350 logical lines; full file; no gaps.
- Purpose: Worldgen feature placement adapter over a live mutable level.
- Dependencies / cross-references: LevelAccess/Sim, LiveFeatures, biome registry, block properties/tags, voxel shapes and structure placement.
- Implementation / logic: Exposes state reads/writes, block entities, survival, collision, fluid ticks, biome identity, height and worldgen placement hooks.
- Findings: Sea levels are fixed by dimension (Overworld 63, Nether 32, End 0) rather than a level-specific value. Height queries scan vertical columns, which can be expensive. Loot-table block-entity behavior is explicitly not implemented here.
- RustChunk relevance: Strong adapter pattern between worldgen algorithms and the live game world, with clear boundaries for storage and side effects.
- Decision: adaptable; reference
- 1.20.1 implications: Forge dimensions and custom world heights/sea levels require target host configuration; 1.20.1 worldgen data/biome IDs differ.
- Follow-up checks: Expose configured sea level/heightmap queries, profile column scans and test all vanilla/custom dimensions and block-entity feature placement.
- RustChunk implementation mapping: Implement a RustChunk FeatureWorld adapter backed by chunk heightmaps, explicit dimension metadata and queued entity/block-entity side effects.
- Evidence required before adoption: Java 1.20.1 FeaturePlaceContext/WorldGenLevel behavior, target biome/feature registry, custom dimension tests and worldgen parity runs.
- Completion state: COMPLETE — 350 logical lines through EOF; pinned SHA and 12663-byte size match; inspected, not modified; tests not executed.


### 2519. `crates/horizon-world/src/level/fluids.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f94ca19ebb4c5b304671072ff2f1ad6321706635`
- Size: 29967 bytes
- Coverage: Complete through EOF: 833 logical lines; 1–400/401–800/801–833; no gaps.
- Purpose: Liquid-container rules, source conversion, fluid spread, lava random ticks and face occlusion.
- Dependencies / cross-references: FluidState/FluidKind, SpreadContext, BlockSet tags, block shapes, LevelAccess spread permissions and random source.
- Implementation / logic: Caches local states/holes for spread distance, computes slopes and flow targets, updates containers, schedules fluid ticks and handles lava ignition/random behavior.
- Findings: SpreadContext computes wrapping coordinate differences then adds offsets with ordinary i32 arithmetic, risking overflow at coordinate extremes. Several random failures fall back to zero. place_liquid can ignore a failed set and return success to its caller; shape-query errors can collapse to empty/non-occluding results.
- RustChunk relevance: Fluid simulation is a central gameplay subsystem and directly affects chunk/world behavior.
- Decision: adaptable; core candidate; 1.20.1 verification required
- 1.20.1 implications: Reproduce exact Java 1.20.1 flow levels, source-conversion rules, tick delays and Forge fluid hooks; 26.3 block tags and states are not target proof.
- Follow-up checks: Test all fluid levels, corner/slope propagation, coordinate extremes, unloaded chunks, cancelled events, custom fluids and failed block writes against Java 1.20.1.
- RustChunk implementation mapping: Port the algorithm into a bounded RustChunk fluid engine with checked coordinates, explicit error paths and host callbacks for modded fluids.
- Evidence required before adoption: Java 1.20.1 FlowingFluid/LiquidBlock sources, deterministic random traces, fluid test matrix and Forge fluid interoperability results.
- Completion state: COMPLETE — 833 logical lines through EOF; pinned SHA and 29967-byte size match; inspected, not modified; tests not executed.


### 2520. `crates/horizon-world/src/level/growth.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0ca8aaf284d031f140c2f238b230a9b5419d6e5b`
- Size: 16625 bytes
- Coverage: Complete through EOF: 462 logical lines; 1–400/401–462; no gaps.
- Purpose: TreeGrower, configured/placed feature and jigsaw/structure placement from block growth.
- Dependencies / cross-references: LiveFeatures, LevelFeatureWorld, block registry/tags, legacy random and structure piece placement.
- Implementation / logic: Selects weighted tree features, handles 2x2 saplings, places configured features, restores saplings on failed growth and returns structure entities as NBT.
- Findings: place_jigsaw logs failed piece placement but still returns assembled=true, hiding partial failure. Weighted/random helpers substitute zero when a bounded draw fails. Tree minimum height casts i64 to i32 without range validation. Grower/resource lists include later-version trees.
- RustChunk relevance: Feature-driven tree growth is a useful architecture for worldgen behavior but must be separated from 26.3 content.
- Decision: adaptable; worldgen-version-specific; reference
- 1.20.1 implications: Cherry exists in 1.20 but pale oak/poplar and newer feature assets do not; exact 1.20.1 feature IDs and random draw order are required.
- Follow-up checks: Test partial jigsaw failure reporting, sapling restoration, 2x2 selection, feature absence, random errors, height overflow and target-version grower tables.
- RustChunk implementation mapping: Port a version-scoped TreeGrower catalog and return structured placement errors/partial success instead of collapsing them to a boolean.
- Evidence required before adoption: Java 1.20.1 TreeGrower/ConfiguredFeature sources, target resource extraction, deterministic RNG and structure-placement parity tests.
- Completion state: COMPLETE — 462 logical lines through EOF; pinned SHA and 16625-byte size match; inspected, not modified; tests not executed.


### 2521. `crates/horizon-world/src/level/growth_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5bfbf29f69896eb115a46f80dfce583f73f84108`
- Size: 28267 bytes
- Coverage: Complete through EOF: 792 logical lines; 1–400/401–792; no gaps.
- Purpose: Integration test harness for bonemeal, trees, mushrooms, plants, precipitation and template entities.
- Dependencies / cross-references: Rust test harness, prepared 26.3 resources, BlockRegistry, Behaviours, LiveFeatures and mock LevelAccess.
- Implementation / logic: Builds an in-memory world and tests seeded growth scenarios, mod bonemeal callbacks, crop aging, water plants, templates, eyeblossoms and eggs.
- Findings: Tests return early when resources are absent, so a green run can skip all cases unless the resource precondition is checked. Mock is_loaded always returns true and omits realistic chunk/bounds failure modes. Seeded probabilistic thresholds test a narrow sample and do not constitute exhaustive Java parity.
- RustChunk relevance: Fixture architecture is useful for RustChunk regression testing; the tests are not runtime code.
- Decision: fixture-only; do not ship
- 1.20.1 implications: Several test blocks/features are absent in 1.20.1 and fixtures depend on the 26.3 resource bundle.
- Follow-up checks: Make skipped-resource status explicit, add 1.20.1 fixtures and unloaded/bounds/failure mocks, and compare exact outputs/RNG traces to Java rather than only growth thresholds.
- RustChunk implementation mapping: Use the fixture style for RustChunk tests but isolate target-version resources and add a fail-on-skip CI mode for required suites.
- Evidence required before adoption: Executed test output with resources present, Java 1.20.1 oracle fixtures, negative-path coverage and non-skipped CI logs.
- Completion state: COMPLETE — 792 logical lines through EOF; pinned SHA and 28267-byte size match; inspected, not modified; tests not executed.


### 2522. `crates/horizon-world/src/level/hooks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `92b0c01873a3535198f4738cb47171ec60507aae`
- Size: 30565 bytes
- Coverage: Complete through EOF: 749 logical lines; 1–400/401–749; no gaps.
- Purpose: Block lifecycle hooks for neighbor changes, placement, removal, mining and entity collisions.
- Dependencies / cross-references: Class hierarchy, block state properties, LevelEvent, scheduled ticks, redstone/piston helpers and host-owned entities/block entities.
- Implementation / logic: Dispatches the first recognized class handler in the class chain and emits events or mutates block state for doors, rails, redstone, TNT, sponge, ice and other blocks.
- Findings: Only the first recognized handler in class-chain order runs, so class ancestry ordering is semantic. Random errors often fall back to fixed values. Sponge traversal depends on host pickup/drop and loaded-state semantics. Several block events intentionally defer actual entity/block-entity effects to the caller.
- RustChunk relevance: Useful lifecycle/event bridge, especially for keeping block rules independent from the client renderer and entity storage.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: The mapped class set includes blocks added after 1.20.1; event IDs, hooks and block-entity callbacks must be regenerated for Forge 47.x.
- Follow-up checks: Verify class-chain dispatch ordering, event sequences, sponge BFS cap/depth, moved-by-piston cases, block entity pre-remove behavior and random draw order.
- RustChunk implementation mapping: Use a version-generated hook table and a typed event queue whose consumer owns sounds, drops, block entities and entity spawning.
- Evidence required before adoption: Java 1.20.1 BlockState/Block hooks, exact event traces, piston/neighbor integration and Forge block-entity mod tests.
- Completion state: COMPLETE — 749 logical lines through EOF; pinned SHA and 30565-byte size match; inspected, not modified; tests not executed.


### 2523. `crates/horizon-world/src/level/interact.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e83250b3674ad81477117e8914398bd55a7ea09d`
- Size: 41306 bytes
- Coverage: Complete through EOF: 1037 logical lines; 1–400/401–800/801–1037; no gaps.
- Purpose: Block right-click interaction with and without held items, including redstone, cake, containers, beds and buckets.
- Dependencies / cross-references: PlayerContext, item bindings, block class chain, inventory/item-use callbacks, LevelEvent and host UI/block entities.
- Implementation / logic: Maps hit coordinates to selectable slots, handles useWithoutItem, routes useItemOn, returns InteractionResult/UseBranch and emits player effects.
- Findings: hit_slot is public and does not validate rows/columns; zero sections can panic. Unhandled item interactions return None and fall through to use_without_item, so the exact Java TRY_WITH_EMPTY_HAND gating with a non-empty held item must be verified. PlayerContext omits much of the real player/inventory/entity state.
- RustChunk relevance: Interaction routing is central to modded Minecraft play; result/swing/consumption semantics must match Java exactly.
- Decision: adaptable; reference; player-integration required
- 1.20.1 implications: Block/item class data and behaviors include 26.3 content (e.g. copper-golem statue/chest); reconstruct only 1.20.1 features and Forge overrides.
- Follow-up checks: Validate slot dimensions, all InteractionResult branches, held-item fallback, creative/adventure restrictions, empty hand, item mutation, block entity menus and sound/event ordering.
- RustChunk implementation mapping: Split interaction dispatch into a RustChunk-owned vanilla 1.20.1 layer plus Forge mod callbacks; validate hit geometry and return explicit branch/result metadata.
- Evidence required before adoption: Java 1.20.1 BlockBehaviour.useItemOn/useWithoutItem sources, Player interaction traces, menu/network packets and Forge mod tests.
- Completion state: COMPLETE — 1037 logical lines through EOF; pinned SHA and 41306-byte size match; inspected, not modified; tests not executed.


### 2524. `crates/horizon-world/src/level/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04a959bb28f1563cc81fcea3f42980699868d059`
- Size: 49704 bytes
- Coverage: Complete through EOF: 1266 logical lines; 1–400/401–800/801–1200/1201–1266; no gaps.
- Purpose: Public block-behavior API, LevelAccess host contract, level events, game rules and simulation context.
- Dependencies / cross-references: BlockRegistry, BlockStateId, FluidState, Ticks, Behaviours, Sim, LevelEvent and all level submodules.
- Implementation / logic: Defines game-side adapter methods, defaults, dimensions/game rules, public behavior wrappers and the mutable Sim view used by behavior code.
- Findings: Many LevelAccess methods have permissive defaults (client side false, loaded-tick true, entity count zero, default Overworld/rules, default heights/lighting). An incomplete host implementation can compile yet silently diverge. Dimension is limited to three vanilla values and cannot represent custom Forge dimensions.
- RustChunk relevance: The clearest architectural boundary between native simulation and a host/client; useful if RustChunk adopts a Rust world core.
- Decision: adaptable; architectural reference
- 1.20.1 implications: Need 1.20.1 semantics and Forge custom-dimension/mod hooks; don't inherit 26.3 interfaces or defaults unchanged.
- Follow-up checks: Create a host conformance checklist, make critical capabilities required instead of silently defaulted, test unloaded chunks/custom dimensions and event-consumer completeness.
- RustChunk implementation mapping: Define RustChunk WorldAccess/WorldEvents traits with explicit capabilities, validated dimension metadata and a single authoritative state/tick owner.
- Evidence required before adoption: Complete 1.20.1 host contract, conformance suite, Forge dimension/mod hooks and end-to-end world event traces.
- Completion state: COMPLETE — 1266 logical lines through EOF; pinned SHA and 49704-byte size match; inspected, not modified; tests not executed.


### 2525. `crates/horizon-world/src/level/orientation.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7a34fccefcd49dd0faabc5b22f467c05d1f80944`
- Size: 5422 bytes
- Coverage: Complete through EOF: 184 logical lines; full file; no gaps.
- Purpose: The 48 update-context orientations and redstone experimental orientation selection.
- Dependencies / cross-references: Direction/Axis geometry, random source, LevelAccess redstone-experiments flag and redstone neighbor-update code.
- Implementation / logic: Maps up/front/side-bias tuples to indices and back, derives side direction by cross product, supports orientation transforms and randomly selects an initial context.
- Findings: Public new()/from_index() assert on invalid inputs; random() unwraps a bounded draw. Correctness depends on the precise 48-entry ordering and side-bias cross-product convention. Orientation mode is only enabled when redstone experiments are active.
- RustChunk relevance: Useful for correct experimental redstone update ordering and avoiding arbitrary neighbor-order heuristics.
- Decision: adaptable; reference; redstone-experiment-gated
- 1.20.1 implications: Only relevant if RustChunk 1.20.1 enables the matching experimental redstone feature; confirm the exact target version's feature gate and orientation table.
- Follow-up checks: Exhaustively round-trip all 48 indices, test every transform/orthogonal pair, verify direction iteration order and compare randomized redstone traces to Java 1.20.1.
- RustChunk implementation mapping: Port as a small typed value with validated constructors and exhaustive compile-time/unit tests, gated by target world feature settings.
- Evidence required before adoption: Java 1.20.1 ExperimentalRedstoneUtils/Orientation source, all-48 mapping tests and end-to-end redstone differential traces.
- Completion state: COMPLETE — 184 logical lines through EOF; pinned SHA and 5422-byte size match; inspected, not modified; tests not executed.

## Checkpoint 2501–2525 — COMPLETE (25/25)

- All 25 files inspected one at a time through EOF in deterministic lexical path order after excluding Index entries 1–2500. Pinned source SHA and byte sizes match the complete untruncated tree.
- Long-file coverage: java_hash_map.rs 1–400/401–791; bonemeal.rs 1–400/401–800/801–1200/1201–1256; data.rs 1–400/401–800/801–1200/1201–1222; data/cache.rs 1–400/401–800/801–1166; fluids.rs 1–400/401–800/801–833; growth.rs 1–400/401–462; growth_tests.rs 1–400/401–792; hooks.rs 1–400/401–749; interact.rs 1–400/401–800/801–1037; mod.rs 1–400/401–800/801–1200/1201–1266.
- Key risks: mutable shared EMPTY handle; unvalidated negative/overflow stack operations; Java HashMap copy-capacity parity; symlink-following JSON traversal and suppressed nested enumeration errors; bonemeal placement errors collapsed to None; BucketContent::Fluid(Empty) becoming lava; modern class/registry data; cache metadata-only source stamps and pre-read allocation; unchecked coordinates/random fallbacks; partial jigsaw errors reported as assembled; host adapter defaults can silently diverge.
- Ledger contains **125 records (2401–2525)**; every record has exactly 15 fields. Index now contains **2,525 sequential unique paths** with matching path/Decision rows.
- Explicit user authorization covers **2501–2600**. Continue with 2526–2550 without requesting another permission; report at 2550. The second 50-file section 2551–2600 remains covered. **STOP at 2600**.
- No tests/examples executed; no app source changes, builds, APK, device, renderer, Minecraft, Forge, multiplayer or Aternos validation. No backup at 2525; next backup only after entry 3000 and global verification.


### 2526. `crates/horizon-world/src/level/pistons.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `af9352feb4a8df4fdd6c3b9330943bc31e929e78`
- Size: 36047 bytes
- Coverage: Complete through EOF: 954 logical lines; ranges 1–200/201–400/401–600/601–800/801–954; no gaps.
- Purpose: Piston extension/retraction, structure resolution and moving-block lifecycle.
- Dependencies / cross-references: LevelAccess/Sim, Ticks, block states, neighbor updates, piston events and host entity callbacks.
- Implementation / logic: Resolves pushed/destroyed block sets, checks movement constraints, mutates states, queues moving pistons and handles client event replay.
- Findings: Move and ticker order are semantic; entity collision and push results remain host-owned. Verify all refusal and block-entity cases.
- RustChunk relevance: High-value world behavior reference, not a complete entity/world adapter.
- Decision: adaptable; reference; entity-integration required
- 1.20.1 implications: Regenerate 1.20.1 piston limits, tags, block entities, event payloads and Forge piston hooks.
- Follow-up checks: Compare push/pull order, immovable blocks, slime/honey, block entities, cancelled moves and client event replay.
- RustChunk implementation mapping: Use a versioned piston subsystem that emits ordered move plans; keep entity collision in explicit host callbacks.
- Evidence required before adoption: Java 1.20.1 piston sources, deterministic move traces, block-entity/cancellation cases and Forge piston-hook tests.
- Completion state: COMPLETE — 954 logical lines through EOF; pinned SHA and 36047-byte size match; inspected, not modified; tests not executed.

### 2527. `crates/horizon-world/src/level/place.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2f4372415975cf31801af01e9ee3fa81f74543aa`
- Size: 73280 bytes
- Coverage: Complete through EOF: 1816 logical lines; ranges 1–200/201–400/401–600/601–800/801–1000/1001–1200/1201–1400/1401–1600/1601–1800/1801–1816; no gaps.
- Purpose: Block-item placement state selection and specialized placement for block families.
- Dependencies / cross-references: PlaceContext, block/item registry, StateTable, survival/shape queries, LevelAccess hooks, flags and events.
- Implementation / logic: Computes target position/orientation, derives states, dispatches placement handlers, merges/places paired blocks and emits updates.
- Findings: The double-height path prepares or clears the upper cell before the main placement write is known to succeed; partial-write/cancel behavior needs exact Java comparison.
- RustChunk relevance: Core player-world interaction reference, not an inventory or item-use implementation.
- Decision: adaptable; reference; Forge-placement-hooks required
- 1.20.1 implications: Use the 1.20.1 block/item catalogue, state properties and Forge placement callbacks; do not reuse modern catalogue data.
- Follow-up checks: Test paired blocks, refused writes, replaceability, collision, waterlogging, permission and dispenser contexts.
- RustChunk implementation mapping: Implement a 1.20.1 placement engine with explicit context and paired-write outcome handling.
- Evidence required before adoption: Java 1.20.1 BlockItem/BlockBehaviour placement, failed-write traces and Forge placement-event tests.
- Completion state: COMPLETE — 1816 logical lines through EOF; pinned SHA and 73280-byte size match; inspected, not modified; tests not executed.

### 2528. `crates/horizon-world/src/level/portal.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b59dd477c5c93675fa66b029c22e277e736c9e70`
- Size: 6799 bytes
- Coverage: Complete through EOF: 184 logical lines; full file; no gaps.
- Purpose: Portal frame detection and portal-block placement.
- Dependencies / cross-references: LevelAccess/Sim, portal shape checks, block tags/registry, state writes and dimension metadata.
- Implementation / logic: Finds valid portal frames and fills the interior with portal states.
- Findings: Multiple writes do not surface every individual write result; a host refusal may leave partial creation. Exact frame/tag data is version-specific.
- RustChunk relevance: Useful isolated frame algorithm, dependent on host world lifecycle.
- Decision: adaptable; reference; partial-write validation required
- 1.20.1 implications: Rebuild 1.20.1 frame rules, axis/state IDs, dimensions and Forge portal hooks.
- Follow-up checks: Test frame variants, blocked writes, unloaded chunks, interior ordering and malformed frames.
- RustChunk implementation mapping: Port as a version-scoped portal-frame service with ordered writes and explicit result.
- Evidence required before adoption: Java 1.20.1 PortalShape/NetherPortalBlock sources, frame matrix, cancelled-write traces and Forge portal evidence.
- Completion state: COMPLETE — 184 logical lines through EOF; pinned SHA and 6799-byte size match; inspected, not modified; tests not executed.

### 2529. `crates/horizon-world/src/level/portal_travel.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3bd8a11f4c686ce183cbe31d18b830e27b513157`
- Size: 22767 bytes
- Coverage: Complete through EOF: 628 logical lines; ranges 1–200/201–400/401–600/601–628; no gaps.
- Purpose: Portal search, rectangle selection and exit-position calculation.
- Dependencies / cross-references: Portal helpers, dimension metadata, chunk search, entity dimensions/pose and collision callbacks.
- Implementation / logic: Finds nearest/largest portal areas, searches candidates, computes relative exit placement and creates a destination portal when needed.
- Findings: Tie-breaking and iteration order affect destination choice. Actual entity teleport, dimension transfer and collision resolution are host responsibilities; creation can partially fail.
- RustChunk relevance: Valuable travel geometry reference, not a complete cross-dimension entity implementation.
- Decision: adaptable; reference; dimension-and-entity integration required
- 1.20.1 implications: Match 1.20.1 search radius, world border, entity pose, custom dimensions and Forge travel hooks.
- Follow-up checks: Compare portal ties, nearest selection, exit collision, scaled coordinates, search limits and failed creation.
- RustChunk implementation mapping: Separate portal search/geometry from RustChunk's authoritative entity/dimension transfer service.
- Evidence required before adoption: Java 1.20.1 PortalForcer/PortalShape/entity travel sources and Forge custom-dimension traces.
- Completion state: COMPLETE — 628 logical lines through EOF; pinned SHA and 22767-byte size match; inspected, not modified; tests not executed.

### 2530. `crates/horizon-world/src/level/query.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5b293f81dc24369ad09e68ddd9b3d63891dd5c73`
- Size: 10213 bytes
- Coverage: Complete through EOF: 262 logical lines; full file; no gaps.
- Purpose: Shared block/fluid/support/collision and classification queries.
- Dependencies / cross-references: LevelAccess/Sim, BlockRegistry/StateTable, tags, voxel shapes and moving-piston records.
- Implementation / logic: Provides state, fluid, support, collision and block-class queries for behavior modules.
- Findings: Dynamic support/collision depends on block-entity state; static state-only queries cannot represent every moving or host-defined shape.
- RustChunk relevance: Reusable world-query seam with explicit dynamic context.
- Decision: adaptable; reference; dynamic-block-entity integration required
- 1.20.1 implications: Regenerate target block facts/shapes; Forge blocks can extend collision/support rules.
- Follow-up checks: Compare sturdy faces, occlusion, moving pistons and mod-defined collision/support.
- RustChunk implementation mapping: Use pure registry queries plus explicit host dynamic-shape callbacks.
- Evidence required before adoption: Java 1.20.1 BlockStateBase/VoxelShape support code and Forge custom-block conformance.
- Completion state: COMPLETE — 262 logical lines through EOF; pinned SHA and 10213-byte size match; inspected, not modified; tests not executed.

### 2531. `crates/horizon-world/src/level/rails.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cd71619b6461c6d8b5a64dc146d23507cb5e9ddd`
- Size: 27152 bytes
- Coverage: Complete through EOF: 756 logical lines; ranges 1–200/201–400/401–600/601–756; no gaps.
- Purpose: Rail shape updates, powered propagation, detector rails and tripwire hooks.
- Dependencies / cross-references: LevelAccess/Sim, block properties, redstone, neighbor updates, scheduled ticks and entity/minecart queries.
- Implementation / logic: Recomputes rail connections/shapes, updates powered chains and checks detector/tripwire conditions.
- Findings: Direction/iteration order and propagation bounds are observable; minecart entity queries are supplied by the host.
- RustChunk relevance: Reference for redstone-connected rail state; no minecart physics here.
- Decision: adaptable; reference; minecart-integration required
- 1.20.1 implications: Use 1.20.1 rail properties and entity types; support Forge rail extensions explicitly.
- Follow-up checks: Test junctions, slopes, powered propagation length, detector bounds and tripwire limits against Java.
- RustChunk implementation mapping: Keep block transitions in world core and supply bounded entity queries through a host API.
- Evidence required before adoption: Java 1.20.1 rail/tripwire sources, deterministic layouts and Forge rail/minecart tests.
- Completion state: COMPLETE — 756 logical lines through EOF; pinned SHA and 27152-byte size match; inspected, not modified; tests not executed.

### 2532. `crates/horizon-world/src/level/redstone.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `590f3bcdb80ffe7b3bd2556080e91fefc12bf4fc`
- Size: 59404 bytes
- Coverage: Complete through EOF: 1507 logical lines; ranges 1–200/201–400/401–600/601–800/801–1000/1001–1200/1201–1400/1401–1507; no gaps.
- Purpose: Vanilla redstone signal and block component behavior.
- Dependencies / cross-references: LevelAccess/Sim, StateTable, tags/shapes, neighbor updater, ticks, comparator outputs and experimental helpers.
- Implementation / logic: Implements signal strength, wire recomputation, torches, repeaters, comparators, buttons, observers and related block behavior.
- Findings: Exact direction/class order, emulated Java collection order, event sequencing and RNG draws are semantic. Pinned source is 59,404 bytes and 1,507 lines; this corrects the earlier erroneous progress estimate.
- RustChunk relevance: Strong candidate for a dedicated deterministic world-simulation subsystem after target-version regeneration.
- Decision: adaptable; core candidate; Java parity required
- 1.20.1 implications: Rebuild class/property/tag maps for 1.20.1 and gate experimental behavior exactly; later registry content is not target proof.
- Follow-up checks: Diff circuits, signal strengths, wire loops, burnout, comparator/repeater timing, observer pulses, update order and RNG traces.
- RustChunk implementation mapping: Implement against RustChunk authoritative versioned world state with observable ordered neighbor changes.
- Evidence required before adoption: Java 1.20.1 redstone sources, deterministic circuit corpus, RNG/event traces and Forge redstone hooks.
- Completion state: COMPLETE — 1507 logical lines through EOF; pinned SHA and 59404-byte size match; inspected, not modified; tests not executed.

### 2533. `crates/horizon-world/src/level/redstone/experimental.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b7cc9d6b621be93408c936d1496d0f6474103741`
- Size: 8046 bytes
- Coverage: Complete through EOF: 214 logical lines; full file; no gaps.
- Purpose: Experimental redstone graph evaluation and ordered propagation.
- Dependencies / cross-references: Redstone module, orientation context, state properties, LevelAccess and neighbor updates.
- Implementation / logic: Builds local wire graph, evaluates signal and applies changes in discovery order.
- Findings: An assertion assumes a monotonic signal invariant; prove it for every legal experimental configuration. Separate from ordinary redstone logic.
- RustChunk relevance: Optional reference only; exclude from stable world core absent exact feature.
- Decision: reference; experimental-feature-gated; do not enable by default
- 1.20.1 implications: Confirm the exact 1.20.1 experiment/configuration and algorithm before considering it.
- Follow-up checks: Test cycles, loops, source removal, orientation bias and assertion reachability against target Java.
- RustChunk implementation mapping: Keep isolated behind a disabled-by-default versioned feature interface.
- Evidence required before adoption: Java 1.20.1 experimental redstone source/config and deterministic graph traces.
- Completion state: COMPLETE — 214 logical lines through EOF; pinned SHA and 8046-byte size match; inspected, not modified; tests not executed.

### 2534. `crates/horizon-world/src/level/removal.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6fe55c79073c1affb1e5024637f111982dbd379d`
- Size: 6657 bytes
- Coverage: Complete through EOF: 163 logical lines; full file; no gaps.
- Purpose: Block-removal side effects, block-entity inventory spill and drop/experience dispatch.
- Dependencies / cross-references: LevelAccess/Sim, block-entity hooks, registry, loot/drop events and random source.
- Implementation / logic: Dispatches pre-removal behavior and emits inventory/drop/experience-related events before state replacement.
- Findings: Drop location and RNG order matter; fixed block-entity mapping needs an extension path for arbitrary Forge block entities.
- RustChunk relevance: Important lifecycle reference; item stacks, loot tables and entity spawning are host-owned.
- Decision: adaptable; reference; block-entity-and-drop integration required
- 1.20.1 implications: Regenerate the 1.20.1 block-entity set and implement Forge pre-remove callbacks.
- Follow-up checks: Verify inventory spill, empty containers, creative/silk-touch, drop offsets, XP and failed writes.
- RustChunk implementation mapping: Use typed pre-remove events; host owns inventory, loot, item entities and XP.
- Evidence required before adoption: Java 1.20.1 BlockEntity/container removal sources and Forge block-entity tests.
- Completion state: COMPLETE — 163 logical lines through EOF; pinned SHA and 6657-byte size match; inspected, not modified; tests not executed.

### 2535. `crates/horizon-world/src/level/sculk.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `46073cf5e7146a4a1223f4b21f03b8b2a2ba0184`
- Size: 32236 bytes
- Coverage: Complete through EOF: 871 logical lines; ranges 1–200/201–400/401–600/601–800/801–871; no gaps.
- Purpose: Sculk charge cursors, spreader merge/decay, catalyst, shrieker and growth.
- Dependencies / cross-references: LevelAccess/Sim, SculkSpreader, StateTable, tags, shapes and random source.
- Implementation / logic: Merges and advances charge cursors, finds spread targets, places sculk/veins and emits events/particles.
- Findings: Cursor cap/merge policy and traversal/RNG order are observable; wrapping distance semantics must be compared to Java integer behavior.
- RustChunk relevance: Useful 1.20-era behavior reference, tied to versioned registry data.
- Decision: adaptable; reference; version-and-RNG parity required
- 1.20.1 implications: Regenerate 1.20.1 sculk tags/states and exact spread behavior; do not carry later registry IDs.
- Follow-up checks: Compare cursor cap/merge, decay, distance edge cases, target scan order, XP/particles and random draws.
- RustChunk implementation mapping: Implement data-driven SculkSpreader in RustChunk versioned world simulation.
- Evidence required before adoption: Java 1.20.1 SculkSpreader/blocks sources, cursor persistence and Forge tag extension tests.
- Completion state: COMPLETE — 871 logical lines through EOF; pinned SHA and 32236-byte size match; inspected, not modified; tests not executed.

### 2536. `crates/horizon-world/src/level/shapes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e8a31f83cc7b98981a2c1d18befdfe6116c9a26f`
- Size: 50532 bytes
- Coverage: Complete through EOF: 1323 logical lines; ranges 1–200/201–400/401–600/601–800/801–1000/1001–1200/1201–1323; no gaps.
- Purpose: Neighbor-dependent block update-shape behavior for many block families.
- Dependencies / cross-references: LevelAccess/Sim, class-chain dispatch, state properties, tags, fluids, collision and neighbor updates.
- Implementation / logic: Computes updateShape-style state changes for walls, fences, rails, stairs, doors, plants and more.
- Findings: Class-chain and direction sequence are semantic; later block classes are mixed into dispatcher. Never alphabetically sort/flatten dispatch.
- RustChunk relevance: Broad reference for architecture; needs a target-generated dispatch table.
- Decision: adaptable; reference; target-version class-dispatch required
- 1.20.1 implications: Remove later blocks and use exact 1.20.1 properties, tags, order and Forge callbacks.
- Follow-up checks: Test each target block update family, multiple neighbors, waterlogging and unloaded boundaries.
- RustChunk implementation mapping: Generate 1.20.1 dispatch and preserve ordered shape updates.
- Evidence required before adoption: Java 1.20.1 updateShape sources, neighbor traces and Forge mod block hooks.
- Completion state: COMPLETE — 1323 logical lines through EOF; pinned SHA and 50532-byte size match; inspected, not modified; tests not executed.

### 2537. `crates/horizon-world/src/level/shelf.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f060fa530ae6665d61a9b05d5a4989e6750c46cb`
- Size: 8291 bytes
- Coverage: Complete through EOF: 241 logical lines; full file; no gaps.
- Purpose: Modern shelf pairing, placement and interaction behavior.
- Dependencies / cross-references: LevelAccess/Sim, StateTable, item/slot interactions, tags and events.
- Implementation / logic: Computes shelf chain/part states and shelf interactions.
- Findings: This shelf family is absent from vanilla 1.20.1; exclude its block/state data from target vanilla tables.
- RustChunk relevance: Not applicable to vanilla 1.20.1; only a future mod-specific reference.
- Decision: not applicable to 1.20.1; do not ship
- 1.20.1 implications: No vanilla 1.20.1 shelf equivalent; any mod implementation belongs in its own adapter.
- Follow-up checks: No vanilla parity work; future shelf mod must provide its registry/source/version evidence.
- RustChunk implementation mapping: Keep outside the 1.20.1 vanilla core.
- Evidence required before adoption: Proof from target 1.20.1 registry that absent; separate mod source/license and tests if ever needed.
- Completion state: COMPLETE — 241 logical lines through EOF; pinned SHA and 8291-byte size match; inspected, not modified; tests not executed.

### 2538. `crates/horizon-world/src/level/signals.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `95b7130d7532ae06d41b0aa2330a06b116625fd1`
- Size: 1610 bytes
- Coverage: Complete through EOF: 45 logical lines; full file; no gaps.
- Purpose: Analog/redstone output notification helpers.
- Dependencies / cross-references: LevelAccess/Sim, block registry, output signal queries and neighbor notifications.
- Implementation / logic: Compares output states and triggers neighbor notifications for analog signal changes.
- Findings: Small but order-sensitive bridge; missed host notification leaves stale comparator/wire state.
- RustChunk relevance: Useful seam for central mutation/redstone pipeline.
- Decision: adaptable; reference; redstone-host-contract required
- 1.20.1 implications: Use 1.20.1 comparator semantics and Forge analog output hooks.
- Follow-up checks: Test unchanged-state output change, comparator adjacency, flags and callback order.
- RustChunk implementation mapping: Integrate with RustChunk central set-block/redstone notification pipeline.
- Evidence required before adoption: Java 1.20.1 Level analog signal sources and Forge output tests.
- Completion state: COMPLETE — 45 logical lines through EOF; pinned SHA and 1610-byte size match; inspected, not modified; tests not executed.

### 2539. `crates/horizon-world/src/level/speleothem.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f9781279e9bfde6471e2cf793f04525605b58fdc`
- Size: 14511 bytes
- Coverage: Complete through EOF: 382 logical lines; full file; no gaps.
- Purpose: Pointed-dripstone growth, survival, fluid drip and cauldron filling.
- Dependencies / cross-references: LevelAccess/Sim, shape queries, fluids, random source, cauldron state and events.
- Implementation / logic: Handles tip transitions, growth/collapse, drip delays and cauldron fill events.
- Findings: Random draws, direction scans and entity fall damage callbacks are observable; entity outcomes are host-handled.
- RustChunk relevance: Useful 1.20-era block behavior reference requiring exact target RNG and entity handling.
- Decision: adaptable; reference; fluid-RNG-and-entity integration required
- 1.20.1 implications: Verify exact 1.20.1 drip probabilities, fluid rules, cauldron callbacks and Forge fluid hooks.
- Follow-up checks: Compare growth, tip states, drip chance/delay, cauldron sequence and falling stalactite outcomes.
- RustChunk implementation mapping: Implement in versioned behavior and route damage/spawning through typed events.
- Evidence required before adoption: Java 1.20.1 PointedDripstone sources, RNG traces and Forge fluid/entity cases.
- Completion state: COMPLETE — 382 logical lines through EOF; pinned SHA and 14511-byte size match; inspected, not modified; tests not executed.

### 2540. `crates/horizon-world/src/level/states.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8d2544953e51ed2db08fcf0d2ea44925050e6972`
- Size: 13403 bytes
- Coverage: Complete through EOF: 351 logical lines; ranges 1–200/201–351; no gaps.
- Purpose: Arithmetic block-state property indexing and stride computation.
- Dependencies / cross-references: BlockRegistry descriptors, StateTable, BlockStateId and property ordering.
- Implementation / logic: Derives property slots/strides from contiguous state IDs and provides get/set/cycle/default operations.
- Findings: Correctness depends on exact Cartesian-product and registry order; bounded identifiers and unchecked lookups require valid generated target metadata.
- RustChunk relevance: Registry-critical; state mismatch would corrupt all downstream block behavior.
- Decision: adaptable; registry-critical; 1.20.1 generation required
- 1.20.1 implications: Generate exact 1.20.1 block/state/property table; prove Java StateDefinition order.
- Follow-up checks: Exhaustively compare every target state/property, defaults, cycles, ID limits and malformed registries.
- RustChunk implementation mapping: Use generated tables with strict load-time validation and fail closed on ordering mismatch.
- Evidence required before adoption: Java 1.20.1 StateDefinition/BlockState sources, hashed registry dump and exhaustive mapping.
- Completion state: COMPLETE — 351 logical lines through EOF; pinned SHA and 13403-byte size match; inspected, not modified; tests not executed.

### 2541. `crates/horizon-world/src/level/survive.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d0788b40534a2ea0f43a88f86f706e5b451e986`
- Size: 26331 bytes
- Coverage: Complete through EOF: 616 logical lines; ranges 1–200/201–400/401–600/601–616; no gaps.
- Purpose: Block canSurvive/mayPlaceOn rules for numerous block classes.
- Dependencies / cross-references: LevelAccess/Sim, class-chain data, state properties, tags, support faces, light and fluids.
- Implementation / logic: Checks support, neighboring states, light/fluid, paired halves, plants, rails, torches and host survival overrides.
- Findings: First matching class-chain rule and exact target tags matter; missing dynamic support facts may silently diverge.
- RustChunk relevance: Strong placement-rule reference after target versioning.
- Decision: adaptable; reference; target-version class-and-tag map required
- 1.20.1 implications: Regenerate 1.20.1 class/tag data and Forge canSurvive/mayPlaceOn overrides.
- Follow-up checks: Test each target override, class inheritance, dynamic piston support, waterlogging and mod hooks.
- RustChunk implementation mapping: Implement as separate versioned survival service over explicit world queries.
- Evidence required before adoption: Java 1.20.1 BlockState survival sources, tags and Forge survival tests.
- Completion state: COMPLETE — 616 logical lines through EOF; pinned SHA and 26331-byte size match; inspected, not modified; tests not executed.

### 2542. `crates/horizon-world/src/level/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4f4a92bd06ca0ef324ed299c0561c0b301549406`
- Size: 118047 bytes
- Coverage: Complete through EOF: 3515 logical lines; ranges 1–200/201–400/601–800/801–1000/1001–1200/1201–1400/1401–1600/1601–1800/1801–2000/2001–2200/2201–2400/2401–2600/2601–2800/2801–3000/3001–3200/3201–3400/3401–3515; no gaps.
- Purpose: Integration test harness for blocks, fluids, redstone, pistons, interactions, sculk and ticks.
- Dependencies / cross-references: Rust tests, fixture registry/resources, Behaviours and mock LevelAccess.
- Implementation / logic: Builds in-memory test worlds and asserts block states, events, scheduled ticks and interaction results.
- Findings: Fixture loader can return None when block-behaviours.json is missing and each test returns early; suite may silently skip without explicit resource/skip gates. Mock assumes loaded chunks.
- RustChunk relevance: Use the patterns for a RustChunk differential/regression harness; never ship this test source.
- Decision: fixture-only; do not ship
- 1.20.1 implications: Create 1.20.1 fixture resources and fail CI on required test skips; later block cases are not target proof.
- Follow-up checks: Check asset presence and skip count, then add unloaded/bounds/error mocks and Java differential traces.
- RustChunk implementation mapping: Rebuild selected scenarios in a test-only 1.20.1 crate; exclude from Android release.
- Evidence required before adoption: Non-skipped test output, pinned resource hashes, Java 1.20.1 state/event/RNG traces and Forge negative-path tests.
- Completion state: COMPLETE — 3515 logical lines through EOF; pinned SHA and 118047-byte size match; inspected, not modified; tests not executed.

### 2543. `crates/horizon-world/src/level/tick.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b1fedc8897f9e03dba483369d4a8a078146366de`
- Size: 79690 bytes
- Coverage: Complete through EOF: 1940 logical lines; ranges 1–200/201–400/401–600/601–800/801–1000/1001–1200/1201–1400/1401–1600/1601–1800/1801–1940; no gaps.
- Purpose: Scheduled block/fluid ticks, random ticks and class-specific block tick logic.
- Dependencies / cross-references: Behaviours/Sim, LevelAccess, Ticks, registry/state data, fluids, weather and random source.
- Implementation / logic: Runs scheduled block then fluid ticks, per-block random behavior and chunk random-tick traversal with a 4096-bit candidate mask.
- Findings: Queue errors can return early or be ignored; tick order, random draws and mask refresh are optimization-sensitive.
- RustChunk relevance: Core simulation candidate; scheduler/RNG correctness affects nearly every behavior.
- Decision: adaptable; core candidate; 1.20.1 tick parity required
- 1.20.1 implications: Regenerate block classes/tags and exact 1.20.1 queue, game-rule and weather semantics.
- Follow-up checks: Compare block-before-fluid order, priorities, deduplication, tick cap, LCG, section traversal, mask invalidation and RNG traces.
- RustChunk implementation mapping: Create deterministic versioned scheduler/tick dispatcher with explicit queue error reporting.
- Evidence required before adoption: Java 1.20.1 LevelTicks/ServerLevel/BlockState tick sources and exact tick/RNG traces.
- Completion state: COMPLETE — 1940 logical lines through EOF; pinned SHA and 79690-byte size match; inspected, not modified; tests not executed.

### 2544. `crates/horizon-world/src/level/ticks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa0d882cb0972bb2251bcde20a9d62b2c6836511`
- Size: 7117 bytes
- Coverage: Complete through EOF: 182 logical lines; full file; no gaps.
- Purpose: Per-level scheduled tick state, neighbor updater and random tick position generator.
- Dependencies / cross-references: horizon-core LevelTicks/ChunkTicks, BlockId/FluidKind, NeighborUpdater and MovingPiston.
- Implementation / logic: Owns block/fluid queues, shared sub-tick counter, random LCG, in-flight neighbor updates, block events and captured worldgen ticks.
- Findings: Chunk lifecycle, save/load of ticks and transient entity/event collections remain caller responsibilities.
- RustChunk relevance: Useful deterministic world-state component, not a complete persistence model.
- Decision: adaptable; core candidate; persistence-and-order verification required
- 1.20.1 implications: Match 1.20.1 queue persistence, priority ordering and chunk load/unload semantics.
- Follow-up checks: Test duplicate ticks, same-time ordering, chunk reload, shared block/fluid counter and capture mode.
- RustChunk implementation mapping: Create RustChunk tick-state storage with explicit persistence and chunk ownership contracts.
- Evidence required before adoption: Java 1.20.1 LevelTicks/SavedTick sources, serialized fixtures and reload/order traces.
- Completion state: COMPLETE — 182 logical lines through EOF; pinned SHA and 7117-byte size match; inspected, not modified; tests not executed.

### 2545. `crates/horizon-world/src/level/timeline.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ef96f5a792316690a4b1cf652e971f63369a7c2`
- Size: 9389 bytes
- Coverage: Complete through EOF: 235 logical lines; full file; no gaps.
- Purpose: Time-window environment behavior for eyeblossoms, turtle eggs and dried ghasts.
- Dependencies / cross-references: LevelAccess/Sim, dimension/day time, random source, block registry and events.
- Implementation / logic: Maps day time to timed block actions, schedules nearby changes, ticks turtle eggs and handles dried-ghast hydration.
- Findings: Eyeblossom and dried-ghast behavior postdates 1.20.1; isolate turtle egg behavior and verify exact target schedule/random window.
- RustChunk relevance: Selective reference only; do not adopt module wholesale.
- Decision: version-specific; selective reference; later blocks excluded
- 1.20.1 implications: Exclude later blocks; compare turtle egg hatch timing/chance and event behavior with 1.20.1.
- Follow-up checks: Test target day-time normalization, hatch windows, event order and RNG traces.
- RustChunk implementation mapping: Separate generic timeline service from version-specific block callbacks.
- Evidence required before adoption: Java 1.20.1 TurtleEggBlock/world timeline sources and deterministic day-time traces.
- Completion state: COMPLETE — 235 logical lines through EOF; pinned SHA and 9389-byte size match; inspected, not modified; tests not executed.

### 2546. `crates/horizon-world/src/level/update.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b894d4e5427da89099c7506ae07f26a32d789b3`
- Size: 23320 bytes
- Coverage: Complete through EOF: 694 logical lines; ranges 1–200/201–400/401–600/601–694; no gaps.
- Purpose: Level.setBlock mutation pipeline, shape updates and chained neighbor dispatcher.
- Dependencies / cross-references: LevelAccess, Sim, NeighborUpdater, block registry, flags, block entities, events and Ticks.
- Implementation / logic: Validates bounds/loading, writes state, runs block-entity side effects and schedules ordered shape/neighbor notifications.
- Findings: One-million chained update cap drops excess work without a public error; raw writes and block identity can suppress later callbacks.
- RustChunk relevance: Central world mutation candidate; every gameplay transition depends on this contract.
- Decision: adaptable; core candidate; host-contract required
- 1.20.1 implications: Rebuild 1.20.1 flags, update order, block-entity lifecycle and Forge hooks.
- Follow-up checks: Test flags, failed writes, chunk bounds, block entities, moving piston, update cap and exact neighbor order.
- RustChunk implementation mapping: Create one authoritative RustChunk state mutation pipeline with typed side effects and observable budget exhaustion.
- Evidence required before adoption: Java 1.20.1 Level/LevelChunk/NeighborUpdater sources and Forge event traces.
- Completion state: COMPLETE — 694 logical lines through EOF; pinned SHA and 23320-byte size match; inspected, not modified; tests not executed.

### 2547. `crates/horizon-world/src/level/weather.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `93027d889375454e6952f938e6d3266ed3fbd46c`
- Size: 5551 bytes
- Coverage: Complete through EOF: 138 logical lines; full file; no gaps.
- Purpose: Biome precipitation, motion-blocking surface and local rain/freeze/snow queries.
- Dependencies / cross-references: Behaviours/LevelAccess, biome temperature data, dimension sea levels, block tags, fluids and light.
- Implementation / logic: Computes precipitation kind, finds surface position, checks freezing water, snow placement and rain exposure.
- Findings: Sea levels are hardcoded and top lookup scans a vertical column instead of using an authoritative heightmap; target biome/settings are external.
- RustChunk relevance: Weather query architecture reference with host-provided biome and heightmap services.
- Decision: adaptable; reference; biome-heightmap integration required
- 1.20.1 implications: Use 1.20.1 biome temperatures, precipitation, dimension sea level and exact heightmap semantics; custom Forge dimensions need data.
- Follow-up checks: Compare thresholds, heights, light, biome boundaries and custom dimensions with target Java.
- RustChunk implementation mapping: Use chunk heightmaps and versioned biome/dimension metadata, not repeated scans.
- Evidence required before adoption: Java 1.20.1 Biome/ServerLevel precipitation sources, heightmap fixtures and Forge biome tests.
- Completion state: COMPLETE — 138 logical lines through EOF; pinned SHA and 5551-byte size match; inspected, not modified; tests not executed.

### 2548. `crates/horizon-world/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f094b34d5fd9fec03ebdcde82fb04c5ba5eff873`
- Size: 3484 bytes
- Coverage: Complete through EOF: 152 logical lines; full file; no gaps.
- Purpose: Public crate module surface, unsafe-code policy and data-version constant.
- Dependencies / cross-references: Rust module declarations, horizon-core version constant and submodules.
- Implementation / logic: Exports module surface, forbids unsafe code and aliases DATA_VERSION.
- Findings: This is a module inventory only, not game implementation; listed modules belong to a newer source snapshot.
- RustChunk relevance: Navigation/architecture reference only.
- Decision: architectural reference; version-specific; do not copy wholesale
- 1.20.1 implications: Design RustChunk crates around selected 1.20.1 systems and own DataVersion.
- Follow-up checks: Inspect selected subsystem dependency graph and target version constants.
- RustChunk implementation mapping: Use only as a map for decomposition, not copied crate structure.
- Evidence required before adoption: RustChunk-owned crate graph and 1.20.1 DataVersion source.
- Completion state: COMPLETE — 152 logical lines through EOF; pinned SHA and 3484-byte size match; inspected, not modified; tests not executed.

### 2549. `crates/horizon-world/src/liquid_collision.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `89bc88f624f449e033a1e837798ef22381d3a9d8`
- Size: 2214 bytes
- Coverage: Complete through EOF: 63 logical lines; full file; no gaps.
- Purpose: Context-sensitive water/lava collision selection and entity standing-on-fluid behavior.
- Dependencies / cross-references: BlockState, CollisionContext, FluidState, EntityIdentity and LiquidCollisionWorld host callbacks.
- Implementation / logic: Selects source liquid collision shape based on always-collide mode, living entity shape and fluid above.
- Findings: Callback order and entity shape/standing behavior are host responsibilities; flowing fluids return empty collision.
- RustChunk relevance: Useful isolated world/entity collision adapter.
- Decision: adaptable; reference; entity-collision integration required
- 1.20.1 implications: Match 1.20.1 fluid collision context and Forge custom fluid/entity behavior.
- Follow-up checks: Test source/flowing fluids, above-fluid callback order, empty shapes and custom entity cases.
- RustChunk implementation mapping: Port as a typed host callback boundary without hidden entity storage.
- Evidence required before adoption: Java 1.20.1 LiquidBlock/CollisionContext and Forge fluid/entity tests.
- Completion state: COMPLETE — 63 logical lines through EOF; pinned SHA and 2214-byte size match; inspected, not modified; tests not executed.

### 2550. `crates/horizon-world/src/live_attribute_updates.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `632a414385532fe13226da13d3ad77c62fba6353`
- Size: 3544 bytes
- Coverage: Complete through EOF: 85 logical lines; full file; no gaps.
- Purpose: Live entity attribute reads and pending-update refresh dispatch.
- Dependencies / cross-references: LiveAttributes, AttributeMap, UpdateWorld, instance handles, cursor and entity callbacks.
- Implementation / logic: Reads attribute values and iterates pending instances to dispatch specialized attribute refresh before clearing pending updates.
- Findings: Callbacks may mutate map selection or pending cursor; failures must not clear the set. Only a small registered-attribute subset is specially classified.
- RustChunk relevance: Entity subsystem integration pattern; not a world/block module.
- Decision: adaptable; reference; entity-attribute-host integration required
- 1.20.1 implications: Use 1.20.1 attribute holders/instances and Forge custom attribute registry/hooks.
- Follow-up checks: Test map replacement, cursor invalidation, callback failure, custom attributes and refresh ordering.
- RustChunk implementation mapping: Keep storage and refresh in a dedicated RustChunk entity crate behind typed world/attribute traits.
- Evidence required before adoption: Java 1.20.1 AttributeMap/LivingEntity sources, callback mutation traces and Forge custom-attribute tests.
- Completion state: COMPLETE — 85 logical lines through EOF; pinned SHA and 3544-byte size match; inspected, not modified; tests not executed.

## Checkpoint 2526–2550 — COMPLETE (25/25)

- All 25 source files were re-read from the start, one at a time, through EOF. The pinned recursive tree at `aa193ad1daa40ab307340c4a61dc6b0176bd0e23` is untruncated; all 25 paths are blobs and each SHA/byte-size pair in these entries matches that tree.
- Long-file coverage is recorded per entry; every range is contiguous, sequential and no larger than 200 logical lines. Smaller files were read whole.
- Important findings: double-high placement may expose a partial-write ordering risk; portal creation has multiple writes whose refusal behavior must be checked; dynamic block/entity collision context is delegated to the host; redstone and shape dispatch depend on exact order; the state table assumes Java-compatible registry ordering; tests silently skip without prepared resources; scheduled tick and neighbor-update queue failures need explicit host handling; weather uses hardcoded sea levels and vertical scans.
- `redstone.rs` metadata corrected from an earlier progress estimate: pinned blob is 59,404 bytes and 1,507 logical lines.
- Ledger: **150 records (2401–2550)**, exactly 15 required fields each. Index: **2,550 sequential unique paths** with path/Decision alignment. Structural/provenance QA for 2401–2550 is recorded in the checkpoint QA entry.
- Tests/examples inspected only; none executed. No application source changes or build/APK/device/Minecraft/Forge/server/Aternos validation. No backup at 2550.
- The user's latest instruction makes this the stop boundary. **STOP at 2550.** Entries 2551–2600 require a separate explicit authorization. After that range is complete, recheck all 200 entries 2401–2600 as requested.



### 2551. `crates/horizon-world/src/living_air.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dec52f4bc624adc56b9cd3332bdcda3096f08e99`
- Size: 6488 bytes
- Coverage: Complete through EOF: 171 logical lines; full file; no gaps.
- Purpose: Default airborne travel, relative input, ladder clamping and friction calculations.
- Dependencies / cross-references: LivingAirWorld; horizon_core Vec3/BlockPos/trig; gravity, fluid, movement-support and entity callbacks.
- Implementation / logic: Normalizes local input, rotates it by native trig, updates velocity and delegates movement/collision; applies climbing, levitation/gravity, missing-client-chunk fallback and drag.
- Findings: Callback order and mutable host state are observable; move_self must perform collision/landing. Float narrowing, input threshold, ladder logic and trig table are behavior-critical.
- RustChunk relevance: Reusable movement-physics reference; requires target Java parity, not direct data reuse.
- Decision: adaptable; reference
- 1.20.1 implications: Rebuild from Minecraft 1.20.1 Entity/LivingEntity movement and Forge attributes/effects; gate later-only modifiers.
- Follow-up checks: Compare float bits for input/yaw, friction thresholds, ladder/powder snow, levitation, gravity and missing-client-chunk branches.
- RustChunk implementation mapping: Put in RustChunk entity-physics with narrow host movement callbacks; preserve Entity.move as collision boundary.
- Evidence required before adoption: Mapped 1.20.1 Java movement sources, ARM64 float traces and collision callback-order tests.
- Completion state: COMPLETE — 171 logical lines through EOF; pinned SHA and 6488 byte size match; inspected, not modified; tests not executed.

### 2552. `crates/horizon-world/src/living_damage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9abf38199fa7d804fad9479f087407caaab56d88`
- Size: 3696 bytes
- Coverage: Complete through EOF: 104 logical lines; full file; no gaps.
- Purpose: Inner living/player damage application after immunity and outer hit gates.
- Dependencies / cross-references: DamageWorld; armor/magic mitigation, absorption, health, exhaustion, statistics and damage-event services.
- Implementation / logic: Applies armor then magic mitigation, updates absorption/statistics, returns on fully absorbed hits, then exhaustion, damage tracking, health loss and event.
- Findings: Repeated absorption reads and partial writes are observable; callback errors do not roll back earlier effects. Java Math.round, NaN and saturation behavior need exact checks.
- RustChunk relevance: Core combat behavior reference; service boundary useful but implementation must be target-version-derived.
- Decision: adaptable; reference
- 1.20.1 implications: Use 1.20.1 DamageSource, CombatRules, absorption and exhaustion semantics; verify Forge damage hooks/attributes.
- Follow-up checks: Trace partial/full absorption, player/non-player, NaN/infinity, stat overflow and failure at every mutation boundary.
- RustChunk implementation mapping: Implement ordered damage pipeline in RustChunk gameplay with one authoritative entity-state provider.
- Evidence required before adoption: Java 1.20.1 source traces, float/int edge corpus and Forge event/attribute tests.
- Completion state: COMPLETE — 104 logical lines through EOF; pinned SHA and 3696 byte size match; inspected, not modified; tests not executed.

### 2553. `crates/horizon-world/src/living_fall.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `df44dd9e5edcd431cca4e35358b2e4da101b3cc0`
- Size: 8040 bytes
- Coverage: Complete through EOF: 240 logical lines; full file; no gaps.
- Purpose: Fall-distance damage, passenger propagation and impulse-based fall-damage exemptions.
- Dependencies / cross-references: LivingFallWorld; Vec3 impact context; fall immunity, passenger damage, sound and hurt callbacks.
- Implementation / logic: Tracks impact position/grace; clamps fall distance by impact height; calculates floor-based damage; propagates to passengers and then orders modification, sounds and hurt.
- Findings: Impulse/grace context includes later wind-charge/mace-era behavior absent in 1.20.1. Passenger notification differs from final damage result; immunity is queried twice.
- RustChunk relevance: Fall damage order/formula is reusable reference; impulse exemption is later-version-specific.
- Decision: adaptable; version-specific; reference
- 1.20.1 implications: Port only 1.20.1 baseline fall damage; exclude modern mace/wind-charge impulse state.
- Follow-up checks: Compare epsilon, rider propagation, immunity query count, sound threshold, floor conversion and damage-source retention.
- RustChunk implementation mapping: Separate baseline fall calculation from optional target-version extensions; delegate landing/collision to world movement.
- Evidence required before adoption: Java 1.20.1 Entity/LivingEntity source, rider traces and explicit evidence for any impulse mechanic.
- Completion state: COMPLETE — 240 logical lines through EOF; pinned SHA and 8040 byte size match; inspected, not modified; tests not executed.

### 2554. `crates/horizon-world/src/living_fluid.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `44cf7be241407268cb415292ac9b1cb7fbaa2924`
- Size: 5172 bytes
- Coverage: Complete through EOF: 154 logical lines; full file; no gaps.
- Purpose: Water/lava travel, fluid escape and ridden-entity buoyancy.
- Dependencies / cross-references: LivingAirWorld; fluid-height/jump-threshold, efficiency, sprint, dolphins-grace and vehicle callbacks.
- Implementation / logic: Blends water slowdown/speed by efficiency, halves efficiency off-ground, applies climb/escape behavior; applies lava damping/gravity and ridden buoyancy.
- Findings: Any non-water state is sent through lava branch; caller must guard dispatch domain. Fluid tags, floatable-while-ridden behavior and movement modifiers are host/version-specific.
- RustChunk relevance: Physics structure useful; fluid identities/data require target-era reconstruction.
- Decision: adaptable; reference
- 1.20.1 implications: Recreate 1.20.1 water/lava and Forge fluid semantics; do not assume modern EntityFloatable or movement-efficiency attributes exist.
- Follow-up checks: Test water/lava boundary, caller domain, sprint/dolphins grace, partial fluid height, collision escape and ridden buoyancy.
- RustChunk implementation mapping: Integrate into entity travel with typed FluidState/tag lookups and guarded water/lava dispatch.
- Evidence required before adoption: Java 1.20.1 travel sources, Forge fluid-tag registration and per-tick velocity traces.
- Completion state: COMPLETE — 154 logical lines through EOF; pinned SHA and 5172 byte size match; inspected, not modified; tests not executed.

### 2555. `crates/horizon-world/src/living_health.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5b071e93f931f43df4a6217f583cc063eee64063`
- Size: 3646 bytes
- Coverage: Complete through EOF: 110 logical lines; full file; no gaps.
- Purpose: Living health/absorption limits, healing and synchronized-float dirty-state behavior.
- Dependencies / cross-references: HealthWorld; max-health/max-absorption attributes; synchronized data object; Java boxed-float equality.
- Implementation / logic: Clamps health/absorption to attributes, dispatches healing through virtual set-health, and models one synced float including callback/dirty order.
- Findings: SyncedFloat changes value before callback but sets dirty flags only after callback succeeds; failures retain new value with old dirty state. Repeated reads/attribute calls are deliberate.
- RustChunk relevance: Useful exact callback/state reference; one float helper is not a full data manager.
- Decision: adaptable; reference
- 1.20.1 implications: Rebuild against 1.20.1 SynchedEntityData and attributes; verify MAX_ABSORPTION registration in Forge 47.x.
- Follow-up checks: Test signed-zero/NaN equality, dirty state after errors, max-attribute narrowing and reentrant set-health.
- RustChunk implementation mapping: Add synced-data adapter and keep health logic separate from Android/UI.
- Evidence required before adoption: 1.20.1 data/attribute sources, callback exception traces and complete health-attribute registry.
- Completion state: COMPLETE — 110 logical lines through EOF; pinned SHA and 3646 byte size match; inspected, not modified; tests not executed.

### 2556. `crates/horizon-world/src/living_hurt.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d40fb8d9576d8c707f4cee9c2cc3f539be6d184c`
- Size: 7524 bytes
- Coverage: Complete through EOF: 213 logical lines; full file; no gaps.
- Purpose: Outer living damage dispatch, cooldown state and post-hit services.
- Dependencies / cross-references: HurtWorld/HurtState; damage tags, armor/helmet, blocking, freezing, death protection, stats, sounds, criteria and effects.
- Implementation / logic: Runs immunity/death/fire gates and hit-start callback; applies blocking/modifiers/helmet, cooldown and damage; then resolution, impact/death/sound, source/effects and criteria.
- Findings: Exact callback order/state writes/reentrancy are observable. Tag set, freezing, death protection and hook stages must be checked against 1.20.1; errors can leave partial state.
- RustChunk relevance: High-value combat pipeline reference, not a drop-in compatibility layer.
- Decision: adaptable; version-specific; reference
- 1.20.1 implications: Reconstruct from 1.20.1 LivingEntity.hurt and Forge event path; do not assume modern damage tags or hook APIs.
- Follow-up checks: Trace blocked/unblocked, cooldown partial/reject, helmet/fire, death protection and errors after each write.
- RustChunk implementation mapping: Use Rust-owned HurtState and ordered hooks with a Forge-facing compatibility adapter.
- Evidence required before adoption: Java 1.20.1 hurt/cooldown source, DamageType tags and Forge 47.x damage-event order.
- Completion state: COMPLETE — 213 logical lines through EOF; pinned SHA and 7524 byte size match; inspected, not modified; tests not executed.

### 2557. `crates/horizon-world/src/living_landing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cb28df84ca899cb4d5ebddcf6d0114d62aea97a0`
- Size: 2901 bytes
- Coverage: Complete through EOF: 69 logical lines; full file; no gaps.
- Purpose: Living landing fluid refresh, changed-block notification, landing particles and fall handling.
- Dependencies / cross-references: FallDamageWorld/LivingLandingWorld; world side/position, block state, particle and climbable-position callbacks.
- Implementation / logic: Refreshes fluid interaction; calls changed-block on server landing; computes safe-distance particle strength/position; calls fall logic and clears climbable position.
- Findings: Callback failures leave partial effects. Caller must pass original movement-phase state/landing coordinates. Edge normalization needs degenerate/NaN tests; level side differs from client flag.
- RustChunk relevance: Landing callback-order reference; block/particle behavior target-specific.
- Decision: Rebuild from 1.20.1 Entity.checkFallDamage/LivingEntity landing and Forge block callbacks.
- 1.20.1 implications: Test changed-block/particle/fall order, edge-center/degenerate positions, water refresh and level-side behavior.
- Follow-up checks: Keep in RustChunk world movement with typed block/particle sinks and no hidden writes.
- RustChunk implementation mapping: Java 1.20.1 landing source and particle coordinate/callback-failure tests.
- Evidence required before adoption: undefined
- Completion state: COMPLETE — 69 logical lines through EOF; pinned SHA and 2901 byte size match; inspected, not modified; tests not executed.

### 2558. `crates/horizon-world/src/living_motion.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9c496ed63ac75cc2f0000c8a738f4d99f9b46369`
- Size: 8144 bytes
- Coverage: Complete through EOF: 240 logical lines; full file; no gaps.
- Purpose: Default living jump, gravity, flying-speed and air-drag calculations.
- Dependencies / cross-references: LivingMotionWorld; Vec3/trig; jump boost, block jump factor, sprint, gravity/slow-falling and passenger callbacks.
- Implementation / logic: Computes narrowed jump power, optional sprint look impulse, gravity/slow-falling cap, swimming vertical speed, default flying speed and air drag.
- Findings: Float narrowing and trig order matter. Nonfinite added delta is ignored, but existing velocity/sum overflow is not; attribute/passenger callbacks are host-owned.
- RustChunk relevance: Core entity-motion algorithms are reusable references.
- Decision: adaptable; reference
- 1.20.1 implications: Use exact 1.20.1 jump/slow-falling/gravity definitions; gate newer omnidirectional-air-mover attribute.
- Follow-up checks: Compare jump rounding, sprint impulse across yaw, NaN velocity, gravity extremes and flying-speed branches.
- RustChunk implementation mapping: Implement deterministic entity-motion module behind RustChunk attribute/effect interfaces.
- Evidence required before adoption: Java 1.20.1 movement sources, bit-level float traces and Forge attribute tests.
- Completion state: COMPLETE — 240 logical lines through EOF; pinned SHA and 8144 byte size match; inspected, not modified; tests not executed.

### 2559. `crates/horizon-world/src/living_use_flags.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ae1ecb31ed5a10661beea44be4c3300fe6e00652`
- Size: 869 bytes
- Coverage: Complete through EOF: 26 logical lines; full file; no gaps.
- Purpose: Living-entity using-item and used-hand flags in synchronized byte storage.
- Dependencies / cross-references: SyncedValues<u8>; Observer; InteractionHand; byte field identifier.
- Implementation / logic: Reads bit 0 for using-item and bit 1 for off-hand; updates byte using signed extension, bit-mask and narrowing.
- Findings: Identifier 8 and flag layout are source-snapshot-specific; setter accepts arbitrary masks. Regenerate target SynchedEntityData IDs.
- RustChunk relevance: Small generic bit-flag pattern; numeric schema is version-specific.
- Decision: adaptable; version-specific; reference
- 1.20.1 implications: Regenerate entity-data IDs and bits from Minecraft 1.20.1; do not copy modern field index.
- Follow-up checks: Compare flag combinations, signed-byte edges, unknown IDs and observer order.
- RustChunk implementation mapping: Use typed RustChunk entity flags generated from a 1.20.1 schema.
- Evidence required before adoption: 1.20.1 SynchedEntityData definitions, flag traces and Forge hand synchronization.
- Completion state: COMPLETE — 26 logical lines through EOF; pinned SHA and 869 byte size match; inspected, not modified; tests not executed.

### 2560. `crates/horizon-world/src/loot.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `05fa2f2f7d395c5cdcc5447e82b6a84f709984c3`
- Size: 93839 bytes
- Coverage: Complete through EOF: 2,298 logical lines; full file; no gaps.
- Purpose: Large loot engine: data-pack overlays, pools/entries, conditions, modifiers, enchantments, stack splitting and entity context.
- Dependencies / cross-references: Serde JSON/NBT; LazyJson; LegacyRandom/RandomSource; resource loading; tags, recipes, item defaults, native load hooks and LootStack.
- Implementation / logic: Loads namespace loot tables/modifiers/predicates/tags/recipes; caches tables by root/file metadata and overlay; evaluates weighted pools, nested tables, conditions, item functions and enchantment selection with preserved RNG progression.
- Findings: Cache key hashes path/size/mtime rather than bytes; some directory/read errors are suppressed and overlay parse errors skipped. Recursion tags stop after depth 16. Current ItemStack components, recipe fields, body/saddle slots, predicates, instruments, stew/ominous functions and native hooks are not 1.20.1 schema; exploration search deferred to host.
- RustChunk relevance: Architecture and RNG-order reference only; data model/function dispatch require target-era rewrite.
- Decision: rewrite required; version-specific; reference
- 1.20.1 implications: Minecraft 1.20.1 ItemStack/NBT predates item components (1.20.5); regenerate all loot data, tags, recipes, enchantments and item facts for 1.20.1/Forge 47.x; do not copy modern corpus.
- Follow-up checks: Build 1.20.1 loot corpus; differential-test RNG and every pool/entry/condition/function, pack replacement, invalid/missing context, recursion/cache invalidation and stack splitting; add data bounds/error visibility.
- RustChunk implementation mapping: Split into target-version data loader, deterministic evaluator/RNG, item-stack/NBT model and Forge datapack/loot-event adapter; isolate extension hooks.
- Evidence required before adoption: Version-hashed 1.20.1 vanilla resources, mapped Java sources, Forge loot events, seed-by-seed oracle traces and adversarial datapack tests.
- Completion state: COMPLETE — 2,298 logical lines through EOF; pinned SHA and 93839 byte size match; inspected, not modified; tests not executed.

### 2561. `crates/horizon-world/src/main.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c88fb978eac2f4d46a0886e1aafd545fe9bbf7fe`
- Size: 7934 bytes
- Coverage: Complete through EOF: 198 logical lines; full file; no gaps.
- Purpose: Standalone CLI for region/NBT inspection, copying, block-data editing and codec conversion.
- Dependencies / cross-references: BlockPos; RegionFile/RegionWriter; ChunkData; NBT limits; codecs; filesystem and I/O.
- Implementation / logic: Implements inspect/copy-region/get-block/edit-block-data/nbt-copy/nbt-network-copy; bounds reads to NBT limit and refuses existing outputs.
- Findings: Not an Android client loop. Block edit explicitly omits lighting, heightmaps, ticks and neighbors. Write failure removes newly created output.
- RustChunk relevance: Useful diagnostic/developer tool reference, not runtime architecture.
- Decision: adaptable; optional; reference
- 1.20.1 implications: Keep codecs/Anvil version-aware; never expose saved-data editor as live-world mutation.
- Follow-up checks: Test oversized/malformed input, codecs, trailing NBT, output refusal, interrupted writes and Java 1.20.1 chunk validity.
- RustChunk implementation mapping: Separate developer/test binary, excluded from Android release unless deliberately packaged.
- Evidence required before adoption: 1.20.1 Anvil/NBT fixtures, CLI integration tests and regenerated derived-chunk data.
- Completion state: COMPLETE — 198 logical lines through EOF; pinned SHA and 7934 byte size match; inspected, not modified; tests not executed.

### 2562. `crates/horizon-world/src/movement_emission.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f762ced22f08f7e9cc801e16ad71758ae93e5d72`
- Size: 5607 bytes
- Coverage: Complete through EOF: 158 logical lines; full file; no gaps.
- Purpose: Movement distance accounting and step/swim/flap sound/event dispatch.
- Dependencies / cross-references: MovementEmissionWorld; Vec3/BlockPos; block state/tags, crouch/ground/rail/swim and sound/event host services.
- Implementation / logic: Accumulates movement/flight distances; selects legacy/current block contexts for step events; advances thresholds; dispatches swim or flap events.
- Findings: Callbacks may mutate distance fields and later reads observe them. Sounds/events use different state/position contexts; caller must enforce server/authority/passenger guards.
- RustChunk relevance: Movement side-effect ordering reference; target tags/sounds need regeneration.
- Decision: adaptable; reference
- 1.20.1 implications: Rebuild from 1.20.1 Entity movement emission and step/flap logic; regenerate climbable/powder-snow/sound tags.
- Follow-up checks: Compare float accumulation, Java int wrap in next-step, same/different positions, sounds-only/events-only and water/flap paths.
- RustChunk implementation mapping: Invoke as a movement side-effect stage after authoritative RustChunk guards.
- Evidence required before adoption: Java 1.20.1 emission source and trace tests for positions, callbacks and event packet order.
- Completion state: COMPLETE — 158 logical lines through EOF; pinned SHA and 5607 byte size match; inspected, not modified; tests not executed.

### 2563. `crates/horizon-world/src/movement_properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9e1dc623677cb398177fee6d530235a46e15dc73`
- Size: 7398 bytes
- Coverage: Complete through EOF: 205 logical lines; full file; no gaps.
- Purpose: Registry-bound block movement coefficients and reloadable movement tags.
- Dependencies / cross-references: BlockRegistry/BlockTable; NBT Tag/DATA_VERSION; MovementBlockWorld; movement_support.
- Implementation / logic: Loads per-block friction/speed/jump/bounce/fall coefficients from versioned NBT, requires definitions for every registry state, then binds fence/wall/bounce tags into a state-indexed snapshot.
- Findings: Raw float bits are accepted without finite/range validation. DataVersion and registry order are strict; bound tables preserve old tag snapshots across reload.
- RustChunk relevance: Useful generated-data architecture; values and registry tables belong to newer version.
- Decision: adaptable; version-specific; reference
- 1.20.1 implications: Regenerate every coefficient/tag from exact 1.20.1 block registry; confirm which bounce coefficients/tags exist.
- Follow-up checks: Test missing/extra blocks, NaN/infinity/ranges, registry reorder, tag reload snapshots and lookups.
- RustChunk implementation mapping: Use generated, version-hashed RustChunk movement tables with immutable reload bindings.
- Evidence required before adoption: 1.20.1 registry dump, Java formula sources, coefficient bounds and Forge tag reload tests.
- Completion state: COMPLETE — 205 logical lines through EOF; pinned SHA and 7398 byte size match; inspected, not modified; tests not executed.

### 2564. `crates/horizon-world/src/movement_restitution.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f994548ffeef9cff9073c81efb5298c565d4cd86`
- Size: 3604 bytes
- Coverage: Complete through EOF: 100 logical lines; full file; no gaps.
- Purpose: Post-collision bounce/restitution response and side-effect dispatch.
- Dependencies / cross-references: RestitutionWorld; block IDs; collision outcome; entity bounciness, gravity, drag and event callbacks.
- Implementation / logic: Computes horizontal/vertical rebound using entity/block restitution, collision direction, gravity fraction and drag; fires block callback/event and sync mark before storing velocity.
- Findings: Provider errors preserve prior side effects. Vertical gravity fraction divides by old Y; zero/nonfinite inputs need exact testing. Suppression callback may be queried multiple times.
- RustChunk relevance: Reusable collision formula reference; requires exact movement caller guards.
- Decision: adaptable; reference
- 1.20.1 implications: Rebuild from 1.20.1 slime/bounce semantics; newer tags/attributes require evidence.
- Follow-up checks: Test zero/near-zero old Y, NaN/signed zero, horizontal/vertical collision, suppression, nonliving factor and callback failures.
- RustChunk implementation mapping: Call after collision resolution; world provider owns block/entity collision decisions.
- Evidence required before adoption: Java 1.20.1 Entity/slime source, collision guard traces and float-bit differential tests.
- Completion state: COMPLETE — 100 logical lines through EOF; pinned SHA and 3604 byte size match; inspected, not modified; tests not executed.

### 2565. `crates/horizon-world/src/movement_statistics.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `39385cc18631514bdc8e8a925b34cbea34470c22`
- Size: 4514 bytes
- Coverage: Complete through EOF: 128 logical lines; full file; no gaps.
- Purpose: Server-player movement stats and movement-related exhaustion.
- Dependencies / cross-references: MovementStatisticsWorld; Vec3; passenger, water, climb, ground, sprint, crouch, flight and stat/exhaustion services.
- Implementation / logic: Chooses swim/underwater/water-surface/climb/sprint/crouch/walk/aviate/fly stats in ordered priority; converts distance to centimeters using Java rounding and applies exhaustion.
- Findings: Must run only after accepted server movement. Branch priority, float narrowing before scale/round and long-to-i32 wrap are observable; some flight stats may be later-version-only.
- RustChunk relevance: Useful separation of server accounting from physics.
- Decision: adaptable; reference
- 1.20.1 implications: Use exact 1.20.1 stat IDs and exhaustion rates; gate absent newer flight modes.
- Follow-up checks: Test half-tie centimeters, negative/overflow deltas, water/eyes priority, passenger skips, climb/sprint exhaustion.
- RustChunk implementation mapping: Invoke only from accepted authoritative server movement path, not client prediction.
- Evidence required before adoption: Java 1.20.1 ServerPlayer movement-stat source, packet listener trace and stat registry.
- Completion state: COMPLETE — 128 logical lines through EOF; pinned SHA and 4514 byte size match; inspected, not modified; tests not executed.

### 2566. `crates/horizon-world/src/movement_support.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d42f5d00896ecfe1150a70a08ef0b95cce41e9ef`
- Size: 3597 bytes
- Coverage: Complete through EOF: 95 logical lines; full file; no gaps.
- Purpose: Support-block position and block-derived jump/speed factors.
- Dependencies / cross-references: MovementBlockWorld; BlockPos/Vec3; fence/wall/gate/water/bubble and coefficient providers.
- Implementation / logic: Computes support block from captured supporting block and float offsets; reads current/lower block jump/speed factors in native short-circuit order.
- Findings: Offsets 0.2, 0.500001 and 1e-5 and float widening are exact. Jump always reads lower block; speed skips lower lookup for water/bubble or non-default current factor.
- RustChunk relevance: Compact entity movement rule reference; predicates/data version-bound.
- Decision: Recreate 1.20.1 support and fence/wall/gate semantics; regenerate water/bubble/tag identities.
- 1.20.1 implications: Test negative-coordinate flooring, offset boundary, supporting-block cases, short-circuit query order and lower-factor precedence.
- Follow-up checks: Keep in RustChunk entity physics with registry-backed block predicates.
- RustChunk implementation mapping: Java 1.20.1 Entity support/factor sources and tag/position traces.
- Evidence required before adoption: undefined
- Completion state: COMPLETE — 95 logical lines through EOF; pinned SHA and 3597 byte size match; inspected, not modified; tests not executed.

### 2567. `crates/horizon-world/src/native_load_hooks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0c6597a8d848182e49676ec38d9d838578735995`
- Size: 1615 bytes
- Coverage: Complete through EOF: 53 logical lines; full file; no gaps.
- Purpose: Process-wide callbacks for native mod data-loading and loot behavior.
- Dependencies / cross-references: OnceLock; JSON loot tables, LootStack/NativeLootContext; function pointers for table/default-component/function/condition hooks.
- Implementation / logic: Installs one process-wide callback table once; later installs fail; all custom dispatch calls invoke stored function pointers directly.
- Findings: No isolation, replacement, unload lifecycle or privilege separation; arbitrary native code is a high-trust boundary, not Forge's Java mod/event system.
- RustChunk relevance: Not suitable for RustChunk Forge mod loading; keep outside compatibility path.
- Decision: unnecessary; reference; privileged native-extension model only
- 1.20.1 implications: Forge 47.x mods must run through Forge's Java runtime path; native hooks cannot substitute for Forge loading.
- Follow-up checks: Only evaluate native extensions under a separately specified signed, versioned, isolated permission/lifecycle model.
- RustChunk implementation mapping: Do not use for ordinary mods; use narrow internal Rust traits and official Forge integration.
- Evidence required before adoption: Threat model, signed-plugin design, isolation/lifecycle and proof Forge mods remain loaded by Forge.
- Completion state: COMPLETE — 53 logical lines through EOF; pinned SHA and 1615 byte size match; inspected, not modified; tests not executed.

### 2568. `crates/horizon-world/src/object_open_map.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `60f633a612580a5430f2e39751368945143145a3`
- Size: 13116 bytes
- Coverage: Complete through EOF: 412 logical lines; full file; no gaps.
- Purpose: Fastutil-style object open-addressing map with nullable keys, indexed entries, cursors and mutable traversal.
- Dependencies / cross-references: ObjectKey/CursorError; Rc/RefCell shared table; Vec slot arrays; Cell-backed Entry handles.
- Implementation / logic: Uses mixed hashes, linear probing, 0.75 fill threshold, rehash/shift deletion, null slot, deep-copy clone and cursors with wrapped-key tracking; for_each permits callback mutation.
- Findings: Entry/cursor indices interact with rehash; for_each captures old keys but reads live values. Wrapped external deletion is excluded; remove unwraps shift errors. Huge capacity arithmetic and mutation need parity tests.
- RustChunk relevance: Use only if exact fastutil traversal/mutation semantics are observable; otherwise prefer standard collections.
- Decision: adaptable; reference
- 1.20.1 implications: Pin exact 1.20.1 fastutil version and Java methods; this port's order is not automatically compatible.
- Follow-up checks: Compare collisions/nulls, growth/shrink, wrapping shifts, clone isolation, indexed entries and callback rehash/clear/remove.
- RustChunk implementation mapping: Isolate as a collection compatibility crate only for proven call sites.
- Evidence required before adoption: Exact fastutil version and property-based Java/Rust cursor/entry traces.
- Completion state: COMPLETE — 412 logical lines through EOF; pinned SHA and 13116 byte size match; inspected, not modified; tests not executed.

### 2569. `crates/horizon-world/src/object_open_set.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `49150f53d6fa14c8fd61b4bb65ce2d69a62f6875`
- Size: 7537 bytes
- Coverage: Complete through EOF: 243 logical lines; full file; no gaps.
- Purpose: Fastutil-style object open-addressing set with nullable member and live mutation-aware cursors.
- Dependencies / cross-references: ObjectKey hash/equality; Vec slot table; Cursor/CursorError; mixed hashing and shift deletion.
- Implementation / logic: Uses linear probing, reverse slot traversal, 0.75 load threshold, rehash, wrapped cursor elements and remove-current logic.
- Findings: Cursor reads live slots without modification counter; clear/resize may invalidate assumptions. Hash/equality must be pure; wrapped deletion and callback mutation need differential testing.
- RustChunk relevance: Potential deterministic collection compatibility helper; not a universal replacement.
- Decision: adaptable; reference
- 1.20.1 implications: Pin 1.20.1 fastutil version and verify ObjectOpenHashSet order/iterator behavior.
- Follow-up checks: Test collisions, null insertion/removal, wraparound shifts, resize during cursor traversal and repeated removals.
- RustChunk implementation mapping: Isolate behind Java-collection compatibility module; standard collections elsewhere.
- Evidence required before adoption: Exact fastutil source and generated Java-vs-Rust operation sequences.
- Completion state: COMPLETE — 243 logical lines through EOF; pinned SHA and 7537 byte size match; inspected, not modified; tests not executed.

### 2570. `crates/horizon-world/src/piston_movement.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bf94ff87b3b2920247d13c6799af6cea86b74603`
- Size: 1837 bytes
- Coverage: Complete through EOF: 50 logical lines; full file; no gaps.
- Purpose: Per-tick piston displacement accumulation and limiting.
- Dependencies / cross-references: PistonMovement; Vec3; game-time callback.
- Implementation / logic: Passes tiny motion without time query; resets per-axis accumulated deltas on tick change; clamps cumulative amount to ±0.51 and returns first nonzero axis change.
- Findings: Only first nonzero axis is returned per call, making caller order significant. Threshold, NaN behavior and time-query failure semantics need native tick parity.
- RustChunk relevance: Small self-contained piston rule; caller timing is essential.
- Decision: adaptable; reference
- 1.20.1 implications: Verify against 1.20.1 piston entity movement/tick schedule; don't assume source-era scheduling.
- Follow-up checks: Test threshold equality, multi-axis inputs, repeated calls, time errors, NaN/infinity and sign changes.
- RustChunk implementation mapping: Use as piston entity state called in Java-equivalent axis order.
- Evidence required before adoption: Java 1.20.1 PistonMovingBlockEntity traces and per-tick deltas.
- Completion state: COMPLETE — 50 logical lines through EOF; pinned SHA and 1837 byte size match; inspected, not modified; tests not executed.

### 2571. `crates/horizon-world/src/piston_push.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e6912e582a0cde9d637002c9c387ce9f55cc6c06`
- Size: 18226 bytes
- Coverage: Complete through EOF: 483 logical lines; full file; no gaps.
- Purpose: Moving piston push geometry, sticky honey selection, slime bounce and piston-base recovery.
- Dependencies / cross-references: Aabb/Vec3/Direction; MovingPistonState; PushReaction; piston_shape and host collision/entity queries.
- Implementation / logic: Computes swept slabs, overlap push amounts, sticky standing area, retraction-base correction and axis-specific slime bounce; caller performs Entity.move with per-tick limit.
- Findings: Entity storage/enumeration/collision resolution are deliberately external. Host must enforce IGNORE_ENTITY, collision queries/order and piston cap; geometry/progress assumptions need target traces.
- RustChunk relevance: Pure-geometry algorithms useful; not a complete piston engine.
- Decision: adaptable; reference
- 1.20.1 implications: Validate 1.20.1 piston/honey/slime and PushReaction; modern entity exclusions must not be copied blindly.
- Follow-up checks: Test six directions, negative/zero progress, grazing overlaps, retraction, sticky edges, entity filters and overlapping shapes.
- RustChunk implementation mapping: Port pure geometry only; keep entity selection, collision movement and tick mutation in world host.
- Evidence required before adoption: Java 1.20.1 piston sources, collision-box corpus, push-reaction table and tick tests.
- Completion state: COMPLETE — 483 logical lines through EOF; pinned SHA and 18226 byte size match; inspected, not modified; tests not executed.

### 2572. `crates/horizon-world/src/piston_shape.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d14f8f301ab7c9d6cf495dd4d9d77ad52b4885a3`
- Size: 7368 bytes
- Coverage: Complete through EOF: 196 logical lines; full file; no gaps.
- Purpose: Moving-piston collision-shape selection, head interpolation and registry-bound piston-state mapping.
- Dependencies / cross-references: BlockRegistry/BlockStateId; VoxelShape/ShapeOp; collision host callback and MovingPistonState.
- Implementation / logic: Binds piston base/head variants, chooses retracting-source collision-related state, handles noclip direction and combines stationary base with translated moving/head shape.
- Findings: Property names, direction order and state IDs are pinned-registry-specific. Progress thresholds and pre-noclip stationary-base query are observable.
- RustChunk relevance: Pure shape/control design adaptable only after registry rebuild.
- Decision: adaptable; version-specific; reference
- 1.20.1 implications: Regenerate piston block states from 1.20.1 registry; do not reuse modern IDs/order.
- Follow-up checks: Compare every facing/type/short/progress/source and noclip combination to Java 1.20.1 VoxelShape.
- RustChunk implementation mapping: Bind target states at load and delegate collision shape lookup to RustChunk world storage.
- Evidence required before adoption: 1.20.1 registry dump, Java piston collision methods and per-shape differential fixtures.
- Completion state: COMPLETE — 196 logical lines through EOF; pinned SHA and 7368 byte size match; inspected, not modified; tests not executed.

### 2573. `crates/horizon-world/src/player_equipment.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7e9c9b95f3414ce3882d878707d2c33ca575c0fb`
- Size: 2485 bytes
- Coverage: Complete through EOF: 83 logical lines; full file; no gaps.
- Purpose: Player hand/equipment slot access and equip callback dispatch.
- Dependencies / cross-references: PlayerInventory; EquipmentSlot; StackRef; InteractionHand; EquipWorld.
- Implementation / logic: Maps main/off hand; main hand aliases selected inventory slot; stores new stack before calling on_equip.
- Findings: Callback error leaves storage changed. Slot enum and reference identity must match target; later equipment slots/components cannot be assumed.
- RustChunk relevance: Useful inventory abstraction and callback ordering reference.
- Decision: adaptable; reference
- 1.20.1 implications: Regenerate 1.20.1 EquipmentSlot set/hand behavior; exclude later body/saddle slots.
- Follow-up checks: Test selected-slot changes, offhand/armor, StackRef aliases, replacement callback and failure-after-write.
- RustChunk implementation mapping: Use Rust-owned item-stack handles and inventory; call equip hooks after authoritative storage change.
- Evidence required before adoption: 1.20.1 PlayerInventory/EquipmentSlot sources, Forge equip events and alias traces.
- Completion state: COMPLETE — 83 logical lines through EOF; pinned SHA and 2485 byte size match; inspected, not modified; tests not executed.

### 2574. `crates/horizon-world/src/player_food.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c7869799a526fc677016d29f4edf7a3e254f6fa5`
- Size: 2232 bytes
- Coverage: Complete through EOF: 54 logical lines; full file; no gaps.
- Purpose: Player-specific hunger guards, exhaustion and direct-versus-virtual abilities.
- Dependencies / cross-references: FoodData; PlayerFoodWorld; abilities invulnerability; level side; virtual enough-food/may-fly callbacks.
- Implementation / logic: Adds exhaustion only if not directly invulnerable and not client-side; allows eating if invulnerable/always-edible/hungry; manoeuvre guard accepts enough-food or may-fly.
- Findings: Some paths intentionally read direct ability fields while others invoke virtual methods; conflating them changes modded-player behavior.
- RustChunk relevance: Compact player-food semantics reference.
- Decision: adaptable; reference
- 1.20.1 implications: Rebuild using 1.20.1 Player/Abilities/FoodData and Forge subclass overrides.
- Follow-up checks: Test invulnerability/client guards, always-edible, hunger threshold and virtual may-fly/enough-food behavior.
- RustChunk implementation mapping: Keep direct and virtual queries distinct in RustChunk player interfaces.
- Evidence required before adoption: 1.20.1 Player/FoodData sources and Forge subclass traces.
- Completion state: COMPLETE — 54 logical lines through EOF; pinned SHA and 2232 byte size match; inspected, not modified; tests not executed.

### 2575. `crates/horizon-world/src/player_health.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b64f545cb6863dd33b1ab3fddb22a90bf486366b`
- Size: 5950 bytes
- Coverage: Complete through EOF: 157 logical lines; full file; no gaps.
- Purpose: Player-specific health/absorption bridge to attributes and synchronized floats.
- Dependencies / cross-references: HealthAttributes/EntityAttributes; SyncedFloats/Observer; living_health::HealthWorld; player float IDs.
- Implementation / logic: Validates distinct data IDs and health attributes; provides direct health reads and shared living-health methods with virtual absorption access.
- Findings: IDs are supplied by pinned schema. HealthWorld::Data is `()`, so this adapter does not snapshot the data object by handle; verify provider cannot rebind during attribute callbacks. MAX_ABSORPTION availability is target-dependent.
- RustChunk relevance: Reusable adapter structure; target IDs/attributes/data access must be rebuilt.
- Decision: adaptable; version-specific; reference
- 1.20.1 implications: Rebuild synced IDs and health/absorption attributes from 1.20.1 Forge 47.x.
- Follow-up checks: Test missing/aliased IDs, missing attributes, observer failures, data rebinding and direct-vs-virtual access.
- RustChunk implementation mapping: Implement typed PlayerHealthWorld over generated 1.20.1 schema and attribute store.
- Evidence required before adoption: 1.20.1 Player/SynchedEntityData/attribute sources, exact IDs and callback-rebinding tests.
- Completion state: COMPLETE — 157 logical lines through EOF; pinned SHA and 5950 byte size match; inspected, not modified; tests not executed.