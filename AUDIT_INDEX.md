
- Source ledgers: entries 1–400 in [AUDIT_LEDGER_1_400.md](AUDIT_LEDGER_1_400.md); entries 401–600 in [AUDIT_LEDGER_401_600.md](AUDIT_LEDGER_401_600.md); entries 601–800 in [AUDIT_LEDGER_601_800.md](AUDIT_LEDGER_601_800.md); entries 801–1000 in [AUDIT_LEDGER_801_1000.md](AUDIT_LEDGER_801_1000.md); entries 1001–1200 in [AUDIT_LEDGER_1001_1200.md](AUDIT_LEDGER_1001_1200.md) (in progress; durable checkpoint every 20 entries).
- Source repository/ref: `starling-client/starling` / `starling`
- Pinned source snapshot: `5432370d573642041585c69b40fb045b27133488`
- Decisions and one-line reasons summarize each ledger entry's Decision field; consult the ledger for evidence, limitations, implementation mapping and follow-up checks.

## Part 1 — Entries 1–25

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1 | `README.md` | Adaptable; Rewrite required | adaptable for high-level Android product design, diagnostics, touch controls, resource acquisition/verification, and build organization; rewrite required for any feature whose implementation assumes Starling/Horizon... |
| 2 | `LICENSE` | Adaptable; Reference | adaptable as a licensing model reference; copying Starling/Horizon code requires preserving notices and checking each dependency's actual license. Minecraft files remain outside this license scope. |
| 3 | `THIRD_PARTY_NOTICES.md` | Essential; Adaptable | essential as a compliance/attribution checklist; adaptable as a model for RustChunk's own third-party notices and on-device fetching policy. Individual libraries/features are not approved for reuse until their code... |
| 4 | `CONTRIBUTING.md` | Adaptable; Rewrite required | adaptable for core-vs-Android separation, device testing, PR gates, provenance records and public-tree protection. Rewrite required for toolchain/version policy and project-specific paths. The exact third-party... |
| 5 | `Cargo.toml` | Adaptable; Rewrite required; Version-specific | adaptable for crate boundaries, Android-specific target isolation, local patch rationale tracking, and quick-vs-release profiles; rewrite required for version-specific game crates and preparation inputs; keep the... |
| 6 | `Cargo.lock` | Adaptable; Unnecessary; Reference | adaptable as a shortlist/reference for specific Android Rust subsystems after reviewing manifests and licenses; unnecessary to copy wholesale. Every dependency must be justified against RustChunk's narrower... |
| 7 | `.gitignore` | Adaptable | adaptable for generated/private-data protection, with paths and ignore breadth rewritten for RustChunk. |
| 8 | `.github/workflows/android.yml` | Adaptable; Rewrite required | adaptable for matrix builds, preview/release separation, signing discipline, symbols, and release gating; rewrite required for RustChunk package IDs, supported ABIs, secrets, branch policy, toolchain versions, and... |
| 9 | `packaging/android/build_apk.py` | Adaptable; Rewrite required | adaptable as an APK orchestration pattern; rewrite required for RustChunk's 1.20.1 game-data preparation, engine integration, package identity and supported ABI set. Keep asset/legal boundaries and symbol/signing ideas. |
| 10 | `packaging/lowbudget/make.py` | Unnecessary; Adaptable; Rewrite required | Keep low-budget setup packaging out of the core; only adapt it later with a 1.20.1-specific pack schema. |
| 11 | `research-lock.json` | Essential; Rewrite required | essential as a concept for external-input pinning and deterministic preparation; rewrite required for RustChunk's 1.20.1 source artifacts, preparation tools and supported platforms. |
| 12 | `roadmap/starling.md` | Adaptable; Rewrite required | adaptable for mobile feature sequencing and clear Done/Left records; rewrite required for the 1.20.1 feature plan and integration assumptions. The roadmap is a planning source, not implementation proof. |
| 13 | `docs/horizon-parity.md` | Essential; Rewrite required | essential as a testing philosophy and differential-harness model; the actual 26.3 test suite is rewrite required for the Rust-native 1.20.1 target. |
| 14 | `docs/android-armv7.md` | Adaptable; Rewrite required | adaptable for ABI-aware packaging/update selection, low-memory controls, GPU capability checks and resource budgets; rewrite required for RustChunk's 1.20.1 resource preparation and renderer integration. |
| 15 | `docs/android-controls.md` | Adaptable; Rewrite required; Version-specific | adaptable for input architecture, editable layouts, lifecycle handling and IME focus contracts; rewrite required for Rust-native Minecraft input integration and version-specific UI hooks. |
| 16 | `docs/android-accounts.md` | Essential; Adaptable; Rewrite required | essential as a set of account-security and profile-management requirements; adaptable as architecture, but rewrite required for RustChunk APIs, Rust-native profile/authentication and Android account integration,... |
| 17 | `docs/android-diagnostics.md` | Essential; Adaptable; Rewrite required | essential as reliability, observability and privacy requirements; adaptable for data model, retention limits and recovery workflow; rewrite required for RustChunk crash capture and Android integration. |
| 18 | `docs/native-mods.md` | Unnecessary; Adaptable; Optional/future | The native-mod architecture is not the first vanilla milestone; retain only as a future extension-governance reference, not as Forge support. |
| 19 | `docs/starling-native-mods.md` | Adaptable; Unnecessary | adaptable for remote build delivery, APK-bound cache invalidation, resumable verified downloads, rollback and explicit service trust; unnecessary for the base 1.20.1 client; consider only after a native Rust... |
| 20 | `docs/mods.md` | Unnecessary; Adaptable; Optional/future | The importer is not the game core and does not execute Java mods; a future data-only import workflow is a separate optional layer. |
| 21 | `docs/version-ports.md` | Adaptable; Rewrite required | adaptable as the porting methodology (version pinning, provenance/hash verification, generated constants/tables, data diffs, oracle mapping, parity gates and ranked blockers); rewrite/substantial adaptation required... |
| 22 | `docs/versioning.md` | Unnecessary; Adaptable; Optional/future | Version/release management patterns may help the launcher later, but are not part of the game-engine port. |
| 23 | `docs/world-data.md` | Adaptable; Rewrite required | adaptable for generic NBT/Anvil/palette mechanics and the methodology of DataFixer parity; rewrite/version-gate required for registries, DataVersion, block-state tables, chunk schema and any 1.20.1 world semantics.... |
| 24 | `docs/native-client.md` | Adaptable; Rewrite required | adaptable as a Rust-native client architecture, resource-preparation design, scoped Java-oracle methodology and module decomposition; major rewrite/version adaptation required for 1.20.1 semantics and Android... |
| 25 | `docs/architecture.md` | Adaptable; Reference | adaptable as an audit/navigation map and modular architecture reference; insufficient by itself to establish 1.20.1 support. No implementation decision should be made from this index without reading the actual... |

## Part 2 — Entries 26–50

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 26 | `docs/how-horizon-is-built.md` | Adaptable | adaptable for user-owned asset preparation, public-source hygiene, signed release checksums and scoped parity gates; no direct runtime code reuse decision from this policy page. Reconfirm actual license and... |
| 27 | `docs/decompilation-status.md` | Adaptable; Rewrite required | adaptable as a bytecode research/decompilation workflow and as a warning to validate coverage; rewrite/adapt required for the 1.20.1 mapping and toolchain inputs. Decompilation is an analysis tool, not a route to... |
| 28 | `docs/port-ledger.md` | Adaptable; Rewrite required | adaptable as a core audit tool/concept (method mapping, verdict map, gap taxonomy and harness reachability); rewrite/version-adapt required for historical 1.20.1 JAR/mappings and its different Java API. The ledger is... |
| 29 | `docs/parity-workflow.md` | Adaptable; Rewrite required | adaptable for differential testing, dependency fingerprinting, repeat/flake gates, deterministic scene fixtures and staged integration; rewrite/version-adapt required for a 1.20.1 vanilla oracle, fixtures, baselines,... |
| 30 | `crates/horizon-prepare/Cargo.toml` | Adaptable | adaptable as a resource-preparer dependency pattern; no direct version-port decision can be made from the manifest alone. |
| 31 | `crates/horizon-prepare/src/check.rs` | Adaptable | adaptable as a preparation-completeness/stamp/version guard. It is mostly version-agnostic; exact 1.20.1 support depends on the manifest, lock reader, exporters, resources and checks in other modules. |
| 32 | `crates/horizon-prepare/src/main.rs` | Adaptable; Version-specific | adaptable as the preparation CLI and launch API; version-specific implementation changes required in the lock/artifact resolver, Java selection, exporters, and output resource contracts before it can prepare 1.20.1... |
| 33 | `crates/horizon-prepare/src/java.rs` | Adaptable | adaptable for Android-supplied Java, JDK discovery, controlled batch execution and timeout handling; version/toolchain policy changes and device-specific memory validation required for 1.20.1. No claim is made that... |
| 34 | `crates/horizon-prepare/src/artifacts.rs` | Adaptable; Rewrite required; Version-specific | major adaptation required for 1.20.1. Reuse secure path handling, checksum verification, manifest validation, explicit user-provided game files and local-only artifact storage; implement verified version-aware remapping. |
| 35 | `crates/horizon-prepare/src/lib.rs` | Adaptable; Rewrite required; Version-specific | reuse the pipeline architecture, especially provenance, content hashes, reproducible generation, output stamps, lock protection, local-only source data and explicit staged preparation; substantial version-specific... |
| 36 | `crates/horizon-protocol/Cargo.toml` | Adaptable; Rewrite required | adaptable at the transport/serialization/cryptography library level; rewrite/introduce a 1.20.1 protocol adapter and reconcile shared core/world types. Protocol 777 must not be reused for 1.20.1. |
| 37 | `crates/horizon-protocol/src/adapter.rs` | Adaptable | adaptable as a multi-version routing and ID-map pattern; insufficient and unsafe as a 1.20.1 compatibility layer without a complete packet-body/state adapter. |
| 38 | `crates/horizon-protocol/src/adapter/v26_2.rs` | Adaptable; Version-specific | adaptable as a code-generation/output pattern; not reusable as data for 1.20.1. Generate a fresh 1.20.1 packet table from authoritative version-specific data only after its complete packet schemas and state... |
| 39 | `crates/horizon-protocol/src/packets/handshake.rs` | Adaptable | mostly adaptable for the basic handshake field codec; version-gate protocol constant and intent options for 1.20.1. Do not carry forward protocol 777 or later-only transfer semantics. |
| 40 | `crates/horizon-protocol/src/packets/login.rs` | Adaptable; Rewrite required | reuse low-level codecs and cryptographic primitives only where protocol semantics match; rewrite the Login state packet set and connection transition for 1.20.1. Do not expose current cookie/acknowledgement/session... |
| 41 | `crates/horizon-protocol/src/lib.rs` | Adaptable; Version-specific | adaptable as the central protocol crate structure, shared state/direction/error model and safe Rust module boundaries; substantial version-specific adaptation required across nearly every exposed module. Keep version... |
| 42 | `crates/horizon-protocol/src/ids.rs` | Adaptable; Version-specific | do not reuse data; regenerate entirely for 1.20.1 using a verified 1.20.1 vanilla report. The generator pattern and `name()` lookup API are adaptable, provided all packet consumers switch atomically to coherent... |
| 43 | `crates/horizon-protocol/src/connection.rs` | Adaptable | largely adaptable as transport and packet framing infrastructure; verify and adapt boundaries for 1.20.1 frame/compression rules, crypto activation and connection-state orchestration. Do not confuse the in-memory... |
| 44 | `crates/horizon-protocol/src/buf.rs` | Adaptable; Version-specific | largely adaptable for primitive codecs and macro composition; method-by-method validation and selective replacement required for special packed/vector/NBT codecs and version-specific structures. |
| 45 | `crates/horizon-protocol/src/frame.rs` | Adaptable; Version-specific | likely reusable with targeted parity verification. Keep framing separate from version-specific packet codecs and add 1.20.1 boundary vectors before trusting interoperability. |
| 46 | `crates/horizon-protocol/src/packets/mod.rs` | Adaptable; Reference | adaptable as typed-packet macro and unknown-packet representation; regenerate/reorganize state-specific enums and IDs for 1.20.1. Preserve strict decode guarantees only where the reference client does so. |
| 47 | `crates/horizon-protocol/src/packets/status.rs` | Adaptable; Version-specific | body codecs largely adaptable; IDs and response metadata must be version-specific for 1.20.1. |
| 48 | `crates/horizon-protocol/src/packets/configuration.rs` | Adaptable | do not reuse this module as a 1.20.1 Configuration state. Extract only genuinely shared packet/data structures and primitive codecs; design 1.20.1 Status/Login/Play packets and move any compatible configuration-like... |
| 49 | `crates/horizon-protocol/src/packets/play.rs` | Adaptable; Reference | do not port the current Play packet structs as a set. Keep the typed packet design and generic codecs, but build a dedicated 1.20.1 Play packet catalogue with independently sourced IDs, payload schemas, registry... |
| 50 | `crates/horizon-protocol/src/chunk.rs` | Rewrite required; Version-specific | adapt the palette/storage and light primitives, but rewrite the chunk packet envelope, section structure, heightmap format and registry integration for 1.20.1. Keep a dedicated version-specific chunk decoder rather... |

## Part 3 — Entries 51–75

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 51 | `crates/horizon-protocol/src/registry.rs` | Adaptable; Rewrite required | adaptable as registry name/ID lookup and data-storage architecture; rebuild registry inputs and rewire dynamic registry acquisition for 1.20.1's Login/Play packet flow. Exclude modern dialog/component/default-clock... |
| 52 | `crates/horizon-protocol/src/entity_data.rs` | Adaptable | reuse the generic metadata-list mechanism and primitive codecs, but replace serializer IDs, value codecs, particle IDs/data, item/profile handling and registry bindings with a 1.20.1-specific implementation. Do not... |
| 53 | `crates/horizon-protocol/src/item.rs` | Adaptable; Optional/future; Version-specific | do not adapt current item.rs by editing component IDs. Implement a separate legacy 1.20.1 ItemStack/NBT codec and version-specific inventory synchronization. Reuse the safe Reader/Writer, NBT library, nesting... |
| 54 | `crates/horizon-protocol/src/item/custom.rs` | Unnecessary; Adaptable; Optional/future | No 1.20.1 vanilla custom-item protocol feature; only generic NBT-validation concepts might help a future extension. |
| 55 | `crates/horizon-protocol/src/packets/recipes.rs` | Rewrite required | rewrite the recipe networking codecs for 1.20.1. Retain only generic codec patterns and possibly recipe UI concepts after translating them into the older recipe model. Treat current recipe display/slot display IDs as... |
| 56 | `crates/horizon-protocol/src/crypto.rs` | Adaptable; Version-specific | largely adaptable/reusable for 1.20.1 cryptographic primitives and offline UUID derivation. Keep key exchange and login state ordering version-specific; audit platform AES backend separately. |
| 57 | `crates/horizon-protocol/src/rsa.rs` | Adaptable; Version-specific | cryptographic standards and data model are adaptable, but version-specific login/chat wiring and security review required. Prefer vetted constant-time implementations for private operations where feasible; do not... |
| 58 | `crates/horizon-protocol/src/secure_chat.rs` | Rewrite required | adapt selected cryptographic primitives and abstract concepts, but rewrite 1.20.1 signed payload, login/player-info key exchange, chain protocol, last-seen updates and packet integration from target-version vanilla.... |
| 59 | `crates/horizon-protocol/src/resolver.rs` | Adaptable | largely adaptable/reusable for 1.20.1 and Android. Keep the SRV selection rules and injected DNS provider; verify target-version behavior and ensure the Android host supplies current DNS servers. |
| 60 | `crates/horizon-protocol/src/resolver/dns.rs` | Adaptable | Reusable for 1.20.1 query, UDP/TCP fallback and platform DNS-source abstraction; verify parser edge behavior and Android active-network wiring. No protocol-version-specific rewrite is indicated. |
| 61 | `crates/horizon-protocol/src/attachment_sync.rs` | Unnecessary; Adaptable; Optional/future | Attachment sync is not a baseline protocol feature; bounded NBT handling may inform a future extension. |
| 62 | `crates/horizon-protocol/src/hash_ops.rs` | Reference; Version-specific | Version-specific and unsuitable for protocol 1.20.1; retain only as a reference for implementing the newer Codec-hash scheme. |
| 63 | `crates/horizon-protocol/src/mod_list.rs` | Unnecessary; Adaptable; Optional/future | The mod-list packet is not vanilla 1.20.1; file-hash parity may inform a future optional mod-compatibility layer. |
| 64 | `crates/horizon-protocol/src/mod_net.rs` | Unnecessary; Adaptable; Optional/future | Custom mod networking is not in the vanilla baseline; preserve only generic codec-validation ideas for a future extension. |
| 65 | `crates/horizon-protocol/src/quiet.rs` | Unnecessary; Adaptable; Optional/future | The custom quiet/batching packet is not baseline gameplay; bounded batching may inform a future extension. |
| 66 | `crates/horizon-protocol/src/servers.rs` | Adaptable | Adaptable after verifying the 1.20.1 schema and filesystem semantics. |
| 67 | `crates/horizon-protocol/src/text.rs` | Adaptable | Adaptable only as a plain-text fallback; the 1.20.1 rich-component parser/renderer must be implemented separately. |
| 68 | `crates/horizon-protocol/src/crypto/blocks.rs` | Adaptable | The CFB8 algorithm is adaptable; the custom ARM AES backend is usable only after security review and safe feature dispatch. |
| 69 | `crates/horizon-protocol/src/item/hash.rs` | Unnecessary; Version-specific | Version-specific and unusable for 1.20.1; unnecessary for the target protocol. The ItemStack+NBT path must be rewritten. |
| 70 | `crates/horizon-protocol/src/item/hash/exact.rs` | Unnecessary; Version-specific | Version-specific and unnecessary for the 1.20.1 core; predicate parsing for the target version must be rewritten separately. |
| 71 | `crates/horizon-protocol/src/item/hash/gameplay.rs` | Version-specific | Version-specific and unusable for 1.20.1; the legacy implementation must be written separately. |
| 72 | `crates/horizon-protocol/src/item/hash/raw.rs` | Adaptable; Version-specific | Version-specific and unusable for 1.20.1; the golden-vector/oracle-testing method is reusable. |
| 73 | `crates/horizon-protocol/src/item/skip.rs` | Unnecessary; Version-specific | Version-specific and unnecessary for 1.20.1; the legacy ItemStack decoder must be implemented separately. |
| 74 | `crates/horizon-client/src/chunk_io.rs` | Adaptable | Adaptable; the current code is tied to Horizon's save/toast systems and must be rewritten. |
| 75 | `crates/horizon-client/src/content_identity.rs` | Adaptable; Optional/future; Version-specific | Adaptable only as an idea for future mod-content parity; the current implementation is version-specific and outside the base milestone. |

## Part 4 — Entries 76–100

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 76 | `crates/horizon-client/src/player_sync.rs` | Adaptable | The baseline and delta-handling algorithm is adaptable; packet IDs, thresholds, and the steering extension must be validated or rewritten for 1.20.1. |
| 77 | `crates/horizon-client/src/client_lib.rs` | Adaptable; Reference | A reference/adaptable example for crate organization; this file contains no runtime code to port directly. |
| 78 | `crates/horizon-client/src/heartbeat.rs` | Unnecessary | Unnecessary for the base milestone and for Minecraft 1.20.1 compatibility. |
| 79 | `crates/horizon-client/src/reconnect.rs` | Adaptable | The backoff logic is adaptable; the classification table and 1.20.1 login/disconnect integration must be rewritten or verified. |
| 80 | `crates/horizon-client/src/server_packs.rs` | Adaptable | The state machine and recovery design are adaptable; status codes, the download/security pipeline, and pack-priority semantics must be verified or rewritten for 1.20.1. |
| 81 | `crates/horizon-client-input/src/lib.rs` | Adaptable; Reference | The crate structure is adaptable; this file itself is only an organizational reference. |
| 82 | `crates/horizon-client-input/src/input.rs` | Adaptable | The vector-normalization approach and multiplier concept are adaptable; the multiplier values and lifecycle logic must be reviewed against 1.20.1. |
| 83 | `crates/horizon-client-input/src/input_mode.rs` | Adaptable | The mode-switching design and modifier reconstruction are adaptable; the Android keycode/source allowlist must be validated on target devices. |
| 84 | `crates/horizon-client-input/src/interaction_input.rs` | Adaptable | Tick buffering and a stable hit result are adaptable patterns; lifecycle behavior and edge cases must be rewritten/validated against 1.20.1 code. |
| 85 | `crates/horizon-client-input/src/key_names.rs` | Adaptable; Version-specific | Generate equivalent key-name data from 1.20.1; do not copy the 26.3 table. |
| 86 | `crates/horizon-client-input/src/keybinds.rs` | Adaptable | The KeyBinds abstraction and physical-key-to-action translation are adaptable; MAPPINGS/KEYS and container behavior must be regenerated or adapted for 1.20.1. |
| 87 | `crates/horizon-client-input/src/keymapping.rs` | Adaptable | The base state machine is adaptable; pad toggling and event/screen semantics must be aligned with the 1.20.1 and Android implementations. |
| 88 | `crates/horizon-client-input/src/modifiers.rs` | Adaptable | Modifier reconciliation and paired-key logic are adaptable; snapshot restoration must be synchronous/serialized and tested against the 1.20.1 key lifecycle. |
| 89 | `crates/horizon-client-input/src/mouse.rs` | Adaptable | core mouse delta aggregation, sensitivity curve and scroll accumulation are adaptable; 1.20.1 constants and platform-specific grab semantics require verification. |
| 90 | `crates/horizon-client-input/src/window.rs` | Adaptable | GUI scaling and throttling concepts are adaptable; the FPS limiter needs Android-specific design and measurement; fullscreen/video-mode implementation is primarily desktop-only and must be adapted or omitted. |
| 91 | `crates/horizon-client-input/src/clipboard.rs` | Adaptable | The Clipboard trait and append-selection pattern are adaptable; filter/chat/shortcut rules and the Android bridge must be reviewed against 1.20.1 and the Android platform. |
| 92 | `crates/horizon-client-input/src/gamepad.rs` | Adaptable; Unnecessary; Optional/future | PadState/Edges/action-map architecture and radial deadzone are adaptable; layout, tuning, navigation, and haptics should be rewritten/ported as optional features, not required parts of the 1.20.1 core. |
| 93 | `crates/horizon-client-input/src/gamepad/android.rs` | Adaptable | The common-state adapter structure is adaptable; Android key/source codes and noise/takeover thresholds require device testing and a complete disconnect lifecycle. |
| 94 | `crates/horizon-client/src/control.rs` | Unnecessary; Optional/future | unnecessary for the base 1.20.1 client; optional tooling only after a separate security hardening pass. |
| 95 | `crates/horizon-client/src/touch.rs` | Adaptable; Rewrite required | Touch state-machine ownership/action patterns are reusable; rewrite layout/UI gestures and Bedrock-specific interactions as a separate Android layer for vanilla 1.20.1. |
| 96 | `crates/horizon-client/src/bedrock_touch.rs` | Adaptable; Unnecessary; Optional/future | Unnecessary for the 1.20.1 core; pinned revisions, hash verification, and optional asset fallback are adaptable patterns, but the Bedrock touch-sprite pack itself is not directly portable. |
| 97 | `crates/horizon-client/src/osk.rs` | Adaptable; Unnecessary; Optional/future | Unnecessary for the 1.20.1 gameplay core; an optional gamepad OSK fallback is adaptable, while Android IME should remain the primary text-input path. |
| 98 | `crates/horizon-client/src/android_control_keyboard.rs` | Adaptable; Optional/future | Optional for the Android control editor; touch hit-testing/output design is adaptable, but the keyboard layout is duplicated and Latin-only. Attribution/license compliance is required. |
| 99 | `crates/horizon-client/src/android_control_text.rs` | Adaptable | The linear helper is adaptable for simple labels; it is not a replacement for a text-layout engine for rich/RTL text or shaping. MIT attribution is required. |
| 100 | `crates/horizon-client/src/net.rs` | Adaptable | Thread/event/writer architecture and the high-level state-machine structure are adaptable; all wire packet IDs/schemas and 26.3 registry/auth extensions must be rewritten for 1.20.1. |

## Part 5 — Entries 101–125

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 101 | `crates/horizon-client-core/Cargo.toml` | Adaptable; Version-specific | Adaptable as a minimal client-core crate boundary; version-specific module implementations must be validated or rewritten for 1.20.1. |
| 102 | `crates/horizon-client-input/Cargo.toml` | Adaptable; Optional/future | Adaptable as a platform-input boundary; key tables and backends require target-specific validation, and controller support should remain optional. |
| 103 | `crates/horizon-client-lib/Cargo.toml` | Adaptable | Adaptable as an integration/workspace pattern; major dependency, feature and game-version pruning is required before using it as RustChunk's 1.20.1 client library. |
| 104 | `crates/horizon-client-render/Cargo.toml` | Adaptable; Version-specific | Adaptable as a render/UI crate boundary; version-specific world, mesh, lighting, asset and packet consumers must be ported, and Android backend selection must be verified. |
| 105 | `crates/horizon-client/Cargo.toml` | Adaptable; Rewrite required | Adaptable as the native executable and Android platform boundary; the dependency graph, features and game-facing implementation require substantial 1.20.1-specific pruning and rewrite. |
| 106 | `crates/horizon-world/Cargo.toml` | Adaptable | Adaptable as a dedicated world-data crate; the versioned world formats and resource pipeline must be rebuilt or verified for 1.20.1. |
| 107 | `packaging/android/java/io/github/juanluci/starling/NativeMods.java` | Adaptable; Unnecessary | Unnecessary for the vanilla 1.20.1 base; the APK-stamp/startup-rollback pattern is adaptable later, while the current native-mod integration is Horizon-specific. |
| 108 | `packaging/android/native_mod_permissions.py` | Adaptable; Unnecessary | Unnecessary for the vanilla 1.20.1 base; the idea of predeclaring native-extension permissions is adaptable, but this scanner is heuristic and Horizon-specific. |
| 109 | `crates/horizon-client-render/src/chunk.rs` | Adaptable; Rewrite required | Highly adaptable as a memory-efficient resident chunk representation; global layout, section indexing, light/heightmap semantics and registry data require validation or rewrite for 1.20.1. |
| 110 | `crates/horizon-world/src/chunk.rs` | Adaptable; Rewrite required | Adaptable as storage algorithms and palette machinery; the existing data version, registry IDs and serialized chunk layout require a 1.20.1-specific rewrite/adapter. |
| 111 | `crates/horizon-world/src/nbt.rs` | Essential; Adaptable | Essential reusable subsystem for a Java client, but version-sensitive edge semantics and custom mixed-list behavior must be separated and validated before use in RustChunk 1.20.1. |
| 112 | `crates/horizon-world/src/region.rs` | Adaptable | Adaptable as a region-storage implementation; verify 1.20.1 compression/repair semantics and replace or supplement hard-link publication if Android filesystem support is insufficient. |
| 113 | `crates/horizon-world/src/registry.rs` | Essential; Version-specific | Essential concept for the client core, but the current registry data and exporters are version-specific and require a 1.20.1 registry pipeline. |
| 114 | `crates/horizon-client-core/src/lib.rs` | Adaptable; Reference | Adaptable as a module-boundary/reference only; the underlying module implementations must be audited individually for 1.20.1. |
| 115 | `crates/horizon-client-render/src/lib.rs` | Adaptable; Rewrite required; Version-specific | Adaptable as a rendering/UI module boundary; prune the broad API and reimplement version-specific modules for 1.20.1. |
| 116 | `crates/horizon-client/examples/probe/main.rs` | Adaptable; Version-specific | Adaptable as a parity-test harness registry; the listed probes and fixture semantics are version-specific and need 1.20.1 replacements. |
| 117 | `crates/horizon-client/src/bin/horizon-ctl/main.rs` | Adaptable; Unnecessary | Unnecessary for the vanilla 1.20.1 gameplay core; adaptable later as a separately secured developer/test-control tool after the runtime and command API stabilize. |
| 118 | `crates/horizon-client/src/lib.rs` | Adaptable | Adaptable as a thin platform composition layer; Android unsafe/JNI boundary and all game-specific modules require independent implementation and review. |
| 119 | `crates/horizon-client/src/main.rs` | Adaptable | Adaptable as a minimal executable/bootstrap pattern; the 26.3 mod marker and allocator feature set are Horizon-specific and must be reconsidered. |
| 120 | `crates/horizon-world/examples/probe/main.rs` | Adaptable | Adaptable as a parity harness organization pattern; every child fixture must be replaced or versioned for 1.20.1. |
| 121 | `crates/horizon-client/src/entry.rs` | Adaptable | Adaptable as startup architecture; mod integration and server branches require redesign or omission for vanilla 1.20.1. |
| 122 | `crates/horizon-client/src/android.rs` | Adaptable; Reference | Highly relevant as an Android architecture reference; implement a smaller RustChunk bridge with explicit unsafe/JNI review and device validation. |
| 123 | `crates/horizon-client/src/diagnostics.rs` | Adaptable | Adaptable diagnostics architecture; redaction, durability and Android exit collection need security/privacy review. |
| 124 | `crates/horizon-client-render/src/chunk_map.rs` | Adaptable | Adaptable as resident chunk membership bookkeeping; revision lifetime, reset semantics and ordering assumptions need tests. |
| 125 | `crates/horizon-client-render/src/mesher.rs` | Essential; Rewrite required | Essential rendering subsystem concept; reimplement against 1.20.1 model/registry/biome/light/shader data rather than copy as-is. |

## Part 6 — Entries 126–150

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 126 | `crates/horizon-client-render/src/mesher/lighting.rs` | Adaptable; Reference | Highly adaptable as algorithm reference; constants, state facts, float narrowing and shader packing need 1.20.1 parity tests and ARM64 validation. |
| 127 | `crates/horizon-client-render/src/world.rs` | Adaptable | Adaptable as resident-world orchestration; ownership and invalidation should be simplified and verified against 1.20.1. |
| 128 | `crates/horizon-client-render/src/chunk/block_entity.rs` | Adaptable | Adaptable as typed-data architecture; enum/extension types must be rebuilt for 1.20.1 and revision semantics made safer. |
| 129 | `crates/horizon-world/src/compression.rs` | Adaptable | Adaptable as bounded compression layer; LZ4 variant and codec-ID table require version verification. |
| 130 | `crates/horizon-world/src/datafix/mod.rs` | Reference | Reference for migration architecture only; current 26.3 fix pipeline is unsuitable for base and must be independently versioned if legacy imports are required. |
| 131 | `crates/horizon-world/src/datafix/block_state_data.rs` | Unnecessary; Reference | Unnecessary for 1.20.1 base; retain only as historical migration reference if pre-1.13 import becomes a requirement. |
| 132 | `crates/horizon-world/src/datafix/chunk_flatten.rs` | Unnecessary | Unnecessary for 1.20.1 runtime; keep only if pre-1.13 world import becomes explicit. |
| 133 | `crates/horizon-world/src/datafix/world_upgrader.rs` | Unnecessary; Optional/future | Unnecessary for first 1.20.1 milestone; transactional background conversion is a future-tool pattern only. |
| 134 | `crates/horizon-world/src/datafix/tests.rs` | Reference | Reference-only for migration-test structure; expected 26.3 output is not a 1.20.1 oracle. |
| 135 | `crates/horizon-world/examples/probe/datafix_world.rs` | Adaptable; Unnecessary | Unnecessary in production; adaptable as development-only runner with safer destination safeguards. |
| 136 | `crates/horizon-client/examples/probe/dispatch.rs` | Adaptable | Adaptable as development-only test harness utility; no production dependency. |
| 137 | `crates/horizon-world/examples/probe/dispatch.rs` | Adaptable | Adaptable as development/test harness only; not shipping runtime. |
| 138 | `crates/horizon-world/examples/probe/chunk_dump.rs` | Adaptable | Adaptable as host-side worldgen parity tool; current generator/resources/fixtures are 26.3-specific. |
| 139 | `crates/horizon-core/src/no_ai.rs` | Unnecessary; Reference | Unnecessary for vanilla 1.20.1 base; retain as reference only if developer/agent control is introduced. |
| 140 | `crates/horizon-core/src/random/worldgen.rs` | Adaptable | Adaptable RNG architecture; exact source selection and worldgen semantics need 1.20.1 validation/versioning. |

| 141 | `.claude/skills/vanilla-parity/SKILL.md` | Adaptable; Rewrite required | Use the parity-review method, but rebuild the mapping and oracle for Minecraft 1.20.1. |
| 142 | `.dockerignore` | Adaptable | Keep Docker contexts small and private; rewrite exclusions after verifying RustChunk build inputs. |
| 143 | `.gitattributes` | Adaptable | Enforce stable line endings for RustChunk-generated files and cross-platform checks. |
| 144 | `.githooks/pre-commit` | Adaptable | Useful local public-tree check; must be backed by CI because hooks can be bypassed. |
| 145 | `.githooks/pre-push` | Adaptable | Useful pre-push public-tree guard; must be backed by CI and checker tests. |
| 146 | `.github/actionlint.yaml` | Reference; Optional/future | Only relevant if RustChunk adopts self-hosted CI runners; runner trust is critical. |
| 147 | `.github/CODEOWNERS` | Adaptable | Protect signing, update, resource-acquisition and release paths with enforced owner review. |
| 148 | `.github/ISSUE_TEMPLATE/bug_report.yml` | Adaptable | Strong Android bug-triage template; rewrite fields and verify log privacy/redaction. |
| 149 | `.github/ISSUE_TEMPLATE/config.yml` | Adaptable | Issue routing pattern only; replace the Horizon-specific destination. |
| 150 | `.github/ISSUE_TEMPLATE/native-build.yml` | Unnecessary; Optional/future | Not needed for vanilla 1.20.1; Starling native builds are not Forge support. |

## Part 7 — Entries 151–160

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 151 | `.github/pull_request_template.md` | Adaptable; Rewrite required | Useful PR provenance and evidence checklist; replace 26.3 references and validate with CI. |
| 152 | `.github/release-line.txt` | Undetermined | Meaning is not clear until its consumers are traced; do not copy the value yet. |
| 153 | `.github/trusted-reviewers.txt` | Reference; Optional/future | Repository moderation allowlist only; consumer scripts and permissions need review. |
| 154 | `.github/workflows/cancel-on-close.yml` | Adaptable | CI cost-control pattern; verify pull_request_target safety and run-selection behavior. |
| 155 | `.github/workflows/ci.yml` | Adaptable; Rewrite required | Strong CI foundation, but RustChunk needs 1.20.1 parity and separate Android/device gates. |
| 156 | `.github/workflows/claim-command.yml` | Unnecessary; Optional/future | Contributor-claim automation only; not part of the client and requires parser/auth review. |
| 157 | `.github/workflows/mod-registry.yml` | Adaptable; Unnecessary; Optional/future | Isolated workspace CI pattern; mod registry is not needed in the vanilla milestone. |
| 158 | `.github/workflows/mojang-republish.yml` | Adaptable | Useful pinned-metadata change alert; rewrite and test against RustChunk's 1.20.1 acquisition lock. |
| 159 | `.github/workflows/native-hook-stamp.yml` | Unnecessary; Optional/future | Generated native-hook ID service is irrelevant until an extension ABI is approved. |
| 160 | `.github/workflows/native-mods.yml` | Unnecessary; Reference; Optional/future | High-risk Starling native-code pipeline; not Forge and not part of the vanilla client. |

## Part 8 — Entries 161–175

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 161 | `.github/workflows/overlap-note.yml` | Adaptable; Reference | Trusted main-branch, diff-only PR overlap automation; privileged event/runner isolation needs security tests. |
| 162 | `.github/workflows/pack-bench.yml` | Adaptable; Optional/future | Resource-pack branch/base benchmark pattern; replace 26.3 fixtures and add Android measurements. |
| 163 | `.github/workflows/parity-main.yml` | Adaptable; Rewrite required | Useful parity gate/bisect pattern; requires complete 1.20.1 oracle and protected cache/runner gates. |
| 164 | `.github/workflows/pr-labels.yml` | Adaptable; Reference | Metadata-only PR labeling; test untrusted event data and never execute PR-head code. |
| 165 | `AGENTS.md` | Adaptable; Rewrite required | Strong navigation/governance template; rewrite Starling paths, versions, tools and policy assumptions. |
| 166 | `bench/bench.py` | Adaptable; Optional/future | Server-load benchmark only; later needs protocol-763 bots/server and process-cleanup hardening. |
| 167 | `benchmarks/entity-model-document-2026-10-14/generate_fixture.py` | Adaptable; Reference; Optional/future | Deterministic synthetic model fixture; regenerate using 1.20.1 resource schema. |
| 168 | `benchmarks/entity-model-document-2026-10-14/README.md` | Adaptable; Reference | Output-equality and warm/alternating method; 42.94× result is only a synthetic parser segment. |
| 169 | `benchmarks/entity-model-document-2026-10-14/results.json` | Reference; Adaptable | Raw timings agree with README but lack independent fixture/source/hardware provenance. |
| 170 | `benchmarks/graphics/after-4080.json` | Adaptable; Reference; Version-specific | RTX4080/Vulkan post-change report; RD16 drawn_sections=0 requires investigation. |
| 171 | `benchmarks/graphics/after-lavapipe.json` | Adaptable; Reference; Version-specific | Software Vulkan stress data only; RD16 drawn_sections=0 and RD32 lows need context. |
| 172 | `benchmarks/graphics/baseline-4080.json` | Reference; Version-specific | Historical RTX4080 baseline; zero/near-zero draw counters limit interpretation. |
| 173 | `benchmarks/graphics/baseline-lavapipe.json` | Reference; Version-specific | Historical llvmpipe baseline; not a physical GPU or phone proxy. |
| 174 | `benchmarks/model-state-parse-2026-10-14.md` | Adaptable; Reference | Shared parse reuse gives a scoped synthetic 1.306× result; validate 1.20.1 and A22. |
| 175 | `benchmarks/overlay-real-packs-2026-10-15.json` | Adaptable; Reference | Cache-hit/cold overlay methodology; pack schema is 26.3 and licenses remain separate. |

## Part 9 — Entries 176–200

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 176 | `benchmarks/perf/horizon-m4pro-1080p-rd16.json` | Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 177 | `benchmarks/perf/horizon-m4pro-1080p-rd32.json` | Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 178 | `benchmarks/perf/horizon-m4pro-1080p-rd8.json` | Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 179 | `benchmarks/perf/horizon-win-9800x3d-5080-rd16.json` | Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 180 | `benchmarks/perf/horizon-win-9800x3d-5080-rd32.json` | Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 181 | `benchmarks/perf/horizon-win-9800x3d-5080-rd8.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 182 | `benchmarks/perf/macmini-m4pro-loaded-2026-11-01.md` | Reference; Version-specific | The report explicitly says it is not a fair player comparison: system contention differs; vanilla draws 14% fewer… |
| 183 | `benchmarks/perf/vanilla-m4pro-1080p-rd16.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 184 | `benchmarks/perf/vanilla-m4pro-1080p-rd32.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 185 | `benchmarks/perf/vanilla-m4pro-1080p-rd8.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 186 | `benchmarks/perf/vanilla-mods-m4pro-1080p-rd16.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 187 | `benchmarks/perf/vanilla-mods-m4pro-1080p-rd32.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 188 | `benchmarks/perf/vanilla-mods-m4pro-1080p-rd8.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 189 | `benchmarks/perf/vanilla-mods-win-9800x3d-5080-rd16.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 190 | `benchmarks/perf/vanilla-mods-win-9800x3d-5080-rd32.json` | Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 191 | `benchmarks/perf/vanilla-mods-win-9800x3d-5080-rd8.json` | Reference; Version-specific | Each report contains three raw flights per scene. Across all 18 files in this batch, FPS summary values match the… |
| 192 | `benchmarks/perf/vanilla-win-9800x3d-5080-rd16.json` | Reference; Version-specific | Each report contains three raw flights per scene. Across all 18 files in this batch, FPS summary values match the… |
| 193 | `benchmarks/perf/vanilla-win-9800x3d-5080-rd32.json` | Reference; Version-specific | Each report contains three raw flights per scene. Across all 18 files in this batch, FPS summary values match the… |
| 194 | `benchmarks/perf/vanilla-win-9800x3d-5080-rd8.json` | Reference; Version-specific | Each report contains three raw flights per scene. Across all 18 files in this batch, FPS summary values match the… |
| 195 | `benchmarks/perf/windows-9800x3d-5080-2026-10-29.md` | Adaptable; Rewrite required; Version-specific | Use the benchmark controls and reporting method; rewrite workloads and environment for Minecraft 1.20.1 and Android/Mali. |
| 196 | `benchmarks/rust-active-effect-tick-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |
| 197 | `benchmarks/rust-active-effects-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |
| 198 | `benchmarks/rust-anchor-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |
| 199 | `benchmarks/rust-attribute-health-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |
| 200 | `benchmarks/rust-attribute-map-queries-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |

## Part 10 — Entries 201–225

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 201 | `benchmarks/rust-attribute-memory-2026-10-07/after.json` | Adaptable; Reference; Version-specific | گزارش x86_64 لینوکس است و scope صریحاً world/entity کامل، زمان، رندر و FPS را خارج می‌کند؛ با before مقایسه می‌شود،… |
| 202 | `benchmarks/rust-attribute-memory-2026-10-07/before.json` | Adaptable; Reference; Version-specific | مقایسهٔ before/after به تغییر یک منبع entity attributes اشاره دارد؛ این اعداد RSS کل process هستند و شامل… |
| 203 | `benchmarks/rust-attribute-memory-2026-10-07/entity-regression.json` | Adaptable; Version-specific | نسخه 26.3؛ bulk assignment، supplierهای سفارشی، save/network codec و entity زنده خارج از دامنه‌اند |
| 204 | `benchmarks/rust-attribute-memory-2026-10-07/health-regression.json` | Adaptable; Version-specific | 26.3؛ absorption synchronization، damage کامل و gameplay زنده هنوز خارج از دامنه‌اند |
| 205 | `benchmarks/rust-attribute-updates-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ هر دو reference hash یکسان دارند، اما فیلد top-level به نام rust_output_sha256 وجود ندارد؛ پیوند digest نهایی… |
| 206 | `benchmarks/rust-attributes-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ attribute registry/entity/equipment integration و packed data/network codecs جداگانه باقی‌اند؛ client/server… |
| 207 | `benchmarks/rust-bamboo-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ heightها کنترل‌شده‌اند و پس از write به‌روزرسانی نمی‌شوند؛ world updates و generation کامل خارج از دامنه‌اند |
| 208 | `benchmarks/rust-basic-feature-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ chunk mutation واقعی، neighbor updates، lighting، entity/block drops و complete generated worlds بیرون دامنه‌اند |
| 209 | `benchmarks/rust-biome-chunk-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ identityهای holder مصنوعی‌اند؛ native palette layout، wire encoding، biome generation، blending و retrogen… |
| 210 | `benchmarks/rust-biome-generation-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ holderها identity placeholder هستند؛ preset builder، full biome-source codec، blending، retrogen و complete… |
| 211 | `benchmarks/rust-biome-placement-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ biome registry equality، complete decoration و gameplay باقی‌اند؛ خروجی client/server با Rust hash برابر است |
| 212 | `benchmarks/rust-biome-source-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ End density ورودی ثابت است؛ واقعیات seeded density، codec، registry/preset و world integration جدا مانده‌اند |
| 213 | `benchmarks/rust-biome-source-codec-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ full inline biome definitions و live registry reload خارج دامنه‌اند؛ client/server hashها با Rust خروجی منطبق‌اند |
| 214 | `benchmarks/rust-biome-temperature-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ weather placement/freezing کامل و auto-migration بین threadها خارج scope است |
| 215 | `benchmarks/rust-biome-weather-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ dynamic block entities، lighting propagation، mutation و complete weather simulation خارج scope هستند |
| 216 | `benchmarks/rust-biome-zoom-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ storage/source generation، blending و chunk integration خارج scope هستند |
| 217 | `benchmarks/rust-block-blob-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ Rust یک work cap دارد و resident-world updates و complete gameplay خارج scope هستند |
| 218 | `benchmarks/rust-block-collision-scan-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ registry-specific shape dispatch، live chunk loading و entity movement خارج scope‌اند |
| 219 | `benchmarks/rust-block-column-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ actual chunks، neighbor updates و complete gameplay خارج scope؛ bounded work limit در Rust اعمال شده است |
| 220 | `benchmarks/rust-block-entity-storage-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ ساخت entity از NBT، serialization کامل، active behavior و chunk lifecycle بیرون scope‌اند |
| 221 | `benchmarks/rust-block-landing-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ health واقعی، entity storage، network/audio و movement lifecycle کامل خارج دامنه‌اند |
| 222 | `benchmarks/rust-block-pile-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ work limit اعمال شده؛ block entities، resident chunks، block updates و gameplay کامل pending هستند |
| 223 | `benchmarks/rust-block-properties-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ item/fluid replacement، support، survival، dynamic collision و gameplay کامل بیرون scope است |
| 224 | `benchmarks/rust-block-push-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ collision shapes/entity selections کنترل‌شده‌اند؛ live entity indexing و teleport/network behavior خارج scope است |
| 225 | `benchmarks/rust-block-state-codec-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ registry metadata از دو JAR آمده؛ warning text و lifecycle/complete material codecs pending |

## Part 11 — Entries 226–250

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 226 | `benchmarks/rust-block-support-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ dynamic support، survival و complete gameplay خارج scope است |
| 227 | `benchmarks/rust-block-survival-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ full placement/update، world/entity lifecycle و complete gameplay pending هستند |
| 228 | `benchmarks/rust-blocking-item-damage-2026-10-07/scoped-parity.json` | Adaptable; Reference; Version-specific | 26.3؛ components/codecs، enchantment effects، inventory/equipment lifecycle و combat کامل pending |
| 229 | `benchmarks/rust-blocking-rules-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ direction/acos، component codecs، durability، sounds و complete blocking dispatch خارج scope |
| 230 | `benchmarks/rust-blockstate-model-binding-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ no weighted random choice، transformed bake، renderer integration یا gameplay parity |
| 231 | `benchmarks/rust-blockstate-model-binding-2026-10-06/shared-integer-codec.json` | Adaptable; Reference; Version-specific | 26.3؛ این فایل evidence مستقل از full state-codec path است، نه مجوز استفاده از registry IDs نسخهٔ جدید |
| 232 | `benchmarks/rust-blockstate-model-choice-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ leaf models inert و predicateها از پیش تعیین‌شده‌اند؛ baking، material evaluation و renderer خارج scope است |
| 233 | `benchmarks/rust-blockstate-model-decoding-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ registry binding، property selection، RNG choice، transformed baking و rendered output بیرون scope است |
| 234 | `benchmarks/rust-blockstate-transforms-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ atlas packing، shader pixels، lighting، render classification و runtime renderer integration بررسی نشده‌اند |
| 235 | `benchmarks/rust-blue-ice-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ chunk updates، fluid/ice gameplay و full worldgen خارج scope است؛ Rust work bounds وجود دارد |
| 236 | `benchmarks/rust-bonus-chest-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ loot generation، inventory، automatic block-entity lifecycle و complete spawn generation pending است |
| 237 | `benchmarks/rust-bulk-section-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ Rust exclusive region borrow دارد؛ concurrent ownership، ordinary ore geometry و region side effects خارج scope‌اند |
| 238 | `benchmarks/rust-bundle-weight-2026-10-07/scoped-parity.json` | Unnecessary; Reference; Version-specific | مدل bundle/data components جدید 26.3 است؛ 1.20.1 vanilla item stack مدل legacy NBT دارد و معادل مستقیم ندارد |
| 239 | `benchmarks/rust-canonical-collision-2026-10-06/contextual-regression.json` | Adaptable; Version-specific | 26.3؛ active shulker/piston و entity lifecycle جدا؛ در scope، ۱۹۹ uncached و ۳۲ context-sensitive states مشخص‌اند |
| 240 | `benchmarks/rust-canonical-collision-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ uncached و entity-context behavior به suiteهای جدا موکول شده؛ نتیجه کامل collision engine نیست |
| 241 | `benchmarks/rust-canyon-carver-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ gaussian/weighted providers، neighbor-carver seeding، biome selection و full worlds خارج scope؛ Rust bounds کار دارد |
| 242 | `benchmarks/rust-carving-geometry-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | فقط primitives مشترک هندسی را پوشش می‌دهد؛ randomized cave/canyon paths، codecs و provider sampling را ثابت نمی‌کند |
| 243 | `benchmarks/rust-carving-mask-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ تست mask به‌تنهایی cave/canyon generation، mask application یا terrain generation را اثبات نمی‌کند |
| 244 | `benchmarks/rust-cave-carver-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ gaussian/weighted provider codecs، neighbor biome selection، blending و complete worldgen بیرون scope‌اند |
| 245 | `benchmarks/rust-chorus-plant-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ neighbor-update execution، random ticks، drops، rendering و full End generation خارج scope است |
| 246 | `benchmarks/rust-chunk-stage-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ scheduler، persisted metadata، retrogen، block entities، lighting و complete world خارج scope است |
| 247 | `benchmarks/rust-chunk-status-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ این manifest generation-step execution یا lighting را آزمایش نمی‌کند |
| 248 | `benchmarks/rust-climbing-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ live movement binding، equipment-based glide eligibility و full gameplay pending است |
| 249 | `benchmarks/rust-collision-2026-10-05/scoped-parity.json` | Adaptable; Reference; Version-specific | نسخه 26.3؛ uncached states و entity-context در این manifest پشتیبانی نمی‌شوند |
| 250 | `benchmarks/rust-collision-context-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ entity lifecycle، inventory mutation بعد از capture، minecart experimental contexts و live movement pending |

## Part 12 — Entries 251–275

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 251 | `benchmarks/rust-combat-rules-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ enchantment armor hook، enchanted components، damage tags/equipment و pipeline کامل باقی‌اند |
| 252 | `benchmarks/rust-component-durability-2026-10-07/blocking-regression.json` | Adaptable; Reference; Version-specific | این گزارش جدا از entry 228 است اما scope و خروجی هم‌پوشانی دارند؛ component codec، enchantment و full inventory… |
| 253 | `benchmarks/rust-component-durability-2026-10-07/scalar-regression.json` | Adaptable; Version-specific | 26.3؛ no-enchantment path؛ full component store/codec، enchantment و equipment handling خارج scope است |
| 254 | `benchmarks/rust-component-durability-2026-10-07/scoped-parity.json` | Adaptable; Reference; Version-specific | 26.3 component architecture است؛ full item-component codecs، enchantments و equipment lifecycle pending هستند |
| 255 | `benchmarks/rust-component-patch-2026-10-07/scoped-parity.json` | Unnecessary; Reference; Version-specific | 26.3 data component system؛ prototype hash-table construction، identity hashes، mutable aliases و item/inventory… |
| 256 | `benchmarks/rust-consumable-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ eligibility/start-use، stat service، effect bodies و event delivery کنترل‌شده‌اند؛ full lifecycle/gameplay pending |
| 257 | `benchmarks/rust-consumable-emission-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ audio playback، rendering، completion/effects و full gameplay خارج scope‌اند |
| 258 | `benchmarks/rust-consume-effect-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ actor storage و sound delivery کنترل‌شده‌اند؛ holder binding، teleport handling و integration کامل pending |
| 259 | `benchmarks/rust-contextual-collision-2026-10-06/scoped-parity.json` | Adaptable; Reference; Version-specific | هم‌پوشانی عمدی با entry 239 دارد؛ دو manifest را یکی فرض نکنید، اما رفتارهای مشترک را دوباره پیاده‌سازی نکنید |
| 260 | `benchmarks/rust-core-2026-10-05/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ انتخابی و غیرجامع است؛ gameplay، rendering و performance را اثبات نمی‌کند |
| 261 | `benchmarks/rust-damage-immunity-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ flags/tags/rules کنترل‌شده و enchantment equipment خالی است؛ registered source/rule binding و full pipeline pending |
| 262 | `benchmarks/rust-damage-reduction-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ enchantments، durability effects، live registries و full damage integration باقی‌اند |
| 263 | `benchmarks/rust-damage-source-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ catalog indices محلی‌اند نه network IDs؛ live registry reload، Rust codec و death messages pending |
| 264 | `benchmarks/rust-decoration-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ structures disabled و feature payloadها synthetic هستند؛ full registry/world lifecycle خارج scope |
| 265 | `benchmarks/rust-decoration-region-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ WorldGenRegion policy، POI، block entities، lighting، structures و full terrain lifecycle خارج scope |
| 266 | `benchmarks/rust-delta-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ world updates، fluid behavior و full generation خارج scope‌اند |
| 267 | `benchmarks/rust-density-2026-10-05/scoped-parity.json` | Adaptable; Version-specific | manifest فیلد صریح minecraft ندارد؛ manifest_sha1 پین 26.3 را مشخص می‌کند. Blend غیرخالی، باقی graph definitions،… |
| 268 | `benchmarks/rust-disk-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ manifest درخواست mark را ثبت می‌کند، نه اجرای کامل post-processing بعدی؛ full generation pending |
| 269 | `benchmarks/rust-dynamic-support-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ entity lifecycle، ticking/pushing و full gameplay خارج scope‌اند |
| 270 | `benchmarks/rust-edge-backoff-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ live crouch state، attribute provisioning و integrated ledge behavior خارج scope هستند |
| 271 | `benchmarks/rust-effect-attribute-modifiers-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ effect registry definitions، pending sets ownership، notifications و full gameplay جدا باقی‌اند |
| 272 | `benchmarks/rust-effect-notifications-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | manifest فیلد top-level rust_output_sha256 ندارد؛ هر دو JAR reference خروجی یکسان و ۱٬۶۶۷٬۶۱۶ matched lines دارند،… |
| 273 | `benchmarks/rust-effect-sync-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | فیلد top-level rust_output_sha256 وجود ندارد؛ دو reference خروجی برابر با ۲٬۷۵۴٬۷۴۴ matched lines ثبت کرده‌اند، اما… |
| 274 | `benchmarks/rust-effect-sync-2026-10-07/synced-bytes-regression.json` | Adaptable; Version-specific | 26.3؛ heterogeneous serializer maps، entity schema/network codecs و complete item-use flow خارج scope است |
| 275 | `benchmarks/rust-effect-sync-2026-10-07/synced-floats-regression.json` | Adaptable; Version-specific | 26.3؛ سایر serializers، type mismatch handling، wire codecs و player absorption integration pending |

## Part 13 — Entries 276–300

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 276 | `benchmarks/rust-end-gateway-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ entity lifecycle، dirty propagation، teleport/search و complete End generation خارج scope‌اند |
| 277 | `benchmarks/rust-end-podium-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ destruction outcome کنترل‌شده است؛ drops، block entities، neighbor updates و dragon/portal gameplay خارج scope‌اند |
| 278 | `benchmarks/rust-entity-attributes-2026-10-07/scoped-parity.json` | Adaptable; Reference; Version-specific | 26.3؛ bulk assignment، custom suppliers، save/network codecs، entity live و gameplay pending |
| 279 | `benchmarks/rust-entity-collision-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ snapshots و vehicle links کنترل‌شده‌اند؛ live storage، broad-phase indexing و entity lifecycle خارج scope هستند |
| 280 | `benchmarks/rust-entity-fluid-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ height/flow و camera-support کنترل‌شده؛ live fluid propagation/world integration pending |
| 281 | `benchmarks/rust-entity-fluid-tick-2026-10-07/immersion-regression.json` | Adaptable; Reference; Version-specific | این regression به entry280 هم‌پوشانی دارد؛ مستقل ثبت شده اما نباید به‌عنوان قابلیت دوم یا پوشش کامل fluid tick شمرده شود |
| 282 | `benchmarks/rust-entity-fluid-tick-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ splash render/audio و live world/entity binding و gameplay کامل خارج scope است |
| 283 | `benchmarks/rust-entity-move-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ step height صفر و بسیاری از world providers کنترل‌شده‌اند؛ full player/world movement integration باقی است |
| 284 | `benchmarks/rust-entity-obstruction-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ entity construction/lifecycle، spatial indexing، vehicle maintenance و complete gameplay خارج scope‌اند |
| 285 | `benchmarks/rust-extra-items-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ item handlers، full component schemas و actual dropped-entity creation/lifecycle pending |
| 286 | `benchmarks/rust-face-support-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ انتخاب support shape برای هر block، world/entity context و survival خارج scope است |
| 287 | `benchmarks/rust-face-visibility-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ chunk mesh integration، lighting، translucent sorting، fluid surfaces، GPU pixels و gameplay ثابت نشده‌اند |
| 288 | `benchmarks/rust-fall-damage-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ LivingEntity fluid updates، Block.fallOn، health/damage/attributes و full movement lifecycle در این تست نیست |
| 289 | `benchmarks/rust-feature-catalog-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ unused/cyclic registry resources، reload/lifecycle، encoding کامل، همهٔ feature algorithms و generated worlds… |
| 290 | `benchmarks/rust-feature-graph-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ named registry holders/tags، encoders، full feature registries و complete generated worlds pending |
| 291 | `benchmarks/rust-feature-placement-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ configured feature algorithms, registry, nested features, Gaussian providers, encoding و complete worldgen خارج scope |
| 292 | `benchmarks/rust-feature-region-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ storage proxy جای WorldGenRegion واقعی است؛ write-zone، POI، block entities، postprocessing، lighting و… |
| 293 | `benchmarks/rust-feature-region-destroy-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ WorldGenRegion واقعی، POI، block entity lifecycle، lighting و اجرای واقعی tickها خارج scope هستند |
| 294 | `benchmarks/rust-feature-region-queries-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ actual WorldGenRegion، postprocessing execution، lighting و complete generation هنوز بیرون scope است |
| 295 | `benchmarks/rust-feature-region-ticks-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ tick execution، WorldGenRegion واقعی، lighting و complete feature registry خارج scope است |
| 296 | `benchmarks/rust-feature-region-upgrade-height-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ retrogen activation caller-owned است؛ full retrogen replacement/masking، biome behavior و complete worlds… |
| 297 | `benchmarks/rust-feature-region-write-zone-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ vertical upgrade restriction، region construction، POIs, lighting, ticks و full worldgen بیرون scope‌اند |
| 298 | `benchmarks/rust-feature-sort-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ diagnostic minimization، complete registry ownership، decoration seeds و feature execution خارج scope‌اند |
| 299 | `benchmarks/rust-fill-layer-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | 26.3؛ world side effects و other configured features/full worlds خارج scope‌اند |
| 300 | `benchmarks/rust-float-log-2026-10-05/scoped-parity.json` | Adaptable; Version-specific | تست فقط scalar float log روی یک JDK/platform است؛ double Math.log، دیگر platformها، bulk composition، terrain یا… |

## Part 14 — Entries 301–325

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 301 | `benchmarks/rust-fluid-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Registered state inventory is from 26.3; fluid spreading, live world binding and gameplay are absent. |
| 302 | `benchmarks/rust-fluid-box-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Passenger lifecycle and actual live boat/world integration are controlled or absent. |
| 303 | `benchmarks/rust-fluid-flow-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | World registry/provider binding, propagation and spreading are not covered. |
| 304 | `benchmarks/rust-fluid-occupancy-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Resident snapshots are controlled; live entity collision composition and world integration remain open. |
| 305 | `benchmarks/rust-fluid-shape-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Fluid propagation, immersion and ray/world binding are outside scope. |
| 306 | `benchmarks/rust-food-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Player damage/health services, consumables, persistence and complete player tick are not integrated. |
| 307 | `benchmarks/rust-food-properties-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Sound playback, concrete consumable dispatch, particles/effects and full use lifecycle are omitted. |
| 308 | `benchmarks/rust-food-storage-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Only FoodData fields are covered; complete player save lifecycle and other fields are outside scope. |
| 309 | `benchmarks/rust-free-space-2026-10-07/collision-regression.json` | Adaptable; Version-specific | Entity lists, border conditions and chunk snapshots are controlled; full live entity indexing is absent. |
| 310 | `benchmarks/rust-free-space-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Chunk generation, live entity indexing and complete gameplay are not represented. |
| 311 | `benchmarks/rust-generation-ticks-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | This records queue behavior, not execution of actual block/fluid ticks or complete generation. |
| 312 | `benchmarks/rust-geometry-2026-10-05/scoped-parity.json` | Adaptable; Version-specific | Selected operations are tested in isolation; not a full world collision or camera pipeline. |
| 313 | `benchmarks/rust-geometry-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Only selected primitives and controlled cases; complete collision/shape consumers are not included. |
| 314 | `benchmarks/rust-ground-support-2026-10-07/scan-regression.json` | Adaptable; Version-specific | Controlled providers and contexts; registered 1.20.1 shapes, live loading and full entity movement remain untested. |
| 315 | `benchmarks/rust-ground-support-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No top-level Rust output digest is present; provider/registry integration is controlled. |
| 316 | `benchmarks/rust-height-provider-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Warning logs and complete generation placement are excluded; 26.3 codec data is version-specific. |
| 317 | `benchmarks/rust-heightmap-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Imported air/fluid facts do not port block behavior; full chunk lifecycle and generation are not covered. |
| 318 | `benchmarks/rust-interaction-shape-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Shape table belongs to 26.3; world ray binding and complete gameplay are excluded. |
| 319 | `benchmarks/rust-inventory-pickup-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Pickup entities, menus, networking, arbitrary overrides and full item components are not included. |
| 320 | `benchmarks/rust-inventory-tick-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Concrete registered item effects and the enclosing player/world tick are pending. |
| 321 | `benchmarks/rust-inventory-use-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Does not cover full start-use path, concrete consumable effects, packet delivery or complete Player.tick. |
| 322 | `benchmarks/rust-inventory-use-start-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Packet delivery, combat effects and full tick/network lifecycle are outside scope. |
| 323 | `benchmarks/rust-item-after-use-2026-10-07/remainder-regression.json` | Adaptable; Version-specific | Actual item insertion/drop, full component codec and concrete use effects remain outside scope. |
| 324 | `benchmarks/rust-item-after-use-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Concrete consumables, complete inventory transactions and whole item-use lifecycle are not covered. |
| 325 | `benchmarks/rust-item-blocking-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Real damageable item components, full combat and death flow are not integrated. |

## Part 15 — Entries 326–350

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 326 | `benchmarks/rust-item-cooldowns-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Network synchronization and complete player/item lifecycle are absent. |
| 327 | `benchmarks/rust-item-durability-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Full component codecs, enchantments and complete gameplay are not covered. |
| 328 | `benchmarks/rust-item-particles-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Particle queues/rendering, sound/effects integration and real gameplay are outside scope. |
| 329 | `benchmarks/rust-item-stack-storage-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Component codecs, menu transactions, equipment synchronization and full use flow are not included. |
| 330 | `benchmarks/rust-item-template-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Does not exercise every component codec or complete containers and use lifecycle. |
| 331 | `benchmarks/rust-item-use-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Start-use path, synchronized flags, concrete consumables and full lifecycle remain pending. |
| 332 | `benchmarks/rust-item-use-catalog-2026-10-07/scoped-parity.json` | Reference; Version-specific | Import is 26.3-specific; other component codecs, inventory, enchantments and use effects remain unverified. |
| 333 | `benchmarks/rust-item-use-lifecycle-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Starting use, actual synchronized flags, concrete consumables, remainders and complete tick are incomplete. |
| 334 | `benchmarks/rust-item-use-start-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Transport, wire encoding, persistent data and server/client lifecycle are not exercised. |
| 335 | `benchmarks/rust-java-hash-map-2026-10-07/scoped-parity.json` | Reference; Version-specific | JDK collection parity is not game logic; JDK version/platform semantics must not be assumed. |
| 336 | `benchmarks/rust-java-identity-map-2026-10-07/cooldown-regression.json` | Reference; Version-specific | This is a Java/JDK regression context, not a complete in-game cooldown lifecycle. |
| 337 | `benchmarks/rust-java-identity-map-2026-10-07/ordered-map-regression.json` | Reference; Version-specific | It is a JDK implementation comparison, not Minecraft feature parity. |
| 338 | `benchmarks/rust-java-identity-map-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No top-level rust_output_sha256; JAR references have one matching output digest, but process identity inputs remain environment-dependent. |
| 339 | `benchmarks/rust-liquid-collision-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Does not implement entity movement or fluid propagation in a live world. |
| 340 | `benchmarks/rust-live-attribute-health-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Live refresh paths and complete damage/player tick are not fully integrated. |
| 341 | `benchmarks/rust-live-attribute-updates-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No top-level Rust output digest; client/server reference outputs differ, so not a single equivalent aggregate. |
| 342 | `benchmarks/rust-live-attributes-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No top-level Rust output digest; client/server outputs differ; complete entity supplier lifecycle remains open. |
| 343 | `benchmarks/rust-live-player-combat-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Several knockback, criterion, source and world lifecycle consumers remain controlled; output has separate Rust/live hashes. |
| 344 | `benchmarks/rust-live-upgrade-region-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Proxy is not a real WorldGenRegion; complete chunks/worldgen, POI, light and postprocessing are pending. |
| 345 | `benchmarks/rust-living-air-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Full Player.travel and live effects/world providers are not integrated. |
| 346 | `benchmarks/rust-living-damage-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Damage service composition and full world/player lifecycle remain controlled. |
| 347 | `benchmarks/rust-living-fall-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Actual world landing callbacks, enchantment hooks and complete damage application are not included. |
| 348 | `benchmarks/rust-living-fluid-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Fluid propagation, full vehicle logic and playable world integration remain pending. |
| 349 | `benchmarks/rust-living-health-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Full registered attribute binding, damage sources and complete player lifecycle are not covered. |
| 350 | `benchmarks/rust-living-hurt-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Totems, criteria, blocking, source service and enclosing server tick are not fully integrated. |

## Part 16 — Entries 351–375

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 351 | `benchmarks/rust-living-landing-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Full fluid immersion, enchantment hooks, particles rendering and Player/world integration remain pending. |
| 352 | `benchmarks/rust-living-motion-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Live overrides, effects and complete Player/entity integration are not included. |
| 353 | `benchmarks/rust-material-condition-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Caller-owned context/data and complete worldgen registry/encoding are not included. |
| 354 | `benchmarks/rust-material-extension-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Thresholds and callbacks are controlled; does not establish complete biome or chunk generation. |
| 355 | `benchmarks/rust-material-rule-library-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Encoding, reload, registry validation and full generated worlds are not covered. |
| 356 | `benchmarks/rust-material-stage-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Only bounded 3x3 biome input and generated terrain fixtures; full chunk/biome pipeline is absent. |
| 357 | `benchmarks/rust-material-surface-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Full rule evaluation, extensions, generation stages and chunk integration remain pending. |
| 358 | `benchmarks/rust-model-baking-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No atlas packing, blockstate transform integration, shader/pixel or GPU parity. |
| 359 | `benchmarks/rust-model-resolution-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Does not cover quad baking, blockstate variants, atlas, lighting or visual parity. |
| 360 | `benchmarks/rust-movement-emission-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Does not play audio or connect to a live player movement loop. |
| 361 | `benchmarks/rust-movement-registry-2026-10-07/scoped-parity.json` | Reference; Version-specific | 26.3 coefficients/tags are not 1.20.1 data; does not prove live player movement. |
| 362 | `benchmarks/rust-movement-support-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Not an integrated Player movement loop; target block/attribute registries differ. |
| 363 | `benchmarks/rust-native-attribute-updates-2026-10-07/input-scene.json` | Reference; Version-specific | No aggregate Rust output hash; scene manifest is an input description, not an acceptance result. |
| 364 | `benchmarks/rust-native-attribute-updates-2026-10-07/interactive-smoke.json` | Reference; Version-specific | Linux x86_64 Vulkan llvmpipe CPU renderer; default actor/scene smoke only, not game, GPU performance or Android evidence. |
| 365 | `benchmarks/rust-native-attribute-updates-2026-10-07/scene-motion.json` | Adaptable; Version-specific | No top-level Rust output digest despite reference data; fluids/effects/inventory/network and full Player tick are omitted. |
| 366 | `benchmarks/rust-native-attribute-updates-2026-10-07/startup-smoke.json` | Reference; Version-specific | CPU llvmpipe scene only; startup snapshot cannot demonstrate playable game, Android FPS or native pixel parity. |
| 367 | `benchmarks/rust-native-blockstate-client-2026-10-06/startup-smoke.json` | Adaptable; Version-specific | Linux llvmpipe software renderer; no native pixel comparison or hardware FPS. |
| 368 | `benchmarks/rust-native-chunk-models-2026-10-06/startup-smoke.json` | Reference; Version-specific | Linux llvmpipe and private screenshots; not Android GPU/frame pacing or whole-world loading evidence. |
| 369 | `benchmarks/rust-native-crouch-2026-10-07/input-scene.json` | Reference; Version-specific | Controlled scene and query gates; full client/server tick, effects, vehicles and network are omitted. |
| 370 | `benchmarks/rust-native-crouch-2026-10-07/interactive-smoke.json` | Reference; Version-specific | Linux llvmpipe; only 8 cm recorded walk, no fluid/other poses/full client lifecycle or Android device result. |
| 371 | `benchmarks/rust-native-crouch-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Does not prove full LocalPlayer tick, GUI settings persistence, network or full gameplay. |
| 372 | `benchmarks/rust-native-crouch-2026-10-07/sprint-input.json` | Adaptable; Version-specific | No full parent movement, complete client tick, GUI persistence, networking or gameplay parity. |
| 373 | `benchmarks/rust-native-crouch-2026-10-07/startup-smoke.json` | Reference; Version-specific | 30-frame CPU llvmpipe startup; no active movement, game lifecycle or hardware FPS claim. |
| 374 | `benchmarks/rust-native-health-2026-10-07/interactive-smoke.json` | Reference; Version-specific | Linux llvmpipe resident scene; death/drop/respawn, full hunger and device performance are not tested. |
| 375 | `benchmarks/rust-native-health-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Full Player.tick, inventory/equipment, freezing, packets and death/drop/respawn services remain outside scope. |

## Part 17 — Entries 376–400

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 376 | `benchmarks/rust-native-health-2026-10-07/startup-smoke.json` | Reference; Version-specific | No player lifecycle or movement; Linux llvmpipe CPU renderer only. |
| 377 | `benchmarks/rust-native-hunger-2026-10-07/interactive-smoke.json` | Reference; Version-specific | Elapsed wall time is not simulation time; complete enclosing ticks, item use, death lifecycle and GPU/device evidence are absent. |
| 378 | `benchmarks/rust-native-hunger-2026-10-07/scene-motion.json` | Adaptable; Version-specific | No complete outer Player.tick, item-use path, full death/drop/respawn or device integration. |
| 379 | `benchmarks/rust-native-landing-2026-10-07/interactive-smoke.json` | Reference; Version-specific | Damage request is not applied to health; Linux CPU scene only, no full gameplay or device acceptance. |
| 380 | `benchmarks/rust-native-landing-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Controlled scene excludes many world callbacks, packet acceptance, full player tick and complete death flow. |
| 381 | `benchmarks/rust-native-model-client-2026-10-06/startup-smoke.json` | Reference; Version-specific | Pre-final interactive view differs from final fingerprints; CPU llvmpipe only, no gameplay or complete pixel parity. |
| 382 | `benchmarks/rust-native-motion-tick-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No full Player.aiStep/tick, local input, freezing, health/damage or complete gameplay claim. |
| 383 | `benchmarks/rust-native-movement-statistics-2026-10-07/interactive-smoke.json` | Reference; Version-specific | Linux llvmpipe; sprint/full gameplay and Android hardware FPS are not established. |
| 384 | `benchmarks/rust-native-movement-statistics-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Still excludes complete client/server packet acceptance, inventory/effects and full Player.tick. |
| 385 | `benchmarks/rust-native-scene-collision-2026-10-07/interactive-smoke.json` | Reference; Version-specific | No gameplay simulation; Vulkan llvmpipe CPU adapter, not target GPU or Android FPS. |
| 386 | `benchmarks/rust-native-scene-collision-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No aggregate Rust output hash; no gravity, tick schedule, fluid/entity gameplay or pixel renderer parity. |
| 387 | `benchmarks/rust-native-scene-collision-2026-10-07/startup-smoke.json` | Reference; Version-specific | 30-frame CPU llvmpipe snapshot; not a performance benchmark or playable game proof. |
| 388 | `benchmarks/rust-native-scene-motion-2026-10-07/collision-regression.json` | Adaptable; Version-specific | Collision adapter only; does not establish live player travel, gravity, tick, fluids or full gameplay. |
| 389 | `benchmarks/rust-native-scene-motion-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Fluids, vehicles, effects, crouch, landing/damage, audio and network consumers remain controlled; no full player tick. |
| 390 | `benchmarks/rust-native-scene-motion-2026-10-07/startup-smoke.json` | Reference; Version-specific | Linux llvmpipe CPU renderer only; no visual pixel parity or Android device measurement. |
| 391 | `benchmarks/rust-native-sprint-2026-10-07/double-tap-smoke.json` | Reference; Version-specific | Linux llvmpipe; focus pauses walking, and full pose/fluid/effect/item-use/network lifecycle is outside scope. |
| 392 | `benchmarks/rust-native-sprint-2026-10-07/scene-motion.json` | Adaptable; Version-specific | No complete outer client/server tick, GUI persistence, effects, inventory, networking or full gameplay. |
| 393 | `benchmarks/rust-native-sprint-2026-10-07/sprint-input.json` | Adaptable; Version-specific | Setter records requests; actual parent movement, GUI options and network lifecycle are separate. |
| 394 | `benchmarks/rust-native-sprint-2026-10-07/sprint-key-smoke.json` | Reference; Version-specific | Linux llvmpipe; no fluid/effect/item-use/client-server lifecycle, native pixel or A22 performance claim. |
| 395 | `benchmarks/rust-native-sprint-2026-10-07/sprint-scene.json` | Adaptable; Version-specific | Raw shift is recorded but crouch is unbound; effects/equipment/vehicles/fluids, complete ticks and networking remain excluded. |
| 396 | `benchmarks/rust-native-walking-2026-10-07/interactive-smoke.json` | Reference; Version-specific | Linux llvmpipe development scene; no full gameplay, native pixel parity or device acceptance. |
| 397 | `benchmarks/rust-native-walking-2026-10-07/local-input.json` | Adaptable; Version-specific | Does not establish full LocalPlayer tick, input remapping, sprint eligibility, pose/camera or network lifecycle. |
| 398 | `benchmarks/rust-native-walking-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Full Player.tick, inventory/equipment ticks, freezing, health/damage and network consumers are not covered. |
| 399 | `benchmarks/rust-native-walking-2026-10-07/startup-smoke.json` | Reference; Version-specific | Linux llvmpipe CPU renderer only; no walking/game loop or hardware FPS evidence. |
| 400 | `benchmarks/rust-neighbor-spread-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Actual chunks, neighbor callbacks and complete feature/world generation are pending. |

## Part 18 — Entries 401–425

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 401 | `benchmarks/rust-noise-2026-10-05/scoped-parity.json` | Adaptable; Version-specific | Selected maths only; no complete terrain/world proof |
| 402 | `benchmarks/rust-object-open-map-2026-10-07/scoped-parity.json` | Adaptable; Reference; Version-specific | Collection parity is not game integration |
| 403 | `benchmarks/rust-object-open-set-2026-10-07/scoped-parity.json` | Adaptable; Reference; Version-specific | Not proof of integrated attribute queues |
| 404 | `benchmarks/rust-pillar-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No complete chunk mutation/biome generation |
| 405 | `benchmarks/rust-piston-movement-2026-10-07/movement-registry-regression.json` | Adaptable; Reference; Version-specific | 26.3 table; no live piston event |
| 406 | `benchmarks/rust-piston-movement-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No piston activation/block lifecycle/network |
| 407 | `benchmarks/rust-piston-shape-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No live block entity/piston lifecycle |
| 408 | `benchmarks/rust-placement-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No full configured-feature composition/world mutation |
| 409 | `benchmarks/rust-player-equipment-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No full synchronized/use/save lifecycle |
| 410 | `benchmarks/rust-player-food-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No full player tick/regeneration/damage/persistence |
| 411 | `benchmarks/rust-player-health-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No full damage source/wire/player lifecycle |
| 412 | `benchmarks/rust-player-hurt-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Downstream damage/effect services controlled |
| 413 | `benchmarks/rust-player-inventory-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No menus/recipes/pickup or all components |
| 414 | `benchmarks/rust-player-travel-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No live fluid/collision/vehicle/effect full tick |
| 415 | `benchmarks/rust-powder-snow-collision-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No freezing/sinking/durability/live movement |
| 416 | `benchmarks/rust-registered-entity-fluid-2026-10-07/immersion-regression.json` | Adaptable; Version-specific | No live fluid propagation or broad entity index |
| 417 | `benchmarks/rust-registered-entity-fluid-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Providers fixture-controlled; no complete liquid movement |
| 418 | `benchmarks/rust-registered-entity-fluid-2026-10-07/tick-regression.json` | Adaptable; Version-specific | No particle/audio playback or live propagation |
| 419 | `benchmarks/rust-registered-player-combat-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | PvP/blocking/durability/effect consumers controlled |
| 420 | `benchmarks/rust-registry-fall-ray-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No live chunk provider or complete consumer |
| 421 | `benchmarks/rust-registry-fluid-2026-10-07/scoped-parity.json` | Adaptable; Reference; Version-specific | 26.3 ids/data; no live spreading |
| 422 | `benchmarks/rust-replace-blobs-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No complete chunk mutation/biome generation |
| 423 | `benchmarks/rust-replace-block-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No region effects/structure processors/decorations |
| 424 | `benchmarks/rust-resident-collision-scan-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Controlled proxy; no live chunk/entity broad-phase |
| 425 | `benchmarks/rust-resource-number-scanner-2026-10-06/model-baking.json` | Adaptable; Version-specific | CPU geometry only; no atlas/shader/GPU pixels |

## Part 19 — Entries 426–450

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 426 | `benchmarks/rust-resource-number-scanner-2026-10-06/model-decoding.json` | Adaptable; Version-specific | No parent resolution/atlas/GPU rendering |
| 427 | `benchmarks/rust-resource-number-scanner-2026-10-06/model-resolution.json` | Adaptable; Version-specific | No final quad bake/light/render pixels |
| 428 | `benchmarks/rust-resource-number-scanner-2026-10-06/sprite-animation.json` | Adaptable; Version-specific | No image decode/atlas upload/GPU |
| 429 | `benchmarks/rust-restitution-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No live grounding/tags/effects/world lifecycle |
| 430 | `benchmarks/rust-retained-upgrade-status-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No save-dirty scheduling/biome migration/full retrogen |
| 431 | `benchmarks/rust-retrogen-biome-fill-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No target palette/full biome generation |
| 432 | `benchmarks/rust-retrogen-biome-generation-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Not full generation; holder placeholders limit fidelity |
| 433 | `benchmarks/rust-retrogen-chunk-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No persistence/scheduler/full lifecycle |
| 434 | `benchmarks/rust-retrogen-data-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No automatic schedule or end-to-end upgrade |
| 435 | `benchmarks/rust-rule-test-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No processor execution or structure mutation |
| 436 | `benchmarks/rust-scaffolding-collision-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No placement/support/fall/live movement |
| 437 | `benchmarks/rust-scattered-ore-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No complete veins/chunk decoration |
| 438 | `benchmarks/rust-server-player-combat-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Effects/durability/PvP/network/death controlled |
| 439 | `benchmarks/rust-server-player-combat-immunity-2026-10-07/inner-damage-regression.json` | Adaptable; Version-specific | Outer gates and attribution/world integration separate |
| 440 | `benchmarks/rust-server-player-combat-immunity-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No complete damage/effects/packets/player tick |
| 441 | `benchmarks/rust-server-player-damage-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Criteria/effects/durability/death controlled |
| 442 | `benchmarks/rust-shared-item-use-2026-10-07/item-use-lifecycle.json` | Adaptable; Version-specific | Start/network/concrete consumables/full player lifecycle pending |
| 443 | `benchmarks/rust-shared-item-use-2026-10-07/item-use-start.json` | Adaptable; Version-specific | Transport and synced flag consumers absent |
| 444 | `benchmarks/rust-shared-item-use-2026-10-07/item-use.json` | Adaptable; Version-specific | Full lifecycle/durability/combat integration absent |
| 445 | `benchmarks/rust-simple-block-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Chunk writes/neighbors/full generation not tested |
| 446 | `benchmarks/rust-simple-container-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No persistence/menu transactions/full components |
| 447 | `benchmarks/rust-snow-freeze-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Lighting/broad mutation/surface generation absent |
| 448 | `benchmarks/rust-speleothem-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No neighbor updates/falling/fluid/full caves |
| 449 | `benchmarks/rust-spike-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No complete End chunk/world lifecycle |
| 450 | `benchmarks/rust-spring-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Tick execution/spreading/full worldgen absent |

## Part 20 — Entries 451–475

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 451 | `benchmarks/rust-sprite-animation-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No image decode/atlas/GPU frame interpolation |
| 452 | `benchmarks/rust-sprite-animation-2026-10-06/startup-smoke.json` | Unnecessary; Reference; Version-specific | No player gameplay/device hardware/FPS |
| 453 | `benchmarks/rust-state-provider-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No registry reload/placement/worldgen integration |
| 454 | `benchmarks/rust-status-effect-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Effect tick/registry/UI lifecycle separate |
| 455 | `benchmarks/rust-status-tick-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Real dispatch/registry/full gameplay outside harness |
| 456 | `benchmarks/rust-stored-consume-2026-10-07/scoped-parity.json` | Adaptable; Reference; Version-specific | No aggregate Rust hash; registration/notification/sound controlled |
| 457 | `benchmarks/rust-stored-effect-attributes-2026-10-07/scoped-parity.json` | Undetermined; Reference; Version-specific | No top-level hash and reference outputs differ |
| 458 | `benchmarks/rust-stored-effect-notifications-2026-10-07/scoped-parity.json` | Undetermined; Reference; Version-specific | No top-level hash; reference output hashes differ |
| 459 | `benchmarks/rust-structure-density-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No structure discovery/placement/terrain integration |
| 460 | `benchmarks/rust-suffocation-2026-10-06/contextual-regression.json` | Adaptable; Version-specific | No live entity/minecart/world lifecycle |
| 461 | `benchmarks/rust-suffocation-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No live chunk provider/entity movement |
| 462 | `benchmarks/rust-swimming-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No full water physics/player tick |
| 463 | `benchmarks/rust-synced-bytes-2026-10-07/float-regression.json` | Adaptable; Version-specific | Other types/packet codecs/entity schema absent |
| 464 | `benchmarks/rust-synced-bytes-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Other types/schema/wire codecs pending |
| 465 | `benchmarks/rust-synced-floats-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Only float serializer; no protocol/wire schema |
| 466 | `benchmarks/rust-teleport-consume-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Actual teleport/passenger/world events controlled |
| 467 | `benchmarks/rust-terrain-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Material rules/carvers/structures/full blend absent |
| 468 | `benchmarks/rust-ticks-2026-10-05/scoped-parity.json` | Adaptable; Version-specific | No block/fluid tick execution/full scheduler |
| 469 | `benchmarks/rust-underwater-magma-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Chunk updates/postprocessing/full generation absent |
| 470 | `benchmarks/rust-use-flag-reaction-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Other sync keys/packet delivery/full item use absent |
| 471 | `benchmarks/rust-use-remainder-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No full component schema/insertion/drop/consumables |
| 472 | `benchmarks/rust-value-provider-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Gaussian/full height provider/terrain incomplete |
| 473 | `benchmarks/rust-view-direction-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No platform numeric/action integration proof |
| 474 | `benchmarks/rust-vines-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | No growth tick/neighbor/live chunk |
| 475 | `benchmarks/rust-water-splash-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No audio mixer/particle renderer/live movement |

## Part 21 — Entries 476–500

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 476 | `benchmarks/rust-world-2026-10-05/scoped-parity.json` | Unnecessary; Reference; Version-specific | No output hash/reference pair; not implementation evidence |
| 477 | `benchmarks/rust-world-border-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No persistence/listener/damage/network integration |
| 478 | `benchmarks/rust-world-collision-check-2026-10-07/movement-regression.json` | Adaptable; Version-specific | Providers/entity lists/border controlled; no live broad phase |
| 479 | `benchmarks/rust-world-collision-check-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Live entity broad phase/minecart/world lifecycle incomplete |
| 480 | `benchmarks/rust-world-edge-backoff-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Isolated query, not integrated Player physics/world |
| 481 | `benchmarks/rust-world-movement-2026-10-07/geometry-regression.json` | Adaptable; Version-specific | No live World shape selection/integrated movement |
| 482 | `benchmarks/rust-world-movement-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No live provider/full movement tick/input lifecycle |
| 483 | `benchmarks/rust-world-ray-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No live world/chunk provider/complete consumers |
| 484 | `benchmarks/rust-worldgen-2026-10-06/scoped-parity.json` | Adaptable; Reference; Version-specific | Selected codec/query manifests only; no full terrain/chunk/world proof |
| 485 | `benchmarks/rust-worldgen-random-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Gaussian/concurrent/full decoration outside scope |
| 486 | `benchmarks/server-2026-10-17/README.md` | Adaptable; Reference; Version-specific | Bot host overloaded, unequal workloads and vanilla watchdog; not Android client |
| 487 | `benchmarks/server-2026-10-17/earlier-002f14ce/explore-equal-rate.json` | Unnecessary; Reference; Version-specific | Server only, one pass; not A22/client FPS |
| 488 | `benchmarks/server-2026-10-17/earlier-002f14ce/load.txt` | Unnecessary; Reference; Version-specific | Remote bot host high load/ThinkPad variability confound |
| 489 | `benchmarks/server-2026-10-17/earlier-002f14ce/walk-explore.json` | Unnecessary; Reference; Version-specific | Vanilla watchdog/broken pipe at high load; unequal chunks |
| 490 | `benchmarks/server-2026-10-17/explore-equal-rate.json` | Unnecessary; Reference; Version-specific | Server tick/RSS/CPU only |
| 491 | `benchmarks/server-2026-10-17/load.txt` | Unnecessary; Reference; Version-specific | Mac mini bot host often saturated |
| 492 | `benchmarks/server-2026-10-17/walk-explore.json` | Unnecessary; Reference; Version-specific | Vanilla watchdog high counts; remote bots |
| 493 | `benchmarks/server-equal-mixed-2026-10-18/README.md` | Adaptable; Reference; Version-specific | At25 new-chunk rate still differs; vanilla50 watchdog; remote bot host busy |
| 494 | `benchmarks/server-equal-mixed-2026-10-18/load.txt` | Unnecessary; Reference; Version-specific | Bot host CPU 64–95%; ThinkPad CPU 2–100%; host-load variability caveat |
| 495 | `benchmarks/server-equal-mixed-2026-10-18/mixed-equal.json` | Unnecessary; Reference; Version-specific | Horizon 1.77/4.74/11.72/23.82ms; vanilla 11.34/34.3/50.45 then watchdog; 25 work differs |
| 496 | `benchmarks/server-equal-work-2026-10-17/README.md` | Adaptable; Reference; Version-specific | Horizon/Vanilla server method; Vanilla watchdog at 100 bots; no client measurements |
| 497 | `benchmarks/server-equal-work-2026-10-17/explore-equal-rate.json` | Unnecessary; Reference; Version-specific | Both 20TPS; ThinkPad Linux/OpenJDK25 only |
| 498 | `benchmarks/server-equal-work-2026-10-17/horizon.json` | Unnecessary; Reference; Version-specific | At100 walking ~114.5ms MSPT/8.7TPS; no client data |
| 499 | `benchmarks/server-equal-work-2026-10-17/run-1-before-chunk-cache.json` | Unnecessary; Reference; Version-specific | High-load outliers/watchdog; server revision specific |
| 500 | `benchmarks/server-equal-work-2026-10-17/run-2-chunk-cache.json` | Unnecessary; Reference; Version-specific | No matched vanilla baseline |

## Part 22 — Entries 501–525

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 501 | `benchmarks/server-equal-work-2026-10-17/vanilla.json` | Unnecessary; Reference; Version-specific | Watchdog/broken-pipe at high counts |
| 502 | `benchmarks/server-four-way-2026-10-19/README.md` | Unnecessary; Reference; Version-specific | Paper fastest average MSPT in several loads; remote host involved |
| 503 | `benchmarks/server-four-way-2026-10-19/load.txt` | Unnecessary; Reference; Version-specific | Remote bots 80–90% busy; server host spikes |
| 504 | `benchmarks/server-four-way-2026-10-19/report-25-50.json` | Unnecessary; Reference; Version-specific | Paper25 28.3ms vs Horizon30.92; at50 Horizon47.83/Paper46.86/Fabric58.3; vanilla fails |
| 505 | `benchmarks/server-four-way-2026-10-19/report-paper-10.json` | Unnecessary; Reference; Version-specific | One workload slice only |
| 506 | `benchmarks/server-four-way-2026-10-19/runs.jsonl` | Unnecessary; Reference; Version-specific | Server workload data, not client FPS |
| 507 | `benchmarks/server-mixed-2026-10-18/README.md` | Unnecessary; Reference; Version-specific | Explore throughput differs; compare ticks cautiously |
| 508 | `benchmarks/server-mixed-2026-10-18/load.txt` | Unnecessary; Reference; Version-specific | Mac bot host heavily loaded |
| 509 | `benchmarks/server-mixed-2026-10-18/mixed.json` | Unnecessary; Reference; Version-specific | At50 Horizon MSPT higher than vanilla in run; throughput unequal |
| 510 | `benchmarks/server-vs-pumpkin-2026-10-31.json` | Unnecessary; Reference; Version-specific | Pumpkin faster at20, mixed60, Horizon stronger100; anti-cheat/ground disabled |
| 511 | `benchmarks/shared-json-buffer-lifetime-20261017.json` | Unnecessary; Reference; Version-specific | Near-identical medians; negligible gain |
| 512 | `benchmarks/shared-json-decoder-phases-20261017.json` | Unnecessary; Reference; Version-specific | Typed sample faster but higher peak RSS; loader-only |
| 513 | `benchmarks/shared-json-five-way-20261017.json` | Unnecessary; Reference; Version-specific | Loader-only and variant confounds |
| 514 | `benchmarks/shared-json-five-way-final-20261017.json` | Unnecessary; Reference; Version-specific | Lower RSS vs baseline, but host variance and loader-only scope |
| 515 | `benchmarks/shared-json-historical-20261017.json` | Unnecessary; Reference; Version-specific | Host/build context differs; no full startup |
| 516 | `benchmarks/shared-json-startup-20261017.json` | Unnecessary; Reference; Version-specific | Asset loader only; high host load; not full app startup or first-frame timing |
| 517 | `benchmarks/shared-json-startup-followup-20261017.json` | Unnecessary; Reference; Version-specific | Outliers/load variance prevent robust winner |
| 518 | `benchmarks/shared-json-variant-isolation-20261017.json` | Unnecessary; Reference; Version-specific | No meaningful gain; experiments reverted |
| 519 | `benchmarks/tick-state-2026-10-04/live-observations.json` | Unnecessary; Reference; Version-specific | One demo-world observation, not controlled benchmark |
| 520 | `crates/cargo-horizon-mod/Cargo.toml` | Unnecessary; Reference; Version-specific | Mod ecosystem not needed for vanilla client |
| 521 | `crates/cargo-horizon-mod/src/check.rs` | Unnecessary; Reference; Version-specific | Native source unsandboxed and not run by registry check |
| 522 | `crates/cargo-horizon-mod/src/main.rs` | Unnecessary; Reference; Version-specific | Horizon format, not Forge/1.20.1 loader |
| 523 | `crates/cargo-horizon-mod/src/registry.rs` | Unnecessary; Reference; Version-specific | Horizon mod registry; base load error can appear empty |
| 524 | `crates/cargo-horizon-mod/template/content/block.json` | Unnecessary; Reference; Version-specific | Horizon API 26.3-specific |
| 525 | `crates/cargo-horizon-mod/template/content/blockstate.json` | Unnecessary; Reference; Version-specific | Horizon paths/conventions |

## Part 23 — Entries 526–550

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 526 | `crates/cargo-horizon-mod/template/content/horizon-mod.toml` | Unnecessary; Reference; Version-specific | API0.2, not Forge |
| 527 | `crates/cargo-horizon-mod/template/content/item.json` | Unnecessary; Reference; Version-specific | No Forge registration |
| 528 | `crates/cargo-horizon-mod/template/content/lang.json` | Adaptable; Reference; Version-specific | Localization idea generic; ids/resource paths differ |
| 529 | `crates/cargo-horizon-mod/template/content/loot.json` | Adaptable; Reference; Version-specific | 1.20.1 codec validation required |
| 530 | `crates/cargo-horizon-mod/template/content/model.json` | Adaptable; Reference; Version-specific | Model schema/resolution version-dependent |
| 531 | `crates/cargo-horizon-mod/template/content/pickaxe.json` | Adaptable; Reference; Version-specific | Tag path/version must be checked |
| 532 | `crates/cargo-horizon-mod/template/content/recipe.json` | Adaptable; Reference; Version-specific | Recipe codec/IDs need target validation |
| 533 | `crates/cargo-horizon-mod/template/gitignore` | Adaptable; Reference; Version-specific | Generic hygiene but paths upstream-specific |
| 534 | `crates/cargo-horizon-mod/template/native/api.toml` | Unnecessary; Reference; Version-specific | Pinned Horizon API unrelated to Forge |
| 535 | `crates/cargo-horizon-mod/template/native/horizon-mixin.toml` | Unnecessary; Reference; Version-specific | Not Forge/1.20.1 schema |
| 536 | `crates/cargo-horizon-mod/template/native/lib.rs` | Unnecessary; Reference; Version-specific | Unsandboxed Horizon hooks, not Forge |
| 537 | `crates/horizon-android/Cargo.toml` | Adaptable; Reference; Version-specific | Packaging pattern useful but game/package coupling needs rewrite |
| 538 | `crates/horizon-anticheat/Cargo.toml` | Unnecessary; Reference; Version-specific | Server-only, not client runtime |
| 539 | `crates/horizon-anticheat/src/combat.rs` | Unnecessary; Reference; Version-specific | Server validation only |
| 540 | `crates/horizon-anticheat/src/lib.rs` | Unnecessary; Reference; Version-specific | Server moderation policy, not client engine |
| 541 | `crates/horizon-anticheat/src/prediction.rs` | Unnecessary; Reference; Version-specific | Server authority only |
| 542 | `crates/horizon-anticheat/src/timer.rs` | Unnecessary; Reference; Version-specific | Server-only detection |
| 543 | `crates/horizon-assets/Cargo.toml` | Adaptable; Reference; Version-specific | Data/source APIs 26.3-specific |
| 544 | `crates/horizon-assets/examples/blockstate_binding_fixture.rs` | Adaptable; Reference; Version-specific | Harness only; rebuild target registry data |
| 545 | `crates/horizon-assets/examples/blockstate_model_fixture.rs` | Adaptable; Reference; Version-specific | Not runtime/GPU render |
| 546 | `crates/horizon-assets/examples/cuboid_model_fixture.rs` | Adaptable; Reference; Version-specific | No atlas/parents/GPU |
| 547 | `crates/horizon-assets/examples/item_model_fixture.rs` | Adaptable; Reference; Version-specific | 26.3 catalog and CPU fixture; not Android GPU proof |
| 548 | `crates/horizon-assets/examples/model_baking_fixture.rs` | Adaptable; Reference; Version-specific | CPU geometry only |
| 549 | `crates/horizon-assets/examples/model_choice_fixture.rs` | Adaptable; Reference; Version-specific | RNG target semantics need verification |
| 550 | `crates/horizon-assets/examples/model_resolution_fixture.rs` | Adaptable; Reference; Version-specific | No GPU/final rendering |

## Part 24 — Entries 551–575

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 551 | `crates/horizon-assets/examples/sprite_animation_fixture.rs` | Adaptable; Reference; Version-specific | No image/atlas/GPU stage |
| 552 | `crates/horizon-assets/examples/support/mod.rs` | Adaptable; Reference; Version-specific | Test serialization utilities only |
| 553 | `crates/horizon-assets/src/animation.rs` | Adaptable; Version-specific | CPU side; GPU interpolation/atlas absent |
| 554 | `crates/horizon-assets/src/bake.rs` | Adaptable; Version-specific | CPU geometry, no atlas/shader/pixel parity |
| 555 | `crates/horizon-assets/src/bake/transform.rs` | Adaptable; Version-specific | JOML/asset behavior 26.3-specific |
| 556 | `crates/horizon-assets/src/blockstate.rs` | Adaptable; Version-specific | Target JSON/schema semantics need validation |
| 557 | `crates/horizon-assets/src/blockstate/bind.rs` | Adaptable; Version-specific | Invalid/overlap semantics need 1.20.1 oracle |
| 558 | `crates/horizon-assets/src/blockstate/choice.rs` | Adaptable; Version-specific | Random consumption affects appearance; re-oracle |
| 559 | `crates/horizon-assets/src/item_bake.rs` | Adaptable; Version-specific | BTree ordering may differ from Java HashSet; no GPU |
| 560 | `crates/horizon-assets/src/item_model.rs` | Adaptable; Version-specific | All component ids/selectors are 26.3 data |
| 561 | `crates/horizon-assets/src/item_model/component_predicate.rs` | Adaptable; Reference; Version-specific | Only selected families; rebuild component registry |
| 562 | `crates/horizon-assets/src/item_model/foreign.rs` | Unnecessary; Reference; Version-specific | No actual mod hook execution; loader support limited |
| 563 | `crates/horizon-assets/src/item_model/foreign_tests.rs` | Adaptable; Reference; Version-specific | Fallback is not mod runtime/Forge compatibility |
| 564 | `crates/horizon-assets/src/item_model/special.rs` | Adaptable; Reference; Version-specific | Actual renderer elsewhere; 26.3 catalog |
| 565 | `crates/horizon-assets/src/item_model/tests.rs` | Adaptable; Reference; Version-specific | Not renderer acceptance |
| 566 | `crates/horizon-assets/src/json.rs` | Adaptable; Version-specific | 1024-char token cap and parser edges need tests |
| 567 | `crates/horizon-assets/src/lib.rs` | Adaptable; Reference; Version-specific | Module aggregation, not client integration |
| 568 | `crates/horizon-assets/src/math.rs` | Adaptable; Version-specific | f32/platform-sensitive; ARM64/Java oracle required |
| 569 | `crates/horizon-assets/src/model.rs` | Adaptable; Version-specific | NeoForge OBJ/composite and 26.3 data need review |
| 570 | `crates/horizon-assets/src/obj.rs` | Unnecessary; Reference; Optional/future | Optional extension/MTL not base vanilla requirement |
| 571 | `crates/horizon-assets/src/palette.rs` | Adaptable; Version-specific | CPU utility only; no texture upload |
| 572 | `crates/horizon-assets/src/resolve.rs` | Adaptable; Version-specific | No final GPU parity; 26.3 resource rules |
| 573 | `crates/horizon-bench/Cargo.toml` | Adaptable; Reference; Version-specific | Tooling, not gameplay |
| 574 | `crates/horizon-bench/src/hardware.rs` | Adaptable; Reference; Version-specific | Android needs native APIs |
| 575 | `crates/horizon-bench/src/lib.rs` | Adaptable; Reference; Version-specific | Upstream-specific schema; tests format logic only |

## Part 25 — Entries 576–600

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 576 | `crates/horizon-bots/Cargo.toml` | Unnecessary; Reference; Version-specific | Not playable Android client |
| 577 | `crates/horizon-bots/src/main.rs` | Unnecessary; Reference; Version-specific | Protocol26.3 and synthetic flight; not faithful human client |
| 578 | `crates/horizon-chat-filter/Cargo.toml` | Unnecessary; Reference; Version-specific | Not client/render dependency |
| 579 | `crates/horizon-chat-filter/data/slurs.txt` | Unnecessary; Reference; Version-specific | Not needed in client; false-positive policy concern |
| 580 | `crates/horizon-chat-filter/src/lib.rs` | Unnecessary; Reference; Version-specific | Server moderation policy, not client chat rendering |
| 581 | `crates/horizon-chat-filter/src/normalise.rs` | Unnecessary; Reference; Version-specific | Language-dependent; can false-positive |
| 582 | `crates/horizon-chat-filter/src/tests.rs` | Unnecessary; Reference; Version-specific | One server policy, not general language semantics |
| 583 | `crates/horizon-client-core/src/block_entity_nbt.rs` | Adaptable; Version-specific | Includes 26.3-only block entities and fields; exact IDs/defaults must be rebuilt from 1.20.1. |
| 584 | `crates/horizon-client-core/src/bossbar.rs` | Adaptable; Version-specific | No packet synchronization or HUD rendering; color/catalog details are 26.3-specific. |
| 585 | `crates/horizon-client-core/src/component.rs` | Adaptable; Version-specific | Later component forms differ from 1.20.1; not a full chat screen/renderer. |
| 586 | `crates/horizon-client-core/src/fastutil_order.rs` | Adaptable; Reference; Version-specific | JDK collection internals are implementation details; do not port blindly unless 1.20.1 oracle shows observable behavior. |
| 587 | `crates/horizon-client-core/src/gamerules.rs` | Adaptable; Version-specific | 26.3 rule names/defaults differ; rebuild full 1.20.1 catalog. |
| 588 | `crates/horizon-client-core/src/identifier.rs` | Adaptable; Version-specific | The helpers differ in strictness; compare each with exact 1.20.1 Identifier constructors/codecs. |
| 589 | `crates/horizon-client-core/src/key_display.rs` | Adaptable; Version-specific | SDL/US symbol tables are upstream choices; Android hardware/IME input handling needs native adaptation. |
| 590 | `crates/horizon-client-core/src/lang.rs` | Adaptable; Version-specific | The table is 26.3; load target 1.20.1 language assets and key names. |
| 591 | `crates/horizon-client-core/src/nbt_path.rs` | Adaptable; Version-specific | Path parser, array mutation and max-depth details need exact 1.20.1 Java oracle. |
| 592 | `crates/horizon-client-core/src/nbt_value.rs` | Adaptable; Version-specific | Truncation, float/double conversion and wrong-type fallbacks are version-sensitive. |
| 593 | `crates/horizon-client-core/src/particle_type.rs` | Adaptable; Version-specific | Includes many later particle IDs; regenerate 1.20.1 registry and rendering layers. |
| 594 | `crates/horizon-client-core/src/pattern.rs` | Adaptable; Version-specific | Groups/braces are unsupported; must preserve only verified subset semantics. |
| 595 | `crates/horizon-client-core/src/potions.rs` | Adaptable; Version-specific | Contains later effects and mod registration; regenerate all 1.20.1 potion/effect data. |
| 596 | `crates/horizon-client-core/src/registries.rs` | Adaptable; Version-specific | Server-provided registry state and 26.3 resource sets are version-specific; completion UI and packet lifecycle are external. |
| 597 | `crates/horizon-client-core/src/scoreboard.rs` | Adaptable; Version-specific | Criteria/stat catalog is 26.3; no full network packet synchronization or HUD integration. |
| 598 | `crates/horizon-client-core/src/snbt.rs` | Adaptable; Version-specific | Hash iteration order is heuristic/insertion-based; exact formatting requires a pinned Java 1.20.1 oracle. |
| 599 | `crates/horizon-client-core/src/sound_source.rs` | Adaptable; Version-specific | Small static table; verify target ordering and server/client use sites rather than importing blindly. |
| 600 | `crates/horizon-client-core/src/text.rs` | Adaptable; Version-specific | 823 logical lines; newer event/object fields; preserve local-only open_file trust boundary |

## Part 26 — Entries 601–620

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 601 | `crates/horizon-client-core/src/uuid.rs` | adaptable; reference; version-specific | Parser accepts any hyphen placement if 32 hex digits remain; compare exact Java 1.20.1 behavior. |
| 602 | `crates/horizon-client-core/src/who.rs` | adaptable; reference; version-specific | Mob runtime IDs are scoped/reusable; not persistent identity. |
| 603 | `crates/horizon-client-render/examples/compositor.rs` | adaptable; reference; optional/future | Desktop wgpu/winit example only; no Android surface lifecycle or device validation. |
| 604 | `crates/horizon-client-render/examples/pack_load_bench.rs` | adaptable; reference; version-specific | Synthetic packs and test ZIP shortcuts do not represent real 1.20.1 resource packs. |
| 605 | `crates/horizon-client-render/src/android_controls.rs` | adaptable; optional/future; version-specific | Horizon-specific touch behaviors; rebind item actions and validate multi-touch on Android. |
| 606 | `crates/horizon-client-render/src/android_controls/custom.rs` | adaptable; optional/future; version-specific | GoyControl/MCPE modes are product-specific; verify pointer/key ownership and release paths. |
| 607 | `crates/horizon-client-render/src/android_controls/editor.rs` | adaptable; optional/future; version-specific | US/ASCII key capture and save/error behavior need international keyboard and persistence tests. |
| 608 | `crates/horizon-client-render/src/android_controls/layout.rs` | adaptable; optional/future; version-specific | Fourteen-slot profile schema and presets are Horizon-specific; create a RustChunk-owned versioned schema. |
| 609 | `crates/horizon-client-render/src/assets.rs` | adaptable; version-specific | Pinned 26.3 registry, tags, facts and assets cannot be mixed with 1.20.1. |
| 610 | `crates/horizon-client-render/src/atlas_sources.rs` | adaptable; version-specific | Verify each sprite source codec against 1.20.1 and harden malformed image/path inputs. |
| 611 | `crates/horizon-client-render/src/audio.rs` | adaptable; optional/future | Large OpenAL-style replacement needs 1.20.1 sound semantics and real Android output tests. |
| 612 | `crates/horizon-client-render/src/auth.rs` | rewrite required; version-specific | Handwritten timestamp parser accepts impossible calendar dates; verify 1.20.1 services and token handling. |
| 613 | `crates/horizon-client-render/src/ban_notice.rs` | adaptable; version-specific | Moderation JSON and translation keys must match 1.20.1; fail-closed defaults need explicit tests. |
| 614 | `crates/horizon-client-render/src/bidi.rs` | adaptable; version-specific | Arabic/Persian shaping, Unicode data and styled mixed-direction text need Java 1.20.1 golden tests. |
| 615 | `crates/horizon-client-render/src/biome.rs` | adaptable; version-specific | Biome IDs/attributes are 26.3-specific; parser casts/count ranges and empty table need hardening. |
| 616 | `crates/horizon-client-render/src/block_edit.rs` | adaptable; optional/future; version-specific | It bypasses normal block-entity, neighbor, path and light side effects; callers must explicitly handle them. |
| 617 | `crates/horizon-client-render/src/block_entities.rs` | adaptable; version-specific | Reproduce 1.20.1 ticker promotion/order and chunk-load lifecycle; fastutil order is observable. |
| 618 | `crates/horizon-client-render/src/block_entity_anim.rs` | adaptable; version-specific | Rebuild 1.20.1 ticker catalog and client/server event behavior; later entity states are included. |
| 619 | `crates/horizon-client-render/src/block_entity_cem.rs` | unnecessary; optional/future | Not vanilla core; model catalog and supported parts must be scoped to 1.20.1. |
| 620 | `crates/horizon-client-render/src/block_entity_render.rs` | adaptable; version-specific | Model states/assets are 26.3; separate dynamic render, static cache and target-version entity model data. |
## Part 27 — Entries 621–800

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 621 | `crates/horizon-client-render/src/block_entity_render_tests.rs` | adaptable; reference; version-specific | Exercises vertex parity, cache invalidation, resource reloads and fallback paths for special block-entity models. |
| 622 | `crates/horizon-client-render/src/bluemap.rs` | unnecessary; optional/future | Fetches and displays remote world-map tiles and metadata alongside the in-game world. |
| 623 | `crates/horizon-client-render/src/bluemap_detect.rs` | unnecessary; optional/future | Detects a map endpoint from server-provided or discovered information before map tile loading. |
| 624 | `crates/horizon-client-render/src/border.wgsl` | adaptable; version-specific | Computes the border overlay fragment colour/fade in the GPU shader. |
| 625 | `crates/horizon-client-render/src/border_render.rs` | adaptable; version-specific | Builds border wall/plane geometry and animation parameters for the world-border pass. |
| 626 | `crates/horizon-client-render/src/breaking.rs` | adaptable; version-specific | Builds destroy-stage UVs and geometry for crack overlays on models/entities. |
| 627 | `crates/horizon-client-render/src/brewing.rs` | rewrite required; version-specific | Resolves brewing operations, ingredient matches and brewing time from data-driven definitions/providers. |
| 628 | `crates/horizon-client-render/src/camera.rs` | adaptable; version-specific | Maintains camera pose, projection, third-person clipping, view bob, hurt and interpolation state. |
| 629 | `crates/horizon-client-render/src/chat.rs` | adaptable; version-specific | Coordinates chat history, draft/composition, message display, completion and signing-related UI state. |
| 630 | `crates/horizon-client-render/src/chat_tests.rs` | adaptable; reference; version-specific | Checks chat wrapping, ordering, history, keyboard interactions and related rendering behavior. |
| 631 | `crates/horizon-client/examples/probe/pick_block_data_fixture.rs` | adaptable; reference; version-specific | Block-entity pick/place oracle fixture; item-component representation is newer than 1.20.1. |
| 632 | `crates/horizon-client/examples/probe/place_recipe_fixture.rs` | adaptable; reference; version-specific | Recipe-placement fixture; registry and item-stack formats need 1.20.1 parity. |
| 633 | `crates/horizon-client/examples/probe/player_sync_fixture.rs` | adaptable; reference; version-specific | Movement packet and teleport-velocity oracle; packet IDs/layout are protocol-version-specific. |
| 634 | `crates/horizon-client-render/src/cit.rs` | unnecessary; optional/future; version-specific | This is optional OptiFine-compatible resource-pack behavior, not vanilla rendering. Rule semantics depend on the separate `horizon_optifine` parser and 26.3-era item-component model; th |
| 635 | `crates/horizon-client-render/src/cit_armor.rs` | unnecessary; optional/future; version-specific | The source explicitly maps 26.3 equipment asset names back to pre-1.21.2 texture names. That compatibility mapping is not itself the Java 1.20.1 armor resource schema; this remains an o |
| 636 | `crates/horizon-client-render/src/cit_assets.rs` | unnecessary; optional/future; version-specific | This is a resource-pack compatibility adapter for files vanilla does not normally enumerate. Its reserved namespace and path rebasing must be reviewed as security boundaries; the implem |
| 637 | `crates/horizon-client-render/src/cit_glint.rs` | unnecessary; optional/future; version-specific | Builds shared texture arrays and per-layer blend parameters for custom glint effects. |
| 638 | `crates/horizon-client-render/src/clouds.rs` | adaptable; version-specific | Builds cloud geometry and updates position/phase via uniforms while remaining inside a geometry cell. |
| 639 | `crates/horizon-client-render/src/collisions.rs` | adaptable; version-specific | Uses chunk-section palettes and compact shape data to avoid scanning blocks that cannot collide. |
| 640 | `crates/horizon-client-render/src/command_block.rs` | adaptable; optional/future; version-specific | Provides command-block text/metadata and UI-facing state used by command-block screens. |
| 641 | `crates/horizon-client-render/src/connected.rs` | unnecessary; optional/future | Implements OptiFine-style connected textures, predicates and neighbour-sensitive sprite selection. |
| 642 | `crates/horizon-client-render/src/containers.rs` | adaptable; version-specific | Builds container content, slot layout and specialized chest/furnace-like screen states. |
| 643 | `crates/horizon-client-render/src/crash.rs` | adaptable; reference | Collects crash context and scrubs sensitive/player/server details before presenting or saving reports. |
| 644 | `crates/horizon-client-render/src/datapack.rs` | adaptable; version-specific | Reads datapack metadata/resources and manages reloadable data inputs. |
| 645 | `crates/horizon-client-render/src/dynamic_light.rs` | unnecessary; optional/future | Adds nearby light-source values to shader-side visual shading without changing authoritative world light. |
| 646 | `crates/horizon-client-render/src/effects.rs` | adaptable; version-specific | Stores active effects, durations, modifiers and display-facing state. |
| 647 | `crates/horizon-client-render/src/elyby.rs` | unnecessary; optional/future | Routes account/session behavior through an additional third-party identity provider. |
| 648 | `crates/horizon-client-render/src/entity.wgsl` | adaptable; version-specific | Implements entity texture, lighting, overlay, fog and special-layer rendering. |
| 649 | `crates/horizon-client-render/src/environment.rs` | adaptable; version-specific | Resolves data-driven environmental attributes such as fog, sky, ambience and related visual/gameplay values. |
| 650 | `crates/horizon-client-render/src/extras.rs` | unnecessary; optional/future | Keeps extra renderer/mobile settings outside vanilla options.txt. |
| 651 | `crates/horizon-client-render/src/fast_hash.rs` | adaptable; reference | Centralizes a fast non-cryptographic hasher choice for internal maps/sets. |
| 652 | `crates/horizon-client-render/src/fog.rs` | adaptable; version-specific | Combines fog type, distances, colours, water/lava effects and render-time parameters. |
| 653 | `crates/horizon-client-render/src/font.rs` | adaptable; version-specific | Loads bitmap, Unihex and TTF font providers, glyph metrics and atlas data. |
| 654 | `crates/horizon-client-render/src/fusion.rs` | unnecessary; optional/future | Parses a Fusion-specific model/texture rule and applies connected model conditions. |
| 655 | `crates/horizon-client-render/src/gfx.rs` | adaptable; version-specific | Owns surface/device lifecycle, render passes, terrain/entities/HUD, GPU buffers, shaders, caches and presentation. |
| 656 | `crates/horizon-client-render/src/gfx/adreno.rs` | adaptable; reference; optional/future | Selects driver-specific GPU workarounds for Qualcomm Adreno variants. |
| 657 | `crates/horizon-client-render/src/gfx/animate.rs` | adaptable; version-specific | Builds frame chains and animation state for atlas sprites, including mip levels. |
| 658 | `crates/horizon-client-render/src/gfx/animate.wgsl` | adaptable; version-specific | Runs the GPU part of sprite-frame update and texture copy work. |
| 659 | `crates/horizon-client-render/src/gfx/arena.rs` | adaptable; version-specific | Allocates paged GPU memory, free lists, compaction and spill/reuse for meshes. |
| 660 | `crates/horizon-client-render/src/gfx/block_light.rs` | unnecessary; optional/future | Provides material/shader-pack access to block light values for custom material shading. |
| 661 | `crates/horizon-client-render/src/gfx/compositor.rs` | adaptable; reference; optional/future | Composes multiple offscreen views into one surface. |
| 662 | `crates/horizon-client-render/src/gfx/crumbling_tests.rs` | adaptable; reference; version-specific | Checks destroy overlay and GPU rendering regressions. |
| 663 | `crates/horizon-client-render/src/gfx/cull.rs` | adaptable; version-specific | Reads occlusion results asynchronously and uses prior-frame visibility to skip hidden sections. |
| 664 | `crates/horizon-client-render/src/gfx/cull.wgsl` | adaptable; version-specific | Evaluates occlusion/query results for section visibility. |
| 665 | `crates/horizon-client-render/src/gfx/end_sky.rs` | adaptable; version-specific | Builds and retains End sky cube geometry and atlas bindings. |
| 666 | `crates/horizon-client-render/src/gfx/end_sky/tests.rs` | adaptable; version-specific | Compares resident sky geometry and rendered pixels against expected behavior. |
| 667 | `crates/horizon-client-render/src/gfx/entity_batch.rs` | adaptable; version-specific | Groups entity draw runs by material/overlay to reduce pipeline switches while preserving semantics. |
| 668 | `crates/horizon-client-render/src/gfx/entity_outline.rs` | adaptable; version-specific | Builds glowing entity outline masks and compositing. |
| 669 | `crates/horizon-client-render/src/gfx/entity_outline.wgsl` | adaptable; version-specific | Computes mask/edge colour for glowing entity outlines. |
| 670 | `crates/horizon-client-render/src/gfx/entity_skin.rs` | adaptable; optional/future; version-specific | Transforms entity model vertices on GPU and retains a CPU expansion fallback. |
| 671 | `crates/horizon-client-render/src/gfx/entity_skin.wgsl` | adaptable; optional/future; version-specific | Computes skinned entity vertex positions and normals. |
| 672 | `crates/horizon-client-render/src/gfx/entity_source.rs` | adaptable; reference; optional/future | Lowers or rewrites generated shader source for problematic older Adreno GLES drivers. |
| 673 | `crates/horizon-client-render/src/gfx/flat.rs` | adaptable; optional/future; version-specific | Flattens or batches section draw instances to reduce per-section draw overhead. |
| 674 | `crates/horizon-client-render/src/gfx/flatten.wgsl` | adaptable; optional/future; version-specific | Generates flattened draw indices from draw ranges and page tables. |
| 675 | `crates/horizon-client-render/src/gfx/frame_gen.rs` | unnecessary; optional/future; version-specific | Coordinates offscreen real frames, interpolation, presentation thread and camera/hand motion. |
| 676 | `crates/horizon-client-render/src/gfx/frame_gen/interpolation.rs` | unnecessary; optional/future; version-specific | Interpolates consecutive images using depth, motion vectors and optical flow. |
| 677 | `crates/horizon-client-render/src/gfx/frame_gen/interpolation.wgsl` | unnecessary; optional/future; version-specific | Implements compute passes for depth dilation, vector fields, blending and inpainting. |
| 678 | `crates/horizon-client-render/src/gfx/frame_gen/motion.rs` | unnecessary; optional/future; version-specific | Tracks independently moving entities by ID and rasterizes motion vectors. |
| 679 | `crates/horizon-client-render/src/gfx/frame_gen/motion.wgsl` | unnecessary; optional/future; version-specific | Renders per-entity motion vectors while respecting depth and transparent texels. |
| 680 | `crates/horizon-client-render/src/gfx/frame_gen/optical_flow.rs` | unnecessary; optional/future; version-specific | Implements multilevel luma pyramids, block matching and scene-change detection. |
| 681 | `crates/horizon-client-render/src/gfx/frame_gen/optical_flow.wgsl` | unnecessary; optional/future; version-specific | Computes luma, histograms, multilevel searches and block vectors. |
| 682 | `crates/horizon-client-render/src/gfx/hdr.rs` | unnecessary; optional/future | Selects HDR10/scRGB surfaces, controls paper white/peak and maps world/HUD to HDR output. |
| 683 | `crates/horizon-client-render/src/gfx/hdr_icc.rs` | unnecessary; optional/future | Constructs a BT.2020/PQ profile for HDR screenshot interoperability. |
| 684 | `crates/horizon-client-render/src/gfx/hdr_stats.rs` | adaptable; reference; optional/future | Measures sprite brightness on GPU for adaptive HDR highlight selection. |
| 685 | `crates/horizon-client-render/src/gfx/inter_stage_tests.rs` | adaptable; reference; version-specific | Audits WGSL vertex/fragment interface variables against a downlevel GPU limit. |
| 686 | `crates/horizon-client-render/src/gfx/late_sky.rs` | adaptable; optional/future; version-specific | Draws sky after solid terrain using far-plane depth to avoid shading covered pixels. |
| 687 | `crates/horizon-client-render/src/gfx/late_sky/tests.rs` | adaptable; reference; version-specific | Checks late-sky shader output pixel-for-pixel against the ordinary draw order. |
| 688 | `crates/horizon-client-render/src/gfx/lightning.rs` | adaptable; optional/future | Retains deterministic lightning bolt geometry by seed and reuses it while active. |
| 689 | `crates/horizon-client-render/src/gfx/lightning/tests/mod.rs` | adaptable; reference; optional/future | Compares retained bolt geometry, atlas sampling and GPU draw output with a pinned reference. |
| 690 | `crates/horizon-client-render/src/gfx/lod.rs` | adaptable; optional/future; version-specific | Uploads/fades distant tile meshes and handles replacement by nearer tiles. |
| 691 | `crates/horizon-client-render/src/gfx/native_material.rs` | unnecessary; optional/future | Injects a custom material shader stage into terrain shading. |
| 692 | `crates/horizon-client-render/src/gfx/native_pack.rs` | unnecessary; optional/future | Holds custom stages, settings and timers for Horizon's own shader-pack system. |
| 693 | `crates/horizon-client-render/src/gfx/native_post.rs` | unnecessary; optional/future | Runs arbitrary multi-pass pack post effects with custom targets, bindings and history. |
| 694 | `crates/horizon-client-render/src/gfx/native_render.rs` | unnecessary; optional/future | Provides hook points for native mods to draw at chosen frame phases. |
| 695 | `crates/horizon-client-render/src/gfx/native_render/generated.rs` | unnecessary; optional/future | Defines event structs and ordered dispatch for native renderer hooks. |
| 696 | `crates/horizon-client-render/src/gfx/native_shadow.rs` | unnecessary; optional/future | Renders depth/colour shadow maps for custom shader packs. |
| 697 | `crates/horizon-client-render/src/gfx/native_sky.rs` | unnecessary; optional/future | Allows custom sky shader stage over renderer globals. |
| 698 | `crates/horizon-client-render/src/gfx/occlusion.rs` | adaptable; version-specific | Searches visible section connectivity from camera via open faces on a background worker, preserving last completed graph for frames. |
| 699 | `crates/horizon-client-render/src/gfx/oit.rs` | unnecessary; optional/future | Uses multi-stage depth/transmittance accumulation and final composite for translucent surfaces. |
| 700 | `crates/horizon-client-render/src/gfx/oit_features.rs` | unnecessary; optional/future | Applies OIT stages to particles, weather, border, text and other non-terrain draw categories. |
| 701 | `crates/horizon-client-render/src/gfx/pack_atlas.rs` | unnecessary; reference; optional/future | Optional shader-pack compatibility, not vanilla rendering. |
| 702 | `crates/horizon-client-render/src/gfx/pack_center_depth.rs` | unnecessary; reference; optional/future | Not needed for vanilla gameplay/rendering. |
| 703 | `crates/horizon-client-render/src/gfx/pack_draw.rs` | unnecessary; reference; optional/future | Only useful if a separate shader-pack API is built. |
| 704 | `crates/horizon-client-render/src/gfx/pack_entities.rs` | unnecessary; reference; optional/future | Do not make pack stages a base renderer dependency. |
| 705 | `crates/horizon-client-render/src/gfx/pack_hand.rs` | unnecessary; reference; optional/future | Optional pack feature only. |
| 706 | `crates/horizon-client-render/src/gfx/pack_images.rs` | unnecessary; reference; optional/future | Avoid large untrusted allocations in the base client. |
| 707 | `crates/horizon-client-render/src/gfx/pack_post.rs` | unnecessary; reference; optional/future | Defer until vanilla frame graph and resource limits are mature. |
| 708 | `crates/horizon-client-render/src/gfx/pack_post.wgsl` | unnecessary; reference; optional/future | Optional pack helper, not a base shader. |
| 709 | `crates/horizon-client-render/src/gfx/pack_shadow.rs` | unnecessary; reference; optional/future | Not required for vanilla rendering. |
| 710 | `crates/horizon-client-render/src/gfx/pack_sky.rs` | unnecessary; reference; optional/future | Keep separate from vanilla sky rendering. |
| 711 | `crates/horizon-client-render/src/gfx/pack_textures.rs` | unnecessary; reference; optional/future | Optional shader-pack texture compatibility only. |
| 712 | `crates/horizon-client-render/src/gfx/pack_timing.rs` | adaptable; reference; optional/future | Useful later for GPU profiling, not the initial renderer. |
| 713 | `crates/horizon-client-render/src/gfx/pack_uniforms.rs` | unnecessary; reference; optional/future | Do not import newer pack fields into vanilla state. |
| 714 | `crates/horizon-client-render/src/gfx/pack_weather.rs` | unnecessary; reference; optional/future | Vanilla weather has its own renderer. |
| 715 | `crates/horizon-client-render/src/gfx/page_runs.rs` | adaptable; version-specific | Candidate chunk-render optimization after parity. |
| 716 | `crates/horizon-client-render/src/gfx/panorama.rs` | adaptable; version-specific | Useful later for a polished title screen. |
| 717 | `crates/horizon-client-render/src/gfx/panorama.wgsl` | adaptable; version-specific | Only useful with the title panorama feature. |
| 718 | `crates/horizon-client-render/src/gfx/readback.rs` | adaptable; version-specific | Useful portability foundation for capture. |
| 719 | `crates/horizon-client-render/src/gfx/resident.rs` | essential; version-specific | Core renderer architecture for loaded chunks. |
| 720 | `crates/horizon-client-render/src/gfx/reuse.rs` | adaptable; version-specific | Useful renderer lifecycle optimization. |
| 721 | `crates/horizon-client-render/src/gfx/solid_paths.rs` | unnecessary; reference; optional/future | Do not apply to Mali based on Adreno measurements. |
| 722 | `crates/horizon-client-render/src/gfx/solid_paths/tests.rs` | unnecessary; reference; version-specific | Test infrastructure only. |
| 723 | `crates/horizon-client-render/src/gfx/stars.rs` | adaptable; version-specific | Useful sky-renderer optimization. |
| 724 | `crates/horizon-client-render/src/gfx/stars.wgsl` | adaptable; version-specific | Useful with verified target star geometry. |
| 725 | `crates/horizon-client-render/src/gfx/submit.rs` | adaptable; reference; optional/future | Experimental until exact-device profiling. |
| 726 | `crates/horizon-client-render/src/gfx/sunrise_fan.rs` | adaptable; version-specific | Useful sky optimization after parity. |
| 727 | `crates/horizon-client-render/src/gfx/terrain_gles.wgsl.inc` | adaptable; version-specific | Useful only if GLES is selected, with Mali validation. |
| 728 | `crates/horizon-client-render/src/gfx/terrain_source.rs` | adaptable; version-specific | Potential portability layer if GLES is chosen. |
| 729 | `crates/horizon-client-render/src/gfx/test_gpu.rs` | unnecessary; reference; version-specific | Test helper only. |
| 730 | `crates/horizon-client-render/src/gfx/timing.rs` | adaptable; reference; optional/future | Useful optional diagnostics. |
| 731 | `crates/horizon-client-render/src/gfx/translucency.rs` | essential; version-specific | Core transparent terrain behavior. |
| 732 | `crates/horizon-client-render/src/gfx/upload.rs` | adaptable; version-specific | Useful upload optimization after baseline transfers. |
| 733 | `crates/horizon-client-render/src/gfx/view.rs` | adaptable; reference; optional/future | Single-view separation useful, stereo deferred. |
| 734 | `crates/horizon-client-render/src/gfx/weather.rs` | essential; version-specific | Core client visuals; adapt vanilla weather. |
| 735 | `crates/horizon-client-render/src/gfx/wgsl_prefix.rs` | unnecessary; reference; optional/future | Only needed with pack composition. |
| 736 | `crates/horizon-client-render/src/gfx/world_post.rs` | adaptable; optional/future | Defer until base renderer and profiling. |
| 737 | `crates/horizon-client-render/src/graphics.rs` | adaptable; version-specific | Use separation pattern, not Horizon catalogue. |
| 738 | `crates/horizon-client-render/src/graphics_style.rs` | unnecessary; reference; optional/future | Visual reference only; RustChunk design remains independent. |
| 739 | `crates/horizon-client-render/src/graphics_ui.rs` | adaptable; version-specific | Port only after RustChunk's settings/input model exists. |
| 740 | `crates/horizon-client-render/src/hdr.wgsl` | unnecessary; reference; optional/future | Optional display feature. |
| 741 | `crates/horizon-client-render/src/hdr_calibration.rs` | unnecessary; reference; optional/future | Defer with HDR. |
| 742 | `crates/horizon-client-render/src/hdr_stats.wgsl` | unnecessary; reference; optional/future | Not needed for SDR. |
| 743 | `crates/horizon-client-render/src/hud.rs` | essential; version-specific | Core HUD renderer; rebuild assets/budget for 1.20.1. |
| 744 | `crates/horizon-client-render/src/hud.wgsl` | essential; version-specific | Core GUI shader path. |
| 745 | `crates/horizon-client-render/src/hud/cit_glint.rs` | unnecessary; reference; optional/future | Defer optional pack feature. |
| 746 | `crates/horizon-client-render/src/hud/pack_text.rs` | unnecessary; reference; optional/future | Defer pack shader support. |
| 747 | `crates/horizon-client-render/src/hud_layout.rs` | essential; version-specific | Core client HUD. |
| 748 | `crates/horizon-client-render/src/hud_overlays.rs` | essential; version-specific | Core gameplay UI. |
| 749 | `crates/horizon-client-render/src/inventory_effects.rs` | adaptable; version-specific | Secondary inventory feature. |
| 750 | `crates/horizon-client-render/src/java_text.rs` | essential; version-specific | Important for options/config parity. |
| 751 | `crates/horizon-client-render/src/jigsaw_block.rs` | adaptable; version-specific | Implement when Jigsaw support is added. |
| 752 | `crates/horizon-client-render/src/keypad.rs` | adaptable; reference; optional/future | Use as reference only; design editor/icons independently. |
| 753 | `crates/horizon-client-render/src/level_loading.rs` | adaptable; version-specific | Useful after chunk streaming/login exists. |
| 754 | `crates/horizon-client/examples/probe/raid_fixture.rs` | unnecessary; reference; version-specific | Parity fixture pattern only. |
| 755 | `crates/horizon-client-render/src/light.rs` | essential; version-specific | Core world subsystem. |
| 756 | `crates/horizon-client-render/src/lod.rs` | adaptable; version-specific | High-value far-view architecture after near chunks work. |
| 757 | `crates/horizon-client-render/src/lod/cache.rs` | adaptable; optional/future | Add after LOD is stable. |
| 758 | `crates/horizon-client-render/src/lod/mesh.rs` | adaptable; version-specific | Adopt only with LOD system. |
| 759 | `crates/horizon-client-render/src/logging.rs` | adaptable; version-specific | Adapt to app-private storage. |
| 760 | `crates/horizon-client-render/src/menu_blur.rs` | unnecessary; reference; optional/future | Optional visual feature. |
| 761 | `crates/horizon-client-render/src/menu_blur.wgsl` | unnecessary; reference; optional/future | Only with menu blur. |
| 762 | `crates/horizon-client/examples/probe/random_tick_sections_probe.rs` | unnecessary; reference; version-specific | Test reference only. |
| 763 | `crates/horizon-client-render/src/mesher/fluid.rs` | essential; version-specific | Core mesher. |
| 764 | `crates/horizon-client-render/src/mesher/leaf_culling.rs` | adaptable; optional/future | Optional optimization after baseline meshing. |
| 765 | `crates/horizon-client-render/src/mesher/leaf_culling_tests.rs` | unnecessary; reference; version-specific | Test suite reference. |
| 766 | `crates/horizon-client/examples/probe/recipe_book_fixture.rs` | unnecessary; reference; version-specific | Fixture design only. |
| 767 | `crates/horizon-client-render/src/mesher/particle_tests.rs` | unnecessary; reference; version-specific | Test reference only. |
| 768 | `crates/horizon-client-render/src/mesher/reuse_tests.rs` | unnecessary; reference; version-specific | Reference suite. |
| 769 | `crates/horizon-client-render/src/mesher/sides.rs` | adaptable; version-specific | Potential mesh optimization. |
| 770 | `crates/horizon-client-render/src/mesher/tint_tests.rs` | unnecessary; reference; version-specific | Test reference only. |
| 771 | `crates/horizon-client-render/src/mesher/visibility_tests.rs` | unnecessary; reference; version-specific | Test pattern only. |
| 772 | `crates/horizon-client-render/src/minimap.rs` | unnecessary; reference; optional/future | Optional UX feature. |
| 773 | `crates/horizon-client-render/src/mipmap.rs` | adaptable; version-specific | Useful asset-pipeline algorithm. |
| 774 | `crates/horizon-client-render/src/mod_list.rs` | unnecessary; reference; optional/future | Must not be mistaken for Forge support. |
| 775 | `crates/horizon-client-render/src/model_library.rs` | essential; version-specific | Core asset pipeline. |
| 776 | `crates/horizon-client-render/src/models.rs` | unnecessary; reference; optional/future | Defer from base client. |
| 777 | `crates/horizon-client-render/src/models/catalog.rs` | essential; version-specific | Core model/mesher subsystem. |
| 778 | `crates/horizon-client-render/src/models/cube.rs` | adaptable; version-specific | Optional mesher optimization. |
| 779 | `crates/horizon-client-render/src/multiline_edit.rs` | adaptable; version-specific | Reusable UI widget after IME integration. |
| 780 | `crates/horizon-client-render/src/native_emissive.rs` | unnecessary; reference; optional/future | Defer optional pack/mod feature. |
| 781 | `crates/horizon-client-render/src/offline_name.rs` | adaptable; version-specific | Relevant to offline profile UX with auth separation. |
| 782 | `crates/horizon-client-render/src/oit.wgsl` | unnecessary; reference; optional/future | Defer experimental GPU feature. |
| 783 | `crates/horizon-client-render/src/oit_border.wgsl` | unnecessary; reference; optional/future | Only if OIT is adopted. |
| 784 | `crates/horizon-client-render/src/oit_clouds.wgsl` | unnecessary; reference; optional/future | Optional OIT only. |
| 785 | `crates/horizon-client-render/src/oit_features.wgsl` | unnecessary; reference; optional/future | Optional OIT only. |
| 786 | `crates/horizon-client-render/src/oit_world.wgsl` | unnecessary; reference; optional/future | Keep sorted transparency as baseline. |
| 787 | `crates/horizon-client-render/src/options_file.rs` | essential; version-specific | Core options persistence rebuilt for 1.20.1. |
| 788 | `crates/horizon-client-render/src/options_fixes.rs` | adaptable; version-specific | Regenerate migration chain through 1.20.1. |
| 789 | `crates/horizon-client-render/src/options_ui.rs` | adaptable; version-specific | Port screens incrementally after GUI/input/settings foundation. |
| 790 | `crates/horizon-client-render/src/overlay.rs` | adaptable; version-specific | Useful renderer parity helper. |
| 791 | `crates/horizon-client-render/src/own_buttons.rs` | adaptable; reference; optional/future | Use as inspiration only; build RustChunk editor/icons independently. |
| 792 | `crates/horizon-client-render/src/own_skin.rs` | adaptable; optional/future | Optional offline profile feature. |
| 793 | `crates/horizon-client-render/src/pack_screen.rs` | adaptable; version-specific | Port after resource-pack backend exists. |
| 794 | `crates/horizon-client-render/src/packs.rs` | essential; version-specific | Core asset pipeline feature. |
| 795 | `crates/horizon-client-render/src/packs/cache.rs` | adaptable; optional/future | Add after uncached composition is correct. |
| 796 | `crates/horizon-client-render/src/par.rs` | adaptable; reference; optional/future | Use only if startup profiling proves it useful. |
| 797 | `crates/horizon-client-render/src/particles.rs` | essential; version-specific | Core client visual subsystem. |
| 798 | `crates/horizon-client-render/src/poi_sections.rs` | adaptable; version-specific | Useful world acceleration when POI gameplay is added. |
| 799 | `crates/horizon-client-render/src/post_chain.rs` | adaptable; version-specific | Add after GPU pipeline/shader translator. |
| 800 | `crates/horizon-client-render/src/post_chain.wgsl` | adaptable; version-specific | Fallback paired with verified target post chain. |

## Part 29 — Entries 801–1000

| No. | Source path | Decision | Review note |
|---:|---|---|---|
| 801 | `crates/horizon-client-render/src/post_glsl.rs` | adaptable; reference; optional/future | Not a complete GLSL implementation; unsupported extensions and implicit semantics remain. |
| 802 | `crates/horizon-client-render/src/potent_sulfur.rs` | unnecessary; version-specific | Not a vanilla 1.20.1 feature or registry entry. |
| 803 | `crates/horizon-client-render/src/preedit.rs` | adaptable; reference; version-specific | UTF-8 offsets, graphemes and Java UTF-16 indices are not interchangeable. |
| 804 | `crates/horizon-client-render/src/providers.rs` | adaptable; reference; version-specific | Provider variants and constraints are version-specific; bound dimensions and image sizes. |
| 805 | `crates/horizon-client-render/src/screen.wgsl` | adaptable; reference; version-specific | Must match bind layout, alpha and scaling exactly. |
| 806 | `crates/horizon-client-render/src/screen_draw.rs` | adaptable; version-specific | Ordering and nested scissor are observable; batch only across equivalent state. |
| 807 | `crates/horizon-client-render/src/screen_effects.rs` | adaptable; reference; version-specific | Effect list and ordering are version-specific. |
| 808 | `crates/horizon-client-render/src/sculk.rs` | adaptable; reference; version-specific | Post-1.20.1 states may be included; data table must be filtered. |
| 809 | `crates/horizon-client-render/src/section_mask.rs` | adaptable; reference; version-specific | Index boundary and shift behavior require explicit bounds. |
| 810 | `crates/horizon-client-render/src/server_terrain.rs` | adaptable; reference; optional/future | Horizon-specific extension, not vanilla protocol; stale tiles must never replace nearby real chunks. |
| 811 | `crates/horizon-client-render/src/settings.rs` | adaptable; reference; version-specific | Large set of LOD/FSR/frame-gen/native-pack settings are Horizon-only. |
| 812 | `crates/horizon-client-render/src/shader_options.rs` | adaptable; reference; optional/future | User pack metadata is untrusted and can request invalid ranges/rebuilds. |
| 813 | `crates/horizon-client-render/src/shader_pack_list.rs` | adaptable; reference; optional/future | File traversal, symlinks and pack compatibility need hardening. |
| 814 | `crates/horizon-client-render/src/signs.rs` | adaptable; version-specific | NBT/components and side fields must match 1.20.1 exactly. |
| 815 | `crates/horizon-client-render/src/skins.rs` | adaptable; reference; optional/future | Concurrency, stale profiles and unbounded cache growth risk; later metadata may differ. |
| 816 | `crates/horizon-client-render/src/sound_engine.rs` | adaptable; reference; optional/future | Large replacement for conventional audio backend; source/event catalogue includes later-version data. |
| 817 | `crates/horizon-client-render/src/sound_events.rs` | adaptable; reference; version-specific | Snapshot's event ids/data are 26.3-specific. |
| 818 | `crates/horizon-client-render/src/sound_preview.rs` | adaptable; reference; optional/future | Rapid selection/screen close can leave source lifetime bugs. |
| 819 | `crates/horizon-client-render/src/spans.rs` | adaptable; reference; version-specific | Unicode indexing and bidi reordering can break formatting if byte offsets are used. |
| 820 | `crates/horizon-client-render/src/split_layout.rs` | unnecessary; reference; optional/future | Split-screen is Horizon-only and not first-release mobile scope. |
| 821 | `crates/horizon-client-render/src/split_map.rs` | unnecessary; reference; optional/future | Product-specific; reassignment/removal handling requires lifecycle tests. |
| 822 | `crates/horizon-client-render/src/split_profiles.rs` | unnecessary; reference; optional/future | Not vanilla Minecraft behavior; preset data is Horizon-specific. |
| 823 | `crates/horizon-client-render/src/split_profiles/store.rs` | unnecessary; reference; optional/future | Atomic save/corrupt data handling must be checked. |
| 824 | `crates/horizon-client-render/src/split_ui.rs` | unnecessary; reference; optional/future | Optional product layer with complicated multi-focus semantics. |
| 825 | `crates/horizon-client-render/src/sprite_cache.rs` | adaptable; reference; version-specific | Path/size/mtime can miss same-size content replacement; cache must be versioned and bounded. |
| 826 | `crates/horizon-client-render/src/sprites.rs` | adaptable; version-specific | Codec variants, alpha and image bounds are version-specific. |
| 827 | `crates/horizon-client-render/src/static_entity.wgsl` | adaptable; reference; version-specific | Coordinate-space mismatch can cause double camera offsets. |
| 828 | `crates/horizon-client-render/src/streaming.rs` | adaptable; version-specific | A stale worker result or LOD substitute must never replace nearby authoritative chunk data; near residency policy is critical. |
| 829 | `crates/horizon-client-render/src/structure_block.rs` | adaptable; reference; version-specific | Template/rotation semantics must match 1.20.1. |
| 830 | `crates/horizon-client-render/src/system_stats.rs` | unnecessary; reference; optional/future | OEM/kernel nodes may be missing or permission-restricted; not a stable gameplay dependency. |
| 831 | `crates/horizon-client-render/src/tab_list.rs` | adaptable; version-specific | Later profile fields may not exist in 1.20.1; packet model must be target-version-specific. |
| 832 | `crates/horizon-client-render/src/telemetry.rs` | unnecessary; reference; optional/future | Identifiers, paths and tokens must be scrubbed; telemetry is not gameplay-critical. |
| 833 | `crates/horizon-client-render/src/terrain.wgsl` | adaptable; reference; version-specific | Tightly bound to newer renderer pipelines/26.3 assets and shader features; desktop shader success would not prove Mali/GLES compatibility. |
| 834 | `crates/horizon-client-render/src/test_block.rs` | unnecessary; reference; optional/future | Confirm target fields against 1.20.1 source before reuse. |
| 835 | `crates/horizon-client-render/src/test_instance_block.rs` | unnecessary; version-specific | Feature is newer than 1.20.1. |
| 836 | `crates/horizon-client-render/src/testgen.rs` | adaptable; reference; version-specific | Generated output must be deterministic and version-pinned. |
| 837 | `crates/horizon-client-render/src/title_edition.rs` | unnecessary; reference; optional/future | Branding is Horizon-specific, not a Minecraft feature. |
| 838 | `crates/horizon-client-render/src/trims.rs` | adaptable; version-specific | Armor trims exist in 1.20 but this snapshot's tables/assets may be newer. |
| 839 | `crates/horizon-client-render/src/ui.rs` | adaptable; reference; version-specific | Large Horizon-specific UI integration; bulk port introduces coupling and lifecycle/input-focus bugs. |
| 840 | `crates/horizon-client-render/src/vault.rs` | unnecessary; version-specific | Trial Chamber/Vault are 1.21-era features, not 1.20.1. |
| 841 | `crates/horizon-client-render/src/weather.rs` | adaptable; version-specific | Climate and biome data in snapshot are version-specific. |
| 842 | `crates/horizon-client-render/src/widgets.rs` | adaptable; reference; version-specific | Widget style/focus/IME behavior is Horizon-specific. |
| 843 | `crates/horizon-client-render/src/world_post.wgsl` | adaptable; reference; optional/future | Coupled to Horizon HDR/FSR/native shader stack. |
| 844 | `crates/horizon-client-render/src/worldborder.rs` | adaptable; version-specific | Core logic is vanilla but interpolation/limits must match 1.20.1. |
| 845 | `crates/horizon-client-render/src/zip.rs` | adaptable; version-specific | Archive-bomb limits, CRC and normalized paths must be consistently enforced across methods. |
| 846 | `crates/horizon-client-render/tests/sunrise_shader.rs` | unnecessary; reference; optional/future | GPU test is ignored and was not run; host GPU success would not imply Android Mali compatibility. |
| 847 | `crates/horizon-client/assets/bedrock-touch.json` | unnecessary; reference; optional/future | Hash pinning doesn't replace checking the actual license text and matching each included asset. |
| 848 | `crates/horizon-client/assets/horizon/lang/de_de.json` | unnecessary; reference; optional/future | Partial localization with substantial English fallback. |
| 849 | `crates/horizon-client/assets/horizon/lang/en_us.json` | unnecessary; reference; optional/future | Reference for Horizon UI only, not for Minecraft 1.20.1 vanilla strings. |
| 850 | `crates/horizon-client/assets/horizon/lang/es_es.json` | unnecessary; reference; optional/future | Incomplete localization; fallback will be common. |
| 851 | `crates/horizon-client/assets/horizon/lang/fr_fr.json` | unnecessary; reference; optional/future | Partial localization; long labels may affect layout. |
| 852 | `crates/horizon-client/assets/horizon/lang/ms_my.json` | unnecessary; reference; optional/future | Coverage appears complete by key count, but key parity is not proof of translation quality. |
| 853 | `crates/horizon-client/assets/horizon/lang/pl_pl.json` | unnecessary; reference; optional/future | Partial localization; inflection and long UI text need QA. |
| 854 | `crates/horizon-client/assets/horizon/lang/pt_br.json` | unnecessary; reference; optional/future | Very incomplete localization. |
| 855 | `crates/horizon-client/assets/horizon/lang/ru_ru.json` | unnecessary; reference; optional/future | Partial localization; long labels/plurals need QA. |
| 856 | `crates/horizon-client/assets/horizon/lang/tr_tr.json` | unnecessary; reference; optional/future | Incomplete localization; Turkish I/İ casing needs correct case mapping. |
| 857 | `crates/horizon-client/assets/horizon/lang/uk_ua.json` | unnecessary; reference; optional/future | Partial localization and plural QA needed. |
| 858 | `crates/horizon-client/assets/horizon/lang/zh_cn.json` | unnecessary; reference; optional/future | Partial localization; CJK font fallback/width are relevant. |
| 859 | `crates/horizon-client/assets/touch/attack.png` | unnecessary; reference; optional/future | Rendered pixels inspected; حمله, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 860 | `crates/horizon-client/assets/touch/attack_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; حمله, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 861 | `crates/horizon-client/assets/touch/camera.png` | unnecessary; reference; optional/future | Rendered pixels inspected; دوربین, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 862 | `crates/horizon-client/assets/touch/camera_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; دوربین, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 863 | `crates/horizon-client/assets/touch/chat.png` | unnecessary; reference; optional/future | Rendered pixels inspected; گفت‌وگو, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 864 | `crates/horizon-client/assets/touch/chat_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; گفت‌وگو, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 865 | `crates/horizon-client/assets/touch/command.png` | unnecessary; reference; optional/future | Rendered pixels inspected; فرمان, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 866 | `crates/horizon-client/assets/touch/command_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; فرمان, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 867 | `crates/horizon-client/assets/touch/custom_apple.png` | unnecessary; reference; optional/future | Rendered pixels inspected; سیب, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 868 | `crates/horizon-client/assets/touch/custom_apple_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; سیب, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 869 | `crates/horizon-client/assets/touch/custom_blank.png` | unnecessary; reference; optional/future | Rendered pixels inspected; پس‌زمینهٔ خالی, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 870 | `crates/horizon-client/assets/touch/custom_check.png` | unnecessary; reference; optional/future | Rendered pixels inspected; تأیید, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 871 | `crates/horizon-client/assets/touch/custom_check_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; تأیید, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 872 | `crates/horizon-client/assets/touch/custom_cross.png` | unnecessary; reference; optional/future | Rendered pixels inspected; ضربدر, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 873 | `crates/horizon-client/assets/touch/custom_cross_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; ضربدر, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 874 | `crates/horizon-client/assets/touch/custom_crown.png` | unnecessary; reference; optional/future | Rendered pixels inspected; تاج, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 875 | `crates/horizon-client/assets/touch/custom_crown_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; تاج, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 876 | `crates/horizon-client/assets/touch/custom_fire.png` | unnecessary; reference; optional/future | Rendered pixels inspected; آتش, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 877 | `crates/horizon-client/assets/touch/custom_fire_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; آتش, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 878 | `crates/horizon-client/assets/touch/custom_flag.png` | unnecessary; reference; optional/future | Rendered pixels inspected; پرچم, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 879 | `crates/horizon-client/assets/touch/custom_flag_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; پرچم, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 880 | `crates/horizon-client/assets/touch/custom_heart.png` | unnecessary; reference; optional/future | Rendered pixels inspected; قلب, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 881 | `crates/horizon-client/assets/touch/custom_heart_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; قلب, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 882 | `crates/horizon-client/assets/touch/custom_home.png` | unnecessary; reference; optional/future | Rendered pixels inspected; خانه, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 883 | `crates/horizon-client/assets/touch/custom_home_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; خانه, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 884 | `crates/horizon-client/assets/touch/custom_music.png` | unnecessary; reference; optional/future | Rendered pixels inspected; موسیقی, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 885 | `crates/horizon-client/assets/touch/custom_music_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; موسیقی, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 886 | `crates/horizon-client/assets/touch/custom_pickaxe.png` | unnecessary; reference; optional/future | Rendered pixels inspected; کلنگ, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 887 | `crates/horizon-client/assets/touch/custom_pickaxe_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; کلنگ, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 888 | `crates/horizon-client/assets/touch/custom_plus.png` | unnecessary; reference; optional/future | Rendered pixels inspected; افزودن, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 889 | `crates/horizon-client/assets/touch/custom_plus_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; افزودن, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 890 | `crates/horizon-client/assets/touch/custom_potion.png` | unnecessary; reference; optional/future | Rendered pixels inspected; معجون, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 891 | `crates/horizon-client/assets/touch/custom_potion_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; معجون, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 892 | `crates/horizon-client/assets/touch/custom_shield.png` | unnecessary; reference; optional/future | Rendered pixels inspected; سپر, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 893 | `crates/horizon-client/assets/touch/custom_shield_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; سپر, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 894 | `crates/horizon-client/assets/touch/custom_skull.png` | unnecessary; reference; optional/future | Rendered pixels inspected; جمجمه, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 895 | `crates/horizon-client/assets/touch/custom_skull_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; جمجمه, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 896 | `crates/horizon-client/assets/touch/custom_star.png` | unnecessary; reference; optional/future | Rendered pixels inspected; ستاره, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 897 | `crates/horizon-client/assets/touch/custom_star_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; ستاره, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 898 | `crates/horizon-client/assets/touch/debug.png` | unnecessary; reference; optional/future | Rendered pixels inspected; اشکال‌زدایی, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 899 | `crates/horizon-client/assets/touch/debug_pressed.png` | unnecessary; reference; optional/future | Rendered pixels inspected; اشکال‌زدایی, pressed state; 24×24 RGBA. Reference only pending license/provenance review. |
| 900 | `crates/horizon-client/assets/touch/dpad_down.png` | unnecessary; reference; optional/future | Rendered pixels inspected; جهت پایین, default state; 24×24 RGBA. Reference only pending license/provenance review. |
| 901 | `crates/horizon-client/assets/touch/dpad_down_pressed.png` | unnecessary; reference; optional/future | D-pad down pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 902 | `crates/horizon-client/assets/touch/dpad_left.png` | unnecessary; reference; optional/future | D-pad left default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 903 | `crates/horizon-client/assets/touch/dpad_left_pressed.png` | unnecessary; reference; optional/future | D-pad left pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 904 | `crates/horizon-client/assets/touch/dpad_middle.png` | unnecessary; reference; optional/future | D-pad central/junction tile, not an action button; 24×24 RGBA; reference only. |
| 905 | `crates/horizon-client/assets/touch/dpad_right.png` | unnecessary; reference; optional/future | D-pad right default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 906 | `crates/horizon-client/assets/touch/dpad_right_pressed.png` | unnecessary; reference; optional/future | D-pad right pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 907 | `crates/horizon-client/assets/touch/dpad_up.png` | unnecessary; reference; optional/future | D-pad up default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 908 | `crates/horizon-client/assets/touch/dpad_up_pressed.png` | unnecessary; reference; optional/future | D-pad up pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 909 | `crates/horizon-client/assets/touch/drop.png` | unnecessary; reference; optional/future | Drop-item default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 910 | `crates/horizon-client/assets/touch/drop_pressed.png` | unnecessary; reference; optional/future | Drop-item pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 911 | `crates/horizon-client/assets/touch/inventory.png` | unnecessary; reference; optional/future | Inventory default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 912 | `crates/horizon-client/assets/touch/inventory_pressed.png` | unnecessary; reference; optional/future | Inventory pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 913 | `crates/horizon-client/assets/touch/jump.png` | unnecessary; reference; optional/future | Jump default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 914 | `crates/horizon-client/assets/touch/jump_pressed.png` | unnecessary; reference; optional/future | Jump pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 915 | `crates/horizon-client/assets/touch/offhand.png` | unnecessary; reference; optional/future | Offhand default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 916 | `crates/horizon-client/assets/touch/offhand_pressed.png` | unnecessary; reference; optional/future | Offhand pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 917 | `crates/horizon-client/assets/touch/pause.png` | unnecessary; reference; optional/future | Pause default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 918 | `crates/horizon-client/assets/touch/pause_pressed.png` | unnecessary; reference; optional/future | Pause pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 919 | `crates/horizon-client/assets/touch/pick.png` | unnecessary; reference; optional/future | Pick-block default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 920 | `crates/horizon-client/assets/touch/pick_pressed.png` | unnecessary; reference; optional/future | Pick-block pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 921 | `crates/horizon-client/assets/touch/players.png` | unnecessary; reference; optional/future | Player-list default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 922 | `crates/horizon-client/assets/touch/players_pressed.png` | unnecessary; reference; optional/future | Player-list pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 923 | `crates/horizon-client/assets/touch/sneak.png` | unnecessary; reference; optional/future | Sneak default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 924 | `crates/horizon-client/assets/touch/sneak_pressed.png` | unnecessary; reference; optional/future | Sneak pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 925 | `crates/horizon-client/assets/touch/stick.png` | unnecessary; reference; optional/future | Joystick base; 48×48 RGBA; use only as visual reference. |
| 926 | `crates/horizon-client/assets/touch/stick_knob.png` | unnecessary; reference; optional/future | Joystick knob; 24×24 RGBA; use only as visual reference. |
| 927 | `crates/horizon-client/assets/touch/use.png` | unnecessary; reference; optional/future | Use-item default glyph; 24×24 RGBA; reference only, no reuse approval. |
| 928 | `crates/horizon-client/assets/touch/use_pressed.png` | unnecessary; reference; optional/future | Use-item pressed glyph; 24×24 RGBA; reference only, no reuse approval. |
| 929 | `crates/horizon-client/build.rs` | unnecessary; version-specific | Windows desktop build hook; not Android Gradle/NDK packaging. |
| 930 | `crates/horizon-client/examples/chunk_load_bench.rs` | adaptable; reference; optional/future | Synthetic fixtures/newer asset version and desktop timing are not Galaxy A22 performance evidence. |
| 931 | `crates/horizon-client/examples/collision_bench.rs` | adaptable; reference; optional/future | Synthetic query mix may not represent real movement/world geometry. |
| 932 | `crates/horizon-client/examples/gameplay_decode_profile.rs` | adaptable; reference; optional/future | Desktop profile and snapshot schema do not establish Android performance. |
| 933 | `crates/horizon-client/examples/gen_light_bench.rs` | adaptable; reference; optional/future | Newer data and potentially long-running runs; no benchmark was run. |
| 934 | `crates/horizon-client/examples/json_load_bench.rs` | adaptable; reference; optional/future | I/O benefits depend on Android storage/allocation; no result was run. |
| 935 | `crates/horizon-client/examples/mod_port_probe.rs` | undetermined; reference; version-specific | NeoForge/Horizon carried-content probe only; does not implement or prove Forge 47.x support. |
| 936 | `crates/horizon-client/examples/model_library_bench.rs` | adaptable; reference; optional/future | Desktop allocation instrumentation isn't an Android baseline. |
| 937 | `crates/horizon-client/examples/model_states_bench.rs` | adaptable; reference; optional/future | Synthetic state corpus and newer registry sizes are not representative of 1.20.1. |
| 938 | `crates/horizon-client/examples/old_world_fixture.rs` | unnecessary; reference; optional/future | Destructive fixture deletes region files at its destination; isolate in disposable test data. |
| 939 | `crates/horizon-client/examples/portal_scan_bench.rs` | adaptable; reference; optional/future | Synthetic workload doesn't predict real portals/player session load. |
| 940 | `crates/horizon-client/examples/probe/ambient_fixture.rs` | adaptable; reference; version-specific | Ambient sound state-machine fixture; fake backend, rebuild 1.20.1 sound definitions. |
| 941 | `crates/horizon-client/examples/probe/arrow_probe.rs` | adaptable; reference; version-specific | Item/effect and entity data may be newer; collision order is observable. |
| 942 | `crates/horizon-client/examples/probe/audio_attributes_fixture.rs` | adaptable; reference; version-specific | Sound groups/defaults depend on target version. |
| 943 | `crates/horizon-client/examples/probe/audio_common/mod.rs` | adaptable; reference; version-specific | Fake backend tests internal scheduling only, not actual audio output. |
| 944 | `crates/horizon-client/examples/probe/auto_jump_probe.rs` | adaptable; reference; version-specific | Thresholds and collision geometry must match 1.20.1. |
| 945 | `crates/horizon-client/examples/probe/block_entity_anim_fixture.rs` | adaptable; reference; version-specific | Contains later-added entity/block states beyond 1.20.1; use version-filtered cases. |
| 946 | `crates/horizon-client/examples/probe/block_entity_render_fixture.rs` | adaptable; reference; version-specific | The fixture uses the pinned snapshot's renderer, atlas, texture/model catalog and block-entity set; all of these must be replaced or filtered for Java 1.20.1 before its outputs can serve as an oracle. |
| 947 | `crates/horizon-client/examples/probe/block_entity_tickers_fixture.rs` | adaptable; reference; version-specific | Block-entity ticker lifecycle/order; mapping corrected to the ticker fixture, not render fixture. |
| 948 | `crates/horizon-client/examples/probe/block_particles_fixture.rs` | adaptable; reference; version-specific | Break-particle positions from outline shape/offset and deterministic RNG; rebuild 1.20.1 block data. |
| 949 | `crates/horizon-client/examples/probe/block_tint_probe.rs` | adaptable; reference; version-specific | Default block-model tint fallback; not biome/world tint; rebuild 1.20.1 tables. |
| 950 | `crates/horizon-client/examples/probe/block_transform_fixture.rs` | adaptable; reference; version-specific | Block transformer/honeycomb lookup; state and transformation data are versioned. |
| 951 | `crates/horizon-client/examples/probe/breaking_fixture.rs` | adaptable; reference; version-specific | Destroy-crack overlay model geometry/UV oracle; rebuild target crack sprites and model states. |
| 952 | `crates/horizon-client/examples/probe/camera_probe.rs` | adaptable; reference; version-specific | Exact numeric behavior is visible; source uses current model/input types. |
| 953 | `crates/horizon-client/examples/probe/cauldron_fixture.rs` | adaptable; reference; version-specific | Item/tag table must be the 1.20.1 version. |
| 954 | `crates/horizon-client/examples/probe/chat_screen_fixture.rs` | adaptable; reference; version-specific | Newer chat components/signing flags can appear; separate UI and protocol semantics. |
| 955 | `crates/horizon-client/examples/probe/chat_screen_fixture/registries.rs` | adaptable; reference; version-specific | Registry contents are target-version data. |
| 956 | `crates/horizon-client/examples/probe/chest_pair_fixture.rs` | adaptable; reference; version-specific | Chest state logic should follow 1.20.1 block updates. |
| 957 | `crates/horizon-client/examples/probe/cloud_fixture.rs` | adaptable; reference; version-specific | Custom cloud rendering may differ from vanilla. |
| 958 | `crates/horizon-client/examples/probe/command_parse_fixture.rs` | adaptable; reference; version-specific | Command parse/tree/suggestion harness retained; rebuild 1.20.1 command tree, argument IDs and registries. |
| 959 | `crates/horizon-client/examples/probe/container_screen_probe.rs` | adaptable; reference; version-specific | Menu click semantics must match Java 1.20.1. |
| 960 | `crates/horizon-client/examples/probe/container_title_fixture.rs` | adaptable; reference; version-specific | Modern text components may differ from 1.20.1 NBT display name. |
| 961 | `crates/horizon-client/examples/probe/creative_mining_probe.rs` | unnecessary; version-specific | Uses newer minecraft:tool data component; not the 1.20.1 item schema. |
| 962 | `crates/horizon-client/examples/probe/creative_search_fixture.rs` | adaptable; reference; version-specific | Tabs and items are pinned to newer registry data. |
| 963 | `crates/horizon-client/examples/probe/cushion_placement_probe.rs` | undetermined; reference; version-specific | Cushion placement/survival abstraction; vanilla 1.20.1 applicability remains unverified. |
| 964 | `crates/horizon-client/examples/probe/emitter_particles_probe.rs` | adaptable; reference; version-specific | Particle types and providers are version-specific. |
| 965 | `crates/horizon-client/examples/probe/enchanting_fixture.rs` | adaptable; reference; version-specific | Enchantments/weights and item data must match 1.20.1. |
| 966 | `crates/horizon-client/examples/probe/entity_anim_fixture.rs` | adaptable; reference; version-specific | Includes later species (Creaking/Sniffer/Happy Ghast etc.) beyond 1.20.1. |
| 967 | `crates/horizon-client/examples/probe/entity_components_probe.rs` | adaptable; reference; version-specific | Generic entity predicate harness is reusable; newer species-variant component schema is not 1.20.1 data. |
| 968 | `crates/horizon-client/examples/probe/entity_death_loot_fixture.rs` | adaptable; reference; version-specific | New item components/species can be present; target loot data must be rebuilt. |
| 969 | `crates/horizon-client/examples/probe/entity_particles_probe.rs` | adaptable; reference; version-specific | Event/particle registry is version-dependent. |
| 970 | `crates/horizon-client/examples/probe/environment_fixture.rs` | adaptable; reference; version-specific | Dimension/weather/biome sky and fog sampling fixture; rebuild 1.20.1 data. |
| 971 | `crates/horizon-client/examples/probe/finalize_spawn_fixture.rs` | adaptable; reference; version-specific | Mob finalize-spawn fixture; filter post-1.20.1 species and data. |
| 972 | `crates/horizon-client/examples/probe/fire_starter_sound_probe.rs` | adaptable; reference; version-specific | Small but random-call order is observable and version-bound. |
| 973 | `crates/horizon-client/examples/probe/firework_tooltip_fixture.rs` | adaptable; reference; version-specific | Modern component schema may differ; 1.20.1 uses target NBT forms. |
| 974 | `crates/horizon-client/examples/probe/first_person_probe.rs` | adaptable; reference; version-specific | Exact timing and item categories must match target 1.20.1. |
| 975 | `crates/horizon-client/examples/probe/frame_limiter_probe.rs` | adaptable; reference; version-specific | Virtual-clock frame limiter oracle; desktop scheduler is not Android pacing evidence. |
| 976 | `crates/horizon-client/examples/probe/game_loop_probe.rs` | adaptable; reference; version-specific | Timer, server tick rate, window scaling, mouse and key state; adapt lifecycle for Android. |
| 977 | `crates/horizon-client/examples/probe/ghost_recipe_fixture.rs` | adaptable; reference; version-specific | Recipe and item data are version-specific. |
| 978 | `crates/horizon-client/examples/probe/held_drop_fixture.rs` | adaptable; reference; version-specific | Item stack serialization and action order must match 1.20.1. |
| 979 | `crates/horizon-client/examples/probe/held_item_stack_probe.rs` | adaptable; reference; version-specific | New component fields can differ by version. |
| 980 | `crates/horizon-client/examples/probe/hotbar_fixture.rs` | adaptable; reference; version-specific | Inventory/slot serialization is versioned. |
| 981 | `crates/horizon-client/examples/probe/hud_fixture.rs` | adaptable; reference; version-specific | Some newer HUD indicators/effects may be included; target atlas and behavior needed. |
| 982 | `crates/horizon-client/examples/probe/hud_fixture/kinds.rs` | adaptable; reference; version-specific | Overlay set and game fields may contain later additions. |
| 983 | `crates/horizon-client/examples/probe/inline_object_fixture.rs` | unnecessary; version-specific | Inline atlas-object text syntax is not established as vanilla 1.20.1; exclude baseline. |
| 984 | `crates/horizon-client/examples/probe/input_probe.rs` | adaptable; reference; version-specific | Platform bindings should not leak into game actions. |
| 985 | `crates/horizon-client/examples/probe/interaction_input_fixture.rs` | adaptable; reference; version-specific | Attack/use/pick state-machine fixture; Android touch needs pointer IDs and cancel handling. |
| 986 | `crates/horizon-client/examples/probe/lan_probe.rs` | adaptable; reference; optional/future | LAN discovery/multicast only; optional, direct address joins remain independent. |
| 987 | `crates/horizon-client/examples/probe/light_parity.rs` | adaptable; reference; version-specific | Vanilla saved-chunk light comparison and incremental-relight oracle; high-value QA harness. |
| 988 | `crates/horizon-client/examples/probe/lore_tooltip_fixture.rs` | adaptable; reference; version-specific | New component schema may not match 1.20.1 display.Lore NBT. |
| 989 | `crates/horizon-client/examples/probe/marker_particles_probe.rs` | adaptable; reference; version-specific | Marker-particle probe uses hardcoded 26.3 resources; verify 1.20.1 marker behavior and data. |
| 990 | `crates/horizon-client/examples/probe/menu_fixture.rs` | adaptable; reference; version-specific | Horizon-specific screens should not be mixed with vanilla screen parity. |
| 991 | `crates/horizon-client/examples/probe/mipmap_probe.rs` | adaptable; reference; version-specific | Filtering/alpha behavior affects foliage/cutout textures and needs Java parity. |
| 992 | `crates/horizon-client/examples/probe/mob_ai_probe.rs` | adaptable; reference; version-specific | Mob AI sandbox/goal/event fixture; rebuild only 1.20.1 species and preserve phase/RNG order. |
| 993 | `crates/horizon-client/examples/probe/mob_attributes_fixture.rs` | adaptable; reference; version-specific | Values come from newer registry data and include later attributes. |
| 994 | `crates/horizon-client/examples/probe/music_fixture.rs` | adaptable; reference; version-specific | Music list and timing policy are versioned; fake backend isn't actual audio output. |
| 995 | `crates/horizon-client/examples/probe/natural_spawn_fixture.rs` | adaptable; reference; version-specific | Natural spawn + classic Spawner are relevant; Trial Spawner/Vault are 1.21+ and excluded. |
| 996 | `crates/horizon-client/examples/probe/options_probe.rs` | adaptable; reference; version-specific | Horizon permits render distance 64 then normalizes above 32 for vanilla comparison; this is a deliberate extension. |
| 997 | `crates/horizon-client/examples/probe/pack_resources_fixture.rs` | adaptable; reference; version-specific | Pack overlay/resource lookup oracle; rebuild 1.20.1 formats and reject silent parse fallback. |
| 998 | `crates/horizon-client/examples/probe/pack_screen_fixture.rs` | adaptable; reference; version-specific | Screen labels/compatibility levels and UI structure can be Horizon-specific. |
| 999 | `crates/horizon-client/examples/probe/pack_selection_fixture.rs` | adaptable; reference; version-specific | Pack selection ordering; required/server packs must remain fixed and unremovable. |
| 1000 | `crates/horizon-client/examples/probe/particle_providers_probe.rs` | adaptable; reference; version-specific | Particle provider/tick fixture; prune newer particle types and replace hardcoded newer resources. |

## Part 41 — Entries 1001–1020

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1001 | `crates/horizon-client/examples/probe/recipe_book_render_fixture.rs` | adaptable; reference; version-specific | It is a test transport, not a screen controller or resource loader for the game. Unknown recipe type falls back to Smoker; page/overlay indices are trusted and may panic on malformed fixtures. Displayed draw calls depend on this snapshot's atlas, recipe catalogue and localized strings. |
| 1002 | `crates/horizon-client/examples/probe/recipe_book_view_fixture.rs` | adaptable; reference; version-specific | The fixture models the client recipe display history and local stock accounting but trusts fixture indices and type strings. Registry IDs, display IDs, recipe categories and tooltip search results are snapshot-specific. |
| 1003 | `crates/horizon-client/examples/probe/recipe_fixture.rs` | adaptable; reference; version-specific | The harness is intentionally thin and depends on prepared data correctness. Cooking recipe IDs are normalized with a fixed `minecraft:` prefix, and input indexing is trusted; malformed fixture rows can fail. |
| 1004 | `crates/horizon-client/examples/probe/remote_attack_probe.rs` | adaptable; reference; version-specific | Deliberately excludes mob targets because the fixture notes Entity.hurtClient is false for those in this path. It does not simulate the server's authoritative damage/knockback or the full combat pipeline. |
| 1005 | `crates/horizon-client/examples/probe/scene_collision_probe.rs` | adaptable; reference; version-specific | Uses a fixed test subject identity, a very high candidate budget, and a synthetic scene rather than a complete loaded world. The fixture directory is expected to be prepared externally; it does not persist chunks or exercise server authority. |
| 1006 | `crates/horizon-client/examples/probe/scene_motion_probe.rs` | adaptable; reference; version-specific | The fixture encodes explicit oracle assumptions: one player, test scene radius 1, no armor (armor-hurt events filtered), player attribute identities injected, block effects only enabled for snow_tick, and a simplified portal transition model. It is a differential transport, not a complete live simulation. |
| 1007 | `crates/horizon-client/examples/probe/scoreboard_fixture.rs` | adaptable; reference; version-specific | The fixture has test-only trust assumptions: several args/fields use unwrap/expect/panic; malformed hex in unhex is converted to zero bytes. The newer NumberFormat and component representations are version-specific and may not exist in 1.20.1. |
| 1008 | `crates/horizon-client/examples/probe/screen_layout_fixture.rs` | adaptable; reference; version-specific | The fixture merges two screens but their target-version status differs. Loom exists in 1.20.1; Crafter was introduced after 1.20.1 and must be excluded from the 1.20.1 baseline. Draw output also depends on newer atlas IDs/assets. |
| 1009 | `crates/horizon-client/examples/probe/section_mesh_fixture.rs` | adaptable; reference; version-specific | The fixture hardcodes several rendering modes off (shader-pack lighting, separate AO, translucent layers, leaf culling) and uses only solid/translucent outputs. It depends on the pinned model/atlas/biome/state corpus, not a self-contained 1.20.1 renderer. |
| 1010 | `crates/horizon-client/examples/probe/server_files_probe.rs` | unnecessary; reference; version-specific | This is server-administration logic, not Minecraft client runtime. It includes newer management-server/code-of-conduct/transfer-related properties not in 1.20.1. The temporary directory name is PID plus case name and is recursively removed before use; safe only for isolated test fixtures, not production/user data. |
| 1011 | `crates/horizon-client/examples/probe/situational_music_fixture.rs` | adaptable; reference; version-specific | The harness is concise but depends entirely on newer snapshot music table contents and flag precedence. Several input fields are positional and parsed without robust validation. |
| 1012 | `crates/horizon-client/examples/probe/skin_image_fixture.rs` | adaptable; reference; version-specific | The fixture tests legacy skin normalization only; it does not verify online texture signature validation, profile/session trust, URL fetching, caching or rendering on Android. Image size/dimension limits are enforced downstream and should be confirmed. |
| 1013 | `crates/horizon-client/examples/probe/sound_engine_fixture.rs` | adaptable; reference; version-specific | The fake backend is a useful deterministic boundary but cannot prove OpenAL/AAudio behavior. Sound event catalogues, source categories, cart behavior and operation timing come from the pinned newer snapshot. |
| 1014 | `crates/horizon-client/examples/probe/sound_event_fixture.rs` | adaptable; reference; version-specific | The registered event list and file references are data-version specific. The output only inspects resolution; it does not decode/play audio or verify actual resource files and codecs. |
| 1015 | `crates/horizon-client/examples/probe/spawn_rules_fixture.rs` | adaptable; reference; version-specific | The fixture covers many useful predicates but only a one-column synthetic world; it includes a `trial_spawner` reason introduced after 1.20.1 and relies on data/tag tables from the newer snapshot. Unknown reason strings fall back to Natural, which can hide fixture typos. |
| 1016 | `crates/horizon-client/examples/probe/sprint_probe.rs` | adaptable; reference; version-specific | The test cleanly isolates sprint transition ordering but maps many environment flags manually. It uses `analog: None`, so controller/analogue movement is not covered; the conditions list includes later movement flags. |
| 1017 | `crates/horizon-client/examples/probe/squid_client_probe.rs` | adaptable; reference; version-specific | The velocity-preservation assertion is a valuable authority boundary. The fixed 26.3 resource path harms portability/reproducibility; species data and animation parameters must be sourced for 1.20.1. |
| 1018 | `crates/horizon-client/examples/probe/storage_menu_fixture.rs` | adaptable; reference | The fixture is intentionally narrow and does not model all slot layouts, inventories or menu packets. The shulker-box-in-shulker-box restriction and most container mappings are in 1.20.1 scope, but mapping tables must be rebuilt from target registries. |
| 1019 | `crates/horizon-client/examples/probe/subtitle_fixture.rs` | adaptable; reference; version-specific | The test covers camera-relative subtitle presentation, expiration and listener visibility but does not render through an Android canvas/GPU. Text component styling is flattened to plain strings in the serialized output, and sound keys/data are versioned. |
| 1020 | `crates/horizon-client/examples/probe/tab_list_fixture.rs` | adaptable; reference; version-specific | The fixture mixes stable player-list presentation with later fields such as list-order and show-hat behavior; current NumberFormat and skin texture structures are also tied to the pinned newer protocol. It simulates download states rather than networking. |
