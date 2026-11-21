# RustChunk Starling Audit Ledger — Entries 1801–2000

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range: 1801–2000 (200 unique source files; one shared Ledger for this stage).
- Current progress: entries 1801–2000 complete (200/200); sections 1801–1850, 1851–1900, 1901–1950 and 1951–2000 are closed (50/50 each). Global verification of entries 1–2000 passed on 2026-11-21; final backup created at `backup/audit-2000-final-2026-11-21`.
- Reporting rule: section 1951–2000 and stage 1801–2000 are closed; global verification passed. STOP at 2000; do not inspect entry 2001+ until separately authorized.
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

### 1901. `crates/horizon-optifine/src/random.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `fb356960e2dbc8da007da0dae9ab0facfbfd1e4a`
- Size: 30216 bytes; 802 logical lines
- Coverage: 1–400, 401–800, 801–802 (EOF)
- Purpose: Parses OptiFine/ETF random-entity properties into selectors, weighted variants and condition rules.
- Dependencies / cross-references: Java Properties syntax, entity name/path mapping, OptiFine random-entity rule fields and entity-condition parser.
- Implementation / logic: Reads rule keys, expands variants and weighted alternatives, parses biome/height/health/name/profession/age/team/seed conditions and reports malformed or unsupported entries.
- Findings: Rule interpretation is tied to OptiFine/ETF behavior and several selectors need parity checks. Dynamic entity state requires reevaluation; properties parsing is separate and has its own encoding/escape semantics.
- RustChunk relevance: Potential optional resource-pack feature for entity variation; unrelated to base Minecraft networking or chunk loading.
- Decision: reference; optional resource-pack feature; version-specific
- 1.20.1 implications: Rebuild against the exact 1.20.1 OptiFine/ETF properties contract and Mojmap entity fields; do not assume newer condition names or seed selection match.
- Follow-up checks: Compare against 1.20.1 OptiFine/ETF fixtures for each condition, weighting, inheritance, malformed input and per-frame reevaluation.
- RustChunk implementation mapping: RustChunk optional random-entity texture rules module, behind resource-pack feature gates.
- Evidence required before adoption: Differential fixtures from 1.20.1 OptiFine/ETF, parser fuzz tests and Android resource-pack performance tests.
- Completion state: COMPLETE — 802 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 30216-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1902. `crates/horizon-optifine/src/random_textures.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `7831796f7d3a155bc164e78e748383fb7794510f`
- Size: 23051 bytes; 579 logical lines
- Coverage: 1–400, 401–579 (EOF)
- Purpose: Discovers and organizes random entity texture variants from OptiFine/ETF/MCPatcher resource-pack folders.
- Dependencies / cross-references: Resource-pack filesystem roots, random.properties parsed rules, entity texture atlas names, variant inheritance and file lookup.
- Implementation / logic: Scans candidate folder layouts in priority order, collects variant texture files, resolves rule inheritance and produces deterministic atlas-facing variant names.
- Findings: Folder precedence and naming are format-specific; scans rely on path containment and file filtering. Dynamic rules and texture atlas limits need explicit bounds.
- RustChunk relevance: Optional resource-pack rendering feature; no core client or Forge protocol value.
- Decision: reference; optional resource-pack feature; version-specific
- 1.20.1 implications: Implement only after 1.20.1 texture atlas and entity model integration exists; verify Android asset I/O and memory budgets.
- Follow-up checks: Test precedence among ETF/OptiFine/MCPatcher, duplicate variant IDs, path traversal/symlinks, missing textures, atlas size and reload invalidation.
- RustChunk implementation mapping: RustChunk resource-pack scanner feeding an optional entity texture resolver/atlas builder.
- Evidence required before adoption: Known-good 1.20.1 packs, deterministic atlas fixtures, filesystem security tests and Mali-G52 memory/performance measurements.
- Completion state: COMPLETE — 579 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 23051-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1903. `crates/horizon-port-pack/Cargo.toml`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `53758fc949f376d5ee07da39e5303789cd0d6e44`
- Size: 849 bytes; 22 logical lines
- Coverage: 1–22 (EOF)
- Purpose: Cargo manifest for the Horizon modpack porting command-line tool.
- Dependencies / cross-references: zip, serde/serde_json, TOML, HTTP client and workspace Horizon crates.
- Implementation / logic: Declares a standalone migration utility that imports packs, fetches jars, inspects metadata and carries content into Horizon folders.
- Findings: This is a tooling crate with a separate target/version contract, not a runtime game dependency.
- RustChunk relevance: Useful only as a reference for a future RustChunk pack-inspection/migration utility.
- Decision: unnecessary; tooling-only; version-specific
- 1.20.1 implications: Do not add this manifest or dependency graph to the Android client; its output targets Horizon's newer content API.
- Follow-up checks: Audit only if RustChunk creates a distinct desktop/server-side pack conversion tool; review licenses and dependency MSRV then.
- RustChunk implementation mapping: No player-runtime mapping; optional separate CLI/tooling crate.
- Evidence required before adoption: Explicit product requirement, isolated build and dependency/license review.
- Completion state: COMPLETE — 22 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 849-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1904. `crates/horizon-port-pack/src/carry.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1965e0adab47b256e23cf4ad748722c26993160d`
- Size: 86999 bytes; 2120 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1600, 1601–2000, 2001–2120 (EOF)
- Purpose: Carries content assets/data from a NeoForge mod JAR into Horizon's folder-content mod format.
- Dependencies / cross-references: ZIP archive entries, mod metadata, JSON/TOML, block/item facts, worldgen processors, recipe/loot conversion and Horizon API 0.2 declarations.
- Implementation / logic: Reads a jar, filters namespaces, converts blockstates/models/items/recipes/loot/worldgen, creates declarations and a carry report, and moves recipes for undeclared carried items outside.
- Findings: This migration explicitly targets mod content from 1.21.1 to Horizon 26.3. JAR entries can be read fully into memory without a clear per-entry/uncompressed aggregate ceiling; generated block/item templates can approximate rather than reproduce mod Java behavior.
- RustChunk relevance: Architecture reference for a future content-only pack converter; does not load Forge mods or enable joining Aternos.
- Decision: rewrite required; tooling-only; version-specific; security-sensitive
- 1.20.1 implications: Do not reuse output schemas for Forge 1.20.1. A target-specific converter must model 1.20.1 registries, NBT, recipes, loot and Forge hooks.
- Follow-up checks: Add ZIP bomb/entry size limits, namespace ownership tests, safe extraction/path validation, full target-version parity and deterministic output tests.
- RustChunk implementation mapping: Potential separate RustChunk pack-audit/conversion utility; never include in the game runtime load path.
- Evidence required before adoption: 1.20.1 Forge corpus, ZIP adversarial tests, exact registry/loot/recipe differential tests and resource-limit validation.
- Completion state: COMPLETE — 2120 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 86999-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1905. `crates/horizon-port-pack/src/check.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `45fb20f1ee1f2955721330dcd5c88db448c83b83`
- Size: 14550 bytes; 415 logical lines
- Coverage: 1–400, 401–415 (EOF)
- Purpose: Classifies one mod JAR as content-only Works/Partly/Doesn't-run before carry-over.
- Dependencies / cross-references: JAR metadata/parser, class bytecode census, content carry engine and JSON check report.
- Implementation / logic: Reads jar names/classes, detects loader and legacy-layout incompatibility, performs content carry, summarizes translated assets and skipped code features.
- Findings: The verdict measures content carry capability, not actual launch/runtime behavior. Code is only censused; it is never loaded or executed.
- RustChunk relevance: Useful preflight-report pattern for modpack analysis; not a compatibility guarantee for Forge 1.20.1.
- Decision: adaptable; tooling reference; version-specific
- 1.20.1 implications: A 1.20.1 checker needs Forge 47.x mappings, old recipe/loot formats, and real runtime/API compatibility criteria.
- Follow-up checks: Validate against a representative Forge 1.20.1 mod corpus; distinguish metadata-only, content-only, partial code and runnable mods.
- RustChunk implementation mapping: RustChunk offline mod inspection/report module, separate from game loading.
- Evidence required before adoption: Known modpack fixtures, false-positive/negative compatibility matrix and actual Forge launch results (not obtained here).
- Completion state: COMPLETE — 415 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 14550-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1906. `crates/horizon-port-pack/src/classes.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `d39c72e1aba47f3c8ee0f44241c7796f6df1b8bd`
- Size: 22845 bytes; 642 logical lines
- Coverage: 1–400, 401–642 (EOF)
- Purpose: Parses Java class files and infers mod code features from inheritance, interfaces, method names and constant-pool references.
- Dependencies / cross-references: Class-file constant pool, Java descriptors, Mojang package names, Forge/Fabric event names and feature taxonomy.
- Implementation / logic: Reads class headers/pool/method declarations, follows in-jar superclass chains, classifies behavior bases and scans references for events, commands and Mixins.
- Findings: The package/method map assumes current Mojang names and treats obfuscated/SRG methods as unread. It does not execute or decompile bytecode; modified UTF-8 is decoded lossily.
- RustChunk relevance: Useful inspection approach for mod compatibility reporting, not a loader or source-code port.
- Decision: adaptable; tooling reference; mapping-sensitive
- 1.20.1 implications: Forge 1.20.1 runtime mappings differ by distribution/build; classification must use exact Forge 47.x mappings and Java 17 class-file constraints.
- Follow-up checks: Test Java 17 class-file versions, modified UTF-8, constant-pool edge cases, inheritance depth, Forge SRG/Mojmap mappings and hidden behavior via interfaces/calls.
- RustChunk implementation mapping: RustChunk offline class-file census for diagnostics only.
- Evidence required before adoption: Differential class fixtures built under Forge 47.x, adversarial malformed class corpus and comparison against actual mod behavior.
- Completion state: COMPLETE — 642 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 22845-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1907. `crates/horizon-port-pack/src/classify.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `7ebb65644b2fb60da6e0e5983d3b1f22a40c3303`
- Size: 7670 bytes; 233 logical lines
- Coverage: 1–233 (EOF)
- Purpose: Classifies mods by reviewed ATM10 rows or heuristic loader side/category/content facts and assigns dependency layers.
- Dependencies / cross-references: Embedded ATM10 CSV, Modrinth categories, side support flags, JAR class/content census and required dependency graph.
- Implementation / logic: Normalizes IDs/names, checks known table first, applies side/category heuristics and calculates layer numbers for required in-pack dependencies.
- Findings: The curated table is ATM10-specific. Cycles are all placed at a final shared layer rather than topologically resolved; heuristic confidence must remain visible.
- RustChunk relevance: Reference for a mod inventory/reporting tool only.
- Decision: adaptable; tooling reference; pack-specific
- 1.20.1 implications: Use Forge 1.20.1 mod metadata and a corpus specific to the target modpack; do not rely on ATM10's reviewed labels.
- Follow-up checks: Test normalization collisions, multiple mods sharing a display name, side/category precedence, missing dependencies, duplicate keys and dependency cycles.
- RustChunk implementation mapping: RustChunk mod inventory classifier outside runtime.
- Evidence required before adoption: Representative 1.20.1 modpack labels reviewed by hand and a graph validation suite.
- Completion state: COMPLETE — 233 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 7670-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1908. `crates/horizon-port-pack/src/fetch.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `17a99ef26dab0347ad35f73bdcd5c4446000f5f8`
- Size: 18002 bytes; 494 logical lines
- Coverage: 1–400, 401–494 (EOF)
- Purpose: Fetches required mod JARs from pack links/Modrinth or a user's local instance and verifies SHA-1 when available.
- Dependencies / cross-references: Modrinth cache, pack file URLs, local JAR index, proxy agent, SHA-1 and temporary-file publication.
- Implementation / logic: Chooses local/present/downloaded/NeedsKey outcomes, downloads into a temporary file with a 512MiB limit and publishes after hash verification.
- Findings: Local instance scanning reads each jar fully without a per-file bound. A pack-provided URL can be accepted even without SHA-1, and initial-host allowlisting is not visibly rechecked after redirects.
- RustChunk relevance: Potential utility for an isolated pack manager; not needed for the running Minecraft network client.
- Decision: adaptable; tooling reference; security-sensitive
- 1.20.1 implications: Keep download/catalog logic separate from the 1.20.1 game socket. Verify hashes for every source and constrain redirects/hosts.
- Follow-up checks: Require hashes, enforce local and aggregate byte limits, validate redirect destinations, bound API responses, sanitize filenames and test concurrent publication.
- RustChunk implementation mapping: Separate RustChunk pack acquisition service with strict download policy.
- Evidence required before adoption: Malicious redirect tests, ZIP/JAR size tests, hash mismatch/interruption cases and Android storage permission validation.
- Completion state: COMPLETE — 494 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 18002-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1909. `crates/horizon-port-pack/src/import.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `c9381ae7c94dc176dc55416ec35d82d53cfe8c68`
- Size: 18031 bytes; 484 logical lines
- Coverage: 1–400, 401–484 (EOF)
- Purpose: Imports Modrinth/CurseForge packs, Crash Assistant modlists and Prism/MultiMC instances into a unified Pack manifest.
- Dependencies / cross-references: ZIP JSON/TOML manifests, mmc-pack metadata, modlist.json, filesystem game-root detection and SHA-1.
- Implementation / logic: Parses loader/Minecraft version, jar paths, download links, mod IDs and override counts for each supported pack format.
- Findings: ZIP manifest reads and recursive instance file counting have no clear decompressed-size/depth/symlink limits; legacy dependency/metadata fields are intentionally incomplete.
- RustChunk relevance: Useful format parser for a future separate pack importer.
- Decision: adaptable; tooling reference; security-sensitive
- 1.20.1 implications: A RustChunk 1.20.1 importer must resolve Forge 47.x and preserve legacy config/KubeJS/datapack semantics; no Horizon pack manifest.
- Follow-up checks: Bound archive entry reads and directory recursion, reject symlinks/path traversal, validate duplicate paths and add fixtures for actual target launchers.
- RustChunk implementation mapping: Separate pack import adapter; output should be RustChunk-owned metadata only.
- Evidence required before adoption: Real 1.20.1 CurseForge/Modrinth/Prism exports and adversarial archive/instance tests.
- Completion state: COMPLETE — 484 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 18031-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1910. `crates/horizon-port-pack/src/inventory.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `57c043ae322f4499a296cfbe2da62843824f71d5`
- Size: 8212 bytes; 229 logical lines
- Coverage: 1–229 (EOF)
- Purpose: Builds the per-file mod inventory from pack metadata, jar facts and Modrinth dependency metadata.
- Dependencies / cross-references: PackFile, jar parser/class census, Modrinth cache, known classifier and dependency layer graph.
- Implementation / logic: Chooses stable key/name/version/license/source, resolves required dependencies and sorts records by dependency layer.
- Findings: If jar metadata dependencies are non-empty they replace Modrinth required dependencies rather than being merged; duplicate keys can collide in dependency maps/layer graph.
- RustChunk relevance: Good report model for a pack-management tool, not an in-game subsystem.
- Decision: adaptable; tooling reference; dependency-sensitive
- 1.20.1 implications: Forge 1.20.1 depends on loader metadata, configs and mod-specific dependency rules; inventory cannot replace actual loader resolution.
- Follow-up checks: Merge or explicitly reconcile dependency sources, test duplicate IDs and aliases, verify external dependency mapping and cycle reporting.
- RustChunk implementation mapping: RustChunk pack inventory model separated from mod runtime/registry.
- Evidence required before adoption: Forge 47.x modpack manifests and dependency graph differential tests.
- Completion state: COMPLETE — 229 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 8212-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1911. `crates/horizon-port-pack/src/jar.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `2fcf5a356f872b5142d700b8563d5c69a42617ef`
- Size: 21609 bytes; 621 logical lines
- Coverage: 1–400, 401–621 (EOF)
- Purpose: Reads JAR metadata (NeoForge/Forge/Fabric/legacy Forge) and counts assets, data files and class files.
- Dependencies / cross-references: ZIP central directory, mods.toml, neoforge.mods.toml, fabric.mod.json, mcmod.info and manifest.
- Implementation / logic: Extracts mod identity/license/dependencies and counts blockstates, item models, recipes, loot, tags, worldgen, structures, textures and class files.
- Findings: Metadata read helpers load ZIP entries without explicit uncompressed limits; census counts file paths only and does not prove their data is valid or behavior runs. Legacy metadata intentionally omits dependencies/license.
- RustChunk relevance: Useful pack inspector metadata parser, not a game loader.
- Decision: adaptable; tooling reference; security-sensitive
- 1.20.1 implications: Target Forge 1.20.1 legacy metadata and old plural recipes/loot_tables conventions; maintain Java 17 compatibility.
- Follow-up checks: Add ZIP entry decompression bounds, malformed TOML/JSON cases, duplicate metadata, loader precedence and exact 1.20.1 file-layout fixtures.
- RustChunk implementation mapping: RustChunk offline mod metadata scanner used by tooling only.
- Evidence required before adoption: Representative Forge 1.20.1 jars, ZIP bomb limits and metadata parity tests.
- Completion state: COMPLETE — 621 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 21609-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1912. `crates/horizon-port-pack/src/legacy_structure_mods/ctov.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `49c6fe6cb1727d8988ee2e882e3c7e12b265a5a2`
- Size: 3284 bytes; 104 logical lines
- Coverage: 1–104 (EOF)
- Purpose: Hard-coded carry-over entries for CTOV village and pillager-outpost structures.
- Dependencies / cross-references: CTOV mod ID, default configuration and target vanilla structure-set IDs/weights.
- Implementation / logic: Generates 21 village variants in three size weights and 11 outpost variants with default weights.
- Findings: Only the default CTOV configuration is represented; user config and CTOV releases may change entries/weights.
- RustChunk relevance: A narrow worldgen migration reference only.
- Decision: unnecessary; mod-specific migration data; version-specific
- 1.20.1 implications: Not reusable as general 1.20.1 worldgen logic; the relevant sets and default weights must be sourced from the exact mod/version.
- Follow-up checks: Check CTOV version/config, registry IDs, weights, existing vanilla entries and duplicate set additions.
- RustChunk implementation mapping: No base-client mapping; optional mod-specific data migration if a matching target mod is present.
- Evidence required before adoption: Exact CTOV jar/config fixture and target 1.20.1 structure-set parity.
- Completion state: COMPLETE — 104 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 3284-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1913. `crates/horizon-port-pack/src/legacy_structure_mods/mod.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `acc38a289590a35d4f28bc24d0821ad874d50ee7`
- Size: 806 bytes; 16 logical lines
- Coverage: 1–16 (EOF)
- Purpose: Declares the legacy structure-mod conversion submodule.
- Dependencies / cross-references: ctov module declaration and documentation references to YUNG/Lithostitched helpers.
- Implementation / logic: Exports `ctov` internally; general converter reaches these helpers through crate-private entrypoints.
- Findings: Comments describe YUNG and Lithostitched submodules that are declared elsewhere at crate root or not activated in this file; this is only module wiring.
- RustChunk relevance: No direct player/runtime value.
- Decision: unnecessary; tooling structure
- 1.20.1 implications: No Minecraft 1.20.1 behavior.
- Follow-up checks: Verify crate module ownership and remove stale documentation references only if maintaining this migration tool.
- RustChunk implementation mapping: No RustChunk client mapping.
- Evidence required before adoption: Static module/API review if this tool is adopted.
- Completion state: COMPLETE — 16 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 806-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1914. `crates/horizon-port-pack/src/lib.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `84a01dc4d81e0568297549e6b498c7793f930d5c`
- Size: 775 bytes; 27 logical lines
- Coverage: 1–27 (EOF)
- Purpose: Crate root and module exports for the Horizon modpack migration tool.
- Dependencies / cross-references: carry, check, classes, classify, fetch, import, inventory, jar, modrinth, net, oracle, pipeline, processors, report, resourcepacks, upgrade and worldgen.
- Implementation / logic: Exports public migration steps and keeps legacy structure conversion internals crate-private.
- Findings: The root belongs to a separate CLI/migration package; it is not linked into Minecraft client runtime.
- RustChunk relevance: Only useful as an organizational reference for a separate tool.
- Decision: unnecessary; tooling-only
- 1.20.1 implications: Do not wire this crate into the Android runtime or Forge protocol layer.
- Follow-up checks: Review API visibility and dependency isolation only if a standalone tool is desired.
- RustChunk implementation mapping: No game-runtime mapping; optional independent pack utility.
- Evidence required before adoption: Separate packaging/build requirement and static API review.
- Completion state: COMPLETE — 27 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 775-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1915. `crates/horizon-port-pack/src/lithostitched.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `cfb1013b79ca022eb27c353150bc8fa4878e0a83`
- Size: 19482 bytes; 477 logical lines
- Coverage: 1–400, 401–477 (EOF)
- Purpose: Converts Lithostitched jigsaws, modifiers and processors into Horizon 26.3 worldgen files.
- Dependencies / cross-references: Lithostitched schema, NeoForge mod-loaded conditions, Horizon Worldgen Upgrade/report, processor carry and CTOV compatibility data.
- Implementation / logic: Renames supported jigsaw structures, unwraps selected pool elements, evaluates condition trees, generates horizon:add_to files and recursively carries supported processor lists.
- Findings: Some Lithostitched features/processors are deliberately lost or unsupported; conditions with unknown types fail closed. Size providers are reduced to a selected maximum and jigsaw size is clamped to 20.
- RustChunk relevance: Worldgen conversion architecture reference only; not a renderer or base-client feature.
- Decision: rewrite required; tooling-only; worldgen-specific
- 1.20.1 implications: 1.20.1 has different worldgen data and processor contracts; only target-version Java/Forge behavior can be authoritative.
- Follow-up checks: Test all supported condition types, nested/unknown processors, pool semantics, load order/priority and loss-report completeness.
- RustChunk implementation mapping: Possible separate RustChunk worldgen migration utility, with explicit 1.20.1 target schema.
- Evidence required before adoption: Differential fixtures from the exact source/target mod versions and in-world structure-generation validation.
- Completion state: COMPLETE — 477 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 19482-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1916. `crates/horizon-port-pack/src/main.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `9eaa71e049e525ca6d4008b4bee7b7dd11d9b16d`
- Size: 6478 bytes; 142 logical lines
- Coverage: 1–142 (EOF)
- Purpose: CLI entrypoint that orchestrates import, inventory, fetch, carry, report, check and 26.3 datapack upgrade commands.
- Dependencies / cross-references: horizon_port_pack library, pipeline, fetch, inventory and upgrade modules.
- Implementation / logic: Parses global options and dispatches subcommands; datapack upgrade writes into an output folder or world/datapacks/<name>-26.3.
- Findings: This executable is developer/user tooling for Horizon modpack migration, not a game launcher runtime.
- RustChunk relevance: Potential user-facing workflow reference only.
- Decision: unnecessary; tooling-only; version-specific
- 1.20.1 implications: No 1.20.1 client protocol effect; datapack-upgrade path explicitly targets 26.3.
- Follow-up checks: Validate argument combinations, unknown/unused flags, path handling and cancellation if building an independent RustChunk tool.
- RustChunk implementation mapping: No client runtime mapping; optional separate CLI if product scope requires pack migration.
- Evidence required before adoption: CLI integration tests and target-specific pack fixture suite.
- Completion state: COMPLETE — 142 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 6478-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1917. `crates/horizon-port-pack/src/modrinth.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `ef327cc493e22459d2775e6624e84f3cdaf21e4e`
- Size: 4057 bytes; 115 logical lines
- Coverage: 1–115 (EOF)
- Purpose: Fetches and caches Modrinth version/project metadata by file SHA-1.
- Dependencies / cross-references: Modrinth v2 API, ureq net agent, JSON and BTreeMap cache.
- Implementation / logic: Batches up to 500 hashes per request and 100 project IDs per query; resolves project metadata and required dependencies.
- Findings: API responses are read fully into strings without a clear response-size cap; cache may accumulate indefinitely and no expiry/versioning policy is visible.
- RustChunk relevance: Useful pack-management metadata layer only.
- Decision: adaptable; tooling reference; network-sensitive
- 1.20.1 implications: Not part of Minecraft login or server protocol. A RustChunk pack catalog should have rate limits, cache expiry and user consent.
- Follow-up checks: Bound HTTP responses, enforce API retries/rate limits, validate cache schema/version and handle deleted/renamed projects.
- RustChunk implementation mapping: Separate optional Modrinth metadata provider outside game runtime.
- Evidence required before adoption: API fixtures, oversized/malformed responses, offline/stale cache tests and rate-limit behavior.
- Completion state: COMPLETE — 115 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 4057-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1918. `crates/horizon-port-pack/src/net.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `a441fb13a0648aca7799e9fbf5ee1ac3ac16c5f1`
- Size: 14619 bytes; 389 logical lines
- Coverage: 1–389 (EOF)
- Purpose: Shared proxy selection, NO_PROXY bypass and reusable ureq agents for port-pack network traffic.
- Dependencies / cross-references: ureq Agent/Proxy, HTTP(S)_PROXY/ALL_PROXY, NO_PROXY, mutex cache and redacted diagnostics.
- Implementation / logic: Chooses launcher or environment proxy, bypasses loopback/NO_PROXY hosts, caches agents by proxy and redacts credentials in logs.
- Findings: SOCKS schemes are explicitly unsupported and fall back to direct; URL host parsing is manual and should be tested against IPv6/credentials/edge cases.
- RustChunk relevance: Potential networking utility reference, not a Minecraft packet transport.
- Decision: adaptable; reference; network-security-sensitive
- 1.20.1 implications: Android app proxy policy and Java/Forge networking must be integrated separately; do not assume ureq settings affect Minecraft sockets.
- Follow-up checks: Fuzz URL host extraction, validate proxy schemes, verify environment precedence, redirects, credential redaction and concurrent agent replacement.
- RustChunk implementation mapping: Optional tooling HTTP client proxy utility; separate from game TCP/TLS/auth stack.
- Evidence required before adoption: Proxy integration tests on Android and desktop, IPv6/NO_PROXY matrix and credential-leak tests.
- Completion state: COMPLETE — 389 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 14619-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1919. `crates/horizon-port-pack/src/oracle.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `1f0a1f6f1df6c5da7acf05c482b68a20ce530694`
- Size: 36020 bytes; 858 logical lines
- Coverage: 1–400, 401–800, 801–858 (EOF)
- Purpose: Loads NeoForge oracle facts and chooses vanilla block/item templates plus component patches.
- Dependencies / cross-references: facts.json exported from a running NeoForge game, block/item registries, state shapes/sounds/drops, vanilla defaults, armor assets and report loss tracking.
- Implementation / logic: Scores vanilla block templates by class/state/shape/sound/hardness, infers self-drop rules, builds item declarations/components and compares vanilla template components.
- Findings: Oracle facts are newer-version-specific. Template matching is heuristic; custom classes/food items and invalid armor layer layouts may be left out. File facts are read fully into memory.
- RustChunk relevance: Useful oracle-driven registry migration architecture; block/item data is central to a Rust client but this schema is not reusable unchanged.
- Decision: rewrite required; core reference; version-specific; heuristic
- 1.20.1 implications: Generate new facts from exact Minecraft 1.20.1 + Forge 47.x and use legacy ItemStack NBT rather than 1.21.1/26.3 components.
- Follow-up checks: Check class chain, all blockstate property combinations, geometry equivalence, drop-self conditions, armor paths, food/custom item exclusions and numeric bounds.
- RustChunk implementation mapping: RustChunk optional oracle/facts pipeline and block/item registry adapter for 1.20.1.
- Evidence required before adoption: Complete 1.20.1 vanilla/Forge facts export, per-state parity and in-world block/item behavior fixtures.
- Completion state: COMPLETE — 858 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 36020-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1920. `crates/horizon-port-pack/src/pipeline.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `f3298f1b1d018456feb1e1c023a1478f2f9591f1`
- Size: 16602 bytes; 430 logical lines
- Coverage: 1–400, 401–430 (EOF)
- Purpose: Orchestrates pack import, inventory, jar fetch, content carry and coverage reporting in a resumable work folder.
- Dependencies / cross-references: Pack manifest, Modrinth cache, JAR inventory, Oracle facts, resourcepacks, carry reports, dependency order and harness outputs.
- Implementation / logic: Runs each pipeline step, selects portable data-tier mods, tracks shared namespaces, invokes carry and builds coverage report.
- Findings: Portable means content/data tier; it does not mean code mods execute. Shared namespace handling and mod order are migration-tool rules; oracle/harness outputs are external dependencies.
- RustChunk relevance: Useful workflow architecture for an offline pack migration utility, not gameplay runtime.
- Decision: adaptable; tooling reference; version-specific
- 1.20.1 implications: A RustChunk 1.20.1 pipeline needs Forge loader/resolution and mod runtime support; Horizon folder mods are not equivalent to Forge JARs.
- Follow-up checks: Test missing jars, conflicting namespaces, partial carry reruns, stale reports, interrupted steps and reproducible output.
- RustChunk implementation mapping: Separate RustChunk pack-analysis/import pipeline if needed; never merge with live game tick/network path.
- Evidence required before adoption: Real 1.20.1 Forge pack corpus and repeatable import/carry/harness results.
- Completion state: COMPLETE — 430 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 16602-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1921. `crates/horizon-port-pack/src/processors.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `52acdf1fdf99a93a7e37f92b56dd8165ed1ae2ca`
- Size: 37113 bytes; 887 logical lines
- Coverage: 1–400, 401–800, 801–887 (EOF)
- Purpose: Converts selected Java structure processors into Horizon-native processor data and renames supported rule tests.
- Dependencies / cross-references: NeoForge ZIP entries, Moog/Repurposed Structures data, Lithostitched helpers, blockstate/processor schemas and Horizon worldgen processor types.
- Implementation / logic: Loads spawner lists, vanilla replacement presets and processor lists; converts selected spawner/pillar/trial-spawner/vault/waterlogging/structure-void processors.
- Findings: Only a narrow set of known mod processors is translated; unknown processors remain and can later be dropped. Several transformations assume exact 26.3 NBT/state/property names.
- RustChunk relevance: Worldgen tooling reference only; not part of core client rendering or multiplayer.
- Decision: rewrite required; tooling-only; worldgen-specific
- 1.20.1 implications: Must reconstruct supported processor types and NBT/state semantics for Forge 1.20.1; Horizon custom processor IDs are not valid Forge registrations.
- Follow-up checks: Audit nested list depth, archive entry sizes, NBT field conversion, processor loss accounting, RNG order and target state IDs.
- RustChunk implementation mapping: Potential standalone RustChunk worldgen converter/harness, isolated from client engine.
- Evidence required before adoption: Target 1.20.1 processor fixtures and generated structures validated in a real world with Forge mods.
- Completion state: COMPLETE — 887 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 37113-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1922. `crates/horizon-port-pack/src/report.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `b00e6387f570dedef9fcfbc26d6fbd3eb62d9497`
- Size: 16575 bytes; 468 logical lines
- Coverage: 1–400, 401–468 (EOF)
- Purpose: Generates per-mod coverage status from carry-report, harness results, dependency states and license heuristics.
- Dependencies / cross-references: Inventory records, carry-report.json, harness/result.json and resource-pack outcomes.
- Implementation / logic: Assigns Ported/Partial/NotYet/NotNeeded, aggregates classes/tiers/licenses and writes Markdown/JSON coverage.
- Findings: Status Ported requires a harness result with no failures; absent harness stays Partial. License-family classification is heuristic and not a legal determination.
- RustChunk relevance: Useful reporting concept for a modpack conversion utility; no game runtime value.
- Decision: adaptable; tooling reference; reporting-only
- 1.20.1 implications: Do not use coverage status as proof of Forge 1.20.1 compatibility or legal permission.
- Follow-up checks: Validate harness schema, missing/stale result handling, dependency propagation and SPDX license edge cases.
- RustChunk implementation mapping: RustChunk pack audit dashboard/report, separate from launcher and gameplay.
- Evidence required before adoption: Known harness fixtures, stale report tests and manually reviewed license classifications.
- Completion state: COMPLETE — 468 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 16575-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1923. `crates/horizon-port-pack/src/resourcepacks.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `98b0bfceb0187d21b9caada3a44abff01eab1b13`
- Size: 21730 bytes; 590 logical lines
- Coverage: 1–400, 401–590 (EOF)
- Purpose: Stages embedded/standalone resource packs safely and fetches indexed client resource-pack ZIPs.
- Dependencies / cross-references: MRPack/CurseForge overrides, path sanitizer, ZIP central directory, file-count/byte limits, symlink checks and SHA-1 downloader.
- Implementation / logic: Extracts client resource packs, resolves priority between overrides, validates path/case collisions, writes without overwriting existing packs and reports state.
- Findings: Strong controls include 65,536 file and 512MiB limits, path and symlink checks, duplicate-central-entry rejection and hash verification. Copying explicitly does not validate rendering.
- RustChunk relevance: Relevant to resource-pack management; optional pack staging, not a proof of renderer support.
- Decision: adaptable; core tooling reference; security-sensitive
- 1.20.1 implications: Preserve resource-pack order/format for Minecraft 1.20.1; shader packs need a separate renderer pipeline. Do not treat staged as compatible.
- Follow-up checks: Test ZIP64/central directory edge cases, decompression bombs, case collisions, platform path rules, interrupted writes and large real packs.
- RustChunk implementation mapping: RustChunk resource-pack importer/stager with isolated VFS and explicit user pack activation.
- Evidence required before adoption: Adversarial archive suite, 1.20.1 pack corpus, atomic copy tests and Android storage lifecycle validation.
- Completion state: COMPLETE — 590 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 21730-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1924. `crates/horizon-port-pack/src/resourcepacks/tests.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `4ab07dfce1895e57c929adbb50812c50acb16a3d`
- Size: 15374 bytes; 459 logical lines
- Coverage: 1–400, 401–459 (EOF)
- Purpose: Regression tests for safe resource-pack staging, downloads, profile copying and status reporting.
- Dependencies / cross-references: resourcepacks module, temporary filesystem, ZIP writer, hash store and pipeline/report.
- Implementation / logic: Tests override precedence, CurseForge root, symlink/path traversal, collision and file-count bounds, hash downloads, no-overwrite and interrupted writes.
- Findings: Strong security test inventory, but embedded tests were only inspected; behavior has not been executed in this audit.
- RustChunk relevance: Test design reference for RustChunk's future resource-pack installer.
- Decision: undetermined; tests only; reference
- 1.20.1 implications: Port test intents to 1.20.1 resource-pack semantics and Android scoped storage paths.
- Follow-up checks: Run under target toolchain, add ZIP64 and decompression-bomb cases, Android storage failures and concurrent install cases.
- RustChunk implementation mapping: RustChunk resource-pack import unit/integration/security tests.
- Evidence required before adoption: Executed tests, adversarial fixtures, Android device storage validation and 1.20.1 pack compatibility results.
- Completion state: COMPLETE — 459 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 15374-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1925. `crates/horizon-port-pack/src/upgrade/density.rs`
- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Source SHA: `7ec30c6edcf21a04a558e59f80ca97a642e868a2`
- Size: 14020 bytes; 323 logical lines
- Coverage: 1–323 (EOF)
- Purpose: Converts Minecraft 26.2 density functions into the 26.3 density-function schema.
- Dependencies / cross-references: Named density-function graph, noise settings/router, noise holders, spline recursion, interpolation-cell metadata and conversion loss report.
- Implementation / logic: Traverses function references from noise routers, infers cells, maps operation names and rewrites flat/cache/interpolated/gradient/shifted-noise/spline forms.
- Findings: Explicitly targets 26.2→26.3; shared interpolated functions across different cells are lossy and reported. Unknown types are logged and preserved unchanged; some edge cases such as equal gradient endpoints are approximated.
- RustChunk relevance: Terrain/worldgen conversion reference only; no direct renderer or networking value.
- Decision: rewrite required; tooling-only; worldgen-specific; version-specific
- 1.20.1 implications: 1.20.1 density-function registry and serialization differ; rebuild from exact 1.20.1 sources, not these newer mappings.
- Follow-up checks: Test density graph cycles, unknown IDs, shared-cell functions, y-dependency analysis, overflow in cell-size math, spline nesting and numeric edge cases.
- RustChunk implementation mapping: Potential standalone 1.20.1 worldgen/data migration utility; not live terrain engine implementation.
- Evidence required before adoption: Differential density evaluations against Minecraft 1.20.1 Java with deterministic coordinates/seeds and full world terrain comparisons.
- Completion state: COMPLETE — 323 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 14020-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1926. `crates/horizon-port-pack/src/upgrade/features.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b2d4472b9a6b51eebcd744f44ef252e400dfe48d`
- Size: 4031 bytes; 122 logical lines
- Coverage: 1–122 (EOF)
- Purpose: Rewrites configured/placed worldgen features and block-state providers from the 26.2 schema to 26.3.
- Dependencies / cross-references: Horizon upgrade Report, block-state converter, nested feature/placement codecs and vanilla provider type names.
- Implementation / logic: Unwraps feature config, maps provider type names, converts random_offset into per-axis offset providers and recursively upgrades inline configured/placed features.
- Findings: Several feature types explicitly dropped in 26.3 are only reported unknown; the converter does not synthesize all vanilla replacement compositions.
- RustChunk relevance: Worldgen migration utility reference only; no live game or chunk renderer behavior.
- Decision: rewrite required; tooling-only; worldgen-specific
- 1.20.1 implications: 1.20.1 configured/placed feature registries and providers differ; rebuild from 1.20.1 vanilla/Forge codecs.
- Follow-up checks: Test every provider, nested feature, placement order/random draw sequence, removed features, defaults and unknown type accounting.
- RustChunk implementation mapping: Separate RustChunk 1.20.1 worldgen data migration adapter, not runtime feature execution.
- Evidence required before adoption: Differential generated-world fixtures and source-code parity from exact 1.20.1 Java/Forge.
- Completion state: COMPLETE — 122 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 4031-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1927. `crates/horizon-port-pack/src/upgrade/mod.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0513b47501161ea952da5226dc735eea1d558591`
- Size: 25770 bytes; 632 logical lines
- Coverage: 1–400, 401–632 (EOF)
- Purpose: Top-level 26.2-to-26.3 datapack upgrader for worldgen registries and associated file layouts.
- Dependencies / cross-references: Density/features/noise/registry/settings submodules, serde JSON, ZIP/filesystem reader and Report.
- Implementation / logic: Parses data JSON, rewrites selected worldgen registries and paths, generates named density/material-rule files, updates pack.mcmeta and writes the upgraded file set.
- Findings: Directory reads recursively without symlink containment; ZIP entries and files are read without clear decompressed-size/aggregate limits; duplicate archive paths silently overwrite; writing output is not staged as one atomic directory transaction.
- RustChunk relevance: Useful worldgen migration architecture, not client runtime; explicit source/target is 26.2→26.3.
- Decision: rewrite required; tooling-only; version-specific; security-sensitive
- 1.20.1 implications: Minecraft 1.20.1 datapack and registry schemas differ; do not use format 121 or the newer worldgen API.
- Follow-up checks: Add ZIP bomb/file count limits, symlink-safe directory walking, duplicate-path rejection, atomic output staging and full 1.20.1 parity tests.
- RustChunk implementation mapping: Optional standalone RustChunk 1.20.1 datapack migration tool after target schema design.
- Evidence required before adoption: Adversarial archive/directory corpus, rollback tests and differential generation in the actual 1.20.1 Forge runtime.
- Completion state: COMPLETE — 632 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 25770-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1928. `crates/horizon-port-pack/src/upgrade/noise.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cb770610f14a4ce4946a6ad58fca4fc743250762`
- Size: 3801 bytes; 112 logical lines
- Coverage: 1–112 (EOF)
- Purpose: Converts 26.2 noise parameters into 26.3 NormalNoise parity parameters.
- Dependencies / cross-references: firstOctave/amplitudes, vanilla parity normalization equations and upgrade loss reporting.
- Implementation / logic: Computes base amplitude, base octave, octave count and amplitude modifiers; leaves unsupported inputs unchanged with a loss report.
- Findings: Rejects negative or excessive octave arrays, but casts i64 firstOctave to i32 for parity calculation; extreme values need explicit range validation.
- RustChunk relevance: Worldgen conversion math reference only.
- Decision: rewrite required; tooling-only; worldgen-specific; numeric-review
- 1.20.1 implications: 1.20.1 noise parameter codec and normalization code must be taken from its exact source version.
- Follow-up checks: Test signed octave boundaries, empty/all-zero arrays, Kahan sum vectors, float edge cases and Java implementation parity.
- RustChunk implementation mapping: Versioned noise parameter conversion module for offline 1.20.1 pack tooling only.
- Evidence required before adoption: Bit-for-bit or tolerance-defined differential vectors against target Java NormalNoise.
- Completion state: COMPLETE — 112 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 3801-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1929. `crates/horizon-port-pack/src/upgrade/registries.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0887695e5df7b9c00f67870c152abf5a87cd92e7`
- Size: 5945 bytes; 168 logical lines
- Coverage: 1–168 (EOF)
- Purpose: Converts carvers, biomes, structures and dimension types into 26.3 registry forms.
- Dependencies / cross-references: Horizon Report, block-state conversion, vanilla carver definitions, biome spawn attributes and dimension-type fields.
- Implementation / logic: Unwraps carver config, spells out cave parameters, migrates biome spawners/probability into attributes, converts structure spawn counts and bed-rule field names.
- Findings: Several carver parameters are hard-coded by type; lossy fields are reported, but behavior still depends on the exact 26.2 definitions and target defaults.
- RustChunk relevance: Useful registry migration reference; no rendering or network implementation.
- Decision: rewrite required; tooling-only; registry-specific
- 1.20.1 implications: Forge 1.20.1 uses legacy registry and biome/carver serializers; these 26.3 fields are not compatible.
- Follow-up checks: Verify cave/nether carver constants, spawn category defaults, missing attributes, dimension bed behavior and modded registry fields.
- RustChunk implementation mapping: Target-version registry conversion layer for an optional datapack utility.
- Evidence required before adoption: 1.20.1 vanilla/Forge registry fixtures and generated terrain/spawn behavior comparison.
- Completion state: COMPLETE — 168 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 5945-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1930. `crates/horizon-port-pack/src/upgrade/settings.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8c5ecc5ab0905a7acc070076a225cf421b5f6238`
- Size: 11535 bytes; 252 logical lines
- Coverage: 1–252 (EOF)
- Purpose: Converts noise settings, dimensions and world presets to 26.3 aquifer/material-rule/noise-router schemas.
- Dependencies / cross-references: Density Context, noise settings, named density functions, ore-vein parameters, surface rules and spawn targets.
- Implementation / logic: Extracts named climate functions, moves aquifer controls, creates named preliminary surface/material rules, adds beardifier and converts ore veins into material-rule data.
- Findings: Deep-dark aquifer exclusion and ore-vein composition are manually encoded; spawn target depth/offset outside zero are explicitly lossy; settings already containing material_rule are passed through unchanged.
- RustChunk relevance: Worldgen migration reference only.
- Decision: rewrite required; tooling-only; worldgen-specific
- 1.20.1 implications: 1.20.1 noise router, aquifers, ore veins and surface rules are different; reconstruct from target source and datapacks.
- Follow-up checks: Differentially test density values, interpolation cells, aquifer exclusion boundaries, vein RNG, spawn target mapping and already-upgraded files.
- RustChunk implementation mapping: Offline 1.20.1 worldgen settings migrator, not the terrain sampling runtime.
- Evidence required before adoption: Seed/coordinate sampling against vanilla 1.20.1 and real Forge world-generation fixtures.
- Completion state: COMPLETE — 252 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 11535-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1931. `crates/horizon-port-pack/src/worldgen.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ea48c0833b14a44512d301ea36c7796db8385d07`
- Size: 55709 bytes; 1296 logical lines
- Coverage: 1–400, 401–800, 801–1200, 1201–1296 (EOF)
- Purpose: Carries structure-mod worldgen into Horizon via custom start checks, piece-limited rules and translated processors.
- Dependencies / cross-references: Moog/Repurposed/YUNG/Lithostitched modules, NeoForge JAR data, structure pools/sets, tags, piece counts, processor carry and Horizon worldgen APIs.
- Implementation / logic: Maps selected mod structures and placement types to vanilla/Horizon types, expands structure-set tags, migrates pool elements/piece counts and records unsupported/lost rules or processors.
- Findings: Large hard-coded mod/version tables (Mo' Structures defaults, Moog spacing, CTOV/YUNG contracts) and custom Horizon-only keys make output non-portable; Java processors may be dropped and some layout rules are explicitly lost; ZIP count/tag reads have no visible per-entry size cap.
- RustChunk relevance: Only an offline worldgen migration tool reference; not a standard Forge client implementation.
- Decision: rewrite required; tooling-only; worldgen-specific; security-sensitive
- 1.20.1 implications: Forge 1.20.1 structure codecs and mod versions differ; Horizon `horizon:` keys are not registered in Forge.
- Follow-up checks: Validate mod version/config tables, chunk-spacing math, tag cycles, missing optional tags, processor-loss accounting, archive limits and generation in target world.
- RustChunk implementation mapping: Potential independent RustChunk modpack worldgen converter with exact 1.20.1/Forge adapters.
- Evidence required before adoption: Source/target mod version fixtures, adversarial JAR tests and actual generated-world parity.
- Completion state: COMPLETE — 1296 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 55709-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1932. `crates/horizon-port-pack/src/yung.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3734a1a9572e569a46100b0b9c613e5244b818d2`
- Size: 4156 bytes; 102 logical lines
- Coverage: 1–102 (EOF)
- Purpose: Retains selected YUNG's API structure/pool formats and reports unsupported rules.
- Dependencies / cross-references: YUNG API spec, structure type ids, single-pool element schema and carry loss report.
- Implementation / logic: Passes known YUNG jigsaw/pool records through unchanged and counts fields whose behavior the clean-room spec leaves open.
- Findings: The module admits open spec items; unchanged carry does not establish that all behavior is implemented.
- RustChunk relevance: Narrow migration reference only.
- Decision: unnecessary; mod-specific migration; version-specific
- 1.20.1 implications: YUNG's exact Forge 1.20.1 API version/codecs must be separately examined; Horizon format is not a Forge loader.
- Follow-up checks: Compare the documented open fields against the exact target YUNG API release and world-generation behavior.
- RustChunk implementation mapping: No base-client mapping; optional YUNG-specific conversion adapter only.
- Evidence required before adoption: Exact YUNG 1.20.1 source/spec and in-world structure parity.
- Completion state: COMPLETE — 102 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 4156-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1933. `crates/horizon-prepare/build.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fa2ef8a11a266835200d2de00bcb31c0dd2ce4cb`
- Size: 1479 bytes; 38 logical lines
- Coverage: 1–38 (EOF)
- Purpose: Build script computes a digest of Horizon Prepare crate source for prepared-version cache compatibility.
- Dependencies / cross-references: Cargo.toml, build.rs, direct src/*.rs files and SHA-256.
- Implementation / logic: Sorts top-level Rust source filenames, removes CR bytes and hashes names/lengths/content into a compile-time environment variable.
- Findings: Only direct files under src are included; nested source modules/directories and tests are omitted, so modifications there may not invalidate the digest.
- RustChunk relevance: Cache/versioning pattern can inform RustChunk build metadata.
- Decision: adaptable; build-tooling reference; cache-integrity-sensitive
- 1.20.1 implications: Keep RustChunk cache key tied to exact runtime code, assets and target Minecraft version; this build script is not reusable unchanged.
- Follow-up checks: Include recursive source files, compiler flags and relevant generated inputs; test hash invalidation for nested modules.
- RustChunk implementation mapping: RustChunk build/cache digest generator for offline prepared data bundles.
- Evidence required before adoption: Change-detection tests covering nested modules, resources and compiler setting changes.
- Completion state: COMPLETE — 38 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 1479-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1934. `crates/horizon-prepare/output-versions.txt`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d31a9902c725cbd884976305548612c1455fd631`
- Size: 403 bytes; 5 logical lines
- Coverage: 1–5 (EOF)
- Purpose: Append-only mapping from prepared output version numbers to historical prepared-file digests.
- Dependencies / cross-references: OUTPUT_VERSION source constant and prepare_output.py digest output.
- Implementation / logic: Stores one version/digest pair per line and instructs developers to append rather than edit history.
- Findings: Historical data only; integrity depends on the separate generation workflow and human discipline.
- RustChunk relevance: Useful release bookkeeping reference, not a game runtime component.
- Decision: reference; build metadata only
- 1.20.1 implications: RustChunk may keep independent prepared-data version history; values belong to this Horizon tree.
- Follow-up checks: Verify generator, duplicate versions, digest format and release process if adopting a similar register.
- RustChunk implementation mapping: RustChunk prepared artifact version/digest history file.
- Evidence required before adoption: Reproducible generation and CI validation of monotonic append-only entries.
- Completion state: COMPLETE — 5 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 403-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1935. `crates/horizon-prepare/src/exporters.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ae32af4f94e9089c27f65464faba6d38532f09a2`
- Size: 8664 bytes; 226 logical lines
- Coverage: 1–226 (EOF)
- Purpose: Builds and verifies Java parity exporter bundles from exact version-pinned sources and classpath.
- Dependencies / cross-references: javac/java selection, research-lock, parity Java sources, client/server classpaths, class digests and temp publication.
- Implementation / logic: Hashes exporter sources and pins, compiles separate server/client classes, validates class resolution, manifests outputs and publishes verified bundles.
- Findings: Recursive source walk may follow symlinks; classpath pin correctness depends on research-lock. Bundle is desktop/JVM preparation tooling and not part of game runtime.
- RustChunk relevance: Oracle-generation architecture may help produce authoritative RustChunk facts.
- Decision: adaptable; preparation-tooling reference; version-specific
- 1.20.1 implications: Any RustChunk oracle must compile against exact 1.20.1 Forge/vanilla jars and Java 17; Horizon exporter lists are 26.3-specific.
- Follow-up checks: Test nested source invalidation, JDK mismatch, partial compile cleanup, classpath pin tampering, symlink traversal and atomic publication across OSes.
- RustChunk implementation mapping: Separate RustChunk fact-exporter preparation tool, optional and outside client runtime.
- Evidence required before adoption: Reproducible Java 17 builds, exact Forge 47.x pin hashes, class-resolution tests and deterministic exported facts.
- Completion state: COMPLETE — 226 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 8664-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1936. `crates/horizon-prepare/src/files.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `484c7bf7bd65b0042cfb13392e5fe26a6d7f9b80`
- Size: 6611 bytes; 180 logical lines
- Coverage: 1–180 (EOF)
- Purpose: Clones or streams files into a destination and atomically publishes checksum-verified imports.
- Dependencies / cross-references: Linux FICLONE/macOS clonefile, SHA-1/SHA-256, temporary sibling files and filesystem rename.
- Implementation / logic: Attempts reflink then falls back to streaming copy/hash; Import stages a temporary file, checks digest and renames on commit, deleting unfinished temp files.
- Findings: Independent copies avoid hardlink mutation. `rename` replacement behavior differs across Windows/Unix when destination exists; caller lifecycle and cleanup under partial multi-file commit deserve testing.
- RustChunk relevance: Useful safe-copy/checksum utility design for RustChunk asset and library preparation.
- Decision: adaptable; core preparation reference; cross-platform
- 1.20.1 implications: Android/Linux semantics and Java 17 launcher paths must be tested; do not assume Windows rename behavior from Unix.
- Follow-up checks: Test existing-destination replacement on Windows/Linux/Android, fsync/crash recovery, permissions, reflink fallback and multi-file rollback.
- RustChunk implementation mapping: RustChunk verified artifact import/copy layer for libraries, assets and cached prepared files.
- Evidence required before adoption: Cross-platform tests, interrupted copy/crash tests, checksum mismatch preservation and Android filesystem validation.
- Completion state: COMPLETE — 180 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 6611-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1937. `crates/horizon-prepare/src/inventory.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `826623c767a6bdec4dc14dc69bf98c5fed186934`
- Size: 6420 bytes; 177 logical lines
- Coverage: 1–177 (EOF)
- Purpose: Inventories JAR class files, SourceFile attributes, method counts and package distribution.
- Dependencies / cross-references: ZIP archive, class-file constant pool parser, SourceFile attribute and SHA-256.
- Implementation / logic: Reads each .class, extracts source file/method count, infers source paths and aggregates packages/classes into JSON.
- Findings: JAR class entry and class attributes are read into memory without explicit limits; modified UTF-8 is decoded lossily and malformed constant-pool/attribute edge cases need fuzzing.
- RustChunk relevance: Offline modpack/class inventory reference only.
- Decision: adaptable; tooling reference; archive-security-sensitive
- 1.20.1 implications: Forge 1.20.1 Java 17 class-file and mapping details should be targeted if used for mod analysis.
- Follow-up checks: Bound entry sizes/attribute allocations, handle modified UTF-8 precisely, fuzz malformed classes and test nested/anonymous class source inference.
- RustChunk implementation mapping: RustChunk mod-inspection inventory utility, separate from game loader.
- Evidence required before adoption: Adversarial JAR/class corpus, Java 17 fixtures, resource limits and class/source mapping tests.
- Completion state: COMPLETE — 177 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 6420-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1938. `crates/horizon-prepare/src/launch_api.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c23adc51a24778d7e860ed10ab019d266750a1d5`
- Size: 4420 bytes; 113 logical lines
- Coverage: 1–113 (EOF)
- Purpose: Defines the Horizon Prepare Launch API version, supported command-line options and response schema.
- Dependencies / cross-references: research-lock, pyjson serializer and shared Launch API documentation.
- Implementation / logic: Builds a JSON reply with API/Horizon/Minecraft/Java/schema/output versions and the accepted CLI flags/exit codes.
- Findings: This is a launcher-to-preparer API contract; it says nothing about game network or Forge compatibility.
- RustChunk relevance: Useful design reference for a launcher/preparation-process contract.
- Decision: adaptable; launcher tooling reference; versioned-api
- 1.20.1 implications: RustChunk should define its own versioned Android launcher API and 1.20.1 target; values and flags here are Horizon-specific.
- Follow-up checks: Version compatibility tests, required/optional flag validation, unknown-field behavior and client/preparer contract synchronization.
- RustChunk implementation mapping: RustChunk launcher-to-preparation service API, kept distinct from Minecraft protocol.
- Evidence required before adoption: Contract tests between launcher and preparer versions and CLI compatibility fixtures.
- Completion state: COMPLETE — 113 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 4420-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1939. `crates/horizon-prepare/src/pool.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `017dde877f6049b3a28e185eaf6be87876d42213`
- Size: 5601 bytes; 150 logical lines
- Coverage: 1–150 (EOF)
- Purpose: Runs bounded parallel work lanes for copy, hash and Java exporter preparation.
- Dependencies / cross-references: HORIZON_PREPARE_JOBS, available_parallelism, atomics, Mutex, scoped threads and panic handling for lanes.
- Implementation / logic: Maps independent tasks on up to eight workers, preserves item/result order, stops taking work after an error and serializes all jobs on small 32-bit Android.
- Findings: `map` worker panics are not caught (only lane panics are caught); result ordering and first-error behavior are tested only in embedded tests.
- RustChunk relevance: Useful bounded-concurrency design for a mobile preparation pipeline.
- Decision: adaptable; preparation reference; concurrency-sensitive
- 1.20.1 implications: Samsung ARM64 memory/thermal behavior differs from the 32-bit small-device heuristic; job limits must be measured on target Android.
- Follow-up checks: Add panic containment to map, cancellation tests, job limits under memory pressure and task fairness/cleanup tests.
- RustChunk implementation mapping: RustChunk bounded task pool for download/hash/decode/prepare phases; never spawn unbounded work.
- Evidence required before adoption: ARM64 device profiling, panic/cancellation stress, determinism and memory budget tests.
- Completion state: COMPLETE — 150 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 5601-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1940. `crates/horizon-prepare/src/pyjson.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f8e98787ced084196bec50931268e5594241f2b5`
- Size: 18223 bytes; 541 logical lines
- Coverage: 1–400, 401–541 (EOF)
- Purpose: Python-compatible JSON parser/serializer preserving insertion order, huge integers, float representation and ASCII escaping.
- Dependencies / cross-references: Custom byte parser, Python JSON grammar/repr behavior and ordered Value tree.
- Implementation / logic: Parses objects/arrays/numbers/strings and emits Python-like json.dumps formats, including special floats, duplicate-key position and UTF-16 surrogate pairs where valid.
- Findings: Recursive parser has no explicit nesting/input-size limits; lone surrogate escapes are rejected although Python can preserve them; float repr equivalence is tested with a limited fixed corpus only.
- RustChunk relevance: Could inform deterministic pack/manifest serialization where byte-level parity matters.
- Decision: adaptable; parser reference; robustness-sensitive
- 1.20.1 implications: RustChunk JSON often interoperates with Minecraft/Forge files; use standard JSON unless exact Python byte parity is required.
- Follow-up checks: Add recursion/size limits, differential Python corpus, lone-surrogate behavior, huge exponent, duplicate keys and float edge-case tests.
- RustChunk implementation mapping: Optional deterministic compatibility serializer for tool manifests only, not game packet codec.
- Evidence required before adoption: Python differential corpus, fuzzing, deep nesting limits and exact byte-level regression tests.
- Completion state: COMPLETE — 541 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 18223-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1941. `crates/horizon-prepare/src/sounds.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f702985ac36697dbd77b14147fad74ec942c95e2`
- Size: 40600 bytes; 1005 logical lines
- Coverage: 1–400, 401–800, 801–1005 (EOF)
- Purpose: Copies launcher-indexed sounds, panorama, fonts, built-in packs and languages into prepared version resources.
- Dependencies / cross-references: Minecraft launcher asset index, SHA-1 object store, Lock/manifest, safe_relative, atomic Imports and optional-import reporting.
- Implementation / logic: Validates asset index metadata, locates local object files, stages all selected assets, checks per-object SHA-1 and publishes verified files while preserving indexed replacements/placeholders on failures.
- Findings: Uses local launcher objects only and does not download. Per-file validation and late-corruption behavior are covered by tests; asset index itself is read into memory without an obvious maximum size.
- RustChunk relevance: Asset acquisition pattern directly relevant to a launcher, but platform paths/storage behavior need Android integration.
- Decision: adaptable; core launcher reference; asset-integrity-sensitive
- 1.20.1 implications: Minecraft 1.20.1 asset index/object layout must be used; sounds/font/panorama integration depends on the client resource manager and Android storage.
- Follow-up checks: Bound asset-index size/object count, test interrupted multi-file commit, stale indexes, corrupted cache, duplicate entries and Android scoped-storage permissions.
- RustChunk implementation mapping: RustChunk asset-store importer for 1.20.1 sounds/fonts/panorama/built-in packs, with safe user-facing recovery.
- Evidence required before adoption: 1.20.1 official asset index fixtures, SHA-1 corruption tests and real Samsung Android import/recovery test.
- Completion state: COMPLETE — 1005 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 40600-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1942. `crates/horizon-prepare/src/survival.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `16c451c3265c1d7f3dc1d47f698376de35cec126`
- Size: 5523 bytes; 146 logical lines
- Coverage: 1–146 (EOF)
- Purpose: Maps exported block classes and placement classes to vanilla block survival-placement rule identifiers.
- Dependencies / cross-references: Java oracle exports for class ownership, placement-check class and support tags.
- Implementation / logic: Uses ordered class lookup tables, special Crop/DoublePlant rules and support-tag-derived rules for vegetation subclasses.
- Findings: Hard-coded class mappings reflect the pinned version; unknown vegetation classes may lack definitions and required placement/support tags return an error.
- RustChunk relevance: Potentially useful block-placement semantics for a Rust world client.
- Decision: adaptable; core reference; version-specific
- 1.20.1 implications: Regenerate table from exact 1.20.1 vanilla/Forge classes; custom mod placement rules must be queried or explicitly unsupported.
- Follow-up checks: Test all registered block states, subclass override precedence, support tags, unknown modded blocks and placement parity against Java.
- RustChunk implementation mapping: RustChunk block survival/placement rule registry fed by target-version oracle facts.
- Evidence required before adoption: Complete 1.20.1 vanilla + Forge/modded oracle exports and differential placement tests.
- Completion state: COMPLETE — 146 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 5523-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1943. `crates/horizon-prepare/src/worker.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d2fef5949a8a43116f3770ff208d5dbc63185600`
- Size: 2686 bytes; 79 logical lines
- Coverage: 1–79 (EOF)
- Purpose: Manages a persistent worker JVM for request/reply calls to prepared Java exporter classes.
- Dependencies / cross-references: JDK Java process, two-way stdin/stdout, newline-delimited JSON, stderr log file and PrepareWorker class.
- Implementation / logic: Starts JVM with fixed 2G heap, sends one JSON task per line, waits up to one hour for reply and kills/waits on drop.
- Findings: Stdout lines are unbounded and the per-request timeout is very long; fixed heap may be inappropriate on constrained Android devices. Worker is desktop/JVM tooling, not game runtime.
- RustChunk relevance: Only relevant if RustChunk adopts a separate Java oracle/exporter during preparation.
- Decision: unnecessary; preparation-tooling; resource-sensitive
- 1.20.1 implications: Do not start a 2GiB exporter JVM on Samsung Android baseline; use host-side preparation or tightly bounded target process.
- Follow-up checks: Bound line length, add cancellation/shorter timeout, size heap per device, capture child exit state and avoid orphaned reader threads.
- RustChunk implementation mapping: Optional host-side exporter worker only; exclude from Android game process unless resource budget is proven.
- Evidence required before adoption: Host and Android process stress tests, memory budget, timeout/cancellation and child cleanup tests.
- Completion state: COMPLETE — 79 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 2686-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1944. `crates/horizon-prepare/tests/java_selection.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4e8d60f5590302a7fcf4a88c8f7c9ed7f674cbfc`
- Size: 5561 bytes; 145 logical lines
- Coverage: 1–145 (EOF)
- Purpose: Process-level integration tests for Java/JDK selection across PATH, JAVA_HOME, JRE-only and incompatible versions.
- Dependencies / cross-references: Fake compiled Java probe, subprocess environment isolation and Java selector.
- Implementation / logic: Builds stub java/javac executables and validates selection precedence, version rejection, symlinked paths and error messages in child processes.
- Findings: Tests target only the preparer's selection behavior with simulated Java executables; they do not validate a real Minecraft 1.20.1 Forge JVM launch.
- RustChunk relevance: Test strategy reference for Java runtime detection in a launcher.
- Decision: reference; tests only; launcher-tooling
- 1.20.1 implications: RustChunk needs actual supported Java 17 runtime discovery on Android and Forge 47.x, not the newer Horizon Java baseline.
- Follow-up checks: Run with real Java 17 runtimes on Windows/Linux/Android, verify architecture/ABI and Forge library arguments.
- RustChunk implementation mapping: RustChunk Java runtime detection integration test suite.
- Evidence required before adoption: Real runtime installations and successful 1.20.1 Forge startup on target Android device.
- Completion state: COMPLETE — 145 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 5561-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1945. `crates/horizon-prepare/tests/support/java_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `607c837cc359c8f5330cc9b9a6e37f3f873c70a0`
- Size: 779 bytes; 21 logical lines
- Coverage: 1–21 (EOF)
- Purpose: Fake Java executable used by process-level Java selection tests.
- Dependencies / cross-references: Current executable path, adjacent .version/.exit files and process arguments.
- Implementation / logic: Prints a controlled java/javac version string or exits with a selected status so tests need no installed JDK.
- Findings: Test helper only; it never compiles or runs Java bytecode.
- RustChunk relevance: No game runtime value.
- Decision: unnecessary; test fixture only
- 1.20.1 implications: No 1.20.1 runtime integration.
- Follow-up checks: Keep deterministic and isolated to tests; do not ship in launcher.
- RustChunk implementation mapping: No production mapping.
- Evidence required before adoption: Test build confirms it remains only in the integration-test target.
- Completion state: COMPLETE — 21 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 779-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1946. `crates/horizon-protocol/examples/item_wire_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e89a19856edb9a0125a90637bffa5179fa620b02`
- Size: 2991 bytes; 74 logical lines
- Coverage: 1–74 (EOF)
- Purpose: Reads sample item packet hex lines, decodes ItemStack wire data and checks bytes consumed and round-trip equality.
- Dependencies / cross-references: horizon-protocol Slot codec, component IDs, Reader/Writer and JSONL test fixtures.
- Implementation / logic: Decodes each stack, reports remaining bytes, added/removed/typed component IDs and whether encoding reproduces the original bytes.
- Findings: Harness is tied to the newer component-based item wire protocol; odd/non-ASCII hex input currently returns None through string slicing but malformed records are skipped.
- RustChunk relevance: Useful test-harness pattern for protocol parity, not player runtime.
- Decision: reference; example only; version-specific
- 1.20.1 implications: Minecraft 1.20.1 ItemStack wire/NBT fields differ from newer component-based encoding; adapt fixtures/codecs.
- Follow-up checks: Reject odd-length/non-hex with explicit errors, add malformed stack fuzzing and compare against 1.20.1/Forge oracle bytes.
- RustChunk implementation mapping: RustChunk protocol item-wire differential test tool.
- Evidence required before adoption: Known 1.20.1 packet captures, exhaustive item component/legacy tag fixtures and executed parity tests.
- Completion state: COMPLETE — 74 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 2991-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1947. `crates/horizon-protocol/examples/net_replay.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f5e936173159705de3bba4efeabfa151d5680343`
- Size: 7667 bytes; 205 logical lines
- Coverage: 1–205 (EOF)
- Purpose: Replays captured play packets through Connection receive/decrypt/decompress and times packet/chunk section decoding.
- Dependencies / cross-references: Custom capture format, Connection/Pipe, compression, AES, Clientbound packet decoder and chunk section parser.
- Implementation / logic: Loads captures, runs memory/TCP/compressed+AES receive passes, measures per-packet decode and section parsing, and prints top costs/errors.
- Findings: Truncated capture tails are silently ignored; capture fields/lengths and vector capacities have no explicit upper bound. Benchmark results depend on host CPU and are not Android measurements.
- RustChunk relevance: Strong benchmark harness concept for optimizing network/chunk decode pipeline.
- Decision: adaptable; benchmark reference; protocol-specific
- 1.20.1 implications: Capture format and packet IDs must match 1.20.1/Forge; benchmark must be run on actual ARM64 Mali device, not extrapolated from desktop.
- Follow-up checks: Reject malformed/truncated captures, cap file/packet sizes, add 1.20.1 packet captures, isolate warmup and collect per-stage Android metrics.
- RustChunk implementation mapping: RustChunk packet replay/performance benchmark for 1.20.1 chunk network and section decode.
- Evidence required before adoption: Validated 1.20.1 captures, reproducible Android ARM64 benchmark and correctness checks against live Forge server.
- Completion state: COMPLETE — 205 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 7667-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1948. `crates/horizon-protocol/examples/probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0e39a9f4b29dce18dacac9332e18bd8024ce1c5d`
- Size: 8572 bytes; 209 logical lines
- Coverage: 1–209 (EOF)
- Purpose: Connects in offline mode, collects configuration registries and probes live clientbound play packet/chunk decoding.
- Dependencies / cross-references: Handshake/login/config/play packet codecs, dynamic registry, connection encryption/compression and optional built-in registry file.
- Implementation / logic: Logs in with fixed offline profile, answers keepalive/ping/teleports/code-of-conduct, records packet counts/decode errors and parses chunk sections.
- Findings: Uses the pinned newer protocol and does not negotiate Forge mod channels or mod list; offline login cannot demonstrate Microsoft online-mode/Aternos compatibility.
- RustChunk relevance: Useful manual protocol diagnostic pattern, but not proof of 1.20.1 or modded server support.
- Decision: reference; example only; protocol-specific
- 1.20.1 implications: Replace packet IDs/handshake/config/registries with exact protocol 1.20.1 and Forge 47.x handshake/mod negotiation before use.
- Follow-up checks: Test online and offline auth separately, Forge handshake, registry sync, tags, compression, encrypted transport and chunk section bit widths.
- RustChunk implementation mapping: RustChunk 1.20.1 protocol diagnostic tool behind test builds only.
- Evidence required before adoption: Actual 1.20.1 Forge/Aternos packet capture and successful online/offline interoperability on target Android.
- Completion state: COMPLETE — 209 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 8572-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1949. `crates/horizon-protocol/examples/stop_sound_probe.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d09478bd534c4134a21cc332b94ac8be1e75baee`
- Size: 1182 bytes; 31 logical lines
- Coverage: 1–31 (EOF)
- Purpose: Decodes and re-encodes StopSound packet fixtures and checks the reader consumes the full payload.
- Dependencies / cross-references: horizon-protocol StopSound codec, JSONL wire fixtures and Reader/Writer.
- Implementation / logic: Reads hex strings, decodes StopSound, asserts no remaining bytes and exact byte-for-byte re-encoding; checks two malformed short packets fail.
- Findings: Test harness only; `hex[i..i+2]` can panic for odd-length or non-ASCII hex input rather than returning a clean error.
- RustChunk relevance: Useful packet codec test pattern; no runtime behavior.
- Decision: reference; example only; test-harness
- 1.20.1 implications: Packet layout/ID must be compared with 1.20.1 target protocol before reusing fixtures.
- Follow-up checks: Validate even ASCII hex length and characters, add malformed/truncated fixtures and confirm 1.20.1 StopSound field semantics.
- RustChunk implementation mapping: RustChunk StopSound wire codec regression tests.
- Evidence required before adoption: Executed 1.20.1 codec fixtures, fuzz input and round-trip parity.
- Completion state: COMPLETE — 31 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 1182-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.

### 1950. `crates/horizon-relay/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f6d3edb40d3c56673e61c7233d32b8dc6c393271`
- Size: 471 bytes; 19 logical lines
- Coverage: 1–19 (EOF)
- Purpose: Manifest for Horizon's optional friend-session relay server.
- Dependencies / cross-references: horizon-friends secure invite/tunnel crate, getrandom and serde_json.
- Implementation / logic: Declares a separate executable relay service that introduces hosts/guests and forwards encrypted streams when direct connection fails.
- Findings: This is peer-hosting infrastructure, not the normal client connection path to public Minecraft servers.
- RustChunk relevance: Only a possible optional service if RustChunk later implements friend-hosting.
- Decision: unnecessary; optional server-only; network-specific
- 1.20.1 implications: No Forge 1.20.1/Aternos client requirement; keep relay separate from Android game executable.
- Follow-up checks: Review protocol authentication, abuse/rate limits, bandwidth/resource ceilings, privacy/logging and deployment policy if ever adopted.
- RustChunk implementation mapping: Separate optional RustChunk relay backend, not client/runtime code.
- Evidence required before adoption: Explicit product requirement, threat model, relay interoperability, abuse tests and operational deployment validation.
- Completion state: COMPLETE — 19 logical lines read through EOF; coverage is contiguous; pinned blob SHA and 471-byte size match the complete recursive source tree; embedded tests/examples were inspected only, not run.


### 1951. `crates/horizon-relay/Dockerfile`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8f41b4ea3aee35ff846c7d3358bee57c8cf6d13c`
- Size: 568 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Multi-stage Docker image build for Horizon's optional play-with-friends TCP relay.
- Dependencies / cross-references: Rust 1-slim-bookworm build stage, Debian bookworm-slim runtime, workspace manifests and horizon-relay binary.
- Implementation / logic: Compiles `horizon-relay` in release mode, copies the executable into a minimal Debian image, exposes port 7000, and runs as `nobody`.
- Findings: Separates build tools from runtime and uses a non-root runtime account; this is deployment packaging, not client code.
- RustChunk relevance: No Forge 1.20.1 client, rendering or Aternos connectivity functionality.
- Decision: reference; deployment-only; optional relay
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Review base-image pinning, reproducible build, image scanning, runtime capabilities, CPU/memory limits and relay abuse protections.
- RustChunk implementation mapping: Optional standalone RustChunk relay container, only if friend-hosting is an explicit product requirement.
- Evidence required before adoption: Approved relay threat model, reproducible image build, integration/interoperability and deployment hardening tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1952. `crates/horizon-relay/horizon-relay.service`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `dc67babdd5603219cf13fbac6e191ec79abb8f78`
- Size: 750 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: systemd unit for self-hosting the optional Horizon friend relay.
- Dependencies / cross-references: systemd, `/usr/local/bin/horizon-relay`, TCP port 7000 and RELAY_* environment settings documented by the project.
- Implementation / logic: Starts relay after network-online, uses DynamicUser, NoNewPrivileges, ProtectSystem=strict, ProtectHome and PrivateTmp, and restarts on failure.
- Findings: Includes several useful systemd sandboxing controls; safe deployment still depends on trusted network/proxy configuration and configured resource limits.
- RustChunk relevance: Server deployment configuration only; no Android launcher or Forge 1.20.1 behavior.
- Decision: reference; deployment configuration; optional relay
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Check service filesystem needs, resource caps, trusted proxy boundaries, port exposure, and systemd compatibility on target distributions.
- RustChunk implementation mapping: Template for a separate RustChunk relay service; not copied into Android app.
- Evidence required before adoption: Deployment tests under an unprivileged dynamic user plus network and restart-failure tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1953. `crates/horizon-relay/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f30169107314737d6a18e23472a7390374e7fe28`
- Size: 23494 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: TCP relay service for Horizon play-with-friends sessions; transports opaque end-to-end encrypted bytes between host and guest.
- Dependencies / cross-references: horizon-friends wire/code protocol, rate limiter, TCP sockets, threads, mutexes, atomics and random token generation.
- Implementation / logic: Registers hosts, issues random tokens/nameplates, handles guest join/host attach, bridges two streams, throttles per-session and total bytes, exposes health/whoami HTTP responses and maintains in-memory counts.
- Findings: This is a friend-session fallback relay, not a normal public Minecraft server proxy. max_bridges is checked before incrementing, so concurrent joins can transiently exceed its cap; proxy protocol mode must only be exposed behind a trusted proxy.
- RustChunk relevance: Does not implement Forge 1.20.1 handshake, mod negotiation or Aternos connection. Optional service only.
- Decision: unnecessary; optional friend relay; network-specific
- 1.20.1 implications: This optional relay subsystem does not implement Forge 1.20.1 client protocol or mod negotiation.
- Follow-up checks: Add atomic bridge-slot reservation; validate all numeric limits; bound per-address state; test shutdown/thread cleanup, proxy trust, malformed frames, and concurrent capacity.
- RustChunk implementation mapping: Separate optional RustChunk friend-relay crate, isolated from launcher/game process.
- Evidence required before adoption: Threat model, concurrency stress tests, fuzzing, rate-limit tests, end-to-end encrypted tunnel interoperability and resource-exhaustion tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1954. `crates/horizon-relay/src/limits.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `adf199919887dc3223c6b753f2fff5301d657e0a`
- Size: 8694 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Token-bucket bandwidth and request rate limits plus client-address tracking for the friend relay.
- Dependencies / cross-references: std HashMap/VecDeque, IpAddr and Instant; consumed by horizon-relay.
- Implementation / logic: Implements refillable buckets, per-address registration/join limits, unknown-code blocks, IPv4-mapped address normalization and IPv6 /64 grouping; periodically removes idle address records.
- Findings: Core token-bucket approach is reusable, but float settings are not validated as finite/non-negative; NaN and pathological values can break timing calculations. Address tracking is only periodically swept.
- RustChunk relevance: Optional relay-only utility; unrelated to Minecraft 1.20.1 client transport.
- Decision: adaptable; rate-limiting reference; security-sensitive
- 1.20.1 implications: This optional relay subsystem does not implement Forge 1.20.1 client protocol or mod negotiation.
- Follow-up checks: Reject NaN/infinity/negative rates, define zero-rate semantics, bound tracked-address cardinality, and fuzz timestamp/rate edge cases.
- RustChunk implementation mapping: Optional RustChunk relay limiter with validated typed configuration.
- Evidence required before adoption: Property-based refill tests, invalid-env tests, high-cardinality address stress tests and concurrent rate-limit tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1955. `crates/horizon-relay/src/main.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a5f2dfe837aa9de11b9e68d8e5364d8b59e6fe5e`
- Size: 1624 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Executable entry point for the optional Horizon TCP friend relay.
- Dependencies / cross-references: horizon_relay::Config and Relay, TcpListener, PORT and RELAY_BIND environment variables.
- Implementation / logic: Parses the port, binds a configured address or dual-stack wildcard with IPv4 fallback, constructs relay config and serves until stopped while printing counts every five minutes.
- Findings: Default bind is externally reachable on wildcard interfaces; operators need firewall and trusted-proxy guidance. Configuration validation belongs before serving.
- RustChunk relevance: Standalone server binary, not Android launcher/runtime logic.
- Decision: unnecessary; relay executable; server-only
- 1.20.1 implications: This optional relay subsystem does not implement Forge 1.20.1 client protocol or mod negotiation.
- Follow-up checks: Validate port/config values, document secure bind defaults, add graceful shutdown handling and deployment/network exposure tests.
- RustChunk implementation mapping: Separate optional RustChunk relay executable.
- Evidence required before adoption: Bind-scope tests, graceful SIGTERM test, config-validation tests and deployment checks.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1956. `crates/horizon-relay/tests/tunnel.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e59f48d52ae071549b1f7df90d7d4ee473e6ca44`
- Size: 15034 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Integration tests for direct and relayed friend-session tunnels.
- Dependencies / cross-references: horizon-friends host/guest APIs, horizon-relay local server, TCP echo test server and loopback networking.
- Implementation / logic: Exercises direct path, relay path, fallback, wrong secrets, unknown and ended codes, version mismatch, attach timeout, bandwidth cap, health/whoami and connection/first-line limits.
- Findings: Provides broad end-to-end regression coverage for the optional relay. Tests were inspected only and not executed; no dedicated concurrent max_bridges race or malformed-rate config stress case was observed.
- RustChunk relevance: No proof of Forge 1.20.1 protocol or Aternos compatibility.
- Decision: reference; integration tests only; not executed
- 1.20.1 implications: This optional relay subsystem does not implement Forge 1.20.1 client protocol or mod negotiation.
- Follow-up checks: Add concurrency and invalid configuration tests plus fuzz malformed relay messages and shutdown races; run only in a dedicated test environment.
- RustChunk implementation mapping: Regression suite for a separately implemented RustChunk relay; do not mix into Minecraft client protocol tests.
- Evidence required before adoption: Executed test output, race/stress/fuzz results and cross-implementation relay interoperability.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1957. `crates/horizon-release-sign/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0585adcf83c4b8b3d82548f3fe9056541f7ddc8d`
- Size: 430 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Cargo manifest for release signing and checksum verification utilities.
- Dependencies / cross-references: Workspace lint/edition/license settings, ring 0.17 and base64 0.22; declared minimum Rust 1.85.
- Implementation / logic: Defines the horizon-release-sign crate and its Ed25519/SHA-256 support dependencies.
- Findings: Small host-side integrity tool dependency set; not part of Minecraft world or network runtime.
- RustChunk relevance: Can inform secure updater architecture but does not provide game version/mod compatibility.
- Decision: reference; release tooling; cryptographic dependency
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Confirm dependency policy, supported Rust toolchain and platform builds before adding analogous signing support.
- RustChunk implementation mapping: RustChunk release-integrity crate or build tooling only.
- Evidence required before adoption: Pinned dependency audit, cross-platform build tests and key rotation/verification fixtures.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1958. `crates/horizon-release-sign/legacy-releases.txt`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a0a71182ce10f4a34cdb436e8ab5098bdd729d2a`
- Size: 45772 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Allowlist of SHA-256 digests for release archives published before signed checksum manifests were introduced.
- Dependencies / cross-references: Consumed by horizon-release-sign legacy_digest and release updater flows; generated by tools/ci/legacy_releases.py.
- Implementation / logic: Stores `<tag> <asset> <sha256>` records for historical releases and installer archives; changes require code-owner review.
- Findings: The inspected 427-line file has 423 data rows. Both inspected ranges contained valid 64-hex SHA values and no repeated tag/asset keys; no malformed rows were found.
- RustChunk relevance: Release metadata only; not a Minecraft runtime feature.
- Decision: reference; release integrity data; legacy allowlist
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Recheck all rows as one dataset for global uniqueness and source provenance; ensure legacy hashes cannot be bypassed or silently replaced.
- RustChunk implementation mapping: Optional pinned legacy digest table in RustChunk updater, if rollback support is needed.
- Evidence required before adoption: Independent digest provenance, global uniqueness check, signed-updater integration tests and controlled rollback tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1959. `crates/horizon-release-sign/release-keys.txt`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c39be4435eeaa1740ec269a47fad72d2046f9df2`
- Size: 436 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Trusted public Ed25519 key list used by release verification.
- Dependencies / cross-references: Included by horizon-release-sign at compile time; private signing keys live in release environment secrets.
- Implementation / logic: Contains comments and one Base64-encoded public key; parser requires a 32-byte Ed25519 public key.
- Findings: No private signing key is present in this file. Changes are designated for code-owner review.
- RustChunk relevance: Potential trust-anchor pattern for RustChunk release verification, not game runtime.
- Decision: reference; public key material; security-sensitive
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Validate the decoded key length and key rotation process; document revocation/recovery and require protected review for trust-anchor changes.
- RustChunk implementation mapping: RustChunk updater trust store or embedded public-key resource.
- Evidence required before adoption: Key lifecycle policy, signature verification tests, rotation/revocation tests and protected review evidence.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1960. `crates/horizon-release-sign/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `45c72f09c08a33bcaf4877f674e2a69c1b229ba4`
- Size: 20572 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Ed25519-signed SHA256SUMS verification, nightly/release channel separation, legacy digest lookup and streaming SHA-256 helpers.
- Dependencies / cross-references: ring Ed25519 and SHA-256, base64, release-keys.txt and legacy-releases.txt.
- Implementation / logic: Signs/verifies a message containing release tag plus checksum bytes; accepts a valid trusted signature line, parses checksum rows, compares hashes and generates PKCS#8 signing keys.
- Findings: Fail-closed behavior when no trusted key exists and tag binding are positive. Checksum parser does not reject duplicate names or unsafe path syntax; callers must constrain paths.
- RustChunk relevance: Version-independent release integrity pattern; no Minecraft packet or mod-loader behavior.
- Decision: adaptable; secure update verification; version-independent
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Validate checksum path segments and duplicates at consumers; impose metadata-size limits; test malformed signatures, key rotation, replay and channel separation.
- RustChunk implementation mapping: Dedicated RustChunk update-verification module, kept separate from game protocol.
- Evidence required before adoption: Executed cryptographic test vectors, fuzzed parsers, key rotation/revocation tests and end-to-end download hash checks.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1961. `crates/horizon-release-sign/src/main.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `108b5192a6ea0d586f59df0df1d478eb852c6aa5`
- Size: 5533 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: CLI for verifying a release folder and generating Ed25519 signing keys.
- Dependencies / cross-references: horizon-release-sign library, filesystem, streaming SHA-256 and OS stdout terminal detection.
- Implementation / logic: Reads signed sums/signature, verifies every listed local archive in 64KiB chunks, and writes public key separately while sending private key bytes only to stdout pipe.
- Findings: Keygen explicitly refuses to write a private key to an interactive terminal. `folder.join(name)` trusts checksum asset names; absolute paths or `..` need rejection before file access.
- RustChunk relevance: Host-side release tooling only; not a mobile game launcher feature.
- Decision: adaptable; release CLI tooling; security-sensitive
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Canonicalize/validate each asset as a single safe relative filename, reject duplicates and symlinks as needed, cap sums/signature size, and tighten CLI argument validation.
- RustChunk implementation mapping: Optional RustChunk CI release-signing CLI, not Android runtime.
- Evidence required before adoption: Traversal/symlink/path fuzz tests, secret-output tests, CI integration and reproducible release verification.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1962. `crates/horizon-release-sign/tests/cli.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `12673f87758295491a95a611f6826437387995a8`
- Size: 6207 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: CLI integration tests for release signing, verification, tamper detection, key rotation and key generation.
- Dependencies / cross-references: horizon-release-sign APIs, compiled CLI executable, bash/OpenSSL 3 and temporary directories.
- Implementation / logic: Creates archives/checksum manifests, signs with test keys or the signing shell script, verifies success, rejects tampering and checks secret output behavior.
- Findings: Test cases cover the happy path and cryptographic tampering but do not exercise malicious checksum filenames/path traversal. Tests were inspected only, not run.
- RustChunk relevance: Test-only release tooling reference; no Minecraft 1.20.1 runtime relevance.
- Decision: reference; CLI integration tests only; not executed
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Add absolute-path, parent-component, symlink, duplicate checksum and oversized manifest fixtures; run on Linux/macOS/Windows CI.
- RustChunk implementation mapping: Tests for a future RustChunk release-integrity tool.
- Evidence required before adoption: Executed CI results and regression tests for all path/manifest edge cases.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1963. `crates/horizon-server-files/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2643fc726bbebc3691ccbaf11aa626e1284fca82`
- Size: 680 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Cargo manifest for dedicated-server file and settings compatibility helpers.
- Dependencies / cross-references: horizon-core, horizon-protocol, horizon-world, log, base64 and ureq TLS; minimum Rust 1.90.
- Implementation / logic: Defines server-side Java-properties, JSON, player profile lookup, user cache and user-list functionality.
- Findings: The crate description and module roles explicitly target the dedicated server, not the Android launcher/game client.
- RustChunk relevance: Mostly outside RustChunk mobile client scope; selected serialization semantics may serve as reference.
- Decision: unnecessary; dedicated-server only; host-side dependency
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Only reuse isolated utilities after validating version alignment, dependencies, licensing and resource bounds.
- RustChunk implementation mapping: No direct runtime module; extract targeted pure compatibility utilities only if justified.
- Evidence required before adoption: Version-matched 1.20.1 parity fixtures and platform/resource testing.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1964. `crates/horizon-server-files/src/eula.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `769af94012a4eb6b9d1706bfc2bde959aeb18cbf`
- Size: 1512 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Read/write behavior for the dedicated server's eula.txt.
- Dependencies / cross-references: Local java_properties parser/store and timestamp helpers; Minecraft EULA URL.
- Implementation / logic: Writes default `eula=false`; reads EULA property case-insensitively; missing/unreadable files are replaced with default false and reported as errors.
- Findings: Consent is explicit and defaults to not agreed; write failure is ignored when replacing an unreadable/missing file.
- RustChunk relevance: Legal-consent pattern only; not launcher or Forge implementation.
- Decision: reference; server legal-consent handling; version-specific
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Keep agreement explicit, avoid implying acceptance, and test missing, malformed, read-only and write-failure behavior.
- RustChunk implementation mapping: No direct client module; use only as a legal-consent handling reference if required.
- Evidence required before adoption: Version-matched behavior comparison and tests ensuring no implicit acceptance.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1965. `crates/horizon-server-files/src/java_properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `794e751cbe594b02c8707f52c47f019c5d92d4b6`
- Size: 15177 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Compatibility implementation of Java Properties load/store, including continuation lines, escapes, Latin-1 and key ordering.
- Dependencies / cross-references: Rust standard library collections, UTF-16 encoding/decoding helpers and date conversion functions.
- Implementation / logic: Parses Java-style property lines, resolves unicode escapes, stores keys in UTF-16 order and formats timestamp comments similar to java.util.Date.
- Findings: The code contains custom surrogate/NUL handling; isolated surrogate replacement and NUL marker interactions need differential checks against supported JDKs. Existing tests cover only a small set of examples.
- RustChunk relevance: Potential utility for Java launcher metadata/config files, but exact behavior requires proof against the 1.20.1 runtime's Java version.
- Decision: adaptable; Java-properties compatibility; parity-sensitive
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Build a broad JVM differential corpus for trailing slash, continuations, malformed unicode, surrogate pairs/isolates, NUL, comments and platform newlines.
- RustChunk implementation mapping: Optional RustChunk Java-properties compatibility module, isolated from renderer/protocol.
- Evidence required before adoption: Automated differential test output against supported JDK versions on Android and desktop.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1966. `crates/horizon-server-files/src/json.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `47fe6d5d3ae8a5f442bc8a664d114449a43652b6`
- Size: 14180 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Ordered JSON tree and strict parser/writers aiming to match Gson behavior and preserve numeric token text.
- Dependencies / cross-references: Rust standard library vectors/strings and local Json value model.
- Implementation / logic: Parses JSON values, preserves object member order and number spelling, writes compact and pretty forms, and emits HTML-safe Gson strings.
- Findings: No explicit input-byte or nesting-depth bound is visible; recursive parsing/writing can cause resource exhaustion on deeply nested or huge input. Only a small round-trip test exists.
- RustChunk relevance: May support metadata compatibility, but it is not MC 1.20.1 network protocol implementation.
- Decision: adaptable; JSON compatibility utility; security-sensitive
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Add size/depth/element bounds, duplicate-key and unicode edge-case tests, and differential tests against the exact Gson version.
- RustChunk implementation mapping: Bounded compatibility JSON utility in RustChunk only if target fixtures require it.
- Evidence required before adoption: Fuzzing, memory/stack-limit tests and byte-for-byte Gson differential corpus.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1967. `crates/horizon-server-files/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `08c059905b44d7c26bee545ff032acb3a121a258`
- Size: 526 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Module root exporting dedicated-server EULA, Java-properties, JSON, profiles, settings, cache and user-list modules.
- Dependencies / cross-references: The sibling modules within horizon-server-files; game server consumes them and horizon-server manages them.
- Implementation / logic: Declares and publicly exports seven dedicated server helper modules.
- Findings: The module documentation says the dedicated server reads them and this crate needs neither a window nor the game.
- RustChunk relevance: No direct Android client or launcher functionality.
- Decision: unnecessary; server-only module exports
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Do not port the whole crate to RustChunk client; extract only independently justified compatibility logic.
- RustChunk implementation mapping: No direct mapping to launcher/game executable.
- Evidence required before adoption: A specific client-side requirement and version-matched isolated tests before extracting any helper.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1968. `crates/horizon-server-files/src/profiles.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2c6611c875142079a4cfae01fa3942ca615d13a8`
- Size: 8200 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Resolve player names/UUIDs and fetch Minecraft Services profile-certificate public keys for a dedicated server.
- Dependencies / cross-references: Minecraft Services discovery document/endpoints, ureq TLS, local JSON parser, Base64 and RSA public key parser.
- Implementation / logic: Looks up profile by name or UUID through discovered endpoints and decodes profilePropertyKeys/playerCertificateKeys into RSA public keys.
- Findings: HTTP responses are read fully without explicit byte limits; discovery and returned endpoint URLs lack a visible host allowlist. Empty player name passes `valid_player_name`'s current length/ASCII predicate and needs comparison with the target Java behavior.
- RustChunk relevance: Server account lookup and secure-chat trust data, not Forge client mod negotiation or Aternos routing.
- Decision: reference; server account/profile lookup; network-sensitive
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Bound response sizes, enforce HTTPS and allowed service origins, validate redirects/endpoints and test player-name boundaries against vanilla.
- RustChunk implementation mapping: Optional server-side profile resolver only; do not include in Android client runtime by default.
- Evidence required before adoption: Network mock tests, endpoint allowlist/redirect tests, size-limit tests and 1.20.1 profile lookup parity.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1969. `crates/horizon-server-files/src/properties.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `849e2a54189320e0adadc72c6159bd952d4ba9cf`
- Size: 24332 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Large parser/normalizer for modern dedicated-server server.properties and generator/resource-pack settings.
- Dependencies / cross-references: Local Java Properties and JSON modules, Java string hash, MD5/UUID helpers and entropy provider.
- Implementation / logic: Reads defaults and values for online-mode, permissions, view/simulation distance, management server, world seed/generator, resource pack and other properties; normalizes them back to Properties.
- Findings: Uses a /dev/urandom attempt then falls back to clock-seeded SplitMix64 for management secret generation, which is not a CSPRNG; its setting set is version-specific to the pinned newer server.
- RustChunk relevance: Do not import settings as 1.20.1 truth; derive properties from Forge/Minecraft 1.20.1 target classes. Never reuse weak secret fallback.
- Decision: reference; server settings schema; security-sensitive; version-specific
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Replace with cross-platform OS CSPRNG that fails closed; validate bounds and compare every property/default/serializer with 1.20.1 server classes.
- RustChunk implementation mapping: Version-specific server settings reference only; RustChunk client may only reuse unrelated parsing primitives after tests.
- Evidence required before adoption: CSPRNG assurance and deterministic 1.20.1 parity fixtures for every supported property.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1970. `crates/horizon-server-files/src/user_cache.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2e974637bbd513b24f7963a98d3c1d494b7fd981`
- Size: 11888 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Persistent usercache.json mapping player UUIDs/names to one-month expiration metadata.
- Dependencies / cross-references: Java date conversion, local Gson-like JSON, profile name checks and user list NameAndId type.
- Implementation / logic: Loads cache entries, resolves case-insensitive names/UUIDs, expires name lookups, and serializes most-recently-used entries capped at 1000.
- Findings: The serialized file is capped to 1000 records, but internal `infos`/maps are not evicted and can grow over time; file and JSON parsing have no clear size/depth cap.
- RustChunk relevance: Dedicated-server player cache only; no renderer/chunk loading or mod compatibility.
- Decision: adaptable; user-cache behavior; memory-sensitive
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Implement actual LRU eviction from memory, cap input size/depth and test duplicates, expiration, corrupted files and growth under repeated adds.
- RustChunk implementation mapping: Optional bounded profile cache if RustChunk later provides server tooling; not game client core.
- Evidence required before adoption: Long-running memory tests, file-size adversarial tests and comparison with vanilla cache behavior.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1971. `crates/horizon-server-files/src/user_lists.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0c80fcef869f06f35032f91b456f02999e4373f0`
- Size: 17424 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Stored ops, whitelist, player-ban and IP-ban list handling for the dedicated server.
- Dependencies / cross-references: Local JSON and Java-properties date helpers, UUID conversion, Java string hashing and JavaHashMap from horizon-world.
- Implementation / logic: Loads/serializes list entries, maintains Java HashMap iteration semantics, expires bans and emits watcher change notifications for add/remove/clear.
- Findings: Reads and parses unbounded JSON; `load_str` permits trailing content after the first value; file writes are direct/non-atomic. Date parsing needs extreme-number overflow tests.
- RustChunk relevance: Server administration only; may inform portable account-list serialization but not Minecraft public-server client connectivity.
- Decision: adaptable; access-list persistence; compatibility-sensitive
- 1.20.1 implications: These files are dedicated-server support and modern server compatibility references; derive behavior from the exact 1.20.1 target instead of copying newer assumptions.
- Follow-up checks: Use strict whole-document parsing, input bounds and atomic temp-write/rename; fuzz date fields, malformed entries and map ordering.
- RustChunk implementation mapping: Optional standalone administrative list utility, isolated from Android gameplay.
- Evidence required before adoption: Parity fixtures against target version, corruption recovery, atomic-write interruption tests and parser fuzzing.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1972. `crates/horizon-server/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2b289e057d04e07e021bbcbf565ef07befe24df2`
- Size: 2694 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Cargo manifest for Horizon's dedicated server supervisor/management application.
- Dependencies / cross-references: horizon-server-files, MCP, anticheat, chat-filter, benchmark, release-signing, TLS, WebSocket, TOML, PNG and ZIP packages; minimum Rust 1.90.
- Implementation / logic: Declares the control-plane executable that configures, launches, monitors, updates and backs up the headless game server.
- Findings: The dependency graph is dedicated-server management infrastructure and includes many packages irrelevant to a mobile Minecraft client.
- RustChunk relevance: No direct Forge 1.20.1 launcher or Android rendering implementation.
- Decision: unnecessary; dedicated-server supervisor; server-only
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Keep server manager separate; extract shared crates only after auditing their independent client need.
- RustChunk implementation mapping: No direct runtime mapping; optional RustChunk server-management companion only.
- Evidence required before adoption: A distinct server-management product requirement and server deployment/integration tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1973. `crates/horizon-server/panel/index.html`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5d77cb9561c09c67b27d8cf929ff45e1e421826a`
- Size: 2979 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Static HTML structure for Horizon's server administration dashboard.
- Dependencies / cross-references: panel.css, panel.js and `/api/v1` server management routes.
- Implementation / logic: Defines status/actions, performance charts, player table, backup controls, console and signed-out/down states.
- Findings: It is presentation markup; authentication and authorization must be enforced by the backend, not assumed from this file.
- RustChunk relevance: Server dashboard only; no Android launcher or game-engine support.
- Decision: reference; admin panel markup; server-only
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Review accessibility, CSRF/session design, safe linking and route authorization in associated backend handlers.
- RustChunk implementation mapping: Optional admin companion UI only if RustChunk later needs remote server management.
- Evidence required before adoption: Browser accessibility checks, route authorization tests and end-to-end authenticated session tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1974. `crates/horizon-server/panel/panel.css`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `994f5eac6dabf15015f5fe36f0bbbfe6240bf19e`
- Size: 4491 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Responsive style sheet and light/dark theme for the Horizon server panel.
- Dependencies / cross-references: Panel HTML class/id structure and CSS theme variables.
- Implementation / logic: Defines layout, responsive columns, charts, tables, forms, buttons, notices and dark-mode tokens.
- Findings: Pure presentation layer with no engine, persistence or authentication behavior.
- RustChunk relevance: Not relevant to Minecraft Java runtime or Android launcher logic.
- Decision: unnecessary; presentation-only CSS; server-only
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Check mobile layout, contrast, keyboard focus and CSS feature compatibility if reusing the panel.
- RustChunk implementation mapping: No direct gameplay mapping; optional web admin styling reference.
- Evidence required before adoption: Visual responsive tests, contrast audit and supported-browser checks.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1975. `crates/horizon-server/panel/panel.js`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5405907ca90d30146dbd38a3bd1e28b02c3881e8`
- Size: 10282 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Browser-side logic for the Horizon server admin dashboard.
- Dependencies / cross-references: Panel DOM, same-origin `/api/v1` JSON endpoints and authenticated session cookie/header contract.
- Implementation / logic: Polls status/metrics/logs, draws TPS/MSPT/RAM/CPU charts, renders players/backups, and submits command, restart, stop, restore and update actions.
- Findings: Remote strings are placed via textContent, reducing DOM injection risk. A one-second setInterval can overlap slow polls; a failure in Promise.all can mark the whole panel gone even if some routes respond. Server-side auth/CSRF validation is not proven by this file.
- RustChunk relevance: Admin frontend only; not launcher or Forge game runtime.
- Decision: adaptable; admin UI safety pattern; server-only
- 1.20.1 implications: This file is not evidence of Minecraft 1.20.1 runtime or mod compatibility.
- Follow-up checks: Add in-flight poll guard/backoff, independent handling for optional endpoints, CSRF/session review, backend authorization tests and DOM injection tests.
- RustChunk implementation mapping: Optional RustChunk admin frontend; keep entirely separate from client gameplay.
- Evidence required before adoption: Browser tests for XSS, CSRF, overlapping polls, session expiry and every management action.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1976. `crates/horizon-server/src/admin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `316db518a08b16dac6d2204af5a4e846d6da8bbe`
- Size: 16333 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements the dedicated-server management API for kicking/banning/pardoning players, operator and allowlist changes, and Prometheus metrics.
- Dependencies / cross-references: horizon-server API router and Shared state; server user-list files, UUID/profile lookup, metrics and game console/control endpoint.
- Implementation / logic: Parses JSON actions, validates names/reasons/IP syntax at the route layer, serializes mutations, forwards vanilla console commands and emits response/metrics data.
- Findings: The moderation API is server control-plane code, not a launcher or game client. Its player-name predicate accepts arbitrary non-ASCII non-space/control characters; confirm exact vanilla 26.3 constraints. Request handling depends on authenticated API routing.
- RustChunk relevance: Shows structured moderation routing and metrics, but not Android launcher or Minecraft client functionality.
- Decision: reference; dedicated-server admin API; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Test names against vanilla 26.3; enforce exact player/IP validation; test concurrent moderation and failure recovery; bound metric labels and all text fields.
- RustChunk implementation mapping: No direct gameplay mapping; a separate optional RustChunk server-admin companion could adapt typed moderation routes.
- Evidence required before adoption: Version-matched vanilla command/API parity, adversarial input tests, auth tests and concurrent mutation tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1977. `crates/horizon-server/src/api.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3c1417da25e93693133ca6833bef2cd123b6578`
- Size: 44281 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements the Horizon dedicated-server supervisor's authenticated HTTP control API, shared state, metrics, logs, backups, restore, update and panel login routes.
- Dependencies / cross-references: horizon-server state, minimal HTTP transport, config/settings, backup, engine, process/supervisor, authentication token and panel modules.
- Implementation / logic: Defines API route metadata and dispatch; reads/writes private API token metadata; authenticates Bearer requests and validates Host for private binds; exposes typed status/player/log/event/config/update/backup operations.
- Findings: Useful patterns include explicit route table, token comparison and route metadata. `write_private` applies Unix 0600 only when creating a file and does not repair permissions on an existing one; writes are direct rather than atomic. Public bind is possible by configuration and then relies on bearer-token security.
- RustChunk relevance: Potential architectural reference for a future admin companion; no proof of Forge 1.20.1 gameplay support.
- Decision: adaptable; authenticated control API patterns; server tooling
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Repair existing file permissions; use atomic private-file replacement; enforce TLS or a trusted secure tunnel for non-loopback binds; audit every route's auth/CSRF and input/path limits.
- RustChunk implementation mapping: Only isolated authenticated control-plane patterns for a separate RustChunk companion; keep out of gameplay process.
- Evidence required before adoption: Route-by-route authorization tests, permissions/atomic-write tests, network TLS verification, path traversal tests and API fuzzing.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1978. `crates/horizon-server/src/backup.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c446d56ca7cc593bd6bb58e3889dc29e0d078a43`
- Size: 12540 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Creates, lists, prunes, deletes and restores ZIP backups of the dedicated server's world and selected configuration/player-list files.
- Dependencies / cross-references: zip archive writer/reader, filesystem paths, server config, world level.dat validation, backup directory and supervisor save/stop coordination.
- Implementation / logic: Writes archives through a `.part` file then renames; recursively adds the world while skipping session.lock; restore extracts into a staging directory, validates level.dat, moves the old world aside and installs the restored world.
- Findings: Staging and preserving the old world are positive. Recursive `path.is_dir()` follows symlinked directories and can escape the intended root; restore has no explicit decompressed-size/file-count ceiling and its two renames are not a fully transactional swap. The duration parser uses a byte index derived from string length and can panic for multibyte suffixes.
- RustChunk relevance: Atomic staging and recovery concepts may transfer to launcher instance backup, but server-specific paths and risks need a rewrite.
- Decision: adaptable; backup workflow reference; security-sensitive
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Use symlink-safe traversal rooted at canonical server dir; cap archive entries and total expanded bytes; validate every ZIP path; make restore rollback-safe; fix duration parsing using char boundaries and checked arithmetic.
- RustChunk implementation mapping: Reusable only as a separately designed backup subsystem; do not copy into mobile client.
- Evidence required before adoption: Symlink/path traversal fixtures, ZIP bomb tests, interrupted restore recovery, large-world memory/disk benchmarks and duration parser fuzzing.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1979. `crates/horizon-server/src/cli.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `47cd6d00da5cde503d30b0335bbc6fabba48734f`
- Size: 23145 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Provides command-line operations for launching/stopping/status/report/logs/console/backups/restore/player management and moderation of a managed dedicated server.
- Dependencies / cross-references: API Client, server supervisor lock/state, backup module, process spawning, logs, engine control and interactive terminal I/O.
- Implementation / logic: Starts a detached supervisor, waits for status, issues API requests, tails or displays logs, asks for confirmation before restore and wraps player/config operations.
- Findings: Startup wait timeout does not necessarily terminate the spawned server process; offline log display reads the complete latest.log into memory before selecting lines. CLI behavior is server administration and assumes the target Horizon version.
- RustChunk relevance: Useful operational UX ideas, but not necessary for the RustChunk mobile launcher core.
- Decision: reference; server CLI; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Terminate or track child on timeout; tail logs with bounded memory and a maximum line count; test process orphaning, shutdown and terminal/nonterminal behavior.
- RustChunk implementation mapping: Could inspire separate RustChunk CLI/diagnostic tool, not Android game runtime.
- Evidence required before adoption: Process lifecycle tests on Windows/Linux/macOS, huge-log tests, timeout/crash recovery and CLI integration outputs.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1980. `crates/horizon-server/src/config.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `10c8c28a77d643db3f30960b288cb7112d6a589f`
- Size: 32901 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Defines, parses, validates, persists and imports the dedicated server's typed server.toml and related vanilla server.properties settings.
- Dependencies / cross-references: TOML, Java Properties compatibility helpers, user-list/server settings modules, server directory paths and file I/O.
- Implementation / logic: Provides default template, Config structs, typed property mapping, validation, config load/update/import and generation of server.properties before server start.
- Findings: The template and fixed version assumptions target Minecraft Java 26.3, not 1.20.1. World-name validation rejects separators and `..` but permits `.`, which can resolve the world directory to the server root. Config reads and writes have no explicit file-size cap or atomic replacement.
- RustChunk relevance: Useful validation design pattern but not suitable as a drop-in 1.20.1 configuration schema.
- Decision: reference; version-specific server config; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Reject dot/dot-dot and platform-reserved names; canonicalize world paths under server root; cap TOML size; use atomic writes; derive exact defaults and keys from vanilla/Forge 1.20.1.
- RustChunk implementation mapping: No direct launcher config mapping; only generic typed-config/validation ideas after a new platform schema is defined.
- Evidence required before adoption: 1.20.1/Forge parity fixtures for every key/default, path traversal tests, atomic-write interruption tests and config fuzzing.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1981. `crates/horizon-server/src/console.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9d948e815b40b600d4ea83da5d65a98b492c2e2a`
- Size: 5523 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements an interactive terminal console attached to the running dedicated-server supervisor.
- Dependencies / cross-references: Server event stream, terminal detection/alternate screen, API command route, player/status display and log events.
- Implementation / logic: Shows recent logs and periodically refreshed status, streams new events and sends typed console commands; Ctrl-D detaches without stopping the game.
- Findings: Separation between detaching a console and stopping the server is clear. Multiple display/update sources may interleave terminal output; behavior is host CLI only.
- RustChunk relevance: May inform developer diagnostics but does not address Minecraft 1.20.1 game performance or networking.
- Decision: reference; terminal management UX; host-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Test terminal resize, non-TTY input/output, concurrent events, dropped connections and graceful detach.
- RustChunk implementation mapping: Optional desktop diagnostic console only; exclude from Android game runtime.
- Evidence required before adoption: PTY integration tests across supported terminals and process shutdown/detach tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1982. `crates/horizon-server/src/discord.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `949ea1c2a63693e3713ae975007a7d438e14fbec`
- Size: 25335 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Sends dedicated-server chat, join/leave, death, kick and announcement messages to a Discord webhook, including a rendered chat image.
- Dependencies / cross-references: Webhook HTTP client, event/log stream, pixel/font rendering, queue/batching, rate-limit handling and Discord webhook API.
- Implementation / logic: Batches events, renders chat text to an image, escapes mentions/Markdown, disables allowed_mentions and handles retry-after/backoff.
- Findings: Mention suppression and filtered-chat handling are good. Event text is rendered before a strict input-size cap, allowing large input to consume memory/CPU. A non-finite Retry-After value such as NaN must be rejected before converting to Duration to avoid panic; failed sends may drop queued messages.
- RustChunk relevance: Not needed for Minecraft client/Forge compatibility; only relevant if RustChunk later includes server operations.
- Decision: unnecessary; optional Discord server integration; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Cap event bytes/codepoints before image rendering; validate finite Retry-After; define bounded retry queue and drop policy; test webhook outages, malformed headers and large text.
- RustChunk implementation mapping: Optional server-side Discord integration only, isolated from Android client.
- Evidence required before adoption: Webhook mock tests, memory/CPU stress tests, malformed Retry-After fuzzing and delivery/retry policy tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1983. `crates/horizon-server/src/engine.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f358ac94ded89dfe510e6191a2664d64f1cc6f88`
- Size: 22989 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Locates and launches Horizon's headless dedicated-server engine and its prepared game resources, with a private TCP control endpoint.
- Dependencies / cross-references: Horizon engine/client binaries, prepared-resource manifest, JVM/native launcher environment, local control socket/port and server process.
- Implementation / logic: Finds executable and resources, verifies prepared manifests, starts game with server args and serves/uses a loopback control endpoint authenticated by a token.
- Findings: Resource discovery hardcodes Minecraft 26.3 locations, not 1.20.1. `check_prepared` skips a manifest hash check when its key is absent, so incomplete manifests may pass some checks. Control metadata/replies lack visible size ceilings.
- RustChunk relevance: The process/control patterns may inform tooling but the executable/resource assumptions cannot be ported directly.
- Decision: reference; headless server engine; version-specific
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Parameterize and version-pin resource layout; require all mandatory manifest hashes; bound control metadata, frame and reply sizes; test process exit, timeout and token handling.
- RustChunk implementation mapping: Generic child-process supervision/control channel ideas only; create a distinct Android client/game launch path.
- Evidence required before adoption: Version-matched 1.20.1 resource manifests, strict malformed-manifest tests, process lifecycle tests and local IPC fuzzing.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1984. `crates/horizon-server/src/events.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3445ea40b9084352a9d1921586cab0868a75d51d`
- Size: 10414 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Maintains numbered server/log/game events and exposes them through JSON polling or Server-Sent Events for dashboard clients.
- Dependencies / cross-references: Shared server state, engine control endpoint, event queue, HTTP response streaming and management notifications.
- Implementation / logic: Keeps the last 2,000 events, polls game typed events every 250ms, supports since/type filters, Last-Event-ID reconnection and 15-second keepalive.
- Findings: Bounded event count and replay cursor are useful. Individual event payload sizes are not capped; streaming clients can hold worker threads while writes block, with no clear client cap or write timeout.
- RustChunk relevance: Potentially reusable for RustChunk diagnostics, not core gameplay or Forge mod compatibility.
- Decision: adaptable; event stream pattern; server tooling
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Bound serialized event bytes, apply backpressure/slow-client disconnects, cap SSE clients, add write timeouts and test reconnect/overflow behavior.
- RustChunk implementation mapping: A bounded event bus may be adapted for launcher logs/diagnostics, but replace server-specific event types.
- Evidence required before adoption: Large-event and slow-client tests, reconnect-after-overflow tests and concurrency/resource stress results.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1985. `crates/horizon-server/src/http.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d01580356572b3de581df212152ca4e8d6148da`
- Size: 11940 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements the small HTTP/1.1 transport used by the dedicated-server control API and panel.
- Dependencies / cross-references: Rust TCP streams, std I/O, JSON response encoding, API route handler and WebSocket upgrade helpers.
- Implementation / logic: Reads one request per connection, caps line length at 8KiB and body size at 1MiB, times out reads after 10 seconds, writes fixed or streamed responses, and spawns a thread per connection.
- Findings: The per-line/body limits are positive. Header count guard allows 65 rather than the apparent 64; connections/threads are unbounded; response reads in the client helper are unbounded. Most importantly, the API itself is plain HTTP, so remote bearer tokens are exposed unless traffic is constrained to loopback or a trusted encrypted tunnel.
- RustChunk relevance: Limits and routing patterns may be informative, but its plaintext/unbounded connection architecture is unsuitable as-is.
- Decision: adaptable; minimal HTTP reference; security-sensitive
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Reject duplicate/conflicting Content-Length and Transfer-Encoding; cap connections/threads and response sizes; set write timeouts; enforce TLS for non-loopback traffic; test parser smuggling/fuzz cases.
- RustChunk implementation mapping: Do not reuse as public network transport; if needed, adapt bounded parsing and route separation in a separate authenticated/TLS control client.
- Evidence required before adoption: HTTP parser fuzzing, slowloris/concurrency tests, request-smuggling tests, TLS deployment tests and bounded-response tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1986. `crates/horizon-server/src/init.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `71d01277cfc646a1a5c93938d1dd03a86670b6fa`
- Size: 11627 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements first-run dedicated-server initialization: EULA, world, seed, port, online mode, whitelist, players/operators and resource discovery.
- Dependencies / cross-references: Server config/import, EULA file handling, user profile lookup, user lists, game engine discovery and terminal prompts.
- Implementation / logic: Supports command flags/environment variables or interactive prompts, imports existing server.properties, writes EULA false by default unless accepted, populates lists and writes server.toml.
- Findings: Explicit EULA consent is a positive safeguard. Initialization mutates multiple files before final server.toml publication, so a later failure can leave partial state; world and port inputs should be validated before mutations.
- RustChunk relevance: Useful consent and wizard sequencing ideas, but not a client-launcher implementation.
- Decision: reference; dedicated-server init flow; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Prevalidate all answers and canonicalized world path; stage and atomically publish changes; test failure at every write/network/engine step; align profile validation with 1.20.1.
- RustChunk implementation mapping: Not part of launcher; setup wizard patterns may inform future RustChunk instance setup.
- Evidence required before adoption: Failure-injection tests, EULA consent tests, filesystem rollback tests and version-matched account/profile tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1987. `crates/horizon-server/src/lists.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f5c695e2244f18897e0ac57e65e4b9a11882ff58`
- Size: 9002 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements op/deop and whitelist commands by editing vanilla JSON lists while stopped or delegating to the live game.
- Dependencies / cross-references: Server config, supervisor directory lock, profile resolver, user-list JSON handling and console/API command path.
- Implementation / logic: Uses a lock to prevent the server starting during file edits; while running forwards commands, with a special offline whitelist add path that updates the JSON file and reloads it.
- Findings: Clear separation between live server ownership and offline file mutation. Profile resolution and list format are tied to Horizon's newer server data; offline whitelist special case should be parity-tested.
- RustChunk relevance: Potentially useful format/locking concepts, but not needed for joining Aternos or managing client accounts.
- Decision: reference; dedicated-server access lists; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Compare offline UUID/profile behavior against vanilla 1.20.1; test lock races and partial writes; make user-list file updates atomic and bounded.
- RustChunk implementation mapping: Optional account/list reference for server companion; do not confuse offline server UUID logic with launcher authentication.
- Evidence required before adoption: Vanilla 1.20.1 offline/online UUID parity, lock race tests and corrupt-list recovery tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1988. `crates/horizon-server/src/main.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d650cffd3efad51df5d9dd2c3fce3dc4b6b511d9`
- Size: 9603 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Defines the horizon-server executable entry point, help text, command routing and basic option parsing.
- Dependencies / cross-references: All horizon-server modules, process exit codes, filesystem paths and CLI option strings.
- Implementation / logic: Parses --dir, dispatches lifecycle, config, console, player moderation, backup/restore, service, panel, update and MCP commands.
- Findings: Help text explicitly identifies Minecraft Java Edition 26.3. Several subcommands use permissive ad-hoc argument checks; port is parsed as u16 and zero remains accepted until later configuration validation.
- RustChunk relevance: No direct launcher functionality; useful only as a command-line UX example.
- Decision: reference; server command dispatcher; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Use a typed CLI parser or strict per-command arg validation; reject invalid port early; test unknown/duplicate/missing options.
- RustChunk implementation mapping: Could inspire a separate RustChunk dev CLI, but not a dependency of the Android app.
- Evidence required before adoption: CLI parser tests and cross-platform invocation tests for every command/flag.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1989. `crates/horizon-server/src/management.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `40454bd83d7c2329e09dd7b4c78c9034c0e21d1e`
- Size: 43397 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements the dedicated server's Vanilla-compatible JSON-RPC 2.0 management protocol over WebSocket, optionally protected by TLS.
- Dependencies / cross-references: Management schema/method handlers, HTTP WebSocket upgrade, custom WebSocket socket, TLS certificates, server events, engine control and server properties.
- Implementation / logic: Creates TLS/server settings and credentials, accepts WebSocket clients, authenticates with bearer or subprotocol secret plus Origin checks, dispatches RPC and broadcasts notifications/heartbeats.
- Findings: TLS and Origin-aware authentication are useful security controls. Connection count is not capped and notification channel/outbox is unbounded; a slow client can stall the shared notifier. RPC behavior is explicitly pinned to Vanilla 26.3, not 1.20.1.
- RustChunk relevance: Not a substitute for Minecraft 1.20.1 client networking; only administrative protocol patterns can transfer.
- Decision: reference; server management WebSocket; version-specific
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Cap simultaneous clients and queued notification bytes; disconnect slow consumers; validate complete WebSocket handshake headers/frame limits; use strong fail-closed entropy and version-specific protocol fixtures.
- RustChunk implementation mapping: No direct client protocol mapping; optionally build a separate server-admin transport if RustChunk needs remote management.
- Evidence required before adoption: WebSocket/RPC fuzzing, slow-client/resource exhaustion tests, TLS/auth tests and exact 1.20.1 protocol comparison.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1990. `crates/horizon-server/src/management_methods.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7dd94ce6ab4f45bdc0f4eb8f1400878f95b3c13c`
- Size: 35634 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements the management protocol's method calls for players, server state, game rules, settings, allowlist, operators and ban lists.
- Dependencies / cross-references: management.rs dispatch/schema, engine control channel, server console, config persistence and horizon-server-files user-list/profile models.
- Implementation / logic: Converts JSON DTOs, forwards operations to the game control endpoint, persists selected changed settings and constructs player/ban/operator notifications.
- Findings: The schema lists a larger method surface than this dispatcher visibly handles; several advertised settings fall through to an unsupported-method error. IP DTO conversion stores input as strings without parsing an IP address; list loading ignores read/parse errors and can report empty lists.
- RustChunk relevance: Good example of schema-driven conversion, but its implementation is partial relative to its own schema and tied to 26.3.
- Decision: adaptable; typed server API conversion; version-specific
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Generate parity tests ensuring every advertised incoming schema method has a handler; parse/normalize IP addresses; surface corrupt list errors; validate UUID syntax and DTO bounds.
- RustChunk implementation mapping: Only typed API conversion patterns are reusable; do not port these server RPC methods into client protocol.
- Evidence required before adoption: Schema-to-handler exhaustive coverage, malformed DTO tests, corrupt list tests and version-matched vanilla parity.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1991. `crates/horizon-server/src/management_schema.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e0627c55300d97fcf894c362a8361aaa06991c38`
- Size: 16090 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Declares the management JSON-RPC OpenRPC 1.3.2 document, 89 methods and component schemas exposed by rpc.discover.
- Dependencies / cross-references: management.rs dispatch and methods, serde_json and Vanilla 26.3 management API definitions.
- Implementation / logic: Stores methods in vanilla order, builds JSON schema references and assembles the discover response with parameter/result descriptions.
- Findings: Static schema generation makes capabilities discoverable, but advertised methods should be mechanically compared with actual handlers; the implementation file audited immediately before it does not appear to handle every incoming settings method.
- RustChunk relevance: Schema-generation pattern only; all method names and DTOs need fresh 1.20.1 evidence before reuse.
- Decision: reference; management protocol schema; version-specific
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Add exhaustive method/handler parity checks, schema validation for required/optional fields and compare generated output with the exact vanilla 26.3 reference.
- RustChunk implementation mapping: Schema-first API design may be adapted for a separate management API; not for Minecraft game protocol.
- Evidence required before adoption: OpenRPC schema validation, all-method positive/negative integration tests and upstream-version parity artifact.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1992. `crates/horizon-server/src/management_tls.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d726d89f7cfdf13262b4de5d6706cf524bf912fc`
- Size: 6714 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Loads or generates the PKCS12 certificate/key store and rustls server configuration for the dedicated-server management protocol.
- Dependencies / cross-references: p12_keystore, rustls ring provider, rcgen, SHA-256 fingerprint, server config and SystemEntropy secret generator.
- Implementation / logic: Loads configured keystore, extracts a private key/certificate chain, computes fingerprint, builds TLS server config, or generates a self-signed localhost/host certificate.
- Findings: Certificate fingerprint pinning is useful. Keystore bytes are read wholly with no explicit size bound; configured path is joined directly to server dir and can escape it; generated password uses SystemEntropy with its previously noted weak time/PRNG fallback.
- RustChunk relevance: Generic TLS patterns may transfer, but management service and key format are not required for Minecraft client play.
- Decision: adaptable; TLS primitives; security-sensitive
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Bound keystore file size; enforce canonical path containment/permissions; replace entropy fallback with OS CSPRNG that fails closed; test SAN/hostname and certificate rotation.
- RustChunk implementation mapping: Potentially adapt generic rustls certificate loading for secure update/admin services; keep separate from game protocol.
- Evidence required before adoption: CSPRNG assurance, path traversal/oversize tests, TLS hostname/fingerprint tests and certificate rotation tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1993. `crates/horizon-server/src/mcp.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c1ac757387bf2c9d6e433044b7579fcd644476f6`
- Size: 19129 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Exposes a Model Context Protocol server over stdin/stdout so an agent can inspect and operate a Horizon dedicated server.
- Dependencies / cross-references: horizon_mcp JSON-RPC framing, local API Client, server commands, moderation/config/backup routes and serde_json.
- Implementation / logic: Advertises tools and argument schemas, maps tool calls to authenticated API HTTP requests and requires confirm=true for destructive backup/restore/stop operations.
- Findings: Explicit confirmation for destructive tools and separated protocol stdout are good. Tool inventory advertises some operations with broad destructive/read-only annotations that should be audited against actual effects; user input and returned API output need bounded sizes.
- RustChunk relevance: Not needed for the RustChunk launcher/game client.
- Decision: unnecessary; optional MCP server integration; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Validate all tool schemas against request mappings, refine annotations, cap line/response lengths, restrict arbitrary run_command policy and test malformed MCP frames.
- RustChunk implementation mapping: Do not include in mobile client core; optional separate management adapter only if expressly required.
- Evidence required before adoption: MCP conformance suite, malicious tool-argument tests, output-size limits and audited destructive-action confirmation matrix.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1994. `crates/horizon-server/src/panel.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b9bfd3e965df9ffe691440bd9f22a70d2f2dd31`
- Size: 13174 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Serves the static web administration panel and manages one-time login links, browser sessions, Host validation and request protection.
- Dependencies / cross-references: Embedded HTML/CSS/JS, HTTP server, API token helpers, shared panel auth state and server API routes.
- Implementation / logic: Redeems expiring codes into sessions, limits sessions and failed logins, checks Host, requires X-Horizon for non-GET actions, sets SameSite/HttpOnly cookie and CSP/security headers.
- Findings: One-time code and CSP/CSRF defenses are positive. Panel is served over HTTP and cookie lacks Secure; a non-loopback/public bind can expose login/session traffic. Login-link code list is only pruned on another link/login and can grow during repeated issuance.
- RustChunk relevance: Some browser hardening patterns can transfer to a separate companion UI, not to the launcher runtime.
- Decision: reference; web admin panel auth; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Require HTTPS for remote binds, Secure cookies on TLS, bound and prune pending codes at issuance, add session expiry, test DNS rebinding/Host behavior and CSRF/XSS.
- RustChunk implementation mapping: Optional external admin web UI; keep separated from the Android launcher.
- Evidence required before adoption: HTTPS/session tests, expired-code growth tests, CSRF/XSS and host-rebinding tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1995. `crates/horizon-server/src/process.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8e8d24c70fef5434e9b8812e052c843ffa4c3a5b`
- Size: 861 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Wraps process creation so Windows console child processes can be launched without opening a new console window when the parent has no console.
- Dependencies / cross-references: std::process::Command and windows-sys console/threading APIs behind cfg(windows).
- Implementation / logic: Creates Command and conditionally applies CREATE_NO_WINDOW based on GetConsoleWindow.
- Findings: Small platform-specific helper with narrowly scoped unsafe call; no game or network logic.
- RustChunk relevance: Irrelevant to Android launcher and Minecraft gameplay.
- Decision: unnecessary; Windows server process helper; host-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Compile-check on supported Windows targets and ensure the cfg-gated dependency remains optional for other platforms.
- RustChunk implementation mapping: No Android mapping; optional Windows host tooling helper.
- Evidence required before adoption: Windows process-spawn tests/builds and cross-target compilation.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1996. `crates/horizon-server/src/query.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `003ec4f913183d9edce46a20ec1f85528fd2b832`
- Size: 25709 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements Vanilla GS4 UDP Query handshake, challenge-token validation, basic/full status packets and cached player/server metadata.
- Dependencies / cross-references: UDP sockets, server.properties, ServerProperties parser, game control endpoint, Shared server phase and QueryThreadGs4 packet format.
- Implementation / logic: Binds query UDP port, issues per-source challenge tokens, caches full-stat bytes for five seconds, caches player info for one second and answers basic/full stat requests.
- Findings: Packet-size cap and challenge protocol are present. Challenge map is keyed by source address but has no cardinality ceiling and can grow under many spoofed/source addresses until prune; full-stat response size depends on player names/count without a response cap. Uses 24-bit challenge token as vanilla-style behavior.
- RustChunk relevance: Not required for the RustChunk launcher to join public/Aternos servers.
- Decision: reference; GS4 query server; version-specific
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Cap challenge entries, bound full-stat bytes/player fields, add rate limits and UDP amplification tests; compare all wire bytes against vanilla 1.20.1 if implementing query.
- RustChunk implementation mapping: Optional server query utility only, not client networking.
- Evidence required before adoption: Packet parity fixtures from exact target version, spoofing/amplification/resource stress and malformed UDP fuzzing.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1997. `crates/horizon-server/src/rcon.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5b58d0da340d83d323bed59dfe5c1f38ff11e2ae`
- Size: 26337 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements Vanilla RCON TCP remote console with bounded request packets, password authentication, login deadline and command forwarding.
- Dependencies / cross-references: TCP listener, server properties, constant-time same_token helper, game control endpoint and supervisor logs.
- Implementation / logic: Parses little-endian RCON frames, limits request to 1460 bytes, allows 16 concurrent clients, closes connections that fail to log in within 30 seconds and chunks command output.
- Findings: Connection cap, packet bound, deadline and constant-time password check are positive. Default server-ip empty binds 0.0.0.0; deployments need firewall/TLS/tailnet guidance. Reply chunk uses Unicode scalar count rather than vanilla's 4096 UTF-16 code units, so supplementary characters may differ.
- RustChunk relevance: No role in launcher, chunk rendering, Forge mod loading or Aternos join protocol.
- Decision: reference; RCON server; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Require secure network exposure or TLS tunnel, compare exact UTF-16 chunk behavior, fuzz frame parsing and stress login/connection limits.
- RustChunk implementation mapping: Separate optional server admin utility only; do not use as player connection implementation.
- Evidence required before adoption: RCON 1.20.1 wire parity, UTF-16 boundary fixtures, network exposure tests and protocol fuzzing.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1998. `crates/horizon-server/src/reference.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b2d2e8d16a6b996cef11dccbed1731a9ebcb6497`
- Size: 6997 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Generates the server reference Markdown from help text, config template and API routes, with tests detecting stale docs.
- Dependencies / cross-references: Horizon server HELP, config::TEMPLATE, config::TYPED, API ROUTES and filesystem under cfg(test).
- Implementation / logic: Parses comments and keys from the config template, renders command/API tables and compares generated output against docs/api/server-reference.md.
- Findings: Source-derived docs and stale-doc tests improve maintainability. The implementation is compiled for tests and has brittle text parsing/source scanning; the test was inspected only and not executed.
- RustChunk relevance: No direct game feature; useful only for maintaining future RustChunk documentation.
- Decision: adaptable; generated documentation test pattern; test-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Run the documentation test in CI; validate Markdown escaping and parser behavior for new TOML syntax/routes.
- RustChunk implementation mapping: Could adapt docs-from-source pattern for RustChunk developer docs, not runtime code.
- Evidence required before adoption: Passing generated-doc test and fixtures for special characters/new sections/routes.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 1999. `crates/horizon-server/src/service.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2907e762d88198778b38358b1c42037f8dba0999`
- Size: 19029 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Installs/removes/prints OS service definitions for running the Horizon server supervisor at login/boot on systemd, launchd and Windows Task Scheduler.
- Dependencies / cross-references: CLI process helpers, current release layout, filesystem, environment user/home, systemctl/launchctl/schtasks and service XML/plist templates.
- Implementation / logic: Builds service names/definitions, writes user/system systemd units, launchd agents or UTF-16 scheduled-task XML, then enables/starts or disables/removes them.
- Findings: Uses least-privilege Windows task and explicit service restart behavior. systemd unit interpolates program and working-directory strings without escaping embedded newlines/control syntax; a crafted directory path could alter unit content. Windows temp XML name is predictable in temp directory and merits race/symlink-safe creation.
- RustChunk relevance: Not applicable to Android launcher runtime.
- Decision: reference; OS service integration; host-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Escape/reject systemd control chars and validate user field; use secure create_new temp files with owner-only permissions; test paths with quotes/newlines and permission boundaries.
- RustChunk implementation mapping: No Android mapping; optional Linux desktop server service support only.
- Evidence required before adoption: Service file injection fixtures, temp-path race tests, least-privilege and OS integration tests.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.

### 2000. `crates/horizon-server/src/settings.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `14878876e83ecf64ca492d5cc37f683d8d52349b`
- Size: 12837 bytes
- Coverage: 1–EOF; complete file read (files over 400 lines were read in consecutive, gap-free chunks no larger than 400 lines).
- Purpose: Implements `horizon-server config` display and updates, typed TOML parsing, remote-change allowlist and live/restart classification.
- Dependencies / cross-references: Config::load/update_all, authenticated API Client, vanilla console commands, serde_json and TOML values.
- Implementation / logic: Parses section.field=value changes using existing TOML field types, applies numeric bounds, persists all changes and sends live-supported commands for difficulty/game mode/whitelist.
- Findings: Clear separation of saved/live/restart settings and secret masking in config display are useful. `properties` accepts loose values and changeable-remotely relies on a denylist; this needs review whenever a new sensitive property is added. Settings/defaults belong to the newer 26.3 server schema.
- RustChunk relevance: A generic settings workflow may transfer, but current property names and defaults cannot be copied to Forge 1.20.1.
- Decision: adaptable; typed config change workflow; server-only
- 1.20.1 implications: This file belongs to Horizon's dedicated-server management subsystem and is written against Minecraft Java 26.3 assumptions; it does not establish Minecraft 1.20.1 Forge client support, mod negotiation, Aternos compatibility or Android gameplay behavior.
- Follow-up checks: Use an explicit remote-change allowlist rather than denylist; ensure unknown/secret properties stay hidden; add atomic writes, version-specific property mapping and batch validation tests.
- RustChunk implementation mapping: Typed config change classification may inspire RustChunk settings management after building a distinct launcher/game schema.
- Evidence required before adoption: Security tests for remote property changes, secret redaction, atomic write failure and 1.20.1 setting parity.
- Completion state: COMPLETE — full content read through EOF; source blob SHA and byte size match the complete pinned recursive tree; embedded tests/examples were inspected only, not run.