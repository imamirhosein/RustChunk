# RustChunk Starling Audit Index — Entries 1–600

- Source ledger: entries 1–400 in `AUDIT_LEDGER.md`; entries 401–600 in `AUDIT_LEDGER_401_600.md`
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
| 10 | `packaging/lowbudget/make.py` | Adaptable; Rewrite required; Unnecessary | unnecessary for the core Rust-native 1.20.1 client; adaptable only if RustChunk needs a pre-game setup theme or generated placeholder UI pack. Rewrite pack schema and resource identifiers for Minecraft 1.20.1. |
| 11 | `research-lock.json` | Essential; Rewrite required | essential as a concept for external-input pinning and deterministic preparation; rewrite required for RustChunk's 1.20.1 source artifacts, preparation tools and supported platforms. |
| 12 | `roadmap/starling.md` | Adaptable; Rewrite required | adaptable for mobile feature sequencing and clear Done/Left records; rewrite required for the 1.20.1 feature plan and integration assumptions. The roadmap is a planning source, not implementation proof. |
| 13 | `docs/horizon-parity.md` | Essential; Rewrite required | essential as a testing philosophy and differential-harness model; the actual 26.3 test suite is rewrite required for the Rust-native 1.20.1 target. |
| 14 | `docs/android-armv7.md` | Adaptable; Rewrite required | adaptable for ABI-aware packaging/update selection, low-memory controls, GPU capability checks and resource budgets; rewrite required for RustChunk's 1.20.1 resource preparation and renderer integration. |
| 15 | `docs/android-controls.md` | Adaptable; Rewrite required; Version-specific | adaptable for input architecture, editable layouts, lifecycle handling and IME focus contracts; rewrite required for Rust-native Minecraft input integration and version-specific UI hooks. |
| 16 | `docs/android-accounts.md` | Essential; Adaptable; Rewrite required | essential as a set of account-security and profile-management requirements; adaptable as architecture, but rewrite required for RustChunk APIs, Rust-native profile/authentication and Android account integration,... |
| 17 | `docs/android-diagnostics.md` | Essential; Adaptable; Rewrite required | essential as reliability, observability and privacy requirements; adaptable for data model, retention limits and recovery workflow; rewrite required for RustChunk crash capture and Android integration. |
| 18 | `docs/native-mods.md` | Adaptable; Unnecessary; Reference | unnecessary for the first vanilla 1.20.1 milestone; adaptable later for extension API governance, versioned hook contracts, build reproducibility/cache design, code/content separation, opt-in trust and diagnostics... |
| 19 | `docs/starling-native-mods.md` | Adaptable; Unnecessary | adaptable for remote build delivery, APK-bound cache invalidation, resumable verified downloads, rollback and explicit service trust; unnecessary for the base 1.20.1 client; consider only after a native Rust... |
| 20 | `docs/mods.md` | Adaptable; Unnecessary | unnecessary for the base vanilla 1.20.1 milestone; adaptable later for a separate data-only import/porting layer and integrity/compatibility checks. This content importer does not implement the current 1.20.1 game... |
| 21 | `docs/version-ports.md` | Adaptable; Rewrite required | adaptable as the porting methodology (version pinning, provenance/hash verification, generated constants/tables, data diffs, oracle mapping, parity gates and ranked blockers); rewrite/substantial adaptation required... |
| 22 | `docs/versioning.md` | Adaptable; Unnecessary; Optional/future | unnecessary for the 1.20.1 game-engine port; adaptable as an optional launcher/application release-management pattern. No need to reproduce Horizon's historical bridge tags or its release workflow. |
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
| 34 | `crates/horizon-prepare/src/artifacts.rs` | Adaptable | major adaptation required for 1.20.1. Reuse the secure path handling, checksum verification, manifest validation, explicit user-provided game files and local-only artifact storage; replace or extend the blanket... |
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
| 54 | `crates/horizon-protocol/src/item/custom.rs` | Adaptable; Unnecessary; Optional/future | unnecessary for the current 1.20.1 vanilla protocol milestone. Defer or omit this feature until a future item extension design exists; reuse only generic NBT validation ideas if required by a separate 1.20.1 feature. |
| 55 | `crates/horizon-protocol/src/packets/recipes.rs` | Rewrite required | rewrite the recipe networking codecs for 1.20.1. Retain only generic codec patterns and possibly recipe UI concepts after translating them into the older recipe model. Treat current recipe display/slot display IDs as... |
| 56 | `crates/horizon-protocol/src/crypto.rs` | Adaptable; Version-specific | largely adaptable/reusable for 1.20.1 cryptographic primitives and offline UUID derivation. Keep key exchange and login state ordering version-specific; audit platform AES backend separately. |
| 57 | `crates/horizon-protocol/src/rsa.rs` | Adaptable; Version-specific | cryptographic standards and data model are adaptable, but version-specific login/chat wiring and security review required. Prefer vetted constant-time implementations for private operations where feasible; do not... |
| 58 | `crates/horizon-protocol/src/secure_chat.rs` | Rewrite required | adapt selected cryptographic primitives and abstract concepts, but rewrite 1.20.1 signed payload, login/player-info key exchange, chain protocol, last-seen updates and packet integration from target-version vanilla.... |
| 59 | `crates/horizon-protocol/src/resolver.rs` | Adaptable | largely adaptable/reusable for 1.20.1 and Android. Keep the SRV selection rules and injected DNS provider; verify target-version behavior and ensure the Android host supplies current DNS servers. |
| 60 | `crates/horizon-protocol/src/resolver/dns.rs` | Adaptable; Rewrite required; Version-specific | largely reusable/adaptable for 1.20.1, including query, UDP/TCP fallback and platform DNS-source abstraction. Verify parser edge behavior and Android active-network wiring; no protocol-version-specific rewrite is... |
| 61 | `crates/horizon-protocol/src/attachment_sync.rs` | Adaptable; Unnecessary; Optional/future | Unnecessary for the base 1.20.1 milestone; bounded NBT-payload handling is a reusable pattern for a future extension protocol. |
| 62 | `crates/horizon-protocol/src/hash_ops.rs` | Reference; Version-specific | Version-specific and unsuitable for protocol 1.20.1; retain only as a reference for implementing the newer Codec-hash scheme. |
| 63 | `crates/horizon-protocol/src/mod_list.rs` | Adaptable; Unnecessary; Optional/future | Unnecessary for the vanilla 1.20.1 core; file-hash parity is a reusable concept for a future mod-compatibility layer. |
| 64 | `crates/horizon-protocol/src/mod_net.rs` | Adaptable; Unnecessary; Optional/future | Unnecessary for the base milestone; the codec design and strict validation are reusable for a future extension protocol. |
| 65 | `crates/horizon-protocol/src/quiet.rs` | Adaptable; Unnecessary; Optional/future | Unnecessary for the base milestone; batching and size checks are reusable for a future extension protocol. |
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
| 85 | `crates/horizon-client-input/src/key_names.rs` | See ledger decision | Equivalent data can be generated/adapted; this 26.3 table must be regenerated for 1.20.1. |
| 86 | `crates/horizon-client-input/src/keybinds.rs` | Adaptable | The KeyBinds abstraction and physical-key-to-action translation are adaptable; MAPPINGS/KEYS and container behavior must be regenerated or adapted for 1.20.1. |
| 87 | `crates/horizon-client-input/src/keymapping.rs` | Adaptable | The base state machine is adaptable; pad toggling and event/screen semantics must be aligned with the 1.20.1 and Android implementations. |
| 88 | `crates/horizon-client-input/src/modifiers.rs` | Adaptable | Modifier reconciliation and paired-key logic are adaptable; snapshot restoration must be synchronous/serialized and tested against the 1.20.1 key lifecycle. |
| 89 | `crates/horizon-client-input/src/mouse.rs` | Adaptable | core mouse delta aggregation, sensitivity curve and scroll accumulation are adaptable; 1.20.1 constants and platform-specific grab semantics require verification. |
| 90 | `crates/horizon-client-input/src/window.rs` | Adaptable | GUI scaling and throttling concepts are adaptable; the FPS limiter needs Android-specific design and measurement; fullscreen/video-mode implementation is primarily desktop-only and must be adapted or omitted. |
| 91 | `crates/horizon-client-input/src/clipboard.rs` | Adaptable | The Clipboard trait and append-selection pattern are adaptable; filter/chat/shortcut rules and the Android bridge must be reviewed against 1.20.1 and the Android platform. |
| 92 | `crates/horizon-client-input/src/gamepad.rs` | Adaptable; Unnecessary; Optional/future | PadState/Edges/action-map architecture and radial deadzone are adaptable; layout, tuning, navigation, and haptics should be rewritten/ported as optional features, not required parts of the 1.20.1 core. |
| 93 | `crates/horizon-client-input/src/gamepad/android.rs` | Adaptable | The common-state adapter structure is adaptable; Android key/source codes and noise/takeover thresholds require device testing and a complete disconnect lifecycle. |
| 94 | `crates/horizon-client/src/control.rs` | Unnecessary; Optional/future | unnecessary for the base 1.20.1 client; optional tooling only after a separate security hardening pass. |
| 95 | `crates/horizon-client/src/touch.rs` | Highly relevant/adaptable; Adaptable | The touch-event state machine and ownership/action abstraction are highly adaptable; layout/UI gestures and Bedrock-specific interactions must be rewritten as a separate Android layer and matched to vanilla 1.20.1. |
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
| 109 | `crates/horizon-client-render/src/chunk.rs` | Highly relevant/adaptable; Adaptable; Rewrite required | Highly adaptable as a memory-efficient resident chunk representation; global layout, section indexing, light/heightmap semantics and registry data require validation or rewrite for 1.20.1. |
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
| 122 | `crates/horizon-client/src/android.rs` | Highly relevant/adaptable; Reference | Highly relevant as an Android architecture reference; implement a smaller RustChunk bridge with explicit unsafe/JNI review and device validation. |
| 123 | `crates/horizon-client/src/diagnostics.rs` | Adaptable | Adaptable diagnostics architecture; redaction, durability and Android exit collection need security/privacy review. |
| 124 | `crates/horizon-client-render/src/chunk_map.rs` | Adaptable | Adaptable as resident chunk membership bookkeeping; revision lifetime, reset semantics and ordering assumptions need tests. |
| 125 | `crates/horizon-client-render/src/mesher.rs` | Essential; Rewrite required | Essential rendering subsystem concept; reimplement against 1.20.1 model/registry/biome/light/shader data rather than copy as-is. |

## Part 6 — Entries 126–150

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 126 | `crates/horizon-client-render/src/mesher/lighting.rs` | Highly relevant/adaptable; Adaptable; Reference | Highly adaptable as algorithm reference; constants, state facts, float narrowing and shader packing need 1.20.1 parity tests and ARM64 validation. |
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
| 146 | `.github/actionlint.yaml` | Reference-only | Only relevant if RustChunk adopts self-hosted CI runners; runner trust is critical. |
| 147 | `.github/CODEOWNERS` | Adaptable | Protect signing, update, resource-acquisition and release paths with enforced owner review. |
| 148 | `.github/ISSUE_TEMPLATE/bug_report.yml` | Adaptable | Strong Android bug-triage template; rewrite fields and verify log privacy/redaction. |
| 149 | `.github/ISSUE_TEMPLATE/config.yml` | Adaptable | Issue routing pattern only; replace the Horizon-specific destination. |
| 150 | `.github/ISSUE_TEMPLATE/native-build.yml` | Unnecessary; Optional/future | Not needed for vanilla 1.20.1; Starling native builds are not Forge support. |

## Part 7 — Entries 151–160

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 151 | `.github/pull_request_template.md` | Adaptable; Rewrite required | Useful PR provenance and evidence checklist; replace 26.3 references and validate with CI. |
| 152 | `.github/release-line.txt` | Undetermined | Meaning is not clear until its consumers are traced; do not copy the value yet. |
| 153 | `.github/trusted-reviewers.txt` | Reference-only | Repository moderation allowlist only; consumer scripts and permissions need review. |
| 154 | `.github/workflows/cancel-on-close.yml` | Adaptable | CI cost-control pattern; verify pull_request_target safety and run-selection behavior. |
| 155 | `.github/workflows/ci.yml` | Adaptable; Rewrite required | Strong CI foundation, but RustChunk needs 1.20.1 parity and separate Android/device gates. |
| 156 | `.github/workflows/claim-command.yml` | Unnecessary; Adaptable/future | Contributor-claim automation only; not part of the client and requires parser/auth review. |
| 157 | `.github/workflows/mod-registry.yml` | Adaptable; Unnecessary/future | Isolated workspace CI pattern; mod registry is not needed in the vanilla milestone. |
| 158 | `.github/workflows/mojang-republish.yml` | Adaptable | Useful pinned-metadata change alert; rewrite and test against RustChunk's 1.20.1 acquisition lock. |
| 159 | `.github/workflows/native-hook-stamp.yml` | Unnecessary; Optional/future | Generated native-hook ID service is irrelevant until an extension ABI is approved. |
| 160 | `.github/workflows/native-mods.yml` | Unnecessary; Security reference | High-risk Starling native-code pipeline; not Forge and not part of the vanilla client. |

## Part 8 — Entries 161–175

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 161 | `.github/workflows/overlap-note.yml` | Adaptable; Security reference | Trusted main-branch, diff-only PR overlap automation; privileged event/runner isolation needs security tests. |
| 162 | `.github/workflows/pack-bench.yml` | Adaptable; Optional/future | Resource-pack branch/base benchmark pattern; replace 26.3 fixtures and add Android measurements. |
| 163 | `.github/workflows/parity-main.yml` | Adaptable; Rewrite required | Useful parity gate/bisect pattern; requires complete 1.20.1 oracle and protected cache/runner gates. |
| 164 | `.github/workflows/pr-labels.yml` | Adaptable; Security reference | Metadata-only PR labeling; test untrusted event data and never execute PR-head code. |
| 165 | `AGENTS.md` | Adaptable; Rewrite required | Strong navigation/governance template; rewrite Starling paths, versions, tools and policy assumptions. |
| 166 | `bench/bench.py` | Optional/future; Adaptable methodology | Server-load benchmark only; later needs protocol-763 bots/server and process-cleanup hardening. |
| 167 | `benchmarks/entity-model-document-2026-10-14/generate_fixture.py` | Adaptable; Test-only reference | Deterministic synthetic model fixture; regenerate using 1.20.1 resource schema. |
| 168 | `benchmarks/entity-model-document-2026-10-14/README.md` | Adaptable; Reference | Output-equality and warm/alternating method; 42.94× result is only a synthetic parser segment. |
| 169 | `benchmarks/entity-model-document-2026-10-14/results.json` | Reference; Adaptable schema | Raw timings agree with README but lack independent fixture/source/hardware provenance. |
| 170 | `benchmarks/graphics/after-4080.json` | Adaptable methodology; Raw values not reusable | RTX4080/Vulkan post-change report; RD16 drawn_sections=0 requires investigation. |
| 171 | `benchmarks/graphics/after-lavapipe.json` | Reference; Adaptable test method | Software Vulkan stress data only; RD16 drawn_sections=0 and RD32 lows need context. |
| 172 | `benchmarks/graphics/baseline-4080.json` | Reference; Version-specific | Historical RTX4080 baseline; zero/near-zero draw counters limit interpretation. |
| 173 | `benchmarks/graphics/baseline-lavapipe.json` | Reference; Version-specific | Historical llvmpipe baseline; not a physical GPU or phone proxy. |
| 174 | `benchmarks/model-state-parse-2026-10-14.md` | Adaptable; Reference | Shared parse reuse gives a scoped synthetic 1.306× result; validate 1.20.1 and A22. |
| 175 | `benchmarks/overlay-real-packs-2026-10-15.json` | Adaptable; Reference | Cache-hit/cold overlay methodology; pack schema is 26.3 and licenses remain separate. |

## Part 9 — Entries 176–200

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 176 | `benchmarks/perf/horizon-m4pro-1080p-rd16.json` | Reference; Version-specific | Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with… |
| 177 | `benchmarks/perf/horizon-m4pro-1080p-rd32.json` | Reference; Version-specific | Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with… |
| 178 | `benchmarks/perf/horizon-m4pro-1080p-rd8.json` | Reference; Version-specific | Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with… |
| 179 | `benchmarks/perf/horizon-win-9800x3d-5080-rd16.json` | Reference; Version-specific | Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with… |
| 180 | `benchmarks/perf/horizon-win-9800x3d-5080-rd32.json` | Reference; Version-specific | Internal checks for this report series found three flights per scene; summary avg/1%/0.1% FPS values agree with… |
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
| 195 | `benchmarks/perf/windows-9800x3d-5080-2026-10-29.md` | Reference; Version-specific | Better controlled than the loaded Mac report, but RD16 has a weaker external-load check. The report also identifies… |
| 196 | `benchmarks/rust-active-effect-tick-2026-10-07/scoped-parity.json` | Reference; Version-specific | Reference records include client/server JAR hashes, output hashes and explicit Rust-output hashes; the recorded… |
| 197 | `benchmarks/rust-active-effects-2026-10-07/scoped-parity.json` | Reference; Version-specific | Reference records include client/server JAR hashes, output hashes and explicit Rust-output hashes; the recorded… |
| 198 | `benchmarks/rust-anchor-2026-10-06/scoped-parity.json` | Reference; Version-specific | Client/server reference records have matching output hashes, but the `references` records do not include an… |
| 199 | `benchmarks/rust-attribute-health-2026-10-07/scoped-parity.json` | Reference; Version-specific | Client/server reference records have matching output hashes, but the `references` records do not include an… |
| 200 | `benchmarks/rust-attribute-map-queries-2026-10-07/scoped-parity.json` | Reference; Version-specific | Reference records include client/server JAR hashes, output hashes and explicit Rust-output hashes; the recorded… |

## Part 10 — Entries 201–225

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 201 | `benchmarks/rust-attribute-memory-2026-10-07/after.json` | Reference; Adaptable measurement method; Version-specific baseline | گزارش x86_64 لینوکس است و scope صریحاً world/entity کامل، زمان، رندر و FPS را خارج می‌کند؛ با before مقایسه می‌شود،… |
| 202 | `benchmarks/rust-attribute-memory-2026-10-07/before.json` | Reference; Adaptable measurement method; Version-specific baseline | مقایسهٔ before/after به تغییر یک منبع entity attributes اشاره دارد؛ این اعداد RSS کل process هستند و شامل… |
| 203 | `benchmarks/rust-attribute-memory-2026-10-07/entity-regression.json` | Adaptable test methodology; Version-specific data | نسخه 26.3؛ bulk assignment، supplierهای سفارشی، save/network codec و entity زنده خارج از دامنه‌اند |
| 204 | `benchmarks/rust-attribute-memory-2026-10-07/health-regression.json` | Adaptable test methodology; Version-specific data | 26.3؛ absorption synchronization، damage کامل و gameplay زنده هنوز خارج از دامنه‌اند |
| 205 | `benchmarks/rust-attribute-updates-2026-10-07/scoped-parity.json` | Adaptable test methodology; Version-specific data; Evidence gap | 26.3؛ هر دو reference hash یکسان دارند، اما فیلد top-level به نام rust_output_sha256 وجود ندارد؛ پیوند digest نهایی… |
| 206 | `benchmarks/rust-attributes-2026-10-07/scoped-parity.json` | Adaptable test methodology; Version-specific data | 26.3؛ attribute registry/entity/equipment integration و packed data/network codecs جداگانه باقی‌اند؛ client/server… |
| 207 | `benchmarks/rust-bamboo-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific feature behavior | 26.3؛ heightها کنترل‌شده‌اند و پس از write به‌روزرسانی نمی‌شوند؛ world updates و generation کامل خارج از دامنه‌اند |
| 208 | `benchmarks/rust-basic-feature-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific feature behavior | 26.3؛ chunk mutation واقعی، neighbor updates، lighting، entity/block drops و complete generated worlds بیرون دامنه‌اند |
| 209 | `benchmarks/rust-biome-chunk-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific data | 26.3؛ identityهای holder مصنوعی‌اند؛ native palette layout، wire encoding، biome generation، blending و retrogen… |
| 210 | `benchmarks/rust-biome-generation-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific data | 26.3؛ holderها identity placeholder هستند؛ preset builder، full biome-source codec، blending، retrogen و complete… |
| 211 | `benchmarks/rust-biome-placement-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific data | 26.3؛ biome registry equality، complete decoration و gameplay باقی‌اند؛ خروجی client/server با Rust hash برابر است |
| 212 | `benchmarks/rust-biome-source-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific data | 26.3؛ End density ورودی ثابت است؛ واقعیات seeded density، codec، registry/preset و world integration جدا مانده‌اند |
| 213 | `benchmarks/rust-biome-source-codec-2026-10-06/scoped-parity.json` | Adaptable codec-testing methodology; Version-specific schema | 26.3؛ full inline biome definitions و live registry reload خارج دامنه‌اند؛ client/server hashها با Rust خروجی منطبق‌اند |
| 214 | `benchmarks/rust-biome-temperature-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific behavior | 26.3؛ weather placement/freezing کامل و auto-migration بین threadها خارج scope است |
| 215 | `benchmarks/rust-biome-weather-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific data | 26.3؛ dynamic block entities، lighting propagation، mutation و complete weather simulation خارج scope هستند |
| 216 | `benchmarks/rust-biome-zoom-2026-10-06/scoped-parity.json` | Adaptable algorithmic test; Version-specific seed/data | 26.3؛ storage/source generation، blending و chunk integration خارج scope هستند |
| 217 | `benchmarks/rust-block-blob-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific feature data | 26.3؛ Rust یک work cap دارد و resident-world updates و complete gameplay خارج scope هستند |
| 218 | `benchmarks/rust-block-collision-scan-2026-10-06/scoped-parity.json` | Adaptable algorithmic test; Version-specific shape data | 26.3؛ registry-specific shape dispatch، live chunk loading و entity movement خارج scope‌اند |
| 219 | `benchmarks/rust-block-column-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific feature behavior | 26.3؛ actual chunks، neighbor updates و complete gameplay خارج scope؛ bounded work limit در Rust اعمال شده است |
| 220 | `benchmarks/rust-block-entity-storage-2026-10-06/scoped-parity.json` | Adaptable storage pattern; Version-specific NBT/data | 26.3؛ ساخت entity از NBT، serialization کامل، active behavior و chunk lifecycle بیرون scope‌اند |
| 221 | `benchmarks/rust-block-landing-2026-10-07/scoped-parity.json` | Adaptable test methodology; Version-specific block behavior | 26.3؛ health واقعی، entity storage، network/audio و movement lifecycle کامل خارج دامنه‌اند |
| 222 | `benchmarks/rust-block-pile-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific feature data | 26.3؛ work limit اعمال شده؛ block entities، resident chunks، block updates و gameplay کامل pending هستند |
| 223 | `benchmarks/rust-block-properties-2026-10-06/scoped-parity.json` | Adaptable data-validation method; Version-specific block registry | 26.3؛ item/fluid replacement، support، survival، dynamic collision و gameplay کامل بیرون scope است |
| 224 | `benchmarks/rust-block-push-2026-10-07/scoped-parity.json` | Adaptable algorithmic test; Version-specific collision data | 26.3؛ collision shapes/entity selections کنترل‌شده‌اند؛ live entity indexing و teleport/network behavior خارج scope است |
| 225 | `benchmarks/rust-block-state-codec-2026-10-06/scoped-parity.json` | Adaptable codec structure; Version-specific registry schema | 26.3؛ registry metadata از دو JAR آمده؛ warning text و lifecycle/complete material codecs pending |

## Part 11 — Entries 226–250

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 226 | `benchmarks/rust-block-support-2026-10-06/scoped-parity.json` | Adaptable algorithmic test; Version-specific shape data | 26.3؛ dynamic support، survival و complete gameplay خارج scope است |
| 227 | `benchmarks/rust-block-survival-2026-10-06/scoped-parity.json` | Adaptable test methodology; Version-specific rule catalogue | 26.3؛ full placement/update، world/entity lifecycle و complete gameplay pending هستند |
| 228 | `benchmarks/rust-blocking-item-damage-2026-10-07/scoped-parity.json` | Reference/Adaptable algorithm; Version-specific item model | 26.3؛ components/codecs، enchantment effects، inventory/equipment lifecycle و combat کامل pending |
| 229 | `benchmarks/rust-blocking-rules-2026-10-07/scoped-parity.json` | Adaptable test method; Version-specific combat rules | 26.3؛ direction/acos، component codecs، durability، sounds و complete blocking dispatch خارج scope |
| 230 | `benchmarks/rust-blockstate-model-binding-2026-10-06/scoped-parity.json` | Adaptable parser/binding tests; Version-specific resource data | 26.3؛ no weighted random choice، transformed bake، renderer integration یا gameplay parity |
| 231 | `benchmarks/rust-blockstate-model-binding-2026-10-06/shared-integer-codec.json` | Reference/Adaptable parser technique; Version-specific inputs | 26.3؛ این فایل evidence مستقل از full state-codec path است، نه مجوز استفاده از registry IDs نسخهٔ جدید |
| 232 | `benchmarks/rust-blockstate-model-choice-2026-10-06/scoped-parity.json` | Adaptable random-selection tests; Version-specific model weights | 26.3؛ leaf models inert و predicateها از پیش تعیین‌شده‌اند؛ baking، material evaluation و renderer خارج scope است |
| 233 | `benchmarks/rust-blockstate-model-decoding-2026-10-06/scoped-parity.json` | Adaptable parser tests; Version-specific resource format | 26.3؛ registry binding، property selection، RNG choice، transformed baking و rendered output بیرون scope است |
| 234 | `benchmarks/rust-blockstate-transforms-2026-10-06/scoped-parity.json` | Adaptable geometry test; Version-specific model data | 26.3؛ atlas packing، shader pixels، lighting، render classification و runtime renderer integration بررسی نشده‌اند |
| 235 | `benchmarks/rust-blue-ice-2026-10-06/scoped-parity.json` | Adaptable test method; Version-specific feature data | 26.3؛ chunk updates، fluid/ice gameplay و full worldgen خارج scope است؛ Rust work bounds وجود دارد |
| 236 | `benchmarks/rust-bonus-chest-2026-10-06/scoped-parity.json` | Adaptable test method; Version-specific worldgen data | 26.3؛ loot generation، inventory، automatic block-entity lifecycle و complete spawn generation pending است |
| 237 | `benchmarks/rust-bulk-section-2026-10-06/scoped-parity.json` | Adaptable storage/cache pattern; Version-specific chunk layout | 26.3؛ Rust exclusive region borrow دارد؛ concurrent ownership، ordinary ore geometry و region side effects خارج scope‌اند |
| 238 | `benchmarks/rust-bundle-weight-2026-10-07/scoped-parity.json` | Reference only; Modern item-component schema not portable | مدل bundle/data components جدید 26.3 است؛ 1.20.1 vanilla item stack مدل legacy NBT دارد و معادل مستقیم ندارد |
| 239 | `benchmarks/rust-canonical-collision-2026-10-06/contextual-regression.json` | Adaptable collision-test methodology; Version-specific registry data | 26.3؛ active shulker/piston و entity lifecycle جدا؛ در scope، ۱۹۹ uncached و ۳۲ context-sensitive states مشخص‌اند |
| 240 | `benchmarks/rust-canonical-collision-2026-10-06/scoped-parity.json` | Adaptable geometry/cache tests; Version-specific state data | 26.3؛ uncached و entity-context behavior به suiteهای جدا موکول شده؛ نتیجه کامل collision engine نیست |
| 241 | `benchmarks/rust-canyon-carver-2026-10-06/scoped-parity.json` | Adaptable algorithmic tests; Version-specific carver data | 26.3؛ gaussian/weighted providers، neighbor-carver seeding، biome selection و full worlds خارج scope؛ Rust bounds کار دارد |
| 242 | `benchmarks/rust-carving-geometry-2026-10-06/scoped-parity.json` | Adaptable primitive tests; Version-specific consumers | فقط primitives مشترک هندسی را پوشش می‌دهد؛ randomized cave/canyon paths، codecs و provider sampling را ثابت نمی‌کند |
| 243 | `benchmarks/rust-carving-mask-2026-10-06/scoped-parity.json` | Adaptable data-structure test; Version-specific height layout | 26.3؛ تست mask به‌تنهایی cave/canyon generation، mask application یا terrain generation را اثبات نمی‌کند |
| 244 | `benchmarks/rust-cave-carver-2026-10-06/scoped-parity.json` | Adaptable algorithmic tests; Version-specific carver data | 26.3؛ gaussian/weighted provider codecs، neighbor biome selection، blending و complete worldgen بیرون scope‌اند |
| 245 | `benchmarks/rust-chorus-plant-2026-10-06/scoped-parity.json` | Adaptable block-growth test; Version-specific block logic | 26.3؛ neighbor-update execution، random ticks، drops، rendering و full End generation خارج scope است |
| 246 | `benchmarks/rust-chunk-stage-2026-10-06/scoped-parity.json` | Adaptable chunk-state tests; Version-specific storage model | 26.3؛ scheduler، persisted metadata، retrogen، block entities، lighting و complete world خارج scope است |
| 247 | `benchmarks/rust-chunk-status-2026-10-06/scoped-parity.json` | Adaptable codec/order tests; Version-specific status set | 26.3؛ این manifest generation-step execution یا lighting را آزمایش نمی‌کند |
| 248 | `benchmarks/rust-climbing-2026-10-07/scoped-parity.json` | Adaptable behavior tests; Version-specific tag/state data | 26.3؛ live movement binding، equipment-based glide eligibility و full gameplay pending است |
| 249 | `benchmarks/rust-collision-2026-10-05/scoped-parity.json` | Reference/Adaptable primitive tests; Version-specific | نسخه 26.3؛ uncached states و entity-context در این manifest پشتیبانی نمی‌شوند |
| 250 | `benchmarks/rust-collision-context-2026-10-06/scoped-parity.json` | Adaptable context tests; Version-specific entity semantics | 26.3؛ entity lifecycle، inventory mutation بعد از capture، minecart experimental contexts و live movement pending |

## Part 12 — Entries 251–275

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 251 | `benchmarks/rust-combat-rules-2026-10-07/scoped-parity.json` | Adaptable arithmetic tests; Version-specific combat rules | 26.3؛ enchantment armor hook، enchanted components، damage tags/equipment و pipeline کامل باقی‌اند |
| 252 | `benchmarks/rust-component-durability-2026-10-07/blocking-regression.json` | Reference/Adaptable regression harness; Version-specific item model | این گزارش جدا از entry 228 است اما scope و خروجی هم‌پوشانی دارند؛ component codec، enchantment و full inventory… |
| 253 | `benchmarks/rust-component-durability-2026-10-07/scalar-regression.json` | Adaptable numeric test; Modern data model not portable | 26.3؛ no-enchantment path؛ full component store/codec، enchantment و equipment handling خارج scope است |
| 254 | `benchmarks/rust-component-durability-2026-10-07/scoped-parity.json` | Reference/Adaptable storage idea; Version-specific schema | 26.3 component architecture است؛ full item-component codecs، enchantments و equipment lifecycle pending هستند |
| 255 | `benchmarks/rust-component-patch-2026-10-07/scoped-parity.json` | Reference only; 1.20.1 has legacy item data | 26.3 data component system؛ prototype hash-table construction، identity hashes، mutable aliases و item/inventory… |
| 256 | `benchmarks/rust-consumable-2026-10-07/scoped-parity.json` | Adaptable lifecycle test; Version-specific item/effect model | 26.3؛ eligibility/start-use، stat service، effect bodies و event delivery کنترل‌شده‌اند؛ full lifecycle/gameplay pending |
| 257 | `benchmarks/rust-consumable-emission-2026-10-07/scoped-parity.json` | Adaptable event-order test; Version-specific assets/API | 26.3؛ audio playback، rendering، completion/effects و full gameplay خارج scope‌اند |
| 258 | `benchmarks/rust-consume-effect-2026-10-07/scoped-parity.json` | Adaptable effect sequencing; Version-specific effect catalogue | 26.3؛ actor storage و sound delivery کنترل‌شده‌اند؛ holder binding، teleport handling و integration کامل pending |
| 259 | `benchmarks/rust-contextual-collision-2026-10-06/scoped-parity.json` | Reference/Adaptable collision regression; Version-specific state map | هم‌پوشانی عمدی با entry 239 دارد؛ دو manifest را یکی فرض نکنید، اما رفتارهای مشترک را دوباره پیاده‌سازی نکنید |
| 260 | `benchmarks/rust-core-2026-10-05/scoped-parity.json` | Adaptable low-level test corpus; Version-specific algorithms | 26.3؛ انتخابی و غیرجامع است؛ gameplay، rendering و performance را اثبات نمی‌کند |
| 261 | `benchmarks/rust-damage-immunity-2026-10-07/scoped-parity.json` | Adaptable logic tests; Version-specific damage rules | 26.3؛ flags/tags/rules کنترل‌شده و enchantment equipment خالی است؛ registered source/rule binding و full pipeline pending |
| 262 | `benchmarks/rust-damage-reduction-2026-10-07/scoped-parity.json` | Adaptable arithmetic tests; Version-specific combat behavior | 26.3؛ enchantments، durability effects، live registries و full damage integration باقی‌اند |
| 263 | `benchmarks/rust-damage-source-2026-10-07/scoped-parity.json` | Adaptable schema/test method; Version-specific registry | 26.3؛ catalog indices محلی‌اند نه network IDs؛ live registry reload، Rust codec و death messages pending |
| 264 | `benchmarks/rust-decoration-2026-10-06/scoped-parity.json` | Adaptable worldgen orchestration tests; Version-specific biome/feature data | 26.3؛ structures disabled و feature payloadها synthetic هستند؛ full registry/world lifecycle خارج scope |
| 265 | `benchmarks/rust-decoration-region-2026-10-06/scoped-parity.json` | Adaptable integration harness; Version-specific region semantics | 26.3؛ WorldGenRegion policy، POI، block entities، lighting، structures و full terrain lifecycle خارج scope |
| 266 | `benchmarks/rust-delta-2026-10-06/scoped-parity.json` | Adaptable feature test; Version-specific provider/state data | 26.3؛ world updates، fluid behavior و full generation خارج scope‌اند |
| 267 | `benchmarks/rust-density-2026-10-05/scoped-parity.json` | Adaptable architecture/test methodology; Version-specific density graph | manifest فیلد صریح minecraft ندارد؛ manifest_sha1 پین 26.3 را مشخص می‌کند. Blend غیرخالی، باقی graph definitions،… |
| 268 | `benchmarks/rust-disk-2026-10-06/scoped-parity.json` | Adaptable feature/storage test; Version-specific height/state data | 26.3؛ manifest درخواست mark را ثبت می‌کند، نه اجرای کامل post-processing بعدی؛ full generation pending |
| 269 | `benchmarks/rust-dynamic-support-2026-10-06/scoped-parity.json` | Adaptable dynamic-shape tests; Version-specific block/entity rules | 26.3؛ entity lifecycle، ticking/pushing و full gameplay خارج scope‌اند |
| 270 | `benchmarks/rust-edge-backoff-2026-10-07/scoped-parity.json` | Adaptable movement test; Version-specific player mechanics | 26.3؛ live crouch state، attribute provisioning و integrated ledge behavior خارج scope هستند |
| 271 | `benchmarks/rust-effect-attribute-modifiers-2026-10-07/scoped-parity.json` | Adaptable modifier test; Version-specific effects/attributes | 26.3؛ effect registry definitions، pending sets ownership، notifications و full gameplay جدا باقی‌اند |
| 272 | `benchmarks/rust-effect-notifications-2026-10-07/scoped-parity.json` | Adaptable notification ordering; Version-specific packet/effect schema; Evidence gap | manifest فیلد top-level rust_output_sha256 ندارد؛ هر دو JAR reference خروجی یکسان و ۱٬۶۶۷٬۶۱۶ matched lines دارند،… |
| 273 | `benchmarks/rust-effect-sync-2026-10-07/scoped-parity.json` | Adaptable sync-state testing; Version-specific serializer/effect schema; Evidence gap | فیلد top-level rust_output_sha256 وجود ندارد؛ دو reference خروجی برابر با ۲٬۷۵۴٬۷۴۴ matched lines ثبت کرده‌اند، اما… |
| 274 | `benchmarks/rust-effect-sync-2026-10-07/synced-bytes-regression.json` | Adaptable state-sync test; Version-specific serializer IDs | 26.3؛ heterogeneous serializer maps، entity schema/network codecs و complete item-use flow خارج scope است |
| 275 | `benchmarks/rust-effect-sync-2026-10-07/synced-floats-regression.json` | Adaptable state-sync test; Version-specific serializer IDs | 26.3؛ سایر serializers، type mismatch handling، wire codecs و player absorption integration pending |

## Part 13 — Entries 276–300

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 276 | `benchmarks/rust-end-gateway-2026-10-06/scoped-parity.json` | Adaptable feature/block-entity test; Version-specific End behavior | 26.3؛ entity lifecycle، dirty propagation، teleport/search و complete End generation خارج scope‌اند |
| 277 | `benchmarks/rust-end-podium-2026-10-06/scoped-parity.json` | Adaptable feature test; Version-specific End worldgen | 26.3؛ destruction outcome کنترل‌شده است؛ drops، block entities، neighbor updates و dragon/portal gameplay خارج scope‌اند |
| 278 | `benchmarks/rust-entity-attributes-2026-10-07/scoped-parity.json` | Reference/Adaptable regression; Version-specific data | 26.3؛ bulk assignment، custom suppliers، save/network codecs، entity live و gameplay pending |
| 279 | `benchmarks/rust-entity-collision-2026-10-07/scoped-parity.json` | Adaptable collision query test; Version-specific entity rules | 26.3؛ snapshots و vehicle links کنترل‌شده‌اند؛ live storage، broad-phase indexing و entity lifecycle خارج scope هستند |
| 280 | `benchmarks/rust-entity-fluid-2026-10-07/scoped-parity.json` | Adaptable fluid/entity test; Version-specific fluid tags/physics | 26.3؛ height/flow و camera-support کنترل‌شده؛ live fluid propagation/world integration pending |
| 281 | `benchmarks/rust-entity-fluid-tick-2026-10-07/immersion-regression.json` | Reference/Adaptable regression; Version-specific behavior | این regression به entry280 هم‌پوشانی دارد؛ مستقل ثبت شده اما نباید به‌عنوان قابلیت دوم یا پوشش کامل fluid tick شمرده شود |
| 282 | `benchmarks/rust-entity-fluid-tick-2026-10-07/scoped-parity.json` | Adaptable tick integration test; Version-specific fluid behavior | 26.3؛ splash render/audio و live world/entity binding و gameplay کامل خارج scope است |
| 283 | `benchmarks/rust-entity-move-2026-10-07/scoped-parity.json` | Adaptable movement test; Version-specific collision/motion | 26.3؛ step height صفر و بسیاری از world providers کنترل‌شده‌اند؛ full player/world movement integration باقی است |
| 284 | `benchmarks/rust-entity-obstruction-2026-10-06/scoped-parity.json` | Adaptable collision query test; Version-specific entity rules | 26.3؛ entity construction/lifecycle، spatial indexing، vehicle maintenance و complete gameplay خارج scope‌اند |
| 285 | `benchmarks/rust-extra-items-2026-10-07/scoped-parity.json` | Adaptable inventory lifecycle test; Version-specific ItemStack model | 26.3؛ item handlers، full component schemas و actual dropped-entity creation/lifecycle pending |
| 286 | `benchmarks/rust-face-support-2026-10-06/scoped-parity.json` | Adaptable geometry tests; Version-specific shape semantics | 26.3؛ انتخاب support shape برای هر block، world/entity context و survival خارج scope است |
| 287 | `benchmarks/rust-face-visibility-2026-10-06/scoped-parity.json` | Adaptable face-culling tests; Version-specific model/tag data | 26.3؛ chunk mesh integration، lighting، translucent sorting، fluid surfaces، GPU pixels و gameplay ثابت نشده‌اند |
| 288 | `benchmarks/rust-fall-damage-2026-10-07/scoped-parity.json` | Adaptable physics test; Version-specific entity/block hooks | 26.3؛ LivingEntity fluid updates، Block.fallOn، health/damage/attributes و full movement lifecycle در این تست نیست |
| 289 | `benchmarks/rust-feature-catalog-2026-10-06/scoped-parity.json` | Adaptable registry/codec test; Version-specific worldgen registry | 26.3؛ unused/cyclic registry resources، reload/lifecycle، encoding کامل، همهٔ feature algorithms و generated worlds… |
| 290 | `benchmarks/rust-feature-graph-2026-10-06/scoped-parity.json` | Adaptable graph execution tests; Version-specific codecs | 26.3؛ named registry holders/tags، encoders، full feature registries و complete generated worlds pending |
| 291 | `benchmarks/rust-feature-placement-2026-10-06/scoped-parity.json` | Adaptable placement pipeline test; Version-specific modifiers | 26.3؛ configured feature algorithms, registry, nested features, Gaussian providers, encoding و complete worldgen خارج scope |
| 292 | `benchmarks/rust-feature-region-2026-10-06/scoped-parity.json` | Adaptable worldgen integration harness; Version-specific region behavior | 26.3؛ storage proxy جای WorldGenRegion واقعی است؛ write-zone، POI، block entities، postprocessing، lighting و… |
| 293 | `benchmarks/rust-feature-region-destroy-2026-10-06/scoped-parity.json` | Adaptable region mutation test; Version-specific chunk/tick policy | 26.3؛ WorldGenRegion واقعی، POI، block entity lifecycle، lighting و اجرای واقعی tickها خارج scope هستند |
| 294 | `benchmarks/rust-feature-region-queries-2026-10-06/scoped-parity.json` | Adaptable world proxy test; Version-specific block/fluid data | 26.3؛ actual WorldGenRegion، postprocessing execution، lighting و complete generation هنوز بیرون scope است |
| 295 | `benchmarks/rust-feature-region-ticks-2026-10-06/scoped-parity.json` | Adaptable tick queue test; Version-specific scheduler semantics | 26.3؛ tick execution، WorldGenRegion واقعی، lighting و complete feature registry خارج scope است |
| 296 | `benchmarks/rust-feature-region-upgrade-height-2026-10-06/scoped-parity.json` | Adaptable height-gate tests; Version-specific height/retrogen | 26.3؛ retrogen activation caller-owned است؛ full retrogen replacement/masking، biome behavior و complete worlds… |
| 297 | `benchmarks/rust-feature-region-write-zone-2026-10-06/scoped-parity.json` | Adaptable boundary-condition tests; Version-specific region rules | 26.3؛ vertical upgrade restriction، region construction، POIs, lighting, ticks و full worldgen بیرون scope‌اند |
| 298 | `benchmarks/rust-feature-sort-2026-10-06/scoped-parity.json` | Adaptable graph-ordering algorithm; Version-specific feature graph | 26.3؛ diagnostic minimization، complete registry ownership، decoration seeds و feature execution خارج scope‌اند |
| 299 | `benchmarks/rust-fill-layer-2026-10-06/scoped-parity.json` | Adaptable feature test; Version-specific block states | 26.3؛ world side effects و other configured features/full worlds خارج scope‌اند |
| 300 | `benchmarks/rust-float-log-2026-10-05/scoped-parity.json` | Adaptable exhaustive numeric test; Version-specific runtime math | تست فقط scalar float log روی یک JDK/platform است؛ double Math.log، دیگر platformها، bulk composition، terrain یا… |

## Part 14 — Entries 301–325

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 301 | `benchmarks/rust-fluid-2026-10-06/scoped-parity.json` | Adaptable codec/query tests; rebuild all version data. | Registered state inventory is from 26.3; fluid spreading, live world binding and gameplay are absent. |
| 302 | `benchmarks/rust-fluid-box-2026-10-07/scoped-parity.json` | Adaptable geometry/callback tests; rewrite vehicle semantics. | Passenger lifecycle and actual live boat/world integration are controlled or absent. |
| 303 | `benchmarks/rust-fluid-flow-2026-10-07/scoped-parity.json` | Adaptable flow algorithm tests; rebuild registry/tag fixtures. | World registry/provider binding, propagation and spreading are not covered. |
| 304 | `benchmarks/rust-fluid-occupancy-2026-10-07/scoped-parity.json` | Adaptable chunk-scan tests; port 1.20.1 storage rules. | Resident snapshots are controlled; live entity collision composition and world integration remain open. |
| 305 | `benchmarks/rust-fluid-shape-2026-10-07/scoped-parity.json` | Adaptable cache and shape tests; rebuild fluid states. | Fluid propagation, immersion and ray/world binding are outside scope. |
| 306 | `benchmarks/rust-food-2026-10-07/scoped-parity.json` | Adaptable food-state tests; rewrite player tick integration. | Player damage/health services, consumables, persistence and complete player tick are not integrated. |
| 307 | `benchmarks/rust-food-properties-2026-10-07/scoped-parity.json` | Adaptable consumption callback tests; rebuild item registry integration. | Sound playback, concrete consumable dispatch, particles/effects and full use lifecycle are omitted. |
| 308 | `benchmarks/rust-food-storage-2026-10-07/scoped-parity.json` | Adaptable persistence tests; rewrite 1.20.1 NBT semantics. | Only FoodData fields are covered; complete player save lifecycle and other fields are outside scope. |
| 309 | `benchmarks/rust-free-space-2026-10-07/collision-regression.json` | Adaptable collision regression; rebuild world/query integration. | Entity lists, border conditions and chunk snapshots are controlled; full live entity indexing is absent. |
| 310 | `benchmarks/rust-free-space-2026-10-07/scoped-parity.json` | Adaptable composition tests; rewrite version-specific shapes. | Chunk generation, live entity indexing and complete gameplay are not represented. |
| 311 | `benchmarks/rust-generation-ticks-2026-10-06/scoped-parity.json` | Adaptable scheduler design; rewrite 1.20.1 tick lifecycle. | This records queue behavior, not execution of actual block/fluid ticks or complete generation. |
| 312 | `benchmarks/rust-geometry-2026-10-05/scoped-parity.json` | Adaptable geometry algorithms/tests; verify float semantics for target toolchain. | Selected operations are tested in isolation; not a full world collision or camera pipeline. |
| 313 | `benchmarks/rust-geometry-2026-10-06/scoped-parity.json` | Adaptable geometry tests; rebuild target-specific behavior. | Only selected primitives and controlled cases; complete collision/shape consumers are not included. |
| 314 | `benchmarks/rust-ground-support-2026-10-07/scan-regression.json` | Adaptable collision scan tests; rewrite world providers. | Controlled providers and contexts; registered 1.20.1 shapes, live loading and full entity movement remain untested. |
| 315 | `benchmarks/rust-ground-support-2026-10-07/scoped-parity.json` | Adaptable support-query cases; evidence gap for direct parity. | No top-level Rust output digest is present; provider/registry integration is controlled. |
| 316 | `benchmarks/rust-height-provider-2026-10-06/scoped-parity.json` | Adaptable provider algorithms; rebuild codecs and registries. | Warning logs and complete generation placement are excluded; 26.3 codec data is version-specific. |
| 317 | `benchmarks/rust-heightmap-2026-10-06/scoped-parity.json` | Adaptable packed-heightmap tests; rebuild 1.20.1 state table. | Imported air/fluid facts do not port block behavior; full chunk lifecycle and generation are not covered. |
| 318 | `benchmarks/rust-interaction-shape-2026-10-07/scoped-parity.json` | Adaptable shape abstraction; rewrite registry/shape table. | Shape table belongs to 26.3; world ray binding and complete gameplay are excluded. |
| 319 | `benchmarks/rust-inventory-pickup-2026-10-07/scoped-parity.json` | Adaptable inventory algorithms; rebuild ItemStack model. | Pickup entities, menus, networking, arbitrary overrides and full item components are not included. |
| 320 | `benchmarks/rust-inventory-tick-2026-10-07/scoped-parity.json` | Adaptable tick-order tests; rewrite 1.20.1 item lifecycle. | Concrete registered item effects and the enclosing player/world tick are pending. |
| 321 | `benchmarks/rust-inventory-use-2026-10-07/scoped-parity.json` | Adaptable lifecycle sequencing; rebuild item-use states. | Does not cover full start-use path, concrete consumable effects, packet delivery or complete Player.tick. |
| 322 | `benchmarks/rust-inventory-use-start-2026-10-07/scoped-parity.json` | Adaptable input-to-use flow; rewrite packet and item schema. | Packet delivery, combat effects and full tick/network lifecycle are outside scope. |
| 323 | `benchmarks/rust-item-after-use-2026-10-07/remainder-regression.json` | Adaptable regression suite; rebuild component schema. | Actual item insertion/drop, full component codec and concrete use effects remain outside scope. |
| 324 | `benchmarks/rust-item-after-use-2026-10-07/scoped-parity.json` | Adaptable item-use tests; reimplement 1.20.1 semantics. | Concrete consumables, complete inventory transactions and whole item-use lifecycle are not covered. |
| 325 | `benchmarks/rust-item-blocking-2026-10-07/scoped-parity.json` | Adaptable blocking algorithm; rebuild item/damage rules. | Real damageable item components, full combat and death flow are not integrated. |

## Part 15 — Entries 326–350

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 326 | `benchmarks/rust-item-cooldowns-2026-10-07/scoped-parity.json` | Adaptable cooldown state machine; rewrite protocol integration. | Network synchronization and complete player/item lifecycle are absent. |
| 327 | `benchmarks/rust-item-durability-2026-10-07/scoped-parity.json` | Adaptable durability tests; rebuild 1.20.1 component definitions. | Full component codecs, enchantments and complete gameplay are not covered. |
| 328 | `benchmarks/rust-item-particles-2026-10-07/scoped-parity.json` | Adaptable event/request model; rebuild particle registry. | Particle queues/rendering, sound/effects integration and real gameplay are outside scope. |
| 329 | `benchmarks/rust-item-stack-storage-2026-10-07/scoped-parity.json` | Adaptable value semantics; rebuild ItemStack for 1.20.1. | Component codecs, menu transactions, equipment synchronization and full use flow are not included. |
| 330 | `benchmarks/rust-item-template-2026-10-07/scoped-parity.json` | Adaptable template validation; rewrite component registry. | Does not exercise every component codec or complete containers and use lifecycle. |
| 331 | `benchmarks/rust-item-use-2026-10-07/scoped-parity.json` | Adaptable lifecycle tests; rebind to 1.20.1 entity tick. | Start-use path, synchronized flags, concrete consumables and full lifecycle remain pending. |
| 332 | `benchmarks/rust-item-use-catalog-2026-10-07/scoped-parity.json` | Reference catalog only; rebuild from 1.20.1 registries. | Import is 26.3-specific; other component codecs, inventory, enchantments and use effects remain unverified. |
| 333 | `benchmarks/rust-item-use-lifecycle-2026-10-07/scoped-parity.json` | Adaptable lifecycle sequencing; rewrite against 1.20.1. | Starting use, actual synchronized flags, concrete consumables, remainders and complete tick are incomplete. |
| 334 | `benchmarks/rust-item-use-start-2026-10-07/scoped-parity.json` | Adaptable dispatch tests; rewrite protocol and inventory sync. | Transport, wire encoding, persistent data and server/client lifecycle are not exercised. |
| 335 | `benchmarks/rust-java-hash-map-2026-10-07/scoped-parity.json` | Reference only; use Rust-native collections where justified. | JDK collection parity is not game logic; JDK version/platform semantics must not be assumed. |
| 336 | `benchmarks/rust-java-identity-map-2026-10-07/cooldown-regression.json` | Reference regression; validate only required semantics. | This is a Java/JDK regression context, not a complete in-game cooldown lifecycle. |
| 337 | `benchmarks/rust-java-identity-map-2026-10-07/ordered-map-regression.json` | Reference regression; avoid copying JDK internals blindly. | It is a JDK implementation comparison, not Minecraft feature parity. |
| 338 | `benchmarks/rust-java-identity-map-2026-10-07/scoped-parity.json` | Adaptable collection tests; evidence gap for aggregate Rust output. | No top-level rust_output_sha256; JAR references have one matching output digest, but process identity inputs remain environment-dependent. |
| 339 | `benchmarks/rust-liquid-collision-2026-10-06/scoped-parity.json` | Adaptable collision cases; rebuild fluid registry and shapes. | Does not implement entity movement or fluid propagation in a live world. |
| 340 | `benchmarks/rust-live-attribute-health-2026-10-07/scoped-parity.json` | Adaptable attribute/health composition; rewrite 1.20.1 attributes. | Live refresh paths and complete damage/player tick are not fully integrated. |
| 341 | `benchmarks/rust-live-attribute-updates-2026-10-07/scoped-parity.json` | Adaptable notification ordering; evidence gap and rewrite queue ownership. | No top-level Rust output digest; client/server reference outputs differ, so not a single equivalent aggregate. |
| 342 | `benchmarks/rust-live-attributes-2026-10-07/scoped-parity.json` | Adaptable data model; rebuild 1.20.1 attribute registry. | No top-level Rust output digest; client/server outputs differ; complete entity supplier lifecycle remains open. |
| 343 | `benchmarks/rust-live-player-combat-2026-10-07/scoped-parity.json` | Adaptable combat composition; full server lifecycle rewrite required. | Several knockback, criterion, source and world lifecycle consumers remain controlled; output has separate Rust/live hashes. |
| 344 | `benchmarks/rust-live-upgrade-region-2026-10-06/scoped-parity.json` | Adaptable region harness; rewrite version-specific worldgen. | Proxy is not a real WorldGenRegion; complete chunks/worldgen, POI, light and postprocessing are pending. |
| 345 | `benchmarks/rust-living-air-2026-10-07/scoped-parity.json` | Adaptable physics equations; rewrite player/entity hooks. | Full Player.travel and live effects/world providers are not integrated. |
| 346 | `benchmarks/rust-living-damage-2026-10-07/scoped-parity.json` | Adaptable damage-state tests; rebuild damage sources/attributes. | Damage service composition and full world/player lifecycle remain controlled. |
| 347 | `benchmarks/rust-living-fall-2026-10-07/scoped-parity.json` | Adaptable numeric/callback tests; rewrite block/entity hooks. | Actual world landing callbacks, enchantment hooks and complete damage application are not included. |
| 348 | `benchmarks/rust-living-fluid-2026-10-07/scoped-parity.json` | Adaptable fluid physics tests; rebuild tags and movement. | Fluid propagation, full vehicle logic and playable world integration remain pending. |
| 349 | `benchmarks/rust-living-health-2026-10-07/scoped-parity.json` | Adaptable health storage tests; rebuild entity schema. | Full registered attribute binding, damage sources and complete player lifecycle are not covered. |
| 350 | `benchmarks/rust-living-hurt-2026-10-07/scoped-parity.json` | Adaptable server damage flow; rewrite version-specific hooks. | Totems, criteria, blocking, source service and enclosing server tick are not fully integrated. |

## Part 16 — Entries 351–375

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 351 | `benchmarks/rust-living-landing-2026-10-07/scoped-parity.json` | Adaptable fall-damage composition; rebuild 1.20.1 block hooks. | Full fluid immersion, enchantment hooks, particles rendering and Player/world integration remain pending. |
| 352 | `benchmarks/rust-living-motion-2026-10-07/scoped-parity.json` | Adaptable movement formulas; rewrite against 1.20.1 physics. | Live overrides, effects and complete Player/entity integration are not included. |
| 353 | `benchmarks/rust-material-condition-2026-10-06/scoped-parity.json` | Adaptable rule engine tests; rebuild 1.20.1 rule data. | Caller-owned context/data and complete worldgen registry/encoding are not included. |
| 354 | `benchmarks/rust-material-extension-2026-10-06/scoped-parity.json` | Adaptable extension architecture; rewrite biome/worldgen data. | Thresholds and callbacks are controlled; does not establish complete biome or chunk generation. |
| 355 | `benchmarks/rust-material-rule-library-2026-10-06/scoped-parity.json` | Adaptable registry/rule design; rebuild target-specific rules. | Encoding, reload, registry validation and full generated worlds are not covered. |
| 356 | `benchmarks/rust-material-stage-2026-10-06/scoped-parity.json` | Adaptable surface-stage test; rewrite 1.20.1 surface rules. | Only bounded 3x3 biome input and generated terrain fixtures; full chunk/biome pipeline is absent. |
| 357 | `benchmarks/rust-material-surface-2026-10-06/scoped-parity.json` | Adaptable deterministic terrain subroutine; rebuild target data. | Full rule evaluation, extensions, generation stages and chunk integration remain pending. |
| 358 | `benchmarks/rust-model-baking-2026-10-06/scoped-parity.json` | Adaptable geometry tests; rewrite renderer interfaces. | No atlas packing, blockstate transform integration, shader/pixel or GPU parity. |
| 359 | `benchmarks/rust-model-resolution-2026-10-06/scoped-parity.json` | Adaptable dependency resolution; rebuild 1.20.1 model resources. | Does not cover quad baking, blockstate variants, atlas, lighting or visual parity. |
| 360 | `benchmarks/rust-movement-emission-2026-10-07/scoped-parity.json` | Adaptable event-request model; rebuild sound/tag registry. | Does not play audio or connect to a live player movement loop. |
| 361 | `benchmarks/rust-movement-registry-2026-10-07/scoped-parity.json` | Reference data schema; regenerate from 1.20.1 registries. | 26.3 coefficients/tags are not 1.20.1 data; does not prove live player movement. |
| 362 | `benchmarks/rust-movement-support-2026-10-07/scoped-parity.json` | Adaptable helper tests; rebuild world collision binding. | Not an integrated Player movement loop; target block/attribute registries differ. |
| 363 | `benchmarks/rust-native-attribute-updates-2026-10-07/input-scene.json` | Reference input fixture; regenerate for 1.20.1. | No aggregate Rust output hash; scene manifest is an input description, not an acceptance result. |
| 364 | `benchmarks/rust-native-attribute-updates-2026-10-07/interactive-smoke.json` | Reference smoke evidence only; not a performance benchmark. | Linux x86_64 Vulkan llvmpipe CPU renderer; default actor/scene smoke only, not game, GPU performance or Android evidence. |
| 365 | `benchmarks/rust-native-attribute-updates-2026-10-07/scene-motion.json` | Adaptable movement parity scope; evidence and integration gates remain. | No top-level Rust output digest despite reference data; fluids/effects/inventory/network and full Player tick are omitted. |
| 366 | `benchmarks/rust-native-attribute-updates-2026-10-07/startup-smoke.json` | Reference renderer smoke only. | CPU llvmpipe scene only; startup snapshot cannot demonstrate playable game, Android FPS or native pixel parity. |
| 367 | `benchmarks/rust-native-blockstate-client-2026-10-06/startup-smoke.json` | Adaptable visual inspection checklist; rebuild 1.20.1 assets. | Linux llvmpipe software renderer; no native pixel comparison or hardware FPS. |
| 368 | `benchmarks/rust-native-chunk-models-2026-10-06/startup-smoke.json` | Reference rendering smoke only; no hardware performance claim. | Linux llvmpipe and private screenshots; not Android GPU/frame pacing or whole-world loading evidence. |
| 369 | `benchmarks/rust-native-crouch-2026-10-07/input-scene.json` | Reference input fixture; regenerate against 1.20.1. | Controlled scene and query gates; full client/server tick, effects, vehicles and network are omitted. |
| 370 | `benchmarks/rust-native-crouch-2026-10-07/interactive-smoke.json` | Reference smoke evidence only. | Linux llvmpipe; only 8 cm recorded walk, no fluid/other poses/full client lifecycle or Android device result. |
| 371 | `benchmarks/rust-native-crouch-2026-10-07/scene-motion.json` | Adaptable input/movement tests; integrate 1.20.1 entity state. | Does not prove full LocalPlayer tick, GUI settings persistence, network or full gameplay. |
| 372 | `benchmarks/rust-native-crouch-2026-10-07/sprint-input.json` | Adaptable key-state tests; regenerate 1.20.1 input rules. | No full parent movement, complete client tick, GUI persistence, networking or gameplay parity. |
| 373 | `benchmarks/rust-native-crouch-2026-10-07/startup-smoke.json` | Reference renderer smoke only. | 30-frame CPU llvmpipe startup; no active movement, game lifecycle or hardware FPS claim. |
| 374 | `benchmarks/rust-native-health-2026-10-07/interactive-smoke.json` | Reference gameplay smoke only; not acceptance. | Linux llvmpipe resident scene; death/drop/respawn, full hunger and device performance are not tested. |
| 375 | `benchmarks/rust-native-health-2026-10-07/scene-motion.json` | Adaptable integrated method test; rebuild 1.20.1 damage/food flow. | Full Player.tick, inventory/equipment, freezing, packets and death/drop/respawn services remain outside scope. |

## Part 17 — Entries 376–400

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 376 | `benchmarks/rust-native-health-2026-10-07/startup-smoke.json` | Reference renderer smoke only. | No player lifecycle or movement; Linux llvmpipe CPU renderer only. |
| 377 | `benchmarks/rust-native-hunger-2026-10-07/interactive-smoke.json` | Reference smoke evidence only. | Elapsed wall time is not simulation time; complete enclosing ticks, item use, death lifecycle and GPU/device evidence are absent. |
| 378 | `benchmarks/rust-native-hunger-2026-10-07/scene-motion.json` | Adaptable player-state tests; rewrite tick orchestration. | No complete outer Player.tick, item-use path, full death/drop/respawn or device integration. |
| 379 | `benchmarks/rust-native-landing-2026-10-07/interactive-smoke.json` | Reference smoke evidence only. | Damage request is not applied to health; Linux CPU scene only, no full gameplay or device acceptance. |
| 380 | `benchmarks/rust-native-landing-2026-10-07/scene-motion.json` | Adaptable end-to-end fixture design; rewrite world services. | Controlled scene excludes many world callbacks, packet acceptance, full player tick and complete death flow. |
| 381 | `benchmarks/rust-native-model-client-2026-10-06/startup-smoke.json` | Reference visual QA only; not device performance evidence. | Pre-final interactive view differs from final fingerprints; CPU llvmpipe only, no gameplay or complete pixel parity. |
| 382 | `benchmarks/rust-native-motion-tick-2026-10-07/scoped-parity.json` | Adaptable method-level parity tests; add missing tick lifecycle. | No full Player.aiStep/tick, local input, freezing, health/damage or complete gameplay claim. |
| 383 | `benchmarks/rust-native-movement-statistics-2026-10-07/interactive-smoke.json` | Reference smoke evidence only. | Linux llvmpipe; sprint/full gameplay and Android hardware FPS are not established. |
| 384 | `benchmarks/rust-native-movement-statistics-2026-10-07/scene-motion.json` | Adaptable composed movement/statistics tests; rebuild 1.20.1 server loop. | Still excludes complete client/server packet acceptance, inventory/effects and full Player.tick. |
| 385 | `benchmarks/rust-native-scene-collision-2026-10-07/interactive-smoke.json` | Reference renderer smoke only. | No gameplay simulation; Vulkan llvmpipe CPU adapter, not target GPU or Android FPS. |
| 386 | `benchmarks/rust-native-scene-collision-2026-10-07/scoped-parity.json` | Adaptable collision test fixtures; evidence gap for output digest. | No aggregate Rust output hash; no gravity, tick schedule, fluid/entity gameplay or pixel renderer parity. |
| 387 | `benchmarks/rust-native-scene-collision-2026-10-07/startup-smoke.json` | Reference renderer smoke only. | 30-frame CPU llvmpipe snapshot; not a performance benchmark or playable game proof. |
| 388 | `benchmarks/rust-native-scene-motion-2026-10-07/collision-regression.json` | Adaptable boundary regression; integrate in target world. | Collision adapter only; does not establish live player travel, gravity, tick, fluids or full gameplay. |
| 389 | `benchmarks/rust-native-scene-motion-2026-10-07/scoped-parity.json` | Adaptable method-level motion tests; add excluded integrations separately. | Fluids, vehicles, effects, crouch, landing/damage, audio and network consumers remain controlled; no full player tick. |
| 390 | `benchmarks/rust-native-scene-motion-2026-10-07/startup-smoke.json` | Reference renderer smoke only. | Linux llvmpipe CPU renderer only; no visual pixel parity or Android device measurement. |
| 391 | `benchmarks/rust-native-sprint-2026-10-07/double-tap-smoke.json` | Reference smoke evidence only. | Linux llvmpipe; focus pauses walking, and full pose/fluid/effect/item-use/network lifecycle is outside scope. |
| 392 | `benchmarks/rust-native-sprint-2026-10-07/scene-motion.json` | Adaptable composed sprint tests; rebuild player state for 1.20.1. | No complete outer client/server tick, GUI persistence, effects, inventory, networking or full gameplay. |
| 393 | `benchmarks/rust-native-sprint-2026-10-07/sprint-input.json` | Adaptable input state machine tests; regenerate 1.20.1 controls. | Setter records requests; actual parent movement, GUI options and network lifecycle are separate. |
| 394 | `benchmarks/rust-native-sprint-2026-10-07/sprint-key-smoke.json` | Reference smoke evidence only. | Linux llvmpipe; no fluid/effect/item-use/client-server lifecycle, native pixel or A22 performance claim. |
| 395 | `benchmarks/rust-native-sprint-2026-10-07/sprint-scene.json` | Adaptable sprint integration tests; rewrite target controls. | Raw shift is recorded but crouch is unbound; effects/equipment/vehicles/fluids, complete ticks and networking remain excluded. |
| 396 | `benchmarks/rust-native-walking-2026-10-07/interactive-smoke.json` | Reference smoke evidence only. | Linux llvmpipe development scene; no full gameplay, native pixel parity or device acceptance. |
| 397 | `benchmarks/rust-native-walking-2026-10-07/local-input.json` | Adaptable input math tests; rebuild 1.20.1 controls. | Does not establish full LocalPlayer tick, input remapping, sprint eligibility, pose/camera or network lifecycle. |
| 398 | `benchmarks/rust-native-walking-2026-10-07/scene-motion.json` | Adaptable motion tests; integrate missing 1.20.1 player lifecycle. | Full Player.tick, inventory/equipment ticks, freezing, health/damage and network consumers are not covered. |
| 399 | `benchmarks/rust-native-walking-2026-10-07/startup-smoke.json` | Reference renderer smoke only. | Linux llvmpipe CPU renderer only; no walking/game loop or hardware FPS evidence. |
| 400 | `benchmarks/rust-neighbor-spread-2026-10-06/scoped-parity.json` | Adaptable worldgen algorithm tests; rebuild 1.20.1 block-state/feature data. | Actual chunks, neighbor callbacks and complete feature/world generation are pending. |

## Part 18 — Entries 401–425

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 401 | `benchmarks/rust-noise-2026-10-05/scoped-parity.json` | Adaptable; rebuild version data | Selected maths only; no complete terrain/world proof |
| 402 | `benchmarks/rust-object-open-map-2026-10-07/scoped-parity.json` | Reference; prefer Rust ownership | Collection parity is not game integration |
| 403 | `benchmarks/rust-object-open-set-2026-10-07/scoped-parity.json` | Reference; use Rust set if equivalent | Not proof of integrated attribute queues |
| 404 | `benchmarks/rust-pillar-2026-10-06/scoped-parity.json` | Adaptable feature algorithm | No complete chunk mutation/biome generation |
| 405 | `benchmarks/rust-piston-movement-2026-10-07/movement-registry-regression.json` | Reference; regenerate target data | 26.3 table; no live piston event |
| 406 | `benchmarks/rust-piston-movement-2026-10-07/scoped-parity.json` | Adaptable numeric regression | No piston activation/block lifecycle/network |
| 407 | `benchmarks/rust-piston-shape-2026-10-06/scoped-parity.json` | Adaptable shape tests | No live block entity/piston lifecycle |
| 408 | `benchmarks/rust-placement-2026-10-06/scoped-parity.json` | Adaptable codecs | No full configured-feature composition/world mutation |
| 409 | `benchmarks/rust-player-equipment-2026-10-07/scoped-parity.json` | Adaptable ownership model | No full synchronized/use/save lifecycle |
| 410 | `benchmarks/rust-player-food-2026-10-07/scoped-parity.json` | Adaptable state tests | No full player tick/regeneration/damage/persistence |
| 411 | `benchmarks/rust-player-health-2026-10-07/scoped-parity.json` | Adaptable state composition | No full damage source/wire/player lifecycle |
| 412 | `benchmarks/rust-player-hurt-2026-10-07/scoped-parity.json` | Adaptable damage tests | Downstream damage/effect services controlled |
| 413 | `benchmarks/rust-player-inventory-2026-10-07/scoped-parity.json` | Adaptable inventory algorithms | No menus/recipes/pickup or all components |
| 414 | `benchmarks/rust-player-travel-2026-10-07/scoped-parity.json` | Adaptable movement sequencing | No live fluid/collision/vehicle/effect full tick |
| 415 | `benchmarks/rust-powder-snow-collision-2026-10-06/scoped-parity.json` | Adaptable collision tests | No freezing/sinking/durability/live movement |
| 416 | `benchmarks/rust-registered-entity-fluid-2026-10-07/immersion-regression.json` | Adaptable query/order regression | No live fluid propagation or broad entity index |
| 417 | `benchmarks/rust-registered-entity-fluid-2026-10-07/scoped-parity.json` | Adaptable fluid tests | Providers fixture-controlled; no complete liquid movement |
| 418 | `benchmarks/rust-registered-entity-fluid-2026-10-07/tick-regression.json` | Adaptable tick ordering | No particle/audio playback or live propagation |
| 419 | `benchmarks/rust-registered-player-combat-2026-10-07/scoped-parity.json` | Adaptable combat harness | PvP/blocking/durability/effect consumers controlled |
| 420 | `benchmarks/rust-registry-fall-ray-2026-10-07/scoped-parity.json` | Adaptable ray tests | No live chunk provider or complete consumer |
| 421 | `benchmarks/rust-registry-fluid-2026-10-07/scoped-parity.json` | Reference query design | 26.3 ids/data; no live spreading |
| 422 | `benchmarks/rust-replace-blobs-2026-10-06/scoped-parity.json` | Adaptable feature tests | No complete chunk mutation/biome generation |
| 423 | `benchmarks/rust-replace-block-2026-10-06/scoped-parity.json` | Adaptable placement logic | No region effects/structure processors/decorations |
| 424 | `benchmarks/rust-resident-collision-scan-2026-10-06/scoped-parity.json` | Adaptable collision harness | Controlled proxy; no live chunk/entity broad-phase |
| 425 | `benchmarks/rust-resource-number-scanner-2026-10-06/model-baking.json` | Adaptable geometry oracle | CPU geometry only; no atlas/shader/GPU pixels |

## Part 19 — Entries 426–450

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 426 | `benchmarks/rust-resource-number-scanner-2026-10-06/model-decoding.json` | Adaptable parser fixture | No parent resolution/atlas/GPU rendering |
| 427 | `benchmarks/rust-resource-number-scanner-2026-10-06/model-resolution.json` | Adaptable resolver fixture | No final quad bake/light/render pixels |
| 428 | `benchmarks/rust-resource-number-scanner-2026-10-06/sprite-animation.json` | Adaptable state tests | No image decode/atlas upload/GPU |
| 429 | `benchmarks/rust-restitution-2026-10-07/scoped-parity.json` | Adaptable numeric tests | No live grounding/tags/effects/world lifecycle |
| 430 | `benchmarks/rust-retained-upgrade-status-2026-10-06/scoped-parity.json` | Adaptable metadata model | No save-dirty scheduling/biome migration/full retrogen |
| 431 | `benchmarks/rust-retrogen-biome-fill-2026-10-06/scoped-parity.json` | Adaptable section updates | No target palette/full biome generation |
| 432 | `benchmarks/rust-retrogen-biome-generation-2026-10-06/scoped-parity.json` | Adaptable generator pattern | Not full generation; holder placeholders limit fidelity |
| 433 | `benchmarks/rust-retrogen-chunk-2026-10-06/scoped-parity.json` | Adaptable chunk state | No persistence/scheduler/full lifecycle |
| 434 | `benchmarks/rust-retrogen-data-2026-10-06/scoped-parity.json` | Adaptable codec tests | No automatic schedule or end-to-end upgrade |
| 435 | `benchmarks/rust-rule-test-2026-10-06/scoped-parity.json` | Adaptable codec tests | No processor execution or structure mutation |
| 436 | `benchmarks/rust-scaffolding-collision-2026-10-06/scoped-parity.json` | Adaptable shape matrix | No placement/support/fall/live movement |
| 437 | `benchmarks/rust-scattered-ore-2026-10-06/scoped-parity.json` | Adaptable feature test | No complete veins/chunk decoration |
| 438 | `benchmarks/rust-server-player-combat-2026-10-07/scoped-parity.json` | Adaptable damage composition | Effects/durability/PvP/network/death controlled |
| 439 | `benchmarks/rust-server-player-combat-immunity-2026-10-07/inner-damage-regression.json` | Adaptable regression | Outer gates and attribution/world integration separate |
| 440 | `benchmarks/rust-server-player-combat-immunity-2026-10-07/scoped-parity.json` | Adaptable boundary tests | No complete damage/effects/packets/player tick |
| 441 | `benchmarks/rust-server-player-damage-2026-10-07/scoped-parity.json` | Adaptable damage flow | Criteria/effects/durability/death controlled |
| 442 | `benchmarks/rust-shared-item-use-2026-10-07/item-use-lifecycle.json` | Adaptable lifecycle tests | Start/network/concrete consumables/full player lifecycle pending |
| 443 | `benchmarks/rust-shared-item-use-2026-10-07/item-use-start.json` | Adaptable dispatch tests | Transport and synced flag consumers absent |
| 444 | `benchmarks/rust-shared-item-use-2026-10-07/item-use.json` | Adaptable blocking tests | Full lifecycle/durability/combat integration absent |
| 445 | `benchmarks/rust-simple-block-2026-10-06/scoped-parity.json` | Adaptable feature implementation | Chunk writes/neighbors/full generation not tested |
| 446 | `benchmarks/rust-simple-container-2026-10-07/scoped-parity.json` | Adaptable container pattern | No persistence/menu transactions/full components |
| 447 | `benchmarks/rust-snow-freeze-2026-10-06/scoped-parity.json` | Adaptable terrain feature | Lighting/broad mutation/surface generation absent |
| 448 | `benchmarks/rust-speleothem-2026-10-06/scoped-parity.json` | Adaptable feature geometry | No neighbor updates/falling/fluid/full caves |
| 449 | `benchmarks/rust-spike-2026-10-06/scoped-parity.json` | Adaptable deterministic feature | No complete End chunk/world lifecycle |
| 450 | `benchmarks/rust-spring-2026-10-06/scoped-parity.json` | Adaptable feature tests | Tick execution/spreading/full worldgen absent |

## Part 20 — Entries 451–475

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 451 | `benchmarks/rust-sprite-animation-2026-10-06/scoped-parity.json` | Adaptable animation logic | No image decode/atlas/GPU frame interpolation |
| 452 | `benchmarks/rust-sprite-animation-2026-10-06/startup-smoke.json` | Reference visual smoke only | No player gameplay/device hardware/FPS |
| 453 | `benchmarks/rust-state-provider-2026-10-06/scoped-parity.json` | Adaptable codec architecture | No registry reload/placement/worldgen integration |
| 454 | `benchmarks/rust-status-effect-2026-10-07/scoped-parity.json` | Adaptable value semantics | Effect tick/registry/UI lifecycle separate |
| 455 | `benchmarks/rust-status-tick-2026-10-07/scoped-parity.json` | Adaptable tick algorithm | Real dispatch/registry/full gameplay outside harness |
| 456 | `benchmarks/rust-stored-consume-2026-10-07/scoped-parity.json` | Adaptable regression; evidence gap | No aggregate Rust hash; registration/notification/sound controlled |
| 457 | `benchmarks/rust-stored-effect-attributes-2026-10-07/scoped-parity.json` | Reference only pending evidence | No top-level hash and reference outputs differ |
| 458 | `benchmarks/rust-stored-effect-notifications-2026-10-07/scoped-parity.json` | Reference only pending evidence | No top-level hash; reference output hashes differ |
| 459 | `benchmarks/rust-structure-density-2026-10-06/scoped-parity.json` | Adaptable numeric provider | No structure discovery/placement/terrain integration |
| 460 | `benchmarks/rust-suffocation-2026-10-06/contextual-regression.json` | Adaptable shape tests | No live entity/minecart/world lifecycle |
| 461 | `benchmarks/rust-suffocation-2026-10-06/scoped-parity.json` | Adaptable query tests | No live chunk provider/entity movement |
| 462 | `benchmarks/rust-swimming-2026-10-07/scoped-parity.json` | Adaptable predicates | No full water physics/player tick |
| 463 | `benchmarks/rust-synced-bytes-2026-10-07/float-regression.json` | Adaptable sync design | Other types/packet codecs/entity schema absent |
| 464 | `benchmarks/rust-synced-bytes-2026-10-07/scoped-parity.json` | Adaptable typed tests | Other types/schema/wire codecs pending |
| 465 | `benchmarks/rust-synced-floats-2026-10-07/scoped-parity.json` | Adaptable numeric serialization | Only float serializer; no protocol/wire schema |
| 466 | `benchmarks/rust-teleport-consume-2026-10-07/scoped-parity.json` | Adaptable random algorithm | Actual teleport/passenger/world events controlled |
| 467 | `benchmarks/rust-terrain-2026-10-06/scoped-parity.json` | Adaptable terrain architecture | Material rules/carvers/structures/full blend absent |
| 468 | `benchmarks/rust-ticks-2026-10-05/scoped-parity.json` | Adaptable scheduler tests | No block/fluid tick execution/full scheduler |
| 469 | `benchmarks/rust-underwater-magma-2026-10-06/scoped-parity.json` | Adaptable feature placement | Chunk updates/postprocessing/full generation absent |
| 470 | `benchmarks/rust-use-flag-reaction-2026-10-07/scoped-parity.json` | Adaptable event ordering | Other sync keys/packet delivery/full item use absent |
| 471 | `benchmarks/rust-use-remainder-2026-10-07/scoped-parity.json` | Adaptable component contract | No full component schema/insertion/drop/consumables |
| 472 | `benchmarks/rust-value-provider-2026-10-06/scoped-parity.json` | Adaptable provider algorithms | Gaussian/full height provider/terrain incomplete |
| 473 | `benchmarks/rust-view-direction-2026-10-07/scoped-parity.json` | Adaptable numeric fixtures | No platform numeric/action integration proof |
| 474 | `benchmarks/rust-vines-2026-10-06/scoped-parity.json` | Adaptable feature logic | No growth tick/neighbor/live chunk |
| 475 | `benchmarks/rust-water-splash-2026-10-07/scoped-parity.json` | Adaptable event request model | No audio mixer/particle renderer/live movement |

## Part 21 — Entries 476–500

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 476 | `benchmarks/rust-world-2026-10-05/scoped-parity.json` | Reference only | No output hash/reference pair; not implementation evidence |
| 477 | `benchmarks/rust-world-border-2026-10-07/scoped-parity.json` | Adaptable geometry/state tests | No persistence/listener/damage/network integration |
| 478 | `benchmarks/rust-world-collision-check-2026-10-07/movement-regression.json` | Adaptable physics tests | Providers/entity lists/border controlled; no live broad phase |
| 479 | `benchmarks/rust-world-collision-check-2026-10-07/scoped-parity.json` | Adaptable collision composition | Live entity broad phase/minecart/world lifecycle incomplete |
| 480 | `benchmarks/rust-world-edge-backoff-2026-10-07/scoped-parity.json` | Adaptable edge regression | Isolated query, not integrated Player physics/world |
| 481 | `benchmarks/rust-world-movement-2026-10-07/geometry-regression.json` | Adaptable geometry suite | No live World shape selection/integrated movement |
| 482 | `benchmarks/rust-world-movement-2026-10-07/scoped-parity.json` | Adaptable movement kernel | No live provider/full movement tick/input lifecycle |
| 483 | `benchmarks/rust-world-ray-2026-10-07/scoped-parity.json` | Adaptable ray traversal | No live world/chunk provider/complete consumers |
| 484 | `benchmarks/rust-worldgen-2026-10-06/scoped-parity.json` | Architecture reference; rebuild data | Selected codec/query manifests only; no full terrain/chunk/world proof |
| 485 | `benchmarks/rust-worldgen-random-2026-10-06/scoped-parity.json` | Adaptable deterministic RNG tests | Gaussian/concurrent/full decoration outside scope |
| 486 | `benchmarks/server-2026-10-17/README.md` | Reference method only | Bot host overloaded, unequal workloads and vanilla watchdog; not Android client |
| 487 | `benchmarks/server-2026-10-17/earlier-002f14ce/explore-equal-rate.json` | Reference server data | Server only, one pass; not A22/client FPS |
| 488 | `benchmarks/server-2026-10-17/earlier-002f14ce/load.txt` | Context only | Remote bot host high load/ThinkPad variability confound |
| 489 | `benchmarks/server-2026-10-17/earlier-002f14ce/walk-explore.json` | Reference throughput only | Vanilla watchdog/broken pipe at high load; unequal chunks |
| 490 | `benchmarks/server-2026-10-17/explore-equal-rate.json` | Reference comparison | Server tick/RSS/CPU only |
| 491 | `benchmarks/server-2026-10-17/load.txt` | Benchmark context | Mac mini bot host often saturated |
| 492 | `benchmarks/server-2026-10-17/walk-explore.json` | Reference server result | Vanilla watchdog high counts; remote bots |
| 493 | `benchmarks/server-equal-mixed-2026-10-18/README.md` | Reference only; fairness caveat | At25 new-chunk rate still differs; vanilla50 watchdog; remote bot host busy |
| 494 | `benchmarks/server-equal-mixed-2026-10-18/load.txt` | Context only | Bot host CPU 70–93%; server host spikes |
| 495 | `benchmarks/server-equal-mixed-2026-10-18/mixed-equal.json` | Reference server data | Horizon 1.77/4.74/11.72/23.82ms; vanilla 11.34/34.3/50.45 then watchdog; 25 work differs |
| 496 | `benchmarks/server-equal-work-2026-10-17/README.md` | Reference method | Paper often faster in walking; vanilla watchdog at100; server-only |
| 497 | `benchmarks/server-equal-work-2026-10-17/explore-equal-rate.json` | Reference measurements | Both 20TPS; ThinkPad Linux/OpenJDK25 only |
| 498 | `benchmarks/server-equal-work-2026-10-17/horizon.json` | Reference server benchmark | At100 walking ~114.5ms MSPT/8.7TPS; no client data |
| 499 | `benchmarks/server-equal-work-2026-10-17/run-1-before-chunk-cache.json` | Historical reference | High-load outliers/watchdog; server revision specific |
| 500 | `benchmarks/server-equal-work-2026-10-17/run-2-chunk-cache.json` | Historical reference | No matched vanilla baseline |

## Part 22 — Entries 501–525

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 501 | `benchmarks/server-equal-work-2026-10-17/vanilla.json` | Reference baseline | Watchdog/broken-pipe at high counts |
| 502 | `benchmarks/server-four-way-2026-10-19/README.md` | Reference only | Paper fastest average MSPT in several loads; remote host involved |
| 503 | `benchmarks/server-four-way-2026-10-19/load.txt` | Context only | Remote bots 80–90% busy; server host spikes |
| 504 | `benchmarks/server-four-way-2026-10-19/report-25-50.json` | Reference dedicated-server metrics | Paper25 28.3ms vs Horizon30.92; at50 Horizon47.83/Paper46.86/Fabric58.3; vanilla fails |
| 505 | `benchmarks/server-four-way-2026-10-19/report-paper-10.json` | Reference only | One workload slice only |
| 506 | `benchmarks/server-four-way-2026-10-19/runs.jsonl` | Reference dataset | Server workload data, not client FPS |
| 507 | `benchmarks/server-mixed-2026-10-18/README.md` | Reference method | Explore throughput differs; compare ticks cautiously |
| 508 | `benchmarks/server-mixed-2026-10-18/load.txt` | Context only | Mac bot host heavily loaded |
| 509 | `benchmarks/server-mixed-2026-10-18/mixed.json` | Reference server data | At50 Horizon MSPT higher than vanilla in run; throughput unequal |
| 510 | `benchmarks/server-vs-pumpkin-2026-10-31.json` | Reference server comparison | Pumpkin faster at20, mixed60, Horizon stronger100; anti-cheat/ground disabled |
| 511 | `benchmarks/shared-json-buffer-lifetime-20261017.json` | Reference only | Near-identical medians; negligible gain |
| 512 | `benchmarks/shared-json-decoder-phases-20261017.json` | Reference microbenchmark | Typed sample faster but higher peak RSS; loader-only |
| 513 | `benchmarks/shared-json-five-way-20261017.json` | Adaptable measurement design | Loader-only and variant confounds |
| 514 | `benchmarks/shared-json-five-way-final-20261017.json` | Reference optimization evidence | Lower RSS vs baseline, but host variance and loader-only scope |
| 515 | `benchmarks/shared-json-historical-20261017.json` | Historical reference | Host/build context differs; no full startup |
| 516 | `benchmarks/shared-json-startup-20261017.json` | Reference only | High host load, not full app startup |
| 517 | `benchmarks/shared-json-startup-followup-20261017.json` | Reference only | Outliers/load variance prevent robust winner |
| 518 | `benchmarks/shared-json-variant-isolation-20261017.json` | Historical experiment | No meaningful gain; experiments reverted |
| 519 | `benchmarks/tick-state-2026-10-04/live-observations.json` | Reference telemetry only | One demo-world observation, not controlled benchmark |
| 520 | `crates/cargo-horizon-mod/Cargo.toml` | Unnecessary for first milestone | Mod ecosystem not needed for vanilla client |
| 521 | `crates/cargo-horizon-mod/src/check.rs` | Defer; trust concepts only | Native source unsandboxed and not run by registry check |
| 522 | `crates/cargo-horizon-mod/src/main.rs` | Defer; not Forge | Horizon format, not Forge/1.20.1 loader |
| 523 | `crates/cargo-horizon-mod/src/registry.rs` | Unnecessary for base client | Horizon mod registry; base load error can appear empty |
| 524 | `crates/cargo-horizon-mod/template/content/block.json` | Defer | Horizon API 26.3-specific |
| 525 | `crates/cargo-horizon-mod/template/content/blockstate.json` | Defer | Horizon paths/conventions |

## Part 23 — Entries 526–550

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 526 | `crates/cargo-horizon-mod/template/content/horizon-mod.toml` | Defer | API0.2, not Forge |
| 527 | `crates/cargo-horizon-mod/template/content/item.json` | Defer | No Forge registration |
| 528 | `crates/cargo-horizon-mod/template/content/lang.json` | Adaptable localization only | Localization idea generic; ids/resource paths differ |
| 529 | `crates/cargo-horizon-mod/template/content/loot.json` | Adaptable data-pack concept | 1.20.1 codec validation required |
| 530 | `crates/cargo-horizon-mod/template/content/model.json` | Adaptable sample only | Model schema/resolution version-dependent |
| 531 | `crates/cargo-horizon-mod/template/content/pickaxe.json` | Adaptable resource idea | Tag path/version must be checked |
| 532 | `crates/cargo-horizon-mod/template/content/recipe.json` | Adaptable example | Recipe codec/IDs need target validation |
| 533 | `crates/cargo-horizon-mod/template/gitignore` | Adaptable after path rewrite | Generic hygiene but paths upstream-specific |
| 534 | `crates/cargo-horizon-mod/template/native/api.toml` | Unnecessary initially | Pinned Horizon API unrelated to Forge |
| 535 | `crates/cargo-horizon-mod/template/native/horizon-mixin.toml` | Unnecessary initially | Not Forge/1.20.1 schema |
| 536 | `crates/cargo-horizon-mod/template/native/lib.rs` | Defer | Unsandboxed Horizon hooks, not Forge |
| 537 | `crates/horizon-android/Cargo.toml` | Adaptable Android integration | Packaging pattern useful but game/package coupling needs rewrite |
| 538 | `crates/horizon-anticheat/Cargo.toml` | Exclude from client | Server-only, not client runtime |
| 539 | `crates/horizon-anticheat/src/combat.rs` | Exclude from client | Server validation only |
| 540 | `crates/horizon-anticheat/src/lib.rs` | Exclude from client | Server moderation policy, not client engine |
| 541 | `crates/horizon-anticheat/src/prediction.rs` | Exclude from client | Server authority only |
| 542 | `crates/horizon-anticheat/src/timer.rs` | Exclude from client | Server-only detection |
| 543 | `crates/horizon-assets/Cargo.toml` | Adaptable boundary; rewrite assets | Data/source APIs 26.3-specific |
| 544 | `crates/horizon-assets/examples/blockstate_binding_fixture.rs` | Adaptable parity harness | Harness only; rebuild target registry data |
| 545 | `crates/horizon-assets/examples/blockstate_model_fixture.rs` | Adaptable parser oracle | Not runtime/GPU render |
| 546 | `crates/horizon-assets/examples/cuboid_model_fixture.rs` | Adaptable fixture | No atlas/parents/GPU |
| 547 | `crates/horizon-assets/examples/item_model_fixture.rs` | Adaptable architecture; rebuild catalogs | 26.3 catalog and CPU fixture; not Android GPU proof |
| 548 | `crates/horizon-assets/examples/model_baking_fixture.rs` | Adaptable geometry oracle | CPU geometry only |
| 549 | `crates/horizon-assets/examples/model_choice_fixture.rs` | Adaptable selection harness | RNG target semantics need verification |
| 550 | `crates/horizon-assets/examples/model_resolution_fixture.rs` | Adaptable inheritance tests | No GPU/final rendering |

## Part 24 — Entries 551–575

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 551 | `crates/horizon-assets/examples/sprite_animation_fixture.rs` | Adaptable state tests | No image/atlas/GPU stage |
| 552 | `crates/horizon-assets/examples/support/mod.rs` | Adaptable test support | Test serialization utilities only |
| 553 | `crates/horizon-assets/src/animation.rs` | Adaptable CPU logic | CPU side; GPU interpolation/atlas absent |
| 554 | `crates/horizon-assets/src/bake.rs` | Highly adaptable; port with oracle | CPU geometry, no atlas/shader/pixel parity |
| 555 | `crates/horizon-assets/src/bake/transform.rs` | Adaptable math; rebuild transforms | JOML/asset behavior 26.3-specific |
| 556 | `crates/horizon-assets/src/blockstate.rs` | Adaptable parser | Target JSON/schema semantics need validation |
| 557 | `crates/horizon-assets/src/blockstate/bind.rs` | Adaptable binding | Invalid/overlap semantics need 1.20.1 oracle |
| 558 | `crates/horizon-assets/src/blockstate/choice.rs` | Adaptable algorithm | Random consumption affects appearance; re-oracle |
| 559 | `crates/horizon-assets/src/item_bake.rs` | Adaptable CPU bake | BTree ordering may differ from Java HashSet; no GPU |
| 560 | `crates/horizon-assets/src/item_model.rs` | Architecture adaptable; rewrite data | All component ids/selectors are 26.3 data |
| 561 | `crates/horizon-assets/src/item_model/component_predicate.rs` | Adaptable predicate architecture | Only selected families; rebuild component registry |
| 562 | `crates/horizon-assets/src/item_model/foreign.rs` | Reference future extension only | No actual mod hook execution; loader support limited |
| 563 | `crates/horizon-assets/src/item_model/foreign_tests.rs` | Adaptable tests only | Fallback is not mod runtime/Forge compatibility |
| 564 | `crates/horizon-assets/src/item_model/special.rs` | Adaptable schema; rebuild data | Actual renderer elsewhere; 26.3 catalog |
| 565 | `crates/horizon-assets/src/item_model/tests.rs` | Adaptable focused tests | Not renderer acceptance |
| 566 | `crates/horizon-assets/src/json.rs` | Adaptable only after oracle | 1024-char token cap and parser edges need tests |
| 567 | `crates/horizon-assets/src/lib.rs` | Adaptable boundary | Module aggregation, not client integration |
| 568 | `crates/horizon-assets/src/math.rs` | Highly adaptable math | f32/platform-sensitive; ARM64/Java oracle required |
| 569 | `crates/horizon-assets/src/model.rs` | Adaptable resource model | NeoForge OBJ/composite and 26.3 data need review |
| 570 | `crates/horizon-assets/src/obj.rs` | Defer optional extension | Optional extension/MTL not base vanilla requirement |
| 571 | `crates/horizon-assets/src/palette.rs` | Adaptable utility | CPU utility only; no texture upload |
| 572 | `crates/horizon-assets/src/resolve.rs` | Highly adaptable architecture | No final GPU parity; 26.3 resource rules |
| 573 | `crates/horizon-bench/Cargo.toml` | Adaptable measurement schema | Tooling, not gameplay |
| 574 | `crates/horizon-bench/src/hardware.rs` | Adaptable telemetry | Android needs native APIs |
| 575 | `crates/horizon-bench/src/lib.rs` | Adaptable report design | Upstream-specific schema; tests format logic only |

## Part 25 — Entries 576–600

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 576 | `crates/horizon-bots/Cargo.toml` | Exclude from client; optional QA | Not playable Android client |
| 577 | `crates/horizon-bots/src/main.rs` | Server QA reference only | Protocol26.3 and synthetic flight; not faithful human client |
| 578 | `crates/horizon-chat-filter/Cargo.toml` | Exclude from client | Not client/render dependency |
| 579 | `crates/horizon-chat-filter/data/slurs.txt` | Exclude from client | Not needed in client; false-positive policy concern |
| 580 | `crates/horizon-chat-filter/src/lib.rs` | Exclude from client | Server moderation policy, not client chat rendering |
| 581 | `crates/horizon-chat-filter/src/normalise.rs` | Reference only | Language-dependent; can false-positive |
| 582 | `crates/horizon-chat-filter/src/tests.rs` | Reference tests only | One server policy, not general language semantics |
| 583 | `crates/horizon-client-core/src/block_entity_nbt.rs` | Adaptable boundary | Useful boundary; 26.3 schema |
| 584 | `crates/horizon-client-core/src/bossbar.rs` | High-value architecture; rewrite schema | 26.3-only types/defaults; rebuild exact 1.20.1 ids |
| 585 | `crates/horizon-client-core/src/component.rs` | Adaptable state model | No packet sync/HUD integration; color mappings 26.3 |
| 586 | `crates/horizon-client-core/src/fastutil_order.rs` | Adaptable tree; reimplement codecs | Later 26.3 component forms; 1.20.1 differs |
| 587 | `crates/horizon-client-core/src/gamerules.rs` | Reference only where target oracle needs | Collection internals not universal semantics |
| 588 | `crates/horizon-client-core/src/identifier.rs` | Adaptable typed model | 26.3 catalog; regenerate 1.20.1 table |
| 589 | `crates/horizon-client-core/src/key_display.rs` | Adaptable validation | Two helpers differ in strictness; compare target JVM |
| 590 | `crates/horizon-client-core/src/lang.rs` | Adaptable presentation | SDL/US layout assumptions; Android APIs differ |
| 591 | `crates/horizon-client-core/src/nbt_path.rs` | Adaptable translation architecture | Load target 1.20.1 language resources |
| 592 | `crates/horizon-client-core/src/nbt_value.rs` | Adaptable layering | Module aggregation, not game integration |
| 593 | `crates/horizon-client-core/src/particle_type.rs` | Highly adaptable NBT engine | Need Java 1.20.1 oracle incl mutation/depth edges |
| 594 | `crates/horizon-client-core/src/pattern.rs` | Adaptable utility | Numeric conversion version-specific |
| 595 | `crates/horizon-client-core/src/potions.rs` | Reference schema; rebuild table | Later ids; regenerate 1.20.1 registry/layers |
| 596 | `crates/horizon-client-core/src/registries.rs` | Adaptable parser with conformance tests | No groups/braces; only a subset supported |
| 597 | `crates/horizon-client-core/src/scoreboard.rs` | Adaptable architecture; rewrite tables | Later effect ids/mod registration; regenerate 1.20.1 |
| 598 | `crates/horizon-client-core/src/snbt.rs` | Reference map layout; rebuild ids | 26.3 catalog cannot be used as 1.20.1 truth |
| 599 | `crates/horizon-client-core/src/sound_source.rs` | High-value state/save model; rebuild | 26.3 criteria/stats, no packet/HUD integration |
| 600 | `crates/horizon-client-core/src/text.rs` | Adaptable formatting utility | Hash order heuristic; exact serialization needs Java oracle |
