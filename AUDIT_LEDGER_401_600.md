# Starling → RustChunk Audit Ledger — Entries 401–600

- Source repository/ref: `starling-client/starling` / `starling`.
- Pinned source snapshot: `5432370d573642041585c69b40fb045b27133488`.
- Target repository/branch: `imamirhosein/RustChunk` / `starling-1.20.1-port-audit`.
- Continuation of `AUDIT_LEDGER.md` (entries 1–400); numbering and audit standard are unchanged.

### 401. `benchmarks/rust-noise-2026-10-05/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `bd9a46e65ae2f70f92e8d8ee82ec939d046ae2c7`.
- Size: 5913 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Noise/density.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Seeded Perlin/Simplex, derivatives, random streams and scalar/bulk density
- Findings: Selected maths only; no complete terrain/world proof
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/noise; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable; rebuild version data**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/noise; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 402. `benchmarks/rust-object-open-map-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4ba6d13f376b0e02a0a5b0a574673aedef8c2b5e`.
- Size: 34254 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Fastutil object map.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Open-address map put/get/remove/rehash and iteration behavior
- Findings: Collection parity is not game integration
- RustChunk relevance: Potential design/test input for future RustChunk runtime/collections; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference; prefer Rust ownership**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/collections; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 403. `benchmarks/rust-object-open-set-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `0e0fdebacb78494ee77656aa1be07ee591321931`.
- Size: 33664 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Fastutil object set.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Open-address set ordering and update behavior
- Findings: Not proof of integrated attribute queues
- RustChunk relevance: Potential design/test input for future RustChunk runtime/collections; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference; use Rust set if equivalent**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/collections; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 404. `benchmarks/rust-pillar-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e25ab67e1d38d20eb9c9f41171619ca6e8babb37`.
- Size: 23119 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Pillar feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Pillar codec, seeded placement and bounded search
- Findings: No complete chunk mutation/biome generation
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable feature algorithm**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 405. `benchmarks/rust-piston-movement-2026-10-07/movement-registry-regression.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `c15b8ae5e2f234f771ec34b8f1f8616365c92dc7`.
- Size: 24398 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Piston movement registry.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Registered movement factors/tags and support/jump/speed queries
- Findings: 26.3 table; no live piston event
- RustChunk relevance: Potential design/test input for future RustChunk registry/movement; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference; regenerate target data**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is registry/movement; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 406. `benchmarks/rust-piston-movement-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e8725102d1017371e4db2f70ddb65fc719da4cdc`.
- Size: 23484 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Entity piston displacement.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: limitPistonMovement numeric/time bounds
- Findings: No piston activation/block lifecycle/network
- RustChunk relevance: Potential design/test input for future RustChunk physics/piston; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable numeric regression**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/piston; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 407. `benchmarks/rust-piston-shape-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d13eb09422ccca480daf0a85a87c090e3fe4c92f`.
- Size: 19239 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Moving piston shape.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Moved-piston collision geometry and state selection
- Findings: No live block entity/piston lifecycle
- RustChunk relevance: Potential design/test input for future RustChunk physics/piston-shapes; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable shape tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/piston-shapes; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 408. `benchmarks/rust-placement-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `69df2a84662d349b5dc68dbcc4ee299571999a5d`.
- Size: 16946 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Placement modifiers.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Codec, random sampling and ordered modifier application
- Findings: No full configured-feature composition/world mutation
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/placement; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable codecs**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/placement; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 409. `benchmarks/rust-player-equipment-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `62f6fdef1356d0702f57f74b28008cc8dca85c8e`.
- Size: 31019 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Player equipment.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Equipment slots and shared ItemStack/inventory refs
- Findings: No full synchronized/use/save lifecycle
- RustChunk relevance: Potential design/test input for future RustChunk entity/equipment; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable ownership model**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/equipment; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 410. `benchmarks/rust-player-food-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `77975f641d8d017eaa5ff70aa0a2e8555cfd3c21`.
- Size: 27720 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Player food.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: FoodData hunger/exhaustion predicates and updates
- Findings: No full player tick/regeneration/damage/persistence
- RustChunk relevance: Potential design/test input for future RustChunk entity/player-food; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable state tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/player-food; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 411. `benchmarks/rust-player-health-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `215fcfb67336d26e1616d08c185e4a097b8c4848`.
- Size: 28510 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Player health.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Health/absorption with AttributeMap and SynchedEntityData
- Findings: No full damage source/wire/player lifecycle
- RustChunk relevance: Potential design/test input for future RustChunk entity/player-health; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable state composition**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/player-health; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 412. `benchmarks/rust-player-hurt-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `af64f6a2703029328ada574c74fcfb2d1bb4629c`.
- Size: 28538 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Player server hurt.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Player/ServerPlayer hurtServer and callback order
- Findings: Downstream damage/effect services controlled
- RustChunk relevance: Potential design/test input for future RustChunk entity/combat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable damage tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/combat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 413. `benchmarks/rust-player-inventory-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `62eee6f690d8b7fb4961117510734c73c1190a96`.
- Size: 30795 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Player inventory.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Slot selection/equipment/pick/matching/replacement
- Findings: No menus/recipes/pickup or all components
- RustChunk relevance: Potential design/test input for future RustChunk entity/inventory; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable inventory algorithms**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/inventory; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 414. `benchmarks/rust-player-travel-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1f6dffdf86b4e3193674cb208b77f12bcd704994`.
- Size: 25387 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Player travel.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Player.travel, flying speed and fluid predicates
- Findings: No live fluid/collision/vehicle/effect full tick
- RustChunk relevance: Potential design/test input for future RustChunk physics/player-travel; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable movement sequencing**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/player-travel; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 415. `benchmarks/rust-powder-snow-collision-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `267fa97128f12f7f338ef3c8a387461a7b3f8ced`.
- Size: 22593 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Powder snow collision.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Collision and canWalk decisions over states/tags/equipment
- Findings: No freezing/sinking/durability/live movement
- RustChunk relevance: Potential design/test input for future RustChunk physics/powder-snow; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable collision tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/powder-snow; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 416. `benchmarks/rust-registered-entity-fluid-2026-10-07/immersion-regression.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ae733fd446c27c94c5c2031a5e8859cd539d53bf`.
- Size: 26187 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Entity fluid immersion.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Fluid interaction/current state and loaded-chunk preflight
- Findings: No live fluid propagation or broad entity index
- RustChunk relevance: Potential design/test input for future RustChunk entity/fluid; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable query/order regression**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/fluid; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 417. `benchmarks/rust-registered-entity-fluid-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `0c226bfd355e1f178ab57767b3e3d2fe19aa968d`.
- Size: 28170 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Registered entity-fluid.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Immersion/current queries over resident chunk snapshots
- Findings: Providers fixture-controlled; no complete liquid movement
- RustChunk relevance: Potential design/test input for future RustChunk entity/fluid; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable fluid tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/fluid; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 418. `benchmarks/rust-registered-entity-fluid-2026-10-07/tick-regression.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `fd5ea966c718d612ce8cd32941d3a07b9c48c37c`.
- Size: 26401 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Entity fluid tick.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Update-fluid/current/splash transition ordering
- Findings: No particle/audio playback or live propagation
- RustChunk relevance: Potential design/test input for future RustChunk entity/fluid-tick; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable tick ordering**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/fluid-tick; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 419. `benchmarks/rust-registered-player-combat-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `0d747b10ca0a1fd60abc96412988531c11be2b92`.
- Size: 30584 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Registered player combat.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Damage types/tags and ServerPlayer/Player damage composition
- Findings: PvP/blocking/durability/effect consumers controlled
- RustChunk relevance: Potential design/test input for future RustChunk entity/combat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable combat harness**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/combat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 420. `benchmarks/rust-registry-fall-ray-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `904c3fb298d6a8d9714e4c806c8fcc2bd53a78a5`.
- Size: 26340 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Registry fall ray.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: BlockGetter.clip ray selection for fall/fluid/shape cases
- Findings: No live chunk provider or complete consumer
- RustChunk relevance: Potential design/test input for future RustChunk world/raytrace; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable ray tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/raytrace; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 421. `benchmarks/rust-registry-fluid-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `fc4866047d8ee604ea64b697508246fc10d96b4d`.
- Size: 27540 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Registry fluid queries.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Registered fluid heights, camera/current and state-family lookups
- Findings: 26.3 ids/data; no live spreading
- RustChunk relevance: Potential design/test input for future RustChunk registry/fluid; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference query design**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is registry/fluid; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 422. `benchmarks/rust-replace-blobs-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1a1520072802b5a97de2f768345c9df637126b79`.
- Size: 23829 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Replace-blobs feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: ReplaceBlobs codec, radius shells, random traversal and search
- Findings: No complete chunk mutation/biome generation
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable feature tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 423. `benchmarks/rust-replace-block-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e803da170570bd9cd00e3d19a5f70a2873629c21`.
- Size: 17070 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Replace-block feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Single-block replace rules, target checks and random callbacks
- Findings: No region effects/structure processors/decorations
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable placement logic**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 424. `benchmarks/rust-resident-collision-scan-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `99c2af3eb1eaa4619d24e1863a9cd0f076469018`.
- Size: 24747 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Resident collision scan.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: BlockCollisions over registered states/ProtoChunk snapshots
- Findings: Controlled proxy; no live chunk/entity broad-phase
- RustChunk relevance: Potential design/test input for future RustChunk physics/collision; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable collision harness**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/collision; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 425. `benchmarks/rust-resource-number-scanner-2026-10-06/model-baking.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `5e774bb2c902a0a15bead7d3ced381ee3faed441`.
- Size: 3810 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: FaceBakery fixture.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Model-face geometry, positions, UVs and bake result
- Findings: CPU geometry only; no atlas/shader/GPU pixels
- RustChunk relevance: Potential design/test input for future RustChunk render/model-baking; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable geometry oracle**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-baking; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 426. `benchmarks/rust-resource-number-scanner-2026-10-06/model-decoding.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `33b99b897c25b2866f8877e8ce5c10c81ea4c1e6`.
- Size: 3443 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Cuboid model fixture.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Cuboid JSON decoded to normalized model data
- Findings: No parent resolution/atlas/GPU rendering
- RustChunk relevance: Potential design/test input for future RustChunk render/model-data; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable parser fixture**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-data; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 427. `benchmarks/rust-resource-number-scanner-2026-10-06/model-resolution.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `b479e27f76da517cba21922fd7224ecadb6517e0`.
- Size: 3747 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Model resolution fixture.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Parent model and texture reference resolution
- Findings: No final quad bake/light/render pixels
- RustChunk relevance: Potential design/test input for future RustChunk render/model-resolution; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable resolver fixture**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-resolution; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 428. `benchmarks/rust-resource-number-scanner-2026-10-06/sprite-animation.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `331ab541b3b80af421a3120b483f22c8803dcb93`.
- Size: 3751 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Sprite animation fixture.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Animation metadata/frame/duration sampling
- Findings: No image decode/atlas upload/GPU
- RustChunk relevance: Potential design/test input for future RustChunk render/animation; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable state tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/animation; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 429. `benchmarks/rust-restitution-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `26667829bd6cf4d468d96f7aa6c0422d895489b3`.
- Size: 23194 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Entity movement restitution.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Entity.restituteMovementAfterCollisions with controlled callbacks
- Findings: No live grounding/tags/effects/world lifecycle
- RustChunk relevance: Potential design/test input for future RustChunk physics/movement; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable numeric tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/movement; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 430. `benchmarks/rust-retained-upgrade-status-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `24a3c7ef570e4adb7e29adedb66dbf2cf35876cb`.
- Size: 23133 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Retained below-zero status.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Chunk retrogen metadata/stages/heights/bedrock mask
- Findings: No save-dirty scheduling/biome migration/full retrogen
- RustChunk relevance: Potential design/test input for future RustChunk world/chunk-retrogen; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable metadata model**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/chunk-retrogen; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 431. `benchmarks/rust-retrogen-biome-fill-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ba4a85c93f3341d420db2fc7b9c52c676862f97a`.
- Size: 22857 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Retrogen biome fill.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: ChunkAccess/LevelChunkSection biome filling and retained resolver
- Findings: No target palette/full biome generation
- RustChunk relevance: Potential design/test input for future RustChunk world/chunk-biomes; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable section updates**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/chunk-biomes; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 432. `benchmarks/rust-retrogen-biome-generation-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `7d2a8287edd67cc7a3a89554bccf196b1cd91527`.
- Size: 23748 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Retrogen biome generation.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: NoiseBasedChunkGenerator biome creation/multi-noise/End
- Findings: Not full generation; holder placeholders limit fidelity
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/biomes; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable generator pattern**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/biomes; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 433. `benchmarks/rust-retrogen-chunk-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `7cecf83e7c78247af9851728afdccc694d13d448`.
- Size: 23053 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Retrogen chunk state.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: ProtoChunk retrogen status, mask and height decisions
- Findings: No persistence/scheduler/full lifecycle
- RustChunk relevance: Potential design/test input for future RustChunk world/chunk-retrogen; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable chunk state**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/chunk-retrogen; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 434. `benchmarks/rust-retrogen-data-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `9ead91ad381fbd366bc54d53381e1220fc424961`.
- Size: 21815 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: BelowZeroRetrogen codec.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Codec, masks and hole queries
- Findings: No automatic schedule or end-to-end upgrade
- RustChunk relevance: Potential design/test input for future RustChunk world/chunk-retrogen; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable codec tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/chunk-retrogen; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 435. `benchmarks/rust-rule-test-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `686e1c15a65de6b5ed004cb7729437100d9d5c5e`.
- Size: 17022 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: RuleTest codecs.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: RuleTest decode/tags/probability
- Findings: No processor execution or structure mutation
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/structure-rules; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable codec tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/structure-rules; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 436. `benchmarks/rust-scaffolding-collision-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `dc224cf8bec456d1cb5ddd5bd2804212590d171c`.
- Size: 22305 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Scaffolding collision.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Collision shape matrix for scaffolding states
- Findings: No placement/support/fall/live movement
- RustChunk relevance: Potential design/test input for future RustChunk physics/block-shapes; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable shape matrix**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/block-shapes; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 437. `benchmarks/rust-scattered-ore-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e5abaaeb2f37d86bbd959b553135ea3af9752232`.
- Size: 17098 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Scattered ore feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Feature codec/placement and air checks
- Findings: No complete veins/chunk decoration
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable feature test**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 438. `benchmarks/rust-server-player-combat-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `73bea6731e930da2ecd6588eb82f68c48fc2b705`.
- Size: 29732 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: ServerPlayer combat.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: ServerPlayer damage with health and FoodData
- Findings: Effects/durability/PvP/network/death controlled
- RustChunk relevance: Potential design/test input for future RustChunk entity/combat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable damage composition**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/combat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 439. `benchmarks/rust-server-player-combat-immunity-2026-10-07/inner-damage-regression.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e33d407e5a8d39389dcb08444d40b5bc8cd4cddd`.
- Size: 28754 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: ServerPlayer inner damage.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: actualHurt and absorption/stat order
- Findings: Outer gates and attribution/world integration separate
- RustChunk relevance: Potential design/test input for future RustChunk entity/combat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable regression**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/combat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 440. `benchmarks/rust-server-player-combat-immunity-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `da544da9b6c24e36ff43c516f1d4fd9ac6ccb339`.
- Size: 29866 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: ServerPlayer immunity.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Outer hurt and invulnerability decisions
- Findings: No complete damage/effects/packets/player tick
- RustChunk relevance: Potential design/test input for future RustChunk entity/combat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable boundary tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/combat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 441. `benchmarks/rust-server-player-damage-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `776f2961071b14d986b443abaa9e3ecdade97314`.
- Size: 28930 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: ServerPlayer damage.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Inner damage, armor/magic, health and food
- Findings: Criteria/effects/durability/death controlled
- RustChunk relevance: Potential design/test input for future RustChunk entity/combat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable damage flow**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/combat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 442. `benchmarks/rust-shared-item-use-2026-10-07/item-use-lifecycle.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1f578582e5015cceeab1eed25627b56438049cf8`.
- Size: 31256 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Shared item-use lifecycle.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Item-use tick/update/complete/release/stop
- Findings: Start/network/concrete consumables/full player lifecycle pending
- RustChunk relevance: Potential design/test input for future RustChunk entity/item-use; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable lifecycle tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/item-use; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 443. `benchmarks/rust-shared-item-use-2026-10-07/item-use-start.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `682e04217881a3278020743608b29dab935ca2dc`.
- Size: 31059 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Shared item-use start.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: startUsingItem and equipment sync/packet construction
- Findings: Transport and synced flag consumers absent
- RustChunk relevance: Potential design/test input for future RustChunk network/item-use; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable dispatch tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is network/item-use; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 444. `benchmarks/rust-shared-item-use-2026-10-07/item-use.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1ee4f76a7ffe559884dae5575c744b526deff8c2`.
- Size: 30969 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Shared item blocking.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Blocking eligibility and use timer semantics
- Findings: Full lifecycle/durability/combat integration absent
- RustChunk relevance: Potential design/test input for future RustChunk entity/item-use; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable blocking tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/item-use; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 445. `benchmarks/rust-simple-block-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `416d5a0577210a2da4d32459575d76a54539a42a`.
- Size: 22550 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Simple block feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: SimpleBlockFeature codec and placement
- Findings: Chunk writes/neighbors/full generation not tested
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable feature implementation**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 446. `benchmarks/rust-simple-container-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `c614edde5113872324df4517effe6d3ef731f850`.
- Size: 30637 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: SimpleContainer inventory.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: SimpleContainer stack storage/shared refs
- Findings: No persistence/menu transactions/full components
- RustChunk relevance: Potential design/test input for future RustChunk entity/inventory; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable container pattern**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/inventory; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 447. `benchmarks/rust-snow-freeze-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `3219ea2848a254fbfb7bb3739dc6dbe7bec68421`.
- Size: 24481 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Snow/freeze feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: SnowAndFreezeFeature codec and placement using heights/biomes
- Findings: Lighting/broad mutation/surface generation absent
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable terrain feature**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 448. `benchmarks/rust-speleothem-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4df39deb38baa323259c4b618c1f5f14434dbb45`.
- Size: 24942 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Speleothem feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Config and cave placement rules
- Findings: No neighbor updates/falling/fluid/full caves
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable feature geometry**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 449. `benchmarks/rust-spike-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4311e007bd9a766460db8f00f45fdf09ef9e95ad`.
- Size: 23417 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Spike feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: SpikeFeature codec and seeded placement
- Findings: No complete End chunk/world lifecycle
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable deterministic feature**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 450. `benchmarks/rust-spring-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `df273de7dcfb813d26b6c27e68f80828be5980ac`.
- Size: 22968 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Spring feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: SpringFeature codec, cavity checks and fluid tick requests
- Findings: Tick execution/spreading/full worldgen absent
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable feature tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 451. `benchmarks/rust-sprite-animation-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ee6ec6ac432e31593144e77c2a5b31a80c350210`.
- Size: 3751 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Sprite animation parity.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: AnimationMetadata/SpriteContents frame decisions
- Findings: No image decode/atlas/GPU frame interpolation
- RustChunk relevance: Potential design/test input for future RustChunk render/animation; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable animation logic**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/animation; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 452. `benchmarks/rust-sprite-animation-2026-10-06/startup-smoke.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `064b842ef079809accd8713b58d8706af2371db7`.
- Size: 3489 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Sprite animation smoke.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Native cube/animation startup smoke
- Findings: No player gameplay/device hardware/FPS
- RustChunk relevance: Potential design/test input for future RustChunk render/smoke; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference visual smoke only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/smoke; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 453. `benchmarks/rust-state-provider-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `11dc44ae238f30dc3b9be75baad0f4c7341bcf30`.
- Size: 25707 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: BlockStateProvider codecs.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Provider decode, rotations, random/noise/rule selectors
- Findings: No registry reload/placement/worldgen integration
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/state-provider; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable codec architecture**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/state-provider; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 454. `benchmarks/rust-status-effect-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `38afe568b309cef9527cc801d4092335960e46a7`.
- Size: 32710 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: MobEffectInstance.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Construct/copy/duration/equality/hidden effect/blending
- Findings: Effect tick/registry/UI lifecycle separate
- RustChunk relevance: Potential design/test input for future RustChunk entity/effects; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable value semantics**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/effects; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 455. `benchmarks/rust-status-tick-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `222d8734317bf33ff5b95adb9ff86e178d1b0633`.
- Size: 32958 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Status effect ticking.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Effect tick callbacks/hidden-chain updates/mutation order
- Findings: Real dispatch/registry/full gameplay outside harness
- RustChunk relevance: Potential design/test input for future RustChunk entity/effects; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable tick algorithm**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/effects; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 456. `benchmarks/rust-stored-consume-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `64e8337521eeab30f91296559f67d0225ca4b4b6`.
- Size: 33659 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Stored consumed effects.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Consumed effect application and map interactions
- Findings: No aggregate Rust hash; registration/notification/sound controlled
- RustChunk relevance: Potential design/test input for future RustChunk entity/effects; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable regression; evidence gap**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/effects; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 457. `benchmarks/rust-stored-effect-attributes-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e4d9bd2a11893d97e929136bb85971d2e8686bed`.
- Size: 34944 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Stored effect attributes.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Effect attribute storage/instance updates
- Findings: No top-level hash and reference outputs differ
- RustChunk relevance: Potential design/test input for future RustChunk entity/attributes; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only pending evidence**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/attributes; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 458. `benchmarks/rust-stored-effect-notifications-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `85a2f83312aac33d2037b5b2bed17b050e688bf3`.
- Size: 35561 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Stored effect notifications.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: onEffect/refreshDirtyAttributes paths
- Findings: No top-level hash; reference output hashes differ
- RustChunk relevance: Potential design/test input for future RustChunk entity/attributes; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only pending evidence**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/attributes; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 459. `benchmarks/rust-structure-density-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `b1b4e9bfd57790a2a8f52d44d3bb301987df94e5`.
- Size: 11495 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Structure density.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Beardifier scalar/bulk density with supplied structures
- Findings: No structure discovery/placement/terrain integration
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/structure-density; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable numeric provider**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/structure-density; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 460. `benchmarks/rust-suffocation-2026-10-06/contextual-regression.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `53e27ae08a9827c20e7cbd0b59eef3afb32e1f12`.
- Size: 23955 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Suffocation contextual.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Per-block suffocation dispatch/dynamic shape caching
- Findings: No live entity/minecart/world lifecycle
- RustChunk relevance: Potential design/test input for future RustChunk physics/suffocation; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable shape tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/suffocation; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 461. `benchmarks/rust-suffocation-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `a5aa8ad8043d15391fcd8e8b71536e5cfb0726ac`.
- Size: 24042 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Suffocation parity.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: isSuffocating/large collision/piston/shulker cases
- Findings: No live chunk provider/entity movement
- RustChunk relevance: Potential design/test input for future RustChunk physics/suffocation; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable query tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/suffocation; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 462. `benchmarks/rust-swimming-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `41f9115b0136e0e24ab6b1e6b9647c9bcc90643a`.
- Size: 26296 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Swimming predicates.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Entity/Player swimming/underwater/shallow-water decisions
- Findings: No full water physics/player tick
- RustChunk relevance: Potential design/test input for future RustChunk entity/swimming; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable predicates**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/swimming; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 463. `benchmarks/rust-synced-bytes-2026-10-07/float-regression.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `0245009a7aa86516637be38664a62ae57c8a26c5`.
- Size: 31195 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: SynchedEntityData float regression.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Float fields, dirty/reentrant writes and observers
- Findings: Other types/packet codecs/entity schema absent
- RustChunk relevance: Potential design/test input for future RustChunk network/entity-data; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable sync design**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is network/entity-data; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 464. `benchmarks/rust-synced-bytes-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `0a251b57b1a9a589e8efd0ea61c5434000789004`.
- Size: 31201 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: SynchedEntityData byte.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Byte values and use flags
- Findings: Other types/schema/wire codecs pending
- RustChunk relevance: Potential design/test input for future RustChunk network/entity-data; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable typed tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is network/entity-data; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 465. `benchmarks/rust-synced-floats-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `c2e22e623a1055e320a6913bd52daee378a61acd`.
- Size: 27977 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: SynchedEntityData float.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Focused float synchronized values
- Findings: Only float serializer; no protocol/wire schema
- RustChunk relevance: Potential design/test input for future RustChunk network/entity-data; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable numeric serialization**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is network/entity-data; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 466. `benchmarks/rust-teleport-consume-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d102e8da16569ef8445e77bb77425a761c0a76c8`.
- Size: 33062 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Teleport consume effect.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: TeleportRandomlyConsumeEffect coordinate/RNG selection
- Findings: Actual teleport/passenger/world events controlled
- RustChunk relevance: Potential design/test input for future RustChunk entity/effects; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable random algorithm**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/effects; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 467. `benchmarks/rust-terrain-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `8e06e6bb070be573bff0f2a323266fe6bfd247c2`.
- Size: 12574 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Terrain noise stage.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Noise-stage chunk fill, seed/random/noise/aquifer input
- Findings: Material rules/carvers/structures/full blend absent
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/terrain; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable terrain architecture**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/terrain; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 468. `benchmarks/rust-ticks-2026-10-05/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `c30f1398176ee8144b866309c6de456e7958a5f1`.
- Size: 5800 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Block/fluid tick queue.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Scheduled tick order/dedup/save/eligibility
- Findings: No block/fluid tick execution/full scheduler
- RustChunk relevance: Potential design/test input for future RustChunk world/tick-scheduler; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable scheduler tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/tick-scheduler; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 469. `benchmarks/rust-underwater-magma-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `9492ca7a22a795b5fb2fd770e819e2ca98b1f52a`.
- Size: 23439 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Underwater magma feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: UnderwaterMagmaFeature codec/placement
- Findings: Chunk updates/postprocessing/full generation absent
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable feature placement**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 470. `benchmarks/rust-use-flag-reaction-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `2fa60abee487992cbdbfc8f11828ad9e1838ac80`.
- Size: 31449 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Use-flag observer.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Synced use-flag reactions with inventory/equipment/stacks
- Findings: Other sync keys/packet delivery/full item use absent
- RustChunk relevance: Potential design/test input for future RustChunk entity/item-use; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable event ordering**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is entity/item-use; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 471. `benchmarks/rust-use-remainder-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `aa56f8fb7ea9e925ecd115698fc7387906f562d9`.
- Size: 31649 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: UseRemainder.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Remainder conversion/templates/shared stack identity
- Findings: No full component schema/insertion/drop/consumables
- RustChunk relevance: Potential design/test input for future RustChunk item/components; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable component contract**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is item/components; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 472. `benchmarks/rust-value-provider-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d6c3ab7526db0c83b00c641000b8d80d8d52566f`.
- Size: 16087 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Value providers.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Float/int provider decode/sample
- Findings: Gaussian/full height provider/terrain incomplete
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/providers; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable provider algorithms**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/providers; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 473. `benchmarks/rust-view-direction-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `3716bc833fcb59d54685d282a151ae88e8380420`.
- Size: 29017 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: View-direction math.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Vec3 view-direction/acos/block direction edges
- Findings: No platform numeric/action integration proof
- RustChunk relevance: Potential design/test input for future RustChunk math/view-direction; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable numeric fixtures**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is math/view-direction; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 474. `benchmarks/rust-vines-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ab07429eafbcdb8ff4f8baee3698170a9aad5b15`.
- Size: 22956 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Vines feature.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: VinesFeature support/attachment/random decisions
- Findings: No growth tick/neighbor/live chunk
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/features; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable feature logic**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/features; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 475. `benchmarks/rust-water-splash-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `a7960805c37dfc486aa85ac6a8d37dd3c6cef961`.
- Size: 26618 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Water splash effect.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: doWaterSplashEffect particle/sound/random request order
- Findings: No audio mixer/particle renderer/live movement
- RustChunk relevance: Potential design/test input for future RustChunk render/effects; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable event request model**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/effects; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 476. `benchmarks/rust-world-2026-10-05/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `6e441300a9b83eb24922cc3090a43e9c74b41c16`.
- Size: 5789 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: World overview artifact.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: World reference notes without differential corpus
- Findings: No output hash/reference pair; not implementation evidence
- RustChunk relevance: Potential design/test input for future RustChunk world/architecture; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/architecture; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 477. `benchmarks/rust-world-border-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `39228449ff8c40cdf5e5334579b208a7a5f69e40`.
- Size: 23280 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: WorldBorder.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Stationary/moving extents, update/tick/collision arithmetic
- Findings: No persistence/listener/damage/network integration
- RustChunk relevance: Potential design/test input for future RustChunk world/border; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable geometry/state tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/border; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 478. `benchmarks/rust-world-collision-check-2026-10-07/movement-regression.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ab8a4a92c53e244db039aac56f944eb6de72ea06`.
- Size: 25740 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: World collision movement.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Entity.collide over registered shapes/ProtoChunk
- Findings: Providers/entity lists/border controlled; no live broad phase
- RustChunk relevance: Potential design/test input for future RustChunk physics/collision; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable physics tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/collision; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 479. `benchmarks/rust-world-collision-check-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `27e100115621010a8b0333f3c842f4708b440746`.
- Size: 25547 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: World collision noCollision.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: CollisionGetter.noCollision across registered states/sections
- Findings: Live entity broad phase/minecart/world lifecycle incomplete
- RustChunk relevance: Potential design/test input for future RustChunk physics/collision; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable collision composition**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/collision; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 480. `benchmarks/rust-world-edge-backoff-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `3dc8ef1a05b9d03a0ad47caf902aeb248c5141dc`.
- Size: 25693 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: World edge backoff.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Player.maybeBackOffFromEdge state/chunk/movement contexts
- Findings: Isolated query, not integrated Player physics/world
- RustChunk relevance: Potential design/test input for future RustChunk physics/player-motion; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable edge regression**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/player-motion; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 481. `benchmarks/rust-world-movement-2026-10-07/geometry-regression.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `b8f01cb16d855c02bd24b376a1461de41fed64f9`.
- Size: 9597 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: World movement geometry.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Vec3/AABB/voxel math and step-up over supplied shapes
- Findings: No live World shape selection/integrated movement
- RustChunk relevance: Potential design/test input for future RustChunk physics/geometry; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable geometry suite**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/geometry; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 482. `benchmarks/rust-world-movement-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `8d24c8618999422e952838d5e953917356a923bd`.
- Size: 24898 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: World movement parity.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Entity.collide over registered ProtoChunk collision shapes
- Findings: No live provider/full movement tick/input lifecycle
- RustChunk relevance: Potential design/test input for future RustChunk physics/collision; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable movement kernel**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is physics/collision; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 483. `benchmarks/rust-world-ray-2026-10-07/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ab9e89facf3df875b7f0b4fbb44d7fcfde3e6bf1`.
- Size: 24725 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: World ray query.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: BlockGetter.clip block/fluid modes and fall-reset
- Findings: No live world/chunk provider/complete consumers
- RustChunk relevance: Potential design/test input for future RustChunk world/raytrace; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable ray traversal**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is world/raytrace; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 484. `benchmarks/rust-worldgen-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `be295c20a7b0b357ba66d3d5ebaf462a7249f75c`.
- Size: 68538 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Worldgen source manifest.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Noise/Density/NoiseRouter/biome/climate/TagLoader references
- Findings: Selected codec/query manifests only; no full terrain/chunk/world proof
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/manifest; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Architecture reference; rebuild data**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/manifest; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 485. `benchmarks/rust-worldgen-random-2026-10-06/scoped-parity.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `f0c80d9d4e17c8f1de17d4ba71a246ac597b7d8f`.
- Size: 16567 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Worldgen random generators.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Legacy/Xoroshiro seeds and draw sequences
- Findings: Gaussian/concurrent/full decoration outside scope
- RustChunk relevance: Potential design/test input for future RustChunk worldgen/random; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable deterministic RNG tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is worldgen/random; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 486. `benchmarks/server-2026-10-17/README.md`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4a3cd3dcc8a2f3c2c28e10bb7eba5523f7d27669`.
- Size: 10786 bytes.
- Coverage: Complete file read.
- Purpose: Server benchmark methodology 17 Oct.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Dedicated Horizon/vanilla 26.3, 1–100 creative bots, ThinkPad i7-7500U, OpenJDK25, remote Mac
- Findings: Bot host overloaded, unequal workloads and vanilla watchdog; not Android client
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-method; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference method only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-method; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 487. `benchmarks/server-2026-10-17/earlier-002f14ce/explore-equal-rate.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `404f8d032228739331c7f2335d5ee3c9facd0c4c`.
- Size: 8064 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Equal-rate explore server result.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Horizon/vanilla equal-rate explore 1/10/25 bots
- Findings: Server only, one pass; not A22/client FPS
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference server data**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 488. `benchmarks/server-2026-10-17/earlier-002f14ce/load.txt`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `8532528097d855071aa716ef94144f7b7c19de9a`.
- Size: 24048 bytes.
- Coverage: Complete file read.
- Purpose: Earlier server load trace.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Host load/CPU/RAM during server run
- Findings: Remote bot host high load/ThinkPad variability confound
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-context; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Context only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-context; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 489. `benchmarks/server-2026-10-17/earlier-002f14ce/walk-explore.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `39dcd268cb202f16411d93f75fd375202c7d9810`.
- Size: 16262 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–696.
- Purpose: Earlier walk/explore runs.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Horizon/vanilla walk/explore 1–100 bots
- Findings: Vanilla watchdog/broken pipe at high load; unequal chunks
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference throughput only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 490. `benchmarks/server-2026-10-17/explore-equal-rate.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1356529e338683733c5fa0fe08dbf6849591be04`.
- Size: 8072 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Equal-rate explore data.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Six Horizon/vanilla explore rows 1/10/25 bots
- Findings: Server tick/RSS/CPU only
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference comparison**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 491. `benchmarks/server-2026-10-17/load.txt`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `33d8b191f95059705569cf50524cdd8e37ede838`.
- Size: 24259 bytes.
- Coverage: Complete file read.
- Purpose: Server load trace 17 Oct.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: ThinkPad/Mac mini load averages
- Findings: Mac mini bot host often saturated
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-context; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Benchmark context**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-context; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 492. `benchmarks/server-2026-10-17/walk-explore.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `a9479f455564d10b8a5c5cba6a164955ca37cdbc`.
- Size: 16259 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–696.
- Purpose: Server walk/explore results.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: 20 Horizon/vanilla runs walk/explore 1–100 bots
- Findings: Vanilla watchdog high counts; remote bots
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference server result**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 493. `benchmarks/server-equal-mixed-2026-10-18/README.md`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `695f08c81617363be63655c6ccefeef03ce4bd63`.
- Size: 8969 bytes.
- Coverage: Complete file read.
- Purpose: Equal-mixed server method.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Mixed walk/explore/idle, warmup300s, measure300s, work matching
- Findings: At25 new-chunk rate still differs; vanilla50 watchdog; remote bot host busy
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-method; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only; fairness caveat**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-method; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 494. `benchmarks/server-equal-mixed-2026-10-18/load.txt`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `2992b4a263374a49222eccbad3b1fc9dd13f3a77`.
- Size: 30826 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–703.
- Purpose: Equal-mixed host load.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: ThinkPad/Mac mini conditions
- Findings: Bot host CPU 70–93%; server host spikes
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-context; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Context only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-context; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 495. `benchmarks/server-equal-mixed-2026-10-18/mixed-equal.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `c494f3b24be7566726a4e59ae85d57b611ac7417`.
- Size: 10097 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–422.
- Purpose: Equal-mixed results.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Eight Horizon/vanilla 1/10/25/50 runs MSPT/RSS/chunks
- Findings: Horizon 1.77/4.74/11.72/23.82ms; vanilla 11.34/34.3/50.45 then watchdog; 25 work differs
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference server data**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 496. `benchmarks/server-equal-work-2026-10-17/README.md`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `737b0585821d5e198c68e33a7f624c8489aca81d`.
- Size: 10644 bytes.
- Coverage: Complete file read.
- Purpose: Equal-work server method.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Movement checks/chunk pacing/acks and walk/explore fairness
- Findings: Paper often faster in walking; vanilla watchdog at100; server-only
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-method; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference method**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-method; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 497. `benchmarks/server-equal-work-2026-10-17/explore-equal-rate.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `bd097e43a5955b9a66422f259b65ebad3420309e`.
- Size: 6217 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Equal-work explore data.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Horizon/vanilla 1/10/25 equal-rate explore rows
- Findings: Both 20TPS; ThinkPad Linux/OpenJDK25 only
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference measurements**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 498. `benchmarks/server-equal-work-2026-10-17/horizon.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d0b162c442af379c9fb277b54573e26343efae2a`.
- Size: 9670 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–416.
- Purpose: Horizon server run data.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Horizon walk/explore 1–100 after chunk-cache
- Findings: At100 walking ~114.5ms MSPT/8.7TPS; no client data
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference server benchmark**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 499. `benchmarks/server-equal-work-2026-10-17/run-1-before-chunk-cache.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d79b3f179c2f4a490a72377549aa9d5b48943a26`.
- Size: 16114 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–694.
- Purpose: Pre-chunk-cache runs.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Earlier server workloads before cache fixes
- Findings: High-load outliers/watchdog; server revision specific
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Historical reference**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 500. `benchmarks/server-equal-work-2026-10-17/run-2-chunk-cache.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4c182395c5e067b1018ae8cbd2110fe2561fc9d1`.
- Size: 9614 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–414.
- Purpose: Post-cache Horizon runs.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Horizon server-only after chunk cache
- Findings: No matched vanilla baseline
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Historical reference**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 501. `benchmarks/server-equal-work-2026-10-17/vanilla.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `014554ceeaf1eda6d5c6888fd63a73e7fbb31984`.
- Size: 7525 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Vanilla server runs.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Vanilla 1–100 bot walk/explore results
- Findings: Watchdog/broken-pipe at high counts
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference baseline**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 502. `benchmarks/server-four-way-2026-10-19/README.md`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1c8b7b07e6e1dd2fcf8cef09aed1a2aca8491c1a`.
- Size: 11172 bytes.
- Coverage: Complete file read.
- Purpose: Four-way server method.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Horizon/vanilla/Paper/Fabric+Lithium mixed tests/fairness
- Findings: Paper fastest average MSPT in several loads; remote host involved
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-method; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-method; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 503. `benchmarks/server-four-way-2026-10-19/load.txt`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `786b933b6059ed230e2f7e433cd42d618f641da2`.
- Size: 30295 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–871.
- Purpose: Four-way host trace.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: ThinkPad/Mac mini load trace
- Findings: Remote bots 80–90% busy; server host spikes
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-context; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Context only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-context; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 504. `benchmarks/server-four-way-2026-10-19/report-25-50.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `dacda83973971c90e5e70eb22bcf2a102e449216`.
- Size: 10290 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–438.
- Purpose: Four-way 25/50 results.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Horizon/vanilla/Paper/Fabric eight mixed runs
- Findings: Paper25 28.3ms vs Horizon30.92; at50 Horizon47.83/Paper46.86/Fabric58.3; vanilla fails
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference dedicated-server metrics**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 505. `benchmarks/server-four-way-2026-10-19/report-paper-10.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `3408fd419c95204ab86367b97df9e6d24a4d76cc`.
- Size: 2158 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Paper 10 bot result.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Single Paper 10 bot run
- Findings: One workload slice only
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 506. `benchmarks/server-four-way-2026-10-19/runs.jsonl`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `04a1a6fb8110cddf9fc8a2f4392a126040b05471`.
- Size: 9976 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Four-way raw runs.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: 16 per-run JSONL records
- Findings: Server workload data, not client FPS
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference dataset**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 507. `benchmarks/server-mixed-2026-10-18/README.md`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `a55f5999da316ee75bd2593d4853c4e49328e206`.
- Size: 7595 bytes.
- Coverage: Complete file read.
- Purpose: Mixed server method.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Mixed Horizon/vanilla server run method
- Findings: Explore throughput differs; compare ticks cautiously
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-method; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference method**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-method; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 508. `benchmarks/server-mixed-2026-10-18/load.txt`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `3468f9f86539d74c630b043f756e6f19d79fdb21`.
- Size: 12468 bytes.
- Coverage: Complete file read.
- Purpose: Mixed server load.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Host load during remote bot runs
- Findings: Mac bot host heavily loaded
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark-context; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Context only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark-context; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 509. `benchmarks/server-mixed-2026-10-18/mixed.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `36b21c327f1ae298d547a14280c97b9c19d57174`.
- Size: 7684 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Mixed server results.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Horizon/vanilla 1/10/25/50 mixed bot metrics
- Findings: At50 Horizon MSPT higher than vanilla in run; throughput unequal
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference server data**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 510. `benchmarks/server-vs-pumpkin-2026-10-31.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `fd9a893f0f129d1223e1a53e13311fb257e191d7`.
- Size: 29095 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1498.
- Purpose: Horizon vs Pumpkin.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: 24 runs 0/20/60/100 bots, 3 repeats, EPYC7662 8vCPU pinned, OpenJDK25
- Findings: Pumpkin faster at20, mixed60, Horizon stronger100; anti-cheat/ground disabled
- RustChunk relevance: Potential design/test input for future RustChunk server/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference server comparison**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 511. `benchmarks/shared-json-buffer-lifetime-20261017.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d618906ed595b6fa4f6aa1982536192302d39423`.
- Size: 7481 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Shared JSON buffer lifetime.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: JSON retain/release timings and RSS
- Findings: Near-identical medians; negligible gain
- RustChunk relevance: Potential design/test input for future RustChunk runtime/perf-json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/perf-json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 512. `benchmarks/shared-json-decoder-phases-20261017.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `8c58801242c2f8af28ad0496b64b44ba6c365a16`.
- Size: 3437 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Shared JSON decoder phases.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Validation vs typed decode timing/RSS
- Findings: Typed sample faster but higher peak RSS; loader-only
- RustChunk relevance: Potential design/test input for future RustChunk runtime/perf-json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference microbenchmark**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/perf-json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 513. `benchmarks/shared-json-five-way-20261017.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `61dcb7c3dc2ceb10209f20036a48e7df70ff0a9d`.
- Size: 9582 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–454.
- Purpose: Shared JSON five-way.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Baseline/shared/deferred JSON ownership variants
- Findings: Loader-only and variant confounds
- RustChunk relevance: Potential design/test input for future RustChunk runtime/perf-json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable measurement design**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/perf-json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 514. `benchmarks/shared-json-five-way-final-20261017.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ce92b7b61032f9dbcb26b7391e4a0c27b6793c6a`.
- Size: 11905 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–583.
- Purpose: Shared JSON final.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Repeated baseline/current/shared/deferred variants
- Findings: Lower RSS vs baseline, but host variance and loader-only scope
- RustChunk relevance: Potential design/test input for future RustChunk runtime/perf-json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference optimization evidence**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/perf-json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 515. `benchmarks/shared-json-historical-20261017.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `2e0e7538f752fc49f475e70ca44fb7d28bca4fe4`.
- Size: 4262 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Shared JSON historical.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Historical decode/assets measurements
- Findings: Host/build context differs; no full startup
- RustChunk relevance: Potential design/test input for future RustChunk runtime/perf-json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Historical reference**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/perf-json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 516. `benchmarks/shared-json-startup-20261017.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `92f557b33b07d81ff52d9ff09c5cb8848971f30a`.
- Size: 1927 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Shared JSON startup microbenchmark.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Loader startup timing/RSS over five repeats
- Findings: High host load, not full app startup
- RustChunk relevance: Potential design/test input for future RustChunk runtime/perf-json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/perf-json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 517. `benchmarks/shared-json-startup-followup-20261017.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `c828ac7b4ccd2c622523844be7c3c252fa2c5476`.
- Size: 4584 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Shared JSON follow-up.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Five-repeat loader timing for variants
- Findings: Outliers/load variance prevent robust winner
- RustChunk relevance: Potential design/test input for future RustChunk runtime/perf-json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/perf-json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 518. `benchmarks/shared-json-variant-isolation-20261017.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e399c7ba1ec93f01f8c173e246dd1bcff2391ca3`.
- Size: 9559 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–454.
- Purpose: Shared JSON variant isolation.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: Arc-key/reused sound/flags variants
- Findings: No meaningful gain; experiments reverted
- RustChunk relevance: Potential design/test input for future RustChunk runtime/perf-json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Historical experiment**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/perf-json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 519. `benchmarks/tick-state-2026-10-04/live-observations.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `2c914aabb59e2a0e591d751d35dafc4144312beb`.
- Size: 2679 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Tick-state observation.
- Dependencies / cross-references: Pinned source/fixture artifact; upstream inputs and JAR references are evidence materials, not runtime dependencies.
- Implementation / logic: 285 samples/10s, ~28.4 observed ticks/s and entity flags
- Findings: One demo-world observation, not controlled benchmark
- RustChunk relevance: Potential design/test input for future RustChunk runtime/tick-observation; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference telemetry only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/tick-observation; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 520. `crates/cargo-horizon-mod/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `2523d9146f66c7892c60c4c1e1a3182852636f61`.
- Size: 478 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: cargo-horizon-mod package.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Cargo metadata for Horizon mod authoring/check CLI
- Findings: Mod ecosystem not needed for vanilla client
- RustChunk relevance: Potential design/test input for future RustChunk mods/tooling; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Unnecessary for first milestone**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/tooling; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 521. `crates/cargo-horizon-mod/src/check.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `174e6e8c528f77c147c023db58e9476b4ec205c1`.
- Size: 6285 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Mod package checker.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Checks package size/signatures/content-only/native separation
- Findings: Native source unsandboxed and not run by registry check
- RustChunk relevance: Potential design/test input for future RustChunk mods/validation; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Defer; trust concepts only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/validation; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 522. `crates/cargo-horizon-mod/src/main.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `187f1b13f5267b360e34f4e9a0b1665462511737`.
- Size: 15667 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–402.
- Purpose: Mod authoring CLI.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: new/build/run/check/registry commands/templates
- Findings: Horizon format, not Forge/1.20.1 loader
- RustChunk relevance: Potential design/test input for future RustChunk mods/tooling; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Defer; not Forge**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/tooling; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 523. `crates/cargo-horizon-mod/src/registry.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `38947d1380b0039bfee265eb57a56f714fd41f2c`.
- Size: 9278 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Mod registry CLI.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Build/review/check/list/release/page operations
- Findings: Horizon mod registry; base load error can appear empty
- RustChunk relevance: Potential design/test input for future RustChunk mods/registry; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Unnecessary for base client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/registry; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 524. `crates/cargo-horizon-mod/template/content/block.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ea68a1a27bcb11bd86f5a27585a8ed23bf240190`.
- Size: 94 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Content block template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Minimal JSON block definition
- Findings: Horizon API 26.3-specific
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Defer**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 525. `crates/cargo-horizon-mod/template/content/blockstate.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ea472666e34a367ff01a74006532cad02d42d672`.
- Size: 75 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Content blockstate template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Default blockstate/model mapping
- Findings: Horizon paths/conventions
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Defer**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 526. `crates/cargo-horizon-mod/template/content/horizon-mod.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `3ce05053adb663d3f13e453bc06fdc09cd6f83a9`.
- Size: 306 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Content manifest template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Horizon mod id/API/side/capability metadata
- Findings: API0.2, not Forge
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Defer**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 527. `crates/cargo-horizon-mod/template/content/item.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `411bc44994a8e7b9b991ffa3d07963fb4aabbbb6`.
- Size: 85 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Content item template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Item model JSON referencing block model
- Findings: No Forge registration
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Defer**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 528. `crates/cargo-horizon-mod/template/content/lang.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `74baddab85cf4fcb725d1ea4a690e31be085d36f`.
- Size: 56 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Content language template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Language JSON sample
- Findings: Localization idea generic; ids/resource paths differ
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable localization only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 529. `crates/cargo-horizon-mod/template/content/loot.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `898111f1b9ef1a2a269421f8b285d5ae292084ab`.
- Size: 277 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Content loot template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Block loot-table JSON
- Findings: 1.20.1 codec validation required
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable data-pack concept**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 530. `crates/cargo-horizon-mod/template/content/model.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `5710fecb137ff049892cf1662da03e3d8266785e`.
- Size: 102 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Content model template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Minimal cube model JSON
- Findings: Model schema/resolution version-dependent
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable sample only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 531. `crates/cargo-horizon-mod/template/content/pickaxe.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `8725edebd0f506056585686e6fd9dbabb83c2aac`.
- Size: 42 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Content pickaxe tag.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Mineable pickaxe tag entry
- Findings: Tag path/version must be checked
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable resource idea**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 532. `crates/cargo-horizon-mod/template/content/recipe.json`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `5edb03d945a418050f254f4e092a695927c94949`.
- Size: 193 bytes.
- Coverage: Complete artifact fetched and parsed; all records inspected.
- Purpose: Content recipe template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Crafting recipe JSON
- Findings: Recipe codec/IDs need target validation
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable example**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 533. `crates/cargo-horizon-mod/template/gitignore`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba`.
- Size: 8 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Mod gitignore.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Ignores generated build files
- Findings: Generic hygiene but paths upstream-specific
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable after path rewrite**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 534. `crates/cargo-horizon-mod/template/native/api.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `0bb415ecea9bb5cc87fd6f24ac89d961a4a91263`.
- Size: 303 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Native API Cargo template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Horizon native hook dependency
- Findings: Pinned Horizon API unrelated to Forge
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Unnecessary initially**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 535. `crates/cargo-horizon-mod/template/native/horizon-mixin.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `75ca99244f89ab459360693ab62c6a48503bd9d9`.
- Size: 243 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Native mixin template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Horizon native mixin compatibility manifest
- Findings: Not Forge/1.20.1 schema
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Unnecessary initially**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 536. `crates/cargo-horizon-mod/template/native/lib.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `028a4732c123b949ceec374f77ef9947af53c1da`.
- Size: 680 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Native mod lib template.
- Dependencies / cross-references: Horizon mod authoring/content packaging template; upstream paths and schema are not Forge formats.
- Implementation / logic: Rust hook entrypoint sample
- Findings: Unsandboxed Horizon hooks, not Forge
- RustChunk relevance: Potential design/test input for future RustChunk mods/templates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Defer**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/templates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 537. `crates/horizon-android/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4f8c7cbbca26d2298d57e7d7bfac38dbb1ebe689`.
- Size: 961 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Android Cargo package.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: NativeActivity/winit/JNI and Horizon client linkage
- Findings: Packaging pattern useful but game/package coupling needs rewrite
- RustChunk relevance: Potential design/test input for future RustChunk android/integration; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable Android integration**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is android/integration; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 538. `crates/horizon-anticheat/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `336626f3477778d42a1908e1a7d535c90105d204`.
- Size: 386 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Anti-cheat crate package.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Server anti-cheat dependencies
- Findings: Server-only, not client runtime
- RustChunk relevance: Potential design/test input for future RustChunk server/anti-cheat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/anti-cheat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 539. `crates/horizon-anticheat/src/combat.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `fdb9cecb2fe9bf5a885589a8bb2ffe56bff42abd`.
- Size: 3524 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Anti-cheat combat check.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Per-target/tick attack event counts
- Findings: Server validation only
- RustChunk relevance: Potential design/test input for future RustChunk server/anti-cheat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/anti-cheat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 540. `crates/horizon-anticheat/src/lib.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `83141733efcb4af4baaff45e5f1036e189e59869`.
- Size: 18172 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–427.
- Purpose: Anti-cheat aggregator.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Timer/ground/prediction/combat checks and violation actions
- Findings: Server moderation policy, not client engine
- RustChunk relevance: Potential design/test input for future RustChunk server/anti-cheat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/anti-cheat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 541. `crates/horizon-anticheat/src/prediction.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `eac5b4c4b9d78210e054a19db6b2c308c89460ad`.
- Size: 17374 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–417.
- Purpose: Anti-cheat prediction tracker.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Server observed movement/prediction history
- Findings: Server authority only
- RustChunk relevance: Potential design/test input for future RustChunk server/anti-cheat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/anti-cheat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 542. `crates/horizon-anticheat/src/timer.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `58aa41247c1a696fa357df054c7fb7384c810673`.
- Size: 4037 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Anti-cheat timer check.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Server tick cadence validation
- Findings: Server-only detection
- RustChunk relevance: Potential design/test input for future RustChunk server/anti-cheat; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/anti-cheat; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 543. `crates/horizon-assets/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `892ea09728b637b5fe0a5ec4b7a96da2fb022d20`.
- Size: 401 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Assets crate package.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Resource/model handling dependencies and module boundary
- Findings: Data/source APIs 26.3-specific
- RustChunk relevance: Potential design/test input for future RustChunk render/assets; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable boundary; rewrite assets**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/assets; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 544. `crates/horizon-assets/examples/blockstate_binding_fixture.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `c69884a5c355f7f43325e74d7d2d66673074d874`.
- Size: 2584 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Blockstate binding fixture.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Property schemas/state lists/selectors/multipart
- Findings: Harness only; rebuild target registry data
- RustChunk relevance: Potential design/test input for future RustChunk render/blockstates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable parity harness**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/blockstates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 545. `crates/horizon-assets/examples/blockstate_model_fixture.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `53ec83fd21b2e106d229c69a66766af014f26b2c`.
- Size: 1982 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Blockstate model fixture.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Decodes blockstate variants/multipart
- Findings: Not runtime/GPU render
- RustChunk relevance: Potential design/test input for future RustChunk render/blockstates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable parser oracle**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/blockstates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 546. `crates/horizon-assets/examples/cuboid_model_fixture.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `cff7b67effc8854108b8315e9b28b6513ce7a0a2`.
- Size: 748 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Cuboid model fixture.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Decodes cuboid JSON model
- Findings: No atlas/parents/GPU
- RustChunk relevance: Potential design/test input for future RustChunk render/model-data; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable fixture**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-data; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 547. `crates/horizon-assets/examples/item_model_fixture.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `615492fe673ec50e7312ce9034500bc8b684cd5c`.
- Size: 15494 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Item model fixture.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Item layers/PNG alpha/component prototypes and quads/transforms/tints
- Findings: 26.3 catalog and CPU fixture; not Android GPU proof
- RustChunk relevance: Potential design/test input for future RustChunk render/item-model; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable architecture; rebuild catalogs**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/item-model; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 548. `crates/horizon-assets/examples/model_baking_fixture.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `63e9b0c79530b50c04c5c704a61346f619f004ef`.
- Size: 2893 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Model baking fixture.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Geometry positions/UV/facing/cull/material outputs
- Findings: CPU geometry only
- RustChunk relevance: Potential design/test input for future RustChunk render/model-baking; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable geometry oracle**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-baking; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 549. `crates/horizon-assets/examples/model_choice_fixture.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1bd3aea8b8053c172c61dba17e087535bc6ee32b`.
- Size: 3482 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Model choice fixture.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Weighted variants and random draw selection
- Findings: RNG target semantics need verification
- RustChunk relevance: Potential design/test input for future RustChunk render/blockstates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable selection harness**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/blockstates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 550. `crates/horizon-assets/examples/model_resolution_fixture.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e549783c27b26d94934fb79be513566fbf88ea92`.
- Size: 2687 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Model resolution fixture.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Parent/texture/context resolution
- Findings: No GPU/final rendering
- RustChunk relevance: Potential design/test input for future RustChunk render/model-resolution; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable inheritance tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-resolution; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 551. `crates/horizon-assets/examples/sprite_animation_fixture.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1b9d016a7ccdc1b6338d9d5d1169b34819710a7d`.
- Size: 2639 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Sprite animation fixture.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Frame/tick/duration edge sampling
- Findings: No image/atlas/GPU stage
- RustChunk relevance: Potential design/test input for future RustChunk render/animation; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable state tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/animation; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 552. `crates/horizon-assets/examples/support/mod.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `0d0022baf67063370220585b52053a238e069792`.
- Size: 2651 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Asset fixture helpers.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Normalize geometry/transforms/faces/textures
- Findings: Test serialization utilities only
- RustChunk relevance: Potential design/test input for future RustChunk render/test-support; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable test support**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/test-support; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 553. `crates/horizon-assets/src/animation.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `66f494798788f4193efbe7b5b2722573593df84d`.
- Size: 6832 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Sprite animation implementation.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Metadata parsing, frame sampling, dirty/draw state
- Findings: CPU side; GPU interpolation/atlas absent
- RustChunk relevance: Potential design/test input for future RustChunk render/animation; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable CPU logic**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/animation; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 554. `crates/horizon-assets/src/bake.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ee77e7cd31a17c5c2b5b3773e064807c041eadcf`.
- Size: 12550 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Model baking implementation.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Cube/cuboid quads, UV/normals/face order/material/transforms
- Findings: CPU geometry, no atlas/shader/pixel parity
- RustChunk relevance: Potential design/test input for future RustChunk render/model-baking; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Highly adaptable; port with oracle**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-baking; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 555. `crates/horizon-assets/src/bake/transform.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `f431e3dbd9b7700ae53d716a73925ffed4267148`.
- Size: 8521 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Blockstate transform.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Rotations/UV-lock matrices using 64 cached rotations
- Findings: JOML/asset behavior 26.3-specific
- RustChunk relevance: Potential design/test input for future RustChunk render/blockstate-transform; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable math; rebuild transforms**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/blockstate-transform; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 556. `crates/horizon-assets/src/blockstate.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e482aa66f71da3075799a3899a5a519f1eba230d`.
- Size: 7996 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Blockstate parser.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Variants/multipart parse preserving JSON order/duplicate keys
- Findings: Target JSON/schema semantics need validation
- RustChunk relevance: Potential design/test input for future RustChunk render/blockstates; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable parser**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/blockstates; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 557. `crates/horizon-assets/src/blockstate/bind.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `a55245c11f9b1b39ca0295ff7853e661798ac926`.
- Size: 6172 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Blockstate binder.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Bind selectors to property schemas/states
- Findings: Invalid/overlap semantics need 1.20.1 oracle
- RustChunk relevance: Potential design/test input for future RustChunk render/blockstate-bind; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable binding**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/blockstate-bind; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 558. `crates/horizon-assets/src/blockstate/choice.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `af528cb294035250c69bba539ed3682105240c38`.
- Size: 5898 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Weighted blockstate choice.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Weighted selection/RNG draw ordering
- Findings: Random consumption affects appearance; re-oracle
- RustChunk relevance: Potential design/test input for future RustChunk render/blockstate-choice; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable algorithm**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/blockstate-choice; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 559. `crates/horizon-assets/src/item_bake.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `65c6822025b2ab2129d3b206682d0fedea4fd63a`.
- Size: 13808 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Item model baking.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Generated extrusion via PNG alpha edges/display transforms
- Findings: BTree ordering may differ from Java HashSet; no GPU
- RustChunk relevance: Potential design/test input for future RustChunk render/item-baking; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable CPU bake**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/item-baking; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 560. `crates/horizon-assets/src/item_model.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `2ad862c8d0ab78a3110c5224a59e82548e346441`.
- Size: 64502 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–1757.
- Purpose: Item model tree.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Parses item nodes, 122 components and condition/select/range/special/tint
- Findings: All component ids/selectors are 26.3 data
- RustChunk relevance: Potential design/test input for future RustChunk render/item-model; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Architecture adaptable; rewrite data**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/item-model; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 561. `crates/horizon-assets/src/item_model/component_predicate.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `81776b36737d0598f22bd843fdd2a06e18d9b197`.
- Size: 6652 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Component predicate.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Enchantment/stored enchantment/tag/level predicates
- Findings: Only selected families; rebuild component registry
- RustChunk relevance: Potential design/test input for future RustChunk render/item-predicate; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable predicate architecture**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/item-predicate; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 562. `crates/horizon-assets/src/item_model/foreign.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `bceff247abe88d8f3bb2bca88edc8d45b0ad79b1`.
- Size: 3690 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Foreign item fallback.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Unknown namespaces/external nodes fall back
- Findings: No actual mod hook execution; loader support limited
- RustChunk relevance: Potential design/test input for future RustChunk mods/render-interop; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference future extension only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/render-interop; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 563. `crates/horizon-assets/src/item_model/foreign_tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `f8ee95715f2f9753f1cf5a24c6938d7472bcf46d`.
- Size: 5885 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Foreign model tests.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Fallback/foreign model behavior and fluid container tests
- Findings: Fallback is not mod runtime/Forge compatibility
- RustChunk relevance: Potential design/test input for future RustChunk mods/render-interop; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable tests only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/render-interop; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 564. `crates/horizon-assets/src/item_model/special.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `b7b9635e7db4de7839848c609bb128ae25b9c779`.
- Size: 8659 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Special item models.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Special codecs/banner/pot/player/shield metadata
- Findings: Actual renderer elsewhere; 26.3 catalog
- RustChunk relevance: Potential design/test input for future RustChunk render/special-items; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable schema; rebuild data**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/special-items; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 565. `crates/horizon-assets/src/item_model/tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `65cdfa99fe4bf5384deead8bd4a2168c6a28d031`.
- Size: 891 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Item model tests.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Focused defaults/modulo/display-context/fallback-tint tests
- Findings: Not renderer acceptance
- RustChunk relevance: Potential design/test input for future RustChunk render/item-model; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable focused tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/item-model; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 566. `crates/horizon-assets/src/json.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `32a90d9f58cb11f55b0be91f6b8a7d3f11ba8674`.
- Size: 6445 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: JSON compatibility helpers.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Custom scanner mimicking Gson parsing semantics
- Findings: 1024-char token cap and parser edges need tests
- RustChunk relevance: Potential design/test input for future RustChunk runtime/json; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable only after oracle**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/json; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 567. `crates/horizon-assets/src/lib.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ac28fe50bbafda742d2d2da51bed4c11aacd02f8`.
- Size: 289 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Assets exports.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Exports animation/baking/blockstate/item/model/palette/resolve and forbids unsafe
- Findings: Module aggregation, not client integration
- RustChunk relevance: Potential design/test input for future RustChunk render/assets; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable boundary**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/assets; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 568. `crates/horizon-assets/src/math.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `095a0e6f4c6a4b589630dd1a46ba2778875e0606`.
- Size: 20000 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–579.
- Purpose: Matrix/quaternion math.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: JOML-like Matrix3/4 and quaternion operations/decomposition
- Findings: f32/platform-sensitive; ARM64/Java oracle required
- RustChunk relevance: Potential design/test input for future RustChunk math/renderer; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Highly adaptable math**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is math/renderer; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 569. `crates/horizon-assets/src/model.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `9fc7c9d5696b4944f5b35fe77e5ee139ea6417e1`.
- Size: 18371 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–529.
- Purpose: Cuboid model representation.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Geometry/parents/AO/gui lighting/textures/display transforms
- Findings: NeoForge OBJ/composite and 26.3 data need review
- RustChunk relevance: Potential design/test input for future RustChunk render/model-data; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable resource model**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-data; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 570. `crates/horizon-assets/src/obj.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `42227e163b4570d181ec67296800f7944eace996`.
- Size: 9336 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: OBJ parser.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: OBJ vertices/UV/materials/negative indices/n-gon triangulation
- Findings: Optional extension/MTL not base vanilla requirement
- RustChunk relevance: Potential design/test input for future RustChunk mods/render-interop; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Defer optional extension**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is mods/render-interop; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 571. `crates/horizon-assets/src/palette.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `9f825d05dafc4c434588c052f3983a2808d6ae68`.
- Size: 5107 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Palette permutation.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: ARGB palette mapping and atlas variant grouping
- Findings: CPU utility only; no texture upload
- RustChunk relevance: Potential design/test input for future RustChunk render/palette; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable utility**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/palette; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 572. `crates/horizon-assets/src/resolve.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `f3cfc23d8e61d2248639a902491736319ad0de7c`.
- Size: 22925 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–583.
- Purpose: ModelLibrary resolver.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Parent/alias/overlay/missing/generated models/textures/cycle handling
- Findings: No final GPU parity; 26.3 resource rules
- RustChunk relevance: Potential design/test input for future RustChunk render/model-resolution; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Highly adaptable architecture**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is render/model-resolution; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 573. `crates/horizon-bench/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `5a18849900a08273363120257db7ac7549775123`.
- Size: 453 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Benchmark package.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: Versioned report generation/data dependencies
- Findings: Tooling, not gameplay
- RustChunk relevance: Potential design/test input for future RustChunk runtime/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable measurement schema**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 574. `crates/horizon-bench/src/hardware.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `bb9e48172c2ee64ddf4efddf22ebe4925cfd0a9a`.
- Size: 9665 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Hardware probes.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: OS/CPU/RAM/load/process RSS/CPU; GPU/display external
- Findings: Android needs native APIs
- RustChunk relevance: Potential design/test input for future RustChunk runtime/hardware; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable telemetry**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/hardware; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 575. `crates/horizon-bench/src/lib.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ae70977bd30553f638431bfabfc8ce794e3cef29`.
- Size: 16543 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–464.
- Purpose: Benchmark report schema.
- Dependencies / cross-references: Rust source imports, implementation and in-file tests were reviewed at the pinned commit.
- Implementation / logic: CI95, summaries, frame percentiles/low-FPS/histograms/upload scrub
- Findings: Upstream-specific schema; tests format logic only
- RustChunk relevance: Potential design/test input for future RustChunk runtime/benchmark; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Adaptable report design**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/benchmark; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 576. `crates/horizon-bots/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `5a3e5e5c7b349e7add8a4dc70b3c54f2217fc14b`.
- Size: 346 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Bots package.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Dedicated-server stress bot dependencies
- Findings: Not playable Android client
- RustChunk relevance: Potential design/test input for future RustChunk server/bots; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client; optional QA**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/bots; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 577. `crates/horizon-bots/src/main.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `cb3e60f1b90b16566f909f1b2d11b88001dd7936`.
- Size: 64760 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–1661.
- Purpose: Bots harness.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Offline bot login/chunks/movement/walk/explore/fight/idle/load modes
- Findings: Protocol26.3 and synthetic flight; not faithful human client
- RustChunk relevance: Potential design/test input for future RustChunk server/bots; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Server QA reference only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/bots; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 578. `crates/horizon-chat-filter/Cargo.toml`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `e9b8823ed80ea96388b8ef496a90cddc2a4a9cba`.
- Size: 410 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Chat filter package.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Server moderation crate
- Findings: Not client/render dependency
- RustChunk relevance: Potential design/test input for future RustChunk server/chat-filter; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/chat-filter; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 579. `crates/horizon-chat-filter/data/slurs.txt`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ba2e9ff15d20ecdf83b772b6ae74333650ecdec3`.
- Size: 1475 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Chat denylist.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Moderation denylist data
- Findings: Not needed in client; false-positive policy concern
- RustChunk relevance: Potential design/test input for future RustChunk server/chat-filter; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/chat-filter; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 580. `crates/horizon-chat-filter/src/lib.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `a52a721b01c7801f31ac7e2634bf519498271f63`.
- Size: 17362 bytes.
- Coverage: Complete file read in consecutive chunks of at most 400 lines: 1–400, 401–547.
- Purpose: Chat filter engine.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Normalizes/filter scores/warn-mute-ban and logs/persists
- Findings: Server moderation policy, not client chat rendering
- RustChunk relevance: Potential design/test input for future RustChunk server/chat-filter; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Exclude from client**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/chat-filter; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 581. `crates/horizon-chat-filter/src/normalise.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `51c1443976335775ff6cd51cff1ccc0850a16cfd`.
- Size: 9597 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Chat normalization.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Confusable/leetspeak/accent/separator/invisible-char folding
- Findings: Language-dependent; can false-positive
- RustChunk relevance: Potential design/test input for future RustChunk server/chat-filter; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/chat-filter; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 582. `crates/horizon-chat-filter/src/tests.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `9c17e668191e938bd69058de8de1b5a433e7e188`.
- Size: 9647 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Chat filter tests.
- Dependencies / cross-references: Dedicated-server/bot/tooling artifact; host/workload/protocol context comes from the source report. Not a client runtime dependency.
- Implementation / logic: Obfuscation/allowlist/strike/persistence/restart
- Findings: One server policy, not general language semantics
- RustChunk relevance: Potential design/test input for future RustChunk server/chat-filter; the current workspace does not yet implement this gameplay/rendering subsystem.
- Decision: **Reference tests only**
- 1.20.1 implications: Upstream targets Minecraft 26.3. Reconstruct 1.20.1 APIs, data, registries, assets, save schemas and protocol behavior; version-string substitution is not a port.
- Follow-up checks: Map the equivalent 1.20.1 Java owner/API, rebuild version-correct data and fixtures, compare Java oracle and Rust output, then integrate and validate on ARM64.
- RustChunk implementation mapping: Future destination is server/chat-filter; this upstream file is not evidence of an already implemented RustChunk runtime module.
- Evidence required before adoption: Require pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android validation where applicable.
- Completion state: Complete source/artifact review recorded. No RustChunk build, upstream test rerun, APK, server join or Galaxy A22 test was performed in this audit.

### 583. `crates/horizon-client-core/src/block_entity_nbt.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `fef4e912279d4c8169033779a1fd6a140187debf`.
- Size: 37539 bytes.
- Coverage: Complete source read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1002.
- Purpose: Block-entity NBT schema conversion.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Canonicalizes save/load data, defaults/clamps and item/slot/sign/sculk/vault/trial block-entity fields.
- Findings: Includes 26.3-only block entities and fields; exact IDs/defaults must be rebuilt from 1.20.1.
- RustChunk relevance: Potential architecture/test input for future RustChunk world/block-entity-save; the current workspace has not yet implemented this runtime subsystem.
- Decision: **High-value architecture; rewrite schema**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is world/block-entity-save; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 584. `crates/horizon-client-core/src/bossbar.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `2ecdb25e956813e4d1043f228c8870e60f6ef58b`.
- Size: 7533 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Bossbar state and persistence.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Stores bar title/progress/color/style/visibility/darken/music/fog and player identifiers; NBT packing/unpacking.
- Findings: No packet synchronization or HUD rendering; color/catalog details are 26.3-specific.
- RustChunk relevance: Potential architecture/test input for future RustChunk client/bossbar; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable state model; rebuild target ids/colors**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is client/bossbar; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 585. `crates/horizon-client-core/src/component.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d6c4774d742c80cd0d74c2a199a5e3e6edb4d159`.
- Size: 28226 bytes.
- Coverage: Complete source read in consecutive chunks of at most 400 lines: 1–400, 401–716.
- Purpose: Text component JSON codecs.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Converts component JSON, style/event fields and 26.3-era component content/serialization.
- Findings: Later component forms differ from 1.20.1; not a full chat screen/renderer.
- RustChunk relevance: Potential architecture/test input for future RustChunk client/text; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable text tree; reimplement target codecs**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is client/text; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 586. `crates/horizon-client-core/src/fastutil_order.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `05e12af79bdad68c2939d99026d08b49078ea00a`.
- Size: 8555 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Fastutil-compatible ordering.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Imitates Java/fastutil map/set iteration order for selected observable collection paths.
- Findings: JDK collection internals are implementation details; do not port blindly unless 1.20.1 oracle shows observable behavior.
- RustChunk relevance: Potential architecture/test input for future RustChunk runtime/collections; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Reference regression; use Rust ordering where possible**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/collections; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 587. `crates/horizon-client-core/src/gamerules.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `8ad270baeb5efa704deff76756f913befe42241f`.
- Size: 12442 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Game-rule catalog.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Maps game-rule names, defaults and typed boolean/integer semantics.
- Findings: 26.3 rule names/defaults differ; rebuild full 1.20.1 catalog.
- RustChunk relevance: Potential architecture/test input for future RustChunk world/gamerules; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable typed model; replace data table**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is world/gamerules; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 588. `crates/horizon-client-core/src/identifier.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `a2fe1cd1327223aada1f4a3afced8e0b037c0963`.
- Size: 1351 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Resource identifier validation.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Two helpers normalize `namespace:path`, default namespaces and validate allowed characters.
- Findings: The helpers differ in strictness; compare each with exact 1.20.1 Identifier constructors/codecs.
- RustChunk relevance: Potential architecture/test input for future RustChunk runtime/identifier; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable parsing; enforce target validation**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is runtime/identifier; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 589. `crates/horizon-client-core/src/key_display.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `faf721afd5209904a8b292f7b438b4534aec7efb`.
- Size: 4373 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Keyboard key display.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Learns keyboard-layout symbol labels and formats key display names through SDL-style key naming.
- Findings: SDL/US symbol tables are upstream choices; Android hardware/IME input handling needs native adaptation.
- RustChunk relevance: Potential architecture/test input for future RustChunk input/key-display; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable presentation idea; use Android input APIs**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is input/key-display; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 590. `crates/horizon-client-core/src/lang.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `4810da22e6771fa2dad222ad7ed5bcb5e5e8f1ec`.
- Size: 26194 bytes.
- Coverage: Complete source read in consecutive chunks of at most 400 lines: 1–400, 401–648.
- Purpose: Localization lookup table.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Provides language key translation/fallback lookup for client text.
- Findings: The table is 26.3; load target 1.20.1 language assets and key names.
- RustChunk relevance: Potential architecture/test input for future RustChunk client/localization; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable translation architecture; replace resources**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is client/localization; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 591. `crates/horizon-client-core/src/nbt_path.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `9c4a0505199dfb2458c7210b71ff5bc42cb90739`.
- Size: 30967 bytes.
- Coverage: Complete source read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–902.
- Purpose: NBT path parser and mutation.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Parses paths and implements get/count/set/remove/insert plus compound/list/array matching.
- Findings: Path parser, array mutation and max-depth details need exact 1.20.1 Java oracle.
- RustChunk relevance: Potential architecture/test input for future RustChunk world/nbt-path; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Highly adaptable NBT path engine; differential-test semantics**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is world/nbt-path; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 592. `crates/horizon-client-core/src/nbt_value.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ff78ad1dbb946ade0ae91837504c4c79b02bb28c`.
- Size: 1057 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: NBT primitive value conversions.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Reads NBT strings and numeric tags as integer/floating values with vanilla-like coercion.
- Findings: Truncation, float/double conversion and wrong-type fallbacks are version-sensitive.
- RustChunk relevance: Potential architecture/test input for future RustChunk world/nbt; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable utility; verify numeric edge cases**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is world/nbt; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 593. `crates/horizon-client-core/src/particle_type.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `1e3bdd709345172bd23fda12212a888d4ff643bb`.
- Size: 8263 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Particle type catalog.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Lists particle IDs and per-type override-limiter/translucent-layer flags.
- Findings: Includes many later particle IDs; regenerate 1.20.1 registry and rendering layers.
- RustChunk relevance: Potential architecture/test input for future RustChunk render/particles; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Reference table layout; rebuild target catalog**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is render/particles; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 594. `crates/horizon-client-core/src/pattern.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `0ddd3b826195dd440f066271b88c96804ced2cc5`.
- Size: 4735 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Pack metadata regex subset.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Implements a small Java Pattern subset for `pack.mcmeta` filter matching.
- Findings: Groups/braces are unsupported; must preserve only verified subset semantics.
- RustChunk relevance: Potential architecture/test input for future RustChunk resource/pack-filter; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable minimal parser with conformance tests**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is resource/pack-filter; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 595. `crates/horizon-client-core/src/potions.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `832d1fc0174693ef3b9db956153cd87bb73bd913`.
- Size: 13499 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Potion and effect registry tables.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Defines effect categories/colors, potion recipes/durations/amplifiers, sprite keys and color mixing.
- Findings: Contains later effects and mod registration; regenerate all 1.20.1 potion/effect data.
- RustChunk relevance: Potential architecture/test input for future RustChunk registry/potions; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable table/lookup design; rewrite catalog**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is registry/potions; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 596. `crates/horizon-client-core/src/registries.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `672774462716d3a5b6023c3b45891364bff90f48`.
- Size: 12098 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Command suggestion registries.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Stores registry IDs/tags, entities, items, effects, biomes, dimensions, functions, scoreboard data, datapacks and custom completions.
- Findings: Server-provided registry state and 26.3 resource sets are version-specific; completion UI and packet lifecycle are external.
- RustChunk relevance: Potential architecture/test input for future RustChunk client/registries; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable data model; rebuild target registry ownership**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is client/registries; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 597. `crates/horizon-client-core/src/scoreboard.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `191b623b0bc55127eeeb0ead1c39134bbd41462b`.
- Size: 57539 bytes.
- Coverage: Complete source read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–1200, 1201–1600, 1601–1613.
- Purpose: Scoreboard/team state and NBT.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Implements objectives/criteria, teams, scores, display slots, score holders, NBT packing and component data.
- Findings: Criteria/stat catalog is 26.3; no full network packet synchronization or HUD integration.
- RustChunk relevance: Potential architecture/test input for future RustChunk client/scoreboard; the current workspace has not yet implemented this runtime subsystem.
- Decision: **High-value state/save architecture; rebuild criteria and packets**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is client/scoreboard; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 598. `crates/horizon-client-core/src/snbt.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `ab024fd74d3f1bbc381a56734f8b466945ace4f8`.
- Size: 11535 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: SNBT parse/format behavior.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Formats compact/pretty SNBT and handles Java number spelling and collection iteration heuristics.
- Findings: Hash iteration order is heuristic/insertion-based; exact formatting requires a pinned Java 1.20.1 oracle.
- RustChunk relevance: Potential architecture/test input for future RustChunk world/snbt; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable serialization utilities; verify exact output**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is world/snbt; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 599. `crates/horizon-client-core/src/sound_source.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `d181d6ac5aae9abd6bcbc519cc4ac03bdb622a4e`.
- Size: 243 bytes.
- Coverage: Complete source file read, including in-file tests where present.
- Purpose: Sound source catalog.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Defines the eleven built-in sound source names in vanilla ordering.
- Findings: Small static table; verify target ordering and server/client use sites rather than importing blindly.
- RustChunk relevance: Potential architecture/test input for future RustChunk registry/sound-sources; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Adaptable registry table; validate 1.20.1 list**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is registry/sound-sources; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.

### 600. `crates/horizon-client-core/src/text.rs`
- Source repository/ref: `starling-client/starling` / `5432370d573642041585c69b40fb045b27133488`.
- Source SHA: `5480570d818047e8a3fc0797bf2bbcce071c2b34`.
- Size: 29665 bytes.
- Coverage: Complete source read in consecutive chunks of at most 400 lines: 1–400, 401–800, 801–824.
- Purpose: Styled text and components.
- Dependencies / cross-references: Rust source imports and related client-core modules were reviewed at the pinned commit.
- Implementation / logic: Builds styled spans, colors/decorations, translatable templates, click/hover events, legacy `§` parsing/resolution and JSON component output.
- Findings: The 824-line implementation includes newer event/object fields; `open_file` is explicitly restricted to locally-created text, never server/LAN text. Rebuild 1.20.1 component schema and preserve this trust boundary.
- RustChunk relevance: Potential architecture/test input for future RustChunk client/text; the current workspace has not yet implemented this runtime subsystem.
- Decision: **Highly adaptable text engine; rewrite event/schema compatibility**
- 1.20.1 implications: Upstream targets Minecraft 26.3; reconstruct the corresponding 1.20.1 schema, identifiers, data and Java behavior rather than copying this catalog.
- Follow-up checks: Compare against the official 1.20.1 Java/JAR behavior, rebuild target data and fixtures, test serialization and packet/UI integration, then validate on ARM64.
- RustChunk implementation mapping: Future destination is client/text; this file does not prove the subsystem is already implemented in RustChunk.
- Evidence required before adoption: Pinned 1.20.1 Java oracle, independent Rust output/hash, reproducible CI, end-to-end integration and ARM64/Android tests where applicable.
- Completion state: Complete source review recorded. No build/APK/server session/device test was run in this audit.
### Batch verification — entries 401–600 (2026-11-18)

- Selection: 200 unique source blobs were selected in pinned tree order after entry 400. Two paths in that interval (`crates/horizon-client-core/Cargo.toml` and `crates/horizon-client-core/src/lib.rs`) had already been audited as entries 101 and 114, so they were not counted twice; the next two unreviewed blobs (`sound_source.rs` and `text.rs`) complete the 200 unique-file batch.
- Provenance/structure: every path and SHA is taken from the complete pinned tree (5,749 nodes / 4,968 blobs; `truncated=false`). Every entry has exactly one of the 15 audit fields. JSON/JSONL files were parsed; source files and large artifacts were read fully. Files over 400 lines were reviewed in consecutive chunks no larger than 400 lines, recorded per entry.
- Parity scope: entries 401–485 are mostly scoped tests for Minecraft 26.3, not 1.20.1 results. Some effect/collection manifests lack an aggregate top-level `rust_output_sha256` or contain differing reference output hashes; these remain evidence gaps.
- Server benchmark scope: entries 486–510 measure dedicated servers and remote bot workloads, not client rendering or Android devices. Some tests equalize chunk work; others let each server set the pace. ThinkPad i7-7500U (2 cores/4 threads) hosted the servers while remote bot load ran on a separate Mac mini. Paper/Fabric sometimes beat Horizon on average MSPT; vanilla watchdog/broken-pipe failures occur in some high-load runs. None predicts RustChunk performance on Galaxy A22.
- JSON/performance scope: entries 511–519 are JSON-loader microbenchmarks and a single live tick observation, not full launcher startup, first-frame latency, Android memory or game performance.
- Mod/server separation: entries 520–542 include Horizon mod authoring, native hooks, server anti-cheat and bot tooling. `.hmod`/`.hnmod` are Horizon formats, not Forge JARs; these functions are deferred or excluded from the vanilla client milestone.
- Rendering/client-core relevance: entries 543–600 include CPU model/animation/blockstate/item-model code and NBT/text/scoreboard/registry modules. These are useful architectural references, but 26.3 resource registries, components, particles/effects/potions, block entities and save semantics must be rebuilt for 1.20.1.
- Verification boundary: no upstream tests were rerun; no RustChunk build/APK was made; no server was joined; no Forge mod was loaded; no Galaxy A22 test or performance measurement was performed.
- Storage: entries 1–400 in `AUDIT_LEDGER.md` remain untouched. This continuation is stored in `AUDIT_LEDGER_401_600.md` and linked from the index because the existing ledger exceeds the content retrieval limit.
