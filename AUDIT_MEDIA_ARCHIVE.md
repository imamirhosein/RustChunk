# RustChunk Audit — Permanent Media Archive

> **INCOMPLETE HISTORICAL BACKFILL:** No media entries from audit numbers **1–3300** have yet been backfilled into this archive. This archive is newly created and must be reconciled against prior ledgers and the Index. Historical estimates suggest the first 1,000 audited files may include approximately 70–80 image files; this is an unverified estimate, not a count. Do not treat missing archive numbers as proof that the source was a normal text/code file until the backfill for that range is completed.

## Purpose and permanent scope

This is the single, continuous, permanent archive for **image and video source files only** across the entire Starling audit. Do not split it into 200-file stage archives or separate files by stage. Append media entries in original audit-number order. Ordinary non-media files are intentionally omitted; gaps between media entries indicate audit-numbered files that are not represented in this media-only archive, once the corresponding range is fully audited/backfilled.

## Required duplicate recording

Every image/video file must be recorded in both:
1. Its normal source-audit Ledger entry and Index/QA context, with the required audit details.
2. This permanent media archive, using the **same original audit number**, source path, and media type.

Neither record replaces the other. Archive entries must not use a separate media-only numbering scheme.

## Media review rules

- **Images:** render and visually inspect using available tools. Record a detailed description of visible content, UI text and elements, context, notable details, relevance/findings, and limitations. If visual rendering/inspection is not possible, explicitly mark **NEEDS USER REVIEW**; do not claim visual inspection.
- **Videos:** identify and record as video, with path and verified metadata where available. Per user instruction, mark for the user to watch later; do not claim the video was visually reviewed unless it was actually rendered and inspected.
- Record exact original audit entry number, path, type, review status, and any review notes. For images, include detailed visual analysis when actually possible.
- Media SHA/size should be included when verified against the pinned source.
- Backfill historical media **1–3300** before the backup/closeout at **4000**. Use Index/Ledgers to identify all media; resolve the estimated 70–80 images in the first 1,000 entries rather than relying on the estimate. Keep the archive chronological and verify coverage.
- At the **4000-entry backup/verification milestone**, explicitly check and complete the entire historical media backfill (entries 1–3999, as applicable) and verify the media archive is synchronized with all source Ledgers. Do not create the backup as complete while this media backfill is still outstanding.

## Entries

### Audit entry 3301 — VIDEO — NEEDS USER WATCH
- **Source path:** `launcher/docs/opening.webm`
- **Audit source SHA:** `ed57ee24af542a8091d1c2fdfe9f08ec7104a703`
- **Source size:** 84,132 bytes
- **Review status:** Video identified from file type; visual playback/review unavailable in this session.
- **Action:** User to watch later. Do not claim video content was analyzed.


### Audit entry 3302 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/patch-notes.png`; SHA `05c7bc7eb687427b781b03c1d42ccaafa26fcdac`; 361182 bytes.
- Notes: Horizon dark sidebar and Patch notes; support banner. 2026-10-13 note claims 100% parity/281 harnesses for MC26.3; lists server, worlds, mob AI, gear/enchantments and UI/render-distance features. Claims unverified.

### Audit entry 3303 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/play.png`; SHA `1d85a6f24ed255af0acb987d8b3b76ca7429580e`; 309912 bytes.
- Notes: MC26.3, Horizon v2026.10.12, 88%/270 harnesses, update available, Client/Server/version/Play, resources/folder missing. UI only.

### Audit entry 3304 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/preview.png`; SHA `17d673874f1b0a0306ee88ef3e20f8ba97c5167b`; 35559 bytes.
- Notes: Play, client/server, version, Prepare resources, Choose folder; not prepared/not found.

### Audit entry 3305 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/servers-tailscale.png`; SHA `ce040f1e9f377ebe9ceead98a0c20b87ca65a605`; 106464 bytes.
- Notes: Tailscale sharing/invites, sample host/address/email, same-Wi-Fi/router port forwarding, port 25565-only and no Funnel statement. Examples/security claims unverified.

### Audit entry 3306 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/servers.png`; SHA `bb644f4c0a8c37b1f9625c5b1eb9cf458e77d7e1`; 92934 bytes.
- Notes: running server, 0/10, 20 TPS, 0.6ms, 0.57GB, Stop/Restart, panel/folder, tabs, LAN/Tailscale addresses. Metrics screenshot-only.

### Audit entry 3307 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/settings-012.png`; SHA `99b7064a95a167ba627566616cdeb4875e8e641e`; 417567 bytes.
- Notes: older auto-install/opening/menu/update settings, temporary Linux path, CLI/MCP command. Desktop path/command not portable or reviewed.

### Audit entry 3308 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/settings.png`; SHA `455f8bf276618f3c01f829a6d4801d6e835e0d1f`; 96710 bytes.
- Notes: sign-in unconfigured/no client ID, Stable/Nightly, auto-install, opening/menu, extra args. UI not auth/update implementation proof.

### Audit entry 3309 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/versions.png`; SHA `786866f8443856531f20cf74dd6ea5d8f22d2733`; 234595 bytes.
- Notes: install/remove/in-use versions, per-folder statement, 15/14MB and parity labels; displayed claims unverified.

### Audit entry 3310 — IMAGE — VISUALLY REVIEWED
- `launcher/docs/whats-next.png`; SHA `ad9d9a4416dc75d67e371c97977ae61371ff8e56`; 59064 bytes.
- Notes: roadmap parity and packaged desktop checks, online-mode/skin/icon/hotbar limitations. Unverified and 26.3-specific.


### Audit entry 3645 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/128x128/apps/horizon.png`
- **Audit source SHA:** `82aa505087f9619dd9fd2c21a0cf07b4c6191d3a`
- **Source size:** 1,651 bytes
- **Review status:** PNG rendered at 128×128 and visually reviewed.
- **Notes:** Dark rounded-square icon, purple horizontal band and pale geometric H/pause-like central mark; no visible text. Brand artwork only, not RustChunk identity.

### Audit entry 3646 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/16x16/apps/horizon.png`
- **Audit source SHA:** `034e64bcab27b1dc968cac4bc9644d2a6c2b182e`
- **Source size:** 280 bytes
- **Review status:** PNG rendered at 16×16 and visually reviewed.
- **Notes:** Tiny dark icon with simplified pale central mark; band and fine geometry are barely distinguishable. No visible text.

### Audit entry 3647 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/24x24/apps/horizon.png`
- **Audit source SHA:** `10729734634614a9d04b478820452a0cc66e7ae5`
- **Source size:** 390 bytes
- **Review status:** PNG rendered at 24×24 and visually reviewed.
- **Notes:** Dark rounded square with centered pale geometric mark and subtle horizontal accent; no visible text.

### Audit entry 3648 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/256x256/apps/horizon.png`
- **Audit source SHA:** `5610f66051464788a1329ab905533400cca20890`
- **Source size:** 3,428 bytes
- **Review status:** PNG rendered at 256×256 and visually reviewed.
- **Notes:** Dark rounded-square background, broad purple stripe and two pale vertical strokes forming a minimalist central emblem; no visible text. Not evidence of Samsung Gaming Hub recognition.

### Audit entry 3649 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/32x32/apps/horizon.png`
- **Audit source SHA:** `3603cbe0fa7130493960a71c591b582d389983e1`
- **Source size:** 424 bytes
- **Review status:** PNG rendered at 32×32 and visually reviewed.
- **Notes:** Compact dark square with pale central glyph and horizontal accent; fine details are reduced. No visible text.

### Audit entry 3650 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/48x48/apps/horizon.png`
- **Audit source SHA:** `63f1d4e176df7d214e7b5c1b29ffc30cb405e8f0`
- **Source size:** 616 bytes
- **Review status:** PNG rendered at 48×48 and visually reviewed.
- **Notes:** Rounded dark square with pale central two-stroke mark and purple band; no visible text. Desktop hicolor asset, not Android adaptive icon.

