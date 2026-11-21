# RustChunk Starling Audit Ledger — Entries 1201–1400

- Source repository/ref: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1201–1400 (200 new unique source files; one shared Ledger for the complete stage).
- Current progress: entries 1201–1300 committed (100/200); the first two 50-file reporting sections are complete.
- Next section: 1301–1350, split into checkpoints 1301–1325 and 1326–1350; do not start until explicit user authorization.
- Current cadence: four 50-file reporting sections per 200-file stage; two exact 25-file checkpoints per 50.
- Persistence rule: validate and commit all five audit documents together after every exact 25-file checkpoint, then re-fetch and verify them from GitHub.
- Reporting rule: report and stop after each 50-file section; no approval pause between the two 25-file checkpoints inside an authorized 50.
- Backup rule: next backup only after entries 1001–2000 are all committed and globally verified.
- Each entry contains the 15 required fields defined in `AUDIT_PROTOCOL.md`.
- Long files are read completely in consecutive sections of at most 400 logical lines. Tests were inspected, not executed.

### 1201. `crates/horizon-client/src/game/events.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `034c79c9073147c62fec454765b9fa95676ab522`
- Size: 12482 bytes; 362 logical lines
- Coverage: Complete text file reviewed, lines 1–362.
- Purpose: Implements a Horizon agent-facing event feed and `wait-for` control: sequence-numbered snapshots/deltas for world, screen, chat, inventory and dimension changes, backed by a bounded event ring.
- Dependencies / cross-references: Event producers across Game/UI/network/inventory state; event serialization and parser for wait filters/timeouts; Horizon control/automation command surface.
- Implementation / logic: Collects state changes into a bounded queue, assigns sequence IDs, exposes filtered event reads and deferred waits with timeout handling; it is an observability/automation API rather than the Minecraft play protocol.
- Findings: Entire file is agent-control specific and not needed for vanilla play. `parse_wait` accepts floating-point timeout values; a parsed NaN can survive clamp-like bounds and may later reach `Duration::from_secs_f64`, which can panic. This is a potential robustness defect requiring caller-path confirmation, not a reproduced crash.
- RustChunk relevance: No baseline game-engine or 1.20.1 protocol requirement. At most, its bounded event-feed model could inform a separately sandboxed automation/diagnostic API.
- Decision: **unnecessary; optional/future — Keep the Horizon agent event channel out of the 1.20.1 baseline.**
- 1.20.1 implications: Do not add the Horizon event protocol or commands to the 1.20.1 client. If RustChunk later adds agent automation, define a distinct permission, privacy, bounded-queue and finite-timeout contract.
- Follow-up checks: Trace all wait timeout parsing callers; reject non-finite values before Duration conversion; bound outstanding waits and event payload sizes; keep automation disabled by default.
- RustChunk implementation mapping: No core mapping. Possible future `diagnostics/agent_events.rs`, not created.
- Evidence required before adoption: Threat model, explicit user opt-in, bounded queue/backpressure tests, finite timeout parser tests, privacy review and Android resource measurements.
- Completion state: COMPLETE — 362 logical lines read to EOF; pinned blob SHA and 12482-byte tree size match; no tests or runtime control API were executed.

### 1202. `crates/horizon-client/src/game/execution_role.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e8892dd471d2a0a0021441c9a8f5641a8e4e4fbd`
- Size: 865 bytes; 24 logical lines
- Coverage: Complete text file reviewed, lines 1–24.
- Purpose: Defines an execution-role enum separating ordinary client, integrated-server and dedicated-server responsibilities.
- Dependencies / cross-references: Game lifecycle, world/session ownership, client presentation and server-side persistence/command execution.
- Implementation / logic: Provides a compact role classification used to decide which responsibilities are active in a given runtime.
- Findings: The distinction between client and integrated server is valuable for singleplayer ownership; dedicated-server role is outside the Android client's immediate target. Role labels alone do not establish correct authority boundaries.
- RustChunk relevance: Useful architectural seam for a RustChunk client that may host an integrated 1.20.1 server in singleplayer while also connecting to external servers.
- Decision: **adaptable; reference — Reuse only the role-separation concept; define RustChunk-owned lifecycle contracts.**
- 1.20.1 implications: Model remote client, integrated server and any future headless/server runtime explicitly, but align authority and save ownership to 1.20.1/Forge. Keep client-side prediction separate from authoritative world mutation.
- Follow-up checks: Trace every write to world/player state through the role model; test singleplayer save ownership, remote client restrictions, pause/resume and disconnect cleanup.
- RustChunk implementation mapping: Potential `runtime/execution_role.rs` or session-capability model, not created.
- Evidence required before adoption: Lifecycle diagrams, authoritative mutation matrix, save/reload tests and remote-vs-integrated packet traces.
- Completion state: COMPLETE — 24 logical lines read to EOF; pinned blob SHA and 865-byte tree size match; no code or runtime role tests were executed.

### 1203. `crates/horizon-client/src/game/explosion.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1ef3a557dab0866fbad28bc7aaedd008565cadaf`
- Size: 58320 bytes; 1416 logical lines
- Coverage: Complete text file reviewed, lines 1–1416.
- Purpose: Implements explosions: ray-sampled block exposure/strength, block destruction and drops, entity damage/knockback, fire placement, TNT chaining and special explosion policies.
- Dependencies / cross-references: World/block collision and registries, entities/mobs/players, loot and item drops, RNG, block entities, damage sources, fire, commands, chunk/world mutation and optional LAN/native features.
- Implementation / logic: Samples outward rays from a discretized cube, tracks affected blocks and exposure, applies damage/knockback to entities, performs block interaction/drop decay, handles fire and chained explosive entities, and dispatches special cases.
- Findings: Explosion simulation is a core vanilla mechanic, but the implementation is intertwined with current Starling world types, modern damage/item data, LAN guest attribution, native hooks and later features such as Creaking hearts/cushions. The ray set/order and RNG sequence are compatibility-sensitive; visual similarity is insufficient.
- RustChunk relevance: Essential for TNT, creepers, beds/anchors, ghast/fireball behavior and other 1.20.1 gameplay. Port as a versioned subsystem, not as a direct file copy.
- Decision: **essential; rewrite required; version-specific — Explosion behavior is a core mechanic but requires a complete target-version rewrite.**
- 1.20.1 implications: Reconstruct 1.20.1 explosion radius, ray sampling, block interaction/drop decay, entity exposure, damage attribution, fire, source/explosion types and NBT against the target Java version. Strip later Creaking/cushion/native/LAN-only paths.
- Follow-up checks: Differentially compare deterministic block sets, entity damage/knockback, occlusion/exposure, drops, fire, chained TNT, source attribution and save/reload; verify seeded Java RNG iteration order.
- RustChunk implementation mapping: Proposed `game/explosion/{sampling,exposure,damage,blocks,fire,chain}_1_20_1.rs`, not created.
- Evidence required before adoption: Pinned 1.20.1 Java source/mappings, deterministic world fixtures/seeds, block and entity snapshots, drop/RNG traces, packet captures and Forge event behavior.
- Completion state: COMPLETE — 1416 logical lines read in consecutive chunks 1–400, 401–600, 601–800, 801–1000, 1001–1200 and 1201–1416; pinned SHA and 58320-byte tree size match; embedded tests inspected, not run.

### 1204. `crates/horizon-client/src/game/extras.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a66d0a619ad748d7c2d5d854a4bee0720bcaec8b`
- Size: 11167 bytes; 277 logical lines
- Coverage: Complete text file reviewed, lines 1–277.
- Purpose: Implements Horizon Extras client features such as minimap, zoom, reconnect helpers, FPS/CPS display and native mod frame/click hooks.
- Dependencies / cross-references: Game HUD, camera/zoom, settings, world/minimap sampling, input and Horizon native-mod callbacks.
- Implementation / logic: Maintains optional extra state, draws HUD elements and invokes registered extension callbacks; minimap work is constrained by a small per-frame budget.
- Findings: This is a collection of optional launcher/client conveniences, not vanilla 1.20.1 gameplay. It couples UI, settings and Horizon extension APIs and would add maintenance/performance cost if imported wholesale.
- RustChunk relevance: Exclude from RustChunk's vanilla/Forge baseline. Individual ideas such as zoom or FPS display can be designed independently after core rendering and input work.
- Decision: **unnecessary; optional/future — These are optional Horizon convenience features, not vanilla game requirements.**
- 1.20.1 implications: No protocol or game-mechanics dependency. Implement optional HUD features behind independent settings and measure their CPU/GPU cost on Android.
- Follow-up checks: If selected later, split each feature, audit privacy/network behavior of reconnect, and profile minimap/overlay frame time and battery impact.
- RustChunk implementation mapping: No baseline mapping. Possible optional `ui/extras/` modules after core parity; not created.
- Evidence required before adoption: Independent feature specs, user controls, privacy review for reconnect, HUD golden tests and Android frame-time/power measurements.
- Completion state: COMPLETE — 277 logical lines read to EOF; pinned SHA and 11167-byte tree size match; no UI or performance tests were run.

### 1205. `crates/horizon-client/src/game/falling_damage.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1f3a2055a4fc2214eab75afb0b3a1031e8c3f7d6`
- Size: 7175 bytes; 182 logical lines
- Coverage: Complete text file reviewed, lines 1–182.
- Purpose: Implements landing damage and block impacts for falling blocks, including entity/player damage and special handling for anvils and pointed dripstone.
- Dependencies / cross-references: Falling block entities, collision/landing state, entity damage, block state/loot, game rules, particles/sounds and custom LAN synchronization.
- Implementation / logic: Determines impact strength and affected targets on landing, applies fall-related damage/wear, updates block/entity state and emits impact effects.
- Findings: The landing-damage domain is target-relevant, but this snapshot includes later/non-vanilla sulfur-spike behavior and custom transient-block/LAN synchronization. Target item/block tags and damage APIs must not be copied from this snapshot.
- RustChunk relevance: Essential gameplay domain for falling anvils, stalactites and falling-block interactions; a reduced 1.20.1 implementation is appropriate.
- Decision: **essential; rewrite required; version-specific — Falling-impact mechanics are target-relevant but require target rules and removal of later additions.**
- 1.20.1 implications: Use 1.20.1 falling-block landing rules, anvil damage/degradation chance, pointed-dripstone collision/damage and target damage sources. Exclude sulfur spikes and custom LAN packet formats.
- Follow-up checks: Compare fall distance thresholds, damage caps, anvil degradation RNG, dripstone collision, entity/player damage, game-rule behavior and block placement/drop outcomes.
- RustChunk implementation mapping: Proposed `game/falling_damage_1_20_1.rs` integrated with target falling-block/entity systems, not created.
- Evidence required before adoption: Java 1.20.1 mappings/source, deterministic landing fixtures, RNG traces, damage packets, block-state and drop snapshots.
- Completion state: COMPLETE — 182 logical lines read to EOF; pinned SHA and 7175-byte tree size match; no tests were run.

### 1206. `crates/horizon-client/src/game/falling_trees.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a9b4eadd016f8b783f89a5201b0e97f00a5a28a9`
- Size: 9833 bytes; 243 logical lines
- Coverage: Complete text file reviewed, lines 1–243.
- Purpose: Simulates Create kinetic-saw falling-tree structures as a custom moving group of blocks, including persistence, rotation and rendering.
- Dependencies / cross-references: Create kinetic saw/kinetics integration, block lists and block states, NBT, collision and custom falling-tree renderer.
- Implementation / logic: Captures a tree's blocks, stores them in a moving structure, updates motion/orientation and renders the grouped geometry while preserving its block data.
- Findings: Entirely mod-specific Create behavior; it is neither vanilla falling-block behavior nor a Forge compatibility layer by itself. Persisted vector/block-list data is assumed to have been generated by the matching implementation and deserves validation before reuse.
- RustChunk relevance: Exclude from base client. Could be a future Create-mod interoperability feature only after the Forge 1.20.1 mod boundary exists.
- Decision: **unnecessary; optional/future — Create-specific moving-tree simulation is outside vanilla scope.**
- 1.20.1 implications: No vanilla 1.20.1 protocol behavior to port. Do not include Create 6.0.10 data or schemas in the 1.20.1 baseline.
- Follow-up checks: For any future Create port, validate NBT vector finiteness/axis constraints, block count/volume caps, registry resolution and malformed/missing coordinates; compare against Create's 1.20.1 release.
- RustChunk implementation mapping: No core mapping; possible `mods/create/falling_trees.rs` in an isolated compatibility layer, not created.
- Evidence required before adoption: Exact target Create version and license, Forge 1.20.1 API fixtures, bounded NBT parser tests and block-motion/render parity cases.
- Completion state: COMPLETE — 243 logical lines read to EOF; pinned SHA and 9833-byte tree size match; no Create runtime was run.

### 1207. `crates/horizon-client/src/game/fangs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b8e237004839a5f71b17ffc9f72ce30d420fa348`
- Size: 7989 bytes; 230 logical lines
- Coverage: Complete text file reviewed, lines 1–230.
- Purpose: Implements Evoker Fangs creation, warmup/lifetime ticking, attack event and damage, ally checks for illagers/vexes, and jaw animation rendering.
- Dependencies / cross-references: Evoker mob AI/owner IDs, entity event 4, player and mob damage sources, collision boxes, audio, model assets and rendering.
- Implementation / logic: Creates fangs with a delay, bites entities inside an inflated box at the target tick, attributes magic/indirect-magic damage to the owner, advances client animation on server event and renders the jaw pose.
- Findings: The core Evoker Fangs mechanic is vanilla and relevant. The file also includes Horizon LAN guest swapping and custom entity-list/model interfaces; those are not part of vanilla semantics. The illager ally rules rely on a simplified no-teams model and should be checked against team-aware vanilla behavior.
- RustChunk relevance: Adapt the gameplay timing, hitbox, damage attribution and event semantics; replace rendering/entity glue and validate ownership/team rules.
- Decision: **essential; rewrite required; version-specific — Evoker Fangs are target gameplay; port the vanilla rules and replace Starling integration.**
- 1.20.1 implications: Rebuild against 1.20.1 EvokerFangs entity tick, owner attribution, event 4, damage source registry and 22-tick attack lifetime. Remove LAN guest/custom model coupling.
- Follow-up checks: Compare warmup boundary, bite tick, hitbox, damage immunity/cooldown, owner death attribution, allied illagers/vexes, event delivery and animation frames.
- RustChunk implementation mapping: Proposed `game/entities/evoker_fangs_1_20_1.rs` with separate client animation adapter, not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, timed tick traces, team/alliance scenarios, damage packets and model animation captures.
- Completion state: COMPLETE — 230 logical lines read to EOF; pinned SHA and 7989-byte tree size match; tests were not run.

### 1208. `crates/horizon-client/src/game/fans.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b7a236e59e4d0786975078e52ed86cea9d680f28`
- Size: 15129 bytes; 379 logical lines
- Coverage: Complete text file reviewed, lines 1–379.
- Purpose: Implements Create Encased Fan air-current simulation, item transport and processing catalysts for washing, blasting, smoking and haunting.
- Dependencies / cross-references: Create 6.0.10 block/entity registries, kinetic speed, item entities, depots, Create processing recipes, fluid/fire block facts and custom sound.
- Implementation / logic: Builds a directional current with reach based on speed and blockers; applies item acceleration and processes dropped/depot stacks using Create recipe categories and timers.
- Findings: Explicitly Create-specific and not a vanilla block/entity or protocol mechanic. It depends on Create recipe identifiers and custom block states and should not be mistaken for generic wind/current physics.
- RustChunk relevance: No base RustChunk need. Keep separate from vanilla world simulation; consider only for a future Forge/Create compatibility target.
- Decision: **unnecessary; optional/future — Create Encased Fan is a mod-specific system, not a vanilla block.**
- 1.20.1 implications: Do not expose Create fan blocks, processing IDs or custom movement as vanilla 1.20.1. A future integration must use the precise compatible Create-for-1.20.1 API/data.
- Follow-up checks: Check mod version/license, block/recipe registry mapping, item transport parity, chunk-boundary behavior and bounded per-tick scans before any future integration.
- RustChunk implementation mapping: No baseline mapping; optional `mods/create/encased_fan.rs` not created.
- Evidence required before adoption: Pinned Create 1.20.1 source/API, Forge capabilities/registrations, recipes and deterministic item-motion traces.
- Completion state: COMPLETE — 379 logical lines read to EOF; pinned SHA and 15129-byte tree size match; embedded test inspected, not run.

### 1209. `crates/horizon-client/src/game/files_offer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fd5fae697bf39b99437687400efddb6da9246054`
- Size: 10887 bytes; 282 logical lines
- Coverage: Complete text file reviewed, lines 1–282.
- Purpose: Implements Horizon Android storage-permission onboarding and the migration offer for worlds/resource packs from Horizon Android.
- Dependencies / cross-references: Android all-files-access permission state, app lifecycle/restart, private/shared game folders, Horizon installation detection/import bridge, title-screen HUD and preferences.
- Implementation / logic: Shows ask/wait/restart cards, polls permission, persists decline state, asks the Horizon import bridge to copy worlds/packs and restarts after shared-folder migration.
- Findings: Launcher/platform-specific flow, not Minecraft game logic. It assumes Horizon package paths, permission bridge methods and Java/Android lifecycle behavior; migration code must be separately assessed for overwrite/atomicity and user consent.
- RustChunk relevance: May inform RustChunk Android storage UX, but should remain in the launcher/platform layer and must not be ported into the Rust game core.
- Decision: **unnecessary; optional/future — Android storage/migration onboarding belongs in the launcher layer, not game core.**
- 1.20.1 implications: Use Android scoped-storage policy appropriate to the target OS and user-selected SAF paths; do not copy Horizon app identity, all-files permission assumptions or migration marker filenames.
- Follow-up checks: If reimplemented, test cancellation, partial copy, source retention, duplicate names, disk-full recovery, restart rollback, and only copy after explicit consent.
- RustChunk implementation mapping: No game-engine mapping. Possible Android launcher `storage/migration_offer` module, not created.
- Evidence required before adoption: Current Android storage policy, target SDK requirements, SAF permission tests, migration transaction tests and explicit user-consent UX review.
- Completion state: COMPLETE — 282 logical lines read to EOF; pinned SHA and 10887-byte tree size match; Android UI/storage flow was not executed.

### 1210. `crates/horizon-client/src/game/fireworks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3e3858eb094b666c4325b3d2d5e8951521e3ab9e`
- Size: 29482 bytes; 736 logical lines
- Coverage: Complete text file reviewed, lines 1–736.
- Purpose: Implements firework rocket item use, projectile flight, Elytra boosting, collision, explosion damage and client rendering/particles.
- Dependencies / cross-references: Firework item data, rocket entity/NBT, crossbow/dispenser launch, player flight state, projectile collision, damage sources, LOS/world blocks, RNG, particles and entity renderer.
- Implementation / logic: Creates rockets with randomized velocity/lifetime, advances free or attached flight, resolves entity/block hits, boosts gliding players, applies radius-5 line-of-sight blast damage and spawns firework effects.
- Findings: Core firework/Elytra mechanics are relevant, but rocket payload parsing uses modern item data components, damage/owner handling is intertwined with LAN guests/native hooks, and Breeze deflection is later than 1.20.1. `blast_damage` relies on callers' radius checks and should clamp the square-root input against floating-point edge cases.
- RustChunk relevance: Essential for Elytra and firework behavior; rewrite the data boundary, target entity/projectile interactions and damage attribution for 1.20.1.
- Decision: **essential; rewrite required; version-specific — Firework/Elytra behavior is essential, with legacy NBT and target projectile rules.**
- 1.20.1 implications: Decode 1.20.1 Fireworks/FireworkExplosion legacy item NBT, use target rocket entity fields/tick lifetime and damage source semantics, remove Breeze-only deflection and Starling LAN/native behavior.
- Follow-up checks: Test launch from block/crossbow/dispenser, Elytra acceleration, flight lifetime/RNG, collision order, owner immunity, radius/LOS, self-damage, multiple stars, particles and save/reload against Java 1.20.1.
- RustChunk implementation mapping: Proposed `game/projectiles/firework_rocket_1_20_1.rs` plus legacy NBT codec and renderer adapter, not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, Fireworks NBT fixtures, deterministic projectile tick traces, entity/block hit packet captures and damage/LOS snapshots.
- Completion state: COMPLETE — 736 logical lines read in consecutive chunks 1–400 and 401–736; pinned SHA and 29482-byte tree size match; embedded test inspected, not run.

### 1211. `crates/horizon-client/src/game/fishing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f9e3e61c399cbae365d04dd7dc6d8187ab3f5737`
- Size: 61717 bytes; 1570 logical lines
- Coverage: Complete text file reviewed, lines 1–1570.
- Purpose: Implements fishing rod casting/reeling, bobber flight and water physics, lure/bite timers, open-water checks, entity hooking, loot retrieval, and bobber/line rendering.
- Dependencies / cross-references: Player hands and enchantments, water/fluid heights, block collision, entity/player bounds, biome/loot tables, RNG, XP/items, projectile events, advancement triggers, entity metadata/packets and rendering.
- Implementation / logic: Creates a hook with randomized trajectory, updates flying/bobbing/hooked states, calculates open water and bite windows, reels entities/items or rolls the gameplay fishing loot table, and interpolates local/remote bobbers for rendering.
- Findings: Fishing is core 1.20.1 gameplay, but this file contains Horizon LAN guest synchronization, native mod hooks, remote entity-state glue and modern item/loot data assumptions. `biome_name_at` indexes the biome list directly with chunk-provided biome data; validate index bounds when parsing untrusted/corrupt chunk data. Hook collision and timing must be checked against 1.20.1 Java behavior.
- RustChunk relevance: Essential gameplay; split server-authoritative simulation, client prediction/render and packet adaptation. Do not port LAN server code into an ordinary multiplayer client.
- Decision: **essential; rewrite required; version-specific — Fishing is essential gameplay; rebuild around the 1.20.1 data/protocol boundary.**
- 1.20.1 implications: Use 1.20.1 FishingHook fields/timers, rod enchantments and loot table data, legacy ItemStack NBT, target entity metadata/packets and exact biome/open-water rules. Remove Horizon LAN and control/status hooks.
- Follow-up checks: Differentially verify trajectory RNG, wall/entity collision ordering, water/bobbing friction, open-water 5x4x5 scan, lure/luck timing, catch loot/XP, hooked pull and line rendering; validate biome ID range.
- RustChunk implementation mapping: Proposed `game/fishing/{hook,loot,network,render}_1_20_1.rs`, not created.
- Evidence required before adoption: Java 1.20.1 source/mappings, seeded bobber traces, fishing loot tables, chunk biome fixtures, packet captures and GUI render goldens.
- Completion state: COMPLETE — 1570 logical lines read in consecutive chunks 1–400, 401–800, 801–1200 and 1201–1570; pinned SHA and 61717-byte tree size match; embedded tests inspected, not run.

### 1212. `crates/horizon-client/src/game/fov.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c4e039b06f01d779ee1c9e3f4dacb9fa36d3f3f4`
- Size: 10185 bytes; 249 logical lines
- Coverage: Complete text file reviewed, lines 1–249.
- Purpose: Implements camera FOV, view/hurt bobbing, portal and Nausea distortion, spyglass scope effects, camera fluid classification and avatar movement bob.
- Dependencies / cross-references: Player movement/pose/effects, camera/view state, options, spyglass use, HUD overlays, fog/fluid state and Horizon native FOV/shake/zoom hooks.
- Implementation / logic: Calculates per-tick FOV modifier and easing, death/fluid effects, projection spin, view bob matrices and spyglass overlay scaling; validates extension-provided FOV values for finiteness/range.
- Findings: Most behavior is client presentation math and conceptually applicable, but it is tied to this snapshot's camera API, settings, native mod hooks and HUD abstractions. Target FOV/effect constants and frame interpolation need parity checks.
- RustChunk relevance: Adaptable client-renderer behavior; not a protocol/game-authority subsystem. A lightweight camera implementation is useful for mobile presentation.
- Decision: **adaptable; rewrite required; version-specific — Camera behavior adapts, but target renderer and settings integration must be rewritten.**
- 1.20.1 implications: Rebuild against 1.20.1 Camera/GameRenderer and Forge renderer hooks; preserve vanilla portal/Nausea/death/fluid/spyglass/bobbing behavior but omit Horizon-only hooks and extras.
- Follow-up checks: Compare FOV for sprinting, bow draw, flying, spyglass, portal, Nausea, underwater/lava/powder snow, hurt/death and view bob across partial ticks; test NaN/infinite extension values.
- RustChunk implementation mapping: Proposed `render/camera/fov_1_20_1.rs` and `render/camera/bob_1_20_1.rs`, not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, camera matrices/FOV traces and frame screenshots across supported renderer backends.
- Completion state: COMPLETE — 249 logical lines read to EOF; pinned SHA and 10185-byte tree size match; embedded validation test inspected, not run.

### 1213. `crates/horizon-client/src/game/foxes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c1242cc86052b98d9eea139780afbd6277a07003`
- Size: 7311 bytes; 166 logical lines
- Coverage: Complete text file reviewed, lines 1–166.
- Purpose: Implements Fox and Dolphin item pickup/looting, item selection and target-position updates for their AI goals.
- Dependencies / cross-references: Mob species/state and held equipment, item entities, mob_griefing gamerule, item food registry, inventory/pickup triggers, water facts and entity spawn/drop helpers.
- Implementation / logic: Scans nearby dropped items, applies Fox food/held-item preference, splits stacks and spits previous items; Dolphins pick up whole stacks when their mouth is free and seek eligible items in water.
- Findings: The behaviors are vanilla-domain, but they depend on Starling mob/equipment structures and target-specific pickup ordering. Team/ownership and Forge event hooks are not implemented here.
- RustChunk relevance: Adaptable to RustChunk's mob AI and item-entity simulation; useful as discrete behavior rules, not as a standalone subsystem.
- Decision: **adaptable; version-specific — Fox/Dolphin item behavior can adapt after target AI and tag parity checks.**
- 1.20.1 implications: Verify exact 1.20.1 Fox allowed-food list/held-item preference, Dolphin allowed-item conditions, `mob_griefing`, pickup delay, equipment drop chance and AI goal scheduling.
- Follow-up checks: Test full/partial item stacks, occupied Fox mouth, food vs non-food priorities, previous item spit, Dolphin water requirement, pickup delay, mob_griefing and trigger order.
- RustChunk implementation mapping: Proposed `game/mobs/{fox,dolphin}_pickup_1_20_1.rs`, not created.
- Evidence required before adoption: Java 1.20.1 source/mappings, item tag snapshots, seeded AI tick traces, pickup event and equipment NBT fixtures.
- Completion state: COMPLETE — 166 logical lines read to EOF; pinned SHA and 7311-byte tree size match; no AI tests were run.

### 1214. `crates/horizon-client/src/game/frames.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7521dd155513be73374e9438507194bce8d7ed1e`
- Size: 26519 bytes; 661 logical lines
- Coverage: Complete text file reviewed, lines 1–661.
- Purpose: Implements item-frame and glow-item-frame placement, support/survival checks, interaction/rotation, break/drop behavior, comparator updates, explosion damage and frame/item/map rendering.
- Dependencies / cross-references: Hanging entity bounds, block collision/support facts, item stacks, map IDs/data, drops/loot, game rules, comparator neighbor updates, atlas/model baking and renderer.
- Implementation / logic: Places facing-specific frames, checks build height/support/overlap, inserts or rotates items, removes item/frame on attacks, drops contents, updates comparator output, handles blast/survival ticks and draws frame/map/item geometry.
- Findings: Item frames and glow item frames are vanilla 1.20.1 features. This implementation is tied to current ItemStack extras/components, custom `ItemFrame` representation and native hanging/render hooks. Fixed/invisible/drop-chance behavior, support checks and map rendering need target parity.
- RustChunk relevance: Essential display/interact feature, especially for maps and item displays. Rebuild the entity/block interaction and rendering against 1.20.1 rather than copy the modern data model.
- Decision: **essential; rewrite required; version-specific — Item-frame behavior is essential and should be rewritten with target entity/NBT/rendering.**
- 1.20.1 implications: Use 1.20.1 ItemFrame/GlowItemFrame entity state, legacy entity/item NBT, exact facing/support/rotation/drop logic, map data and Forge hanging-entity event integration.
- Follow-up checks: Test all six facings, support removal, overlapping frames, fixed/invisible flags, add/rotate/remove, comparator output, explosion and gamerule drops, maps/glow brightness and chunk reload.
- RustChunk implementation mapping: Proposed `game/entities/item_frame_1_20_1.rs` with separate `render/item_frame_1_20_1.rs`, not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, ItemFrame NBT fixtures, map packet/state snapshots, comparator traces, model resources and renderer goldens.
- Completion state: COMPLETE — 661 logical lines read in consecutive chunks 1–200, 201–400, 401–600 and 601–661; pinned SHA and 26519-byte tree size match; no rendering/gameplay tests were run.

### 1215. `crates/horizon-client/src/game/friends.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f2480ec76aea698444d83dec9d7f09244682f0cc`
- Size: 12510 bytes; 322 logical lines
- Coverage: Complete text file reviewed, lines 1–322.
- Purpose: Hosts Horizon's Play with Friends flow: opens an integrated world to LAN, starts P2P/STUN/UPnP/relay sharing, generates invite codes and displays connected/relayed sessions.
- Dependencies / cross-references: Horizon Friends networking crate, integrated server/LAN lifecycle, STUN/relay settings, router port mapping, invite-code/key handling, clipboard, friends UI and connection status.
- Implementation / logic: Starts/stops sharing, tracks reachability and sessions, issues codes, optionally maps router ports and tunnels guest connections through loopback into the LAN host.
- Findings: Not ordinary Minecraft multiplayer client behavior and exposes a separate public sharing/network service. It should not be imported without a threat model, explicit user consent, secret handling and isolation from the standard server connection path.
- RustChunk relevance: Exclude from the 1.20.1 client baseline. Could be considered as a separate optional launcher feature, but not required for Aternos or normal multiplayer.
- Decision: **unnecessary; optional/future — Invite-code P2P sharing is a separate optional network product, not vanilla multiplayer.**
- 1.20.1 implications: For standard server joins, implement the official 1.20.1 handshake/login/play connection only. Do not conflate invite-code relaying with vanilla server protocol.
- Follow-up checks: If ever reintroduced, audit invite-code entropy/expiry, relay authentication, UPnP consent, firewall behavior, key storage, abuse limits and stop/cleanup on world close.
- RustChunk implementation mapping: No game-core mapping. Separate future launcher module only, not created.
- Evidence required before adoption: Security threat model, consent UX, protocol design, relay operator trust review, penetration tests and lifecycle teardown verification.
- Completion state: COMPLETE — 322 logical lines read to EOF; pinned SHA and 12510-byte tree size match; no networking session or port mapping was started.

### 1216. `crates/horizon-client/src/game/functions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `76305d0e4daa91a6d11e02f7bfff26e6fe363e82`
- Size: 43012 bytes; 1079 logical lines
- Coverage: Complete text file reviewed, lines 1–1079.
- Purpose: Implements data-pack discovery/stacking and reload, function and function-tag execution, predicates, `/function`, `/return`, `/schedule`, `/datapack` and scheduled-event persistence.
- Dependencies / cross-references: World datapack folder and settings, builtin/feature packs, JSON/SNBT/NBT parsing, command execution/permission model, function macros, predicates, world content tables, function tags, save files and native mod data contributions.
- Implementation / logic: Loads selected packs in priority order, merges contributed files, rebuilds function/predicate tables, runs load/tick tags and scheduled functions, processes function return state and updates/enables/disables datapacks.
- Findings: Datapacks, functions, predicates and `/schedule` are essential 1.20.1 domains, but the file uses newer data-pack format/feature-pack assumptions, `/return`/macro-era command behavior and later feature IDs. Native mod data contributions and dedicated-server permission behavior are Horizon-specific extensions.
- RustChunk relevance: Essential high-level subsystem; split into target 1.20.1 datapack loader, function engine, predicate evaluator, scheduler and command integration. Don't copy later `FEATURE_PACKS` IDs or schema.
- Decision: **essential; rewrite required; version-specific — Datapacks/functions/scheduling are essential, but the modern format and command assumptions must be replaced.**
- 1.20.1 implications: Reconstruct Java 1.20.1 datapack format, pack discovery/order, `/function`, function tags, predicates, `/schedule`, reload semantics and `minecraft:load`/`minecraft:tick`; use the target command set and NBT persistence. Feature packs and return/macro behavior must be gated by actual 1.20.1 support.
- Follow-up checks: Compare selected-pack precedence, enable/disable and reload, invalid JSON/pack errors, function recursion/command limits, schedule append/replace/clear ordering, world-time persistence, load/tick tags and predicate outputs.
- RustChunk implementation mapping: Proposed `game/datapacks/{stack,functions,predicates,scheduler,commands}_1_20_1.rs`, not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, builtin datapack and format fixtures, function/return/schedule command traces, world save NBT and Forge datapack reload events.
- Completion state: COMPLETE — 1079 logical lines read in consecutive chunks 1–400, 401–800 and 801–1079; pinned SHA and 43012-byte tree size match; embedded tests inspected, not run.

### 1217. `crates/horizon-client/src/game/funnels.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dc6e0591d69bf133a6f4d0ed5805e742aaf14494`
- Size: 14510 bytes; 378 logical lines
- Coverage: Complete text file reviewed, lines 1–378.
- Purpose: Simulates Create Funnel and Belt Funnel behavior: item collection/insertion, extraction cadence, redstone pause, brass stack size and belt interception.
- Dependencies / cross-references: Create block states/registries, kinetic belts and speeds, inventories, dropped item entities, depots, redstone neighbor signal and Create flap sound.
- Implementation / logic: Reads funnel facing/mode, collects or extracts stacks on the configured side, uses an 8-tick cooldown, and coordinates belt funnel pushing/pulling with transported belt items.
- Findings: Explicit Create 6.0.10 functionality; no vanilla 1.20.1 Funnel block exists. It is tied to Create registry names/kinetic belt mechanics and is not a general inventory automation primitive.
- RustChunk relevance: Exclude from vanilla game and base client. It may be valuable only within a distinct Forge/Create mod integration.
- Decision: **unnecessary; optional/future — Create Funnel behavior is not part of vanilla 1.20.1.**
- 1.20.1 implications: Do not register Create block IDs or implement funnels as vanilla. Keep target Forge mod block/entity integration separate and version-pinned.
- Follow-up checks: For future support, verify exact Create release for MC 1.20.1, Forge capability/inventory sidedness, block state properties, redstone timing and belt item movement.
- RustChunk implementation mapping: No core mapping; possible `mods/create/funnels.rs` in a mod-specific adapter, not created.
- Evidence required before adoption: Create 1.20.1 source, Forge inventory/capability fixtures, item movement traces and mod-license review.
- Completion state: COMPLETE — 378 logical lines read to EOF; pinned SHA and 14510-byte tree size match; no Create tests were run.

### 1218. `crates/horizon-client/src/game/game_mode.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `afc2db79aacc4a81fe9ec8782e6e4933999e3360`
- Size: 9347 bytes; 241 logical lines
- Coverage: Complete text file reviewed, lines 1–241.
- Purpose: Implements survival, creative, adventure and spectator mode IDs/transitions, abilities, spectator movement/chunk policy and adventure block predicates.
- Dependencies / cross-references: Player abilities/flying/spectator state, vehicle mounting, collision/ground checks, game rules, item data predicates, block/tag registries and world chunk tickets.
- Implementation / logic: Maps game-mode IDs, updates flight/invulnerability/build permissions, dismounts spectators, handles spectator chunk-generation policy and checks held-item break/place predicates against target blocks/properties.
- Findings: Game-mode transitions and spectator behavior are essential, but adventure permission checks rely on modern `can_break`/`can_place_on` item components while 1.20.1 uses legacy NBT semantics. Other client/server authority and chunk-ticket code must be separated.
- RustChunk relevance: Essential state-transition rules for a compliant client, with authoritative mode changes remaining server-driven in multiplayer.
- Decision: **essential; rewrite required; version-specific — Game-mode and spectator mechanics are essential; adventure item predicates need legacy NBT.**
- 1.20.1 implications: Use 1.20.1 game-mode IDs and Abilities semantics, legacy `CanDestroy`/`CanPlaceOn` NBT predicates, spectator camera/input and `spectatorsGenerateChunks` gamerule behavior.
- Follow-up checks: Test all mode transitions, previous-mode cycling, flight toggle/landing, spectator dismount/collision, item predicates/tags, server ability updates and chunk ticket behavior.
- RustChunk implementation mapping: Proposed `game/player/game_mode_1_20_1.rs` and `game/items/adventure_predicates_1_20_1.rs`, not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, player abilities packet captures, target item NBT and block-tag fixtures, spectator chunk-manager traces.
- Completion state: COMPLETE — 241 logical lines read to EOF; pinned SHA and 9347-byte tree size match; embedded mode test inspected, not run.

### 1219. `crates/horizon-client/src/game/game_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `43cdaaab472dd04803bae02fb12ee69392a7451e`
- Size: 56315 bytes; 1650 logical lines
- Coverage: Complete text file reviewed, lines 1–1650.
- Purpose: Implements a GameTest runner, test batches/grid placement, block-based test instances, retry/flaky-test reporting, per-tick assertions and test output.
- Dependencies / cross-references: GameTest registry/test definitions, test-instance and test blocks, structure templates, dimensions, chunk loading/forcing, world mutation, listeners, commands and server-side level ownership.
- Implementation / logic: Builds batches by environment/dimension, positions structures on a grid, activates test environments, runs test blocks/functions over ticks, tracks attempts/successes and schedules retries.
- Findings: This is test infrastructure, not gameplay. It references test-instance blocks, newer environment/retry behavior and a partial built-in function set; test APIs and structures may differ from 1.20.1's GameTest framework. Several methods can force/load chunks for up to 120 seconds, so they must never enter a mobile release runtime.
- RustChunk relevance: Potentially useful as a test-only orchestration reference for deterministic parity and Forge GameTests. It is not a client runtime subsystem.
- Decision: **adaptable; reference; optional/future — GameTest orchestration is test-only reference material; not mobile runtime code.**
- 1.20.1 implications: Build an isolated test harness using the actual Forge 1.20.1 GameTest API and target test blocks/annotations; keep chunk forcing, console output and test runner out of shipping mobile gameplay.
- Follow-up checks: Compare test registration/structure placement/assertions and teardown with Forge 1.20.1 GameTest; bound forced chunks/timeouts; add test-only build feature and ensure no production linkage.
- RustChunk implementation mapping: Possible `tests/gametest/runner_1_20_1.rs` or Java-oracle harness, not created.
- Evidence required before adoption: Forge 1.20.1 GameTest API, target structure fixtures, deterministic tick/test results, chunk cleanup tests and release dependency graph proving test-only isolation.
- Completion state: COMPLETE — 1650 logical lines read in consecutive chunks 1–400, 401–800, 801–1200, 1201–1600 and 1601–1650; pinned SHA and 56315-byte tree size match; embedded tests inspected, not run.

### 1220. `crates/horizon-client/src/game/gamemode_switcher.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5d0c98048e15ec67e399e3f6fc654c35fae17786`
- Size: 3246 bytes; 91 logical lines
- Coverage: Complete text file reviewed, lines 1–91.
- Purpose: Implements the non-pausing F3+F4 game-mode switcher overlay, including selection, hover state, icons, drawing and apply/cancel flow.
- Dependencies / cross-references: GameModeSwitcher UI model, mode list/icon assets, keybind display, cursor, player authority/gamemaster check and GameMode transition.
- Implementation / logic: Opens the overlay, clears movement/mining input, handles selected mode, draws mode slots/help text, then applies the selection if the local player has authority.
- Findings: The F3+F4 mode-switcher concept is present in the 1.20.1 client, so its user flow is relevant. This file depends on Starling's UI/HUD and gamemaster/session model; it is not a complete keyboard input or server permission implementation.
- RustChunk relevance: Adaptable client UI reference for keyboard users; Android touch-first UI may not need it in the baseline.
- Decision: **adaptable; reference; version-specific — F3+F4 is a useful UI reference; rebuild for target controls and Android.**
- 1.20.1 implications: Match 1.20.1 available modes, F3+F4 input sequencing, local singleplayer/creative permission, translated text and icon assets. On Android, expose an optional accessible equivalent rather than relying on a physical F3 key.
- Follow-up checks: Test mode cycle/previous mode, cancel/apply, key repeat, focus/capture restoration, server rejection and touch accessibility on mobile.
- RustChunk implementation mapping: Possible `ui/debug/game_mode_switcher_1_20_1.rs`, not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, key event traces, mode transition tests, UI screenshots and Android accessibility/navigation review.
- Completion state: COMPLETE — 91 logical lines read to EOF; pinned SHA and 3246-byte tree size match; no UI test or runtime mode change was executed.

### 1221. `crates/horizon-client/src/game/ghasts.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b8cc40e5b6498c21a5c8195e19ce15c3a98105b2`
- Size: 7007 bytes; 172 logical lines
- Coverage: 1–172 (complete, EOF)
- Purpose: Happy Ghast harness/mount/dismount, rider state and hatching; includes selected vehicle collision helpers.
- Dependencies / cross-references: Happy/Dried Ghast, riding, player interaction, item use, collision, entity events and render.
- Implementation / logic: Tracks harnessing and riding transitions, rider pose and hatching timers; includes selected vehicle/shulker collision handling.
- Findings: Happy Ghast and Dried Ghast are later than 1.20.1; the main behavior cannot enter target registries. Generic collision ideas need independent call-site proof.
- RustChunk relevance: Exclude from baseline; only extract generic mount/collision concepts if separately verified.
- Decision: **unnecessary; optional/future — Do not register Happy Ghast/Dried Ghast IDs, harness items, metadata or packets in 1.20.1.**
- 1.20.1 implications: Do not register Happy Ghast/Dried Ghast IDs, harness items, metadata or packets in 1.20.1.
- Follow-up checks: Verify any shared helper calls for target-era boats/shulkers; compare dismount search, mount AABBs, entity push and vehicle packets.
- RustChunk implementation mapping: No core mapping; possible generic game/entity/riding_collision reference, not created.
- Evidence required before adoption: 1.20.1 entity roster/source and collision fixtures proving any extracted helper is independent of Happy Ghast.
- Completion state: COMPLETE — 172 logical lines to EOF; pinned SHA and 7007-byte size match; no runtime tests.

### 1222. `crates/horizon-client/src/game/glowing.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `049040f97491f69e388e72320779727bd3745119`
- Size: 2139 bytes; 53 logical lines
- Coverage: 1–53 (complete, EOF)
- Purpose: Tracks glowing status for remote entities and resolves outline/team color for client rendering.
- Dependencies / cross-references: Entity metadata/effect flags, scoreboard teams, entity tracking and outline renderer.
- Implementation / logic: Reads glowing state and team color to decide whether to submit an entity silhouette to outline pass.
- Findings: Glowing and team-colored outlines exist in 1.20.1, but this helper is coupled to Starling team/render models.
- RustChunk relevance: Adaptable client render feature; isolate from gameplay simulation.
- Decision: **adaptable; version-specific — Map 1.20.1 metadata/effect and scoreboard team updates into the outline pass.**
- 1.20.1 implications: Map 1.20.1 metadata/effect and scoreboard team updates into the outline pass.
- Follow-up checks: Compare effect expiry, team color changes/removal, invisible glowing entities and entity unload.
- RustChunk implementation mapping: Proposed render/entity_outline/glowing_1_20_1.rs plus metadata adapter, not created.
- Evidence required before adoption: 1.20.1 metadata/team packet fixtures, render goldens and effect expiry traces.
- Completion state: COMPLETE — 53 logical lines to EOF; pinned SHA and 2139-byte size match.

### 1223. `crates/horizon-client/src/game/golems.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `60c757be8b9938c37f6bd587ad5815d1f49bdcd9`
- Size: 16247 bytes; 396 logical lines
- Coverage: 1–396 (complete, EOF)
- Purpose: Recognizes spawn patterns and creates snow/iron golems alongside later copper-golem/copper-chest additions.
- Dependencies / cross-references: Block states/tags, carved pumpkin, head/body patterns, entity spawn/collision, player-created flag and world mutation.
- Implementation / logic: Checks oriented patterns, changes/consumes blocks and spawns matching golems; includes copper branches from a later version.
- Findings: Snow/iron golem patterns are target-era. Copper Golem and Copper Chest are post-1.20.1 and must be stripped.
- RustChunk relevance: Retain target snow/iron pattern behavior; remove newer golem/chest IDs and paths.
- Decision: **essential; rewrite required; version-specific — Implement only 1.20.1 snow/iron patterns, orientation, pumpkin, spawn position and player-created semantics.**
- 1.20.1 implications: Implement only 1.20.1 snow/iron patterns, orientation, pumpkin, spawn position and player-created semantics.
- Follow-up checks: Test six orientations, partial patterns, obstruction, update flags/drops, player-created, peaceful/difficulty rules and spawn location.
- RustChunk implementation mapping: Proposed game/spawning/golems_1_20_1.rs split into snow and iron patterns, not created.
- Evidence required before adoption: 1.20.1 source/mappings, block fixtures, deterministic pattern tests and spawn NBT/packet snapshots.
- Completion state: COMPLETE — 396 logical lines to EOF; pinned SHA and 16247-byte size match; embedded tests inspected only.

### 1224. `crates/horizon-client/src/game/growth.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c52e7394e44a132ab01a264595db63ace9902c44`
- Size: 5079 bytes; 124 logical lines
- Coverage: 1–124 (complete, EOF)
- Purpose: Implements sapling/plant growth and a shears interaction that removes eligible plant heads.
- Dependencies / cross-references: Bone meal, live feature selection, vegetation states/tags, world RNG and block update/placement.
- Implementation / logic: Delegates growth to a live-feature helper and performs a targeted shears action on certain plant heads.
- Findings: Bone-meal growth is target-relevant; LiveFeatures and plant-head shears paths are snapshot/mod dependent; target IDs/tags need 1.20.1 data.
- RustChunk relevance: Adaptable world interaction reference; do not bind to newer worldgen registry.
- Decision: **adaptable; rewrite required; version-specific — Use 1.20.1 configured features, bonemeal rules and legacy tags; isolate non-vanilla shears behavior.**
- 1.20.1 implications: Use 1.20.1 configured features, bonemeal rules and legacy tags; isolate non-vanilla shears behavior.
- Follow-up checks: Compare bonemeal attempts/RNG, biome feature selection, server item consumption, block updates and target plant shears rules.
- RustChunk implementation mapping: Proposed game/world/growth_1_20_1.rs with target feature registry adapter, not created.
- Evidence required before adoption: 1.20.1 configured-feature data, block/item tags, seeded traces and Forge bonemeal event tests.
- Completion state: COMPLETE — 124 logical lines to EOF; pinned SHA and 5079-byte size match.

### 1225. `crates/horizon-client/src/game/hand.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3374297c13e4039acfc99c747938e734c7acf2dd`
- Size: 41204 bytes; 990 logical lines
- Coverage: 1–990 (complete, EOF)
- Purpose: Implements first-person hand interactions: ray picking, attacks, block breaking, item-use clicks, cooldown, swings and equip state; includes later reach/piercing features.
- Dependencies / cross-references: Player inventory/attributes, hands/shields, attack ticker, first-person render, hit tests, mobs/vehicles/frames, item use, mining, audio and packets.
- Implementation / logic: Processes attack/use clicks, chooses tick-stable targets, calculates destroy progress, starts/stops mining, applies reach-filtered attacks and advances swing/equip transitions.
- Findings: Core hand/block-breaking is essential. Modern attack_range, spear/piercing and kinetic weapon paths are not 1.20.1; server authority must remain separate from prediction.
- RustChunk relevance: Essential client interaction architecture; rewrite target reach, cooldown, mining, hand use and packet behavior.
- Decision: **essential; rewrite required; version-specific — Rebuild 1.20.1 reach/attack cooldown, old item data, mining packets and main/offhand behavior; remove spear and modern attack_range.**
- 1.20.1 implications: Rebuild 1.20.1 reach/attack cooldown, old item data, mining packets and main/offhand behavior; remove spear and modern attack_range.
- Follow-up checks: Compare hit ordering, reach, attack cooldown/damage, mining tools/effects, creative restrictions, equip animation, use order and server correction.
- RustChunk implementation mapping: Proposed game/input/hand_actions_1_20_1.rs, game/combat/reach_1_20_1.rs and render/first_person/hands_1_20_1.rs, not created.
- Evidence required before adoption: 1.20.1 source/mappings, attribute/item NBT fixtures, interaction packet captures and Forge attack/break traces.
- Completion state: COMPLETE — 990 logical lines in contiguous ranges 1–400, 401–800, 801–990; SHA/41204 bytes match; tests inspected only.

### 1226. `crates/horizon-client/src/game/headless.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `587bd9b2e449d67410e6d9518c961519438b98e1`
- Size: 3260 bytes; 98 logical lines
- Coverage: 1–98 (complete, EOF)
- Purpose: Runs a headless Game loop and shutdown/save at nominal 50 ms cadence.
- Dependencies / cross-references: EventLoopCtl, Game tick/idle, control socket, quit/save path, wall clock and worker sleep.
- Implementation / logic: Captures exit/control-flow requests, ticks at 20 Hz or waits to deadline, then shuts down.
- Findings: Useful for deterministic/server harnesses but tied to Horizon control socket, winit flow and current lifecycle; not an Android renderer solution.
- RustChunk relevance: Reference for test-only/headless oracle mode, not necessary for Android external-server join.
- Decision: **adaptable; reference; optional/future — If added, use platform-neutral clock/scheduler and keep headless driver separate from shipping client.**
- 1.20.1 implications: If added, use platform-neutral clock/scheduler and keep headless driver separate from shipping client.
- Follow-up checks: Test pause/resume, stop, sleep drift, save-on-exit, injected clock and worker/network teardown.
- RustChunk implementation mapping: Possible tests/headless_driver.rs behind test feature, not created.
- Evidence required before adoption: Deterministic clock, test-only feature boundary and lifecycle tests; prove no agent socket in production.
- Completion state: COMPLETE — 98 logical lines to EOF; SHA/3260 bytes match; tests inspected, not run.

### 1227. `crates/horizon-client/src/game/ime.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `51a270608beb251429d96849ab8ee454fe2dcfbb`
- Size: 6539 bytes; 151 logical lines
- Coverage: 1–151 (complete, EOF)
- Purpose: Handles OS IME preedit/commit, candidate/caret placement and text dispatch to focused screens.
- Dependencies / cross-references: Winit IME events, focused widgets, chat/command/sign/book/options screens, UI scale and Android text entry.
- Implementation / logic: Tracks whether desktop IME is enabled, maps preedit/cursor and filters control chars on commit; Android bypasses this Winit path.
- Findings: Multilingual composition matters, but this is platform/UI-specific and explicitly disables Winit IME on Android.
- RustChunk relevance: Adapt text entry in platform/UI layer; keep IME out of game simulation/protocol.
- Decision: **adaptable; version-specific — Integrate Android native soft keyboard and desktop IME separately; preserve preedit, commit, Unicode and focus-loss behavior.**
- 1.20.1 implications: Integrate Android native soft keyboard and desktop IME separately; preserve preedit, commit, Unicode and focus-loss behavior.
- Follow-up checks: Test Persian/Arabic/CJK, combining chars, focus/cancel, keyboard show/hide, scale and pause/resume.
- RustChunk implementation mapping: Proposed platform/text_input/android.rs and desktop.rs with UI text sinks, not created.
- Evidence required before adoption: Android input connection tests, desktop preedit fixtures, Unicode cases and accessibility focus traces.
- Completion state: COMPLETE — 151 logical lines to EOF; SHA/6539 bytes match; no OS keyboard tests.

### 1228. `crates/horizon-client/src/game/imports.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d9ab95a0132ddf881d5ff36418f84f800b646871`
- Size: 28209 bytes; 804 logical lines
- Coverage: 1–804 (complete, EOF)
- Purpose: Polls an Android Open With/share/file-picker inbox and presents import progress/results via HUD banners and chime.
- Dependencies / cross-references: Horizon Inbox/Places import service, Android paths/permissions, game/resources folders, HUD/audio, skin/resource/mod refresh and app lifecycle.
- Implementation / logic: Polls inbox every 400 ms, tracks transfer/install states, rate-limits chimes, shows timed banners and refreshes dependent assets.
- Findings: Launcher integration, not game or multiplayer logic. Actual install is delegated to a separate importer; archive/path security is not established by this UI file.
- RustChunk relevance: Optional launcher UX reference; keep outside game core and audit underlying importer independently.
- Decision: **unnecessary; optional/future — Use Android scoped storage/SAF and explicit consent; do not assume Horizon app IDs or inbox layout.**
- 1.20.1 implications: Use Android scoped storage/SAF and explicit consent; do not assume Horizon app IDs or inbox layout.
- Follow-up checks: Audit importer for zip-slip/path traversal, symlinks, size limits, atomic copy, overwrites, JAR provenance and rollback.
- RustChunk implementation mapping: Possible launcher/imports/inbox_ui.rs after importer security review, not created.
- Evidence required before adoption: Android storage policy, archive security review, atomic-copy/rollback tests, consent UX and partial-transfer tests.
- Completion state: COMPLETE — 804 logical lines in ranges 1–400, 401–800, 801–804; SHA/28209 bytes match; no import/UI executed.

### 1229. `crates/horizon-client/src/game/initial_spawn.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ae6956ab40ebf14d5a8a62496a767fd0688adf5b`
- Size: 14855 bytes; 367 logical lines
- Coverage: 1–367 (complete, EOF)
- Purpose: Finds world/player spawn and respawn locations and places bonus chest/loot block entity in new worlds.
- Dependencies / cross-references: World generator/spawn height, chunks/light, biome/flat settings, heightmaps/collision, border, respawn radius/game mode, RNG, NBT and loot.
- Implementation / logic: Walks a chunk spiral, requests chunks, searches up to 1024 respawn columns, sets player height and optionally places bonus chest/torches.
- Findings: Important server/world behavior but tied to Horizon generator/streaming. server_player_spawn skips unloaded candidates instead of waiting, potentially differing from vanilla.
- RustChunk relevance: Essential for integrated world support; a remote client must consume authoritative server spawn/chunk packets.
- Decision: **essential; rewrite required; version-specific — Rebuild 1.20.1 world spawn, respawn radius, collision, bonus-chest RNG/loot NBT and chunk readiness; never override remote server spawn.**
- 1.20.1 implications: Rebuild 1.20.1 world spawn, respawn radius, collision, bonus-chest RNG/loot NBT and chunk readiness; never override remote server spawn.
- Follow-up checks: Compare spiral, heightmaps, border/radius/RNG, chest loot, unloaded chunk behavior and respawn corrections.
- RustChunk implementation mapping: Possible game/world/spawn/{world_spawn,player_respawn,bonus_chest}_1_20_1.rs, not created.
- Evidence required before adoption: 1.20.1 mappings/source, seeded world fixtures, heightmap/collision tests, respawn and bonus chest NBT.
- Completion state: COMPLETE — 367 logical lines to EOF; SHA/14855 bytes match; no worldgen test was run.

### 1230. `crates/horizon-client/src/game/input_mode.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `018916e4d19c6fd7b512956ee5dff6021f26a7db`
- Size: 3695 bytes; 90 logical lines
- Coverage: 1–90 (complete, EOF)
- Purpose: Selects Touch/KeyboardMouse/Gamepad mode and translates Android pointer-capture/mouse events into game window events.
- Dependencies / cross-references: Android pointer capture/keyboard, touch settings, gamepad, winit event translation, event loop and touch-release state.
- Implementation / logic: Updates mode from recent input, releases virtual touch presses on switch, resets gamepad pointer ownership and maps Android mouse/wheel/motion events.
- Findings: Relevant to Android-first design but depends on Horizon JNI and touch UI; adapt, do not copy.
- RustChunk relevance: Useful mobile input policy; isolate Android glue from common input state.
- Decision: **adaptable; rewrite required; version-specific — Create RustChunk-owned touch, keyboard/mouse and controller abstraction with lifecycle-aware Android event translation.**
- 1.20.1 implications: Create RustChunk-owned touch, keyboard/mouse and controller abstraction with lifecycle-aware Android event translation.
- Follow-up checks: Test mode priority, touch release, pointer capture loss, wheel/motion, reconnect, pause/resume and stuck keys.
- RustChunk implementation mapping: Proposed platform/input/{mode,android_events,gamepad}.rs plus UI adapter, not created.
- Evidence required before adoption: Android NDK input fixtures, device traces, controller maps, lifecycle stress and Galaxy A22 latency evidence.
- Completion state: COMPLETE — 90 logical lines to EOF; SHA/3695 bytes match; no Android device test.

### 1231. `crates/horizon-client/src/game/input_trace.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `dec41557ad4e3968dcdc810944862c3314320cee`
- Size: 22937 bytes; 684 logical lines
- Coverage: 1–684 (complete, EOF)
- Purpose: Records keyboard/mouse/gamepad/control-socket input origin; exposes recent summaries and optional deterministic repro recording.
- Dependencies / cross-references: Window event dispatch, key mapping, control socket/agent waits, bounded event ring, repro recorder, parser, password focus and JSON writer.
- Implementation / logic: Keeps a 512-event ring with source/tick/time; counts origins, rate-limits user intervention, redacts selected arguments and suppresses replay-generated events.
- Findings: Agent intrusion is Horizon-specific; provenance/privacy patterns are useful. Redaction list is narrow and not a universal secret scrubber.
- RustChunk relevance: Exclude agent-control API; reuse privacy/provenance only for opt-in crash/repro logs.
- Decision: **unnecessary; reference; optional/future — No protocol behavior; future traces must not persist chat/password/token/server credentials and require explicit consent.**
- 1.20.1 implications: No protocol behavior; future traces must not persist chat/password/token/server credentials and require explicit consent.
- Follow-up checks: Audit command/text paths, improve secret redaction, bound lines/ring, validate overflow and close writers on shutdown.
- RustChunk implementation mapping: Possible diagnostics/input_repro.rs behind explicit opt-in, not created.
- Evidence required before adoption: Privacy threat model, redaction tests, retention policy, bounded queue and replay determinism tests.
- Completion state: COMPLETE — 684 logical lines in ranges 1–400 and 401–684; SHA/22937 bytes match; tests inspected, not run.

### 1232. `crates/horizon-client/src/game/integrated.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `baedd4a77d6b62dc337158bbaf451f8689cb040b`
- Size: 53468 bytes; 1329 logical lines
- Coverage: 1–1329 (complete, EOF)
- Purpose: Runs an integrated singleplayer server on its own thread with asynchronous opening, in-memory client Pipe, task queue, pause/save, progress and optional LAN publishing.
- Dependencies / cross-references: WorldLoader/WorldRequest, Game, threads/channels/flags, save/session locks, LAN owner permissions, streaming, options, native mod hooks, commands and shutdown.
- Implementation / logic: Spawns a server thread, prepares world asynchronously, joins via memory Pipe, runs 20 TPS, handles pause/save, view distance, tasks, LAN publish/unpublish and teardown.
- Findings: Sound concept for singleplayer, but coupled to Horizon LAN hub, offline/splitscreen policy, native lifecycle, harnesses and world resources. WorldRequest folder path needs boundary validation. Not the remote Aternos path.
- RustChunk relevance: Optional/future for client-first RustChunk; useful if singleplayer later required. Keep integrated server separate from remote session.
- Decision: **adaptable; optional/future — Implement 1.20.1 integrated server if needed; do not reuse Horizon P2P/LAN/auth; match Forge lifecycle and side separation.**
- 1.20.1 implications: Implement 1.20.1 integrated server if needed; do not reuse Horizon P2P/LAN/auth; match Forge lifecycle and side separation.
- Follow-up checks: Trace save path/symlinks, world open/cancel/close, ticks, pause save, player save order, memory Pipe, publish policy, reload and Forge lifecycle.
- RustChunk implementation mapping: Possible runtime/integrated_server_1_20_1.rs isolated from network/remote_session.rs, not created.
- Evidence required before adoption: 1.20.1 Java/Forge source, path tests, save/reload fixtures, lifecycle traces, Pipe conformance and shutdown stress.
- Completion state: COMPLETE — 1329 logical lines in ranges 1–400, 401–800, 801–1200, 1201–1329; SHA/53468 bytes match; tests inspected only.

### 1233. `crates/horizon-client/src/game/interactions.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ddf1d967eb21ee7f1673434439aaadd431a7e334`
- Size: 4886 bytes; 132 logical lines
- Coverage: 1–132 (complete, EOF)
- Purpose: Represents and ray-picks remote minecraft:interaction entities: invisible server-defined hitboxes consuming clicks and optionally riding another entity.
- Dependencies / cross-references: Entity spawn/metadata/movement packets, reach, ray-box, vehicle/passenger transform and response flag.
- Implementation / logic: Stores position/width/height/response/vehicle, builds an AABB, updates metadata 8–10, applies remote movement and finds nearest ray hit.
- Findings: Target-era feature, but width/height are floats without finite/nonnegative/bound checks; malformed dimensions may yield invalid bounds and unpredictable ray hits.
- RustChunk relevance: Adapt remote entity metadata and hit testing; reject malformed dimensions before ray/render use.
- Decision: **adaptable; version-specific — Use 1.20.1 metadata indexes/response semantics; validate dimensions finite and bounded; match target mount movement.**
- 1.20.1 implications: Use 1.20.1 metadata indexes/response semantics; validate dimensions finite and bounded; match target mount movement.
- Follow-up checks: Test negative/zero/NaN/infinite/huge sizes, AABB, teleport/vehicle motion, reach and click-result packets.
- RustChunk implementation mapping: Proposed network/entities/interaction_1_20_1.rs and game/raycast/remote_interaction.rs, not created.
- Evidence required before adoption: 1.20.1 metadata captures, adversarial float fixtures, ray-box properties and click traces.
- Completion state: COMPLETE — 132 logical lines to EOF; SHA/4886 bytes match; bounds tests inspected, not run.

### 1234. `crates/horizon-client/src/game/item_draw.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `19776b1660bfe859d1737c9476c7db3a5ddc3489`
- Size: 26733 bytes; 681 logical lines
- Coverage: 1–681 (complete, EOF)
- Purpose: Binds item-model resolver to game data and submits GUI, dropped, held, framed and skinned item geometry with lighting/glint/HDR.
- Dependencies / cross-references: Item model resolver, Stack/components, CIT, sprites/atlas, banner/shield textures, skins, dimension sky/time, light, dropped items, GPU skinning.
- Implementation / logic: Builds ItemEnv, resolves by context/owner/seed, computes potion/bundle/time context and emits GUI/world/skinned geometry with glint/HDR/frustum culling.
- Findings: Item rendering is essential but this pipeline uses newer data-driven item models/components, CIT, HDR and Starling renderer; cannot be copied into 1.20.1 unchanged.
- RustChunk relevance: Essential rendering domain and architecture reference; build target 1.20.1 JSON predicates, legacy NBT and Forge asset namespaces.
- Decision: **essential; rewrite required; version-specific — Rebuild 1.20.1 overrides/contexts/damage/count/enchant/potion/bundle, entity-owner fields and atlas; replace data components with legacy model JSON/NBT.**
- 1.20.1 implications: Rebuild 1.20.1 overrides/contexts/damage/count/enchant/potion/bundle, entity-owner fields and atlas; replace data components with legacy model JSON/NBT.
- Follow-up checks: Compare GUI/ground/hand/frame, seeds, damage/count, glint, maps/banners/shields/skulls, culling, lighting and mod assets.
- RustChunk implementation mapping: Proposed render/items/{resolver,gui,world,entity,first_person}_1_20_1.rs with Forge namespaces, not created.
- Evidence required before adoption: 1.20.1 assets/mappings, render goldens, ItemStack NBT, model predicate traces and Forge namespace tests.
- Completion state: COMPLETE — 681 logical lines in ranges 1–400 and 401–681; SHA/26733 bytes match; tests inspected only.

### 1235. `crates/horizon-client/src/game/item_particles.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `52072f7aaca615cfb8f699fb808cb751d17766a0`
- Size: 5317 bytes; 130 logical lines
- Coverage: 1–130 (complete, EOF)
- Purpose: Resolves item particle sprites and creates eating/use particles and equipment-break feedback/sounds.
- Dependencies / cross-references: Item model resolver, ItemStack, equipment metadata, world RNG, player/mob positions, particle manager and sound resolver.
- Implementation / logic: Resolves Ground context, spawns randomized particles, handles equipment-break events and plays item-defined break sounds.
- Findings: Uses modern break_sound components and model layers; particle RNG order and equipment-break event IDs require exact 1.20.1 mapping.
- RustChunk relevance: Adapt cosmetic item feedback driven by authoritative server equipment events.
- Decision: **adaptable; rewrite required; version-specific — Use 1.20.1 model particle textures, legacy break sounds and target equipment event/slot mapping.**
- 1.20.1 implications: Use 1.20.1 model particle textures, legacy break sounds and target equipment event/slot mapping.
- Follow-up checks: Test slots, silent mobs, local/remote players, event IDs, distributions, sound pitch and missing sprites.
- RustChunk implementation mapping: Proposed render/particles/item_feedback_1_20_1.rs and network/entity_break_events_1_20_1.rs, not created.
- Evidence required before adoption: 1.20.1 models/textures, event packets, seeded particles and sound registry fixtures.
- Completion state: COMPLETE — 130 logical lines to EOF; SHA/5317 bytes match; no particle/audio test was run.

### 1236. `crates/horizon-client/src/game/item_use.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `7b5b46006ac40b458901f34385f680eafbfacef0`
- Size: 19407 bytes; 448 logical lines
- Coverage: 1–448 (complete, EOF)
- Purpose: Implements item-use duration/animation, start/tick/stop/finish hooks, consumable completion and bow charge/release math.
- Dependencies / cross-references: Player hand/inventory, consumables/effects, spyglass, bow/crossbow/trident, custom items, Forge/NeoForge events, cooldowns and first-person pose.
- Implementation / logic: Tracks used slot/time, dispatches extension callbacks, computes countdown, completes food/potions, applies cooldown and calculates bow power/pose.
- Findings: Core use timing is essential, but later brush/bundle/blocks_attacks/kinetic weapon paths and NeoForge event assumptions differ from Forge 1.20.1.
- RustChunk relevance: Essential player interaction and Forge seam; implement target vanilla timing first, then Forge 1.20.1 events.
- Decision: **essential; rewrite required; version-specific — Use 1.20.1 UseAnim, durations, bow/trident/crossbow, potion/eating and old NBT; remove modern component/new-item paths.**
- 1.20.1 implications: Use 1.20.1 UseAnim, durations, bow/trident/crossbow, potion/eating and old NBT; remove modern component/new-item paths.
- Follow-up checks: Compare finish edges, canceled/re-timed start/tick/stop, hand swap, creative use, bow thresholds, potion effects, cooldown and poses.
- RustChunk implementation mapping: Proposed game/items/use_{duration,tick,finish,release}_1_20_1.rs plus Forge event bridge, not created.
- Evidence required before adoption: Forge 1.20.1 API signatures, Java source, tick traces, NBT and projectile packet captures.
- Completion state: COMPLETE — 448 logical lines in ranges 1–400 and 401–448; SHA/19407 bytes match; tests inspected, not run.

### 1237. `crates/horizon-client/src/game/jigsaw_blocks.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4085e80887fb73f5bcf3095dae495a2285d13b7e`
- Size: 4770 bytes; 141 logical lines
- Coverage: 1–141 (complete, EOF)
- Purpose: Handles server-side jigsaw state initialization, edit packets and structure-pool generation.
- Dependencies / cross-references: Jigsaw block entity/orientation, structure pools/targets, worldgen query, generated NBT, block updates, permissions and play packets.
- Implementation / logic: Sets default joint, accepts edit fields, checks pool registry, constructs bounded-depth query, places structure and spawns entities.
- Findings: Jigsaw and pools exist in 1.20.1, but this server handler uses Horizon LAN guest switching and target permissions/pool errors/placement need comparison.
- RustChunk relevance: Reference for integrated server/packet handling; remote client sends packets instead of generating structures locally.
- Decision: **adaptable; rewrite required; version-specific — Match 1.20.1 block entity, joints, packets, pools and template placement; remove Horizon LAN dispatch.**
- 1.20.1 implications: Match 1.20.1 block entity, joints, packets, pools and template placement; remove Horizon LAN dispatch.
- Follow-up checks: Compare orientation, permissions, invalid pools, depth/keep, random choice, processors and NBT/block updates.
- RustChunk implementation mapping: Proposed game/worldgen/jigsaw_{block,editor,generation}_1_20_1.rs with Forge hooks, not created.
- Evidence required before adoption: 1.20.1 source/mappings, packet captures, template pool fixtures and seeded generation.
- Completion state: COMPLETE — 141 logical lines to EOF; SHA/4770 bytes match; no generation test was run.

### 1238. `crates/horizon-client/src/game/jigsaw_edit.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `119e3cea3187755fc3926f5f00d0fcbe793fee50`
- Size: 10795 bytes; 331 logical lines
- Coverage: 1–331 (complete, EOF)
- Purpose: Implements jigsaw editor: pool/name/target/final state/priorities/joint, depth slider and Generate action.
- Dependencies / cross-references: Block editor, jigsaw block entity, identifier parser, text widgets, joint enum and serverbound packets.
- Implementation / logic: Builds fields, validates resource identifiers, packages edit values, clamps depth to 0–20 and sends edit/generate packets.
- Findings: Useful creative UI but widget layout/focus and packet wrappers are Horizon's; final_state is not parsed here and invalid priorities default to zero.
- RustChunk relevance: Adaptable UI for server operators/map makers, not required for ordinary survival play.
- Decision: **adaptable; version-specific — Use 1.20.1 jigsaw packets, target identifier/final-state validation and RustChunk editor widgets.**
- 1.20.1 implications: Use 1.20.1 jigsaw packets, target identifier/final-state validation and RustChunk editor widgets.
- Follow-up checks: Test invalid IDs/final states, priority bounds, mobile/keyboard entry, packet order, permissions and server errors.
- RustChunk implementation mapping: Possible ui/block_editor/jigsaw_1_20_1.rs, not created.
- Evidence required before adoption: 1.20.1 screen/mappings, packet fixtures, localized UI goldens and malformed-input tests.
- Completion state: COMPLETE — 331 logical lines to EOF; SHA/10795 bytes match; editor tests inspected only.

### 1239. `crates/horizon-client/src/game/jukebox.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `03d2f5852cab025beb0364116b7b4a4dfd6788f5`
- Size: 16555 bytes; 410 logical lines
- Coverage: 1–410 (complete, EOF)
- Purpose: Implements jukebox insertion/removal, playback, level events 1010/1011, notes, parrot dancing, redstone/comparator and saving.
- Dependencies / cross-references: Jukebox block entity/record, song data, event packets, audio, parrots, RNG/particles, neighbor/comparator and LAN capture.
- Implementation / logic: Starts/stops songs on item/container interactions, ejects records, advances ticks, emits notes, updates parrots and derives comparator/signal.
- Findings: Core jukebox behavior is target-era, but code uses later jukebox_playable/component song data, dynamic registry and Horizon LAN capture; 1.20.1 record items and event mapping differ.
- RustChunk relevance: Essential world/audio interaction and remote level-event handling; rewrite item/song data and packets.
- Decision: **essential; rewrite required; version-specific — Use 1.20.1 RecordItem/sound IDs, block entity, level events 1010/1011, comparator and parrot; omit later components/registry.**
- 1.20.1 implications: Use 1.20.1 RecordItem/sound IDs, block entity, level events 1010/1011, comparator and parrot; omit later components/registry.
- Follow-up checks: Compare insertion/ejection, hopper/dispenser, start/end/stop packets, note RNG, parrot radius, comparator and save/reload.
- RustChunk implementation mapping: Proposed game/blocks/jukebox_1_20_1.rs and network/level_events/jukebox_1_20_1.rs, not created.
- Evidence required before adoption: 1.20.1 source/mappings, record fixtures, event packets, NBT, seeded particles and comparator tests.
- Completion state: COMPLETE — 410 logical lines in ranges 1–200, 201–400 and 401–410; SHA/16555 bytes match; no playback/world tests.

### 1240. `crates/horizon-client/src/game/key_focus.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a87464b19721a98c309e4650d642e8ad9fba2afb`
- Size: 19617 bytes; 518 logical lines
- Coverage: 1–518 (complete, EOF)
- Purpose: Implements keyboard/controller focus navigation for buttons, fields, rows, tabs and sliders.
- Dependencies / cross-references: Winit KeyCode, gamepad D-pad, widget target geometry, GUI dimensions, drawn widgets and container key handling.
- Implementation / logic: Maps Tab/Shift+Tab, arrows and Enter/Space into focus actions; sorts by geometry, groups list rows, navigates from screen edges and draws outlines.
- Findings: Useful for accessibility, but documented against 26.3 focus semantics and Horizon widget model; Android touch/accessibility may differ.
- RustChunk relevance: Optional UI/accessibility reference; preserve keyboard/gamepad navigation without importing current widgets.
- Decision: **adaptable; reference; optional/future — Implement target screen/container focus and Android accessibility; preserve Tab/hotbar conflicts and slider/edit behavior.**
- 1.20.1 implications: Implement target screen/container focus and Android accessibility; preserve Tab/hotbar conflicts and slider/edit behavior.
- Follow-up checks: Test tab wrap, arrows, caret, controller, slider, inventory key conflicts, screen changes and TalkBack focus.
- RustChunk implementation mapping: Possible ui/focus/navigation.rs as optional accessibility feature, not created.
- Evidence required before adoption: 1.20.1 Screen/AbstractWidget source, key traces, geometry and Android accessibility/controller tests.
- Completion state: COMPLETE — 518 logical lines in ranges 1–400 and 401–518; SHA/19617 bytes match; tests inspected only.

### 1241. `crates/horizon-client/src/game/kinetic_machines.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4e229a928f7f87fc44a58607230d2d61eb1175c4`
- Size: 41962 bytes; 1024 logical lines
- Coverage: 1–400; 401–600; 601–800; 801–1024 (complete, EOF)
- Purpose: Implements Create kinetic-machine entity behavior and integration for moving mechanical blocks and machines.
- Dependencies / cross-references: Create registries and block entities, kinetic speed/rotation state, moving contraptions, inventories/fluids/energy, chunk ticking, collision and renderer.
- Implementation / logic: Updates custom machine entities, movement and rotation, and their interactions with kinetic networks and machine state.
- Findings: Entirely Create mod functionality, coupled to a specific Create API, registries and data model; not vanilla behavior and not directly compatible by assumption with Forge 1.20.1.
- RustChunk relevance: Exclude from core client; consider only as a later Create-for-1.20.1 adapter after vanilla and Forge mod boundaries exist.
- Decision: **unnecessary; optional/future — Do not register these entity or block IDs in vanilla 1.20.1. Pin the precise Create 1.20.1 release and rebuild entity, block-entity, capability and render adapters.**
- 1.20.1 implications: Do not register these entity or block IDs in vanilla 1.20.1. Pin the precise Create 1.20.1 release and rebuild entity, block-entity, capability and render adapters.
- Follow-up checks: Compare target Create source/license, registry IDs, NBT, kinetic tick order, chunk lifecycle, capability/menu sync and renderer.
- RustChunk implementation mapping: No core mapping; possible mods/create/kinetic_machines_1_20_1.rs after mod-adapter infrastructure, not created.
- Evidence required before adoption: Exact Create 1.20.1 source/API, Forge fixtures, NBT snapshots, deterministic kinetic traces, mod assets and license review.
- Completion state: COMPLETE — 1024 lines to EOF in contiguous ranges 1–400, 401–600, 601–800 and 801–1024; SHA and 41962-byte size match; no Create runtime run.

### 1242. `crates/horizon-client/src/game/kinetics.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6b71bc90ba550a480a28568fa4edbd783030fbfe`
- Size: 61786 bytes; 1442 logical lines
- Coverage: 1–400; 401–800; 801–1200; 1201–1400; 1401–1442 (complete, EOF)
- Purpose: Implements Create kinetic network propagation, connected rotational networks, stress/speed calculation and mechanical movement.
- Dependencies / cross-references: Create kinetic block entities, shaft/gear connectivity, block states, network graph/IDs, stress capacity, rotation speed, chunk updates and renderer.
- Implementation / logic: Builds, merges and splits connected kinetic networks, propagates speed/stress through components, handles source changes and updates rotational state.
- Findings: Entirely Create-specific mechanics. Graph algorithms are generic architecture references, but stress units, block topology and tick semantics are mod-defined.
- RustChunk relevance: Exclude from vanilla baseline; possible future mod adapter only.
- Decision: **unnecessary; reference; optional/future — No vanilla 1.20.1 registry or protocol behavior. Keep the graph and stress model behind a version-pinned Create/Forge adapter.**
- 1.20.1 implications: No vanilla 1.20.1 registry or protocol behavior. Keep the graph and stress model behind a version-pinned Create/Forge adapter.
- Follow-up checks: Compare graph merge/split, chunk unload/reload, source rotation, overstress, speed caps, block removal and deterministic tick order against Create 1.20.1.
- RustChunk implementation mapping: No core mapping; possible mods/create/kinetic_network_1_20_1.rs, not created.
- Evidence required before adoption: Create 1.20.1 source, graph property tests, seeded network fixtures, stress/speed traces and Android performance measurements.
- Completion state: COMPLETE — 1442 lines to EOF in contiguous ranges 1–400, 401–800, 801–1200, 1201–1400 and 1401–1442; SHA and 61786-byte size match; no network simulations run.

### 1243. `crates/horizon-client/src/game/lan_actions.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e567c82690937f4f2cfa2da5ebd3d595c2b9ea6e`
- Size: 235964 bytes; 5365 logical lines
- Coverage: 1–400; 401–800; 801–1200; 1201–1600; 1601–2000; 2001–2400; 2401–2800; 2801–3200; 3201–3600; 3601–4000; 4001–4400; 4401–4800; 4801–5200; 5201–5365 (complete, EOF)
- Purpose: Routes actions from Horizon LAN guests to the local host world: movement, interactions, attacks, block breaking, item use, menus, chat, commands and respawn.
- Dependencies / cross-references: LAN Hub/guest identity, server Game/player state, entity/mob systems, menu engine, command permissions, inventory, world mutation, play packets and network limits.
- Implementation / logic: Temporarily applies a guest state as acting player, executes server-side handlers, captures resulting events/updates and restores host state; dispatches incoming play packets into server gameplay paths.
- Findings: This is a LAN-host server packet listener, not a remote multiplayer client connection. Guest-state swapping, host mutation and LAN session assumptions must not be transplanted into RustChunk's Aternos client path.
- RustChunk relevance: Exclude from runtime; use only as server-side behavior reference for packet semantics and authority boundaries.
- Decision: **unnecessary; reference — Implement the 1.20.1 client protocol directly: send client actions and consume server-authoritative corrections. Do not impersonate or mutate a local host player.**
- 1.20.1 implications: Implement the 1.20.1 client protocol directly: send client actions and consume server-authoritative corrections. Do not impersonate or mutate a local host player.
- Follow-up checks: Cross-check each packet family against 1.20.1: movement/vehicle, interact/attack, digging, use item, container click, chat/command, respawn and acknowledgements.
- RustChunk implementation mapping: No runtime mapping; possible research/server_packet_semantics/lan_actions_reference.md; remote-session code must not depend on it.
- Evidence required before adoption: 1.20.1 packet schemas, client/server traces, permission cases, correction packets and Java-client differential fixtures.
- Completion state: COMPLETE — 5365 lines to EOF in 14 contiguous chunks from 1–400 through 4801–5200, then 5201–5365; SHA and 235964-byte size match; no server action or LAN session run.

### 1244. `crates/horizon-client/src/game/lan_attributes.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8c082d982a3f986590767e335024056a9ddc97e4`
- Size: 13076 bytes; 336 logical lines
- Coverage: 1–120; 121–336 (complete, EOF)
- Purpose: Builds server-synchronized attribute snapshots for mobs, armor stands and players, including item/effect modifiers and creative reach changes.
- Dependencies / cross-references: Attribute registry, mob defaults, effect modifiers, equipment modifiers, slot groups, /attribute command state, entity metadata and UpdateAttributes packet.
- Implementation / logic: Creates existing attribute instances, applies base values and equipment/effect modifiers in slot order, then emits synchronized snapshots.
- Findings: Useful attribute packet reference, but uses newer block_interaction_range and entity_interaction_range attributes and modern slot groups; these IDs must not enter 1.20.1.
- RustChunk relevance: Adapt client-side attribute-map updates and modifier evaluation, while server-sent values remain authoritative.
- Decision: **adaptable; rewrite required; version-specific — Use only 1.20.1 attribute registry IDs/defaults/operations and legacy item modifiers. Do not assume modern reach attributes exist in 1.20.1.**
- 1.20.1 implications: Use only 1.20.1 attribute registry IDs/defaults/operations and legacy item modifiers. Do not assume modern reach attributes exist in 1.20.1.
- Follow-up checks: Compare 1.20.1 UpdateAttributes fields, modifier UUIDs/operations, equipment changes, potion add/remove, horse jump, armor stands and respawn.
- RustChunk implementation mapping: Proposed network/attributes/update_1_20_1.rs and game/attributes/modifiers_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 attribute registry/mappings, packet captures, target ItemStack NBT, modifier snapshots and effect/equipment traces.
- Completion state: COMPLETE — 336 lines read in 1–120 and 121–336; SHA and 13076-byte size match; embedded slot-group test inspected, not run.

### 1245. `crates/horizon-client/src/game/lan_camera.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8c64cf2bb8ac075c2869bfda37a4071862d157d3`
- Size: 10043 bytes; 262 logical lines
- Coverage: 1–120; 121–262 (complete, EOF)
- Purpose: Implements spectator guest camera selection, target tracking and reset when the camera dies, the player sneaks, leaves spectator mode or teleports.
- Dependencies / cross-references: Spectator state, entity IDs/targets, hitboxes, pickability, reach, /spectate, SetCamera packet, entity movement and dimensions.
- Implementation / logic: Resolves clicked/commanded camera entity, validates range/border/pickability, sends SetCamera and follows entity until a reset condition occurs.
- Findings: Spectator camera and SetCamera packet behavior are valuable target references. This file is server-side LAN bookkeeping and includes newer entity types.
- RustChunk relevance: Adapt client behavior to consume camera-target updates and restore player view on server update; server remains authoritative.
- Decision: **adaptable; reference; version-specific — Use 1.20.1 SetCamera and supported entity roster; remove later projectile and Horizon guest-camera state. Validate camera target removal and teleport.**
- 1.20.1 implications: Use 1.20.1 SetCamera and supported entity roster; remove later projectile and Horizon guest-camera state. Validate camera target removal and teleport.
- Follow-up checks: Test camera changes to mobs/players/vehicles, entity despawn, spectator exit, sneak, teleport, dimension switch and invalid IDs.
- RustChunk implementation mapping: Proposed network/play/set_camera_1_20_1.rs and render/camera/spectator_target.rs; not created.
- Evidence required before adoption: 1.20.1 SetCamera packet/mappings, tracking fixtures, spectator traces and camera-render goldens.
- Completion state: COMPLETE — 262 lines read in 1–120 and 121–262; SHA and 10043-byte size match; embedded lifecycle test inspected, not run.

### 1246. `crates/horizon-client/src/game/lan_chat.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a1d8720b9530afd27b89e23340f85423733df8b0`
- Size: 9907 bytes; 242 logical lines
- Coverage: 1–120; 121–242 (complete, EOF)
- Purpose: Handles LAN guest signed/unsigned chat commands, signed arguments, last-seen acknowledgements, secure-chat validation and spam throttling.
- Dependencies / cross-references: Chat command packet types, signature chain, last-seen acknowledgements, command parser/signable ranges, permission, broadcast/whisper and disconnect.
- Implementation / logic: Parses commands at guest permission, verifies signable arguments, consumes signatures/acks, routes say/me/msg and applies spam thresholds.
- Findings: Secure chat and signed commands exist in target-era protocol; this is server-side and its payloads/signing context must match exactly 1.20.1. Not a client signing implementation.
- RustChunk relevance: Essential protocol domain; use as behavioral reference while implementing client signing and acknowledgements independently.
- Decision: **essential; rewrite required; version-specific — Implement exact 1.20.1 chat session init, message signatures, signed command arguments, last-seen and acknowledgement packets; exclude server kick policy.**
- 1.20.1 implications: Implement exact 1.20.1 chat session init, message signatures, signed command arguments, last-seen and acknowledgement packets; exclude server kick policy.
- Follow-up checks: Compare codecs, signature canonicalization, timestamp/salt, last-seen window, command ranges, disconnect semantics and unsigned-server compatibility.
- RustChunk implementation mapping: Proposed network/chat/session_signing_last_seen_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 protocol/mappings, signed chat captures, deterministic signature fixtures and acknowledgement traces.
- Completion state: COMPLETE — 242 lines read in 1–120 and 121–242; SHA and 9907-byte size match; no chat session or connection run.

### 1247. `crates/horizon-client/src/game/lan_crafting.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9ab819fffe7c50c8d4136a86e58f29e6aad15856`
- Size: 29151 bytes; 729 logical lines
- Coverage: 1–400; 401–729 (complete, EOF)
- Purpose: Implements LAN guest crafting and container operations with the host menu engine, including crafting and furnace-like menus.
- Dependencies / cross-references: Guest inventory, menu engine, recipes, container slots/data, block entities, crafting inputs/results, click actions, window IDs and container packets.
- Implementation / logic: Creates server menus, applies click/button/quick-move operations to authoritative state, updates result slots and sends inventory/container changes.
- Findings: Menu/crafting packet concepts matter to the 1.20.1 client, but this is a server-side implementation using Starling's current recipe/menu internals.
- RustChunk relevance: Adaptable client menu and slot/state-ID reference; do not port server menu execution into the client.
- Decision: **adaptable; rewrite required; version-specific — Use 1.20.1 menu types, slot layouts, container state IDs, recipe-book packets and legacy ItemStack NBT; accept server corrections as authoritative.**
- 1.20.1 implications: Use 1.20.1 menu types, slot layouts, container state IDs, recipe-book packets and legacy ItemStack NBT; accept server corrections as authoritative.
- Follow-up checks: Build menu matrix for crafting, furnace, blast furnace, smoker, brewing, inventory and recipe book; verify slot indices, shift-click, button/data and cursor updates.
- RustChunk implementation mapping: Proposed ui/menus/crafting_furnace_brewing_1_20_1.rs and network/play/container_click_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 Java menu source, packet traces, slot snapshots, recipes and legacy ItemStack NBT.
- Completion state: COMPLETE — 729 lines read in 1–400 and 401–729; SHA and 29151-byte size match; no menu tests run.

### 1248. `crates/horizon-client/src/game/lan_entities.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `fc5cce5f058b622e7df773f3bae50db8260006ff`
- Size: 34957 bytes; 853 logical lines
- Coverage: 1–400; 401–800; 801–853 (complete, EOF)
- Purpose: Builds and serializes tracked non-mob entities including boats, minecarts, armor stands, item frames and paintings, plus passenger/leash relationships.
- Dependencies / cross-references: Entity type registry, EntityState/Riding, ID/UUID allocation, metadata serializers, velocity/pose/equipment, leashes, block state and tracking lifecycle.
- Implementation / logic: Creates spawn states and metadata deltas, allocates identities, forms passenger links, serializes non-default data and reports tracking start/stop.
- Findings: Highly valuable entity protocol evidence, but generated by a modern server with later types and current metadata conventions. IDs, serializers, defaults and packet order differ by game version.
- RustChunk relevance: Essential reference for RustChunk entity model/decoder: spawn, movement, metadata, equipment, passengers, leashes and removal require exact target handling.
- Decision: **essential; rewrite required; version-specific — Rebuild 1.20.1 entity roster, spawn codecs, metadata indices/serializer types/defaults and passenger/leash/equipment packets; exclude newer entity IDs.**
- 1.20.1 implications: Rebuild 1.20.1 entity roster, spawn codecs, metadata indices/serializer types/defaults and passenger/leash/equipment packets; exclude newer entity IDs.
- Follow-up checks: Create per-entity 1.20.1 metadata tables; test spawn/remove, velocity/teleport, metadata reset, passengers, leash, stands, frames, boats, carts and dragon parts.
- RustChunk implementation mapping: Proposed network/entities/{spawn,metadata,relationships,tracking}_1_20_1.rs and game/entity_registry_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 entity registry/mappings, packet captures, metadata serializer fixtures and Forge entity extension contract.
- Completion state: COMPLETE — 853 lines read in 1–400, 401–800 and 801–853; SHA and 34957-byte size match; tracking tests not run.

### 1249. `crates/horizon-client/src/game/lan_far.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `84bb02ad267bbb8c3df3f5ee6f12a23bfd28eb62`
- Size: 3080 bytes; 79 logical lines
- Coverage: 1–79 (complete, EOF)
- Purpose: Provides Horizon-only far-terrain requests over horizon:lod custom payload, samples distant terrain on a worker and returns bounded replies.
- Dependencies / cross-references: Horizon LOD codec/channel, player position/dimension, sampler worker, connection backlog, custom-payload packet and client brand.
- Implementation / logic: Validates Horizon client requests, serves/refuses/ignores/kicks, queues sampling jobs and applies backpressure above a 1 MiB pending-send backlog.
- Findings: Proprietary extension, not vanilla chunk protocol and not a replacement for nearby chunk loading. Cannot solve missing main chunks without correct regular chunk streaming.
- RustChunk relevance: Optional performance reference only; keep authoritative nearby chunk decoding/render readiness independent.
- Decision: **unnecessary; optional/future — Do not require horizon:lod on ordinary 1.20.1 servers. Decode standard chunk/section packets and prioritize nearby chunks first.**
- 1.20.1 implications: Do not require horizon:lod on ordinary 1.20.1 servers. Decode standard chunk/section packets and prioritize nearby chunks first.
- Follow-up checks: If optional LOD is explored, bound requests/bytes, validate coordinates/dimension, cancel on unload/disconnect and prove nearby chunks remain authoritative.
- RustChunk implementation mapping: Possible optional render/lod/horizon_extension.rs after negotiated capability, not created.
- Evidence required before adoption: Custom payload spec, worker/backlog tests, chunk-distance correctness and Android latency/memory profiles.
- Completion state: COMPLETE — 79 logical lines read to EOF; SHA and 3080-byte size match; no LOD request or worker run.

### 1250. `crates/horizon-client/src/game/lan_mobs.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5b83f8e0b179aef3b6553b9ba27f45a51a010b39`
- Size: 20265 bytes; 545 logical lines
- Coverage: 1–400; 401–545 (complete, EOF)
- Purpose: Chooses which player a mob targets in a hosted LAN world and builds player snapshots for host/guests, including effects, equipment and combat attribution.
- Dependencies / cross-references: Mob AI/anger, LAN Hub guest roster, player pose/bounds/inventory/effects, dimension, team/allies and damage attribution.
- Implementation / logic: Builds guest PlayerView snapshots, chooses eligible survival targets or maintains attacker/angry targets, and derives omen/effect state.
- Findings: Server-side LAN mob AI rather than client entity rendering; includes Trial Omen state newer than 1.20.1. Remote client must not authoritatively choose mob targets.
- RustChunk relevance: Exclude from remote client; a future integrated server may use it only as behavior reference after target-version validation.
- Decision: **unnecessary; reference; optional/future — Remote client consumes server-sent mob state; do not run target selection locally. Remove Trial Omen and later effect assumptions from 1.20.1.**
- 1.20.1 implications: Remote client consumes server-sent mob state; do not run target selection locally. Remove Trial Omen and later effect assumptions from 1.20.1.
- Follow-up checks: For future integrated AI, compare target priority, anger/last-hurt, creative/spectator filtering, teams, dimensions and target effect roster.
- RustChunk implementation mapping: No client mapping; possible future game/server_ai/target_players_1_20_1.rs only for integrated server, not created.
- Evidence required before adoption: 1.20.1 mob AI source, deterministic target traces, anger timers, UUID and entity metadata fixtures.
- Completion state: COMPLETE — 545 lines read in 1–400 and 401–545; SHA and 20265-byte size match; embedded AI tests inspected, not run.

### 1251. `crates/horizon-client/src/game/lan_mount_menu.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d63ac07471237491dca2d51370162ea8c14fb762`
- Size: 9211 bytes; 232 logical lines
- Coverage: 1–232 (complete, EOF)
- Purpose: Implements a guest horse/mount inventory menu, slot sync, click handling, mount inventory and validity checks.
- Dependencies / cross-references: Horse/donkey/mule/llama/camel/nautilus mount inventory, saddle/body/chest slots, cursor stack, menu engine, OpenScreen and container packets.
- Implementation / logic: Opens mount menu after range/permission checks, constructs slots, applies server-side clicks, stores mount inventory and closes on invalid mount/range.
- Findings: Mount menus and container sync are relevant. File is server-side and also has later Nautilus support; current menu/window handling is not a 1.20.1 client implementation.
- RustChunk relevance: Adapt client UI/packet handling for target horse-family mounts; useful slot-layout reference.
- Decision: **adaptable; rewrite required; version-specific — Use 1.20.1 horse, donkey, mule, llama and camel menus/slot order; exclude Nautilus and match target horse screen packet and legacy item NBT.**
- 1.20.1 implications: Use 1.20.1 horse, donkey, mule, llama and camel menus/slot order; exclude Nautilus and match target horse screen packet and legacy item NBT.
- Follow-up checks: Test target mount types, saddle/armor/chest slots, shift-click, cursor recovery, mount death/range close and server correction.
- RustChunk implementation mapping: Proposed ui/menus/mount_inventory_1_20_1.rs and network/play/horse_screen_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 Java mount menu source, horse packet captures, slot fixtures and GUI tests.
- Completion state: COMPLETE — 232 logical lines read to EOF; SHA and 9211-byte size match; no menu session run.

### 1252. `crates/horizon-client/src/game/lan_particles.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `20efad5dfb17ba21a70b6dece9285a02d5000947`
- Size: 33025 bytes; 884 logical lines
- Coverage: 1–400; 401–800; 801–884 (complete, EOF)
- Purpose: Constructs and sends server particle packets near LAN guests, including option serialization, spread/randomization, effects and explosion particles.
- Dependencies / cross-references: Particle type/options registry, block states, RandomSource, LevelParticles/Explosion packets, guest range and mod-particle payload.
- Implementation / logic: Builds particle payloads, samples positions/velocities via Gaussian or uniform spread, selects recipients by dimension/range and restricts unsupported mod IDs to Horizon clients.
- Findings: Useful particle packet reference, but includes later randomization fields and custom mod IDs; unsupported negative or mod IDs must never reach vanilla clients.
- RustChunk relevance: Adapt client packet decoder and particle renderer, not the server fanout logic.
- Decision: **adaptable; rewrite required; version-specific — Decode exactly 1.20.1 LevelParticles/Explosion fields and supported particle codecs. Keep unknown mod payloads isolated behind negotiated Forge channels.**
- 1.20.1 implications: Decode exactly 1.20.1 LevelParticles/Explosion fields and supported particle codecs. Keep unknown mod payloads isolated behind negotiated Forge channels.
- Follow-up checks: Compare every target particle serializer, item/block payload, count-zero semantics, spread/velocity, explosion burst and long-distance flag.
- RustChunk implementation mapping: Proposed network/play/level_particles_1_20_1.rs and render/particles/registry_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 protocol schema, particle registry fixtures, packet captures and deterministic render goldens.
- Completion state: COMPLETE — 884 lines read in 1–400, 401–800 and 801–884; SHA and 33025-byte size match; particle tests not run.

### 1253. `crates/horizon-client/src/game/lan_pvp.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `539b2f478e2e56fb00318e70975d3d40e9d39979`
- Size: 19326 bytes; 490 logical lines
- Coverage: 1–400; 401–490 (complete, EOF)
- Purpose: Implements server-side PvP melee/projectile damage including critical/sweep, shields, armor, knockback, teams and PvP rule.
- Dependencies / cross-references: LAN guest identities, PvP settings, teams, attack strength, weapons/enchantments, damage sources, projectiles, shields, armor/effects and movement.
- Implementation / logic: Checks PvP eligibility, calculates strike/critical/knockback/sweep and resolves arrows/thrown projectiles hitting another player.
- Findings: Authoritative server combat logic; includes later mace smash. Remote client must not treat local damage calculation as final or apply authoritative HP changes.
- RustChunk relevance: Exclude from client runtime; reference only for future integrated server and understanding server corrections.
- Decision: **unnecessary; reference; optional/future — Do not simulate authoritative PvP damage on client. Render server-confirmed health, velocity, hurt animation and equipment; exclude mace-specific branches.**
- 1.20.1 implications: Do not simulate authoritative PvP damage on client. Render server-confirmed health, velocity, hurt animation and equipment; exclude mace-specific branches.
- Follow-up checks: For integrated server, compare cooldown, critical/sweep, projectile hit order, shield disable, armor/enchant modifiers, team/friendly fire and damage packets.
- RustChunk implementation mapping: No remote-client mapping; possible separate game/server_combat/pvp_1_20_1.rs for integrated server, not created.
- Evidence required before adoption: 1.20.1 Java combat source, seeded hit traces, damage/velocity packets and shield/armor fixtures.
- Completion state: COMPLETE — 490 lines read in 1–400 and 401–490; SHA and 19326-byte size match; combat tests not run.

### 1254. `crates/horizon-client/src/game/lan_quiet.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e893ed5610d690c81d941755047a4b9d94dca130`
- Size: 6209 bytes; 176 logical lines
- Coverage: 1–176 (complete, EOF)
- Purpose: Implements Horizon horizon:quiet_commands batch API for mods, queueing quiet commands with per-player limits and suppressed output.
- Dependencies / cross-references: Horizon custom payload decoder, guest permissions, command dispatcher, mod hooks, queued batches and server logs.
- Implementation / logic: Accepts batches, caps pending lines at 4096 and execution at 64 lines per player/tick, refuses signed chat commands and executes without chat output.
- Findings: Proprietary automation behavior. Not a vanilla packet and must never be implicitly enabled on ordinary multiplayer servers.
- RustChunk relevance: Exclude baseline; bounded queue ideas only for explicit mod protocol later.
- Decision: **unnecessary; optional/future — Do not send horizon:quiet_commands to ordinary 1.20.1 servers. Forge custom channels require exact negotiation and permission gates.**
- 1.20.1 implications: Do not send horizon:quiet_commands to ordinary 1.20.1 servers. Forge custom channels require exact negotiation and permission gates.
- Follow-up checks: If future support desired, audit authentication, line length, permissions, rate limits, disconnect cleanup, fairness and capability negotiation.
- RustChunk implementation mapping: Possible opt-in mods/horizon/quiet_commands.rs, not created.
- Evidence required before adoption: Custom payload spec, threat model, permission matrix, bounded queue tests and negotiation fixtures.
- Completion state: COMPLETE — 176 logical lines to EOF; SHA and 6209-byte size match; embedded queue test inspected, not run.

### 1255. `crates/horizon-client/src/game/lan_screen.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `56eb7d47602dc4e717afc7c3fcd984cbf28da22f`
- Size: 75611 bytes; 1879 logical lines
- Coverage: 1–400; 401–800; 801–1200; 1201–1600; 1601–1879 (complete, EOF)
- Purpose: Implements the 26.3 WorldOptionsScreen with modes, cheats, difficulty lock, game rules, restrictions and modern Open-to-LAN controls.
- Dependencies / cross-references: Client screen/widgets, integrated server options, LAN settings, game rules, difficulty, command access, port input, restrictions and serverbound settings packets.
- Implementation / logic: Builds options UI with host-only/remote sections, validates port, snapshots edits and applies hosting/configuration changes.
- Findings: Explicit 26.3 replacement for the old Open-to-LAN screen; not the 1.20.1 UI and belongs to hosting rather than ordinary remote server play.
- RustChunk relevance: Exclude initial client baseline; optional UX reference if integrated world hosting is later implemented.
- Decision: **unnecessary; optional/future — Do not reuse 26.3 buttons, settings or packet payloads. Rebuild the old 1.20.1 options/Open-to-LAN flow only if host mode is required.**
- 1.20.1 implications: Do not reuse 26.3 buttons, settings or packet payloads. Rebuild the old 1.20.1 options/Open-to-LAN flow only if host mode is required.
- Follow-up checks: Compare target 1.20.1 options screen, host permissions, port, difficulty lock, gamerules, LAN publication and Android touch controls.
- RustChunk implementation mapping: Possible ui/world_options/open_to_lan_1_20_1.rs if hosting is later required, not created.
- Evidence required before adoption: 1.20.1 screen source/mappings, LAN host API, settings packet captures, port-boundary tests and UI goldens.
- Completion state: COMPLETE — 1879 lines to EOF in 1–400, 401–800, 801–1200, 1201–1600 and 1601–1879; SHA and 75611-byte size match; no UI tests run.

### 1256. `crates/horizon-client/src/game/lan_stations.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b18a64b0b56b95c010d22c8a554c4f968073627e`
- Size: 35160 bytes; 879 logical lines
- Coverage: 1–400; 401–800; 801–879 (complete, EOF)
- Purpose: Implements LAN guest work-station and lectern menus, including open, slots, data sync, button clicks and inventory return on close.
- Dependencies / cross-references: MenuType registry, block entities, recipes, inventory, lectern book/pages, container IDs, OpenScreen, ContainerClick, SetContent and data packets.
- Implementation / logic: Opens station menus, applies authoritative click operations, syncs changed slots/data and returns carried/input stacks when menu closes.
- Findings: Client menu/packet families matter, but this is server-side Starling menu logic. Slot layouts and smithing/crafter behavior must be checked against 1.20.1; newer Crafter menu does not belong in target.
- RustChunk relevance: Adaptable client menu and slot-synchronization reference; rebuild target screens and codecs.
- Decision: **adaptable; rewrite required; version-specific — Use 1.20.1 menu types/slots for loom, stonecutter, grindstone, anvil, smithing, enchanting, beacon, cartography and lectern; exclude post-target Crafter.**
- 1.20.1 implications: Use 1.20.1 menu types/slots for loom, stonecutter, grindstone, anvil, smithing, enchanting, beacon, cartography and lectern; exclude post-target Crafter.
- Follow-up checks: Test menu open/close, slot/state IDs, button clicks, quick move, lectern pages, beacon, smithing template slot, invalid windows and cursor correction.
- RustChunk implementation mapping: Proposed ui/menus/{loom,stonecutter,grindstone,anvil,smithing,enchanting,beacon,cartography,lectern}_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 menu source, packet traces, slot snapshots, recipe fixtures, legacy ItemStack NBT and UI goldens.
- Completion state: COMPLETE — 879 lines to EOF in 1–400, 401–800 and 801–879; SHA and 35160-byte size match; no menu tests run.

### 1257. `crates/horizon-client/src/game/lan_trading.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `bf94369c80ff12ca680aaaeebcee1e6816e67ee2`
- Size: 17384 bytes; 465 logical lines
- Coverage: 1–400; 401–465 (complete, EOF)
- Purpose: Implements villager and wandering trader menus, offer synchronization, selecting trades, payment slots, XP and reputation.
- Dependencies / cross-references: Villager/trader state, MerchantOffers, reputation/gossip, player UUID, menu, inventory, trade packets, ItemStacks and updates.
- Implementation / logic: Creates merchant session, sends offers/level, handles selection and container clicks, executes trade and updates slots/XP.
- Findings: Merchant screen and offer packets are core client behavior, but this is server authoritative and uses modern item/merchant models; offer fields/NBT/packet IDs must match target.
- RustChunk relevance: Adaptable reference for merchant UI, trade selection and synchronized result/payment slots; server owns trade and reputation.
- Decision: **adaptable; rewrite required; version-specific — Implement 1.20.1 MerchantOffers/MerchantOffer codecs, container state/content, trade selection, villager level/XP and legacy item NBT.**
- 1.20.1 implications: Implement 1.20.1 MerchantOffers/MerchantOffer codecs, container state/content, trade selection, villager level/XP and legacy item NBT.
- Follow-up checks: Compare disabled/locked offers, uses/maxUses, demand/special price, XP/level, restock, payment slots, quick move and result correction.
- RustChunk implementation mapping: Proposed ui/menus/merchant_1_20_1.rs and network/play/merchant_offers_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 merchant/menu source, offer packet captures, villager NBT, price fixtures and UI goldens.
- Completion state: COMPLETE — 465 lines in 1–400 and 401–465; SHA and 17384-byte size match; no trade session run.

### 1258. `crates/horizon-client/src/game/lan_vehicles.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d3cf0cafea56f0da1df387f2b0572009b88d7025`
- Size: 22898 bytes; 564 logical lines
- Coverage: 1–400; 401–564 (complete, EOF)
- Purpose: Handles server-side LAN guest driving of boats/minecarts, rider input, paddling, dismount and vehicle movement events.
- Dependencies / cross-references: Boat/minecart state and physics, input flags, collisions, passenger relations, guest position, teleport/dismount and vehicle callbacks.
- Implementation / logic: Maps player input flags to keys, simulates vehicle movement for riders, tracks movement and resolves dismount destinations.
- Findings: Vehicle input/packet concepts matter for target client, but this is the server authority path and uses Horizon guest state. New entity/input kinds require target validation.
- RustChunk relevance: Adaptable reference for outbound vehicle input and client prediction; remote server remains authoritative for position/collision.
- Decision: **adaptable; rewrite required; version-specific — Use exact 1.20.1 steer-boat, paddle and move-vehicle packets, target vehicle roster and server correction; do not simulate host guest state locally.**
- 1.20.1 implications: Use exact 1.20.1 steer-boat, paddle and move-vehicle packets, target vehicle roster and server correction; do not simulate host guest state locally.
- Follow-up checks: Compare paddle flags, input ordering, minecart movement, passenger packets, dismount search, collision and correction under latency.
- RustChunk implementation mapping: Proposed network/play/{steer_boat,move_vehicle}_1_20_1.rs and game/vehicles/client_prediction.rs; not created.
- Evidence required before adoption: 1.20.1 vehicle source/mappings, outbound packet traces, passenger/movement packets and deterministic collision fixtures.
- Completion state: COMPLETE — 564 lines in 1–400 and 401–564; SHA and 22898-byte size match; no vehicle test or session run.

### 1259. `crates/horizon-client/src/game/lan_waypoints.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4481d49fb94b42542e9701e2950db204565e24ce`
- Size: 4063 bytes; 98 logical lines
- Coverage: 1–98 (complete, EOF)
- Purpose: Maintains per-dimension player locator-bar waypoints for hosted world, including team colors, hide conditions, range and respawn tracking.
- Dependencies / cross-references: Horizon waypoint manager, UUID/dimension/position, scoreboard teams, crouching/invisibility/head equipment, view distance and custom updates.
- Implementation / logic: Builds player snapshots, chooses transmit/receive ranges, hides players based on pose/effects/head item, ticks managers and sends waypoint updates.
- Findings: Horizon-specific later locator/waypoint server feature, not vanilla 1.20.1 client functionality. Its range/visibility policy is custom.
- RustChunk relevance: Exclude baseline. Future optional waypoint UI must use negotiated compatible server support or local-only markers.
- Decision: **unnecessary; optional/future — Do not require/send Horizon waypoint payloads to ordinary 1.20.1 servers; use only target packets or separately negotiated Forge/mod channels.**
- 1.20.1 implications: Do not require/send Horizon waypoint payloads to ordinary 1.20.1 servers; use only target packets or separately negotiated Forge/mod channels.
- Follow-up checks: For future waypoint work, verify protocol, permissions, visibility, teams, teleport/dimension lifecycle and hidden-player privacy.
- RustChunk implementation mapping: Possible optional ui/waypoints/compat.rs behind capability negotiation, not created.
- Evidence required before adoption: Custom waypoint spec, capability negotiation, privacy/visibility tests and target-version feature check.
- Completion state: COMPLETE — 98 logical lines to EOF; SHA and 4063-byte size match; no waypoint manager run.

### 1260. `crates/horizon-client/src/game/lan.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `dda3e7a1da90209d6e40b40eed21018b5c90fc25`
- Size: 129988 bytes; 2982 logical lines
- Coverage: 1–400; 401–800; 801–1000; 1001–1200; 1201–1400; 1401–1600; 1601–2000; 2001–2200; 2201–2400; 2401–2600; 2601–2800; 2801–2982 (complete, EOF)
- Purpose: Hosts an integrated world on LAN, manages Hub/server threads, syncs registries/tags/packs, streams chunks/entities and dispatches guest events.
- Dependencies / cross-references: LAN Hub/network threads, chunk encoder, SyncData/registries/tags, entity serializers, player data, mobs/vehicles/menus/particles/chat/commands, dimensions and world lifecycle.
- Implementation / logic: Starts/stops LAN host, prepares/reloads sync data, preloads registries, ticks Hub, streams world/entity/chunk state, processes guest join/leave and coordinates server subsystems.
- Findings: Horizon LAN host core, not a remote Minecraft client. Contains custom registry/dialog/LOD assumptions, server ownership and guest state. Copying it conflates server/client authority and does not help Aternos connectivity.
- RustChunk relevance: Exclude remote client baseline; use as a map of server packet families only if an integrated server is later considered.
- Decision: **unnecessary; reference; optional/future — Implement a distinct 1.20.1 remote handshake/configuration/play protocol. Do not start a LAN Hub/server thread or Horizon custom registry sync when joining an external server.**
- 1.20.1 implications: Implement a distinct 1.20.1 remote handshake/configuration/play protocol. Do not start a LAN Hub/server thread or Horizon custom registry sync when joining an external server.
- Follow-up checks: If hosting is later planned, split registry sync, chunk/entity tracking, login/respawn, reload and shutdown; compare every packet/registry with Forge 1.20.1 and add resource limits.
- RustChunk implementation mapping: No client mapping; possible future runtime/integrated_server/lan_host_1_20_1.rs isolated from network/remote_session.rs, not created.
- Evidence required before adoption: 1.20.1 server/Forge lifecycle, packet captures, registry/tag fixtures, chunk/entity tests, thread teardown and security review.
- Completion state: COMPLETE — 2982 lines to EOF in 12 contiguous ranges 1–400, 401–800, 801–1000, 1001–1200, 1201–1400, 1401–1600, 1601–2000, 2001–2200, 2201–2400, 2401–2600, 2601–2800 and 2801–2982; SHA and 129988-byte size match; no LAN host operation run.

### 1261. `crates/horizon-client/src/game/launch_api.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6ec77eb621fcef70836333a89d0f2a26b345cfbd`
- Size: 9924 bytes; 271 logical lines
- Coverage: 1–271 (EOF)
- Purpose: Defines the public command-line Launch API, launcher options, JSON capability reply, exit codes and generated Markdown reference.
- Dependencies / cross-references: Game Options parser, session-file/stdin handling, game-dir/home/player-dir paths, quick-play startup, crash-report contract and docs/api/launch.md.
- Implementation / logic: Publishes a versioned allow-list of promised arguments, serializes the supported options and Minecraft/build versions, and standardizes usage/error/panic exit codes.
- Findings: Launcher contract is useful but explicitly tied to Horizon 26.3 and its filesystem layout. Session-file is read once then deleted; credential handling and deletion semantics need a security review before reuse.
- RustChunk relevance: Adaptable launcher-integration reference; not game simulation or Minecraft wire protocol.
- Decision: **adaptable; reference; optional/future**
- 1.20.1 implications: Define a RustChunk-owned, versioned launcher contract; map Android SAF/storage, server selection and authentication securely, without copying Horizon paths or session JSON blindly.
- Follow-up checks: Review parser, symlink/path handling, session-file permissions/deletion, stdin failure, quick-play path validation, exit codes and compatibility negotiation.
- RustChunk implementation mapping: Possible launcher/cli_contract.rs and android/launch_intent.rs; not created.
- Evidence required before adoption: RustChunk CLI/Android launch spec, path traversal tests, credential lifecycle review and launcher integration fixtures.
- Completion state: COMPLETE — 271 logical lines read to EOF; source SHA and 9924-byte size match pinned tree; embedded tests inspected, not run.

### 1262. `crates/horizon-client/src/game/leads.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `26d4ecc1073bba00674e6e981e7dca08d5e887b5`
- Size: 28659 bytes; 680 logical lines
- Coverage: 1–400, 401–680 (contiguous to EOF)
- Purpose: Implements lead attachment, leash knot creation/removal, entity and fence leashing, leash following, drops and leash rendering.
- Dependencies / cross-references: Living entities, leash holder UUID/entity IDs, fence knot entities, player interaction, entity movement/AI, NBT, packets and renderer.
- Implementation / logic: Maintains holder/holder-position state, creates knot entities at fence anchors, updates tether movement and releases/drops leads when the link breaks or holder disappears.
- Findings: Lead and leash-knot behavior exists in 1.20.1; Starling's entity roster, leash data model and newer leash mechanics cannot be copied wholesale.
- RustChunk relevance: Target-era mob interaction and entity-link behavior; adapt data synchronization and rendering.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 lead item use, leash knot, holder UUID/packet state, fence interaction, range/break/drop rules and vanilla mob AI constraints.
- Follow-up checks: Compare player-to-mob, mob-to-mob, fence knot, dimension transfer, unload/reload, death, range break, item drop and server correction.
- RustChunk implementation mapping: Possible game/entities/leash_1_20_1.rs, game/items/lead_1_20_1.rs and network/leash_packets.rs; not created.
- Evidence required before adoption: 1.20.1 Java mappings/source, leash NBT and packet captures, deterministic tether traces and Forge interaction-event tests.
- Completion state: COMPLETE — 680 logical lines read in contiguous ranges 1–400 and 401–680; SHA and 28659-byte tree size match; embedded tests inspected, not run.

### 1263. `crates/horizon-client/src/game/level_particles.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c1a5a97c4171a2156a9da221b5b5d2f3d5f6e784`
- Size: 18814 bytes; 439 logical lines
- Coverage: 1–400, 401–439 (contiguous to EOF)
- Purpose: Translates level events and server/world actions into particle effects, sounds and local visual feedback.
- Dependencies / cross-references: Level event IDs, particle registry/options, block/item/entity state, world RNG, sound engine, local player and packet dispatch.
- Implementation / logic: Maps event payloads and positions to particle batches, including block destruction, potion/splash, smoke, item and entity-related effects.
- Findings: Client feedback is target-relevant, but event IDs, payload encoding, particle names and random ordering must match 1.20.1; later event cases need filtering.
- RustChunk relevance: Adaptable client-side rendering/effect subsystem, driven by authoritative server events.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Implement only 1.20.1 level-event IDs and particle option codecs; preserve deterministic counts/offsets where vanilla specifies them and keep effects cosmetic.
- Follow-up checks: Verify all 1.20.1 event IDs, integer payloads, block-state IDs, sound pairing, RNG sequence, missing particle fallback and malformed packet bounds.
- RustChunk implementation mapping: Possible network/level_events_1_20_1.rs plus render/particles/level_events.rs; not created.
- Evidence required before adoption: 1.20.1 packet captures, particle registry fixtures, seeded effect snapshots and Forge client event tests.
- Completion state: COMPLETE — 439 logical lines read in contiguous ranges 1–400 and 401–439; SHA and 18814-byte tree size match; no particles were run.

### 1264. `crates/horizon-client/src/game/levels.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4bdbdce35c39239d0549dce17382cb2771bcb7d4`
- Size: 30120 bytes; 731 logical lines
- Coverage: 1–400, 401–731 (contiguous to EOF)
- Purpose: Manages multiple loaded levels/dimensions, active and parked worlds, transitions and per-level world state.
- Dependencies / cross-references: Dimension registry, world/chunk storage, level metadata, player transfer, integrated server, world generation and renderer/streaming lifecycle.
- Implementation / logic: Keeps non-active worlds parked, switches the active dimension/world and coordinates the level-owned data needed when changing dimension.
- Findings: Dimension/world separation is foundational, but this is Horizon's integrated multi-world server model; remote clients must not locally create or authoritatively switch server dimensions.
- RustChunk relevance: Essential architectural reference for a complete integrated/singleplayer runtime; remote Aternos sessions consume server-authoritative dimension changes.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Implement 1.20.1 dimension keys, dimension-type effects, world bounds/biomes and respawn/respawn-packet transitions; keep remote session state distinct from integrated worlds.
- Follow-up checks: Check unload/reload, dimension IDs, respawn packets, pending chunk cancellation, entity reset, resource/lighting state and memory release.
- RustChunk implementation mapping: Possible runtime/worlds/level_store_1_20_1.rs and network/dimension_transition.rs; not created.
- Evidence required before adoption: 1.20.1 dimension registry data, respawn/chunk packet traces, world-switch fixtures and lifecycle stress tests.
- Completion state: COMPLETE — 731 logical lines read in contiguous ranges 1–400 and 401–731; SHA and 30120-byte tree size match; no dimension switch was run.

### 1265. `crates/horizon-client/src/game/lightning.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `08c7ebc8f3a7b566300c423e85ed757f9fd59966`
- Size: 29736 bytes; 736 logical lines
- Coverage: 1–400, 401–736 (contiguous to EOF)
- Purpose: Implements lightning strike selection, bolt lifecycle, fire placement, entity damage/transformations and lightning-rod interactions.
- Dependencies / cross-references: Weather/thunder RNG, heightmaps, block tags/states, fire rules, entities/creepers/piglins/skeleton horses, lightning rods, events and particles.
- Implementation / logic: Finds strike positions, advances bolt flashes, applies strike effects to nearby entities, handles channeling and lightning-rod redirection/power events.
- Findings: Lightning and lightning-rod behavior is target-era. Newer entity roster and data/feature gates are version-bound; server owns strike results.
- RustChunk relevance: Adaptable world simulation reference; remote client should render server-sent lightning and events rather than simulate strikes.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 bolt timing, weather RNG, fire placement, rod redirection, channeling trident, creeper charge and skeleton-horse trap semantics.
- Follow-up checks: Compare strike RNG/radius, rod target selection, fire placement/gamerules, entity transforms/damage, bolt flash packet cadence and unload timing.
- RustChunk implementation mapping: Possible game/world/lightning_1_20_1.rs for integrated server and render/lightning_bolt.rs for client visuals; not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, seeded weather fixtures, entity-event packet captures and Forge lightning event tests.
- Completion state: COMPLETE — 736 logical lines read in contiguous ranges 1–400 and 401–736; SHA and 29736-byte tree size match; embedded tests inspected, not run.

### 1266. `crates/horizon-client/src/game/llamas.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `be216e2daec2744d5dbfc196b80d6e43f5fe7dda`
- Size: 6356 bytes; 159 logical lines
- Coverage: 1–159 (EOF)
- Purpose: Implements llama spit projectile behavior, caravan following and llama-specific interaction/state helpers.
- Dependencies / cross-references: Llama species/AI, caravan leader UUID, projectile entity, target selection, movement and entity NBT/metadata.
- Implementation / logic: Maintains caravan relationships and emits/updates llama spit when attack behavior targets a hostile entity.
- Findings: Llama caravan and spit are present in 1.20.1; exact AI timing, target selection and projectile synchronization need target-version comparison.
- RustChunk relevance: Adaptable mob behavior reference, mainly for an integrated server; remote client renders authoritative mobs/projectiles.
- Decision: **adaptable; version-specific**
- 1.20.1 implications: Preserve 1.20.1 caravan limits, lead interaction, spit projectile type/trajectory and server-side AI; omit any later roster assumptions.
- Follow-up checks: Check caravan attach/detach/death/unload, leader replacement, spit speed/gravity/hit effects, target filters and packet ordering.
- RustChunk implementation mapping: Possible game/mobs/llama_1_20_1.rs and network/projectiles/llama_spit.rs; not created.
- Evidence required before adoption: 1.20.1 mob source/mappings, seeded AI tests, projectile packet traces and Forge mob interaction hooks.
- Completion state: COMPLETE — 159 logical lines read to EOF; SHA and 6356-byte tree size match; no mob AI was run.

### 1267. `crates/horizon-client/src/game/locator_bar.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `aa69f7e7d790b020678c1f8400c5009f905e77f9`
- Size: 5035 bytes; 126 logical lines
- Coverage: 1–126 (EOF)
- Purpose: Implements the later locator-bar HUD that shows directional player/entity indicators and related target markers.
- Dependencies / cross-references: New locator-bar game feature, player tracking, HUD layout, entity visibility and modern packet/state sources.
- Implementation / logic: Builds and updates a compact directional marker strip from tracked targets and renders marker state in the HUD.
- Findings: The locator-bar feature is later than Minecraft 1.20.1 and is not part of the baseline client HUD.
- RustChunk relevance: Exclude from 1.20.1 core; optional future feature only if a server/mod explicitly adds it.
- Decision: **unnecessary; optional/future; version-specific**
- 1.20.1 implications: Do not add the locator-bar feature or its protocol assumptions to the 1.20.1 baseline.
- Follow-up checks: Only revisit if a specific Forge mod requires equivalent HUD data; then define a separate extension and permissions model.
- RustChunk implementation mapping: No baseline mapping; possible optional ui/extensions/locator_bar.rs, not created.
- Evidence required before adoption: A concrete 1.20.1 Forge mod requirement and a separately specified extension packet/UI contract.
- Completion state: COMPLETE — 126 logical lines read to EOF; SHA and 5035-byte tree size match; no HUD rendering was run.

### 1268. `crates/horizon-client/src/game/loom.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5d8c7ae771567a8ae2b6bdacb995467e6bf56649`
- Size: 13356 bytes; 385 logical lines
- Coverage: 1–385 (EOF)
- Purpose: Implements loom menu inputs, banner pattern selection, dyeing, result creation and client-side loom screen rendering.
- Dependencies / cross-references: Banner item data, dye items, pattern registry/tags, container menu packets, recipe/result logic, UI widgets and atlas textures.
- Implementation / logic: Validates banner/dye/pattern slots, applies selected patterns to a banner stack and updates the loom menu result/selection.
- Findings: The loom and banner customization exist in 1.20.1, but newer banner-pattern registry entries and component-based item state must be filtered/replaced with legacy BlockEntityTag NBT.
- RustChunk relevance: Adaptable target-era menu and item-data behavior; useful for server-side menu synchronization and client UI.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Use 1.20.1 loom menu slot IDs, banner pattern registry and legacy BlockEntityTag patterns; preserve server-authoritative crafting and Forge menu hooks.
- Follow-up checks: Compare pattern limit, dye/banner validation, selection cycling, result NBT, shift-click, close/reopen and packet/container IDs.
- RustChunk implementation mapping: Possible ui/menus/loom_1_20_1.rs and game/items/banner_patterns_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 LoomMenu Java source, banner NBT fixtures, menu packet captures and Forge container tests.
- Completion state: COMPLETE — 385 logical lines read to EOF; SHA and 13356-byte tree size match; no UI/menu test was run.

### 1269. `crates/horizon-client/src/game/mace.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `46e97e950f40e1af2c500f5e867494d5a4612a14`
- Size: 26195 bytes; 646 logical lines
- Coverage: 1–400, 401–646 (contiguous to EOF)
- Purpose: Implements the mace weapon's smash/fall-distance damage, knockback, wind-burst effects and mace-specific attack behavior.
- Dependencies / cross-references: Mace item, smash damage enchantments, fall distance, entity attack pipeline, wind charge/wind burst, effects and combat events.
- Implementation / logic: Computes smash eligibility and bonus damage from fall distance, applies knockback/launch and triggers mace enchantment effects.
- Findings: Mace and its smash mechanics were added in 1.21, after the 1.20.1 target. The weapon and its enchantments are not baseline content.
- RustChunk relevance: Exclude from 1.20.1 core; optional future mod extension only.
- Decision: **unnecessary; version-specific**
- 1.20.1 implications: Do not add mace item IDs, smash packets, wind-burst or enchantment behavior to the 1.20.1 vanilla baseline.
- Follow-up checks: If a Forge mod supplies a mace, inspect its actual item hooks and custom packets independently; do not port later vanilla assumptions.
- RustChunk implementation mapping: No baseline mapping; optional future game/extensions/mace.rs only if required by a named mod.
- Evidence required before adoption: Specific Forge mod source, registered item/attribute definitions and server-side combat tests.
- Completion state: COMPLETE — 646 logical lines read in contiguous ranges 1–400 and 401–646; SHA and 26195-byte tree size match; no combat test was run.

### 1270. `crates/horizon-client/src/game/machine_entities.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `eaf66d87764429413ae79d8c716f3068ea5febc0`
- Size: 11443 bytes; 292 logical lines
- Coverage: 1–292 (EOF)
- Purpose: Stores custom content-mod machine block-entity state such as Create depots, basins, belts, millstones, presses, mixers, saws, bearings and arms.
- Dependencies / cross-references: Create/Starling kinetic registry, block-entity NBT, item/fluid inventories, belts, contraptions, arms, felling trees and block-entity update packets.
- Implementation / logic: Reads/writes non-vanilla machine tags, reconstructs typed slot/tank layouts and preserves machine contents for saving, nearby updates and rendering.
- Findings: This is content-mod machinery, not vanilla Minecraft. Its NBT schema and Create-specific types cannot be assumed compatible with arbitrary Forge mods.
- RustChunk relevance: Exclude from vanilla baseline; a generic mod block-entity adapter may later support actual Forge machine mods.
- Decision: **unnecessary; optional/future**
- 1.20.1 implications: Do not hard-code Create/Starling machine IDs or NBT into the 1.20.1 core; support modded block entities through Forge-aware registry/data adapters.
- Follow-up checks: Audit spill/save handling, unknown NBT preservation, item drops, chunk save/load and block-entity update packets before any Create compatibility work.
- RustChunk implementation mapping: Possible mod_api/block_entities.rs abstraction only; no Create-specific mapping created.
- Evidence required before adoption: Named target Forge mod/version, its block-entity NBT and packet contract, plus save/load and drop fixtures.
- Completion state: COMPLETE — 292 logical lines read to EOF; SHA and 11443-byte tree size match; embedded tests inspected, not run.

### 1271. `crates/horizon-client/src/game/machines.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `878471ede9add0549e68e73a5ba78921596b350c`
- Size: 69842 bytes; 1841 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1841 (contiguous to EOF)
- Purpose: Implements vanilla container transfer and block behavior: hoppers, dispensers/droppers, container slot rules, item movement, cooldowns and comparator signals; also houses native-mod extensions.
- Dependencies / cross-references: Container/block-entity models, item stack limits, directional slot rules, dispenser behaviors, random ticks, redstone/comparators, item entities, chunk/world mutation and native mod API modules.
- Implementation / logic: Builds container views for single/double chests and special containers, moves items across faces, performs hopper push/pull and dispenser/dropper behavior, and derives comparator fullness.
- Findings: Core hopper/dispenser/dropper/comparator behavior is target-relevant. The crafter submodule and modern item max-stack components are later; native mod hooks are Horizon-specific.
- RustChunk relevance: Essential gameplay subsystem; modularly rewrite against 1.20.1 container semantics and Forge item/block capabilities.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 hopper cooldown/transfer ordering, double chest rules, sided inventories, dispenser behavior registry, dropper and comparator signals; exclude Crafter and Horizon native API.
- Follow-up checks: Compare all face slot rules, blocked chests, hopper timing/cooldowns, item stack caps, special dispenser behaviors, comparator rounding, item entities and Forge IItemHandler integration.
- RustChunk implementation mapping: Possible game/containers/{views,hopper,dispenser,dropper,comparator}_1_20_1.rs plus Forge capability adapter; not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, sided-container fixtures, dispenser behavior traces, redstone tests and Forge capability conformance suite.
- Completion state: COMPLETE — 1841 logical lines read in contiguous ranges 1–400, 401–800, 801–1200, 1201–1600 and 1601–1841; SHA and 69842-byte tree size match; embedded tests inspected, not run.

### 1272. `crates/horizon-client/src/game/machines/crafter.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `20e146575a7b97f8bf2fac9824bbd416226c822b`
- Size: 15545 bytes; 404 logical lines
- Coverage: 1–400, 401–404 (contiguous to EOF)
- Purpose: Implements the redstone-triggered crafter block, crafting animation, result/remainder ejection and crafter menu UI.
- Dependencies / cross-references: Crafter block/entity/menu packets, 3x3 recipe engine, disabled-slot state, hopper/container insertion, advancements and level events.
- Implementation / logic: Finds a matching recipe, starts a six-tick animation, ejects output/remainders, consumes ingredients and manages the toggleable grid slots.
- Findings: Crafter was introduced in 1.21, not 1.20.1. The module is nested under machines.rs but must be excluded from the target baseline.
- RustChunk relevance: Exclude target core; useful only as a later-version reference if a mod adds a crafter-like machine.
- Decision: **unnecessary; version-specific**
- 1.20.1 implications: Do not register crafter block/entity/menu/packet IDs for 1.20.1; a Forge mod's own machine must be implemented through its own schema.
- Follow-up checks: Only revisit for a named mod; compare its recipe, slot-disable and output semantics independently.
- RustChunk implementation mapping: No baseline mapping; possible optional mod_api/crafter_extension.rs if a concrete Forge mod requires it.
- Evidence required before adoption: A specific Forge 1.20.1 mod implementation and its block, menu and packet definitions.
- Completion state: COMPLETE — 404 logical lines read in contiguous ranges 1–400 and 401–404; SHA and 15545-byte tree size match; embedded tests inspected, not run.

### 1273. `crates/horizon-client/src/game/machines/native_energy.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `80b11473904e3b47558fcd46d11d53241cbb58a7`
- Size: 10748 bytes; 275 logical lines
- Coverage: 1–275 (EOF)
- Purpose: Implements a Horizon native-mod energy-buffer API for reading, inserting, extracting and moving energy between mod block entities.
- Dependencies / cross-references: Native mod declarations, per-side insert/extract permissions, block-entity NBT energy field, dimension checks and server-only world simulation.
- Implementation / logic: Simulates and applies bounded transfers, respects capacity and side limits, and moves energy by offering to the target before extracting from the source.
- Findings: This is a proprietary Horizon native-mod API, not Minecraft Forge's standard energy capability; do not confuse it with Forge Energy (IEnergyStorage).
- RustChunk relevance: Exclude from baseline; only the transaction-ordering idea is a generic reference.
- Decision: **unnecessary; optional/future**
- 1.20.1 implications: Implement Forge Energy capability integration only against the actual 1.20.1 Forge API; do not adopt Horizon native buffer NBT or method names.
- Follow-up checks: Review integer overflow, simulation consistency, side permissions, chunk/dimension guards and atomic transfer behavior if an analogous adapter is built.
- RustChunk implementation mapping: Possible forge/capabilities/energy_1_20_1.rs using IEnergyStorage; not created.
- Evidence required before adoption: Forge 47.4.x API signatures, a real mod capability contract and adversarial transfer/overflow tests.
- Completion state: COMPLETE — 275 logical lines read to EOF; SHA and 10748-byte tree size match; embedded tests inspected, not run.

### 1274. `crates/horizon-client/src/game/machines/native_fluids.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `46c4fba0e8bfa4e347e9af97f5411cd380cde7d6`
- Size: 26851 bytes; 673 logical lines
- Coverage: 1–400, 401–673 (contiguous to EOF)
- Purpose: Implements a Horizon native-mod fluid tank API with sided tank access, cauldron conversion, insertion/extraction and block-to-block fluid transfer.
- Dependencies / cross-references: Native mod tank declarations, block-entity NBT, direction faces, water/lava registry, cauldron levels, world mutation and native server simulation.
- Implementation / logic: Reads/writes tank lists, applies side/capacity filters, moves millibuckets, treats cauldrons in bottle thirds and attempts to return any refused fluid.
- Findings: This is not Forge's standard fluid capability; it stores a Horizon-specific NBT shape. Potential concerns include unvalidated tank indices/capacities and negative saved amounts.
- RustChunk relevance: Exclude Horizon API itself; transaction/rollback patterns can inform a Forge FluidHandler adapter after review.
- Decision: **unnecessary; optional/future**
- 1.20.1 implications: Use Forge 1.20.1 IFluidHandler/IFluidTankProperties and FluidStack contracts; do not copy this native NBT schema or cauldron-specific server logic.
- Follow-up checks: Check tank table bounds, capacity invariants, malformed NBT, negative/overflow amounts, face indexes, rollback correctness and fluid registry identity.
- RustChunk implementation mapping: Possible forge/capabilities/fluid_1_20_1.rs backed by IFluidHandler; not created.
- Evidence required before adoption: Forge 47.4.x fluid API, mod tank fixtures, malformed NBT/property tests and simulated-vs-actual transfer traces.
- Completion state: COMPLETE — 673 logical lines read in contiguous ranges 1–400 and 401–673; SHA and 26851-byte tree size match; embedded tests inspected, not run.

### 1275. `crates/horizon-client/src/game/machines/native_items.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `84228d1727093b63a62f5d65a8e483b15b8a483a`
- Size: 10273 bytes; 279 logical lines
- Coverage: 1–279 (EOF)
- Purpose: Implements Horizon native-mod sided container access, item insertion/extraction and loss-avoiding item movement.
- Dependencies / cross-references: Native mod container declarations, vanilla container views, sided slot rules, item stack, composter behavior and server-side world simulation.
- Implementation / logic: Reads slots through a face, extracts permitted items and moves one at a time; if the target refuses an item it restores it to the original source slot.
- Findings: The extension is Horizon-specific and not Forge's IItemHandler capability. Vanilla hopper logic lives in the parent module and is independently relevant.
- RustChunk relevance: Exclude the native API; preserve lossless transfer as a design principle for Forge inventory adapters.
- Decision: **unnecessary; optional/future**
- 1.20.1 implications: Implement Forge 1.20.1 IItemHandler sided insertion/extraction with actual slot validity and simulation semantics; do not adopt Horizon's API/permissions.
- Follow-up checks: Review re-entrant container changes, double-chest identity, composter special cases, item count bounds and restore-on-failure behavior.
- RustChunk implementation mapping: Possible forge/capabilities/item_handler_1_20_1.rs; not created.
- Evidence required before adoption: Forge 47.4.x IItemHandler signatures, mod inventory fixtures, sided slot tests and transfer rollback tests.
- Completion state: COMPLETE — 279 logical lines read to EOF; SHA and 10273-byte tree size match; embedded tests inspected, not run.

### 1276. `crates/horizon-client/src/game/maps.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `39ee40df18751ee751569f799ec3960c05bf80e1`
- Size: 76657 bytes; 1962 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1962 (contiguous to EOF)
- Purpose: Implements map creation, exploration-map resolution, terrain color sampling, saved map data, player/banner/frame decorations, remote map packets and map rendering.
- Dependencies / cross-references: Map item IDs/data, legacy and modern stack metadata, world heightmaps/biomes, banners, item frames, compass/lodestone tracker, map packets, atlas slots and NBT save files.
- Implementation / logic: Creates/scales map centers, incrementally samples terrain, computes color/shading, tracks decorations and holders, serializes map data, applies remote patches and draws map textures/markers.
- Findings: Maps, map packets, banners, item frames and lodestone compasses are 1.20.1 features. This implementation references later map decoration types, item components, trial chambers and Horizon LAN/native callbacks.
- RustChunk relevance: Essential client feature for item rendering and remote map packet handling; world map generation is integrated-server scope.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 map item NBT (map ID/decorations), map saved-data format, packet patch codec, 1.20.1 decoration registry, banner/frame tracking and lodestone tracker; exclude later marker types and LAN APIs.
- Follow-up checks: Compare map scale/center rounding, terrain colors/height/water shading, patch bounds, decoration rotation/order, NBT DataVersion, map ID persistence, frame markers and unloaded chunks.
- RustChunk implementation mapping: Possible game/maps/{data,terrain,decorations,packets,render}_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 Java source/assets/mappings, map NBT fixtures, clientbound map packet captures, seeded color goldens and Forge item tests.
- Completion state: COMPLETE — 1962 logical lines read in contiguous ranges 1–400, 401–800, 801–1200, 1201–1600 and 1601–1962; SHA and 76657-byte tree size match; embedded tests inspected, not run.

### 1277. `crates/horizon-client/src/game/memory.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `303b5e616efa821ef5cd12dd60ef1da7b29a3b67`
- Size: 7363 bytes; 179 logical lines
- Coverage: 1–179 (EOF)
- Purpose: Builds a diagnostic memory census for client/server worlds, shared chunk sections, generation buffers, renderer CPU/GPU allocations and allocator tags.
- Dependencies / cross-references: Chunk census, integrated server, Arc section identity, renderer memory labels, allocator hooks, GPU allocation labels and JSON diagnostics.
- Implementation / logic: Counts unique sections across worlds and returns a structured JSON report of process residency, world columns, renderer and heap categories.
- Findings: Pure diagnostics, not Minecraft behavior. GPU and allocator labels are implementation-specific and the report can expose operational details if exported.
- RustChunk relevance: Exclude from game core; optional developer diagnostics if privacy and performance costs are acceptable.
- Decision: **unnecessary; optional/future**
- 1.20.1 implications: Do not port the Horizon control/report interface; build an opt-in, privacy-safe RustChunk memory diagnostic with Android GPU-driver limitations.
- Follow-up checks: Measure census overhead, Arc identity assumptions, missing allocation categories, GPU API availability and accidental exposure in logs/telemetry.
- RustChunk implementation mapping: Possible diagnostics/memory_census.rs behind a developer flag; not created.
- Evidence required before adoption: Profiling overhead measurements, privacy policy, Android GPU allocator support and bounded diagnostic output tests.
- Completion state: COMPLETE — 179 logical lines read to EOF; SHA and 7363-byte tree size match; no memory profile was run.

### 1278. `crates/horizon-client/src/game/minecarts.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `dd2bfe751f4cd9445ee4720c9bbeeea951f6bb52`
- Size: 122348 bytes; 2895 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–2895 (contiguous to EOF)
- Purpose: Implements minecart entities, rail shapes and movement, powered/activator rails, collisions, riders, variants, containers, TNT explosions, persistence and rendering.
- Dependencies / cross-references: Rail/block states, collision engine, entities/mobs, player input, vehicle packets, item/container models, TNT/explosion rules, NBT and minecart renderer.
- Implementation / logic: Ticks carts along rail segments, applies slope/boost/slowdown, collision and pickup/push, manages chest/hopper/furnace/TNT/rideable/command/spawner variants and serializes/render them.
- Findings: Core minecart, rail, chest/hopper/furnace/TNT behavior is 1.20.1-relevant. Experimental minecart_improvements, step interpolation packets and Horizon LAN/native callbacks are not safe baseline assumptions.
- RustChunk relevance: Essential vehicle simulation and rendering domain; remote client must follow server entity movement and not authoritatively simulate rails.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild classic 1.20.1 OldMinecartBehavior and vehicle packet handling, preserve variant NBT and rider/collision semantics; exclude experimental feature-gated physics unless negotiated by target server.
- Follow-up checks: Compare all rail shapes/slopes, powered/activator rail, max speed, water/gravity, collisions, push/pickup, passenger offsets, dismount, TNT thresholds, hopper suction and NBT round-trip.
- RustChunk implementation mapping: Possible game/vehicles/minecart_{physics,variants,containers,render}_1_20_1.rs and network/vehicles.rs; not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, rail fixture world, entity/vehicle packet captures, seeded TNT traces, NBT fixtures and Forge vehicle event tests.
- Completion state: COMPLETE — 2895 logical lines read in contiguous ranges 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800 and 2801–2895; SHA and 122348-byte tree size match; embedded tests inspected, not run.

### 1279. `crates/horizon-client/src/game/mob_blocks.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `6c4106c832d80207afccb66e2f6df5c78b6e78fb`
- Size: 33450 bytes; 782 logical lines
- Coverage: 1–400, 401–782 (contiguous to EOF)
- Purpose: Renders items and blocks held/worn by mobs, including enderman carried blocks, snow-golem pumpkin, mooshroom mushrooms, iron-golem poppy and raid-captain banners.
- Dependencies / cross-references: Mob species/equipment, item-model resolver, block baked models, entity pose transforms, banner textures, lighting/overlay and GPU skinning.
- Implementation / logic: Builds per-mob transforms for hand/head/carried/worn models, submits item/block geometry and culls back-facing quads for cutout cross models.
- Findings: Several layers are target-era (enderman, snow golem, mooshroom, iron golem, raid captain). Copper-golem antenna rendering and newer species/item components are later-version additions.
- RustChunk relevance: Adaptable rendering subsystem reference; separate vanilla 1.20.1 layers from later entities and item-component data.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 held-item contexts, legacy equipment metadata, enderman block state, snow-golem pumpkin, mooshroom mushrooms and captain banner; exclude copper-golem and modern component paths.
- Follow-up checks: Compare model transforms, left/right hand, spyglass use, item contexts/seeds, hurt overlays, invisibility, block culling, banner patterns and Forge-added mob equipment.
- RustChunk implementation mapping: Possible render/mobs/{held_items,carried_blocks,worn_blocks,head_items}_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 renderer source/assets, mob equipment packet fixtures, render goldens and Forge entity-layer registration tests.
- Completion state: COMPLETE — 782 logical lines read in contiguous ranges 1–400 and 401–782; SHA and 33450-byte tree size match; embedded tests inspected, not run.

### 1280. `crates/horizon-client/src/game/mob_gear.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4e7a2a0fd1f5c97c591034fd0d02fcb21270625a`
- Size: 8737 bytes; 227 logical lines
- Coverage: 1–227 (EOF)
- Purpose: Handles mob spawn equipment enchantments, equipment-table loot rolls, Halloween heads and raid-specific pillager/vindicator buffs.
- Dependencies / cross-references: Mob species/finalization, equipment slots/drop chances, loot tables and random sequences, local difficulty, enchantment registry and raid wave rules.
- Implementation / logic: Rolls equipment tables into mob slots, applies difficulty-scaled enchantment chances/costs, handles the rare pillager piercing crossbow and raid wave buffs.
- Findings: Mob spawn gear and raid buffs are target-era, but this implementation uses modern mob_spawn_equipment providers and newer data-driven enchantment selection; target 1.20.1 source/data must replace them.
- RustChunk relevance: Adaptable integrated-server mob-spawn behavior reference; remote clients only render synchronized equipment.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Use 1.20.1 mob equipment finalization, local-difficulty enchantment rules, raid wave thresholds and legacy enchantment NBT; remove modern provider assumptions.
- Follow-up checks: Compare mob eligibility, RNG draw order, local difficulty, chance/cost bounds, slot drop chances, Halloween conditions and raid wave enchantment timing.
- RustChunk implementation mapping: Possible game/mobs/spawn_equipment_1_20_1.rs and game/raids/equipment_buffs.rs; not created.
- Evidence required before adoption: 1.20.1 Java source/data, seeded mob-spawn fixtures, loot/enchantment NBT and Forge finalize/raid event traces.
- Completion state: COMPLETE — 227 logical lines read to EOF; SHA and 8737-byte tree size match; embedded tests inspected, not run.

### 1281. `crates/horizon-client/src/game/mob_index.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `052a292348f361155e100078d07b0bfcabef2a12`
- Size: 3216 bytes; 90 logical lines
- Coverage: 1–90 (EOF)
- Purpose: Provides a cached network-entity-ID to mob-vector-index lookup with validation and rebuild-on-miss.
- Dependencies / cross-references: Mob vector, spawn/remove/dimension-swap paths and every packet handler that resolves a mob ID.
- Implementation / logic: Checks a cached index against the current slice; if stale or missing, falls back to linear search and rebuilds the map.
- Findings: The cache tolerates unannounced vector mutations because every hit is validated. It assumes entity IDs are unique; rebuilding the whole map on every miss may be costly under churn.
- RustChunk relevance: Highly adaptable optimization for packet-heavy multiplayer entity updates and large farms.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Port only after profiling; retain validation/fallback semantics and confirm all target packet entity IDs are unique within the active level.
- Follow-up checks: Benchmark 3,000+ entities, mass spawn/removal, ID reuse, dimension swap, duplicate-ID defense and stale-index invalidation.
- RustChunk implementation mapping: Possible game/entities/mob_index.rs wrapping Vec<Mob> lookup; not created.
- Evidence required before adoption: Correctness property tests against linear search, packet-load benchmarks and entity-ID lifecycle traces.
- Completion state: COMPLETE — 90 logical lines read to EOF; SHA and 3216-byte size match pinned tree; embedded tests inspected, not run.

### 1282. `crates/horizon-client/src/game/mob_looting.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `11866dae86d99a8b17d409c5ae56425d5edf9d85`
- Size: 8461 bytes; 201 logical lines
- Coverage: 1–201 (EOF)
- Purpose: Implements mob item pickup and equipment replacement: armor/weapons, drop chance for replaced gear and pickup advancements.
- Dependencies / cross-references: Dropped item entities, mob species/AI, equipment slots, item attributes/enchantments, item tags, local RNG and mob-griefing gamerule.
- Implementation / logic: Scans eligible nearby item entities, checks species pickup preferences, compares candidate gear facts with worn gear, equips a stack and may drop the replaced stack.
- Findings: Mob gear pickup exists in 1.20.1. The code references newer spear and disliked-weapon tags and modern item/equipment data; target-era item tags and attribute/NBT representation differ.
- RustChunk relevance: Adaptable integrated-server behavior; a remote client should display authoritative equipment and pickup events, not simulate them.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 pickup reach, mob-griefing gate, equipment preference, armor/weapon comparison, Binding Curse and drop chance using legacy item NBT and target tags.
- Follow-up checks: Compare mob species eligibility, item tag availability, RNG draw order, armor fallback to main hand, drop chance, persistence and pickup advancement attribution.
- RustChunk implementation mapping: Possible game/mobs/looting_1_20_1.rs and game/items/equipment_compare.rs; not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, target item tags, seeded gear fixtures and Forge mob-pickup event tests.
- Completion state: COMPLETE — 201 logical lines read to EOF; SHA and 8461-byte size match; embedded tests inspected, not run.

### 1283. `crates/horizon-client/src/game/mob_sounds.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e74ee36e98b6be895c2d10626b363728c846498e`
- Size: 5601 bytes; 136 logical lines
- Coverage: 1–136 (EOF)
- Purpose: Starts and maintains looping bee, guardian and sniffer sounds, plus later Happy Ghast/Nautilus riding loops.
- Dependencies / cross-references: Mob species/anger/attack state, entity event 21, sound engine instances, rider state, underwater and velocity metadata.
- Implementation / logic: Tracks per-entity sound loops, restarts bee loops when anger changes, starts guardian attack hum on event 21 and follows riding entity motion.
- Findings: Bee buzz and guardian attack hum are target-era; Sniffer is 1.20.1. Happy Ghast and Nautilus riding loops are later additions and must be excluded.
- RustChunk relevance: Adaptable client audio subsystem driven by server entity events and synchronized state.
- Decision: **adaptable; version-specific**
- 1.20.1 implications: Keep 1.20.1 bee/guardian/sniffer loops and event mapping; remove Happy Ghast/Nautilus species and ensure pause, unload, death and silent flags stop instances.
- Follow-up checks: Verify entity event 21, bee anger transitions, sniffer digging condition, silent/unload/death cleanup, audio pause and repeated-start suppression.
- RustChunk implementation mapping: Possible audio/mob_loops_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 sound event registry, entity-event packet traces and deterministic sound-instance lifecycle tests.
- Completion state: COMPLETE — 136 logical lines read to EOF; SHA and 5601-byte size match; no audio playback was run.

### 1284. `crates/horizon-client/src/game/mobile_controls.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3dbb3f39d4a598818876fe0138e97229c3fcec60`
- Size: 82783 bytes; 2035 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2035 (contiguous to EOF)
- Purpose: Implements Android touch controls, virtual mouse, custom bindings, keyboard/preset input, haptics, gestures, inventory/menu touch and contextual interaction.
- Dependencies / cross-references: Android IME/platform bridge, touch-control settings/editor, winit events, gamepad, player/item use, menus, inventory, renderer and custom mod input.
- Implementation / logic: Maintains touch ownership and control modes, translates gestures and virtual pointer events into keyboard/mouse/gamepad actions, routes touch UI commands and releases held keys on lifecycle/menu changes.
- Findings: Directly relevant to Android-first UX, but tightly coupled to Horizon UI/JNI and current control library. `native_key_event(key, down, false)` is called twice consecutively in one custom-token path; verify whether this duplicates an event before porting. `ForceKill` exits the process immediately.
- RustChunk relevance: Essential mobile-input domain; rewrite/adapt into RustChunk-owned Android input state and lifecycle boundaries.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Build a minimal 1.20.1 touch layer first: movement/look, attack/use, jump/sneak, hotbar, inventory, virtual pointer and Android keyboard; remove Horizon-specific GoyControl, preset catalog and mod commands.
- Follow-up checks: Audit duplicate native key dispatch, touch cancel/focus loss, multi-touch ownership, stuck-key prevention, inventory drag, virtual mouse release, keyboard composition, process exit and Android pause/resume.
- RustChunk implementation mapping: Possible platform/android/{touch_controls,virtual_pointer,ime}.rs plus game/input/mobile_router.rs; not created.
- Evidence required before adoption: Android NDK event fixtures, Galaxy A22 touch latency, cancellation/lifecycle tests, keyboard Unicode cases and no-duplicate key-event assertions.
- Completion state: COMPLETE — 2035 logical lines read in six contiguous ranges through EOF; SHA and 82783-byte size match; embedded tests inspected, not run.

### 1285. `crates/horizon-client/src/game/mobile_controls/mobile_platform.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `df2067cbf3bdd3d4a757a19567da95760b8b460f`
- Size: 2696 bytes; 86 logical lines
- Coverage: 1–86 (EOF)
- Purpose: Bridges the mobile control layer to Android text mirror, keyboard events, vibration and opening the Reports folder.
- Dependencies / cross-references: Android JNI text mirror, generation IDs, keyboard event queue, platform folder opener, widgets and gamepad haptics.
- Implementation / logic: Shows/hides an Android text mirror, rejects stale-generation edits, diffs text into backspaces plus insertion, maps action/key events and forwards vibration/document requests.
- Findings: Useful platform seam, but uses Horizon's Android bridge and a global mutex. Text diff is character-based; cursor/selection semantics are not represented here.
- RustChunk relevance: Adaptable platform boundary; keep OS/JNI details out of common game logic.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Implement a RustChunk Android input bridge with generation-safe IME updates, SAF folder opening and lifecycle-aware vibration; do not copy Horizon package paths.
- Follow-up checks: Test stale IME generations, Unicode combining sequences, selection/cursor behavior, concurrent show/hide, mutex poisoning and vibration cancellation on pause.
- RustChunk implementation mapping: Possible platform/android/text_mirror.rs and platform/android/document_picker.rs; not created.
- Evidence required before adoption: Android JNI/NDK contract, Unicode IME tests, lifecycle traces and scoped-storage permission tests.
- Completion state: COMPLETE — 86 logical lines read to EOF; SHA and 2696-byte size match; no Android bridge was run.

### 1286. `crates/horizon-client/src/game/mod_api_host.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `39830ea73f84a0bb43cf2aba041ac2e228294af4`
- Size: 9587 bytes; 271 logical lines
- Coverage: 1–271 (EOF)
- Purpose: Hosts the Horizon native-mod API for level reads/writes, attachments, random ticks, scheduled block/fluid ticks and authoritative entity access.
- Dependencies / cross-references: Native mod API traits, block/chunk/block-entity NBT, player/guest records, entity UUID/index, world ticks, LAN state and generated Host methods.
- Implementation / logic: Resolves Level/Chunk/BlockEntity/Entity/Player attachment holders, records sync targets, prunes removed entity attachments and implements LevelHost/LevelRandomHost/LevelTicksHost/LevelWriteHost.
- Findings: This is a broad proprietary mod host with server-authoritative world mutation and LAN guest state, not Forge's extension API. Attachment data persistence and sync are tightly coupled to Horizon NBT/world internals.
- RustChunk relevance: Exclude as an implementation; use only as a reference when defining a deliberately smaller Forge-facing API.
- Decision: **unnecessary; optional/future**
- 1.20.1 implications: Do not expose arbitrary world writes, teleport/kill or command execution through a client-only mod interface. Map only actual Forge 1.20.1 events/capabilities and explicit server-side permissions.
- Follow-up checks: Threat-model attachment holder spoofing, chunk/block-entity save ordering, guest UUID cleanup, sync floods, stale entity IDs and world mutation authority.
- RustChunk implementation mapping: No baseline mapping; possible narrowly scoped forge/mod_bridge.rs after Forge API design.
- Evidence required before adoption: Forge 47.4.x API contract, mod permission model, server-authority boundary and attachment persistence/security tests.
- Completion state: COMPLETE — 271 logical lines read to EOF; SHA and 9587-byte size match; no mod API was executed.

### 1287. `crates/horizon-client/src/game/mod_api_host_generated.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4f2278e112fe230caea776f68420805d569045a0`
- Size: 17834 bytes; 543 logical lines
- Coverage: 1–400, 401–543 (contiguous to EOF)
- Purpose: Generated implementation of the large Horizon native-mod Host trait, forwarding mod calls to Game's native_* methods.
- Dependencies / cross-references: mod_api_host.rs, mod API trait schema/generator, entity/world/item/container/recipe/GUI/render/network helpers and xtask mod-api generation.
- Implementation / logic: Implements dozens of methods for drawing, world mutation, player inventory, commands, mod settings, recipes, fluid/item transfer, entity access and custom networking.
- Findings: Generated glue is not a standalone subsystem and must not be edited or copied independently of its generator/schema. Its breadth grants mods extensive game and server powers.
- RustChunk relevance: Exclude from the 1.20.1 baseline; a future mod API should generate its bindings from a RustChunk-owned minimal schema.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not port Horizon's Host surface or native_* names; if a mod bridge is needed, generate a constrained API with explicit capability checks and target-version types.
- Follow-up checks: Inspect generator/schema before any adoption; inventory every privileged method, thread/side restriction, network payload limit and mod identity check.
- RustChunk implementation mapping: No direct mapping; possible future mod_api/generated_host.rs generated from RustChunk's own schema.
- Evidence required before adoption: Reviewed API schema/generator, security model, permission tests, bounded payload tests and Forge 1.20.1 compatibility fixtures.
- Completion state: COMPLETE — 543 logical lines read in contiguous ranges 1–400 and 401–543; SHA and 17834-byte size match; generated tests were not run.

### 1288. `crates/horizon-client/src/game/mod_assets.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2bdc37dea236ba216723c2a73e5b0c2d062dfc8a`
- Size: 7364 bytes; 203 logical lines
- Coverage: 1–203 (EOF)
- Purpose: Downloads a content mod's asset JAR from Modrinth, resumes partial transfers, checks SHA-512 and unpacks the manifest-selected assets.
- Dependencies / cross-references: horizon_mods manifest/fetch helpers, Modrinth version/file API, ureq, filesystem cache, progress callback and Mods page failure status.
- Implementation / logic: Requests a version URL, downloads in byte ranges, retries quiet failures, verifies exact size and SHA-512, renames the verified JAR and delegates unpacking.
- Findings: Asset-only retrieval is distinct from executing mod code; integrity is checked. Archive extraction safety is delegated to horizon_mods::fetch::unpack and was not audited in this file.
- RustChunk relevance: Optional launcher/content-mod service, not needed for vanilla multiplayer or protocol compatibility.
- Decision: **adaptable; optional/future**
- 1.20.1 implications: If RustChunk supports content packs, use explicit consent, pinned HTTPS sources, verified hashes, strict archive path/size limits and atomic installation; do not require Modrinth.
- Follow-up checks: Audit unpack() for zip-slip, symlinks, duplicate paths, decompression bombs, manifest allow-list, partial rollback, redirects and cache poisoning.
- RustChunk implementation mapping: Possible launcher/content_assets.rs isolated from the game runtime; not created.
- Evidence required before adoption: Archive extraction security review, Modrinth/other provider terms, hash fixtures, interrupted download tests and atomic install/rollback tests.
- Completion state: COMPLETE — 203 logical lines read to EOF; SHA and 7364-byte size match; no download/unpack was run.

### 1289. `crates/horizon-client/src/game/mod_catalog.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `856be7f82e3e0fd5247e2f63b10fecb4b0f0178f`
- Size: 34967 bytes; 1010 logical lines
- Coverage: 1–400, 401–800, 801–1010 (contiguous to EOF)
- Purpose: Implements the Android in-game catalog screen for mods, resource packs and shader packs: search, paging, install progress and confirmed removal.
- Dependencies / cross-references: starling_catalog search/install/remove, native_builds UI helpers, Android touch/IME, game directories, installed-file scans, worker threads and Mods/pack/shader state.
- Implementation / logic: Draws shelves and rows, debounces search, pages a shared feed, runs installs on a worker thread, reports progress/results and confirms destructive removal before deleting files.
- Findings: This is launcher/catalog UX, not Minecraft 1.20.1 gameplay. It trusts starling_catalog for download/install details; thread completion and folder refresh behavior need the provider's own audit.
- RustChunk relevance: Optional launcher feature if an in-app catalog is desired; not necessary for connecting to Forge servers.
- Decision: **unnecessary; optional/future**
- 1.20.1 implications: Keep mod/pack management outside the game core; if built, use SAF/scoped storage, explicit confirmation, signed/hash-verified sources and transactional installs.
- Follow-up checks: Audit starling_catalog install/remove path validation, archive extraction, duplicate installs, concurrent search/install, worker shutdown, disk-full and deletion rollback.
- RustChunk implementation mapping: Possible launcher/catalog_screen.rs with provider abstraction; not created.
- Evidence required before adoption: Catalog provider security audit, Android storage tests, concurrent task tests, install rollback and destructive-action UX review.
- Completion state: COMPLETE — 1010 logical lines read in contiguous ranges 1–400, 401–800 and 801–1010; SHA and 34967-byte size match; no catalog operation was run.

### 1290. `crates/horizon-client/src/game/mods.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `655897741a6273125f22b91515b56009af21241a`
- Size: 29203 bytes; 700 logical lines
- Coverage: 1–400, 401–700 (contiguous to EOF)
- Purpose: Loads world-tied content mods and their assets, registers mod blocks/items/effects/entities/menus and lists content/native/WebAssembly mod status.
- Dependencies / cross-references: horizon_mods packages/manifests, resource-root builder, registries, mod-list state, native hooks/builds, item components, world_content and protocol mod list.
- Implementation / logic: Resolves enabled mods against world registry set, fetches assets, validates and merges content, registers mod effects/entity/menu types and builds a resource overlay.
- Findings: Content mods and world-specific registry compatibility are useful concepts, but Horizon's .hmod/.hnmod formats, registry sync and native-code model are proprietary. Native mods run with full game access; WASM path is explicitly retired.
- RustChunk relevance: Adaptable high-level content-mod lifecycle, not a direct Forge loader substitute.
- Decision: **adaptable; rewrite required; optional/future**
- 1.20.1 implications: For Forge 1.20.1, rely on the actual Forge mod loader, mod handshake and registry synchronization. Do not invent a parallel .hmod format or run untrusted native libraries.
- Follow-up checks: Audit package validation, registry mismatch handling, world-tied mod sets, asset cache integrity, duplicate IDs, native hook trust and server/client side enforcement.
- RustChunk implementation mapping: Possible launcher/mod_inventory.rs only; game-side integration must use Forge 47.4.x semantics; not created.
- Evidence required before adoption: Forge 1.20.1 mod-list/handshake fixtures, registry sync tests, mod trust model and content asset security review.
- Completion state: COMPLETE — 700 logical lines read in contiguous ranges 1–400 and 401–700; SHA and 29203-byte size match; embedded tests inspected, not run.

### 1291. `crates/horizon-client/src/game/mount_menu.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1b967d7db3f8222dd4b6d87bfc627d744e8f1fa0`
- Size: 23717 bytes; 634 logical lines
- Coverage: 1–400, 401–634 (contiguous to EOF)
- Purpose: Implements horse-family mount inventory UI, saddle/armor/chest slots, slot rules, shift-click, remote menu opening and mount preview rendering.
- Dependencies / cross-references: Horse/donkey/mule/llama species, item stacks, container slot IDs, mount entity metadata, menu packets, inventory clicks and UI atlas.
- Implementation / logic: Builds a mount-specific inventory view, maps saddle/body/chest slots, validates items, synchronizes local mount state and accepts server-owned remote menu contents.
- Findings: Horse, donkey, mule and llama inventories are target-era; Nautilus inventory/armor is later and must be removed. Internal slot constants 700/701/710 are local indexes, not vanilla protocol IDs.
- RustChunk relevance: Essential container UI behavior for mount interaction; server remains authoritative for remote menu content/click results.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Implement 1.20.1 horse/donkey/mule/llama menu layouts and packets, legacy horse armor/carpet NBT and target slot numbering; exclude Nautilus.
- Follow-up checks: Check llama strength columns, chested donkey/mule rows, saddle/armor acceptance, shift-click ordering, distance/death close, remote slot sync and malformed menu contents.
- RustChunk implementation mapping: Possible ui/menus/mount_inventory_1_20_1.rs and network/container_menu.rs; not created.
- Evidence required before adoption: 1.20.1 AbstractHorse/horse menu source, container packet captures, slot fixtures and Forge inventory click tests.
- Completion state: COMPLETE — 634 logical lines read in contiguous ranges 1–400 and 401–634; SHA and 23717-byte size match; no menu was run.

### 1292. `crates/horizon-client/src/game/mounts.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a4dc01b1caa464b592d6ac179ae1997e8952568a`
- Size: 43042 bytes; 1086 logical lines
- Coverage: 1–400, 401–800, 801–1086 (contiguous to EOF)
- Purpose: Implements mount interaction, feeding, saddling, horse jump charging, rider input/position, dismount collision search, mount fall damage and HUD bars.
- Dependencies / cross-references: Horse/donkey/mule/camel/pig/strider/llama, later Happy Ghast/Nautilus/CamelHusk, player motion/pose, item durability, world collision/fluid, events and LAN guest state.
- Implementation / logic: Routes species-specific interaction, applies food/taming/equipment, calculates jump strength, sends rider controls, positions the player and searches safe dismount locations including lava-aware Strider rules.
- Findings: Horse, donkey, mule, camel, pig, strider and llama behaviors are relevant to 1.20.1. Happy Ghast, Nautilus, Zombie Nautilus and CamelHusk are later additions; LAN guest handling is Horizon-specific.
- RustChunk relevance: Essential mount gameplay reference; rewrite target species and separate remote client prediction from server authority.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 horse-family feeding/taming/saddle/chest, pig/strider steering, camel dash/sit, jump charge, passenger offsets and safe dismount; exclude later species.
- Follow-up checks: Compare feeding table, tame/buck events, jump thresholds, rider input, pose/collision dismount candidates, water/lava cases, fall propagation and item durability.
- RustChunk implementation mapping: Possible game/mounts/{interaction,riding,dismount,inventory}_1_20_1.rs; not created.
- Evidence required before adoption: 1.20.1 Java source/mappings, seeded interaction fixtures, vehicle packet traces, collision scenes and Forge mount event tests.
- Completion state: COMPLETE — 1086 logical lines read in contiguous ranges 1–400, 401–800 and 801–1086; SHA and 43042-byte size match; embedded tests inspected, not run.

### 1293. `crates/horizon-client/src/game/moving_pistons.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `654ade5381ccd5cfd4fa585741fd2d43740c38bf`
- Size: 6974 bytes; 167 logical lines
- Coverage: 1–167 (EOF)
- Purpose: Renders moving piston block entities with interpolated block geometry and a temporary fallback while the destination chunk mesh is pending.
- Dependencies / cross-references: MovingPistonState, piston block states, chunk meshing/dirty sections, pending GPU uploads, camera, block tint/light and renderer vertex buffers.
- Implementation / logic: Combines live and just-finished piston states, calculates interpolated offsets, builds piston head/base geometry and retains a finished visual until the chunk mesh upload is ready.
- Findings: Pistons are target-era. The fallback specifically prevents a visual gap/z-fighting while a chunk mesh catches up; this is renderer-specific and must not change server block state.
- RustChunk relevance: Highly relevant rendering reference for smooth block transitions and avoiding visual holes during asynchronous chunk rebuilds.
- Decision: **adaptable; version-specific**
- 1.20.1 implications: Port target piston interpolation and a bounded visual fallback keyed to chunk section mesh/upload state; keep authoritative world/chunk data separate from transient render state.
- Follow-up checks: Test extend/retract, sticky pull, head shortness threshold, chunk unload, mesh dirty/upload races, light/tint, camera movement and fallback expiry.
- RustChunk implementation mapping: Possible render/blocks/moving_piston.rs and streaming/section_upload_state.rs; not created.
- Evidence required before adoption: 1.20.1 piston state fixtures, frame-by-frame render goldens and asynchronous chunk mesh/upload race tests.
- Completion state: COMPLETE — 167 logical lines read to EOF; SHA and 6974-byte size match; no renderer test was run.

### 1294. `crates/horizon-client/src/game/music.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4b0a0a064eeca8aab274c8ab64aab5350298631b`
- Size: 13096 bytes; 328 logical lines
- Coverage: 1–328 (EOF)
- Purpose: Selects situational/background music, manages music frequency and pause behavior, ambient player sounds, sound engine ticking and now-playing toast.
- Dependencies / cross-references: Dimension/biome music tables, music options, ambient sound registry, player fluid/light state, sound engine, mob/vehicle loop contexts and UI toast state.
- Implementation / logic: Chooses music from screen/dimension/biome/boss/underwater situation, applies volume/frequency/off-track filters, pauses/resumes audio and ticks ambient sound handlers.
- Findings: Music and ambient sound selection are target-era, but native mod music layers and Horizon extras are proprietary extensions; sound registries and option keys must match 1.20.1.
- RustChunk relevance: Adaptable client audio subsystem; useful after core rendering/input/network stability.
- Decision: **adaptable; version-specific**
- 1.20.1 implications: Use 1.20.1 music definitions, biome/dimension tags, music delay/frequency and ambient event data; exclude native mod layers and optional Music page.
- Follow-up checks: Compare music situation priority, biome lookup, music delay/RNG, pause/resume, underwater transitions, sound listener and toast settings.
- RustChunk implementation mapping: Possible audio/music_manager_1_20_1.rs and audio/ambient_handlers.rs; not created.
- Evidence required before adoption: 1.20.1 sounds.json/music data, seeded music-selection traces and audio lifecycle tests on Android.
- Completion state: COMPLETE — 328 logical lines read to EOF; SHA and 13096-byte size match; no audio playback was run.

### 1295. `crates/horizon-client/src/game/names.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `048b855ce2bef59ea8b861c8b954549eded52c56`
- Size: 30307 bytes; 757 logical lines
- Coverage: 1–400, 401–757 (contiguous to EOF)
- Purpose: Builds and renders mob/player/armor-stand/item-frame name tags, scoreboard team visibility/colors, below-name scores and remote name metadata.
- Dependencies / cross-references: Mob/player UUID/profile names, scoreboard objectives/teams, item frames, armor stands, entity visibility, camera, font atlas, text layers and mod name hooks.
- Implementation / logic: Collects eligible names by distance/target/team rules, composes styled text and scores, computes pose-relative positions and submits depth-tested/faded/see-through text geometry.
- Findings: Name tags, scoreboard visibility and below-name scores are target-era. Mannequin/mod API hooks and custom own-name display are extensions; later text/item metadata must be translated to 1.20.1 components/legacy formatting.
- RustChunk relevance: Essential rendering and scoreboard presentation for multiplayer clients.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild 1.20.1 team visibility rules, custom-name metadata, player profile names, armor-stand/frame name display, below-name objective and camera-facing text geometry.
- Follow-up checks: Compare 64/10-block thresholds, team visibility modes, invisible entities, discrete crouch tags, pose height, scoreboard formats, frame target rules and remote player lifecycle.
- RustChunk implementation mapping: Possible render/name_tags_1_20_1.rs with scoreboard/name resolver and text geometry; not created.
- Evidence required before adoption: 1.20.1 scoreboard/team packets, entity metadata, player-info fixtures and render goldens.
- Completion state: COMPLETE — 757 logical lines read in contiguous ranges 1–400 and 401–757; SHA and 30307-byte tree size match; embedded tests inspected, not run.

### 1296. `crates/horizon-client/src/game/narrator.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `38d2a892998c292f4421e56a2c7c9a14a387a9dc`
- Size: 13726 bytes; 340 logical lines
- Coverage: 1–340 (EOF)
- Purpose: Integrates text-to-speech narrator status, keyboard hotkey, screen narration, input narration and narrator toggle toast.
- Dependencies / cross-references: horizon-narrator library, settings/voice volume, clipboard shortcut detection, screen widget narration, keyboard/mouse events and toast manager.
- Implementation / logic: Initializes narrator, syncs status/volume, cycles Off/All/System, narrates screen and input actions, queues chat/system speech and draws status toast.
- Findings: Accessibility narration is useful, but the TTS backend and exact screen narration API are platform-specific. This module gracefully continues when TTS fails.
- RustChunk relevance: Optional accessibility feature; do not block the game from launching if Android TTS is unavailable.
- Decision: **adaptable; optional/future**
- 1.20.1 implications: Integrate Android TextToSpeech/accessibility APIs separately from game protocol; preserve keyboard navigation narration and fallback when service is missing.
- Follow-up checks: Test TTS initialization failure, voice volume, status cycle, focus/edit fields, delayed narration, screen changes, Android language support and lifecycle shutdown.
- RustChunk implementation mapping: Possible accessibility/narrator.rs plus platform/android/tts.rs; not created.
- Evidence required before adoption: Android TTS API behavior, TalkBack interaction tests, localization fixtures and no-service fallback tests.
- Completion state: COMPLETE — 340 logical lines read to EOF; SHA and 13726-byte size match; embedded tests inspected, not run.

### 1297. `crates/horizon-client/src/game/native_biome_colors.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `466fc4be3daab5a1cd11c1ca359a92dc8b9bea20`
- Size: 1930 bytes; 42 logical lines
- Coverage: 1–42 (EOF)
- Purpose: Allows Horizon native mods to recolor biome grass/foliage/water and triggers chunk/LOD resource rebuild when colors change.
- Dependencies / cross-references: Native mod callback, biome base/current color tables, renderer worker context, LOD cache and section streaming.
- Implementation / logic: Recomputes biome colors from the base table, compares against current colors, clears LOD and reloads renderer resources when changed.
- Findings: This is a proprietary mod callback, not vanilla behavior. The invalidation pattern is reusable for dynamic biome tint but rebuilding every loaded section can be expensive.
- RustChunk relevance: Exclude native API; retain only the renderer invalidation concept if dynamic seasons/colors are implemented.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not expose a Horizon native callback; implement resource-pack/Forge biome color hooks and schedule bounded chunk remeshing on color changes.
- Follow-up checks: Measure remesh cost, prevent per-frame invalidation, verify world-specific biome tables and synchronize tint updates with far-terrain cache.
- RustChunk implementation mapping: Possible render/biome_color_invalidation.rs only if required; not created.
- Evidence required before adoption: 1.20.1 Forge biome color event/API, chunk remesh benchmark and LOD cache consistency tests.
- Completion state: COMPLETE — 42 logical lines read to EOF; SHA and 1930-byte size match; no renderer rebuild was run.

### 1298. `crates/horizon-client/src/game/native_block_entity_draw.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `582b140b97ddc56ac88156011690e23e7e6db3f9`
- Size: 28650 bytes; 766 logical lines
- Coverage: 1–400, 401–766 (contiguous to EOF)
- Purpose: Provides a native-mod block-entity drawing API with pose stack transforms, custom quads/cubes/blocks/items, culling bounds and cached kept geometry.
- Dependencies / cross-references: Mod block entities/revisions, render frustum, block/item models, sprite atlas, vertex lighting/diffuse, section render pass and resource reload.
- Implementation / logic: Builds geometry relative to block origin, culls by mod-provided bounds, reuses cached geometry keyed by dimension/state/light/entity revision and expires unseen entries after 100 ticks.
- Findings: Useful architecture for modded block-entity rendering, but the API is proprietary. Cache correctness depends on revision/state/light/dimension invalidation and explicit clearing on resource reload; non-finite bounds intentionally bypass frustum culling.
- RustChunk relevance: Optional future mod rendering extension, not needed for vanilla 1.20.1 client baseline.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not port Horizon's native draw API. If Forge block entities are supported, render through target BER registration and keep a strict cache invalidation key.
- Follow-up checks: Verify revision changes on NBT updates, resource/atlas reload clearing, chunk unload, bounds changes, kept geometry expiration and malicious huge geometry limits.
- RustChunk implementation mapping: Possible render/mod_block_entities.rs behind a Forge BER adapter; not created.
- Evidence required before adoption: Forge 1.20.1 BlockEntityRenderer contract, mod BER fixtures, cache invalidation tests and vertex-count limits.
- Completion state: COMPLETE — 766 logical lines read in contiguous ranges 1–400 and 401–766; SHA and 28650-byte size match; embedded tests inspected, not run.

### 1299. `crates/horizon-client/src/game/native_builds.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0e9714064b94183a1c8129b91ca8b55f3ec750a3`
- Size: 56971 bytes; 1424 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1424 (contiguous to EOF)
- Purpose: Android UI and workflow for compiling/download/installing native-mod game libraries from Starling or a custom build server, then selecting the library for the next app start.
- Dependencies / cross-references: Android activity/native library loader, native mod bundles, custom/published build server, GitHub build service, release metadata, APK stamp, SHA-256 and Mods UI.
- Implementation / logic: Lists available builds, submits native bundles, polls remote compile jobs, downloads libraries with range retries, verifies SHA-256, writes build metadata and selects/restarts into the new shared library.
- Findings: This is a high-privilege remote native-code supply chain. The resulting library runs with full game/app access; SHA-256 checks integrity against server-provided metadata but do not establish trust. Custom build server is explicitly supported and warned about.
- RustChunk relevance: Do not adopt for RustChunk. A launcher compiling arbitrary native mods remotely is not equivalent to Forge's normal mod loading and creates a major supply-chain/security boundary.
- Decision: **unnecessary; optional/future**
- 1.20.1 implications: Exclude remote native library builds and activity library swapping. Use Forge 1.20.1 mod loading with signed/trusted mod JARs and user consent; never load an untrusted downloaded .so into the launcher.
- Follow-up checks: If ever reconsidered, threat-model build server compromise, TLS/DNS, metadata authenticity, rollback, APK stamp bypass, custom server, ABI, native library permissions and update channel.
- RustChunk implementation mapping: No baseline mapping. Prefer Forge mod classloading and a separately isolated build pipeline, not app-library replacement.
- Evidence required before adoption: Formal threat model, signed build attestations, authenticated server metadata, reproducible builds, provenance and explicit trust/consent UX.
- Completion state: COMPLETE — 1424 logical lines read in contiguous ranges 1–400, 401–800, 801–1200 and 1201–1424; SHA and 56971-byte size match; no remote build/download/restart was run.

### 1300. `crates/horizon-client/src/game/native_claimed_items.rs`
- Source repository/ref: starling-client/starling @ `5432370d573642041585c69b40fb045b27133488`
- Source SHA: `98aac6fc0e8ac0448cc0ad85b7c054824055d567`
- Size: 12962 bytes; 343 logical lines
- Coverage: 1–343 (EOF)
- Purpose: Lets native client mods claim item right-clicks, receive bounded UseStart/UseStop events, inspect held-item data and raycast blocks including far LOD terrain.
- Dependencies / cross-references: Native mod Host API, item-use pipeline, player inventory/hand, event queue, item custom data SNBT, world clip ray and LOD ground samples.
- Implementation / logic: Stores up to 64 claimed item IDs per mod and 256 pending events, owns a claimed use by slot/tick, emits stop on release/reset and provides a raycast up to 1024 blocks.
- Findings: The extension is proprietary and not a Forge protocol feature. Events beyond the queue cap are silently dropped; LOD ray steps in whole blocks and returns terrain ground only, not exact far block geometry. Custom data uses newer component naming.
- RustChunk relevance: Optional future client-mod input abstraction; not needed for vanilla server play.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Use Forge 1.20.1 item-use hooks and legacy ItemStack NBT; if a client extension exists, bound queues and expose raycast results with explicit LOD approximation semantics.
- Follow-up checks: Test event overflow/ordering, claim replacement, duplicate item ownership, use interruption, slot changes, respawn/join, ray precision, max distance and server-authority boundaries.
- RustChunk implementation mapping: Possible mod_api/client_item_input.rs behind explicit opt-in; not created.
- Evidence required before adoption: Forge 47.4.x item interaction events, legacy NBT fixtures, queue overflow/property tests and far-terrain raycast error bounds.
- Completion state: COMPLETE — 343 logical lines read to EOF; SHA and 12962-byte size match; embedded tests inspected, not run.

### 1301. `crates/horizon-client/src/game_mode_switcher.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a3a4c4f5f31dfbe7334785f16a278613fbfa34a6`
- Size: 3702 bytes; 119 logical lines
- Coverage: 1–119 (EOF)
- Purpose: Implements the F3+F4 debug gamemode selector: mode icon selection, cycling, pointer hit testing, hover state and display geometry.
- Dependencies / cross-references: Debug HUD/input event routing, local game-mode state, the fixed mode table and GUI icon resources.
- Implementation / logic: Maps the selected mode to an icon and screen position, advances through available modes, and resolves pointer coordinates to one of the four selector entries.
- Findings: This is a debug-only selector, not the game-mode permission or server-authority system. The cycle path assumes every selected mode is present in the static mode table; missing/changed modes need defensive handling.
- RustChunk relevance: The small interaction/state model may inform a RustChunk debug overlay, but the source UI and mode enum are version-specific.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Use Minecraft 1.20.1's actual game-mode set and debug shortcut behavior. The selected mode must never locally grant permissions; server authority remains decisive.
- Follow-up checks: Verify target F3+F4 open/cycle semantics, icon assets, mode availability, pointer hit boxes, keyboard/touch interaction and unsupported-mode fallback.
- RustChunk implementation mapping: Possible `ui/debug/gamemode_switcher.rs`; not created.
- Evidence required before adoption: 1.20.1 debug-menu behavior, mode enum and permission tests; visual and input regression fixtures.
- Completion state: COMPLETE — 119 logical lines read to EOF; source SHA and 3702-byte size match the pinned tree; embedded tests inspected, not run.

### 1302. `crates/horizon-client/src/game_rules_screen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1e7ed412a93951ac026c84240c3c5476fbe60d0a`
- Size: 29558 bytes; 771 logical lines
- Coverage: 1–400, 401–771 (contiguous to EOF)
- Purpose: Implements the gamerule settings screen, including rule search, boolean controls, bounded integer editing, category layout, scrolling, focus, keyboard/IME behavior and apply/discard actions.
- Dependencies / cross-references: Game-rule registry/metadata, settings widgets, text input and Android IME bridge, screen narration, scroll/drag handling and the game/server rule state.
- Implementation / logic: Builds editable rows from rule metadata, filters them by query, validates numeric input against declared bounds, tracks pending changes and applies or discards the edited values.
- Findings: The screen relies on a newer rule registry and UI/widget contract. Its client-side range checks are not a substitute for server validation; an invalid focused field blocks completion. Exact rule names, defaults and categories differ from 1.20.1.
- RustChunk relevance: Gamerule visibility/editing is useful UX, but this implementation cannot be carried over as-is to a 1.20.1 Forge client.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild from the complete 1.20.1 gamerule set and Forge/server synchronization. For a normal multiplayer client, present authoritative received values and only send the correct permission-checked command/action.
- Follow-up checks: Compare all 1.20.1 rules and bounds, permission levels, value parsing, cancel/apply behavior, IME focus, touch scrolling, narration and stale-server rejection.
- RustChunk implementation mapping: Possible `ui/screens/game_rules.rs`; not created.
- Evidence required before adoption: 1.20.1 GameRules definitions, server command/permission behavior, Forge extensions, packet/command traces and Android IME/UI tests.
- Completion state: COMPLETE — 771 logical lines read in contiguous ranges 1–400 and 401–771; source SHA and 29558-byte size match the pinned tree; embedded tests inspected, not run.

### 1303. `crates/horizon-client/src/gizmos.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0c9bfbb82fdf0c84c8251b4920f6de786f3915be`
- Size: 22884 bytes; 612 logical lines
- Coverage: 1–400, 401–612 (contiguous to EOF)
- Purpose: Provides debug gizmo geometry and rendering helpers for lines, cuboids, arrows, chunk borders, hitboxes and camera-relative overlays.
- Dependencies / cross-references: Debug render state, camera matrices, GUI/world render passes, vertex buffers, clipping and game/entity geometry.
- Implementation / logic: Builds line and face geometry, transforms it relative to the camera, clips/filters visible portions and renders debug primitives through the modern gizmo path.
- Findings: Geometry concepts are reusable, but the renderer and gizmo API are Horizon-specific. Degenerate arrows, zero-length vectors and malformed/non-finite bounds require explicit target-side validation.
- RustChunk relevance: Useful reference for optional debug hitboxes/chunk-border overlays; not part of the normal Minecraft client rendering path.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Use the 1.20.1 renderer/debug-render hooks and the target camera/projection conventions. Keep debug geometry out of the normal chunk mesh path.
- Follow-up checks: Compare 1.20.1 F3 chunk borders/hitboxes, clipping and partial ticks; test zero-length arrows, inverted bounds, NaN/infinite inputs and camera-origin precision.
- RustChunk implementation mapping: Possible `render/debug/gizmos.rs`; not created.
- Evidence required before adoption: 1.20.1 debug-render call sites, target matrix/vertex APIs and deterministic geometry/render-golden tests.
- Completion state: COMPLETE — 612 logical lines read in contiguous ranges 1–400 and 401–612; source SHA and 22884-byte size match the pinned tree; embedded tests inspected, not run.

### 1304. `crates/horizon-client/src/gossip.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9ea4a668d406f8e3d764801f8cedd03cba9c8f6d`
- Size: 13683 bytes; 394 logical lines
- Coverage: 1–394 (EOF)
- Purpose: Implements villager gossip/reputation storage, weighted reputation aggregation, caps, decay, transfer between villagers and NBT serialization.
- Dependencies / cross-references: Villager AI/trading, gossip type weights and caps, world day/time, entity persistence, random transfer and server-side village behavior.
- Implementation / logic: Stores gossip by target UUID and type, combines weighted values into reputation, clamps values, applies day-based decay and transfers gossip during villager interactions.
- Findings: This is server gameplay state, not a remote-client responsibility. Malformed or out-of-range persisted values need normalization before arithmetic; target-era gossip types, weights and decay rules must be checked independently.
- RustChunk relevance: The model is only a reference if RustChunk later includes an integrated/LAN server. A multiplayer client should consume synchronized villager behavior rather than simulate reputation.
- Decision: **unnecessary; reference; version-specific**
- 1.20.1 implications: Do not add villager gossip simulation to the remote-client baseline. If an integrated server is later added, implement the exact 1.20.1 gossip types, weighting, caps, decay and transfer authority.
- Follow-up checks: Compare 1.20.1 GossipType weights/caps, UUID persistence, day-decay timing, transfer RNG order and malformed-NBT handling; test overflow and negative values.
- RustChunk implementation mapping: No baseline mapping; possible server-side `world/villager/gossip_1_20_1.rs` only for a future integrated server.
- Evidence required before adoption: 1.20.1 Java source/mappings, villager gossip NBT fixtures, seeded interaction traces and day-boundary tests.
- Completion state: COMPLETE — 394 logical lines read to EOF; source SHA and 13683-byte size match the pinned tree; embedded tests inspected, not run.

### 1305. `crates/horizon-client/src/health.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c8d8abaed8433479aa9c310cc741e9f60b9f47d2`
- Size: 50685 bytes; 1328 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1328 (contiguous to EOF)
- Purpose: Contains the Horizon health/damage model for resident players: health and absorption, damage sources, armor/effects, knockback, blocking, totem behavior, environmental damage, healing and hurt-state handling.
- Dependencies / cross-references: Entity attributes, player inventory/equipment, damage catalogs and tags, entity/world collision, server tick hooks, status effects, sound events and player lifecycle.
- Implementation / logic: Applies damage through a set of combat/environment paths, updates health/absorption and hurt state, evaluates armor and defensive items, and emits callbacks/events for the surrounding game loop.
- Findings: The module documents and contains incomplete/local-server assumptions: some entity attribution and gameplay hooks are stubs or no-ops, and player-harm behavior is permissive. It is not proof of a complete 1.20.1 damage simulation. Remote client health must be driven by server packets.
- RustChunk relevance: Health HUD and client-side presentation are essential, but this combined Horizon-side damage model must not be trusted as the 1.20.1 multiplayer authority.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Implement 1.20.1 health/absorption/status and damage presentation from authoritative entity metadata, health/experience packets and events. Do not simulate server damage in the remote client.
- Follow-up checks: Compare 1.20.1 damage-source registry, armor/toughness/enchantments, shields, totems, absorption, starvation/freezing/drowning, knockback, PVP rules, hurt animation and death/respawn ordering.
- RustChunk implementation mapping: Possible `gameplay/health_sync_1_20_1.rs` and `render/hud/health.rs`; not created.
- Evidence required before adoption: 1.20.1 Java damage pipeline, Forge damage events, packet traces and deterministic fixtures for armor, shields, totems, absorption and death.
- Completion state: COMPLETE — 1328 logical lines read in four contiguous ranges through EOF; source SHA and 50685-byte size match the pinned tree; embedded tests inspected, not run.

### 1306. `crates/horizon-client/src/hotbar.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dccae7560bab3a7250ebded00ad5e5bed77e86a5`
- Size: 5188 bytes; 141 logical lines
- Coverage: 1–141 (EOF)
- Purpose: Reads and writes saved creative hotbar groups, normalizes group/item data and filters unsupported entries.
- Dependencies / cross-references: Creative inventory UI, item catalog/registry, NBT/JSON serialization, saved hotbar files and item-stack conversion.
- Implementation / logic: Loads named hotbar groups, parses their entries, converts item data, applies count limits and writes the selected hotbar set back to storage.
- Findings: The persistence/validation pattern is useful, but the data fixer and item-stack representation are from a newer format. It silently filters unknown items and clamps counts, which can hide compatibility or data-loss problems.
- RustChunk relevance: Saved creative hotbars are useful launcher/game UX; this format is not a drop-in 1.20.1 format.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Use the target 1.20.1 creative hotbar file/NBT schema and legacy ItemStack serialization. Preserve unknown/unreadable entries where safe rather than silently discarding them.
- Follow-up checks: Verify 1.20.1 hotbar file names/schema, stack counts, item IDs, malformed entries, migration behavior and round-trip preservation.
- RustChunk implementation mapping: Possible `inventory/creative_hotbar_1_20_1.rs`; not created.
- Evidence required before adoption: 1.20.1 client hotbar persistence code, sample files, migration fixtures and malformed/unknown-item round-trip tests.
- Completion state: COMPLETE — 141 logical lines read to EOF; source SHA and 5188-byte size match the pinned tree; embedded tests inspected, not run.

### 1307. `crates/horizon-client/src/hunger.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `318986b6ec2d59070bf3a91ccf07bff60e3cc06b`
- Size: 7157 bytes; 200 logical lines
- Coverage: 1–200 (EOF)
- Purpose: Implements server-side FoodData behavior: food/saturation/exhaustion, peaceful regeneration, healing, starvation and tick transitions.
- Dependencies / cross-references: Player health, difficulty, hunger/saturation values, game rules, server tick timing and entity damage/heal callbacks.
- Implementation / logic: Consumes exhaustion, converts saturation/food as required, applies timed regeneration/starvation according to game conditions and skips inappropriate player states.
- Findings: This is a server simulation module. A remote client must display synchronized food/saturation values and should not independently apply damage or regeneration. Exact thresholds and conditions are version-specific.
- RustChunk relevance: Hunger HUD synchronization is essential; local FoodData authority is unnecessary for joining a remote server.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Implement 1.20.1 hunger/food packet and metadata presentation; use authoritative server updates rather than running the Horizon tick logic client-side.
- Follow-up checks: Verify 1.20.1 FoodData exhaustion threshold, regeneration/starvation intervals, peaceful behavior, invulnerable/spectator exclusions and packet update order.
- RustChunk implementation mapping: Possible `gameplay/food_sync_1_20_1.rs`; not created.
- Evidence required before adoption: 1.20.1 FoodData Java source, packet traces and deterministic hunger tick fixtures across difficulty/game-rule modes.
- Completion state: COMPLETE — 200 logical lines read to EOF; source SHA and 7157-byte size match the pinned tree; embedded tests inspected, not run.

### 1308. `crates/horizon-client/src/import.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4705976f9b52b4a88bc6ee26db092868c68d2bb5`
- Size: 27098 bytes; 731 logical lines
- Coverage: 1–400, 401–731 (contiguous to EOF)
- Purpose: Implements Android import/share handling for resource packs, worlds, mods, content archives, skins and other incoming files, including classification and extraction.
- Dependencies / cross-references: Android content/share intents, SAF/storage access, inbox staging, ZIP handling, path canonicalization, profile/world directories, mod/pack metadata and worker tasks.
- Implementation / logic: Classifies incoming files, stages them, extracts/places content into selected destinations, filters launcher-private or unsafe data and uses temporary `.part` files for parts of the transfer workflow.
- Findings: ZIP import lacks explicit total-uncompressed-size, per-entry-size, entry-count and compression-ratio quotas in the reviewed path, leaving a ZIP-bomb/disk-exhaustion risk. Canonical-path checks reject links outside the allowed directory, but path validation followed by a later read may be vulnerable to filesystem races. Native `.hnmod` content is a high-privilege Horizon format, not a Forge JAR.
- RustChunk relevance: Android import UX and safe staging are useful, but the Horizon formats and destination model require a RustChunk-specific design.
- Decision: **adaptable; rewrite required; version-specific**
- 1.20.1 implications: Import only supported 1.20.1 game data/mod JAR/resource packs with explicit user consent. Exclude account tokens/private launcher files and enforce strict archive quotas, path checks and atomic finalization.
- Follow-up checks: Add ZIP-bomb quotas and entry limits; test traversal, symlink/TOCTOU, duplicate names, partial failure/rollback, large files, URI permissions, mod-JAR trust and profile isolation.
- RustChunk implementation mapping: Possible `platform/android/import/` plus a separately tested archive extractor; not created.
- Evidence required before adoption: Android SAF/share behavior, Forge 1.20.1 mod/resource-pack layout, malicious ZIP corpus, rollback and permission-lifecycle tests.
- Completion state: COMPLETE — 731 logical lines read in contiguous ranges 1–400 and 401–731; source SHA and 27098-byte size match the pinned tree; embedded tests inspected, not run.

### 1309. `crates/horizon-client/src/inv_sort.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d8b44529a8168e7524c0c0814c0208f2817964fe`
- Size: 8594 bytes; 239 logical lines
- Coverage: 1–239 (EOF)
- Purpose: Implements an inventory sort operation that simulates item movement/stack merging and arranges stacks by a chosen ordering.
- Dependencies / cross-references: Creative inventory/container slots, ItemStack operations, slot restrictions, item categories and client click simulation.
- Implementation / logic: Builds a target order, simulates merge/left-click operations and produces a sequence of slot interactions that sorts inventory contents.
- Findings: This is an extra Horizon inventory feature, not vanilla behavior. A local click simulation can diverge from server slot rules, item-specific restrictions and modded container semantics.
- RustChunk relevance: Optional convenience feature only; not required for RustChunk to join vanilla/Forge servers.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not include inventory sorting in the compatibility baseline. If later implemented, route actions through server-validated container clicks and never assume client simulation is authoritative.
- Follow-up checks: If reconsidered, test slot constraints, armor/offhand, bundles, stack limits, modded slots, shift-click and server correction/rejection.
- RustChunk implementation mapping: No baseline mapping; optional `ui/inventory/sort` only after a separate feature decision.
- Evidence required before adoption: 1.20.1 container slot rules, server click traces and adversarial fixtures for modded inventories.
- Completion state: COMPLETE — 239 logical lines read to EOF; source SHA and 8594-byte size match the pinned tree; embedded tests inspected, not run.

### 1310. `crates/horizon-client/src/item_activation.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ef4c3833b704671f47c58618e0482fb3a9ccb2f0`
- Size: 5605 bytes; 144 logical lines
- Coverage: 1–144 (EOF)
- Purpose: Calculates the totem-of-undying activation overlay animation, including elapsed ticks, random placement, rotation, scale and easing.
- Dependencies / cross-references: Totem activation event, HUD overlay renderer, random-number source, partial-tick interpolation and GUI dimensions.
- Implementation / logic: Creates animation state on activation and derives the icon transform/position over its lifetime, then expires the overlay.
- Findings: The animation concept is reusable, but asset names, event integration and random draw order are version-specific. Any deterministic visual tests need the same seeded/random sequence as the target.
- RustChunk relevance: Totem activation feedback is part of client HUD presentation and can be rebuilt independently of Horizon.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Recreate the 1.20.1 totem overlay timing and rendering with target assets and event order; do not couple it to the server damage model.
- Follow-up checks: Check 1.20.1 activation packet/event, duration, partial tick behavior, random position/rotation draw order, screen resize and repeated activation.
- RustChunk implementation mapping: Possible `render/hud/totem_overlay.rs`; not created.
- Evidence required before adoption: 1.20.1 HUD source/assets, event packet traces and deterministic frame-by-frame visual tests.
- Completion state: COMPLETE — 144 logical lines read to EOF; source SHA and 5605-byte size match the pinned tree; embedded tests inspected, not run.

### 1311. `crates/horizon-client/src/item_render.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4080e463d4c3be10bfe64435e3242f73e63b7d5b`
- Size: 56460 bytes; 1439 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1439 (contiguous to EOF)
- Purpose: Implements the modern item renderer core: item model selection, baked model/quad rendering, atlas/model resources, display contexts, tint, foil/glint, special models and CPU/GPU render paths.
- Dependencies / cross-references: Item registry and stack format, model/texture atlas loaders, resource reload, render buffers, matrix/pose state, lighting, GUI/world render passes and specialized item renderers.
- Implementation / logic: Resolves an item to its model, chooses model/quad variants and display transforms, applies tints/foil and emits geometry through renderer-specific paths; resource/model data is cached and reused.
- Findings: The renderer is a core subsystem, but this implementation is based on modern item components/model/display contexts and a newer renderer contract. Minecraft 1.20.1 uses older ItemStack NBT, item model JSON, overrides and Forge extension points. Cache invalidation and missing/unknown model behavior need explicit target tests.
- RustChunk relevance: Item rendering is essential for inventory, hand, dropped items and GUI. Reuse concepts only; rebuild around the target renderer and old resource format.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Implement 1.20.1 baked model/ItemRenderer behavior, JSON parents/overrides, display transforms, tint, glint and Forge model hooks. Do not import 26.3 component semantics.
- Follow-up checks: Trace 1.20.1 model loader/atlas lifecycle, override predicates, perspective transforms, tint indices, foil, lighting, missing models, modded item models and cache invalidation on reload.
- RustChunk implementation mapping: Possible `render/item/` subsystem with target model resolver and GPU backend; not created.
- Evidence required before adoption: 1.20.1 Java/Forge ItemRenderer and model bakery sources, representative vanilla/mod item models, resource reload fixtures and render-golden tests.
- Completion state: COMPLETE — 1439 logical lines read in four contiguous ranges through EOF; source SHA and 56460-byte size match the pinned tree; embedded tests inspected, not run.

### 1312. `crates/horizon-client/src/item_render/cit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7faaedb09590cd494f35fa73659d8d70c9cdcf17`
- Size: 32249 bytes; 810 logical lines
- Coverage: 1–400, 401–800, 801–810 (contiguous to EOF)
- Purpose: Implements an OptiFine-style Custom Item Textures (CIT) extension: rules, aliases, alternate models/textures, damage and worn conditions, enchantment glint and caching.
- Dependencies / cross-references: Resource-pack parser, item renderer/model loader, item identity/damage/enchantment data, texture atlases and rule priority.
- Implementation / logic: Loads CIT definitions and matching assets, evaluates item conditions and switches model/texture or visual effects when a rule matches.
- Findings: CIT is an optional compatibility extension, not vanilla 1.20.1 behavior. This implementation assumes modern stack components and Horizon model loading. Rule priority, pack override semantics and damage/enchantment data need an independent versioned adapter.
- RustChunk relevance: Not required for Forge 1.20.1 multiplayer baseline; may be a future resource-pack compatibility feature.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not include CIT in core item rendering. If later offered, treat it as an optional pack extension layered over the 1.20.1 item/model pipeline.
- Follow-up checks: Evaluate actual 1.20.1 OptiFine CIT syntax and common pack expectations; test rule priority, overlapping packs, item damage, enchantment matching and reload cleanup.
- RustChunk implementation mapping: No baseline mapping; optional `resource_packs/cit/` adapter only after a separate decision.
- Evidence required before adoption: Representative 1.20.1 CIT packs, rule parser corpus, resource reload and compatibility tests.
- Completion state: COMPLETE — 810 logical lines read in contiguous ranges 1–400, 401–800 and 801–810; source SHA and 32249-byte size match the pinned tree; embedded tests inspected, not run.

### 1313. `crates/horizon-client/src/item_render/gui.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a25ae62bf11f987ed6bc7628ab301eec3bb5b0e3`
- Size: 25712 bytes; 781 logical lines
- Coverage: 1–400, 401–781 (contiguous to EOF)
- Purpose: Contains GUI-specific item rendering helpers for lighting, UV/polygon clipping, face depth sorting, glint, book pose and cached sprite/render state.
- Dependencies / cross-references: GUI item renderer, model quads, sprite/atlas tables, matrix/pose operations, GUI lighting and resource reload.
- Implementation / logic: Transforms item quads into GUI coordinates, clips and sorts faces for depth correctness, applies GUI lighting/foil and reuses render-related cached data.
- Findings: The helpers are useful, but their types and draw contract are Horizon-specific. A cache keyed by pointer/length may become stale if the underlying sprite table mutates in place; verify target resource reload ownership before copying that pattern.
- RustChunk relevance: GUI item rendering is essential for inventory and menus; this implementation needs a target-specific rewrite.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild against 1.20.1 GUI lighting, BakedModel quads, sprite atlas and Forge resource reload lifecycle.
- Follow-up checks: Compare GUI item light values, face order, UV clipping, depth behavior, book pose, glint, pointer/cache invalidation and sprite reload tests.
- RustChunk implementation mapping: Possible `render/item/gui.rs`; not created.
- Evidence required before adoption: 1.20.1 GUI item renderer source, atlas reload fixtures and pixel/render-golden tests across flat and 3D items.
- Completion state: COMPLETE — 781 logical lines read in contiguous ranges 1–400 and 401–781; source SHA and 25712-byte size match the pinned tree; embedded tests inspected, not run.

### 1314. `crates/horizon-client/src/item_render/gui/depth.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aaaa0c55dcc20a072ad042fca179cd74593a3f50`
- Size: 4081 bytes; 124 logical lines
- Coverage: 1–124 (EOF)
- Purpose: Implements GUI item face-depth sorting, including splitting intersecting faces and interpolating their UV coordinates.
- Dependencies / cross-references: GUI quad renderer, polygon geometry, face transforms, depth ordering and UV interpolation.
- Implementation / logic: Computes face depths, detects intersections, splits polygons and orders fragments so GUI item geometry is drawn in a stable depth order.
- Findings: The geometric technique is reusable, but target integration is renderer-specific. Degenerate, coplanar, non-finite or adversarial geometry needs guarded behavior; the source has no demonstrated fragment-count safety cap.
- RustChunk relevance: A reference algorithm for correct rendering of overlapping 3D GUI item faces, not a standalone engine requirement.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Only port the geometry approach if the 1.20.1 renderer has the same ordering defect; retain explicit bounds and numeric validation.
- Follow-up checks: Test zero-area faces, coplanar/intersecting polygons, NaN/infinite coordinates, UV continuity, deterministic ordering and maximum fragment growth.
- RustChunk implementation mapping: Possible helper inside `render/item/gui.rs`; not created.
- Evidence required before adoption: 1.20.1 GUI rendering behavior, generated intersecting-quad fixtures and deterministic image tests.
- Completion state: COMPLETE — 124 logical lines read to EOF; source SHA and 4081-byte size match the pinned tree; embedded tests inspected, not run.

### 1315. `crates/horizon-client/src/item_render/special.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `802ab97d3e7bafcaad9b40fd58549a26e629f8ab`
- Size: 31115 bytes; 882 logical lines
- Coverage: 1–400, 401–800, 801–882 (contiguous to EOF)
- Purpose: Implements special 3D item renderers for bells, books, conduit, chests, shulker boxes, heads/player heads, tridents, banners, shields, decorated pots and newer special models.
- Dependencies / cross-references: Special item model registry, block/item renderers, atlas sprites, block entity render models, transformations, dye/pattern data, foil and culling.
- Implementation / logic: Selects specialized model geometry and textures, applies item display transforms and lighting, and emits the required render parts for each special item family.
- Findings: The architecture is useful, but model kinds/assets and data are from a newer version. Copper Golem statue and EndCube-related content are not 1.20.1 baseline. Banner/shield layer limits and fallback behavior need validation; embedded tests were inspected only.
- RustChunk relevance: Special item rendering is required for a complete renderer, but each 1.20.1 special model and data source must be reconstructed.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Implement only the 1.20.1 special item set and Forge extension hooks, using legacy item NBT/pattern/dye data and target renderer model types.
- Follow-up checks: Check each 1.20.1 special renderer, player-head profiles, banner/shield patterns, trident, chest/shulker, conduit, book and decorated-pot data; test missing assets, transformations, foil and culling.
- RustChunk implementation mapping: Possible `render/item/special_1_20_1.rs`; not created.
- Evidence required before adoption: 1.20.1 Java/Forge special item renderers, model/texture assets, NBT fixtures and render-golden tests.
- Completion state: COMPLETE — 882 logical lines read in contiguous ranges 1–400, 401–800 and 801–882; source SHA and 31115-byte size match the pinned tree; embedded tests inspected, not run.

### 1316. `crates/horizon-client/src/item_sync.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1158c71be535d111db0c4118999110633b3e9ace`
- Size: 47772 bytes; 1185 logical lines
- Coverage: 1–400, 401–800, 801–1185 (contiguous to EOF)
- Purpose: Serializes and deserializes modern network ItemStacks, mapping registry IDs and DataComponentPatch entries to item data/NBT and handling dynamic registry-backed component values.
- Dependencies / cross-references: Item and component registries, network stack codec, enchantments/potions/trims, container contents, player profiles, block-entity data and packet stack readers/writers.
- Implementation / logic: Encodes the current item id/count and component patch, converts component payloads using registry context, and applies known values while skipping or preserving cases according to the codec.
- Findings: The entire wire contract is for modern component-based ItemStacks. Minecraft 1.20.1 uses legacy stack/NBT packet encoding and different fields. Modern `glider`/`equippable` and related component names are not 1.20.1 equivalents.
- RustChunk relevance: Item-stack network synchronization is essential, but this serializer is not reusable as a 1.20.1 codec; only registry-mapping and defensive conversion concepts may transfer.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Build the exact 1.20.1 ItemStack network codec and legacy NBT semantics, then layer Forge capabilities/modded item data without adopting modern component IDs.
- Follow-up checks: Compare 1.20.1 FriendlyByteBuf ItemStack format, empty/count semantics, damage/enchantments/potions, container NBT, player profile, unknown mod items and Forge capability handling.
- RustChunk implementation mapping: Possible `protocol/item_stack_1_20_1.rs`; not created.
- Evidence required before adoption: 1.20.1 Java packet codec/mappings, packet captures, vanilla and Forge modded stack fixtures, malformed payload and round-trip tests.
- Completion state: COMPLETE — 1185 logical lines read in contiguous ranges 1–400, 401–800 and 801–1185; source SHA and 47772-byte size match the pinned tree; embedded tests inspected, not run.

### 1317. `crates/horizon-client/src/item_tooltip.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8bc29957828f1d2bebef855f169735c8813408f0`
- Size: 66147 bytes; 1740 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1740 (contiguous to EOF)
- Purpose: Builds item tooltip content and formatting from item data: names/lore, attributes, rarity, potion effects, map/player-head data, bees, books, jukeboxes, dyed items, fireworks, smithing templates and container contents.
- Dependencies / cross-references: Modern item components, localization/format styles, attribute modifiers, effect catalog, item registry, tooltip renderer, resource data and player/world context.
- Implementation / logic: Chooses which tooltip sections to show, reads item-specific metadata, formats component values and composes text/style rows in the expected display order.
- Findings: Much of the logic depends on post-1.20.1 DataComponents (tooltip display, ominous bottle, modern potion contents, jukebox playable and other newer fields). It cannot be copied into a legacy stack pipeline. Some effect/attribute display coverage is intentionally partial.
- RustChunk relevance: Tooltips are essential inventory UX, but need a 1.20.1 data adapter and target formatting rules.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild tooltips from legacy 1.20.1 ItemStack NBT, enchantments, potion data, attributes, custom name/lore and Forge item extensions; keep ordering and translations target-accurate.
- Follow-up checks: Verify tooltip ordering/flags, rarity, attribute operation formatting, potion/stew effects, map/book/profile/firework data, dyed items, container contents, localization and missing-component fallback.
- RustChunk implementation mapping: Possible `ui/item_tooltip_1_20_1.rs`; not created.
- Evidence required before adoption: 1.20.1 ItemStack tooltip Java source, vanilla/Forge item fixtures, translation snapshots and screenshot/render-golden tests.
- Completion state: COMPLETE — 1740 logical lines read in five contiguous ranges through EOF; source SHA and 66147-byte size match the pinned tree; embedded tests inspected, not run.

### 1318. `crates/horizon-client/src/kinetics.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ea2305632796232d95982b8702536b501c033f53`
- Size: 41234 bytes; 992 logical lines
- Coverage: 1–400, 401–800, 801–992 (contiguous to EOF)
- Purpose: Implements Create-style kinetic machinery: shafts, gears, gearboxes, motors, cranks, belts, water wheels, RPM propagation, network stress/capacity and gear phases.
- Dependencies / cross-references: Create content-mod block declarations, kinetic part metadata, machine registry data, entity/block state, mod-specific machinery and world simulation.
- Implementation / logic: Builds connectivity links between kinetic parts, solves source speed propagation and direction, detects jammed/overstressed networks, and calculates gear phases/rotation.
- Findings: This is Create/Horizon content-mod simulation, not vanilla Minecraft behavior. A remote client should render server-authoritative block/entity state and must not simulate machine authority. The machinery registry and modern content definitions are not Forge substitutes.
- RustChunk relevance: Not needed for the baseline client; some rotation/phase math may be a reference for future Create visual compatibility.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Exclude kinetic network simulation from the 1.20.1 core. If Create support is targeted, use the exact supported Create/Forge version and its synchronized state/render APIs.
- Follow-up checks: For future Create support, verify target Create RPM/stress algorithms, blockstate connection rules, belt groups, update packets, rotation interpolation and server/client authority.
- RustChunk implementation mapping: No baseline mapping; optional `mods/create/kinetics_visuals.rs` only after a separate compatibility decision.
- Evidence required before adoption: Target Create release source/mappings, Forge packet traces, machine fixtures and visual interpolation tests.
- Completion state: COMPLETE — 992 logical lines read in contiguous ranges 1–400, 401–800 and 801–992; source SHA and 41234-byte size match the pinned tree; embedded tests inspected, not run.

### 1319. `crates/horizon-client/src/lan/advancements.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ec7b44c6e2fd8c7ff5aa9427de32e8cf49d576c`
- Size: 40192 bytes; 1015 logical lines
- Coverage: 1–400, 401–800, 801–1015 (contiguous to EOF)
- Purpose: Implements LAN/dedicated-server advancement and recipe-book handling: first-tick criteria, visible advancement packets, progress updates, rewards, reload and recipe-display encoding.
- Dependencies / cross-references: Advancement tree/progress, recipe index and network recipe data, synchronized registries, server player lifecycle, recipe-book state and LAN hub packet dispatch.
- Implementation / logic: Grants tick criteria on first player tick, determines visible advancements, writes advancement/progress packets, unlocks recipe rewards and builds modern recipe display/recipe book entries.
- Findings: This is server/hub logic. Its output uses the modern protocol's recipe display and registry structures, not 1.20.1 recipe book/advancement packets. Unsupported recipe display types return no entry; support for NeoForge compound ingredients is an extension.
- RustChunk relevance: Not required to connect as a remote client; useful only if RustChunk later hosts a local/LAN server or implements server-side advancement authority.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not port this server outbound packet encoder into the client baseline. Implement the client-side 1.20.1 advancement/recipe-book packet handlers separately.
- Follow-up checks: If local hosting is added, compare 1.20.1 AdvancementProgress/PlayerAdvancements, recipe unlock order, packet fields, recipe serializers, reload visibility and Forge ingredient adapters.
- RustChunk implementation mapping: No baseline mapping; future server module `server/advancements_1_20_1.rs` only if hosting is approved.
- Evidence required before adoption: 1.20.1 server Java source/mappings, packet captures, vanilla/Forge recipe fixtures and reload/rejoin integration tests.
- Completion state: COMPLETE — 1015 logical lines read in contiguous ranges 1–400, 401–800 and 801–1015; source SHA and 40192-byte size match the pinned tree; embedded tests inspected, not run.

### 1320. `crates/horizon-client/src/lan/anticheat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fbb7346499f0330c003b27d5e70b7c4f10505db5`
- Size: 22177 bytes; 597 logical lines
- Coverage: 1–400, 401–597 (contiguous to EOF)
- Purpose: Implements Horizon dedicated-server anticheat checks for timer, combat, movement prediction and ground claims, with strike persistence, alerts, logging, kicks and bans.
- Dependencies / cross-references: LAN/dedicated server hub, player movement/attack packets, prediction replay, collision scene, server tick timing, anticheat settings and ban/strike files.
- Implementation / logic: Accumulates violation levels, classifies suspicious actions, optionally drops/setbacks actions, logs flags, sends operator alerts and persists kicks/ban thresholds.
- Findings: This is server-side security logic, not a client feature. The module comments explicitly distinguish dedicated server from LAN mode. Strike persistence uses a direct file write rather than an atomic replacement, so interruption may corrupt the file; vehicle allowances also include later entity types.
- RustChunk relevance: No baseline client mapping. Client-side anticheat would be ineffective and would risk false positives; authoritative validation belongs to the server.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not port anticheat checks into RustChunk's remote client. If RustChunk later hosts a server, build a separate 1.20.1/Forge-aware server security design.
- Follow-up checks: For any future host, test lag/teleport exemptions, movement prediction against 1.20.1 physics, false positives, strike-file atomicity, ban recovery and Forge entity/movement extensions.
- RustChunk implementation mapping: No baseline mapping; separate future server module only after product scope and threat model approval.
- Evidence required before adoption: 1.20.1 server movement/attack code, Forge event semantics, packet traces, lag simulations and persistent-strike recovery tests.
- Completion state: COMPLETE — 597 logical lines read in contiguous ranges 1–400 and 401–597; source SHA and 22177-byte size match the pinned tree; embedded tests inspected, not run.

### 1321. `crates/horizon-client/src/lan/attributes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eef565409df0b7545deadc8953a9ffa25f6c9777`
- Size: 6107 bytes; 185 logical lines
- Coverage: 1–185 (EOF)
- Purpose: Converts live attribute instances to network snapshots and applies received snapshots, including base values and ordered modifiers.
- Dependencies / cross-references: Attribute registry/IDs, entity attribute instances, modifier IDs/operations, dirty tracking, player/entity packets and live attribute observer callbacks.
- Implementation / logic: Filters syncable attributes, collects/clears dirty handles, serializes base/modifier data and applies incoming snapshots by matching registry IDs; unknown attributes are skipped.
- Findings: The synchronization concept is essential, but Horizon uses newer attribute registries and packet types. Unknown string or numeric modifier operation values default to AddValue, which must be checked against 1.20.1's decoding/error behavior.
- RustChunk relevance: Attribute synchronization affects movement speed, health, attack, armor and many modded mechanics; the implementation must be rebuilt for 1.20.1.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Implement target registry names/IDs, modifier UUID/identifier semantics, operation mapping, dirty updates and the 1.20.1 UpdateAttributes packet layout.
- Follow-up checks: Compare 1.20.1 AttributeMap syncability, default-vs-instantiated attributes, modifier ordering, operation IDs, unknown attributes, duplicate modifiers and Forge-added attributes.
- RustChunk implementation mapping: Possible `protocol/entity_attributes_1_20_1.rs`; not created.
- Evidence required before adoption: 1.20.1 Java attribute packet source/mappings, captures for player/mob/modded entities and round-trip/apply tests.
- Completion state: COMPLETE — 185 logical lines read to EOF; source SHA and 6107-byte size match the pinned tree; embedded tests inspected, not run.

### 1322. `crates/horizon-client/src/lan/border.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b4e3ffc63d04146701fbfb0c4222cc1f53b0e4c7`
- Size: 4491 bytes; 130 logical lines
- Coverage: 1–130 (EOF)
- Purpose: Tracks the world border state last sent to connected players and constructs outbound packets for initialization and border changes.
- Dependencies / cross-references: WorldBorder state, center/size interpolation, warning thresholds, packet encoder and LAN/dedicated-server broadcast lifecycle.
- Implementation / logic: Compares current border values with the last-sent snapshot, avoids resending an unchanged lerp countdown and emits packets only for changed center, size/lerp or warning settings.
- Findings: This is the server-side broadcaster rather than the client border renderer/interpolator. The change-detection pattern is reusable, but target packet structures and timing must be verified for 1.20.1.
- RustChunk relevance: World border display is a client feature; this outbound helper is only a reference for an optional local server.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Implement the receiving/interpolating 1.20.1 world-border state from server packets. Do not port the server's broadcast manager as the client implementation.
- Follow-up checks: Verify 1.20.1 initialize/set-size/lerp/center/warning packets, paused ticks, lerp completion and update ordering.
- RustChunk implementation mapping: Possible `world/border_sync_1_20_1.rs`; not created.
- Evidence required before adoption: 1.20.1 world-border Java source/mappings, packet captures and deterministic tick/interpolation tests.
- Completion state: COMPLETE — 130 logical lines read to EOF; source SHA and 4491-byte size match the pinned tree; embedded tests inspected, not run.

### 1323. `crates/horizon-client/src/lan/chunks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6b58f6bfb3cb818de92be2d2248e3cfef3f6136a`
- Size: 85008 bytes; 2102 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2102 (contiguous to EOF)
- Purpose: Implements outbound chunk/biome/light packet encoding for resident and saved columns, including palette conversion, block entity update tags, heightmaps, light masks/layers, cache management and multi-threaded warm-up.
- Dependencies / cross-references: Chunk/section storage, block-state and biome registries, block-entity containers, heightmap save tables, light storage, dimension layout, packet codecs, LAN world state and background worker pool.
- Implementation / logic: Converts stored block palette IDs into first-seen network palettes, serializes sections and block entities, derives light sections from neighboring non-air sections, writes chunk-with-light packet bodies and caches section/heightmap encodings keyed by immutable Arc section identity. A warm-up pool pre-encodes batches before packet assembly.
- Findings: This is the server-to-client encoder in the LAN path, not the remote client's inbound chunk decoder. Its protocol and block-entity mapping include later registries/types and Starling-only machine entities; do not treat it as a 1.20.1 chunk codec. The palette/cache/light architecture is a useful reference. Cache identity relies on immutable Arc sections and edit-by-copy. The code also emulates Java/fastutil block-entity map iteration order, which should not be assumed identical to target 1.20.1 without proof.
- RustChunk relevance: Chunk decoding, lighting and mesh updates are core RustChunk concerns, but this outbound implementation is not the required client path. Only isolated algorithms such as palette conversion, light-layer layout and cache invalidation may inform a separately versioned 1.20.1 decoder/renderer.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Build the exact 1.20.1 inbound LevelChunkWithLight and LightUpdate decoders, 1.20.1 block/biome palettes and block-entity NBT mapping. Exclude Starling machine entity type 4000 and post-1.20.1 block entities; keep server authority and chunk lifecycle separate.
- Follow-up checks: Compare 1.20.1 packet captures and section palette modes, global bits, biome data, heightmap NBT, block-entity ordering/tags, light masks/layers, dimension min_y/height, empty/edge sections, edit invalidation and large-batch memory/worker behavior.
- RustChunk implementation mapping: Possible reusable concepts in `world/chunk_palette.rs`, `world/light_decode.rs` and `render/chunk_cache.rs`; no files created.
- Evidence required before adoption: 1.20.1 Java packet codecs/mappings, vanilla and Forge packet captures, palette/light fixtures, NBT block-entity corpus, deterministic cache invalidation tests and Android memory/performance profiling.
- Completion state: COMPLETE — 2102 logical lines read in six contiguous ranges (1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2102); source SHA and 85008-byte size match the pinned tree; embedded tests inspected, not run.

### 1324. `crates/horizon-client/src/lan/code_of_conduct.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7dceb19319a246651439a9ee652b02fe7413c58b`
- Size: 5487 bytes; 159 logical lines
- Coverage: 1–159 (EOF)
- Purpose: Reads optional language-specific server code-of-conduct text files, strips formatting color codes, normalizes line endings and selects a language/fallback document.
- Dependencies / cross-references: Dedicated-server optional task negotiation, language code, filesystem directory contents, text encoding and configuration handshake.
- Implementation / logic: Reads sorted `.txt` files beneath a configured directory, canonicalizes paths to reject files resolving outside the directory, strips supported section-sign color codes, joins lines and selects the player's language, then `en_us`, then the first available entry.
- Findings: This is server configuration/optional-task preparation, not the client-side task UI. The canonical-parent check blocks straightforward external symlinks, but filesystem races and exact case/locale fallback behavior need platform-aware validation. The task itself is a modern protocol feature.
- RustChunk relevance: Not needed for normal 1.20.1 server joining; optional server-task handling could be considered only if target protocol exposes a compatible equivalent.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not add server filesystem code-of-conduct loading to the Android client. If handling optional server tasks, implement only the exact 1.20.1 client response/consent flow supported by server packets.
- Follow-up checks: Check 1.20.1 configuration task negotiation, locale fallback, text encoding, section-sign formatting, symlink/TOCTOU behavior and consent persistence.
- RustChunk implementation mapping: No baseline mapping; optional server task screen only if a compatible target protocol feature is found.
- Evidence required before adoption: 1.20.1 configuration protocol and optional task source/mappings, locale fixtures and Android acceptance/decline UI tests.
- Completion state: COMPLETE — 159 logical lines read to EOF; source SHA and 5487-byte size match the pinned tree; embedded tests inspected, not run.

### 1325. `crates/horizon-client/src/lan/command_order.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e908443f822bb575f55106a445e93fb81197fa77`
- Size: 28247 bytes; 892 logical lines
- Coverage: 1–400, 401–800, 801–892 (contiguous to EOF)
- Purpose: Stores a generated table of command-tree node paths and child names to impose deterministic registration/display order on the LAN command tree.
- Dependencies / cross-references: Brigadier command dispatcher, command registration nodes, server command completion/suggestion tree, command generator script and many Horizon/modern command extensions.
- Implementation / logic: Maps a path of command node names to a space-separated ordered list of child names. The table is generated from the current command registrations and is used to reorder children consistently.
- Findings: This is generated ordering metadata for Horizon's server command tree, not command parsing or client-side completion logic. It includes commands and arguments not present in vanilla 1.20.1 (e.g. compute, stopwatch, waypoint, modern test/time features) and omits/reshapes depending on the current server tree. Hardcoding it as target command order would be incorrect.
- RustChunk relevance: Not needed for a remote Minecraft client, which receives server command suggestions/tree and must respect that server's order. The generated-table technique may help a future local server produce deterministic output.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not port this 26.3/Horizon command list as a 1.20.1 Brigadier registry. Let the target server supply the command tree; if RustChunk later hosts a server, generate order from its exact Forge 1.20.1 registrations.
- Follow-up checks: Compare 1.20.1 dispatcher registrations and suggestion packet format; confirm ordering requirements from the server and generate from the exact enabled command set rather than static future names.
- RustChunk implementation mapping: No baseline mapping; possible build-time generator for a future local server command tree only.
- Evidence required before adoption: 1.20.1 Brigadier/Forge command registrations, command tree packet captures and deterministic suggestion ordering tests.
- Completion state: COMPLETE — 892 logical lines read in contiguous ranges 1–400, 401–800 and 801–892; source SHA and 28247-byte size match the pinned tree; embedded tests inspected, not run.

### 1326. `crates/horizon-client/src/lan/command_suggestions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e55e6ef92fd23d979f58e3ffb35f6a2cc8f95202`
- Size: 11272 bytes; 336 logical lines
- Coverage: 1–336 (EOF)
- Purpose: Provides generated ordering/selection metadata for command suggestion providers keyed by command argument paths.
- Dependencies / cross-references: Horizon/Brigadier command tree registrations, suggestion provider identifiers, command generator and server-side command dispatcher.
- Implementation / logic: Maps known command node paths to ordered suggestion provider names so the server can reproduce stable completion behavior.
- Findings: This is generated metadata for Horizon's server command system, not the client's command-tree decoder. It includes modern command paths and providers that do not exist in vanilla 1.20.1.
- RustChunk relevance: Not needed by a remote client; deterministic generation may be a reference only if RustChunk later hosts a local server.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Use the command tree and suggestions supplied by a 1.20.1 server. Do not hardcode this later-version provider table.
- Follow-up checks: Compare the 1.20.1 Brigadier suggestion packet/tree and Forge command registrations only if local hosting is added.
- RustChunk implementation mapping: No baseline mapping; optional server-side command metadata generator only.
- Evidence required before adoption: 1.20.1 Brigadier/Forge registrations and command-tree packet captures.
- Completion state: COMPLETE — 336 logical lines read to EOF; source SHA and 11272-byte size match the pinned tree; embedded tests inspected, not run.

### 1327. `crates/horizon-client/src/lan/commands.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa6193840d1daaf4f4f8cb60df6155a1fc7fcefe`
- Size: 15489 bytes; 434 logical lines
- Coverage: 1–400, 401–434 (contiguous to EOF)
- Purpose: Builds and sends the server's Brigadier command tree and command-completion data, including permission-aware command nodes and argument metadata.
- Dependencies / cross-references: LAN Hub, command registrar/dispatcher, permission levels, suggestion providers, protocol command-tree encoder and modern command argument registry.
- Implementation / logic: Collects registered server commands, filters or shapes nodes for a guest, encodes the tree and its argument/suggestion metadata for the client.
- Findings: The file is a server-side outbound encoder and uses Horizon's modern command/argument types. It is not the inbound command-tree implementation needed by a 1.20.1 client.
- RustChunk relevance: Not a remote-client subsystem; the client should parse the command tree advertised by its server.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Implement 1.20.1 ClientboundCommands decoding and Brigadier node reconstruction separately; preserve server-sent permissions and suggestions.
- Follow-up checks: Compare 1.20.1 command packet node indices, redirects, argument serializers, suggestion provider IDs and Forge command additions.
- RustChunk implementation mapping: No baseline mapping; future local-server module only.
- Evidence required before adoption: 1.20.1 Java/Forge command tree source, packet captures and parser fixtures.
- Completion state: COMPLETE — 434 logical lines read in contiguous ranges 1–400 and 401–434; source SHA and 15489-byte size match the pinned tree; embedded tests inspected, not run.

### 1328. `crates/horizon-client/src/lan/conn.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fb6750acef6ead34fcc5b5430d308183cea5313a`
- Size: 72052 bytes; 1908 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1908 (contiguous to EOF)
- Purpose: Implements the LAN server connection lifecycle from admission and login through configuration and play, including authentication, packet routing and disconnect handling.
- Dependencies / cross-references: LAN listener, guest/player state, protocol packet codecs, authentication/profile identity, configuration registries, resource/data synchronization and server tick channels.
- Implementation / logic: Maintains per-connection state, validates and dispatches inbound packets according to protocol phase, sends phase-specific responses and transitions guests into the active play hub.
- Findings: This is the server-side connection handler. Its authentication, packet phases and outbound responses are not a replacement for the Android client's clientbound handlers or client login flow; protocol details are from the pinned later version.
- RustChunk relevance: Architecture reference only for a future RustChunk integrated/LAN server; not part of joining an existing server as a client.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Build a separate 1.20.1 client connection state machine and packet handling path. Never copy later-version handshake or configuration packet layouts.
- Follow-up checks: Compare 1.20.1 login/configuration transitions, offline identity behavior, Forge handshake, disconnect reasons, timeout/backpressure and packet ordering.
- RustChunk implementation mapping: No baseline mapping; possible future `server/lan/connection.rs` only if hosting is scoped.
- Evidence required before adoption: 1.20.1 protocol captures, Forge handshake traces and state-machine integration tests.
- Completion state: COMPLETE — 1908 logical lines read in five contiguous ranges through EOF; source SHA and 72052-byte size match the pinned tree; embedded tests inspected, not run.

### 1329. `crates/horizon-client/src/lan/containers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7c8c2c785fcf400d8b2e8ed0f7aaa045d8c1dbe8`
- Size: 32069 bytes; 928 logical lines
- Coverage: 1–400, 401–800, 801–928 (contiguous to EOF)
- Purpose: Implements LAN Hub guest-facing container operations: nearby sound and level/entity events, opening/closing menus, inventory synchronization, click handling and container data slots.
- Dependencies / cross-references: LAN Hub and Guest, item-stack codecs, menu slot abstractions, block entities, server world state, sound/level-event packets and native-menu event queues.
- Implementation / logic: Broadcasts gameplay events to nearby guests, creates menu windows, validates guest clicks against server state, corrects client predictions and synchronizes changed slots and furnace/brewing/crafter data.
- Findings: This is authoritative server-side menu and event handling. Its click implementation is not the client's local prediction logic; later item-stack/component and menu packet structures also differ from 1.20.1.
- RustChunk relevance: Container interaction is essential to a playable client, but this source is the server authority side and cannot be used as the client packet receiver.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Implement 1.20.1 container open/content/slot/data packet handlers and local click prediction separately; accept server corrections and use legacy ItemStack NBT plus Forge menu extensions.
- Follow-up checks: Verify 1.20.1 container state IDs, click modes, carried stack, hidden/modded slots, shift-click, furnace/brewing data and stale-click correction.
- RustChunk implementation mapping: No baseline mapping; future `server/lan/containers.rs` only.
- Evidence required before adoption: 1.20.1 AbstractContainerMenu source, Forge menu APIs, packet captures and click/correction fixtures.
- Completion state: COMPLETE — 928 logical lines read in contiguous ranges 1–400, 401–800 and 801–928; source SHA and 32069-byte size match the pinned tree; embedded tests inspected, not run.

### 1330. `crates/horizon-client/src/lan/data.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `12c926f0b7130b38e993a0fef3c485004101577f`
- Size: 29506 bytes; 784 logical lines
- Coverage: 1–400, 401–784 (contiguous to EOF)
- Purpose: Provides LAN server registry/data synchronization and preparation of the data sent to joining guests, including registry-backed world/game data.
- Dependencies / cross-references: LAN connection configuration, synchronized registries/tags, world resources and packet serializers.
- Implementation / logic: Builds the server-side data snapshots used during the LAN configuration/play transition and serializes the appropriate registry-backed values for connected guests.
- Findings: This is a server-side data publisher using the newer protocol and registry model. It is not the 1.20.1 client's registry/tag decoder and cannot establish target registry IDs or payload layouts.
- RustChunk relevance: Registry synchronization is essential to multiplayer, but this is the outbound server side only.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Implement 1.20.1 clientbound registry/tag/configuration handling and Forge registry extensions from target-era sources; do not import modern registry payload shapes.
- Follow-up checks: Compare 1.20.1 registry synchronization, tags, recipe/biome data, configuration phase and modded registry negotiation.
- RustChunk implementation mapping: No baseline mapping; future LAN server data publisher only.
- Evidence required before adoption: 1.20.1 protocol/mappings, Forge handshake captures, registry/tag packet fixtures and unknown-registry tests.
- Completion state: COMPLETE — 784 logical lines read in contiguous ranges 1–400 and 401–784; source SHA and 29506-byte size match the pinned tree; embedded tests inspected, not run.

### 1331. `crates/horizon-client/src/lan/discovery.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `58e54bec7c5c7528f645d957344e26893624fa9c`
- Size: 8637 bytes; 249 logical lines
- Coverage: 1–249 (EOF)
- Purpose: Implements LAN server discovery by sending and receiving local-network discovery/status messages and collecting candidate servers.
- Dependencies / cross-references: UDP sockets, local network interfaces/broadcast or multicast behavior, server status response decoding and discovery result lifecycle.
- Implementation / logic: Sends discovery probes, receives responses and maintains the set of discovered LAN servers for the surrounding UI.
- Findings: This is optional discovery UX, not the Minecraft server connection protocol. Android multicast permissions, Wi-Fi lifecycle, duplicate responses and network changes need platform-specific handling.
- RustChunk relevance: Could be a useful optional convenience for discovering local servers on Android, but manual address entry and direct connection are sufficient for the baseline.
- Decision: **adaptable; reference; optional/future**
- 1.20.1 implications: Implement only if LAN discovery is desired; use Android-compatible multicast/UDP permissions and never assume all LAN servers advertise this Horizon discovery protocol.
- Follow-up checks: Test Android Wi-Fi lifecycle, multicast lock/permissions, subnet changes, duplicate/forged responses, timeout and manual-server fallback.
- RustChunk implementation mapping: Possible `platform/android/lan_discovery.rs`; not created.
- Evidence required before adoption: Android network permission/lifecycle tests and interoperability captures from supported 1.20.1 LAN servers.
- Completion state: COMPLETE — 249 logical lines read to EOF; source SHA and 8637-byte size match the pinned tree; embedded tests inspected, not run.

### 1332. `crates/horizon-client/src/lan/entity.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `32223100d56123a1b3fc07181dfb1c989e9ad623`
- Size: 83433 bytes; 2132 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2132 (contiguous to EOF)
- Purpose: Implements server-side entity tracking and synchronization: spawn/removal, visibility and relationships, metadata, attributes, motion and per-player entity update packets.
- Dependencies / cross-references: LAN Hub and Guest, entity registry, world/entity state, tracker ranges, view distance/chunks, protocol entity packet codecs and player lifecycle.
- Implementation / logic: Tracks which guests should know each entity, sends initial and incremental state, pairs/unpairs entities and emits updates as server entities move or change.
- Findings: The implementation is server-side tracking and packet production, not the client's entity packet decoder. It is tied to later registries, entity types and packet fields, so it cannot be copied as the 1.20.1 client entity lifecycle.
- RustChunk relevance: Entity synchronization is essential to multiplayer correctness, but RustChunk needs the receiving/rendering path and target-era entity metadata mapping.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Implement 1.20.1 clientbound spawn, remove, metadata, attributes, movement, passengers and relationship handlers separately; keep server visibility authority external.
- Follow-up checks: Compare 1.20.1 packet families, tracker range, entity ID reuse, metadata serializers, passenger ordering and Forge-added entity types.
- RustChunk implementation mapping: No baseline mapping; future `server/entity_tracking.rs` only if local hosting is added.
- Evidence required before adoption: 1.20.1 Java server/client packet handlers, packet captures and deterministic entity lifecycle tests.
- Completion state: COMPLETE — 2132 logical lines read in six contiguous ranges through EOF; source SHA and 83433-byte size match the pinned tree; embedded tests inspected, not run.

### 1333. `crates/horizon-client/src/lan/far.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `958d0bb3619a5b31a07e185bd1fe437419d26a6a`
- Size: 15377 bytes; 429 logical lines
- Coverage: 1–400, 401–429 (contiguous to EOF)
- Purpose: Implements server-side far-chunk delivery for distant players, maintaining a ring/cache of resident or saved columns and sending far terrain without generating or ticking those chunks.
- Dependencies / cross-references: LAN Hub, player view distance, saved chunk storage, dimension geometry, far-chunk packet format and background/cache state.
- Implementation / logic: Selects distant columns for each player, reuses cached/saved chunk data and schedules far terrain updates separately from normal nearby ticking.
- Findings: The algorithm is a server-side far-terrain delivery path, not a client renderer or authoritative chunk decoder. It relies on Horizon's custom far/LOD packet path and saved-column assumptions.
- RustChunk relevance: The separation between active nearby chunks and cheaper distant terrain is a useful architecture reference for RustChunk's performance goal, but the protocol and storage format are not directly reusable.
- Decision: **adaptable; reference; version-specific**
- 1.20.1 implications: Do not expect a 1.20.1 server to send Horizon far packets. Any distant terrain must be derived client-side or negotiated through an explicit compatible mod/protocol.
- Follow-up checks: Profile memory and scheduling under large view distances; compare saved vs generated columns, stale cache invalidation, dimension boundaries and client/server authority.
- RustChunk implementation mapping: Possible future `world/far_terrain_cache.rs`; not created.
- Evidence required before adoption: 1.20.1 chunk packet support, saved-chunk fixtures, memory benchmarks and visual seam/invalidations tests.
- Completion state: COMPLETE — 429 logical lines read in contiguous ranges 1–400 and 401–429; source SHA and 15377-byte size match the pinned tree; embedded tests inspected, not run.

### 1334. `crates/horizon-client/src/lan/forwarding.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `346b15935fe5b05c12b281e557241d5b388289a1`
- Size: 8111 bytes; 231 logical lines
- Coverage: 1–231 (EOF)
- Purpose: Implements server-side proxy forwarding identity verification, including Velocity-style signed forwarding data and HMAC validation.
- Dependencies / cross-references: LAN server login, proxy forwarding configuration, connection addresses, profile identity and cryptographic HMAC-SHA256 helpers.
- Implementation / logic: Parses forwarded identity/address data, validates its signature against configured secret material and returns the trusted profile/network identity to the server connection flow.
- Findings: This is server-side trust-boundary code, not a client login feature. Proxy forwarding must be explicitly configured and authenticated; accepting unsigned or mismatched forwarded identity would permit identity spoofing.
- RustChunk relevance: Not needed by the Android remote client; the target server owns proxy authentication and identity mapping.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not implement or trust proxy-forwarded identity in RustChunk's client. Follow the server's ordinary 1.20.1/Forge login handshake.
- Follow-up checks: If local hosting/proxy integration is ever added, verify signature inputs, timestamp/nonce handling, constant-time comparison, secret storage and replay resistance.
- RustChunk implementation mapping: No baseline mapping; separate future server security module only.
- Evidence required before adoption: Velocity forwarding specification, 1.20.1 proxy integration captures and cryptographic test vectors (not run in this audit).
- Completion state: COMPLETE — 231 logical lines read to EOF; source SHA and 8111-byte size match the pinned tree; embedded tests inspected, not run.

### 1335. `crates/horizon-client/src/lan/guest.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `82c2eabc8197ca0cf0829953b3c45ea0e7ab5737`
- Size: 66325 bytes; 1614 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1614 (contiguous to EOF)
- Purpose: Stores the LAN server's per-connected-player state and operations, including inventory, movement, chat, abilities, menus, attributes, recipes and outgoing packet queues.
- Dependencies / cross-references: LAN Hub and connection handler, entity/player models, item registry and modern ItemStack format, scoreboard, chat validation, packet serialization and profile data.
- Implementation / logic: Maintains authoritative guest state, applies inbound player actions, builds/queues clientbound updates and exposes the per-player view used by the server hub.
- Findings: The module is a server-side player/session model. It includes modern component-oriented stack and protocol structures and is not a substitute for RustChunk's client player state or 1.20.1 legacy item handling.
- RustChunk relevance: Only an architectural reference if RustChunk later implements an integrated/LAN server; remote-client baseline needs packet-driven state.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Build the client player/inventory model from 1.20.1 clientbound packets and local input; keep server validation and authoritative inventory state external.
- Follow-up checks: Compare target player abilities, inventory, chat/session, attributes, menu state, respawn and Forge capability synchronization.
- RustChunk implementation mapping: No baseline mapping; future `server/guest.rs` only if hosting is added.
- Evidence required before adoption: 1.20.1 player/server source, Forge handshake/packet traces and inventory/session lifecycle tests.
- Completion state: COMPLETE — 1614 logical lines read in five contiguous ranges through EOF; source SHA and 66325-byte size match the pinned tree; embedded tests inspected, not run.

### 1336. `crates/horizon-client/src/lan/handoff.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6c0d84664cf7178100bd19c4f08388293ea39e27`
- Size: 4681 bytes; 132 logical lines
- Coverage: 1–132 (EOF)
- Purpose: Allows the in-process single-player server and client window to share identical chunk-section allocations after the window decodes the normal chunk packet.
- Dependencies / cross-references: Chunk/section storage, Arc/Weak ownership, packet decode path, block/biome/light equality and the memory-connection handoff between integrated server and client.
- Implementation / logic: The server offers weak references to sections; after decoding, the client adopts only sections that still exist and compare equal, preserving isolation through copy-on-write when either side edits.
- Findings: This optimization is specific to a same-process single-player server/client memory connection. It does not affect the packet format and weak references do not keep sections alive; equality must include blocks, biomes and light. It is not available when joining a remote server.
- RustChunk relevance: Potentially useful to reduce memory duplication if RustChunk later hosts an integrated server in the same process, but not needed for remote multiplayer or a standalone client.
- Decision: **adaptable; reference; optional/future**
- 1.20.1 implications: Do not use this path for Aternos/remote joins. Consider only after an integrated-server architecture exists and with strict immutable-section/copy-on-write guarantees.
- Follow-up checks: Benchmark saved memory, verify section equality includes light/biomes, test server edit between offer/take, stale weak refs, dimension/min_y offset and concurrent ownership.
- RustChunk implementation mapping: Possible future `platform/integrated/handoff.rs`; not created.
- Evidence required before adoption: Integrated-server proof of concept, Arc ownership stress tests and chunk-section equality fixtures.
- Completion state: COMPLETE — 132 logical lines read to EOF; source SHA and 4681-byte size match the pinned tree; embedded tests inspected, not run.

### 1337. `crates/horizon-client/src/lan/hub.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `aa2939dced8a51670fecdf33f57ee238b39b66de`
- Size: 181745 bytes; 4525 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2800, 2801–3200, 3201–3600, 3601–4000, 4001–4400, 4401–4525 (contiguous to EOF)
- Purpose: Implements the central LAN/dedicated server hub tick: guest lifecycle, packet handling, movement authority, chunk delivery, entity tracking, broadcasts, menus, teleports and server-side world interactions.
- Dependencies / cross-references: LAN connection/Guest, world/chunk storage, movement prediction, entity tracker, scoreboard, recipes, registries, menu and packet codecs, server tick and worker pools.
- Implementation / logic: Coordinates connected guests per server tick, applies inbound actions, updates player/entity state, decides visibility and packet delivery, handles joins/leaves and performs authoritative world/menu operations.
- Findings: This is a large server authority module, not the Android client loop or renderer. It relies on Horizon-specific later-version packet/content structures. Some subsystems (chunk batching and adaptive delivery) are architectural references only.
- RustChunk relevance: Not a direct RustChunk client port. Only isolated algorithms may inform future integrated-server work; the remote client must never simulate Hub authority.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Keep the 1.20.1 client networking/tick pipeline separate from server logic; decode server updates and render authoritative state.
- Follow-up checks: If hosting is ever added, split responsibilities and validate tick ordering, packet backpressure, chunk view, entity tracking, teleport/respawn, menu lifecycle and concurrent guest changes.
- RustChunk implementation mapping: No baseline mapping; future `server/hub.rs` only if product scope expands.
- Evidence required before adoption: 1.20.1 server tick/packet sources, Forge event traces, multi-client integration tests and load profiling.
- Completion state: COMPLETE — 4525 logical lines read in twelve contiguous ranges through EOF; source SHA and 181745-byte size match the pinned tree; embedded tests inspected, not run.

### 1338. `crates/horizon-client/src/lan/hub/chat.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ec506e84e04dc66daff68d8bd6e8871be782cf55`
- Size: 18651 bytes; 513 logical lines
- Coverage: 1–400, 401–513 (contiguous to EOF)
- Purpose: Implements server-side secure chat and command handling: profile-key validation, signed message chains, last-seen acknowledgements, visibility and relaying.
- Dependencies / cross-references: LAN Hub/Guest, chat signing keys, player profiles, message chain state, command parser and play packet codecs.
- Implementation / logic: Validates signed/unsigned guest chat and command arguments, updates last-seen state and relays accepted messages to eligible players.
- Findings: This is server-side message validation/relay, not the client chat UI or receiving/signing state machine. It uses the pinned later protocol and must not be treated as a 1.20.1 chat codec.
- RustChunk relevance: Chat is essential UX, but the client requires target-specific receive, display and signing logic rather than server relay code.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Implement 1.20.1 secure-chat packet parsing, chat type decoration, signature/acknowledgement behavior and Forge compatibility using target-era sources.
- Follow-up checks: Compare 1.20.1 chat session, player profile keys, signed/unsigned commands, last-seen acknowledgements, filtering and disconnect/error handling.
- RustChunk implementation mapping: No baseline mapping; future `server/chat.rs` only for local hosting.
- Evidence required before adoption: 1.20.1 chat protocol source, packet captures, signed-chat fixtures and localization/UI tests.
- Completion state: COMPLETE — 513 logical lines read in contiguous ranges 1–400 and 401–513; source SHA and 18651-byte size match the pinned tree; embedded tests inspected, not run.

### 1339. `crates/horizon-client/src/lan/hub/chat_filter.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f9dc6ca6bf3dbef0dcf86ec7eecfcc761a7c087b`
- Size: 8090 bytes; 247 logical lines
- Coverage: 1–247 (EOF)
- Purpose: Implements server-side chat moderation/filtering for muted users, blocked language and moderation escalation across chat, commands and text-entry channels.
- Dependencies / cross-references: LAN Hub chat flow, server moderation settings, player mute/ban state and server event/log dispatch.
- Implementation / logic: Evaluates submitted text against moderation rules and player state, then allows, suppresses or escalates the corresponding server action.
- Findings: This is server policy enforcement and not a client-side moderation substitute. The rules are Horizon/server-specific and can differ from a target server's policy.
- RustChunk relevance: Not required to connect to a server; the server remains authoritative for moderation.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not impose this filter locally in RustChunk's baseline. Display server-provided filtering/system messages according to the 1.20.1 client protocol.
- Follow-up checks: If hosting is added, threat-model false positives, normalization/Unicode evasion, audit logging, mute/ban enforcement and command/book/anvil coverage.
- RustChunk implementation mapping: No baseline mapping; future server moderation module only.
- Evidence required before adoption: 1.20.1 server policy requirements, adversarial Unicode/text corpus and moderation event tests.
- Completion state: COMPLETE — 247 logical lines read to EOF; source SHA and 8090-byte size match the pinned tree; embedded tests inspected, not run.

### 1340. `crates/horizon-client/src/lan/menu.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9326c9a62a923cc922d0e214e1b924fa86f30405`
- Size: 41131 bytes; 1126 logical lines
- Coverage: 1–400, 401–800, 801–1126 (contiguous to EOF)
- Purpose: Implements the authoritative server-side container click engine: slot rules, carried stacks, click modes, shift-click, swaps, throws, bundles and hidden/native menu slots.
- Dependencies / cross-references: LAN Hub/Guest, container slot abstractions, item stacks and tags, menu types, player inventory and Forge/Horizon menu callbacks.
- Implementation / logic: Applies a guest's click request to server-owned slots, checks menu validity and slot restrictions, handles complex transfer/click modes and returns changed slots for synchronization.
- Findings: This is server-authoritative menu simulation, not the client-side click prediction/input model. It contains later item formats, bundle/native menu behavior and Horizon-specific menu extensions.
- RustChunk relevance: Container UI and local click prediction are essential, but this server implementation is not a drop-in 1.20.1 client subsystem.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Build the client menu state and prediction against 1.20.1 container packets and legacy ItemStack NBT; accept authoritative server corrections and target Forge menu extensions.
- Follow-up checks: Compare 1.20.1 AbstractContainerMenu click modes, slot validity, shift-click ranges, bundle support, carried stack and correction sequencing.
- RustChunk implementation mapping: No baseline mapping; future `server/menu.rs` only.
- Evidence required before adoption: 1.20.1 Java menu code, Forge slot fixtures and packet-driven click prediction/correction tests.
- Completion state: COMPLETE — 1126 logical lines read in three contiguous ranges through EOF; source SHA and 41131-byte size match the pinned tree; embedded tests inspected, not run.

### 1341. `crates/horizon-client/src/lan/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `816cd885f0976184b705a59f4e670f85efb1bb55`
- Size: 40640 bytes; 1028 logical lines
- Coverage: 1–400, 401–800, 801–1028 (contiguous to EOF)
- Purpose: Defines the LAN/dedicated server module, listener/configuration setup, login admission, online-mode identity and per-connection session lifecycle.
- Dependencies / cross-references: Server settings and secrets, network listener, connection workers, profile authentication, play protocol and LAN Hub.
- Implementation / logic: Starts and configures the server-side listener, accepts and gates incoming connections, manages login identity and hands established sessions to the hub.
- Findings: This module starts a server and handles incoming guests; it is not a client connector. Authentication, admission limits and server lifecycle must not be confused with the Android launcher's offline account mode.
- RustChunk relevance: Not needed for joining remote servers. Future local-hosting would be a separate product feature with additional security and lifecycle requirements.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not start or expose a LAN listener in the remote-client baseline. Implement only the 1.20.1 client login/Forge handshake path.
- Follow-up checks: If hosting is considered, verify admission limits under concurrent logins, private/local address exemptions, secret storage, online/offline identity and shutdown cleanup.
- RustChunk implementation mapping: No baseline mapping; separate future `server/lan/mod.rs` only.
- Evidence required before adoption: 1.20.1 login/Forge handshake captures, concurrent admission tests and Android network-security review.
- Completion state: COMPLETE — 1028 logical lines read in three contiguous ranges through EOF; source SHA and 40640-byte size match the pinned tree; embedded tests inspected, not run.

### 1342. `crates/horizon-client/src/lan/playerdata.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f36b7605f7df18911cdf40168b82c7f60e85c88f`
- Size: 31535 bytes; 782 logical lines
- Coverage: 1–400, 401–782 (contiguous to EOF)
- Purpose: Persists and restores the LAN server's player save data, including inventory, attributes, effects, abilities, recipe-book state, vehicle and world/dimension fields.
- Dependencies / cross-references: Server player/entity state, NBT serialization, UUID-based player files, Level.dat owner data, world storage and entity/attachment data.
- Implementation / logic: Loads player NBT from the UUID file with fallback handling, applies saved fields to a new server player and writes current state back to persistent storage.
- Findings: This is server persistence, not a remote client's local profile or launcher account database. The saved field set and NBT semantics are version-specific; modded dimensions/attachments need separate compatibility checks.
- RustChunk relevance: Not required for a remote client. It may be a reference if RustChunk later includes an integrated server or local worlds.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not port server player save handling into client profile storage. Keep account/profile metadata and server-authoritative player NBT separate.
- Follow-up checks: If local hosting is added, compare 1.20.1 playerdata NBT keys, `.dat_old` recovery, dimension IDs, Forge capabilities/attachments, inventory and vehicle persistence.
- RustChunk implementation mapping: No baseline mapping; future integrated-server player save module only.
- Evidence required before adoption: 1.20.1 playerdata fixtures, Forge capability serialization tests and save/restart recovery tests.
- Completion state: COMPLETE — 782 logical lines read in two contiguous ranges through EOF; source SHA and 31535-byte size match the pinned tree; embedded tests inspected, not run.

### 1343. `crates/horizon-client/src/lan/prediction.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5b0c39858f70ae6c4823c3c8a2403b7cf497be63`
- Size: 62143 bytes; 1497 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1497 (contiguous to EOF)
- Purpose: Implements server-side replay of guest movement inputs to compare claimed movement with deterministic player physics and detect/excuse deviations.
- Dependencies / cross-references: PlayerMotion simulation, collision scene, movement attributes/abilities, velocity pushes, item-use slowdown, sprint/jump state, server tick/packet record and LAN Hub.
- Implementation / logic: Records client inputs and server-observed changes, replays movement ticks, tracks offsets/candidate states and accumulates unexplained movement deviations for authoritative correction/moderation.
- Findings: This is server-side movement validation, not the client's local movement predictor. It depends on Horizon physics and modern movement state; replay tests may require prepared local game resources and were inspected only.
- RustChunk relevance: Accurate client movement and prediction are important, but this anti-cheat replay engine is not the client movement loop. Its deterministic replay concepts are a reference for testing RustChunk's own prediction.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Implement 1.20.1 client movement/collision locally and treat server corrections as authoritative. Do not add this server anti-cheat to the Android client.
- Follow-up checks: Compare 1.20.1 movement tick order, sprint modifiers, jump delay, item slowdown, velocity packets, teleport/burst handling and tolerance against a target server.
- RustChunk implementation mapping: Possible future `tests/movement_replay_oracle.rs`; not created.
- Evidence required before adoption: 1.20.1 Java player physics and packet traces, deterministic client/server movement harness and latency/lag fixtures.
- Completion state: COMPLETE — 1497 logical lines read in four contiguous ranges through EOF; source SHA and 62143-byte size match the pinned tree; embedded tests inspected, not run.

### 1344. `crates/horizon-client/src/lan/recipes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cf983e79698bc07c8b939551ef0896ee3d1fb877`
- Size: 17725 bytes; 471 logical lines
- Coverage: 1–400, 401–471 (contiguous to EOF)
- Purpose: Builds server-side recipe data for synchronization, recipe-property item sets, stonecutter displays and optional Horizon native-mod recipe-content payloads.
- Dependencies / cross-references: Recipe JSON/resource packs, tags and item registries, recipe filters, crafting ingredient parser, LAN SyncData, network packet writer and custom payload channel.
- Implementation / logic: Loads/overlays recipe files, filters and normalizes supported ingredients, encodes recipe-property sets and stonecutter outputs, and chunks mod recipe content into bounded payloads.
- Findings: This is the outbound server recipe encoder and includes Horizon-specific native recipe-content synchronization and later slot-display structures. It is not the 1.20.1 client's recipe packet decoder or the Forge recipe model.
- RustChunk relevance: Recipe-book and crafting data are essential client features, but the client needs a separate target-era inbound parser.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Implement 1.20.1 recipe/recipe-book packet handling and Forge serializer compatibility from target-era sources; do not adopt Horizon custom payload or slot-display formats.
- Follow-up checks: Compare 1.20.1 recipe packet structure, ingredient/tag expansion, recipe filters, stonecutter, furnace property sets, unknown mod ingredient handling and payload limits.
- RustChunk implementation mapping: No baseline mapping; future `server/recipe_sync.rs` only for local hosting.
- Evidence required before adoption: 1.20.1 Java/Forge recipe serializers and packet captures, modded ingredient corpus and round-trip fixtures.
- Completion state: COMPLETE — 471 logical lines read in contiguous ranges 1–400 and 401–471; source SHA and 17725-byte size match the pinned tree; embedded tests inspected, not run.

### 1345. `crates/horizon-client/src/lan/scoreboard.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2a9a158758932ef1bcb8ef673c13fb09feadf925`
- Size: 12393 bytes; 337 logical lines
- Coverage: 1–337 (EOF)
- Purpose: Maintains a per-player view of server scoreboard objectives, display slots, scores and teams, then emits the packets needed to update that client's board.
- Dependencies / cross-references: Server scoreboard state, team membership/color/parameters, objective render/number formats, display-slot registry and play packet encoders.
- Implementation / logic: Filters objectives to displayed ones, snapshots scores/teams and diffs two snapshots into ordered team, objective, display-slot, score and reset packets.
- Findings: This is the server-side scoreboard diff/broadcaster, not the client's scoreboard packet receiver/state model. Display-slot IDs, number formats and packet forms are version-specific.
- RustChunk relevance: Scoreboard display is important for multiplayer HUD, but this code is useful only as a server-side reference; RustChunk needs 1.20.1 inbound packet handling.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Build the 1.20.1 client scoreboard/team state from server packets, preserving team leave-before-join ordering and target formatting rules.
- Follow-up checks: Compare 1.20.1 objective/team/score/display packet semantics, team reassignment order, number formats, reset behavior and Forge extensions.
- RustChunk implementation mapping: No baseline mapping; future server scoreboard broadcaster only.
- Evidence required before adoption: 1.20.1 scoreboard Java source, packet captures and ordered team/objective update fixtures.
- Completion state: COMPLETE — 337 logical lines read to EOF; source SHA and 12393-byte size match the pinned tree; embedded tests inspected, not run.

### 1346. `crates/horizon-client/src/lan/tracking.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f39031e256fda5f5f3c955de2c251142881be24b`
- Size: 7728 bytes; 180 logical lines
- Coverage: 1–180 (EOF)
- Purpose: Provides a generated entity-type table of server tracking range, update interval and delta-tracking behavior, with fallback defaults for unknown types.
- Dependencies / cross-references: Generated EntityTypes metadata, LAN entity tracker, entity registry and projectile template mapping.
- Implementation / logic: Looks up each entity type in a sorted constant table and returns its chunk tracking range, update interval and delta flag; unknown thrown projectiles may inherit snowball settings.
- Findings: This is server tracking configuration for a later Minecraft version and includes entity types absent from 1.20.1. It does not describe client render distance or client entity interpolation.
- RustChunk relevance: A reference for how entity tracking policy is kept data-driven; not a client subsystem.
- Decision: **unnecessary; reference; version-specific**
- 1.20.1 implications: Do not use this table for 1.20.1 rendering or culling. Reconstruct server tracking only if hosting is added, from the target entity builders.
- Follow-up checks: Compare all 1.20.1 EntityType.Builder tracking ranges/update intervals and Forge mod entity defaults; validate unknown/projectile fallback.
- RustChunk implementation mapping: No baseline mapping; future server tracker configuration generator only.
- Evidence required before adoption: 1.20.1 decompiled EntityTypes source/mappings and per-entity spawn/update packet traces.
- Completion state: COMPLETE — 180 logical lines read to EOF; source SHA and 7728-byte size match the pinned tree; embedded tests inspected, not run.

### 1347. `crates/horizon-client/src/lan/waypoints.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3e92d820dcafd1a5c6368dffff7e6d8e7a00ee4d`
- Size: 24828 bytes; 690 logical lines
- Coverage: 1–400, 401–690 (contiguous to EOF)
- Purpose: Implements server-side waypoint/locator-bar tracking between players, including block/chunk/azimuth targets, visibility, team color, range and update/untrack packets.
- Dependencies / cross-references: LAN player position and view distance, waypoint transmit/receive attributes, spectator/hiding state, team colors, game rule and TrackedWaypoint packet encoder.
- Implementation / logic: Maintains source/receiver connections each tick, chooses block/chunk/azimuth representation based on visibility and distance, and sends track/update/untrack packets as players move or hide.
- Findings: This is the server manager for a newer locator-bar feature, not client rendering or a 1.20.1 server protocol. It relies on later waypoint attributes and packet types.
- RustChunk relevance: Not needed for baseline 1.20.1 multiplayer. It can be a future optional protocol/UI feature only if both client and server support it.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not expect ordinary 1.20.1 servers to send TrackedWaypoint packets or expose these attributes. Do not implement server waypoint authority in the client.
- Follow-up checks: If considered later, verify privacy/hiding rules, spectators, team colors, view-distance transitions, cross-dimension untracking and packet lifecycle.
- RustChunk implementation mapping: No baseline mapping; optional future locator/waypoint protocol adapter only.
- Evidence required before adoption: Target server/plugin protocol, privacy rules, packet captures and cross-dimension/range transition tests.
- Completion state: COMPLETE — 690 logical lines read in contiguous ranges 1–400 and 401–690; source SHA and 24828-byte size match the pinned tree; embedded tests inspected, not run.

### 1348. `crates/horizon-client/src/landing.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bc0235ecdf722966347c3ecca5379f9f8260409c`
- Size: 22924 bytes; 706 logical lines
- Coverage: 1–400, 401–706 (contiguous to EOF)
- Purpose: Implements player fall/landing behavior and block landing reactions, producing sound, particle, damage, statistic and block-change events through the motion simulation.
- Dependencies / cross-references: MotionWorld/player state, fall-damage and living-fall rules, landing definition registry, movement attributes, collision scene, block push and native client hooks.
- Implementation / logic: Applies safe-fall distance, fall-damage multipliers, landing-specific behavior (hay, honey, slime, powder snow, pointed dripstone, farmland, turtle eggs), event generation and block/entity reactions.
- Findings: This is shared movement/landing simulation with explicit client/server distinctions, but it contains later block families and native hooks. Some block mutation and damage effects are event-produced and must remain server-authoritative in multiplayer.
- RustChunk relevance: Fall physics, landing feedback and collision interactions are core gameplay behavior; the architecture is useful but the implementation needs target-version reconstruction.
- Decision: **essential; rewrite required; version-specific**
- 1.20.1 implications: Rebuild the 1.20.1 fall/landing path, target block set and Forge event hooks. The remote client may predict presentation but must reconcile damage/block changes from the server.
- Follow-up checks: Compare 1.20.1 LivingEntity fall logic, safe-fall attributes, hay/slime/honey/farmland/egg/dripstone behavior, creative/mayfly exemptions, statistics, random draw order and server/client block authority.
- RustChunk implementation mapping: Possible `gameplay/movement/landing_1_20_1.rs` plus event consumer; not created.
- Evidence required before adoption: 1.20.1 Java fall/landing source, Forge living-fall events, block behavior fixtures and deterministic movement/event traces.
- Completion state: COMPLETE — 706 logical lines read in contiguous ranges 1–400 and 401–706; source SHA and 22924-byte size match the pinned tree; embedded tests inspected, not run.

### 1349. `crates/horizon-client/src/locator_bar.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ecead2c52ccd1e45f4ddf66e48233a9bc2c18e11`
- Size: 27216 bytes; 742 logical lines
- Coverage: 1–400, 401–742 (contiguous to EOF)
- Purpose: Implements client-side waypoint state, style/animated sprite loading and rendering of the locator bar with distance-based dots, direction arrows and contextual HUD priority.
- Dependencies / cross-references: TrackedWaypoint play packets, HUD sprite/animation atlas, camera projection/yaw/pitch/FOV, entity lookup, GUI draw pipeline and waypoint-style JSON resources.
- Implementation / logic: Applies track/update/untrack packets, loads waypoint styles and sprite animation metadata, projects targets to screen, sorts waypoints and draws dots/arrows while choosing between locator, XP and jumpable-vehicle bars.
- Findings: This is client-side rendering for a later waypoint protocol and requires matching server packets, attributes and resources. The packet model and built-in locator feature are not ordinary Minecraft 1.20.1 behavior.
- RustChunk relevance: A self-contained HUD/rendering example, but not required to connect to Forge 1.20.1 servers; only reusable UI projection/animation ideas may transfer.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not enable this feature in the 1.20.1 baseline unless a compatible server/mod protocol is explicitly supported. Keep waypoint data optional and server-originated.
- Follow-up checks: If added later, validate packet lifecycle, privacy/untrack behavior, sprite resource reload, GUI scaling/FOV/pitch edge cases, animation timing and accessibility.
- RustChunk implementation mapping: No baseline mapping; optional future `ui/hud/locator_bar.rs` behind explicit compatibility support.
- Evidence required before adoption: Target-compatible waypoint protocol/server extension, resource-pack sprites and screenshot/projection tests across camera angles and GUI scales.
- Completion state: COMPLETE — 742 logical lines read in contiguous ranges 1–400 and 401–742; source SHA and 27216-byte size match the pinned tree; embedded tests inspected, not run.

### 1350. `crates/horizon-client/src/lod_net.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c9cd80ba2e22a031de4a29cd15bc33c66afcfbfa`
- Size: 19954 bytes; 555 logical lines
- Coverage: 1–400, 401–555 (contiguous to EOF)
- Purpose: Defines Horizon's custom far-terrain network channel, including configuration offer, tile requests/replies, server-side distance/rate limits and client-side tile decoding.
- Dependencies / cross-references: Horizon LOD sampler/column data, custom payload protocol, configuration phase, block/biome registries, dimension heights, remote tile inbox/server service and background sampling.
- Implementation / logic: Negotiates protocol version and limits, validates aligned power-of-two tile requests, encodes bounded 16×16 column replies, decodes biome/state/span palettes and rejects oversized or malformed terrain data.
- Findings: This is a proprietary `horizon:lod` custom payload protocol, explicitly not sent to vanilla clients; the offer is disabled by default and dedicated-server-only. The file warns that distant terrain can reveal information about a world's seed. It is not Minecraft 1.20.1 vanilla chunk synchronization.
- RustChunk relevance: Not part of the standard Forge 1.20.1 client baseline. Its bounded payload parsing and request throttling are useful security references only if a separately negotiated far-terrain extension is ever designed.
- Decision: **unnecessary; reference; optional/future**
- 1.20.1 implications: Do not send or expect `horizon:lod` packets when joining ordinary 1.20.1 servers. Use normal chunk packets and client-side distance/LOD processing unless both endpoints explicitly support a compatible extension.
- Follow-up checks: If reconsidered, threat-model seed leakage, verify server opt-in and authorization, enforce in-flight/rate/size/span limits, test trailing-byte handling and validate dimension/state/biome mappings.
- RustChunk implementation mapping: No baseline mapping; possible isolated future `protocol/extensions/horizon_lod.rs` only behind explicit opt-in and server negotiation.
- Evidence required before adoption: Compatible server implementation, protocol/version specification, seed-leakage review, hostile-payload corpus and Android memory/network profiling.
- Completion state: COMPLETE — 555 logical lines read in contiguous ranges 1–400 and 401–555; source SHA and 19954-byte size match the pinned tree; embedded tests inspected, not run.

### 1351. `CLAUDE.md`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `47dc3e3d863cfb5727b87d785d09abf9743c0a72`
- Size: 9 bytes; 1 logical lines
- Coverage: symlink target only
- Purpose: Repository guidance entry; Git tree records this path as a symbolic link whose 9-byte target is `AGENTS.md`.
- Dependencies / cross-references: Git tree mode `120000`; blob content is the path `AGENTS.md`, not an independent instruction document.
- Implementation / logic: The path is a symlink; the contents endpoint followed it and returned the target document, so the target was separately checked through the exact blob.
- Findings: Do not count the followed target text as the symlink's own source. Preserve link semantics and audit `AGENTS.md` at its own path if not already recorded.
- RustChunk relevance: Documentation metadata only; no runtime or Forge implementation.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: No Minecraft 1.20.1 runtime implication.
- Follow-up checks: Check target path resolution and avoid duplicate auditing of the target as this path.
- RustChunk implementation mapping: No code mapping.
- Evidence required before adoption: Exact Git blob, mode `120000`, target bytes and prior audit status of `AGENTS.md`.
- Completion state: COMPLETE — symlink blob verified as 9 bytes / one line; mode and SHA match pinned tree; target-following behavior explicitly accounted for.

### 1352. `crates/horizon-client/src/game/native_components.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `014ad78c1da33aa8ea1dc07d058afaae700d21fa`
- Size: 3145 bytes; 93 logical lines
- Coverage: 1–93 (EOF)
- Purpose: Reads and writes custom data components on native-mod item stacks.
- Dependencies / cross-references: `Game`, `ItemStack`, item component encode/decode helpers and native hook callbacks.
- Implementation / logic: Exposes item component lookup/mutation through the Horizon native-mod API and translates values to the game's stack representation.
- Findings: This is Horizon's private native-mod contract, not a Forge/FML component implementation.
- RustChunk relevance: No direct compatibility: RustChunk must support Java item component semantics through the 1.20.1 protocol/runtime rather than this API.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 component/NBT behavior must follow that version's item stack and packet model.
- Follow-up checks: Compare component serialization against 1.20.1 item NBT and packet fixtures if a future bridge is proposed.
- RustChunk implementation mapping: No baseline mapping; optional isolated native-extension layer only.
- Evidence required before adoption: Cross-version component fixtures, stack round trips and exact 1.20.1 packet evidence.
- Completion state: COMPLETE — all 93 lines read to EOF; SHA and 3145-byte tree size verified; tests inspected, not run.

### 1353. `crates/horizon-client/src/game/native_criteria.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4a683434e4553b1e87a3d3d35d84ff2c3bf77344`
- Size: 8141 bytes; 215 logical lines
- Coverage: 1–215 (EOF)
- Purpose: Implements custom native-mod criteria and advancement/trigger observations.
- Dependencies / cross-references: Native hook declarations, advancement state, game/player lifecycle and criteria identifiers.
- Implementation / logic: Registers and evaluates mod-defined criteria from observed game events, forwarding matching progress to the native-mod callbacks.
- Findings: The API is Horizon-specific; it does not load Java Forge advancement triggers or implement Forge event buses.
- RustChunk relevance: Useful only as a design reference for event-to-criteria dispatch; not a route to Forge mod compatibility.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 advancements use Java-side trigger/serializer registration and versioned packet/state semantics.
- Follow-up checks: Check vanilla trigger coverage, reload behavior and event ordering before implementing any analogous Rust feature.
- RustChunk implementation mapping: No baseline mapping; possible generic event/criteria abstraction only.
- Evidence required before adoption: 1.20.1 advancement trigger definitions, event-order tests and compatibility fixtures.
- Completion state: COMPLETE — all 215 lines read to EOF; SHA and 8141-byte size verified; tests inspected, not run.

### 1354. `crates/horizon-client/src/game/native_data_maps.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b3d4984a4ec740f7bd6f506c9a46c5b3d728ee69`
- Size: 14847 bytes; 408 logical lines
- Coverage: 1–400, 401–408 (contiguous to EOF)
- Purpose: Provides native mods with data-map access and typed lookup helpers.
- Dependencies / cross-references: Game resources, registries, tags/data maps and native hook APIs.
- Implementation / logic: Reads the loaded data-driven maps and exposes resolved values through bounded helpers for native mods.
- Findings: This is an internal Horizon data API; it is not Forge's Java data-map or registry bootstrap.
- RustChunk relevance: Potential reference for typed data lookup and resource reload boundaries, but not reusable as a Forge loader.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 data-driven resources and registries must be sourced from the exact target version and server sync rules.
- Follow-up checks: Verify resource reload invalidation, missing-key behavior and registry identity if adapting concepts.
- RustChunk implementation mapping: No baseline mapping; optional `resources/data_maps` service behind a version adapter.
- Evidence required before adoption: 1.20.1 resource-pack fixtures, registry/tag tests and reload tests.
- Completion state: COMPLETE — all 408 lines read in contiguous ranges to EOF; SHA and 14847-byte size verified; tests inspected, not run.

### 1355. `crates/horizon-client/src/game/native_entities.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cdbb13e3c4202c0445fe42da79d9c613eda1d880`
- Size: 24199 bytes; 605 logical lines
- Coverage: 1–400, 401–605 (contiguous to EOF)
- Purpose: Implements native-mod entity queries and operations over Horizon's simulated world.
- Dependencies / cross-references: Game world, mob/entity storage, entity references, dimensions, collision and native hook callbacks.
- Implementation / logic: Provides entity lookup and controlled entity operations while enforcing world/side and entity-state constraints.
- Findings: Horizon entities are Rust-native simulations; the API does not instantiate or execute Java Forge entity classes.
- RustChunk relevance: Useful as an internal entity-service reference only; cannot substitute for Forge entity registries, Java classes or mixins.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Minecraft 1.20.1 entity types, metadata, spawn packets, attributes and lifecycle are version-specific.
- Follow-up checks: Compare every exposed entity field and operation with 1.20.1 protocol and vanilla behavior before any bridge.
- RustChunk implementation mapping: No baseline mapping; separate Rust entity model only if explicitly scoped, not Forge support.
- Evidence required before adoption: 1.20.1 entity packet captures, spawn/metadata tests and server interoperability.
- Completion state: COMPLETE — all 605 lines read in contiguous ranges to EOF; SHA and 24199-byte size verified; tests inspected, not run.

### 1356. `crates/horizon-client/src/game/native_entity_refs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa227ce08e9f494107c86ed04cf7579b8080b0d0`
- Size: 5243 bytes; 114 logical lines
- Coverage: 1–114 (EOF)
- Purpose: Maintains native-mod entity references/handles and validates their lifetime.
- Dependencies / cross-references: Native entity model, world epochs/lifecycle and entity lookup helpers.
- Implementation / logic: Associates stable handles with entities and checks validity against current world/entity state before use.
- Findings: These are Horizon-local handles, not Minecraft entity IDs or Forge capability references.
- RustChunk relevance: Reference for stale-handle prevention; not a compatible Java entity-reference mechanism.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 network entity IDs and Java object lifetimes remain distinct concepts.
- Follow-up checks: Test stale handles across entity removal, dimension changes and world close if reusing the pattern.
- RustChunk implementation mapping: No baseline mapping; optional internal generational-handle utility.
- Evidence required before adoption: Lifecycle tests for removal/reuse and exact 1.20.1 entity-ID mapping.
- Completion state: COMPLETE — all 114 lines read to EOF; SHA and 5243-byte size verified; tests inspected, not run.

### 1357. `crates/horizon-client/src/game/native_explosions.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d3350bab24c9cb903d5fa2a27f17ef05357c8fd`
- Size: 5310 bytes; 138 logical lines
- Coverage: 1–138 (EOF)
- Purpose: Allows native server mods to create/configure explosions in Horizon's world simulation.
- Dependencies / cross-references: Game world, explosion simulation, block/entity damage and native callbacks.
- Implementation / logic: Validates dimension and inputs, then applies server-authoritative explosion effects through Horizon's own world model.
- Findings: This is a custom server-side extension; it is not Java `Explosion`/Forge explosion event behavior.
- RustChunk relevance: Reference only for input validation and authority checks; no direct Forge compatibility.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Explosion radius, block interaction, damage and packet effects must match 1.20.1 exactly if implemented.
- Follow-up checks: Compare block resistance, entity damage, fire and explosion packet behavior against vanilla 1.20.1.
- RustChunk implementation mapping: No baseline mapping; optional server extension only.
- Evidence required before adoption: 1.20.1 explosion packet fixtures and deterministic world/entity tests.
- Completion state: COMPLETE — all 138 lines read to EOF; SHA and 5310-byte size verified; tests inspected, not run.

### 1358. `crates/horizon-client/src/game/native_falling.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e8e18ef5a392bc059acd9415cc4ad396f7fddac1`
- Size: 4803 bytes; 132 logical lines
- Coverage: 1–132 (EOF)
- Purpose: Spawns and simulates mod-requested falling blocks on the authoritative server world.
- Dependencies / cross-references: Game, falling-block simulation, block-state registry, loaded chunks and world bounds.
- Implementation / logic: Rejects invalid dimensions/states/positions, caps count and velocity, requires loaded chunks, then creates a falling-block entity with placement/drop/damage behavior.
- Findings: This models Horizon's own falling-block entity and server simulation, not Forge's Java `FallingBlockEntity` lifecycle.
- RustChunk relevance: Reference for bounded entity creation and loaded-chunk guards; not a mod loader path.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 falling-block spawn, movement, landing and block update packets must match vanilla.
- Follow-up checks: Test collision, placement failure, entity persistence and damage against 1.20.1.
- RustChunk implementation mapping: No baseline mapping; optional Rust-side falling entity only.
- Evidence required before adoption: Vanilla 1.20.1 falling-block traces and server/client landing tests.
- Completion state: COMPLETE — all 132 lines read to EOF; SHA and 4803-byte size verified; tests inspected, not run.

### 1359. `crates/horizon-client/src/game/native_hooks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a9a06e641684c939559a90e0a424afe25cfe38f8`
- Size: 455449 bytes; 10360 logical lines
- Coverage: 1–10360 in contiguous chunks of at most 400 lines (26 chunks; EOF verified)
- Purpose: Central native-mod hook surface and helper implementation for Horizon game, world, UI, networking, entities and server events.
- Dependencies / cross-references: Game/GameLevel, registries, world simulation, render/HUD systems, LAN server, item/recipe/container helpers, native lifecycle and `native-hooks.json` catalog/fragments.
- Implementation / logic: Declares the hook points and implements extensive helpers for block/state access, items, inventories, recipes, menus, mobs, rendering, input, sound, world data, networking, lifecycle and server interactions; includes contract tests that compare declarations with the hook catalog and module roots.
- Findings: This is Horizon's bespoke Rust-native mod API, not Forge's Java event bus, ModLauncher, Mixin, classloading or JVM ABI. Its broad surface must not be mistaken for Forge compatibility.
- RustChunk relevance: Architectural reference for explicit hook contracts, bounds, validation and tests only; implementing this file does not load or run Forge mods.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Minecraft 1.20.1 Forge compatibility requires the target Java runtime, Forge loader, mappings, mixin/transformer and Java mod dependencies; these hooks cannot replace them.
- Follow-up checks: If ever porting individual behaviors, map each hook to a specific 1.20.1 vanilla/Forge contract and test both client/server authority; do not port the whole API wholesale.
- RustChunk implementation mapping: No baseline mapping; keep separate from any future Java/Forge compatibility runtime.
- Evidence required before adoption: A complete 1.20.1 Forge launch with real Java mods, event/mixin tests, protocol captures and server interoperability—not merely native-hook tests.
- Completion state: COMPLETE — all 10360 lines read in 26 contiguous chunks to EOF; SHA and 455449-byte tree size verified; embedded tests inspected, not run.

### 1360. `crates/horizon-client/src/game/native_inventories.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `429acdc1c7a881d296e606d517f074ee779fdb31`
- Size: 14962 bytes; 401 logical lines
- Coverage: 1–400, 401 (contiguous to EOF)
- Purpose: Reads and moves online/offline players' inventories for native server mods.
- Dependencies / cross-references: Game LAN host/guest state, playerdata NBT files, item stacks, native menu stack-fit logic and save read/write.
- Implementation / logic: Maps 41 player slots and 27 ender slots, reads saved player data, validates expected source stacks, moves whole stacks and writes offline files before mutating live state.
- Findings: Server-side Horizon inventory administration API; offline file editing and live guest access are not a client-side Forge feature.
- RustChunk relevance: Reference for transactional inventory movement and NBT preservation only; not a Forge inventory integration.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 player inventory slot mapping and item components/NBT need exact vanilla compatibility.
- Follow-up checks: Test slot indices, NBT preservation, stale offers, file-write failure and no-duplication invariants.
- RustChunk implementation mapping: No baseline mapping; optional isolated inventory service, not core client behavior.
- Evidence required before adoption: 1.20.1 playerdata fixtures, packet/menu sync tests and crash-safe file-write tests.
- Completion state: COMPLETE — all 401 lines read in contiguous ranges to EOF; SHA and 14962-byte size verified; tests inspected, not run.

### 1361. `crates/horizon-client/src/game/native_item_container.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2d43685285e1558374007ad7773435d712ace57a`
- Size: 13596 bytes; 370 logical lines
- Coverage: 1–370 (EOF)
- Purpose: Implements item-stack containers for native mods using the container component.
- Dependencies / cross-references: ItemStack component data, NBT tags, item registry limits and native item-container callbacks.
- Implementation / logic: Reads container slots, inserts/extracts one item at a time, respects per-slot limits and shulker restrictions, preserves other custom data and clears stale wire data.
- Findings: This is a Horizon custom item capability implemented in Rust; it is not NeoForge `IItemHandler` or Java capability registration.
- RustChunk relevance: Reference for bounded item-container semantics only; no direct Forge capability support.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 container component semantics and shulker restrictions must be checked against the target version.
- Follow-up checks: Compare slot serialization, stack limits, empty-container stacking and item component round trips.
- RustChunk implementation mapping: No baseline mapping; optional `items/container` model only if version-correct.
- Evidence required before adoption: 1.20.1 item component/NBT fixtures and container behavior tests.
- Completion state: COMPLETE — all 370 lines read to EOF; SHA and 13596-byte size verified; tests inspected, not run.

### 1362. `crates/horizon-client/src/game/native_item_energy.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5518915ccbc59e13c8f47ea2f16e0124a7c71cfb`
- Size: 12990 bytes; 341 logical lines
- Coverage: 1–341 (EOF)
- Purpose: Implements a custom energy buffer stored on mod item stacks.
- Dependencies / cross-references: ItemStack, custom-data NBT, native item energy callbacks and item registry.
- Implementation / logic: Validates capacity and per-call limits, stores energy under a Horizon-specific custom-data key, and returns changed one-item stacks for insertion/extraction.
- Findings: The energy API is explicitly Horizon-specific and resembles NeoForge's item energy capability but does not implement Java capability interfaces.
- RustChunk relevance: Reference for bounded arithmetic and stack immutability; does not enable Forge energy capabilities.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 energy APIs are mod/library-defined and cannot be inferred from this custom NBT key.
- Follow-up checks: Check overflow, negative values, serialization and interoperability with the specific Forge energy API in scope.
- RustChunk implementation mapping: No baseline mapping; optional separate energy capability adapter.
- Evidence required before adoption: Exact target mod/library API, NBT fixtures and round-trip tests.
- Completion state: COMPLETE — all 341 lines read to EOF; SHA and 12990-byte size verified; tests inspected, not run.

### 1363. `crates/horizon-client/src/game/native_item_fluids.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4be8b73b845c12d098d283b30872684d24d5d872`
- Size: 13643 bytes; 357 logical lines
- Coverage: 1–357 (EOF)
- Purpose: Implements bucket and custom item fluid tanks for native mods.
- Dependencies / cross-references: ItemStack, custom-data NBT, item fluid callbacks and item registry.
- Implementation / logic: Models water/lava buckets, validates mod tank capacities, fills/drains one-item copies and preserves unrelated custom data.
- Findings: This is a restricted Horizon fluid model, not NeoForge `IFluidHandler` or a complete Forge fluid registry/capability implementation.
- RustChunk relevance: Reference for fill/drain invariants only; no direct Forge fluid compatibility.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 fluid identifiers, bucket behavior and modded fluid registries need versioned Java-side definitions.
- Follow-up checks: Test partial fills, empty/full buckets, mod fluid IDs, serialization and capability negotiation.
- RustChunk implementation mapping: No baseline mapping; optional isolated fluid model after target API selection.
- Evidence required before adoption: 1.20.1 bucket packet traces and concrete Forge fluid API compatibility tests.
- Completion state: COMPLETE — all 357 lines read to EOF; SHA and 13643-byte size verified; tests inspected, not run.

### 1364. `crates/horizon-client/src/game/native_lifecycle.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9428b6092f3ee4d877b606e29b44e0f7fc3ab88c`
- Size: 13103 bytes; 362 logical lines
- Coverage: 1–362 (EOF)
- Purpose: Tracks authoritative world open/close epochs and server tick lifecycle for native mods.
- Dependencies / cross-references: Game execution role, atomic epoch allocation, lifecycle context types and world/server hook callbacks.
- Implementation / logic: Tracks Opening/Ready/Closing states, invalidates failed epochs, pairs non-paused tick start/end and scopes reads to the active tick context.
- Findings: Lifecycle is specific to Horizon's server architecture and native hooks; it does not reproduce Forge lifecycle events or mod construction.
- RustChunk relevance: Reference for explicit lifecycle epochs and read scopes only; not a replacement for Forge mod lifecycle.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 lifecycle phases, event ordering and thread constraints are Java/Forge-specific.
- Follow-up checks: Map any needed lifecycle event to exact Forge 1.20.1 event timing and thread ownership.
- RustChunk implementation mapping: No baseline mapping; optional internal world-session epoch abstraction.
- Evidence required before adoption: Forge 1.20.1 lifecycle event traces, thread assertions and world reload/failure tests.
- Completion state: COMPLETE — all 362 lines read to EOF; SHA and 13103-byte size verified; tests inspected, not run.

### 1365. `crates/horizon-client/src/game/native_menus.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b690387efa0033e2a5236bb762f7aaac56bb5274`
- Size: 19766 bytes; 518 logical lines
- Coverage: 1–400, 401–518 (contiguous to EOF)
- Purpose: Implements server-opened chest-style menus, inventory exchange and player menu lifecycle observations for native mods.
- Dependencies / cross-references: LAN host/guest, menu protocol, player inventories, block/container state and native menu hooks.
- Implementation / logic: Opens/syncs/closes native menus, tracks player container transitions, routes clicks and performs checked all-or-nothing inventory exchanges.
- Findings: Horizon-specific menu implementation and packet assumptions; not Forge `MenuType`/`AbstractContainerMenu` Java integration.
- RustChunk relevance: Reference for transactional trade and menu event ordering only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 menu IDs, slot semantics, click modes and packet sync must be matched exactly.
- Follow-up checks: Verify menu lifecycle, death/close behavior, shift-click and stale inventory offers against 1.20.1.
- RustChunk implementation mapping: No baseline mapping; optional server menu layer after protocol parity.
- Evidence required before adoption: 1.20.1 container packet captures, slot mapping tests and two-player no-duplication tests.
- Completion state: COMPLETE — all 518 lines read in contiguous ranges to EOF; SHA and 19766-byte size verified; tests inspected, not run.

### 1366. `crates/horizon-client/src/game/native_mob_draw.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8d4ce9c2f952a1064ffcfd1285686d1c27be795f`
- Size: 8549 bytes; 230 logical lines
- Coverage: 1–230 (EOF)
- Purpose: Provides native mods with client-side mob/player render state and custom geometry hooks.
- Dependencies / cross-references: Mob animation/interpolation, camera transforms, block-entity draw helpers, vertex buffers and render lighting.
- Implementation / logic: Builds interpolated render snapshots, poses custom geometry relative to mob feet/body yaw and conditionally replaces or layers vanilla geometry when hooks listen.
- Findings: Custom Horizon renderer hooks are not Java Forge renderer events or entity model layers.
- RustChunk relevance: Reference for render-state snapshots and pose math only; no direct Forge renderer support.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 entity renderer/model layer APIs and render-thread contracts differ.
- Follow-up checks: Compare interpolation, yaw conventions, camera-relative transforms and render ordering with 1.20.1.
- RustChunk implementation mapping: No baseline mapping; optional Rust-native rendering extension only.
- Evidence required before adoption: 1.20.1 renderer captures, camera-angle screenshots and model-layer tests.
- Completion state: COMPLETE — all 230 lines read to EOF; SHA and 8549-byte size verified; tests inspected, not run.

### 1367. `crates/horizon-client/src/game/native_mobs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bb562a9e8a1d6ed41833ba5620d8da4a780ef0fb`
- Size: 12853 bytes; 323 logical lines
- Coverage: 1–323 (EOF)
- Purpose: Exposes server-side mob spawn, query, movement, health, equipment and removal operations to native mods.
- Dependencies / cross-references: Game mob simulation, world/chunk bounds, dimensions, LAN guests, entity IDs, attributes and native mob hooks.
- Implementation / logic: Validates authority/dimension/chunk/bounds, creates mobs, exposes snapshots, mutates health/velocity/position, kills/removes and observes equipment changes.
- Findings: This operates Horizon's own mob model; it does not load Java entity classes or Forge entity registrations.
- RustChunk relevance: Reference for authority and validation rules only; not Forge entity compatibility.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 spawn/entity metadata, attributes, equipment and movement packets must be exact.
- Follow-up checks: Test peaceful difficulty, unloaded chunks, entity tracking and player/mob equipment sync.
- RustChunk implementation mapping: No baseline mapping; separate Rust entity simulation only.
- Evidence required before adoption: 1.20.1 entity packet traces, attributes/equipment tests and server join tests.
- Completion state: COMPLETE — all 323 lines read to EOF; SHA and 12853-byte size verified; tests inspected, not run.

### 1368. `crates/horizon-client/src/game/native_motion.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `84f234e7a9434f782cb354804d0ebf00bfebb4d6`
- Size: 3675 bytes; 82 logical lines
- Coverage: 1–82 (EOF)
- Purpose: Adds client player pushes and server-side creative flying-speed control for native mods.
- Dependencies / cross-references: Player velocity, spectator state, LAN host abilities and movement validation.
- Implementation / logic: Clamps push components and flying speed, rejects non-finite values and sends updated abilities to online players.
- Findings: The helper changes Horizon player state and has explicit anti-cheat implications; it is not Forge movement event handling.
- RustChunk relevance: Reference for finite-value clamps only; not a general movement compatibility layer.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 movement speed, ability packets and moved-too-quickly checks are server-version dependent.
- Follow-up checks: Test speed limits, server correction and chunk-loading behavior at high flight speeds.
- RustChunk implementation mapping: No baseline mapping; optional movement extension after vanilla parity.
- Evidence required before adoption: 1.20.1 player ability/movement packet tests and server correction traces.
- Completion state: COMPLETE — all 82 lines read to EOF; SHA and 3675-byte size verified; tests inspected, not run.

### 1369. `crates/horizon-client/src/game/native_mounts.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2877ffc527fa4b2c9ad85d43723cde86e448c96c`
- Size: 3329 bytes; 85 logical lines
- Coverage: 1–85 (EOF)
- Purpose: Lets native server mods allow or cancel player/mob mounting and dismounting.
- Dependencies / cross-references: Game vehicles, mobs, LAN acting-player identity and entity-mount hook callback.
- Implementation / logic: Resolves rider/vehicle identities and types, invokes the mount callback on authoritative server contexts and honors cancellation.
- Findings: Horizon-specific vehicle/mob identity and mount callbacks, not Forge `EntityMountEvent` registration.
- RustChunk relevance: Reference for authority-aware mount cancellation only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 vehicle/passenger packets and mount event order must be validated independently.
- Follow-up checks: Test player/mob/vehicle cases, cancellation and dismount edge cases.
- RustChunk implementation mapping: No baseline mapping; optional mount event abstraction.
- Evidence required before adoption: 1.20.1 passenger packet traces and server/client mount tests.
- Completion state: COMPLETE — all 85 lines read to EOF; SHA and 3329-byte size verified; tests inspected, not run.

### 1370. `crates/horizon-client/src/game/native_net.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa3bef8743adb7348eab3a77637d5b56f67b8b7a`
- Size: 6308 bytes; 165 logical lines
- Coverage: 1–165 (EOF)
- Purpose: Queues and sends Horizon native-mod client/server custom messages.
- Dependencies / cross-references: Horizon protocol mod-net channel, LAN host, player UUIDs, custom payload packets and per-tick counters.
- Implementation / logic: Validates channel names and payload encoding, queues guest/server messages, applies per-player/per-tick limits and flushes messages at tick end.
- Findings: The `horizon:mod_net` channel is proprietary and requires both endpoints to implement it; it is not Forge's SimpleChannel/vanilla payload contract.
- RustChunk relevance: Reference for payload validation and rate limiting only; not compatible with ordinary Forge channels.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 custom payload registration and packet phases differ and must be negotiated with the server.
- Follow-up checks: Threat-model payload sizes, channel authorization, connection phases and abuse limits.
- RustChunk implementation mapping: No baseline mapping; optional isolated extension channel only after explicit negotiation.
- Evidence required before adoption: 1.20.1 custom payload fixtures and server/client compatibility tests.
- Completion state: COMPLETE — all 165 lines read to EOF; SHA and 6308-byte size verified; tests inspected, not run.

### 1371. `crates/horizon-client/src/game/native_panel.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2ed64ef2c0ec7ef5b700d3d4b360c41998122093`
- Size: 23899 bytes; 629 logical lines
- Coverage: 1–400, 401–629 (contiguous to EOF)
- Purpose: Implements a native-mod item/recipe browser panel beside container screens.
- Dependencies / cross-references: HUD batch/rendering, item stacks, screen/input state, native panel event callbacks and recipe pages.
- Implementation / logic: Manages panel ownership/versioning, item conversion, pagination, search, pointer/key events and drawing while preserving the container UI.
- Findings: This is a Horizon-native UI integration, not a Forge client GUI or JEI/REI plugin API.
- RustChunk relevance: Reference for UI ownership, search and input handling only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 screens, rendering and JEI/REI integrations require Java APIs and their own lifecycle.
- Follow-up checks: Check GUI scale, narrow windows, focus, keyboard shortcuts, tooltip layering and disabled mods.
- RustChunk implementation mapping: No baseline mapping; optional Rust-only overlay, not Forge GUI compatibility.
- Evidence required before adoption: 1.20.1 screen/input tests, multiple GUI scales and screenshots on target Android GPU.
- Completion state: COMPLETE — all 629 lines read in contiguous ranges to EOF; SHA and 23899-byte size verified; tests inspected, not run.

### 1372. `crates/horizon-client/src/game/native_panel_recipes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5b70f2e357721f6ce7995b554051394bea0795f1`
- Size: 21642 bytes; 568 logical lines
- Coverage: 1–400, 401–568 (contiguous to EOF)
- Purpose: Lays out and renders native-mod recipe pages inside the Horizon item panel.
- Dependencies / cross-references: Native panel layout, item/recipe stack conversion, HUD sprites, recipe event callbacks and item atlas.
- Implementation / logic: Builds bounded recipe/tab pages, computes grid/cooking/smithing/brewing layouts, cycles stacks, renders controls and dispatches slot/tab/action/page events.
- Findings: This is a custom recipe-viewer UI; it is not Forge recipe registration or JEI/REI integration.
- RustChunk relevance: Reference for recipe presentation and bounded UI data only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 recipe serializers, recipe book displays and client recipe viewer APIs differ.
- Follow-up checks: Validate each recipe layout, missing item behavior, page boundaries and click targets if porting UI concepts.
- RustChunk implementation mapping: No baseline mapping; optional recipe UI separate from loader/runtime.
- Evidence required before adoption: 1.20.1 recipe fixtures and UI tests at multiple scales.
- Completion state: COMPLETE — all 568 lines read in contiguous ranges to EOF; SHA and 21642-byte size verified; tests inspected, not run.

### 1373. `crates/horizon-client/src/game/native_projectiles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `afebd339acba05552580473c02f1a08fcd6772be`
- Size: 11000 bytes; 302 logical lines
- Coverage: 1–302 (EOF)
- Purpose: Routes projectile impacts through native server-mod hooks and replays canceled impacts.
- Dependencies / cross-references: Projectile/arrow simulation, hit targets, mobs/players/carts, wind charges, End dragon/crystals and world collision.
- Implementation / logic: Collects impacts, maps owners/targets, lets hooks cancel them, then re-simulates canceled projectiles and removes associated hit/arrow events consistently.
- Findings: This is Horizon's own projectile simulation and cancellation flow, not Forge projectile events.
- RustChunk relevance: Reference for deferred impact settlement only; no Java Forge compatibility.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 projectile movement, hit ordering, entity/block collision and event cancellation must match vanilla.
- Follow-up checks: Test multiple impacts in one tick, canceled/replayed arrows, player PvP and special entity targets.
- RustChunk implementation mapping: No baseline mapping; optional server simulation behavior only.
- Evidence required before adoption: 1.20.1 projectile packet traces, deterministic collision tests and multiplayer validation.
- Completion state: COMPLETE — all 302 lines read to EOF; SHA and 11000-byte size verified; tests inspected, not run.

### 1374. `crates/horizon-client/src/game/native_quick_move.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `62e9a0f19d24828ac2dbbab7801b8e46cb8dacb1`
- Size: 12808 bytes; 364 logical lines
- Coverage: 1–364 (EOF)
- Purpose: Routes shift-click/quick-move actions in native mod containers through mod callbacks.
- Dependencies / cross-references: LAN menu state, block entities, item stacks, player inventory and container-click packets.
- Implementation / logic: Maps menu/inventory slots, checks menu validity and drag state, calls the mod hook, settles remainder/original stacks and resynchronizes or closes the menu.
- Findings: Horizon-specific custom container routing; not Forge `AbstractContainerMenu.quickMoveStack` execution.
- RustChunk relevance: Reference for no-duplication settlement and slot validation only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 click modes, slot ranges, hidden slots and prediction corrections must match the target protocol.
- Follow-up checks: Test stale menu IDs, dead/spectator players, drag state, block removal and item conservation.
- RustChunk implementation mapping: No baseline mapping; optional container click layer after protocol parity.
- Evidence required before adoption: 1.20.1 container-click packet fixtures and item-conservation property tests.
- Completion state: COMPLETE — all 364 lines read to EOF; SHA and 12808-byte size verified; tests inspected, not run.

### 1375. `crates/horizon-client/src/game/native_recipes.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d8eea22d58ba6aa803bd6b0d295aecdadeb302e5`
- Size: 18308 bytes; 454 logical lines
- Coverage: 1–400, 401–454 (contiguous to EOF)
- Purpose: Builds native-mod recipe/item catalogues, tracks recipe-book displays and synchronizes selected server recipe content.
- Dependencies / cross-references: Crafting/recipe book, item registry/tags, cooking/station/brewing recipes, custom payload and native datapack sync callbacks.
- Implementation / logic: Builds indexed item/recipe tables, supports creative give/place actions, receives bounded recipe-content parts and overlays selected server recipe types onto client recipes.
- Findings: The recipe-content channel and native recipe API are Horizon-specific; they do not register or execute Java Forge recipe serializers.
- RustChunk relevance: Reference for recipe indexing and bounded synchronization only; no Forge mod compatibility.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 recipe JSON serializers, datapack reload and recipe-book packet semantics must be matched exactly.
- Follow-up checks: Test recipe reload, malformed/incomplete content parts, item/tag resolution and recipe-book invalidation.
- RustChunk implementation mapping: No baseline mapping; optional isolated recipe-data service after 1.20.1 parity.
- Evidence required before adoption: 1.20.1 recipe JSON fixtures, server reload tests and packet captures.
- Completion state: COMPLETE — all 454 lines read in contiguous ranges to EOF; SHA and 18308-byte tree size verified; tests inspected, not run.

### 1376. `crates/horizon-client/src/game/native_server.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1b2771fb103e3ddd86255823bb2e06816373bfa8`
- Size: 48368 bytes; 1217 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1217 (EOF)
- Purpose: Server-side native-mod integration surface and game-server operations.
- Dependencies / cross-references: LAN host, server world/game state, player and entity registries, native hook dispatch, persistence and packet/control paths.
- Implementation / logic: Implements the Horizon native server API and its server-scoped operations, validating the active server/world context before exposing actions to native mods.
- Findings: The API is Horizon-specific and does not implement Forge's Java server, FML lifecycle, Java event bus or ModLauncher.
- RustChunk relevance: Reference for authority boundaries, request validation and server/client separation only; it cannot load or run Forge mods.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 server behavior and packets must come from the matching Java/Forge version, not this native server contract.
- Follow-up checks: Map any desired operation to a concrete 1.20.1 server method/event and validate on a real dedicated/integrated server.
- RustChunk implementation mapping: No direct mapping; keep any native server extension isolated from the Forge compatibility runtime.
- Evidence required before adoption: 1.20.1 server API/event contracts, dedicated-server tests, packet traces and a real Forge mod test.
- Completion state: COMPLETE — all 1217 lines read in contiguous chunks to EOF; SHA and 48368-byte size match pinned tree; tests inspected, not run.

### 1377. `crates/horizon-client/src/game/native_shaders.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0b6dda34b908ee93e5ff08f511e23bf3cce4196a`
- Size: 6871 bytes; 158 logical lines
- Coverage: 1–158 (EOF)
- Purpose: Exposes shader-pack draw hooks and native shader-pack lifecycle to Horizon mods.
- Dependencies / cross-references: Horizon renderer, shader-pack selection, GPU pipelines, resource reload and native callbacks.
- Implementation / logic: Builds/updates Horizon shader-pack state and routes render-stage callbacks into the native extension layer.
- Findings: This is Horizon's own shader API; it is not Forge shader-mod integration or a Minecraft 1.20.1 shader loader.
- RustChunk relevance: Optional renderer reference only; not needed for Java Forge mod loading.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Shader pipeline formats, render stages and GPU APIs differ across game versions and Android drivers.
- Follow-up checks: Compare shader stages, resource lifetimes and render-thread requirements before adapting any isolated idea.
- RustChunk implementation mapping: No baseline mapping; optional Rust renderer extension, separate from Forge support.
- Evidence required before adoption: 1.20.1 shader/resource fixtures, GPU validation and Android Mali-G52 tests.
- Completion state: COMPLETE — all 158 lines read to EOF; SHA and 6871-byte size verified; tests inspected, not run.

### 1378. `crates/horizon-client/src/game/native_ticks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `af15c30322de8ac966fe21e1a606f6554324638f`
- Size: 5538 bytes; 169 logical lines
- Coverage: 1–169 (EOF)
- Purpose: Dispatches native-mod tick callbacks for game/server/player/entity scopes.
- Dependencies / cross-references: Game tick loop, server authority, native mod registry and callback queues.
- Implementation / logic: Collects and dispatches tick-stage events with scope and lifecycle checks around the Horizon simulation.
- Findings: Custom Rust-native callbacks are not Forge TickEvent or Java event-bus listeners.
- RustChunk relevance: Reference for explicit tick phases only; does not supply Forge mod event compatibility.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: Forge 1.20.1 tick phases and thread order must be mapped to the Java implementation.
- Follow-up checks: Check paused/world-load behavior, callback order, reentrancy and client/server ownership if porting concepts.
- RustChunk implementation mapping: No baseline mapping; optional event dispatcher independent of Forge.
- Evidence required before adoption: 1.20.1 tick-event traces and client/integrated/dedicated server tests.
- Completion state: COMPLETE — all 169 lines read to EOF; SHA and 5538-byte size verified; tests inspected, not run.

### 1379. `crates/horizon-client/src/game/native_visuals.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5c5c41c795e091bb9ee613ab25b9a581eb0a6b3b`
- Size: 5537 bytes; 151 logical lines
- Coverage: 1–151 (EOF)
- Purpose: Exposes visual-only native-mod effects and presentation hooks.
- Dependencies / cross-references: Client renderer, particles/visual state, entity/world state and native visual callbacks.
- Implementation / logic: Validates and dispatches visual requests into Horizon's client rendering/effect systems.
- Findings: These hooks are tied to Horizon's renderer and native extension ABI, not Forge client events.
- RustChunk relevance: Optional reference for keeping visual effects client-only; not a Forge rendering API.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 Forge render events, particle factories and render-thread rules differ.
- Follow-up checks: Test visual effects under resource reload, world changes and dedicated-server separation.
- RustChunk implementation mapping: No baseline mapping; optional client-only Rust visual layer.
- Evidence required before adoption: 1.20.1 render/particle event mapping and Android GPU validation.
- Completion state: COMPLETE — all 151 lines read to EOF; SHA and 5537-byte size verified; tests inspected, not run.

### 1380. `crates/horizon-client/src/game/native_world_columns.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ad176db211875f0f8c8ef95bff66c62db7626599`
- Size: 3424 bytes; 72 logical lines
- Coverage: 1–72 (EOF)
- Purpose: Provides native mods with world-column queries.
- Dependencies / cross-references: Loaded world/chunks, heightmaps, dimension bounds and native world-read callbacks.
- Implementation / logic: Reads bounded column information from currently available world data and returns it through the native API.
- Findings: Horizon's data access rules are not Forge's `Level`/`ChunkAccess` API.
- RustChunk relevance: Reference only for loaded-chunk and bounds checks; not directly reusable as a Java mod interface.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 heightmaps, world height and chunk statuses need version-specific behavior.
- Follow-up checks: Check unloaded chunks, min/max build height and heightmap variants before adaptation.
- RustChunk implementation mapping: No baseline mapping; optional read-only world service.
- Evidence required before adoption: 1.20.1 chunk/heightmap fixtures and boundary tests.
- Completion state: COMPLETE — all 72 lines read to EOF; SHA and 3424-byte size verified; tests inspected, not run.

### 1381. `crates/horizon-client/src/game/native_world_edit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f649e383f9718f445521b8034a606b116a618b20`
- Size: 3014 bytes; 67 logical lines
- Coverage: 1–67 (EOF)
- Purpose: Exposes controlled world/block edits to native mods.
- Dependencies / cross-references: Server world mutation, block-state registry, chunk bounds and native edit callbacks.
- Implementation / logic: Validates edit requests and applies state changes through Horizon's authoritative world model.
- Findings: This is a private Rust mod-edit API, not Forge's `Level#setBlock` or event/capability contract.
- RustChunk relevance: Reference for mutation validation and authority checks only.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 block update flags, neighbor updates, block entities and packet sync must match vanilla/Forge.
- Follow-up checks: Test edit permissions, unloaded chunks, block entity replacement and neighbor notifications.
- RustChunk implementation mapping: No baseline mapping; optional internal edit service only.
- Evidence required before adoption: 1.20.1 block update packet traces and server-side mutation tests.
- Completion state: COMPLETE — all 67 lines read to EOF; SHA and 3014-byte size verified; tests inspected, not run.

### 1382. `crates/horizon-client/src/game/native_world_read.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e348f5fb79111373b812cd01fc2d4f87fb8c168c`
- Size: 6615 bytes; 189 logical lines
- Coverage: 1–189 (EOF)
- Purpose: Exposes native-mod world/block reads and bounded queries.
- Dependencies / cross-references: World/chunk state, block registry, block entities, dimension and native callbacks.
- Implementation / logic: Reads block/state information from the active world under Horizon's availability and bounds rules.
- Findings: This API is not Forge's Java world access and does not include arbitrary mod block classes.
- RustChunk relevance: Reference for safe read boundaries only; no direct Forge compatibility.
- Decision: unnecessary; reference; optional/future
- 1.20.1 implications: 1.20.1 block states, block entities and chunk lifecycle differ by target mapping and server state.
- Follow-up checks: Verify loaded/unloaded chunk semantics and block-entity NBT if reusing query concepts.
- RustChunk implementation mapping: No baseline mapping; optional Rust-side read API.
- Evidence required before adoption: 1.20.1 world query fixtures and multiplayer authority tests.
- Completion state: COMPLETE — all 189 lines read to EOF; SHA and 6615-byte size verified; tests inspected, not run.

### 1383. `crates/horizon-client/src/game/natural_spawning.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a13ed8620a549f2c385ad6f200c527e6f2cdb5c5`
- Size: 25367 bytes; 679 logical lines
- Coverage: 1–400, 401–679 (contiguous to EOF)
- Purpose: Implements natural mob spawning, spawn placement and spawn-cycle behavior.
- Dependencies / cross-references: Biome and structure data, spawn categories, mob types, difficulty, chunks, collision, random source and world tick.
- Implementation / logic: Selects eligible spawn entries, checks biome/category/cap/placement conditions, searches positions and creates mobs with spawn finalization.
- Findings: Core vanilla-like simulation logic, but tied to this Starling source version and its newer entity/content registry.
- RustChunk relevance: High-value gameplay reference; port only the 1.20.1 subset and preserve vanilla spawn caps/order.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Newer species, biome tags and spawn rules must be removed or adapted to 1.20.1; exact RNG call order affects outcomes.
- Follow-up checks: Diff every species/tag/rule against 1.20.1 data and verify local difficulty, spawn caps, pack spawning and structure rules.
- RustChunk implementation mapping: Map to RustChunk's server/world spawning subsystem with a strict 1.20.1 data adapter.
- Evidence required before adoption: 1.20.1 spawn JSON/tags, seeded RNG comparisons, cap/placement tests and multiplayer spawn traces.
- Completion state: COMPLETE — all 679 lines read in contiguous chunks to EOF; SHA and 25367-byte size verified; tests inspected, not run.

### 1384. `crates/horizon-client/src/game/nautili.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `25bd4afdd96a31ab7e119d1c58484d9be2cacb98`
- Size: 6546 bytes; 158 logical lines
- Coverage: 1–158 (EOF)
- Purpose: Implements the newer Nautilus mob's behavior and interactions.
- Dependencies / cross-references: Nautilus species data, mob AI, water movement, player riding and newer content registry.
- Implementation / logic: Adds Nautilus-specific movement, interaction and mounted behavior to Starling's simulation.
- Findings: Nautilus is newer than Minecraft 1.20.1 and does not exist in the requested target version.
- RustChunk relevance: Exclude from the 1.20.1 baseline; retain only as a future-version reference.
- Decision: unnecessary; version mismatch; future
- 1.20.1 implications: No 1.20.1 entity, registry entry, AI or packet contract exists for this mob.
- Follow-up checks: Do not port unless RustChunk later targets the version that introduces Nautilus.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Versioned vanilla entity registry and target-version decision before adoption.
- Completion state: COMPLETE — all 158 lines read to EOF; SHA and 6546-byte size verified; tests inspected, not run.

### 1385. `crates/horizon-client/src/game/net_harness.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b8efd04e9f379ee22ee6ef1021732a225715f4d5`
- Size: 7065 bytes; 164 logical lines
- Coverage: 1–164 (EOF)
- Purpose: Provides a test harness for exercising networking behavior without a full live game session.
- Dependencies / cross-references: LAN/network packet types, test transport and simulated connection endpoints.
- Implementation / logic: Constructs controlled local test connections and exercises message exchange/error paths for automated tests.
- Findings: This is test infrastructure, not production networking or a Forge loader component.
- RustChunk relevance: Optional test-only reference; do not include in the runtime APK unless a RustChunk test target needs it.
- Decision: **undetermined; optional; test only; no runtime** — optional; test only; no runtime
- 1.20.1 implications: No Minecraft-version feature itself; packet schemas exercised by the harness remain version-specific.
- Follow-up checks: Reuse only after adapting test fixtures to the 1.20.1 protocol and isolating test-only dependencies.
- RustChunk implementation mapping: No production mapping; test harness under RustChunk integration tests.
- Evidence required before adoption: 1.20.1 packet fixtures, deterministic connection tests and proof it is excluded from release runtime.
- Completion state: COMPLETE — all 164 lines read to EOF; SHA and 7065-byte size verified; tests inspected, not run.

### 1386. `crates/horizon-client/src/game/nether_portal.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `97545106a38799906f582d92bf2ee6d2b8655c92`
- Size: 56826 bytes; 1396 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1396 (contiguous to EOF)
- Purpose: Implements Nether portal contact, cooldown, teleport preparation and dimension-transfer behavior for entities.
- Dependencies / cross-references: World dimensions, portal blocks/shapes, entity movement, collision, destination search, chunk loading and saved portal state.
- Implementation / logic: Finds portal surfaces/destinations, tracks entity portal cooldown/process, prepares transfers and applies dimension-aware entity movement while respecting world/chunk state.
- Findings: Core portal behavior is version-sensitive; the source includes newer entity/content assumptions beyond 1.20.1.
- RustChunk relevance: High-value reference for portal state and transfer flow; port with a 1.20.1 behavior/data comparison.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: 1.20.1 portal search radius, POI behavior, destination creation, entity transfer and packet synchronization must be verified.
- Follow-up checks: Compare portal search/fallback, passenger trees, cooldown persistence, Nether/Overworld scaling and End gateway paths against 1.20.1.
- RustChunk implementation mapping: Map to RustChunk dimension-transfer and portal subsystem, not native-mod APIs.
- Evidence required before adoption: 1.20.1 portal fixtures, seeded destination tests, passenger-transfer tests and server packet traces.
- Completion state: COMPLETE — all 1396 lines read in contiguous chunks to EOF; SHA and 56826-byte size verified; tests inspected, not run.

### 1387. `crates/horizon-client/src/game/no_resources.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `12e1cf5b01f88648ba18db9c87ff12d98faa793d`
- Size: 5966 bytes; 170 logical lines
- Coverage: 1–170 (EOF)
- Purpose: Provides fallback behavior when resource assets or registries are unavailable/not yet loaded.
- Dependencies / cross-references: Resource manager readiness, fallback assets, game startup and UI/render initialization.
- Implementation / logic: Defines safe defaults and fallback paths so startup/loading can proceed without dereferencing unavailable resource bundles.
- Findings: Fallback semantics are Starling-specific; it is not a resource-pack format or loader.
- RustChunk relevance: Reference for defensive startup and absent-resource handling; adapt to RustChunk's own asset lifecycle.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: 1.20.1 assets and registry bootstrap must still be complete before world gameplay; fallback must not mask missing required data.
- Follow-up checks: Test missing/corrupt assets, partial initialization and transitions from fallback to loaded resources.
- RustChunk implementation mapping: Map only to RustChunk bootstrap/resource readiness guards.
- Evidence required before adoption: Corrupt/missing 1.20.1 asset fixtures and startup failure tests.
- Completion state: COMPLETE — all 170 lines read to EOF; SHA and 5966-byte size verified; tests inspected, not run.

### 1388. `crates/horizon-client/src/game/ominous_item_spawner.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b74a4729927beea0b191c6ba655248b3c7d524f6`
- Size: 9234 bytes; 235 logical lines
- Coverage: 1–235 (EOF)
- Purpose: Implements the newer ominous item spawner entity and its timed item release behavior.
- Dependencies / cross-references: Ominous item spawner entity state, trial/ominous data, item drops and newer registry entries.
- Implementation / logic: Ticks the spawner's state/timers, validates its contents and emits configured item entities under the newer game rules.
- Findings: The ominous item spawner is absent from Minecraft 1.20.1 and belongs to later trial/ominous content.
- RustChunk relevance: Exclude from 1.20.1 baseline; keep only as a later-version reference.
- Decision: unnecessary; version mismatch; future
- 1.20.1 implications: No 1.20.1 entity type, registry ID or packet representation exists for this feature.
- Follow-up checks: Do not port into 1.20.1; revisit only for a later target version.
- RustChunk implementation mapping: No baseline mapping.
- Evidence required before adoption: Target-version entity registry and later-version data before adoption.
- Completion state: COMPLETE — all 235 lines read to EOF; SHA and 9234-byte size verified; tests inspected, not run.

### 1389. `crates/horizon-client/src/game/options.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04006f0941ebad0f7d455c0a0410b288828967cc`
- Size: 35467 bytes; 786 logical lines
- Coverage: 1–400, 401–786 (contiguous to EOF)
- Purpose: Implements in-game options behavior, settings UI events and persistence coordination.
- Dependencies / cross-references: Game settings, options screens, controls, graphics/audio/language settings, key bindings and platform UI.
- Implementation / logic: Routes option changes, validates values, applies immediate runtime effects and persists settings through the client configuration path.
- Findings: The UI and option set are Starling-specific; only vanilla-equivalent settings map cleanly to 1.20.1.
- RustChunk relevance: Useful client feature reference; port only settings that exist in 1.20.1 and keep Android-specific controls separate.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Option names, ranges, defaults and screen layout differ across 1.20.1 and newer versions.
- Follow-up checks: Build a 1.20.1 option inventory and test save/reload plus runtime application for each setting.
- RustChunk implementation mapping: Map to RustChunk settings model and Android settings screens, with version-gated fields.
- Evidence required before adoption: 1.20.1 defaults/options, config round trips, input and renderer tests on Android.
- Completion state: COMPLETE — all 786 lines read in contiguous chunks to EOF; SHA and 35467-byte size verified; tests inspected, not run.

### 1390. `crates/horizon-client/src/game/own_skin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `730a9384e00cb801598f8fa2ba51dd7b3f00986f`
- Size: 25338 bytes; 618 logical lines
- Coverage: 1–400, 401–618 (contiguous to EOF)
- Purpose: Loads and applies the local player's skin, title-screen player model and skin customization/upload actions.
- Dependencies / cross-references: Account/profile identity, skin files/downloads, texture atlas, renderer, title screen and external skin providers.
- Implementation / logic: Tracks the selected profile's skin, loads model/texture asynchronously, supports title-screen drag preview and account/provider upload actions.
- Findings: Account upload paths assume Starling account integrations; offline profiles cannot use Microsoft upload without credentials.
- RustChunk relevance: Optional feature; local skin loading/preview can be adapted, but account upload must be disabled or provider-gated for offline use.
- Decision: **undetermined; optional; adapt; offline profile** — optional; adapt; offline profile
- 1.20.1 implications: 1.20.1 skin model/texture formats and authentication/session profile APIs must be respected.
- Follow-up checks: Test offline/default skins, profile switching, slim/classic models, atlas reload and unavailable network providers.
- RustChunk implementation mapping: Map local skin selection to RustChunk profile storage; isolate account upload behind an authenticated provider.
- Evidence required before adoption: 1.20.1 skin fixtures, profile identity tests and Android texture-atlas validation.
- Completion state: COMPLETE — all 618 lines read in contiguous chunks to EOF; SHA and 25338-byte size verified; tests inspected, not run.

### 1391. `crates/horizon-client/src/game/packs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dde6c234bdd3a4ab818d4a7f78e766c6cf82c49c`
- Size: 27865 bytes; 697 logical lines
- Coverage: 1–400, 401–697 (contiguous to EOF)
- Purpose: Manages resource-pack discovery/selection and asynchronous resource reload.
- Dependencies / cross-references: Resource pack sources, server/world downloaded packs, resource manager, render resources, textures, language, sounds, mod content and chunk meshing.
- Implementation / logic: Builds layered resource roots, stages CPU-side reload on a worker, applies GPU/resource swaps on render thread and rolls back failed selections.
- Findings: This is a strong resource lifecycle reference but assumes Starling's own resource manager and mod content pipeline.
- RustChunk relevance: High-value client infrastructure; adapt to 1.20.1 pack format and RustChunk resource/renderer architecture.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Pack format versions, server pack acceptance, reload listeners and mod resource integration differ in 1.20.1.
- Follow-up checks: Verify pack priority, server-required packs, failure rollback, resource reload races and Android memory/GPU behavior.
- RustChunk implementation mapping: Map to RustChunk resource-pack manager with separate CPU staging and render-thread application.
- Evidence required before adoption: 1.20.1 pack fixtures, reload failure tests, memory profiling and Android GPU/resource lifecycle tests.
- Completion state: COMPLETE — all 697 lines read in contiguous chunks to EOF; SHA and 27865-byte tree size verified; tests inspected, not run.

### 1392. `crates/horizon-client/src/game/pad_menu.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0faac3f35ae1642d72f596f1ad4596c0fd7b947d`
- Size: 63252 bytes; 1594 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1594 (contiguous to EOF)
- Purpose: Implements controller-driven navigation for menus, inventory, text fields and on-screen keyboard.
- Dependencies / cross-references: Gamepad target registry, screen/widget focus, synthetic mouse events, inventory slots, options pages, OSK and input narration.
- Implementation / logic: Supports focus and pointer modes, D-pad/stick navigation, sliders/tabs, synthetic clicks, scroll/retry, keyboard focus, text entry and cursor drawing.
- Findings: Controller UI is optional and deeply coupled to Starling's screen/widget event system.
- RustChunk relevance: Optional input feature; useful only if RustChunk explicitly adds gamepad navigation.
- Decision: **undetermined; optional; controller feature** — optional; controller feature
- 1.20.1 implications: 1.20.1 screen/widget targets and Android IME behavior differ; source also assumes current Starling UI components.
- Follow-up checks: Test focus across screen transitions, list scrolling, slider changes, keyboard dismissal, touch/mouse handoff and Android navigation.
- RustChunk implementation mapping: No baseline mapping; optional controller UI subsystem.
- Evidence required before adoption: 1.20.1 UI focus fixtures, Android OSK tests and physical-controller tests.
- Completion state: COMPLETE — all 1594 lines read in contiguous chunks to EOF; SHA and 63252-byte size verified; tests inspected, not run.

### 1393. `crates/horizon-client/src/game/pad.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `90cdabbb03f15107b9eee43a6e9b5023f401489b`
- Size: 23197 bytes; 578 logical lines
- Coverage: 1–400, 401–578 (contiguous to EOF)
- Purpose: Polls controller state, maps button/stick/trigger input and dispatches actions into the game.
- Dependencies / cross-references: Hardware/Android pad backends, virtual input commands, touch controls, key mappings, input trace and menu controller.
- Implementation / logic: Merges hardware, injected and touch state; handles button edges/tick-safe releases, action bindings, movement/look, vibration and command injection.
- Findings: Controller support is optional and platform-specific; not required for keyboard/touch gameplay.
- RustChunk relevance: Optional feature; do not prioritize ahead of 1.20.1 core gameplay and Android touch controls.
- Decision: **undetermined; optional; controller feature** — optional; controller feature
- 1.20.1 implications: Gamepad action names and movement/ability behavior need the 1.20.1 control mapping.
- Follow-up checks: Test simultaneous touch/controller sources, short taps, focus loss, remapping and tick-boundary releases.
- RustChunk implementation mapping: Optional RustChunk input backend behind a feature flag.
- Evidence required before adoption: Android controller event tests and 1.20.1 action-binding tests.
- Completion state: COMPLETE — all 578 lines read in contiguous chunks to EOF; SHA and 23197-byte size verified; tests inspected, not run.

### 1394. `crates/horizon-client/src/game/pad/android.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ffb6b32336bfbf8b6216f292cfc680f6afba748c`
- Size: 3171 bytes; 84 logical lines
- Coverage: 1–84 (EOF)
- Purpose: Reads Android controller device state and sends vibration to the active device.
- Dependencies / cross-references: Android activity bridge, controller device/vendor IDs, shared PadState and rumble pulse.
- Implementation / logic: Tracks seen Android device IDs and vendor styles, reads the active controller and sends clamped vibration through the Android bridge.
- Findings: Android-specific Starling glue; not usable outside Android and not needed for touch controls.
- RustChunk relevance: Optional Android gamepad enhancement only.
- Decision: **undetermined; optional; Android controller feature** — optional; Android controller feature
- 1.20.1 implications: Android input APIs and activity bridge must be matched to RustChunk's own JNI/Kotlin layer.
- Follow-up checks: Test device reconnect, multiple controllers, unknown vendors and vibration support/failure.
- RustChunk implementation mapping: Optional RustChunk Android controller bridge.
- Evidence required before adoption: Android API 33 controller events and vibration tests on Samsung hardware.
- Completion state: COMPLETE — all 84 lines read to EOF; SHA and 3171-byte size verified; tests inspected, not run.

### 1395. `crates/horizon-client/src/game/pad/hardware.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bfb8af4b7f571f61c040d3cb804be55f83b275f6`
- Size: 22943 bytes; 559 logical lines
- Coverage: 1–400, 401–559 (contiguous to EOF)
- Purpose: Implements desktop controller discovery/input, gilrs mappings, force feedback and DualSense-specific output.
- Dependencies / cross-references: gilrs, SDL controller mappings, HID DualSense link, rumble/haptics, split-screen router and game input state.
- Implementation / logic: Discovers devices, maps buttons/axes, tracks active pad, routes split-screen events and controls DualSense triggers/lightbar/haptics plus generic rumble.
- Findings: Desktop-specific dependencies and HID output are not part of the Android client path.
- RustChunk relevance: Optional desktop-only feature; exclude from Android baseline and avoid importing desktop HID dependencies.
- Decision: **undetermined; optional; desktop only** — optional; desktop only
- 1.20.1 implications: Not relevant to 1.20.1 Forge compatibility; platform-specific controller behavior only.
- Follow-up checks: Check Cargo feature gating and ensure Android builds never pull desktop gilrs/HID code.
- RustChunk implementation mapping: No Android baseline mapping; keep behind desktop-only feature flags if ever ported.
- Evidence required before adoption: Cross-platform compile matrix and physical controller tests on supported desktop OSes.
- Completion state: COMPLETE — all 559 lines read in contiguous chunks to EOF; SHA and 22943-byte size verified; tests inspected, not run.

### 1396. `crates/horizon-client/src/game/paintings.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `13704e4262983000662c6c58e34aee7d00d942ed`
- Size: 18718 bytes; 496 logical lines
- Coverage: 1–400, 401–496 (contiguous to EOF)
- Purpose: Implements painting variant loading, placement, survival checks, breaking, ticking and rendering.
- Dependencies / cross-references: Painting variant data/tags, collision world, block support, entity bounds, item stacks, renderer atlas and save logic.
- Implementation / logic: Loads variants/placeable tag, selects largest surviving painting, validates wall/support/overlap, handles break/drop and emits front/back/edge geometry.
- Findings: Core vanilla entity behavior; data and asset IDs must be aligned to 1.20.1.
- RustChunk relevance: Essential gameplay behavior to adapt for 1.20.1 paintings.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Check variant registry/tag names, painting NBT, placement randomness and rendering atlas semantics for the target version.
- Follow-up checks: Port variant data, placement/support checks, entity lifecycle and renderer using 1.20.1 assets.
- RustChunk implementation mapping: Map to RustChunk painting entity and entity persistence/rendering subsystems.
- Evidence required before adoption: 1.20.1 painting variant/tag fixtures, wall collision tests, NBT round trips and render screenshots.
- Completion state: COMPLETE — all 496 lines read in contiguous chunks to EOF; SHA and 18718-byte size verified; tests inspected, not run.

### 1397. `crates/horizon-client/src/game/pandas.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e2ab536a262fb752da1ed91849afeb4006210123`
- Size: 7154 bytes; 185 logical lines
- Coverage: 1–185 (EOF)
- Purpose: Implements panda ground-food pickup, bamboo interaction, breeding/age effects and sneeze particles.
- Dependencies / cross-references: Panda AI/state, item tags, dropped-item collection, player interaction, love-mode events and particle engine.
- Implementation / logic: Finds tagged food, picks up and holds items, processes bamboo interactions for babies/adults, records love cause and emits sneeze/bite effects.
- Findings: Panda behavior exists in 1.20.1, but newer source AI/data details may differ.
- RustChunk relevance: Essential mob behavior reference; adapt exact 1.20.1 panda rules and tags.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Verify food tags, age-up/eating timing, mob-griefing behavior, sneeze effects and love-mode event ordering.
- Follow-up checks: Map to RustChunk panda AI/interaction and particles with 1.20.1 data.
- RustChunk implementation mapping: 1.20.1 panda tags, deterministic AI tests and interaction/particle checks.
- Evidence required before adoption: COMPLETE — all 185 lines read to EOF; SHA and 7154-byte size verified; tests inspected, not run.
- Completion state: undefined

### 1398. `crates/horizon-client/src/game/passengers.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a0d97f505d566b645feba58a289098ffc7a74c80`
- Size: 26285 bytes; 626 logical lines
- Coverage: 1–400, 401–626 (contiguous to EOF)
- Purpose: Implements mob/vehicle passenger mounting, seat placement, jockey spawning and skeleton horse traps.
- Dependencies / cross-references: Mob/vehicle models, boats, minecarts, text displays, armor stands, player entity, spawn rules and random source.
- Implementation / logic: Maintains mount links and seat positions, handles multi-level rider trees, spawns natural jockeys and ticks/springs skeleton horse traps.
- Findings: Core behavior is version-sensitive; this source includes newer mount species (camel husk/nautilus) absent in 1.20.1.
- RustChunk relevance: Essential passenger/vehicle behavior to adapt; omit newer species-specific branches.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: 1.20.1 mount types, passenger attachment offsets, spawn odds and skeleton trap rules must be matched exactly.
- Follow-up checks: Port shared passenger graph/seat logic and only 1.20.1 jockey/trap species; test save/load and multi-rider ordering.
- RustChunk implementation mapping: Map to RustChunk entity riding and spawn subsystems.
- Evidence required before adoption: 1.20.1 passenger packet traces, mount graph invariants, seeded spawn tests and NBT round trips.
- Completion state: COMPLETE — all 626 lines read in contiguous chunks to EOF; SHA and 26285-byte size verified; tests inspected, not run.

### 1399. `crates/horizon-client/src/game/perf.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b54f24c650e673ab4feca0da63a06d1a2591ead2`
- Size: 4540 bytes; 116 logical lines
- Coverage: 1–116 (EOF)
- Purpose: Collects performance metrics and draws the stats overlay/perf commands.
- Dependencies / cross-references: Frame graph, system sampler, GPU allocator report, chunk streaming/meshing counters, renderer and overlay UI.
- Implementation / logic: Samples CPU/system and GPU allocation metrics, reports chunks/meshing/draw calls and exposes overlay modes and JSON status.
- Findings: Useful diagnostics, but GPU allocator reporting walks allocations and is throttled to once per second.
- RustChunk relevance: Optional diagnostic feature; not required for compatibility or game correctness.
- Decision: **undetermined; optional; diagnostics** — optional; diagnostics
- 1.20.1 implications: Renderer/backend metric APIs are Starling-specific and Android GPU counters may be unavailable.
- Follow-up checks: Keep sampling low-overhead; test absent GPU reports, thermal/throttling fields and memory cost on Android.
- RustChunk implementation mapping: Optional RustChunk diagnostics module.
- Evidence required before adoption: Benchmark overhead, Android memory/GPU metrics and 1.20.1 chunk pipeline counters.
- Completion state: COMPLETE — all 116 lines read to EOF; SHA and 4540-byte size verified; tests inspected, not run.

### 1400. `crates/horizon-client/src/game/persist.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3adc5719137a377eca50cb45fc39bba7df115b44`
- Size: 78691 bytes; 1879 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1879 (contiguous to EOF)
- Purpose: Loads/saves entity NBT, unknown entities, passenger trees, entity attachments and scheduled block/fluid ticks for chunks.
- Dependencies / cross-references: Chunk extras, entity NBT codecs, UUID/raw-tag memory, world tick scheduler, block entities, portals, mobs, projectiles, vehicles, player RootVehicle and region save writer.
- Implementation / logic: Retains unknown/raw entity data, restores known entity types, handles fresh-add/same-level-move markers, preserves attachments recursively, saves passenger trees and vehicles, and serializes scheduled ticks/moving pistons across chunk load/unload.
- Findings: Persistence is core and highly version-sensitive; entity IDs/types, NBT fields, passenger semantics and tick timing must match 1.20.1.
- RustChunk relevance: Essential infrastructure reference; port only after a complete 1.20.1 NBT/schema map and round-trip tests.
- Decision: essential; adapt; 1.20.1
- 1.20.1 implications: Newer entity types/fields and attachments may not exist in 1.20.1; preserve unknown compounds and avoid dropping modded NBT.
- Follow-up checks: Audit every entity codec, tick save format, raw-field merge, UUID restore, passenger tree and chunk unload path against 1.20.1.
- RustChunk implementation mapping: Map to RustChunk entity persistence and chunk extras, with strict version adapters and lossless unknown-NBT retention.
- Evidence required before adoption: 1.20.1 world/player/entity NBT fixtures, region save/reload tests, crash recovery, passenger-tree tests and scheduled tick round trips.
- Completion state: COMPLETE — all 1879 lines read in five contiguous chunks to EOF; SHA and 78691-byte size verified; tests inspected, not run.
