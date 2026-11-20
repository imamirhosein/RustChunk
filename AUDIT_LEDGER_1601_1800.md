# RustChunk Starling Audit Ledger — Entries 1601–1800

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1601–1800 (200 unique source files; one shared Ledger for this stage).
- Current progress: entries 1601–1775 complete (175/200); reporting sections 1601–1650, 1651–1700 and 1701–1750 are closed; checkpoint 1751–1775 is complete; checkpoint 1776–1800 remains authorized.
- Reporting rule: STOP at entry 1800. Do not inspect entry 1801 or later before fresh explicit authorization.
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

### 1626. `ROADMAP.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b090355a168341625d58d6fc4bbc1b50b06b6b1`
- Size: 61791 bytes; 907 logical lines
- Coverage: 1–400, 401–800, 801–907 (EOF)
- Purpose: Project roadmap and staged compatibility/feature plan.
- Dependencies / cross-references: Project scope, launcher/client split, server protocol and mod architecture.
- Implementation / logic: Documents milestones and separates native Rust extensions from later Java/Forge ambitions.
- Findings: Planning material, not implementation; native mods and Java Forge mods are distinct targets.
- RustChunk relevance: Roadmap helps prevent scope confusion, especially around Forge.
- Decision: reference; project planning only
- 1.20.1 implications: Do not infer Forge/FML compatibility or completion from roadmap wording.
- Follow-up checks: Compare claims against code, manifests, builds and tests.
- RustChunk implementation mapping: RustChunk milestone/architecture decision records.
- Evidence required before adoption: Verified implementation, build/test/device evidence for each claimed milestone.
- Completion state: COMPLETE — 907 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 61791 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1627. `SYNC.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0c000a6a8a72256506135da5c6a2b1c01d1023db`
- Size: 2600 bytes; 43 logical lines
- Coverage: 1–43 (EOF)
- Purpose: Repository synchronization and contributor handoff notes.
- Dependencies / cross-references: Branch/worktree state, Git remotes and current implementation handoff.
- Implementation / logic: Records sync context and commands/notes for coordinating changes.
- Findings: Operational notes can become stale; not authoritative product behavior or build record.
- RustChunk relevance: Useful only for audit handoff continuity.
- Decision: reference; workflow notes only
- 1.20.1 implications: Confirm current repository/branch before reusing any instruction.
- Follow-up checks: Check referenced commits and paths against current Git state.
- RustChunk implementation mapping: RustChunk audit handoff notes.
- Evidence required before adoption: Current remote refs, commit graph, clean status and reproducible command outcomes.
- Completion state: COMPLETE — 43 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 2600 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1628. `crates/horizon-client/src/native_data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9cf3dda3e9d416dcc52c36f6d661f592d7c0dc86`
- Size: 2311 bytes; 53 logical lines
- Coverage: 1–53 (EOF)
- Purpose: Compile-time/runtime lookup of data files shipped by Starling native mods.
- Dependencies / cross-references: Cargo workspace, CARGO_MANIFEST_DIR, executable path and native mod post-build staging.
- Implementation / logic: Exports native_data! include_bytes macro and resolves native-data/<mod id> beside executable.
- Findings: Starling native Rust-mod packaging, not Forge mods directory, Java classloading or resource loader.
- RustChunk relevance: Potential optional Rust-native extension packaging pattern.
- Decision: reference; native Rust extension data only; not Forge
- 1.20.1 implications: Do not present as Forge support; define separate versioned RustChunk native extension format if needed.
- Follow-up checks: Verify path traversal, mod ID validation, APK packaging and Android executable/data paths.
- RustChunk implementation mapping: Optional RustChunk native-data packaging isolated from Forge/Java mods.
- Evidence required before adoption: Cargo tests, Android package layout, path validation and mod ID/ABI policy.
- Completion state: COMPLETE — 53 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 2311 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1629. `crates/horizon-client/src/native_tombstone.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0d54f86448a5cdd2aa1ff27af936cdd7f3822ac9`
- Size: 5380 bytes; 169 logical lines
- Coverage: 1–169 (EOF)
- Purpose: Bounded, privacy-conscious parser for Android protobuf tombstones.
- Dependencies / cross-references: Android debuggerd protobuf fields and UTF-8/varint parsing.
- Implementation / logic: Extracts signal/crashed thread/limited frames; omits registers, memory dumps, logs, descriptors and command lines.
- Findings: Input capped at 1 MiB, fields at 10,000, frames at 64; malformed input is bounded.
- RustChunk relevance: Directly useful for Android native crash diagnostics.
- Decision: adaptable; Android diagnostics
- 1.20.1 implications: Adapt only with pinned Android schema and strict privacy limits; not gameplay.
- Follow-up checks: Fuzz malformed varints, wire types, oversized lengths and real tombstones.
- RustChunk implementation mapping: Opt-in local-first RustChunk Android crash summarizer.
- Evidence required before adoption: Fuzz/property tests, real-device tombstones, privacy review and bounded-memory tests.
- Completion state: COMPLETE — 169 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5380 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1630. `crates/horizon-client/src/nether_portal.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `709d16e112516aaa61a004096c6f68e574021266`
- Size: 2392 bytes; 80 logical lines
- Coverage: 1–80 (EOF)
- Purpose: Nether portal entry/cooldown state and entity transition timing.
- Dependencies / cross-references: Entity ticks, portal position, transition delay and dimension-change authority.
- Implementation / logic: Tracks portal process, inside state, cooldown, decay and entry block.
- Findings: Concept is vanilla-era; exact 1.20.1 tick order and server authority need parity.
- RustChunk relevance: Small reference for portal state correctness.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild against 1.20.1 Entity portal tick order and ownership.
- Follow-up checks: Verify cooldowns, transition duration, re-entry and interrupted portal cases.
- RustChunk implementation mapping: RustChunk entity portal-state component.
- Evidence required before adoption: 1.20.1 source/mappings, tick traces and dimension-transition packet tests.
- Completion state: COMPLETE — 80 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 2392 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1631. `crates/horizon-client/src/no_resources.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7b639201d6e471d4044519713304295a63a6b0bf`
- Size: 9373 bytes; 278 logical lines
- Coverage: 1–278 (EOF)
- Purpose: Fallback screen when executable starts without launcher-prepared Minecraft files.
- Dependencies / cross-references: Bundled UI theme/fonts, launcher URL and prepared-resource discovery.
- Implementation / logic: Draws explanation and Download/Quit actions with shared layout/hit-testing.
- Findings: Horizon-specific launcher UX, not a Minecraft screen; URL/branding are product-specific.
- RustChunk relevance: Useful startup failure UX pattern.
- Decision: reference; launcher startup UX
- 1.20.1 implications: Adapt only if RustChunk has a launcher; do not reuse Horizon branding/URL.
- Follow-up checks: Test small screens, localization, touch targets and missing resources.
- RustChunk implementation mapping: RustChunk pre-client startup guard screen.
- Evidence required before adoption: UI/device tests, localized copy and verified official download destination.
- Completion state: COMPLETE — 278 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 9373 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1632. `crates/horizon-client/src/old_world.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `991b8acbdee907510337b343afcc74ec32097bd2`
- Size: 11133 bytes; 299 logical lines
- Coverage: 1–299 (EOF)
- Purpose: Compatibility UI/state for old or customized worlds.
- Dependencies / cross-references: World summary, DataVersion, generator settings, packs and file-fixer availability.
- Implementation / logic: Classifies old/customized worlds and exposes compatibility decisions.
- Findings: Upgrade semantics target 26.3 layout; not equivalent to 1.20.1 compatibility.
- RustChunk relevance: Useful for safe world-open warnings.
- Decision: reference; world compatibility
- 1.20.1 implications: Rebuild checks around 1.20.1 DataVersion and supported DataFixers.
- Follow-up checks: Test 1.20.1 worlds, old versions, custom dimensions and backup/rollback.
- RustChunk implementation mapping: RustChunk world compatibility summary and guarded migration.
- Evidence required before adoption: 1.20.1 DataFixer registry, sample worlds and backup/rollback tests.
- Completion state: COMPLETE — 299 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 11133 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1633. `crates/horizon-client/src/patrol.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `59c7539d90ac04962cdfaf0c1554fa5914d6b23e`
- Size: 12534 bytes; 318 logical lines
- Coverage: 1–318 (EOF)
- Purpose: Pillager patrol spawning, leader selection and movement/target behavior.
- Dependencies / cross-references: World time/difficulty, spawn rules, biome/light, player proximity, raider AI and RNG.
- Implementation / logic: Selects patrol positions and constructs/updates patrol members and captain state.
- Findings: Target-era behavior, but newer world/spawn APIs and server authority are present.
- RustChunk relevance: Relevant to local-world simulation only.
- Decision: adaptable; version-specific; server-side
- 1.20.1 implications: Remote clients must accept server spawn packets, not simulate patrols.
- Follow-up checks: Verify 1.20.1 chance, interval, captain, biome and distance predicates.
- RustChunk implementation mapping: RustChunk local patrol scheduler and captain metadata.
- Evidence required before adoption: 1.20.1 PatrolSpawner source and deterministic spawn fixtures.
- Completion state: COMPLETE — 318 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 12534 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1634. `crates/horizon-client/src/perf_overlay.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fd0827f518189b1b4265820c730483e178c6deff`
- Size: 5420 bytes; 152 logical lines
- Coverage: 1–152 (EOF)
- Purpose: On-screen performance overlay for frame/CPU/GPU samples.
- Dependencies / cross-references: Performance samples, UI draw layer, frame clock and graph scaling.
- Implementation / logic: Builds labels/graphs and renders recent measurements.
- Findings: Overlay displays data; it does not improve FPS or prove bottleneck cause.
- RustChunk relevance: Useful for diagnosing frame-time spikes.
- Decision: reference; optional diagnostics
- 1.20.1 implications: Port only after defining stable metrics and low overhead.
- Follow-up checks: Verify units, sample window, dropped samples, GPU timing and overlay overhead.
- RustChunk implementation mapping: RustChunk optional performance HUD.
- Evidence required before adoption: Profiler correlation, frame pacing and measurement-overhead tests.
- Completion state: COMPLETE — 152 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5420 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1635. `crates/horizon-client/src/performance.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a5e6151e77e2732aa77f677f70b1a8224f8acb37`
- Size: 12449 bytes; 328 logical lines
- Coverage: 1–328 (EOF)
- Purpose: Performance telemetry and sample aggregation for diagnostic overlay.
- Dependencies / cross-references: Frame loop, CPU/GPU timers, counters and bounded sample storage.
- Implementation / logic: Collects performance samples and exposes rolling statistics/spike-oriented values.
- Findings: Instrumentation identifies stalls only with trustworthy clocks/providers; no measured gain is claimed.
- RustChunk relevance: High relevance to the user's progressive FPS decline.
- Decision: reference; performance diagnostics
- 1.20.1 implications: Adapt to Android graphics capabilities; never block render thread on GPU queries.
- Follow-up checks: Check ring bounds, monotonic clock, percentiles, GPU query stalls and queue correlation.
- RustChunk implementation mapping: RustChunk low-overhead telemetry/frame-time recorder.
- Evidence required before adoption: Galaxy A22 profiler traces and overhead comparison with telemetry disabled.
- Completion state: COMPLETE — 328 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 12449 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1636. `crates/horizon-client/src/pick_item.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b8eb7e500f843608636e3025cc455ce22b489152`
- Size: 5027 bytes; 132 logical lines
- Coverage: 1–132 (EOF)
- Purpose: Pick-block/item behavior for client interaction.
- Dependencies / cross-references: Hit result, block state, creative inventory, item registry and packets.
- Implementation / logic: Maps targeted block/entity interaction to selected/requested item and UI action.
- Findings: Version-sensitive and server-authoritative in multiplayer; modern components may differ.
- RustChunk relevance: Client interaction reference.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Reimplement 1.20.1 semantics and respect server inventory sync.
- Follow-up checks: Verify block entity NBT pick data, creative rules, hotbar and packet order.
- RustChunk implementation mapping: RustChunk client pick-block action.
- Evidence required before adoption: 1.20.1 source, packet traces and creative/survival tests.
- Completion state: COMPLETE — 132 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5027 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1637. `crates/horizon-client/src/platform.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `10ee05433109d84426351cac89ed6c26ef539f01`
- Size: 4338 bytes; 104 logical lines
- Coverage: 1–104 (EOF)
- Purpose: Platform abstraction for OS-specific client services.
- Dependencies / cross-references: OS APIs, filesystem, process/window/runtime and cfg compilation.
- Implementation / logic: Provides narrow platform boundary for OS-specific operations.
- Findings: Desktop assumptions may not map to Android lifecycle, scoped storage or Samsung services.
- RustChunk relevance: Useful abstraction boundary.
- Decision: rewrite required; platform-specific
- 1.20.1 implications: Rewrite Android/JNI/NDK adapters; keep platform-neutral core.
- Follow-up checks: Check cfg targets, permissions, app lifecycle, paths and unsupported syscalls.
- RustChunk implementation mapping: RustChunk platform trait with Android/desktop implementations.
- Evidence required before adoption: Android NDK/API 33 tests and Windows 7 host-toolchain constraints.
- Completion state: COMPLETE — 104 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 4338 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1638. `crates/horizon-client/src/portal_sections.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `92982f09ee4da9a2f6d9fe78a013a092bdd2af33`
- Size: 5114 bytes; 132 logical lines
- Coverage: 1–132 (EOF)
- Purpose: Portal-related chunk/section lookup and bookkeeping.
- Dependencies / cross-references: Chunk coordinates, section state and portal block search/placement.
- Implementation / logic: Provides bounded section-level operations for portal processing.
- Findings: Lookup rules must match 1.20.1 world-height and coordinate behavior.
- RustChunk relevance: Potential portal search helper.
- Decision: adaptable; version-specific
- 1.20.1 implications: Adopt only after checking exact 1.20.1 portal search bounds and block rules.
- Follow-up checks: Verify negative coordinates, section boundaries and search order.
- RustChunk implementation mapping: RustChunk portal section index/helper.
- Evidence required before adoption: 1.20.1 portal-forcer source and coordinate fixtures.
- Completion state: COMPLETE — 132 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5114 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1639. `crates/horizon-client/src/pose.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f0764a22e28276ba096562e8faf47929459bcd74`
- Size: 8485 bytes; 213 logical lines
- Coverage: 1–213 (EOF)
- Purpose: Entity pose/dimension state for rendering and interaction bounds.
- Dependencies / cross-references: Entity type, pose flags, dimensions, crouch/sleep/swim/fall-flying and collision.
- Implementation / logic: Maps pose states to dimensions/transforms and client state.
- Findings: Modern pose list may include post-1.20.1 entries; bounds affect hit testing/collision.
- RustChunk relevance: Core rendering/physics reference.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild 1.20.1 pose registry and dimensions; exclude later entries.
- Follow-up checks: Compare pose dimensions, eye heights and transition ticks.
- RustChunk implementation mapping: RustChunk versioned pose/dimension table shared by renderer/physics.
- Evidence required before adoption: 1.20.1 EntityDimensions/Pose source and hitbox tests.
- Completion state: COMPLETE — 213 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 8485 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1640. `crates/horizon-client/src/pregen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `01215928a0fc40ab39f140fa654086e2aab5a2d0`
- Size: 7106 bytes; 191 logical lines
- Coverage: 1–191 (EOF)
- Purpose: Chunk/world pre-generation scheduling and progress state.
- Dependencies / cross-references: World generator, chunk queue, cancellation, progress UI and disk persistence.
- Implementation / logic: Schedules pre-generation work and reports completion/progress.
- Findings: Can saturate CPU, memory and disk I/O and worsen interactive frame pacing.
- RustChunk relevance: Optional offline utility only.
- Decision: reference; optional offline utility
- 1.20.1 implications: Throttle/pause during active play; enforce Android budgets.
- Follow-up checks: Verify cancellation, worker cap, disk backpressure and autosave interaction.
- RustChunk implementation mapping: RustChunk offline pregen service with mobile budget.
- Evidence required before adoption: Android thermal/memory tests and queue pause/resume tests.
- Completion state: COMPLETE — 191 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7106 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1641. `crates/horizon-client/src/preparer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5fe6672a88ffbedd4a804929d430cbf5fda3dda7`
- Size: 13270 bytes; 350 logical lines
- Coverage: 1–350 (EOF)
- Purpose: Launcher preparation pipeline for game files.
- Dependencies / cross-references: Manifest, downloads/extraction, assets/libraries, checksums and filesystem layout.
- Implementation / logic: Coordinates validation and preparation of required resources.
- Findings: Horizon packaging/manifests are not automatically valid for Forge 1.20.1.
- RustChunk relevance: Relevant launcher architecture.
- Decision: rewrite required; launcher integration
- 1.20.1 implications: Use RustChunk's own signed manifest and 1.20.1 asset/library layout.
- Follow-up checks: Verify signatures/checksums, recovery, atomic install, offline cache and paths.
- RustChunk implementation mapping: RustChunk installer/preflight stage.
- Evidence required before adoption: Reproducible manifest and interrupted/offline install tests.
- Completion state: COMPLETE — 350 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 13270 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1642. `crates/horizon-client/src/preparer_install.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `703505b34289e8346ec286d4078b3800f3205f89`
- Size: 5210 bytes; 136 logical lines
- Coverage: 1–136 (EOF)
- Purpose: Install/preparation progress UI and state transitions.
- Dependencies / cross-references: Preparer job state, UI events, cancellation and error reporting.
- Implementation / logic: Displays preparation progress and routes user actions.
- Findings: Must not claim completion before file verification.
- RustChunk relevance: Launcher UX reference.
- Decision: reference; install UI
- 1.20.1 implications: Adapt with truthful progress and cancellable jobs.
- Follow-up checks: Test cancellation, retry, errors, missing network and slow storage.
- RustChunk implementation mapping: RustChunk install progress screen/state adapter.
- Evidence required before adoption: UI state tests and Android interrupted-download/retry tests.
- Completion state: COMPLETE — 136 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 5210 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1643. `crates/horizon-client/src/projectiles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dd1bb9ec3cfde039da6dd4fe52378be6f79f37b3`
- Size: 56872 bytes; 1497 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1497 (EOF)
- Purpose: Projectile simulation, collision, motion and entity-specific behavior.
- Dependencies / cross-references: Entity physics, ray/AABB collision, block shapes, damage/effects, RNG and network authority.
- Implementation / logic: Implements projectile types, movement, gravity/drag, hit detection and impact effects.
- Findings: Contains later projectile kinds and modern item/damage APIs; filter to 1.20.1.
- RustChunk relevance: Core combat/physics reference.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Rebuild target projectile set and server-authoritative collision/damage.
- Follow-up checks: Verify arrow/trident/snowball/egg/pearl/potion/fireball timing, drag, gravity and RNG.
- RustChunk implementation mapping: RustChunk 1.20.1 projectile trait and packet adapters.
- Evidence required before adoption: 1.20.1 sources, deterministic trajectories and collision fixtures.
- Completion state: COMPLETE — 1497 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 56872 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1644. `crates/horizon-client/src/raid.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4b1185970904b5e3152e9c42f06673f2e08fbe12`
- Size: 57270 bytes; 1705 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1705 (EOF)
- Purpose: Raid lifecycle, wave composition, omen trigger, victory/loss and rewards.
- Dependencies / cross-references: POI/village centers, difficulty, raiders, spawn placement, boss bars, world time and server events.
- Implementation / logic: Tracks raid state/waves, spawn positions/members, progress and resolution.
- Findings: World/server-authoritative; newer Ominous Trial/Bottle data must be filtered.
- RustChunk relevance: Local-world reference; remote client consumes server state.
- Decision: rewrite required; version-specific; server-authoritative
- 1.20.1 implications: Rebuild 1.20.1 RaidManager; do not run competing client simulation.
- Follow-up checks: Verify omen transition, wave counts, spawn rings, hero rewards, timeout and packets.
- RustChunk implementation mapping: RustChunk local raid manager and client raid renderer.
- Evidence required before adoption: 1.20.1 RaidManager source and deterministic wave traces.
- Completion state: COMPLETE — 1705 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 57270 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1645. `crates/horizon-client/src/ram_limit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `57c1f095e2f246652a6b9b4dba7b8bf6cb40cd34`
- Size: 12330 bytes; 355 logical lines
- Coverage: 1–355 (EOF)
- Purpose: Memory-pressure governor reducing expensive far-terrain/render work.
- Dependencies / cross-references: Memory pressure signal, far-terrain state, render distance, scheduler and platform memory reporting.
- Implementation / logic: Detects pressure and applies staged reductions including far-terrain disable and render-distance decrease.
- Findings: Relevant to mobile stability; thresholds/cadence/recovery require device validation.
- RustChunk relevance: High-value mobile performance reference.
- Decision: reference; mobile memory governor
- 1.20.1 implications: Adapt conservatively, keep hysteresis and preserve gameplay chunk priority.
- Follow-up checks: Verify signal, cadence, thresholds, recovery and no oscillation on 6 GB device.
- RustChunk implementation mapping: RustChunk mobile memory governor for LOD/render budget.
- Evidence required before adoption: Galaxy A22 memory traces, thermal runs and allocation profiling.
- Completion state: COMPLETE — 355 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 12330 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1646. `crates/horizon-client/src/random_tick_sections.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9cab4987e743811860aebe950977a6803590b77e`
- Size: 28126 bytes; 702 logical lines
- Coverage: 1–400, 401–702 (EOF)
- Purpose: Section-level cache/index for random-tick candidate selection.
- Dependencies / cross-references: Loaded sections, block random-tick flags, world scheduler and cache lifetime.
- Implementation / logic: Builds/reuses section candidate sets with weak/shared ownership to avoid repeated scans.
- Findings: Must preserve exact random-tick distribution/order and not retain unloaded chunks.
- RustChunk relevance: Potential local simulation CPU/allocation optimization.
- Decision: adaptable; performance reference; version-specific
- 1.20.1 implications: Adapt only with deterministic 1.20.1 random-tick semantics and bounded cache.
- Follow-up checks: Verify invalidation, unload cleanup, distribution and tick order.
- RustChunk implementation mapping: RustChunk section random-tick candidate cache.
- Evidence required before adoption: 1.20.1 random-tick source, RNG differential tests and CPU/allocation benchmarks.
- Completion state: COMPLETE — 702 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 28126 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1647. `crates/horizon-client/src/recipe_book.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `19c777a1070cd681b889f79b615f1f8300c8cb66`
- Size: 55271 bytes; 1364 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1364 (EOF)
- Purpose: Recipe-book state, filtering, grouping and recipe unlock model.
- Dependencies / cross-references: Recipe registry, item/tag ingredients, player settings, recipe packets and UI.
- Implementation / logic: Models recipe entries, search/filter/category state and book settings/selection.
- Findings: Current data/component structures may differ from 1.20.1; unlock state is server-driven.
- RustChunk relevance: Client UI/state reference.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild with 1.20.1 identifiers, ingredient/tag and packet formats.
- Follow-up checks: Verify categories, unlock packets, search/filter and server sync.
- RustChunk implementation mapping: RustChunk 1.20.1 recipe-book model.
- Evidence required before adoption: 1.20.1 RecipeBook source, packet captures and UI state tests.
- Completion state: COMPLETE — 1364 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 55271 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1648. `crates/horizon-client/src/recipe_book_screen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c4974721cf55503a562bc3fc46251384a2833d10`
- Size: 22576 bytes; 734 logical lines
- Coverage: 1–400, 401–734 (EOF)
- Purpose: Recipe-book screen, search, overlay recipes, ghost slots and tooltips.
- Dependencies / cross-references: Recipe-book model, UI calls, item rendering, localization, input and layout.
- Implementation / logic: Builds draw-state calls for tabs/buttons/search/overlay/ghost slots.
- Findings: Geometry/assets and modern overlay data are version-specific.
- RustChunk relevance: Optional client UI reference.
- Decision: reference; optional client UI
- 1.20.1 implications: Rebuild target 1.20.1 geometry/assets and touch-friendly controls.
- Follow-up checks: Verify GUI scale, narrow screens, search cursor, ghost slots and item cycling.
- RustChunk implementation mapping: RustChunk optional recipe-book UI over 1.20.1 state.
- Evidence required before adoption: 1.20.1 UI source/assets, screenshot goldens and Android touch tests.
- Completion state: COMPLETE — 734 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 22576 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1649. `crates/horizon-client/src/save.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6540a2c8b465cc22387aa093ce33d1f6d9c9464f`
- Size: 322390 bytes; 7946 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4800, 4801–5200, 5201–5600, 5601–6000, 6001–6400, 6401–6800, 6801–7200, 7201–7600, 7601–7946 (EOF)
- Purpose: World/chunk/entity/POI persistence, region I/O, player/world save data, async writing and saved-chunk LOD.
- Dependencies / cross-references: NBT, palettes, MCA regions, DataFixers, block entities, POI, generator, entities, filesystem and worker threads.
- Implementation / logic: Encodes/decodes chunks/player/world; LRU region handles; async writer; pending snapshots; saved LOD surfaces and bounded decoded cache.
- Findings: Explicitly 26.3 layout with later entities/rules (Shelf, Creaking Heart, Trial Spawner, Vault, Potent Sulfur); schema is incompatible with 1.20.1.
- RustChunk relevance: Critical persistence/I/O/LOD architecture reference.
- Decision: rewrite required; core persistence/performance reference; version-specific
- 1.20.1 implications: Rebuild all NBT/region/player/world schemas for 1.20.1; study bounded caches/async writer/saved-LOD separately.
- Follow-up checks: Verify 1.20.1 chunk NBT/light/heightmaps/POI/entities, DataFixers, durability, LRU cap and Android I/O stalls.
- RustChunk implementation mapping: RustChunk 1.20.1 persistence with bounded region cache and serialized writer.
- Evidence required before adoption: 1.20.1 save sources/fixtures, differential round-trips, crash tests and Android I/O profiling.
- Completion state: COMPLETE — 7946 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 322390 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1650. `crates/horizon-client/src/save/gateway.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ba8bacdc7de562b8295fd400ef113f1685ce0018`
- Size: 9663 bytes; 234 logical lines
- Coverage: 1–234 (EOF)
- Purpose: One-time migration of legacy Horizon End Gateway exits into chunk block-entity NBT.
- Dependencies / cross-references: SaveDir region I/O, palette decode, End Gateway NBT and durability barrier.
- Implementation / logic: Reads legacy JSON, validates actual gateway blocks, merges exits and removes legacy file only after durable write.
- Findings: Horizon-specific migration; modern NBT keys/layout require target mapping.
- RustChunk relevance: Optional migration transaction pattern.
- Decision: reference; optional legacy migration
- 1.20.1 implications: Reuse only validation/durability principle; implement dedicated 1.20.1 migration if old RustChunk saves exist.
- Follow-up checks: Verify malformed JSON retention, write failure, stale block entities and idempotency.
- RustChunk implementation mapping: RustChunk optional versioned world migration helper.
- Evidence required before adoption: 1.20.1 gateway NBT fixtures, interrupted-write and idempotency tests.
- Completion state: COMPLETE — 234 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 9663 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1651. `crates/horizon-client/src/scoreboard_cmd.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f1fcff285526fe20590dd0d0c41605391426b939`
- Size: 42206 bytes; 1143 logical lines
- Coverage: 1–400, 401–800, 801–1143 (EOF)
- Purpose: Scoreboard command parsing, argument completion and command-side objective, player-score, team and display-slot operations.
- Dependencies / cross-references: Depends on scoreboard state, command dispatcher/brigadier argument types, player/team registries, text components, permissions and server command execution.
- Implementation / logic: Defines command tree/argument parsing and routes scoreboard operations to the scoreboard model; test coverage is separated in the paired test file.
- Findings: The snapshot is newer than the 1.20.1 command tree and data model; exact command syntax, argument suggestions, objective criteria and text formatting must be matched to 1.20.1.
- RustChunk relevance: Useful reference for client command syntax and scoreboard interaction, but not a substitute for server-authoritative command implementation.
- Decision: rewrite required; version-specific; optional client command layer
- 1.20.1 implications: Rebuild the 1.20.1 scoreboard command tree and command execution on the server; do not port later argument or component APIs verbatim.
- Follow-up checks: Compare 1.20.1 scoreboard commands, criteria registry, team options, score holders, display slots, permission levels and error text.
- RustChunk implementation mapping: RustChunk 1.20.1 command registration plus server-side scoreboard service and packet synchronization.
- Evidence required before adoption: 1.20.1 Brigadier/source mappings, command parse fixtures, scoreboard state tests and multiplayer authority tests.
- Completion state: COMPLETE — 1143 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 42206 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1652. `crates/horizon-client/src/scoreboard_cmd_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `293670dd5021ee0fb54a3990e1221c4bd0cd407b`
- Size: 12151 bytes; 358 logical lines
- Coverage: 1–358 (EOF)
- Purpose: Unit tests for scoreboard command parsing, argument validation and scoreboard command behavior.
- Dependencies / cross-references: Depends on scoreboard_cmd.rs, command dispatcher fixtures, scoreboard state and text/error handling.
- Implementation / logic: Exercises command parsing and representative command outcomes, including invalid and edge-case arguments.
- Findings: Test fixtures are tied to the newer command API represented by the paired source; expected strings and syntax need 1.20.1 translation.
- RustChunk relevance: Useful as a test-case inventory, not a runtime module.
- Decision: reference; tests only
- 1.20.1 implications: Do not port as executable code; translate only applicable cases into RustChunk 1.20.1 command tests.
- Follow-up checks: Reconcile every command and expected parse/error result with the 1.20.1 command tree.
- RustChunk implementation mapping: RustChunk scoreboard command integration tests.
- Evidence required before adoption: 1.20.1 Brigadier parsing fixtures and differential command result/error tests.
- Completion state: COMPLETE — 358 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 12151 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1653. `crates/horizon-client/src/screen_layout.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c627e6b4a5688500dae7770f5e5fee35fb5f7c7b`
- Size: 7875 bytes; 265 logical lines
- Coverage: 1–265 (EOF)
- Purpose: Reusable screen geometry and layout calculations for common UI regions, rows, columns and control placement.
- Dependencies / cross-references: Depends on viewport dimensions, GUI scaling, text metrics and screen widget geometry.
- Implementation / logic: Provides pure coordinate/rectangle helpers used by screen rendering and hit-testing.
- Findings: Layout values are not gameplay behavior; exact dimensions depend on target-version widgets, font metrics and mobile viewport constraints.
- RustChunk relevance: Useful optional UI reference, especially for deterministic layout and hit-test consistency.
- Decision: reference; optional client UI
- 1.20.1 implications: Recreate only the geometry needed by RustChunk screens; validate against 1.20.1 widget layouts and Android aspect ratios.
- Follow-up checks: Check rounding, odd/even viewport sizes, GUI scale, clipping and pointer/touch hitboxes.
- RustChunk implementation mapping: RustChunk shared UI layout helpers, isolated from simulation and networking.
- Evidence required before adoption: 1.20.1 screen geometry references and screenshot/coordinate golden tests across Android resolutions.
- Completion state: COMPLETE — 265 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7875 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1654. `crates/horizon-client/src/screen_narrator.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `039a58f80ca42d5edee06d6d52387ac38a9ed5e9`
- Size: 56807 bytes; 1530 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1530 (EOF)
- Purpose: Screen narration/accessibility text collection, ordering, focus and announcement behavior for menus, lists, buttons and edit fields.
- Dependencies / cross-references: Depends on widget focus, translated text components, screen identity, accessibility options and input navigation.
- Implementation / logic: Collects narratable widgets, composes screen and control descriptions, orders announcements and exposes current/urgent narration state.
- Findings: The source includes newer screen IDs/widgets and localization keys; narration behavior is client-only and not required for Forge protocol compatibility.
- RustChunk relevance: Useful accessibility design reference; can be implemented independently of gameplay and renderer.
- Decision: reference; optional accessibility UI
- 1.20.1 implications: Port target-era narration semantics selectively and keep narration disabled/low-cost when unused; do not import modern widget IDs blindly.
- Follow-up checks: Verify 1.20.1 narration keys, focus order, list row announcements, screen transitions and text component flattening.
- RustChunk implementation mapping: RustChunk optional accessibility/narrator module and shared widget narration metadata.
- Evidence required before adoption: 1.20.1 language resources, widget focus traces and screen-reader behavior tests.
- Completion state: COMPLETE — 1530 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 56807 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1655. `crates/horizon-client/src/server_packs/download.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0ae237c02d8a37d215796542f9602a631ab3e7e9`
- Size: 21257 bytes; 596 logical lines
- Coverage: 1–400, 401–596 (EOF)
- Purpose: Server resource-pack download queue, HTTP headers, SHA-1 verification, cache reuse, progress events, JSON download log and cache cleanup.
- Dependencies / cross-references: Depends on server-pack request packets, URL/HTTP client, SHA-1, filesystem, progress UI, pack hash and resource reload pipeline.
- Implementation / logic: Serializes batches on a worker thread, downloads with connect/read timeouts and size caps, verifies requested hashes, logs outcomes and retains a bounded cache.
- Findings: Header advertises pack format 97.1 and modern version constants; 250 MiB cap, path layout, timeout semantics and cache policy need comparison to 1.20.1. Failed hash downloads may leave partial files until cleanup; cache/log concurrency and crash recovery require review.
- RustChunk relevance: Critical optional client feature for joining servers with resource packs; Android network and storage stalls matter.
- Decision: rewrite required; version-specific; security-sensitive
- 1.20.1 implications: Implement 1.20.1 packet/header/resource-pack semantics with strict URL, size, hash and path validation; preserve bounded worker I/O and cleanup without blocking render thread.
- Follow-up checks: Verify 1.20.1 X-Minecraft headers, accepted pack formats, required/optional prompts, SHA-1 behavior, redirect policy, timeout, partial-file cleanup and cache ordering.
- RustChunk implementation mapping: RustChunk 1.20.1 server-pack download service with worker queue, hash verifier, bounded cache and reload callback.
- Evidence required before adoption: 1.20.1 Java HttpUtil/DownloadQueue source, malicious URL/oversize/hash fixtures, interrupted-download tests and Android I/O profiling.
- Completion state: COMPLETE — 596 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 21257 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1656. `crates/horizon-client/src/shadows.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c27ed38da14a272b87eb1437fd876680eb4e7bd7`
- Size: 18017 bytes; 445 logical lines
- Coverage: 1–445 (EOF)
- Purpose: Entity and display shadow radius/strength rules, camera-distance fading, block-light sampling and collision-shape-clipped shadow quads.
- Dependencies / cross-references: Depends on entity species/render state, block collision AABBs, world light, camera position, sprite atlas and vertex pipeline.
- Implementation / logic: Calculates species-specific radii, interpolates display shadow values, gates by distance/radius, samples block tops and emits clipped textured quads.
- Findings: Contains later species (Breeze, Creaking, Copper Golem, Nautilus, Happy Ghast, Camel Husk, Sulfur Cube) and display behavior that must be filtered or matched to 1.20.1.
- RustChunk relevance: Useful client rendering reference; shadow mesh generation may be CPU-sensitive on mobile.
- Decision: adaptable; version-specific; rendering
- 1.20.1 implications: Keep only target-era entity rules and compare vanilla 1.20.1 shadow projection, light and collision behavior; profile vertex generation before optimizing.
- Follow-up checks: Verify 1.20.1 radius table, invisibility behavior, camera fade, block-shape clipping, face winding, alpha and display entity availability.
- RustChunk implementation mapping: RustChunk optional shadow renderer with versioned species table and reusable vertex buffer.
- Evidence required before adoption: 1.20.1 renderer source/mappings, block-shape/light fixtures, render goldens and Android GPU/CPU profiling.
- Completion state: COMPLETE — 445 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 18017 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1657. `crates/horizon-client/src/social_screen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0d0a4fac6f652d112b72030787c8750c6ef5e6e3`
- Size: 16492 bytes; 507 logical lines
- Coverage: 1–507 (EOF)
- Purpose: Social Interactions screen with All/Hidden/Blocked tabs, search, player skins, hide/show chat actions, offline status and disabled report affordance.
- Dependencies / cross-references: Depends on player list, UUID/name/skin, chat visibility preferences, translation keys, widget focus and social/reporting service.
- Implementation / logic: Sorts and filters player entries, computes list/scroll geometry, draws player rows and mute/unmute buttons, and emits hide/show/done actions.
- Findings: Blocked/reporting is explicitly unavailable in this implementation; online moderation/report service is external. Exact player list and mute persistence semantics need target-version parity.
- RustChunk relevance: Useful 1.20.1 client UI behavior and per-player chat visibility feature.
- Decision: adaptable; version-specific; optional client UI
- 1.20.1 implications: Adapt list and hide/show behavior using 1.20.1 player/social APIs; do not imply a blocked/report function that is not implemented.
- Follow-up checks: Verify 1.20.1 sorting order, local-player row, hidden/offline labels, mute persistence, search and button geometry.
- RustChunk implementation mapping: RustChunk optional social interactions screen backed by local UUID chat-visibility settings.
- Evidence required before adoption: 1.20.1 SocialInteractionsScreen source, translation resources, list fixtures and chat-filter integration tests.
- Completion state: COMPLETE — 507 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 16492 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1658. `crates/horizon-client/src/spear.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `67f04818cb6370c1d43ef4e8ebe48a0f57f9896e`
- Size: 17431 bytes; 500 logical lines
- Coverage: 1–400, 401–500 (EOF)
- Purpose: Kinetic spear charge/dismount/knockback/damage conditions, attack reach and first-/third-person stab animation transforms.
- Dependencies / cross-references: Depends on spear item registry, kinetic weapon attributes, attack collision, movement vectors, swing animation and rendering matrices.
- Implementation / logic: Defines material-specific spear parameters, speed/time conditions, reach collision and eased hand/item animation matrices.
- Findings: Wooden/stone/copper/iron/golden/diamond/netherite spear items and kinetic weapon behavior are not in the 1.20.1 baseline.
- RustChunk relevance: No direct gameplay adoption; generic animation/easing math is only a separate rendering reference.
- Decision: exclude; post-1.20.1
- 1.20.1 implications: Exclude spear items, attack types, sounds, packet and animation state from 1.20.1.
- Follow-up checks: Confirm target item/entity/packet registries contain no spear/kinetic-weapon identifiers.
- RustChunk implementation mapping: No 1.20.1 gameplay mapping; optional future-version research only.
- Evidence required before adoption: 1.20.1 item, sound, attack and protocol registries confirming absence.
- Completion state: COMPLETE — 500 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 17431 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1659. `crates/horizon-client/src/spectator_menu.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f15f721637993313258c3267ede9e27a9379b91a`
- Size: 13664 bytes; 419 logical lines
- Coverage: 1–419 (EOF)
- Purpose: Spectator hotbar/menu pages for teleport-to-player and teleport-to-team, scrolling, selection, close action, fade timing and icons.
- Dependencies / cross-references: Depends on spectator game mode, player list/game modes, teams, UUID ordering, teleport packet and HUD sprite/font resources.
- Implementation / logic: Builds root/player/team categories, filters non-spectators, sorts by UUID, requires selection then activation, emits teleport outcomes and draws the fading hotbar.
- Findings: Core spectator menu exists in 1.20.1; team membership, UUID ordering, item/icon assets and packet mapping need exact target verification.
- RustChunk relevance: Useful complete spectator UI behavior, optional to RustChunk if spectator mode is supported.
- Decision: adaptable; version-specific; client UI
- 1.20.1 implications: Adapt against 1.20.1 SpectatorMenu and ServerboundTeleportToEntityPacket; preserve two-step selection and server authority.
- Follow-up checks: Verify 1.20.1 menu slot/page rules, spectator filtering, signed UUID ordering, team list order, fade clock and teleport packet.
- RustChunk implementation mapping: RustChunk 1.20.1 spectator menu/HUD module with serverbound teleport action.
- Evidence required before adoption: 1.20.1 source/mappings, player/team fixtures, packet traces and UI geometry goldens.
- Completion state: COMPLETE — 419 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 13664 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1660. `crates/horizon-client/src/sprint.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b63aefee824a9a685410d95392687b3cc5dd0005`
- Size: 8582 bytes; 261 logical lines
- Coverage: 1–261 (EOF)
- Purpose: Local sprint start/stop conditions, double-tap forward window, sprint-key handling, swimming/flying restrictions and analog-stick direction mapping.
- Dependencies / cross-references: Depends on keyboard/controller input, player food/mobility/passenger state, vehicle sprint rules, water/fall-flying state and movement packet generation.
- Implementation / logic: Converts analog stick to nearest eight key directions, tracks previous forward/shift and trigger window, evaluates can-start/should-stop and invokes sprint state setter.
- Findings: Vanilla sprint conditions are target-era, but analog stick integration and some state fields are Starling-specific; exact 1.20.1 tick order and window must be checked.
- RustChunk relevance: Highly relevant mobile control behavior; correctness affects movement feel and server prediction.
- Decision: adaptable; version-specific; mobile input integration
- 1.20.1 implications: Port target-era sprint logic, keep analog stick as a separate input adapter and preserve vanilla key semantics/state transition order.
- Follow-up checks: Verify 1.20.1 seven-tick double-tap, hunger/passenger rules, water/fall-flying conditions, collision stop and analog diagonal normalization.
- RustChunk implementation mapping: RustChunk 1.20.1 sprint state machine plus Android gamepad/virtual-stick adapter.
- Evidence required before adoption: 1.20.1 LocalPlayer source, deterministic input traces and movement packet parity tests.
- Completion state: COMPLETE — 261 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 8582 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1661. `crates/horizon-client/src/starling_releases.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `973ed14de346c83e1aa438253a8ed1afc6b34639`
- Size: 14863 bytes; 386 logical lines
- Coverage: 1–386 (EOF)
- Purpose: Starling APK release discovery, ABI selection, update asset/digest validation, native-mod release parsing and build-server address validation.
- Dependencies / cross-references: Depends on GitHub Releases API, Android process ABI, Starling release naming, native-mod metadata and launcher account/update UI.
- Implementation / logic: Selects newer APK by process ABI, merges native builds across repositories, validates HTTPS build server host and parses asset names/digests.
- Findings: Entire module is project/release infrastructure, not Minecraft 1.20.1 client or server behavior; release repositories and asset naming are Starling-specific.
- RustChunk relevance: Not a game engine port target; ideas around strict URL/ABI validation may inform launcher code separately.
- Decision: exclude; Starling-specific launcher/release integration
- 1.20.1 implications: Exclude from Minecraft simulation/protocol port; reimplement only if RustChunk needs its own update/native-mod service.
- Follow-up checks: Verify RustChunk distribution/update architecture independently; never inherit Starling repo URLs or release identity.
- RustChunk implementation mapping: No Minecraft gameplay mapping; optional RustChunk launcher update module if explicitly required.
- Evidence required before adoption: RustChunk release policy, trusted repository identity, ABI packaging and update signature/digest tests.
- Completion state: COMPLETE — 386 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 14863 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1662. `crates/horizon-client/src/stations.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e8749c52e1fc5f1064930ad8a105463161479b7e`
- Size: 37655 bytes; 1054 logical lines
- Coverage: 1–400, 401–800, 801–1054 (EOF)
- Purpose: Enchanting table offers, anvil repair/enchant/rename costs, grindstone results/XP, smithing transform/trim recipes and stonecutter recipe selection.
- Dependencies / cross-references: Depends on ItemStack components/NBT, enchantment registry/tags, legacy RNG, recipe codecs, datapack overlays, loot, game rules and menu slot state.
- Implementation / logic: Loads station recipes and related data, computes deterministic enchant offers/clues, anvil costs/results, grindstone outputs/XP and smithing/stonecutting results.
- Findings: The code explicitly assumes modern 26.3 item-component/enchantment semantics (including comments that all items carry enchantments); 1.20.1 uses older ItemStack NBT, recipe and trim data.
- RustChunk relevance: Core workstation logic is essential for 1.20.1 gameplay, but this source is not a direct port.
- Decision: rewrite required; core gameplay reference; version-specific
- 1.20.1 implications: Rewrite around 1.20.1 ItemStack NBT, enchantment storage, recipe JSON/registry and station menu rules; retain legacy RNG draw order.
- Follow-up checks: Compare bookshelf offsets, enchantment RNG/clues, anvil penalties/cost cap, repair arithmetic, grindstone curse retention/XP, smithing trim and stonecutter recipe order.
- RustChunk implementation mapping: RustChunk 1.20.1 station services plus versioned recipe/enchantment codecs and server-authoritative menu results.
- Evidence required before adoption: 1.20.1 Java source/mappings, recipe/tag exports, item NBT fixtures, seeded RNG differential tests and station interaction tests.
- Completion state: COMPLETE — 1054 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 37655 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1663. `crates/horizon-client/src/stats_overlay.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a7f7b425d9ddd798c60b9598d34ab8714cce7d69`
- Size: 9010 bytes; 271 logical lines
- Coverage: 1–271 (EOF)
- Purpose: Performance overlay showing FPS, frame/CPU/GPU time, 1% low, RAM/GPU memory, chunk/meshing queues, draw calls, temperature, thermal status, render distance and backend.
- Dependencies / cross-references: Depends on frame profiler, system/thermal telemetry, GPU allocator metrics, chunk streaming/meshing counters and UI theme.
- Implementation / logic: Formats performance samples into colored lines and JSON `horizon-ctl perf` output, then draws a compact top-left panel.
- Findings: Starling/Horizon diagnostics, not vanilla Minecraft behavior; some telemetry unavailable on Android GPU backends and values can be absent.
- RustChunk relevance: Directly relevant to RustChunk's mobile FPS and memory diagnosis, but optional instrumentation.
- Decision: reference; optional mobile diagnostics
- 1.20.1 implications: Port as opt-in diagnostics with low-frequency sampling and no per-frame allocations; clearly label unavailable metrics and avoid expensive telemetry polling.
- Follow-up checks: Verify measurement definitions, 1% low window, CPU/GPU timing source, thermal API support, memory units and overlay overhead.
- RustChunk implementation mapping: RustChunk optional performance HUD and diagnostic JSON endpoint, gated behind settings.
- Evidence required before adoption: Android API availability, profiler accuracy, allocation/frame-time benchmark and thermal polling overhead tests.
- Completion state: COMPLETE — 271 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 9010 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1664. `crates/horizon-client/src/stick_memory.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `30baedda89786232756e753ccba869104cadeb55`
- Size: 2116 bytes; 63 logical lines
- Coverage: 1–63 (EOF)
- Purpose: Persists per-server controller stick mode in `starling-servers.txt`, normalizing server address keys and replacing prior mode choices.
- Dependencies / cross-references: Depends on server address, gamepad `ServerStick` mode parser and game-directory filesystem.
- Implementation / logic: Reads address=mode lines, looks up a normalized address and rewrites the file when a server's stick mode changes.
- Findings: Custom Starling input preference; not a vanilla 1.20.1 protocol or gameplay feature.
- RustChunk relevance: Useful small mobile UX pattern for server-specific control preferences.
- Decision: reference; Starling-specific mobile input preference
- 1.20.1 implications: Adapt only if RustChunk offers per-server stick modes; use atomic file writes and robust parsing rather than direct overwrite.
- Follow-up checks: Verify address normalization, duplicate entries, malformed lines, write failure handling and server identity/port semantics.
- RustChunk implementation mapping: RustChunk optional per-server controller preference store.
- Evidence required before adoption: Persistence tests for malformed/corrupt files, interrupted writes and distinct host/port identities.
- Completion state: COMPLETE — 63 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 2116 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1665. `crates/horizon-client/src/stopwatches.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b058e9d797df33e9e3a52d0190f03cf10766eed0`
- Size: 3638 bytes; 121 logical lines
- Coverage: 1–121 (EOF)
- Purpose: Named elapsed-time stopwatches persisted in `data/minecraft/stopwatches.dat` as accumulated milliseconds.
- Dependencies / cross-references: Depends on command layer, NBT read/write, save scheduling and monotonic process clock.
- Implementation / logic: Loads named timers from NBT, creates/restarts/removes them, reports elapsed seconds and saves accumulated elapsed time.
- Findings: A custom stopwatch feature rather than core vanilla 1.20.1 gameplay; monotonic clock resets across process restart and stored elapsed time is resumed from current process time.
- RustChunk relevance: Optional utility only; not required for Forge/mod compatibility.
- Decision: reference; optional command utility
- 1.20.1 implications: Exclude from core port unless RustChunk intentionally supports a stopwatch command; if adopted, preserve monotonic-time semantics and atomic save behavior.
- Follow-up checks: Check clock semantics, negative/corrupt NBT entries, dirty-save lifecycle and command integration.
- RustChunk implementation mapping: RustChunk optional stopwatch command/state service.
- Evidence required before adoption: NBT fixtures, process-restart behavior and save interruption tests.
- Completion state: COMPLETE — 121 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 3638 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1666. `crates/horizon-client/src/structure_blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f85edaf319b3498fd30944727f7beffc0b8882c0`
- Size: 20130 bytes; 544 logical lines
- Coverage: 1–400, 401–544 (EOF)
- Purpose: Generated structure block entities, container loot unpacking, loot table caching, item components and deferred exploration-map search data.
- Dependencies / cross-references: Depends on ProtoChunk block entities, loot tables/functions, block/entity registries, ItemStack NBT, pack overlays, biome and chunk persistence.
- Implementation / logic: Converts generated block-entity NBT, rolls chest loot, caches loot data by resource root/pack overlay, preserves loot seed and carries map search metadata through custom data.
- Findings: Loot item components, custom-data representation and newer loot fields differ from 1.20.1; position-derived fallback seed is a Horizon-specific divergence from vanilla's level RNG when no seed is stored.
- RustChunk relevance: Important for structure chest correctness, deterministic worldgen and saved block entities.
- Decision: rewrite required; core worldgen/persistence reference; version-specific
- 1.20.1 implications: Rebuild against 1.20.1 RandomizableContainer and loot-function timing, legacy ItemStack NBT and exploration-map function; do not roll loot at generation if vanilla defers it to first open.
- Follow-up checks: Verify 1.20.1 loot seed fallback, first-open unpack timing, item enchant/potion NBT, map decoration/search, overlay precedence and cache invalidation.
- RustChunk implementation mapping: RustChunk 1.20.1 generated block-entity conversion and deferred container loot service.
- Evidence required before adoption: 1.20.1 Java source/mappings, structure chest NBT/loot fixtures, seeded loot differential tests and world reload tests.
- Completion state: COMPLETE — 544 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 20130 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1667. `crates/horizon-client/src/text_display.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1b6c84b7d097f12f86fb4a85ac86c6ede230804b`
- Size: 24453 bytes; 653 logical lines
- Coverage: 1–400, 401–653 (EOF)
- Purpose: Display/Text Display synced fields, interpolation, billboard transforms, transformation matrix decomposition, text wrapping/alignment/background and culling bounds.
- Dependencies / cross-references: Depends on entity metadata serializers, NBT transformation codecs, text components, font metrics, camera matrices and render pipeline.
- Implementation / logic: Applies metadata IDs 8–27, interpolates transform/shadow values, decodes transformation vectors/quaternions/matrices, lays out and draws text quads.
- Findings: Display and Text Display entities exist in 1.20.1 (1.19.4+), but metadata fields/codec ordering, component formats and newer save semantics must be verified; file header explicitly cites 26.3.
- RustChunk relevance: Useful client entity rendering feature; separate from core chunk renderer and server simulation.
- Decision: adaptable; version-specific; rendering
- 1.20.1 implications: Keep only target-era display metadata and NBT codec; match 1.20.1 transformation decomposition, text component and billboard semantics.
- Follow-up checks: Verify 1.20.1 metadata IDs/types, billboard enum, interpolation defaults, text opacity/background flags, matrix decomposition and culling bounds.
- RustChunk implementation mapping: RustChunk 1.20.1 display entity state and optional text-display renderer.
- Evidence required before adoption: 1.20.1 mappings/source, entity metadata packet fixtures, transformation goldens and text render tests.
- Completion state: COMPLETE — 653 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 24453 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1668. `crates/horizon-client/src/tick_chunk_membership.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e238f64f7be2f7c9e2c9bc956dacd0ba0f209b65`
- Size: 12648 bytes; 342 logical lines
- Coverage: 1–342 (EOF)
- Purpose: Incremental cache of resident chunk membership for tick scheduler registration, including added/removed deltas and saved-tick waiting state.
- Dependencies / cross-references: Depends on ChunkMap revision/delta API, block/fluid scheduled tick queues and chunk load/unload lifecycle.
- Implementation / logic: Maintains a set keyed by chunk coordinates, replays deltas when possible, falls back to a full scan, recycles added buffers and starts waiting chunks once tickable.
- Findings: Algorithm is a Starling optimization over vanilla scheduler behavior; correctness depends on exact unload/reload/reset semantics and delta ordering.
- RustChunk relevance: High-value mobile CPU optimization candidate because it avoids repeated full resident-chunk scans.
- Decision: adaptable; core performance reference; version-specific
- 1.20.1 implications: Port only after matching 1.20.1 tick scheduler lifecycle; keep full-scan fallback and test reset/unload/re-add behavior before optimization.
- Follow-up checks: Verify chunk-map revision invalidation, add/remove cancellation, scheduler reset, saved ticks unpack exactly once and payload replacement behavior.
- RustChunk implementation mapping: RustChunk tick scheduler membership cache with reusable scratch buffers.
- Evidence required before adoption: 1.20.1 LevelTicks/ChunkMap source, deterministic load/unload/reset traces and large-world CPU/allocation benchmarks.
- Completion state: COMPLETE — 342 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 12648 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1669. `crates/horizon-client/src/ticket_levels.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `83e7694212e903f87f6d6f993225d712741b9ff1`
- Size: 22666 bytes; 580 logical lines
- Coverage: 1–400, 401–580 (EOF)
- Purpose: Chunk ticket level calculation for player view/simulation distance, forced chunks, portal tickets, entity/block/random ticking and loaded status.
- Dependencies / cross-references: Depends on DistanceManager ticket types, chunk status levels, player positions, view/simulation distances, portal lifecycle and server tick scheduler.
- Implementation / logic: Builds ticket influence maps using a bounded dense grid or sparse hash map, merges minimum loading/simulation levels and exposes ticking predicates/bounds.
- Findings: The ticket types/level arithmetic are target-relevant, but this is a custom compact map representation; verify all constants and status transitions against 1.20.1, especially portal timeout and forced ticket radius.
- RustChunk relevance: Core server simulation behavior and a strong optimization candidate for many entity/block tick checks.
- Decision: rewrite required; core simulation reference; version-specific
- 1.20.1 implications: Rebuild exact 1.20.1 DistanceManager/ChunkLevel rules first; then benchmark dense-grid versus sparse map while preserving per-chunk results.
- Follow-up checks: Verify player ticket levels, simulation-distance formula, forced/portal radius and expiry, entity/block/random tick predicates, negative/extreme coordinates and grid overflow.
- RustChunk implementation mapping: RustChunk 1.20.1 chunk ticket manager with versioned ticket semantics and dense/sparse index.
- Evidence required before adoption: 1.20.1 Java source/mappings, differential ticket grids, multi-player/portal/forced fixtures and Android/server CPU memory benchmarks.
- Completion state: COMPLETE — 580 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 22666 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1670. `crates/horizon-client/src/title.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4d0da6dfbb37852bc18eb10ecb43e64c524e1acc`
- Size: 206670 bytes; 5009 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4800, 4801–5009 (EOF)
- Purpose: Title, world select/create/edit, multiplayer entry, world backup/recovery/optimization, data-pack/experiment flows, onboarding, account and ban-notice screen state.
- Dependencies / cross-references: Depends on saves/level.dat, worldgen options, datapacks, account/auth, multiplayer screens, widget focus, translations, world upgrader, backups and platform actions.
- Implementation / logic: Defines screen modes/actions, world list sorting, backup/recovery/upgrade workflows, world creation state, input/focus navigation, rendering and screen narration integration.
- Findings: Massive newer Starling/Horizon UI with many project-specific account, ban, mod, data-pack and control-socket additions; several worldgen/version APIs differ from 1.20.1. Must be split into smaller RustChunk modules, not copied as one file.
- RustChunk relevance: Useful broad reference for world lifecycle UI, but not a direct launcher or gameplay implementation.
- Decision: rewrite required; client UI reference; version-specific
- 1.20.1 implications: Rebuild only required 1.20.1 world/title flows; isolate Starling account/ban/update integrations and mobile-specific UI; preserve safe backup/recovery behavior.
- Follow-up checks: Verify 1.20.1 TitleScreen/SelectWorld/CreateWorld flows, world backup prompts, file-fix/downgrade behavior, data-pack preparation, screen focus and action routing.
- RustChunk implementation mapping: RustChunk modular title/world-management UI with separate create-world, select-world, backup/recovery and optional account screens.
- Evidence required before adoption: 1.20.1 source/mappings, level.dat fixtures, screen action traces, Android layout goldens and save/backup failure tests.
- Completion state: COMPLETE — 5009 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 206670 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1671. `crates/horizon-client/src/title/account.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2a7fd1d5782049404dc79f1f2eec5d67938b1393`
- Size: 21628 bytes; 595 logical lines
- Coverage: 1–595 (EOF)
- Purpose: Account/profile screen for Microsoft, Ely.by and offline profiles, account status, device-code sign-in, credential fields, game-file download and profile management.
- Dependencies / cross-references: Depends on Starling account crate, Microsoft/Ely auth, local profile storage, clipboard, platform browser, game file distribution and account state.
- Implementation / logic: Implements profile list/add/remove, offline profile creation, Microsoft device code and Ely login forms, password masking, status/progress and game-file actions.
- Findings: This is Starling's own authentication/profile system, not vanilla 1.20.1 game behavior; account endpoints and offline profile model must not be treated as Mojang/Microsoft protocol compatibility.
- RustChunk relevance: No direct game engine adoption; account UI can be considered only if RustChunk intentionally needs the same account providers.
- Decision: exclude; Starling-specific account/auth UI
- 1.20.1 implications: Exclude from Minecraft client simulation and do not adopt credential/auth flows without a separate security design and provider review.
- Follow-up checks: Verify RustChunk's chosen account/session architecture independently; review secret handling, token storage, clipboard and password/OTP lifecycle if any auth UI is built.
- RustChunk implementation mapping: No 1.20.1 gameplay mapping; optional independent launcher account module only.
- Evidence required before adoption: Provider API documentation, secure token-storage design, credential redaction tests and offline-profile policy.
- Completion state: COMPLETE — 595 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 21628 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1672. `crates/horizon-client/src/title/buffet.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `72cdbfa42c2d2569e893ab24195f28d091dec5a4`
- Size: 11468 bytes; 325 logical lines
- Coverage: 1–325 (EOF)
- Purpose: Single-biome world creation screen with searchable biome list, sorted names, selected biome, scrolling and Done/Cancel actions.
- Dependencies / cross-references: Depends on prepared biome registry, localization, world preset selection, GUI scroll/hit testing and worldgen settings serialization.
- Implementation / logic: Loads biome IDs from prepared resources, resolves localized display names, filters/sorts rows, maintains selection and draws the list/search/footer.
- Findings: Single-biome surface preset is available in 1.20.1, but biome registry data source and UI layout must match target; resource pack/modded biomes need correct ordering.
- RustChunk relevance: Useful optional world-creation UI for modded biome selection.
- Decision: adaptable; version-specific; optional client UI
- 1.20.1 implications: Adapt against 1.20.1 biome registry and CreateBuffetWorldScreen behavior; keep the selection serialized into target worldgen settings.
- Follow-up checks: Verify 1.20.1 single-biome preset, biome registry IDs, localized sort order, search behavior and generated dimension settings.
- RustChunk implementation mapping: RustChunk optional single-biome worldgen preset screen backed by 1.20.1 biome registry.
- Evidence required before adoption: 1.20.1 biome registry/source, modded biome fixtures, worldgen NBT round-trips and UI geometry tests.
- Completion state: COMPLETE — 325 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 11468 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1673. `crates/horizon-client/src/title/confirm_experiments.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `58d194468b0ef97d76f62b3facd5a639dd3703ca`
- Size: 8622 bytes; 237 logical lines
- Coverage: 1–237 (EOF)
- Purpose: Confirmation screen for data packs requiring experimental features, with warning text, details page listing feature flags, Proceed and Back actions.
- Dependencies / cross-references: Depends on data-pack feature metadata, localization, font wrapping, screen layout and world-creation data-pack selection.
- Implementation / logic: Builds entries for selected feature packs, renders warning/details views and emits confirm/cancel outcomes without mutating the world directly.
- Findings: The flow is present in 1.20.1, but exact experimental feature flags, pack metadata and translation keys vary by target and mod loader.
- RustChunk relevance: Useful optional safety confirmation in modded world creation.
- Decision: adaptable; version-specific; optional client UI
- 1.20.1 implications: Port target-era experimental feature warning and pack selection semantics; keep user confirmation explicit and preserve selection on Back.
- Follow-up checks: Verify 1.20.1 feature pack metadata, enabled-feature registry, confirmation text, details ordering and cancel behavior.
- RustChunk implementation mapping: RustChunk world-create experimental-pack confirmation screen.
- Evidence required before adoption: 1.20.1 datapack/feature registry fixtures, selected-pack state tests and UI screenshot goldens.
- Completion state: COMPLETE — 237 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 8622 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1674. `crates/horizon-client/src/title/create_world.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d17ff74dcddc7317cc7d09cbf21d5cfe811afe36`
- Size: 30729 bytes; 821 logical lines
- Coverage: 1–400, 401–800, 801–821 (EOF)
- Purpose: Create-world options, game mode/difficulty/cheats, world preset, structures/bonus chest, game rules, data packs, experiments and tab/navigation layout.
- Dependencies / cross-references: Depends on Level/WorldGen NBT, game rules, datapack registry, feature flags, flat/buffet presets, localization, widgets and world creation pipeline.
- Implementation / logic: Defines WorldOptions serialization/defaults, reconstructs options from saved worlds, cycles tabs/options, computes field/button geometry and draws create-world controls.
- Findings: Modern feature-pack/mod-selection fields and newer worldgen/recipe interfaces exceed the 1.20.1 baseline; base game mode/preset controls are target-relevant.
- RustChunk relevance: Important world-creation reference, especially preserving worldgen options for modded worlds.
- Decision: adaptable; version-specific; client UI
- 1.20.1 implications: Adapt 1.20.1 CreateWorldScreen options and NBT; separate Starling-specific mod selection and newer feature-pack metadata.
- Follow-up checks: Verify 1.20.1 difficulty/hardcore/commands, world presets, flat settings, structures/bonus chest, game rules, datapack ordering and worldgen serialization.
- RustChunk implementation mapping: RustChunk 1.20.1 world creation options model plus modular UI and worldgen serializer.
- Evidence required before adoption: 1.20.1 CreateWorldScreen/WorldGenSettings source, world option NBT fixtures, modded datapack cases and UI/input tests.
- Completion state: COMPLETE — 821 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 30729 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1675. `crates/horizon-client/src/title/experiments.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9099d12263bbd45953a7c1f9e36358b43c4f1d30`
- Size: 7733 bytes; 218 logical lines
- Coverage: 1–218 (EOF)
- Purpose: Experiments screen listing shipped feature packs with enable/disable switches, descriptions, warning and Done/Cancel behavior.
- Dependencies / cross-references: Depends on feature-pack registry, localization, font wrapping, UI layout and world-creation data-pack configuration.
- Implementation / logic: Loads feature pack flags, lays out warning/switch rows, toggles flags and returns selected state on Done.
- Findings: The UI exists in the target era in related form, but exact pack list, enabled feature names and modern data-pack metadata must be sourced from 1.20.1 and Forge.
- RustChunk relevance: Useful optional client-side safety UI for experimental modded worldgen/data packs.
- Decision: adaptable; version-specific; optional client UI
- 1.20.1 implications: Adapt the concept to 1.20.1 datapack/experimental settings; do not hardcode newer shipped pack names or feature flags.
- Follow-up checks: Verify 1.20.1 experimental datapack registry, feature names/descriptions, default enabled states and Done/Cancel persistence.
- RustChunk implementation mapping: RustChunk optional 1.20.1 experiments/data-pack selection screen.
- Evidence required before adoption: 1.20.1 datapack metadata, Forge feature flags, localization fixtures and selection-state tests.
- Completion state: COMPLETE — 218 logical lines read through EOF in the listed contiguous coverage; pinned blob SHA and 7733 byte size match the source tree; embedded tests, where present, were inspected only and not run.

### 1676. `crates/horizon-client/src/title/flat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bf9789c1b345a70a27bd1a6aac18c93799f9da13`
- Size: 36334 bytes; 1081 logical lines
- Coverage: 1–400, 401–800, 801–1081 EOF
- Purpose: Superflat presets, layer editor, biome selection, structures and preview.
- Dependencies / cross-references: Block/biome registries, flat settings, worldgen serialization, localization and widgets.
- Implementation / logic: Edits layer rows, presets, biome and structure flags; converts selection to flat generator settings.
- Findings: Preset IDs, codecs and modded registry data need 1.20.1/Forge parity.
- RustChunk relevance: Useful superflat modpack world-creation reference.
- Decision: adaptable; version-specific UI
- 1.20.1 implications: Rebuild 1.20.1 FlatLevelSource settings; validate modded IDs.
- Follow-up checks: Layer order, height limits, presets, structures, NBT round trip.
- RustChunk implementation mapping: RustChunk 1.20.1 flat-world editor/serializer.
- Evidence required before adoption: 1.20.1 generator source and registry/world-save fixtures.
- Completion state: COMPLETE — 1081 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 36334 byte size match source tree; tests were inspected only and not run.

### 1677. `crates/horizon-client/src/title/onboarding.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cb1470182eea639e28f00c38b80c070d7d4a881c`
- Size: 12664 bytes; 362 logical lines
- Coverage: 1–362 EOF
- Purpose: First-run onboarding, account prompts, controls hints, launcher links and setup actions.
- Dependencies / cross-references: Starling identity, launcher preferences, URL opening, control settings and UI.
- Implementation / logic: Builds onboarding panels and action routing.
- Findings: Branding and service actions are not vanilla 1.20.1.
- RustChunk relevance: Optional UX reference only.
- Decision: exclude; Starling-specific UI
- 1.20.1 implications: Exclude from core; recreate only RustChunk-specific setup help.
- Follow-up checks: Review external links, permissions and preference persistence.
- RustChunk implementation mapping: Optional RustChunk first-run guide.
- Evidence required before adoption: RustChunk product choices and Android UX tests.
- Completion state: COMPLETE — 362 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 12664 byte size match source tree; tests were inspected only and not run.

### 1678. `crates/horizon-client/src/title/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04787ee4366e9f2d55d42b29c3833c4f06abf2df`
- Size: 87555 bytes; 2467 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2467 EOF
- Purpose: Title/world screen tests for text, world listing/creation/editing, backups, navigation and errors.
- Dependencies / cross-references: Title modules, fake saves, widgets, localization, worldgen and filesystem fixtures.
- Implementation / logic: Exercises screen state/actions and save workflows; no production logic.
- Findings: Fixtures follow newer APIs; translate only 1.20.1 cases.
- RustChunk relevance: Regression-test inventory.
- Decision: reference; tests only
- 1.20.1 implications: Do not port as runtime; translate target-relevant cases.
- Follow-up checks: Map tests to 1.20.1 behavior; retain save safety cases.
- RustChunk implementation mapping: RustChunk title/world UI tests.
- Evidence required before adoption: 1.20.1 screen behavior and deterministic filesystem fixtures.
- Completion state: COMPLETE — 2467 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 87555 byte size match source tree; tests were inspected only and not run.

### 1679. `crates/horizon-client/src/title/world_management.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a56fed462e4e83863104aeb80e530dd66d809bc6`
- Size: 6808 bytes; 203 logical lines
- Coverage: 1–203 EOF
- Purpose: World backup, restore, optimize, delete and open-folder actions.
- Dependencies / cross-references: Save metadata, level.dat, filesystem, upgrader and confirmation dialogs.
- Implementation / logic: Maps screen choices to save operations.
- Findings: Upgrader/platform actions differ; destructive actions need safety checks.
- RustChunk relevance: Important optional save lifecycle UI.
- Decision: adaptable; version-specific UI
- 1.20.1 implications: Implement safe 1.20.1 backup/restore/optimize/delete.
- Follow-up checks: Backup names, rollback, delete confirmation and failure recovery.
- RustChunk implementation mapping: RustChunk save-management service/screen.
- Evidence required before adoption: Filesystem failure tests and level.dat fixtures.
- Completion state: COMPLETE — 203 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 6808 byte size match source tree; tests were inspected only and not run.

### 1680. `crates/horizon-client/src/toasts.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4550d946141378f9118c3f9dd8a7a6c4c65a58e`
- Size: 13342 bytes; 357 logical lines
- Coverage: 1–357 EOF
- Purpose: Recipe, system, tutorial, advancement and social toast notifications.
- Dependencies / cross-references: Toast queue, text, icons, sound and HUD renderer.
- Implementation / logic: Defines variants, queue timing, visibility and drawing.
- Findings: Some variants/text APIs are newer.
- RustChunk relevance: Useful HUD feedback feature.
- Decision: adaptable; version-specific UI
- 1.20.1 implications: Port 1.20.1 variants/timing; separate modded toast API.
- Follow-up checks: Durations, priority, icons and scaling.
- RustChunk implementation mapping: RustChunk toast queue/HUD.
- Evidence required before adoption: 1.20.1 source and render/event fixtures.
- Completion state: COMPLETE — 357 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 13342 byte size match source tree; tests were inspected only and not run.

### 1681. `crates/horizon-client/src/ui_theme.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `83af0fe19246dbd8e1155cf3c01649b60f3dede0`
- Size: 30434 bytes; 842 logical lines
- Coverage: 1–400, 401–800, 801–842 EOF
- Purpose: Custom palette, fonts, atlas/cache, rounded geometry, boot screen and preferences.
- Dependencies / cross-references: horizon-theme, HUD atlas/batch, Android controls and font patches.
- Implementation / logic: Implements cached glyph/corner masks, atlas allocation, text and boot log rendering.
- Findings: Custom launcher design system, not vanilla screens.
- RustChunk relevance: Optional UI reference only.
- Decision: exclude; Horizon/Starling-specific UI
- 1.20.1 implications: Do not transplant into vanilla screen renderer.
- Follow-up checks: Atlas invalidation, overflow, thread-local cache, font license and Android memory.
- RustChunk implementation mapping: Optional RustChunk launcher UI layer.
- Evidence required before adoption: Atlas reload tests, Android density tests and font license.
- Completion state: COMPLETE — 842 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 30434 byte size match source tree; tests were inspected only and not run.

### 1682. `crates/horizon-client/src/vanillagen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5fbed7164202e5a4cbb99784be2a150db9ad4719`
- Size: 44140 bytes; 1090 logical lines
- Coverage: 1–400, 401–800, 801–1090 EOF
- Purpose: Overworld generator/streaming integration, proto/final chunks, biomes, ticks, structures, block entities and saved proto restore.
- Dependencies / cross-references: Worldgen, chunk palettes, proto saves, tick queues, structure worker, loot and streaming.
- Implementation / logic: Builds generators from preset/dimension/datapacks; converts/decorates/saves/restores proto chunks; exposes locate/biome/jigsaw queries.
- Findings: Horizon-specific integration; RNG, stage order, structure refs, block-entity/loot timing and datapack codecs need parity.
- RustChunk relevance: Central worldgen reference.
- Decision: rewrite required; core worldgen
- 1.20.1 implications: Rebuild against 1.20.1 ChunkGenerator/WorldGenRegion/ProtoChunk and Forge datapacks.
- Follow-up checks: Generation stages, seed, decoration halo, ticks, block entities, structures, heightmaps, biome quart mapping.
- RustChunk implementation mapping: RustChunk 1.20.1 worldgen-streaming adapter.
- Evidence required before adoption: 1.20.1 source, seeded chunk goldens, proto fixtures and Forge datapack tests.
- Completion state: COMPLETE — 1090 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 44140 byte size match source tree; tests were inspected only and not run.

### 1683. `crates/horizon-client/src/vanillagen/lod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a9ebe5d33f86eca16ce87cb92d723af11094b24a`
- Size: 45796 bytes; 1190 logical lines
- Coverage: 1–400, 401–800, 801–1190 EOF
- Purpose: Async far decoration batches, cached LOD columns, vegetation/island estimates and detail requests.
- Dependencies / cross-references: Generator, LOD columns, features, worker pool, queues and dimensions.
- Implementation / logic: Builds 8x8 speculative batches on idle threads and refines distant detail without saving speculative chunks.
- Findings: Horizon-only rendering pipeline; must not mutate gameplay state; cache/thread settings may be heavy on Android.
- RustChunk relevance: High-value far-chunk concept, not proven performance.
- Decision: reference; far-render optimization
- 1.20.1 implications: Isolate far rendering from authoritative chunks; profile adaptive threads/cache.
- Follow-up checks: Cancellation, halos, deterministic features, cache eviction and memory caps.
- RustChunk implementation mapping: RustChunk isolated far-detail worker.
- Evidence required before adoption: Seeded LOD goldens, concurrency tests and Android profiling.
- Completion state: COMPLETE — 1190 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 45796 byte size match source tree; tests were inspected only and not run.

### 1684. `crates/horizon-client/src/vanillagen/lod/islands.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f9acdf3c3008ba5e81c07690fb218b62ef534db8`
- Size: 6750 bytes; 206 logical lines
- Coverage: 1–206 EOF
- Purpose: Estimates small End island cones from feature origins and deterministic radius/shrink.
- Dependencies / cross-references: End feature origin query, seed hash and LOD spans.
- Implementation / logic: Caches origins and paints merged cone spans into far columns.
- Findings: Visual approximation only; never changes generated chunks.
- RustChunk relevance: Optional End far-render reference.
- Decision: reference; optional LOD
- 1.20.1 implications: Keep visual-only; verify 1.20.1 feature RNG.
- Follow-up checks: Origins, radius/shrink draws, negative coordinates and overlap.
- RustChunk implementation mapping: RustChunk End-island LOD overlay.
- Evidence required before adoption: 1.20.1 End feature source and seeded goldens.
- Completion state: COMPLETE — 206 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 6750 byte size match source tree; tests were inspected only and not run.

### 1685. `crates/horizon-client/src/vanillagen/lod/vegetation.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e1e6ee8b3b683cf2627384da5ebb46a0ddad3d3c`
- Size: 16703 bytes; 486 logical lines
- Coverage: 1–400, 401–486 EOF
- Purpose: Estimates trees/chorus from biome feature JSON, expected counts and stable hashing.
- Dependencies / cross-references: Biome feature JSON, registry, providers, placement modifiers and LOD spans.
- Implementation / logic: Traverses recognized features and paints canopy/trunk/chorus spans.
- Findings: Mean probabilities do not reproduce exact RNG or arbitrary modded features.
- RustChunk relevance: Visual approximation only.
- Decision: reference; optional LOD
- 1.20.1 implications: Keep outside gameplay; restrict to target codecs.
- Follow-up checks: Placement modifiers, provider schema, hash and density.
- RustChunk implementation mapping: RustChunk optional vegetation LOD.
- Evidence required before adoption: Target feature fixtures and visual comparison.
- Completion state: COMPLETE — 486 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 16703 byte size match source tree; tests were inspected only and not run.

### 1686. `crates/horizon-client/src/vanillagen/structure_mobs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2232355e22ca4fdbebb985fb0a17f603ccd99bee`
- Size: 17998 bytes; 523 logical lines
- Coverage: 1–400, 401–523 EOF
- Purpose: Structure/chunk spawn finalizeSpawn, group data, equipment, attributes, brain NBT and passengers.
- Dependencies / cross-references: Mob types, spawn registries, enchantments/items, biome tags, local difficulty, RNG and entity NBT.
- Implementation / logic: Loads registries, finalizes structure mobs and writes passenger/entity NBT.
- Findings: Requires Camel Husk and tests expect Zombie Nautilus; later than 1.20.1.
- RustChunk relevance: Useful behavior reference but incompatible unchanged.
- Decision: rewrite required; worldgen mob finalization
- 1.20.1 implications: Rebuild target species/registries and 1.20.1 NBT/Brain.
- Follow-up checks: Spawn reason, local difficulty, RNG, tags, equipment and passenger NBT.
- RustChunk implementation mapping: RustChunk 1.20.1 structure mob finalizer.
- Evidence required before adoption: Target finalizeSpawn source and deterministic structure fixtures.
- Completion state: COMPLETE — 523 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 17998 byte size match source tree; tests were inspected only and not run.

### 1687. `crates/horizon-client/src/win_screen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `039faab176fa42114b04b3e54a6c49af2871ae1e`
- Size: 20074 bytes; 490 logical lines
- Coverage: 1–400, 401–490 EOF
- Purpose: End poem/credits parsing, player name, obfuscation, narration, scrolling and backgrounds.
- Dependencies / cross-references: Text resources, styles, font, HUD sprites, portal blend and input.
- Implementation / logic: Loads poem/credits, builds narration and renders scrolling text/logo.
- Findings: Edition branding and resource/text APIs differ.
- RustChunk relevance: Useful target-era UI reference.
- Decision: adaptable; version-specific UI
- 1.20.1 implications: Adapt 1.20.1 resources/styles and remove Horizon edition sprite.
- Follow-up checks: Text assets, obfuscation RNG, credits JSON, wrap and scroll.
- RustChunk implementation mapping: RustChunk WinScreen/narration.
- Evidence required before adoption: 1.20.1 WinScreen and vanilla assets.
- Completion state: COMPLETE — 490 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 20074 byte size match source tree; tests were inspected only and not run.

### 1688. `crates/horizon-client/src/world_spawn.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ee6e0510183972f99d9bea4450b9abc6d0d5f536`
- Size: 8246 bytes; 269 logical lines
- Coverage: 1–269 EOF
- Purpose: Initial spawn spiral, respawn column search, heightmap/fluid/top-face checks and candidate ordering.
- Dependencies / cross-references: Heightmaps, collision/fluid, border, chunks and RNG.
- Implementation / logic: Implements 11x11 spiral, x-major chunk search and coprime-stride respawn candidates.
- Findings: Stride/radius/order need exact target parity.
- RustChunk relevance: Core spawn/respawn reference.
- Decision: adaptable; core world init
- 1.20.1 implications: Port 1.20.1 spawn search and border clamp.
- Follow-up checks: Spiral, heightmap, fluid, top face, candidate cap and RNG.
- RustChunk implementation mapping: RustChunk 1.20.1 spawn/respawn service.
- Evidence required before adoption: Target MinecraftServer/PlayerSpawnFinder source.
- Completion state: COMPLETE — 269 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 8246 byte size match source tree; tests were inspected only and not run.

### 1689. `crates/horizon-client/tests/custom_component_api.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c7af1d8fe0854bc851ec9a86e0bc1f63423a4164`
- Size: 5834 bytes; 151 logical lines
- Coverage: 1–151 EOF
- Purpose: Tests custom component registration, NBT quotas/depth, content identity and mod acceptance.
- Dependencies / cross-references: Mod loader, custom protocol IDs, NBT validation and pack identity.
- Implementation / logic: Exercises invalid/duplicate IDs, malformed NBT, oversize data and component filtering.
- Findings: Horizon extension, not vanilla 1.20.1.
- RustChunk relevance: Security test inventory.
- Decision: reference; tests only
- 1.20.1 implications: Do not port runtime; translate invariants if custom components added.
- Follow-up checks: Forge capability/data component design, quotas and recursion limits.
- RustChunk implementation mapping: RustChunk custom component security tests.
- Evidence required before adoption: Forge item NBT design and fuzz fixtures.
- Completion state: COMPLETE — 151 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 5834 byte size match source tree; tests were inspected only and not run.

### 1690. `crates/horizon-client/tests/custom_component_unregistered.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0ac8dbec889d37d813f2b804b20b099cf0aa2ff6`
- Size: 590 bytes; 23 logical lines
- Coverage: 1–23 EOF
- Purpose: Rejects an unregistered custom component wire ID.
- Dependencies / cross-references: Item slot codec and custom component registry.
- Implementation / logic: Sends first custom ID and checks unsupported error.
- Findings: Horizon wire extension only.
- RustChunk relevance: Negative protocol test concept.
- Decision: reference; tests only
- 1.20.1 implications: Do not port unchanged; define Forge protocol policy.
- Follow-up checks: 1.20.1 unknown component handling.
- RustChunk implementation mapping: RustChunk protocol negative test.
- Evidence required before adoption: Target protocol mappings and malformed packet fixtures.
- Completion state: COMPLETE — 23 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 590 byte size match source tree; tests were inspected only and not run.

### 1691. `crates/horizon-client/tests/custom_component_wire.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8463331d5cad472405a17741a2142a558c112b7d`
- Size: 6289 bytes; 163 logical lines
- Coverage: 1–163 EOF
- Purpose: Custom components across save NBT, wire slots, hashes, bundles, containers and malformed data.
- Dependencies / cross-references: Custom IDs, item NBT, slot/template codec, nested stacks and dynamic registries.
- Implementation / logic: Round-trips custom values and tests permission gating/hash identity.
- Findings: Not stock 1.20.1 protocol without negotiated extension.
- RustChunk relevance: Optional protocol test matrix.
- Decision: reference; tests only
- 1.20.1 implications: Keep out of baseline absent explicit extension handshake.
- Follow-up checks: Wire compatibility, nested bounds and Forge interoperability.
- RustChunk implementation mapping: RustChunk optional custom protocol tests.
- Evidence required before adoption: Protocol spec and client/server interoperability.
- Completion state: COMPLETE — 163 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 6289 byte size match source tree; tests were inspected only and not run.

### 1692. `crates/horizon-client/tests/datapack_biome_sync.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `781daa58ce7bf8402076cfbc49b3346e5c88e26f`
- Size: 5363 bytes; 121 logical lines
- Coverage: 1–121 EOF
- Purpose: Datapack biome registry sync and client reconstruction/recoloring.
- Dependencies / cross-references: Biome registry packets, overlays, biome JSON and colormaps.
- Implementation / logic: Builds pack biome and checks registry order/data and color reload.
- Findings: Horizon packet shape differs from 1.20.1.
- RustChunk relevance: Useful sync regression design.
- Decision: reference; tests only
- 1.20.1 implications: Translate to Forge 1.20.1 registry sync tests.
- Follow-up checks: Biome IDs, datapack reload and color behavior.
- RustChunk implementation mapping: RustChunk Forge biome sync tests.
- Evidence required before adoption: Target registry packets and pack fixtures.
- Completion state: COMPLETE — 121 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 5363 byte size match source tree; tests were inspected only and not run.

### 1693. `crates/horizon-client/tests/forced_chunks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3e8efa689ea54d4f0d8f26baf080ff9b879a4da2`
- Size: 5343 bytes; 143 logical lines
- Coverage: 1–143 EOF
- Purpose: Forced chunk ticket rings, generation status and lighting.
- Dependencies / cross-references: Streaming, ticket levels, chunk status and light engine.
- Implementation / logic: Runs synthetic forced chunk and checks loaded/light/feature/terrain rings.
- Findings: Horizon statuses are custom; assertions need target mapping.
- RustChunk relevance: Useful ticket regression cases.
- Decision: reference; tests only
- 1.20.1 implications: Translate to 1.20.1 ChunkMap/DistanceManager.
- Follow-up checks: Ticket level, ring distance, status and light.
- RustChunk implementation mapping: RustChunk forced ticket tests.
- Evidence required before adoption: 1.20.1 ChunkMap sources and fixtures.
- Completion state: COMPLETE — 143 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 5343 byte size match source tree; tests were inspected only and not run.

### 1694. `crates/horizon-client/tests/generation_saves.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `97d7bd942bb4b37a691babec2fd9805bb9ec167f`
- Size: 18765 bytes; 519 logical lines
- Coverage: 1–400, 401–519 EOF
- Purpose: Decoration consistency across travel/restart, proto restore, block entity IDs and tick persistence.
- Dependencies / cross-references: Streaming, proto save, decoration halo, block entities, ticks and region files.
- Implementation / logic: Simulates sessions and compares generated/saved/restored chunks.
- Findings: Horizon save architecture; target semantics need mapping.
- RustChunk relevance: High-value chunk correctness test inventory.
- Decision: reference; tests only
- 1.20.1 implications: Translate invariants to RustChunk 1.20.1 persistence.
- Follow-up checks: Proto decoration, neighbor writes, IDs, ticks, loot and save/load order.
- RustChunk implementation mapping: RustChunk worldgen persistence tests.
- Evidence required before adoption: 1.20.1 proto/full chunk sources and NBT fixtures.
- Completion state: COMPLETE — 519 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 18765 byte size match source tree; tests were inspected only and not run.

### 1695. `crates/horizon-client/tests/lan_bot.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2dd05f1d28b60a2e0d5ffb9f714751ca2d1caa81`
- Size: 59586 bytes; 1645 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1645 EOF
- Purpose: Offline protocol bot tests for login/config, registries, chunks/light, players/entities, recipes, menus, biome sync and LAN auth.
- Dependencies / cross-references: Protocol codecs, LAN Hub, chunk encoder, sessions, entities, menus and tick order.
- Implementation / logic: Drives bots against server and validates login, chunk packets, interactions, recipe sync, movement order and respawn.
- Findings: Packet IDs/version belong to newer pinned build; not Aternos/1.20.1 proof.
- RustChunk relevance: Very useful end-to-end test inventory.
- Decision: reference; tests only; protocol integration
- 1.20.1 implications: Rebuild tests against Forge 1.20.1 protocol; do not copy packet bytes.
- Follow-up checks: Handshake/config, offline login, chunk/light, recipes, movement and player-loaded semantics.
- RustChunk implementation mapping: RustChunk LAN/protocol integration bots.
- Evidence required before adoption: 1.20.1 protocol spec, Forge/Aternos interoperability and packet captures.
- Completion state: COMPLETE — 1645 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 59586 byte size match source tree; tests were inspected only and not run.

### 1696. `crates/horizon-client/tests/launch_api.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6f127c400f67a070dddda57a233c88badcac63b3`
- Size: 4189 bytes; 113 logical lines
- Coverage: 1–113 EOF
- Purpose: Launch API JSON, CLI exit codes, crash report paths and home/game-dir handling.
- Dependencies / cross-references: CLI parser, environment, filesystem and crash writer.
- Implementation / logic: Spawns binary and checks output and filesystem effects.
- Findings: Starling launcher-specific contract.
- RustChunk relevance: Useful launcher test idea.
- Decision: reference; tests only
- 1.20.1 implications: Translate to RustChunk CLI contract.
- Follow-up checks: Paths with spaces, exit codes, crash report and side effects.
- RustChunk implementation mapping: RustChunk CLI tests.
- Evidence required before adoption: RustChunk CLI specification and platform path tests.
- Completion state: COMPLETE — 113 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 4189 byte size match source tree; tests were inspected only and not run.

### 1697. `crates/horizon-client/tests/mod_api_hooks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c1b78d99b5eb814ec05ef0e1656db94147c81ee6`
- Size: 2639 bytes; 80 logical lines
- Coverage: 1–80 EOF
- Purpose: Compile/dispatch tests for key, movement and recipe-book category hooks.
- Dependencies / cross-references: Mod API traits, host, event dispatch and recipe registry.
- Implementation / logic: Implements sample Hooks and checks mutable writeback.
- Findings: Horizon API, not Forge lifecycle.
- RustChunk relevance: Behavior checklist only.
- Decision: reference; tests only; mod API
- 1.20.1 implications: Map to Forge event bus/input hooks.
- Follow-up checks: Event order, cancellation and supported recipe categories.
- RustChunk implementation mapping: RustChunk Forge adapter tests.
- Evidence required before adoption: Forge 1.20.1 event source and sample mod.
- Completion state: COMPLETE — 80 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 2639 byte size match source tree; tests were inspected only and not run.

### 1698. `crates/horizon-client/tests/streaming_meshes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bb5fae4d1a2d830601069dde178b15781df912b7`
- Size: 9766 bytes; 254 logical lines
- Coverage: 1–254 EOF
- Purpose: Tests loaded columns mesh on approach and relit neighbors remesh.
- Dependencies / cross-references: Streaming queues, light engine, mesher and upload lifecycle.
- Implementation / logic: Moves render center and compares mesh output after light changes.
- Findings: Horizon renderer fixtures; invariants broadly apply.
- RustChunk relevance: Directly relevant to far-chunk void issue.
- Decision: reference; tests only; core renderer regression
- 1.20.1 implications: Translate to RustChunk loaded-column mesh request and light invalidation tests.
- Follow-up checks: Range transitions, queue retention, neighbor relight and stale mesh.
- RustChunk implementation mapping: RustChunk chunk visibility/meshing tests.
- Evidence required before adoption: Android captures and deterministic mesh fixtures.
- Completion state: COMPLETE — 254 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 9766 byte size match source tree; tests were inspected only and not run.

### 1699. `crates/horizon-client/tests/tall_column.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `482d2dd3c6c2479a12690da424e00ed948e6e9ef`
- Size: 6204 bytes; 161 logical lines
- Coverage: 1–161 EOF
- Purpose: Tests streaming/light/meshing through 131 sections to y=1800.
- Dependencies / cross-references: Level shape, chunk layout, lighting, streaming and mesher.
- Implementation / logic: Builds tall spire and checks sky/block light and mesh consistency.
- Findings: Not default 1.20.1 height but relevant to modded dimensions.
- RustChunk relevance: Useful variable-height robustness tests.
- Decision: reference; tests only; height rendering
- 1.20.1 implications: Translate for Forge dimension height ranges.
- Follow-up checks: MinY/height, section count, light arrays and allocation bounds.
- RustChunk implementation mapping: RustChunk variable-height tests.
- Evidence required before adoption: Forge dimension type fixtures and memory tests.
- Completion state: COMPLETE — 161 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 6204 byte size match source tree; tests were inspected only and not run.

### 1700. `crates/horizon-client/tests/wide_states.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f65fa52cebe268708860177441a93af0908f8ba3`
- Size: 1560 bytes; 41 logical lines
- Coverage: 1–41 EOF
- Purpose: Tests palette state IDs beyond 65,536 and 17-bit global storage.
- Dependencies / cross-references: Block state registry, paletted container and section storage.
- Implementation / logic: Expands palette width and verifies section pack/unpack.
- Findings: Registry scale varies; must match 1.20.1/Forge.
- RustChunk relevance: Relevant to large modpacks.
- Decision: reference; tests only; registry scale
- 1.20.1 implications: Translate to RustChunk palette thresholds/serialization.
- Follow-up checks: Global/local palette transition and save/load.
- RustChunk implementation mapping: RustChunk palette tests.
- Evidence required before adoption: 1.20.1 PalettedContainer source and synthetic registry.
- Completion state: COMPLETE — 41 logical lines read through EOF in contiguous chunks no larger than 400; pinned blob SHA and 1560 byte size match source tree; tests were inspected only and not run.

### 1701. `crates/horizon-client/windows-segment-heap.manifest`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `67c8d113c62331dc59e8edc37631ac34c7ccfee6`
- Size: 707 bytes; 16 logical lines
- Coverage: 1–16 (EOF)
- Purpose: Windows application manifest settings for the Horizon client executable.
- Dependencies / cross-references: Windows PE application manifest and Segment Heap process-heap policy.
- Implementation / logic: Declares assembly identity, compatibility and execution-level metadata, and opts the executable into Segment Heap.
- Findings: Platform deployment metadata only; no Android runtime, rendering or Minecraft logic.
- RustChunk relevance: No direct Android target value; at most a desktop packaging reference.
- Decision: unnecessary; version-specific
- 1.20.1 implications: Do not include this manifest in Android artifacts.
- Follow-up checks: Revisit only if RustChunk gains a separately packaged Windows executable.
- RustChunk implementation mapping: None for the Android application.
- Evidence required before adoption: A concrete Windows packaging target and Windows compatibility testing.
- Completion state: COMPLETE — 16 logical lines read through EOF; pinned blob SHA and 707-byte size match the source tree.

### 1702. `crates/horizon-command/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `10191ed6f669ed3291bc3847b12f7b62f8a16c27`
- Size: 468 bytes; 15 logical lines
- Coverage: 1–15 (EOF)
- Purpose: Cargo package manifest for the command-parser crate.
- Dependencies / cross-references: Workspace settings and horizon-client-core, horizon-protocol and horizon-world crates.
- Implementation / logic: Declares package metadata, workspace lints, Rust workspace versioning and crate identity.
- Findings: Build metadata, not command behavior; dependencies are coupled to Starling workspace.
- RustChunk relevance: Dependency-boundary example only; not a drop-in manifest.
- Decision: unnecessary; reference
- 1.20.1 implications: Use RustChunk workspace and target-compatible dependencies.
- Follow-up checks: Review RustChunk workspace, MSRV and dependency graph before introducing a command crate.
- RustChunk implementation mapping: RustChunk workspace manifest and supported Android toolchain.
- Evidence required before adoption: Workspace graph, supported toolchain and dependency/license audit.
- Completion state: COMPLETE — 15 logical lines read through EOF; pinned blob SHA and 468-byte size match the source tree.

### 1703. `crates/horizon-command/src/args.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d49778698ac64b0142cefa8b2e865b8810bf3b9c`
- Size: 68804 bytes; 1838 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1838 (EOF)
- Purpose: Defines command argument values, registries, parsers and completion suggestions.
- Dependencies / cross-references: Brigadier Reader/Dispatcher, selector, block/item/NBT parsers, scoreboard and resource registries, text/component codecs.
- Implementation / logic: Parses primitive, numeric, coordinate, entity, resource/tag, item/block, NBT, particle, scoreboard, slot and version-specific argument types; generates context-sensitive suggestions.
- Findings: Includes Clock/Timeline, SlotSource, ResourceOrId and newer resource registry assumptions.
- RustChunk relevance: Core reference for command UX and parser structure, but argument set and validation are not 1.20.1-compatible unchanged.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Rebuild supported ArgType set from Minecraft 1.20.1 Brigadier registrations and Forge 47.x packet schemas.
- Follow-up checks: Compare target argument types, cursor rollback, error keys, range rules, registry lookup and suggestion sources.
- RustChunk implementation mapping: RustChunk command argument layer and completion adapters.
- Evidence required before adoption: 1.20.1 argument sources, Brigadier version, Forge command packet captures and differential parser tests.
- Completion state: COMPLETE — 1838 logical lines read through EOF; pinned blob SHA and 68804-byte size match the source tree; embedded tests were inspected only, not run.

### 1704. `crates/horizon-command/src/block.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `db38a6fbd3ee9f4d3320388e6efec292697e2a96`
- Size: 16212 bytes; 460 logical lines
- Coverage: 1–400, 401–460 (EOF)
- Purpose: Parses block states and block predicates, including properties, tags, optional block-entity NBT and suggestions.
- Dependencies / cross-references: Block and block-tag registries, property definitions, SNBT parser and block-entity registry.
- Implementation / logic: Validates property names/values, rejects duplicate or unknown properties, parses vague tag properties and tracks suggestion state.
- Findings: Core algorithm is reusable in concept, but registry shape and codec behavior come from the newer target snapshot.
- RustChunk relevance: Relevant to command parsing and block selection in modded worlds.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild registry/property access and NBT availability using 1.20.1 BlockState/BlockEntity rules and Forge registries.
- Follow-up checks: Test property ordering, duplicate properties, tags, empty states, SNBT rollback and completion against 1.20.1.
- RustChunk implementation mapping: RustChunk BlockStateArgument and BlockPredicate parser.
- Evidence required before adoption: 1.20.1 block-state definitions, block-tag fixtures and differential parser cases.
- Completion state: COMPLETE — 460 logical lines read through EOF; pinned blob SHA and 16212-byte size match the source tree.

### 1705. `crates/horizon-command/src/codec.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `40f5c1ce0cc5d512f44c678125fcd08caf6186f5`
- Size: 7641 bytes; 205 logical lines
- Coverage: 1–205 (EOF)
- Purpose: Reproduces codec validation and errors for resource-or-inline command arguments.
- Dependencies / cross-references: SNBT Tag model, resource registries, DataFixerUpper-style codecs, loot predicates/modifiers, context providers, slot sources and dialogs.
- Implementation / logic: Validates maps, required fields, registry type dispatch, element-or-list encodings and inline ResourceOrId values.
- Findings: Dialog, SlotSource and newer context-provider codecs exceed the 1.20.1 command/data schema.
- RustChunk relevance: Codec error ordering and validation helpers may inform a target-specific parser.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Implement only registries and codec schemas present in 1.20.1/Forge; exclude newer dialog/context-provider IDs.
- Follow-up checks: Verify target-era loot predicate/function codecs, resource-or-id semantics and exact failure cursor/message behavior.
- RustChunk implementation mapping: RustChunk versioned command codec validation module.
- Evidence required before adoption: 1.20.1 codec sources, registries and malformed-SNBT differential fixtures.
- Completion state: COMPLETE — 205 logical lines read through EOF; pinned blob SHA and 7641-byte size match the source tree.

### 1706. `crates/horizon-command/src/component.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2933afeca43484d5cbba69e8658f9beeb37b37a5`
- Size: 11139 bytes; 364 logical lines
- Coverage: 1–364 (EOF)
- Purpose: Validates text Component and Style SNBT codecs and reproduces partial/error results.
- Dependencies / cross-references: NBT tags, text/color helpers, entity selectors, NBT paths and click/hover event codecs.
- Implementation / logic: Decodes component content variants, style fields, click/hover actions, selectors, NBT sources and object components.
- Findings: Object content, dialog click action and other codec shapes are newer than the 1.20.1 text component model.
- RustChunk relevance: Important architecture reference for validating command-supplied components and style values.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild around 1.20.1 ComponentSerialization and Style.Serializer fields/actions; exclude newer content/action kinds.
- Follow-up checks: Compare text/translatable/score/selector/NBT components, colors, style booleans and click/hover payloads with 1.20.1.
- RustChunk implementation mapping: RustChunk 1.20.1 text component SNBT codec.
- Evidence required before adoption: Target-era component codec source and positive/negative SNBT fixtures.
- Completion state: COMPLETE — 364 logical lines read through EOF; pinned blob SHA and 11139-byte size match the source tree.

### 1707. `crates/horizon-command/src/item.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `da9c4b8b091c61c0425a1216d1f1706d86a4a472`
- Size: 21806 bytes; 657 logical lines
- Coverage: 1–400, 401–657 (EOF)
- Purpose: Parses item stacks and item predicates using the newer data-component representation.
- Dependencies / cross-references: Item/data-component registries, SNBT, Packrat grammar engine and block-entity data.
- Implementation / logic: Parses bracketed component assignments/removals and predicate expressions, validates values and suggests component IDs.
- Findings: The central data model is post-1.20.1; 1.20.1 uses legacy item NBT semantics and a different item command argument.
- RustChunk relevance: Parser architecture and negative-test concepts are useful, not the payload model.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Do not port component syntax as baseline behavior; rebuild ItemArgument/ItemPredicateArgument for 1.20.1 ItemStack and NBT.
- Follow-up checks: Test count/damage predicates, item NBT matching, enchantments, custom names, block-entity data and Forge items.
- RustChunk implementation mapping: RustChunk 1.20.1 item stack and item predicate arguments.
- Evidence required before adoption: 1.20.1 ItemParser/ItemPredicateParser source and Forge modded-item fixtures.
- Completion state: COMPLETE — 657 logical lines read through EOF; pinned blob SHA and 21806-byte size match the source tree; embedded tests were inspected only, not run.

### 1708. `crates/horizon-command/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f809dd166f7075a77c39abd44bb7731ee44c6fdb`
- Size: 41388 bytes; 1149 logical lines
- Coverage: 1–400, 401–800, 801–1149 (EOF)
- Purpose: Implements the Brigadier-style command dispatcher, reader, parse context, completion suggestions and smart usage.
- Dependencies / cross-references: Args, vanilla command tree, text components, command protocol tree, selectors and suggestion providers.
- Implementation / logic: Builds literal/argument nodes, parses redirects and permissions, records argument ranges/errors, merges/sorts completions and derives usage strings.
- Findings: Core dispatch architecture is broadly useful; Starling mod-command integration and newer providers/argument assumptions are project-specific.
- RustChunk relevance: High-value reference for client command parsing, chat coloring, Tab completion and context.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild target command tree and argument model while preserving only Brigadier semantics verified against Forge 47.x.
- Follow-up checks: Test parse branch selection, redirects, permission pruning, UTF-8 cursor boundaries, suggestion merge/dedup and signed message ranges.
- RustChunk implementation mapping: RustChunk Brigadier-compatible dispatcher and chat completion layer.
- Evidence required before adoption: Forge 1.20.1 Brigadier dependency/source, command tree captures and differential tests.
- Completion state: COMPLETE — 1149 logical lines read through EOF; pinned blob SHA and 41388-byte size match the source tree; embedded tests were inspected only, not run.

### 1709. `crates/horizon-command/src/network.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bf03276631438fe6ca51397ce3b06b3b18a79bc4`
- Size: 19453 bytes; 498 logical lines
- Coverage: 1–400, 401–498 (EOF)
- Purpose: Converts a server-sent command tree packet into a local dispatcher.
- Dependencies / cross-references: Horizon protocol command packet, parser ID table, ArgumentProperties, local vanilla tree and suggestion providers.
- Implementation / logic: Remaps root/node indices, matches local nodes by path, decodes parser properties and binds custom providers.
- Findings: Parser IDs, property payloads, command node layout and providers are tied to the pinned newer protocol.
- RustChunk relevance: Useful network-to-command-tree adapter pattern for multiplayer client UX.
- Decision: adaptable; version-specific
- 1.20.1 implications: Decode exact 1.20.1 ClientboundCommandsPacket and Forge extensions; do not reuse current parser IDs or payload parsing unchanged.
- Follow-up checks: Compare parser ID mapping, property flags, redirects, restricted nodes and server suggestions against 1.20.1 captures.
- RustChunk implementation mapping: RustChunk versioned server command-tree decoder feeding local Brigadier.
- Evidence required before adoption: 1.20.1 protocol specification, Forge command packet captures and interoperability tests.
- Completion state: COMPLETE — 498 logical lines read through EOF; pinned blob SHA and 19453-byte size match the source tree; embedded tests were inspected only, not run.

### 1710. `crates/horizon-command/src/packrat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ebe9df09d0a6dbfd84a15d8292ad869327a26592`
- Size: 20129 bytes; 627 logical lines
- Coverage: 1–400, 401–627 (EOF)
- Purpose: Implements a memoizing packrat grammar engine with scoped values, backtracking and error collection.
- Dependencies / cross-references: Command Reader, SNBT grammar, item predicate grammar and completion suggestions.
- Implementation / logic: Supports named/custom rules, sequence/alternation/optional/repeated terms, cuts, lookahead, frame scopes and furthest-position errors.
- Findings: Generic grammar machinery is relatively independent of Minecraft version; surrounding grammars are not.
- RustChunk relevance: Strong reusable architecture candidate for complex command arguments and SNBT parsing.
- Decision: adaptable; core reference
- 1.20.1 implications: Port only after checking UTF-8 cursor and error behavior; supply target-era argument/SNBT grammars.
- Follow-up checks: Verify memoization, frame merging, cut, repeated separators, furthest error and suggestions with fixtures.
- RustChunk implementation mapping: RustChunk packrat/parser combinator module for command grammars.
- Evidence required before adoption: Unit tests for grammar engine and differential SNBT/item-predicate fixtures against 1.20.1.
- Completion state: COMPLETE — 627 logical lines read through EOF; pinned blob SHA and 20129-byte size match the source tree; embedded tests were inspected only, not run.

### 1711. `crates/horizon-command/src/selector.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `91fc9d9e25cdfccdbc4c6a1d063cbb4e6f8c9b56`
- Size: 29084 bytes; 816 logical lines
- Coverage: 1–400, 401–800, 801–816 (EOF)
- Purpose: Parses entity selectors, UUID/player targets and selector completion suggestions.
- Dependencies / cross-references: Entity/player registries, GameMode, NBT parser, scoreboard, advancement and predicate registries.
- Implementation / logic: Handles selector kinds, options, invertible sets, ranges, entity types/tags, teams, gamemode, scores, advancements and NBT.
- Findings: Includes @n and newer registry/suggestion behavior; these are not assumed to exist in 1.20.1.
- RustChunk relevance: Core command parser reference for vanilla entity targeting and online-player completion.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild selector kinds, option registration order, validation and suggestions from 1.20.1 EntitySelectorParser.
- Follow-up checks: Compare duplicate/inversion, player-vs-entity rules, UUID parsing, permissions, score/advancement filters and error cursors.
- RustChunk implementation mapping: RustChunk 1.20.1 entity selector parser used by commands and packet tree.
- Evidence required before adoption: Mojang-mapped 1.20.1 selector source, registry/tag fixtures and differential selector test matrix.
- Completion state: COMPLETE — 816 logical lines read through EOF; pinned blob SHA and 29084-byte size match the source tree; embedded tests were inspected only, not run.

### 1712. `crates/horizon-command/src/snbt.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7472c7ae3e0fe45500917e7aae7f4c4f0611fdd5`
- Size: 31817 bytes; 981 logical lines
- Coverage: 1–400, 401–800, 801–981 (EOF)
- Purpose: Parses SNBT into NBT tags and supplies command argument completion/error behavior.
- Dependencies / cross-references: Packrat grammar engine, NBT Tag/Compound representation, Java numeric parsing and selector UUID parser.
- Implementation / logic: Parses scalar numbers, typed arrays, lists, compounds, quoted/unquoted strings, escapes and built-in operations.
- Findings: Adds binary/hex numerals, underscore separators, signed/unsigned suffixes, Unicode-name escapes and bool/uuid builtins beyond 1.20.1.
- RustChunk relevance: High-value parser architecture reference; grammar details must be versioned.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Reconstruct 1.20.1 TagParser grammar and number/escape semantics; do not accept newer syntax on 1.20.1 servers.
- Follow-up checks: Test numeric boundaries/suffixes, typed arrays, duplicate keys, escaping, trailing input and error positions against 1.20.1.
- RustChunk implementation mapping: RustChunk 1.20.1 SNBT parser and completion engine.
- Evidence required before adoption: Target TagParser source and differential SNBT corpus including malformed input.
- Completion state: COMPLETE — 981 logical lines read through EOF; pinned blob SHA and 31817-byte size match the source tree; embedded tests were inspected only, not run.

### 1713. `crates/horizon-command/src/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `69a9c402d2b778914fefd0f7f9a61f8395720454`
- Size: 33064 bytes; 881 logical lines
- Coverage: 1–400, 401–800, 801–881 (EOF)
- Purpose: Test inventory for command parsing, vanilla errors, permissions, suggestions and command-tree behavior.
- Dependencies / cross-references: Dispatcher, vanilla registrations, selector/block/item/NBT parsers and registry fixtures.
- Implementation / logic: Defines smoke and edge-case tests for targets, coordinates, execute/data/scoreboard/world commands and dedicated commands.
- Findings: Includes 26.3-era additions such as /tick, newer /random, @n and modern nodes; tests read but not run.
- RustChunk relevance: Useful test-case inventory, not proof of RustChunk correctness or 1.20.1 parity.
- Decision: undetermined; reference; tests only
- 1.20.1 implications: Translate relevant cases and expected values from the 1.20.1 command tree; remove later-version features.
- Follow-up checks: Build a target-version command regression matrix and execute it only in RustChunk test environment.
- RustChunk implementation mapping: RustChunk 1.20.1 Brigadier/command integration tests.
- Evidence required before adoption: 1.20.1 command source/packet fixture and runnable RustChunk test harness.
- Completion state: COMPLETE — 881 logical lines read through EOF; pinned blob SHA and 33064-byte size match the source tree; embedded tests were inspected only, not run.

### 1714. `crates/horizon-command/src/tree_data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `34ce17bea6933e7322298ddf15092ed6717eba13`
- Size: 6162 bytes; 181 logical lines
- Coverage: 1–181 (EOF)
- Purpose: Registers the /data and /tag command trees.
- Dependencies / cross-references: Dispatcher, argument types, selectors, NBT/NBT path parsing and context-provider codecs.
- Implementation / logic: Builds merge/get/remove/modify providers for entity, block and storage sources, then tag add/remove/list.
- Findings: Base structure is recognizable, but new compute context-provider subcommands are later additions.
- RustChunk relevance: Useful blueprint for organizing target-era data commands.
- Decision: adaptable; version-specific
- 1.20.1 implications: Keep only branches present in 1.20.1 and map arguments to legacy item/NBT semantics.
- Follow-up checks: Compare /data modify source operations, path parsing, permissions and tag command registration with 1.20.1.
- RustChunk implementation mapping: RustChunk 1.20.1 data/tag command tree.
- Evidence required before adoption: Target Commands.java/DataCommands sources and parser fixtures.
- Completion state: COMPLETE — 181 logical lines read through EOF; pinned blob SHA and 6162-byte size match the source tree.

### 1715. `crates/horizon-command/src/tree_execute.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `53085410ad3cd277d3d7174b25eaff0352fcef45`
- Size: 8907 bytes; 249 logical lines
- Coverage: 1–249 (EOF)
- Purpose: Registers the /execute command tree and redirect/conditional branches.
- Dependencies / cross-references: Dispatcher redirects, selector/entity arguments, coordinates, scoreboard, block/biome/data predicates and registries.
- Implementation / logic: Creates as/at/positioned/rotated/facing/align/anchored/in/on/store/if/unless/run branches and conditional nodes.
- Findings: Contains newer stopwatch, item/slot and relation forms beyond 1.20.1; baseline forms need target verification.
- RustChunk relevance: High-value model of a complex Brigadier redirecting command tree.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild /execute grammar exactly for 1.20.1, preserving redirect/fork semantics but excluding newer nodes.
- Follow-up checks: Test every modifier, nested execute, if/unless, store target, function fork and error cursor against vanilla.
- RustChunk implementation mapping: RustChunk 1.20.1 execute tree and parse-context adapter.
- Evidence required before adoption: Mojang-mapped 1.20.1 ExecuteCommand source and differential command fixtures.
- Completion state: COMPLETE — 249 logical lines read through EOF; pinned blob SHA and 8907-byte size match the source tree.

### 1716. `crates/horizon-command/src/tree_extra.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d1dc7116c907d93d97d0ec9c7a26a8f511011d34`
- Size: 10424 bytes; 279 logical lines
- Coverage: 1–279 (EOF)
- Purpose: Registers hosted-game and dedicated-server administration command trees.
- Dependencies / cross-references: Permission constants, profile/entity arguments, text components, test instance registry and Dispatcher.
- Implementation / logic: Adds dialog, kick, posteffect, fetchprofile, spectate, publish, waypoint, debug, JFR/test and dedicated op/ban/whitelist.
- Findings: Most command families are Starling/26.3-specific; dedicated-vs-integrated selection and permissions are the transferable concepts.
- RustChunk relevance: Limited reference for server-side command registration and admin separation.
- Decision: unnecessary; version-specific
- 1.20.1 implications: Do not register these names into the 1.20.1 baseline; implement only commands present in the target server.
- Follow-up checks: Compare Commands.CommandSelection behavior and administrative permissions in 1.20.1.
- RustChunk implementation mapping: Optional dedicated/integrated command registration policy.
- Evidence required before adoption: 1.20.1 Commands registration source and RustChunk server-mode model.
- Completion state: COMPLETE — 279 logical lines read through EOF; pinned blob SHA and 10424-byte size match the source tree.

### 1717. `crates/horizon-command/src/tree_misc.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1bcd880bbf21f6e057abb898d4f5b00c1eeb7d80`
- Size: 11995 bytes; 319 logical lines
- Coverage: 1–319 (EOF)
- Purpose: Registers miscellaneous item, bossbar, function, schedule, random, tick, team-message, reload and datapack command trees.
- Dependencies / cross-references: Dispatcher, Item/SlotSource, resource registries, GameRules and permissions.
- Implementation / logic: Constructs nested literal/argument branches, redirects aliases, applies permission levels and attaches item/resource arguments.
- Findings: Item uses modern component/slot machinery and Stopwatch/Random/Tick commands are newer than 1.20.1.
- RustChunk relevance: Command-tree construction pattern is useful; exact command set is not.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild registrations from 1.20.1 sources; retain only target-era forms for item, bossbar, function, schedule and datapack.
- Follow-up checks: Check target branches, min/max constraints, aliases, command IDs and permissions.
- RustChunk implementation mapping: RustChunk target-era miscellaneous command registration module.
- Evidence required before adoption: 1.20.1 command sources and regression fixtures for every supported branch.
- Completion state: COMPLETE — 319 logical lines read through EOF; pinned blob SHA and 11995-byte size match the source tree.

### 1718. `crates/horizon-command/src/tree_scoreboard.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9bb22a4a6932c6dd348145fe7878d6130fbfcad1`
- Size: 8862 bytes; 216 logical lines
- Coverage: 1–216 (EOF)
- Purpose: Registers scoreboard, team and trigger command trees.
- Dependencies / cross-references: ScoreHolder, objective/criteria/team/scoreboard-slot arguments and component/style parsing.
- Implementation / logic: Builds objective/player/team management operations, trigger commands and suggestion providers.
- Findings: NumberFormat and display-autoupdate branches belong to a later schema; conventional scoreboard operations remain relevant.
- RustChunk relevance: Useful organizational reference for a 1.20.1 command subsystem.
- Decision: adaptable; version-specific
- 1.20.1 implications: Remove later number-format/display-auto-update branches and align criteria/slot forms with 1.20.1.
- Follow-up checks: Verify objective criteria, operations, team visibility/collision literals, trigger permissions and suggestion order.
- RustChunk implementation mapping: RustChunk 1.20.1 scoreboard/team/trigger command tree.
- Evidence required before adoption: Target ScoreboardCommand/TeamCommand sources and expected parse/suggestion results.
- Completion state: COMPLETE — 216 logical lines read through EOF; pinned blob SHA and 8862-byte size match the source tree.

### 1719. `crates/horizon-command/src/tree_world.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `86ebba0caf210268fd5ff0f92bfd8ff15af3888b`
- Size: 16260 bytes; 419 logical lines
- Coverage: 1–400, 401–419 (EOF)
- Purpose: Registers world/entity command trees for recipe, damage, rotation, sound, particle, attribute, ride, forceload, biome, border and loot.
- Dependencies / cross-references: Entity/Player selectors, block positions, sound/particle/attribute registries, slots, loot codecs and Dispatcher.
- Implementation / logic: Builds nested command branches with argument constraints, aliases and execution IDs.
- Findings: Includes later /swing and expanded item-slot/loot forms; several core commands also exist in 1.20.1.
- RustChunk relevance: Good structural reference, but not a target-era specification.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild registrations against 1.20.1 command signatures; exclude commands and forms absent in target.
- Follow-up checks: Verify parameter order/ranges for fillbiome, forceload, worldborder, attributes, particles, sounds and loot.
- RustChunk implementation mapping: RustChunk 1.20.1 world command tree modules.
- Evidence required before adoption: Mojang-mapped 1.20.1 command sources and command packet fixtures.
- Completion state: COMPLETE — 419 logical lines read through EOF; pinned blob SHA and 16260-byte size match the source tree.

### 1720. `crates/horizon-command/src/vanilla_order.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fda8cc69a2b357d68d348d8024cc851566cf5beb`
- Size: 5804 bytes; 242 logical lines
- Coverage: 1–242 (EOF)
- Purpose: Generated child-node ordering table for the command dispatcher.
- Dependencies / cross-references: Vanilla command-tree registration and tools/generate_command_order.py.
- Implementation / logic: Lists path-specific child order for root, execute, item, loot and scoreboard nodes.
- Findings: Reflects the newer pinned command set and is generated from that tree; copying it gives wrong 1.20.1 commands/order.
- RustChunk relevance: Shows Brigadier child registration order affects usage and completion order.
- Decision: unnecessary; version-specific
- 1.20.1 implications: Generate a fresh ordering fixture from the 1.20.1 command tree rather than reuse this table.
- Follow-up checks: Compare registration order, smart usage and Tab completion order after target-tree construction.
- RustChunk implementation mapping: RustChunk 1.20.1 generated command-order fixture if order replication is needed.
- Evidence required before adoption: Target 1.20.1 child-order oracle and generated output.
- Completion state: COMPLETE — 242 logical lines read through EOF; pinned blob SHA and 5804-byte size match the source tree.

### 1721. `crates/horizon-command/src/vanilla.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `adee86016d6945d5e06ccca593675b8c0947a085`
- Size: 26455 bytes; 682 logical lines
- Coverage: 1–400, 401–682 (EOF)
- Purpose: Builds the shared vanilla command dispatcher and conditionally selects integrated, dedicated, development and Horizon trees.
- Dependencies / cross-references: All command tree modules, vanilla_order, gamerule registry, Dispatcher and versioned resource arguments.
- Implementation / logic: Registers broad commands, applies permissions and generated ordering, caches trees and optionally adds Horizon pregeneration commands.
- Findings: Contains newer Clock/Timeline time commands, Stopwatch and Minecart Improvements branches plus Horizon-specific /pregen.
- RustChunk relevance: Useful dispatcher composition and integrated/dedicated selection architecture.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Build command tree from 1.20.1 Commands.java and Forge 47.x feature set; omit newer time/stopwatch/feature branches.
- Follow-up checks: Check root order, aliases, permissions, gamerules and integrated/dedicated differences against 1.20.1.
- RustChunk implementation mapping: RustChunk command-tree assembly and server-mode selection layer.
- Evidence required before adoption: Target 1.20.1 Commands source, Forge command registrations and packet captures.
- Completion state: COMPLETE — 682 logical lines read through EOF; pinned blob SHA and 26455-byte size match the source tree.

### 1722. `crates/horizon-core/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `97c9356b87738441531292ef1be67a73a365bcbc`
- Size: 816 bytes; 41 logical lines
- Coverage: 1–41 (EOF)
- Purpose: Cargo manifest for simulation-foundations core and parity binaries.
- Dependencies / cross-references: Workspace settings, pinned libm dependency, RNG-trace feature and six parity executable targets.
- Implementation / logic: Declares package metadata, feature toggling and targets for parity, tick, geometry, noise, density and float-log comparisons.
- Findings: Workspace-specific build graph; binary declarations are tooling, not runtime modules.
- RustChunk relevance: Useful pattern for isolating deterministic simulation code from parity tools.
- Decision: adaptable; reference
- 1.20.1 implications: Create a RustChunk core crate only with target-version boundaries and verified numerical dependencies.
- Follow-up checks: Audit license/MSRV and decide whether parity tools belong in dev-only targets or separate crates.
- RustChunk implementation mapping: RustChunk simulation/core manifest and development parity tools.
- Evidence required before adoption: RustChunk workspace graph, target toolchain and dependency/license review.
- Completion state: COMPLETE — 41 logical lines read through EOF; pinned blob SHA and 816-byte size match the source tree.

### 1723. `crates/horizon-core/examples/biome_zoom_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4099bed1379de74ebb473905f80f79884bc26e29`
- Size: 1839 bytes; 46 logical lines
- Coverage: 1–46 (EOF)
- Purpose: Text-based differential fixture transport for biome zoom and seed-obfuscation calculations.
- Dependencies / cross-references: BiomeZoom, noise-quart calculations, BlockPos and private input/output fixture files.
- Implementation / logic: Reads scripted seed/block/position operations, calls the relevant functions and writes deterministic text results.
- Findings: Development oracle adapter, not runtime generation; malformed rows can panic due to unchecked fields.
- RustChunk relevance: Useful shape for comparing worldgen calculations against a 1.20.1 oracle.
- Decision: adaptable; reference
- 1.20.1 implications: Replace current functions with exact 1.20.1 biome zoom/noise semantics and keep outside production runtime.
- Follow-up checks: Add malformed-input handling and compare seed/zoom outputs for fixed target-version fixtures.
- RustChunk implementation mapping: Optional RustChunk biome-parity fixture executable.
- Evidence required before adoption: 1.20.1 BiomeManager/zoom source and trusted input/output corpus.
- Completion state: COMPLETE — 46 logical lines read through EOF; pinned blob SHA and 1839-byte size match the source tree.

### 1724. `crates/horizon-core/examples/generation_tick_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3d154e82ae8c9cc5d543a538459e770e8209f055`
- Size: 6883 bytes; 188 logical lines
- Coverage: 1–188 (EOF)
- Purpose: Differential fixture for scheduled-tick containers across proto and level chunk stages.
- Dependencies / cross-references: ProtoChunkTicks, ChunkTicks, SavedTick, ScheduledTick, TickPriority and WorldGenTickAccess.
- Implementation / logic: Replays reset/load/schedule/query/dump/snapshot/promote/unpack/remove operations and emits events/counts.
- Findings: Captures lifecycle/container transitions; fixture alone does not prove behavior matches 1.20.1.
- RustChunk relevance: High-value test design for chunk tick persistence and generation promotion.
- Decision: adaptable; core reference
- 1.20.1 implications: Rebuild semantics from 1.20.1 ProtoChunkTicks/LevelChunkTicks and scheduled tick ordering.
- Follow-up checks: Compare duplicate scheduling, chunk lookup order, priority/sub-order, promotion, failure propagation and save/load.
- RustChunk implementation mapping: RustChunk chunk tick differential fixture and regression tests.
- Evidence required before adoption: Target-era tick source and deterministic fixture outputs; no execution performed.
- Completion state: COMPLETE — 188 logical lines read through EOF; pinned blob SHA and 6883-byte size match the source tree.

### 1725. `crates/horizon-core/examples/paletted_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `feff4db89dda52a5ff68d4161976e0b2cd4c4d63`
- Size: 4230 bytes; 136 logical lines
- Coverage: 1–136 (EOF)
- Purpose: Differential fixture for PalettedContainer edits, wire serialization and packed storage round trips.
- Dependencies / cross-references: PalettedContainer, Strategy, palette/long-array encoding, VarInt and VanillaPalettedContainerOracle.
- Implementation / logic: Replays set operations, calculates hashes/checkpoints, serializes/rereads wire data, packs storage and compares loaded values.
- Findings: Useful regression harness pattern; strategy parameters and wire rules are newer-version-specific.
- RustChunk relevance: Directly relevant to large modpack registry sizes and section palette correctness.
- Decision: adaptable; core reference
- 1.20.1 implications: Rebuild block/biome palette strategy and network/NBT formats from exact 1.20.1 rules; do not reuse bit widths or thresholds without proof.
- Follow-up checks: Test local/global palette transitions, bits-per-entry limits, packed-long layout, VarInt lengths and read/write/save/load parity.
- RustChunk implementation mapping: RustChunk 1.20.1 PalettedContainer differential fixture.
- Evidence required before adoption: 1.20.1 PalettedContainer/BitStorage source and controlled registry-size fixtures.
- Completion state: COMPLETE — 136 logical lines read through EOF; pinned blob SHA and 4230-byte size match the source tree.

### 1726. `crates/horizon-core/src/bin/density_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `07bdf65374187727e7e9a318a095414fc8a0cf32`
- Size: 48978 bytes; 1103 logical lines
- Coverage: 1–400, 401–800, 801–1103 (EOF)
- Purpose: Differential transport for comparing density samplers and compiled density graphs.
- Dependencies / cross-references: DensitySampler, DensityCompiler/GraphNode, noise/random APIs, spline/surface samplers, intervals and volume layouts.
- Implementation / logic: Builds scripted samplers/graphs, queries scalar and bulk values, exercises caches, contexts and volumes, and emits float-bit results.
- Findings: Extensive parity tooling, not runtime generation; expected outputs are meaningful only against the matching vanilla version.
- RustChunk relevance: Strong test-harness reference for density evaluation, graph compilation, volume layout and cache behavior.
- Decision: adaptable; core reference
- 1.20.1 implications: Rebuild fixture operations and expected outputs against Minecraft 1.20.1 density graph/noise semantics; exclude newer nodes and schemas.
- Follow-up checks: Compare scalar/bulk bit patterns, cache reuse, failure writes, spline coordinates, volume indexing and seeded noise against 1.20.1.
- RustChunk implementation mapping: RustChunk differential density-graph test executable and parity corpus.
- Evidence required before adoption: Pinned 1.20.1 DensityFunctions/NoiseChunk sources and trusted oracle output; inspected, not executed.
- Completion state: COMPLETE — 1103 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 48978-byte size match the source tree; embedded tests were inspected only, not run.

### 1727. `crates/horizon-core/src/bin/float_log_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `570a7c666e5271472fe8800f24efa13fa26900a1`
- Size: 1930 bytes; 55 logical lines
- Coverage: 1–55 (EOF)
- Purpose: Binary transport for exhaustive comparison of the compiled float logarithm sampler.
- Dependencies / cross-references: UnarySampler Log operation, DensitySampler interface and IEEE-754 float bits.
- Implementation / logic: Feeds 32-bit float patterns through the log sampler in bounded batches and writes normalized result bits.
- Findings: Narrow parity utility; no gameplay logic or user-facing dependency.
- RustChunk relevance: Useful for validating Java-compatible float math and NaN canonicalization.
- Decision: adaptable; reference
- 1.20.1 implications: Use only as a test utility for exact 1.20.1 math; do not ship in Android runtime.
- Follow-up checks: Compare finite/exceptional float classes, signed zeros, infinities, NaN output and bit-range boundaries.
- RustChunk implementation mapping: Optional RustChunk float-math differential executable.
- Evidence required before adoption: 1.20.1 MathHelper/Java math oracle and generated expected corpus.
- Completion state: COMPLETE — 55 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 1930-byte size match the source tree.

### 1728. `crates/horizon-core/src/bin/geometry_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a51f518877644988f7c532c4046068b5e6e7ce46`
- Size: 21882 bytes; 564 logical lines
- Coverage: 1–400, 401–564 (EOF)
- Purpose: Differential transport for geometry, collision shapes and entity movement.
- Dependencies / cross-references: Vec3/AABB/BlockPos, VoxelShape boolean operations, block traversal, ray clipping and movement resolvers.
- Implementation / logic: Exercises vector math, boxes, shapes, voxel grids, block sweeps, collision resolution and step movement with bit-precise output.
- Findings: Oracle adapter rather than implementation; behavior and ordering must be checked against target mappings.
- RustChunk relevance: High relevance to player/entity physics and block-collision correctness.
- Decision: adaptable; core reference
- 1.20.1 implications: Reconstruct 1.20.1 Vec3, AABB, VoxelShape and Entity collision semantics; verify pinned 26.3 behavior separately.
- Follow-up checks: Differentially test negative coordinates, boundary contact, shape unions, clipping face order, sweep order, step-up and float edges.
- RustChunk implementation mapping: RustChunk geometry/physics differential test harness.
- Evidence required before adoption: Mojang-mapped 1.20.1 geometry/collision sources and target-version traces.
- Completion state: COMPLETE — 564 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 21882-byte size match the source tree; embedded tests were inspected only, not run.

### 1729. `crates/horizon-core/src/bin/noise_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `52085cddd888b1f60649f8642ae8f70b10271915`
- Size: 16503 bytes; 423 logical lines
- Coverage: 1–400, 401–423 (EOF)
- Purpose: Differential transport for Perlin/Simplex/Normal noise and noise-stack evaluation.
- Dependencies / cross-references: Noise implementations, LegacyRandom/XoroshiroRandom, GradientTable and DensityVolume.
- Implementation / logic: Creates noise samplers, emits permutations/ranges, samples points/volumes and probes derivatives, stacks and random stream state.
- Findings: Noise generation, normalization and RNG choice are version- and seed-sensitive.
- RustChunk relevance: Useful test design for world generation reproducibility and exact noise output.
- Decision: adaptable; core reference
- 1.20.1 implications: Rebuild 1.20.1 noise constructors, octave selection and random-consumption order; verify RNG algorithm paths independently.
- Follow-up checks: Compare permutations, offsets, octave frequencies/amplitudes, normalization, derivatives, volume order and seeded outputs.
- RustChunk implementation mapping: RustChunk worldgen noise differential tests.
- Evidence required before adoption: 1.20.1 Perlin/NormalNoise sources and deterministic seed/output fixtures.
- Completion state: COMPLETE — 423 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 16503-byte size match the source tree; embedded tests were inspected only, not run.

### 1730. `crates/horizon-core/src/bin/parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `964391b3f8697fcb2cd95b8a8bb457d23a4aa52c`
- Size: 11853 bytes; 321 logical lines
- Coverage: 1–321 (EOF)
- Purpose: Parity adapter for random streams, positional factories, coordinates and packed storage.
- Dependencies / cross-references: Legacy/Xoroshiro random APIs, BlockPos/ChunkPos/SectionPos and PackedStorage.
- Implementation / logic: Replays random calls, positional seed/hash construction, coordinate pack/unpack and storage set/fill/import operations.
- Findings: RNG APIs and packed-storage invariants are version-specific; adapter does not prove save or protocol compatibility.
- RustChunk relevance: Useful test inventory for deterministic RNG, coordinate packing and palette backing storage.
- Decision: adaptable; core reference
- 1.20.1 implications: Rebuild target-era RNG and coordinate/storage rules from 1.20.1 sources; do not carry newer RNG stream selection without evidence.
- Follow-up checks: Check RNG consumption, bounded values, UTF-16 hashing, signed coordinate extremes, pack/unpack and storage bounds.
- RustChunk implementation mapping: RustChunk deterministic RNG/position/PackedStorage regression harness.
- Evidence required before adoption: Mojang-mapped 1.20.1 RNG/BlockPos/BitStorage sources and fixtures.
- Completion state: COMPLETE — 321 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 11853-byte size match the source tree; embedded tests were inspected only, not run.

### 1731. `crates/horizon-core/src/bin/tick_parity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4b8c46ea0bb1db5f5817ea7d14cba4631c3c8bb`
- Size: 9531 bytes; 246 logical lines
- Coverage: 1–246 (EOF)
- Purpose: Trace adapter for chunk scheduled-tick lifecycle and level tick execution.
- Dependencies / cross-references: LevelTicks/ChunkTicks, SavedTick/ScheduledTick, TickPriority, TickArea and chunk lifecycle.
- Implementation / logic: Exercises add/load/unpack/remove/restore, duplicate scheduling, query, save/reload, clear/copy and callback-time mutation.
- Findings: Trace adapter only; tick ordering and chunk-removal semantics remain target-version-dependent.
- RustChunk relevance: High relevance to deterministic block/fluid ticks and chunk persistence.
- Decision: adaptable; core reference
- 1.20.1 implications: Rebuild scheduler behavior from 1.20.1 LevelTicks, ProtoChunkTicks and LevelChunkTicks; verify exact queue priority/sub-order.
- Follow-up checks: Test duplicates, callback scheduling, chunk unload/reload, tick-area copy, save delays and deterministic order.
- RustChunk implementation mapping: RustChunk scheduled-tick service and persistence regression tests.
- Evidence required before adoption: 1.20.1 tick scheduler sources and reference traces; fixture not executed.
- Completion state: COMPLETE — 246 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 9531-byte size match the source tree; embedded tests were inspected only, not run.

### 1732. `crates/horizon-core/src/biome.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9a26bac139444307c97c43721b2e07f5c18e4b61`
- Size: 12184 bytes; 301 logical lines
- Coverage: 1–301 (EOF)
- Purpose: Block-to-quart biome selection and biome seed obfuscation for Java Edition 26.3.
- Dependencies / cross-references: BlockPos, seed hash, signed coordinate wrapping and eight-corner jitter selection.
- Implementation / logic: Hashes the world seed, computes eight-corner jitter and selects the nearest corner with strict comparison.
- Findings: Explicitly targets Java 26.3; seed transform and zoom algorithm are not presumed to match 1.20.1.
- RustChunk relevance: Conceptual reference for biome quart lookup only; no direct target adoption.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Replace with exact 1.20.1 biome zoom/seed algorithm; retain only details independently verified against target source.
- Follow-up checks: Compare seed transform, negative-coordinate arithmetic, corner order, tie behavior and boundary positions with 1.20.1.
- RustChunk implementation mapping: RustChunk 1.20.1 biome quart lookup adapter.
- Evidence required before adoption: 1.20.1 BiomeManager/zoom source and deterministic world-seed fixtures; embedded tests not run.
- Completion state: COMPLETE — 301 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 12184-byte size match the source tree; embedded tests were inspected only, not run.

### 1733. `crates/horizon-core/src/biome/seed_hash.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e88d67ae1ddc37507f074f3498bd0763ff3b901b`
- Size: 2780 bytes; 66 logical lines
- Coverage: 1–66 (EOF)
- Purpose: Specialized SHA-256 transform for an eight-byte world seed.
- Dependencies / cross-references: Fixed SHA-256 constants, little-endian seed input and digest-byte ordering.
- Implementation / logic: Pads one seed value, performs one SHA-256 compression block and converts the first eight digest bytes to a signed long.
- Findings: Compatibility transform for a newer biome seed scheme, not a general cryptography API.
- RustChunk relevance: No baseline value unless 1.20.1 evidence requires this exact seed mapping.
- Decision: unnecessary; version-specific
- 1.20.1 implications: Do not use for 1.20.1 biome seed handling unless target source proves an identical transform.
- Follow-up checks: Verify target biome seed derivation independently; do not reuse as general-purpose crypto.
- RustChunk implementation mapping: None unless a separately verified 1.20.1 seed transform is needed.
- Evidence required before adoption: Authoritative 1.20.1 biome seed source demonstrating identical SHA-256 transform.
- Completion state: COMPLETE — 66 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 2780-byte size match the source tree.

### 1734. `crates/horizon-core/src/carving.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e539203f31c7e2079d4872be78e1c6cf4ef6cdcd`
- Size: 5493 bytes; 164 logical lines
- Coverage: 1–164 (EOF)
- Purpose: Packed carving mask with ordered contiguous column-run iteration.
- Dependencies / cross-references: Chunk-local X/Z coordinates, inclusive Y bounds, bitset storage and visitor iteration.
- Implementation / logic: Stores carved cells as bits and traverses ascending set runs split at column boundaries; visitor mutations affect later searches.
- Findings: Height cap, index arithmetic and run order may differ from 1.20.1.
- RustChunk relevance: Useful architecture reference for compact chunk carving masks.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild mask layout, height bounds, out-of-range behavior and iteration order from 1.20.1 CarvingMask.
- Follow-up checks: Test negative/minY coordinates, section ranges, aliasing, cross-word runs, visitor mutation and capacity boundaries.
- RustChunk implementation mapping: RustChunk chunk carving-mask implementation and tests.
- Evidence required before adoption: 1.20.1 CarvingMask source and boundary fixtures; no tests run.
- Completion state: COMPLETE — 164 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 5493-byte size match the source tree.

### 1735. `crates/horizon-core/src/carving/ellipsoid.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `49b9d0e3a60656203c2eccdd31af8419480c3818`
- Size: 4404 bytes; 124 logical lines
- Coverage: 1–124 (EOF)
- Purpose: Shared ellipsoid traversal and path reachability for cave/canyon carvers.
- Dependencies / cross-references: ChunkPos, CarvingMask/CarverOutput, floor conversion and horizontal/vertical radii.
- Implementation / logic: Computes chunk reach, visits candidate cells in X/Z/descending-Y order, applies skip predicate and enforces traversal budget.
- Findings: Budget, traversal caps and inclusive/exclusive bounds are safeguards that may diverge from vanilla order.
- RustChunk relevance: Relevant to cave/canyon generation and chunk-boundary behavior.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild cave/canyon candidate bounds, floor rounding, traversal order and skip semantics from 1.20.1 carver sources.
- Follow-up checks: Compare chunk reach, min/max Y, negative coordinates, zero radii, boundary cells, failure retention and RNG consumption.
- RustChunk implementation mapping: RustChunk 1.20.1 carver ellipsoid helper.
- Evidence required before adoption: 1.20.1 CaveWorldCarver/CanyonWorldCarver sources and seeded fixtures.
- Completion state: COMPLETE — 124 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 4404-byte size match the source tree.

### 1736. `crates/horizon-core/src/density/arithmetic.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `51892ffc78dc68b7f04979e622575fbe1a7ca161`
- Size: 16419 bytes; 508 logical lines
- Coverage: 1–400, 401–508 (EOF)
- Purpose: Scalar and bulk arithmetic samplers with Java-compatible float semantics.
- Dependencies / cross-references: DensitySampler, intervals, Java power/log helpers, unary/binary operations and volume buffers.
- Implementation / logic: Implements unary/binary operations, signed-zero/NaN-aware min/max, power and specialized bulk loops; includes bit-exact unit-test cases.
- Findings: Bulk operations intentionally differ in some NaN handling from scalar paths; semantics require target parity.
- RustChunk relevance: Strong reference for deterministic density arithmetic and avoiding accidental performance-only behavior changes.
- Decision: adaptable; core reference
- 1.20.1 implications: Port only operations present in 1.20.1 and verify exact Java float semantics before enabling bulk specialization.
- Follow-up checks: Compare NaN payloads, signed zeros, infinities, overflow, power/log edges and scalar-vs-volume results.
- RustChunk implementation mapping: RustChunk density arithmetic with scalar/bulk differential tests.
- Evidence required before adoption: 1.20.1 DensityFunctions/MathHelper behavior and float-bit fixtures; tests read only.
- Completion state: COMPLETE — 508 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 16419-byte size match the source tree; embedded tests were inspected only, not run.

### 1737. `crates/horizon-core/src/density/beardifier.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b041be160ef371923355d54c2c622fb4fd7f1c7b`
- Size: 14409 bytes; 384 logical lines
- Coverage: 1–384 (EOF)
- Purpose: Structure terrain adaptation for rigid pieces, jigsaw junctions and optional terrain kernels.
- Dependencies / cross-references: Structure bounds/pieces, junctions, DensitySampler, Beardify kernel and YUNG/Moogs-style terrain adaptation.
- Implementation / logic: Combines vanilla-like beard/bury terms with optional custom terrain-kernel pieces and junctions; supports scalar and volume sampling.
- Findings: File mixes vanilla terrain adaptation with mod-library-inspired kernels; those are not vanilla evidence.
- RustChunk relevance: Useful for structure terrain density and modded structure compatibility if separated cleanly.
- Decision: adaptable; version-specific; modded extension
- 1.20.1 implications: Port target 1.20.1 vanilla Beardifier separately; treat YUNG/Moogs kernels as optional mod integration with separate source/license review.
- Follow-up checks: Verify vanilla offsets, kernel indexing, affected bounds, junction weights and modded kernels separately.
- RustChunk implementation mapping: RustChunk structure density adapter and isolated optional mod terrain-kernel interface.
- Evidence required before adoption: 1.20.1 Beardifier source and separate YUNG/Moogs library source/version fixtures; tests not run.
- Completion state: COMPLETE — 384 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 14409-byte size match the source tree; embedded tests were inspected only, not run.

### 1738. `crates/horizon-core/src/density/cache.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `477f247c057b40dee670233471872c3a2f898289`
- Size: 4711 bytes; 144 logical lines
- Coverage: 1–144 (EOF)
- Purpose: Context-owned scalar/volume caching for density samplers, including sparse and plane-cache modes.
- Dependencies / cross-references: SamplerContext, DensityVolume, BlockPos packing, graph cache IDs and recursive evaluation state.
- Implementation / logic: Caches scalar/volume samples, guards recursive entry, reuses scratch buffers and optionally retains Y-fixed XZ planes for previews.
- Findings: Sparse/plane caching is a preview extension; graph cache IDs and scratch lifetimes require isolation.
- RustChunk relevance: Relevant to reducing repeated density work for far-chunk/LOD previews, but no performance benefit is proven.
- Decision: adaptable; optional/future
- 1.20.1 implications: Keep preview-only sparse/plane cache separate from authoritative 1.20.1 terrain generation; validate context reset and cache namespace isolation.
- Follow-up checks: Test key collisions, graph reuse, worker isolation, failed writes, recursion, memory retention and preview/full consistency.
- RustChunk implementation mapping: RustChunk LOD density preview cache, separate from vanilla terrain cache.
- Evidence required before adoption: Target density semantics plus thread-safety, memory and exact-output tests; no benchmarks.
- Completion state: COMPLETE — 144 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 4711-byte size match the source tree.

### 1739. `crates/horizon-core/src/density/end_islands.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2ae9f9b5fe1ab150c5605f1fc13f8fe7a8033678`
- Size: 8854 bytes; 217 logical lines
- Coverage: 1–217 (EOF)
- Purpose: End outer-island density sampler for Java Edition 26.3.
- Dependencies / cross-references: SimplexNoise, LegacyRandom, world seed, signed division/remainder and DensityVolume.
- Implementation / logic: Consumes fixed random sequence, evaluates candidate outer islands and accelerates volume evaluation by collecting island candidates once.
- Findings: Explicitly targets Java 26.3; RNG consumption and truncating negative-coordinate arithmetic need target proof.
- RustChunk relevance: Conceptual End-island reference only; not safe for direct 1.20.1 adoption.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild from 1.20.1 End island generation and exact random-consumption order; exclude 26.3-specific seed/volume optimization until verified.
- Follow-up checks: Compare random draws, island exclusion circle, negative-coordinate truncation, clamping and scalar/bulk output.
- RustChunk implementation mapping: RustChunk 1.20.1 End-island density implementation.
- Evidence required before adoption: 1.20.1 End-island source and seeded boundary fixtures; embedded tests not run.
- Completion state: COMPLETE — 217 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 8854-byte size match the source tree; embedded tests were inspected only, not run.

### 1740. `crates/horizon-core/src/density/graph/compiler.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `53d8fe2eb2947933d3fb1b33bc371bf592d71304`
- Size: 19876 bytes; 498 logical lines
- Coverage: 1–400, 401–498 (EOF)
- Purpose: Compiler from immutable density definitions to executable samplers.
- Dependencies / cross-references: DensityGraph/GraphNode, cache IDs, seeded noise context, interval ranges, spline and sampler implementations.
- Implementation / logic: Rewrites graph children, prepares cache nodes, slices uniform axes, compiles sampler variants and folds constant arithmetic/selection nodes.
- Findings: Optimization and evaluation order are encoded in cache preparation, range pruning and child compilation; changes can affect failures and float results.
- RustChunk relevance: High-value architecture for a typed density DAG and compile-once sampler graph.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Build target-version node set/compiler from 1.20.1 density definitions; verify each rewrite/specialization before enabling.
- Follow-up checks: Test compile traversal, cache IDs, uniform-axis slicing, range elimination, NaNs, child failures and exact sample order.
- RustChunk implementation mapping: RustChunk immutable density graph and compiler.
- Evidence required before adoption: 1.20.1 DensityFunctions/NoiseChunk implementation and differential compile/sample fixtures.
- Completion state: COMPLETE — 498 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 19876-byte size match the source tree; embedded tests were inspected only, not run.

### 1741. `crates/horizon-core/src/density/graph/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8eefe2d1df3ae251eae0597fe543cc23121eea12`
- Size: 24454 bytes; 712 logical lines
- Coverage: 1–400, 401–712 (EOF)
- Purpose: Typed immutable density graph definitions, registration, interning, identity and range analysis.
- Dependencies / cross-references: GraphNode/FunctionId/NoiseId/HolderId, graph entries, noise/function registries and domain-axis masks.
- Implementation / logic: Defines node variants, canonical keys, child rewriting, equality/hash identity, depth limits, holder/noise registration, axis masks and range propagation.
- Findings: Code comments say codecs/remaining function kinds are pending; graph includes newer features such as EndIslands/IntervalSelect.
- RustChunk relevance: Core architecture reference, but not a complete vanilla 1.20.1 graph or codec.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild node schema and range rules from 1.20.1 DensityFunctions; retain only structures validated against target.
- Follow-up checks: Test identity, signed-zero/hash behavior, depth, duplicate registry IDs, holder identity, axis masks and ranges.
- RustChunk implementation mapping: RustChunk versioned density DAG and range-analysis layer.
- Evidence required before adoption: 1.20.1 density-function schema and deterministic graph equality/range fixtures.
- Completion state: COMPLETE — 712 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 24454-byte size match the source tree; embedded tests were inspected only, not run.

### 1742. `crates/horizon-core/src/density/graph/seeded.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `17d66ce0f3feb82e7eab7b8474992c421827b108`
- Size: 3177 bytes; 97 logical lines
- Coverage: 1–97 (EOF)
- Purpose: Seed-bound context for creating and caching registered noise samplers.
- Dependencies / cross-references: Legacy/Xoroshiro positional random factories, NormalNoise registry entries, world seed and mutex-protected sampler map.
- Implementation / logic: Creates seeded factories, hashes named noise IDs to derive instances, caches ordinary noises and recreates two legacy Nether noises per lookup.
- Findings: RNG factories, name hashing and Nether exceptions are version/seed sensitive.
- RustChunk relevance: Useful separation of immutable graph definitions from seed-dependent runtime state.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild RandomState semantics, name hashing, cache identity and Nether exceptions from 1.20.1.
- Follow-up checks: Compare RNG initialization, UTF-16 name hashing, noise reuse, Nether reseeding and synchronization.
- RustChunk implementation mapping: RustChunk seeded worldgen compilation context.
- Evidence required before adoption: 1.20.1 RandomState/NormalNoise source and deterministic named-noise fixtures.
- Completion state: COMPLETE — 97 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 3177-byte size match the source tree.

### 1743. `crates/horizon-core/src/density/graph/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4f3358460769d3941f8899ad68adb4b459f70ab0`
- Size: 18414 bytes; 552 logical lines
- Coverage: 1–400, 401–552 (EOF)
- Purpose: Regression test inventory for graph compilation, optimization, cache identity and density evaluation.
- Dependencies / cross-references: DensityGraph/Compiler, samplers, seeded noise, spline, Interval and cache.
- Implementation / logic: Covers compilation failures, cache ID reuse, graph equality, axis slicing, scalar/bulk values, EndIslands and range pruning.
- Findings: Tests cover newer-version behavior and internal implementation contracts; inspected only, not run.
- RustChunk relevance: Useful high-coverage checklist for RustChunk density-compiler testing.
- Decision: undetermined; reference; tests only
- 1.20.1 implications: Translate cases and expected values from 1.20.1; remove graph nodes absent from target.
- Follow-up checks: Add target-era differential tests for cache prep, graph deduplication, failures, signed zeros, spline ranges and buffers.
- RustChunk implementation mapping: RustChunk density graph test matrix.
- Evidence required before adoption: 1.20.1 density source, verified oracle outputs and runnable test harness; tests not executed.
- Completion state: COMPLETE — 552 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 18414-byte size match the source tree; embedded tests were inspected only, not run.

### 1744. `crates/horizon-core/src/density/interpolation.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4bcb51a3a10d56dd00225df837d1d142fca0d850`
- Size: 8523 bytes; 219 logical lines
- Coverage: 1–219 (EOF)
- Purpose: Coarse-grid density interpolation for scalar and bulk worldgen evaluation.
- Dependencies / cross-references: DensitySampler, DensityVolume, per-context buffers and floating-point lerp order.
- Implementation / logic: Samples coarse corner grids, interpolates X/Y/Z and supports scalar queries, dense volumes and sparse preview evaluation.
- Findings: Bulk path preserves repeated-addition rounding and distinct scalar/volume calls; sparse path is preview-specific.
- RustChunk relevance: High relevance to terrain density interpolation and chunk generation.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild interpolation cell sizes, boundaries and exact float order from 1.20.1 NoiseChunk/InterpolatedNoiseSampler.
- Follow-up checks: Compare negative cell coordinates, strided volumes, terminal cells, repeated addition, sparse/full results and buffer tails.
- RustChunk implementation mapping: RustChunk 1.20.1 density interpolation sampler.
- Evidence required before adoption: 1.20.1 NoiseChunk interpolation source and scalar/bulk oracle traces.
- Completion state: COMPLETE — 219 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 8523-byte size match the source tree.

### 1745. `crates/horizon-core/src/density/interval.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0e2652fe1c58c11e968a5487c1a098c3b912160f`
- Size: 13799 bytes; 412 logical lines
- Coverage: 1–400, 401–412 (EOF)
- Purpose: Interval arithmetic for density graph range analysis.
- Dependencies / cross-references: Java-compatible min/max/power/log helpers, f32 endpoints, NAI and infinities.
- Implementation / logic: Implements arithmetic bounds, reciprocal/division, powers, clamps, monotonic maps, lerp and sign ranges with signed-zero handling.
- Findings: Range rules mirror selected vanilla behavior but must be compared to 1.20.1; float edge cases are observable.
- RustChunk relevance: Useful for safe graph optimization and range-based pruning.
- Decision: adaptable; core reference
- 1.20.1 implications: Reconstruct target 1.20.1 range semantics and preserve NaN, infinity and signed-zero behavior before pruning.
- Follow-up checks: Test operators at zero crossings, infinities, negative bases, NaNs, signed zeros and non-monotonic inputs.
- RustChunk implementation mapping: RustChunk density interval/range-analysis module.
- Evidence required before adoption: 1.20.1 density range source and bit-precise edge-case fixtures.
- Completion state: COMPLETE — 412 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 13799-byte size match the source tree.

### 1746. `crates/horizon-core/src/density/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3f5ae7a7280b3be7a2913d3433f95605afc63dd`
- Size: 13472 bytes; 407 logical lines
- Coverage: 1–400, 401–407 (EOF)
- Purpose: Core DensitySampler trait, worker evaluation context and primitive sampler implementations.
- Dependencies / cross-references: DensityVolume, Axis, cache/slice helpers, Noise, GradientSampler and shifted noise.
- Implementation / logic: Defines scalar/bulk contract, context-owned scratch/caches/world fields, buffer validation, constant/gradient/noise and shifted-noise samplers.
- Findings: Source explicitly targets Java 26.3 and says complete terrain generation is pending.
- RustChunk relevance: Core API architecture reference; target behavior and graph features need reconstruction.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Define RustChunk 1.20.1 sampler API, then port target-compatible primitives; isolate preview-specific context modes.
- Follow-up checks: Test buffer tails, exact/minimum buffer sizes, scratch reuse, worker isolation, gradient tiling and shift order.
- RustChunk implementation mapping: RustChunk density sampler trait and per-worker evaluation context.
- Evidence required before adoption: 1.20.1 NoiseChunk/DensityFunction sources and target scalar/bulk fixtures.
- Completion state: COMPLETE — 407 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 13472-byte size match the source tree.

### 1747. `crates/horizon-core/src/density/power.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b400093ae91bcb2565c7d3d0e2fc29315e4a773a`
- Size: 2317 bytes; 72 logical lines
- Coverage: 1–72 (EOF)
- Purpose: Power density sampler and constant-exponent specialization.
- Dependencies / cross-references: DensitySampler, Java-compatible power helper and unary arithmetic.
- Implementation / logic: Samples base then exponent, evaluates bulk operands separately and specializes sqrt/identity/square/cube with selected negative-exponent reciprocals.
- Findings: Specialization and operand order are implementation-sensitive and require target parity.
- RustChunk relevance: Useful small optimization reference for density power functions.
- Decision: adaptable; version-specific
- 1.20.1 implications: Compare constant exponents and evaluation order with 1.20.1; retain signed fallback for unsupported exponents.
- Follow-up checks: Test negative/positive exponents, zero/one bases, NaNs, infinities and scalar/bulk failures.
- RustChunk implementation mapping: RustChunk power-density sampler.
- Evidence required before adoption: 1.20.1 power density function source and float-bit tests.
- Completion state: COMPLETE — 72 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 2317-byte size match the source tree.

### 1748. `crates/horizon-core/src/density/rounding.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4ac9bbe83d732363996c8c448a49f12b12fbda7a`
- Size: 3153 bytes; 98 logical lines
- Coverage: 1–98 (EOF)
- Purpose: Round/floor/ceil/truncate density operations with optional multiple sampler.
- Dependencies / cross-references: DensitySampler, RoundMode, exact buffer helpers and Java Math.round semantics.
- Implementation / logic: Handles integer and multiple-based rounding with literal-one fast path; preserves child evaluation and buffer rules.
- Findings: Float-to-int rounding and sampled-multiple behavior are subtle Java compatibility details.
- RustChunk relevance: Useful direct reference for 1.20.1 density rounding.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild exact RoundMode and multiple semantics from 1.20.1 density functions.
- Follow-up checks: Test half ties, negative values, NaN/infinity, signed zero, zero multiple and bulk sizing/failure order.
- RustChunk implementation mapping: RustChunk density rounding sampler.
- Evidence required before adoption: 1.20.1 DensityFunctions rounding source and Java edge-value fixtures.
- Completion state: COMPLETE — 98 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 3153-byte size match the source tree.

### 1749. `crates/horizon-core/src/density/selection.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eed6145ffad1933f4323b88e853dce37159ee35a`
- Size: 7704 bytes; 228 logical lines
- Coverage: 1–228 (EOF)
- Purpose: Range-choice, lerp and interval-branch density samplers.
- Dependencies / cross-references: DensitySampler, DensityValue, volume scratch buffers and interval threshold/branch arrays.
- Implementation / logic: Implements conditional branches, constant-aware interpolation and interval selection with bulk child evaluation and delayed output selection.
- Findings: Bulk branch evaluation order and threshold order are observable; this is not proof of target parity.
- RustChunk relevance: Useful reference for graph selection/interpolation organization.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Reconstruct exact 1.20.1 range-choice/lerp/threshold operators and branch evaluation order.
- Follow-up checks: Test inclusive/exclusive boundaries, NaNs, equal thresholds, constant endpoints, child errors and scratch bounds.
- RustChunk implementation mapping: RustChunk density selection samplers.
- Evidence required before adoption: 1.20.1 DensityFunctions selection/lerp sources and deterministic scalar/bulk fixtures.
- Completion state: COMPLETE — 228 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 7704-byte size match the source tree.

### 1750. `crates/horizon-core/src/density/slice.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3e7fa87a25b590f8e74f0c53c1a7370ca09057e6`
- Size: 4105 bytes; 118 logical lines
- Coverage: 1–118 (EOF)
- Purpose: Density sampler wrapper that fixes one or more world axes and broadcasts reduced-volume samples.
- Dependencies / cross-references: DensitySampler, DensityVolume, SamplerContext plane cache and fixed X/Y/Z/XZ coordinate keys.
- Implementation / logic: Pins selected coordinates, reduces sample volume dimensions, broadcasts child outputs and optionally reuses Y-fixed XZ planes.
- Findings: Plane cache is an optimization extension; reduced descriptors preserve original steps and need target validation.
- RustChunk relevance: Useful pattern for axis-dependent density graphs.
- Decision: adaptable; optional/future
- 1.20.1 implications: Implement only after 1.20.1 axis-dependency and sampler-volume semantics are verified; keep plane cache optional for preview.
- Follow-up checks: Test fixed axes, combined XZ slicing, reduced volume descriptors, plane keys, buffer tails and full-vs-sparse outputs.
- RustChunk implementation mapping: RustChunk density graph slice sampler and optional LOD plane cache.
- Evidence required before adoption: 1.20.1 density graph axis dependencies and NoiseChunk volume fixtures.
- Completion state: COMPLETE — 118 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 4105-byte size match the source tree.

### 1751. `crates/horizon-core/src/density/spline.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `14eaa0adf2a4eeb6652b4619b2010056cc31072e`
- Size: 13399 bytes; 344 logical lines
- Coverage: 1–344 (EOF)
- Purpose: Cubic spline density sampler with range and coordinate evaluation.
- Dependencies / cross-references: DensitySampler, FunctionId-backed coordinate samplers, spline point lists, Interval arithmetic and volume evaluation.
- Implementation / logic: Evaluates constant and multipoint splines, interpolates between control points, computes conservative value ranges and shares coordinate sampling during bulk evaluation.
- Findings: Spline point ordering, derivative behavior, NaN handling and compile-time coordinate caching are observable; pinned implementation belongs to the newer density system.
- RustChunk relevance: Useful architecture and regression reference for spline terrain density.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild spline evaluation and codec semantics from Minecraft 1.20.1 CubicSpline/DensityFunctions; do not reuse newer graph interfaces unchanged.
- Follow-up checks: Compare endpoint clamping, equal locations, derivative interpolation, float operation order, NaN and bulk coordinate reuse against 1.20.1.
- RustChunk implementation mapping: RustChunk cubic-spline density node and scalar/bulk tests.
- Evidence required before adoption: Mojang-mapped 1.20.1 CubicSpline source and deterministic spline fixtures.
- Completion state: COMPLETE — 344 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 13399-byte size match the source tree.

### 1752. `crates/horizon-core/src/density/surface.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `df437bc2be767725299953e5eeeb0c60522322ce`
- Size: 3177 bytes; 99 logical lines
- Coverage: 1–99 (EOF)
- Purpose: Finds the highest terrain surface crossing in a density sampler.
- Dependencies / cross-references: DensitySampler, DensityVolume, upper/lower bounds and sampler context.
- Implementation / logic: Searches down from an upper density boundary, samples the density crossing and exposes scalar plus single-Y-layer volume behavior.
- Findings: The implementation's integer rounding and search bounds are version-specific; it explicitly uses the pinned newer Mth behavior.
- RustChunk relevance: Potential reference for surface-height search and terrain preview.
- Decision: adaptable; version-specific
- 1.20.1 implications: Reconstruct surface search from 1.20.1 NoiseChunk/WorldgenRandomState callers and density rules; preserve target integer floor semantics.
- Follow-up checks: Test crossing/no-crossing, inclusive limits, negative heights, integer overflow and exact one-layer buffer requirements.
- RustChunk implementation mapping: RustChunk surface-height sampler for terrain generation.
- Evidence required before adoption: 1.20.1 surface-search call sites and trusted scalar/bulk height fixtures.
- Completion state: COMPLETE — 99 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 3177-byte size match the source tree.

### 1753. `crates/horizon-core/src/density/terrain_noise.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `908a0657621a80f10f7e2ddfdfb0a535cfe5bba9`
- Size: 7260 bytes; 220 logical lines
- Coverage: 1–220 (EOF)
- Purpose: Compiles shift-noise, blended-noise and distance-to-point density primitives.
- Dependencies / cross-references: Perlin/Normal noise stacks, seeded random factories, DensitySampler, DistanceMetric and graph compiler.
- Implementation / logic: Builds shifted noise variants, blended fBm samplers, distance metrics and octave-based terrain noise helpers.
- Findings: Noise octave ordering, random consumption and newer graph node variants are tied to the pinned target; exact output is seed-sensitive.
- RustChunk relevance: Core architectural reference for density noise and terrain shaping.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Implement only the 1.20.1 density noise functions and BlendedNoise algorithm, driven by the target-era random state.
- Follow-up checks: Compare octave frequencies/amplitudes, positional offsets, legacy RNG draws, distance metric formulas and scalar/bulk float bits.
- RustChunk implementation mapping: RustChunk 1.20.1 terrain-noise sampler family.
- Evidence required before adoption: 1.20.1 BlendedNoise/NoiseChunk/DensityFunctions source and seed-based oracle traces.
- Completion state: COMPLETE — 220 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 7260-byte size match the source tree.

### 1754. `crates/horizon-core/src/density/tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1c76399d0fb47cbf517885fb0786f5cabec5e698`
- Size: 33778 bytes; 1012 logical lines
- Coverage: 1–400, 401–800, 801–1012 (EOF)
- Purpose: Unit-test inventory for density sampler primitives, volume behavior, caching and error propagation.
- Dependencies / cross-references: DensitySampler implementations, DensityVolume, cache/slice/interpolation, noise and surface samplers.
- Implementation / logic: Exercises buffer sizing/tails, partial writes, cache reset/isolation, interpolation, slices, spline, surface and failure paths.
- Findings: Tests assert the pinned implementation's newer-version details; they were read only, not executed.
- RustChunk relevance: Useful regression checklist for RustChunk's density engine.
- Decision: undetermined; reference; tests only
- 1.20.1 implications: Translate applicable tests to the exact 1.20.1 function set and expected results; remove cases for newer-only sampler behavior.
- Follow-up checks: Build target-era test matrix for scalar/bulk parity, failed child writes, cache namespace, interpolation, spline and surface limits.
- RustChunk implementation mapping: RustChunk density unit and integration regression tests.
- Evidence required before adoption: 1.20.1 source-derived expected outputs and a runnable RustChunk test harness; no tests run.
- Completion state: COMPLETE — 1012 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 33778-byte size match the source tree; embedded tests were inspected only, not run.

### 1755. `crates/horizon-core/src/density/world_fields.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7709de8199ae68efd4271e0c40b9c964c44423d3`
- Size: 4521 bytes; 151 logical lines
- Coverage: 1–151 (EOF)
- Purpose: Binds world-dependent density inputs to a per-evaluation sampler context.
- Dependencies / cross-references: BlendAlpha, BlendOffset, Beardifier, DensityBlender trait, WorldFields and ContextBoundSampler.
- Implementation / logic: Stores and resolves named world fields, supplies defaults and routes density input through a blender interface.
- Findings: The file documents that the full blender data model/distance weighting is pending; this is an interface, not a complete Minecraft blending implementation.
- RustChunk relevance: Useful separation pattern for per-world context and density dependencies.
- Decision: adaptable; reference; incomplete
- 1.20.1 implications: Build target-compatible blend alpha/offset/beardifier bindings from 1.20.1 blending data; keep the incomplete blender abstraction separate from authoritative terrain logic.
- Follow-up checks: Verify default field values, context replacement, cache invalidation, blending coordinates and missing-field behavior.
- RustChunk implementation mapping: RustChunk world-field context and an isolated blending-provider interface.
- Evidence required before adoption: 1.20.1 Blender/BlendingData source, blending test worlds and complete target algorithm.
- Completion state: COMPLETE — 151 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 4521-byte size match the source tree.

### 1756. `crates/horizon-core/src/fast_hash.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9382adbb8dab55a9fe8378972441ede30a30050b`
- Size: 2537 bytes; 88 logical lines
- Coverage: 1–88 (EOF)
- Purpose: Fast non-cryptographic hash builder and map/set aliases for hot integer-keyed collections.
- Dependencies / cross-references: Fx-style hash state, integer hashing and standard HashMap/HashSet interfaces.
- Implementation / logic: Implements a lightweight hasher and type aliases used for block/chunk/tick keys.
- Findings: Not a cryptographic hash; performance and collision behavior are implementation choices, and iteration order must not be treated as stable.
- RustChunk relevance: Potential performance-oriented collection reference for RustChunk hot paths.
- Decision: adaptable; optional
- 1.20.1 implications: Use a fast hasher only for non-adversarial internal keys after profiling; retain robust hashing at untrusted input boundaries.
- Follow-up checks: Benchmark realistic chunk/tick workloads and test collision handling, determinism assumptions and denial-of-service exposure.
- RustChunk implementation mapping: Optional RustChunk internal fast-map aliases.
- Evidence required before adoption: Threat model, benchmark corpus and review of every key that can be influenced by server/mod data.
- Completion state: COMPLETE — 88 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 2537-byte size match the source tree.

### 1757. `crates/horizon-core/src/geometry/border.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `92b865927235040277db5cd14dd2a65311613020`
- Size: 9458 bytes; 270 logical lines
- Coverage: 1–270 (EOF)
- Purpose: World-border geometry, movement transitions and collision shape calculations.
- Dependencies / cross-references: AABB/Vec3 geometry, border center/size, lerp timing and collision queries.
- Implementation / logic: Computes border bounds, current interpolated size, distance-to-border and collision restrictions around the player.
- Findings: This is the geometric portion only; persistence, packet synchronization, damage, warnings and listeners live elsewhere. Transition edge cases need target parity.
- RustChunk relevance: Useful reference for movement collision and world-border rendering.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild border geometry and transition semantics from 1.20.1 WorldBorder; integrate separately with protocol sync and gameplay damage logic.
- Follow-up checks: Test shrinking/growing borders, zero-duration size changes, negative coordinates, boundary inclusivity and player collision at corners.
- RustChunk implementation mapping: RustChunk world-border geometry service, separate from packet/state handling.
- Evidence required before adoption: 1.20.1 WorldBorder source, packet captures and edge-case transition traces.
- Completion state: COMPLETE — 270 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 9458-byte size match the source tree.

### 1758. `crates/horizon-core/src/geometry/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `016bda36d2843998c684055fc4878b24881836f8`
- Size: 22670 bytes; 714 logical lines
- Coverage: 1–400, 401–714 (EOF)
- Purpose: Core vector, direction, AABB, ray and block-hit geometry primitives.
- Dependencies / cross-references: Vec3, Axis/Direction, AABB, BlockPos, block shape clipping and Java-compatible floating-point helpers.
- Implementation / logic: Implements vector operations, rotations, axis ordering, box intersections, ray clipping, hit-face resolution and related numeric edge behavior.
- Findings: Geometry and clipping order affect movement/mining; code contains newer target assumptions and machine/float behavior requires parity.
- RustChunk relevance: High relevance to player physics, raycasting and block interaction.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Reconstruct each primitive against Mojang-mapped 1.20.1 Vec3/AABB/Direction/BlockHitResult and preserve operation order.
- Follow-up checks: Test parallel rays, face ties, touching boxes, signed zero/NaN, negative coordinates, degenerate boxes and hit-face tie-breaking.
- RustChunk implementation mapping: RustChunk geometry kernel for raycasting, collision and block selection.
- Evidence required before adoption: 1.20.1 geometry source and deterministic edge-case/ray traces.
- Completion state: COMPLETE — 714 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 22670-byte size match the source tree.

### 1759. `crates/horizon-core/src/geometry/movement.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `21ed18302ce729502c4a02075f2f77970f2dce22`
- Size: 20319 bytes; 554 logical lines
- Coverage: 1–400, 401–554 (EOF)
- Purpose: Entity movement collision resolution and step-up candidate selection.
- Dependencies / cross-references: AABB, VoxelShape, block colliders, movement vectors and axis ordering.
- Implementation / logic: Resolves movement against colliders, evaluates axis clipping and step-up alternatives, and handles collision movement branches.
- Findings: The internal two-million-case randomized test was inspected but not run; collision ordering and newer implementation shortcuts need target comparison.
- RustChunk relevance: High relevance to player movement, stepping and block collision.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Rebuild movement resolver from 1.20.1 Entity/CollisionGetter/Shapes sources; preserve exact candidate ordering and tie behavior.
- Follow-up checks: Compare axis clipping, step heights, grounded/fall conditions, empty colliders, negative motion and deterministic random test seeds.
- RustChunk implementation mapping: RustChunk entity collision/movement resolver and property-test suite.
- Evidence required before adoption: 1.20.1 Entity.move/Shapes sources and reference movement traces; randomized tests not run.
- Completion state: COMPLETE — 554 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 20319-byte size match the source tree; embedded tests were inspected only, not run.

### 1760. `crates/horizon-core/src/geometry/shape.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a2aa95dd8eb017e02013479ee4ef4ab97ea0aeb1`
- Size: 22282 bytes; 607 logical lines
- Coverage: 1–400, 401–607 (EOF)
- Purpose: VoxelShape representation, occupancy, decomposition and collision queries.
- Dependencies / cross-references: Discrete voxel grids, AABB decomposition, coordinate lists, boolean shape joins and ray clipping.
- Implementation / logic: Stores shape occupancy, derives bounding boxes, enumerates boxes and performs collision/axis-distance and clipping operations.
- Findings: Grid capacity and identity fast paths are implementation constraints; all query/shape ordering must be verified for 1.20.1.
- RustChunk relevance: High relevance to block collision and selection, especially modded block shapes.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Port the 1.20.1 Shapes/VoxelShape/DiscreteVoxelShape semantics and limits rather than assuming the newer occupancy grid matches.
- Follow-up checks: Test empty/infinite shapes, coordinate quantization, unions/intersections, face occlusion, box decomposition, clipping and large modded shapes.
- RustChunk implementation mapping: RustChunk VoxelShape and collision-shape engine.
- Evidence required before adoption: 1.20.1 Shapes/BitSetDiscreteVoxelShape sources and modded shape fixtures.
- Completion state: COMPLETE — 607 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 22282-byte size match the source tree; embedded tests were inspected only, not run.

### 1761. `crates/horizon-core/src/geometry/shape/create.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `312e030b3d5ed83d52bc90f46ec2a7dfc390c2c5`
- Size: 5418 bytes; 147 logical lines
- Coverage: 1–147 (EOF)
- Purpose: Constructs voxel shapes from AABBs and joins/optimizes them.
- Dependencies / cross-references: AABB coordinates, discrete shape grid, face masks and VoxelShape boolean join.
- Implementation / logic: Quantizes box boundaries to the discrete grid, builds occupancy and applies optimized shape joining.
- Findings: Thin-box acceptance, coordinate quantization and optimization identity are version-sensitive.
- RustChunk relevance: Useful focused reference for block-shape construction.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild Shapes.create/from/AABB conversion and optimization from 1.20.1 source; do not carry newer quantization assumptions blindly.
- Follow-up checks: Test fractional and one-eighth boundaries, zero-thickness boxes, inflate, negative extents and shape joins.
- RustChunk implementation mapping: RustChunk AABB-to-VoxelShape constructor.
- Evidence required before adoption: 1.20.1 Shapes source and known block collision-box fixtures.
- Completion state: COMPLETE — 147 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 5418-byte size match the source tree.

### 1762. `crates/horizon-core/src/geometry/shape/face.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5a615019721974a59d15d0b14b942fd0f7b049a9`
- Size: 6451 bytes; 168 logical lines
- Coverage: 1–168 (EOF)
- Purpose: Computes and caches voxel-shape faces for occlusion and contact queries.
- Dependencies / cross-references: VoxelShape occupancy, face direction, discrete masks and lazy/transient cache state.
- Implementation / logic: Derives a face mask from the occupancy grid and caches selected results, with a transient path to avoid retaining large data.
- Findings: Cache lifetime and invalidation behavior are coupled to shape immutability and the pinned implementation.
- RustChunk relevance: Useful reference for face occlusion and block-face comparisons.
- Decision: adaptable; version-specific
- 1.20.1 implications: Reconstruct 1.20.1 FaceShape/Shapes face extraction and cache semantics; avoid introducing persistent cache leaks.
- Follow-up checks: Test all six faces, empty/full shapes, transient behavior, equality, cache reuse and memory retention.
- RustChunk implementation mapping: RustChunk voxel-face mask helper and occlusion cache.
- Evidence required before adoption: 1.20.1 FaceShape source and block occlusion fixtures.
- Completion state: COMPLETE — 168 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 6451-byte size match the source tree.

### 1763. `crates/horizon-core/src/geometry/shape/join.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `261c33802de4a478ab3531fa082298e09180cfff`
- Size: 11960 bytes; 361 logical lines
- Coverage: 1–361 (EOF)
- Purpose: Performs boolean union/intersection/only-first shape operations.
- Dependencies / cross-references: VoxelShape, coordinate mergers, discrete occupancy and boolean join truth tables.
- Implementation / logic: Merges axis coordinate sets, evaluates the requested boolean operation across cells and constructs a result shape.
- Findings: Unbounded-shape shortcuts and coordinate merge edge cases can affect correctness and require exact target parity.
- RustChunk relevance: Core reference for collision/occlusion shape composition.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Port target 1.20.1 BooleanOp/Shapes.join and coordinate merger logic; preserve truth table and merger ordering.
- Follow-up checks: Test empty/infinite cases, duplicate coordinates, one-cell overlap, all boolean operations and result simplification.
- RustChunk implementation mapping: RustChunk VoxelShape boolean composition engine.
- Evidence required before adoption: 1.20.1 BooleanOp/IndirectMerger sources and differential shape fixtures.
- Completion state: COMPLETE — 361 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 11960-byte size match the source tree.

### 1764. `crates/horizon-core/src/geometry/sweep.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e90ee56349670a6998d47655b4edd414b2317fa9`
- Size: 7625 bytes; 224 logical lines
- Coverage: 1–224 (EOF)
- Purpose: Visits blocks intersected by a swept movement volume.
- Dependencies / cross-references: AABB/Vec3, block traversal, chunk/block position math and bounded deduplication.
- Implementation / logic: Enumerates the movement path's block candidates, tracks already visited blocks and enforces work limits.
- Findings: This is candidate visitation only; collision resolution and block effects are handled by other layers.
- RustChunk relevance: Useful to organize broad-phase collision candidate gathering.
- Decision: adaptable; reference
- 1.20.1 implications: Implement the 1.20.1 collision-sweep visitation independently from actual collision and block update effects.
- Follow-up checks: Test corner/edge crossings, stationary axes, duplicate block visits, large movements and work-limit behavior.
- RustChunk implementation mapping: RustChunk broad-phase movement sweep iterator.
- Evidence required before adoption: 1.20.1 BlockCollisions source and movement-volume test fixtures.
- Completion state: COMPLETE — 224 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 7625-byte size match the source tree.

### 1765. `crates/horizon-core/src/geometry/traversal.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6dee29f6ec6e39d804416771c44defa2d1ba7697`
- Size: 4706 bytes; 155 logical lines
- Coverage: 1–155 (EOF)
- Purpose: Traverses voxel blocks along a ray in deterministic order.
- Dependencies / cross-references: Ray direction, BlockPos, voxel boundary stepping and block visitor callback.
- Implementation / logic: Steps through crossed cells with explicit corner tie-breaking and a maximum visitation count.
- Findings: Tie resolution and traversal cap are observable, and must match the 1.20.1 clip traversal before adoption.
- RustChunk relevance: Useful for raycast traversal and block targeting.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild the target 1.20.1 BlockGetter.clip/ray traversal and face tie rules; keep visitation caps only if behavior-compatible.
- Follow-up checks: Test axis-aligned rays, exact edge/corner crossings, negative directions, zero components and max visit boundaries.
- RustChunk implementation mapping: RustChunk voxel ray traversal helper.
- Evidence required before adoption: 1.20.1 BlockGetter.clip and traversal oracle fixtures.
- Completion state: COMPLETE — 155 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 4706-byte size match the source tree.

### 1766. `crates/horizon-core/src/geometry/trig.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f7b2484b5f30cba7cf5155c59aef9ff76bfd20b3`
- Size: 13325 bytes; 384 logical lines
- Coverage: 1–384 (EOF)
- Purpose: Fast trigonometric lookup and fdlibm atan2 support.
- Dependencies / cross-references: Sine table, angle reduction and Java StrictMath-compatible fdlibm routines.
- Implementation / logic: Provides table-backed sine and fdlibm atan2 behavior for exact angle calculations.
- Findings: The tables and expected bit patterns are tied to the pinned runtime; do not infer Android ART/HotSpot results from them.
- RustChunk relevance: Useful only where exact angle compatibility is required.
- Decision: rewrite required; architecture-specific
- 1.20.1 implications: Use the target Java 17/Android ARM runtime behavior or a proven 1.20.1-compatible math implementation; verify the chosen path on ARM64.
- Follow-up checks: Compare trigonometric bit patterns, quadrants, signed zero, huge angles and boundary rounding on the Galaxy-class ARM64 target.
- RustChunk implementation mapping: RustChunk target-runtime trig compatibility layer if required.
- Evidence required before adoption: 1.20.1 Java runtime math expectations and ARM64 device differential vectors.
- Completion state: COMPLETE — 384 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 13325-byte size match the source tree.

### 1767. `crates/horizon-core/src/jmath.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3ffe3929b99c4ede8081313987222861c7e50851`
- Size: 22665 bytes; 707 logical lines
- Coverage: 1–400, 401–707 (EOF)
- Purpose: Implements selected java.lang.Math/StrictMath behavior for the pinned HotSpot runtime.
- Dependencies / cross-references: fdlibm routines, AArch64 FMA trig kernels, platform libm and geometry trig helpers.
- Implementation / logic: Dispatches different math paths by architecture for sin/cos/tan/exp/log/pow/cbrt/atan2/hypot, with HotSpot-specific AArch64 code.
- Findings: Comments explicitly describe HotSpot x86-64/AArch64 differences and Java 25 behavior; the exact platform/runtime target is not Android ART by default.
- RustChunk relevance: Important cautionary reference for float compatibility, not a portable drop-in implementation.
- Decision: rewrite required; architecture-specific; version-specific
- 1.20.1 implications: Define math parity for the actual Minecraft 1.20.1 Java runtime on Android ARM64; do not copy HotSpot 25 paths without device evidence.
- Follow-up checks: Run differential vectors for Java 17/target runtime versus Rust on Android ARM64, including FMA, NaN, subnormal, huge-angle and pow/log edges.
- RustChunk implementation mapping: RustChunk Android ARM64 math adapter only if differential testing proves necessary.
- Evidence required before adoption: Target Minecraft Java runtime and device-specific oracle outputs; source tests were inspected only.
- Completion state: COMPLETE — 707 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 22665-byte size match the source tree; embedded tests were inspected only, not run.

### 1768. `crates/horizon-core/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `169f537e9d7186206c2e4d64d427950f66b593d6`
- Size: 615 bytes; 26 logical lines
- Coverage: 1–26 (EOF)
- Purpose: Exports the simulation primitive modules of horizon-core.
- Dependencies / cross-references: Biome, carving, density, geometry, noise, palette, positions, random, storage, text, ticks and version modules.
- Implementation / logic: Declares public crate modules and forbids unsafe Rust.
- Findings: The crate explicitly states it is not a playable game and only a limited subset of Vanilla has differential checks; full terrain generation is still absent elsewhere.
- RustChunk relevance: Useful evidence of current upstream scope and modular boundaries.
- Decision: reference; incomplete; not a playable client
- 1.20.1 implications: Do not mistake this crate for a complete game engine or use its module list as proof of 1.20.1 Forge support.
- Follow-up checks: Map each required client subsystem to implemented code and identify missing protocol, rendering, entity, inventory, resource and mod-loader systems.
- RustChunk implementation mapping: RustChunk gap analysis and module boundary planning.
- Evidence required before adoption: Full 1.20.1/Forge feature inventory plus end-to-end playability criteria.
- Completion state: COMPLETE — 26 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 615-byte size match the source tree.

### 1769. `crates/horizon-core/src/noise/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e715b10f0c98f2d8330fd31b8a41326893524a73`
- Size: 10722 bytes; 305 logical lines
- Coverage: 1–305 (EOF)
- Purpose: Shared Perlin/Simplex/gradient table and density-volume noise utilities.
- Dependencies / cross-references: LegacyRandom, GradientTable, Perlin/Simplex/NormalNoise and DensityVolume.
- Implementation / logic: Builds gradient permutation tables, wraps coordinates, supplies interpolation helpers and baseline noise tests.
- Findings: File explicitly targets pinned Java 26.3 and states complete density graphs/terrain generation remain absent.
- RustChunk relevance: Useful noise architecture and seed-trace reference, not direct 1.20.1 implementation.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Rebuild noise generation and helpers from Minecraft 1.20.1 Java sources and its exact random stream.
- Follow-up checks: Compare gradient permutation, random consumption, wrap boundaries, float interpolation order and deterministic noise vectors.
- RustChunk implementation mapping: RustChunk 1.20.1 noise core.
- Evidence required before adoption: 1.20.1 PerlinNoise/SimplexNoise sources and seed-specific expected bit patterns.
- Completion state: COMPLETE — 305 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 10722-byte size match the source tree.

### 1770. `crates/horizon-core/src/noise/normal.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1a0b8b85317eb10afecdb2042227c5cca1689225`
- Size: 13116 bytes; 387 logical lines
- Coverage: 1–387 (EOF)
- Purpose: NormalNoise octave builder, normalization and legacy FBM construction.
- Dependencies / cross-references: PerlinNoise, NoiseStack, positional random factories and amplitude modifiers.
- Implementation / logic: Builds selected octave layers, computes amplitude normalization and implements a legacy FBM path with explicit random-consumption order.
- Findings: Builder permits up to 4096 octaves while the source comment notes Vanilla's data codec allows 32; legacy behavior is constrained and version-specific.
- RustChunk relevance: High-value reference for worldgen octave logic and RNG auditing.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Implement 1.20.1 NormalNoise/BlendedNoise octave selection and normalization from target Java source; use the target codec bounds.
- Follow-up checks: Compare amplitude modifiers, absent/zero octaves, positive/negative octave ranges, legacy draw counts and normalized range.
- RustChunk implementation mapping: RustChunk 1.20.1 NormalNoise builder and deterministic RNG tests.
- Evidence required before adoption: Mojang-mapped 1.20.1 NormalNoise source, codec limits and seed-based traces.
- Completion state: COMPLETE — 387 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 13116-byte size match the source tree.

### 1771. `crates/horizon-core/src/noise/perlin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `130238c4157a82ecbec2c7ccdf54a9edde043229`
- Size: 15287 bytes; 432 logical lines
- Coverage: 1–400, 401–432 (EOF)
- Purpose: Perlin and smeared Perlin sampling with derivatives and bulk volume accumulation.
- Dependencies / cross-references: GradientTable, random source, DensityVolume, coordinate wrapping and interpolation functions.
- Implementation / logic: Computes gradients/corner values, noise derivatives and batched Y-volume samples; includes bit-identical scalar/bulk tests.
- Findings: Optimization batches four Ys within a cell; matching floats does not prove 1.20.1 seed or arithmetic parity. Tests were not run.
- RustChunk relevance: High relevance to terrain noise and chunk generation.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Port the exact 1.20.1 Perlin/SmearedNoise implementation first; enable batching only after scalar oracle parity.
- Follow-up checks: Check offsets/permutations, negative coordinates, Y fudge, cell-boundary batches, volume tail handling and scalar/bulk bit equality.
- RustChunk implementation mapping: RustChunk Perlin noise sampler and optional verified bulk specialization.
- Evidence required before adoption: 1.20.1 PerlinNoise source and differential output fixtures; tests not executed.
- Completion state: COMPLETE — 432 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 15287-byte size match the source tree; embedded tests were inspected only, not run.

### 1772. `crates/horizon-core/src/noise/simplex.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8dcb56f85efda2fc5b6ac46d2994996037d0b25c`
- Size: 4322 bytes; 121 logical lines
- Coverage: 1–121 (EOF)
- Purpose: Simplex noise sampler for two- and three-dimensional coordinates.
- Dependencies / cross-references: GradientTable, permutation table, floor conversion and corner attenuation.
- Implementation / logic: Computes skewed-grid simplex corners, gradient lookup, attenuation and normalized outputs.
- Findings: Seeded permutation order and signed-coordinate arithmetic are version-sensitive.
- RustChunk relevance: Potential reference for biome/End noise only after target verification.
- Decision: adaptable; version-specific
- 1.20.1 implications: Rebuild the target 1.20.1 SimplexNoise algorithm and call sites; avoid assuming it is used identically in target worldgen.
- Follow-up checks: Compare permutation table, corner tie ordering, negative coordinates, NaNs and exact seed vectors.
- RustChunk implementation mapping: RustChunk Simplex noise utility if required by 1.20.1.
- Evidence required before adoption: 1.20.1 SimplexNoise source and fixed-seed output corpus.
- Completion state: COMPLETE — 121 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 4322-byte size match the source tree.

### 1773. `crates/horizon-core/src/noise/stack.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9cfe093d5ede317bf0f123a3a860e927f72e2f0d`
- Size: 7224 bytes; 233 logical lines
- Coverage: 1–233 (EOF)
- Purpose: Composable ordered stack of noise layers and scalar/bulk evaluation.
- Dependencies / cross-references: Noise trait, Perlin/Simplex implementations, layer frequency/amplitude and DensityVolume.
- Implementation / logic: Aggregates layers in insertion order, supports flattened nested stacks and sums each layer's volume contribution.
- Findings: Layer ordering and float accumulation are observable; range behavior also depends on signed zero/infinity.
- RustChunk relevance: Useful abstraction for multi-octave noise composition.
- Decision: adaptable; version-specific
- 1.20.1 implications: Recreate target 1.20.1 noise layering with the same accumulation order and normalization.
- Follow-up checks: Test empty/zero-amplitude layers, nested flattening, infinities/NaNs, range bounds and scalar/bulk sum order.
- RustChunk implementation mapping: RustChunk NoiseStack abstraction.
- Evidence required before adoption: 1.20.1 NormalNoise composition source and bit-level layer fixtures.
- Completion state: COMPLETE — 233 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 7224-byte size match the source tree.

### 1774. `crates/horizon-core/src/noise/volume.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d0362ee1bd386a1ee025f4ea20ab0f2ce8ce4095`
- Size: 3460 bytes; 94 logical lines
- Coverage: 1–94 (EOF)
- Purpose: Validated strided density-volume descriptor and flat buffer indexing.
- Dependencies / cross-references: Axis, dimensions, minimum block coordinates, coordinate steps and Java wrapping arithmetic.
- Implementation / logic: Bounds dimensions/steps and total size, maps coordinates with wrapping i32 math and uses Z-X-Y storage with Y innermost.
- Findings: The chosen layout and i32 maximum size must match each target sampler's buffer contract.
- RustChunk relevance: Useful common descriptor for density volume calculations.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Define RustChunk's 1.20.1 chunk-density lattice layout and maximum allocation limits independently from this newer implementation.
- Follow-up checks: Test dimensions/steps validation, negative coordinates, index mapping, overflow and buffer-tail behavior.
- RustChunk implementation mapping: RustChunk DensityVolume descriptor.
- Evidence required before adoption: 1.20.1 NoiseChunk volume ordering and deterministic index fixtures.
- Completion state: COMPLETE — 94 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 3460-byte size match the source tree.

### 1775. `crates/horizon-core/src/paletted.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5b5b3be1ee020d9db973b0a3ac9f8e8ed7237cce`
- Size: 50805 bytes; 1306 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1306 (EOF)
- Purpose: Resident PalettedContainer for block-state and biome values with wire and saved encodings.
- Dependencies / cross-references: Strategy thresholds, palette variants, packed storage, VarInt-facing wire data and palette ID lookup.
- Implementation / logic: Supports single/list/global palette states, resize, set/get, pack/unpack, wire load/save, palette queries and storage round trips.
- Findings: This file explicitly targets Java 26.3; block/biome bit thresholds and transitions must not be copied directly to 1.20.1. Embedded tests and ignored microbenchmarks were inspected only, not run.
- RustChunk relevance: Very high relevance to chunk sections and large modded registries, but version-specific format is critical.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Reconstruct 1.20.1 PalettedContainer strategies, bit widths, wire/NBT layout and palette resize behavior from exact target sources.
- Follow-up checks: Test palette transition thresholds, entries spanning long words, registry IDs >65k, wire round trips, malformed data, stale palette entries and save/load parity.
- RustChunk implementation mapping: RustChunk 1.20.1 block/biome palettes and PackedStorage.
- Evidence required before adoption: 1.20.1 PalettedContainer/PalettedContainer.Strategy/BitStorage sources and Forge registry-size fixtures.
- Completion state: COMPLETE — 1306 logical lines read through EOF in the stated contiguous ranges; pinned blob SHA and 50805-byte size match the source tree; embedded tests were inspected only, not run.
