# RustChunk Starling Audit Index — Entries 1–120

- Source ledger: `AUDIT_LEDGER.md`
- Source repository/ref: `starling-client/starling` / `starling`
- Pinned source snapshot: `5432370d573642041585c69b40fb045b27133488`
- Decisions and one-line reasons are summaries of the corresponding ledger entry. Consult the ledger for detailed evidence, limitations and follow-up checks.

## Part 1 — Entries 1–25

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1 | `README.md` | Adaptable; Rewrite required | high-level Android product design, diagnostics, touch controls, resource acquisition/verification, and build organization; rewrite required for any feature whose implementation assumes Starling/Horizon... |
| 2 | `LICENSE` | Adaptable | a licensing model reference; copying Starling/Horizon code requires preserving notices and checking each dependency's actual license. Minecraft files remain outside this license scope. |
| 3 | `THIRD_PARTY_NOTICES.md` | Essential; Adaptable | a compliance/attribution checklist; adaptable as a model for RustChunk's own third-party notices and on-device fetching policy. Individual libraries/features are not approved for reuse until their code and... |
| 4 | `CONTRIBUTING.md` | Adaptable; Rewrite required | core-vs-Android separation, device testing, PR gates, provenance records and public-tree protection. Rewrite required for toolchain/version policy and project-specific paths. The exact third-party policy is... |
| 5 | `Cargo.toml` | Adaptable; Rewrite required | crate boundaries, Android-specific target isolation, local patch rationale tracking, and quick-vs-release profiles; rewrite required for version-specific game crates and preparation inputs; keep the... |
| 6 | `Cargo.lock` | Adaptable; Unnecessary | a shortlist/reference for specific Android Rust subsystems after reviewing manifests and licenses; unnecessary to copy wholesale. Every dependency must be justified against RustChunk's narrower Rust-native... |
| 7 | `.gitignore` | Adaptable | generated/private-data protection, with paths and ignore breadth rewritten for RustChunk. |
| 8 | `.github/workflows/android.yml` | Adaptable; Rewrite required | matrix builds, preview/release separation, signing discipline, symbols, and release gating; rewrite required for RustChunk package IDs, supported ABIs, secrets, branch policy, toolchain versions, and... |
| 9 | `packaging/android/build_apk.py` | Adaptable; Rewrite required | an APK orchestration pattern; rewrite required for RustChunk's 1.20.1 game-data preparation, engine integration, package identity and supported ABI set. Keep asset/legal boundaries and symbol/signing ideas. |
| 10 | `packaging/lowbudget/make.py` | Unnecessary; Adaptable | the core Rust-native 1.20.1 client; adaptable only if RustChunk needs a pre-game setup theme or generated placeholder UI pack. Rewrite pack schema and resource identifiers for Minecraft 1.20.1. |
| 11 | `research-lock.json` | Essential; Rewrite required | a concept for external-input pinning and deterministic preparation; rewrite required for RustChunk's 1.20.1 source artifacts, preparation tools and supported platforms. |
| 12 | `roadmap/starling.md` | Adaptable; Rewrite required | mobile feature sequencing and clear Done/Left records; rewrite required for the 1.20.1 feature plan and integration assumptions. The roadmap is a planning source, not implementation proof. |
| 13 | `docs/horizon-parity.md` | Essential; Rewrite required | a testing philosophy and differential-harness model; the actual 26.3 test suite is rewrite required for the Rust-native 1.20.1 target. |
| 14 | `docs/android-armv7.md` | Adaptable; Rewrite required | ABI-aware packaging/update selection, low-memory controls, GPU capability checks and resource budgets; rewrite required for RustChunk's 1.20.1 resource preparation and renderer integration. |
| 15 | `docs/android-controls.md` | Adaptable; Rewrite required | input architecture, editable layouts, lifecycle handling and IME focus contracts; rewrite required for Rust-native Minecraft input integration and version-specific UI hooks. |
| 16 | `docs/android-accounts.md` | Essential; Adaptable; Rewrite required | a set of account-security and profile-management requirements; adaptable as architecture, but rewrite required for RustChunk APIs, Rust-native profile/authentication and Android account integration, storage... |
| 17 | `docs/android-diagnostics.md` | Essential; Adaptable; Rewrite required | reliability, observability and privacy requirements; adaptable for data model, retention limits and recovery workflow; rewrite required for RustChunk crash capture and Android integration. |
| 18 | `docs/native-mods.md` | Unnecessary; Adaptable | the first vanilla 1.20.1 milestone; adaptable later for extension API governance, versioned hook contracts, build reproducibility/cache design, code/content separation, opt-in trust and diagnostics... |
| 19 | `docs/starling-native-mods.md` | Adaptable; Unnecessary | remote build delivery, APK-bound cache invalidation, resumable verified downloads, rollback and explicit service trust; unnecessary for the base 1.20.1 client; consider only after a native Rust extension... |
| 20 | `docs/mods.md` | Unnecessary; Adaptable | the base vanilla 1.20.1 milestone; adaptable later for a separate data-only import/porting layer and integrity/compatibility checks. This content importer does not implement the current 1.20.1 game core and... |
| 21 | `docs/version-ports.md` | Adaptable; Rewrite required | the porting methodology (version pinning, provenance/hash verification, generated constants/tables, data diffs, oracle mapping, parity gates and ranked blockers); rewrite/substantial adaptation required for... |
| 22 | `docs/versioning.md` | Unnecessary; Adaptable | the 1.20.1 game-engine port; adaptable as an optional launcher/application release-management pattern. No need to reproduce Horizon's historical bridge tags or its release workflow. |
| 23 | `docs/world-data.md` | Adaptable; Rewrite required | generic NBT/Anvil/palette mechanics and the methodology of DataFixer parity; rewrite/version-gate required for registries, DataVersion, block-state tables, chunk schema and any 1.20.1 world semantics. The... |
| 24 | `docs/native-client.md` | Adaptable; Rewrite required | a Rust-native client architecture, resource-preparation design, scoped Java-oracle methodology and module decomposition; major rewrite/version adaptation required for 1.20.1 semantics and Android ARM64/Mali... |
| 25 | `docs/architecture.md` | Adaptable | an audit/navigation map and modular architecture reference; insufficient by itself to establish 1.20.1 support. No implementation decision should be made from this index without reading the actual... |

## Part 2 — Entries 26–50

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 26 | `docs/how-horizon-is-built.md` | Adaptable | user-owned asset preparation, public-source hygiene, signed release checksums and scoped parity gates; no direct runtime code reuse decision from this policy page. Reconfirm actual license and contribution... |
| 27 | `docs/decompilation-status.md` | Adaptable; Rewrite required | a bytecode research/decompilation workflow and as a warning to validate coverage; rewrite/adapt required for the 1.20.1 mapping and toolchain inputs. Decompilation is an analysis tool, not a route to... |
| 28 | `docs/port-ledger.md` | Adaptable; Rewrite required | a core audit tool/concept (method mapping, verdict map, gap taxonomy and harness reachability); rewrite/version-adapt required for historical 1.20.1 JAR/mappings and its different Java API. The ledger is a... |
| 29 | `docs/parity-workflow.md` | Adaptable; Rewrite required | differential testing, dependency fingerprinting, repeat/flake gates, deterministic scene fixtures and staged integration; rewrite/version-adapt required for a 1.20.1 vanilla oracle, fixtures, baselines,... |
| 30 | `crates/horizon-prepare/Cargo.toml` | Adaptable | a resource-preparer dependency pattern; no direct version-port decision can be made from the manifest alone. |
| 31 | `crates/horizon-prepare/src/check.rs` | Adaptable | a preparation-completeness/stamp/version guard. It is mostly version-agnostic; exact 1.20.1 support depends on the manifest, lock reader, exporters, resources and checks in other modules. |
| 32 | `crates/horizon-prepare/src/main.rs` | Adaptable; Version-specific | the preparation CLI and launch API; version-specific implementation changes required in the lock/artifact resolver, Java selection, exporters, and output resource contracts before it can prepare 1.20.1... |
| 33 | `crates/horizon-prepare/src/java.rs` | Adaptable | Android-supplied Java, JDK discovery, controlled batch execution and timeout handling; version/toolchain policy changes and device-specific memory validation required for 1.20.1. No claim is made that the... |
| 34 | `crates/horizon-prepare/src/artifacts.rs` | Adaptable | major adaptation required for 1.20.1. Reuse the secure path handling, checksum verification, manifest validation, explicit user-provided game files and local-only artifact storage; replace or extend the... |
| 35 | `crates/horizon-prepare/src/lib.rs` | Rewrite required; Version-specific | reuse the pipeline architecture, especially provenance, content hashes, reproducible generation, output stamps, lock protection, local-only source data and explicit staged preparation; substantial... |
| 36 | `crates/horizon-protocol/Cargo.toml` | Adaptable; Rewrite required | at the transport/serialization/cryptography library level; rewrite/introduce a 1.20.1 protocol adapter and reconcile shared core/world types. Protocol 777 must not be reused for 1.20.1. |
| 37 | `crates/horizon-protocol/src/adapter.rs` | Adaptable | a multi-version routing and ID-map pattern; insufficient and unsafe as a 1.20.1 compatibility layer without a complete packet-body/state adapter. |
| 38 | `crates/horizon-protocol/src/adapter/v26_2.rs` | Adaptable | a code-generation/output pattern; not reusable as data for 1.20.1. Generate a fresh 1.20.1 packet table from authoritative version-specific data only after its complete packet schemas and state transitions... |
| 39 | `crates/horizon-protocol/src/packets/handshake.rs` | Adaptable | mostly adaptable for the basic handshake field codec; version-gate protocol constant and intent options for 1.20.1. Do not carry forward protocol 777 or later-only transfer semantics. |
| 40 | `crates/horizon-protocol/src/packets/login.rs` | Rewrite required | reuse low-level codecs and cryptographic primitives only where protocol semantics match; rewrite the Login state packet set and connection transition for 1.20.1. Do not expose current... |
| 41 | `crates/horizon-protocol/src/lib.rs` | Adaptable; Version-specific | the central protocol crate structure, shared state/direction/error model and safe Rust module boundaries; substantial version-specific adaptation required across nearly every exposed module. Keep version... |
| 42 | `crates/horizon-protocol/src/ids.rs` | Adaptable; Version-specific | do not reuse data; regenerate entirely for 1.20.1 using a verified 1.20.1 vanilla report. The generator pattern and `name()` lookup API are adaptable, provided all packet consumers switch atomically to... |
| 43 | `crates/horizon-protocol/src/connection.rs` | Adaptable | largely adaptable as transport and packet framing infrastructure; verify and adapt boundaries for 1.20.1 frame/compression rules, crypto activation and connection-state orchestration. Do not confuse the... |
| 44 | `crates/horizon-protocol/src/buf.rs` | Adaptable; Rewrite required | largely adaptable for primitive codecs and macro composition; method-by-method validation and selective replacement required for special packed/vector/NBT codecs and version-specific structures. |
| 45 | `crates/horizon-protocol/src/frame.rs` | Adaptable; Version-specific | likely reusable with targeted parity verification. Keep framing separate from version-specific packet codecs and add 1.20.1 boundary vectors before trusting interoperability. |
| 46 | `crates/horizon-protocol/src/packets/mod.rs` | Adaptable | typed-packet macro and unknown-packet representation; regenerate/reorganize state-specific enums and IDs for 1.20.1. Preserve strict decode guarantees only where the reference client does so. |
| 47 | `crates/horizon-protocol/src/packets/status.rs` | Adaptable; Version-specific | body codecs largely adaptable; IDs and response metadata must be version-specific for 1.20.1. |
| 48 | `crates/horizon-protocol/src/packets/configuration.rs` | Adaptable | do not reuse this module as a 1.20.1 Configuration state. Extract only genuinely shared packet/data structures and primitive codecs; design 1.20.1 Status/Login/Play packets and move any compatible... |
| 49 | `crates/horizon-protocol/src/packets/play.rs` | Adaptable; Reference | do not port the current Play packet structs as a set. Keep the typed packet design and generic codecs, but build a dedicated 1.20.1 Play packet catalogue with independently sourced IDs, payload schemas,... |
| 50 | `crates/horizon-protocol/src/chunk.rs` | Adaptable; Rewrite required; Integrate | adapt the palette/storage and light primitives, but rewrite the chunk packet envelope, section structure, heightmap format and registry integration for 1.20.1. Keep a dedicated version-specific chunk... |

## Part 3 — Entries 51–75

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 51 | `crates/horizon-protocol/src/registry.rs` | Adaptable | registry name/ID lookup and data-storage architecture; rebuild registry inputs and rewire dynamic registry acquisition for 1.20.1's Login/Play packet flow. Exclude modern dialog/component/default-clock... |
| 52 | `crates/horizon-protocol/src/entity_data.rs` | Rewrite required | reuse the generic metadata-list mechanism and primitive codecs, but replace serializer IDs, value codecs, particle IDs/data, item/profile handling and registry bindings with a 1.20.1-specific... |
| 53 | `crates/horizon-protocol/src/item.rs` | Adaptable | do not adapt current item.rs by editing component IDs. Implement a separate legacy 1.20.1 ItemStack/NBT codec and version-specific inventory synchronization. Reuse the safe Reader/Writer, NBT library,... |
| 54 | `crates/horizon-protocol/src/item/custom.rs` | Unnecessary | the current 1.20.1 vanilla protocol milestone. Defer or omit this feature until a future item extension design exists; reuse only generic NBT validation ideas if required by a separate 1.20.1 feature. |
| 55 | `crates/horizon-protocol/src/packets/recipes.rs` | Rewrite required | rewrite the recipe networking codecs for 1.20.1. Retain only generic codec patterns and possibly recipe UI concepts after translating them into the older recipe model. Treat current recipe display/slot... |
| 56 | `crates/horizon-protocol/src/crypto.rs` | Adaptable | largely adaptable/reusable for 1.20.1 cryptographic primitives and offline UUID derivation. Keep key exchange and login state ordering version-specific; audit platform AES backend separately. |
| 57 | `crates/horizon-protocol/src/rsa.rs` | Adaptable; Version-specific | cryptographic standards and data model are adaptable, but version-specific login/chat wiring and security review required. Prefer vetted constant-time implementations for private operations where feasible;... |
| 58 | `crates/horizon-protocol/src/secure_chat.rs` | Adaptable; Rewrite required; Integrate | adapt selected cryptographic primitives and abstract concepts, but rewrite 1.20.1 signed payload, login/player-info key exchange, chain protocol, last-seen updates and packet integration from target-version... |
| 59 | `crates/horizon-protocol/src/resolver.rs` | Adaptable | largely adaptable/reusable for 1.20.1 and Android. Keep the SRV selection rules and injected DNS provider; verify target-version behavior and ensure the Android host supplies current DNS servers. |
| 60 | `crates/horizon-protocol/src/resolver/dns.rs` | Adaptable | largely reusable/adaptable for 1.20.1, including query, UDP/TCP fallback and platform DNS-source abstraction. Verify parser edge behavior and Android active-network wiring; no protocol-version-specific... |
| 61 | `crates/horizon-protocol/src/attachment_sync.rs` | Unnecessary | the base 1.20.1 milestone; bounded NBT-payload handling is a reusable pattern for a future extension protocol. |
| 62 | `crates/horizon-protocol/src/hash_ops.rs` | Version-specific; Reference | and unsuitable for protocol 1.20.1; retain only as a reference for implementing the newer Codec-hash scheme. |
| 63 | `crates/horizon-protocol/src/mod_list.rs` | Unnecessary | the vanilla 1.20.1 core; file-hash parity is a reusable concept for a future mod-compatibility layer. |
| 64 | `crates/horizon-protocol/src/mod_net.rs` | Unnecessary | the base milestone; the codec design and strict validation are reusable for a future extension protocol. |
| 65 | `crates/horizon-protocol/src/quiet.rs` | Unnecessary | the base milestone; batching and size checks are reusable for a future extension protocol. |
| 66 | `crates/horizon-protocol/src/servers.rs` | Adaptable | after verifying the 1.20.1 schema and filesystem semantics. |
| 67 | `crates/horizon-protocol/src/text.rs` | Adaptable | as a plain-text fallback; the 1.20.1 rich-component parser/renderer must be implemented separately. |
| 68 | `crates/horizon-protocol/src/crypto/blocks.rs` | Adaptable | The CFB8 algorithm is adaptable; the custom ARM AES backend is usable only after security review and safe feature dispatch. |
| 69 | `crates/horizon-protocol/src/item/hash.rs` | Unnecessary; Version-specific | and unusable for 1.20.1; unnecessary for the target protocol. The ItemStack+NBT path must be rewritten. |
| 70 | `crates/horizon-protocol/src/item/hash/exact.rs` | Unnecessary; Version-specific | and unnecessary for the 1.20.1 core; predicate parsing for the target version must be rewritten separately. |
| 71 | `crates/horizon-protocol/src/item/hash/gameplay.rs` | Version-specific | and unusable for 1.20.1; the legacy implementation must be written separately. |
| 72 | `crates/horizon-protocol/src/item/hash/raw.rs` | Version-specific | and unusable for 1.20.1; the golden-vector/oracle-testing method is reusable. |
| 73 | `crates/horizon-protocol/src/item/skip.rs` | Unnecessary; Version-specific | and unnecessary for 1.20.1; the legacy ItemStack decoder must be implemented separately. |
| 74 | `crates/horizon-client/src/chunk_io.rs` | Adaptable | the current code is tied to Horizon's save/toast systems and must be rewritten. |
| 75 | `crates/horizon-client/src/content_identity.rs` | Adaptable; Version-specific | as an idea for future mod-content parity; the current implementation is version-specific and outside the base milestone. |

## Part 4 — Entries 76–100

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 76 | `crates/horizon-client/src/player_sync.rs` | Adaptable | The baseline and delta-handling algorithm is adaptable; packet IDs, thresholds, and the steering extension must be validated or rewritten for 1.20.1. |
| 77 | `crates/horizon-client/src/client_lib.rs` | Adaptable; Reference | A reference/adaptable example for crate organization; this file contains no runtime code to port directly. |
| 78 | `crates/horizon-client/src/heartbeat.rs` | Unnecessary | the base milestone and for Minecraft 1.20.1 compatibility. |
| 79 | `crates/horizon-client/src/reconnect.rs` | Adaptable; Integrate | The backoff logic is adaptable; the classification table and 1.20.1 login/disconnect integration must be rewritten or verified. |
| 80 | `crates/horizon-client/src/server_packs.rs` | Adaptable | The state machine and recovery design are adaptable; status codes, the download/security pipeline, and pack-priority semantics must be verified or rewritten for 1.20.1. |
| 81 | `crates/horizon-client-input/src/lib.rs` | Adaptable; Reference | The crate structure is adaptable; this file itself is only an organizational reference. |
| 82 | `crates/horizon-client-input/src/input.rs` | Adaptable | The vector-normalization approach and multiplier concept are adaptable; the multiplier values and lifecycle logic must be reviewed against 1.20.1. |
| 83 | `crates/horizon-client-input/src/input_mode.rs` | Adaptable | The mode-switching design and modifier reconstruction are adaptable; the Android keycode/source allowlist must be validated on target devices. |
| 84 | `crates/horizon-client-input/src/interaction_input.rs` | Adaptable | Tick buffering and a stable hit result are adaptable patterns; lifecycle behavior and edge cases must be rewritten/validated against 1.20.1 code. |
| 85 | `crates/horizon-client-input/src/key_names.rs` | Adaptable | Equivalent data can be generated/adapted; this 26.3 table must be regenerated for 1.20.1. |
| 86 | `crates/horizon-client-input/src/keybinds.rs` | Adaptable | The KeyBinds abstraction and physical-key-to-action translation are adaptable; MAPPINGS/KEYS and container behavior must be regenerated or adapted for 1.20.1. |
| 87 | `crates/horizon-client-input/src/keymapping.rs` | Adaptable | The base state machine is adaptable; pad toggling and event/screen semantics must be aligned with the 1.20.1 and Android implementations. |
| 88 | `crates/horizon-client-input/src/modifiers.rs` | Adaptable | Modifier reconciliation and paired-key logic are adaptable; snapshot restoration must be synchronous/serialized and tested against the 1.20.1 key lifecycle. |
| 89 | `crates/horizon-client-input/src/mouse.rs` | Adaptable | core mouse delta aggregation, sensitivity curve and scroll accumulation are adaptable; 1.20.1 constants and platform-specific grab semantics require verification. |
| 90 | `crates/horizon-client-input/src/window.rs` | Adaptable | GUI scaling and throttling concepts are adaptable; the FPS limiter needs Android-specific design and measurement; fullscreen/video-mode implementation is primarily desktop-only and must be adapted or omitted. |
| 91 | `crates/horizon-client-input/src/clipboard.rs` | Adaptable | The Clipboard trait and append-selection pattern are adaptable; filter/chat/shortcut rules and the Android bridge must be reviewed against 1.20.1 and the Android platform. |
| 92 | `crates/horizon-client-input/src/gamepad.rs` | Adaptable; Unnecessary; Optional | PadState/Edges/action-map architecture and radial deadzone are adaptable; layout, tuning, navigation, and haptics should be rewritten/ported as optional features, not required parts of the 1.20.1 core. |
| 93 | `crates/horizon-client-input/src/gamepad/android.rs` | Adaptable | The common-state adapter structure is adaptable; Android key/source codes and noise/takeover thresholds require device testing and a complete disconnect lifecycle. |
| 94 | `crates/horizon-client/src/control.rs` | Unnecessary; Optional | the base 1.20.1 client; optional tooling only after a separate security hardening pass. |
| 95 | `crates/horizon-client/src/touch.rs` | Adaptable | The touch-event state machine and ownership/action abstraction are highly adaptable; layout/UI gestures and Bedrock-specific interactions must be rewritten as a separate Android layer and matched to vanilla... |
| 96 | `crates/horizon-client/src/bedrock_touch.rs` | Adaptable; Unnecessary; Optional | the 1.20.1 core; pinned revisions, hash verification, and optional asset fallback are adaptable patterns, but the Bedrock touch-sprite pack itself is not directly portable. |
| 97 | `crates/horizon-client/src/osk.rs` | Adaptable; Unnecessary; Optional | the 1.20.1 gameplay core; an optional gamepad OSK fallback is adaptable, while Android IME should remain the primary text-input path. |
| 98 | `crates/horizon-client/src/android_control_keyboard.rs` | Adaptable; Optional | the Android control editor; touch hit-testing/output design is adaptable, but the keyboard layout is duplicated and Latin-only. Attribution/license compliance is required. |
| 99 | `crates/horizon-client/src/android_control_text.rs` | Adaptable; Rewrite required | The linear helper is adaptable for simple labels; it is not a replacement for a text-layout engine for rich/RTL text or shaping. MIT attribution is required. |
| 100 | `crates/horizon-client/src/net.rs` | Adaptable | Thread/event/writer architecture and the high-level state-machine structure are adaptable; all wire packet IDs/schemas and 26.3 registry/auth extensions must be rewritten for 1.20.1. |

## Part 5 — Entries 101–120

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 101 | `crates/horizon-client-core/Cargo.toml` | Adaptable; Rewrite required; Version-specific | a minimal client-core crate boundary; version-specific module implementations must be validated or rewritten for 1.20.1. |
| 102 | `crates/horizon-client-input/Cargo.toml` | Adaptable; Optional | a platform-input boundary; key tables and backends require target-specific validation, and controller support should remain optional. |
| 103 | `crates/horizon-client-lib/Cargo.toml` | Adaptable | an integration/workspace pattern; major dependency, feature and game-version pruning is required before using it as RustChunk's 1.20.1 client library. |
| 104 | `crates/horizon-client-render/Cargo.toml` | Adaptable; Version-specific | a render/UI crate boundary; version-specific world, mesh, lighting, asset and packet consumers must be ported, and Android backend selection must be verified. |
| 105 | `crates/horizon-client/Cargo.toml` | Adaptable; Rewrite required | the native executable and Android platform boundary; the dependency graph, features and game-facing implementation require substantial 1.20.1-specific pruning and rewrite. |
| 106 | `crates/horizon-world/Cargo.toml` | Adaptable | a dedicated world-data crate; the versioned world formats and resource pipeline must be rebuilt or verified for 1.20.1. |
| 107 | `packaging/android/java/io/github/juanluci/starling/NativeMods.java` | Adaptable; Unnecessary | the vanilla 1.20.1 base; the APK-stamp/startup-rollback pattern is adaptable later, while the current native-mod integration is Horizon-specific. |
| 108 | `packaging/android/native_mod_permissions.py` | Adaptable; Unnecessary | the vanilla 1.20.1 base; the idea of predeclaring native-extension permissions is adaptable, but this scanner is heuristic and Horizon-specific. |
| 109 | `crates/horizon-client-render/src/chunk.rs` | Adaptable; Rewrite required | Highly adaptable as a memory-efficient resident chunk representation; global layout, section indexing, light/heightmap semantics and registry data require validation or rewrite for 1.20.1. |
| 110 | `crates/horizon-world/src/chunk.rs` | Adaptable; Rewrite required | storage algorithms and palette machinery; the existing data version, registry IDs and serialized chunk layout require a 1.20.1-specific rewrite/adapter. |
| 111 | `crates/horizon-world/src/nbt.rs` | Essential; Adaptable | reusable subsystem for a Java client, but version-sensitive edge semantics and custom mixed-list behavior must be separated and validated before use in RustChunk 1.20.1. |
| 112 | `crates/horizon-world/src/region.rs` | Adaptable; Rewrite required | a region-storage implementation; verify 1.20.1 compression/repair semantics and replace or supplement hard-link publication if Android filesystem support is insufficient. |
| 113 | `crates/horizon-world/src/registry.rs` | Essential; Version-specific | concept for the client core, but the current registry data and exporters are version-specific and require a 1.20.1 registry pipeline. |
| 114 | `crates/horizon-client-core/src/lib.rs` | Adaptable; Reference | a module-boundary/reference only; the underlying module implementations must be audited individually for 1.20.1. |
| 115 | `crates/horizon-client-render/src/lib.rs` | Adaptable; Version-specific | a rendering/UI module boundary; prune the broad API and reimplement version-specific modules for 1.20.1. |
| 116 | `crates/horizon-client/examples/probe/main.rs` | Adaptable; Version-specific | a parity-test harness registry; the listed probes and fixture semantics are version-specific and need 1.20.1 replacements. |
| 117 | `crates/horizon-client/src/bin/horizon-ctl/main.rs` | Adaptable; Unnecessary | the vanilla 1.20.1 gameplay core; adaptable later as a separately secured developer/test-control tool after the runtime and command API stabilize. |
| 118 | `crates/horizon-client/src/lib.rs` | Adaptable | a thin platform composition layer; Android unsafe/JNI boundary and all game-specific modules require independent implementation and review. |
| 119 | `crates/horizon-client/src/main.rs` | Adaptable | a minimal executable/bootstrap pattern; the 26.3 mod marker and allocator feature set are Horizon-specific and must be reconsidered. |
| 120 | `crates/horizon-world/examples/probe/main.rs` | Adaptable | a parity harness organization pattern; every child fixture must be replaced or versioned for 1.20.1. |
