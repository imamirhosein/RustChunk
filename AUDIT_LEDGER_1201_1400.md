# RustChunk Starling Audit Ledger — Entries 1201–1400

- Source repository/ref: `starling-client/starling` @ `5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1201–1400 (200 new unique source files; one shared Ledger for both halves)
- Current first-half progress: entries 1201–1220 committed (20/100); next checkpoint 1221–1240.
- Stage cadence: five 20-file checkpoints to entry 1300, then pause at 100/200 and wait for a separate user instruction before 1301–1400.
- Persistence rule: validate and commit every exact 20 completed entries.
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

