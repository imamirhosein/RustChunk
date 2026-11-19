# RustChunk Starling Audit Ledger — Entries 1401–1600

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1401–1600 (200 new unique source files; one shared Ledger for the complete stage).
- Current progress: entries 1401–1425 committed (25/200); first checkpoint of section 1401–1450 complete.
- Authorized section: 1401–1450, split into checkpoints 1401–1425 and 1426–1450; complete the second checkpoint without another permission request.
- Reporting rule: stop after 1450 and wait for explicit authorization before 1451–1500.
- Each entry contains the 15 required fields defined in AUDIT_PROTOCOL.md.
- Long files were read in consecutive, gap-free ranges of at most 400 logical lines through EOF. Source SHA and byte size match the pinned source tree.
- Embedded tests were inspected only, not executed.

### 1401. `crates/horizon-client/src/game/perspective.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `15cb1c8d1b371e67eab62fab9cebe5606ec4ab1f`
- Size: 69498 bytes; 1,708 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1708
- Purpose: Camera modes, camera collision and player model/player-preview rendering, including armor, cape and elytra.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Camera modes, camera collision and player model/player-preview rendering, including armor, cape and elytra. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild against 1.20.1 camera, pose, model-layer, attribute and Forge render contracts.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 1,708 logical lines read through EOF in the listed coverage; pinned SHA and 69,498 byte size match; embedded tests inspected only, not run.

### 1402. `crates/horizon-client/src/game/pick_entity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `41138a89ade674ccbc481cad603c5370ab7939d2`
- Size: 5416 bytes; 126 logical lines
- Coverage: 1–126
- Purpose: Entity crosshair targeting and creative pick-block item resolution.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Entity crosshair targeting and creative pick-block item resolution. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild 1.20.1 entity set, reach, dragon parts and pick-result mapping.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 126 logical lines read through EOF in the listed coverage; pinned SHA and 5,416 byte size match; embedded tests inspected only, not run.

### 1403. `crates/horizon-client/src/game/pickups.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `192b227f44a287de0bf785811a24eea9d343cf60`
- Size: 6411 bytes; 193 logical lines
- Coverage: 1–193
- Purpose: Visual pickup interpolation for dropped items, experience orbs, arrows and tridents.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Visual pickup interpolation for dropped items, experience orbs, arrows and tridents. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: adaptable; reference; optional/future
- 1.20.1 implications: Keep transfer server-authoritative; optionally recreate 1.20.1 pickup animation.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 193 logical lines read through EOF in the listed coverage; pinned SHA and 6,411 byte size match; embedded tests inspected only, not run.

### 1404. `crates/horizon-client/src/game/piglins.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0891cd0fb89a553820bec3d0f753801f13d713dd`
- Size: 18359 bytes; 444 logical lines
- Coverage: 1–400, 401–444
- Purpose: Piglin pickup/equipment, item preference, barter, baby behavior and protected-block anger.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Piglin pickup/equipment, item preference, barter, baby behavior and protected-block anger. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Map 1.20.1 tags, barter loot, pickup, armor preference and anger rules.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 444 logical lines read through EOF in the listed coverage; pinned SHA and 18,359 byte size match; embedded tests inspected only, not run.

### 1405. `crates/horizon-client/src/game/piston_push.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dada9d77af35ea86616a0e3fc059b1cf5a385f2c`
- Size: 25809 bytes; 668 logical lines
- Coverage: 1–400, 401–668
- Purpose: Piston movement of players/entities, shulker pushes, slime bounce, honey and collision constraints.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Piston movement of players/entities, shulker pushes, slime bounce, honey and collision constraints. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild exact 1.20.1 push order, clipping, displacement cap and sticky/slime/honey rules.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 668 logical lines read through EOF in the listed coverage; pinned SHA and 25,809 byte size match; embedded tests inspected only, not run.

### 1406. `crates/horizon-client/src/game/player_equip.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a565fac457c0ece303178b7e9b75306b47b90bf1`
- Size: 3816 bytes; 96 logical lines
- Coverage: 1–96
- Purpose: Tracks player armor/offhand changes and emits equip sounds/game events.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Tracks player armor/offhand changes and emits equip sounds/game events. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: adaptable; rewrite required; 1.20.1
- 1.20.1 implications: Implement 1.20.1 armor-slot/sound rules; exclude Horizon native callback.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 96 logical lines read through EOF in the listed coverage; pinned SHA and 3,816 byte size match; embedded tests inspected only, not run.

### 1407. `crates/horizon-client/src/game/postprocess.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `77cba6832fc2945e0d2b0ead4e73f87d8995fbd4`
- Size: 2808 bytes; 67 logical lines
- Coverage: 1–67
- Purpose: Delays chunk postprocessing until neighboring chunks are ready; handles fluid/block/shape updates.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Delays chunk postprocessing until neighboring chunks are ready; handles fluid/block/shape updates. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild 1.20.1 postprocess order/flags/fluid scheduling without needless chunk display delay.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 67 logical lines read through EOF in the listed coverage; pinned SHA and 2,808 byte size match; embedded tests inspected only, not run.

### 1408. `crates/horizon-client/src/game/potent_sulfur.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c2dd56d29671fb78ea2faa5b28b1ee55c462fc74`
- Size: 18425 bytes; 485 logical lines
- Coverage: 1–400, 401–485
- Purpose: Later-version Potent Sulfur geysers, gas states, eruptions, launch forces and visuals.
- Dependencies / cross-references: Horizon LAN-host, native Create, later dialog/content systems; exact dependencies are version-specific.
- Implementation / logic: Later-version Potent Sulfur geysers, gas states, eruptions, launch forces and visuals. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Not needed for ordinary external-server client baseline; future optional reference only.
- Decision: unnecessary; version mismatch; future
- 1.20.1 implications: Do not port to baseline; a mod-specific feature needs its own 1.20.1 spec.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: No baseline mapping; optional isolated future subsystem only.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 485 logical lines read through EOF in the listed coverage; pinned SHA and 18,425 byte size match; embedded tests inspected only, not run.

### 1409. `crates/horizon-client/src/game/potions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0cb83dba5a234e140a2a9361beb56f2b671ca5b2`
- Size: 83202 bytes; 1,950 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600 (exact full-blob slice), 1601–1950 (exact full-blob slice)
- Purpose: Potion use/throwing, splash and lingering effects, clouds, projectile impact, effect ticking and HUD.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Potion use/throwing, splash and lingering effects, clouds, projectile impact, effect ticking and HUD. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: Two ambiguous range reads were resolved by exact slices from the complete pinned file; EOF coverage is complete.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Use 1.20.1 potion NBT/brewing/effect registries and cloud rules; isolate modded effects.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 1,950 logical lines read through EOF in the listed coverage; pinned SHA and 83,202 byte size match; embedded tests inspected only, not run.

### 1410. `crates/horizon-client/src/game/powder_snow.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d145d1c8e51bba3c69768fc0929169653be67e51`
- Size: 4049 bytes; 106 logical lines
- Coverage: 1–106
- Purpose: Powder-snow bucket pickup and fire extinguishing, with sound/events and inventory result.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Powder-snow bucket pickup and fire extinguishing, with sound/events and inventory result. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Map bucket result, creative rules and events to 1.20.1; movement/freezing lives elsewhere.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 106 logical lines read through EOF in the listed coverage; pinned SHA and 4,049 byte size match; embedded tests inspected only, not run.

### 1411. `crates/horizon-client/src/game/prediction.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3378c6a5698be1c85dedc88c1c487a182818c68`
- Size: 8509 bytes; 187 logical lines
- Coverage: 1–187
- Purpose: Replays LAN guest movement on host to compare prediction and anti-cheat state.
- Dependencies / cross-references: Horizon LAN-host, native Create, later dialog/content systems; exact dependencies are version-specific.
- Implementation / logic: Replays LAN guest movement on host to compare prediction and anti-cheat state. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Not needed for ordinary external-server client baseline; future optional reference only.
- Decision: optional; reference; LAN host
- 1.20.1 implications: Exclude from standalone client; future hosting must use Forge-authoritative 1.20.1 movement.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: No baseline mapping; optional isolated future subsystem only.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 187 logical lines read through EOF in the listed coverage; pinned SHA and 8,509 byte size match; embedded tests inspected only, not run.

### 1412. `crates/horizon-client/src/game/processing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8520e06d05a23f4111bae7bab8f2f1cbc8266b2c`
- Size: 25072 bytes; 646 logical lines
- Coverage: 1–400, 401–646
- Purpose: Native simulation of Create 6.0.10 processing machines and item/recipe handling.
- Dependencies / cross-references: Horizon LAN-host, native Create, later dialog/content systems; exact dependencies are version-specific.
- Implementation / logic: Native simulation of Create 6.0.10 processing machines and item/recipe handling. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: Native Create machine simulation is not Java Forge mod loading.
- RustChunk relevance: Not needed for ordinary external-server client baseline; future optional reference only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Do not include in base client; Create 1.20.1 requires its actual target mod behavior/network integration.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: No baseline mapping; optional isolated future subsystem only.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 646 logical lines read through EOF in the listed coverage; pinned SHA and 25,072 byte size match; embedded tests inspected only, not run.

### 1413. `crates/horizon-client/src/game/pumpkin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b58adc7135a0ec1fcc56c0f3ab90b1638cb8d5b`
- Size: 3555 bytes; 95 logical lines
- Coverage: 1–95
- Purpose: Shears carve pumpkins, choose facing, drop seeds and emit sounds/events.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Shears carve pumpkins, choose facing, drop seeds and emit sounds/events. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Match 1.20.1 facing, loot, durability, drop origin and golem-head updates.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 95 logical lines read through EOF in the listed coverage; pinned SHA and 3,555 byte size match; embedded tests inspected only, not run.

### 1414. `crates/horizon-client/src/game/quick_actions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8c9a9bea31ccbecf1944697ad8c17357115c799a`
- Size: 1902 bytes; 50 logical lines
- Coverage: 1–50
- Purpose: Opens Quick Actions dialog from synchronized dialog registry/tag.
- Dependencies / cross-references: Horizon LAN-host, native Create, later dialog/content systems; exact dependencies are version-specific.
- Implementation / logic: Opens Quick Actions dialog from synchronized dialog registry/tag. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Not needed for ordinary external-server client baseline; future optional reference only.
- Decision: unnecessary; version mismatch; future
- 1.20.1 implications: Exclude; later dialog registry/packet support is not assumed in 1.20.1.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: No baseline mapping; optional isolated future subsystem only.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 50 logical lines read through EOF in the listed coverage; pinned SHA and 1,902 byte size match; embedded tests inspected only, not run.

### 1415. `crates/horizon-client/src/game/raids.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `03a6833216da1fe9274d3461eda3a38d8a8e51a9`
- Size: 57517 bytes; 1,471 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1471
- Purpose: Patrol spawning, raid omen conversion, raid waves/boss bar, raider sync and village sieges.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Patrol spawning, raid omen conversion, raid waves/boss bar, raider sync and village sieges. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild from 1.20.1 Bad Omen, raid waves, patrols, POIs, siege schedule and raids.dat.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 1,471 logical lines read through EOF in the listed coverage; pinned SHA and 57,517 byte size match; embedded tests inspected only, not run.

### 1416. `crates/horizon-client/src/game/ram_limit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f9661b1930a6cbb9e00b277b4a8879b8e22ca7e0`
- Size: 3131 bytes; 75 logical lines
- Coverage: 1–75
- Purpose: Adapts render distance/far terrain to periodic memory pressure and exposes RAM-limit command.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Adapts render distance/far terrain to periodic memory pressure and exposes RAM-limit command. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: adaptable; reference; Android optimization
- 1.20.1 implications: Implement Android-aware 1.20.1 budgets and user override; do not copy Horizon LOD assumptions.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 75 logical lines read through EOF in the listed coverage; pinned SHA and 3,131 byte size match; embedded tests inspected only, not run.

### 1417. `crates/horizon-client/src/game/random_textures.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c80deba7352bdc125fd2dd45fdfc62998c6f029f`
- Size: 2396 bytes; 66 logical lines
- Coverage: 1–66
- Purpose: Chooses OptiFine-style random mob textures from environment and persistent seed.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Chooses OptiFine-style random mob textures from environment and persistent seed. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: optional; reference; OptiFine compatibility
- 1.20.1 implications: Add only for explicitly selected 1.20.1 OptiFine/ETF format; isolate from authoritative mob logic.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 66 logical lines read through EOF in the listed coverage; pinned SHA and 2,396 byte size match; embedded tests inspected only, not run.

### 1418. `crates/horizon-client/src/game/recipe_book.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a0875bbaf0dc05789abcf1381ed3da0da0bd3d0b`
- Size: 62401 bytes; 1,609 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1609
- Purpose: Recipe book UI for crafting/furnace: search, tabs, filtering, pages, placement, ghost slots and packets.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Recipe book UI for crafting/furnace: search, tabs, filtering, pages, placement, ghost slots and packets. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Map 1.20.1 recipe IDs, packets, known recipes, saved flags and menu placement; not later display IDs.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 1,609 logical lines read through EOF in the listed coverage; pinned SHA and 62,401 byte size match; embedded tests inspected only, not run.

### 1419. `crates/horizon-client/src/game/remote_camera.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d6139f85e2c9bae15985738e5b89c32c899b9db0`
- Size: 6206 bytes; 158 logical lines
- Coverage: 1–158
- Purpose: Server-directed spectator camera attachment, entity lookup and post-processing.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Server-directed spectator camera attachment, entity lookup and post-processing. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Match 1.20.1 packet semantics, supported entities, interpolation and post-effect mapping.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 158 logical lines read through EOF in the listed coverage; pinned SHA and 6,206 byte size match; embedded tests inspected only, not run.

### 1420. `crates/horizon-client/src/game/remote_chat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6d912d95955346fa310f5d1dccdd75330149ac19`
- Size: 19039 bytes; 523 logical lines
- Coverage: 1–523
- Purpose: Secure chat session, signed chats/commands, last-seen tracking, ACKs and unattended-command confirmation.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Secure chat session, signed chats/commands, last-seen tracking, ACKs and unattended-command confirmation. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: Offline profiles have no signing session and remain unsigned; this does not bypass enforce-secure-profile.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Implement exact 1.20.1 signing/last-seen scheme; never fabricate offline signatures.
- Follow-up checks: Compare 1.20.1 session, signature chain, command signing, ACKs and offline unsigned handling.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 523 logical lines read through EOF in the listed coverage; pinned SHA and 19,039 byte size match; embedded tests inspected only, not run.

### 1421. `crates/horizon-client/src/game/remote_chunks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `49c02bc5a98522328a1838343612a1bd2a9663af`
- Size: 19982 bytes; 486 logical lines
- Coverage: 1–400, 401–486
- Purpose: Decodes remote chunk-with-light packets, maps palettes/biomes/light and inserts/replaces columns.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Decodes remote chunk-with-light packets, maps palettes/biomes/light and inserts/replaces columns. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: Ordered decoder and immutable format snapshot are useful patterns; wire format must be rebuilt for 1.20.1.
- RustChunk relevance: Critical to chunk arrival, decoding, light and rendering latency; directly relevant to missing/late chunk symptoms.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Implement exact 1.20.1 ChunkData, heightmaps, palettes, biome maps, light masks and block-entity NBT.
- Follow-up checks: Compare 1.20.1 chunk/light codecs, palette packing, registry remaps, rapid bursts, unload/reload and ordered insertion.
- RustChunk implementation mapping: RustChunk network chunk decoder, immutable format snapshot, ordered worker queue and streaming insertion.
- Evidence required before adoption: Captured 1.20.1 packet fixtures, decode/remap tests, ordered completion and Android latency/frame traces.
- Completion state: COMPLETE — all 486 logical lines read through EOF in the listed coverage; pinned SHA and 19,982 byte size match; embedded tests inspected only, not run.

### 1422. `crates/horizon-client/src/game/remote_mob_data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a59560647c36bd439a052b0e81be91fcccdd73e3`
- Size: 47625 bytes; 1,036 logical lines
- Coverage: 1–400, 401–800, 801–1036
- Purpose: Applies synchronized server metadata to remote mobs/players for correct model and animation state.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Applies synchronized server metadata to remote mobs/players for correct model and animation state. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild metadata table/serializers from 1.20.1 hierarchy; handle unknown metadata safely.
- Follow-up checks: Build 1.20.1 metadata index/type matrix for each supported entity and validate packet fixtures.
- RustChunk implementation mapping: RustChunk remote entity metadata decoder and species state synchronization.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 1,036 logical lines read through EOF in the listed coverage; pinned SHA and 47,625 byte size match; embedded tests inspected only, not run.

### 1423. `crates/horizon-client/src/game/remote_mob_particles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1474ce4c13fb84dfc6964d8f034772c030db3e14`
- Size: 4454 bytes; 126 logical lines
- Coverage: 1–126
- Purpose: Client-side particles and local knockback for selected remote mob events.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Client-side particles and local knockback for selected remote mob events. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: optional; adapt; entity effects
- 1.20.1 implications: Implement only event handlers present in 1.20.1; keep authoritative motion/damage server-side.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 126 logical lines read through EOF in the listed coverage; pinned SHA and 4,454 byte size match; embedded tests inspected only, not run.

### 1424. `crates/horizon-client/src/game/remote_mob_use.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8ac6977349550a7a33f07f49b20859b666e140f5`
- Size: 6736 bytes; 145 logical lines
- Coverage: 1–145
- Purpose: Predicts client right-click mob result before choosing held-item fallback.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Predicts client right-click mob result before choosing held-item fallback. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Recreate 1.20.1 Mob.interactOn result semantics/hand choice; server remains authoritative.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 145 logical lines read through EOF in the listed coverage; pinned SHA and 6,736 byte size match; embedded tests inspected only, not run.

### 1425. `crates/horizon-client/src/game/remote_objects.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `250be19958e9e2523e2512037c08dcc091ddbc98`
- Size: 18942 bytes; 445 logical lines
- Coverage: 1–400, 401–445
- Purpose: Creates, updates, interpolates and removes remote vehicles, armor stands, frames, paintings and display objects.
- Dependencies / cross-references: Game/session state, registries, protocol or world data, rendering/UI and the related entity/block subsystem; exact interfaces are version-specific.
- Implementation / logic: Creates, updates, interpolates and removes remote vehicles, armor stands, frames, paintings and display objects. The implementation translates this state into client-side behavior, UI or packet-driven updates.
- Findings: The behavior/data contracts are version-sensitive; later-version APIs or species must not be assumed present in 1.20.1.
- RustChunk relevance: Relevant to faithful 1.20.1 client behavior; server authority must remain intact.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Build supported object types and metadata mapping from 1.20.1; exclude later-only objects.
- Follow-up checks: Compare this behavior against exact 1.20.1 source, packet/data fixtures and deterministic regression cases.
- RustChunk implementation mapping: RustChunk 1.20.1 client/game subsystem matching the file purpose.
- Evidence required before adoption: 1.20.1 source/packet fixtures, deterministic tests, multiplayer traces and Android validation where applicable.
- Completion state: COMPLETE — all 445 logical lines read through EOF in the listed coverage; pinned SHA and 18,942 byte size match; embedded tests inspected only, not run.
