# RustChunk Starling Audit Ledger — Entries 1601–1800

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1601–1800 (200 unique source files; one shared Ledger for this stage).
- Current progress: entries 1601–1625 complete (25/200); checkpoint 1601–1625 closed; checkpoint 1626–1650 remains authorized.
- Reporting rule: stop at entry 1650 and wait for explicit authorization before inspecting entry 1651 or later.
- Each entry contains the 15 required fields defined in AUDIT_PROTOCOL.md. Purpose is separate from dependencies, implementation, findings, relevance, decision, implications, follow-up, mapping and evidence.
- Long files are read in consecutive, gap-free ranges of at most 400 logical lines through EOF. Source SHA and byte size are checked against the pinned recursive tree.
- Embedded tests are inspected only; they are not executed.

### 1601. `crates/horizon-client/src/mobs/turtle.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e5e9b440b575cc82cdf254843022a55823e55566`
- Size: 24658 bytes; 684 logical lines
- Coverage: 1–400, 401–684 (EOF)
- Purpose: adaptable; version-specific
- Dependencies / cross-references: Turtle swimming/travel goals, egg laying, home beach, breeding and movement control.
- Implementation / logic: Turtle state, water/sand tags, navigation, breeding, block placement and RNG.
- Findings: Registers land/water goals; selects home/travel positions; handles eggs, breeding and swimming travel.
- RustChunk relevance: Target-era behavior, but travel's loaded-chunk checks need 1.20.1 parity.
- Decision: Useful vanilla mob and aquatic movement reference.
- 1.20.1 implications: Rebuild target goal timing, tags, egg placement and chunk checks for 1.20.1.
- Follow-up checks: Home radius, beach selection, egg counts, navigation, chunk edges and RNG.
- RustChunk implementation mapping: RustChunk 1.20.1 turtle AI and egg-placement adapter.
- Evidence required before adoption: 1.20.1 source, tag fixtures, chunk-edge/egg traces and seeded tests.
- Completion state: COMPLETE — 684 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 24658 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1602. `crates/horizon-client/src/mobs/undead.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bede34a88a8f1b666eddd56ad66b4200b6d018a8`
- Size: 51895 bytes; 1421 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1421 (EOF)
- Purpose: rewrite required; version-specific
- Dependencies / cross-references: Shared zombie, husk, drowned, zombified piglin, skeleton, stray, bogged and wither-skeleton behavior.
- Implementation / logic: Goals/targets, legacy equipment/NBT, damage, conversion, weather/light, projectiles and RNG.
- Findings: Registers species combat/avoidance, ranged attacks, conversion, sunlight and anger/equipment behavior.
- RustChunk relevance: Newer SpearUse/Parched branches and modern item/damage assumptions must be excluded; target classes need exact parity.
- Decision: Broad vanilla hostile mob reference.
- 1.20.1 implications: Rebuild target-era selectors, item NBT, damage and conversions; exclude SpearUse/Parched.
- Follow-up checks: Species goals, sunlight, drowned conversion, reinforcement, anger, ranged intervals, equipment and spawn RNG.
- RustChunk implementation mapping: RustChunk undead species/hostile AI and conversion service.
- Evidence required before adoption: 1.20.1 sources, per-species NBT/equipment, conversion and combat traces.
- Completion state: COMPLETE — 1421 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 51895 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1603. `crates/horizon-client/src/mobs/vex.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `432ec660ac7feb5bd9d1d15eb3faef09f3d5f4bb`
- Size: 9475 bytes; 277 logical lines
- Coverage: 1–277 (EOF)
- Purpose: adaptable; version-specific
- Dependencies / cross-references: Vex phasing flight, charge attacks, Evoker target copying and limited lifetime.
- Implementation / logic: Evoker owner/target, custom movement, phasing/collision, attack goals and timer.
- Findings: Registers goals, steers through blocks and handles charge, attack and lifetime.
- RustChunk relevance: Vex exists in 1.20.1; no-physics behavior differs from generic flying navigation.
- Decision: Useful hostile summon and special movement reference.
- 1.20.1 implications: Rebuild phasing, target handoff and owner/lifetime rules for 1.20.1.
- Follow-up checks: Phasing boundaries, entity collision, charge, damage and owner death.
- RustChunk implementation mapping: RustChunk Vex AI and dedicated phasing movement.
- Evidence required before adoption: 1.20.1 Vex/Evoker source and summon/collision/timing fixtures.
- Completion state: COMPLETE — 277 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 9475 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1604. `crates/horizon-client/src/mobs/villager.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5aca72e96be43e44ce481471e7f67b536e818b1b`
- Size: 42655 bytes; 1150 logical lines
- Coverage: 1–400, 401–800, 801–1150 (EOF)
- Purpose: rewrite required; core reference; version-specific
- Dependencies / cross-references: Villager professions, sleep, inventory/food, gossip, work-site and golem-related state.
- Implementation / logic: POI tickets, schedules, villager Brain, legacy items/NBT, trades, gossip, raids and breeding.
- Findings: Stores/updates profession, work/sleep/home, inventory, food sharing and gossip interactions.
- RustChunk relevance: POI/schedule and serialization are target-version sensitive; modern trade/inventory formats cannot be assumed.
- Decision: High-value village, trade and persistence reference.
- 1.20.1 implications: Rebuild with 1.20.1 POI, schedules, legacy ItemStack/NBT; use villager_brain for AI.
- Follow-up checks: Profession/job-site, POI tickets, gossip, food thresholds, schedule, restock and save/reload.
- RustChunk implementation mapping: RustChunk villager state, POI, inventory/trade services.
- Evidence required before adoption: 1.20.1 villager/POI source, trade/gossip and NBT fixtures.
- Completion state: COMPLETE — 1150 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 42655 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1605. `crates/horizon-client/src/mobs/villager_brain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `200483df9c7fe7278db0c120b33816eed80c0b12`
- Size: 159029 bytes; 4578 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4578 (EOF)
- Purpose: rewrite required; core reference; version-specific
- Dependencies / cross-references: Villager Brain memory/sensors/POI, schedule, work, farming, breeding, trade and raid behaviors.
- Implementation / logic: Brain TTLs, POI tickets, pathfinding, tags, professions, schedules, raids, gossip and RNG.
- Findings: Implements sensors/memories, activities, weighted behaviors, work/trade/farm, sleep/home and raid response.
- RustChunk relevance: Relevant Brain architecture, but provider order, POI/activity timing and RNG in 26.3 must be rebuilt from target source.
- Decision: Central villager AI and reusable memory scheduling reference.
- 1.20.1 implications: Rebuild 1.20.1 Brain providers, sensors, TTLs, activities, behavior order and POI keys.
- Follow-up checks: Sensor cadence/order, memory expiry, activity priority, POI tickets, schedules, raid and RNG.
- RustChunk implementation mapping: RustChunk villager Brain/memory/sensor/behavior engine.
- Evidence required before adoption: 1.20.1 Brain providers, POI/tags, schedules and deterministic behavior traces.
- Completion state: COMPLETE — 4578 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 159029 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1606. `crates/horizon-client/src/mobs/villager_farming_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `de42b3873b2d8a5e4a0394aa98548cc5c97ae0aa`
- Size: 10793 bytes; 322 logical lines
- Coverage: 1–322 (EOF)
- Purpose: reference; tests only; version-specific
- Dependencies / cross-references: Regression-test checklist for villager farming, modded crops and seed selection.
- Implementation / logic: Prepared resource packs, crop/block registry, tags, villager farming and test-world fixtures.
- Findings: Defines farming scenarios for crop age, harvest/replant and data/tag-driven seed choices.
- RustChunk relevance: Some cases ignored or need 26.3 resources; read only, not executed; no 1.20.1 proof.
- Decision: Checklist of farming edge cases, not implementation.
- 1.20.1 implications: Use as test-design guide; build new 1.20.1 fixtures and expected values.
- Follow-up checks: Crop age, harvest/replant, seed availability, mod tags and authoritative block changes.
- RustChunk implementation mapping: RustChunk 1.20.1 farming integration tests.
- Evidence required before adoption: Runnable 1.20.1 resources, crop/tag registry and deterministic farm traces.
- Completion state: COMPLETE — 322 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 10793 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1607. `crates/horizon-client/src/mobs/villager_hats.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b81fa35ef85813a780baf1a9f4a21f440229ce1d`
- Size: 4817 bytes; 128 logical lines
- Coverage: 1–128 (EOF)
- Purpose: reference; optional client rendering; version-specific
- Dependencies / cross-references: Villager/zombie-villager hat visibility from texture metadata.
- Implementation / logic: PNG metadata, texture resources, profession/biome textures and model head layers.
- Findings: Reads texture metadata and sets headwear visibility to none/partial/full.
- RustChunk relevance: Optional client feature; ignored tests use newer prepared assets; no gameplay effect.
- Decision: Small optional render feature.
- 1.20.1 implications: Consider only after 1.20.1 texture/model/resource-pack parity.
- Follow-up checks: PNG metadata format, profession textures, visibility rules and fallback.
- RustChunk implementation mapping: Optional villager headwear render adapter.
- Evidence required before adoption: 1.20.1 texture assets, metadata fixtures and model-layer goldens.
- Completion state: COMPLETE — 128 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 4817 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1608. `crates/horizon-client/src/mobs/warden.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e1419102ef6e9a467eef437b343292117c18782f`
- Size: 35556 bytes; 1075 logical lines
- Coverage: 1–400, 401–800, 801–1075 (EOF)
- Purpose: adaptable; version-specific
- Dependencies / cross-references: Warden anger/vibration, targeting, melee/sonic boom, Darkness and pose state.
- Implementation / logic: Warden Brain, sculk/vibration registry, target visibility, damage/effects, sounds and movement.
- Findings: Handles state, anger/targets, vibration/hurt/push reactions, attacks and animations.
- RustChunk relevance: Target-era mob; event IDs, tags and effect cadence need 1.20.1 parity.
- Decision: Important hostile/sculk reference.
- 1.20.1 implications: Keep target-era behavior after matching event IDs, listeners, anger and damage semantics.
- Follow-up checks: Vibration occlusion/range, anger, target priority, sniff/roar, sonic boom and Darkness timing.
- RustChunk implementation mapping: RustChunk Warden state/AI and sculk event adapter.
- Evidence required before adoption: 1.20.1 Warden/sculk source, event registry and deterministic traces.
- Completion state: COMPLETE — 1075 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 35556 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1609. `crates/horizon-client/src/mobs/warden_brain.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8237dfd9c33d0e4ae8d4030d1ee687215fac89ac`
- Size: 45667 bytes; 1308 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1308 (EOF)
- Purpose: rewrite required; core reference; version-specific
- Dependencies / cross-references: Warden Brain memory/sensors/activities for investigation, sniffing, roaring, digging and combat.
- Implementation / logic: Warden state, scan cadence, memory TTLs, vibration disturbances, pathfinding, attacks and RNG.
- Findings: Builds Brain provider/activity packages and ticks sensor/memory/behavior routines.
- RustChunk relevance: Provider order, TTLs, behavior priorities and RNG need exact 1.20.1 source verification.
- Decision: Core Warden Brain reference and possible reusable memory scheduling.
- 1.20.1 implications: Rebuild target-era memory keys, sensors, activity packages and timing; keep separate from generic goals.
- Follow-up checks: Sensor delays, disturbance/anger, sniff/roar/dig duration, behavior order and RNG.
- RustChunk implementation mapping: RustChunk Warden Brain module on versioned memory/sensor engine.
- Evidence required before adoption: 1.20.1 Brain source and tick-by-tick Warden traces.
- Completion state: COMPLETE — 1308 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 45667 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1610. `crates/horizon-client/src/mobs/witch.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `167a4c33173738bb8e337e80b12d39ed1b847129`
- Size: 13666 bytes; 407 logical lines
- Coverage: 1–400, 401–407 (EOF)
- Purpose: adaptable; version-specific
- Dependencies / cross-references: Witch potion drinking, defensive potion choice, thrown-potion attacks and raider support.
- Implementation / logic: Potion/effect registry, targets/visibility, projectile physics, damage, raiders and RNG.
- Findings: Chooses beneficial potions, selects splash-potion types and throws on goal/cooldown schedule.
- RustChunk relevance: Target-era but potion registry, cooldown and projectile RNG need 1.20.1 parity.
- Decision: Useful hostile ranged/potion AI.
- 1.20.1 implications: Adapt with target potion registry, legacy item data, target predicates and RNG.
- Follow-up checks: Potion choice, drinking delay, trajectory, raider support and effect duration.
- RustChunk implementation mapping: RustChunk Witch AI and splash projectile adapter.
- Evidence required before adoption: 1.20.1 potion/effect registry, Witch source and seeded projectile traces.
- Completion state: COMPLETE — 407 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 13666 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1611. `crates/horizon-client/src/mobs/wither.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `086f8cdd88dc60804eda826bbd5da07b2f5a63bd`
- Size: 21304 bytes; 554 logical lines
- Coverage: 1–554 (EOF)
- Purpose: adaptable; version-specific; boss simulation
- Dependencies / cross-references: Wither spawn charge, explosion, multi-head target, skull projectiles, block damage, healing and immunity.
- Implementation / logic: Wither timers, targets, skull physics, damage tags, block resistance/destruction and RNG.
- Findings: Implements spawn charge/explosion, head attacks, skull variants, block damage and server/client state.
- RustChunk relevance: Target-era boss, but damage tags, block list, difficulty and event behavior need 1.20.1 parity.
- Decision: High-value boss reference; authoritative state remains server/local host.
- 1.20.1 implications: Adapt exact target registries, damage, blocks and projectile physics.
- Follow-up checks: Charge, radius, head cadence, skull damage, block rules, arrow immunity and regeneration.
- RustChunk implementation mapping: RustChunk Wither state machine, projectile and block-damage adapters.
- Evidence required before adoption: 1.20.1 Wither source, boss traces, skull/damage/block fixtures.
- Completion state: COMPLETE — 554 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 21304 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1612. `crates/horizon-client/src/mobs/world.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ad49ac7f4df642d913ddf89a87140ed444fa6ecd`
- Size: 20343 bytes; 538 logical lines
- Coverage: 1–400, 401–538 (EOF)
- Purpose: rewrite required; core reference; version-specific
- Dependencies / cross-references: Mob AI world-query adapter for collision, ray clipping, environment and nearby entities.
- Implementation / logic: World/chunk state, block facts, collisions, entity bounds/index, sky/light/biome/fluid and targets.
- Findings: Provides line-of-sight, collision/clip, environmental, target and nearby-grid queries with fallback.
- RustChunk relevance: Ray flags, unloaded chunks, collision context and nearby iteration order need 1.20.1 parity.
- Decision: Foundational for all local mob AI; query cost matters.
- 1.20.1 implications: Rebuild over RustChunk's 1.20.1 world model; preserve query semantics before optimizing.
- Follow-up checks: Ray modes, bounds/order, height/border, chunk loading, sky/light/biome and fallback.
- RustChunk implementation mapping: RustChunk MobWorld query interface over chunk/collision services.
- Evidence required before adoption: 1.20.1 World/Level mappings, ray/entity query goldens and chunk-edge fixtures.
- Completion state: COMPLETE — 538 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 20343 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1613. `crates/horizon-client/src/mobs/zombie_villager.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0ab7f517ea39de3c7bf32d493ba8331effaf42cf`
- Size: 9507 bytes; 271 logical lines
- Coverage: 1–271 (EOF)
- Purpose: adaptable; version-specific
- Dependencies / cross-references: Zombie Villager cure timer, Weakness/Strength, acceleration near iron bars/beds and saved conversion state.
- Implementation / logic: Effects, cure interaction, tags, entity conversion, gossip, NBT and world time.
- Findings: Ticks cure state, applies tagged-block speedups and preserves fields for conversion.
- RustChunk relevance: Actual entity replacement occurs outside this module; target NBT and RNG must be confirmed.
- Decision: Useful survival mechanic and persistence reference.
- 1.20.1 implications: Adapt state into 1.20.1 conversion transaction and NBT.
- Follow-up checks: Cure duration/conditions, iron bars/beds, effects, gossip and conversion equipment/attributes.
- RustChunk implementation mapping: RustChunk ZombieVillager cure state and conversion adapter.
- Evidence required before adoption: 1.20.1 source, cure NBT, tags and conversion traces.
- Completion state: COMPLETE — 271 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 9507 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1614. `crates/horizon-client/src/mod_api.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0c899f4756938c73499f46b55d6f19d46590a9cf`
- Size: 27788 bytes; 753 logical lines
- Coverage: 1–400, 401–753 (EOF)
- Purpose: reference; not Forge; architecture only
- Dependencies / cross-references: Starling native mod API/lifecycle, Host, entity/world handles, attachments and hook dispatch.
- Implementation / logic: Starling native runtime, Host, Rust mod ABI, world/entity/item/menu and process lifecycle.
- Findings: Defines native mod contracts and dispatches lifecycle/client/server hooks through Starling runtime.
- RustChunk relevance: Not Forge ModLoader/FML/event bus/ModLauncher; tightly coupled to Starling types.
- Decision: Architecture reference for separate Rust-native extension API, not Java Forge.
- 1.20.1 implications: Keep separate from required Forge 1.20.1 path.
- Follow-up checks: Map lifecycle/handle ideas against Forge 47.x; inspect threading, cancellation and isolation.
- RustChunk implementation mapping: Optional RustChunk native extension API isolated from Forge.
- Evidence required before adoption: Forge 47.x API map and independent ABI/lifecycle/security design.
- Completion state: COMPLETE — 753 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 27788 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1615. `crates/horizon-client/src/mod_api/generated.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3bdd4a940382a0cd18bd63e8c9aa91f03cf88936`
- Size: 502223 bytes; 14334 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4800, 4801–5200, 5201–5600, 5601–6000, 6001–6400, 6401–6800, 6801–7200, 7201–7600, 7601–8000, 8001–8400, 8401–8800, 8801–9200, 9201–9600, 9601–10000, 10001–10400, 10401–10800, 10801–11200, 11201–11600, 11601–12000, 12001–12400, 12401–12800, 12801–13200, 13201–13600, 13601–14000, 14001–14334 (EOF)
- Purpose: reference; not Forge; generated native API
- Dependencies / cross-references: Generated Starling native hook declarations/dispatch tables for client/server subsystems.
- Implementation / logic: Hook schema/version, NativeClientHooks, component/item/data APIs, entities, menus, registries and rendering.
- Findings: Declares hook contracts/dispatch glue for lifecycle, world, item/component, block, entity, menu, recipe, render and client events.
- RustChunk relevance: Contains modern Component/DataMap/ItemContainer and Starling types; not Forge event compatibility.
- Decision: Catalogue of extension points; direct copy binds RustChunk to another engine.
- 1.20.1 implications: Use architecture only; design separate RustChunk/Forge bridge around 1.20.1 events/registries.
- Follow-up checks: Classify hooks against Forge 47.x; audit threading, cancellation, errors and isolation.
- RustChunk implementation mapping: Optional versioned native hook schema, separate from ForgeModLoader.
- Evidence required before adoption: Forge 47.x API map, 1.20.1 hook needs, ABI/security and schema tests.
- Completion state: COMPLETE — 14334 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 502223 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1616. `crates/horizon-client/src/mod_api/generated_host.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fe82c2cb860326f7e0334d605513ef0fdddc6192`
- Size: 61315 bytes; 1345 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1345 (EOF)
- Purpose: reference; not Forge; architecture only
- Dependencies / cross-references: Generated Starling Host calls for world/entity/player, inventory/menu, item/fluid transfer, render and recipes.
- Implementation / logic: Generated API, Host handles, Starling registry/items, world, UI/menu and renderer.
- Findings: Maps native calls to Host operations and marshals Starling data to runtime.
- RustChunk relevance: Uses Starling-native types; does not load/execute Forge Java mods.
- Decision: Architecture reference for extension-host boundaries only.
- 1.20.1 implications: Do not use as Forge layer; isolate optional native Host.
- Follow-up checks: Review ownership/lifetimes, thread affinity, reentrancy, permissions and versioned handles.
- RustChunk implementation mapping: Optional RustChunk Host over scoped world/entity/item handles.
- Evidence required before adoption: Forge bridge proof, threading/ownership model, permissions and isolation tests.
- Completion state: COMPLETE — 1345 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 61315 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1617. `crates/horizon-client/src/mod_content.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f81c71ff533342a0818aa68b422c90b86cb0932f`
- Size: 115395 bytes; 2828 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–2828 (EOF)
- Purpose: rewrite required; partial Forge data overlay; version-specific
- Dependencies / cross-references: Starling content declarations/resource overlay for blocks/items/tags/registries/structures and selected worldgen.
- Implementation / logic: Manifests, assets/resources, registry IDs, block/item states, NBT, tags, biome/structure and overlay cache.
- Findings: Clones templates, applies state/item additions, rewrites registry/tag data, merges selected worldgen, caches collision NBT and links/copies overlay.
- RustChunk relevance: Starling/Horizon, not full ModLauncher/FML/Java execution or all Forge serializers; partial readers do not prove complete Forge.
- Decision: Offline data-overlay reference only; insufficient alone for Forge support.
- 1.20.1 implications: Rebuild 1.20.1 registries/NBT/blockstates/resources; use as pipeline reference, not Forge loading.
- Follow-up checks: Overlay precedence, registry stability, state expansion, tag merge, biome modifiers, structures, serializers and cache/memory.
- RustChunk implementation mapping: RustChunk data staging plus separate Forge loader/registry service.
- Evidence required before adoption: Forge 47.x source, 1.20.1 registry/tag fixtures, mod packs and overlay tests.
- Completion state: COMPLETE — 2828 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 115395 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1618. `crates/horizon-client/src/mod_content_biomes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4b3940209920baaf23af999d7b563ad8ee54f279`
- Size: 12795 bytes; 330 logical lines
- Coverage: 1–330 (EOF)
- Purpose: reference; partial Forge/NeoForge format support; version-specific
- Dependencies / cross-references: Subset of Forge/NeoForge biome modifier parsing for add_features/remove_features.
- Implementation / logic: Modifier paths, biome/placed-feature registries, tags, selectors and generation order.
- Findings: Parses selectors, resolves holders by ID/tag/any/or/and/not, adds/removes features per step.
- RustChunk relevance: Skips spawn/carver and unknown/custom serializers; modern snapshot not proof of Forge 47.x 1.20.1.
- Decision: Useful clue for biome modifier support, incomplete.
- 1.20.1 implications: Reimplement from Forge 47.x 1.20.1 codecs/registries and support required serializers.
- Follow-up checks: Modifier types/order, holder/tag resolution, missing IDs, paths and custom serializers.
- RustChunk implementation mapping: RustChunk Forge biome modifier parser/ordered feature patcher.
- Evidence required before adoption: Forge 47.x codecs/registries, serializers, tags and generation fixtures.
- Completion state: COMPLETE — 330 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 12795 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1619. `crates/horizon-client/src/mod_content_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d354197a7a09ea00e7344f2a2273548554e645e7`
- Size: 164381 bytes; 4102 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4102 (EOF)
- Purpose: reference; tests only; version-specific
- Dependencies / cross-references: Tests for content overlay, registries/states, tags, worldgen/structures, voxel collision and overlay cache.
- Implementation / logic: Prepared 26.3 packs, registry/tag/worldgen schema, manifests, NBT/collision and overlay filesystem.
- Findings: Defines cases for block/item states, IDs, tags, biome modifiers, structures, VoxelShapes, declarations, signs and cache semantics.
- RustChunk relevance: Some tests ignored or need 26.3 resources; inspected only, not run; no 1.20.1/Forge proof.
- Decision: Checklist for replacement data overlay tests, not implementation.
- 1.20.1 implications: Port selected cases using 1.20.1 Forge fixtures and add custom serializer/actual Forge-load tests.
- Follow-up checks: Registry determinism, overlay precedence, tag replacement, modifier order, structures, collision, NBT and cache.
- RustChunk implementation mapping: RustChunk 1.20.1 Forge/data regression suite.
- Evidence required before adoption: Runnable 1.20.1 fixture, registry/tag/structure data, modded serializers and CI logs.
- Completion state: COMPLETE — 4102 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 164381 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1620. `crates/horizon-client/src/motion.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `82ade0b0249368dd00eae61bb9be64a91d85bca8`
- Size: 222939 bytes; 5459 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4800, 4801–5200, 5201–5459 (EOF)
- Purpose: rewrite required; core reference; version-specific
- Dependencies / cross-references: Player input/movement, collision, jump/fall, Elytra, swimming, flight/spectator, effects and synchronization.
- Implementation / logic: AABB/collision, block/fluid facts, attributes/effects, health/hunger, entities, world/chunks and network synchronization.
- Findings: Defines PlayerMotion/MotionWorld and coordinates movement, jumps, collision, fluid/environment, fall, Elytra, spectator, sprint and events.
- RustChunk relevance: Newer restitution/bounciness/air-drag and 26.3 assumptions; some vehicle/current domain incomplete.
- Decision: Core correctness/performance hotspot; no FPS/performance claim measured.
- 1.20.1 implications: Rebuild from 1.20.1 LocalPlayer/Player/LivingEntity; isolate later physics and add prediction only after parity.
- Follow-up checks: Input/acceleration, sprint, jump, step/collision, fall, fluid, Elytra/vehicles, spectator, friction/gravity, effects and reconciliation.
- RustChunk implementation mapping: RustChunk 1.20.1 movement kernel split into input, living physics, fluid, collision, effects and network services.
- Evidence required before adoption: 1.20.1 source/mappings, movement/collision/fluid goldens, packet reconciliation and Android profiling.
- Completion state: COMPLETE — 5459 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 222939 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1621. `crates/horizon-client/src/motion/environment.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6864016fd37cb26230c8472ea13c0f5d3e25df0c`
- Size: 14570 bytes; 386 logical lines
- Coverage: 1–386 (EOF)
- Purpose: rewrite required; version-specific
- Dependencies / cross-references: Environmental damage/liquid movement: fire, air/drowning, lava, bubble columns and fluid jumps.
- Implementation / logic: Player effects/attributes, damage types, fluid states/tags, breath events, collision and RNG.
- Findings: Applies fire timers/damage, air/drowning, lava/void/wall hazards, fire contact, liquid jump and bubble push.
- RustChunk relevance: Contains Nautilus breathing and native hooks plus modern registry assumptions; Nautilus is later-version.
- Decision: Useful environment/authoritative damage decomposition.
- 1.20.1 implications: Rebuild 1.20.1 fire/drowning/lava; remove Nautilus breathing/native hook semantics unless separately supported.
- Follow-up checks: Air/Respiration, drowning, fire intervals, lava fall rules, bubble push, void and fluid jump.
- RustChunk implementation mapping: RustChunk 1.20.1 environment and bubble-column services.
- Evidence required before adoption: 1.20.1 damage/effect registry, source, fire/air NBT and fluid tests.
- Completion state: COMPLETE — 386 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 14570 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1622. `crates/horizon-client/src/motion/fluids.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9ef5e4ad3f10805b4b68431aea471f3b93bc3e77`
- Size: 20118 bytes; 521 logical lines
- Coverage: 1–400, 401–521 (EOF)
- Purpose: rewrite required; core reference; version-specific
- Dependencies / cross-references: Player fluid immersion/flow, splash, bubble-column and passenger interaction.
- Implementation / logic: Fluid states/tags/heights/shapes, flow rules, block facts, collision, chunks, boats and movement.
- Findings: Precomputes fluid facts; tracks immersion/first tick; updates currents, caches shapes, emits splash/bubble events.
- RustChunk relevance: Modern breath/boat assumptions (Nautilus) and custom section strategy require target rebuild.
- Decision: Core physics reference; cache idea may inform mobile optimization but unprofiled.
- 1.20.1 implications: Reimplement 1.20.1 FluidState/FluidTags/Entity.updateFluidInteraction; filter later extensions.
- Follow-up checks: Waterlogged flow, surface, eye immersion, boat passenger, bubble, first tick, splash RNG and cache memory.
- RustChunk implementation mapping: RustChunk 1.20.1 fluid subsystem with bounded shape cache.
- Evidence required before adoption: 1.20.1 fluid registries, flow vectors, passenger/splash fixtures and Android profiling.
- Completion state: COMPLETE — 521 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 20118 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1623. `crates/horizon-client/src/multiplayer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `78f3f69cdb5a65de3fad1c99687fb1ae003d3857`
- Size: 104281 bytes; 2611 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2611 (EOF)
- Purpose: rewrite required; client UI; protocol-specific
- Dependencies / cross-references: Multiplayer UI: server list/edit/delete, ping/status/favicon, LAN, direct connect, friend invite and reconnect.
- Implementation / logic: servers.dat, status/legacy ping, LAN scanner, friend/relay, UI and filesystem.
- Findings: Loads/saves servers, pings, displays MOTD/players/favicon and handles list/edit/direct/friend/disconnected screens.
- RustChunk relevance: UI reusable; ping and friend/relay are Starling/Horizon-specific, not vanilla Forge.
- Decision: Useful server-list/LAN/status reference.
- 1.20.1 implications: Rebuild ping/join path for 1.20.1 protocol/Forge handshake; port UI/LAN separately.
- Follow-up checks: servers.dat, status protocol, LAN, favicon, relay security, reconnect and thread cancellation.
- RustChunk implementation mapping: RustChunk server list/editor, 1.20.1 status adapter and LAN discovery.
- Evidence required before adoption: 1.20.1 status protocol, servers.dat fixtures, LAN traces and secure invite proof.
- Completion state: COMPLETE — 2611 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 104281 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1624. `crates/horizon-client/src/music.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f7eee2487d39837fd13a284b943c6e10e622ccab`
- Size: 38581 bytes; 1082 logical lines
- Coverage: 1–400, 401–800, 801–1082 (EOF)
- Purpose: adaptable; version-specific; optional client feature
- Dependencies / cross-references: Situational music, frequency/delay, fades, native music layers and track grouping.
- Implementation / logic: Sound registry/pools, biome/dimension attributes, options, sound engine, RNG and JSON resources.
- Findings: Loads environment music tables; selects menu/game/end/creative/underwater music; schedules/fades playback and layers.
- RustChunk relevance: Modern audio attributes/native layers may differ from 1.20.1.
- Decision: Optional client feature; music concepts reusable.
- 1.20.1 implications: Adapt baseline event/pool behavior to 1.20.1; isolate native layers/modern attributes.
- Follow-up checks: Music constants/delay/replace, biome precedence, fades, event paths and RNG.
- RustChunk implementation mapping: RustChunk optional 1.20.1 music manager.
- Evidence required before adoption: 1.20.1 Music/Musics source, sound pools and deterministic tests.
- Completion state: COMPLETE — 1082 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 38581 byte size match the source tree; embedded tests, where present, were inspected only, not run.

### 1625. `crates/horizon-client/src/native_client_hooks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `639895c39392ef342a079dfbe98b3e1c87f703f2`
- Size: 2967 bytes; 108 logical lines
- Coverage: 1–108 (EOF)
- Purpose: reference; not Forge; architecture only
- Dependencies / cross-references: Native callback table for menu slots, living heal/breathe/drown/fall/jump and ingredient handling.
- Implementation / logic: NativeClientHooks ABI, Rust ItemStack, JSON ingredient and OnceLock.
- Findings: Installs fixed function pointers once and dispatches callbacks.
- RustChunk relevance: Starling bridge, not Forge event bus/FML; direct calls, first install wins, no visible panic isolation.
- Decision: Minimal native extension-hook design reference only.
- 1.20.1 implications: Do not use for Forge integration; separate Rust API needs versioning/isolation/permissions.
- Follow-up checks: Install ordering/duplicates, panics/reentrancy, thread affinity, cancellation, ABI/lifetimes.
- RustChunk implementation mapping: Optional RustChunk native hooks separate from Forge.
- Evidence required before adoption: Forge event map, Rust ABI/version design and security tests.
- Completion state: COMPLETE — 108 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 2967 byte size match the source tree; embedded tests, where present, were inspected only, not run.
