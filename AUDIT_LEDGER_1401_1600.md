# RustChunk Starling Audit Ledger — Entries 1401–1600

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1401–1600 (200 unique source files; one shared Ledger for this stage).
- Current progress: entries 1401–1600 complete (200/200); reporting sections 1401–1450, 1451–1500, 1501–1550 and 1551–1600 are closed.
- Reporting rule: stop at entry 1600 and wait for explicit authorization before inspecting entry 1601 or later.
- Each entry contains the 15 required fields defined in AUDIT_PROTOCOL.md. Purpose is kept separate from dependencies, implementation, findings, relevance, decision, implications, follow-up, mapping and evidence.
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
- Decision: **undetermined; optional; reference; LAN host** — optional; reference; LAN host
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
- Decision: **undetermined; optional; reference; OptiFine compatibility** — optional; reference; OptiFine compatibility
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
- Decision: **undetermined; optional; adapt; entity effects** — optional; adapt; entity effects
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
- Coverage: Complete through EOF: 12042 logical lines; exact consecutive ranges 1–400; 401–800; 801–1200; 1201–1600; 1601–2000; 2001–2400; 2401–2800; 2801–3200; 3201–3600; 3601–4000; 4001–4400; 4401–4800; 4801–5200; 5201–5600; 5601–6000; 6001–6400; 6401–6800; 6801–7200; 7201–7600; 7601–8000; 8001–8400; 8401–8800; 8801–9200; 9201–9600; 9601–10000; 10001–10400; 10401–10800; 10801–11200; 11201–11600; 11601–12000; 12001–12042; no gaps.
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
- Decision: **undetermined; optional; developer diagnostics only** — optional; developer diagnostics only
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
- Decision: **undetermined; optional; reference; diagnostics** — optional; reference; diagnostics
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
- Decision: **undetermined; optional; developer diagnostics only** — optional; developer diagnostics only
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
- Decision: **undetermined; optional; reference; LAN host** — optional; reference; LAN host
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
- Decision: **undetermined; optional; adapt; Android launcher** — optional; adapt; Android launcher
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
- Decision: **undetermined; optional; adapt; Android launcher** — optional; adapt; Android launcher
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
- Decision: **undetermined; optional; future polish** — optional; future polish
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
- Decision: **undetermined; optional; reference; Android graphics** — optional; reference; Android graphics
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


### 1451. `crates/horizon-client/src/game/shoulders.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `28ec3c372aa3deff980921a0e931d14638514c22`
- Size: 7912 bytes; 213 logical lines
- Coverage: 1–213 (EOF)
- Purpose: Parrot shoulder attachment, landing/dismount transitions, shoulder NBT and ambient behavior.
- Dependencies / cross-references: Player pose/movement; parrot state/NBT; collision/landing; sound and random source.
- Implementation / logic: Tracks shoulder occupants, attachment timers and dismount triggers; persists occupant data and emits occasional sounds.
- Findings: Core parrot feature exists in 1.20.1; exact transition order, NBT keys and later additions need target-era parity.
- RustChunk relevance: Vanilla survival gameplay; useful for player/parrot state, not launcher or transport infrastructure.
- Decision: adaptable; version-specific
- 1.20.1 implications: Implement classic 1.20.1 shoulder attachment/dismount rules and legacy entity NBT; do not import newer fields without source proof.
- Follow-up checks: Compare both shoulders, jump/landing, water, sneaking, death, dimension change, logout/reload and persistence.
- RustChunk implementation mapping: RustChunk 1.20.1 player shoulder state and parrot interaction subsystem.
- Evidence required before adoption: 1.20.1 vanilla source/mappings, deterministic player/parrot fixtures, save/reload and multiplayer traces.
- Completion state: COMPLETE — 213 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 7912 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1452. `crates/horizon-client/src/game/sign_edit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `039d9e6dfb0c8089506e941e0988a82e244bd82b`
- Size: 35787 bytes; 903 logical lines
- Coverage: 1–400, 401–800, 801–903 (contiguous to EOF)
- Purpose: Sign front/back editing, text input/filtering, dye/glow controls, orientation and click-event editing for local and hosted players.
- Dependencies / cross-references: Sign block entities/text; permissions; item use; LAN guest state; sign-update packets; EditBox/clipboard.
- Implementation / logic: Maintains side text, filter/style state; handles wax/ink/dye interactions and sends edited text in serverbound packets.
- Findings: Classic editing is 1.20.1 functionality, but show_dialog/custom actions and newer text/component assumptions are later behavior; current packets are not target wire-format evidence.
- RustChunk relevance: Server-visible sign editing/text; important after target text and protocol compatibility are established.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild with 1.20.1 SignBlockEntity NBT/text and UpdateSign semantics; omit later click actions and filtering behavior absent from target sources.
- Follow-up checks: Check front/back packets, line limits, filtered/unfiltered text, wax/ink/dye/glow, rotation, permissions and click events.
- RustChunk implementation mapping: RustChunk 1.20.1 sign text model, editor UI and legacy protocol adapter.
- Evidence required before adoption: 1.20.1 mappings/source, packet byte fixtures, sign NBT round-trip, permission and hostile text tests.
- Completion state: COMPLETE — 903 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 35787 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1453. `crates/horizon-client/src/game/sniffers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e47afb31c1c625101e0902bc4a6c2f6d84fbadbc`
- Size: 4505 bytes; 110 logical lines
- Coverage: 1–110 (EOF)
- Purpose: Sniffer egg hatching, sniffing behavior, particles, sounds and player-facing effects.
- Dependencies / cross-references: Sniffer AI/state; egg timer; world random/block queries; particle/sound system; player proximity.
- Implementation / logic: Advances egg hatch and sniff/dig phases, checks nearby conditions and emits effects.
- Findings: Sniffer and egg exist in 1.20.1; exact ticks and transitions require oracle comparison.
- RustChunk relevance: A vanilla 1.20.1 mob and related gameplay behavior.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 sniffer states, egg timing, AI and archaeology behavior only; exclude later additions.
- Follow-up checks: Verify hatch stages/ticks, support, weather, cooldown, dig duration, drops, unload/reload and authority.
- RustChunk implementation mapping: RustChunk 1.20.1 sniffer entity/egg simulation and effects.
- Evidence required before adoption: 1.20.1 source mapping, fixed-random tick fixtures, save/load and multiplayer traces.
- Completion state: COMPLETE — 110 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 4505 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1454. `crates/horizon-client/src/game/social.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `085472a3abadaee73929c949315f33ecc67b82ed`
- Size: 9573 bytes; 244 logical lines
- Coverage: 1–244 (EOF)
- Purpose: Local social-hide/player-tab state and player session presentation.
- Dependencies / cross-references: Player profiles/UUIDs; server player list; client storage/options; social UI and session lifecycle.
- Implementation / logic: Stores locally hidden player identities and updates player-tab/session state.
- Findings: Client-side account/social UX, not packet interoperability; platform social APIs and persistence are host-specific.
- RustChunk relevance: Optional local social preference; not required for multiplayer connectivity.
- Decision: unnecessary; optional/future
- 1.20.1 implications: Do not port Horizon service assumptions. A future local hide feature must be independent of auth and cannot claim to block server-side chat.
- Follow-up checks: If retained, test UUID changes, offline profiles, duplicate names, reconnect, deletion and privacy-safe persistence.
- RustChunk implementation mapping: Optional local player-hide settings module; no baseline mapping.
- Evidence required before adoption: Explicit UX requirement, privacy/data-retention design and offline/authenticated identity tests.
- Completion state: COMPLETE — 244 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 9573 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1455. `crates/horizon-client/src/game/spawner.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8ae12c3e8306ff8ff35049f2b7dc2dd89e6e7123`
- Size: 23627 bytes; 563 logical lines
- Coverage: 1–400, 401–563 (contiguous to EOF)
- Purpose: Base spawner ticking/spawn attempts, spawn-egg behavior and spawner display animation.
- Dependencies / cross-references: Spawner block entity; mob registry/spawn rules; collision/difficulty; spawn eggs; renderer clock.
- Implementation / logic: Ticks delay/count/range checks, samples positions, validates placement and spawns mobs; renders preview entity.
- Findings: Vanilla BaseSpawner is relevant to 1.20.1. TrialSpawner/Vault paths are not target baseline.
- RustChunk relevance: Core vanilla mob spawning and spawn-egg behavior.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Implement 1.20.1 BaseSpawner and spawn eggs from target mappings; exclude trial spawner/vault and later spawn data.
- Follow-up checks: Compare delay reset, count/range, nearby cap, spawn potentials, light/placement, egg NBT and unload timing.
- RustChunk implementation mapping: RustChunk 1.20.1 spawner tick, spawn-egg use and preview renderer.
- Evidence required before adoption: 1.20.1 vanilla source, deterministic RNG fixtures, spawner NBT and placement tests.
- Completion state: COMPLETE — 563 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 23627 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1456. `crates/horizon-client/src/game/spear.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `24d918718c3b8a68b3352bdf54924183a288c53c`
- Size: 16402 bytes; 402 logical lines
- Coverage: 1–400, 401–402 (contiguous to EOF)
- Purpose: Kinetic spear attacks with speed, piercing and lunge behavior.
- Dependencies / cross-references: Later spear weapon data; attack/damage/knockback; entity collision; item use and durability.
- Implementation / logic: Computes kinetic attack effects, selects targets and applies piercing/lunge damage or movement.
- Findings: The weapon mechanic is later-version content absent from vanilla Minecraft 1.20.1.
- RustChunk relevance: No vanilla 1.20.1 relevance; only potential reference for a specifically selected weapon mod.
- Decision: unnecessary; version-specific
- 1.20.1 implications: Exclude entirely from vanilla 1.20.1; a modded spear needs its own Forge mod specification.
- Follow-up checks: No baseline work unless a named 1.20.1 mod requires this behavior.
- RustChunk implementation mapping: None for vanilla 1.20.1.
- Evidence required before adoption: Specific 1.20.1 mod source, item registration, combat rules and license.
- Completion state: COMPLETE — 402 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 16402 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1457. `crates/horizon-client/src/game/spectator.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9181c033907bde2f0d961fccc3490807e1e5c0c2`
- Size: 4664 bytes; 115 logical lines
- Coverage: 1–115 (EOF)
- Purpose: Spectator HUD/radial selection and teleport-to-player/entity action.
- Dependencies / cross-references: Spectator mode; player list/entity IDs; teleport-spectator packet; wheel/input state.
- Implementation / logic: Builds spectator targets, draws selection UI and sends a target-selection packet.
- Findings: Spectator mode exists in 1.20.1; packet ID/body and target construction must come from its protocol.
- RustChunk relevance: Relevant multiplayer game mode/client interaction; packet compatibility is essential.
- Decision: adaptable; version-specific
- 1.20.1 implications: Recreate target rules/player list and packet layout with 1.20.1 sources; never reuse current IDs.
- Follow-up checks: Check filtering, disconnected/dead targets, ordering, touch-wheel selection, respawn and packet traces.
- RustChunk implementation mapping: RustChunk spectator HUD/input plus versioned teleport-spectator packet.
- Evidence required before adoption: 1.20.1 mappings/packet captures, spectator state and touch interaction tests.
- Completion state: COMPLETE — 115 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 4664 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1458. `crates/horizon-client/src/game/spill.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1cd442effcc4fa8e50526b297dc5cc6345d0191b`
- Size: 33131 bytes; 840 logical lines
- Coverage: 1–400, 401–800, 801–840 (contiguous to EOF)
- Purpose: Block/container destruction and spilling of stored contents and block-entity data into drops.
- Dependencies / cross-references: Block entities/item stacks; loot context/tables; NBT/item serialization; break/explosion; drop entities.
- Implementation / logic: Selects block-entity contents, serializes data into drops and routes destruction output through loot logic.
- Findings: Important 1.20.1 behavior, but this snapshot uses later item components and includes non-target types.
- RustChunk relevance: Important vanilla block/container drops and item-persistence correctness.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Use 1.20.1 legacy item NBT for shulkers, banners, skulls, pots and inventories; do not assume components or later entities.
- Follow-up checks: Verify every target block entity's drops, loot context, explosion, silk touch/creative and NBT round-trip.
- RustChunk implementation mapping: RustChunk 1.20.1 block-break/drop pipeline and legacy BE-to-ItemStack serializer.
- Evidence required before adoption: 1.20.1 mappings, world fixtures, seeded loot cases and item golden files.
- Completion state: COMPLETE — 840 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 33131 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1459. `crates/horizon-client/src/game/splash_water.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `071f6e3e4c25feedd1789809eea642480ea471b4`
- Size: 10992 bytes; 259 logical lines
- Coverage: 1–259 (EOF)
- Purpose: Splash-water effects: extinguishing and water-susceptibility behavior on selected entities.
- Dependencies / cross-references: Splash projectile/effects; fire and entity status; axolotl/armor stand; particles/sounds/world state.
- Implementation / logic: Selects affected targets, applies water-sensitive transitions/extinguishing and emits splash feedback.
- Findings: Vanilla behavior is relevant to 1.20.1; species and tag assumptions need version matching.
- RustChunk relevance: Vanilla potion/entity behavior with moderate gameplay relevance.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port the target-era entity list and exact splash/fire semantics; avoid later species/tags.
- Follow-up checks: Test fire, axolotl, armor stands, radius, line of sight, multiple targets and event ordering.
- RustChunk implementation mapping: RustChunk 1.20.1 splash-potion effect handler and water-susceptibility adapter.
- Evidence required before adoption: 1.20.1 source/mappings, entity fixtures, radius tests and server event traces.
- Completion state: COMPLETE — 259 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 10992 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1460. `crates/horizon-client/src/game/split.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b99c122fb2cc3deb06364124b04c5875a946d079`
- Size: 35013 bytes; 854 logical lines
- Coverage: 1–400, 401–800, 801–854 (contiguous to EOF)
- Purpose: Coordinates multiple local game instances for desktop split-screen rendering, input, compositor and session lifecycle.
- Dependencies / cross-references: Window/compositor; local guests; gamepad routing; multiple renderers and local server/player lifecycle.
- Implementation / logic: Creates/removes local participants, computes viewports, routes input and renders synchronized local game views.
- Findings: Desktop multi-window/local sessions, not external network multiplayer and not needed for Android A22 baseline.
- RustChunk relevance: Desktop local split-screen, unrelated to external server connections.
- Decision: unnecessary; optional/future
- 1.20.1 implications: Exclude; any future mobile multi-instance feature needs Android-specific lifecycle and memory design.
- Follow-up checks: No baseline follow-up; future work needs surface lifecycle, memory, touch focus and state isolation.
- RustChunk implementation mapping: None for Android 1.20.1 baseline.
- Evidence required before adoption: Explicit multi-instance product requirement and Android resource/lifecycle design.
- Completion state: COMPLETE — 854 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 35013 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1461. `crates/horizon-client/src/game/split/frames.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a5f4ba67c24b7313d66f1eb8d16f8f6f8683d989`
- Size: 2027 bytes; 44 logical lines
- Coverage: 1–44 (EOF)
- Purpose: Split-screen frame diagnostics and viewport bookkeeping.
- Dependencies / cross-references: Split-screen compositor/frame state and debug presentation.
- Implementation / logic: Tracks/displays frame diagnostics for split-screen renderer.
- Findings: Desktop split-screen-only, no standalone 1.20.1 gameplay role.
- RustChunk relevance: Diagnostics tied exclusively to desktop split-screen.
- Decision: unnecessary; desktop-only
- 1.20.1 implications: Do not port to Android baseline.
- Follow-up checks: No baseline follow-up; only consider with independently scoped split-screen system.
- RustChunk implementation mapping: None.
- Evidence required before adoption: Approved split-screen feature and Android diagnostics design.
- Completion state: COMPLETE — 44 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 2027 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1462. `crates/horizon-client/src/game/split/hud.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4519a4b20ee2a819bbc5e43cee68c0cd8c2cfc7d`
- Size: 9258 bytes; 231 logical lines
- Coverage: 1–231 (EOF)
- Purpose: Split-screen player badges and controller prompts.
- Dependencies / cross-references: Local guest/player list; controller state; HUD sprites/widgets.
- Implementation / logic: Draws per-viewport player labels and context-sensitive gamepad overlays.
- Findings: Only meaningful for multiple local players and controller navigation.
- RustChunk relevance: Desktop split-screen/controller HUD, not Android touch baseline.
- Decision: unnecessary; desktop-only
- 1.20.1 implications: Exclude; implement touch affordances in a separate mobile control layer.
- Follow-up checks: No baseline follow-up; future controller support needs Android input and coexistence tests.
- RustChunk implementation mapping: None now; optional controller HUD later.
- Evidence required before adoption: Approved controller UX specification, supported assets and Android controller tests.
- Completion state: COMPLETE — 231 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 9258 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1463. `crates/horizon-client/src/game/split/pads.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `629953f976e32b32fbaddc51df8bb82dddf23c17`
- Size: 21882 bytes; 593 logical lines
- Coverage: 1–400, 401–593 (contiguous to EOF)
- Purpose: Split-screen controller roster, assignment and disconnect/reconnect management.
- Dependencies / cross-references: Desktop gamepad backends; local participants; input polling and player state.
- Implementation / logic: Enumerates/assigns controllers and handles joining/leaving and reconnects.
- Findings: Desktop controller model, not Android single-session touch controls.
- RustChunk relevance: Desktop controller assignment, not Android device input.
- Decision: unnecessary; desktop-only
- 1.20.1 implications: Do not port desktop discovery; future Android controller code must use Android device IDs/lifecycle.
- Follow-up checks: Future: test detach/reattach, multiple pads, vibration and touch focus.
- RustChunk implementation mapping: None for baseline.
- Evidence required before adoption: Android controller request and device-level validation.
- Completion state: COMPLETE — 593 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 21882 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1464. `crates/horizon-client/src/game/split/players.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `10cae54c917873fc004c2ab99490dd63bfda8d58`
- Size: 16857 bytes; 424 logical lines
- Coverage: 1–400, 401–424 (contiguous to EOF)
- Purpose: Local split-screen guest/player profile and shared session management.
- Dependencies / cross-references: Split-screen coordinator; profiles; local server guest records; renderer/session ownership.
- Implementation / logic: Creates/removes local participants and reconciles their profile/world session state.
- Findings: Local co-op sessions, not external multiplayer client or Forge handshake.
- RustChunk relevance: Local co-op guest/session management, distinct from remote server players.
- Decision: unnecessary; desktop-only
- 1.20.1 implications: Exclude; keep external server-player identity separate from local co-op guests.
- Follow-up checks: No baseline follow-up; do not conflate local IDs with remote entity/player IDs.
- RustChunk implementation mapping: None.
- Evidence required before adoption: No adoption evidence unless local co-op is explicitly scoped.
- Completion state: COMPLETE — 424 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 16857 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1465. `crates/horizon-client/src/game/split/setup.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bc3d4c03ede440ad84cea99c31ca5c1e6bf1819c`
- Size: 7892 bytes; 188 logical lines
- Coverage: 1–188 (EOF)
- Purpose: Split-screen setup UI and desktop monitor/layout persistence.
- Dependencies / cross-references: Desktop monitor enumeration; window sizing and viewport layout; settings.
- Implementation / logic: Stores/applies display arrangement and monitor choices for local split-screen.
- Findings: Desktop display configuration has no direct Android counterpart or game-core role.
- RustChunk relevance: Desktop display setup with no direct Android baseline role.
- Decision: unnecessary; desktop-only
- 1.20.1 implications: Do not port; use Android window/inset APIs for any separately designed feature.
- Follow-up checks: No baseline follow-up.
- RustChunk implementation mapping: None.
- Evidence required before adoption: Separately scoped Android multi-window/local co-op feature.
- Completion state: COMPLETE — 188 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 7892 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1466. `crates/horizon-client/src/game/split/windows.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `54ac330ba144ec6544e395c39a9603b131b9d63e`
- Size: 27203 bytes; 678 logical lines
- Coverage: 1–400, 401–678 (contiguous to EOF)
- Purpose: Desktop split-screen multi-window creation, monitor selection and lifecycle.
- Dependencies / cross-references: Desktop windowing APIs; monitor enumeration; compositor; per-window renderer/input ownership.
- Implementation / logic: Creates/arranges/destroys multiple native windows linked to local participants.
- Findings: Desktop window APIs do not map to Android Activity/SurfaceView lifecycle or external multiplayer.
- RustChunk relevance: Desktop multi-window management cannot be reused for Android surfaces.
- Decision: unnecessary; desktop-only
- 1.20.1 implications: Exclude desktop window management; Android rendering remains surface/activity-lifecycle aware.
- Follow-up checks: No baseline follow-up; future Android multi-window needs platform/memory/surface assessment.
- RustChunk implementation mapping: None for baseline.
- Evidence required before adoption: Explicit Android multi-window requirement and device tests.
- Completion state: COMPLETE — 678 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 27203 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1467. `crates/horizon-client/src/game/startup_menu.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a46ac19e34fb8d9cafdee1060afff7381fa97a4a`
- Size: 74444 bytes; 1794 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1794 (contiguous to EOF); empty EOF probe 1795–2194
- Purpose: Startup/title UI, async resource-worker loading, retry/error handling, queued world/server entry and transition to active play.
- Dependencies / cross-references: Resource reload worker; renderer/title; local world/remote server entry; platform events/input; mod/resource restart.
- Implementation / logic: Runs startup tasks asynchronously, reports errors/progress, retries, waits for first title frame and dispatches local/world/server entry.
- Findings: Loader/hand-off pattern is useful, but direct port carries Horizon render/assets and later-version assumptions; Android lifecycle needs separate handling.
- RustChunk relevance: Strong startup/retry/entry architecture reference, but not evidence of Forge compatibility.
- Decision: adaptable; reference; rewrite required
- 1.20.1 implications: Build RustChunk bootstrap for 1.20.1 resources/Java/Forge dependencies with responsive UI, cancellation and readiness gates; do not import Horizon native-mod restart.
- Follow-up checks: Check cancellation, panic/error reports, idempotent retry, partial cleanup, foreground/background, low-memory and connect failure recovery.
- RustChunk implementation mapping: RustChunk Android startup coordinator and versioned resource-preparation state machine.
- Evidence required before adoption: Deterministic worker tests, Android lifecycle tests, failure injection and first-frame/session traces.
- Completion state: COMPLETE — 1794 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 74444 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1468. `crates/horizon-client/src/game/station_control.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `93ee87a7dd140ac91ccc5af425178c4c40074e30`
- Size: 10492 bytes; 239 logical lines
- Coverage: 1–239 (EOF)
- Purpose: Control-socket commands for menus, slots, text input, XP, trades and effects.
- Dependencies / cross-references: Developer control socket; game/menu engine; inventory/stations; parsing and permission boundary.
- Implementation / logic: Routes external debug commands to menu clicks, text, XP and entity/effect mutations.
- Findings: Developer/debug interface and a sensitive command boundary, not player-facing game code.
- RustChunk relevance: Developer-facing command surface with security implications.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Exclude from production baseline; any diagnostic socket must be local, authenticated, allowlisted and absent from release APKs.
- Follow-up checks: If retained, test binding, auth, allowlist, bounds, release exclusion and privacy/log behavior.
- RustChunk implementation mapping: No runtime mapping; optional authenticated developer diagnostics.
- Evidence required before adoption: Threat model, authenticated protocol, dev-build gate and security review.
- Completion state: COMPLETE — 239 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 10492 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1469. `crates/horizon-client/src/game/stations.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `29bccdb8a4cf7966f14018a2dd150a3e706ee2f4`
- Size: 124600 bytes; 3120 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3120 (contiguous to EOF); empty EOF probe 3121–3520
- Purpose: Workstation/container station state, slot actions, anvil/enchanting/smithing/grindstone/stonecutter/trading/beacon/cartography/loom UI, tooltips and brewing.
- Dependencies / cross-references: Items/inventory/menu; recipes/enchantments/brewing; block entities; UI atlas/tooltips; sound/random; remote menus and native hooks.
- Implementation / logic: Maintains station inputs/results/offers, handles menu transactions/clicks, draws screens/tooltips and ticks brewing stands.
- Findings: Core 1.20.1 workstations matter; modern item components, bundles, later smithing templates/entities/blocks and Horizon native hooks make direct reuse unsafe.
- RustChunk relevance: High gameplay relevance for vanilla menus, but target-version rewrite is required.
- Decision: rewrite required; version-specific; reference
- 1.20.1 implications: Rebuild target 1.20.1 stations with legacy ItemStack/NBT, target recipe/enchantment registries and 1.20.1 container packet sync; gate later-only branches.
- Follow-up checks: Verify station recipes/results, shift-click/cursor, costs/XP, menu/state IDs, anvil break chance, brewing fuel/ticks and legacy tooltip data.
- RustChunk implementation mapping: RustChunk 1.20.1 station domain modules plus separate legacy remote-menu adapter.
- Evidence required before adoption: 1.20.1 mappings/packet fixtures, station inventory scenarios, recipe/XP goldens and multiplayer click traces.
- Completion state: COMPLETE — 3120 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 124600 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1470. `crates/horizon-client/src/game/stats_screen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `03fac444c8cff2d553e4093433e7eb597f7d2144`
- Size: 26610 bytes; 654 logical lines
- Coverage: 1–400, 401–654 (contiguous to EOF)
- Purpose: Statistics screen with General/Items/Mobs tabs, sorting, tooltips, scrolling and server refresh request.
- Dependencies / cross-references: Player stats; item/mob registry/translations; UI sprites/widgets; ClientCommand packet; scroll/input.
- Implementation / logic: Builds custom stat rows, sorts item columns, lists mob kill stats and handles tab/sort/click/scroll.
- Findings: Statistics UI exists in 1.20.1, but later stat names/species and widget contracts must be filtered/adapted.
- RustChunk relevance: Optional statistics UI; not a first-join requirement.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 stat registry keys and packet format; remove non-target entities/stats and adapt Android input/UI.
- Follow-up checks: Compare ordering, formatting, empty tabs, sort direction, scroll bounds and 1.20.1 request/response.
- RustChunk implementation mapping: RustChunk stats view and Android-adapted statistics screen.
- Evidence required before adoption: 1.20.1 stat mappings, packet traces, locale goldens and touch-scroll tests.
- Completion state: COMPLETE — 654 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 26610 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1471. `crates/horizon-client/src/game/stats.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4d3f1d3cbe77f62461e3b30816e9584e67833263`
- Size: 19209 bytes; 500 logical lines
- Coverage: 1–400, 401–500 (contiguous to EOF); empty EOF probe 501–900
- Purpose: Player stats persistence/dirty events, movement/time counters and phantom spawning.
- Dependencies / cross-references: Player UUID/world save; JSON storage; entities/mounts; block/fluid facts; LAN guest stats; random/difficulty.
- Implementation / logic: Maintains stats maps and dirty updates, reads/writes stats JSON, records tick/movement counters and runs time-since-rest phantom spawning.
- Findings: Basic counters/phantoms are 1.20.1-relevant; later keys/species such as Happy Ghast/Nautilus must be excluded.
- RustChunk relevance: Player-stat persistence and phantom behavior are relevant vanilla systems requiring target registries.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use target 1.20.1 stat keys, player stats path and movement categories; remove new mount species and verify Java integer/JSON semantics.
- Follow-up checks: Test dirty sync/reset, movement thresholds, sleep/death, save/load UUID, phantom interval/difficulty and bounds.
- RustChunk implementation mapping: RustChunk 1.20.1 stat store, counters and phantom routine.
- Evidence required before adoption: 1.20.1 stat source, JSON fixtures, seeded movement/spawn tests and server packet traces.
- Completion state: COMPLETE — 500 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 19209 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1472. `crates/horizon-client/src/game/storage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `61d9f59ebb2a7b2d06b194f7554d19c9323f1d49`
- Size: 20803 bytes; 522 logical lines
- Coverage: 1–400, 401–522 (contiguous to EOF)
- Purpose: Shulker/ender chest inventory, spectator opening, double-chest pairing, titles/sounds and reach-based menu closure.
- Dependencies / cross-references: Chest/shulker/barrel block entities; item stacks; world states/mob collision; menus; sound; player reach.
- Implementation / logic: Checks blocked containers, composes chest halves, maps menu slots to block entities, preserves ender inventory and draws screens.
- Findings: Shulker/ender chests are 1.20.1 features; copper-chest branches are later content. Local simulation must be separated from remote server menu authority.
- RustChunk relevance: Important container/inventory behavior and player interactions.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port 1.20.1 chest/shulker/ender-chest types and legacy item NBT; exclude copper chest; separate remote menus from local state.
- Follow-up checks: Verify double chest order, blocked cat/conductor, ender inventory save/close, reach, slot IDs, spectator and sounds.
- RustChunk implementation mapping: RustChunk container model and legacy UI connected to remote menu adapter.
- Evidence required before adoption: 1.20.1 block/BE schemas, packet traces, inventory round-trip and double chest tests.
- Completion state: COMPLETE — 522 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 20803 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1473. `crates/horizon-client/src/game/structure_blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `52fdf7ff29476324c4116c264296a2396544e825`
- Size: 28181 bytes; 739 logical lines
- Coverage: 1–400, 401–739 (contiguous to EOF); empty EOF probe 740–1140
- Purpose: Structure block save/load/scan, template NBT serialization/placement, redstone trigger and bounding-box visualization.
- Dependencies / cross-references: Structure block entity; NBT/gzip; StructureTemplate; chunks/entities; block registry transforms; permissions/protocol.
- Implementation / logic: Caches templates, captures block/entity data, writes generated structure NBT, detects corners, transforms/places structures and updates entity state.
- Findings: Core structure blocks exist in 1.20.1. TestInstanceBlock/later data assumptions are not target baseline; template IO paths need security review.
- RustChunk relevance: Optional creative/admin functionality, not required for normal server joining.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 template NBT/DataVersion/placement; exclude TestInstanceBlock and retain path/permission guards.
- Follow-up checks: Test save/load, corner scan, invalid size, rotations/mirrors, integrity seed, entity/BE persistence and guest permissions.
- RustChunk implementation mapping: RustChunk 1.20.1 creative structure-template manager behind game-master gates.
- Evidence required before adoption: 1.20.1 mappings/NBT fixtures, rotation goldens, traversal tests and permission traces.
- Completion state: COMPLETE — 739 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 28181 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1474. `crates/horizon-client/src/game/structure_edit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `150c4b355116e668f5cb044bce29e484d624b9b7`
- Size: 16591 bytes; 507 logical lines
- Coverage: 1–400, 401–507 (contiguous to EOF)
- Purpose: Structure-block edit screen, text fields, mode/options and SetStructureBlock request.
- Dependencies / cross-references: Block editor; structure modes/fields; EditBox/UI; serverbound SetStructureBlock packet.
- Implementation / logic: Builds mode-dependent controls, validates names, formats integrity, serializes offsets/options and restores state on cancel.
- Findings: Creative editor is relevant, but ordinals and packet body are version-sensitive; current packet is not 1.20.1 evidence.
- RustChunk relevance: Optional creative editor; its packet must be target-version exact.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use exact 1.20.1 packet fields/ordinals and target client mappings for parsing and screen behavior.
- Follow-up checks: Check modes/options, cancel/done, invalid numbers, names, mirror/rotation and packet bytes.
- RustChunk implementation mapping: RustChunk 1.20.1 structure editor and protocol adapter.
- Evidence required before adoption: 1.20.1 mappings, packet goldens, edit/cancel fixtures and creative permission tests.
- Completion state: COMPLETE — 507 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 16591 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1475. `crates/horizon-client/src/game/structures.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `95a65e4ae92d2b32110fae1141f9443b43e3ad4f`
- Size: 2066 bytes; 54 logical lines
- Coverage: 1–54 (EOF)
- Purpose: Finds structures reaching a block position using chunk structure refs and cached structure starts/pieces.
- Dependencies / cross-references: Loaded chunk refs; structure locator; starts and piece bounding boxes.
- Implementation / logic: Filters refs by structure ID, queries starts at referenced chunk coordinates and merges whole-start/piece containment.
- Findings: Applicable query pattern, but structure registry IDs and generation/cache APIs are version-specific.
- RustChunk relevance: Optional structure lookup utility, not core multiplayer connectivity.
- Decision: adaptable; reference
- 1.20.1 implications: Rebuild against 1.20.1 chunk references, generated starts and piece bounds; do not carry later structure IDs/generator formats.
- Follow-up checks: Verify cross-chunk refs, multiple starts, whole box vs piece containment and unloaded chunks.
- RustChunk implementation mapping: RustChunk 1.20.1 structure-reference query service.
- Evidence required before adoption: 1.20.1 chunk NBT and structure-start fixtures with deterministic containment tests.
- Completion state: COMPLETE — 54 logical lines read to EOF in the listed contiguous coverage; pinned blob SHA and 2066 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1476. `crates/horizon-client/src/game/subtitle_hud.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f54ca874d2bf5998a800a305a5a1189a3c73893d`
- Size: 2050 bytes; 58 logical lines
- Coverage: 1–58 (EOF)
- Purpose: Displays timed sound subtitles and directional cues as a HUD overlay.
- Dependencies / cross-references: Sound/subtitle event queue; sound positions; subtitle option; HUD text/atlas; hidden-GUI state and localization.
- Implementation / logic: Consumes active subtitle events, maintains lifetime and placement, and draws localized text with directional indicators while respecting HUD visibility.
- Findings: Subtitles and directional HUD cues are vanilla 1.20.1 features; event queue and expiry semantics must match target sources.
- RustChunk relevance: Useful accessibility/awareness feature, but optional for initial multiplayer connectivity.
- Decision: adaptable; optional
- 1.20.1 implications: Use 1.20.1 subtitle option, sound listener, expiry timing, directional cue calculation and HUD layout; verify the snapshot event model.
- Follow-up checks: Check enabled/disabled, overlapping events, duplicates, expiry, speaker direction, localization, F1 and phone-scaled layout.
- RustChunk implementation mapping: RustChunk audio subtitle queue and Android-safe subtitle HUD layer.
- Evidence required before adoption: 1.20.1 mappings/source, deterministic sound-event fixtures, direction/expiry tests and phone aspect-ratio checks.
- Completion state: COMPLETE — 58 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 2050 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1477. `crates/horizon-client/src/game/sulfur_cubes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f8bcb24555f31aa17739467e94091749c6e7b33d`
- Size: 13634 bytes; 347 logical lines
- Coverage: 1–347 (EOF)
- Purpose: Implements Sulfur Cube variants, swallowed-block behavior, shearing, explosive interactions, splitting, feeding and effects.
- Dependencies / cross-references: Sulfur Cube entity/AI; block states and pickup; explosions/TNT; entity damage; feeding/items; particles/sounds and test geometry.
- Implementation / logic: Updates the later mob's state/interactions, including storing/releasing blocks, variant behavior and explosive/splitting actions.
- Findings: Sulfur Cube is not a vanilla 1.20.1 entity or mechanic; its registry, AI and content are outside the target baseline.
- RustChunk relevance: No baseline value; reference only if a specific 1.20.1 Forge mod adds this mob.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude from vanilla 1.20.1; add only after identifying a concrete Forge 1.20.1 mod and validating registrations.
- Follow-up checks: For a named mod verify entity registry, spawn eggs, AI, block interaction, damage, drops and server authority.
- RustChunk implementation mapping: No vanilla mapping; isolated optional mod-specific entity adapter.
- Evidence required before adoption: Named 1.20.1 Forge mod source/version, license, registry IDs, entity data and server integration tests.
- Completion state: COMPLETE — 347 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 13634 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1478. `crates/horizon-client/src/game/test_block_edit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a391b377cd07570214edf28a71340eff3a5b6c43`
- Size: 5379 bytes; 185 logical lines
- Coverage: 1–185 (EOF)
- Purpose: Provides an editor for GameTest TestBlock configuration and submits mode/message settings.
- Dependencies / cross-references: TestBlock block entity; mode/message fields; editor widgets; SetTestBlock packet and permissions.
- Implementation / logic: Builds test-block controls, maps choices to mode/message and sends serverbound update.
- Findings: Later GameTest tooling, not an ordinary 1.20.1 workflow; its packet is not target protocol evidence.
- RustChunk relevance: Developer/test-framework feature, irrelevant to initial Android server-join milestone.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Do not implement in the 1.20.1 baseline or add its packet/block entity to the target registry.
- Follow-up checks: Only reconsider if a named 1.20.1 mod exposes compatible TestBlock; validate packet, permissions and block-entity schema.
- RustChunk implementation mapping: No baseline mapping; optional isolated developer test UI.
- Evidence required before adoption: Confirmed target registry/protocol absence and a specific compatible mod requirement.
- Completion state: COMPLETE — 185 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5379 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1479. `crates/horizon-client/src/game/test_blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6c0448b0dedd96cb319be2041ea062462a379514`
- Size: 5156 bytes; 155 logical lines
- Coverage: 1–155 (EOF)
- Purpose: Implements server-side TestBlock activation, mode state, redstone-neighbor handling and permission-gated test operations.
- Dependencies / cross-references: GameTest TestBlock block entity; redstone updates; server world/block state; creative/guest permissions.
- Implementation / logic: Tracks test-block state, reacts to triggers/redstone and dispatches test actions subject to permissions.
- Findings: TestBlock and GameTest workflow are absent from vanilla 1.20.1; later framework assumptions apply.
- RustChunk relevance: Not needed for regular gameplay or Forge server interoperability.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude TestBlock state, trigger behavior and packet handling from vanilla 1.20.1.
- Follow-up checks: Only revisit against a specific mod backporting GameTest blocks to Forge 1.20.1.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Target block registry/packet comparison and named mod source/server behavior if applicable.
- Completion state: COMPLETE — 155 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5156 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1480. `crates/horizon-client/src/game/test_entities.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4bdb61317758f61ba9e29bea55889a554f128f09`
- Size: 5570 bytes; 144 logical lines
- Coverage: 1–144 (EOF)
- Purpose: Manages entity bounds/intersection and removal for GameTest workspaces and force-loaded test chunks.
- Dependencies / cross-references: GameTest workspace bounds; entity AABBs/lifecycle; chunk force-loading; teardown and world entity storage.
- Implementation / logic: Finds entities intersecting test regions and cleans them as fixtures/workspaces are reset.
- Findings: Test-harness lifecycle logic, not vanilla 1.20.1 runtime entity simulation; depends on newer GameTest workspaces.
- RustChunk relevance: No value for initial Android client or remote protocol.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude GameTest-only entity cleanup; preserve ordinary 1.20.1 entity/chunk unload behavior separately.
- Follow-up checks: Only revisit if a 1.20.1 test-framework mod is selected; verify ownership, boundary intersection and force-load cleanup.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Specific compatible test-framework mod plus deterministic boundary and cleanup tests.
- Completion state: COMPLETE — 144 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5570 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1481. `crates/horizon-client/src/game/test_environments.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b321ac7bed199db5514b33441ee3d355c4fa4f2`
- Size: 12821 bytes; 375 logical lines
- Coverage: 1–375 (EOF)
- Purpose: Parses/applies GameTest batch environment settings including clock, difficulty, functions, gamerules and weather.
- Dependencies / cross-references: GameTest batch metadata; world clock/difficulty/weather/gamerules; function runner and teardown.
- Implementation / logic: Reads environment definitions, applies test-world conditions and restores/transitions environment state.
- Findings: Later GameTest infrastructure, not a 1.20.1 client feature.
- RustChunk relevance: Not needed for ordinary worlds or joining modded servers.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude and do not reuse this test environment schema as world configuration.
- Follow-up checks: If a target Forge mod needs it, verify mutations and teardown after test failures.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Named 1.20.1-compatible GameTest mod, format fixtures, teardown/failure tests and server authority checks.
- Completion state: COMPLETE — 375 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 12821 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1482. `crates/horizon-client/src/game/test_instance_edit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4069619069ef18e305c816fc3755641775a6822e`
- Size: 12796 bytes; 412 logical lines
- Coverage: 1–400, 401–412 (contiguous to EOF)
- Purpose: Edits TestInstance settings/actions including initialize, query, set, reset, save and run.
- Dependencies / cross-references: TestInstance metadata; definitions; size/rotation/entities; editor widgets; serverbound action packets.
- Implementation / logic: Builds mode-specific controls, serializes dimensions/options and dispatches initialize/reset/save/run actions.
- Findings: TestInstance and associated blocks are later GameTest infrastructure, not vanilla 1.20.1; action packets are not target protocol.
- RustChunk relevance: No initial mobile client value; developer test tooling only.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude TestInstance UI, state and packet IDs from the 1.20.1 baseline.
- Follow-up checks: Only reconsider for a 1.20.1 Forge backport; validate actions, modes, rotation, entity flags, permissions and packet body.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Named target mod, packet captures, schema fixtures and permission/lifecycle tests.
- Completion state: COMPLETE — 412 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 12796 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1483. `crates/horizon-client/src/game/test_instances.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d9872ee138fdd6d95718c0cce3a330cb7d81b22d`
- Size: 25069 bytes; 666 logical lines
- Coverage: 1–400, 401–666 (contiguous to EOF)
- Purpose: Loads GameTest instance definitions/bounds/markers, force-loads test chunks, resets/places templates and starts tests.
- Dependencies / cross-references: Test-instance JSON/resources; structure templates; chunk loading; block/entity registries; GameTest runner and permissions.
- Implementation / logic: Parses definitions, builds bounds, renders markers and coordinates reset, template placement, chunk forcing and test execution.
- Findings: Tightly coupled to TestInstance/GameTest and later schemas; not a general 1.20.1 structure implementation.
- RustChunk relevance: Developer-only reference at most; not required for joining a modded server.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude its registries, definition format and execution flow; implement target structure-template semantics independently.
- Follow-up checks: If a compatible test mod is selected, verify schema, DataVersion, chunk ownership, cleanup, placement and permissions.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Target-compatible mod source, template/NBT fixtures, resource validation and chunk lifecycle tests.
- Completion state: COMPLETE — 666 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 25069 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1484. `crates/horizon-client/src/game/text_displays.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dcdc9fd3a0efdf1ab955e3a130b030c0a27e163c`
- Size: 18306 bytes; 470 logical lines
- Coverage: 1–400, 401–470 (contiguous to EOF)
- Purpose: Synchronizes/renders TextDisplay, BlockDisplay and ItemDisplay entities with interpolation, lighting and culling.
- Dependencies / cross-references: Display metadata/data indices; entity interpolation/transforms; item/block/text models; lighting/culling; server entity packets.
- Implementation / logic: Reads display entity state, interpolates transforms/billboard/brightness, resolves content and submits render geometry.
- Findings: Display entities exist in 1.20.1; metadata indices and later ItemDisplay contexts need exact target mapping; shelf behavior is later.
- RustChunk relevance: High relevance to correct rendering of modern vanilla display entities, but version-specific and not proof of Forge support.
- Decision: adaptable; version-specific
- 1.20.1 implications: Implement 1.20.1 display entities/metadata; omit later-only fields and contexts unless target source confirms them.
- Follow-up checks: Check spawn/update/remove metadata, interpolation, billboard, brightness, shadow, text flags, block transforms, item context and chunk unload.
- RustChunk implementation mapping: RustChunk 1.20.1 display-entity model, packet decoder and render interpolation.
- Evidence required before adoption: 1.20.1 metadata tables, captured entity packets and transform/render fixtures.
- Completion state: COMPLETE — 470 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 18306 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1485. `crates/horizon-client/src/game/titles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bf3ecead5dd9dd79b8fc7833a52d0b4cb96283ba`
- Size: 3946 bytes; 96 logical lines
- Coverage: 1–96 (EOF)
- Purpose: Ticks/renders title, subtitle, action-bar, selected-item-name and save-indicator HUD elements.
- Dependencies / cross-references: Title/action-bar packet events; HUD visibility; localized text; item name/rarity; save indicator/timers.
- Implementation / logic: Maintains message timers/fades and draws centered/action-bar text and indicators.
- Findings: Core title/action-bar and item-name HUD behavior exists in 1.20.1; durations, reset rules and rarity formatting need target comparison.
- RustChunk relevance: Useful vanilla UX, but not essential for first protocol handshake.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 packet behavior, fade timings, selected-item presentation and legacy item naming/rarity.
- Follow-up checks: Verify replacement, clear/reset packets, durations, pause, action-bar overlap, F1 and localization.
- RustChunk implementation mapping: RustChunk versioned HUD message state and Android-scaled renderer.
- Evidence required before adoption: 1.20.1 source/mappings, packet fixtures and deterministic tick/fade tests.
- Completion state: COMPLETE — 96 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 3946 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1486. `crates/horizon-client/src/game/tnt.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `044e868ea43c8803efa70c8485faa3cb7512d3ae`
- Size: 32253 bytes; 840 logical lines
- Coverage: 1–400, 401–800, 801–840 (contiguous to EOF)
- Purpose: Simulates/renders primed TNT entities, fuse timing, motion, fluid drag, collision, ignition and explosion.
- Dependencies / cross-references: Primed TNT state/NBT; entity collision; world blocks/fluids; explosion pipeline; owner attribution; renderer/audio.
- Implementation / logic: Ticks fuse/movement, resolves collision/fluid drag, handles ignition/explosion and owner credit, and renders the entity.
- Findings: TNT is 1.20.1 content; Horizon/native hooks and physics must be compared with target fuse, explosion and entity packet rules.
- RustChunk relevance: Core survival feature with visible/destructive multiplayer effects; correctness matters.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild with 1.20.1 TNT NBT, fuse order, physics, explosion resistance/drop logic and owner attribution; native hooks are not Forge events.
- Follow-up checks: Test fuse, fluids, collision, ignition, chain reactions, damage, drops, unload/reload and authority.
- RustChunk implementation mapping: RustChunk 1.20.1 primed-TNT simulation, packet interpolation and explosion integration.
- Evidence required before adoption: 1.20.1 mappings/source, NBT fixtures, seeded explosion tests and server entity traces.
- Completion state: COMPLETE — 840 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 32253 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1487. `crates/horizon-client/src/game/totem.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c3bbfbc806e9b138d81ab4bfba472fc6ebbdb657`
- Size: 7639 bytes; 196 logical lines
- Coverage: 1–196 (EOF)
- Purpose: Handles Totem of Undying activation during lethal damage, effects, item consumption and client presentation.
- Dependencies / cross-references: Player damage/death; hands/inventory; effects; stack consumption; status event and HUD/audio.
- Implementation / logic: Detects eligible totem activation, applies survival effects and feedback, and coordinates hand/item state.
- Findings: Totem exists in 1.20.1; snapshot uses modern death-protection component abstractions, not legacy target ItemStack flow.
- RustChunk relevance: Important vanilla survival behavior; target-era exactness required.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Use 1.20.1 hand selection/consumption, effects, health/fire/absorption and entity-event handling; do not require modern components.
- Follow-up checks: Check hand priority, count, creative, damage sources, fire/health/effects, consumption, status event and authority.
- RustChunk implementation mapping: RustChunk 1.20.1 death-prevention/totem handler over legacy ItemStack/NBT.
- Evidence required before adoption: 1.20.1 mappings, lethal-damage matrix, inventory fixtures and client/server traces.
- Completion state: COMPLETE — 196 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7639 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1488. `crates/horizon-client/src/game/touch.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5e47fb80934651a3ebbe6375d18bed7bdc827991`
- Size: 85678 bytes; 2039 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2039 (contiguous to EOF)
- Purpose: Implements on-screen touch controls, layout editing, custom buttons, virtual-pad actions, multitouch gestures, pointer aiming, Android keyboard coordination and chat controls.
- Dependencies / cross-references: Winit touch events; Android mobile-controls bridge; virtual gamepad/keybinds; pointer capture; options/layout JSON; renderer/HUD; IME/clipboard; custom actions.
- Implementation / logic: Routes finger phases into key/mouse actions, draws/edits touch layouts, supports custom controls/gestures, synchronizes held state and bridges Android keyboard/chat.
- Findings: Highly relevant Android architecture reference, but Starling/Horizon layouts, assets and key mappings are not drop-in RustChunk code; lifecycle and held-input correctness are critical.
- RustChunk relevance: Very high relevance to Android A22; requires RustChunk platform bridge and lifecycle/memory design.
- Decision: adaptable; rewrite required; Android-specific
- 1.20.1 implications: Port needed 1.20.1 controls through RustChunk Android input; decouple layout/config from Starling assets and release held inputs on focus loss/background.
- Follow-up checks: Test multitouch, drag/look, attack/use, jump/sneak, pointer capture, layout persistence, custom buttons, controller coexistence, IME, pause/resume, rotation and touch cancellation.
- RustChunk implementation mapping: RustChunk Android touch controller/router, layout editor and lifecycle-safe IME integration.
- Evidence required before adoption: SM-A225F/API 33 tests, deterministic touch traces, IME lifecycle tests and stuck-key regressions.
- Completion state: COMPLETE — 2039 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 85678 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1489. `crates/horizon-client/src/game/touch/slots.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c87b6c1fb196d1439140e008028c36f8d6ad317f`
- Size: 32059 bytes; 824 logical lines
- Coverage: 1–400, 401–800, 801–824 (contiguous to EOF)
- Purpose: Implements touch-first inventory/menu slot actions: pick stacks, half-stack hold, double-tap quick-move, drag distribution, outside drop and player rotation.
- Dependencies / cross-references: Touch event stream; menu slots/ItemStack; creative inventory; container click pipeline; synthesized mouse events; pointer/scroll ownership; tooltip renderer.
- Implementation / logic: Converts touch gestures to slot click/quick-craft events, tracks picked stacks, distributes across slots and animates moved stacks/tooltips.
- Findings: Useful mobile interaction model, but local menu click assumptions do not establish correct remote Forge container synchronization.
- RustChunk relevance: Strong mobile UX feature; local simulation cannot replace remote server-authoritative packet handling.
- Decision: adaptable; version-specific
- 1.20.1 implications: Adapt gestures to 1.20.1 menus and server-authoritative click/quick-craft with exact IDs; preserve server correction handling.
- Follow-up checks: Test split-stack, quick-move, creative tabs, result exclusion, drag order, cursor recovery, close/disconnect during hold and delayed slot updates.
- RustChunk implementation mapping: RustChunk touch slot gesture adapter over legacy container/protocol transactions.
- Evidence required before adoption: 1.20.1 mappings/packet traces, slot-action goldens, inventory race/correction tests and phone UX validation.
- Completion state: COMPLETE — 824 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 32059 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1490. `crates/horizon-client/src/game/touch/text_mirror.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a6ebd0b2e56adc6e9eb7616615ba9c0dc8a04bec`
- Size: 32941 bytes; 797 logical lines
- Coverage: 1–400, 401–797 (contiguous to EOF)
- Purpose: Mirrors game text fields into Android IME and synchronizes text, cursor, selection, action keys and keyboard visibility.
- Dependencies / cross-references: Android TextMirror bridge; IME events; EditBox; chat/sign/book/anvil/command/options/world/dialog/catalog fields; clipboard/password handling.
- Implementation / logic: Tracks field identity/generation to reject stale edits, maps keyboard actions, applies filters/limits and synchronizes show/update/hide and fallback.
- Findings: Valuable Android text-entry design; depends on companion platform code and exact field APIs. Password redaction/generation checks are important.
- RustChunk relevance: High Android usability for chat, server addresses, commands, books and signs.
- Decision: adaptable; Android-specific; rewrite required
- 1.20.1 implications: Implement RustChunk IME bridge for 1.20.1 UI; preserve field generation and redacted password diagnostics using Android APIs independently.
- Follow-up checks: Test Unicode/selection, autocorrect/paste, field-switch races, Back/Enter/Tab, multiline filtering, password redaction, orientation and fallback.
- RustChunk implementation mapping: RustChunk Android IME mirror and game-field adapters.
- Evidence required before adoption: API 33 instrumentation, Unicode fixtures, stale-event races and privacy review.
- Completion state: COMPLETE — 797 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 32941 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1491. `crates/horizon-client/src/game/trading.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `217a232180f7141d569026d1e5c5f4437d364777`
- Size: 37199 bytes; 934 logical lines
- Coverage: 1–400, 401–800, 801–934 (contiguous to EOF)
- Purpose: Implements villager/wandering-trader interactions, offer generation/session state, trading UI, rewards, maps and persistence.
- Dependencies / cross-references: Villager/trader data; profession/variant registries; trade tables; merchant session; random sequences; menus; map NBT/structure locator; SelectTrade packet; reputation/effects.
- Implementation / logic: Generates offers, opens local/remote sessions, processes trades/XP/reputation, creates exploration maps and draws/scrolls offers.
- Findings: Trading is core 1.20.1, but random-sequence persistence, map/item conventions, native hooks and modern merchant abstractions need target comparison.
- RustChunk relevance: Relevant vanilla gameplay and multiplayer menu interaction, not a shortcut to Forge support.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Reimplement 1.20.1 offers, legacy map/item NBT, merchant slot IDs and SelectTrade/ContainerClick semantics; remove later random-sequence/native assumptions absent from target.
- Follow-up checks: Verify profession/level offers, demand/discounts, depletion/restock, XP, map NBT, trader sets, remote click order and mob persistence.
- RustChunk implementation mapping: RustChunk 1.20.1 merchant model and Android UI over legacy menu/protocol adapter.
- Evidence required before adoption: 1.20.1 trade tables/mappings, deterministic RNG fixtures, map NBT, merchant packets and server traces.
- Completion state: COMPLETE — 934 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 37199 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1492. `crates/horizon-client/src/game/transient_blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d28cf2b5762a54f834435295c25dbac5d2393439`
- Size: 5977 bytes; 161 logical lines
- Coverage: 1–161 (EOF)
- Purpose: Temporarily renders server-announced blocks until replacement section mesh is compiled/uploaded.
- Dependencies / cross-references: Custom AddTransientBlock packet; section meshing sequence; block-state registry; render geometry and TTL queue.
- Implementation / logic: Queues transient block states, renders until TTL and removes them when a later section mesh uploads.
- Findings: Custom packet/render bridge is not vanilla 1.20.1 protocol; idea may inform local chunk rebuild transitions but is not server-interoperable.
- RustChunk relevance: Interesting renderer reference for chunk visibility, not an initial baseline feature.
- Decision: unnecessary; reference; later-version/custom protocol
- 1.20.1 implications: Do not add custom packet to 1.20.1 client; smoothing must be client-only and cannot fabricate server block state.
- Follow-up checks: For local exploration, test mesh upload races, duplicates, TTL, unload/reload and stale state without protocol changes.
- RustChunk implementation mapping: Optional local chunk-render transition overlay independent of network decoding.
- Evidence required before adoption: 1.20.1 server traces proving no dependency, meshing stress tests and Mali-G52 validation.
- Completion state: COMPLETE — 161 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5977 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1493. `crates/horizon-client/src/game/trial_spawner.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cf901b66a28bcead1302f8c790d5adfa38e8b99c`
- Size: 14438 bytes; 368 logical lines
- Coverage: 1–368 (EOF)
- Purpose: Runs Trial Spawner simulation, player detection, spawning, reward ejection, Trial Omen effects and block transitions.
- Dependencies / cross-references: TrialSpawner block entity/state; Trial Omen; mob spawning/loot; player/guest UUIDs; world random; game events/rewards.
- Implementation / logic: Ticks trial spawners, plans entities/UUIDs, handles loot/effects/events, mutates state and transforms Bad Omen to Trial Omen.
- Findings: Trial Spawners and Trial Omen belong to 1.21 Trial Chambers, not vanilla 1.20.1.
- RustChunk relevance: No vanilla baseline use; only for an explicit backport mod.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude TrialSpawner block/entity/effect/protocol additions from 1.20.1.
- Follow-up checks: For an exact Forge backport verify registries, effects, loot, packet data and server-authoritative ticks.
- RustChunk implementation mapping: No vanilla mapping; optional mod-specific adapter.
- Evidence required before adoption: Named Forge 1.20.1 backport, source/license, NBT/registry fixtures and deterministic simulations.
- Completion state: COMPLETE — 368 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 14438 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1494. `crates/horizon-client/src/game/trial_visuals.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3a69bd9dd43adab37ba2da4b53cf30f8e18c400`
- Size: 14123 bytes; 345 logical lines
- Coverage: 1–345 (EOF)
- Purpose: Renders Trial Spawner/Vault events, particles, ambient sounds, spin animation and nearby-player connection effects.
- Dependencies / cross-references: TrialSpawner/Vault entities/states; particle/sound registries; event IDs 3011–3021; connected player UUID/position; client random.
- Implementation / logic: Maps trial/vault events to effects, ticks state emissions and animates display mobs/items and connection particles.
- Findings: Depends on 1.21 Trial Chambers and Vault/TrialSpawner event IDs; not vanilla 1.20.1 behavior.
- RustChunk relevance: No baseline value; reference only for a backported mod.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude these block entities, event IDs and particle assumptions from 1.20.1.
- Follow-up checks: For a Forge backport validate event ID collisions, particle types, timing/random sequence and proximity behavior.
- RustChunk implementation mapping: No vanilla mapping.
- Evidence required before adoption: Target mod event registry, packet captures and deterministic visual/event tests.
- Completion state: COMPLETE — 345 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 14123 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1495. `crates/horizon-client/src/game/trident.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1e1557390f52a9197d5df7c6627137d9d4658dd2`
- Size: 35360 bytes; 873 logical lines
- Coverage: 1–400, 401–800, 801–873 (contiguous to EOF)
- Purpose: Implements trident use/throw, Riptide, Loyalty return, projectile collision/damage, Impaling/Channeling, pickup, NBT and rendering.
- Dependencies / cross-references: Trident ItemStack/enchantments; player water/rain/movement; projectile collision; mobs/damage; lightning rods; projectile packets/NBT; renderer/pickups.
- Implementation / logic: Checks release conditions, launches/ticks tridents, resolves impacts, applies enchantments, returns Loyalty projectiles, handles pickup and rendering.
- Findings: Tridents/core enchantments exist in 1.20.1; packet/NBT, damage, collision and enchantment APIs need target-era sources. Native hooks are not Forge.
- RustChunk relevance: Core vanilla combat/projectile feature with remote-server relevance.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Implement exact 1.20.1 trident behavior, legacy item/entity NBT and spawn/move/pickup packets; adapt Channeling without current native hooks.
- Follow-up checks: Test charge threshold, durability, creative, Riptide water/rain/lift, Loyalty return/disconnect, Impaling, Channeling/rod, deflection, pickup and save/reload.
- RustChunk implementation mapping: RustChunk 1.20.1 trident item logic, projectile simulation, network adapter and renderer.
- Evidence required before adoption: 1.20.1 mappings/packet traces, physics fixtures, enchantment matrix, NBT goldens and multiplayer pickup tests.
- Completion state: COMPLETE — 873 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 35360 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1496. `crates/horizon-client/src/game/tutorial.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ee9e801bcb38732af6edb69722363f35d4e59937`
- Size: 19848 bytes; 563 logical lines
- Coverage: 1–400, 401–563 (contiguous to EOF)
- Purpose: Tracks first-world tutorial steps, movement/look progress, tree/punch/inventory/crafting hints and toasts.
- Dependencies / cross-references: Client tutorial option; player input/game mode; item/block tags and stats; HUD/localization/toast slots; inventory/mining.
- Implementation / logic: Advances tutorial from input and inventory/mining/crafting, persists step and displays contextual hints/progress.
- Findings: Basic tutorial exists in 1.20.1; tag names, persistence and timing/layout need target comparison.
- RustChunk relevance: Optional onboarding for new mobile players, not required for core gameplay/server join.
- Decision: adaptable; optional
- 1.20.1 implications: Use 1.20.1 step enum/options and hint triggers; adapt prompts/toasts to touch rather than desktop keybinds.
- Follow-up checks: Test survival/creative, movement/look, existing logs/planks, persistence, disabled tutorial, touch input, toast overlap and re-entry.
- RustChunk implementation mapping: RustChunk optional tutorial state machine and touch-oriented presenter.
- Evidence required before adoption: 1.20.1 tutorial mappings/options, progression fixtures and Android UI checks.
- Completion state: COMPLETE — 563 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 19848 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1497. `crates/horizon-client/src/game/tweaks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `67c143834e1be0e765aac6574a2f12738389b043`
- Size: 20695 bytes; 546 logical lines
- Coverage: 1–400, 401–546 (contiguous to EOF)
- Purpose: Adds optional inventory mouse tweaks, stack drag/wheel operations, slot sorting and HDR shortcut.
- Dependencies / cross-references: Menu slot model/click engine; remote menu packets; ItemStack; sort planner; settings/keybinds; HDR/display APIs.
- Implementation / logic: Implements alternate drag/wheel inventory actions through menu clicks and optional sorting/HDR actions; disabled by default.
- Findings: Non-vanilla enhancements; remote paths synthesize menu clicks and need exact 1.20.1 transactions. HDR is platform-specific.
- RustChunk relevance: Optional quality-of-life feature, not required for A22 baseline or protocol correctness.
- Decision: unnecessary; optional/future
- 1.20.1 implications: Keep out of baseline; if selected, gate behind settings and use authoritative 1.20.1 menu transactions; separate HDR capability.
- Follow-up checks: Test no-op defaults, drag/wheel, result slots, server corrections, sort determinism and unsupported HDR displays.
- RustChunk implementation mapping: Optional RustChunk inventory enhancements after menu/protocol stability.
- Evidence required before adoption: Explicit UX approval, packet captures, deterministic sort tests and Android display capability checks.
- Completion state: COMPLETE — 546 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 20695 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1498. `crates/horizon-client/src/game/ui_control.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `82877543667d2efd11576631966436fb56ea6ce7`
- Size: 8684 bytes; 220 logical lines
- Coverage: 1–220 (EOF)
- Purpose: Exposes UI widget tree and agent commands to inspect/click/set widgets via synthetic input.
- Dependencies / cross-references: Agent/control channel; labelled widget tree/generation; screen slots; synthetic window events; slider/menu handlers.
- Implementation / logic: Serializes widgets/bounds, defers stale-frame requests and routes clicks/slider positioning through normal UI events.
- Findings: Agent/developer automation surface, not vanilla gameplay; any command channel is a security boundary.
- RustChunk relevance: Not needed in release Android client; useful only in local development builds.
- Decision: unnecessary; developer-only
- 1.20.1 implications: Exclude from production or gate behind authenticated local debug mode; never expose unauthenticated widget mutation to network clients.
- Follow-up checks: Test build gating, local binding/auth, stale-frame races, bounds, command allowlists and remote isolation.
- RustChunk implementation mapping: No release mapping; optional local development UI automation.
- Evidence required before adoption: Threat model, release artifact inspection, auth/allowlist tests and dev-tools requirement.
- Completion state: COMPLETE — 220 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 8684 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1499. `crates/horizon-client/src/game/update_offer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `50fa3ad4959f52570bd4d2f54d8fe3ecc67d9b24`
- Size: 17111 bytes; 450 logical lines
- Coverage: 1–400, 401–450 (contiguous to EOF)
- Purpose: Checks GitHub releases for a newer Android APK, presents prompt, downloads/verifies and launches Android installer.
- Dependencies / cross-references: GitHub release API/allowlist; HTTPS download; SHA-256 metadata; private storage; package installer permission; threads/cancellation.
- Implementation / logic: Finds newer release, tracks progress, downloads to temp file, checks optional digest, renames and requests install.
- Findings: Launcher update infrastructure, not Minecraft 1.20.1 behavior. HTTPS plus optional hash does not establish publisher authenticity; release trust policy is needed.
- RustChunk relevance: Potentially useful only if RustChunk owns a trusted update channel; irrelevant to Forge game support.
- Decision: unnecessary; optional/future; security review required
- 1.20.1 implications: Do not port into game core. If self-update is added, require trusted manifest/signature or mandatory digest, rollback and Android package-signature continuity.
- Follow-up checks: Review allowlist, redirects, TLS, authenticity, partial cleanup, cancellation races, storage permissions, versions and signing continuity.
- RustChunk implementation mapping: Separate RustChunk Android updater/release service, independent of Minecraft runtime.
- Evidence required before adoption: Threat model, signed manifest/release process, tamper tests and Android upgrade tests across package signatures.
- Completion state: COMPLETE — 450 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 17111 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1500. `crates/horizon-client/src/game/vault.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b94358e5a8572f6c5b5841f7db6a363113bfd55b`
- Size: 9674 bytes; 249 logical lines
- Coverage: 1–249 (EOF)
- Purpose: Runs Vault block-entity state, player/key interactions, reward ejection, state changes and display-item rendering.
- Dependencies / cross-references: Vault block entity/state; Trial Chambers player data; key/item interactions; loot tables/random; block events; item renderer.
- Implementation / logic: Ticks vault state, accepts items/keys, consumes matching items, emits rewards/events and draws spinning display item.
- Findings: Vaults are 1.21 Trial Chambers content, not vanilla 1.20.1; reward/item paths depend on later registries/components.
- RustChunk relevance: No vanilla baseline applicability; only for a mod explicitly backporting Vaults.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude Vault block/entity, packet, loot and rendering from 1.20.1.
- Follow-up checks: For a specific Forge backport verify registry/NBT, key semantics, loot, event IDs and remote authority.
- RustChunk implementation mapping: No vanilla mapping; optional mod-specific adapter.
- Evidence required before adoption: Named Forge 1.20.1 backport source/version, license, registry/loot fixtures and server-authoritative interaction tests.
- Completion state: COMPLETE — 249 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 9674 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1501. `crates/horizon-client/src/game/villagers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `84496d20c66b97f158d0df6e43fe5240b665c247`
- Size: 21089 bytes; 521 logical lines
- Coverage: 1–400, 401–521 (EOF)
- Purpose: Villager routines: food sharing/composting, village schedules, gossip, reputation, meetings, sleeping, breeding and golem summoning.
- Dependencies / cross-references: Villager brain/POIs; inventories/items; gossip and UUID; beds/bells; spawn checks; game events.
- Implementation / logic: Runs local village routines, transfers food/items, updates gossip and reputation, breeds villagers and attempts golem summoning.
- Findings: Core 1.20.1 behavior; target brain memories, POI tags, gossip decay and golem rules are version-sensitive.
- RustChunk relevance: Important local gameplay simulation; remote server remains authoritative.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port target POI/tag sets, gossip decay, inventory and golem spawn rules; do not copy later brain fields blindly.
- Follow-up checks: Test schedule/POIs, food thresholds, sharing, gossip decay, birth cooldown and golem geometry/limits.
- RustChunk implementation mapping: 1.20.1 villager AI, gossip store and POI adapter.
- Evidence required before adoption: 1.20.1 source/mappings, POI fixtures, seeded day-cycle tests, NBT round-trips and server traces.
- Completion state: COMPLETE — 521 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 21089 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1502. `crates/horizon-client/src/game/wandering_trader.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `11ff5cd3beb2a7ce5ed97802926c23edcc56795f`
- Size: 8986 bytes; 256 logical lines
- Coverage: 1–256 (EOF)
- Purpose: World-saved wandering-trader spawn timers/chance, player/bell search, biome/space checks, trader and two leashed llamas, despawn.
- Dependencies / cross-references: Saved world data; trader/llama entities; biome/collision; players/bells; offers; leash/spawn pipeline.
- Implementation / logic: Loads/saves spawn data, advances chance, samples locations and spawns trader/llamas with despawn timer.
- Findings: Wandering trader is 1.20.1; saved-data schema, timers, biome exclusion and search order need target parity.
- RustChunk relevance: Useful local-world simulation, not required to join a remote server.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 spawn timers/chance, saved data, placement and leash initialization.
- Follow-up checks: Test save/load, timer reset, chance limits, player/bell origin, excluded biomes, collision, two llamas and despawn.
- RustChunk implementation mapping: Local 1.20.1 wandering-trader spawner and legacy world-data serializer.
- Evidence required before adoption: 1.20.1 source, saved-data fixtures, seeded spawn tests and leash persistence.
- Completion state: COMPLETE — 256 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 8986 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1503. `crates/horizon-client/src/game/win.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5d2ea95a582c77fe8ad6cf33ac1870dcf1aa8c7c`
- Size: 4702 bytes; 132 logical lines
- Coverage: 1–132 (EOF)
- Purpose: End poem/credits screen, portal-fill and timed scroll, keyboard navigation, completion and exit behavior.
- Dependencies / cross-references: End-portal event; credits resources; UI/input/frame clock; session close handling.
- Implementation / logic: Starts and advances the win scene, then exits or returns when complete.
- Findings: The credits screen exists in 1.20.1; optional UI, not protocol functionality.
- RustChunk relevance: Low priority for first server join; useful for full vanilla UX.
- Decision: adaptable; optional; version-specific
- 1.20.1 implications: Reproduce 1.20.1 resources, scroll timing, skip and finish semantics; handle closed sessions safely.
- Follow-up checks: Test trigger, scroll/frame pacing, skip controls, pause/resume and missing credits assets.
- RustChunk implementation mapping: Optional Android-scaled end-screen presenter.
- Evidence required before adoption: 1.20.1 assets/mappings, frame tests and Android navigation tests.
- Completion state: COMPLETE — 132 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 4702 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1504. `crates/horizon-client/src/game/wind.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3fbd55f5ab59ee5f224413ac3b487fd99169d83d`
- Size: 23738 bytes; 603 logical lines
- Coverage: 1–400, 401–603 (EOF)
- Purpose: Wind Charge/Breeze projectiles and bursts, knockback, particles, sounds and affected block interactions.
- Dependencies / cross-references: Wind charge/Breeze entities; collision/explosion; block tags; effects; native hooks.
- Implementation / logic: Resolves wind projectiles and burst interactions, applies knockback and emits effects.
- Findings: Wind Charge and Breeze are 1.21, not vanilla 1.20.1.
- RustChunk relevance: Only useful for an explicitly selected backport mod.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude wind-charge/Breeze registries, packets and behavior from vanilla 1.20.1.
- Follow-up checks: For a named backport verify projectile, radius, block tags, knockback, events and authority.
- RustChunk implementation mapping: No vanilla mapping; optional isolated mod adapter.
- Evidence required before adoption: Named Forge 1.20.1 backport, source/license, registry and multiplayer tests.
- Completion state: COMPLETE — 603 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 23738 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1505. `crates/horizon-client/src/game/wither.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5c651017b86a74be729a1374647fe7c836ab3e76`
- Size: 13441 bytes; 327 logical lines
- Coverage: 1–327 (EOF)
- Purpose: Wither summoning, skull hits/blue-skull block destruction, Wither Rose, effects, target damage and multipart interactions.
- Dependencies / cross-references: Wither/skull entities; summon pattern; damage/effects; block resistance; rose loot; owner and entity parts.
- Implementation / logic: Checks summon pattern, resolves skull impacts/destruction and handles rose/effect/healing events.
- Findings: Wither is 1.20.1 vanilla; damage, explosion and entity event details must match target.
- RustChunk relevance: High boss/survival correctness relevance.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild target skull NBT/packet, summon rules, blue-skull resistance, rose timing and multipart targeting; native hooks are not Forge events.
- Follow-up checks: Test summon structure, skull impacts, blue-skull destruction, drops, effects, healing and tracking.
- RustChunk implementation mapping: 1.20.1 Wither boss and skull projectile adapter.
- Evidence required before adoption: 1.20.1 mappings, summon fixtures, seeded skull/explosion tests and multiplayer traces.
- Completion state: COMPLETE — 327 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 13441 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1506. `crates/horizon-client/src/game/world_content.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a617bb56c67a5225ad9930c72fa604dac7c1f518`
- Size: 13984 bytes; 389 logical lines
- Coverage: 1–389 (EOF)
- Purpose: Horizon per-world native content manifests and restart decisions when required local content differs.
- Dependencies / cross-references: Horizon content manifests; world files; installed native-content list; platform process restart.
- Implementation / logic: Compares world-required content with installed content and records/executes a restart before world loading.
- Findings: This native content lifecycle is not Forge mod discovery, classpath, transformers, dependency resolution or event-bus startup.
- RustChunk relevance: Only generic per-world modset metadata is an architectural reference.
- Decision: **undetermined; reference; rewrite required; not Forge** — reference; rewrite required; not Forge
- 1.20.1 implications: Do not treat this as a Forge loader. Build a real Forge 1.20.1 Java runtime/classpath/discovery/transformer/lifecycle integration.
- Follow-up checks: Test manifest integrity, mismatches, interrupted restart, rollback and missing dependencies; verify actual Forge loader separately.
- RustChunk implementation mapping: Per-world modset metadata only, isolated from Forge bootstrap.
- Evidence required before adoption: Forge 1.20.1 loader evidence, dependency fixtures, launch traces and real mod startup.
- Completion state: COMPLETE — 389 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 13984 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1507. `crates/horizon-client/src/game/world_loading.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4755e3703615beb548f1dbd5cc8ec9965c5ce43f`
- Size: 35797 bytes; 878 logical lines
- Coverage: 1–400, 401–800, 801–878 (EOF); empty probe 1201–1600
- Purpose: Local-world creation/loading, level.dat recovery/upgrade/locking, data fixes, registry/dimension/resource/generator initialization and worker lifecycle.
- Dependencies / cross-references: Save/level.dat; DataFixer; registries/dimensions; resource packs/loot; generator/chunks; streaming; task worker and Android lifecycle.
- Implementation / logic: Runs a cancellable load task, repairs/updates save metadata, initializes world resources/generator and builds local world/streaming state.
- Findings: Useful local-load architecture, but save/version/renderer contracts are snapshot-specific and it is not remote server connection code.
- RustChunk relevance: Strong Android lifecycle reference, separate from multiplayer handshake.
- Decision: adaptable; rewrite required; version-specific
- 1.20.1 implications: Build a 1.20.1 local-world coordinator with target DataVersion and bounded Android memory/thread use; separate remote join.
- Follow-up checks: Test corrupted saves, locks, phase cancellation, worker failure, resource reload, partial cleanup and low-memory/background transitions.
- RustChunk implementation mapping: Android asynchronous world bootstrap and legacy 1.20.1 save adapter.
- Evidence required before adoption: 1.20.1 DataFixer/save fixtures, fault injection, cancellation cleanup and SM-A225F lifecycle tests.
- Completion state: COMPLETE — 878 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 35797 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1508. `crates/horizon-client/src/game/zombie_reinforcements.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f704da901865c154642028b44771b9441220eeca`
- Size: 5184 bytes; 135 logical lines
- Coverage: 1–135 (EOF)
- Purpose: Zombie reinforcement calls: difficulty/attribute gates, sampled offsets and spawn validation.
- Dependencies / cross-references: Reinforcement attribute; local difficulty; spawn rules; player distance; collision/light/liquid; RNG.
- Implementation / logic: Checks probability and candidate positions then validates and adds reinforcement mobs.
- Findings: Vanilla 1.20.1; exact attributes, RNG order and spawn conditions are version-bound.
- RustChunk relevance: Useful local gameplay simulation; server authority must be preserved.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 attribute/difficulty and spawn checks with Java RNG semantics.
- Follow-up checks: Test difficulty boundaries, modifiers, candidate offsets, player range, light/collision/liquid and caller limits.
- RustChunk implementation mapping: 1.20.1 zombie reinforcement spawn routine.
- Evidence required before adoption: 1.20.1 source, seeded spawn fixtures, attribute tests and spawn traces.
- Completion state: COMPLETE — 135 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5184 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1509. `crates/horizon-client/src/game/zombie_villagers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1922d264529348be89664f2af21d476860274226`
- Size: 10804 bytes; 279 logical lines
- Coverage: 1–279 (EOF)
- Purpose: Zombie-villager conversion and cure: weakness/golden apple, timer and restoration of profession/trades/XP/gossip/equipment.
- Dependencies / cross-references: Villager/zombie-villager data; difficulty; effects; golden apple; offers/gossip; player UUID; conversion events.
- Implementation / logic: Applies difficulty-dependent conversion, starts cure and restores villager state when cured.
- Findings: Vanilla 1.20.1; chance, persistence and restored villager data require target parity.
- RustChunk relevance: Important survival progression and conversion behavior.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 legacy NBT, conversion chances, cure timer and restoration rules.
- Follow-up checks: Test difficulty conversion, cure start/cancel, unload/reload, trades/XP/gossip/equipment restoration.
- RustChunk implementation mapping: 1.20.1 zombie-villager conversion/cure state machine.
- Evidence required before adoption: 1.20.1 mappings, before/during/after cure fixtures, seeded conversion tests and event traces.
- Completion state: COMPLETE — 279 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 10804 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1510. `crates/horizon-client/src/lod_net/client.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2466636e0c0ea73b3b61f7e4657ce606f6b47da8`
- Size: 10114 bytes; 285 logical lines
- Coverage: 1–285 (EOF)
- Purpose: Client half of Horizon custom LOD channel: distant tile requests, throttling, in-flight tracking and timeout/epochs.
- Dependencies / cross-references: Custom horizon:lod messages; request queue; sampler; timeout/epoch and server extension.
- Implementation / logic: Requests distant tile data and manages limits/retries/responses.
- Findings: Custom Horizon protocol, not vanilla 1.20.1 and not guaranteed on Forge/Aternos servers.
- RustChunk relevance: Conceptual scheduling reference only; does not solve authoritative near-chunk delivery.
- Decision: unnecessary; reference; custom protocol
- 1.20.1 implications: Never require/send horizon:lod to ordinary 1.20.1 servers; derive client LOD only from legitimately received chunk data.
- Follow-up checks: Test absent extension, request caps/timeouts, stale epochs and dimension changes.
- RustChunk implementation mapping: Optional client-only LOD scheduler, separate from vanilla protocol.
- Evidence required before adoption: 1.20.1 protocol captures, extension negotiation spec and mobile LOD visual/memory tests.
- Completion state: COMPLETE — 285 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 10114 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1511. `crates/horizon-client/src/lod_net/server.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c97b735f048e65acb6629e52b12d0438ba83caf2`
- Size: 14244 bytes; 401 logical lines
- Coverage: 1–400, 401 (EOF)
- Purpose: Server half of Horizon custom LOD: per-client quotas, tokens/strikes, worker queue, sampling and response delivery.
- Dependencies / cross-references: Custom horizon:lod protocol; connected-client limits; tile sampler; worker/queue and response writer.
- Implementation / logic: Rate-limits and schedules custom distant-data requests, returns tile responses and tracks abuse.
- Findings: Server-only Horizon extension, not vanilla 1.20.1 networking; ordinary servers do not provide it.
- RustChunk relevance: No client baseline use; potential separate server extension reference.
- Decision: unnecessary; reference; custom protocol
- 1.20.1 implications: Exclude from client baseline; do not assume Aternos/Forge servers deploy this extension.
- Follow-up checks: If paired server mod is planned, test auth/negotiation, quotas, queue saturation, cancellation and malformed requests.
- RustChunk implementation mapping: No client mapping; optional separate server-extension project.
- Evidence required before adoption: Published extension spec, server implementation, fuzzing, quota tests and operator deployment.
- Completion state: COMPLETE — 401 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 14244 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1512. `crates/horizon-client/src/menu.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `58fb73c24321f035fa886b865cbc8447dc1d95e7`
- Size: 155236 bytes; 3895 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–3895 (EOF); empty probe 3896–4295
- Purpose: Local menu/slot engine: click types, slot rules, quick-move, swap, clone, throw, drag, crafting results/remainders, containers and station menu models.
- Dependencies / cross-references: Menu/slot types; modern ItemStack/components; recipes; cursor; local inventories/block entities; remote click wrappers and packet interface.
- Implementation / logic: Implements local pickup/quick-move/swap/clone/throw/quick-craft/pickup-all and computes recipe/result/remainder behavior.
- Findings: Important reference for 1.20.1 menus, but modern item data/local simulation cannot replace remote server-authoritative Forge menu transactions.
- RustChunk relevance: Core inventory UX; correctness is essential for multiplayer.
- Decision: rewrite required; version-specific; core reference
- 1.20.1 implications: Implement target 1.20.1 menu/slot IDs, state IDs, legacy ItemStack, ContainerClick packet semantics and authoritative server corrections.
- Follow-up checks: Test click modes, shift-click maps, crafting remainders, drag, cursor recovery, state rollback, close/reconnect and packet bytes.
- RustChunk implementation mapping: RustChunk local menu model plus distinct 1.20.1 remote transaction layer.
- Evidence required before adoption: 1.20.1 mappings/protocol captures, per-menu slot maps, click goldens and resync/race tests.
- Completion state: COMPLETE — 3895 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 155236 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1513. `crates/horizon-client/src/menu/bundle.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `78a52655a0c89dfa9a1f5913526f9e3ac05e42d0`
- Size: 4260 bytes; 105 logical lines
- Coverage: 1–105 (EOF)
- Purpose: Bundle item insert/extract operations, click rules and sounds.
- Dependencies / cross-references: Bundle item and modern component data; ItemStack; slot acceptance; cursor and sound.
- Implementation / logic: Moves stacks into/out of bundle according to component-based capacity and menu rules.
- Findings: Released vanilla bundle behavior is later than 1.20.1; modern component assumptions are outside target baseline.
- RustChunk relevance: No 1.20.1 vanilla requirement; could be mod-specific.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude bundle item/component/menu path from vanilla 1.20.1 unless a specific Forge mod registers it.
- Follow-up checks: For a named mod, verify item registry, NBT, capacity, click semantics and synchronization.
- RustChunk implementation mapping: No vanilla mapping; optional isolated item extension.
- Evidence required before adoption: Specific Forge mod source/version, item registry/NBT fixtures and packet traces.
- Completion state: COMPLETE — 105 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 4260 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1514. `crates/horizon-client/src/menu/place.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5deb1fdc2e1e1f5ed2bc9022803323f3f4b7c183`
- Size: 25749 bytes; 707 logical lines
- Coverage: 1–400, 401–707 (EOF)
- Purpose: Recipe ingredient placement into crafting/furnace grids, ingredient matching, inventory extraction, clearing and recipe-book ghosts.
- Dependencies / cross-references: Recipe manager/ingredient predicates; StackedContents; inventory/menu slots; grids; recipe book.
- Implementation / logic: Plans ingredient allocation from available stacks and places/clears grid contents for recipe-book interactions.
- Findings: Relevant to 1.20.1, but target recipe/item/menu schemas and packet behavior are required.
- RustChunk relevance: Useful crafting UX once remote menu synchronization works.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 recipes/ingredients, legacy ItemStack and target recipe-book/menu packets; server remains authoritative for crafting.
- Follow-up checks: Test shaped/shapeless, duplicate ingredients, stack counts, remainders, partial inventory, grid clearing and server corrections.
- RustChunk implementation mapping: 1.20.1 recipe-placement planner over legacy ItemStack and remote slots.
- Evidence required before adoption: 1.20.1 recipe fixtures, deterministic allocation tests, slot maps and recipe-book packet captures.
- Completion state: COMPLETE — 707 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 25749 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1515. `crates/horizon-client/src/merchant.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f64cce912935bfd02ddcdbfc30c87bbe53cab06b`
- Size: 44859 bytes; 1188 logical lines
- Coverage: 1–400, 401–800, 801–1188 (EOF)
- Purpose: Merchant offers/session and trade generation, price/demand/reputation/discount logic, XP and trade events.
- Dependencies / cross-references: Villager profession/level; modern trade registries/components; ItemStack/loot functions; RNG; map/enchantment/potion data; reputation.
- Implementation / logic: Builds offers, calculates prices/discounts, processes trades and advances merchant XP/level events.
- Findings: Trading is 1.20.1, but this version's registry/loot/component algorithm is not assumed equivalent to target tables or RNG.
- RustChunk relevance: Relevant gameplay and remote merchant UX; requires target implementation.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Reimplement 1.20.1 trade tables, legacy costs/results/map NBT, demand/reputation/discount formulas, XP and target merchant packets.
- Follow-up checks: Verify offers by profession/level, RNG order, restock/depletion, reputation, discounts, XP and remote select/click packets.
- RustChunk implementation mapping: 1.20.1 merchant engine and Android trade UI over legacy menu adapter.
- Evidence required before adoption: 1.20.1 trade tables, seeded fixtures, map/item NBT goldens and packet traces.
- Completion state: COMPLETE — 1188 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 44859 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1516. `crates/horizon-client/src/mobs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e33409997a876ee2074a66e786facce040850daf`
- Size: 173582 bytes; 4649 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4649 (EOF)
- Purpose: Shared Mob data model, species/type metadata, attributes, damage/events, synchronized extra state, lifecycle/tick helpers and model/render structures.
- Dependencies / cross-references: Entity type/attributes; ItemStack/equipment; entity metadata/NBT; damage/effects; world/physics; renderer/model parts and animation.
- Implementation / logic: Defines shared mob state/species, damage/armor/knockback/effects, equipment/extra data, event queues, movement/lifecycle helpers and render-facing model data.
- Findings: Fundamental engine reference, but later species/events and modern death/item components require strict filtering and target metadata/NBT mappings.
- RustChunk relevance: Very high relevance to native Rust game core; direct reuse brings snapshot-specific assumptions.
- Decision: rewrite required; version-specific; core reference
- 1.20.1 implications: Define target 1.20.1 species/entity metadata and legacy NBT; port only target attributes/damage/effects and keep Forge mod entities behind extension interfaces.
- Follow-up checks: Check target species, dimensions/attributes, metadata indices, passengers/leashes/equipment, damage/effects, NBT, ID reuse and update order.
- RustChunk implementation mapping: 1.20.1 entity model, metadata codec, legacy NBT serializer and renderer-facing state.
- Evidence required before adoption: 1.20.1 mappings/type data, protocol metadata captures, NBT goldens and deterministic combat/movement tests.
- Completion state: COMPLETE — 4649 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 173582 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1517. `crates/horizon-client/src/mobs/ai.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c1831baa2f7cea12f1ef73eddc64fd167fcfd0b5`
- Size: 158104 bytes; 3969 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–3969 (EOF); empty probe 3970–4369
- Purpose: Mob AI/tick orchestration: goals/brains, movement, collision/pushing, despawn, age, effects, attacks, sounds and species actions.
- Dependencies / cross-references: Mob model/goals/targets; navigation/controls; attributes/damage/effects; world sensing; species modules; events and RNG.
- Implementation / logic: Runs tick stages and goal/brain dispatch, movement/push/despawn/age/effects, melee and species-specific overrides in deterministic order.
- Findings: Many routines match 1.20.1 but newer species/actions and exact tick/RNG order require target-specific reconstruction.
- RustChunk relevance: Core simulation reference, not a substitute for remote server authority.
- Decision: rewrite required; version-specific; core reference
- 1.20.1 implications: Build strict 1.20.1 species dispatch and reproduce goal/tick order, RNG, collision/damage/effect timing and despawn; omit later branches.
- Follow-up checks: Test target species goal priorities, NoAI, sunlight/fire, effects, target selection, pushes, despawn, age/breeding and seeded traces.
- RustChunk implementation mapping: 1.20.1 mob AI scheduler and species behavior modules.
- Evidence required before adoption: 1.20.1 source/mappings, tags, seeded tick replay, goal traces and combat/collision goldens.
- Completion state: COMPLETE — 3969 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 158104 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1518. `crates/horizon-client/src/mobs/allay.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `229b427eb92e8654fa68bdb66642cefbc8c5f844`
- Size: 18763 bytes; 504 logical lines
- Coverage: 1–504 (EOF)
- Purpose: Allay item fetching/delivery, liked-player/note-block memory, vibration listening, dancing and animation behavior.
- Dependencies / cross-references: Allay NBT; ItemStack/potions; vibration/game events; note blocks/jukeboxes; navigation/goals; player UUID; animation.
- Implementation / logic: Finds matching drops, carries/delivers items, tracks liked note block/player, processes vibrations and updates healing/cooldowns/animation.
- Findings: Allay is 1.20.1; item equality, NBT, vibration tags and event details must match target.
- RustChunk relevance: Useful target vanilla mob reference and visible client behavior.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port target item/NBT equality, pickup/delivery, note-block memory and vibration tags; verify duplication/event rules against 1.20.1 source.
- Follow-up checks: Test potion/stack matching, pickup reach, memory persistence, jukebox dance, duplication cooldown, vibration occlusion and dimension change.
- RustChunk implementation mapping: 1.20.1 Allay behavior and vibration event adapter.
- Evidence required before adoption: 1.20.1 mappings/tags, NBT/item fixtures, seeded vibration tests and delivery traces.
- Completion state: COMPLETE — 504 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 18763 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1519. `crates/horizon-client/src/mobs/ambient.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8cf710233b362b9894d757464bc0e5c3781b2a4e`
- Size: 4804 bytes; 121 logical lines
- Coverage: 1–121 (EOF)
- Purpose: Client-only mob ambient visuals/sounds: portal particles, Blaze smoke/burn, Phantom wing dust and Bee nectar particles.
- Dependencies / cross-references: Client mob copy/animation; client RNG; particle/sound events; Bee nectar state.
- Implementation / logic: Generates local cosmetic events only; no server/network effects are sent.
- Findings: Mostly optional cosmetic behavior; filter species and particle IDs to 1.20.1.
- RustChunk relevance: Optional visual polish, not multiplayer interoperability.
- Decision: adaptable; client-only; optional
- 1.20.1 implications: Keep on client with 1.20.1 particle/sound registries and client RNG; exclude later-only species/effects.
- Follow-up checks: Test counts/positions, silence, RNG source, remote entity animation and no duplicate server events.
- RustChunk implementation mapping: Client-only ambient effect scheduler and renderer event sink.
- Evidence required before adoption: 1.20.1 particle/sound data, deterministic RNG fixtures and visual tests.
- Completion state: COMPLETE — 121 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 4804 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1520. `crates/horizon-client/src/mobs/anim.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2b56b9b6b486f8ab77d9fb4cb93815debbc47273`
- Size: 57201 bytes; 1547 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1547 (EOF); empty probe 1548–1947
- Purpose: Pose maps/pools, interpolated render state, humanoid/item arm poses, swim/walk cycles and per-species model animation.
- Dependencies / cross-references: Mob animation clocks/model Parts; renderer pose cache; item-use/swing/equipment; species animation modules.
- Implementation / logic: Builds/reuses pose maps, interpolates clocks, applies model transforms/visibility and dispatches setupAnim routines.
- Findings: Target model families apply, but later species/poses (Armadillo, Copper Golem, Happy Ghast, Creaking, Nautilus, spear, brush and others) must be removed or gated.
- RustChunk relevance: Relevant for Android entity visuals; pooled pose structures may help avoid per-frame allocation.
- Decision: adaptable; rewrite required; client rendering
- 1.20.1 implications: Keep 1.20.1 model families/poses; exclude later species/weapon branches and preserve allocation-conscious pose reuse and partial-tick interpolation.
- Follow-up checks: Test target models, held-item/shield/bow/trident/crossbow poses, spectator visibility, baby scale, swimming and Mali-G52 allocation/FPS.
- RustChunk implementation mapping: 1.20.1 mob pose layer with pooled transform buffers and Android renderer integration.
- Evidence required before adoption: 1.20.1 model assets/mappings, pose goldens, per-frame allocation profiling and device visual tests.
- Completion state: COMPLETE — 1547 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 57201 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1521. `crates/horizon-client/src/mobs/animal_brain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `06dcdb0574905968262112c78d79b6e76c274ccc`
- Size: 101411 bytes; 3050 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3050 (EOF); empty probe 3051–3450
- Purpose: Memory/sensor animal brains and behavior scheduling for Sniffer, Axolotl and later brain-driven mobs; breeding, temptation, panic, navigation and state changes.
- Dependencies / cross-references: Brain memories/TTL; sensors and visibility; behavior scheduler; path/navigation; Axolotl/Sniffer states; item/player/world; RNG/events.
- Implementation / logic: Ticks memories/sensors, schedules prioritized behaviors, chooses activities and handles breeding, play-dead, sniff/dig and path targets.
- Findings: Axolotl and Sniffer brain behavior matter in 1.20.1; current file adds Copper Golem/Happy Ghast and a later Tadpole brain redesign. Tadpoles are 1.20.1 content, but keep their target goal-based AI rather than this redesign.
- RustChunk relevance: Useful target Axolotl/Sniffer AI reference; must split from newer species and scheduling changes.
- Decision: rewrite required; version-specific; core reference
- 1.20.1 implications: Rebuild 1.20.1 Axolotl/Sniffer memories, sensors, activity order and RNG. Implement 1.20.1 Tadpole using target goals; exclude Copper Golem/Happy Ghast.
- Follow-up checks: Compare memory TTL, scan cadence, visibility/ranges, activity priority, panic, temptation, breeding, Axolotl play-dead and Sniffer dig transitions.
- RustChunk implementation mapping: 1.20.1 Axolotl/Sniffer brain modules and distinct goal-based Tadpole AI.
- Evidence required before adoption: 1.20.1 sources/tags, seeded behavior traces, TTL tests and saved entity-state fixtures.
- Completion state: COMPLETE — 3050 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 101411 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1522. `crates/horizon-client/src/mobs/aquatic.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `091a2fafd45614226c4075dec61950f8b974e057`
- Size: 45978 bytes; 1229 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1229 (EOF); empty probe 1232–1631
- Purpose: Aquatic mob registration, squid/glow-squid movement and ink, fish schooling, pufferfish puff/sting, dolphin breathing/swimming and animations.
- Dependencies / cross-references: Aquatic species/attributes; fluids/pathfinding; goals; school state; damage/particles/sounds; model animation.
- Implementation / logic: Registers movement/navigation/goals and handles aquatic travel, land flops, schooling, pufferfish threat/sting, squid ink and per-species rendering.
- Findings: Most systems and Tadpole content are 1.20.1; later species/tag entries (Nautilus, ZombieNautilus, SulfurCube) must be filtered.
- RustChunk relevance: Strong 1.20.1 gameplay reference for water movement, schools, aquatic combat and visuals.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port target species, fluid physics, air/moisture, schooling and pufferfish/squid rules; retain 1.20.1 Tadpole but filter later types.
- Follow-up checks: Test water/land movement, flop, school size/leadership, pufferfish sting timing/immunity, squid ink/tentacles, dolphin air/moisture and reload.
- RustChunk implementation mapping: 1.20.1 aquatic behavior modules, fluid movement adapter and school manager.
- Evidence required before adoption: 1.20.1 mappings/tags, seeded movement/school tests, sting fixtures and entity packet traces.
- Completion state: COMPLETE — 1229 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 45978 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1523. `crates/horizon-client/src/mobs/armadillo.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `83a4077b99ae4531b0196e18851b3ff40e15204c`
- Size: 24776 bytes; 753 logical lines
- Coverage: 1–400, 401–753 (EOF); empty probe 754–1153
- Purpose: Armadillo fear/shell state machine, threat sensing, rolling/unrolling/peek, damage reduction, breeding and baby-follow goals.
- Dependencies / cross-references: Armadillo metadata/animation; damage/panic tags; nearby players/undead; goals/breeding; block support.
- Implementation / logic: Scans threats, enters shell states, peeks while danger persists, reduces damage and gates movement/breeding.
- Findings: Armadillo was added after 1.20.1 (1.20.5), so it is not target vanilla content.
- RustChunk relevance: Only support for an explicitly identified Forge 1.20.1 backport.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude Armadillo entity, metadata, model, fear tags and damage from vanilla 1.20.1.
- Follow-up checks: For a specific backport, verify registry/NBT, panic tags, shell damage and animation events.
- RustChunk implementation mapping: No vanilla mapping; optional mod-specific entity adapter.
- Evidence required before adoption: Named Forge 1.20.1 backport, license, metadata fixtures and authoritative behavior tests.
- Completion state: COMPLETE — 753 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 24776 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1524. `crates/horizon-client/src/mobs/axolotl.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cd6555c3fffa0fc05809ab2aceb757c79544c4cc`
- Size: 7051 bytes; 207 logical lines
- Coverage: 1–207 (EOF)
- Purpose: Axolotl prey eligibility/hunting cooldown, play-dead damage roll, support-player event and baby/adult animation states.
- Dependencies / cross-references: Axolotl entity state; damage/RNG; animal brain; prey tags; animation/events.
- Implementation / logic: Evaluates aquatic prey, applies ordered random play-dead checks, triggers support-player event on kills and updates animation state.
- Findings: Axolotl is vanilla 1.20.1; exact target prey tags, random order and events must match.
- RustChunk relevance: Relevant target mob behavior and client presentation.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use target prey tags/range, play-dead timing/RNG, hunting cooldown and support-player trigger integrated with 1.20.1 Axolotl AI.
- Follow-up checks: Test prey water/range/hostility/cooldown, damage gates, RNG order, play-dead exit, support radius and baby poses.
- RustChunk implementation mapping: 1.20.1 Axolotl state/behavior integrated with target brain and animation.
- Evidence required before adoption: 1.20.1 mappings/tags, seeded damage fixtures, cooldown tests and pose/event goldens.
- Completion state: COMPLETE — 207 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7051 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1525. `crates/horizon-client/src/mobs/bat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bd20ab9008b6b0b9f845f33fbbf54fe0b6ea51f3`
- Size: 4987 bytes; 147 logical lines
- Coverage: 1–147 (EOF)
- Purpose: Bat resting/hanging, wake-up, target-air selection and flying movement, takeoff effect and animations.
- Dependencies / cross-references: Bat resting metadata; redstone-conductor tag; nearby player visibility; movement RNG; sound and animation.
- Implementation / logic: Hangs under a solid conductor, wakes for nearby players or invalid support, chooses air targets and updates movement/visual states.
- Findings: Bat behavior is vanilla 1.20.1; support tag, player detection and synchronized state need target parity.
- RustChunk relevance: Relevant vanilla ambient mob behavior and lightweight visuals.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port 1.20.1 resting metadata/support, player wake-up, target sampling and vertical drag with target RNG/state sync.
- Follow-up checks: Test support removal, player/spectator range, takeoff sound/silence, target refresh, world-bottom and entity metadata updates.
- RustChunk implementation mapping: 1.20.1 Bat behavior and renderer animation adapter.
- Evidence required before adoption: 1.20.1 mappings/tag fixtures, seeded target tests and entity metadata/event packet captures.
- Completion state: COMPLETE — 147 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 4987 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1526. `crates/horizon-client/src/mobs/bee.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cb04324e25c0411e469cbc0a9e73086ff327cd50`
- Size: 40659 bytes; 1244 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1244 (EOF); empty EOF probe 1245–1644
- Purpose: Implements Bee goals and state: flower seeking, nectar collection, hive entry/exit, breeding/temptation, anger persistence, stinging/poison, crop pollination and hive/home behavior.
- Dependencies / cross-references: Bee synchronized flags and NBT; flower/hive blocks and tags; item/player/entity sensors; navigation/goals; anger targets; effects/damage; particles/sounds and world events.
- Implementation / logic: Registers and ticks bee behavior, tracks hive/flower targets and travel, collects nectar, returns to hive, propagates anger, stings targets and emits effects/events for pollination and crop growth.
- Findings: Bee behavior is part of vanilla 1.20.1; exact anger persistence, hive capacity/occupancy, flower tags, item/effect and Java RNG order need target-era parity.
- RustChunk relevance: Important vanilla mob and world-interaction behavior; local simulation must remain distinct from remote server authority.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild with 1.20.1 Bee NBT/metadata, flower and hive tags, anger timers/UUID, nectar and sting/poison timings; do not import newer item-component assumptions unverified.
- Follow-up checks: Test hive entrance/exit and capacity, invalid/blocked hive, flower search/range, nectar persistence, anger target/logout, sting/poison duration, death after sting, breeding and crop growth.
- RustChunk implementation mapping: RustChunk 1.20.1 Bee state/goal module, hive POI adapter and client effect renderer.
- Evidence required before adoption: 1.20.1 mappings/tags, bee/hive NBT fixtures, seeded goal traces, sting/effect tests and server event captures.
- Completion state: COMPLETE — 1244 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 40659 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1527. `crates/horizon-client/src/mobs/body_anim.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d31603156afe30e41f39313884107eb215cc5981`
- Size: 37234 bytes; 1056 logical lines
- Coverage: 1–400, 401–800, 801–1056 (EOF); empty EOF probe 1057–1456
- Purpose: Contains species-specific body/model animation setup for many mobs, including movement cycles, head/body/limb poses, baby variants, attacks, sitting, swimming and special states.
- Dependencies / cross-references: Mob animation state and partial ticks; model-part hierarchy; species metadata/flags; renderer pose buffers; texture/model definitions and entity-specific animation modules.
- Implementation / logic: Computes interpolated model-part rotations, offsets and visibility for the listed mob families and blends locomotion with species-specific actions and states.
- Findings: Many 1.20.1 species are included, but this snapshot also contains later entities such as Armadillo, Creaking, Copper Golem, Happy Ghast and Nautilus; all must be filtered against target registries/models.
- RustChunk relevance: Client-only visual layer with strong mobile relevance, but not required for the first network join.
- Decision: adaptable; client-rendering; version-specific
- 1.20.1 implications: Keep only the 1.20.1 model families and synchronized animation flags; remove later-only species/poses and avoid allocations or expensive transforms per frame on Android.
- Follow-up checks: Compare all target species, baby/adult scaling, attack/sit/swim poses, partial-tick interpolation, hidden parts, first-person/spectator visibility and per-frame allocation/FPS.
- RustChunk implementation mapping: RustChunk 1.20.1 body animation dispatch and pooled Android pose buffers.
- Evidence required before adoption: 1.20.1 model/mapping/assets, pose golden fixtures, render regression screenshots and Mali-G52 allocation/performance profiles.
- Completion state: COMPLETE — 1056 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 37234 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1528. `crates/horizon-client/src/mobs/brain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `67bb6a2a661ce146cdfab02cf2aa1fa65a108883`
- Size: 5896 bytes; 190 logical lines
- Coverage: 1–190 (EOF)
- Purpose: Provides shared vanilla Brain primitives: expiring memory TTL, timed behavior bookkeeping, nearby-living-entity sensing, walk targets, and MoveToTargetSink navigation.
- Dependencies / cross-references: Mob memory store and age/tick; level and entity-nearby queries; goal context/navigation/pathfinding; Java-compatible random duration sampling.
- Implementation / logic: Implements memory expiry, behavior start/end timing, nearest living entity ordering, Manhattan target completion and bounded navigation with fallback random pathing.
- Findings: The primitives are relevant to 1.20.1 brain-driven entities, but exact TTL tick order, random source and call ordering must be matched to target species brains; it is not the complete Brain/behavior system.
- RustChunk relevance: Useful shared simulation foundation for Sniffer/Axolotl and other 1.20.1 brain users.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild the 1.20.1 memory/behavior contracts and preserve the exact set/tick/expire convention, level-vs-entity RNG source, target-distance rule and navigation timeout order.
- Follow-up checks: Test TTL inclusive tick, timed behavior duration draws, nearest-entity ordering/ties, walk-target Manhattan threshold, path failure fallback and cancellation.
- RustChunk implementation mapping: RustChunk 1.20.1 brain memory primitives and movement-to-target sink.
- Evidence required before adoption: 1.20.1 Brain/Sensor mappings, deterministic RNG fixtures, expiry boundary tests and navigation replay traces.
- Completion state: COMPLETE — 190 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5896 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1529. `crates/horizon-client/src/mobs/breeze.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `66f8b4607bd50f65ce808fbdd50aeb4ee74fd19c`
- Size: 10669 bytes; 310 logical lines
- Coverage: 1–310 (EOF)
- Purpose: Implements Breeze AI, wind charge attacks, jumping/projectile behavior, knockback and associated movement/effects.
- Dependencies / cross-references: Breeze entity and attributes; wind-charge projectile; combat/target goals; collision and knockback; particles/sounds and later entity data.
- Implementation / logic: Registers/ticks Breeze goals, selects targets, performs jumps/attacks and creates wind-charge attacks with hit/knockback effects.
- Findings: Breeze and Wind Charge are 1.21 Trial Chambers content, absent from vanilla Minecraft 1.20.1.
- RustChunk relevance: No vanilla baseline use; only potentially applicable through an explicitly chosen backport mod.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude Breeze species, wind-charge item/projectile, entity metadata and related packet/effect assumptions from the 1.20.1 baseline.
- Follow-up checks: Only reconsider for a named Forge 1.20.1 backport; verify registry, attributes, projectile packets, damage/knockback, tags and server authority.
- RustChunk implementation mapping: No vanilla mapping; isolated optional mod-specific entity/projectile adapter.
- Evidence required before adoption: Named Forge 1.20.1 backport source/version, license, registry/NBT fixtures and multiplayer projectile tests.
- Completion state: COMPLETE — 310 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 10669 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1530. `crates/horizon-client/src/mobs/camel.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `14fe8d289af4346cad7e5d52a83e0798010afa12`
- Size: 25239 bytes; 683 logical lines
- Coverage: 1–400, 401–683 (EOF); empty EOF probe 684–1083
- Purpose: Implements Camel behavior including sitting/standing timers, dash/jump, rider movement, passenger positioning, goals, breeding/temptation and animation.
- Dependencies / cross-references: Camel entity state and synchronized pose; player/rider input; goals/navigation; collision and movement; breeding/items; passenger seat/animation.
- Implementation / logic: Ticks sit/stand transitions, refuses movement in specified states, processes dash and rider input, controls passenger pose and runs Camel AI.
- Findings: Camel is vanilla 1.20.1; CamelHusk and other later mounted species/branches are not part of the target baseline.
- RustChunk relevance: Relevant 1.20.1 mount/gameplay behavior, but not a substitute for remote vehicle packet handling.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port 1.20.1 Camel state, sit/stand timings, dash stamina/cooldown, rider movement and passenger seat rules; exclude CamelHusk and newer mount integrations.
- Follow-up checks: Test sit/stand trigger and timing, dash impulse/cooldown, rider jump/sprint, passenger alignment, leash effects, breeding and client/server vehicle movement packets.
- RustChunk implementation mapping: RustChunk 1.20.1 Camel AI/state module and legacy remote-vehicle movement adapter.
- Evidence required before adoption: 1.20.1 mappings/NBT, deterministic dash and sit tests, passenger position goldens and captured MoveVehicle traces.
- Completion state: COMPLETE — 683 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 25239 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1531. `crates/horizon-client/src/mobs/caravan.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `15f01d59e2b69d333a6420544e9b728544c4790b`
- Size: 8367 bytes; 273 logical lines
- Coverage: 1–273 (EOF)
- Purpose: Implements llama caravan membership and follow behavior: joining/leaving a leader, follower chain selection, distance checks and movement toward preceding llamas.
- Dependencies / cross-references: Llama/trader-llama entity state; caravan leader/follower references; navigation goals; nearby entity queries; entity removal/death lifecycle.
- Implementation / logic: Finds or maintains a caravan leader, updates followers and navigation, and cleans up invalid/disconnected/dead caravan links.
- Findings: Llama caravans are vanilla 1.20.1 behavior; exact follow range, chain order and cleanup semantics need target source comparison.
- RustChunk relevance: Useful vanilla mob-AI behavior, not relevant to external server handshake itself.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use target 1.20.1 caravan data and leader/follower rules, ensuring stale links are cleared on death, unload, dimension change and entity removal.
- Follow-up checks: Test joining/removing leaders, chain order, range boundaries, unloaded chunks, leader death, despawn, trader-lama conversion and re-acquisition.
- RustChunk implementation mapping: RustChunk 1.20.1 Llama caravan link manager and navigation goal.
- Evidence required before adoption: 1.20.1 source/mappings, seeded caravan fixtures, unload/death lifecycle tests and multiplayer entity traces.
- Completion state: COMPLETE — 273 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 8367 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1532. `crates/horizon-client/src/mobs/cem_anim.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5829365c0790c14184d1814733c1e642bc940aa4`
- Size: 45878 bytes; 1184 logical lines
- Coverage: 1–400, 401–800, 801–1184 (EOF); empty EOF probe 1185–1584
- Purpose: Parses and evaluates custom entity model animation expressions and channels for CEM/OptiFine-compatible resource-pack animations.
- Dependencies / cross-references: CEM animation JSON; model parts/variables; renderer frame state; math/expression evaluator; entity and world properties; custom resource-pack assets.
- Implementation / logic: Loads animation expressions and keyframes, evaluates variable bindings and animation curves, then applies transformations to named model parts during rendering.
- Findings: Custom entity model animation is optional resource-pack compatibility, not vanilla 1.20.1 simulation or Forge support; slot use/global variables/frame lifetime require auditing.
- RustChunk relevance: Potential future client-rendering feature for custom resource packs; not a first multiplayer milestone.
- Decision: **undetermined; reference; optional/future; rewrite required** — reference; optional/future; rewrite required
- 1.20.1 implications: Do not port wholesale into the game core; if CEM support is selected, implement the target OptiFine/CEM expression subset and connect it through an isolated resource-pack renderer interface.
- Follow-up checks: Test expression parsing, variable scope/reset, animation start/stop, part lookup, invalid resources, concurrent entities, frame-time cost and resource reload cleanup.
- RustChunk implementation mapping: Optional RustChunk CEM animation interpreter and renderer adapter, isolated from simulation.
- Evidence required before adoption: Published CEM specification/behavior fixtures, representative 1.20.1 resource packs, parser fuzzing and Android frame-time/allocation tests.
- Completion state: COMPLETE — 1184 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 45878 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1533. `crates/horizon-client/src/mobs/cem.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9bf2bfdaead087ba01fefc64f4babaf1f65b210f`
- Size: 44540 bytes; 1076 logical lines
- Coverage: 1–400, 401–800, 801–1076 (EOF); empty EOF probe 1077–1476
- Purpose: Loads custom entity models and textures from CEM/OptiFine-style resource packs and resolves model replacements, layers, attachments and fallback behavior.
- Dependencies / cross-references: Resource-pack files and namespaces; entity/model registry; textures and render layers; model-part tree; CEM parser and renderer replacement registry.
- Implementation / logic: Discovers and parses custom model definitions, maps entity identifiers to replacement models/layers and chooses fallback/attachment paths when assets are missing or reload.
- Findings: CEM compatibility is an optional client rendering feature; this implementation depends on its own model/asset pipeline and does not imply Forge mod rendering compatibility.
- RustChunk relevance: Potential future resource-pack feature; should not delay baseline 1.20.1 gameplay/server support.
- Decision: **undetermined; reference; optional/future; rewrite required** — reference; optional/future; rewrite required
- 1.20.1 implications: Keep CEM loading isolated from target entity simulation; if adopted, target the desired OptiFine CEM subset and preserve secure resource-pack path validation and reload behavior.
- Follow-up checks: Test namespace/path traversal, malformed JSON, missing textures, replacement precedence, layer attachment, pack priority, reload/unload and GPU memory on Android.
- RustChunk implementation mapping: Optional RustChunk CEM resource resolver and custom model registry.
- Evidence required before adoption: CEM compatibility fixtures, malicious-pack tests, 1.20.1 asset samples and Mali-G52 render/memory profiling.
- Completion state: COMPLETE — 1076 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 44540 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1534. `crates/horizon-client/src/mobs/combat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c79506441646aea8b994f8167f834fc1f2650c8c`
- Size: 3274 bytes; 95 logical lines
- Coverage: 1–95 (EOF)
- Purpose: Tracks recent mob combat damage history and supplies combat-state/death context, including the retention period used for hurt-by bookkeeping.
- Dependencies / cross-references: Mob damage events and timestamps; attacker/source identity; entity age/tick; death-message attribution.
- Implementation / logic: Adds and expires damage records, tracks recent attackers and determines combat/death context from the retained history.
- Findings: CombatTracker is a vanilla 1.20.1 concept; retention thresholds, environmental damage categories and ordering need target parity.
- RustChunk relevance: Relevant combat correctness and kill attribution; should be a small versioned simulation service.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild the exact 1.20.1 CombatTracker tick/expiry and damage-source attribution rules; preserve order and attacker identity across entity unload/death.
- Follow-up checks: Test record expiry boundaries, multiple attackers, environmental damage, combat exit, death message selection and entity removal.
- RustChunk implementation mapping: RustChunk 1.20.1 combat history tracker and death-attribution service.
- Evidence required before adoption: 1.20.1 mappings, deterministic 100/300-tick cases, multi-attacker fixtures and death-message tests.
- Completion state: COMPLETE — 95 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 3274 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1535. `crates/horizon-client/src/mobs/control.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bccf5c7f3b2488d7d5af5c51f1ff4e0f52e7a489`
- Size: 21510 bytes; 519 logical lines
- Coverage: 1–400, 401–519 (EOF); empty EOF probe 520–919
- Purpose: Implements mob MoveControl, LookControl, JumpControl and body-rotation controllers for ground, flying, swimming and special movement states.
- Dependencies / cross-references: Mob AI control flags and wanted positions; navigation/path state; movement attributes; yaw/pitch; physics and species dispatch.
- Implementation / logic: Consumes wanted movement/look targets, computes acceleration and rotation changes, handles jump triggers and aligns body/head yaw for movement and attack.
- Findings: Core controller concepts apply to 1.20.1, but newer Happy Ghast and other species-specific branches and some control states are outside target.
- RustChunk relevance: Core simulation reference with high relevance to entity motion and consistent client rendering.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild the 1.20.1 controller types and species dispatch; exclude Happy Ghast/Nautilus-era branches and match target interpolation, speed modifiers and rotation limits.
- Follow-up checks: Test wanted-position arrival, path stop, yaw wraparound, head/body limits, ground vs flying/swimming movement, jump edge-trigger and NoAI.
- RustChunk implementation mapping: RustChunk 1.20.1 mob control layer shared by AI scheduler and physics integration.
- Evidence required before adoption: 1.20.1 source/mappings, controller unit fixtures, deterministic movement replay and collision/rotation regression tests.
- Completion state: COMPLETE — 519 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 21510 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1536. `crates/horizon-client/src/mobs/conversion.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ccfd3ce0079a1bbe7310bd841f748b4740aa3c53`
- Size: 6564 bytes; 160 logical lines
- Coverage: 1–160 (EOF)
- Purpose: Implements entity conversion between mob types while preserving selected state, equipment, effects, position, motion, riding and identity-linked data.
- Dependencies / cross-references: Entity type registry; Mob NBT/state; equipment and effects; passengers/vehicle links; damage and conversion reason.
- Implementation / logic: Creates/converts a mob, transfers target-prescribed attributes and state, updates ride relationships and replaces the old entity.
- Findings: Mob conversion (for example Zombie/Drowned, Husk/Zombie and Skeleton/Stray) is relevant to 1.20.1, but exact copied fields and data fixups are target-class-specific.
- RustChunk relevance: Important vanilla lifecycle behavior; remote servers remain authoritative for conversions.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use explicit 1.20.1 conversion tables for each source/target pair, preserving only fields vanilla copies and serializing legacy entity NBT/metadata correctly.
- Follow-up checks: Test each target conversion pair, health/attributes, equipment/effects, custom name, age, passengers, UUID/ID replacement, death/loot and save/reload.
- RustChunk implementation mapping: RustChunk 1.20.1 entity-conversion service with versioned copy-field rules.
- Evidence required before adoption: 1.20.1 mappings/source, before/after NBT goldens, conversion pair matrix and multiplayer entity replacement traces.
- Completion state: COMPLETE — 160 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 6564 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1537. `crates/horizon-client/src/mobs/copper_golem.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c5e0dbece1b3803721ecddbc2dc448472dc355de`
- Size: 7944 bytes; 216 logical lines
- Coverage: 1–216 (EOF)
- Purpose: Implements Copper Golem creation, oxidation/weather state, statue stages, button interaction and copper-golem AI/animation.
- Dependencies / cross-references: Copper Golem entity and copper block oxidation; weather/exposure; buttons; interaction and AI state; newer entity registration.
- Implementation / logic: Updates oxidation/statue stages and interacts with buttons while running Copper Golem goals and associated effects.
- Findings: Copper Golem is not vanilla Minecraft 1.20.1 content; it belongs to a later version/content cycle.
- RustChunk relevance: No baseline vanilla use; only applicable through a specific 1.20.1 Forge backport/mod.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude Copper Golem entity, block interactions, states, metadata, assets and packet assumptions from the 1.20.1 baseline.
- Follow-up checks: Only revisit for a named Forge 1.20.1 mod; validate entity/block registry, oxidation, buttons, AI, NBT and server interaction.
- RustChunk implementation mapping: No vanilla mapping; optional isolated mod entity adapter.
- Evidence required before adoption: Named compatible Forge 1.20.1 mod source/version, license, registry fixtures and authoritative interaction tests.
- Completion state: COMPLETE — 216 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7944 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1538. `crates/horizon-client/src/mobs/creaking_heart.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0437e6d393a039f502fffc02223f0ec213aa46f4`
- Size: 21270 bytes; 609 logical lines
- Coverage: 1–400, 401–609 (EOF); empty EOF probe 610–1009
- Purpose: Implements Creaking Heart block/entity behavior: linked Creaking spawning, day/night activation, resin-related changes and heart-bound entity lifecycle.
- Dependencies / cross-references: Creaking Heart block entity/state; Pale Oak/log/resin blocks; Creaking entity UUID/link; time/weather/world random; spawn and block-update events.
- Implementation / logic: Tracks heart activation, linked protector status and spawn/despawn conditions, updates related blocks and emits entity/block events.
- Findings: Creaking Heart and Pale Garden content arrived after Minecraft 1.20.1 (1.21.4); none of this belongs to vanilla target registries/protocol.
- RustChunk relevance: No baseline vanilla use; only relevant for a concrete backport mod.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude Creaking Heart block entity, Pale Garden tags, linked Creaking lifecycle and event/packet IDs from 1.20.1.
- Follow-up checks: Only reconsider for a named Forge 1.20.1 backport; verify block/entity registrations, UUID persistence, activation timing and world updates.
- RustChunk implementation mapping: No vanilla mapping; optional isolated mod-specific block/entity adapter.
- Evidence required before adoption: Named Forge 1.20.1 backport source, license, NBT/registry fixtures and server lifecycle tests.
- Completion state: COMPLETE — 609 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 21270 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1539. `crates/horizon-client/src/mobs/creaking.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6a9a75341c04b734585f0a38635be97c5a271e9d`
- Size: 13880 bytes; 395 logical lines
- Coverage: 1–395 (EOF)
- Purpose: Implements Creaking mob behavior, freeze-when-looked-at, heart tether/activation, attack reactions and special movement/render state.
- Dependencies / cross-references: Creaking entity metadata; Creaking Heart linkage; player gaze and line-of-sight; damage/target AI; Pale Garden tags and animation.
- Implementation / logic: Checks gaze and heart link, handles activation/freeze/attack interactions and updates the entity's special state.
- Findings: Creaking is 1.21.4 Pale Garden content, not vanilla Minecraft 1.20.1.
- RustChunk relevance: No baseline vanilla use; only for an explicit Forge backport.
- Decision: unnecessary; later-version content
- 1.20.1 implications: Exclude Creaking entity, state metadata, AI, renderer and Creaking Heart protocol from the 1.20.1 baseline.
- Follow-up checks: For a named backport verify registry/NBT, heart link, gaze/LOS, damage rules, metadata and server authority.
- RustChunk implementation mapping: No vanilla mapping; optional mod-specific entity adapter.
- Evidence required before adoption: Named Forge 1.20.1 backport source/version, license and server/client entity-state fixtures.
- Completion state: COMPLETE — 395 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 13880 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1540. `crates/horizon-client/src/mobs/cube.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `188cded78ae4d08beea32bf3533480e6cf95fb5e`
- Size: 19714 bytes; 572 logical lines
- Coverage: 1–400, 401–572 (EOF); empty EOF probe 573–972
- Purpose: Implements Slime and Magma Cube sizing, movement, jumping, splitting, contact damage/knockback, target behavior and animations.
- Dependencies / cross-references: Slime/MagmaCube entity size and attributes; collision; damage and loot; goals/navigation; world difficulty and random source.
- Implementation / logic: Calculates size-based health/damage/speed, performs jumping and target behavior, splits on death and generates contact effects and sounds.
- Findings: Slime/Magma Cube are 1.20.1 vanilla entities, but the source also includes custom/later Sulfur Cube branches which must not enter the baseline.
- RustChunk relevance: Important 1.20.1 mob simulation and combat behavior.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port target Slime/MagmaCube size formula, attributes, splitting and contact rules; remove Sulfur Cube and later-only branches and match legacy entity NBT.
- Follow-up checks: Test size 1–4, split counts/health, peaceful behavior, contact damage/knockback, water, collision, sounds and unload/reload.
- RustChunk implementation mapping: RustChunk 1.20.1 Slime/MagmaCube AI, size/attribute adapter and renderer state.
- Evidence required before adoption: 1.20.1 mappings/source, seeded split tests, damage/attribute fixtures and entity NBT captures.
- Completion state: COMPLETE — 572 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 19714 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1541. `crates/horizon-client/src/mobs/data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b21d68475c0c4cfff139a81d8f7303536fc104ea`
- Size: 11947 bytes; 287 logical lines
- Coverage: 1–287 (EOF)
- Purpose: Loads extracted mob runtime data for block pathfinding/placement, heightmaps, entity/biome tags, villager equipment/hats, CEM and texture/animation metadata.
- Dependencies / cross-references: Generated mob-data.json and other resource exports; block/entity/biome registries; path types and placement rules; renderer/CEM assets.
- Implementation / logic: Parses packaged JSON tables and exposes data used by mob spawn, pathfinding, placement, tags and visual systems.
- Findings: Data-driven architecture is useful, but the extracted files are tied to the source snapshot and likely newer game data; using them unfiltered would introduce incorrect 1.20.1 registries and tags.
- RustChunk relevance: Foundational resource pipeline; high importance to correctness and Android startup size.
- Decision: rewrite required; data-version-specific
- 1.20.1 implications: Generate every mob-data/resource table from exact Minecraft 1.20.1 assets/mappings/tags; attach explicit DataVersion/build provenance and never ship current 26.3 exports as 1.20.1 data.
- Follow-up checks: Verify JSON schema/version, all tag memberships, block path types, heightmap/placement facts, villager gear, CEM data, missing-key fallback and invalid-resource handling.
- RustChunk implementation mapping: RustChunk generated 1.20.1 mob-data pack and versioned validated resource loader.
- Evidence required before adoption: Pinned 1.20.1 data source hashes, generator reproducibility, schema tests, tag/registry diff and packaged APK size checks.
- Completion state: COMPLETE — 287 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 11947 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1542. `crates/horizon-client/src/mobs/dolphin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9c3a276638c06d84f7f6e021ac4d5da3d002d6d4`
- Size: 19782 bytes; 593 logical lines
- Coverage: 1–400, 401–593 (EOF); empty EOF probe 594–993
- Purpose: Implements Dolphin treasure finding, swimming-player gifts/Dolphin's Grace, jumping, item seeking/pickup/play and aquatic goal behavior.
- Dependencies / cross-references: Dolphin AI/goals; buried treasure structure lookup; player swimming/UUID/effects; dropped items and inventory; water physics, sounds and particles.
- Implementation / logic: Finds treasure targets, guides dolphins and players, grants effect on player interaction, retrieves and plays with items, and updates movement/animation.
- Findings: Dolphin behavior is vanilla 1.20.1; any Nautilus/new mount-specific integration in this snapshot is later and must be separated.
- RustChunk relevance: Useful vanilla aquatic AI and visible gameplay feature; not necessary for initial server handshake.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port 1.20.1 treasure target search/range, player gift rules, Dolphin's Grace duration and item goals; exclude Nautilus-era rider integration.
- Follow-up checks: Test treasure search, swim/boat/player eligibility, grace effect and cooldown, item pickup/drop delay, play behavior, land/water air and target loss.
- RustChunk implementation mapping: RustChunk 1.20.1 Dolphin goal module and aquatic effect/event adapter.
- Evidence required before adoption: 1.20.1 mappings/tags, treasure fixtures, seeded goal tests, effect timing and entity packet traces.
- Completion state: COMPLETE — 593 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 19782 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1543. `crates/horizon-client/src/mobs/doors.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `138d86f3eb5eceab732978efd1c13138fef3caf4`
- Size: 11694 bytes; 348 logical lines
- Coverage: 1–348 (EOF)
- Purpose: Implements mob door navigation/interactions, zombie/vindicator door breaking and raid illager door opening, plus Piglin path door management.
- Dependencies / cross-references: Door block/tag registry; ground/wall/amphibious navigation; mob-griefing and difficulty; raids and path nodes; block update events.
- Implementation / logic: Detects wooden doors along paths, opens/closes them and runs timed break-progress events for eligible mobs and raiders.
- Findings: Door goals and zombie/vindicator behavior exist in 1.20.1; exact tag names, raid conditions and time/difficulty gates require target comparison.
- RustChunk relevance: Relevant vanilla mob pathfinding and raid gameplay.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 mob_interactable_doors and wooden-door rules, zombie hard difficulty and raid Vindicator behavior, with exact 240-tick break/event semantics only where target source confirms.
- Follow-up checks: Test iron vs wooden doors, mob_griefing, hard/normal difficulty, raid state, path node transitions, break progress, door close and chunk unload.
- RustChunk implementation mapping: RustChunk 1.20.1 door-interaction goals and block-event bridge.
- Evidence required before adoption: 1.20.1 mappings/tags, deterministic path/difficulty fixtures and server block-event traces.
- Completion state: COMPLETE — 348 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 11694 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1544. `crates/horizon-client/src/mobs/dragon_contacts.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `169f2eb85e12ea03d62415cc781c56ec61036a12`
- Size: 6085 bytes; 181 logical lines
- Coverage: 1–181 (EOF)
- Purpose: Handles Ender Dragon wing/head collision contacts with ordinary mobs, including pushing, damage and team-protection exceptions.
- Dependencies / cross-references: Ender Dragon position/part AABBs; mob bounds/damage tracker; team rules; dragon sitting/recently-hurt state.
- Implementation / logic: Tests dragon wing/head intersections, applies knockback and damage with recent-attacker suppression and team-aware protection.
- Findings: Ender Dragon contacts are vanilla gameplay, but this file explicitly documents parity comparison against Java 26.3 rather than the target 1.20.1 release.
- RustChunk relevance: Potential reference for boss/mob collision; unsafe to claim target parity without version-matched evidence.
- Decision: **undetermined; reference; version-specific; verification required** — reference; version-specific; verification required
- 1.20.1 implications: Rebuild/verify contact geometry and damage against Minecraft 1.20.1 EnderDragon/PartEntity semantics; do not carry later parity assumptions as proven.
- Follow-up checks: Test flying vs perched, both wings, head contacts, team rules, hurt cooldown, dragon recently hurt and multi-mob collision order.
- RustChunk implementation mapping: RustChunk 1.20.1 Ender Dragon part-contact handler and entity collision integration.
- Evidence required before adoption: 1.20.1 source/mappings, exact part AABB fixtures, deterministic contact cases and multiplayer damage traces.
- Completion state: COMPLETE — 181 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 6085 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1545. `crates/horizon-client/src/mobs/drops.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `efa6b43f9ba3e2004625a084f1ac2bab2431b46d`
- Size: 24700 bytes; 734 logical lines
- Coverage: 1–400, 401–734 (EOF); empty EOF probe 735–1134
- Purpose: Builds mob death drops: charged-creeper heads, loot-table rolls, equipment/drop chances, species extras, carried blocks, inventories and experience orbs.
- Dependencies / cross-references: Mob species/equipment; loot tables/conditions; damage source and killer/luck; entity/world random sequences; item stacks/NBT; XP and mob-loot gamerule.
- Implementation / logic: Constructs loot parameters, rolls seeded or sequence-based loot, clears dropped equipment, adds species-specific items and creates XP orb values.
- Findings: Death loot is central 1.20.1 behavior, but current source includes post-target species and modern item component representations/loot facts; random-source order is especially sensitive.
- RustChunk relevance: High survival correctness and server-visible gameplay importance.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild 1.20.1 loot context/conditions, item NBT, equipment chance and durability, species extras and XP; exclude newer species and component-only data while preserving Java RNG order.
- Follow-up checks: Test each 1.20.1 loot table path, player vs environmental kill, mobLoot off, equipment chance, charged-creeper head, Enderman block, horse inventory, XP splits and saved death seeds.
- RustChunk implementation mapping: RustChunk 1.20.1 mob-death loot coordinator and legacy ItemStack/NBT drop adapter.
- Evidence required before adoption: 1.20.1 loot tables/mappings, seeded loot fixtures, drop-order goldens, inventory NBT tests and server loot traces.
- Completion state: COMPLETE — 734 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 24700 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1546. `crates/horizon-client/src/mobs/enderman.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5d94b8b1c03fbbb3fbafe0d59b511f68edbeef3f`
- Size: 26061 bytes; 719 logical lines
- Coverage: 1–400, 401–719 (EOF); empty EOF probe 720–1119
- Purpose: Implements Enderman stare/anger detection, teleport behavior, projectile dodge, water/rain response, block pickup/placement and held-block rendering/death drops.
- Dependencies / cross-references: Enderman target AI; player eye/look/LOS; world border and collision; enderman_holdable tags; fluids; block state and loot; damage/projectiles and event/sound.
- Implementation / logic: Detects stare cone, acquires player targets, teleports under specified triggers, avoids projectile hits, picks up/places eligible blocks and drops carried blocks on death.
- Findings: Core Enderman mechanics are 1.20.1 vanilla; target tags, random sequence order, teleport bounds and silk-touch carried-block loot need version-matched source.
- RustChunk relevance: Important vanilla hostile mob behavior and visible block/world interaction.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 stare cone/line of sight, pumpkin exception, teleport attempts, passenger constraints and enderman_holdable tags; use 1.20.1 block drop/loot behavior.
- Follow-up checks: Test stare range/LOS, carved pumpkin, anger delay, projectile types, potion/water, world border/loaded chunks, pickup/place with mobGriefing and death drops.
- RustChunk implementation mapping: RustChunk 1.20.1 Enderman goal/teleport manager and carried-block NBT/drop adapter.
- Evidence required before adoption: 1.20.1 source/mappings/tags, seeded teleport tests, collision fixtures and multiplayer block/entity traces.
- Completion state: COMPLETE — 719 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 26061 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1547. `crates/horizon-client/src/mobs/equine.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b7e49897461954352bc132f7f45fedfb6a5d72e`
- Size: 47611 bytes; 1308 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1308 (EOF); empty EOF probe 1309–1708
- Purpose: Implements AbstractHorse behavior for horses, donkeys, mules and undead horses, including random stats/coats, taming/rearing/eating, breeding inheritance, rider movement/jump, sounds, textures and model animation; also contains Camel/Llama integration.
- Dependencies / cross-references: Equine species and attributes; player/rider input and vehicle packets; breeding/age; item feeding/saddles/chests; movement/navigation; animation/models and sounds.
- Implementation / logic: Initializes mount stats and variants, handles taming and rearing, applies rider-controlled movement/jumps, follows parents, updates grazing/animation and exposes render/sound selection.
- Findings: Most classic horse/donkey/mule/skeleton/zombie horse and camel/llama behavior exists in 1.20.1; CamelHusk, Happy Ghast and Nautilus-linked rider branches are later.
- RustChunk relevance: Important 1.20.1 mount/gameplay behavior and Android presentation; server vehicle authority must be separated.
- Decision: adaptable; version-specific
- 1.20.1 implications: Port only the 1.20.1 species and rider rules, attributes, legacy saddle/armor/inventory NBT and synced flags; exclude CamelHusk/HappyGhast/Nautilus branches and match server MoveVehicle semantics.
- Follow-up checks: Test random health/speed/jump, horse variant/markings, tame temper/rear, feeding, offspring stats, mounted steering/jump, saddle/chest, water sounds, passenger position and remote vehicle packets.
- RustChunk implementation mapping: RustChunk 1.20.1 equine state/AI modules, legacy mount inventory serializer and remote vehicle adapter.
- Evidence required before adoption: 1.20.1 mappings/NBT, seeded spawn/breeding tests, vehicle packet traces, animation goldens and save/reload fixtures.
- Completion state: COMPLETE — 1308 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 47611 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1548. `crates/horizon-client/src/mobs/equip.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b20fa7d09fb9c8b45e319e76f68557911bb7a3cb`
- Size: 16819 bytes; 500 logical lines
- Coverage: 1–500 (EOF)
- Purpose: Implements mob equipment pickup gates and replacement comparisons for armor/weapons, including attribute-modifier calculation by slot group.
- Dependencies / cross-references: Mob species/equipment; item slot/attribute modifiers and enchantments; preferred-weapon tags; item pickup and inventory/drop logic.
- Implementation / logic: Maps equippable slot, computes item attribute values, compares armor toughness/attack damage/enchantment/durability/name and determines whether each species can pick up an item.
- Findings: Core armor/weapon pickup behavior exists in 1.20.1, but this snapshot uses modern item-component modifier groups and later spear/Parched species; target item/NBT and attribute APIs differ.
- RustChunk relevance: Useful vanilla mob equipment behavior; important for hostile mob combat and dropped-item interactions.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Reimplement against 1.20.1 item attribute modifiers, legacy ItemStack/NBT and exact Mob.wantsToPickUp species rules; remove spear tag and later species assumptions.
- Follow-up checks: Test armor/toughness, weapon preference, enchantments, durability/name tie-breaks, Binding curse, slot mapping, piglin brute, zombie/skeleton egg/glow-ink rules and despawn/pickup timing.
- RustChunk implementation mapping: RustChunk 1.20.1 mob equipment evaluator and item pickup policy over legacy ItemStack.
- Evidence required before adoption: 1.20.1 item attribute/mapping tables, preferred weapon tags, pickup matrix and deterministic comparison tests.
- Completion state: COMPLETE — 500 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 16819 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1549. `crates/horizon-client/src/mobs/farm.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ffb5331c7aeb6929ef48362df450018afccd34e`
- Size: 6407 bytes; 173 logical lines
- Coverage: 1–173 (EOF)
- Purpose: Registers pig/cow/mooshroom/sheep/chicken goals, chicken egg laying and wing/fall animation, and climate-dependent sheep color selection.
- Dependencies / cross-references: Farm mob species and food tags; breeding/temptation/eat-block goals; egg timer/loot event; animation flags; biome climate tags and RNG.
- Implementation / logic: Builds the species-specific goal priorities, initializes chicken egg timer, ticks egg drops/flap movement and rolls sheep wool color based on climate.
- Findings: Most farm-animal AI/egg behavior is 1.20.1, but climate-specific spawn tags/color weighting may reflect later data and should not be assumed target-compatible.
- RustChunk relevance: Relevant vanilla animal simulation and lightweight client animation.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use 1.20.1 farm animal goal priorities, chicken egg timing/loot and sheep color spawn rules/tags from target data; exclude later warm/cold animal variants where absent.
- Follow-up checks: Test goal priority order, feeding/breeding, sheep eat-block, chicken egg timer/reset/jockey/baby gates, fall damping, wing interpolation and biome color distribution.
- RustChunk implementation mapping: RustChunk 1.20.1 farm animal AI module and chicken/flap effects adapter.
- Evidence required before adoption: 1.20.1 source/mappings, biome/tag fixtures, seeded sheep-color distribution tests and chicken egg tick traces.
- Completion state: COMPLETE — 173 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 6407 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1550. `crates/horizon-client/src/mobs/finalize.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `060ad60cc87b82958b0daa35d0c355a382651183`
- Size: 94742 bytes; 2501 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2501 (EOF); empty EOF probe 2502–2901
- Purpose: Implements Mob.finalizeSpawn across spawn reasons: attribute modifiers, baby/group data, variants/sound sets, equipment/enchanting, zombie jockeys, species-specific spawn rules, conversions and spawned rider/mount chains.
- Dependencies / cross-references: Mob species/attributes; entity and biome/structure registries; local difficulty and RNG; spawn reasons/group data; equipment/enchantments/loot; world POIs, tags and collision; entity creation/riding.
- Implementation / logic: Dispatches finalizeSpawn by species and reason, applies age/variant/attribute/equipment rules, rolls group data, creates jockey/mount entities and seeds species-specific memories/states.
- Findings: This is a central vanilla spawn pipeline reference, but this snapshot is 26.3 and contains post-1.20.1 entities (Breeze, Creaking, Armadillo, CamelHusk, Nautilus, Happy Ghast, Copper Golem), later variant registries, Copper armor, spears and TrialSpawner reasons.
- RustChunk relevance: Extremely important for locally simulated mob parity, but requires a strict target-version rebuild; it does not implement Forge mod discovery or server spawn packets.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Rebuild the 1.20.1 EntitySpawnReason/group/finalizeSpawn chain, local-difficulty modifiers, RNG draw order, jockeys, attributes, variant tables and legacy equipment NBT; remove all later species/reasons/items and modern data-component assumptions.
- Follow-up checks: Test every 1.20.1 spawn reason/species, pack group inheritance, baby chance, jockey creation, zombie reinforcement attributes, armor/weapon/enchant rolls, biome/structure variants, Halloween equipment, rider chains and save/reload.
- RustChunk implementation mapping: RustChunk 1.20.1 spawn finalization service, versioned species dispatcher, legacy equipment serializer and deterministic spawn RNG layer.
- Evidence required before adoption: 1.20.1 mappings/source and registries, per-species spawn fixtures, seeded RNG traces, jockey/group goldens and NBT/attribute packet tests.
- Completion state: COMPLETE — 2501 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 94742 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1551. `crates/horizon-client/src/mobs/fox.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `694940323dbadbcfc0891c88cb6d7f382b508d97`
- Size: 45136 bytes; 1297 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1297 (EOF)
- Purpose: Fox state, trust UUIDs, defensive behavior, stalking/pouncing, item pickup, berry and chorus-fruit behavior, poses and rendering.
- Dependencies / cross-references: Fox AI depends on ordered goals, food/item tags, berries, chorus fruit, damage/target state, UUID persistence, navigation and mob events.
- Implementation / logic: Registers fox goals and target rules; handles trust and breeding state, stalking/pouncing, eating, item pickup, teleport, sleep/sit poses and texture selection.
- Findings: Core fox behavior is present in 1.20.1. The snapshot also includes a newer Nautilus alertable target and modern item-consumption/data-component assumptions; village trusted-player behavior is commented as unported.
- RustChunk relevance: Useful reference for vanilla fox simulation, item interactions and lightweight client state.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep the 1.20.1 fox behavior, but rebuild target selection, item/NBT persistence, effects and RNG against target-version sources; exclude Nautilus and modern item APIs.
- Follow-up checks: Verify exact 1.20.1 fox goal priorities, trust UUID NBT, item pickup/food tags, pounce damage, chorus-fruit teleport and seeded RNG order.
- RustChunk implementation mapping: RustChunk 1.20.1 fox AI/state module, legacy NBT codec and item-interaction event adapter.
- Evidence required before adoption: 1.20.1 Java mappings/source, fox NBT fixtures, food-tag registry, deterministic goal/RNG traces and interaction tests.
- Completion state: COMPLETE — 1297 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 45136 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1552. `crates/horizon-client/src/mobs/frog.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4b5532aee599c858efd2ed385e6b83d14b4c3789`
- Size: 40312 bytes; 1097 logical lines
- Coverage: 1–400, 401–800, 801–1097 (EOF)
- Purpose: Frog amphibious AI, tongue prey capture, eating, frogspawn placement, long jumps, croaking and pose state.
- Dependencies / cross-references: Depends on amphibious navigation, water/land state, slime/magma-cube targeting, frogspawn/block placement, shared LongJump logic, variants, sensors and RNG.
- Implementation / logic: Registers frog goals and prey filters; controls tongue pull/eat, spawn laying, long-jump destination selection, cooldowns, amphibious movement and animation state.
- Findings: Frog behavior belongs to the target era. LongJump is shared with Goat; later code paths and memory/tag architecture must not replace the 1.20.1 goal/sensor implementation. Sound and shuffle RNG sources require parity checks.
- RustChunk relevance: Strong reference for 1.20.1 frog behavior and aquatic navigation.
- Decision: adaptable; version-specific
- 1.20.1 implications: Retain frog behavior but reconstruct target-era goal priorities, variants/tags, frogspawn interaction and exact seeded RNG; keep Goat and Frog parameterization distinct.
- Follow-up checks: Compare 1.20.1 FrogAi goals/sensors, tongue target sizes, frogspawn placement, long-jump preferences, water malus and RNG draws.
- RustChunk implementation mapping: RustChunk 1.20.1 frog AI, amphibious movement adapter, tongue/projectile event and frogspawn block interaction.
- Evidence required before adoption: Target-version brain/goal source, tag/variant exports, seeded RNG traces and water/land behavior fixtures.
- Completion state: COMPLETE — 1097 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 40312 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1553. `crates/horizon-client/src/mobs/gear.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d93dc169c9e0c625defbc9953fac76822ff54ece`
- Size: 10094 bytes; 279 logical lines
- Coverage: 1–279 (EOF)
- Purpose: Gear-side calculations for equipment slots, enchantments, trims, weapon damage and armor protection.
- Dependencies / cross-references: Depends on ItemStack, enchantment registry/tags, damage types, equipment slots, attribute modifiers and combat formulas.
- Implementation / logic: Maps equipment to slots and evaluates enchantment bonuses, ranged weapon modifiers, armor protection and damage-type bypass rules.
- Findings: The file uses modern ItemStack/component and damage-type assumptions and includes later species/damage categories; 1.20.1 uses legacy enchantment NBT and older registries.
- RustChunk relevance: Useful reference for the combat/equipment calculation boundaries, not a direct port.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rewrite against 1.20.1 ItemStack NBT, enchantment registry and damage sources; filter later species and damage categories.
- Follow-up checks: Verify 1.20.1 Sharpness/Smite/Bane, Power/Punch/Flame/Piercing, armor/toughness, protection tags, trims and modifier ordering.
- RustChunk implementation mapping: RustChunk 1.20.1 legacy item/enchantment evaluator and combat attribute pipeline.
- Evidence required before adoption: 1.20.1 mappings, enchantment/damage registries, legacy ItemStack fixtures and deterministic damage/protection test matrix.
- Completion state: COMPLETE — 279 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 10094 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1554. `crates/horizon-client/src/mobs/goals.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `27779577378b5cee016efe0f573192d261804bce`
- Size: 100636 bytes; 2972 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–2972 (EOF)
- Purpose: Goal/target selectors, priority and control locks, melee, panic, avoidance, temptation, breeding, strolling, water seeking and block interaction.
- Dependencies / cross-references: Depends on Mob AI state, navigation/pathfinding, sensors/visibility, item and block tags, damage/target rules, world time and deterministic RNG.
- Implementation / logic: Defines generic Goal lifecycle and selector arbitration; implements many vanilla goal behaviors, target filters and movement/interaction routines.
- Findings: Many core goals exist in 1.20.1, but this snapshot adds newer species and memory behaviors. Exact selector ordering, locks, RNG and tag semantics must be compared to target code.
- RustChunk relevance: Central reference for the RustChunk mob AI engine; correctness affects many species.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild the goal framework and target registrations from 1.20.1, excluding later-only entities/behaviors; do not mix it with the newer Breeze/Creaking Brain framework.
- Follow-up checks: Compare goal priorities/flags, canUse/canContinue/start/tick/stop order, selector lock semantics, mobGriefing, target visibility, item tags and RNG.
- RustChunk implementation mapping: RustChunk 1.20.1 Goal/TargetSelector core plus versioned species goal registration.
- Evidence required before adoption: 1.20.1 Java source/mappings, selector traces, target/tag fixtures and deterministic behavior tests for each adopted goal.
- Completion state: COMPLETE — 2972 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 100636 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1555. `crates/horizon-client/src/mobs/goat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e9c1ef3d8c4d36f962b853e77be176debd805756`
- Size: 19643 bytes; 544 logical lines
- Coverage: 1–400, 401–544 (EOF)
- Purpose: Goat and screaming-goat ram attack, target selection, head lowering, charge, horn drops and long-jump behavior.
- Dependencies / cross-references: Depends on Goat/Frog LongJump, target predicates, pathfinding, collision, sound/events, horn items/tags and seeded UUID/RNG state.
- Implementation / logic: Registers ram and jump goals; selects targets, prepares a path, lowers head, charges, knocks back and drops horns on eligible blocks.
- Findings: Goat and screaming goat are target-era content. Current shared LongJump and cooldown/RNG details require exact 1.20.1 verification.
- RustChunk relevance: Useful reference for a distinctive vanilla mob AI with deterministic timing and block interaction.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep core goat logic but compare goal priorities, cooldown ranges, horn eligibility/drop NBT and RNG draw order to 1.20.1.
- Follow-up checks: Verify RamTarget rules, normal/screaming cooldowns, charge vector, knockback, horn loss, offspring and shared LongJump parameters.
- RustChunk implementation mapping: RustChunk 1.20.1 goat AI/state, horn drop event and shared parameterized long-jump routine.
- Evidence required before adoption: 1.20.1 source/mappings, goat entity NBT, horn tag/drop fixtures and seeded target/ram traces.
- Completion state: COMPLETE — 544 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 19643 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1556. `crates/horizon-client/src/mobs/golem.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `718d81b59fcdc24b5d11879cd6162677839ffb89`
- Size: 29649 bytes; 815 logical lines
- Coverage: 1–400, 401–800, 801–815 (EOF)
- Purpose: Iron-golem village defense and gifting, attack/crack state, plus snow-golem snowball, melting and snow-trail behavior.
- Dependencies / cross-references: Depends on POI/village reputation, player and mob targets, pathfinding, weather/temperature, mobGriefing, block changes and item events.
- Implementation / logic: Registers golem goals, target selection, village stroll/defense, poppy gift, melee/launch, crackiness, snowball attack, melting and snow placement.
- Findings: Core Iron/Snow Golem behavior exists in 1.20.1. Copper Golem gifting/acceptance is newer and must be removed; universal-anger reset is not fully ported in this snapshot.
- RustChunk relevance: Relevant for village simulation, hostile defense and snow/block interactions.
- Decision: adaptable; version-specific
- 1.20.1 implications: Adapt only 1.20.1 Iron/Snow Golem behavior; exclude Copper Golem and reconstruct village POI/anger state.
- Follow-up checks: Check 1.20.1 POI tags, target/reputation rules, poppy timing, attack damage/launch, snow melting and mobGriefing snow placement.
- RustChunk implementation mapping: RustChunk 1.20.1 golem AI, village reputation adapter and block-change event pipeline.
- Evidence required before adoption: Target-era Java source, POI/tag fixtures, player reputation cases and deterministic snow/attack tests.
- Completion state: COMPLETE — 815 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 29649 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1557. `crates/horizon-client/src/mobs/guardian.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f86fd0bc43f0984b46d421860ce296fc8d3a6272`
- Size: 17109 bytes; 487 logical lines
- Coverage: 1–400, 401–487 (EOF)
- Purpose: Guardian/Elder Guardian laser attack, swimming, spikes/thorns, land flop, beam animation and Mining Fatigue effect.
- Dependencies / cross-references: Depends on aquatic movement, target visibility, damage sources, effect application, sound/particles, difficulty and Elder Guardian timing.
- Implementation / logic: Implements beam charge/attack phases, target acquisition, swim control, thorns, tail/spike poses and periodic Elder Guardian effect.
- Findings: Core guardian behavior is target-era; the current spike path includes newer DamageKind::Spear and other enum details that must be excluded from 1.20.1.
- RustChunk relevance: Useful for underwater mob simulation and boss-like attack/effect timing.
- Decision: adaptable; version-specific
- 1.20.1 implications: Retain 1.20.1 Guardian/Elder Guardian logic after matching damage-source types, targeting and effect cadence.
- Follow-up checks: Verify laser duration/range, thorns damage source, target categories, difficulty scaling and 1200-tick Elder Guardian effect.
- RustChunk implementation mapping: RustChunk 1.20.1 aquatic hostile AI, beam attack event and Elder Guardian effect scheduler.
- Evidence required before adoption: 1.20.1 source/mappings, damage-type mapping, target visibility fixtures and seeded timing tests.
- Completion state: COMPLETE — 487 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 17109 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1558. `crates/horizon-client/src/mobs/happy_ghast.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d61dcbb784ecc753f0e5a2d422905d2cd4a6f49`
- Size: 24401 bytes; 706 logical lines
- Coverage: 1–400, 401–706 (EOF)
- Purpose: Happy Ghast/Ghastling lifecycle, harness equipment, controllable flight/riding, healing and multi-lead handling.
- Dependencies / cross-references: Depends on later entity species, harness/lead equipment, rider input, flight navigation, collision and newer entity data.
- Implementation / logic: Implements Happy Ghast variants, harness/rider movement, flight controls, healing and lead attachment handling.
- Findings: Happy Ghast/Ghastling and their harness/ride systems are absent from the 1.20.1 baseline.
- RustChunk relevance: No direct 1.20.1 gameplay adoption; only generic flight-control ideas may be studied separately.
- Decision: **unnecessary; post-1.20.1** — exclude; post-1.20.1
- 1.20.1 implications: Exclude the complete species and entity-specific state from the 1.20.1 build.
- Follow-up checks: Confirm target registry has no Happy Ghast/Ghastling, harness item or related entity/packet identifiers.
- RustChunk implementation mapping: No 1.20.1 implementation mapping; optional later-version research only.
- Evidence required before adoption: 1.20.1 entity/item registry and protocol identifiers confirming absence.
- Completion state: COMPLETE — 706 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 24401 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1559. `crates/horizon-client/src/mobs/hostile.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d2d054f8df302f680eb493b9db488bd9001ac987`
- Size: 14615 bytes; 422 logical lines
- Coverage: 1–400, 401–422 (EOF)
- Purpose: Creeper fuse/explosion and charged state, Spider wall/daylight behavior, Silverfish wake/infested-block behavior and Endermite handling.
- Dependencies / cross-references: Depends on hostile target goals, explosion/loot rules, potion cloud effects, wall collision, block tags, damage and entity conversion.
- Implementation / logic: Implements Creeper swell/defuse/explosion events, Spider climb and daylight rules, Silverfish ally wake/infested merge and related hostile behavior.
- Findings: Core species are 1.20.1. Avoid-Armadillo spider behavior is newer and must be filtered; explosion, loot and mobGriefing rules need version mapping.
- RustChunk relevance: Important vanilla hostile AI and combat parity.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep target-era Creeper/Spider/Silverfish/Endermite behavior and remove newer species interactions.
- Follow-up checks: Verify charged-creeper radius, fuse timing, explosion block/loot behavior, spider daylight/target rules and Silverfish infested-block merge.
- RustChunk implementation mapping: RustChunk 1.20.1 hostile mob AI, explosion event adapter and infested-block interaction.
- Evidence required before adoption: Target Java source, explosion/loot fixtures, entity tags and deterministic fuse/target tests.
- Completion state: COMPLETE — 422 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 14615 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1560. `crates/horizon-client/src/mobs/illager.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b53ca2f2d4ec64ecad6b2cc0ff12f4cbc342a36a`
- Size: 34647 bytes; 986 logical lines
- Coverage: 1–400, 401–800, 801–986 (EOF)
- Purpose: Pillager/Vindicator/Evoker/Illusioner shared behavior, crossbow charge/fire lifecycle, Evoker spells and illager targeting/poses.
- Dependencies / cross-references: Depends on Goal/TargetSelector, crossbow/item NBT, projectile RNG, Vex/Fangs, villager/wolf targets, spell effects and animations.
- Implementation / logic: Registers shared illager goals; handles crossbow states, ranged attacks, Evoker summons/fangs/Wololo and pose/animation state.
- Findings: Avoid-Creaking and modern charged-projectile ItemStack components are later architecture; target crossbows store charged projectiles in legacy NBT.
- RustChunk relevance: Useful species behavior, but item and crossbow state need a target-version rewrite.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Reimplement with 1.20.1 crossbow ItemStack NBT, entity targets, spell timings and older AI selector semantics; exclude Creaking.
- Follow-up checks: Check charged-projectile NBT, Quick Charge, shot ordering/spread, Evoker spell cooldowns, Vex lifetime, Fang placement and target filters.
- RustChunk implementation mapping: RustChunk 1.20.1 illager goal groups, legacy crossbow codec and spell/projectile event handlers.
- Evidence required before adoption: 1.20.1 mappings/source, crossbow NBT fixtures, projectile traces and per-spell tick/target tests.
- Completion state: COMPLETE — 986 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 34647 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1561. `crates/horizon-client/src/mobs/illusioner.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aa1621fdc74ec94c9d4942806431aec16b2438c7`
- Size: 8491 bytes; 274 logical lines
- Coverage: 1–274 (EOF)
- Purpose: Illusioner bow/spell behavior, invisibility and blindness, four mirror images, target selection and render offsets.
- Dependencies / cross-references: Depends on illager spell/target logic, bow/projectile damage, effects, entity pose and mirror-image client state.
- Implementation / logic: Implements Illusioner ranged attack and spells, creates/updates mirror offsets, invisibility and Blindness effect, and animation-facing state.
- Findings: Illusioner exists in 1.20.1 but is not a normal natural spawn. Avoid-Creaking is a newer branch and must be excluded.
- RustChunk relevance: Useful for complete entity registry behavior and optional Illusioner rendering/AI.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep Illusioner for target-era parity, verify spell durations and mirror synchronization; remove newer Creaking target logic.
- Follow-up checks: Compare spell cooldown/effect duration, bow charge/shot, mirror count/offset, invisibility and target acquisition with 1.20.1.
- RustChunk implementation mapping: RustChunk 1.20.1 Illusioner AI/spell state plus client mirror-image render adapter.
- Evidence required before adoption: 1.20.1 source/mappings, effect/goal fixtures and render goldens for mirror offsets.
- Completion state: COMPLETE — 274 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 8491 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1562. `crates/horizon-client/src/mobs/item_break.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3d8866f2ca8fae5e0fa4c2d6794c3154b1f7cc16`
- Size: 9278 bytes; 224 logical lines
- Coverage: 1–224 (EOF)
- Purpose: Client equipment-break events, slot identifiers, equipment lookup and broken-item/scute particle handling.
- Dependencies / cross-references: Depends on entity equipment state, ItemStack serialization, slot mapping and client event/particle IDs.
- Implementation / logic: Maps equipment-break event slots to held/equipped stacks and emits the matching break particle/render event.
- Findings: Core hand/armor slots are target-era; Body/Saddle slots and Wolf Armor/Armadillo scute handling are newer than 1.20.1.
- RustChunk relevance: Relevant client feedback for mob equipment durability, but modern slots cannot be reused.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Implement only 1.20.1 event slots and legacy stack lookup; remove Body/Saddle and later Wolf Armor paths.
- Follow-up checks: Verify event IDs/slot indices 47–52, hand/armor stack source and correct particle item metadata on 1.20.1.
- RustChunk implementation mapping: RustChunk 1.20.1 client equipment-break event adapter over legacy ItemStack.
- Evidence required before adoption: 1.20.1 packet/event mappings, equipment NBT fixtures and client particle goldens.
- Completion state: COMPLETE — 224 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 9278 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1563. `crates/horizon-client/src/mobs/keyframes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `edcee8835e55695071dced0c735431dfcb3a86b4`
- Size: 7492 bytes; 234 logical lines
- Coverage: 1–234 (EOF)
- Purpose: Generic keyframe animation parser/interpolator for position, rotation and scale channels, including linear and Catmull-Rom sampling.
- Dependencies / cross-references: Depends on JSON animation assets, model bone names, animation state/timing and client rendering transforms.
- Implementation / logic: Parses per-bone keyframes and samples channels into pose transforms; supports applying animation/walk state.
- Findings: Generic engine is not itself a gameplay feature; data/assets must be confirmed against 1.20.1 resource packs and model definitions.
- RustChunk relevance: Potential optional client-side resource-pack/model animation capability.
- Decision: **undetermined; reference; optional client feature** — reference; optional client feature
- 1.20.1 implications: Keep as a separate optional rendering feature only after verifying target JSON schema, assets and Android CPU/GPU cost.
- Follow-up checks: Check keyframe time units, interpolation edge cases, bone mapping, animation blending and memory allocation on Android.
- RustChunk implementation mapping: RustChunk optional client animation module; keep outside mob simulation/protocol compatibility.
- Evidence required before adoption: 1.20.1 asset-format fixtures, interpolation goldens and Android frame-time/memory profiling.
- Completion state: COMPLETE — 234 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7492 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1564. `crates/horizon-client/src/mobs/landing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6f20f809aca457e2fdc37e207c1618ff5953d9ac`
- Size: 7413 bytes; 213 logical lines
- Coverage: 1–213 (EOF)
- Purpose: Shared mob landing/block-fall integration for fall damage, sound, particles and block-specific effects.
- Dependencies / cross-references: Depends on Block.fallOn behavior, block registry/tags, mobGriefing, entity bounds, damage and block-change events.
- Implementation / logic: Adapts mob landing to block behavior including hay, honey, slime, bed, powder snow, eggs and pointed dripstone.
- Findings: Most interactions are present in 1.20.1. Newer shelf-mushroom behavior must not enter the target block set.
- RustChunk relevance: Useful shared interface for vanilla and modded block collision/landing hooks.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use the interface concept but populate exact 1.20.1 block behaviors and preserve modded block callback boundaries.
- Follow-up checks: Verify safe-fall distance, hay reduction, honey slide, slime bounce, bed bounce, powder snow and turtle egg/block effects.
- RustChunk implementation mapping: RustChunk block-landing callback interface invoked by versioned block registry and mob physics.
- Evidence required before adoption: 1.20.1 Block.fallOn source/mappings, block property/tag registry and deterministic landing fixtures.
- Completion state: COMPLETE — 213 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7413 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1565. `crates/horizon-client/src/mobs/leash.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3520c3fd10e557045f5a62a722c767d438b12e68`
- Size: 29434 bytes; 840 logical lines
- Coverage: 1–400, 401–800, 801–840 (EOF)
- Purpose: Leash attachment/persistence for players, mobs, knots and vehicles, connection geometry, offsets and leash movement forces.
- Dependencies / cross-references: Depends on UUID/NT-array persistence, entity spawn/destroy, leash knots, pathfinding, vehicle state and modern lead attachment support.
- Implementation / logic: Stores leash holders, resolves anchors, calculates endpoints/forces, supports species offsets and current multi-holder/vehicle attachment types.
- Findings: Current elastic spring/snapping distances, boat/raft, four-way and Happy Ghast/Nautilus attachments differ from 1.20.1 vanilla leash behavior.
- RustChunk relevance: Important entity persistence/interaction feature, but this implementation has substantial version drift.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild 1.20.1 lead/leash mechanics and NBT from target sources; exclude multi-holder, boats/rafts and all later species.
- Follow-up checks: Verify leash knot UUID and chunk/entity load order, break distance/force, eligible species, item drops and player interaction packets.
- RustChunk implementation mapping: RustChunk 1.20.1 leash service, UUID/NBT codec and client rope render adapter.
- Evidence required before adoption: 1.20.1 Java source/mappings, leash persistence fixtures, entity lifecycle traces and rope geometry goldens.
- Completion state: COMPLETE — 840 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 29434 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1566. `crates/horizon-client/src/mobs/living.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `35af3afa4c6f30528bf209649abffdbe18ca4393`
- Size: 84717 bytes; 2078 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2078 (EOF)
- Purpose: Living mob movement/physics: fluid movement, collision, dimensions/poses, gravity, fall damage, jumping, bouncing and walk animation.
- Dependencies / cross-references: Depends on AABB collision, block/fluid state, entity dimensions, attributes/effects, damage sources, movement controls, sounds and world border.
- Implementation / logic: Implements movement and collision resolution, fluid currents, landing/fall handling, pose/baby bounds, jump, step, push and animation state.
- Findings: Critical source snapshot is newer than 1.20.1 and includes new restitution/air drag, Sulfur Cube and later fluid/ride behavior; rebuild only the target-era physics model.
- RustChunk relevance: Core reference for correctness and performance of all mob movement; directly affects simulation cost and visual movement.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Rewrite from 1.20.1 LivingEntity/Mob movement and collision semantics; isolate newer restitution/air-drag branches and never port them blindly.
- Follow-up checks: Compare 1.20.1 fluid acceleration/drag, step height, collision order, fall damage, pose dimensions, world border, push and RNG; profile allocations and CPU time.
- RustChunk implementation mapping: RustChunk 1.20.1 shared living-entity physics core with mobile-oriented but behavior-preserving collision implementation.
- Evidence required before adoption: 1.20.1 source/mappings, collision/fall/fluid golden tests, server movement traces and Android CPU/allocation benchmarks.
- Completion state: COMPLETE — 2078 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 84717 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1567. `crates/horizon-client/src/mobs/llama.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b3a1f9d84974b265e748a0de7c81377614d8d084`
- Size: 11463 bytes; 332 logical lines
- Coverage: 1–332 (EOF)
- Purpose: Llama/Trader Llama coat, strength, chest capacity, taming temper, caravan, spit attack, breeding and despawn state.
- Dependencies / cross-references: Depends on AbstractHorse, caravan behavior, target species, legacy inventory/NBT, ranged projectile, carpet/decor assets and RNG.
- Implementation / logic: Registers Llama and Trader Llama goals/targets; rolls health/strength/coat, handles spit, healing, offspring inheritance, inventory columns and NBT fields.
- Findings: Both llama types are target-era. Exact Trader Llama defensive targets, despawn timer, inventory/temper fields and RNG draw order need parity checks.
- RustChunk relevance: Useful for passive mob, caravan and ranged-spit behavior with legacy inventory implications.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep 1.20.1 llama behaviors but validate inherited horse state, caravan, item/slot serialization and seeded random sequence.
- Follow-up checks: Verify 1.20.1 Llama/TraderLlama goals, target matrix, strength distribution, breeding, chest columns, temper and despawn delay.
- RustChunk implementation mapping: RustChunk 1.20.1 llama module integrated with shared equine and caravan systems, plus legacy NBT inventory codec.
- Evidence required before adoption: 1.20.1 mappings/source, llama NBT/inventory fixtures, caravan traces and seeded spawn/breeding/spit tests.
- Completion state: COMPLETE — 332 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 11463 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1568. `crates/horizon-client/src/mobs/monster_brain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e5ab25ed4475e6f533df6188c882a1c4f4c2f4da`
- Size: 54008 bytes; 1591 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1591 (EOF)
- Purpose: Brain memory/sensor/activity/behavior implementation for Breeze and Creaking.
- Dependencies / cross-references: Depends on Breeze/Creaking species, villager-brain memory slots, sensors, path/navigation, projectile and movement behaviors, seeded shuffle RNG.
- Implementation / logic: Implements timed Brain memories, nearest-living/player sensors, attack targeting, activities, weighted RunOne behavior and species-specific fight/idle packages.
- Findings: This file is exclusively Breeze/Creaking AI, both post-1.20.1. Generic memory/behavior abstractions are not evidence for replacing 1.20.1 Goal/TargetSelector behavior.
- RustChunk relevance: No baseline species adoption; generic memory framework may inform a future separate architecture study.
- Decision: **unnecessary; post-1.20.1; architecture reference only** — exclude; post-1.20.1; architecture reference only
- 1.20.1 implications: Exclude Breeze/Creaking and their species logic from 1.20.1; retain goal-based AI for target-era mobs and evaluate generic abstractions independently.
- Follow-up checks: Confirm 1.20.1 species and brain provider registries; compare any reusable memory semantics against target Java source before design adoption.
- RustChunk implementation mapping: No 1.20.1 species mapping; optional future AI architecture note only.
- Evidence required before adoption: 1.20.1 entity/brain registry and source proof for any generic memory-system reuse.
- Completion state: COMPLETE — 1591 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 54008 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1569. `crates/horizon-client/src/mobs/nautilus.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e44ef50a6769feabb51c02a2f827db67ffc1ee21`
- Size: 22095 bytes; 628 logical lines
- Coverage: 1–400, 401–628 (EOF)
- Purpose: Nautilus/Zombie Nautilus taming, saddle/armor, rider movement and dash, swimming, pufferfish charge, bubbles and breath effect.
- Dependencies / cross-references: Depends on new Nautilus species, tame/owner, saddle/equipment, rider controls, water navigation, pufferfish targets, new sounds/effects and NBT.
- Implementation / logic: Implements swim/rider travel, dash cooldown, anger/charge targets, breath effect, variants, saddle armor, animation and saved fields.
- Findings: Nautilus and Zombie Nautilus plus their rider/breath/equipment systems are absent from 1.20.1.
- RustChunk relevance: No direct target-era gameplay adoption.
- Decision: **unnecessary; post-1.20.1** — exclude; post-1.20.1
- 1.20.1 implications: Exclude species, item/effect IDs, rider packets and state from the 1.20.1 baseline.
- Follow-up checks: Confirm absence from 1.20.1 entity/item/effect registries and protocol before filtering all references.
- RustChunk implementation mapping: No 1.20.1 implementation mapping; later-version research only.
- Evidence required before adoption: 1.20.1 entity/item/effect registry and protocol identifiers.
- Completion state: COMPLETE — 628 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 22095 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1570. `crates/horizon-client/src/mobs/nav.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2b265fcaaa23f6927e9b743e2b82c9d59e22410a`
- Size: 28934 bytes; 776 logical lines
- Coverage: 1–400, 401–776 (EOF)
- Purpose: PathNavigation variants, path creation/recompute, waypoint following, corner cutting, stuck detection and timeouts.
- Dependencies / cross-references: Depends on PathFinder/Evaluator, Mob movement/controls, world block/fluid state, path type malus, block update notifications and game time.
- Implementation / logic: Implements ground/wall/water/flying/amphibious navigation, max path length, path requests, delayed recompute, waypoint progression and stuck timeouts.
- Findings: Core path-following concepts are target-relevant, but current amphibious/new species settings and exact timing need 1.20.1 comparison; implementation allocates and caches per query.
- RustChunk relevance: Critical mob AI and world traversal reference; path recompute cost is relevant to Android CPU/frame budget.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Rebuild target-era PathNavigation and variant support; preserve deterministic waypoint semantics while separately optimizing allocations/caches after correctness tests.
- Follow-up checks: Verify 1.20.1 path recompute throttle, waypoint reach, stuck/timeout thresholds, door/sun handling and allowed navigation variants; profile frequent path requests.
- RustChunk implementation mapping: RustChunk 1.20.1 navigation service over the versioned A* pathfinder with mobile profiling hooks.
- Evidence required before adoption: 1.20.1 source/mappings, path golden fixtures, deterministic navigation traces and Android CPU/allocation benchmarks.
- Completion state: COMPLETE — 776 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 28934 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1571. `crates/horizon-client/src/mobs/nether.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `505de22f7ec0d2c04a24354c9618d19551b9449b`
- Size: 14412 bytes; 419 logical lines
- Coverage: 1–400, 401–419 (EOF)
- Purpose: Blaze and Ghast AI, movement, ranged fireball attacks, target selection, audio events and model animation.
- Dependencies / cross-references: Depends on hostile goals, line-of-sight, projectile/fireball implementation, difficulty, RNG and flight movement control.
- Implementation / logic: Implements Blaze melee/three-shot volley and cooldown, hovering, smoke/burn events, Ghast free flight, target gaze and charged Large Fireball.
- Findings: Both mobs are target-era; projectile launch vectors, charge timing and random spread must be compared with the 1.20.1 Java behavior.
- RustChunk relevance: Useful Nether hostile simulation and ranged projectile behavior.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep target species but port exact 1.20.1 timers, sound/event IDs, projectile physics and RNG order.
- Follow-up checks: Test Blaze volley phases and rest, attack range, float acceleration, Ghast 10/20-tick warning/shot phases, fireball direction and difficulty scaling.
- RustChunk implementation mapping: RustChunk 1.20.1 Blaze/Ghast AI and projectile spawn/event adapter.
- Evidence required before adoption: 1.20.1 source/mappings, projectile traces, seeded RNG fixtures and sound/animation goldens.
- Completion state: COMPLETE — 419 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 14412 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1572. `crates/horizon-client/src/mobs/panda.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `35d97ac2c8ce28aee036a20405020f3beeb9af9e`
- Size: 34305 bytes; 1097 logical lines
- Coverage: 1–400, 401–800, 801–1097 (EOF)
- Purpose: Panda gene inheritance, eating, sitting, rolling, sneezing, fear, breeding and model/item animation.
- Dependencies / cross-references: Depends on gene registry, bamboo/food tags, weather/thunder, breeding/goal selection, item holding, sneeze loot and client animation state.
- Implementation / logic: Implements genes and attributes, gene inheritance, synchronized flags/counters, eating particles, sneeze event/loot, rest/roll/sneeze goals and rendering state.
- Findings: Panda systems are target-era; exact gene weighting, RNG sequence, bamboo conditions, goal priorities and sneeze loot must match 1.20.1.
- RustChunk relevance: Useful for passive mob simulation and specialized synchronized client animation.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep target-era Panda behavior while verifying gene data, tick counters, Bamboo/food tags and loot table; preserve server/client split.
- Follow-up checks: Compare 1.20.1 panda gene values/weighting, inherited main/hidden genes, eating timing, sneeze chance, thunder fear, breeding conditions and animation sync.
- RustChunk implementation mapping: RustChunk 1.20.1 Panda state/AI module and client pose/crumb particle adapter.
- Evidence required before adoption: 1.20.1 Java source/mappings, gene/tag/loot exports, seeded inheritance traces and client animation goldens.
- Completion state: COMPLETE — 1097 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 34305 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1573. `crates/horizon-client/src/mobs/parrot.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a6c9a69922914c2053cd6c16258e28b803bea1b2`
- Size: 15861 bytes; 399 logical lines
- Coverage: 1–399 (EOF)
- Purpose: Parrot flying navigation, tree-seeking wander, flapping/gliding, jukebox dance and mob-imitating sounds.
- Dependencies / cross-references: Depends on flying movement/navigation, shoulder entity behavior, nearby species list, jukebox state, level RNG and client wing animation.
- Implementation / logic: Registers flying controls, handles flap/glide and flight sound, selects tree perches, checks jukebox dance and imitates nearby/shoulder mob sounds.
- Findings: Core parrot behavior exists in 1.20.1; current imitation table contains post-target species such as Breeze, Creaking, Camel Husk, Happy Ghast and Nautilus.
- RustChunk relevance: Relevant flying mob and client animation behavior with a versioned species sound table.
- Decision: adaptable; version-specific
- 1.20.1 implications: Retain target-era flight/dance/imitations; build the imitation table only from mobs and sounds present in 1.20.1.
- Follow-up checks: Verify exact 1.20.1 imitable species, nearby search order, shoulder sound RNG, tree search order and flap/glide tick math.
- RustChunk implementation mapping: RustChunk 1.20.1 parrot flight/animation module and target-era imitation sound registry.
- Evidence required before adoption: 1.20.1 sound/entity registries, shoulder entity behavior source, seeded sound-selection traces and flight animation goldens.
- Completion state: COMPLETE — 399 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 15861 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1574. `crates/horizon-client/src/mobs/path.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4a7d98a1640cd17527eb50ce28cf01139d6dcdb8`
- Size: 49928 bytes; 1397 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1397 (EOF)
- Purpose: A* PathFinder, node/heap/path structures and Walk/Swim/Fly/Amphibious node evaluators.
- Dependencies / cross-references: Depends on block path classifications, collision/AABB, fluid state, mob malus/size, navigation, world bounds and node visit limits.
- Implementation / logic: Builds per-query node arena/index/open heap and caches; evaluates node types and neighbors; runs A* and reconstructs the chosen path.
- Findings: High-value algorithm reference, but newer path types/evaluators and world-data assumptions exist; exact neighbor ordering, tie behavior, costs and limits are compatibility-sensitive.
- RustChunk relevance: Critical infrastructure for mob navigation and CPU use; cache design may reduce repeated block classification but must preserve outputs.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Reimplement against 1.20.1 PathFinder/NodeEvaluator, then optimize only behind golden tests; exclude post-target PathTypes/evaluators.
- Follow-up checks: Verify node evaluator classifications, start position, diagonal/vertical neighbor order, heap tie-breaking, malus, maxVisited, maxPathLength and path reconstruction.
- RustChunk implementation mapping: RustChunk 1.20.1 A* pathfinding core with versioned node evaluators and bounded scratch buffers.
- Evidence required before adoption: 1.20.1 source/mappings, path golden fixtures across terrain/fluid/door/fence cases and Android time/allocation profiles.
- Completion state: COMPLETE — 1397 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 49928 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1575. `crates/horizon-client/src/mobs/phantom.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0168801a3c9220937cce1c9b9d3af7bc5bfd6a5d`
- Size: 20067 bytes; 543 logical lines
- Coverage: 1–400, 401–543 (EOF)
- Purpose: Phantom circling/swooping, player targeting, cat avoidance, movement control, size scaling and flap animation.
- Dependencies / cross-references: Depends on target visibility, cats, anchor/home state, custom movement control, melee damage, sea level, difficulty and RNG.
- Implementation / logic: Selects player targets, circles above an anchor, schedules swoops, checks cat presence, bites on AABB overlap and banks toward movement points.
- Findings: Phantom is present in 1.20.1. Some source comments/tests reference 26.3; use the 1.20.1 target source for attack/visibility semantics.
- RustChunk relevance: Useful for hostile flying AI and custom movement controls.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep the phantom system but verify size, anchor height, scan/cooldown intervals, cat fear and line-of-sight against 1.20.1.
- Follow-up checks: Compare 1.20.1 attack scan volume, target visibility, circle/swoop durations, bite collision, cat scare radius and size/attack attributes.
- RustChunk implementation mapping: RustChunk 1.20.1 Phantom AI and custom flying movement controller.
- Evidence required before adoption: 1.20.1 Java source/mappings, target visibility/occlusion fixtures, movement traces and deterministic RNG tests.
- Completion state: COMPLETE — 543 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 20067 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1576. `crates/horizon-client/src/mobs/piglin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1560290ce641930fdb92ee5cb38a95140c92890b`
- Size: 83650 bytes; 2234 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2234 (EOF)
- Purpose: Piglin, Piglin Brute, Hoglin and Zoglin behavior: anger, hunting, bartering, equipment, riding, conversions and family interactions.
- Dependencies / cross-references: Depends on Piglin/Hoglin Brain memories and sensors, barter tables and gold-item handling, target tags, equipment/crossbow state, navigation, zombification, NBT and deterministic RNG.
- Implementation / logic: Registers species behavior and tasks; updates targets and anger; handles barter/equipment, Hoglin/Piglin interactions, conversion and movement/animation state.
- Findings: Both Piglins and Hoglins are target-era content, but this snapshot mixes them with newer Brain/data assumptions and Spear-related branches; it is not a clean 1.20.1 port.
- RustChunk relevance: Important Nether mob AI, barter and family-behavior reference; remote server authority remains authoritative in multiplayer.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild the 1.20.1 Piglin/Hoglin memory and task sets from target Java sources; preserve barter/conversion and legacy NBT; remove Spear and later-only equipment/targets.
- Follow-up checks: Verify 1.20.1 anger/hunt memories, gold barter eligibility and loot, zombification, Hoglin fear, baby riding, crossbow rules, target tags and RNG order.
- RustChunk implementation mapping: RustChunk 1.20.1 Piglin/Hoglin AI, barter transaction/loot service, anger-memory codec and legacy equipment pipeline.
- Evidence required before adoption: 1.20.1 source/mappings, Brain memory/task registry, Piglin/Hoglin NBT, barter loot tables, tags, seeded AI traces and server event captures.
- Completion state: COMPLETE — 2234 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 83650 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1577. `crates/horizon-client/src/mobs/polar_bear.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ec0afc70009e56b342bc0fafa6acc79268a84554`
- Size: 6616 bytes; 203 logical lines
- Coverage: 1–203 (EOF)
- Purpose: Polar Bear neutral-hostile transition, cub protection, warning posture and melee attack behavior.
- Dependencies / cross-references: Depends on age/baby state, player and mob target predicates, visibility, difficulty, damage, movement goals and animation flags.
- Implementation / logic: Registers movement and target goals; adults react to threats near cubs, warn before attacking and apply melee damage with pose state.
- Findings: Polar Bear is target-era content; cub radius, target exclusions, warning duration and mob-target tags require 1.20.1 verification.
- RustChunk relevance: Useful vanilla neutral-mob behavior and compact deterministic AI reference.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep the species but rebuild target predicates, proximity tests and timings from 1.20.1 source and tags.
- Follow-up checks: Compare cub-defense radius, player/mob exclusions, warning phase, attack damage, difficulty scaling and goal priority.
- RustChunk implementation mapping: RustChunk 1.20.1 Polar Bear goal/target module and synchronized warning-pose event.
- Evidence required before adoption: 1.20.1 PolarBear source/mappings, cub/adult target fixtures, tag export and deterministic tick traces.
- Completion state: COMPLETE — 203 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 6616 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1578. `crates/horizon-client/src/mobs/rabbit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5aa5415d1c03e03e3e8053b0c4b9aa085b7a8896`
- Size: 25870 bytes; 755 logical lines
- Coverage: 1–400, 401–755 (EOF)
- Purpose: Rabbit variants and movement, hopping, temptation/avoidance, crop eating and Killer Bunny combat.
- Dependencies / cross-references: Depends on variant registry, carrot crop age/block state, food and target tags, jump/move controls, damage, breeding, sounds and RNG.
- Implementation / logic: Registers rabbit goals and target selectors; changes hop timing/speed by state, eats eligible crops, flees predators/players and configures Killer Bunny targeting/attacks.
- Findings: Rabbit and Killer Bunny behavior exist in the target era; crop griefing, variant selection and jump timing depend on block state, tags and RNG order.
- RustChunk relevance: Useful passive/neutral mob AI and block-interaction coverage.
- Decision: adaptable; version-specific
- 1.20.1 implications: Retain target-era behavior; use 1.20.1 variant registry and crop/food tags and preserve server-authoritative crop changes.
- Follow-up checks: Verify Killer Bunny targets, carrot crop age and mobGriefing, hop impulse/timing, panic/tempt goals, breeding and variant RNG.
- RustChunk implementation mapping: RustChunk 1.20.1 Rabbit AI/variant module, jump controller and crop-eating block event.
- Evidence required before adoption: 1.20.1 source/mappings, rabbit variant/tag data, crop-state fixtures, seeded hop/variant traces and block-change events.
- Completion state: COMPLETE — 755 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 25870 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1579. `crates/horizon-client/src/mobs/raider.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `46765c80211d6bf280cfc8ffe6fda28ee9e786d7`
- Size: 26577 bytes; 774 logical lines
- Coverage: 1–400, 401–774 (EOF)
- Purpose: Illager raider patrol state, raid leader/banner handling, village movement and shared raid behavior.
- Dependencies / cross-references: Depends on Raid state/wave controller, village POIs, patrol spawn rules, illager species/targets, banner/equipment NBT, navigation and game events.
- Implementation / logic: Implements common raider and patrol/leader state, tracks raid association and banner state, and supplies movement/raid hooks for individual illagers.
- Findings: Relevant to 1.20.1 raids, but this module is only one part of Raid; center, wave scheduling, victory/loss and other transitions are owned elsewhere.
- RustChunk relevance: Useful entity-state reference for integration with the larger village/raid system.
- Decision: adaptable; version-specific
- 1.20.1 implications: Use only with a complete 1.20.1 Raid and Patrol implementation; do not treat this file alone as a full raid engine.
- Follow-up checks: Compare patrol eligibility, captain/banner NBT, wave membership, target selection, village POIs, raid events and save/reload behavior.
- RustChunk implementation mapping: RustChunk 1.20.1 Raider base state integrated with a separate authoritative Raid/Patrol service.
- Evidence required before adoption: 1.20.1 Raider/Raid source and mappings, raid NBT fixtures, POI data, banner/captain cases and deterministic wave traces.
- Completion state: COMPLETE — 774 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 26577 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1580. `crates/horizon-client/src/mobs/random_textures.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8ec6105cff57e2c4b596699df61908f1e67cbc6c`
- Size: 23001 bytes; 595 logical lines
- Coverage: 1–400, 401–595 (EOF)
- Purpose: OptiFine/ETF-style random entity texture rule loading and variant-based texture selection.
- Dependencies / cross-references: Depends on resource-pack reload, texture atlas/identifiers, entity species/variant data, biome/world conditions, predicate parsing and render state.
- Implementation / logic: Loads random-texture definitions and conditions, resolves candidate variants and selects a matching texture for an entity/render state.
- Findings: Optional resource-pack compatibility, not vanilla mob simulation; rule formats and condition sets are external compatibility contracts.
- RustChunk relevance: Potential visual feature, but CPU and memory cost must be bounded on Android.
- Decision: **undetermined; reference; optional client feature** — reference; optional client feature
- 1.20.1 implications: Keep outside the initial 1.20.1 gameplay/protocol core; implement only an explicitly scoped optional OptiFine/ETF subset.
- Follow-up checks: Verify rule precedence, random group choice, fallback, reload invalidation, variant/biome predicates and cache memory.
- RustChunk implementation mapping: RustChunk optional entity-texture compatibility service feeding the versioned renderer.
- Evidence required before adoption: Resource-pack fixtures, OptiFine/ETF behavior references, deterministic selection tests, reload/leak tests and Android memory/frame-time profiles.
- Completion state: COMPLETE — 595 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 23001 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1581. `crates/horizon-client/src/mobs/ravager.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `15ad5ec41adfa402d20c40c7f766c20c477582aa`
- Size: 11473 bytes; 337 logical lines
- Coverage: 1–337 (EOF)
- Purpose: Ravager charge and melee attacks, stun/roar state, knockback, damage reactions and foliage/block interaction.
- Dependencies / cross-references: Depends on raid/illager targets, movement control, shield blocking, damage/knockback, mobGriefing, block tags, sound and animation events.
- Implementation / logic: Registers goals and targets; manages charging, attack timing, roar/stun reactions, shield interaction and block/plant destruction.
- Findings: Ravager is target-era; Torchflower and Pitcher Plant destruction paths in this snapshot are later and must be filtered.
- RustChunk relevance: Useful hostile raid-mob behavior and collision/damage feedback.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep 1.20.1 attack/stun behavior; remove later crop species and verify mobGriefing/block rules.
- Follow-up checks: Compare charge distance/speed, reach/damage, shield stun and roar timing, knockback, block tags and raid target rules.
- RustChunk implementation mapping: RustChunk 1.20.1 Ravager AI and server-authoritative attack/block-change adapter.
- Evidence required before adoption: 1.20.1 Ravager source/mappings, shield/damage fixtures, block-tag export, deterministic attack traces and raid target tests.
- Completion state: COMPLETE — 337 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 11473 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1582. `crates/horizon-client/src/mobs/render.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f5571b282382d42b639d4e52782b9d12a7c3d7ed`
- Size: 153430 bytes; 3984 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–3984 (EOF)
- Purpose: Shared mob model rendering, pose assembly, geometry/vertex generation, model layers, textures and CPU/GPU rendering paths.
- Dependencies / cross-references: Depends on entity model/layer definitions, pose transforms, texture/atlas assets, render pipeline/shaders, GPU capabilities, equipment and entity render state.
- Implementation / logic: Builds model poses and render layers, transforms geometry into draw data and routes eligible geometry through GPU paths with CPU/fallback handling.
- Findings: Contains useful shared renderer architecture but also many later-version entity/model identifiers: Happy Ghast, Nautilus, Sulfur Cube, Copper Golem, Breeze, Creaking, Bogged and Spear-era assets.
- RustChunk relevance: Potentially important for mob rendering, but it is a client renderer subsystem, not Forge compatibility or game simulation.
- Decision: rewrite required; optional client rendering; version-specific
- 1.20.1 implications: Reconstruct model/layer registries from 1.20.1 and isolate rendering from server-authoritative simulation; adopt GPU paths only after visual parity and device profiling.
- Follow-up checks: Verify target model/layers, armor/eyes, pose transforms, vertex winding/normals, CPU fallback, shader limits and Android allocations.
- RustChunk implementation mapping: RustChunk 1.20.1 entity renderer with versioned model registry, shared pose builder and optional GPU backend.
- Evidence required before adoption: 1.20.1 model/layer/resource exports, render golden images, CPU/GPU parity fixtures, shader capability checks and Android frame-time/memory benchmarks.
- Completion state: COMPLETE — 3984 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 153430 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1583. `crates/horizon-client/src/mobs/riders.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a7eee679cb39fef288f352efbf95d00a7c8b3c90`
- Size: 15545 bytes; 399 logical lines
- Coverage: 1–399 (EOF)
- Purpose: Entity riding relationships, passenger seat offsets, jockey/trap mounting, dismount and rider-driven movement.
- Dependencies / cross-references: Depends on vehicle/passenger graph, entity dimensions/pose, mount eligibility, movement controls, player input, collision and riding NBT/packets.
- Implementation / logic: Calculates passenger placement/offsets, manages rider/vehicle interactions and dismount, and handles species-specific jockey/trap-rider cases.
- Findings: Mounting is core 1.20.1 behavior, but current entries include later Camel/Husk, Nautilus and Sulfur Cube seat layouts; protocol and vehicle authority must match the target.
- RustChunk relevance: Important for vanilla mounts and modded riding interoperability, especially server packet/state parity.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild passenger graph, offsets and dismount rules from 1.20.1; filter later entities and preserve server authority in multiplayer.
- Follow-up checks: Verify passenger order, boats/minecarts/horses, jockey spawn, collision/dismount safety, teleport and metadata/packet order.
- RustChunk implementation mapping: RustChunk 1.20.1 passenger graph and versioned vehicle/rider placement service.
- Evidence required before adoption: 1.20.1 mappings/protocol fixtures, mount/dismount traces, NBT, seat transform goldens and multiplayer vehicle captures.
- Completion state: COMPLETE — 399 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 15545 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1584. `crates/horizon-client/src/mobs/seen_grid.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4f5b1dd8bc6598bff2f1c8810f1c7320c4f29a1`
- Size: 5540 bytes; 148 logical lines
- Coverage: 1–148 (EOF)
- Purpose: Spatial broad-phase grid for grouping entity bounding boxes and querying nearby collision candidates.
- Dependencies / cross-references: Depends on entity AABBs, world/chunk coordinates, entity insertion/removal/movement and stable iteration/snapshot order.
- Implementation / logic: Partitions mob bounds into fixed 4-block cells and returns nearby candidates; falls back to broader scanning for unbounded queries.
- Findings: Internal acceleration structure, not vanilla semantics. Correctness depends on no missed boundary-crossing boxes and deterministic candidate order.
- RustChunk relevance: Potential CPU optimization for collision and nearby-entity queries; no gain has been measured.
- Decision: **undetermined; reference; performance candidate; parity required** — reference; performance candidate; parity required
- 1.20.1 implications: Evaluate only behind parity tests against exhaustive 1.20.1 entity queries; never alter collision results.
- Follow-up checks: Test negative coordinates, cell edges, large entities, movement, removal, duplicates and ordering; benchmark density distributions on Android.
- RustChunk implementation mapping: RustChunk optional entity spatial index used by collision/AI query adapters after exact results are proven.
- Evidence required before adoption: Property tests versus exhaustive AABB scans, deterministic ordering fixtures, lifecycle stress and Android CPU/allocation benchmarks.
- Completion state: COMPLETE — 148 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5540 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1585. `crates/horizon-client/src/mobs/shulker.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b37dc28fc49ca7f638df02281f7ba0a0b5205b81`
- Size: 22569 bytes; 657 logical lines
- Coverage: 1–400, 401–657 (EOF)
- Purpose: Shulker attachment/facing, shell opening, entity pushing, bullet homing/attacks and teleport behavior.
- Dependencies / cross-references: Depends on block-face attachment/survival, target visibility, projectile homing, collision, damage, teleport validation and synchronized animation.
- Implementation / logic: Chooses/persists an attach face, opens/closes shell, pushes entities as bounds expand, fires homing bullets and searches valid teleport destinations.
- Findings: Shulker and Shulker Bullet are target-era; interaction with Happy Ghast and other modern entities is newer and must be removed.
- RustChunk relevance: Useful End-mob AI, projectile tracking and block-attached entity behavior.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep target-era behavior while rebuilding target checks, face order, projectile NBT/packets and collision against 1.20.1.
- Follow-up checks: Verify attach-face priority, support validity, shell hitbox, bullet homing/deflection, teleport search, damage and RNG.
- RustChunk implementation mapping: RustChunk 1.20.1 Shulker state/AI, block-face attachment service and Shulker Bullet adapter.
- Evidence required before adoption: 1.20.1 source/mappings, attach/detach NBT, projectile trajectories, collision tests and seeded teleport traces.
- Completion state: COMPLETE — 657 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 22569 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1586. `crates/horizon-client/src/mobs/skin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `62967bc7bed14deeb3a36df8200c998b4e183a47`
- Size: 30550 bytes; 794 logical lines
- Coverage: 1–400, 401–794 (EOF)
- Purpose: GPU skinning/cache path for mob geometry and pose-specific draw jobs, with CPU fallback.
- Dependencies / cross-references: Depends on GPU buffer/vertex formats, model meshes/normals, pose matrices, draw-job cache keys, render-thread lifecycle and shader capabilities.
- Implementation / logic: Flattens/caches model geometry, derives pose/draw jobs, reuses matching work and sends eligible models through GPU while falling back for unsupported geometry.
- Findings: Potentially useful architecture, but asset/test expectations target newer resources; no CPU/GPU parity test, build or Android benchmark has run.
- RustChunk relevance: Relevant to mobile FPS goals, but must be optional and guarded by capability checks; benefit is unproven.
- Decision: **undetermined; reference; optional client feature; performance candidate** — reference; optional client feature; performance candidate
- 1.20.1 implications: Port only after 1.20.1 model parity; keep CPU fallback, bounded caches and invalidation on reload/device loss.
- Follow-up checks: Measure frame time, allocations, buffer uploads, cache hit/miss, shader compatibility and visual equivalence on target Android GPU and fallback paths.
- RustChunk implementation mapping: RustChunk optional GPU entity-skinning backend integrated with the 1.20.1 renderer and CPU fallback.
- Evidence required before adoption: 1.20.1 model fixtures, CPU/GPU image parity, shader compile tests, cache invalidation and Android frame-time/memory profiles.
- Completion state: COMPLETE — 794 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 30550 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1587. `crates/horizon-client/src/mobs/sniffer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a5a4ee8ef21c8c5e65d61057c007ac7954e6410d`
- Size: 7430 bytes; 216 logical lines
- Coverage: 1–216 (EOF)
- Purpose: Sniffer sniffing, digging, seed-drop state, remembered excavation positions and model/animation state.
- Dependencies / cross-references: Depends on Sniffer Brain goals/memories/sensors, diggable block tags, seed loot, world time, dimensions, NBT and synchronized animation.
- Implementation / logic: Stores sniff/dig counters and state, tracks positions dug, emits seed/drop and sound/particle events and supplies pose/render information.
- Findings: Sniffer was added in Minecraft 1.20 and exists in 1.20.1. This file is state/animation support, not a complete AI implementation.
- RustChunk relevance: Useful target-era mob state and client animation reference; AI/loot are separate.
- Decision: adaptable; version-specific
- 1.20.1 implications: Retain target-era state while implementing its 1.20.1 Brain/task registry, block tags, seed loot and NBT independently.
- Follow-up checks: Compare sniff cooldown/range, dig duration, block eligibility, remembered positions, seed loot, baby size and synchronized event order.
- RustChunk implementation mapping: RustChunk 1.20.1 Sniffer state codec, Brain task adapter and client sniff/dig animation handler.
- Evidence required before adoption: 1.20.1 Sniffer source/mappings, brain-memory registry, diggable tags, loot table, NBT fixtures and seeded behavior traces.
- Completion state: COMPLETE — 216 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7430 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1588. `crates/horizon-client/src/mobs/spawn_rules.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5e2079ebcc84857c4a9dcfc77f3af6006d763fd4`
- Size: 25769 bytes; 756 logical lines
- Coverage: 1–400, 401–756 (EOF)
- Purpose: Mob spawn placement and validity rules using light, biome, block/fluid state, slime chunks and spawn reasons.
- Dependencies / cross-references: Depends on spawn-placement registry, block/biome tags, light levels, fluid state, heightmaps, collision shapes, dimension data and spawn reason.
- Implementation / logic: Evaluates placement categories and species conditions, including water/lava/ground placement, light checks, slime-chunk logic and cached spawn-spot classification.
- Findings: Rules are highly version/data-sensitive and many species/reasons in the snapshot are newer than 1.20.1. The spot table is derived from block registry/shape data.
- RustChunk relevance: Important for local world simulation or parity tests; a multiplayer client must not override server spawn authority.
- Decision: rewrite required; version-specific; server-side reference
- 1.20.1 implications: Rebuild the 1.20.1 SpawnPlacements registry and tag exports; separate integrated-host simulation from remote-client prediction.
- Follow-up checks: Compare retained EntityType placement, heightmap, light/sky rules, biome/block/fluid tags, mobGriefing and slime-chunk seed behavior.
- RustChunk implementation mapping: RustChunk 1.20.1 versioned spawn-placement registry and bounded block-shape lookup cache.
- Evidence required before adoption: 1.20.1 source/mappings, entity/biome/block tags, light/heightmap fixtures, dimension tests and seeded slime-chunk/spawn traces.
- Completion state: COMPLETE — 756 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 25769 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1589. `crates/horizon-client/src/mobs/spawn_selection.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04f12db2122d8cb7ea42747e7723291093e379fb`
- Size: 49165 bytes; 1382 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1382 (EOF)
- Purpose: Natural mob spawn selection, category caps, weighted pack placement, player distance and structure/dimension modifiers.
- Dependencies / cross-references: Depends on NaturalSpawner, entity categories and spawn costs, biome/structure spawn lists, chunk generation, player/respawn positions, spawn rules and level RNG.
- Implementation / logic: Computes eligible categories/caps, chooses spawn entries and pack positions, applies structure modifiers and spawn-charge/energy accounting.
- Findings: Models server-side world spawning and chunk-generation, not just client visuals. New species and iteration/cache assumptions need target 1.20.1 verification.
- RustChunk relevance: Behavioral reference for integrated-world parity tests, not authority for a remote multiplayer client.
- Decision: rewrite required; version-specific; server-side reference
- 1.20.1 implications: Recreate 1.20.1 NaturalSpawner semantics in local-host mode; keep remote spawning server-authoritative and preserve Java RNG order.
- Follow-up checks: Verify category caps, per-player additions, spawn charge, pack attempts, nearest-player order, structure overrides, respawn exclusion and chunk-gen RNG.
- RustChunk implementation mapping: RustChunk 1.20.1 NaturalSpawner-compatible service behind an integrated-world authority boundary.
- Evidence required before adoption: 1.20.1 NaturalSpawner source/mappings, biome/structure spawn data, deterministic seeded world fixtures and server spawn traces.
- Completion state: COMPLETE — 1382 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 49165 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1590. `crates/horizon-client/src/mobs/spawner.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b4d65d1c7a104f31aaf68f99446307f670a2a14a`
- Size: 30903 bytes; 836 logical lines
- Coverage: 1–400, 401–800, 801–836 (EOF)
- Purpose: Vanilla Monster Spawner block-entity state, spawn potentials, delay/near-player checks and client visual feedback.
- Dependencies / cross-references: Depends on spawner block-entity NBT, SpawnData/SpawnPotentials codecs, entity registry, placement/obstruction rules, player proximity, block events and particles.
- Implementation / logic: Loads/saves spawner state, selects weighted spawn data, manages delay/count checks, attempts spawning and emits events, sounds and spin/particle state.
- Findings: Ordinary Monster Spawner exists in 1.20.1. TrialSpawner and its newer config/state are separate 1.21 features.
- RustChunk relevance: Relevant to local world parity and spawner visuals; remote operation remains server-authoritative.
- Decision: adaptable; version-specific; server-side reference
- 1.20.1 implications: Use the classic 1.20.1 MobSpawnerBlockEntity schema and logic; exclude TrialSpawner and preserve legacy SpawnData NBT.
- Follow-up checks: Verify delay ranges, spawn count/range, nearby entity cap, player activation radius, obstruction and client spin/event timing.
- RustChunk implementation mapping: RustChunk 1.20.1 Monster Spawner codec, spawn service and client visual adapter.
- Evidence required before adoption: 1.20.1 Java source/mappings, spawner NBT, SpawnData/SpawnPotentials fixtures, seeded weighted picks and block-event goldens.
- Completion state: COMPLETE — 836 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 30903 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1591. `crates/horizon-client/src/mobs/spawning.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ed4f27d08db5246617b1c7c81f70567ce52dcddd`
- Size: 19624 bytes; 495 logical lines
- Coverage: 1–400, 401–495 (EOF)
- Purpose: Local difficulty and mob finalizeSpawn initialization: variants, equipment, attributes, spawn reason and state.
- Dependencies / cross-references: Depends on local/regional difficulty, spawn reason, species finalizeSpawn, loot/equipment tables, item registries, attributes, NBT and Java RNG.
- Implementation / logic: Calculates local difficulty and finalizes mob state, rolling variants/equipment/weapons and initializing special spawn or vehicle relationships.
- Findings: Highly version-sensitive; snapshot includes later Spear, Copper Armor, Camel Husk, Nautilus and Sulfur Cube branches.
- RustChunk relevance: Reference for spawn initialization, but local spawning is server/world authority and exact RNG compatibility is critical.
- Decision: rewrite required; version-specific; server-side reference
- 1.20.1 implications: Rebuild retained 1.20.1 finalizeSpawn methods and legacy equipment/attribute/item NBT; exclude later species/items.
- Follow-up checks: Compare difficulty, spawn-reason branches, armor/weapon chance, variants, baby/vehicle initialization and RNG order.
- RustChunk implementation mapping: RustChunk 1.20.1 local-difficulty and species-finalize service with legacy equipment serialization.
- Evidence required before adoption: 1.20.1 mappings/source, per-species equipment tables, difficulty fixtures, legacy NBT round-trips and seeded spawn traces.
- Completion state: COMPLETE — 495 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 19624 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1592. `crates/horizon-client/src/mobs/spear_use.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d18bfdda35962f8d8f9536d0a1ef78e9f6282bb7`
- Size: 31642 bytes; 895 logical lines
- Coverage: 1–400, 401–800, 801–895 (EOF)
- Purpose: Spear/Kinetic Weapon use, stab/charge behavior, knockback/dismount and spear-related hostile mob tactics.
- Dependencies / cross-references: Depends on later Spear item/damage types, modern item components, mob Brain tasks, mount state and attack events.
- Implementation / logic: Implements spear charge/approach/attack behavior and zombie/piglin tactics, including dismount/push and kinetic damage.
- Findings: Spear and KineticWeapon behavior are absent from the Minecraft 1.20.1 baseline; associated DamageKind and task branches are later additions.
- RustChunk relevance: No direct target-era gameplay adoption; retain only if a later-version roadmap explicitly adds this item.
- Decision: **unnecessary; post-1.20.1** — exclude; post-1.20.1
- 1.20.1 implications: Exclude the module and all Spear item/damage/task identifiers from the 1.20.1 build.
- Follow-up checks: Confirm 1.20.1 item, damage-type and entity-goal registries contain no Spear/Kinetic Weapon dependency.
- RustChunk implementation mapping: No 1.20.1 mapping; do not create a placeholder item or protocol identifier.
- Evidence required before adoption: 1.20.1 item/damage/entity-goal registries and mappings proving the feature is absent.
- Completion state: COMPLETE — 895 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 31642 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1593. `crates/horizon-client/src/mobs/species.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cc2379c9010eb7747976b44a507d7b14ae7ff870`
- Size: 44431 bytes; 1718 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1718 (EOF)
- Purpose: Central mob species metadata: entity classification, dimensions, movement flags, foods/tags, textures, sounds, model and animation identifiers.
- Dependencies / cross-references: Depends on EntityType registry, attributes/dimensions/poses, biome and item tags, sounds, model layers, renderer registry and Brain/goal families.
- Implementation / logic: Defines per-species metadata and helper lookups used by entity construction, rendering, movement, food/tempt predicates and AI dispatch.
- Findings: Registry mixes post-1.20.1 species/data. Tadpole itself is 1.20.1 content; only its later Brain redesign is excluded, not the entity or target-era AI.
- RustChunk relevance: Foundational versioned mob registry reference, but cannot be consumed without filtering and target data exports.
- Decision: rewrite required; core registry reference; version-specific
- 1.20.1 implications: Generate strict 1.20.1 EntityType/attribute/dimension/food/sound/model metadata; preserve Tadpole and its target-era goal AI; remove later species.
- Follow-up checks: Diff species list, dimensions, attributes, tags, sounds, model layers and AI family against 1.20.1 registries/mappings.
- RustChunk implementation mapping: RustChunk generated 1.20.1 mob metadata registry consumed by simulation and renderer through versioned IDs.
- Evidence required before adoption: 1.20.1 EntityType/attribute/tag/sound/model exports, dimension fixtures and entity construction tests for retained species.
- Completion state: COMPLETE — 1718 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 44431 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1594. `crates/horizon-client/src/mobs/strider.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ff5796048fa70d13ef2fb02f648faff48435413f`
- Size: 5992 bytes; 170 logical lines
- Coverage: 1–170 (EOF)
- Purpose: Strider movement and AI on lava, warmth/cold speed changes, floating and rider behavior.
- Dependencies / cross-references: Depends on lava/fluid pathfinding, warm-block tags, temperature modifier, player steering, mount state and movement controls.
- Implementation / logic: Registers goals and controls lava-seeking, warm/cold state, speed modifier, floating and animation flags.
- Findings: Strider is target-era content; warm-block tag contents, search order and movement/temperature timing require 1.20.1 verification.
- RustChunk relevance: Useful distinctive Nether mount and fluid-navigation behavior.
- Decision: adaptable; version-specific
- 1.20.1 implications: Retain target-era Strider AI and movement; adapt tags, attributes and rider controls from 1.20.1.
- Follow-up checks: Verify warm-block tag, lava path cost, cold/warm speed modifier, float/steering, dismount and passenger interactions.
- RustChunk implementation mapping: RustChunk 1.20.1 Strider AI, lava navigation profile and mount input adapter.
- Evidence required before adoption: 1.20.1 source/mappings, warm-block tag export, seeded path traces and rider/movement tests.
- Completion state: COMPLETE — 170 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5992 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1595. `crates/horizon-client/src/mobs/sulfur_cube.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `935b1bd250982434eca6a663bea2857956084b0c`
- Size: 47122 bytes; 1390 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1390 (EOF)
- Purpose: Sulfur Cube species, archetypes, swallowed-block/item state, bounce/friction variants, fuse and explosion behavior.
- Dependencies / cross-references: Depends on later species registration, archetype item tags, swallowed block/item data, entity physics, damage immunity, fuse/explosion and render assets.
- Implementation / logic: Implements archetype selection, swallowing/storing blocks or items, physical response variants, ignition/fuse, explosion and specialized state/animation.
- Findings: Sulfur Cube and its archetypes are later-version content and absent from the 1.20.1 entity/item/block registries.
- RustChunk relevance: No direct 1.20.1 gameplay use; broad kinetic-mob ideas do not establish vanilla compatibility.
- Decision: **unnecessary; post-1.20.1** — exclude; post-1.20.1
- 1.20.1 implications: Exclude the species, archetype tags, item IDs, damage immunity and explosion logic from the 1.20.1 build.
- Follow-up checks: Verify 1.20.1 EntityType/item/block registries contain no Sulfur Cube or archetype dependencies.
- RustChunk implementation mapping: No 1.20.1 implementation mapping; optional later-version research only.
- Evidence required before adoption: 1.20.1 entity/item/block registries and mappings proving species and archetype IDs are absent.
- Completion state: COMPLETE — 1390 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 47122 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1596. `crates/horizon-client/src/mobs/sync.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dd195d7697c83ec8dcecf493824538be1f77649f`
- Size: 5759 bytes; 136 logical lines
- Coverage: 1–136 (EOF)
- Purpose: Mob synchronized pose flags, entity-event routing and client animation/state update dispatch.
- Dependencies / cross-references: Depends on entity metadata indices, entity event IDs, pose flags, species registry, client tick scheduling and server-to-client packets.
- Implementation / logic: Maps entity events and synchronized state to per-species pose/animation updates, separating server events from local client interpolation/ticks.
- Findings: Synchronization boundary matters for 1.20.1, but the table includes later Warden, Breeze, Creaking, Copper Golem, Armadillo and Camel Husk species.
- RustChunk relevance: Essential architecture for correct multiplayer visuals, provided metadata and event IDs exactly match 1.20.1.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild mappings from 1.20.1 metadata/event registries; keep local animation separate from server-authoritative AI.
- Follow-up checks: Verify metadata indices, pose transitions, event IDs, interpolation order, login/reload and server/client tick responsibilities.
- RustChunk implementation mapping: RustChunk 1.20.1 mob metadata/event decoder feeding a separate client animation state machine.
- Evidence required before adoption: 1.20.1 protocol/source mappings, metadata captures, event packet fixtures, pose transition tests and multiplayer traces.
- Completion state: COMPLETE — 136 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5759 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1597. `crates/horizon-client/src/mobs/tame.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3290aa2aa492bc22a97c6ad27a250c66c5deee2a`
- Size: 70470 bytes; 1975 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1975 (EOF)
- Purpose: Tameable Wolf/Cat/Ocelot/Parrot behavior: ownership, sitting, following, defense, anger, begging, relaxation and animations.
- Dependencies / cross-references: Depends on owner UUID/entity NBT, goal/target selectors, tame items/food tags, path/teleport, damage, effects, bed/chest blocks, game rules and synchronized events.
- Implementation / logic: Registers tameable goals and target rules; follows/teleports to owners, responds to attacks, maintains wolf anger, handles wet-wolf shake and cat sleep/gifts, and computes poses/textures.
- Findings: Core tameable species exist in 1.20.1, but snapshot has later Wolf variants/equipment and changed registry/texture state. Behavior and field semantics need target-era checks.
- RustChunk relevance: High relevance to survival gameplay and synchronized mob behavior; owner/NBT and event parity are essential.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild owner/tame/anger state and goal order from 1.20.1; keep target-era Cat/Ocelot/Wolf/Parrot behavior and exclude later Wolf variants/equipment.
- Follow-up checks: Verify owner UUID NBT, tame-item interactions, sit/follow/teleport thresholds, owner defense, universal anger, wet-shake events, cat gifts, variant selection and RNG order.
- RustChunk implementation mapping: RustChunk 1.20.1 tameable state/goal service, legacy owner NBT codec and server/client event adapter.
- Evidence required before adoption: 1.20.1 source/mappings, tameable NBT fixtures, goal traces, owner combat cases, sleep/gift loot and synchronized event captures.
- Completion state: COMPLETE — 1975 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 70470 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1598. `crates/horizon-client/src/mobs/trader.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `324e9d870042189ebef622a19bb68e86c2974fca`
- Size: 9621 bytes; 322 logical lines
- Coverage: 1–322 (EOF)
- Purpose: Wandering Trader routine: invisibility potion/milk, hostile avoidance, trading pause, visit movement, home restriction and despawn delay.
- Dependencies / cross-references: Depends on effects/potion/milk, trader inventory/trades, hostile species, navigation, home radius, trading-player state, sounds and despawn NBT.
- Implementation / logic: Registers drink/avoid/trade/wander/restriction goals; applies invisibility or clears effects, moves to a visit target, pauses while trading and counts down despawn.
- Findings: Wandering Trader exists in 1.20.1. Threat species list and potion/ItemStack effect handling need target mappings and legacy serialization.
- RustChunk relevance: Useful vanilla NPC AI reference; offers and merchant inventory belong to a separate trading subsystem.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep target-era routines but connect potion/milk and trade state to 1.20.1 item/effect and merchant packet/NBT behavior.
- Follow-up checks: Verify 32-tick drink, day/night conditions, invisibility duration, milk clearing, hostile avoidance, home distance, trade pause and 48000-tick despawn.
- RustChunk implementation mapping: RustChunk 1.20.1 Wandering Trader goals/state integrated with merchant inventory and effects.
- Evidence required before adoption: 1.20.1 source/mappings, trader NBT/trade packets, effect fixtures, threat-target tests and deterministic despawn timing.
- Completion state: COMPLETE — 322 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 9621 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1599. `crates/horizon-client/src/mobs/transport_items.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8d36dcbfd8a527d4e0304bd1fd3957e06bba1b07`
- Size: 29219 bytes; 827 logical lines
- Coverage: 1–400, 401–800, 801–827 (EOF)
- Purpose: Copper Golem chest-to-chest item transport: chest search, queue/open interaction, pickup/place and visited/unreachable memory.
- Dependencies / cross-references: Depends on Copper Golem and Brain memories, copper chest blocks/tags, double-chest/container APIs, item events, pathfinding and opener state.
- Implementation / logic: Searches chest block entities, chooses source/destination, queues behind another golem, opens/interacts for a fixed duration, moves up to 16 items and records visited/unreachable positions.
- Findings: Copper Golem and copper chest transport were introduced after 1.20.1; this module relies on later Brain tasks, blocks and item/container state.
- RustChunk relevance: No direct 1.20.1 functionality; do not use copper-chest interactions as a generic vanilla container layer.
- Decision: **unnecessary; post-1.20.1** — exclude; post-1.20.1
- 1.20.1 implications: Exclude this module, transport memories, copper-chest tags and related block/event IDs from the 1.20.1 baseline.
- Follow-up checks: Confirm 1.20.1 registries contain no Copper Golem or copper chest; keep ordinary chest/container code separately versioned.
- RustChunk implementation mapping: No 1.20.1 implementation mapping; future-version research only.
- Evidence required before adoption: 1.20.1 entity/block/item/tag registries and mappings confirming absence of Copper Golem and copper chest.
- Completion state: COMPLETE — 827 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 29219 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1600. `crates/horizon-client/src/mobs/trial_spawner.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d210178aa8cb4935c7d6b75367b40570010ddb4b`
- Size: 48901 bytes; 1256 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1256 (EOF)
- Purpose: Trial Spawner configuration/state machine: player detection, waves, ominous conversion, reward ejection, item spawners and cooldown.
- Dependencies / cross-references: Depends on Trial Spawner block-entity/config codecs, Trial Omen/Bad Omen effects, Trial Chamber loot/config, entity spawn/loot services, UUID tracking and block events.
- Implementation / logic: Loads/saves normal and ominous configs, detects players, converts Bad Omen, spawns configured waves, tracks mobs, ejects per-player rewards and runs cooldown/item-spawner events.
- Findings: Trial Spawners, Trial Omen and Trial Chambers are 1.21 content and absent from 1.20.1; ordinary Monster Spawner in spawner.rs is a separate feature.
- RustChunk relevance: No direct 1.20.1 gameplay adoption.
- Decision: **unnecessary; post-1.20.1** — exclude; post-1.20.1
- 1.20.1 implications: Exclude TrialSpawner block/entity/state, Trial Omen and trial-specific packet, loot/config and resource identifiers from 1.20.1.
- Follow-up checks: Confirm absence of trial_spawner block entity/state, effects, loot tables and registries in 1.20.1; retain vanilla Mob Spawner independently.
- RustChunk implementation mapping: No 1.20.1 mapping; do not alias TrialSpawner to ordinary Monster Spawner.
- Evidence required before adoption: 1.20.1 registries, effects, block-entity types and resource manifests proving Trial Spawner/Trial Omen absent.
- Completion state: COMPLETE — 1256 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 48901 byte size match the source tree; embedded tests, where present, were inspected only and not run.
