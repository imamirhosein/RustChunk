- Source ledgers: 1–2600 in prior ledgers; active `AUDIT_LEDGER_2601_2800.md` now has **125 records (2601–2725)**. Stage 2601–2800 is active (125/200); sections 2601–2650 and 2651–2700 are closed, and section 2701–2750 is in progress (25/50). The exceptional grant **2701–2800** is active; do not stop before entry 2800. Backup remains `backup/audit-2000-final-2026-11-21`.
- Ordering correction at entry 2701: a full scan of the pinned tree found **75 unindexed Rust blobs** lexically before or equal to the old last indexed path. The first 25 are now entries 2701–2725; the remaining 50 stay in the next unindexed candidate sequence. Selection is always from the complete tree after excluding the full Index; historical entry numbers are not renumbered.
- Cross-chat resume state: [AUDIT_CONTINUITY.md](AUDIT_CONTINUITY.md). Read this file before resuming work in a new chat.
- Source repository/ref: `starling-client/starling` / `starling`
- Pinned source snapshot: `5432370d573642041585c69b40fb045b27133488`
- Decisions and one-line reasons summarize each ledger entry's Decision field; consult the ledger for evidence, limitations, implementation mapping and follow-up checks.
> **Stable numbering / source order:** Index numbers preserve the audit's historical sequence; they do not imply that every row from 1–2400 is in raw GitHub recursive-tree API order. 70 adjacent transitions relative to the pinned GitHub recursive-tree API order were found among the 2,400 stable Index rows at the time of reconciliation: 69 occur in entries 1–2000, and one occurs at entry 2033. No additional raw-tree-position regressions occur from entry 2034 through 2400. This is a historical ordering divergence, not a missing-file, duplicate-path or source-provenance defect: all 2,600 Index paths are unique and present in the pinned tree. The audit IDs are stable historical chronology and must not be renumbered because Ledger, QA and cross-document references depend on them. Do not describe the whole Index as a literal raw recursive-tree traversal. Do not renumber old entries. Future source selection follows deterministic lexical path order strictly after the latest indexed path, excluding the full Index.

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
| 62 | `crates/horizon-protocol/src/hash_ops.rs` | undetermined; Reference; Version-specific | Version-specific and unsuitable for protocol 1.20.1; retain only as a reference for implementing the newer Codec-hash scheme. |
| 63 | `crates/horizon-protocol/src/mod_list.rs` | Unnecessary; Adaptable; Optional/future | The mod-list packet is not vanilla 1.20.1; file-hash parity may inform a future optional mod-compatibility layer. |
| 64 | `crates/horizon-protocol/src/mod_net.rs` | Unnecessary; Adaptable; Optional/future | Custom mod networking is not in the vanilla baseline; preserve only generic codec-validation ideas for a future extension. |
| 65 | `crates/horizon-protocol/src/quiet.rs` | Unnecessary; Adaptable; Optional/future | The custom quiet/batching packet is not baseline gameplay; bounded batching may inform a future extension. |
| 66 | `crates/horizon-protocol/src/servers.rs` | Adaptable | Adaptable after verifying the 1.20.1 schema and filesystem semantics. |
| 67 | `crates/horizon-protocol/src/text.rs` | Adaptable | Adaptable only as a plain-text fallback; the 1.20.1 rich-component parser/renderer must be implemented separately. |
| 68 | `crates/horizon-protocol/src/crypto/blocks.rs` | Adaptable | The CFB8 algorithm is adaptable; the custom ARM AES backend is usable only after security review and safe feature dispatch. |
| 69 | `crates/horizon-protocol/src/item/hash.rs` | Unnecessary; Version-specific | Version-specific and unusable for 1.20.1; unnecessary for the target protocol. The ItemStack+NBT path must be rewritten. |
| 70 | `crates/horizon-protocol/src/item/hash/exact.rs` | Unnecessary; Version-specific | Version-specific and unnecessary for the 1.20.1 core; predicate parsing for the target version must be rewritten separately. |
| 71 | `crates/horizon-protocol/src/item/hash/gameplay.rs` | undetermined; Version-specific | Version-specific and unusable for 1.20.1; the legacy implementation must be written separately. |
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
| 130 | `crates/horizon-world/src/datafix/mod.rs` | undetermined; Reference | Reference for migration architecture only; current 26.3 fix pipeline is unsuitable for base and must be independently versioned if legacy imports are required. |
| 131 | `crates/horizon-world/src/datafix/block_state_data.rs` | Unnecessary; Reference | Unnecessary for 1.20.1 base; retain only as historical migration reference if pre-1.13 import becomes a requirement. |
| 132 | `crates/horizon-world/src/datafix/chunk_flatten.rs` | Unnecessary | Unnecessary for 1.20.1 runtime; keep only if pre-1.13 world import becomes explicit. |
| 133 | `crates/horizon-world/src/datafix/world_upgrader.rs` | Unnecessary; Optional/future | Unnecessary for first 1.20.1 milestone; transactional background conversion is a future-tool pattern only. |
| 134 | `crates/horizon-world/src/datafix/tests.rs` | undetermined; Reference | Reference-only for migration-test structure; expected 26.3 output is not a 1.20.1 oracle. |
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
| 146 | `.github/actionlint.yaml` | undetermined; Reference; Optional/future | Only relevant if RustChunk adopts self-hosted CI runners; runner trust is critical. |
| 147 | `.github/CODEOWNERS` | Adaptable | Protect signing, update, resource-acquisition and release paths with enforced owner review. |
| 148 | `.github/ISSUE_TEMPLATE/bug_report.yml` | Adaptable | Strong Android bug-triage template; rewrite fields and verify log privacy/redaction. |
| 149 | `.github/ISSUE_TEMPLATE/config.yml` | Adaptable | Issue routing pattern only; replace the Horizon-specific destination. |
| 150 | `.github/ISSUE_TEMPLATE/native-build.yml` | Unnecessary; Optional/future | Not needed for vanilla 1.20.1; Starling native builds are not Forge support. |

## Part 7 — Entries 151–175

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 151 | `.github/pull_request_template.md` | Adaptable; Rewrite required | Useful PR provenance and evidence checklist; replace 26.3 references and validate with CI. |
| 152 | `.github/release-line.txt` | Undetermined | Meaning is not clear until its consumers are traced; do not copy the value yet. |
| 153 | `.github/trusted-reviewers.txt` | undetermined; Reference; Optional/future | Repository moderation allowlist only; consumer scripts and permissions need review. |
| 154 | `.github/workflows/cancel-on-close.yml` | Adaptable | CI cost-control pattern; verify pull_request_target safety and run-selection behavior. |
| 155 | `.github/workflows/ci.yml` | Adaptable; Rewrite required | Strong CI foundation, but RustChunk needs 1.20.1 parity and separate Android/device gates. |
| 156 | `.github/workflows/claim-command.yml` | Unnecessary; Optional/future | Contributor-claim automation only; not part of the client and requires parser/auth review. |
| 157 | `.github/workflows/mod-registry.yml` | Adaptable; Unnecessary; Optional/future | Isolated workspace CI pattern; mod registry is not needed in the vanilla milestone. |
| 158 | `.github/workflows/mojang-republish.yml` | Adaptable | Useful pinned-metadata change alert; rewrite and test against RustChunk's 1.20.1 acquisition lock. |
| 159 | `.github/workflows/native-hook-stamp.yml` | Unnecessary; Optional/future | Generated native-hook ID service is irrelevant until an extension ABI is approved. |
| 160 | `.github/workflows/native-mods.yml` | Unnecessary; Reference; Optional/future | High-risk Starling native-code pipeline; not Forge and not part of the vanilla client. |
| 161 | `.github/workflows/overlap-note.yml` | Adaptable; Reference | Trusted main-branch, diff-only PR overlap automation; privileged event/runner isolation needs security tests. |
| 162 | `.github/workflows/pack-bench.yml` | Adaptable; Optional/future | Resource-pack branch/base benchmark pattern; replace 26.3 fixtures and add Android measurements. |
| 163 | `.github/workflows/parity-main.yml` | Adaptable; Rewrite required | Useful parity gate/bisect pattern; requires complete 1.20.1 oracle and protected cache/runner gates. |
| 164 | `.github/workflows/pr-labels.yml` | Adaptable; Reference | Metadata-only PR labeling; test untrusted event data and never execute PR-head code. |
| 165 | `AGENTS.md` | Adaptable; Rewrite required | Strong navigation/governance template; rewrite Starling paths, versions, tools and policy assumptions. |
| 166 | `bench/bench.py` | Adaptable; Optional/future | Server-load benchmark only; later needs protocol-763 bots/server and process-cleanup hardening. |
| 167 | `benchmarks/entity-model-document-2026-10-14/generate_fixture.py` | Adaptable; Reference; Optional/future | Deterministic synthetic model fixture; regenerate using 1.20.1 resource schema. |
| 168 | `benchmarks/entity-model-document-2026-10-14/README.md` | Adaptable; Reference | Output-equality and warm/alternating method; 42.94× result is only a synthetic parser segment. |
| 169 | `benchmarks/entity-model-document-2026-10-14/results.json` | undetermined; Reference; Adaptable | Raw timings agree with README but lack independent fixture/source/hardware provenance. |
| 170 | `benchmarks/graphics/after-4080.json` | Adaptable; Reference; Version-specific | RTX4080/Vulkan post-change report; RD16 drawn_sections=0 requires investigation. |
| 171 | `benchmarks/graphics/after-lavapipe.json` | Adaptable; Reference; Version-specific | Software Vulkan stress data only; RD16 drawn_sections=0 and RD32 lows need context. |
| 172 | `benchmarks/graphics/baseline-4080.json` | undetermined; Reference; Version-specific | Historical RTX4080 baseline; zero/near-zero draw counters limit interpretation. |
| 173 | `benchmarks/graphics/baseline-lavapipe.json` | undetermined; Reference; Version-specific | Historical llvmpipe baseline; not a physical GPU or phone proxy. |
| 174 | `benchmarks/model-state-parse-2026-10-14.md` | Adaptable; Reference | Shared parse reuse gives a scoped synthetic 1.306× result; validate 1.20.1 and A22. |
| 175 | `benchmarks/overlay-real-packs-2026-10-15.json` | Adaptable; Reference | Cache-hit/cold overlay methodology; pack schema is 26.3 and licenses remain separate. |

## Part 8 — Entries 176–200

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 176 | `benchmarks/perf/horizon-m4pro-1080p-rd16.json` | undetermined; Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 177 | `benchmarks/perf/horizon-m4pro-1080p-rd32.json` | undetermined; Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 178 | `benchmarks/perf/horizon-m4pro-1080p-rd8.json` | undetermined; Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 179 | `benchmarks/perf/horizon-win-9800x3d-5080-rd16.json` | undetermined; Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 180 | `benchmarks/perf/horizon-win-9800x3d-5080-rd32.json` | undetermined; Reference; Adaptable | Use the report as a desktop methodology reference only; adapt the schema/scenario method and never transfer these device FPS values to Android. |
| 181 | `benchmarks/perf/horizon-win-9800x3d-5080-rd8.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 182 | `benchmarks/perf/macmini-m4pro-loaded-2026-11-01.md` | undetermined; Reference; Version-specific | The report explicitly says it is not a fair player comparison: system contention differs; vanilla draws 14% fewer… |
| 183 | `benchmarks/perf/vanilla-m4pro-1080p-rd16.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 184 | `benchmarks/perf/vanilla-m4pro-1080p-rd32.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 185 | `benchmarks/perf/vanilla-m4pro-1080p-rd8.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 186 | `benchmarks/perf/vanilla-mods-m4pro-1080p-rd16.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 187 | `benchmarks/perf/vanilla-mods-m4pro-1080p-rd32.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 188 | `benchmarks/perf/vanilla-mods-m4pro-1080p-rd8.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 189 | `benchmarks/perf/vanilla-mods-win-9800x3d-5080-rd16.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 190 | `benchmarks/perf/vanilla-mods-win-9800x3d-5080-rd32.json` | undetermined; Reference; Version-specific | Each file has three raw flights per scene. Across the full 18-report set, FPS summaries agree with means of raw… |
| 191 | `benchmarks/perf/vanilla-mods-win-9800x3d-5080-rd8.json` | undetermined; Reference; Version-specific | Each report contains three raw flights per scene. Across all 18 files in this batch, FPS summary values match the… |
| 192 | `benchmarks/perf/vanilla-win-9800x3d-5080-rd16.json` | undetermined; Reference; Version-specific | Each report contains three raw flights per scene. Across all 18 files in this batch, FPS summary values match the… |
| 193 | `benchmarks/perf/vanilla-win-9800x3d-5080-rd32.json` | undetermined; Reference; Version-specific | Each report contains three raw flights per scene. Across all 18 files in this batch, FPS summary values match the… |
| 194 | `benchmarks/perf/vanilla-win-9800x3d-5080-rd8.json` | undetermined; Reference; Version-specific | Each report contains three raw flights per scene. Across all 18 files in this batch, FPS summary values match the… |
| 195 | `benchmarks/perf/windows-9800x3d-5080-2026-10-29.md` | Adaptable; Rewrite required; Version-specific | Use the benchmark controls and reporting method; rewrite workloads and environment for Minecraft 1.20.1 and Android/Mali. |
| 196 | `benchmarks/rust-active-effect-tick-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |
| 197 | `benchmarks/rust-active-effects-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |
| 198 | `benchmarks/rust-anchor-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |
| 199 | `benchmarks/rust-attribute-health-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |
| 200 | `benchmarks/rust-attribute-map-queries-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Reuse the provenance-manifest format; rebuild game behavior and fixtures for 1.20.1. |

## Part 9 — Entries 201–225

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

## Part 10 — Entries 226–250

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

## Part 11 — Entries 251–275

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

## Part 12 — Entries 276–300

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

## Part 13 — Entries 301–325

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

## Part 14 — Entries 326–350

| No. | Source path | Decision | One-line reason ||---:|---|---|---|
| 326 | `benchmarks/rust-item-cooldowns-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Network synchronization and complete player/item lifecycle are absent. |
| 327 | `benchmarks/rust-item-durability-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Full component codecs, enchantments and complete gameplay are not covered. |
| 328 | `benchmarks/rust-item-particles-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Particle queues/rendering, sound/effects integration and real gameplay are outside scope. |
| 329 | `benchmarks/rust-item-stack-storage-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Component codecs, menu transactions, equipment synchronization and full use flow are not included. |
| 330 | `benchmarks/rust-item-template-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Does not exercise every component codec or complete containers and use lifecycle. |
| 331 | `benchmarks/rust-item-use-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Start-use path, synchronized flags, concrete consumables and full lifecycle remain pending. |
| 332 | `benchmarks/rust-item-use-catalog-2026-10-07/scoped-parity.json` | undetermined; Reference; Version-specific | Import is 26.3-specific; other component codecs, inventory, enchantments and use effects remain unverified. |
| 333 | `benchmarks/rust-item-use-lifecycle-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Starting use, actual synchronized flags, concrete consumables, remainders and complete tick are incomplete. |
| 334 | `benchmarks/rust-item-use-start-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Transport, wire encoding, persistent data and server/client lifecycle are not exercised. |
| 335 | `benchmarks/rust-java-hash-map-2026-10-07/scoped-parity.json` | undetermined; Reference; Version-specific | JDK collection parity is not game logic; JDK version/platform semantics must not be assumed. |
| 336 | `benchmarks/rust-java-identity-map-2026-10-07/cooldown-regression.json` | undetermined; Reference; Version-specific | This is a Java/JDK regression context, not a complete in-game cooldown lifecycle. |
| 337 | `benchmarks/rust-java-identity-map-2026-10-07/ordered-map-regression.json` | undetermined; Reference; Version-specific | It is a JDK implementation comparison, not Minecraft feature parity. |
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

## Part 15 — Entries 351–375

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
| 361 | `benchmarks/rust-movement-registry-2026-10-07/scoped-parity.json` | undetermined; Reference; Version-specific | 26.3 coefficients/tags are not 1.20.1 data; does not prove live player movement. |
| 362 | `benchmarks/rust-movement-support-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Not an integrated Player movement loop; target block/attribute registries differ. |
| 363 | `benchmarks/rust-native-attribute-updates-2026-10-07/input-scene.json` | undetermined; Reference; Version-specific | No aggregate Rust output hash; scene manifest is an input description, not an acceptance result. |
| 364 | `benchmarks/rust-native-attribute-updates-2026-10-07/interactive-smoke.json` | undetermined; Reference; Version-specific | Linux x86_64 Vulkan llvmpipe CPU renderer; default actor/scene smoke only, not game, GPU performance or Android evidence. |
| 365 | `benchmarks/rust-native-attribute-updates-2026-10-07/scene-motion.json` | Adaptable; Version-specific | No top-level Rust output digest despite reference data; fluids/effects/inventory/network and full Player tick are omitted. |
| 366 | `benchmarks/rust-native-attribute-updates-2026-10-07/startup-smoke.json` | undetermined; Reference; Version-specific | CPU llvmpipe scene only; startup snapshot cannot demonstrate playable game, Android FPS or native pixel parity. |
| 367 | `benchmarks/rust-native-blockstate-client-2026-10-06/startup-smoke.json` | Adaptable; Version-specific | Linux llvmpipe software renderer; no native pixel comparison or hardware FPS. |
| 368 | `benchmarks/rust-native-chunk-models-2026-10-06/startup-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe and private screenshots; not Android GPU/frame pacing or whole-world loading evidence. |
| 369 | `benchmarks/rust-native-crouch-2026-10-07/input-scene.json` | undetermined; Reference; Version-specific | Controlled scene and query gates; full client/server tick, effects, vehicles and network are omitted. |
| 370 | `benchmarks/rust-native-crouch-2026-10-07/interactive-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe; only 8 cm recorded walk, no fluid/other poses/full client lifecycle or Android device result. |
| 371 | `benchmarks/rust-native-crouch-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Does not prove full LocalPlayer tick, GUI settings persistence, network or full gameplay. |
| 372 | `benchmarks/rust-native-crouch-2026-10-07/sprint-input.json` | Adaptable; Version-specific | No full parent movement, complete client tick, GUI persistence, networking or gameplay parity. |
| 373 | `benchmarks/rust-native-crouch-2026-10-07/startup-smoke.json` | undetermined; Reference; Version-specific | 30-frame CPU llvmpipe startup; no active movement, game lifecycle or hardware FPS claim. |
| 374 | `benchmarks/rust-native-health-2026-10-07/interactive-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe resident scene; death/drop/respawn, full hunger and device performance are not tested. |
| 375 | `benchmarks/rust-native-health-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Full Player.tick, inventory/equipment, freezing, packets and death/drop/respawn services remain outside scope. |

## Part 16 — Entries 376–400

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 376 | `benchmarks/rust-native-health-2026-10-07/startup-smoke.json` | undetermined; Reference; Version-specific | No player lifecycle or movement; Linux llvmpipe CPU renderer only. |
| 377 | `benchmarks/rust-native-hunger-2026-10-07/interactive-smoke.json` | undetermined; Reference; Version-specific | Elapsed wall time is not simulation time; complete enclosing ticks, item use, death lifecycle and GPU/device evidence are absent. |
| 378 | `benchmarks/rust-native-hunger-2026-10-07/scene-motion.json` | Adaptable; Version-specific | No complete outer Player.tick, item-use path, full death/drop/respawn or device integration. |
| 379 | `benchmarks/rust-native-landing-2026-10-07/interactive-smoke.json` | undetermined; Reference; Version-specific | Damage request is not applied to health; Linux CPU scene only, no full gameplay or device acceptance. |
| 380 | `benchmarks/rust-native-landing-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Controlled scene excludes many world callbacks, packet acceptance, full player tick and complete death flow. |
| 381 | `benchmarks/rust-native-model-client-2026-10-06/startup-smoke.json` | undetermined; Reference; Version-specific | Pre-final interactive view differs from final fingerprints; CPU llvmpipe only, no gameplay or complete pixel parity. |
| 382 | `benchmarks/rust-native-motion-tick-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No full Player.aiStep/tick, local input, freezing, health/damage or complete gameplay claim. |
| 383 | `benchmarks/rust-native-movement-statistics-2026-10-07/interactive-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe; sprint/full gameplay and Android hardware FPS are not established. |
| 384 | `benchmarks/rust-native-movement-statistics-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Still excludes complete client/server packet acceptance, inventory/effects and full Player.tick. |
| 385 | `benchmarks/rust-native-scene-collision-2026-10-07/interactive-smoke.json` | undetermined; Reference; Version-specific | No gameplay simulation; Vulkan llvmpipe CPU adapter, not target GPU or Android FPS. |
| 386 | `benchmarks/rust-native-scene-collision-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | No aggregate Rust output hash; no gravity, tick schedule, fluid/entity gameplay or pixel renderer parity. |
| 387 | `benchmarks/rust-native-scene-collision-2026-10-07/startup-smoke.json` | undetermined; Reference; Version-specific | 30-frame CPU llvmpipe snapshot; not a performance benchmark or playable game proof. |
| 388 | `benchmarks/rust-native-scene-motion-2026-10-07/collision-regression.json` | Adaptable; Version-specific | Collision adapter only; does not establish live player travel, gravity, tick, fluids or full gameplay. |
| 389 | `benchmarks/rust-native-scene-motion-2026-10-07/scoped-parity.json` | Adaptable; Version-specific | Fluids, vehicles, effects, crouch, landing/damage, audio and network consumers remain controlled; no full player tick. |
| 390 | `benchmarks/rust-native-scene-motion-2026-10-07/startup-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe CPU renderer only; no visual pixel parity or Android device measurement. |
| 391 | `benchmarks/rust-native-sprint-2026-10-07/double-tap-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe; focus pauses walking, and full pose/fluid/effect/item-use/network lifecycle is outside scope. |
| 392 | `benchmarks/rust-native-sprint-2026-10-07/scene-motion.json` | Adaptable; Version-specific | No complete outer client/server tick, GUI persistence, effects, inventory, networking or full gameplay. |
| 393 | `benchmarks/rust-native-sprint-2026-10-07/sprint-input.json` | Adaptable; Version-specific | Setter records requests; actual parent movement, GUI options and network lifecycle are separate. |
| 394 | `benchmarks/rust-native-sprint-2026-10-07/sprint-key-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe; no fluid/effect/item-use/client-server lifecycle, native pixel or A22 performance claim. |
| 395 | `benchmarks/rust-native-sprint-2026-10-07/sprint-scene.json` | Adaptable; Version-specific | Raw shift is recorded but crouch is unbound; effects/equipment/vehicles/fluids, complete ticks and networking remain excluded. |
| 396 | `benchmarks/rust-native-walking-2026-10-07/interactive-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe development scene; no full gameplay, native pixel parity or device acceptance. |
| 397 | `benchmarks/rust-native-walking-2026-10-07/local-input.json` | Adaptable; Version-specific | Does not establish full LocalPlayer tick, input remapping, sprint eligibility, pose/camera or network lifecycle. |
| 398 | `benchmarks/rust-native-walking-2026-10-07/scene-motion.json` | Adaptable; Version-specific | Full Player.tick, inventory/equipment ticks, freezing, health/damage and network consumers are not covered. |
| 399 | `benchmarks/rust-native-walking-2026-10-07/startup-smoke.json` | undetermined; Reference; Version-specific | Linux llvmpipe CPU renderer only; no walking/game loop or hardware FPS evidence. |
| 400 | `benchmarks/rust-neighbor-spread-2026-10-06/scoped-parity.json` | Adaptable; Version-specific | Actual chunks, neighbor callbacks and complete feature/world generation are pending. |

## Part 17 — Entries 401–425

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

## Part 18 — Entries 426–450

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

## Part 19 — Entries 451–475

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

## Part 20 — Entries 476–500

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

## Part 21 — Entries 501–525

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

## Part 22 — Entries 526–550

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

## Part 23 — Entries 551–575

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

## Part 24 — Entries 576–600

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

## Part 25 — Entries 601–625

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
| 621 | `crates/horizon-client-render/src/block_entity_render_tests.rs` | adaptable; reference; version-specific | Exercises vertex parity, cache invalidation, resource reloads and fallback paths for special block-entity models. |
| 622 | `crates/horizon-client-render/src/bluemap.rs` | unnecessary; optional/future | Fetches and displays remote world-map tiles and metadata alongside the in-game world. |
| 623 | `crates/horizon-client-render/src/bluemap_detect.rs` | unnecessary; optional/future | Detects a map endpoint from server-provided or discovered information before map tile loading. |
| 624 | `crates/horizon-client-render/src/border.wgsl` | adaptable; version-specific | Computes the border overlay fragment colour/fade in the GPU shader. |
| 625 | `crates/horizon-client-render/src/border_render.rs` | adaptable; version-specific | Builds border wall/plane geometry and animation parameters for the world-border pass. |

## Part 26 — Entries 626–650

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 27 — Entries 651–675

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 28 — Entries 676–700

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 29 — Entries 701–725

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 30 — Entries 726–750

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 31 — Entries 751–775

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 32 — Entries 776–800

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 33 — Entries 801–825

| No. | Source path | Decision | One-line reason |
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

## Part 34 — Entries 826–850

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 35 — Entries 851–875

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 36 — Entries 876–900

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 37 — Entries 901–925

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 38 — Entries 926–950

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 39 — Entries 951–975

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 40 — Entries 976–1000

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
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

## Part 41 — Entries 1001–1025

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
| 1021 | `crates/horizon-client/examples/probe/team_rules_fixture.rs` | adaptable; reference; version-specific | Deterministic scoreboard-team rule probe; omits packet synchronization, lifecycle, collision resolution and rendering; rebuild 1.20.1 rules. |
| 1022 | `crates/horizon-client/examples/probe/text_display_fixture.rs` | adaptable; reference; version-specific | Useful metadata-to-model test split; current packet ID and serializer schema are not 1.20.1. |
| 1023 | `crates/horizon-client/examples/probe/text_display_render_fixture.rs` | adaptable; reference; version-specific | Headless text-display quad/layout oracle; rebuild metadata, fonts and atlas for 1.20.1; no GPU output. |
| 1024 | `crates/horizon-client/examples/probe/toggle_keys_probe.rs` | adaptable; reference; version-specific | Reusable toggle-key state test pattern; key slots are source-specific and touch/multi-pointer cancellation is not covered. |
| 1025 | `crates/horizon-client/examples/probe/tooltip_display_fixture.rs` | rewrite required; reference; version-specific | Modern tooltip_display item component is not the 1.20.1 legacy ItemStack/NBT schema. |

## Part 42 — Entries 1026–1050

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1026 | `crates/horizon-client/examples/probe/trading_fixture.rs` | adaptable; reference; version-specific | Comprehensive merchant parity harness; rebuild trades, professions, RNG and NBT from 1.20.1. |
| 1027 | `crates/horizon-client/examples/probe/translucency_fixture.rs` | adaptable; reference; version-specific | CPU quad-sort/cache oracle only; GPU blending and Android visual parity remain separate. |
| 1028 | `crates/horizon-client/examples/probe/trident_probe.rs` | adaptable; reference; version-specific | Narrow projectile block-hit/movement probe; rebuild physics and block shapes for 1.20.1. |
| 1029 | `crates/horizon-client/examples/probe/vehicle_container_fixture.rs` | adaptable; reference; version-specific | Vehicle-to-menu mapping is useful; entity types, menu IDs and slot counts require exact 1.20.1 allowlist. |
| 1030 | `crates/horizon-client/examples/probe/weather_fixture.rs` | adaptable; reference; version-specific | Deterministic rain/snow column and cache geometry oracle; no GPU/weather-transition behavior tested. |
| 1031 | `crates/horizon-client/examples/probe/world_border_render_fixture.rs` | adaptable; reference; version-specific | World-border state/render preparation is useful; rebuild 1.20.1 interpolation and target viewport. |
| 1032 | `crates/horizon-client/examples/probe/world_options_fixture.rs` | rewrite required; reference; version-specific | Protocol 777 packet IDs/layouts cannot be reused as Minecraft 1.20.1 packet vectors. |
| 1033 | `crates/horizon-client/examples/random_tick_eligibility_bench.rs` | adaptable; reference; optional/future | Benchmark methodology for cache selection; requires 1.20.1 facts and real Android measurements before performance claims. |
| 1034 | `crates/horizon-client/examples/resource_memory.rs` | adaptable; reference; optional/future | Resource-loader heap/RSS profile; Linux/macOS metrics do not substitute for Android PSS/native/GPU memory. |
| 1035 | `crates/horizon-client/examples/save_batch_bench.rs` | adaptable; reference; version-specific | Exact NBT equality before timing is valuable; rebuild save schema and isolate temporary files for 1.20.1. |
| 1036 | `crates/horizon-client/examples/shared_json_startup_profile.rs` | adaptable; reference; optional/future | Compares loader strategies; requires semantic equality and Android-specific memory telemetry. |
| 1037 | `crates/horizon-client/examples/sprite_cache_parity.rs` | adaptable; reference; optional/future | Cold/warm cache parity pattern; deletes cache in supplied ROOT, so only run on a disposable copy. |
| 1038 | `crates/horizon-client/src/accounts.rs` | adaptable; reference; version-specific | Security-sensitive account/session coordinator; separate offline profiles from online auth and review providers, pins and Android secret storage independently. |
| 1039 | `crates/horizon-client/src/advancements.rs` | adaptable; reference; version-specific | Advancement tree/progress/index/layout concepts are useful; 26.x predicate/data-component schemas need a 1.20.1 legacy rewrite. |
| 1040 | `crates/horizon-client/src/ambience.rs` | adaptable; reference; version-specific | Ambient state/fade/RNG model is useful, but 26.x environment attributes must be adapted to 1.20.1 biome effects. |
| 1041 | `crates/horizon-client/src/android_control_assets.rs` | adaptable; reference; version-specific | Use the integrity and pinned-revision download pattern only; Bedrock sample artwork is subject to Mojang terms and is not approved for redistribution. |
| 1042 | `crates/horizon-client/src/android_control_theme.rs` | adaptable; reference; optional/future | Horizon-specific theme/raster/widget system; reuse only generic UI drawing ideas in an independently designed Android theme. |
| 1043 | `crates/horizon-client/src/android_control_ui.rs` | adaptable; reference; version-specific | Editable touch-control editor and multi-pointer patterns are relevant; build a smaller RustChunk-owned UI/input subsystem. |
| 1044 | `crates/horizon-client/src/android_lib.rs` | adaptable; reference; version-specific | JNI/Android entry-boundary pattern; package, callbacks and lifecycle are app-specific. |
| 1045 | `crates/horizon-client/src/animate_tick.rs` | adaptable; reference; version-specific | Ambient tick architecture is useful, but newer block types and 26.x biome environment attributes must be rebuilt for 1.20.1. |
| 1046 | `crates/horizon-client/src/arrows.rs` | adaptable; reference; version-specific | Projectile physics and collision architecture useful; modern item components, registries and exact 1.20.1 arrow/trident hit flow need rebuild. |
| 1047 | `crates/horizon-client/src/attack_strength.rs` | adaptable; reference; version-specific | Small attack-cooldown model; verify legacy ItemStack identity, attack-speed and exact 1.20.1 thresholds. |
| 1048 | `crates/horizon-client/src/attribute_updates.rs` | adaptable; reference; version-specific | Thin player attribute/health update bridge; omit newer waypoint attributes and validate server-level semantics. |
| 1049 | `crates/horizon-client/src/auto_jump.rs` | adaptable; reference; version-specific | Auto-jump geometry/decision model; rebuild collision shapes and movement conditions from 1.20.1. |
| 1050 | `crates/horizon-client/src/banner.rs` | rewrite required; reference; version-specific | Rebuild the banner data bridge for 1.20.1 legacy NBT; only the ordered layer/rendering model is reusable. |

## Part 43 — Entries 1051–1075

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1051 | `crates/horizon-client/src/beehive.rs` | adaptable; reference; version-specific | Beehive storage/release/honey logic is relevant; replace 26.x environment attributes with 1.20.1 weather/time/obstruction rules. |
| 1052 | `crates/horizon-client/src/benchmark.rs` | adaptable; reference; optional/future | Reproducible scene/FPS/memory benchmark design; Horizon scenes/backend hooks are not RustChunk or Galaxy A22 measurements. Destructive benchmark world requires isolation. |
| 1053 | `crates/horizon-client/src/bin/horizon-ctl/commands.rs` | unnecessary; reference; optional/future | Horizon desktop control CLI is not core runtime; borrow a typed command registry only for a separately scoped developer tool. |
| 1054 | `crates/horizon-client/src/bin/horizon-ctl/launch.rs` | adaptable; reference; optional/future | Process discovery/readiness patterns are useful; binary layout, launcher manifests and desktop process model are Horizon-specific. |
| 1055 | `crates/horizon-client/src/bin/horizon-ctl/mcp.rs` | unnecessary; reference; optional/future | Optional desktop MCP automation is not a mobile client dependency; any future endpoint needs a restricted RustChunk allowlist. |
| 1056 | `crates/horizon-client/src/block_contact.rs` | unnecessary; reference; version-specific | Custom/modded block-contact callback is not part of vanilla 1.20.1; reconsider only with a separately designed extension ABI. |
| 1057 | `crates/horizon-client/src/block_entity_item.rs` | rewrite required; reference; version-specific | 26.3 data components cannot be used for 1.20.1 ItemStack/BlockEntityTag; rewrite legacy pick/place NBT and operator restrictions. |
| 1058 | `crates/horizon-client/src/block_pattern.rs` | adaptable; reference; version-specific | Generic oriented block-pattern matching can support 1.20.1 golems/Wither after target pattern/orientation verification. |
| 1059 | `crates/horizon-client/src/block_predictions.rs` | adaptable; reference; version-specific | Client block prediction/ack model useful; sequence, teleport and correction behavior must match the 1.20.1 protocol. |
| 1060 | `crates/horizon-client/src/block_transform.rs` | rewrite required; reference; version-specific | Reimplement 1.20.1 axe/hoe/shovel interactions independently; the newer block_transformer registry is not target data. |
| 1061 | `crates/horizon-client/src/boss_overlay.rs` | adaptable; reference; version-specific | Useful state model, but packet enums are version-specific. Verify duplicate-add order, invalid enum clamping and renderer effects against 1.20.1. |
| 1062 | `crates/horizon-client/src/breathing.rs` | unnecessary; reference; optional/future | Air arithmetic is useful, but purpose is a native mod event bridge with API-versioned server callbacks; RustChunk vanilla has no such extension lifecycle. |
| 1063 | `crates/horizon-client/src/cauldron.rs` | adaptable; reference; version-specific | Decision tree is reusable, but ItemStack component access differs from 1.20.1 legacy NBT/BlockEntityTag; sound/effect ordering needs target verification. |
| 1064 | `crates/horizon-client/src/clock.rs` | adaptable; reference; version-specific | Fixed-step accumulator is reusable; full management surface includes newer commands/sync beyond a basic 1.20.1 client timer. Unbounded catch-up risks stalls. |
| 1065 | `crates/horizon-client/src/command_storage.rs` | adaptable; reference; version-specific | Useful model, but namespace/path must be validated before creating paths; atomic writes, symlink traversal and dirty clearing on failed writes need scrutiny. |
| 1066 | `crates/horizon-client/src/confirm_screen.rs` | adaptable; reference; optional/future | Generic layout/focus behavior is reusable; exact atlas, fonts, styling and dimensions are Horizon UI details. |
| 1067 | `crates/horizon-client/src/container_screen.rs` | adaptable; reference; version-specific | High-value input model, not authoritative inventory logic. 1.20.1 click packets and Android multi-pointer/cancel need separate adapters. |
| 1068 | `crates/horizon-client/src/crafting.rs` | rewrite required; reference; version-specific | Essential domain but tied to 26.3 components, later recipe/display serializers, custom ingredients and NeoForge. Too coupled to port as one file. |
| 1069 | `crates/horizon-client/src/creative.rs` | adaptable; reference; version-specific | Tab catalogue, item components/equality and lock metadata are snapshot-specific; general search/click model remains useful. |
| 1070 | `crates/horizon-client/src/cushion.rs` | undetermined; reference; version-specific | No evidence this is vanilla 1.20.1 content; must be traced to target registry or supported mod. Generic support checks can be reused independently. |
| 1071 | `crates/horizon-client/src/custom_armor.rs` | unnecessary; reference; optional/future | Third-party mod/resource-pack compatibility, not vanilla armor; relies on ItemsAdder conventions and newer renderer APIs. |
| 1072 | `crates/horizon-client/src/data_cmd.rs` | adaptable; reference; version-specific | Algorithms useful, but NbtPath semantics/errors need 1.20.1 parity. String length may count UTF-8 bytes rather than Java UTF-16 code units. |
| 1073 | `crates/horizon-client/src/debug_chart.rs` | adaptable; reference; optional/future | Useful telemetry separation, but hotkeys/counters/sample semantics are snapshot-specific; no Android/GPU metrics guaranteed. |
| 1074 | `crates/horizon-client/src/debug_entries.rs` | adaptable; reference; optional/future | Configuration model reusable; entry catalogue, IDs and migration rules are 26.3-specific; profile schema cannot be copied unchanged. |
| 1075 | `crates/horizon-client/src/debug_keys.rs` | adaptable; reference; version-specific | Good key lifecycle pattern; later debug mappings and Horizon repro keys are not target data. Deliberate crash behavior is unsafe for production/mobile. |

## Part 44 — Entries 1076–1100

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1076 | `crates/horizon-client/src/debug_overlay.rs` | adaptable; reference; optional/future | Generic grouping useful; entry data is snapshot-specific. Linux /proc counters do not include GPU/Java memory; Android semantics need validation. Shader timings are Horizon-only. |
| 1077 | `crates/horizon-client/src/debugworld.rs` | adaptable; reference; version-specific | Useful deterministic QA world, but grid order/count depend on registry order/version; current numeric IDs cannot be reused. |
| 1078 | `crates/horizon-client/src/diagnostics/exit_reports.rs` | adaptable; reference; optional/future | Useful crash/OOM/ANR tooling. Raw traces/logs can contain sensitive content; keep local by default and redact before explicit export. Historical APIs require Android 11+. |
| 1079 | `crates/horizon-client/src/dialog_screen.rs` | unnecessary; reference; version-specific | Specific to server-dialog protocol/schema, not vanilla 1.20.1. General layout concepts don't create protocol compatibility. |
| 1080 | `crates/horizon-client/src/dialog.rs` | unnecessary; reference; version-specific | Protocol feature, not generic UI. Dialogs were added in Java 1.21.6; server-provided command/custom actions and input payloads are a trust boundary. |
| 1081 | `crates/horizon-client/src/discord_presence.rs` | unnecessary; reference; optional/future | Optional desktop feature, not gameplay or Android runtime; never expose addresses, world names or invite codes. |
| 1082 | `crates/horizon-client/src/end_crystal.rs` | adaptable; reference; version-specific | Rotation order/model-space convention must match Java 1.20.1 exactly. |
| 1083 | `crates/horizon-client/src/end_gateway.rs` | adaptable; reference; version-specific | Swept-AABB is reusable; destination search/cooldown/tick order require Java 1.20.1 parity. |
| 1084 | `crates/horizon-client/src/entities.rs` | rewrite required; reference; version-specific | Modern Item Components/later entity types are mixed in; ItemStack merging must use 1.20.1 NBT; resource-dependent early returns are skipped, not passed. |
| 1085 | `crates/horizon-client/src/entities/fluids.rs` | adaptable; reference; version-specific | Fluid heights/vectors/tags, eye-height and chunk rules are version-specific. |
| 1086 | `crates/horizon-client/src/entity_culling.rs` | adaptable; reference; optional/future | Fail-open is safe; partial blocks/leaves/glass/openings must not be treated fully opaque; mobile CPU cost matters. |
| 1087 | `crates/horizon-client/src/entity_nbt.rs` | rewrite required; reference; version-specific | 26.3 schema uses Item Components/modern entities; 1.20.1 uses old ItemStack NBT and different fields. |
| 1088 | `crates/horizon-client/src/entity_rules.rs` | adaptable; reference; version-specific | Per-entity overrides/constants must come from 1.20.1; blanket defaults are unsafe. |
| 1089 | `crates/horizon-client/src/game/blockctl.rs` | unnecessary; reference; optional/future | Horizon-only scripted test controls; exclude from the 1.20.1 core and consider only as bounded development tooling. |
| 1090 | `crates/horizon-client/src/equipment.rs` | unnecessary; reference; version-specific | New equipment/equippable format, not Java 1.20.1 armor layers. |
| 1091 | `crates/horizon-client/src/extras_hud.rs` | adaptable; reference; optional/future | Panel layout generic; minimap data/assets need target validation; not vanilla HUD. |
| 1092 | `crates/horizon-client/src/falling_block.rs` | adaptable; reference; version-specific | Mostly vanilla; exact formula/caps/wear/event order need 1.20.1. Mob impact test ignored, not passed. |
| 1093 | `crates/horizon-client/src/fire_render.rs` | adaptable; reference; version-specific | Sprite paths/timing, transforms and light must match 1.20.1; first-person overlay elsewhere. |
| 1094 | `crates/horizon-client/src/first_person.rs` | adaptable; reference; version-specific | Spear/kinetic/modern item components are out of 1.20.1; HandWorld is a useful boundary. |
| 1095 | `crates/horizon-client/src/flatgen.rs` | adaptable; reference; version-specific | Layer/spawn logic useful; modern structure overrides need 1.20.1 validation; missing-resource test early-return is not pass. |
| 1096 | `crates/horizon-client/src/friends_screen.rs` | unnecessary; reference; optional/future | Not vanilla multiplayer; tokens/network addresses sensitive. |
| 1097 | `crates/horizon-client/src/furnace.rs` | adaptable; reference; version-specific | Component equality/new fuel data must become 1.20.1 NBT ItemStack/fuel rules. |
| 1098 | `crates/horizon-client/src/game.rs` | rewrite required; reference; version-specific | Monolith mixes vanilla-like behavior with LAN/friends, native hooks, Discord, Bedrock inventory, LOD and later mechanics; ItemStack/NBT uses components. |
| 1099 | `crates/horizon-client/src/game/advancements_screen.rs` | adaptable; reference; version-specific | Five roots match vanilla; visibility/assets/text wrapping/server positions need 1.20.1 parity. UI separate from trigger service. |
| 1100 | `crates/horizon-client/src/game/advancements.rs` | rewrite required; reference; version-specific | Modern predicate/trigger schema, spear_mobs, entity variants and ItemStack components are not 1.20.1; tests unexecuted. |

## Part 45 — Entries 1101–1125

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1101 | `crates/horizon-client/src/game/agent_navigation.rs` | adaptable; reference; optional/future | keep only generic search ideas; do not import agent commands. |
| 1102 | `crates/horizon-client/src/game/agent_settings.rs` | unnecessary; reference; optional/future | do not port Horizon agent settings UI. |
| 1103 | `crates/horizon-client/src/game/agent.rs` | unnecessary; reference; optional/future | keep out of runtime; task patterns only for approved QA tooling. |
| 1104 | `crates/horizon-client/src/game/allays.rs` | adaptable; version-specific | preserve target Allay behavior but rebuild state and item persistence. |
| 1105 | `crates/horizon-client/src/game/armor_stands.rs` | adaptable; version-specific | port behavior and rebuild equipment/render data for 1.20.1. |
| 1106 | `crates/horizon-client/src/game/arms.rs` | undetermined; reference; version-specific | defer until Create and target release are confirmed. |
| 1107 | `crates/horizon-client/src/game/atlas_slots.rs` | adaptable; reference | integrate with RustChunk's atlas/upload lifecycle. |
| 1108 | `crates/horizon-client/src/game/attachment_sync.rs` | unnecessary; reference; optional/future | exclude Horizon attachment protocol. |
| 1109 | `crates/horizon-client/src/game/banners.rs` | adaptable; version-specific | rebuild registries, NBT and map integration for 1.20.1. |
| 1110 | `crates/horizon-client/src/game/basins.rs` | undetermined; reference; version-specific | defer until target Create 1.20.1 source is confirmed. |
| 1111 | `crates/horizon-client/src/game/beacon.rs` | adaptable; version-specific | port classic beacon behavior/UI and exclude TestInstanceBlock. |
| 1112 | `crates/horizon-client/src/game/bearings.rs` | undetermined; reference; version-specific | defer until exact target source and dependency are confirmed. |
| 1113 | `crates/horizon-client/src/game/beds.rs` | adaptable; version-specific | port ordinary bed cleanup and exclude unverified straw-bed behavior. |
| 1114 | `crates/horizon-client/src/game/beehives.rs` | adaptable; version-specific | preserve vanilla behavior while rebuilding state, NBT and integration. |
| 1115 | `crates/horizon-client/src/game/belts.rs` | undetermined; reference; version-specific | defer until Create and target release are confirmed. |
| 1116 | `crates/horizon-client/src/game/block_editor.rs` | adaptable; version-specific | keep only target Structure/Jigsaw screens; omit later test editors. |
| 1117 | `crates/horizon-client/src/game/block_edits.rs` | unnecessary; reference; optional/future | exclude Horizon bulk-edit API; retain bounded-read ideas for tooling. |
| 1118 | `crates/horizon-client/src/game/block_entity_anims.rs` | adaptable; version-specific | preserve vanilla animations and exclude custom/later paths. |
| 1119 | `crates/horizon-client/src/game/block_entity_ticks.rs` | rewrite required; reference; version-specific | rebuild target ticker registry/order and omit later/custom entities. |
| 1120 | `crates/horizon-client/src/game/block_items.rs` | rewrite required; reference; version-specific | replace modern components with 1.20.1 item NBT/BlockEntityTag. |
| 1121 | `crates/horizon-client/src/game/boat_authority_fixture.rs` | unnecessary; reference; optional/future | Boat authority test fixture only; no client code to port. |
| 1122 | `crates/horizon-client/src/game/boat_leads.rs` | unnecessary; reference; optional/future | Later boat-leash extension for happy-ghast-era behavior; exclude. |
| 1123 | `crates/horizon-client/src/game/boat_passengers.rs` | adaptable; version-specific | Adapt vanilla boat passenger/collision subset; remove LAN and newer hooks. |
| 1124 | `crates/horizon-client/src/game/boats.rs` | essential; rewrite required; version-specific | Essential vehicle feature area; implement anew for 1.20.1. |
| 1125 | `crates/horizon-client/src/game/books.rs` | adaptable; rewrite required; version-specific | Book and lectern features are relevant, but legacy NBT/protocol/UI must be rebuilt. |

## Part 46 — Entries 1126–1150

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1126 | `crates/horizon-client/src/game/bossbars.rs` | adaptable; version-specific | Boss-bar concepts adapt; version-specific packets and viewer logic require rebuilding. |
| 1127 | `crates/horizon-client/src/game/breezes.rs` | unnecessary; reference; optional/future; version-specific | Later Breeze-specific particles; exclude from 1.20.1 vanilla. |
| 1128 | `crates/horizon-client/src/game/brush.rs` | adaptable; version-specific | Archaeology is in target version; adapt legacy loot/NBT and interaction APIs. |
| 1129 | `crates/horizon-client/src/game/build_mining.rs` | unnecessary; optional/future | Unrelated build waiting-room mini-game; exclude from game client. |
| 1130 | `crates/horizon-client/src/game/bundles.rs` | unnecessary; reference; optional/future; version-specific | Modern bundle selection; optional only after target-version support is confirmed. |
| 1131 | `crates/horizon-client/src/game/campfire.rs` | adaptable; version-specific | Vanilla campfire cooking/rendering; adapt recipes, NBT and block-entity sync. |
| 1132 | `crates/horizon-client/src/game/campfire_douse.rs` | adaptable; version-specific | Vanilla dousing; adapt target tags and interaction prediction. |
| 1133 | `crates/horizon-client/src/game/cartography.rs` | adaptable; version-specific | Vanilla cartography; adapt map persistence/menu and omit native custom icons. |
| 1134 | `crates/horizon-client/src/game/cat_spawner.rs` | adaptable; version-specific | Vanilla cat spawner; rebuild POI, structures and RNG against 1.20.1. |
| 1135 | `crates/horizon-client/src/game/chat.rs` | adaptable; rewrite required; version-specific | Chat UI is important, but module must be split and secure-chat rebuilt for 1.20.1. |
| 1136 | `crates/horizon-client/src/game/chat_filter.rs` | unnecessary; optional/future | Horizon-only server moderation; exclude from client baseline. |
| 1137 | `crates/horizon-client/src/game/chiseled_bookshelf.rs` | adaptable; version-specific | Vanilla chiseled bookshelf; adapt slot geometry and entity/state sync. |
| 1138 | `crates/horizon-client/src/game/chutes.rs` | undetermined; reference; version-specific | Create-only chute; pending explicit 1.20.1 mod compatibility scope. |
| 1139 | `crates/horizon-client/src/game/client_ambient.rs` | adaptable; version-specific | Ambient particle framework adapts; exclude later effect identifiers and verify RNG. |
| 1140 | `crates/horizon-client/src/game/clipboard.rs` | adaptable; reference | Generic edit shortcut handling adapts; Android clipboard/IME requires native integration. |
| 1141 | `crates/horizon-client/src/game/code_of_conduct.rs` | unnecessary; reference; optional/future | Server-specific conduct prompt; exclude from baseline protocol unless the target server requires it. |
| 1142 | `crates/horizon-client/src/game/combat.rs` | adaptable; version-specific | Vanilla combat tracker concepts adapt; rebuild damage registry, save and event integration for 1.20.1. |
| 1143 | `crates/horizon-client/src/game/command_blocks.rs` | essential; rewrite required; version-specific | Command blocks are core gameplay/admin compatibility; rebuild execution, scheduling, editor and packets for 1.20.1. |
| 1144 | `crates/horizon-client/src/game/command_harness.rs` | unnecessary; reference; optional/future | Horizon-only headless test harness; keep outside the client core. |
| 1145 | `crates/horizon-client/src/game/command_minecarts.rs` | adaptable; version-specific | Vanilla command minecart behavior adapts; rebuild packet, cart NBT and tick scheduling for 1.20.1. |
| 1146 | `crates/horizon-client/src/game/commands.rs` | essential; rewrite required; version-specific | Command compatibility is important; rebuild modularly from the 1.20.1 dispatcher and protocol, excluding later commands. |
| 1147 | `crates/horizon-client/src/game/commands/admin.rs` | adaptable; reference; version-specific | Feedback broadcast classification is useful; rebuild the 1.20.1 key list rather than copy newer entries. |
| 1148 | `crates/horizon-client/src/game/commands/compute.rs` | unnecessary; reference; optional/future; version-specific | Later /compute command and context provider registries; exclude from vanilla 1.20.1. |
| 1149 | `crates/horizon-client/src/game/commands/data.rs` | essential; rewrite required; version-specific | NBT command support matters for maps/datapacks; rewrite serializers and entity mutation for 1.20.1. |
| 1150 | `crates/horizon-client/src/game/commands/effects.rs` | adaptable; version-specific | Vanilla effect/enchant command behavior adapts; translate registry and item data to target 1.20.1. |

## Part 47 — Entries 1151–1175

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1151 | `crates/horizon-client/src/game/commands/execute.rs` | essential; rewrite required; version-specific | /execute semantics are foundational; rebuild source/fork/condition/store logic against 1.20.1. |
| 1152 | `crates/horizon-client/src/game/commands/extra.rs` | unnecessary; reference; optional/future; version-specific | Mixed Horizon and later command collection; split and omit non-1.20.1 command surfaces. |
| 1153 | `crates/horizon-client/src/game/commands/items.rs` | essential; rewrite required; version-specific | Item command/slot behavior is useful, but rewrite component-based mutations to 1.20.1 ItemStack NBT. |
| 1154 | `crates/horizon-client/src/game/commands/misc.rs` | adaptable; version-specific | Team messaging is a target feature; isolate it from newer tick/stopwatch controls. |
| 1155 | `crates/horizon-client/src/game/commands/place.rs` | adaptable; version-specific | Vanilla /place operations adapt; target worldgen registries and NBT templates must be used. |
| 1156 | `crates/horizon-client/src/game/commands/players.rs` | adaptable; reference; version-specific | Player selector/display semantics adapt; LAN guest swapping is not client-core behavior. |
| 1157 | `crates/horizon-client/src/game/commands/pregen.rs` | unnecessary; reference; optional/future | Horizon server pregeneration extension; exclude from the 1.20.1 client core. |
| 1158 | `crates/horizon-client/src/game/commands/profiles.rs` | unnecessary; reference; optional/future | Dedicated-server user-list commands; keep outside Android client core. |
| 1159 | `crates/horizon-client/src/game/commands/raid.rs` | unnecessary; reference; optional/future | Developer raid-control command; omit from vanilla 1.20.1 command surface. |
| 1160 | `crates/horizon-client/src/game/commands/terrain_cmds.rs` | adaptable; version-specific | Forceload, fillbiome, worldborder and loot are target-relevant; rebuild against 1.20.1 world and item APIs. |
| 1161 | `crates/horizon-client/src/game/commands/test_locate.rs` | unnecessary; reference; optional/future; version-specific | Later test-instance/GameTest command extension; omit from vanilla 1.20.1. |
| 1162 | `crates/horizon-client/src/game/commands/test_runs.rs` | unnecessary; reference; optional/future; version-specific | Modern GameTest orchestration; not part of the target client baseline. |
| 1163 | `crates/horizon-client/src/game/commands/text_commands.rs` | adaptable; rewrite required; version-specific | Target command behavior adapts, but text component and packet implementation require a 1.20.1 rewrite. |
| 1164 | `crates/horizon-client/src/game/commands/world_cmds.rs` | adaptable; rewrite required; version-specific | Many world commands are target-relevant, but their shared modern implementation must be split and rewritten. |
| 1165 | `crates/horizon-client/src/game/composter.rs` | adaptable; version-specific | Vanilla composter behavior adapts after replacing modern compute/provider data with target tags/chances. |
| 1166 | `crates/horizon-client/src/game/consume.rs` | adaptable; rewrite required; version-specific | Eating/drinking is core and adaptable, but component-based item effects require a 1.20.1 rewrite. |
| 1167 | `crates/horizon-client/src/game/control.rs` | unnecessary; reference; optional/future | Horizon-only privileged control socket; keep out of the release client core. |
| 1168 | `crates/horizon-client/src/game/copper_golems.rs` | unnecessary; reference; optional/future; version-specific | Later copper-golem/copper-chest features; exclude from vanilla 1.20.1. |
| 1169 | `crates/horizon-client/src/game/cosmetics.rs` | unnecessary; reference; optional/future | Horizon-specific cosmetics service; not required for gameplay or server compatibility. |
| 1170 | `crates/horizon-client/src/game/creaking.rs` | unnecessary; reference; optional/future; version-specific | Later Creaking/Creaking Heart subsystem; exclude from 1.20.1 baseline. |
| 1171 | `crates/horizon-client/src/game/creative_hotbar.rs` | adaptable; version-specific | Creative hotbar behavior adapts; rewrite persistence to 1.20.1 item NBT. |
| 1172 | `crates/horizon-client/src/game/creative_shift.rs` | unnecessary; reference | Horizon-specific build workaround; omit. |
| 1173 | `crates/horizon-client/src/game/creatures.rs` | essential; rewrite required; version-specific | Critical creature engine area; must be modularly rewritten for 1.20.1 with later species and Horizon/LAN code excluded. |
| 1174 | `crates/horizon-client/src/game/crossbow.rs` | adaptable; rewrite required; version-specific | Crossbow behavior adapts; charged component storage must be rewritten to target NBT. |
| 1175 | `crates/horizon-client/src/game/cushions.rs` | unnecessary; reference; optional/future | Horizon-only cushion furniture entity; exclude from vanilla 1.20.1. |

## Part 48 — Entries 1176–1200

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1176 | `crates/horizon-client/src/game/debug_keys.rs` | adaptable; rewrite required; version-specific | Debug keys are useful, but split/rewrite them against 1.20.1 and strip Horizon-only controls. |
| 1177 | `crates/horizon-client/src/game/debug_lines.rs` | adaptable; version-specific | F3 data model adapts; rebuild exact 1.20.1 entry set and presentation. |
| 1178 | `crates/horizon-client/src/game/debug_options.rs` | unnecessary; reference; optional/future; version-specific | Newer 26.3 DebugOptionsScreen; omit from the 1.20.1 baseline. |
| 1179 | `crates/horizon-client/src/game/debug_renderers.rs` | adaptable; version-specific | Selected debug geometry can adapt; target flags/data and mobile scan costs must be rebuilt. |
| 1180 | `crates/horizon-client/src/game/debug_stick.rs` | adaptable; rewrite required; version-specific | Vanilla debug stick adapts, but component persistence must be rewritten to 1.20.1 NBT. |
| 1181 | `crates/horizon-client/src/game/decorated_pot.rs` | adaptable; version-specific | Vanilla decorated pot is target-relevant; adapt loot/item NBT and update packets. |
| 1182 | `crates/horizon-client/src/game/dedicated.rs` | unnecessary; reference; optional/future | Horizon dedicated-server runtime; keep out of Android client. |
| 1183 | `crates/horizon-client/src/game/dedicated/events.rs` | unnecessary; reference; optional/future | Horizon dedicated-server event stream; not a client requirement. |
| 1184 | `crates/horizon-client/src/game/dedicated/management.rs` | unnecessary; reference; optional/future | Horizon privileged management RPCs; exclude from Android client. |
| 1185 | `crates/horizon-client/src/game/dedicated/report.rs` | unnecessary; reference; optional/future | Horizon dedicated telemetry; optional future server feature only. |
| 1186 | `crates/horizon-client/src/game/dedicated/status.rs` | unnecessary; reference; optional/future | Horizon host status endpoint; not server-list ping or client functionality. |
| 1187 | `crates/horizon-client/src/game/deflect.rs` | adaptable; version-specific | Only target-version deflection behavior may adapt; remove later wind-charge types. |
| 1188 | `crates/horizon-client/src/game/diagnostics.rs` | unnecessary; reference; optional/future | Horizon device diagnostics; optional, separate from the game engine. |
| 1189 | `crates/horizon-client/src/game/dialogs.rs` | unnecessary; reference; optional/future; version-specific | Modern dialog protocol/UI; omit from 1.20.1 baseline. |
| 1190 | `crates/horizon-client/src/game/discord.rs` | unnecessary; reference; optional/future | Optional Discord Rich Presence; not a core launcher/game feature. |
| 1191 | `crates/horizon-client/src/game/dispense_fixture.rs` | unnecessary; reference; optional/future | Horizon dispenser parity harness; retain methodology only, not game runtime. |
| 1192 | `crates/horizon-client/src/game/dispenser.rs` | essential; rewrite required; version-specific | Dispenser is essential but must be modularly rewritten for 1.20.1 with newer content removed. |
| 1193 | `crates/horizon-client/src/game/dragon.rs` | essential; rewrite required; version-specific | Essential End boss feature; modularly rewrite for 1.20.1 and strip later extensions. |
| 1194 | `crates/horizon-client/src/game/dynamic_lights.rs` | unnecessary; reference; optional/future | Horizon optional dynamic lighting; omit from vanilla baseline. |
| 1195 | `crates/horizon-client/src/game/enchant_effects.rs` | essential; rewrite required; version-specific | Enchantment gameplay is essential; rewrite modern effect components into 1.20.1 enchant logic. |
| 1196 | `crates/horizon-client/src/game/end.rs` | essential; rewrite required; version-specific | End dimension features are essential; rewrite gateway/portal and persistence against 1.20.1. |
| 1197 | `crates/horizon-client/src/game/entity.rs` | essential; rewrite required; version-specific | Shared entity abstraction is essential; rebuild kind roster/behavior and Forge extension points for 1.20.1. |
| 1198 | `crates/horizon-client/src/game/entity_culling.rs` | adaptable; optional/future; version-specific | Optional rendering optimization; preserve correctness and profile before enabling on Android. |
| 1199 | `crates/horizon-client/src/game/entity_model_loading_tests.rs` | unnecessary; reference; optional/future | Test-only model-loading benchmark/equivalence fixture; no runtime code to port. |
| 1200 | `crates/horizon-client/src/game/event_loop.rs` | adaptable; reference | Small reusable platform abstraction; adapt to Android/headless runtime. |

## Part 49 — Entries 1201–1225

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1201 | `crates/horizon-client/src/game/events.rs` | unnecessary; optional/future | Horizon agent event feed and wait-for API; keep outside vanilla client baseline. |
| 1202 | `crates/horizon-client/src/game/execution_role.rs` | adaptable; reference | Execution-role separation helps distinguish remote client and integrated-server ownership. |
| 1203 | `crates/horizon-client/src/game/explosion.rs` | essential; rewrite required; version-specific | Core explosion physics/damage/drop behavior; rebuild to 1.20.1 and remove later extensions. |
| 1204 | `crates/horizon-client/src/game/extras.rs` | unnecessary; optional/future | Horizon Extras HUD conveniences; optional features, not game core. |
| 1205 | `crates/horizon-client/src/game/falling_damage.rs` | essential; rewrite required; version-specific | Falling-block/anvil/dripstone impact behavior is target gameplay; remove later/custom sync cases. |
| 1206 | `crates/horizon-client/src/game/falling_trees.rs` | unnecessary; optional/future | Create-specific falling-tree simulation; exclude from vanilla baseline. |
| 1207 | `crates/horizon-client/src/game/fangs.rs` | essential; rewrite required; version-specific | Evoker Fangs attack timing, owner damage and hitbox are target gameplay. |
| 1208 | `crates/horizon-client/src/game/fans.rs` | unnecessary; optional/future | Create Encased Fan and recipe processing; mod-specific only. |
| 1209 | `crates/horizon-client/src/game/files_offer.rs` | unnecessary; optional/future | Horizon Android file-permission/migration UI; keep in launcher layer. |
| 1210 | `crates/horizon-client/src/game/fireworks.rs` | essential; rewrite required; version-specific | Firework rockets and Elytra boosts are essential; replace components/LAN and later deflection. |
| 1211 | `crates/horizon-client/src/game/fishing.rs` | essential; rewrite required; version-specific | Fishing hook physics, loot and interactions are essential 1.20.1 behavior. |
| 1212 | `crates/horizon-client/src/game/fov.rs` | adaptable; rewrite required; version-specific | Camera FOV, bobbing, spyglass and screen effects adapt to target renderer. |
| 1213 | `crates/horizon-client/src/game/foxes.rs` | adaptable; version-specific | Fox/Dolphin pickup and item-seeking rules adapt after target AI/tag verification. |
| 1214 | `crates/horizon-client/src/game/frames.rs` | essential; rewrite required; version-specific | Item-frame placement, interaction, drops, maps and rendering are target features. |
| 1215 | `crates/horizon-client/src/game/friends.rs` | unnecessary; optional/future | Horizon invite-code P2P/LAN sharing is separate from vanilla server joining. |
| 1216 | `crates/horizon-client/src/game/functions.rs` | essential; rewrite required; version-specific | Datapacks, functions, predicates and scheduling are core; rebuild 1.20.1 format/commands. |
| 1217 | `crates/horizon-client/src/game/funnels.rs` | unnecessary; optional/future | Create Funnel/Belt Funnel item transport is mod-specific, not vanilla. |
| 1218 | `crates/horizon-client/src/game/game_mode.rs` | essential; rewrite required; version-specific | Game-mode/spectator mechanics are core; convert adventure predicates to legacy NBT. |
| 1219 | `crates/horizon-client/src/game/game_tests.rs` | adaptable; reference; optional/future | GameTest runner is useful only as a versioned test harness, not shipping runtime. |
| 1220 | `crates/horizon-client/src/game/gamemode_switcher.rs` | adaptable; reference; version-specific | F3+F4 switcher is a target UI reference; adapt input, authority and mobile navigation. |
| 1221 | `crates/horizon-client/src/game/ghasts.rs` | unnecessary; optional/future | Happy Ghast harness/mount/dismount, rider state and hatching; includes selected vehicle collision helpers. |
| 1222 | `crates/horizon-client/src/game/glowing.rs` | adaptable; version-specific | Tracks glowing status for remote entities and resolves outline/team color for client rendering. |
| 1223 | `crates/horizon-client/src/game/golems.rs` | essential; rewrite required; version-specific | Recognizes spawn patterns and creates snow/iron golems alongside later copper-golem/copper-chest additions. |
| 1224 | `crates/horizon-client/src/game/growth.rs` | adaptable; rewrite required; version-specific | Implements sapling/plant growth and a shears interaction that removes eligible plant heads. |
| 1225 | `crates/horizon-client/src/game/hand.rs` | essential; rewrite required; version-specific | Implements first-person hand interactions: ray picking, attacks, block breaking, item-use clicks, cooldown, swings and equip state; includes later reach/piercing features. |

## Part 50 — Entries 1226–1250

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1226 | `crates/horizon-client/src/game/headless.rs` | adaptable; reference; optional/future | Runs a headless Game loop and shutdown/save at nominal 50 ms cadence. |
| 1227 | `crates/horizon-client/src/game/ime.rs` | adaptable; version-specific | Handles OS IME preedit/commit, candidate/caret placement and text dispatch to focused screens. |
| 1228 | `crates/horizon-client/src/game/imports.rs` | unnecessary; optional/future | Polls an Android Open With/share/file-picker inbox and presents import progress/results via HUD banners and chime. |
| 1229 | `crates/horizon-client/src/game/initial_spawn.rs` | essential; rewrite required; version-specific | Finds world/player spawn and respawn locations and places bonus chest/loot block entity in new worlds. |
| 1230 | `crates/horizon-client/src/game/input_mode.rs` | adaptable; rewrite required; version-specific | Selects Touch/KeyboardMouse/Gamepad mode and translates Android pointer-capture/mouse events into game window events. |
| 1231 | `crates/horizon-client/src/game/input_trace.rs` | unnecessary; reference; optional/future | Records keyboard/mouse/gamepad/control-socket input origin; exposes recent summaries and optional deterministic repro recording. |
| 1232 | `crates/horizon-client/src/game/integrated.rs` | adaptable; optional/future | Runs an integrated singleplayer server on its own thread with asynchronous opening, in-memory client Pipe, task queue, pause/save, progress and optional LAN publishing. |
| 1233 | `crates/horizon-client/src/game/interactions.rs` | adaptable; version-specific | Represents and ray-picks remote minecraft:interaction entities: invisible server-defined hitboxes consuming clicks and optionally riding another entity. |
| 1234 | `crates/horizon-client/src/game/item_draw.rs` | essential; rewrite required; version-specific | Binds item-model resolver to game data and submits GUI, dropped, held, framed and skinned item geometry with lighting/glint/HDR. |
| 1235 | `crates/horizon-client/src/game/item_particles.rs` | adaptable; rewrite required; version-specific | Resolves item particle sprites and creates eating/use particles and equipment-break feedback/sounds. |
| 1236 | `crates/horizon-client/src/game/item_use.rs` | essential; rewrite required; version-specific | Implements item-use duration/animation, start/tick/stop/finish hooks, consumable completion and bow charge/release math. |
| 1237 | `crates/horizon-client/src/game/jigsaw_blocks.rs` | adaptable; rewrite required; version-specific | Handles server-side jigsaw state initialization, edit packets and structure-pool generation. |
| 1238 | `crates/horizon-client/src/game/jigsaw_edit.rs` | adaptable; version-specific | Implements jigsaw editor: pool/name/target/final state/priorities/joint, depth slider and Generate action. |
| 1239 | `crates/horizon-client/src/game/jukebox.rs` | essential; rewrite required; version-specific | Implements jukebox insertion/removal, playback, level events 1010/1011, notes, parrot dancing, redstone/comparator and saving. |
| 1240 | `crates/horizon-client/src/game/key_focus.rs` | adaptable; reference; optional/future | Implements keyboard/controller focus navigation for buttons, fields, rows, tabs and sliders. |
| 1241 | `crates/horizon-client/src/game/kinetic_machines.rs` | unnecessary; optional/future | Implements Create kinetic-machine entity behavior and integration for moving mechanical blocks and machines. |
| 1242 | `crates/horizon-client/src/game/kinetics.rs` | unnecessary; reference; optional/future | Implements Create kinetic network propagation, connected rotational networks, stress/speed calculation and mechanical movement. |
| 1243 | `crates/horizon-client/src/game/lan_actions.rs` | unnecessary; reference | Routes actions from Horizon LAN guests to the local host world: movement, interactions, attacks, block breaking, item use, menus, chat, commands and respawn. |
| 1244 | `crates/horizon-client/src/game/lan_attributes.rs` | adaptable; rewrite required; version-specific | Builds server-synchronized attribute snapshots for mobs, armor stands and players, including item/effect modifiers and creative reach changes. |
| 1245 | `crates/horizon-client/src/game/lan_camera.rs` | adaptable; reference; version-specific | Implements spectator guest camera selection, target tracking and reset when the camera dies, the player sneaks, leaves spectator mode or teleports. |
| 1246 | `crates/horizon-client/src/game/lan_chat.rs` | essential; rewrite required; version-specific | Handles LAN guest signed/unsigned chat commands, signed arguments, last-seen acknowledgements, secure-chat validation and spam throttling. |
| 1247 | `crates/horizon-client/src/game/lan_crafting.rs` | adaptable; rewrite required; version-specific | Implements LAN guest crafting and container operations with the host menu engine, including crafting and furnace-like menus. |
| 1248 | `crates/horizon-client/src/game/lan_entities.rs` | essential; rewrite required; version-specific | Builds and serializes tracked non-mob entities including boats, minecarts, armor stands, item frames and paintings, plus passenger/leash relationships. |
| 1249 | `crates/horizon-client/src/game/lan_far.rs` | unnecessary; optional/future | Provides Horizon-only far-terrain requests over horizon:lod custom payload, samples distant terrain on a worker and returns bounded replies. |
| 1250 | `crates/horizon-client/src/game/lan_mobs.rs` | unnecessary; reference; optional/future | Chooses which player a mob targets in a hosted LAN world and builds player snapshots for host/guests, including effects, equipment and combat attribution. |

## Part 51 — Entries 1251–1275

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1251 | `crates/horizon-client/src/game/lan_mount_menu.rs` | adaptable; rewrite required; version-specific | Implements a guest horse/mount inventory menu, slot sync, click handling, mount inventory and validity checks. |
| 1252 | `crates/horizon-client/src/game/lan_particles.rs` | adaptable; rewrite required; version-specific | Constructs and sends server particle packets near LAN guests, including option serialization, spread/randomization, effects and explosion particles. |
| 1253 | `crates/horizon-client/src/game/lan_pvp.rs` | unnecessary; reference; optional/future | Implements server-side PvP melee/projectile damage including critical/sweep, shields, armor, knockback, teams and PvP rule. |
| 1254 | `crates/horizon-client/src/game/lan_quiet.rs` | unnecessary; optional/future | Implements Horizon horizon:quiet_commands batch API for mods, queueing quiet commands with per-player limits and suppressed output. |
| 1255 | `crates/horizon-client/src/game/lan_screen.rs` | unnecessary; optional/future | Implements the 26.3 WorldOptionsScreen with modes, cheats, difficulty lock, game rules, restrictions and modern Open-to-LAN controls. |
| 1256 | `crates/horizon-client/src/game/lan_stations.rs` | adaptable; rewrite required; version-specific | Implements LAN guest work-station and lectern menus, including open, slots, data sync, button clicks and inventory return on close. |
| 1257 | `crates/horizon-client/src/game/lan_trading.rs` | adaptable; rewrite required; version-specific | Implements villager and wandering trader menus, offer synchronization, selecting trades, payment slots, XP and reputation. |
| 1258 | `crates/horizon-client/src/game/lan_vehicles.rs` | adaptable; rewrite required; version-specific | Handles server-side LAN guest driving of boats/minecarts, rider input, paddling, dismount and vehicle movement events. |
| 1259 | `crates/horizon-client/src/game/lan_waypoints.rs` | unnecessary; optional/future | Maintains per-dimension player locator-bar waypoints for hosted world, including team colors, hide conditions, range and respawn tracking. |
| 1260 | `crates/horizon-client/src/game/lan.rs` | unnecessary; reference; optional/future | Hosts an integrated world on LAN, manages Hub/server threads, syncs registries/tags/packs, streams chunks/entities and dispatches guest events. |
| 1261 | `crates/horizon-client/src/game/launch_api.rs` | adaptable; reference; optional/future | Defines the public command-line Launch API, launcher options, JSON capability reply, exit codes and generated Markdown reference. |
| 1262 | `crates/horizon-client/src/game/leads.rs` | adaptable; rewrite required; version-specific | Implements lead attachment, leash knot creation/removal, entity and fence leashing, leash following, drops and leash rendering. |
| 1263 | `crates/horizon-client/src/game/level_particles.rs` | adaptable; rewrite required; version-specific | Translates level events and server/world actions into particle effects, sounds and local visual feedback. |
| 1264 | `crates/horizon-client/src/game/levels.rs` | essential; rewrite required; version-specific | Manages multiple loaded levels/dimensions, active and parked worlds, transitions and per-level world state. |
| 1265 | `crates/horizon-client/src/game/lightning.rs` | adaptable; rewrite required; version-specific | Implements lightning strike selection, bolt lifecycle, fire placement, entity damage/transformations and lightning-rod interactions. |
| 1266 | `crates/horizon-client/src/game/llamas.rs` | adaptable; version-specific | Implements llama spit projectile behavior, caravan following and llama-specific interaction/state helpers. |
| 1267 | `crates/horizon-client/src/game/locator_bar.rs` | unnecessary; optional/future; version-specific | Implements the later locator-bar HUD that shows directional player/entity indicators and related target markers. |
| 1268 | `crates/horizon-client/src/game/loom.rs` | adaptable; rewrite required; version-specific | Implements loom menu inputs, banner pattern selection, dyeing, result creation and client-side loom screen rendering. |
| 1269 | `crates/horizon-client/src/game/mace.rs` | unnecessary; version-specific | Implements the mace weapon's smash/fall-distance damage, knockback, wind-burst effects and mace-specific attack behavior. |
| 1270 | `crates/horizon-client/src/game/machine_entities.rs` | unnecessary; optional/future | Stores custom content-mod machine block-entity state such as Create depots, basins, belts, millstones, presses, mixers, saws, bearings and arms. |
| 1271 | `crates/horizon-client/src/game/machines.rs` | essential; rewrite required; version-specific | Implements vanilla container transfer and block behavior: hoppers, dispensers/droppers, container slot rules, item movement, cooldowns and comparator signals; also houses native-mod extensions. |
| 1272 | `crates/horizon-client/src/game/machines/crafter.rs` | unnecessary; version-specific | Implements the redstone-triggered crafter block, crafting animation, result/remainder ejection and crafter menu UI. |
| 1273 | `crates/horizon-client/src/game/machines/native_energy.rs` | unnecessary; optional/future | Implements a Horizon native-mod energy-buffer API for reading, inserting, extracting and moving energy between mod block entities. |
| 1274 | `crates/horizon-client/src/game/machines/native_fluids.rs` | unnecessary; optional/future | Implements a Horizon native-mod fluid tank API with sided tank access, cauldron conversion, insertion/extraction and block-to-block fluid transfer. |
| 1275 | `crates/horizon-client/src/game/machines/native_items.rs` | unnecessary; optional/future | Implements Horizon native-mod sided container access, item insertion/extraction and loss-avoiding item movement. |

## Part 52 — Entries 1276–1300

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1276 | `crates/horizon-client/src/game/maps.rs` | essential; rewrite required; version-specific | Implements map creation, exploration-map resolution, terrain color sampling, saved map data, player/banner/frame decorations, remote map packets and map rendering. |
| 1277 | `crates/horizon-client/src/game/memory.rs` | unnecessary; optional/future | Builds a diagnostic memory census for client/server worlds, shared chunk sections, generation buffers, renderer CPU/GPU allocations and allocator tags. |
| 1278 | `crates/horizon-client/src/game/minecarts.rs` | essential; rewrite required; version-specific | Implements minecart entities, rail shapes and movement, powered/activator rails, collisions, riders, variants, containers, TNT explosions, persistence and rendering. |
| 1279 | `crates/horizon-client/src/game/mob_blocks.rs` | adaptable; rewrite required; version-specific | Renders items and blocks held/worn by mobs, including enderman carried blocks, snow-golem pumpkin, mooshroom mushrooms, iron-golem poppy and raid-captain banners. |
| 1280 | `crates/horizon-client/src/game/mob_gear.rs` | adaptable; rewrite required; version-specific | Handles mob spawn equipment enchantments, equipment-table loot rolls, Halloween heads and raid-specific pillager/vindicator buffs. |
| 1281 | `crates/horizon-client/src/game/mob_index.rs` | adaptable; reference; version-specific | Provides a cached network-entity-ID to mob-vector-index lookup with validation and rebuild-on-miss. |
| 1282 | `crates/horizon-client/src/game/mob_looting.rs` | adaptable; rewrite required; version-specific | Implements mob item pickup and equipment replacement: armor/weapons, drop chance for replaced gear and pickup advancements. |
| 1283 | `crates/horizon-client/src/game/mob_sounds.rs` | adaptable; version-specific | Starts and maintains looping bee, guardian and sniffer sounds, plus later Happy Ghast/Nautilus riding loops. |
| 1284 | `crates/horizon-client/src/game/mobile_controls.rs` | essential; rewrite required; version-specific | Implements Android touch controls, virtual mouse, custom bindings, keyboard/preset input, haptics, gestures, inventory/menu touch and contextual interaction. |
| 1285 | `crates/horizon-client/src/game/mobile_controls/mobile_platform.rs` | adaptable; reference; version-specific | Bridges the mobile control layer to Android text mirror, keyboard events, vibration and opening the Reports folder. |
| 1286 | `crates/horizon-client/src/game/mod_api_host.rs` | unnecessary; optional/future | Hosts the Horizon native-mod API for level reads/writes, attachments, random ticks, scheduled block/fluid ticks and authoritative entity access. |
| 1287 | `crates/horizon-client/src/game/mod_api_host_generated.rs` | unnecessary; reference; optional/future | Generated implementation of the large Horizon native-mod Host trait, forwarding mod calls to Game's native_* methods. |
| 1288 | `crates/horizon-client/src/game/mod_assets.rs` | adaptable; optional/future | Downloads a content mod's asset JAR from Modrinth, resumes partial transfers, checks SHA-512 and unpacks the manifest-selected assets. |
| 1289 | `crates/horizon-client/src/game/mod_catalog.rs` | unnecessary; optional/future | Implements the Android in-game catalog screen for mods, resource packs and shader packs: search, paging, install progress and confirmed removal. |
| 1290 | `crates/horizon-client/src/game/mods.rs` | adaptable; rewrite required; optional/future | Loads world-tied content mods and their assets, registers mod blocks/items/effects/entities/menus and lists content/native/WebAssembly mod status. |
| 1291 | `crates/horizon-client/src/game/mount_menu.rs` | essential; rewrite required; version-specific | Implements horse-family mount inventory UI, saddle/armor/chest slots, slot rules, shift-click, remote menu opening and mount preview rendering. |
| 1292 | `crates/horizon-client/src/game/mounts.rs` | essential; rewrite required; version-specific | Implements mount interaction, feeding, saddling, horse jump charging, rider input/position, dismount collision search, mount fall damage and HUD bars. |
| 1293 | `crates/horizon-client/src/game/moving_pistons.rs` | adaptable; version-specific | Renders moving piston block entities with interpolated block geometry and a temporary fallback while the destination chunk mesh is pending. |
| 1294 | `crates/horizon-client/src/game/music.rs` | adaptable; version-specific | Selects situational/background music, manages music frequency and pause behavior, ambient player sounds, sound engine ticking and now-playing toast. |
| 1295 | `crates/horizon-client/src/game/names.rs` | essential; rewrite required; version-specific | Builds and renders mob/player/armor-stand/item-frame name tags, scoreboard team visibility/colors, below-name scores and remote name metadata. |
| 1296 | `crates/horizon-client/src/game/narrator.rs` | adaptable; optional/future | Integrates text-to-speech narrator status, keyboard hotkey, screen narration, input narration and narrator toggle toast. |
| 1297 | `crates/horizon-client/src/game/native_biome_colors.rs` | unnecessary; reference; optional/future | Allows Horizon native mods to recolor biome grass/foliage/water and triggers chunk/LOD resource rebuild when colors change. |
| 1298 | `crates/horizon-client/src/game/native_block_entity_draw.rs` | unnecessary; reference; optional/future | Provides a native-mod block-entity drawing API with pose stack transforms, custom quads/cubes/blocks/items, culling bounds and cached kept geometry. |
| 1299 | `crates/horizon-client/src/game/native_builds.rs` | unnecessary; optional/future | Android UI and workflow for compiling/download/installing native-mod game libraries from Starling or a custom build server, then selecting the library for the next app start. |
| 1300 | `crates/horizon-client/src/game/native_claimed_items.rs` | unnecessary; reference; optional/future | Lets native client mods claim item right-clicks, receive bounded UseStart/UseStop events, inspect held-item data and raycast blocks including far LOD terrain. |

## Part 53 — Entries 1301–1325

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1301 | `crates/horizon-client/src/game_mode_switcher.rs` | adaptable; reference; version-specific | Implements the F3+F4 debug gamemode selector: mode icon selection, cycling, pointer hit testing, hover state and display geometry. |
| 1302 | `crates/horizon-client/src/game_rules_screen.rs` | essential; rewrite required; version-specific | Implements the gamerule settings screen, including rule search, boolean controls, bounded integer editing, category layout, scrolling, focus, keyboard/IME behavior and apply/discard actions. |
| 1303 | `crates/horizon-client/src/gizmos.rs` | adaptable; reference; version-specific | Provides debug gizmo geometry and rendering helpers for lines, cuboids, arrows, chunk borders, hitboxes and camera-relative overlays. |
| 1304 | `crates/horizon-client/src/gossip.rs` | unnecessary; reference; version-specific | Implements villager gossip/reputation storage, weighted reputation aggregation, caps, decay, transfer between villagers and NBT serialization. |
| 1305 | `crates/horizon-client/src/health.rs` | adaptable; rewrite required; version-specific | Contains the Horizon health/damage model for resident players: health and absorption, damage sources, armor/effects, knockback, blocking, totem behavior, environmental damage, healing and hurt-state handling. |
| 1306 | `crates/horizon-client/src/hotbar.rs` | adaptable; rewrite required; version-specific | Reads and writes saved creative hotbar groups, normalizes group/item data and filters unsupported entries. |
| 1307 | `crates/horizon-client/src/hunger.rs` | adaptable; rewrite required; version-specific | Implements server-side FoodData behavior: food/saturation/exhaustion, peaceful regeneration, healing, starvation and tick transitions. |
| 1308 | `crates/horizon-client/src/import.rs` | adaptable; rewrite required; version-specific | Implements Android import/share handling for resource packs, worlds, mods, content archives, skins and other incoming files, including classification and extraction. |
| 1309 | `crates/horizon-client/src/inv_sort.rs` | unnecessary; reference; optional/future | Implements an inventory sort operation that simulates item movement/stack merging and arranges stacks by a chosen ordering. |
| 1310 | `crates/horizon-client/src/item_activation.rs` | adaptable; reference; version-specific | Calculates the totem-of-undying activation overlay animation, including elapsed ticks, random placement, rotation, scale and easing. |
| 1311 | `crates/horizon-client/src/item_render.rs` | essential; rewrite required; version-specific | Implements the modern item renderer core: item model selection, baked model/quad rendering, atlas/model resources, display contexts, tint, foil/glint, special models and CPU/GPU render paths. |
| 1312 | `crates/horizon-client/src/item_render/cit.rs` | unnecessary; reference; optional/future | Implements an OptiFine-style Custom Item Textures (CIT) extension: rules, aliases, alternate models/textures, damage and worn conditions, enchantment glint and caching. |
| 1313 | `crates/horizon-client/src/item_render/gui.rs` | essential; rewrite required; version-specific | Contains GUI-specific item rendering helpers for lighting, UV/polygon clipping, face depth sorting, glint, book pose and cached sprite/render state. |
| 1314 | `crates/horizon-client/src/item_render/gui/depth.rs` | adaptable; reference; version-specific | Implements GUI item face-depth sorting, including splitting intersecting faces and interpolating their UV coordinates. |
| 1315 | `crates/horizon-client/src/item_render/special.rs` | essential; rewrite required; version-specific | Implements special 3D item renderers for bells, books, conduit, chests, shulker boxes, heads/player heads, tridents, banners, shields, decorated pots and newer special models. |
| 1316 | `crates/horizon-client/src/item_sync.rs` | essential; rewrite required; version-specific | Serializes and deserializes modern network ItemStacks, mapping registry IDs and DataComponentPatch entries to item data/NBT and handling dynamic registry-backed component values. |
| 1317 | `crates/horizon-client/src/item_tooltip.rs` | essential; rewrite required; version-specific | Builds item tooltip content and formatting from item data: names/lore, attributes, rarity, potion effects, map/player-head data, bees, books, jukeboxes, dyed items, fireworks, smithing templates and container contents. |
| 1318 | `crates/horizon-client/src/kinetics.rs` | unnecessary; reference; optional/future | Implements Create-style kinetic machinery: shafts, gears, gearboxes, motors, cranks, belts, water wheels, RPM propagation, network stress/capacity and gear phases. |
| 1319 | `crates/horizon-client/src/lan/advancements.rs` | unnecessary; reference; optional/future | Implements LAN/dedicated-server advancement and recipe-book handling: first-tick criteria, visible advancement packets, progress updates, rewards, reload and recipe-display encoding. |
| 1320 | `crates/horizon-client/src/lan/anticheat.rs` | unnecessary; reference; optional/future | Implements Horizon dedicated-server anticheat checks for timer, combat, movement prediction and ground claims, with strike persistence, alerts, logging, kicks and bans. |
| 1321 | `crates/horizon-client/src/lan/attributes.rs` | essential; rewrite required; version-specific | Converts live attribute instances to network snapshots and applies received snapshots, including base values and ordered modifiers. |
| 1322 | `crates/horizon-client/src/lan/border.rs` | adaptable; reference; version-specific | Tracks the world border state last sent to connected players and constructs outbound packets for initialization and border changes. |
| 1323 | `crates/horizon-client/src/lan/chunks.rs` | adaptable; reference; version-specific | Implements outbound chunk/biome/light packet encoding for resident and saved columns, including palette conversion, block entity update tags, heightmaps, light masks/layers, cache management and multi-threaded warm-up. |
| 1324 | `crates/horizon-client/src/lan/code_of_conduct.rs` | unnecessary; reference; optional/future | Reads optional language-specific server code-of-conduct text files, strips formatting color codes, normalizes line endings and selects a language/fallback document. |
| 1325 | `crates/horizon-client/src/lan/command_order.rs` | unnecessary; reference; optional/future | Stores a generated table of command-tree node paths and child names to impose deterministic registration/display order on the LAN command tree. |

## Part 54 — Entries 1326–1350

| No. | Source path | Decision | One-line reason ||---:|---|---|---|
| 1326 | `crates/horizon-client/src/lan/command_suggestions.rs` | unnecessary; reference; optional/future | Provides generated ordering/selection metadata for command suggestion providers keyed by command argument paths. |
| 1327 | `crates/horizon-client/src/lan/commands.rs` | unnecessary; reference; optional/future | Builds and sends the server's Brigadier command tree and command-completion data, including permission-aware command nodes and argument metadata. |
| 1328 | `crates/horizon-client/src/lan/conn.rs` | unnecessary; reference; optional/future | Implements the LAN server connection lifecycle from admission and login through configuration and play, including authentication, packet routing and disconnect handling. |
| 1329 | `crates/horizon-client/src/lan/containers.rs` | unnecessary; reference; optional/future | Implements LAN Hub guest-facing container operations: nearby sound and level/entity events, opening/closing menus, inventory synchronization, click handling and container data slots. |
| 1330 | `crates/horizon-client/src/lan/data.rs` | unnecessary; reference; optional/future | Provides LAN server registry/data synchronization and preparation of the data sent to joining guests, including registry-backed world/game data. |
| 1331 | `crates/horizon-client/src/lan/discovery.rs` | adaptable; reference; optional/future | Implements LAN server discovery by sending and receiving local-network discovery/status messages and collecting candidate servers. |
| 1332 | `crates/horizon-client/src/lan/entity.rs` | unnecessary; reference; optional/future | Implements server-side entity tracking and synchronization: spawn/removal, visibility and relationships, metadata, attributes, motion and per-player entity update packets. |
| 1333 | `crates/horizon-client/src/lan/far.rs` | adaptable; reference; version-specific | Implements server-side far-chunk delivery for distant players, maintaining a ring/cache of resident or saved columns and sending far terrain without generating or ticking those chunks. |
| 1334 | `crates/horizon-client/src/lan/forwarding.rs` | unnecessary; reference; optional/future | Implements server-side proxy forwarding identity verification, including Velocity-style signed forwarding data and HMAC validation. |
| 1335 | `crates/horizon-client/src/lan/guest.rs` | unnecessary; reference; optional/future | Stores the LAN server's per-connected-player state and operations, including inventory, movement, chat, abilities, menus, attributes, recipes and outgoing packet queues. |
| 1336 | `crates/horizon-client/src/lan/handoff.rs` | adaptable; reference; optional/future | Allows the in-process single-player server and client window to share identical chunk-section allocations after the window decodes the normal chunk packet. |
| 1337 | `crates/horizon-client/src/lan/hub.rs` | unnecessary; reference; optional/future | Implements the central LAN/dedicated server hub tick: guest lifecycle, packet handling, movement authority, chunk delivery, entity tracking, broadcasts, menus, teleports and server-side world interactions. |
| 1338 | `crates/horizon-client/src/lan/hub/chat.rs` | unnecessary; reference; optional/future | Implements server-side secure chat and command handling: profile-key validation, signed message chains, last-seen acknowledgements, visibility and relaying. |
| 1339 | `crates/horizon-client/src/lan/hub/chat_filter.rs` | unnecessary; reference; optional/future | Implements server-side chat moderation/filtering for muted users, blocked language and moderation escalation across chat, commands and text-entry channels. |
| 1340 | `crates/horizon-client/src/lan/menu.rs` | unnecessary; reference; optional/future | Implements the authoritative server-side container click engine: slot rules, carried stacks, click modes, shift-click, swaps, throws, bundles and hidden/native menu slots. |
| 1341 | `crates/horizon-client/src/lan/mod.rs` | unnecessary; reference; optional/future | Defines the LAN/dedicated server module, listener/configuration setup, login admission, online-mode identity and per-connection session lifecycle. |
| 1342 | `crates/horizon-client/src/lan/playerdata.rs` | unnecessary; reference; optional/future | Persists and restores the LAN server's player save data, including inventory, attributes, effects, abilities, recipe-book state, vehicle and world/dimension fields. |
| 1343 | `crates/horizon-client/src/lan/prediction.rs` | unnecessary; reference; optional/future | Implements server-side replay of guest movement inputs to compare claimed movement with deterministic player physics and detect/excuse deviations. |
| 1344 | `crates/horizon-client/src/lan/recipes.rs` | unnecessary; reference; optional/future | Builds server-side recipe data for synchronization, recipe-property item sets, stonecutter displays and optional Horizon native-mod recipe-content payloads. |
| 1345 | `crates/horizon-client/src/lan/scoreboard.rs` | unnecessary; reference; optional/future | Maintains a per-player view of server scoreboard objectives, display slots, scores and teams, then emits the packets needed to update that client's board. |
| 1346 | `crates/horizon-client/src/lan/tracking.rs` | unnecessary; reference; version-specific | Provides a generated entity-type table of server tracking range, update interval and delta-tracking behavior, with fallback defaults for unknown types. |
| 1347 | `crates/horizon-client/src/lan/waypoints.rs` | unnecessary; reference; optional/future | Implements server-side waypoint/locator-bar tracking between players, including block/chunk/azimuth targets, visibility, team color, range and update/untrack packets. |
| 1348 | `crates/horizon-client/src/landing.rs` | essential; rewrite required; version-specific | Implements player fall/landing behavior and block landing reactions, producing sound, particle, damage, statistic and block-change events through the motion simulation. |
| 1349 | `crates/horizon-client/src/locator_bar.rs` | unnecessary; reference; optional/future | Implements client-side waypoint state, style/animated sprite loading and locator-bar rendering with distance-based dots, arrows and contextual HUD priority. |
| 1350 | `crates/horizon-client/src/lod_net.rs` | unnecessary; reference; optional/future | Defines Horizon's custom far-terrain protocol, bounded tile requests/replies, rate limits and LOD data decoding. |

## Part 55 — Entries 1351–1375

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1351 | `CLAUDE.md` | unnecessary; reference; optional/future | Symlink to AGENTS.md (9-byte target); repository guidance metadata, not a separate source file. |
| 1352 | `crates/horizon-client/src/game/native_components.rs` | unnecessary; reference; optional/future | Reads/writes native-mod item components through Horizon's custom stack data API. |
| 1353 | `crates/horizon-client/src/game/native_criteria.rs` | unnecessary; reference; optional/future | Dispatches native-mod criteria/advancement observations from Horizon game events. |
| 1354 | `crates/horizon-client/src/game/native_data_maps.rs` | unnecessary; reference; optional/future | Exposes loaded data-map values and typed resource lookups to native mods. |
| 1355 | `crates/horizon-client/src/game/native_entities.rs` | unnecessary; reference; optional/future | Provides Horizon-native entity lookup and server-side entity operations. |
| 1356 | `crates/horizon-client/src/game/native_entity_refs.rs` | unnecessary; reference; optional/future | Maintains native-mod entity references with lifetime/world validity checks. |
| 1357 | `crates/horizon-client/src/game/native_explosions.rs` | unnecessary; reference; optional/future | Creates and applies custom authoritative-server explosions in Horizon's simulation. |
| 1358 | `crates/horizon-client/src/game/native_falling.rs` | unnecessary; reference; optional/future | Spawns bounded falling-block entities in loaded chunks on the authoritative server. |
| 1359 | `crates/horizon-client/src/game/native_hooks.rs` | unnecessary; reference; optional/future | Central 10,360-line Horizon native-mod hook API and helpers for game, world, UI, render, network and server behavior. |
| 1360 | `crates/horizon-client/src/game/native_inventories.rs` | unnecessary; reference; optional/future | Reads/moves online and offline player inventories with checked stack transfers and playerdata persistence. |
| 1361 | `crates/horizon-client/src/game/native_item_container.rs` | unnecessary; reference; optional/future | Implements custom item-stack container slots and bounded insert/extract operations. |
| 1362 | `crates/horizon-client/src/game/native_item_energy.rs` | unnecessary; reference; optional/future | Implements Horizon custom energy buffers stored in item custom data. |
| 1363 | `crates/horizon-client/src/game/native_item_fluids.rs` | unnecessary; reference; optional/future | Implements water/lava buckets and custom item fluid tanks. |
| 1364 | `crates/horizon-client/src/game/native_lifecycle.rs` | unnecessary; reference; optional/future | Tracks authoritative world epochs, open/close phases and non-paused server tick scopes. |
| 1365 | `crates/horizon-client/src/game/native_menus.rs` | unnecessary; reference; optional/future | Implements server-opened native menus, player menu lifecycle and transactional item exchanges. |
| 1366 | `crates/horizon-client/src/game/native_mob_draw.rs` | unnecessary; reference; optional/future | Builds client mob/player render snapshots and poses custom native-mod geometry. |
| 1367 | `crates/horizon-client/src/game/native_mobs.rs` | unnecessary; reference; optional/future | Exposes validated server-side mob spawn, query, movement, health and equipment operations. |
| 1368 | `crates/horizon-client/src/game/native_motion.rs` | unnecessary; reference; optional/future | Provides bounded player pushes and server-controlled creative flying speed. |
| 1369 | `crates/horizon-client/src/game/native_mounts.rs` | unnecessary; reference; optional/future | Lets native server mods allow/cancel player and mob mounting or dismounting. |
| 1370 | `crates/horizon-client/src/game/native_net.rs` | unnecessary; reference; optional/future | Queues Horizon proprietary client/server native-mod payload messages with per-tick limits. |
| 1371 | `crates/horizon-client/src/game/native_panel.rs` | unnecessary; reference; optional/future | Renders a native-mod item browser panel with paging, search and input events. |
| 1372 | `crates/horizon-client/src/game/native_panel_recipes.rs` | unnecessary; reference; optional/future | Lays out and renders bounded recipe pages, tabs, inputs and actions in the native item panel. |
| 1373 | `crates/horizon-client/src/game/native_projectiles.rs` | unnecessary; reference; optional/future | Routes projectile impacts through native hooks and replays impacts canceled by mods. |
| 1374 | `crates/horizon-client/src/game/native_quick_move.rs` | unnecessary; reference; optional/future | Routes shift-clicks in native mod containers and settles item remainders safely. |
| 1375 | `crates/horizon-client/src/game/native_recipes.rs` | unnecessary; reference; optional/future | Builds native-mod recipe catalogues, tracks recipe-book state and syncs selected recipe content. |

## Part 56 — Entries 1376–1400

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1376 | `crates/horizon-client/src/game/native_server.rs` | unnecessary; reference; optional/future | Horizon-specific native server-mod API and server-scoped operations. |
| 1377 | `crates/horizon-client/src/game/native_shaders.rs` | unnecessary; reference; optional/future | Native shader-pack lifecycle and renderer callbacks. |
| 1378 | `crates/horizon-client/src/game/native_ticks.rs` | unnecessary; reference; optional/future | Native-mod game/server/player/entity tick callback dispatch. |
| 1379 | `crates/horizon-client/src/game/native_visuals.rs` | unnecessary; reference; optional/future | Client-only native visual effects and render callbacks. |
| 1380 | `crates/horizon-client/src/game/native_world_columns.rs` | unnecessary; reference; optional/future | Bounded native-mod world-column and heightmap queries. |
| 1381 | `crates/horizon-client/src/game/native_world_edit.rs` | unnecessary; reference; optional/future | Validated server-side block/world edits for native mods. |
| 1382 | `crates/horizon-client/src/game/native_world_read.rs` | unnecessary; reference; optional/future | Read-only world/block queries through Horizon's native API. |
| 1383 | `crates/horizon-client/src/game/natural_spawning.rs` | essential; adapt; 1.20.1 | Natural mob spawning, spawn placement, category caps and spawn finalization. |
| 1384 | `crates/horizon-client/src/game/nautili.rs` | unnecessary; version mismatch; future | Nautilus mob AI and interactions; species is absent from Minecraft 1.20.1. |
| 1385 | `crates/horizon-client/src/game/net_harness.rs` | undetermined; optional; test only; no runtime | Local networking test harness; not production game runtime. |
| 1386 | `crates/horizon-client/src/game/nether_portal.rs` | essential; adapt; 1.20.1 | Portal contact, cooldown, destination search and dimension transfer. |
| 1387 | `crates/horizon-client/src/game/no_resources.rs` | essential; adapt; 1.20.1 | Fallback behavior for absent or not-yet-loaded resources. |
| 1388 | `crates/horizon-client/src/game/ominous_item_spawner.rs` | unnecessary; version mismatch; future | Ominous item spawner entity and timed item release; later-version feature. |
| 1389 | `crates/horizon-client/src/game/options.rs` | essential; adapt; 1.20.1 | In-game settings UI, option changes and configuration persistence. |
| 1390 | `crates/horizon-client/src/game/own_skin.rs` | undetermined; optional; adapt; offline profile | Local player skin loading/customization; authenticated cloud upload must be gated. |
| 1391 | `crates/horizon-client/src/game/packs.rs` | essential; adapt; 1.20.1 | Resource-pack discovery, layered server packs and asynchronous resource reload. |
| 1392 | `crates/horizon-client/src/game/pad_menu.rs` | undetermined; optional; controller feature | Controller focus/pointer navigation, menu input and on-screen keyboard. |
| 1393 | `crates/horizon-client/src/game/pad.rs` | undetermined; optional; controller feature | Controller polling, action mapping, movement/look, rumble and virtual input. |
| 1394 | `crates/horizon-client/src/game/pad/android.rs` | undetermined; optional; Android controller feature | Android controller device discovery and vibration bridge. |
| 1395 | `crates/horizon-client/src/game/pad/hardware.rs` | undetermined; optional; desktop only | gilrs controller backend and DualSense HID/haptics; exclude from Android baseline. |
| 1396 | `crates/horizon-client/src/game/paintings.rs` | essential; adapt; 1.20.1 | Painting variants, placement/survival, breaking, ticking and rendering. |
| 1397 | `crates/horizon-client/src/game/pandas.rs` | essential; adapt; 1.20.1 | Panda food pickup, bamboo interaction, breeding/age effects and particles. |
| 1398 | `crates/horizon-client/src/game/passengers.rs` | essential; adapt; 1.20.1 | Entity passenger graphs, vehicle seating, jockeys and skeleton horse traps. |
| 1399 | `crates/horizon-client/src/game/perf.rs` | undetermined; optional; diagnostics | Performance sampling, GPU memory and stats overlay. |
| 1400 | `crates/horizon-client/src/game/persist.rs` | essential; adapt; 1.20.1 | Entity/chunk NBT persistence, passenger trees, attachments and scheduled ticks. |

## Part 57 — Entries 1401–1425

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1401 | `crates/horizon-client/src/game/perspective.rs` | essential; rewrite required; 1.20.1 | Camera mode, collision zoom and player model/render pipeline. |
| 1402 | `crates/horizon-client/src/game/pick_entity.rs` | essential; rewrite required; 1.20.1 | Entity ray-pick selection and creative pick-result mapping. |
| 1403 | `crates/horizon-client/src/game/pickups.rs` | adaptable; reference; optional/future | Visual item/orb/arrow/trident pickup interpolation. |
| 1404 | `crates/horizon-client/src/game/piglins.rs` | essential; rewrite required; 1.20.1 | Piglin item preference, equipment, barter and anger. |
| 1405 | `crates/horizon-client/src/game/piston_push.rs` | essential; rewrite required; 1.20.1 | Piston entity pushes, collision clipping and sticky movement. |
| 1406 | `crates/horizon-client/src/game/player_equip.rs` | adaptable; rewrite required; 1.20.1 | Player armor/offhand equip sounds and game events. |
| 1407 | `crates/horizon-client/src/game/postprocess.rs` | essential; rewrite required; 1.20.1 | Chunk-neighborhood fluid and block postprocessing. |
| 1408 | `crates/horizon-client/src/game/potent_sulfur.rs` | unnecessary; version mismatch; future | Later-version Potent Sulfur geyser; exclude from baseline. |
| 1409 | `crates/horizon-client/src/game/potions.rs` | essential; rewrite required; 1.20.1 | Potion use, splash/lingering effects, clouds and HUD. |
| 1410 | `crates/horizon-client/src/game/powder_snow.rs` | essential; adapt; 1.20.1 | Powder-snow bucket pickup and fire extinguishing. |
| 1411 | `crates/horizon-client/src/game/prediction.rs` | undetermined; optional; reference; LAN host | LAN-host guest movement replay; not remote-client baseline. |
| 1412 | `crates/horizon-client/src/game/processing.rs` | unnecessary; reference; optional/future | Create native processing simulation; not Forge mod loading. |
| 1413 | `crates/horizon-client/src/game/pumpkin.rs` | essential; adapt; 1.20.1 | Shears carving pumpkin, loot and orientation. |
| 1414 | `crates/horizon-client/src/game/quick_actions.rs` | unnecessary; version mismatch; future | Later-version Quick Actions dialog registry/keybind flow. |
| 1415 | `crates/horizon-client/src/game/raids.rs` | essential; rewrite required; 1.20.1 | Raids, patrol spawning, waves and village siege. |
| 1416 | `crates/horizon-client/src/game/ram_limit.rs` | adaptable; reference; Android optimization | Adaptive render-distance policy under memory pressure. |
| 1417 | `crates/horizon-client/src/game/random_textures.rs` | undetermined; optional; reference; OptiFine compatibility | OptiFine random entity texture selection. |
| 1418 | `crates/horizon-client/src/game/recipe_book.rs` | essential; rewrite required; 1.20.1 | Recipe-book search/filter, placement and ghost slots. |
| 1419 | `crates/horizon-client/src/game/remote_camera.rs` | essential; adapt; 1.20.1 | Spectator camera attachment and entity post effects. |
| 1420 | `crates/horizon-client/src/game/remote_chat.rs` | essential; rewrite required; 1.20.1 | Secure chat signing and last-seen acknowledgement. |
| 1421 | `crates/horizon-client/src/game/remote_chunks.rs` | essential; rewrite required; 1.20.1 | Remote chunk decode, lighting, biome mapping and ordered insertion. |
| 1422 | `crates/horizon-client/src/game/remote_mob_data.rs` | essential; rewrite required; 1.20.1 | Remote mob/player synchronized metadata and model state. |
| 1423 | `crates/horizon-client/src/game/remote_mob_particles.rs` | undetermined; optional; adapt; entity effects | Remote mob event particles and ravager feedback. |
| 1424 | `crates/horizon-client/src/game/remote_mob_use.rs` | essential; rewrite required; 1.20.1 | Client prediction of mob interaction and held-item fallback. |
| 1425 | `crates/horizon-client/src/game/remote_objects.rs` | essential; rewrite required; 1.20.1 | Remote vehicle, armor stand, frame, painting and display lifecycle. |

## Part 58 — Entries 1426–1450

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1426 | `crates/horizon-client/src/game/remote_projectiles.rs` | essential; rewrite required; 1.20.1 | Remote projectile spawn, metadata and interpolation for multiplayer combat. |
| 1427 | `crates/horizon-client/src/game/remote_stations.rs` | adaptable; rewrite required; 1.20.1 | Workstation/station interactions and menu routing. |
| 1428 | `crates/horizon-client/src/game/remote_vehicles.rs` | essential; rewrite required; 1.20.1 | Remote vehicle/mount state, steering and passenger synchronization. |
| 1429 | `crates/horizon-client/src/game/remote.rs` | essential; rewrite required; 1.20.1 | Central multiplayer protocol dispatcher and synchronized remote game state. |
| 1430 | `crates/horizon-client/src/game/render_control.rs` | undetermined; optional; developer diagnostics only | Developer control-socket rendering commands and A/B diagnostics. |
| 1431 | `crates/horizon-client/src/game/render_tick.rs` | essential; adapt; 1.20.1 | Frame visual updates for fog, weather, rain splashes and particles. |
| 1432 | `crates/horizon-client/src/game/repro.rs` | undetermined; optional; reference; diagnostics | Reproducible session capture/replay and privacy-filtered diagnostic export. |
| 1433 | `crates/horizon-client/src/game/respawn_anchor.rs` | essential; adapt; 1.20.1 | Respawn anchor charging, explosion and safe respawn resolution. |
| 1434 | `crates/horizon-client/src/game/saves_control.rs` | undetermined; optional; developer diagnostics only | Developer control commands for saved-world management and server-list inspection. |
| 1435 | `crates/horizon-client/src/game/saving.rs` | essential; adapt; 1.20.1 | Incremental world save, durable writer completion and Android background saving. |
| 1436 | `crates/horizon-client/src/game/saws.rs` | unnecessary; reference; optional/future | Create 6.0.10 native saw simulation; not Forge mod loading. |
| 1437 | `crates/horizon-client/src/game/scoreboard.rs` | essential; rewrite required; 1.20.1 | Scoreboard objectives, scores, teams and display rendering. |
| 1438 | `crates/horizon-client/src/game/screen_info.rs` | adaptable; rewrite required; UI framework | Reusable screen titles, prompts and navigation metadata. |
| 1439 | `crates/horizon-client/src/game/screens.rs` | essential; rewrite required; 1.20.1 | Game screen/menu dispatcher and shared UI input/draw flow. |
| 1440 | `crates/horizon-client/src/game/sculk_fixture.rs` | essential; rewrite required; 1.20.1 | Sculk fixture block interactions and vibration/game-event behavior. |
| 1441 | `crates/horizon-client/src/game/sculk.rs` | essential; rewrite required; 1.20.1 | Sculk vibration, sensor, shrieker, catalyst and Warden event routing. |
| 1442 | `crates/horizon-client/src/game/server_links.rs` | unnecessary; version mismatch; future | Later-version server link prompt, validation and open/copy actions. |
| 1443 | `crates/horizon-client/src/game/server_packs.rs` | essential; rewrite required; 1.20.1 | Server resource-pack consent, download, reload and status handling. |
| 1444 | `crates/horizon-client/src/game/server_travel.rs` | undetermined; optional; reference; LAN host | LAN-host dimension/portal trips, vehicles, respawn and chunk-ready arrival. |
| 1445 | `crates/horizon-client/src/game/setup.rs` | undetermined; optional; adapt; Android launcher | Android account/resource/Java setup and onboarding UI. |
| 1446 | `crates/horizon-client/src/game/setup/account_form.rs` | undetermined; optional; adapt; Android launcher | Credential/offline account form, Android IME bridge and password handling. |
| 1447 | `crates/horizon-client/src/game/setup/fishing.rs` | undetermined; optional; future polish | Optional launcher fishing mini-game and generated sound/animation. |
| 1448 | `crates/horizon-client/src/game/shader_packs.rs` | undetermined; optional; reference; Android graphics | External shader-pack loading, profiles, uniforms and mobile fallback. |
| 1449 | `crates/horizon-client/src/game/shelf.rs` | unnecessary; version mismatch; future | Later-version wooden shelf inventory swap and item rendering. |
| 1450 | `crates/horizon-client/src/game/shield.rs` | essential; rewrite required; 1.20.1 | Shield item blocking, damage reduction, durability and disable rules. |

## Part 59 — Entries 1451–1475

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1451 | `crates/horizon-client/src/game/shoulders.rs` | adaptable; version-specific | Parrot shoulder attachment, landing/dismount transitions, shoulder NBT and ambient behavior. |
| 1452 | `crates/horizon-client/src/game/sign_edit.rs` | rewrite required; version-specific | Sign front/back editing, text input/filtering, dye/glow controls, orientation and click-event editing for local and hosted players. |
| 1453 | `crates/horizon-client/src/game/sniffers.rs` | adaptable; version-specific | Sniffer egg hatching, sniffing behavior, particles, sounds and player-facing effects. |
| 1454 | `crates/horizon-client/src/game/social.rs` | unnecessary; optional/future | Local social-hide/player-tab state and player session presentation. |
| 1455 | `crates/horizon-client/src/game/spawner.rs` | rewrite required; version-specific | Base spawner ticking/spawn attempts, spawn-egg behavior and spawner display animation. |
| 1456 | `crates/horizon-client/src/game/spear.rs` | unnecessary; version-specific | Kinetic spear attacks with speed, piercing and lunge behavior. |
| 1457 | `crates/horizon-client/src/game/spectator.rs` | adaptable; version-specific | Spectator HUD/radial selection and teleport-to-player/entity action. |
| 1458 | `crates/horizon-client/src/game/spill.rs` | rewrite required; version-specific | Block/container destruction and spilling of stored contents and block-entity data into drops. |
| 1459 | `crates/horizon-client/src/game/splash_water.rs` | adaptable; version-specific | Splash-water effects: extinguishing and water-susceptibility behavior on selected entities. |
| 1460 | `crates/horizon-client/src/game/split.rs` | unnecessary; optional/future | Coordinates multiple local game instances for desktop split-screen rendering, input, compositor and session lifecycle. |
| 1461 | `crates/horizon-client/src/game/split/frames.rs` | unnecessary; desktop-only | Split-screen frame diagnostics and viewport bookkeeping. |
| 1462 | `crates/horizon-client/src/game/split/hud.rs` | unnecessary; desktop-only | Split-screen player badges and controller prompts. |
| 1463 | `crates/horizon-client/src/game/split/pads.rs` | unnecessary; desktop-only | Split-screen controller roster, assignment and disconnect/reconnect management. |
| 1464 | `crates/horizon-client/src/game/split/players.rs` | unnecessary; desktop-only | Local split-screen guest/player profile and shared session management. |
| 1465 | `crates/horizon-client/src/game/split/setup.rs` | unnecessary; desktop-only | Split-screen setup UI and desktop monitor/layout persistence. |
| 1466 | `crates/horizon-client/src/game/split/windows.rs` | unnecessary; desktop-only | Desktop split-screen multi-window creation, monitor selection and lifecycle. |
| 1467 | `crates/horizon-client/src/game/startup_menu.rs` | adaptable; reference; rewrite required | Startup/title UI, async resource-worker loading, retry/error handling, queued world/server entry and transition to active play. |
| 1468 | `crates/horizon-client/src/game/station_control.rs` | unnecessary; reference; optional/future | Control-socket commands for menus, slots, text input, XP, trades and effects. |
| 1469 | `crates/horizon-client/src/game/stations.rs` | rewrite required; version-specific; reference | Workstation/container station state, slot actions, anvil/enchanting/smithing/grindstone/stonecutter/trading/beacon/cartography/loom UI, tooltips and brewing. |
| 1470 | `crates/horizon-client/src/game/stats_screen.rs` | adaptable; version-specific | Statistics screen with General/Items/Mobs tabs, sorting, tooltips, scrolling and server refresh request. |
| 1471 | `crates/horizon-client/src/game/stats.rs` | adaptable; version-specific | Player stats persistence/dirty events, movement/time counters and phantom spawning. |
| 1472 | `crates/horizon-client/src/game/storage.rs` | adaptable; version-specific | Shulker/ender chest inventory, spectator opening, double-chest pairing, titles/sounds and reach-based menu closure. |
| 1473 | `crates/horizon-client/src/game/structure_blocks.rs` | adaptable; version-specific | Structure block save/load/scan, template NBT serialization/placement, redstone trigger and bounding-box visualization. |
| 1474 | `crates/horizon-client/src/game/structure_edit.rs` | adaptable; version-specific | Structure-block edit screen, text fields, mode/options and SetStructureBlock request. |
| 1475 | `crates/horizon-client/src/game/structures.rs` | adaptable; reference | Finds structures reaching a block position using chunk structure refs and cached structure starts/pieces. |

## Part 60 — Entries 1476–1500

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1476 | `crates/horizon-client/src/game/subtitle_hud.rs` | adaptable; optional | Displays timed sound subtitles and directional cues as a HUD overlay. |
| 1477 | `crates/horizon-client/src/game/sulfur_cubes.rs` | unnecessary; later-version content | Implements Sulfur Cube variants, swallowed-block behavior, shearing, explosive interactions, splitting, feeding and effects. |
| 1478 | `crates/horizon-client/src/game/test_block_edit.rs` | unnecessary; later-version content | Provides an editor for GameTest TestBlock configuration and submits mode/message settings. |
| 1479 | `crates/horizon-client/src/game/test_blocks.rs` | unnecessary; later-version content | Implements server-side TestBlock activation, mode state, redstone-neighbor handling and permission-gated test operations. |
| 1480 | `crates/horizon-client/src/game/test_entities.rs` | unnecessary; later-version content | Manages entity bounds/intersection and removal for GameTest workspaces and force-loaded test chunks. |
| 1481 | `crates/horizon-client/src/game/test_environments.rs` | unnecessary; later-version content | Parses/applies GameTest batch environment settings including clock, difficulty, functions, gamerules and weather. |
| 1482 | `crates/horizon-client/src/game/test_instance_edit.rs` | unnecessary; later-version content | Edits TestInstance settings/actions including initialize, query, set, reset, save and run. |
| 1483 | `crates/horizon-client/src/game/test_instances.rs` | unnecessary; later-version content | Loads GameTest instance definitions/bounds/markers, force-loads test chunks, resets/places templates and starts tests. |
| 1484 | `crates/horizon-client/src/game/text_displays.rs` | adaptable; version-specific | Synchronizes/renders TextDisplay, BlockDisplay and ItemDisplay entities with interpolation, lighting and culling. |
| 1485 | `crates/horizon-client/src/game/titles.rs` | adaptable; version-specific | Ticks/renders title, subtitle, action-bar, selected-item-name and save-indicator HUD elements. |
| 1486 | `crates/horizon-client/src/game/tnt.rs` | rewrite required; version-specific | Simulates/renders primed TNT entities, fuse timing, motion, fluid drag, collision, ignition and explosion. |
| 1487 | `crates/horizon-client/src/game/totem.rs` | rewrite required; version-specific | Handles Totem of Undying activation during lethal damage, effects, item consumption and client presentation. |
| 1488 | `crates/horizon-client/src/game/touch.rs` | adaptable; rewrite required; Android-specific | Implements on-screen touch controls, layout editing, custom buttons, virtual-pad actions, multitouch gestures, pointer aiming, Android keyboard coordination and chat controls. |
| 1489 | `crates/horizon-client/src/game/touch/slots.rs` | adaptable; version-specific | Implements touch-first inventory/menu slot actions: pick stacks, half-stack hold, double-tap quick-move, drag distribution, outside drop and player rotation. |
| 1490 | `crates/horizon-client/src/game/touch/text_mirror.rs` | adaptable; Android-specific; rewrite required | Mirrors game text fields into Android IME and synchronizes text, cursor, selection, action keys and keyboard visibility. |
| 1491 | `crates/horizon-client/src/game/trading.rs` | rewrite required; version-specific | Implements villager/wandering-trader interactions, offer generation/session state, trading UI, rewards, maps and persistence. |
| 1492 | `crates/horizon-client/src/game/transient_blocks.rs` | unnecessary; reference; later-version/custom protocol | Temporarily renders server-announced blocks until replacement section mesh is compiled/uploaded. |
| 1493 | `crates/horizon-client/src/game/trial_spawner.rs` | unnecessary; later-version content | Runs Trial Spawner simulation, player detection, spawning, reward ejection, Trial Omen effects and block transitions. |
| 1494 | `crates/horizon-client/src/game/trial_visuals.rs` | unnecessary; later-version content | Renders Trial Spawner/Vault events, particles, ambient sounds, spin animation and nearby-player connection effects. |
| 1495 | `crates/horizon-client/src/game/trident.rs` | rewrite required; version-specific | Implements trident use/throw, Riptide, Loyalty return, projectile collision/damage, Impaling/Channeling, pickup, NBT and rendering. |
| 1496 | `crates/horizon-client/src/game/tutorial.rs` | adaptable; optional | Tracks first-world tutorial steps, movement/look progress, tree/punch/inventory/crafting hints and toasts. |
| 1497 | `crates/horizon-client/src/game/tweaks.rs` | unnecessary; optional/future | Adds optional inventory mouse tweaks, stack drag/wheel operations, slot sorting and HDR shortcut. |
| 1498 | `crates/horizon-client/src/game/ui_control.rs` | unnecessary; developer-only | Exposes UI widget tree and agent commands to inspect/click/set widgets via synthetic input. |
| 1499 | `crates/horizon-client/src/game/update_offer.rs` | unnecessary; optional/future; security review required | Checks GitHub releases for a newer Android APK, presents prompt, downloads/verifies and launches Android installer. |
| 1500 | `crates/horizon-client/src/game/vault.rs` | unnecessary; later-version content | Runs Vault block-entity state, player/key interactions, reward ejection, state changes and display-item rendering. |

## Part 61 — Entries 1501–1525

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1501 | `crates/horizon-client/src/game/villagers.rs` | adaptable; version-specific | Villager routines: food sharing/composting, village schedules, gossip, reputation, meetings, sleeping, breeding and golem summoning. |
| 1502 | `crates/horizon-client/src/game/wandering_trader.rs` | adaptable; version-specific | World-saved wandering-trader spawn timers/chance, player/bell search, biome/space checks, trader and two leashed llamas, despawn. |
| 1503 | `crates/horizon-client/src/game/win.rs` | adaptable; optional; version-specific | End poem/credits screen, portal-fill and timed scroll, keyboard navigation, completion and exit behavior. |
| 1504 | `crates/horizon-client/src/game/wind.rs` | unnecessary; later-version content | Wind Charge/Breeze projectiles and bursts, knockback, particles, sounds and affected block interactions. |
| 1505 | `crates/horizon-client/src/game/wither.rs` | adaptable; version-specific | Wither summoning, skull hits/blue-skull block destruction, Wither Rose, effects, target damage and multipart interactions. |
| 1506 | `crates/horizon-client/src/game/world_content.rs` | undetermined; reference; rewrite required; not Forge | Horizon per-world native content manifests and restart decisions when required local content differs. |
| 1507 | `crates/horizon-client/src/game/world_loading.rs` | adaptable; rewrite required; version-specific | Local-world creation/loading, level. |
| 1508 | `crates/horizon-client/src/game/zombie_reinforcements.rs` | adaptable; version-specific | Zombie reinforcement calls: difficulty/attribute gates, sampled offsets and spawn validation. |
| 1509 | `crates/horizon-client/src/game/zombie_villagers.rs` | adaptable; version-specific | Zombie-villager conversion and cure: weakness/golden apple, timer and restoration of profession/trades/XP/gossip/equipment. |
| 1510 | `crates/horizon-client/src/lod_net/client.rs` | unnecessary; reference; custom protocol | Client half of Horizon custom LOD channel: distant tile requests, throttling, in-flight tracking and timeout/epochs. |
| 1511 | `crates/horizon-client/src/lod_net/server.rs` | unnecessary; reference; custom protocol | Server half of Horizon custom LOD: per-client quotas, tokens/strikes, worker queue, sampling and response delivery. |
| 1512 | `crates/horizon-client/src/menu.rs` | rewrite required; version-specific; core reference | Local menu/slot engine: click types, slot rules, quick-move, swap, clone, throw, drag, crafting results/remainders, containers and station menu models. |
| 1513 | `crates/horizon-client/src/menu/bundle.rs` | unnecessary; later-version content | Bundle item insert/extract operations, click rules and sounds. |
| 1514 | `crates/horizon-client/src/menu/place.rs` | adaptable; version-specific | Recipe ingredient placement into crafting/furnace grids, ingredient matching, inventory extraction, clearing and recipe-book ghosts. |
| 1515 | `crates/horizon-client/src/merchant.rs` | rewrite required; version-specific | Merchant offers/session and trade generation, price/demand/reputation/discount logic, XP and trade events. |
| 1516 | `crates/horizon-client/src/mobs.rs` | rewrite required; version-specific; core reference | Shared Mob data model, species/type metadata, attributes, damage/events, synchronized extra state, lifecycle/tick helpers and model/render structures. |
| 1517 | `crates/horizon-client/src/mobs/ai.rs` | rewrite required; version-specific; core reference | Mob AI/tick orchestration: goals/brains, movement, collision/pushing, despawn, age, effects, attacks, sounds and species actions. |
| 1518 | `crates/horizon-client/src/mobs/allay.rs` | adaptable; version-specific | Allay item fetching/delivery, liked-player/note-block memory, vibration listening, dancing and animation behavior. |
| 1519 | `crates/horizon-client/src/mobs/ambient.rs` | adaptable; client-only; optional | Client-only mob ambient visuals/sounds: portal particles, Blaze smoke/burn, Phantom wing dust and Bee nectar particles. |
| 1520 | `crates/horizon-client/src/mobs/anim.rs` | adaptable; rewrite required; client rendering | Pose maps/pools, interpolated render state, humanoid/item arm poses, swim/walk cycles and per-species model animation. |
| 1521 | `crates/horizon-client/src/mobs/animal_brain.rs` | rewrite required; version-specific; core reference | Memory/sensor animal brains and behavior scheduling for Sniffer, Axolotl and later brain-driven mobs; breeding, temptation, panic, navigation and state changes. |
| 1522 | `crates/horizon-client/src/mobs/aquatic.rs` | adaptable; version-specific | Aquatic mob registration, squid/glow-squid movement and ink, fish schooling, pufferfish puff/sting, dolphin breathing/swimming and animations. |
| 1523 | `crates/horizon-client/src/mobs/armadillo.rs` | unnecessary; later-version content | Armadillo fear/shell state machine, threat sensing, rolling/unrolling/peek, damage reduction, breeding and baby-follow goals. |
| 1524 | `crates/horizon-client/src/mobs/axolotl.rs` | adaptable; version-specific | Axolotl prey eligibility/hunting cooldown, play-dead damage roll, support-player event and baby/adult animation states. |
| 1525 | `crates/horizon-client/src/mobs/bat.rs` | adaptable; version-specific | Bat resting/hanging, wake-up, target-air selection and flying movement, takeoff effect and animations. |

## Part 62 — Entries 1526–1550

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1526 | `crates/horizon-client/src/mobs/bee.rs` | adaptable; version-specific | Implements Bee goals and state: flower seeking, nectar collection, hive entry/exit, breeding/temptation, anger persistence, stinging/poison, crop pollination and hive/home behavior. |
| 1527 | `crates/horizon-client/src/mobs/body_anim.rs` | adaptable; client-rendering; version-specific | Contains species-specific body/model animation setup for many mobs, including movement cycles, head/body/limb poses, baby variants, attacks, sitting, swimming and special states. |
| 1528 | `crates/horizon-client/src/mobs/brain.rs` | adaptable; core reference; version-specific | Provides shared vanilla Brain primitives: expiring memory TTL, timed behavior bookkeeping, nearby-living-entity sensing, walk targets, and MoveToTargetSink navigation. |
| 1529 | `crates/horizon-client/src/mobs/breeze.rs` | unnecessary; later-version content | Implements Breeze AI, wind charge attacks, jumping/projectile behavior, knockback and associated movement/effects. |
| 1530 | `crates/horizon-client/src/mobs/camel.rs` | adaptable; version-specific | Implements Camel behavior including sitting/standing timers, dash/jump, rider movement, passenger positioning, goals, breeding/temptation and animation. |
| 1531 | `crates/horizon-client/src/mobs/caravan.rs` | adaptable; version-specific | Implements llama caravan membership and follow behavior: joining/leaving a leader, follower chain selection, distance checks and movement toward preceding llamas. |
| 1532 | `crates/horizon-client/src/mobs/cem_anim.rs` | undetermined; reference; optional/future; rewrite required | Parses and evaluates custom entity model animation expressions and channels for CEM/OptiFine-compatible resource-pack animations. |
| 1533 | `crates/horizon-client/src/mobs/cem.rs` | undetermined; reference; optional/future; rewrite required | Loads custom entity models and textures from CEM/OptiFine-style resource packs and resolves model replacements, layers, attachments and fallback behavior. |
| 1534 | `crates/horizon-client/src/mobs/combat.rs` | adaptable; version-specific | Tracks recent mob combat damage history and supplies combat-state/death context, including the retention period used for hurt-by bookkeeping. |
| 1535 | `crates/horizon-client/src/mobs/control.rs` | adaptable; core reference; version-specific | Implements mob MoveControl, LookControl, JumpControl and body-rotation controllers for ground, flying, swimming and special movement states. |
| 1536 | `crates/horizon-client/src/mobs/conversion.rs` | adaptable; version-specific | Implements entity conversion between mob types while preserving selected state, equipment, effects, position, motion, riding and identity-linked data. |
| 1537 | `crates/horizon-client/src/mobs/copper_golem.rs` | unnecessary; later-version content | Implements Copper Golem creation, oxidation/weather state, statue stages, button interaction and copper-golem AI/animation. |
| 1538 | `crates/horizon-client/src/mobs/creaking_heart.rs` | unnecessary; later-version content | Implements Creaking Heart block/entity behavior: linked Creaking spawning, day/night activation, resin-related changes and heart-bound entity lifecycle. |
| 1539 | `crates/horizon-client/src/mobs/creaking.rs` | unnecessary; later-version content | Implements Creaking mob behavior, freeze-when-looked-at, heart tether/activation, attack reactions and special movement/render state. |
| 1540 | `crates/horizon-client/src/mobs/cube.rs` | adaptable; version-specific | Implements Slime and Magma Cube sizing, movement, jumping, splitting, contact damage/knockback, target behavior and animations. |
| 1541 | `crates/horizon-client/src/mobs/data.rs` | rewrite required; data-version-specific | Loads extracted mob runtime data for block pathfinding/placement, heightmaps, entity/biome tags, villager equipment/hats, CEM and texture/animation metadata. |
| 1542 | `crates/horizon-client/src/mobs/dolphin.rs` | adaptable; version-specific | Implements Dolphin treasure finding, swimming-player gifts/Dolphin's Grace, jumping, item seeking/pickup/play and aquatic goal behavior. |
| 1543 | `crates/horizon-client/src/mobs/doors.rs` | adaptable; version-specific | Implements mob door navigation/interactions, zombie/vindicator door breaking and raid illager door opening, plus Piglin path door management. |
| 1544 | `crates/horizon-client/src/mobs/dragon_contacts.rs` | undetermined; reference; version-specific; verification required | Handles Ender Dragon wing/head collision contacts with ordinary mobs, including pushing, damage and team-protection exceptions. |
| 1545 | `crates/horizon-client/src/mobs/drops.rs` | rewrite required; version-specific | Builds mob death drops: charged-creeper heads, loot-table rolls, equipment/drop chances, species extras, carried blocks, inventories and experience orbs. |
| 1546 | `crates/horizon-client/src/mobs/enderman.rs` | adaptable; version-specific | Implements Enderman stare/anger detection, teleport behavior, projectile dodge, water/rain response, block pickup/placement and held-block rendering/death drops. |
| 1547 | `crates/horizon-client/src/mobs/equine.rs` | adaptable; version-specific | Implements AbstractHorse behavior for horses, donkeys, mules and undead horses, including random stats/coats, taming/rearing/eating, breeding inheritance, rider movement/jump, sounds, textures and model animation; also contains Camel/Llama integration. |
| 1548 | `crates/horizon-client/src/mobs/equip.rs` | rewrite required; version-specific | Implements mob equipment pickup gates and replacement comparisons for armor/weapons, including attribute-modifier calculation by slot group. |
| 1549 | `crates/horizon-client/src/mobs/farm.rs` | adaptable; version-specific | Registers pig/cow/mooshroom/sheep/chicken goals, chicken egg laying and wing/fall animation, and climate-dependent sheep color selection. |
| 1550 | `crates/horizon-client/src/mobs/finalize.rs` | rewrite required; core reference; version-specific | Implements Mob. |

## Part 63 — Entries 1551–1575

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1551 | `crates/horizon-client/src/mobs/fox.rs` | adaptable; version-specific | Fox state, trust UUIDs, defensive behavior, stalking/pouncing, item pickup, berry and chorus-fruit behavior, poses and rendering. |
| 1552 | `crates/horizon-client/src/mobs/frog.rs` | adaptable; version-specific | Frog amphibious AI, tongue prey capture, eating, frogspawn placement, long jumps, croaking and pose state. |
| 1553 | `crates/horizon-client/src/mobs/gear.rs` | rewrite required; version-specific | Gear-side calculations for equipment slots, enchantments, trims, weapon damage and armor protection. |
| 1554 | `crates/horizon-client/src/mobs/goals.rs` | rewrite required; version-specific | Goal/target selectors, priority and control locks, melee, panic, avoidance, temptation, breeding, strolling, water seeking and block interaction. |
| 1555 | `crates/horizon-client/src/mobs/goat.rs` | adaptable; version-specific | Goat and screaming-goat ram attack, target selection, head lowering, charge, horn drops and long-jump behavior. |
| 1556 | `crates/horizon-client/src/mobs/golem.rs` | adaptable; version-specific | Iron-golem village defense and gifting, attack/crack state, plus snow-golem snowball, melting and snow-trail behavior. |
| 1557 | `crates/horizon-client/src/mobs/guardian.rs` | adaptable; version-specific | Guardian/Elder Guardian laser attack, swimming, spikes/thorns, land flop, beam animation and Mining Fatigue effect. |
| 1558 | `crates/horizon-client/src/mobs/happy_ghast.rs` | unnecessary; post-1.20.1 | Happy Ghast/Ghastling lifecycle, harness equipment, controllable flight/riding, healing and multi-lead handling. |
| 1559 | `crates/horizon-client/src/mobs/hostile.rs` | adaptable; version-specific | Creeper fuse/explosion and charged state, Spider wall/daylight behavior, Silverfish wake/infested-block behavior and Endermite handling. |
| 1560 | `crates/horizon-client/src/mobs/illager.rs` | rewrite required; version-specific | Pillager/Vindicator/Evoker/Illusioner shared behavior, crossbow charge/fire lifecycle, Evoker spells and illager targeting/poses. |
| 1561 | `crates/horizon-client/src/mobs/illusioner.rs` | adaptable; version-specific | Illusioner bow/spell behavior, invisibility and blindness, four mirror images, target selection and render offsets. |
| 1562 | `crates/horizon-client/src/mobs/item_break.rs` | rewrite required; version-specific | Client equipment-break events, slot identifiers, equipment lookup and broken-item/scute particle handling. |
| 1563 | `crates/horizon-client/src/mobs/keyframes.rs` | undetermined; reference; optional client feature | Generic keyframe animation parser/interpolator for position, rotation and scale channels, including linear and Catmull-Rom sampling. |
| 1564 | `crates/horizon-client/src/mobs/landing.rs` | adaptable; version-specific | Shared mob landing/block-fall integration for fall damage, sound, particles and block-specific effects. |
| 1565 | `crates/horizon-client/src/mobs/leash.rs` | rewrite required; version-specific | Leash attachment/persistence for players, mobs, knots and vehicles, connection geometry, offsets and leash movement forces. |
| 1566 | `crates/horizon-client/src/mobs/living.rs` | rewrite required; core reference; version-specific | Living mob movement/physics: fluid movement, collision, dimensions/poses, gravity, fall damage, jumping, bouncing and walk animation. |
| 1567 | `crates/horizon-client/src/mobs/llama.rs` | adaptable; version-specific | Llama/Trader Llama coat, strength, chest capacity, taming temper, caravan, spit attack, breeding and despawn state. |
| 1568 | `crates/horizon-client/src/mobs/monster_brain.rs` | unnecessary; post-1.20.1; architecture reference only | Brain memory/sensor/activity/behavior implementation for Breeze and Creaking. |
| 1569 | `crates/horizon-client/src/mobs/nautilus.rs` | unnecessary; post-1.20.1 | Nautilus/Zombie Nautilus taming, saddle/armor, rider movement and dash, swimming, pufferfish charge, bubbles and breath effect. |
| 1570 | `crates/horizon-client/src/mobs/nav.rs` | rewrite required; core reference; version-specific | PathNavigation variants, path creation/recompute, waypoint following, corner cutting, stuck detection and timeouts. |
| 1571 | `crates/horizon-client/src/mobs/nether.rs` | adaptable; version-specific | Blaze and Ghast AI, movement, ranged fireball attacks, target selection, audio events and model animation. |
| 1572 | `crates/horizon-client/src/mobs/panda.rs` | adaptable; version-specific | Panda gene inheritance, eating, sitting, rolling, sneezing, fear, breeding and model/item animation. |
| 1573 | `crates/horizon-client/src/mobs/parrot.rs` | adaptable; version-specific | Parrot flying navigation, tree-seeking wander, flapping/gliding, jukebox dance and mob-imitating sounds. |
| 1574 | `crates/horizon-client/src/mobs/path.rs` | rewrite required; core reference; version-specific | A* PathFinder, node/heap/path structures and Walk/Swim/Fly/Amphibious node evaluators. |
| 1575 | `crates/horizon-client/src/mobs/phantom.rs` | adaptable; version-specific | Phantom circling/swooping, player targeting, cat avoidance, movement control, size scaling and flap animation. |

## Part 64 — Entries 1576–1600

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1576 | `crates/horizon-client/src/mobs/piglin.rs` | rewrite required; version-specific | Piglin, Piglin Brute, Hoglin and Zoglin behavior: anger, hunting, bartering, equipment, riding, conversions and family interactions. |
| 1577 | `crates/horizon-client/src/mobs/polar_bear.rs` | adaptable; version-specific | Polar Bear neutral-hostile transition, cub protection, warning posture and melee attack behavior. |
| 1578 | `crates/horizon-client/src/mobs/rabbit.rs` | adaptable; version-specific | Rabbit variants and movement, hopping, temptation/avoidance, crop eating and Killer Bunny combat. |
| 1579 | `crates/horizon-client/src/mobs/raider.rs` | adaptable; version-specific | Illager raider patrol state, raid leader/banner handling, village movement and shared raid behavior. |
| 1580 | `crates/horizon-client/src/mobs/random_textures.rs` | undetermined; reference; optional client feature | OptiFine/ETF-style random entity texture rule loading and variant-based texture selection. |
| 1581 | `crates/horizon-client/src/mobs/ravager.rs` | adaptable; version-specific | Ravager charge and melee attacks, stun/roar state, knockback, damage reactions and foliage/block interaction. |
| 1582 | `crates/horizon-client/src/mobs/render.rs` | rewrite required; optional client rendering; version-specific | Shared mob model rendering, pose assembly, geometry/vertex generation, model layers, textures and CPU/GPU rendering paths. |
| 1583 | `crates/horizon-client/src/mobs/riders.rs` | rewrite required; version-specific | Entity riding relationships, passenger seat offsets, jockey/trap mounting, dismount and rider-driven movement. |
| 1584 | `crates/horizon-client/src/mobs/seen_grid.rs` | undetermined; reference; performance candidate; parity required | Spatial broad-phase grid for grouping entity bounding boxes and querying nearby collision candidates. |
| 1585 | `crates/horizon-client/src/mobs/shulker.rs` | adaptable; version-specific | Shulker attachment/facing, shell opening, entity pushing, bullet homing/attacks and teleport behavior. |
| 1586 | `crates/horizon-client/src/mobs/skin.rs` | undetermined; reference; optional client feature; performance candidate | GPU skinning/cache path for mob geometry and pose-specific draw jobs, with CPU fallback. |
| 1587 | `crates/horizon-client/src/mobs/sniffer.rs` | adaptable; version-specific | Sniffer sniffing, digging, seed-drop state, remembered excavation positions and model/animation state. |
| 1588 | `crates/horizon-client/src/mobs/spawn_rules.rs` | rewrite required; version-specific; server-side reference | Mob spawn placement and validity rules using light, biome, block/fluid state, slime chunks and spawn reasons. |
| 1589 | `crates/horizon-client/src/mobs/spawn_selection.rs` | rewrite required; version-specific; server-side reference | Natural mob spawn selection, category caps, weighted pack placement, player distance and structure/dimension modifiers. |
| 1590 | `crates/horizon-client/src/mobs/spawner.rs` | adaptable; version-specific; server-side reference | Vanilla Monster Spawner block-entity state, spawn potentials, delay/near-player checks and client visual feedback. |
| 1591 | `crates/horizon-client/src/mobs/spawning.rs` | rewrite required; version-specific; server-side reference | Local difficulty and mob finalizeSpawn initialization: variants, equipment, attributes, spawn reason and state. |
| 1592 | `crates/horizon-client/src/mobs/spear_use.rs` | unnecessary; post-1.20.1 | Spear/Kinetic Weapon use, stab/charge behavior, knockback/dismount and spear-related hostile mob tactics. |
| 1593 | `crates/horizon-client/src/mobs/species.rs` | rewrite required; core registry reference; version-specific | Central mob species metadata: entity classification, dimensions, movement flags, foods/tags, textures, sounds, model and animation identifiers. |
| 1594 | `crates/horizon-client/src/mobs/strider.rs` | adaptable; version-specific | Strider movement and AI on lava, warmth/cold speed changes, floating and rider behavior. |
| 1595 | `crates/horizon-client/src/mobs/sulfur_cube.rs` | unnecessary; post-1.20.1 | Sulfur Cube species, archetypes, swallowed-block/item state, bounce/friction variants, fuse and explosion behavior. |
| 1596 | `crates/horizon-client/src/mobs/sync.rs` | rewrite required; version-specific | Mob synchronized pose flags, entity-event routing and client animation/state update dispatch. |
| 1597 | `crates/horizon-client/src/mobs/tame.rs` | rewrite required; version-specific | Tameable Wolf/Cat/Ocelot/Parrot behavior: ownership, sitting, following, defense, anger, begging, relaxation and animations. |
| 1598 | `crates/horizon-client/src/mobs/trader.rs` | adaptable; version-specific | Wandering Trader routine: invisibility potion/milk, hostile avoidance, trading pause, visit movement, home restriction and despawn delay. |
| 1599 | `crates/horizon-client/src/mobs/transport_items.rs` | unnecessary; post-1.20.1 | Copper Golem chest-to-chest item transport: chest search, queue/open interaction, pickup/place and visited/unreachable memory. |
| 1600 | `crates/horizon-client/src/mobs/trial_spawner.rs` | unnecessary; post-1.20.1 | Trial Spawner configuration/state machine: player detection, waves, ominous conversion, reward ejection, item spawners and cooldown. |

## Part 65 — Entries 1601–1625

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1601 | `crates/horizon-client/src/mobs/turtle.rs` | adaptable; version-specific | Turtle swimming/travel goals, egg laying and home-beach/breeding behavior. |
| 1602 | `crates/horizon-client/src/mobs/undead.rs` | rewrite required; version-specific | Shared zombie, drowned, skeleton and other undead combat/conversion/equipment behavior; later Spear/Parched paths excluded. |
| 1603 | `crates/horizon-client/src/mobs/vex.rs` | adaptable; version-specific | Vex phasing flight, Evoker target copying, charge attack and limited lifetime. |
| 1604 | `crates/horizon-client/src/mobs/villager.rs` | rewrite required; core reference; version-specific | Villager state, profession/work/sleep/POI, inventory/food, gossip and trade-related behavior. |
| 1605 | `crates/horizon-client/src/mobs/villager_brain.rs` | rewrite required; core reference; version-specific | Villager Brain: memories, sensors, POI acquisition, activity scheduling, trade, farm, breed and raid behavior. |
| 1606 | `crates/horizon-client/src/mobs/villager_farming_tests.rs` | undetermined; reference; tests only; version-specific | Regression-test checklist for villager farming and modded crop/seed behavior; not executed. |
| 1607 | `crates/horizon-client/src/mobs/villager_hats.rs` | undetermined; reference; optional client rendering; version-specific | Optional villager/zombie-villager headwear texture metadata and model-layer visibility. |
| 1608 | `crates/horizon-client/src/mobs/warden.rs` | adaptable; version-specific | Warden anger/vibration response, melee/sonic boom, Darkness and animation state. |
| 1609 | `crates/horizon-client/src/mobs/warden_brain.rs` | rewrite required; core reference; version-specific | Warden Brain memories, sensors, investigation, sniff/roar/dig and combat activity scheduling. |
| 1610 | `crates/horizon-client/src/mobs/witch.rs` | adaptable; version-specific | Witch potion selection/drinking/throwing and raider-support behavior. |
| 1611 | `crates/horizon-client/src/mobs/wither.rs` | adaptable; version-specific; boss simulation | Wither spawn charge, multi-head targeting/attack, skull projectiles, healing and block damage. |
| 1612 | `crates/horizon-client/src/mobs/world.rs` | rewrite required; core reference; version-specific | Mob AI world queries: collision/ray clip, environment sampling and nearby entity lookup. |
| 1613 | `crates/horizon-client/src/mobs/zombie_villager.rs` | adaptable; version-specific | Zombie Villager cure timer, Weakness/Strength and cure acceleration near tagged blocks. |
| 1614 | `crates/horizon-client/src/mod_api.rs` | undetermined; reference; not Forge; architecture only | Starling native mod API/lifecycle/Host contracts; not Forge Mod Loader or FML. |
| 1615 | `crates/horizon-client/src/mod_api/generated.rs` | undetermined; reference; not Forge; generated native API | Generated Starling native-mod hook declarations and dispatch tables; not Forge events or API. |
| 1616 | `crates/horizon-client/src/mod_api/generated_host.rs` | undetermined; reference; not Forge; architecture only | Generated Starling native Host implementation over its own entity/world/item/menu types. |
| 1617 | `crates/horizon-client/src/mod_content.rs` | rewrite required; partial Forge data overlay; version-specific | Starling native content/resource overlay and selected registry/tag/worldgen/structure additions. |
| 1618 | `crates/horizon-client/src/mod_content_biomes.rs` | undetermined; reference; partial Forge/NeoForge format support; version-specific | Partial parser for Forge/NeoForge add_features/remove_features biome modifiers; custom/spawn/carver serializers not covered. |
| 1619 | `crates/horizon-client/src/mod_content_tests.rs` | undetermined; reference; tests only; version-specific | Regression tests for overlay, registries/tags/worldgen/voxel collision and content declarations; not executed. |
| 1620 | `crates/horizon-client/src/motion.rs` | rewrite required; core reference; version-specific | Player input, movement, collision, jump, fall, fluids, flight/Elytra, sprint, effects and client/server sync. |
| 1621 | `crates/horizon-client/src/motion/environment.rs` | rewrite required; version-specific | Player environmental damage and movement in liquids: fire, air/drowning, lava, bubble columns and liquid jumps. |
| 1622 | `crates/horizon-client/src/motion/fluids.rs` | rewrite required; core reference; version-specific | Player fluid immersion/flow, water splash, bubble columns, fluid shape cache and passenger interaction. |
| 1623 | `crates/horizon-client/src/multiplayer.rs` | rewrite required; client UI; protocol-specific | Multiplayer list/edit/delete, ping/status/favicon, direct connect, LAN discovery, friend invite and reconnect UI. |
| 1624 | `crates/horizon-client/src/music.rs` | adaptable; version-specific; optional client feature | Situational music selection, delay/frequency, fades, biome/dimension volume and music-layer UI. |
| 1625 | `crates/horizon-client/src/native_client_hooks.rs` | undetermined; reference; not Forge; architecture only | One-time native client hook table for menu/living/ingredient callbacks; not Forge event bus. |

## Part 66 — Entries 1626–1650

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1626 | `ROADMAP.md` | undetermined; reference; project planning only | Project roadmap and staged compatibility/feature plan. |
| 1627 | `SYNC.md` | undetermined; reference; workflow notes only | Repository synchronization and contributor handoff notes. |
| 1628 | `crates/horizon-client/src/native_data.rs` | undetermined; reference; native Rust extension data only; not Forge | Compile-time/runtime lookup of data files shipped by Starling native mods. |
| 1629 | `crates/horizon-client/src/native_tombstone.rs` | adaptable; Android diagnostics | Bounded, privacy-conscious parser for Android protobuf tombstones. |
| 1630 | `crates/horizon-client/src/nether_portal.rs` | adaptable; version-specific | Nether portal entry/cooldown state and entity transition timing. |
| 1631 | `crates/horizon-client/src/no_resources.rs` | undetermined; reference; launcher startup UX | Fallback screen when executable starts without launcher-prepared Minecraft files. |
| 1632 | `crates/horizon-client/src/old_world.rs` | undetermined; reference; world compatibility | Compatibility UI/state for old or customized worlds. |
| 1633 | `crates/horizon-client/src/patrol.rs` | adaptable; version-specific; server-side | Pillager patrol spawning, leader selection and movement/target behavior. |
| 1634 | `crates/horizon-client/src/perf_overlay.rs` | undetermined; reference; optional diagnostics | On-screen performance overlay for frame/CPU/GPU samples. |
| 1635 | `crates/horizon-client/src/performance.rs` | undetermined; reference; performance diagnostics | Performance telemetry and sample aggregation for diagnostic overlay. |
| 1636 | `crates/horizon-client/src/pick_item.rs` | rewrite required; version-specific | Pick-block/item behavior for client interaction. |
| 1637 | `crates/horizon-client/src/platform.rs` | rewrite required; platform-specific | Platform abstraction for OS-specific client services. |
| 1638 | `crates/horizon-client/src/portal_sections.rs` | adaptable; version-specific | Portal-related chunk/section lookup and bookkeeping. |
| 1639 | `crates/horizon-client/src/pose.rs` | rewrite required; version-specific | Entity pose/dimension state for rendering and interaction bounds. |
| 1640 | `crates/horizon-client/src/pregen.rs` | undetermined; reference; optional offline utility | Chunk/world pre-generation scheduling and progress state. |
| 1641 | `crates/horizon-client/src/preparer.rs` | rewrite required; launcher integration | Launcher preparation pipeline for game files. |
| 1642 | `crates/horizon-client/src/preparer_install.rs` | undetermined; reference; install UI | Install/preparation progress UI and state transitions. |
| 1643 | `crates/horizon-client/src/projectiles.rs` | rewrite required; core reference; version-specific | Projectile simulation, collision, motion and entity-specific behavior. |
| 1644 | `crates/horizon-client/src/raid.rs` | rewrite required; version-specific; server-authoritative | Raid lifecycle, wave composition, omen trigger, victory/loss and rewards. |
| 1645 | `crates/horizon-client/src/ram_limit.rs` | undetermined; reference; mobile memory governor | Memory-pressure governor reducing expensive far-terrain/render work. |
| 1646 | `crates/horizon-client/src/random_tick_sections.rs` | adaptable; performance reference; version-specific | Section-level cache/index for random-tick candidate selection. |
| 1647 | `crates/horizon-client/src/recipe_book.rs` | rewrite required; version-specific | Recipe-book state, filtering, grouping and recipe unlock model. |
| 1648 | `crates/horizon-client/src/recipe_book_screen.rs` | undetermined; reference; optional client UI | Recipe-book screen, search, overlay recipes, ghost slots and tooltips. |
| 1649 | `crates/horizon-client/src/save.rs` | rewrite required; core persistence/performance reference; version-specific | World/chunk/entity/POI persistence, region I/O, player/world save data, async writing and saved-chunk LOD. |
| 1650 | `crates/horizon-client/src/save/gateway.rs` | undetermined; reference; optional legacy migration | One-time migration of legacy Horizon End Gateway exits into chunk block-entity NBT. |

## Part 67 — Entries 1651–1675

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1651 | `crates/horizon-client/src/scoreboard_cmd.rs` | rewrite required; version-specific; optional client command layer | Scoreboard command parsing, argument completion and command-side objective, player-score, team and display-slot operations. |
| 1652 | `crates/horizon-client/src/scoreboard_cmd_tests.rs` | undetermined; reference; tests only | Unit tests for scoreboard command parsing, argument validation and scoreboard command behavior. |
| 1653 | `crates/horizon-client/src/screen_layout.rs` | undetermined; reference; optional client UI | Reusable screen geometry and layout calculations for common UI regions, rows, columns and control placement. |
| 1654 | `crates/horizon-client/src/screen_narrator.rs` | undetermined; reference; optional accessibility UI | Screen narration/accessibility text collection, ordering, focus and announcement behavior for menus, lists, buttons and edit fields. |
| 1655 | `crates/horizon-client/src/server_packs/download.rs` | rewrite required; version-specific; security-sensitive | Server resource-pack download queue, HTTP headers, SHA-1 verification, cache reuse, progress events, JSON download log and cache cleanup. |
| 1656 | `crates/horizon-client/src/shadows.rs` | adaptable; version-specific; rendering | Entity and display shadow radius/strength rules, camera-distance fading, block-light sampling and collision-shape-clipped shadow quads. |
| 1657 | `crates/horizon-client/src/social_screen.rs` | adaptable; version-specific; optional client UI | Social Interactions screen with All/Hidden/Blocked tabs, search, player skins, hide/show chat actions, offline status and disabled report affordance. |
| 1658 | `crates/horizon-client/src/spear.rs` | unnecessary; post-1.20.1 | Kinetic spear charge/dismount/knockback/damage conditions, attack reach and first-/third-person stab animation transforms. |
| 1659 | `crates/horizon-client/src/spectator_menu.rs` | adaptable; version-specific; client UI | Spectator hotbar/menu pages for teleport-to-player and teleport-to-team, scrolling, selection, close action, fade timing and icons. |
| 1660 | `crates/horizon-client/src/sprint.rs` | adaptable; version-specific; mobile input integration | Local sprint start/stop conditions, double-tap forward window, sprint-key handling, swimming/flying restrictions and analog-stick direction mapping. |
| 1661 | `crates/horizon-client/src/starling_releases.rs` | unnecessary; Starling-specific launcher/release integration | Starling APK release discovery, ABI selection, update asset/digest validation, native-mod release parsing and build-server address validation. |
| 1662 | `crates/horizon-client/src/stations.rs` | rewrite required; core gameplay reference; version-specific | Enchanting table offers, anvil repair/enchant/rename costs, grindstone results/XP, smithing transform/trim recipes and stonecutter recipe selection. |
| 1663 | `crates/horizon-client/src/stats_overlay.rs` | undetermined; reference; optional mobile diagnostics | Performance overlay showing FPS, frame/CPU/GPU time, 1% low, RAM/GPU memory, chunk/meshing queues, draw calls, temperature, thermal status, render distance and backend. |
| 1664 | `crates/horizon-client/src/stick_memory.rs` | undetermined; reference; Starling-specific mobile input preference | Persists per-server controller stick mode in `starling-servers.txt`, normalizing server address keys and replacing prior mode choices. |
| 1665 | `crates/horizon-client/src/stopwatches.rs` | undetermined; reference; optional command utility | Named elapsed-time stopwatches persisted in `data/minecraft/stopwatches.dat` as accumulated milliseconds. |
| 1666 | `crates/horizon-client/src/structure_blocks.rs` | rewrite required; core worldgen/persistence reference; version-specific | Generated structure block entities, container loot unpacking, loot table caching, item components and deferred exploration-map search data. |
| 1667 | `crates/horizon-client/src/text_display.rs` | adaptable; version-specific; rendering | Display/Text Display synced fields, interpolation, billboard transforms, transformation matrix decomposition, text wrapping/alignment/background and culling bounds. |
| 1668 | `crates/horizon-client/src/tick_chunk_membership.rs` | adaptable; core performance reference; version-specific | Incremental cache of resident chunk membership for tick scheduler registration, including added/removed deltas and saved-tick waiting state. |
| 1669 | `crates/horizon-client/src/ticket_levels.rs` | rewrite required; core simulation reference; version-specific | Chunk ticket level calculation for player view/simulation distance, forced chunks, portal tickets, entity/block/random ticking and loaded status. |
| 1670 | `crates/horizon-client/src/title.rs` | rewrite required; client UI reference; version-specific | Title, world select/create/edit, multiplayer entry, world backup/recovery/optimization, data-pack/experiment flows, onboarding, account and ban-notice screen state. |
| 1671 | `crates/horizon-client/src/title/account.rs` | unnecessary; Starling-specific account/auth UI | Account/profile screen for Microsoft, Ely.by and offline profiles, account status, device-code sign-in, credential fields, game-file download and profile management. |
| 1672 | `crates/horizon-client/src/title/buffet.rs` | adaptable; version-specific; optional client UI | Single-biome world creation screen with searchable biome list, sorted names, selected biome, scrolling and Done/Cancel actions. |
| 1673 | `crates/horizon-client/src/title/confirm_experiments.rs` | adaptable; version-specific; optional client UI | Confirmation screen for data packs requiring experimental features, with warning text, details page listing feature flags, Proceed and Back actions. |
| 1674 | `crates/horizon-client/src/title/create_world.rs` | adaptable; version-specific; client UI | Create-world options, game mode/difficulty/cheats, world preset, structures/bonus chest, game rules, data packs, experiments and tab/navigation layout. |
| 1675 | `crates/horizon-client/src/title/experiments.rs` | adaptable; version-specific; optional client UI | Experiments screen listing shipped feature packs with enable/disable switches, descriptions, warning and Done/Cancel behavior. |

## Part 68 — Entries 1676–1700

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1676 | `crates/horizon-client/src/title/flat.rs` | adaptable; version-specific UI | Superflat presets, layer editor, biome selection, structures and preview. |
| 1677 | `crates/horizon-client/src/title/onboarding.rs` | unnecessary; Starling-specific UI | First-run onboarding, account prompts, controls hints, launcher links and setup actions. |
| 1678 | `crates/horizon-client/src/title/tests.rs` | undetermined; reference; tests only | Title/world screen tests for text, world listing/creation/editing, backups, navigation and errors. |
| 1679 | `crates/horizon-client/src/title/world_management.rs` | adaptable; version-specific UI | World backup, restore, optimize, delete and open-folder actions. |
| 1680 | `crates/horizon-client/src/toasts.rs` | adaptable; version-specific UI | Recipe, system, tutorial, advancement and social toast notifications. |
| 1681 | `crates/horizon-client/src/ui_theme.rs` | unnecessary; Horizon/Starling-specific UI | Custom palette, fonts, atlas/cache, rounded geometry, boot screen and preferences. |
| 1682 | `crates/horizon-client/src/vanillagen.rs` | rewrite required; core worldgen | Overworld generator/streaming integration, proto/final chunks, biomes, ticks, structures, block entities and saved proto restore. |
| 1683 | `crates/horizon-client/src/vanillagen/lod.rs` | undetermined; reference; far-render optimization | Async far decoration batches, cached LOD columns, vegetation/island estimates and detail requests. |
| 1684 | `crates/horizon-client/src/vanillagen/lod/islands.rs` | undetermined; reference; optional LOD | Estimates small End island cones from feature origins and deterministic radius/shrink. |
| 1685 | `crates/horizon-client/src/vanillagen/lod/vegetation.rs` | undetermined; reference; optional LOD | Estimates trees/chorus from biome feature JSON, expected counts and stable hashing. |
| 1686 | `crates/horizon-client/src/vanillagen/structure_mobs.rs` | rewrite required; worldgen mob finalization | Structure/chunk spawn finalizeSpawn, group data, equipment, attributes, brain NBT and passengers. |
| 1687 | `crates/horizon-client/src/win_screen.rs` | adaptable; version-specific UI | End poem/credits parsing, player name, obfuscation, narration, scrolling and backgrounds. |
| 1688 | `crates/horizon-client/src/world_spawn.rs` | adaptable; core world init | Initial spawn spiral, respawn column search, heightmap/fluid/top-face checks and candidate ordering. |
| 1689 | `crates/horizon-client/tests/custom_component_api.rs` | undetermined; reference; tests only | Tests custom component registration, NBT quotas/depth, content identity and mod acceptance. |
| 1690 | `crates/horizon-client/tests/custom_component_unregistered.rs` | undetermined; reference; tests only | Rejects an unregistered custom component wire ID. |
| 1691 | `crates/horizon-client/tests/custom_component_wire.rs` | undetermined; reference; tests only | Custom components across save NBT, wire slots, hashes, bundles, containers and malformed data. |
| 1692 | `crates/horizon-client/tests/datapack_biome_sync.rs` | undetermined; reference; tests only | Datapack biome registry sync and client reconstruction/recoloring. |
| 1693 | `crates/horizon-client/tests/forced_chunks.rs` | undetermined; reference; tests only | Forced chunk ticket rings, generation status and lighting. |
| 1694 | `crates/horizon-client/tests/generation_saves.rs` | undetermined; reference; tests only | Decoration consistency across travel/restart, proto restore, block entity IDs and tick persistence. |
| 1695 | `crates/horizon-client/tests/lan_bot.rs` | undetermined; reference; tests only; protocol integration | Offline protocol bot tests for login/config, registries, chunks/light, players/entities, recipes, menus, biome sync and LAN auth. |
| 1696 | `crates/horizon-client/tests/launch_api.rs` | undetermined; reference; tests only | Launch API JSON, CLI exit codes, crash report paths and home/game-dir handling. |
| 1697 | `crates/horizon-client/tests/mod_api_hooks.rs` | undetermined; reference; tests only; mod API | Compile/dispatch tests for key, movement and recipe-book category hooks. |
| 1698 | `crates/horizon-client/tests/streaming_meshes.rs` | undetermined; reference; tests only; core renderer regression | Tests loaded columns mesh on approach and relit neighbors remesh. |
| 1699 | `crates/horizon-client/tests/tall_column.rs` | undetermined; reference; tests only; height rendering | Tests streaming/light/meshing through 131 sections to y=1800. |
| 1700 | `crates/horizon-client/tests/wide_states.rs` | undetermined; reference; tests only; registry scale | Tests palette state IDs beyond 65,536 and 17-bit global storage. |

## Part 69 — Entries 1701–1725

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1701 | `crates/horizon-client/windows-segment-heap.manifest` | unnecessary; version-specific | Windows-only application manifest; not relevant to Android runtime. |
| 1702 | `crates/horizon-command/Cargo.toml` | unnecessary; reference | Workspace-specific command crate metadata, not runtime behavior. |
| 1703 | `crates/horizon-command/src/args.rs` | rewrite required; core reference; version-specific | Broad argument parser, but newer registry and argument types require 1.20.1 reconstruction. |
| 1704 | `crates/horizon-command/src/block.rs` | adaptable; version-specific | Block-state/property parser pattern needs target registry and tag parity. |
| 1705 | `crates/horizon-command/src/codec.rs` | rewrite required; version-specific | Codec schemas include Dialog and other post-1.20.1 forms. |
| 1706 | `crates/horizon-command/src/component.rs` | rewrite required; version-specific | Text/style component codec includes newer content and click-action forms. |
| 1707 | `crates/horizon-command/src/item.rs` | rewrite required; core reference; version-specific | Uses data components; 1.20.1 item arguments use legacy NBT semantics. |
| 1708 | `crates/horizon-command/src/lib.rs` | adaptable; core reference; version-specific | Brigadier dispatch, parse context and suggestions are valuable with target-specific arguments/tree. |
| 1709 | `crates/horizon-command/src/network.rs` | adaptable; version-specific | Server command-tree adapter must decode 1.20.1 protocol parser IDs. |
| 1710 | `crates/horizon-command/src/packrat.rs` | adaptable; core reference | Reusable grammar engine; surrounding argument grammars still need target parity. |
| 1711 | `crates/horizon-command/src/selector.rs` | adaptable; core reference; version-specific | Selector parser includes @n and later registry behavior absent from target baseline. |
| 1712 | `crates/horizon-command/src/snbt.rs` | adaptable; core reference; version-specific | SNBT architecture useful, but syntax includes newer numeric and built-in forms. |
| 1713 | `crates/horizon-command/src/tests.rs` | undetermined; reference; tests only | Command regression inventory; contains later-version cases and tests were not run. |
| 1714 | `crates/horizon-command/src/tree_data.rs` | adaptable; version-specific | Data/tag command structure is useful; newer compute branches must be excluded. |
| 1715 | `crates/horizon-command/src/tree_execute.rs` | adaptable; core reference; version-specific | Complex execute redirect tree; newer stopwatch/items/slots branches require exclusion. |
| 1716 | `crates/horizon-command/src/tree_extra.rs` | unnecessary; version-specific | Mostly Starling hosted-game and newer dedicated-server commands. |
| 1717 | `crates/horizon-command/src/tree_misc.rs` | adaptable; version-specific | Command registration patterns useful; current item/tick/random forms are newer. |
| 1718 | `crates/horizon-command/src/tree_scoreboard.rs` | adaptable; version-specific | Scoreboard/team base useful; NumberFormat and display-autoupdate are newer. |
| 1719 | `crates/horizon-command/src/tree_world.rs` | adaptable; version-specific | Several world commands are relevant, but exact branch signatures differ. |
| 1720 | `crates/horizon-command/src/vanilla_order.rs` | unnecessary; version-specific | Generated command order belongs to the newer command tree. |
| 1721 | `crates/horizon-command/src/vanilla.rs` | adaptable; core reference; version-specific | Dispatcher composition is useful; Clock/Timeline and newer feature branches differ. |
| 1722 | `crates/horizon-core/Cargo.toml` | adaptable; reference | Core/parity tool separation pattern; manifest itself is workspace-specific. |
| 1723 | `crates/horizon-core/examples/biome_zoom_fixture.rs` | adaptable; reference | Biome zoom parity fixture pattern; calculations require 1.20.1 oracle. |
| 1724 | `crates/horizon-core/examples/generation_tick_fixture.rs` | adaptable; core reference | Scheduled tick lifecycle fixture is relevant; target semantics must be verified. |
| 1725 | `crates/horizon-core/examples/paletted_fixture.rs` | adaptable; core reference | Palette wire and packed-storage round-trip fixture is relevant to large modpacks. |

## Part 70 — Entries 1726–1750

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1726 | `crates/horizon-core/src/bin/density_parity.rs` | adaptable; core reference | Differential harness for density graphs, samplers, caches and bulk outputs. |
| 1727 | `crates/horizon-core/src/bin/float_log_parity.rs` | adaptable; reference | Float-bit parity transport for Java-compatible logarithm behavior. |
| 1728 | `crates/horizon-core/src/bin/geometry_parity.rs` | adaptable; core reference | Differential geometry/collision and entity-movement fixtures. |
| 1729 | `crates/horizon-core/src/bin/noise_parity.rs` | adaptable; core reference | Noise, random-stream and octave-volume differential harness. |
| 1730 | `crates/horizon-core/src/bin/parity.rs` | adaptable; core reference | RNG, positional seeds, coordinate packing and PackedStorage fixtures. |
| 1731 | `crates/horizon-core/src/bin/tick_parity.rs` | adaptable; core reference | Scheduled tick lifecycle and chunk persistence trace adapter. |
| 1732 | `crates/horizon-core/src/biome.rs` | rewrite required; version-specific | Biome zoom and seed transform explicitly target Java 26.3. |
| 1733 | `crates/horizon-core/src/biome/seed_hash.rs` | unnecessary; version-specific | Specialized SHA-256 seed transform for the newer biome scheme. |
| 1734 | `crates/horizon-core/src/carving.rs` | adaptable; version-specific | Packed carving mask and ordered column-run traversal. |
| 1735 | `crates/horizon-core/src/carving/ellipsoid.rs` | adaptable; version-specific | Cave/canyon ellipsoid traversal with reach and iteration budget. |
| 1736 | `crates/horizon-core/src/density/arithmetic.rs` | adaptable; core reference | Java float arithmetic and specialized scalar/bulk density operations. |
| 1737 | `crates/horizon-core/src/density/beardifier.rs` | adaptable; version-specific; modded extension | Structure terrain beardifier mixes vanilla-like and mod-library kernels. |
| 1738 | `crates/horizon-core/src/density/cache.rs` | adaptable; optional/future | Context-local density cache plus sparse and plane preview caching. |
| 1739 | `crates/horizon-core/src/density/end_islands.rs` | rewrite required; version-specific | End island sampler explicitly targets Java 26.3. |
| 1740 | `crates/horizon-core/src/density/graph/compiler.rs` | adaptable; core reference; version-specific | Density DAG compiler with cache preparation and graph optimizations. |
| 1741 | `crates/horizon-core/src/density/graph/mod.rs` | adaptable; core reference; version-specific | Immutable density graph, interning, axes and interval range analysis. |
| 1742 | `crates/horizon-core/src/density/graph/seeded.rs` | adaptable; version-specific | Seed-bound noise factories and named-noise cache behavior. |
| 1743 | `crates/horizon-core/src/density/graph/tests.rs` | undetermined; reference; tests only | Density graph, cache, identity and compiler regression test inventory. |
| 1744 | `crates/horizon-core/src/density/interpolation.rs` | adaptable; core reference; version-specific | Coarse-grid density interpolation and sparse preview mode. |
| 1745 | `crates/horizon-core/src/density/interval.rs` | adaptable; core reference | Interval arithmetic including Java NaN, infinities and signed zero. |
| 1746 | `crates/horizon-core/src/density/mod.rs` | adaptable; core reference; version-specific | DensitySampler API, per-worker context and primitive samplers. |
| 1747 | `crates/horizon-core/src/density/power.rs` | adaptable; version-specific | Power sampler with constant-exponent specialization. |
| 1748 | `crates/horizon-core/src/density/rounding.rs` | adaptable; version-specific | Java-compatible floor/round/ceil/truncate and multiple operations. |
| 1749 | `crates/horizon-core/src/density/selection.rs` | adaptable; core reference; version-specific | Range choice, lerp and interval-branch density samplers. |
| 1750 | `crates/horizon-core/src/density/slice.rs` | adaptable; optional/future | Axis-fixed sampler and optional cached XZ plane broadcasting. |

## Part 71 — Entries 1751–1775

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1751 | `crates/horizon-core/src/density/spline.rs` | adaptable; core reference; version-specific | Cubic spline sampler, interpolation, range and coordinate caching. |
| 1752 | `crates/horizon-core/src/density/surface.rs` | adaptable; version-specific | Density crossing search for highest terrain surface. |
| 1753 | `crates/horizon-core/src/density/terrain_noise.rs` | rewrite required; core reference; version-specific | Shift/blended noise and distance density nodes need 1.20.1 RNG parity. |
| 1754 | `crates/horizon-core/src/density/tests.rs` | undetermined; reference; tests only | Density sampler buffer, cache, interpolation and failure-path test inventory. |
| 1755 | `crates/horizon-core/src/density/world_fields.rs` | adaptable; reference; incomplete | Context-bound blend/beardifier fields; complete blender algorithm is pending. |
| 1756 | `crates/horizon-core/src/fast_hash.rs` | adaptable; optional | Non-cryptographic fast hasher for internal hot-key collections. |
| 1757 | `crates/horizon-core/src/geometry/border.rs` | adaptable; version-specific | World-border geometry, transitions and collision calculations. |
| 1758 | `crates/horizon-core/src/geometry/mod.rs` | adaptable; core reference; version-specific | Vector/AABB/ray geometry with Java numeric edge behavior. |
| 1759 | `crates/horizon-core/src/geometry/movement.rs` | adaptable; core reference; version-specific | Entity collision resolver and step-up candidate selection. |
| 1760 | `crates/horizon-core/src/geometry/shape.rs` | adaptable; core reference; version-specific | VoxelShape occupancy, AABB decomposition, collision and clipping. |
| 1761 | `crates/horizon-core/src/geometry/shape/create.rs` | adaptable; version-specific | AABB-to-voxel-grid construction and shape optimization. |
| 1762 | `crates/horizon-core/src/geometry/shape/face.rs` | adaptable; version-specific | Lazy/transient voxel-face mask computation and caching. |
| 1763 | `crates/horizon-core/src/geometry/shape/join.rs` | adaptable; core reference; version-specific | Boolean VoxelShape composition and coordinate merging. |
| 1764 | `crates/horizon-core/src/geometry/sweep.rs` | adaptable; reference | Movement sweep block visitation and deduplication, not collision effects. |
| 1765 | `crates/horizon-core/src/geometry/traversal.rs` | adaptable; version-specific | Ray traversal through voxel blocks with explicit tie-break order. |
| 1766 | `crates/horizon-core/src/geometry/trig.rs` | rewrite required; architecture-specific | Sine lookup and fdlibm angle functions tied to the pinned runtime. |
| 1767 | `crates/horizon-core/src/jmath.rs` | rewrite required; architecture-specific; version-specific | Java math paths assume HotSpot x86/AArch64; Android ARM64 needs direct validation. |
| 1768 | `crates/horizon-core/src/lib.rs` | reference; incomplete; not a playable client | Upstream explicitly says core is not a playable game and parity covers only a subset. |
| 1769 | `crates/horizon-core/src/noise/mod.rs` | rewrite required; core reference; version-specific | Noise core explicitly targets 26.3 and says terrain generation remains incomplete. |
| 1770 | `crates/horizon-core/src/noise/normal.rs` | rewrite required; core reference; version-specific | NormalNoise octaves, normalization and legacy random consumption. |
| 1771 | `crates/horizon-core/src/noise/perlin.rs` | adaptable; core reference; version-specific | Perlin/smeared noise and batched Y-volume evaluation. |
| 1772 | `crates/horizon-core/src/noise/simplex.rs` | adaptable; version-specific | 2D/3D Simplex noise using seeded permutation tables. |
| 1773 | `crates/horizon-core/src/noise/stack.rs` | adaptable; version-specific | Ordered noise-layer aggregation and scalar/bulk summation. |
| 1774 | `crates/horizon-core/src/noise/volume.rs` | adaptable; core reference; version-specific | Bounded density volume descriptor with Z-X-Y flat indexing. |
| 1775 | `crates/horizon-core/src/paletted.rs` | rewrite required; core reference; version-specific | 26.3 palette strategy/wire thresholds differ from target 1.20.1. |

## Part 72 — Entries 1776–1800

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1776 | `crates/horizon-core/src/position.rs` | adaptable; core reference; version-specific | Block/section/chunk packed formats and negative-coordinate layouts. |
| 1777 | `crates/horizon-core/src/position/iter.rs` | adaptable; version-specific | Inclusive block-box iterator with directional traversal order. |
| 1778 | `crates/horizon-core/src/prepared_files.rs` | unnecessary; optional; version-specific | Horizon prepared-resource binary cache, tied to its own VFS. |
| 1779 | `crates/horizon-core/src/random.rs` | rewrite required; core reference; version-specific | Legacy/Xoroshiro streams and exact Java draw-consumption semantics. |
| 1780 | `crates/horizon-core/src/random/hash.rs` | adaptable; version-specific | MD5 name-derived seed transform for positional RNG. |
| 1781 | `crates/horizon-core/src/random/positional.rs` | rewrite required; core reference; version-specific | Position/name/seed factories differ between Legacy and Xoroshiro. |
| 1782 | `crates/horizon-core/src/release.rs` | unnecessary; reference | Release/nightly build labels, separate from Minecraft version. |
| 1783 | `crates/horizon-core/src/storage.rs` | adaptable; core reference; version-specific; architecture-specific | Packed storage; ARM64 path needs on-device measurement. |
| 1784 | `crates/horizon-core/src/text.rs` | rewrite required; version-specific | Integer parser explicitly uses JDK 25 digit semantics, not Java 17. |
| 1785 | `crates/horizon-core/src/ticks/generation.rs` | adaptable; core reference; version-specific | Proto-chunk tick deduplication and generation-stage routing. |
| 1786 | `crates/horizon-core/src/ticks/heap.rs` | adaptable; version-specific | Priority queue heap with JDK equal-key/removal behavior. |
| 1787 | `crates/horizon-core/src/ticks/index.rs` | adaptable; version-specific | Deadline hash index and fastutil-compatible traversal order. |
| 1788 | `crates/horizon-core/src/ticks/mod.rs` | rewrite required; core reference; version-specific | Full scheduled tick engine explicitly based on Java 26.3. |
| 1789 | `crates/horizon-core/src/version.rs` | unnecessary; version-specific | Hard-coded Minecraft 26.3/protocol 777/data version 5023. |
| 1790 | `crates/horizon-core/tests/carving_bounds.rs` | undetermined; reference; tests only | Carving bounds and iteration-budget regression cases. |
| 1791 | `crates/horizon-core/tests/density_world_fields.rs` | undetermined; reference; tests only | World-field defaults, blending order and graph optimization tests. |
| 1792 | `crates/horizon-cosmetics/Cargo.toml` | unnecessary; optional; version-specific | Optional remote cosmetics dependencies, not core client requirements. |
| 1793 | `crates/horizon-cosmetics/src/fetch_tests.rs` | undetermined; reference; tests only | Cosmetics batching, TTL, offline retry and disk cache tests. |
| 1794 | `crates/horizon-cosmetics/src/fetch.rs` | unnecessary; optional; version-specific | Third-party cosmetics fetch worker; separate from Minecraft protocol. |
| 1795 | `crates/horizon-cosmetics/src/item.rs` | unnecessary; optional; version-specific | Custom cosmetic model/texture format, unrelated to ItemStack. |
| 1796 | `crates/horizon-cosmetics/src/lib.rs` | unnecessary; optional; version-specific | Opt-in cosmetics facade with external service endpoint. |
| 1797 | `crates/horizon-cosmetics/src/pose.rs` | adaptable; optional; version-specific | Custom player-bone pose and animation matrices for cosmetics. |
| 1798 | `crates/horizon-dualsense/Cargo.toml` | unnecessary; platform-specific; optional | DualSense HID/CPAL manifest has no declared Android backend. |
| 1799 | `crates/horizon-dualsense/examples/dualsense.rs` | unnecessary; example only; platform-specific | Hardware output/rumble/trigger diagnostic; not gameplay input. |
| 1800 | `crates/horizon-dualsense/src/device.rs` | unnecessary; platform-specific; optional | USB/Bluetooth HID writer; Android compatibility is unproven. |


## Part 73 — Entries 1801–1825 (corrected against pinned source tree)

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1801 | `crates/horizon-dualsense/src/haptics.rs` | unnecessary; optional; platform-specific | USB DualSense waveform/audio output; optional controller haptics, not Android baseline. |
| 1802 | `crates/horizon-dualsense/src/haptics_stub.rs` | unnecessary; reference; platform-specific | iOS no-op haptics interface; does not establish Android support. |
| 1803 | `crates/horizon-dualsense/src/lib.rs` | unnecessary; optional; platform-specific | DualSense USB/BT output packet builder; hardware-specific optional feature. |
| 1804 | `crates/horizon-dualsense/src/trigger.rs` | unnecessary; optional; platform-specific | DualSense adaptive-trigger effect encoding; controller-specific only. |
| 1805 | `crates/horizon-friends/Cargo.toml` | unnecessary; optional/future; security-sensitive | Manifest for invite-based peer-hosting and secure friend tunnel. |
| 1806 | `crates/horizon-friends/src/code.rs` | adaptable; optional/future; security-sensitive | Crockford invite code parser/generator; security-sensitive short secret. |
| 1807 | `crates/horizon-friends/src/crypto.rs` | adaptable; reference; security-sensitive; optional/future | SPAKE2 plus authenticated encrypted stream framing; requires independent security audit. |
| 1808 | `crates/horizon-friends/src/direct.rs` | unnecessary; optional/future; network-specific | TCP simultaneous-open and direct peer connection race; not public-server transport. |
| 1809 | `crates/horizon-friends/src/guest.rs` | unnecessary; optional/future; version-specific | Guest-side direct/relay friend tunnel; one local loopback game connection. |
| 1810 | `crates/horizon-friends/src/host.rs` | unnecessary; optional/future; network-specific | Host-side integrated-server sharing over direct/relay paths. |
| 1811 | `crates/horizon-friends/src/invite.rs` | rewrite required; optional/future; security-sensitive | Versioned endpoint invite codec; decoder endpoint count needs hardening. |
| 1812 | `crates/horizon-friends/src/lib.rs` | unnecessary; optional/future; network-specific | Friend-tunnel module façade and stream pump; not normal Aternos transport. |
| 1813 | `crates/horizon-friends/src/portmap.rs` | unnecessary; optional/future; security-sensitive; platform-specific | PCP/NAT-PMP/UPnP router port mapping; explicit consent and SSRF review required. |
| 1814 | `crates/horizon-friends/src/reach.rs` | adaptable; reference; optional/future; privacy-sensitive | Interface, CGNAT and STUN reachability analysis for hosting. |
| 1815 | `crates/horizon-friends/src/session.rs` | adaptable; reference; version-specific; optional/future | Negotiates direct versus relay path for Horizon friend sessions. |
| 1816 | `crates/horizon-friends/src/stun.rs` | unnecessary; optional/future; privacy-sensitive | STUN public-IP discovery; default third-party disclosure. |
| 1817 | `crates/horizon-friends/src/wire.rs` | adaptable; reference; version-specific | Bounded JSON relay control framing, distinct from Minecraft protocol. |
| 1818 | `crates/horizon-hdr/Cargo.toml` | unnecessary; optional; platform-specific; version-specific | Manifest for optional wgpu/Vulkan HDR metadata and Linux Wayland dependencies. |
| 1819 | `crates/horizon-hdr/src/lib.rs` | adaptable; optional/future; platform-specific; architecture-specific | Vulkan HDR metadata bridge; platform/GPU support unverified. |
| 1820 | `crates/horizon-hdr/src/wayland.rs` | unnecessary; platform-specific | Linux Wayland display-luminance watcher; not Android. |
| 1821 | `crates/horizon-items/Cargo.toml` | rewrite required; version-specific | Modern Horizon block/item component crate manifest; target-specific dependencies. |
| 1822 | `crates/horizon-items/examples/enchantment_components_probe.rs` | undetermined; reference; example only | Offline item-component/enchantment fixture probe; example only. |
| 1823 | `crates/horizon-items/src/banner_layer.rs` | rewrite required; version-specific | Modern banner pattern component NBT conversion; incompatible with legacy 1.20.1 schema. |
| 1824 | `crates/horizon-items/src/blocks.rs` | rewrite required; core reference; version-specific | Per-block-state gameplay facts loader; architecture useful, source-version data differs. |
| 1825 | `crates/horizon-items/src/blocks/decode.rs` | adaptable; core reference; version-specific | Typed low-allocation JSON/shape decoder; validate geometry semantics strictly. |


## Part 74 — Entries 1826–1850

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1826 | `crates/horizon-items/src/blocks/decode_tests.rs` | undetermined; reference; tests only | Decoder tests useful to port; source tests were inspected, not run. |
| 1827 | `crates/horizon-items/src/bundle.rs` | adaptable; optional/future; version-specific; arithmetic-review | Bundle logic is useful but modern component format and bounds need adaptation. |
| 1828 | `crates/horizon-items/src/content_namespaces.rs` | adaptable; registry-integration; version-specific | Namespace discovery pattern may translate, but marker files differ from Forge's resource loader. |
| 1829 | `crates/horizon-items/src/cooldowns.rs` | rewrite required; version-specific | Grouped cooldown behavior depends on the modern use_cooldown component. |
| 1830 | `crates/horizon-items/src/enchant.rs` | rewrite required; core reference; version-specific | Enchantment architecture is relevant but its registry/effect model needs a full target rewrite. |
| 1831 | `crates/horizon-items/src/firework.rs` | rewrite required; version-specific | Modern firework component differs from 1.20.1 legacy NBT. |
| 1832 | `crates/horizon-items/src/flowers.rs` | rewrite required; version-specific | Flower/effect table includes post-1.20.1 content. |
| 1833 | `crates/horizon-items/src/item_data.rs` | rewrite required; core reference; version-specific | Modern ItemStack components do not match the 1.20.1 item/tag format. |
| 1834 | `crates/horizon-items/src/item_nbt.rs` | rewrite required; version-specific | ItemStack codec uses modern components rather than 1.20.1 legacy fields. |
| 1835 | `crates/horizon-items/src/items.rs` | rewrite required; core reference; version-specific | Item/loot architecture useful but context and predicate coverage are incomplete. |
| 1836 | `crates/horizon-items/src/lib.rs` | adaptable; module-structure reference | Small module façade and unsafe-code policy can inform crate organization. |
| 1837 | `crates/horizon-items/src/random_sequences.rs` | adaptable; core reference; version-specific | Named deterministic streams are useful but need exact 1.20.1 parity. |
| 1838 | `crates/horizon-items/src/recipe_filters.rs` | adaptable; optional/future; recipe-pipeline | Recipe filters may aid modpacks after target serializer integration. |
| 1839 | `crates/horizon-items/src/use_animation.rs` | rewrite required; version-specific | Modern consumable animation component differs from 1.20.1 item behavior. |
| 1840 | `crates/horizon-mcp/Cargo.toml` | unnecessary; server-tooling | MCP manifest is developer/server tooling, not game runtime. |
| 1841 | `crates/horizon-mcp/src/lib.rs` | unnecessary; server-tooling; protocol-specific | MCP JSON-RPC stdio is unrelated to Minecraft network protocol. |
| 1842 | `crates/horizon-mod-registry/Cargo.lock` | unnecessary; server-only; lockfile | Backend dependency lock graph should not enter Android client workspace. |
| 1843 | `crates/horizon-mod-registry/Cargo.toml` | unnecessary; server-only; toolchain-specific | Standalone registry manifest targets Rust 1.90 and server libraries. |
| 1844 | `crates/horizon-mod-registry/Dockerfile` | unnecessary; deployment-only | Dockerfile deploys hosted service, not mobile game. |
| 1845 | `crates/horizon-mod-registry/README.md` | reference; server-security; deployment-only | Security and workflow documentation is for hosted registry only. |
| 1846 | `crates/horizon-mod-registry/railway.json` | unnecessary; deployment-only | Railway config is hosted-service deployment metadata only. |
| 1847 | `crates/horizon-mod-registry/src/github.rs` | reference; server-tooling; security-sensitive | GitHub OAuth/API and PR automation are backend-only security-sensitive tools. |
| 1848 | `crates/horizon-mod-registry/src/lib.rs` | unnecessary; server-tooling; security-sensitive | Hosted registry API must not enter Android client. |
| 1849 | `crates/horizon-mod-registry/src/main.rs` | unnecessary; server-tooling | Registry main launches hosted Axum service, not Minecraft client. |
| 1850 | `crates/horizon-mod-registry/src/ratings.rs` | unnecessary; server-tooling | SQLite rating persistence belongs to hosted registry backend. |

## Part 75 — Entries 1851–1875

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1851 | `crates/horizon-mod-registry/tests/submit.rs` | undetermined; tests only; server-tooling | Mock-driven server tests only; fixtures and test expectations describe Horizon 26. |
| 1852 | `crates/horizon-mods/Cargo.toml` | rewrite required; version-specific; optional | Custom package/index formats and modern dependency versions; no Android target declaration or Forge bridge. |
| 1853 | `crates/horizon-mods/src/api_level.rs` | rewrite required; reference; version-specific | Hard-coded Horizon function catalog and level 7; unknown import is conservatively treated as newer; only component imports are parsed. |
| 1854 | `crates/horizon-mods/src/content.rs` | rewrite required; core reference; version-specific | Modern component and registry schema is not 1. |
| 1855 | `crates/horizon-mods/src/fetch.rs` | unnecessary; optional; external-service | Completion marker trusts cache contents without rehash; duplicate archive paths can overwrite; bounded aggregate extraction exists. |
| 1856 | `crates/horizon-mods/src/index.rs` | rewrite required; security-sensitive; reference | HTTPS validation is prefix-only rather than full URL parsing; resource-pack inspection checks path traversal but lacks explicit total expanded-byte/entry caps; revocation metadata validation is limited; unknown kinds are retained but not installable. |
| 1857 | `crates/horizon-mods/src/index/native.rs` | unnecessary; optional/future; toolchain-specific | Native bundle build pipeline is Horizon-specific; content entries are skipped from source bundle and no RustChunk/Forge native mod ABI exists. |
| 1858 | `crates/horizon-mods/src/index/registry.rs` | unnecessary; server-tooling; security-sensitive | New repository releases can be imported from configured repo without per-release review; review returns notes rather than hard errors for owner changes; date validation checks shape not calendar validity. |
| 1859 | `crates/horizon-mods/src/index/registry/tests.rs` | undetermined; tests only; server-tooling | Tests use Horizon sample versions and custom formats; none executed during this audit. |
| 1860 | `crates/horizon-mods/src/index/tests.rs` | undetermined; tests only | Sample game target is 26. |
| 1861 | `crates/horizon-mods/src/lib.rs` | adaptable; module-structure reference | Small organization file; public API is tied to Horizon-specific formats. |
| 1862 | `crates/horizon-mods/src/manifest.rs` | rewrite required; version-specific; security-sensitive | Manifest API 0. |
| 1863 | `crates/horizon-mods/src/modpack.rs` | rewrite required; optional; security-sensitive | Extraction writes through existing filesystem paths without explicit symlink-safe openat semantics; duplicate ZIP paths can be processed repeatedly; custom . |
| 1864 | `crates/horizon-mods/tests/index/coords_hud-0.1.0.hmod` | unnecessary; fixture-only | Binary test fixture only; custom Horizon format, not Forge . |
| 1865 | `crates/horizon-mods/tests/index/coords_hud-0.1.0.hmod.minisig` | unnecessary; fixture-only; security-sensitive | Throwaway test signature; no production trust root. |
| 1866 | `crates/horizon-mods/tests/index/cozy-1.0.0.hpack` | unnecessary; fixture-only | Custom Horizon modpack format; binary fixture only. |
| 1867 | `crates/horizon-mods/tests/index/glow-1.0.0.hshader` | unnecessary; fixture-only | Not a Minecraft 1. |
| 1868 | `crates/horizon-mods/tests/index/index-v1.json` | adaptable; reference; version-specific | Sample versions target 26. |
| 1869 | `crates/horizon-mods/tests/index/test.pub` | unnecessary; fixture-only; security-sensitive | Test-only key; private half deleted; not a RustChunk production key. |
| 1870 | `crates/horizon-narrator/Cargo.toml` | adaptable; optional; platform-specific | No target_os=android backend or Android speech dependency is declared; Android does not match target_os=linux. |
| 1871 | `crates/horizon-narrator/examples/say.rs` | unnecessary; example-only | Example-only; sleep duration uses message byte length and is not a reliable completion signal for multilingual speech. |
| 1872 | `crates/horizon-narrator/src/lib.rs` | adaptable; optional; platform-specific; architecture reference | Global mutex is held while calling sink methods; speech command channel is unbounded; set_volume does not validate finite 0. |
| 1873 | `crates/horizon-narrator/src/screen.rs` | adaptable; optional; UI reference | Collector retains old entries indefinitely; generation i64 increment can overflow in very long-lived state; caller must maintain stable widget IDs and monotonic clock. |
| 1874 | `crates/horizon-narrator/src/speech.rs` | adaptable; optional; platform-specific | Unbounded std mpsc command channel; thread spawn failure reported asynchronously as inactive; cfg targets omit Android backend. |
| 1875 | `crates/horizon-narrator/src/speech/linux.rs` | unnecessary; platform-specific; optional | Compiled only for target_os=linux, not target_os=android; native libraries may be unavailable; unbounded piece queue and batch counter can wrap; raw C ABI needs ABI/library validation. |


## Part 100 — Entries 1876–1900

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1876 | `crates/horizon-narrator/src/speech/linux_split.rs` | unnecessary; platform-specific helper | Only used by Linux backend. |
| 1877 | `crates/horizon-narrator/src/speech/mac.rs` | unnecessary; platform-specific | Queue is unbounded. |
| 1878 | `crates/horizon-narrator/src/speech/windows.rs` | unnecessary; platform-specific | Volume is cast to SAPI integer without explicit finite/range validation. |
| 1879 | `crates/horizon-optifine/Cargo.toml` | adaptable; optional; version-specific | Workspace Rust version and crate dependencies must be checked against RustChunk toolchain/Android build. |
| 1880 | `crates/horizon-optifine/examples/cem_expr_bench.rs` | undetermined; benchmark/example only | Synthetic single-entity workload. |
| 1881 | `crates/horizon-optifine/src/blink.rs` | adaptable; optional; resource-pack feature | Blink properties use read_to_string rather than Java-compatible Latin-1 decoder. |
| 1882 | `crates/horizon-optifine/src/cem/curves.rs` | adaptable; optional; animation subsystem | Public keyframe helper assumes non-empty frames and can underflow/index panic when called directly with an empty slice. |
| 1883 | `crates/horizon-optifine/src/cem/expr.rs` | adaptable; optional; core reference for CEM only | Parser caps nesting 64 and nodes 1024. |
| 1884 | `crates/horizon-optifine/src/cem/files.rs` | adaptable; optional; resource-pack feature | CEM discovery is limited to assets/minecraft namespace. |
| 1885 | `crates/horizon-optifine/src/cem/json.rs` | adaptable; optional; parser reference | No intrinsic input-byte/node-count cap. |
| 1886 | `crates/horizon-optifine/src/cem/mod.rs` | adaptable; module-structure reference | Small module organization file. |
| 1887 | `crates/horizon-optifine/src/cem/model.rs` | rewrite required; optional; version-specific | Target model schema and transforms need OptiFine/EMF 1.20.1 parity. |
| 1888 | `crates/horizon-optifine/src/cit.rs` | rewrite required; optional; version-specific | File explicitly follows 1.21 component rules and CIT Resewn Continuation. |
| 1889 | `crates/horizon-optifine/src/ctm.rs` | adaptable; optional; rendering feature | `load` reads each properties file fully without a per-file byte cap. |
| 1890 | `crates/horizon-optifine/src/ctm/connect.rs` | adaptable; optional; renderer-specific | Public repeat_index assumes sensible width/height. |
| 1891 | `crates/horizon-optifine/src/emissive.rs` | adaptable; optional; rendering feature | Properties suffix files are size-capped, but overlay discovery inherits walker count/symlink constraints. |
| 1892 | `crates/horizon-optifine/src/fusion.rs` | unnecessary; optional; third-party format | Fusion is a separate mod's format, not native vanilla or Forge connected-texture format. |
| 1893 | `crates/horizon-optifine/src/fusion/layout.rs` | unnecessary; optional; third-party format | Layout tables are derived from public wiki/example template images. |
| 1894 | `crates/horizon-optifine/src/fusion/model.rs` | unnecessary; optional; third-party format | Reference resolution is capped at 32 hops. |
| 1895 | `crates/horizon-optifine/src/fusion/predicate.rs` | adaptable; optional; renderer-specific | Nested predicates rely on JSON parser depth cap rather than a dedicated predicate-node budget. |
| 1896 | `crates/horizon-optifine/src/fusion/texture.rs` | unnecessary; optional; third-party format | This is a mod-specific .mcmeta extension. |
| 1897 | `crates/horizon-optifine/src/lib.rs` | adaptable; optional; architecture reference | Broad public surface may pull in features unnecessary for RustChunk and increase binary/dependency footprint.. |
| 1898 | `crates/horizon-optifine/src/matchers.rs` | adaptable; optional; parser utility | Range parser accepts reversed bounds that simply never match. |
| 1899 | `crates/horizon-optifine/src/paths.rs` | rewrite required; security-sensitive; reference | File symlinks are not canonicalized against the resource root before returned paths are opened. |
| 1900 | `crates/horizon-optifine/src/properties.rs` | rewrite required; parser correctness/security-sensitive | `split` scans UTF-8 bytes and advances two bytes after backslash. |


## Part 76 — Entries 1901–1925

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1901 | `crates/horizon-optifine/src/random.rs` | reference; optional resource-pack feature; version-specific | OptiFine/ETF random-entity rules may support optional entity texture variants after 1.20.1 parity. |
| 1902 | `crates/horizon-optifine/src/random_textures.rs` | reference; optional resource-pack feature; version-specific | Variant scanner is useful only for a future opt-in entity texture feature. |
| 1903 | `crates/horizon-port-pack/Cargo.toml` | unnecessary; tooling-only; version-specific | Horizon port-pack is a migration tool, not an Android game runtime dependency. |
| 1904 | `crates/horizon-port-pack/src/carry.rs` | rewrite required; tooling-only; version-specific; security-sensitive | The carry engine targets 1.21.1→26.3 and requires a 1.20.1 rewrite plus archive hardening. |
| 1905 | `crates/horizon-port-pack/src/check.rs` | adaptable; tooling reference; version-specific | Preflight verdict design can guide a future mod report, but it does not prove mods run. |
| 1906 | `crates/horizon-port-pack/src/classes.rs` | adaptable; tooling reference; mapping-sensitive | Class census is a diagnostics idea, but its modern name map is not valid proof for Forge 1.20.1. |
| 1907 | `crates/horizon-port-pack/src/classify.rs` | adaptable; tooling reference; pack-specific | ATM10-specific rules and cycle layering are not general-purpose compatibility decisions. |
| 1908 | `crates/horizon-port-pack/src/fetch.rs` | adaptable; tooling reference; security-sensitive | Download flow has good temp/hash patterns but missing hashes, redirects and local scans need hardening. |
| 1909 | `crates/horizon-port-pack/src/import.rs` | adaptable; tooling reference; security-sensitive | Pack import formats can be reused conceptually, but archive and path limits need work. |
| 1910 | `crates/horizon-port-pack/src/inventory.rs` | adaptable; tooling reference; dependency-sensitive | Inventory architecture is useful, but dependency-source precedence and duplicate keys need validation. |
| 1911 | `crates/horizon-port-pack/src/jar.rs` | adaptable; tooling reference; security-sensitive | JAR census is useful for inspection but has unbounded metadata reads and is not runtime validation. |
| 1912 | `crates/horizon-port-pack/src/legacy_structure_mods/ctov.rs` | unnecessary; mod-specific migration data; version-specific | Hard-coded CTOV defaults are version/config specific, not general game functionality. |
| 1913 | `crates/horizon-port-pack/src/legacy_structure_mods/mod.rs` | unnecessary; tooling structure | undefined |
| 1914 | `crates/horizon-port-pack/src/lib.rs` | unnecessary; tooling-only | undefined |
| 1915 | `crates/horizon-port-pack/src/lithostitched.rs` | rewrite required; tooling-only; worldgen-specific | undefined |
| 1916 | `crates/horizon-port-pack/src/main.rs` | unnecessary; tooling-only; version-specific | undefined |
| 1917 | `crates/horizon-port-pack/src/modrinth.rs` | adaptable; tooling reference; network-sensitive | undefined |
| 1918 | `crates/horizon-port-pack/src/net.rs` | adaptable; reference; network-security-sensitive | undefined |
| 1919 | `crates/horizon-port-pack/src/oracle.rs` | rewrite required; core reference; version-specific; heuristic | undefined |
| 1920 | `crates/horizon-port-pack/src/pipeline.rs` | adaptable; tooling reference; version-specific | Pipeline architecture may aid tooling but its content-only portability model is not Forge mod execution. |
| 1921 | `crates/horizon-port-pack/src/processors.rs` | rewrite required; tooling-only; worldgen-specific | undefined |
| 1922 | `crates/horizon-port-pack/src/report.rs` | adaptable; tooling reference; reporting-only | undefined |
| 1923 | `crates/horizon-port-pack/src/resourcepacks.rs` | adaptable; core tooling reference; security-sensitive | undefined |
| 1924 | `crates/horizon-port-pack/src/resourcepacks/tests.rs` | undetermined; tests only; reference | undefined |
| 1925 | `crates/horizon-port-pack/src/upgrade/density.rs` | rewrite required; tooling-only; worldgen-specific; version-specific | This module upgrades 26.2 to 26.3 density schemas and is not a 1.20.1 implementation. |


## Part 77 — Entries 1926–1950

| No. | Source path | Decision | One-line reason |
|---:|---|---|---|
| 1926 | `crates/horizon-port-pack/src/upgrade/features.rs` | rewrite required; tooling-only; worldgen-specific | undefined |
| 1927 | `crates/horizon-port-pack/src/upgrade/mod.rs` | rewrite required; tooling-only; version-specific; security-sensitive | undefined |
| 1928 | `crates/horizon-port-pack/src/upgrade/noise.rs` | rewrite required; tooling-only; worldgen-specific; numeric-review | undefined |
| 1929 | `crates/horizon-port-pack/src/upgrade/registries.rs` | rewrite required; tooling-only; registry-specific | undefined |
| 1930 | `crates/horizon-port-pack/src/upgrade/settings.rs` | rewrite required; tooling-only; worldgen-specific | undefined |
| 1931 | `crates/horizon-port-pack/src/worldgen.rs` | rewrite required; tooling-only; worldgen-specific; security-sensitive | undefined |
| 1932 | `crates/horizon-port-pack/src/yung.rs` | unnecessary; mod-specific migration; version-specific | undefined |
| 1933 | `crates/horizon-prepare/build.rs` | adaptable; build-tooling reference; cache-integrity-sensitive | undefined |
| 1934 | `crates/horizon-prepare/output-versions.txt` | reference; build metadata only | undefined |
| 1935 | `crates/horizon-prepare/src/exporters.rs` | adaptable; preparation-tooling reference; version-specific | undefined |
| 1936 | `crates/horizon-prepare/src/files.rs` | adaptable; core preparation reference; cross-platform | undefined |
| 1937 | `crates/horizon-prepare/src/inventory.rs` | adaptable; tooling reference; archive-security-sensitive | undefined |
| 1938 | `crates/horizon-prepare/src/launch_api.rs` | adaptable; launcher tooling reference; versioned-api | undefined |
| 1939 | `crates/horizon-prepare/src/pool.rs` | adaptable; preparation reference; concurrency-sensitive | undefined |
| 1940 | `crates/horizon-prepare/src/pyjson.rs` | adaptable; parser reference; robustness-sensitive | undefined |
| 1941 | `crates/horizon-prepare/src/sounds.rs` | adaptable; core launcher reference; asset-integrity-sensitive | undefined |
| 1942 | `crates/horizon-prepare/src/survival.rs` | adaptable; core reference; version-specific | undefined |
| 1943 | `crates/horizon-prepare/src/worker.rs` | unnecessary; preparation-tooling; resource-sensitive | undefined |
| 1944 | `crates/horizon-prepare/tests/java_selection.rs` | reference; tests only; launcher-tooling | undefined |
| 1945 | `crates/horizon-prepare/tests/support/java_probe.rs` | unnecessary; test fixture only | undefined |
| 1946 | `crates/horizon-protocol/examples/item_wire_probe.rs` | reference; example only; version-specific | undefined |
| 1947 | `crates/horizon-protocol/examples/net_replay.rs` | adaptable; benchmark reference; protocol-specific | undefined |
| 1948 | `crates/horizon-protocol/examples/probe.rs` | reference; example only; protocol-specific | undefined |
| 1949 | `crates/horizon-protocol/examples/stop_sound_probe.rs` | reference; example only; test-harness | undefined |
| 1950 | `crates/horizon-relay/Cargo.toml` | unnecessary; optional server-only; network-specific | undefined |

| 1951 | `crates/horizon-relay/Dockerfile` | reference; deployment-only; optional relay | Docker packaging only; no Minecraft client runtime code. |
| 1952 | `crates/horizon-relay/horizon-relay.service` | reference; deployment configuration; optional relay | systemd hardening directives for a separately hosted relay. |
| 1953 | `crates/horizon-relay/src/lib.rs` | unnecessary; optional friend relay; network-specific | Opaque encrypted-stream relay, not public-server client transport; concurrency and config validation need hardening. |
| 1954 | `crates/horizon-relay/src/limits.rs` | adaptable; rate-limiting reference; security-sensitive | Token-bucket and address grouping ideas; reject NaN/non-positive rates and bound tracked addresses. |
| 1955 | `crates/horizon-relay/src/main.rs` | unnecessary; relay executable; server-only | Standalone service listener and count logging, not launcher code. |
| 1956 | `crates/horizon-relay/tests/tunnel.rs` | reference; integration tests only; not executed | Exercises direct/relay paths and abuse cases; no test was run during this audit. |
| 1957 | `crates/horizon-release-sign/Cargo.toml` | reference; release tooling; cryptographic dependency | Manifest for Ed25519/SHA-256 release-signing tools, not game runtime. |
| 1958 | `crates/horizon-release-sign/legacy-releases.txt` | reference; release integrity data; legacy allowlist | Pins archive hashes for pre-signing releases; treat as trusted release metadata. |
| 1959 | `crates/horizon-release-sign/release-keys.txt` | reference; public key material; security-sensitive | Contains public Ed25519 trust anchor only; private key is maintained outside source. |
| 1960 | `crates/horizon-release-sign/src/lib.rs` | adaptable; secure update verification; version-independent | Tag-bound Ed25519 signatures and streaming SHA-256 are reusable patterns; checksum path safety and strict parsing need hardening. |
| 1961 | `crates/horizon-release-sign/src/main.rs` | adaptable; release CLI tooling; security-sensitive | Key generation and verification flow is useful only after path traversal and input-size validation. |
| 1962 | `crates/horizon-release-sign/tests/cli.rs` | reference; CLI integration tests only; not executed | Tests signing, tampering, rotation and keygen; malicious checksum path cases are absent. |
| 1963 | `crates/horizon-server-files/Cargo.toml` | unnecessary; dedicated-server only; host-side dependency | Dependencies for Horizon's server-side files and account lookup, not Android launcher runtime. |
| 1964 | `crates/horizon-server-files/src/eula.rs` | reference; server legal-consent handling; version-specific | EULA defaults to false and missing/unreadable file is rewritten as not agreed. |
| 1965 | `crates/horizon-server-files/src/java_properties.rs` | adaptable; Java-properties compatibility; parity-sensitive | Emulates Java load/store and UTF-16 key ordering; surrogate/NUL edge cases need JVM differential tests. |
| 1966 | `crates/horizon-server-files/src/json.rs` | adaptable; JSON compatibility utility; security-sensitive | Preserves member order/number text and Gson-style output; parser needs depth and input-size bounds. |
| 1967 | `crates/horizon-server-files/src/lib.rs` | unnecessary; server-only module exports | Exports EULA, properties, JSON, profile, cache and access-list modules for the dedicated server. |
| 1968 | `crates/horizon-server-files/src/profiles.rs` | reference; server account/profile lookup; network-sensitive | Uses service discovery and public profile keys; response limits and endpoint-origin policy need hardening. |
| 1969 | `crates/horizon-server-files/src/properties.rs` | reference; server settings schema; security-sensitive; version-specific | Modern dedicated-server property set; time-based entropy fallback is unsafe for management secrets and semantics differ from 1.20.1. |
| 1970 | `crates/horizon-server-files/src/user_cache.rs` | adaptable; user-cache behavior; memory-sensitive | One-month name/UUID cache and MRU output; internal vectors/maps need bounded eviction. |
| 1971 | `crates/horizon-server-files/src/user_lists.rs` | adaptable; access-list persistence; compatibility-sensitive | Ops/whitelist/ban lists and Java HashMap ordering; add strict JSON/file bounds and atomic saves. |
| 1972 | `crates/horizon-server/Cargo.toml` | unnecessary; dedicated-server supervisor; server-only | Server control plane, TLS, updates, backups and moderation filters; not the Android game client. |
| 1973 | `crates/horizon-server/panel/index.html` | reference; admin panel markup; server-only | Static HTML for a server dashboard; all controls depend on authenticated backend routes. |
| 1974 | `crates/horizon-server/panel/panel.css` | unnecessary; presentation-only CSS; server-only | Responsive light/dark dashboard styling, no engine or launcher logic. |
| 1975 | `crates/horizon-server/panel/panel.js` | adaptable; admin UI safety pattern; server-only | textContent and same-origin API patterns are useful; add polling overlap guard and verify backend auth. |
| 1976 | `crates/horizon-server/src/admin.rs` | reference; dedicated-server admin API; server-only | Shows structured moderation routing and metrics, but not Android launcher or Minecraft client functionality. |
| 1977 | `crates/horizon-server/src/api.rs` | adaptable; authenticated control API patterns; server tooling | Potential architectural reference for a future admin companion; no proof of Forge 1.20.1 gameplay support. |
| 1978 | `crates/horizon-server/src/backup.rs` | adaptable; backup workflow reference; security-sensitive | Atomic staging and recovery concepts may transfer to launcher instance backup, but server-specific paths and risks need a rewrite. |
| 1979 | `crates/horizon-server/src/cli.rs` | reference; server CLI; server-only | Useful operational UX ideas, but not necessary for the RustChunk mobile launcher core. |
| 1980 | `crates/horizon-server/src/config.rs` | reference; version-specific server config; server-only | Useful validation design pattern but not suitable as a drop-in 1.20.1 configuration schema. |
| 1981 | `crates/horizon-server/src/console.rs` | reference; terminal management UX; host-only | May inform developer diagnostics but does not address Minecraft 1.20.1 game performance or networking. |
| 1982 | `crates/horizon-server/src/discord.rs` | unnecessary; optional Discord server integration; server-only | Not needed for Minecraft client/Forge compatibility; only relevant if RustChunk later includes server operations. |
| 1983 | `crates/horizon-server/src/engine.rs` | reference; headless server engine; version-specific | The process/control patterns may inform tooling but the executable/resource assumptions cannot be ported directly. |
| 1984 | `crates/horizon-server/src/events.rs` | adaptable; event stream pattern; server tooling | Potentially reusable for RustChunk diagnostics, not core gameplay or Forge mod compatibility. |
| 1985 | `crates/horizon-server/src/http.rs` | adaptable; minimal HTTP reference; security-sensitive | Limits and routing patterns may be informative, but its plaintext/unbounded connection architecture is unsuitable as-is. |
| 1986 | `crates/horizon-server/src/init.rs` | reference; dedicated-server init flow; server-only | Useful consent and wizard sequencing ideas, but not a client-launcher implementation. |
| 1987 | `crates/horizon-server/src/lists.rs` | reference; dedicated-server access lists; server-only | Potentially useful format/locking concepts, but not needed for joining Aternos or managing client accounts. |
| 1988 | `crates/horizon-server/src/main.rs` | reference; server command dispatcher; server-only | No direct launcher functionality; useful only as a command-line UX example. |
| 1989 | `crates/horizon-server/src/management.rs` | reference; server management WebSocket; version-specific | Not a substitute for Minecraft 1.20.1 client networking; only administrative protocol patterns can transfer. |
| 1990 | `crates/horizon-server/src/management_methods.rs` | adaptable; typed server API conversion; version-specific | Good example of schema-driven conversion, but its implementation is partial relative to its own schema and tied to 26.3. |
| 1991 | `crates/horizon-server/src/management_schema.rs` | reference; management protocol schema; version-specific | Schema-generation pattern only; all method names and DTOs need fresh 1.20.1 evidence before reuse. |
| 1992 | `crates/horizon-server/src/management_tls.rs` | adaptable; TLS primitives; security-sensitive | Generic TLS patterns may transfer, but management service and key format are not required for Minecraft client play. |
| 1993 | `crates/horizon-server/src/mcp.rs` | unnecessary; optional MCP server integration; server-only | Not needed for the RustChunk launcher/game client. |
| 1994 | `crates/horizon-server/src/panel.rs` | reference; web admin panel auth; server-only | Some browser hardening patterns can transfer to a separate companion UI, not to the launcher runtime. |
| 1995 | `crates/horizon-server/src/process.rs` | unnecessary; Windows server process helper; host-only | Irrelevant to Android launcher and Minecraft gameplay. |
| 1996 | `crates/horizon-server/src/query.rs` | reference; GS4 query server; version-specific | Not required for the RustChunk launcher to join public/Aternos servers. |
| 1997 | `crates/horizon-server/src/rcon.rs` | reference; RCON server; server-only | No role in launcher, chunk rendering, Forge mod loading or Aternos join protocol. |
| 1998 | `crates/horizon-server/src/reference.rs` | adaptable; generated documentation test pattern; test-only | No direct game feature; useful only for maintaining future RustChunk documentation. |
| 1999 | `crates/horizon-server/src/service.rs` | reference; OS service integration; host-only | Not applicable to Android launcher runtime. |
| 2000 | `crates/horizon-server/src/settings.rs` | adaptable; typed config change workflow; server-only | A generic settings workflow may transfer, but current property names and defaults cannot be copied to Forge 1.20.1. |
| 2001 | `crates/horizon-server/src/signals.rs` | unnecessary; OS stop-signal handler; dedicated-server-only | Registration failures are ignored. |
| 2002 | `crates/horizon-server/src/supervisor.rs` | unnecessary; dedicated-server process supervisor; host-only | Early stop-signal race; failed program/resource resolution inside restart loop is silently ignored and stale paths reused; unbounded console mpsc channel; log pump stops on read/UTF-8 error and may stop draining child pipes; thread spawn failures are often ignored. |
| 2003 | `crates/horizon-server/src/update.rs` | adaptable; signed release/update workflow; server updater | Download and expanded archive sizes/counts are unbounded; Windows switch removes old junction before creating new (not atomic); replacing a non-current target removes it before staged rename; release tags permit characters unsafe on Windows; previous marker is plain text without strict tag validation. |
| 2004 | `crates/horizon-server/src/websocket.rs` | adaptable; WebSocket framing and TLS I/O reference; management-only | Does not validate RSV bits, canonical extended lengths, 64-bit high bit, control-frame fragmentation/size rules or close payload. |
| 2005 | `crates/horizon-shader-api/Cargo.toml` | adaptable; shader tooling dependency manifest; version-specific | Naga 30. |
| 2006 | `crates/horizon-shader-api/packs/golden_hour/README.md` | adaptable; example shader-pack documentation; sample-only | Concise example docs, but depend on Horizon shader API/CLI. |
| 2007 | `crates/horizon-shader-api/packs/golden_hour/common.wgsl` | adaptable; reusable WGSL helper examples; sample-only | Generic math is portable; dimension ID and frame semantics are API-specific. |
| 2008 | `crates/horizon-shader-api/packs/golden_hour/horizon-shaders.toml` | adaptable; shader pack manifest example; API-specific | Readable declarative composition; relies on Horizon stage/resource rules. |
| 2009 | `crates/horizon-shader-api/packs/golden_hour/post/glow.wgsl` | adaptable; post-process glow sample; renderer-specific | Bounded tap count; reversed-depth and sampler assumptions are renderer-specific. |
| 2010 | `crates/horizon-shader-api/packs/golden_hour/post/grade.wgsl` | adaptable; color-grading sample; renderer-specific | Small composable effect; Frame semantics/thresholds are Horizon-specific. |
| 2011 | `crates/horizon-shader-api/packs/golden_hour/sky.wgsl` | adaptable; sky shader sample; renderer-specific | Assumes renderer later draws sun, moon, stars and clouds over result. |
| 2012 | `crates/horizon-shader-api/src/bin/horizon-shader.rs` | adaptable; shader authoring and validation CLI; host tooling | Argument handling permissive: unknown/missing flags may be ignored; --name only replaces quote and not newline/backslash; pack can overwrite output directly, non-atomically. |
| 2013 | `crates/horizon-shader-api/src/check.rs` | adaptable; WGSL validation and backend translation; security-sensitive | cost() claims samples/gathers/loads but counts only ImageSample and ImageLoad: ImageGather escapes MAX_TEXTURE_READS. |
| 2014 | `crates/horizon-shader-api/src/inputs.rs` | rewrite required; renderer frame and stage input contract; version-specific | 49-field Frame includes camera/sun/moon/dimension/weather/lighting/reflection/shadow/mod/temporal semantics specific to current renderer; tests do not assert each field offset/alignment vs WGSL. |
| 2015 | `crates/horizon-shader-api/src/lib.rs` | adaptable; shader API crate exports; module structure | Small cohesive public surface; inherits version-specific internals. |
| 2016 | `crates/horizon-shader-api/src/manifest.rs` | adaptable; strict shader manifest schema; validation gaps | Stage-key validation omits Weather for fog, scale/output and sky-only clouds/sun/stars; Vertex is omitted from one sky-only check. |
| 2017 | `crates/horizon-shader-api/src/package.rs` | adaptable; shader package reader and archive safety; security-sensitive | ZIP entries are buffered individually before 8MiB aggregate check, allowing excess memory. |
| 2018 | `crates/horizon-shader-api/src/reference.rs` | adaptable; generated shader API reference; documentation tooling | Source-generated docs reduce drift, but helper prose is partly manual and may diverge. |
| 2019 | `crates/horizon-shader-api/src/stages.rs` | rewrite required; renderer stage composition and WGSL preludes; version-specific | Bindings and entrypoints tightly coupled to current renderer. |
| 2020 | `crates/horizon-shader-api/templates/material/horizon-shaders.toml` | adaptable; material shader starter manifest; sample-only | Minimal template; Horizon-specific keys. |
| 2021 | `crates/horizon-shader-api/templates/material/material.wgsl` | adaptable; material shader starter; sample-only | Horizon tint/lightmap semantics. |
| 2022 | `crates/horizon-shader-api/templates/post/horizon-shaders.toml` | adaptable; post shader starter manifest; sample-only | Budget is declarative/overlay-oriented, not measured enforcement. |
| 2023 | `crates/horizon-shader-api/templates/post/post.wgsl` | adaptable; post shader starter example; sample-only | Assumes Horizon pass pixel and linear color semantics. |
| 2024 | `crates/horizon-shader-api/templates/sky/horizon-shaders.toml` | adaptable; sky shader starter manifest; sample-only | Assumes Horizon sky pass order. |
| 2025 | `crates/horizon-shader-api/templates/sky/sky.wgsl` | adaptable; sky shader starter example; sample-only | Depends on normalized sun vector and later sky overlays. |

| 2026 | `crates/horizon-shader-api/tests/check.rs` | reference; WGSL checker test coverage; test-only | The budget case generates 70 textureLoad calls; no case exercises ImageGather, leaving the checker accounting gap unguarded. Tests are not a runtime proof. |
| 2027 | `crates/horizon-shader-api/tests/reference.rs` | reference; Naga shader-layout tests; test-only | Layout checks size and offsets but not a complete field-type/interface contract; Naga validation does not establish Android backend correctness. |
| 2028 | `crates/horizon-shader-api/tests/samples/schema-1/common.wgsl` | adaptable; generic luminance math; sample-only | Math is generic, but color-space and input-linearization assumptions belong to the calling renderer. |
| 2029 | `crates/horizon-shader-api/tests/samples/schema-1/horizon-shaders.toml` | reference; schema-1 manifest fixture; sample-only | Header claims every schema-1 key, but this fixture omits several optional schema features covered elsewhere by tests; it is not a complete conformance fixture. |
| 2030 | `crates/horizon-shader-api/tests/samples/schema-1/material.wgsl` | rewrite required; material stage function; Horizon-specific | Uses Horizon MaterialInput/Surface fields and reflection bindings; fluid/face IDs and lighting semantics are not Minecraft 1.20.1 contracts. |
| 2031 | `crates/horizon-shader-api/tests/samples/schema-1/post/grade.wgsl` | adaptable; post-process color grading; renderer-specific | The declared choice has three values, but `tonemap > 0` selects the same branch for both nonzero choices; Soft and Filmic are not distinguished. |
| 2032 | `crates/horizon-shader-api/tests/samples/schema-1/post/vignette.wgsl` | adaptable; vignette post-process; renderer-specific | The manifest declares `samples`, but this shader never reads it; UV/color conventions are renderer-specific. |
| 2033 | `crates/horizon-shader-api/tests/samples/schema-1/README.md` | reference; shader fixture documentation; test-only | Documentation is one line and does not describe the broader schema or prove compatibility. |
| 2034 | `crates/horizon-shader-api/tests/samples/schema-1/sky.wgsl` | adaptable; sky pass passthrough; sample-only | Only demonstrates function shape; sky color, cloud, sun and fog composition are Horizon-specific. |
| 2035 | `crates/horizon-shader-api/tests/schema.rs` | reference; manifest and package validation tests; test-only | Tests use PID-derived temporary directory names and remove them recursively; collision/isolation should be considered. No tests were run. |
| 2036 | `crates/horizon-shader-api/tests/sdk.rs` | reference; shader CLI integration tests; host-only | Subprocess output is captured without an explicit timeout or output cap; tests are host-only and were inspected, not executed. |
| 2037 | `crates/horizon-shaders/Cargo.toml` | reference; GLSL shaderpack dependencies; version-specific | Naga 30.0.1 and this dependency graph are tied to the pinned Starling workspace, not automatically suitable for Android or Java 1.20.1. |
| 2038 | `crates/horizon-shaders/examples/pack_report.rs` | reference; shader-pack diagnostic report tool; host-only | `--example N` prints at most one sample per cause; invalid `--dims` tokens are silently dropped; backend check examines only the first entry point; synthetic game inputs approximate rather than prove actual pipeline compatibility. |
| 2039 | `crates/horizon-shaders/src/biomes.rs` | rewrite required; biome category and precipitation constants; version-specific | Substring categories can misclassify modded/new names; exact biome registry ordering, legacy replacements, categories and temperature rule must come from 1.20.1 data. |
| 2040 | `crates/horizon-shaders/src/block_ids.rs` | adaptable; block state and entity/item ID rules; optional shaderpack extension | No identifier or ID-range validation; malformed/empty filters can be accepted; `%tag` expansion is absent and note wording is block-specific for all kinds. |
| 2041 | `crates/horizon-shaders/src/condition.rs` | adaptable; GLSL #if evaluator; optional shaderpack extension | No short-circuit for &&///; macro-expanded float values can be evaluated with integer division; #ifdef/#ifndef ignores extra tokens and can accept a missing name. Integer precision/overflow also diverges from C. |
| 2042 | `crates/horizon-shaders/src/config.rs` | adaptable; render-target and shadow config parser; renderer-specific | Shadow settings do not consistently validate declared GLSL types, non-finite/range values can pass silently, and DRAWBUFFERS accepts out-of-range single digits 8/9. |
| 2043 | `crates/horizon-shaders/src/core_shader.rs` | rewrite required; vanilla core GLSL translation; Minecraft 26.3-specific | Hardcoded 26.3 uniform blocks and samplers; angle-bracket include paths are not traversal-normalized; fragment/vertex interfaces compare locations but not types/interpolation; line-oriented declaration parsing omits complex GLSL forms. |
| 2044 | `crates/horizon-shaders/src/custom.rs` | adaptable; custom expression evaluator; optional shaderpack extension | `if`/`ifb` eagerly evaluate every branch, `in()` with no arguments can panic, Val::vector accepts unchecked component counts, and expression depth/argument counts are uncapped. |
| 2045 | `crates/horizon-shaders/src/discover.rs` | adaptable; shader-pack discovery; host-only | read_dir errors are silently converted to an empty result; probing every ZIP reads the full archive into memory; symlink folders/ZIPs can point outside the selected directory. |
| 2046 | `crates/horizon-shaders/src/lang.rs` | adaptable; localization fallback utility; portable | Simple key=value parsing has no escape/continuation handling and duplicate keys silently use the last value; this is not Minecraft's general resource-language system. |
| 2047 | `crates/horizon-shaders/src/lib.rs` | reference; shaderpack crate public API map; module-boundary only | The crate documentation states it knows nothing of wgpu and targets a Horizon clean-room shader-pack specification; this export surface is not a Minecraft client API. |
| 2048 | `crates/horizon-shaders/src/macros.rs` | adaptable; GLSL macro compatibility layer; optional shaderpack extension | Expansion depth/output are unbounded; function macro arity is not validated; supported features named only as required are not defined as IRIS_FEATURE_*; defaults hardcode MC 26.3/Iris 1.11.7. |
| 2049 | `crates/horizon-shaders/src/options.rs` | adaptable; shaderpack option discovery and apply; optional shaderpack extension | switches_on misses switches tested only via `#if defined`; option_texts follows only one include level; public apply accepts arbitrary unvalidated replacement strings, while only Pack::with_chosen filters values. |
| 2050 | `crates/horizon-shaders/src/pack.rs` | rewrite required; pack folder/ZIP resource reader; security-sensitive | ZIP input and decompressed entry sizes are unbounded; folder reads follow symlinks beyond the pack root; recursive file enumeration has no file/depth caps. Lexical path normalization alone does not prevent symlink escape. |

| 2051 | `crates/horizon-shaders/src/post.rs` | rewrite required; Horizon shaderpack render-pass manager; renderer-specific | Condition evaluation errors are fail-open (treated as enabled). Buffer/image extents and relative scales lack complete finite/product/device-limit checks; image names and buffer indices are weakly validated. Geometry-stage settings appear incompletely consulted. Panic capture can hide internal defects. |
| 2052 | `crates/horizon-shaders/src/preprocess.rs` | adaptable; GLSL preprocessor and origin tracking; optional shaderpack subsystem | Depth is capped but aggregate include fan-out/output size is not; source IDs cast to u16 unchecked. Unclosed comments and malformed #version can be silently accepted; include delimiter grammar is loose; line joining may skew source origins. |
| 2053 | `crates/horizon-shaders/src/properties.rs` | adaptable; shaderpack properties parser; optional extension | Input bytes, line/entry counts and continuation length are uncapped; u32 line counting is unchecked; duplicate vector replacement can become quadratic; function-like property macros are not modeled. |
| 2054 | `crates/horizon-shaders/src/screens.rs` | adaptable; shader settings screen schema; optional extension | Columns permit zero/huge values; sliders are not fully type-checked; profile values are not verified against allowed option values; missing screen links and total item counts need validation/caps. |
| 2055 | `crates/horizon-shaders/src/textures.rs` | rewrite required; shaderpack texture and image loader; security-sensitive | Decoded PNG allocation has no explicit aggregate memory cap; raw 3D dimension products may be huge; multiplications need checked arithmetic. Dynamic/atlas prefixes bypass normal path validation; mcmeta flags use brittle substring searches. |
| 2056 | `crates/horizon-shaders/src/translate/anonymous_structs.rs` | adaptable; anonymous GLSL struct naming; optional translator helper | Pattern-based rewrite rather than GLSL AST; unusual declarators, nested structures and generated-name collisions need broader tests. |
| 2057 | `crates/horizon-shaders/src/translate/atomic_vectors.rs` | adaptable; shared-vector atomic lowering; optional GLSL compute translator | First-argument resource detection is shallow; declaration shape and component bounds are weakly validated; generated helper names may collide; malformed missing-semicolon cases can produce bad rewrites. |
| 2058 | `crates/horizon-shaders/src/translate/geometry.rs` | rewrite required; geometry-stage emulation; renderer/backend-specific | Relies on exact unindexed triangle draws and instance semantics; geometry output arrays are explicitly unsupported; max_vertices expects a simple literal. Token-based write gating and generated u16 file-offset mapping need stronger checks. |
| 2059 | `crates/horizon-shaders/src/translate/image_parameters.rs` | adaptable; storage-image parameter specialization; optional translator helper | Overloads selected by name/argument count only; ambiguous overloads fail. Image handles must be a single identifier; dynamic expressions unsupported. A 256-specialization cap exists, but recursion/cycle and resource-identity handling need tests. |
| 2060 | `crates/horizon-shaders/src/translate/integer_bounds.rs` | adaptable; targeted signed/unsigned clamp disambiguation; optional translator helper | Type discovery is lexical/global, not scope-aware; same-name declarations suppress rewriting; only one literal call shape is covered. |
| 2061 | `crates/horizon-shaders/src/translate/interface_blocks.rs` | adaptable; GLSL interface-block flattening; optional translator helper | Pattern-based block/member parser and scope inference; generated names can collide; whole-instance uses remain unflattened; cross-stage type/location/interpolation matching is deferred. |
| 2062 | `crates/horizon-shaders/src/translate/mod.rs` | rewrite required; GLSL-to-Naga shader translator; renderer-specific | Per-stage Naga validation is not full linked-pipeline ABI validation; varying type/interpolation and sampler conflict checks are incomplete. Generated array products need caps; workgroup values use spec minima not queried device limits; numeric casts and AlphaTest parsing lack strict finite/range/grammar checks. |
| 2063 | `crates/horizon-shaders/src/translate/ordering.rs` | adaptable; compute image ordering and barrier rewrite; optional translator pass | Not a formal control-flow/call-graph or memory-model analysis; helper recursion/complex scopes may be missed. Any early return in hz_main suppresses all barrier insertion. |
| 2064 | `crates/horizon-shaders/src/translate/rewrite.rs` | rewrite required; GLSL source compatibility rewrite; renderer-specific | Token-level rewriting rather than full AST; depends on inferred scopes/declaration shapes, generated symbols and limited compatibility substitutions; source/token sizes uncapped. |
| 2065 | `crates/horizon-shaders/src/translate/scan.rs` | rewrite required; GLSL stage declaration scanner; optional compiler front end | Token-shape grammar is incomplete; is_ident validates only the first char; SSBO binding/declarator parsing and alpha-cut inference are heuristic; no resource-count caps. |
| 2066 | `crates/horizon-shaders/src/translate/shadowed_types.rs` | adaptable; struct-name shadowing rewrite; optional translator helper | Global token heuristic lacks robust scope modelling; same identifiers in other functions/nested scopes may be renamed incorrectly; identifier validation is weak. |
| 2067 | `crates/horizon-shaders/src/values.rs` | rewrite required; shader option values and profiles; input-validation sensitive | Public set/parse_list accepts arbitrary raw strings and apply inserts values into source without option-domain validation; newline injection possible. Inheritance has depth cap but no visited set/breadth cap; NaN/inf caps and narrow parser are concerns. |
| 2068 | `crates/horizon-shaders/tests/packs/post-test/README.md` | reference; Horizon post-processing test README; test fixture only | Explicitly says it is written for Horizon, not another pack's code; it is not third-party compatibility evidence. |
| 2069 | `crates/horizon-shaders/tests/packs/post-test/shaders/composite.fsh` | reference; Horizon composite fragment fixture; test-only | Horizon-only; projection/depth/far and DRAWBUFFERS contracts are renderer-specific. |
| 2070 | `crates/horizon-shaders/tests/packs/post-test/shaders/composite.vsh` | reference; Horizon fullscreen vertex fixture; test-only | Depends on compatibility built-ins and Horizon quad/matrix state; draw geometry and winding are not specified here. |
| 2071 | `crates/horizon-shaders/tests/packs/post-test/shaders/composite1.fsh` | reference; Horizon second composite fixture; test-only | Horizon-specific uniforms; fixed 256 noise scale may behave differently by resolution; GLSL 330 compatibility. |
| 2072 | `crates/horizon-shaders/tests/packs/post-test/shaders/composite1.vsh` | reference; Horizon composite1 vertex fixture; test-only | Different UV path than composite.vsh without explaining why; runtime conventions decide correctness. |
| 2073 | `crates/horizon-shaders/tests/packs/post-test/shaders/final.fsh` | reference; Horizon final vignette fragment fixture; test-only | Does not clamp the factor; if UV domain differs, color may become negative; legacy GLSL 1.20. |
| 2074 | `crates/horizon-shaders/tests/packs/post-test/shaders/final.vsh` | reference; Horizon final vertex fixture; test-only | Relies on Horizon matrix state; confirm identity/intended transform for this pass. |
| 2075 | `crates/horizon-shaders/tests/packs/post-test/shaders/shaders.properties` | reference; Horizon shader pass property fixture; test-only | Only a two-line fixture; does not test malformed directives, nested includes or profile resolution. |

| 2076 | `crates/horizon-shaders/tests/packs/terrain-test/README.md` | reference; Horizon terrain shaderpack README; test fixture only | Explicitly identifies the pack as Horizon's own code; not third-party compatibility evidence. |
| 2077 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/block.properties` | reference; Horizon block-ID fixture; test-only | Static Horizon fixture mapping, not a stable vanilla numeric registry contract. |
| 2078 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/composite.fsh` | reference; Horizon composite fragment fixture; test-only | Projection conventions, homogeneous division and output attachment are Horizon-specific. |
| 2079 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/composite.vsh` | reference; Horizon fullscreen vertex fixture; test-only | Requires Horizon compatibility-profile built-ins and quad state. |
| 2080 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/gbuffers_skybasic.fsh` | reference; Horizon sky-basic fragment fixture; test-only | Injected stage constants and vertex-color contract are Horizon-specific. |
| 2081 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/gbuffers_skybasic.vsh` | reference; Horizon sky-basic vertex fixture; test-only | Depends on Horizon fixed-function compatibility inputs. |
| 2082 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/gbuffers_terrain.fsh` | reference; Horizon terrain fragment fixture; test-only | Numeric block ID 1, atlas coordinates and Horizon stage/attribute semantics are test-only assumptions. |
| 2083 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/gbuffers_terrain.vsh` | reference; Horizon terrain vertex fixture; test-only | Relies on Horizon vertex attributes and compatibility built-ins; defaults must be defined by renderer. |
| 2084 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/gbuffers_textured.fsh` | reference; Horizon textured fragment fixture; test-only | Horizon render-stage constant and blend behavior are fixture-specific. |
| 2085 | `crates/horizon-shaders/tests/packs/terrain-test/shaders/gbuffers_textured.vsh` | reference; Horizon textured vertex fixture; test-only | Fixed-function inputs are supplied by Horizon renderer. |
| 2086 | `crates/horizon-shaders/tests/real_packs.rs` | reference; ignored real shaderpack compatibility harness; test-only | Marked ignored; external input, no timeout/output cap, read_dir errors flattened, origin indexing trusted. Synthetic GameInputs do not prove real draw ABI or MobileGlues. |
| 2087 | `crates/horizon-telemetry-server/.gitignore` | reference; telemetry server gitignore; hygiene | No further generated-file exclusions are declared here. |
| 2088 | `crates/horizon-telemetry-server/Cargo.lock` | reference; telemetry server lockfile; host-only | A lockfile is provenance, not an advisory/license audit; dependency set is host service, not Android client. |
| 2089 | `crates/horizon-telemetry-server/Cargo.toml` | reference; standalone telemetry server manifest; host service | Rust 1.90, edition 2024 and bundled SQLite/native dependency footprint are host-service decisions, not Android client requirements. |
| 2090 | `crates/horizon-telemetry-server/Dockerfile` | rewrite required; telemetry Docker runtime hardening; host-only | No USER specified, so runtime likely root; floating image tags. TRUST_PROXY=1 only safe behind trusted ingress. |
| 2091 | `crates/horizon-telemetry-server/README.md` | reference; opt-in telemetry server documentation; service policy | IP not stored persistently but used in-memory for rate limiting; rich hardware details and report text persist for documented periods. Token/proxy configuration governs sensitive surfaces; README isn't proof of deployed behavior. |
| 2092 | `crates/horizon-telemetry-server/railway.json` | reference; Railway deployment configuration; host service | SQLite file motivates one replica; multi-instance scaling needs different storage/coordination; provider settings are external. |
| 2093 | `crates/horizon-telemetry-server/src/classes.rs` | adaptable; coarse hardware classification; optional telemetry | Infinity enters highest bucket; unknown string values become absent. Buckets are policy categories, not precise hardware facts. |
| 2094 | `crates/horizon-telemetry-server/src/export.rs` | rewrite required; protected telemetry export; sensitive data surface | Hardware export materializes all matching machine aggregates before offset/limit pagination, risking memory spikes; offset addition needs checked math; raw hardware report is privileged data. |
| 2095 | `crates/horizon-telemetry-server/src/github.rs` | adaptable; sanitized GitHub issue intake; backend-only | Bespoke text sanitization needs adversarial Unicode/Markdown tests; response body has no explicit cap; repo config and token require strict trust. |
| 2096 | `crates/horizon-telemetry-server/src/history.rs` | rewrite required; telemetry history and rollups; unbounded history | u64 casts to i64 unchecked; daily table retained forever and Range::All can grow unbounded; response not capped. |
| 2097 | `crates/horizon-telemetry-server/src/hub.rs` | reference; roadmap voting; backend-only | Voter is client-generated and only shape-checked, not authenticated; IDs can be minted. Missing/non-bool up defaults true; IDs not checked against roadmap catalogue. |
| 2098 | `crates/horizon-telemetry-server/src/intake.rs` | adaptable; bounded report intake queue; support-service only | Immediate handler and background retry can race and file duplicate issues; no atomic claim/lease/idempotency; ambiguous GitHub success can cause retry duplicate. |
| 2099 | `crates/horizon-telemetry-server/src/lib.rs` | rewrite required; telemetry API server and shared state; standalone service | Synchronous SQLite under global Mutex inside async handlers may block Tokio workers and serialize load. TRUST_PROXY trusts X-Real-IP/last XFF when enabled; direct exposure permits spoofing. Env rates/ranges lack full validation; invalid CORS origin falls back to wildcard. |
| 2100 | `crates/horizon-telemetry-server/src/live.rs` | adaptable; in-memory telemetry live counter; optional service | Malformed hex session ID silently returns success without storing; expiry can scan whole map when full; data resets on restart by design. |

| 2101 | `crates/horizon-telemetry-server/src/main.rs` | rewrite required; standalone telemetry server entry point; host-only | Binds all interfaces at 0.0.0.0 with no configurable bind host; deployment must provide trusted ingress/firewall. Synchronous SQLite maintenance and record_live execute directly inside async tasks, potentially blocking the two Tokio workers. Spawned recorder/upkeep tasks are not explicitly stopped or joined on shutdown. |
| 2102 | `crates/horizon-telemetry-server/src/store.rs` | rewrite required; telemetry persistence, aggregation and migrations; host-only | Benchmark uploads and hardware export first materialize all matching rows/results in memory; large retained datasets can create memory spikes. unpack checks blob length but does not reject non-finite floats. Migrations have no schema-version table and perf-column ALTER precedes the separate systems-table transaction, so a failed migration can leave a partially advanced schema. Some malformed stored zstd/JSON bodies silently become JSON null. |
| 2103 | `crates/horizon-telemetry-server/src/summary.rs` | rewrite required; benchmark comparability and aggregation rules; optional telemetry | Validation is permissive: absent mods on a minecraft report counts as vanilla; phase presence does not require exactly one of each scene; any numeric render distance passes then is cast f64 to i64, truncating fractions and saturating out-of-range values. Reports/metrics are not fully schema-validated before grouping. |
| 2104 | `crates/horizon-telemetry-server/tests/export.rs` | reference; protected telemetry export HTTP tests; test-only | Covers only small in-memory fixtures and successful/invalid token paths; does not stress unbounded hardware aggregation, huge responses, concurrent deletion/export, token rotation, malformed headers or timing properties. |
| 2105 | `crates/horizon-telemetry-server/tests/fixtures/benchmarks/README.md` | reference; benchmark fixture README; test documentation | Fixture copy must be changed in sync with the separate website repository; synthetic identifiers/metrics do not prove real-world benchmark comparability. |
| 2106 | `crates/horizon-telemetry-server/tests/fixtures/benchmarks/expected.json` | reference; benchmark golden output; test-only | One deterministic golden sample covers only a narrow valid result; it does not cover edge rounding, ties, invalid numbers, duplicates, missing metadata or version sort anomalies. |
| 2107 | `crates/horizon-telemetry-server/tests/fixtures/benchmarks/items.jsonl` | reference; benchmark item fixture corpus; test-only | Records are synthetic; only one primary valid GPU group meets threshold. Secret-like fields are sentinel values and demonstrate redaction checks, not actual private data. Fixture does not represent Forge/mod-heavy workloads. |
| 2108 | `crates/horizon-telemetry-server/tests/github.rs` | reference; GitHub report neutralization tests; test-only | Only a handful of crafted strings; Unicode normalization, nested Markdown, encoded URLs, control characters, long text and actual GitHub rendering are not covered. |
| 2109 | `crates/horizon-telemetry-server/tests/hub.rs` | reference; hub intake and retry tests; test-only | Tests are sequential; no concurrent duplicate POST or concurrent file_pending/POST race is tested, nor crash after GitHub accepted but before filed state persisted. Voter identity is self-generated and shape-only. |
| 2110 | `crates/horizon-telemetry-server/tests/live.rs` | reference; live endpoint integration tests; test-only | Tests validate the telemetry protocol only; the client/server sessions are not Minecraft multiplayer sessions. Fixed sleeps/timeouts can be timing-sensitive and proxy trust is set true in test config. |
| 2111 | `crates/horizon-telemetry-server/tests/server.rs` | reference; telemetry server integration tests; test-only | Coverage is broad but still sequential/synthetic; no concurrent daily-cap race, corrupted DB migration, malicious stored payload, server overload, real Android upload or network retry-under-process-death scenario is tested. |
| 2112 | `crates/horizon-telemetry-server/tests/summary.rs` | reference; benchmark summary tests; test-only | Golden corpus is narrow and synthetic; no exact-one-phase rule, duplicate phase, non-finite/huge numeric, semver edge ordering or Forge mod identity tests. |
| 2113 | `crates/horizon-telemetry/Cargo.toml` | adaptable; telemetry crate feature separation; optional dependency design | Default feature set turns on both upload and live ping support, and the latter is explicitly not gated by per-kind consent. Ureq TLS/native dependency and getrandom impact need Android ABI/footprint audit. |
| 2114 | `crates/horizon-telemetry/src/batch.rs` | adaptable; bounded telemetry batch codec; optional library | Batch::json and split convert serialization errors into empty/zero-size fallback instead of propagating them; split sizing is approximate with a 1 KiB safety margin. Decoder does not explicitly reject trailing/concatenated compressed frames. JSON payload validation remains shallow for nested numeric/hardware fields. |
| 2115 | `crates/horizon-telemetry/src/consent.rs` | adaptable; per-kind consent and pseudonymous install lifecycle; optional library | Install ID is retained when all choices are turned off for later deletion; this is deliberate but requires clear UX. Save errors surface to caller, but fallback random source in crate root is weak if OS RNG fails. |
| 2116 | `crates/horizon-telemetry/src/lib.rs` | rewrite required; shared telemetry control and persistence primitives; security-sensitive | random_hex falls back to repeated bytes derived from current nanoseconds when getrandom fails, so install/session IDs and temporary names are predictable and can collide. Endpoint override accepts arbitrary scheme/URL without validation. write_atomic has no explicit permission restriction or fsync durability. |
| 2117 | `crates/horizon-telemetry/src/live.rs` | rewrite required; live presence ping and count client; privacy-sensitive optional feature | The source explicitly says live pings do not wait for consent, unlike every other telemetry kind. Session entropy inherits the predictable random_hex fallback. Endpoint may be plaintext HTTP when overridden; no server authentication or certificate pinning beyond ureq defaults. |
| 2118 | `crates/horizon-telemetry/src/outbox.rs` | rewrite required; shared telemetry outbox; concurrency and local-path sensitive | Game and launcher share a read-modify-write outbox and kept.json without cross-process lock/atomic counter transaction; concurrent pushes can exceed caps or collide. Directory scanning silently treats read errors as empty; it follows file metadata/read paths and has no symlink containment check. External oversized/corrupt files are read whole before validation/removal. |
| 2119 | `crates/horizon-telemetry/src/payload.rs` | rewrite required; telemetry payload schema and timing aggregation; privacy-sensitive | Validation is incomplete: non-finite/implausible Display.scale, timing summaries and SlowFrames numeric values are not rejected; benchmark report checks only object shape; System strings have no field-specific length/range rules beyond total item bytes. Timings count is u64 and increments unchecked while bucket counters saturate. |
| 2120 | `crates/horizon-telemetry/src/scrub.rs` | rewrite required; heuristic telemetry text scrubber; privacy-critical | Heuristic scanning cannot guarantee removal of all personal data; IP/UUID parsing is not a full standards parser and false positives can alter version/build strings. The scrubber is a caller-invoked utility, not enforced by Item::validate/outbox, so every producer must consistently call it. |
| 2121 | `crates/horizon-telemetry/src/server_status.rs` | adaptable; bounded Minecraft status-ping client; protocol-version-specific | Only first DNS result is attempted; DNS lookup itself has no timeout and TCP timeout is per connect/read/write, not an overall deadline. IPv6 and malformed host validation is permissive. Handshake protocol constant 777 is Horizon 26.3, even though status servers often tolerate it. |
| 2122 | `crates/horizon-telemetry/src/system.rs` | rewrite required; cross-platform hardware detection; privacy- and platform-sensitive | Several commands have no timeout/output cap and may block startup. Linux non-NVIDIA VRAM scanning ignores requested vendor and may choose another adapter's largest memory. Some usize/u64 counts cast to u16/u32 without range checks; mountinfo escaping/spaces and complex storage stacks may misclassify; full OS/CPU/GPU/driver/display data is more identifying than coarse hardware categories. |
| 2123 | `crates/horizon-telemetry/src/upload.rs` | rewrite required; consent-aware network uploader; privacy and retry sensitive | Consent is checked before the loop but not coordinated with a simultaneous mid-flush withdrawal; state save errors are silently ignored, so retry counters may not persist. Endpoint override is not scheme-validated and a 400/401/413 permanently deletes the batch. retain_allowed and initial consent loads are separate reads. |
| 2124 | `crates/horizon-telemetry/tests/consent_gating.rs` | reference; telemetry consent gating tests; test-only | Tests do not cover in-flight withdrawal during a multi-batch flush, concurrent producers, DNS/TLS/redirect behavior, permission failures or the separately documented no-consent live ping. |
| 2125 | `crates/horizon-theme/Cargo.toml` | unnecessary; empty theme crate manifest for RustChunk core | Manifest alone proves only a package boundary, not token definitions, font/icon licensing, runtime memory cost or Android asset integration. |

| 2126 | `crates/horizon-theme/fonts/BricolageGrotesque-Bold.ttf` | reference; optional Latin display/body typography; no Arabic/Persian support | No Arabic/Persian cmap coverage; shaping scripts DFLT/latn only. No malformed outline, out-of-bounds table or checksum mismatch. Bricolage's license says Cyrillic glyph material is merged from Geist, so both attributions matter. |
| 2127 | `crates/horizon-theme/fonts/BricolageGrotesque-Medium.ttf` | reference; optional Latin display/body typography; no Arabic/Persian support | No Arabic/Persian cmap coverage; shaping scripts DFLT/latn only. No malformed outline, out-of-bounds table or checksum mismatch. Bricolage's license says Cyrillic glyph material is merged from Geist, so both attributions matter. |
| 2128 | `crates/horizon-theme/fonts/BricolageGrotesque-Regular.ttf` | reference; optional Latin display/body typography; no Arabic/Persian support | No Arabic/Persian cmap coverage; shaping scripts DFLT/latn only. No malformed outline, out-of-bounds table or checksum mismatch. Bricolage's license says Cyrillic glyph material is merged from Geist, so both attributions matter. |
| 2129 | `crates/horizon-theme/fonts/BricolageGrotesque-SemiBold.ttf` | reference; optional Latin display/body typography; no Arabic/Persian support | No Arabic/Persian cmap coverage; shaping scripts DFLT/latn only. No malformed outline, out-of-bounds table or checksum mismatch. Bricolage's license says Cyrillic glyph material is merged from Geist, so both attributions matter. |
| 2130 | `crates/horizon-theme/fonts/BricolageGrotesqueDisplay-Bold.ttf` | reference; optional Latin display/body typography; no Arabic/Persian support | No Arabic/Persian cmap coverage; shaping scripts DFLT/latn only. No malformed outline, out-of-bounds table or checksum mismatch. Bricolage's license says Cyrillic glyph material is merged from Geist, so both attributions matter. |
| 2131 | `crates/horizon-theme/fonts/BricolageGrotesqueDisplay-SemiBold.ttf` | reference; optional Latin display/body typography; no Arabic/Persian support | No Arabic/Persian cmap coverage; shaping scripts DFLT/latn only. No malformed outline, out-of-bounds table or checksum mismatch. Bricolage's license says Cyrillic glyph material is merged from Geist, so both attributions matter. |
| 2132 | `crates/horizon-theme/fonts/BricolageGrotesqueHeading-SemiBold.ttf` | reference; optional Latin display/body typography; no Arabic/Persian support | No Arabic/Persian cmap coverage; shaping scripts DFLT/latn only. No malformed outline, out-of-bounds table or checksum mismatch. Bricolage's license says Cyrillic glyph material is merged from Geist, so both attributions matter. |
| 2133 | `crates/horizon-theme/fonts/Geist-Medium.ttf` | adaptable; Latin/Greek/Cyrillic interface typography; no Arabic/Persian support | No Arabic/Persian coverage. Greek/Cyrillic shaping support does not imply Arabic-script shaping. No invalid outlines or checksum errors. |
| 2134 | `crates/horizon-theme/fonts/Geist-Regular.ttf` | adaptable; Latin/Greek/Cyrillic interface typography; no Arabic/Persian support | No Arabic/Persian coverage. Greek/Cyrillic shaping support does not imply Arabic-script shaping. No invalid outlines or checksum errors. |
| 2135 | `crates/horizon-theme/fonts/Geist-SemiBold.ttf` | adaptable; Latin/Greek/Cyrillic interface typography; no Arabic/Persian support | No Arabic/Persian coverage. Greek/Cyrillic shaping support does not imply Arabic-script shaping. No invalid outlines or checksum errors. |
| 2136 | `crates/horizon-theme/fonts/GeistMono-Regular.ttf` | adaptable; fixed-pitch Latin code/log typography; no Arabic/Persian support | No Arabic/Persian. Fixed-pitch flag corroborated by actual hmtx mapping; RTL code/log behavior and lack of kern feature require runtime tests. |
| 2137 | `crates/horizon-theme/fonts/OFL-BricolageGrotesque.txt` | essential; preserve SIL OFL 1.1 notices for bundled font assets | Fonts can be bundled/embedded with software when every copy carries the copyright and license notice; fonts cannot be sold by themselves; modified font software remains under OFL and reserved names cannot be reused without permission. Bricolage includes Cyrillic glyphs attributed to Geist. |
| 2138 | `crates/horizon-theme/fonts/OFL-Geist.txt` | essential; preserve SIL OFL 1.1 notices for bundled font assets | Fonts can be bundled/embedded with software when every copy carries the copyright and license notice; fonts cannot be sold by themselves; modified font software remains under OFL and reserved names cannot be reused without permission. Bricolage includes Cyrillic glyphs attributed to Geist. |
| 2139 | `crates/horizon-theme/fonts/OFL-PixelifySans.txt` | essential; preserve SIL OFL 1.1 notices for bundled font assets | Fonts can be bundled/embedded with software when every copy carries the copyright and license notice; fonts cannot be sold by themselves; modified font software remains under OFL and reserved names cannot be reused without permission. Bricolage includes Cyrillic glyphs attributed to Geist. |
| 2140 | `crates/horizon-theme/fonts/PixelifySans-Bold.ttf` | reference; decorative Pixelify pixel typography; no Arabic/Persian support | No Arabic/Persian mappings. No malformed outline/checksum issues. Pixel design is decorative and may reduce small-text readability. |
| 2141 | `crates/horizon-theme/fonts/PixelifySans-Medium.ttf` | reference; decorative Pixelify pixel typography; no Arabic/Persian support | No Arabic/Persian mappings. No malformed outline/checksum issues. Pixel design is decorative and may reduce small-text readability. |
| 2142 | `crates/horizon-theme/fonts/PixelifySans-SemiBold.ttf` | reference; decorative Pixelify pixel typography; no Arabic/Persian support | No Arabic/Persian mappings. No malformed outline/checksum issues. Pixel design is decorative and may reduce small-text readability. |
| 2143 | `crates/horizon-theme/icons/glass/LICENSE-hugeicons.txt` | essential; preserve MIT attribution for Hugeicons SVG assets | Notice must accompany copies/substantial portions. This license file's scope is the reviewed Hugeicons glass subset, not every icon family in the repository. |
| 2144 | `crates/horizon-theme/icons/glass/ai-chat-02.svg` | adaptable; compact currentColor SVG interface icon; preserve MIT attribution | No script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external reference found. No Android renderer/conversion tested; icon contains no interaction or accessibility label. |
| 2145 | `crates/horizon-theme/icons/glass/arrow-down-01.svg` | adaptable; compact currentColor SVG interface icon; preserve MIT attribution | No script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external reference found. No Android renderer/conversion tested; icon contains no interaction or accessibility label. |
| 2146 | `crates/horizon-theme/icons/glass/arrow-right-01.svg` | adaptable; compact currentColor SVG interface icon; preserve MIT attribution | No script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external reference found. No Android renderer/conversion tested; icon contains no interaction or accessibility label. |
| 2147 | `crates/horizon-theme/icons/glass/cancel-01.svg` | adaptable; compact currentColor SVG interface icon; preserve MIT attribution | No script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external reference found. No Android renderer/conversion tested; icon contains no interaction or accessibility label. |
| 2148 | `crates/horizon-theme/icons/glass/color-picker.svg` | adaptable; compact currentColor SVG interface icon; preserve MIT attribution | No script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external reference found. No Android renderer/conversion tested; icon contains no interaction or accessibility label. |
| 2149 | `crates/horizon-theme/icons/glass/computer-terminal-01.svg` | adaptable; compact currentColor SVG interface icon; preserve MIT attribution | No script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external reference found. No Android renderer/conversion tested; icon contains no interaction or accessibility label. |
| 2150 | `crates/horizon-theme/icons/glass/computer.svg` | adaptable; compact currentColor SVG interface icon; preserve MIT attribution | No script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external reference found. No Android renderer/conversion tested; icon contains no interaction or accessibility label. |

| 2151 | `crates/horizon-theme/icons/glass/copy-01.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2152 | `crates/horizon-theme/icons/glass/dashboard-square-01.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2153 | `crates/horizon-theme/icons/glass/delete-02.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2154 | `crates/horizon-theme/icons/glass/discord.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2155 | `crates/horizon-theme/icons/glass/download-04.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2156 | `crates/horizon-theme/icons/glass/earth.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2157 | `crates/horizon-theme/icons/glass/favourite.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2158 | `crates/horizon-theme/icons/glass/folder-01.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2159 | `crates/horizon-theme/icons/glass/game-controller-01.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2160 | `crates/horizon-theme/icons/glass/globe-02.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2161 | `crates/horizon-theme/icons/glass/image-01.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2162 | `crates/horizon-theme/icons/glass/information-circle.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2163 | `crates/horizon-theme/icons/glass/link-01.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2164 | `crates/horizon-theme/icons/glass/minus-sign.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2165 | `crates/horizon-theme/icons/glass/more-horizontal.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2166 | `crates/horizon-theme/icons/glass/more-vertical.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2167 | `crates/horizon-theme/icons/glass/package-01.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2168 | `crates/horizon-theme/icons/glass/pencil-edit-02.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2169 | `crates/horizon-theme/icons/glass/play.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2170 | `crates/horizon-theme/icons/glass/plus-sign.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2171 | `crates/horizon-theme/icons/glass/puzzle.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2172 | `crates/horizon-theme/icons/glass/refresh.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2173 | `crates/horizon-theme/icons/glass/search-01.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2174 | `crates/horizon-theme/icons/glass/sliders-horizontal.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |
| 2175 | `crates/horizon-theme/icons/glass/stop.svg` | adaptable; compact Hugeicons SVG interface icon; preserve MIT attribution | Complete content scan found no script, foreignObject, iframe, image, use, href/xlink:href or CSS url() external resource. XML source is self-contained; rendering/conversion itself was not tested. |

| 2176 | `crates/horizon-theme/icons/glass/tick-02.svg` | adaptable; Hugeicons Glass SVG UI icon; preserve the separate Hugeicons MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Android rendering/conversion was not tested. |
| 2177 | `crates/horizon-theme/icons/glass/upload-04.svg` | adaptable; Hugeicons Glass SVG UI icon; preserve the separate Hugeicons MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Android rendering/conversion was not tested. |
| 2178 | `crates/horizon-theme/icons/glass/user-add-01.svg` | adaptable; Hugeicons Glass SVG UI icon; preserve the separate Hugeicons MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Android rendering/conversion was not tested. |
| 2179 | `crates/horizon-theme/icons/glass/user-multiple-02.svg` | adaptable; Hugeicons Glass SVG UI icon; preserve the separate Hugeicons MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Android rendering/conversion was not tested. |
| 2180 | `crates/horizon-theme/icons/glass/user.svg` | adaptable; Hugeicons Glass SVG UI icon; preserve the separate Hugeicons MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Android rendering/conversion was not tested. |
| 2181 | `crates/horizon-theme/icons/glass/view-off-slash.svg` | adaptable; Hugeicons Glass SVG UI icon; preserve the separate Hugeicons MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Android rendering/conversion was not tested. |
| 2182 | `crates/horizon-theme/icons/glass/view.svg` | adaptable; Hugeicons Glass SVG UI icon; preserve the separate Hugeicons MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Android rendering/conversion was not tested. |
| 2183 | `crates/horizon-theme/icons/horizon-64.png` | reference; Horizon 64px bitmap branding asset; verify ownership before redistribution | Binary structure and CRC integrity are valid; image preview was checked. Ownership and redistribution permission are not established by the adjacent icon notices. |
| 2184 | `crates/horizon-theme/icons/pixel/LICENSE-pixelarticons.txt` | essential; preserve Pixelarticons MIT copyright and permission notice | Pixelarticons uses a separate MIT notice and copyright holder from Hugeicons. The Hugeicons/Glass notice must not replace this license. |
| 2185 | `crates/horizon-theme/icons/pixel/archive.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2186 | `crates/horizon-theme/icons/pixel/arrow-left.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2187 | `crates/horizon-theme/icons/pixel/book-open.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2188 | `crates/horizon-theme/icons/pixel/box.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2189 | `crates/horizon-theme/icons/pixel/bug.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2190 | `crates/horizon-theme/icons/pixel/check.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2191 | `crates/horizon-theme/icons/pixel/chevron-down.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2192 | `crates/horizon-theme/icons/pixel/chevron-right.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2193 | `crates/horizon-theme/icons/pixel/clock.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2194 | `crates/horizon-theme/icons/pixel/close.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2195 | `crates/horizon-theme/icons/pixel/download.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2196 | `crates/horizon-theme/icons/pixel/external-link.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2197 | `crates/horizon-theme/icons/pixel/folder.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2198 | `crates/horizon-theme/icons/pixel/gamepad.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2199 | `crates/horizon-theme/icons/pixel/globe.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |
| 2200 | `crates/horizon-theme/icons/pixel/heart.svg` | adaptable; Pixelarticons pixel SVG UI glyph; preserve its distinct MIT notice | No script, foreignObject, iframe, embedded image/use, href/xlink:href or CSS url() reference found. Pixel-grid visual style is distinct from the Glass line-art set. |

| 2201 | `crates/horizon-theme/icons/pixel/home.svg` | adaptable; Pixelarticons home glyph; retain its separate MIT notice | One path using currentColor; static vector only, not a navigation action. |
| 2202 | `crates/horizon-theme/icons/pixel/image.svg` | adaptable; Pixelarticons image glyph; retain its separate MIT notice | Two static paths; no script, embedded image, link or external resource found. |
| 2203 | `crates/horizon-theme/icons/pixel/info-box.svg` | adaptable; Pixelarticons information glyph; retain its separate MIT notice | One compact currentColor path; no executable or external reference. |
| 2204 | `crates/horizon-theme/icons/pixel/message.svg` | adaptable; Pixelarticons message glyph; retain its separate MIT notice | One currentColor path; no external reference or message behavior. |
| 2205 | `crates/horizon-theme/icons/pixel/monitor.svg` | adaptable; Pixelarticons monitor glyph; retain its separate MIT notice | Static path geometry; no executable or external resource. |
| 2206 | `crates/horizon-theme/icons/pixel/notes.svg` | adaptable; Pixelarticons notes glyph; retain its separate MIT notice | Static currentColor paths; no document storage or external resource. |
| 2207 | `crates/horizon-theme/icons/pixel/package.svg` | adaptable; Pixelarticons package glyph; retain its separate MIT notice | Static vector only; no package-management action or external content. |
| 2208 | `crates/horizon-theme/icons/pixel/play.svg` | adaptable; Pixelarticons play glyph; retain its separate MIT notice | CurrentColor path only; does not launch or control a process. |
| 2209 | `crates/horizon-theme/icons/pixel/plus.svg` | adaptable; Pixelarticons plus glyph; retain its separate MIT notice | Static path; no action, state or external dependency. |
| 2210 | `crates/horizon-theme/icons/pixel/reload.svg` | adaptable; Pixelarticons reload glyph; retain its separate MIT notice | Static vector with currentColor; no network or retry behavior. |
| 2211 | `crates/horizon-theme/icons/pixel/search.svg` | adaptable; Pixelarticons search glyph; retain its separate MIT notice | Single currentColor path; no input or search logic. |
| 2212 | `crates/horizon-theme/icons/pixel/server.svg` | adaptable; Pixelarticons server glyph; retain its separate MIT notice | Only a static symbol; no endpoint or multiplayer behavior. |
| 2213 | `crates/horizon-theme/icons/pixel/sliders.svg` | adaptable; Pixelarticons sliders glyph; retain its separate MIT notice | Root declares viewBox and currentColor but omits explicit width and height; parent sizing is required. |
| 2214 | `crates/horizon-theme/icons/pixel/star.svg` | undetermined; custom Horizon star glyph; verify its project-level asset license | The crate registry labels this as not Pixelarticons and says it was drawn for Horizon ratings. Do not apply the neighboring Pixelarticons notice automatically; standalone asset licensing remains unverified. |
| 2215 | `crates/horizon-theme/icons/pixel/terminal.svg` | adaptable; Pixelarticons terminal glyph; retain its separate MIT notice | Static currentColor path only; no terminal execution behavior. |
| 2216 | `crates/horizon-theme/icons/pixel/trash.svg` | adaptable; Pixelarticons trash glyph; retain its separate MIT notice | Static path; second line begins at column zero but is still valid XML. No external reference. |
| 2217 | `crates/horizon-theme/icons/pixel/user.svg` | adaptable; Pixelarticons user glyph; retain its separate MIT notice | No account or identity data; static vector only. |
| 2218 | `crates/horizon-theme/icons/pixel/zap.svg` | adaptable; Pixelarticons zap glyph; retain its separate MIT notice | Static geometry only; no performance or power behavior. |
| 2219 | `crates/horizon-theme/src/glass.rs` | adaptable; palette and OKLab helpers only; validate public numeric inputs | Glass::new accepts unvalidated tint/blur/accent values; invalid or non-finite values can yield invalid CSS or colour math. on_accent uses a luminance >0.4 heuristic, not a general contrast guarantee. Chrome reference tests are present but unrun. |
| 2220 | `crates/horizon-theme/src/lib.rs` | adaptable; port selected token/registry patterns only; keep game semantics separate | 877 lines read in 1–400, 401–800 and 801–877. Public float-bearing values and bezier() have no finite/range validation; bisection assumes a valid monotone cubic-bezier x curve. Icon tests only search for viewBox/currentColor substrings rather than parse SVG. CSS-regeneration and WCAG tests exist but were not run. Previously reviewed fonts do not establish Persian/Arabic shaping coverage. |
| 2221 | `crates/horizon-theme/src/night.rs` | adaptable; launcher-only theme; validate alpha and keep game-menu separation | Public over() does not clamp/validate alpha; public Night fields can carry invalid alpha values, producing extrapolated colours or invalid CSS. Source documentation explicitly says game menus do not use this look. Tests unrun. |
| 2222 | `crates/horizon-theme/tokens.css` | adaptable; generated CSS artifact; Rust constants remain authoritative | File identifies itself as generated from the Rust crate and says do not edit. Repeated dark values under media and explicit data-theme selectors are intentional. Regeneration consistency test exists in lib.rs but was not run. |
| 2223 | `crates/horizon-threads/Cargo.toml` | adaptable; manifest layout only; resolve dependencies per target | Allocator/native dependencies are unconditional while OS APIs are target-gated. Android grouping does not prove NDK/ABI compatibility; mimalloc dependency unification and duplicate native-link behavior need checking. |
| 2224 | `crates/horizon-threads/src/boot_paint.rs` | rewrite required; keep only boot sequencing idea; replace GDI and harden resource/error handling | Windows GDI only. AddFontMemResourceEx handles/results and GetClientRect/FillRect/DrawTextW/CreateFontW failures are mostly ignored. Font byte lifetime is documented but not enforced by slice signature; log strings have no byte cap and are converted to UTF-16 on each paint. No build/paint test run. |
| 2225 | `crates/horizon-threads/src/heap_census.rs` | rewrite required; diagnostic-only concept; fix guard mobility, checked sizes, Android tags and self-measurement | Tag is automatically Send although Drop restores thread-local STEP, so cross-thread guard movement can restore the wrong TLS state. layout.size()+header and new_size+header use unchecked arithmetic. census() allocates Vec storage and can perturb its own readings when this allocator is active. Thread naming supports Apple/Linux but not Android, which falls back to unnamed thread. Registry has 512 slots; overflow is folded into untagged. No tests/allocator execution run. |

| 2226 | `crates/horizon-threads/src/jobs.rs` | rewrite required; scheduler ideas only after thread-lifecycle and failure semantics are corrected | Pool::new reports at least one worker via started.max(1) even when every OS thread spawn failed, potentially leaving queued work with zero actual workers. A slow/blocking source.next under registration synchronization can delay unregister; dropping a registration from its own job can deadlock. Requires concurrency/race tests around idle/wake and deregistration. |
| 2227 | `crates/horizon-threads/src/lib.rs` | adaptable; split pure lifecycle abstraction from OS backends; do not port process-control code wholesale | Unix getpriority handling needs errno-aware distinction for a valid -1 priority versus failure. Signal handlers replace prior handlers without restoring them; signal-safety assumptions for atomics and handler work need target verification. Windows junction/path conversions using lossy strings can mangle non-UTF paths. Child std-handle inheritance and Windows process behavior need platform tests. |
| 2228 | `crates/horizon-threads/src/memory_limit.rs` | adaptable; retain only a bounded, restartable memory-pressure interface; reimplement per Android target | Watcher startup uses Once; if initial thread spawn fails, Once is consumed and later attempts cannot restart enforcement. u64::MAX doubles as the no-limit sentinel while Some(u64::MAX) is representable, causing ambiguity. Platform paths and callback semantics need failure-injection tests. |
| 2229 | `crates/horizon-world/examples/attribute_memory.rs` | reference; profiling example only; do not ship in the client | The entity count is bounded, but positional arguments are indexed without arity checks and unknown attribute ids/entities are unwrapped; malformed invocation panics. This is an external profiling workload, not an allocator contract or game runtime. |
| 2230 | `crates/horizon-world/examples/behaviour_load_profile.rs` | reference; benchmark harness only; repeat with controlled Android/1.20.1 resources before drawing performance conclusions | Resource path is required with expect; the profile is a one-process timing sample and does not isolate disk cache, warm-up or device variance. It measures Horizon's resource loader, not Minecraft/Forge load time. |
| 2231 | `crates/horizon-world/examples/chunkgen_bench.rs` | reference; do not trust its pass result until EOF completeness and argument validation are fixed | Verifier can accept truncated output: after EOF it may leave still-active candidates marked as matched rather than requiring every expected line to be consumed. This is a correctness flaw in the benchmark's oracle comparison. CLI size/thread/coordinate inputs also lack robust bounds and may cause zero-work, overflow, panic or excessive allocations. |
| 2232 | `crates/horizon-world/examples/lod_grid_bench.rs` | reference; LOD benchmark idea only; validate parameters and do not treat numbers as Android or Minecraft client performance | tiles is not validated: zero leads to division by zero in per-tile metrics; negative values produce misleading/empty work. Level shifts and tile-coordinate multiplication can overflow or panic for invalid levels/large values. The benchmark's 5-column coordinate formula assumes a bounded tile layout. |
| 2233 | `crates/horizon-world/examples/loot_fill.rs` | reference; fixture/oracle utility only; do not ship | No cap on table/seed argument counts or generated output; missing positional arguments return usage errors but large input lists can create unbounded work/output. It is a CLI oracle, not loot implementation integration evidence. |
| 2234 | `crates/horizon-world/examples/probe/active_effect_tick_fixture.rs` | reference; useful behavioral test design only; repair comparator and validate fixture inputs before relying on results | Key::tie_break returns Less when identity <= other.identity; if two distinct keys share identity this violates ordering consistency and can corrupt collision-bin ordering assumptions. Extensive unwrap/indexing relies on trusted generated JSON; side is incorporated into output filenames. No fixture corpus or oracle comparison was executed. |
| 2235 | `crates/horizon-world/examples/probe/active_effects_fixture.rs` | reference; test-only transport; comparator and input-contract limitations must be recorded | Its JavaKey tie_break uses the same <= comparison; duplicate identity values can return Less in both directions. Input arrays and ids are trusted and indexed/unwrap-heavy; side-based input/output paths are not sandboxed. This is fixture harness behavior, not production collection implementation. |
| 2236 | `crates/horizon-world/examples/probe/anchor_fixture.rs` | reference; use as an oracle shape only; validate numeric ranges in a RustChunk-owned fixture runner | JSON values are unwrapped and narrowed from i64 to i32, so malformed/out-of-range fixture values panic or wrap. No production validation is implemented in this example; actual VerticalAnchor behavior lives in the library. |
| 2237 | `crates/horizon-world/examples/probe/attribute_fixture.rs` | reference; fixture-only coverage; use library code for behavior and add schema validation to any new oracle | Fixture schema is trusted: it assumes two definitions, valid modifier indices and valid Operation::ALL indices. Numerous unwraps/indexes turn malformed corpus data into panics. No production Attribute implementation is included in this example. |
| 2238 | `crates/horizon-world/examples/probe/attribute_health_fixture.rs` | reference; useful integration matrix concept only; not a production health subsystem | The test adapter assumes catalog keys, two instances, valid operation/difficulty/attribute indexes and parseable bit strings. Invalid fixture data panics; comments explicitly state this fixture does not test cross-attribute hash order. |
| 2239 | `crates/horizon-world/examples/probe/attribute_map_queries_fixture.rs` | reference; fixture coverage only; preserve its modeled call ordering but add bounded and validated inputs | The fixture relies on generated handle/index invariants and unchecked references into instance/template/supplier arrays. Its key-order/comparator assumptions and input schema are not validated at runtime. The side argument forms fixture filenames directly. No production API behavior is proved by this transport alone. |
| 2240 | `crates/horizon-world/examples/probe/attribute_updates_fixture.rs` | reference; extract ordering scenarios only; validate comparator and fixture schema before oracle use | Key::tie_break has the same <= ordering defect under duplicate identities. Many JSON arrays/indexes are trusted; side is interpolated into input/output paths. It tests callback ordering in a mock world, not live network packet compatibility with Minecraft 1.20.1. |
| 2241 | `crates/horizon-world/examples/probe/basic_feature_fixture.rs` | reference; feature placement test harness only; do not treat it as a supported feature implementation | The mock height query always returns Unavailable; rem_euclid denominator is fixture-provided and can be zero, and malformed/negative repeat values produce panic or empty-loop behavior. Not a complete terrain/world integration. |
| 2242 | `crates/horizon-world/examples/probe/biome_chunk_fixture.rs` | reference; retain section/retrogen scenario coverage only; validate dimensions and divisors | Fixture height is narrowed/cast and later used as usize for section loops; negative or implausible dimensions can yield huge iteration. rem_euclid divisor comes from input and can be zero. It is a focused storage/fill fixture, not actual climate biome generation. |
| 2243 | `crates/horizon-world/examples/probe/biome_generation_fixture.rs` | reference; useful generator test design; pin registries and validate bounds before adoption | Uses min + height - 1 and height conversions without local bounds validation; invalid dimensions could overflow or create excessive loops. Data is loaded from a synthetic fixture bundle, so it is not by itself proof of exact vanilla 1.20.1 registry/codec parity. |
| 2244 | `crates/horizon-world/examples/probe/biome_placement_fixture.rs` | reference; use only for controlled oracle scenarios; add recursion depth/cycle guard in any new test harness | When nested=true, the feature callback recursively invokes another biome placement with the same nested flag; if the biome predicate keeps accepting and no injected feature failure terminates it, recursion can grow without bound. Direct mode only invokes modifiers[0], explicitly relying on a unit-biome-filter corpus rather than arbitrary modifier chains. |
| 2245 | `crates/horizon-world/examples/probe/biome_source_codec_fixture.rs` | reference; codec oracle only after preserving integer-vs-float JSON representation | Normalization converts every JSON number through f64 before comparing bits; integer JSON values above 2^53 lose precision and distinct values may collapse. This is a differential-oracle blind spot if large integer fields enter the schema; `assert!` also assumes no tag names are missing. |
| 2246 | `crates/horizon-world/examples/probe/biome_source_fixture.rs` | reference; focused source oracle only; keep separate from full generator validation | Fixture JSON and position/biome indexes are unwrapped; checkerboard scale and erosion bit patterns depend on trusted corpus. It isolates source algorithms but not registry codec or full generation parity. |
| 2247 | `crates/horizon-world/examples/probe/biome_temperature_fixture.rs` | reference; temperature oracle scenarios only; distinguish thread recreation from concurrent worker behavior | Input JSON/hex strings and operation indexes are unvalidated. Negative fill counts silently run no loop; thread operation creates a fresh worker, so it does not prove cross-thread cache sharing or parallel safety. |
| 2248 | `crates/horizon-world/examples/probe/block_collision_scan_fixture.rs` | reference; useful collision scenario generator only; validate shape and chunk bounds against target 1.20.1 | Palette/state indices and AABB bit inputs are trusted; a malformed fixture may panic. The one-million scan budget bounds the harness but does not establish production-world chunk loading, collision-shape correctness or target-version parity. |
| 2249 | `crates/horizon-world/examples/probe/block_entity_storage_fixture.rs` | reference; block-entity scenario harness only; do not reuse its parser for untrusted NBT | Input::tag reads a value token before checking the kind; a `none` kind therefore still needs a placeholder token or it panics. `pending()` panics when marker is absent/not Long. split_whitespace cannot preserve arbitrary strings with spaces. This is a strict fixture line protocol, not a robust NBT reader. |
| 2250 | `crates/horizon-world/examples/probe/block_landing_fixture.rs` | reference; integration-oracle design only; independently validate entity collision and block effects against Java 1.20.1 | The mock `collision_shape` expects registered cached shapes and panics if assumptions fail. Entity/world flags and binary NBT inputs are trusted; this harness is not a full game server nor a target-1.20.1 packet/event test. No example was executed. |

| 2251 | `crates/horizon-world/examples/probe/block_properties_fixture.rs` | adaptable; reference; fixture-only; do not ship | The fixture asserts property/set agreement, but registry and JSON assumptions are untrusted-index prone: bbox indexes six numbers, fact states and names are unwrapped, and values are cast to i32/u32. Negative or malformed inputs panic. It validates a synthetic resource bundle, not the full target-version block-property registry. |
| 2252 | `crates/horizon-world/examples/probe/block_push_fixture.rs` | adaptable; reference; fixture-only; do not ship | Collision state ids directly index a two-element shape array; entity identity directly indexes the entity vector; empty/malformed arrays panic. The mock is a compact push_entities_up harness and does not model entity broad-phase/chunk residency. |
| 2253 | `crates/horizon-world/examples/probe/block_state_codec_fixture.rs` | adaptable; reference; fixture-only; do not ship | The `integers` whitelist and state facts are generated fixture inputs; arrays and field types are unwrapped. It does not load the canonical 1.20.1 block registry or establish Forge-added state-property compatibility. |
| 2254 | `crates/horizon-world/examples/probe/block_support_fixture.rs` | adaptable; reference; fixture-only; do not ship | Hard-coded assertions require exactly 35,723 registry states and 35,524 cached states; this only fits the corresponding pinned data bundle. All per-state queries assume that corpus. It is not a generic registry validator. |
| 2255 | `crates/horizon-world/examples/probe/block_survival_fixture.rs` | adaptable; reference; fixture-only; do not ship | The mock's `neighbors[old % len]` and `fluid_states[old % len]` panic on empty vectors; fixture rules and indexes are unwrapped; unknown rule names panic. A synthetic world returns fixed/mock entities, brightness, fluids and tags, so it does not prove full world survival behavior. |
| 2256 | `crates/horizon-world/examples/probe/block_update_fixture.rs` | adaptable; reference; fixture-only; do not ship | This harness deliberately simplifies the world: `is_loaded` always true, entity counts are zero, block entities are treated as fresh with hard-coded behavior, and a fixed -3..=3 chunk area is initialized. JSON positions/arrays/state ids and operation names are trusted and often unwrap/index directly. It cannot establish full vanilla/Forge world scheduling or multiplayer parity. |
| 2257 | `crates/horizon-world/examples/probe/blocking_item_damage_fixture.rs` | adaptable; reference; fixture-only; do not ship | `process_enchantments` intentionally returns `unenchanted_change`, so enchantment durability adjustment is not covered. JSON values are unwrapped and narrowed to i32/u8; it does not represent a full inventory/hand/weapon runtime. |
| 2258 | `crates/horizon-world/examples/probe/blocking_rules_fixture.rs` | adaptable; reference; fixture-only; do not ship | The synthetic TypeMatchWorld supports only set ids 1–4; any other set panics. The mock returns static ids and does not model item tags, full damage sources or runtime enchantments. Inputs are trusted and narrowed. |
| 2259 | `crates/horizon-world/examples/probe/bulk_section_fixture.rs` | adaptable; reference; fixture-only; do not ship | Fill chooses `states[((x+3*y+5*z+salt) as usize) % states.len()]`, which panics on empty input and can overflow/cast unexpectedly; height/min and registry indexes are unvalidated. Dumps can produce very large output. The region is synthetic and does not model live chunk ticket/loading behavior. |
| 2260 | `crates/horizon-world/examples/probe/bundle_weight_fixture.rs` | adaptable; reference; fixture-only; do not ship | Nested bundle references are assembled only from already-created bundles, so this fixture cannot express forward/cyclic references. Bundle/index/prototype references and counts are trusted and unchecked; signed counts/maxima are narrowed to i32. It is test transport, not item inventory code. |
| 2261 | `crates/horizon-world/examples/probe/canyon_carver_fixture.rs` | adaptable; reference; fixture-only; do not ship | World-generation bounds compute `min_y + 1` and `min_y + height - 8` without checked arithmetic; context/target/origin/repeat values are trusted. Mask writes unwrap and can panic if callback coordinates escape bounds. No Java oracle was run. |
| 2262 | `crates/horizon-world/examples/probe/carving_geometry_fixture.rs` | adaptable; reference; fixture-only; do not ship | Fixture JSON/hex values and coordinate ranges are trusted; `mask.carve(...).unwrap()` can panic for out-of-range callbacks. Skip mode and synthetic normalized geometry cover only selected predicate variants. |
| 2263 | `crates/horizon-world/examples/probe/carving_mask_fixture.rs` | adaptable; reference; fixture-only; do not ship | Fill loops can expand without a practical size cap and iterate attacker-controlled coordinate ranges; callback tracing uses unwrap and unknown operation paths are unreachable. This is a focused in-memory mask transport. |
| 2264 | `crates/horizon-world/examples/probe/cave_carver_fixture.rs` | adaptable; reference; fixture-only; do not ship | Same fixture risks as canyon: unchecked min_y/height arithmetic, unbounded repeats/input, trusted target/origin and `mask.carve(...).unwrap()` on callbacks. The corpus is not a target-version registry/seed parity proof. |
| 2265 | `crates/horizon-world/examples/probe/chunk_stage_fixture.rs` | adaptable; reference; fixture-only; do not ship | The fill path divides/modulos by `states.len()` and panics on empty arrays; dimensions, state ids, mask words and `i32`→`u32` casts are trusted. `height / 16` and full section dumps can create large loops/output. It is not full chunk-status scheduling/loading. |
| 2266 | `crates/horizon-world/examples/probe/chunk_status_fixture.rs` | adaptable; reference; fixture-only; do not ship | CLI arguments are indexed without an arity guard; JSON row fields are unwrapped and integer inputs narrowed. It queries only fixed synthetic tag variants; this does not validate real chunk files or async chunk scheduling. |
| 2267 | `crates/horizon-world/examples/probe/climbing_fixture.rs` | adaptable; reference; fixture-only; do not ship | Input tag arrays and 0..2 binding selectors are unchecked; registry lookups and JSON coordinates are unwrapped/narrowed. The mock models a small set of world queries rather than real entity movement, step-up or collision resolution. |
| 2268 | `crates/horizon-world/examples/probe/collision_context_fixture.rs` | adaptable; reference; fixture-only; do not ship | 450 lines were read as 1–400 and 401–450. Many JSON enum/index fields are directly indexed/unwrapped; the mock fluid/entity calls use fixed snapshots and some callbacks bypass failure instrumentation. Registry-backed modes require exact fixture NBT. This is not a complete entity/world collision simulation. |
| 2269 | `crates/horizon-world/examples/probe/collision_fixture.rs` | adaptable; reference; fixture-only; do not ship | The line-based SplitWhitespace parser unwraps token counts and numeric formats then asserts no extra tokens. State IDs and f64 hex are unvalidated; imported cache is synthetic/version-bound. Pointer identity observations are process-local and not a renderer/GPU property. |
| 2270 | `crates/horizon-world/examples/probe/combat_rules_fixture.rs` | adaptable; reference; fixture-only; do not ship | `modify_armor_effectiveness` returns its input unchanged, so no weapon/enchantment modifier is actually modeled. JSON/hex numeric values are trusted. The fixture does not model full damage sources, invulnerability frames or multiplayer combat. |
| 2271 | `crates/horizon-world/examples/probe/component_durability_fixture.rs` | adaptable; reference; fixture-only; do not ship | The harness uses `unenchanted_change` as its enchantment processing path, so enchantment-based durability reduction is not covered. Patch/input keys, counts and i32 conversions are trusted; unrecognized patch kinds map to Unbreakable rather than being rejected by the fixture parser. |
| 2272 | `crates/horizon-world/examples/probe/component_patch_fixture.rs` | adaptable; reference; fixture-only; do not ship | Three map/patch arrays are indexed without bounds checks; arbitrary component keys are accepted while the diagnostic values/flags snapshot inspects only keys 0–15. Numeric JSON values are narrowed to i32 and malformed float bits panic. |
| 2273 | `crates/horizon-world/examples/probe/consumable_emission_fixture.rs` | adaptable; reference; fixture-only; do not ship | The `effects` list on Consumable is always empty, so status-effect application is not covered. Random-source and animation indexes assume valid small ranges; item/count values are narrowed to u8/i32 and callback logs use unwrap. The synthetic world is not a real level. |
| 2274 | `crates/horizon-world/examples/probe/consumable_fixture.rs` | adaptable; reference; fixture-only; do not ship | 413 lines read in contiguous ranges 1–400 and 401–413. Random-source selectors index a two-element array unchecked; animation uses a fixed 12-entry table; effect count is cast to u8 and can wrap; numerous JSON fields and callback output paths unwrap. The test world has only synthetic food/effect logic and no real inventory/network lifecycle. |
| 2275 | `crates/horizon-world/examples/probe/consume_effect_fixture.rs` | adaptable; reference; fixture-only; do not ship | Input effect indices are narrowed to u8 and directly index the three-item input vector; `returns[i % len]` panics if the return array is empty; operation/list/position arrays are trusted. Callback return/failure behavior is synthetic and no server/network synchronization is modeled. |

| 2276 | `crates/horizon-world/examples/probe/damage_immunity_fixture.rs` | adaptable; reference | The removed() mock always reads initial state after event mutation; slot inputs are unvalidated. Synthetic policy world, not a real entity/server damage source. | |
| 2277 | `crates/horizon-world/examples/probe/damage_reduction_fixture.rs` | adaptable; reference | ArmorWorld::modify_armor_effectiveness returns the input unchanged and protection always returns zero; real enchantment/armor protection is not modeled. Slot values and bit strings are unvalidated. | |
| 2278 | `crates/horizon-world/examples/probe/damage_source_fixture.rs` | adaptable; reference | Entity kinds use hard-coded numeric IDs; entity/position/weapon arrays and constructor values are unvalidated. Snapshot registry data does not prove target 1.20.1 ID/message parity. | |
| 2279 | `crates/horizon-world/examples/probe/decoration_fixture.rs` | adaptable; reference | Palette selection indexes a 3×3 array from unvalidated coordinates and repeats synthetic biome sequences; heightmap queries always return Unavailable. No real chunk biome storage. | |
| 2280 | `crates/horizon-world/examples/probe/decoration_region_fixture.rs` | adaptable; reference | Builds resident TerrainRegion/BiomeRegion; exercises pre-light stages, retrogen, write-zone/height gates, block/fluid tags, heightmaps, scheduled ticks and mutation lifecycle. Uses snapshot DATA_VERSION; state/palette IDs and coordinates unvalidated; not async chunk acquisition. | |
| 2281 | `crates/horizon-world/examples/probe/dynamic_support_fixture.rs` | adaptable; reference | State/direction indexes are unchecked; unsupported states panic at expect. Entity/piston states are synthetic JSON, not live ticking block entities. | |
| 2282 | `crates/horizon-world/examples/probe/edge_backoff_fixture.rs` | adaptable; reference | Missing clear-array entries silently become false; AABB/vector/step inputs are trusted. Scripted collision answers are not a live block/entity collision world. | |
| 2283 | `crates/horizon-world/examples/probe/effect_attribute_modifiers_fixture.rs` | adaptable; reference | Models definitions/templates, instance routing, permanent modifier changes, observer callbacks and in-callback mutation. Keys/templates/routes/indexes are unvalidated; mock routing is not a live entity map. | |
| 2284 | `crates/horizon-world/examples/probe/effect_notifications_fixture.rs` | adaptable; reference | JavaKey::tie_break uses <= so equal identities compare Less. Passenger/connection rules are hard-coded; packets are strings, not wire encoding. Input maps/handles unbounded/unwrapped. | |
| 2285 | `crates/horizon-world/examples/probe/effect_sync_fixture.rs` | adaptable; reference | Repeats JavaKey tie_break <= defect; effect/hash/schema IDs trusted. Client/server is a boolean mock; no real metadata packet encoding or client rendering. | |
| 2286 | `crates/horizon-world/examples/probe/entity_attribute_fixture.rs` | adaptable; reference | Attribute/index/operation inputs unvalidated; every operation builds a Modifier even for read-only lookups. No entity lifecycle or packet sync. | |
| 2287 | `crates/horizon-world/examples/probe/entity_collision_fixture.rs` | adaptable; reference | Identity indexes are 1-based vector offsets; parent/box indexes unchecked. Mock returns all candidates without query-bound prefilter and has no dynamic removal/building state. | |
| 2288 | `crates/horizon-world/examples/probe/entity_fluid_fixture.rs` | adaptable; reference | Synthetic cells/tags/states and optional registries. is_pushed_by_fluid uses values.len()-1 and panics for empty arrays; other state/tag indexes unchecked. Embedded budget assertions were not run. | |
| 2289 | `crates/horizon-world/examples/probe/entity_move_fixture.rs` | adaptable; reference | Mock shapes and scripted world queries; passenger() always true and apply_movement_emission intentionally panics if guard missed. Inputs unvalidated; no async chunk collision or network authority. | |
| 2290 | `crates/horizon-world/examples/probe/entity_obstruction_fixture.rs` | adaptable; reference | Parent/entity indexes and AABB arrays unvalidated. removed and blocks_building fields are stored but ignored by mock candidate filtering; no live world state. | |
| 2291 | `crates/horizon-world/examples/probe/extra_items_fixture.rs` | adaptable; reference | Combines ItemStacks, component patches, inventory, remainder, predicted drops and cooldowns. Inventory hard-coded to 43 slots; inputs unvalidated; component model is not 1.20.1 legacy ItemStack/NBT. | |
| 2292 | `crates/horizon-world/examples/probe/face_support_fixture.rs` | adaptable; reference | Grid coordinates may be empty/inconsistent with occupied bits; cube dimensions use len()-1 and may underflow. No cap on geometry complexity. | |
| 2293 | `crates/horizon-world/examples/probe/face_visibility_fixture.rs` | adaptable; reference | Loads snapshot block/occlusion registries; rules include mangrove roots and require version verification. Pair parser validates little beyond split and integer conversion. | |
| 2294 | `crates/horizon-world/examples/probe/fall_damage_fixture.rs` | adaptable; reference | block_state always returns ID 1 and position is fixed; no actual damage source, sound or server event. Values/positions are trusted JSON. | |
| 2295 | `crates/horizon-world/examples/probe/feature_catalog_fixture.rs` | adaptable; reference | Builds catalog and registry with current DATA_VERSION facts; arrays/repeats/coordinates unvalidated. Heightmap always Unavailable; mock blocks use deterministic hashes rather than resident terrain. | |
| 2296 | `crates/horizon-world/examples/probe/feature_graph_fixture.rs` | adaptable; reference | Mock height unavailable and block lookups hash-derived; repeats/origin unvalidated. Does not load feature registry, tags, bindings or terrain. | |
| 2297 | `crates/horizon-world/examples/probe/feature_placement_fixture.rs` | adaptable; reference | Uses synthetic query adapter/result modes and supplied positions; no real terrain or registry. Input modifiers/context are trusted. | |
| 2298 | `crates/horizon-world/examples/probe/feature_region_fixture.rs` | adaptable; reference | Builds 3×3 TerrainRegion/BiomeRegion; exercises features, pre-light stage, retrogen, write-zone/height gates, fluid/block tags, heightmaps and tick queues. DATA_VERSION/current snapshot; IDs/arrays unchecked; no async chunk acquisition. | |
| 2299 | `crates/horizon-world/examples/probe/feature_sort_fixture.rs` | adaptable; reference | Converts feature lists and reports topological steps/cycles. IDs cast to u32; fixture does not validate registry resolution or placed-feature dependency data. | |
| 2300 | `crates/horizon-world/examples/probe/fill_layer_fixture.rs` | adaptable; reference | Loads current DATA_VERSION block registry; mock world hashes positions to seven states, keeps writes in memory and returns synthetic height failures. Repeats/origin/world_min unvalidated. | |


## Decision taxonomy normalization — entries 2276–2300 — 2026-11-21

- The initial Ledger/Index rows used reference as a standalone Decision. Protocol requires a primary decision; reference is a qualifier only.
- All 25 rows are normalized to adaptable; reference. Supporting Findings and RustChunk relevance retain the fixture-only/no-production-runtime limit.
- Entry IDs, source paths, SHAs, sizes and technical findings are unchanged.

| 2301 | `crates/horizon-world/examples/probe/fluid_box_fixture.rs` | adaptable; reference | Vehicle ID narrows u64→u8; unknown vehicle values panic. AABB coordinates and callback fields are trusted JSON; numeric hex parsing unwraps. Only three vehicle cases are modeled, not a full vehicle registry. |
| 2302 | `crates/horizon-world/examples/probe/fluid_fixture.rs` | adaptable; reference | State IDs narrow to u32 and positions i64→i32; registry/above-state lookups unwrap. Correctness depends on the private facts-server registry snapshot; no target 1.20.1 fluid registry parity is established. |
| 2303 | `crates/horizon-world/examples/probe/fluid_flow_fixture.rs` | adaptable; reference | Cell/state/direction selectors index fixture arrays directly; coordinates narrow to i32. Missing map cells use a trusted default. The mock is not a loaded-chunk/block-tag simulation. |
| 2304 | `crates/horizon-world/examples/probe/fluid_occupancy_fixture.rs` | adaptable; reference | Fixture dimensions, sections, height and coordinates are unvalidated before indexing/arithmetic; min+height−1 and section conversions can overflow or index out of range. Single synthetic chunk model. |
| 2305 | `crates/horizon-world/examples/probe/fluid_shape_fixture.rs` | adaptable; reference | Position/state JSON and operations are trusted and unwrap-heavy. sturdy_down intentionally panics to assert this shape path does not request camera-immersion support; this is a narrow contract fixture. |
| 2306 | `crates/horizon-world/examples/probe/food_fixture.rs` | adaptable; reference | Difficulty indexes a four-item array unchecked; food/timer/nutrition narrow to i32 and bit strings/operation names are trusted. Synthetic world, not a live player or authoritative server. |
| 2307 | `crates/horizon-world/examples/probe/food_properties_fixture.rs` | adaptable; reference | Random source and food index address fixed two-element arrays directly; values narrow to i32/u32 and unwrap. Mock callbacks do not model inventory/network lifecycle. |
| 2308 | `crates/horizon-world/examples/probe/food_storage_fixture.rs` | adaptable; reference | Tag variants 7–12 are replaced with fixed synthetic values rather than arbitrary payloads; integer narrowing/parsing unwrap. Not a general NBT codec test. |
| 2309 | `crates/horizon-world/examples/probe/ground_state_fixture.rs` | adaptable; reference | Coordinates narrow i64→i32; results are indexed by callback count and mode values are trusted. Support queries are scripted closures, not live chunk collision. |
| 2310 | `crates/horizon-world/examples/probe/height_provider_fixture.rs` | adaptable; reference | Context values narrow to i32 and sample count is unbounded input; JSON fields unwrap. Synthetic definition list does not prove 1.20.1 provider registry coverage. |
| 2311 | `crates/horizon-world/examples/probe/heightmap_fixture.rs` | adaptable; reference | height*256 allocation, min+height bounds and section calculations are unchecked; invalid/negative dimensions can overflow or allocate incorrectly. State IDs, raw words, masks and op indexes are trusted. One synthetic chunk only. |
| 2312 | `crates/horizon-world/examples/probe/interaction_shape_fixture.rs` | adaptable; reference | Other RayWorld methods deliberately panic because the fixture calls direct interaction clipping. Positions/state IDs/geometry arrays are trusted; not general block/fluid ray tracing. |
| 2313 | `crates/horizon-world/examples/probe/inventory_pickup_fixture.rs` | adaptable; reference | 410 lines read in ranges 1–400 and 401–410. Snapshots assume 43 slots; item/prototype/ref/equipment indexes are trusted and many unwraps can panic on malformed input. Horizon component stacks are not 1.20.1 legacy ItemStack/NBT. |
| 2314 | `crates/horizon-world/examples/probe/inventory_tick_fixture.rs` | adaptable; reference | Fixed slot/equipment indexes are unchecked; levels narrow to u8 and item_tick asserts server level 1. Synthetic refs do not model a full player tick, held-slot sync or packets. |
| 2315 | `crates/horizon-world/examples/probe/inventory_use_fixture.rs` | adaptable; reference | Uses three mock items, fixed inventory and one synced flag ID 8; callback arrays/item indexes are trusted. Not the full server/client interaction or packet state machine. |
| 2316 | `crates/horizon-world/examples/probe/inventory_use_start_fixture.rs` | adaptable; reference | 436 lines read in ranges 1–400 and 401–436. Three item refs, fixed 43-slot inventory and one synced data ID 8; arrays/conversions unchecked. Does not encode actual 1.20.1 packets. |
| 2317 | `crates/horizon-world/examples/probe/item_after_use_fixture.rs` | adaptable; reference | Observer::ended is deliberately unreachable in the fixture; source/finish/prototype indexes and counts are trusted. Only a small three-ref item set exists; no full inventory remainder placement. |
| 2318 | `crates/horizon-world/examples/probe/item_blocking_fixture.rs` | adaptable; reference | TypeMatchWorld supports only test set IDs 1–4; other IDs panic. Entity/projectile kinds are hard-coded; pierce narrows to i8 and component indexes are trusted. Not a full damage/enchantment pipeline. |
| 2319 | `crates/horizon-world/examples/probe/item_cooldowns_fixture.rs` | adaptable; reference | Default group lookup indexes a three-element array by item ID; group/mutation indexes are trusted. Recursive mutation is synthetic; no player cooldown packets are generated. |
| 2320 | `crates/horizon-world/examples/probe/item_durability_fixture.rs` | adaptable; reference | process_enchantments explicitly uses unenchanted_change, so enchantment durability modifiers are not covered. Numeric values narrow to i32; callback JSON is trusted. |
| 2321 | `crates/horizon-world/examples/probe/item_particles_fixture.rs` | adaptable; reference | Random source indexes a fixed two-element array; particle count is input-controlled without a local cap; item/bit values narrow or unwrap. No packet delivery/rendering is exercised. |
| 2322 | `crates/horizon-world/examples/probe/item_stack_storage_fixture.rs` | adaptable; reference | Fixed three slots; prototype/operation/component indexes and counts are trusted and narrowed. Component-stack model is not proof of 1.20.1 legacy ItemStack NBT or Forge capability behavior. |
| 2323 | `crates/horizon-world/examples/probe/item_template_fixture.rs` | adaptable; reference | Three template slots; prototype/template/stack indexes trusted; keys/counts narrow to u8/i32 and patch fields unwrap. Mock contents validator always succeeds and max-damage/stack rules are simplified. |
| 2324 | `crates/horizon-world/examples/probe/item_use_catalog_fixture.rs` | adaptable; reference | Unknown catalog names unwrap and panic; float hex/integer ranges are not validated. Definitions belong to this snapshot and are not a 1.20.1 registry or packet contract. |
| 2325 | `crates/horizon-world/examples/probe/item_use_fixture.rs` | adaptable; reference | Two-item component/delay array is fixed; selectors/durations are trusted. Uses Horizon BlockingRules and does not implement full 1.20.1 hand-use animation, release packets or Forge events. |

| 2326 | `crates/horizon-world/examples/probe/item_use_lifecycle_fixture.rs` | adaptable; reference | Small fixed item/hand catalogue; JSON indexes, modes and references are trusted, narrowed or indexed directly. Not a live client/server hand-use packet state machine. |
| 2327 | `crates/horizon-world/examples/probe/item_use_start_fixture.rs` | adaptable; reference | Selectors and durations are trusted or narrowed; fixed mock references do not establish real inventory sync, packet contents or Forge event ordering. |
| 2328 | `crates/horizon-world/examples/probe/java_hash_map_fixture.rs` | adaptable; reference | IDs/hash narrow to i32/usize; key indices and operation fields are unvalidated. Container contract only, not production gameplay or cross-thread safety proof. |
| 2329 | `crates/horizon-world/examples/probe/java_identity_map_fixture.rs` | adaptable; reference | `tie_break` returns Less for `identity <= other.identity`; equal identities compare Less both ways, invalid if distinct keys share an identity. Input indexes/conversions are unchecked. |
| 2330 | `crates/horizon-world/examples/probe/level_place_fixture.rs` | adaptable; reference | `is_loaded` always true and `set_raw` always None; neighbour/face/state indices and coordinates are trusted. No real chunk availability or server placement packets. |
| 2331 | `crates/horizon-world/examples/probe/level_shape_fixture.rs` | adaptable; reference | `is_loaded` is always true; cell arrays, directions, positions and tick IDs are trusted/cast. Not full neighbour scheduling or unloaded-chunk behavior. |
| 2332 | `crates/horizon-world/examples/probe/live_attribute_updates_fixture.rs` | adaptable; reference | 598 lines read in 1–400 and 401–598. EffectKey::tie_break always Less; only consistent with the fixture's single logical key. Attribute/map indexes, operations and arrays are trusted. Passengers empty, server-player false, connection and send_packet are no-ops. |
| 2333 | `crates/horizon-world/examples/probe/live_attributes_fixture.rs` | adaptable; reference | Instance-hash iterator uses next().unwrap(); operation/attribute indexes directly access arrays. Hex/amplifier values are trusted; no real entity lifecycle or network sync. |
| 2334 | `crates/horizon-world/examples/probe/living_air_fixture.rs` | adaptable; reference | Sequence indexes `a[min(n,len-1)]`; empty arrays underflow/panic. JSON flags/bit strings are trusted; fixed support coordinates and synthetic environment. |
| 2335 | `crates/horizon-world/examples/probe/living_damage_fixture.rs` | adaptable; reference | State/bit inputs trusted; conversions and event counts unbounded. Attacker is a unit mock; incomplete DamageSource/death/entity-attribute/network model. |
| 2336 | `crates/horizon-world/examples/probe/living_fall_fixture.rs` | adaptable; reference | Mode/array indexes are trusted; empty immunity array panics and grace ticks narrow to i32. Passenger/server are synthetic. |
| 2337 | `crates/horizon-world/examples/probe/living_fluid_fixture.rs` | adaptable; reference | Ground/climb/sprint sequences are trusted; empty arrays panic. Numeric fields/selectors are unchecked; chunk, vehicle, fluid and free-space queries are scripted mocks. |
| 2338 | `crates/horizon-world/examples/probe/living_health_fixture.rs` | adaptable; reference | Active data indexes fixed two-element arrays unchecked; JSON and hex floats trusted. Dead/attribute data come from a synthetic context. |
| 2339 | `crates/horizon-world/examples/probe/living_hurt_fixture.rs` | adaptable; reference | Effect count unbounded; cooldown/duration/amplifier/counts narrow to i32 and JSON is trusted. Mock source/blocking/sound/player attribution; no live entity packets or Forge hooks. |
| 2340 | `crates/horizon-world/examples/probe/living_landing_fixture.rs` | adaptable; reference | Support/landing coordinates narrow to i32 and block_position unwraps a required value; block state is fixed u8, no live registry collision shapes. |
| 2341 | `crates/horizon-world/examples/probe/living_motion_fixture.rs` | adaptable; reference | Amplifier narrows to i32; bit strings and operation names trusted. Synthetic callbacks do not validate entity physics, movement packets or interpolation. |
| 2342 | `crates/horizon-world/examples/probe/material_condition_fixture.rs` | adaptable; reference | 735 lines read in ranges 1–400 and 401–735. Condition/rule schemas, anchors and indexes unwrap/direct-index; context/IDs narrow to i32/u32; ore channels index a fixed three-slot array. Numeric JSON normalization uses f64, so preserve integer-vs-double representation when extending the schema. |
| 2343 | `crates/horizon-world/examples/probe/material_extension_fixture.rs` | adaptable; reference | Column is synthetic; `remaining==0` injects failure while negative means unbounded reads. Block IDs, biome/noise names, heights and coordinates are trusted/cast; same_block has a hard-coded alias map. |
| 2344 | `crates/horizon-world/examples/probe/material_stage_fixture.rs` | adaptable; reference | 828 lines read in 1–400, 401–800 and 801–828. Height/min/dimensions and min+height−1/min+height−8 arithmetic are not fully checked; usize casts can amplify invalid inputs. State arrays/properties/registry lookups unwrap. Results depend on private registry snapshots and mock policies, not canonical vanilla 1.20.1/Forge data. |
| 2345 | `crates/horizon-world/examples/probe/material_surface_fixture.rs` | adaptable; reference | Point coordinates narrow to i32; override arrays assume three values; missing noise or invalid hex unwrap. ConstantNoise asserts a restricted query dimension; not a full material stage. |
| 2346 | `crates/horizon-world/examples/probe/movement_emission_fixture.rs` | adaptable; reference | MovementEmission directly indexes a four-item array from unvalidated input; callback state indexes and JSON arrays are trusted. Synthetic sound/event hooks do not deliver packets or play audio. |
| 2347 | `crates/horizon-world/examples/probe/movement_registry_fixture.rs` | adaptable; reference | State provider indexes `states[call % states.len()]`; empty array panics. Required tags and registry facts assert/unwrap; corpus is tied to a resource snapshot. |
| 2348 | `crates/horizon-world/examples/probe/movement_support_fixture.rs` | adaptable; reference | State selection uses `reads % states.len()` and panics on empty inputs; mode/state selectors and factors are trusted. No real collision-shape/chunk query. |
| 2349 | `crates/horizon-world/examples/probe/object_open_map_fixture.rs` | adaptable; reference | Key/entry indexes directly access arrays; values narrow to i32 and malformed operation/action arrays panic. Key equality is ID-only while object/hash are independent. |
| 2350 | `crates/horizon-world/examples/probe/object_open_set_fixture.rs` | adaptable; reference | Hash/ID arrays and key indexes are unchecked; operation values narrow and malformed modes panic. Container contract fixture only; no concurrency proof. |

| 2351 | `crates/horizon-world/examples/probe/piston_movement_fixture.rs` | adaptable; reference | Hex-float and operation fields are trusted JSON and parsed with unwrap; the fixture covers the state helper and callback sequencing, not a full moving-piston block entity or entity push lifecycle. |
| 2352 | `crates/horizon-world/examples/probe/piston_push_fixture.rs` | adaptable; reference | AABB coordinates are read by unchecked six-element indexing; positions narrow i64 to i32; direction/reaction are normalized with rem_euclid, but box lists and hex numbers remain trusted. No live entity broad-phase, chunk or block-entity state. |
| 2353 | `crates/horizon-world/examples/probe/piston_shape_fixture.rs` | adaptable; reference | Direction directly indexes Direction::ALL; state indexes cached/dynamic registry arrays; nonnegative noclip directly indexes Direction::ALL. Dynamic-support entries are expected to exist and use expect; fixture NBT/corpus is trusted. |
| 2354 | `crates/horizon-world/examples/probe/placement_fixture.rs` | adaptable; reference | World-generation context and positions narrow i64 to i32; JSON fields and operation modes are trusted. Query semantics are delegated to a fixture transport, not a live chunk/worldgen scheduler. |
| 2355 | `crates/horizon-world/examples/probe/player_equipment_fixture.rs` | adaptable; reference | Snapshot loops hard-code 43 inventory slots; EquipmentSlot::ALL and stack-reference vectors are indexed without bounds checks; numeric fields narrow to u8/i32. Mock callbacks can mutate selection, slots and stack counts, not actual entity synchronization. |
| 2356 | `crates/horizon-world/examples/probe/player_food_fixture.rs` | adaptable; reference | Direct/virtual indexes select fixed two-slot food arrays without range checks; bit values and fields unwrap. StoredPlayerFood branch unwraps run() so an injected callback error can panic instead of being emitted as a normal fixture error. |
| 2357 | `crates/horizon-world/examples/probe/player_health_fixture.rs` | adaptable; reference | Health/absorption IDs narrow to u8; attribute selector directly indexes a two-key array; Operation::ALL is directly indexed. JSON/hex values and assigned field selectors are trusted; no live entity or network sync. |
| 2358 | `crates/horizon-world/examples/probe/player_hurt_fixture.rs` | adaptable; reference | Difficulty selection uses an unchecked fixed enum index; attacker mode uses a small hard-coded match and panics outside 0–3. Durations/counts narrow to i32 and JSON/hex inputs unwrap; sound/entity/network adapters are synthetic. |
| 2359 | `crates/horizon-world/examples/probe/player_inventory_fixture.rs` | adaptable; reference | Snapshots hard-code 43 slots; inventory target, refs, prototype, equipment order and component indexes are trusted and directly indexed; item/count values narrow to u8/i32. It models Horizon component stacks, not 1.20.1 legacy ItemStack NBT or Forge capabilities. |
| 2360 | `crates/horizon-world/examples/probe/player_travel_fixture.rs` | adaptable; reference | `living_travel` is intentionally scripted with a hard-coded fluid query at (17,-64,-33), not a real world/block lookup. Bit fields and modes unwrap; state is mock, so it cannot establish actual fluid/chunk/player-travel behavior. |
| 2361 | `crates/horizon-world/examples/probe/portal_travel_fixture.rs` | adaptable; reference | World/chunk availability is scripted and treated as loaded; dimension/position/velocity values are fixture inputs. Does not model real chunk tickets, portal search or cross-dimension network entity synchronization. |
| 2362 | `crates/horizon-world/examples/probe/registered_player_combat_fixture.rs` | adaptable; reference | 609 lines read in contiguous ranges 1–400 and 401–609. Several unsupported hooks deliberately panic to assert the chosen path; input selectors/catalog keys/operation indices are trusted. Mock player/source callbacks are not live combat or packet delivery. |
| 2363 | `crates/horizon-world/examples/probe/registry_collision_scan_fixture.rs` | adaptable; reference | 591 lines read in contiguous ranges 1–400 and 401–591. Scan/corpus bounds are fixture-defined; registry state indexes and shape data rely on private NBT snapshots. It does not prove live chunk collision or renderer/GPU behavior. |
| 2364 | `crates/horizon-world/examples/probe/registry_fall_ray_fixture.rs` | adaptable; reference | State/position indexes and geometric bit inputs are trusted; registry corpus is pinned to private data. No real entity movement, unloaded chunk or server-side ray packet path. |
| 2365 | `crates/horizon-world/examples/probe/registry_fluid_fixture.rs` | adaptable; reference | Bindings depend on fixture NBT/registry snapshots; state IDs and tag/name inputs are trusted. This does not independently verify vanilla 1.20.1 fluid tags or full fluid physics. |
| 2366 | `crates/horizon-world/examples/probe/replace_block_fixture.rs` | adaptable; reference | Mock `height()` deliberately returns Unavailable, so paths requiring a heightmap are not covered; block/state/context indexes and JSON values are trusted. The synthetic world does not model real heightmaps or chunk availability. |
| 2367 | `crates/horizon-world/examples/probe/restitution_fixture.rs` | adaptable; reference | Vectors/scalars arrive as hex float strings and are unwrapped without finite/range validation; fixture does not execute a real collision solver or entity physics loop. |
| 2368 | `crates/horizon-world/examples/probe/retrogen_data_fixture.rs` | adaptable; reference | NBT/JSON field shapes and mask sizes are trusted; height/section conversions and bit operations are not a general hostile-input validator. Does not run actual chunk retrogen scheduling. |
| 2369 | `crates/horizon-world/examples/probe/rule_test_fixture.rs` | adaptable; reference | Height/state arrays are indexed with modulo and can panic on empty input; JSON schemas, IDs and repeat counts are trusted. |
| 2370 | `crates/horizon-world/examples/probe/scattered_ore_fixture.rs` | adaptable; reference | Mode values index a fixed six-offset array without full bounds validation; `height()` is Unavailable in the mock, so height-dependent branches are not covered. Registry/state inputs are fixture-specific. |
| 2371 | `crates/horizon-world/examples/probe/server_player_combat_fixture.rs` | adaptable; reference | 629 lines read in ranges 1–400 and 401–629. Several callbacks intentionally panic because only the registered player path is expected; player, weapon and source adapters are mocks, not real packets/server entities. |
| 2372 | `crates/horizon-world/examples/probe/server_player_damage_fixture.rs` | adaptable; reference | `modify_armor_effectiveness` returns its input unchanged, so armor-modifier hooks are not actually applied in this fixture. Attacker-related methods deliberately panic for the player-only path; JSON attribute/operation/slot indexes are trusted. |
| 2373 | `crates/horizon-world/examples/probe/simple_block_fixture.rs` | adaptable; reference | 773 lines read in ranges 1–400 and 401–773. World height/generation is hard-coded (-64..319); state/position/registry/feature inputs unwrap or cast unchecked. Chunk load state and real server/entity/network behavior are not modeled. The fixture consumes private registry/NBT/catalog snapshots and is not proof of canonical 1.20.1/Forge data parity. |
| 2374 | `crates/horizon-world/examples/probe/simple_container_fixture.rs` | adaptable; reference | Prototype/ref/slot/component indexes and numeric values are trusted and narrowed; many snapshot and callback writes unwrap. The mock uses small item/count queries (0–2) and does not model real player inventory packets or Forge capabilities. |
| 2375 | `crates/horizon-world/examples/probe/spawn_origin.rs` | adaptable; reference | The registry is intentionally leaked with Box::leak for a process-lifetime reference; seed list is unbounded CLI input. The utility depends on supplied client resource snapshot and DATA_VERSION and emits parity lines only. |

| 2376 | `crates/horizon-world/examples/probe/state_provider_fixture.rs` | adaptable; reference | 530 lines read in 1–400 and 401–530. Registry facts are private-snapshot dependent; provider/operation IDs and state arrays are trusted or directly indexed. Missing registry/tag entries can fail via unwrap/expect; mock callbacks do not prove real chunk/world behavior. |
| 2377 | `crates/horizon-world/examples/probe/status_effect_fixture.rs` | adaptable; reference | Effect/catalog indexes and integer conversions are trusted; fixed samples do not represent the complete 1.20.1 effect registry. Malformed ids or numbers may panic or narrow unexpectedly. |
| 2378 | `crates/horizon-world/examples/probe/status_tick_fixture.rs` | adaptable; reference | Effect and callback selectors are trusted/direct-indexed; counts/durations narrow to fixed integer widths. Mock callbacks do not model real entity ticking, packet delivery or concurrent effect mutation. |
| 2379 | `crates/horizon-world/examples/probe/stepped_interpolation_fixture.rs` | adaptable; reference | Whitespace parser relies on fixed token counts/order and numeric parsing; malformed or extra tokens are not handled as a general input format. Boundary and non-finite values are corpus-controlled. |
| 2380 | `crates/horizon-world/examples/probe/stored_consume_fixture.rs` | adaptable; reference | 407 lines read in 1–400 and 401–407. Key ordering uses a JavaKey tie-breaker that returns Less for equal identity; operation/item/effect indexes and random selectors are trusted and may panic. Not a complete inventory/server consumption path. |
| 2381 | `crates/horizon-world/examples/probe/stored_effect_attributes_fixture.rs` | adaptable; reference | 483 lines read in 1–400 and 401–483. Instance hashes/template/operation indexes and vector references are trusted; i64→i32 narrowing can wrap. Test corpus controls identity/hash arrangements rather than arbitrary duplicate-key populations. |
| 2382 | `crates/horizon-world/examples/probe/stored_effect_notifications_fixture.rs` | adaptable; reference | EffectKey comparator always returns Less, including equal keys; invalid if key space expands beyond one logical key. Passengers are empty, server-player is false, connection is a no-op and send_packet discards packets; network notifications are not covered. JSON arrays/hashes/operation ids are trusted. |
| 2383 | `crates/horizon-world/examples/probe/structure_density_fixture.rs` | adaptable; reference | Bounding-box coordinates, structure references and numeric inputs are trusted; only a small fixture structure set is loaded. Not the complete 1.20.1 structure registry/generation settings. |
| 2384 | `crates/horizon-world/examples/probe/structure_pieces.rs` | adaptable; reference | Output depends on exact input NBT and available piece decoders; unknown/invalid tags follow limited parser assumptions. Not a structure placement engine. |
| 2385 | `crates/horizon-world/examples/probe/structure_start_nbt.rs` | adaptable; reference | Expects fixture NBT keys and supported piece encodings; missing/wrong-type tags rely on parser errors and corpus assumptions. Does not validate world-level references or scheduling. |
| 2386 | `crates/horizon-world/examples/probe/suffocation_fixture.rs` | adaptable; reference | State ids, position selectors and registry/NBT inputs are trusted or directly indexed. Uses a private registry/collision snapshot; does not model live chunk loading or dynamic block-entity shapes comprehensively. |
| 2387 | `crates/horizon-world/examples/probe/support/common.rs` | adaptable; reference | i64→i32 casts can narrow/wrap; position conversion indexes the first three array elements without length check; unsupported JSON forms panic. Numeric normalization through f64 can lose integer precision above 2^53. |
| 2388 | `crates/horizon-world/examples/probe/support/decoration_storage.rs` | adaptable; reference | Depends on private facts-server.json/tags snapshots; required tags/state records unwrap/assert and state ids are trusted. Corpus-specific, not a reusable versioned resource loader. |
| 2389 | `crates/horizon-world/examples/probe/support/placement_queries.rs` | adaptable; reference | Coordinates use narrowing conversions; query modes/array selectors are corpus-controlled. Some coordinate updates use wrapping arithmetic and dimensions/defaults are fixed, not a real world implementation. |
| 2390 | `crates/horizon-world/examples/probe/support/shapes.rs` | adaptable; reference | Serialization is limited to fixture output; does not validate collision geometry, canonical shape identity or Android/GPU rendering. |
| 2391 | `crates/horizon-world/examples/probe/swimming_fixture.rs` | adaptable; reference | Fluid selector feeds a bit shift/mask without full range validation; mock uses a fixed position rather than sampling live world. Narrow predicate test, not movement/fluid simulation. |
| 2392 | `crates/horizon-world/examples/probe/synced_bytes_fixture.rs` | adaptable; reference | IDs and masks narrow to u8; field selectors and callback operation arrays are trusted. Nested observers can mutate/clear dirty state; outcome depends on re-entry order. |
| 2393 | `crates/horizon-world/examples/probe/synced_floats_fixture.rs` | adaptable; reference | ID casts and field selectors are unchecked; float inputs may be NaN/infinite/negative and callbacks are synthetic. No network packet serializer is validated. |
| 2394 | `crates/horizon-world/examples/probe/teleport_consume_fixture.rs` | adaptable; reference | Random-source selectors index a fixed two-element array; item/count/position values are trusted/narrowed. Reset may re-read config; no actual teleport event/server packet is delivered. |
| 2395 | `crates/horizon-world/examples/probe/terrain_fixture.rs` | adaptable; reference | Dimension/min/height/state indexes and repeat counts are fixture inputs; invalid height/section alignment can produce incorrect allocations/iteration. Density/router/registries come from synthetic resource bundle, not canonical 1.20.1 resources. |
| 2396 | `crates/horizon-world/examples/probe/tick_fixture.rs` | adaptable; reference | Corpus-specific assertion expects exactly 251 ticks; input id casts to u32 and required NBT fields are trusted. Not a general region/tick corruption validator. |
| 2397 | `crates/horizon-world/examples/probe/tree_feature_fixture.rs` | adaptable; reference | 672 lines read in 1–400 and 401–672. Terrain uses fixed test world/height and private registry/tag snapshots; state/property/feature indexes and JSON inputs are trusted. Does not establish production chunk loading, Forge feature injection or biome/worldgen parity. |
| 2398 | `crates/horizon-world/examples/probe/use_flag_reaction_fixture.rs` | adaptable; reference | Fixed item/slot/flag layout and operation/index values are trusted; numeric selectors narrow to small integer types. Not the full 1.20.1 item-use packet/sync lifecycle. |
| 2399 | `crates/horizon-world/examples/probe/use_remainder_fixture.rs` | adaptable; reference | Item/template/slot indexes and counts are trusted; fixed mock inventory capacity/callbacks do not model complete player inventory or Forge capabilities. |
| 2400 | `crates/horizon-world/examples/probe/value_provider_fixture.rs` | adaptable; reference | Provider IDs/names, sample count and state references are fixture-controlled; unknown providers or empty state lists can fail. Catalogue is not complete target-version registry. |







## Ledger/Index correction — 2026-11-21

- The first checkpoint registration read-back exposed a material mismatch: Index had advanced to 2,550 while Ledger still contained only 2401–2525. The 25 new Index rows were present but the 25 corresponding Ledger records had not been appended.
- This was corrected before declaring the checkpoint complete. The corrective atomic six-document commit includes all 25 Ledger entries 2526–2550; the intended final structural state is 150 Ledger records and 2,550 sequential Index rows with matching paths/Decisions.
- This correction note is retained as an audit trail; the first registration must not be treated as the final completed checkpoint.

## Decision-alignment correction — 2026-11-21

- Post-correction read-back found one remaining Index wording mismatch at 2536: the Index said `version-specific class-dispatch required`, while the Ledger's Decision was `target-version class-dispatch required`. The Index row has been normalized to the Ledger's exact Decision wording. No path or numbering changed.

| 2401 | `crates/horizon-world/examples/probe/view_direction_fixture.rs` | adaptable; reference | JSON modes/vectors are unwrapped; malformed hex, short vectors or unknown modes panic. No camera/Android input integration. |
| 2402 | `crates/horizon-world/examples/probe/water_splash_fixture.rs` | adaptable; reference | Passenger/mutation fields are trusted; unknown passenger panics. Fixed mock entity model and hard-coded budget/RNG checks. |
| 2403 | `crates/horizon-world/examples/probe/world_border_fixture.rs` | adaptable; reference | Operation arrays unwrapped; limit narrows i64 to i32; fixed defaults, not packet/timing integration. |
| 2404 | `crates/horizon-world/examples/probe/world_fixture.rs` | adaptable; reference | Filename-derived count trusted (zero causes modulo panic); IDs narrow to u32; assumes fixed state naming/layout. |
| 2405 | `crates/horizon-world/examples/probe/world_ray_fixture.rs` | adaptable; reference | Mode indexes fixed arrays unchecked; unknown cells default to state zero; mock flags do not represent live chunks/entities. |
| 2406 | `crates/horizon-world/examples/probe/worldgen_random_fixture.rs` | adaptable; reference | Inputs narrow i64 to i32/u16; unknown ops/layers panic; selected paths only. |
| 2407 | `crates/horizon-world/examples/structure_blocks.rs` | adaptable; reference | Registry intentionally leaked for process lifetime; cx±1/cz±1 can overflow at i32 edges; external resource bundle required. |
| 2408 | `crates/horizon-world/src/active_effect_tick.rs` | adaptable; reference | Server unwraps current effect after cursor advancement; callback mutation/cursor invalidation needs parity. Client RNG bounds unwrap. |
| 2409 | `crates/horizon-world/src/active_effects.rs` | adaptable; reference | Some mutations precede callbacks; callback errors do not roll back. Public direct map mutations bypass lifecycle hooks. |
| 2410 | `crates/horizon-world/src/attachments.rs` | adaptable; reference | 695 lines read 1–400/401–695. apply_sync defers codec validation until read; identity is namespace/name without serialized type ID. |
| 2411 | `crates/horizon-world/src/attribute_health.rs` | adaptable; reference | Base LivingEntity absorption only; Player synchronized absorption and virtual dispatch excluded; missing attributes returns None. |
| 2412 | `crates/horizon-world/src/attribute_map.rs` | adaptable; reference | Callback failures preserve queued updates and may leave partial state; None keys pass to suppliers; reentrancy matters. |
| 2413 | `crates/horizon-world/src/attribute_updates.rs` | adaptable; reference | Waypoint path preserves hook order; max values cast f64 to f32 before comparison. |
| 2414 | `crates/horizon-world/src/attributes/entity.rs` | adaptable; reference | DataVersion checked but entity IDs not canonicalized; AttributeId indexes assume catalog-origin IDs. |
| 2415 | `crates/horizon-world/src/attributes/live.rs` | adaptable; reference | 416 lines read 1–400/401–416. Clones copy owner and allocator state; divergent mutation can create colliding logical handles. |
| 2416 | `crates/horizon-world/src/attributes/mod.rs` | adaptable; reference | Modifier::new accepts empty path; add_or_update_transient retains old operation bucket. Verify Java parity. |
| 2417 | `crates/horizon-world/src/attributes/operation_map.rs` | adaptable; reference | Capacity uses f64 ceil/next_power_of_two; extreme lengths may overflow but allocation constrains them. Slot order is compatibility-critical. |
| 2418 | `crates/horizon-world/src/bin/worldgen_parity.rs` | adaptable; reference | 915 lines read 1–200/201–400/401–600/601–800/801–915. CLI indexes args/bundle keys without full arity/range validation (e.g. settings_create). |
| 2419 | `crates/horizon-world/src/block_collision.rs` | adaptable; reference | PistonBlockBindings errors discarded with .ok(), conflating invalid metadata with no pistons. |
| 2420 | `crates/horizon-world/src/block_collision_scan.rs` | adaptable; reference | Wrapping arithmetic intentional; budget counts skipped cursor cells. Correctness relies on may_reach_out never false-negative. |
| 2421 | `crates/horizon-world/src/block_entities.rs` | adaptable; reference | Missing/wrong-type coordinates default to zero; Long narrows to i32; pending_mut can alter coordinates without rekeying. |
| 2422 | `crates/horizon-world/src/block_entity_push.rs` | adaptable; reference | Selection uses added-shape bounds; entity boxes re-read before each teleport; depends on stable handles/query order. |
| 2423 | `crates/horizon-world/src/block_landing.rs` | adaptable; reference | 464 lines read 1–400/401–464. Farmland RNG draw precedes entity gating; hook contracts control event order; definitions strict for all block names. |
| 2424 | `crates/horizon-world/src/block_placement.rs` | adaptable; reference | State IDs checked; relative coords wrap i32 by design; assumes complete pale-moss registry. |
| 2425 | `crates/horizon-world/src/block_properties.rs` | adaptable; reference | Every state needs definitions and occlusion shape; face mask assumes Direction discriminants fit u8. |
| 2426 | `crates/horizon-world/src/block_render.rs` | adaptable; reference | Requires every registry state to have render definition and cached shape; cache clears at 4096 pairs. Opaque face visibility only, not translucent sorting, fluids, meshing or lighting. |
| 2427 | `crates/horizon-world/src/block_state_properties.rs` | adaptable; reference | 418 lines read 1–400/401–418. Registry/schema completeness and IDs trusted at binding boundaries; properties parsed from string snapshots, not live Forge state. |
| 2428 | `crates/horizon-world/src/block_support.rs` | adaptable; reference | Cached geometry models context-free branch only; moving-piston/block-entity behavior remains callback-owned. Unsupported dynamic states return error. |
| 2429 | `crates/horizon-world/src/block_survival.rs` | adaptable; reference | 1220 lines read 1–400/401–600/601–800/801–1200/1201–1220. CactusFlower reads same below-position twice; Farmland may read above twice after solid test. Duplicate world callbacks can diverge for stateful providers. Registry/tag completeness required. |
| 2430 | `crates/horizon-world/src/blocking_item_damage.rs` | adaptable; reference | Stack identity and server-player state are callback-owned; ordering is contractual. Does not decode components or sounds. |
| 2431 | `crates/horizon-world/src/blocking_rules.rs` | adaptable; reference | NaN/signed-zero explicit; round uses f64 intermediate on f32 input; angle is caller-supplied radians. |
| 2432 | `crates/horizon-world/src/bundle_weight.rs` | adaptable; reference | Weight cached in Rc OnceCell and permanently captures first ItemWeights view; mutations to that view leave stale weight. Fraction constructor does not fully normalize immediately. |
| 2433 | `crates/horizon-world/src/chunk_status.rs` | adaptable; reference | by_name maps unknown names to Empty; parse/from_nbt return errors. Status assignment does not execute generation or lighting. |
| 2434 | `crates/horizon-world/src/climbing.rs` | adaptable; reference | Only positive result updates last-climbable position; negative checks preserve old value. Facing limited to horizontal directions. |
| 2435 | `crates/horizon-world/src/collision.rs` | adaptable; reference | Context-free branch only; uncached states must invoke block behavior. Older exports without canonical identity retain local aliases only. |
| 2436 | `crates/horizon-world/src/collision_context.rs` | adaptable; reference | with_position(None,y) ignores y and uses -MAX; can_stand_on_fluid calls entity callback before same-family test. |
| 2437 | `crates/horizon-world/src/combat_rules.rs` | adaptable; reference | NaN/signed-zero behavior explicit; depends on correct weapon/server-level callbacks. |
| 2438 | `crates/horizon-world/src/component_durability.rs` | adaptable; reference | Raw components remain while stack empty; grow/shrink wrap integers. Only one component family, not full ItemStack. |
| 2439 | `crates/horizon-world/src/component_patch.rs` | adaptable; reference | set mutates patch before returning MissingPrototype for unsanitized removal without prototype; partial mutation documented, needs native exception parity. |
| 2440 | `crates/horizon-world/src/conditions.rs` | adaptable; reference | Missing/non-array `values` in `and` becomes empty iterator and passes via vacuous all; malformed shape may be accepted instead of rejected by codec. |
| 2441 | `crates/horizon-world/src/consumable.rs` | adaptable; reference | emit unconditionally draws nextBoolean, triangleFloat and nextFloat before choosing Eat/Drink. Java conditional path likely draws only selected animation values; likely over-consumes RNG. |
| 2442 | `crates/horizon-world/src/consume_effect.rs` | adaptable; reference | NaN probability passes because random>=NaN is false; usual Java random<probability predicate returns false. Confirm source oracle. |
| 2443 | `crates/horizon-world/src/damage_immunity.rs` | adaptable; reference | Player rule loop returns on first matching tag; Drowning/Fall/Fire/Freezing order matters and Fall tag is also used in base immunity. |
| 2444 | `crates/horizon-world/src/damage_reduction.rs` | adaptable; reference | Resistance amplifier wraps i32; resisted-stat award uses distinct victim/attacker server-player lookups. |
| 2445 | `crates/horizon-world/src/damage_source.rs` | adaptable; reference | Immutable catalog snapshot; unresolved required tags fail. LivingNonPlayer intentionally excludes Player. |
| 2446 | `crates/horizon-world/src/datafix/block_state_data.txt` | adaptable; reference | 1703 lines read 1–400/401–800/801–1200/1201–1600/1601–1703. 1695 rows; IDs strictly increasing/no duplicates; no malformed columns/IDs/property order; max 81 aliases per row. Generated from vanilla 26.3, not 1.20.1. |
| 2447 | `crates/horizon-world/src/datafix/chunk_leaves.rs` | adaptable; reference | 488 lines read 1–400/401–488. Trapped-chest local x/y/z pass to get_index without explicit bounds; malformed coordinates can alias another local index. Bad palette/storage can make section skippable. |
| 2448 | `crates/horizon-world/src/datafix/dynamic.rs` | adaptable; reference | Number narrowing follows Java-style casts; stream clones lists and expands arrays. String conversions may be lossy for unpaired UTF-16. |
| 2449 | `crates/horizon-world/src/datafix/files.rs` | adaptable; reference | 1147 lines read 1–200/201–400/401–600/601–800/801–1000/1001–1147. files_with_suffix/subdirectories suppress read_dir errors into empty lists, silently skipping inaccessible world data. Legacy chunk_x±8 ranges may overflow i32. Upgrade targets 26.3, not a 1.20.1 loader. |
| 2450 | `crates/horizon-world/src/datafix/fixes_a.rs` | adaptable; reference | 1116 lines read 1–200/201–400/401–600/601–800/801–1000/1001–1116. Multiple fixes share same version/substep keys and order is semantic; chested-horse slot conversion subtracts unchecked numeric values. |
| 2451 | `crates/horizon-world/src/datafix/fixes_b.rs` | adaptable; reference | Multiple fixes share version keys and order is semantic. UUID conversion helpers require both long halves nonzero; malformed/valid UUIDs with one zero half may be lost. Some unknown IDs collapse to fallback names. |
| 2452 | `crates/horizon-world/src/datafix/fixes_c.rs` | adaptable; reference | Version-specific assumptions include fixed biome container layouts and old tick/height encodings; malformed lists may be replaced by defaults. Some keys are repeated across ordered substeps. |
| 2453 | `crates/horizon-world/src/datafix/fixes_cliffs.rs` | adaptable; reference | 1623 lines read 1–400/401–800/801–1200/1201–1623. Tick conversion skips packed position <=0, which may discard valid local origin (0,0,0); confirm native semantics. Coordinate differences use abs and may hit i32::MIN edge behavior. |
| 2454 | `crates/horizon-world/src/datafix/fixes_flat.rs` | adaptable; reference | 1224 lines read in 1–400/401–600/601–800/801–1200/1201–1224. Unknown flat structure names can fall back to a default config; palette and biome conversions assume known version schemas. |
| 2455 | `crates/horizon-world/src/datafix/fixes_legacy.rs` | adaptable; reference | 787 lines read 1–400/401–787. UUID parser accepts variable component widths and masks overflow rather than enforcing canonical UUID groups; recursive Riding chains can exhaust stack on adversarial depth. |
| 2456 | `crates/horizon-world/src/datafix/fixes_mid.rs` | adaptable; reference | 1815 lines read 1–400/401–800/801–1200/1201–1600/1601–1815. UUID-from-longs returns None when either half is zero; may drop valid UUIDs. Packed-storage alignment and boundary behavior need exact 1.18-era oracle tests. |
| 2457 | `crates/horizon-world/src/datafix/fixes_old.rs` | adaptable; reference | 714 lines read 1–400/401–714. Several conversions silently default unknown enum/variant values; recursive entity tree transformations assume finite depth and well-formed compounds. |
| 2458 | `crates/horizon-world/src/datafix/flat_tables.rs` | adaptable; reference | 2791 lines checked in <=400-line ranges; static tuple tables parsed for duplicate keys, none found in checked tuple maps. Contains apparent `minecraft:enhanting_table` misspelling in legacy item-to-block-entity mapping; verify if source historical typo or broken lookup. |
| 2459 | `crates/horizon-world/src/datafix/items.rs` | adaptable; reference | 1411 lines read in 1–400/401–600/601–800/801–1000/1001–1200/1201–1411. Malformed map-decoration IDs can fall back to empty key instead of leaving stack untouched; typo `enhanting_table` appears in legacy mapping path. |
| 2460 | `crates/horizon-world/src/datafix/json.rs` | adaptable; reference | 644 lines read 1–400/401–644. Strict quoted parser appears to accept escaped apostrophe; UTF-16 escape conversion rejects lone surrogates; lenient top-level null may return early despite trailing input. Exact-long exponent arithmetic has extreme i64 overflow risk. |
| 2461 | `crates/horizon-world/src/datafix/progress.rs` | adaptable; reference | 282 lines; relaxed atomics are used consistently for independent counters; poisoned dimension mutex silently drops updates and reads as zero. |
| 2462 | `crates/horizon-world/src/datafix/schema.rs` | adaptable; reference | 947 lines read 1–400/401–600/601–800/801–947. Schema registration and repeated key order are critical; recursive tree walking assumes finite NBT depth. Unknown IDs intentionally remain untyped. |
| 2463 | `crates/horizon-world/src/datafix/schema_flat.rs` | adaptable; reference | 508 lines read 1–400/401–508. MARKING thread-local bool is set/reset manually; panic during walk can leave it true for later work on same thread. Nested mark_untyped calls can also clear outer state. |
| 2464 | `crates/horizon-world/src/datafix/text.rs` | adaptable; reference | 252 lines; URI validator approximates java.net.URI and chat-character filter uses UTF-16 units; malformed legacy hover payloads are converted to display text rather than rejected. |
| 2465 | `crates/horizon-world/src/datafix/world_upgrader_tests.rs` | adaptable; reference | 298 lines; test explicitly confirms a current/skipped chunk short-circuits the rest of that region-file pass, so an older later chunk remains unconverted. Mixed-version chunks can therefore leave stale data unless native behavior is intentional. |
| 2466 | `crates/horizon-world/src/dynamic_support.rs` | adaptable; reference | Shulker collision uses snapshot bounding box while support uses open/closed state callback; Direction discriminants index fixed six-element shape array. |
| 2467 | `crates/horizon-world/src/edge_backoff.rs` | adaptable; reference | 108 lines; max_up_step is not validated for negative/NaN values; query budget exhaustion returns error after earlier world callbacks have already occurred. |
| 2468 | `crates/horizon-world/src/effect_attribute_modifiers.rs` | adaptable; reference | Re-reads indexed entry after remove callback in add_modifiers; remove_modifiers does not re-read because it has no later use. Cursor invalidation is surfaced explicitly. |
| 2469 | `crates/horizon-world/src/effect_notifications.rs` | adaptable; reference | Immunity tags are exclusive priority order (Infested before Oozing before Poison/Regeneration). Packet send loops query current entity ID per passenger; stable connection captured before virtual call. |
| 2470 | `crates/horizon-world/src/effect_particles.rs` | adaptable; reference | Catalog validates DataVersion and ambient alpha; equality is pointer identity, so fresh equal-color options remain unequal by design. |
| 2471 | `crates/horizon-world/src/effect_sync.rs` | adaptable; reference | Flag reads sign-extend u8 through i8 before bit tests; shift uses wrapping_shl to mirror Java int shift masking. Empty effects clear particles/invisibility but ambient flag is not rewritten on that branch. |
| 2472 | `crates/horizon-world/src/entity_collision.rs` | adaptable; reference | No explicit candidate budget; broadphase may return supersets, but snapshot handles must remain available through callbacks. |
| 2473 | `crates/horizon-world/src/entity_fluid.rs` | adaptable; reference | 454 lines read 1–400/401–454. Query budget counts chunk, section and fluid cells; tracker/current writes persist if later provider call fails. Float NaN behavior uses Java-compatible max helper. |
| 2474 | `crates/horizon-world/src/entity_fluid_tick.rs` | adaptable; reference | is_pushed_by_fluid queried twice by design; water/lava membership captured before splash callbacks, preserving native phase ordering. |
| 2475 | `crates/horizon-world/src/entity_move.rs` | adaptable; reference | No-physics branch sets position/record/flags then returns without later fall/ground/emission callbacks. MovementHistory compresses oldest two at 100 entries; exact +0.0 check distinguishes negative zero. |
| 2476 | `crates/horizon-world/src/entity_obstruction.rs` | adaptable; reference | AABB query and entity-state callbacks are ordered; no explicit candidate budget or unloaded-chunk policy in this adapter. |
| 2477 | `crates/horizon-world/src/extra_items.rs` | adaptable; reference | Static identifiers are version-bound and assume registry IDs are supplied by matching data. |
| 2478 | `crates/horizon-world/src/fall_damage.rs` | adaptable; reference | Float NaN/infinity and negative fall distance follow Rust/Java comparison/cast behavior; callback ordering controls damage and reset. |
| 2479 | `crates/horizon-world/src/fluid.rs` | adaptable; reference | FluidState::new is public and accepts any u8 amount; FluidShapeCache indexes directly from it, so amount >8 can panic/out-of-bounds. from_json also defaults missing falling to true for nonempty fluid; verify default-state semantics. |
| 2480 | `crates/horizon-world/src/fluid_box.rs` | adaptable; reference | Assumes amount 0–8 and canonical fluid shape registry; callers can construct invalid FluidState unless constructor validates. |
| 2481 | `crates/horizon-world/src/fluid_flow.rs` | adaptable; reference | Neighbor reads and fluid-family comparisons are callback-driven; tie and zero-vector handling need exact float parity. |
| 2482 | `crates/horizon-world/src/fluid_occupancy.rs` | adaptable; reference | Uses state IDs as indexes into registry presence arrays; invalid IDs can panic. Chunk/section iteration and query budget are callback-sensitive. |
| 2483 | `crates/horizon-world/src/fluid_shape.rs` | adaptable; reference | Indexing assumes amount <=8; invalid states can exceed the fixed table. |
| 2484 | `crates/horizon-world/src/food.rs` | adaptable; reference | Effect RNG/callback order and food-level checks are external hooks; a callback error may leave earlier food mutation committed. |
| 2485 | `crates/horizon-world/src/food_properties.rs` | adaptable; reference | Effect probability and numeric values are data-driven; malformed component shapes rely on decoder defaults/errors. |
| 2486 | `crates/horizon-world/src/food_storage.rs` | adaptable; reference | Float comparisons and exhaustion carry are compatibility-sensitive; fields are public and may be externally inconsistent. |
| 2487 | `crates/horizon-world/src/free_space.rs` | adaptable; reference | Only supplied candidate positions and collision provider policy are covered; no chunk-loading guarantee. |
| 2488 | `crates/horizon-world/src/ground_support.rs` | adaptable; reference | Ground support may use a snapshot of entity position and block state; moving platform and unloaded-chunk behavior remain external. |
| 2489 | `crates/horizon-world/src/heightmap.rs` | adaptable; reference | Packed bit width and section/minY offsets are version-sensitive; malformed or short long arrays may decode with fallback values rather than hard error. |
| 2490 | `crates/horizon-world/src/identifier.rs` | adaptable; reference | Validation must match Java ResourceLocation exactly; empty path and uppercase handling are compatibility-sensitive. |
| 2491 | `crates/horizon-world/src/interaction_shapes.rs` | adaptable; reference | Strictly rejects partial tables; shape lookup returns error for missing state. No contextual interaction shapes are supported by design. |
| 2492 | `crates/horizon-world/src/inventory_pickup.rs` | adaptable; reference | Return value after partial pickup appears wrong: loop exits on final no-progress iteration and compares against that iteration's count, so a partially inserted stack can return false. Damaged stack is copy-cleared before target slot validation. |
| 2493 | `crates/horizon-world/src/inventory_tick.rs` | adaptable; reference | Level is fetched separately for each nonempty stack; equipment reads each entry when reached to preserve callback mutation visibility. |
| 2494 | `crates/horizon-world/src/inventory_use.rs` | adaptable; reference | 485 lines read 1–400/401–485. set_hand_item mutates inventory before on_equip callback; flags observer errors leave the flag mutation committed. Reentrancy and same-stack aliasing are core contracts. |
| 2495 | `crates/horizon-world/src/item_after_use.rs` | adaptable; reference | Remainder conversion may mutate result and emit extra item before cooldown; callback failures preserve preceding mutations. |
| 2496 | `crates/horizon-world/src/item_blocking.rs` | adaptable; reference | Direct entity is queried once for piercing and again after hurt; callback mutation may change entity identity between phases. Native virtual-call order must be preserved. |
| 2497 | `crates/horizon-world/src/item_cooldowns.rs` | adaptable; reference | Duration zero yields 0/0 NaN in percent; negative/overflowing duration relies on Java int wrap. Observer mutation may invalidate cursor. |
| 2498 | `crates/horizon-world/src/item_defaults.rs` | adaptable; reference | stack_size uses as_f64 then casts to i64, truncating fractional JSON values (e.g. 2.9→2) although vanilla ExtraCodecs.intRange expects integer; verify and reject non-integral values. |
| 2499 | `crates/horizon-world/src/item_durability.rs` | adaptable; reference | is_damageable requires explicit damage component Some, while vanilla may determine damageability from maxDamage>0 and !unbreakable and default missing Damage to zero. This could make pristine damageable items non-damageable. |
| 2500 | `crates/horizon-world/src/item_particles.rs` | adaptable; reference | Uses actor's direct random field rather than virtual getRandom; repeated pitch/yaw/position/eye/level callbacks per particle are order-sensitive. |
| 2501 | `crates/horizon-world/src/item_stack_refs.rs` | adaptable; reference | Shared mutable StackRef handles; pointer identity is distinct from value equality, and the shared empty reference is publicly writable. |
| 2502 | `crates/horizon-world/src/item_stack_storage.rs` | adaptable; reference; version-sensitive | Stack split/grow/shrink/copy operations use wrapping integer arithmetic and direct mutable fields; negative and overflow boundaries need Java 1.20.1 parity. |
| 2503 | `crates/horizon-world/src/item_template.rs` | adaptable; reference; component-version-specific | Immutable item templates apply component patches and strict stack validation; direct adoption would import 26.3 component semantics. |
| 2504 | `crates/horizon-world/src/item_use.rs` | adaptable; reference | Item blocking/use-duration calculations are compact but re-query the held item across multiple virtual callbacks. |
| 2505 | `crates/horizon-world/src/item_use_catalog.rs` | reference; version-specific; do not ship | Build-time item-use catalogue parses 26.3 JSON tables, but duration floats and schema data require target-version validation. |
| 2506 | `crates/horizon-world/src/item_use_lifecycle.rs` | adaptable; reference | Item-use tick/release/complete/stop ordering is callback-driven and can leave partial state after callback errors. |
| 2507 | `crates/horizon-world/src/item_use_start.rs` | adaptable; reference | Starting use mutates held-stack state before later duration/level callbacks, with no rollback on callback failure. |
| 2508 | `crates/horizon-world/src/java_hash_map.rs` | reference; compatibility-only; do not ship | Custom Java HashMap models bucket order, tree bins and fail-fast cursors; copy-capacity and collision paths need differential proof. |
| 2509 | `crates/horizon-world/src/json_folder.rs` | adaptable; reference; filesystem-hardening required | Folder resource enumeration is cached and sorted, but recursive traversal follows symlinks and suppresses some nested read errors. |
| 2510 | `crates/horizon-world/src/lazy_json.rs` | adaptable; reference | Small OnceLock-backed JSON wrapper provides lazy parse and clone behavior; serialization round-trip costs and numeric fidelity need profiling. |
| 2511 | `crates/horizon-world/src/level/bonemeal.rs` | adaptable; worldgen-version-specific; reference | Large bonemeal/growth implementation integrates features and templates, but several error/random/bounds paths need hardening and Java parity. |
| 2512 | `crates/horizon-world/src/level/buckets.rs` | adaptable; reference; API-validation required | Bucket placement/pickup covers fluids, powder snow and mob buckets; invalid FluidKind::Empty is treated as lava and placement results need checking. |
| 2513 | `crates/horizon-world/src/level/classes.rs` | reference; version-specific; do not ship | Java class/interface mapping drives many dispatches and contains modern classes beyond 1.20.1; IDs and inheritance need version-owned generation. |
| 2514 | `crates/horizon-world/src/level/creaking.rs` | not applicable to 1.20.1; do not ship | Creaking-heart behavior is a later-version feature and not part of the 1.20.1 target. |
| 2515 | `crates/horizon-world/src/level/data.rs` | reference; rewrite registry for 1.20.1 | Large registry/facts loader has permissive defaults and modern named blocks; regenerate target-version data and tighten validation. |
| 2516 | `crates/horizon-world/src/level/data/cache.rs` | adaptable; cache-hardening required; reference | Snapshot cache has checksums and size limits but can allocate before checking file length and uses metadata-only source stamps. |
| 2517 | `crates/horizon-world/src/level/falling.rs` | adaptable; reference; entity-integration required | Falling-block transitions cover landing and stalactite damage, but actual entity simulation and drop outcomes are delegated to the host. |
| 2518 | `crates/horizon-world/src/level/feature_world.rs` | adaptable; reference | FeatureWorld adapts live features to LevelAccess, but hardcodes sea levels and scans columns instead of relying on target heightmaps. |
| 2519 | `crates/horizon-world/src/level/fluids.rs` | adaptable; core candidate; 1.20.1 verification required | Fluid spreading and pickup/placement logic is a strong core reference, but coordinate arithmetic, fallbacks and placement results need hardening. |
| 2520 | `crates/horizon-world/src/level/growth.rs` | adaptable; worldgen-version-specific; reference | Tree/mushroom growth integrates live features, but reports partial jigsaw failure as success and uses silent random fallbacks. |
| 2521 | `crates/horizon-world/src/level/growth_tests.rs` | fixture-only; do not ship | Seeded integration fixtures exercise growth and interactions, but tests skip without resources and use a simplified always-loaded world. |
| 2522 | `crates/horizon-world/src/level/hooks.rs` | adaptable; reference; version-specific | Class-dispatched lifecycle hooks cover neighbor/place/removal/entity events, with exact dispatch order and host-side effects as critical contracts. |
| 2523 | `crates/horizon-world/src/level/interact.rs` | adaptable; reference; player-integration required | Block interaction routing and effects are comprehensive, but hit-slot inputs and empty-hand fallback require validation. |
| 2524 | `crates/horizon-world/src/level/mod.rs` | adaptable; architectural reference | Defines the LevelAccess host boundary and event contract; numerous defaults can silently yield incorrect behavior if the host omits overrides. |
| 2525 | `crates/horizon-world/src/level/orientation.rs` | adaptable; reference; redstone-experiment-gated | 48-context orientation mapping is compact and invertible by design; assertions and exact side-bias order need exhaustive verification. |
| 2526 | `crates/horizon-world/src/level/pistons.rs` | adaptable; reference; entity-integration required | Preserve ordered piston move plans and block events; entity movement remains a host responsibility. |
| 2527 | `crates/horizon-world/src/level/place.rs` | adaptable; reference; Forge-placement-hooks required | Placement covers many block families, but paired writes and Forge placement hooks need transactional parity. |
| 2528 | `crates/horizon-world/src/level/portal.rs` | adaptable; reference; partial-write validation required | Portal frame detection is useful; multi-block placement refusal may leave partial state. |
| 2529 | `crates/horizon-world/src/level/portal_travel.rs` | adaptable; reference; dimension-and-entity integration required | Portal search/exit geometry is reusable only with correct entity collision and dimension-transfer ownership. |
| 2530 | `crates/horizon-world/src/level/query.rs` | adaptable; reference; dynamic-block-entity integration required | Shared support queries are useful but must receive dynamic block-entity collision facts. |
| 2531 | `crates/horizon-world/src/level/rails.rs` | adaptable; reference; minecart-integration required | Rail updates need exact direction/order parity and bounded minecart/entity queries. |
| 2532 | `crates/horizon-world/src/level/redstone.rs` | adaptable; core candidate; Java parity required | Strong world-core candidate; redstone ordering, RNG and target class data must be proven. |
| 2533 | `crates/horizon-world/src/level/redstone/experimental.rs` | reference; experimental-feature-gated; do not enable by default | Keep experimental redstone separate and disabled unless the 1.20.1 feature gate is proven. |
| 2534 | `crates/horizon-world/src/level/removal.rs` | adaptable; reference; block-entity-and-drop integration required | Removal side effects need host-owned loot, inventory, item and XP entity handling. |
| 2535 | `crates/horizon-world/src/level/sculk.rs` | adaptable; reference; version-and-RNG parity required | Sculk charge/cursor mechanics are reusable after 1.20.1 tag and RNG parity. |
| 2536 | `crates/horizon-world/src/level/shapes.rs` | adaptable; reference; target-version class-dispatch required | Update-shape dispatcher requires target-version class tables and preserved order. |
| 2537 | `crates/horizon-world/src/level/shelf.rs` | not applicable to 1.20.1; do not ship | Shelf behavior is absent from vanilla 1.20.1; exclude from the target core. |
| 2538 | `crates/horizon-world/src/level/signals.rs` | adaptable; reference; redstone-host-contract required | Small but order-sensitive bridge for analog/redstone output notifications. |
| 2539 | `crates/horizon-world/src/level/speleothem.rs` | adaptable; reference; fluid-RNG-and-entity integration required | Dripstone rules fit the target era but random, fluid and entity effects need parity. |
| 2540 | `crates/horizon-world/src/level/states.rs` | adaptable; registry-critical; 1.20.1 generation required | Registry-critical arithmetic requires exact 1.20.1 state enumeration and validation. |
| 2541 | `crates/horizon-world/src/level/survive.rs` | adaptable; reference; target-version class-and-tag map required | Survival rules depend on exact class-chain order, target tags and dynamic support. |
| 2542 | `crates/horizon-world/src/level/tests.rs` | fixture-only; do not ship | Fixture-only tests skip when resources are missing; do not ship or treat green skips as evidence. |
| 2543 | `crates/horizon-world/src/level/tick.rs` | adaptable; core candidate; 1.20.1 tick parity required | Core tick candidate; deterministic scheduling, random draws and masks require Java parity. |
| 2544 | `crates/horizon-world/src/level/ticks.rs` | adaptable; core candidate; persistence-and-order verification required | Useful deterministic tick-state container; chunk lifecycle and persistence remain host-owned. |
| 2545 | `crates/horizon-world/src/level/timeline.rs` | version-specific; selective reference; later blocks excluded | Only selectively relevant; eyeblossom and dried-ghast behavior is newer than 1.20.1. |
| 2546 | `crates/horizon-world/src/level/update.rs` | adaptable; core candidate; host-contract required | Central set-block/update pipeline candidate; make update-budget and mutation failures observable. |
| 2547 | `crates/horizon-world/src/level/weather.rs` | adaptable; reference; biome-heightmap integration required | Weather query reference; target biome tables, sea level and heightmaps must be supplied. |
| 2548 | `crates/horizon-world/src/lib.rs` | architectural reference; version-specific; do not copy wholesale | Navigation only; do not copy the modern crate module surface wholesale. |
| 2549 | `crates/horizon-world/src/liquid_collision.rs` | adaptable; reference; entity-collision integration required | Contextual liquid collision bridge; entity shape and standing rules belong to the host. |
| 2550 | `crates/horizon-world/src/live_attribute_updates.rs` | adaptable; reference; entity-attribute-host integration required | Attribute refresh dispatch belongs in a separate entity subsystem with explicit callback lifecycle. |
| 2551 | `crates/horizon-world/src/living_air.rs` | adaptable; reference | Default airborne travel, relative input, ladder clamping and friction calculations. |
| 2552 | `crates/horizon-world/src/living_damage.rs` | adaptable; reference | Inner living/player damage application after immunity and outer hit gates. |
| 2553 | `crates/horizon-world/src/living_fall.rs` | adaptable; version-specific; reference | Fall-distance damage, passenger propagation and impulse-based fall-damage exemptions. |
| 2554 | `crates/horizon-world/src/living_fluid.rs` | adaptable; reference | Water/lava travel, fluid escape and ridden-entity buoyancy. |
| 2555 | `crates/horizon-world/src/living_health.rs` | adaptable; reference | Living health/absorption limits, healing and synchronized-float dirty-state behavior. |
| 2556 | `crates/horizon-world/src/living_hurt.rs` | adaptable; version-specific; reference | Outer living damage dispatch, cooldown state and post-hit services. |
| 2557 | `crates/horizon-world/src/living_landing.rs` | Rebuild from 1.20.1 Entity.checkFallDamage/LivingEntity landing and Forge block callbacks. | Living landing fluid refresh, changed-block notification, landing particles and fall handling. |
| 2558 | `crates/horizon-world/src/living_motion.rs` | adaptable; reference | Default living jump, gravity, flying-speed and air-drag calculations. |
| 2559 | `crates/horizon-world/src/living_use_flags.rs` | adaptable; version-specific; reference | Living-entity using-item and used-hand flags in synchronized byte storage. |
| 2560 | `crates/horizon-world/src/loot.rs` | rewrite required; version-specific; reference | Large loot engine: data-pack overlays, pools/entries, conditions, modifiers, enchantments, stack splitting and entity context. |
| 2561 | `crates/horizon-world/src/main.rs` | adaptable; optional; reference | Standalone CLI for region/NBT inspection, copying, block-data editing and codec conversion. |
| 2562 | `crates/horizon-world/src/movement_emission.rs` | adaptable; reference | Movement distance accounting and step/swim/flap sound/event dispatch. |
| 2563 | `crates/horizon-world/src/movement_properties.rs` | adaptable; version-specific; reference | Registry-bound block movement coefficients and reloadable movement tags. |
| 2564 | `crates/horizon-world/src/movement_restitution.rs` | adaptable; reference | Post-collision bounce/restitution response and side-effect dispatch. |
| 2565 | `crates/horizon-world/src/movement_statistics.rs` | adaptable; reference | Server-player movement stats and movement-related exhaustion. |
| 2566 | `crates/horizon-world/src/movement_support.rs` | Recreate 1.20.1 support and fence/wall/gate semantics; regenerate water/bubble/tag identities. | Support-block position and block-derived jump/speed factors. |
| 2567 | `crates/horizon-world/src/native_load_hooks.rs` | unnecessary; reference; privileged native-extension model only | Process-wide callbacks for native mod data-loading and loot behavior. |
| 2568 | `crates/horizon-world/src/object_open_map.rs` | adaptable; reference | Fastutil-style object open-addressing map with nullable keys, indexed entries, cursors and mutable traversal. |
| 2569 | `crates/horizon-world/src/object_open_set.rs` | adaptable; reference | Fastutil-style object open-addressing set with nullable member and live mutation-aware cursors. |
| 2570 | `crates/horizon-world/src/piston_movement.rs` | adaptable; reference | Per-tick piston displacement accumulation and limiting. |
| 2571 | `crates/horizon-world/src/piston_push.rs` | adaptable; reference | Moving piston push geometry, sticky honey selection, slime bounce and piston-base recovery. |
| 2572 | `crates/horizon-world/src/piston_shape.rs` | adaptable; version-specific; reference | Moving-piston collision-shape selection, head interpolation and registry-bound piston-state mapping. |
| 2573 | `crates/horizon-world/src/player_equipment.rs` | adaptable; reference | Player hand/equipment slot access and equip callback dispatch. |
| 2574 | `crates/horizon-world/src/player_food.rs` | adaptable; reference | Player-specific hunger guards, exhaustion and direct-versus-virtual abilities. |
| 2575 | `crates/horizon-world/src/player_health.rs` | adaptable; version-specific; reference | Player-specific health/absorption bridge to attributes and synchronized floats. |
| 2576 | `crates/horizon-world/src/player_hurt.rs` | Rebuild 1.20.1 Player.hurtServer, difficulty and server PvP rules; verify Forge events. | Player hurt, difficulty scaling and PvP gates. |
| 2577 | `crates/horizon-world/src/player_inventory.rs` | Rebuild 1.20.1 inventory/equipment layout and pre-component ItemStack; exclude Body/Saddle. | Player inventory slots, selected hotbar and shared equipment/stack references. |
| 2578 | `crates/horizon-world/src/player_travel.rs` | Rebuild 1.20.1 Player.travel and swimming; check Forge overrides. | Player swimming and ability-flight adjustment around living travel. |
| 2579 | `crates/horizon-world/src/powder_snow_collision.rs` | Verify 1.20.1 PowderSnowBlock/CollisionContext and tag semantics. | Context-dependent powder-snow collision shape and entity walkability. |
| 2580 | `crates/horizon-world/src/registry/facts.rs` | Create a new 1.20.1 facts schema/data corpus; do not use 26.3 facts file. | Streaming deserializer for compact registry facts and shared state-property strings. |
| 2581 | `crates/horizon-world/src/registry/json.rs` | Regenerate 1.20.1 property kinds and confirm target compact/full codec semantics. | Compact/full block-state JSON codec over a registry. |
| 2582 | `crates/horizon-world/src/registry/snapshot.rs` | Version/hash-bind cache to 1.20.1 plus Forge and generated registry digest. | Compact registry snapshot format for states, shared strings, flags and lookup tables. |
| 2583 | `crates/horizon-world/src/registry_collision_scan.rs` | Regenerate 1.20.1 registry shapes/tags and block-entity collision callbacks. | Registry-bound collision scanning over resident chunks. |
| 2584 | `crates/horizon-world/src/registry_fall_ray.rs` | Rebuild 1.20.1 reset tags/portals/ray semantics and cooldown. | Registry-bound fall-reset ray behavior for reset blocks, portals and water. |
| 2585 | `crates/horizon-world/src/registry_fluid.rs` | Regenerate exact 1.20.1 fluid registries, ice and water/flow tags; verify Forge fluids. | Registry fluid contents, flow obstruction, ice membership and entity immersion. |
| 2586 | `crates/horizon-world/src/retrogen.rs` | Rebuild 1.20.1 BelowZeroRetrogen, chunk statuses and dimension heights. | Below-zero retrogen metadata, bedrock masks, terrain transforms and biome resolution. |
| 2587 | `crates/horizon-world/src/server_player_combat.rs` | Rebuild 1.20.1 ServerPlayer damage and Forge armor/damage event flow; enforce immutable schema. | Composed server-player hurt pipeline with synchronized health, armor and world services. |
| 2588 | `crates/horizon-world/src/server_player_damage.rs` | Use 1.20.1 attribute set, damage reduction, exhaustion and Forge events. | Server-player inner damage bridge to armor, mitigation, health/absorption, food and stats. |
| 2589 | `crates/horizon-world/src/simple_container.rs` | Use 1.20.1 NBT ItemStack matching and Forge container hooks; no data components. | Variable-size SimpleContainer storage, stack transfer and change callbacks. |
| 2590 | `crates/horizon-world/src/snbt_text.rs` | Rebuild exact 1.20.1 SNBT StringTagVisitor and Java numeric conversion. | SNBT tag stringification and Java-like float/double formatting. |
| 2591 | `crates/horizon-world/src/status_effect.rs` | Rebuild 1.20.1 MobEffectInstance fields, hidden chain and Forge effect hooks. | Shared status effects, hidden chains, ticking and client blend interpolation. |
| 2592 | `crates/horizon-world/src/stepped_interpolation.rs` | Rebuild 1.20.1 entity update intervals and movement packet semantics. | Client interpolation of remote living entities along stepped server position/rotation paths. |
| 2593 | `crates/horizon-world/src/stored_consume.rs` | Rebuild 1.20.1 consumable/effect add-remove and sound semantics; use target random streams. | Adapter wiring consume effects to shared active-effect storage. |
| 2594 | `crates/horizon-world/src/stored_effect_attributes.rs` | Regenerate 1.20.1 effect modifiers, UUIDs and operations; Forge custom attributes. | Adapter for effect attribute modifier dispatch on captured attribute map instances. |
| 2595 | `crates/horizon-world/src/suffocation.rs` | Recreate 1.20.1 suffocation rules/tags and piston/shulker behavior. | Registry-driven suffocation rules for full collision, piston and shulker blocks. |
| 2596 | `crates/horizon-world/src/swimming.rs` | Rebuild 1.20.1 swimming and fluid jump threshold; confirm Forge abilities overrides. | Entity swimming transitions, player flying guard and fluid jump threshold. |
| 2597 | `crates/horizon-world/src/synced_floats.rs` | Use 1.20.1 serializer registry and generated field IDs. | Float-specific aliases for generic synchronized values. |
| 2598 | `crates/horizon-world/src/synced_values.rs` | Rebuild 1.20.1 SynchedEntityData serializers, IDs and packet dirty behavior. | Generic synchronized values, dirty flags and observer callback semantics. |
| 2599 | `crates/horizon-world/src/teleport_consume.rs` | Rebuild 1.20.1 chorus fruit/teleport and Forge event ordering; gate modern extras. | Random teleport consumable candidate generation, hook veto/mutation and success effects. |
| 2600 | `crates/horizon-world/src/tick_data.rs` | Validate 1.20.1 ChunkSerializer saved tick schema and priority mapping. | NBT codec for saved scheduled block/fluid ticks. |
| 2601 | `crates/horizon-world/src/use_remainder.rs` | adaptable; reference; target ItemStack/use-hook parity required | Stack identity and count-based remainder creation. |
| 2602 | `crates/horizon-world/src/view_direction.rs` | adaptable; reference; JVM trig and acos parity required | Pitch/yaw view vector and horizontal blocking-angle math. |
| 2603 | `crates/horizon-world/src/water_splash.rs` | adaptable; reference; random/event/particle parity required | Water-entry sound, bubble/splash particles and game event. |
| 2604 | `crates/horizon-world/src/world_movement.rs` | rewrite required; reference; rebuild 1.20.1 collision and step pipeline | Block/entity/border collision aggregation, step resolution and ground reach. |
| 2605 | `crates/horizon-world/src/world_ray.rs` | rewrite required; reference; rebuild target ray clipping and portal reset | Finite block/fluid ray clipping and fall-reset shape selection. |
| 2606 | `crates/horizon-world/src/worldgen.rs` | rewrite required; version-specific; codec is Java 26.3 | Worldgen module surface and JSON/noise helpers for Java 26.3. |
| 2607 | `crates/horizon-world/src/worldgen/anchor.rs` | adaptable; reference; verify target anchor bounds and context | Vertical generation anchors and worldgen Y context. |
| 2608 | `crates/horizon-world/src/worldgen/aquifer.rs` | rewrite required; version-specific; target aquifer global-fluid behavior | Global fluid picker, disabled aquifer and FluidStatus. |
| 2609 | `crates/horizon-world/src/worldgen/aquifer/config.rs` | rewrite required; version-specific; target aquifer roots differ | Six aquifer density roots and compiled sampler binding. |
| 2610 | `crates/horizon-world/src/worldgen/aquifer/noise_based.rs` | rewrite required; version-specific; rebuild 1.20.1 aquifer and RNG | Stateful noise-based aquifer grid, pressure and fluid scheduling for 26.3. |
| 2611 | `crates/horizon-world/src/worldgen/bamboo.rs` | adaptable; reference; verify config, tags, RNG and writes | Bamboo configured feature with podzol patch, trunk and caps. |
| 2612 | `crates/horizon-world/src/worldgen/biome.rs` | adaptable; reference; regenerate target multi-noise tables/codecs | Imported multi-noise biome index and worker resolver. |
| 2613 | `crates/horizon-world/src/worldgen/biome_chunk.rs` | adaptable; reference; verify target quart storage, sections and retrogen | Quart biome chunk storage, atomic section fill, zoom and retrogen. |
| 2614 | `crates/horizon-world/src/worldgen/biome_predicate.rs` | adaptable; reference; bind target biome tags and identity | Biome names/tags resolved to immutable holder-ID sets. |
| 2615 | `crates/horizon-world/src/worldgen/biome_source.rs` | adaptable; version-specific; verify target fixed/checkerboard/End behavior | Fixed, checkerboard and End biome-source sampling. |
| 2616 | `crates/horizon-world/src/worldgen/biome_source/codec.rs` | adaptable; reference; verify target holder codec and preset JSON | Holder-aware biome-source codecs for variants and presets. |
| 2617 | `crates/horizon-world/src/worldgen/biome_source/runtime.rs` | adaptable; reference; authenticate target presets and worker state | Biome-source resolution and per-worker climate state. |
| 2618 | `crates/horizon-world/src/worldgen/biome_temperature.rs` | adaptable; reference; verify target temperature noise and float cache | Seeded biome temperature, altitude cooling and bounded cache. |
| 2619 | `crates/horizon-world/src/worldgen/biome_weather.rs` | adaptable; reference; bind target fluids, snow, height and Forge hooks | Registry-bound water-freezing and snow-placement checks. |
| 2620 | `crates/horizon-world/src/worldgen/blending.rs` | adaptable; reference; rebuild blending against 1.20.1 saved chunks | Legacy chunk-border blending of height, density, biome, carving and structure starts. |
| 2621 | `crates/horizon-world/src/worldgen/block_blob.rs` | adaptable; reference; verify blob RNG and update flags | Descending support search and randomized three-stage block blob. |
| 2622 | `crates/horizon-world/src/worldgen/block_column.rs` | adaptable; reference; verify target layers, truncation and provider RNG | Multi-layer directional column and tip-priority truncation. |
| 2623 | `crates/horizon-world/src/worldgen/block_feature.rs` | rewrite required; reference; bind target block/tag/property/biome/fluid registries | Registry-bound feature state/tag/property binding plus fill and replacement features. |
| 2624 | `crates/horizon-world/src/worldgen/block_pile.rs` | adaptable; reference; verify dirt_path, support and provider RNG | Randomized two-layer block pile with support/path validation. |
| 2625 | `crates/horizon-world/src/worldgen/block_predicate.rs` | adaptable; reference; rebuild target predicate set, bounds, order and Forge hooks | Composable recursive world-position predicate AST and evaluator. |
| 2626 | `crates/horizon-world/src/worldgen/blue_ice.rs` | adaptable; verify target BlueIceFeature configuration, RNG and flags | Blue-ice feature validates an underwater origin and performs 200 randomized replacement attempts. |
| 2627 | `crates/horizon-world/src/worldgen/bonus_chest.rs` | adaptable; verify target heightmap, chest loot and torch order | Bonus-chest feature permutes candidate positions, checks terrain, places chest and cardinal torches. |
| 2628 | `crates/horizon-world/src/worldgen/bulk_section.rs` | adaptable; require target packed-coordinate and exclusive-write parity | Provides bounded direct section access for high-volume worldgen mutation. |
| 2629 | `crates/horizon-world/src/worldgen/carver.rs` | rewrite required; rebuild 1.20.1 CaveWorldCarver and RNG | Decodes and runs cave-carver configuration including rooms, tunnels and branches. |
| 2630 | `crates/horizon-world/src/worldgen/carver/canyon.rs` | rewrite required; rebuild 1.20.1 CanyonWorldCarver | Canyon carver config, width profile and segmented path generation. |
| 2631 | `crates/horizon-world/src/worldgen/carver/generation.rs` | rewrite required; target carver registry, biome sample and seed order | Schedules configured carvers over neighboring origin chunks and returns a local carve mask. |
| 2632 | `crates/horizon-world/src/worldgen/carving.rs` | adaptable; verify 1.20.1 mask, surface repair and fluid ticks | Applies carve masks and handles replacement fluids, surface soil and postprocessing. |
| 2633 | `crates/horizon-world/src/worldgen/cave.rs` | adaptable; rebuild and validate each feature separately for 1.20.1 | Implements lake, monster-room and geode configured features. |
| 2634 | `crates/horizon-world/src/worldgen/chorus_plant.rs` | adaptable; verify target chorus state table and branch RNG | Generates chorus branches and terminal flowers with derived connection states. |
| 2635 | `crates/horizon-world/src/worldgen/climate.rs` | adaptable; validate target climate scale and coordinates | Quantizes climate dimensions and samples density climate values in quart space or bulk volumes. |
| 2636 | `crates/horizon-world/src/worldgen/climate/codec.rs` | rewrite required; target 1.20.1 Climate.Parameter codec | Decodes bounded climate intervals and offset values. |
| 2637 | `crates/horizon-world/src/worldgen/climate/index.rs` | rewrite required; do not adopt without vanilla tie parity | Constructs and searches custom multidimensional climate parameter index. |
| 2638 | `crates/horizon-world/src/worldgen/decoration.rs` | rewrite required; rebuild target WorldGenRegion decoration and seeding | Coordinates structure and biome decoration feature steps across neighboring chunks. |
| 2639 | `crates/horizon-world/src/worldgen/decoration_order.rs` | rewrite required; derive dependencies from 1.20.1 ChunkStatus pipeline | Emulates chunk decoration ticket scheduler and neighboring readiness order. |
| 2640 | `crates/horizon-world/src/worldgen/delta.rs` | adaptable; verify 1.20.1 delta codec, block checks and RNG | Places Nether delta rim/content with randomized diamond-like sampling. |
| 2641 | `crates/horizon-world/src/worldgen/density.rs` | rewrite required; rebuild 1.20.1 DensityFunctions and NoiseRouter | Builds named density graph and decodes arithmetic, spatial, noise, interpolation and spline operators. |
| 2642 | `crates/horizon-world/src/worldgen/disk.rs` | adaptable; verify target DiskFeature codec and write behavior | Disk feature writes provider-generated circular layers and marks postprocessing above exposed runs. |
| 2643 | `crates/horizon-world/src/worldgen/dripstone.rs` | rewrite required; rebuild 1.20.1 DripstoneClusterFeature/LargeDripstoneFeature | Implements speleothem-cluster and large-dripstone features. |
| 2644 | `crates/horizon-world/src/worldgen/end_gateway.rs` | adaptable; verify target EndGatewayFeature and BE lifecycle | Places fixed End gateway portal structure and optionally configures exit position. |
| 2645 | `crates/horizon-world/src/worldgen/end_podium.rs` | adaptable; verify 1.20.1 EndPodiumFeature and replacement semantics | Builds End exit podium, bedrock base, portal and cardinal torches. |
| 2646 | `crates/horizon-world/src/worldgen/end_spike.rs` | adaptable; verify 1.20.1 SpikeFeature layout and crystal spawn order | Generates ten End obsidian pillars, cages and crystals. |
| 2647 | `crates/horizon-world/src/worldgen/feature.rs` | adaptable; verify coordinate origin, End island RNG and flags | Basic void-start platform, End island and End platform features plus shared FeatureWorld interface. |
| 2648 | `crates/horizon-world/src/worldgen/feature_catalog.rs` | adaptable; rebuild target feature holder/tag and reload lifecycle | Builds immutable feature resource snapshot and expands named references/tags. |
| 2649 | `crates/horizon-world/src/worldgen/feature_graph.rs` | rewrite required; build target 1.20.1/Forge serializer catalogue and graph | Typed configured-feature graph, bounded decoder and recursive execution engine. |
| 2650 | `crates/horizon-world/src/worldgen/feature_region.rs` | adaptable; rebuild against 1.20.1 WorldGenRegion/ProtoChunk and Forge side effects | Resident multi-chunk terrain region with feature query and mutation adapter. |
| 2651 | `crates/horizon-world/src/worldgen/feature_sort.rs` | adaptable; verify vanilla FeatureSorter graph and tie ordering | Duplicate feature appearances and step order affect edge creation; cycles or invalid graph bounds must fail explicitly. Stable tie-breaking is essential for worldgen parity. |
| 2652 | `crates/horizon-world/src/worldgen/fluid_predicate.rs` | adaptable; verify target FluidTag/HolderSet codec and Forge registry lifecycle | Unknown names/tags and registry mismatches are explicit errors; the set is valid only for the exact registry snapshot used during binding. |
| 2653 | `crates/horizon-world/src/worldgen/fungus.rs` | adaptable; verify target HugeFungusFeature config, cap geometry, tags and RNG | Shape dimensions, block tests, branch order and RNG consumption determine the result. State-property and tag assumptions are registry-specific; partial placement remains after an error. |
| 2654 | `crates/horizon-world/src/worldgen/generator.rs` | rewrite required; architecture reference only; rebuild the full pipeline for 1.20.1/Forge 47 | This is a central version-specific engine, not an isolated utility. Stage sequencing, seeded randomness, worker ownership, chunk status transitions and side effects are coupled; the pinned implementation is from a later Java Edition and contains schemas/paths unavailable in 1.20.1. |
| 2655 | `crates/horizon-world/src/worldgen/generator_blocks.rs` | adaptable; rebuild state bindings against 1.20.1 registries | Cached handles are tied to one registry snapshot; hard-coded classifications and modern registry identities must not leak into a reload or target-version registry. |
| 2656 | `crates/horizon-world/src/worldgen/generator_decoration.rs` | rewrite required; rebuild target biome decoration/feature dispatch | Feature and placement schemas are snapshot-specific; feature ordering and RNG reseeding are compatibility-critical. Exact callback and chunk-write semantics need target Java/Forge comparison. |
| 2657 | `crates/horizon-world/src/worldgen/generator_structures.rs` | rewrite required; rebuild 1.20.1 Structure/StructureSet and chunk persistence integration | Structure placement salts, spacing/separation, biome eligibility, start reuse, reference limits and piece write ordering are target-version-sensitive; persistent start/reference data must match chunk serialization. |
| 2658 | `crates/horizon-world/src/worldgen/height_provider.rs` | adaptable; verify target HeightProvider codec and random bounds | Random draw sequence, inclusive/exclusive endpoints, vertical-anchor bounds and overflow behavior affect placed features and carvers. |
| 2659 | `crates/horizon-world/src/worldgen/iceberg.rs` | adaptable; verify target IcebergFeature dimensions, cleanup and RNG | Random draw order, noise samples, negative-coordinate handling and cleanup traversal are observable. Modern state/tag assumptions and geometry constants require target parity. |
| 2660 | `crates/horizon-world/src/worldgen/live.rs` | rewrite required; version-specific; rebuild 1.20.1 datapack overlay and registry reload | The implementation explicitly describes 26.3 folder naming and currently leaves added biomes out; tags have additive/replacement semantics distinct from ordinary entries. This is not a complete datapack reload/registry implementation. |
| 2661 | `crates/horizon-world/src/worldgen/long_set.rs` | adaptable; retain only after overflow/collision and benchmark verification | Hash mixing, sentinel values, resize thresholds and duplicate insertion results can affect memory and collision performance; ordering must not be assumed. |
| 2662 | `crates/horizon-world/src/worldgen/material_condition.rs` | rewrite required; rebuild 1.20.1 SurfaceRules condition types and evaluation context | Condition evaluation order, short-circuit behavior, cache key identity and world-column context are observable. The material-rule schema is later than 1.20.1 in places. |
| 2663 | `crates/horizon-world/src/worldgen/material_condition/codec.rs` | rewrite required; rebuild target SurfaceRules condition codecs | Accepted type names and field shapes are snapshot-specific; decode depth/node limits and alternative precedence must match target codecs. |
| 2664 | `crates/horizon-world/src/worldgen/material_condition/density.rs` | adaptable; verify target density sampling and cache parity | The sampling coordinate transform and cache lifetime affect material decisions; target DensityFunction nodes and surface Y semantics differ. |
| 2665 | `crates/horizon-world/src/worldgen/material_condition/library.rs` | adaptable; rebuild catalogue and references from target SurfaceRules | Reference resolution and compile ordering are important; partial resource visibility and cache sharing can differ from vanilla/Forge registry bootstrapping. |
| 2666 | `crates/horizon-world/src/worldgen/material_condition/noise.rs` | adaptable; verify target noise registry, positional RNG and scale | Noise identity, seed derivation, coordinate scaling and float conversions affect surface selection; modern noise definitions may differ. |
| 2667 | `crates/horizon-world/src/worldgen/material_condition/rule.rs` | rewrite required; reconstruct 1.20.1 SurfaceRules.RuleSource and RuleSource evaluation | Short-circuit and compile timing are observable; conditions may be compiled even for branches not ultimately selected. Some modern rule variants and constants are absent in 1.20.1. |
| 2668 | `crates/horizon-world/src/worldgen/material_extensions.rs` | rewrite required; only adopt if target-version behavior includes the corresponding extension | These are later-version extensions, not automatically part of 1.20.1 surface rules; added effects and block placement order alter terrain if copied. |
| 2669 | `crates/horizon-world/src/worldgen/material_stage.rs` | rewrite required; rebuild the 1.20.1 surface replacement stage | Column/vertical traversal direction, context update, bottom bound and side-effect ordering matter. A modern hard-coded lower scan bound is not evidence for 1.20.1 dimension limits. |
| 2670 | `crates/horizon-world/src/worldgen/material_surface.rs` | adaptable; verify target ClayBands generation and indexing | Band initialization order, random draws, thickness boundaries and indexing behavior are deterministic terrain inputs. |
| 2671 | `crates/horizon-world/src/worldgen/ore_feature.rs` | adaptable; verify target OreFeature/ScatteredOreFeature geometry and replacement behavior | Random stream, ellipsoid overlap, candidate iteration, state-provider sampling and direct section mutation order determine ore distribution; bypassed side effects need explicit handling. |
| 2672 | `crates/horizon-world/src/worldgen/overrides.rs` | rewrite required; version-specific; rebuild 1.20.1 datapack overlay and registry reload | The folder-kind catalogue explicitly uses Java 26.3 names and leaves new biomes out; dimension validation and tag replace semantics must be target-version exact. Embedded tests were inspected, not run. |
| 2673 | `crates/horizon-world/src/worldgen/placement.rs` | rewrite required; rebuild the 1.20.1 PlacementModifier catalogue and execution | Modifier order, random draw consumption, emission order, inclusive bounds, short-circuiting and work budgets are observable. The catalogue includes newer/custom modifiers and bounds not automatically equivalent to 1.20.1. |
| 2674 | `crates/horizon-world/src/worldgen/placement/chain.rs` | adaptable; verify Java 1.20.1 PlacedFeature modifier chain order and failure cleanup | Buffer contents persist after errors, so reusing a placer after failure can retain partial state. Nested feature callbacks intentionally do not inherit the root top-feature context. |
| 2675 | `crates/horizon-world/src/worldgen/placement/context.rs` | adaptable; rebuild adapters to match 1.20.1 ChunkAccess/WorldGenRegion and Forge side effects | The trait carefully separates raw section writes from ChunkAccess/WorldGenRegion effects; many callbacks default to unavailable or no-op, so host adapters must implement required operations explicitly. |
| 2676 | `crates/horizon-world/tests/movement_properties.rs` | adaptable; reference; test-only | Movement-property registry decoding and tag-binding regression tests. |
| 2677 | `crates/horizon-world/tests/native_load_hooks.rs` | adaptable; reference; test-only | Native loot/item hook installation and callback dispatch tests. |
| 2678 | `crates/horizon-world/tests/placement_bounds.rs` | adaptable; reference; test-only | Placement-modifier negative-extent and world-query work-budget tests. |
| 2679 | `crates/horizon-world/tests/placement_queries.rs` | adaptable; reference; test-only | Surface-water-depth placement query ordering and failure short-circuit test. |
| 2680 | `crates/horizon-world/tests/retrogen_biomes.rs` | adaptable; reference; test-only | Retrogen biome selection precedence and section-atomic fill tests. |
| 2681 | `crates/horizon-world/tests/rule_test_bounds.rs` | adaptable; reference; test-only | Rule-test recursive decode budgets, tag binding and matter-phase semantics tests. |
| 2682 | `crates/horizon-world/tests/state_provider_bounds.rs` | adaptable; reference; test-only | Block-state provider recursion, aggregate budgets, tags and catalogue resolution tests. |
| 2683 | `crates/horizon-world/tests/tick_data.rs` | adaptable; reference; test-only | Saved block/fluid tick NBT codec, coordinate filtering and atomic chunk update tests. |
| 2684 | `crates/horizon-world/tests/weighted_provider.rs` | adaptable; reference; test-only | Weighted-list validation, aggregate decoder budgets and optional zero-weight sampling tests. |
| 2685 | `crates/horizon-world/tests/world_io.rs` | adaptable; reference; test-only | Region-file sector allocation, external chunk records, chunk NBT edits and compression regression tests. |
| 2686 | `crates/horizon-world/tests/world_movement.rs` | adaptable; reference; test-only | Movement collision-scan candidate budget and full-sweep behavior tests. |
| 2687 | `crates/horizon-world/tests/world_ray_limits.rs` | adaptable; reference; test-only | Ray traversal visit-limit, non-finite input and world-provider error tests. |
| 2688 | `crates/starling-account/Cargo.toml` | adaptable; reference; dependency baseline only | Cargo package and dependency definition for Starling's account, file-download and Android Java-runtime subsystem. |
| 2689 | `crates/starling-account/src/ely.rs` | adaptable; optional; third-party provider | Ely.by Yggdrasil authentication, refresh, profile validation and skin/cape lookup. |
| 2690 | `crates/starling-account/src/files.rs` | adaptable; reference; harden before reuse | Pinned Minecraft manifest, client/server/library/asset downloader with hash verification, retry and progress reporting. |
| 2691 | `crates/starling-account/src/files/tests.rs` | adaptable; reference; test-only | Local mock-server tests for manifest pinning, verified download layout, retries and resumability. |
| 2692 | `crates/starling-account/src/java.rs` | adaptable; reference; harden before reuse | Downloads and installs pinned Android OpenJDK 25 runtimes for ARM64, ARMv7 and x86_64. |
| 2693 | `crates/starling-account/src/lib.rs` | adaptable; reference; harden and separate providers | Microsoft OAuth device-code sign-in, Xbox Live/XSTS exchange, Minecraft ownership/profile checks, session refresh and secret-store abstraction. |
| 2694 | `crates/starling-account/src/profiles.rs` | adaptable; reference; harden before reuse | Provider-aware Microsoft/Ely/offline profile selection, activation, addition and removal. |
| 2695 | `crates/starling-account/src/skins.rs` | adaptable; optional; harden before reuse | Microsoft skin upload, profile refresh and Ely.by skin-site guidance. |
| 2696 | `crates/starling-account/src/tests.rs` | adaptable; reference; test-only | Local mock tests for the Microsoft device-code/Xbox/XSTS/Minecraft login chain, refresh, cancellation and secret redaction. |
| 2697 | `crates/starling-catalog/Cargo.toml` | adaptable; optional; dependency baseline only | Cargo package and dependency definition for Starling's Android-facing mod/resource/shader catalog. |
| 2698 | `crates/starling-catalog/src/launcher/browser.rs` | unnecessary; placeholder/build shim only | Build-time path shim documenting the desktop browser module's alternate source location. |
| 2699 | `crates/starling-catalog/src/lib.rs` | rewrite required; optional; ecosystem-specific adapter | Android adapter for Horizon's mod/resource/shader catalog and installation flow. |
| 2700 | `crates/starling-catalog/src/list.rs` | adaptable; reference; UI utility | UI-independent catalog pagination feed, stale-request rejection and touch scrolling helpers. |
| 2701 | `crates/horizon-world/src/worldgen/placement/layer.rs` | adaptable; reference; version-specific | Layered configured-feature placement modifier that selects vertical candidates using a layer layout and local terrain checks. |
| 2702 | `crates/horizon-world/src/worldgen/projected.rs` | rewrite required; reference; version-specific | Projected configured features, including patchy-square projection and stepped-column clustering. |
| 2703 | `crates/horizon-world/src/worldgen/replace_blobs.rs` | adaptable; reference; version-specific | Configured feature that replaces a connected blob of a selected block state around an origin. |
| 2704 | `crates/horizon-world/src/worldgen/root_system.rs` | rewrite required; reference; version-specific | Root-system configured feature that places tree-root blocks, rooted dirt and hanging roots around a tree origin. |
| 2705 | `crates/horizon-world/src/worldgen/router.rs` | rewrite required; reference; version-specific | Defines and compiles the modern noise-router channels used by the later terrain-generation pipeline. |
| 2706 | `crates/horizon-world/src/worldgen/rule_test.rs` | adaptable; reference; version-specific | Implements recursive block rule tests and composite predicates used by the later world-generation feature system. |
| 2707 | `crates/horizon-world/src/worldgen/sculk.rs` | rewrite required; reference; version-specific | Implements sculk-related world-generation behavior, including cursor growth, spread, decay and placement around sculk catalysts. |
| 2708 | `crates/horizon-world/src/worldgen/settings.rs` | rewrite required; reference; version-specific | Defines and validates the noise-generation vertical settings, including minimum Y and generation height. |
| 2709 | `crates/horizon-world/src/worldgen/simple_block.rs` | adaptable; reference; version-specific | Implements the simple-block configured feature: sample a block-state provider, check survival, place the block and optionally schedule a tick. |
| 2710 | `crates/horizon-world/src/worldgen/snow_freeze.rs` | rewrite required; reference; version-specific | Performs the freeze-water and place-snow pass across a 16×16 feature footprint. |
| 2711 | `crates/horizon-world/src/worldgen/spawn_finder.rs` | rewrite required; reference; version-specific | Finds a candidate world-spawn column by minimizing the climate fitness of configured spawn-target points. |
| 2712 | `crates/horizon-world/src/worldgen/speleothem.rs` | rewrite required; reference; version-specific | Generates pointed dripstone-like formations, surrounding base patches and waterlogged pointed states. |
| 2713 | `crates/horizon-world/src/worldgen/spike.rs` | rewrite required; reference; version-specific | Builds randomized tapered spike structures with mirrored lower sections and discontinuous root columns. |
| 2714 | `crates/horizon-world/src/worldgen/spring.rs` | rewrite required; reference; version-specific | Places a configured fluid spring after checking its supporting blocks and exact neighbor rock/hole counts. |
| 2715 | `crates/horizon-world/src/worldgen/state_provider.rs` | rewrite required; reference; version-specific | Implements the data-driven block-state provider family, registry bindings, state selection and state-property transformations. |
| 2716 | `crates/horizon-world/src/worldgen/state_provider/catalog.rs` | adaptable; reference; version-specific | Stores named block-state provider resources and resolves provider holders lazily against a shared registry binding. |
| 2717 | `crates/horizon-world/src/worldgen/state_provider/noise.rs` | rewrite required; reference; version-specific | Implements noise, noise-threshold and dual-noise block-state provider variants. |
| 2718 | `crates/horizon-world/src/worldgen/state_provider/properties.rs` | adaptable; reference; version-specific | Builds compact lookup tables for block-state property identities, copying, reading and replacing Cartesian property values. |
| 2719 | `crates/horizon-world/src/worldgen/structure.rs` | rewrite required; reference; version-specific | Shared structure-template resources, block-state transforms, bounding boxes and processor/template lookup infrastructure. |
| 2720 | `crates/horizon-world/src/worldgen/structure/end_city.rs` | rewrite required; reference; version-specific | Procedurally assembles End City towers, bridges, houses and ship pieces, then handles structure markers during placement. |
| 2721 | `crates/horizon-world/src/worldgen/structure/enhanced_terrain.rs` | undetermined; reference; optional/future | Adapts external structure-library terrain kernels, including YUNG's API and MoogsStructureLib-style piece/junction rules. |
| 2722 | `crates/horizon-world/src/worldgen/structure/features.rs` | rewrite required; reference; version-specific | Implements template and fossil configured features, including weighted template rotation and fossil/overlay placement. |
| 2723 | `crates/horizon-world/src/worldgen/structure/formations.rs` | undetermined; reference; optional/future | Implements the external Formations mod's simple and pieced structures using pool elements, height placement and jigsaw children. |
| 2724 | `crates/horizon-world/src/worldgen/structure/fortress.rs` | rewrite required; reference; version-specific | Generates and places Nether Fortress bridge/castle pieces, corridors, rooms, spawners, chests and terrain supports. |
| 2725 | `crates/horizon-world/src/worldgen/structure/jigsaw.rs` | rewrite required; reference; version-specific | Implements structure pool elements, jigsaw blocks, pool decoding, vanilla/YUNG/Lithostitched candidate rules, structure assembly and piece placement. |
