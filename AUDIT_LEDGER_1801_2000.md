# RustChunk Starling Audit Ledger — Entries 1801–2000

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1801–2000 (200 unique source files; one shared Ledger for this stage).
- Current progress: entries 1801–1875 complete (75/200); section 1851–1900 is active (25/50), checkpoint 1851–1875 closed; 1876–1900 remains authorized.
- Reporting rule: complete the authorized section 1851–1900, then STOP at 1900 and wait for fresh explicit authorization before entry 1901.
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


### 1812. `crates/horizon-friends/src/lib.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1d13b6dce79f84a9a0517192a2d7d471922b2510`
- Size: 8147 bytes; 230 logical lines
- Coverage: 1–230 (EOF)
- Purpose: Shared module façade and TCP stream pump for Horizon's invite-based friend-to-friend world tunnel.
- Dependencies / cross-references: code/crypto/direct/guest/host/invite/portmap/reach/session/stun/wire modules; getrandom; TCP streams, threads, mpsc channels and optional relay URL.
- Implementation / logic: Exports friend modules and the `friend:` address prefix; supplies OS randomness, absolute handshake deadline cancellation, direct/relay mode selection, relay DNS resolution and bidirectional forwarding between a plain game TCP stream and authenticated SecureReader/SecureWriter.
- Findings: Bridges an already-established game stream; does not implement Minecraft login or packet negotiation. Deadline tests address slow byte-by-byte peers but were inspected only. Relay URL parsing is a simple host:port/tcp:// parser and needs caller-side validation.
- RustChunk relevance: Optional friend-hosting/tunnel only; it does not help join ordinary public Forge/Aternos servers.
- Decision: unnecessary; optional/future; network-specific
- 1.20.1 implications: Transparent transport only for peers using Horizon's separate invite/relay protocol; not compatible with a normal public server address or vanilla/Forge negotiation.
- Follow-up checks: Review deadline-thread cancellation/leaks, Android lifecycle and network switches, relay URL validation, socket shutdown races, IPv6/NAT and resource limits.
- RustChunk implementation mapping: Keep as an isolated optional friends-tunnel adapter. Never route ordinary multiplayer through it or couple it to the Minecraft packet codec.
- Evidence required before adoption: Two-device Android interoperability, NAT/relay tests, network-switch/cancellation stress, secure-stream audit and verified 1.20.1 integrated-server behavior.
- Completion state: COMPLETE — 230 logical lines read through EOF; SHA and size match the complete pinned recursive tree; tests were inspected only, not run.
### 1813. `crates/horizon-friends/src/portmap.rs`
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

### 1814. `crates/horizon-friends/src/reach.rs`
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

### 1815. `crates/horizon-friends/src/session.rs`
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

### 1816. `crates/horizon-friends/src/stun.rs`
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

### 1817. `crates/horizon-friends/src/wire.rs`
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

### 1818. `crates/horizon-hdr/Cargo.toml`
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

### 1819. `crates/horizon-hdr/src/lib.rs`
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

### 1820. `crates/horizon-hdr/src/wayland.rs`
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

### 1821. `crates/horizon-items/Cargo.toml`
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

### 1822. `crates/horizon-items/examples/enchantment_components_probe.rs`
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

### 1823. `crates/horizon-items/src/banner_layer.rs`
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

### 1824. `crates/horizon-items/src/blocks.rs`
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

### 1825. `crates/horizon-items/src/blocks/decode.rs`
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

### 1826. `crates/horizon-items/src/blocks/decode_tests.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c222e26bd04e479fc200ddffcafbc50e1f146056`
- Size: 6279 bytes; 166 logical lines
- Coverage: 1–166 (EOF)
- Purpose: Regression tests for shape canonicalization and typed facts decoding.
- Dependencies / cross-references: serde_json/RawValue, Arc identity, temp files and BlockRegistry fixture.
- Implementation / logic: Compares shapes/interning with legacy Value semantics; checks defaults, duplicate keys, wrong types, overflow/depth and validation order.
- Findings: Good test inventory for malformed JSON and decoder compatibility; tests were not run.
- RustChunk relevance: Test-design reference for RustChunk block facts, not production implementation.
- Decision: undetermined; reference; tests only
- 1.20.1 implications: Use test intent only; add exact 1.20.1 shape and registry fixtures.
- Follow-up checks: Run on chosen toolchain, add malformed geometry/fuzz cases and compare against a 1.20.1 oracle.
- RustChunk implementation mapping: RustChunk block-facts unit/property/fuzz tests.
- Evidence required before adoption: Executed tests, deterministic 1.20.1 fixtures and fuzz corpus.
- Completion state: COMPLETE — 166 logical lines read through EOF; contiguous coverage; pinned SHA and 6279-byte size match the recursive tree; tests/examples inspected only, not run.

### 1827. `crates/horizon-items/src/bundle.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f32d23a5d2567bad25be6182631a5457756ede7b`
- Size: 16103 bytes; 441 logical lines
- Coverage: 1–400, 401–441 (EOF)
- Purpose: Bundle component weight, selection, insertion/removal and tooltip behavior.
- Dependencies / cross-references: ItemStack components, registry weights, nested bundle contents and tooltip code.
- Implementation / logic: Computes nested weight with exact rational arithmetic; provides admission, insert/remove and selection helpers.
- Findings: Uses newer bundle_contents component model; extreme counts, nesting depth and arithmetic bounds need review.
- RustChunk relevance: Optional inventory/gameplay architecture, unrelated to chunk loading.
- Decision: adaptable; optional/future; version-specific; arithmetic-review
- 1.20.1 implications: 1.20.1 has older bundle item/NBT semantics; not a direct drop-in.
- Follow-up checks: Check integer overflow, recursion/depth, shulker exclusion, inventory/cursor rules, stack merging and Forge overrides.
- RustChunk implementation mapping: Optional bundle adapter on 1.20.1 NBT/registries, isolated from world loading.
- Evidence required before adoption: Overflow/deep nesting tests and vanilla/Forge inventory parity.
- Completion state: COMPLETE — 441 logical lines read through EOF; contiguous coverage; pinned SHA and 16103-byte size match the recursive tree; tests/examples inspected only, not run.

### 1828. `crates/horizon-items/src/content_namespaces.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c4d595e224e9e937c675a570e00201fcd0a7b7ac`
- Size: 1659 bytes; 43 logical lines
- Coverage: 1–43 (EOF)
- Purpose: Discovers content namespaces and mod IDs from generated markers.
- Dependencies / cross-references: horizon-mod-content.json, horizon-mod-ids.json and resource loader.
- Implementation / logic: Reads marker JSON and builds namespace set for data-folder scanning.
- Findings: Horizon-specific markers can be missing, stale or malformed, silently omitting namespaces.
- RustChunk relevance: Reference for content enumeration only, not the Forge registry mechanism.
- Decision: adaptable; registry-integration; version-specific
- 1.20.1 implications: Use 1.20.1 Forge mod/resource manager and registries, not this marker contract.
- Follow-up checks: Check marker creation, malformed/missing files, duplicate IDs, case and pack precedence.
- RustChunk implementation mapping: Implement namespace discovery from real Forge 1.20.1 APIs.
- Evidence required before adoption: Vanilla plus modded registry fixtures, corrupt marker cases and namespace coverage.
- Completion state: COMPLETE — 43 logical lines read through EOF; contiguous coverage; pinned SHA and 1659-byte size match the recursive tree; tests/examples inspected only, not run.

### 1829. `crates/horizon-items/src/cooldowns.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `31e8827127ab950428cb59a648b358d4be77e254`
- Size: 6660 bytes; 174 logical lines
- Coverage: 1–174 (EOF)
- Purpose: Runtime model for item use-cooldowns grouped by a data component.
- Dependencies / cross-references: ItemStack use_cooldown component, ticks and group map.
- Implementation / logic: Converts component seconds to ticks, records group end ticks and reports remaining percentage.
- Findings: Modern schema; invalid floats, conversion overflow and tick wrap need validation.
- RustChunk relevance: Cooldown is gameplay behavior, but this source is version-specific.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Rebuild using 1.20.1 cooldown classes and Forge hooks.
- Follow-up checks: Test NaN/infinity/negative/zero durations, overflow, tick rollover, groups and world reset.
- RustChunk implementation mapping: RustChunk target-version item cooldown manager.
- Evidence required before adoption: 1.20.1 parity and boundary/property tests.
- Completion state: COMPLETE — 174 logical lines read through EOF; contiguous coverage; pinned SHA and 6660-byte size match the recursive tree; tests/examples inspected only, not run.

### 1830. `crates/horizon-items/src/enchant.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `75291649297d6667ce1119e6d37cc959ef60fa30`
- Size: 47359 bytes; 1193 logical lines
- Coverage: 1–400, 401–800, 801–1193 (EOF)
- Purpose: Modern data-driven enchantment registry, selection and effects.
- Dependencies / cross-references: Enchantment registry/tags, ItemStack, item tags, RNG, text and effect model.
- Implementation / logic: Loads tags/metadata, selects random enchantments and evaluates supported conditions/effects.
- Findings: Targets newer effects schema; tag recursion is bounded, malformed inputs may be skipped; RNG parity and numeric bounds need audit.
- RustChunk relevance: High-level algorithm reference; not a compatible 1.20.1 implementation.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Reconstruct 1.20.1 enchantment classes, costs, incompatibilities, loot/anvil and Forge hooks.
- Follow-up checks: Verify RNG, tag cycles, effect conditions, numeric bounds, translations and modded enchantments.
- RustChunk implementation mapping: Rewrite against target 1.20.1 registries/NBT; retain only verified algorithms.
- Evidence required before adoption: Vanilla/Forge fixtures, deterministic seed parity, malformed tags and modded enchant tests.
- Completion state: COMPLETE — 1193 logical lines read through EOF; contiguous coverage; pinned SHA and 47359-byte size match the recursive tree; tests/examples inspected only, not run.

### 1831. `crates/horizon-items/src/firework.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9e6668e5dacf8f3503f255a7a7142fab580910c4`
- Size: 1656 bytes; 61 logical lines
- Coverage: 1–61 (EOF)
- Purpose: Enums/colors for firework explosion shapes.
- Dependencies / cross-references: Dye RGB table and firework_explosion component.
- Implementation / logic: Maps shape strings and dye colors to typed explosion values.
- Findings: Modern representation; no legacy 1.20.1 tag conversion.
- RustChunk relevance: Optional gameplay reference only.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Use 1.20.1 firework tag and target color/shape IDs.
- Follow-up checks: Compare shape IDs, RGB, defaults and multiple explosions to target classes.
- RustChunk implementation mapping: Build a target-version firework tag adapter.
- Evidence required before adoption: 1.20.1 vanilla NBT round-trip and modded firework fixtures.
- Completion state: COMPLETE — 61 logical lines read through EOF; contiguous coverage; pinned SHA and 1656-byte size match the recursive tree; tests/examples inspected only, not run.

### 1832. `crates/horizon-items/src/flowers.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a6dbf7c01b551c3a749697216a50fb72de9e2d08`
- Size: 3992 bytes; 78 logical lines
- Coverage: 1–78 (EOF)
- Purpose: Suspicious-stew flower status effects and durations.
- Dependencies / cross-references: Block/item registry IDs and effect IDs/ticks.
- Implementation / logic: Maps flower IDs to stew effect/duration values.
- Findings: Contains golden dandelion and eyeblossoms, absent from 1.20.1.
- RustChunk relevance: Versioned gameplay data reference.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Use only flowers and effects in 1.20.1 vanilla/Forge.
- Follow-up checks: Compare flower set, effect IDs/durations, stew NBT and modded flowers.
- RustChunk implementation mapping: Generate target stew mapping from 1.20.1 registries.
- Evidence required before adoption: 1.20.1 parity and modded flower behavior tests.
- Completion state: COMPLETE — 78 logical lines read through EOF; contiguous coverage; pinned SHA and 3992-byte size match the recursive tree; tests/examples inspected only, not run.

### 1833. `crates/horizon-items/src/item_data.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ddcda970da953a4d9e05ddf823d182ea27ef30d1`
- Size: 56528 bytes; 1443 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1443 (EOF)
- Purpose: Modern ItemStack component projection and NBT/network round-trip.
- Dependencies / cross-references: Item model, component registry, NBT, text, item registry, nested containers and wire patches.
- Implementation / logic: Encodes/decodes enchantments, names, potions, books, trims, containers, projectiles, fireworks, maps and banners; preserves unknown data.
- Findings: Data-component era differs from 1.20.1 legacy tag payload; preservation is an architecture idea, not wire compatibility proof.
- RustChunk relevance: Core gameplay domain architecture, but exact target implementation needs rewrite.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Implement 1.20.1 ItemStack id/Count/tag NBT, legacy item tags and Forge custom data.
- Follow-up checks: Audit unknown tags, nested container bounds, counts, text, enchantments and server round trips.
- RustChunk implementation mapping: Rebuild ItemStack model/serializer for 1.20.1 vanilla plus Forge.
- Evidence required before adoption: Golden NBT/protocol fixtures for supported item features and modded round trips.
- Completion state: COMPLETE — 1443 logical lines read through EOF; contiguous coverage; pinned SHA and 56528-byte size match the recursive tree; tests/examples inspected only, not run.

### 1834. `crates/horizon-items/src/item_nbt.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e0101228b72a2c46fc0e1aebe23ff280cba24ed0`
- Size: 2026 bytes; 62 logical lines
- Coverage: 1–62 (EOF)
- Purpose: Converts ItemStack and inventory slots to/from newer NBT.
- Dependencies / cross-references: ItemStack codec, NBT compound/list and slot identifiers.
- Implementation / logic: Encodes id/count/components and serializes slot lists; accepts alternate count casing.
- Findings: Target 1.20.1 uses Count/tag; slot-to-u8 may truncate and duplicates need checks.
- RustChunk relevance: Serializer organization reference only.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Implement 1.20.1 legacy ItemStack NBT and inventory slot rules.
- Follow-up checks: Test slot bounds/duplicates, empty slots, count overflow, missing IDs and unknown tags.
- RustChunk implementation mapping: Version-specific RustChunk ItemStack NBT module.
- Evidence required before adoption: Vanilla/Forge NBT round-trip and malformed slot/count fuzz tests.
- Completion state: COMPLETE — 62 logical lines read through EOF; contiguous coverage; pinned SHA and 2026-byte size match the recursive tree; tests/examples inspected only, not run.

### 1835. `crates/horizon-items/src/items.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `67ab3591e8f3c28c1f7d9c950111b935524c44e7`
- Size: 116410 bytes; 2763 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2763 (EOF)
- Purpose: Loads item facts and implements item behavior plus block loot evaluation.
- Dependencies / cross-references: Item registry/default JSON, tags, tools, consumables, RNG, loot tables and mod hooks.
- Implementation / logic: Loads properties and evaluates block drops, conditions, pools and functions; bridges mod-produced stack changes.
- Findings: Partial engine: missing entity/damage/player/biome/luck context; location_check can succeed without world check and entity predicates are incomplete/default false.
- RustChunk relevance: Valuable design reference, but cannot be considered a complete game implementation.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Port exact 1.20.1 item/loot conditions/functions, RNG and Forge hooks with complete LootContext.
- Follow-up checks: Audit every predicate/context field, seeds, recursion, explosion decay, counts, tags and mod hooks.
- RustChunk implementation mapping: Rewrite RustChunk item/loot engine with complete 1.20.1 LootContext and Forge integration.
- Evidence required before adoption: Differential tests against 1.20.1 vanilla/Forge for loot, seeds and modded tables.
- Completion state: COMPLETE — 2763 logical lines read through EOF; contiguous coverage; pinned SHA and 116410-byte size match the recursive tree; tests/examples inspected only, not run.

### 1836. `crates/horizon-items/src/lib.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e94458712884aaf91637762ec2b9cb75d6e5537b`
- Size: 664 bytes; 21 logical lines
- Coverage: 1–21 (EOF)
- Purpose: Module/export boundary for Horizon items crate.
- Dependencies / cross-references: Sibling item/block modules and crate lint settings.
- Implementation / logic: Exports item and block modules; applies forbid(unsafe_code).
- Findings: Small organization file; no runtime behavior.
- RustChunk relevance: Simple modularization/security reference.
- Decision: adaptable; module-structure reference
- 1.20.1 implications: No target-version data semantics.
- Follow-up checks: Review public API boundaries and isolate unsafe code in reviewed platform crates.
- RustChunk implementation mapping: Use explicit module façade and unsafe boundary in RustChunk core.
- Evidence required before adoption: Static API review and target toolchain check.
- Completion state: COMPLETE — 21 logical lines read through EOF; contiguous coverage; pinned SHA and 664-byte size match the recursive tree; tests/examples inspected only, not run.

### 1837. `crates/horizon-items/src/random_sequences.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9fb2a80f4ff62bb1851ff525dc7be5212672d199`
- Size: 6405 bytes; 173 logical lines
- Coverage: 1–173 (EOF)
- Purpose: Named random sequence state for gameplay and loot.
- Dependencies / cross-references: World seed, sequence ID/salt, RNG and NBT persistence.
- Implementation / logic: Creates named streams, advances state and serializes/restores them.
- Findings: Seed mixing and persistence schema are version-specific; tests not run.
- RustChunk relevance: Potentially important for deterministic loot/world logic.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Verify exact 1.20.1 algorithm and state format.
- Follow-up checks: Compare seed mixing, salt, IDs, reset, persistence and long-run determinism.
- RustChunk implementation mapping: Target-specific world/loot random-state manager.
- Evidence required before adoption: Seed differential vectors from 1.20.1 and Forge sequences.
- Completion state: COMPLETE — 173 logical lines read through EOF; contiguous coverage; pinned SHA and 6405-byte size match the recursive tree; tests/examples inspected only, not run.

### 1838. `crates/horizon-items/src/recipe_filters.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `97ec8e8aab20a8aa1ba4bef38bc559cc910d57ee`
- Size: 15155 bytes; 374 logical lines
- Coverage: 1–374 (EOF)
- Purpose: JSON filters remove recipes or replace inputs/outputs.
- Dependencies / cross-references: PackOverlay, recipe JSON, namespace discovery and serde_json.
- Implementation / logic: Loads filters after datapack merge; matches id/type/input/output and applies remove/replace actions in order.
- Findings: Horizon-specific folder/walker may not model all mod serializer schemas; unknown actions are skipped with warnings.
- RustChunk relevance: Optional modpack compatibility layer, not base network/rendering.
- Decision: adaptable; optional/future; recipe-pipeline
- 1.20.1 implications: Use Forge 1.20.1 serializers, conditions and reload lifecycle.
- Follow-up checks: Check nested JSON matching, output variants, tags, ordering and reload invalidation.
- RustChunk implementation mapping: Optional recipe filter after RustChunk 1.20.1 datapack/Forge load.
- Evidence required before adoption: Vanilla/Forge recipe corpus and malformed-filter fuzzing.
- Completion state: COMPLETE — 374 logical lines read through EOF; contiguous coverage; pinned SHA and 15155-byte size match the recursive tree; tests/examples inspected only, not run.

### 1839. `crates/horizon-items/src/use_animation.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `af7f6681780a1ce7a423900127a9749950ea812f`
- Size: 432 bytes; 14 logical lines
- Coverage: 1–14 (EOF)
- Purpose: Maps consumable component animation string to internal enum.
- Dependencies / cross-references: serde_json Value and consumable UseAnimation enum.
- Implementation / logic: Maps eat/drink strings; unknown/absent values use default/None behavior.
- Findings: Tiny helper with modern input schema; no tests.
- RustChunk relevance: Minimal gameplay mapping reference.
- Decision: rewrite required; version-specific
- 1.20.1 implications: Resolve animation from 1.20.1 item classes/metadata and Forge capability.
- Follow-up checks: Verify eat, drink, block, bow, crossbow, spyglass and modded animations.
- RustChunk implementation mapping: Target-version item animation resolver.
- Evidence required before adoption: Vanilla and modded use-animation parity fixtures.
- Completion state: COMPLETE — 14 logical lines read through EOF; contiguous coverage; pinned SHA and 432-byte size match the recursive tree; tests/examples inspected only, not run.

### 1840. `crates/horizon-mcp/Cargo.toml`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `47ba9ff4bad0b36b24bf7b222d2475111a07d051`
- Size: 331 bytes; 13 logical lines
- Coverage: 1–13 (EOF)
- Purpose: Manifest for Horizon Model Context Protocol server crate.
- Dependencies / cross-references: serde_json and workspace lints; Rust 1.88 MSRV.
- Implementation / logic: Declares a standalone MCP utility crate for server processes.
- Findings: Agent-control utility, not game runtime.
- RustChunk relevance: No base client value.
- Decision: unnecessary; server-tooling
- 1.20.1 implications: Keep out of Android client dependency graph.
- Follow-up checks: Revisit only for a separate packaged developer-control process.
- RustChunk implementation mapping: No player client mapping; isolated dev tool only.
- Evidence required before adoption: Separate requirement/security/package review.
- Completion state: COMPLETE — 13 logical lines read through EOF; contiguous coverage; pinned SHA and 331-byte size match the recursive tree; tests/examples inspected only, not run.

### 1841. `crates/horizon-mcp/src/lib.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9014b4bbe1f30070b79235e948f5ab60d4d180a4`
- Size: 19010 bytes; 476 logical lines
- Coverage: 1–400, 401–476 (EOF)
- Purpose: JSON-RPC 2.0 line-framed MCP over stdin/stdout.
- Dependencies / cross-references: serde_json, tool callbacks and stdio I/O.
- Implementation / logic: Supports protocol negotiation, discovery, initialize, tools/list/call, notifications and errors.
- Findings: MCP is an agent protocol, not Minecraft; tests inspected only.
- RustChunk relevance: Possible separate developer tool, not player runtime.
- Decision: unnecessary; server-tooling; protocol-specific
- 1.20.1 implications: No relation to 1.20.1 handshake/login/play/Forge negotiation.
- Follow-up checks: Review line bounds, request IDs, negotiation, stdio ownership and tool authorization if reused.
- RustChunk implementation mapping: Separate developer-control process, never MC packet codec.
- Evidence required before adoption: MCP compliance, oversized input and tool authorization tests.
- Completion state: COMPLETE — 476 logical lines read through EOF; contiguous coverage; pinned SHA and 19010-byte size match the recursive tree; tests/examples inspected only, not run.

### 1842. `crates/horizon-mod-registry/Cargo.lock`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f7f3a35b00261e2ff6784a692d2e8844c631822e`
- Size: 46342 bytes; 1835 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–1835 (EOF)
- Purpose: Pinned dependency graph for hosted mod registry.
- Dependencies / cross-references: Axum/Tokio, SQLite, rustls/ureq, serde, ZIP and platform crates.
- Implementation / logic: Locks standalone registry service dependencies and checksums in Cargo lockfile v4.
- Findings: Server/network/database/wasm dependencies; not game lockfile.
- RustChunk relevance: No client runtime value other than reproducible locking example.
- Decision: unnecessary; server-only; lockfile
- 1.20.1 implications: Do not merge into Android dependency graph.
- Follow-up checks: Check MSRV/Android features only for crates actually adopted.
- RustChunk implementation mapping: Separate lockfile only for a future registry backend.
- Evidence required before adoption: Dependency, license and security audit if adopted.
- Completion state: COMPLETE — 1835 logical lines read through EOF; contiguous coverage; pinned SHA and 46342-byte size match the recursive tree; tests/examples inspected only, not run.

### 1843. `crates/horizon-mod-registry/Cargo.toml`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f5c027b7e8279d68da45e27aed8da3178051c593`
- Size: 1476 bytes; 38 logical lines
- Coverage: 1–38 (EOF)
- Purpose: Manifest for hosted mod submission/rating service.
- Dependencies / cross-references: horizon-mods index, Axum, Tokio, ureq, HMAC, getrandom, rusqlite, Tower.
- Implementation / logic: Defines separate Rust 2024 workspace and web/database dependencies.
- Findings: Explicitly isolates server from game workspace; not client footprint.
- RustChunk relevance: No Android gameplay requirement.
- Decision: unnecessary; server-only; toolchain-specific
- 1.20.1 implications: Keep backend separate from Android runtime.
- Follow-up checks: Review only if RustChunk needs a hosted mod index.
- RustChunk implementation mapping: No MC client mapping; separate backend.
- Evidence required before adoption: Independent backend build/security/deploy validation.
- Completion state: COMPLETE — 38 logical lines read through EOF; contiguous coverage; pinned SHA and 1476-byte size match the recursive tree; tests/examples inspected only, not run.

### 1844. `crates/horizon-mod-registry/Dockerfile`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ee2065008b998d846cfc84f2830c19cc9879c42c`
- Size: 1162 bytes; 18 logical lines
- Coverage: 1–18 (EOF)
- Purpose: Builds/runs registry web service container.
- Dependencies / cross-references: Rust build image, registry crates, Debian, CA certs and setpriv.
- Implementation / logic: Builds locked release binary then runs as unprivileged user with persistent DB volume.
- Findings: Linux container deployment only.
- RustChunk relevance: No Android client value.
- Decision: unnecessary; deployment-only
- 1.20.1 implications: No Android or MC protocol mapping.
- Follow-up checks: Review container permissions, DB volume and image hardening only if needed.
- RustChunk implementation mapping: No client mapping; deployment only.
- Evidence required before adoption: Image scan, UID/volume tests and deployment smoke test.
- Completion state: COMPLETE — 18 logical lines read through EOF; contiguous coverage; pinned SHA and 1162-byte size match the recursive tree; tests/examples inspected only, not run.

### 1845. `crates/horizon-mod-registry/README.md`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a5b8228ba52fad3e2797a6d3ee2447da0542f7d1`
- Size: 6079 bytes; 87 logical lines
- Coverage: 1–87 (EOF)
- Purpose: Documents mod submission, OAuth, ratings and deployment.
- Dependencies / cross-references: GitHub OAuth/token, HMAC owner update, registry and SQLite.
- Implementation / logic: Describes repository/file submissions, PRs, owner updates, ratings, rate limits, CSRF and deployment.
- Findings: Documents 64MiB upload cap, rate limits, origin/CSRF and review workflow; service not run.
- RustChunk relevance: May inform separate mod portal, not the game client.
- Decision: reference; server-security; deployment-only
- 1.20.1 implications: No 1.20.1 runtime integration; secrets must remain server-side.
- Follow-up checks: Verify docs against implementation, scope, privacy, CSRF and operations before reuse.
- RustChunk implementation mapping: Reference only for a separately threat-modeled backend.
- Evidence required before adoption: Threat model, endpoint tests, OAuth/CSRF and retention policy.
- Completion state: COMPLETE — 87 logical lines read through EOF; contiguous coverage; pinned SHA and 6079-byte size match the recursive tree; tests/examples inspected only, not run.

### 1846. `crates/horizon-mod-registry/railway.json`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `cfafe4367faa9ba99102d65a76c561c8fe458bd9`
- Size: 534 bytes; 21 logical lines
- Coverage: 1–21 (EOF)
- Purpose: Deployment build, health-check and restart policy config.
- Dependencies / cross-references: Dockerfile path, watch patterns, /healthz and regional replica config.
- Implementation / logic: Sets Dockerfile build, watched paths, health timeout and restart retries.
- Findings: Provider-specific service deployment metadata.
- RustChunk relevance: No Android/game relevance.
- Decision: unnecessary; deployment-only
- 1.20.1 implications: No 1.20.1 mapping.
- Follow-up checks: Only revisit if RustChunk deploys its own service there.
- RustChunk implementation mapping: No client mapping.
- Evidence required before adoption: Provider schema and deployment smoke test.
- Completion state: COMPLETE — 21 logical lines read through EOF; contiguous coverage; pinned SHA and 534-byte size match the recursive tree; tests/examples inspected only, not run.

### 1847. `crates/horizon-mod-registry/src/github.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0ff5b28f4464a27523f136b6b1653f3623f8c4a3`
- Size: 12970 bytes; 390 logical lines
- Coverage: 1–390 (EOF)
- Purpose: GitHub API adapter for repo lookup, release staging, branches and PRs.
- Dependencies / cross-references: ureq, OAuth/API endpoints, registry token, Entry model, JSON/Base64 and files.
- Implementation / logic: Does OAuth profile lookup, reads entries, checks public repositories, uploads releases, manages branches/resets, commits files and opens PRs.
- Findings: Upload reads whole file in memory; branch reset force-updates; raw OAuth state inserted by helper (caller uses random hex and signed-cookie check); token not zeroized explicitly.
- RustChunk relevance: Reference for a separate mod portal only.
- Decision: reference; server-tooling; security-sensitive
- 1.20.1 implications: Keep credentials server-side; never embed GitHub tokens in Android launcher.
- Follow-up checks: Audit repo/path validation, OAuth, token scope/lifetime, reset authorization, memory limits and API rates.
- RustChunk implementation mapping: Separate server-only registry if explicitly required.
- Evidence required before adoption: Threat model, OAuth tests, restricted token, hostile repo/path and mock API tests.
- Completion state: COMPLETE — 390 logical lines read through EOF; contiguous coverage; pinned SHA and 12970-byte size match the recursive tree; tests/examples inspected only, not run.

### 1848. `crates/horizon-mod-registry/src/lib.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `edcf53fc53bc4eff5e503dc7cc628ff5b7d1489d`
- Size: 47987 bytes; 1312 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1312 (EOF)
- Purpose: Axum app for GitHub sign-in, submissions, owner updates and ratings.
- Dependencies / cross-references: GitHub adapter, HMAC/SHA-256, SQLite, Axum multipart/JSON, cookies and index validator.
- Implementation / logic: Implements signed sessions/CSRF, origin checks, rate limits, streamed uploads, file inspection, HMAC owner updates, PR and ratings.
- Findings: Positive controls include signed session/state, Secure/HttpOnly/SameSite, origin+CSRF and body limits. Rate windows are in-process memory; tests inspected only.
- RustChunk relevance: Server-side workflow reference only.
- Decision: unnecessary; server-tooling; security-sensitive
- 1.20.1 implications: Keep OAuth secrets, GitHub token and DB backend-only.
- Follow-up checks: Review origin parsing, multi-replica rates, cookie lifecycle, duplicate multipart, temp cleanup, HMAC and idempotency.
- RustChunk implementation mapping: No player client mapping; isolated backend only.
- Evidence required before adoption: Security review, parser fuzzing, OAuth/CSRF, multi-replica and privacy tests.
- Completion state: COMPLETE — 1312 logical lines read through EOF; contiguous coverage; pinned SHA and 47987-byte size match the recursive tree; tests/examples inspected only, not run.

### 1849. `crates/horizon-mod-registry/src/main.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a1e40a57a8f2d728d5763c486d72c45876e9fa4d`
- Size: 1101 bytes; 26 logical lines
- Coverage: 1–26 (EOF)
- Purpose: Executable entry point for registry web service.
- Dependencies / cross-references: Tokio runtime, Config, App/router, TCP listener and Ctrl-C.
- Implementation / logic: Parses PORT fallback, binds 0.0.0.0, loads config/SQLite, serves Axum and handles graceful shutdown.
- Findings: Backend process entrypoint only.
- RustChunk relevance: No Android gameplay value.
- Decision: unnecessary; server-tooling
- 1.20.1 implications: Not part of Minecraft 1.20.1 implementation.
- Follow-up checks: For future service, verify bind exposure, env validation and shutdown.
- RustChunk implementation mapping: Keep server main in separate workspace.
- Evidence required before adoption: Container smoke test and DB shutdown consistency.
- Completion state: COMPLETE — 26 logical lines read through EOF; contiguous coverage; pinned SHA and 1101-byte size match the recursive tree; tests/examples inspected only, not run.

### 1850. `crates/horizon-mod-registry/src/ratings.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0cba8514298f3332d80e6a1fca3f7426ca81b416`
- Size: 4187 bytes; 129 logical lines
- Coverage: 1–129 (EOF)
- Purpose: SQLite per-account/per-project ratings and aggregate summaries.
- Dependencies / cross-references: rusqlite, WAL schema, unique project/account key and SQL aggregate queries.
- Implementation / logic: Creates ratings table, upserts 1–5 stars and reads individual/project summaries.
- Findings: Server-side data; u64 account/timestamp cast to i64 and count back to u64; DB checks star range; tests inspected only.
- RustChunk relevance: Hosted registry only; unrelated to Minecraft client/mod loading/chunk performance.
- Decision: unnecessary; server-tooling
- 1.20.1 implications: Keep ratings DB server-side.
- Follow-up checks: Review integer bounds, schema migration, privacy/retention and backup/restore.
- RustChunk implementation mapping: Separate optional backend DB only if RustChunk runs a mod portal.
- Evidence required before adoption: SQLite migration, integer-boundary, privacy and restore tests.
- Completion state: COMPLETE — 129 logical lines read through EOF; contiguous coverage; pinned SHA and 4187-byte size match the recursive tree; tests/examples inspected only, not run.

## Checkpoint 1851–1875

### 1851. `crates/horizon-mod-registry/tests/submit.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ab08ac94af6c6cfc79805b7d2e27f12059920b70`
- Size: 42905 bytes; 1056 logical lines
- Coverage: 1–400, 401–800, 801–1056 (EOF)
- Purpose: Integration tests for registry submissions, GitHub PR creation, owner updates, ratings, rejection cases and retry behavior.
- Dependencies / cross-references: Axum router, mock GitHub API, temporary ZIP fixtures, HMAC, SQLite-backed registry and Tower test client.
- Implementation / logic: Exercises sign-in/CSRF, upload checks, signatures, PR generation, owner-update HMAC, limits, retry/timeout and rating flows.
- Findings: Mock-driven server tests only; fixtures and test expectations describe Horizon 26.3 packages, not Forge behavior. No test was executed.
- RustChunk relevance: Server/backend test reference only; no Android gameplay or chunk/rendering implementation.
- Decision: undetermined; tests only; server-tooling
- 1.20.1 implications: No direct 1.20.1 client semantics; registry fixture uses game 26.3 and custom .hmod/.hnmod formats.
- Follow-up checks: Review service behavior if a separate mod portal is ever built; execute tests only in an isolated server workspace.
- RustChunk implementation mapping: No player-client mapping; possible backend regression-test patterns only.
- Evidence required before adoption: Isolated registry build, test execution, security review and mock/real GitHub boundary tests.
- Completion state: COMPLETE — 1056 logical lines read through EOF in contiguous chunks <=400; pinned SHA and 42905-byte size match; tests inspected, not run.

### 1852. `crates/horizon-mods/Cargo.toml`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0c003cc7ebfecc8efebaa7391fee8e90d3e788c4`
- Size: 1166 bytes; 29 logical lines
- Coverage: 1–29 (EOF)
- Purpose: Manifest for Horizon's mod package/content/index crate.
- Dependencies / cross-references: Serde/JSON/TOML, ZIP deflate, log; optional wasmparser, SHA-256, minisign and shader API behind index feature.
- Implementation / logic: Defines Rust 1.95 package, feature-gated registry index dependencies and workspace lints.
- Findings: Custom package/index formats and modern dependency versions; no Android target declaration or Forge bridge.
- RustChunk relevance: Package-format concept can inform a separate RustChunk mod metadata layer, not a direct dependency.
- Decision: rewrite required; version-specific; optional
- 1.20.1 implications: Must model Forge 1.20.1 mods, metadata, dependencies and loader lifecycle rather than Horizon API/content package.
- Follow-up checks: Check Android Rust MSRV and ZIP/crypto crate support only after selecting required components.
- RustChunk implementation mapping: New RustChunk mod metadata/verification crate, isolated from game runtime until needed.
- Evidence required before adoption: Android ARM64 dependency build, license/security review and Forge 1.20.1 metadata interoperability.
- Completion state: COMPLETE — 29 logical lines read through EOF; pinned SHA and 1166-byte size match; tests not present.

### 1853. `crates/horizon-mods/src/api_level.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e3df644cc115c8f1d7a3ec3e8e22301051de89e5`
- Size: 7240 bytes; 204 logical lines
- Coverage: 1–204 (EOF)
- Purpose: Versioned compatibility level for Horizon's Wasm component imports.
- Dependencies / cross-references: Horizon WIT function names, wasmparser, registry index api_level field and fixed LEVEL/FUNCTIONS table.
- Implementation / logic: Maps imported interface.function names to the newest API level; embeds and reads a 27-byte marker in Wasm binaries.
- Findings: Hard-coded Horizon function catalog and level 7; unknown import is conservatively treated as newer; only component imports are parsed.
- RustChunk relevance: Compatibility-gating pattern is useful, but the API contract itself is unrelated to Forge mod compatibility.
- Decision: rewrite required; reference; version-specific
- 1.20.1 implications: Use Forge/Minecraft 1.20.1 mod loader versions, mod IDs, dependency ranges and protocol compatibility; do not reuse Horizon levels.
- Follow-up checks: Define explicit RustChunk/Forge compatibility metadata and reject unsupported loader/game versions before install.
- RustChunk implementation mapping: RustChunk profile mod resolver and metadata compatibility gate, not a replacement for Forge.
- Evidence required before adoption: Differential tests against Forge 47.4.x metadata, malformed Wasm/parser bounds if Wasm is retained, and Android build.
- Completion state: COMPLETE — 204 logical lines read through EOF; pinned SHA and 7240-byte size match; embedded unit tests inspected, not run.

### 1854. `crates/horizon-mods/src/content.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `96af309222f040d401946eb411153fb6d44328da`
- Size: 99453 bytes; 2486 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2400, 2401–2486 (EOF)
- Purpose: Loads and validates Horizon content assets and declarative blocks, items, effects, menus and kinetic machines.
- Dependencies / cross-references: Manifest namespaces/capabilities, serde JSON, ZIP reader, file walker, block/item component schema and content fetch cache.
- Implementation / logic: Scans directories/archives, caps file counts and bytes, parses typed declarations, checks namespace ownership, geometry, state properties, machine metadata and cross-declaration references.
- Findings: Modern component and registry schema is not 1.20.1 legacy NBT/Forge registry data. Several custom machine behaviors are Starling/Create-like; block model/shape validation is partial relative to Minecraft's full state/model rules.
- RustChunk relevance: Strong architecture reference for bounded asset scanning and typed data validation; not a drop-in game data engine.
- Decision: rewrite required; core reference; version-specific
- 1.20.1 implications: Rebuild against 1.20.1 registries, BlockState/ItemStack NBT, Forge data loaders, loot/recipe serializers and vanilla collision/render semantics.
- Follow-up checks: Differentially compare all declarations against 1.20.1/Forge; fuzz ZIP paths/JSON; test namespace collisions, state-space limits and shape boundaries.
- RustChunk implementation mapping: RustChunk data-pack/mod asset ingestion layer, with target-specific registries and serializers.
- Evidence required before adoption: 1.20.1 vanilla+Forge parity fixtures, malformed archive fuzzing, Android ARM64 memory/CPU tests and explicit block geometry/render validation.
- Completion state: COMPLETE — 2486 logical lines read through EOF in contiguous chunks <=400; pinned SHA and 99453-byte size match; tests inspected, not run.

### 1855. `crates/horizon-mods/src/fetch.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f26c14d7c051f5e6f01d8c65281262b8dd56a2e6`
- Size: 11529 bytes; 294 logical lines
- Coverage: 1–294 (EOF)
- Purpose: Downloads and unpacks optional external mod assets from Modrinth by SHA-512.
- Dependencies / cross-references: Manifest Fetch metadata, Modrinth version API/CDN, ZIP archive, content path/size limits and JSON normalizer.
- Implementation / logic: Finds file by hash, enforces Modrinth CDN URL prefix, extracts selected assets into partial cache, normalizes JSON and writes completion marker.
- Findings: Completion marker trusts cache contents without rehash; duplicate archive paths can overwrite; bounded aggregate extraction exists. Feature is third-party-asset fetch, not standard mod install.
- RustChunk relevance: Optional asset-fetch pattern only; not needed for ordinary Forge mod downloads from user/server sources.
- Decision: unnecessary; optional; external-service
- 1.20.1 implications: RustChunk should not silently fetch unrelated external assets; any mod downloads need explicit source, consent, integrity and cache policy.
- Follow-up checks: If adopted, verify cached files, reject duplicate paths, constrain extraction roots and test CDN redirects/host validation.
- RustChunk implementation mapping: Separate optional asset acquisition/cache module; never part of Minecraft packet or chunk pipeline.
- Evidence required before adoption: Hash verification, malicious ZIP/duplicate-entry tests, path traversal and cache-tamper tests on Android.
- Completion state: COMPLETE — 294 logical lines read through EOF; pinned SHA and 11529-byte size match; tests inspected, not run.

### 1856. `crates/horizon-mods/src/index.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `382556b50c238fc07eef17bdbf4852da32a81ab1`
- Size: 25608 bytes; 695 logical lines
- Coverage: 1–695 (EOF)
- Purpose: Parses and validates mod registry index, file trust, signatures, revocations and package contents.
- Dependencies / cross-references: Serde, SHA-256, minisign, ZIP, horizon-mods manifest/modpack/shader validators and optional index feature.
- Implementation / logic: Checks project/version IDs, file names, HTTPS strings, sizes, signatures, dependency hashes, revocations and inspects package-specific manifests before install.
- Findings: HTTPS validation is prefix-only rather than full URL parsing; resource-pack inspection checks path traversal but lacks explicit total expanded-byte/entry caps; revocation metadata validation is limited; unknown kinds are retained but not installable.
- RustChunk relevance: Integrity/index design is a useful pattern; package kinds and trust policy need redesign for Forge ecosystem.
- Decision: rewrite required; security-sensitive; reference
- 1.20.1 implications: Forge 1.20.1 mod distribution requires Modrinth/CurseForge/local sources, loader/game dependency constraints, signed hash manifests and safe archive inspection.
- Follow-up checks: Use URL parser and allowlisted schemes/hosts, ZIP bomb bounds, canonical revocation schema, dependency-cycle checks and strict manifest parsing.
- RustChunk implementation mapping: RustChunk optional mod catalog/installer trust layer, separate from Forge's actual mod loader.
- Evidence required before adoption: Threat model, URL/parser fuzzing, ZIP bomb corpus, signature vectors, dependency resolution tests and Android storage permission tests.
- Completion state: COMPLETE — 695 logical lines read through EOF; pinned SHA and 25608-byte size match; tests inspected, not run.

### 1857. `crates/horizon-mods/src/index/native.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `64dafe6e3b70667d7ef5874e8b94acb13d52d9ff`
- Size: 17259 bytes; 464 logical lines
- Coverage: 1–464 (EOF)
- Purpose: Reads and validates source-only Horizon native mixin bundles.
- Dependencies / cross-references: ZIP, serde JSON, TOML, manifest identity, Rust source/data path policy and build-service bundle format.
- Implementation / logic: Rejects unsafe paths and WebAssembly, caps source entry count/expanded bytes, checks manifest/mixin identity and emits deterministic build ZIP.
- Findings: Native bundle build pipeline is Horizon-specific; content entries are skipped from source bundle and no RustChunk/Forge native mod ABI exists.
- RustChunk relevance: Bounded source-bundle validation pattern only; not a route to load Forge mods from Rust.
- Decision: unnecessary; optional/future; toolchain-specific
- 1.20.1 implications: Forge 1.20.1 uses Java bytecode/mod loader; this bundle format cannot replace Forge .jar mods or mixins without a separate compiler/loader.
- Follow-up checks: Do not execute uploaded source; enforce per-entry and aggregate limits, duplicate/path checks and build isolation if a service is added.
- RustChunk implementation mapping: No base client mapping; separate future mod-build service only.
- Evidence required before adoption: Sandboxed build service, malicious ZIP tests, dependency pinning, output reproducibility and Android-independent validation.
- Completion state: COMPLETE — 464 logical lines read through EOF; pinned SHA and 17259-byte size match; tests inspected, not run.

### 1858. `crates/horizon-mods/src/index/registry.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5d9cd1047ed3db336bae29018e719b0199beaafa`
- Size: 26181 bytes; 689 logical lines
- Coverage: 1–689 (EOF)
- Purpose: Builds and reviews the hosted mod registry from TOML project entries and release assets.
- Dependencies / cross-references: Index models, SHA-256/minisign, package inspection, local registry cache, GitHub release URLs and review workflow.
- Implementation / logic: Loads entries/revocations, validates versions and files, resolves repository page metadata, builds index, checks release assets and flags immutable-file/owner/kind changes.
- Findings: New repository releases can be imported from configured repo without per-release review; review returns notes rather than hard errors for owner changes; date validation checks shape not calendar validity.
- RustChunk relevance: Backend registry governance reference, not a game client component.
- Decision: unnecessary; server-tooling; security-sensitive
- 1.20.1 implications: Keep GitHub automation, release signing and repository authority on a backend; do not embed tokens or CI trust in Android launcher.
- Follow-up checks: Validate repo ownership and release provenance, calendar dates, version ordering, revoked IDs, duplicate cache records and signed build artifacts.
- RustChunk implementation mapping: Separate RustChunk mod catalog backend only if explicitly built.
- Evidence required before adoption: Threat model, hostile repository tests, release asset hash/signature validation and review authorization tests.
- Completion state: COMPLETE — 689 logical lines read through EOF; pinned SHA and 26181-byte size match; tests inspected, not run.

### 1859. `crates/horizon-mods/src/index/registry/tests.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c35ad4acf63bb2270784451aa6fca1352098d050`
- Size: 13747 bytes; 408 logical lines
- Coverage: 1–408 (EOF)
- Purpose: Tests registry index construction, repository-backed releases, immutable versions and kind transitions.
- Dependencies / cross-references: Scratch registry checkout, sample signed/unsigned packages, cache files and index builder.
- Implementation / logic: Asserts sample index reproduction, signature/hash failures, review notes, mod-to-native transition and release/page resolution.
- Findings: Tests use Horizon sample versions and custom formats; none executed during this audit.
- RustChunk relevance: Backend regression-test reference only.
- Decision: undetermined; tests only; server-tooling
- 1.20.1 implications: No 1.20.1 runtime implications.
- Follow-up checks: Run only in isolated backend workspace after implementing registry; add adversarial repository/release fixtures.
- RustChunk implementation mapping: No player-client mapping.
- Evidence required before adoption: Test execution, malicious repo/path fixtures, release integrity and cache migration tests.
- Completion state: COMPLETE — 408 logical lines read through EOF; pinned SHA and 13747-byte size match; tests inspected, not run.

### 1860. `crates/horizon-mods/src/index/tests.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f507296291ebafc08a7946bd965fe41354a301bd`
- Size: 10611 bytes; 260 logical lines
- Coverage: 1–260 (EOF)
- Purpose: Tests sample index parsing, package checks, signatures, revocation and resource-pack validation.
- Dependencies / cross-references: Index parser, test public key, .hmod/.hpack/.hshader fixtures and ZIP creation helpers.
- Implementation / logic: Exercises unknown kinds, tampering, signatures, manifest parity, resource-pack metadata and installable API-level selection.
- Findings: Sample game target is 26.3; resource-pack ZIP inspection lacks demonstrated expanded-size stress coverage; tests were not run.
- RustChunk relevance: Test fixture patterns for package trust and negative validation cases.
- Decision: undetermined; tests only
- 1.20.1 implications: Build equivalent fixtures for Forge 1.20.1 .jar/mod metadata and vanilla resource packs; do not assume custom formats.
- Follow-up checks: Add ZIP bomb, duplicate entry, malformed URL, revocation and signature boundary tests.
- RustChunk implementation mapping: RustChunk installer test corpus only.
- Evidence required before adoption: Run unit tests in isolated workspace and fuzz archive/index parsers.
- Completion state: COMPLETE — 260 logical lines read through EOF; pinned SHA and 10611-byte size match; tests inspected, not run.

### 1861. `crates/horizon-mods/src/lib.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `19c61266b60256bfbce3bcb3d0d902e3c9b584c9`
- Size: 430 bytes; 14 logical lines
- Coverage: 1–14 (EOF)
- Purpose: Module façade for Horizon mod APIs, content, fetch, manifest and modpack crates.
- Dependencies / cross-references: Sibling Rust modules and feature-gated index module.
- Implementation / logic: Exports public modules and selected manifest/content types.
- Findings: Small organization file; public API is tied to Horizon-specific formats.
- RustChunk relevance: Module-boundary reference only.
- Decision: adaptable; module-structure reference
- 1.20.1 implications: No game-version semantics.
- Follow-up checks: Keep client/server/loader APIs separated and avoid unnecessary public surface.
- RustChunk implementation mapping: RustChunk crate organization.
- Evidence required before adoption: Static API review and cargo feature matrix.
- Completion state: COMPLETE — 14 logical lines read through EOF; pinned SHA and 430-byte size match.

### 1862. `crates/horizon-mods/src/manifest.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `31903fb51870fa5fd2a5329a919330e8150e1667`
- Size: 25891 bytes; 627 logical lines
- Coverage: 1–627 (EOF)
- Purpose: Defines Horizon mod manifest, capability model, package reader/writer and file discovery.
- Dependencies / cross-references: Serde/TOML, ZIP, Wasm component APIs, content path checker and platform-independent package files.
- Implementation / logic: Validates mod IDs, API levels, side/capabilities, namespaces, optional asset fetch and package structure; reads/writes .hmod.
- Findings: Manifest API 0.1/0.2/0.3 and capability model are not Forge 1.20.1 metadata; directory Package::read reads Wasm without a streaming cap before checking size; archive writer relies on ZIP path checks but doesn't reject duplicate content names.
- RustChunk relevance: Metadata validation and explicit capability declarations are useful patterns, not a compatible mod format.
- Decision: rewrite required; version-specific; security-sensitive
- 1.20.1 implications: Forge mod metadata, dependencies, mixins, access transformers and loader versions must be modeled as Forge 47.x expects; do not substitute this manifest.
- Follow-up checks: Stream/cap directory Wasm reads, reject duplicate ZIP entries, cap archive entry counts and align metadata with Forge dependency graph.
- RustChunk implementation mapping: RustChunk mod metadata/index layer only if it preserves real Forge jars and Forge semantics.
- Evidence required before adoption: Android archive tests, malformed ZIP corpus, manifest fuzzing and Forge 1.20.1 metadata parity.
- Completion state: COMPLETE — 627 logical lines read through EOF; pinned SHA and 25891-byte size match; tests inspected, not run.

### 1863. `crates/horizon-mods/src/modpack.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2ee9543737fd5a52d2701608342cec578938f918`
- Size: 10798 bytes; 300 logical lines
- Coverage: 1–300 (EOF)
- Purpose: Reads Horizon .hpack manifests and extracts constrained profile overrides.
- Dependencies / cross-references: ZIP, TOML manifest, listed mod IDs, storage directory policy and game home paths.
- Implementation / logic: Validates pack version and mod hashes, constrains override destinations and extracts options/resourcepacks/mod storage.
- Findings: Extraction writes through existing filesystem paths without explicit symlink-safe openat semantics; duplicate ZIP paths can be processed repeatedly; custom .hpack schema differs from common Forge modpack formats.
- RustChunk relevance: Profile packaging/override whitelist is useful concept; direct format reuse is not.
- Decision: rewrite required; optional; security-sensitive
- 1.20.1 implications: Forge modpacks typically carry manifests and actual mod artifacts/loader metadata; retain 1.20.1 Forge install semantics and safe profile isolation.
- Follow-up checks: Reject symlink traversal, duplicate entries, oversized/zip-bomb archives and path collisions; stage writes then atomically replace.
- RustChunk implementation mapping: RustChunk profile import/export and safe overrides subsystem.
- Evidence required before adoption: Malicious archive/symlink tests, extraction rollback, duplicate path and Android scoped-storage validation.
- Completion state: COMPLETE — 300 logical lines read through EOF; pinned SHA and 10798-byte size match; tests inspected, not run.

### 1864. `crates/horizon-mods/tests/index/coords_hud-0.1.0.hmod`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1a4a5ba35e922e3837f43a4c85ae532885f95956`
- Size: 438 bytes; binary fixture
- Coverage: binary ZIP; complete base64 blob fetched
- Purpose: Signed Horizon example mod package fixture.
- Dependencies / cross-references: ZIP contains horizon-mod.toml and mod.wasm; referenced by sample index and registry tests.
- Implementation / logic: Fixture is a 438-byte archive whose SHA-256 is pinned by sample index.
- Findings: Binary test fixture only; custom Horizon format, not Forge .jar.
- RustChunk relevance: Test data only.
- Decision: unnecessary; fixture-only
- 1.20.1 implications: No runtime adoption; cannot substitute for Forge 1.20.1 mod JAR.
- Follow-up checks: Preserve only as a fixture if testing Horizon format parser.
- RustChunk implementation mapping: No application mapping.
- Evidence required before adoption: Reproduce exact hash and archive structure before any fixture use.
- Completion state: COMPLETE — Complete binary blob fetched; pinned SHA and 438-byte size match; not executed.

### 1865. `crates/horizon-mods/tests/index/coords_hud-0.1.0.hmod.minisig`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5f9b17534b7f4945deede8bd1c3bd6150f1c3965`
- Size: 278 bytes; 5 logical lines
- Coverage: 1–5 (EOF)
- Purpose: Minisign signature fixture for the sample coordinates HUD package.
- Dependencies / cross-references: Test public key test.pub and .hmod package hash/signature.
- Implementation / logic: Contains untrusted comment, signature payload and trusted comment identifying coords_hud 0.1.0.
- Findings: Throwaway test signature; no production trust root.
- RustChunk relevance: Useful signature fixture pattern only.
- Decision: unnecessary; fixture-only; security-sensitive
- 1.20.1 implications: Never trust this test key or signature for released RustChunk mods.
- Follow-up checks: Generate a dedicated production signing key offline and keep secrets out of launcher/source repository.
- RustChunk implementation mapping: Test-only signing fixture.
- Evidence required before adoption: Verify against exact fixture and ensure no test key is trusted in production.
- Completion state: COMPLETE — 5 logical lines read through EOF; pinned SHA and 278-byte size match.

### 1866. `crates/horizon-mods/tests/index/cozy-1.0.0.hpack`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b256634800edef5c08a63d9036273dd9d3bb83d4`
- Size: 471 bytes; binary fixture
- Coverage: binary ZIP; complete base64 blob fetched
- Purpose: Horizon sample modpack archive fixture with one options override.
- Dependencies / cross-references: ZIP contains horizon-modpack.toml and overrides/options.txt.
- Implementation / logic: Archive is referenced by sample index with pinned hash/size and exercised by registry/index tests.
- Findings: Custom Horizon modpack format; binary fixture only.
- RustChunk relevance: Test data only.
- Decision: unnecessary; fixture-only
- 1.20.1 implications: No direct Forge 1.20.1 pack compatibility.
- Follow-up checks: Use as a negative/format fixture only, not as RustChunk's modpack format.
- RustChunk implementation mapping: No application mapping.
- Evidence required before adoption: Exact hash and archive parsing verification in isolated fixture tests.
- Completion state: COMPLETE — Complete binary blob fetched; pinned SHA and 471-byte size match; not executed.

### 1867. `crates/horizon-mods/tests/index/glow-1.0.0.hshader`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `dda1651f89f0f8cdb22180846e9e10316dcb1680`
- Size: 689 bytes; binary fixture
- Coverage: binary ZIP; complete base64 blob fetched
- Purpose: Horizon WGSL shader pack fixture.
- Dependencies / cross-references: ZIP contains horizon-shaders.toml and post.wgsl.
- Implementation / logic: Sample index pins archive hash/size; shader validator tests exercise valid and invalid WGSL.
- Findings: Not a Minecraft 1.20.1 shaderpack or OptiFine/Iris shader archive.
- RustChunk relevance: Test data only.
- Decision: unnecessary; fixture-only
- 1.20.1 implications: Forge 1.20.1 shader ecosystem requires actual supported renderer/shader API, not Horizon .hshader.
- Follow-up checks: Keep separate from resource-pack/mod ZIP handling; validate only with its own format parser.
- RustChunk implementation mapping: No application mapping.
- Evidence required before adoption: Renderer-specific shader compatibility and exact fixture hash check.
- Completion state: COMPLETE — Complete binary blob fetched; pinned SHA and 689-byte size match; not executed.

### 1868. `crates/horizon-mods/tests/index/index-v1.json`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5b4a87d0972bcd09dabf57b4869d10a5ea4f4c74`
- Size: 4349 bytes; 139 logical lines
- Coverage: 1–139 (EOF)
- Purpose: Sample registry index containing mod, modpack, shader and unknown future kind.
- Dependencies / cross-references: References test package hashes, signatures, project metadata, dependencies and revocation.
- Implementation / logic: Defines schema 1 projects with version/game/API metadata, file URLs, SHA-256, signatures and revoked entry.
- Findings: Sample versions target 26.3; unknown kind demonstrates forward retention but not installability; future_field intentionally tolerated.
- RustChunk relevance: Index schema example and integrity fixture only.
- Decision: adaptable; reference; version-specific
- 1.20.1 implications: Need explicit Forge 1.20.1 loader/game compatibility and real .jar source provenance; do not reuse sample entries as runtime data.
- Follow-up checks: Validate schema evolution, URL parsing, unknown fields, dependency graph and revocation semantics.
- RustChunk implementation mapping: RustChunk optional catalog schema fixture.
- Evidence required before adoption: Parser round-trip, unknown-kind and malformed-data tests.
- Completion state: COMPLETE — 139 logical lines read through EOF; pinned SHA and 4349-byte size match.

### 1869. `crates/horizon-mods/tests/index/test.pub`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `5cb41bd62d869630ba2e4e742663318247f6cb51`
- Size: 113 bytes; 3 logical lines
- Coverage: 1–3 (EOF)
- Purpose: Public half of the throwaway minisign test key.
- Dependencies / cross-references: Paired with signature fixture and sample index.
- Implementation / logic: Contains key identifier comment and base64 public key.
- Findings: Test-only key; private half deleted; not a RustChunk production key.
- RustChunk relevance: Test fixture only.
- Decision: unnecessary; fixture-only; security-sensitive
- 1.20.1 implications: Never pin this key in production launcher or trust store.
- Follow-up checks: Generate and distribute an independent RustChunk signing key with rotation/revocation plan.
- RustChunk implementation mapping: No app mapping.
- Evidence required before adoption: Key provenance, offline private-key custody and signature verification tests.
- Completion state: COMPLETE — 3 logical lines read through EOF; pinned SHA and 113-byte size match.

### 1870. `crates/horizon-narrator/Cargo.toml`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `31eaf3d2ee0b876d4e85435b3b6f68f74f1861b7`
- Size: 1159 bytes; 28 logical lines
- Coverage: 1–28 (EOF)
- Purpose: Manifest for cross-platform text-to-speech and screen narration crate.
- Dependencies / cross-references: Windows SAPI, macOS AppKit speech, Linux libloading/flite; log.
- Implementation / logic: Declares target-specific native speech dependencies and Rust 1.85 minimum.
- Findings: No target_os=android backend or Android speech dependency is declared; Android does not match target_os=linux.
- RustChunk relevance: Accessibility architecture is relevant, but this manifest does not establish Android narrator support.
- Decision: adaptable; optional; platform-specific
- 1.20.1 implications: Android TTS needs JNI/Android TextToSpeech bridge or another verified backend; Linux Flite code is not compiled for Android.
- Follow-up checks: Add Android target implementation and lifecycle/thread tests; check native dependency packaging.
- RustChunk implementation mapping: Optional RustChunk accessibility narrator crate with Android adapter.
- Evidence required before adoption: Android API integration, lifecycle, locale/voice, interruption and memory tests.
- Completion state: COMPLETE — 28 logical lines read through EOF; pinned SHA and 1159-byte size match.

### 1871. `crates/horizon-narrator/examples/say.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2d76ce239076344c6470eaad1f49c8359a858f33`
- Size: 1074 bytes; 29 logical lines
- Coverage: 1–29 (EOF)
- Purpose: CLI example for testing platform speech output.
- Dependencies / cross-references: horizon-narrator public API and command-line args.
- Implementation / logic: Parses optional volume, initializes narrator, queues speech and sleeps heuristically before exit.
- Findings: Example-only; sleep duration uses message byte length and is not a reliable completion signal for multilingual speech.
- RustChunk relevance: Useful manual diagnostic pattern only; not mobile gameplay code.
- Decision: unnecessary; example-only
- 1.20.1 implications: No Android CLI or platform backend proof.
- Follow-up checks: Replace fixed sleep with completion signaling if adapting diagnostic harness.
- RustChunk implementation mapping: Developer diagnostic only.
- Evidence required before adoption: Platform smoke tests and asynchronous completion test.
- Completion state: COMPLETE — 29 logical lines read through EOF; pinned SHA and 1074-byte size match.

### 1872. `crates/horizon-narrator/src/lib.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3e66cf3b5bb232aaf112d0ebbf123dc239a5c531`
- Size: 13352 bytes; 422 logical lines
- Coverage: 1–400, 401–422 (EOF)
- Purpose: Core narrator state, status filtering, queued speech API and screen-narration façade.
- Dependencies / cross-references: Speech::Sink trait, global Mutex<GameNarrator>, speech thread and platform modules.
- Implementation / logic: Maintains Off/All/Chat/System states, volume, recent 32 messages and asynchronous queued/interrupting speech.
- Findings: Global mutex is held while calling sink methods; speech command channel is unbounded; set_volume does not validate finite 0..1 range; platform open failure disables narration.
- RustChunk relevance: Accessibility state machine and bounded recent-message idea are reusable; implementation needs Android adapter and queue bounds.
- Decision: adaptable; optional; platform-specific; architecture reference
- 1.20.1 implications: No Android speech backend in this source; volume and thread semantics need mobile-specific handling.
- Follow-up checks: Bound speech queue, validate/clamp volume, avoid slow platform calls under global lock and handle lifecycle cancellation.
- RustChunk implementation mapping: Optional RustChunk accessibility service separate from render/game thread.
- Evidence required before adoption: Concurrency stress, queue flood, NaN/range, Android lifecycle and TTS interruption tests.
- Completion state: COMPLETE — 422 logical lines read through EOF in contiguous chunks <=400; pinned SHA and 13352-byte size match; tests inspected, not run.

### 1873. `crates/horizon-narrator/src/screen.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `748e0a88c10a8ff68dbc7d54eb3a3a78fd8c12d8`
- Size: 14121 bytes; 418 logical lines
- Coverage: 1–400, 401–418 (EOF)
- Purpose: Screen narration collector, widget priority selection and delayed narration scheduling.
- Dependencies / cross-references: BTreeMap keyed by element type/depth, generation counter, screen trigger/time state.
- Implementation / logic: Collects changed narration elements in order, suppresses after screen init and schedules mouse/keyboard/system narration.
- Findings: Collector retains old entries indefinitely; generation i64 increment can overflow in very long-lived state; caller must maintain stable widget IDs and monotonic clock.
- RustChunk relevance: Useful UI accessibility behavior for RustChunk settings/menus, not Minecraft protocol or renderer core.
- Decision: adaptable; optional; UI reference
- 1.20.1 implications: Port interaction model to Android accessibility/TalkBack conventions; do not assume desktop mouse/keyboard timing.
- Follow-up checks: Prune stale entries, bound depth/count, test clock rollback/overflow and dynamic widget removal.
- RustChunk implementation mapping: Optional UI narration layer around RustChunk settings screens.
- Evidence required before adoption: Android TalkBack/TTS parity, long-session memory tests and navigation accessibility tests.
- Completion state: COMPLETE — 418 logical lines read through EOF in contiguous chunks <=400; pinned SHA and 14121-byte size match; tests inspected, not run.

### 1874. `crates/horizon-narrator/src/speech.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `52f74f8f69902fd606ed78bcae53e900523cdb16`
- Size: 4306 bytes; 130 logical lines
- Coverage: 1–130 (EOF)
- Purpose: Platform speech trait, worker thread and command dispatch.
- Dependencies / cross-references: std mpsc channels, per-platform speech backends and thread spawn.
- Implementation / logic: Opens one platform speech backend on a dedicated thread, forwards Say/Clear commands and polls backend idle every 50 ms.
- Findings: Unbounded std mpsc command channel; thread spawn failure reported asynchronously as inactive; cfg targets omit Android backend.
- RustChunk relevance: Thread-isolated speech abstraction is useful but needs bounded queue and Android backend.
- Decision: adaptable; optional; platform-specific
- 1.20.1 implications: Android is not covered by Windows/macOS/Linux cfg selection; unsupported platform returns error.
- Follow-up checks: Implement Android JNI/TTS backend, bounded queue, explicit shutdown and completion/cancellation semantics.
- RustChunk implementation mapping: Optional RustChunk accessibility adapter.
- Evidence required before adoption: Android device tests, queue backpressure and thread lifecycle stress.
- Completion state: COMPLETE — 130 logical lines read through EOF; pinned SHA and 4306-byte size match.

### 1875. `crates/horizon-narrator/src/speech/linux.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2316653ee2b4d3b1f74dcca9f2c2b341be6ba19f`
- Size: 8293 bytes; 206 logical lines
- Coverage: 1–206 (EOF)
- Purpose: Linux Flite text-to-speech backend with dynamically loaded native libraries.
- Dependencies / cross-references: libloading, libflite.so, libflite_cmu_us_kal16.so, C ABI pointers, atomic batch counter and worker channel.
- Implementation / logic: Loads Flite symbols, registers voice, splits text into pieces, synthesizes/rescales audio and plays sequentially.
- Findings: Compiled only for target_os=linux, not target_os=android; native libraries may be unavailable; unbounded piece queue and batch counter can wrap; raw C ABI needs ABI/library validation.
- RustChunk relevance: Not usable as Android backend by cfg; C-ABI ownership pattern is platform-specific.
- Decision: unnecessary; platform-specific; optional
- 1.20.1 implications: Android requires a distinct platform adapter; Linux shared libraries cannot be assumed present in Android app sandbox.
- Follow-up checks: Do not force cfg(linux) on Android; validate FFI signatures, library lifetime, queue bounds and cancellation.
- RustChunk implementation mapping: No direct mapping; optional Linux desktop backend only.
- Evidence required before adoption: Linux ABI/runtime tests and separate Android TTS implementation/device validation.
- Completion state: COMPLETE — 206 logical lines read through EOF; pinned SHA and 8293-byte size match.


## Checkpoint 1876–1900

### 1876. `crates/horizon-narrator/src/speech/linux_split.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `77615043eb7f0d01f3fa11a77d345ed44e876f26`
- Size: 2793 bytes; 84 logical lines
- Coverage: 1–84 (EOF)
- Purpose: Pure helper splitting narrator text into Flite-sized speech units at punctuation.
- Dependencies / cross-references: Linux speech backend; Java String.isBlank whitespace semantics; no external dependencies.
- Implementation / logic: Splits punctuation runs while preserving leading spaces of nonblank pieces and implements Java whitespace classification.
- Findings: Only used by Linux backend; behavior differs from generic Rust whitespace for NBSP and U+0085; pure helper has no Android runtime role.
- RustChunk relevance: Small text segmentation algorithm can be reused only if Android TTS needs the same chunking.
- Decision: unnecessary; platform-specific helper
- 1.20.1 implications: Not part of Minecraft 1.20.1 gameplay; Android TTS should use its own sentence/chunk policy.
- Follow-up checks: Test Unicode whitespace and long multilingual messages if reused.
- RustChunk implementation mapping: Optional narrator adapter utility, not core launcher.
- Evidence required before adoption: Android speech tests for punctuation, CJK/emoji and chunk boundaries.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1877. `crates/horizon-narrator/src/speech/mac.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `260862e8ce43f01b593b2e81940efe70884c47ec`
- Size: 4518 bytes; 109 logical lines
- Coverage: 1–109 (EOF)
- Purpose: macOS AppKit NSSpeechSynthesizer backend with system voice/rate and a private message queue.
- Dependencies / cross-references: objc2, AppKit run loop, NSSpeechSynthesizer, VecDeque.
- Implementation / logic: Runs synthesizer on speech worker thread, pumps current run loop in idle, polls speaking state and starts next queued message.
- Findings: Queue is unbounded; interrupt stops active speech but leaves queued messages ahead of interrupting message; AppKit run-loop callback timing is polled rather than integrated as a delegate.
- RustChunk relevance: Platform lifecycle/queue reference only; not portable to Android.
- Decision: unnecessary; platform-specific
- 1.20.1 implications: Android must use Android TextToSpeech/JNI and lifecycle; AppKit is unavailable.
- Follow-up checks: Bound queue, define interrupt semantics and ensure UI/main-thread constraints for any macOS reuse.
- RustChunk implementation mapping: No RustChunk Android mapping; optional desktop speech backend.
- Evidence required before adoption: macOS thread/run-loop tests; Android TTS tests independently.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1878. `crates/horizon-narrator/src/speech/windows.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f40058ca0002698c70f4f0283e65e6ac0d7a93e8`
- Size: 2939 bytes; 84 logical lines
- Coverage: 1–84 (EOF)
- Purpose: Windows SAPI SpVoice backend for asynchronous speech.
- Dependencies / cross-references: windows crate SAPI and COM APIs, HSTRING, ManuallyDrop.
- Implementation / logic: Initializes COM MTA on worker thread, creates SpVoice, supports async plain-text speech, purge-before-speak and queue skip; releases voice before COM uninitialize.
- Findings: Volume is cast to SAPI integer without explicit finite/range validation; Windows-only COM implementation.
- RustChunk relevance: Lifecycle ordering and per-thread COM ownership are platform-specific references.
- Decision: unnecessary; platform-specific
- 1.20.1 implications: No Android support or Forge integration; SAPI cannot be used on Android.
- Follow-up checks: Validate volume centrally and test COM failure/teardown paths if desktop backend is retained.
- RustChunk implementation mapping: No Android implementation mapping.
- Evidence required before adoption: Windows COM apartment, failure, cancellation and volume boundary tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1879. `crates/horizon-optifine/Cargo.toml`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a15595b286f0cd9722aa1d77fa4953f855e290e0`
- Size: 673 bytes; 18 logical lines
- Coverage: 1–18 (EOF)
- Purpose: Manifest for OptiFine-format resource-pack parsing crate.
- Dependencies / cross-references: horizon-core, log, regex with std/unicode features; horizon-bench dev dependency.
- Implementation / logic: Declares Rust crate with parser modules for CEM, CIT, CTM, Fusion, paths, properties and matchers.
- Findings: Workspace Rust version and crate dependencies must be checked against RustChunk toolchain/Android build; no direct Forge bridge.
- RustChunk relevance: Strong optional reference for resource-pack feature parsing, but no runtime rendering integration by itself.
- Decision: adaptable; optional; version-specific
- 1.20.1 implications: Minecraft 1.20.1 resource pack semantics differ from later data-component mappings; ensure target-version compatibility and ARM64 compile.
- Follow-up checks: Audit license/dependency tree, MSRV, regex footprint and Android cross-compilation.
- RustChunk implementation mapping: Optional RustChunk resource-pack parser crate behind feature flag.
- Evidence required before adoption: Android ARM64 build, dependency size, resource-pack compatibility fixtures and renderer integration.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1880. `crates/horizon-optifine/examples/cem_expr_bench.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8d6f0cbf41d0dae118b5b8892d867d403fbb67f2`
- Size: 4505 bytes; 137 logical lines
- Coverage: 1–137 (EOF)
- Purpose: Standalone microbenchmark for compiled CEM animation expressions over two million entity frames.
- Dependencies / cross-references: horizon-optifine::cem::expr, horizon-bench CPU timer, Instant and black_box.
- Implementation / logic: Compiles a walk/idle/head animation expression set once, evaluates each over 2,000,000 simulated frames and reports wall/CPU time.
- Findings: Synthetic single-entity workload; benchmark was not run; results would not establish Android Mali or whole-game FPS.
- RustChunk relevance: Benchmark design can inform isolated expression evaluator profiling only.
- Decision: undetermined; benchmark/example only
- 1.20.1 implications: No direct 1.20.1 game or Android performance evidence.
- Follow-up checks: Run on target ARM64 device with realistic entity counts, thermal controls and full rendering load before making performance claims.
- RustChunk implementation mapping: Optional CEM evaluator benchmark harness.
- Evidence required before adoption: Repeatable Android ARM64 measurements and comparison against vanilla/Forge behavior.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1881. `crates/horizon-optifine/src/blink.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `681928d04dfcd3ae411a9836107002c11bcc5d59`
- Size: 9378 bytes; 245 logical lines
- Coverage: 1–245 (EOF)
- Purpose: ETF-compatible blinking texture discovery and per-entity frame timing.
- Dependencies / cross-references: RandomTextures, resource-root file walker, Properties and filesystem.
- Implementation / logic: Finds `_blink.png`/`_blink2.png`, optional blink properties, maps random variants and computes shut/half/open frame from tick and phase.
- Findings: Blink properties use read_to_string rather than Java-compatible Latin-1 decoder; invalid UTF-8 silently falls back to defaults; file discovery inherits walker symlink/count limitations.
- RustChunk relevance: Useful optional entity texture feature, not required for chunk renderer.
- Decision: adaptable; optional; resource-pack feature
- 1.20.1 implications: ETF/OptiFine entity texture behavior must be checked against 1.20.1 pack conventions and client atlas implementation.
- Follow-up checks: Use shared bounded properties reader with Java encoding behavior; test huge frequency, missing files, random variants and symlinked packs.
- RustChunk implementation mapping: Optional RustChunk entity texture feature layer.
- Evidence required before adoption: 1.20.1 resource-pack fixtures, atlas upload tests, Android file access and long-session memory checks.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1882. `crates/horizon-optifine/src/cem/curves.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2161f9906d0ac46ec46349ea00f0102a1a533d8d`
- Size: 4945 bytes; 158 logical lines
- Coverage: 1–158 (EOF)
- Purpose: Math curves/easing, interpolation and keyframe/spline helpers for CEM animations.
- Dependencies / cross-references: f32 math; expression evaluator calls ease, keyframe, Catmull-Rom, Hermite and Bezier functions.
- Implementation / logic: Implements easing families, bounce, lerp, keyframe/loop, Catmull-Rom, Hermite and quadratic/cubic Bezier.
- Findings: Public keyframe helper assumes non-empty frames and can underflow/index panic when called directly with an empty slice; expression parser currently enforces at least one frame.
- RustChunk relevance: Useful animation math reference; requires guards and parity checks.
- Decision: adaptable; optional; animation subsystem
- 1.20.1 implications: Not chunk/rendering logic; CEM support may be optional for 1.20.1 resource packs.
- Follow-up checks: Add empty-frame guard, NaN/out-of-range behavior tests and compare curves against EMF/OptiFine.
- RustChunk implementation mapping: Optional RustChunk CEM animation evaluator.
- Evidence required before adoption: Differential test vectors from 1.20.1 OptiFine/EMF and ARM64 numerical tolerance tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1883. `crates/horizon-optifine/src/cem/expr.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2fe9c48b5010b9ee4ef914c2953819c6beee95e6`
- Size: 35900 bytes; 997 logical lines
- Coverage: 1–400, 401–800, 801–997 (EOF)
- Purpose: Compiles CEM expression strings into a flat stack bytecode and evaluates per entity/frame.
- Dependencies / cross-references: CEM variable resolver slots, curves helpers, thread-local seeded/unseeded random state and bounded parser.
- Implementation / logic: Implements arithmetic/comparison, lazy if/boolean branches, math functions, random, keyframes/splines and bytecode stack-depth validation.
- Findings: Parser caps nesting 64 and nodes 1024; runtime evaluation is allocation-free; `nbt()`/unknown context semantics require caller mapping; random() is thread-local and launch-dependent.
- RustChunk relevance: Strong reusable architecture for animation expression VM, but CEM variable names and semantics must be reimplemented.
- Decision: adaptable; optional; core reference for CEM only
- 1.20.1 implications: No Forge 1.20.1 integration; expression inputs depend on entity/model transforms and vanilla timing semantics.
- Follow-up checks: Compare supported functions/variables with OptiFine 1.20.1/EMF; validate slots, stack bounds, deterministic random and invalid-expression handling.
- RustChunk implementation mapping: Optional RustChunk CEM VM isolated from render loop; compile once, evaluate from per-entity state.
- Evidence required before adoption: Differential expression corpus, fuzz parser, stack invariants, per-frame ARM64 profiling and game-version parity.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1884. `crates/horizon-optifine/src/cem/files.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f4fe5f63798293628ef13c716470f0b41d30187a`
- Size: 5348 bytes; 137 logical lines
- Coverage: 1–137 (EOF)
- Purpose: Discovers and loads EMF/OptiFine .jem custom entity models and resolves atlas texture names.
- Dependencies / cross-references: Filesystem, BTreeMap, model parser, MAX_BYTES cap and resource-root path.
- Implementation / logic: Prioritizes EMF folder over OptiFine and flat file over nested; reads .jem/.jpm text under MAX_BYTES and logs parser warnings.
- Findings: CEM discovery is limited to assets/minecraft namespace; metadata-then-read has a file-growth race; local pack symlinks are followed by path joins.
- RustChunk relevance: Useful CEM loading precedence and bounded file-read pattern.
- Decision: adaptable; optional; resource-pack feature
- 1.20.1 implications: CEM models are not Minecraft chunk geometry; preserve 1.20.1 entity/model compatibility.
- Follow-up checks: Use streaming capped reads, canonical-root checks and verify modded namespace support.
- RustChunk implementation mapping: Optional RustChunk entity model loader.
- Evidence required before adoption: CEM fixtures, symlink escape tests, malformed/oversized JSON and atlas registration tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1885. `crates/horizon-optifine/src/cem/json.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `8b0bb2150991dd47db32f17f8a58784e9a3df20a`
- Size: 11637 bytes; 341 logical lines
- Coverage: 1–341 (EOF)
- Purpose: Lenient Gson-like JSON parser for CEM .jem/.jpm files preserving object key order.
- Dependencies / cross-references: Custom byte parser, UTF-8 lossy conversion, depth limit 128.
- Implementation / logic: Accepts BOM, comments, trailing commas, single quotes, bare keys, `=`/`=>`, surrogate pairs and ordered duplicate keys.
- Findings: No intrinsic input-byte/node-count cap; callers rely on MAX_BYTES. Object lookup returns last duplicate key while iteration preserves all duplicates; parser recursion bounded at 128.
- RustChunk relevance: Parser may be reused for lenient CEM only after resource limits and duplicate-key policy are explicit.
- Decision: adaptable; optional; parser reference
- 1.20.1 implications: JSON semantics should match 1.20.1 OptiFine/EMF, not general mod config parsing.
- Follow-up checks: Add byte/token/array/object limits, duplicate-key compatibility tests, malformed UTF-8 and surrogate fuzzing.
- RustChunk implementation mapping: Optional CEM-specific parser, not shared generic settings parser.
- Evidence required before adoption: Fuzzing, depth/size limits, differential Gson tests and Android memory stress.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1886. `crates/horizon-optifine/src/cem/mod.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `e569f11e6df30f3b7a9a52ce18ef464f300d10d1`
- Size: 371 bytes; 11 logical lines
- Coverage: 1–11 (EOF)
- Purpose: CEM module façade exporting expression, curves, JSON, files and model APIs.
- Dependencies / cross-references: Sibling cem modules.
- Implementation / logic: Declares modules and re-exports selected public types/functions.
- Findings: Small module organization file; API is specific to Horizon CEM parser.
- RustChunk relevance: Module organization reference only.
- Decision: adaptable; module-structure reference
- 1.20.1 implications: No direct game behavior.
- Follow-up checks: Keep CEM behind optional feature and avoid coupling with core chunk pipeline.
- RustChunk implementation mapping: RustChunk optional CEM module structure.
- Evidence required before adoption: Static API and feature-matrix review.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1887. `crates/horizon-optifine/src/cem/model.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `07bba1f08e33f117b30858b09d598e18b2aee458`
- Size: 50118 bytes; 1234 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1234 (EOF)
- Purpose: Parses CEM entity model geometry, bones, boxes, sprites, animations and model inheritance.
- Dependencies / cross-references: CEM JSON parser, expression VM, atlas texture paths, model bone/quad types and geometry limits.
- Implementation / logic: Builds bones and parts, applies baseId inheritance, parses boxes/sprites, transforms, animation expressions and bakes quads with bounded node/animation budgets.
- Findings: Target model schema and transforms need OptiFine/EMF 1.20.1 parity; geometry flips and face normals need differential verification; some invalid numeric vectors default to zero rather than rejecting.
- RustChunk relevance: Substantial reusable CEM architecture, but not Minecraft chunk mesh or world renderer.
- Decision: rewrite required; optional; version-specific
- 1.20.1 implications: CEM entities are separate from chunk meshing; don't conflate model geometry with terrain chunks or Forge model loader.
- Follow-up checks: Verify UV/normal winding for invertAxis/box mirror, negative/zero dimensions, baseId resolution, animation cost limits and 1.20.1 transform semantics.
- RustChunk implementation mapping: Optional RustChunk CEM entity model renderer, separate from chunk pipeline.
- Evidence required before adoption: Golden model fixtures from 1.20.1 OptiFine/EMF, face normals, UVs, matrix transforms, parser fuzz and Mali GPU tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1888. `crates/horizon-optifine/src/cit.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `89718087fbee1fecaa1e20b8d5718677c1214a4a`
- Size: 52741 bytes; 1363 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1363 (EOF)
- Purpose: Parses Custom Item Textures rules, item/armor/elytra/enchantment matches, components, assets and glint options.
- Dependencies / cross-references: Properties, matchers, path resolver, legacy NBT mapping and OpenGL blend-factor enum.
- Implementation / logic: Builds rules from CIT properties, resolves texture/model assets, maps legacy NBT tests to component paths and orders rules by weight/path.
- Findings: File explicitly follows 1.21 component rules and CIT Resewn Continuation; several legacy tag mappings target post-1.20.5/1.21.5 components and are not valid 1.20.1 NBT semantics.
- RustChunk relevance: Potential optional resource-pack feature, but must be version-specialized.
- Decision: rewrite required; optional; version-specific
- 1.20.1 implications: 1.20.1 still uses legacy item NBT and Forge ItemStack behavior; remove later component assumptions and map against exact 1.20.1 NBT.
- Follow-up checks: Build 1.20.1-specific legacy NBT matcher, validate damage/stack/enchantment semantics and OpenGL blend compatibility.
- RustChunk implementation mapping: Optional RustChunk CIT layer only after version-specific stack adapter.
- Evidence required before adoption: Differential CIT Resewn/OptiFine 1.20.1 corpus, item NBT fixtures, renderer blend tests and Android resource pack tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1889. `crates/horizon-optifine/src/ctm.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1b4c78d828f9f4a9766181918ac1bcd038d578ef`
- Size: 33616 bytes; 879 logical lines
- Coverage: 1–400, 401–800, 801–879 (EOF)
- Purpose: Parses OptiFine/Continuity connected-texture properties and produces ordered CTM rules.
- Dependencies / cross-references: Properties, matchers, resource path helpers, filesystem and CTM neighbour-mask module.
- Implementation / logic: Supports CTM, compact, horizontal/vertical, random/repeat/fixed and overlay methods, block/tile matching, faces, biome/height/name predicates and tile caps.
- Findings: `load` reads each properties file fully without a per-file byte cap; discovery has no global file-count cap and follows symlinks; some unknown option values silently default.
- RustChunk relevance: High relevance to resource-pack visual compatibility, but requires renderer/world neighbour integration.
- Decision: adaptable; optional; rendering feature
- 1.20.1 implications: 1.20.1 block states, face directions, biomes and model sprite atlas must match Forge/Minecraft exactly.
- Follow-up checks: Bound property reads and number of files; validate tile weights/size and compare all CTM templates with OptiFine 1.20.1.
- RustChunk implementation mapping: Optional resource-pack CTM rules feeding a renderer-side neighbour query.
- Evidence required before adoption: Template image parity, world block state tests, malformed/oversized pack tests, ARM64 and Mali performance.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1890. `crates/horizon-optifine/src/ctm/connect.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `66ac83a8a1a6925381e8a98277dda1de8f385b4f`
- Size: 22969 bytes; 657 logical lines
- Coverage: 1–400, 401–657 (EOF)
- Purpose: Computes connected-texture neighbour masks and maps masks to CTM/Fusion tile indices.
- Dependencies / cross-references: Face axes, neighbour callbacks, 47-tile templates, overlay/compact tables and deterministic hash.
- Implementation / logic: Computes texture axes from quad positions/UVs, neighbour offsets, symmetry, repeat indices and weighted deterministic random tile choices.
- Findings: Public repeat_index assumes sensible width/height; extreme direct arguments can overflow intermediate i64 product; callers' tile caps currently constrain normal paths; all tile tables require golden-template parity.
- RustChunk relevance: Reusable renderer-side CTM/Fusion algorithm; among the more directly useful visual subsystems.
- Decision: adaptable; optional; renderer-specific
- 1.20.1 implications: Must use exact 1.20.1 quad UV conventions, block face orientation and world neighbour semantics.
- Follow-up checks: Guard public dimensions, validate normals/UVs for all rotations, compare all 256 masks and run on Mali GPU.
- RustChunk implementation mapping: RustChunk optional connected-texture renderer adapter.
- Evidence required before adoption: Exhaustive mask tests, Forge baked-quad fixtures, orientation parity and Android GPU profiling.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1891. `crates/horizon-optifine/src/emissive.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f49e42c9f482d35ed9145ef484240837bae6ca86`
- Size: 11679 bytes; 302 logical lines
- Coverage: 1–302 (EOF)
- Purpose: Pairs emissive overlay textures with entity atlas textures, random variants and blink frames.
- Dependencies / cross-references: Resource walker, RandomTextures, Blinks, Properties and filesystem.
- Implementation / logic: Reads suffix configuration from first available properties file, finds `_e` overlays, maps atlas names and tracks random-variant overlays.
- Findings: Properties suffix files are size-capped, but overlay discovery inherits walker count/symlink constraints; only entity atlas is scanned here, block/item atlas needs separate handling.
- RustChunk relevance: Useful optional emissive texture pairing concept.
- Decision: adaptable; optional; rendering feature
- 1.20.1 implications: Needs separate block/item/entity atlas integration and 1.20.1 renderer lightmap/full-bright behavior.
- Follow-up checks: Bound file enumeration, validate suffix/path handling and verify overlay draw order/lightmap on target renderer.
- RustChunk implementation mapping: Optional emissive overlay pass in resource-pack renderer.
- Evidence required before adoption: 1.20.1 atlas fixtures, lightmap/shader compatibility, symlink/huge-pack tests and Mali overdraw profiling.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1892. `crates/horizon-optifine/src/fusion.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c3769c661c0ade0e0e587c97ce631dfdeb10f20a`
- Size: 2309 bytes; 65 logical lines
- Coverage: 1–65 (EOF)
- Purpose: Fusion resource-pack module façade and shared typed JSON field helpers.
- Dependencies / cross-references: CEM lenient JSON parser, Fusion layout/model/predicate/texture modules.
- Implementation / logic: Exports metadata/model/predicate/layout types and parses boolean/string/integer fields.
- Findings: Fusion is a separate mod's format, not native vanilla or Forge connected-texture format; numeric conversion should retain explicit range checks.
- RustChunk relevance: Optional compatibility target only if RustChunk chooses Fusion pack support.
- Decision: unnecessary; optional; third-party format
- 1.20.1 implications: No Forge core or chunk renderer requirement; implement only behind optional pack-format support.
- Follow-up checks: Confirm current Fusion specification/version and test integer boundary conversion.
- RustChunk implementation mapping: Optional Fusion parser module only.
- Evidence required before adoption: Fusion pack corpus, schema/version tests and renderer integration.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1893. `crates/horizon-optifine/src/fusion/layout.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d21d63a1353cd1c5cae63f597cec2a6a15d782b9`
- Size: 14883 bytes; 400 logical lines
- Coverage: 1–400 (EOF)
- Purpose: Maps Fusion connected-texture neighbour masks to tile layouts and overlay tile sequences.
- Dependencies / cross-references: CTM connect masks, fixed full/simple templates, compact and overlay layout algorithms.
- Implementation / logic: Supports pieced/simple/full/vertical/horizontal/compact/overlay layouts and enumerates overlay pieces by joined sides/corners.
- Findings: Layout tables are derived from public wiki/example template images; need versioned fixture verification; overlay sequence order affects alpha compositing.
- RustChunk relevance: Potential reusable layout math if Fusion compatibility is explicitly required.
- Decision: unnecessary; optional; third-party format
- 1.20.1 implications: Not required for vanilla/Forge 1.20.1 base client; keep separate from OptiFine CTM semantics.
- Follow-up checks: Compare every mask/layout against exact Fusion examples and verify overlay order and blending.
- RustChunk implementation mapping: Optional Fusion renderer layout adapter.
- Evidence required before adoption: Exhaustive 256-mask fixture, image golden tests and Android GPU overlay validation.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1894. `crates/horizon-optifine/src/fusion/model.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a3c888cc3c2b17057446ab994e040d1e0ccc9803`
- Size: 8971 bytes; 245 logical lines
- Coverage: 1–245 (EOF)
- Purpose: Parses Fusion block model types and per-texture connection references with inheritance.
- Dependencies / cross-references: Lenient CEM JSON, Fusion predicates, connection model, keyed texture refs.
- Implementation / logic: Supports base/connecting/composite type identification, all-face/keyed predicates, #key references, default fallback and parent inheritance.
- Findings: Reference resolution is capped at 32 hops; malformed/empty references fall back to default or no predicate; this is Fusion model-loader logic only.
- RustChunk relevance: Optional third-party format reference.
- Decision: unnecessary; optional; third-party format
- 1.20.1 implications: No core Forge requirement; model loader integration and inheritance must match selected Fusion version.
- Follow-up checks: Test reference chains at boundary, dangling refs, duplicate keys and parent/child overrides.
- RustChunk implementation mapping: Optional Fusion model adapter.
- Evidence required before adoption: Fusion model fixture corpus and model inheritance/render integration tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1895. `crates/horizon-optifine/src/fusion/predicate.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `36b44bc1d893d5ea757e204d3cb07cf799c3f7db`
- Size: 13342 bytes; 350 logical lines
- Coverage: 1–350 (EOF)
- Purpose: Parses Fusion connection predicates and evaluates per-neighbour world queries.
- Dependencies / cross-references: BlockMatch parser, JSON, CTM mask directions and caller-provided world query callbacks.
- Implementation / logic: Supports same block/state, match block/state, in-front, face-visible, direction, and/or/not/constant predicates.
- Findings: Nested predicates rely on JSON parser depth cap rather than a dedicated predicate-node budget; callback semantics must align with world block/face visibility.
- RustChunk relevance: Predicate AST/query abstraction is reusable for optional connected textures.
- Decision: adaptable; optional; renderer-specific
- 1.20.1 implications: Forge 1.20.1 block state and face-visibility checks must be supplied by the client world adapter.
- Follow-up checks: Add explicit predicate node cap and test all directions/face occlusion against vanilla 1.20.1.
- RustChunk implementation mapping: Optional renderer neighbour-query adapter.
- Evidence required before adoption: World-query correctness, malformed/deep JSON fuzzing and Android performance tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1896. `crates/horizon-optifine/src/fusion/texture.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `0be229bc42f3d735febf87effa6f9a0eda122169`
- Size: 13948 bytes; 418 logical lines
- Coverage: 1–400, 401–418 (EOF)
- Purpose: Parses Fusion texture .mcmeta metadata for base, connecting, random, continuous and scrolling textures.
- Dependencies / cross-references: Lenient JSON parser, Fusion layout/predicate, numeric/enum helpers.
- Implementation / logic: Validates layout, connection predicates, random grids/seeds, per-tile animation and scrolling frame parameters.
- Findings: This is a mod-specific .mcmeta extension; dimensions/seed semantics and animation timing must match exact Fusion version; recursive sub_texture depth relies on JSON parser limit.
- RustChunk relevance: Optional third-party resource-pack format.
- Decision: unnecessary; optional; third-party format
- 1.20.1 implications: Not vanilla 1.20.1 metadata; only parse when Fusion compatibility is intentionally implemented.
- Follow-up checks: Check multiplication/frame dimensions against actual image sizes and compare animation behavior to Fusion reference.
- RustChunk implementation mapping: Optional Fusion texture metadata adapter.
- Evidence required before adoption: Image dimension, animation frame, seed determinism and Fusion pack corpus tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1897. `crates/horizon-optifine/src/lib.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c3084d88a5fe4e623b024e73b92e55a7fa64ff93`
- Size: 1563 bytes; 31 logical lines
- Coverage: 1–31 (EOF)
- Purpose: OptiFine-format resource-pack crate façade and public parser exports.
- Dependencies / cross-references: Sibling blink/CEM/CIT/CTM/emissive/Fusion/matcher/path/property modules.
- Implementation / logic: Exports optional resource-pack parser features without direct game types.
- Findings: Broad public surface may pull in features unnecessary for RustChunk and increase binary/dependency footprint.
- RustChunk relevance: Module organization reference only.
- Decision: adaptable; optional; architecture reference
- 1.20.1 implications: No direct 1.20.1 runtime; choose feature-gated subsets to reduce Android footprint.
- Follow-up checks: Measure dependency size and isolate optional features in Cargo features.
- RustChunk implementation mapping: Optional RustChunk resource-pack module façade.
- Evidence required before adoption: Android binary size and compile-feature matrix.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1898. `crates/horizon-optifine/src/matchers.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `13e5bd4a85e4cabda8a09370aa09b4a2939b02cc`
- Size: 10936 bytes; 307 logical lines
- Coverage: 1–307 (EOF)
- Purpose: Implements OptiFine-style name/glob/regex, integer range, ID and block-state matchers.
- Dependencies / cross-references: regex crate, block ID/state adapters and property parsers.
- Implementation / logic: Provides O(pattern×name) glob, anchored regex, ranges, inclusion/exclusion IDs and block state lists.
- Findings: Range parser accepts reversed bounds that simply never match; empty/invalid namespace tokens can be retained; regex crate avoids catastrophic backtracking but patterns still need input limits.
- RustChunk relevance: Useful resource-pack matcher primitives with strict input normalization.
- Decision: adaptable; optional; parser utility
- 1.20.1 implications: IDs and state values must match 1.20.1 registry identifiers and property sets exactly.
- Follow-up checks: Reject malformed/empty IDs and reversed ranges; fuzz Unicode glob, regex, numeric bounds and block state lists.
- RustChunk implementation mapping: Optional shared matcher crate for CIT/CTM.
- Evidence required before adoption: Differential matcher corpus, regex/input-size limits and 1.20.1 registry integration.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1899. `crates/horizon-optifine/src/paths.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a351d85089e9b7d0b1eee3bfe7393236c134c4dd`
- Size: 8576 bytes; 242 logical lines
- Coverage: 1–242 (EOF)
- Purpose: Resolves resource paths and walks OptiFine/MCPatcher feature folders under an overlay resource root.
- Dependencies / cross-references: Filesystem canonicalization, HashSet, prepared resource-root files and path validators.
- Implementation / logic: Rejects dot/dotdot/empty/backslash segments, follows directory symlinks once by canonical path, sorts discovered property/resource files.
- Findings: File symlinks are not canonicalized against the resource root before returned paths are opened; walker has no file-count cap; symlink targets can expose files outside intended pack root.
- RustChunk relevance: Path normalization is valuable but needs stronger containment and enumeration limits.
- Decision: rewrite required; security-sensitive; reference
- 1.20.1 implications: Android SAF/scoped-storage and pack overlay symlink behavior differ; do not rely on desktop filesystem link semantics.
- Follow-up checks: Canonicalize every file target and prove it remains under permitted roots; cap depth, files and aggregate bytes.
- RustChunk implementation mapping: RustChunk resource-pack filesystem adapter.
- Evidence required before adoption: Symlink escape, huge directory, path encoding and Android storage tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.

### 1900. `crates/horizon-optifine/src/properties.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `3b4b2c7ab242d49995b342a98e488f61c1a15c73`
- Size: 7743 bytes; 210 logical lines
- Coverage: 1–210 (EOF)
- Purpose: Parses Java .properties syntax including comments, continuations, escapes and duplicate keys.
- Dependencies / cross-references: UTF-8 text input, HashMap index, custom byte/character scanning and Java-style Unicode escape handling.
- Implementation / logic: Supports `=`, `:`, whitespace separators, continuations, \\t/\\n/\\uXXXX escapes and equals-first mode for CIT keys.
- Findings: `split` scans UTF-8 bytes and advances two bytes after backslash; an escaped multibyte character can leave the slice index inside a UTF-8 code point and panic. Invalid/incomplete Unicode escapes are silently dropped rather than Java-compatible failure behavior.
- RustChunk relevance: Important shared parser for CIT/CTM/ETF; requires Unicode safety before reuse.
- Decision: rewrite required; parser correctness/security-sensitive
- 1.20.1 implications: 1.20.1 packs may contain Java properties encoding/escape behavior; preserve exact Java Properties.load semantics and Latin-1 handling.
- Follow-up checks: Make scanning char-boundary safe, test escaped multibyte UTF-8, malformed Unicode/surrogate pairs, CRLF/continuations and Latin-1 decoding.
- RustChunk implementation mapping: RustChunk resource-pack properties parser shared by optional pack features.
- Evidence required before adoption: Fuzz UTF-8 boundary cases, Java reference vectors, parser size limits and Android file encoding tests.
- Completion state: COMPLETE — full source file read and SHA/size verified against pinned tree; tests not run.
