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



### Audit entry 3651 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/512x512/apps/horizon.png`
- **Audit source SHA:** `9bf6b676468876407f1c55cdda0f34636d01a4e0`
- **Source size:** 7,618 bytes
- **Review status:** PNG rendered at 512×512 and visually reviewed.
- **Notes:** Dark rounded-square tile with a broad purple horizontal stripe; unlike the paired vector/ICNS mark, no central vertical glyph is visible in this standalone raster.

### Audit entry 3652 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/64x64/apps/horizon.png`
- **Audit source SHA:** `1add8b2d610b44bd2b16ec3816882154c7d000e2`
- **Source size:** 797 bytes
- **Review status:** PNG rendered at 64×64 and visually reviewed.
- **Notes:** Dark rounded tile with two pale vertical bars, muted lower halves and a narrow purple horizontal line; no text.

### Audit entry 3653 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/hicolor/scalable/apps/horizon.svg`
- **Audit source SHA:** `94fb593fc7a57ff5199a5a230a04883698d5b538`
- **Source size:** 537 bytes
- **Review status:** SVG source inspected and rendered at 256×256.
- **Notes:** Title Horizon; dark rounded tile, two pale upper vertical strokes, dimmer lower strokes and purple horizontal stripe. Vector mark is clear at small size.

### Audit entry 3654 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/horizon.icns`
- **Audit source SHA:** `4c691e990bb37a7dea5734ad5ee75db6c4c94324`
- **Source size:** 40,332 bytes
- **Review status:** ICNS decoded; all 11 embedded PNG chunks rendered and visually reviewed.
- **Notes:** Contains icp4, icp5, icp6, ic07, ic08, ic09, ic10, ic11, ic12, ic13 and ic14 chunks, spanning tiny icons through 1024px. The larger variants show the dark rounded tile, pale two-bar glyph and purple stripe.

### Audit entry 3655 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/icons/horizon.ico`
- **Audit source SHA:** `b580493d67639d431db4dacecd4720b9f27b7b90`
- **Source size:** 7,704 bytes
- **Review status:** ICO decoded; all seven embedded PNG sizes rendered and visually reviewed.
- **Notes:** Contains 16, 24, 32, 48, 64, 128 and 256px PNG variants with dark rounded tile, pale central glyph and purple horizontal accent; small sizes simplify detail.

### Audit entry 3662 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_0.png`
- **Audit source SHA:** `f644128716d56725ede497da47462fbf0fb5c3a1`
- **Source size:** 12,956 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Transparent bitmap font atlas with white hand-drawn punctuation, digits, uppercase and lowercase ASCII glyphs arranged in rows.

### Audit entry 3663 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_1.png`
- **Audit source SHA:** `470be6b7c43370d22edfd2a0d511e7da4fc6784a`
- **Source size:** 13,189 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Same handwritten ASCII glyph grid as hand_0 with subtle changes in stroke shapes, slant and spacing.

### Audit entry 3664 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_2.png`
- **Audit source SHA:** `b9f321fa646a0f89d7d8beebc4306b046a084b7d`
- **Source size:** 12,959 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Alternate transparent handwritten ASCII glyph atlas with changed stroke jitter and shapes.

### Audit entry 3665 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_3.png`
- **Audit source SHA:** `ed6d005088ea8bae9e351d69c28c60cb05fe275d`
- **Source size:** 13,148 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Handwritten ASCII atlas variant with altered slant, wobble and stroke thickness.

### Audit entry 3666 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/font/hand_4.png`
- **Audit source SHA:** `91bacf1717c4ffeec4b266174fac037e1e9df9c9`
- **Source size:** 13,153 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Fifth transparent handwritten ASCII glyph atlas with small stroke, slant and spacing changes.

### Audit entry 3667 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/footer_separator.png`
- **Audit source SHA:** `23200ec05988269bf8fad133446e6bbc56204476`
- **Source size:** 157 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Very thin horizontal pale gray separator with slight pixel variation and transparent background.

### Audit entry 3668 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/header_separator.png`
- **Audit source SHA:** `70df76ea6e379083f3190cdc3a4853d723ca84a8`
- **Source size:** 150 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Thin light gray/white horizontal separator on transparency, visually close to footer separator but a distinct source blob.

### Audit entry 3669 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/menu_background.png`
- **Audit source SHA:** `f2022746015d315d38dd5d04ab83561dab52c23a`
- **Source size:** 9,801 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Dark charcoal field with subtle mottled/noise texture; no visible text or objects.

### Audit entry 3670 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/menu_list_background.png`
- **Audit source SHA:** `0bbddd06e28b0bcaf882a7df7203d13a70af1521`
- **Source size:** 9,530 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Dark charcoal textured panel background, close in appearance to menu_background but separately stored.

### Audit entry 3671 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button.png`
- **Audit source SHA:** `a1808f7e5081c4d41f9a7df7b09f66bca100135e`
- **Source size:** 61,321 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Very wide dark mottled button sprite with a thin irregular pale outline and transparent exterior; adjacent .mcmeta specifies nine-slice scaling.

### Audit entry 3673 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_1.png`
- **Audit source SHA:** `bad89171c4cfa1578e59f231e4861f6cc466e1f1`
- **Source size:** 58,780 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Alternate wide dark textured button with irregular pale border; visually distinct interior/edge noise and same nine-slice geometry as button.png.

### Audit entry 3675 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_2.png`
- **Audit source SHA:** `2666fa2ba9e43a2a7ca3338d01d4ecc3bd964644`
- **Source size:** 55,408 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Third wide dark mottled button sprite with thin irregular pale border; no matching sidecar metadata occurs in this checkpoint.



### Audit entry 3677 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_3.png`
- **Audit source SHA:** `8e717353b0adfeb088ae119ba1d18fbe759a3660`
- **Source size:** 59,783 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Wide dark mottled button with thin irregular pale outline and transparent exterior; fourth normal-state texture variant.

### Audit entry 3679 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled.png`
- **Audit source SHA:** `bb879590a36723ccfeeef9765da14e0a51b39b4b`
- **Source size:** 46,306 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Wide dark textured disabled button with pale irregular outline and a darker interior than active variants.

### Audit entry 3681 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_1.png`
- **Audit source SHA:** `1323629b71cfbd34875be97881e03efe1809163c`
- **Source size:** 49,000 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Disabled button variant with dark mottled interior and thin pale border; grain differs from base.

### Audit entry 3683 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_2.png`
- **Audit source SHA:** `1ee071c636833d55c9cee7d21ace714bb449c1a0`
- **Source size:** 46,092 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Dark wide disabled button with pale irregular outline and a distinct texture pattern.

### Audit entry 3685 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_disabled_3.png`
- **Audit source SHA:** `5b2d578852a2c0c0a7bd9b7ecfaaf806a4809d67`
- **Source size:** 46,159 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Dark textured disabled button with a thin pale frame and another distinct grain pattern.

### Audit entry 3687 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted.png`
- **Audit source SHA:** `1efcc21db9607546246a6246fef3917394213741`
- **Source size:** 50,632 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Brighter gray diagonal-hatch texture inside a pale irregular rectangular outline, representing a highlighted button state.

### Audit entry 3689 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_1.png`
- **Audit source SHA:** `69677683354f60b4857e3cb5fabb9b202759f04d`
- **Source size:** 45,077 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Alternate highlighted texture with lighter diagonal hatch and pale outline; one edge has a visibly different transparent/empty section.

### Audit entry 3691 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_2.png`
- **Audit source SHA:** `bf5d1071e6af3dc92c8283ffd2013afea7498167`
- **Source size:** 49,611 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Diagonal gray hatch texture with thin irregular pale frame.

### Audit entry 3693 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/button_highlighted_3.png`
- **Audit source SHA:** `ad0d55b50a5569274287af5e06d323158602210f`
- **Source size:** 49,197 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Light diagonal hatch texture with irregular pale outline and some edge transparency variation.

### Audit entry 3695 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/scroller.png`
- **Audit source SHA:** `18c8cd4dfd173b892ec2d105ea838018dcbf57f7`
- **Source size:** 2,292 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Narrow vertical dark textured scroller thumb with pale irregular side border and transparent center/edges.

### Audit entry 3697 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/scroller_background.png`
- **Audit source SHA:** `f08f91b8368fe35d60ad8517584152d2c05f58c5`
- **Source size:** 2,151 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Narrow vertical dark textured scroller track with thin pale side edges and transparent portions.

### Audit entry 3699 — IMAGE — VISUALLY REVIEWED
- **Source path:** `packaging/lowbudget/pack/assets/minecraft/textures/gui/sprites/widget/slider.png`
- **Audit source SHA:** `c68f0015867d6d1b6e8096032c8541a9bf2d46aa`
- **Source size:** 47,583 bytes
- **Review status:** PNG rendered and visually reviewed.
- **Notes:** Wide dark mottled slider track with thin pale irregular outline and transparent exterior; the knob/handle is not represented by this file.

