# RustChunk Starling Audit Ledger — Entries 1801–2000

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1801–2000 (200 unique source files; one shared Ledger for this stage).
- Current progress: entries 1801–1825 complete (25/200); reporting section 1801–1850 authorized; checkpoint 1826–1850 remains authorized.
- Reporting rule: STOP at entry 1850 after both authorized 25-file checkpoints are complete. Wait for fresh explicit authorization before entry 1851.
- Each entry contains the 15 required fields defined in AUDIT_PROTOCOL.md. Purpose is separate from dependencies, implementation, findings, relevance, decision, implications, follow-up, mapping and evidence.
- Long files are read in consecutive, gap-free ranges of at most 400 logical lines through EOF. Source SHA and byte size are checked against the pinned recursive tree.
- Embedded tests/examples are inspected only; they are not executed.
- The active 200-file progress register is AUDIT_200_FILE_BOUNDARY.md; update and read it back at every checkpoint.

### 1801. `crates/horizon-dualsense/src/haptics.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6aa9d5ab823eb308a7477d2c215e571c47e32d2a`
- Size: 5850 bytes; 149 logical lines
- Coverage: 1–149 (EOF)
- Purpose: USB DualSense HD haptics waveform generator and CPAL output stream.
- Dependencies / cross-references: CPAL audio endpoint enumeration; four-channel 48 kHz stream; queued left/right actuator samples.
- Implementation / logic: Finds a four-channel F32 endpoint, leaves channels 1–2 silent and synthesizes ramped heavy/light tones on channels 3–4.
- Findings: Controller-specific USB audio path; the source's device-name and waveform tests were inspected only.
- RustChunk relevance: No core Minecraft, chunk or Android touchscreen value; optional hardware haptics only.
- Decision: unnecessary; optional; platform-specific
- 1.20.1 implications: Android CPAL backend and a four-channel USB endpoint are not established; do not infer compatibility.
- Follow-up checks: Check Android CPAL target, USB permission, endpoint channel layout, stream lifecycle, queue growth and real-device stop behavior.
- RustChunk implementation mapping: Isolated optional RustChunk controller-haptics backend; never a dependency of world, protocol or renderer crates.
- Evidence required before adoption: Android build, controller endpoint capability, channel mapping and hardware playback/stop tests.
- Completion state: COMPLETE — 149 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 5850-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1802. `crates/horizon-dualsense/src/haptics_stub.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dc4e616672858ee9bddbba312fe7e94d57741258`
- Size: 733 bytes; 20 logical lines
- Coverage: 1–20 (EOF)
- Purpose: No-op DualSense HD haptics implementation for iOS.
- Dependencies / cross-references: Shared Haptics interface; iOS cfg; CPAL/hidapi excluded on that target.
- Implementation / logic: Exports open/play/stop signatures but returns None and otherwise does nothing.
- Findings: The stub explicitly describes no four-channel phone audio device; it is not Android support.
- RustChunk relevance: Only a platform-stub pattern; no direct gameplay value.
- Decision: unnecessary; reference; platform-specific
- 1.20.1 implications: Android needs its own tested backend or explicit unsupported path; iOS cfg is not Android evidence.
- Follow-up checks: Check target cfgs and dependency feature gates to ensure unsupported platforms compile cleanly.
- RustChunk implementation mapping: Optional capability abstraction if controller output is implemented; otherwise exclude.
- Evidence required before adoption: Android compile matrix and device capability proof for a supported backend or explicit unsupported behavior.
- Completion state: COMPLETE — 20 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 733-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1803. `crates/horizon-dualsense/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4ae55de9a3ab1098d689c7214a0b30e66591d5a9`
- Size: 14189 bytes; 370 logical lines
- Coverage: 1–370 (EOF)
- Purpose: DualSense USB/Bluetooth output-report builder for rumble, adaptive triggers, lightbar and player LEDs.
- Dependencies / cross-references: HID device writer; CPAL haptics; trigger encoder; Sony VID/PIDs; Bluetooth CRC32 and firmware conventions.
- Implementation / logic: Builds a shared 47-byte block and wraps it as USB 0x02 or Bluetooth 0x31 with sequence nibble and CRC; selects legacy/V2 vibration.
- Findings: Contains byte-layout and CRC tests; tests and physical hardware behavior were not executed; iOS omits HID/haptics.
- RustChunk relevance: Optional low-level controller-output reference only; unrelated to MC protocol, rendering and chunk streaming.
- Decision: unnecessary; optional; platform-specific
- 1.20.1 implications: Do not import DualSense reports into generic Android input; Android transport and permission model remain unverified.
- Follow-up checks: Cross-check report layouts, firmware variants, Bluetooth sequence/CRC, disconnect cleanup and Android transport.
- RustChunk implementation mapping: Separate optional DualSense output module with platform adapters; keep generic input independent.
- Evidence required before adoption: Authoritative report references, Android HID implementation, firmware matrix and USB/BT hardware tests.
- Completion state: COMPLETE — 370 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 14189-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1804. `crates/horizon-dualsense/src/trigger.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `973c3e30df6e08c3470254edeec920b1523171b5`
- Size: 4806 bytes; 148 logical lines
- Coverage: 1–148 (EOF)
- Purpose: Encoder for DualSense adaptive-trigger resistance, weapon and vibration effects.
- Dependencies / cross-references: Parent TriggerEffect/TRIGGER_OFF definitions and external DualSense protocol references.
- Implementation / logic: Encodes ten zones with active masks and three-bit strength values; clamps parameters and writes firmware mode bytes.
- Findings: Controller-specific modes and byte test vectors are present; tests were not run on hardware.
- RustChunk relevance: Not needed for Android baseline or chunk/rendering work; optional device-specific feature.
- Decision: unnecessary; optional; platform-specific
- 1.20.1 implications: Keep out of common game controls unless Android exposes tested adaptive-trigger output.
- Follow-up checks: Verify modes and parameter bounds on DualSense/Edge firmware; test release and reconnect.
- RustChunk implementation mapping: Optional DualSense adapter, capability-gated if ever implemented.
- Evidence required before adoption: Current protocol references and Android HID tests on DualSense/Edge hardware.
- Completion state: COMPLETE — 148 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 4806-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1805. `crates/horizon-friends/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d53b9435f2085c3e47ce9f4c18a9de1fbe990bc2`
- Size: 774 bytes; 23 logical lines
- Coverage: 1–23 (EOF)
- Purpose: Manifest for invitation-based peer-hosting and encrypted friend tunnel.
- Dependencies / cross-references: SPAKE2, ChaCha20-Poly1305, HKDF, SHA-256, OS randomness, socket2, if-addrs and serde/JSON.
- Implementation / logic: Defines a Rust 2024 crate for cryptographic handshake, direct sockets, interface discovery and relay messages.
- Findings: This subsystem exposes an integrated server to invited friends; it is not the ordinary client protocol stack.
- RustChunk relevance: Could inform a separate future friend-hosting feature; does not enable joining normal Aternos servers.
- Decision: unnecessary; optional/future; security-sensitive
- 1.20.1 implications: Keep outside the minimum Android dependency graph; audit Android RNG/socket support and dependency versions first.
- Follow-up checks: Review dependency Android support/features, crypto maintenance, network permissions and threat model.
- RustChunk implementation mapping: Isolated optional peer-hosting package, independent of Forge login and public-server client transport.
- Evidence required before adoption: Android dependency build, independent security review, threat model and interoperability vectors.
- Completion state: COMPLETE — 23 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 774-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1806. `crates/horizon-friends/src/code.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ab06ff826e0e86f91061cb0e996e3617536f466`
- Size: 5134 bytes; 159 logical lines
- Coverage: 1–159 (EOF)
- Purpose: Parser and generator for short Crockford-base32 friend invitation codes.
- Dependencies / cross-references: Shared alphabet/HALF constants and OS randomness via crate::fill_random.
- Implementation / logic: Splits eight symbols into a 20-bit nameplate and 20-bit secret; normalizes ambiguous characters and redacts secret in Debug.
- Findings: Short secret entropy is 20 bits; PAKE can protect against offline guessing but online rate limits and rotation are essential.
- RustChunk relevance: Optional invite UX only; unrelated to standard server addresses and Mojang/Microsoft credentials.
- Decision: adaptable; optional/future; security-sensitive
- 1.20.1 implications: Never reuse as a login token or normal server address; combine with PAKE and online attempt limits.
- Follow-up checks: Check RNG distribution, guessing budget, reconnect rate limits, rotation and log/clipboard redaction.
- RustChunk implementation mapping: Invite-code component inside a separate peer-hosting feature.
- Evidence required before adoption: Security-reviewed PAKE, entropy analysis, online abuse tests and secret-redaction tests.
- Completion state: COMPLETE — 159 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 5134-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1807. `crates/horizon-friends/src/crypto.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `837377205fd4a9c959cb639950872287b9d9b1a9`
- Size: 13327 bytes; 392 logical lines
- Coverage: 1–392 (EOF)
- Purpose: SPAKE2 key agreement and authenticated encrypted framing for friend-tunnel streams.
- Dependencies / cross-references: Ed25519Group SPAKE2; HKDF-SHA256; ChaCha20-Poly1305; OS randomness; Read/Write streams.
- Implementation / logic: Derives a session key, exchanges per-stream randoms, separates directional keys, authenticates confirmation and frames data/control with counter nonces and length AAD.
- Findings: Caps payload at 16 KiB and includes tamper/replay/wrong-code tests; tests not run. Reader counter increment and handshake ordering merit review.
- RustChunk relevance: Useful security architecture research for optional private tunnel; unnecessary for ordinary public-server join.
- Decision: adaptable; reference; security-sensitive; optional/future
- 1.20.1 implications: Do not adopt as production crypto without independent review and Android RNG/stream validation.
- Follow-up checks: Audit counter overflow symmetry, handshake ordering, identity binding, truncation/DoS, key erasure and nonce uniqueness.
- RustChunk implementation mapping: Optional secure-stream adapter outside Minecraft packet codec and normal login.
- Evidence required before adoption: Independent cryptographic audit, adversarial fuzzing, property tests and Android RNG/interoperability validation.
- Completion state: COMPLETE — 392 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 13327-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1808. `crates/horizon-friends/src/direct.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7c80822d54d21ea1dd94201cde92945cb4070faa`
- Size: 11753 bytes; 335 logical lines
- Coverage: 1–335 (EOF)
- Purpose: Direct TCP endpoint selection, simultaneous-open dialing and session-introduction helpers.
- Dependencies / cross-references: socket2, IPv4/IPv6 sockets, interface addresses, atomics, channels and invite endpoints.
- Implementation / logic: Binds dual-stack listeners on one port, attempts TCP simultaneous-open and races/retries peer endpoints while checking session magic.
- Findings: Loopback tests exist but were not run; real NAT traversal and Android socket behavior are not proven.
- RustChunk relevance: Optional peer-to-peer hosting; not needed to connect to an Aternos TCP address.
- Decision: unnecessary; optional/future; network-specific
- 1.20.1 implications: Android socket reuse, NAT, IPv6, foreground/background and cellular behavior require independent checks.
- Follow-up checks: Test CGNAT/symmetric NAT, Wi-Fi/cellular transitions, socket options, cancellation and thread/resource bounds.
- RustChunk implementation mapping: Isolated direct transport for a future friends feature; standard multiplayer path remains direct client TCP.
- Evidence required before adoption: Real-network NAT matrix, Android socket tests and bounded resource/cancellation testing.
- Completion state: COMPLETE — 335 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 11753-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1809. `crates/horizon-friends/src/guest.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `492f4d48296fb22cba1c99216e45f2d9c6954361`
- Size: 11421 bytes; 299 logical lines
- Coverage: 1–299 (EOF)
- Purpose: Guest workflow for joining a peer invite through direct TCP or relay and exposing a local tunnel socket.
- Dependencies / cross-references: Invite parser, direct sockets, relay resolver/wire, SPAKE2 crypto, session negotiation and loopback listener.
- Implementation / logic: Parses short/long invite, tries direct then relay, checks relay game protocol/version, secures stream and accepts one 127.0.0.1 client connection for a limited time.
- Findings: The tunnel carries a friend's integrated server; it does not implement generic Forge server login or mod negotiation.
- RustChunk relevance: Optional friends guest path only; not an Aternos client transport.
- Decision: unnecessary; optional/future; version-specific
- 1.20.1 implications: Keep distinct from ordinary multiplayer and do not map invite syntax to vanilla ServerAddress.
- Follow-up checks: Review loopback listener lifetime/peer check, cancellation, version negotiation, relay trust and error paths.
- RustChunk implementation mapping: Future friends-guest adapter above tunnel API; base client remains standard-server capable.
- Evidence required before adoption: Reviewed relay protocol, Android loopback tests, security review and integrated-server interoperability.
- Completion state: COMPLETE — 299 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 11421-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1810. `crates/horizon-friends/src/host.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cc1cfe417d557928f287145564b3db5657df0125`
- Size: 21247 bytes; 594 logical lines
- Coverage: 1–400, 401–594 (EOF)
- Purpose: Host lifecycle for sharing a local integrated Minecraft server with invited guests.
- Dependencies / cross-references: Direct listeners, Reach/portmap, relay registration/control, invites, crypto/session, local server TCP endpoint and threads.
- Implementation / logic: Publishes direct/relay invites, renews router mapping, accepts encrypted guests, caps sessions/handshakes at eight and pumps streams to integrated-server port.
- Findings: A 10-second handshake bound and wrong-code rotation threshold are present; no real router, NAT or game-server test was run.
- RustChunk relevance: Peer-hosting architecture only; does not improve outbound connection to Forge/Aternos servers.
- Decision: unnecessary; optional/future; network-specific
- 1.20.1 implications: Keep outside core Android client; lifecycle, background threads, user consent and port mapping need separate design.
- Follow-up checks: Review thread shutdown, timeout enforcement, accounting, lease release, concurrent stop/start and local-server errors.
- RustChunk implementation mapping: Optional integrated-server sharing service behind explicit user action.
- Evidence required before adoption: Android server lifecycle, security review, real router/NAT tests and two-device interoperability.
- Completion state: COMPLETE — 594 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 21247-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1811. `crates/horizon-friends/src/invite.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `be586a57a9f41ec2967f24dc9ec91687464dde20`
- Size: 13395 bytes; 403 logical lines
- Coverage: 1–400, 401–403 (EOF)
- Purpose: Versioned binary invite format encoded as Crockford base32 with endpoints, secret and optional relay reference.
- Dependencies / cross-references: Code alphabet/parser, SHA-256 checksum, IPv4/IPv6 SocketAddr, RelayRef and MAX_ENDPOINTS.
- Implementation / logic: Serializes version, 80-bit secret, port, typed endpoint records and optional relay short code; validates checksum/version on decode.
- Findings: Encoder truncates at eight endpoints but decoder does not visibly enforce the same cap; two-byte checksum detects typos but is not authentication.
- RustChunk relevance: Optional invite mechanism, not normal Minecraft server list/protocol.
- Decision: rewrite required; optional/future; security-sensitive
- 1.20.1 implications: If reused, enforce endpoint/string/input caps and do not confuse this schema with MC protocol/version fields.
- Follow-up checks: Add decoder endpoint cap, total length bounds, duplicate/unspecified/multicast policy, URL validation and fuzz tests.
- RustChunk implementation mapping: Separate versioned invite codec for future peer-hosting; no change to standard server address parser.
- Evidence required before adoption: Fuzzed parser, strict bounds, canonicalization and reviewed PAKE integration.
- Completion state: COMPLETE — 403 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 13395-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1812. `crates/horizon-friends/src/portmap.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6ad9a2b9477b8b37be1638388db4ca8baafb3b6e`
- Size: 35392 bytes; 980 logical lines
- Coverage: 1–400, 401–800, 801–980 (EOF)
- Purpose: Router port-mapping client for PCP, NAT-PMP and UPnP IGD with lease renewal/removal.
- Dependencies / cross-references: UDP/TCP sockets, SSDP multicast, HTTP/SOAP parser, OS gateway parsers, random PCP nonce and Mapping lifecycle.
- Implementation / logic: Discovers gateways, requests TCP mapping, renews at half lease and removes mappings; tries PCP, NAT-PMP then UPnP; includes mock-router tests.
- Findings: Can open inbound router ports. SSDP LOCATION is fetched without an evident host allow-list (potential local-network request surface); PCP nonce RNG failure is ignored. Tests unrun.
- RustChunk relevance: Not required to join Aternos; hosting-only with high security/privacy implications.
- Decision: unnecessary; optional/future; security-sensitive; platform-specific
- 1.20.1 implications: Never auto-enable on Android; require user consent, destination validation and reliable cleanup.
- Follow-up checks: Audit SSDP URL/redirect/DNS-rebinding, scope, XML parsing, HTTP bounds, RNG errors, mapping ownership and crash cleanup.
- RustChunk implementation mapping: Feature-gated host-only port mapping with explicit confirmation and strict target validation.
- Evidence required before adoption: Security review against malicious SSDP, router matrix, Android lifecycle/consent and lease cleanup tests.
- Completion state: COMPLETE — 980 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 35392-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1813. `crates/horizon-friends/src/reach.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b8176315cbd54f0b22095bf906015d29dcc7fcf6`
- Size: 13014 bytes; 361 logical lines
- Coverage: 1–361 (EOF)
- Purpose: Network reachability assessment and candidate endpoint collection for peer hosting.
- Dependencies / cross-references: if-addrs interface enumeration, portmap, STUN public IP, CGNAT ranges and invite endpoint cap.
- Implementation / logic: Ranks LAN/VPN/IPv6 interfaces, optionally opens router mapping, requests STUN public IPv4 and produces a reachability verdict.
- Findings: Skips selected virtual-interface prefixes and classifies 100.64/10 as shared; STUN disclosure and interface heuristics are environment-dependent.
- RustChunk relevance: Optional host diagnostic; irrelevant to outbound connection to public Aternos servers.
- Decision: adaptable; reference; optional/future; privacy-sensitive
- 1.20.1 implications: Keep discovery off unless hosting is requested; disclose external IP use and provide disable controls.
- Follow-up checks: Test Android Wi-Fi/cellular/VPN names, IPv6 ULA/global, multi-router, CGNAT and interface ranking.
- RustChunk implementation mapping: Optional hosting reachability diagnostic, isolated from normal multiplayer.
- Evidence required before adoption: Android interface coverage, controlled network scenarios, consent review and classification verification.
- Completion state: COMPLETE — 361 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 13014-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1814. `crates/horizon-friends/src/session.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `389aa85df38bafffe773a238b6c4af0bedf81dbc`
- Size: 6788 bytes; 195 logical lines
- Coverage: 1–195 (EOF)
- Purpose: Secure session path negotiation between direct TCP and relay tunnel.
- Dependencies / cross-references: Secure reader/writer, crypto key, direct candidate race, serde control messages, roles and PathMode.
- Implementation / logic: Exchanges candidates over protected relay; guest chooses authenticated direct stream or relay fallback; host follows the decision.
- Findings: Caps parsed candidates at 16 and uses fixed race windows; protocol is Horizon-specific rather than Minecraft negotiation.
- RustChunk relevance: Optional transport-selection pattern only; not a chunk-streaming fix.
- Decision: adaptable; reference; version-specific; optional/future
- 1.20.1 implications: Do not reuse control messages in vanilla/Forge connection; Android timeout/cancel/reconnect semantics need checks.
- Follow-up checks: Test peer disagreement, direct loss, relay/direct-only combinations, shutdown timing and malformed control.
- RustChunk implementation mapping: Optional transport selection beneath the separate friends tunnel.
- Evidence required before adoption: Interoperability traces, fault injection and Android network-switch tests.
- Completion state: COMPLETE — 195 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 6788-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1815. `crates/horizon-friends/src/stun.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e1716d268c53eb4b35ff391a6b270c1e3c234c23`
- Size: 7343 bytes; 207 logical lines
- Coverage: 1–207 (EOF)
- Purpose: STUN Binding request/response parser and public IPv4 discovery.
- Dependencies / cross-references: UDP socket, DNS, Cloudflare/Google STUN defaults, random transaction ID and XOR-MAPPED-ADDRESS parsing.
- Implementation / logic: Queries configured servers sequentially with retries, validates response source/transaction ID and returns first mapped address; env can disable it.
- Findings: Query resolves IPv4 only though parser supports IPv6; defaults disclose public IP to third parties. Tests use local mock and were not run.
- RustChunk relevance: Optional peer-host feature; not needed for an outbound client connection.
- Decision: unnecessary; optional/future; privacy-sensitive
- 1.20.1 implications: Do not query external STUN on ordinary client startup; hosting must disclose use and support opt-out.
- Follow-up checks: Check padding/bounds, server trust, IPv6, DNS policy, retries and privacy defaults.
- RustChunk implementation mapping: Optional host networking helper invoked only after user chooses hosting.
- Evidence required before adoption: RFC vectors, malicious-response fuzzing, Android UDP/DNS tests and privacy review.
- Completion state: COMPLETE — 207 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 7343-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1816. `crates/horizon-friends/src/wire.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5991bdf2f363145abca3da88da5e6294a082ed2a`
- Size: 6747 bytes; 206 logical lines
- Coverage: 1–206 (EOF)
- Purpose: Line-delimited JSON control protocol between friend clients and relay.
- Dependencies / cross-references: Serde, Read/Write/BufRead, MAX_LINE 4096 and relay protocol version 1.
- Implementation / logic: Frames host/join/attach/pong control records; bounded line read preserves raw bytes after bridge response.
- Findings: Length bound is useful; fixtures use MC protocol 777 and game 26.3. Tests inspected only.
- RustChunk relevance: Custom relay protocol only; not Minecraft status/login/Forge negotiation.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Do not confuse relay VERSION=1 with Minecraft protocol version; no direct wire compatibility with 1.20.1.
- Follow-up checks: Check newline/UTF-8/schema limits, unknown fields, relay authentication, bridge boundary and downgrade policy.
- RustChunk implementation mapping: Isolated optional relay control codec, separate from the Minecraft packet codec.
- Evidence required before adoption: Versioned spec, parser fuzzing, malicious-relay tests and cross-version interop.
- Completion state: COMPLETE — 206 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 6747-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1817. `crates/horizon-hdr/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `297c758b1334ce590d7c7b1b1fb4ab98723e518e`
- Size: 896 bytes; 23 logical lines
- Coverage: 1–23 (EOF)
- Purpose: Manifest for optional HDR swapchain metadata integration.
- Dependencies / cross-references: wgpu 30 Vulkan feature, ash 0.38 and Linux Wayland color-management crates.
- Implementation / logic: Declares a separate HDR crate with newer rendering stack and OS-specific native dependencies.
- Findings: No Android backend is declared; dependency versions need not match RustChunk renderer.
- RustChunk relevance: HDR is display quality, not chunk loading/culling or direct FPS improvement.
- Decision: unnecessary; optional; platform-specific; version-specific
- 1.20.1 implications: Do not add exact versions to Android baseline; Vulkan extension availability varies by GPU/driver.
- Follow-up checks: Check Android dependency resolution, Vulkan extension, surface lifecycle and display semantics.
- RustChunk implementation mapping: Optional renderer HDR adapter only if target GPU capability and benefit are demonstrated.
- Evidence required before adoption: Android target build, Vulkan capability enumeration and real-device display validation.
- Completion state: COMPLETE — 23 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 896-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1818. `crates/horizon-hdr/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4f7d737692eef53e552178d20119a7d4718f8f09`
- Size: 3924 bytes; 99 logical lines
- Coverage: 1–99 (EOF)
- Purpose: Vulkan HDR metadata bridge for wgpu swapchains.
- Dependencies / cross-references: wgpu Vulkan HAL, ash VK_EXT_hdr_metadata and adapter/device/surface handles.
- Implementation / logic: Checks extension availability, opens adapter with the extension and sets BT.709 primaries plus luminance metadata.
- Findings: Uses unsafe HAL handles with safety comments and no-ops without extension/swapchain; physical output not verified.
- RustChunk relevance: Display HDR only; no direct chunk geometry/load-scheduling effect.
- Decision: adaptable; optional/future; platform-specific; architecture-specific
- 1.20.1 implications: Mali-G52/Android VK_EXT_hdr_metadata support is unknown; do not assume wgpu 30 integration.
- Follow-up checks: Validate extension, handle lifetime, luminance input bounds, swapchain recreation and actual output.
- RustChunk implementation mapping: Capability-gated optional HDR renderer extension, disabled by default.
- Evidence required before adoption: Android GPU extension query, Vulkan validation, lifecycle tests and calibrated display measurement.
- Completion state: COMPLETE — 99 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 3924-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1819. `crates/horizon-hdr/src/wayland.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d5742ba2e94a5ed96f308eb69275ebaaebbfd658`
- Size: 8283 bytes; 231 logical lines
- Coverage: 1–231 (EOF)
- Purpose: Wayland color-management Watch for preferred display luminance data.
- Dependencies / cross-references: Wayland foreign handles, event queue, wl_surface, wp_color_management_v1 and winit raw handles.
- Implementation / logic: Binds compositor color manager and tracks preferred image-description reference/target luminances.
- Findings: Linux-only Wayland module; no Android windowing relevance.
- RustChunk relevance: No Android target value; desktop HDR detail only.
- Decision: unnecessary; platform-specific
- 1.20.1 implications: Exclude from Android; use platform-native display APIs if HDR is later supported.
- Follow-up checks: Only revisit for Linux desktop; verify object lifetimes, event dispatch and compositor versions.
- RustChunk implementation mapping: No Android mapping; keep Linux-only if desktop build requires it.
- Evidence required before adoption: Linux compositor integration tests, not needed for Android baseline.
- Completion state: COMPLETE — 231 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 8283-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1820. `crates/horizon-items/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4663c4d9505e17dd6956a1b9589c0cd43faba352`
- Size: 716 bytes; 22 logical lines
- Coverage: 1–22 (EOF)
- Purpose: Manifest for Horizon block/item facts and modern item-component crate.
- Dependencies / cross-references: horizon-client-core, horizon-core, horizon-world, serde and serde_json raw_value; Rust 1.90.
- Implementation / logic: Defines horizon-items package and test-support feature atop newer world/client abstractions.
- Findings: Package/API and Rust MSRV belong to pinned Horizon tree; not a drop-in dependency.
- RustChunk relevance: Items/blocks are essential gameplay domains, but only the architecture may be reused.
- Decision: rewrite required; version-specific
- 1.20.1 implications: RustChunk targets MC 1.20.1/Forge 47.x legacy item NBT and its supported Android toolchain.
- Follow-up checks: Audit target registry/item format, MSRV, serde features, dependency graph and Android cross-compile.
- RustChunk implementation mapping: Reimplement required block/item model inside RustChunk 1.20.1 core.
- Evidence required before adoption: 1.20.1 Forge/vanilla registries and NBT fixtures plus Android toolchain proof.
- Completion state: COMPLETE — 22 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 716-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1821. `crates/horizon-items/examples/enchantment_components_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2221ff609fc617fa1789468fad228696cda0c078`
- Size: 1470 bytes; 45 logical lines
- Coverage: 1–45 (EOF)
- Purpose: Offline probe converting modern item-component SNBT into enchantment summary fixtures.
- Dependencies / cross-references: Enchantments, item_from_nbt, SNBT parser, registry JSON and input/output files.
- Implementation / logic: Reads tab-delimited fixtures, wraps components in an item, decodes ItemStack and writes damage plus active/stored enchantments.
- Findings: Diagnostic executable only; modern components shape; inspected but not run.
- RustChunk relevance: Useful parity-tool pattern, not production game behavior.
- Decision: undetermined; reference; example only
- 1.20.1 implications: Rewrite fixture input for 1.20.1 ItemStack NBT; this components map is not target-era storage/wire data.
- Follow-up checks: Compare enchantment IDs, stored enchantments, damage and display names with 1.20.1 Forge.
- RustChunk implementation mapping: Optional RustChunk parity/fixture executable for items.
- Evidence required before adoption: 1.20.1 source fixtures, deterministic output and vanilla/Forge oracle comparison.
- Completion state: COMPLETE — 45 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 1470-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1822. `crates/horizon-items/src/banner_layer.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5f76ad575779866f9c4b259bca433406cf97d6a9`
- Size: 1605 bytes; 56 logical lines
- Coverage: 1–56 (EOF)
- Purpose: Conversion helpers for modern banner pattern component layers and NBT.
- Dependencies / cross-references: horizon_world::nbt::Tag, Arc strings and BannerPatternLayers.Layer schema.
- Implementation / logic: Serializes/deserializes list compounds with lowercase pattern/color fields and adds minecraft namespace.
- Findings: Modern component schema differs from 1.20.1 BlockEntityTag.Patterns and its legacy field capitalization.
- RustChunk relevance: Banner behavior is relevant, but this exact model is version-specific.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Build target NBT and rendering from 1.20.1; do not consume modern component form directly.
- Follow-up checks: Verify pattern IDs, color enum mapping, block-entity NBT and modded patterns.
- RustChunk implementation mapping: RustChunk banner ItemStack/BlockEntityTag adapter for 1.20.1.
- Evidence required before adoption: 1.20.1 banner item and block-entity round-trip fixtures plus Forge custom-pattern tests.
- Completion state: COMPLETE — 56 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 1605-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1823. `crates/horizon-items/src/blocks.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d7dcb5db6c640e599e0d422844cbd56f56fcac83`
- Size: 21467 bytes; 581 logical lines
- Coverage: 1–400, 401–581 (EOF)
- Purpose: Loader and runtime model for exported per-block-state gameplay facts.
- Dependencies / cross-references: BlockRegistry/BlockStateId, gameplay-facts and block-behaviours JSON, geometry interning, fluid logic, sounds and Behaviours projection.
- Implementation / logic: Validates count/order against registry, decodes collision/outline, lighting, hardness, sounds, fluids, class chains, sign wood and map-color facts; supports parallel validation and compact projection.
- Findings: Fixed MapColor table and exported schema are pinned-version data. State accessors index vectors directly; callers must pass valid IDs.
- RustChunk relevance: High-value architecture for large/modded registries, but not a compatible drop-in.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Regenerate all state facts from exact 1.20.1 vanilla plus Forge/mod registries: fluid levels, map colors, classes, shapes and sound sets.
- Follow-up checks: Check mod class inheritance, state order, waterlogged rules, defaults, bounds, lighting opacity and block-entity types.
- RustChunk implementation mapping: RustChunk block-state fact ingestion and compact runtime projection with an independent target data generator.
- Evidence required before adoption: Complete 1.20.1 export, all-state coverage and shape/fluid/light parity fixtures.
- Completion state: COMPLETE — 581 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 21467-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1824. `crates/horizon-items/src/blocks/decode.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7feb3ac41c445c3c347145003e158ad061bcadac`
- Size: 10111 bytes; 293 logical lines
- Coverage: 1–293 (EOF)
- Purpose: Typed low-allocation JSON decoder for gameplay facts and interned geometry shapes.
- Dependencies / cross-references: Serde visitors, serde_json Value/RawValue, ShapeKey hashing/equality, Arc and parent BlockFacts loader.
- Implementation / logic: Runs recursive validation separately, borrows raw shapes, projects typed fields and interns canonical values while preserving float-bit distinctions.
- Findings: Non-array shapes fall back; incomplete boxes are zero-filled and malformed box values filtered; valid JSON does not guarantee valid geometry.
- RustChunk relevance: Useful low-allocation ingestion design for large block registries; schema needs adaptation.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Validate 1.20.1 geometry strictly before use; preserve target registry semantics and profile memory/CPU on Android.
- Follow-up checks: Add malformed bounds, NaN/infinity policy, depth/size caps, duplicate-key semantics and fuzz cases.
- RustChunk implementation mapping: RustChunk block-facts decoder or generated registry loader with strict target schema.
- Evidence required before adoption: Property/fuzz tests, malformed-shape rejection, memory benchmarks and exact 1.20.1 facts parity.
- Completion state: COMPLETE — 293 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 10111-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

### 1825. `crates/horizon-items/src/blocks/decode_tests.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c222e26bd04e479fc200ddffcafbc50e1f146056`
- Size: 6279 bytes; 166 logical lines
- Coverage: 1–166 (EOF)
- Purpose: Regression tests for shape canonicalization and typed gameplay-facts decoder.
- Dependencies / cross-references: Parent decode module, serde_json, RawValue, Arc pointer identity, temp filesystem and BlockRegistry fixture.
- Implementation / logic: Compares shape values/interning with legacy Value semantics; tests defaults, duplicate keys, wrong types, overflow/depth errors and validation order.
- Findings: Strong test inventory for decoder compatibility and malformed JSON; all tests were inspected only, not executed.
- RustChunk relevance: Useful test design for RustChunk block-fact ingestion, not production code.
- Decision: undetermined; reference; tests only
- 1.20.1 implications: Port test intents rather than modern schema; add 1.20.1 shape/registry fixtures and Android-scale memory cases.
- Follow-up checks: Run on target Rust toolchain, extend malformed geometry/fuzz corpus and compare to 1.20.1 oracle.
- RustChunk implementation mapping: RustChunk block-facts unit/property/fuzz tests; no production mapping.
- Evidence required before adoption: Executed tests, deterministic 1.20.1 fixtures, fuzz corpus and verified decoder compatibility.
- Completion state: COMPLETE — 166 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 6279-byte size match the recursive source tree; embedded tests/examples were inspected only, not run.

