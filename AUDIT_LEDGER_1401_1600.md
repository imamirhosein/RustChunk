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

### 1426. `crates/horizon-client/src/game/remote_projectiles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0e3ebe1a088005363877a571f8b153ac1de6ee7b`
- Size: 40,702 bytes; 986 logical lines
- Coverage: 1–400, 401–800, 801–986 (contiguous to EOF)
- Purpose: Creates, updates, interpolates and removes server-spawned projectiles, including owner/velocity data and special projectile behavior.
- Dependencies / cross-references: Spawn and entity metadata packets; projectile/item registries; owners and entity IDs; world collision; particles, sounds and renderer.
- Implementation / logic: Maps spawn data to projectile models, updates positions/rotations and owner state, handles projectile-specific metadata and removes expired entities.
- Findings: Core projectile families are shared with 1.20.1, but later entity types and spawn metadata are included; packet IDs and serialized fields are version-bound.
- RustChunk relevance: Essential for faithful multiplayer combat and projectile visuals; incorrect spawn decoding or interpolation is visible and may break hit timing.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild the projectile type map and packet layouts from 1.20.1; preserve server authority for impacts/damage and distinguish arrows, tridents, potions, fireballs, eggs, snowballs and other supported types.
- Follow-up checks: Compare every 1.20.1 projectile spawn/data packet, owner resolution, velocity/interpolation, removal, potion payload and dimension cleanup.
- RustChunk implementation mapping: RustChunk remote projectile registry, packet decoder, interpolation and render-state pipeline.
- Evidence required before adoption: Captured 1.20.1 spawn/entity-data packet fixtures, per-projectile decode tests, interpolation traces and multiplayer visual regression.
- Completion state: COMPLETE — 986 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 40,702 byte size match; embedded tests, where present, inspected only and not run.

### 1427. `crates/horizon-client/src/game/remote_stations.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5c532f7f5568c5769e41d7a9bbc34c1ac3b1dd53`
- Size: 6,173 bytes; 156 logical lines
- Coverage: 1–156 (EOF)
- Purpose: Routes interactions with workstation/station entities and their associated client menu or screen state.
- Dependencies / cross-references: Block/entity hit results; player hand and inventory; station block entities; menu/screen registry; client interaction packets.
- Implementation / logic: Identifies supported station targets and maps the interaction to the matching menu/state transition, preserving slot and hand context.
- Findings: Station identifiers and menu set are later-version-specific; do not assume every station represented here exists in 1.20.1.
- RustChunk relevance: Useful for correct workstation UI and interaction routing, but not central to connection or chunk delivery.
- Decision: adaptable; rewrite required; 1.20.1
- 1.20.1 implications: Retain only workstation types present in 1.20.1 and map their menu IDs, slots and open-menu packets to the exact target protocol.
- Follow-up checks: Compare 1.20.1 crafting, furnace, anvil, smithing, loom, cartography, stonecutter and merchant screen-open paths; exclude later stations.
- RustChunk implementation mapping: RustChunk workstation interaction router and menu-opening adapter.
- Evidence required before adoption: 1.20.1 menu registry mapping, open-screen packet fixtures, slot-layout tests and touch/UI tests.
- Completion state: COMPLETE — 156 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 6,173 byte size match; embedded tests, where present, inspected only and not run.

### 1428. `crates/horizon-client/src/game/remote_vehicles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b89ec0249fe2358df0d18c73c9f3e56a4da3d9cd`
- Size: 25,211 bytes; 591 logical lines
- Coverage: 1–400, 401–591 (contiguous to EOF)
- Purpose: Maintains remote vehicles and controllable mounts, their synchronized state, passengers, steering and movement.
- Dependencies / cross-references: Vehicle spawn/data packets; boats and minecarts; horse-family and other rideable mob metadata; passenger links; item/control inputs; world physics.
- Implementation / logic: Creates the remote vehicle model, applies server metadata, interpolates movement, updates steering/control state and keeps rider/passenger relationships coherent.
- Findings: Contains later additions such as Happy Ghast and later vehicle metadata; these must not leak into the 1.20.1 supported-type table.
- RustChunk relevance: Essential for multiplayer riding, boats, minecarts and mounted travel; incorrect passenger state can detach players visually.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild supported vehicle types, steering fields, spawn data and passenger semantics from 1.20.1; omit Happy Ghast and every later-only type.
- Follow-up checks: Check boat variants, chest boats if supported by exact 1.20.1, minecart variants, horse/donkey/mule/llama control, passenger ordering and dismount.
- RustChunk implementation mapping: RustChunk remote vehicle registry, synchronized state adapter and rider-control path.
- Evidence required before adoption: 1.20.1 spawn/metadata/passenger fixtures, deterministic interpolation tests and multiplayer mount/dismount captures.
- Completion state: COMPLETE — 591 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 25,211 byte size match; embedded tests, where present, inspected only and not run.

### 1429. `crates/horizon-client/src/game/remote.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `00c0c773f3169b9727a2b34f8a7df9ffac2a817a`
- Size: 527,736 bytes; 12,042 logical lines
- Coverage: 1–400 through 11601–12000 in consecutive 400-line chunks; 12001–12042 (contiguous to EOF)
- Purpose: Central remote multiplayer session implementation: consumes server play packets, updates client world/player/UI state and sends client actions and acknowledgements.
- Dependencies / cross-references: Protocol play packet enums/codecs; connection/session lifecycle; chunk/light decoder; entities, players, inventories and menus; chat/signatures; scoreboard; resources, skins, commands, settings and renderer/game state.
- Implementation / logic: Dispatches serverbound and clientbound play messages, updates authoritative snapshots and local predictions, routes inventory/menu/entity/chat/scoreboard updates, handles teleports/disconnects and queues relevant world/render work.
- Findings: This is the largest protocol integration point and contains later packet variants and 26.3-era behavior. Packet IDs, serializers, registry data, chat signatures and entity metadata must be version-pinned; a partial transplant is unsafe.
- RustChunk relevance: Critical to joining and remaining synchronized with Aternos/Forge servers. It is also the integration point for chunk arrival latency and visible stale state.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Construct the exact 1.20.1 play-state packet matrix and codecs; separate protocol decoding from render-thread application; preserve ordered chunk/entity updates and server authority; explicitly model offline unsigned chat and unknown packet handling.
- Follow-up checks: Build a complete 1.20.1 clientbound/serverbound packet inventory, compare state transitions, verify every packet family and ensure disconnect/dimension/profile changes cancel queued work safely.
- RustChunk implementation mapping: RustChunk 1.20.1 multiplayer session state machine, packet router, action encoder and synchronized game-state dispatcher.
- Evidence required before adoption: Official 1.20.1 protocol/source matrix, captured packet fixtures, decoder/encoder round trips, connection integration tests against a 1.20.1 server and chunk-latency/frame-time traces.
- Completion state: COMPLETE — 12,042 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 527,736 byte size match; embedded tests, where present, inspected only and not run.

### 1430. `crates/horizon-client/src/game/render_control.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2e996f8c8acf997b709a611e6785acfdef99a323`
- Size: 8,616 bytes; 184 logical lines
- Coverage: 1–184 (EOF)
- Purpose: Provides developer/control-socket commands for rendering experiments, including world edits, particles, sign editing and renderer toggles/A-B tests.
- Dependencies / cross-references: Control socket/parser; local world/block APIs; particles; sign editing; settings and renderer state.
- Implementation / logic: Parses authorized control commands and applies test-only changes or toggles used to compare rendering paths and collect visual/debug results.
- Findings: Developer instrumentation, not normal user gameplay. A remotely reachable control socket would require strict access control.
- RustChunk relevance: Optional debugging aid; no baseline client requirement.
- Decision: optional; developer diagnostics only
- 1.20.1 implications: Keep out of release builds unless explicitly gated; ensure any control socket is loopback-only or authenticated and cannot be reached by untrusted servers.
- Follow-up checks: Check build flags, socket bind address, command authorization, destructive command confirmation and release APK exclusion.
- RustChunk implementation mapping: Optional RustChunk developer diagnostics/control interface.
- Evidence required before adoption: Threat model, loopback/authentication tests, release-build symbol/config audit and explicit user opt-in.
- Completion state: COMPLETE — 184 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 8,616 byte size match; embedded tests, where present, inspected only and not run.

### 1431. `crates/horizon-client/src/game/render_tick.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fb7bd252fdfb2133dd53200ec7f1b66ceebf5d7e`
- Size: 27,298 bytes; 661 logical lines
- Coverage: 1–400, 401–661 (contiguous to EOF)
- Purpose: Advances and prepares per-frame visual effects such as fog, weather, rain splashes, particles and environment animation.
- Dependencies / cross-references: World weather/time; camera and dimension; particles; entity animation state; renderer and frame timing.
- Implementation / logic: Updates visual timers, computes camera/environment effects and spawns/draws particles and effects according to world state and elapsed ticks.
- Findings: Visual timing and particle conditions are version-sensitive; frame-driven updates must not accidentally become dependent on unstable render cadence.
- RustChunk relevance: Important for visual fidelity, but separate from server-authoritative simulation and networking.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Match 1.20.1 weather, fog, camera and particle triggers; keep simulation tick effects deterministic and interpolate only presentation values on frames.
- Follow-up checks: Compare rain/splash, fog, dimension effects, particle counts, pause behavior and low-FPS frame pacing with 1.20.1.
- RustChunk implementation mapping: RustChunk environment presentation tick and particle scheduling/render adapter.
- Evidence required before adoption: 1.20.1 source comparison, deterministic tick fixtures, frame captures and mobile frame-time tests.
- Completion state: COMPLETE — 661 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 27,298 byte size match; embedded tests, where present, inspected only and not run.

### 1432. `crates/horizon-client/src/game/repro.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b5eba648351c8c7ef0a350676286d5836b9182a`
- Size: 71,960 bytes; 1,972 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1972 (contiguous to EOF)
- Purpose: Records and exports reproducible game sessions, input/state snapshots and diagnostic events for later replay and bug investigation.
- Dependencies / cross-references: Input/event capture; world/player/entity snapshots; ZIP/serialization; diagnostics; replay command dispatcher and sensitive-data filtering.
- Implementation / logic: Captures timeline records, packages replay/diagnostic data, supports a restricted replay command set and filters personal/server/world details before export.
- Findings: Valuable reliability tooling. The allowed-command list, export sanitization and archive limits are security boundaries and must be retained or strengthened; embedded tests inspected, not run.
- RustChunk relevance: Optional diagnostic subsystem with high value for diagnosing chunk stalls, crashes and state divergence.
- Decision: optional; reference; diagnostics
- 1.20.1 implications: Design a RustChunk-specific format; preserve allowlisted commands, path/symlink safety, bounded archive extraction, secret scrubbing and user consent before sharing.
- Follow-up checks: Threat-test malicious archives, oversized files, path traversal, symlink entries, credentials/tokens, server addresses and determinism across 1.20.1 replay.
- RustChunk implementation mapping: Optional RustChunk replay/diagnostics recorder and secure export pipeline.
- Evidence required before adoption: Documented format, red-team archive fixtures, privacy review, deterministic playback tests and explicit user-consent flow.
- Completion state: COMPLETE — 1,972 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 71,960 byte size match; embedded tests, where present, inspected only and not run.

### 1433. `crates/horizon-client/src/game/respawn_anchor.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bc1bf6e97c3c0613bba4e1271a7c74eb6beaca03`
- Size: 26,150 bytes; 686 logical lines
- Coverage: 1–400, 401–686 (contiguous to EOF)
- Purpose: Implements respawn-anchor charging/use, dimension restrictions, explosion behavior and player respawn-point resolution.
- Dependencies / cross-references: Player interaction and inventory; respawn anchor block state/charge; dimension; world collision and spawn search; sound/events and death/respawn state.
- Implementation / logic: Consumes glowstone charges, sets or triggers anchor behavior, rejects unsafe dimension use with explosion behavior and resolves safe respawn positions and related player movement.
- Findings: Respawn anchors and their core rules exist in 1.20.1, but helper APIs and later respawn configuration details need mapping.
- RustChunk relevance: Essential survival gameplay and correct death/respawn behavior.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Use exact 1.20.1 charge cap, interaction order, explosion/fire behavior, respawn position search and player spawn packet flow.
- Follow-up checks: Compare charge levels, creative/survival consumption, Overworld/End explosion, obstruction search, spawn orientation and multiplayer respawn packets.
- RustChunk implementation mapping: RustChunk respawn-anchor interaction, block-state updates and safe respawn resolver.
- Evidence required before adoption: 1.20.1 source mapping, deterministic anchor-state tests, explosion fixtures and respawn packet integration tests.
- Completion state: COMPLETE — 686 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 26,150 byte size match; embedded tests, where present, inspected only and not run.

### 1434. `crates/horizon-client/src/game/saves_control.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `869bfd709c5891d05dbe3c1ac0802689d2ae765e`
- Size: 5,377 bytes; 124 logical lines
- Coverage: 1–124 (EOF)
- Purpose: Exposes control-socket commands to inspect, rename, back up and delete saved worlds and list multiplayer servers.
- Dependencies / cross-references: Control socket; title-screen world management; save folders and ZIP backups; multiplayer servers.dat; asynchronous reply channel.
- Implementation / logic: Lists world metadata and server entries; requires title screen for rename/delete/backup; requires an explicit --confirm for irreversible deletion; runs long backup work off the game thread.
- Findings: Powerful local administrative commands, not normal game UI. Deletion confirmation and socket exposure are security-sensitive.
- RustChunk relevance: Optional developer/admin utility; not required for playing on external servers.
- Decision: optional; developer diagnostics only
- 1.20.1 implications: Keep outside standard release functionality unless authenticated and explicit; preserve deletion confirmation and perform backup work off the render/game thread.
- Follow-up checks: Check socket authentication/reachability, world path validation, confirmation semantics, backup ZIP safety and concurrent save handling.
- RustChunk implementation mapping: Optional RustChunk local world-management diagnostics interface.
- Evidence required before adoption: Threat model, path traversal tests, destructive-action confirmation tests, archive integrity checks and release-build gating.
- Completion state: COMPLETE — 124 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5,377 byte size match; embedded tests, where present, inspected only and not run.

### 1435. `crates/horizon-client/src/game/saving.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cbdff8333aba9323615a11bde6b5bda1a8de2cc6`
- Size: 14,171 bytes; 355 logical lines
- Coverage: 1–355 (EOF)
- Purpose: Coordinates orderly world save and shutdown, incremental chunk/entity snapshots, writer completion and Android background-save behavior.
- Dependencies / cross-references: SaveDir and region writers; chunk streaming/generation; entity snapshots; raids/attachments/forced chunks; integrated server; event loop and Android focus callbacks.
- Implementation / logic: Freezes or pauses simulation as appropriate, snapshots bounded batches per frame, queues writes, waits for durable completion before closing and triggers a nonblocking background save when Android focus is lost.
- Findings: Strong mobile reliability design: bounded work avoids a long main-thread stall, and durable completion is distinguished from queued snapshots. Save error ownership is retained rather than falsely declaring success.
- RustChunk relevance: Critical for preventing world loss and Android process-kill corruption; also relevant to smooth exit and reduced frame stalls.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Rebuild save ordering and data schema from 1.20.1 worlds/region/NBT while retaining bounded snapshot work, safe writer shutdown, pause semantics and Android lifecycle handling.
- Follow-up checks: Test force-stop/background transitions, app focus loss, partially generated chunks, writer failure, quit during save, integrated/remote sessions and crash recovery.
- RustChunk implementation mapping: RustChunk incremental save coordinator, background flush worker and Android lifecycle integration.
- Evidence required before adoption: Crash/force-stop recovery fixtures, write-order tests, region/NBT validation, Android lifecycle stress and measured frame-time/save-latency data.
- Completion state: COMPLETE — 355 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 14,171 byte size match; embedded tests, where present, inspected only and not run.

### 1436. `crates/horizon-client/src/game/saws.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ec5a29c5739a16714b95b9600032c1e4e2e322ae`
- Size: 25,806 bytes; 722 logical lines
- Coverage: 1–400, 401–722 (contiguous to EOF)
- Purpose: Implements Create 6.0.10 saw-related processing and machine interactions in Starling's Rust-native mod simulation.
- Dependencies / cross-references: Native Create machine/entity state; kinetics; item processing/recipes; inventories and block interactions.
- Implementation / logic: Updates saw machine behavior, handles its inputs/outputs and related item/world interactions through the bundled native implementation.
- Findings: This is a native simulation of a particular Create release, not a Java Forge loader. Create 0.5.x for Minecraft 1.20.1 differs from Create 6.0.10.
- RustChunk relevance: Not required to connect to Forge 1.20.1 servers. May be a reference only if a separately specified native Create bridge is considered.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Do not port this as Forge compatibility; inspect the actual Create 0.5.x/1.20.1 mod implementation and packet/block-entity behavior if selected.
- Follow-up checks: No baseline port; verify the target Create 1.20.1 recipes, kinetic contracts, block entities and network state separately.
- RustChunk implementation mapping: No baseline mapping; possible isolated future native-mod experiment only.
- Evidence required before adoption: Exact 1.20.1 Create source/version, registry and recipe maps, packet/state fixtures, license review and native bridge specification.
- Completion state: COMPLETE — 722 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 25,806 byte size match; embedded tests, where present, inspected only and not run.

### 1437. `crates/horizon-client/src/game/scoreboard.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0fb922cb07b8f2f9c845abc6cdea080607f08fee`
- Size: 42,004 bytes; 1,069 logical lines
- Coverage: 1–400, 401–800, 801–1069 (contiguous to EOF)
- Purpose: Maintains scoreboard objectives, scores, teams and display slots for player list, sidebar and below-name views.
- Dependencies / cross-references: Scoreboard packets; objective/team/score IDs; text components; player names; display slots; HUD and list rendering.
- Implementation / logic: Applies objective creation/removal/update, score changes, team membership/visibility/collision/name formatting and renders the selected displays.
- Findings: Scoreboard protocol and text component shapes are version-sensitive. The later source may include data absent in 1.20.1.
- RustChunk relevance: Essential multiplayer UI for Aternos minigames, teams, objectives and server-provided status.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild packet handlers, objective/display enums, team flags and text component codecs using 1.20.1 protocol definitions.
- Follow-up checks: Compare all scoreboard packet variants, team join/leave, score reset, display-slot changes, formatting, hidden entries and player-list interaction.
- RustChunk implementation mapping: RustChunk scoreboard state store, packet adapter and HUD/player-list rendering.
- Evidence required before adoption: 1.20.1 packet fixtures and round-trip tests, objective/team lifecycle tests and captures from representative servers.
- Completion state: COMPLETE — 1,069 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 42,004 byte size match; embedded tests, where present, inspected only and not run.

### 1438. `crates/horizon-client/src/game/screen_info.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fb874d9df5912e2d0f5d4befd253caf38b0107bb`
- Size: 6,904 bytes; 193 logical lines
- Coverage: 1–193 (EOF)
- Purpose: Builds reusable screen information, labels, prompts and navigation metadata for game UI screens.
- Dependencies / cross-references: UI widgets; text components/localization; screen IDs; modal/confirmation state and navigation actions.
- Implementation / logic: Creates consistent screen descriptors and helper structures for titles, messages, buttons and navigation/confirmation presentation.
- Findings: UI shape is broadly reusable, but wording, screen IDs, navigation and widget contracts are client-version/platform-specific.
- RustChunk relevance: Useful shared UI foundation, especially for touch-friendly mobile presentation.
- Decision: adaptable; rewrite required; UI framework
- 1.20.1 implications: Implement against RustChunk's UI system and 1.20.1 screen/menu behavior; do not assume the current Horizon widget APIs exist.
- Follow-up checks: Check back/escape semantics, modal dismissal, focus, localization, keyboard/gamepad and Android touch layout.
- RustChunk implementation mapping: RustChunk shared screen metadata and prompt/navigation helpers.
- Evidence required before adoption: UI state tests, localization fixtures, keyboard/touch accessibility checks and screen transition captures.
- Completion state: COMPLETE — 193 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 6,904 byte size match; embedded tests, where present, inspected only and not run.

### 1439. `crates/horizon-client/src/game/screens.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1dcd8774a70a6991e370e3947b8720f656743438`
- Size: 36,940 bytes; 965 logical lines
- Coverage: 1–400, 401–800, 801–965 (contiguous to EOF)
- Purpose: Coordinates game screens, menu transitions, shared input handling and drawing/actions for inventory, containers and other screens.
- Dependencies / cross-references: Menu and screen registry; widgets; inventory/container state; input focus; text and localization; HUD/renderer.
- Implementation / logic: Routes open/close/change-screen events, updates screen-specific state and forwards drawing and input actions to the selected UI.
- Findings: Core screen dispatch is valuable, but its menu registry and screen IDs reflect the source version; later menus and display components must be excluded.
- RustChunk relevance: Essential client usability for inventory, crafting, containers and options; needs an Android-first UI adaptation.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Rebuild menu type IDs and screen flows from 1.20.1; preserve server-driven open/close semantics and safe slot/action handling.
- Follow-up checks: Compare inventory, crafting, furnace, anvil, smithing, merchant, book, sign and container screens plus touch/back behavior.
- RustChunk implementation mapping: RustChunk screen state machine, menu dispatcher and Android touch UI.
- Evidence required before adoption: 1.20.1 menu packet fixtures, slot/action integration tests, visual regression and touch-input tests.
- Completion state: COMPLETE — 965 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 36,940 byte size match; embedded tests, where present, inspected only and not run.

### 1440. `crates/horizon-client/src/game/sculk_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cf64d369f86cab39ddfb60feef985b8b051d8071`
- Size: 27,969 bytes; 715 logical lines
- Coverage: 1–400, 401–715 (contiguous to EOF)
- Purpose: Implements sculk-related fixture/block behaviors, including sensor, shrieker/catalyst and vibration-sensitive environmental interactions.
- Dependencies / cross-references: Sculk block states; vibration/game-event system; redstone signals; entity/player movement; Warden and world event scheduling.
- Implementation / logic: Resolves sculk fixture triggers and state transitions, consumes game events and updates connected block/AI effects.
- Findings: Core sculk features exist in 1.20.1, but later states/tags and event APIs must be separated from the target baseline.
- RustChunk relevance: Essential for Deep Dark/Ancient City gameplay and redstone/vibration correctness.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Map only 1.20.1 sensor/shrieker/catalyst behavior, cooldowns, redstone output, event tags and Warden summon rules; exclude later additions.
- Follow-up checks: Compare calibrated sensor frequencies, wool occlusion, vibration range/cooldowns, shrieker warning/anger and catalyst sculk spread to 1.20.1.
- RustChunk implementation mapping: RustChunk sculk fixture state machine and vibration event integration.
- Evidence required before adoption: 1.20.1 source/data fixtures, deterministic event-sequence tests, redstone output tests and multiplayer Warden traces.
- Completion state: COMPLETE — 715 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 27,969 byte size match; embedded tests, where present, inspected only and not run.

### 1441. `crates/horizon-client/src/game/sculk.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b1b66e5712de2c28811d3796a3040ff9a00060eb`
- Size: 79,276 bytes; 2,031 logical lines
- Coverage: 1–400, 401–800; 801–1200 and 1201–1600 exact slices from complete pinned blob after ambiguous repeated range; 1601–2000; 2001–2031 (contiguous to EOF)
- Purpose: Implements sculk vibration listeners, calibrated sensors, shriekers, catalysts, Warden-related tracking, darkness and vibration-driven world/player events.
- Dependencies / cross-references: Game-event registry; vibration frequencies and listeners; sculk block entities; entity movement/pose; Warden AI; effects, particles, redstone and world scheduling.
- Implementation / logic: Collects and filters game events, routes vibration signals by frequency/range/occlusion, updates sensors/shriekers/catalysts, emits client/world effects and maintains Warden-related state.
- Findings: Large, version-sensitive subsystem. Ranged API returned a repeated segment at 801–1200; exact slices from the complete pinned blob replaced the ambiguous ranges and ensure full 2,031-line coverage. Embedded tests inspected, not run.
- RustChunk relevance: Essential to 1.20.1 Deep Dark, Warden and vibration gameplay; needs exact target game-event behavior.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Recreate 1.20.1 game-event IDs, vibration occlusion/range, wool dampening, sensor cooldowns, shrieker warning/anger and Warden spawn rules; exclude later-only event IDs.
- Follow-up checks: Compare every 1.20.1 event source and frequency, projectile/item/step/elytra triggers, wool behavior, calibrated sensor output, shrieker state and Warden anger.
- RustChunk implementation mapping: RustChunk game-event/vibration router and sculk block/entity state machines.
- Evidence required before adoption: 1.20.1 source/data mapping, deterministic vibration tests, occlusion fixtures, block-state/redstone tests and multiplayer Warden behavior traces.
- Completion state: COMPLETE — 2,031 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 79,276 byte size match; embedded tests, where present, inspected only and not run.

### 1442. `crates/horizon-client/src/game/server_links.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fbb637d8c93cf64a7e6c33dbfaf470e35396327e`
- Size: 21,382 bytes; 579 logical lines
- Coverage: 1–400, 401–579 (contiguous to EOF)
- Purpose: Receives and presents server-provided external links with user confirmation, validation and open/copy/disconnect actions.
- Dependencies / cross-references: Server link packet/data; URI parser; UI confirmation; platform browser and clipboard APIs; connection/session controls.
- Implementation / logic: Validates permitted link schemes, builds a confirmation UI and routes accepted links to open/copy actions while supporting server disconnect behavior.
- Findings: Server Links as represented here is a later protocol feature and not part of the Minecraft 1.20.1 baseline. Scheme validation and explicit consent are important security patterns.
- RustChunk relevance: No 1.20.1 baseline requirement; possible future protocol feature.
- Decision: unnecessary; version mismatch; future
- 1.20.1 implications: Exclude the server-links packet path from the 1.20.1 client; do not invent packet IDs or expect this screen from target servers.
- Follow-up checks: No baseline task; if later supported, compare the exact protocol version and preserve strict http/https allowlisting and consent.
- RustChunk implementation mapping: None for baseline; possible future safe external-link prompt.
- Evidence required before adoption: Protocol version proving support, URI fuzz tests, unsafe-scheme rejection and explicit user-consent/security review.
- Completion state: COMPLETE — 579 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 21,382 byte size match; embedded tests, where present, inspected only and not run.

### 1443. `crates/horizon-client/src/game/server_packs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `869bd88a2af6ab99ac6112162d86d2f6e4c0464d`
- Size: 25,453 bytes; 654 logical lines
- Coverage: 1–400, 401–654 (contiguous to EOF)
- Purpose: Handles server resource-pack pushes/removals, consent prompts, download progress, reload and failure notifications.
- Dependencies / cross-references: Server resource-pack packets; download queue; pack SHA/hash and URL; user settings/consent; resource manager/reload worker; toast UI.
- Implementation / logic: Interprets pack requests and remembered choices, starts asynchronous downloads, stages pack reloads, tracks failures and notifies the user of progress/results.
- Findings: Server resource packs existed in 1.20.1, but packet fields/status codes and pack prompt semantics must exactly match that protocol version.
- RustChunk relevance: Essential for many modded/community servers that require or offer server resource packs.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Implement 1.20.1 resource-pack send/remove packets and status responses; preserve consent policy, hash verification, asynchronous reload and failure reporting.
- Follow-up checks: Test required/optional packs, accept/decline, invalid URL/hash, download failure, reload failure/recovery, removal and disconnect during download.
- RustChunk implementation mapping: RustChunk server-pack state machine, download worker, consent UI and resource reload coordinator.
- Evidence required before adoption: 1.20.1 packet fixtures, hash verification tests, failure/recovery tests, malicious-pack validation and Android storage/reload tests.
- Completion state: COMPLETE — 654 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 25,453 byte size match; embedded tests, where present, inspected only and not run.

### 1444. `crates/horizon-client/src/game/server_travel.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e608146ac5a1b0ff93221f6f225b2e5c2180f147`
- Size: 38,403 bytes; 958 logical lines
- Coverage: 1–400, 401–800, 801–958 (contiguous to EOF)
- Purpose: Implements server-side player trips between dimensions and portal/gateway/respawn arrival handling for Starling's LAN-hosted world.
- Dependencies / cross-references: LAN host/player guest state; dimension levels; Nether portal plans; End gateway jobs; respawn config; chunk streaming/readiness; vehicles/passengers; native mod callbacks.
- Implementation / logic: Moves players into destination levels, waits for surrounding chunks, resolves portal/gateway/respawn exits, transports rideable vehicles and sends dimension/teleport notifications.
- Findings: This is primarily an integrated/LAN server implementation, not the external remote-client teleport packet handler. It includes Starling native-mod callbacks.
- RustChunk relevance: Not required for a client that only connects to Aternos, but a useful future reference if RustChunk hosts local worlds or needs portal simulation.
- Decision: optional; reference; LAN host
- 1.20.1 implications: Keep out of the standalone external-server client baseline; if local hosting is built, reimplement 1.20.1 server-side portal/respawn rules and keep server authority.
- Follow-up checks: Compare 1.20.1 dimension-change order, Nether coordinate scaling, End/gateway destinations, vehicle passengers, chunk readiness and respawn search.
- RustChunk implementation mapping: No baseline mapping; possible RustChunk integrated-server travel and respawn subsystem.
- Evidence required before adoption: Defined LAN-host scope, 1.20.1 server source mapping, dimension/portal integration tests and packet traces.
- Completion state: COMPLETE — 958 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 38,403 byte size match; embedded tests, where present, inspected only and not run.

### 1445. `crates/horizon-client/src/game/setup.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f608d17a8ebbebb3f7981e86072d1531d1bb1309`
- Size: 39,516 bytes; 1,031 logical lines
- Coverage: 1–400, 401–800, 801–1031 (contiguous to EOF)
- Purpose: Implements Android game setup/onboarding: account login or offline setup, game-file/Java preparation, imports and lightweight progress UI.
- Dependencies / cross-references: Account provider APIs; Microsoft/offline/Ely authentication; official game files and Java preparer; Android storage picker/import; UI, diagnostics and fishing mini-game.
- Implementation / logic: Models setup phases, polls sign-in/download/preparation, renders progress and confirmation screens, handles Android storage permissions/import and retries failures.
- Findings: Launcher-specific onboarding, not Minecraft 1.20.1 game protocol. Offline profile flow does not create signed chat identity for servers requiring secure profiles.
- RustChunk relevance: Useful launcher feature for Android, but should be independently designed around RustChunk packaging, storage and account policy.
- Decision: optional; adapt; Android launcher
- 1.20.1 implications: Implement a RustChunk-specific onboarding flow; preserve clear offline limitations, secure credential handling and Android scoped-storage behavior. Do not assume Java/asset installation flow transfers unchanged.
- Follow-up checks: Validate Microsoft device-code flow, offline profile limits, Android 13 storage/import, interrupted downloads, Java compatibility and user-visible recovery.
- RustChunk implementation mapping: RustChunk Android setup state machine and account/resource preparation UI.
- Evidence required before adoption: Android device tests, account provider security review, interrupted-download recovery, scoped-storage tests and clear offline/server-security messaging.
- Completion state: COMPLETE — 1,031 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 39,516 byte size match; embedded tests, where present, inspected only and not run.

### 1446. `crates/horizon-client/src/game/setup/account_form.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ba438a48a6ff11bb3a4723767b6a277b8b750444`
- Size: 11,594 bytes; 353 logical lines
- Coverage: 1–353 (EOF)
- Purpose: Provides account/offline-profile input forms, focus, keyboard/IME synchronization, password masking and submit handling.
- Dependencies / cross-references: Account providers; text EditBox; Android text mirror/IME; clipboard; account form UI and authentication state.
- Implementation / logic: Handles field limits, focus generations, text replacement, password/OTP concealment, copy/cut restrictions for secrets and login/offline profile submission.
- Findings: Security-sensitive credential entry. Generation checks prevent stale IME updates from changing another field; password copy/cut is blocked, and sensitive buffers should not be logged.
- RustChunk relevance: Useful for an Android launcher, but requires independent provider and credential handling.
- Decision: optional; adapt; Android launcher
- 1.20.1 implications: Retain stale-IME protection and secret handling if reimplemented; use secure storage/provider APIs appropriate to RustChunk and avoid plaintext persistence/logging.
- Follow-up checks: Test stale IME generations, Unicode/cursor limits, password masking, paste/copy/cut, OTP, field clearing and authentication failure.
- RustChunk implementation mapping: RustChunk Android account form and safe text-input bridge.
- Evidence required before adoption: IME race tests, credential memory/log review, account-provider security review and Android keyboard/touch tests.
- Completion state: COMPLETE — 353 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 11,594 byte size match; embedded tests, where present, inspected only and not run.

### 1447. `crates/horizon-client/src/game/setup/fishing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6b4d7b2f10a56a72fec656eaa972aa347243330c`
- Size: 21,877 bytes; 601 logical lines
- Coverage: 1–400, 401–601 (contiguous to EOF)
- Purpose: Implements an optional fishing mini-game shown during launcher setup waits, with water/fish animation, catch timing and generated short sound effects.
- Dependencies / cross-references: Setup screen; frame timing; touch press/release; UI drawing primitives; deterministic pseudo-random state and audio sample playback.
- Implementation / logic: Tracks cast, waiting, fish approach, bite and catch/miss phases; draws water, bobber, fish and ripples and synthesizes short sound samples.
- Findings: Purely optional presentation feature, not Minecraft fishing simulation or server gameplay.
- RustChunk relevance: Cosmetic onboarding polish; no requirement for launching or connecting to servers.
- Decision: optional; future polish
- 1.20.1 implications: Do not mix this with in-game fishing mechanics; only port as an isolated setup UI component if it does not delay core launcher work.
- Follow-up checks: No baseline task; verify touch handling, audio lifecycle, frame pacing and accessibility only if retained.
- RustChunk implementation mapping: Optional RustChunk setup mini-game module.
- Evidence required before adoption: Touch-state tests, audio bounds/clipping checks, low-end Android frame-time tests and accessibility/reduced-motion review.
- Completion state: COMPLETE — 601 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 21,877 byte size match; embedded tests, where present, inspected only and not run.

### 1448. `crates/horizon-client/src/game/shader_packs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `de665bab2ed52f6e972da940e327b35e9a5296c5`
- Size: 47,474 bytes; 1,087 logical lines
- Coverage: 1–400, 401–800, 801–1087 (contiguous to EOF)
- Purpose: Loads and applies external shader packs, parses options/profiles, builds render inputs and entity/block spans, and handles shader failure/device-loss recovery.
- Dependencies / cross-references: Shader-pack parser and API; renderer shader workers; pack properties/options; block/entity registries; lighting/biomes; player status; GPU/device lifecycle and resource packs.
- Implementation / logic: Selects active pack/profile, calculates mobile Auto limits, translates pack properties/macros, builds IDs/uniforms and submits shader compilation asynchronously; disables failed/heavy packs safely.
- Findings: High relevance to GPU cost and mobile stability. Shader-pack formats and APIs are not necessarily compatible with 1.20.1 OptiFine/Iris versions; parsing untrusted packs should be isolated from crashes.
- RustChunk relevance: Optional renderer extension, potentially useful for visual features but not a prerequisite for Forge server connection.
- Decision: optional; reference; Android graphics
- 1.20.1 implications: Target a clearly specified 1.20.1 shader format/version; keep the Android Auto profile, bounded resolution/shadow budgets and crash/device-loss fallback as design references, not direct API reuse.
- Follow-up checks: Test pack parsing, invalid/hostile shaders, per-dimension reload, option persistence, device loss, GPU time/memory, Mali-G52 compatibility and fallback without world crash.
- RustChunk implementation mapping: Optional RustChunk shader-pack loader, renderer uniform adapter and Android adaptive quality controller.
- Evidence required before adoption: Supported shader-format specification, test packs, compiler fuzzing, GPU captures, Mali-G52/Android device runs and recovery tests.
- Completion state: COMPLETE — 1,087 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 47,474 byte size match; embedded tests, where present, inspected only and not run.

### 1449. `crates/horizon-client/src/game/shelf.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `819df24308508bd4064fbf8cb0416ed8179df3b9`
- Size: 10,444 bytes; 260 logical lines
- Coverage: 1–260 (EOF)
- Purpose: Implements wooden shelf block interaction, connected shelf hotbar swaps and rendering of displayed items.
- Dependencies / cross-references: Shelf block/entity state; item stacks and inventory; block tags/properties; hit-slot geometry; game events, sound and item model renderer.
- Implementation / logic: Maps click location to shelf slot, performs single or connected powered shelf swaps and renders item models with facing/alignment transforms.
- Findings: Wooden shelf block/entity behavior is later-version content and not part of the 1.20.1 vanilla baseline.
- RustChunk relevance: No baseline requirement; possible future custom block/mod reference only.
- Decision: unnecessary; version mismatch; future
- 1.20.1 implications: Exclude from 1.20.1 vanilla implementation; a modded shelf requires its own 1.20.1 registry, block-entity and packet specification.
- Follow-up checks: No baseline task; verify target mod and exact block/entity implementation if requested later.
- RustChunk implementation mapping: None for 1.20.1 baseline; isolated future custom block adapter only.
- Evidence required before adoption: Selected 1.20.1 mod source, block-state/slot fixtures, packet mapping and licensing/compatibility review.
- Completion state: COMPLETE — 260 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 10,444 byte size match; embedded tests, where present, inspected only and not run.

### 1450. `crates/horizon-client/src/game/shield.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2e6d89c31aa34f4f21875d86d436d2a456d2acf9`
- Size: 11,715 bytes; 288 logical lines
- Coverage: 1–288 (EOF)
- Purpose: Implements item-based attack blocking, shield raising, directional damage reduction, item wear, attacker knockback and shield disabling.
- Dependencies / cross-references: ItemStack/equipment slots; BlocksAttacks components; damage types/tags; hurt/block outcomes; mob/player combat; item durability and cooldowns.
- Implementation / logic: Resolves active hand and block component, checks delay and attack angle, calculates damage reduction/bypass/pierce, applies durability, sound/knockback and disable cooldown.
- Findings: This uses newer BlocksAttacks data-component behavior and does not directly match 1.20.1 classic shield mechanics. New Warden/Ravager special cases also require exact target-version verification.
- RustChunk relevance: Essential combat interaction, but this implementation cannot be copied into 1.20.1 unchanged.
- Decision: essential; rewrite required; 1.20.1
- 1.20.1 implications: Reimplement 1.20.1 shield use/delay, facing-angle block, damage reduction, axe disable, durability, sound and knockback using the 1.20.1 item and damage model; do not assume BlocksAttacks components.
- Follow-up checks: Compare shield use in each hand, cooldown, axe disable duration, projectile/melee block, damage source bypass tags, durability and attacker knockback against 1.20.1.
- RustChunk implementation mapping: RustChunk 1.20.1 shield blocking and combat outcome pipeline.
- Evidence required before adoption: 1.20.1 source mapping, deterministic combat fixtures, damage-type/tag tests and multiplayer shield packet traces.
- Completion state: COMPLETE — 288 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 11,715 byte size match; embedded tests, where present, inspected only and not run.
